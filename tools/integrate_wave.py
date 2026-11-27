#!/usr/bin/env python3
"""Integrate one phase-2 wave: wire modules, regenerate, test, replay gate.

    python3 tools/integrate_wave.py prep   oc37 oc38 ...   # overrides + features
    python3 tools/integrate_wave.py regen                  # both generated forms
    python3 tools/integrate_wave.py repoint oc37 ...       # tests -> __generated
    python3 tools/integrate_wave.py test   oc37 ...        # module tests
    python3 tools/integrate_wave.py build                  # ssx3-headless
    python3 tools/integrate_wave.py gate                   # race replay vs baseline
    python3 tools/integrate_wave.py all    oc37 ...        # everything above
    python3 tools/integrate_wave.py race-input             # print the gate's SSX3_INPUT

    python3 tools/integrate_wave.py slots  WAVE.txt ...    # slot-only lines for a staged wave + regen
    python3 tools/integrate_wave.py verify [oc37 ...]      # lint + prep + strict tests, small cycle budget
    python3 tools/integrate_wave.py prune                  # drop unused wave slots + regen
    python3 tools/integrate_wave.py moved-tests [REV]      # strict tests of the units changed since REV

`slots` (right after `stage_wave.py stage`, while the writers work) puts every
function of the wave AND every function those call in overrides.txt as slot-only
lines tagged `# wave-slot` (a callee without a slot cannot be faked in a test)
and regenerates, so `f_X__generated` exists and tests can name it from the
start (no `repoint`). `verify` is run repeatedly while the writers report: it
lints the modules that have arrived (all not-yet-prepped `ocN` modules with
entries when none are named), preps them, runs their strict tests with
`SSX3_CYCLE_BUDGET=5000000` (runaways fail in seconds) and summarises the
failures per module in `target/gate/fail/` (`tools/failsum.py`). `prune` at
the end of the cycle removes the wave slots nobody registered or references.
`moved-tests` replaces the full-suite run after the reorg: it runs only the
units that changed (the full suite runs at the end of a session).

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
# Gate data (the baseline RAM dump, base/ds1.bin): kept out of git.
SCRATCH = os.environ.get("SSX3_SCRATCH", os.path.join(ROOT, "target", "gate"))
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
    for a, fn, name in re.findall(r'\(\s*0x([0-9a-f_]+)\s*,\s*([\w:]+)\s*,\s*"([^"]+)"\s*,?\s*\)', block):
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
    # The names CI builds the hand-written crates against (commit it).
    run([sys.executable, "tools/ee_skeleton.py", "dump"])


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
    # The baseline is the old schedule: SND as a priority-scheduled thread. The
    # step mode (the default) is checked with tools/accept.py.
    env = dict(os.environ, SSX3_RAM_DUMP=out, SSX3_INPUT=RACE_INPUT, SSX3_SND_THREAD="1")
    run(["./target/game/ssx3-headless", "6000"], env=env, stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL)
    same = open(out, "rb").read() == open(BASELINE, "rb").read()
    os.unlink(out)
    if not same:
        sys.exit("GATE FAILED: race replay RAM differs from the baseline")
    print("GATE: race replay RAM identical to the baseline")


SLOT_TAG = "# wave-slot"
OVERRIDES_TXT = os.path.join(ROOT, "crates/ssx3-game/overrides.txt")
OVERRIDE_ENTRY = re.compile(
    r'\(\s*(0x[0-9a-fA-F_]{8,10})\s*,\s*(?:ee::\w+::\w+\s*,\s*)?[\w:]+\s*,\s*"[\w]+"')


def listed_addrs():
    out = set()
    for l in open(OVERRIDES_TXT):
        w = l.split("#")[0].split()
        if w:
            try:
                out.add(int(w[0], 16))
            except ValueError:
                pass
    return out


def source_text():
    parts = []
    for d, _dirs, files in os.walk(os.path.join(ROOT, "crates/ssx3-game/src")):
        for f in files:
            if f.endswith(".rs"):
                parts.append(open(os.path.join(d, f)).read())
    return "\n".join(parts)


def callee_targets(addr):
    """Call targets (function starts) in the generated body of `addr`."""
    sys.path.insert(0, os.path.join(ROOT, "tools"))
    from lint_port import generated_body

    body = generated_body(addr) or ""
    t = set(re.findall(r"call_indirect\(c, (0x[0-9a-f]+)\)", body))
    t |= set(re.findall(r"c\.pc = (0x[0-9a-f]+); \w+\(c\);", body))
    return {int(x, 16) for x in t}


def function_names():
    out = {}
    for l in open(os.path.join(ROOT, "generated/functions.tsv")):
        p = l.rstrip("\n").split("\t")
        try:
            out[int(p[0], 16)] = p[2] if len(p) > 2 else f"func_{p[0].upper()}"
        except ValueError:
            pass
    return out


def slots(files):
    """Slot-only lines for every function of the wave files and for every
    function those call: a test can only fake (`fakes::hook`) a callee that
    has an override slot, and one without runs for real (allocator spins)."""
    seen = listed_addrs()
    names = function_names()
    new = []
    wave = []
    for f in files:
        for l in open(f):
            if l.startswith("0x"):
                a, n = l.rstrip("\n").split("\t")[:2]
                wave.append(int(a, 16))
                if int(a, 16) not in seen:
                    seen.add(int(a, 16))
                    new.append(f"{a}    {n}    {SLOT_TAG}\n")
    for a in wave:
        for t in sorted(callee_targets(a)):
            if t not in seen and t in names:
                seen.add(t)
                new.append(f"0x{t:08x}    {names[t]}    {SLOT_TAG}\n")
    with open(OVERRIDES_TXT, "a") as f:
        f.write("".join(new))
    print(f"{len(new)} wave slot(s) added")
    regen()


def prune():
    text = open(OVERRIDES_TXT).read()
    src = source_text()
    registered = {int(m.group(1).replace("_", ""), 16) for m in OVERRIDE_ENTRY.finditer(src)}
    keep, dropped, kept_tag = [], 0, 0
    for l in text.splitlines(keepends=True):
        if SLOT_TAG in l:
            a = int(l.split()[0], 16)
            used = a in registered or re.search(rf"f_{a:08x}__generated|OVERRIDE_{a:08X}", src)
            if not used:
                dropped += 1
                continue
            l = l.replace("    " + SLOT_TAG, "")
            kept_tag += 1
        keep.append(l)
    open(OVERRIDES_TXT, "w").write("".join(keep))
    print(f"pruned {dropped} unused wave slot(s); {kept_tag} kept (registered or referenced)")
    if dropped:
        regen()


def staged_modules():
    """ocN modules with OVERRIDES entries that are not in the default features yet."""
    cargo = open(os.path.join(ROOT, "crates/ssx3-game/Cargo.toml")).read()
    line = [l for l in cargo.splitlines() if l.startswith("packages = [")][0]
    out = []
    src = os.path.join(ROOT, "crates/ssx3-game/src")
    for f in sorted(os.listdir(src), key=lambda x: (len(x), x)):
        m = re.fullmatch(r"(oc\d+)\.rs", f)
        if m and f'"{m.group(1)}"' not in line and module_overrides(m.group(1)):
            out.append(m.group(1))
    return out


def verify(mods):
    mods = mods or staged_modules()
    if not mods:
        print("no staged modules with entries")
        return
    print("modules:", " ".join(mods))
    subprocess.run([sys.executable, "tools/lint_port.py", *mods], cwd=ROOT)
    prep(mods)
    gen = open(os.path.join(ROOT, "generated/ee-symbols.tsv")).read()
    if any(f"{a:08x}" not in gen for m in mods for a, _f, _n, _m in module_overrides(m)):
        print("some slots are missing from the generated code: regenerating")
        regen()
    repoint(mods)
    tag = f"{mods[0]}-{mods[-1]}"
    log = os.path.join(SCRATCH, f"verify-{tag}.log")
    env = dict(os.environ, SSX3_DIFF_STRICT="args", SSX3_CYCLE_BUDGET="5000000")
    with open(log, "w") as f:
        r = subprocess.run(
            ["cargo", "test", "--profile", "game", "-p", "ssx3-game", "--lib", "--",
             *[f"{m}::" for m in mods]],
            cwd=ROOT, env=env, stdout=f, stderr=subprocess.STDOUT)
    text = open(log).read()
    if "error[" in text or "could not compile" in text:
        errs = re.findall(r"^error.*\n\s+--> (\S+)", text, re.M)
        print(f"COMPILE ERRORS ({len(errs)}), see {log}")
        for m in re.finditer(r"^(error[^\n]*)\n\s+--> (\S+)", text, re.M):
            print(f"  {m.group(2)}: {m.group(1)[:100]}")
        sys.exit(1)
    subprocess.run([sys.executable, "tools/failsum.py", log], cwd=ROOT)
    print(f"log: {log}")


def moved_tests(rev="HEAD"):
    files = subprocess.run(["git", "diff", "--name-only", rev], cwd=ROOT,
                           capture_output=True, text=True).stdout.split()
    st = subprocess.run(["git", "status", "--porcelain"], cwd=ROOT, capture_output=True, text=True).stdout
    files += [l[3:] for l in st.splitlines()]
    filters = set()
    for f in files:
        m = re.fullmatch(r"crates/ssx3-game/src/(game/.+|difftest/\w+|[\w]+)\.rs", f)
        if m:
            path = m.group(1).removesuffix("/mod").replace("/", "::")
            filters.add(path + "::")
    if not filters:
        print("no changed units")
        return
    print(f"{len(filters)} unit(s):", " ".join(sorted(filters)))
    log = os.path.join(SCRATCH, "moved-tests.log")
    env = dict(os.environ, SSX3_DIFF_STRICT="args")
    with open(log, "w") as f:
        r = subprocess.run(
            ["cargo", "test", "--profile", "game", "-p", "ssx3-game", "--lib", "--", *sorted(filters)],
            cwd=ROOT, env=env, stdout=f, stderr=subprocess.STDOUT)
    subprocess.run([sys.executable, "tools/failsum.py", log], cwd=ROOT)
    print(f"log: {log}")
    sys.exit(r.returncode)


def main(argv):
    if not argv:
        sys.exit(__doc__)
    mode, mods = argv[0], argv[1:]
    if mode == "slots":
        slots(mods)
        return
    if mode == "prune":
        prune()
        return
    if mode == "verify":
        verify(mods)
        return
    if mode == "moved-tests":
        moved_tests(mods[0] if mods else "HEAD")
        return
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
    elif mode == "race-input":
        print(RACE_INPUT)
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
