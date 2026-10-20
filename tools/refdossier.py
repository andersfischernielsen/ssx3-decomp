#!/usr/bin/env python3
"""Make reference dossiers for phase-2 porting waves.

    python3 tools/refdossier.py OUTDIR 0xADDR [0xADDR...]

For each address writes OUTDIR/0xADDR.md containing the reference project's
symbol, its C++ decompilation (when it has one), the declaration, and where the
generated Rust is. OUTDIR/0xADDR.cpp holds just the extracted body. A worker
uses the C++ to understand the game's own logic; `generated/ee` remains the
authority for registers, cycles and memory.
"""
import os
import re
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from refsrc import REF, load_symbols  # noqa: E402
from refbody import find_impl, find_asm, find_decl  # noqa: E402


def generated_location(addr, generated_root):
    """File:line of the generated Rust function for an address."""
    name = f"f_{addr:08x}"
    for shard in sorted(os.listdir(generated_root)):
        p = os.path.join(generated_root, shard, "src", "lib.rs")
        if not os.path.exists(p):
            continue
        for i, line in enumerate(open(p, errors="replace"), 1):
            if f"pub fn {name}(" in line:
                kind = "generated"
                if "__generated(" in line:
                    kind = "generated"
                return p, i, kind
            if f"pub fn {name}__generated(" in line:
                return p, i, "generated"
    return None


def main(argv):
    outdir, addrs = argv[0], argv[1:]
    os.makedirs(outdir, exist_ok=True)
    syms = load_symbols()
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
        elif asm:
            lines += [f"Reference has only assembly (`{asm[0]}:{asm[1]}`).", ""]
        else:
            lines += ["Reference has no implementation (not in its symtab).", ""]
        if decl:
            lines += ["Declaration:", "", "```c++", decl[2], "```", ""]
        if gen:
            lines += [f"Generated Rust (the authority): `{gen[0]}:{gen[1]}`", ""]
        open(os.path.join(outdir, f"{a:#010x}.md"), "w").write("\n".join(lines))
        print(f"{a:#010x} {name or '?':40s} {'C++' if impl else ('asm' if asm else 'none'):4s} {os.path.join(outdir, f'{a:#010x}.md')}")


if __name__ == "__main__":
    main(sys.argv[1:])
