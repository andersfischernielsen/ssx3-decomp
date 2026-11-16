//! Staging module oc382 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc382`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0032_4678, input_parser_emit_node, "InputParser_EmitNode")];

// ---------------------------------------------------------------------------
// `InputParser_EmitNode` (cInputMapParser::compileParseNode).

/// "opcode buffer overflow".
const EMIT_NODE_MSG_OVERFLOW: u32 = 0x0048_e228;
/// "failed to allocate temporary register".
const EMIT_NODE_MSG_NO_REGISTER: u32 = 0x0048_e240;
/// "branch offset overflow".
const EMIT_NODE_MSG_BRANCH: u32 = 0x0048_e268;
/// The node-type jump table in `.rodata` (entry `type - 1`).
const EMIT_NODE_JUMP_TABLE: u32 = 0x0048_e280;
/// The jump table's targets, by node type 1..=33 (the original leaves the
/// chosen one in `a0`).
const EMIT_NODE_CASES: [u32; 33] = [
    0x0032_478c,
    0x0032_47a4,
    0x0032_4838,
    0x0032_46e8,
    0x0032_48d4,
    0x0032_4a34,
    0x0032_4a5c,
    0x0032_4a84,
    0x0032_4aac,
    0x0032_4ad4,
    0x0032_4afc,
    0x0032_4b24,
    0x0032_4b4c,
    0x0032_4b74,
    0x0032_4b9c,
    0x0032_4bc4,
    0x0032_4bec,
    0x0032_4e44,
    0x0032_4e6c,
    0x0032_4c14,
    0x0032_4c3c,
    0x0032_4c64,
    0x0032_4c8c,
    0x0032_4cb4,
    0x0032_4cdc,
    0x0032_4d04,
    0x0032_4d2c,
    0x0032_4d54,
    0x0032_4d7c,
    0x0032_4da4,
    0x0032_4dcc,
    0x0032_4df4,
    0x0032_4e1c,
];
/// The temporary-register pool lives at `self + 0x10000`: the free list
/// (`int FreeRegisters[]`) and its count.
pub(crate) const EMIT_NODE_REGS_BASE: u32 = 0x1_0000;
pub(crate) const EMIT_NODE_FREE_REGS: u32 = 0x8ec;
pub(crate) const EMIT_NODE_NUM_FREE: u32 = 0x9e8;
/// Parse node fields: type, first operand / value, second operand /
/// modifier, third operand (the `else` branch).
pub(crate) const EMIT_NODE_TYPE: u32 = 0x0;
pub(crate) const EMIT_NODE_A: u32 = 0x4;
pub(crate) const EMIT_NODE_B: u32 = 0x8;
pub(crate) const EMIT_NODE_C: u32 = 0xc;
/// A variable's expression node.
pub(crate) const EMIT_NODE_VAR_EXPR: u32 = 0x40;
/// "Use the default modifier" (node field `b`).
const EMIT_NODE_DEFAULT_MODIFIER: u64 = 5;
/// Low 6 bits of an instruction word: the opcode.
const EMIT_NODE_OP_MASK: u64 = 0xffff_ffff_ffff_ffc0;

const EMIT_NODE_ERROR: Callee = Callee {
    addr: 0x0032_1500,
    f: ee::s10::InputParser_Error,
};
const EMIT_NODE_RECURSE: Callee = Callee {
    addr: 0x0032_4678,
    f: ee::s10::InputParser_EmitNode,
};
const EMIT_NODE_PAD_OPERAND: Callee = Callee {
    addr: 0x0032_1ad8,
    f: ee::s10::InputParser_PadChannelOperand,
};
const EMIT_NODE_BUILTIN_OPERAND: Callee = Callee {
    addr: 0x0032_1ad0,
    f: ee::s10::InputParser_BuiltinOperand,
};
const EMIT_NODE_BINARY: Callee = Callee {
    addr: 0x0032_4ec8,
    f: ee::s10::InputParser_EmitBinaryOp,
};
const EMIT_NODE_UNARY: Callee = Callee {
    addr: 0x0032_50d8,
    f: ee::s10::InputParser_EmitUnaryOp,
};

