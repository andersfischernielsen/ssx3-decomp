#!/usr/bin/env python3
"""Lockstep per-pair trace comparison of our VU (reference interpreter over
the same semantic library) and ps2-web-recomp's interpreter; prints the first
pair whose resulting state differs, with the generated-code text of that pair
and the previous few pairs.

  first_diff.py <vu_diff> <vu-ref> <capture> <index...> [--ref-model] [--summary]
"""
import re
import subprocess
import sys
from collections import Counter

args = [a for a in sys.argv[1:] if not a.startswith("--")]
flags = [a for a in sys.argv[1:] if a.startswith("--")]
vu_diff, vu_ref, cap = args[:3]
indices = args[3:]
summary = "--summary" in flags
extra = ["--ref-model"] if "--ref-model" in flags else []

gen_cache = {}


def pair_text(hash_, pc):
    """Generated Rust for the pair at pc (from its first one-pair arm)."""
    if hash_ not in gen_cache:
        try:
            gen_cache[hash_] = open(f"generated/vu/vu_{hash_}.rs").read()
        except OSError:
            gen_cache[hash_] = ""
    src = gen_cache[hash_]
    m = re.search(rf"// {pc:#06x}: [^\n]*\n((?:(?!\s*// 0x)[^\n]*\n)*)", src)
    if not m:
        return "?"
    body = [l.strip() for l in m.group(0).splitlines()]
    return " ".join(l for l in body if l and "begin_pair" not in l and "end_pair" not in l and "out_of_budget" not in l)


def blocks(proc):
    blk = []
    for line in proc.stdout:
        if not line.startswith(" ") and blk:
            yield blk
            blk = []
        blk.append(line.rstrip("\n"))
    if blk:
        yield blk


def first_diff(idx, hash_):
    a = subprocess.Popen([vu_diff, "trace", cap, idx] + extra, stdout=subprocess.PIPE, text=True, stderr=subprocess.DEVNULL)
    b = subprocess.Popen([vu_ref, cap, "/dev/null", "--only", idx, "--trace", idx], stdout=subprocess.PIPE, text=True, stderr=subprocess.DEVNULL)
    hist = []
    n = 0
    result = None
    for x, y in zip(blocks(a), blocks(b)):
        n += 1
        if x != y:
            pc = int(x[0][:4], 16)
            diffs = [f"ours: {p.strip()}\n      ref:  {q.strip()}" for p, q in zip(x, y) if p != q]
            result = (pc, n, hist[-4:], diffs)
            break
        hist.append(int(x[0][:4], 16))
    a.kill()
    b.kill()
    return result


recs = {}
for line in subprocess.run([vu_diff, "list", cap], capture_output=True, text=True).stdout.splitlines():
    f = line.split()
    recs[f[0]] = f[7]

counter = Counter()
for idx in indices:
    h = recs[idx]
    r = first_diff(idx, h)
    if r is None:
        print(f"#{idx}: traces identical")
        continue
    pc, n, hist, diffs = r
    key = pair_text(h, pc)
    counter[(h, pc)] += 1
    if summary:
        print(f"#{idx} {h} pair {n} pc {pc:#06x}: {key} :: {diffs[0].splitlines()[0][6:60]}")
        continue
    print(f"#{idx} {h}: first difference after pair {n} at pc {pc:#06x}")
    for p in hist:
        print(f"    {p:#06x}: {pair_text(h, p)}")
    print(f" -> {pc:#06x}: {key}")
    for d in diffs[:8]:
        print("    " + d)
if summary:
    print("most common first-divergence pcs:")
    for (h, pc), c in counter.most_common(30):
        print(f"  {c:4} {h} {pc:#06x}: {pair_text(h, pc)}")
