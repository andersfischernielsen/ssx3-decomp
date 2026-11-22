#!/usr/bin/env python3
"""Mutation-test ported functions with cargo-mutants
(docs/porting/mutation-testing.md).

    python3 tools/mutants.py UNIT FUNCTION...   # these functions of one unit
    python3 tools/mutants.py --in-diff PR.diff  # every function a diff touches
    python3 tools/mutants.py --list ...         # count the mutants, run nothing

UNIT is a source file (`crates/ssx3-game/src/game/dirtysock/tags.rs`).
PR.diff: `git diff origin/main... > PR.diff` on the PR's branch. Lists the
mutants once, drops those on `c.cycles +=` lines (a cycle charge's `-=` or
`*=` is always caught, or equivalent where the count happens to match, and
they are more than half of all mutants), then runs the rest in place
(`.cargo/mutants.toml`: the `game` profile, incremental,
`SSX3_DIFF_STRICT=args`) with only the touched units' tests, and prints each
function's counts and its surviving mutants. Results: `target/mutants.out/`.
"""
import argparse
import collections
import json
import os
import re
import subprocess
import sys
import time

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = "crates/ssx3-game/src/"
CYCLES = re.compile(r"^\s*(?:\$?c|ctx)\.cycles\s*\+=")
OUT = os.path.join(ROOT, "target", "mutants.out", "outcomes.json")


def test_filter(path):
    """crates/ssx3-game/src/game/dirtysock/tags.rs -> game::dirtysock::tags::"""
    mod = path[len(SRC):-len(".rs")]
    if mod.endswith("/mod"):
        mod = mod[: -len("/mod")]
    return mod.replace("/", "::") + "::"


def line(path, n, cache={}):
    if path not in cache:
        with open(os.path.join(ROOT, path)) as f:
            cache[path] = f.read().splitlines()
    return cache[path][n - 1]


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("unit", nargs="?", help="source file of the functions")
    ap.add_argument("functions", nargs="*", help="function names")
    ap.add_argument("--in-diff", metavar="DIFF", help="mutate the lines this diff adds")
    ap.add_argument("--list", action="store_true", help="count the mutants only")
    a = ap.parse_args()
    if bool(a.in_diff) == bool(a.unit and a.functions):
        ap.error("give UNIT FUNCTION... or --in-diff DIFF")

    cmd = ["cargo", "mutants", "--package", "ssx3-game"]
    if a.in_diff:
        # Operator mutants only (`... in <fn>`): `replace <fn> with ()` is always caught.
        cmd += ["--in-diff", os.path.abspath(a.in_diff), "-F", " in "]
    else:
        names = "|".join(re.escape(f) for f in a.functions)
        cmd += ["--file", a.unit, "-F", rf"in (?:{names})\b"]
    out = subprocess.run(cmd + ["--list", "--json"], cwd=ROOT,
                         check=True, capture_output=True, text=True).stdout
    listed = json.loads(out) if out.strip() else []  # nothing printed: no mutants
    if a.functions:
        listed = [m for m in listed if m["function"]["function_name"] in a.functions]

    cycle_lines = collections.defaultdict(set)
    kept = collections.Counter()
    for m in listed:
        n = m["span"]["start"]["line"]
        if "+=" in m["name"] and CYCLES.match(line(m["file"], n)):
            cycle_lines[m["file"]].add(n)
        else:
            kept[m["function"]["function_name"]] += 1
    for fn, n in sorted(kept.items()):
        print(f"{n:5}  {fn}")
    print(f"{sum(kept.values()):5}  mutants ({sum(map(len, cycle_lines.values()))} cycle-charge lines left out)")
    if a.list or not kept:
        return

    for path, lines in sorted(cycle_lines.items()):
        lines = sorted(lines)
        for i in range(0, len(lines), 50):
            alt = "|".join(map(str, lines[i:i + 50]))
            cmd += ["-E", rf"^{re.escape(path)}:(?:{alt}):\d+: replace \+= with "]
    units = sorted({m["file"] for m in listed})
    cmd += ["--in-place", "--output", "target", "--"] + [test_filter(u) for u in units]
    t0 = time.monotonic()
    r = subprocess.run(cmd, cwd=ROOT)
    wall = time.monotonic() - t0
    if r.returncode not in (0, 2, 3):  # 2: survivors, 3: timeouts
        sys.exit(r.returncode)

    with open(OUT) as f:
        outcomes = json.load(f)["outcomes"]
    counts = collections.defaultdict(collections.Counter)
    missed, done = [], 0
    for o in outcomes:
        m = o["scenario"].get("Mutant") if isinstance(o["scenario"], dict) else None
        if not m:
            continue
        fn = m["function"]["function_name"]
        counts[fn][o["summary"]] += 1
        done += 1
        if o["summary"] == "MissedMutant":
            n = m["span"]["start"]["line"]
            col = m["span"]["start"]["column"]
            what = m["name"].split(": ", 1)[1].rsplit(" in ", 1)[0]
            missed.append(f"  {m['file']}:{n}:{col}  {fn}: {what}\n      {line(m['file'], n).strip()}")
    print(f"\n{'caught':>6} {'missed':>6} {'unviable':>8} {'timeout':>7}  function")
    for fn in sorted(counts):
        c = counts[fn]
        print(f"{c['CaughtMutant']:6} {c['MissedMutant']:6} {c['Unviable']:8} {c['Timeout']:7}  {fn}")
    if done:
        print(f"{done} mutants in {wall:.0f} s ({wall / done:.1f} s each, with the baseline)")
    if missed:
        print("\nSurvivors:")
        print("\n".join(missed))
    sys.exit(1 if missed else 0)


if __name__ == "__main__":
    main()
