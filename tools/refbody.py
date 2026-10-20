#!/usr/bin/env python3
"""Extract the DevvyDont C++ decompilation of SSX 3 functions by address.

    python3 tools/refbody.py 0x00121818 [more...]

For each address it prints:
  * the reference symbol name (possibly still func_XXXXXXXX),
  * the file:line of its C++ implementation (inside `#ifdef SKIP_ASM`) or of
    its `INCLUDE_ASM`,
  * the implementation itself when there is one.

This is a readability aid: the generated Rust in `generated/ee` remains the
authority for every register, cycle and memory write.
"""
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from refsrc import REF, load_symbols  # noqa: E402


def all_sources():
    """(path -> text) for every C++ file of the reference."""
    for root, _dirs, files in os.walk(os.path.join(REF, "src")):
        for f in files:
            if f.endswith(".cpp"):
                p = os.path.join(root, f)
                yield p, open(p, errors="replace").read()


def line_of(text, offset):
    return text.count("\n", 0, offset) + 1


def extract_body(text, start):
    """`text[start]` is the first char after the opening brace of a function;
    return the body up to the matching close brace."""
    depth = 1
    i = start
    in_str = None
    while i < len(text):
        c = text[i]
        if in_str:
            if c == "\\":
                i += 2
                continue
            if c == in_str:
                in_str = None
        elif c in "\"'":
            in_str = c
        elif c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                return text[start:i], i
        i += 1
    return text[start:], len(text)


def find_impl(name):
    """(path, line, body or None, kind) for the function's implementation."""
    name_re = re.compile(r"\b" + re.escape(name) + r"\b")
    for path, text in all_sources():
        # A definition of `name` at the start of a line (allow attributes /
        # return type on the same line).
        for m in name_re.finditer(text):
            # The name must be followed by an argument list.
            j = m.end()
            while j < len(text) and text[j] in " \t":
                j += 1
            if j >= len(text) or text[j] != "(":
                continue
            # Look backwards to the beginning of the line: a definition is not
            # a declaration (no trailing ';' before the body) and not a call.
            ls = text.rfind("\n", 0, m.start()) + 1
            line = text[ls:j]
            if ";" in line or "=" in line or "INCLUDE_ASM" in line:
                continue
            # Find the opening brace after the parameter list.
            depth = 0
            k = j
            while k < len(text):
                if text[k] == "(":
                    depth += 1
                elif text[k] == ")":
                    depth -= 1
                    if depth == 0:
                        break
                elif text[k] == ";" and depth == 0:
                    break
                k += 1
            if k >= len(text) or text[k] != ")":
                continue
            k += 1
            while k < len(text) and text[k] in " \t\r\n":
                k += 1
            if k < len(text) and text[k] == "{":
                body, _ = extract_body(text, k + 1)
                return path, line_of(text, m.start()), body, "c++"
    return None


def find_asm(name):
    pat = re.compile(r'INCLUDE_ASM\("[^"]*",\s*' + re.escape(name) + r"\)")
    for path, text in all_sources():
        m = pat.search(text)
        if m:
            return path, line_of(text, m.start())
    return None


def find_decl(name):
    pat = re.compile(r"^[^\n;]*\b" + re.escape(name) + r"\s*\([^;]*\)\s*;", re.M)
    for root in ("include", "src"):
        for dirpath, _d, files in os.walk(os.path.join(REF, root)):
            for f in files:
                if not f.endswith((".h", ".cpp")):
                    continue
                p = os.path.join(dirpath, f)
                text = open(p, errors="replace").read()
                m = pat.search(text)
                if m:
                    return p, line_of(text, m.start()), m.group(0).strip()
    return None


def main(argv):
    syms = load_symbols()
    for arg in argv:
        addr = int(arg, 16)
        name = syms.get(addr)
        print(f"===== {addr:#010x}  {name or '(not in reference symtab)'} =====")
        if not name:
            continue
        impl = find_impl(name)
        asm = find_asm(name)
        decl = find_decl(name)
        if decl:
            print(f"decl: {decl[0]}:{decl[1]}: {decl[2]}")
        if impl:
            path, line, body, kind = impl
            print(f"impl: {path}:{line}")
            print(body.strip("\n"))
        elif asm := asm:
            print(f"asm only: {asm[0]}:{asm[1]}")
        else:
            print("no implementation found")
        print()


if __name__ == "__main__":
    main(sys.argv[1:])
