//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacement for `InputVM_Execute`, the input parser's bytecode
//! interpreter, with a differential test against the generated body.
//!
//! The interpreter walks a 32-bit word stream. Each word holds an opcode in
//! its low six bits plus up to three operand indices; the parser object holds
//! a file of virtual registers (`parser + 8 + 4*i`, float bits). Opcodes
//! 0x01..0x1f evaluate an expression into a destination register
//! (`dst = (word >> 6) & 0x3f`), operands are read through
//! `InputVM_ReadOperand`, and 0x02/0x03 skip forward. Opcode 0x20 ends the
//! run, after which the original reads operand 0 (its result, in `f0`).
//!
//! The original dispatches through a jump table at [`OPCODES`]; the port
//! matches on the opcode instead and keeps only that table's entry address in
//! `v1` (the original leaves it there when a handler does not overwrite it).
//! Opcodes 0x04/0x05 keep their destination in `s3` and use `s0` as a flag;
//! every other handler keeps it in `s0`.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0032_5450, input_vm_execute, "InputVM_Execute")];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

mod callee {
    use super::{ee, Callee};

    pub const GET_REG_PTR: Callee =
        Callee { addr: 0x0032_52e8, f: ee::s10::InputVM_GetRegPtr };
    pub const READ_OPERAND: Callee =
        Callee { addr: 0x0032_5260, f: ee::s10::InputVM_ReadOperand };
    pub const MATH_ATAN: Callee = Callee { addr: 0x0031_c228, f: ee::s10::Math_Atan };
    pub const MATH_SIN: Callee = Callee { addr: 0x0031_bf60, f: ee::s10::Math_Sin };
    pub const MATH_COS: Callee = Callee { addr: 0x0031_c040, f: ee::s10::Math_Cos };

    /// `logf` and `expf`: cross-shard, so the original reaches them through
    /// the function table.
    pub const LOGF: u32 = 0x0040_da10;
    pub const EXPF: u32 = 0x0040_d758;
}

/// The opcode dispatch table (`jpt_3254c4`). The port matches on the opcode;
/// the address is only kept to reproduce what the original leaves in `v1`.
const OPCODES: u32 = 0x0048_e330;

/// Opcode that ends the run (`s6 = 1`).
const OP_STOP: u32 = 0x20;

/// Loop head: the resume address of the back-edge event check.
const HEAD: u32 = 0x0032_5498;
/// The shared "store `f0` into the destination" block, and the event check
/// that resumes there.
const STORE: u32 = 0x0032_5524;
/// The block that reads the first operand again before that store.
const STORE_OPERAND: u32 = 0x0032_5518;

/// `parser` fields: the incoming argument (`+4`) and the virtual register
/// file (`+8`, one 32-bit float each).
const PARSER_ARG: u32 = 0x4;
#[allow(dead_code)] // the register file base is only named in the test
const PARSER_REGS: u32 = 0x8;

/// `gp`-relative literals the expression opcodes read: the atan2 half-plane
/// corrections (0x13) and the angle wrap (0x1d).
mod literal {
    pub const HALF_PI_NEG: i32 = -0x2e24;
    pub const HALF_PI: i32 = -0x2e20;
    pub const PI: i32 = -0x2e1c;
    pub const PI_SUB: i32 = -0x2e18;
    pub const INV_TWO_PI: i32 = -0x2e14;
    pub const TWO_PI: i32 = -0x2e10;
}

// ---------------------------------------------------------------------------
// The interpreter's state: the values the original keeps in s0..s6.

struct Vm {
    /// s2: the parser (its virtual registers at `+8`, four bytes each).
    parser: u32,
    /// s5: the opcode stream.
    stream: u32,
    /// s1: the current instruction.
    ins: u32,
    /// s4: the index of the word after the current instruction.
    idx: u32,
    /// s0: destination of most opcodes (a 0/1 flag in 0x04/0x05).
    dst: u32,
    /// s3: destination of 0x04/0x05, which use `s0` as a flag.
    alt: u32,
    /// s6: set by opcode 0x20 to end the run.
    stop: u64,
}

