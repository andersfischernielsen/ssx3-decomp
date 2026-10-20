#!/usr/bin/env python3
"""Enriched reference dossiers for phase-2 porting waves.

    python3 tools/refdossier2.py OUTDIR 0xADDR [0xADDR...]

Like refdossier.py, plus a table of the reference body's callees and globals
translated to addresses, with the generated Rust location of each callee. A
worker can then map the readable C++ onto our runtime without hunting through
`generated/ee`.
"""
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from refsrc import REF, load_symbols  # noqa: E402
from refbody import find_impl, find_asm, find_decl  # noqa: E402

KEYWORDS = {
    "if", "for", "while", "switch", "return", "sizeof", "cast", "do", "else",
    "void", "int", "float", "char", "unsigned", "signed", "short", "long",
    "struct", "union", "enum", "typedef", "static", "const", "extern",
    "inline", "asm", "goto", "case", "break", "continue", "default", "double",
}


def generated_location(addr, generated_root):
    name = f"f_{addr:08x}"
    for shard in sorted(os.listdir(generated_root)):
        p = os.path.join(generated_root, shard, "src", "lib.rs")
        if not os.path.exists(p):
            continue
        for i, line in enumerate(open(p, errors="replace"), 1):
            if f"pub fn {name}(" in line:
                return os.path.relpath(p, os.path.dirname(generated_root)), i
    return None


def callee_table(body, syms, by_name, gen_root):
    rows = []
    seen = set()
    for m in re.finditer(r"\b([A-Za-z_]\w*)\s*\(", body):
        ident = m.group(1)
        if ident in KEYWORDS or ident in seen:
            continue
        seen.add(ident)
        if ident.startswith("func_") and len(ident) == 13:
            try:
                addr = int(ident[5:], 16)
            except ValueError:
                continue
        else:
            addr = by_name.get(ident)
            if addr is None:
                continue
        gen = generated_location(addr, gen_root)
        where = f"`{gen[0]}:{gen[1]}`" if gen else "(no generated shard: library/SDK?)"
        rows.append((ident, addr, where))
    if not rows:
        return []
    out = ["", "## Callees (reference name -> address -> generated Rust)", "",
           "| reference | address | generated |", "| --- | --- | --- |"]
    for ident, addr, where in rows:
        out.append(f"| `{ident}` | {addr:#010x} | {where} |")
    return out


def global_table(body, syms):
    rows = []
    seen = set()
    for m in re.finditer(r"\b(D_[0-9A-Fa-f]{8})\b", body):
        if m.group(1) in seen:
            continue
        seen.add(m.group(1))
        rows.append((m.group(1), int(m.group(1)[2:], 16)))
    if not rows:
        return []
    out = ["", "## Globals referenced", ""]
    for name, addr in rows:
        out.append(f"- `{name}` = {addr:#010x}")
    return out


def main(argv):
    outdir, addrs = argv[0], argv[1:]
    os.makedirs(outdir, exist_ok=True)
    syms = load_symbols()
    by_name = {}
    for a, n in syms.items():
        by_name.setdefault(n, a)
    gen_root = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
                            "generated", "ee")
    for arg in addrs:
        a = int(arg, 16)
        name = syms.get(a)
        impl = find_impl(name) if name else None
        decl = find_decl(name) if name else None
        asm = find_asm(name) if name else None
        gen = generated_location(a, gen_root)
        lines = [f"# {a:#010x} `{name or '?'}`", ""]
        if impl:
            path, line, body, _ = impl
            lines += [f"Reference C++: `{path}:{line}` (same executable; a",
                      "readability aid, not the authority).", ""]
            open(os.path.join(outdir, f"{a:#010x}.cpp"), "w").write(body.strip("\n") + "\n")
            lines += [f"Body: `{a:#010x}.cpp`", ""]
            lines += callee_table(body, syms, by_name, gen_root)
            lines += global_table(body, syms)
        elif asm:
            lines += [f"Reference has only assembly (`{asm[0]}:{asm[1]}`).", ""]
        else:
            lines += ["Reference has no implementation (not in its symtab).", ""]
        if decl:
            lines += ["", "Declaration:", "", "```c++", decl[2], "```"]
        if gen:
            lines += ["", f"Generated Rust (the authority): `{gen[0]}:{gen[1]}`"]
        open(os.path.join(outdir, f"{a:#010x}.md"), "w").write("\n".join(lines) + "\n")
        print(f"{a:#010x} {name or '?':40s} {'C++' if impl else ('asm' if asm else 'none'):4s}")


if __name__ == "__main__":
    main(sys.argv[1:])
