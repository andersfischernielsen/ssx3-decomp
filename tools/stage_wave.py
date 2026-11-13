#!/usr/bin/env python3
"""Split a picked wave into writer lists and staging modules, and unwire them
again after the reorg (AGENTS.md, porting loop).

    python3 tools/stage_wave.py stage WAVE.txt WAVEDIR FIRST_OC   # lists + modules
    python3 tools/stage_wave.py unstage oc313 oc314 ...           # after reorg apply

`stage` reads `WAVE.txt` (`0xADDR<TAB>name` lines from `pick_wave.py`) and
groups the functions for writers: one function of `SSX3_BIG_BYTES` bytes or
more (default 1300) per writer, the rest `SSX3_PER_WRITER` (default 10) per
writer, in wave order. For each group `ocN` (from `FIRST_OC` up) it writes
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
    return out


def stage(wave_txt, wave_dir, first):
    big = int(os.environ.get("SSX3_BIG_BYTES", 1300))
    per = int(os.environ.get("SSX3_PER_WRITER", 10))
    size = sizes()
    funcs = []
    for l in open(wave_txt):
        if l.startswith("0x"):
            a, n = l.rstrip("\n").split("\t")[:2]
            funcs.append((int(a, 16), n))
    groups, small = [], []
    for a, n in funcs:
        if size.get(a, 0) >= big:
            groups.append([(a, n)])
        else:
            small.append((a, n))
            if len(small) == per:
                groups.append(small)
                small = []
    if small:
        groups.append(small)
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
            lines.append(f"- {a:#010x} {n} ({size.get(a, 0)} B) — dossier {d}.md{extra}")
        open(os.path.join(wave_dir, f"list-{m}.txt"), "w").write("\n".join(lines) + "\n")
    wire(mods)
    for m, g in zip(mods, groups):
        print(f"{m}: {len(g)} function(s), {sum(size.get(a, 0) for a, _ in g)} B")


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
    elif len(argv) > 1 and argv[0] == "unstage":
        unstage(argv[1:])
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main(sys.argv[1:])
