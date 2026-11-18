//! Staging module oc439 (wave wave222): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc439`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Flow, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0027_8710,
        script_engine_clear_skip_delay_on_enter,
        "ScriptEngine_ClearSkipDelayOnEnter",
    ),
    (
        0x0027_72b8,
        script_list_get_field_b0,
        "ScriptList_GetFieldB0",
    ),
    (
        0x0016_ca10,
        camera_trigger_man_empty_hook,
        "CameraTriggerMan_EmptyHook",
    ),
    (
        0x0010_f2b8,
        rider_reset_mission_metrix,
        "Rider_ResetMissionMetrix",
    ),
    (0x0011_1948, rider_base_update_fx, "RiderBase_UpdateFX"),
    (0x0011_19f8, rider_base_render_fx, "RiderBase_RenderFX"),
    (0x0011_1d98, rider_base_load_state, "RiderBase_LoadState"),
    (
        0x0011_3130,
        rider_get_start_dist_to_finish,
        "Rider_GetStartDistToFinish",
    ),
    (
        0x0011_3e18,
        race_state_end_race_enter,
        "RaceStateEndRace_Enter",
    ),
    (
        0x0011_95a8,
        rider_metrix_show_hud_msg_2a,
        "RiderMetrix_ShowHUDMsg2A",
    ),
    (
        0x0011_95d8,
        rider_metrix_show_hud_msg_2b,
        "RiderMetrix_ShowHUDMsg2B",
    ),
    (
        0x0011_9938,
        rider_metrix_start_uber_grind,
        "RiderMetrix_StartUberGrind",
    ),
    (
        0x0011_9c38,
        rider_metrix_finish_handplant,
        "RiderMetrix_FinishHandplant",
    ),
    (
        0x0011_9ef8,
        rider_metrix_award_points,
        "RiderMetrix_AwardPoints",
    ),
    (
        0x0011_a088,
        rider_metrix_show_wrong_way,
        "RiderMetrix_ShowWrongWay",
    ),
    (
        0x0011_a0e0,
        rider_metrix_show_hud_msg_1a,
        "RiderMetrix_ShowHUDMsg1A",
    ),
    (
        0x0011_b2c0,
        rider_tuning_fill_table_4a6310,
        "RiderTuning_FillTable4a6310",
    ),
    (0x0011_b978, rider_dtor, "Rider_dtor"),
    (
        0x0011_fef8,
        rider_get_sub_state_name,
        "Rider_GetSubStateName",
    ),
    (
        0x0011_ff18,
        rider_get_control_input,
        "Rider_GetControlInput",
    ),
];

// -- Callees ----------------------------------------------------------------

/// `RiderMetrix_ShowHUDNumber(self, kind, value, arg, duration)` (0x117b88).
pub(crate) const OC439_SHOW_HUD_NUMBER: Callee = Callee {
    addr: 0x0011_7b88,
    f: ee::s00::RiderMetrix_ShowHUDNumber,
};

/// `RiderMetrix_ShowHUDText(self, type, text, repeat, scale)` (0x117ae8).
pub(crate) const OC439_SHOW_HUD_TEXT: Callee = Callee {
    addr: 0x0011_7ae8,
    f: ee::s00::RiderMetrix_ShowHUDText,
};

/// `RiderMetrix_ShowHUDMsg1A(self)` (0x11a0e0).
pub(crate) const OC439_SHOW_HUD_MSG_1A: Callee = Callee {
    addr: 0x0011_a0e0,
    f: ee::s00::RiderMetrix_ShowHUDMsg1A,
};

/// `RiderMetrix_FinishTrick(...)` (0x11a228).
pub(crate) const OC439_FINISH_TRICK: Callee = Callee {
    addr: 0x0011_a228,
    f: ee::s00::RiderMetrix_FinishTrick,
};

/// `RiderMetrix_ResetTrick(self)` (0x117838).
pub(crate) const OC439_RESET_TRICK: Callee = Callee {
    addr: 0x0011_7838,
    f: ee::s00::RiderMetrix_ResetTrick,
};

/// `RiderBase_GetSubStateName(sub_state)` (0x111860).
pub(crate) const OC439_GET_SUB_STATE_NAME: Callee = Callee {
    addr: 0x0011_1860,
    f: ee::s00::RiderBase_GetSubStateName,
};

/// `a0 = regs[base] + off; call target` (the sub-object calls of the
/// rider's per-frame / load / save chains).
#[inline]
fn oc439_call_sub(c: &mut Cpu, base: usize, off: u32, target: u32, ret: u32) -> Flow {
    c.g[A0][0] = sx((c.g[base][0] as u32).wrapping_add(off));
    call_ptr(c, target, ret)
}

// -- Trivial leaves -----------------------------------------------------------

/// `void ScriptEngine_ClearSkipDelayOnEnter(ScriptEngine* self)` (0x278710):
/// `self->skip_delay_on_enter = 0` (+0x554).
pub fn script_engine_clear_skip_delay_on_enter(c: &mut Cpu) {
    c.cycles += 2;
    let this = c.g[A0][0] as u32;
    c.sw(this.wrapping_add(0x554), 0);
    ret(c);
}

/// `int ScriptList_GetFieldB0(ScriptList* self)` (0x2772b8): the word at +0xb0.
pub fn script_list_get_field_b0(c: &mut Cpu) {
    c.cycles += 2;
    let this = c.g[A0][0] as u32;
    c.g[V0][0] = c.lw(this.wrapping_add(0xb0));
    ret(c);
}