impl Vm {
    /// Put the loop state where a callee (or an event handler) expects it: the
    /// original keeps these in the callee-saved registers throughout.
    fn spill(&self, c: &mut Cpu) {
        c.g[S0][0] = sx(self.dst);
        c.g[S1][0] = sx(self.ins);
        c.g[S2][0] = sx(self.parser);
        c.g[S3][0] = sx(self.alt);
        c.g[S4][0] = sx(self.idx);
        c.g[S5][0] = sx(self.stream);
        c.g[S6][0] = sx(self.stop as u32);
    }
}

// ---------------------------------------------------------------------------
// FPU and call helpers.

/// FPU register `n`.
#[inline(always)]
fn fr(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

/// `fN = v`.
#[inline(always)]
fn setf(c: &mut Cpu, n: usize, v: FpuF32) -> FpuF32 {
    c.f[n] = v.0;
    v
}

/// `li at, bits; mtc1 at, fN`.
#[inline(always)]
fn li_f(c: &mut Cpu, n: usize, bits: u32) -> FpuF32 {
    c.g[AT][0] = sx(bits);
    setf(c, n, FpuF32(bits))
}

/// `lwc1 fN, gp+off`.
#[inline(always)]
fn gp_float(c: &mut Cpu, off: i32, n: usize) -> FpuF32 {
    let a = r32(c, GP).wrapping_add(off as u32);
    let v = Addr(a).read_fpu(c);
    setf(c, n, v)
}

/// Set the FPU condition bit; returns it.
#[inline(always)]
fn fcc(c: &mut Cpu, cond: bool) -> bool {
    abi::set_fcc(c, cond);
    cond
}

/// `InputVM_GetRegPtr(parser, idx)`: the address of virtual register `idx`.
fn dst_of(c: &mut Cpu, vm: &mut Vm, idx: u32, ret: u32) -> Result<u32, Unwound> {
    vm.spill(c);
    c.g[A0][0] = sx(vm.parser);
    c.g[A1][0] = sx(idx);
    callee::GET_REG_PTR.call(c, ret)?;
    Ok(r32(c, V0))
}

/// `f0 = InputVM_ReadOperand(parser, idx)`.
fn operand(c: &mut Cpu, vm: &Vm, idx: u32, ret: u32) -> Result<FpuF32, Unwound> {
    vm.spill(c);
    c.g[A0][0] = sx(vm.parser);
    c.g[A1][0] = sx(idx);
    callee::READ_OPERAND.call(c, ret)?;
    Ok(fr(c, 0))
}

/// The shared `0x00325524` tail: store `f0` in the destination.
fn store_result(c: &mut Cpu, vm: &Vm) {
    c.cycles += 2;
    let v = fr(c, 0);
    Addr(vm.dst).write_fpu(c, v);
}

/// The shared `0x00325518` tail: `f0 = ReadOperand(parser, idx)`, then store.
fn store_operand(c: &mut Cpu, vm: &Vm, idx: u32) -> Flow {
    c.cycles += 3;
    operand(c, vm, idx, STORE)?;
    store_result(c, vm);
    Ok(())
}

// ---------------------------------------------------------------------------
// InputVM_Execute.

/// `float InputVM_Execute(InputVM* parser, void* arg, const u32* code)`
/// (0x325450): run the opcode stream at `code` (kept at `parser + 4`) up to
/// opcode 0x20, then return operand 0 of the parser's register file.
pub fn input_vm_execute(c: &mut Cpu) {
    guest(c, |c| {
        // Prologue: 0xa0 frame, s0-s3 and f20-f22 saved.
        c.cycles += 18;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S2, 0x50);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save(c, S3, 0x40);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 22, 0x90);
        frame.save_fpr(c, 21, 0x88);
        frame.save_fpr(c, 20, 0x80);
        // s2/s5 are set here, s4/s6 are zeroed; s0/s1/s3 keep the incoming
        // values (as the prologue leaves them, visible only if a callee
        // unwinds before the epilogue).
        let mut vm = Vm {
            parser: r32(c, A0),
            stream: r32(c, A2),
            ins: r32(c, S1),
            idx: 0,
            dst: r32(c, S0),
            alt: r32(c, S3),
            stop: 0,
        };
        Addr(vm.parser.wrapping_add(PARSER_ARG)).sw(c, c.g[A1][0]);
        c.g[V0][0] = 0;

        'run: loop {
            // 0x00325498: fetch and dispatch.
            c.cycles += 6;
            vm.ins = vm.stream.wrapping_add(vm.idx.wrapping_shl(2));
            let word = c.lw(vm.ins) as u32;
            let op = word & 0x3f;
            c.g[V0][0] = sx(word);
            c.g[A0][0] = sx(op);
            let dispatched = op <= OP_STOP;
            c.g[V1][0] = dispatched as u64;
            vm.idx = vm.idx.wrapping_add(1);
            if dispatched {
                // 0x003254b0: the jump table (`v1` keeps its entry address).
                c.cycles += 7;
                c.g[V0][0] = sx(OPCODES);
                c.g[V1][0] = sx(OPCODES.wrapping_add(op << 2));
                run_op(c, &mut vm, op, word)?;
            }

            // 0x00325efc: loop while opcode 0x20 has not been seen.
            c.cycles += 2;
            c.g[V0][0] = sx(vm.idx.wrapping_shl(2));
            if vm.stop != 0 {
                break 'run;
            }
            if event_due(c) && deliver_event(c, HEAD) {
                return Ok(());
            }
        }

        // 0x00325f04: the result is operand 0.
        c.cycles += 16;
        operand(c, &vm, 0, 0x0032_5f10)?;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x90);
        frame.restore_fpr(c, 21, 0x88);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    });
}