/// How a node case leaves: through `v0 = -1` (0x324e94), straight to the
/// epilogue (0x324e98), or with `s0` already restored (0x324e9c).
#[derive(Clone, Copy, PartialEq)]
enum EmitNodeExit {
    Fail,
    Done,
    S0Restored,
}

/// `int InputParser_EmitNode(InputParser* self, ParseNode* node, int* dst,
/// InputOp* code, int cap, int defaultModifier)` (0x324678,
/// `cInputMapParser::compileParseNode`): compile one expression node of an
/// input map into input-VM opcodes at `code` (at most `cap` words), the
/// result in register `*dst` (allocated from the parser's temporary pool when
/// negative; `-1` lets operand nodes return their operand id instead).
/// Returns the number of words written, or -1 after reporting an error.
/// Variables (type 1) loop on their expression, conditionals (5) emit
/// `cond; jz; then; jmp; else`, types 6..33 go to the binary/unary emitters.
pub fn input_parser_emit_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x80);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x70);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S4, 0x60);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x50);
        c.g[S4][0] = c.g[T0][0];
        frame.save(c, S6, 0x40);
        c.g[S5][0] = c.g[T1][0];
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        let exit = emit_node_cases(c)?;
        if exit == EmitNodeExit::Fail {
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
        }
        if exit != EmitNodeExit::S0Restored {
            c.cycles += 1;
            frame.restore(c, S0, 0xa0);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `InputParser_Error(self, msg)` returning to `ret`, then `v0 = -1`.
fn emit_node_report(c: &mut Cpu, msg: u32, ret: u32) -> Result<EmitNodeExit, Unwound> {
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = sx(msg);
    EMIT_NODE_ERROR.call(c, ret)?;
    c.g[V0][0] = u64::MAX;
    Ok(EmitNodeExit::Done)
}

/// The "opcode buffer overflow" block (0x324748) reached from a later
/// block: a back edge with an event check.
fn emit_node_overflow_back(c: &mut Cpu) -> Result<EmitNodeExit, Unwound> {
    if event_due(c) {
        deliver_event_here(c, 0x0032_4748);
    }
    c.cycles += 6;
    emit_node_report(c, EMIT_NODE_MSG_OVERFLOW, 0x0032_4758)
}

/// `(cap as i64) <= 0` on the 64-bit `s4`.
fn emit_node_full(c: &Cpu) -> bool {
    (c.g[S4][0] as i64) <= 0
}

/// The node-type switch (0x3246bc), looping on variable nodes.
fn emit_node_cases(c: &mut Cpu) -> Result<EmitNodeExit, Unwound> {
    loop {
        c.cycles += 5;
        let node = r32(c, S1);
        let ty = Addr(node.wrapping_add(EMIT_NODE_TYPE)).lw(c);
        let index = sx((ty as u32).wrapping_sub(1));
        c.g[A0][0] = index;
        c.g[V1][0] = (index < 0x21) as u64;
        c.g[V0][0] = 0x49_0000;
        if index >= 0x21 {
            return Ok(EmitNodeExit::Fail);
        }
        c.cycles += 6;
        let entry = EMIT_NODE_JUMP_TABLE.wrapping_add((index as u32) << 2);
        c.g[V0][0] = sx(EMIT_NODE_JUMP_TABLE);
        c.g[V1][0] = sx(entry);
        let case = EMIT_NODE_CASES[index as usize];
        c.g[A0][0] = sx(case);
        let ty = ty as u32;
        match ty {
            1 => {
                // Variable: compile its expression with the variable's
                // modifier unless that says "default".
                c.cycles += 6;
                let modifier = Addr(node.wrapping_add(EMIT_NODE_B)).lw(c);
                c.g[V0][0] = modifier;
                let var = Addr(node.wrapping_add(EMIT_NODE_A)).lw(c);
                c.g[A0][0] = var;
                c.g[V1][0] = modifier ^ EMIT_NODE_DEFAULT_MODIFIER;
                if modifier != EMIT_NODE_DEFAULT_MODIFIER {
                    c.g[S5][0] = modifier;
                }
                c.g[S1][0] = Addr((var as u32).wrapping_add(EMIT_NODE_VAR_EXPR)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0032_46bc);
                }
                continue;
            }
            2 => return emit_node_pad_channel(c),
            3 => return emit_node_builtin(c),
            4 => return emit_node_constant(c),
            5 => return emit_node_conditional(c),
            6..=17 => return emit_node_operator(c, EMIT_NODE_BINARY, ty - 2, case),
            20..=23 => return emit_node_operator(c, EMIT_NODE_BINARY, ty - 4, case),
            24..=33 => return emit_node_operator(c, EMIT_NODE_UNARY, ty - 4, case),
            _ => return emit_node_operator(c, EMIT_NODE_UNARY, ty + 12, case),
        }
    }
}

