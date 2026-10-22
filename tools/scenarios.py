#!/usr/bin/env python3
"""Scripted headless scenarios for reviewing hand-written ports
(docs/plans/port-architecture.md, task 1).

    tools/scenarios.py list                 # scenario names
    tools/scenarios.py input NAME           # its SSX3_INPUT string
    tools/scenarios.py frames NAME          # how many frames it runs

Inputs are `frame:buttons:frames` presses (libpad bits: Select 0x1,
Start 0x8, Up 0x10, Right 0x20, Down 0x40, Left 0x80, L2 0x100, R2 0x200,
L1 0x400, R1 0x800, Triangle 0x1000, Circle 0x2000, Cross 0x4000,
Square 0x8000), from a fresh memory card with the movies skipped (the
headless default).

Menu timings were found with screenshots: the title at ~1600, then the main
menu, Select Character, Setup Character, Select Peak (Peak 1 selected; Up
for 2, 3), Select Mode (Race; Down for Freestyle), Select Event, My Rules,
loading, the event card (X), play. Menus take an input every ~50 frames.
"""
import random
import sys

START, X, CIRCLE, SQUARE, TRI = 0x8, 0x4000, 0x2000, 0x8000, 0x1000
UP, RIGHT, DOWN, LEFT = 0x10, 0x20, 0x40, 0x80
L1, R1, L2, R2 = 0x400, 0x800, 0x100, 0x200

# (peak, mode, events): mode 0 race, 1 freestyle.
EVENTS = {
    (1, 0): ["snow_jam", "metro_city", "happiness"],
    (1, 1): ["r_and_b", "crows_nest", "the_junction", "happiness_jam"],
    (2, 0): ["ruthless_ridge", "intimidator", "ruthless"],
    (2, 1): ["style_mile", "launch_time", "schizophrenia", "ruthless_jam"],
    (3, 0): ["gravitude", "the_throne"],
    (3, 1): ["kick_doubt", "much_2_much", "perpendiculous", "throne_jam"],
}

PLAY = 3000


def press(ev, f, b, n=8):
    ev.append(f"{f}:{b:04x}:{n}")


def gameplay(ev, start, frames, seed):
    """Riding: steering, boost candidates, charged jumps with grabs and
    spins, trick buttons; a fixed 240-frame cycle with a seeded twist."""
    r = random.Random(seed)
    f = start
    while f < start + frames:
        steer = r.choice([LEFT, RIGHT, 0])
        press(ev, f, CIRCLE | steer, 60)          # boost (Circle) while steering
        press(ev, f + 60, SQUARE, 50)             # Square (boost / other)
        press(ev, f + 115, X, 35)                 # crouch; release = jump
        press(ev, f + 152, R1 | r.choice([LEFT, RIGHT]), 30)  # grab + spin
        press(ev, f + 184, r.choice([L1, L2, R2, L1 | R1, L2 | R2]), 25)  # more grabs
        press(ev, f + 212, r.choice([TRI, CIRCLE | X, 0]) or SQUARE, 12)
        f += 240


def single_event(peak, mode, index):
    ev = []
    press(ev, 1600, START)
    press(ev, 2000, X)          # Single Event
    press(ev, 2500, X)          # character
    press(ev, 2700, X)          # Continue
    f = 2850
    for _ in range(peak - 1):
        press(ev, f, UP, 6)
        f += 50
    press(ev, f, X)             # peak
    f += 150
    if mode:
        press(ev, f, DOWN, 6)
        f += 50
    press(ev, f, X)             # mode
    f += 150
    for _ in range(index):
        press(ev, f, DOWN, 6)
        f += 50
    press(ev, f, X)             # event
    f += 200
    press(ev, f, X)             # My Rules: Continue
    # Loading, then the event card: X until the event has surely started.
    for k in range(1, 15):
        press(ev, f + 100 * k, X)
    gameplay(ev, f + 1600, PLAY, seed=peak * 100 + mode * 10 + index)
    return ",".join(ev), f + 1600 + PLAY


def monkey():
    r = random.Random(7)
    btn = [0x10, 0x20, 0x40, 0x80, X, X, X, TRI, CIRCLE, START]
    ev = ["1600:0008:8"] + [f"{f}:{r.choice(btn):04x}:6" for f in range(1700, 20000, 30)]
    return ",".join(ev), 20000


def scenarios():
    out = {}
    for (peak, mode), names in EVENTS.items():
        for i, n in enumerate(names):
            out[f"p{peak}_{'race' if mode == 0 else 'free'}_{n}"] = single_event(peak, mode, i)
    out["monkey"] = monkey()
    return out


if __name__ == "__main__":
    s = scenarios()
    cmd = sys.argv[1] if len(sys.argv) > 1 else "list"
    if cmd == "list":
        print("\n".join(s))
    elif cmd == "input":
        print(s[sys.argv[2]][0])
    elif cmd == "frames":
        print(s[sys.argv[2]][1])
