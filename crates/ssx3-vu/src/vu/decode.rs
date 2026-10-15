//! VU instruction decoder: micro-mode upper/lower words and EE COP2 macro
//! words into [`Upper`] / [`Lower`] operations. Shared by the static
//! recompiler (`recomp`), macro mode (`Vu::macro_op`) and the test oracle.
//!
//! Field layout (both modes): dest 24..21 (x=bit 24 .. w=bit 21), ft/it
//! 20..16, fs/is 15..11, fd/id 10..6; DIV-family fsf 22..21, ftf 24..23.

use super::Src;

/// The FMAC operation families with broadcast/Q/I/vector variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fmac {
    Add,
    Sub,
    Madd,
    Msub,
    Mul,
    Max,
    Mini,
}

impl Fmac {
    pub fn name(self) -> &'static str {
        match self {
            Fmac::Add => "add",
            Fmac::Sub => "sub",
            Fmac::Madd => "madd",
            Fmac::Msub => "msub",
            Fmac::Mul => "mul",
            Fmac::Max => "max",
            Fmac::Mini => "mini",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Upper {
    Nop,
    /// `acc`: the ...A form writing ACC instead of `fd`.
    Fmac {
        op: Fmac,
        acc: bool,
        dest: u8,
        fd: u8,
        fs: u8,
        src: Src,
    },
    Opmula {
        fs: u8,
        ft: u8,
    },
    Opmsub {
        fd: u8,
        fs: u8,
        ft: u8,
    },
    Itof {
        frac: u8,
        dest: u8,
        ft: u8,
        fs: u8,
    },
    Ftoi {
        frac: u8,
        dest: u8,
        ft: u8,
        fs: u8,
    },
    Abs {
        dest: u8,
        ft: u8,
        fs: u8,
    },
    Clip {
        fs: u8,
        ft: u8,
    },
    Unknown(u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lower {
    Lq {
        dest: u8,
        ft: u8,
        is: u8,
        imm: i16,
    },
    Sq {
        dest: u8,
        fs: u8,
        it: u8,
        imm: i16,
    },
    Ilw {
        dest: u8,
        it: u8,
        is: u8,
        imm: i16,
    },
    Isw {
        dest: u8,
        it: u8,
        is: u8,
        imm: i16,
    },
    Iaddiu {
        it: u8,
        is: u8,
        imm: u16,
    },
    Isubiu {
        it: u8,
        is: u8,
        imm: u16,
    },
    Fceq {
        imm: u32,
    },
    Fcset {
        imm: u32,
    },
    Fcand {
        imm: u32,
    },
    Fcor {
        imm: u32,
    },
    Fseq {
        it: u8,
        imm: u16,
    },
    Fsset {
        imm: u16,
    },
    Fsand {
        it: u8,
        imm: u16,
    },
    Fsor {
        it: u8,
        imm: u16,
    },
    Fmeq {
        it: u8,
        is: u8,
    },
    Fmand {
        it: u8,
        is: u8,
    },
    Fmor {
        it: u8,
        is: u8,
    },
    Fcget {
        it: u8,
    },
    B {
        off: i16,
    },
    Bal {
        it: u8,
        off: i16,
    },
    Jr {
        is: u8,
    },
    Jalr {
        it: u8,
        is: u8,
    },
    Ibeq {
        it: u8,
        is: u8,
        off: i16,
    },
    Ibne {
        it: u8,
        is: u8,
        off: i16,
    },
    Ibltz {
        is: u8,
        off: i16,
    },
    Ibgtz {
        is: u8,
        off: i16,
    },
    Iblez {
        is: u8,
        off: i16,
    },
    Ibgez {
        is: u8,
        off: i16,
    },
    Iadd {
        id: u8,
        is: u8,
        it: u8,
    },
    Isub {
        id: u8,
        is: u8,
        it: u8,
    },
    Iaddi {
        it: u8,
        is: u8,
        imm: i8,
    },
    Iand {
        id: u8,
        is: u8,
        it: u8,
    },
    Ior {
        id: u8,
        is: u8,
        it: u8,
    },
    Move {
        dest: u8,
        ft: u8,
        fs: u8,
    },
    Mr32 {
        dest: u8,
        ft: u8,
        fs: u8,
    },
    Lqi {
        dest: u8,
        ft: u8,
        is: u8,
    },
    Sqi {
        dest: u8,
        fs: u8,
        it: u8,
    },
    Lqd {
        dest: u8,
        ft: u8,
        is: u8,
    },
    Sqd {
        dest: u8,
        fs: u8,
        it: u8,
    },
    Div {
        fs: u8,
        fsf: u8,
        ft: u8,
        ftf: u8,
    },
    Sqrt {
        ft: u8,
        ftf: u8,
    },
    Rsqrt {
        fs: u8,
        fsf: u8,
        ft: u8,
        ftf: u8,
    },
    Waitq,
    Mtir {
        it: u8,
        fs: u8,
        fsf: u8,
    },
    Mfir {
        dest: u8,
        ft: u8,
        is: u8,
    },
    Ilwr {
        dest: u8,
        it: u8,
        is: u8,
    },
    Iswr {
        dest: u8,
        it: u8,
        is: u8,
    },
    Rnext {
        dest: u8,
        ft: u8,
    },
    Rget {
        dest: u8,
        ft: u8,
    },
    Rinit {
        fs: u8,
        fsf: u8,
    },
    Rxor {
        fs: u8,
        fsf: u8,
    },
    Mfp {
        dest: u8,
        ft: u8,
    },
    Xtop {
        it: u8,
    },
    Xitop {
        it: u8,
    },
    Xgkick {
        is: u8,
    },
    /// EFU ops on fs.xyz / fs.xyzw (ESADD, ERSADD, ELENG, ERLENG, EATANxy, EATANxz, ESUM).
    EfuVec {
        op: Efu,
        fs: u8,
    },
    /// EFU ops on fs.fsf (ESQRT, ERSQRT, ERCPR, ESIN, EATAN, EEXP).
    EfuScalar {
        op: Efu,
        fs: u8,
        fsf: u8,
    },
    Waitp,
    Unknown(u32),
}

/// EFU ops (defined in the EE core: macro-mode code names them).
pub use ssx3_ee_rt::vu0::Efu;

/// EFU timing (kept here, with the rest of the VU timing model).
pub trait EfuLatency {
    fn latency(self) -> u64;
}

impl EfuLatency for Efu {
    /// Latency in cycles until P holds the result (VU user's manual).
    fn latency(self) -> u64 {
        match self {
            Efu::Esadd => 11,
            Efu::Ersadd => 18,
            Efu::Eleng => 18,
            Efu::Erleng => 24,
            Efu::Eatanxy | Efu::Eatanxz | Efu::Eatan => 54,
            Efu::Esum => 12,
            Efu::Esqrt => 12,
            Efu::Ersqrt => 18,
            Efu::Ercpr => 12,
            Efu::Esin => 29,
            Efu::Eexp => 44,
        }
    }
}

/// Macro-mode (EE COP2, bit 25 set) operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Macro {
    Upper(Upper),
    Lower(Lower),
    /// VCALLMS / VCALLMSR: handled by the EE via `Vu::start`.
    CallMs,
    Unknown(u32),
}

#[inline]
fn dest(w: u32) -> u8 {
    ((w >> 21) & 15) as u8
}
#[inline]
fn ft(w: u32) -> u8 {
    ((w >> 16) & 31) as u8
}
#[inline]
fn fs(w: u32) -> u8 {
    ((w >> 11) & 31) as u8
}
#[inline]
fn fd(w: u32) -> u8 {
    ((w >> 6) & 31) as u8
}
#[inline]
fn it(w: u32) -> u8 {
    ((w >> 16) & 15) as u8
}
#[inline]
fn is(w: u32) -> u8 {
    ((w >> 11) & 15) as u8
}
#[inline]
fn id(w: u32) -> u8 {
    ((w >> 6) & 15) as u8
}
#[inline]
fn fsf(w: u32) -> u8 {
    ((w >> 21) & 3) as u8
}
#[inline]
fn ftf(w: u32) -> u8 {
    ((w >> 23) & 3) as u8
}
#[inline]
fn imm11(w: u32) -> i16 {
    (((w << 21) as i32) >> 21) as i16
}
#[inline]
fn imm12(w: u32) -> u16 {
    (((w >> 10) & 0x800) | (w & 0x7ff)) as u16
}
#[inline]
fn imm15(w: u32) -> u16 {
    (((w >> 10) & 0x7800) | (w & 0x7ff)) as u16
}

/// Decode an upper word (also the macro-mode arithmetic encoding).
pub fn decode_upper(w: u32) -> Upper {
    let funct = w & 0x3f;
    let (d, t, s, f) = (dest(w), ft(w), fs(w), fd(w));
    let bc = (w & 3) as u8;
    let fm = |op: Fmac, acc: bool, src: Src| Upper::Fmac {
        op,
        acc,
        dest: d,
        fd: if acc { 0 } else { f },
        fs: s,
        src,
    };
    if funct < 0x30 {
        return match funct {
            0x00..=0x03 => fm(Fmac::Add, false, Src::Bc(t, bc)),
            0x04..=0x07 => fm(Fmac::Sub, false, Src::Bc(t, bc)),
            0x08..=0x0b => fm(Fmac::Madd, false, Src::Bc(t, bc)),
            0x0c..=0x0f => fm(Fmac::Msub, false, Src::Bc(t, bc)),
            0x10..=0x13 => fm(Fmac::Max, false, Src::Bc(t, bc)),
            0x14..=0x17 => fm(Fmac::Mini, false, Src::Bc(t, bc)),
            0x18..=0x1b => fm(Fmac::Mul, false, Src::Bc(t, bc)),
            0x1c => fm(Fmac::Mul, false, Src::Q),
            0x1d => fm(Fmac::Max, false, Src::I),
            0x1e => fm(Fmac::Mul, false, Src::I),
            0x1f => fm(Fmac::Mini, false, Src::I),
            0x20 => fm(Fmac::Add, false, Src::Q),
            0x21 => fm(Fmac::Madd, false, Src::Q),
            0x22 => fm(Fmac::Add, false, Src::I),
            0x23 => fm(Fmac::Madd, false, Src::I),
            0x24 => fm(Fmac::Sub, false, Src::Q),
            0x25 => fm(Fmac::Msub, false, Src::Q),
            0x26 => fm(Fmac::Sub, false, Src::I),
            0x27 => fm(Fmac::Msub, false, Src::I),
            0x28 => fm(Fmac::Add, false, Src::Vec(t)),
            0x29 => fm(Fmac::Madd, false, Src::Vec(t)),
            0x2a => fm(Fmac::Mul, false, Src::Vec(t)),
            0x2b => fm(Fmac::Max, false, Src::Vec(t)),
            0x2c => fm(Fmac::Sub, false, Src::Vec(t)),
            0x2d => fm(Fmac::Msub, false, Src::Vec(t)),
            0x2e => Upper::Opmsub {
                fd: f,
                fs: s,
                ft: t,
            },
            _ => fm(Fmac::Mini, false, Src::Vec(t)), // 0x2f
        };
    }
    if funct < 0x3c {
        return Upper::Unknown(w);
    }
    let index = (((w >> 6) & 31) << 2) | (w & 3);
    const FRAC: [u8; 4] = [0, 4, 12, 15];
    match index {
        0x00..=0x03 => fm(Fmac::Add, true, Src::Bc(t, bc)),
        0x04..=0x07 => fm(Fmac::Sub, true, Src::Bc(t, bc)),
        0x08..=0x0b => fm(Fmac::Madd, true, Src::Bc(t, bc)),
        0x0c..=0x0f => fm(Fmac::Msub, true, Src::Bc(t, bc)),
        0x10..=0x13 => Upper::Itof {
            frac: FRAC[(index & 3) as usize],
            dest: d,
            ft: t,
            fs: s,
        },
        0x14..=0x17 => Upper::Ftoi {
            frac: FRAC[(index & 3) as usize],
            dest: d,
            ft: t,
            fs: s,
        },
        0x18..=0x1b => fm(Fmac::Mul, true, Src::Bc(t, bc)),
        0x1c => fm(Fmac::Mul, true, Src::Q),
        0x1d => Upper::Abs {
            dest: d,
            ft: t,
            fs: s,
        },
        0x1e => fm(Fmac::Mul, true, Src::I),
        0x1f => Upper::Clip { fs: s, ft: t },
        0x20 => fm(Fmac::Add, true, Src::Q),
        0x21 => fm(Fmac::Madd, true, Src::Q),
        0x22 => fm(Fmac::Add, true, Src::I),
        0x23 => fm(Fmac::Madd, true, Src::I),
        0x24 => fm(Fmac::Sub, true, Src::Q),
        0x25 => fm(Fmac::Msub, true, Src::Q),
        0x26 => fm(Fmac::Sub, true, Src::I),
        0x27 => fm(Fmac::Msub, true, Src::I),
        0x28 => fm(Fmac::Add, true, Src::Vec(t)),
        0x29 => fm(Fmac::Madd, true, Src::Vec(t)),
        0x2a => fm(Fmac::Mul, true, Src::Vec(t)),
        0x2c => fm(Fmac::Sub, true, Src::Vec(t)),
        0x2d => fm(Fmac::Msub, true, Src::Vec(t)),
        0x2e => Upper::Opmula { fs: s, ft: t },
        0x2f => Upper::Nop,
        _ => Upper::Unknown(w),
    }
}

/// The 0x40-format / macro lower ops selected by the special2 index.
/// `micro` enables the micro-only ops (MFP, XTOP, XGKICK, EFU, ...).
fn decode_special2_lower(w: u32, index: u32, micro: bool) -> Lower {
    let (d, t, s) = (dest(w), ft(w), fs(w));
    let (ti, si) = (it(w), is(w));
    match index {
        0x30 => Lower::Move {
            dest: d,
            ft: t,
            fs: s,
        },
        0x31 => Lower::Mr32 {
            dest: d,
            ft: t,
            fs: s,
        },
        0x34 => Lower::Lqi {
            dest: d,
            ft: t,
            is: si,
        },
        0x35 => Lower::Sqi {
            dest: d,
            fs: s,
            it: ti,
        },
        0x36 => Lower::Lqd {
            dest: d,
            ft: t,
            is: si,
        },
        0x37 => Lower::Sqd {
            dest: d,
            fs: s,
            it: ti,
        },
        0x38 => Lower::Div {
            fs: s,
            fsf: fsf(w),
            ft: t,
            ftf: ftf(w),
        },
        0x39 => Lower::Sqrt { ft: t, ftf: ftf(w) },
        0x3a => Lower::Rsqrt {
            fs: s,
            fsf: fsf(w),
            ft: t,
            ftf: ftf(w),
        },
        0x3b => Lower::Waitq,
        0x3c => Lower::Mtir {
            it: ti,
            fs: s,
            fsf: fsf(w),
        },
        0x3d => Lower::Mfir {
            dest: d,
            ft: t,
            is: si,
        },
        0x3e => Lower::Ilwr {
            dest: d,
            it: ti,
            is: si,
        },
        0x3f => Lower::Iswr {
            dest: d,
            it: ti,
            is: si,
        },
        0x40 => Lower::Rnext { dest: d, ft: t },
        0x41 => Lower::Rget { dest: d, ft: t },
        0x42 => Lower::Rinit { fs: s, fsf: fsf(w) },
        0x43 => Lower::Rxor { fs: s, fsf: fsf(w) },
        _ if !micro => Lower::Unknown(w),
        0x64 => Lower::Mfp { dest: d, ft: t },
        0x68 => Lower::Xtop { it: ti },
        0x69 => Lower::Xitop { it: ti },
        0x6c => Lower::Xgkick { is: si },
        0x70 => Lower::EfuVec {
            op: Efu::Esadd,
            fs: s,
        },
        0x71 => Lower::EfuVec {
            op: Efu::Ersadd,
            fs: s,
        },
        0x72 => Lower::EfuVec {
            op: Efu::Eleng,
            fs: s,
        },
        0x73 => Lower::EfuVec {
            op: Efu::Erleng,
            fs: s,
        },
        0x74 => Lower::EfuVec {
            op: Efu::Eatanxy,
            fs: s,
        },
        0x75 => Lower::EfuVec {
            op: Efu::Eatanxz,
            fs: s,
        },
        0x76 => Lower::EfuVec {
            op: Efu::Esum,
            fs: s,
        },
        0x78 => Lower::EfuScalar {
            op: Efu::Esqrt,
            fs: s,
            fsf: fsf(w),
        },
        0x79 => Lower::EfuScalar {
            op: Efu::Ersqrt,
            fs: s,
            fsf: fsf(w),
        },
        0x7a => Lower::EfuScalar {
            op: Efu::Ercpr,
            fs: s,
            fsf: fsf(w),
        },
        0x7b => Lower::Waitp,
        0x7c => Lower::EfuScalar {
            op: Efu::Esin,
            fs: s,
            fsf: fsf(w),
        },
        0x7d => Lower::EfuScalar {
            op: Efu::Eatan,
            fs: s,
            fsf: fsf(w),
        },
        0x7e => Lower::EfuScalar {
            op: Efu::Eexp,
            fs: s,
            fsf: fsf(w),
        },
        _ => Lower::Unknown(w),
    }
}

/// Integer ALU ops shared by the micro 0x40 format and macro special1.
fn decode_int_alu(w: u32, funct: u32) -> Option<Lower> {
    let (ti, si, di) = (it(w), is(w), id(w));
    Some(match funct {
        0x30 => Lower::Iadd {
            id: di,
            is: si,
            it: ti,
        },
        0x31 => Lower::Isub {
            id: di,
            is: si,
            it: ti,
        },
        0x32 => Lower::Iaddi {
            it: ti,
            is: si,
            imm: ((((w >> 6) & 31) << 27) as i32 >> 27) as i8,
        },
        0x34 => Lower::Iand {
            id: di,
            is: si,
            it: ti,
        },
        0x35 => Lower::Ior {
            id: di,
            is: si,
            it: ti,
        },
        _ => return None,
    })
}

/// Decode a micro-mode lower word (the caller handles the I bit).
pub fn decode_lower(w: u32) -> Lower {
    let (d, t, s) = (dest(w), ft(w), fs(w));
    let (ti, si) = (it(w), is(w));
    let off = imm11(w);
    match w >> 25 {
        0x00 => Lower::Lq {
            dest: d,
            ft: t,
            is: si,
            imm: off,
        },
        0x01 => Lower::Sq {
            dest: d,
            fs: s,
            it: ti,
            imm: off,
        },
        0x04 => Lower::Ilw {
            dest: d,
            it: ti,
            is: si,
            imm: off,
        },
        0x05 => Lower::Isw {
            dest: d,
            it: ti,
            is: si,
            imm: off,
        },
        0x08 => Lower::Iaddiu {
            it: ti,
            is: si,
            imm: imm15(w),
        },
        0x09 => Lower::Isubiu {
            it: ti,
            is: si,
            imm: imm15(w),
        },
        0x10 => Lower::Fceq { imm: w & 0xff_ffff },
        0x11 => Lower::Fcset { imm: w & 0xff_ffff },
        0x12 => Lower::Fcand { imm: w & 0xff_ffff },
        0x13 => Lower::Fcor { imm: w & 0xff_ffff },
        0x14 => Lower::Fseq {
            it: ti,
            imm: imm12(w),
        },
        0x15 => Lower::Fsset { imm: imm12(w) },
        0x16 => Lower::Fsand {
            it: ti,
            imm: imm12(w),
        },
        0x17 => Lower::Fsor {
            it: ti,
            imm: imm12(w),
        },
        0x18 => Lower::Fmeq { it: ti, is: si },
        0x1a => Lower::Fmand { it: ti, is: si },
        0x1b => Lower::Fmor { it: ti, is: si },
        0x1c => Lower::Fcget { it: ti },
        0x20 => Lower::B { off },
        0x21 => Lower::Bal { it: ti, off },
        0x24 => Lower::Jr { is: si },
        0x25 => Lower::Jalr { it: ti, is: si },
        0x28 => Lower::Ibeq {
            it: ti,
            is: si,
            off,
        },
        0x29 => Lower::Ibne {
            it: ti,
            is: si,
            off,
        },
        0x2c => Lower::Ibltz { is: si, off },
        0x2d => Lower::Ibgtz { is: si, off },
        0x2e => Lower::Iblez { is: si, off },
        0x2f => Lower::Ibgez { is: si, off },
        0x40 => {
            let funct = w & 0x3f;
            if funct < 0x3c {
                decode_int_alu(w, funct).unwrap_or(Lower::Unknown(w))
            } else {
                decode_special2_lower(w, (((w >> 6) & 31) << 2) | (w & 3), true)
            }
        }
        _ => Lower::Unknown(w),
    }
}

/// Decode an EE COP2 macro instruction (bit 25 set).
pub fn decode_macro(w: u32) -> Macro {
    let funct = w & 0x3f;
    match funct {
        0x00..=0x2f => match decode_upper(w) {
            Upper::Unknown(x) => Macro::Unknown(x),
            u => Macro::Upper(u),
        },
        0x38 | 0x39 => Macro::CallMs,
        0x30..=0x3b => decode_int_alu(w, funct)
            .map(Macro::Lower)
            .unwrap_or(Macro::Unknown(w)),
        _ => {
            let index = (((w >> 6) & 31) << 2) | (w & 3);
            if index < 0x30 {
                match decode_upper(w) {
                    Upper::Unknown(x) => Macro::Unknown(x),
                    u => Macro::Upper(u),
                }
            } else {
                match decode_special2_lower(w, index, false) {
                    Lower::Unknown(x) => Macro::Unknown(x),
                    l => Macro::Lower(l),
                }
            }
        }
    }
}

/// Branch kind of a lower op, for control-flow analysis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flow {
    None,
    /// Unconditional relative branch (B / BAL).
    Jump {
        off: i16,
    },
    /// Conditional relative branch.
    Cond {
        off: i16,
    },
    /// Register jump (JR / JALR).
    Indirect,
}

impl Lower {
    pub fn flow(&self) -> Flow {
        match *self {
            Lower::B { off } | Lower::Bal { off, .. } => Flow::Jump { off },
            Lower::Ibeq { off, .. }
            | Lower::Ibne { off, .. }
            | Lower::Ibltz { off, .. }
            | Lower::Ibgtz { off, .. }
            | Lower::Iblez { off, .. }
            | Lower::Ibgez { off, .. } => Flow::Cond { off },
            Lower::Jr { .. } | Lower::Jalr { .. } => Flow::Indirect,
            _ => Flow::None,
        }
    }
    /// True for BAL / JALR (which write a return address).
    pub fn links(&self) -> bool {
        matches!(self, Lower::Bal { .. } | Lower::Jalr { .. })
    }
}

const XYZ: u8 = 0b1110;

fn lane_mask(field: u8) -> u8 {
    8 >> field
}

/// VF fields read by an upper op (for the stall model): (register, field mask).
pub fn upper_reads(u: &Upper, out: &mut Vec<(u8, u8)>) {
    match *u {
        Upper::Fmac { dest, fs, src, .. } => {
            out.push((fs, dest));
            match src {
                Src::Vec(t) => out.push((t, dest)),
                Src::Bc(t, b) => out.push((t, lane_mask(b))),
                _ => {}
            }
        }
        Upper::Opmula { fs, ft } | Upper::Opmsub { fs, ft, .. } => {
            out.push((fs, XYZ));
            out.push((ft, XYZ));
        }
        Upper::Itof { dest, fs, .. }
        | Upper::Ftoi { dest, fs, .. }
        | Upper::Abs { dest, fs, .. } => out.push((fs, dest)),
        Upper::Clip { fs, ft } => {
            out.push((fs, XYZ));
            out.push((ft, 1));
        }
        Upper::Nop | Upper::Unknown(_) => {}
    }
}

/// A lower op that waits for the DIV unit (WAITQ, or a new DIV/SQRT/RSQRT
/// while one is in flight). The wait stalls the whole pair before its upper
/// op runs (see [`super::Vu::stall_q`]).
pub fn lower_waits_q(l: &Lower) -> bool {
    matches!(l, Lower::Waitq | Lower::Div { .. } | Lower::Sqrt { .. } | Lower::Rsqrt { .. })
}

/// VF fields read by a lower op (for the stall model).
pub fn lower_reads(l: &Lower, out: &mut Vec<(u8, u8)>) {
    match *l {
        Lower::Sq { dest, fs, .. } | Lower::Sqi { dest, fs, .. } | Lower::Sqd { dest, fs, .. } => {
            out.push((fs, dest))
        }
        Lower::Move { dest, fs, .. } => out.push((fs, dest)),
        Lower::Mr32 { fs, .. } => out.push((fs, 0xf)),
        Lower::Div { fs, fsf, ft, ftf } | Lower::Rsqrt { fs, fsf, ft, ftf } => {
            out.push((fs, lane_mask(fsf)));
            out.push((ft, lane_mask(ftf)));
        }
        Lower::Sqrt { ft, ftf } => out.push((ft, lane_mask(ftf))),
        Lower::Mtir { fs, fsf, .. } | Lower::Rinit { fs, fsf } | Lower::Rxor { fs, fsf } => {
            out.push((fs, lane_mask(fsf)))
        }
        Lower::EfuVec { op, fs } => out.push((fs, if op == Efu::Esum { 0xf } else { XYZ })),
        Lower::EfuScalar { fs, fsf, .. } => out.push((fs, lane_mask(fsf))),
        _ => {}
    }
}

/// Combined, de-duplicated read set for a pair (vf0 is always ready and dropped).
pub fn pair_reads(upper: &Upper, lower: Option<&Lower>) -> Vec<(u8, u8)> {
    let mut v = Vec::new();
    upper_reads(upper, &mut v);
    if let Some(l) = lower {
        lower_reads(l, &mut v);
    }
    let mut out: Vec<(u8, u8)> = Vec::new();
    for (r, m) in v {
        if r == 0 || m == 0 {
            continue;
        }
        if let Some(e) = out.iter_mut().find(|e| e.0 == r) {
            e.1 |= m;
        } else {
            out.push((r, m));
        }
    }
    out
}
