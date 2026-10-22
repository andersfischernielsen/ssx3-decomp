#!/usr/bin/env python3
"""Race replay at 60 Hz and at N Hz, screenshots side by side at matched real time.

    python3 tools/tick_ab.py [HZ] [OUT]      # default 120, scratch dir under /tmp

Runs ./target/game/ssx3-headless twice (one heavy process at a time): the
gate's race input at 60 Hz, then with SSX3_TICK_HZ=HZ and the input's frame
numbers and durations scaled by HZ/60. Writes OUT/sheet*.png: each row is
60 Hz frame F (left) next to N Hz frame F*HZ/60 (right). Compare the HUD's
race clock and course % (they must agree up to a constant start offset),
the speed, and anything that visibly runs fast or slow (camera, animation,
effects, AI riders). Build first: cargo build --profile game -p ssx3 --bins.
"""
import os
import subprocess
import sys

from PIL import Image

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import integrate_wave  # noqa: E402

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
HZ = int(sys.argv[1]) if len(sys.argv) > 1 else 120
OUT = sys.argv[2] if len(sys.argv) > 2 else f"/tmp/ssx3-tick-ab-{HZ}"
FRAMES, EVERY = 7000, 250


def run(hz):
    d = os.path.join(OUT, str(hz))
    os.makedirs(d, exist_ok=True)
    k = hz / 60
    inp = ",".join(
        f"{round(int(a) * k)}:{b}:{round(int(n) * k)}"
        for a, b, n in (e.split(":")[:3] for e in integrate_wave.RACE_INPUT.split(","))
    )
    env = dict(os.environ, SSX3_SHOTS=d, SSX3_INPUT=inp)
    env.pop("SSX3_TICK_HZ", None)
    if hz != 60:
        env["SSX3_TICK_HZ"] = str(hz)
    with open(os.path.join(OUT, f"{hz}.log"), "w") as log:
        subprocess.run(
            ["./target/game/ssx3-headless", str(round(FRAMES * k)), "--shot-every", str(round(EVERY * k))],
            cwd=ROOT, env=env, stdout=log, stderr=subprocess.STDOUT, check=False,
        )
    subprocess.run(["git", "checkout", "-q", "generated/entries.txt"], cwd=ROOT, check=False)
    return d


a, b = run(60), run(HZ)
rows = []
for f in range(4500, FRAMES + 1, EVERY):
    pa, pb = os.path.join(a, f"frame{f:05d}.png"), os.path.join(b, f"frame{round(f * HZ / 60):05d}.png")
    if not (os.path.exists(pa) and os.path.exists(pb)):
        continue
    A, B = Image.open(pa).convert("RGB"), Image.open(pb).convert("RGB").resize(Image.open(pa).size)
    r = Image.new("RGB", (A.width * 2 + 8, A.height), "white")
    r.paste(A, (0, 0))
    r.paste(B, (A.width + 8, 0))
    rows.append(r)
for i in range(0, len(rows), 4):
    g = rows[i:i + 4]
    s = Image.new("RGB", (g[0].width, sum(x.height + 8 for x in g)), "white")
    y = 0
    for x in g:
        s.paste(x, (0, y))
        y += x.height + 8
    s.save(os.path.join(OUT, f"sheet{i // 4}.png"))
print(f"{OUT}/sheet*.png: {len(rows)} rows (left 60 Hz, right {HZ} Hz)")
