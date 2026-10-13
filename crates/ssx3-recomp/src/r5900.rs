//! R5900 (PlayStation 2 Emotion Engine) instruction decoder.
//!
//! The R5900 is a MIPS III core extended with the **MMI** (multimedia) and
//! **COP2/VU** instruction groups. This decodes the encoding into a structured
//! [`Instruction`] and renders canonical MIPS mnemonics.
//!
//! Notes on fidelity:
//! * Mnemonics are canonical MIPS, not Ghidra's pseudo-instructions. Ghidra
//!   prints `mthi $0` as `clear hi` and `sll $0,$0,0` as `nop`; we do not.
//! * Encodings we do not yet name are preserved as [`Operands::Word`] so no
//!   information is lost; the goal is to grow coverage, not to drop bits.

use std::fmt;

/// ABI register names, index 0..31.
pub const REG_NAMES: [&str; 32] = [
    "zero", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3", "t4", "t5", "t6",
    "t7", "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "t8", "t9", "k0", "k1", "gp", "sp",
    "fp", "ra",
];

pub fn reg(n: u8) -> &'static str {
    REG_NAMES[(n & 31) as usize]
}

/// A decoded instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    /// The raw little-endian word, preserved for exact round-tripping.
    pub word: u32,
    pub mnemonic: &'static str,
    pub operands: Operands,
}

impl Instruction {
    pub fn is_known(&self) -> bool {
        !matches!(self.operands, Operands::Word)
    }
}

/// Operand shapes, one per encoding class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operands {
    /// e.g. `nop`, `eret`
    None,
    /// `rd, rs, rt` — add/or/slt/...
    RdRsRt { rd: u8, rs: u8, rt: u8 },
    /// `rd, rt, sa` — sll/srl/dsll32/...
    RdRtSa { rd: u8, rt: u8, sa: u8 },
    /// `rd, rt, rs` — sllv/...
    RdRtRs { rd: u8, rt: u8, rs: u8 },
    /// `rs, rt` — mult/div
    RsRt { rs: u8, rt: u8 },
    /// `rd` — mfhi/mflo
    Rd { rd: u8 },
    /// `rs` — mthi/mtlo/jr
    Rs { rs: u8 },
    /// `rd, rs` — jalr
    RdRs { rd: u8, rs: u8 },
    /// `rt, rs, imm` (signed) — addiu/andi/...
    RtRsImm { rt: u8, rs: u8, imm: i16 },
    /// `rt, rs, imm` (unsigned) — andi/ori/xori
    RtRsImmU { rt: u8, rs: u8, imm: u16 },
    /// `rt, imm` — lui
    RtImm { rt: u8, imm: u16 },
    /// `rt, offset(rs)` — loads/stores
    Mem { rt: u8, rs: u8, offset: i16 },
    /// `rs, rt, offset` — beq/bne
    Branch { rs: u8, rt: u8, offset: i16 },
    /// `rs, offset` — blez/bgtz/bltz/...
    Branch1 { rs: u8, offset: i16 },
    /// `target` — j/jal (absolute, as the raw 26-bit field)
    Jump { target: u32 },
    /// `code`-bearing op — syscall/break
    Code { code: u32 },
    /// `rt, rd, sel` — mfc0/mtc0/mfc1/...
    CopMove { rt: u8, rd: u8, sel: u8 },
    /// `rt, rd` for COP2/VU data moves (qmfc2/qmtc2/cfc2/ctc2)
    Cop2Move { rt: u8, rd: u8 },
    /// COP1 arithmetic: `fd, fs, ft` with a format suffix in the mnemonic.
    Cop1Arith { fd: u8, fs: u8, ft: u8 },
    /// `fs, ft` — COP1 compare
    Cop1Cmp { fs: u8, ft: u8 },
    /// `offset` — COP1 branch
    Cop1Branch { offset: i16 },
    /// MMI (opcode 0x1C): carries `mmiop` (the `sa` field) and `fct`.
    Mmi { mmiop: u8, fct: u8, rd: u8, rs: u8, rt: u8 },
    /// Unrecognised or not-yet-modelled encoding.
    Word,
}

