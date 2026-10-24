//! Compares the decoder against Ghidra's disassembly (the golden corpus in
//! `generated/ghidra_disasm/`).
//!
//! Ghidra renders pseudo-instructions (`clear hi`, `nop`, `move`, `li`) that we
//! deliberately do not reproduce, so we validate at the level that matters for
//! correctness: the **instruction stream**. For every function in the corpus we
//! require that
//!   * we decode exactly as many instructions as Ghidra lists, and
//!   * each instruction word we read from the ELF equals Ghidra's bytes.

use std::path::{Path, PathBuf};

use ssx3_recomp::index;
use ssx3_recomp::program::Program;

fn corpus_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../generated/ghidra_disasm")
}

fn open_program() -> Option<Program> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../orig/SLUS_207.72");
    if !path.exists() {
        eprintln!("skipping: {} not found", path.display());
        return None;
    }
    Some(Program::open(&path).expect("open SLUS_207.72"))
}

/// Parse `generated/ghidra_disasm/<hex>.txt` into `(address, word)` pairs.
fn parse_golden(path: &Path) -> Vec<(u32, u32)> {
    let text = std::fs::read_to_string(path).unwrap();
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let mut it = line.split_whitespace();
            let addr = u32::from_str_radix(it.next().unwrap(), 16).unwrap();
            let bytes = it.next().unwrap();
            let b: Vec<u8> = (0..4)
                .map(|i| u8::from_str_radix(&bytes[i * 2..i * 2 + 2], 16).unwrap())
                .collect();
            (addr, u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        })
        .collect()
}

#[test]
fn decoder_matches_ghidra_for_every_corpus_function() {
    let Some(program) = open_program() else {
        return;
    };
    let dir = corpus_dir();
    let Ok(entries) = std::fs::read_dir(&dir) else {
        eprintln!("skipping: no golden corpus at {}", dir.display());
        return;
    };

    let mut files = 0;
    let mut instructions = 0;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("txt") {
            continue;
        }
        let start = u32::from_str_radix(path.file_stem().unwrap().to_str().unwrap(), 16).unwrap();
        let golden = parse_golden(&path);
        let f = index()
            .by_address(start)
            .unwrap_or_else(|| panic!("{} is not a function entry", path.display()));

        // With authoritative boundaries the instruction address set must match
        // Ghidra's exactly, and every word must agree.
        let decoded = program.decode_with_addresses(f);
        let got: std::collections::BTreeMap<u32, u32> =
            decoded.iter().map(|(a, i)| (*a, i.word)).collect();
        let want: std::collections::BTreeSet<u32> = golden.iter().map(|(a, _)| *a).collect();
        let got_addrs: std::collections::BTreeSet<u32> = got.keys().copied().collect();
        assert_eq!(
            got_addrs, want,
            "instruction addresses differ from Ghidra for {} at {start:08x}",
            f.name
        );
        for (addr, word) in &golden {
            assert_eq!(
                got.get(addr),
                Some(word),
                "word mismatch in {} at {addr:08x}",
                f.name
            );
            assert_eq!(
                program.elf.read_u32(*addr).unwrap(),
                *word,
                "ELF word mismatch in {} at {addr:08x}",
                f.name
            );
        }

        files += 1;
        instructions += golden.len();
    }

    assert_eq!(files, 7, "expected the full 7-file corpus");
    assert_eq!(instructions, 600, "corpus instruction total changed");
}

#[test]
fn every_corpus_file_is_a_real_function_entry() {
    let dir = corpus_dir();
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("txt") {
            continue;
        }
        let start = u32::from_str_radix(path.file_stem().unwrap().to_str().unwrap(), 16).unwrap();
        assert!(
            index().by_address(start).is_some(),
            "{} does not correspond to a function",
            path.display()
        );
    }
}
