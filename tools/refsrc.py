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

Fallback reference: the ``peak3project/ssx3`` fork (~97% matched, the same
layout; DevvyDont merges from it) at ``~/Desktop/peak3-ssx3`` (override with
SSX3_REF2=/path; absent -> everything behaves as with DevvyDont alone).
DevvyDont stays primary: its names and units are what this tool prints and
what reorg_modules.py places by. When DevvyDont has an address only as asm
(or not at all) and the fork has a C++ body, a fourth column says so:
``fallback: peak3project fork <name> src/...:<line>``. The fork has its own
index, ``tools/.refindex2.json`` (refindex.py builds both).
"""
import os
import re
import subprocess
import sys

REF = os.environ.get("SSX3_REF", os.path.expanduser("~/Desktop/devvydont-ssx3"))
REF2 = os.environ.get("SSX3_REF2", os.path.expanduser("~/Desktop/peak3-ssx3"))
if not os.path.isdir(os.path.join(REF2, "config")):
    REF2 = None  # no fallback reference

SYMBOL_RE = re.compile(r"^(\S+)\s*=\s*(0x[0-9A-Fa-f]+)\s*;", re.M)


def load_symbols(root=None):
    """address (int) -> name, from every config symbol file of the reference
    at `root` (default: the primary, REF)."""
    root = root or REF
    out = {}
    cfg = os.path.join(root, "config")
    if not os.path.isdir(cfg):
        sys.exit(f"no reference tree at {root} (set SSX3_REF / SSX3_REF2)")
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


def built_units(root=None):
    """The units the reference at `root` (default REF) builds ('dir/unit', from the splat config
    `config/ssx3_us.yaml`), or None without it. src/ also holds ~180 files
    no build uses (prototype file names, superseded splits such as
    replay/replay2.cpp beside the built replay/replaycache.cpp) with stale
    `INCLUDE_ASM` copies; a function's unit is the built one."""
    root = root or REF
    cache = built_units.__dict__.setdefault("units", {})
    if root not in cache:
        try:
            cache[root] = set(UNIT_RE.findall(open(os.path.join(root, "config", "ssx3_us.yaml")).read()))
        except OSError:
            cache[root] = None
    return cache[root]


def is_built(path, root=None):
    """Whether a src/ file (absolute or root-relative) is a built unit of the
    reference at `root` (default REF)."""
    root = root or REF
    units = built_units(root)
    if units is None:
        return True
    rel = os.path.relpath(path, os.path.join(root, "src")) if os.path.isabs(path) else path.removeprefix("src/")
    return rel.rsplit(".", 1)[0] in units


TOOLS = os.path.dirname(os.path.abspath(__file__))
INDEX = os.path.join(TOOLS, ".refindex.json")
INDEX2 = os.path.join(TOOLS, ".refindex2.json")
LABEL = "DevvyDont"
LABEL2 = "peak3project fork"
# (root, index file, label) of each reference, the primary first.
REFS = [(REF, INDEX, LABEL)] + ([(REF2, INDEX2, LABEL2)] if REF2 else [])


def label_of(root):
    return next((lab for r, _i, lab in REFS if r == root), root)


def load_index(root=None):
    """name -> (kind, path, line) from the reference's index (refindex.py:
    `tools/.refindex.json` for REF, `tools/.refindex2.json` for REF2), or {}
    when there is none. Regenerate it after updating the reference."""
    root = root or REF
    cache = load_index.__dict__.setdefault("tables", {})
    if root not in cache:
        index = INDEX2 if REF2 and root == REF2 else INDEX
        try:
            import json
            table = json.load(open(index))
            sample = next(iter(table.values()), None)
            if sample and not sample[1].startswith(os.path.join(root, "")):
                table = {}  # indexed from another tree (SSX3_REF / SSX3_REF2)
            head = os.path.join(root, ".git", "index")
            if table and os.path.exists(head) and os.path.getmtime(head) > os.path.getmtime(index):
                print(f"refsrc: tools/{os.path.basename(index)} is older than the {label_of(root)}"
                      " checkout; run tools/refindex.py", file=sys.stderr)
        except (OSError, ValueError):
            table = {}
        cache[root] = table
    return cache[root]


def find_source(name, root=None):
    """(file, line, kind) for the function's definition in the reference at
    `root` (default REF), or None. `kind` is "c++" when the reference has a
    C++ body (also a matched one kept under `#ifdef SKIP_ASM` after its
    `INCLUDE_ASM`), else "asm"."""
    root = root or REF
    hit = load_index(root).get(name)
    if hit:
        kind, path, line = hit
        return os.path.relpath(path, root), line, kind
    if root != REF and load_index(root):
        return None  # (fallback: the index covers its names; no slow grep)
    try:
        r = subprocess.run(
            ["grep", "-rn", "--include=*.cpp", "--include=*.c", "-w", name, "src"],
            cwd=root, capture_output=True, text=True, timeout=30)
    except subprocess.TimeoutExpired:
        return None
    best = None
    lines = r.stdout.splitlines()
    for line in sorted(lines, key=lambda l: not is_built(l.split(":", 1)[0], root)):
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


def lookup(syms, addr, root=None):
    """(name, source) for `addr` in the reference at `root` (default REF;
    `syms` is its load_symbols): its symbol, else the reference's
    `func_XXXXXXXX` (most unnamed functions are in no config symbol file,
    only in an `INCLUDE_ASM`/definition in src/). (None, None) if unknown."""
    name = syms.get(addr)
    if name is not None:
        src = find_source(name, root)
        if src or root in (None, REF):
            return name, src
        # (The fork may know the address under its func_ name only.)
    name = f"func_{addr:08X}"
    src = find_source(name, root)
    return (name, src) if src else (None, None)


def fallback(addr, primary_src=None):
    """(name, (file, line, "c++")) of the fork's C++ body for `addr` when the
    primary has none (`primary_src` is the primary's find_source result,
    kind "asm" or None) and REF2 has one; else None. Paths are
    REF2-relative."""
    if not REF2 or (primary_src and primary_src[2] == "c++"):
        return None
    syms = fallback.__dict__.get("syms")
    if syms is None:
        syms = fallback.syms = load_symbols(REF2)
    name, src = lookup(syms, addr, REF2)
    return (name, src) if src and src[2] == "c++" else None


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

    # Columns 1-3 are DevvyDont's (reorg_modules.py places by column 3); a
    # fourth names the fork's body when only the fork has C++.
    syms = load_symbols()
    matched = matched2 = 0
    for a in addrs:
        name, src = lookup(syms, a)
        fb = fallback(a, src)
        extra = f"\tfallback: {LABEL2} {fb[0]} {fb[1][0]}:{fb[1][1]}" if fb else ""
        matched2 += bool(fb)
        if name is None:
            print(f"{a:#010x}\t?\t(not in the reference symtab){extra}")
            continue
        if src:
            path, ln, kind = src
            where = f"{path}:{ln}" + (" (asm)" if kind == "asm" else "")
        else:
            where = "(source not found)"
        if src and src[2] == "c++":
            matched += 1
        print(f"{a:#010x}\t{name}\t{where}{extra}")
    if len(addrs) > 1:
        more = f" (+{matched2} from the {LABEL2})" if REF2 else ""
        print(f"# {matched}/{len(addrs)} with readable C++ source{more}", file=sys.stderr)


if __name__ == "__main__":
    main(sys.argv[1:])
