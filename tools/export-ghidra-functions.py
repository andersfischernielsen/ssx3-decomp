#!/usr/bin/env python3
"""Export the full function inventory from the running GhidraMCP backend.

Talks to the GhidraMCP loopback HTTP backend (default http://127.0.0.1:8080),
paging through ``/functions`` and writing a canonical, sorted inventory:

  generated/functions.json  -- {name, address} list, as returned
  generated/functions.tsv   -- "<hex-address>\\t<name>" per line, sortable

The TSV is what the Rust side embeds, so keep it stable.
"""
from __future__ import annotations

import argparse
import json
import pathlib
import sys
import urllib.request

PAGE = 2000  # backend cap observed on /functions


def fetch_functions(base: str) -> list[dict]:
    items: list[dict] = []
    offset = 0
    while True:
        url = f"{base}/functions?offset={offset}&limit={PAGE}"
        with urllib.request.urlopen(url, timeout=60) as resp:
            payload = json.load(resp)
        data = payload["data"]
        batch = data["items"]
        items.extend(batch)
        print(f"  fetched {len(items):>6}/{data['total']}", file=sys.stderr)
        if not data.get("has_more") or not batch:
            break
        offset += len(batch)
    return items


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--base", default="http://127.0.0.1:8080")
    ap.add_argument("--out-dir", default="generated")
    args = ap.parse_args()

    out = pathlib.Path(args.out_dir)
    out.mkdir(parents=True, exist_ok=True)

    print(f"Exporting functions from {args.base} ...", file=sys.stderr)
    items = fetch_functions(args.base)

    # Canonicalise: dedupe by address, then sort ascending.
    by_addr: dict[int, str] = {}
    for it in items:
        by_addr[int(it["address"], 16)] = it["name"]
    ordered = sorted(by_addr.items())

    json_path = out / "functions.json"
    json_path.write_text(
        json.dumps(
            [{"name": n, "address": f"{a:08x}"} for a, n in ordered],
            indent=0,
        )
        + "\n"
    )

    tsv_path = out / "functions.tsv"
    tsv_path.write_text("".join(f"{a:08x}\t{n}\n" for a, n in ordered))

    print(f"wrote {len(ordered)} functions -> {json_path} and {tsv_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
