#!/usr/bin/env python3
"""Split a picked wave into writer lists and staging modules, and unwire them
again after the reorg (AGENTS.md, porting loop).

    python3 tools/stage_wave.py stage WAVE.txt WAVEDIR FIRST_OC   # lists + modules
    python3 tools/stage_wave.py drafts WAVEDIR                    # dispatcher drafts
    python3 tools/stage_wave.py unstage oc313 oc314 ...           # after reorg apply

`stage` reads `WAVE.txt` (`0xADDR<TAB>name` lines from `pick_wave.py`) and
routes the functions to writers by what a small model can finish:

- resumable dispatchers (the generated body has an `__entries` twin) go to
  Sonnet lists of `SSX3_DISP_PER` (default 4); the list names a draft that
  `drafts` makes with `tools/port_dispatcher.py` once the slots exist;
- functions of `SSX3_BIG_BYTES` bytes or more (default 1300) get a Sonnet
  writer each;
- functions of `SSX3_SONNET_BYTES` bytes or more (default 500) go to Sonnet
  lists of `SSX3_SONNET_PER` (default 5);
- the rest go to Haiku lists of `SSX3_PER_WRITER` (default 8).

`WAVEDIR/models.tsv` has one `ocN<TAB>model<TAB>kind` row per writer list: spawn
each writer on that model. For each group `ocN` (from `FIRST_OC` up) it writes
`WAVEDIR/list-ocN.txt` (the functions with their dossier paths in
`WAVEDIR/ref/`), creates `crates/ssx3-game/src/ocN.rs` with an empty
`OVERRIDES`, and wires it: the `ocN` feature in `Cargo.toml`, `pub mod ocN`
and its `OVERRIDES` in `install()`'s list in `lib.rs`. The modules are not in
the default features until `integrate_wave.py prep` adds them.

`unstage` removes the `Cargo.toml` and `lib.rs` lines of modules that
`reorg_modules.py apply` has moved (and deleted).
"""
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CARGO = os.path.join(R, "crates/ssx3-game/Cargo.toml")
LIB = os.path.join(R, "crates/ssx3-game/src/lib.rs")
SRC = os.path.join(R, "crates/ssx3-game/src")

SKELETON = """//! Staging module {m} (wave {wave}): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply {m}`.

#![allow(unused_imports)]

use crate::abi::{{r32, ret}};
use crate::call::{{call_ptr, guest, sp, Callee, Frame}};
use crate::events::{{deliver_event_here, event_due}};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{{reg::*, sx}};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

#[cfg(test)]
mod tests {{
    use super::*;
}}
"""


def sizes():
    out = {}
    for l in open(os.path.join(R, "generated/functions.tsv")):
        p = l.split("\t")
        try:
            a, e = int(p[0], 16), int(p[1], 16)
        except (ValueError, IndexError):
            continue
        out[a] = max(e - a, 4)
    for l in open(os.path.join(R, "generated/ee-symbols.tsv")):
        p = l.rstrip("\n").split("\t")
        if l.startswith("#") or len(p) < 3:
            continue
        a, e = int(p[1], 16), int(p[2], 16)
        out.setdefault(a, max(e - a, 4))
    return out


def dispatchers():
    """Addresses whose generated body has an `__entries` twin (resumable)."""
    out = set()
    gen = os.path.join(R, "generated/ee")
    for shard in sorted(os.listdir(gen)):
        lib = os.path.join(gen, shard, "src/lib.rs")
        if os.path.exists(lib):
            out.update(int(x, 16) for x in re.findall(r"pub fn f_([0-9a-f]{8})__entries\(", open(lib).read()))
    return out


def route(funcs, size, disp):
    """Group `funcs` into (model, kind, [(addr, name)]) writer lists."""
    big = int(os.environ.get("SSX3_BIG_BYTES", 1300))
    mid = int(os.environ.get("SSX3_SONNET_BYTES", 500))
    per = int(os.environ.get("SSX3_PER_WRITER", 8))
    sper = int(os.environ.get("SSX3_SONNET_PER", 5))
    dper = int(os.environ.get("SSX3_DISP_PER", 4))
    buckets = {"disp": [], "mid": [], "small": []}
    groups = []
    for a, n in funcs:
        if a in disp:
            buckets["disp"].append((a, n))
        elif size.get(a, 0) >= big:
            groups.append(("sonnet", "big", [(a, n)]))
        elif size.get(a, 0) >= mid:
            buckets["mid"].append((a, n))
        else:
            buckets["small"].append((a, n))
    for key, model, kind, k in (("disp", "sonnet", "dispatcher", dper), ("mid", "sonnet", "mid", sper), ("small", "haiku", "small", per)):
        b = buckets[key]
        groups += [(model, kind, b[i : i + k]) for i in range(0, len(b), k)]
    return groups


