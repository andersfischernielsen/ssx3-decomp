//! `src/animation/rideranimbase.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call;
use crate::call::call_ptr;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::guest;
use crate::call::VSlot;
use crate::ps2f::FpuF32;
use crate::call::Unwound;
use crate::abi::r32;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::sp;
pub(crate) use crate::game::ai::aianimeventmap::BUILTIN_DELETE;
pub(crate) use crate::game::animation::animmodel::EA_MEM_FREE_ADDR;
pub(crate) use crate::game::visualfx::boardwakefx::p;
pub(crate) use crate::game::visualfx::boardwakefx::lw;
pub(crate) use crate::game::visualfx::boardwakefx::lh;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_2;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_18d0, anim_controller_ctor, "AnimController_ctor"),
    (0x0031_1958, anim_controller_dtor, "AnimController_dtor"),
    (0x0031_1e38, anim_controller_clear_layer, "AnimController_ClearLayer"),
    (0x0031_2490, anim_controller_dispatch_end_events, "AnimController_DispatchEndEvents"),
    (0x0031_2790, anim_controller_get_current_anim_length, "AnimController_GetCurrentAnimLength"),
    (0x0031_2820, anim_controller_get_current_anim_trigger_time, "AnimController_GetCurrentAnimTriggerTime"),
    (0x0031_2ab0, anim_controller_get_layer_progress, "AnimController_GetLayerProgress"),
    (0x0031_2c20, anim_data_base_eval_chan_curves, "AnimDataBase_EvalChanCurves"),
    (0x0031_3508, anim_sequence_ctor, "AnimSequence_ctor"),
    (0x0031_35b0, anim_sequence_advance_track, "AnimSequence_AdvanceTrack"),
    (0x0031_3868, anim_sequence_fire_triggers_in_range, "AnimSequence_FireTriggersInRange"),
    (0x0031_3938, anim_sequence_fire_triggers_at_time, "AnimSequence_FireTriggersAtTime"),
    (0x0031_39a8, anim_sequence_fire_end_triggers, "AnimSequence_FireEndTriggers"),
];

// ---------------------------------------------------------------------------
// AnimSequence trigger lists (0x0031_38xx / 0x0031_39xx / 0x0031_35xx).

/// Trigger-chain node layout (stride 0x10 in practice; only the first four
/// words are used by the firing helpers).
pub(crate) mod trigger {

    pub const BIT: u32 = 0x0;

    pub const COUNT: u32 = 0x4;

    pub const TIME: u32 = 0x8;

    pub const NEXT: u32 = 0xc;
}

// ---------------------------------------------------------------- AnimSequence

/// Direct-call targets of the animation functions (shard s10).
pub(crate) mod anim_callee {
    use super::Callee;
    use ssx3_ee::s10 as ee;
    pub(crate) use crate::game::animation::animmodel::anim_callee::FIRE_TRIGGERS_AT_TIME;
    pub(crate) use crate::game::animation::animmodel::anim_callee::FIRE_END_TRIGGERS;
    pub(crate) use crate::game::animation::animmodel::anim_callee::FIRE_TRIGGERS_IN_RANGE;
    pub(crate) use crate::game::animation::animmodel::anim_callee::GET_TRACK_BLEND_WEIGHT;
    pub(crate) use crate::game::animation::animmodel::anim_callee::ALLOC_CHAN_CACHE;
    pub(crate) use crate::game::animation::animmodel::anim_callee::DECODE_CHAN_CACHE;
    pub(crate) use crate::game::animation::animmodel::anim_callee::EVAL_CHAN_CURVES;
}

/// `AnimSequence` layout used here (tracks of 0x1c bytes from +0).
pub(crate) mod seq {
    pub(crate) use crate::game::animation::animsequencer::seq::TRACK_SIZE;
    pub(crate) use crate::game::animation::animsequencer::seq::T_ANIM;
    pub(crate) use crate::game::animation::animsequencer::seq::T_TIME;
    pub(crate) use crate::game::animation::animsequencer::seq::T_RATE;
    pub(crate) use crate::game::animation::animsequencer::seq::T_LENGTH;
    pub(crate) use crate::game::animation::animsequencer::seq::T_ACTIVE;
    pub(crate) use crate::game::animation::animsequencer::seq::T_MODE;
    pub(crate) use crate::game::animation::animsequencer::seq::BLEND_A;
    pub(crate) use crate::game::animation::animsequencer::seq::BLEND_B;
    pub(crate) use crate::game::animation::animsequencer::seq::LAYER;
    pub(crate) use crate::game::animation::animsequencer::seq::FLAGS;
    pub(crate) use crate::game::animation::animsequencer::seq::USER;
    pub(crate) use crate::game::animation::animsequencer::seq::RATE;
    pub(crate) use crate::game::animation::animsequencer::seq::FIRED_MASK;
    pub(crate) use crate::game::animation::animsequencer::seq::END_FLAG;
    pub(crate) use crate::game::animation::animsequencer::seq::DIRTY;
    pub(crate) use crate::game::animation::animsequencer::seq::NEXT;
}

/// `AnimDataBase_EvalChanCurves` entry points and jump tables.
pub(crate) mod eval {
    pub(crate) use crate::game::animation::animmodel::eval::ENTRY;
    pub(crate) use crate::game::animation::animmodel::eval::END;
    pub(crate) use crate::game::animation::animmodel::eval::LERP_HEAD;
    pub(crate) use crate::game::animation::animmodel::eval::KEY_HEAD;
    pub(crate) use crate::game::animation::animmodel::eval::LERP_TABLE_AT;
    pub(crate) use crate::game::animation::animmodel::eval::KEY_TABLE_AT;
    pub(crate) use crate::game::animation::animmodel::eval::LERP_TABLE;
    pub(crate) use crate::game::animation::animmodel::eval::KEY_TABLE;
    pub(crate) use crate::game::animation::animmodel::eval::CURVE_SIZE;
}

// ---------------------------------------------------------------------------
// Callees reached through the generated function table, and fixed addresses.

#[allow(dead_code)]
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::ANIM_SEQ_LIST_GET_AT;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::RIDER_COMPUTE_LOD;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::TRANSITION_EFFECT_UPDATE_SCREEN_RECT;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::RIDER_MODEL_RENDER;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::FLAGS64_TEST;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::MATH_COS;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::EMITTER_UPDATE;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::WORLD_PAINTER_QUERY_RESET_ALL;
}

pub(crate) const ANIM_SEQ_LIST_CLEAR: Callee = Callee { addr: 0x0031_4668, f: ee::s10::f_00314668 };

pub(crate) const ANIM_SEQ_LIST_GET_AT: Callee = Callee { addr: 0x0031_4760, f: ee::s10::f_00314760 };

pub(crate) const OPERATOR_NEW_NAMED2_ADDR: u32 = 0x0031_7e30;

pub(crate) const ANIM_SEQ_LIST_INIT: Callee = Callee { addr: 0x0031_42e8, f: ee::s10::f_003142e8 };

pub(crate) const ANIM_CONTROLLER_RESET: Callee = Callee { addr: 0x0031_1a50, f: ee::s10::f_00311a50 };


// ---------------------------------------------------------------------------
// AnimController.

