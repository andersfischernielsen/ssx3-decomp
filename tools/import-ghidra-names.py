#!/usr/bin/env python3
"""Re-apply function names from generated/functions.tsv to the open Ghidra program.

Use after Ghidra lost unsaved renames (e.g. a crash): the TSV export is the
committed source of truth for names. Only functions whose current Ghidra name
differs from the TSV and is a default `FUN_xxxxxxxx` name are renamed, unless
--force. Talks to the GhidraMCP loopback backend (default http://127.0.0.1:8080).
Save the program in Ghidra afterwards (Ctrl/Cmd-S).

  tools/import-ghidra-names.py [--dry-run] [--force]
"""
import argparse
import json
import pathlib
import sys
import urllib.parse
import urllib.request


def get_json(url):
    with urllib.request.urlopen(url, timeout=60) as r:
        return json.load(r)


def post(base, endpoint, **params):
    data = urllib.parse.urlencode(params).encode()
    with urllib.request.urlopen(urllib.request.Request(f"{base}/{endpoint}", data=data), timeout=60) as r:
        return json.load(r)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--base", default="http://127.0.0.1:8080")
    ap.add_argument("--tsv", default="generated/functions.tsv")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--force", action="store_true", help="also overwrite non-default Ghidra names")
    a = ap.parse_args()

    current = {}
    off = 0
    while True:
        d = get_json(f"{a.base}/functions?offset={off}&limit=2000")["data"]
        for it in d["items"]:
            current[int(it["address"], 16)] = it["name"]
        if not d.get("has_more") or not d["items"]:
            break
        off += len(d["items"])

    want = {}
    for line in pathlib.Path(a.tsv).read_text().splitlines():
        start, _end, name = line.split("\t")
        want[int(start, 16)] = name

    todo, missing, kept = [], 0, 0
    for addr, name in sorted(want.items()):
        if name.startswith("FUN_"):
            continue
        cur = current.get(addr)
        if cur is None:
            missing += 1
        elif cur != name:
            if cur.startswith("FUN_") or a.force:
                todo.append((addr, cur, name))
            else:
                kept += 1
    print(f"ghidra {len(current)} functions; tsv names {sum(1 for n in want.values() if not n.startswith('FUN_'))}; "
          f"to rename {len(todo)}; no function at tsv address {missing}; differing non-default kept {kept}",
          file=sys.stderr)
    if a.dry_run:
        return 0
    fails = 0
    for i, (addr, cur, name) in enumerate(todo, 1):
        try:
            r = post(a.base, "rename_function_by_address", function_address=f"0x{addr:08x}", new_name=name)
            if not r.get("success", True):
                fails += 1
                print(f"  fail {addr:08x} {name}: {r}", file=sys.stderr)
        except Exception as e:  # noqa: BLE001
            fails += 1
            print(f"  fail {addr:08x} {name}: {e}", file=sys.stderr)
        if i % 500 == 0:
            print(f"  {i}/{len(todo)}", file=sys.stderr)
    print(f"renamed {len(todo) - fails}, failed {fails}", file=sys.stderr)
    return 1 if fails else 0


if __name__ == "__main__":
    sys.exit(main())
