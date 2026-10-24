//! EE code generator: R5900 machine code → Rust, against `ssx3_ee_rt::Cpu`.
//!
//! Every instruction of every function in the inventory is lowered to Rust at
//! build time; nothing is decoded at run time. Simple ALU operations are
//! emitted inline; loads/stores, multiply/divide, MMI, FPU and COP2 call the
//! semantics library in `ssx3-ee-rt` (the stable core; the hardware model in
//! `ssx3-ps2` is reached through its `Platform` trait object and hooks).
//!
//! ## Control flow
//!
//! A function is emitted as a block dispatcher over its *labels*: its entry,
//! every branch target, every return point after a call, the instruction after
//! every syscall, and every address inside it that data refers to (jump
//! tables). Any label can be an entry point, which is what the runtime needs
//! to resume a thread at the exact instruction where it was switched out.
//!
//! * `jal` is a direct Rust call (same shard) or `call_indirect` (other shard),
//!   followed by an unwind check: if `c.pc` is not the return address after the
//!   callee returns, the thread was switched (or the call was deferred because
//!   the host stack is deep), so this frame returns too.
//! * `jr $ra` returns from the Rust function after setting `c.pc`.
//! * Backward branches check `c.cycles >= c.next_event` to deliver interrupts.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

/// Instruction source: address → word.
pub trait Image {
    fn word(&self, addr: u32) -> Option<u32>;
}

/// A function to translate: entry, extent, and authoritative instruction runs.
#[derive(Clone, Debug)]
pub struct Func {
    pub start: u32,
    pub end: u32,
    pub runs: Vec<(u32, u32)>,
    /// Recovered symbol name (from Ghidra), if not an auto-generated one.
    pub name: Option<String>,
}

pub fn fn_name(addr: u32) -> String {
    format!("f_{addr:08x}")
}

/// How control leaves an instruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl {
    /// Not a control transfer.
    None,
    /// Conditional branch (with delay slot). `likely`: delay slot only if taken.
    Branch {
        target: u32,
        likely: bool,
        link: bool,
    },
    /// `j`.
    Jump { target: u32 },
    /// `jal`.
    Call { target: u32 },
    /// `jr rs`.
    JumpReg { rs: u8 },
    /// `jalr rd, rs`.
    CallReg { rd: u8, rs: u8 },
    /// `syscall`: ends a block, next instruction is a label.
    Syscall,
    /// `eret`, `break`: no fallthrough.
    Stop,
}

fn branch_target(pc: u32, w: u32) -> u32 {
    pc.wrapping_add(4)
        .wrapping_add(((w as i16 as i32) << 2) as u32)
}

fn classify(pc: u32, w: u32) -> Ctl {
    let op = w >> 26;
    let rs = ((w >> 21) & 31) as u8;
    let rt = (w >> 16) & 31;
    let funct = w & 63;
    let b = |likely, link| Ctl::Branch {
        target: branch_target(pc, w),
        likely,
        link,
    };
    match op {
        0x00 => match funct {
            0x08 => Ctl::JumpReg { rs },
            0x09 => Ctl::CallReg {
                rd: ((w >> 11) & 31) as u8,
                rs,
            },
            0x0C => Ctl::Syscall,
            0x0D => Ctl::Stop,
            _ => Ctl::None,
        },
        0x01 => match rt {
            0x00 | 0x01 => b(false, false),
            0x02 | 0x03 => b(true, false),
            0x10 | 0x11 => b(false, true),
            0x12 | 0x13 => b(true, true),
            _ => Ctl::None,
        },
        0x02 => Ctl::Jump {
            target: (pc & 0xF000_0000) | ((w & 0x03FF_FFFF) << 2),
        },
        0x03 => Ctl::Call {
            target: (pc & 0xF000_0000) | ((w & 0x03FF_FFFF) << 2),
        },
        0x04..=0x07 => b(false, false),
        0x14..=0x17 => b(true, false),
        0x10 => {
            if rs == 0x08 {
                b(rt & 2 != 0, false)
            } else if rs == 0x10 && funct == 0x18 {
                Ctl::Stop
            } else {
                Ctl::None
            }
        }
        0x11 | 0x12 if rs == 0x08 => b(rt & 2 != 0, false),
        _ => Ctl::None,
    }
}

/// Rust expression for the branch condition (evaluated before the delay slot).
fn branch_cond(w: u32) -> String {
    let op = w >> 26;
    let rs = (w >> 21) & 31;
    let rt = (w >> 16) & 31;
    let r = |n: u32| {
        if n == 0 {
            "0u64".to_string()
        } else {
            format!("c.g[{n}][0]")
        }
    };
    let s = |n: u32| {
        if n == 0 {
            "0i64".to_string()
        } else {
            format!("(c.g[{n}][0] as i64)")
        }
    };
    match op {
        0x04 | 0x14 => {
            if rs == rt {
                "true".into()
            } else {
                format!("{} == {}", r(rs), r(rt))
            }
        }
        0x05 | 0x15 => format!("{} != {}", r(rs), r(rt)),
        0x06 | 0x16 => format!("{} <= 0", s(rs)),
        0x07 | 0x17 => format!("{} > 0", s(rs)),
        0x01 => match rt & 1 {
            0 => format!("{} < 0", s(rs)),
            _ => format!("{} >= 0", s(rs)),
        },
        // bc0f/bc0t: CPCOND0 (DMA idle) — transfers complete instantly, so true.
        0x10 => (if rt & 1 == 1 { "true" } else { "false" }).into(),
        0x11 => (if rt & 1 == 1 { "c.fcc()" } else { "!c.fcc()" }).into(),
        // bc2f/bc2t: VU0 busy — VU0 microprograms run synchronously, so never busy.
        0x12 => (if rt & 1 == 1 { "false" } else { "true" }).into(),
        _ => "false".into(),
    }
}

fn reg(n: u32) -> String {
    format!("c.g[{n}][0]")
}

fn r32(n: u32) -> String {
    if n == 0 {
        "0u32".into()
    } else {
        format!("(c.g[{n}][0] as u32)")
    }
}

fn r64(n: u32) -> String {
    if n == 0 {
        "0u64".into()
    } else {
        format!("c.g[{n}][0]")
    }
}

