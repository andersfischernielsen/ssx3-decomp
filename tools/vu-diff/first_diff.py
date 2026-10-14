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
        if n > 40000:
            break
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

def words(line):
    return [w for w in line.split() if re.fullmatch(r"[0-9a-f]{8}", w)]


def classify(key, hist_keys, diffs):
    """Known ps2-web-recomp vs PCSX2 differences (see vu-differential.md)."""
    hdr = [d for d in diffs if " vi " in d]
    regs = [d for d in diffs if " vi " not in d]
    pairs = []
    for d in regs:
        o, r = d.split("\n")
        pairs += list(zip(words(o), words(r)))
    if hdr:
        o, r = hdr[0].split("\n")
        get = lambda s, k: s.split(k + " ")[1].split()[0]
        if get(o, "q") != get(r, "q") and re.search(r"div|sqrt", key):
            return "div-unit: x/0 or 0/0 Q (ours 0x7f7fffff like PCSX2, ref 0x7fffffff)"
        if get(o, "p") != get(r, "p") and "efu" in key:
            return "efu: P approximation (1 ulp)"
        mo, mr = int(get(o, "mac"), 16), int(get(r, "mac"), 16)
        so, sr = int(get(o, "st"), 16), int(get(r, "st"), 16)
        if (mo ^ mr) & 0x0f00 and not (mo ^ mr) & 0xf0ff:
            return "underflow: U flag (ours sets U like PCSX2 interp, ref never)"
        if (mo ^ mr) & 0xf000 or (so ^ sr) & 0x208:
            return "overflow flag / exponent-255 result"
        if (so ^ sr) & 0x104 and not (mo ^ mr) & 0xf0ff:
            return "underflow: U flag (ours sets U like PCSX2 interp, ref never)"
        if o.split(" q ")[0] != r.split(" q ")[0] and re.search(r"fmand|fmor|fmeq|fsand|fseq|fsor|fcand|fcor|fceq|fcget", key):
            return "flag timing: stall-aware (ours, PCSX2) vs pair count (ref)"
        if o.split(" q ")[0] != r.split(" q ")[0] and re.search(r"fmand|fsand|fcand|fmor|fsor|fcor|fmeq|fseq|fceq", " ".join(hist_keys)):
            return "flag timing: stall-aware (ours, PCSX2) vs pair count (ref)"
    for a, b in pairs:
        ea, eb = (int(a, 16) >> 23) & 0xff, (int(b, 16) >> 23) & 0xff
        if eb == 255 or a in ("7f7fffff", "ff7fffff"):
            return "overflow / exponent-255 operand (ours clamps like PCSX2, ref keeps exp 255)"
    if any(re.search(r"Src::Q", k) for k in [key]):
        return "Q-dependent value (upstream div-unit difference)"
    return "UNCLASSIFIED"


counter = Counter()
classes = Counter()
for idx in indices:
    h = recs[idx]
    r = first_diff(idx, h)
    if r is None:
        print(f"#{idx}: traces identical")
        continue
    pc, n, hist, diffs = r
    key = pair_text(h, pc)
    counter[(h, pc)] += 1
    cls = classify(key, [pair_text(h, p) for p in hist], diffs)
    classes[cls] += 1
    if summary:
        print(f"#{idx} {h} pair {n} pc {pc:#06x} [{cls}]: {key} :: {diffs[0].splitlines()[0][6:60]}")
        continue
    print(f"class: {cls}")
    print(f"#{idx} {h}: first difference after pair {n} at pc {pc:#06x}")
    for p in hist:
        print(f"    {p:#06x}: {pair_text(h, p)}")
    print(f" -> {pc:#06x}: {key}")
    for d in diffs[:8]:
        print("    " + d)
print("classes:")
for c, k in classes.most_common():
    print(f"  {k:4} {c}")
if summary:
    print("most common first-divergence pcs:")
    for (h, pc), c in counter.most_common(30):
        print(f"  {c:4} {h} {pc:#06x}: {pair_text(h, pc)}")
