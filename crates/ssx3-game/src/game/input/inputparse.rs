//! `src/input/inputparse.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::aianimeventmap::BUILTIN_DELETE;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_5;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_4;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festatetrophyroom::r;
pub(crate) use crate::game::fe::festatetrophyroom::Ops;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::intersect::worldsphtree::p;
pub(crate) use crate::game::misc::add32;
pub(crate) use crate::game::misc::MEMCPY;
pub(crate) use crate::game::misc::PAD_PORTS;
pub(crate) use crate::game::misc::PAD_PORT_SIZE;
pub(crate) use crate::game::misc::PAD_SLOT_SIZE;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::visualfx::worldlightman::deleting_dtor;
pub(crate) use crate::game::world::worldview::p_2 as p_3;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0032_17d0,
        input_parser_lookup_modifier,
        "InputParser_LookupModifier",
    ),
    (
        0x0032_1a40,
        input_parser_find_identifier,
        "InputParser_FindIdentifier",
    ),
    (
        0x0032_1ad0,
        input_parser_builtin_operand,
        "InputParser_BuiltinOperand",
    ),
    (
        0x0032_1ad8,
        input_parser_pad_channel_operand,
        "InputParser_PadChannelOperand",
    ),
    (
        0x0032_45a8,
        input_parser_compile_identifier,
        "InputParser_CompileIdentifier",
    ),
    (0x0032_5250, input_vm_init, "InputVM_Init"),
    (0x0032_5260, input_vm_read_operand, "InputVM_ReadOperand"),
    (0x0032_52e8, input_vm_get_reg_ptr, "InputVM_GetRegPtr"),
    (0x0032_5430, input_vm_read_builtin, "InputVM_ReadBuiltin"),
    (0x0032_5450, input_vm_execute, "InputVM_Execute"),
    (
        0x0032_6308,
        input_preproc_write_string,
        "InputPreproc_WriteString",
    ),
    (0x0032_6360, input_preproc_write, "InputPreproc_Write"),
    (
        0x0032_6430,
        input_preproc_emit_line_marker,
        "InputPreproc_EmitLineMarker",
    ),
    (
        0x0032_6478,
        input_preproc_next_char,
        "InputPreproc_NextChar",
    ),
    (
        0x0032_65c0,
        input_preproc_skip_line_comment,
        "InputPreproc_SkipLineComment",
    ),
    (
        0x0032_6678,
        input_preproc_skip_line,
        "InputPreproc_SkipLine",
    ),
    (0x0032_6a28, pad_queue_init, "PadQueue_Init"),
    (0x0032_6a68, pad_queue_shutdown, "PadQueue_Shutdown"),
    (0x0032_6c60, pad_queue_flush, "PadQueue_Flush"),
    (
        0x0032_6cf0,
        pad_device_stop_actuators,
        "PadDevice_StopActuators",
    ),
    (0x0032_6d60, ps2_pad_create, "PS2Pad_Create"),
    (0x0032_6df0, ps2_pad_ctor, "PS2Pad_ctor"),
    (0x0032_6e50, ps2_pad_dtor, "PS2Pad_dtor"),
    (0x0032_7740, ps2pad_set_actuator, "PS2Pad_SetActuator"),
    (0x0032_77d0, pad_device_dtor, "PadDevice_dtor"),
    (0x0032_7810, ai_fwd_diff_cache_ctor, "AIFwdDiffCache_ctor"),
    (0x0032_7828, ai_fwd_diff_cache_dtor, "AIFwdDiffCache_dtor"),
    (0x0032_6eb0, ps2_pad_update, "PS2Pad_Update"),
    (
        0x0032_52f8,
        input_vm_read_pad_channel,
        "InputVM_ReadPadChannel",
    ),
    (0x0032_6b88, pad_queue_poll, "PadQueue_Poll"),
    (0x003f_fa58, sce_pad_read, "scePadRead"),
    (0x003f_fbc0, sce_pad_get_state, "scePadGetState"),
    (
        0x0032_1880,
        input_parser_lookup_builtin_name,
        "InputParser_LookupBuiltinName",
    ),
    (
        0x0032_1af0,
        input_parser_next_token,
        "InputParser_NextToken",
    ),
    (
        0x0032_36d8,
        input_parser_parse_comparison,
        "InputParser_ParseComparison",
    ),
    (
        0x0032_3900,
        input_parser_parse_additive,
        "InputParser_ParseAdditive",
    ),
    (
        0x0032_3a68,
        input_parser_parse_multiplicative,
        "InputParser_ParseMultiplicative",
    ),
    (
        0x0032_3bd8,
        input_parser_parse_unary,
        "InputParser_ParseUnary",
    ),
    (0x0032_4678, input_parser_emit_node, "InputParser_EmitNode"),
    (
        0x0032_30e0,
        input_parser_parse_statement,
        "InputParser_ParseStatement",
    ),
    (
        0x0032_3268,
        input_parser_parse_conditional,
        "InputParser_ParseConditional",
    ),
    (
        0x0032_33b8,
        input_parser_parse_logical_or,
        "InputParser_ParseLogicalOr",
    ),
    (
        0x0032_3540,
        input_parser_parse_logical_and,
        "InputParser_ParseLogicalAnd",
    ),
    (
        0x0032_4138,
        input_parser_parse_function_2,
        "InputParser_ParseFunction2",
    ),
    (
        0x0032_4ec8,
        input_parser_emit_binary_op,
        "InputParser_EmitBinaryOp",
    ),
    (
        0x0032_50d8,
        input_parser_emit_unary_op,
        "InputParser_EmitUnaryOp",
    ),
    (
        0x0032_6150,
        input_preproc_process_text,
        "InputPreproc_ProcessText",
    ),
    (
        0x0032_4398,
        input_parser_parse_function1,
        "InputParser_ParseFunction1",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_EMPTY_STUB;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_NEXT_CHAR;

    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;

    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_PAD_CTOR;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::PS2_PAD_CREATE;

    pub(crate) use crate::game::intersect::worldsphtree::callee::INPUT_VM_READ_BUILTIN;
    pub(crate) use crate::game::intersect::worldsphtree::callee::INPUT_VM_READ_PAD_CHANNEL;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_PREPROC_WRITE;

    pub(crate) use crate::game::world::wscriptcache::callee::INPUT_PREPROC_WRITE_STRING;

    pub(crate) use crate::game::world::wscriptcache::callee::SPRINTF_2;

    pub const GET_REG_PTR: Callee = Callee {
        addr: 0x0032_52e8,
        f: ee::s10::InputVM_GetRegPtr,
    };

    pub const READ_OPERAND: Callee = Callee {
        addr: 0x0032_5260,
        f: ee::s10::InputVM_ReadOperand,
    };

    pub const MATH_ATAN: Callee = Callee {
        addr: 0x0031_c228,
        f: ee::s10::Math_Atan,
    };

    pub const MATH_SIN: Callee = Callee {
        addr: 0x0031_bf60,
        f: ee::s10::Math_Sin,
    };

    pub const MATH_COS: Callee = Callee {
        addr: 0x0031_c040,
        f: ee::s10::Math_Cos,
    };

    /// `logf` and `expf`: cross-shard, so the original reaches them through
    /// the function table.
    pub const LOGF: u32 = 0x0040_da10;

    pub const EXPF: u32 = 0x0040_d758;
}

/// `gp`-relative literals the expression opcodes read: the atan2 half-plane
/// corrections (0x13) and the angle wrap (0x1d).
pub(crate) mod literal {

    pub const HALF_PI_NEG: i32 = -0x2e24;

    pub const HALF_PI: i32 = -0x2e20;

    pub const PI: i32 = -0x2e1c;

    pub const PI_SUB: i32 = -0x2e18;

    pub const INV_TWO_PI: i32 = -0x2e14;

    pub const TWO_PI: i32 = -0x2e10;
}

// ---------------------------------------------------------------------------
// Input system.

/// `int InputParser_PadChannelOperand(int player, int channel, int base)`
/// (0x321ad8): `base + player + channel * 6 + 0x60`.
pub fn input_parser_pad_channel_operand(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = 0x6;
    let scaled = abi::mult(c, abi::arg(c, 1), 6);
    c.g[A0][0] = sx(scaled);
    let base = scaled.wrapping_add(abi::arg(c, 2));
    c.g[V0][0] = sx(base.wrapping_add(0x60));
    abi::ret(c);
}

/// `void InputVM_Init(InputVM *vm)` (0x325250): clear the first word and
/// return the object.
pub fn input_vm_init(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    Addr(abi::arg(c, 0)).sw(c, 0);
    abi::ret(c);
}

/// `float* InputVM_GetRegPtr(InputVM *vm, int index)` (0x3252e8): the register
/// slot at `vm + index*4 + 8`.
pub fn input_vm_get_reg_ptr(c: &mut Cpu) {
    c.cycles += 4;
    let off = abi::arg(c, 1).wrapping_shl(2).wrapping_add(8);
    c.g[V0][0] = sx(abi::arg(c, 0).wrapping_add(off));
    abi::ret(c);
}

/// `float InputVM_ReadBuiltin(InputVM *vm, int index)` (0x32525430): 0.0 for
/// index 0, otherwise the builtin float at `vm + index*4 + 0x108`.
pub fn input_vm_read_builtin(c: &mut Cpu) {
    c.cycles += 2;
    let index = abi::arg64(c, 1);
    let off = abi::arg(c, 1).wrapping_shl(2);
    c.g[V0][0] = sx(off);
    if index == 0 {
        c.cycles += 3;
        c.f[0] = 0;
    } else {
        c.cycles += 3;
        let ptr = abi::arg(c, 0).wrapping_add(off);
        c.g[V0][0] = sx(ptr);
        c.f[0] = Addr(ptr).off(0x108).read_fpu(c).0;
    }
    abi::ret(c);
}

/// `int InputPreproc_NextChar(InputPreproc *p, InputPreproc* /*cursor*/)`
/// (0x326478): advance the read cursor past the byte at the end of the
/// buffered range; store the byte (or -1) in the output word.
pub fn input_preproc_next_char(c: &mut Cpu) {
    c.cycles += 6;
    let out = Addr(abi::arg(c, 0));
    let cur = Addr(abi::arg(c, 1));
    let start = cur.lw(c);
    let len = cur.off(4).lw(c);
    let ptr = cur.off(8).lw(c);
    let end = sx((start as u32).wrapping_add(len as u32));
    c.g[V1][0] = len;
    c.g[A2][0] = ptr;
    c.g[V0][0] = end;
    if ptr != end {
        let byte = Addr(ptr as u32).lbu(c);
        c.cycles += 4;
        c.g[V0][0] = byte;
        let next = sx((ptr as u32).wrapping_add(1));
        c.g[V1][0] = next;
        out.off(0xc).sw(c, byte);
        cur.off(8).sw(c, next);
    } else {
        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        out.off(0xc).sw(c, u64::MAX);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Pad queue.

/// `int PadQueue_Flush(PadQueue *q)` (0x326c60): wrap the write cursor at
/// `+0x2ee4` by 30 and return the remainder of that against the count at
/// `+0x2ee0`.
pub fn pad_queue_flush(c: &mut Cpu) {
    c.cycles += 4;
    let q = Addr(abi::arg(c, 0));
    let cursor = q.off(0x2ee4).lw(c);
    c.g[V1][0] = cursor;
    c.g[A1][0] = 0x1e;
    c.cycles += 12;
    c.g[T0][0] = 0x1e;
    let next = sx((cursor as u32).wrapping_add(0x1d));
    c.g[V0][0] = next;
    let count = q.off(0x2ee0).lw(c);
    c.g[A2][0] = count;
    let (_, rem) = abi::div(c, next as u32, 0x1e);
    let wrapped = sx((cursor as u32)
        .wrapping_sub(count as u32)
        .wrapping_add(0x1e));
    c.g[V1][0] = wrapped;
    c.g[A3][0] = sx(rem);
    let (_, rem2) = abi::div(c, wrapped as u32, 0x1e);
    q.off(0x2ee0).sw(c, sx(rem));
    c.g[V0][0] = sx(rem2);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Forward-difference cache.

/// `AIFwdDiffCache* AIFwdDiffCache_ctor(AIFwdDiffCache *c)` (0x327810): zero
/// the four head words.
pub fn ai_fwd_diff_cache_ctor(c: &mut Cpu) {
    c.cycles += 6;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, 0);
    this.off(4).sw(c, 0);
    this.off(8).sw(c, 0);
    this.off(0xc).sw(c, 0);
    abi::ret(c);
}

pub(crate) const EA_MEM_FREE: Callee = Callee {
    addr: 0x0031_7e98,
    f: ee::s10::EA_MemFree,
};

/// `AIFwdDiffCache *AIFwdDiffCache_dtor(AIFwdDiffCache *this, uint flags)`
/// (0x327828): free the two owned blocks (`this+0` and `this+8`) and, when
/// `flags & 1`, delete `this` itself.
pub fn ai_fwd_diff_cache_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let has_first = c.g[A0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if has_first {
            c.cycles += 2;
            EA_MEM_FREE.call(c, 0x0032_7850)?;
        }
        c.cycles += 3;
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(8)).lw(c);
        let has_second = c.g[A0][0] != 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if has_second {
            c.cycles += 3;
            EA_MEM_FREE.call(c, 0x0032_7864)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            BUILTIN_DELETE.call(c, 0x0032_7878)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Input VM: operand values.

/// `float InputVM_ReadOperand(sInputVM* self, int id)` (0x00325260): the
/// value of one operand. `id < 0x40` is a script variable (`self->v[id]`),
/// `0x40..0x60` a builtin, `0x60..0x3fc` controller channel
/// `(id - 0x60) / 6` reading `(id - 0x60) % 6`, and anything else 0.
pub fn input_vm_read_operand(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let id = c.g[A1][0];
        c.g[V0][0] = (id < 0x40) as u64;
        let not_variable = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !not_variable {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let at = p(c.g[V0][0] as u32, 0x8);
            c.lwc1(0, at);
            c.cycles += 1;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        // 0x40..0x60: a builtin.
        c.cycles += 4;
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x40));
        c.g[V0][0] = (c.g[V1][0] < 0x20) as u64;
        let not_builtin = c.g[V0][0] == 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x60));
        if !not_builtin {
            c.cycles += 4;
            c.g[A1][0] = c.g[V1][0];
            callee::INPUT_VM_READ_BUILTIN.call(c, 0x0032_5298)?;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        // 0x60..0x3fc: controller channel `id / 6`, reading `id % 6`.
        c.cycles += 3;
        c.g[V0][0] = (c.g[A1][0] < 0x39c) as u64;
        let padded = c.g[V0][0] != 0;
        c.g[V0][0] = 0x6;
        if !padded {
            c.cycles += 3;
            c.mtc1(0, 0);
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 4;
        c.g[V1][0] = 0x6;
        c.div(0, A1, V0);
        c.cycles += 4;
        c.g[A1][0] = c.lo[0];
        c.g[A2][0] = c.hi[0];
        callee::INPUT_VM_READ_PAD_CHANNEL.call(c, 0x0032_52d8)?;
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Misc small accessors.

/// `char* InputParser_BuiltinOperand(InputParser* p, int index)` (0x321ad0):
/// the built-in operand block for a pad, 0x40 bytes into the parser.
pub fn input_parser_builtin_operand(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx(abi::arg(c, 1).wrapping_add(0x40));
    abi::ret(c);
}

/// `PadDevice_dtor` (0x3277d0): vtable at `+0`.
pub fn pad_device_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_e4b8, 0x0032_77f4);
}

/// `void PadQueue_Init(PadQueue* self)` (0x326A28): create the four pad queues
/// at +0x2EEC and set the queue parameters (0x1D, 0).
pub fn pad_queue_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 4;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r(c, S0).wrapping_add(0x2eec));
        callee::PS2_PAD_CREATE.call(c, 0x0032_6a44)?;
        c.g[V1][0] = 0x1d;
        c.sw_r_2(r(c, S0).wrapping_add(0x2ee8), V0);
        Addr(r(c, S0).wrapping_add(0x2ee0)).sw(c, 0x1d);
        Addr(r(c, S0).wrapping_add(0x2ee4)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void InputPreproc_EmitLineMarker(InputPreproc* p, int line, char* text)`
/// (0x326430): append `# <line> "<text>"` to the preprocessed output.
pub fn input_preproc_emit_line_marker(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xd0);
        c.g[A3][0] = c.g[A2][0];
        frame.save(c, S0, 0xc0);
        c.g[A2][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = 0x490000;
        frame.save_ra(c, 0xb0);
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(0x1c38));
        callee::SPRINTF_2.call(c, 0x0032_6458)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        callee::INPUT_PREPROC_WRITE_STRING.call(c, 0x0032_6464)?;
        frame.restore(c, S0, 0xc0);
        frame.restore_ra(c, 0xb0);
        frame.close(c);
        Ok(())
    })
}

/// `void InputPreproc_WriteString(InputPreproc* in, const char* s)` (0x326308):
/// call `InputPreproc_Write(in, *s++)` for every character up to the NUL. The
/// back edge re-enters at 0x326328 after a thread switch.
pub fn input_preproc_write_string(c: &mut Cpu) {
    loop {
        match c.pc {
            0x0032_6308 => {
                c.cycles += 7;
                let frame = Frame::open(c, 0x30);
                frame.save(c, S0, 0x20);
                frame.save(c, S1, 0x10);
                c.g[S0][0] = c.g[A1][0];
                frame.save_ra(c, 0);
                c.g[S1][0] = c.g[A0][0];
                c.pc = 0x0032_633c;
            }
            0x0032_6324 => {
                c.cycles += 1;
                c.pc = 0x0032_6328;
            }
            0x0032_6328 => {
                c.cycles += 5;
                c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                if callee::INPUT_PREPROC_WRITE.call(c, 0x0032_633c).is_err() {
                    return;
                }
                c.pc = 0x0032_633c;
            }
            0x0032_633c => {
                // Registers at the call were set by the previous block.
                c.cycles += 3;
                c.g[V0][0] = Addr(p_2(c, S0, 0)).lb(c);
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = Addr(p_2(c, S0, 0)).lbu(c);
                if more {
                    if back_edge_unwinds(c, 0x0032_6328) {
                        return;
                    }
                    c.pc = 0x0032_6328;
                } else {
                    c.pc = 0x0032_6348;
                }
            }
            0x0032_6348 => {
                c.cycles += 5;
                let frame = Frame { size: 0x30 };
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return;
            }
            pc => {
                c.jump_out(pc, 0x0032_6308, 0x0032_635c);
                return;
            }
        }
    }
}

