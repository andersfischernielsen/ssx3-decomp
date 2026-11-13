#!/usr/bin/env python3
"""Enriched reference dossiers for phase-2 porting waves.

    python3 tools/refdossier2.py OUTDIR 0xADDR [0xADDR...]

Like refdossier.py, plus a table of the reference body's callees and globals
translated to addresses, with the generated Rust location of each callee. A
worker can then map the readable C++ onto our runtime without hunting through
`generated/ee`.

A function DevvyDont has only as asm (or not at all) takes its C++ body from
the fallback reference, the peak3project fork (`SSX3_REF2`, refsrc.py), when
that has one; the dossier says so, and its callee table maps the fork's names.
Functions with no C++ body in either reference get Ghidra's decompilation
(`0xADDR.ghidra.c`, pseudo-C) when the GhidraMCP backend is up
(`SSX3_GHIDRA`, default http://127.0.0.1:8080, SLUS_207.72 open in a
CodeBrowser); without it the dossier says so and the step is skipped.
"""
import json
import os
import re
import sys
import urllib.request

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from refsrc import LABEL2, REF, REF2, load_symbols  # noqa: E402
from refbody import find_asm, find_decl, find_impl_named, impl_for  # noqa: E402

KEYWORDS = {
    "if", "for", "while", "switch", "return", "sizeof", "cast", "do", "else",
    "void", "int", "float", "char", "unsigned", "signed", "short", "long",
    "struct", "union", "enum", "typedef", "static", "const", "extern",
    "inline", "asm", "goto", "case", "break", "continue", "default", "double",
}


GHIDRA = os.environ.get("SSX3_GHIDRA", "http://127.0.0.1:8080")


def ghidra_decompile(addr):
    """Ghidra's pseudo-C for the function at `addr`, or None."""
    try:
        with urllib.request.urlopen(f"{GHIDRA}/decompile_function?address={addr:#x}", timeout=60) as r:
            reply = json.load(r)
    except (OSError, ValueError):
        return None
    if not reply.get("success"):
        return None
    return (reply.get("data") or {}).get("code")


def ontour_names():
    """address -> (On Tour name, DWARF signature, unit, tier) from the
    reference's docs/ontour_names.tsv (names and signatures from SSX On Tour's
    unstripped PSP dev builds, transferred to SSX 3 by evidence)."""
    out = {}
    try:
        lines = open(os.path.join(REF, "docs", "ontour_names.tsv"), errors="replace").read().splitlines()
    except OSError:
        return out
    cols = None
    for line in lines:
        if line.startswith("#"):
            continue
        f = line.split("\t")
        if cols is None:
            cols = {c: i for i, c in enumerate(f)}
            continue
        try:
            out[int(f[cols["address"]], 16)] = tuple(
                f[cols[c]] for c in ("ontour_name", "ontour_signature", "ontour_unit", "tier"))
        except (KeyError, ValueError, IndexError):
            pass
    return out


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


def helper_table(body, path, by_name):
    """Unaddressed functions the body calls that its file defines (static
    inline helpers such as `updateMatSeg_A0B0`): name and line, so their
    bodies can be read too."""
    try:
        text = open(path, errors="replace").read()
    except OSError:
        return []
    rows, seen = [], set()
    for m in re.finditer(r"\b([A-Za-z_]\w*)\s*\(", body):
        ident = m.group(1)
        if ident in KEYWORDS or ident in seen or ident in by_name or ident.startswith("func_"):
            continue
        seen.add(ident)
        hit = find_impl_named(ident, [(path, text)])
        if hit:
            rows.append(f"- `{ident}`: `{path}:{hit[1]}`")
    return ["", "## Local helpers (defined in the same file, inlined)", ""] + rows if rows else []


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
    ontour = ontour_names()
    fork_syms, fork_by_name = None, {}
    for arg in addrs:
        a = int(arg, 16)
        # (Functions without a symbol are `func_XXXXXXXX` in the reference.)
        name = syms.get(a) or f"func_{a:08X}"
        ref, ref_name, impl = impl_for(a, name)
        decl = find_decl(name)
        if ref != REF:
            decl = find_decl(ref_name, REF2) or decl
        asm = find_asm(name)
        gen = generated_location(a, gen_root)
        lines = [f"# {a:#010x} `{name or '?'}`", ""]
        if a in ontour:
            ot_name, ot_sig, ot_unit, tier = ontour[a]
            lines += [f"On Tour dev-build symbol (tier {tier}, `{ot_unit}`): `{ot_name}`",
                      f"signature: `{ot_sig}`", ""]
        if impl and ref != REF:
            path, line, body, _ = impl
            primary = f"only assembly (`{asm[0]}:{asm[1]}`)" if asm else "no implementation"
            lines += [f"DevvyDont reference has {primary}.", "",
                      f"Reference C++ from the {LABEL2} (`{REF2}`; same executable,",
                      f"named `{ref_name}` there): `{path}:{line}` (a readability",
                      "aid, not the authority).", ""]
            open(os.path.join(outdir, f"{a:#010x}.cpp"), "w").write(body.strip("\n") + "\n")
            lines += [f"Body: `{a:#010x}.cpp` (from the {LABEL2})", ""]
            if fork_syms is None:
                fork_syms = load_symbols(REF2)
                for fa, fn in fork_syms.items():
                    fork_by_name.setdefault(fn, fa)
            lines += callee_table(body, fork_syms, fork_by_name, gen_root)
            lines += helper_table(body, path, fork_by_name)
            lines += global_table(body, fork_syms)
        elif impl:
            path, line, body, _ = impl
            lines += [f"Reference C++: `{path}:{line}` (same executable; a",
                      "readability aid, not the authority).", ""]
            open(os.path.join(outdir, f"{a:#010x}.cpp"), "w").write(body.strip("\n") + "\n")
            lines += [f"Body: `{a:#010x}.cpp`", ""]
            lines += callee_table(body, syms, by_name, gen_root)
            lines += helper_table(body, path, by_name)
            lines += global_table(body, syms)
        elif asm:
            lines += [f"Reference has only assembly (`{asm[0]}:{asm[1]}`).", ""]
        else:
            lines += ["Reference has no implementation (not in its symtab).", ""]
        if not impl:
            code = ghidra_decompile(a)
            if code:
                open(os.path.join(outdir, f"{a:#010x}.ghidra.c"), "w").write(code.strip("\n") + "\n")
                lines += [f"Ghidra decompilation: `{a:#010x}.ghidra.c` (pseudo-C from the",
                          "Ghidra project: control flow and calls are reliable, types and",
                          "names are guesses; registers, cycles and order come from the",
                          "generated Rust).", ""]
            else:
                lines += ["(No Ghidra decompilation: backend not reachable.)", ""]
        if decl:
            lines += ["", "Declaration:", "", "```c++", decl[2], "```"]
        if gen:
            lines += ["", f"Generated Rust (the authority): `{gen[0]}:{gen[1]}`"]
        open(os.path.join(outdir, f"{a:#010x}.md"), "w").write("\n".join(lines) + "\n")
        kind = ('C++' + ('(peak3)' if ref != REF else '')) if impl else ('asm' if asm else 'none')
        if not impl and os.path.exists(os.path.join(outdir, f"{a:#010x}.ghidra.c")):
            kind += '+ghidra'
        print(f"{a:#010x} {name or '?':40s} {kind}")


if __name__ == "__main__":
    main(sys.argv[1:])