/// Types 6..33: `EmitBinaryOp`/`EmitUnaryOp(self, node, dst, code, cap,
/// op, defaultModifier)` and return its result.
fn emit_node_operator(
    c: &mut Cpu,
    emitter: Callee,
    op: u32,
    case: u32,
) -> Result<EmitNodeExit, Unwound> {
    c.cycles += 10;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = c.g[S3][0];
    c.g[A3][0] = c.g[S2][0];
    c.g[T0][0] = c.g[S4][0];
    c.g[T2][0] = c.g[S5][0];
    c.g[T1][0] = op as u64;
    emitter.call(c, case + 0x20)?;
    let frame = Frame { size: 0xb0 };
    frame.restore(c, S0, 0xa0);
    Ok(EmitNodeExit::S0Restored)
}

/// Type 4, a constant: allocate `*dst` from the free list when negative,
/// then emit `load dst` and the float value (2 words).
fn emit_node_constant(c: &mut Cpu) -> Result<EmitNodeExit, Unwound> {
    c.cycles += 3;
    let dst_ptr = r32(c, S3);
    let dst = Addr(dst_ptr).lw(c);
    c.g[V0][0] = ((c.g[S4][0] as i64) < 2) as u64;
    if (dst as i64) < 0 {
        c.cycles += 5;
        let pool = r32(c, S0).wrapping_add(EMIT_NODE_REGS_BASE);
        c.g[V0][0] = 0x1_0000;
        c.g[A0][0] = sx(pool);
        let count = Addr(pool.wrapping_add(EMIT_NODE_NUM_FREE)).lw(c);
        let left = sx((count as u32).wrapping_sub(1));
        c.g[V0][0] = left;
        if (count as i64) > 0 {
            c.cycles += 5;
            Addr(pool.wrapping_add(EMIT_NODE_NUM_FREE)).sw(c, left);
            let slot = pool.wrapping_add((left as u32) << 2);
            c.g[V1][0] = sx(slot);
            let reg = Addr(slot.wrapping_add(EMIT_NODE_FREE_REGS)).lw(c);
            c.g[V0][0] = reg;
            Addr(dst_ptr).sw(c, reg);
        }
        c.cycles += 3;
        let dst = Addr(dst_ptr).lw(c);
        c.g[V0][0] = ((c.g[S4][0] as i64) < 2) as u64;
        if (dst as i64) < 0 {
            c.cycles += 6;
            return emit_node_report(c, EMIT_NODE_MSG_NO_REGISTER, 0x0032_4738);
        }
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.cycles += 6;
        return emit_node_report(c, EMIT_NODE_MSG_OVERFLOW, 0x0032_4758);
    }
    let code = r32(c, S2);
    let word = Addr(code).lw(c);
    c.cycles += 11;
    let word = word & EMIT_NODE_OP_MASK;
    c.g[A0][0] = word;
    Addr(code).sw(c, word);
    c.g[V0][0] = 2;
    let dst_field = sx(((Addr(r32(c, S3)).lw(c) & 0x3f) as u32) << 6);
    c.g[V1][0] = dst_field;
    Addr(code).sw(c, dst_field);
    let value = r32(c, S1).wrapping_add(EMIT_NODE_A);
    c.lwc1(0, value);
    c.swc1(0, code.wrapping_add(4));
    Ok(EmitNodeExit::Done)
}

