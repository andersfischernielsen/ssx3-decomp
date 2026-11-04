#!/usr/bin/env python3
"""Index the reference C++ decompilation: name -> implementation location.

    python3 tools/refindex.py [--out FILE]   # default: tools/.refindex.json

Scans `~/Desktop/devvydont-ssx3/src/**/*.{cpp,c}` once. For every function it
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


ALIAS_RE = re.compile(r'([A-Za-z_]\w*)\s*\([^;{}]*\)\s*__asm__\s*\(\s*"([A-Za-z_]\w*)"\s*\)')


def code_only(text):
    """`text` with comments and string/char literals blanked (same length,
    newlines kept), so braces and parentheses can be counted."""
    out = list(text)
    i, n = 0, len(text)
    while i < n:
        c = text[i]
        if text.startswith("//", i):
            j = text.find("\n", i)
            j = n if j < 0 else j
            for k in range(i, j):
                out[k] = " "
            i = j
        elif text.startswith("/*", i):
            j = text.find("*/", i + 2)
            j = n if j < 0 else j + 2
            for k in range(i, j):
                if text[k] != "\n":
                    out[k] = " "
            i = j
        elif c in "\"'":
            j = i + 1
            while j < n and text[j] != c and text[j] != "\n":
                j += 2 if text[j] == "\\" else 1
            for k in range(i + 1, min(j, n)):
                out[k] = " "
            i = j + 1
        else:
            i += 1
    return "".join(out)


def index_file(path, text):
    impl = {}
    # INCLUDE_ASM sites.
    for m in ASM_RE.finditer(text):
        impl.setdefault(m.group(1), ("asm", path, text.count("\n", 0, m.start()) + 1))
    # Definitions: a `{` at brace depth 0 (outside any body; `extern "C"`
    # and namespace blocks count as depth 0) after a parameter list.
    code = code_only(text)
    depth = 0
    opaque = []  # per open brace: whether it opened a function body
    for i, ch in enumerate(code):
        if ch == "}":
            if opaque:
                if opaque.pop():
                    depth -= 1
            continue
        if ch != "{":
            continue
        if depth > 0:
            opaque.append(False)
            continue
        j = i - 1
        while j >= 0 and code[j] in " \t\r\n":
            j -= 1
        # (`) const {`, `) noexcept {`: one trailing word allowed.)
        if j >= 0 and code[j] != ")":
            w = j
            while w >= 0 and (code[w].isalnum() or code[w] == "_"):
                w -= 1
            if code[w + 1 : j + 1] in ("const", "noexcept", "override"):
                j = w
                while j >= 0 and code[j] in " \t\r\n":
                    j -= 1
        if j < 0 or code[j] != ")":
            opaque.append(False)  # namespace, extern "C", struct, ...
            continue
        k, d = j, 0
        while k >= 0:
            if code[k] == ")":
                d += 1
            elif code[k] == "(":
                d -= 1
                if d == 0:
                    break
            k -= 1
        nm = NAME_BEFORE_PAREN.search(code[:k]) if k >= 0 else None
        if not nm or nm.group(1) in ("if", "for", "while", "switch", "return", "sizeof"):
            opaque.append(False)
            continue
        opaque.append(True)
        depth += 1
        name = nm.group(1)
        line = text.count("\n", 0, nm.start(1)) + 1
        # A definition wins over a plain INCLUDE_ASM for the same name.
        impl[name] = ("c++", path, line)
    # `T f_impl(...) __asm__("func_X")`: f_impl's body is func_X's.
    for m in ALIAS_RE.finditer(code):
        body = impl.get(m.group(1))
        if body and body[0] == "c++":
            impl[m.group(2)] = body
    return impl


def main(argv):
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), ".refindex.json")
    if "--out" in argv:
        out = argv[argv.index("--out") + 1]
    table = {}
    for root, _dirs, files in os.walk(os.path.join(REF, "src")):
        for f in files:
            if not f.endswith((".cpp", ".c")):
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
