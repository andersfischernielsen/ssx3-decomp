//! Validates the ELF loader against the real `SLUS_207.72` image.
//!
//! The image is extracted from the disc by `tools/extract-from-iso.sh` into
//! `orig/`. When it is absent (e.g. a fresh clone) each test skips rather than
//! failing, so the suite stays useful without committing the copyrighted data.

use std::path::Path;

use ssx3_core::elf::Elf;
use ssx3_core::memory_map;

const ELF_PATH: &str = "../../orig/SLUS_207.72";

fn load() -> Option<Elf> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(ELF_PATH);
    if !path.exists() {
        eprintln!(
            "skipping: {} not found (run tools/extract-from-iso.sh)",
            path.display()
        );
        return None;
    }
    Some(Elf::from_file(&path).expect("SLUS_207.72 should parse"))
}

#[test]
fn header_matches_known_facts() {
    let Some(elf) = load() else { return };
    assert_eq!(elf.entry, memory_map::ENTRY_POINT);
    assert!(!elf.segments.is_empty());
    let (lo, hi) = elf.image_range().unwrap();
    assert!(lo >= memory_map::IMAGE_BASE);
    // .text itself starts exactly at the image base.
    assert_eq!(lo, memory_map::IMAGE_BASE);
    assert!(hi > lo);
}

#[test]
fn text_segment_covers_the_whole_image() {
    let Some(elf) = load() else { return };
    let text = elf
        .segment_for(memory_map::ENTRY_POINT, 4)
        .expect("entry must be inside a PT_LOAD segment");
    assert!(text.flags & 1 != 0, "segment should be executable");
    assert!(text.filesz as usize >= memory_map::SLUS_207_72[0].len() as usize / 2);
}

#[test]
fn entry_word_matches_ghidra_bytes() {
    // Ghidra's disassembly of `entry` reports bytes "280c0070" at 00100008,
    // i.e. the little-endian word 0x70000c28 (`clear at`, MMI mmiop=0x10 fct=0x28).
    let Some(elf) = load() else { return };
    assert_eq!(
        elf.read(memory_map::ENTRY_POINT, 4).unwrap(),
        &[0x28, 0x0c, 0x00, 0x70]
    );
    assert_eq!(elf.read_u32(memory_map::ENTRY_POINT).unwrap(), 0x7000_0c28);
}

#[test]
fn loads_into_memory_and_entry_bytes_round_trip() {
    let Some(elf) = load() else { return };
    let mut mem = ssx3_core::Memory::new();
    elf.load_into(&mut mem);
    assert_eq!(
        mem.read_u32(memory_map::ENTRY_POINT),
        elf.read_u32(memory_map::ENTRY_POINT).unwrap()
    );
}

#[test]
fn every_function_address_is_readable() {
    let Some(elf) = load() else { return };
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../generated/functions.tsv");
    let Ok(tsv) = std::fs::read_to_string(&path) else {
        eprintln!("skipping: inventory not found");
        return;
    };
    let mut checked = 0;
    for line in tsv.lines() {
        let addr = u32::from_str_radix(line.split_once('\t').unwrap().0, 16).unwrap();
        assert!(
            elf.read_u32(addr).is_some(),
            "function at {addr:08x} is not backed by the loaded image"
        );
        checked += 1;
    }
    assert_eq!(checked, 11_844);
}