/// Type 2, a pad channel: its operand id; with `*dst == -1` that id
/// becomes the result, else emit `move dst, id` (1 word).
fn emit_node_pad_channel(c: &mut Cpu) -> Result<EmitNodeExit, Unwound> {
    c.cycles += 11;
    let node = r32(c, S1);
    let modifier = Addr(node.wrapping_add(EMIT_NODE_B)).lw(c);
    c.g[A2][0] = modifier;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = Addr(node.wrapping_add(EMIT_NODE_A)).lw(c);
    c.g[V0][0] = modifier ^ EMIT_NODE_DEFAULT_MODIFIER;
    if modifier == EMIT_NODE_DEFAULT_MODIFIER {
        c.g[A2][0] = c.g[S5][0];
    }
    EMIT_NODE_PAD_OPERAND.call(c, 0x0032_47bc)?;
    c.g[A2][0] = c.g[V0][0];
    let dst = Addr(r32(c, S3)).lw(c);
    c.g[V1][0] = dst;
    c.g[V0][0] = 0;
    if dst == u64::MAX {
        c.cycles += 2;
        let operand = c.g[A2][0];
        Addr(r32(c, S3)).sw(c, operand);
        return Ok(EmitNodeExit::Done);
    }
    c.cycles += 2;
    c.g[V0][0] = EMIT_NODE_OP_MASK;
    if emit_node_full(c) {
        return emit_node_overflow_back(c);
    }
    c.cycles += 22;
    let code = r32(c, S2);
    let word = Addr(code).lw(c);
    c.g[A0][0] = 0xffff_ffff_ffff_f03f;
    let word = (word & EMIT_NODE_OP_MASK) | 1;
    c.g[A3][0] = 0xffff_ffff_ffc0_0fff;
    Addr(code).sw(c, word);
    let src_field = sx(((c.g[A2][0] & 0x3ff) as u32) << 12);
    c.g[A2][0] = src_field;
    let word = word & 0xffff_ffff_ffff_f03f;
    c.g[A1][0] = 0x3f_ffff;
    let dst_field = sx(((Addr(r32(c, S3)).lw(c) & 0x3f) as u32) << 6);
    c.g[A0][0] = dst_field;
    c.g[V0][0] = 1;
    let word = (((word | dst_field) & 0xffff_ffff_ffc0_0fff) | src_field) & 0x3f_ffff;
    c.g[V1][0] = word;
    Addr(code).sw(c, word);
    Ok(EmitNodeExit::Done)
}