/// `void CameraTriggerMan_EmptyHook()` (0x16ca10): does nothing.
pub fn camera_trigger_man_empty_hook(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `float Rider_GetStartDistToFinish(Rider* self)` (0x113130): the float at
/// +0x4d8.
pub fn rider_get_start_dist_to_finish(c: &mut Cpu) {
    c.cycles += 2;
    let this = c.g[A0][0] as u32;
    c.lwc1(0, this.wrapping_add(0x4d8));
    ret(c);
}

/// `void RiderMetrix_StartUberGrind(RiderMetrix* self, int a, int b)`
/// (0x119938): clear the grind progress float (+0x6c), start the grind
/// (+0x5c = 1) and record `b` at +0x20 / +0xc and `a` at +0x70. Leaves the
/// cleared float in f0 and 1 in v0.
pub fn rider_metrix_start_uber_grind(c: &mut Cpu) {
    c.cycles += 8;
    let this = c.g[A0][0] as u32;
    c.sw(this.wrapping_add(0x6c), 0);
    c.g[V0][0] = 1;
    c.sw(this.wrapping_add(0x20), c.g[A2][0]);
    c.lwc1(0, this.wrapping_add(0x6c));
    c.sw(this.wrapping_add(0x70), c.g[A1][0]);
    c.sw(this.wrapping_add(0x5c), c.g[V0][0]);
    c.sw(this.wrapping_add(0xc), c.g[A2][0]);
    ret(c);
}

// -- HUD message wrappers -------------------------------------------------------

/// The 0x10-byte-frame HUD wrappers: `RiderMetrix_ShowHUDNumber(self, kind,
/// value, 0, scale)` with the scale in f12 (its bits go through `at`),
/// returning 0.0f. `value` is the wrapper's second argument (`a1`), or 0.
fn oc439_show_hud_number_msg(
    c: &mut Cpu,
    scale: u32,
    kind: u32,
    value_from_a1: bool,
    ret_addr: u32,
) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = if value_from_a1 { c.g[A1][0] } else { 0 };
        c.g[AT][0] = scale as u64;
        c.mtc1(1, 12);
        frame.save_ra(c, 0);
        c.g[A1][0] = kind as u64;
        c.g[A3][0] = 0;
        OC439_SHOW_HUD_NUMBER.call(c, ret_addr)?;
        c.mtc1(0, 0);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float RiderMetrix_ShowHUDMsg2A(RiderMetrix* self, int value)` (0x1195a8,
/// the split-time event): show HUD number 0x2a with `value`, 5.0 scale.
pub fn rider_metrix_show_hud_msg_2a(c: &mut Cpu) {
    oc439_show_hud_number_msg(c, 0x40a0_0000, 0x2a, true, 0x0011_95c8);
}

/// `float RiderMetrix_ShowHUDMsg2B(RiderMetrix* self, int value)` (0x1195d8):
/// as 2A with number 0x2b.
pub fn rider_metrix_show_hud_msg_2b(c: &mut Cpu) {
    oc439_show_hud_number_msg(c, 0x40a0_0000, 0x2b, true, 0x0011_95f8);
}

/// `float RiderMetrix_ShowHUDMsg1A(RiderMetrix* self)` (0x11a0e0, the
/// mission-start event): show HUD number 0x1a with value 0, 1.0 scale.
pub fn rider_metrix_show_hud_msg_1a(c: &mut Cpu) {
    oc439_show_hud_number_msg(c, 0x3f80_0000, 0x1a, false, 0x0011_a100);
}

/// `float RiderMetrix_ShowWrongWay(RiderMetrix* self)` (0x11a088): show the
/// "wrong way" text (string at 0x457888) as HUD type 0x33 at 1.5 scale.
pub fn rider_metrix_show_wrong_way(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[AT][0] = 0x3fc0_0000;
        c.mtc1(1, 12);
        c.g[A2][0] = 0x45_0000;
        frame.save_ra(c, 0);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x7888));
        c.g[A1][0] = 0x33;
        c.g[A3][0] = 0;
        OC439_SHOW_HUD_TEXT.call(c, 0x0011_a0ac)?;
        c.mtc1(0, 0);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Rider_ResetMissionMetrix(Rider* self)` (0x10f2b8): fire the
/// mission-start HUD event on the rider's metrics (+0x790).
pub fn rider_reset_mission_metrix(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x790));
        OC439_SHOW_HUD_MSG_1A.call(c, 0x0010_f2c8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RiderMetrix_FinishHandplant(RiderMetrix* self)` (0x119c38): count the
/// handplant (+0x80), finish the running trick, reset the trick state and
/// clear +0x30. Returns the float the finish left in f20 (restored after).
pub fn rider_metrix_finish_handplant(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x20);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
        c.g[T1][0] = 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x80), c.g[V0][0]);
        OC439_FINISH_TRICK.call(c, 0x0011_9c70)?;
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S0][0];
        OC439_RESET_TRICK.call(c, 0x0011_9c7c)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x30), 0);
        c.mov_s(0, 20);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

// -- Rider wrappers -------------------------------------------------------------

/// `const char* Rider_GetSubStateName(Rider* self)` (0x11fef8): the rider
/// base's sub-state name for the sub state at +0x77c.
pub fn rider_get_sub_state_name(c: &mut Cpu) {
    // A resume entry inside the generated frame: its dispatcher continues it
    // (at install this becomes `f_0011fef8__generated`).
    if c.pc != 0x0011_fef8 {
        return ee::s00::f_0011fef8__generated(c);
    }
    tail_wrapper(
        c,
        Some(0x77c),
        Target::Direct(OC439_GET_SUB_STATE_NAME),
        0x0011_ff08,
    );
}

/// `int Rider_GetControlInput(Rider* self)` (0x11ff18): virtual call on the
/// control object at `self->sub + 0xde8` (vtable slot +8 / +0xc).
pub fn rider_get_control_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x77c));
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xde8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0011_ff38)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// -- Rider FX passes ---------------------------------------------------------------

/// `RiderBase_UpdateFX`'s sub-object calls: (offset in the rider base, callee,
/// return address): the spray, wake and trail FX objects' per-frame updates.
pub(crate) const OC439_UPDATE_FX_CALLS: [(u32, u32, u32); 12] = [
    (0xd30, 0x0039_2d18, 0x0011_198c),
    (0x3b0, 0x002d_d0b8, 0x0011_1994),
    (0x470, 0x002d_abc8, 0x0011_199c),
    (0x520, 0x002e_8938, 0x0011_19a4),
    (0x610, 0x002e_66b8, 0x0011_19ac),
    (0x9c0, 0x002e_add0, 0x0011_19b4),
    (0xad0, 0x002e_f6d0, 0x0011_19bc),
    (0xaf0, 0x002d_4c08, 0x0011_19c4),
    (0xb00, 0x002e_39d8, 0x0011_19cc),
    (0xb40, 0x002d_f920, 0x0011_19d4),
    (0xc70, 0x002f_1150, 0x0011_19dc),
    (0xd20, 0x002f_6518, 0x0011_19e4),
];

/// `RiderBase_RenderFX`'s sub-object calls (same shape as the update's).
pub(crate) const OC439_RENDER_FX_CALLS: [(u32, u32, u32); 11] = [
    (0x3b0, 0x002d_dab8, 0x0011_1a3c),
    (0x470, 0x002d_b478, 0x0011_1a44),
    (0x520, 0x002e_a480, 0x0011_1a4c),
    (0x610, 0x002e_7a10, 0x0011_1a54),
    (0x9c0, 0x002e_b198, 0x0011_1a5c),
    (0xad0, 0x002e_f950, 0x0011_1a64),
    (0xaf0, 0x002d_5048, 0x0011_1a6c),
    (0xb00, 0x002e_3af8, 0x0011_1a74),
    (0xb40, 0x002e_24d0, 0x0011_1a7c),
    (0xc70, 0x002f_1510, 0x0011_1a84),
    (0xd20, 0x002f_6908, 0x0011_1a8c),
];

/// The shape of `RiderBase_UpdateFX` / `RiderBase_RenderFX`: when the FX
/// switch (gp+0x1444) is set, ask the rider (`self->rider + 0x6c0`, virtual
/// call at slot +0x48) whether to skip; unless it says so, run every FX
/// object's call in `calls`. `first_ret` is the virtual call's return
/// address, `chain_cycles` the cycles of the block before the calls.
fn oc439_rider_fx_pass(c: &mut Cpu, first_ret: u32, chain_cycles: u64, calls: &[(u32, u32, u32)]) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1444));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let fx_off = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        'done: {
            if !fx_off {
                c.cycles += 9;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                call_ptr(c, target, first_ret)?;
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x10);
                    break 'done;
                }
            }
            c.cycles += chain_cycles;
            for &(off, target, ret) in calls {
                oc439_call_sub(c, S0, off, target, ret)?;
            }
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderBase_UpdateFX(RiderBase* self)` (0x111948): per-frame update of
/// the rider's FX objects (board trail, spray, wake ...), skipped when the FX
/// switch is off or the rider says to.
pub fn rider_base_update_fx(c: &mut Cpu) {
    oc439_rider_fx_pass(c, 0x0011_197c, 25, &OC439_UPDATE_FX_CALLS);
}

/// `void RiderBase_RenderFX(RiderBase* self)` (0x1119f8): the render pass of
/// the same FX objects.
pub fn rider_base_render_fx(c: &mut Cpu) {
    oc439_rider_fx_pass(c, 0x0011_1a2c, 23, &OC439_RENDER_FX_CALLS);
}

/// `void RaceStateEndRace_Enter()` (0x113e18): tell the game-comm object (the
/// global at gp-0x204), if there is one, that the race ended (0x258a48).
pub fn race_state_end_race_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
        let none = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 2;
            call_ptr(c, 0x0025_8a48, 0x0011_3e30)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// -- RiderMetrix_AwardPoints ---------------------------------------------------------

/// The jump table of `RiderMetrix_AwardPoints`' award kind (5 entries).
#[cfg(test)]
pub(crate) const OC439_AWARD_TABLE: u32 = 0x0045_7870;

/// `float RiderMetrix_AwardPoints(RiderMetrix* self, int kind, int cash)`
/// (0x119ef8, the `evAddCash` event): unless `cash` is 0 or the game's
/// player count (`*(*(world+0x84)+0x28)`) is 1..9, credit `cash` to the
/// rider's character profile through the BE, show HUD number 0x18, and then
/// by `kind` (0..4) show the cash-source HUD number (0x31 / 0x2e / 0x2f /
/// 0x30 or none).
pub fn rider_metrix_award_points(c: &mut Cpu) {
    // A resume entry inside the generated frame: its dispatcher continues it
    // (at install this becomes `f_00119ef8__generated`).
    if c.pc != 0x0011_9ef8 {
        return ee::s00::f_00119ef8__generated(c);
    }
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        'done: {
            if c.g[S1][0] == 0 {
                break 'done;
            }
            'players: {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[A0][0] = c.lw(c.g[A0][0] as u32);
                c.g[A1][0] = 0;
                if c.g[A0][0] == 0 {
                    break 'players;
                }
                c.cycles += 1;
                c.g[A1][0] = ((c.g[A0][0] as i64) < 10) as u64;
            }
            c.cycles += 2;
            if c.g[A1][0] != 0 {
                break 'done;
            }
            c.cycles += 36;
            // be = BE_GetBE(); stats = BE_GetStats(be, 13)
            call_ptr(c, 0x0014_dc80, 0x0011_9f4c)?;
            c.g[A1][0] = 0xd;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_dd58, 0x0011_9f58)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1ac));
            c.g[S0][0] = c.g[V0][0];
            // id = self->rider->vcall_38()
            oc439_award_rider_vcall(c, 0x0011_9f78)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x001e_3760, 0x0011_9f84)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1ac));
            oc439_award_rider_vcall(c, 0x0011_9fa0)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x0015_97b0, 0x0011_9fb0)?;
            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x79fc));
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0x18;
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = 0;
            OC439_SHOW_HUD_NUMBER.call(c, 0x0011_9fc8)?;
            c.g[V1][0] = (c.g[S3][0] < 5) as u64;
            c.g[V0][0] = 0x45_0000;
            if c.g[V1][0] == 0 {
                break 'done;
            }
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[S3][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7870));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
            let case = c.g[A0][0] as u32;
            // The compiled `switch`: the table's entries are code addresses.
            match case {
                0x0011_9fec => {
                    c.cycles += 9;
                    c.g[A2][0] = c.g[S1][0];
                    c.g[AT][0] = 0x3fc0_0000;
                    c.mtc1(1, 12);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = 0x31;
                    c.g[A3][0] = c.g[A2][0];
                    OC439_SHOW_HUD_NUMBER.call(c, 0x0011_a008)?;
                }
                0x0011_a010 | 0x0011_a028 => {
                    if case == 0x0011_a010 {
                        c.cycles += 6;
                        c.g[A1][0] = 0x2e;
                    } else {
                        c.cycles += 5;
                        c.g[A1][0] = 0x2f;
                    }
                    c.g[AT][0] = 0x4020_0000;
                    c.mtc1(1, 12);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.cycles += 4;
                    c.g[A3][0] = 0;
                    OC439_SHOW_HUD_NUMBER.call(c, 0x0011_a044)?;
                }
                0x0011_a04c => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[AT][0] = 0x4020_0000;
                    c.mtc1(1, 12);
                    c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x9c));
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A1][0] = 0x30;
                    OC439_SHOW_HUD_NUMBER.call(c, 0x0011_a068)?;
                }
                0x0011_a068 => {}
                _ => {
                    c.jump_out(case, 0x0011_9ef8, 0x0011_a088);
                    return Ok(());
                }
            }
        }
        c.cycles += 8;
        c.mtc1(0, 0);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `a0 += 0x6c0` object's virtual call (slot +0x38): `a0 = rider->sub + delta`,