/// `void PS2Pad_SetActuator(PS2Pad* pad, int mode, float strength)`
/// (0x00327740): with mode 1 store whether the actuator is on at +0x30;
/// otherwise scale `strength` to a motor value clamped to 50..255 at +0x38.
pub fn ps2pad_set_actuator(c: &mut Cpu) {
    c.cycles += 3;
    let pad = abi::arg(c, 0);
    let mode = abi::arg64(c, 1);
    c.g[V0][0] = 1;
    if mode == 1 {
        // *(int*)(pad + 0x30) = strength != 0
        c.cycles += 5;
        c.mtc1(0, 0); // f0 = 0.0
        c.c_eq_s(12, 0);
        if c.fcc() {
            c.g[V0][0] = 0;
        }
        c.cycles += 2;
        c.sw(p_3(pad, 0x30), c.g[V0][0]);
        abi::ret(c);
        return;
    }
    // *(int*)(pad + 0x38) = clamp((int)(strength * 205 + 50), 50, 255)
    c.cycles += 5;
    c.mtc1(0, 0); // f0 = 0.0
    c.c_eq_s(12, 0);
    let off = c.fcc();
    c.g[A0][0] = sx(p_3(pad, 0x38));
    let value = if off {
        c.cycles += 1;
        c.g[V0][0] = 0;
        0u64
    } else {
        c.cycles += 11;
        c.g[AT][0] = 0x434d_0000; // 205.0f
        c.mtc1(1, 0);
        c.g[AT][0] = 0x4248_0000; // 50.0f
        c.mtc1(1, 1);
        c.mul_s(0, 12, 0);
        c.add_s(0, 0, 1);
        c.cvt_w_s(1, 0);
        c.mfc1(3, 1); // v1 = (int)(strength * 205 + 50)
        c.g[V0][0] = ((c.g[V1][0] as i64) < 50) as u64;
        let below = c.g[V0][0] != 0;
        c.g[A1][0] = 0xff;
        if below {
            c.cycles += 1;
            c.g[V1][0] = 0x32; // 50
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[A1][0];
            }
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[V1][0];
        c.g[V1][0]
    };
    c.cycles += 2;
    c.sw(c.g[A0][0] as u32, value);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Pads and strings.

/// `void PS2Pad_ctor(PS2Pad* pad, int port, int slot)` (0x326df0): install the
/// vtable, remember the port/slot and clear the pad state.
pub fn ps2_pad_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x48_e488;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx(0x0048_e488);
        let self_ = c.g[A0][0];
        let port = c.g[A1][0];
        let slot = c.g[A2][0];
        c.g[S0][0] = self_;
        frame.save_ra(c, 0);
        Addr(at(c, S0, 0x4)).sw(c, port);
        c.g[A0][0] = sx(at(c, S0, 0xc));
        Addr(at(c, S0, 0x8)).sw(c, slot);
        c.g[A1][0] = 0;
        Addr(at(c, S0, 0x0)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 0x24;
        call_ptr(c, 0x0041_6210, 0x0032_6e24)?;
        c.g[V0][0] = c.g[S0][0];
        Addr(at(c, S0, 0x54)).sw(c, 0);
        Addr(at(c, S0, 0x58)).sw(c, 0);
        Addr(at(c, S0, 0x50)).sw(c, 0);
        Addr(at(c, S0, 0x5c)).sw(c, 0);
        Addr(at(c, S0, 0x48)).sw(c, 0);
        Addr(at(c, S0, 0x4c)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2Pad_dtor(PS2Pad* pad, int free)` (0x326e50): restore the vtable,
/// tear the pad down and optionally delete it.
pub fn ps2_pad_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x48_e488;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0048_e488);
        frame.save(c, S1, 0x10);
        let free = c.g[A1][0];
        c.g[S0][0] = free;
        frame.save_ra(c, 0);
        let self_ = c.g[A0][0];
        c.g[S1][0] = self_;
        Addr(at(c, S1, 0x0)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(at(c, S1, 0x4)).lw(c);
        c.g[A1][0] = Addr(at(c, S1, 0x8)).lw(c);
        call_ptr(c, 0x003f_f8f0, 0x0032_6e80)?;
        c.g[V1][0] = 0x48_e4b8;
        c.g[S0][0] = c.g[S0][0] & 0x1;
        c.g[V1][0] = sx(0x0048_e4b8);
        let keep = c.g[S0][0] == 0;
        Addr(at(c, S1, 0x0)).sw(c, c.g[V1][0]);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0032_6e9c)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `PadQueue_StopAllActuators(PadQueue* self)` (0x326ae0): stop the actuators
/// of every queued pad device.
pub fn pad_queue_stop_all_actuators(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        let this = c.g[A0][0];
        c.g[S2][0] = this;
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, p(this as u32, 0x2ee8));
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[S0][0] = sx(p(this as u32, 0x2eec));
            c.g[A0][0] = lw(c, p(this as u32, 0x2eec));
            loop {
                c.cycles += 7;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                call_ptr(c, 0x0032_6cf0, 0x0032_6b1c)?;
                c.g[V0][0] = lw(c, p(this as u32, 0x2ee8));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 0));
                if event_due(c) {
                    deliver_event_here(c, 0x0032_6b10);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The opcode dispatch table (`jpt_3254c4`). The port matches on the opcode;
/// the address is only kept to reproduce what the original leaves in `v1`.
pub(crate) const OPCODES: u32 = 0x0048_e330;

/// Opcode that ends the run (`s6 = 1`).
pub(crate) const OP_STOP: u32 = 0x20;

/// Loop head: the resume address of the back-edge event check.
pub(crate) const HEAD: u32 = 0x0032_5498;

/// The shared "store `f0` into the destination" block, and the event check
/// that resumes there.
pub(crate) const STORE: u32 = 0x0032_5524;

/// The block that reads the first operand again before that store.
pub(crate) const STORE_OPERAND: u32 = 0x0032_5518;

/// `parser` fields: the incoming argument (`+4`) and the virtual register
/// file (`+8`, one 32-bit float each).
pub(crate) const PARSER_ARG: u32 = 0x4;

#[allow(dead_code)] // the register file base is only named in the test
pub(crate) const PARSER_REGS: u32 = 0x8;

// ---------------------------------------------------------------------------
// The interpreter's state: the values the original keeps in s0..s6.

pub(crate) struct Vm {
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
pub(crate) fn fr(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

/// `fN = v`.
#[inline(always)]
pub(crate) fn setf(c: &mut Cpu, n: usize, v: FpuF32) -> FpuF32 {
    c.f[n] = v.0;
    v
}

/// `li at, bits; mtc1 at, fN`.
#[inline(always)]
pub(crate) fn li_f(c: &mut Cpu, n: usize, bits: u32) -> FpuF32 {
    c.g[AT][0] = sx(bits);
    setf(c, n, FpuF32(bits))
}

/// `lwc1 fN, gp+off`.
#[inline(always)]
pub(crate) fn gp_float(c: &mut Cpu, off: i32, n: usize) -> FpuF32 {
    let a = r32(c, GP).wrapping_add(off as u32);
    let v = Addr(a).read_fpu(c);
    setf(c, n, v)
}

/// Set the FPU condition bit; returns it.
#[inline(always)]
pub(crate) fn fcc(c: &mut Cpu, cond: bool) -> bool {
    abi::set_fcc(c, cond);
    cond
}

/// `InputVM_GetRegPtr(parser, idx)`: the address of virtual register `idx`.
pub(crate) fn dst_of(c: &mut Cpu, vm: &mut Vm, idx: u32, ret: u32) -> Result<u32, Unwound> {
    vm.spill(c);
    c.g[A0][0] = sx(vm.parser);
    c.g[A1][0] = sx(idx);
    callee::GET_REG_PTR.call(c, ret)?;
    Ok(r32(c, V0))
}

/// `f0 = InputVM_ReadOperand(parser, idx)`.
pub(crate) fn operand(c: &mut Cpu, vm: &Vm, idx: u32, ret: u32) -> Result<FpuF32, Unwound> {
    vm.spill(c);
    c.g[A0][0] = sx(vm.parser);
    c.g[A1][0] = sx(idx);
    callee::READ_OPERAND.call(c, ret)?;
    Ok(fr(c, 0))
}

/// The shared `0x00325524` tail: store `f0` in the destination.
pub(crate) fn store_result(c: &mut Cpu, vm: &Vm) {
    c.cycles += 2;
    let v = fr(c, 0);
    Addr(vm.dst).write_fpu(c, v);
}

/// The shared `0x00325518` tail: `f0 = ReadOperand(parser, idx)`, then store.
pub(crate) fn store_operand(c: &mut Cpu, vm: &Vm, idx: u32) -> Flow {
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
pub(crate) fn run_op(c: &mut Cpu, vm: &mut Vm, op: u32, w: u32) -> Flow {
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
        6 => compare(
            c,
            vm,
            w,
            [0x0032_566c, 0x0032_5684, 0x0032_5698],
            true,
            |a, b| b.lt(a),
        )?,
        // 0x06b8: `dst = a >= b` (the compare is `op2 <= op1`).
        7 => compare(
            c,
            vm,
            w,
            [0x0032_56cc, 0x0032_56e4, 0x0032_56f8],
            true,
            |a, b| b.le(a),
        )?,
        // 0x0718: `dst = a < b`.
        8 => compare(
            c,
            vm,
            w,
            [0x0032_572c, 0x0032_5744, 0x0032_5758],
            true,
            |a, b| a.lt(b),
        )?,
        // 0x0778: `dst = a <= b`.
        9 => compare(
            c,
            vm,
            w,
            [0x0032_578c, 0x0032_57a4, 0x0032_57b8],
            true,
            |a, b| a.le(b),
        )?,
        // 0x07d8: `dst = a == b`.
        10 => compare(
            c,
            vm,
            w,
            [0x0032_57ec, 0x0032_5804, 0x0032_5818],
            true,
            |a, b| a.eq_s(b),
        )?,
        // 0x0838: `dst = a != b` (the compare is an equality test).
        11 => compare(
            c,
            vm,
            w,
            [0x0032_584c, 0x0032_5864, 0x0032_5878],
            false,
            |a, b| a.eq_s(b),
        )?,
        // 0x0898: `dst = a + b` (the result stays in f20).
        12 => binary(c, vm, w, 0x0032_58ac, 0x0032_58c4, 0x0032_58d8, |a, b| {
            a + b
        })?,
        // 0x08e4: `dst = a - b`.
        13 => binary(c, vm, w, 0x0032_58f8, 0x0032_5910, 0x0032_5924, |a, b| {
            a - b
        })?,
        // 0x0930: `dst = a * b`.
        14 => binary(c, vm, w, 0x0032_5944, 0x0032_595c, 0x0032_5970, |a, b| {
            a * b
        })?,
        // 0x097c: `dst = a / b`.
        15 => binary(c, vm, w, 0x0032_5990, 0x0032_59a8, 0x0032_59bc, |a, b| {
            a.div(b)
        })?,
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
        20 => unary(
            c,
            vm,
            w,
            0x0032_5bcc,
            0x0032_5be4,
            &callee::MATH_ATAN,
            0x0032_5bec,
        )?,
        // 0x0bf4: `dst = sin(a)`.
        21 => unary(
            c,
            vm,
            w,
            0x0032_5c08,
            0x0032_5c20,
            &callee::MATH_SIN,
            0x0032_5c28,
        )?,
        // 0x0c30: `dst = cos(a)`.
        22 => unary(
            c,
            vm,
            w,
            0x0032_5c44,
            0x0032_5c5c,
            &callee::MATH_COS,
            0x0032_5c64,
        )?,
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
pub(crate) fn skip_words(c: &mut Cpu, vm: &mut Vm) {
    c.cycles += 3;
    let n = c.lhu(vm.ins.wrapping_add(2)) as u32;
    c.g[V0][0] = sx(n);
    vm.idx = vm.idx.wrapping_add(n);
}

/// Opcodes 6..11: `dst = (op1 OP op2) ? 1.0 : 0.0`, where `one_if_compare`
/// says which side of the compare stores 1.0 (0x0b tests equality and stores
/// 1.0 when it fails).
pub(crate) fn compare(
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
pub(crate) fn binary(
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
pub(crate) fn unary(
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
pub(crate) fn exp_into(c: &mut Cpu, vm: &Vm, dst: u32) -> Flow {
    c.cycles += 4;
    vm.spill(c);
    call_ptr(c, callee::EXPF, 0x0032_5ca4)?;
    let v = fr(c, 0);
    Addr(dst).write_fpu(c, v);
    Ok(())
}

/// `void InputPreproc_SkipLine(InputPreproc* p_4, Stream* s)` (0x326678): consume
/// characters up to and including the next newline (0x0a), or until the
/// stream ends; then advance the stream's line counter.
pub fn input_preproc_skip_line(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 16, 0x30);
        sqsp(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        sqsp(c, 18, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = 0xa;
        c.g[V0][0] = Addr(p_4(r32(c, S0), 0xc)).lw(c);
        loop {
            c.cycles += 2;
            let eof = (c.g[V0][0] as i64) < 0;
            c.g[A0][0] = c.g[S0][0];
            if eof {
                break;
            }
            c.cycles += 5;
            c.g[A1][0] = c.g[S1][0];
            call::call_ptr(c, 0x0032_6478, 0x0032_66b0)?;
            c.g[V0][0] = Addr(p_4(r32(c, S0), 0xc)).lw(c);
            let not_newline = c.g[V0][0] != c.g[S2][0];
            c.g[A0][0] = c.g[S0][0];
            if !not_newline {
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                call::call_ptr(c, 0x0032_6478, 0x0032_66c4)?;
                c.g[V0][0] = Addr(p_4(r32(c, S1), 0xc)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                Addr(p_4(r32(c, S1), 0xc)).sw(c, c.g[V0][0]);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_66a0);
            }
        }
        c.cycles += 6;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `void PadDevice_StopActuators(PadDevice* d)` (0x326cf0): call the actuator
/// setter with 0.0f for every actuator the device reports.
pub fn pad_device_stop_actuators(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0032_6cf0 => {
                    c.cycles += 7;
                    let frame = Frame::open(c, 0x30);
                    frame.save(c, 16, 0x20);
                    frame.save(c, 17, 0x10);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[S1][0] = 0;
                    pc = 0x0032_6d24;
                }
                0x0032_6d0c => {
                    c.cycles += 1;
                    pc = 0x0032_6d10;
                }
                0x0032_6d10 => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                    c.g[A0][0] = Addr(p_4(c.g[V0][0] as u32, 0x20)).lh(c);
                    c.g[V1][0] = Addr(p_4(c.g[V0][0] as u32, 0x24)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
                    call::call_ptr(c, target, 0x0032_6d24)?;
                    pc = 0x0032_6d24;
                }
                0x0032_6d24 => {
                    c.cycles += 10;
                    c.g[V1][0] = Addr(r32(c, S0)).lw(c);
                    c.g[A0][0] = Addr(p_4(c.g[V1][0] as u32, 0x18)).lh(c);
                    c.g[V0][0] = Addr(p_4(c.g[V1][0] as u32, 0x1c)).lw(c);
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
                    call::call_ptr(c, target, 0x0032_6d38)?;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.mtc1(0, 12);
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    if more {
                        if crate::events::back_edge_unwinds(c, 0x0032_6d10) {
                            return Ok(());
                        }
                        pc = 0x0032_6d10;
                    } else {
                        pc = 0x0032_6d4c;
                    }
                }
                0x0032_6d4c => {
                    c.cycles += 5;
                    let frame = Frame { size: 0x30 };
                    frame.restore(c, 16, 0x20);
                    frame.restore(c, 17, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0032_6cf0, 0x0032_6d60);
                    return Ok(());
                }
            }
        }
    });
}

// ---------------------------------------------------------------------------
// List walks and buffer management.

/// `func_00326A68` (0x326a68): shut down the pad queue. For each of the
/// `count` entries at `this+0x2eec` (a null entry is skipped) call its virtual
/// destructor (`vtable[8]`, flag 3). The loop's back edge carries an event
/// check. Signature: `void shutdown(PadQueue* this)`.
pub fn pad_queue_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_r(c, S2, 0x2ee8);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[S0][0] = sx(low(c, S2).wrapping_add(0x2eec));
            loop {
                c.cycles += 3;
                c.g[A2][0] = lw_r(c, S0, 0);
                let skip = c.g[A2][0] == 0;
                c.g[A1][0] = 0x3;
                if !skip {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(low(c, A2)).lw(c);
                    c.g[A0][0] = lh_r(c, V0, 0x8);
                    c.g[V1][0] = lw_r(c, V0, 0xc);
                    let target = low(c, V1);
                    c.g[RA][0] = sx(0x0032_6ab0);
                    c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
                    call_ptr(c, target, 0x0032_6ab0)?;
                }
                c.cycles += 5;
                c.g[V0][0] = lw_r(c, S2, 0x2ee8);
                c.g[S1][0] = sx(low(c, S1).wrapping_add(1));
                let more = (c.g[S1][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[S0][0] = sx(low(c, S0).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_6a90);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Input parser.

/// `InputEntry* InputParser_FindIdentifier(InputParser* self, char* name)`
/// (0x321a40): look `name` up in the parser's 0x44-byte entry table.
pub fn input_parser_find_identifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x8000;
        frame.save(c, S1, 0x30);
        c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        frame.save(c, S3, 0x10);
        c.g[S1][0] = 0;
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, V1), 0xe4)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = c.g[A1][0];
        if !empty {
            c.cycles += 4;
            c.g[V0][0] = 0x80e8;
            c.g[S2][0] = c.g[V1][0];
            c.g[S0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0041_aa88, 0x0032_1a8c)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    c.cycles += 7;
                    frame.restore(c, S0, 0x40);
                    frame.restore(c, S1, 0x30);
                    frame.restore(c, S2, 0x20);
                    frame.restore(c, S3, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                c.g[V0][0] = Addr(p(r32(c, S2), 0xe4)).lw(c);
                c.cycles += 4;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(p(r32(c, S0), 0x44));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_1a80);
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 0;
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PS2Pad_Create(PS2Pad** out, int n)` (0x326d60): allocate and build up
/// to two pads, storing them in `out`, and return how many were made.
pub fn ps2_pad_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        let out = abi::arg(c, 0);
        let _n = abi::arg64(c, 1);
        c.g[V0][0] = ((c.g[A1][0] as i64) < 3) as u64;
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = 2;
        frame.save(c, S1, 0x30);
        if c.g[V0][0] != 0 {
            c.g[S2][0] = c.g[A1][0];
        }
        frame.save(c, S3, 0x10);
        c.g[S0][0] = 0;
        let none = (c.g[S2][0] as i64) <= 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 3;
            c.g[S1][0] = sx(out);
            c.g[S3][0] = 0x004a_0000;
            loop {
                c.cycles += 14;
                c.g[A0][0] = 0x180;
                c.g[A1][0] = sx(p(r32(c, S3), 0x3fb0));
                c.g[A2][0] = 0x400_0000;
                c.g[A3][0] = 0;
                call_ptr(c, callee::MEM_ALLOC, 0x0032_6dac)?;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = 0;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                call_ptr(c, callee::PS2_PAD_CTOR, 0x0032_6dc0)?;
                c.sw(r32(c, S1), c.g[V0][0]);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_6d98);
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int InputParser_LookupModifier(const char* name, void* parser)` (0x3217d0):
/// map a modifier name to its index (0..3), the "any" modifier (4), a
/// fallback (6) or "no modifier" (0xff... never: 0 for the first name).
pub fn input_parser_lookup_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let name = c.g[A1][0];
        c.g[S0][0] = name;
        c.g[A0][0] = name;
        c.g[A1][0] = 0x4a3ef8;
        call_ptr(c, 0x0041_aa88, 0x0032_17f0)?;
        c.g[A1][0] = 0x4a_0000;
        let differs = c.g[V0][0] != 0;
        if !differs {
            c.cycles += 2;
            c.g[V0][0] = 0;
            frame.restore(c, S0, 0x10);
            c.cycles += 4;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = name;
        c.g[A1][0] = 0x4a3f00;
        call_ptr(c, 0x0041_aa88, 0x0032_180c)?;
        c.g[A1][0] = 0x4a_0000;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 1;
            frame.restore(c, S0, 0x10);
            c.cycles += 4;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = name;
        c.g[A1][0] = 0x4a3f08;
        call_ptr(c, 0x0041_aa88, 0x0032_1828)?;
        c.g[A1][0] = 0x49_0000;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 2;
            frame.restore(c, S0, 0x10);
            c.cycles += 4;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = name;
        c.g[A1][0] = 0x48ddd8;
        call_ptr(c, 0x0041_aa88, 0x0032_1844)?;
        c.g[A1][0] = 0x4a_0000;
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = 3;
            frame.restore(c, S0, 0x10);
            c.cycles += 4;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 8;
        c.g[A0][0] = name;
        c.g[A1][0] = 0x4a3f10;
        call_ptr(c, 0x0041_aa88, 0x0032_1858)?;
        c.g[A0][0] = 0x4;
        c.g[V1][0] = 0x6;
        if c.g[V0][0] == 0 {
            c.g[V1][0] = c.g[A0][0];
        }
        c.g[V0][0] = c.g[V1][0];
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Input preprocessor.

/// `void InputPreproc_SkipLineComment(InputPreproc* p, Cursor* cur, void* a,
/// void* b)` (0x3265c0): copy the rest of a `//` comment line to the output,
/// stopping at the newline (which is written) or at end of input.
pub fn input_preproc_skip_line_comment(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        let pre = r(c, A0);
        c.g[S0][0] = sx(pre);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0xa;
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A3][0];
        c.g[V0][0] = Addr(p(pre, 0xc)).lw(c);
        'outer_scan: loop {
            'body: {
                'newline: {
                    c.cycles += 2;
                    let is_nl = c.g[V0][0] != c.g[S2][0];
                    c.g[A0][0] = c.g[S0][0];
                    if is_nl {
                        break 'newline;
                    }
                    c.cycles += 10;
                    c.g[V0][0] = Addr(p(r(c, S1), 0xc)).lw(c);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(1));
                    Addr(p(r(c, S1), 0xc)).sw(c, c.g[V0][0]);
                    c.g[RA][0] = sx(0x0032_6614);
                    c.g[A1][0] = Addr(p(r(c, S0), 0xc)).lb(c);
                    callee::INPUT_PREPROC_WRITE.call(c, 0x0032_6614)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0032_6620);
                    c.g[A1][0] = c.g[S1][0];
                    callee::INPUT_PREPROC_NEXT_CHAR.call(c, 0x0032_6620)?;
                    frame.restore(c, 16, 0x50);
                    break 'body;
                }
                'eof: {
                    c.cycles += 2;
                    let at_end = (c.g[V0][0] as i64) < 0;
                    c.g[T0][0] = sx(0x0049_0000);
                    if at_end {
                        break 'eof;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0032_6638);
                    c.g[A1][0] = c.g[S1][0];
                    callee::INPUT_PREPROC_NEXT_CHAR.call(c, 0x0032_6638)?;
                    c.g[V0][0] = Addr(p(r(c, S0), 0xc)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_65f8);
                    }
                    continue 'outer_scan;
                }
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[A3][0] = c.g[S4][0];
                c.g[RA][0] = sx(0x0032_6658);
                c.g[T0][0] = sx(r(c, T0).wrapping_sub(0x1c28));
                callee::INPUT_PREPROC_EMPTY_STUB.call(c, 0x0032_6658)?;
                frame.restore(c, 16, 0x50);
                break 'body;
            }
            c.cycles += 7;
            frame.restore(c, 17, 0x40);
            frame.restore(c, 18, 0x30);
            frame.restore(c, 19, 0x20);
            frame.restore(c, 20, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `int InputParser_CompileIdentifier(InputParser *self, ...)` (0x3245a8):
/// look the identifier up and either emit its node or raise the parser error.
pub fn input_parser_compile_identifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        c.g[S2][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[S3][0] = c.g[T0][0];
        call_ptr(c, 0x0032_1a40, 0x0032_45dc)?;
        if c.g[V0][0] == 0 {
            c.cycles += 7;
            c.g[A1][0] = 0x0048_e208;
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S4][0];
            call_ptr(c, 0x0032_1500, 0x0032_45f8)?;
            c.g[V0][0] = u64::MAX;
        } else {
            c.g[A1][0] = Addr(p_5(c.g[V0][0], 0x40)).lw(c);
            c.cycles += 12;
            c.g[A3][0] = c.g[S1][0];
            c.g[T0][0] = c.g[S0][0];
            c.g[T1][0] = c.g[S3][0];
            Addr(frame.at(c, 0x0)).sw(c, 0);
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.g[SP][0];
            call_ptr(c, 0x0032_4678, 0x0032_461c)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            let empty = (c.g[S0][0] as i64) <= 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            if empty {
                c.cycles += 5;
                c.g[A1][0] = 0x0048_e228;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0032_1500, 0x0032_4650)?;
                c.g[V0][0] = u64::MAX;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x20;
                Addr(c.g[S1][0] as u32).sw(c, c.g[V0][0]);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void InputPreproc_Write(InputPreproc *self, int c)` (0x326360): append a
/// character to the growing buffer.
pub fn input_preproc_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = Addr(p_5(c.g[S0][0], 0x4)).lw(c);
        let not_full = c.g[V1][0] != c.g[V0][0];
        c.g[S3][0] = sx((((c.g[A1][0] as u32) as i32) >> 24) as u32);
        if !not_full {
            c.cycles += 22;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 1);
            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x400));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A1][0] = 0x0048_e3b8;
            c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A2][0] = 0;
            c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> 1) as u32);
            c.g[A3][0] = 0;
            c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V1][0] != 0 {
                c.g[S1][0] = c.g[V0][0];
            }
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e30, 0x0032_63c8)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[A1][0] = Addr(p_5(c.g[S0][0], 0x8)).lw(c);
            c.g[A2][0] = Addr(p_5(c.g[S0][0], 0x4)).lw(c);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0041_605c, 0x0032_63dc)?;
            c.g[A0][0] = Addr(p_5(c.g[S0][0], 0x8)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x0032_63f0)?;
            }
            Addr(p_5(c.g[S0][0], 0x8)).sw(c, c.g[S2][0]);
            c.cycles += 2;
            Addr(p_5(c.g[S0][0], 0x4)).sw(c, c.g[S1][0]);
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        }
        c.cycles += 12;
        c.g[V0][0] = Addr(p_5(c.g[S0][0], 0x8)).lw(c);
        frame.restore(c, S1, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        frame.restore(c, S2, 0x20);
        Addr(c.g[V0][0] as u32).sb(c, c.g[S3][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        Addr(c.g[S0][0] as u32).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod ps2pad {

    pub const PORT: u32 = 0x04;

    pub const SLOT: u32 = 0x08;

    /// Requested small / big motor values and the last ones sent.
    pub const ACT_SMALL: u32 = 0x30;

    pub const LAST_SMALL: u32 = 0x34;

    pub const ACT_BIG: u32 = 0x38;

    pub const LAST_BIG: u32 = 0x3c;

    /// The 6-byte actuator block handed to `scePadSetActDirect`.
    pub const ACT_DIRECT: u32 = 0x40;

    /// Connection state machine phase (0 = port closed, 10 = reading).
    pub const PHASE: u32 = 0x48;

    pub const RETRY: u32 = 0x4c;

    pub const MODE_SET: u32 = 0x50;

    /// Pressure-sensitive buttons enabled (else digital ones).
    pub const PRESSURE: u32 = 0x54;

    pub const ANALOG: u32 = 0x58;

    pub const ACTUATOR: u32 = 0x5c;

    /// The 64-byte aligned DMA buffer given to `scePadPortOpen`.
    pub const DMA_BUF: u32 = 0x80;

    pub const PORT_OPEN: u32 = 0x003f_f708;

    pub const GET_STATE: u32 = 0x003f_fbc0;

    pub const GET_REQ_STATE: u32 = 0x003f_fcd8;

    pub const INFO_ACT: u32 = 0x003f_fd60;

    pub const INFO_MODE: u32 = 0x003f_ffa0;

    pub const SET_MAIN_MODE: u32 = 0x0040_00d8;

    pub const SET_ACT_DIRECT: u32 = 0x0040_0190;

    pub const SET_ACT_ALIGN: u32 = 0x0040_0250;

    pub const INFO_PRESS_MODE: u32 = 0x0040_0490;

    pub const ENTER_PRESS_MODE: u32 = 0x0040_04f0;

    pub const READ: u32 = 0x003f_fa58;

    /// The `switch (phase)` jump table in `.rodata` and its case labels
    /// (phases 1..=10).
    pub const JUMP_TABLE: u32 = 0x0048_e460;

    pub const CASES: [u32; 10] = [
        0x0032_6f70,
        0x0032_7028,
        0x0032_707c,
        0x0032_709c,
        0x0032_70c8,
        0x0032_70e8,
        0x0032_712c,
        0x0032_715c,
        0x0032_7188,
        0x0032_71d8,
    ];

    /// `D_004A3FB8`, the actuator alignment (gp-relative).
    pub const ACT_ALIGN_GP: u32 = 0xec8;

    /// `1/255` scale for pressure bytes and for the stick ramps (gp-relative).
    pub const PRESSURE_SCALE_GP: u32 = 0x2e0c;

    pub const STICK_SCALE_GP: u32 = 0x2e08;

    /// Number of output floats written when the read succeeded.
    pub const OUT_COUNT: u64 = 0x18;
}

/// 1.0f
pub(crate) const PAD_ONE: u32 = 0x3f80_0000;

/// One active-low digital button as a float: `f0` = 1.0 when its bit is
/// clear (pressed), else 0.0; stored at `dst`. The caller charges the test.
pub(crate) fn pad_button(c: &mut Cpu, bits: u64, mask: u64, dst: u32) {
    c.f[0] = 0;
    if bits & mask == 0 {
        c.cycles += 3;
        c.g[AT][0] = PAD_ONE as u64;
        c.f[0] = PAD_ONE;
    }
    c.swc1(0, dst);
}

/// `mtc1 zero, f0` then 1.0 into `f0` (2 cycles) when the button is
/// pressed; no store (the next block stores it).
pub(crate) fn pad_button_late(c: &mut Cpu, pressed: bool) {
    c.f[0] = 0;
    if pressed {
        c.cycles += 2;
        c.g[AT][0] = PAD_ONE as u64;
        c.f[0] = PAD_ONE;
    }
}

/// `div n, 0x4f` (pipeline 0): sets LO/HI, returns the quotient.
pub(crate) fn pad_div79(c: &mut Cpu, n: u64) -> u64 {
    let n = n as i32;
    c.lo[0] = sx((n / 0x4f) as u32);
    c.hi[0] = sx((n % 0x4f) as u32);
    c.lo[0]
}

/// One stick axis byte: the two ramp numerators `(0x4f - b) * 255` and
/// `(b - 0xb0) * 255` (computed `x << 8 - x`) and their `/ 0x4f`
/// quotients clamped at 0 (the clamp flag is `-1 < q`). Returns
/// `(neg_quotient, neg_flag, pos_quotient, pos_flag, pos_delta)`.
pub(crate) fn pad_stick(c: &mut Cpu, b: u64) -> (u64, u64, u64, u64, u64) {
    let neg = sx(0x4fu32.wrapping_sub(b as u32));
    let pos = sx((b as u32).wrapping_sub(0xb0));
    let neg_num = sx(((neg as u32) << 8).wrapping_sub(neg as u32));
    let pos_num = sx(((pos as u32) << 8).wrapping_sub(pos as u32));
    let qn = pad_div79(c, neg_num);
    let qp = pad_div79(c, pos_num);
    let fneg = ((qn as i64) > -1) as u64;
    let fpos = ((qp as i64) > -1) as u64;
    (
        if fneg == 0 { 0 } else { qn },
        fneg,
        if fpos == 0 { 0 } else { qp },
        fpos,
        pos,
    )
}

/// `int PS2Pad_Update(PS2Pad* pad, float* out)` (0x326eb0): one frame of
/// a controller. Opens the port if needed, then steps the libpad
/// connection state machine (`pad->phase`): probe the controller id and
/// mode (DualShock 2 / analog), lock analog mode, enter pressure mode,
/// query and align the vibration motors, and finally read (phase 10). On
/// a good read it fills 24 floats in `out`: d-pad, the 12 face/shoulder
/// buttons (pressure bytes / 255 when pressure mode is on, else 0/1 from
/// the active-low bits) and the 8 stick half-axes (ramp outside the dead
/// zone 0x4f..0xb0, scaled by 1/255), and returns 0x18; else returns 0.
/// While reading, sends changed motor values with `scePadSetActDirect`.
pub fn ps2_pad_update(c: &mut Cpu) {
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;
    use ps2pad::*;

    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S2, 0x40);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        let pad = Addr(c.g[S0][0] as u32);
        let out = c.g[S1][0] as u32;
        let gp = c.g[GP][0] as u32;

        // Phase 0: open the port; success resets the state.
        let phase = pad.off(PHASE).lw(c);
        c.g[V0][0] = phase;
        c.g[S3][0] = 0;
        if phase != 0 {
            c.cycles += 1;
        } else {
            c.cycles += 8;
            c.g[A0][0] = pad.off(PORT).lw(c);
            c.g[A2][0] = sx(pad.0.wrapping_add(DMA_BUF));
            c.g[A1][0] = pad.off(SLOT).lw(c);
            call_ptr(c, PORT_OPEN, 0x0032_6eec)?;
            let opened = c.g[V0][0];
            c.g[V1][0] = opened;
            c.g[V0][0] = 1;
            if opened == 1 {
                c.cycles += 7 + 1;
                c.g[V0][0] = u64::MAX;
                pad.off(PHASE).sw(c, opened);
                pad.off(LAST_BIG).sw(c, u64::MAX);
                pad.off(MODE_SET).sw(c, 0);
                pad.off(ACT_SMALL).sw(c, 0);
                pad.off(LAST_SMALL).sw(c, u64::MAX);
                pad.off(ACT_BIG).sw(c, 0);
            }
        }
        c.g[A1][0] = pad.off(SLOT).lw(c);

        // Disconnected (0) or error (7): start over at probing.
        c.cycles += 5;
        c.g[A0][0] = pad.off(PORT).lw(c);
        call_ptr(c, GET_STATE, 0x0032_6f24)?;
        let state = c.g[V0][0];
        c.g[A1][0] = state;
        c.g[V0][0] = 7;
        if state != 0 {
            c.cycles += 2;
        }
        if state == 0 || state == 7 {
            c.cycles += 4;
            c.g[V0][0] = 1;
            pad.off(MODE_SET).sw(c, 0);
            pad.off(PHASE).sw(c, 1);
        }
        let phase = pad.off(PHASE).lw(c);
        c.g[V0][0] = phase;

        c.cycles += 4;
        let index = sx((phase as u32).wrapping_sub(1));
        c.g[A0][0] = index;
        c.g[V1][0] = (index < 10) as u64;
        c.g[V0][0] = 0x0049_0000;
        let mut read_ok = false;
        if index >= 10 {
            c.cycles += 1;
        } else {
            c.cycles += 6;
            c.g[V1][0] = sx(JUMP_TABLE.wrapping_add((index as u32) << 2));
            c.g[V0][0] = sx(JUMP_TABLE);
            c.g[A0][0] = CASES[index as usize] as u64;
            let port = |c: &mut Cpu| pad.off(PORT).lw(c);
            let slot = |c: &mut Cpu| pad.off(SLOT).lw(c);
            match index + 1 {
                // Stable (6) or busy (2): probe the controller id and mode.
                1 => {
                    c.cycles += 3;
                    c.g[V0][0] = 2;
                    let mut probe = state == 6;
                    if !probe {
                        // `bne a1, 2` with `move s2, zero` in its delay
                        // slot: s2 is 0 at the InfoMode call (callees
                        // save it on their frames)
                        c.cycles += 2;
                        c.g[S2][0] = 0;
                        probe = state == 2;
                    }
                    if probe {
                        c.cycles += 11;
                        pad.off(PRESSURE).sw(c, 0);
                        c.g[A2][0] = 1;
                        pad.off(ANALOG).sw(c, 0);
                        c.g[A3][0] = 0;
                        pad.off(ACTUATOR).sw(c, 0);
                        c.g[A0][0] = port(c);
                        c.g[A1][0] = slot(c);
                        call_ptr(c, INFO_MODE, 0x0032_6fa4)?;
                        let id = c.g[V0][0];
                        c.g[S2][0] = id;
                        c.g[A2][0] = 2;
                        if id == 0 {
                            c.cycles += 1;
                        } else {
                            c.cycles += 8;
                            c.g[A0][0] = port(c);
                            c.g[A1][0] = slot(c);
                            c.g[A3][0] = 0;
                            call_ptr(c, INFO_MODE, 0x0032_6fc0)?;
                            let ext = c.g[V0][0];
                            c.g[V1][0] = ext;
                            c.g[A0][0] = 4;
                            c.g[V0][0] = ((ext as i64) < 5) as u64;
                            if ext == 4 {
                                // digital DualShock: set the analog mode
                                c.cycles += 3 + 1;
                                c.g[V0][0] = 2;
                                pad.off(PHASE).sw(c, 2);
                            } else if (ext as i64) >= 5 {
                                c.cycles += 2 + 2;
                                c.g[V0][0] = 7;
                                if ext == 7 {
                                    // analog DualShock
                                    c.cycles += 5 + 1;
                                    c.g[V0][0] = 1;
                                    c.g[V1][0] = 2;
                                    pad.off(ANALOG).sw(c, 1);
                                    pad.off(PHASE).sw(c, 2);
                                } else {
                                    c.cycles += 2;
                                }
                            } else {
                                c.cycles += 2 + 2;
                                if ext != 0 {
                                    c.cycles += 2;
                                } else {
                                    c.cycles += 2;
                                    if id == 4 {
                                        // plain digital pad: read directly
                                        c.cycles += 2 + 2 + 1;
                                        c.g[V0][0] = 0xa;
                                        pad.off(PHASE).sw(c, 0xa);
                                    }
                                }
                            }
                        }
                    }
                }
                // Lock analog mode (unless already set for an analog pad).
                2 => {
                    c.cycles += 3;
                    let analog = pad.off(ANALOG).lw(c);
                    c.g[V0][0] = analog;
                    let mut set_mode = true;
                    if analog != 0 {
                        c.cycles += 3;
                        let mode_set = pad.off(MODE_SET).lw(c);
                        c.g[V0][0] = 4;
                        if mode_set != 0 {
                            set_mode = false;
                            c.cycles += 2 + 1;
                            pad.off(PHASE).sw(c, 4);
                        } else {
                            c.cycles += 1;
                        }
                    }
                    if set_mode {
                        c.g[A0][0] = port(c);
                        c.cycles += 8;
                        c.g[A2][0] = 1;
                        c.g[A1][0] = slot(c);
                        c.g[A3][0] = 3;
                        call_ptr(c, SET_MAIN_MODE, 0x0032_7054)?;
                        let r = c.g[V0][0];
                        c.g[V1][0] = r;
                        c.g[V0][0] = 1;
                        if r == 1 {
                            c.cycles += 4;
                            c.g[V0][0] = 3;
                            pad.off(MODE_SET).sw(c, r);
                            pad.off(PHASE).sw(c, 3);
                        }
                    }
                }
                // Wait for the mode change to finish.
                3 => {
                    c.cycles += 6;
                    c.g[A0][0] = port(c);
                    c.g[A1][0] = slot(c);
                    call_ptr(c, GET_STATE, 0x0032_7088)?;
                    c.g[V1][0] = 5;
                    let done = c.g[V0][0] == 5;
                    c.g[V0][0] = 1;
                    if !done {
                        c.cycles += 2;
                        pad.off(PHASE).sw(c, 1);
                    }
                    c.cycles += 1;
                }
                // Pressure mode available?
                4 => {
                    c.cycles += 6;
                    c.g[A0][0] = port(c);
                    c.g[A1][0] = slot(c);
                    call_ptr(c, INFO_PRESS_MODE, 0x0032_70a8)?;
                    c.g[V1][0] = 1;
                    if c.g[V0][0] != 1 {
                        c.g[V0][0] = 7;
                        c.cycles += 2;
                        pad.off(PHASE).sw(c, 7);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 5;
                        pad.off(PHASE).sw(c, 5);
                    }
                    c.cycles += 1;
                }
                // Enter pressure mode.
                5 => {
                    c.cycles += 6;
                    c.g[A0][0] = port(c);
                    c.g[A1][0] = slot(c);
                    call_ptr(c, ENTER_PRESS_MODE, 0x0032_70d4)?;
                    c.g[V1][0] = 1;
                    if c.g[V0][0] == 1 {
                        c.cycles += 2 + 3 + 1;
                        c.g[V0][0] = 6;
                        pad.off(RETRY).sw(c, 0);
                        pad.off(PHASE).sw(c, 6);
                    }
                }
                // Wait for the request; 5 failures: pressure mode on.
                6 | 9 => {
                    let ret = if index + 1 == 6 {
                        0x0032_70f4
                    } else {
                        0x0032_7194
                    };
                    c.cycles += 6;
                    c.g[A0][0] = port(c);
                    c.g[A1][0] = slot(c);
                    call_ptr(c, GET_REQ_STATE, ret)?;
                    c.g[A0][0] = 1;
                    let req = c.g[V0][0];
                    if req == 1 {
                        if index + 1 == 9 {
                            c.cycles += 1;
                        }
                        c.cycles += 2;
                        pad.off(PHASE).sw(c, 1);
                        pad.off(MODE_SET).sw(c, 0);
                    } else {
                        c.cycles += 2;
                        if req != 0 {
                            pad.off(RETRY).sw(c, 0);
                        } else {
                            c.cycles += 5;
                            let tries = sx((pad.off(RETRY).lw(c) as u32).wrapping_add(1));
                            c.g[V0][0] = tries;
                            c.g[V1][0] = ((tries as i64) < 5) as u64;
                            pad.off(RETRY).sw(c, tries);
                            if (tries as i64) >= 5 {
                                c.cycles += 4;
                                if index + 1 == 6 {
                                    c.g[V0][0] = 7;
                                    pad.off(PRESSURE).sw(c, 1);
                                    pad.off(PHASE).sw(c, 7);
                                } else {
                                    c.g[V0][0] = 0xa;
                                    pad.off(ACTUATOR).sw(c, 1);
                                    pad.off(PHASE).sw(c, 0xa);
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                }
                // Motors present? Align them, else go reading.
                7 => {
                    c.cycles += 7;
                    c.g[A0][0] = port(c);
                    c.g[A2][0] = u64::MAX;
                    c.g[A1][0] = slot(c);
                    c.g[A3][0] = 0;
                    call_ptr(c, INFO_ACT, 0x0032_7140)?;
                    if c.g[V0][0] == 0 {
                        c.cycles += 1 + 2;
                        c.g[V0][0] = 0xa;
                        pad.off(PHASE).sw(c, 0xa);
                    } else {
                        c.g[V0][0] = 8;
                        c.cycles += 2;
                        pad.off(PHASE).sw(c, 8);
                    }
                    c.cycles += 1;
                }
                8 => {
                    c.cycles += 7;
                    c.g[A0][0] = port(c);
                    c.g[A2][0] = sx(gp.wrapping_add(ACT_ALIGN_GP));
                    c.g[A1][0] = slot(c);
                    call_ptr(c, SET_ACT_ALIGN, 0x0032_716c)?;
                    c.g[V1][0] = 1;
                    if c.g[V0][0] == 1 {
                        c.cycles += 1 + 3 + 1;
                        c.g[V0][0] = 9;
                        pad.off(RETRY).sw(c, 0);
                        pad.off(PHASE).sw(c, 9);
                    }
                }
                // Read; a bad read or a non-zero status byte reprobes.
                _ => {
                    c.cycles += 6;
                    c.g[A0][0] = port(c);
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A1][0] = slot(c);
                    call_ptr(c, READ, 0x0032_71e8)?;
                    let len = c.g[V0][0];
                    let status = Addr(sp(c)).lbu(c);
                    c.g[V0][0] = status;
                    if len != 0 {
                        c.cycles += 2;
                        read_ok = status == 0;
                    }
                    if read_ok {
                        c.g[S3][0] = 1;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 1;
                        pad.off(MODE_SET).sw(c, 0);
                        pad.off(PHASE).sw(c, 1);
                    }
                    c.cycles += 1;
                }
            }
        }
        c.g[S2][0] = 0;

        c.cycles += 2;
        let data = Addr(sp(c));
        c.g[V0][0] = data.off(2).lbu(c);
        let mut result = 0;
        if read_ok {
            result = OUT_COUNT;
            c.g[S2][0] = OUT_COUNT;
            let b2 = data.off(2).lbu(c);
            // d-pad
            c.cycles += 5;
            pad_button_late(c, b2 & 0x01 == 0);
            c.cycles += 6;
            c.g[V1][0] = b2;
            c.swc1(0, out);
            pad_button(c, b2, 0x08, out + 0x04);
            c.cycles += 4;
            pad_button(c, b2, 0x02, out + 0x08);
            c.cycles += 4;
            pad_button(c, b2, 0x04, out + 0x0c);

            c.cycles += 3;
            if pad.off(PRESSURE).lw(c) != 0 {
                // 12 pressure bytes * (1/255)
                c.cycles += 55 + 6;
                let scale = Addr(gp.wrapping_sub(PRESSURE_SCALE_GP)).read_fpu(c);
                // destination FPRs of bytes 8..=0x13, in order
                const FREGS: [usize; 12] = [2, 3, 7, 8, 9, 10, 11, 12, 6, 4, 5, 1];
                for (i, &f) in FREGS.iter().enumerate() {
                    let b = data.off(8 + i as u32).lbu(c);
                    let v = FpuF32::from_i32(b as i32) * scale;
                    c.f[f] = v.0;
                    Addr(out + 0x10 + 4 * i as u32).write_fpu(c, v);
                }
                c.f[0] = scale.0;
                c.g[V0][0] = data.off(0x12).lbu(c);
                c.g[V1][0] = data.off(0x13).lbu(c);
                c.g[A0][0] = data.off(0x0e).lbu(c);
                c.g[A1][0] = data.off(0x0f).lbu(c);
                c.g[A2][0] = data.off(0x10).lbu(c);
                c.g[A3][0] = data.off(0x11).lbu(c);
            } else {
                // digital face and shoulder buttons
                c.cycles += 4;
                pad_button(c, b2, 0x20, out + 0x10);
                c.cycles += 4;
                pad_button(c, b2, 0x80, out + 0x14);
                c.cycles += 4;
                pad_button(c, b2, 0x10, out + 0x18);
                c.cycles += 4;
                let b3 = data.off(3).lbu(c);
                pad_button_late(c, b2 & 0x40 == 0);
                c.cycles += 5;
                c.g[V1][0] = b3;
                c.swc1(0, out + 0x1c);
                pad_button(c, b3, 0x10, out + 0x20);
                c.cycles += 4;
                pad_button(c, b3, 0x20, out + 0x24);
                c.cycles += 4;
                pad_button(c, b3, 0x40, out + 0x28);
                c.cycles += 4;
                pad_button(c, b3, 0x80, out + 0x2c);
                c.cycles += 4;
                pad_button(c, b3, 0x04, out + 0x30);
                c.cycles += 4;
                pad_button_late(c, b3 & 0x08 == 0);
                c.cycles += 6;
                c.swc1(0, out + 0x34);
                pad_button(c, b3, 0x01, out + 0x38);
                c.cycles += 4;
                pad_button(c, b3, 0x02, out + 0x3c);
                c.g[V0][0] = b3 & 0x02;
            }

            // stick half-axes
            c.cycles += 3;
            let analog = pad.off(ANALOG).lw(c);
            c.g[V0][0] = analog;
            c.g[V1][0] = data.off(4).lbu(c);
            if analog == 0 {
                c.cycles += 8;
                for i in 0..8 {
                    Addr(out + 0x40 + 4 * i).sw(c, 0);
                }
            } else {
                c.cycles += 6 + 103;
                let scale = Addr(gp.wrapping_sub(STICK_SCALE_GP)).read_fpu(c);
                c.f[2] = scale.0;
                // bytes 4..=7: left x, left y, right x, right y
                let axis = |c: &mut Cpu, i: u32| {
                    let b = data.off(4 + i).lbu(c);
                    let ramps = pad_stick(c, b);
                    let neg = FpuF32::from_i32(ramps.0 as i32) * scale;
                    let pos = FpuF32::from_i32(ramps.2 as i32) * scale;
                    Addr(out + 0x40 + 8 * i).write_fpu(c, neg);
                    Addr(out + 0x44 + 8 * i).write_fpu(c, pos);
                    c.f[0] = neg.0;
                    c.f[1] = pos.0;
                    ramps
                };
                axis(c, 0);
                axis(c, 1);
                axis(c, 2);
                let (qn, fneg, qp, fpos, pos) = axis(c, 3);
                c.g[V1][0] = qn;
                c.g[A0][0] = fneg;
                c.g[V0][0] = qp;
                c.g[A3][0] = fpos;
                c.g[A1][0] = pos;
                c.g[A2][0] = 0x4f;
                for r in [T0, T1, T2, T3, T4, T5] {
                    c.g[r][0] = 0x4f;
                }
            }
        }

        // While reading: send changed motor values.
        c.cycles += 4;
        c.g[V1][0] = pad.off(PHASE).lw(c);
        c.g[V0][0] = result;
        if c.g[V1][0] == 0xa {
            c.cycles += 4;
            let act_small = pad.off(ACT_SMALL).lw(c);
            c.g[T0][0] = act_small;
            c.g[V0][0] = pad.off(LAST_SMALL).lw(c);
            let mut changed = act_small != c.g[V0][0];
            if !changed {
                c.cycles += 4;
                c.g[V1][0] = pad.off(ACT_BIG).lw(c);
                let last_big = pad.off(LAST_BIG).lw(c);
                c.g[V0][0] = result;
                if c.g[V1][0] != last_big {
                    c.cycles += 1;
                    changed = true;
                }
            }
            if changed {
                c.g[V0][0] = pad.off(ACT_SMALL).lbu(c);
                c.cycles += 11;
                c.g[A2][0] = sx(pad.0.wrapping_add(ACT_DIRECT));
                c.g[V1][0] = pad.off(ACT_BIG).lbu(c);
                c.g[A3][0] = pad.off(ACT_BIG).lw(c);
                c.g[A0][0] = pad.off(PORT).lw(c);
                c.g[A1][0] = pad.off(SLOT).lw(c);
                let (small, big) = (c.g[V0][0], c.g[V1][0]);
                pad.off(ACT_DIRECT).sb(c, small);
                pad.off(ACT_DIRECT + 1).sb(c, big);
                pad.off(LAST_SMALL).sw(c, act_small);
                let a3 = c.g[A3][0];
                pad.off(LAST_BIG).sw(c, a3);
                call_ptr(c, SET_ACT_DIRECT, 0x0032_7718)?;
                c.g[V0][0] = result;
            }
        }

        c.cycles += 7;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// InputVM_ReadPadChannel (0x003252f8)
// ---------------------------------------------------------------------------

/// The `switch (kind)` jump table of `InputVM_ReadPadChannel` in `.rodata`
/// and its case labels (kinds 0..=5).
pub(crate) const PAD_CHANNEL_TABLE: u32 = 0x0048_e310;

pub(crate) const PAD_CHANNEL_CASES: [u32; 6] = [
    0x0032_5320,
    0x0032_5344,
    0x0032_539c,
    0x0032_53d4,
    0x0032_53f8,
    0x0032_541c,
];

/// `InputVM.defKind`: the kind used for kind 5 ("default").
pub(crate) const PADVM_DEF_KIND: u32 = 0x0;

/// `InputVM.list`: the pad channel list (`{ int count; Entry e[]; }`).
pub(crate) const PADVM_LIST: u32 = 0x4;

/// One channel entry is 0x1c bytes; the list's entries start at +4.
pub(crate) const PAD_ENTRY_SIZE: u32 = 0x1c;

/// Entry fields (relative to `list + idx * 0x1c`): the float value (+4) and
/// the unsigned words read by kinds 1..=4.
pub(crate) const PAD_ENTRY_VALUE: u32 = 0x4;

pub(crate) const PAD_ENTRY_KIND_WORD: [u32; 5] = [0, 0x10, 0x8, 0xc, 0x14];

/// `float InputVM_ReadPadChannel(InputVM* self, int idx, int kind)`
/// (0x003252f8): the value of channel `idx` of the input VM's pad channel
/// list, as a float. Kind 0 reads the entry's float, kinds 1..=4 one of its
/// unsigned words (converted unsigned to float), kind 5 retries with the
/// VM's default kind; an index past the count gives 0 (kinds 0) or the
/// conversion of 0, an unknown kind 0.0.
pub fn input_vm_read_pad_channel(c: &mut Cpu) {
    c.cycles += 1;
    c.g[A3][0] = c.g[A0][0];
    let this = r32(c, A3);
    let idx = c.g[A1][0];

    // 0x003252fc: the switch head (kind 5 comes back here).
    loop {
        c.cycles += 3;
        let kind = c.g[A2][0];
        if kind >= 6 {
            c.g[V0][0] = 0x49_0000;
            break;
        }
        c.cycles += 6;
        let entry = PAD_CHANNEL_TABLE.wrapping_add((kind as u32) << 2);
        c.g[V1][0] = sx(entry);
        c.g[V0][0] = sx(PAD_CHANNEL_TABLE);
        c.g[A0][0] = sx(PAD_CHANNEL_CASES[kind as usize]);

        if kind == 5 {
            // 0x0032541c: kind = self->defKind, back to the head.
            c.cycles += 2;
            c.g[A2][0] = Addr(this.wrapping_add(PADVM_DEF_KIND)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0032_52fc);
            }
            continue;
        }

        // Every case: `idx < list->count` (64-bit signed compare).
        c.cycles += if kind == 1 { 3 } else { 5 };
        let list = Addr(this.wrapping_add(PADVM_LIST)).lw(c);
        c.g[A0][0] = list;
        let count = Addr(list as u32).lw(c);
        let in_range = (idx as i64) < (count as i64);
        if kind == 1 {
            c.cycles += 2;
        }
        c.g[V0][0] = PAD_ENTRY_SIZE as u64;

        if kind == 0 {
            if !in_range {
                break;
            }
            // 0x00325334: f0 = entry.value.
            c.cycles += 4;
            let off = abi::mult(c, idx as u32, PAD_ENTRY_SIZE);
            c.g[V1][0] = sx(off);
            let at = off.wrapping_add(list as u32);
            c.g[V0][0] = sx(at);
            c.f[0] = Addr(at.wrapping_add(PAD_ENTRY_VALUE)).read_u32(c);
            abi::ret(c);
            return;
        }

        // Kinds 1..=4: the entry's word, or 0 past the count.
        let word = if in_range {
            c.cycles += 4;
            let off = abi::mult(c, idx as u32, PAD_ENTRY_SIZE);
            c.g[V1][0] = sx(off);
            let at = off.wrapping_add(list as u32);
            c.g[V0][0] = sx(at);
            let w = Addr(at.wrapping_add(PAD_ENTRY_KIND_WORD[kind as usize])).lw(c);
            if kind == 3 || kind == 4 {
                c.g[V0][0] = w;
                if event_due(c) {
                    deliver_event_here(c, 0x0032_53c4);
                }
            }
            w
        } else {
            // (Kinds 3 and 4 jump back to 0x003253c0.)
            if kind >= 3 && event_due(c) {
                deliver_event_here(c, 0x0032_53c0);
            }
            c.cycles += 1;
            0
        };
        c.g[V0][0] = word;

        // 0x0032536c / 0x003253c4: unsigned int to float.
        c.cycles += 2;
        let half = sx((word as u32) >> 1);
        c.g[V1][0] = half;
        if (word as i64) >= 0 {
            if kind != 1 && event_due(c) {
                deliver_event_here(c, 0x0032_5374);
            }
            // 0x00325374
            c.cycles += 4;
            c.f[0] = FpuF32::from_i32(word as i32).0;
        } else {
            // (x >> 1 | x & 1) converted and doubled.
            c.cycles += if kind == 1 { 1 } else { 2 };
            c.g[V0][0] = word & 1;
            if kind != 1 && event_due(c) {
                deliver_event_here(c, 0x0032_5388);
            }
            // 0x00325388
            c.cycles += 5;
            let v = c.g[V0][0] | half;
            c.g[V0][0] = v;
            let f = FpuF32::from_i32(v as i32);
            c.f[0] = (f + f).0;
        }
        abi::ret(c);
        return;
    }

    // 0x00325424: no value.
    c.cycles += 3;
    c.f[0] = 0;
    abi::ret(c);
}

pub(crate) mod pad_queue {

    pub const TAIL: u32 = 0x2ee4;

    pub const COUNT: u32 = 0x2ee8;

    pub const DEVICES: u32 = 0x2eec;

    pub const FRAME_SIZE: u64 = 0x190;

    pub const RECORD_SIZE: u32 = 0x64;

    pub const READ_SIZE: u64 = 0x18;

    pub const FRAMES: u64 = 30;

    /// Device vtable: `read(data, size)` at `+0x10`.
    pub const VT_READ: u32 = 0x10;
}

/// `void PadQueue_Poll(PadQueue* q)` (0x326b88): read every device into
/// the queue's current frame (`record.result = dev->read(record.data,
/// 0x18)`), then advance the write index modulo 30.
pub fn pad_queue_poll(c: &mut Cpu) {
    use pad_queue::*;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S3, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        let q = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(q.wrapping_add(COUNT)).lw(c);
        c.g[S3][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[S4][0] = FRAME_SIZE;
            c.g[S2][0] = 0;
            c.g[S1][0] = sx(q.wrapping_add(DEVICES));
            loop {
                // Device s3 (s1 its pointer, s2 its record offset).
                c.cycles += 23;
                c.g[A1][0] = Addr(q.wrapping_add(TAIL)).lw(c);
                c.g[A2][0] = READ_SIZE;
                c.g[A3][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.mult(0, V0, A1, S4);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
                let dev = c.g[A3][0] as u32;
                c.g[V1][0] = Addr(dev).lw(c);
                let vt = c.g[V1][0] as u32;
                c.g[A0][0] = Addr(vt.wrapping_add(VT_READ)).lh(c);
                let record = (c.g[V0][0] as u32)
                    .wrapping_add(q)
                    .wrapping_add(c.g[S2][0] as u32);
                c.g[A1][0] = sx(record);
                c.g[V0][0] = Addr(vt.wrapping_add(VT_READ + 4)).lw(c);
                c.g[A0][0] = sx(dev.wrapping_add(c.g[A0][0] as u32));
                let read = c.g[V0][0] as u32;
                c.g[A1][0] = sx(record.wrapping_add(4));
                call_ptr(c, read, 0x0032_6bf8)?;
                // The record of the (re-read) current frame gets the result.
                c.g[V1][0] = Addr(q.wrapping_add(TAIL)).lw(c);
                c.mult(0, A0, V1, S4);
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[V1][0] = sx(q.wrapping_add(c.g[V1][0] as u32));
                let result = c.g[V0][0];
                Addr(c.g[V1][0] as u32).sw(c, result);
                c.g[V0][0] = Addr(q.wrapping_add(COUNT)).lw(c);
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(RECORD_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_6bc0);
                }
            }
        }
        c.cycles += 4;
        c.g[V1][0] = Addr(q.wrapping_add(TAIL)).lw(c);
        c.g[V0][0] = FRAMES;
        c.cycles += 12;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.divu(0, V1, V0);
        c.g[A0][0] = c.hi[0];
        let tail = c.g[A0][0];
        Addr(q.wrapping_add(TAIL)).sw(c, tail);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x50);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod libpad {
    use super::*;

    /// The port/slot table (`0x526250 + port * 0x70 + slot * 0x1c`).
    pub const PORTS: u32 = 0x0052_6250;

    pub const PORT_STRIDE: u32 = 0x70;

    pub const SLOT_STRIDE: u32 = 0x1c;

    /// Non-zero while the port is open.
    pub const OPEN: u32 = 0x10;

    /// The DMA buffer's state byte (6: stable) and request state (2: busy).
    pub const STATE: u32 = 0x70;

    pub const REQ_STATE: u32 = 0x71;

    /// `u8* libpad_GetDmaBuffer(int port, int slot)` (0x3ff9a8).
    pub const GET_DMA_BUFFER: Callee = Callee {
        addr: 0x003f_f9a8,
        f: ee::s15::libpad_GetDmaBuffer,
    };
}

pub(crate) const PAD_OPEN: u32 = 0x10;

/// Offset of the data length word in a pad DMA buffer half.
pub(crate) const PAD_DATA_LEN: u32 = 0x60;

pub(crate) const D_INTR: Callee = Callee {
    addr: 0x0042_c078,
    f: ee::s15::f_0042c078,
};

pub(crate) const E_INTR: Callee = Callee {
    addr: 0x0042_c0c0,
    f: ee::s15::f_0042c0c0,
};

pub(crate) const LIBPAD_GET_DMA_BUFFER: Callee = Callee {
    addr: 0x003f_f9a8,
    f: ee::s15::f_003ff9a8,
};

/// `int scePadRead(int port, int slot, u8* data)` (0x3ffa58): 0 when the port
/// is closed; otherwise, with interrupts off, copy the current 0x80-byte DMA
/// half to the stack, copy its first `len` (`+0x60`) bytes to `data`, turn
/// interrupts back on if they were, and return `len`.
pub fn sce_pad_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xe0);
        c.g[V1][0] = PAD_PORT_SIZE;
        c.sd(frame.at(c, 0x90), c.g[S1][0]);
        c.g[V0][0] = 0x52_0000;
        c.sd(frame.at(c, 0xa0), c.g[S2][0]);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        c.g[A0][0] = PAD_SLOT_SIZE;
        c.mult(1, V1, S1, V1);
        c.mult(0, A0, S2, A0);
        c.sd(frame.at(c, 0xc0), c.g[S4][0]);
        c.g[V0][0] = sx(PAD_PORTS);
        frame.save_ra(c, 0xd0);
        c.sd(frame.at(c, 0xb0), c.g[S3][0]);
        c.g[A0][0] = add32(c.g[A0][0], c.g[V1][0]);
        c.sd(frame.at(c, 0x80), c.g[S0][0]);
        c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
        c.g[V1][0] = Addr(r32(c, V0) + PAD_OPEN).lw(c);
        c.g[S4][0] = c.g[A2][0];
        if c.g[V1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 11;
            D_INTR.call(c, 0x003f_fab8)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[S0][0] = c.g[SP][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            LIBPAD_GET_DMA_BUFFER.call(c, 0x003f_facc)?;
            // Copy the 0x80-byte half to the stack, 32 bytes at_m2 a time.
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = c.g[V1][0] & 7;
            let aligned = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[SP][0];
            c.cycles += 1;
            c.g[V0][0] = add32(c.g[V1][0], 0x80);
            if aligned {
                loop {
                    c.cycles += 13;
                    let (src, dst) = (r32(c, V1), r32(c, A0));
                    for (k, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                        c.g[reg][0] = c.ld(src + 8 * k as u32);
                    }
                    for (k, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                        c.sd(dst + 8 * k as u32, c.g[reg][0]);
                    }
                    c.g[V1][0] = add32(c.g[V1][0], 0x20);
                    c.g[A0][0] = add32(c.g[A0][0], 0x20);
                    if c.g[V1][0] == c.g[V0][0] {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_fb40);
                    }
                }
                c.cycles += 1;
            } else {
                loop {
                    c.cycles += 21;
                    let (src, dst) = (r32(c, V1), r32(c, A0));
                    for (k, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                        let at_m2 = src + 8 * k as u32;
                        c.ldl(reg, at_m2 + 7);
                        c.ldr(reg, at_m2);
                    }
                    for (k, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                        let at_m2 = dst + 8 * k as u32;
                        c.sdl(reg, at_m2 + 7);
                        c.sdr(reg, at_m2);
                    }
                    c.g[V1][0] = add32(c.g[V1][0], 0x20);
                    c.g[A0][0] = add32(c.g[A0][0], 0x20);
                    if c.g[V1][0] == c.g[V0][0] {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_fae0);
                    }
                }
                c.cycles += 2;
            }
            let buf = r32(c, S0);
            c.g[A2][0] = Addr(buf + PAD_DATA_LEN).lw(c);
            c.cycles += 6;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S0][0];
            MEMCPY.call(c, 0x003f_fb84)?;
            c.g[V1][0] = 1;
            if c.g[S3][0] == c.g[V1][0] {
                c.cycles += 3;
                E_INTR.call(c, 0x003f_fb98)?;
            }
            c.g[V0][0] = Addr(buf + PAD_DATA_LEN).lw(c);
        }
        c.cycles += 8;
        frame.restore_ra(c, 0xd0);
        c.g[S4][0] = c.ld(frame.at(c, 0xc0));
        c.g[S3][0] = c.ld(frame.at(c, 0xb0));
        c.g[S2][0] = c.ld(frame.at(c, 0xa0));
        c.g[S1][0] = c.ld(frame.at(c, 0x90));
        c.g[S0][0] = c.ld(frame.at(c, 0x80));
        frame.close(c);
        Ok(())
    })
}

