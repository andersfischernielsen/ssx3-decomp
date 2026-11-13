//! `src/input/inputmap.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
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
}