fn ea(rs: u32, off: i16) -> String {
    if off == 0 {
        r32(rs)
    } else {
        format!("{}.wrapping_add({:#x})", r32(rs), off as i32 as u32)
    }
}

/// Assign a 64-bit value expression to GPR `rd` (dropped for `$zero`).
fn set(rd: u32, e: String) -> String {
    if rd == 0 {
        String::new()
    } else {
        format!("{} = {};", reg(rd), e)
    }
}

/// Lower a non-control instruction to Rust statements. Returns `None` if the
/// encoding is invalid.
pub fn lower(pc: u32, w: u32) -> Option<String> {
    let op = w >> 26;
    let rs = (w >> 21) & 31;
    let rt = (w >> 16) & 31;
    let rd = (w >> 11) & 31;
    let sa = (w >> 6) & 31;
    let funct = w & 63;
    let imm = w as u16;
    let simm = w as i16;
    let s = match op {
        0x00 => match funct {
            0x00 => set(rd, format!("sx({} << {sa})", r32(rt))),
            0x02 => set(rd, format!("sx({} >> {sa})", r32(rt))),
            0x03 => set(rd, format!("sx(({} as i32 >> {sa}) as u32)", r32(rt))),
            0x04 => set(rd, format!("sx({} << ({} & 31))", r32(rt), r32(rs))),
            0x06 => set(rd, format!("sx({} >> ({} & 31))", r32(rt), r32(rs))),
            0x07 => set(
                rd,
                format!("sx(({} as i32 >> ({} & 31)) as u32)", r32(rt), r32(rs)),
            ),
            0x0A => {
                if rd == 0 {
                    String::new()
                } else {
                    format!("if {} == 0 {{ {} = {}; }}", r64(rt), reg(rd), r64(rs))
                }
            }
            0x0B => {
                if rd == 0 {
                    String::new()
                } else {
                    format!("if {} != 0 {{ {} = {}; }}", r64(rt), reg(rd), r64(rs))
                }
            }
            0x0F => String::new(), // sync
            0x10 => set(rd, "c.hi[0]".into()),
            0x11 => format!("c.hi[0] = {};", r64(rs)),
            0x12 => set(rd, "c.lo[0]".into()),
            0x13 => format!("c.lo[0] = {};", r64(rs)),
            0x14 => set(rd, format!("{} << ({} & 63)", r64(rt), r64(rs))),
            0x16 => set(rd, format!("{} >> ({} & 63)", r64(rt), r64(rs))),
            0x17 => set(
                rd,
                format!("(({} as i64) >> ({} & 63)) as u64", r64(rt), r64(rs)),
            ),
            0x18 => format!("c.mult(0, {rd}, {rs}, {rt});"),
            0x19 => format!("c.multu(0, {rd}, {rs}, {rt});"),
            0x1A => format!("c.div(0, {rs}, {rt});"),
            0x1B => format!("c.divu(0, {rs}, {rt});"),
            0x20 | 0x21 => set(rd, format!("sx({}.wrapping_add({}))", r32(rs), r32(rt))),
            0x22 | 0x23 => set(rd, format!("sx({}.wrapping_sub({}))", r32(rs), r32(rt))),
            0x24 => set(rd, format!("{} & {}", r64(rs), r64(rt))),
            0x25 if rt == 0 => set(rd, r64(rs)), // move
            0x25 if rs == 0 => set(rd, r64(rt)),
            0x25 => set(rd, format!("{} | {}", r64(rs), r64(rt))),
            0x26 => set(rd, format!("{} ^ {}", r64(rs), r64(rt))),
            0x27 => set(rd, format!("!({} | {})", r64(rs), r64(rt))),
            0x28 => set(rd, "c.sa as u64".into()),
            0x29 => format!("c.sa = {} as u32;", r64(rs)),
            0x2A => set(
                rd,
                format!("(({} as i64) < ({} as i64)) as u64", r64(rs), r64(rt)),
            ),
            0x2B => set(rd, format!("({} < {}) as u64", r64(rs), r64(rt))),
            0x2C | 0x2D if rt == 0 => set(rd, r64(rs)), // move
            0x2C | 0x2D if rs == 0 => set(rd, r64(rt)),
            0x2C | 0x2D => set(rd, format!("{}.wrapping_add({})", r64(rs), r64(rt))),
            0x2E | 0x2F => set(rd, format!("{}.wrapping_sub({})", r64(rs), r64(rt))),
            0x30..=0x36 => String::new(), // traps: not modelled
            0x38 => set(rd, format!("{} << {sa}", r64(rt))),
            0x3A => set(rd, format!("{} >> {sa}", r64(rt))),
            0x3B => set(rd, format!("(({} as i64) >> {sa}) as u64", r64(rt))),
            0x3C => set(rd, format!("{} << {}", r64(rt), sa + 32)),
            0x3E => set(rd, format!("{} >> {}", r64(rt), sa + 32)),
            0x3F => set(rd, format!("(({} as i64) >> {}) as u64", r64(rt), sa + 32)),
            _ => return None,
        },
        0x01 => match rt {
            0x08..=0x0E => String::new(), // trap immediates
            0x18 => format!("c.mtsab({rs}, {});", imm),
            0x19 => format!("c.mtsah({rs}, {});", imm),
            _ => return None,
        },
        0x08 | 0x09 if rs == 0 => set(rt, format!("{:#x}", simm as i64 as u64)), // li
        0x08 | 0x09 => set(
            rt,
            format!("sx({}.wrapping_add({:#x}))", r32(rs), simm as i32 as u32),
        ),
        0x0A => set(rt, format!("(({} as i64) < {}i64) as u64", r64(rs), simm)),
        0x0B => set(
            rt,
            format!("({} < {:#x}u64) as u64", r64(rs), simm as i64 as u64),
        ),
        0x0C => set(rt, format!("{} & {:#x}", r64(rs), imm)),
        0x0D if rs == 0 => set(rt, format!("{:#x}", imm)),
        0x0D => set(rt, format!("{} | {:#x}", r64(rs), imm)),
        0x0E => set(rt, format!("{} ^ {:#x}", r64(rs), imm)),
        0x0F => set(
            rt,
            format!("{:#x}", (((imm as u32) << 16) as i32 as i64) as u64),
        ),
        0x10 => lower_cop0(w, rs, rt, rd)?,
        0x11 => lower_cop1(w, rs, rt, rd, sa)?,
        0x12 => lower_cop2(w, rs, rt, rd)?,
        0x18 | 0x19 => set(
            rt,
            format!("{}.wrapping_add({:#x})", r64(rs), simm as i64 as u64),
        ),
        0x1A => format!("c.ldl({rt}, {});", ea(rs, simm)),
        0x1B => format!("c.ldr({rt}, {});", ea(rs, simm)),
        0x1C => lower_mmi(w, rs, rt, rd, sa)?,
        0x1E => format!("c.lq({rt}, {});", ea(rs, simm)),
        0x1F => format!("c.sq({rt}, {});", ea(rs, simm)),
        0x20 => load(rt, "lb", rs, simm),
        0x21 => load(rt, "lh", rs, simm),
        0x22 => format!("c.lwl({rt}, {});", ea(rs, simm)),
        0x23 => load(rt, "lw", rs, simm),
        0x24 => load(rt, "lbu", rs, simm),
        0x25 => load(rt, "lhu", rs, simm),
        0x26 => format!("c.lwr({rt}, {});", ea(rs, simm)),
        0x27 => load(rt, "lwu", rs, simm),
        0x28 => format!("c.sb({}, {});", ea(rs, simm), r64(rt)),
        0x29 => format!("c.sh({}, {});", ea(rs, simm), r64(rt)),
        0x2A => format!("c.swl({rt}, {});", ea(rs, simm)),
        0x2B => format!("c.sw({}, {});", ea(rs, simm), r64(rt)),
        0x2C => format!("c.sdl({rt}, {});", ea(rs, simm)),
        0x2D => format!("c.sdr({rt}, {});", ea(rs, simm)),
        0x2E => format!("c.swr({rt}, {});", ea(rs, simm)),
        0x2F | 0x33 => String::new(), // cache, pref
        0x31 => format!("c.lwc1({rt}, {});", ea(rs, simm)),
        0x36 => format!("c.lqc2({rt}, {});", ea(rs, simm)),
        0x37 => load(rt, "ld", rs, simm),
        0x39 => format!("c.swc1({rt}, {});", ea(rs, simm)),
        0x3E => format!("c.sqc2({rt}, {});", ea(rs, simm)),
        0x3F => format!("c.sd({}, {});", ea(rs, simm), r64(rt)),
        _ => return None,
    };
    let _ = pc;
    Some(s)
}

