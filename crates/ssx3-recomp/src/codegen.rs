//! R5900 → Rust code generation.
//!
//! A translated function is a Rust function over the runtime state. Syscalls are
//! constant-folded into direct calls to the native SDK ([`crate::sdk`] in the
//! runtime crate), and direct `jal`s become direct Rust calls to the callee.
//!
//! Control flow within a function is currently expressed as a per-function basic
//! block dispatcher. That is deliberate scaffolding: recovering structured Rust
//! control flow (loops, `if`/`else`, no labels) from the CFG is the next stage,
//! and the CFG this uses is already correct (authoritative boundaries + branches).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use crate::program::Program;
use crate::r5900::{Instruction, Operands};
use crate::symbols::Function;

/// Rust identifier for a translated function.
pub fn fn_name(addr: u32) -> String {
    format!("f_{addr:08x}")
}

/// `(syscall number, SDK function, argument count, returns a value)`.
const SYSCALLS: &[(u32, &str, usize, bool)] = &[
    (0x01, "reset_ee", 1, false),
    (0x02, "set_gs_crt", 3, false),
    (0x3c, "init_main_thread", 4, true),
    (0x3d, "init_heap", 2, true),
    (0x3e, "end_of_heap", 0, true),
    (0x3f, "get_heap_end", 0, true),
    (0x64, "flush_cache", 1, false),
    (0x70, "gs_get_imr", 0, true),
    (0x71, "gs_put_imr", 1, false),
    (0x74, "set_syscall", 2, false),
    (0x83, "find_address", 1, true),
    (0x85, "set_memory_mode", 1, true),
    (0xfc, "kprintf", 1, false),
];

fn is_control(m: &str) -> bool {
    matches!(
        m,
        "beq" | "bne" | "beql" | "bnel"
            | "blez" | "bgtz" | "blezl" | "bgtzl"
            | "bltz" | "bgez" | "bltzl" | "bgezl"
            | "bltzal" | "bgezal" | "bltzall" | "bgezall"
            | "bc1f" | "bc1t" | "j" | "jal" | "jr" | "jalr" | "eret"
    )
}

/// Destination general-purpose register written by an instruction, if any.
fn dest_reg(inst: &Instruction) -> Option<u8> {
    use Operands::*;
    let m = inst.mnemonic;
    match inst.operands {
        RdRsRt { rd, .. } | RdRtSa { rd, .. } | RdRtRs { rd, .. } => Some(rd),
        Rd { rd } => Some(rd),
        RdRs { rd, .. } => Some(rd),
        RtRsImm { rt, .. } | RtRsImmU { rt, .. } | RtImm { rt, .. } => Some(rt),
        Mem { rt, .. } if m.starts_with('l') => Some(rt),
        CopMove { rt, .. } if m.starts_with('m') => Some(rt), // mfc0/mfc1
        Mmi { rd, .. } => match m {
            "mfhi1" | "mflo1" | "clear" => Some(rd),
            _ => Option::None,
        },
        Jump { .. } if m == "jal" => Some(31),
        Jump { .. } if m == "jalr" => Some(31),
        _ => Option::None,
    }
}

/// Constant written to a register by an immediate/lui instruction, if any.
fn const_value(inst: &Instruction) -> Option<u64> {
    use Operands::*;
    match (inst.mnemonic, inst.operands) {
        ("addiu" | "addi" | "daddiu" | "daddi", RtRsImm { rs: 0, imm, .. }) => {
            Some(imm as i64 as u64)
        }
        ("ori", RtRsImmU { rs: 0, imm, .. }) => Some(imm as u64),
        ("lui", RtImm { imm, .. }) => Some((imm as u64) << 16),
        _ => Option::None,
    }
}

struct Emit<'a> {
    out: String,
    translated: &'a BTreeSet<u32>,
    indent: usize,
    /// Constant currently in `$v1`, tracked along a straight-line region.
    v1: Option<u64>,
    /// True when emitting the basic-block dispatcher (which uses a `pc` variable).
    dispatcher: bool,
}

impl<'a> Emit<'a> {
    fn line(&mut self, pc: u32, s: &str) {
        for _ in 0..self.indent {
            self.out.push_str("    ");
        }
        let _ = writeln!(self.out, "{}", s.replace("$PC", &format!("{pc:#010x}")));
    }