/// Type 3, a built-in value: like the pad channel, with the operand id from
/// `InputParser_BuiltinOperand(self, index)`.
fn emit_node_builtin(c: &mut Cpu) -> Result<EmitNodeExit, Unwound> {
    c.cycles += 4;
    let dst = Addr(r32(c, S3)).lw(c);
    c.g[V1][0] = dst;
    c.g[V0][0] = u64::MAX;
    if dst == u64::MAX {
        c.cycles += 6;
        c.g[A1][0] = Addr(r32(c, S1).wrapping_add(EMIT_NODE_A)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        EMIT_NODE_BUILTIN_OPERAND.call(c, 0x0032_4854)?;
        let operand = c.g[V0][0];
        Addr(r32(c, S3)).sw(c, operand);
        c.g[V0][0] = 0;
        return Ok(EmitNodeExit::Done);
    }
    c.cycles += 2;
    c.g[V0][0] = EMIT_NODE_OP_MASK;
    if emit_node_full(c) {
        return emit_node_overflow_back(c);
    }
    c.cycles += 27;
    let code = r32(c, S2);
    let word = Addr(code).lw(c);
    c.g[A1][0] = 0xffff_ffff_ffff_f03f;
    c.g[A0][0] = c.g[S0][0];
    let word = (word & EMIT_NODE_OP_MASK) | 1;
    Addr(code).sw(c, word);
    let word = word & 0xffff_ffff_ffff_f03f;
    let dst_field = sx(((Addr(r32(c, S3)).lw(c) & 0x3f) as u32) << 6);
    c.g[V0][0] = dst_field;
    let word = word | dst_field;
    c.g[V1][0] = word;
    Addr(code).sw(c, word);
    c.g[A1][0] = Addr(r32(c, S1).wrapping_add(EMIT_NODE_A)).lw(c);
    EMIT_NODE_BUILTIN_OPERAND.call(c, 0x0032_48a0)?;
    let code = r32(c, S2);
    let word = Addr(code).lw(c);
    let src_field = sx(((c.g[V0][0] & 0x3ff) as u32) << 12);
    let word = ((word & 0xffff_ffff_ffc0_0fff) | src_field) & 0x3f_ffff;
    c.g[V1][0] = 0x3f_ffff;
    c.g[V0][0] = 1;
    c.g[A0][0] = word;
    Addr(code).sw(c, word);
    Ok(EmitNodeExit::Done)
}

/// Type 5, a conditional: compile the condition into a temporary (`cond` at
/// `0(sp)`), emit `jz cond, +n` (op 2) and free `cond`, the `then` branch,
/// `jmp +m` (op 3), the `else` branch; patch both offsets (16 bits, at +2).
fn emit_node_conditional(c: &mut Cpu) -> Result<EmitNodeExit, Unwound> {
    c.cycles += 12;
    c.g[V0][0] = u64::MAX;
    c.g[A1][0] = Addr(r32(c, S1).wrapping_add(EMIT_NODE_A)).lw(c);
    let cond_slot = sp(c);
    Addr(cond_slot).sw(c, u64::MAX);
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[SP][0];
    c.g[A3][0] = c.g[S2][0];
    c.g[T0][0] = c.g[S4][0];
    c.g[T1][0] = c.g[S5][0];
    EMIT_NODE_RECURSE.call(c, 0x0032_48f8)?;
    c.g[FP][0] = c.g[V0][0];
    c.g[V0][0] = u64::MAX;
    if (c.g[FP][0] as i64) < 0 {
        return Ok(EmitNodeExit::Done);
    }
    // code += n1; cap -= n1.
    c.cycles += 4;
    let n1 = c.g[FP][0] as u32;
    c.g[V0][0] = sx(n1 << 2);
    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(n1));
    c.g[S2][0] = sx(r32(c, S2).wrapping_add(n1 << 2));
    if emit_node_full(c) {
        return emit_node_overflow_back(c);
    }
    // jz cond.
    c.cycles += 14;
    let code = r32(c, S2);
    let word = Addr(code).lw(c);
    c.g[V1][0] = EMIT_NODE_OP_MASK;
    let cond = Addr(sp(c)).lw(c);
    c.g[A1][0] = cond;
    c.g[S7][0] = 0xffff_ffff_ffff_003f;
    let reg_field = sx(((cond & 0x3ff) as u32) << 6);
    c.g[V1][0] = reg_field;
    let word = (((word & EMIT_NODE_OP_MASK) | 2) & c.g[S7][0]) | reg_field;
    c.g[V0][0] = word;
    let temporary = cond < 0x40;
    c.g[A0][0] = temporary as u64;
    Addr(code).sw(c, word);
    if temporary {
        // Give the condition's temporary back to the pool.
        c.cycles += 8;
        let pool = r32(c, S0).wrapping_add(EMIT_NODE_REGS_BASE);
        c.g[A0][0] = sx(pool);
        let count = Addr(pool.wrapping_add(EMIT_NODE_NUM_FREE)).lw(c);
        let slot = pool.wrapping_add((count as u32) << 2);
        c.g[V0][0] = sx(slot);
        c.g[V1][0] = sx((count as u32).wrapping_add(1));
        Addr(slot.wrapping_add(EMIT_NODE_FREE_REGS)).sw(c, cond);
        let count = c.g[V1][0];
        Addr(pool.wrapping_add(EMIT_NODE_NUM_FREE)).sw(c, count);
    }
    // The `then` branch after the jz.
    c.cycles += 11;
    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(1));
    c.g[A1][0] = Addr(r32(c, S1).wrapping_add(EMIT_NODE_B)).lw(c);
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S3][0];
    c.g[A3][0] = sx(r32(c, S2).wrapping_add(4));
    c.g[T0][0] = c.g[S4][0];
    c.g[T1][0] = c.g[S5][0];
    EMIT_NODE_RECURSE.call(c, 0x0032_498c)?;
    c.g[S6][0] = c.g[V0][0];
    let n2 = c.g[S6][0];
    c.g[V1][0] = sx((n2 as u32).wrapping_add(1));
    if (n2 as i64) < 0 {
        return Ok(EmitNodeExit::Fail);
    }
    c.cycles += 4;
    c.g[A0][0] = 0xffff;
    let too_far = 0xffff < (c.g[V1][0] as i64);
    c.g[V0][0] = sx((n2 as u32) << 2);
    if too_far {
        c.cycles += 1 + 5;
        return emit_node_report(c, EMIT_NODE_MSG_BRANCH, 0x0032_49b8);
    }
    // Patch the jz offset; code += n2 + 1; cap -= n2.
    c.cycles += 5;
    let code = r32(c, S2);
    let offset = c.g[V1][0];
    Addr(code.wrapping_add(2)).sh(c, offset);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S6][0] as u32));
    c.g[S2][0] = sx(code.wrapping_add(c.g[V0][0] as u32));
    if emit_node_full(c) {
        return emit_node_overflow_back(c);
    }
    // jmp, then the `else` branch.
    c.cycles += 16;
    let code = r32(c, S2);
    let word = Addr(code).lw(c);
    c.g[V1][0] = EMIT_NODE_OP_MASK;
    c.g[A2][0] = c.g[S3][0];
    c.g[T0][0] = sx((c.g[S4][0] as u32).wrapping_sub(1));
    c.g[T1][0] = c.g[S5][0];
    let word = ((word & EMIT_NODE_OP_MASK) | 3) & c.g[S7][0];
    c.g[V0][0] = word;
    c.g[A0][0] = c.g[S0][0];
    c.g[A3][0] = sx(code.wrapping_add(4));
    Addr(code).sw(c, word);
    c.g[A1][0] = Addr(r32(c, S1).wrapping_add(EMIT_NODE_C)).lw(c);
    EMIT_NODE_RECURSE.call(c, 0x0032_4a08)?;
    let n3 = c.g[V0][0];
    c.g[V1][0] = n3;
    c.g[A0][0] = 0xffff;
    if (n3 as i64) < 0 {
        return Ok(EmitNodeExit::Fail);
    }
    c.cycles += 3;
    let too_far = 0xffff < (n3 as i64);
    c.g[V0][0] = too_far as u64;
    c.g[A1][0] = 0x49_0000;
    if too_far {
        if event_due(c) {
            deliver_event_here(c, 0x0032_49ac);
        }
        c.cycles += 5;
        return emit_node_report(c, EMIT_NODE_MSG_BRANCH, 0x0032_49b8);
    }
    // Patch the jmp offset; n1 + n2 + n3 + 2.
    c.cycles += 5;
    let total = (c.g[FP][0] as u32).wrapping_add(c.g[S6][0] as u32);
    Addr(r32(c, S2).wrapping_add(2)).sh(c, n3);
    c.g[V0][0] = sx(total.wrapping_add(n3 as u32).wrapping_add(2));
    Ok(EmitNodeExit::Done)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, scramble, Rng, DATA, GAME_GP};

    /// The parser (`self`), the result register `*dst`, the opcode buffer
    /// and the parse-node pool (0x80 bytes per node, room for a variable's
    /// expression at +0x40).
    const IPEN_SELF: u32 = DATA;
    const IPEN_DST: u32 = DATA + 0x200;
    const IPEN_CODE: u32 = DATA + 0x400;
    const IPEN_NODES: u32 = DATA + 0x1000;
    const IPEN_MAX_NODES: u32 = 0x100;

    /// Build a random parse tree of at most `depth` levels; returns the root.
    fn ipen_node(c: &mut Cpu, r: &mut Rng, next: &mut u32, depth: u32, ty: Option<u32>) -> u32 {
        let at = IPEN_NODES + 0x80 * *next;
        *next += 1;
        let ty = ty.unwrap_or_else(|| {
            if depth == 0 || *next + 8 > IPEN_MAX_NODES {
                [2, 3, 4, 4, 2, 3, 4, 0][(r.next_u32() % 8) as usize]
            } else {
                match r.next_u32() % 8 {
                    0 => 1,
                    1 => 5,
                    2 => 6 + r.next_u32() % 28,
                    _ => 2 + r.next_u32() % 3,
                }
            }
        });
        c.bus.write32(at + EMIT_NODE_TYPE, ty);
        let modifier = if r.next_u32() % 2 == 0 {
            5
        } else {
            r.next_u32() % 5
        };
        match ty {
            1 => {
                let var = IPEN_NODES + 0x80 * *next;
                *next += 1;
                let sub = depth.saturating_sub(1);
                let expr = ipen_node(c, r, next, sub, None);
                c.bus.write32(var + EMIT_NODE_VAR_EXPR, expr);
                c.bus.write32(at + EMIT_NODE_A, var);
                c.bus.write32(at + EMIT_NODE_B, modifier);
            }
            2 => {
                c.bus.write32(at + EMIT_NODE_A, r.next_u32() % 8);
                c.bus.write32(at + EMIT_NODE_B, modifier);
            }
            3 => {
                // Mostly real built-ins; sometimes an index giving an id
                // below 0x40 (a "temporary" the conditional frees).
                let index = if r.next_u32() % 4 == 0 {
                    (r.next_u32() % 0x40).wrapping_sub(0x40)
                } else {
                    r.next_u32() % 0x20
                };
                c.bus.write32(at + EMIT_NODE_A, index);
            }
            4 => {
                let value = r.f32(100.0);
                c.bus.write32(at + EMIT_NODE_A, value);
            }
            5..=33 => {
                let sub = depth.saturating_sub(1);
                for off in [EMIT_NODE_A, EMIT_NODE_B, EMIT_NODE_C] {
                    let child = ipen_node(c, r, next, sub, None);
                    c.bus.write32(at + off, child);
                }
            }
            _ => {}
        }
        at
    }

    /// Every node type (1..=33 and the out-of-range ones) at the root, on
    /// random trees: variables (chained, default and own modifiers), pad
    /// channels and built-ins with `*dst == -1` or a register, constants with
    /// the register pool full, empty or holding no register, conditionals
    /// freeing a temporary or not, every "opcode buffer overflow" with small
    /// `cap`, failures in each branch; the binary/unary emitters and the
    /// recursion run the generated bodies. Not reached: "branch offset
    /// overflow" (needs a branch of 0xffff words).
    #[test]
    fn input_parser_emit_node_matches() {
        check(
            ee::s10::f_00324678__generated,
            input_parser_emit_node,
            300,
            &|c, seed| {
                let mut r = Rng(0x0032_4678 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                elf_image(c);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.pc = 0x0032_4678;
                // The parser: file name and line for the error message, the
                // temporary register pool.
                for (i, b) in b"map.txt\0".iter().enumerate() {
                    c.bus.write8(IPEN_SELF + 0xc + i as u32, *b);
                }
                c.bus.write32(IPEN_SELF + 0x8c, r.next_u32() % 500);
                let pool = IPEN_SELF + EMIT_NODE_REGS_BASE;
                let free = r.next_u32() % 5;
                for i in 0..free {
                    let reg = if r.next_u32() % 6 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 0x40
                    };
                    c.bus.write32(pool + EMIT_NODE_FREE_REGS + 4 * i, reg);
                }
                c.bus.write32(pool + EMIT_NODE_NUM_FREE, free);
                // The opcode buffer: junk the emitter masks.
                for i in 0..0x40 {
                    c.bus.write32(IPEN_CODE + 4 * i, r.next_u32());
                }
                let dst = match r.next_u32() % 5 {
                    0 | 1 => u32::MAX,
                    2 => 0xffff_fffe,
                    3 => r.next_u32() % 0x40,
                    _ => 0x50,
                };
                c.bus.write32(IPEN_DST, dst);
                let root_type = match seed % 37 {
                    n @ 0..=32 => n + 1,
                    33 => 0,
                    34 => 34,
                    _ => 1 + r.next_u32() % 5,
                };
                let mut next = 0;
                let root = ipen_node(c, &mut r, &mut next, 2, Some(root_type));
                let cap = [0u32, 1, 2, 3, 4, 6, 10, 40, 40][(r.next_u32() % 9) as usize];
                let default_modifier = r.next_u32() % 6;
                c.g[A0][0] = sx(IPEN_SELF);
                c.g[A1][0] = sx(root);
                c.g[A2][0] = sx(IPEN_DST);
                c.g[A3][0] = sx(IPEN_CODE);
                c.g[T0][0] = sx(cap);
                c.g[T1][0] = sx(default_modifier);
            },
        );
    }
}
