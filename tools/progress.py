#!/usr/bin/env python3
"""Update the porting progress shown in the README.

    python3 tools/progress.py

Counts the functions in `crates/ssx3-game/overrides.txt` (ported to Rust)
against `generated/functions.tsv` (every function of the game), overall and
split by purpose, and writes the progress bars `docs/images/progress.svg`
(shown in the README). Run it after installing ported functions.

A function's purpose comes from its source directory: the C++ reference's
(`tools/.refindex.json`, `refindex.py`) or, for a ported function, its
`crates/ssx3-game/src/game/<dir>`; functions in neither go by name prefix.
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


def ref_dirs():
    """Address -> top source directory of the reference, if available."""
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


def counts():
    ported = set()
    for line in open(os.path.join(ROOT, "crates/ssx3-game/overrides.txt")):
        word = line.split("#")[0].split()
        if word:
            try:
                ported.add(int(word[0], 16))
            except ValueError:
                pass
    ref, rust = ref_dirs(), rust_dirs()
    # [functions done, functions, bytes done, bytes] overall and per group
    stats = {g: [0, 0, 0, 0] for g in ["All"] + GROUPS}
    for line in open(os.path.join(ROOT, "generated/functions.tsv")):
        cols = line.rstrip("\n").split("\t")
        start, end = int(cols[0], 16), int(cols[1], 16)
        name = cols[2] if len(cols) > 2 else ""
        done = start in ported
        for g in ("All", classify(name, ref.get(start), rust.get(start))):
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


def main():
    stats = counts()
    with open(os.path.join(ROOT, "docs/images/progress.svg"), "w") as f:
        f.write(svg(stats))
    for g, (d, n, ds, s) in stats.items():
        print(f"{g:28} {d:6,}/{n:6,} functions ({100 * d / n:4.1f}%)  {100 * ds / s:4.1f}% of bytes")


if __name__ == "__main__":
    main()