    fn lower(&mut self, pc: u32, inst: &Instruction) -> bool {
        use Operands::*;
        let t = format!("t_{pc:08x}");
        let m = inst.mnemonic;
        match (m, inst.operands) {
            ("clear" | "move", Rd { rd }) => {
                self.line(pc, &format!("c.set_reg({rd}, 0);"));
                true
            }
            ("clear", Mmi { rd, .. }) => {
                self.line(pc, &format!("c.set_reg({rd}, 0);"));
                true
            }
            ("moveq", RdRs { rd, rs }) => {
                self.line(pc, &format!("c.set_reg({rd}, c.reg({rs}));"));
                true
            }
            ("mthi", Rs { rs }) => {
                self.line(pc, &format!("c.hi = c.reg({rs});"));
                true
            }
            ("mtlo", Rs { rs }) => {
                self.line(pc, &format!("c.lo = c.reg({rs});"));
                true
            }
            ("mfhi", Rd { rd }) => {
                self.line(pc, &format!("c.set_reg({rd}, c.hi);"));
                true
            }
            ("mflo", Rd { rd }) => {
                self.line(pc, &format!("c.set_reg({rd}, c.lo);"));
                true
            }
            // MMI hi/lo forms carry Operands::Mmi.
            ("mthi1", Mmi { rs, .. }) => {
                self.line(pc, &format!("c.hi = c.reg({rs});"));
                true
            }
            ("mtlo1", Mmi { rs, .. }) => {
                self.line(pc, &format!("c.lo = c.reg({rs});"));
                true
            }
            ("mfhi1", Mmi { rd, .. }) => {
                self.line(pc, &format!("c.set_reg({rd}, c.hi);"));
                true
            }
            ("mflo1", Mmi { rd, .. }) => {
                self.line(pc, &format!("c.set_reg({rd}, c.lo);"));
                true
            }
            ("sync" | "nop" | "ei" | "di" | "cache" | "pref", _) => true,
            // Clearing ops for state we do not model (SA field, FPU registers).
            ("mtsab", _) | ("mtsah", _) | ("clear_f", _) | ("clear_fcsr", _) => true,
            // COP1 data moves.
            ("mtc1", CopMove { rt, rd, .. }) => {
                self.line(pc, &format!("c.set_f32({rd}, f32::from_bits(c.reg({rt}) as u32));"));
                true
            }
            ("mfc1", CopMove { rt, rd, .. }) => {
                self.line(pc, &format!("c.set_reg({rt}, (c.f32({rd}).to_bits() as i32) as i64 as u64);"));
                true
            }
            ("cfc1", CopMove { rt, .. }) => {
                self.line(pc, &format!("c.set_reg({rt}, c.fcr31_cc as u64);"));
                true
            }
            ("ctc1", _) => true,
            // COP1 arithmetic and compares.
            (_, Cop1Arith { .. }) | (_, Cop1Cmp { .. }) => self.emit_cop1(pc, inst),

            ("addiu" | "addi" | "daddiu" | "daddi", RtRsImm { rt, rs, imm }) => {
                self.line(
                    pc,
                    &format!("c.set_reg({rt}, (c.reg({rs}) as i64).wrapping_add({imm}i64) as u64);"),
                );
                true
            }
            ("slti" | "sltiu", RtRsImm { rt, rs, imm }) => {
                let cast = if m == "slti" { "i64" } else { "u64" };
                self.line(
                    pc,
                    &format!("c.set_reg({rt}, ((c.reg({rs}) as {cast}) < ({imm} as {cast})) as u64);"),
                );
                true
            }
            ("andi" | "ori" | "xori", RtRsImmU { rt, rs, imm }) => {
                let op = match m {
                    "andi" => "&",
                    "ori" => "|",
                    _ => "^",
                };
                self.line(pc, &format!("c.set_reg({rt}, c.reg({rs}) {op} ({imm}u64));"));
                true
            }
            ("lui", RtImm { rt, imm }) => {
                self.line(pc, &format!("c.set_reg({rt}, ({imm}u64) << 16);"));
                true
            }

            ("addu" | "add" | "subu" | "sub" | "daddu" | "dadd" | "dsubu" | "dsub" | "and" | "or"
            | "xor" | "nor", RdRsRt { rd, rs, rt }) => {
                let expr = match m {
                    "addu" | "add" | "daddu" | "dadd" => {
                        format!("c.reg({rs}).wrapping_add(c.reg({rt}))")
                    }
                    "subu" | "sub" | "dsubu" | "dsub" => {
                        format!("c.reg({rs}).wrapping_sub(c.reg({rt}))")
                    }
                    "and" => format!("c.reg({rs}) & c.reg({rt})"),
                    "or" => format!("c.reg({rs}) | c.reg({rt})"),
                    "xor" => format!("c.reg({rs}) ^ c.reg({rt})"),
                    _ => format!("!(c.reg({rs}) | c.reg({rt}))"),
                };
                self.line(pc, &format!("let {t} = {expr};"));
                self.line(pc, &format!("c.set_reg({rd}, {t});"));
                true
            }
            ("slt", RdRsRt { rd, rs, rt }) => {
                self.line(
                    pc,
                    &format!("c.set_reg({rd}, ((c.reg({rs}) as i64) < (c.reg({rt}) as i64)) as u64);"),
                );
                true
            }
            ("sltu", RdRsRt { rd, rs, rt }) => {
                self.line(pc, &format!("c.set_reg({rd}, (c.reg({rs}) < c.reg({rt})) as u64);"));
                true
            }
            ("movz" | "movn", RdRsRt { rd, rs, rt }) => {
                let cond = if m == "movz" { "==" } else { "!=" };
                self.line(
                    pc,
                    &format!("if c.reg({rt}) {cond} 0 {{ c.set_reg({rd}, c.reg({rs})); }}"),
                );
                true
            }

            ("sll" | "srl" | "sra" | "dsll" | "dsrl" | "dsra" | "dsll32" | "dsrl32" | "dsra32",
                RdRtSa { rd, rt, sa }) => {
                let (base, extra) = match m {
                    "dsll32" => ("dsll", 32),
                    "dsrl32" => ("dsrl", 32),
                    "dsra32" => ("dsra", 32),
                    other => (other, 0),
                };
                let sh = format!("({sa} + {extra}) & 63");
                let expr = match base {
                    "sll" => format!("c.reg({rt}) << {sh}"),
                    "srl" => format!("c.reg({rt}) >> {sh}"),
                    _ => format!("((c.reg({rt}) as i64) >> {sh}) as u64"),
                };
                self.line(pc, &format!("let {t} = {expr};"));
                self.line(pc, &format!("c.set_reg({rd}, {t});"));
                true
            }
            ("sllv" | "srlv" | "srav" | "dsllv" | "dsrlv" | "dsrav", RdRtRs { rd, rt, rs }) => {
                let expr = match m {
                    "sllv" | "dsllv" => format!("c.reg({rt}) << (c.reg({rs}) & 63)"),
                    "srlv" | "dsrlv" => format!("c.reg({rt}) >> (c.reg({rs}) & 63)"),
                    _ => format!("((c.reg({rt}) as i64) >> (c.reg({rs}) & 63)) as u64"),
                };
                self.line(pc, &format!("let {t} = {expr};"));
                self.line(pc, &format!("c.set_reg({rd}, {t});"));
                true
            }

            ("mult" | "multu" | "mult1" | "multu1", RsRt { rs, rt }) => {
                self.line(pc, &format!("let {t} = (c.reg({rs}) as u128) * (c.reg({rt}) as u128);"));
                self.line(pc, &format!("c.lo = {t} as u64; c.hi = ({t} >> 64) as u64;"));
                true
            }
            ("div" | "divu" | "div1" | "divu1", RsRt { rs, rt }) => {
                self.line(pc, &format!("let {t} = c.reg({rt});"));
                self.line(
                    pc,
                    &format!("if {t} != 0 {{ c.lo = c.reg({rs}).wrapping_div({t}); c.hi = c.reg({rs}) % {t}; }}"),
                );
                true
            }
            ("madd" | "maddu" | "madd1" | "maddu1", RsRt { rs, rt }) => {
                self.line(pc, &format!("let {t} = (c.reg({rs}) as u128) * (c.reg({rt}) as u128);"));
                self.line(pc, &format!("let acc = ((c.hi as u128) << 64) | c.lo as u128; let acc = acc.wrapping_add({t});"));
                self.line(pc, "c.lo = acc as u64; c.hi = (acc >> 64) as u64;");
                true
            }

            ("lb" | "lbu" | "lh" | "lhu" | "lw" | "lwu" | "ld" | "sb" | "sh" | "sw" | "sd",
                Mem { rt, rs, offset }) => {                self.line(
                    pc,
                    &format!("let a_{pc:08x} = (c.reg({rs}) as i64).wrapping_add({offset}i64) as u32;"),
                );
                match m {
                    "lb" => self.line(pc, &format!("let {t} = c.load_i8(a_{pc:08x}); c.set_reg({rt}, {t});")),
                    "lbu" => self.line(pc, &format!("let {t} = c.load_u8(a_{pc:08x}); c.set_reg({rt}, {t});")),
                    "lh" => self.line(pc, &format!("let {t} = c.load_i16(a_{pc:08x}); c.set_reg({rt}, {t});")),
                    "lhu" => self.line(pc, &format!("let {t} = c.load_u16(a_{pc:08x}); c.set_reg({rt}, {t});")),
                    "lw" => self.line(pc, &format!("let {t} = c.load_i32(a_{pc:08x}); c.set_reg({rt}, {t});")),
                    "lwu" => self.line(pc, &format!("let {t} = c.load_u32(a_{pc:08x}); c.set_reg({rt}, {t});")),
                    "ld" => self.line(pc, &format!("let {t} = c.load_u64(a_{pc:08x}); c.set_reg({rt}, {t});")),
                    "sb" => self.line(pc, &format!("let {t} = c.reg({rt}); c.store_u8(a_{pc:08x}, {t});")),
                    "sh" => self.line(pc, &format!("let {t} = c.reg({rt}); c.store_u16(a_{pc:08x}, {t});")),
                    "sw" => self.line(pc, &format!("let {t} = c.reg({rt}); c.store_u32(a_{pc:08x}, {t});")),
                    _ => self.line(pc, &format!("let {t} = c.reg({rt}); c.store_u64(a_{pc:08x}, {t});")),
                }
                true
            }

            // 128-bit pair loads/stores: approximated to the low 64 bits (see
            // docs/investigation/codegen.md).
            ("lq", Mem { rt, rs, offset }) => {
                self.line(
                    pc,
                    &format!("let a_{pc:08x} = (c.reg({rs}) as i64).wrapping_add({offset}i64) as u32;"),
                );
                self.line(pc, &format!("let {t} = c.load_u64(a_{pc:08x}); c.set_reg({rt}, {t});"));
                true
            }
            ("sq", Mem { rt, rs, offset }) => {
                self.line(
                    pc,
                    &format!("let a_{pc:08x} = (c.reg({rs}) as i64).wrapping_add({offset}i64) as u32;"),
                );
                self.line(pc, &format!("let {t} = c.reg({rt}); c.store_u64(a_{pc:08x}, {t});"));
                true
            }

            ("syscall", _) => false, // handled in emit_function (needs the v1 constant)
            _ => false,
        }
    }

