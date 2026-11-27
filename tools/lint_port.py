#!/usr/bin/env python3
"""Lint hand-written ports against their generated bodies, without compiling.

    python3 tools/lint_port.py oc867 oc868 ...        # staging modules
    python3 tools/lint_port.py path/to/unit.rs        # any file with an OVERRIDES table

For every `OVERRIDES` entry the port (the function and the module-level helpers
it calls, tests excluded) is compared with `f_ADDR__generated` (or `f_ADDR`
before the slot exists) in `generated/ee`:

- LITERAL   a hex literal with more than 8 digits (`0x001b_babcc`): the typo
            class that cost a debug round in waves 251-253;
- ADDR      a literal inside the function's own address range that the
            generated body never uses (a typo of a block/return address);
- RA        (warning) a return address of the generated body that appears in
            the port neither whole nor as its low 16 bits (a dropped call or a
            wrong address; callee targets behind named constants are not checked);
- cycles?   (warning) the total of the `c.cycles += N` charges differs from the
            generated body (a port may merge charges, but a missing block or an
            extra one shows up here).

LITERAL and ADDR make the exit status 1. Run it on every writer's module before
the first build.
"""
import collections
import os
import re
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
GEN = os.path.join(ROOT, "generated/ee")
SRC = os.path.join(ROOT, "crates/ssx3-game/src")

LIT = re.compile(r"0x[0-9a-fA-F_]+")


def lit_val(s):
    return int(s[2:].replace("_", ""), 16)


def sizes():
    out = {}
    for l in open(os.path.join(ROOT, "generated/functions.tsv")):
        p = l.split("\t")
        try:
            out[int(p[0], 16)] = int(p[1], 16)
        except (ValueError, IndexError):
            pass
    return out


_SHARDS = {}


def generated_body(addr):
    if not _SHARDS:
        for shard in sorted(os.listdir(GEN)):
            path = os.path.join(GEN, shard, "src/lib.rs")
            if os.path.exists(path):
                _SHARDS[path] = open(path).read()
    for name in (f"f_{addr:08x}__generated", f"f_{addr:08x}"):
        for t in _SHARDS.values():
            i = t.find(f"pub fn {name}(c: &mut Cpu)")
            if i >= 0:
                m = re.search(r"\n(?=/// |pub fn |pub use |#\[)", t[i + 10:])
                return t[i : i + 10 + (m.start() if m else len(t))]
    return None


def split_fns(text):
    """Module-level functions of `text`: name -> body text."""
    fns = {}
    for m in re.finditer(r"^(?:pub(?:\(crate\))? )?fn (\w+)", text, re.M):
        depth, i, started = 0, m.start(), False
        j = text.index("{", m.end()) if "{" in text[m.end():] else len(text)
        i = j
        while i < len(text):
            ch = text[i]
            if ch == "{":
                depth += 1
                started = True
            elif ch == "}":
                depth -= 1
                if started and depth == 0:
                    break
            i += 1
        fns[m.group(1)] = text[m.start() : i + 1]
    return fns


def entries(text):
    i = text.find("pub const OVERRIDES")
    if i < 0:
        return []
    block = text[i : text.index("];", i)]
    return [
        (int(a.replace("_", ""), 16), fn.split("::")[-1], name)
        for a, fn, name in re.findall(r'\(\s*0x([0-9a-fA-F_]+)\s*,\s*([\w:]+)\s*,\s*"([^"]+)"\s*,?\s*\)', block)
    ]


def strip_comments(text):
    """Drop `//` comments (addresses in comments are not literals of the port)."""
    return re.sub(r"//[^\n]*", "", text)


def closure(fns, root):
    seen, todo = set(), [root]
    while todo:
        n = todo.pop()
        if n in seen or n not in fns:
            continue
        seen.add(n)
        for w in set(re.findall(r"\b\w+\b", fns[n])):
            if w in fns and w not in seen:
                todo.append(w)
    return strip_comments("\n".join(fns[n] for n in sorted(seen)))


def lint_file(path, size):
    text = open(path).read()
    cut = text.find("#[cfg(test)]\nmod tests")
    main = text if cut < 0 else text[:cut]
    fns = split_fns(main)
    bad = 0
    for addr, fn, name in entries(main):
        gen = generated_body(addr)
        if gen is None:
            print(f"{path}: {addr:#010x} {name}: no generated body found")
            continue
        port = closure(fns, fn)
        if not port:
            print(f"{path}: {addr:#010x} {name}: port function `{fn}` not found")
            continue
        pl = {lit_val(x) for x in LIT.findall(port)}
        low = {x & 0xFFFF for x in pl}
        want = {int(x, 16) for x in re.findall(r"c\.g\[RA\]\[0\] = sx\((0x[0-9a-f]+)\)", gen)}
        missing = sorted(x for x in want if x not in pl and (x & 0xFFFF) not in low)
        issues, warns = [], []
        if missing:
            warns.append("RA not in port: " + " ".join(f"{x:#x}" for x in missing[:6]) + (" ..." if len(missing) > 6 else ""))
        odd = sorted({x for x in LIT.findall(port) if len(x[2:].replace("_", "")) not in (*range(1, 9), 16)})
        if odd:
            issues.append("LITERAL with more than 8 digits: " + " ".join(odd[:5]))
        gl = {int(x, 16) for x in re.findall(r"0x[0-9a-f]+", gen)}
        lo, hi = addr, size.get(addr, addr + 0x1000)
        stray = sorted(x for x in pl if lo < x < hi and x not in gl)
        if stray:
            issues.append("ADDR in the function's range but not in the generated body: " + " ".join(f"{x:#x}" for x in stray[:6]))
        gc = collections.Counter(int(x) for x in re.findall(r"c\.cycles \+= (\d+);", gen))
        pc = collections.Counter(int(x) for x in re.findall(r"c\.cycles \+= (\d+);", port))
        gs, ps = sum(k * v for k, v in gc.items()), sum(k * v for k, v in pc.items())
        if ps and gs != ps:
            warns.append(f"cycles? generated {sum(gc.values())} charges/{gs}, port {sum(pc.values())}/{ps}")
        if issues:
            bad += 1
            print(f"FAIL {addr:#010x} {name}: " + "; ".join(issues + warns))
        elif warns:
            print(f"warn {addr:#010x} {name}: " + "; ".join(warns))
    return bad


def main(argv):
    if not argv:
        sys.exit(__doc__)
    size = sizes()
    bad = 0
    for a in argv:
        path = a if a.endswith(".rs") else os.path.join(SRC, f"{a}.rs")
        bad += lint_file(path, size)
    print(f"{bad} function(s) with findings")
    sys.exit(1 if bad else 0)


if __name__ == "__main__":
    main(sys.argv[1:])