fn load(rt: u32, f: &str, rs: u32, off: i16) -> String {
    if rt == 0 {
        format!("c.{f}({});", ea(rs, off))
    } else {
        format!("{} = c.{f}({});", reg(rt), ea(rs, off))
    }
}

fn lower_cop0(w: u32, rs: u32, rt: u32, rd: u32) -> Option<String> {
    Some(match rs {
        0x00 => format!("c.mfc0({rt}, {rd});"),
        0x04 => format!("c.mtc0({rt}, {rd});"),
        0x10 => match w & 63 {
            0x01 | 0x02 | 0x06 | 0x08 => String::new(), // TLB ops
            0x38 => "c.ei();".into(),
            0x39 => "c.di();".into(),
            _ => return None,
        },
        _ => return None,
    })
}

fn lower_cop1(w: u32, rs: u32, rt: u32, rd: u32, sa: u32) -> Option<String> {
    let (ft, fs, fd) = (rt, rd, sa);
    let funct = w & 63;
    Some(match rs {
        0x00 => format!("c.mfc1({rt}, {fs});"),
        0x02 => format!("c.cfc1({rt}, {fs});"),
        0x04 => format!("c.mtc1({rt}, {fs});"),
        0x06 => format!("c.ctc1({rt}, {fs});"),
        0x10 => match funct {
            0x00 => format!("c.add_s({fd}, {fs}, {ft});"),
            0x01 => format!("c.sub_s({fd}, {fs}, {ft});"),
            0x02 => format!("c.mul_s({fd}, {fs}, {ft});"),
            0x03 => format!("c.div_s({fd}, {fs}, {ft});"),
            0x04 => format!("c.sqrt_s({fd}, {ft});"),
            0x05 => format!("c.abs_s({fd}, {fs});"),
            0x06 => format!("c.mov_s({fd}, {fs});"),
            0x07 => format!("c.neg_s({fd}, {fs});"),
            0x16 => format!("c.rsqrt_s({fd}, {fs}, {ft});"),
            0x18 => format!("c.adda_s({fs}, {ft});"),
            0x19 => format!("c.suba_s({fs}, {ft});"),
            0x1A => format!("c.mula_s({fs}, {ft});"),
            0x1C => format!("c.madd_s({fd}, {fs}, {ft});"),
            0x1D => format!("c.msub_s({fd}, {fs}, {ft});"),
            0x1E => format!("c.madda_s({fs}, {ft});"),
            0x1F => format!("c.msuba_s({fs}, {ft});"),
            0x24 => format!("c.cvt_w_s({fd}, {fs});"),
            0x28 => format!("c.max_s({fd}, {fs}, {ft});"),
            0x29 => format!("c.min_s({fd}, {fs}, {ft});"),
            0x30 => "c.c_f_s();".into(),
            0x32 => format!("c.c_eq_s({fs}, {ft});"),
            0x34 => format!("c.c_lt_s({fs}, {ft});"),
            0x36 => format!("c.c_le_s({fs}, {ft});"),
            _ => return None,
        },
        0x14 => match funct {
            0x20 => format!("c.cvt_s_w({fd}, {fs});"),
            _ => return None,
        },
        _ => return None,
    })
}

fn lower_cop2(w: u32, rs: u32, rt: u32, rd: u32) -> Option<String> {
    Some(match rs {
        0x01 => format!("c.qmfc2({rt}, {rd});"),
        0x02 => format!("c.cfc2({rt}, {rd});"),
        0x05 => format!("c.qmtc2({rt}, {rd});"),
        0x06 => format!("c.ctc2({rt}, {rd});"),
        0x10..=0x1F => {
            let funct = w & 63;
            match funct {
                0x38 => format!("c.vcallms({:#x});", ((w >> 6) & 0x7FFF) * 8),
                0x39 => "c.vcallmsr();".into(),
                _ => match ssx3_vu::vu::recomp::macro_source(w) {
                    Some(src) => src.replacen("vu.", "c.vu0().", 1),
                    None => format!("c.vu0_macro({w:#010x});"),
                },
            }
        }
        _ => return None,
    })
}