    /// Lower a COP1 arithmetic or compare instruction.
    fn emit_cop1(&mut self, pc: u32, inst: &Instruction) -> bool {
        use Operands::*;
        let m = inst.mnemonic;
        let (base, double) = if let Some(b) = m.strip_suffix(".s") {
            (b, false)
        } else if let Some(b) = m.strip_suffix(".d") {
            (b, true)
        } else {
            (m, false)
        };
        let read = if double { "c.f64" } else { "c.f32" };
        let setf = if double { "set_f64" } else { "set_f32" };

        match inst.operands {
            Cop1Arith { fd, fs, ft } => {
                // NOTE: adda/suba/mula/diva are approximated as their non-accumulating
                // forms; see docs/investigation/fpu.md.
                let expr = match base {
                    "add" | "adda" => "a + b",
                    "sub" | "suba" => "a - b",
                    "mul" | "mula" => "a * b",
                    "div" | "diva" => "a / b",
                    "abs" => "a.abs()",
                    "neg" => "-a",
                    "mov" => "a",
                    "sqrt" => "a.sqrt()",
                    _ => return false,
                };
                self.line(pc, &format!("let a = {read}({fs}); let b = {read}({ft});"));
                self.line(pc, &format!("c.{setf}({fd}, {expr});"));
                true
            }
            Cop1Cmp { fs, ft } => {
                let cmp = match base {
                    "c.f" | "c.sf" => "false",
                    "c.un" => "a.is_nan() || b.is_nan()",
                    "c.eq" | "c.seq" => "a == b",
                    "c.ueq" => "a.is_nan() || b.is_nan() || a == b",
                    "c.olt" => "a < b",
                    "c.ult" => "a.is_nan() || b.is_nan() || a < b",
                    "c.ole" => "a <= b",
                    "c.ule" => "a.is_nan() || b.is_nan() || a <= b",
                    "c.ngle" => "a.is_nan() || b.is_nan() || a > b",
                    "c.ngl" => "a.is_nan() || b.is_nan() || !(a < b)",
                    _ => return false,
                };
                self.line(pc, &format!("let a = {read}({fs}); let b = {read}({ft});"));
                self.line(pc, &format!("c.fcr31_cc = {cmp};"));
                true
            }
            _ => false,
        }
    }

