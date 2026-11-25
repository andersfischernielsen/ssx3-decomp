//! `src/input/inputmap.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::shl32;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::render::lightman::addu;
pub(crate) use crate::game::render::lightman::off;
pub(crate) use crate::game::render::lightman::Oc49Mem;
pub(crate) use crate::game::sound::bankmonitor::p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0032_0ca0, input_map_init, "InputMap_Init"),
    (
        0x0032_0f10,
        input_map_finalize_opcodes,
        "InputMap_FinalizeOpcodes",
    ),
    (0x0032_0fa8, input_map_eval_float, "InputMap_EvalFloat"),
    (0x0032_1108, input_map_eval_bool, "InputMap_EvalBool"),
    (0x0032_1298, input_pad_update, "InputPad_Update"),
    (0x0032_1500, input_parser_error, "InputParser_Error"),
    (
        0x0032_15f8,
        input_parser_alloc_node,
        "InputParser_AllocNode",
    ),
    (
        0x0032_16e8,
        input_parser_lookup_button,
        "InputParser_LookupButton",
    ),
    (
        0x0032_0e18,
        input_map_compile_entry,
        "InputMap_CompileEntry",
    ),
    (0x0032_1428, input_parser_ctor, "InputParser_ctor"),
    (
        0x0032_0db0,
        input_map_create_parser_2,
        "InputMap_CreateParser",
    ),
    (0x0032_0d08, input_map_free, "InputMap_Free"),
    (0x0032_0d50, input_map_parse_file, "InputMap_ParseFile"),
    (
        0x0032_1010,
        input_map_eval_float_args,
        "InputMap_EvalFloatArgs",
    ),
    (
        0x0032_1188,
        input_map_eval_bool_args,
        "InputMap_EvalBoolArgs",
    ),
    (
        0x0032_1590,
        input_parser_skip_statement,
        "InputParser_SkipStatement",
    ),
    (0x0032_1638, input_parser_free_node, "InputParser_FreeNode"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {

    pub(crate) use crate::game::be::beintnetwork::callee::INPUT_VM_EXECUTE;
    pub(crate) use crate::game::be::beintnetwork::callee::INPUT_VM_INIT;

    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_4 as EA_MEM_FREE;

    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2 as MEMCPY;

    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;

    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_5 as SPRINTF;

    pub(crate) use crate::game::fe::feasyncfile::callee::VSPRINTF;

    pub(crate) use crate::game::render::lightman::callee::EA_MEMSET;

    pub(crate) use crate::game::sound::bankmonitor::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;

    pub(crate) use crate::game::sound::bankmonitor::callee::MEMSET_2 as MEMSET;
}

/// `sInputPad` button array starts at `+4`, stride 0x1c.
pub(crate) mod pad {

    pub const BUTTON_SIZE: u32 = 0x1c;
}

// ---------------------------------------------------------------------------
// Input VM.

/// `float InputMap_EvalFloat(InputMap *map, int value, int binding)`
/// (0x320fa8): run the binding's operand list through the input VM and return
/// its result.
pub fn input_map_eval_float(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x1d0);
        frame.save(c, S1, 0x1b0);
        frame.save(c, S0, 0x1c0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x1a0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = c.g[SP][0];
        frame.save_ra(c, 0x190);
        c.g[S2][0] = c.g[A1][0];
        callee::INPUT_VM_INIT.call(c, 0x0032_0fd0)?;
        // The operand list of binding `a2`: `ops + 4 * table[a2]`.
        c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x14).lw(c);
        c.g[A1][0] = c.g[S2][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        callee::INPUT_VM_EXECUTE.call(c, 0x0032_0ff8)?;
        frame.restore(c, S0, 0x1c0);
        frame.restore(c, S1, 0x1b0);
        frame.restore(c, S2, 0x1a0);
        frame.restore_ra(c, 0x190);
        frame.close(c);
        Ok(())
    })
}

/// `bool InputMap_EvalBool(InputMap *map, int value, int binding)`
/// (0x321108): [`input_map_eval_float`] with the result compared against 0.
pub fn input_map_eval_bool(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x1d0);
        frame.save(c, S1, 0x1b0);
        frame.save(c, S0, 0x1c0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x1a0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = c.g[SP][0];
        frame.save_ra(c, 0x190);
        c.g[S2][0] = c.g[A1][0];
        callee::INPUT_VM_INIT.call(c, 0x0032_1130)?;
        c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x14).lw(c);
        c.g[A1][0] = c.g[S2][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        callee::INPUT_VM_EXECUTE.call(c, 0x0032_1158)?;
        c.mtc1(ZERO, 1); // f1 = 0.0
        c.c_eq_s(0, 1);
        let mut result = 1u64;
        if c.fcc() {
            // The VM's value is exactly 0.
            c.cycles += 1;
            result = 0;
        }
        c.g[V0][0] = result;
        c.cycles += 6;
        frame.restore(c, S0, 0x1c0);
        frame.restore(c, S1, 0x1b0);
        frame.restore(c, S2, 0x1a0);
        frame.restore_ra(c, 0x190);
        frame.close(c);
        Ok(())
    })
}

/// Ticks after an edge during which a button's value is not compared (60 Hz
/// frames; `slti 3` in the original).
const SETTLE_FRAMES: u32 = 3;
/// A held button repeats first after 24 and then every 12 60 Hz frames.
const REPEAT_DELAY_FRAMES: u32 = 0x18;
const REPEAT_PERIOD_FRAMES: u32 = 0xc;