/// `AnimController *AnimController_ctor(AnimController *this)` (0x3118d0):
/// allocate six sequence lists at +0x50, register the vtable at +0x58 and
/// reset the controller.
pub fn anim_controller_ctor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S1, 0x10);
        c.g[V0][0] = sx(0x49_0000u32.wrapping_sub(0x2828));
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x49_0000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x54), 0);
        c.g[A1][0] = sx(0x49_0000u32.wrapping_sub(0x5b20));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x58), c.g[V0][0]);
        c.g[A0][0] = 0x30;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, OPERATOR_NEW_NAMED2_ADDR, 0x0031_190c)?;
        c.g[S0][0] = 0;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x50), c.g[V0][0]);
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x50)).lw(c);
        loop {
            c.cycles += 7;
            c.g[A0][0] = sx((c.g[S0][0] as u32) << 3);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            ANIM_SEQ_LIST_INIT.call(c, 0x0031_1928)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 6i64) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x50)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0031_1918);
            }
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S1][0];
        ANIM_CONTROLLER_RESET.call(c, 0x0031_193c)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void AnimController_dtor(AnimController *this, uint flags)` (0x311958):
/// clear the six sequence lists, free them and, when `flags & 1`, delete
/// `this`.
pub fn anim_controller_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S1, 0x20);
        c.g[V0][0] = sx(0x49_0000u32.wrapping_sub(0x2828));
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S0][0] = 0;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x58), c.g[V0][0]);
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x50)).lw(c);
        loop {
            c.cycles += 7;
            c.g[A0][0] = sx((c.g[S0][0] as u32) << 3);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            ANIM_SEQ_LIST_CLEAR.call(c, 0x0031_1998)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 6i64) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x50)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0031_1988);
            }
        }
        c.cycles += 3;
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x50)).lw(c);
        let no_lists = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S2][0] & 1;
        if !no_lists {
            c.cycles += 3;
            call_ptr(c, EA_MEM_FREE_ADDR, 0x0031_19b8)?;
            c.g[V0][0] = c.g[S2][0] & 1;
        }
        c.cycles += 2;
        let keep = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x30);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            BUILTIN_DELETE.call(c, 0x0031_19cc)?;
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


/// `void AnimController_ClearLayer(AnimController *this, int layer)`
/// (0x311e38): clear the layer's sequence list and mark the layer slot 0x1b6.
pub fn anim_controller_clear_layer(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A0][0] = sx((c.g[S0][0] as u32) << 3);
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x50)).lw(c);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        ANIM_SEQ_LIST_CLEAR.call(c, 0x0031_1e60)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V0][0] = 0x1b6;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        frame.restore_ra(c, 0);
        c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `float AnimController_GetCurrentAnimLength(AnimController *this)`
/// (0x312790): the current anim's length in seconds: ask the vtable at
/// `this+0x58` for the layer, look the anim up in the tables at `gp+0xd8c` /
/// `gp+0xd08`, and scale the frame count by the constant at `gp-0x3230`.
pub fn anim_controller_get_current_anim_length(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x58)).lw(c);
        c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        call_ptr(c, c.g[V0][0] as u32, 0x0031_27ac)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 0x207;
        let no_anim = c.g[A0][0] == c.g[V0][0];
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xd8c)).lw(c);
        if no_anim {
            c.cycles += 1;
            c.mtc1(0, 0);
        } else {
            c.cycles += 20;
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xd08)).lw(c);
            c.g[A1][0] = 0x14;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3230));
            c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1030)).lw(c);
            c.g[V0][0] = c.g[V1][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 8);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let prod = abi::mult(c, c.g[V1][0] as u32, c.g[A1][0] as u32);
            c.g[V1][0] = sx(prod);
            c.g[A0][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(8)).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lhu(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.mul_s(0, 0, 1);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `float AnimController_GetCurrentAnimTriggerTime(AnimController *this, void *unused, int trigger)`
/// (0x312820): the time of trigger `trigger` of the current anim, or -1.0 when
/// it is out of range.
pub fn anim_controller_get_current_anim_trigger_time(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x58)).lw(c);
        c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        call_ptr(c, c.g[V0][0] as u32, 0x0031_2844)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 0x207;
        let is_anim = c.g[A0][0] != c.g[V0][0];
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xd8c)).lw(c);
        if !is_anim {
            c.cycles += 3;
            c.mtc1(0, 0);
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 17;
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xd08)).lw(c);
        c.g[A2][0] = 0x14;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x1030)).lw(c);
        c.g[V1][0] = c.g[A0][0] & 0xff;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[A0][0] as u32) >> 8);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let prod = abi::mult(c, c.g[A0][0] as u32, c.g[A2][0] as u32);
        c.g[A0][0] = sx(prod);
        c.g[A1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(8)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x12)).lhu(c);
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[AT][0] = sx(0xbf80_0000);
            c.mtc1(1, 0);
        } else {
            c.cycles += 11;
            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x10)).lhu(c);
            c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x10)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x322c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
            c.mtc1(3, 0);
            c.cvt_s_w(0, 0);
            c.mul_s(0, 0, 1);
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `float AnimController_GetLayerProgress(AnimController *this, int layer)`
/// (0x312ab0): the ratio `seq->elapsed(+8) / seq->length(+0x10)` of the layer's
/// current sequence.
pub fn anim_controller_get_layer_progress(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x50)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        ANIM_SEQ_LIST_GET_AT.call(c, 0x0031_2acc)?;
        let seq = c.g[V0][0] as u32;
        c.lwc1(1, seq.wrapping_add(0x10));
        c.lwc1(0, seq.wrapping_add(8));
        frame.restore_ra(c, 0);
        c.div_s(0, 0, 1);
        frame.close(c);
        Ok(())
    })
}


/// `AnimSequence` fields the firing helpers touch.
pub(crate) const TRIGGERS: u32 = 0xac;

pub(crate) const MASK_A: u32 = 0xb0;

pub(crate) const MASK_B: u32 = 0xb8;

pub(crate) const END_FLAG: u32 = 0xc0;


/// Set bit `bit` of the two trigger masks from `base` (a `T0`/`T1` holding 1),
/// mirroring the original's `dsll32`-free `dsll` + `or`/`nor`/`and` sequence.
pub(crate) fn fire_mask_bit(c: &mut Cpu, seq: u32, base: u64, bit: u64) {
    let mask_a = Addr(seq.wrapping_add(MASK_A)).ld(c);
    c.g[V1][0] = mask_a;
    let set = base << (bit & 63);
    c.g[V0][0] = set;
    let mask_b = Addr(seq.wrapping_add(MASK_B)).ld(c);
    c.g[A0][0] = mask_b;
    let inv = !mask_a;
    c.g[A1][0] = inv;
    let new_a = mask_a | set;
    c.g[V1][0] = new_a;
    let clear = set & inv;
    c.g[V0][0] = clear;
    Addr(seq.wrapping_add(MASK_A)).sd(c, new_a);
    let new_b = mask_b | clear;
    c.g[A0][0] = new_b;
    Addr(seq.wrapping_add(MASK_B)).sd(c, new_b);
}


/// `void AnimSequence_FireEndTriggers(AnimSequence *this)` (0x3139a8): set the
/// end flag (+0xc0) and, for every trigger in the chain at +0xac whose count
/// (+4) is non-zero, set bit `n` of the mask at +0xb8 (`n` = the trigger's word
/// at +0); bit `n` is cleared from +0xb0. 8 cycles per node plus a back-edge
/// event check at 0x3139c8.
pub fn anim_sequence_fire_end_triggers(c: &mut Cpu) {
    const HEAD: u32 = 0x0031_39c8;
    c.cycles += 5;
    let seq = Addr(abi::arg(c, 0));
    c.g[A3][0] = seq.0 as u64;
    c.g[V0][0] = 1;
    let mut trigger = seq.off(TRIGGERS).lw(c);
    c.g[A2][0] = trigger;
    seq.off(END_FLAG).sw(c, 1);
    if trigger != 0 {
        c.cycles += 3;
        c.g[T0][0] = 1;
        let mut count = Addr(trigger as u32).off(trigger::COUNT).lw(c);
        c.g[V0][0] = count;
        loop {
            c.cycles += 2;
            if count == 0 {
                trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
                c.g[A2][0] = trigger;
            } else {
                c.cycles += 11;
                let bit = Addr(trigger as u32).off(trigger::BIT).lw(c);
                c.g[V0][0] = bit;
                fire_mask_bit(c, seq.0, c.g[T0][0], bit);
                trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
                c.g[A2][0] = trigger;
            }
            c.cycles += 2;
            if trigger == 0 {
                break;
            }
            count = Addr(trigger as u32).off(trigger::COUNT).lw(c);
            c.g[V0][0] = count;
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `void AnimSequence_FireTriggersAtTime(AnimSequence *this, float time)`
/// (0x313938): for every trigger at +0xac whose count (+4) is non-zero and
/// whose time (+8) equals the argument (f12), set bit `n` (word at +0) of the
/// mask at +0xb0, clear it from +0xb8. Back-edge event check at 0x313950.
pub fn anim_sequence_fire_triggers_at_time(c: &mut Cpu) {
    const HEAD: u32 = 0x0031_3950;
    let at = FpuF32(c.f[12]);
    c.cycles += 4;
    let seq = Addr(abi::arg(c, 0));
    c.g[A3][0] = seq.0 as u64;
    let mut trigger = seq.off(TRIGGERS).lw(c);
    c.g[A2][0] = trigger;
    c.g[T0][0] = 1;
    if trigger != 0 {
        c.cycles += 2;
        let mut count = Addr(trigger as u32).off(trigger::COUNT).lw(c);
        c.g[V0][0] = count;
        loop {
            c.cycles += 2;
            if count != 0 {
                trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
                c.g[A2][0] = trigger;
            } else {
                c.cycles += 5;
                let t = Addr(trigger as u32).off(trigger::TIME).read_fpu(c);
                c.f[0] = t.0;
                let eq = t.eq_s(at);
                abi::set_fcc(c, eq);
                if !eq {
                    trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
                    c.g[A2][0] = trigger;
                } else {
                    c.cycles += 11;
                    let bit = Addr(trigger as u32).off(trigger::BIT).lw(c);
                    c.g[V0][0] = bit;
                    fire_mask_bit(c, seq.0, c.g[T0][0], bit);
                    trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
                    c.g[A2][0] = trigger;
                }
            }
            c.cycles += 2;
            if trigger == 0 {
                break;
            }
            count = Addr(trigger as u32).off(trigger::COUNT).lw(c);
            c.g[V0][0] = count;
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `void AnimSequence_FireTriggersInRange(AnimSequence *this, float lo,
/// float hi)` (0x313868): fire every trigger whose time lies in [lo, hi]
/// (inclusive, either edge may be open); `lo == hi` fires nothing. Bit `n`
/// (word at +0) is set in +0xb8 and cleared from +0xb0. Back-edge event check
/// at 0x3138a0.
pub fn anim_sequence_fire_triggers_in_range(c: &mut Cpu) {
    const HEAD: u32 = 0x0031_38a0;
    let lo = FpuF32(c.f[12]);
    let hi = FpuF32(c.f[13]);
    c.cycles += 4;
    let empty = lo.eq_s(hi);
    abi::set_fcc(c, empty);
    let seq = Addr(abi::arg(c, 0));
    c.g[A3][0] = seq.0 as u64;
    if empty {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    let lo_le_hi = lo.le(hi);
    abi::set_fcc(c, lo_le_hi);
    if lo_le_hi {
        c.g[T0][0] = 1;
    } else {
        c.cycles += 1;
        c.g[T0][0] = 0;
    }
    c.cycles += 3;
    let mut trigger = seq.off(TRIGGERS).lw(c);
    c.g[A2][0] = trigger;
    c.g[T1][0] = 1;
    if trigger == 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let mut count = Addr(trigger as u32).off(trigger::COUNT).lw(c);
    c.g[V0][0] = count;
    loop {
        c.cycles += 2;
        let mut fire = false;
        if count != 0 {
            trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
            c.g[A2][0] = trigger;
        } else {
            c.cycles += 2;
            let ascending = c.g[T0][0] != 0;
            let t = Addr(trigger as u32).off(trigger::TIME).read_fpu(c);
            c.f[0] = t.0;
            'test: {
                if ascending {
                    c.cycles += 4;
                    let lt = lo.lt(t);
                    abi::set_fcc(c, lt);
                    if !lt {
                        trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
                        c.g[A2][0] = trigger;
                        break 'test;
                    }
                    c.cycles += 4;
                    let le = t.le(hi);
                    abi::set_fcc(c, le);
                    if !le {
                        c.cycles += 2;
                        trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
                        c.g[A2][0] = trigger;
                        break 'test;
                    }
                } else {
                    c.cycles += 4;
                    let le = hi.le(t);
                    abi::set_fcc(c, le);
                    if !le {
                        trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
                        c.g[A2][0] = trigger;
                        break 'test;
                    }
                    c.cycles += 4;
                    let lt = t.lt(lo);
                    abi::set_fcc(c, lt);
                    if !lt {
                        trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
                        c.g[A2][0] = trigger;
                        break 'test;
                    }
                    c.cycles += 1;
                }
                c.g[V0][0] = Addr(trigger as u32).off(trigger::BIT).lw(c);
                fire = true;
            }
            if fire {
                c.cycles += 10;
                fire_mask_bit(c, seq.0, c.g[T1][0], c.g[V0][0]);
                trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
                c.g[A2][0] = trigger;
            }
        }
        c.cycles += 2;
        if trigger == 0 {
            break;
        }
        count = Addr(trigger as u32).off(trigger::COUNT).lw(c);
        c.g[V0][0] = count;
        if event_due(c) {
            deliver_event_here(c, HEAD);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `void AnimSequence_ctor(AnimSequence *this, void *arg)` (0x313508):
/// store `arg` at +0, clear the three track records at +0x18.., then fill the
/// trigger/mask block from the constants at 0x4ff230. Back-edge event check
/// at 0x313528.
pub fn anim_sequence_ctor(c: &mut Cpu) {
    const HEAD: u32 = 0x0031_3528;
    c.cycles += 8;
    let seq = abi::arg(c, 0);
    let arg = abi::arg64(c, 1);
    c.g[A3][0] = seq as u64;
    c.g[V0][0] = 2;
    c.g[V1][0] = u64::MAX;
    Addr(seq).sw(c, arg);
    c.g[V1][0] = 2;
    let mut p = seq.wrapping_add(0x50);
    c.g[V0][0] = sx(p);
    loop {
        c.cycles += 7;
        Addr(p).sw(c, 0);
        let k = (c.g[V1][0] as u32).wrapping_sub(1);
        c.g[V1][0] = sx(k);
        let more = (c.g[V1][0] as i64) >= 0;
        let np = p.wrapping_sub(0x1c);
        c.g[V0][0] = sx(np);
        p = np;
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, HEAD);
        }
    }
    c.cycles += 26;
    c.g[A0][0] = 0x0050_0000;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0); // f0 = 1.0f
    let table = 0x0050_0000u32.wrapping_sub(0xdd0);
    c.g[A0][0] = sx(table);
    c.g[A2][0] = u64::MAX;
    c.lq(5, table);
    c.g[V0][0] = seq as u64;
    c.sq(5, seq.wrapping_add(0x60));
    c.lq(3, table.wrapping_add(0x10));
    Addr(seq).off(0x88).sd(c, u64::MAX);
    c.sq(3, seq.wrapping_add(0x70));
    Addr(seq).off(0x98).write_u32(c, 0x3f80_0000);
    Addr(seq).off(0x80).sw(c, 0);
    Addr(seq).off(0x84).sw(c, 0);
    Addr(seq).off(0x90).write_u32(c, 0x3f80_0000);
    Addr(seq).off(0x9c).sw(c, 0);
    Addr(seq).off(0x94).write_u32(c, 0x3f80_0000);
    Addr(seq).off(0xa0).sw(c, 0);
    Addr(seq).off(0xa4).sw(c, 0);
    Addr(seq).off(0xa8).sw(c, 0);
    Addr(seq).off(0xb8).sd(c, 0);
    Addr(seq).off(0xb0).sd(c, 0);
    Addr(seq).off(0xac).sw(c, 0);
    Addr(seq).off(0xc0).sw(c, 0);
    c.sw(seq.wrapping_add(0xc4), 0);
    abi::ret(c);
}


/// `void AnimSequence_AdvanceTrack(AnimSequence* seq, int track, float dt)`
/// (0x3135b0): move a track's time by `dt * track rate * sequence rate` and
/// fire the animation triggers passed over. Track 0 first clears the fired
/// masks and, if the sequence is dirty, fires the triggers at the old time.
/// A track with loop mode 0 clamps to `[0, length]` (firing the end triggers
/// when it hits an end); a looping track wraps by whole lengths, fires the
/// end triggers and the triggers of every range crossed (forward or
/// backward, at most one full extra loop). Only track 0 fires triggers.
/// Stores the new time; returns 0, with v1 the track.
pub fn anim_sequence_advance_track(c: &mut Cpu) {
    /// `AnimSequence_FireTriggersInRange(seq, lo, hi)` returning to `ret`.
    fn fire_range(c: &mut Cpu, lo: FpuF32, hi: FpuF32, ret: u32) -> Flow {
        c.g[A0][0] = c.g[S0][0];
        c.f[12] = lo.0;
        c.f[13] = hi.0;
        anim_callee::FIRE_TRIGGERS_IN_RANGE.call(c, ret)
    }
    /// `AnimSequence_FireEndTriggers(seq)` returning to `ret`.
    fn fire_end(c: &mut Cpu, ret: u32) -> Flow {
        c.g[A0][0] = c.g[S0][0];
        anim_callee::FIRE_END_TRIGGERS.call(c, ret)
    }
    const ZERO: FpuF32 = FpuF32::ZERO;
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save_fpr(c, 20, 0x40);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 23, 0x58);
        frame.save_fpr(c, 22, 0x50);
        frame.save_fpr(c, 21, 0x48);
        // s0 = seq, s2 = track index (the whole register: the tests below
        // compare all 64 bits)
        c.g[S2][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        let s = Addr(abi::arg(c, 0));
        let index = c.g[S2][0] as u32;
        let first_track = c.g[S2][0] == 0;
        let dt = FpuF32(c.f[12]);
        let track = s.off(abi::mult(c, index, seq::TRACK_SIZE));
        abi::set_word(c, V1, track.0);
        c.g[A0][0] = c.g[V1][0];
        c.g[V0][0] = c.g[V1][0];
        let seq_rate = s.off(seq::RATE).read_fpu(c);
        let rate = track.off(seq::T_RATE).read_fpu(c);
        let old_time = track.off(seq::T_TIME).read_fpu(c);
        let length = track.off(seq::T_LENGTH).read_fpu(c);
        let step = rate * seq_rate * dt;
        let mut time = old_time + step;
        c.f[0] = step.0;
        c.f[1] = seq_rate.0;
        c.f[20] = time.0;
        c.f[21] = length.0;
        c.f[22] = old_time.0;
        if first_track {
            c.cycles += 4;
            let dirty = s.off(seq::DIRTY).lw(c);
            c.g[V0][0] = dirty;
            s.off(seq::FIRED_MASK).sd(c, 0);
            s.off(seq::END_FLAG).sw(c, 0);
            if dirty != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                c.f[12] = old_time.0;
                anim_callee::FIRE_TRIGGERS_AT_TIME.call(c, 0x0031_362c)?;
                s.off(seq::DIRTY).sw(c, 0);
            }
        }
        c.cycles += 6;
        c.g[V0][0] = seq::TRACK_SIZE as u64;
        let track = s.off(abi::mult(c, index, seq::TRACK_SIZE));
        abi::set_word(c, V0, track.0);
        let mode = track.off(seq::T_MODE).lw(c);
        c.g[V1][0] = mode;
        // Every path that fires a range ends with `FireTriggersInRange(lo,
        // time)` returning to 0x3137a4 (`fire_to_time`), or with one other
        // range call; then v1 = 0x1c and the epilogue.
        'out: {
            if mode == 0 {
                // clamp to [0, length]
                c.cycles += 4;
                c.f[0] = 0;
                let at_start = time.le(ZERO);
                abi::set_fcc(c, at_start);
                let hit = if at_start {
                    c.cycles += 2;
                    Some((ZERO, 0x0031_3668, 4))
                } else {
                    c.cycles += 3;
                    let at_end = length.le(time);
                    abi::set_fcc(c, at_end);
                    if at_end {
                        c.cycles += 2;
                        Some((length, 0x0031_368c, 2))
                    } else {
                        None
                    }
                };
                if let Some((clamped, ret, cycles)) = hit {
                    time = clamped;
                    c.f[20] = time.0;
                    if !first_track {
                        c.cycles += 1;
                        c.g[V1][0] = seq::TRACK_SIZE as u64;
                        break 'out;
                    }
                    c.cycles += cycles;
                    fire_end(c, ret)?;
                }
                c.cycles += 2;
                c.g[V1][0] = seq::TRACK_SIZE as u64;
                if !first_track {
                    break 'out;
                }
                c.cycles += 2;
                fire_to_time(c, old_time, time)?;
            } else {
                // loop: wrap when outside [0, length)
                c.cycles += 4;
                c.f[23] = 0;
                let before = time.lt(ZERO);
                abi::set_fcc(c, before);
                let wrap = before || {
                    c.cycles += 3;
                    let past = length.le(time);
                    abi::set_fcc(c, past);
                    past
                };
                if !wrap {
                    c.cycles += 2;
                    c.g[V1][0] = seq::TRACK_SIZE as u64;
                    if !first_track {
                        break 'out;
                    }
                    c.cycles += 4;
                    fire_range(c, old_time, time, 0x0031_37c4)?;
                    c.cycles += 1;
                    c.g[V1][0] = seq::TRACK_SIZE as u64;
                    break 'out;
                }
                // loops = floor(time / length)
                c.cycles += 6;
                let ratio = time.div(length);
                let whole = FpuF32::from_i32(ratio.to_i32());
                c.f[0] = ratio.0;
                c.f[1] = whole.0;
                let negative = ratio.lt(whole);
                abi::set_fcc(c, negative);
                let floor = if negative {
                    c.cycles += 3;
                    c.g[AT][0] = 0x3f80_0000;
                    c.f[0] = FpuF32::ONE.0;
                    whole - FpuF32::ONE
                } else {
                    whole
                };
                c.f[1] = floor.0;
                c.cycles += 7;
                let loops = floor.to_i32();
                c.g[S1][0] = sx(loops as u32);
                let back = length * FpuF32::from_i32(loops);
                c.f[0] = back.0;
                time = time - back;
                c.f[20] = time.0;
                if !first_track {
                    c.cycles += 1;
                    c.g[V1][0] = seq::TRACK_SIZE as u64;
                    break 'out;
                }
                c.cycles += 5;
                fire_end(c, 0x0031_3700)?;
                let loops = c.g[S1][0] as i64;
                // the triggers between the old time and the end (or start)
                // of the clip, then those of whole loops and the new part
                let (lo, hi) = if loops >= 2 {
                    c.g[V0][0] = (loops < -1) as u64;
                    c.cycles += 3;
                    (ZERO, length)
                } else {
                    c.g[V0][0] = (loops < -1) as u64;
                    c.cycles += 2;
                    if loops < -1 {
                        c.cycles += 1;
                        (length, ZERO)
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = u64::MAX;
                        if loops == 1 {
                            c.cycles += 4;
                            let forward = old_time.le(time);
                            abi::set_fcc(c, forward);
                            if forward {
                                c.cycles += 2;
                                (ZERO, length)
                            } else {
                                c.cycles += 5;
                                fire_range(c, old_time, length, 0x0031_3750)?;
                                fire_to_time(c, ZERO, time)?;
                                break 'out;
                            }
                        } else {
                            c.cycles += 2;
                            c.g[V1][0] = seq::TRACK_SIZE as u64;
                            if loops != -1 {
                                break 'out;
                            }
                            c.cycles += 4;
                            let backward = time.le(old_time);
                            abi::set_fcc(c, backward);
                            if backward {
                                c.cycles += 1;
                                (length, ZERO)
                            } else {
                                c.cycles += 5;
                                fire_range(c, old_time, ZERO, 0x0031_3794)?;
                                fire_to_time(c, length, time)?;
                                break 'out;
                            }
                        }
                    }
                };
                c.cycles += 4;
                fire_range(c, lo, hi, 0x0031_377c)?;
                c.g[V1][0] = seq::TRACK_SIZE as u64;
            }
        }
        // epilogue: store the new time
        c.cycles += 14;
        frame.restore(c, S1, 0x20);
        let (index, size) = (c.g[S2][0] as u32, c.g[V1][0] as u32);
        let off = abi::mult(c, index, size);
        abi::set_word(c, A0, off);
        frame.restore_ra(c, 0);
        frame.restore(c, S2, 0x10);
        c.g[V0][0] = 0;
        frame.restore_fpr(c, 23, 0x58);
        frame.restore_fpr(c, 22, 0x50);
        frame.restore_fpr(c, 21, 0x48);
        let track = (c.g[S0][0] as u32).wrapping_add(off);
        abi::set_word(c, V1, track);
        let new_time = c.f[20];
        Addr(track).off(seq::T_TIME).write_u32(c, new_time);
        frame.restore(c, S0, 0x30);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    });

    /// The final `FireTriggersInRange(lo, time)` (5 cycles, returning to
    /// 0x3137a4), then v1 = 0x1c.
    fn fire_to_time(c: &mut Cpu, lo: FpuF32, time: FpuF32) -> Flow {
        c.cycles += 5;
        fire_range(c, lo, time, 0x0031_37a4)?;
        c.g[V1][0] = seq::TRACK_SIZE as u64;
        Ok(())
    }
}