    /// Emit a `syscall`, folding the `$v1` number into a direct SDK call.
    fn emit_syscall(&mut self, pc: u32, v1: Option<u64>) {
        let n = v1.map(|v| v as u32);
        if n == Some(0x04) {
            self.line(pc, "let a0 = c.reg(4);");
            self.line(pc, "crate::sdk::exit(c, a0);");
            return; // diverges
        }
        match n.and_then(|n| SYSCALLS.iter().find(|(num, ..)| *num == n).copied()) {
            Some((_, name, argc, returns)) => {
                if let Some(n) = n {
                    self.line(pc, &format!("c.note_syscall({n:#x});"));
                }
                for i in 0..argc {
                    self.line(pc, &format!("let a{i} = c.reg({});", 4 + i));
                }
                let args: Vec<String> = (0..argc).map(|i| format!("a{i}")).collect();
                let call = if args.is_empty() {
                    format!("crate::sdk::{name}(c)")
                } else {
                    format!("crate::sdk::{name}(c, {})", args.join(", "))
                };
                if returns {
                    self.line(pc, &format!("let r = {call};"));
                    self.line(pc, "c.set_reg(2, r);");
                } else {
                    self.line(pc, &format!("{call};"));
                }
            }
            None => {
                self.line(pc, "let r = crate::sdk::dynamic_syscall(c);");
                self.line(pc, "c.set_reg(2, r);");
            }
        }
        if self.dispatcher {
            self.line(pc, &format!("pc = {:#010x};", pc.wrapping_add(4)));
        }
    }

    fn emit_control(&mut self, pc: u32, inst: &Instruction, fall: u32) {
        use Operands::*;
        let m = inst.mnemonic;
        match (m, inst.operands) {
            ("beq" | "beql", Branch { rs, rt, offset }) => {
                let target = branch_target(pc, offset);
                self.line(pc, &format!("if c.reg({rs}) == c.reg({rt}) {{ pc = {target:#010x}; }} else {{ pc = {fall:#010x}; }}"));
            }
            ("bne" | "bnel", Branch { rs, rt, offset }) => {
                let target = branch_target(pc, offset);
                self.line(pc, &format!("if c.reg({rs}) != c.reg({rt}) {{ pc = {target:#010x}; }} else {{ pc = {fall:#010x}; }}"));
            }
            (m, Branch1 { rs, offset }) => {
                let target = branch_target(pc, offset);
                let cond = match m {
                    "blez" | "blezl" => format!("(c.reg({rs}) as i64) <= 0"),
                    "bgtz" | "bgtzl" => format!("(c.reg({rs}) as i64) > 0"),
                    "bltz" | "bltzl" | "bltzal" | "bltzall" => format!("(c.reg({rs}) as i64) < 0"),
                    _ => format!("(c.reg({rs}) as i64) >= 0"),
                };
                self.line(pc, &format!("if {cond} {{ pc = {target:#010x}; }} else {{ pc = {fall:#010x}; }}"));
            }
            ("j", Jump { target }) => {
                self.line(pc, &format!("pc = {:#010x};", jump_target(pc, target)));
            }
            ("jal", Jump { target }) => {
                let t = jump_target(pc, target);
                self.line(pc, &format!("c.set_reg(31, {fall:#010x});"));
                if self.translated.contains(&t) {
                    self.line(pc, &format!("{}(c);", fn_name(t)));
                    self.line(pc, &format!("pc = {fall:#010x};"));
                } else {
                    self.line(pc, &format!("c.untranslated_call({pc:#010x}, {t:#010x}); return;"));
                }
            }
            ("jr", Rs { rs }) => {
                self.line(pc, &format!("if ({rs} & 31) == 31 {{ return; }}"));
                self.line(pc, &format!("c.stop({pc:#010x}, 0, \"indirect jump\"); return;"));
            }
            ("jalr", RdRs { rs: _, .. }) => {
                self.line(pc, &format!("c.set_reg(31, {fall:#010x});"));
                self.line(pc, &format!("c.stop({pc:#010x}, 0, \"indirect call\"); return;"));
            }
            ("bc1f" | "bc1t", Cop1Branch { offset }) => {
                let target = branch_target(pc, offset);
                let cond = if m == "bc1t" { "c.fcr31_cc" } else { "!c.fcr31_cc" };
                self.line(pc, &format!("if {cond} {{ pc = {target:#010x}; }} else {{ pc = {fall:#010x}; }}"));
            }
            _ => {
                // eret and anything else we cannot model yet.
                self.line(pc, &format!("c.unimplemented({pc:#010x}, 0); return;"));
            }
        }
    }
}

fn branch_target(pc: u32, offset: i16) -> u32 {    pc.wrapping_add(4)
        .wrapping_add((offset as i32 as u32).wrapping_mul(4))
}

