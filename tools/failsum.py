#!/usr/bin/env python3
"""Summarise the failures of a libtest log, per staging module.

    python3 tools/failsum.py LOG [OUTDIR]

Writes `OUTDIR/<module>.txt` (default `target/gate/fail/`) with each failing
test's output, ready to hand to the module's writer, and prints one line per
failure: `ocN test: runaway at pc 0x... | diff: cycles ... | panic ...`.

The `"x vs y"` lines of a differential failure read GENERATED vs PORT (the
first value is the generated body). Runaways at pc 0x3de768 / 0x31ef40 /
0x31e968 are the allocator spin: use `fakes::machine_rt` in the fixture.
"""
import collections
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def main(argv):
    if not argv:
        sys.exit(__doc__)
    log = open(argv[0]).read()
    out = argv[1] if len(argv) > 1 else os.path.join(ROOT, "target/gate/fail")
    os.makedirs(out, exist_ok=True)
    per = collections.defaultdict(str)
    rows = []
    for m in re.finditer(r"---- (\S+) stdout ----\n(.*?)(?=\n---- |\nfailures:\n)", log, re.S):
        name, body = m.group(1), m.group(2)
        mod = name.split("::")[0]
        per[mod] += f"==== {name}\n" + "\n".join(l[:300] for l in body.split("\n")[:60]) + "\n"
        why = "?"
        r = re.search(r"cycle budget exceeded \(\d+ cycles, pc (0x[0-9a-f]+)\)", body)
        if r:
            pc = r.group(1)
            hint = " (allocator spin: use fakes::machine_rt)" if pc in ("0x3de768", "0x3de878", "0x31ef40", "0x31ea40", "0x31e968") else ""
            why = f"runaway at pc {pc}{hint}"
        else:
            d = [l.strip(' ",') for l in body.split("\n") if re.search(r"cycles:|call args|seed \d+", l)]
            why = "diff: " + " | ".join(d[:2]) if d else "panic: " + (body.strip().split("\n")[1] if "\n" in body.strip() else body.strip())[:120]
        rows.append((mod, name.split("::", 2)[-1], why))
    for mod, text in per.items():
        open(os.path.join(out, f"{mod}.txt"), "w").write(text)
    for mod, test, why in rows:
        print(f"{mod} {test}: {why}")
    tot = re.findall(r"test result: \w+\. (\d+) passed; (\d+) failed", log)
    if tot:
        print("passed %d, failed %d" % (sum(int(a) for a, _ in tot), sum(int(b) for _, b in tot)))


if __name__ == "__main__":
    main(sys.argv[1:])
