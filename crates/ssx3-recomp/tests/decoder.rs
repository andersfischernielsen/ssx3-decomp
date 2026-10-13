//! Encoding-level tests for the R5900 decoder.
//!
//! Every expected word here is hand-assembled from the R5900/MIPS III encoding,
//! so these pin the decoder to the ISA rather than to Ghidra's rendering.

use ssx3_recomp::r5900::{decode, Operands};

fn op(w: u32) -> Operands {
    decode(w).operands
}
fn mn(w: u32) -> &'static str {
    decode(w).mnemonic
}

#[test]
fn nop_is_sll_zero_zero_zero() {
    // Ghidra prints this as `nop`; canonically it is `sll $0,$0,0`.
    assert_eq!(mn(0x0000_0000), "sll");
    assert_eq!(op(0x0000_0000), Operands::RdRtSa { rd: 0, rt: 0, sa: 0 });
}

#[test]
fn immediate_arith() {
    assert_eq!(mn(0x2402_1234), "addiu");
    assert_eq!(op(0x2402_1234), Operands::RtRsImm { rt: 2, rs: 0, imm: 0x1234 });
    assert_eq!(mn(0x3408_1234), "ori");
    assert_eq!(op(0x3408_1234), Operands::RtRsImmU { rt: 8, rs: 0, imm: 0x1234 });
    assert_eq!(mn(0x3C08_8000), "lui");
    assert_eq!(op(0x3C08_8000), Operands::RtImm { rt: 8, imm: 0x8000 });
}

#[test]
fn negative_immediate_is_sign_extended() {
    assert_eq!(op(0x2402_FFFC), Operands::RtRsImm { rt: 2, rs: 0, imm: -4 });
}

#[test]
fn control_transfer() {
    assert_eq!(mn(0x03E0_0008), "jr");
    assert_eq!(op(0x03E0_0008), Operands::Rs { rs: 31 });
    assert_eq!(mn(0x0C04_0000), "jal");
    assert_eq!(op(0x0C04_0000), Operands::Jump { target: 0x40000 });
}

#[test]
fn syscall_and_sync() {
    assert_eq!(mn(0x0000_000C), "syscall");
    assert_eq!(op(0x0000_000C), Operands::Code { code: 0 });
    assert_eq!(mn(0x0000_000F), "sync");
    assert_eq!(op(0x0000_000F), Operands::None);
}

#[test]
fn hi_lo_moves() {
    assert_eq!(mn(0x0000_0011), "mthi");
    assert_eq!(op(0x0000_0011), Operands::Rs { rs: 0 });
    assert_eq!(mn(0x0000_0013), "mtlo");
    assert_eq!(mn(0x0000_0012), "mflo");
    assert_eq!(op(0x0000_0012), Operands::Rd { rd: 0 });
}

#[test]
fn ee_hi_lo_moves_via_mmi() {
    // 0x7000_0011: opcode 0x1C (MMI), mmiop=0, fct=0x11 -> mthi1.
    assert_eq!(mn(0x7000_0011), "mthi1");
    assert_eq!(mn(0x7000_0013), "mtlo1");
    assert_eq!(
        op(0x7000_0011),
        Operands::Mmi { mmiop: 0, fct: 0x11, rd: 0, rs: 0, rt: 0 }
    );
}

#[test]
fn clear_is_the_mmi_pseudo_from_crt0() {
    // `clear at`: mmiop=0x10, fct=0x28, rd=1. This is the first instruction of
    // SLUS_207.72's entry.
    assert_eq!(mn(0x7000_0C28), "clear");
    assert_eq!(
        op(0x7000_0C28),
        Operands::Mmi { mmiop: 0x10, fct: 0x28, rd: 1, rs: 0, rt: 0 }
    );
}

#[test]
fn unknown_mmi_keeps_its_structure() {
    // An MMI encoding we do not name must still decode, not become .word.
    let inst = decode(0x7129_4008);
    assert_eq!(inst.mnemonic, "mmi");
    assert!(inst.is_known());
}