/// `int scePadGetState(int port, int slot)` (0x3ffbc0): 0x63 when the
/// port is not open; otherwise the DMA buffer's state byte, except that a
/// stable pad (6) with a request still running (2) reports 5 (`scePadStateExecCmd`).
pub fn sce_pad_get_state(c: &mut Cpu) {
    use libpad::*;
    guest(c, |c| {
        c.cycles += 14;
        c.g[A2][0] = c.g[A0][0];
        c.g[V1][0] = PORT_STRIDE as u64;
        c.g[A0][0] = SLOT_STRIDE as u64;
        c.mult(1, V1, A2, V1); // port * 0x70
        c.mult(0, A0, A1, A0); // slot * 0x1c
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(PORTS);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        c.g[V0][0] = sx(PORTS.wrapping_add(r32(c, A0)));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(OPEN));
        let open = c.g[V1][0] != 0;
        c.g[V0][0] = 0x63;
        if !open {
            c.cycles += 1;
        } else {
            c.cycles += 7;
            c.g[A0][0] = c.g[A2][0];
            GET_DMA_BUFFER.call(c, 0x003f_fc00)?;
            c.g[A0][0] = c.g[V0][0];
            let buf = r32(c, A0);
            c.g[V1][0] = 6;
            c.g[V0][0] = c.lbu(buf + STATE);
            let stable = c.g[V0][0] == c.g[V1][0];
            if stable {
                c.cycles += 4;
                c.g[V1][0] = c.lbu(buf + REQ_STATE);
                c.g[V0][0] = 2;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V0][0] = c.lbu(buf + STATE);
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 5;
                }
            }
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// InputParser_NextToken (`cInputMapParser_readToken`)

/// The tokenizer's fields (`sTok` in the reference): the source text
/// (`base`, `len`), the read position, the `#line` file name, the line
/// counter, "at the start of a line", the error flag, the token kind, its
/// text and its numeric value.
pub(crate) const NEXT_TOKEN_BASE: u32 = 0x0;

pub(crate) const NEXT_TOKEN_LEN: u32 = 0x4;

pub(crate) const NEXT_TOKEN_POS: u32 = 0x8;

pub(crate) const NEXT_TOKEN_FNAME: u32 = 0xc;

pub(crate) const NEXT_TOKEN_LINE: u32 = 0x8c;

pub(crate) const NEXT_TOKEN_LINE_START: u32 = 0x90;

#[cfg(test)]
pub(crate) const NEXT_TOKEN_ERROR: u32 = 0x94;

pub(crate) const NEXT_TOKEN_KIND: u32 = 0x98;

pub(crate) const NEXT_TOKEN_TEXT: u32 = 0x9c;

pub(crate) const NEXT_TOKEN_VALUE: u32 = 0xdc;

/// `InputParser_Error(parser, fmt, ...)`.
pub(crate) const NEXT_TOKEN_ERROR_FN: Callee = Callee {
    addr: 0x0032_1500,
    f: ee::s10::InputParser_Error,
};

/// `strcasecmp` and `sscanf` (called through the function table).
pub(crate) const NEXT_TOKEN_STRCASECMP: u32 = 0x0041_aa88;

pub(crate) const NEXT_TOKEN_SSCANF: u32 = 0x0041_78b0;

#[inline(always)]
pub(crate) fn nt_r(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// Address of a tokenizer field (`s0` holds the tokenizer).
#[inline(always)]
pub(crate) fn nt_f(c: &Cpu, off: u32) -> u32 {
    nt_r(c, S0).wrapping_add(off)
}

/// `(int8)(x << 24) >> 24` of a register value.
#[inline(always)]
pub(crate) fn nt_sext8(v: u64) -> u64 {
    sx((((v as u32) << 24) as i32 >> 24) as u32)
}

/// `v1 = len; a0 = pos; v0 = (pos < v0 + len)` with `v0` = base already
/// loaded: is there input left?
pub(crate) fn nt_more_v0(c: &mut Cpu) -> bool {
    c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
    c.g[A0][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
    c.g[V0][0] = sx(nt_r(c, V0).wrapping_add(nt_r(c, V1)));
    c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
    c.g[V0][0] != 0
}

/// `nt_more_v0` loading the base first.
pub(crate) fn nt_more(c: &mut Cpu) -> bool {
    c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
    nt_more_v0(c)
}

/// Step past the current character (the 7-cycle inline form).
pub(crate) fn nt_adv7(c: &mut Cpu) {
    c.cycles += 7;
    let more = nt_more(c);
    c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
    if more {
        c.cycles += 1;
        c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
    }
}

/// `s2` = the current character (unsigned byte) or -1 at the end.
pub(crate) fn nt_peek_s2(c: &mut Cpu) {
    c.cycles += 7;
    let more = nt_more(c);
    c.g[S2][0] = u64::MAX;
    if more {
        c.cycles += 1;
        c.g[S2][0] = c.lbu(nt_r(c, A0));
    }
}

/// Append register `src`'s byte to the token text (`s1`) and step past it.
pub(crate) fn nt_push_adv(c: &mut Cpu, src: usize) {
    c.cycles += 8;
    c.sb(nt_r(c, S1), c.g[src][0]);
    let more = nt_more(c);
    c.g[S1][0] = sx(nt_r(c, S1).wrapping_add(1));
    if more {
        c.cycles += 2;
        c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
        c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
    }
}

/// `(unsigned)(ch - '0') < 10` into `v0`.
#[inline(always)]
pub(crate) fn nt_digit(c: &mut Cpu, ch: u64) -> bool {
    c.g[V0][0] = sx((ch as u32).wrapping_sub(0x30));
    c.g[V0][0] = (c.g[V0][0] < 10) as u64;
    c.g[V0][0] != 0
}

/// Whitespace test, the shape that loads 9 and 13 into `v0` (the first
/// compare, against ' ', and `flag = 0` are the caller's).
pub(crate) fn nt_space_v0(c: &mut Cpu, ch: u64, flag: usize) {
    if ch == 0x20 {
        c.cycles += 1;
        c.g[flag][0] = 1;
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = 9;
    if ch == 9 {
        c.g[flag][0] = 1;
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0xd;
    if ch == 0xa {
        c.cycles += 1;
        c.g[flag][0] = 1;
        return;
    }
    c.cycles += 2;
    if ch == 0xd {
        c.cycles += 1;
        c.g[flag][0] = 1;
    }
}

/// Whitespace test with the constants held in registers.
pub(crate) fn nt_space_t(c: &mut Cpu, ch: u64, flag: usize) {
    if ch == 0x20 {
        c.cycles += 1;
        c.g[flag][0] = 1;
        return;
    }
    c.cycles += 2;
    if ch == 9 {
        c.g[flag][0] = 1;
        return;
    }
    c.cycles += 2;
    if ch == 0xa {
        c.g[flag][0] = 1;
        return;
    }
    c.cycles += 2;
    if ch == 0xd {
        c.cycles += 1;
        c.g[flag][0] = 1;
    }
}

/// `v0` on leaving `nt_blank_run`: base (`late`) or `a3 + a2` (the end).
pub(crate) fn nt_blank_exit(c: &mut Cpu, late: bool) {
    c.g[V0][0] = if late {
        c.lw(nt_f(c, NEXT_TOKEN_BASE))
    } else {
        sx(nt_r(c, A3).wrapping_add(nt_r(c, A2)))
    };
}

/// `while (isWh(pk) && pk != -1 && pk != '\n') adv` with `a1` = end of
/// input; `end_reg` holds the end too. On exit `v0` = end (`late` = false)
/// or base (`late`), `a0` = position.
pub(crate) fn nt_blank_run(c: &mut Cpu, ev_head: u32, end_reg: usize, late: bool) {
    let exit = nt_blank_exit;
    loop {
        c.cycles += 5;
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
        c.g[V1][0] = u64::MAX;
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[V1][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 4;
        c.g[V0][0] = nt_sext8(c.g[V1][0]);
        c.g[V1][0] = 0;
        let ch = c.g[V0][0];
        nt_space_t(c, ch, V1);
        c.cycles += 2;
        c.g[V0][0] = (c.g[A0][0] < c.g[end_reg][0]) as u64;
        if c.g[V1][0] == 0 {
            c.cycles += 1;
            exit(c, late);
            return;
        }
        if late {
            c.cycles += 2;
        }
        if event_due(c) {
            deliver_event_here(c, ev_head);
        }
        c.cycles += 2;
        c.g[V1][0] = u64::MAX;
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[V1][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 2;
        if c.g[V1][0] == u64::MAX {
            exit(c, late);
            return;
        }
        c.cycles += 2;
        c.g[V1][0] = u64::MAX;
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[V1][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 2;
        let newline = c.g[V1][0] == 0xa;
        exit(c, late);
        if newline {
            return;
        }
        let more = if late {
            c.cycles += 6;
            nt_more_v0(c)
        } else {
            c.cycles += 7;
            nt_more(c)
        };
        c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
        if more {
            c.cycles += 1;
            c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
        }
    }
}

/// `InputParser_NextToken` (0x321af0, `cInputMapParser_readToken(parser)`):
/// read the next token of an input-map script. Skips whitespace, `//`
/// comments and `# line "file"` directives (which set the line counter and
/// file name for error messages), then reads an operator, an identifier
/// (keywords `max`..`sqrt` and `minradian` get their own kinds), a number
/// (parsed into `+0xdc` with `sscanf("%f")`) or reports an unexpected
/// character. The kind goes to `+0x98`, the text to `+0x9c`.
pub fn input_parser_next_token(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S3, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S5, 0x30);
        c.g[S3][0] = 0xa;
        frame.save(c, S1, 0x70);
        c.g[S5][0] = 0x0049_0000;
        frame.save(c, S2, 0x60);
        frame.save(c, S4, 0x40);
        frame.save(c, S6, 0x20);
        frame.save(c, S7, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(nt_r(c, S0));

        nt_skip_blank(c)?;
        let store = nt_token(c)?;
        if store {
            c.cycles += 1;
            c.sw(nt_f(c, NEXT_TOKEN_KIND), c.g[V0][0]);
        }

        c.cycles += 11;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The whitespace loop: skip blanks, newlines (counting lines), comments and
/// line directives while the current character is whitespace. Leaves
/// `v0` = base, `s2` = the last character looked at, `s4` = token text.
pub(crate) fn nt_skip_blank(c: &mut Cpu) -> Result<(), Unwound> {
    loop {
        // 0x321b28: peek (v0 = base).
        c.cycles += 8;
        c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
        c.g[A0][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
        c.g[A3][0] = c.g[V0][0];
        c.g[V0][0] = sx(nt_r(c, V0).wrapping_add(nt_r(c, V1)));
        c.g[A2][0] = c.g[V1][0];
        c.g[V1][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
        c.g[S2][0] = u64::MAX;
        if c.g[V1][0] != 0 {
            c.cycles += 1;
            c.g[S2][0] = c.lbu(nt_r(c, A0));
        }
        let ready = nt_skip_one(c)?;
        if !ready {
            c.cycles += 1;
            c.g[V1][0] = sx(nt_r(c, S2) << 24);
        }
        // 0x3222fc: loop while the character is whitespace.
        c.cycles += 4;
        c.g[V1][0] = sx(((nt_r(c, V1)) as i32 >> 24) as u32);
        c.g[V0][0] = 0x20;
        c.g[A0][0] = 0;
        let ch = c.g[V1][0];
        nt_space_v0(c, ch, A0);
        c.cycles += 2;
        c.g[V0][0] = c.lw(nt_r(c, S0));
        if c.g[A0][0] == 0 {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_1b28);
        }
    }
}

/// One step of the whitespace loop for the character in `s2`. True when `v1`
/// already holds `s2 << 24`.
pub(crate) fn nt_skip_one(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 2;
    c.g[V0][0] = 0x23;
    if c.g[S2][0] == c.g[S3][0] {
        // Newline: step, line start, count the line.
        c.cycles += 2;
        let more = c.g[V1][0] != 0;
        c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
        if more {
            c.cycles += 1;
            c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
        }
        c.cycles += 7;
        c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_LINE));
        c.g[V0][0] = 1;
        c.sw(nt_f(c, NEXT_TOKEN_LINE_START), c.g[V0][0]);
        c.g[S4][0] = sx(nt_f(c, NEXT_TOKEN_TEXT));
        c.g[V1][0] = sx(nt_r(c, V1).wrapping_add(1));
        c.sw(nt_f(c, NEXT_TOKEN_LINE), c.g[V1][0]);
        return Ok(false);
    }
    c.cycles += 2;
    let hash = c.g[S2][0] == c.g[V0][0];
    c.g[V0][0] = 0x2f;
    if hash {
        c.cycles += 3;
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_LINE_START));
        let line_start = c.g[V0][0] != 0;
        c.g[V0][0] = 0x2f;
        if line_start {
            return nt_directive(c);
        }
    }
    // 0x3221ec: `//` comment?
    c.cycles += 2;
    let slash = c.g[S2][0] == c.g[V0][0];
    c.g[V1][0] = sx(nt_r(c, S2) << 24);
    if slash {
        c.cycles += 5;
        c.g[V0][0] = sx(nt_r(c, A3).wrapping_add(nt_r(c, A2)));
        c.g[V1][0] = sx(nt_r(c, A0).wrapping_add(1));
        c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
        c.g[V0][0] = u64::MAX;
        if c.g[V1][0] != 0 {
            c.cycles += 1;
            c.g[V0][0] = c.lbu(nt_r(c, A0).wrapping_add(1));
        }
        c.cycles += 2;
        let second = c.g[V0][0] == c.g[S2][0];
        c.g[V1][0] = sx(nt_r(c, S2) << 24);
        if second {
            nt_line_comment(c);
            return Ok(false);
        }
    }
    // 0x3222ac: other whitespace is stepped over.
    c.cycles += 4;
    c.g[V1][0] = sx((nt_r(c, V1) as i32 >> 24) as u32);
    c.g[V0][0] = 0x20;
    c.g[A1][0] = 0;
    let ch = c.g[V1][0];
    nt_space_v0(c, ch, A1);
    c.cycles += 2;
    c.g[S4][0] = sx(nt_f(c, NEXT_TOKEN_TEXT));
    if c.g[A1][0] != 0 {
        c.cycles += 4;
        c.g[V0][0] = sx(nt_r(c, A3).wrapping_add(nt_r(c, A2)));
        c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
        if more {
            c.cycles += 1;
            c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
        }
    }
    Ok(false)
}

/// `//`: step over both slashes and the rest of the line (up to the
/// newline); the character becomes ' '.
pub(crate) fn nt_line_comment(c: &mut Cpu) {
    c.cycles += 7;
    c.g[A0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
    c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
    c.g[A1][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
    c.g[A0][0] = sx(nt_r(c, A0).wrapping_add(nt_r(c, V1)));
    c.g[V0][0] = (c.g[A1][0] < c.g[A0][0]) as u64;
    let more = c.g[V0][0] != 0;
    c.g[V0][0] = sx(nt_r(c, A1).wrapping_add(1));
    if more {
        c.cycles += 3;
        c.g[V1][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
        c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
        if c.g[V1][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = sx(nt_r(c, A1).wrapping_add(2));
            c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
        }
    }
    c.cycles += 2;
    c.g[S4][0] = sx(nt_f(c, NEXT_TOKEN_TEXT));
    loop {
        c.cycles += 8;
        c.g[A0][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
        c.g[A2][0] = u64::MAX;
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
        c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
        c.g[V0][0] = sx(nt_r(c, V0).wrapping_add(nt_r(c, A0)));
        c.g[A1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
        c.g[A0][0] = c.g[V1][0];
        if c.g[A1][0] != 0 {
            c.cycles += 1;
            c.g[A2][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 2;
        if c.g[A2][0] == c.g[S3][0] {
            c.cycles += 1;
            c.sw(nt_f(c, NEXT_TOKEN_LINE_START), 0);
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_2250);
        }
        c.cycles += 2;
        c.g[V1][0] = u64::MAX;
        if c.g[A1][0] != 0 {
            c.cycles += 1;
            c.g[V1][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        if c.g[V1][0] == c.g[V0][0] {
            c.sw(nt_f(c, NEXT_TOKEN_LINE_START), 0);
            break;
        }
        c.cycles += 2;
        c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
        if c.g[A1][0] != 0 {
            c.cycles += 1;
            c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
        }
    }
    c.cycles += 2;
    c.g[S2][0] = 0x20;
}

/// `#` at the start of a line: `# <line> "<file>"`. Sets the line counter
/// and the file name, reporting "malformed pre-processor line" (and skipping
/// to the end of the line) when the shape is wrong. True when `v1` already
/// holds `s2 << 24`.
pub(crate) fn nt_directive(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 2;
    let more = c.g[V1][0] != 0;
    c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
    if more {
        c.cycles += 1;
        c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
    }
    c.cycles += 2;
    c.g[S4][0] = sx(nt_f(c, NEXT_TOKEN_TEXT));
    // Blanks before the line number.
    loop {
        c.cycles += 9;
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
        c.g[A1][0] = u64::MAX;
        c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
        c.g[A0][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
        c.g[A3][0] = c.g[V0][0];
        c.g[V0][0] = sx(nt_r(c, V0).wrapping_add(nt_r(c, V1)));
        c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
        c.g[A2][0] = c.g[V1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[A1][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 5;
        c.g[V1][0] = sx(nt_r(c, A1) << 24);
        c.g[V0][0] = 0x20;
        c.g[V1][0] = sx((nt_r(c, V1) as i32 >> 24) as u32);
        c.g[A1][0] = 0;
        let ch = c.g[V1][0];
        nt_space_v0(c, ch, A1);
        c.cycles += 2;
        c.g[V0][0] = sx(nt_r(c, A3).wrapping_add(nt_r(c, A2)));
        if c.g[A1][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_1ba8);
        }
        c.cycles += 3;
        c.g[V1][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
        c.g[A1][0] = u64::MAX;
        if c.g[V1][0] != 0 {
            c.cycles += 1;
            c.g[A1][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        if c.g[A1][0] == c.g[V0][0] {
            c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
            break;
        }
        c.cycles += 2;
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 2;
        let newline = c.g[V0][0] == c.g[S3][0];
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
        if newline {
            break;
        }
        c.cycles += 6;
        let more = nt_more_v0(c);
        c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
        if more {
            c.cycles += 1;
            c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
        }
    }
    // 0x321c58: a digit?
    c.cycles += 6;
    let more = nt_more_v0(c);
    c.g[A1][0] = u64::MAX;
    if more {
        c.cycles += 1;
        c.g[A1][0] = c.lbu(nt_r(c, A0));
    }
    c.cycles += 6;
    let ch = nt_sext8(c.g[A1][0]);
    if nt_digit(c, ch) {
        c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
        return nt_directive_line(c);
    }
    // Malformed: skip to the end of the line.
    c.cycles += 10;
    c.g[A1][0] = sx(nt_r(c, S5).wrapping_sub(0x2218));
    c.g[A0][0] = c.g[S0][0];
    NEXT_TOKEN_ERROR_FN.call(c, 0x0032_1c98)?;
    c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
    c.g[T0][0] = 0xa;
    c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
    c.g[A3][0] = u64::MAX;
    c.g[A1][0] = sx(nt_r(c, V1).wrapping_add(nt_r(c, V0)));
    c.g[A2][0] = c.g[A1][0];
    loop {
        c.cycles += 5;
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
        c.g[V1][0] = u64::MAX;
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[V1][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 2;
        let newline = c.g[V1][0] == c.g[T0][0];
        c.g[V0][0] = (c.g[A0][0] < c.g[A2][0]) as u64;
        if newline {
            return Ok(false);
        }
        c.cycles += 2;
        if event_due(c) {
            deliver_event_here(c, 0x0032_1cb8);
        }
        c.cycles += 2;
        c.g[V1][0] = u64::MAX;
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[V1][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 2;
        let end = c.g[V1][0] == c.g[A3][0];
        c.g[V1][0] = sx(nt_r(c, S2) << 24);
        if end {
            return Ok(true);
        }
        c.cycles += 7;
        let more = nt_more(c);
        c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
        if more {
            c.cycles += 1;
            c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
        }
    }
}

/// The directive's line number and quoted file name (`v1` = base, `a0` =
/// position).
pub(crate) fn nt_directive_line(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 5;
    c.g[A3][0] = 0xa;
    c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
    c.sw(nt_f(c, NEXT_TOKEN_LINE), 0);
    c.g[A2][0] = sx(nt_r(c, V1).wrapping_add(nt_r(c, V0)));
    // line = line * 10 + digit
    loop {
        c.cycles += 4;
        c.g[A0][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
        c.g[V0][0] = (c.g[A0][0] < c.g[A2][0]) as u64;
        c.g[V1][0] = u64::MAX;
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[V1][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 6;
        let ch = nt_sext8(c.g[V1][0]);
        let digit = nt_digit(c, ch);
        c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
        if !digit {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_1d28);
        }
        c.cycles += 5;
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
        c.g[V1][0] = sx(nt_r(c, V1).wrapping_add(nt_r(c, V0)));
        c.g[V1][0] = (c.g[A0][0] < c.g[V1][0]) as u64;
        c.g[A1][0] = u64::MAX;
        if c.g[V1][0] != 0 {
            c.cycles += 1;
            c.g[A1][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 6;
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_LINE));
        c.mult(0, V0, V0, A3);
        c.g[V0][0] = sx(nt_r(c, V0).wrapping_sub(0x30));
        c.g[V0][0] = sx(nt_r(c, V0).wrapping_add(nt_r(c, A1)));
        let more = c.g[V1][0] != 0;
        c.sw(nt_f(c, NEXT_TOKEN_LINE), c.g[V0][0]);
        if more {
            c.cycles += 2;
            c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
            c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
        }
    }
    // Blanks, then the opening quote.
    c.cycles += 11;
    c.g[T5][0] = 0x20;
    c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
    c.g[T4][0] = 9;
    c.g[A3][0] = c.g[V1][0];
    c.g[T0][0] = 0xa;
    c.g[A1][0] = sx(nt_r(c, V1).wrapping_add(nt_r(c, V0)));
    c.g[A2][0] = c.g[V0][0];
    c.g[T1][0] = c.g[A1][0];
    c.g[T3][0] = 0xd;
    c.g[T2][0] = u64::MAX;
    nt_blank_run(c, 0x0032_1db8, T1, false);
    c.cycles += 3;
    c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
    c.g[V1][0] = u64::MAX;
    if c.g[V0][0] != 0 {
        c.cycles += 1;
        c.g[V1][0] = c.lbu(nt_r(c, A0));
    }
    c.cycles += 3;
    c.g[V0][0] = 0x22;
    let quote = c.g[V1][0] == c.g[V0][0];
    c.g[A1][0] = sx(nt_r(c, S5).wrapping_sub(0x2218));
    if !quote {
        // Malformed: skip to the end of the line; the character becomes ' '.
        c.cycles += 9;
        c.g[A0][0] = c.g[S0][0];
        NEXT_TOKEN_ERROR_FN.call(c, 0x0032_1e74)?;
        c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
        c.g[T0][0] = 0xa;
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
        c.g[A3][0] = u64::MAX;
        c.g[A1][0] = sx(nt_r(c, V1).wrapping_add(nt_r(c, V0)));
        c.g[A2][0] = c.g[A1][0];
        loop {
            c.cycles += 5;
            c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
            c.g[A0][0] = c.g[V0][0];
            c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
            c.g[V1][0] = u64::MAX;
            if c.g[V0][0] != 0 {
                c.cycles += 1;
                c.g[V1][0] = c.lbu(nt_r(c, A0));
            }
            c.cycles += 2;
            let newline = c.g[V1][0] == c.g[T0][0];
            c.g[V0][0] = (c.g[A0][0] < c.g[A2][0]) as u64;
            if newline {
                c.cycles += 2;
                c.g[S2][0] = 0x20;
                return Ok(false);
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_1e90);
            }
            c.cycles += 2;
            c.g[V1][0] = u64::MAX;
            if c.g[V0][0] != 0 {
                c.cycles += 1;
                c.g[V1][0] = c.lbu(nt_r(c, A0));
            }
            c.cycles += 2;
            let end = c.g[V1][0] == c.g[A3][0];
            c.g[S2][0] = 0x20;
            if end {
                return Ok(false);
            }
            c.cycles += 6;
            c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
            c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
            c.g[V0][0] = sx(nt_r(c, V0).wrapping_add(nt_r(c, V1)));
            c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
            if more {
                c.cycles += 1;
                c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
            }
        }
    }
    nt_adv7(c);
    nt_directive_file(c)
}

/// The quoted file name (at most 127 characters kept) and the end of the
/// directive line.
pub(crate) fn nt_directive_file(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 4;
    c.g[S1][0] = sx(nt_f(c, NEXT_TOKEN_FNAME));
    c.g[T0][0] = 0;
    c.g[A1][0] = 0x0049_0000;
    'name: loop {
        c.cycles += 1;
        c.g[A0][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
        loop {
            c.cycles += 7;
            c.g[A3][0] = u64::MAX;
            c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
            c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
            c.g[V0][0] = sx(nt_r(c, V0).wrapping_add(nt_r(c, A0)));
            c.g[A2][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
            c.g[A0][0] = c.g[V1][0];
            if c.g[A2][0] != 0 {
                c.cycles += 1;
                c.g[A3][0] = c.lbu(nt_r(c, A0));
            }
            c.cycles += 2;
            let unterminated = if c.g[A3][0] == c.g[S3][0] {
                c.g[A0][0] = c.g[S0][0];
                true
            } else {
                c.cycles += 2;
                c.g[V1][0] = u64::MAX;
                if c.g[A2][0] != 0 {
                    c.cycles += 1;
                    c.g[V1][0] = c.lbu(nt_r(c, A0));
                }
                c.cycles += 3;
                c.g[V0][0] = u64::MAX;
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S0][0];
                    true
                } else {
                    false
                }
            };
            if unterminated {
                c.cycles += 9;
                c.g[A1][0] = sx(nt_r(c, A1).wrapping_sub(0x2218));
                NEXT_TOKEN_ERROR_FN.call(c, 0x0032_1f68)?;
                let more = nt_more(c);
                c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
                if more {
                    c.cycles += 2;
                    c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
                }
                break 'name;
            }
            c.cycles += 2;
            c.g[V1][0] = u64::MAX;
            if c.g[A2][0] != 0 {
                c.cycles += 1;
                c.g[V1][0] = c.lbu(nt_r(c, A0));
            }
            c.cycles += 3;
            c.g[V0][0] = 0x22;
            let close = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = ((c.g[T0][0] as i64) < 127) as u64;
            if close {
                c.cycles += 2;
                let more = c.g[A2][0] != 0;
                c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
                if more {
                    c.cycles += 2;
                    c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
                }
                break 'name;
            }
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
            } else {
                c.cycles += 2;
                c.g[V0][0] = u64::MAX;
                if c.g[A2][0] != 0 {
                    c.cycles += 1;
                    c.g[V0][0] = c.lbu(nt_r(c, A0));
                }
                c.cycles += 4;
                c.sb(nt_r(c, S1), c.g[V0][0]);
                c.g[T0][0] = sx(nt_r(c, T0).wrapping_add(1));
                c.g[S1][0] = sx(nt_r(c, S1).wrapping_add(1));
                c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
            }
            c.cycles += 6;
            if !nt_more_v0(c) {
                c.g[A0][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
                if event_due(c) {
                    deliver_event_here(c, 0x0032_1f1c);
                }
                continue;
            }
            c.cycles += 3;
            c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
            c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
            if event_due(c) {
                deliver_event_here(c, 0x0032_1f18);
            }
            continue 'name;
        }
    }
    // 0x321ffc: terminate the name; blanks; then the newline.
    c.cycles += 13;
    c.sb(nt_r(c, S1), 0);
    c.g[T5][0] = 0x20;
    c.g[T4][0] = 9;
    c.g[T1][0] = 0xa;
    c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
    c.g[T3][0] = 0xd;
    c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
    c.g[T2][0] = u64::MAX;
    c.g[A3][0] = c.g[V1][0];
    c.g[A1][0] = sx(nt_r(c, V1).wrapping_add(nt_r(c, V0)));
    c.g[A2][0] = c.g[V0][0];
    c.g[T0][0] = c.g[A1][0];
    nt_blank_run(c, 0x0032_2030, T0, false);
    c.cycles += 3;
    c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
    c.g[V1][0] = u64::MAX;
    if c.g[V0][0] != 0 {
        c.cycles += 1;
        c.g[V1][0] = c.lbu(nt_r(c, A0));
    }
    c.cycles += 2;
    let newline = c.g[V1][0] == c.g[S3][0];
    c.g[A1][0] = sx(nt_r(c, S5).wrapping_sub(0x2218));
    if newline {
        nt_adv7(c);
        c.cycles += 1;
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
    } else {
        // Junk after the name: report it and skip blanks.
        c.cycles += 12;
        c.g[A0][0] = c.g[S0][0];
        NEXT_TOKEN_ERROR_FN.call(c, 0x0032_20e8)?;
        c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
        c.g[T3][0] = 0x20;
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
        c.g[T2][0] = 9;
        c.g[A2][0] = 0xa;
        c.g[T1][0] = 0xd;
        c.g[A1][0] = sx(nt_r(c, V1).wrapping_add(nt_r(c, V0)));
        c.g[T0][0] = u64::MAX;
        c.g[A3][0] = c.g[A1][0];
        nt_blank_run(c, 0x0032_2110, A3, true);
    }
    // 0x3221cc: the current character.
    c.cycles += 6;
    let more = nt_more_v0(c);
    c.g[S2][0] = u64::MAX;
    if more {
        c.cycles += 2;
        c.g[S2][0] = c.lbu(nt_r(c, A0));
    }
    Ok(false)
}

/// `cmp`: the dispatch compare `s2 == ch` (2 cycles) with the delay slot's
/// register load.
#[inline(always)]
pub(crate) fn nt_is(c: &mut Cpu, ch: u64, delay: usize, value: u64) -> bool {
    c.cycles += 2;
    let hit = c.g[S2][0] == ch;
    c.g[delay][0] = value;
    hit
}

/// A one-character token: step, terminate the text, kind `kind`.
pub(crate) fn nt_single(c: &mut Cpu, kind: u64) -> Result<bool, Unwound> {
    nt_adv7(c);
    c.cycles += 3;
    c.sb(nt_r(c, S1), 0);
    c.g[V0][0] = kind;
    Ok(true)
}

/// `X` or `X=` (`!`, `=`, `<`, `>`; `a1` = '='), after stepping past `X`.
pub(crate) fn nt_pair_eq(c: &mut Cpu, first: u64, single: u64, pair: u64) -> Result<bool, Unwound> {
    nt_adv7(c);
    nt_peek_s2(c);
    c.cycles += first;
    if first == 3 {
        c.g[V0][0] = 0x3d;
    }
    let eq_reg = if first == 3 { V0 } else { A1 };
    if c.g[S2][0] != c.g[eq_reg][0] {
        c.sb(nt_r(c, S1), 0);
        c.cycles += 2;
        c.g[V0][0] = single;
        return Ok(true);
    }
    nt_push_adv(c, S2);
    c.cycles += 3;
    c.sb(nt_r(c, S1), 0);
    c.g[V0][0] = pair;
    Ok(true)
}

/// `++` / `-+`: "Operator '++' not defined.", kind 1.
pub(crate) fn nt_plus_plus(c: &mut Cpu) -> Result<bool, Unwound> {
    nt_push_adv(c, S2);
    c.cycles += 7;
    c.sb(nt_r(c, S1), 0);
    c.g[A1][0] = 0x0049_0000;
    c.g[A1][0] = sx(nt_r(c, A1).wrapping_sub(0x21f8));
    c.g[A0][0] = c.g[S0][0];
    NEXT_TOKEN_ERROR_FN.call(c, 0x0032_24b4)?;
    c.g[V0][0] = 1;
    Ok(true)
}

/// `&&` / `||`: `None` (with `v0` = `fail_v0`) when the next character is
/// not the same, and the dispatch goes on.
pub(crate) fn nt_double(c: &mut Cpu, fail_v0: u64, kind: u64) -> Option<()> {
    c.cycles += 8;
    c.g[A1][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
    c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
    c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
    c.g[A2][0] = sx(nt_r(c, A1).wrapping_add(1));
    c.g[V0][0] = sx(nt_r(c, V0).wrapping_add(nt_r(c, V1)));
    c.g[A0][0] = (c.g[A2][0] < c.g[V0][0]) as u64;
    c.g[A3][0] = u64::MAX;
    if c.g[A0][0] != 0 {
        c.cycles += 1;
        c.g[A3][0] = c.lbu(nt_r(c, A1).wrapping_add(1));
    }
    c.cycles += 2;
    if c.g[A3][0] != c.g[S2][0] {
        c.g[V0][0] = fail_v0;
        return None;
    }
    c.cycles += 3;
    c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
    if c.g[V0][0] != 0 {
        c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[A2][0]);
    }
    c.cycles += 7;
    let more = nt_more(c);
    c.g[A1][0] = u64::MAX;
    if more {
        c.cycles += 1;
        c.g[A1][0] = c.lbu(nt_r(c, A0));
    }
    nt_push_adv(c, A1);
    c.cycles += 3;
    c.sb(nt_r(c, S1), 0);
    c.g[V0][0] = kind;
    Some(())
}

/// Store `s2` in the token text, or (63 characters reached) report
/// `msg = s6 - msg_off`, set the kind to 1 (`s5`) and store it anyway.
pub(crate) fn nt_push_checked(c: &mut Cpu, msg_off: u32, ret_addr: u32) -> Result<(), Unwound> {
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.sb(nt_r(c, S1), c.g[S2][0]);
        return Ok(());
    }
    c.cycles += 6;
    c.sb(nt_r(c, S1), 0);
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = sx(nt_r(c, S6).wrapping_sub(msg_off));
    NEXT_TOKEN_ERROR_FN.call(c, ret_addr)?;
    c.sw(nt_f(c, NEXT_TOKEN_KIND), c.g[S5][0]);
    c.sb(nt_r(c, S1), c.g[S2][0]);
    Ok(())
}

/// The exponent's checked store (the length computed from `s1`).
pub(crate) fn nt_push_checked_exp(c: &mut Cpu, ret_addr: u32) -> Result<(), Unwound> {
    c.cycles += 5;
    c.g[V0][0] = sx(nt_r(c, S1).wrapping_sub(0x9c));
    c.g[V0][0] = sx(nt_r(c, V0).wrapping_sub(nt_r(c, S0)));
    c.g[V0][0] = ((c.g[V0][0] as i64) < 63) as u64;
    if c.g[V0][0] != 0 {
        c.sb(nt_r(c, S1), c.g[S2][0]);
        return Ok(());
    }
    c.cycles += 8;
    c.sb(nt_r(c, S1), 0);
    c.g[A1][0] = 0x0049_0000;
    c.g[A1][0] = sx(nt_r(c, A1).wrapping_sub(0x21a0));
    c.g[A0][0] = c.g[S0][0];
    NEXT_TOKEN_ERROR_FN.call(c, ret_addr)?;
    c.g[V0][0] = 1;
    c.sw(nt_f(c, NEXT_TOKEN_KIND), c.g[V0][0]);
    c.sb(nt_r(c, S1), c.g[S2][0]);
    Ok(())
}

/// Step after a stored character, `s1` moving on (the 7-cycle form).
pub(crate) fn nt_step_s1(c: &mut Cpu) {
    c.cycles += 7;
    let more = nt_more(c);
    c.g[S1][0] = sx(nt_r(c, S1).wrapping_add(1));
    if more {
        c.cycles += 2;
        c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
        c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
    }
}

/// Count, step and move `s1` after a stored character (8 cycles); the
/// position store costs `store_cycles` (3 on a loop back edge, else 2).
pub(crate) fn nt_count_step(c: &mut Cpu, store_cycles: u64) {
    c.cycles += 8;
    c.g[S3][0] = sx(nt_r(c, S3).wrapping_add(1));
    let more = nt_more(c);
    c.g[S1][0] = sx(nt_r(c, S1).wrapping_add(1));
    if more {
        c.cycles += store_cycles;
        c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
        c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
    }
}

/// Peek into `s2`, then: is it a digit? (`v0` = `s3 < 63` after.)
pub(crate) fn nt_peek_digit(c: &mut Cpu) -> bool {
    nt_peek_s2(c);
    c.cycles += 6;
    let ch = nt_sext8(c.g[S2][0]);
    let digit = nt_digit(c, ch);
    c.g[V0][0] = ((c.g[S3][0] as i64) < 63) as u64;
    digit
}

/// `isalpha` of `a0` (sign-extended): `a1` = result. The first compare is
/// the caller's.
pub(crate) fn nt_alpha_tail(c: &mut Cpu) {
    if c.g[V0][0] != 0 {
        c.cycles += 1;
        c.g[A1][0] = 1;
        return;
    }
    c.cycles += 4;
    c.g[V0][0] = sx(nt_r(c, A0).wrapping_sub(0x41));
    c.g[V0][0] = (c.g[V0][0] < 26) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 1;
        c.g[A1][0] = 1;
    }
}

/// The token itself, from the first non-blank character. True when the
/// kind in `v0` is still to be stored.
pub(crate) fn nt_token(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 8;
    c.g[S1][0] = c.g[S4][0];
    c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
    c.g[S2][0] = u64::MAX;
    c.g[A0][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
    c.g[V0][0] = sx(nt_r(c, V0).wrapping_add(nt_r(c, V1)));
    c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
    let more = c.g[V0][0] != 0;
    c.sw(nt_f(c, NEXT_TOKEN_LINE_START), 0);
    if more {
        c.cycles += 1;
        c.g[S2][0] = c.lbu(nt_r(c, A0));
    }
    c.cycles += 4;
    c.sb(nt_r(c, S1), c.g[S2][0]);
    c.g[V0][0] = u64::MAX;
    let end = c.g[S2][0] == c.g[V0][0];
    c.g[S1][0] = sx(nt_r(c, S1).wrapping_add(1));
    if end {
        nt_adv7(c);
        c.cycles += 3;
        c.sb(nt_r(c, S1), 0);
        c.sw(nt_f(c, NEXT_TOKEN_KIND), 0);
        return Ok(false);
    }

    // Operators.
    c.cycles += 1;
    if nt_is(c, 0x21, V0, 0x21) {
        c.g[A1][0] = 0x2b;
        return nt_pair_eq(c, 3, 0x18, 0x13);
    }
    c.g[A1][0] = 0x2b;
    if nt_is(c, 0x2b, V0, 0x2d) {
        nt_adv7(c);
        nt_peek_s2(c);
        c.cycles += 2;
        if c.g[S2][0] != c.g[A1][0] {
            c.sb(nt_r(c, S1), 0);
            c.cycles += 2;
            c.g[V0][0] = 0x14;
            return Ok(true);
        }
        return nt_plus_plus(c);
    }
    if nt_is(c, 0x2d, V0, 0x2e) {
        nt_adv7(c);
        nt_peek_s2(c);
        c.cycles += 2;
        let plus = c.g[S2][0] == c.g[A1][0];
        c.g[V0][0] = 0x15;
        if plus {
            if event_due(c) {
                deliver_event_here(c, 0x0032_2478);
            }
            return nt_plus_plus(c);
        }
        c.cycles += 2;
        c.sb(nt_r(c, S1), 0);
        return Ok(true);
    }
    if nt_is(c, 0x2e, V0, 0x2a) {
        return nt_single(c, 4);
    }
    if nt_is(c, 0x2a, V0, 0x2f) {
        return nt_single(c, 0x16);
    }
    if nt_is(c, 0x2f, A1, 0x3d) {
        return nt_single(c, 0x17);
    }
    if nt_is(c, 0x3d, V0, 0x3c) {
        return nt_pair_eq(c, 2, 3, 0x12);
    }
    if nt_is(c, 0x3c, V0, 0x3e) {
        return nt_pair_eq(c, 2, 0x10, 0x11);
    }
    if nt_is(c, 0x3e, V0, 0x26) {
        return nt_pair_eq(c, 2, 0xe, 0xf);
    }
    if nt_is(c, 0x26, V0, 0x7c) && nt_double(c, 0x7c, 0xd).is_some() {
        return Ok(true);
    }
    if nt_is(c, 0x7c, V0, 0x28) && nt_double(c, 0x28, 0xc).is_some() {
        return Ok(true);
    }
    if nt_is(c, 0x28, V0, 0x29) {
        return nt_single(c, 9);
    }
    if nt_is(c, 0x29, V0, 0x2c) {
        return nt_single(c, 0xa);
    }
    if nt_is(c, 0x2c, V0, 0x3f) {
        return nt_single(c, 8);
    }
    if nt_is(c, 0x3f, V0, 0x3b) {
        return nt_single(c, 6);
    }
    if nt_is(c, 0x3b, V0, 0x3a) {
        return nt_single(c, 5);
    }
    let shifted = sx(nt_r(c, S2) << 24);
    if nt_is(c, 0x3a, V1, shifted) {
        return nt_single(c, 7);
    }

    // Identifier or keyword?
    c.cycles += 5;
    c.g[A0][0] = sx((nt_r(c, V1) as i32 >> 24) as u32);
    c.g[V0][0] = sx(nt_r(c, A0).wrapping_sub(0x61));
    c.g[V0][0] = (c.g[V0][0] < 26) as u64;
    c.g[A1][0] = 0;
    nt_alpha_tail(c);
    c.cycles += 2;
    c.g[V0][0] = 0x2b;
    if c.g[A1][0] != 0 {
        return nt_identifier(c);
    }

    // Number?
    c.cycles += 2;
    let number = if c.g[S2][0] == c.g[V0][0] {
        c.g[V0][0] = 0x2d;
        true
    } else {
        c.g[V0][0] = 0x2d;
        c.cycles += 2;
        let minus = c.g[S2][0] == c.g[V0][0];
        c.g[V0][0] = sx((nt_r(c, V1) as i32 >> 24) as u32);
        if minus {
            true
        } else {
            c.cycles += 4;
            c.g[V0][0] = sx(nt_r(c, V0).wrapping_sub(0x30));
            c.g[V0][0] = (c.g[V0][0] < 10) as u64;
            c.g[A1][0] = 0x0049_0000;
            c.g[V0][0] != 0
        }
    };
    if number {
        return nt_number(c);
    }

    // "Unexpected character in input '%c'", kind 1.
    c.cycles += 12;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A1][0] = sx(nt_r(c, A1).wrapping_sub(0x2158));
    NEXT_TOKEN_ERROR_FN.call(c, 0x0032_3040)?;
    c.sb(nt_r(c, S1), 0);
    let more = nt_more(c);
    c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
    if more {
        c.cycles += 1;
        c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
    }
    c.cycles += 1;
    c.g[V0][0] = 1;
    Ok(true)
}

/// Letters, digits and `_` (63 kept, "Identifer exceeds 64 characters."
/// beyond), then the keyword lookup (`strcasecmp`).
pub(crate) fn nt_identifier(c: &mut Cpu) -> Result<bool, Unwound> {
    nt_adv7(c);
    c.cycles += 5;
    c.g[V0][0] = sx(nt_r(c, S1).wrapping_sub(0x9c));
    c.g[S7][0] = 0x5f;
    c.g[S3][0] = sx(nt_r(c, V0).wrapping_sub(nt_r(c, S0)));
    c.g[S6][0] = 0x0049_0000;
    c.g[S5][0] = 1;
    loop {
        nt_peek_s2(c);
        c.cycles += 6;
        c.g[V1][0] = sx(nt_r(c, S2) << 24);
        c.g[A0][0] = sx((nt_r(c, V1) as i32 >> 24) as u32);
        c.g[V0][0] = sx(nt_r(c, A0).wrapping_sub(0x61));
        c.g[V0][0] = (c.g[V0][0] < 26) as u64;
        c.g[A1][0] = 0;
        nt_alpha_tail(c);
        c.cycles += 2;
        c.g[V0][0] = ((c.g[S3][0] as i64) < 63) as u64;
        let take = if c.g[A1][0] != 0 {
            true
        } else {
            c.cycles += 5;
            let ch = sx((nt_r(c, V1) as i32 >> 24) as u32);
            let digit = nt_digit(c, ch);
            c.g[V0][0] = ((c.g[S3][0] as i64) < 63) as u64;
            if digit {
                true
            } else {
                c.cycles += 2;
                c.g[A1][0] = 0x004a_0000;
                c.g[S2][0] == c.g[S7][0]
            }
        };
        if !take {
            break;
        }
        nt_push_checked(c, 0x21d8, 0x0032_2aa8)?;
        nt_count_step(c, 3);
        if event_due(c) {
            deliver_event_here(c, 0x0032_2a20);
        }
    }
    // Keywords: max min pow atan2 atan sin cos exp log floor ceil abs sqrt
    // (0x4a3f30 + 8 * i, kinds 0x19..0x25), then "minradian" (0x26).
    c.cycles += 6;
    c.sb(nt_r(c, S1), 0);
    c.g[A1][0] = sx(nt_r(c, A1).wrapping_add(0x3f30));
    c.g[A0][0] = c.g[S4][0];
    call_ptr(c, NEXT_TOKEN_STRCASECMP, 0x0032_2aec)?;
    let differs = c.g[V0][0] != 0;
    c.g[A1][0] = 0x004a_0000;
    if !differs {
        c.cycles += 2;
        c.g[V0][0] = 0x19;
        return Ok(true);
    }
    for i in 1..13u32 {
        c.cycles += 5;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = sx(nt_r(c, A1).wrapping_add(0x3f30 + 8 * i));
        call_ptr(c, NEXT_TOKEN_STRCASECMP, 0x0032_2aec + 0x1c * i)?;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0x19 + i as u64;
            return Ok(true);
        }
        c.g[A1][0] = if i == 12 { 0x0049_0000 } else { 0x004a_0000 };
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = sx(nt_r(c, A1).wrapping_sub(0x21b0));
    call_ptr(c, NEXT_TOKEN_STRCASECMP, 0x0032_2c58)?;
    let differs = c.g[V0][0] != 0;
    c.g[V0][0] = 2;
    if !differs {
        c.cycles += 2;
        c.g[V0][0] = 0x26;
    }
    Ok(true)
}

/// `[sign] digits [. digits] [(E|e) [sign] digits] [f]` (63 characters kept,
/// "Constant exceeds 64 characters." beyond), parsed with `sscanf("%f")`
/// into the value; kind 0xb.
pub(crate) fn nt_number(c: &mut Cpu) -> Result<bool, Unwound> {
    nt_adv7(c);
    c.cycles += 6;
    c.g[V0][0] = sx(nt_r(c, S1).wrapping_sub(0x9c));
    c.g[S7][0] = sx(nt_f(c, NEXT_TOKEN_VALUE));
    c.g[S3][0] = sx(nt_r(c, V0).wrapping_sub(nt_r(c, S0)));
    c.g[S6][0] = 0x0049_0000;
    c.g[S5][0] = 1;
    while nt_peek_digit(c) {
        nt_push_checked(c, 0x21a0, 0x0032_2d10)?;
        nt_count_step(c, 3);
        if event_due(c) {
            deliver_event_here(c, 0x0032_2cc0);
        }
    }
    // '.', 'E' or 'e'?
    c.cycles += 3;
    c.g[V0][0] = 0x2e;
    let dot = c.g[S2][0] == c.g[V0][0];
    c.g[V0][0] = 0x45;
    let mut check_e = false;
    let mut exponent = false;
    if dot {
        // Fraction.
        c.cycles += 6;
        c.g[V0][0] = sx(nt_r(c, S1).wrapping_sub(0x9c));
        c.g[S6][0] = 0x0049_0000;
        c.g[S3][0] = sx(nt_r(c, V0).wrapping_sub(nt_r(c, S0)));
        c.g[S5][0] = 1;
        c.g[V0][0] = ((c.g[S3][0] as i64) < 63) as u64;
        loop {
            nt_push_checked(c, 0x21a0, 0x0032_2d98)?;
            nt_count_step(c, 2);
            if !nt_peek_digit(c) {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_2d80);
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 0x45;
        check_e = true;
    } else {
        c.cycles += 2;
        let upper = c.g[S2][0] == c.g[V0][0];
        c.g[V0][0] = 0x65;
        if upper {
            exponent = true;
        } else {
            c.cycles += 2;
            let lower = c.g[S2][0] == c.g[V0][0];
            c.g[V0][0] = 0x66;
            if lower {
                c.cycles += 2;
                c.g[V0][0] = 0x45;
                check_e = true;
            }
        }
    }
    if check_e {
        c.cycles += 2;
        let upper = c.g[S2][0] == c.g[V0][0];
        c.g[V0][0] = 0x65;
        if upper {
            exponent = true;
        } else {
            c.cycles += 2;
            exponent = c.g[S2][0] == c.g[V0][0];
            c.g[V0][0] = 0x66;
        }
    }
    if exponent {
        nt_push_checked_exp(c, 0x0032_2e3c)?;
        nt_step_s1(c);
        nt_peek_s2(c);
        c.cycles += 3;
        c.g[V0][0] = 0x2d;
        let minus = c.g[S2][0] == c.g[V0][0];
        c.g[V0][0] = 0x2b;
        let sign = if minus {
            true
        } else {
            c.cycles += 2;
            let plus = c.g[S2][0] == c.g[V0][0];
            c.g[V0][0] = sx(nt_r(c, S2) << 24);
            plus
        };
        if sign {
            nt_push_checked_exp(c, 0x0032_2ec8)?;
            nt_step_s1(c);
            nt_peek_s2(c);
            c.cycles += 1;
            c.g[V0][0] = sx(nt_r(c, S2) << 24);
        }
        c.cycles += 5;
        let ch = sx((nt_r(c, V0) as i32 >> 24) as u32);
        let digit = nt_digit(c, ch);
        c.g[V0][0] = sx(nt_r(c, S1).wrapping_sub(0x9c));
        if !digit {
            // "Malformed floating point constant.", kind 1.
            c.cycles += 8;
            c.sb(nt_r(c, S1), 0);
            c.g[A1][0] = 0x0049_0000;
            c.g[A1][0] = sx(nt_r(c, A1).wrapping_sub(0x2180));
            c.g[A0][0] = c.g[S0][0];
            NEXT_TOKEN_ERROR_FN.call(c, 0x0032_2f44)?;
            c.g[V0][0] = 1;
            c.sw(nt_f(c, NEXT_TOKEN_KIND), c.g[V0][0]);
            c.g[V0][0] = sx(nt_r(c, S1).wrapping_sub(0x9c));
        }
        c.cycles += 4;
        c.g[S6][0] = 0x0049_0000;
        c.g[S3][0] = sx(nt_r(c, V0).wrapping_sub(nt_r(c, S0)));
        c.g[S5][0] = 1;
        c.g[V0][0] = ((c.g[S3][0] as i64) < 63) as u64;
        loop {
            nt_push_checked(c, 0x21a0, 0x0032_2f78)?;
            nt_count_step(c, 2);
            if !nt_peek_digit(c) {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_2f60);
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 0x66;
    }
    // Optional 'f' suffix, then sscanf(text, "%f", &value).
    c.cycles += 2;
    if c.g[S2][0] != c.g[V0][0] {
        c.sb(nt_r(c, S1), 0);
    } else {
        nt_adv7(c);
        c.cycles += 1;
        c.sb(nt_r(c, S1), 0);
    }
    c.cycles += 8;
    c.g[A1][0] = 0x004a_0000;
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = sx(nt_r(c, A1).wrapping_add(0x3f98));
    c.g[A2][0] = c.g[S7][0];
    call_ptr(c, NEXT_TOKEN_SSCANF, 0x0032_3024)?;
    c.g[V1][0] = 0xb;
    c.sw(nt_f(c, NEXT_TOKEN_KIND), c.g[V1][0]);
    Ok(false)
}

// ---------------------------------------------------------------------------
// `InputParser_EmitNode` (cInputMapParser::compileParseNode).

/// "opcode buffer overflow".
pub(crate) const EMIT_NODE_MSG_OVERFLOW: u32 = 0x0048_e228;

/// "failed to allocate temporary register".
pub(crate) const EMIT_NODE_MSG_NO_REGISTER: u32 = 0x0048_e240;

/// "branch offset overflow".
pub(crate) const EMIT_NODE_MSG_BRANCH: u32 = 0x0048_e268;

/// The node-type jump table in `.rodata` (entry `type - 1`).
pub(crate) const EMIT_NODE_JUMP_TABLE: u32 = 0x0048_e280;

/// The jump table's targets, by node type 1..=33 (the original leaves the
/// chosen one in `a0`).
pub(crate) const EMIT_NODE_CASES: [u32; 33] = [
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
pub(crate) const EMIT_NODE_DEFAULT_MODIFIER: u64 = 5;

/// Low 6 bits of an instruction word: the opcode.
pub(crate) const EMIT_NODE_OP_MASK: u64 = 0xffff_ffff_ffff_ffc0;

pub(crate) const EMIT_NODE_ERROR: Callee = Callee {
    addr: 0x0032_1500,
    f: ee::s10::InputParser_Error,
};

pub(crate) const EMIT_NODE_RECURSE: Callee = Callee {
    addr: 0x0032_4678,
    f: ee::s10::InputParser_EmitNode,
};

pub(crate) const EMIT_NODE_PAD_OPERAND: Callee = Callee {
    addr: 0x0032_1ad8,
    f: ee::s10::InputParser_PadChannelOperand,
};

pub(crate) const EMIT_NODE_BUILTIN_OPERAND: Callee = Callee {
    addr: 0x0032_1ad0,
    f: ee::s10::InputParser_BuiltinOperand,
};

pub(crate) const EMIT_NODE_BINARY: Callee = Callee {
    addr: 0x0032_4ec8,
    f: ee::s10::InputParser_EmitBinaryOp,
};

pub(crate) const EMIT_NODE_UNARY: Callee = Callee {
    addr: 0x0032_50d8,
    f: ee::s10::InputParser_EmitUnaryOp,
};

/// How a node case leaves: through `v0 = -1` (0x324e94), straight to the
/// epilogue (0x324e98), or with `s0` already restored (0x324e9c).
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum EmitNodeExit {
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
pub(crate) fn emit_node_report(c: &mut Cpu, msg: u32, ret: u32) -> Result<EmitNodeExit, Unwound> {
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = sx(msg);
    EMIT_NODE_ERROR.call(c, ret)?;
    c.g[V0][0] = u64::MAX;
    Ok(EmitNodeExit::Done)
}

/// The "opcode buffer overflow" block (0x324748) reached from a later
/// block: a back edge with an event check.
pub(crate) fn emit_node_overflow_back(c: &mut Cpu) -> Result<EmitNodeExit, Unwound> {
    if event_due(c) {
        deliver_event_here(c, 0x0032_4748);
    }
    c.cycles += 6;
    emit_node_report(c, EMIT_NODE_MSG_OVERFLOW, 0x0032_4758)
}

/// `(cap as i64) <= 0` on the 64-bit `s4`.
pub(crate) fn emit_node_full(c: &Cpu) -> bool {
    (c.g[S4][0] as i64) <= 0
}

/// The node-type switch (0x3246bc), looping on variable nodes.
pub(crate) fn emit_node_cases(c: &mut Cpu) -> Result<EmitNodeExit, Unwound> {
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
pub(crate) fn emit_node_operator(
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
pub(crate) fn emit_node_constant(c: &mut Cpu) -> Result<EmitNodeExit, Unwound> {
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
pub(crate) fn emit_node_pad_channel(c: &mut Cpu) -> Result<EmitNodeExit, Unwound> {
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
pub(crate) fn emit_node_builtin(c: &mut Cpu) -> Result<EmitNodeExit, Unwound> {
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
pub(crate) fn emit_node_conditional(c: &mut Cpu) -> Result<EmitNodeExit, Unwound> {
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

// ---------------------------------------------------------------------------
// InputParser_ParseUnary (`cInputMapParser_parseTerm`)

/// Parser fields the term parser reads: the current token kind, its text
/// and numeric value, the expression nodes' free list, and the identifier
/// being defined (a reference to it is recursive).
pub(crate) const PARSE_UNARY_TOKEN: u32 = 0x98;

pub(crate) const PARSE_UNARY_TEXT: u32 = 0x9c;

pub(crate) const PARSE_UNARY_VALUE: u32 = 0xdc;

#[cfg(test)]
pub(crate) const PARSE_UNARY_FREE_NODES: u32 = 0xe0;

#[cfg(test)]
pub(crate) const PARSE_UNARY_IDENT_COUNT: u32 = 0x80e4;

#[cfg(test)]
pub(crate) const PARSE_UNARY_IDENTS: u32 = 0x80e8;

/// `self + 0x10000 + 0x8e8`: the identifier whose definition is parsed.
pub(crate) const PARSE_UNARY_CURRENT_IDENT: u32 = 0x8e8;

/// An identifier entry's expression pointer (entries are 0x44 bytes, the
/// name first).
pub(crate) const PARSE_UNARY_IDENT_EXPR: u32 = 0x40;

/// Expression node kinds the term parser builds (`node[0]`; `node[1]` is
/// the operand or value, `node[2]` the modifier).
pub(crate) const PU_NODE_IDENT: u64 = 1;

pub(crate) const PU_NODE_BUTTON: u64 = 2;

pub(crate) const PU_NODE_CONFIG: u64 = 3;

pub(crate) const PU_NODE_CONST: u64 = 4;

pub(crate) const PU_NODE_NEG: u64 = 0x12;

pub(crate) const PU_NODE_NOT: u64 = 0x13;

/// Token kinds: an identifier, `.`, `)`.
pub(crate) const PU_TOK_NAME: u64 = 2;

pub(crate) const PU_TOK_DOT: u64 = 4;

pub(crate) const PU_TOK_RPAREN: u64 = 0xa;

/// "No modifier" and "unknown modifier" from LookupModifier.
pub(crate) const PU_MOD_NONE: u64 = 5;

pub(crate) const PU_MOD_BAD: u64 = 6;

/// The token switch's jump table (`0x490000 - 0x1f00`) and its targets.
pub(crate) const PU_JUMP_TABLE: u32 = 0x0048_e100;

pub(crate) const PU_CASE_NAME: u32 = 0x0032_3c24;

pub(crate) const PU_CASE_NAME_DOT: u32 = 0x0032_3d64;

pub(crate) const PU_CASE_NUMBER: u32 = 0x0032_3e28;

pub(crate) const PU_CASE_PLUS: u32 = 0x0032_3e4c;

pub(crate) const PU_CASE_MINUS: u32 = 0x0032_3e5c;

pub(crate) const PU_CASE_NOT: u32 = 0x0032_3eac;

pub(crate) const PU_CASE_PAREN: u32 = 0x0032_3f20;

/// `max min pow atan2` (ParseFunction2) then the ten one-argument
/// functions (ParseFunction1), 0x1c bytes apart, node kinds 0x14..0x21.
pub(crate) const PU_CASE_FUNC_FIRST: u32 = 0x0032_3f78;

pub(crate) const PU_CASE_FUNC_LAST: u32 = 0x0032_40e4;

pub(crate) const PU_CASE_DEFAULT: u32 = 0x0032_4100;

/// Error messages.
pub(crate) const PU_MSG_EXPECTED_MODIFIER: u64 = 0x0048_e008;

pub(crate) const PU_MSG_BAD_BUTTON_MODIFIER: u64 = 0x0048_e028;

pub(crate) const PU_MSG_RECURSIVE: u64 = 0x0048_e048;

pub(crate) const PU_MSG_BAD_REF_MODIFIER: u64 = 0x0048_e070;

pub(crate) const PU_MSG_UNEXPECTED_IDENT: u64 = 0x0048_e098;

pub(crate) const PU_MSG_EXPECTED_RPAREN: u64 = 0x0048_e0b8;

pub(crate) const PU_MSG_EXPECTED_TERM: u64 = 0x0048_e0d8;

pub(crate) const PU_NEXT_TOKEN: Callee = Callee {
    addr: 0x0032_1af0,
    f: ee::s10::InputParser_NextToken,
};

pub(crate) const PU_ALLOC_NODE: Callee = Callee {
    addr: 0x0032_15f8,
    f: ee::s10::InputParser_AllocNode,
};

pub(crate) const PU_ERROR: Callee = Callee {
    addr: 0x0032_1500,
    f: ee::s10::InputParser_Error,
};

pub(crate) const PU_LOOKUP_BUTTON: Callee = Callee {
    addr: 0x0032_16e8,
    f: ee::s10::InputParser_LookupButton,
};

pub(crate) const PU_LOOKUP_MODIFIER: Callee = Callee {
    addr: 0x0032_17d0,
    f: ee::s10::InputParser_LookupModifier,
};

pub(crate) const PU_LOOKUP_BUILTIN: Callee = Callee {
    addr: 0x0032_1880,
    f: ee::s10::InputParser_LookupBuiltinName,
};

pub(crate) const PU_FIND_IDENTIFIER: Callee = Callee {
    addr: 0x0032_1a40,
    f: ee::s10::InputParser_FindIdentifier,
};

/// Recursion goes through the generated alias (and so the override slot).
pub(crate) const PU_PARSE_UNARY: Callee = Callee {
    addr: 0x0032_3bd8,
    f: ee::s10::InputParser_ParseUnary,
};

pub(crate) const PU_PARSE_CONDITIONAL: Callee = Callee {
    addr: 0x0032_3268,
    f: ee::s10::InputParser_ParseConditional,
};

pub(crate) const PU_FREE_NODE: Callee = Callee {
    addr: 0x0032_1638,
    f: ee::s10::InputParser_FreeNode,
};

pub(crate) const PU_PARSE_FUNCTION2: Callee = Callee {
    addr: 0x0032_4138,
    f: ee::s10::InputParser_ParseFunction2,
};

pub(crate) const PU_PARSE_FUNCTION1: Callee = Callee {
    addr: 0x0032_4398,
    f: ee::s10::InputParser_ParseFunction1,
};

/// How a term ends: the shared tails of the original.
pub(crate) enum PuExit {
    /// `a0`/`a1` (and `a2`) hold an error call's arguments (0x32410c):
    /// report it, return 0.
    Error,
    /// Return 0 (0x324114).
    Zero,
    /// `v0` holds the result; restore `s0` (0x324118).
    Done,
    /// `v0` holds the result and `s0` is restored (0x32411c).
    Restored,
}

#[inline(always)]
pub(crate) fn pu_r(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// The current token kind (`s1` is the parser).
#[inline(always)]
pub(crate) fn pu_token(c: &mut Cpu) -> u64 {
    let a = pu_r(c, S1).wrapping_add(PARSE_UNARY_TOKEN);
    c.lw(a)
}

/// `a0 = self; NextToken()` returning to `ret`.
#[inline(always)]
pub(crate) fn pu_next_token(c: &mut Cpu, ret: u32) -> Result<(), Unwound> {
    c.g[A0][0] = c.g[S1][0];
    PU_NEXT_TOKEN.call(c, ret)
}

/// `a0 = self; AllocNode()` returning to `ret`; the node is left in `s0`.
#[inline(always)]
pub(crate) fn pu_alloc_node(c: &mut Cpu, ret: u32) -> Result<(), Unwound> {
    c.g[A0][0] = c.g[S1][0];
    PU_ALLOC_NODE.call(c, ret)?;
    c.g[S0][0] = c.g[V0][0];
    Ok(())
}

/// Error arguments `a0 = self, a1 = msg` (0x32410c follows).
#[inline(always)]
pub(crate) fn pu_error_args(c: &mut Cpu, msg: u64) -> PuExit {
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = msg;
    PuExit::Error
}

/// The node tail at 0x323f10: `s0[1] = s2; s0[0] = v0; return s0`.
pub(crate) fn pu_store_node(c: &mut Cpu) -> PuExit {
    c.cycles += 4;
    let node = pu_r(c, S0);
    c.sw(node.wrapping_add(4), c.g[S2][0]);
    c.sw(node, c.g[V0][0]);
    c.g[V0][0] = c.g[S0][0];
    PuExit::Done
}

/// `name` is a button (index in `s3`, name text in `s0`): an optional
/// `.modifier`, then a button node `{2, index, modifier}`.
pub(crate) fn pu_button(c: &mut Cpu) -> Result<PuExit, Unwound> {
    c.cycles += 6;
    c.g[S2][0] = PU_MOD_NONE;
    pu_next_token(c, 0x0032_3c48)?;
    c.g[V1][0] = pu_token(c);
    c.g[V0][0] = PU_TOK_DOT;
    if c.g[V1][0] == c.g[V0][0] {
        c.cycles += 6;
        pu_next_token(c, 0x0032_3c60)?;
        c.g[V1][0] = pu_token(c);
        c.g[V0][0] = PU_TOK_NAME;
        c.g[A1][0] = 0x0049_0000;
        if c.g[V1][0] != c.g[V0][0] {
            c.cycles += 3;
            return Ok(pu_error_args(c, PU_MSG_EXPECTED_MODIFIER));
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        PU_LOOKUP_MODIFIER.call(c, 0x0032_3c88)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V0][0] = PU_MOD_BAD;
        c.g[A1][0] = 0x0049_0000;
        if c.g[S2][0] == c.g[V0][0] {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = PU_MSG_BAD_BUTTON_MODIFIER;
            c.g[A2][0] = c.g[S0][0];
            PU_ERROR.call(c, 0x0032_3ca8)?;
            c.g[V0][0] = 0;
            return Ok(PuExit::Done);
        }
        c.cycles += 2;
        pu_next_token(c, 0x0032_3cb8)?;
    }
    c.cycles += 9;
    pu_alloc_node(c, 0x0032_3cc0)?;
    let node = pu_r(c, S0);
    c.g[V0][0] = PU_NODE_BUTTON;
    c.sw(node.wrapping_add(4), c.g[S3][0]);
    c.sw(node, c.g[V0][0]);
    c.g[V0][0] = c.g[S0][0];
    c.sw(node.wrapping_add(8), c.g[S2][0]);
    Ok(PuExit::Done)
}

/// After an identifier reference's `.` (0x323d64; `v1` = the next token):
/// expect a modifier name, then build the reference.
pub(crate) fn pu_ident_dot(c: &mut Cpu) -> Result<PuExit, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = PU_TOK_NAME;
    c.g[A1][0] = 0x0049_0000;
    if c.g[V1][0] != c.g[V0][0] {
        c.cycles += 3;
        return Ok(pu_error_args(c, PU_MSG_EXPECTED_MODIFIER));
    }
    c.cycles += 7;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S0][0];
    PU_LOOKUP_MODIFIER.call(c, 0x0032_3d88)?;
    c.g[S4][0] = c.g[V0][0];
    c.g[V0][0] = PU_MOD_BAD;
    c.g[A1][0] = 0x0049_0000;
    if c.g[S4][0] == c.g[V0][0] {
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = PU_MSG_BAD_REF_MODIFIER;
        c.g[A2][0] = c.g[S0][0];
        PU_ERROR.call(c, 0x0032_3da8)?;
        c.g[V0][0] = 0;
        return Ok(PuExit::Done);
    }
    c.cycles += 2;
    pu_next_token(c, 0x0032_3db8)?;
    pu_ident_node(c)
}

/// The reference node (0x323db8; identifier entry in `s3`, modifier in
/// `s4`): a constant definition is copied (`{4, value}`), anything else
/// becomes `{1, entry, modifier}`.
pub(crate) fn pu_ident_node(c: &mut Cpu) -> Result<PuExit, Unwound> {
    c.cycles += 5;
    let expr_at = pu_r(c, S3).wrapping_add(PARSE_UNARY_IDENT_EXPR);
    c.g[V0][0] = c.lw(expr_at);
    c.g[V1][0] = PU_NODE_CONST;
    c.g[S2][0] = c.lw(pu_r(c, V0));
    if c.g[S2][0] == c.g[V1][0] {
        c.cycles += 8;
        pu_alloc_node(c, 0x0032_3dd4)?;
        let node = pu_r(c, S0);
        c.sw(node, c.g[S2][0]);
        let expr_at = pu_r(c, S3).wrapping_add(PARSE_UNARY_IDENT_EXPR);
        c.g[V1][0] = c.lw(expr_at);
        c.lwc1(0, pu_r(c, V1).wrapping_add(4));
        c.swc1(0, node.wrapping_add(4));
        return Ok(PuExit::Done);
    }
    c.cycles += 9;
    pu_alloc_node(c, 0x0032_3df4)?;
    let node = pu_r(c, S0);
    c.g[V0][0] = PU_NODE_IDENT;
    c.sw(node.wrapping_add(4), c.g[S3][0]);
    c.sw(node, c.g[V0][0]);
    c.g[V0][0] = c.g[S0][0];
    c.sw(node.wrapping_add(8), c.g[S4][0]);
    Ok(PuExit::Done)
}

/// An identifier token (0x323c24): a button, a built-in config value, or a
/// defined identifier; anything else is an error.
pub(crate) fn pu_name(c: &mut Cpu) -> Result<PuExit, Unwound> {
    c.cycles += 7;
    c.g[S0][0] = sx(pu_r(c, S1).wrapping_add(PARSE_UNARY_TEXT));
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S0][0];
    PU_LOOKUP_BUTTON.call(c, 0x0032_3c34)?;
    c.g[S3][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    if (c.g[S3][0] as i64) >= 0 {
        return pu_button(c);
    }
    c.cycles += 5;
    c.g[A1][0] = c.g[S0][0];
    PU_LOOKUP_BUILTIN.call(c, 0x0032_3ce4)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    if (c.g[S2][0] as i64) >= 0 {
        c.cycles += 7;
        pu_next_token(c, 0x0032_3cf8)?;
        pu_alloc_node(c, 0x0032_3d00)?;
        c.g[V0][0] = PU_NODE_CONFIG;
        return Ok(pu_store_node(c));
    }
    c.cycles += 5;
    c.g[A1][0] = c.g[S0][0];
    PU_FIND_IDENTIFIER.call(c, 0x0032_3d14)?;
    c.g[S3][0] = c.g[V0][0];
    c.g[A1][0] = 0x0049_0000;
    if c.g[S3][0] == 0 {
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = PU_MSG_UNEXPECTED_IDENT;
        c.g[A2][0] = c.g[S0][0];
        PU_ERROR.call(c, 0x0032_3e20)?;
        c.g[V0][0] = 0;
        return Ok(PuExit::Done);
    }
    c.cycles += 7;
    pu_next_token(c, 0x0032_3d28)?;
    c.g[V0][0] = sx(pu_r(c, S1).wrapping_add(0x10000));
    let current = pu_r(c, V0).wrapping_add(PARSE_UNARY_CURRENT_IDENT);
    c.g[V1][0] = c.lw(current);
    if c.g[S3][0] == c.g[V1][0] {
        c.cycles += 4;
        return Ok(pu_error_args(c, PU_MSG_RECURSIVE));
    }
    c.g[V1][0] = pu_token(c);
    c.cycles += 3;
    c.g[V0][0] = PU_TOK_DOT;
    c.g[S4][0] = PU_MOD_NONE;
    if c.g[V1][0] != c.g[V0][0] {
        return pu_ident_node(c);
    }
    c.cycles += 3;
    pu_next_token(c, 0x0032_3d60)?;
    c.g[V1][0] = pu_token(c);
    pu_ident_dot(c)
}

/// `-term` / `!term` (0x323e5c / 0x323eac): parse the operand; a constant
/// is folded (negated, or `x == 0 ? 1 : 0`), anything else wrapped in a
/// `{kind, operand}` node.
pub(crate) fn pu_prefix(c: &mut Cpu, not: bool) -> Result<PuExit, Unwound> {
    let (next_ret, term_ret) = if not {
        (0x0032_3eb4, 0x0032_3ebc)
    } else {
        (0x0032_3e64, 0x0032_3e6c)
    };
    c.cycles += 7;
    pu_next_token(c, next_ret)?;
    c.g[A0][0] = c.g[S1][0];
    PU_PARSE_UNARY.call(c, term_ret)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[V0][0] = PU_NODE_CONST;
    if c.g[S2][0] == 0 {
        return Ok(PuExit::Zero);
    }
    c.cycles += 3;
    let term = pu_r(c, S2);
    c.g[V1][0] = c.lw(term);
    if c.g[V1][0] == c.g[V0][0] {
        if not {
            c.cycles += 6;
            c.mtc1(0, 0);
            c.lwc1(1, term.wrapping_add(4));
            c.c_eq_s(1, 0);
            let zero = c.fcc();
            c.mov_s(2, 0);
            if zero {
                c.cycles += 2;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 2);
            }
            c.cycles += 3;
            c.swc1(2, term.wrapping_add(4));
        } else {
            c.cycles += 5;
            c.lwc1(0, term.wrapping_add(4));
            c.neg_s(0, 0);
            c.swc1(0, term.wrapping_add(4));
        }
        c.g[V0][0] = c.g[S2][0];
        return Ok(PuExit::Done);
    }
    if not {
        c.cycles += 4;
        pu_alloc_node(c, 0x0032_3f08)?;
        c.g[V0][0] = PU_NODE_NOT;
    } else {
        c.cycles += 5;
        pu_alloc_node(c, 0x0032_3ea0)?;
        c.g[V0][0] = PU_NODE_NEG;
    }
    Ok(pu_store_node(c))
}

/// One switch case (the jump-table target `case`); `None` loops back to the
/// switch (a unary `+`), `Err` is an unwind.
pub(crate) fn pu_case(c: &mut Cpu, case: u32) -> Result<Option<PuExit>, Unwound> {
    let exit = match case {
        PU_CASE_NAME => pu_name(c)?,
        PU_CASE_NAME_DOT => pu_ident_dot(c)?,
        PU_CASE_NUMBER => {
            // A number: `{4, value}`, then the next token.
            c.cycles += 9;
            pu_alloc_node(c, 0x0032_3e30)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[V0][0] = PU_NODE_CONST;
            let node = pu_r(c, S0);
            c.sw(node, c.g[V0][0]);
            c.lwc1(0, pu_r(c, S1).wrapping_add(PARSE_UNARY_VALUE));
            c.swc1(0, node.wrapping_add(4));
            pu_token_and_return_s0(c, 0x0032_3f70)?
        }
        PU_CASE_PLUS => {
            c.cycles += 4;
            pu_next_token(c, 0x0032_3e54)?;
            c.g[V0][0] = pu_token(c);
            return Ok(None);
        }
        PU_CASE_MINUS => pu_prefix(c, false)?,
        PU_CASE_NOT => pu_prefix(c, true)?,
        PU_CASE_PAREN => {
            // `( expression )`.
            c.cycles += 9;
            pu_next_token(c, 0x0032_3f28)?;
            c.g[A0][0] = c.g[S1][0];
            PU_PARSE_CONDITIONAL.call(c, 0x0032_3f30)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V1][0] = pu_token(c);
            c.g[V0][0] = PU_TOK_RPAREN;
            c.g[A1][0] = 0x0049_0000;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 1;
                c.g[A0][0] = c.g[S1][0];
                pu_token_and_return_s0(c, 0x0032_3f70)?
            } else {
                c.cycles += 8;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = PU_MSG_EXPECTED_RPAREN;
                PU_ERROR.call(c, 0x0032_3f50)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                PU_FREE_NODE.call(c, 0x0032_3f5c)?;
                c.g[V0][0] = 0;
                PuExit::Done
            }
        }
        PU_CASE_FUNC_FIRST..=PU_CASE_FUNC_LAST if (case - PU_CASE_FUNC_FIRST) % 0x1c == 0 => {
            // A function call: the node kind follows the keyword.
            let index = (case - PU_CASE_FUNC_FIRST) / 0x1c;
            let parse = if index < 4 {
                PU_PARSE_FUNCTION2
            } else {
                PU_PARSE_FUNCTION1
            };
            c.cycles += 7;
            pu_next_token(c, case + 8)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x14 + index as u64;
            parse.call(c, case + 0x14)?;
            c.lq(16, sp(c).wrapping_add(0x50));
            PuExit::Restored
        }
        PU_CASE_DEFAULT => pu_default(c),
        _ => unreachable!(),
    };
    Ok(Some(exit))
}

/// `a0 = self` already: next token, return `s0` (0x323f68).
pub(crate) fn pu_token_and_return_s0(c: &mut Cpu, ret: u32) -> Result<PuExit, Unwound> {
    c.cycles += 4;
    PU_NEXT_TOKEN.call(c, ret)?;
    c.g[V0][0] = c.g[S0][0];
    Ok(PuExit::Done)
}

/// "Expected term in expression" (0x324100).
pub(crate) fn pu_default(c: &mut Cpu) -> PuExit {
    c.cycles += 3;
    pu_error_args(c, PU_MSG_EXPECTED_TERM)
}

/// Is `t` one of the switch's targets?
pub(crate) fn pu_is_case(t: u32) -> bool {
    matches!(
        t,
        PU_CASE_NAME
            | PU_CASE_NAME_DOT
            | PU_CASE_NUMBER
            | PU_CASE_PLUS
            | PU_CASE_MINUS
            | PU_CASE_NOT
            | PU_CASE_PAREN
            | PU_CASE_DEFAULT
    ) || ((PU_CASE_FUNC_FIRST..=PU_CASE_FUNC_LAST).contains(&t)
        && (t - PU_CASE_FUNC_FIRST) % 0x1c == 0)
}

/// `ParseNode* InputParser_ParseUnary(InputParser* self)` (0x323bd8,
/// `cInputMapParser_parseTerm`): parse one term of an input-map expression
/// at the current token: a button (`button12.pressed`), a config value
/// (`padtype`, `arg3`, `config7`), a defined identifier (constants are
/// copied), a number, unary `+`/`-`/`!` (constants folded), a parenthesised
/// expression or a built-in function call (`max(a, b)`, `sin(x)`). Returns
/// the node, or 0 after reporting a syntax error.
pub fn input_parser_parse_unary(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = pu_token(c);
        let exit = loop {
            // switch (token - 2), 0x25 cases.
            c.cycles += 4;
            c.g[A0][0] = sx(pu_r(c, V0).wrapping_sub(2));
            c.g[V1][0] = (c.g[A0][0] < 0x25) as u64;
            c.g[V0][0] = 0x0049_0000;
            if c.g[V1][0] == 0 {
                break pu_default(c);
            }
            c.cycles += 6;
            c.g[V1][0] = sx(pu_r(c, A0) << 2);
            c.g[V0][0] = sx(PU_JUMP_TABLE);
            c.g[V1][0] = sx(pu_r(c, V1).wrapping_add(PU_JUMP_TABLE));
            c.g[A0][0] = c.lw(pu_r(c, V1));
            let case = pu_r(c, A0);
            if !pu_is_case(case) {
                c.jump_out(case, 0x0032_3bd8, 0x0032_4138);
                return Ok(());
            }
            match pu_case(c, case)? {
                Some(exit) => break exit,
                None => {
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_3bfc);
                    }
                }
            }
        };
        let mut exit = exit;
        if let PuExit::Error = exit {
            c.cycles += 2;
            PU_ERROR.call(c, 0x0032_4114)?;
            exit = PuExit::Zero;
        }
        if let PuExit::Zero = exit {
            c.cycles += 1;
            c.g[V0][0] = 0;
            exit = PuExit::Done;
        }
        if let PuExit::Done = exit {
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// InputParser_LookupBuiltinName (0x321880)

/// How a `<prefix><number>` name parse ended.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum BuiltinNameParse {
    /// No digit after the prefix.
    NoDigit,
    /// The number runs into a letter.
    Alpha,
    /// Followed by `_`, or the number is too large.
    Rejected,
    /// `v0` = number + base.
    Accepted,
}

/// Parse the decimal number at `name + off` (s0 = name): `n` in register
/// `n_reg`, the multiplier 10 / the letter flag in `ten_reg`, the next char
/// (unsigned) in `ch_reg`. Accepted when the number is followed by neither a
/// letter nor `_` and is below `limit`; `v0` = n + `base`.
#[allow(clippy::too_many_arguments)]
pub(crate) fn builtin_name_parse(
    c: &mut Cpu,
    off: u32,
    n_reg: usize,
    ten_reg: usize,
    ch_reg: usize,
    limit: i64,
    base: u32,
    head: u32,
    second: bool,
) -> BuiltinNameParse {
    let name = c.g[S0][0] as u32;
    c.cycles += 6;
    c.g[V1][0] = c.lb(name.wrapping_add(off));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30));
    c.g[V0][0] = c.g[V1][0] & 0xff;
    c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
    c.g[n_reg][0] = c.g[V1][0];
    if c.g[V0][0] == 0 {
        return BuiltinNameParse::NoDigit;
    }
    c.cycles += 5;
    c.g[V0][0] = c.lb(name.wrapping_add(off + 1));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
    c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
    c.g[A1][0] = sx(name.wrapping_add(off + 1));
    if c.g[V0][0] != 0 {
        c.cycles += 1;
        c.g[ten_reg][0] = 0xa;
        loop {
            // n = n * 10 + (*p++ - '0')
            c.cycles += 9;
            c.mult(0, V1, n_reg, ten_reg);
            c.g[A0][0] = c.lb(c.g[A1][0] as u32);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = c.lb(c.g[A1][0] as u32);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
            c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
            let more = c.g[V0][0] != 0;
            c.g[n_reg][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, head);
            }
        }
    }
    // A letter after the number?
    c.cycles += 6;
    c.g[V1][0] = c.lb(c.g[A1][0] as u32);
    c.g[ten_reg][0] = 0;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x61));
    c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
    c.g[ch_reg][0] = c.lbu(c.g[A1][0] as u32);
    if c.g[V0][0] == 0 {
        c.cycles += 4;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x41));
        c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
    }
    if c.g[V0][0] != 0 {
        c.cycles += 1;
        c.g[ten_reg][0] = 0x1;
    }
    c.cycles += 2;
    if second {
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    }
    if c.g[ten_reg][0] != 0 {
        return BuiltinNameParse::Alpha;
    }
    c.cycles += 5;
    c.g[V0][0] = sx((c.g[ch_reg][0] as u32) << 24);
    c.g[V1][0] = 0x5f;
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
    let underscore = c.g[V0][0] == c.g[V1][0];
    c.g[V0][0] = ((c.g[n_reg][0] as i64) < limit) as u64;
    if underscore {
        return BuiltinNameParse::Rejected;
    }
    c.cycles += 2;
    let small = c.g[V0][0] != 0;
    c.g[V0][0] = sx((c.g[n_reg][0] as u32).wrapping_add(base));
    if small {
        BuiltinNameParse::Accepted
    } else {
        BuiltinNameParse::Rejected
    }
}

/// `int InputParser_LookupBuiltinName(cInputMapParser* self, const char*
/// name)` (0x321880): the built-in config names of an input map: `padtype`
/// -> 0, `arg<N>` (N < 8) -> N + 1, `config<N>` (N < 23) -> N + 9, else -1
/// (case-insensitive prefixes; the number may not run into a letter or `_`).
pub fn input_parser_lookup_builtin_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x4a_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3f18)); // "padtype"
        call_ptr(c, 0x0041_aa88, 0x0032_18a0)?; // strcasecmp
        let differs = c.g[V0][0] != 0;
        c.g[A1][0] = 0x4a_0000;
        'exit: {
            if !differs {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'exit;
            }
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3f20)); // "arg"
            c.g[A2][0] = 0x3;
            call_ptr(c, 0x0041_ab78, 0x0032_18c0)?; // strncasecmp
            let differs = c.g[V0][0] != 0;
            c.g[A1][0] = 0x4a_0000;
            if !differs {
                match builtin_name_parse(c, 3, A0, A2, A1, 8, 1, 0x0032_18f8, false) {
                    BuiltinNameParse::NoDigit => {}
                    BuiltinNameParse::Alpha => c.g[A1][0] = 0x4a_0000,
                    BuiltinNameParse::Rejected => {
                        c.cycles += 1;
                        c.g[A1][0] = 0x4a_0000;
                    }
                    BuiltinNameParse::Accepted => break 'exit,
                }
            }
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3f28)); // "config"
            c.g[A2][0] = 0x6;
            call_ptr(c, 0x0041_ab78, 0x0032_1980)?; // strncasecmp
            let differs = c.g[V0][0] != 0;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            if differs {
                break 'exit;
            }
            match builtin_name_parse(c, 6, A2, A3, A0, 23, 9, 0x0032_19b8, true) {
                BuiltinNameParse::Alpha | BuiltinNameParse::Accepted => {}
                BuiltinNameParse::NoDigit | BuiltinNameParse::Rejected => {
                    c.cycles += 1;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// The input map's expression parser: comparison, additive and
// multiplicative levels (0x3236d8, 0x323900, 0x323a68).

/// Parser fields: the current token kind and the free node list.
pub(crate) const INPUT_PARSER_TOKEN: u32 = 0x98;

/// Parse node type of a constant (float value at +4).
pub(crate) const INPUT_PARSER_CONST: u64 = 0x4;

pub(crate) const INPUT_PARSER_NEXT_TOKEN: Callee = Callee {
    addr: 0x0032_1af0,
    f: ee::s10::InputParser_NextToken,
};

pub(crate) const INPUT_PARSER_FREE_NODE: Callee = Callee {
    addr: 0x0032_1638,
    f: ee::s10::InputParser_FreeNode,
};

pub(crate) const INPUT_PARSER_ALLOC_NODE: Callee = Callee {
    addr: 0x0032_15f8,
    f: ee::s10::InputParser_AllocNode,
};

pub(crate) const INPUT_PARSER_ADDITIVE: Callee = Callee {
    addr: 0x0032_3900,
    f: ee::s10::InputParser_ParseAdditive,
};

pub(crate) const INPUT_PARSER_MULTIPLICATIVE: Callee = Callee {
    addr: 0x0032_3a68,
    f: ee::s10::InputParser_ParseMultiplicative,
};

pub(crate) const INPUT_PARSER_UNARY: Callee = Callee {
    addr: 0x0032_3bd8,
    f: ee::s10::InputParser_ParseUnary,
};

/// One left-associative binary level (`+ -` or `* /`): its operand parser,
/// tokens, node types and the return addresses of its calls.
pub(crate) struct InputParserLevel {
    operand: Callee,
    /// Tokens of the two operators, and their node types.
    tok: [u64; 2],
    op: [u64; 2],
    /// Multiplicative (`* /`, the inverse is 1/x) or additive (`+ -`, -x).
    mul: bool,
    /// Return addresses: first operand, NextToken, operand, free (failure),
    /// free (folded), alloc (constant chain), alloc (node).
    ret: [u32; 7],
    /// Loop heads: outer (reload the token), inner (token already in v1).
    head: [u32; 2],
}

pub(crate) const INPUT_PARSER_ADDITIVE_LEVEL: InputParserLevel = InputParserLevel {
    operand: INPUT_PARSER_MULTIPLICATIVE,
    tok: [0x14, 0x15],
    op: [0xe, 0xf],
    mul: false,
    ret: [
        0x0032_3924,
        0x0032_3974,
        0x0032_397c,
        0x0032_3994,
        0x0032_39f4,
        0x0032_3a1c,
        0x0032_3a34,
    ],
    head: [0x0032_3940, 0x0032_3944],
};

pub(crate) const INPUT_PARSER_MULTIPLICATIVE_LEVEL: InputParserLevel = InputParserLevel {
    operand: INPUT_PARSER_UNARY,
    tok: [0x16, 0x17],
    op: [0x10, 0x11],
    mul: true,
    ret: [
        0x0032_3a8c,
        0x0032_3adc,
        0x0032_3ae4,
        0x0032_3afc,
        0x0032_3b5c,
        0x0032_3b8c,
        0x0032_3ba4,
    ],
    head: [0x0032_3aa8, 0x0032_3aac],
};

/// The shared body of [`input_parser_parse_additive`] and
/// [`input_parser_parse_multiplicative`]: `left (op right)*`. Constants are
/// folded into the first constant operand (s4); the inverse operator turns a
/// constant right operand into `-x` / `1/x` and chains it with the first
/// operator; anything else becomes a binary node. Returns the tree, or 0 (an
/// operand failed; the left tree is freed).
pub(crate) fn input_parser_binary_level(c: &mut Cpu, lv: &InputParserLevel) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x50);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        lv.operand.call(c, lv.ret[0])?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = 0;
        'failed: {
            if c.g[S1][0] == 0 {
                break 'failed;
            }
            c.cycles += 4;
            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
            c.g[S4][0] = 0;
            c.g[V0][0] ^= INPUT_PARSER_CONST;
            if c.g[V0][0] == 0 {
                c.g[S4][0] = c.g[S1][0];
            }
            loop {
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(INPUT_PARSER_TOKEN));
                loop {
                    // The operator, or done.
                    c.cycles += 3;
                    c.g[V0][0] = lv.tok[0];
                    let first = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = lv.tok[1];
                    if first {
                        c.cycles += 2;
                        c.g[S3][0] = lv.op[0];
                    } else {
                        c.cycles += 2;
                        let second = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = c.g[S1][0];
                        if !second {
                            c.cycles += 2;
                            frame.restore(c, S0, 0x50);
                            c.cycles += 7;
                            input_parser_binary_level_epilogue(c, frame);
                            return Ok(());
                        }
                        c.cycles += 1;
                        c.g[S3][0] = lv.op[1];
                    }
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S2][0];
                    INPUT_PARSER_NEXT_TOKEN.call(c, lv.ret[1])?;
                    c.g[A0][0] = c.g[S2][0];
                    lv.operand.call(c, lv.ret[2])?;
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[S0][0] == 0 {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S1][0];
                        INPUT_PARSER_FREE_NODE.call(c, lv.ret[3])?;
                        c.g[V0][0] = 0;
                        break 'failed;
                    }
                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                    c.cycles += 3;
                    c.g[V0][0] = INPUT_PARSER_CONST;
                    if c.g[V1][0] != c.g[V0][0] {
                        // A binary node (op, left, right).
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S2][0];
                        INPUT_PARSER_ALLOC_NODE.call(c, lv.ret[6])?;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                        c.sw(c.g[V0][0] as u32, c.g[S3][0]);
                        break;
                    }
                    c.cycles += 2;
                    let no_const = c.g[S4][0] == 0;
                    c.g[V0][0] = lv.op[0];
                    if no_const {
                        // Start a constant chain: invert the right operand
                        // for the second operator, then node(op0, left, right).
                        c.cycles += 3;
                        c.g[V0][0] = lv.op[1];
                        let inverse = c.g[S3][0] == c.g[V0][0];
                        c.g[S4][0] = c.g[S0][0];
                        if inverse {
                            let rv = (c.g[S0][0] as u32).wrapping_add(0x4);
                            if lv.mul {
                                c.cycles += 5;
                                c.lwc1(1, rv);
                                c.g[AT][0] = 0x3f80_0000;
                                c.mtc1(1, 0);
                                c.div_s(0, 0, 1);
                            } else {
                                c.cycles += 3;
                                c.lwc1(0, rv);
                                c.neg_s(0, 0);
                            }
                            c.swc1(0, rv);
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S2][0];
                        INPUT_PARSER_ALLOC_NODE.call(c, lv.ret[5])?;
                        c.g[V1][0] = lv.op[0];
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                        break;
                    }
                    // Fold the right constant into s4 and free it.
                    c.cycles += 2;
                    let is_first = c.g[S3][0] == c.g[V0][0];
                    c.g[V0][0] = lv.op[1];
                    let mut folded = true;
                    let (s4, s0) = (c.g[S4][0] as u32, c.g[S0][0] as u32);
                    if is_first {
                        c.cycles += 4;
                        c.lwc1(0, s4.wrapping_add(0x4));
                        c.lwc1(1, s0.wrapping_add(0x4));
                        if lv.mul {
                            c.mul_s(0, 0, 1);
                        } else {
                            c.add_s(0, 0, 1);
                        }
                    } else {
                        c.cycles += 2;
                        let is_second = c.g[S3][0] == c.g[V0][0];
                        c.g[A1][0] = c.g[S0][0];
                        if is_second {
                            c.cycles += 3;
                            c.lwc1(0, s4.wrapping_add(0x4));
                            c.lwc1(1, s0.wrapping_add(0x4));
                            if lv.mul {
                                c.div_s(0, 0, 1);
                            } else {
                                c.sub_s(0, 0, 1);
                            }
                        } else {
                            c.cycles += 2;
                            folded = false;
                        }
                    }
                    if folded {
                        c.cycles += 2;
                        c.swc1(0, s4.wrapping_add(0x4));
                        c.g[A1][0] = c.g[S0][0];
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    INPUT_PARSER_FREE_NODE.call(c, lv.ret[4])?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(INPUT_PARSER_TOKEN));
                    if event_due(c) {
                        deliver_event_here(c, lv.head[1]);
                    }
                }
                // left = node; node.right = right.
                c.cycles += 3;
                c.g[S1][0] = c.g[V0][0];
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[S0][0]);
                if event_due(c) {
                    deliver_event_here(c, lv.head[0]);
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x50);
        c.cycles += 7;
        input_parser_binary_level_epilogue(c, frame);
        Ok(())
    })
}

pub(crate) fn input_parser_binary_level_epilogue(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore(c, S4, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `ParseNode* InputParser_ParseAdditive(cInputMapParser* self)` (0x323900):
/// `term ((+|-) term)*` over [`input_parser_parse_multiplicative`], folding
/// constants (node types 0xe add, 0xf subtract).
pub fn input_parser_parse_additive(c: &mut Cpu) {
    input_parser_binary_level(c, &INPUT_PARSER_ADDITIVE_LEVEL)
}

/// `ParseNode* InputParser_ParseMultiplicative(cInputMapParser* self)`
/// (0x323a68): `unary ((*|/) unary)*`, folding constants (node types 0x10
/// multiply, 0x11 divide).
pub fn input_parser_parse_multiplicative(c: &mut Cpu) {
    input_parser_binary_level(c, &INPUT_PARSER_MULTIPLICATIVE_LEVEL)
}

/// `ParseNode* InputParser_ParseComparison(cInputMapParser* self)`
/// (0x3236d8): `sum (cmp sum)*` with `> >= < <= == !=` (tokens 0xe..0x13,
/// node types 8, 9, 0xa, 0xb, 0xc, 0xd through the jump table at 0x48dfd0).
/// Two constants compare to a constant 1.0 / 0.0 (the left node is reused,
/// the right one freed); otherwise a binary node. 0 when an operand fails.
pub fn input_parser_parse_comparison(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        INPUT_PARSER_ADDITIVE.call(c, 0x0032_36f8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = 0;
        'exit: {
            if c.g[S0][0] == 0 {
                break 'exit;
            }
            c.cycles += 1;
            // 0x323708: reload the token; 0x32370c: dispatch on it.
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(INPUT_PARSER_TOKEN));
            loop {
                c.cycles += 4;
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
                c.g[V1][0] = (c.g[A0][0] < 0x6) as u64;
                let is_cmp = c.g[V1][0] != 0;
                c.g[V0][0] = 0x49_0000;
                if !is_cmp {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    break 'exit;
                }
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2030));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                let op = match c.g[A0][0] as u32 {
                    0x0032_3734 => 0xa,
                    0x0032_373c => 0xb,
                    0x0032_3744 => 0x9,
                    0x0032_374c => 0x8,
                    0x0032_3754 => 0xc,
                    0x0032_375c => 0xd,
                    t => {
                        c.jump_out(t, 0x0032_36d8, 0x0032_3900);
                        return Ok(());
                    }
                };
                c.cycles += 2;
                c.g[S2][0] = op;
                c.cycles += 7;
                c.g[A0][0] = c.g[S3][0];
                INPUT_PARSER_NEXT_TOKEN.call(c, 0x0032_3774)?;
                c.g[A0][0] = c.g[S3][0];
                INPUT_PARSER_ADDITIVE.call(c, 0x0032_377c)?;
                c.g[S1][0] = c.g[V0][0];
                if c.g[S1][0] == 0 {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S0][0];
                    INPUT_PARSER_FREE_NODE.call(c, 0x0032_3794)?;
                    c.g[V0][0] = 0;
                    break 'exit;
                }
                c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                let mut fold = false;
                c.cycles += 3;
                c.g[V0][0] = INPUT_PARSER_CONST;
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                    fold = c.g[V0][0] == c.g[V1][0];
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x8));
                }
                if !fold {
                    // node(op, left, right)
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S3][0];
                    INPUT_PARSER_ALLOC_NODE.call(c, 0x0032_38d0)?;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                    c.sw(c.g[V0][0] as u32, c.g[S2][0]);
                    c.g[S0][0] = c.g[V0][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[S1][0]);
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_3708);
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(INPUT_PARSER_TOKEN));
                    continue;
                }
                // Both constant: left.value = (left op right) ? 1 : 0.
                c.cycles += 3;
                c.g[V0][0] = (c.g[V1][0] < 0x6) as u64;
                let in_table = c.g[V0][0] != 0;
                c.g[V0][0] = 0x49_0000;
                if in_table {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2010));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let target = c.g[A0][0] as u32;
                    if !matches!(
                        target,
                        0x0032_37d8
                            | 0x0032_37fc
                            | 0x0032_3820
                            | 0x0032_3844
                            | 0x0032_3868
                            | 0x0032_388c
                    ) {
                        c.jump_out(target, 0x0032_36d8, 0x0032_3900);
                        return Ok(());
                    }
                    c.cycles += 7;
                    let (l, r) = (
                        (c.g[S0][0] as u32).wrapping_add(4),
                        (c.g[S1][0] as u32).wrapping_add(4),
                    );
                    c.lwc1(1, l);
                    c.lwc1(0, r);
                    c.mtc1(0, 2);
                    // fcc = the condition for 1.0 (f1 = left, f0 = right).
                    let truth = match target {
                        0x0032_37d8 => {
                            c.c_lt_s(1, 0);
                            c.fcc()
                        }
                        0x0032_37fc => {
                            c.c_le_s(1, 0);
                            c.fcc()
                        }
                        0x0032_3820 => {
                            c.c_le_s(0, 1);
                            c.fcc()
                        }
                        0x0032_3844 => {
                            c.c_lt_s(0, 1);
                            c.fcc()
                        }
                        0x0032_3868 => {
                            c.c_eq_s(1, 0);
                            c.fcc()
                        }
                        _ => {
                            c.c_eq_s(1, 0);
                            !c.fcc()
                        }
                    };
                    if !truth {
                        c.swc1(2, l);
                    } else {
                        if target != 0x0032_388c {
                            c.cycles += 2;
                        }
                        c.cycles += 3;
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(1, 2);
                        c.swc1(2, l);
                    }
                }
                // Free the right constant.
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S3][0];
                INPUT_PARSER_FREE_NODE.call(c, 0x0032_38c0)?;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(INPUT_PARSER_TOKEN));
                if event_due(c) {
                    deliver_event_here(c, 0x0032_370c);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------- helpers

/// `lw` through a register: `*(reg + off)` (sign-extended word).
pub(crate) fn o406_lw(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    c.lw(a)
}

/// `sw` through a register: `*(reg + off) = val_reg`.
pub(crate) fn o406_sw(c: &mut Cpu, reg: usize, off: u32, val_reg: usize) {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    let v = c.g[val_reg][0];
    c.sw(a, v);
}

// ----------------------------------------------- InputParser_ParseLogicalAnd

/// Is the constant node `node` (kind 4, value at +4) equal to 0.0 on the FPU
/// compare the parser uses (`c.eq.s f1, f0`)? Sets f1/f0/fcc like the original.
pub(crate) fn o406_const_is_zero(c: &mut Cpu, node_reg: usize) -> bool {
    let a = (c.g[node_reg][0] as u32).wrapping_add(4);
    c.lwc1(1, a);
    c.mtc1(0, 0);
    c.c_eq_s(1, 0);
    c.fcc()
}

/// `tParseNode *InputParser_ParseLogicalAnd(InputParser *self)` (0x323540):
/// `a && b && ...` with constant folding (a constant 0 operand makes the
/// whole chain 0, a constant non-zero operand disappears). Node kind 7 is
/// the `&&` node (children +4, +8); token 0xd is `&&`.
pub fn input_parser_parse_logical_and(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S2, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S0, 0x70);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x88);
        frame.save_fpr(c, 20, 0x80);
        call_ptr(c, 0x0032_36d8, 0x0032_3574)?;
        c.g[S1][0] = c.g[V0][0];
        'body: {
            if c.g[S1][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'body;
            }
            c.g[V1][0] = o406_lw(c, S1, 0);
            c.cycles += 3;
            c.g[V0][0] = 4;
            c.g[S4][0] = 0;
            if c.g[V1][0] == 4 {
                c.cycles += 6;
                if o406_const_is_zero(c, S1) {
                    c.g[S4][0] = 1;
                }
            }
            c.cycles += 4;
            c.g[V1][0] = o406_lw(c, S2, 0x98);
            c.g[V0][0] = 0xd;
            c.g[V0][0] = c.g[S1][0];
            if c.g[V1][0] != 0xd {
                break 'body;
            }
            c.cycles += 5;
            c.mtc1(0, 21);
            c.g[S6][0] = 7;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 20);
            c.g[S5][0] = 0xd;
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0032_1af0, 0x0032_35e8)?;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0032_36d8, 0x0032_35f0)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] == 0 {
                    c.g[A0][0] = c.g[S2][0];
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_3588);
                    }
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0032_1638, 0x0032_3590)?;
                    c.g[V0][0] = 0;
                    break 'body;
                }
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                'iter: {
                    if c.g[S4][0] != 0 {
                        // The chain is already constant 0: drop the operand.
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0032_1638, 0x0032_360c)?;
                        c.g[V0][0] = o406_lw(c, S2, 0x98);
                        break 'iter;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = o406_lw(c, S0, 0);
                    c.g[S3][0] = 4;
                    if c.g[V0][0] != 4 {
                        c.g[V0][0] = o406_lw(c, S1, 0);
                        c.cycles += 2;
                        c.g[A1][0] = c.g[S1][0];
                        if c.g[V0][0] == 4 {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x0032_1638, 0x0032_367c)?;
                            c.g[S1][0] = c.g[S0][0];
                        } else {
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x0032_15f8, 0x0032_368c)?;
                            o406_sw(c, V0, 4, S1);
                            o406_sw(c, V0, 8, S0);
                            c.g[S1][0] = c.g[V0][0];
                            o406_sw(c, V0, 0, S6);
                        }
                    } else {
                        c.cycles += 5;
                        let a = (c.g[S0][0] as u32).wrapping_add(4);
                        c.lwc1(0, a);
                        c.c_eq_s(0, 21);
                        let nonzero = !c.fcc();
                        c.g[A1][0] = c.g[S1][0];
                        if nonzero {
                            c.cycles += 6;
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x0032_1638, 0x0032_3658)?;
                            c.g[V0][0] = o406_lw(c, S1, 0);
                            if c.g[V0][0] == c.g[S3][0] {
                                let a = (c.g[S1][0] as u32).wrapping_add(4);
                                c.swc1(20, a);
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] = o406_lw(c, S2, 0x98);
                                break 'iter;
                            }
                        } else {
                            // The operand is the constant 0: the chain is 0.
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x0032_1638, 0x0032_3640)?;
                            c.g[S1][0] = c.g[S0][0];
                            c.g[S4][0] = 1;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = o406_lw(c, S2, 0x98);
                }
                c.cycles += 2;
                let more = c.g[V0][0] == c.g[S5][0];
                c.g[V0][0] = c.g[S1][0];
                if !more {
                    break 'body;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_35e0);
                }
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x88);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ InputParser_ParseLogicalOr

