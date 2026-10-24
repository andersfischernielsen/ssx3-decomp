//! Lowering and readability checks for the EE generator.

use ssx3_recomp::eegen::{is_lowerable, lower, prettify};

fn word(op: u32, rs: u32, rt: u32, imm: u16) -> u32 {
    (op << 26) | (rs << 21) | (rt << 16) | imm as u32
}

#[test]
fn li_is_a_constant() {
    // addiu v0, zero, -1
    let s = lower(0, word(0x09, 0, 2, 0xffff)).unwrap();
    assert_eq!(s, "c.g[2][0] = 0xffffffffffffffff;");
}

#[test]
fn addiu_sign_extends_32_bit_result() {
    // addiu sp, sp, -0x50
    let s = lower(0, word(0x09, 29, 29, (-0x50i16) as u16)).unwrap();
    assert_eq!(
        s,
        "c.g[29][0] = sx((c.g[29][0] as u32).wrapping_add(0xffffffb0));"
    );
    assert_eq!(
        prettify(&s),
        "c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));"
    );
}

#[test]
fn move_is_a_copy() {
    // daddu a0, sp, zero
    let w = (29 << 21) | (0 << 16) | (4 << 11) | 0x2d;
    assert_eq!(lower(0, w).unwrap(), "c.g[4][0] = c.g[29][0];");
    // or a1, zero, s0
    let w = (0 << 21) | (16 << 16) | (5 << 11) | 0x25;
    assert_eq!(lower(0, w).unwrap(), "c.g[5][0] = c.g[16][0];");
}

#[test]
fn writes_to_zero_are_dropped() {
    // sll zero, zero, 0 (nop)
    assert_eq!(lower(0, 0).unwrap(), "");
}

#[test]
fn every_r5900_major_opcode_class_is_lowerable() {
    // A few representative encodings across classes (MMI, COP1, COP2, loads/stores).
    for w in [
        0x7000_0488u32, // MMI0 paddw-ish class
        0x4600_0000,    // add.s
        0x4a00_02ff,    // vnop
        0x7c00_0000,    // sq
        0x7800_0000,    // lq
        0xd800_0000,    // lqc2
    ] {
        assert!(is_lowerable(0, w), "{w:#010x}");
    }
}
