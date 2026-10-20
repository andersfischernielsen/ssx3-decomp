#!/usr/bin/env python3
"""Index the reference C++ decompilation: name -> implementation location.

    python3 tools/refindex.py [--out FILE]   # default: tools/.refindex.json

Scans `~/Desktop/devvydont-ssx3/src/**/*.cpp` once. For every function it
records whether it has a C++ body (inside `#ifdef SKIP_ASM` or otherwise) or
only `INCLUDE_ASM`. The index lets wave selection ask "does the reference have
readable source for this address?" without rescanning the tree per address.
"""
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from refsrc import REF  # noqa: E402

ASM_RE = re.compile(r'INCLUDE_ASM\("[^"]*",\s*([A-Za-z_]\w*)\)')
NAME_BEFORE_PAREN = re.compile(r"([A-Za-z_]\w*)\s*$")


def index_file(path, text):
    impl = {}
    # INCLUDE_ASM sites.
    for m in ASM_RE.finditer(text):
        impl.setdefault(m.group(1), ("asm", path, text.count("\n", 0, m.start()) + 1))
    # Definitions: a `{` at the start of a line (after whitespace), preceded by
    # a parameter list.
    for m in re.finditer(r"(?m)^[ \t]*\{", text):
        # Walk back over whitespace to the closing parenthesis.
        j = m.start() - 1
        while j >= 0 and text[j] in " \t\r\n":
            j -= 1
        if j < 0 or text[j] != ")":
            continue
        # Balance parentheses to the opening one.
        depth = 0
        k = j
        while k >= 0:
            if text[k] == ")":
                depth += 1
            elif text[k] == "(":
                depth -= 1
                if depth == 0:
                    break
            k -= 1
        if k < 0:
            continue
        before = text[:k]
        nm = NAME_BEFORE_PAREN.search(before)
        if not nm:
            continue
        name = nm.group(1)
        line = text.count("\n", 0, nm.start(1)) + 1
        # A definition wins over a plain INCLUDE_ASM for the same name.
        impl[name] = ("c++", path, line)
    return impl


def main(argv):
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), ".refindex.json")
    if "--out" in argv:
        out = argv[argv.index("--out") + 1]
    table = {}
    for root, _dirs, files in os.walk(os.path.join(REF, "src")):
        for f in files:
            if not f.endswith(".cpp"):
                continue
            p = os.path.join(root, f)
            text = open(p, errors="replace").read()
            for name, v in index_file(p, text).items():
                if v[0] == "c++" or name not in table:
                    table[name] = v
    json.dump(table, open(out, "w"))
    cpp = sum(1 for v in table.values() if v[0] == "c++")
    print(f"{len(table)} names indexed ({cpp} with C++, {len(table) - cpp} asm only) -> {out}")


if __name__ == "__main__":
    main(sys.argv[1:])