fn lower_mmi(w: u32, rs: u32, rt: u32, rd: u32, sa: u32) -> Option<String> {
    let funct = w & 63;
    let three = |m: &str| format!("c.{m}({rd}, {rs}, {rt});");
    let two_t = |m: &str| format!("c.{m}({rd}, {rt});");
    Some(match funct {
        0x00 => format!("c.madd(0, {rd}, {rs}, {rt});"),
        0x01 => format!("c.maddu(0, {rd}, {rs}, {rt});"),
        0x04 => format!("c.plzcw({rd}, {rs});"),
        0x08 => match sa {
            0x00 => three("paddw"),
            0x01 => three("psubw"),
            0x02 => three("pcgtw"),
            0x03 => three("pmaxw"),
            0x04 => three("paddh"),
            0x05 => three("psubh"),
            0x06 => three("pcgth"),
            0x07 => three("pmaxh"),
            0x08 => three("paddb"),
            0x09 => three("psubb"),
            0x0A => three("pcgtb"),
            0x10 => three("paddsw"),
            0x11 => three("psubsw"),
            0x12 => three("pextlw"),
            0x13 => three("ppacw"),
            0x14 => three("paddsh"),
            0x15 => three("psubsh"),
            0x16 => three("pextlh"),
            0x17 => three("ppach"),
            0x18 => three("paddsb"),
            0x19 => three("psubsb"),
            0x1A => three("pextlb"),
            0x1B => three("ppacb"),
            0x1E => two_t("pext5"),
            0x1F => two_t("ppac5"),
            _ => return None,
        },
        0x09 => match sa {
            0x00 => three("pmaddw"),
            0x02 => three("psllvw"),
            0x03 => three("psrlvw"),
            0x04 => three("pmsubw"),
            0x08 => format!("c.pmfhi({rd});"),
            0x09 => format!("c.pmflo({rd});"),
            0x0A => three("pinth"),
            0x0C => three("pmultw"),
            0x0D => format!("c.pdivw({rs}, {rt});"),
            0x0E => three("pcpyld"),
            0x10 => three("pmaddh"),
            0x11 => three("phmadh"),
            0x12 => three("pand"),
            0x13 => three("pxor"),
            0x14 => three("pmsubh"),
            0x15 => three("phmsbh"),
            0x1A => two_t("pexeh"),
            0x1B => two_t("prevh"),
            0x1C => three("pmulth"),
            0x1D => format!("c.pdivbw({rs}, {rt});"),
            0x1E => two_t("pexew"),
            0x1F => two_t("prot3w"),
            _ => return None,
        },
        0x10 => set(rd, "c.hi[1]".into()),
        0x11 => format!("c.hi[1] = {};", r64(rs)),
        0x12 => set(rd, "c.lo[1]".into()),
        0x13 => format!("c.lo[1] = {};", r64(rs)),
        0x18 => format!("c.mult(1, {rd}, {rs}, {rt});"),
        0x19 => format!("c.multu(1, {rd}, {rs}, {rt});"),
        0x1A => format!("c.div(1, {rs}, {rt});"),
        0x1B => format!("c.divu(1, {rs}, {rt});"),
        0x20 => format!("c.madd(1, {rd}, {rs}, {rt});"),
        0x21 => format!("c.maddu(1, {rd}, {rs}, {rt});"),
        0x28 => match sa {
            0x01 => two_t("pabsw"),
            0x02 => three("pceqw"),
            0x03 => three("pminw"),
            0x04 => three("padsbh"),
            0x05 => two_t("pabsh"),
            0x06 => three("pceqh"),
            0x07 => three("pminh"),
            0x0A => three("pceqb"),
            0x10 => three("padduw"),
            0x11 => three("psubuw"),
            0x12 => three("pextuw"),
            0x14 => three("padduh"),
            0x15 => three("psubuh"),
            0x16 => three("pextuh"),
            0x18 => three("paddub"),
            0x19 => three("psubub"),
            0x1A => three("pextub"),
            0x1B => three("qfsrv"),
            _ => return None,
        },
        0x29 => match sa {
            0x00 => three("pmadduw"),
            0x03 => three("psravw"),
            0x08 => format!("c.pmthi({rs});"),
            0x09 => format!("c.pmtlo({rs});"),
            0x0A => three("pinteh"),
            0x0C => three("pmultuw"),
            0x0D => format!("c.pdivuw({rs}, {rt});"),
            0x0E => three("pcpyud"),
            0x12 => three("por"),
            0x13 => three("pnor"),
            0x1A => two_t("pexch"),
            0x1B => two_t("pcpyh"),
            0x1E => two_t("pexcw"),
            _ => return None,
        },
        0x30 => format!("c.pmfhl({rd}, {sa});"),
        0x31 => format!("c.pmthl({rs}, {sa});"),
        0x34 => format!("c.psllh({rd}, {rt}, {sa});"),
        0x36 => format!("c.psrlh({rd}, {rt}, {sa});"),
        0x37 => format!("c.psrah({rd}, {rt}, {sa});"),
        0x3C => format!("c.psllw({rd}, {rt}, {sa});"),
        0x3E => format!("c.psrlw({rd}, {rt}, {sa});"),
        0x3F => format!("c.psraw({rd}, {rt}, {sa});"),
        _ => return None,
    })
}

/// Shared knowledge across all functions.
pub struct Ctx<'a> {
    /// Function start → shard index.
    pub shard_of: &'a BTreeMap<u32, usize>,
    /// Addresses referenced from data that land on instructions.
    pub data_refs: &'a BTreeSet<u32>,
    /// Extra entry points discovered at run time (`generated/entries.txt`).
    pub extra_labels: &'a BTreeSet<u32>,
    /// Host-thread mode (`ssx3_ps2::threads`): no unwinding, so calls and
    /// syscalls continue inline and return points are not labels.
    pub threads: bool,
    /// Recovered, globally unique symbol names usable as Rust identifiers.
    pub names: &'a BTreeMap<u32, String>,
    /// (Unwind form) functions the host-thread form emits structured: their
    /// cycle charges are reproduced block for block.
    pub structured: Option<&'a BTreeSet<u32>>,
}

