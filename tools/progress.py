#!/usr/bin/env python3
"""Update the porting progress shown in the README.

    python3 tools/progress.py

Counts the functions in `crates/ssx3-game/overrides.txt` (ported to Rust)
against `generated/functions.tsv` (every function of the game), overall and
split by purpose, and writes the progress bars `docs/images/progress.svg`
(shown in the README). Run it after installing ported functions.

A function's purpose comes from `generated/purpose.tsv` (address -> group,
frozen by `tools/purpose_map.py` from the C++ reference); functions missing
from the map classify from their ported `crates/ssx3-game/src/game/<dir>` or,
failing that, by name prefix. Same tree = same SVG, every run: no reference
checkout needed.
"""
import glob
import json
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(ROOT, "tools"))

GAME = "Game logic"
PRESENTATION = "Front end, graphics, sound"
ENGINE = "Engine, libraries, online"
GROUPS = [GAME, PRESENTATION, ENGINE]

# Source directory (reference C++ or ported Rust) -> group.
DIRS = {
    **dict.fromkeys(
        "ai animation be camera input intersect luno main object path replay "
        "scripter wscript world".split(),
        GAME,
    ),
    **dict.fromkeys(
        "fe ui render visualfx worldpainter movie sound".split(), PRESENTATION
    ),
    **dict.fromkeys(
        "bx mem util ealib seg dirtysock lib hashvalue md5 misc".split(), ENGINE
    ),
}

# Name prefixes for functions with no source directory (EA and PS2 libraries,
# the C runtime, unnamed code fall through to the engine group).
PREFIXES = [
    (GAME, "Rider ComputerActor AI_ WScript InSeq WorldObj BE ScriptEngine Race Trick "
     "Game World_ Camera Chase Replay Path_ PathData"),
    (PRESENTATION, "SND SPCH PATH_ libmpeg SSXAudio MusicPlayer Song Speech SHP tWPIGD "
     "OVScreen OVState cSplash HUD FE UI Crowd Vfx Lens PS2Graphics TexMan Render"),
]


def classify(name, ref_dir, rust_dir):
    for d in (ref_dir, rust_dir):
        if d in DIRS:
            return DIRS[d]
    for group, prefixes in PREFIXES:
        if any(name.startswith(p) for p in prefixes.split()):
            return group
    return ENGINE


SHORT = {"game": GAME, "presentation": PRESENTATION, "engine": ENGINE}


def ref_dirs():
    """Address -> top source directory of the reference, if available.

    The chart path (`counts`) no longer uses this; it is kept for
    `pick_wave.py`'s `SSX3_FOCUS` group filter.
    """
    try:
        from refsrc import REF, load_symbols

        index = json.load(open(os.path.join(ROOT, "tools/.refindex.json")))
        symbols = load_symbols()
    except (OSError, ImportError, ValueError):
        return {}
    out = {}
    for addr, name in symbols.items():
        entry = index.get(name)
        if entry:
            rel = os.path.relpath(entry[1], os.path.join(REF, "src"))
            out[addr] = rel.split("/")[0].removesuffix(".cpp").removesuffix(".c")
    return out


def purpose_groups():
    """Address -> purpose group from `generated/purpose.tsv` (frozen by
    `tools/purpose_map.py`); {} entries fall back to the ported source
    directory or the name prefix in `classify`."""
    out = {}
    for line in open(os.path.join(ROOT, "generated/purpose.tsv")):
        if line.startswith("#") or not line.strip():
            continue
        addr, group = line.split()
        if group in SHORT:
            try:
                out[int(addr, 16)] = SHORT[group]
            except ValueError:
                pass
    return out


