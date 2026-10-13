//! Integration tests for the `ssx3-core` machine model.
//!
//! These exercise the public API the way the recompiled game will: raw
//! little-endian accesses, the cached/uncached RAM aliases, and the syscall and
//! memory-map tables.

use ssx3_core::memory_map::{self as map, device, Segment};
use ssx3_core::{syscall, Memory};

#[test]
fn ram_roundtrip_u32() {
    let mut m = Memory::new();
    m.write_u32(0x0010_0000, 0xdead_beef);
    assert_eq!(m.read_u32(0x0010_0000), 0xdead_beef);
}

#[test]
fn uncached_alias_shares_backing_ram() {
    let mut m = Memory::new();
    m.write_u32(0x0010_0000, 0x1122_3344);
    // 0x2000_0000 is the uncached alias of the same physical RAM.
    assert_eq!(m.read_u32(0x2010_0000), 0x1122_3344);
    // ...and writes through the alias are visible at the cached address.
    m.write_u32(0x2010_0000, 0xaabb_ccdd);
    assert_eq!(m.read_u32(0x0010_0000), 0xaabb_ccdd);
}

#[test]
fn accelerated_alias_shares_backing_ram() {
    let mut m = Memory::new();
    m.write_u32(0x0010_0000, 0x0bad_f00d);
    assert_eq!(m.read_u32(0x3010_0000), 0x0bad_f00d);
}

#[test]
fn little_endian_byte_order() {
    let mut m = Memory::new();
    m.write_u32(0x0010_0000, 0x1122_3344);
    assert_eq!(m.read_u8(0x0010_0000), 0x44);
    assert_eq!(m.read_u8(0x0010_0001), 0x33);
    assert_eq!(m.read_u8(0x0010_0002), 0x22);
    assert_eq!(m.read_u8(0x0010_0003), 0x11);
}

#[test]
fn unaligned_u16_roundtrips() {
    let mut m = Memory::new();
    m.write_u16(0x0010_0001, 0xabcd);
    assert_eq!(m.read_u16(0x0010_0001), 0xabcd);
    // Neighbouring bytes were not disturbed beyond the two written.
    assert_eq!(m.read_u8(0x0010_0000), 0x00);
    assert_eq!(m.read_u8(0x0010_0003), 0x00);
}

#[test]
fn scratchpad_roundtrip() {
    let mut m = Memory::new();
    let a = device::SCRATCHPAD_BASE + 0x40;
    m.write_u32(a, 0x1234_5678);
    assert_eq!(m.read_u32(a), 0x1234_5678);
    assert_eq!(m.last_device_addr, None);
}

#[test]
fn iop_ram_mirrors_every_2mib() {
    let mut m = Memory::new();
    let a = device::IOP_RAM_BASE + 0x100;
    m.write_u32(a, 0x0f0f_0f0f);
    // The window is 32 MiB but only 2 MiB is real, so it repeats.
    assert_eq!(m.read_u32(a + device::IOP_RAM_SIZE as u32), 0x0f0f_0f0f);
}

#[test]
fn load_copies_block() {
    let mut m = Memory::new();
    let bytes = [1u8, 2, 3, 4, 5, 6, 7, 8];
    m.load(0x0010_0100, &bytes);
    for (i, b) in bytes.iter().enumerate() {
        assert_eq!(m.read_u8(0x0010_0100 + i as u32), *b);
    }
}

#[test]
fn device_access_is_counted_not_fatal() {
    let mut m = Memory::new();
    m.write_u32(device::GS_REGS_BASE, 0xffff_ffff);
    assert_eq!(m.unmapped_writes, 1);
    assert_eq!(m.last_device_addr, Some(device::GS_REGS_BASE));

    let v = m.read_u32(map::device::REGISTERS_IO_BASE);
    assert_eq!(v, 0);
    assert_eq!(m.unmapped_reads, 1);
}

#[test]
fn syscall_names_are_known() {
    assert_eq!(syscall::name(syscall::nr::INIT_MAIN_THREAD), Some("InitMainThread"));
    assert_eq!(syscall::name(syscall::nr::INIT_HEAP), Some("InitHeap"));
    assert_eq!(syscall::name(syscall::nr::EXIT), Some("Exit"));
}

#[test]
fn entry_point_syscalls_are_the_crt0_prologue() {
    // `entry` calls syscall 0x3c then 0x3d, i.e. InitMainThread / InitHeap.
    assert_eq!(syscall::name(0x3c), Some("InitMainThread"));
    assert_eq!(syscall::name(0x3d), Some("InitHeap"));
}

#[test]
fn unknown_syscall_has_no_name() {
    assert_eq!(syscall::name(0x9999), None);
}

#[test]
fn segment_len_and_contains() {
    let text = map::SLUS_207_72[0];
    assert_eq!(text.name, ".text");
    assert_eq!(text.start, map::IMAGE_BASE);
    assert_eq!(text.len(), text.end - text.start + 1);
    assert!(text.contains(map::ENTRY_POINT));
    assert!(!text.contains(map::IMAGE_BASE - 1));
}

#[test]
fn entry_point_lies_inside_the_text_segment() {
    let containing = map::SLUS_207_72
        .iter()
        .find(|s: &&Segment| s.contains(map::ENTRY_POINT));
    assert_eq!(containing.map(|s| s.name), Some(".text"));
}

#[test]
fn vu1_microcode_segment_is_present() {
    // SSX 3 ships VU1 microcode; the linker section is `.vutext`.
    let vu = map::SLUS_207_72.iter().find(|s| s.name == ".vutext");
    assert!(vu.is_some());
    assert!(vu.unwrap().perm.x);
}
