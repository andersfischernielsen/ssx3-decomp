//! Checks of the IOP runtime that need no generated code.

use ssx3_iop::{imports, irx, spu2, Iop};

#[test]
fn memory_and_unaligned_access() {
    let mut c = Iop::new();
    c.sw(0x8000_1000, 0x1122_3344);
    assert_eq!(c.lw(0xA000_1000), 0x1122_3344, "KSEG mirrors");
    assert_eq!(c.lbu(0x1001), 0x33);
    assert_eq!(c.lh(0x1002) as i32, 0x1122);
    // lwr/lwl pair loading the unaligned word at 0x1001 (bytes 33 22 11 00).
    c.r[5] = 0;
    c.lwr(5, 0x1001);
    c.lwl(5, 0x1004);
    assert_eq!(c.r[5], 0x0011_2233);
}

#[test]
fn division_by_zero_matches_r3000() {
    let mut c = Iop::new();
    c.r[1] = 7;
    c.r[2] = 0;
    c.div(1, 2);
    assert_eq!((c.lo, c.hi), (0xFFFF_FFFF, 7));
    c.r[1] = (-7i32) as u32;
    c.div(1, 2);
    assert_eq!((c.lo, c.hi), (1, (-7i32) as u32));
}

#[test]
fn printf_formatting() {
    let mut c = Iop::new();
    c.write_bytes(0x2000, b"v%d %04x %s\0");
    c.write_bytes(0x2100, b"ok\0");
    c.r[4] = 0x2000;
    c.r[5] = (-3i32) as u32;
    c.r[6] = 0x2a;
    c.r[7] = 0x2100;
    assert_eq!(imports::format(&c, 0x2000, 1), "v-3 002a ok");
}

#[test]
fn allocator_bottom_and_top() {
    let mut c = Iop::new();
    let a = c.k.alloc(0x1000, 0, 0);
    let b = c.k.alloc(0x1000, 1, 0);
    assert_eq!(a, irx::layout::HEAP_START);
    assert_eq!(b, irx::layout::HEAP_END - 0x1000);
    c.k.free(a);
    assert_eq!(c.k.alloc(0x100, 0, 0), a);
}

/// A voice playing a looping one-block ADPCM sample produces output once keyed on.
#[test]
fn spu2_voice_plays_adpcm() {
    let mut c = Iop::new();
    let base = 0x5000usize;
    c.spu.ram[base] = 0x0700; // shift 0, filter 0, flags loop-start | loop | end
    for i in 1..8 {
        c.spu.ram[base + i] = 0x7777;
    }
    let reg = |off: u32| 0x1F90_0000 + off;
    spu2::write_reg(&mut c, reg(0x1C0), 0); // SSA hi, voice 0
    spu2::write_reg(&mut c, reg(0x1C2), base as u32); // SSA lo
    spu2::write_reg(&mut c, reg(0x000), 0x3FFF); // VOLL
    spu2::write_reg(&mut c, reg(0x002), 0x3FFF); // VOLR
    spu2::write_reg(&mut c, reg(0x004), 0x1000); // pitch: 48 kHz
    spu2::write_reg(&mut c, reg(0x006), 0x000F); // ADSR1: fastest attack
    spu2::write_reg(&mut c, reg(0x008), 0x0000);
    spu2::write_reg(&mut c, reg(0x1A0), 1); // KON voice 0
    for _ in 0..8 {
        spu2::tick(&mut c);
    }
    let v = &c.spu.cores[0].voices[0];
    assert_ne!(v.phase, spu2::Phase::Off);
    assert!(v.env.level > 0x4000, "envelope rises: {}", v.env.level);
}

/// The IRX parser on the real module, when the disc is mounted.
#[test]
fn parse_snddrv_if_present() {
    let Ok(f) = std::fs::read("/Volumes/SSX3/DATA/MODULES/SNDDRV.IRX") else { return };
    let m = irx::load(&f, 0x30000).unwrap();
    assert_eq!(m.entry, 0x39244);
    let libs: Vec<&str> = m.imports.iter().map(|i| i.lib.as_str()).collect();
    assert!(libs.contains(&"libsd") && libs.contains(&"sifcmd"));
    assert!(m.symbols.iter().any(|s| s.name == "threadmain"));
}
