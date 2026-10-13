#!/usr/bin/env python3
"""Export the full function inventory from the running GhidraMCP backend.

Talks to the GhidraMCP loopback HTTP backend (default http://127.0.0.1:8080).
Phase 1 pages ``/functions`` for names; phase 2 queries ``/function`` per entry
for the exact body extents. Writes:

  generated/functions.json  -- [{name, address, end}] sorted by address
  generated/functions.tsv   -- "<hex-address>\\t<hex-end>\\t<name>" per line

``end`` is exclusive (Ghidra's inclusive ``body_end`` plus one). The TSV is what
the Rust side embeds, so keep it stable.
"""
from __future__ import annotations

import argparse
import json
import pathlib
import sys
import urllib.request

PAGE = 2000  # backend cap observed on /functions


def get_json(url: str) -> dict:
    with urllib.request.urlopen(url, timeout=60) as resp:
        return json.load(resp)


def fetch_names(base: str) -> dict[int, str]:
    by_addr: dict[int, str] = {}
    offset = 0
    while True:
        data = get_json(f"{base}/functions?offset={offset}&limit={PAGE}")["data"]
        batch = data["items"]
        for it in batch:
            by_addr[int(it["address"], 16)] = it["name"]
        print(f"  names {len(by_addr):>6}/{data['total']}", file=sys.stderr)
        if not data.get("has_more") or not batch:
            break
        offset += len(batch)
    return by_addr


def fetch_ends(base: str, addresses: list[int]) -> dict[int, int]:
    ends: dict[int, int] = {}
    for i, a in enumerate(addresses, 1):
        d = get_json(f"{base}/function?address=0x{a:08x}")["data"]
        ends[a] = int(d["body_end"], 16) + 1  # exclusive
        if i % 1000 == 0:
            print(f"  bodies {i:>6}/{len(addresses)}", file=sys.stderr)
    return ends


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--base", default="http://127.0.0.1:8080")
    ap.add_argument("--out-dir", default="generated")
    ap.add_argument("--no-bodies", action="store_true")
    args = ap.parse_args()

    out = pathlib.Path(args.out_dir)
    out.mkdir(parents=True, exist_ok=True)

    print(f"Exporting functions from {args.base} ...", file=sys.stderr)
    names = fetch_names(args.base)
    addresses = sorted(names)

    if args.no_bodies:
        ends = {a: names_next(addresses, i) for i, a in enumerate(addresses)}
    else:
        ends = fetch_ends(args.base, addresses)

    rows = [(a, ends[a], names[a]) for a in addresses]

    (out / "functions.json").write_text(
        json.dumps(
            [{"name": n, "address": f"{a:08x}", "end": f"{e:08x}"} for a, e, n in rows],
            indent=0,
        )
        + "\n"
    )
    (out / "functions.tsv").write_text(
        "".join(f"{a:08x}\t{e:08x}\t{n}\n" for a, e, n in rows)
    )

    print(f"wrote {len(rows)} functions with extents -> {out}/functions.{{json,tsv}}")
    return 0


def names_next(addresses: list[int], i: int) -> int:
    return addresses[i + 1] if i + 1 < len(addresses) else addresses[i] + 4


if __name__ == "__main__":
    raise SystemExit(main())