fn fields(w: u32) -> (u8, u8, u8, u8, u8, u16) {
    (
        (w >> 26) as u8 & 0x3F,
        (w >> 21) as u8 & 0x1F,
        (w >> 16) as u8 & 0x1F,
        (w >> 11) as u8 & 0x1F,
        (w >> 6) as u8 & 0x1F,
        w as u16,
    )
}

fn un(w: u32) -> Instruction {
    Instruction {
        word: w,
        mnemonic: ".word",
        operands: Operands::Word,
    }
}

fn mk(w: u32, mnemonic: &'static str, operands: Operands) -> Instruction {
    Instruction {
        word: w,
        mnemonic,
        operands,
    }
}

/// Decode one little-endian instruction word.
pub fn decode(w: u32) -> Instruction {
    let (op, rs, rt, rd, sa, imm) = fields(w);
    let funct = (w & 0x3F) as u8;

    match op {
        0x00 => decode_special(w, rs, rt, rd, sa, funct),
        0x01 => decode_regimm(w, rs, rt, imm),
        0x02 => mk(w, "j", Operands::Jump { target: w & 0x03FF_FFFF }),
        0x03 => mk(w, "jal", Operands::Jump { target: w & 0x03FF_FFFF }),
        0x04 => mk(w, "beq", Operands::Branch { rs, rt, offset: imm as i16 }),
        0x05 => mk(w, "bne", Operands::Branch { rs, rt, offset: imm as i16 }),
        0x06 => mk(w, "blez", Operands::Branch1 { rs, offset: imm as i16 }),
        0x07 => mk(w, "bgtz", Operands::Branch1 { rs, offset: imm as i16 }),
        0x08 => mk(w, "addi", Operands::RtRsImm { rt, rs, imm: imm as i16 }),
        0x09 => mk(w, "addiu", Operands::RtRsImm { rt, rs, imm: imm as i16 }),
        0x0A => mk(w, "slti", Operands::RtRsImm { rt, rs, imm: imm as i16 }),
        0x0B => mk(w, "sltiu", Operands::RtRsImm { rt, rs, imm: imm as i16 }),
        0x0C => mk(w, "andi", Operands::RtRsImmU { rt, rs, imm }),
        0x0D => mk(w, "ori", Operands::RtRsImmU { rt, rs, imm }),
        0x0E => mk(w, "xori", Operands::RtRsImmU { rt, rs, imm }),
        0x0F => mk(w, "lui", Operands::RtImm { rt, imm }),
        0x10 => decode_cop0(w, rs, rt, rd, funct),
        0x11 => decode_cop1(w, rs, rt, rd, funct),
        0x12 => decode_cop2(w, rs, rt, rd, funct),
        0x14 => mk(w, "beql", Operands::Branch { rs, rt, offset: imm as i16 }),
        0x15 => mk(w, "bnel", Operands::Branch { rs, rt, offset: imm as i16 }),
        0x16 => mk(w, "blezl", Operands::Branch1 { rs, offset: imm as i16 }),
        0x17 => mk(w, "bgtzl", Operands::Branch1 { rs, offset: imm as i16 }),
        0x18 => mk(w, "daddi", Operands::RtRsImm { rt, rs, imm: imm as i16 }),
        0x19 => mk(w, "daddiu", Operands::RtRsImm { rt, rs, imm: imm as i16 }),
        0x1A => mk(w, "ldl", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x1B => mk(w, "ldr", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x1C => decode_mmi(w, rs, rt, rd, sa, funct),
        0x1E => mk(w, "lq", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x1F => mk(w, "sq", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x20 => mk(w, "lb", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x21 => mk(w, "lh", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x22 => mk(w, "lwl", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x23 => mk(w, "lw", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x24 => mk(w, "lbu", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x25 => mk(w, "lhu", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x26 => mk(w, "lwr", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x27 => mk(w, "lwu", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x28 => mk(w, "sb", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x29 => mk(w, "sh", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x2A => mk(w, "swl", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x2B => mk(w, "sw", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x2C => mk(w, "sdl", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x2D => mk(w, "sdr", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x2E => mk(w, "swr", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x2F => mk(w, "cache", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x30 => mk(w, "ll", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x31 => mk(w, "lwc1", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x33 => mk(w, "pref", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x35 => mk(w, "ldc1", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x36 => mk(w, "lqc2", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x37 => mk(w, "ld", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x39 => mk(w, "swc1", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x3D => mk(w, "sdc1", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x3E => mk(w, "sqc2", Operands::Mem { rt, rs, offset: imm as i16 }),
        0x3F => mk(w, "sd", Operands::Mem { rt, rs, offset: imm as i16 }),
        _ => un(w),
    }
}

fn decode_special(w: u32, rs: u8, rt: u8, rd: u8, sa: u8, funct: u8) -> Instruction {
    match funct {
        0x00 => mk(w, "sll", Operands::RdRtSa { rd, rt, sa }),
        0x02 => mk(w, "srl", Operands::RdRtSa { rd, rt, sa }),
        0x03 => mk(w, "sra", Operands::RdRtSa { rd, rt, sa }),
        0x04 => mk(w, "sllv", Operands::RdRtRs { rd, rt, rs }),
        0x06 => mk(w, "srlv", Operands::RdRtRs { rd, rt, rs }),
        0x07 => mk(w, "srav", Operands::RdRtRs { rd, rt, rs }),
        0x08 => mk(w, "jr", Operands::Rs { rs }),
        0x09 => mk(w, "jalr", Operands::RdRs { rd, rs }),
        0x0A => mk(w, "movz", Operands::RdRsRt { rd, rs, rt }),
        0x0B => mk(w, "movn", Operands::RdRsRt { rd, rs, rt }),
        0x0C => mk(w, "syscall", Operands::Code { code: w >> 6 }),
        0x0D => mk(w, "break", Operands::Code { code: w >> 6 }),
        0x0F => mk(w, "sync", Operands::None),
        0x10 => mk(w, "mfhi", Operands::Rd { rd }),
        0x11 => mk(w, "mthi", Operands::Rs { rs }),
        0x12 => mk(w, "mflo", Operands::Rd { rd }),
        0x13 => mk(w, "mtlo", Operands::Rs { rs }),
        0x14 => mk(w, "dsllv", Operands::RdRtRs { rd, rt, rs }),
        0x16 => mk(w, "dsrlv", Operands::RdRtRs { rd, rt, rs }),
        0x17 => mk(w, "dsrav", Operands::RdRtRs { rd, rt, rs }),
        0x18 => mk(w, "mult", Operands::RsRt { rs, rt }),
        0x19 => mk(w, "multu", Operands::RsRt { rs, rt }),
        0x1A => mk(w, "div", Operands::RsRt { rs, rt }),
        0x1B => mk(w, "divu", Operands::RsRt { rs, rt }),
        0x20 => mk(w, "add", Operands::RdRsRt { rd, rs, rt }),
        0x21 => mk(w, "addu", Operands::RdRsRt { rd, rs, rt }),
        0x22 => mk(w, "sub", Operands::RdRsRt { rd, rs, rt }),
        0x23 => mk(w, "subu", Operands::RdRsRt { rd, rs, rt }),
        0x24 => mk(w, "and", Operands::RdRsRt { rd, rs, rt }),
        0x25 => mk(w, "or", Operands::RdRsRt { rd, rs, rt }),
        0x26 => mk(w, "xor", Operands::RdRsRt { rd, rs, rt }),
        0x27 => mk(w, "nor", Operands::RdRsRt { rd, rs, rt }),
        0x2A => mk(w, "slt", Operands::RdRsRt { rd, rs, rt }),
        0x2B => mk(w, "sltu", Operands::RdRsRt { rd, rs, rt }),
        0x2C => mk(w, "dadd", Operands::RdRsRt { rd, rs, rt }),
        0x2D => mk(w, "daddu", Operands::RdRsRt { rd, rs, rt }),
        0x2E => mk(w, "dsub", Operands::RdRsRt { rd, rs, rt }),
        0x2F => mk(w, "dsubu", Operands::RdRsRt { rd, rs, rt }),
        0x34 => mk(w, "teq", Operands::RsRt { rs, rt }),
        0x36 => mk(w, "tne", Operands::RsRt { rs, rt }),
        0x38 => mk(w, "dsll", Operands::RdRtSa { rd, rt, sa }),
        0x3A => mk(w, "dsrl", Operands::RdRtSa { rd, rt, sa }),
        0x3B => mk(w, "dsra", Operands::RdRtSa { rd, rt, sa }),
        0x3C => mk(w, "dsll32", Operands::RdRtSa { rd, rt, sa }),
        0x3E => mk(w, "dsrl32", Operands::RdRtSa { rd, rt, sa }),
        0x3F => mk(w, "dsra32", Operands::RdRtSa { rd, rt, sa }),
        _ => un(w),
    }
}

fn decode_regimm(w: u32, rs: u8, rt: u8, imm: u16) -> Instruction {
    let offset = imm as i16;
    let one = Operands::Branch1 { rs, offset };
    match rt {
        0x00 => mk(w, "bltz", one),
        0x01 => mk(w, "bgez", one),
        0x02 => mk(w, "bltzl", one),
        0x03 => mk(w, "bgezl", one),
        0x08 => mk(w, "tgei", Operands::RtRsImm { rt, rs, imm: offset }),
        0x09 => mk(w, "tgeiu", Operands::RtRsImm { rt, rs, imm: offset }),
        0x0A => mk(w, "tlti", Operands::RtRsImm { rt, rs, imm: offset }),
        0x0B => mk(w, "tltiu", Operands::RtRsImm { rt, rs, imm: offset }),
        0x0C => mk(w, "teqi", Operands::RtRsImm { rt, rs, imm: offset }),
        0x0E => mk(w, "tnei", Operands::RtRsImm { rt, rs, imm: offset }),
        0x10 => mk(w, "bltzal", one),
        0x11 => mk(w, "bgezal", one),
        0x12 => mk(w, "bltzall", one),
        0x13 => mk(w, "bgezall", one),
        // `clear fsa`: REGIMM cond=25 clears the SA field (Ghidra: `clear sa`).
        0x19 => mk(w, "clear_sa", Operands::None),
        _ => un(w),
    }
}

fn decode_cop0(w: u32, rs: u8, rt: u8, rd: u8, _funct: u8) -> Instruction {
    let funct = (w & 0x3F) as u8;
    match rs {
        0x00 => mk(w, "mfc0", Operands::CopMove { rt, rd, sel: 0 }),
        0x04 => mk(w, "mtc0", Operands::CopMove { rt, rd, sel: 0 }),
        0x10 => match funct {
            // R5900 control-transfer ops.
            0x01 => mk(w, "tlbr", Operands::None),
            0x02 => mk(w, "tlbwi", Operands::None),
            0x06 => mk(w, "tlbwr", Operands::None),
            0x08 => mk(w, "tlbp", Operands::None),
            0x18 => mk(w, "eret", Operands::None),
            0x38 => mk(w, "ei", Operands::None),
            0x39 => mk(w, "di", Operands::None),
            _ => un(w),
        },
        _ => un(w),
    }
}

const COP1_FUNCTS: &[(u8, &str)] = &[
    (0x00, "add"), (0x01, "sub"), (0x02, "mul"), (0x03, "div"),
    (0x04, "sqrt"), (0x05, "abs"), (0x06, "mov"), (0x07, "neg"),
    (0x08, "round.l"), (0x09, "trunc.l"), (0x0A, "ceil.l"), (0x0B, "floor.l"),
    (0x0C, "round.w"), (0x0D, "trunc.w"), (0x0E, "ceil.w"), (0x0F, "floor.w"),
    (0x18, "adda"), (0x19, "suba"), (0x1A, "mula"), (0x1B, "diva"),
    (0x20, "cvt.s"), (0x21, "cvt.d"), (0x24, "cvt.w"),
    (0x30, "c.f"), (0x31, "c.un"), (0x32, "c.eq"), (0x33, "c.ueq"),
    (0x34, "c.olt"), (0x35, "c.ult"), (0x36, "c.ole"), (0x37, "c.ule"),
    (0x3C, "c.sf"), (0x3D, "c.ngle"), (0x3E, "c.seq"), (0x3F, "c.ngl"),
];

fn decode_cop1(w: u32, rs: u8, rt: u8, _rd: u8, _funct: u8) -> Instruction {
    let funct = (w & 0x3F) as u8;
    // COP1 R-type places `fs` in bits 15..11 and `fd` in bits 10..6 (unlike the
    // integer R-type layout).
    let fs = (w >> 11) as u8 & 0x1F;
    let fd = (w >> 6) as u8 & 0x1F;
    let ft = rt & 0x1F;
    // `clear fs` (Ghidra): fmt=W, ft=0, funct= corres 0 — zeroes an FPU register.
    if rs == 0x04 && rt == 0 && funct == 0 {
        return mk(w, "clear_f", Operands::Rd { rd: fs });
    }
    // `clear fs_fcr` (Ghidra): fmt=6, fs=31 — clears FCSR.
    if rs == 0x06 && rt == 0 && fs == 31 {
        return mk(w, "clear_fcsr", Operands::None);
    }
    match rs {
        0x00 => mk(w, "mfc1", Operands::CopMove { rt, rd: fs, sel: 0 }),
        0x02 => mk(w, "cfc1", Operands::CopMove { rt, rd: fs, sel: 0 }),
        0x04 => mk(w, "mtc1", Operands::CopMove { rt, rd: fs, sel: 0 }),
        0x06 => mk(w, "ctc1", Operands::CopMove { rt, rd: fs, sel: 0 }),
        0x08 => {
            let name = if rt & 1 == 0 { "bc1f" } else { "bc1t" };
            mk(w, name, Operands::Cop1Branch { offset: w as u16 as i16 })
        }
        0x10 | 0x11 => {
            let suffix = if rs == 0x10 { 's' } else { 'd' };
            let Some(base) = COP1_FUNCTS
                .iter()
                .find(|(f, _)| *f == funct)
                .map(|(_, b)| *b)
            else {
                return un(w);
            };
            let Some(m) = cop1_name(base, suffix) else {
                return un(w);
            };
            if base.starts_with("c.") {
                mk(w, m, Operands::Cop1Cmp { fs, ft })
            } else {
                mk(w, m, Operands::Cop1Arith { fd, fs, ft })
            }
        }
        _ => un(w),
    }
}

/// Map a COP1 base mnemonic + format suffix to a stable `'static` string.
fn cop1_name(base: &str, suffix: char) -> Option<&'static str> {
    const TABLE: &[(&str, &str, &str)] = &[
        ("add", "add.s", "add.d"), ("sub", "sub.s", "sub.d"),
        ("mul", "mul.s", "mul.d"), ("div", "div.s", "div.d"),
        ("sqrt", "sqrt.s", "sqrt.d"), ("abs", "abs.s", "abs.d"),
        ("mov", "mov.s", "mov.d"), ("neg", "neg.s", "neg.d"),
        ("adda", "adda.s", "adda.d"), ("suba", "suba.s", "suba.d"),
        ("mula", "mula.s", "mula.d"), ("diva", "diva.s", "diva.d"),
        ("cvt.s", "cvt.s", "cvt.s"), ("cvt.d", "cvt.d", "cvt.d"),
        ("cvt.w", "cvt.w", "cvt.w"), ("c.f", "c.f.s", "c.f.d"),
        ("c.un", "c.un.s", "c.un.d"), ("c.eq", "c.eq.s", "c.eq.d"),
        ("c.ueq", "c.ueq.s", "c.ueq.d"), ("c.olt", "c.olt.s", "c.olt.d"),
        ("c.ult", "c.ult.s", "c.ult.d"), ("c.ole", "c.ole.s", "c.ole.d"),
        ("c.ule", "c.ule.s", "c.ule.d"), ("c.sf", "c.sf.s", "c.sf.d"),
        ("c.ngle", "c.ngle.s", "c.ngle.d"), ("c.seq", "c.seq.s", "c.seq.d"),
        ("c.ngl", "c.ngl.s", "c.ngl.d"),
    ];
    TABLE
        .iter()
        .find(|(b, _, _)| *b == base)
        .map(|(_, s, d)| if suffix == 's' { *s } else { *d })
}

fn decode_cop2(w: u32, rs: u8, rt: u8, rd: u8, _funct: u8) -> Instruction {
    let funct = (w & 0x3F) as u8;
    match rs {
        0x00 => mk(w, "qmfc2", Operands::Cop2Move { rt, rd }),
        0x01 => mk(w, "qmtc2", Operands::Cop2Move { rt, rd }),
        0x02 => mk(w, "cfc2", Operands::Cop2Move { rt, rd }),
        0x04 => mk(w, "mtc2", Operands::Cop2Move { rt, rd }),
        0x06 => mk(w, "ctc2", Operands::Cop2Move { rt, rd }),
        // COP2 arithmetic (VU0 macro mode) is a large set; preserve structure.
        _ => mk(
            w,
            "cop2",
            Operands::Mmi { mmiop: rd, fct: funct, rd, rs, rt },
        ),
    }
}

/// MMI group names (opcode 0x1C). `mmiop` is the `sa` field, `fct` the funct.
fn mmi_mnemonic(mmiop: u8, fct: u8) -> Option<&'static str> {
    // Base group: selected by `fct` alone when `mmiop` is part of the operand.
    let base: Option<&'static str> = match fct {
        0x00 => Some("madd"),
        0x01 => Some("maddu"),
        0x02 => Some("mul"),
        0x04 => Some("plzcw"),
        0x10 => Some("mfhi1"),
        0x11 => Some("mthi1"),
        0x12 => Some("mflo1"),
        0x13 => Some("mtlo1"),
        0x14 => Some("mult1"),
        0x15 => Some("multu1"),
        0x16 => Some("div1"),
        0x17 => Some("divu1"),
        0x18 => Some("madd1"),
        0x19 => Some("maddu1"),
        0x30 => Some("pmfhl"),
        0x31 => Some("pmthl"),
        0x34 => Some("psllh"),
        0x36 => Some("psrlh"),
        0x37 => Some("psrah"),
        0x3C => Some("psllw"),
        0x3E => Some("psrlw"),
        0x3F => Some("psraw"),
        _ => None,
    };
    if let Some(b) = base {
        return Some(b);
    }
    // MMI0 group (fct == 0x08): `mmiop` selects the operation. We name the
    // ones confirmed against the emotion-engine Sleigh; the rest stay unnamed.
    if fct == 0x08 {
        return match mmiop {
            0x08 => Some("paddb"),
            0x04 => Some("paddh"),
            0x18 => Some("paddsb"),
            0x14 => Some("paddsh"),
            _ => None,
        };
    }
    // `clear rd` — mmiop=0x10, fct=0x28 with rs=rt=0.
    if fct == 0x28 && mmiop == 0x10 {
        return Some("clear");
    }
    None
}

fn decode_mmi(w: u32, rs: u8, rt: u8, rd: u8, mmiop: u8, fct: u8) -> Instruction {
    let operands = Operands::Mmi { mmiop, fct, rd, rs, rt };
    match mmi_mnemonic(mmiop, fct) {
        Some(name) => mk(w, name, operands),
        // Unknown MMI: keep the structure instead of discarding it.
        None => mk(w, "mmi", operands),
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.mnemonic)?;
        match self.operands {
            Operands::None | Operands::Word => Ok(()),
            Operands::RdRsRt { rd, rs, rt } => write!(f, " {}, {}, {}", reg(rd), reg(rs), reg(rt)),
            Operands::RdRtSa { rd, rt, sa } => write!(f, " {}, {}, {sa}", reg(rd), reg(rt)),
            Operands::RdRtRs { rd, rt, rs } => write!(f, " {}, {}, {}", reg(rd), reg(rt), reg(rs)),
            Operands::RsRt { rs, rt } => write!(f, " {}, {}", reg(rs), reg(rt)),
            Operands::Rd { rd } => write!(f, " {}", reg(rd)),
            Operands::Rs { rs } => write!(f, " {}", reg(rs)),
            Operands::RdRs { rd, rs } => write!(f, " {}, {}", reg(rd), reg(rs)),
            Operands::RtRsImm { rt, rs, imm } => {
                write!(f, " {}, {}, {}", reg(rt), reg(rs), imm)
            }
            Operands::RtRsImmU { rt, rs, imm } => {
                write!(f, " {}, {}, 0x{imm:x}", reg(rt), reg(rs))
            }
            Operands::RtImm { rt, imm } => write!(f, " {}, 0x{imm:x}", reg(rt)),
            Operands::Mem { rt, rs, offset } => {
                write!(f, " {}, {}({})", reg(rt), offset, reg(rs))
            }
            Operands::Branch { rs, rt, offset } => {
                write!(f, " {}, {}, {offset}", reg(rs), reg(rt))
            }
            Operands::Branch1 { rs, offset } => write!(f, " {}, {offset}", reg(rs)),
            Operands::Jump { target } => write!(f, " 0x{target:x}"),
            Operands::Code { code } => write!(f, " 0x{code:x}"),
            Operands::CopMove { rt, rd, sel } => {
                if sel == 0 {
                    write!(f, " {}, {}", reg(rt), reg(rd))
                } else {
                    write!(f, " {}, {}, {sel}", reg(rt), reg(rd))
                }
            }
            Operands::Cop2Move { rt, rd } => write!(f, " {}, {}", reg(rt), reg(rd)),
            Operands::Cop1Arith { fd, fs, ft } => {
                write!(f, " f{fd}, f{fs}, f{ft}")
            }
            Operands::Cop1Cmp { fs, ft } => write!(f, " f{fs}, f{ft}"),
            Operands::Cop1Branch { offset } => write!(f, " {offset}"),
            Operands::Mmi { mmiop, fct, rd, rs, rt } => match self.mnemonic {
                "clear" => write!(f, " {}", reg(rd)),
                "mthi1" | "mtlo1" => write!(f, " {}", reg(rs)),
                "mfhi1" | "mflo1" | "pmfhl" => write!(f, " {}", reg(rd)),
                "pmthl" => write!(f, " {}", reg(rs)),
                "mmi" => write!(
                    f,
                    " 0x{mmiop:02x},0x{fct:02x} {}, {}, {}",
                    reg(rd),
                    reg(rs),
                    reg(rt)
                ),
                _ => write!(f, " {}, {}, {}", reg(rd), reg(rs), reg(rt)),
            },
        }
    }
}