/// `tParseNode *InputParser_ParseLogicalOr(InputParser *self)` (0x3233b8):
/// `a || b || ...` over `ParseLogicalAnd`, with constant folding (a constant
/// non-zero operand makes the chain 1.0, a constant 0 operand disappears).
/// Node kind 6 is the `||` node; token 0xc is `||`.
pub fn input_parser_parse_logical_or(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S2, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S0, 0x60);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x78);
        frame.save_fpr(c, 20, 0x70);
        call_ptr(c, 0x0032_3540, 0x0032_33e8)?;
        c.g[S1][0] = c.g[V0][0];
        'body: {
            if c.g[S1][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'body;
            }
            c.g[V1][0] = o406_lw(c, S1, 0);
            c.cycles += 3;
            c.g[V0][0] = 4;
            c.g[S3][0] = 0;
            if c.g[V1][0] == 4 {
                c.cycles += 6;
                if !o406_const_is_zero(c, S1) {
                    c.g[S3][0] = 1;
                }
            }
            c.cycles += 4;
            c.g[V1][0] = o406_lw(c, S2, 0x98);
            c.g[V0][0] = 0xc;
            c.g[V0][0] = c.g[S1][0];
            if c.g[V1][0] != 0xc {
                break 'body;
            }
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 20);
            c.g[S5][0] = 6;
            c.mtc1(0, 21);
            c.g[S4][0] = 0xc;
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0032_1af0, 0x0032_3460)?;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0032_3540, 0x0032_3468)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] == 0 {
                    c.g[A0][0] = c.g[S2][0];
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_33fc);
                    }
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0032_1638, 0x0032_3404)?;
                    c.g[V0][0] = 0;
                    break 'body;
                }
                c.cycles += 2;
                'iter: {
                    // true: go on to the node-combining code at 0x323484
                    // (after the `s3` shortcut).
                    let mut combine = true;
                    if c.g[S3][0] != 0 {
                        // The chain is already constant 1.0.
                        c.cycles += 2;
                        let a = (c.g[S1][0] as u32).wrapping_add(4);
                        c.swc1(20, a);
                        combine = false;
                    } else {
                        c.g[V0][0] = o406_lw(c, S0, 0);
                    }
                    if combine {
                        c.cycles += 3;
                        c.g[V1][0] = 4;
                        if c.g[V0][0] != c.g[V1][0] {
                            c.g[V0][0] = o406_lw(c, S1, 0);
                            c.cycles += 2;
                            c.g[A1][0] = c.g[S1][0];
                            if c.g[V0][0] == c.g[V1][0] {
                                c.cycles += 4;
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x0032_1638, 0x0032_34e4)?;
                                c.g[S1][0] = c.g[S0][0];
                            } else {
                                c.cycles += 6;
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x0032_15f8, 0x0032_34f4)?;
                                o406_sw(c, V0, 4, S1);
                                o406_sw(c, V0, 8, S0);
                                c.g[S1][0] = c.g[V0][0];
                                o406_sw(c, V0, 0, S5);
                            }
                            c.cycles += 1;
                            c.g[V0][0] = o406_lw(c, S2, 0x98);
                            break 'iter;
                        }
                        c.cycles += 5;
                        let a = (c.g[S0][0] as u32).wrapping_add(4);
                        c.lwc1(0, a);
                        c.c_eq_s(0, 21);
                        let zero = c.fcc();
                        c.g[A1][0] = c.g[S0][0];
                        if !zero {
                            // The operand is a non-zero constant: the chain is 1.0.
                            c.cycles += 7;
                            let a = (c.g[S0][0] as u32).wrapping_add(4);
                            c.swc1(20, a);
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A0][0] = c.g[S2][0];
                            c.g[S1][0] = c.g[S0][0];
                            call_ptr(c, 0x0032_1638, 0x0032_34b8)?;
                            c.g[S3][0] = 1;
                            c.cycles += 1;
                            c.g[V0][0] = o406_lw(c, S2, 0x98);
                            break 'iter;
                        }
                    } else {
                        c.cycles += 1;
                        c.g[A1][0] = c.g[S0][0];
                    }
                    // 0x3234c4: drop the operand `s0`.
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0032_1638, 0x0032_34cc)?;
                    c.g[V0][0] = o406_lw(c, S2, 0x98);
                }
                c.cycles += 2;
                let more = c.g[V0][0] == c.g[S4][0];
                c.g[V0][0] = c.g[S1][0];
                if !more {
                    break 'body;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_3458);
                }
            }
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x78);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ InputParser_ParseConditional