fn jump_target(pc: u32, field: u32) -> u32 {
    (pc.wrapping_add(4) & 0xF000_0000) | (field << 2)
}

/// Attempt to emit **structured** Rust for a function.
///
/// Returns `None` when the CFG cannot be structured yet (loops, irreducible flow,
/// targets outside the decoded set), in which case the caller falls back to the
/// basic-block dispatcher.
fn structured_emit(
    f: &Function,
    decoded: &[(u32, Instruction)],
    translated: &BTreeSet<u32>,
) -> Option<String> {
    let dbg = std::env::var("SSX3_DEBUG_STRUCT").is_ok();
    let fail = |why: &str| {
        if dbg {
            eprintln!("struct {} @ {:08x}: {why}", f.name, f.address);
        }
    };
    let Some(blocks) = build_cfg(decoded) else {
        fail("build_cfg None");
        return Option::None;
    };
    let some_ipdom = compute_ipdom(&blocks);
    let Some(headers) = compute_loops(&blocks) else {
        fail("compute_loops None (multi-exit loop?)");
        return Option::None;
    };
    let ipdom = some_ipdom;

    let mut s = Struct {
        e: Emit {
            out: String::new(),
            translated,
            indent: 1,
            v1: Option::None,
            dispatcher: false,
        },
        blocks: &blocks,
        decoded,
        ipdom,
        emitted: BTreeSet::new(),
        headers,
    };

    let _ = writeln!(s.e.out, "/// {} @ {:08x}", f.name, f.address);
    let _ = writeln!(s.e.out, "pub fn {}(c: &mut Cpu) {{", fn_name(f.address));

    let stop: BTreeSet<usize> = BTreeSet::new();
    if s.region(0, &stop, &[]).is_err() {
        fail("region Err");
        return Option::None;
    }

    let _ = writeln!(s.e.out, "}}");
    Option::Some(s.e.out)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Fall,
    Branch,
    Jump,
    TailCall,
    Call,
    Ret,
    Stop,
}

struct Blk {
    start: u32,
    lo: usize,
    hi: usize,
    /// Index of the control instruction, or `hi` for a fall-through block.
    term: usize,
    kind: Kind,
    succ: [usize; 2],
    nsucc: usize,
}

/// Static successor address of a control instruction, if any.
fn static_target(pc: u32, inst: &Instruction) -> Option<u32> {
    match inst.operands {
        Operands::Branch { offset, .. }
        | Operands::Branch1 { offset, .. }
        | Operands::Cop1Branch { offset } => Some(branch_target(pc, offset)),
        Operands::Jump { target } => Some(jump_target(pc, target)),
        _ => Option::None,
    }
}

fn build_cfg(decoded: &[(u32, Instruction)]) -> Option<Vec<Blk>> {
    if decoded.is_empty() {
        return Option::None;
    }
    let mut addr_to_idx: BTreeMap<u32, usize> = BTreeMap::new();
    for (i, (a, _)) in decoded.iter().enumerate() {
        addr_to_idx.insert(*a, i);
    }

    let mut leaders: BTreeSet<usize> = BTreeSet::new();
    leaders.insert(0);
    for i in 0..decoded.len() {
        let inst = &decoded[i].1;
        if is_control(inst.mnemonic) {
            if i + 2 <= decoded.len() {
                leaders.insert(i + 2);
            }
            if let Some(t) = static_target(decoded[i].0, inst) {
                // Targets outside this function (callees, tail calls) are not
                // basic-block leaders here; they are handled as calls.
                if let Some(&ti) = addr_to_idx.get(&t) {
                    leaders.insert(ti);
                }
            }
        }
    }
    let starts: Vec<usize> = leaders.into_iter().filter(|&x| x < decoded.len()).collect();
    // Map a decoded instruction index that is a leader to its block index.
    let block_of: BTreeMap<usize, usize> = starts.iter().enumerate().map(|(k, &s)| (s, k)).collect();
    let to_block = |di: usize| block_of.get(&di).copied().unwrap_or(usize::MAX);

    let mut blocks: Vec<Blk> = Vec::with_capacity(starts.len());
    for (bi, &lo) in starts.iter().enumerate() {
        let hi = starts.get(bi + 1).copied().unwrap_or(decoded.len());
        let mut term = hi;
        for i in (lo..hi).rev() {
            if is_control(decoded[i].1.mnemonic) {
                term = i;
                break;
            }
        }
        blocks.push(Blk {
            start: decoded[lo].0,
            lo,
            hi,
            term,
            kind: Kind::Fall,
            succ: [usize::MAX, usize::MAX],
            nsucc: 0,
        });
    }

    for bi in 0..blocks.len() {
        let (hi, term) = (blocks[bi].hi, blocks[bi].term);
        if term >= hi {
            blocks[bi].kind = Kind::Fall;
            if bi + 1 < blocks.len() {
                blocks[bi].succ[0] = bi + 1;
                blocks[bi].nsucc = 1;
            }
            continue;
        }
        let pc = decoded[term].0;
        let inst = &decoded[term].1;
        let fall = addr_to_idx
            .get(&pc.wrapping_add(8))
            .map(|&di| to_block(di))
            .unwrap_or(usize::MAX);
        match inst.operands {
            Operands::Branch { offset, .. }
            | Operands::Branch1 { offset, .. }
            | Operands::Cop1Branch { offset } => {
                let taken = addr_to_idx
                    .get(&branch_target(pc, offset))
                    .map(|&di| to_block(di))
                    .unwrap_or(usize::MAX);
                blocks[bi].kind = Kind::Branch;
                blocks[bi].succ = [taken, fall];
                blocks[bi].nsucc = 2;
            }
            Operands::Jump { target } if inst.mnemonic == "j" => {
                let t = jump_target(pc, target);
                match addr_to_idx.get(&t) {
                    Some(&di) => {
                        blocks[bi].kind = Kind::Jump;
                        blocks[bi].succ[0] = to_block(di);
                        blocks[bi].nsucc = 1;
                    }
                    // A jump outside this function is a tail call.
                    Option::None => {
                        blocks[bi].kind = Kind::TailCall;
                        blocks[bi].nsucc = 0;
                    }
                }
            }
            Operands::Jump { .. } if inst.mnemonic == "jal" => {
                blocks[bi].kind = Kind::Call;
                blocks[bi].succ[0] = fall;
                blocks[bi].nsucc = 1;
            }
            Operands::Rs { rs } if inst.mnemonic == "jr" => {
                blocks[bi].kind = if rs & 31 == 31 { Kind::Ret } else { Kind::Stop };
            }
            Operands::RdRs { .. } if inst.mnemonic == "jalr" => blocks[bi].kind = Kind::Stop,
            _ if inst.mnemonic == "eret" => blocks[bi].kind = Kind::Stop,
            _ => {
                blocks[bi].kind = Kind::Fall;
                if bi + 1 < blocks.len() {
                    blocks[bi].succ[0] = bi + 1;
                    blocks[bi].nsucc = 1;
                }
            }
        }
    }

    for b in &blocks {
        for k in 0..b.nsucc {
            if b.succ[k] == usize::MAX {
                if std::env::var("SSX3_DEBUG_STRUCT").is_ok() {
                    let (pc, inst) = decoded[b.term];
                    eprintln!(
                        "  unresolved successor: block {:08x} term {:08x} {} (k={k})",
                        b.start, pc, inst.mnemonic
                    );
                }
                return Option::None;
            }
        }
    }
    Option::Some(blocks)
}

