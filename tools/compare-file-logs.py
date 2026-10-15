#!/usr/bin/env python3
"""Compare two SSX3_FILE_LOG read logs (emulated CD vs native files).

    SSX3_FILE_LOG=emu.tsv ssx3-headless N ...
    SSX3_NATIVE_FILES=1 SSX3_FILE_LOG=native.tsv ssx3-headless N ...
    tools/compare-file-logs.py emu.tsv native.tsv

Each line is `name offset size fnv64` for one completed read. Reads of the
same (name, offset, size) must have the same hash. Order and timing may
differ (native reads complete sooner), so the comparison is per read key; it
also reports reads that appear in only one log (a run that got further).
"""

import collections
import sys


def load(path):
    reads = collections.defaultdict(list)
    order = []
    for line in open(path):
        name, off, size, h = line.rstrip("\n").split("\t")
        key = (name, int(off, 16), int(size, 16))
        reads[key].append(h)
        order.append(key)
    return reads, order


def main():
    a, oa = load(sys.argv[1])
    b, ob = load(sys.argv[2])
    common = set(a) & set(b)
    bad = [k for k in common if set(a[k]) != set(b[k])]
    print(f"{sys.argv[1]}: {len(oa)} reads, {len(a)} distinct; {sys.argv[2]}: {len(ob)} reads, {len(b)} distinct")
    print(f"common distinct reads: {len(common)}, content mismatches: {len(bad)}")
    for k in bad[:20]:
        print(f"  MISMATCH {k[0]} @{k[1]:#x} +{k[2]:#x}: {sorted(set(a[k]))} vs {sorted(set(b[k]))}")
    only_a = [k for k in oa if k not in b]
    only_b = [k for k in ob if k not in a]
    print(f"only in first: {len(set(only_a))}, only in second: {len(set(only_b))}")
    for k in list(dict.fromkeys(only_a))[:10]:
        print(f"  first only:  {k[0]} @{k[1]:#x} +{k[2]:#x}")
    for k in list(dict.fromkeys(only_b))[:10]:
        print(f"  second only: {k[0]} @{k[1]:#x} +{k[2]:#x}")
    # Same order of first appearance for the common part?
    fa = [k for k in dict.fromkeys(oa) if k in common]
    fb = [k for k in dict.fromkeys(ob) if k in common]
    print("first-appearance order identical" if fa == fb else "first-appearance order differs")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