/// Statistics for one emitted function.
#[derive(Default, Debug, Clone, Copy)]
pub struct Stats {
    pub instructions: usize,
    pub invalid: usize,
    pub labels: usize,
}

/// Emit one function. `shard` is the index of the crate it is emitted into.
pub fn emit_function(
    img: &dyn Image,
    f: &Func,
    shard: usize,
    ctx: &Ctx,
    out: &mut String,
) -> Stats {
    emit_function_named(img, f, shard, ctx, out, &fn_name(f.start))
}

/// `emit_function` with an explicit Rust name (dispatch twins of structured functions).
pub fn emit_function_named(
    img: &dyn Image,
    f: &Func,
    shard: usize,
    ctx: &Ctx,
    out: &mut String,
    name: &str,
) -> Stats {
    let mut st = Stats::default();
    // Instruction list in address order.
    let mut insns: Vec<(u32, u32)> = Vec::new();
    for &(lo, hi) in &f.runs {
        let mut a = lo;
        while a < hi {
            insns.push((a, img.word(a).unwrap_or(0)));
            a += 4;
        }
    }
    st.instructions = insns.len();
    let is_insn: BTreeSet<u32> = insns.iter().map(|(a, _)| *a).collect();
    let index: BTreeMap<u32, usize> = insns
        .iter()
        .enumerate()
        .map(|(i, (a, _))| (*a, i))
        .collect();
    let inside = |a: u32| a >= f.start && a < f.end && is_insn.contains(&a);

    // Labels.
    let mut labels: BTreeSet<u32> = BTreeSet::new();
    labels.insert(f.start);
    // Return points of calls and syscalls (unwind form): their cycles are
    // charged with the call's block, as the host-thread form charges them
    // (a call does not end a basic block there), unless something else
    // also makes them a label.
    let mut cont_labels: BTreeSet<u32> = BTreeSet::new();
    // How the host-thread form emitted this function (its block starts are
    // where its cycle charges split): structured (`structure.rs`) or this
    // emitter's dispatch form.
    let as_structured = ctx.structured.is_some_and(|s| s.contains(&f.start));
    let has_switch = insns
        .iter()
        .any(|&(pc, w)| matches!(classify(pc, w), Ctl::JumpReg { rs } if rs != 31));
    for &(pc, w) in &insns {
        match classify(pc, w) {
            Ctl::Branch { target, link, .. } => {
                if inside(target) && !(as_structured && link) {
                    labels.insert(target);
                }
                if inside(pc + 8) {
                    labels.insert(pc + 8);
                }
            }
            Ctl::Jump { target } => {
                if inside(target) {
                    labels.insert(target);
                }
            }
            Ctl::Call { .. } | Ctl::CallReg { .. } if !ctx.threads => {
                if inside(pc + 8) {
                    cont_labels.insert(pc + 8);
                }
            }
            Ctl::Syscall if !ctx.threads => {
                if inside(pc + 4) {
                    cont_labels.insert(pc + 4);
                }
            }
            _ => {}
        }
    }
    // Data references: block starts of the structured form only as jump-table
    // targets (functions with a computed jump).
    let refs: Vec<u32> = ctx
        .data_refs
        .range(f.start..f.end)
        .copied()
        .filter(|a| inside(*a))
        .collect();
    if !as_structured || has_switch {
        labels.extend(refs.iter().copied());
    }
    if !as_structured {
        labels.extend(
            ctx.extra_labels
                .range(f.start..f.end)
                .copied()
                .filter(|a| inside(*a) && !cont_labels.contains(a)),
        );
    }
    // Extra entries (resume points, entries from elsewhere) and other data
    // references of a structured function: labels its main body does not
    // split its cycle charge at (its `__entries` twin charges an entry from
    // outside; see the entry charge below).
    let extra_only: BTreeSet<u32> = ctx
        .extra_labels
        .range(f.start..f.end)
        .copied()
        .chain(refs.iter().copied())
        .filter(|a| inside(*a) && !labels.contains(a) && !cont_labels.contains(a))
        .collect();
    let cont_only: BTreeSet<u32> = cont_labels.difference(&labels).copied().collect();
    let pass: BTreeSet<u32> = cont_only.union(&extra_only).copied().collect();
    labels.extend(cont_labels.iter().copied());
    labels.extend(extra_only.iter().copied());
    st.labels = labels.len();
    let mut raw_count: BTreeMap<u32, usize> = BTreeMap::new();
    let mut cont_edge: BTreeMap<u32, u32> = BTreeMap::new();
    let fn_text_start = out.len();

    if let Some(sym) = &f.name {
        let _ = writeln!(out, "/// `{sym}`");
    }
    let _ = writeln!(out, "pub fn {name}(c: &mut Cpu) {{");
    if labels.len() == 1 && simple_straight(&insns) {
        // Fast path: a single-entry function with no internal labels.
        let _ = writeln!(out, "    let mut pc: u32 = {:#010x};", f.start);
        let _ = writeln!(out, "    let _ = &mut pc;");
    } else {
        let _ = writeln!(out, "    let mut pc: u32 = c.pc;");
        let _ = writeln!(out, "@@ENTRY@@");
    }
    let _ = writeln!(out, "    loop {{");
    let _ = writeln!(out, "        match pc {{");
    let label_list: Vec<u32> = labels.iter().copied().collect();
    for (li, &l) in label_list.iter().enumerate() {
        let next_label = label_list.get(li + 1).copied();
        let Some(&start_i) = index.get(&l) else {
            continue;
        };
        let _ = writeln!(out, "            {l:#010x} => {{");
        let mut body = String::new();
        let mut count = 0usize;
        let mut i = start_i;
        let mut terminated = false;
        while i < insns.len() {
            let (pc, w) = insns[i];
            if pc != l && Some(pc) == next_label.filter(|n| *n <= pc) {
                // Fall into the next label.
                cont_edge.insert(l, pc);
                let _ = writeln!(body, "                pc = {pc:#010x}; continue;");
                terminated = true;
                break;
            }
            if pc != l && labels.contains(&pc) {
                cont_edge.insert(l, pc);
                let _ = writeln!(body, "                pc = {pc:#010x}; continue;");
                terminated = true;
                break;
            }
            // Gap in the runs: the next instruction isn't contiguous.
            if i > start_i && insns[i - 1].0 + 4 != pc {
                let _ = writeln!(body, "                pc = {pc:#010x}; continue;");
                terminated = true;
                break;
            }
            count += 1;
            let ctl = classify(pc, w);
            if ctl == Ctl::None {
                match lower(pc, w) {
                    Some(s) => {
                        if !s.is_empty() {
                            let _ = writeln!(body, "                {s}");
                        }
                    }
                    None => {
                        st.invalid += 1;
                        let _ = writeln!(
                            body,
                            "                c.fatal({pc:#010x}, \"invalid instruction {w:#010x}\"); return;"
                        );
                        terminated = true;
                        break;
                    }
                }
                i += 1;
                continue;
            }
            // Control transfer: include the delay slot where applicable.
            let delay = insns
                .get(i + 1)
                .filter(|(a, _)| *a == pc + 4)
                .map(|(_, dw)| *dw);
            let delay_code = |st: &mut Stats| -> String {
                match delay {
                    Some(dw) if classify(pc + 4, dw) == Ctl::None => match lower(pc + 4, dw) {
                        Some(s) => s,
                        None => {
                            st.invalid += 1;
                            String::new()
                        }
                    },
                    _ => String::new(),
                }
            };
            if delay.is_some() && !matches!(ctl, Ctl::Syscall | Ctl::Stop) {
                count += 1;
            }
            let goto = |target: u32, back: bool| -> String {
                if inside(target) && labels.contains(&target) {
                    if back || target <= pc {
                        format!(
                            "if c.cycles >= c.next_event {{ c.pc = {target:#010x}; if c.event() {{ return; }} }} pc = {target:#010x}; continue;"
                        )
                    } else {
                        format!("pc = {target:#010x}; continue;")
                    }
                } else {
                    // Branch/jump out of this function: a tail jump.
                    tail(target, shard, ctx)
                }
            };
            if ctx.threads && matches!(ctl, Ctl::Call { .. } | Ctl::CallReg { .. } | Ctl::Syscall) {
                let ret = if ctl == Ctl::Syscall { pc + 4 } else { pc + 8 };
                match ctl {
                    Ctl::Call { target } => {
                        let ds = delay_code(&mut st);
                        let callee = direct_callee(target, shard, ctx);
                        let _ = writeln!(body, "                c.g[31][0] = sx({ret:#010x});");
                        if !ds.is_empty() {
                            let _ = writeln!(body, "                {ds}");
                        }
                        let _ = writeln!(
                            body,
                            "                c.pc = {target:#010x}; {callee}; if c.pc != {ret:#010x} {{ return; }}"
                        );
                    }
                    Ctl::CallReg { rd, rs } => {
                        let ds = delay_code(&mut st);
                        let _ = writeln!(body, "                let t = c.g[{rs}][0] as u32;");
                        if rd != 0 {
                            let _ =
                                writeln!(body, "                c.g[{rd}][0] = sx({ret:#010x});");
                        }
                        if !ds.is_empty() {
                            let _ = writeln!(body, "                {ds}");
                        }
                        let _ = writeln!(
                            body,
                            "                c.pc = t; call_indirect(c, t); if c.pc != {ret:#010x} {{ return; }}"
                        );
                    }
                    _ => {
                        let _ = writeln!(
                            body,
                            "                c.pc = {ret:#010x}; c.syscall({pc:#010x}); if c.pc != {ret:#010x} {{ return; }}"
                        );
                        let _ = writeln!(
                            body,
                            "                if c.cycles >= c.next_event {{ c.event(); }}"
                        );
                    }
                }
                if !inside(ret) {
                    let _ = writeln!(body, "                {}", tail(ret, shard, ctx));
                    terminated = true;
                    break;
                }
                i = index[&ret];
                continue;
            }
            match ctl {
                Ctl::Branch {
                    target,
                    likely,
                    link,
                } => {
                    let cond = branch_cond(w);
                    let ds = delay_code(&mut st);
                    let link_s = if link {
                        format!("c.g[31][0] = sx({:#010x});", pc + 8)
                    } else {
                        String::new()
                    };
                    let fall = if inside(pc + 8) {
                        format!("pc = {:#010x}; continue;", pc + 8)
                    } else {
                        tail(pc + 8, shard, ctx)
                    };
                    let take = if link {
                        call_code(target, pc + 8, shard, ctx, &labels, f)
                    } else {
                        goto(target, false)
                    };
                    if likely {
                        let _ = writeln!(body, "                {link_s}");
                        let _ = writeln!(body, "                if {cond} {{ {ds} {take} }}");
                    } else {
                        let _ = writeln!(body, "                let t = {cond}; {link_s}");
                        let _ = writeln!(body, "                {ds}");
                        let _ = writeln!(body, "                if t {{ {take} }}");
                    }
                    let _ = writeln!(body, "                {fall}");
                }
                Ctl::Jump { target } => {
                    let ds = delay_code(&mut st);
                    let _ = writeln!(body, "                {ds}");
                    let _ = writeln!(body, "                {}", goto(target, false));
                }
                Ctl::Call { target } => {
                    let ds = delay_code(&mut st);
                    let _ = writeln!(body, "                c.g[31][0] = sx({:#010x});", pc + 8);
                    let _ = writeln!(body, "                {ds}");
                    let _ = writeln!(
                        body,
                        "                {}",
                        call_code(target, pc + 8, shard, ctx, &labels, f)
                    );
                    if inside(pc + 8) {
                        cont_edge.insert(l, pc + 8);
                        let _ = writeln!(body, "                pc = {:#010x}; continue;", pc + 8);
                    } else {
                        let _ = writeln!(body, "                return;");
                    }
                }
                Ctl::JumpReg { rs } => {
                    let ds = delay_code(&mut st);
                    let _ = writeln!(body, "                let t = c.g[{rs}][0] as u32;");
                    let _ = writeln!(body, "                {ds}");
                    if rs == 31 {
                        let _ = writeln!(body, "                c.pc = t; return;");
                    } else {
                        // Jump table (target is a label here) or tail jump.
                        let _ = writeln!(body, "                pc = t; continue;");
                    }
                }
                Ctl::CallReg { rd, rs } => {
                    let ds = delay_code(&mut st);
                    let _ = writeln!(body, "                let t = c.g[{rs}][0] as u32;");
                    if rd != 0 {
                        let _ =
                            writeln!(body, "                c.g[{rd}][0] = sx({:#010x});", pc + 8);
                    }
                    let _ = writeln!(body, "                {ds}");
                    let _ = writeln!(
                        body,
                        "                if !c.enter(t) {{ return; }} call_indirect(c, t); c.leave(); if c.pc != {:#010x} {{ return; }}",
                        pc + 8
                    );
                    if inside(pc + 8) {
                        cont_edge.insert(l, pc + 8);
                        let _ = writeln!(body, "                pc = {:#010x}; continue;", pc + 8);
                    } else {
                        let _ = writeln!(body, "                return;");
                    }
                }
                Ctl::Syscall => {
                    let _ = writeln!(
                        body,
                        "                c.pc = {:#010x}; c.syscall({pc:#010x}); if c.pc != {:#010x} {{ return; }}",
                        pc + 4,
                        pc + 4
                    );
                    let _ = writeln!(
                        body,
                        "                if c.cycles >= c.next_event && c.event() {{ return; }}"
                    );
                    if inside(pc + 4) {
                        cont_edge.insert(l, pc + 4);
                        let _ = writeln!(body, "                pc = {:#010x}; continue;", pc + 4);
                    } else {
                        let _ = writeln!(body, "                return;");
                    }
                }
                Ctl::Stop => {
                    if (w >> 26) == 0x10 {
                        let _ = writeln!(body, "                c.pc = c.cop0[14]; return;");
                    } else {
                        let _ = writeln!(
                            body,
                            "                c.brk({pc:#010x}, {}); return;",
                            (w >> 6) & 0xFFFFF
                        );
                    }
                }
                Ctl::None => unreachable!(),
            }
            terminated = true;
            break;
        }
        if !terminated {
            // Ran off the end of the function: continue into whatever follows.
            let after = insns.last().map(|(a, _)| a + 4).unwrap_or(f.end);
            let _ = writeln!(body, "                {}", tail(after, shard, ctx));
        }
        raw_count.insert(l, count);
        let _ = writeln!(out, "                c.cycles += @@CYC{l:08x}@@;");
        out.push_str(&body);
        let _ = writeln!(out, "            }}");
    }
    // Cycle charges: a block ending in a call also pays for the
    // continuation-only blocks after it; those pay nothing themselves.
    // Sum the pass-through blocks from `x` on (`through`: which ones).
    let chain = |mut x: u32, through: &BTreeSet<u32>| -> usize {
        let mut n = 0;
        let mut guard = 0;
        while through.contains(&x) && guard < 100_000 {
            guard += 1;
            n += raw_count.get(&x).copied().unwrap_or(0);
            match cont_edge.get(&x) {
                Some(&y) => x = y,
                None => break,
            }
        }
        n
    };
    let mut text = out.split_off(fn_text_start);
    // An entry from outside at an extra entry pays for its block (up to the
    // next label of the twin: calls do not end it there either).
    let mut entry = String::new();
    for &e in &extra_only {
        if let Some(&n) = raw_count.get(&e) {
            let charge = n + cont_edge.get(&e).map_or(0, |&y| chain(y, &cont_only));
            if charge > 0 {
                let _ = writeln!(entry, "        {e:#010x} => c.cycles += {charge},");
            }
        }
    }
    let entry = if entry.is_empty() {
        String::new()
    } else {
        format!("    match pc {{\n{entry}        _ => {{}}\n    }}\n")
    };
    text = text.replacen("@@ENTRY@@\n", &entry, 1);
    for (&l, &n) in &raw_count {
        let charge = if pass.contains(&l) {
            0
        } else {
            n + cont_edge.get(&l).map_or(0, |&y| chain(y, &pass))
        };
        let ph = format!("                c.cycles += @@CYC{l:08x}@@;\n");
        let rep = if charge == 0 {
            String::new()
        } else {
            format!("                c.cycles += {charge};\n")
        };
        text = text.replacen(&ph, &rep, 1);
    }
    out.push_str(&text);
    let _ = writeln!(
        out,
        "            _ => {{ c.jump_out(pc, {:#010x}, {:#010x}); return; }}",
        f.start, f.end
    );
    let _ = writeln!(out, "        }}");
    let _ = writeln!(out, "    }}");
    let _ = writeln!(out, "}}");
    st
}