/// Immediate post-dominators, or `None` when a block has no post-dominator.
fn compute_ipdom(blocks: &[Blk]) -> Vec<Option<usize>> {
    let n = blocks.len();
    let mut post: Vec<Vec<bool>> = vec![vec![true; n]; n];
    for i in 0..n {
        if blocks[i].nsucc == 0 {
            for v in post[i].iter_mut() {
                *v = false;
            }
        }
        post[i][i] = true;
    }
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..n {
            if blocks[i].nsucc == 0 {
                continue;
            }
            let mut acc = vec![true; n];
            for k in 0..blocks[i].nsucc {
                let s = blocks[i].succ[k];
                for j in 0..n {
                    acc[j] &= post[s][j];
                }
            }
            acc[i] = true;
            if acc != post[i] {
                post[i] = acc;
                changed = true;
            }
        }
    }
    let mut ipdom = vec![Option::None; n];
    for i in 0..n {
        for x in 0..n {
            if x == i || !post[i][x] {
                continue;
            }
            // x is the immediate post-dominator when every other post-dominator
            // of i also post-dominates x.
            let ok = (0..n).all(|z| z == i || !post[i][z] || z == x || post[z][x]);
            if ok {
                ipdom[i] = Some(x);
                break;
            }
        }
    }
    ipdom
}

struct Struct<'a> {
    e: Emit<'a>,
    blocks: &'a [Blk],
    decoded: &'a [(u32, Instruction)],
    ipdom: Vec<Option<usize>>,
    emitted: BTreeSet<usize>,
    /// Natural loops: header block index -> loop extent and exit.
    headers: BTreeMap<usize, LoopInfo>,
}

/// A natural loop, approximated by `[lo, hi]` (block indices, address order).
#[derive(Clone, Copy)]
struct LoopInfo {
    lo: usize,
    hi: usize,
    exit: Option<usize>,
}

/// How control leaves a region.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Flow {
    /// Control continues at the given block.
    Next(usize),
    /// The path terminates here (return, stop, `break`, `continue`).
    End,
}

/// Detect natural loops from back edges. Returns `None` if a loop has more than
/// one exit (not yet supported).
fn compute_loops(blocks: &[Blk]) -> Option<BTreeMap<usize, LoopInfo>> {
    let mut backs: BTreeMap<usize, usize> = BTreeMap::new();
    for b in 0..blocks.len() {
        for k in 0..blocks[b].nsucc {
            let s = blocks[b].succ[k];
            if s <= b {
                let e = backs.entry(s).or_insert(b);
                if b > *e {
                    *e = b;
                }
            }
        }
    }
    let mut map = BTreeMap::new();
    for (&h, &hi) in &backs {
        let lo = h;
        let mut exits: BTreeSet<usize> = BTreeSet::new();
        for b in lo..=hi {
            for k in 0..blocks[b].nsucc {
                let x = blocks[b].succ[k];
                if x < lo || x > hi {
                    exits.insert(x);
                }
            }
        }
        if exits.len() > 1 {
            return Option::None;
        }
        map.insert(
            h,
            LoopInfo {
                lo,
                hi,
                exit: exits.iter().next().copied(),
            },
        );
    }
    Option::Some(map)
}

impl<'a> Struct<'a> {
    /// Emit a straight-line run of instructions `[lo, hi)`.
    fn normal(&mut self, lo: usize, hi: usize) -> Result<(), ()> {
        for i in lo..hi {
            let (pc, inst) = self.decoded[i];
            self.e.line(pc, &format!("c.tick({pc:#010x}, {:#010x});", inst.word));
            if inst.mnemonic == "syscall" {
                self.e.emit_syscall(pc, self.e.v1);
            } else if !self.e.lower(pc, &inst) {
                return Err(());
            } else if dest_reg(&inst) == Some(3) {
                self.e.v1 = const_value(&inst);
            }
        }
        Ok(())
    }

