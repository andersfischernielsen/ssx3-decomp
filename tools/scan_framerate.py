#!/usr/bin/env python3
"""Find the game's 60 Hz assumptions in the executable.

    python3 tools/scan_framerate.py [--tsv OUT]

Scans orig/SLUS_207.72 for frame-rate constants and reports every use by
function (generated/functions.tsv) and source unit (the reference's splat
config), and whether the function is already hand-written (overrides.txt):

- float constants loaded with `lwc1` from .lit4/.rodata/.sdata (gp-relative or
  `lui`+offset) or built in a register (`lui`[+`ori`] then `mtc1`):
  1/60, 60.0, 1/30, 30.0, 3600.0, 0.01667 (movie player), 1/50, 50.0;
- reads of the app manager's tick rate (+0x10, 60) and step (+0x14, 1/60)
  through the global D_004A5B64: the code that already follows the rate;
- integer division by 60 (the compiler's 0x88888889 reciprocal multiply) and
  the integers 3600 (frames per minute) and 216000 (frames per hour).

Implicit per-frame rates (`v *= 0.98` once per frame) carry no marker and are
not found by this scan.
"""
import bisect
import collections
import os
import re
import struct
import sys

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
ELF = os.path.join(ROOT, "orig/SLUS_207.72")
REF = os.environ.get("SSX3_REF", os.path.expanduser("~/Desktop/devvydont-ssx3"))

FLOATS = {
    0x3C888889: "1/60",
    0x42700000: "60.0",
    0x3D088889: "1/30",
    0x41F00000: "30.0",
    0x45610000: "3600.0",
    0x3C888CE7: "0.01667",
    0x3CA3D70A: "1/50",
    0x42480000: "50.0",
}
APPMAN = 0x4A5B64  # cAppMan* (rate +0x10, dt +0x14)
INTS = {3600: "int 3600", 216000: "int 216000"}


def sections(d):
    shoff = struct.unpack_from("<I", d, 0x20)[0]
    n, stri = struct.unpack_from("<HH", d, 0x30)
    sh = [struct.unpack_from("<10I", d, shoff + i * 40) for i in range(n)]
    so = sh[stri][4]
    out = {}
    for s in sh:
        name = d[so + s[0]:d.index(b"\0", so + s[0])].decode()
        out[name] = (s[3], s[4], s[5])  # addr, offset, size
    return out


