#!/usr/bin/env python3
"""Export Ghidra's disassembly of selected functions as a golden corpus.

Writes generated/ghidra_disasm/<address>.txt with one line per instruction:

    <address> <8-hex-bytes> <gnu/generic disassembly text>

The Rust decoder is validated against these: we check that the words we read
from the ELF match Ghidra's bytes and that we cover the same addresses.

Usage:
    tools/export-ghidra-disasm.py [0xADDR ...]
"""
from __future__ import annotations

import argparse
import json
import pathlib
import sys
import urllib.request

# A spread of functions: the crt0 entry, early .text, an SDK entry, and a few
# from the far end of .text. Addresses come from generated/functions.tsv.
DEFAULT = [
    "0x00100008",  # entry
    "0x001001c8",  # FUN_001001c8
    "0x0012ab20",  # FUN_0012ab20
    "0x00200218",  # FUN_00200218 (mid .text)
    "0x004239a0",  # SetGsCrt
    "0x004240e0",  # GsPutIMR
    "0x0042d470",  # far end of .text
]


def fetch(base: str, addr: str) -> list[dict]:
    url = f"{base}/disassemble_function?address={addr}&limit=5000"
    with urllib.request.urlopen(url, timeout=120) as resp:
        payload = json.load(resp)
    return payload["data"]["items"]


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("addresses", nargs="*", default=DEFAULT)
    ap.add_argument("--base", default="http://127.0.0.1:8080")
    ap.add_argument("--out-dir", default="generated/ghidra_disasm")
    args = ap.parse_args()

    out = pathlib.Path(args.out_dir)
    out.mkdir(parents=True, exist_ok=True)

    for addr in args.addresses:
        items = fetch(args.base, addr)
        lines = []
        for it in items:
            text = it.get("instruction") or ""
            cmt = it.get("comment")
            if cmt:
                text = f"{text}  ; {cmt}"
            lines.append(f"{it['address']} {it['bytes']} {text}")
        path = out / f"{addr.lower().removeprefix('0x')}.txt"
        path.write_text("\n".join(lines) + "\n")
        print(f"{addr}: {len(items):>4} instructions -> {path}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