def rust_dirs():
    """Address -> `src/game/<dir>` of each ported function."""
    out = {}
    entry = re.compile(r'\(\s*0x([0-9a-f]{4}_[0-9a-f]{4}),\s*\w+,\s*"')
    base = os.path.join(ROOT, "crates/ssx3-game/src/game")
    for path in glob.glob(os.path.join(base, "**/*.rs"), recursive=True):
        d = os.path.relpath(path, base).split("/")[0].removesuffix(".rs")
        for m in entry.finditer(open(path).read()):
            out[int(m.group(1).replace("_", ""), 16)] = d
    return out


def registered():
    """Addresses with a hand-written Rust replacement: the entries of every
    `OVERRIDES` table under crates/ssx3-game/src. Lines in overrides.txt that
    only hold a slot for the generated body do not count."""
    import glob

    done = set()
    pat = re.compile(r"pub(?:\(crate\))? (?:const|static) OVERRIDES[^=]*=\s*&?\[")
    for path in glob.glob(os.path.join(ROOT, "crates/ssx3-game/src/**/*.rs"), recursive=True):
        text = open(path).read()
        for m in pat.finditer(text):
            end = text.index("];", m.end())
            for a in re.findall(r"\(\s*0x([0-9a-fA-F_]+)\s*,", text[m.end():end]):
                done.add(int(a.replace("_", ""), 16))
    return done


def universe():
    """Every function of the EE code: (start, end, name) from
    `generated/ee-symbols.tsv` (committed, so this works without the generated
    code), named from `generated/functions.tsv` when the Ghidra list has it.
    The symbol table also holds the small functions (stubs, thunks) that the
    Ghidra list leaves out."""
    names = {}
    path = os.path.join(ROOT, "generated/functions.tsv")
    if os.path.exists(path):
        for line in open(path):
            cols = line.rstrip("\n").split("\t")
            try:
                names[int(cols[0], 16)] = cols[2] if len(cols) > 2 else ""
            except ValueError:
                pass
    rows = []
    for line in open(os.path.join(ROOT, "generated/ee-symbols.tsv")):
        cols = line.rstrip("\n").split("\t")
        if line.startswith("#") or len(cols) < 3:
            continue
        try:
            rows.append((int(cols[1], 16), int(cols[2], 16)))
        except ValueError:
            continue
    rows.sort()
    return [(s, e, names.get(s, "")) for s, e in rows]


def groups_of(rows, mapped, rust):
    """Group of each function: its purpose group, ported directory or name
    prefix; an unnamed small function takes the group of the nearest named
    function before it (a stub sits next to the code it belongs to)."""
    out, last = {}, ENGINE
    for start, _end, name in rows:
        g = mapped.get(start)
        if g is None and (name or rust.get(start)):
            g = classify(name, None, rust.get(start))
        if g is None:
            g = last
        else:
            last = g
        out[start] = g
    return out


def counts():
    ported = registered()
    mapped, rust = purpose_groups(), rust_dirs()
    rows = universe()
    group = groups_of(rows, mapped, rust)
    # [functions done, functions, bytes done, bytes] overall and per group
    stats = {g: [0, 0, 0, 0] for g in ["All"] + GROUPS}
    for start, end, _name in rows:
        done = start in ported
        for g in ("All", group[start]):
            s = stats[g]
            s[1] += 1
            s[3] += end - start
            if done:
                s[0] += 1
                s[2] += end - start
    return stats


ROW = 24  # px per bar


def bar(y, label, s, strong):
    """One horizontal bar (share of functions), filled from the left; the
    percentage and the counts follow it (functions, with the
    megabytes under them)."""
    x, w = 160, 490  # the counts are right-aligned at 800
    done, fns, done_size, size = s
    fill = round(w * done / fns)
    cls = "label strong" if strong else "label"
    return f"""
  <text x="0" y="{y + 11}" class="{cls}">{label}</text>
  <rect x="{x}" y="{y}" width="{w}" height="14" rx="2" class="track"/>
  <rect x="{x}" y="{y}" width="{fill}" height="14" rx="2" class="fill"/>
  <text x="{x + w + 8}" y="{y + 11}" class="pct">{100 * done / fns:.0f}%</text>
  <text x="800" y="{y + 7}" text-anchor="end" class="sub">{done:,} / {fns:,} functions</text>
  <text x="800" y="{y + 16}" text-anchor="end" class="mb">{done_size / 1e6:.2f} / {size / 1e6:.2f} MB</text>"""