def stage(wave_txt, wave_dir, first):
    size = sizes()
    funcs = []
    for l in open(wave_txt):
        if l.startswith("0x"):
            a, n = l.rstrip("\n").split("\t")[:2]
            funcs.append((int(a, 16), n))
    routed = route(funcs, size, dispatchers())
    groups = [g for _m, _k, g in routed]
    models = [(m, k) for m, k, _g in routed]
    wave = os.path.basename(wave_txt).split("-")[0]
    ref = os.path.join(os.path.abspath(wave_dir), "ref")
    mods = []
    for k, g in enumerate(groups):
        m = f"oc{first + k}"
        mods.append(m)
        path = os.path.join(SRC, f"{m}.rs")
        if os.path.exists(path):
            sys.exit(f"{path} exists: pick another FIRST_OC")
        open(path, "w").write(SKELETON.format(m=m, wave=wave))
        lines = [f"Your functions, in this order ({len(g)}), in crates/ssx3-game/src/{m}.rs:"]
        for a, n in g:
            d = f"{ref}/{a:#010x}"
            extra = f" (Ghidra pseudo-C: {d}.ghidra.c)" if os.path.exists(d + ".ghidra.c") else ""
            if models[k][1] == "dispatcher":
                extra += f" — DISPATCHER: start from the draft {os.path.abspath(wave_dir)}/draft/{a:#010x}.rs if it exists"
            lines.append(f"- {a:#010x} {n} ({size.get(a, 0)} B) — dossier {d}.md{extra}")
        open(os.path.join(wave_dir, f"list-{m}.txt"), "w").write("\n".join(lines) + "\n")
    wire(mods)
    with open(os.path.join(wave_dir, "models.tsv"), "w") as f:
        for m, (model, kind) in zip(mods, models):
            f.write(f"{m}\t{model}\t{kind}\n")
    for m, g, (model, kind) in zip(mods, groups, models):
        print(f"{m}: {model} {kind}, {len(g)} function(s), {sum(size.get(a, 0) for a, _ in g)} B")


def snake(name):
    s = re.sub(r"(?<=[a-z0-9])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])", "_", name)
    return re.sub(r"_+", "_", s).lower().strip("_")


def drafts(wave_dir):
    """Run tools/port_dispatcher.py for every dispatcher list entry (after the
    slots exist, i.e. after `integrate_wave.py slots` and its regen)."""
    import subprocess

    out = os.path.join(wave_dir, "draft")
    os.makedirs(out, exist_ok=True)
    gen = os.path.join(R, "generated/ee")
    where = {}
    for shard in sorted(os.listdir(gen)):
        lib = os.path.join(gen, shard, "src/lib.rs")
        if os.path.exists(lib):
            for x in re.findall(r"pub fn f_([0-9a-f]{8})__entries\(", open(lib).read()):
                where[int(x, 16)] = shard
    models = [l.split("\t") for l in open(os.path.join(wave_dir, "models.tsv")).read().splitlines()]
    n = 0
    for m, _model, kind in models:
        if kind != "dispatcher":
            continue
        for l in open(os.path.join(wave_dir, f"list-{m}.txt")):
            mo = re.match(r"- (0x[0-9a-f]{8}) (\S+)", l)
            if not mo:
                continue
            a, name = mo.group(1), mo.group(2)
            r = subprocess.run(
                [sys.executable, "tools/port_dispatcher.py", a, where[int(a, 16)], name, snake(name)],
                cwd=R, capture_output=True, text=True,
            )
            open(os.path.join(out, f"{a}.rs"), "w").write(r.stdout)
            if r.stderr.strip():
                print(f"{a} {name}: {r.stderr.strip().splitlines()[0]}")
            n += 1
    print(f"{n} draft(s) in {out}")


def wire(mods):
    s = open(CARGO).read()
    anchor = "world_painter = []\n"
    s = s.replace(anchor, "".join(f"{m} = []\n" for m in mods) + anchor, 1)
    open(CARGO, "w").write(s)
    s = open(LIB).read()
    anchor = '#[cfg(feature = "rider_logic")]\npub mod rider_logic;\n'
    s = s.replace(anchor, "".join(f'#[cfg(feature = "{m}")]\npub mod {m};\n' for m in mods) + anchor, 1)
    anchor = "    v.extend(game::overrides());\n"
    s = s.replace(
        anchor,
        anchor + "".join(f'    #[cfg(feature = "{m}")]\n    v.extend_from_slice({m}::OVERRIDES);\n' for m in mods),
        1,
    )
    open(LIB, "w").write(s)


def unstage(mods):
    for m in mods:
        if os.path.exists(os.path.join(SRC, f"{m}.rs")):
            sys.exit(f"{m}.rs still exists: run reorg_modules.py apply {m} first")
    s = open(CARGO).read()
    for m in mods:
        s = s.replace(f"{m} = []\n", "")
        s = s.replace(f', "{m}"', "")
    open(CARGO, "w").write(s)
    s = open(LIB).read()
    for m in mods:
        s = s.replace(f'#[cfg(feature = "{m}")]\npub mod {m};\n', "")
        s = s.replace(f'    #[cfg(feature = "{m}")]\n    v.extend_from_slice({m}::OVERRIDES);\n', "")
    open(LIB, "w").write(s)
    left = [m for m in mods if re.search(rf"\b{m}\b", open(CARGO).read() + open(LIB).read())]
    print("unstaged" if not left else f"left over (fix by hand): {left}")


def main(argv):
    if len(argv) == 4 and argv[0] == "stage":
        stage(argv[1], argv[2], int(argv[3].removeprefix("oc")))
    elif len(argv) == 2 and argv[0] == "drafts":
        drafts(argv[1])
    elif len(argv) > 1 and argv[0] == "unstage":
        unstage(argv[1:])
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main(sys.argv[1:])