/// One dispatched opcode. The cycle charges are the original's per-block
/// costs; `w` is the instruction word.
fn run_op(c: &mut Cpu, vm: &mut Vm, op: u32, w: u32) -> Flow {
    let d_idx = (w >> 6) & 0x3f;
    let a_idx = (w >> 12) & 0x3ff;
    let b_idx = w >> 22;
    match op {
        // 0x04cc: `dst = the float word following the instruction`.
        0 => {
            c.cycles += 11;
            let dst = dst_of(c, vm, d_idx, 0x0032_54e0)?;
            vm.dst = dst;
            let at = vm.stream.wrapping_add(vm.idx.wrapping_shl(2));
            vm.idx = vm.idx.wrapping_add(1);
            c.g[V1][0] = sx(at);
            let v = Addr(at).read_fpu(c);
            setf(c, 0, v);
            Addr(dst).write_fpu(c, v);
        }
        // 0x04f8: `dst = operand(a)`.
        1 => {
            c.cycles += 8;
            let dst = dst_of(c, vm, d_idx, 0x0032_550c)?;
            vm.dst = dst;
            store_operand(c, vm, a_idx)?;
        }
        // 0x052c: skip `(word >> 16)` words when operand `(word >> 6) & 0x3ff`
        // is zero.
        2 => {
            c.cycles += 9;
            let v = operand(c, vm, (w >> 6) & 0x3ff, 0x0032_5540)?;
            setf(c, 1, FpuF32::ZERO);
            if fcc(c, v.eq_s(fr(c, 1))) {
                skip_words(c, vm);
            }
        }
        // 0x0550: skip forward unconditionally.
        3 => skip_words(c, vm),
        // 0x055c: `dst = (a != 0) || (b != 0)`.
        4 => {
            c.cycles += 17;
            vm.dst = 0;
            setf(c, 20, FpuF32::ZERO);
            let dst = dst_of(c, vm, d_idx, 0x0032_5578)?;
            vm.alt = dst;
            let a = operand(c, vm, a_idx, 0x0032_5590)?;
            let zero = fr(c, 20);
            let a_is_zero = fcc(c, a.eq_s(zero));
            let nonzero = if !a_is_zero {
                c.cycles += 1;
                true
            } else {
                c.cycles += 6;
                let b = operand(c, vm, b_idx, 0x0032_55ac)?;
                let b_is_zero = fcc(c, b.eq_s(zero));
                if !b_is_zero {
                    c.cycles += 1;
                }
                !b_is_zero
            };
            // 0x003255bc/0x0032562c + `mtc1 s0, f0; cvt.s.w f0, f0`.
            vm.dst = nonzero as u32;
            c.cycles += 2;
            c.cycles += 4;
            let v = setf(c, 0, FpuF32::from_i32(nonzero as i32));
            Addr(dst).write_fpu(c, v);
        }
        // 0x05cc: `dst = (a != 0) && (b != 0)`.
        5 => {
            c.cycles += 17;
            vm.dst = 0;
            setf(c, 20, FpuF32::ZERO);
            let dst = dst_of(c, vm, d_idx, 0x0032_55e8)?;
            vm.alt = dst;
            let a = operand(c, vm, a_idx, 0x0032_5600)?;
            let zero = fr(c, 20);
            let a_is_zero = fcc(c, a.eq_s(zero));
            let nonzero = if a_is_zero {
                false
            } else {
                c.cycles += 7;
                let b = operand(c, vm, b_idx, 0x0032_561c)?;
                let b_is_zero = fcc(c, b.eq_s(zero));
                !b_is_zero
            };
            // 0x003255bc/0x0032562c + `mtc1 s0, f0; cvt.s.w f0, f0`.
            vm.dst = nonzero as u32;
            c.cycles += 2;
            c.cycles += 4;
            let v = setf(c, 0, FpuF32::from_i32(nonzero as i32));
            Addr(dst).write_fpu(c, v);
        }
        // 0x0658: `dst = a > b` (the compare is `op2 < op1`).
        6 => compare(c, vm, w, [0x0032_566c, 0x0032_5684, 0x0032_5698], true, |a, b| {
            b.lt(a)
        })?,
        // 0x06b8: `dst = a >= b` (the compare is `op2 <= op1`).
        7 => compare(c, vm, w, [0x0032_56cc, 0x0032_56e4, 0x0032_56f8], true, |a, b| {
            b.le(a)
        })?,
        // 0x0718: `dst = a < b`.
        8 => compare(c, vm, w, [0x0032_572c, 0x0032_5744, 0x0032_5758], true, |a, b| {
            a.lt(b)
        })?,
        // 0x0778: `dst = a <= b`.
        9 => compare(c, vm, w, [0x0032_578c, 0x0032_57a4, 0x0032_57b8], true, |a, b| {
            a.le(b)
        })?,
        // 0x07d8: `dst = a == b`.
        10 => compare(c, vm, w, [0x0032_57ec, 0x0032_5804, 0x0032_5818], true, |a, b| {
            a.eq_s(b)
        })?,
        // 0x0838: `dst = a != b` (the compare is an equality test).
        11 => compare(c, vm, w, [0x0032_584c, 0x0032_5864, 0x0032_5878], false, |a, b| {
            a.eq_s(b)
        })?,
        // 0x0898: `dst = a + b` (the result stays in f20).
        12 => binary(c, vm, w, 0x0032_58ac, 0x0032_58c4, 0x0032_58d8, |a, b| a + b)?,
        // 0x08e4: `dst = a - b`.
        13 => binary(c, vm, w, 0x0032_58f8, 0x0032_5910, 0x0032_5924, |a, b| a - b)?,
        // 0x0930: `dst = a * b`.
        14 => binary(c, vm, w, 0x0032_5944, 0x0032_595c, 0x0032_5970, |a, b| a * b)?,
        // 0x097c: `dst = a / b`.
        15 => binary(c, vm, w, 0x0032_5990, 0x0032_59a8, 0x0032_59bc, |a, b| a.div(b))?,
        // 0x09c8: `dst = max(a, b)` — the larger operand is read again.
        16 => {
            c.cycles += 20;
            let dst = dst_of(c, vm, d_idx, 0x0032_59dc)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_59f4)?;
            setf(c, 20, a);
            let b = operand(c, vm, b_idx, 0x0032_5a08)?;
            let a = fr(c, 20);
            if fcc(c, b.lt(a)) {
                c.cycles += 2;
                if event_due(c) && deliver_event(c, STORE_OPERAND) {
                    return Ok(());
                }
                store_operand(c, vm, a_idx)?;
            } else {
                c.cycles += 5;
                operand(c, vm, b_idx, 0x0032_5a2c)?;
                let v = fr(c, 0);
                Addr(dst).write_fpu(c, v);
            }
        }
        // 0x0a34: `dst = min(a, b)`.
        17 => {
            c.cycles += 20;
            let dst = dst_of(c, vm, d_idx, 0x0032_5a48)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5a60)?;
            setf(c, 20, a);
            let b = operand(c, vm, b_idx, 0x0032_5a74)?;
            let a = fr(c, 20);
            if fcc(c, a.lt(b)) {
                c.cycles += 2;
                if event_due(c) && deliver_event(c, STORE_OPERAND) {
                    return Ok(());
                }
                store_operand(c, vm, a_idx)?;
            } else {
                c.cycles += 5;
                operand(c, vm, b_idx, 0x0032_5a98)?;
                let v = fr(c, 0);
                Addr(dst).write_fpu(c, v);
            }
        }
        // 0x0aa0: `dst = pow(a, b)` = `exp(log(a) * b)`.
        18 => {
            c.cycles += 21;
            let dst = dst_of(c, vm, d_idx, 0x0032_5ab4)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5acc)?;
            setf(c, 21, a);
            let b = operand(c, vm, b_idx, 0x0032_5ae0)?;
            setf(c, 20, b);
            setf(c, 12, fr(c, 21));
            vm.spill(c);
            call_ptr(c, callee::LOGF, 0x0032_5aec)?;
            let (log, exp) = (fr(c, 0), fr(c, 20));
            setf(c, 12, log * exp);
            exp_into(c, vm, dst)?;
        }
        // 0x0af4: `atan2(a, b)`, with the two half-plane corrections.
        19 => {
            c.cycles += 21;
            let dst = dst_of(c, vm, d_idx, 0x0032_5b0c)?;
            vm.dst = dst;
            setf(c, 22, FpuF32::ZERO);
            let a = operand(c, vm, a_idx, 0x0032_5b24)?;
            setf(c, 21, a);
            let b = operand(c, vm, b_idx, 0x0032_5b38)?;
            setf(c, 20, b);
            let (b, zero) = (fr(c, 20), fr(c, 22));
            if !fcc(c, b.eq_s(zero)) {
                c.cycles += 7;
                let a = fr(c, 21);
                setf(c, 12, a.div(b));
                callee::MATH_ATAN.call(c, 0x0032_5b80)?;
                let below = fcc(c, b.lt(zero));
                let at = fr(c, 0);
                setf(c, 12, at);
                if below {
                    c.cycles += 3;
                    let a = fr(c, 21);
                    if fcc(c, zero.lt(a)) {
                        c.cycles += 3;
                        let v = gp_float(c, literal::PI, 0);
                        let t = fr(c, 12);
                        setf(c, 12, t + v);
                    } else {
                        c.cycles += 2;
                        let v = gp_float(c, literal::PI_SUB, 0);
                        let t = fr(c, 12);
                        setf(c, 12, t - v);
                    }
                }
                c.cycles += 2;
                let t = fr(c, 12);
                setf(c, 0, t);
                if event_due(c) && deliver_event(c, STORE) {
                    return Ok(());
                }
                store_result(c, vm);
            } else {
                c.cycles += 4;
                let (a, zero) = (fr(c, 21), fr(c, 22));
                let a_is_zero = fcc(c, a.eq_s(zero));
                setf(c, 0, a);
                if a_is_zero {
                    if event_due(c) && deliver_event(c, STORE) {
                        return Ok(());
                    }
                    store_result(c, vm);
                } else {
                    c.cycles += 4;
                    let (a, zero) = (fr(c, 21), fr(c, 22));
                    let above = fcc(c, zero.le(a));
                    if !above {
                        let v = gp_float(c, literal::HALF_PI_NEG, 0);
                        Addr(dst).write_fpu(c, v);
                    } else {
                        c.cycles += 3;
                        let v = gp_float(c, literal::HALF_PI, 0);
                        Addr(dst).write_fpu(c, v);
                    }
                }
            }
        }
        // 0x0bb8: `dst = atan(a)`.
        20 => unary(c, vm, w, 0x0032_5bcc, 0x0032_5be4, &callee::MATH_ATAN, 0x0032_5bec)?,
        // 0x0bf4: `dst = sin(a)`.
        21 => unary(c, vm, w, 0x0032_5c08, 0x0032_5c20, &callee::MATH_SIN, 0x0032_5c28)?,
        // 0x0c30: `dst = cos(a)`.
        22 => unary(c, vm, w, 0x0032_5c44, 0x0032_5c5c, &callee::MATH_COS, 0x0032_5c64)?,
        // 0x0c6c: `dst = exp(a)`.
        23 => {
            c.cycles += 12;
            let dst = dst_of(c, vm, d_idx, 0x0032_5c80)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5c98)?;
            setf(c, 12, a);
            exp_into(c, vm, dst)?;
        }
        // 0x0cac: `dst = log(a)`.
        24 => {
            c.cycles += 15;
            let dst = dst_of(c, vm, d_idx, 0x0032_5cc0)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5cd8)?;
            setf(c, 12, a);
            vm.spill(c);
            call_ptr(c, callee::LOGF, 0x0032_5ce0)?;
            let v = fr(c, 0);
            Addr(dst).write_fpu(c, v);
        }
        // 0x0ce8: `dst = floor(a)`.
        25 => {
            c.cycles += 17;
            let dst = dst_of(c, vm, d_idx, 0x0032_5cfc)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5d14)?;
            let n = setf(c, 1, FpuF32::from_i32(a.to_i32()));
            if fcc(c, a.lt(n)) {
                c.cycles += 4;
                li_f(c, 0, 0x3f80_0000);
                let one = fr(c, 0);
                setf(c, 1, n - one);
                c.cycles += 2;
            }
            let v = fr(c, 1);
            Addr(dst).write_fpu(c, v);
        }
        // 0x0d3c: `dst = ceil(a)`.
        26 => {
            c.cycles += 17;
            let dst = dst_of(c, vm, d_idx, 0x0032_5d50)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5d68)?;
            let n = setf(c, 1, FpuF32::from_i32(a.to_i32()));
            if fcc(c, n.lt(a)) {
                c.cycles += 4;
                li_f(c, 0, 0x3f80_0000);
                let one = fr(c, 0);
                setf(c, 1, n + one);
                c.cycles += 2;
            }
            let v = fr(c, 1);
            Addr(dst).write_fpu(c, v);
        }
        // 0x0d90: `dst = abs(a)`.
        27 => {
            c.cycles += 14;
            let dst = dst_of(c, vm, d_idx, 0x0032_5da4)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5dbc)?;
            let v = setf(c, 0, a.abs());
            Addr(dst).write_fpu(c, v);
        }
        // 0x0dc8: `dst = sqrt(a)`.
        28 => {
            c.cycles += 14;
            let dst = dst_of(c, vm, d_idx, 0x0032_5ddc)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5df4)?;
            let v = setf(c, 0, a.sqrt());
            Addr(dst).write_fpu(c, v);
        }
        // 0x0e00: `dst = a - round(a / 2pi) * 2pi` (wrap into (-pi, pi]).
        29 => {
            c.cycles += 22;
            let dst = dst_of(c, vm, d_idx, 0x0032_5e14)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5e2c)?;
            setf(c, 2, a);
            li_f(c, 1, 0x3f00_0000);
            let inv = gp_float(c, literal::INV_TWO_PI, 0);
            let turns = setf(c, 0, a * inv);
            let half = fr(c, 1);
            let scaled = setf(c, 0, turns + half);
            let n = setf(c, 1, FpuF32::from_i32(scaled.to_i32()));
            if fcc(c, scaled.lt(n)) {
                c.cycles += 3;
                li_f(c, 0, 0x3f80_0000);
                let one = fr(c, 0);
                setf(c, 1, n - one);
            }
            c.cycles += 4;
            let two_pi = gp_float(c, literal::TWO_PI, 0);
            let n = fr(c, 1);
            let whole = setf(c, 0, n * two_pi);
            let a = fr(c, 2);
            setf(c, 0, a - whole);
            if event_due(c) && deliver_event(c, STORE) {
                return Ok(());
            }
            store_result(c, vm);
        }
        // 0x0e74: `dst = -a`.
        30 => {
            c.cycles += 13;
            let dst = dst_of(c, vm, d_idx, 0x0032_5e88)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5ea0)?;
            setf(c, 0, -a);
            if event_due(c) && deliver_event(c, STORE) {
                return Ok(());
            }
            store_result(c, vm);
        }
        // 0x0ea8: `dst = (a != 0) ? 1.0 : 0.0`.
        31 => {
            c.cycles += 16;
            let dst = dst_of(c, vm, d_idx, 0x0032_5ebc)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5ed4)?;
            setf(c, 1, FpuF32::ZERO);
            let a_is_zero = fcc(c, a.eq_s(fr(c, 1)));
            if !a_is_zero {
                let v = fr(c, 1);
                Addr(dst).write_fpu(c, v);
            } else {
                c.cycles += 2;
                li_f(c, 1, 0x3f80_0000);
                c.cycles += 2;
                let v = fr(c, 1);
                Addr(dst).write_fpu(c, v);
            }
        }
        // 0x0ef8: stop.
        32 => {
            c.cycles += 1;
            vm.stop = 1;
        }
        _ => {}
    }
    Ok(())
}

