#!/usr/bin/env python3
"""The EE code's symbols without its code, so CI can build the hand-written crates.

    python3 tools/ee_skeleton.py dump    # generated/ee -> generated/ee-symbols.tsv
    python3 tools/ee_skeleton.py write   # generated/ee-symbols.tsv -> generated/ee (skeleton)

`generated/ee` is the game's code translated, so it is never committed. The
hand-written crates (`ssx3-game`, `ssx3-engine`, the bins) name its functions,
override slots and generated bodies, so they cannot build without it. `dump`
records only those names (shard, address range, slot, resume entries,
aliases: no code, no data) in `generated/ee-symbols.tsv`; `write` turns them
into crates with the same API whose bodies panic. Installed overrides still
run through their slots.

`integrate_wave.py regen` runs `dump`, so the list follows `overrides.txt`.
`write` refuses to overwrite a real `generated/ee`.
"""
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
EE = os.path.join(ROOT, "generated", "ee")
TSV = os.path.join(ROOT, "generated", "ee-symbols.tsv")
SKELETON_MARKER = "//! Skeleton of the generated EE code"

ROW = re.compile(r"\s+FnEntry \{ start: 0x([0-9a-f]{8}), end: 0x([0-9a-f]{8}), f: ssx3_ee_(s\d\d)::f_\1 \},")


def dump():
    game = open(os.path.join(EE, "game", "src", "lib.rs")).read()
    if SKELETON_MARKER in game:
        sys.exit("error: generated/ee is the skeleton; dump needs the real generated code")
    rows = [m.groups() for m in map(ROW.match, game.splitlines()) if m]
    shards = sorted({s for _, _, s in rows})
    slot, entries, aliases = set(), set(), {}
    for s in shards:
        for line in open(os.path.join(EE, s, "src", "lib.rs")):
            if line.startswith("pub static mut OVERRIDE_"):
                slot.add(line[24:32].lower())
            elif line.startswith("pub fn f_") and line[17:26] == "__entries":
                entries.add(line[9:17])
            elif line.startswith("pub use self::f_"):
                m = re.match(r"pub use self::f_([0-9a-f]{8}) as (\w+);", line)
                aliases.setdefault(m[1], []).append(m[2])
    with open(TSV, "w") as f:
        f.write("# Symbols of generated/ee (tools/ee_skeleton.py dump): shard, start, end,\n"
                "# flags (o: override slot, e: resume entries), aliases. Names only, no code.\n")
        for start, end, s in rows:
            flags = ("o" if start in slot else "") + ("e" if start in entries else "")
            f.write(f"{s}\t{start}\t{end}\t{flags or '-'}\t{','.join(aliases.get(start, [])) or '-'}\n")
    print(f"{TSV}: {len(rows)} functions, {len(slot)} slots, {len(shards)} shards")


def read_tsv():
    rows = []
    for line in open(TSV):
        if line.startswith("#"):
            continue
        s, start, end, flags, al = line.rstrip("\n").split("\t")
        rows.append((s, start, end, flags, [] if al == "-" else al.split(",")))
    return rows


def manifest(name, deps):
    return (f'[package]\nname = "{name}"\nversion = "0.1.0"\nedition = "2021"\npublish = false\n\n'
            f'[lib]\npath = "src/lib.rs"\n\n[dependencies]\nssx3-ee-rt = {{ path = "../../../crates/ssx3-ee-rt" }}\n'
            + deps)


def put(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        f.write(text)


def write():
    lib = os.path.join(EE, "game", "src", "lib.rs")
    if os.path.isfile(lib) and SKELETON_MARKER not in open(lib).read() and "FnEntry; 0]" not in open(lib).read():
        sys.exit("error: generated/ee holds the real generated code; not overwriting it")
    rows = read_tsv()
    shards = sorted({r[0] for r in rows})
    for s in shards:
        out = [f"{SKELETON_MARKER} (tools/ee_skeleton.py): names only, bodies panic.\n"
               "#![allow(non_snake_case, non_upper_case_globals)]\nuse ssx3_ee_rt::Cpu;\n\n"
               "#[cold]\nfn absent(c: &mut Cpu) {\n"
               "    panic!(\"generated EE code is not present (skeleton build): pc {:#x}\", c.pc)\n}\n"]
        for sh, start, _, flags, al in rows:
            if sh != s:
                continue
            if "o" in flags:
                up = start.upper()
                out.append(f"pub fn f_{start}(c: &mut Cpu) {{ unsafe {{ OVERRIDE_{up}(c) }} }}\n"
                           f"pub fn f_{start}__generated(c: &mut Cpu) {{ absent(c) }}\n"
                           f"pub static mut OVERRIDE_{up}: fn(&mut Cpu) = f_{start}__generated;\n")
            else:
                out.append(f"pub fn f_{start}(c: &mut Cpu) {{ absent(c) }}\n")
            if "e" in flags:
                out.append(f"pub fn f_{start}__entries(c: &mut Cpu) {{ absent(c) }}\n")
            for a in al:
                out.append(f"pub use self::f_{start} as {a};\n")
        put(os.path.join(EE, s, "Cargo.toml"), manifest(f"ssx3-ee-{s}", ""))
        put(os.path.join(EE, s, "src", "lib.rs"), "".join(out))
    slots = [r for r in rows if "o" in r[3]]
    g = [f"{SKELETON_MARKER} (tools/ee_skeleton.py): the function table and slots.\n"
         "use ssx3_ee_rt::funcs::FnEntry;\n\n",
         f"pub static FUNCTIONS: [FnEntry; {len(rows)}] = [\n"]
    g += [f"    FnEntry {{ start: 0x{a}, end: 0x{b}, f: ssx3_ee_{s}::f_{a} }},\n" for s, a, b, _, _ in rows]
    g.append("];\n\n")
    g += [f"pub use ssx3_ee_{s} as {s};\n" for s in shards]
    g.append("\npub fn set_override(addr: u32, f: fn(&mut ssx3_ee_rt::Cpu)) -> bool {\n    match addr {\n")
    g += [f"        0x{a} => unsafe {{ ssx3_ee_{s}::OVERRIDE_{a.upper()} = f }},\n" for s, a, _, _, _ in slots]
    g.append("        _ => return false,\n    }\n    true\n}\n\n"
             "pub fn generated_body(addr: u32) -> Option<fn(&mut ssx3_ee_rt::Cpu)> {\n    Some(match addr {\n")
    g += [f"        0x{a} => ssx3_ee_{s}::f_{a}__generated,\n" for s, a, _, _, _ in slots]
    g.append("        _ => return None,\n    })\n}\n\npub const HOST_THREADS: bool = true;\n")
    deps = "".join(f'ssx3-ee-{s} = {{ path = "../{s}" }}\n' for s in shards)
    put(os.path.join(EE, "game", "Cargo.toml"), manifest("ssx3-ee", deps))
    put(lib, "".join(g))
    print(f"generated/ee skeleton: {len(rows)} functions, {len(slots)} slots, {len(shards)} shards")


if __name__ == "__main__":
    {"dump": dump, "write": write}.get(sys.argv[1] if len(sys.argv) > 1 else "", lambda: sys.exit(__doc__))()