/// `void AnimDataBase_EvalChanCurves(Chan* ch, int first, int count, float*
/// out, float frame)` (0x312c20): evaluate `count` (at least one) curves of
/// a decoded channel at `frame`, one float each into `out`. Kinds: 0 zero,
/// 1 constant, 2..4 polynomials in `frame`, 5 two linear pieces split at a
/// knee, 6/7/8/9 keyed: u8 keys at full / half rate, u16 keys, f32 keys,
/// interpolated linearly between this frame's key and the next one (scale
/// and bias for the integer keys). When the frame (and the half-rate frame)
/// falls on a key the cheaper key loop runs instead.
///
/// The channel (`ch`): +0 curve records, +4 u8 keys, +8 half-rate u8 keys,
/// +0xc u16 keys, +0x10 f32 keys, +0x14..+0x17 their per-frame strides.
/// A dispatcher: re-entered at a loop head after an unwind, with the loop
/// state in t0..t9, s0, a2, a3, v1, f4, f5, f12.
pub fn anim_data_base_eval_chan_curves(c: &mut Cpu) {
    let interpolate = match c.pc {
        eval::ENTRY => eval_chan_prologue(c),
        eval::LERP_HEAD => true,
        eval::KEY_HEAD => false,
        pc => {
            c.jump_out(pc, eval::ENTRY, eval::END);
            return;
        }
    };
    let finished = if interpolate { eval_lerp_loop(c) } else { eval_key_loop(c) };
    if finished {
        c.cycles += 5;
        let frame = Frame { size: 0x40 };
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore(c, S3, 0);
        frame.close(c);
    }
}


