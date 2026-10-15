#!/usr/bin/env python3
"""Summarise a dynamic EE call graph recorded with the `callprof` feature.

    CARGO_TARGET_DIR=target-prof cargo build --profile game -p ssx3 \
        --bin ssx3-headless --features callprof
    SSX3_CALLPROF=edges.tsv:5000:5300 SSX3_INPUT=... target-prof/game/ssx3-headless 5300
    tools/callgraph.py edges.tsv [--root NAME] [--depth N] [--min CALLS_PER_FRAME]

Prints:
  1. functions by calls per frame (with the number of distinct callers);
  2. the call tree below each root (default: the frame-loop and thread
     entry points), with calls per frame on each edge;
  3. calls per frame between subsystems (name prefix groups), which is where
     phase 3 boundaries sit.

Names come from generated/functions.tsv. Caller 0 means the return address
was outside any function (thread entry, exception return).
"""

import argparse
import collections
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

# Subsystem of a function, by name. First match wins; order matters.
GROUPS = [
    ("files", r"^(EAFile|AsyncFile|StreamFile|BigFile|sceCd|EAFile_Cd|sceOpen|sceRead|sceLseek|sceClose)"),
    ("sound", r"^(SND|SNDSTRM|SNDMIDI|SNDENC|PATH_|SPCH|Speech|SSXAudio|Audio_|BankMonitor|SndBank|SndLoad|MusicPlayer|SongInstance|StreamArray|StreamBuffer|CrowdAudio|MIDIBanks|PS2SoundManager)"),
    ("frame", r"^(App_|SSXApp|PS2ExecMan|EATimer|PadQueue|EAThread|InputPad)"),
    ("render", r"^(PS2GraphicsMan|Renderer_|DeferMan|TexMan|SprStream|Dma_|Gs_|PS2BezierMan|cPS2|LensFX|Vu0_|Vu1_|VifBuff|GifBuff)"),
    ("kernel", r"^(sce|_?Sif|Sif|CreateThread|StartThread|WaitSema|SignalSema|iSignalSema|PollSema|FlushCache|SetAlarm|iSetAlarm|Exit|_?kernel|DI$|EI$)"),
    ("libc", r"^(mem|str|__|_|printf|sprintf|vsprintf|malloc|free|qsort|sqrt|sin|cos|atan|floor|ceil|abs)"),
    ("math", r"^(Math_|Vec|Mat|Quat|XForm)"),
    ("ui", r"^(UI|FE|MenuSys|HUD|Font_|OV|cFE|cOV)"),
    ("world", r"^(World|Nash|cWorld|cStream|Path|PathData|Coll|Grid|Fence|cFlag|SkyBox|Weather|Snow)"),
    ("riders", r"^(Rider|AI_|ComputerActor|Board|cBoard|Trick|Anim|GameAnim)"),
    ("game", r"^(GameModule|GameMode|BE|Focus|Danger|Script|Replay|Playback|Camera|Cam)"),
]


def group(name):
    for g, pat in GROUPS:
        if re.match(pat, name):
            return g
    return "other"


def load_names():
    names = {}
    with open(os.path.join(ROOT, "generated", "functions.tsv")) as f:
        for line in f:
            if line.startswith("#"):
                continue
            a, _, n = line.rstrip("\n").split("\t")[:3]
            names[int(a, 16)] = n
    # Entries the recompiler found that Ghidra has no function for (pass 5).
    extra = os.path.join(ROOT, "generated", "discovered-names.tsv")
    if os.path.exists(extra):
        with open(extra) as f:
            for line in f:
                if not line.startswith("#"):
                    a, n = line.split("\t")[:2]
                    names.setdefault(int(a, 16), n)
    return names


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("edges")
    ap.add_argument("--root", action="append", help="function name or hex address (repeatable)")
    ap.add_argument("--depth", type=int, default=6)
    ap.add_argument("--min", type=float, default=0.5, help="hide edges below this many calls per frame")
    ap.add_argument("--top", type=int, default=80)
    a = ap.parse_args()

    names = load_names()
    frames = 1
    edges = collections.Counter()
    with open(a.edges) as f:
        for line in f:
            m = re.match(r"# frames (\d+)\.\.=(\d+)", line)
            if m:
                frames = int(m.group(2)) - int(m.group(1)) + 1
                continue
            if line.startswith("#"):
                continue
            x, y, n = line.split()
            edges[(int(x, 16), int(y, 16))] += int(n)

    def nm(addr):
        return names.get(addr, "<none>" if addr == 0 else f"f_{addr:08x}")

    calls = collections.Counter()
    callers = collections.defaultdict(set)
    children = collections.defaultdict(list)
    for (x, y), n in edges.items():
        calls[y] += n
        callers[y].add(x)
        children[x].append((y, n))

    total = sum(calls.values())
    print(f"# {frames} frames, {len(calls)} functions called, {total / frames:.0f} calls/frame\n")

    print(f"## Top {a.top} functions by calls per frame\n")
    print("| calls/frame | callers | function | group |\n|---:|---:|---|---|")
    for y, n in calls.most_common(a.top):
        print(f"| {n / frames:.1f} | {len(callers[y])} | `{nm(y)}` {y:#x} | {group(nm(y))} |")

    by_name = {v: k for k, v in names.items()}
    roots = []
    for r in a.root or []:
        roots.append(int(r, 16) if r.startswith("0x") else by_name[r])
    if not roots:
        roots = [y for (x, y) in edges if x == 0]
        roots = sorted(set(roots), key=lambda y: -calls[y])

    print("\n## Call trees (calls per frame)\n")
    for r in roots:
        print(f"### `{nm(r)}` {r:#x} ({calls[r] / frames:.2f}/frame)\n\n```")
        seen = set()

        def walk(f, d):
            for y, n in sorted(children.get(f, []), key=lambda t: -t[1]):
                if n / frames < a.min:
                    continue
                again = y in seen
                print(f"{'  ' * d}{n / frames:8.1f}  {nm(y)}{' ...' if again and children.get(y) else ''}")
                if not again and d < a.depth:
                    seen.add(y)
                    walk(y, d + 1)

        seen.add(r)
        walk(r, 0)
        print("```\n")

    print("## Calls per frame between subsystems (caller group -> callee group)\n")
    cross = collections.Counter()
    entries = collections.defaultdict(collections.Counter)
    for (x, y), n in edges.items():
        gx, gy = group(nm(x)), group(nm(y))
        if gx != gy:
            cross[(gx, gy)] += n
            entries[gy][y] += n
    print("| from | to | calls/frame |\n|---|---|---:|")
    for (gx, gy), n in cross.most_common():
        if n / frames >= a.min:
            print(f"| {gx} | {gy} | {n / frames:.1f} |")
    print("\n## Entry points into each subsystem (called from outside it)\n")
    for g in ["files", "sound", "frame", "render", "kernel"]:
        print(f"### {g}\n")
        for y, n in entries[g].most_common(40):
            print(f"- `{nm(y)}` {y:#x}: {n / frames:.1f}/frame")
        print()


if __name__ == "__main__":
    sys.exit(main())