fn simple_straight(_insns: &[(u32, u32)]) -> bool {
    false
}

/// Code for a tail jump to `target` outside the current function.
fn tail(target: u32, shard: usize, ctx: &Ctx) -> String {
    let callee = direct_callee(target, shard, ctx);
    if ctx.threads {
        return format!("c.pc = {target:#010x}; {callee}; return;");
    }
    format!("if !c.enter({target:#010x}) {{ return; }} {callee}; c.leave(); return;")
}

fn direct_callee(target: u32, shard: usize, ctx: &Ctx) -> String {
    match ctx.shard_of.get(&target) {
        Some(&s) if s == shard => match ctx.names.get(&target) {
            Some(n) => format!("{n}(c)"),
            None => format!("{}(c)", fn_name(target)),
        },
        _ => format!("call_indirect(c, {target:#010x})"),
    }
}

/// Code for a call to `target` returning to `ret`.
fn call_code(
    target: u32,
    ret: u32,
    shard: usize,
    ctx: &Ctx,
    _labels: &BTreeSet<u32>,
    _f: &Func,
) -> String {
    let callee = direct_callee(target, shard, ctx);
    if ctx.threads {
        return format!("c.pc = {target:#010x}; {callee}; if c.pc != {ret:#010x} {{ return; }}");
    }
    format!(
        "if !c.enter({target:#010x}) {{ return; }} {callee}; c.leave(); if c.pc != {ret:#010x} {{ return; }}"
    )
}