    /// Emit a block's body, including the control instruction and its delay slot
    /// (the control *decision* is emitted by the structurer).
    fn block_body(&mut self, bi: usize) -> Result<(), ()> {
        let (lo, hi, term) = (self.blocks[bi].lo, self.blocks[bi].hi, self.blocks[bi].term);
        self.normal(lo, term)?;
        if term < hi {
            let (pc, inst) = self.decoded[term];
            self.e.line(pc, &format!("c.tick({pc:#010x}, {:#010x});", inst.word));
            if term + 1 < hi {
                let (dpc, dinst) = self.decoded[term + 1];
                if !is_control(dinst.mnemonic) {
                    self.e.line(dpc, &format!("c.tick({dpc:#010x}, {:#010x});", dinst.word));
                    if !self.e.lower(dpc, &dinst) {
                        return Err(());
                    }
                }
            }
        }
        Ok(())
    }

    fn region(
        &mut self,
        start: usize,
        stop: &BTreeSet<usize>,
        loops: &[usize],
    ) -> Result<Flow, ()> {
        let mut cur = start;
        let mut guard = 0usize;
        loop {
            guard += 1;
            if guard > 200_000 {
                return Err(());
            }
            if stop.contains(&cur) || self.emitted.contains(&cur) {
                return Ok(Flow::Next(cur));
            }

            // Inside a loop: leaving the loop body breaks out of it.
            if let Some(&h) = loops.last() {
                let info = self.headers[&h];
                if cur < info.lo || cur > info.hi {
                    self.e.line(self.blocks[cur].start, "break;");
                    return Ok(Flow::End);
                }
            }

            // Enter a natural loop when we reach its header from outside.
            if let Some(info) = self.headers.get(&cur).copied() {
                if !loops.contains(&cur) {
                    let start_addr = self.blocks[cur].start;
                    self.e.line(start_addr, "loop {");
                    self.e.indent += 1;
                    self.e.line(start_addr, "if c.trap.is_some() { return; }");
                    let mut inner = loops.to_vec();
                    inner.push(cur);
                    self.region(cur, stop, &inner)?;
                    self.e.indent -= 1;
                    self.e.line(start_addr, "}");
                    // Continue the enclosing region at the loop's exit.
                    match info.exit {
                        Some(x) if !stop.contains(&x) => {
                            cur = x;
                            continue;
                        }
                        _ => return Ok(Flow::End),
                    }
                }
            }

            self.emitted.insert(cur);
            let b = &self.blocks[cur];
            let (start_addr, kind, succ, nsucc, term) =
                (b.start, b.kind, b.succ, b.nsucc, b.term);
            self.block_body(cur)?;

            match kind {
                Kind::Ret => {
                    self.e.line(start_addr, "return;");
                    return Ok(Flow::End);
                }
                Kind::Stop => {
                    self.e.line(start_addr, &format!("c.stop({start_addr:#010x}, 0, \"unresolved control flow\"); return;"));
                    return Ok(Flow::End);
                }
                Kind::Fall => {
                    if nsucc == 0 {
                        return Ok(Flow::End);
                    }
                    cur = succ[0];
                }
                Kind::Jump => {
                    let t = succ[0];
                    if let Some(&h) = loops.last() {
                        if t == h {
                            self.e.line(start_addr, "continue;");
                            return Ok(Flow::End);
                        }
                        let info = self.headers[&h];
                        if t < info.lo || t > info.hi {
                            self.e.line(start_addr, "break;");
                            return Ok(Flow::End);
                        }
                    }
                    if t <= cur {
                        return Err(()); // a backward edge that is not a known loop
                    }
                    cur = t;
                }
                Kind::Call => {
                    let (pc, inst) = self.decoded[term];
                    let target = match inst.operands {
                        Operands::Jump { target } => jump_target(pc, target),
                        _ => return Err(()),
                    };
                    let fall = pc.wrapping_add(8);
                    self.e.line(pc, &format!("c.set_reg(31, {fall:#010x});"));
                    if self.e.translated.contains(&target) {
                        self.e.line(pc, &format!("{}(c);", fn_name(target)));
                        self.e.line(pc, "if c.trap.is_some() { return; }");
                    } else {
                        self.e.line(pc, &format!("c.untranslated_call({pc:#010x}, {target:#010x}); return;"));
                        return Ok(Flow::End);
                    }
                    if nsucc == 0 {
                        return Ok(Flow::End);
                    }
                    cur = succ[0];
                }
                Kind::TailCall => {
                    let (pc, inst) = self.decoded[term];
                    let target = match inst.operands {
                        Operands::Jump { target } => jump_target(pc, target),
                        _ => return Err(()),
                    };
                    if self.e.translated.contains(&target) {
                        self.e.line(pc, &format!("{}(c);", fn_name(target)));
                    } else {
                        self.e.line(pc, &format!("c.untranslated_call({pc:#010x}, {target:#010x});"));
                    }
                    self.e.line(start_addr, "return;");
                    return Ok(Flow::End);
                }
                Kind::Branch => {
                    let cond = self.branch_cond(cur)?;
                    let join = self.ipdom[cur];
                    let mut inner = stop.clone();
                    if let Some(j) = join {
                        inner.insert(j);
                    }
                    self.e.line(start_addr, &format!("if {cond} {{"));
                    self.e.indent += 1;
                    self.e.v1 = Option::None;
                    let ft = self.arm(succ[0], &inner, loops)?;
                    self.e.v1 = Option::None;
                    self.e.indent -= 1;
                    self.e.line(start_addr, "} else {");
                    self.e.indent += 1;
                    let ff = self.arm(succ[1], &inner, loops)?;
                    self.e.v1 = Option::None;
                    self.e.indent -= 1;
                    self.e.line(start_addr, "}");
                    if ft == Flow::End && ff == Flow::End {
                        return Ok(Flow::End);
                    }
                    match join {
                        Some(j) if !stop.contains(&j) => cur = j,
                        _ => return Ok(Flow::End),
                    }
                }
            }
        }
    }

