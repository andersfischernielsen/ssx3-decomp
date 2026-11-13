#!/usr/bin/env python3
"""Extract the DevvyDont C++ decompilation of SSX 3 functions by address.

    python3 tools/refbody.py 0x00121818 [more...]

For each address it prints:
  * the reference symbol name (possibly still func_XXXXXXXX),
  * the file:line of its C++ implementation (inside `#ifdef SKIP_ASM`) or of
    its `INCLUDE_ASM`,
  * the implementation itself when there is one.

When DevvyDont has the address only as asm (or not at all) and the fallback
reference (the peak3project fork, `SSX3_REF2`, refsrc.py) has a C++ body,
that body is printed instead, its `impl:` line marked `(peak3project fork)`
and the fork's name given.

This is a readability aid: the generated Rust in `generated/ee` remains the
authority for every register, cycle and memory write.
"""
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from refsrc import LABEL, LABEL2, REF, REF2, fallback, is_built, load_index, load_symbols  # noqa: E402


def all_sources(ref=None):
    """(path -> text) for every C++ file of the reference at `ref` (default
    REF), the built units first (refsrc.built_units: the others hold stale
    INCLUDE_ASM copies)."""
    ref = ref or REF
    paths = []
    for root, dirs, files in os.walk(os.path.join(ref, "src")):
        dirs.sort()
        paths += [os.path.join(root, f) for f in sorted(files) if f.endswith((".cpp", ".c"))]
    paths.sort(key=lambda p: not is_built(p, ref))
    for p in paths:
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


def find_impl(name, ref=None):
    """(path, line, body or None, kind) for the function's implementation in
    the reference at `ref` (default REF; also through a
    `T alias(...) __asm__("name")` declaration)."""
    hit = load_index(ref).get(name)
    if hit and hit[0] == "c++":
        # Fast path: the file refindex.py found the body in.
        text = open(hit[1], errors="replace").read()
        r = find_impl_named(name, [(hit[1], text)])
        if r is None:
            m = re.search(r'([A-Za-z_]\w*)\s*\([^;{}]*\)\s*__asm__\s*\(\s*"' + re.escape(name) + r'"\s*\)', text)
            if m and m.group(1) != name:
                r = find_impl_named(m.group(1), [(hit[1], text)])
        if r:
            return r
    if ref not in (None, REF) and load_index(ref) and not hit:
        return None  # (fallback: its index knows every body; no slow scan)
    r = find_impl_named(name, ref=ref)
    if r is None:
        alias = re.compile(r'([A-Za-z_]\w*)\s*\([^;{}]*\)\s*__asm__\s*\(\s*"' + re.escape(name) + r'"\s*\)')
        for _path, text in all_sources(ref):
            m = alias.search(text)
            if m and m.group(1) != name:
                r = find_impl_named(m.group(1), ref=ref)
                if r:
                    break
    return r


def find_impl_named(name, sources=None, ref=None):
    name_re = re.compile(r"\b" + re.escape(name) + r"\b")
    for path, text in sources if sources is not None else all_sources(ref):
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


def find_asm(name, ref=None):
    pat = re.compile(r'INCLUDE_ASM\("[^"]*",\s*' + re.escape(name) + r"\)")
    for path, text in all_sources(ref):
        m = pat.search(text)
        if m:
            return path, line_of(text, m.start())
    return None


def find_decl(name, ref=None):
    pat = re.compile(r"^[^\n;]*\b" + re.escape(name) + r"\s*\([^;]*\)\s*;", re.M)
    for root in ("include", "src"):
        for dirpath, _d, files in os.walk(os.path.join(ref or REF, root)):
            for f in files:
                if not f.endswith((".h", ".cpp", ".c")):
                    continue
                p = os.path.join(dirpath, f)
                text = open(p, errors="replace").read()
                m = pat.search(text)
                if m:
                    return p, line_of(text, m.start()), m.group(0).strip()
    return None


def impl_for(addr, name):
    """(ref root, name there, find_impl result or None) for `addr`, whose
    primary (DevvyDont) name is `name`: DevvyDont's body when it has one,
    else the fallback reference's (refsrc.fallback), else (REF, name, None)."""
    impl = find_impl(name)
    if impl or not REF2:
        return REF, name, impl
    fb = fallback(addr)
    if fb:
        impl2 = find_impl(fb[0], REF2)
        if impl2:
            return REF2, fb[0], impl2
    return REF, name, None


def main(argv):
    syms = load_symbols()
    for arg in argv:
        addr = int(arg, 16)
        # (Most unnamed functions are only `func_XXXXXXXX` in src/, in no
        # config symbol file.)
        name = syms.get(addr) or f"func_{addr:08X}"
        ref, ref_name, impl = impl_for(addr, name)
        asm = find_asm(name)
        if not impl and not asm and name not in syms.values():
            print(f"===== {addr:#010x}  (not in the reference) =====")
            continue
        if ref == REF:
            print(f"===== {addr:#010x}  {name} =====")
        else:
            where = f"asm only: {asm[0]}:{asm[1]}" if asm else "not in it"
            print(f"===== {addr:#010x}  {name}  ({LABEL}: {where}) =====")
        decl = find_decl(name) if ref == REF else (find_decl(ref_name, REF2) or find_decl(name))
        if decl:
            print(f"decl: {decl[0]}:{decl[1]}: {decl[2]}")
        if impl:
            path, line, body, kind = impl
            if ref == REF:
                print(f"impl: {path}:{line}")
            else:
                print(f"impl ({LABEL2}, `{ref_name}` there): {path}:{line}")
            print(body.strip("\n"))
        elif asm := asm:
            print(f"asm only: {asm[0]}:{asm[1]}")
        else:
            print("no implementation found")
        print()


if __name__ == "__main__":
    main(sys.argv[1:])
