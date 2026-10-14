//! Translate the whole EE inventory into Rust crates under `generated/ee/`.
//!
//! ```sh
//! cargo run --release -p ssx3-recomp --bin recomp-ee            # all functions
//! cargo run --release -p ssx3-recomp --bin recomp-ee -- --shards 16
//! ```
//!
//! Output (gitignored, fully derived from the ELF + `generated/*.tsv`):
//! * `generated/ee/sNN/` — shard crates `ssx3-ee-sNN`, each a contiguous
//!   address range of functions.
//! * `generated/ee/game/` — crate `ssx3-ee`, the address → function table.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use ssx3_core::elf::Elf;
use ssx3_recomp::eegen::{emit_function, emit_function_named, extend_runs, fn_name, targets, Ctx, Func, Image, Stats};
use ssx3_recomp::{boundaries, index};

struct ElfImage(Elf);

impl Image for ElfImage {
    fn word(&self, addr: u32) -> Option<u32> {
        self.0.read_u32(addr)
    }
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").canonicalize().unwrap()
}

fn main() {
    let mut shards = 16usize;
    let mut threads = false;
    let mut structured = false;
    let args: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--threads" {
            threads = true;
        } else if args[i] == "--structured" {
            threads = true;
            structured = true;
        } else if args[i] == "--shards" {
            shards = args[i + 1].parse().expect("--shards N");
            i += 1;
        }
        i += 1;
    }
    let root = repo_root();
    let elf = Elf::from_file(root.join("orig/SLUS_207.72")).expect("orig/SLUS_207.72 (see tools/extract-from-iso.sh)");
    let img = ElfImage(elf);

    // Functions with their instruction runs.
    let mut funcs: Vec<Func> = Vec::new();
    for f in index().functions() {
        let runs = boundaries()
            .runs(f.address)
            .map(|r| r.to_vec())
            .unwrap_or_else(|| vec![(f.address, f.end)]);
        let name = (!f.is_auto_named()).then(|| f.name.to_string());
        funcs.push(Func { start: f.address, end: f.end, runs, name });
    }
    funcs.sort_by_key(|f| f.start);

    // New function entries: pointers from data into .text that no known
    // function covers, plus entries recorded at run time (generated/entries.txt).
    let text_end = 0x0042_e590u32;
    let covered = |funcs: &[Func], a: u32| {
        let i = funcs.partition_point(|f| f.start <= a);
        i > 0 && a < funcs[i - 1].end
    };
    let mut new_entries: BTreeSet<u32> = BTreeSet::new();
    let mut text_ptrs: BTreeSet<u32> = BTreeSet::new();
    let mut a = 0x0043_ce00u32;
    while a + 4 <= 0x004a_5bbc {
        if let Some(v) = img.word(a) {
            if v & 3 == 0 && (0x0010_0000..text_end).contains(&v) {
                text_ptrs.insert(v);
                if !covered(&funcs, v) {
                    new_entries.insert(v);
                }
            }
        }
        a += 4;
    }
    // Address constants built in code: `lui rt, hi` followed (within a few
    // instructions) by `addiu/ori rx, rt, lo` that lands in .text.
    for f in funcs.clone() {
        let mut lui: [Option<u32>; 32] = [None; 32];
        for &(lo, hi) in &f.runs {
            let mut a = lo;
            while a < hi {
                let w = img.word(a).unwrap_or(0);
                let op = w >> 26;
                let rs = ((w >> 21) & 31) as usize;
                let rt = ((w >> 16) & 31) as usize;
                if op == 0x0F {
                    lui[rt] = Some((w & 0xFFFF) << 16);
                } else if (op == 0x09 || op == 0x0D) && lui[rs].is_some() {
                    let base = lui[rs].unwrap();
                    let v = if op == 0x09 { base.wrapping_add(w as i16 as i32 as u32) } else { base | (w & 0xFFFF) };
                    if v & 3 == 0 && (0x0010_0000..text_end).contains(&v) && !covered(&funcs, v) {
                        new_entries.insert(v);
                    }
                }
                a += 4;
            }
        }
    }
    let mut extra = BTreeSet::new();
    if let Ok(s) = fs::read_to_string(root.join("generated/entries.txt")) {
        for line in s.lines() {
            let t = line.split('#').next().unwrap().trim();
            if let Ok(v) = u32::from_str_radix(t.trim_start_matches("0x"), 16) {
                extra.insert(v);
                if !covered(&funcs, v) {
                    new_entries.insert(v);
                }
            }
        }
    }
    let discovered = new_entries.len();
    for e in new_entries {
        if !covered(&funcs, e) {
            funcs.push(Func { start: e, end: e, runs: Vec::new(), name: None });
        }
    }
    funcs.sort_by_key(|f| f.start);

    // Recover code Ghidra did not disassemble (after "noreturn" calls etc.).
    let starts: Vec<u32> = funcs.iter().map(|f| f.start).collect();
    let mut recovered = 0;
    for (i, f) in funcs.iter_mut().enumerate() {
        let limit = starts.get(i + 1).copied().unwrap_or(text_end);
        recovered += extend_runs(&img, f, limit, &text_ptrs);
    }
    funcs.retain(|f| !f.runs.is_empty());
    let all_insns: HashSet<u32> = funcs
        .iter()
        .flat_map(|f| f.runs.iter().flat_map(|&(lo, hi)| (lo..hi).step_by(4)))
        .collect();

    // Data → code references (jump tables, function pointers, resume points).
    let mut data_refs = BTreeSet::new();
    for &(lo, hi) in &[(0x0043_ce00u32, 0x004a_5bbcu32)] {
        let mut a = lo;
        while a + 4 <= hi {
            if let Some(v) = img.word(a) {
                if v & 3 == 0 && all_insns.contains(&v) {
                    data_refs.insert(v);
                }
            }
            a += 4;
        }
    }

    // Shard by cumulative instruction count.
    let total: usize = funcs.iter().map(|f| f.runs.iter().map(|r| ((r.1 - r.0) / 4) as usize).sum::<usize>()).sum();
    let per = total.div_ceil(shards);
    let mut shard_of = BTreeMap::new();
    let mut acc = 0usize;
    for f in &funcs {
        let s = (acc / per).min(shards - 1);
        shard_of.insert(f.start, s);
        acc += f.runs.iter().map(|r| ((r.1 - r.0) / 4) as usize).sum::<usize>();
    }

    // Cross-function branch targets must be labels where they land.
    let mut external: BTreeSet<u32> = BTreeSet::new();
    for f in &funcs {
        let ts = targets(&img, f, threads);
        for &t in &ts {
            if t < f.start || t >= f.end {
                external.insert(t);
            }
        }
        extra.extend(ts);
    }
    external.extend(data_refs.iter().copied());
    if let Ok(s) = fs::read_to_string(root.join("generated/entries.txt")) {
        for line in s.lines() {
            let t = line.split('#').next().unwrap().trim();
            if let Ok(v) = u32::from_str_radix(t.trim_start_matches("0x"), 16) {
                external.insert(v);
            }
        }
    }
    let mut structured_count = 0usize;
    let ctx = Ctx { shard_of: &shard_of, data_refs: &data_refs, extra_labels: &extra, threads };
    let out_root = root.join("generated/ee");
    let _ = fs::remove_dir_all(&out_root);
    let mut totals = Stats::default();
    let mut bytes = 0usize;
    for s in 0..shards {
        let dir = out_root.join(format!("s{s:02}"));
        fs::create_dir_all(dir.join("src")).unwrap();
        fs::write(
            dir.join("Cargo.toml"),
            format!(
                "[package]\nname = \"ssx3-ee-s{s:02}\"\nversion = \"0.1.0\"\nedition = \"2021\"\npublish = false\n\n[lib]\npath = \"src/lib.rs\"\n\n[dependencies]\nssx3-ps2 = {{ path = \"../../../crates/ssx3-ps2\" }}\n"
            ),
        )
        .unwrap();
        let mut code = String::new();
        code.push_str("//! Recompiled SSX 3 EE code. Auto-generated by `recomp-ee`; do not edit.\n");
        code.push_str("#![allow(unused_imports, unused_mut, unused_variables, unused_assignments, unreachable_code, unused_parens, clippy::all)]\n");
        code.push_str("use ssx3_ps2::Cpu;\nuse ssx3_ps2::cpu::sx;\nuse ssx3_ps2::vu::{Src, W, X, Y, Z};\nuse ssx3_ps2::vu::decode::Efu;\n\n");
        for f in funcs.iter().filter(|f| shard_of[&f.start] == s) {
            if structured {
                if let Some(src) = ssx3_recomp::structure::emit_structured(&img, f, s, &ctx, &external) {
                    code.push_str(&src);
                    structured_count += 1;
                    if external.range(f.start + 1..f.end).next().is_some() {
                        let twin = format!("{}__entries", fn_name(f.start));
                        let st = emit_function_named(&img, f, s, &ctx, &mut code, &twin);
                        totals.invalid += st.invalid;
                    }
                    totals.instructions += f.runs.iter().map(|r| ((r.1 - r.0) / 4) as usize).sum::<usize>();
                    code.push('\n');
                    continue;
                }
            }
            let st = emit_function(&img, f, s, &ctx, &mut code);
            totals.instructions += st.instructions;
            totals.invalid += st.invalid;
            totals.labels += st.labels;
            code.push('\n');
        }
        // Readable aliases for recovered names.
        let mut seen = BTreeSet::new();
        for f in funcs.iter().filter(|f| shard_of[&f.start] == s) {
            if let Some(n) = &f.name {
                let ok = n.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
                    && n.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                    && !n.starts_with("f_");
                if ok && seen.insert(n.clone()) {
                    let _ = writeln!(code, "pub use self::{} as {n};", fn_name(f.start));
                }
            }
        }
        bytes += code.len();
        fs::write(dir.join("src/lib.rs"), code).unwrap();
    }

    // Game crate: dependencies + sorted function table.
    let game = out_root.join("game");
    fs::create_dir_all(game.join("src")).unwrap();
    let mut toml = String::from(
        "[package]\nname = \"ssx3-ee\"\nversion = \"0.1.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\nssx3-ps2 = { path = \"../../../crates/ssx3-ps2\" }\n",
    );
    for s in 0..shards {
        let _ = writeln!(toml, "ssx3-ee-s{s:02} = {{ path = \"../s{s:02}\" }}");
    }
    fs::write(game.join("Cargo.toml"), toml).unwrap();
    let mut lib = String::from(
        "//! SSX 3 EE function table. Auto-generated by `recomp-ee`; do not edit.\nuse ssx3_ps2::machine::FnEntry;\n\n",
    );
    let _ = writeln!(lib, "pub static FUNCTIONS: [FnEntry; {}] = [", funcs.len());
    for f in &funcs {
        let _ = writeln!(
            lib,
            "    FnEntry {{ start: {:#010x}, end: {:#010x}, f: ssx3_ee_s{:02}::{} }},",
            f.start,
            f.end,
            shard_of[&f.start],
            fn_name(f.start)
        );
    }
    lib.push_str("];\n");
    let _ = writeln!(lib, "\n/// True when generated for host-thread mode (`ssx3_ps2::threads`).\npub const HOST_THREADS: bool = {threads};");
    fs::write(game.join("src/lib.rs"), lib).unwrap();

    println!("functions    : {}", funcs.len());
    println!("instructions : {}", totals.instructions);
    println!("invalid      : {}", totals.invalid);
    println!("labels       : {}", totals.labels);
    println!("discovered   : {discovered} new function entries");
    println!("recovered    : {recovered} instructions beyond Ghidra's runs");
    println!("data refs    : {}", data_refs.len());
    println!("extra entries: {}", extra.len());
    println!("shards       : {shards}");
    if structured {
        println!("structured   : {structured_count} of {} functions", funcs.len());
    }
    println!("rust bytes   : {bytes}");

    // The IOP sound modules (recomp-iop) are part of the same build.
    let iso = std::env::var("SSX3_ISO").map(PathBuf::from).unwrap_or_else(|_| root.join("SSX 3 (USA).iso"));
    match ssx3_recomp::iopgen::generate_from_disc(&iso, &root.join("generated/iop/modules")) {
        Ok(o) => println!("iop          : {} functions, {} instructions -> generated/iop/modules", o.functions, o.instructions),
        Err(e) => eprintln!("iop          : not generated ({e})"),
    }
}
