#!/usr/bin/env python3
"""Acceptance runs for the single-loop work (docs/plans/single-loop.md).

    python3 tools/accept.py record DIR [FRAMES]   # run the gate replay, keep the evidence
    python3 tools/accept.py compare A B           # compare two recordings

`record` runs ./target/game/ssx3-headless on the gate's race input with
SSX3_STATE_LOG (a hash of every 4 KiB page of EE RAM per 5 frames, guest thread
stacks left out) and screenshots every 250 frames, and copies the final RAM dump
next to them: DIR/state.bin, DIR/shots/frameNNNNN.png, DIR/ram.bin.

`compare` reports, for the state logs, the first frame whose pages differ and how
the number of differing pages grows (a changed schedule shows as a few pages that
differ for a while and then converge, a real divergence as pages that keep
growing), and, for the screenshots, the mean absolute difference per frame.
Exit status 0 when both runs finished, logged the same frames, and every screenshot is within
SHOT_TOL (mean absolute channel difference, default 2.0) and the last logged frame
differs in no more than PAGE_TOL pages (default 64 of 8192); identical RAM does not
skip the screenshot check. Missing screenshots, missing frames, a truncated log or
a missing Pillow fail the comparison.
Build first: cargo build --profile game -p ssx3 --bins.
"""
import os
import struct
import subprocess
import sys

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import integrate_wave  # noqa: E402

SHOT_TOL = float(os.environ.get("SHOT_TOL", 2.0))
PAGE_TOL = int(os.environ.get("PAGE_TOL", 64))


def record(d, frames):
    os.makedirs(os.path.join(d, "shots"), exist_ok=True)
    env = dict(
        os.environ,
        SSX3_STATE_LOG=os.path.join(d, "state.bin"),
        SSX3_STATE_EVERY="5",
        SSX3_SHOTS=os.path.join(d, "shots"),
        SSX3_RAM_DUMP=os.path.join(d, "ram.bin"),
        SSX3_INPUT=integrate_wave.RACE_INPUT,
    )
    subprocess.run(["./target/game/ssx3-headless", str(frames), "--shot-every", "250"],
                   cwd=ROOT, env=env, check=True, stdout=subprocess.DEVNULL,
                   stderr=subprocess.DEVNULL)
    print(f"recorded {d}")


def read_log(path):
    b = open(path, "rb").read()
    assert b[:8] == b"SSX3STAT", path
    page, n = struct.unpack_from("<II", b, 8)
    rec = 4 + 8 * n
    if (len(b) - 16) % rec:
        sys.exit(f"{path}: truncated state log")
    out = {}
    for o in range(16, len(b) - rec + 1, rec):
        f = struct.unpack_from("<I", b, o)[0]
        out[f] = struct.unpack_from(f"<{n}Q", b, o + 4)
    return page, n, out


def compare(a, b):
    ok = True
    for d in (a, b):
        if not os.path.exists(os.path.join(d, "ram.bin")):
            print(f"{d}: ram.bin missing (the run did not finish)")
            return 1
    ram_same = open(os.path.join(a, "ram.bin"), "rb").read() == open(os.path.join(b, "ram.bin"), "rb").read()
    print("RAM identical at exit" if ram_same else "RAM differs at exit")
    page, n, la = read_log(os.path.join(a, "state.bin"))
    _, _, lb = read_log(os.path.join(b, "state.bin"))
    if set(la) != set(lb):
        print(f"logged frames differ: {len(set(la) ^ set(lb))} frames in only one recording")
        ok = False
    frames = sorted(set(la) & set(lb))
    first = None
    last_diff = 0
    series = []
    for f in frames:
        diff = [i for i in range(n) if la[f][i] != lb[f][i]]
        series.append((f, len(diff)))
        if diff and first is None:
            first = (f, diff)
        last_diff = len(diff)
    if first is None:
        print("state logs identical (RAM differs only outside the logged frames)")
    else:
        f, diff = first
        print(f"first difference: frame {f}, {len(diff)} pages, e.g. "
              + ", ".join(f"{i * page:#x}" for i in diff[:12]))
        step = max(1, len(series) // 24)
        print("differing pages by frame: "
              + " ".join(f"{f}:{k}" for f, k in series[::step]))
    print(f"differing pages at the last logged frame: {last_diff} (tolerance {PAGE_TOL})")
    if last_diff > PAGE_TOL:
        ok = False
    try:
        from PIL import Image, ImageChops, ImageStat
    except ImportError:
        print("PIL missing: screenshots cannot be compared (pip install pillow)")
        return 1
    sa, sb = os.path.join(a, "shots"), os.path.join(b, "shots")
    for name in sorted(os.listdir(sb)):
        if not os.path.exists(os.path.join(sa, name)):
            print(f"{name}: missing in {a}")
            ok = False
    worst = 0.0
    for name in sorted(os.listdir(sa)):
        if not os.path.exists(os.path.join(sb, name)):
            print(f"{name}: missing in {b}")
            ok = False
            continue
        ia, ib = Image.open(os.path.join(sa, name)).convert("RGB"), Image.open(os.path.join(sb, name)).convert("RGB")
        if ia.size != ib.size:
            print(f"{name}: size differs {ia.size} vs {ib.size}")
            ok = False
            continue
        m = sum(ImageStat.Stat(ImageChops.difference(ia, ib)).mean) / 3
        worst = max(worst, m)
        if m > SHOT_TOL:
            print(f"{name}: mean abs diff {m:.2f} > {SHOT_TOL}")
            ok = False
    print(f"worst screenshot difference {worst:.2f} (tolerance {SHOT_TOL})")
    if ram_same and ok:
        print("ACCEPT: RAM identical, screenshots match")
        return 0
    print("ACCEPT: equivalent" if ok else "ACCEPT FAILED")
    return 0 if ok else 1


if __name__ == "__main__":
    if len(sys.argv) >= 3 and sys.argv[1] == "record":
        record(sys.argv[2], int(sys.argv[3]) if len(sys.argv) > 3 else 6000)
    elif len(sys.argv) == 4 and sys.argv[1] == "compare":
        sys.exit(compare(sys.argv[2], sys.argv[3]))
    else:
        sys.exit(__doc__)