/// The prologue of `AnimDataBase_EvalChanCurves`: key pointers for this
/// frame and the next, the fractions; true for the interpolating loop.
pub(crate) fn eval_chan_prologue(c: &mut Cpu) -> bool {
    const HALF: FpuF32 = FpuF32(0x3f00_0000);
    const ZERO: FpuF32 = FpuF32::ZERO;
    c.cycles += 45;
    let ch = Addr(abi::arg(c, 0));
    let (first, count) = (abi::arg(c, 1), abi::arg(c, 2));
    let t = FpuF32(c.f[12]);
    c.g[AT][0] = HALF.0 as u64;
    let i = t.to_i32() as u32;
    abi::set_word(c, V1, i);
    let t_half = t * HALF;
    let j = t_half.to_i32() as u32;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S0, 0x30);
    frame.save(c, S1, 0x20);
    frame.save(c, S2, 0x10);
    frame.save(c, S3, 0);
    let first_off = abi::mult(c, first, eval::CURVE_SIZE);
    let count_off = abi::mult(c, count, eval::CURVE_SIZE);
    let stride_byte = ch.off(0x14).lbu(c) as u32;
    let stride_half = ch.off(0x15).lbu(c) as u32;
    let stride_u16 = ch.off(0x16).lbu(c) as u32;
    let stride_f32 = ch.off(0x17).lbu(c) as u32;
    let half_off = abi::mult(c, j, stride_half);
    let records = ch.lw(c) as u32;
    let u16_off = abi::mult(c, i, stride_u16) << 1;
    let f32_off = abi::mult(c, i, stride_f32) << 2;
    let curve = records.wrapping_add(first_off);
    let eps = Addr((c.g[GP][0] as u32).wrapping_sub(0x3228)).read_fpu(c);
    let byte_off = abi::mult(c, i, stride_byte);
    let half_base = ch.off(8).lw(c) as u32;
    let f32_keys = (ch.off(0x10).lw(c) as u32).wrapping_add(f32_off);
    let byte_keys = (ch.off(4).lw(c) as u32).wrapping_add(byte_off);
    let u16_base = ch.off(0xc).lw(c) as u32;
    let u16_keys = u16_base.wrapping_add(u16_off);
    let half_keys = half_base.wrapping_add(half_off);
    let end = curve.wrapping_add(count_off);
    let j_f = FpuF32::from_i32(j as i32);
    let mut frac = t - FpuF32::from_i32(i as i32);
    let mut frac_half = t_half - j_f;
    // the loop state, in the original's registers
    c.g[S2][0] = stride_byte as u64;
    c.g[S3][0] = stride_half as u64;
    c.g[T8][0] = stride_u16 as u64;
    c.g[T9][0] = stride_f32 as u64;
    c.g[A1][0] = sx(half_base);
    c.g[V0][0] = sx(u16_base);
    abi::set_word(c, V1, byte_off);
    abi::set_word(c, T2, f32_off);
    abi::set_word(c, T3, u16_off);
    abi::set_word(c, T5, curve);
    abi::set_word(c, T6, half_keys);
    abi::set_word(c, T4, f32_keys);
    abi::set_word(c, T1, byte_keys);
    abi::set_word(c, T0, u16_keys);
    abi::set_word(c, A2, end);
    c.f[0] = j_f.0;
    c.f[1] = t_half.0;
    c.f[2] = eps.0;
    // fractions below epsilon count as on the key
    let small = frac.lt(eps);
    abi::set_fcc(c, small);
    if small {
        c.cycles += 1;
        frac = ZERO;
    }
    c.cycles += 3;
    let small = frac_half.lt(eps);
    abi::set_fcc(c, small);
    if small {
        c.cycles += 1;
        frac_half = ZERO;
    }
    c.f[4] = frac.0;
    c.f[5] = frac_half.0;
    c.cycles += 5;
    c.f[0] = 0;
    c.g[V0][0] = sx(stride_u16 << 1);
    let on_key = frac.eq_s(ZERO);
    abi::set_fcc(c, on_key);
    if on_key {
        c.cycles += 4;
        let on_half_key = frac_half.eq_s(ZERO);
        abi::set_fcc(c, on_half_key);
        if on_half_key {
            c.g[V1][0] = Addr(curve).lw(c);
            return false;
        }
        c.cycles += 1;
    }
    // the next frame's keys
    c.cycles += 6;
    abi::set_word(c, V1, stride_f32 << 2);
    abi::set_word(c, S0, u16_keys.wrapping_add(stride_u16 << 1));
    abi::set_word(c, T7, f32_keys.wrapping_add(stride_f32 << 2));
    abi::set_word(c, T3, byte_keys.wrapping_add(stride_byte));
    abi::set_word(c, T2, half_keys.wrapping_add(stride_half));
    c.g[V1][0] = Addr(curve).lw(c);
    true
}