/// `v0 = (*sub->vtable[+0x3c])(a0)`.
fn oc439_award_rider_vcall(c: &mut Cpu, ret: u32) -> Flow {
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    call_ptr(c, target, ret)
}

// -- RiderTuning_FillTable4a6310 --------------------------------------------------------

/// The float table `RiderTuning_FillTable4a6310` fills (0x4a6310).
#[cfg(test)]
pub(crate) const OC439_TUNING_TABLE: u32 = 0x004a_6310;

/// `void RiderTuning_FillTable4a6310()` (0x11b2c0): fill the 2 x 3 x 4 x 2
/// float table at 0x4a6310 (row stride 0x60 bytes, then 0x20, 8 and 4) with
/// `120.0 - score`, where the integer score is a sum of per-axis weights: 5
/// for the first outer index (0 for the second), 0x14 / 0xb / 0 for the
/// middle one (0 and 1 / 2), 0x14 / 0xf / 0xa / 0 for the inner one and 5
/// for the first innermost element. All loops keep the original's registers
/// and event checks.
pub fn rider_tuning_fill_table_4a6310(c: &mut Cpu) {
    c.cycles += 18;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x50));
    c.g[V0][0] = 0x4a_0000;
    c.sq(16, sp(c).wrapping_add(0x40));
    c.g[V1][0] = 0;
    c.sq(17, sp(c).wrapping_add(0x30));
    c.g[T8][0] = 5;
    c.sq(18, sp(c).wrapping_add(0x20));
    c.g[S1][0] = 1;
    c.sq(19, sp(c).wrapping_add(0x10));
    c.g[S2][0] = 0xb;
    c.sq(20, sp(c));
    c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6310));
    c.g[AT][0] = 0x42f0_0000;
    c.mtc1(1, 1);
    c.g[S0][0] = 0xa;
    c.g[S4][0] = 0x60;
    c.mult(0, 15, 3, 20);
    'outer: loop {
        c.cycles += 6;
        c.g[T3][0] = c.g[T8][0];
        if c.g[V1][0] != 0 {
            c.g[T3][0] = 0;
        }
        c.g[T0][0] = 0;
        c.g[T9][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.g[A2][0] = 0;
        'middle: loop {
            c.cycles += 4;
            c.g[T6][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
            c.g[T4][0] = c.g[T0][0] ^ 1;
            c.g[T5][0] = sx((c.g[T0][0] as u32) << 5);
            c.g[V0][0] = sx((c.g[A2][0] as u32) << 3);
            'row: loop {
                c.cycles += 6;
                c.g[A3][0] = 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T5][0] as u32));
                c.g[T2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T7][0] as u32));
                c.g[T1][0] = c.g[A2][0] ^ 2;
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                'cell: loop {
                    // Middle weight: 0x14 for index 0, 0xb for 1, 0 for 2.
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S2][0];
                    if c.g[T0][0] == 0 {
                        c.cycles += 1;
                        c.g[V0][0] = 0x14;
                    } else {
                        c.cycles += 2;
                        if c.g[T4][0] != 0 {
                            c.g[V0][0] = 0;
                        }
                    }
                    // Inner weight: 0x14 / 0xf / 0xa / 0 for 0 / 1 / 2 / 3.
                    c.cycles += 2;
                    c.g[A0][0] = 0x14;
                    if c.g[A2][0] != 0 {
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S0][0];
                        if c.g[A2][0] == c.g[S1][0] {
                            c.cycles += 1;
                            c.g[A0][0] = 0xf;
                        } else {
                            c.cycles += 2;
                            if c.g[T1][0] != 0 {
                                c.g[A0][0] = 0;
                            }
                        }
                    }
                    c.cycles += 13;
                    c.g[V1][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.g[T8][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    if c.g[A3][0] != 0 {
                        c.g[V0][0] = 0;
                    }
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                    c.mtc1(3, 0);
                    c.cvt_s_w(0, 0);
                    c.g[V0][0] = ((c.g[A3][0] as i64) < 2) as u64;
                    c.sub_s(0, 1, 0);
                    c.swc1(0, c.g[A1][0] as u32);
                    let more_cells = c.g[V0][0] != 0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
                    if more_cells {
                        if event_due(c) {
                            deliver_event_here(c, 0x0011_b348);
                        }
                        continue 'cell;
                    }
                    break 'cell;
                }
                c.cycles += 4;
                c.g[A2][0] = c.g[T2][0];
                c.g[V0][0] = ((c.g[A2][0] as i64) < 4) as u64;
                let more_rows = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[A2][0] as u32) << 3);
                if more_rows {
                    if event_due(c) {
                        deliver_event_here(c, 0x0011_b330);
                    }
                    continue 'row;
                }
                break 'row;
            }
            c.cycles += 4;
            c.g[T0][0] = c.g[T6][0];
            c.g[V0][0] = ((c.g[T0][0] as i64) < 3) as u64;
            if c.g[V0][0] != 0 {
                c.g[A2][0] = 0;
                if event_due(c) {
                    deliver_event_here(c, 0x0011_b320);
                }
                continue 'middle;
            }
            break 'middle;
        }
        c.cycles += 4;
        c.g[V1][0] = c.g[T9][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        if c.g[V0][0] != 0 {
            c.mult(0, 15, 3, 20);
            if event_due(c) {
                deliver_event_here(c, 0x0011_b308);
            }
            continue 'outer;
        }
        break 'outer;
    }
    c.cycles += 7;
    c.lq(16, sp(c).wrapping_add(0x40));
    c.lq(17, sp(c).wrapping_add(0x30));
    c.lq(18, sp(c).wrapping_add(0x20));
    c.lq(19, sp(c).wrapping_add(0x10));
    c.lq(20, sp(c));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
    ret(c);
}

// -- Rider_dtor ------------------------------------------------------------------------------

/// `RiderMetrix_dtor(metrix, flags)` (0x117290).
pub(crate) const OC439_RIDER_METRIX_DTOR: Callee = Callee {
    addr: 0x0011_7290,
    f: ee::s00::RiderMetrix_dtor,
};

/// `Rider_dtorFree(object, flags)` (0x113170).
pub(crate) const OC439_RIDER_DTOR_FREE: Callee = Callee {
    addr: 0x0011_3170,
    f: ee::s00::Rider_dtorFree,
};

/// `FocusMan_RemoveFocusBox(box)` (0x1033f8).
pub(crate) const OC439_REMOVE_FOCUS_BOX: Callee = Callee {
    addr: 0x0010_33f8,
    f: ee::s00::FocusMan_RemoveFocusBox,
};

/// `operator delete` (0x317e50).
const OC439_DELETE: u32 = 0x0031_7e50;

