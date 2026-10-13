#!/usr/bin/env python3
"""Export authoritative per-instruction boundaries from Ghidra.

For each function Ghidra lists instructions at specific addresses; the gaps are
bytes it did not disassemble (embedded data, jump tables, unreachable slots).
We collapse the addresses into contiguous runs:

  generated/boundaries.tsv : "<hex-func-addr>\\t<hex-start>-<hex-end>[,<start>-<end>...]"
                             (end exclusive, addresses ascending)

This is the authoritative instruction map the recompiler decodes against, and
the reference our own recursive-descent disassembler is validated against.

Usage: tools/export-ghidra-boundaries.py
"""
from __future__ import annotations

import argparse
import json
import pathlib
import sys
import urllib.request

PAGE = 2000


def get_json(url: str) -> dict:
    with urllib.request.urlopen(url, timeout=120) as resp:
        return json.load(resp)


def function_addresses(base: str) -> list[int]:
    addrs, offset = [], 0
    while True:
        data = get_json(f"{base}/functions?offset={offset}&limit={PAGE}")["data"]
        for it in data["items"]:
            addrs.append(int(it["address"], 16))
        if not data.get("has_more") or not data["items"]:
            break
        offset += len(data["items"])
    return sorted(addrs)


def runs_for(base: str, addr: int) -> list[tuple[int, int]]:
    items = get_json(
        f"{base}/disassemble_function?address=0x{addr:08x}&limit=100000"
    )["data"]["items"]
    addrs = sorted(int(it["address"], 16) for it in items)
    runs: list[tuple[int, int]] = []
    for a in addrs:
        if runs and a == runs[-1][1]:
            runs[-1] = (runs[-1][0], a + 4)
        else:
            runs.append((a, a + 4))
    return runs


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--base", default="http://127.0.0.1:8080")
    ap.add_argument("--out", default="generated/boundaries.tsv")
    args = ap.parse_args()

    addrs = function_addresses(args.base)
    print(f"functions: {len(addrs)}", file=sys.stderr)

    lines, total_instr, holed = [], 0, 0
    for i, a in enumerate(addrs, 1):
        runs = runs_for(args.base, a)
        n = sum((hi - lo) // 4 for lo, hi in runs)
        total_instr += n
        if len(runs) > 1:
            holed += 1
        spec = ",".join(f"{lo:08x}-{hi:08x}" for lo, hi in runs)
        lines.append(f"{a:08x}\t{spec}")
        if i % 1000 == 0:
            print(f"  {i:>6}/{len(addrs)}", file=sys.stderr)

    path = pathlib.Path(args.out)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join(lines) + "\n")

    print(f"wrote {len(lines)} functions -> {path}")
    print(f"  instructions   : {total_instr}")
    print(f"  with holes     : {holed}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
