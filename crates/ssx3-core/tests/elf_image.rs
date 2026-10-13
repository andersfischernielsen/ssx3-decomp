//! Validates the machine model against the real `SLUS_207.72` image.
//!
//! The image is extracted from the disc by `tools/extract-from-iso.sh` into
//! `orig/`. When it is absent (e.g. a fresh clone) the test skips rather than
//! failing, so it stays useful locally without committing the copyrighted data.

use std::fs;
use std::path::Path;

use ssx3_core::memory_map;
use ssx3_core::Memory;

const ELF_PATH: &str = "../../orig/SLUS_207.72";

fn rd_u16(b: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([b[off], b[off + 1]])
}
fn rd_u32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

#[test]
fn real_elf_header_matches_known_facts() {
    let path = Path::new(ENV_DIR).join(ELF_PATH);
    let Ok(bytes) = fs::read(&path) else {
        eprintln!("skipping: {} not found (run tools/extract-from-iso.sh)", path.display());
        return;
    };

    // ELF identification.
    assert_eq!(&bytes[0..4], b"\x7fELF");
    assert_eq!(bytes[4], 1, "ELFCLASS32");
    assert_eq!(bytes[5], 1, "ELFDATA2LSB");
    assert_eq!(rd_u16(&bytes, 18), 8, "EM_MIPS");

    // The values we recorded from Ghidra.
    assert_eq!(rd_u32(&bytes, 24), memory_map::ENTRY_POINT);
    assert!(
        rd_u32(&bytes, 24) >= memory_map::IMAGE_BASE,
        "entry inside image"
    );
}

/// A tiny ELF32 program-header walker, enough to drop PT_LOAD segments into
/// [`Memory`]. This is the seed of the loader the runtime will need.
fn load_pt_load(bytes: &[u8], mem: &mut Memory) -> usize {
    let phoff = rd_u32(bytes, 28) as usize;
    let phentsize = rd_u16(bytes, 42) as usize;
    let phnum = rd_u16(bytes, 44) as usize;

    let mut loaded = 0;
    for i in 0..phnum {
        let ph = phoff + i * phentsize;
        let p_type = rd_u32(bytes, ph);
        if p_type != 1 {
            continue; // PT_LOAD
        }
        let p_offset = rd_u32(bytes, ph + 4) as usize;
        let p_vaddr = rd_u32(bytes, ph + 8);
        let p_filesz = rd_u32(bytes, ph + 16) as usize;
        mem.load(p_vaddr, &bytes[p_offset..p_offset + p_filesz]);
        loaded += 1;
    }
    loaded
}

#[test]
fn real_elf_loads_and_entry_bytes_match() {
    let path = Path::new(ENV_DIR).join(ELF_PATH);
    let Ok(bytes) = fs::read(&path) else {
        eprintln!("skipping: {} not found", path.display());
        return;
    };

    let mut mem = Memory::new();
    let n = load_pt_load(&bytes, &mut mem);
    assert!(n >= 1, "expected at least one PT_LOAD segment");

    // The bytes at the entry address in memory must equal the bytes at the
    // corresponding file offset. Find the segment covering the entry.
    let phoff = rd_u32(&bytes, 28) as usize;
    let phentsize = rd_u16(&bytes, 42) as usize;
    let phnum = rd_u16(&bytes, 44) as usize;
    for i in 0..phnum {
        let ph = phoff + i * phentsize;
        if rd_u32(&bytes, ph) != 1 {
            continue;
        }
        let p_offset = rd_u32(&bytes, ph + 4) as usize;
        let p_vaddr = rd_u32(&bytes, ph + 8);
        let p_filesz = rd_u32(&bytes, ph + 16) as usize;
        let end = p_vaddr + p_filesz as u32;
        if (p_vaddr..end).contains(&memory_map::ENTRY_POINT) {
            let file_off = p_offset + (memory_map::ENTRY_POINT - p_vaddr) as usize;
            let expect = rd_u32(&bytes, file_off);
            assert_eq!(mem.read_u32(memory_map::ENTRY_POINT), expect);
            return;
        }
    }
    panic!("no PT_LOAD segment covers the entry point");
}

/// `CARGO_MANIFEST_DIR` is `crates/ssx3-core`; tests run from the crate root.
const ENV_DIR: &str = env!("CARGO_MANIFEST_DIR");