/// Opcodes 0x02/0x03: `idx += *(u16*)(ins + 2)`.
fn skip_words(c: &mut Cpu, vm: &mut Vm) {
    c.cycles += 3;
    let n = c.lhu(vm.ins.wrapping_add(2)) as u32;
    c.g[V0][0] = sx(n);
    vm.idx = vm.idx.wrapping_add(n);
}

/// Opcodes 6..11: `dst = (op1 OP op2) ? 1.0 : 0.0`, where `one_if_compare`
/// says which side of the compare stores 1.0 (0x0b tests equality and stores
/// 1.0 when it fails).
fn compare(
    c: &mut Cpu,
    vm: &mut Vm,
    w: u32,
    rets: [u32; 3],
    one_if_compare: bool,
    cond: fn(FpuF32, FpuF32) -> bool,
) -> Flow {
    c.cycles += 20;
    let dst = dst_of(c, vm, (w >> 6) & 0x3f, rets[0])?;
    vm.dst = dst;
    let a = operand(c, vm, (w >> 12) & 0x3ff, rets[1])?;
    setf(c, 20, a);
    let b = operand(c, vm, w >> 22, rets[2])?;
    let a = fr(c, 20);
    let held = fcc(c, cond(a, b));
    if held == one_if_compare {
        c.cycles += 4;
        li_f(c, 0, 0x3f80_0000);
    } else {
        setf(c, 0, FpuF32::ZERO);
    }
    let v = fr(c, 0);
    Addr(dst).write_fpu(c, v);
    Ok(())
}

