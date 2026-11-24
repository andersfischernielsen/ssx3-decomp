#!/usr/bin/env python3
"""Mutation-test ported functions (docs/porting/mutation-testing.md).

    python3 tools/mutants.py UNIT FUNCTION...   # these functions of one unit
    python3 tools/mutants.py --in-diff PR.diff  # every function a diff touches
    python3 tools/mutants.py --list ...         # count the mutants, run nothing
    python3 tools/mutants.py --cargo-mutants ...  # one rebuild per mutant

UNIT is a source file (`crates/ssx3-game/src/game/dirtysock/tags.rs`).
PR.diff: `git diff origin/main... > PR.diff` on the PR's branch. Lists the
mutants once with cargo-mutants, drops those on `c.cycles +=` lines (a cycle
charge's `-=` or `*=` is always caught, or equivalent where the count happens
to match, and they are more than half of all mutants), then builds ssx3-game
once with every mutant behind a switch and runs the touched units' tests once
per mutant with only its switch on. `--cargo-mutants` runs cargo-mutants in
place instead, one rebuild per mutant. Both use `.cargo/mutants.toml` (the
`game` profile, incremental, `SSX3_DIFF_STRICT=args`) and print each
function's counts and its surviving mutants. Results: `target/mutants.out/`.
"""
import argparse
import collections
import json
import os
import re
import shutil
import subprocess
import sys
import time
import tomllib

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = "crates/ssx3-game/src/"
CYCLES = re.compile(r"^\s*(?:\$?c|ctx)\.cycles\s*\+=")
OUT_DIR = os.path.join(ROOT, "target", "mutants.out")
OUT = os.path.join(OUT_DIR, "outcomes.json")
SPANS = os.path.join(ROOT, "tools", "mutant-spans")
SWITCH = "SSX3_MUTANT"


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


def run_cargo_mutants(cmd, filters, cycle_lines):
    """One rebuild per mutant: cargo-mutants in place. Returns [(mutant, summary)]."""
    for path, lines in sorted(cycle_lines.items()):
        lines = sorted(lines)
        for i in range(0, len(lines), 50):
            alt = "|".join(map(str, lines[i:i + 50]))
            cmd += ["-E", rf"^{re.escape(path)}:(?:{alt}):\d+: replace \+= with "]
    cmd += ["--in-place", "--output", "target", "--"] + filters
    r = subprocess.run(cmd, cwd=ROOT)
    if r.returncode not in (0, 2, 3):  # 2: survivors, 3: timeouts
        sys.exit(r.returncode)
    with open(OUT) as f:
        outcomes = json.load(f)["outcomes"]
    return [(o["scenario"]["Mutant"], o["summary"]) for o in outcomes
            if isinstance(o["scenario"], dict) and "Mutant" in o["scenario"]]


class Unit:
    """A source file and its mutants (`s`, `e`: character offsets of the span)."""

    def __init__(self, path, muts):
        self.path = path
        with open(os.path.join(ROOT, path)) as f:
            self.text = f.read()
        starts = [0]
        for ln in self.text.split("\n"):
            starts.append(starts[-1] + len(ln) + 1)
        self.off = lambda ln, col: starts[ln - 1] + col - 1
        self.muts = muts
        for m in muts:
            sp = m["span"]
            m["s"] = self.off(sp["start"]["line"], sp["start"]["column"])
            m["e"] = self.off(sp["end"]["line"], sp["end"]["column"])

    def place(self, spans_bin):
        """Switch each mutant at the smallest expression around it. Function-value
        mutants and those in a const context or a macro call are built alone."""
        out = subprocess.run([spans_bin, os.path.join(ROOT, self.path)], check=True,
                             capture_output=True, text=True).stdout
        exprs = []
        for ln in out.splitlines():
            sl, sc, el, ec, flags = ln.split("\t")
            exprs.append((self.off(int(sl), int(sc)), self.off(int(el), int(ec)), flags))
        for m in self.muts:
            around = [x for x in exprs if x[0] <= m["s"] and m["e"] <= x[1]]
            if m["genre"] == "FnValue" or not around:
                continue
            s, e, flags = min(around, key=lambda x: x[1] - x[0])
            if "c" not in flags and "m" not in flags:
                m["at"], m["bare"] = (s, e), "s" in flags

    def render(self, on):
        """The file with a switch for each mutant in `on`:
        `(if __mutant(7) { <mutant 7> } else if __mutant(8) { ... } else { <original> })`.
        Each branch is the original text with one mutant; smaller switches nest
        in the else branch. `/*+7*/ ... /*-7*/` marks mutant 7's lines."""
        groups = collections.defaultdict(list)
        for m in self.muts:
            if m["id"] in on:
                groups[m["at"]].append(m)
        spans = sorted(groups, key=lambda u: (u[0], -u[1]))
        t = self.text
        call = f"crate::{test_filter(self.path)}__mutant"

        def fill(s, e):
            out, pos = [], s
            for u in spans:
                if u[0] >= pos and u[1] <= e and u != (s, e):
                    out += [t[pos:u[0]], switch(u)]
                    pos = u[1]
            return "".join(out) + t[pos:e]

        def switch(u):
            ms = sorted(groups[u], key=lambda m: m["id"])
            arms = [f"if {call}({m['id']}) {{/*+{m['id']}*/\n"
                    f"{t[u[0]:m['s']]}{m['replacement']}{t[m['e']:u[1]]}\n/*-{m['id']}*/}}" for m in ms]
            body = " else ".join(arms) + " else {\n" + fill(*u) + "\n}"
            return body if ms[0]["bare"] else f"({body})"

        return fill(0, len(t)) + (
            "\n#[allow(dead_code)]\nfn __mutant(id: u32) -> bool {\n"
            "    static ON: std::sync::OnceLock<u32> = std::sync::OnceLock::new();\n"
            f"    *ON.get_or_init(|| std::env::var(\"{SWITCH}\").ok().and_then(|v| v.parse().ok()).unwrap_or(0))\n"
            "        == id\n}\n")

    def solo(self, m):
        return self.text[:m["s"]] + m["replacement"] + self.text[m["e"]:]