/// `void InputPad_Update(sInputPad *self, int n, float *values)`
/// (0x00321298): resize the button array, then per button store the value,
/// run the debounce/settle logic and the auto-repeat timer.
///
/// Tick rate: the settle length and the repeat delay/period are 60 Hz frame
/// counts, converted with `tick::ticks_from_frames60` (exact at 60), so the
/// debounce and the held-direction menu repeat keep their wall-clock times.
pub fn input_pad_update(c: &mut Cpu) {
    use pad::*;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.ldw(c.g[S0][0] as u32);
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S1][0] as i64)) as u64;
        let fits = c.g[V0][0] == 0;
        c.g[S2][0] = c.g[A2][0];
        if !fits {
            c.cycles += 9;
            c.g[A2][0] = BUTTON_SIZE as u64;
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
            c.mult(0, A0, A0, A2);
            c.g[A1][0] = 0;
            c.mult(0, 0, V0, A2);
            c.g[A0][0] = sx(off(c.g[A0][0] as u32, 4));
            c.g[A2][0] = c.lo[0];
            c.g[A0][0] = addu(c.g[S0][0], c.g[A0][0]);
            callee::EA_MEMSET.call(c, 0x0032_12e8)?;
        }
        c.cycles += 3;
        c.stw(c.g[S0][0] as u32, c.g[S1][0]);
        c.g[T4][0] = 0;
        let empty = (c.g[S1][0] as i64) <= 0;
        if !empty {
            c.cycles += 11;
            c.g[T5][0] = sx(off(c.g[S0][0] as u32, 0x18));
            c.mtc1(0, 1);
            c.g[A2][0] = c.g[S2][0];
            c.g[T3][0] = 1;
            c.g[T2][0] = sx(off(c.g[S0][0] as u32, 0x14));
            c.g[A3][0] = c.g[T5][0];
            c.g[T0][0] = 0;
            c.g[T1][0] = sx(off(c.g[S0][0] as u32, 0x10));
            c.g[A0][0] = sx(off(c.g[S0][0] as u32, 0x1c));
            // Debounce and auto-repeat lengths, authored in 60 Hz frames
            // (3, 24, 12): the same wall-clock times at any rate.
            let settle = crate::tick::ticks_from_frames60(c, SETTLE_FRAMES) as i64;
            c.g[T7][0] = crate::tick::ticks_from_frames60(c, REPEAT_DELAY_FRAMES) as u64;
            c.g[T6][0] = crate::tick::ticks_from_frames60(c, REPEAT_PERIOD_FRAMES) as u64;
            loop {
                // value = *values; settle/timer below.
                c.cycles += 5;
                c.lwc1(0, c.g[A2][0] as u32);
                c.g[V1][0] = c.ldw(c.g[A0][0] as u32);
                c.g[V0][0] = ((c.g[V1][0] as i64) < settle) as u64;
                let settling = c.g[V0][0] != 0;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x18));
                if settling {
                    c.cycles += 6;
                    c.g[V0][0] = sx(off(c.g[V1][0] as u32, 1));
                    c.stw((c.g[A0][0] as u32).wrapping_sub(0x14), 0);
                    c.stw(c.g[A0][0] as u32, c.g[V0][0]);
                    c.g[A1][0] = sx(off(c.g[S0][0] as u32, 0x10));
                    c.stw((c.g[A0][0] as u32).wrapping_sub(0x10), 0);
                } else {
                    // on = 0 < value
                    c.cycles += 4;
                    c.c_lt_s(1, 0);
                    let on = c.fcc();
                    c.g[V1][0] = on as u64;
                    if !on {
                        c.cycles += 1;
                        c.g[V1][0] = 0;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.ldw((c.g[A0][0] as u32).wrapping_sub(0xc));
                    let changed = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = sx(off(c.g[S0][0] as u32, 0x10));
                    if !changed {
                        c.cycles += 3;
                        c.stw((c.g[A0][0] as u32).wrapping_sub(0x14), 0);
                        c.stw((c.g[A0][0] as u32).wrapping_sub(0x10), 0);
                    } else {
                        c.cycles += 2;
                        let released = c.g[V1][0] == 0;
                        c.stw(c.g[T1][0] as u32, c.g[V1][0]);
                        if released {
                            c.cycles += 2;
                            c.stw((c.g[A0][0] as u32).wrapping_sub(0x14), 0);
                            c.stw((c.g[A0][0] as u32).wrapping_sub(0x10), c.g[T3][0]);
                        } else {
                            c.cycles += 3;
                            c.stw((c.g[A0][0] as u32).wrapping_sub(0x14), c.g[T3][0]);
                            c.stw((c.g[A0][0] as u32).wrapping_sub(0x10), 0);
                        }
                        c.cycles += 1;
                        c.stw(c.g[A0][0] as u32, 0);
                    }
                }
                // Auto-repeat.
                c.cycles += 4;
                c.g[V0][0] = addu(c.g[A1][0], c.g[T0][0]);
                c.g[V1][0] = c.ldw(c.g[V0][0] as u32);
                let up = c.g[V1][0] == 0;
                c.g[V0][0] = sx(off(c.g[T2][0] as u32, c.g[T0][0] as u32));
                if up {
                    c.cycles += 3;
                    c.g[V1][0] = sx(off(c.g[T5][0] as u32, c.g[T0][0] as u32));
                    c.stw(c.g[V0][0] as u32, 0);
                    c.stw(c.g[V1][0] as u32, 0);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.ldw((c.g[A0][0] as u32).wrapping_sub(4));
                    let has_timer = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    if !has_timer {
                        c.cycles += 3;
                        c.stw((c.g[A0][0] as u32).wrapping_sub(8), c.g[T3][0]);
                        c.stw((c.g[A0][0] as u32).wrapping_sub(4), c.g[T7][0]);
                    } else {
                        c.cycles += 2;
                        let still = (c.g[V0][0] as i64) > 0;
                        c.stw(c.g[A3][0] as u32, c.g[V0][0]);
                        if still {
                            c.cycles += 3;
                            c.g[V0][0] = sx(off(c.g[T2][0] as u32, c.g[T0][0] as u32));
                            c.stw(c.g[V0][0] as u32, 0);
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = sx(off(c.g[T2][0] as u32, c.g[T0][0] as u32));
                            c.stw(c.g[V0][0] as u32, c.g[T3][0]);
                            c.stw(c.g[A3][0] as u32, c.g[T6][0]);
                        }
                    }
                }
                c.cycles += 8;
                c.g[T4][0] = sx(off(c.g[T4][0] as u32, 1));
                c.g[A3][0] = sx(off(c.g[A3][0] as u32, BUTTON_SIZE));
                c.g[T0][0] = sx(off(c.g[T0][0] as u32, BUTTON_SIZE));
                c.g[T1][0] = sx(off(c.g[T1][0] as u32, BUTTON_SIZE));
                c.g[A0][0] = sx(off(c.g[A0][0] as u32, BUTTON_SIZE));
                c.g[V0][0] = ((c.g[T4][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = sx(off(c.g[A2][0] as u32, 4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_1320);
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

/// `void* InputParser_AllocNode(InputParser* self)` (0x3215f8): pop the free
/// list at `+0xe0`, clear the node (0x10 bytes) and return it.
pub fn input_parser_alloc_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x10;
        c.g[S0][0] = Addr(r32(c, V0)).off(0xe0).lw(c);
        c.g[V1][0] = Addr(r32(c, S0)).off(4).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.sw(p(r32(c, V0), 0xe0), c.g[V1][0]);
        callee::MEMSET.call(c, 0x0032_1624)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void InputMap_Init(InputMap* self, int count)` (0x320ca0): size the map
/// array, allocate `count * 4` bytes for it and fill the buffer with -1.
pub fn input_map_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x20);
        c.g[V0][0] = c.g[A1][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = 0x490000;
        c.g[S0][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2300));
        c.g[RA][0] = sx(0x0032_0cd0);
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        call_ptr(c, 0x0031_7e30, 0x0032_0cd0)?;
        c.g[A2][0] = c.lw(c.g[S0][0] as u32);
        c.g[A1][0] = 0xffff_ffff_ffff_0000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
        c.g[RA][0] = sx(0x0032_0cec);
        c.g[A1][0] = c.g[A1][0] | 0xffff;
        call_ptr(c, 0x003e_6448, 0x0032_0cec)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x20);
        Ok(())
    })
}

/// `void InputMap_CreateParser(InputMap* self, name, arg)` (0x320db0):
/// allocate a 0x109ec-byte parser from the game heap and construct it.
pub fn input_map_create_parser(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x40);
        c.g[A3][0] = 0u64;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S2][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = 0x490000;
        c.g[A0][0] = 0x10000;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x22f0));
        c.g[A0][0] = c.g[A0][0] | 0x9ec;
        c.g[RA][0] = sx(0x0032_0dec);
        c.g[A2][0] = 0x100;
        call_ptr(c, 0x0031_7d70, 0x0032_0dec)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0032_0dfc);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0032_1428, 0x0032_0dfc)?;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x40);
        Ok(())
    })
}

/// `void InputParser_Error(InputParser* self, const char* fmt, ...)`
/// (0x321500): format the parser's name and the message into a local buffer
/// and flag the parser as failed.
pub fn input_parser_error(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x4b0);
        let self_ = abi::arg(c, 0);
        let fmt = abi::arg64(c, 1);
        let a2 = abi::arg64(c, 2);
        let a3 = abi::arg64(c, 3);
        let t0 = abi::arg64(c, 4);
        let t1 = abi::arg64(c, 5);
        let t2 = abi::arg64(c, 6);
        let t3 = abi::arg64(c, 7);
        frame.save(c, S0, 0x430);
        frame.save(c, S1, 0x420);
        c.g[S0][0] = sx(self_);
        frame.save(c, S2, 0x410);
        c.g[S1][0] = sx(frame.at(c, 0x480));
        frame.save_ra(c, 0x400);
        c.g[S2][0] = fmt;
        c.sd(frame.at(c, 0x488), a3);
        c.g[A1][0] = 0x0049_0000;
        c.g[A3][0] = c.lw(at(c, S0, 0x8c));
        c.g[A0][0] = sx(frame.at(c, 0));
        c.sd(frame.at(c, 0x480), a2);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x22c0));
        c.sd(frame.at(c, 0x490), t0);
        c.g[A2][0] = sx(p(r32(c, S0), 0xc));
        c.sd(frame.at(c, 0x498), t1);
        c.sd(frame.at(c, 0x4a0), t2);
        c.sd(frame.at(c, 0x4a8), t3);
        frame.save_fpr(c, 12, 0x470);
        frame.save_fpr(c, 14, 0x474);
        frame.save_fpr(c, 16, 0x478);
        frame.save_fpr(c, 18, 0x47c);
        call_ptr(c, callee::SPRINTF, 0x0032_1560)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = sx(r32(c, SP).wrapping_add(r32(c, V0)));
        call_ptr(c, callee::VSPRINTF, 0x0032_1570)?;
        c.g[V1][0] = 1;
        frame.restore(c, S1, 0x420);
        c.sw(at(c, S0, 0x94), c.g[V1][0]);
        frame.restore(c, S0, 0x430);
        frame.restore(c, S2, 0x410);
        frame.restore_ra(c, 0x400);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Input map file

/// `InputMap_FinalizeOpcodes` (0x320f10): replace the map's binding array:
/// delete the old list, allocate a copy of the new one, copy it in and free
/// the temporary. `this+8` is the old array, `+0x10` the new array (word
/// count), `+0x14` the source list to copy from.
pub fn input_map_finalize_opcodes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);

        let old = Addr(p(this, 8)).lw(c);
        if old == 0 {
            c.g[A0][0] = Addr(p(this, 0x10)).lw(c);
        } else {
            c.cycles += 3;
            c.g[A0][0] = old;
            c.g[RA][0] = sx(0x0032_0f38);
            callee::BUILTIN_DELETE.call(c, 0x0032_0f38)?;
            c.g[A0][0] = Addr(p(this, 0x10)).lw(c);
        }

        // The new array: `count * 4` bytes from the named pool.
        c.cycles += 16;
        c.g[A1][0] = 0x0049_0000;
        Addr(p(this, 8)).sw(c, 0);
        c.g[A2][0] = 0;
        c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(0x22d0));
        c.g[A0][0] = shl32(c.g[A0][0], 2);
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x0032_0f58);
        callee::OPERATOR_NEW_NAMED.call(c, 0x0032_0f58)?;
        c.g[A2][0] = Addr(p(this, 0x10)).lw(c);
        let array = c.g[V0][0];
        c.g[S1][0] = array;
        c.g[A1][0] = Addr(p(this, 0x14)).lw(c);
        c.g[A0][0] = array;
        c.g[RA][0] = sx(0x0032_0f70);
        c.g[A2][0] = shl32(c.g[A2][0], 2);
        call_ptr(c, callee::MEMCPY, 0x0032_0f70)?;
        c.g[A0][0] = Addr(p(this, 0x14)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0032_0f84);
            callee::EA_MEM_FREE.call(c, 0x0032_0f84)?;
        }
        c.g[V0][0] = Addr(p(this, 0x10)).lw(c);
        c.cycles += 7;
        Addr(p(this, 0x14)).sw(c, array);
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Input parser.

/// `int InputParser_LookupButton(const char* name)` (0x3216e8): parse
/// `"buttonNN[aA]?"` and return the button index, or -1 when the name does
/// not have the `button` prefix, has no digits, ends in a letter, or is `_`.
pub fn input_parser_lookup_button(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 6;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(r(c, A1).wrapping_add(0x3ef0));
        call_ptr(c, 0x0041_ab78, 0x0032_170c)?;

        let mismatch = r(c, V0) != 0;
        c.g[V0][0] = u64::MAX;
        if mismatch {
            return input_parser_tail(c, frame);
        }
        c.cycles += 6;
        c.g[V1][0] = c.lb(Addr(r(c, S0)).off(0x6).0);
        c.g[V1][0] = sx(r(c, V1).wrapping_sub(0x30));
        c.g[V0][0] = c.g[V1][0] & 0xff;
        c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
        let no_digit = c.g[V0][0] == 0;
        c.g[V0][0] = u64::MAX;
        if no_digit {
            return input_parser_tail(c, frame);
        }

        'number: {
            c.cycles += 6;
            c.g[V0][0] = c.lb(Addr(r(c, S0)).off(0x7).0);
            c.g[A2][0] = c.g[V1][0];
            c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x30));
            c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
            let no_more = c.g[V0][0] == 0;
            c.g[A1][0] = sx(p(r(c, S0), 0x7));
            if no_more {
                break 'number;
            }
            c.cycles += 1;
            c.g[A3][0] = 0xa;
            loop {
                c.cycles += 9;
                c.mult(0, V1, A2, A3);
                c.g[A0][0] = c.lb(r(c, A1));
                c.g[A1][0] = sx(p(r(c, A1), 0x1));
                c.g[V0][0] = c.lb(r(c, A1));
                c.g[V1][0] = sx(r(c, V1).wrapping_sub(0x30));
                c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x30));
                c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = sx(p(r(c, V1), r(c, A0)));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_1748);
                }
            }
        }

        'alpha: {
            'inner: {
                c.cycles += 6;
                c.g[V1][0] = c.lb(r(c, A1));
                c.g[A3][0] = 0;
                c.g[V0][0] = sx(r(c, V1).wrapping_sub(0x61));
                c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
                let lower = c.g[V0][0] != 0;
                c.g[A0][0] = c.lbu(r(c, A1));
                if lower {
                    break 'inner;
                }
                c.cycles += 4;
                c.g[V0][0] = sx(r(c, V1).wrapping_sub(0x41));
                c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
                let not_upper = c.g[V0][0] == 0;
                if not_upper {
                    break 'alpha;
                }
            }
            c.cycles += 1;
            c.g[A3][0] = 1;
        }

        c.cycles += 2;
        let letter = c.g[A3][0] != 0;
        c.g[V0][0] = u64::MAX;
        if letter {
            return input_parser_tail(c, frame);
        }

        'end: {
            c.cycles += 5;
            c.g[V0][0] = sx(r(c, A0) << 24);
            c.g[V1][0] = 0x5f;
            c.g[V0][0] = sx((r(c, V0) as i32 >> 24) as u32);
            let underscore = c.g[V0][0] == c.g[V1][0];
            c.g[V1][0] = ((c.g[A2][0] as i64) < 924) as u64;
            if underscore {
                break 'end;
            }
            c.cycles += 2;
            let small = c.g[V1][0] != 0;
            c.g[V0][0] = c.g[A2][0];
            if small {
                return input_parser_tail(c, frame);
            }
        }
        c.cycles += 1;
        c.g[V0][0] = u64::MAX;
        input_parser_tail(c, frame)
    })
}

/// The `lq s0; ld ra; addiu sp; jr ra` tail.
pub(crate) fn input_parser_tail(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    frame.restore(c, 16, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// InputMap_CompileEntry (0x320e18)

/// `operator new(size, name, align, flags)` (0x317e30).
pub(crate) const OPERATOR_NEW_NAMED: Callee = Callee {
    addr: 0x0031_7e30,
    f: ee::s10::operator_new_named,
};

/// `EA_MemFree(ptr)` (0x317e98).
pub(crate) const EA_MEM_FREE_CALLEE: Callee = Callee {
    addr: 0x0031_7e98,
    f: ee::s10::EA_MemFree,
};

/// `cInputMapParser::compileStatement` (0x3245a8).
pub(crate) const INPUT_PARSER_COMPILE_IDENTIFIER: Callee = Callee {
    addr: 0x0032_45a8,
    f: ee::s10::InputParser_CompileIdentifier,
};

/// `EA_MemCopy(dst, src, bytes)` (0x3e6574; called through the table).
pub(crate) const EA_MEM_COPY_ADDR: u32 = 0x003e_6574;

/// InputMap fields: offsets table (+4), parser (+8), capacity (+0xc), used
/// (+0x10), code buffer (+0x14), all in words.
pub(crate) const INPUT_MAP_OFFSETS: u32 = 0x4;

pub(crate) const INPUT_MAP_PARSER: u32 = 0x8;

pub(crate) const INPUT_MAP_CAP: u32 = 0xc;

pub(crate) const INPUT_MAP_USED: u32 = 0x10;

pub(crate) const INPUT_MAP_CODE: u32 = 0x14;

/// `void InputMap::CompileEntry(this, idx, src)` (0x320e18): grow the opcode
/// buffer by 0x400 words when less than 0x100 remain, record the current
/// `used` as `offsets[idx]`, compile the statement at the end of the buffer
/// and add the words it produced to `used`.
pub fn input_map_compile_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S3, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_CAP));
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_USED));
        c.g[V0][0] = sx(r32(c, A0).wrapping_sub(r32(c, V0)));
        c.g[V0][0] = ((c.g[V0][0] as i64) < 256) as u64;
        let roomy = c.g[V0][0] == 0;
        c.g[S4][0] = c.g[A2][0];
        if !roomy {
            // Fewer than 0x100 words left: allocate a bigger buffer.
            c.cycles += 10;
            c.g[S2][0] = sx(r32(c, A0).wrapping_add(0x400));
            c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(0x22e0));
            c.g[A0][0] = sx(r32(c, S2) << 2);
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            OPERATOR_NEW_NAMED.call(c, 0x0032_0e70)?;
            c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_CODE));
            let no_old = c.g[A1][0] == 0;
            c.g[S1][0] = c.g[V0][0];
            let mut store_code = true;
            if !no_old {
                c.cycles += 7;
                c.g[A2][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_CAP));
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = sx(r32(c, A2) << 2);
                call_ptr(c, EA_MEM_COPY_ADDR, 0x0032_0e8c)?;
                c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_CODE));
                if c.g[A0][0] == 0 {
                    let new_code = c.g[S1][0];
                    c.sw(r32(c, S0).wrapping_add(INPUT_MAP_CODE), new_code);
                    store_code = false;
                } else {
                    c.cycles += 2;
                    EA_MEM_FREE_CALLEE.call(c, 0x0032_0ea0)?;
                }
            }
            if store_code {
                c.cycles += 1;
                let new_code = c.g[S1][0];
                c.sw(r32(c, S0).wrapping_add(INPUT_MAP_CODE), new_code);
            }
            c.cycles += 1;
            let new_cap = c.g[S2][0];
            c.sw(r32(c, S0).wrapping_add(INPUT_MAP_CAP), new_cap);
        }
        c.cycles += 26;
        c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_OFFSETS));
        c.g[V0][0] = sx(r32(c, S3) << 2);
        c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_USED));
        c.g[A1][0] = c.g[S4][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[T0][0] = 5;
        let used = c.g[A0][0];
        c.sw(r32(c, V0), used);
        c.g[A3][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_USED));
        c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_CAP));
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_CODE));
        c.g[A2][0] = sx(r32(c, A3) << 2);
        c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_PARSER));
        c.g[A3][0] = sx(r32(c, V1).wrapping_sub(r32(c, A3)));
        c.g[A2][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
        INPUT_PARSER_COMPILE_IDENTIFIER.call(c, 0x0032_0ee4)?;
        c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_USED));
        frame.restore(c, S1, 0x40);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        frame.restore(c, S2, 0x30);
        let used = c.g[V1][0];
        c.sw(r32(c, S0).wrapping_add(INPUT_MAP_USED), used);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `InputParser* InputParser_ctor(InputParser* self, char* file, int length)`: sets the source and length, builds the free-id stack (63..1) and the 0x800-node free list (the loop at 0x321480 is `ids[n++] = i`), and parses the whole file.
/// (`InputParser_ctor`, 0x00321428)
pub fn input_parser_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.g[V0][0] = 0x8000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = 0x10000;
        c.g[S0][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.g[A0][0] = 0x1;
        c.sw(c.g[S0][0] as u32, c.g[A1][0]);
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0xe4), 0u64);
        c.g[A1][0] = c.g[V1][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x90), c.g[A0][0]);
        c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8ec));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8c), c.g[A0][0]);
        c.sb((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
        c.g[A0][0] = 0x3f;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x94), 0u64);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x9e8), 0u64);
        'l_321480: loop {
            c.cycles += 8;
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x9e8));
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            c.sw(c.g[V0][0] as u32, c.g[A0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
            let t = (c.g[A0][0] as i64) > 0;
            c.sw((c.g[A1][0] as u32).wrapping_add(0x9e8), c.g[V1][0]);
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00321480);
                }
                continue 'l_321480;
            } else {
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xe4));
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xe8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xe0), c.g[V0][0]);
                c.g[A0][0] = 0x7fe;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xf4));
                'l_3214b8: loop {
                    c.cycles += 7;
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                    let t = (c.g[A0][0] as i64) >= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003214b8);
                        }
                        continue 'l_3214b8;
                    } else {
                        c.cycles += 10;
                        c.g[V0][0] = 0x8000;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0xd8), 0u64);
                        Callee {
                            addr: 0x00323098,
                            f: ee::s10::InputParser_ParseAll,
                        }
                        .call(c, 0x003214e8)?;
                        c.g[V0][0] = c.g[S0][0];
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    })
}

/// `void InputMap_CreateParser(InputMap* self, char* mem, int len)` (0x320db0): allocates the
/// 0x109ec-byte input parser (256-aligned) and constructs it over `mem` / `len`; the
/// parser goes to `self + 0x8`.
pub fn input_map_create_parser_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[A3][0] = 0u64;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S2][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = 0x490000;
        c.g[A0][0] = 0x10000;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x22f0));
        c.g[A0][0] = c.g[A0][0] | 0x9ec;
        c.g[A2][0] = 0x100;
        Callee {
            addr: 0x00317d70,
            f: ee::s10::EA_MemAllocNamed,
        }
        .call(c, 0x00320dec)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x00321428,
            f: ee::s10::InputParser_ctor,
        }
        .call(c, 0x00320dfc)?;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `str_InputMap`, the allocation tag of the map (0x480000 - 0x6550).
pub(crate) const STR_INPUT_MAP: u64 = 0x0047_9ab0;

/// The bindings `InputMap_LoadFromFile` compiles, in index order: the return
/// address of each `InputMap_CompileEntry` call (the generated code's layout),
/// the address of the binding's name string, and the name (the reference's
/// `str_` identifier without its prefix).
pub(crate) const COMPILE_BINDINGS: [(u32, u32, &str); 205] = [
    (0x00226bdc, 0x00479ac0, "CruiseTurn"),
    (0x00226bf0, 0x00479ad0, "CruiseCrouch"),
    (0x00226c04, 0x00479ae0, "CruiseBrake"),
    (0x00226c18, 0x00479af0, "BoardPress"),
    (0x00226c2c, 0x00479b00, "BoardPivot"),
    (0x00226c40, 0x00479b10, "AttackLeft"),
    (0x00226c54, 0x00479b20, "AttackRight"),
    (0x00226c68, 0x00479b30, "PrewindTurn"),
    (0x00226c7c, 0x00479b40, "PrewindSpin"),
    (0x00226c90, 0x00479b50, "PrewindFlip"),
    (0x00226ca4, 0x00479b60, "AirAdjRotFB"),
    (0x00226cb8, 0x00479b70, "AirAdjRotLR"),
    (0x00226ccc, 0x00479b80, "LateSpin"),
    (0x00226ce0, 0x00479b90, "RailSpin"),
    (0x00226cf4, 0x004a28b8, "Spin"),
    (0x00226d08, 0x004a28c0, "Flip"),
    (0x00226d1c, 0x00479ba0, "WipeoutRecover"),
    (0x00226d30, 0x00479bb0, "JumpPressed"),
    (0x00226d44, 0x00479bc0, "JumpHeld"),
    (0x00226d58, 0x00479bd0, "BoostPressed"),
    (0x00226d6c, 0x00479be0, "BoostHeld"),
    (0x00226d80, 0x004a28c8, "Tweak"),
    (0x00226d94, 0x00479bf0, "OllieHeld"),
    (0x00226da8, 0x00479c00, "RailBalance"),
    (0x00226dbc, 0x00479c10, "HandplantBalance"),
    (0x00226dd0, 0x00479c28, "Handplant"),
    (0x00226de4, 0x00479c38, "GateAnticipate"),
    (0x00226df8, 0x00479c48, "UberGrind1"),
    (0x00226e0c, 0x00479c58, "UberGrind2"),
    (0x00226e20, 0x00479c68, "UberGrind3"),
    (0x00226e34, 0x00479c78, "UberGrind4"),
    (0x00226e48, 0x004a28d0, "Trick1"),
    (0x00226e5c, 0x004a28d8, "Trick2"),
    (0x00226e70, 0x004a28e0, "Trick3"),
    (0x00226e84, 0x004a28e8, "Trick4"),
    (0x00226e98, 0x004a28f0, "Trick5"),
    (0x00226eac, 0x004a28f8, "Trick6"),
    (0x00226ec0, 0x004a2900, "Trick7"),
    (0x00226ed4, 0x004a2908, "Trick8"),
    (0x00226ee8, 0x004a2910, "Trick9"),
    (0x00226efc, 0x004a2918, "Trick10"),
    (0x00226f10, 0x004a2920, "Trick11"),
    (0x00226f24, 0x004a2928, "Trick12"),
    (0x00226f38, 0x004a2930, "Trick13"),
    (0x00226f4c, 0x004a2938, "Trick14"),
    (0x00226f60, 0x004a2940, "Trick15"),
    (0x00226f74, 0x00479c88, "ResetPath"),
    (0x00226f88, 0x00479c98, "CameraActivate"),
    (0x00226f9c, 0x00479ca8, "CameraTurn"),
    (0x00226fb0, 0x00479cb8, "CameraPitch"),
    (0x00226fc4, 0x00479cc8, "CameraMove"),
    (0x00226fd8, 0x00479cd8, "CameraStrafe"),
    (0x00226fec, 0x00479ce8, "CameraBoost"),
    (0x00227000, 0x00479cf8, "CameraInGameTurn"),
    (0x00227014, 0x00479d10, "CameraInGamePitch"),
    (0x00227028, 0x00479d28, "CameraInGameZoom"),
    (0x0022703c, 0x00479d40, "CameraInGameNext"),
    (0x00227050, 0x00479d58, "CameraNextStatCam"),
    (0x00227064, 0x00479d70, "CameraPrevStatCam"),
    (0x00227078, 0x00479d88, "CameraGoToStatCam"),
    (0x0022708c, 0x00479da0, "MenuButton"),
    (0x002270a0, 0x004a2948, "MenuKey"),
    (0x002270b4, 0x00479db0, "MenuLeft"),
    (0x002270c8, 0x00479dc0, "MenuRight"),
    (0x002270dc, 0x004a2950, "MenuUp"),
    (0x002270f0, 0x00479dd0, "MenuDown"),
    (0x00227104, 0x00479de0, "MenuLeftShift"),
    (0x00227118, 0x00479df0, "MenuRightShift"),
    (0x0022712c, 0x00479e00, "EditorHelp"),
    (0x00227140, 0x00479e10, "EditorCursorSnap"),
    (0x00227154, 0x00479e28, "EditorRotateLeft"),
    (0x00227168, 0x00479e40, "EditorRotateRight"),
    (0x0022717c, 0x00479e58, "EditorRotateUp"),
    (0x00227190, 0x00479e68, "EditorRotateDown"),
    (0x002271a4, 0x00479e80, "EditorRotateHold"),
    (0x002271b8, 0x00479e98, "EditorZoomIn"),
    (0x002271cc, 0x00479ea8, "EditorZoomOut"),
    (0x002271e0, 0x00479eb8, "EditorModeOrient"),
    (0x002271f4, 0x00479ed0, "EditorModeSlow"),
    (0x00227208, 0x00479ee0, "EditorModeFast"),
    (0x0022721c, 0x00479ef0, "EditorModeFaster"),
    (0x00227230, 0x00479f08, "EditorModeLeft"),
    (0x00227244, 0x00479f18, "EditorModeRight"),
    (0x00227258, 0x00479f28, "EditorModeUp"),
    (0x0022726c, 0x00479f38, "EditorModeDown"),
    (0x00227280, 0x00479f48, "EditorTeleportAI"),
    (0x00227294, 0x00479f60, "PEAddEvent"),
    (0x002272a8, 0x00479f70, "PEStartPath"),
    (0x002272bc, 0x00479f80, "PEMoveNode"),
    (0x002272d0, 0x00479f90, "PEDeleteNode"),
    (0x002272e4, 0x00479fa0, "PEWaitForNoInput"),
    (0x002272f8, 0x00479fb8, "CVToggleDisplayMode"),
    (0x0022730c, 0x00479fd0, "CVMoveCursor"),
    (0x00227320, 0x00479fe0, "CVTriggerHeld"),
    (0x00227334, 0x00479ff0, "CVTriggerReleased"),
    (0x00227348, 0x0047a008, "CVMoveModelChangeAxis"),
    (0x0022735c, 0x0047a020, "CVMoveModelToggle"),
    (0x00227370, 0x0047a038, "CVMoveModelCancel"),
    (0x00227384, 0x0047a050, "CVEditModelSelect"),
    (0x00227398, 0x0047a068, "CVEditModelHideMenu"),
    (0x002273ac, 0x0047a080, "CVEditModelZeroValue"),
    (0x002273c0, 0x0047a098, "FEAdvance"),
    (0x002273d4, 0x0047a0a8, "ReplayExit"),
    (0x002273e8, 0x0047a0b8, "ReplayPlay"),
    (0x002273fc, 0x0047a0c8, "ReplayFForward"),
    (0x00227410, 0x0047a0d8, "ReplayFBackward"),
    (0x00227424, 0x0047a0e8, "ReplayStepForward"),
    (0x00227438, 0x0047a100, "ReplayStepSlow"),
    (0x0022744c, 0x0047a110, "ReplayCycleCamera"),
    (0x00227460, 0x0047a128, "ReplayDebugLoad"),
    (0x00227474, 0x0047a138, "ReplayDebugSave"),
    (0x00227488, 0x004a2958, "NISSkip"),
    (0x0022749c, 0x004a2960, "UIStart"),
    (0x002274b0, 0x0047a148, "UISelect"),
    (0x002274c4, 0x004a2968, "UIUp"),
    (0x002274d8, 0x004a2970, "UIDown"),
    (0x002274ec, 0x004a2978, "UILeft"),
    (0x00227500, 0x004a2980, "UIRight"),
    (0x00227514, 0x004a2988, "UIUp2"),
    (0x00227528, 0x004a2990, "UIDown2"),
    (0x0022753c, 0x004a2998, "UILeft2"),
    (0x00227550, 0x0047a158, "UIRight2"),
    (0x00227564, 0x004a29a0, "UINext"),
    (0x00227578, 0x0047a168, "UIPrevious"),
    (0x0022758c, 0x0047a178, "UIOption"),
    (0x002275a0, 0x004a29a8, "UIMisc"),
    (0x002275b4, 0x0047a188, "UIRShift1"),
    (0x002275c8, 0x0047a198, "UIRShift2"),
    (0x002275dc, 0x0047a1a8, "UILShift1"),
    (0x002275f0, 0x0047a1b8, "UILShift2"),
    (0x00227604, 0x0047a1c8, "UIRShift1Held"),
    (0x00227618, 0x0047a1d8, "UIRShift2Held"),
    (0x0022762c, 0x0047a1e8, "UILShift1Held"),
    (0x00227640, 0x0047a1f8, "UILShift2Held"),
    (0x00227654, 0x0047a208, "UICheatMode"),
    (0x00227668, 0x0047a218, "UICheatUp"),
    (0x0022767c, 0x0047a228, "UICheatDown"),
    (0x00227690, 0x0047a238, "UICheatLeft"),
    (0x002276a4, 0x0047a248, "UICheatRight"),
    (0x002276b8, 0x0047a258, "UICheatButton1"),
    (0x002276cc, 0x0047a268, "UICheatButton2"),
    (0x002276e0, 0x0047a278, "UICheatButton3"),
    (0x002276f4, 0x0047a288, "UICheatButton4"),
    (0x00227708, 0x0047a298, "UICheatAnyKey"),
    (0x0022771c, 0x0047a2a8, "UICamZoom"),
    (0x00227730, 0x0047a2b8, "UICamRotate"),
    (0x00227744, 0x0047a2c8, "UICamPan"),
    (0x00227758, 0x0047a2d8, "UIUpHeld"),
    (0x0022776c, 0x0047a2e8, "UIDownHeld"),
    (0x00227780, 0x0047a2f8, "UILeftHeld"),
    (0x00227794, 0x0047a308, "UIRightHeld"),
    (0x002277a8, 0x0047a318, "UINextHeld"),
    (0x002277bc, 0x0047a328, "UIPreviousHeld"),
    (0x002277d0, 0x0047a338, "UIOptionHeld"),
    (0x002277e4, 0x0047a348, "UIMiscHeld"),
    (0x002277f8, 0x0047a358, "UINextReleased"),
    (0x0022780c, 0x0047a368, "UIPreviousReleased"),
    (0x00227820, 0x0047a380, "UIOptionReleased"),
    (0x00227834, 0x0047a398, "UIMiscReleased"),
    (0x00227848, 0x0047a3a8, "ScriptMenuKey"),
    (0x0022785c, 0x0047a3b8, "ScriptEditHelp"),
    (0x00227870, 0x0047a3c8, "ScriptEditHeadingDelta"),
    (0x00227884, 0x0047a3e0, "ScriptEditPitchDelta"),
    (0x00227898, 0x0047a3f8, "ScriptEditFOVDelta"),
    (0x002278ac, 0x0047a410, "ScriptEditRollDelta"),
    (0x002278c0, 0x0047a428, "ScriptEditJitterDelta"),
    (0x002278d4, 0x0047a440, "ScriptEditJitterRollDelta"),
    (0x002278e8, 0x0047a460, "ScriptEditManualForwardMove"),
    (0x002278fc, 0x0047a480, "ScriptEditManualSideMove"),
    (0x00227910, 0x0047a4a0, "ScriptEditManualUpMove"),
    (0x00227924, 0x0047a4b8, "ScriptEditTargetForwardMove"),
    (0x00227938, 0x0047a4d8, "ScriptEditTargetSideMove"),
    (0x0022794c, 0x0047a4f8, "ScriptEditTargetUpMove"),
    (0x00227960, 0x0047a510, "ScriptEditTargetCamDistDelta"),
    (0x00227974, 0x0047a530, "ScriptEditSubjectLeadDistDelta"),
    (0x00227988, 0x0047a550, "ScriptEditSubjectLeadHeightDelta"),
    (0x0022799c, 0x0047a578, "ScriptEditSubjectMoveCamForward"),
    (0x002279b0, 0x0047a598, "ScriptEditAlgoLeadDistDelta"),
    (0x002279c4, 0x0047a5b8, "ScriptEditAlgoLeadHeightDelta"),
    (0x002279d8, 0x0047a5d8, "ScriptEditAlgoCamDistDelta"),
    (0x002279ec, 0x0047a5f8, "ScreenCapture"),
    (0x00227a00, 0x0047a608, "VideoCapture"),
    (0x00227a14, 0x0047a618, "NetRiderAnalyzerToggle"),
    (0x00227a28, 0x0047a630, "MemCardKey"),
    (0x00227a3c, 0x0047a640, "TestCameraFB"),
    (0x00227a50, 0x0047a650, "TestCameraLR"),
    (0x00227a64, 0x0047a660, "TestCameraOnOff"),
    (0x00227a78, 0x0047a670, "ChaseCamZoom"),
    (0x00227a8c, 0x0047a680, "ChaseCamLR"),
    (0x00227aa0, 0x0047a690, "ChaseCam"),
    (0x00227ab4, 0x0047a6a0, "ChaseManualCamHeading"),
    (0x00227ac8, 0x0047a6b8, "ChaseManualCamPitch"),
    (0x00227adc, 0x0047a6d0, "ChaseManualCamStrafe"),
    (0x00227af0, 0x0047a6e8, "ChaseManualCamForward"),
    (0x00227b04, 0x0047a700, "ChaseManualCamFast"),
    (0x00227b18, 0x0047a718, "TestTerrainType"),
    (0x00227b2c, 0x0047a728, "MissionDebugPlay"),
    (0x00227b40, 0x0047a740, "MissionDebugPreview"),
    (0x00227b54, 0x0047a758, "MissionDebugRecord"),
    (0x00227b68, 0x0047a770, "MissionDebugAnyPressed"),
    (0x00227b7c, 0x0047a788, "VoipOnOff"),
    (0x00227b90, 0x0047a798, "ConsoleReset"),
    (0x00227ba4, 0x0047a7a8, "ReplayMenuUp"),
    (0x00227bb8, 0x0047a7b8, "ReplayMenuDown"),
    (0x00227bcc, 0x0047a7c8, "ReplayTimeLine"),
];

/// `void InputMap_LoadFromFile(void *self, const char *path)` (0x226b60),
/// `cSSXApp_loadInputMap` in the reference. Reads the map file at `path`
/// (its `#include`s expanded) into a new 0x18-byte `InputMap` initialised for
/// 205 bindings, frees the text, compiles the bindings and purges the file.
/// Returns the map in v0.
///
/// Not registered in `OVERRIDES`: the first callee,
/// `cInputPreProcessor_loadFileSize` (0x325f48, `InputPreproc_Run` in the
/// generated code), reads the file through the resumable preprocessor. It has
/// no override slot, so a test cannot fake it, and running it on test data
/// does not stay bounded.
pub fn input_map_load_from_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[A0][0] = c.g[SP][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0x20);
        // `size` out-parameter in the frame.
        c.g[A2][0] = sx(frame.at(c, 0x10));
        call_ptr(c, 0x0032_5f48, 0x0022_6b7c)?;
        c.g[S1][0] = c.g[V0][0];

        // `new InputMap` (0x18 bytes, tagged str_InputMap), then `init(205)`.
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A1][0] = STR_INPUT_MAP;
        c.g[A0][0] = 0x18;
        call_ptr(c, 0x0031_7d70, 0x0022_6b98)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0xcd;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0032_0ca0, 0x0022_6ba8)?;

        // loadMapFile(map, data, size): the size is read back from the frame.
        c.g[A2][0] = Addr(frame.at(c, 0x10)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0032_0db0, 0x0022_6bb8)?;

        // Free the text when the loader returned one.
        c.g[A2][0] = 0x0048_0000;
        if c.g[S1][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e98, 0x0022_6bc8)?;
            c.g[A2][0] = 0x0048_0000;
        }
        c.cycles += 1032;

        // compileMap(map, index, name) for every binding; a1 and a2 are left
        // as the last call set them.
        for (index, &(ret, name, _)) in COMPILE_BINDINGS.iter().enumerate() {
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = index as u64;
            c.g[A2][0] = sx(name);
            call_ptr(c, 0x0032_0e18, ret)?;
        }

        // purgeMapFile(map); the result is the map.
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0032_0f10, 0x0022_7bd4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x30);
        frame.restore(c, S0, 0x40);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `EA_MemFree(void*)` (0x317e98).
pub(crate) const IM_EA_MEM_FREE: Callee = Callee {
    addr: 0x0031_7e98,
    f: ee::s10::f_00317e98,
};

/// `InputMap_CreateParser(map, buffer)` (0x320db0).
pub(crate) const IM_CREATE_PARSER: Callee = Callee {
    addr: 0x0032_0db0,
    f: ee::s10::f_00320db0,
};

/// `InputParser_NextToken(parser)` (0x321af0).
pub(crate) const IM_NEXT_TOKEN: Callee = Callee {
    addr: 0x0032_1af0,
    f: ee::s10::f_00321af0,
};

/// `InputVM_Init(vm, pad)` (0x325250).
pub(crate) const IM_VM_INIT: Callee = Callee {
    addr: 0x0032_5250,
    f: ee::s10::f_00325250,
};

/// `InputVM_Execute(vm, pad, ...)` (0x325450).
pub(crate) const IM_VM_EXECUTE: Callee = Callee {
    addr: 0x0032_5450,
    f: ee::s10::f_00325450,
};

/// `void InputMap_Free(InputMap* map)` (0x320d08): frees the two buffers
/// the map owns (`+4` and `+0x14`), each only when set.
pub fn input_map_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let map = c.g[S0][0] as u32;
        c.g[A0][0] = Addr(map.wrapping_add(4)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            IM_EA_MEM_FREE.call(c, 0x0032_0d2c)?;
        }
        c.g[A0][0] = Addr(map.wrapping_add(0x14)).lw(c);
        c.cycles += 2;
        frame.restore(c, S0, 0x10);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            IM_EA_MEM_FREE.call(c, 0x0032_0d40)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void InputMap_ParseFile(InputMap* map, const char* path)` (0x320d50):
/// reads the file size (into the stack word at sp), then creates the
/// parser from the loaded buffer; a null buffer is not freed.
pub fn input_map_parse_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0x100;
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        // EAFile_LoadFileGetSizeEx(path, &size, 0x100), through the table.
        call_ptr(c, 0x003e_1a10, 0x0032_0d74)?;
        c.g[S1][0] = c.g[V0][0];
        let size_addr = sp(c);
        c.g[A2][0] = Addr(size_addr).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        IM_CREATE_PARSER.call(c, 0x0032_0d88)?;
        let no_buffer = c.g[S1][0] == 0;
        frame.restore(c, S0, 0x30);
        if !no_buffer {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            IM_EA_MEM_FREE.call(c, 0x0032_0d98)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// Shared body of `cInputMap_getValue1` and `cInputMap_getState1`
/// (0x321010 / 0x321188): spills the eight float arguments (f12..f19) to a
/// stack array, initialises the VM, looks up the state's operand table and
/// runs the VM. `prologue` is the cycle charge of the first block, the two
/// return addresses are the VM calls' return points.
pub(crate) fn oc561_eval_args(c: &mut Cpu, prologue: u64, init_ret: u32, exec_ret: u32) -> Flow {
    c.cycles += prologue;
    let frame = Frame::open(c, 0x220);
    frame.save(c, S3, 0x1a0);
    frame.save(c, S0, 0x1d0);
    c.g[S3][0] = c.g[A0][0];
    frame.save(c, S1, 0x1c0);
    c.g[S0][0] = c.g[A2][0];
    frame.save(c, S2, 0x1b0);
    c.g[A0][0] = c.g[SP][0];
    frame.save_fpr(c, 27, 0x218);
    c.g[S2][0] = c.g[A3][0];
    frame.save_fpr(c, 26, 0x210);
    c.mov_s(27, 19);
    frame.save_fpr(c, 25, 0x208);
    c.mov_s(26, 18);
    frame.save_fpr(c, 24, 0x200);
    c.mov_s(25, 17);
    frame.save_fpr(c, 23, 0x1f8);
    c.mov_s(24, 16);
    frame.save_fpr(c, 22, 0x1f0);
    c.mov_s(23, 15);
    frame.save_fpr(c, 21, 0x1e8);
    c.mov_s(22, 14);
    frame.save_fpr(c, 20, 0x1e0);
    c.mov_s(21, 13);
    c.mov_s(20, 12);
    frame.save_ra(c, 0x190);
    c.g[S1][0] = c.g[A1][0];
    IM_VM_INIT.call(c, init_ret)?;
    let table = (c.g[S3][0] as u32).wrapping_add(4);
    c.g[V1][0] = Addr(table).lw(c);
    c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
    let stack_word = sp(c);
    let modifier = c.g[S2][0];
    Addr(stack_word).sw(c, modifier);
    c.g[A1][0] = c.g[S1][0];
    frame.save_fpr(c, 20, 0x10c);
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    frame.save_fpr(c, 21, 0x110);
    c.g[A0][0] = c.g[SP][0];
    frame.save_fpr(c, 22, 0x114);
    frame.save_fpr(c, 23, 0x118);
    frame.save_fpr(c, 24, 0x11c);
    frame.save_fpr(c, 25, 0x120);
    frame.save_fpr(c, 26, 0x124);
    frame.save_fpr(c, 27, 0x128);
    c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x14)).lw(c);
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    IM_VM_EXECUTE.call(c, exec_ret)?;
    Ok(())
}

/// Restores what [`oc561_eval_args`] saved and returns.
pub(crate) fn oc561_eval_restore(c: &mut Cpu) {
    let frame = Frame { size: 0x220 };
    frame.restore(c, S0, 0x1d0);
    frame.restore(c, S1, 0x1c0);
    frame.restore(c, S2, 0x1b0);
    frame.restore(c, S3, 0x1a0);
    frame.restore_ra(c, 0x190);
    frame.restore_fpr(c, 27, 0x218);
    frame.restore_fpr(c, 26, 0x210);
    frame.restore_fpr(c, 25, 0x208);
    frame.restore_fpr(c, 24, 0x200);
    frame.restore_fpr(c, 23, 0x1f8);
    frame.restore_fpr(c, 22, 0x1f0);
    frame.restore_fpr(c, 21, 0x1e8);
    frame.restore_fpr(c, 20, 0x1e0);
    frame.close(c);
}

/// `float cInputMap_getValue1(InputMap* map, InputPad* pad, int state,
/// int modifier)` (0x321010): runs the state's VM and returns its float
/// result (left in f0 by the VM).
pub fn input_map_eval_float_args(c: &mut Cpu) {
    guest(c, |c| {
        oc561_eval_args(c, 62, 0x0032_1080, 0x0032_10cc)?;
        oc561_eval_restore(c);
        Ok(())
    });
}

/// `bool cInputMap_getState1(InputMap* map, InputPad* pad, int state,
/// int modifier)` (0x321188): as the value variant, returning whether the
/// VM's float result is non-zero.
pub fn input_map_eval_bool_args(c: &mut Cpu) {
    guest(c, |c| {
        oc561_eval_args(c, 52, 0x0032_11f8, 0x0032_1244)?;
        c.mtc1(0, 1);
        c.c_eq_s(0, 1);
        let nonzero = !c.fcc();
        c.g[V0][0] = 0x1;
        if !nonzero {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 15;
        oc561_eval_restore(c);
        Ok(())
    });
}

/// `void cInputMapParser_skipPastSemicolon(InputMapParser* self)` (0x321590):
/// reads tokens until the token type at `+0x98` is 5 (the end marker) or 0.
pub fn input_parser_skip_statement(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 5;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let parser = c.g[S0][0] as u32;
        c.g[V1][0] = Addr(parser.wrapping_add(0x98)).lw(c);
        if c.g[V1][0] != c.g[V0][0] {
            c.cycles += 2;
            let empty = c.g[V1][0] == 0;
            c.g[S1][0] = 5;
            if !empty {
                c.cycles += 1;
                loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    IM_NEXT_TOKEN.call(c, 0x0032_15c8)?;
                    let token_at = (c.g[S0][0] as u32).wrapping_add(0x98);
                    c.g[V0][0] = Addr(token_at).lw(c);
                    if c.g[V0][0] == c.g[S1][0] {
                        break;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_15c0);
                    }
                }
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S0][0];
        IM_NEXT_TOKEN.call(c, 0x0032_15e4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `InputParser_FreeNode` (0x00321638): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn input_parser_free_node(c: &mut Cpu) {
    if c.pc != 0x00321638 {
        return ee::s10::f_00321638__generated(c);
    }
    guest(c, |c| {
        'b_3216d4: {
            c.cycles += 7;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A1][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            let t = c.g[S0][0] == 0u64;
            c.g[S1][0] = c.g[A0][0];
            if t {
                break 'b_3216d4;
            } else {
                'b_3216cc: {
                    'b_3216c8: {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                        c.g[V0][0] = (c.g[V1][0] < 0x22u64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x490000;
                        if t {
                            break 'b_3216c8;
                        } else {
                            'b_3216a4: {
                                'b_321694: {
                                    'b_321684: {
                                        c.cycles += 6;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x22b0));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x0032167c => {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                if t {
                                                    break 'b_321694;
                                                } else {
                                                    break 'b_321684;
                                                }
                                            }
                                            0x00321684 => {
                                                break 'b_321684;
                                            }
                                            0x003216a4 => {
                                                break 'b_3216a4;
                                            }
                                            0x003216c8 => {
                                                break 'b_3216c8;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x00321638, 0x003216e8);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[RA][0] = sx(0x00321690);
                                    c.g[A0][0] = c.g[S1][0];
                                    Callee {
                                        addr: 0x00321638,
                                        f: ee::s10::InputParser_FreeNode,
                                    }
                                    .call(c, 0x00321690)?;
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    break 'b_321694;
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x0032169c);
                                c.g[A0][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x00321638,
                                    f: ee::s10::InputParser_FreeNode,
                                }
                                .call(c, 0x0032169c)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe0));
                                if t {
                                    break 'b_3216cc;
                                } else {
                                    break 'b_3216a4;
                                }
                            }
                            c.cycles += 9;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[RA][0] = sx(0x003216b0);
                            c.g[A0][0] = c.g[S1][0];
                            Callee {
                                addr: 0x00321638,
                                f: ee::s10::InputParser_FreeNode,
                            }
                            .call(c, 0x003216b0)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[RA][0] = sx(0x003216bc);
                            c.g[A0][0] = c.g[S1][0];
                            Callee {
                                addr: 0x00321638,
                                f: ee::s10::InputParser_FreeNode,
                            }
                            .call(c, 0x003216bc)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                            c.g[RA][0] = sx(0x003216c8);
                            c.g[A0][0] = c.g[S1][0];
                            Callee {
                                addr: 0x00321638,
                                f: ee::s10::InputParser_FreeNode,
                            }
                            .call(c, 0x003216c8)?;
                            break 'b_3216c8;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe0));
                    break 'b_3216cc;
                }
                c.cycles += 2;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0xe0), c.g[S0][0]);
                break 'b_3216d4;
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes::machine;

    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as RET;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::camera::camera::tests::input_map;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::cstr;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_3 as alloc_base;

    pub(crate) use crate::game::fe::feasyncfile::tests::env;

    pub(crate) use crate::game::fe::feasyncfile::tests::run_7 as run;

    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;

    pub(crate) use crate::game::fe::fereal::tests::base;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_3;

    pub(crate) use crate::game::fe::festaterewards::tests::strict;

    use ssx3_ee as ee;

    #[test]
    fn input_map_eval_float_matches() {
        check(
            ee::s10::f_00320fa8__generated,
            input_map_eval_float,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x0032_0fa8 ^ seed as u64);
                scramble(c, &mut r);
                let map = DATA + 0x1000;
                c.g[A0][0] = map as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = (seed % 4) as u64; // binding
                input_map(c, map);
            },
        );
    }

    #[test]
    fn input_map_eval_bool_matches() {
        check(
            ee::s10::f_00321108__generated,
            input_map_eval_bool,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x0032_1108 ^ seed as u64);
                scramble(c, &mut r);
                let map = DATA + 0x1000;
                c.g[A0][0] = map as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = (seed % 4) as u64; // binding
                input_map(c, map);
                // The stopped VM leaves `*(parser + 8)` in f0 (parser = EvalBool's
                // frame): cover both compare outcomes.
                let value = if seed % 2 == 0 { 0.0f32 } else { 1.0f32 };
                c.bus.write32(SP - 0x1d0 + 8, value.to_bits());
            },
        );
    }

    pub(crate) fn put_f32(c: &mut Cpu, a: u32, bits: u32) {
        c.bus.write32(a, bits);
    }

    #[test]
    fn input_pad_update_matches() {
        check_lohi(
            ssx3_ee::s10::f_00321298__generated,
            input_pad_update,
            300,
            &|c, seed| {
                let mut r = Rng(0x3212_98 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                let selfp = DATA;
                let count = seed % 3;
                let n = (seed / 3) % 5;
                let vals = DATA + 0x400;
                c.g[A0][0] = selfp as u64;
                c.g[A1][0] = n as u64;
                c.g[A2][0] = vals as u64;
                c.bus.write32(selfp, count);
                for i in 0..4 {
                    let b = selfp + 4 + 0x1c * i;
                    put_f32(c, b, r.edge_f32());
                    c.bus.write32(b + 4, if i % 2 == 0 { 1 } else { 0 });
                    c.bus.write32(b + 8, 0);
                    c.bus.write32(b + 0xc, r.next_u32() % 2);
                    c.bus.write32(b + 0x10, 0);
                    c.bus.write32(b + 0x14, r.next_u32() % 0x20);
                    c.bus.write32(b + 0x18, r.next_u32() % 4);
                }
                for i in 0..4 {
                    put_f32(c, vals + 4 * i, r.edge_f32());
                }
            },
        );
    }

    /// One held button (value 1.0) through `ticks` updates at rate `hz` from
    /// state `held` and settle counter `settle`: the ticks (1-based) that set
    /// the pressed edge (`+4`) and the repeat flag (`+0x10`).
    fn pad_events(hz: u32, ticks: u32, held: u32, settle: u32) -> (Vec<u32>, Vec<u32>) {
        let mut c = crate::difftest::fresh();
        crate::tick::set_for_test(&mut c, hz);
        let pad = DATA;
        let b = pad + 4;
        let vals = DATA + 0x400;
        c.bus.write32(pad, 1);
        for (off, v) in [
            (0u32, 0u32),
            (4, 0),
            (8, 0),
            (0xc, held),
            (0x10, 0),
            (0x14, 0),
            (0x18, settle),
        ] {
            c.bus.write32(b + off, v);
        }
        put_f32(&mut c, vals, 1.0f32.to_bits());
        let (mut pressed, mut repeats) = (Vec::new(), Vec::new());
        for t in 1..=ticks {
            c.g[A0][0] = pad as u64;
            c.g[A1][0] = 1;
            c.g[A2][0] = vals as u64;
            input_pad_update(&mut c);
            assert_eq!(c.pc, RET, "update did not return");
            if c.bus.read32(b + 4) != 0 {
                pressed.push(t);
            }
            if c.bus.read32(b + 0x10) != 0 {
                repeats.push(t);
            }
        }
        (pressed, repeats)
    }

    /// The debounce (3 frames after an edge) and the held-button repeat (first
    /// after 24 frames, then every 12) keep their wall-clock times: at rate r
    /// a 60 Hz event on tick i happens on tick (i - 1) * r / 60 + 1.
    #[test]
    fn input_pad_update_timers_follow_the_rate() {
        let (p60, _) = pad_events(60, 60, 0, 0);
        let (_, r60) = pad_events(60, 60, 1, 100);
        assert_eq!(p60, vec![4]);
        assert_eq!(r60, vec![1, 25, 37, 49]);
        for hz in [120u32, 240] {
            let q = hz / 60;
            let scaled = |v: &[u32]| v.iter().map(|&i| (i - 1) * q + 1).collect::<Vec<_>>();
            let (p, _) = pad_events(hz, 60 * q, 0, 0);
            let (_, r) = pad_events(hz, 60 * q, 1, 100);
            assert_eq!(p, scaled(&p60), "{hz} Hz: debounce");
            assert_eq!(r, scaled(&r60), "{hz} Hz: auto-repeat");
        }
    }

    #[test]
    fn input_parser_alloc_node_matches() {
        check(
            ee::s10::f_003215f8__generated,
            input_parser_alloc_node,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x3215f8, 0);
                let self_at = DATA + 0x200;
                let node = DATA + 0x400;
                c.g[A0][0] = self_at as u64;
                if seed % 3 == 0 {
                    c.bus.write32(self_at + 0xe0, 0);
                } else {
                    c.bus.write32(self_at + 0xe0, node);
                    c.bus
                        .write32(node + 4, if seed % 2 == 0 { 0 } else { DATA + 0x500 });
                }
            },
        );
    }

    /// The engine's size-class pool allocator (see `oc57`): short free lists
    /// behind `gp + 0x2a60..0x2a84`, enabled by the "pools ready" flag at
    /// `gp + 0xdd8`.
    pub(crate) fn pool_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x32000),
            (0x2a7c, 0x25800),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        let nodes = DATA + 0x28_000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes + 0x100 * i as u32;
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                c.bus.write32(node, if n == 3 { 0 } else { node + 0x40 });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
        }
    }

    #[test]
    fn input_map_init_matches() {
        check(
            ee::s10::f_00320ca0__generated,
            input_map_init,
            40,
            &|c, seed| {
                base(c, seed, 0x32_0ca0);
                pool_allocator(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (1 + seed % 8) as u64;
            },
        );
    }

    #[test]
    fn input_parser_error_matches() {
        check(
            ee::s10::f_00321500__generated,
            input_parser_error,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3215, 0);
                env(c);
                let this = DATA + 0x400;
                let fmt = DATA + 0x800;
                cstr(c, fmt, "%d %d %d %d %d %d");
                w32(c, this + 0x8c, r.next_u32());
                w32(c, this + 0xc, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(fmt);
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
                c.g[T1][0] = (seed % 7) as u64;
                c.g[T2][0] = r.next_u32() as u64;
                c.g[T3][0] = r.next_u32() as u64;
                for i in 0..7 {
                    c.f[12 + i] = r.edge_f32();
                }
            },
        );
    }

    /// A real heap block of `size` bytes.
    pub(crate) fn alloc(c: &mut Cpu, size: u32) -> u32 {
        run(
            c,
            0x0031_7d70,
            ee::s10::f_00317d70,
            &[size as u64, 0x0048_0000, 0, 0],
        ) as u32
    }

    // ------------------------------------------------------------------ input map

    #[test]
    fn input_map_finalize_opcodes_matches() {
        check(
            ee::s10::f_00320f10__generated,
            input_map_finalize_opcodes,
            SEEDS,
            &|c, seed| {
                let mut r = alloc_base(c, seed, 0x320f, 4);
                let this = DATA;
                let count = seed % 5;
                let src = alloc(c, 0x40);
                for i in 0..count {
                    w(c, src + 4 * i, r.next_u32());
                }
                w(c, this + 0x10, count);
                w(c, this + 0x14, src);
                let old = if seed & 1 == 0 { 0 } else { alloc(c, 0x40) };
                w(c, this + 8, old);
                c.g[A0][0] = this as u64;
            },
        );
    }

    // --------------------------------------------------------- input parser

    #[test]
    fn input_parser_lookup_button_matches() {
        strict(
            ee::s10::f_003216e8__generated,
            input_parser_lookup_button,
            60,
            &|c, seed| {
                let _ = base_3(c, seed, 0x3216, 6);
                // The "button" prefix the parser compares against.
                for (i, b) in b"button\0".iter().enumerate() {
                    c.bus.write8(0x004a_3ef0 + i as u32, *b);
                }
                let name: &[u8] = match seed % 8 {
                    0 => b"button0\0",
                    1 => b"button12\0",
                    2 => b"button12_\0",
                    3 => b"buttonA\0",
                    4 => b"button\0",
                    5 => b"buttons\0",
                    6 => b"button9999\0",
                    _ => b"butto\0",
                };
                for (i, b) in name.iter().enumerate() {
                    c.bus.write8(DATA + 0x900 + i as u32, *b);
                }
                c.g[A1][0] = (DATA + 0x900) as u64;
            },
        );
    }

    pub(crate) use crate::game::world::worldview::tests::oc410_machine;
    pub(crate) use crate::game::world::worldview::tests::oc410_result;

    pub(crate) const IMAP: u32 = DATA + 0x1000;

    /// `InputMap_CompileEntry`: grows the opcode buffer when fewer than 0x100
    /// words remain, then compiles the statement at the end.
    #[test]
    fn input_map_compile_entry_matches() {
        check(
            ee::s10::f_00320e18__generated,
            input_map_compile_entry,
            300,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x0032_0e18, 0);
                c.pc = 0x0032_0e18;
                c.g[A0][0] = IMAP as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (DATA + 0x700) as u64;
                c.bus.write32(IMAP + INPUT_MAP_OFFSETS, DATA + 0x1800);
                c.bus.write32(IMAP + INPUT_MAP_PARSER, DATA + 0x1900);
                let used = r.next_u32() % 0x300;
                let cap = match seed % 4 {
                    0 => used + 0x100 + r.next_u32() % 0x200,
                    1 => used + r.next_u32() % 0x100,
                    2 => used.wrapping_sub(r.next_u32() % 8),
                    _ => used + 0xff,
                };
                c.bus.write32(IMAP + INPUT_MAP_CAP, cap);
                c.bus.write32(IMAP + INPUT_MAP_USED, used);
                c.bus.write32(
                    IMAP + INPUT_MAP_CODE,
                    if seed % 5 == 0 { 0 } else { DATA + 0x5000 },
                );
                c.bus.write32(oc410_result(1), DATA + 0x6000);
                c.bus.write32(oc410_result(4), r.next_u32() % 40);
            },
        );
    }

    use crate::difftest::fakes::covered;

    pub(crate) use crate::game::be::belibrary::tests::oc431_install;
    pub(crate) use crate::game::be::belibrary::tests::oc431_machine;

    pub(crate) use crate::game::be::belibrary::tests::OC431_BASE;
    pub(crate) use crate::game::be::belibrary::tests::OC431_CTRL;

    /// `InputParser_ctor`: the free-id stack and the node free list are
    /// built; `ParseAll` (real) runs `NextToken` and, while the parser has
    /// input, `ParseStatement` (fakes).
    #[test]
    fn input_parser_ctor_matches() {
        oc431_install();
        covered(
            ee::s10::f_00321428__generated,
            input_parser_ctor,
            100,
            &|c, seed| {
                let mut r = oc431_machine(c, seed, 0x321428, 2, 0x0032_1428);
                c.bus.write32(OC431_CTRL + 4, seed % 2);
                c.g[A0][0] = (OC431_BASE + 0x1_0000) as u64;
                c.g[A1][0] = (OC431_BASE + 0x100) as u64;
                c.g[A2][0] = (r.next_u32() % 0x1000) as u64;
            },
        );
    }

    use crate::difftest::fakes;

    pub(crate) use crate::game::ai::ai::tests::oc434_setup;

    /// `InputMap_CreateParser`: allocation and parser constructor are fakes.
    #[test]
    fn input_map_create_parser_matches() {
        fakes::covered(
            ee::s10::f_00320db0__generated,
            input_map_create_parser_2,
            12,
            &|c, seed| {
                let mut r = oc434_setup(c, seed, 0x320db0, 2, 0x0032_0db0);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    use std::ptr::addr_of_mut;

    /// The fakes below only log while this word is set at `OC561_GATE`.
    pub(crate) const OC561_GATE: u32 = DATA + 0x20_0000;

    pub(crate) const OC561_MAGIC: u32 = 0x0561_0561;

    /// Result cells the setups write: file-size call's v0 and size, the
    /// VM exec call's f0, and the token script read by the next-token fake.
    pub(crate) const OC561_FILE_V0: u32 = DATA + 0x20_0010;

    pub(crate) const OC561_FILE_SIZE: u32 = DATA + 0x20_0014;

    pub(crate) const OC561_EXEC_F0: u32 = DATA + 0x20_0018;

    pub(crate) const OC561_TOK_IDX: u32 = DATA + 0x20_0020;

    pub(crate) const OC561_TOKS: u32 = DATA + 0x20_0040;

    pub(crate) fn oc561_gated(c: &mut Cpu) -> bool {
        c.bus.read32(OC561_GATE) == OC561_MAGIC
    }

    /// Scrambled registers, the gate on, and no fake unwinds.
    pub(crate) fn oc561_setup(c: &mut Cpu, r: &mut Rng) {
        scramble(c, r);
        c.bus.write32(OC561_GATE, OC561_MAGIC);
        c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
    }

    pub(crate) fn oc561_fake_mem_free(c: &mut Cpu) {
        if !oc561_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc561_fake_mem_free),
                ee::s10::f_00317e98__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_7e98, ra);
    }

    pub(crate) fn oc561_fake_create_parser(c: &mut Cpu) {
        if !oc561_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc561_fake_create_parser),
                ee::s10::f_00320db0__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0032_0db0, ra);
    }

    pub(crate) fn oc561_fake_file_size(c: &mut Cpu) {
        if !oc561_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc561_fake_file_size),
                ee::s14::f_003e1a10__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let size_out = c.g[A1][0] as u32;
        let size = c.bus.read32(OC561_FILE_SIZE);
        c.bus.write32(size_out, size);
        fakes::fake(c, 0x003e_1a10, ra);
        c.g[V0][0] = c.bus.read32(OC561_FILE_V0) as u64;
    }

    pub(crate) fn oc561_fake_next_token(c: &mut Cpu) {
        if !oc561_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc561_fake_next_token),
                ee::s10::f_00321af0__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let parser = c.g[A0][0] as u32;
        let k = c.bus.read32(OC561_TOK_IDX);
        let token = c.bus.read32(OC561_TOKS + 4 * k.min(7));
        c.bus.write32(OC561_TOK_IDX, k + 1);
        fakes::fake(c, 0x0032_1af0, ra);
        c.bus.write32(parser.wrapping_add(0x98), token);
    }

    pub(crate) fn oc561_fake_vm_init(c: &mut Cpu) {
        if !oc561_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc561_fake_vm_init),
                ee::s10::f_00325250__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0032_5250, ra);
    }

    pub(crate) fn oc561_fake_vm_execute(c: &mut Cpu) {
        if !oc561_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc561_fake_vm_execute),
                ee::s10::f_00325450__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0032_5450, ra);
        c.f[0] = c.bus.read32(OC561_EXEC_F0);
    }

    pub(crate) fn oc561_fake_pad_stop(c: &mut Cpu) {
        if !oc561_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc561_fake_pad_stop),
                ee::s10::f_00326cf0__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0032_6cf0, ra);
    }

    pub(crate) fn oc561_fake_assign(c: &mut Cpu) {
        if !oc561_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc561_fake_assign),
                ee::s10::f_00329a28__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0032_9a28, ra);
    }

    /// Puts the gated fakes into their override slots (once per process).
    pub(crate) fn oc561_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc561_fake_mem_free,
            fakes::prev_cell(oc561_fake_mem_free),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00320DB0),
            oc561_fake_create_parser,
            fakes::prev_cell(oc561_fake_create_parser),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E1A10),
            oc561_fake_file_size,
            fakes::prev_cell(oc561_fake_file_size),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00321AF0),
            oc561_fake_next_token,
            fakes::prev_cell(oc561_fake_next_token),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00325250),
            oc561_fake_vm_init,
            fakes::prev_cell(oc561_fake_vm_init),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00325450),
            oc561_fake_vm_execute,
            fakes::prev_cell(oc561_fake_vm_execute),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00326CF0),
            oc561_fake_pad_stop,
            fakes::prev_cell(oc561_fake_pad_stop),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00329A28),
            oc561_fake_assign,
            fakes::prev_cell(oc561_fake_assign),
        );
    }

    /// The map at DATA, its operand table at DATA+0x500, its base at
    /// DATA+0x600, and the float arguments f12..f19 (shared by both
    /// `InputMap_Eval*Args` tests).
    pub(crate) fn oc561_eval_setup(c: &mut Cpu, r: &mut Rng, seed: u32) {
        oc561_setup(c, r);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DATA + 0x1000);
        c.g[A2][0] = (seed % 4) as u64;
        c.g[A3][0] = (seed % 5) as u64;
        c.bus.write32(DATA + 4, DATA + 0x500);
        c.bus.write32(DATA + 0x14, DATA + 0x600);
        for k in 0..4u32 {
            c.bus.write32(DATA + 0x500 + 4 * k, r.next_u32() % 8);
        }
        for i in 0..8usize {
            c.f[12 + i] = r.f32(100.0);
        }
        c.bus
            .write32(OC561_EXEC_F0, if seed % 2 == 0 { 0 } else { 0x3f80_0000 });
    }

    #[test]
    fn input_map_free_matches() {
        oc561_install_fakes();
        check(
            ee::s10::f_00320d08__generated,
            input_map_free,
            96,
            &|c, seed| {
                let mut r = Rng(0x0032_0d08 ^ seed as u64);
                oc561_setup(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 4, if seed % 3 == 0 { 0 } else { DATA + 0x400 });
                c.bus
                    .write32(DATA + 0x14, if seed % 2 == 0 { 0 } else { DATA + 0x800 });
            },
        );
    }

    #[test]
    fn input_map_parse_file_matches() {
        oc561_install_fakes();
        check(
            ee::s10::f_00320d50__generated,
            input_map_parse_file,
            96,
            &|c, seed| {
                let mut r = Rng(0x0032_0d50 ^ seed as u64);
                oc561_setup(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.bus.write32(OC561_FILE_SIZE, DATA + 0x2000 + 16 * seed);
                c.bus
                    .write32(OC561_FILE_V0, if seed % 3 == 0 { 0 } else { DATA + 0x3000 });
            },
        );
    }

    #[test]
    fn input_map_eval_float_args_matches() {
        oc561_install_fakes();
        check(
            ee::s10::f_00321010__generated,
            input_map_eval_float_args,
            96,
            &|c, seed| {
                let mut r = Rng(0x0032_1010 ^ seed as u64);
                oc561_eval_setup(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn input_map_eval_bool_args_matches() {
        oc561_install_fakes();
        check(
            ee::s10::f_00321188__generated,
            input_map_eval_bool_args,
            96,
            &|c, seed| {
                let mut r = Rng(0x0032_1188 ^ seed as u64);
                oc561_eval_setup(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn input_parser_skip_statement_matches() {
        oc561_install_fakes();
        check(
            ee::s10::f_00321590__generated,
            input_parser_skip_statement,
            96,
            &|c, seed| {
                let mut r = Rng(0x0032_1590 ^ seed as u64);
                oc561_setup(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let first = [5u32, 0, 3, 7][(seed % 4) as usize];
                c.bus.write32(DATA + 0x98, first);
                c.bus.write32(OC561_TOK_IDX, 0);
                for i in 0..7u32 {
                    c.bus.write32(OC561_TOKS + 4 * i, r.next_u32() % 4);
                }
                // The script always ends with the end marker (5).
                c.bus.write32(OC561_TOKS + 4 * 7, 5);
            },
        );
    }

    #[test]
    fn input_parser_free_node_matches() {
        check(
            ee::s10::f_00321638__generated,
            input_parser_free_node,
            200,
            &|c, seed| {
                machine(c, seed, 0x1638, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00321638;
            },
        );
    }
}