/// The curve-kind dispatch both loops start with (v1 = kind): v0 =
/// 0x490000, and for kinds 0..9 the table pointer in v0/v1 and the target
/// in a0 (6 more cycles). None for other kinds.
pub(crate) fn eval_kind(c: &mut Cpu, table_at: u32, table: &[u32; 10]) -> Option<usize> {
    c.cycles += 3;
    let kind = c.g[V1][0];
    c.g[V0][0] = 0x49_0000;
    if kind >= 10 {
        return None;
    }
    c.cycles += 6;
    abi::set_word(c, V1, ((kind as u32) << 2).wrapping_add(table_at));
    abi::set_word(c, V0, table_at);
    abi::set_word(c, A0, table[kind as usize]);
    Some(kind as usize)
}


/// Horner evaluation of the polynomial kinds 2..4: `((c0 t + c1) t + c2)
/// t + c3` with the coefficients from +4; leaves the coefficients after the
/// first in f1.. and the result in f0.
pub(crate) fn eval_poly(c: &mut Cpu, curve: Addr, t: FpuF32, terms: u32) -> FpuF32 {
    let mut v = curve.off(4).read_fpu(c);
    for k in 1..terms {
        let coef = curve.off(4 + 4 * k).read_fpu(c);
        c.f[k as usize] = coef.0;
        v = t * v + coef;
    }
    c.f[0] = v.0;
    v
}


/// The interpolating loop of `AnimDataBase_EvalChanCurves` (head 0x312d28).
/// False when an event unwound the thread.
pub(crate) fn eval_lerp_loop(c: &mut Cpu) -> bool {
    loop {
        if let Some(kind) = eval_kind(c, eval::LERP_TABLE_AT, &eval::LERP_TABLE) {
            let curve = Addr(c.g[T5][0] as u32);
            let out = Addr(c.g[A3][0] as u32);
            let t = FpuF32(c.f[12]);
            match kind {
                0 => {
                    c.cycles += 2;
                    out.sw(c, 0);
                }
                1 => {
                    c.cycles += 3;
                    let v = curve.off(4).read_fpu(c);
                    c.f[0] = v.0;
                    out.write_fpu(c, v);
                }
                2..=4 => {
                    c.cycles += 3 * kind as u64;
                    let v = eval_poly(c, curve, t, kind as u32);
                    out.write_fpu(c, v);
                }
                5 => {
                    c.cycles += 6;
                    let knee = curve.off(0x10).read_fpu(c);
                    let a = curve.off(4).read_fpu(c);
                    let before = t.le(knee);
                    abi::set_fcc(c, before);
                    let b = curve.off(8).read_fpu(c);
                    let k = curve.off(0xc).read_fpu(c);
                    c.f[2] = b.0;
                    c.f[3] = k.0;
                    if before {
                        c.cycles += 4;
                        let v = t * a + b;
                        c.f[0] = v.0;
                        c.f[1] = a.0;
                        out.write_fpu(c, v);
                    } else {
                        c.cycles += 5 + 1 + 1;
                        let at_knee = a * knee + b;
                        let tail = k * (t - knee);
                        let v = at_knee + tail;
                        c.f[0] = tail.0;
                        c.f[1] = v.0;
                        out.write_fpu(c, v);
                    }
                }
                6 | 7 => {
                    // u8 keys (6: full rate, t1/t3, f4; 7: half rate, t6/t2, f5)
                    c.cycles += 17 + 1;
                    let (cur_base, next_base, frac) = if kind == 6 {
                        (c.g[T1][0] as u32, c.g[T3][0] as u32, FpuF32(c.f[4]))
                    } else {
                        (c.g[T6][0] as u32, c.g[T2][0] as u32, FpuF32(c.f[5]))
                    };
                    let index = curve.off(0xc).lw(c) as u32;
                    let scale = curve.off(8).read_fpu(c);
                    let bias = curve.off(4).read_fpu(c);
                    let (next_at, cur_at) = (next_base.wrapping_add(index), cur_base.wrapping_add(index));
                    let next = Addr(next_at).lbu(c);
                    let cur = Addr(cur_at).lbu(c);
                    abi::set_word(c, A0, next_at);
                    abi::set_word(c, V0, cur_at);
                    c.g[A1][0] = next;
                    c.g[V1][0] = cur;
                    let cur_f = FpuF32::from_i32(cur as i32);
                    let step = frac * (FpuF32::from_i32(next as i32) - cur_f);
                    let v = (cur_f + step) * scale + bias;
                    c.f[0] = step.0;
                    c.f[1] = v.0;
                    c.f[2] = bias.0;
                    c.f[3] = scale.0;
                    out.write_fpu(c, v);
                }
                8 => {
                    // u16 keys (t0, next s0)
                    c.cycles += 18 + 1;
                    let index = (curve.off(0xc).lw(c) as u32) << 1;
                    let scale = curve.off(8).read_fpu(c);
                    let bias = curve.off(4).read_fpu(c);
                    let next_at = index.wrapping_add(c.g[S0][0] as u32);
                    let cur_at = index.wrapping_add(c.g[T0][0] as u32);
                    let next = Addr(next_at).lhu(c);
                    let cur = Addr(cur_at).lhu(c);
                    abi::set_word(c, A1, next_at);
                    abi::set_word(c, V0, cur_at);
                    c.g[A0][0] = next;
                    c.g[V1][0] = cur;
                    let cur_f = FpuF32::from_i32(cur as i32);
                    let step = FpuF32(c.f[4]) * (FpuF32::from_i32(next as i32) - cur_f);
                    let v = (cur_f + step) * scale + bias;
                    c.f[0] = step.0;
                    c.f[1] = v.0;
                    c.f[2] = bias.0;
                    c.f[3] = scale.0;
                    out.write_fpu(c, v);
                }
                _ => {
                    // 9: f32 keys (t4, next t7)
                    c.cycles += 8 + 1 + 1;
                    let index = (curve.off(0xc).lw(c) as u32) << 2;
                    let next_at = index.wrapping_add(c.g[T7][0] as u32);
                    let cur_at = index.wrapping_add(c.g[T4][0] as u32);
                    abi::set_word(c, V1, next_at);
                    abi::set_word(c, V0, cur_at);
                    let next = Addr(next_at).read_fpu(c);
                    let cur = Addr(cur_at).read_fpu(c);
                    let step = FpuF32(c.f[4]) * (next - cur);
                    let v = cur + step;
                    c.f[0] = step.0;
                    c.f[1] = v.0;
                    out.write_fpu(c, v);
                }
            }
            c.cycles += 1;
            abi::set_word(c, A3, out.0.wrapping_add(4));
        }
        c.cycles += 3;
        let next = (c.g[T5][0] as u32).wrapping_add(eval::CURVE_SIZE);
        abi::set_word(c, T5, next);
        if c.g[T5][0] == c.g[A2][0] {
            Frame { size: 0x40 }.restore(c, S0, 0x30);
            return true;
        }
        c.cycles += 2;
        c.g[V1][0] = Addr(next).lw(c);
        if back_edge_unwinds(c, eval::LERP_HEAD) {
            return false;
        }
    }
}