/// `tParseNode *InputParser_ParseConditional(InputParser *self)` (0x323268):
/// `cond ? a : b` over `ParseLogicalOr` (token 6 `?`, token 7 `:`), folding
/// a constant condition. Node kind 5 is the select node (children +4 cond,
/// +8 then, +0xc else).
pub fn input_parser_parse_conditional(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0032_33b8, 0x0032_3288)?;
        c.g[S1][0] = c.g[V0][0];
        'body: {
            if c.g[S1][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'body;
            }
            c.g[V1][0] = o406_lw(c, S0, 0x98);
            c.cycles += 3;
            c.g[V0][0] = 6;
            c.g[V0][0] = c.g[S1][0];
            if c.g[V1][0] != 6 {
                break 'body;
            }
            // `?`: the "then" expression.
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0032_1af0, 0x0032_32b0)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0032_3268, 0x0032_32b8)?;
            c.g[S3][0] = c.g[V0][0];
            'fail: {
                // `fail`: free the condition, then the "then" node, return 0.
                if c.g[S3][0] == 0 {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    break 'fail;
                }
                c.g[V1][0] = o406_lw(c, S0, 0x98);
                c.cycles += 3;
                c.g[V0][0] = 7;
                c.g[A1][0] = 0x0049_0000;
                if c.g[V1][0] != 7 {
                    // Missing `:`.
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2058));
                    call_ptr(c, 0x0032_1500, 0x0032_32e8)?;
                    c.g[A1][0] = c.g[S1][0];
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0032_1af0, 0x0032_32f8)?;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0032_3268, 0x0032_3300)?;
                    c.g[S2][0] = c.g[V0][0];
                    if c.g[S2][0] != 0 {
                        // Both branches parsed: fold or build the select node.
                        c.g[V1][0] = o406_lw(c, S1, 0);
                        c.cycles += 3;
                        c.g[V0][0] = 4;
                        if c.g[V1][0] == 4 {
                            c.cycles += 6;
                            let zero = o406_const_is_zero(c, S1);
                            c.g[A0][0] = c.g[S0][0];
                            if !zero {
                                // Constant true: the "then" node.
                                c.cycles += 4;
                                c.g[A1][0] = c.g[S2][0];
                                call_ptr(c, 0x0032_1638, 0x0032_335c)?;
                                c.g[V0][0] = c.g[S3][0];
                                break 'body;
                            }
                            // Constant false: the "else" node.
                            c.cycles += 4;
                            c.g[A1][0] = c.g[S3][0];
                            call_ptr(c, 0x0032_1638, 0x0032_336c)?;
                            c.g[V0][0] = c.g[S2][0];
                            break 'body;
                        }
                        c.cycles += 9;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0032_15f8, 0x0032_337c)?;
                        c.g[V1][0] = c.g[V0][0];
                        c.g[V0][0] = 5;
                        o406_sw(c, V1, 4, S1);
                        o406_sw(c, V1, 0, V0);
                        o406_sw(c, V1, 8, S3);
                        c.g[V0][0] = c.g[V1][0];
                        o406_sw(c, V1, 0xc, S2);
                        break 'body;
                    }
                    c.cycles += 1;
                    c.g[A1][0] = c.g[S1][0];
                }
                // 0x323310: free the condition, then the "then" node.
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0032_1638, 0x0032_3318)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S3][0];
            }
            // 0x323320: free `a1` and return 0.
            c.cycles += 4;
            call_ptr(c, 0x0032_1638, 0x0032_3328)?;
            c.g[V0][0] = 0;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ InputParser_ParseStatement