def owners(path, text):
    """{(file, line): mutant id} for the lines of each switch branch."""
    own = {}
    for mo in re.finditer(r"/\*\+(\d+)\*/\n(.*?)\n/\*-\1\*/", text, re.S):
        first = text.count("\n", 0, mo.start(2)) + 1
        for n in range(first, first + mo.group(2).count("\n") + 1):
            own[path, n] = int(mo.group(1))
    return own


def build(args):
    """Build the lib test binary: (executable or None, compiler error messages)."""
    p = subprocess.run(["cargo", "test", "--no-run", "--package", "ssx3-game", *args,
                        "--message-format=json"], cwd=ROOT, capture_output=True, text=True)
    exe, errors = None, []
    for ln in p.stdout.splitlines():
        j = json.loads(ln) if ln.startswith("{") else {}
        if j.get("reason") == "compiler-artifact" and j.get("executable"):
            exe = j["executable"]
        if j.get("reason") == "compiler-message" and j["message"]["level"] == "error":
            errors.append(j["message"])
    if p.returncode and not errors:
        sys.exit(f"build failed:\n{p.stderr[-3000:]}")
    return (exe if p.returncode == 0 else None), errors


def run_switched(listed, filters):
    """Build once with every mutant behind a switch, then test each one.
    Returns [(mutant, summary)] in cargo-mutants' terms."""
    with open(os.path.join(ROOT, ".cargo", "mutants.toml"), "rb") as f:
        cfg = tomllib.load(f)
    args = ["--profile", cfg["profile"], *cfg["additional_cargo_args"], *cfg["additional_cargo_test_args"]]
    env = dict(os.environ, CARGO_MANIFEST_DIR=os.path.join(ROOT, "crates", "ssx3-game"))
    env.update(re.findall(r'^env\.(\w+)="(.*)"$', "\n".join(cfg["additional_cargo_args"]), re.M))
    env.pop(SWITCH, None)

    def test(exe, mid=None, timeout=None):
        e = dict(env, **({SWITCH: str(mid)} if mid else {}))
        t0 = time.monotonic()
        try:
            p = subprocess.run([exe, *filters], cwd=env["CARGO_MANIFEST_DIR"], env=e,
                               capture_output=True, text=True, timeout=timeout)
        except subprocess.TimeoutExpired:
            return "Timeout", time.monotonic() - t0, ""
        summary = "MissedMutant" if p.returncode == 0 else "CaughtMutant"
        return summary, time.monotonic() - t0, p.stdout + p.stderr

    spans_bin = os.path.join(ROOT, "target", "mutant-spans", "release", "mutant-spans")
    subprocess.run(["cargo", "build", "--release", "--locked", "--quiet", "--target-dir",
                    os.path.dirname(os.path.dirname(spans_bin))], cwd=SPANS, check=True)
    for i, m in enumerate(listed, 1):
        m["id"] = i
    units = {}
    for m in listed:
        units.setdefault(m["file"], []).append(m)
    units = {p: Unit(p, ms) for p, ms in units.items()}
    for u in units.values():
        u.place(spans_bin)
    os.makedirs(os.path.join(OUT_DIR, "log"))

    def write(on=(), solo=None):
        texts = {}
        for p, u in units.items():
            texts[p] = u.solo(solo) if solo and solo["file"] == p else u.render(on) if on else u.text
            with open(os.path.join(ROOT, p), "w") as f:
                f.write(texts[p])
        return texts

    def bisect(ids):
        if len(ids) == 1:
            return set(ids)
        bad = set()
        for part in (ids[:len(ids) // 2], ids[len(ids) // 2:]):
            write(set(part))
            if not build(args)[0]:
                bad |= bisect(part)
        return bad

    results = {}
    on = {m["id"] for m in listed if "at" in m}
    try:
        while True:
            texts = write(on)
            own = {}
            for p, t in texts.items():
                own.update(owners(p, t))
            exe, errors = build(args)
            if exe:
                break
            if not on:
                sys.exit("ssx3-game does not build; see `cargo test --no-run`")
            # Drop the mutants whose lines the errors start on; bisect if none.
            bad = {own[s["file_name"], s["line_start"]] for e in errors
                   for s in e["spans"] + [s for c in e["children"] for s in c["spans"]]
                   if (s["file_name"], s["line_start"]) in own} or bisect(sorted(on))
            if not bad:
                sys.exit("the switched build fails, but no single mutant does")
            print(f"{len(bad)} mutant(s) do not build behind a switch; building them alone")
            on -= bad
        os.makedirs(os.path.join(OUT_DIR, "switched"))
        for p, t in texts.items():
            with open(os.path.join(OUT_DIR, "switched", os.path.basename(p)), "w") as f:
                f.write(t)
        summary, base, out = test(exe)
        if summary != "MissedMutant":
            sys.exit(f"the unmutated tests fail:\n{out[-3000:]}")
        timeout = max(20.0, 5 * base)  # cargo-mutants' default
        # The switched mutants first: a solo build overwrites the binary.
        for m in sorted(listed, key=lambda m: m["id"] not in on):
            if m["id"] in on:
                results[m["id"]] = test(exe, m["id"], timeout)
            else:
                write(solo=m)
                solo_exe = build(args)[0]
                results[m["id"]] = test(solo_exe, timeout=timeout) if solo_exe else ("Unviable", 0, "")
            summary, dt, out = results[m["id"]]
            print(f"{summary:12} {dt:6.1f} s  {m['name']}", flush=True)
            with open(os.path.join(OUT_DIR, "log", f"{m['id']}.txt"), "w") as f:
                f.write(f"{m['name']}\n{summary} {dt:.1f} s\n{out}")
    finally:
        write()
        for p, u in units.items():
            with open(os.path.join(ROOT, p)) as f:
                if f.read() != u.text:
                    sys.exit(f"{p} was not restored")
    names = {"CaughtMutant": "caught", "MissedMutant": "missed", "Unviable": "unviable", "Timeout": "timeout"}
    for summary, name in names.items():
        with open(os.path.join(OUT_DIR, f"{name}.txt"), "w") as f:
            f.writelines(m["name"] + "\n" for m in listed if results[m["id"]][0] == summary)
    return [(m, results[m["id"]][0]) for m in listed]


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("unit", nargs="?", help="source file of the functions")
    ap.add_argument("functions", nargs="*", help="function names")
    ap.add_argument("--in-diff", metavar="DIFF", help="mutate the lines this diff adds")
    ap.add_argument("--list", action="store_true", help="count the mutants only")
    ap.add_argument("--cargo-mutants", action="store_true", help="rebuild for every mutant, with cargo-mutants")
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
    kept = []
    for m in listed:
        n = m["span"]["start"]["line"]
        if "+=" in m["name"] and CYCLES.match(line(m["file"], n)):
            cycle_lines[m["file"]].add(n)
        else:
            kept.append(m)
    for fn, n in sorted(collections.Counter(m["function"]["function_name"] for m in kept).items()):
        print(f"{n:5}  {fn}")
    print(f"{len(kept):5}  mutants ({sum(map(len, cycle_lines.values()))} cycle-charge lines left out)")
    if a.list or not kept:
        return

    filters = [test_filter(u) for u in sorted({m["file"] for m in listed})]
    t0 = time.monotonic()
    if a.cargo_mutants:
        results = run_cargo_mutants(cmd, filters, cycle_lines)
    else:
        if os.path.exists(OUT_DIR):
            shutil.rmtree(OUT_DIR + ".old", ignore_errors=True)
            os.rename(OUT_DIR, OUT_DIR + ".old")
        results = run_switched(kept, filters)
    wall = time.monotonic() - t0

    counts = collections.defaultdict(collections.Counter)
    missed = []
    for m, summary in results:
        fn = m["function"]["function_name"]
        counts[fn][summary] += 1
        if summary == "MissedMutant":
            n = m["span"]["start"]["line"]
            col = m["span"]["start"]["column"]
            what = m["name"].split(": ", 1)[1].rsplit(" in ", 1)[0]
            missed.append(f"  {m['file']}:{n}:{col}  {fn}: {what}\n      {line(m['file'], n).strip()}")
    print(f"\n{'caught':>6} {'missed':>6} {'unviable':>8} {'timeout':>7}  function")
    for fn in sorted(counts):
        c = counts[fn]
        print(f"{c['CaughtMutant']:6} {c['MissedMutant']:6} {c['Unviable']:8} {c['Timeout']:7}  {fn}")
    if results:
        print(f"{len(results)} mutants in {wall:.0f} s ({wall / len(results):.1f} s each, with the baseline)")
    if missed:
        print("\nSurvivors:")
        print("\n".join(missed))
    sys.exit(1 if missed else 0)


if __name__ == "__main__":
    main()
