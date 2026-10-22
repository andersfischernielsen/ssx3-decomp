#!/usr/bin/env python3
"""Scripted headless scenarios for reviewing hand-written ports
(docs/plans/port-architecture.md, task 1).

    tools/scenarios.py list                 # scenario names
    tools/scenarios.py input NAME           # its SSX3_INPUT string
    tools/scenarios.py frames NAME          # how many frames it runs

Inputs are `frame:buttons:frames` presses (libpad bits: Select 0x1,
Start 0x8, Up 0x10, Right 0x20, Down 0x40, Left 0x80, L2 0x100, R2 0x200,
L1 0x400, R1 0x800, Triangle 0x1000, Circle 0x2000, Cross 0x4000,
Square 0x8000), with the movies skipped (the headless default). Runs start
from a copy of a memory card (`card NAME`): `saves/ssx3-mc0.ps2` (a
completed career: everything bought, Conquer The Mountain resumes at the
Peak 2 station) or `fresh` (a new, unformatted card).

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


# Conquer The Mountain (career) on the progressed card: main menu Down, X,
# X (Mac, resume); the rider starts at the Peak 2 station. Right for 30
# frames leads into the Transport gate (its Select Peak menu at ~3450,
# Peak 2 selected); left then right after that into the Lodge gate.
CTM = "1600:0008:8,1800:0040:6,1900:4000:8,2300:4000:8"
CARD = "saves/ssx3-mc0.ps2"

# Transport goals: (peak, goal, events); goal 0 race, 1 freestyle,
# 2 freeride (Big Challenges).
TRANSPORT = {
    (3, 2): ["the_throne", "black_station", "gravitude", "much_2_much", "perpendiculous", "kick_doubt"],
    (2, 1): ["style_mile"],
    (2, 0): ["ruthless_ridge"],
    (1, 2): ["happiness"],
}


def ctm_transport(peak, goal, index):
    """Ride into the Transport gate, teleport to an event's area, ride it."""
    ev = [CTM, "2420:0020:30"]
    f = 3450
    if peak != 2:
        press(ev, f, UP if peak == 3 else DOWN, 6)
    f += 50
    press(ev, f, X)              # peak
    f += 150
    for _ in range(goal):
        press(ev, f, DOWN, 6)
        f += 50
    press(ev, f, X)              # goal
    f += 150
    for _ in range(index):
        press(ev, f, DOWN, 6)
        f += 50
    press(ev, f, X)              # event
    f += 150
    press(ev, f, X)              # "Transport to this area now?" Yes
    # The ride there, the event card (freestyle / race): X until it starts.
    for k in range(1, 25):
        press(ev, f + 100 * k, X)
    gameplay(ev, f + 2600, PLAY, seed=1000 + peak * 100 + goal * 10 + index)
    return ",".join(ev), f + 2600 + PLAY


def ctm_lodge():
    """Into the Lodge; Equip Gear: another hat (Peacekeeper); Rider Details,
    Ubertrick Setup: bind Bar Hop for Mute; Save Game (overwrite); Return to
    Game, save progress (overwrite again); ride."""
    ev = [CTM, "2420:0020:30,2960:0080:20,3080:0020:20,3600:4000:8"]
    seq = [  # (frame, button, frames)
        (3800, DOWN, 6), (3850, X, 8),                      # Equip Gear
        (3950, X, 8), (4050, DOWN, 6), (4100, X, 8),        # Head, Hats
        (4200, DOWN, 6), (4250, DOWN, 6), (4300, X, 8),     # Peacekeeper
        (4400, TRI, 8), (4500, TRI, 8), (4600, TRI, 8),     # back to the lodge
        (4800, DOWN, 6), (4850, DOWN, 6), (4900, DOWN, 6), (4950, X, 8),   # Rider Details
        (5050, DOWN, 6), (5100, DOWN, 6), (5150, DOWN, 6), (5200, X, 8),   # Ubertrick Setup
        (5350, X, 8), (5450, X, 8), (5550, TRI, 8),        # Mute: Bar Hop
        *[(f, DOWN, 6) for f in range(5650, 5950, 50)], (5950, X, 8),      # DONE
        (6100, TRI, 8), (6200, DOWN, 6), (6250, DOWN, 6), (6300, X, 8),    # Save Game
        (6450, X, 8), (6600, X, 8), (6750, UP, 6), (6800, X, 8), (7000, X, 8),  # slot, name, overwrite
        *[(f, UP, 6) for f in range(7150, 7450, 50)], (7450, X, 8),        # Return to Game
        (7600, X, 8), (7750, X, 8), (7900, X, 8), (8050, UP, 6), (8100, X, 8), (8300, X, 8),  # save progress
    ]
    for f, b, n in seq:
        press(ev, f, b, n)
    gameplay(ev, 8500, 1500, seed=2000)
    return ",".join(ev), 10000


def ctm_collectible():
    """From the Peak 2 station to the right: picks up a collectible ~3250."""
    return CTM + ",2420:0020:60,2600:0020:30", 3600


def ctm_fresh():
    """A new career on a fresh card: Zoe at the top of Peak 1, riding down."""
    ev = ["1600:0008:8,1800:0040:6,1900:4000:8,2000:4000:8"]
    gameplay(ev, 2300, 6000, seed=3000)
    return ",".join(ev), 8300


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
    for (peak, goal), names in TRANSPORT.items():
        kind = ["race", "free", "ride"][goal]
        for i, n in enumerate(names):
            out[f"ctm_tp_p{peak}_{kind}_{n}"] = ctm_transport(peak, goal, i)
    out["ctm_lodge"] = ctm_lodge()
    out["ctm_collectible"] = ctm_collectible()
    out["ctm_fresh"] = ctm_fresh()
    out["monkey"] = monkey()
    return out


def card(name):
    return "fresh" if name == "ctm_fresh" else CARD


if __name__ == "__main__":
    s = scenarios()
    cmd = sys.argv[1] if len(sys.argv) > 1 else "list"
    if cmd == "list":
        print("\n".join(s))
    elif cmd == "input":
        print(s[sys.argv[2]][0])
    elif cmd == "frames":
        print(s[sys.argv[2]][1])
    elif cmd == "card":
        print(card(sys.argv[2]))
