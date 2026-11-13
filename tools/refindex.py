#!/usr/bin/env python3
"""Index the reference C++ decompilation: name -> implementation location.

    python3 tools/refindex.py [--out FILE]   # default: tools/.refindex.json
                                             # (+ tools/.refindex2.json)

Scans `~/Desktop/devvydont-ssx3/src/**/*.{cpp,c}` once. For every function it
records whether it has a C++ body (inside `#ifdef SKIP_ASM` or otherwise) or
only `INCLUDE_ASM`. The index lets wave selection ask "does the reference have
readable source for this address?" without rescanning the tree per address.
A function's entry is its built unit (`config/ssx3_us.yaml`); unbuilt files
in src/ only fill in names no built unit has. `refsrc.py`, `refbody.py` and
`progress.py` read the index: rerun this after updating the reference.

With the fallback reference present (the peak3project fork, `SSX3_REF2`,
default `~/Desktop/peak3-ssx3`; same layout) it also writes its index,
`tools/.refindex2.json` (`--out` names only the primary's).
"""
import bisect
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from refsrc import REFS, is_built  # noqa: E402

ASM_RE = re.compile(r'INCLUDE_ASM\("[^"]*",\s*([A-Za-z_]\w*)\)')


def name_before(code, k):
    """(identifier, its offset) ending just before `code[k]` (whitespace
    allowed in between), or (None, 0)."""
    e = k
    while e > 0 and code[e - 1] in " \t\r\n":
        e -= 1
    b = e
    while b > 0 and (code[b - 1].isalnum() or code[b - 1] == "_"):
        b -= 1
    while b < e and code[b].isdigit():
        b += 1
    return (code[b:e], b) if b < e else (None, 0)


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
    newlines = [i for i, c in enumerate(text) if c == "\n"]

    def line_at(offset):
        return bisect.bisect_left(newlines, offset) + 1

    # INCLUDE_ASM sites.
    for m in ASM_RE.finditer(text):
        impl.setdefault(m.group(1), ("asm", path, line_at(m.start())))
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
        name, start = name_before(code, k) if k >= 0 else (None, 0)
        if not name or name in ("if", "for", "while", "switch", "return", "sizeof"):
            opaque.append(False)
            continue
        opaque.append(True)
        depth += 1
        line = line_at(start)
        # A definition wins over a plain INCLUDE_ASM for the same name.
        impl[name] = ("c++", path, line)
    # `T f_impl(...) __asm__("func_X")`: f_impl's body is func_X's (used
    # only where no file defines func_X itself; see main).
    # (Matched on `text`: `code` blanks the string literal; a match inside a
    # comment is skipped.)
    aliases = {}
    for m in ALIAS_RE.finditer(text):
        if code[m.start()] != text[m.start()]:
            continue
        body = impl.get(m.group(1))
        if body and body[0] == "c++":
            aliases.setdefault(m.group(2), body)
    return impl, aliases


def main(argv):
    for i, (ref, out, label) in enumerate(REFS):
        if i == 0 and "--out" in argv:
            out = argv[argv.index("--out") + 1]
        build(ref, out, label)


def build(ref, out, label):
    # Built units first; files no build uses only fill in names the built
    # units lack (refsrc.built_units).
    paths = []
    for root, dirs, files in os.walk(os.path.join(ref, "src")):
        dirs.sort()
        paths += [os.path.join(root, f) for f in sorted(files) if f.endswith((".cpp", ".c"))]
    paths.sort(key=lambda p: not is_built(p, ref))
    table, aliased, unbuilt = {}, {}, set()
    for p in paths:
        built = is_built(p, ref)
        text = open(p, errors="replace").read()
        impl, aliases = index_file(p, text)
        for name, v in impl.items():
            if name in table and name not in unbuilt and not built:
                continue
            if v[0] == "c++" or name not in table:
                table[name] = v
                if not built:
                    unbuilt.add(name)
                else:
                    unbuilt.discard(name)
        for name, v in aliases.items():
            aliased.setdefault(name, v)
    for name, v in aliased.items():
        if name not in table or table[name][0] != "c++":
            table[name] = v
    json.dump(table, open(out, "w"))
    cpp = sum(1 for v in table.values() if v[0] == "c++")
    print(f"{label}: {len(table)} names indexed ({cpp} with C++, {len(table) - cpp} asm only) -> {out}")


if __name__ == "__main__":
    main(sys.argv[1:])