    /// Emit one arm of a branch. Back edges to the innermost loop become
    /// `continue`, and jumps out of it become `break`.
    fn arm(
        &mut self,
        s: usize,
        stop: &BTreeSet<usize>,
        loops: &[usize],
    ) -> Result<Flow, ()> {
        if let Some(&h) = loops.last() {
            if s == h {
                self.e.line(self.blocks[s].start, "continue;");
                return Ok(Flow::End);
            }
            let info = self.headers[&h];
            if s < info.lo || s > info.hi {
                self.e.line(self.blocks[s].start, "break;");
                return Ok(Flow::End);
            }
        }
        self.region(s, stop, loops)
    }

    fn branch_cond(&self, bi: usize) -> Result<String, ()> {
        let term = self.blocks[bi].term;
        let (pc, inst) = self.decoded[term];
        let _ = pc;
        use Operands::*;
        let m = inst.mnemonic;
        let c = match (m, inst.operands) {
            ("beq" | "beql", Branch { rs, rt, .. }) => format!("c.reg({rs}) == c.reg({rt})"),
            ("bne" | "bnel", Branch { rs, rt, .. }) => format!("c.reg({rs}) != c.reg({rt})"),
            (m, Branch1 { rs, .. }) => match m {
                "blez" | "blezl" => format!("(c.reg({rs}) as i64) <= 0"),
                "bgtz" | "bgtzl" => format!("(c.reg({rs}) as i64) > 0"),
                "bltz" | "bltzl" | "bltzal" | "bltzall" => format!("(c.reg({rs}) as i64) < 0"),
                _ => format!("(c.reg({rs}) as i64) >= 0"),
            },
            ("bc1t", _) => "c.fcr31_cc".to_string(),
            ("bc1f", _) => "!c.fcr31_cc".to_string(),
            _ => return Err(()),
        };
        Ok(c)
    }
}

/// Emit a translated Rust function for `f`. `translated` is the set of function
/// addresses that have generated bodies (for direct calls).
///
/// Structured control flow is attempted first; when it cannot be recovered the
/// function falls back to a per-function basic-block dispatch (scaffolding).
pub fn emit_function(program: &Program, f: &Function, translated: &BTreeSet<u32>) -> String {
    let decoded = program.decode_with_addresses(f);
    if let Some(src) = structured_emit(f, &decoded, translated) {
        return src;
    }
    emit_dispatcher(f, &decoded, translated)
}

/// Fallback: a per-function basic-block dispatcher over the function's own CFG.
fn emit_dispatcher(f: &Function, decoded: &[(u32, Instruction)], translated: &BTreeSet<u32>) -> String {
    let mut e = Emit {
        out: String::new(),
        translated,
        indent: 3,
        v1: Option::None,
        dispatcher: true,
    };

    let name = fn_name(f.address);
    let _ = writeln!(e.out, "/// {} @ {:08x}", f.name, f.address);
    let _ = writeln!(e.out, "pub fn {name}(c: &mut Cpu) {{");
    let _ = writeln!(e.out, "    let mut pc: u32 = {:#010x};", f.address);
    let _ = writeln!(e.out, "    // NOTE: basic-block dispatch is scaffolding for structured control flow.");
    let _ = writeln!(e.out, "    while c.trap.is_none() {{");
    let _ = writeln!(e.out, "        match pc {{");

    let mut v1: Option<u64> = Option::None;
    let mut i = 0;
    while i < decoded.len() {
        let (pc, inst) = decoded[i];
        let delay = decoded.get(i + 1).copied();
        let _ = writeln!(e.out, "            {pc:#010x} => {{");
        e.line(pc, &format!("c.tick({pc:#010x}, {:#010x});", inst.word));

        if inst.mnemonic == "syscall" {
            e.emit_syscall(pc, v1);
        } else if is_control(inst.mnemonic) {
            let fall = pc.wrapping_add(8);
            let mut ok = true;
            if let Some((dpc, dinst)) = delay {
                if dpc == pc.wrapping_add(4) && !is_control(dinst.mnemonic) {
                    e.line(dpc, &format!("c.tick({dpc:#010x}, {:#010x});", dinst.word));
                    ok = e.lower(dpc, &dinst);
                }
            }
            if ok {
                e.emit_control(pc, &inst, fall);
            } else {
                e.line(pc, &format!("c.unimplemented({pc:#010x}, {:#010x}); return;", inst.word));
            }
            i += 2;
            v1 = Option::None;
            let _ = writeln!(e.out, "            }}");
            continue;
        } else if !e.lower(pc, &inst) {
            e.line(pc, &format!("c.unimplemented({pc:#010x}, {:#010x}); return;", inst.word));
        } else {
            e.line(pc, &format!("pc = {:#010x};", pc.wrapping_add(4)));
        }

        // Track the syscall-number register ($v1 == 3).
        if dest_reg(&inst) == Some(3) {
            v1 = const_value(&inst);
        }

        i += 1;
        let _ = writeln!(e.out, "            }}");
    }

    let _ = writeln!(e.out, "            _ => {{ c.stop(pc, 0, \"pc outside translated function\"); return; }}");
    let _ = writeln!(e.out, "        }}");
    let _ = writeln!(e.out, "    }}");
    let _ = writeln!(e.out, "}}");
    e.out
}