def svg(stats):
    rows = ["All"] + GROUPS
    labels = {"All": "All", **{g: g for g in GROUPS}}
    height = 3 + ROW * len(rows) - (ROW - 18)
    a = stats["All"]
    body = "".join(bar(3 + ROW * i, labels[r], stats[r], r == "All") for i, r in enumerate(rows))
    return f"""<svg xmlns="http://www.w3.org/2000/svg" width="800" height="{height}" viewBox="0 0 800 {height}" role="img"
     aria-label="Ported to Rust: {a[0]} of {a[1]} functions">
  <style>
    .track {{ fill: #8b949e; fill-opacity: 0.3; }}
    .fill {{ fill: #1e90e0; }}
    text {{ font-family: -apple-system, 'Segoe UI', Helvetica, Arial, sans-serif; fill: #8b949e; }}
    .pct {{ font-size: 12px; font-weight: 600; }}
    .label {{ font-size: 12px; }}
    .strong {{ font-weight: 600; }}
    .sub {{ font-size: 10px; text-anchor: end; }}
    .mb {{ font-size: 8px; fill-opacity: 0.8; text-anchor: end; }}
  </style>{body}
</svg>
"""


def quality():
    """Port quality of the hand-written replacements (docs/plans/roadmap.md,
    "Port quality"): the functions registered in an `OVERRIDES` table of
    `ssx3-game`, split by their control flow.

    - goto-style: a labelled loop (`'l_ADDR: loop`), labelled blocks
      (`'b_ADDR: {`) or a `match pc` state machine, transcribed from the
      generated body (L1);
    - structured: plain `if`/`loop`/`while` over the register file (L2).

    Typed signatures and idiomatic code (L3, L4) are not counted yet."""
    import re

    goto = structured = 0
    root = os.path.join(ROOT, "crates/ssx3-game/src")
    for d, _dirs, files in os.walk(root):
        for f in files:
            if not f.endswith(".rs"):
                continue
            text = open(os.path.join(d, f)).read()
            cut = text.find("#[cfg(test)]\nmod tests")
            text = text if cut < 0 else text[:cut]
            i = text.find("pub const OVERRIDES")
            if i < 0:
                continue
            block = text[i : text.index("];", i)]
            names = {fn.split("::")[-1] for _a, fn in re.findall(
                r'\(\s*(0x[0-9a-fA-F_]+)\s*,\s*([\w:]+)\s*,\s*"[^"]+"\s*,?\s*\)', block)}
            parts = re.split(r"\n(?=(?:pub(?:\(crate\))? )?fn )", text)
            for part in parts[1:]:
                m = re.match(r"(?:pub(?:\(crate\))? )?fn (\w+)", part)
                if not m or m.group(1) not in names:
                    continue
                if re.search(r"'[lb]_[0-9a-f]+: (?:loop|\{)|let mut pc: u32 = 0x", part):
                    goto += 1
                else:
                    structured += 1
    return goto, structured


def main():
    stats = counts()
    with open(os.path.join(ROOT, "docs/images/progress.svg"), "w") as f:
        f.write(svg(stats))
    for g, (d, n, ds, s) in stats.items():
        print(f"{g:28} {d:6,}/{n:6,} functions ({100 * d / n:4.1f}%)  {100 * ds / s:4.1f}% of bytes")
    goto, structured = quality()
    print(f"{'Port quality (hand-written)':28} goto-style {goto:,}, structured {structured:,} "
          f"({100 * structured / max(goto + structured, 1):.1f}% structured)")


if __name__ == "__main__":
    main()