/// `void InputParser_ParseStatement(InputParser *self)` (0x3230e0): one
/// `name = expression;` statement of an input map script. The name token
/// (kind 2) must not be a button, a builtin or an existing identifier; it
/// gets the next entry of the variable table (0x44 bytes each from +0x80e8,
/// count at +0x80e4), which is constructed; then `=` (token 3), the
/// expression (`ParseConditional`) and `;` (token 5) follow. Any error
/// skips to the end of the statement.
pub fn input_parser_parse_statement(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = 2;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = o406_lw(c, S0, 0x98);
        c.g[A1][0] = 0x0049_0000;
        // The error exits: `'skip` skips the statement (0x32322c),
        // `'finish` is the normal end (0x323254).
        'finish: {
            'accepted: {
                'skip: {
                    'error: {
                        if c.g[V0][0] != c.g[V1][0] {
                            c.cycles += 2;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2130));
                            break 'error;
                        }
                        c.cycles += 6;
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x0032_16e8, 0x0032_3120)?;
                        let found = (c.g[V0][0] as i64) >= 0;
                        c.g[A1][0] = 0x0049_0000;
                        if found {
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2108));
                            call_ptr(c, 0x0032_1500, 0x0032_3138)?;
                            break 'skip;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x0032_1880, 0x0032_314c)?;
                        let found = (c.g[V0][0] as i64) >= 0;
                        c.g[A1][0] = 0x0049_0000;
                        if found {
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x20e0));
                            call_ptr(c, 0x0032_1500, 0x0032_3164)?;
                            break 'skip;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x0032_1a40, 0x0032_3178)?;
                        let found = c.g[V0][0] != 0;
                        c.g[A1][0] = 0x0049_0000;
                        if found {
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x20b8));
                            call_ptr(c, 0x0032_1500, 0x0032_3190)?;
                            break 'skip;
                        }
                        // A new variable: entry `count` of the table.
                        c.cycles += 20;
                        c.g[V0][0] = 0x8000;
                        c.g[A2][0] = 0x44;
                        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A0][0] = 0x80e8;
                        c.g[V0][0] = o406_lw(c, S2, 0xe4);
                        c.g[A1][0] = c.g[S1][0];
                        c.g[V1][0] = 0x10000;
                        c.mult(0, V0, V0, A2);
                        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        o406_sw(c, V1, 0x8e8, S1);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x0041_66f4, 0x0032_31d0)?;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0032_1af0, 0x0032_31d8)?;
                        c.g[V1][0] = o406_lw(c, S0, 0x98);
                        c.g[V0][0] = 3;
                        c.g[A1][0] = 0x0049_0000;
                        if c.g[V1][0] != 3 {
                            // Missing `=`.
                            c.cycles += 3;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2098));
                            break 'error;
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0032_1af0, 0x0032_31fc)?;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0032_3268, 0x0032_3204)?;
                        o406_sw(c, S1, 0x40, V0);
                        if c.g[V0][0] == 0 {
                            break 'skip;
                        }
                        c.cycles += 4;
                        c.g[V1][0] = o406_lw(c, S0, 0x98);
                        c.g[V0][0] = 5;
                        c.g[A1][0] = 0x0049_0000;
                        if c.g[V1][0] == 5 {
                            break 'accepted;
                        }
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2078));
                    }
                    // 0x323224: report the error.
                    c.cycles += 2;
                    call_ptr(c, 0x0032_1500, 0x0032_322c)?;
                }
                // 0x32322c: skip the rest of the statement.
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0032_1590, 0x0032_3234)?;
                frame.restore(c, S0, 0x30);
                break 'finish;
            }
            // 0x32323c: the `;` ends the statement; count the variable.
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0032_1af0, 0x0032_3244)?;
            c.g[V0][0] = o406_lw(c, S2, 0xe4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            o406_sw(c, S2, 0xe4, V0);
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ InputParser_EmitBinaryOp

/// `int InputParser_EmitBinaryOp(InputParser *self, Node *node, int *dst,
/// u32 *code, int cap, int op, int a, int b)` (0x324ec8): compile both
/// operands of a binary node (`node+4`, `node+8`) with `EmitNode`, free
/// their temporary registers, pick the destination register, and emit the
/// opcode word (`op` 6 bits, `dst` 6 bits, src 10 bits, arg 10 bits).
/// Returns the number of words written, or -1 after an error.
pub fn input_parser_emit_binary_op(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        c.g[S0][0] = c.g[T0][0];
        frame.save(c, S2, 0x80);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x70);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S4, 0x60);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S5, 0x50);
        c.g[S4][0] = c.g[T2][0];
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        c.g[S6][0] = c.g[A2][0];
        frame.save(c, FP, 0x20);
        c.g[S7][0] = u64::MAX;
        frame.save_ra(c, 0x10);
        c.g[FP][0] = c.g[T1][0];
        c.g[A2][0] = c.g[SP][0];
        c.g[A1][0] = o406_lw(c, S3, 4);
        o406_sw(c, SP, 0, S7);
        c.g[T1][0] = c.g[S4][0];
        call_ptr(c, 0x0032_4678, 0x0032_4f28)?;
        c.g[S5][0] = c.g[V0][0];
        'body: {
            if (c.g[S5][0] as i64) < 0 {
                c.g[V0][0] = u64::MAX;
                break 'body;
            }
            c.cycles += 14;
            c.g[V0][0] = sx((c.g[S5][0] as u32) << 2);
            c.g[A1][0] = o406_lw(c, S3, 8);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[S5][0] as u32));
            c.g[T0][0] = c.g[S0][0];
            o406_sw(c, SP, 4, S7);
            c.g[T1][0] = c.g[S4][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = c.g[SP][0] | 4;
            c.g[A3][0] = c.g[S2][0];
            call_ptr(c, 0x0032_4678, 0x0032_4f60)?;
            c.g[T0][0] = c.g[V0][0];
            if (c.g[T0][0] as i64) < 0 {
                c.g[V0][0] = u64::MAX;
                break 'body;
            }
            c.cycles += 6;
            c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
            c.g[A1][0] = o406_lw(c, SP, 4);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = (c.g[A1][0] < 0x40) as u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[T0][0] as u32));
            if c.g[V0][0] != 0 {
                c.cycles += 8;
                o406_emit_free_reg(c);
            }
            c.cycles += 4;
            c.g[A1][0] = o406_lw(c, SP, 0);
            c.g[V0][0] = (c.g[A1][0] < 0x40) as u64;
            c.g[A0][0] = 0x10000;
            if c.g[V0][0] != 0 {
                c.cycles += 7;
                o406_emit_free_reg(c);
            }
            'overflow: {
                'alloc_failed: {
                    c.cycles += 3;
                    c.g[V0][0] = o406_lw(c, S6, 0);
                    let have_dst = (c.g[V0][0] as i64) >= 0;
                    c.g[V0][0] = 0x10000;
                    if !have_dst {
                        c.cycles += 4;
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V0][0] = o406_lw(c, A0, 0x9e8);
                        let none = (c.g[V0][0] as i64) <= 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                        if !none {
                            c.cycles += 5;
                            c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                            o406_sw(c, A0, 0x9e8, V0);
                            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[V0][0] = o406_lw(c, V1, 0x8ec);
                            o406_sw(c, S6, 0, V0);
                        }
                        c.cycles += 3;
                        c.g[V0][0] = o406_lw(c, S6, 0);
                        let have_dst = (c.g[V0][0] as i64) >= 0;
                        c.g[A1][0] = 0x0049_0000;
                        if !have_dst {
                            c.cycles += 3;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1dc0));
                            break 'alloc_failed;
                        }
                    }
                    c.cycles += 2;
                    let no_room = (c.g[S0][0] as i64) <= 0;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffc0;
                    if no_room {
                        c.cycles += 3;
                        c.g[A1][0] = 0x0049_0000;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1dd8));
                        break 'overflow;
                    }
                    // Emit the opcode word.
                    c.cycles += 28;
                    c.g[V1][0] = o406_lw(c, S2, 0);
                    c.g[A0][0] = c.g[FP][0] & 0x3f;
                    c.g[A2][0] = o406_lw(c, SP, 0);
                    c.g[V1][0] &= c.g[V0][0];
                    c.g[A1][0] = 0xffff_ffff_ffff_f03f;
                    c.g[V1][0] |= c.g[A0][0];
                    c.g[A3][0] = 0xffff_ffff_ffc0_0000;
                    o406_sw(c, S2, 0, V1);
                    c.g[A2][0] &= 0x3ff;
                    c.g[V1][0] &= c.g[A1][0];
                    c.g[A3][0] |= 0xfff;
                    c.g[V0][0] = o406_lw(c, S6, 0);
                    c.g[A2][0] = sx((c.g[A2][0] as u32) << 12);
                    c.g[A1][0] = o406_lw(c, SP, 4);
                    c.g[A0][0] = 0x3f_0000;
                    c.g[V0][0] &= 0x3f;
                    c.g[A0][0] |= 0xffff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 22);
                    c.g[V1][0] |= c.g[V0][0];
                    c.g[V1][0] &= c.g[A3][0];
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    c.g[V1][0] |= c.g[A2][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                    c.g[V1][0] &= c.g[A0][0];
                    c.g[V1][0] |= c.g[A1][0];
                    o406_sw(c, S2, 0, V1);
                    break 'body;
                }
                // 0x3250a8-side: both failures end in the parser error.
                break 'overflow;
            }
            c.cycles += 3;
            call_ptr(c, 0x0032_1500, 0x0032_50a4)?;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xa0);
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
    })
}

/// Return a temporary register (`a1`, below 0x40) to the parser's free list:
/// `list[count++] = a1` (list at +0x108ec, count at +0x109e8), as both
/// operand registers of `EmitBinaryOp` are.
pub(crate) fn o406_emit_free_reg(c: &mut Cpu) {
    c.g[A0][0] = 0x10000;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = o406_lw(c, A0, 0x9e8);
    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    o406_sw(c, V0, 0x8ec, A1);
    o406_sw(c, A0, 0x9e8, V1);
}

// ------------------------------------------------ InputPreproc_ProcessText

/// `int InputPreproc_ProcessText(InputPreproc *self, Cursor *in, a2, a3)`
/// (0x326150): copy one source line to the output. Leading blanks (space,
/// tab, CR) are written through; a `#` starts a directive, `//` a line
/// comment, `/*` a block comment (each handled by its own function); any
/// other character copies the rest of the line, counting it at its
/// newline. Returns 1 at end of input (when the lookahead char at +0xc is
/// negative), else 0.
pub fn input_preproc_process_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S2, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x30);
        c.g[S4][0] = c.g[A3][0];
        frame.save(c, S6, 0x20);
        c.g[S5][0] = 0xd;
        frame.save(c, S7, 0x10);
        c.g[S6][0] = 9;
        frame.save(c, S1, 0x70);
        c.g[S7][0] = 0x20;
        frame.save_ra(c, 0);
        'epilogue: {
            'to_d8: {
                'to_d4: {
                    // Leading blanks.
                    let mut first = true;
                    loop {
                        if !first {
                            c.cycles += 8;
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0032_6360, 0x0032_61a0)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[S2][0];
                            call_ptr(c, 0x0032_6478, 0x0032_61ac)?;
                            c.g[V0][0] = o406_lw(c, S0, 0xc);
                            let eof = (c.g[V0][0] as i64) < 0;
                            c.g[V0][0] = 0;
                            if eof {
                                break 'epilogue;
                            }
                        }
                        first = false;
                        c.cycles += 3;
                        c.g[S1][0] = o406_lw(c, S0, 0xc);
                        c.g[V0][0] = 0;
                        if c.g[S1][0] == c.g[S7][0] {
                            c.cycles += 1;
                            c.g[V0][0] = 1;
                        } else {
                            c.cycles += 2;
                            if c.g[S1][0] == c.g[S6][0] {
                                c.g[V0][0] = 1;
                            } else {
                                c.cycles += 2;
                                if c.g[S1][0] == c.g[S5][0] {
                                    c.cycles += 1;
                                    c.g[V0][0] = 1;
                                }
                            }
                        }
                        c.cycles += 2;
                        if c.g[V0][0] == 0 {
                            break;
                        }
                        let a = (c.g[S0][0] as u32).wrapping_add(0xc);
                        c.g[A1][0] = c.lb(a);
                        if event_due(c) {
                            deliver_event_here(c, 0x0032_6198);
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x23;
                    let not_hash = c.g[S1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2f;
                    if !not_hash {
                        // `#`: a directive.
                        c.cycles += 10;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[S2][0];
                        call_ptr(c, 0x0032_6478, 0x0032_61f8)?;
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S3][0];
                        c.g[A3][0] = c.g[S4][0];
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0032_66e8, 0x0032_620c)?;
                        c.g[V0][0] = o406_lw(c, S0, 0xc);
                        break 'to_d4;
                    }
                    c.cycles += 2;
                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[S1][0] = 0xa;
                    } else {
                        // `/`: a comment, or a plain slash.
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[S2][0];
                        call_ptr(c, 0x0032_6478, 0x0032_6228)?;
                        c.g[V1][0] = o406_lw(c, S0, 0xc);
                        let not_slash = c.g[V1][0] != c.g[S1][0];
                        c.g[V0][0] = 0x2a;
                        if !not_slash {
                            c.cycles += 7;
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S3][0];
                            c.g[A3][0] = c.g[S4][0];
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0032_65c0, 0x0032_6248)?;
                            c.g[V0][0] = o406_lw(c, S0, 0xc);
                            break 'to_d4;
                        }
                        c.cycles += 2;
                        let not_star = c.g[V1][0] != c.g[V0][0];
                        c.g[A0][0] = c.g[S0][0];
                        if !not_star {
                            c.cycles += 9;
                            c.g[A1][0] = c.g[S2][0];
                            call_ptr(c, 0x0032_6478, 0x0032_6260)?;
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S3][0];
                            c.g[A3][0] = c.g[S4][0];
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0032_64b0, 0x0032_6274)?;
                            c.g[V0][0] = o406_lw(c, S0, 0xc);
                            break 'to_d4;
                        }
                        // A plain `/`: write it, then the rest of the line.
                        c.cycles += 4;
                        c.g[A1][0] = 0x2f;
                        call_ptr(c, 0x0032_6360, 0x0032_6284)?;
                        c.g[S1][0] = 0xa;
                    }
                    // The rest of the line, up to the newline in s1.
                    loop {
                        c.cycles += 3;
                        c.g[V0][0] = o406_lw(c, S0, 0xc);
                        let eof = (c.g[V0][0] as i64) < 0;
                        c.g[V0][0] = !c.g[V0][0];
                        if eof {
                            break 'to_d8;
                        }
                        c.cycles += 6;
                        let a = (c.g[S0][0] as u32).wrapping_add(0xc);
                        c.g[A1][0] = c.lb(a);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0032_6360, 0x0032_62b0)?;
                        c.g[V0][0] = o406_lw(c, S0, 0xc);
                        let more = c.g[V0][0] != c.g[S1][0];
                        c.g[A0][0] = c.g[S0][0];
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x0032_6290);
                            }
                            c.cycles += 2;
                            c.g[A1][0] = c.g[S2][0];
                            call_ptr(c, 0x0032_6478, 0x0032_6298)?;
                            continue;
                        }
                        // Newline: count the line, step past it.
                        c.cycles += 6;
                        c.g[V0][0] = o406_lw(c, S2, 0xc);
                        c.g[A1][0] = c.g[S2][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                        o406_sw(c, S2, 0xc, V0);
                        call_ptr(c, 0x0032_6478, 0x0032_62d0)?;
                        c.g[V0][0] = o406_lw(c, S0, 0xc);
                        break 'to_d4;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = !c.g[V0][0];
            }
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 31);
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// InputParser_EmitUnaryOp (0x003250d8)

/// `InputParser_EmitNode` (0x00324678).
pub(crate) const INPUT_PARSER_EMIT_NODE_CALL: Callee = Callee {
    addr: 0x0032_4678,
    f: ee::s10::InputParser_EmitNode,
};

/// `InputParser_Error` (0x00321500).
pub(crate) const INPUT_PARSER_ERROR_CALL: Callee = Callee {
    addr: 0x0032_1500,
    f: ee::s10::InputParser_Error,
};

/// `int InputParser::EmitUnaryOp(this, node, dst*, code, cap, op, modifier)` (0x3250d8): compile the operand of a unary node, return its source register to the free-register stack, allocate a destination register when `*dst < 0`, then emit one opcode word (op, dst, src, modifier) into the buffer. Returns the number of words used (-1 after an error or when the buffer overflows).
pub fn input_parser_emit_unary_op(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x70 };
        'b_32522c: {
            c.cycles += 21;
            Frame::open(c, 0x70);
            c.g[V0][0] = 0xffffffffffffffff;
            frame.save(c, S0, 0x60);
            frame.save(c, S1, 0x50);
            c.g[S0][0] = c.g[A0][0];
            frame.save(c, S2, 0x40);
            c.g[S1][0] = c.g[T0][0];
            frame.save(c, S3, 0x30);
            c.g[S2][0] = c.g[A3][0];
            frame.save(c, S4, 0x20);
            c.g[S3][0] = c.g[A2][0];
            frame.save_ra(c, 0x10);
            c.g[S4][0] = c.g[T1][0];
            c.g[T1][0] = c.g[T2][0];
            c.g[A1][0] = c.lw(r32(c, A1).wrapping_add(0x4));
            c.sw(r32(c, SP), c.g[V0][0]);
            c.g[A2][0] = c.g[SP][0];
            INPUT_PARSER_EMIT_NODE_CALL.call(c, 0x0032_5120)?;
            c.g[T0][0] = c.g[V0][0];
            let t = (c.g[T0][0] as i64) < 0;
            c.g[V0][0] = 0xffffffffffffffff;
            if t {
                break 'b_32522c;
            } else {
                'b_325164: {
                    c.cycles += 6;
                    c.g[V0][0] = sx(r32(c, T0) << 2);
                    c.g[A1][0] = c.lw(r32(c, SP));
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
                    c.g[V0][0] = (c.g[A1][0] < 0x40u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_sub(r32(c, T0)));
                    if t {
                        break 'b_325164;
                    } else {
                        c.cycles += 8;
                        c.g[A0][0] = 0x10000;
                        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x9e8));
                        c.g[V0][0] = sx(r32(c, V1) << 2);
                        c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x1));
                        c.sw(r32(c, V0).wrapping_add(0x8ec), c.g[A1][0]);
                        c.sw(r32(c, A0).wrapping_add(0x9e8), c.g[V1][0]);
                        break 'b_325164;
                    }
                }
                'b_325220: {
                    'b_3251ac: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(r32(c, S3));
                        let t = (c.g[V0][0] as i64) >= 0;
                        c.g[V0][0] = 0x10000;
                        if t {
                            break 'b_3251ac;
                        } else {
                            'b_325194: {
                                c.cycles += 4;
                                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                                c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x9e8));
                                let t = (c.g[V0][0] as i64) <= 0;
                                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1));
                                if t {
                                    break 'b_325194;
                                } else {
                                    c.cycles += 5;
                                    c.g[V1][0] = sx(r32(c, V0) << 2);
                                    c.sw(r32(c, A0).wrapping_add(0x9e8), c.g[V0][0]);
                                    c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                                    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x8ec));
                                    c.sw(r32(c, S3), c.g[V0][0]);
                                    break 'b_325194;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(r32(c, S3));
                            let t = (c.g[V0][0] as i64) >= 0;
                            c.g[A1][0] = 0x490000;
                            if t {
                                break 'b_3251ac;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.g[S0][0];
                                let t = true;
                                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x1dc0));
                                if t {
                                    break 'b_325220;
                                } else {
                                    break 'b_3251ac;
                                }
                            }
                        }
                    }
                    'b_325214: {
                        c.cycles += 2;
                        let t = (c.g[S1][0] as i64) <= 0;
                        c.g[V0][0] = 0xffffffffffffffc0;
                        if t {
                            break 'b_325214;
                        } else {
                            c.cycles += 24;
                            c.g[V1][0] = c.lw(r32(c, S2));
                            c.g[A0][0] = c.g[S4][0] & 0x3f;
                            c.g[A2][0] = c.lw(r32(c, SP));
                            c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                            c.g[A1][0] = 0xfffffffffffff03f;
                            c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                            c.g[A3][0] = 0xffffffffffc00000;
                            c.sw(r32(c, S2), c.g[V1][0]);
                            c.g[A2][0] = c.g[A2][0] & 0x3ff;
                            c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
                            c.g[A3][0] = c.g[A3][0] | 0xfff;
                            c.g[A0][0] = c.lw(r32(c, S3));
                            c.g[A2][0] = sx(r32(c, A2) << 12);
                            c.g[A1][0] = 0x3f0000;
                            c.g[V0][0] = sx(r32(c, T0).wrapping_add(0x1));
                            c.g[A0][0] = c.g[A0][0] & 0x3f;
                            c.g[A1][0] = c.g[A1][0] | 0xffff;
                            c.g[A0][0] = sx(r32(c, A0) << 6);
                            c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                            c.g[V1][0] = c.g[V1][0] & c.g[A3][0];
                            c.g[V1][0] = c.g[V1][0] | c.g[A2][0];
                            c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
                            let t = true;
                            c.sw(r32(c, S2), c.g[V1][0]);
                            if t {
                                break 'b_32522c;
                            } else {
                                break 'b_325214;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[A1][0] = 0x490000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x1dd8));
                    break 'b_325220;
                }
                c.cycles += 3;
                INPUT_PARSER_ERROR_CALL.call(c, 0x0032_5228)?;
                c.g[V0][0] = 0xffffffffffffffff;
                break 'b_32522c;
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `Node* InputParser_ParseFunction2(Parser* self, int op)` (0x324138): parse
/// the `( a , b )` arguments of a two-argument expression function; with two
/// constant operands fold `min`/`max`/`pow`/`atan2` (ops 21, 20, 22, 23),
/// else build a binary node. Errors free the parsed operands and return 0.
/// Kept as the generated block dispatcher (`pc` is the block address).
pub fn input_parser_parse_function_2(c: &mut Cpu) {
    const INPUT_PARSER_ALLOC_NODE: Callee = Callee {
        addr: 0x003215f8,
        f: ee::s10::f_003215f8,
    };
    const INPUT_PARSER_ERROR: Callee = Callee {
        addr: 0x00321500,
        f: ee::s10::f_00321500,
    };
    const INPUT_PARSER_FREE_NODE: Callee = Callee {
        addr: 0x00321638,
        f: ee::s10::f_00321638,
    };
    const INPUT_PARSER_NEXT_TOKEN: Callee = Callee {
        addr: 0x00321af0,
        f: ee::s10::f_00321af0,
    };
    const INPUT_PARSER_PARSE_CONDITIONAL: Callee = Callee {
        addr: 0x00323268,
        f: ee::s10::f_00323268,
    };
    const MATH_ATAN: Callee = Callee {
        addr: 0x0031c228,
        f: ee::s10::f_0031c228,
    };
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00324138 => {
                    c.cycles += 14;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                    c.g[V0][0] = 0x9;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x58));
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[S3][0] = c.g[A1][0];
                    if t {
                        pc = 0x00324184;
                        continue;
                    }
                    pc = 0x00324170;
                    continue;
                }
                0x00324170 => {
                    c.cycles += 3;
                    c.g[A1][0] = 0x490000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1e68));
                    INPUT_PARSER_ERROR.call(c, 0x0032417c)?;
                    pc = 0x0032417c;
                    continue;
                }
                0x0032417c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0032436c;
                        continue;
                    }
                    pc = 0x00324184;
                    continue;
                }
                0x00324184 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    INPUT_PARSER_NEXT_TOKEN.call(c, 0x0032418c)?;
                    c.g[A0][0] = c.g[S0][0];
                    INPUT_PARSER_PARSE_CONDITIONAL.call(c, 0x00324194)?;
                    c.g[S1][0] = c.g[V0][0];
                    let t = c.g[S1][0] == 0u64;
                    c.g[V0][0] = 0x8;
                    if t {
                        if back_edge_unwinds(c, 0x0032417c) {
                            return Ok(());
                        }
                        pc = 0x0032417c;
                        continue;
                    }
                    pc = 0x003241a0;
                    continue;
                }
                0x003241a0 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A1][0] = 0x490000;
                    if t {
                        pc = 0x003241c4;
                        continue;
                    }
                    pc = 0x003241ac;
                    continue;
                }
                0x003241ac => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1e48));
                    INPUT_PARSER_ERROR.call(c, 0x003241b8)?;
                    c.g[A0][0] = c.g[S0][0];
                    let t = true;
                    c.g[A1][0] = c.g[S1][0];
                    if t {
                        pc = 0x0032420c;
                        continue;
                    }
                    pc = 0x003241c4;
                    continue;
                }
                0x003241c4 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    INPUT_PARSER_NEXT_TOKEN.call(c, 0x003241cc)?;
                    c.g[A0][0] = c.g[S0][0];
                    INPUT_PARSER_PARSE_CONDITIONAL.call(c, 0x003241d4)?;
                    c.g[S2][0] = c.g[V0][0];
                    let t = c.g[S2][0] == 0u64;
                    c.g[V0][0] = 0xa;
                    if t {
                        if back_edge_unwinds(c, 0x0032417c) {
                            return Ok(());
                        }
                        pc = 0x0032417c;
                        continue;
                    }
                    pc = 0x003241e0;
                    continue;
                }
                0x003241e0 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A1][0] = 0x490000;
                    if t {
                        pc = 0x0032421c;
                        continue;
                    }
                    pc = 0x003241ec;
                    continue;
                }
                0x003241ec => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1f48));
                    INPUT_PARSER_ERROR.call(c, 0x003241f8)?;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S0][0];
                    INPUT_PARSER_FREE_NODE.call(c, 0x00324204)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S2][0];
                    pc = 0x0032420c;
                    continue;
                }
                0x0032420c => {
                    c.cycles += 4;
                    INPUT_PARSER_FREE_NODE.call(c, 0x00324214)?;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0032436c;
                        continue;
                    }
                    pc = 0x0032421c;
                    continue;
                }
                0x0032421c => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    INPUT_PARSER_NEXT_TOKEN.call(c, 0x00324224)?;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = 0x4;
                    let t = c.g[V1][0] != c.g[V0][0];

                    if t {
                        pc = 0x00324354;
                        continue;
                    }
                    pc = 0x00324234;
                    continue;
                }
                0x00324234 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[V0][0] = 0x15;
                    if t {
                        pc = 0x00324354;
                        continue;
                    }
                    pc = 0x00324240;
                    continue;
                }
                0x00324240 => {
                    c.cycles += 2;
                    let t = c.g[S3][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[S3][0] as i64) < 22i64) as u64;
                    if t {
                        pc = 0x0032427c;
                        continue;
                    }
                    pc = 0x00324248;
                    continue;
                }
                0x00324248 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x14;
                    if t {
                        pc = 0x00324260;
                        continue;
                    }
                    pc = 0x00324250;
                    continue;
                }
                0x00324250 => {
                    c.cycles += 2;
                    let t = c.g[S3][0] == c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x0032428c;
                        continue;
                    }
                    pc = 0x00324258;
                    continue;
                }
                0x00324258 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x00324344;
                        continue;
                    }
                    pc = 0x00324260;
                    continue;
                }
                0x00324260 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x16;
                    let t = c.g[S3][0] == c.g[V0][0];
                    c.g[V0][0] = 0x17;
                    if t {
                        pc = 0x0032429c;
                        continue;
                    }
                    pc = 0x0032426c;
                    continue;
                }
                0x0032426c => {
                    c.cycles += 2;
                    let t = c.g[S3][0] == c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x003242b8;
                        continue;
                    }
                    pc = 0x00324274;
                    continue;
                }
                0x00324274 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x00324344;
                        continue;
                    }
                    pc = 0x0032427c;
                    continue;
                }
                0x0032427c => {
                    c.cycles += 4;
                    c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x4));
                    c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x4));
                    let t = true;
                    c.min_s(0, 0, 1);
                    if t {
                        pc = 0x0032433c;
                        continue;
                    }
                    pc = 0x0032428c;
                    continue;
                }
                0x0032428c => {
                    c.cycles += 4;
                    c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x4));
                    c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x4));
                    let t = true;
                    c.max_s(0, 0, 1);
                    if t {
                        pc = 0x0032433c;
                        continue;
                    }
                    pc = 0x0032429c;
                    continue;
                }
                0x0032429c => {
                    c.cycles += 7;
                    c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x4));
                    c.lwc1(20, (c.g[S2][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x0040da10, 0x003242a8)?;
                    c.mul_s(12, 0, 20);
                    call_ptr(c, 0x0040d758, 0x003242b0)?;
                    let t = true;
                    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00324340;
                        continue;
                    }
                    pc = 0x003242b8;
                    continue;
                }
                0x003242b8 => {
                    c.cycles += 6;
                    c.lwc1(20, (c.g[S2][0] as u32).wrapping_add(0x4));
                    c.mtc1(0, 22);
                    c.c_eq_s(20, 22);
                    let t = !c.fcc();
                    c.lwc1(21, (c.g[S1][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x003242fc;
                        continue;
                    }
                    pc = 0x003242d0;
                    continue;
                }
                0x003242d0 => {
                    c.cycles += 4;
                    c.c_eq_s(21, 22);
                    let t = c.fcc();
                    c.mov_s(0, 21);
                    if t {
                        pc = 0x0032433c;
                        continue;
                    }
                    pc = 0x003242e0;
                    continue;
                }
                0x003242e0 => {
                    c.cycles += 4;
                    c.c_le_s(22, 21);
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2e3c));

                    if !c.fcc() {
                        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                        pc = 0x00324340;
                        continue;
                    }
                    pc = 0x003242f0;
                    continue;
                }
                0x003242f0 => {
                    c.cycles += 3;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2e38));
                    let t = true;
                    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00324340;
                        continue;
                    }
                    pc = 0x003242fc;
                    continue;
                }
                0x003242fc => {
                    c.cycles += 7;
                    c.div_s(12, 21, 20);
                    MATH_ATAN.call(c, 0x00324308)?;
                    c.c_lt_s(20, 22);
                    let t = !c.fcc();
                    c.mov_s(12, 0);
                    if t {
                        pc = 0x00324338;
                        continue;
                    }
                    pc = 0x00324318;
                    continue;
                }
                0x00324318 => {
                    c.cycles += 3;
                    c.c_lt_s(22, 21);
                    let t = !c.fcc();

                    if t {
                        pc = 0x00324330;
                        continue;
                    }
                    pc = 0x00324324;
                    continue;
                }
                0x00324324 => {
                    c.cycles += 3;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2e34));
                    let t = true;
                    c.add_s(12, 12, 0);
                    if t {
                        pc = 0x00324338;
                        continue;
                    }
                    pc = 0x00324330;
                    continue;
                }
                0x00324330 => {
                    c.cycles += 2;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2e30));
                    c.sub_s(12, 12, 0);
                    pc = 0x00324338;
                    continue;
                }
                0x00324338 => {
                    c.cycles += 1;
                    c.mov_s(0, 12);
                    pc = 0x0032433c;
                    continue;
                }
                0x0032433c => {
                    c.cycles += 1;
                    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                    pc = 0x00324340;
                    continue;
                }
                0x00324340 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S0][0];
                    pc = 0x00324344;
                    continue;
                }
                0x00324344 => {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S2][0];
                    INPUT_PARSER_FREE_NODE.call(c, 0x0032434c)?;
                    let t = true;
                    c.g[V0][0] = c.g[S1][0];
                    if t {
                        pc = 0x0032436c;
                        continue;
                    }
                    pc = 0x00324354;
                    continue;
                }
                0x00324354 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    INPUT_PARSER_ALLOC_NODE.call(c, 0x0032435c)?;
                    c.g[V1][0] = c.g[V0][0];
                    c.sw(c.g[V1][0] as u32, c.g[S3][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[S2][0]);
                    pc = 0x0032436c;
                    continue;
                }
                0x0032436c => {
                    c.cycles += 10;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x58));
                    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00324138, 0x00324394);
                    return Ok(());
                }
            }
        }
    });
}