/// Opcodes 0x0c..0x0f: `dst = op1 OP op2`, left in `f20`.
fn binary(
    c: &mut Cpu,
    vm: &mut Vm,
    w: u32,
    ret_dst: u32,
    ret_a: u32,
    ret_b: u32,
    op: fn(FpuF32, FpuF32) -> FpuF32,
) -> Flow {
    c.cycles += 19;
    let dst = dst_of(c, vm, (w >> 6) & 0x3f, ret_dst)?;
    vm.dst = dst;
    let a = operand(c, vm, (w >> 12) & 0x3ff, ret_a)?;
    setf(c, 20, a);
    let b = operand(c, vm, w >> 22, ret_b)?;
    let a = fr(c, 20);
    let r = setf(c, 20, op(a, b));
    Addr(dst).write_fpu(c, r);
    Ok(())
}

/// Opcodes 0x14..0x16: `dst = f(a)` through one of the maths callees.
fn unary(
    c: &mut Cpu,
    vm: &mut Vm,
    w: u32,
    ret_dst: u32,
    ret_arg: u32,
    target: &Callee,
    ret_call: u32,
) -> Flow {
    c.cycles += 15;
    let dst = dst_of(c, vm, (w >> 6) & 0x3f, ret_dst)?;
    vm.dst = dst;
    let a = operand(c, vm, (w >> 12) & 0x3ff, ret_arg)?;
    setf(c, 12, a);
    target.call(c, ret_call)?;
    let v = fr(c, 0);
    Addr(dst).write_fpu(c, v);
    Ok(())
}

