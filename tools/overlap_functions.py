#!/usr/bin/env python3
"""Write `overlap-functions.tsv`: our functions that the reference C++
decompilation also has, with its unit and name next to ours.

    python3 tools/overlap_functions.py [--rows overlap-functions.tsv] [--out FILE]

Columns: address, bytes (from `generated/functions.tsv`; 0 for the entry
points of `generated/discovered-names.tsv`), fork_unit (the reference's built
unit, `src/<dir>/<unit>.cpp`), fork_name (its symbol, GCC mangling dropped, or
`func_XXXXXXXX`), anders_name (ours).

Without `--rows` every function both sides know is written. With `--rows`
only the addresses already in that table are kept (refreshes the reference
columns of an existing table without changing its row set). Needs
`tools/.refindex.json` (`refindex.py`) for the current reference.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from refsrc import REF, load_index, load_symbols  # noqa: E402

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def ours():
    """address -> (bytes, name)."""
    out = {}
    for line in open(os.path.join(ROOT, "generated/functions.tsv")):
        f = line.rstrip("\n").split("\t")
        out[int(f[0], 16)] = (int(f[1], 16) - int(f[0], 16), f[2])
    for line in open(os.path.join(ROOT, "generated/discovered-names.tsv")):
        if line.startswith("#"):
            continue
        f = line.rstrip("\n").split("\t")
        if len(f) > 1 and not f[1].startswith("NOTFUNC_"):
            out.setdefault(int(f[0], 16), (0, f[1]))
    return out


def plain(name):
    """`name` without its GCC 2 function mangling (`f__FPvi` -> `f`)."""
    i = name.rfind("__F")
    return name[:i] if i > 0 else name


def main(argv):
    out = os.path.join(ROOT, "overlap-functions.tsv")
    if "--out" in argv:
        out = argv[argv.index("--out") + 1]
    keep = None
    if "--rows" in argv:
        keep = {int(l.split("\t")[0], 16) for l in list(open(argv[argv.index("--rows") + 1]))[1:]}
    syms, index = load_symbols(), load_index()
    if not index:
        sys.exit("no tools/.refindex.json: run tools/refindex.py first")
    rows = []
    for addr, (size, name) in sorted(ours().items()):
        if keep is not None and addr not in keep:
            continue
        ref = syms.get(addr)
        hit = index.get(ref) if ref else None
        if hit is None:
            ref = f"func_{addr:08X}"
            hit = index.get(ref)
        if hit is None:
            continue
        unit = os.path.relpath(hit[1], REF)
        rows.append((unit, addr, f"0x{addr:08x}\t{size}\t{unit}\t{plain(ref)}\t{name}\n"))
    # Grouped by unit, the biggest first; by address within a unit.
    count = {}
    for unit, _, _ in rows:
        count[unit] = count.get(unit, 0) + 1
    rows.sort(key=lambda r: (-count[r[0]], r[0], r[1]))
    with open(out, "w") as f:
        f.write("address\tbytes\tfork_unit\tfork_name\tanders_name\n")
        f.writelines(r[2] for r in rows)
    print(f"{len(rows)} functions -> {out}", file=sys.stderr)


if __name__ == "__main__":
    main(sys.argv[1:])
