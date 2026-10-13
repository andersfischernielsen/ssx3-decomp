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

use std::collections::BTreeSet;
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
}

impl<'a> Emit<'a> {
    fn line(&mut self, pc: u32, s: &str) {
        let _ = writeln!(self.out, "            {}", s.replace("$PC", &format!("{pc:#010x}")));
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
            ("clear_sa", _) | ("clear_f", _) | ("clear_fcsr", _) => true,
            // FPU arithmetic is not modelled yet; these are no-ops for now so the
            // init path can run. See docs/investigation/codegen.md.
            (m, _) if m.contains('.') => true,

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
        self.line(pc, &format!("pc = {:#010x};", pc.wrapping_add(4)));
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
            _ => {
                // bc1f/bc1t and eret: no FPU / no exceptions modelled yet.
                self.line(pc, &format!("c.unimplemented({pc:#010x}, 0); return;"));
            }
        }
    }
}

fn branch_target(pc: u32, offset: i16) -> u32 {
    pc.wrapping_add(4)
        .wrapping_add((offset as i32 as u32).wrapping_mul(4))
}

fn jump_target(pc: u32, field: u32) -> u32 {
    (pc.wrapping_add(4) & 0xF000_0000) | (field << 2)
}

/// Emit a translated Rust function for `f`. `translated` is the set of function
/// addresses that have generated bodies (for direct calls).
pub fn emit_function(program: &Program, f: &Function, translated: &BTreeSet<u32>) -> String {
    let decoded = program.decode_with_addresses(f);
    let mut e = Emit {
        out: String::new(),
        translated,
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
