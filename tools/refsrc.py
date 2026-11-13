#!/usr/bin/env python3
"""Look up the DevvyDont C++ decompilation for an SSX 3 function address.

The reference tree is a byte-matching C++ decompilation of the same executable
(91.3% of its functions matched, 13,782 / 15,099, as of 2026-11-12). For an
address it gives either

  * the C++ source file and line of its body (the most readable ground truth
    there is; matched bodies sit under ``#ifdef SKIP_ASM`` after the unit's
    ``INCLUDE_ASM``), or
  * the ``INCLUDE_ASM`` site (the function is still assembly there), or
  * nothing (not in that project).

Unnamed functions are ``func_XXXXXXXX`` there and mostly in no config symbol
file, so an address without a symbol is looked up under that name. Only the
units the reference builds count (``config/ssx3_us.yaml``; src/ also holds
unbuilt files with stale copies). Lookups go through ``tools/.refindex.json``
(``refindex.py``): regenerate it after updating the reference.

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


UNIT_RE = re.compile(r"^\s*-\s*\[0x[0-9A-Fa-f]+,\s*(?:cpp|c)\s*,\s*([^\]\s,]+)", re.M)


def built_units():
    """The units the reference builds ('dir/unit', from the splat config
    `config/ssx3_us.yaml`), or None without it. src/ also holds ~180 files
    no build uses (prototype file names, superseded splits such as
    replay/replay2.cpp beside the built replay/replaycache.cpp) with stale
    `INCLUDE_ASM` copies; a function's unit is the built one."""
    if not hasattr(built_units, "units"):
        try:
            built_units.units = set(UNIT_RE.findall(open(os.path.join(REF, "config", "ssx3_us.yaml")).read()))
        except OSError:
            built_units.units = None
    return built_units.units


def is_built(path):
    """Whether a src/ file (absolute or REF-relative) is a built unit."""
    units = built_units()
    if units is None:
        return True
    rel = os.path.relpath(path, os.path.join(REF, "src")) if os.path.isabs(path) else path.removeprefix("src/")
    return rel.rsplit(".", 1)[0] in units


INDEX = os.path.join(os.path.dirname(os.path.abspath(__file__)), ".refindex.json")


def load_index():
    """name -> (kind, path, line) from `tools/.refindex.json` (refindex.py),
    or {} when there is none. Regenerate it after updating the reference."""
    if not hasattr(load_index, "table"):
        try:
            import json
            load_index.table = json.load(open(INDEX))
            sample = next(iter(load_index.table.values()), None)
            if sample and not sample[1].startswith(os.path.join(REF, "")):
                load_index.table = {}  # indexed from another tree (SSX3_REF)
            head = os.path.join(REF, ".git", "index")
            if load_index.table and os.path.exists(head) and os.path.getmtime(head) > os.path.getmtime(INDEX):
                print("refsrc: tools/.refindex.json is older than the reference checkout;"
                      " run tools/refindex.py", file=sys.stderr)
        except (OSError, ValueError):
            load_index.table = {}
    return load_index.table


def find_source(name):
    """(file, line, kind) for the function's definition, or None. `kind` is
    "c++" when the reference has a C++ body (also a matched one kept under
    `#ifdef SKIP_ASM` after its `INCLUDE_ASM`), else "asm"."""
    hit = load_index().get(name)
    if hit:
        kind, path, line = hit
        return os.path.relpath(path, REF), line, kind
    try:
        r = subprocess.run(
            ["grep", "-rn", "--include=*.cpp", "--include=*.c", "-w", name, "src"],
            cwd=REF, capture_output=True, text=True, timeout=30)
    except subprocess.TimeoutExpired:
        return None
    best = None
    lines = r.stdout.splitlines()
    for line in sorted(lines, key=lambda l: not is_built(l.split(":", 1)[0])):
        path, ln, code = line.split(":", 2)
        ln = int(ln)
        if "INCLUDE_ASM" in code:
            if best is None:
                best = (path, ln, "asm")
            continue
        # A definition: the name starts a parameter list and the line is
        # not a declaration, call or assignment.
        head = code.split(name, 1)[1].lstrip() if name in code else ""
        if head.startswith("(") and ";" not in code and "=" not in code.split(name, 1)[0]:
            return (path, ln, "c++")
    return best


def lookup(syms, addr):
    """(name, source) for `addr`: its symbol, else the reference's
    `func_XXXXXXXX` (most unnamed functions are in no config symbol file,
    only in an `INCLUDE_ASM`/definition in src/). (None, None) if unknown."""
    name = syms.get(addr)
    if name is not None:
        return name, find_source(name)
    name = f"func_{addr:08X}"
    src = find_source(name)
    return (name, src) if src else (None, None)


def describe(addr):
    syms = describe.syms if hasattr(describe, "syms") else load_symbols()
    describe.syms = syms
    name, src = lookup(syms, addr)
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
        name, src = lookup(syms, a)
        if name is None:
            print(f"{a:#010x}\t?\t(not in the reference symtab)")
            continue
        if src:
            path, ln, kind = src
            where = f"{path}:{ln}" + (" (asm)" if kind == "asm" else "")
        else:
            where = "(source not found)"
        if src and src[2] == "c++":
            matched += 1
        print(f"{a:#010x}\t{name}\t{where}")
    if len(addrs) > 1:
        print(f"# {matched}/{len(addrs)} with readable C++ source", file=sys.stderr)


if __name__ == "__main__":
    main(sys.argv[1:])