/// Extend a function's instruction runs by recursive descent.
///
/// Ghidra stops disassembling after calls it believes never return (asserts,
/// error handlers) and after some computed jumps, but at run time execution
/// does continue there. Starting from every return point and branch target that
/// lies outside the known runs (and below `limit`, the next function's start),
/// follow fallthrough and branches until an unconditional transfer.
pub fn extend_runs(img: &dyn Image, f: &mut Func, limit: u32, seeds: &BTreeSet<u32>) -> usize {
    let mut have: BTreeSet<u32> = BTreeSet::new();
    for &(lo, hi) in &f.runs {
        let mut a = lo;
        while a < hi {
            have.insert(a);
            a += 4;
        }
    }
    let mut work: Vec<u32> = Vec::new();
    if !have.contains(&f.start) {
        work.push(f.start);
    }
    for &a in seeds.range(f.start..limit) {
        if !have.contains(&a) {
            work.push(a);
        }
    }
    let seed = |pc: u32, w: u32, work: &mut Vec<u32>| match classify(pc, w) {
        Ctl::Branch { target, .. } => {
            work.push(target);
            work.push(pc + 8);
        }
        Ctl::Call { .. } | Ctl::CallReg { .. } => work.push(pc + 8),
        Ctl::Syscall => work.push(pc + 4),
        _ => {}
    };
    for &a in &have {
        if let Some(w) = img.word(a) {
            seed(a, w, &mut work);
        }
    }
    let mut added = 0;
    while let Some(start) = work.pop() {
        let mut a = start;
        loop {
            if a < f.start || a >= limit || have.contains(&a) {
                break;
            }
            let Some(w) = img.word(a) else { break };
            if !is_lowerable(a, w) {
                break;
            }
            have.insert(a);
            added += 1;
            let ctl = classify(a, w);
            seed(a, w, &mut work);
            match ctl {
                Ctl::Jump { .. } | Ctl::JumpReg { .. } => {
                    // Include the delay slot, then stop.
                    if a + 4 < limit && !have.contains(&(a + 4)) {
                        if let Some(dw) = img.word(a + 4) {
                            if is_lowerable(a + 4, dw) {
                                have.insert(a + 4);
                                added += 1;
                            }
                        }
                    }
                    break;
                }
                Ctl::Stop => break,
                _ => a += 4,
            }
        }
    }
    if added > 0 {
        // Rebuild contiguous runs.
        let mut runs: Vec<(u32, u32)> = Vec::new();
        for a in have {
            match runs.last_mut() {
                Some(r) if r.1 == a => r.1 = a + 4,
                _ => runs.push((a, a + 4)),
            }
        }
        f.end = f.end.max(runs.last().map(|r| r.1).unwrap_or(f.end));
        f.runs = runs;
    }
    added
}

