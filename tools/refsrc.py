#!/usr/bin/env python3
"""Look up the DevvyDont C++ decompilation for an SSX 3 function address.

The reference tree is a byte-matching C++ decompilation of the same executable
(84.5% matched at the time of writing). For an address it gives either

  * the C++ source file and line of a matched function (the most readable
    ground truth there is), or
  * the ``INCLUDE_ASM`` site (the function is still assembly there), or
  * nothing (not in that project).

Usage:
  python3 tools/refsrc.py 0x00122278 [more addresses...]
  python3 tools/refsrc.py --tsv file.tsv     # first column = address

The reference root defaults to ``~/Desktop/devvydont-ssx3`` (override with
SSX3_REF=/path).
"""
import os
import re
import subprocess
import sys

REF = os.environ.get("SSX3_REF", os.path.expanduser("~/Desktop/devvydont-ssx3"))

SYMBOL_RE = re.compile(r"^(\S+)\s*=\s*(0x[0-9A-Fa-f]+)\s*;", re.M)


def load_symbols():
    """address (int) -> name, from every config symbol file."""
    out = {}
    cfg = os.path.join(REF, "config")
    if not os.path.isdir(cfg):
        sys.exit(f"no reference tree at {REF} (set SSX3_REF)")
    for fn in sorted(os.listdir(cfg)):
        if "symbol" not in fn or not fn.endswith(".txt"):
            continue
        text = open(os.path.join(cfg, fn), errors="replace").read()
        for name, addr in SYMBOL_RE.findall(text):
            # Prefer a real name over func_XXXXXXXX.
            a = int(addr, 16)
            if a not in out or (out[a].startswith("func_") and not name.startswith("func_")):
                out[a] = name
    return out


def find_source(name):
    """(file, line, kind) for the function's definition, or None."""
    try:
        r = subprocess.run(
            ["grep", "-rn", "--include=*.cpp", "--include=*.h", "-m1", name, "src", "include"],
            cwd=REF, capture_output=True, text=True, timeout=30)
    except subprocess.TimeoutExpired:
        return None
    best = None
    for line in r.stdout.splitlines():
        path, ln, code = line.split(":", 2)
        ln = int(ln)
        if name not in code:
            continue
        kind = "asm" if "INCLUDE_ASM" in code else "c++"
        # Prefer a definition line (C++ body or INCLUDE_ASM) over declarations.
        if best is None or (kind == "asm" and best[2] != "asm"):
            best = (path, ln, kind)
        if kind == "c++" and ("(" in code) and not code.rstrip().endswith(";"):
            return (path, ln, kind)
    return best


def describe(addr):
    syms = describe.syms if hasattr(describe, "syms") else load_symbols()
    describe.syms = syms
    name = syms.get(addr)
    if name is None:
        return addr, None, None, None
    src = find_source(name)
    return addr, name, src, None


def main(argv):
    args = [a for a in argv if not a.startswith("-")]
    if "--tsv" in argv:
        rows = []
        for fn in args:
            for line in open(fn):
                line = line.strip()
                if not line or line.startswith("#"):
                    continue
                a = line.split("\t")[0]
                try:
                    rows.append(int(a, 16))
                except ValueError:
                    pass
        addrs = rows
    else:
        try:
            addrs = [int(a, 16) for a in args]
        except ValueError:
            sys.exit(__doc__)

    syms = load_symbols()
    matched = 0
    for a in addrs:
        name = syms.get(a)
        if name is None:
            print(f"{a:#010x}\t?\t(not in the reference symtab)")
            continue
        src = find_source(name)
        named = not name.startswith("func_")
        if src:
            path, ln, kind = src
            where = f"{path}:{ln}" + (" (asm)" if kind == "asm" else "")
        else:
            where = "(source not found)"
        if named and src and src[2] == "c++":
            matched += 1
        print(f"{a:#010x}\t{name}\t{where}")
    if len(addrs) > 1:
        print(f"# {matched}/{len(addrs)} with readable C++ source", file=sys.stderr)


if __name__ == "__main__":
    main(sys.argv[1:])
