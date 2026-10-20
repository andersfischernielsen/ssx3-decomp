#!/usr/bin/env python3
"""Integrate one phase-2 wave: wire modules, regenerate, test, replay gate.

    python3 tools/integrate_wave.py prep   oc37 oc38 ...   # overrides + features
    python3 tools/integrate_wave.py regen                  # both generated forms
    python3 tools/integrate_wave.py repoint oc37 ...       # tests -> __generated
    python3 tools/integrate_wave.py test   oc37 ...        # module tests
    python3 tools/integrate_wave.py build                  # ssx3-headless
    python3 tools/integrate_wave.py gate                   # race replay vs baseline
    python3 tools/integrate_wave.py all    oc37 ...        # everything above

The baseline RAM is the deterministic race replay of the accepted tree
(SSX3_INPUT race, 6000 frames, SSX3_RAM_DUMP); the gate compares the whole of
EE RAM at exit. Run one heavy step at a time (builds and game runs).
"""
import os
import re
import shutil
import subprocess
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SCRATCH = os.environ.get(
    "SSX3_SCRATCH",
    "/private/tmp/claude-501/-Users-fischer-Desktop-ssx3-decomp/"
    "68f2ad6e-0159-47ca-a8a9-b59b61a6720b/scratchpad",
)
BASELINE = os.path.join(SCRATCH, "base", "ds1.bin")
RACE_INPUT = (
    "1600:0008:8,2000:4000:8,2500:4000:8,2700:4000:8,2900:4000:8,"
    "3100:4000:8,3300:4000:8,3500:4000:8,3700:4000:8,4250:4000:8,"
    "4600:4000:3000"
)


def run(cmd, **kw):
    print("+", " ".join(cmd), flush=True)
    return subprocess.run(cmd, cwd=ROOT, check=True, **kw)


def module_overrides(m):
    path = os.path.join(ROOT, "crates", "ssx3-game", "src", f"{m}.rs")
    text = open(path).read()
    i = text.index("pub const OVERRIDES")
    block = text[i: text.index("];", i)]
    out = []
    for a, fn, name in re.findall(r'\(0x([0-9a-f_]+),\s*([\w:]+),\s*"([^"]+)"\)', block):
        out.append((int(a.replace("_", ""), 16), fn, name, m))
    return out


def prep(mods):
    addrs = []
    for m in mods:
        addrs += module_overrides(m)
    seen = set()
    for l in open(os.path.join(ROOT, "crates/ssx3-game/overrides.txt")):
        l = l.strip()
        if l and not l.startswith("#"):
            try:
                seen.add(int(l.split()[0], 16))
            except ValueError:
                pass
    dups = [n for a, _f, n, _m in addrs if a in seen]
    add = "".join(f"0x{a:08x}    {n}\n" for a, _f, n, _m in addrs if a not in seen)
    p = os.path.join(ROOT, "crates/ssx3-game/overrides.txt")
    s = open(p).read()
    s = s.replace("# debug slots", f"# Phase 2 wave: {', '.join(mods)}\n" + add + "# debug slots", 1)
    open(p, "w").write(s)

    p = os.path.join(ROOT, "crates/ssx3-game/Cargo.toml")
    s = open(p).read()
    line = [l for l in s.splitlines() if l.startswith("packages = [")][0]
    new = line[:-1] + "".join(f', "{m}"' for m in mods if f'"{m}"' not in line) + "]"
    open(p, "w").write(s.replace(line, new, 1))
    print(f"{len(addrs)} overrides ({len(dups)} already present: {dups})")


def regen():
    if shutil.which("cargo") is None:
        sys.exit("cargo not found")
    run(["cargo", "run", "--release", "-p", "ssx3-recomp", "--bin", "recomp-ee"])
    run([
        "cargo", "run", "--release", "-p", "ssx3-recomp", "--bin", "recomp-ee",
        "--", "--unwind", "--out", "generated/ee-unwind",
        "--package-prefix", "ssx3-ee-unwind",
    ])


def repoint(mods):
    addrs = {}
    for m in mods:
        for a, _f, _n, _m in module_overrides(m):
            addrs[a] = m
    for m in mods:
        p = os.path.join(ROOT, "crates/ssx3-game", "src", f"{m}.rs")
        s = open(p).read()
        i = s.index("#[cfg(test)]")
        n = [0]

        def repl(mo):
            if int(mo.group(1), 16) in addrs and addrs[int(mo.group(1), 16)] == m:
                n[0] += 1
                return f"::f_{mo.group(1)}__generated"
            return mo.group(0)

        t = re.sub(r"::f_([0-9a-f]{8})(?![_0-9a-zA-Z])", repl, s[i:])
        open(p, "w").write(s[:i] + t)
        print(m, n[0], "test references")


def test(mods):
    filters = [f"{m}::" for m in mods]
    run(["cargo", "test", "--profile", "game", "-p", "ssx3-game", "--lib", "--", *filters])


def build():
    run(["cargo", "build", "--profile", "game", "-p", "ssx3", "--bin", "ssx3-headless"])


def gate():
    out = tempfile.mktemp(suffix=".bin")
    env = dict(os.environ, SSX3_RAM_DUMP=out, SSX3_INPUT=RACE_INPUT)
    run(["./target/game/ssx3-headless", "6000"], env=env, stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL)
    same = open(out, "rb").read() == open(BASELINE, "rb").read()
    os.unlink(out)
    if not same:
        sys.exit("GATE FAILED: race replay RAM differs from the baseline")
    print("GATE: race replay RAM identical to the baseline")


def main(argv):
    if not argv:
        sys.exit(__doc__)
    mode, mods = argv[0], argv[1:]
    if mode == "prep":
        prep(mods)
    elif mode == "regen":
        regen()
    elif mode == "repoint":
        repoint(mods)
    elif mode == "test":
        test(mods)
    elif mode == "build":
        build()
    elif mode == "gate":
        gate()
    elif mode == "all":
        prep(mods)
        regen()
        repoint(mods)
        test(mods)
        build()
        gate()
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main(sys.argv[1:])
