//! Static VU microcode recompiler driver.
//!
//! Usage: `vu_recomp [ELF] [OUT_DIR]` (defaults: `orig/SLUS_207.72`,
//! `generated/vu`). Finds the `.vutext` section, extracts every MPG upload
//! (consecutive MPGs with contiguous load addresses form one segment, the
//! same rule `Vu::write_code` uses at runtime), translates each segment with
//! `ssx3_ps2::vu::recomp`, and writes `OUT_DIR/vu_<hash>.rs` plus
//! `OUT_DIR/mod.rs` (the registry keyed by segment hash).

use ssx3_ps2::vu::recomp::{extract_mpg_segments, registry_source, translate};
use std::path::PathBuf;

fn section<'a>(elf: &'a [u8], want: &str) -> Option<(u32, &'a [u8])> {
    let rd16 = |o: usize| u16::from_le_bytes([elf[o], elf[o + 1]]) as usize;
    let rd32 = |o: usize| u32::from_le_bytes(elf[o..o + 4].try_into().unwrap()) as usize;
    let shoff = rd32(0x20);
    let shnum = rd16(0x30);
    let shstrndx = rd16(0x32);
    let sh = |i: usize| shoff + i * 40;
    let strtab = rd32(sh(shstrndx) + 16);
    for i in 0..shnum {
        let name_off = strtab + rd32(sh(i));
        let end = elf[name_off..].iter().position(|&b| b == 0)? + name_off;
        if &elf[name_off..end] == want.as_bytes() {
            let addr = rd32(sh(i) + 12) as u32;
            let off = rd32(sh(i) + 16);
            let size = rd32(sh(i) + 20);
            return Some((addr, &elf[off..off + size]));
        }
    }
    None
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let elf_path = PathBuf::from(
        args.get(1)
            .map(String::as_str)
            .unwrap_or("orig/SLUS_207.72"),
    );
    let out = PathBuf::from(args.get(2).map(String::as_str).unwrap_or("generated/vu"));
    let elf =
        std::fs::read(&elf_path).unwrap_or_else(|e| panic!("read {}: {e}", elf_path.display()));
    let (vaddr, vutext) = section(&elf, ".vutext").expect(".vutext section");
    let words: Vec<u32> = vutext
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
        .collect();
    let segs = extract_mpg_segments(&words);
    std::fs::create_dir_all(&out).expect("create output dir");
    // Remove stale translations.
    if let Ok(rd) = std::fs::read_dir(&out) {
        for e in rd.flatten() {
            let n = e.file_name().to_string_lossy().to_string();
            if n.starts_with("vu_") && n.ends_with(".rs") {
                let _ = std::fs::remove_file(e.path());
            }
        }
    }
    let mut ts = Vec::new();
    let mut total_pairs = 0;
    let mut total_unknown = 0;
    for s in &segs {
        let t = translate(&s.bytes, s.addr);
        let pairs = s.bytes.len() / 8;
        total_pairs += pairs;
        total_unknown += t.unknown.len();
        println!(
            "segment from {:#010x}: load {:#06x} len {:#06x} ({} pairs) hash {:#018x}: {} leaders, {} unknown, {} unsupported",
            vaddr + s.source_word as u32 * 4,
            s.addr,
            s.bytes.len(),
            pairs,
            t.hash,
            t.leaders.len(),
            t.unknown.len(),
            t.unsupported.len()
        );
        for (pc, w) in t.unknown.iter().take(16) {
            println!("    unknown {w:#010x} at {pc:#06x}");
        }
        std::fs::write(out.join(format!("{}.rs", t.module)), &t.source).expect("write translation");
        ts.push(t);
    }
    std::fs::write(out.join("mod.rs"), registry_source(&ts)).expect("write mod.rs");
    println!(
        "{} segments, {} pairs, {} unknown words",
        ts.len(),
        total_pairs,
        total_unknown
    );
}