/// `a1 = 3` virtual-destructor call `(*obj->vtable[slot])(obj + delta, 3)`
/// of a sub-object: `v0 = lw(obj + vt_off)` is its vtable, slot at +8 / +0xc.
fn oc439_dtor_vcall(c: &mut Cpu, vt_off: u32, ret: u32) -> Flow {
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(vt_off));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `void Rider_dtor(Rider* self, int flags)` (0x11b978): reset the rider's
/// vtables to the base class's, destroy its parts (anim, sound handles, air
/// predictor, focus box, metrics, controllers, sound emitters ...) and, when
/// `flags & 1`, delete the rider itself.
pub fn rider_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x46_0000;
        frame.save(c, S0, 0x30);
        c.g[V1][0] = 0x46_0000;
        c.g[A2][0] = 0x46_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x63e8));
        frame.save(c, S2, 0x10);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6470));
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6430));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6c0), c.g[A2][0]);
        c.g[S2][0] = c.g[A1][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6d0), c.g[V1][0]);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6dc));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6e8), c.g[V0][0]);
        call_ptr(c, 0x0028_b180, 0x0011_b9c0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0028_9af8, 0x0011_b9cc)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0028_32d8, 0x0011_b9d4)?;
        // The anim object (+0x784): virtual destructor (slot +0x58).
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x784));
        c.g[A1][0] = 3;
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            oc439_dtor_vcall(c, 0x58, 0x0011_b9f4)?;
        }
        // The script/sound objects (+0x780, +0xaa0).
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x780));
        if c.g[A0][0] == 0 {
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xaa0));
        } else {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0030_d540, 0x0011_ba08)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xaa0));
        }
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0032_9970, 0x0011_ba1c)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xac0));
        call_ptr(c, OC439_DELETE, 0x0011_ba24)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xabc));
        call_ptr(c, OC439_DELETE, 0x0011_ba2c)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x788));
        if c.g[A0][0] == 0 {
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78c));
        } else {
            c.cycles += 3;
            c.g[A1][0] = 3;
            OC439_RIDER_DTOR_FREE.call(c, 0x0011_ba40)?;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78c));
        }
        // The focus box (+0x78c): remove it from the focus manager.
        c.cycles += 2;
        let no_focus_box = c.g[A1][0] == 0;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        if !no_focus_box {
            c.cycles += 4;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xa4));
            OC439_REMOVE_FOCUS_BOX.call(c, 0x0011_ba5c)?;
        }
        c.cycles += 5;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78c));
        call_ptr(c, OC439_DELETE, 0x0011_ba64)?;
        // The metrics (+0x790).
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x790));
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = 3;
            OC439_RIDER_METRIX_DTOR.call(c, 0x0011_ba78)?;
        }
        c.cycles += 9;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x860));
        call_ptr(c, OC439_DELETE, 0x0011_ba80)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x868));
        call_ptr(c, OC439_DELETE, 0x0011_ba88)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x864));
        call_ptr(c, OC439_DELETE, 0x0011_ba90)?;
        // The controller objects (+0x890, +0x88c): virtual destructors.
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x890));
        c.g[A1][0] = 3;
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            oc439_dtor_vcall(c, 0xc4, 0x0011_bab0)?;
        }
        c.cycles += 3;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88c));
        c.g[A1][0] = 3;
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            oc439_dtor_vcall(c, 0xa4, 0x0011_bad0)?;
        }
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xaa8));
        c.g[V0][0] = 0x48_0000;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0037_5918, 0x0011_bae4)?;
            c.g[V0][0] = 0x48_0000;
        }
        // Back to the base class's vtables, destroy the part at +0x6dc.
        c.cycles += 13;
        c.g[V1][0] = 0x48_0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2018));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1fd8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6e8), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6d0), c.g[V1][0]);
        c.g[A1][0] = 0;
        call_ptr(c, 0x0028_3228, 0x0011_bb08)?;
        c.g[V0][0] = 0x46_0000;
        c.g[V1][0] = c.g[S2][0] & 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6338));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6c0), c.g[V0][0]);
        if c.g[V1][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, OC439_DELETE, 0x0011_bb24)?;
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

// -- RiderBase_LoadState -------------------------------------------------------------------------