def main():
    d = open(ELF, "rb").read()
    sec = sections(d)
    gp = struct.unpack_from("<i", d, sec[".reginfo"][1] + 20)[0]

    # Constant pools: address -> label.
    pool = {}
    for name in (".lit4", ".rodata", ".sdata", ".data"):
        a, off, size = sec[name]
        for i in range(0, size - 3, 4):
            w = struct.unpack_from("<I", d, off + i)[0]
            if w in FLOATS:
                pool[a + i] = FLOATS[w]

    funcs = []
    for line in open(os.path.join(ROOT, "generated/functions.tsv")):
        p = line.rstrip("\n").split("\t")
        if len(p) >= 3:
            funcs.append((int(p[0], 16), int(p[1], 16), p[2]))
    funcs.sort()
    starts = [f[0] for f in funcs]

    units = []
    yml = os.path.join(REF, "config/ssx3_us.yaml")
    if os.path.exists(yml):
        for m in re.finditer(r"- \[0x([0-9a-fA-F]+), \w+, ([\w/]+)\]", open(yml).read()):
            units.append((int(m.group(1), 16) + 0xFF000, m.group(2)))
        units.sort()
    ustarts = [u[0] for u in units]

    ported = set()
    for line in open(os.path.join(ROOT, "crates/ssx3-game/overrides.txt")):
        if line.startswith("0x"):
            ported.add(int(line.split()[0], 16))

    def func_of(a):
        i = bisect.bisect_right(starts, a) - 1
        return funcs[i] if i >= 0 and a < funcs[i][1] else (a, a, "?")

    def unit_of(a):
        i = bisect.bisect_right(ustarts, a) - 1
        return units[i][1] if i >= 0 else "?"

    hits = []  # (insn addr, label)
    ta, toff, tsize = sec[".text"]
    lui = {}  # reg -> (hi value, addr), reset at function starts
    app = set()  # registers holding the cAppMan pointer (D_004A5B64)
    fset = set(starts)
    for i in range(0, tsize, 4):
        a = ta + i
        if a in fset:
            lui.clear()
            app.clear()
        w = struct.unpack_from("<I", d, toff + i)[0]
        op, rs, rt, imm = w >> 26, (w >> 21) & 31, (w >> 16) & 31, w & 0xFFFF
        simm = imm - 0x10000 if imm & 0x8000 else imm
        if op == 0x0F:  # lui
            lui[rt] = imm << 16
            v = imm << 16
            if v in FLOATS:
                hits.append((a, FLOATS[v] + " (imm)"))
            if v == 0x88880000:
                pass
            continue
        if op == 0x0D and rs in lui:  # ori
            v = lui[rs] | imm
            if v in FLOATS:
                hits.append((a, FLOATS[v] + " (imm)"))
            elif v == 0x88888889:
                hits.append((a, "/60 (int divide)"))
            lui[rt] = v
            continue
        if op == 0x23 and rs == 28 and (gp + simm) & 0xFFFFFFFF == APPMAN:  # lw rt, D_004A5B64
            app.add(rt)
            continue
        if rs in app and op in (0x23, 0x31) and simm in (0x10, 0x14):
            hits.append((a, "AppMan rate" if simm == 0x10 else "AppMan dt"))
            if rt != rs or op == 0x31:
                pass
            if op == 0x23:
                app.discard(rt)
            continue
        if op == 0x23:
            app.discard(rt)
        if op == 0x31:  # lwc1
            if rs == 28:
                tgt = (gp + simm) & 0xFFFFFFFF
            elif rs in lui:
                tgt = (lui[rs] + simm) & 0xFFFFFFFF
            else:
                tgt = None
            if tgt in pool:
                hits.append((a, pool[tgt]))
            continue
        if op in (0x09, 0x0D) and rs == 0 and imm in INTS:  # li
            hits.append((a, INTS[imm]))
        if op == 0x0D and rs in lui and (lui[rs] | imm) in INTS:
            hits.append((a, INTS[lui[rs] | imm]))
        if op not in (0x0F,) and rt in lui and op in (0x08, 0x09, 0x0D, 0x23, 0x2B, 0x31, 0x39):
            # Keep the lui base for loads/stores from it (rs), drop when overwritten.
            if op in (0x09, 0x0D, 0x23) and rt != rs:
                lui.pop(rt, None)

    by_func = collections.OrderedDict()
    for a, label in hits:
        f = func_of(a)
        by_func.setdefault(f, []).append((a, label))

    out = None
    if "--tsv" in sys.argv:
        out = open(sys.argv[sys.argv.index("--tsv") + 1], "w")
        out.write("func\tname\tunit\tported\tlabels\tsites\n")
    per_label = collections.Counter(l.replace(" (imm)", "") for _, l in hits)
    per_unit = collections.Counter()
    nport = 0
    for f, hs in sorted(by_func.items()):
        u = unit_of(f[0])
        per_unit[u] += 1
        p = f[0] in ported
        nport += p
        labels = ",".join(sorted(set(l for _, l in hs)))
        if out:
            out.write(f"0x{f[0]:08x}\t{f[2]}\t{u}\t{'yes' if p else 'no'}\t{labels}\t"
                      + " ".join(f"0x{a:x}" for a, _ in hs) + "\n")
    print(f"{len(hits)} sites in {len(by_func)} functions ({nport} already hand-written)")
    print("by constant:", ", ".join(f"{k}={v}" for k, v in per_label.most_common()))
    print("by unit (functions):")
    for u, n in per_unit.most_common():
        print(f"  {n:4d}  {u}")


if __name__ == "__main__":
    main()