/// Every static control-flow target in `f`: branch and jump targets and
/// return points. Used to make cross-function targets labels of whichever
/// function contains them.
pub fn targets(img: &dyn Image, f: &Func, threads: bool) -> Vec<u32> {
    let mut out = Vec::new();
    for &(lo, hi) in &f.runs {
        let mut a = lo;
        while a < hi {
            if let Some(w) = img.word(a) {
                match classify(a, w) {
                    Ctl::Branch { target, .. } => {
                        out.push(target);
                        out.push(a + 8);
                    }
                    Ctl::Jump { target } => {
                        out.push(target);
                        out.push(a + 8);
                    }
                    Ctl::Call { target } => {
                        out.push(target);
                        if !threads {
                            out.push(a + 8);
                        }
                    }
                    Ctl::CallReg { .. } if !threads => out.push(a + 8),
                    Ctl::Syscall if !threads => out.push(a + 4),
                    _ => {}
                }
            }
            a += 4;
        }
    }
    out
}

/// Validate lowering coverage over a word stream (used by tests and stats).
pub fn is_lowerable(pc: u32, w: u32) -> bool {
    classify(pc, w) != Ctl::None || lower(pc, w).is_some()
}

// Helpers shared with `structure.rs`.
pub fn classify_pub(pc: u32, w: u32) -> Ctl {
    classify(pc, w)
}
pub fn branch_cond_pub(w: u32) -> String {
    branch_cond(w)
}
pub fn direct_callee_pub(target: u32, shard: usize, ctx: &Ctx) -> String {
    direct_callee(target, shard, ctx)
}
pub fn tail_pub(target: u32, shard: usize, ctx: &Ctx) -> String {
    tail(target, shard, ctx)
}

const ABI: [&str; 32] = [
    "ZERO", "AT", "V0", "V1", "A0", "A1", "A2", "A3", "T0", "T1", "T2", "T3", "T4", "T5", "T6",
    "T7", "S0", "S1", "S2", "S3", "S4", "S5", "S6", "S7", "T8", "T9", "K0", "K1", "GP", "SP", "FP",
    "RA",
];

/// Readability pass over emitted code: `c.g[29]` → `c.g[SP]`, and
/// `wrapping_add(<negative hex>)` → `wrapping_sub(<magnitude>)`.
pub fn prettify(code: &str) -> String {
    let b = code.as_bytes();
    let mut out = String::with_capacity(code.len());
    let mut i = 0;
    while i < b.len() {
        if code[i..].starts_with("c.g[") {
            let start = i + 4;
            let mut j = start;
            while j < b.len() && b[j].is_ascii_digit() {
                j += 1;
            }
            if j > start && j < b.len() && b[j] == b']' {
                let n: usize = code[start..j].parse().unwrap_or(99);
                if n < 32 {
                    out.push_str("c.g[");
                    out.push_str(ABI[n]);
                    out.push(']');
                    i = j + 1;
                    continue;
                }
            }
        }
        if code[i..].starts_with(".wrapping_add(0x") {
            let start = i + ".wrapping_add(0x".len();
            let mut j = start;
            while j < b.len() && b[j].is_ascii_hexdigit() {
                j += 1;
            }
            let digits = &code[start..j];
            if j < b.len() && b[j] == b')' && (digits.len() == 8 || digits.len() == 16) {
                if let Ok(v) = u64::from_str_radix(digits, 16) {
                    let neg = if digits.len() == 8 {
                        (v as u32 >= 0x8000_0000).then(|| (v as u32).wrapping_neg() as u64)
                    } else {
                        (v >= 0x8000_0000_0000_0000).then(|| v.wrapping_neg())
                    };
                    if let Some(m) = neg {
                        out.push_str(&format!(".wrapping_sub({m:#x})"));
                        i = j + 1;
                        continue;
                    }
                }
            }
        }
        let ch = code[i..].chars().next().unwrap();
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}