#[test]
fn cop0_control_ops() {
    assert_eq!(mn(0x4200_0038), "ei");
    assert_eq!(mn(0x4200_0039), "di");
    assert_eq!(mn(0x4200_0018), "eret");
    assert_eq!(op(0x4200_0018), Operands::None);
}

#[test]
fn cop0_moves() {
    assert_eq!(mn(0x4008_6000), "mfc0");
    assert_eq!(op(0x4008_6000), Operands::CopMove { rt: 8, rd: 12, sel: 0 });
    assert_eq!(mn(0x4088_6000), "mtc0");
}

#[test]
fn loads_and_stores() {
    assert_eq!(mn(0x8FA8_0010), "lw");
    assert_eq!(op(0x8FA8_0010), Operands::Mem { rt: 8, rs: 29, offset: 0x10 });
    assert_eq!(mn(0xAFA8_0010), "sw");
    assert_eq!(mn(0x7888_0000), "lq");
    assert_eq!(mn(0x7C88_0000), "sq");
    assert_eq!(mn(0xD888_0000), "lqc2");
    assert_eq!(op(0xD888_0000), Operands::Mem { rt: 8, rs: 4, offset: 0 });
}

#[test]
fn branches() {
    assert_eq!(mn(0x1100_0010), "beq");
    assert_eq!(op(0x1100_0010), Operands::Branch { rs: 8, rt: 0, offset: 0x10 });
    assert_eq!(mn(0x1520_FFFC), "bne");
    assert_eq!(op(0x1520_FFFC), Operands::Branch { rs: 9, rt: 0, offset: -4 });
    assert_eq!(mn(0x0480_FFF8), "bltz");
    assert_eq!(op(0x0480_FFF8), Operands::Branch1 { rs: 4, offset: -8 });
}

#[test]
fn integer_r_type_and_64bit_ops() {
    assert_eq!(mn(0x0001_102B), "sltu");
    assert_eq!(op(0x0001_102B), Operands::RdRsRt { rd: 2, rs: 0, rt: 1 });
    assert_eq!(mn(0x0064_102D), "daddu");
    assert_eq!(mn(0x0008_413C), "dsll32");
    assert_eq!(op(0x0008_413C), Operands::RdRtSa { rd: 8, rt: 8, sa: 4 });
}

#[test]
fn cop1_moves_and_arith_use_fpu_field_layout() {
    // mfc1 $t0, $f2
    assert_eq!(mn(0x4408_1000), "mfc1");
    assert_eq!(op(0x4408_1000), Operands::CopMove { rt: 8, rd: 2, sel: 0 });
    // add.s $f3, $f2, $f1
    assert_eq!(mn(0x4601_10C0), "add.s");
    assert_eq!(op(0x4601_10C0), Operands::Cop1Arith { fd: 3, fs: 2, ft: 1 });
    // bc1t +0x10
    assert_eq!(mn(0x4501_0010), "bc1t");
    assert_eq!(op(0x4501_0010), Operands::Cop1Branch { offset: 0x10 });
}

#[test]
fn cop2_data_moves() {
    assert_eq!(mn(0x4808_1000), "qmfc2");
    assert_eq!(op(0x4808_1000), Operands::Cop2Move { rt: 8, rd: 2 });
}

#[test]
fn unassigned_encodings_are_preserved_as_word() {
    // opcode 0x13 is unassigned on the R5900.
    let inst = decode(0x4C12_3456);
    assert_eq!(inst.mnemonic, ".word");
    assert!(!inst.is_known());
    assert_eq!(inst.word, 0x4C12_3456);
}

#[test]
fn display_renders_canonical_operands() {
    assert_eq!(decode(0x2402_1234).to_string(), "addiu v0, zero, 4660");
    assert_eq!(decode(0x8FA8_0010).to_string(), "lw t0, 16(sp)");
    assert_eq!(decode(0x3C08_8000).to_string(), "lui t0, 0x8000");
    assert_eq!(decode(0x03E0_0008).to_string(), "jr ra");
}
