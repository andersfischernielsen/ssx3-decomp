#!/usr/bin/env python3
"""Drive the Select Peak screen of ssx3-custom-headless with scripted pad input
and start a Tricky event from it (needs `ssx3 plugins build`, the discs and the
prepared courses; the plugin at plugins/tricky of an ssx3-decomp checkout).

    tools/menu_run.py [--frames N] [--shots DIR] [--events K] [--tab freestyle] [--every N] PEAK_ROW_DOWNS

Defaults: first event of the Race list of Peak Tricky. `--index i` picks the
i-th event of the list (0 based), `--freestyle` the Freestyle list.
Frames at ~2850: the Select Peak screen. Input timings follow
ssx3-decomp/tools/plugin_menu_test.py.
"""
import argparse
import os
import subprocess

START, X, TRI = 0x8, 0x4000, 0x1000
UP, DOWN = 0x10, 0x40


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", default=os.path.abspath(os.path.join(os.path.dirname(__file__), "../../..")))
    ap.add_argument("--frames", type=int, default=7000)
    ap.add_argument("--shots", default="/tmp/tricky-menu-shots")
    ap.add_argument("--shot-from", type=int, default=2800)
    ap.add_argument("--every", type=int, default=150)
    ap.add_argument("--index", type=int, default=0)
    ap.add_argument("--freestyle", action="store_true")
    ap.add_argument("--stop-at-list", action="store_true", help="stop in the event list (no start)")
    ap.add_argument("--extra", default="", help="more SSX3_INPUT entries frame:buttons:n")
    a = ap.parse_args()
    ev = []

    def press(frame, buttons, n=8):
        ev.append(f"{frame}:{buttons:04x}:{n}")

    press(1600, START)
    press(2000, X)
    press(2500, X)
    press(2700, X)
    press(2900, DOWN, 6)            # Peak Tricky
    press(3050, X)                  # the peak: Race / Freestyle goals
    if a.freestyle:
        press(3150, DOWN, 6)
    press(3250, X)                  # the goal: event list
    for i in range(a.index):
        press(3400 + 40 * i, DOWN, 6)
    t = 3400 + 40 * a.index + 100
    if not a.stop_at_list:
        for f in (t, t + 200, t + 400, t + 600):
            press(f, X)
    if a.extra:
        ev.append(a.extra)
    env = dict(os.environ, SSX3_INPUT=",".join(ev), SSX3_SHOTS=a.shots, SSX3_SHOT_FROM=str(a.shot_from),
               SSX3_FILE_LOG=os.path.join(a.shots, "files.log"))
    os.makedirs(a.shots, exist_ok=True)
    exe = os.path.join(a.root, "target/game/ssx3-custom-headless")
    p = subprocess.run([exe, str(a.frames), "--shot-every", str(a.every)], cwd=a.root, env=env,
                       capture_output=True, text=True)
    open(os.path.join(a.shots, "out.log"), "w").write(p.stdout + p.stderr)
    print(p.stdout[-2000:], p.stderr[-3000:])


if __name__ == "__main__":
    main()
