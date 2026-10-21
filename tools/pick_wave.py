#!/usr/bin/env python3
"""Pick a porting wave by *runtime hotness*, not size.

    python3 tools/pick_wave.py EDGES.TSV OUTDIR N MODULES...

`EDGES.TSV` is a `SSX3_CALLPROF` call graph (caller, callee, count over a frame
range). The weight of an unported function is

    calls/frame * bytes

(calls from the profile, bytes from generated/functions.tsv), which is a
better proxy for time than size. Functions already installed (`overrides.txt`
or any `oc*.rs` OVERRIDES) and functions assigned to an earlier `wave*.txt` are
skipped. Output: `OUTDIR/waveN-<module>.txt`, 50 per module, hottest first.

The dossiers (reference C++) are made separately with `refdossier2.py`; for
functions with no C++ body the dossier says "asm only" and the generated Rust
is the source to port from.
"""
import glob
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from refsrc import load_symbols  # noqa: E402


def installed():
    s = set()
    for l in open('crates/ssx3-game/overrides.txt'):
        l = l.strip()
        if l and not l.startswith('#'):
            try:
                s.add(int(l.split()[0], 16))
            except ValueError:
                pass
    for p in glob.glob('crates/ssx3-game/src/oc*.rs'):
        t = open(p).read()
        try:
            i = t.index('pub const OVERRIDES')
        except ValueError:
            continue
        for a in re.findall(r'\(\s*0x([0-9a-f_]+)\s*,', t[i:t.index('];', i)]):
            s.add(int(a.replace('_', ''), 16))
    return s


def main(argv):
    edges, outdir, wave = argv[0], argv[1], argv[2]
    mods = argv[3:]
    S = os.environ.get('SSX3_SCRATCH', '')
    assigned = installed()
    for p in glob.glob(os.path.join(S, 'wave*.txt')) + glob.glob(os.path.join(S, 'wave*.tsv')):
        for l in open(p):
            if l.startswith('0x'):
                try:
                    assigned.add(int(l.split('\t')[0], 16))
                except ValueError:
                    pass

    size, name = {}, {}
    for l in open('generated/functions.tsv'):
        p = l.split('\t')
        try:
            a, e = int(p[0], 16), int(p[1], 16)
        except ValueError:
            continue
        size[a] = max(e - a, 4)
        name[a] = p[2].strip() if len(p) > 2 else ''
    syms = load_symbols()

    calls = {}
    frames = 0
    for l in open(edges):
        if l.startswith('#'):
            m = re.match(r'# frames (\d+)\.\.=(\d+)', l)
            if m:
                frames = int(m.group(2)) - int(m.group(1)) + 1
            continue
        if not l.strip():
            continue
        _caller, callee, n = l.split()
        a = int(callee, 16)
        calls[a] = calls.get(a, 0) + int(n)
    frames = max(frames, 1)

    rows = []
    for a, c in calls.items():
        if a in assigned or a not in size:
            continue
        cf = c / frames
        rows.append((cf * size[a], cf, size[a], a))
    rows.sort(reverse=True)

    n = 50
    for k, m in enumerate(mods):
        chunk = rows[k * n:(k + 1) * n]
        with open(os.path.join(outdir, f'{wave}-{m}.txt'), 'w') as f:
            for w, cf, b, a in chunk:
                f.write(f'{a:#010x}\t{name.get(a) or syms.get(a) or "?"}\n')
        print(f'{m}: {len(chunk)} functions')
    print('\ntop of the wave (weight = calls/frame * bytes):')
    for w, cf, b, a in rows[:min(15, len(rows))]:
        print(f'{w:9.1f}  {cf:8.2f} calls/f  {b:5d} B  {a:#010x}  {name.get(a) or syms.get(a)}')


if __name__ == '__main__':
    main(sys.argv[1:])