/// The `0x00325c9c` tail shared by opcodes 0x12 and 0x17: `dst = exp(f12)`.
fn exp_into(c: &mut Cpu, vm: &Vm, dst: u32) -> Flow {
    c.cycles += 4;
    vm.spill(c);
    call_ptr(c, callee::EXPF, 0x0032_5ca4)?;
    let v = fr(c, 0);
    Addr(dst).write_fpu(c, v);
    Ok(())
}

// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, scramble, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 300;

    /// The parser object (`+8`: 64 virtual registers).
    const PARSER: u32 = DATA;
    /// The opcode stream.
    const STREAM: u32 = DATA + 0x1000;
    /// The object the original receives as its second argument (stored at
    /// `parser + 4`; the pad-channel operands read through it).
    const PAD: u32 = DATA + 0x2000;

    /// Eight instructions whose opcodes are chosen by the seed (every opcode
    /// lands in every position over 33 consecutive seeds), followed by
    /// opcode 0x20 padding. Skips are one word, so a run stays inside.
    fn put_code(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        let mut at = STREAM;
        // A reserved opcode (0x21..0x3f): not dispatched, the run continues.
        c.bus.write32(at, 0x21 + seed % 31);
        at += 4;
        for i in 0..8u32 {
            let op = seed.wrapping_mul(5).wrapping_add(i * 11) % 33;
            let dst = seed.wrapping_add(i * 7) & 0x3f;
            let a = seed.wrapping_add(i * 13) & 0x3f;
            let b = seed.wrapping_add(i * 29) & 0x3f;
            let mut ins = op | (dst << 6) | (a << 12);
            if op == 2 || op == 3 {
                ins |= 1 << 16; // skip one word
            } else {
                ins |= b << 22;
            }
            c.bus.write32(at, ins);
            at += 4;
            if op == 0 {
                // The immediate float. Its low bits are forced to opcode 1
                // with in-range operands, so a skip that lands on it is inert.
                let imm = (r.next_u32() & 0xffc0_0000)
                    | 1
                    | ((r.next_u32() & 0x3f) << 6)
                    | ((r.next_u32() & 0x3f) << 12);
                c.bus.write32(at, imm);
                at += 4;
            }
        }
        at
    }

    #[test]
    fn input_vm_execute_matches() {
        check(ee::s10::f_00325450__generated, input_vm_execute, SEEDS, &|c, seed| {
            let mut r = Rng(0x3254_5000 ^ (seed as u64) << 9);
            scramble(c, &mut r);
            // The game's data: the `gp` tables the float opcodes and the maths
            // callees read, and the opcode dispatch table.
            elf_image(c);
            c.pc = 0x0032_5450;
            c.g[GP][0] = sx(GAME_GP);
            for i in 0..0x40u32 {
                let v = if i % 2 == 0 { r.edge_f32() } else { r.f32(1000.0) };
                c.bus.write32(PARSER + PARSER_REGS + 4 * i, v);
            }
            c.g[A0][0] = sx(PARSER);
            c.g[A1][0] = sx(PAD);
            c.g[A2][0] = sx(STREAM);
            let mut at = put_code(c, &mut r, seed);
            // Opcode 0x20 (stop): where every run ends.
            for _ in 0..64 {
                c.bus.write32(at, OP_STOP);
                at += 4;
            }
        });
    }
}