/// The key loop of `AnimDataBase_EvalChanCurves` (head 0x312f18): every
/// keyed curve reads this frame's key. False when an event unwound the
/// thread.
pub(crate) fn eval_key_loop(c: &mut Cpu) -> bool {
    loop {
        if let Some(kind) = eval_kind(c, eval::KEY_TABLE_AT, &eval::KEY_TABLE) {
            let curve = Addr(c.g[T5][0] as u32);
            let out = Addr(c.g[A3][0] as u32);
            let t = FpuF32(c.f[12]);
            match kind {
                0 => {
                    c.cycles += 2;
                    out.sw(c, 0);
                }
                1 => {
                    c.cycles += 2 + 1;
                    let v = curve.off(4).read_fpu(c);
                    c.f[0] = v.0;
                    out.write_fpu(c, v);
                }
                2..=4 => {
                    c.cycles += 3 * kind as u64 - 1 + 1;
                    let v = eval_poly(c, curve, t, kind as u32);
                    out.write_fpu(c, v);
                }
                5 => {
                    c.cycles += 6;
                    let knee = curve.off(0x10).read_fpu(c);
                    let a = curve.off(4).read_fpu(c);
                    let before = t.le(knee);
                    abi::set_fcc(c, before);
                    let b = curve.off(8).read_fpu(c);
                    let k = curve.off(0xc).read_fpu(c);
                    c.f[2] = b.0;
                    c.f[3] = k.0;
                    if before {
                        c.cycles += 3 + 1;
                        let v = t * a + b;
                        c.f[0] = v.0;
                        c.f[1] = a.0;
                        out.write_fpu(c, v);
                    } else {
                        c.cycles += 7;
                        let at_knee = a * knee + b;
                        let tail = k * (t - knee);
                        let v = at_knee + tail;
                        c.f[0] = tail.0;
                        c.f[1] = v.0;
                        out.write_fpu(c, v);
                    }
                }
                6 | 7 => {
                    c.cycles += 10 + 1;
                    let base = if kind == 6 { c.g[T1][0] as u32 } else { c.g[T6][0] as u32 };
                    let at = base.wrapping_add(curve.off(0xc).lw(c) as u32);
                    let scale = curve.off(8).read_fpu(c);
                    let bias = curve.off(4).read_fpu(c);
                    let key = Addr(at).lbu(c);
                    abi::set_word(c, V1, at);
                    c.g[V0][0] = key;
                    let v = FpuF32::from_i32(key as i32) * scale + bias;
                    c.f[0] = v.0;
                    c.f[1] = bias.0;
                    c.f[2] = scale.0;
                    out.write_fpu(c, v);
                }
                8 => {
                    c.cycles += 11 + 1;
                    let index = (curve.off(0xc).lw(c) as u32) << 1;
                    let scale = curve.off(8).read_fpu(c);
                    let bias = curve.off(4).read_fpu(c);
                    let at = index.wrapping_add(c.g[T0][0] as u32);
                    let key = Addr(at).lhu(c);
                    abi::set_word(c, V1, at);
                    c.g[V0][0] = key;
                    let v = FpuF32::from_i32(key as i32) * scale + bias;
                    c.f[0] = v.0;
                    c.f[1] = bias.0;
                    c.f[2] = scale.0;
                    out.write_fpu(c, v);
                }
                _ => {
                    c.cycles += 4 + 1;
                    let index = (curve.off(0xc).lw(c) as u32) << 2;
                    let at = index.wrapping_add(c.g[T4][0] as u32);
                    abi::set_word(c, V0, at);
                    let v = Addr(at).read_fpu(c);
                    c.f[0] = v.0;
                    out.write_fpu(c, v);
                }
            }
            c.cycles += 1;
            abi::set_word(c, A3, out.0.wrapping_add(4));
        }
        c.cycles += 3;
        let next = (c.g[T5][0] as u32).wrapping_add(eval::CURVE_SIZE);
        abi::set_word(c, T5, next);
        if c.g[T5][0] == c.g[A2][0] {
            c.cycles += 1;
            Frame { size: 0x40 }.restore(c, S0, 0x30);
            return true;
        }
        c.g[V1][0] = Addr(next).lw(c);
        if back_edge_unwinds(c, eval::KEY_HEAD) {
            return false;
        }
    }
}


// ---------------------------------------------------------------------------
// AnimController_DispatchEndEvents (0x312490).

/// `void func_00312490(AnimController* a)` (0x312490): scan the six animation
/// sequences (`a+0x50`, 8-byte list heads) and every node of each, collect the
/// nodes whose `+0xb0` flags have bit `0x3f` (`Flags64_Test(node+0xb0, 0x3f)`),
/// then call the handler vtable slot 3 (`a+0x58`) for each collected
/// `(sequence, node)` pair in order.
pub fn anim_controller_dispatch_end_events(c: &mut Cpu) {
    c.cycles += 12;
    let frame = Frame::open(c, 0x170);
    frame.save(c, S0, 0x160);
    frame.save(c, S2, 0x140);
    c.g[S0][0] = 0;
    frame.save(c, S3, 0x130);
    let anim = r32(c, A0);
    c.g[S2][0] = c.g[A0][0];
    frame.save(c, S1, 0x150);
    c.g[S3][0] = 0;
    frame.save(c, S4, 0x120);
    frame.save(c, S5, 0x110);
    frame.save_ra(c, 0x100);
    c.g[A0][0] = sx(r32(c, S0) << 3);
    guest(c, |c| {
        // Outer loop: i = 0..5.
        'seq_loop: loop {
            'no_nodes: {
                c.cycles += 9;
                let lists = lw(c, p(anim, 0x50));
                c.g[V0][0] = lists;
                c.g[S4][0] = c.g[A0][0];
                c.g[A1][0] = 0;
                c.g[A0][0] = sx((lists as u32).wrapping_add(r32(c, A0)));
                let index = c.g[S0][0];
                c.g[RA][0] = sx(0x0031_24d8);
                c.g[S5][0] = sx((index as u32).wrapping_add(1));
                callee::ANIM_SEQ_LIST_GET_AT.call(c, 0x0031_24d8)?;
                c.g[S0][0] = c.g[V0][0];
                let empty = c.g[S0][0] == 0;
                c.g[V0][0] = sx(r32(c, S3) << 3);
                if empty {
                    break 'no_nodes;
                }
                c.cycles += 3;
                c.g[S1][0] = sx(r32(c, V0).wrapping_add(r32(c, SP)));
                let mut node = r32(c, S0);
                c.g[A0][0] = sx(p(node, 0xb0));
                // Inner loop: walk this sequence's node chain.
                'node_loop: loop {
                    'step: {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0031_24f8);
                        c.g[A1][0] = 0x3f;
                        call_ptr(c, callee::FLAGS64_TEST, 0x0031_24f8)?;
                        if c.g[V0][0] == 0 {
                            node = lw(c, p(node, 0xc8)) as u32;
                            c.g[S0][0] = sx(node);
                            break 'step;
                        }
                        c.cycles += 7;
                        let lists = lw(c, p(anim, 0x50));
                        c.g[V0][0] = lists;
                        let n = r32(c, S3).wrapping_add(1);
                        c.g[S3][0] = sx(n);
                        Addr(r32(c, S1)).sw(c, sx(node) as u64);
                        let seq = (lists as u32).wrapping_add(r32(c, S4));
                        c.g[V0][0] = sx(seq);
                        Addr(p(r32(c, S1), 4)).sw(c, sx(seq) as u64);
                        c.g[S1][0] = sx(r32(c, S1).wrapping_add(8));
                        node = lw(c, p(node, 0xc8)) as u32;
                        c.g[S0][0] = sx(node);
                        break 'step;
                    }
                    c.cycles += 2;
                    let more = c.g[S0][0] != 0;
                    c.g[A0][0] = sx(p(r32(c, S0), 0xb0));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_24f0);
                        }
                        continue 'node_loop;
                    }
                    break 'no_nodes;
                }
            }
            c.cycles += 4;
            c.g[S0][0] = c.g[S5][0];
            c.g[V0][0] = ((c.g[S0][0] as i64) < 6i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx(r32(c, S0) << 3);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0031_24c0);
                }
                continue 'seq_loop;
            }
            break;
        }
        // Dispatch.
        'dispatch: {
            c.cycles += 2;
            let none = (c.g[S3][0] as i64) <= 0;
            c.g[S1][0] = c.g[S3][0];
            if none {
                break 'dispatch;
            }
            c.cycles += 3;
            c.g[S0][0] = c.g[SP][0];
            let vt = lw(c, p(anim, 0x58));
            c.g[V0][0] = vt;
            loop {
                c.cycles += 10;
                c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                c.g[A1][0] = lw(c, p(r32(c, S0), 4));
                c.g[A0][0] = lh(c, p(vt as u32, 0x18));
                c.g[A2][0] = lw(c, p(r32(c, S0), 0));
                let f = lw(c, p(vt as u32, 0x1c));
                c.g[V1][0] = f;
                c.g[A0][0] = sx(anim.wrapping_add(c.g[A0][0] as u32));
                let target = f as u32;
                c.g[RA][0] = sx(0x0031_2568);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(8));
                call_ptr(c, target, 0x0031_2568)?;
                if c.g[S1][0] == 0 {
                    break 'dispatch;
                }
                let vt = lw(c, p(anim, 0x58));
                c.g[V0][0] = vt;
                if event_due(c) {
                    deliver_event_here(c, 0x0031_2548);
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S1, 0x150);
        frame.restore(c, S2, 0x140);
        frame.restore(c, S3, 0x130);
        frame.restore(c, S4, 0x120);
        frame.restore(c, S5, 0x110);
        frame.restore(c, S0, 0x160);
        frame.restore_ra(c, 0x100);
    frame.close(c);
        Ok(())
    })
}