/// `Node* InputParser::ParseFunction1(Parser* self, int op)` (0x324398): parse the
/// `( expr )` argument of a one-argument expression function; a constant
/// operand is folded in place (ops 0x18..0x21: atan, sin, cos, two helper
/// calls, floor, ceil, abs, sqrt, wrap to a period), otherwise a unary node
/// {op, operand} is built. A missing `(` or `)` reports the error and returns
/// 0 (freeing the operand). Kept as the generated block dispatcher (`pc` is the
/// block address), so resume entries work.
pub fn input_parser_parse_function1(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00324398 => {
                    c.cycles += 10;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.g[V0][0] = 0x9;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x98));
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[S2][0] = c.g[A1][0];
                    if t {
                        pc = 0x003243d4;
                        continue;
                    }
                    pc = 0x003243c0;
                    continue;
                }
                0x003243c0 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0x490000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1e68));
                    Callee {
                        addr: 0x00321500,
                        f: ee::s10::InputParser_Error,
                    }
                    .call(c, 0x003243cc)?;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00324590;
                        continue;
                    }
                    pc = 0x003243d4;
                    continue;
                }
                0x003243d4 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x00321af0,
                        f: ee::s10::InputParser_NextToken,
                    }
                    .call(c, 0x003243dc)?;
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x00323268,
                        f: ee::s10::InputParser_ParseConditional,
                    }
                    .call(c, 0x003243e4)?;
                    c.g[S0][0] = c.g[V0][0];

                    if c.g[S0][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x98));
                        pc = 0x003243f8;
                        continue;
                    }
                    pc = 0x003243f0;
                    continue;
                }
                0x003243f0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00324590;
                        continue;
                    }
                    pc = 0x003243f8;
                    continue;
                }
                0x003243f8 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xa;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A1][0] = 0x490000;
                    if t {
                        pc = 0x00324424;
                        continue;
                    }
                    pc = 0x00324404;
                    continue;
                }
                0x00324404 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1f48));
                    Callee {
                        addr: 0x00321500,
                        f: ee::s10::InputParser_Error,
                    }
                    .call(c, 0x00324410)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00321638,
                        f: ee::s10::InputParser_FreeNode,
                    }
                    .call(c, 0x0032441c)?;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00324590;
                        continue;
                    }
                    pc = 0x00324424;
                    continue;
                }
                0x00324424 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x00321af0,
                        f: ee::s10::InputParser_NextToken,
                    }
                    .call(c, 0x0032442c)?;
                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V0][0] = 0x4;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x18));
                    if t {
                        pc = 0x0032457c;
                        continue;
                    }
                    pc = 0x0032443c;
                    continue;
                }
                0x0032443c => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x490000;
                    if t {
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x00324448;
                    continue;
                }
                0x00324448 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1e20));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00324460 => {
                    c.cycles += 4;
                    c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x4));
                    Callee {
                        addr: 0x0031c228,
                        f: ee::s10::Math_Atan,
                    }
                    .call(c, 0x00324468)?;
                    let t = true;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x00324470;
                    continue;
                }
                0x00324470 => {
                    c.cycles += 4;
                    c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x4));
                    Callee {
                        addr: 0x0031bf60,
                        f: ee::s10::Math_Sin,
                    }
                    .call(c, 0x00324478)?;
                    let t = true;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x00324480;
                    continue;
                }
                0x00324480 => {
                    c.cycles += 4;
                    c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x4));
                    Callee {
                        addr: 0x0031c040,
                        f: ee::s10::Math_Cos,
                    }
                    .call(c, 0x00324488)?;
                    let t = true;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x00324490;
                    continue;
                }
                0x00324490 => {
                    c.cycles += 4;
                    c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x0040d758, 0x00324498)?;
                    let t = true;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x003244a0;
                    continue;
                }
                0x003244a0 => {
                    c.cycles += 4;
                    c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x0040da10, 0x003244a8)?;
                    let t = true;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x003244b0;
                    continue;
                }
                0x003244b0 => {
                    c.cycles += 7;
                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    c.cvt_w_s(1, 0);
                    c.cvt_s_w(1, 1);
                    c.c_lt_s(0, 1);

                    if !c.fcc() {
                        c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x4));
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x003244cc;
                    continue;
                }
                0x003244cc => {
                    c.cycles += 4;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    let t = true;
                    c.sub_s(1, 1, 0);
                    if t {
                        pc = 0x00324504;
                        continue;
                    }
                    pc = 0x003244dc;
                    continue;
                }
                0x003244dc => {
                    c.cycles += 7;
                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    c.cvt_w_s(1, 0);
                    c.cvt_s_w(1, 1);
                    c.c_lt_s(1, 0);

                    if !c.fcc() {
                        c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x4));
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x003244f8;
                    continue;
                }
                0x003244f8 => {
                    c.cycles += 3;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.add_s(1, 1, 0);
                    pc = 0x00324504;
                    continue;
                }
                0x00324504 => {
                    c.cycles += 2;
                    let t = true;
                    c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x0032450c;
                    continue;
                }
                0x0032450c => {
                    c.cycles += 4;
                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    c.abs_s(0, 0);
                    let t = true;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x0032451c;
                    continue;
                }
                0x0032451c => {
                    c.cycles += 4;
                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    c.sqrt_s(0, 0);
                    let t = true;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x0032452c;
                    continue;
                }
                0x0032452c => {
                    c.cycles += 11;
                    c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x4));
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2e2c));
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 1);
                    c.mul_s(0, 2, 0);
                    c.add_s(0, 0, 1);
                    c.cvt_w_s(1, 0);
                    c.cvt_s_w(1, 1);
                    c.c_lt_s(0, 1);
                    let t = !c.fcc();

                    if t {
                        pc = 0x00324564;
                        continue;
                    }
                    pc = 0x00324558;
                    continue;
                }
                0x00324558 => {
                    c.cycles += 3;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.sub_s(1, 1, 0);
                    pc = 0x00324564;
                    continue;
                }
                0x00324564 => {
                    c.cycles += 4;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2e28));
                    c.mul_s(0, 1, 0);
                    c.sub_s(0, 2, 0);
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    pc = 0x00324574;
                    continue;
                }
                0x00324574 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.g[S0][0];
                    if t {
                        pc = 0x00324590;
                        continue;
                    }
                    pc = 0x0032457c;
                    continue;
                }
                0x0032457c => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x003215f8,
                        f: ee::s10::InputParser_AllocNode,
                    }
                    .call(c, 0x00324584)?;
                    c.g[V1][0] = c.g[V0][0];
                    c.sw(c.g[V1][0] as u32, c.g[S2][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                    pc = 0x00324590;
                    continue;
                }
                0x00324590 => {
                    c.cycles += 6;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00324398, 0x003245a8);
                    return Ok(());
                }
            }
        }
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;

    use crate::difftest::fakes;

    use crate::difftest::fakes::machine;

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::slot;

    pub(crate) use crate::game::ai::ai::tests::LOAD_B0;
    pub(crate) use crate::game::ai::aianimeventmap::tests::alloc_setup;

    pub(crate) use crate::game::be::beintplayer::tests::w8;
    pub(crate) use crate::game::bx::bxstringctor::tests::alloc_base;

    pub(crate) use crate::game::bx::memblockman::tests::fake_slot;
    pub(crate) use crate::game::camera::trigger::cameratriggerlist::tests::wb;
    pub(crate) use crate::game::fe::debugfe::tests::w as w_2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;

    pub(crate) use crate::game::fe::feasyncfile::tests::env;

    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_15 as SEEDS_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_7;

    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_6;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::SEEDS as SEEDS_4;
    pub(crate) use crate::game::intersect::worldsphtree::tests::SEEDS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_init;

    pub(crate) use crate::game::sound::bankmanager::tests::mixed;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::visualfx::worldlightman::tests::dtor_case;
    pub(crate) use crate::game::world::worldview::tests::pick;
    pub(crate) use crate::game::world::worldview::tests::word;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn input_parser_pad_channel_operand_matches() {
        check_lohi(
            ssx3_ee::s10::f_00321ad8,
            input_parser_pad_channel_operand,
            200,
            &|c, seed| {
                let mut r = Rng(0x1007 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (r.next_u32() % 8) as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | (r.next_u32() as u64);
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn input_vm_init_matches() {
        check(ssx3_ee::s10::f_00325250, input_vm_init, 200, &|c, seed| {
            let mut r = Rng(0x1008 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA as u64) | ((r.next_u32() as u64) << 32);
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn input_vm_get_reg_ptr_matches() {
        check(
            ssx3_ee::s10::f_003252e8,
            input_vm_get_reg_ptr,
            200,
            &|c, seed| {
                let mut r = Rng(0x1009 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 16) as u64;
            },
        );
    }

    #[test]
    fn input_vm_read_builtin_matches() {
        check(
            ssx3_ee::s10::f_00325430,
            input_vm_read_builtin,
            200,
            &|c, seed| {
                let mut r = Rng(0x100a ^ seed as u64);
                scramble(c, &mut r);
                let index = if seed % 3 == 0 { 0 } else { (seed % 16) as u32 };
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = index as u64;
                let base = DATA + index * 4;
                c.bus.write32(base + 0x108, r.edge_f32());
            },
        );
    }

    #[test]
    fn input_preproc_next_char_matches() {
        check(
            ssx3_ee::s10::f_00326478,
            input_preproc_next_char,
            200,
            &|c, seed| {
                let mut r = Rng(0x100b ^ seed as u64);
                scramble(c, &mut r);
                let out = DATA;
                let st = DATA + 0x100;
                let start = DATA + 0x400;
                let len = (r.next_u32() % 16) as u32;
                c.g[A0][0] = out as u64;
                c.g[A1][0] = st as u64;
                c.bus.write32(st, start);
                c.bus.write32(st + 4, len);
                let ptr = if seed % 3 == 0 {
                    start.wrapping_add(len) // cursor at end -> -1 branch
                } else {
                    start // cursor inside -> byte branch
                };
                c.bus.write32(st + 8, ptr);
                c.bus.write8(ptr, (r.next_u32() & 0xff) as u8);
            },
        );
    }

    #[test]
    fn pad_queue_flush_matches() {
        check_lohi(
            ssx3_ee::s10::f_00326c60,
            pad_queue_flush,
            200,
            &|c, seed| {
                let mut r = Rng(0x100c ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x2ee4, r.next_u32());
                c.bus.write32(DATA + 0x2ee0, r.next_u32());
            },
        );
    }

    #[test]
    fn ai_fwd_diff_cache_ctor_matches() {
        check(
            ssx3_ee::s10::f_00327810,
            ai_fwd_diff_cache_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x100d ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..4 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ai_fwd_diff_cache_dtor_matches() {
        check(
            ee::s10::f_00327828,
            ai_fwd_diff_cache_dtor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x7828, 0);
                alloc_setup(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                // Cover null and non-null for both owned blocks.
                c.bus
                    .write32(DATA, if seed % 3 == 0 { 0 } else { DATA + 0x100 });
                c.bus
                    .write32(DATA + 8, if seed % 5 == 0 { 0 } else { DATA + 0x200 });
            },
        );
    }

    #[test]
    fn input_vm_read_operand_matches() {
        check(
            ssx3_ee::s10::f_00325260__generated,
            input_vm_read_operand,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x4002, 0);
                c.g[A0][0] = sx(DATA);
                let id = match seed % 4 {
                    0 => r.next_u32() % 0x40,          // a script variable
                    1 => 0x40 + r.next_u32() % 0x20,   // a builtin
                    2 => 0x60 + r.next_u32() % 0x39c,  // a controller channel
                    _ => 0x3fc + r.next_u32() % 0x100, // past the end
                };
                c.g[A1][0] = sx(id);
                c.bus.write32(DATA + 4, DATA + 0x400); // pad state
                c.bus.write32(DATA + 0x400, 8); // channel count
                for k in 0..0x90 {
                    c.bus.write32(DATA + 8 + 4 * k, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn input_parser_builtin_operand_matches() {
        check(
            ee::s10::f_00321ad0__generated,
            input_parser_builtin_operand,
            SEEDS_3,
            &|c, seed| {
                let mut r = rng(0x0032_1ad0, seed);
                scramble(c, &mut r);
                let v = r.next_u32();
                c.g[A1][0] = mixed(&mut r, seed, v);
            },
        );
    }

    #[test]
    fn pad_device_dtor_matches() {
        dtor_case(ee::s10::f_003277d0__generated, pad_device_dtor, 0x3277d0);
    }

    #[test]
    fn pad_queue_init_matches() {
        check(
            ssx3_ee::s10::f_00326a28__generated,
            pad_queue_init,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x326a_28, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn input_preproc_emit_line_marker_matches() {
        check(
            ee::s10::f_00326430__generated,
            input_preproc_emit_line_marker,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32_6430, 0);
                // The line-marker format lives in the game's data image:
                // `# %d "%s"\n` as UTF-16.
                let fmt = [
                    0x23u16, 0x20, 0x25, 0x64, 0x20, 0x22, 0x25, 0x73, 0x22, 0x0a, 0,
                ];
                for (i, w) in fmt.iter().enumerate() {
                    c.bus.write16(0x0048_e3c8 + 2 * i as u32, *w);
                }
                let text = DATA + 0x2000;
                for (i, b) in b"hello world\0".iter().enumerate() {
                    c.bus.write8(text + i as u32, *b);
                }
                let p = DATA + 0x1000;
                c.bus.write32(p, 0);
                c.bus.write32(p + 4, 0x1000);
                c.bus.write32(p + 8, DATA + 0x4000);
                c.g[A0][0] = sx(p);
                c.g[A1][0] = (r.next_u32() % 1000) as u64;
                c.g[A2][0] = sx(text);
            },
        );
    }

    #[test]
    fn input_preproc_write_string_matches() {
        check(
            ssx3_ee::s10::f_00326308__generated,
            input_preproc_write_string,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x6308, 0);
                heap_init(c);
                let inp = DATA + 0x400;
                let s = DATA + 0x500;
                let n = (seed % 8) as u32;
                for i in 0..n {
                    c.bus.write8(s + i, b'A' + (i as u8 % 26));
                }
                c.bus.write8(s + n, 0);
                if seed % 3 == 0 {
                    // Empty buffer: the write grows it (operator_new_named).
                    c.bus.write32(inp, 0);
                    c.bus.write32(inp + 4, 0);
                    c.bus.write32(inp + 8, 0);
                } else {
                    let count = r.next_u32() % 4;
                    c.bus.write32(inp, count);
                    c.bus.write32(inp + 4, count + 1 + r.next_u32() % 4);
                    c.bus.write32(inp + 8, DATA + 0x600);
                }
                c.g[A0][0] = inp as u64;
                c.g[A1][0] = s as u64;
                // The loop is entered at its own address (it re-enters at
                // 0x326328 after a thread switch).
                c.pc = 0x0032_6308;
            },
        );
    }

    #[test]
    fn ps2pad_set_actuator_matches() {
        check(
            ssx3_ee::s10::f_00327740__generated,
            ps2pad_set_actuator,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x327740, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 3 == 0 { 1 } else { (seed % 5) as u64 };
                c.f[12] = pick(
                    &mut r,
                    &[
                        0.0, -0.0, 0.244, 0.5, 0.899, 0.9, 1.0, -1.0, 1.0e9, -1.0e9, 1.0e-30,
                    ],
                );
                for i in 0..0x40 {
                    word(c, DATA + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ps2_pad_ctor_matches() {
        check(
            ssx3_ee::s10::f_00326df0__generated,
            ps2_pad_ctor,
            60,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x326df0, 0);
                fill(c, &mut r, OBJ, 0x20);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.g[A2][0] = (seed % 4) as u64;
            },
        );
    }

    #[test]
    fn ps2_pad_dtor_matches() {
        check(
            ssx3_ee::s10::f_00326e50__generated,
            ps2_pad_dtor,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x326e50, 0);
                w_2(c, OBJ + 0x4, seed.wrapping_mul(7));
                w_2(c, OBJ + 0x8, seed.wrapping_mul(11));
                w_2(c, OBJ, 0);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    /// The parser object (`+8`: 64 virtual registers).
    pub(crate) const PARSER: u32 = DATA;

    /// The opcode stream.
    pub(crate) const STREAM: u32 = DATA + 0x1000;

    /// The object the original receives as its second argument (stored at
    /// `parser + 4`; the pad-channel operands read through it).
    pub(crate) const PAD: u32 = DATA + 0x2000;

    /// Eight instructions whose opcodes are chosen by the seed (every opcode
    /// lands in every position over 33 consecutive seeds), followed by
    /// opcode 0x20 padding. Skips are one word, so a run stays inside.
    pub(crate) fn put_code(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
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
        check(
            ee::s10::f_00325450__generated,
            input_vm_execute,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x3254_5000 ^ (seed as u64) << 9);
                scramble(c, &mut r);
                // The game's data: the `gp` tables the float opcodes and the maths
                // callees read, and the opcode dispatch table.
                elf_image(c);
                c.pc = 0x0032_5450;
                c.g[GP][0] = sx(GAME_GP);
                for i in 0..0x40u32 {
                    let v = if i % 2 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(1000.0)
                    };
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
            },
        );
    }

    /// An empty guest function (returns immediately, 2 cycles).
    pub(crate) const EMPTY: u32 = 0x0038_dc60;

    #[test]
    fn input_preproc_skip_line_matches() {
        check(
            ssx3_ee::s10::f_00326678__generated,
            input_preproc_skip_line,
            SEEDS_6,
            &|c, seed| {
                let _ = machine(c, seed, 0x326678, 0);
                let p_4 = DATA + 0x1000;
                let s = DATA + 0x2000;
                let buf = DATA + 0x3000;
                c.g[A0][0] = p_4 as u64;
                c.g[A1][0] = s as u64;
                // A 8-byte stream with a newline at position seed % 8.
                w32(c, s, 8);
                w32(c, s + 4, 0);
                w32(c, s + 8, buf);
                w32(c, s + 0xc, 0);
                let nl = (seed % 8) as u32;
                w8(c, p_4 + 0xc, b'x');
                for i in 0..8u32 {
                    w8(c, buf + i, if i == nl { 0x0a } else { b'a' });
                }
            },
        );
    }

    #[test]
    fn pad_device_stop_actuators_matches() {
        check(
            ssx3_ee::s10::f_00326cf0__generated,
            pad_device_stop_actuators,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x326cf0, 0);
                let d = DATA + 0x1000;
                let vt = DATA + 0x5000;
                c.g[A0][0] = d as u64;
                c.pc = 0x0032_6cf0;
                w32(c, d, vt);
                slot(c, vt, 0x18, 0, LOAD_B0);
                slot(c, vt, 0x20, 0, EMPTY);
                w32(c, d + 0xb0, (seed % 4) as u32);
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn pad_queue_shutdown_matches() {
        check(
            ee::s10::f_00326a68__generated,
            pad_queue_shutdown,
            SEEDS_7,
            &|c, seed| {
                machine(c, seed, 0x326a68, 6);
                be_library(c);
                world(c);
                let q = DATA;
                c.g[A0][0] = q as u64;
                let n = seed % 4;
                w(c, q + 0x2ee8, n);
                for i in 0..n {
                    let e = q + 0x2f00 + i * 0x40;
                    let ptr = if (seed + i) % 3 == 0 { 0 } else { e };
                    w(c, q + 0x2eec + i * 4, ptr);
                    fake_slot(c, e, DATA + 0x3000, 8);
                }
            },
        );
    }

    #[test]
    fn input_parser_find_identifier_matches() {
        check(
            ee::s10::f_00321a40__generated,
            input_parser_find_identifier,
            150,
            &|c, seed| {
                let _ = base(c, seed, 0x32_1a40);
                globals(c);
                const TABLE: u32 = DATA + 0x80e8;
                w(c, DATA + 0x8000 + 0xe4, if seed % 3 == 0 { 0 } else { 3 });
                for i in 0..3u32 {
                    let e = TABLE + 0x44 * i;
                    for (j, b) in [
                        b"alpha\0".as_slice(),
                        b"beta\0".as_slice(),
                        b"gamma\0".as_slice(),
                    ][i as usize]
                        .iter()
                        .enumerate()
                    {
                        c.bus.write8(e + j as u32, *b);
                    }
                }
                // The name looked up.
                let name = b"beta\0";
                for (j, b) in name.iter().enumerate() {
                    c.bus.write8(DATA + 0x200 + j as u32, *b);
                }
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
                if seed % 3 == 1 {
                    c.bus.write8(DATA + 0x200, b'B');
                }
            },
        );
    }

    #[test]
    fn ps2_pad_create_matches() {
        check(
            ee::s10::f_00326d60__generated,
            ps2_pad_create,
            SEEDS_8,
            &|c, seed| {
                machine(c, seed, 0x326d, 0);
                env(c);
                c.g[A0][0] = sx(DATA + 0x800);
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }

    // ------------------------------------------------------- input and bolts

    pub(crate) fn wstr(c: &mut Cpu, a: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(a + i as u32, *b);
        }
        c.bus.write8(a + s.len() as u32, 0);
    }

    #[test]
    fn input_parser_lookup_modifier_matches() {
        const NAME: u32 = DATA + 0x1000;
        check(
            ee::s10::f_003217d0__generated,
            input_parser_lookup_modifier,
            12,
            &|c, seed| {
                let _ = base_2(c, seed, 0x0032_17d0, 0);
                let globals = [
                    0x004a_3ef8u32,
                    0x004a_3f00,
                    0x004a_3f08,
                    0x0048_ddd8,
                    0x004a_3f10,
                ];
                for (i, g) in globals.iter().enumerate() {
                    let name = [b'a' + i as u8];
                    wstr(c, *g, &name);
                }
                let pick = seed % 6;
                if pick < 5 {
                    wstr(c, NAME, &[b'a' + pick as u8]);
                } else {
                    wstr(c, NAME, b"zz");
                }
                c.g[A1][0] = NAME as u64;
            },
        );
    }

    // ------------------------------------------------------ input preproc

    #[test]
    fn input_preproc_skip_line_comment_matches() {
        check(
            ee::s10::f_003265c0__generated,
            input_preproc_skip_line_comment,
            SEEDS_9,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x3265, 0);
                let pre = DATA;
                let cur = DATA + 0x100;
                if seed % 3 == 0 {
                    // End of input: the reader reports -1.
                    w(c, pre + 0xc, 0);
                    w(c, cur, DATA + 0x400);
                    w(c, cur + 4, 0);
                    w(c, cur + 8, DATA + 0x400);
                } else {
                    // Newline: the comment is written and the cursor moves.
                    w(c, pre + 0xc, 0xa);
                    w(c, cur, DATA + 0x400);
                    w(c, cur + 4, 4);
                    w(c, cur + 8, DATA + 0x400);
                    wb(c, DATA + 0x400, 0xa);
                }
                w(c, pre, r.next_u32());
                c.g[A0][0] = pre as u64;
                c.g[A1][0] = cur as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn input_parser_compile_identifier_matches() {
        check(
            ee::s10::f_003245a8__generated,
            input_parser_compile_identifier,
            40,
            &|c, seed| {
                let _r = base_4(c, seed, 0x3245);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = (DATA + 0x2000) as u64;
                c.g[A3][0] = (seed % 8) as u64;
                c.g[T0][0] = (seed % 4) as u64;
            },
        );
    }

    #[test]
    fn input_preproc_write_matches() {
        check(
            ee::s10::f_00326360__generated,
            input_preproc_write,
            60,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x3263);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = seed as u64;
                let cap = if seed % 4 == 0 { 8 } else { 0x100 };
                let pos = if seed % 4 == 0 {
                    cap
                } else {
                    (seed % 8) as u32
                };
                c.bus.write32(DATA, pos);
                c.bus.write32(DATA + 4, cap);
                c.bus
                    .write32(DATA + 8, if seed % 8 == 0 { 0 } else { DATA + 0x2000 });
                for i in 0..0x20 {
                    c.bus.write8(DATA + 0x2000 + i, 0);
                }
            },
        );
    }

    // --------------------------------------------------------- PS2Pad_Update

    /// `PS2Pad_Update` against the generated body, with the real libpad
    /// callees on their short paths: the pad's libpad port entry points at
    /// a test DMA frame (state, request state, mode bytes and pad data
    /// chosen per seed) or is closed (every call returns 0); the SIF RPC
    /// packet pool is empty so `scePadPortOpen`, `scePadSetMainMode` and
    /// `scePadSetActAlign` fail at once. Every phase 0..=10 plus junk
    /// phases, every state/req-state/id/ext answer, retry counts at the
    /// limit, pressure and digital buttons, analog sticks on both sides of
    /// the dead zone (and off), changed / unchanged motor values.
    #[test]
    fn ps2_pad_update_matches() {
        use super::ps2pad::*;
        use crate::difftest::{check, fill, put_words, scramble, Rng, DATA, GAME_GP};

        const OUT: u32 = DATA + 0x1000;
        /// The libpad frame pair (2 x 0x80) of the pad's port.
        const DMA: u32 = DATA + 0x2000;
        /// libpad's port table: entry `port * 0x70 + slot * 0x1c` holds the
        /// DMA buffer (+0) and the opened flag (+0x10).
        const LIBPAD_PORTS: u32 = 0x0052_6250;
        const SIF_RPC_PACKETS: u32 = 0x0052_d680;
        const LIBPAD_PRINTF: u32 = 0x0045_15cc;
        // DMA frame fields
        const F_FRAME: u32 = 0x58;
        const F_LENGTH: u32 = 0x60;
        const F_MODE_TABLE: u32 = 0x50;
        const F_MODE_CONFIG: u32 = 0x64;
        const F_MODE_CUR_ID: u32 = 0x65;
        const F_MODE_CUR_OFFS: u32 = 0x69;
        const F_STATE: u32 = 0x70;
        const F_REQ_STATE: u32 = 0x71;
        const F_OK: u32 = 0x72;

        check(
            ee::s10::f_00326eb0__generated,
            ps2_pad_update,
            400,
            &|c, seed| {
                let mut r = Rng(0x0326_eb0d ^ (seed as u64) << 4);
                scramble(c, &mut r);
                let pick = |r: &mut Rng, xs: &[u32]| xs[(r.next_u32() as usize) % xs.len()];
                c.g[GP][0] = GAME_GP as u64;
                put_words(c, JUMP_TABLE, &CASES);
                let scale = if seed % 5 == 0 {
                    r.f32(2.0)
                } else {
                    0x3b80_8081
                };
                put_words(c, GAME_GP - PRESSURE_SCALE_GP, &[scale]);
                put_words(c, GAME_GP - STICK_SCALE_GP, &[scale ^ (seed & 1)]);
                put_words(c, LIBPAD_PRINTF, &[0]);
                put_words(c, SIF_RPC_PACKETS, &[0, 0, 0, 0]);

                // the pad (DMA buffer at +0x80 misaligned for some seeds)
                let pad = DATA + if seed % 7 == 3 { 0x10 } else { 0 };
                let port = r.next_u32() % 2;
                let slot = r.next_u32() % 2;
                fill(c, &mut r, pad, 0x30);
                c.bus.write32(pad + PORT, port);
                c.bus.write32(pad + SLOT, slot);
                let phase = match seed % 13 {
                    11 => r.next_u32(),
                    12 => 10,
                    p => p,
                };
                c.bus.write32(pad + PHASE, phase);
                let retry = pick(&mut r, &[0, 3, 4, 4, 7]);
                c.bus.write32(pad + RETRY, retry);
                for f in [MODE_SET, PRESSURE, ANALOG, ACTUATOR] {
                    let v = pick(&mut r, &[0, 1, 1]);
                    c.bus.write32(pad + f, v);
                }
                let mut acts = [0u32; 4];
                for a in acts.iter_mut() {
                    *a = pick(&mut r, &[0, 1, 0xff, 0xffff_ffff]);
                }

                // libpad port entry and its DMA frame (frame 0 is the newer)
                let entry = LIBPAD_PORTS + port * 0x70 + slot * 0x1c;
                fill(c, &mut r, entry, 7);
                c.bus.write32(entry, DMA);
                c.bus.write32(entry + 0xc, 0);
                c.bus.write32(entry + 0x10, (seed % 6 != 0) as u32);
                fill(c, &mut r, DMA, 0x40);
                c.bus.write32(DMA + F_FRAME, 1);
                c.bus.write32(DMA + 0x80 + F_FRAME, 0);
                let len = pick(&mut r, &[0, 0x12, 0x20, 0x20]);
                c.bus.write32(DMA + F_LENGTH, len);
                if r.next_u32() % 3 != 0 {
                    c.bus.write8(DMA, 0);
                }
                for i in 4..8 {
                    let any = r.next_u32() & 0xff;
                    let b = pick(
                        &mut r,
                        &[0, 0x4e, 0x4f, 0x50, 0x80, 0xaf, 0xb0, 0xb1, 0xff, any],
                    );
                    c.bus.write8(DMA + i, b as u8);
                }
                let state = pick(&mut r, &[0, 1, 2, 5, 6, 6, 6, 7, 2]);
                c.bus.write8(DMA + F_STATE, state as u8);
                let req = pick(&mut r, &[0, 1, 2, 3]);
                c.bus.write8(DMA + F_REQ_STATE, req as u8);
                let ok = pick(&mut r, &[0, 1, 1, 1, 2]);
                c.bus.write8(DMA + F_OK, ok as u8);
                let any = r.next_u32() & 0xff;
                let config = pick(&mut r, &[0, 1, 2, any]);
                c.bus.write8(DMA + F_MODE_CONFIG, config as u8);
                let any = r.next_u32() & 0xff;
                let id = pick(&mut r, &[0x41, 0x73, 0x79, 0x23, 0xf3, any]);
                c.bus.write8(DMA + F_MODE_CUR_ID, id as u8);
                let offs = r.next_u32() % 4;
                c.bus.write8(DMA + F_MODE_CUR_OFFS, offs as u8);
                for i in 0..4 {
                    let ext = pick(&mut r, &[0, 4, 7, 5, 1]);
                    c.bus.write16(DMA + F_MODE_TABLE + 2 * i, ext as u16);
                }
                // keep scePadSetActDirect off its SIF DMA path (needs the
                // frame's ok byte 1): no motor change then
                if ok == 1 {
                    acts[1] = acts[0];
                    acts[3] = acts[2];
                }
                for (i, a) in acts.iter().enumerate() {
                    c.bus.write32(pad + ACT_SMALL + 4 * i as u32, *a);
                }

                fill(c, &mut r, OUT, 0x18);
                c.g[A0][0] = sx(pad);
                c.g[A1][0] = sx(OUT);
                c.pc = 0x0032_6eb0;
            },
        );
    }

    use crate::difftest::put_words;

    /// `InputVM_ReadPadChannel`: every kind (0..=5 and out of range), the
    /// index inside and past the count, words with and without bit 31 (the
    /// unsigned conversion's two paths); kind 5 retries with the default
    /// kind (never 5 again).
    #[test]
    fn input_vm_read_pad_channel_matches() {
        check_lohi(
            ee::s10::f_003252f8__generated,
            input_vm_read_pad_channel,
            300,
            &|c, seed| {
                let mut r = Rng(0x3252_f800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0032_52f8;
                put_words(c, PAD_CHANNEL_TABLE, &PAD_CHANNEL_CASES);
                let this = DATA;
                let list = DATA + 0x100;
                let count = 1 + r.next_u32() % 6;
                let def = [0u32, 1, 2, 3, 4, 6, 9][(seed / 7 % 7) as usize];
                c.bus.write32(this + PADVM_DEF_KIND, def);
                c.bus.write32(this + PADVM_LIST, list);
                c.bus.write32(list, count);
                for i in 0..(count * PAD_ENTRY_SIZE / 4 + 8) {
                    let w = match r.next_u32() % 3 {
                        0 => r.next_u32() | 0x8000_0000,
                        1 => r.next_u32() & 0x7fff_ffff,
                        _ => r.next_u32() % 300,
                    };
                    c.bus.write32(list + 4 + i * 4, w);
                }
                c.g[A0][0] = sx(this);
                let idx = match seed % 4 {
                    0 => count,
                    // A negative index passes the signed compare.
                    1 => (-(1 + (r.next_u32() % 3) as i32)) as u32,
                    _ => r.next_u32() % count,
                };
                c.g[A1][0] = sx(idx);
                c.g[A2][0] = match seed % 7 {
                    6 => [6u64, 0xffff_ffff_ffff_ffff, 1 << 40][(seed / 7 % 3) as usize],
                    k => k as u64,
                };
            },
        );
    }

    /// PadQueue_Poll: 0-4 devices (and a negative count), write index 0-29
    /// (29 wraps); the devices' read methods are fakes (some unwind).
    #[test]
    fn pad_queue_poll_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, DATA};
        check(
            ee::s10::f_00326b88__generated,
            pad_queue_poll,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32_6b88, 4);
                c.pc = 0x0032_6b88;
                let q = DATA;
                let n = if seed % 9 == 0 { u32::MAX } else { seed % 5 };
                c.bus.write32(q + 0x2ee8, n);
                c.bus.write32(
                    q + 0x2ee4,
                    if seed % 3 == 0 { 29 } else { r.next_u32() % 30 },
                );
                for i in 0..4u32 {
                    let dev = DATA + 0x4000 + 0x40 * i;
                    let vt = DATA + 0x5000 + 0x40 * i;
                    c.bus.write32(q + 0x2eec + 4 * i, dev);
                    c.bus.write32(dev, vt);
                    c.bus
                        .write16(vt + 0x10, (r.next_u32() as i16 as i32 & !3) as u16);
                    c.bus.write32(vt + 0x14, FAKE + 0x10 * i);
                }
                c.g[A0][0] = sx(q);
            },
        );
    }

    use ssx3_ee_rt::cpu::sx;
    use ssx3_ee_rt::Cpu;

    /// `scePadRead` on closed and open ports, with an 8-byte aligned and a
    /// 4-byte aligned DMA buffer (the two copy loops), either half current,
    /// interrupts on or off, data lengths 0..0x20.
    #[test]
    fn sce_pad_read_matches() {
        check(
            ee::s15::f_003ffa58__generated,
            sce_pad_read,
            120,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x3f_fa58, 0);
                c.cop0[12] = if seed % 2 == 0 { 0 } else { 0x1_0000 };
                let port = (seed / 2) % 2;
                let slot = (seed / 4) % 4;
                let at_m2 = PAD_PORTS + 0x70 * port + 0x1c * slot;
                let buf = DATA + 0x1000 + if seed % 3 == 0 { 4 } else { 0 };
                c.bus.write32(at_m2, buf);
                c.bus
                    .write32(at_m2 + PAD_OPEN, if seed % 7 == 0 { 0 } else { 1 });
                for k in 0..0x40 {
                    c.bus.write32(buf + 4 * k, r.next_u32());
                }
                // The current half: the one with the larger frame counter.
                c.bus.write32(buf + 0x58, r.next_u32() % 4);
                c.bus.write32(buf + 0xd8, r.next_u32() % 4);
                c.bus.write32(buf + PAD_DATA_LEN, r.next_u32() % 0x21);
                c.bus
                    .write32(buf + 0x80 + PAD_DATA_LEN, r.next_u32() % 0x21);
                c.g[A0][0] = port as u64;
                c.g[A1][0] = slot as u64;
                c.g[A2][0] = sx(DATA + 0x3000);
                c.pc = 0x003f_fa58;
            },
        );
    }

    // -------------------------------------------------------- scePadGetState

    /// `scePadGetState`: ports and slots 0..3, the port closed or open, a
    /// DMA buffer (either half, by the real `libpad_GetDmaBuffer`) whose
    /// state is 6 or another value, request state 2 or another value.
    #[test]
    fn sce_pad_get_state_matches() {
        use super::libpad::*;
        use crate::difftest::fakes::machine;
        use crate::difftest::{check_lohi, elf_image, DATA};
        check_lohi(
            ee::s15::f_003ffbc0__generated,
            sce_pad_get_state,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003f_fbc0, 0);
                elf_image(c);
                c.pc = 0x003f_fbc0;
                let (port, slot) = (r.next_u32() % 2, r.next_u32() % 4);
                let entry = PORTS + port * PORT_STRIDE + slot * SLOT_STRIDE;
                let buf = DATA + 0x100 * (r.next_u32() % 8);
                c.bus.write32(entry, buf);
                c.bus
                    .write32(entry + OPEN, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(buf + 0x58, r.next_u32() % 4);
                c.bus.write32(buf + 0xd8, r.next_u32() % 4);
                for half in [buf, buf + 0x80] {
                    let state = if r.next_u32() % 2 == 0 {
                        6
                    } else {
                        r.next_u32() % 8
                    };
                    let req = if r.next_u32() % 2 == 0 {
                        2
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write8(half + STATE, state as u8);
                    c.bus.write8(half + REQ_STATE, req as u8);
                }
                c.g[A0][0] = port as u64;
                c.g[A1][0] = slot as u64;
            },
        );
    }

    pub(crate) use crate::game::fe::feridermanager::tests::put_str_oc396;

    /// Script snippets reaching every branch of the tokenizer.
    pub(crate) fn next_token_cases() -> Vec<Vec<u8>> {
        let lits: &[&[u8]] = &[
            b"",
            b"   \t\r  ",
            b"\n\n  foo",
            b"\t\r\n  max",
            // Line directives.
            b"# 12 \"file.map\"\nmax",
            b"#12\"a\"  \nx",
            b"# x\nfoo",
            b"# 12 x\n=",
            b"# 7 \"unterminated\nabc",
            b"# 7 \"unterminated",
            b"# 3 \"f\" junk\n<",
            b"# 3 \"f\"  ",
            b"#",
            b"# 5",
            b"#\t9\"f\"\r\n:",
            b"# 1 \"f\"\t \n;",
            b"# 123456789 \"f\"\n(",
            b"#5 \"x\"\n",
            b"\n#1 \"g\"\n\n  foo",
            b" \n// c\n# 4 \"h\"\n 7.5",
            // Comments and operators.
            b"// comment here\n  sin(x)",
            b"//",
            b"// x",
            b"/ 3",
            b"/",
            b"/\n",
            b"!=",
            b"!x",
            b"!",
            b"++",
            b"+",
            b"+=",
            b"-+",
            b"-",
            b"-1",
            b".",
            b"*",
            b"==",
            b"= 1",
            b"=",
            b"<=",
            b"<",
            b">=",
            b">",
            b"&&",
            b"&x",
            b"&",
            b"||",
            b"|",
            b"|y",
            b"(",
            b")",
            b",",
            b"?",
            b";",
            b":",
            // Identifiers and keywords.
            b"max",
            b"MIN(",
            b"pow",
            b"atan2",
            b"atan",
            b"sin",
            b"cos",
            b"exp",
            b"log",
            b"floor",
            b"ceil",
            b"abs",
            b"sqrt",
            b"minradian",
            b"foo_bar9 ",
            b"x",
            b"Zebra_",
            // Numbers.
            b"123",
            b"42;",
            b"1.5",
            b"0.25f",
            b"3f",
            b"2E3",
            b"2e-3",
            b"1.5e+2f",
            b"7e",
            b"7e+",
            b"7E-x",
            b"1.e5",
            b"1.",
            b"5x",
            b"9",
            b"0.5e",
            // Unexpected characters.
            b"@",
            b"\x80",
            b"\xff",
            b"~a",
            b"\xe9t",
            b"_x",
            b"$",
        ];
        let mut v: Vec<Vec<u8>> = lits.iter().map(|s| s.to_vec()).collect();
        let mut long_name = b"# 3 \"".to_vec();
        long_name.extend(std::iter::repeat(b'n').take(140));
        long_name.extend_from_slice(b"\"\n1");
        v.push(long_name);
        let mut long_ident = vec![b'a'; 60];
        long_ident.extend_from_slice(b"b_12345678");
        v.push(long_ident);
        let mut long_int = vec![b'0'; 70];
        long_int.push(b'1');
        v.push(long_int);
        let mut long_frac = b"0.".to_vec();
        long_frac.extend(std::iter::repeat(b'0').take(70));
        long_frac.push(b'5');
        v.push(long_frac);
        let mut long_exp = b"1e".to_vec();
        long_exp.extend(std::iter::repeat(b'0').take(70));
        long_exp.push(b'2');
        v.push(long_exp);
        let mut long_exp_sign = b"1".to_vec();
        long_exp_sign.extend(std::iter::repeat(b'0').take(66));
        long_exp_sign.extend_from_slice(b"e-1f");
        v.push(long_exp_sign);
        v
    }

    #[test]
    fn input_parser_next_token_matches() {
        let cases = next_token_cases();
        let n = cases.len() as u32;
        check(
            ee::s10::f_00321af0__generated,
            input_parser_next_token,
            2 * n,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0032_1af0, 0);
                elf_image(c);
                env(c);
                c.pc = 0x0032_1af0;
                let text = &cases[(seed % n) as usize];
                let this = DATA + 0x400;
                let buf = DATA + 0x1000;
                // Junk after the end: the bounds checks must stop there.
                for i in 0..0x40 {
                    c.bus
                        .write8(buf + text.len() as u32 + i, (r.next_u32() | 0x20) as u8);
                }
                for (i, b) in text.iter().enumerate() {
                    c.bus.write8(buf + i as u32, *b);
                }
                for off in (0..0x100).step_by(4) {
                    c.bus.write32(this + off, r.next_u32());
                }
                c.bus.write32(this + NEXT_TOKEN_BASE, buf);
                c.bus.write32(this + NEXT_TOKEN_LEN, text.len() as u32);
                c.bus.write32(this + NEXT_TOKEN_POS, buf);
                for (i, b) in b"pad.map\0".iter().enumerate() {
                    c.bus.write8(this + NEXT_TOKEN_FNAME + i as u32, *b);
                }
                c.bus.write32(this + NEXT_TOKEN_LINE, r.next_u32() % 1000);
                // Second round: not at the start of a line.
                let line_start = if seed < n { r.next_u32() | 1 } else { 0 };
                c.bus.write32(this + NEXT_TOKEN_LINE_START, line_start);
                c.bus.write32(this + NEXT_TOKEN_ERROR, r.next_u32() & 1);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// The parser (`self`), the result register `*dst`, the opcode buffer
    /// and the parse-node pool (0x80 bytes per node, room for a variable's
    /// expression at +0x40).
    pub(crate) const IPEN_SELF: u32 = DATA;

    pub(crate) const IPEN_DST: u32 = DATA + 0x200;

    pub(crate) const IPEN_CODE: u32 = DATA + 0x400;

    pub(crate) const IPEN_NODES: u32 = DATA + 0x1000;

    pub(crate) const IPEN_MAX_NODES: u32 = 0x100;

    /// Build a random parse tree of at most `depth` levels; returns the root.
    pub(crate) fn ipen_node(
        c: &mut Cpu,
        r: &mut Rng,
        next: &mut u32,
        depth: u32,
        ty: Option<u32>,
    ) -> u32 {
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

    // ------------------------------------------------- InputParser_ParseUnary

    /// The parser for the ParseUnary test, its source text, node pool and
    /// identifier expressions.
    pub(crate) const PU_PARSER: u32 = crate::difftest::DATA;

    pub(crate) const PU_SRC: u32 = crate::difftest::DATA + 0x4_0000;

    pub(crate) const PU_NODES: u32 = crate::difftest::DATA + 0x2_0000;

    pub(crate) const PU_EXPRS: u32 = crate::difftest::DATA + 0x3_0000;

    /// Defined identifiers: name, expression kind (4 = a constant).
    pub(crate) const PU_IDENTS: [(&[u8], u32); 4] =
        [(b"speed", 4), (b"gate", 7), (b"self", 1), (b"tilt", 4)];

    /// Scenarios: the current token (kind, text) and the source after it.
    pub(crate) const PU_SCENARIOS: &[(u32, &[u8], &[u8])] = &[
        // Buttons, plain and with modifiers (good, unknown, missing).
        (2, b"button5", b" + 1"),
        (2, b"button12", b".held;"),
        (2, b"button7", b" . pressed"),
        (2, b"button0", b".released"),
        (2, b"button3", b".bogus"),
        (2, b"button3", b".(1)"),
        // Config values.
        (2, b"padtype", b")"),
        (2, b"config4", b""),
        (2, b"arg2", b"*2"),
        // Identifiers: constant, expression, recursive, modifiers.
        (2, b"speed", b""),
        (2, b"gate", b";"),
        (2, b"self", b""),
        (2, b"gate", b".released"),
        (2, b"tilt", b".repeat"),
        (2, b"gate", b".nope"),
        (2, b"gate", b".7"),
        (2, b"button999", b""),
        (2, b"mystery", b""),
        // Numbers and unary operators.
        (11, b"2.5", b" ;"),
        (20, b"+", b"4"),
        (20, b"+", b"+ + button2"),
        (21, b"-", b"3.25"),
        (21, b"-", b"button4.value"),
        (21, b"-", b"-gate"),
        (21, b"-", b";"),
        (24, b"!", b"0"),
        (24, b"!", b"2"),
        (24, b"!", b"speed"),
        (24, b"!", b"(button1)"),
        (24, b"!", b")"),
        // Parentheses.
        (9, b"(", b"1 + 2)"),
        (9, b"(", b"button3 * 2;"),
        (9, b"(", b"1;"),
        // Not a term.
        (0, b"", b""),
        (1, b"#", b""),
        (3, b"=", b"1"),
        (10, b")", b""),
        (22, b"*", b"2"),
        (39, b"?", b""),
        (0xffff_ffff, b"", b""),
    ];

    pub(crate) fn pu_test_str(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, &b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// `InputParser_ParseUnary` against the generated body with every
    /// callee real (the tokenizer reads a source text after the current
    /// token): each kind of term, each modifier and syntax error, nested
    /// unary operators (the generated recursion), parenthesised expressions
    /// and every built-in function keyword, and tokens that are no term.
    #[test]
    fn input_parser_parse_unary_matches() {
        use crate::difftest::{check, elf_image, scramble, Rng, GAME_GP};
        let funcs = 14u32;
        let n = PU_SCENARIOS.len() as u32 + funcs + 2;
        check(
            ee::s10::f_00323bd8__generated,
            input_parser_parse_unary,
            200,
            &|c, seed| {
                let mut r = Rng(0x0032_3bd8 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                elf_image(c);
                c.pc = 0x0032_3bd8;
                c.g[A0][0] = sx(PU_PARSER);
                let k = seed % n;
                let (tok, text, rest): (u32, &[u8], &[u8]) = if (k as usize) < PU_SCENARIOS.len() {
                    PU_SCENARIOS[k as usize]
                } else if k - (PU_SCENARIOS.len() as u32) < funcs {
                    // max min pow atan2 take two arguments, the rest one.
                    let f = k - PU_SCENARIOS.len() as u32;
                    let rest: &[u8] = if f < 4 {
                        &b"(1, button2)"[..]
                    } else {
                        &b"(speed)"[..]
                    };
                    (25 + f, &b"f"[..], rest)
                } else if k % 2 == 0 {
                    (25, &b"max"[..], &b"1)"[..])
                } else {
                    (30, &b"sin"[..], &b""[..])
                };
                let p = PU_PARSER;
                // Tokenizer: source, position, file name, line.
                pu_test_str(c, PU_SRC, rest);
                c.bus.write32(p, PU_SRC);
                c.bus.write32(p + 4, rest.len() as u32);
                c.bus.write32(p + 8, PU_SRC);
                pu_test_str(c, p + 0xc, b"keys.map");
                c.bus.write32(p + 0x8c, r.next_u32() % 200);
                c.bus.write32(p + 0x90, 0);
                c.bus.write32(p + 0x94, 0);
                // The current token.
                c.bus.write32(p + PARSE_UNARY_TOKEN, tok);
                pu_test_str(c, p + PARSE_UNARY_TEXT, text);
                let value = if seed % 5 == 0 { 0 } else { r.f32(100.0) };
                c.bus.write32(p + PARSE_UNARY_VALUE, value);
                // Free expression nodes.
                for i in 0..48u32 {
                    let node = PU_NODES + 0x10 * i;
                    c.bus.write32(node, r.next_u32());
                    c.bus
                        .write32(node + 4, if i == 47 { 0 } else { node + 0x10 });
                }
                c.bus.write32(p + PARSE_UNARY_FREE_NODES, PU_NODES);
                // Identifiers and the one being defined ("self").
                c.bus
                    .write32(p + PARSE_UNARY_IDENT_COUNT, PU_IDENTS.len() as u32);
                for (i, (name, kind)) in PU_IDENTS.iter().enumerate() {
                    let e = p + PARSE_UNARY_IDENTS + 0x44 * i as u32;
                    let expr = PU_EXPRS + 0x10 * i as u32;
                    pu_test_str(c, e, name);
                    c.bus.write32(e + PARSE_UNARY_IDENT_EXPR, expr);
                    c.bus.write32(expr, *kind);
                    let v = if seed % 7 == 0 { 0 } else { r.f32(10.0) };
                    c.bus.write32(expr + 4, v);
                    c.bus.write32(expr + 8, r.next_u32());
                }
                let current = p + PARSE_UNARY_IDENTS + 0x44 * 2;
                c.bus
                    .write32(p + 0x1_0000 + PARSE_UNARY_CURRENT_IDENT, current);
            },
        );
    }

    /// The libc character-class table `tolower` reads (bit 0: uppercase).
    pub(crate) fn oc396_ctype_upper(c: &mut Cpu) {
        for ch in b'A'..=b'Z' {
            c.bus.write8(0x0049_9c81 + ch as u32, 1);
        }
    }

    /// The input map's built-in names, from a pool of near misses.
    #[test]
    fn input_parser_lookup_builtin_name_matches() {
        const NAMES: &[&[u8]] = &[
            b"padtype",
            b"PadType",
            b"padtyp",
            b"arg0",
            b"ARG7",
            b"arg8",
            b"arg12",
            b"arg3x",
            b"arg3_",
            b"arg",
            b"argx",
            b"config0",
            b"Config22",
            b"config23",
            b"config5Z",
            b"config9_",
            b"config",
            b"configs",
            b"conf1",
            b"x",
            b"",
            b"arg05",
            b"config017",
        ];
        check(
            ee::s10::f_00321880__generated,
            input_parser_lookup_builtin_name,
            300,
            &|c, seed| {
                let mut r = Rng(0x0396_0006 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0032_1880;
                oc396_ctype_upper(c);
                put_str_oc396(c, 0x004a_3f18, b"padtype");
                put_str_oc396(c, 0x004a_3f20, b"arg");
                put_str_oc396(c, 0x004a_3f28, b"config");
                let name = DATA + 0x100;
                put_str_oc396(c, name, NAMES[(seed as usize) % NAMES.len()]);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(name);
            },
        );
    }

    // ------------------------------------------------- expression parser

    /// Run guest function `f` at `addr` with `a0` from a setup, back to `RA`.
    pub(crate) fn oc396_run(c: &mut Cpu, addr: u32, f: GuestFn, a0: u32) {
        c.g[A0][0] = sx(a0);
        c.g[RA][0] = sx(0x0000_4f00);
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, 0x0000_4f00, "setup call {addr:#x} did not return");
        c.g[RA][0] = crate::difftest::RA as u64;
    }

    /// A parser over `text` with its first token read (the real tokenizer),
    /// a free list of 24 nodes, entered at `entry`.
    pub(crate) fn oc396_parser(c: &mut Cpu, seed: u32, text: &[u8], entry: u32) {
        machine(c, seed, entry as u64, 0);
        crate::difftest::elf_image(c);
        crate::game::fe::feasyncfile::tests::env(c);
        let this = DATA + 0x400;
        let buf = DATA + 0x1000;
        for off in (0..0x100).step_by(4) {
            c.bus.write32(this + off, 0);
        }
        put_str_oc396(c, buf, text);
        c.bus.write32(this, buf); // text
        c.bus.write32(this + 4, text.len() as u32); // length
        c.bus.write32(this + 8, buf); // position
        put_str_oc396(c, this + 0xc, b"pad.map");
        c.bus.write32(this + 0x8c, 1); // line
        let nodes = DATA + 0x2000;
        for i in 0..24 {
            let next = if i == 23 { 0 } else { nodes + 0x10 * (i + 1) };
            put_words(c, nodes + 0x10 * i, &[0, next, 0, 0]);
        }
        c.bus.write32(this + 0xe0, nodes);
        oc396_run(c, 0x0032_1af0, ee::s10::f_00321af0, this);
        c.pc = entry;
        c.g[A0][0] = sx(this);
    }

    #[test]
    fn input_parser_parse_comparison_matches() {
        const EXPRS: &[&[u8]] = &[
            b"1<2",
            b"2>=2",
            b"3>4",
            b"1<=0",
            b"2==2",
            b"2!=3",
            b"5==4",
            b"3!=3",
            b"1<2<3",
            b"1+2>2",
            b"arg1<2",
            b"2>arg3",
            b"arg1==arg2",
            b"1<",
            b"",
            b"7",
            b"padtype!=1",
            b"1>=2>=0",
            b"4<=4",
            b"2>1",
            b"0.5<=0.25",
            b"1==1!=0",
        ];
        check(
            ee::s10::f_003236d8__generated,
            input_parser_parse_comparison,
            2 * EXPRS.len() as u32,
            &|c, seed| {
                oc396_parser(c, seed, EXPRS[seed as usize % EXPRS.len()], 0x0032_36d8);
            },
        );
    }

    #[test]
    fn input_parser_parse_additive_matches() {
        const EXPRS: &[&[u8]] = &[
            b"1+2",
            b"5-3",
            b"1+2-4+8",
            b"arg1+2",
            b"2+arg1",
            b"arg1-1+3",
            b"-1+2",
            b"1-",
            b"3",
            b"2*3+1",
            b"1+2*3",
            b"arg2+arg3-1",
            b"1+2<3",
            b"+",
            b"1-arg1",
            b"arg1-arg2",
            b"4-2-1",
        ];
        check(
            ee::s10::f_00323900__generated,
            input_parser_parse_additive,
            2 * EXPRS.len() as u32,
            &|c, seed| {
                oc396_parser(c, seed, EXPRS[seed as usize % EXPRS.len()], 0x0032_3900);
            },
        );
    }

    #[test]
    fn input_parser_parse_multiplicative_matches() {
        const EXPRS: &[&[u8]] = &[
            b"2*3",
            b"8/2",
            b"2*3/4*5",
            b"arg1*2",
            b"2*arg1",
            b"arg1/4*2",
            b"3*",
            b"6",
            b"2/0",
            b"1*2+3",
            b"arg1*arg2",
            b"4/arg1",
            b"*",
            b"2/4/8",
            b"arg2/arg3",
        ];
        check(
            ee::s10::f_00323a68__generated,
            input_parser_parse_multiplicative,
            2 * EXPRS.len() as u32,
            &|c, seed| {
                oc396_parser(c, seed, EXPRS[seed as usize % EXPRS.len()], 0x0032_3a68);
            },
        );
    }

    use std::ptr::addr_of_mut;

    pub(crate) use crate::game::ai::rider::tests::w32_2;
    pub(crate) use crate::game::world::worldview::tests::oc410_machine;
    pub(crate) use crate::game::world::worldview::tests::oc410_out;
    pub(crate) use crate::game::world::worldview::tests::oc410_result;

    pub(crate) use crate::game::ai::ai::tests::oc414_env;

    // ------------------------------------------------------------ fixtures

    /// Gate of the callee fakes (a magic word in the machine's RAM). Above
    /// the fakes' call log (`DATA + 0x10_0000`).
    pub(crate) const O406_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const O406_MAGIC: u32 = 0x4f34_3036;

    /// Index of the next 64-bit fake result, and the results (24).
    pub(crate) const O406_RESULT_IDX: u32 = O406_CTRL + 0x10;

    pub(crate) const O406_RESULTS: u32 = O406_CTRL + 0x20;

    /// Index of the next token kind a `NextToken` fake delivers, and the kinds (32).
    pub(crate) const O406_KIND_IDX: u32 = O406_CTRL + 0x14;

    pub(crate) const O406_KINDS: u32 = O406_CTRL + 0x104;

    /// Index of the next character a `NextChar` fake delivers, and the characters (64).
    pub(crate) const O406_CHAR_IDX: u32 = O406_CTRL + 0x18;

    pub(crate) const O406_CHARS: u32 = O406_CTRL + 0x204;

    /// The InputParser, the expression node pool, the bolt database.
    pub(crate) const O406_PARSER: u32 = DATA + 0x4_0000;

    pub(crate) const O406_NODES: u32 = DATA + 0x5_0000;

    pub(crate) const O406_BOLT_DB: u32 = DATA + 0x1000;

    /// A sequence track (the `GetTrack` fake's result).
    pub(crate) const O406_TRACK: u32 = DATA + 0x400;

    pub(crate) fn o406_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    pub(crate) fn o406_pop64(c: &mut Cpu) -> u64 {
        let i = c.bus.read32(O406_RESULT_IDX);
        c.bus.write32(O406_RESULT_IDX, i + 1);
        if i >= 24 {
            0
        } else {
            c.bus.read64(O406_RESULTS + 8 * i)
        }
    }

    pub(crate) fn o406_pop_kind(c: &mut Cpu) -> u32 {
        let i = c.bus.read32(O406_KIND_IDX);
        c.bus.write32(O406_KIND_IDX, i + 1);
        if i >= 32 {
            0
        } else {
            c.bus.read32(O406_KINDS + 4 * i)
        }
    }

    pub(crate) fn o406_pop_char(c: &mut Cpu) -> u32 {
        let i = c.bus.read32(O406_CHAR_IDX);
        c.bus.write32(O406_CHAR_IDX, i + 1);
        if i >= 64 {
            u32::MAX
        } else {
            c.bus.read32(O406_CHARS + 4 * i)
        }
    }

    /// A gated callee fake: logs the call like `fakes::fake` and, for the
    /// callees whose results the parsers and sequences act on, delivers the
    /// next queued value (the setup of the test fills the queues).
    pub(crate) fn o406_fake_call(c: &mut Cpu, addr: u32, ra: u32) {
        let a0 = c.g[A0][0] as u32;
        let a2 = c.g[A2][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        match addr {
            // ParseComparison, Lookup*/FindIdentifier, FindBoltIndex,
            // SetCharProfileBoltEquipped, ChannelGetLength/Listener/Flush, the
            // rider state, an animation id, a flag test.
            0x0032_36d8 | 0x0032_16e8 | 0x0032_1880 | 0x0032_1a40 | 0x0014_d448 | 0x0015_1c90
            | 0x0027_4c70 | 0x0027_4d70 | 0x0027_3d20 | 0x0011_fe98 | 0x0031_1ae8 | 0x0014_46a0 => {
                c.g[V0][0] = o406_pop64(c);
            }
            // NextToken: the parser's token kind.
            0x0032_1af0 => {
                let kind = o406_pop_kind(c);
                c.bus.write32(a0 + 0x98, kind);
            }
            // EmitNode: a word count and the register it chose.
            0x0032_4678 => {
                c.g[V0][0] = o406_pop64(c);
                let reg = o406_pop64(c) as u32;
                c.bus.write32(a2, reg);
            }
            // NextChar: the preprocessor's lookahead character.
            0x0032_6478 => {
                let ch = o406_pop_char(c);
                c.bus.write32(a0 + 0xc, ch);
            }
            // EventTrack_NextKey: the next key.
            0x0027_3468 => {
                let key = o406_pop64(c) as u32;
                c.bus.write32(a0 + 0x14, key);
            }
            0x0014_bdb8 => c.g[V0][0] = O406_BOLT_DB as u64,
            0x0027_42c8 => c.g[V0][0] = O406_TRACK as u64,
            _ => {}
        }
    }

    macro_rules! o406_fakes {
        ($(($name:ident, $addr:expr, $shard:ident, $slot:ident, $gen:ident)),* $(,)?) => {
            $(
                pub(crate) fn $name(c: &mut Cpu) {
                    if c.bus.read32(O406_CTRL) != O406_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), ee::$shard::$gen);
                    }
                    let ra = c.g[RA][0] as u32;
                    o406_fake_call(c, $addr, ra);
                }
            )*

            /// Hooks the gated fakes into the callees' override slots (once
            /// per process; with the gate off they run what the slots held).
            pub(crate) fn o406_install_fakes() {
                $( fakes::hook(addr_of_mut!(ee::$shard::$slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    o406_fakes! {
        (o406_fake_boost, 0x0011_4130, s00, OVERRIDE_00114130, f_00114130__generated),
        (o406_fake_wait_push, 0x0012_c0c0, s00, OVERRIDE_0012C0C0, f_0012c0c0__generated),
        (o406_fake_push, 0x0012_c130, s00, OVERRIDE_0012C130, f_0012c130__generated),
        (o406_fake_charge, 0x0012_c230, s00, OVERRIDE_0012C230, f_0012c230__generated),
        (o406_fake_launch, 0x0012_c408, s00, OVERRIDE_0012C408, f_0012c408__generated),
        (o406_fake_rider_state, 0x0011_fe98, s00, OVERRIDE_0011FE98, f_0011fe98__generated),
        (o406_fake_rider_misc, 0x0011_5640, s00, OVERRIDE_00115640, f_00115640__generated),
        (o406_fake_rider_leave, 0x0011_fec8, s00, OVERRIDE_0011FEC8, f_0011fec8__generated),
        (o406_fake_compare, 0x0032_36d8, s10, OVERRIDE_003236D8, f_003236d8__generated),
        (o406_fake_next_token, 0x0032_1af0, s10, OVERRIDE_00321AF0, f_00321af0__generated),
        (o406_fake_alloc_node, 0x0032_15f8, s10, OVERRIDE_003215F8, f_003215f8__generated),
        (o406_fake_parser_error, 0x0032_1500, s10, OVERRIDE_00321500, f_00321500__generated),
        (o406_fake_lookup_button, 0x0032_16e8, s10, OVERRIDE_003216E8, f_003216e8__generated),
        (o406_fake_lookup_builtin, 0x0032_1880, s10, OVERRIDE_00321880, f_00321880__generated),
        (o406_fake_find_identifier, 0x0032_1a40, s10, OVERRIDE_00321A40, f_00321a40__generated),
        (o406_fake_emit_node, 0x0032_4678, s10, OVERRIDE_00324678, f_00324678__generated),
        (o406_fake_preproc_write, 0x0032_6360, s10, OVERRIDE_00326360, f_00326360__generated),
        (o406_fake_next_char, 0x0032_6478, s10, OVERRIDE_00326478, f_00326478__generated),
        (o406_fake_skip_line_comment, 0x0032_65c0, s10, OVERRIDE_003265C0, f_003265c0__generated),
        (o406_fake_rand, 0x0031_77f0, s10, OVERRIDE_003177F0, f_003177f0__generated),
        (o406_fake_new_named, 0x0031_7e30, s10, OVERRIDE_00317E30, f_00317e30__generated),
        (o406_fake_anim_a, 0x0031_1ae8, s10, OVERRIDE_00311AE8, f_00311ae8__generated),
        (o406_fake_anim_b, 0x0031_1b20, s10, OVERRIDE_00311B20, f_00311b20__generated),
        (o406_fake_chunk_record, 0x0032_c540, s10, OVERRIDE_0032C540, f_0032c540__generated),
        (o406_fake_event_fire_key, 0x0027_3418, s07, OVERRIDE_00273418, f_00273418__generated),
        (o406_fake_event_next_key, 0x0027_3468, s07, OVERRIDE_00273468, f_00273468__generated),
        (o406_fake_flush_events, 0x0027_3d20, s07, OVERRIDE_00273D20, f_00273d20__generated),
        (o406_fake_get_length, 0x0027_4c70, s07, OVERRIDE_00274C70, f_00274c70__generated),
        (o406_fake_get_listener, 0x0027_4d70, s07, OVERRIDE_00274D70, f_00274d70__generated),
        (o406_fake_get_track, 0x0027_42c8, s07, OVERRIDE_002742C8, f_002742c8__generated),
        (o406_fake_node_matrix, 0x0034_fed8, s11, OVERRIDE_0034FED8, f_0034fed8__generated),
        (o406_fake_collide_instance, 0x0033_5960, s11, OVERRIDE_00335960, f_00335960__generated),
        (o406_fake_collide_object, 0x0033_5b90, s11, OVERRIDE_00335B90, f_00335b90__generated),
        (o406_fake_bolts_db, 0x0014_bdb8, s01, OVERRIDE_0014BDB8, f_0014bdb8__generated),
        (o406_fake_find_bolt, 0x0014_d448, s01, OVERRIDE_0014D448, f_0014d448__generated),
        (o406_fake_set_bolt, 0x0015_1c90, s01, OVERRIDE_00151C90, f_00151c90__generated),
        (o406_fake_flags_test, 0x0014_46a0, s01, OVERRIDE_001446A0, f_001446a0__generated),
        (o406_fake_mem_fill, 0x003e_6448, s14, OVERRIDE_003E6448, f_003e6448__generated),
    }

    /// The miss hook of the sequence tests: the track player's tick (vtable
    /// target `FAKE`) answers from the result queue and writes a flag word;
    /// the key-distance call (`FAKE + 0x30`) answers a small count.
    pub(crate) fn o406_miss(c: &mut Cpu, target: u32) {
        let a1 = c.g[A1][0] as u32;
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        if c.pc != ra {
            return;
        }
        if target == fakes::FAKE {
            c.g[V0][0] = o406_pop64(c);
            let flags = o406_pop64(c) as u32;
            c.bus.write32(a1, flags);
        } else if target == fakes::FAKE + 0x30 {
            c.g[V0][0] = o406_pop64(c);
        }
    }

    /// Scrambled registers, the gate, `pc` at the entry, the fakes.
    pub(crate) fn o406_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        o406_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.pc = entry;
        c.bus.write32(O406_CTRL, O406_MAGIC);
        c.hooks.miss = o406_miss;
        r
    }

    /// Fill the 64-bit result queue.
    pub(crate) fn o406_results(c: &mut Cpu, values: &[u64]) {
        for (i, v) in values.iter().enumerate() {
            c.bus.write64(O406_RESULTS + 8 * i as u32, *v);
        }
    }

    pub(crate) fn o406_kinds(c: &mut Cpu, kinds: &[u32]) {
        for (i, k) in kinds.iter().enumerate() {
            c.bus.write32(O406_KINDS + 4 * i as u32, *k);
        }
    }

    /// A pool of 12 expression nodes at `O406_NODES`: constants (kind 4,
    /// value 0.0 / 1.0 / -3.5 / 0.5) and other leaf kinds (children null).
    pub(crate) fn o406_node_pool(c: &mut Cpu, r: &mut Rng) {
        const VALUES: [u32; 4] = [0, 0x3f80_0000, 0xc060_0000, 0x3f00_0000];
        const KINDS: [u32; 5] = [5, 6, 7, 0x10, 0x21];
        for i in 0..12u32 {
            let n = O406_NODES + 0x40 * i;
            for off in (0..0x40).step_by(4) {
                c.bus.write32(n + off, 0);
            }
            if r.next_u32() % 20 < 11 {
                c.bus.write32(n, 4);
                c.bus.write32(n + 4, VALUES[(r.next_u32() % 4) as usize]);
            } else {
                c.bus.write32(n, KINDS[(r.next_u32() % 5) as usize]);
            }
        }
    }

    /// Queue of node results: the pool in a random order, null now and then.
    pub(crate) fn o406_node_results(c: &mut Cpu, r: &mut Rng, first: usize) {
        let mut values = vec![0u64; first];
        for i in 0..14u32 {
            let k = (r.next_u32() % 12) + i % 3;
            values.push(if r.next_u32() % 8 == 0 {
                0
            } else {
                (O406_NODES + 0x40 * (k % 12)) as u64
            });
        }
        o406_results(c, &values);
    }

    /// An InputParser at `O406_PARSER` holding `kind` as the current token.
    pub(crate) fn o406_parser(c: &mut Cpu, kind: u32) {
        for off in (0..0x200).step_by(4) {
            c.bus.write32(O406_PARSER + off, 0);
        }
        c.bus.write32(O406_PARSER + 0x98, kind);
        c.g[A0][0] = O406_PARSER as u64;
    }

    /// A random token kind out of `kinds`.
    pub(crate) fn o406_pick(r: &mut Rng, kinds: &[u32]) -> u32 {
        kinds[(r.next_u32() as usize) % kinds.len()]
    }

    // --------------------------------------- InputParser_ParseLogicalAnd/Or/...

    /// `a && b && ...`: a constant 0 / 1 / other operands, `&&` tokens, a null
    /// operand (error), node kinds; the comparison/NextToken/alloc callees are
    /// fakes fed from queues; `FreeNode` is real.
    #[test]
    fn input_parser_parse_logical_and_matches() {
        fakes::covered(
            ee::s10::f_00323540__generated,
            input_parser_parse_logical_and,
            200,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x323540, 8, 0x0032_3540);
                elf_image(c);
                o406_node_pool(c, &mut r);
                o406_node_results(c, &mut r, 0);
                let mut kinds = vec![];
                for _ in 0..6 {
                    kinds.push(o406_pick(&mut r, &[0xd, 0xd, 0xd, 0]));
                }
                o406_kinds(c, &kinds);
                o406_parser(c, o406_pick(&mut r, &[0xd, 0xd, 0xd, 0xc]));
            },
        );
    }

    /// `a || b || ...` over the real `ParseLogicalAnd`.
    #[test]
    fn input_parser_parse_logical_or_matches() {
        fakes::covered(
            ee::s10::f_003233b8__generated,
            input_parser_parse_logical_or,
            200,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x3233b8, 8, 0x0032_33b8);
                elf_image(c);
                o406_node_pool(c, &mut r);
                o406_node_results(c, &mut r, 0);
                let mut kinds = vec![];
                for _ in 0..8 {
                    kinds.push(o406_pick(&mut r, &[0xc, 0xc, 0xd, 0]));
                }
                o406_kinds(c, &kinds);
                o406_parser(c, o406_pick(&mut r, &[0xc, 0xc, 0xd, 0xd, 0]));
            },
        );
    }

    /// `c ? a : b` (recursing into the real `ParseConditional`).
    #[test]
    fn input_parser_parse_conditional_matches() {
        fakes::covered(
            ee::s10::f_00323268__generated,
            input_parser_parse_conditional,
            240,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x323268, 10, 0x0032_3268);
                elf_image(c);
                o406_node_pool(c, &mut r);
                o406_node_results(c, &mut r, 0);
                let mut kinds = vec![];
                for _ in 0..10 {
                    kinds.push(o406_pick(&mut r, &[7, 7, 6, 0, 0xc, 0xd, 5]));
                }
                o406_kinds(c, &kinds);
                o406_parser(c, o406_pick(&mut r, &[6, 6, 6, 0, 7]));
            },
        );
    }

    /// `name = expression;`: the name token, the three lookups (button,
    /// builtin, identifier: mostly "not found"), the new variable (real
    /// `strcpy`), `=`, the expression (real parser chain) and `;`, with the
    /// error exits (missing `=`, missing `;`, failed expression, redefined
    /// name).
    #[test]
    fn input_parser_parse_statement_matches() {
        fakes::covered(
            ee::s10::f_003230e0__generated,
            input_parser_parse_statement,
            260,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x3230e0, 10, 0x0032_30e0);
                elf_image(c);
                o406_node_pool(c, &mut r);
                o406_node_results(c, &mut r, 3);
                let lookups = [u64::MAX, u64::MAX, 0, 0, 0, 1];
                let mut head = vec![];
                for i in 0..3 {
                    let pick = if r.next_u32() % 5 == 0 {
                        lookups[(r.next_u32() % 6) as usize]
                    } else if i < 2 {
                        u64::MAX
                    } else {
                        0
                    };
                    head.push(pick);
                }
                for (i, v) in head.iter().enumerate() {
                    c.bus.write64(O406_RESULTS + 8 * i as u32, *v);
                }
                let kinds = [
                    o406_pick(&mut r, &[3, 3, 3, 3, 4]),
                    o406_pick(&mut r, &[5, 5, 5, 5, 0xd, 0xc, 6, 0]),
                    o406_pick(&mut r, &[5, 5, 0, 7]),
                    o406_pick(&mut r, &[5, 0]),
                    0,
                    5,
                ];
                o406_kinds(c, &kinds);
                o406_parser(c, o406_pick(&mut r, &[2, 2, 2, 2, 2, 5]));
                // The token text and the variable table (count at +0x80e4).
                c.bus.write32(O406_PARSER + 0x9c, 0x0000_6261);
                c.bus.write32(O406_PARSER + 0x80e4, r.next_u32() % 4);
            },
        );
    }

    // --------------------------------------------- InputParser_EmitBinaryOp

    /// Compile a binary node: both `EmitNode` calls (fakes: a word count or -1
    /// and the chosen register), the free-register list, the destination
    /// choice, the allocation and overflow errors and the opcode word.
    #[test]
    fn input_parser_emit_binary_op_matches() {
        fakes::covered(
            ee::s10::f_00324ec8__generated,
            input_parser_emit_binary_op,
            260,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x324ec8, 3, 0x0032_4ec8);
                let regs = [u64::MAX, 3, 0x20, 0x3f, 0x40, 0x7f, 1, 2];
                let counts = [u64::MAX, 0, 1, 2, 3, 1, 2];
                let mut values = vec![];
                for _ in 0..2 {
                    values.push(counts[(r.next_u32() % 7) as usize]);
                    values.push(regs[(r.next_u32() % 8) as usize]);
                }
                o406_results(c, &values);
                o406_parser(c, 0);
                let free_count = r.next_u32() % 4;
                // The free list (+0x108ec) and its count (+0x109e8).
                c.bus.write32(O406_PARSER + 0x10000 + 0x9e8, free_count);
                for i in 0..4 {
                    c.bus
                        .write32(O406_PARSER + 0x10000 + 0x8ec + 4 * i, r.next_u32() % 0x40);
                }
                c.g[A1][0] = (O406_NODES) as u64;
                c.g[A2][0] = (DATA + 0x100) as u64;
                let dst = [u32::MAX, 5, 0x30, u32::MAX][(r.next_u32() % 4) as usize];
                c.bus.write32(DATA + 0x100, dst);
                c.g[A3][0] = (DATA + 0x800) as u64;
                c.bus.write32(DATA + 0x800, r.next_u32());
                c.g[T0][0] = (r.next_u32() % 9) as u64;
                c.g[T1][0] = r.next_u32() as u64;
                c.g[T2][0] = r.next_u32() as u64;
            },
        );
    }

    // --------------------------------------------- InputPreproc_ProcessText

    /// One source line: leading blanks, `//` and `/*` comments, a plain `/`
    /// and ordinary text up to the newline (lookahead characters from a
    /// queue; `Write`, `NextChar` and the line-comment skipper are fakes, the
    /// block-comment skipper is real). The `#` directive path is not
    /// reached (the real directive handler reads files).
    #[test]
    fn input_preproc_process_text_matches() {
        fakes::covered(
            ee::s10::f_00326150__generated,
            input_preproc_process_text,
            260,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x326150, 6, 0x0032_6150);
                const ALPHABET: [u32; 9] = [0x20, 9, 0xd, 0x78, 0x2f, 0x2a, 0xa, 0x2f, 0x79];
                let first = ALPHABET[(r.next_u32() % 9) as usize];
                for i in 0..20u32 {
                    let ch = if i == 16 {
                        u32::MAX
                    } else {
                        ALPHABET[(r.next_u32() % 9) as usize]
                    };
                    c.bus.write32(O406_CHARS + 4 * i, ch);
                }
                o406_w(c, DATA + 0xc, if seed % 17 == 0 { u32::MAX } else { first });
                o406_w(c, DATA + 0x100 + 0xc, r.next_u32() % 50);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    /// `InputParser_EmitUnaryOp`: operand compile, register bookkeeping and the
    /// error exits.
    #[test]
    fn input_parser_emit_unary_op_matches() {
        const PARSER: u32 = DATA + 0x2_0000;
        check(
            ee::s10::f_003250d8__generated,
            input_parser_emit_unary_op,
            400,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x0032_50d8, 2);
                c.pc = 0x0032_50d8;
                c.g[A0][0] = PARSER as u64;
                c.g[A1][0] = (DATA + 0x600) as u64;
                c.g[A2][0] = (DATA + 0x700) as u64;
                c.g[A3][0] = (DATA + 0x800) as u64;
                let cap = [0xffff_ffffu32, 0, 1, 2, 5][(seed % 5) as usize];
                c.g[T0][0] = cap as u64;
                c.g[T1][0] = r.next_u32() as u64;
                c.g[T2][0] = r.next_u32() as u64;
                w32_2(c, DATA + 0x604, DATA + 0x900);
                w32_2(
                    c,
                    DATA + 0x700,
                    if seed % 2 == 0 {
                        0xffff_fff0
                    } else {
                        r.next_u32() % 64
                    },
                );
                w32_2(c, PARSER + 0x1_09e8, r.next_u32() % 4);
                for k in 0..6 {
                    w32_2(c, PARSER + 0x1_08ec + 4 * k, r.next_u32() % 64);
                }
                let n = [0, 1, 2, 3, 0xffff_ffff][((seed / 5) % 5) as usize];
                w32_2(c, oc410_result(21), n);
                w32_2(
                    c,
                    oc410_out(21),
                    [5, 0x3f, 0x40, 0xffff_ffff][((seed / 3) % 4) as usize],
                );
            },
        );
    }

    /// `InputParser_ParseFunction2`: the first-token error (the current token
    /// is not `(`), and `(` followed by an empty input (the first operand
    /// fails to parse). The folding paths need a tokenised expression and are
    /// not reached by this fixture.
    #[test]
    fn input_parser_parse_function_2_matches() {
        check(
            ee::s10::f_00324138__generated,
            input_parser_parse_function_2,
            24,
            &|c, seed| {
                let _ = machine(c, seed, 0x324138, 0);
                elf_image(c);
                oc414_env(c);
                c.pc = 0x0032_4138;
                let this = DATA + 0x100;
                c.bus
                    .write32(this + 0x98, if seed % 3 == 0 { 5 } else { 9 });
                c.g[A0][0] = sx(this);
                c.g[A1][0] = [20u64, 21, 22, 23][(seed % 4) as usize];
            },
        );
    }

    use crate::difftest::fakes::covered;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc427_base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc427_gp;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc427_queue;

    /// `InputParser_ParseFunction1`: `(` expected, the operand parse failing,
    /// `)` expected (the error frees the operand), and the folding of
    /// constants for every function op (0x18..0x21, floor/ceil/wrap over
    /// random floats) against other ops and operand types (a unary node is
    /// built). Entered at the function start only.
    #[test]
    fn input_parser_parse_function1_matches() {
        covered(
            ee::s10::f_00324398__generated,
            input_parser_parse_function1,
            240,
            &|c, seed| {
                let mut r = oc427_base(c, seed, 0x324398, 6, 0x0032_4398);
                let this = DATA + 0x1000;
                let operand = DATA + 0x2000;
                let node = DATA + 0x3000;
                let handlers = [
                    0x0032_4460u32,
                    0x0032_4470,
                    0x0032_4480,
                    0x0032_4490,
                    0x0032_44a0,
                    0x0032_44b0,
                    0x0032_44dc,
                    0x0032_450c,
                    0x0032_451c,
                    0x0032_452c,
                ];
                for (k, h) in handlers.iter().enumerate() {
                    c.bus.write32(0x0048_e1e0 + 4 * k as u32, *h);
                }
                c.bus.write32(oc427_gp(-0x2e2c), 0x3e22_f983);
                c.bus.write32(oc427_gp(-0x2e28), 0x40c9_0fdb);
                let first = if seed % 7 == 0 { 5 } else { 9 };
                c.bus.write32(this + 0x98, first);
                // after `(`: a token, the operand (or a failed parse), then `)`
                let second = if seed % 5 == 0 { 3 } else { 10 };
                oc427_queue(c, 0x0032_1af0, &[second, 1]);
                let fold = second == 10 && seed % 4 != 0;
                let ty = if fold { 4 } else { 0x40 };
                c.bus.write32(operand, ty);
                c.bus.write32(operand + 4, r.edge_f32());
                oc427_queue(
                    c,
                    0x0032_3268,
                    &[if seed % 11 == 0 { 0 } else { operand as u64 }],
                );
                oc427_queue(c, 0x0032_15f8, &[node as u64]);
                let op = if seed % 6 == 0 {
                    0x40 + r.next_u32() % 4
                } else {
                    0x18 + r.next_u32() % 10
                };
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(op);
            },
        );
    }
}