/// The save-state stream's virtual `read(buf, len)` call every field read
/// makes: `a0 = stream + delta` and `v0 = *(vtable + 0x14)` of the stream
/// `s0` (a1/a2/a3 set by the caller).
fn oc439_stream_read(c: &mut Cpu, vtable: usize, ret: u32) -> Flow {
    c.g[A0][0] = c.lh((c.g[vtable][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = c.lw((c.g[vtable][0] as u32).wrapping_add(0x14));
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `RiderAnim_LoadState(anim, stream)` (0x104de0).
pub(crate) const OC439_RIDER_ANIM_LOAD_STATE: Callee = Callee {
    addr: 0x0010_4de0,
    f: ee::s00::RiderAnim_LoadState,
};

/// `AirPredictor_LoadState(predictor, stream)` (0x113ad8).
pub(crate) const OC439_AIR_PREDICTOR_LOAD_STATE: Callee = Callee {
    addr: 0x0011_3ad8,
    f: ee::s00::AirPredictor_LoadState,
};

/// `RiderMetrix_LoadState(metrix, stream)` (0x1174e8).
pub(crate) const OC439_RIDER_METRIX_LOAD_STATE: Callee = Callee {
    addr: 0x0011_74e8,
    f: ee::s00::RiderMetrix_LoadState,
};

/// `Rider_UpdateSphereTree(rider)` (0x106828).
pub(crate) const OC439_UPDATE_SPHERE_TREE: Callee = Callee {
    addr: 0x0010_6828,
    f: ee::s00::Rider_UpdateSphereTree,
};

/// The sub-state load calls with the stream in `a1`: (offset in the rider
/// base, callee, return address).
pub(crate) const OC439_LOAD_STATE_SUBS_A: [(u32, u32, u32); 14] = [
    (0x20, 0x0013_ace8, 0x0011_1fb4),
    (0x30, 0x0013_8b10, 0x0011_1fc0),
    (0xb0, 0x0013_c618, 0x0011_1fcc),
    (0x110, 0x0013_99a8, 0x0011_1fd8),
    (0x1d0, 0x0013_15d0, 0x0011_1fe4),
    (0x210, 0x0012_fc28, 0x0011_1ff0),
    (0x230, 0x0013_6230, 0x0011_1ffc),
    (0x290, 0x0012_c590, 0x0011_2008),
    (0x2b0, 0x0013_2878, 0x0011_2014),
    (0x2c0, 0x0012_e740, 0x0011_2020),
    (0x350, 0x0012_f5e8, 0x0011_202c),
    (0x360, 0x0012_c9f8, 0x0011_2038),
    (0x370, 0x0013_30f0, 0x0011_2044),
    (0x390, 0x0013_6920, 0x0011_2050),
];

/// The FX objects' load calls (no stream argument is set up; they ignore it):
/// (offset, callee, return address).
pub(crate) const OC439_LOAD_STATE_SUBS_B: [(u32, u32, u32); 9] = [
    (0x3b0, 0x002d_cf28, 0x0011_2058),
    (0x470, 0x002d_aa78, 0x0011_2060),
    (0x520, 0x002e_8560, 0x0011_2068),
    (0x610, 0x002e_6640, 0x0011_2070),
    (0x9c0, 0x002e_adc0, 0x0011_2078),
    (0xad0, 0x002e_f6a0, 0x0011_2080),
    (0xaf0, 0x002d_4be0, 0x0011_2088),
    (0xb00, 0x002e_3930, 0x0011_2090),
    (0xc70, 0x002f_1148, 0x0011_2098),
];

/// The resource lookup after a rider-base id read: `v0` holds `(id & 0xff) <<
/// 2`, `a1` the address of the resource table global and `a2` the id
/// (`index | slot << 8`). Walks `table[index]->+0xc[slot]` and leaves the
/// resource pointer (`word >> 8 << 2`, or 0 for an empty slot or entry) in
/// `a0`, the scratch registers as the original leaves them.
fn oc439_lookup_resource(c: &mut Cpu) {
    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
    c.g[A0][0] = 0;
    if c.g[V1][0] != 0 {
        c.cycles += 8;
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        let nonzero = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if !nonzero {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 1;
        c.g[A0][0] = c.g[V0][0];
    }
}

/// `void RiderBase_LoadState(RiderBase* self, Stream* s)` (0x111d98): restore
/// the rider from a save state: the 0x6c0-byte base block, then each part's
/// own `LoadState` (anim, script, air predictor, metrics), two resource ids
/// (looked up and stored at +0x864 / +0x868 of the part at +0x18), the
/// sub-state objects, the FX objects, three words and two pointer fixups,
/// and finally the sphere tree.
pub fn rider_base_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 51;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0x6c0;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        c.g[S1][0] = c.g[A0][0];
        // stream->read(self->part, 0x6c0)
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        oc439_stream_read(c, V1, 0x0011_1dcc)?;
        // The parts' LoadState calls: (part offset in self->part, callee).
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x784));
        OC439_RIDER_ANIM_LOAD_STATE.call(c, 0x0011_1ddc)?;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x780));
        call_ptr(c, 0x0031_0ee0, 0x0011_1dec)?;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x788));
        OC439_AIR_PREDICTOR_LOAD_STATE.call(c, 0x0011_1dfc)?;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x790));
        OC439_RIDER_METRIX_LOAD_STATE.call(c, 0x0011_1e0c)?;
        // First resource id (-1 = none): read into sp+0x10, look it up in the
        // resource table at gp+0x16c8 and store the pointer at part+0x864.
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] = sx(sp(c).wrapping_add(0x10));
        c.sw(sp(c).wrapping_add(0x10), c.g[V0][0]);
        c.g[A2][0] = 4;
        oc439_stream_read(c, V1, 0x0011_1e34)?;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A2][0] = 0x10;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x864));
        c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let target = c.g[A3][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0011_1e54)?;
        c.g[A2][0] = c.lw(sp(c).wrapping_add(0x10));
        c.g[V0][0] = !c.g[A2][0];
        let none = c.g[V0][0] == 0;
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
        if !none {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            oc439_lookup_resource(c);
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x864));
            c.sw(c.g[V1][0] as u32, c.g[A0][0]);
        }
        // Second resource id: sp+0x14, stored at part+0x868.
        c.cycles += 22;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] = sx(sp(c).wrapping_add(0x14));
        c.sw(sp(c).wrapping_add(0x14), c.g[V0][0]);
        c.g[A2][0] = 4;
        oc439_stream_read(c, V1, 0x0011_1ee0)?;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A2][0] = 0x10;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x868));
        c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let target = c.g[A3][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0011_1f00)?;
        c.g[A2][0] = c.lw(sp(c).wrapping_add(0x14));
        c.g[V0][0] = !c.g[A2][0];
        let none = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[A2][0] & 0xff;
        if !none {
            c.cycles += 8;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            oc439_lookup_resource(c);
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x868));
            c.sw(c.g[V1][0] as u32, c.g[A0][0]);
        }
        // Two words straight into the base block's tail (+0xde0, +0xde4).
        c.cycles += 119;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xde0));
        c.g[A2][0] = 4;
        oc439_stream_read(c, V1, 0x0011_1f80)?;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A2][0] = 4;
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xde4));
        oc439_stream_read(c, V1, 0x0011_1f9c)?;
        // The sub-state objects.
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0013_f880, 0x0011_1fa8)?;
        for &(off, target, ret) in &OC439_LOAD_STATE_SUBS_A {
            c.g[A1][0] = c.g[S0][0];
            oc439_call_sub(c, S1, off, target, ret)?;
        }
        // The FX objects.
        for &(off, target, ret) in &OC439_LOAD_STATE_SUBS_B {
            oc439_call_sub(c, S1, off, target, ret)?;
        }
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xd20));
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x002f_6a90, 0x0011_20a4)?;
        // Two more resource handles (+0xabc, +0xac0 of the part), 0x20 bytes each.
        c.g[A3][0] = c.lw(c.g[S0][0] as u32);
        c.g[A2][0] = 0x20;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[A0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xabc));
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x14));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0011_20c4)?;
        c.g[A3][0] = c.lw(c.g[S0][0] as u32);
        c.g[A2][0] = 0x20;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[A0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x14));
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xac0));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0011_20e4)?;
        // Two words into sp+0 / sp+4 (the string pool ids).
        c.g[A3][0] = c.lw(c.g[S0][0] as u32);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 8;
        c.g[A0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x14));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0011_2100)?;
        // Two pointer fixups: string pool lookups of the ids just read.
        c.g[S0][0] = 0x4d_0000;
        c.g[A1][0] = c.lw(sp(c));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x33a0));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0026_af70, 0x0011_2114)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[A0][0] = c.g[S0][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0xab4), c.g[V0][0]);
        c.g[A1][0] = c.lw(sp(c).wrapping_add(4));
        call_ptr(c, 0x0026_af88, 0x0011_2128)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.sw((c.g[V1][0] as u32).wrapping_add(0xab8), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78c));
        c.g[A1][0] = 0;
        if c.g[V1][0] != 0 {
            // Copy the 16 bytes at part+0x110 into the focus box (+0x78c).
            c.cycles += 3;
            c.lq(2, (c.g[V0][0] as u32).wrapping_add(0x110));
            c.sq(2, c.g[V1][0] as u32);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        }
        c.cycles += 12;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
        call_ptr(c, 0x0031_2598, 0x0011_2154)?;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x780));
        call_ptr(c, 0x0031_03f0, 0x0011_2160)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        OC439_UPDATE_SPHERE_TREE.call(c, 0x0011_2168)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, FAKE};
    use crate::difftest::{check, fill, put_words, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate of this module's callee fakes (other tests' machines fall back).
    pub(crate) const OC439_CTRL: u32 = DATA + 0xf_e000;

    pub(crate) const OC439_MAGIC: u32 = 0x0c43_9a11;

    /// Scripted value a "scripted" fake method returns in v0.
    pub(crate) const OC439_SCRIPTED: u32 = OC439_CTRL + 0x40;

    /// The stream-read fake's scripted words (id, id, word, word) and its counter.
    pub(crate) const OC439_READS: u32 = OC439_CTRL + 0x20;

    pub(crate) const OC439_READ_COUNT: u32 = OC439_CTRL + 0x30;

    /// Fake method addresses (below the first translated function).
    pub(crate) const OC439_FAKE_PLAIN: u32 = FAKE;

    pub(crate) const OC439_FAKE_SCRIPTED: u32 = FAKE + 0x10;

    pub(crate) const OC439_FAKE_READ: u32 = FAKE + 0x20;

    /// Gated fakes in the override slots of callees that already have a hand
    /// port (each logs its call like `difftest::fakes::fake`; the gate is off
    /// in every other test of the binary, where the slot runs what it held).
    macro_rules! oc439_gated {
        ($($fake:ident => $addr:literal, $slot:path, $generated:path;)*) => {
            $(
                pub(crate) fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC439_CTRL) != OC439_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $generated);
                    }
                    let ra = c.g[RA][0] as u32;
                    fakes::fake(c, $addr, ra);
                }
            )*

            pub(crate) fn oc439_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake)); )*
            }
        };
    }

    oc439_gated! {
        oc439_fake_00392d18 => 0x0039_2d18, ee::s13::OVERRIDE_00392D18, ee::s13::f_00392d18__generated;
        oc439_fake_002dd0b8 => 0x002d_d0b8, ee::s09::OVERRIDE_002DD0B8, ee::s09::f_002dd0b8__generated;
        oc439_fake_002dabc8 => 0x002d_abc8, ee::s09::OVERRIDE_002DABC8, ee::s09::f_002dabc8__generated;
        oc439_fake_002e8938 => 0x002e_8938, ee::s09::OVERRIDE_002E8938, ee::s09::f_002e8938__generated;
        oc439_fake_002e66b8 => 0x002e_66b8, ee::s09::OVERRIDE_002E66B8, ee::s09::f_002e66b8__generated;
        oc439_fake_002eadd0 => 0x002e_add0, ee::s09::OVERRIDE_002EADD0, ee::s09::f_002eadd0__generated;
        oc439_fake_002ef6d0 => 0x002e_f6d0, ee::s09::OVERRIDE_002EF6D0, ee::s09::f_002ef6d0__generated;
        oc439_fake_002d4c08 => 0x002d_4c08, ee::s09::OVERRIDE_002D4C08, ee::s09::f_002d4c08__generated;
        oc439_fake_002e39d8 => 0x002e_39d8, ee::s09::OVERRIDE_002E39D8, ee::s09::f_002e39d8__generated;
        oc439_fake_002df920 => 0x002d_f920, ee::s09::OVERRIDE_002DF920, ee::s09::f_002df920__generated;
        oc439_fake_002f1150 => 0x002f_1150, ee::s09::OVERRIDE_002F1150, ee::s09::f_002f1150__generated;
        oc439_fake_002f6518 => 0x002f_6518, ee::s09::OVERRIDE_002F6518, ee::s09::f_002f6518__generated;
        oc439_fake_002ddab8 => 0x002d_dab8, ee::s09::OVERRIDE_002DDAB8, ee::s09::f_002ddab8__generated;
        oc439_fake_002db478 => 0x002d_b478, ee::s09::OVERRIDE_002DB478, ee::s09::f_002db478__generated;
        oc439_fake_002ea480 => 0x002e_a480, ee::s09::OVERRIDE_002EA480, ee::s09::f_002ea480__generated;
        oc439_fake_002e7a10 => 0x002e_7a10, ee::s09::OVERRIDE_002E7A10, ee::s09::f_002e7a10__generated;
        oc439_fake_002eb198 => 0x002e_b198, ee::s09::OVERRIDE_002EB198, ee::s09::f_002eb198__generated;
        oc439_fake_002ef950 => 0x002e_f950, ee::s09::OVERRIDE_002EF950, ee::s09::f_002ef950__generated;
        oc439_fake_002d5048 => 0x002d_5048, ee::s09::OVERRIDE_002D5048, ee::s09::f_002d5048__generated;
        oc439_fake_002e3af8 => 0x002e_3af8, ee::s09::OVERRIDE_002E3AF8, ee::s09::f_002e3af8__generated;
        oc439_fake_002e24d0 => 0x002e_24d0, ee::s09::OVERRIDE_002E24D0, ee::s09::f_002e24d0__generated;
        oc439_fake_002f1510 => 0x002f_1510, ee::s09::OVERRIDE_002F1510, ee::s09::f_002f1510__generated;
        oc439_fake_002f6908 => 0x002f_6908, ee::s09::OVERRIDE_002F6908, ee::s09::f_002f6908__generated;
        oc439_fake_00104de0 => 0x0010_4de0, ee::s00::OVERRIDE_00104DE0, ee::s00::f_00104de0__generated;
        oc439_fake_00106828 => 0x0010_6828, ee::s00::OVERRIDE_00106828, ee::s00::f_00106828__generated;
        oc439_fake_00113ad8 => 0x0011_3ad8, ee::s00::OVERRIDE_00113AD8, ee::s00::f_00113ad8__generated;
        oc439_fake_001174e8 => 0x0011_74e8, ee::s00::OVERRIDE_001174E8, ee::s00::f_001174e8__generated;
        oc439_fake_0012c590 => 0x0012_c590, ee::s00::OVERRIDE_0012C590, ee::s00::f_0012c590__generated;
        oc439_fake_0012c9f8 => 0x0012_c9f8, ee::s00::OVERRIDE_0012C9F8, ee::s00::f_0012c9f8__generated;
        oc439_fake_0012e740 => 0x0012_e740, ee::s00::OVERRIDE_0012E740, ee::s00::f_0012e740__generated;
        oc439_fake_0012f5e8 => 0x0012_f5e8, ee::s00::OVERRIDE_0012F5E8, ee::s00::f_0012f5e8__generated;
        oc439_fake_0012fc28 => 0x0012_fc28, ee::s00::OVERRIDE_0012FC28, ee::s00::f_0012fc28__generated;
        oc439_fake_001315d0 => 0x0013_15d0, ee::s00::OVERRIDE_001315D0, ee::s00::f_001315d0__generated;
        oc439_fake_00132878 => 0x0013_2878, ee::s01::OVERRIDE_00132878, ee::s01::f_00132878__generated;
        oc439_fake_001330f0 => 0x0013_30f0, ee::s01::OVERRIDE_001330F0, ee::s01::f_001330f0__generated;
        oc439_fake_00136230 => 0x0013_6230, ee::s01::OVERRIDE_00136230, ee::s01::f_00136230__generated;
        oc439_fake_00136920 => 0x0013_6920, ee::s01::OVERRIDE_00136920, ee::s01::f_00136920__generated;
        oc439_fake_00138b10 => 0x0013_8b10, ee::s01::OVERRIDE_00138B10, ee::s01::f_00138b10__generated;
        oc439_fake_001399a8 => 0x0013_99a8, ee::s01::OVERRIDE_001399A8, ee::s01::f_001399a8__generated;
        oc439_fake_0013ace8 => 0x0013_ace8, ee::s01::OVERRIDE_0013ACE8, ee::s01::f_0013ace8__generated;
        oc439_fake_0013c618 => 0x0013_c618, ee::s01::OVERRIDE_0013C618, ee::s01::f_0013c618__generated;
        oc439_fake_0013f880 => 0x0013_f880, ee::s01::OVERRIDE_0013F880, ee::s01::f_0013f880__generated;
        oc439_fake_0026af70 => 0x0026_af70, ee::s07::OVERRIDE_0026AF70, ee::s07::f_0026af70__generated;
        oc439_fake_0026af88 => 0x0026_af88, ee::s07::OVERRIDE_0026AF88, ee::s07::f_0026af88__generated;
        oc439_fake_002d4be0 => 0x002d_4be0, ee::s09::OVERRIDE_002D4BE0, ee::s09::f_002d4be0__generated;
        oc439_fake_002daa78 => 0x002d_aa78, ee::s09::OVERRIDE_002DAA78, ee::s09::f_002daa78__generated;
        oc439_fake_002e3930 => 0x002e_3930, ee::s09::OVERRIDE_002E3930, ee::s09::f_002e3930__generated;
        oc439_fake_002e6640 => 0x002e_6640, ee::s09::OVERRIDE_002E6640, ee::s09::f_002e6640__generated;
        oc439_fake_002e8560 => 0x002e_8560, ee::s09::OVERRIDE_002E8560, ee::s09::f_002e8560__generated;
        oc439_fake_002eadc0 => 0x002e_adc0, ee::s09::OVERRIDE_002EADC0, ee::s09::f_002eadc0__generated;
        oc439_fake_002ef6a0 => 0x002e_f6a0, ee::s09::OVERRIDE_002EF6A0, ee::s09::f_002ef6a0__generated;
        oc439_fake_002f6a90 => 0x002f_6a90, ee::s09::OVERRIDE_002F6A90, ee::s09::f_002f6a90__generated;
        oc439_fake_003103f0 => 0x0031_03f0, ee::s10::OVERRIDE_003103F0, ee::s10::f_003103f0__generated;
        oc439_fake_00310ee0 => 0x0031_0ee0, ee::s10::OVERRIDE_00310EE0, ee::s10::f_00310ee0__generated;
        oc439_fake_00312598 => 0x0031_2598, ee::s10::OVERRIDE_00312598, ee::s10::f_00312598__generated;
        oc439_fake_001033f8 => 0x0010_33f8, ee::s00::OVERRIDE_001033F8, ee::s00::f_001033f8__generated;
        oc439_fake_00113170 => 0x0011_3170, ee::s00::OVERRIDE_00113170, ee::s00::f_00113170__generated;
        oc439_fake_00283228 => 0x0028_3228, ee::s07::OVERRIDE_00283228, ee::s07::f_00283228__generated;
        oc439_fake_002832d8 => 0x0028_32d8, ee::s07::OVERRIDE_002832D8, ee::s07::f_002832d8__generated;
        oc439_fake_0028b180 => 0x0028_b180, ee::s07::OVERRIDE_0028B180, ee::s07::f_0028b180__generated;
        oc439_fake_0030d540 => 0x0030_d540, ee::s10::OVERRIDE_0030D540, ee::s10::f_0030d540__generated;
        oc439_fake_00317e50 => 0x0031_7e50, ee::s10::OVERRIDE_00317E50, ee::s10::f_00317e50__generated;
        oc439_fake_00329970 => 0x0032_9970, ee::s10::OVERRIDE_00329970, ee::s10::f_00329970__generated;
        oc439_fake_00375918 => 0x0037_5918, ee::s12::OVERRIDE_00375918, ee::s12::f_00375918__generated;
        oc439_fake_00292ae8 => 0x0029_2ae8, ee::s07::OVERRIDE_00292AE8, ee::s07::f_00292ae8__generated;
        oc439_fake_00317e98 => 0x0031_7e98, ee::s10::OVERRIDE_00317E98, ee::s10::f_00317e98__generated;
        oc439_fake_0014dc80 => 0x0014_dc80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated;
        oc439_fake_0014dd58 => 0x0014_dd58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated;
        oc439_fake_00117b88 => 0x0011_7b88, ee::s00::OVERRIDE_00117B88, ee::s00::f_00117b88__generated;
        oc439_fake_00117ae8 => 0x0011_7ae8, ee::s00::OVERRIDE_00117AE8, ee::s00::f_00117ae8__generated;
        oc439_fake_0011a228 => 0x0011_a228, ee::s00::OVERRIDE_0011A228, ee::s00::f_0011a228__generated;
        oc439_fake_00117838 => 0x0011_7838, ee::s00::OVERRIDE_00117838, ee::s00::f_00117838__generated;
        oc439_fake_00111860 => 0x0011_1860, ee::s00::OVERRIDE_00111860, ee::s00::f_00111860__generated;
        oc439_fake_0014a0e0 => 0x0014_a0e0, ee::s01::OVERRIDE_0014A0E0, ee::s01::f_0014a0e0__generated;
        oc439_fake_0014a080 => 0x0014_a080, ee::s01::OVERRIDE_0014A080, ee::s01::f_0014a080__generated;
        oc439_fake_00146008 => 0x0014_6008, ee::s01::OVERRIDE_00146008, ee::s01::f_00146008__generated;
        oc439_fake_001509f8 => 0x0015_09f8, ee::s01::OVERRIDE_001509F8, ee::s01::f_001509f8__generated;
        oc439_fake_001e3a30 => 0x001e_3a30, ee::s04::OVERRIDE_001E3A30, ee::s04::f_001e3a30__generated;
        oc439_fake_001e3a78 => 0x001e_3a78, ee::s04::OVERRIDE_001E3A78, ee::s04::f_001e3a78__generated;
    }

    /// Calls into the fake method area: logged like every fake; the scripted
    /// method answers with the fixture's word, the stream-read method fills
    /// 4-byte buffers from the fixture's script.
    pub(crate) fn oc439_miss(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let (buf, len) = (c.g[A1][0] as u32, c.g[A2][0] as u32);
        fakes::fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        if target == OC439_FAKE_SCRIPTED {
            let v = c.bus.read32(OC439_SCRIPTED);
            c.g[V0][0] = sx(v);
        } else if target == OC439_FAKE_READ && len == 4 {
            let n = c.bus.read32(OC439_READ_COUNT);
            c.bus.write32(OC439_READ_COUNT, n + 1);
            let v = c.bus.read32(OC439_READS + 4 * (n & 3));
            c.bus.write32(buf, v);
        }
    }

    /// A scrambled machine with the fakes on, gp set, `pc` at the entry.
    pub(crate) fn oc439_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        oc439_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC439_CTRL, OC439_MAGIC);
        c.hooks.miss = oc439_miss;
        c.pc = entry;
        r
    }

    /// An object at `addr` whose vtable record (at `vt`) has delta `delta` at
    /// +8 and the fake `method` at +0xc; `vt_slot` is where the object keeps
    /// the vtable pointer.
    pub(crate) fn oc439_vobject(
        c: &mut Cpu,
        addr: u32,
        vt_slot: u32,
        vt: u32,
        vt_off: u32,
        method: u32,
        delta: u32,
    ) {
        c.bus.write32(addr + vt_slot, vt);
        c.bus.write16(vt + vt_off, delta as u16);
        c.bus.write32(vt + vt_off + 4, method);
    }

    // -- trivial leaves ----------------------------------------------------------

    /// `ScriptEngine_ClearSkipDelayOnEnter`: clears +0x554 whatever it held.
    #[test]
    fn script_engine_clear_skip_delay_on_enter_matches() {
        check(
            ee::s07::f_00278710__generated,
            script_engine_clear_skip_delay_on_enter,
            40,
            &|c, seed| {
                let mut r = Rng(0x2787_1000 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0027_8710;
                let this = DATA + 0x100 * (seed % 8);
                c.g[A0][0] = sx(this);
                fill(c, &mut r, this + 0x550, 4);
            },
        );
    }

    /// `ScriptList_GetFieldB0`: loads the +0xb0 word sign-extended.
    #[test]
    fn script_list_get_field_b0_matches() {
        check(
            ee::s07::f_002772b8__generated,
            script_list_get_field_b0,
            40,
            &|c, seed| {
                let mut r = Rng(0x2772_b800 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0027_72b8;
                let this = DATA + 0x100 * (seed % 8);
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0xb0, r.next_u32());
            },
        );
    }

    /// `CameraTriggerMan_EmptyHook`: only the return.
    #[test]
    fn camera_trigger_man_empty_hook_matches() {
        check(
            ee::s02::f_0016ca10__generated,
            camera_trigger_man_empty_hook,
            10,
            &|c, seed| {
                let mut r = Rng(0x16ca_1000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0016_ca10;
            },
        );
    }

    /// `Rider_GetStartDistToFinish`: the float at +0x4d8 into f0.
    #[test]
    fn rider_get_start_dist_to_finish_matches() {
        check(
            ee::s00::f_00113130__generated,
            rider_get_start_dist_to_finish,
            40,
            &|c, seed| {
                let mut r = Rng(0x1131_3000 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0011_3130;
                let this = DATA + 0x800 * (seed % 4);
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x4d8, r.edge_f32());
            },
        );
    }

    /// `RiderMetrix_StartUberGrind`: seven stores and f0 (a1/a2 random).
    #[test]
    fn rider_metrix_start_uber_grind_matches() {
        check(
            ee::s00::f_00119938__generated,
            rider_metrix_start_uber_grind,
            60,
            &|c, seed| {
                let mut r = Rng(0x1199_3800 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0011_9938;
                let this = DATA + 0x800 * (seed % 4);
                c.g[A0][0] = sx(this);
                fill(c, &mut r, this, 0x40);
            },
        );
    }

    // -- HUD wrappers ------------------------------------------------------------

    /// `RiderMetrix_ShowHUDMsg2A`: HUD number 0x2a with the argument as value
    /// (the HUD number call is faked; it unwinds on some seeds).
    #[test]
    fn rider_metrix_show_hud_msg_2a_matches() {
        covered(
            ee::s00::f_001195a8__generated,
            rider_metrix_show_hud_msg_2a,
            60,
            &|c, seed| {
                let mut r = oc439_machine(c, seed, 0x1195a8, 1, 0x0011_95a8);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    /// `RiderMetrix_ShowHUDMsg2B`: as 2A with number 0x2b.
    #[test]
    fn rider_metrix_show_hud_msg_2b_matches() {
        covered(
            ee::s00::f_001195d8__generated,
            rider_metrix_show_hud_msg_2b,
            60,
            &|c, seed| {
                let mut r = oc439_machine(c, seed, 0x1195d8, 1, 0x0011_95d8);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    /// `RiderMetrix_ShowHUDMsg1A`: HUD number 0x1a, value 0.
    #[test]
    fn rider_metrix_show_hud_msg_1a_matches() {
        covered(
            ee::s00::f_0011a0e0__generated,
            rider_metrix_show_hud_msg_1a,
            60,
            &|c, seed| {
                let _ = oc439_machine(c, seed, 0x11a0e0, 1, 0x0011_a0e0);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    /// `RiderMetrix_ShowWrongWay`: the HUD text call (faked, unwinding on
    /// some seeds).
    #[test]
    fn rider_metrix_show_wrong_way_matches() {
        covered(
            ee::s00::f_0011a088__generated,
            rider_metrix_show_wrong_way,
            60,
            &|c, seed| {
                let _ = oc439_machine(c, seed, 0x11a088, 1, 0x0011_a088);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    /// `Rider_ResetMissionMetrix`: the rider's metrics (+0x790) get the
    /// mission-start message. `Msg1A` runs for real over the faked HUD
    /// number call.
    #[test]
    fn rider_reset_mission_metrix_matches() {
        covered(
            ee::s00::f_0010f2b8__generated,
            rider_reset_mission_metrix,
            60,
            &|c, seed| {
                let _ = oc439_machine(c, seed, 0x10f2b8, 1, 0x0010_f2b8);
                let rider = DATA + 0x100;
                c.g[A0][0] = sx(rider);
                c.bus.write32(rider + 0x790, DATA + 0x400);
            },
        );
    }

    /// `RiderMetrix_FinishHandplant`: the handplant counter (+0x80, wrapping
    /// through 0x7fffffff), the finish and reset calls (faked; one of them
    /// unwinds on some seeds) and the +0x30 clear.
    #[test]
    fn rider_metrix_finish_handplant_matches() {
        covered(
            ee::s00::f_00119c38__generated,
            rider_metrix_finish_handplant,
            80,
            &|c, seed| {
                let mut r = oc439_machine(c, seed, 0x119c38, 2, 0x0011_9c38);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                let count = if seed % 5 == 0 {
                    0x7fff_ffff
                } else {
                    r.next_u32()
                };
                c.bus.write32(this + 0x80, count);
                c.bus.write32(this + 0x30, r.next_u32());
            },
        );
    }

    // -- Rider wrappers ------------------------------------------------------------

    /// `Rider_GetSubStateName`: the base class call with `+0x77c` (faked).
    #[test]
    fn rider_get_sub_state_name_matches() {
        covered(
            ee::s00::f_0011fef8__generated,
            rider_get_sub_state_name,
            60,
            &|c, seed| {
                let mut r = oc439_machine(c, seed, 0x11fef8, 1, 0x0011_fef8);
                c.g[A0][0] = sx(DATA + 0x100);
                c.bus.write32(DATA + 0x100 + 0x77c, r.next_u32());
            },
        );
    }

    /// `Rider_GetControlInput`: the virtual call on the control object
    /// (`rider->+0x77c->+0xde8`), through a fake method with a random delta.
    #[test]
    fn rider_get_control_input_matches() {
        covered(
            ee::s00::f_0011ff18__generated,
            rider_get_control_input,
            80,
            &|c, seed| {
                let mut r = oc439_machine(c, seed, 0x11ff18, 1, 0x0011_ff18);
                let (rider, sub, obj) = (DATA + 0x100, DATA + 0x1000, DATA + 0x2000);
                c.g[A0][0] = sx(rider);
                c.bus.write32(rider + 0x77c, sub);
                c.bus.write32(sub + 0xde8, obj);
                let delta = (r.next_u32() % 64) & !3;
                let neg = if seed % 3 == 0 {
                    0u32.wrapping_sub(delta)
                } else {
                    delta
                };
                c.bus.write16(obj + 8, neg as u16);
                c.bus.write32(obj + 0xc, OC439_FAKE_PLAIN);
            },
        );
    }

    // -- Rider FX passes -------------------------------------------------------------

    /// The FX passes' fixture: the FX switch (gp+0x1444) on or off, a rider
    /// whose virtual `skip?` method (`+0x6c0` object, slot +0x48) answers 0 or
    /// non-zero, and a base object `this` (the 12 FX calls are faked).
    fn oc439_fx_fixture(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        let mut r = oc439_machine(c, seed, salt, 13, entry);
        let (this, part, vt) = (DATA + 0x1000, DATA + 0x3000, DATA + 0x5000);
        c.g[A0][0] = sx(this);
        c.bus.write32(this + 0x18, part);
        c.bus.write32(part + 0x6c0, vt);
        let delta = (r.next_u32() % 64) & !3;
        c.bus.write16(vt + 0x48, delta as u16);
        c.bus.write32(vt + 0x4c, OC439_FAKE_SCRIPTED);
        c.bus.write32(
            OC439_SCRIPTED,
            if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
        );
        c.bus.write32(
            GAME_GP + 0x1444,
            if seed % 7 == 0 { 0 } else { r.next_u32() | 1 },
        );
    }

    /// `RiderBase_UpdateFX`: switch off, rider says skip, or the 12 FX calls.
    #[test]
    fn rider_base_update_fx_matches() {
        covered(
            ee::s00::f_00111948__generated,
            rider_base_update_fx,
            120,
            &|c, seed| oc439_fx_fixture(c, seed, 0x111948, 0x0011_1948),
        );
    }

    /// `RiderBase_RenderFX`: as the update pass with 11 calls.
    #[test]
    fn rider_base_render_fx_matches() {
        covered(
            ee::s00::f_001119f8__generated,
            rider_base_render_fx,
            120,
            &|c, seed| oc439_fx_fixture(c, seed, 0x1119f8, 0x0011_19f8),
        );
    }

    /// `RaceStateEndRace_Enter`: with and without the game-comm object; the
    /// real `0x258a48` runs on an object whose timers (+0x38/+0x3c) are set
    /// and whose state (+0x70) skips the state change.
    #[test]
    fn race_state_end_race_enter_matches() {
        check(
            ee::s00::f_00113e18__generated,
            race_state_end_race_enter,
            60,
            &|c, seed| {
                let mut r = Rng(0x1131_e180 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0011_3e18;
                c.g[GP][0] = GAME_GP as u64;
                let obj = DATA + 0x400;
                c.bus
                    .write32(GAME_GP - 0x204, if seed % 4 == 0 { 0 } else { obj });
                c.bus.write32(obj + 0x70, 15 + r.next_u32() % 4);
                c.bus.write32(obj + 0x38, 1 + r.next_u32() % 100);
                c.bus.write32(obj + 0x3c, 1 + r.next_u32() % 100);
                fill(c, &mut r, obj + 0x40, 2);
            },
        );
    }

    // -- RiderMetrix_AwardPoints -------------------------------------------------------

    /// `RiderMetrix_AwardPoints`: no cash, few players (1..9, skipped), many
    /// players or none; every award kind 0..6 (kinds 5 and 6 are past the
    /// switch table), a corrupt table entry (the jump-out), unwinds in the
    /// BE and rider calls; the unported `0x1e3760` / `0x1597b0` run for real
    /// over faked BE pointers.
    #[test]
    fn rider_metrix_award_points_matches() {
        covered(
            ee::s00::f_00119ef8__generated,
            rider_metrix_award_points,
            160,
            &|c, seed| {
                let mut r = oc439_machine(c, seed, 0x119ef8, 7, 0x0011_9ef8);
                let (this, rider, vt) = (DATA + 0x100, DATA + 0x400, DATA + 0x800);
                let (world, w84, players) = (DATA + 0x1000, DATA + 0x1100, DATA + 0x1200);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, w84);
                c.bus.write32(w84 + 0x28, players);
                let count = match seed % 4 {
                    0 => 0,
                    1 => 1 + r.next_u32() % 9,
                    _ => 10 + r.next_u32() % 5,
                };
                c.bus.write32(players, count);
                c.bus.write32(GAME_GP - 0x1e90, 0);
                c.bus.write32(this + 0x1ac, rider);
                c.bus.write32(rider + 0x6c0, vt);
                c.bus.write16(vt + 0x38, ((r.next_u32() % 16) * 4) as u16);
                c.bus.write32(vt + 0x3c, OC439_FAKE_PLAIN);
                c.bus.write32(this + 0x9c, r.next_u32());
                let kind = seed / 4 % 7;
                let mut table = [
                    0x0011_9fec,
                    0x0011_a010,
                    0x0011_a028,
                    0x0011_a04c,
                    0x0011_a068,
                ];
                if seed % 29 == 11 && kind < 5 {
                    table[kind as usize] = 0x0011_9ff0;
                }
                put_words(c, OC439_AWARD_TABLE, &table);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = kind as u64;
                c.g[A2][0] = if seed % 11 == 3 {
                    0
                } else {
                    (1 + r.next_u32() % 5000) as u64
                };
            },
        );
    }

    // -- RiderTuning_FillTable4a6310 -----------------------------------------------------

    /// `RiderTuning_FillTable4a6310`: pure table fill over scrambled registers
    /// and stale table contents.
    #[test]
    fn rider_tuning_fill_table_4a6310_matches() {
        check(
            ee::s00::f_0011b2c0__generated,
            rider_tuning_fill_table_4a6310,
            12,
            &|c, seed| {
                let mut r = Rng(0x11b2_c000 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0011_b2c0;
                fill(c, &mut r, OC439_TUNING_TABLE, 0x60);
            },
        );
    }

    // -- Rider_dtor -------------------------------------------------------------------------

    /// `Rider_dtor`: every optional part present or absent (sound objects,
    /// anim, air predictor, focus box, metrics, controllers, emitters), the
    /// `flags & 1` delete, unwinds in any of the ~25 calls. All callees are
    /// faked except `RiderMetrix_dtor` and `0x289af8` (real, over zeroed
    /// parts).
    #[test]
    fn rider_dtor_matches() {
        covered(ee::s00::f_0011b978__generated, rider_dtor, 160, &|c, seed| {
            let mut r = oc439_machine(c, seed, 0x11b978, 28, 0x0011_b978);
            let this = DATA + 0x1000;
            let vts = DATA + 0x6000;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = r.next_u32() as u64;
            // An optional object per field: absent when its bit is clear.
            let present = |bit: u32| (seed >> bit) & 1 == 1 || seed % 13 == 0;
            let obj = |slot: u32| DATA + 0x2000 + 0x100 * slot;
            for (slot, off) in [
                0x780u32, 0xaa0, 0xac0, 0xabc, 0x788, 0x860, 0x864, 0x868, 0xaa8,
            ]
            .into_iter()
            .enumerate()
            {
                let v = if present(slot as u32 % 6) {
                    obj(slot as u32)
                } else {
                    0
                };
                c.bus.write32(this + off, v);
            }
            // Objects destroyed through a virtual destructor: (field, vtable slot, bit).
            for (n, (field, vt_off, bit)) in [
                (0x784u32, 0x58u32, 0u32),
                (0x890, 0xc4, 1),
                (0x88c, 0xa4, 2),
            ]
            .into_iter()
            .enumerate()
            {
                let o = DATA + 0x3000 + 0x200 * n as u32;
                if present(bit) {
                    c.bus.write32(this + field, o);
                    oc439_vobject(
                        c,
                        o,
                        vt_off,
                        vts + 0x40 * n as u32,
                        8,
                        OC439_FAKE_PLAIN,
                        (r.next_u32() % 32) & !3,
                    );
                } else {
                    c.bus.write32(this + field, 0);
                }
            }
            // The focus box (+0x78c): the focus manager is found through the world.
            let focus = if present(3) { DATA + 0x2900 } else { 0 };
            c.bus.write32(this + 0x78c, focus);
            let (world, w84, w0c) = (DATA + 0x4000, DATA + 0x4100, DATA + 0x4200);
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x84, w84);
            c.bus.write32(w84 + 0xc, w0c);
            c.bus.write32(w0c + 0xa4, DATA + 0x4300);
            // The metrics (+0x790): its own HUD table decides a call.
            let metrix = if present(4) { DATA + 0x4400 } else { 0 };
            c.bus.write32(this + 0x790, metrix);
            c.bus.write32(
                DATA + 0x4400 + 0x1b0,
                if seed % 3 == 0 { DATA + 0x4500 } else { 0 },
            );
        });
    }

    // -- RiderBase_LoadState ---------------------------------------------------------------------

    /// `RiderBase_LoadState`: the save-state stream (its `read` is a fake
    /// method that fills the two resource ids from the script), the resource
    /// table lookups (id none, absent entry, zero or non-zero slot), the
    /// optional focus-box copy, and unwinds in the ~50 calls. The FX objects'
    /// `0x2dcf28` / `0x2f1148` run for real over zeroed memory.
    #[test]
    fn rider_base_load_state_matches() {
        covered(
            ee::s00::f_00111d98__generated,
            rider_base_load_state,
            200,
            &|c, seed| {
                let mut r = oc439_machine(c, seed, 0x111d98, 50, 0x0011_1d98);
                let (this, part, stream, vt) =
                    (DATA + 0x1000, DATA + 0x3000, DATA + 0x8000, DATA + 0x8100);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(stream);
                c.bus.write32(this + 0x18, part);
                // The stream: vtable record with delta at +0x10, read method at +0x14.
                c.bus.write32(stream, vt);
                c.bus.write16(vt + 0x10, ((r.next_u32() % 8) * 4) as u16);
                c.bus.write32(vt + 0x14, OC439_FAKE_READ);
                // The part's objects and the resource destinations.
                for off in [0x780u32, 0x784, 0x788, 0x790, 0xabc, 0xac0] {
                    c.bus
                        .write32(part + off, DATA + 0x9000 + 0x40 * (off & 0xff));
                }
                c.bus.write32(part + 0x864, DATA + 0x7000);
                c.bus.write32(part + 0x868, DATA + 0x7100);
                let focus = if seed % 3 == 0 { 0 } else { DATA + 0x7200 };
                c.bus.write32(part + 0x78c, focus);
                fill(c, &mut r, part + 0x110, 4);
                // The resource table: gp+0x16c8 -> table -> +8 -> entries (by
                // `id & 0xff`) -> +0xc -> slot words (by `id >> 8`).
                let (tp, table, entries) = (DATA + 0xa000, DATA + 0xa100, DATA + 0xa200);
                c.bus.write32(GAME_GP + 0x16c8, tp);
                c.bus.write32(tp, table);
                c.bus.write32(table + 8, entries);
                for i in 0..4u32 {
                    let entry = DATA + 0xa300 + 0x40 * i;
                    c.bus
                        .write32(entries + 4 * i, if (seed + i) % 5 == 0 { 0 } else { entry });
                    let slots = DATA + 0xa500 + 0x40 * i;
                    c.bus.write32(entry + 0xc, slots);
                    for j in 0..4u32 {
                        let w = if (seed + i + j) % 4 == 0 {
                            r.next_u32() & 0xff
                        } else {
                            r.next_u32()
                        };
                        c.bus.write32(slots + 4 * j, w);
                    }
                }
                // The scripted ids: none (-1), or `slot << 8 | index`.
                for k in 0..2u32 {
                    let id = if (seed >> k) % 3 == 0 {
                        u32::MAX
                    } else {
                        (r.next_u32() % 4) << 8 | (r.next_u32() % 4)
                    };
                    c.bus.write32(OC439_READS + 4 * k, id);
                }
                c.bus.write32(OC439_READS + 8, r.next_u32());
                c.bus.write32(OC439_READS + 12, r.next_u32());
                c.bus.write32(OC439_READ_COUNT, 0);
            },
        );
    }
}