/// `sw v` to `a + off`.
#[inline(always)]
pub(crate) fn sw(c: &mut Cpu, a: u32, off: u32, v: u64) {
    Addr(p(a, off)).sw(c, v);
}


// ---------------------------------------------------------------------------
// `AnimController_PlayAnimDirect` (0x003129e0).
//
// `void AnimController::PlayAnimDirect(AnimController* self, AnimTable* table,
//  int anim, AnimSeqList* seq, float speed, int flags)`: resolve the table
// entry, look up its bank, play the animation and record the sequence.

pub fn anim_controller_play_anim_direct(c: &mut Cpu) {
    guest(c, |c| {

        'b_312a38: {
        c.cycles += 19;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S1][0] = c.g[A1][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S2][0] = c.g[A0][0];
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S4][0] = c.g[A2][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.mov_s(20, 12);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x00312a18);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003115c8, 0x00312a18)?;
        c.mtc1(0, 0);
        c.c_lt_s(20, 0);
        let t = !c.fcc();
        c.g[S3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        if t {
        break 'b_312a38;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x00312a34);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003115c8, 0x00312a34)?;
        c.lwc1(20, (c.g[V0][0] as u32).wrapping_add(0x10));
        break 'b_312a38;
        }
        }
        c.cycles += 25;
        c.g[V0][0] = 0x490000;
        c.g[S0][0] = sx((c.g[S3][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x27f8));
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A1][0] = sx((c.g[S3][0] as u32) << 3);
        c.g[T1][0] = c.lw((c.g[V0][0] as u32));
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S4][0];
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.g[RA][0] = sx(0x00312a70);
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x00311f00, 0x00312a70)?;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sw((c.g[S0][0] as u32), c.g[V0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t; return Ok(());

        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::*;
    use crate::difftest::fill;
    use crate::difftest::put_words;
    use ssx3_ee as ee;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::put_vec;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;
    pub(crate) use crate::game::ai::aianimeventmap::tests::MEM_MUTEX;
    pub(crate) use crate::game::ai::aianimeventmap::tests::alloc_setup;
    pub(crate) use crate::game::visualfx::worldlightman::tests::f;
    pub(crate) use crate::game::visualfx::worldlightman::tests::pick;
    pub(crate) use crate::game::animation::animsequencer::tests::sequence;
    pub(crate) use crate::game::animation::animmodel::tests::eval_constants;
    pub(crate) use crate::game::animation::animmodel::tests::curves;
    pub(crate) use crate::game::animation::animmodel::tests::channel;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::SEEDS_3 as SEEDS_2;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::BASE;


    #[test]
    fn anim_controller_get_current_anim_length_matches() {
        check(ee::s10::f_00312790, anim_controller_get_current_anim_length, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2790, 1);
            let this = DATA;
            c.g[A0][0] = this as u64;
            let vt = vtable(c, &mut r, &[0x20]);
            c.bus.write32(this + 0x58, vt);
            let index = if seed % 4 == 0 { 0x207 } else { (seed % 5) + 1 };
            c.bus.write32(crate::difftest::fakes::LOG_RESULTS, index);
            // Anim table at gp+0xd8c: entry +0x1030 holds the packed index.
            let table = DATA + 0x200;
            c.bus.write32(GAME_GP + 0xd8c, table);
            c.bus.write32(table + index * 4 + 0x1030, 0x203);
            // Channel-cache table at gp+0xd08.
            c.bus.write32(GAME_GP + 0xd08, DATA + 0x400);
            c.bus.write32(DATA + 0x400 + 0x0c, DATA + 0x600);
            c.bus.write32(DATA + 0x608, 0x100);
            c.bus.write16(DATA + 0x134, 5);
            c.bus.write32(GAME_GP - 0x3230, r.edge_f32());
        });
    }


    #[test]
    fn anim_controller_get_current_anim_trigger_time_matches() {
        check(ee::s10::f_00312820, anim_controller_get_current_anim_trigger_time, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2820, 1);
            let this = DATA;
            c.g[A0][0] = this as u64;
            let vt = vtable(c, &mut r, &[0x20]);
            c.bus.write32(this + 0x58, vt);
            let index = if seed % 4 == 0 { 0x207 } else { (seed % 5) + 1 };
            c.bus.write32(crate::difftest::fakes::LOG_RESULTS, index);
            let table = DATA + 0x200;
            c.bus.write32(GAME_GP + 0xd8c, table);
            c.bus.write32(table + index * 4 + 0x1030, 0x203);
            c.bus.write32(GAME_GP + 0xd08, DATA + 0x400);
            c.bus.write32(DATA + 0x400 + 0x0c, DATA + 0x600);
            c.bus.write32(DATA + 0x608, 0x100);
            c.bus.write16(DATA + 0x710, 0);
            c.bus.write16(DATA + 0x712, 10);
            c.bus.write32(DATA + 0x610, DATA + 0x800);
            for i in 0..12u32 {
                c.bus.write16(DATA + 0x800 + i * 4, r.next_u32() as u16);
            }
            c.g[A2][0] = (seed % 12) as u64;
            c.bus.write32(GAME_GP - 0x322c, r.edge_f32());
        });
    }


    #[test]
    fn anim_controller_ctor_matches() {
        check(ee::s10::f_003118d0, anim_controller_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x18d0, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
        });
    }


    #[test]
    fn anim_controller_dtor_matches() {
        check(ee::s10::f_00311958, anim_controller_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x1958, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
            c.bus.write32(DATA + 0x50, 0);
        });
    }


    #[test]
    fn anim_controller_clear_layer_matches() {
        check(ee::s10::f_00311e38, anim_controller_clear_layer, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x1e38, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 4) as u64;
            // A layer's sequence list: head at +4 (empty keeps the free path
            // short and safe).
            c.bus.write32(DATA + 0x50, DATA + 0x100);
            c.bus.write32(DATA + 0x100 + 4, 0);
        });
    }


    #[test]
    fn anim_controller_get_layer_progress_matches() {
        check(ee::s10::f_00312ab0, anim_controller_get_layer_progress, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2ab0, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = 0;
            c.bus.write32(DATA + 0x50, DATA + 0x100);
            c.bus.write32(DATA + 0x100 + 4, DATA + 0x300);
            c.bus.write32(DATA + 0x300 + 8, r.edge_f32());
            c.bus.write32(DATA + 0x300 + 0x10, r.edge_f32());
        });
    }


    /// A chain of trigger nodes at DATA + 0x1000, each `{bit, count, time,
    /// next}`, with the header's `+0xac` pointing at the first. `mask` controls
    /// how many counts are non-zero.
    pub(crate) fn build_triggers(c: &mut Cpu, r: &mut Rng, seq: u32, mask: u64) {
        let n = (r.next_u32() % 4) as u32;
        let mut node = seq + 0x1000;
        c.bus.write32(seq + TRIGGERS, if n == 0 { 0 } else { node });
        for i in 0..n {
            c.bus.write32(node + trigger::BIT, r.next_u32() % 64);
            let count = if mask >> i & 1 != 0 { r.next_u32() % 3 + 1 } else { 0 };
            c.bus.write32(node + trigger::COUNT, count);
            c.bus.write32(node + trigger::TIME, r.edge_f32());
            c.bus.write32(node + trigger::NEXT, if i + 1 < n { node + 0x20 } else { 0 });
            node += 0x20;
        }
    }


    #[test]
    fn anim_sequence_fire_end_triggers_matches() {
        check_events(
            ssx3_ee::s10::f_003139a8,
            anim_sequence_fire_end_triggers,
            60,
            &|c, seed| {
                let mut r = Rng(0x3139_a800 ^ seed as u64);
                scramble(c, &mut r);
                let seq = DATA;
                c.g[A0][0] = seq as u64;
                c.bus.write64(seq + MASK_A, ((r.next_u32() as u64) << 32) | r.next_u32() as u64);
                c.bus.write64(seq + MASK_B, ((r.next_u32() as u64) << 32) | r.next_u32() as u64);
                let mask = (r.next_u32() as u64) | 1;
                build_triggers(c, &mut r, seq, mask);
                let mode = schedule(c, &mut r, seed, 0x0031_39a8);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }


    #[test]
    fn anim_sequence_fire_triggers_at_time_matches() {
        check_events(
            ssx3_ee::s10::f_00313938,
            anim_sequence_fire_triggers_at_time,
            60,
            &|c, seed| {
                let mut r = Rng(0x3139_3800 ^ seed as u64);
                scramble(c, &mut r);
                let seq = DATA;
                c.g[A0][0] = seq as u64;
                c.bus.write64(seq + MASK_A, ((r.next_u32() as u64) << 32) | r.next_u32() as u64);
                c.bus.write64(seq + MASK_B, ((r.next_u32() as u64) << 32) | r.next_u32() as u64);
                let mask = (r.next_u32() as u64) | 1;
                build_triggers(c, &mut r, seq, mask);
                // Half the seeds match a node's time exactly.
                let target = if seed % 2 == 0 {
                    let first = c.bus.read32(seq + TRIGGERS);
                    if first != 0 { c.bus.read32(first + trigger::TIME) } else { r.edge_f32() }
                } else {
                    r.edge_f32()
                };
                c.f[12] = target;
                let mode = schedule(c, &mut r, seed, 0x0031_3938);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }


    #[test]
    fn anim_sequence_fire_triggers_in_range_matches() {
        check_events(
            ssx3_ee::s10::f_00313868,
            anim_sequence_fire_triggers_in_range,
            80,
            &|c, seed| {
                let mut r = Rng(0x3138_6800 ^ seed as u64);
                scramble(c, &mut r);
                let seq = DATA;
                c.g[A0][0] = seq as u64;
                c.bus.write64(seq + MASK_A, ((r.next_u32() as u64) << 32) | r.next_u32() as u64);
                c.bus.write64(seq + MASK_B, ((r.next_u32() as u64) << 32) | r.next_u32() as u64);
                let mask = (r.next_u32() as u64) | 1;
                build_triggers(c, &mut r, seq, mask);
                let (lo, hi) = match seed % 4 {
                    0 => (1.0f32.to_bits(), 1.0f32.to_bits()),
                    1 => (0.0f32.to_bits(), 10.0f32.to_bits()),
                    2 => (10.0f32.to_bits(), 0.0f32.to_bits()),
                    _ => (r.f32(10.0), r.f32(10.0)),
                };
                c.f[12] = lo;
                c.f[13] = hi;
                let mode = schedule(c, &mut r, seed, 0x0031_3868);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }


    #[test]
    fn anim_sequence_ctor_matches() {
        check_events(
            ssx3_ee::s10::f_00313508,
            anim_sequence_ctor,
            40,
            &|c, seed| {
                let mut r = Rng(0x3135_0800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                let mode = schedule(c, &mut r, seed, 0x0031_3508);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }


    #[test]
    fn anim_sequence_advance_track_matches() {
        check(ee::s10::f_003135b0__generated, anim_sequence_advance_track, 400, &|c, seed| {
            let mut r = Rng(0xadfa_7ace ^ (seed as u64) << 12);
            scramble(c, &mut r);
            let s = DATA;
            sequence(c, &mut r, s);
            let index = [0u32, 0, 1, 2][seed as usize % 4];
            let t = s + index * 0x1c;
            let length = pick(&mut r, &[2.0f32, 1.0, 0.5]);
            c.bus.write32(t + 0x10, f(length));
            // loop mode: 0 clamps, anything else loops
            c.bus.write32(t + 0x1c, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
            let old = pick(&mut r, &[0.5f32, 0.0, 0.4, 5.0, -3.0, -0.5, 1.9]);
            c.bus.write32(t + 0x8, f(old * length));
            let dt = pick(&mut r, &[0.1f32, -0.7, 2.6, 7.0, -5.0, 0.0, 1.5, -2.0, 3.0, -0.25, -3.5]);
            if seed % 7 == 3 {
                c.bus.write32(t + 0xc, r.edge_f32());
                c.bus.write32(s + 0x90, r.edge_f32());
                c.f[12] = r.edge_f32();
            } else {
                c.bus.write32(t + 0xc, f(1.0));
                c.bus.write32(s + 0x90, f(pick(&mut r, &[1.0f32, 0.5, 2.0])));
                c.f[12] = f(dt * length);
            }
            c.bus.write32(s + 0xc4, (seed % 3 == 0) as u32);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = index as u64;
        });
    }


    #[test]
    fn anim_data_base_eval_chan_curves_matches() {
        check(ee::s10::f_00312c20__generated, anim_data_base_eval_chan_curves, 400, &|c, seed| {
            let mut r = Rng(0xe7a1_c0e5 ^ (seed as u64) << 12);
            scramble(c, &mut r);
            eval_constants(c);
            let (ch, recs, keys, out) = (DATA, DATA + 0x100, DATA + 0x1000, DATA + 0x3000);
            channel(c, &mut r, ch, recs, keys);
            curves(c, &mut r, recs, keys + 0x400, 12);
            let frame = (r.next_u32() % 16) as f32;
            let t = match seed % 6 {
                0 => frame - frame % 2.0,                 // on a key and a half-rate key
                1 => frame - frame % 2.0 + 1.0,           // on a key only
                2 => frame + 0.004,                       // within epsilon of a key
                3 => frame + 0.5,
                _ => frame + (r.next_u32() % 1000) as f32 / 1000.0,
            };
            c.f[12] = f(t);
            let first = r.next_u32() % 4;
            c.g[A0][0] = sx(ch);
            c.g[A1][0] = first as u64;
            c.g[A2][0] = (1 + r.next_u32() % 6) as u64;
            c.g[A3][0] = sx(out);
            c.pc = eval::ENTRY;
            if seed % 5 == 4 {
                // re-entry at a loop head after an unwind: the frame is
                // open and the loop state is in the registers
                c.pc = if seed % 2 == 0 { eval::LERP_HEAD } else { eval::KEY_HEAD };
                c.g[SP][0] = sx(crate::difftest::SP - 0x40);
                let cur = recs + first * 0x14;
                c.g[T5][0] = sx(cur);
                c.g[A2][0] = sx(cur + (1 + r.next_u32() % 5) * 0x14);
                c.g[V1][0] = c.bus.read32(cur) as u64;
                for (reg, base) in [(T1, 0u32), (T3, 0x10), (T6, 0x100), (T2, 0x110), (T0, 0x200), (S0, 0x210), (T4, 0x300), (T7, 0x310)] {
                    c.g[reg][0] = sx(keys + base + (r.next_u32() % 8) * 4);
                }
                c.f[4] = r.f32(1.0);
                c.f[5] = r.f32(1.0);
            }
        });
    }


    #[test]
    fn anim_controller_dispatch_end_events_matches() {
        check(ee::s10::f_00312490__generated, anim_controller_dispatch_end_events, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0x2490, 1);
            let anim = BASE;
            let lists = BASE + 0x1_0000;
            let nodes = BASE + 0x2_0000;
            c.bus.write32(anim + 0x50, lists);
            let vt = vtable(c, &mut r, &[0x18]);
            c.bus.write32(anim + 0x58, vt);
            for i in 0..6u32 {
                let len = (seed / (i + 1)) % 3;
                let mut head = 0u32;
                for k in 0..len {
                    let n = nodes + 0x100 * (i * 3 + k);
                    c.bus.write32(n + 0xc8, head);
                    c.bus.write32(n + 0xb0, r.next_u32());
                    c.bus.write32(
                        n + 0xb4,
                        if (seed + k + i) % 3 == 0 { 0x8000_0000 } else { 0 },
                    );
                    head = n;
                }
                c.bus.write32(lists + 8 * i, r.next_u32());
                c.bus.write32(lists + 8 * i + 4, head);
            }
            c.g[A0][0] = sx(anim);
        });
    }
}
