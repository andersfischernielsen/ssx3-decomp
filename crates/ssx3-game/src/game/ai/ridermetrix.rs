//! `src/ai/ridermetrix.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::ai::stream_write_member;
pub(crate) use crate::game::ai::ai::virtual_call_slot;
pub(crate) use crate::game::ai::rider::fcc;
pub(crate) use crate::game::ai::rider::fr;
pub(crate) use crate::game::ai::rider::gp;
pub(crate) use crate::game::ai::rider::ldf;
pub(crate) use crate::game::ai::rider::li_f;
pub(crate) use crate::game::ai::rider::lw;
pub(crate) use crate::game::ai::rider::p;
pub(crate) use crate::game::ai::rider::per_tick_60;
pub(crate) use crate::game::ai::rider::qcopy;
pub(crate) use crate::game::ai::rider::rate_step_60;
pub(crate) use crate::game::ai::rider::setf;
pub(crate) use crate::game::ai::rider::smoothing_60;
pub(crate) use crate::game::ai::rider::stf;
pub(crate) use crate::game::ai::rider::v_length;
pub(crate) use crate::game::fe::debugfe::at as at_2;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::fe::festateload::glo;
pub(crate) use crate::game::main::gamemode::gpo;
pub(crate) use crate::game::main::gamemode::low as low_2;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::wscript::wscriptman::sw;
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
        0x0011_73b8,
        rider_metrix_clear_hud_messages,
        "RiderMetrix_ClearHUDMessages",
    ),
    (
        0x0011_74e8,
        rider_metrix_load_state,
        "RiderMetrix_LoadState",
    ),
    (
        0x0011_7520,
        rider_metrix_set_base_score,
        "RiderMetrix_SetBaseScore",
    ),
    (0x0011_7540, rider_metrix_reset, "RiderMetrix_Reset"),
    (
        0x0011_76f8,
        rider_metrix_clear_peak_a4,
        "RiderMetrix_ClearPeakA4",
    ),
    (
        0x0011_7708,
        rider_metrix_update_peak_a4,
        "RiderMetrix_UpdatePeakA4",
    ),
    (
        0x0011_7900,
        rider_metrix_get_trick_points,
        "RiderMetrix_GetTrickPoints",
    ),
    (
        0x0011_7948,
        rider_metrix_get_rounded_trick_points,
        "RiderMetrix_GetRoundedTrickPoints",
    ),
    (
        0x0011_7a58,
        rider_metrix_find_hud_slot,
        "RiderMetrix_FindHUDSlot",
    ),
    (0x0011_7c28, rider_metrix_update, "RiderMetrix_Update"),
    (
        0x0011_7fe0,
        rider_metrix_update_trick_hud,
        "RiderMetrix_UpdateTrickHUD",
    ),
    (
        0x0011_9210,
        rider_metrix_check_score_milestone,
        "RiderMetrix_CheckScoreMilestone",
    ),
    (
        0x0011_9368,
        rider_metrix_on_rider_reset,
        "RiderMetrix_OnRiderReset",
    ),
    (
        0x0011_93e0,
        rider_metrix_on_race_finished,
        "RiderMetrix_OnRaceFinished",
    ),
    (
        0x0011_9400,
        rider_metrix_on_knockdown_opponent,
        "RiderMetrix_OnKnockdownOpponent",
    ),
    (
        0x0011_9448,
        rider_metrix_record_best_jump,
        "RiderMetrix_RecordBestJump",
    ),
    (
        0x0011_94c0,
        rider_metrix_on_split_marker,
        "RiderMetrix_OnSplitMarker",
    ),
    (
        0x0011_9958,
        rider_metrix_chain_uber_grind,
        "RiderMetrix_ChainUberGrind",
    ),
    (
        0x0011_99f8,
        rider_metrix_start_board_press,
        "RiderMetrix_StartBoardPress",
    ),
    (
        0x0011_9a38,
        rider_metrix_finish_board_press,
        "RiderMetrix_FinishBoardPress",
    ),
    (
        0x0011_9ad8,
        rider_metrix_set_press_spin,
        "RiderMetrix_SetPressSpin",
    ),
    (
        0x0011_9bf0,
        rider_metrix_start_handplant,
        "RiderMetrix_StartHandplant",
    ),
    (
        0x0011_9c98,
        rider_metrix_start_spin_trick,
        "RiderMetrix_StartSpinTrick",
    ),
    (
        0x0011_a110,
        rider_metrix_on_mission_completed,
        "RiderMetrix_OnMissionCompleted",
    ),
    (
        0x0011_b3f8,
        rider_update_target_speed,
        "Rider_UpdateTargetSpeed",
    ),
    (
        0x0011_98d8,
        rider_metrix_set_flip_amount,
        "RiderMetrix_SetFlipAmount",
    ),
    (
        0x0011_7908,
        rider_metrix_get_trick_bonus,
        "RiderMetrix_GetTrickBonus",
    ),
    (
        0x0011_9898,
        rider_metrix_set_spin_amount,
        "RiderMetrix_SetSpinAmount",
    ),
    (
        0x0011_a228,
        rider_metrix_finish_trick,
        "RiderMetrix_FinishTrick",
    ),
    (
        0x0011_74b0,
        rider_metrix_save_state,
        "RiderMetrix_SaveState",
    ),
    (
        0x0011_7718,
        rider_metrix_bank_combo,
        "RiderMetrix_BankCombo",
    ),
    (0x0011_9708, rider_metrix_add_trick, "RiderMetrix_AddTrick"),
    (
        0x0011_9d40,
        rider_metrix_land_trick,
        "RiderMetrix_LandTrick",
    ),
    (
        0x0011_9e38,
        rider_metrix_interrupt_trick,
        "RiderMetrix_InterruptTrick",
    ),
    (
        0x0011_7638,
        rider_metrix_add_combo_points,
        "RiderMetrix_AddComboPoints",
    ),
    (
        0x0011_9068,
        rider_metrix_add_trick_history,
        "RiderMetrix_AddTrickHistory",
    ),
    (
        0x0011_90f0,
        rider_metrix_count_trick_repeats,
        "RiderMetrix_CountTrickRepeats",
    ),
    (
        0x0011_97d8,
        rider_metrix_complete_trick,
        "RiderMetrix_CompleteTrick",
    ),
    (
        0x0011_b1a8,
        trick_lookup_bonus_trick,
        "Trick_LookupBonusTrick",
    ),
    (
        0x0011_75f8,
        rider_metrix_reset_combo,
        "RiderMetrix_ResetCombo",
    ),
    (
        0x0011_7990,
        rider_metrix_get_trick_score,
        "RiderMetrix_GetTrickScore",
    ),
    (
        0x0011_7ae8,
        rider_metrix_show_hud_text,
        "RiderMetrix_ShowHUDText",
    ),
    (
        0x0011_8ff8,
        rider_metrix_show_trick_name,
        "RiderMetrix_ShowTrickName",
    ),
    (
        0x0011_a7a8,
        ridermetrix_bank_pending_score,
        "RiderMetrix_BankPendingScore",
    ),
    (
        0x0011_9b08,
        rider_metrix_on_wipeout,
        "RiderMetrix_OnWipeout",
    ),
    (
        0x0011_9bb0,
        rider_metrix_get_recovery_boost,
        "RiderMetrix_GetRecoveryBoost",
    ),
    (
        0x0011_a0c0,
        rider_metrix_return_zero_a0c0,
        "RiderMetrix_ReturnZero_a0c0",
    ),
    (
        0x0011_a168,
        rider_metrix_announce_trick,
        "RiderMetrix_AnnounceTrick",
    ),
    (
        0x0011_a8c8,
        rider_metrix_build_trick_desc,
        "RiderMetrix_BuildTrickDesc",
    ),
];

/// Direct calls (same shard; these go through the shard's override slots, so
/// an installed replacement is reached as well).
pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::ai::rider::callee::CHUDFX_UPDATE;

    pub(crate) use crate::game::ai::rider::callee::MEMSET;

    pub(crate) use crate::game::ai::rider::callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::ai::rider::callee::RIDER_GET_SUB_STATE;
    pub(crate) use crate::game::ai::rider::callee::RIDER_METRIX_BANK_COMBO;
    pub(crate) use crate::game::ai::rider::callee::RIDER_METRIX_CHECK_SCORE_MILESTONE;
    pub(crate) use crate::game::ai::rider::callee::RIDER_METRIX_UPDATE_TRICK_HUD;

    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_SYSTEM_SET_PLAYER_TRICK_SCORE;

    pub(crate) use crate::game::fe::festateload::callee::RIDER_METRIX_SET_SPIN_AMOUNT;

    pub(crate) use crate::game::fe::ovstatepause::callee::RIDER_METRIX_CLEAR_PEAK_A4;

    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;

    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_BANK_PENDING_SCORE;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_CLEAR_HUD_MESSAGES;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_COMBO;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_TRICK;

    pub(crate) use crate::game::movie::movieplayer::callee::REMOVE_HUD_MESSAGE;

    pub(crate) use crate::game::movie::movieplayer::callee::SHOW_HUD_NUMBER;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_SHOW_HUD_NUMBER_2;

    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_CLEAR_PEAK_A4 as RIDER_METRIX_CLEAR_PEAK_A4_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_SHOW_HUD_NUMBER;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RIDER_METRIX_BANK_COMBO as RIDER_METRIX_BANK_COMBO_2;

    pub const GET_ROUNDED_TRICK_POINTS: Callee = Callee {
        addr: 0x0011_7948,
        f: ee::s00::RiderMetrix_GetRoundedTrickPoints,
    };

    pub const GET_SCORE_TIER: Callee = Callee {
        addr: 0x0011_9310,
        f: ee::s00::RiderMetrix_GetScoreTier,
    };

    pub const SHOW_HUD_NUMBER_2: Callee = Callee {
        addr: 0x0011_7b88,
        f: ee::s00::RiderMetrix_ShowHUDNumber,
    };

    pub const SHOW_HUD_TEXT: Callee = Callee {
        addr: 0x0011_7ae8,
        f: ee::s00::RiderMetrix_ShowHUDText,
    };

    pub const SHOW_NUMBER_DELAYED: Callee = Callee {
        addr: 0x0011_71a8,
        f: ee::s00::cHUDFX_ShowNumberDelayed,
    };

    pub const REMOVE_HUD_MESSAGE_2: Callee = Callee {
        addr: 0x0011_79e0,
        f: ee::s00::RiderMetrix_RemoveHUDMessage,
    };

    pub const GET_TRICK_BONUS: Callee = Callee {
        addr: 0x0011_7908,
        f: ee::s00::RiderMetrix_GetTrickBonus,
    };

    pub const GET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fee8,
        f: ee::s00::Rider_GetSubState,
    };

    pub const GET_HUD_ANGLE: Callee = Callee {
        addr: 0x0012_f118,
        f: ee::s00::RiderPrewindSub_GetHUDAngle,
    };

    pub const GET_FRAME_COUNTER: Callee = Callee {
        addr: 0x0012_98c8,
        f: ee::s00::AI_GetFrameCounter,
    };

    pub(crate) use crate::game::lib::snd::callee::RIDER_METRIX_REMOVE_HUD_MESSAGE;
    pub(crate) use crate::game::lib::snd::callee::RIDER_METRIX_ROUNDED_TRICK_POINTS;
    pub(crate) use crate::game::lib::snd::callee::RIDER_METRIX_SHOW_HUD_NUMBER as RIDER_METRIX_SHOW_HUD_NUMBER_3;
    pub(crate) use crate::game::lib::snd::callee::RIDER_METRIX_TRICK_BONUS;

    pub(crate) use crate::game::lib::snd::callee::RIDER_METRIX_SHOW_HUD_NUMBER as RIDER_METRIX_SHOW_HUD_NUMBER_4;
}

/// `RiderBase` (and its owner/board sub-objects) fields the HUD reads.
pub(crate) mod rider {

    pub(crate) use crate::game::ai::computer::rider::METRIC_SCALE;

    /// `+0x2f0`: the boost meter (`0` when no boost is available).
    pub const BOOST_METER: u32 = 0x2f0;

    /// `+0x2f4`: the boost level (`< 11` is shown as the number itself).
    pub const BOOST_LEVEL: u32 = 0x2f4;

    /// `+0x304 boostMode`: 3 is "no boost" (`lw & 3`).
    pub const BOOST_MODE: u32 = 0x304;

    /// `+0x2f8 boostStrength` and the owner's meter at `+0x330`.
    pub const BOOST_STRENGTH: u32 = 0x2f8;

    pub const OWNER_METER: u32 = 0x330;

    /// `+0x6c0`: the board object whose `+0x3c` is a virtual method.
    pub const BOARD: u32 = 0x6c0;

    /// `+0x77c owner`: the `RiderBase` (`+0x2c0` is its variant word).
    pub const OWNER: u32 = 0x77c;

    pub const OWNER_MODE: u32 = 0x2c0;
}

/// `gp`-relative globals used here.
pub(crate) mod g {}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {

    pub(crate) use crate::game::scripter::ssxscriptengine::addr::HUD_SHOW_NUMBER_DELAYED;
}

// ---------------------------------------------------------------------------
// Field offsets.

/// `RiderMetrix` fields the trick HUD reads and writes (the reference names
/// from `sMetrix17C28`; the ones the C++ left unnamed keep their offset).
pub(crate) mod mx {

    /// `+0x14 trickBoost` (float): the multiplier `GetRoundedTrickPoints` uses.
    pub const TRICK_BOOST: u32 = 0x14;

    /// `+0x54`: the running trick counter used by the history slot.
    pub const TRICK_COUNT: u32 = 0x54;

    /// `+0x5c`: "a trick landed" flag for the history slot.
    pub const TRICK_FLAG: u32 = 0x5c;

    /// `+0x84 points`: the bonus booked at the end of the frame.
    pub const BONUS: u32 = 0x84;

    /// `+0x9c combo count`, `+0xa0 combo points`, `+0xa4 combo timer`.
    pub const COMBO_COUNT: u32 = 0x9c;

    pub const COMBO_POINTS: u32 = 0xa0;

    pub const COMBO_TIMER: u32 = 0xa4;

    /// `+0x1ac rider` (`RiderBase*`).
    pub const RIDER: u32 = 0x1ac;

    /// `+0x1b0 slots`: the 0x9c-byte HUD message records.
    pub const SLOTS: u32 = 0x1b0;

    /// A record's size (`index` at `+0x98`).
    pub const SLOT_SIZE: u32 = 0x9c;
}

/// Cross-shard calls the generator emitted as `call_indirect`.
pub(crate) mod table {

    /// `SSXAudioSystem_GetInstance()`.
    pub const AUDIO_SYSTEM: u32 = 0x0028_b180;

    /// `SSXAudioSystem_PlayTrickPointsTick(audio, rider, points)`.
    pub const AUDIO_TRICK_TICK: u32 = 0x0029_a7d8;

    /// Audio calls for the combo/boost counters.
    pub const AUDIO_CALL_A: u32 = 0x0029_b0e0;

    pub const AUDIO_CALL_B: u32 = 0x0029_b3c0;

    pub const AUDIO_CALL_C: u32 = 0x0029_9638;

    pub const AUDIO_CALL_D: u32 = 0x0029_97b8;

    /// `cBE_getBE()` and `cBE_getInterface(be, kind)`.
    pub const BE_GET_BE: u32 = 0x0014_dc80;

    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;

    pub const BE_CALL: u32 = 0x0015_0960;

    /// The board's wake-effect methods (`rider+0x230` vtable).
    pub const BOARD_WAKE_CLEAR: u32 = 0x0013_5f70;

    /// `bool board_done(board, bool* out)` (0x1360c8): whether the board's
    /// blend has reached its rest value.
    pub const BOARD_DONE: u32 = 0x0013_60c8;

    pub const BOARD_WAKE_A: u32 = 0x0013_6100;

    pub const BOARD_WAKE_B: u32 = 0x0013_6168;

    /// A sine used by the HUD-effect timer.
    pub const SIN: u32 = 0x0031_c040;

    /// `func_00312aa0(anim, int)` (0x312aa0): play an animation.
    pub const PLAY_ANIM: u32 = 0x0031_2aa0;
}

/// `void RiderMetrix_SetBaseScore(RiderMetrix *m, int base)` (0x117520):
/// `delta = base - m->base (+0x1c8)`, store the new base, and add `delta` to
/// the running total at +0x198.
pub fn rider_metrix_set_base_score(c: &mut Cpu) {
    c.cycles += 7;
    let m = Addr(abi::arg(c, 0));
    let old_base = m.off(0x1c8).lw(c);
    c.g[V0][0] = old_base;
    let total = m.off(0x198).lw(c);
    c.g[V1][0] = total;
    let delta = abi::arg(c, 1).wrapping_sub(old_base as u32);
    c.g[V0][0] = sx(delta);
    m.off(0x1c8).sw(c, c.g[A1][0]);
    let new_total = (total as u32).wrapping_add(delta);
    c.g[V1][0] = sx(new_total);
    m.off(0x198).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `int RiderMetrix_GetRoundedTrickPoints(RiderMetrix *m)` (0x117948): compute
/// `round_down((m->pts (+0x1c4) * m->scale (+0x14) * k + 5.0), 10)`, the
/// division being a signed 32-bit `div` whose remainder is subtracted.
pub fn rider_metrix_get_rounded_trick_points(c: &mut Cpu) {
    let m = Addr(abi::arg(c, 0));

    c.cycles += 5;
    let scale = m.off(0x14).read_fpu(c); // f2
    c.f[2] = scale.0;
    c.g[A1][0] = 0xa;
    let points = m.off(0x1c4).read_fpu(c); // f0
    c.f[0] = points.0;
    // (the original's `a1 == 0` guard is dead: a1 is 10)

    c.cycles += 12;
    let k = Addr((c.g[GP][0] as u32).wrapping_sub(0x7b08)).read_fpu(c); // f3
    c.f[3] = k.0;
    let scaled = points * scale;
    c.f[0] = scaled.0;
    c.g[AT][0] = 0x40a0_0000; // 5.0
    c.f[1] = c.g[AT][0] as u32; // mtc1 f1, at
    let scaled = scaled * k;
    c.f[0] = scaled.0;
    let biased = scaled + FpuF32(c.f[1]); // add.s f0, f0, f1
    c.f[0] = biased.0;

    let v = biased.to_i32(); // cvt.w.s f1, f0
    c.f[1] = v as u32;
    let vx = sx(v as u32); // mfc1 v0, f1
    c.g[V0][0] = vx;

    let (_, rem) = abi::div(c, v as u32, 10); // div v0, a1
    c.g[V1][0] = sx(rem);
    c.g[V0][0] = sx((vx as u32).wrapping_sub(rem));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// RiderMetrix_Update.

/// `void RiderMetrix_Update(RiderMetrix* m)` (0x117c28): per-frame scoring
/// of the rider `+0x1ac`. With `dt = frame_dt * gp-0x7b00` and the speed:
/// the bank-combo timer `+0xa4` runs down (`RiderMetrix_BankCombo` when it
/// expires, unless in a main state or sub-state 1); the running timers
/// (`+0x30` air, `+0x40`, `+0x24` distance, `+0x2c`, `+0x6c`, `+0x78`)
/// advance while non-negative, add to the score `+0x14` and report score
/// milestones (`RiderMetrix_CheckScoreMilestone`, bonus into `+0x84`);
/// distance/peak/frame statistics; the trick HUD and HUD effects; then
/// the score goes to the audio system.
pub fn rider_metrix_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x80);
        let dt_scale = ldf(c, 0, gp(c, -0x7b00));
        frame.save(c, S0, 0x50);
        frame.save_fpr(c, 21, 0x68);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 22, 0x70);
        c.f[21] = 0;
        let m = r32(c, S0);
        c.g[A0][0] = lw(c, p(m, 0x1ac));
        let r = r32(c, A0);
        let bank = ldf(c, 1, p(m, 0xa4));
        let speed = v_length(c, p(r, 0x1e0), V0, 22);
        let frame_dt = ldf(c, 2, p(r, 0x300));
        let zero = fr(c, 21);
        let banking = fcc(c, zero.le(bank));
        let dt = setf(c, 20, frame_dt * dt_scale);

        // The bank-combo timer. 0: timer path (reload air, clear f21),
        // 1: sub-state 1 (clear f21), 2: in a main state.
        let mut path = 0;
        if banking {
            c.cycles += 4;
            callee::RIDER_GET_MAIN_STATE.call(c, 0x0011_7ca4)?;
            if c.g[V0][0] != 0 {
                ldf(c, 0, p(m, 0x30));
                path = 2;
            } else {
                c.cycles += 5;
                c.g[A0][0] = lw(c, p(m, 0x1ac));
                callee::RIDER_GET_SUB_STATE.call(c, 0x0011_7cb4)?;
                c.g[V1][0] = 1;
                if c.g[V0][0] == c.g[V1][0] {
                    ldf(c, 0, p(m, 0x30));
                    path = 1;
                } else {
                    c.cycles += 6;
                    let left = ldf(c, 0, p(m, 0xa4));
                    let left = setf(c, 0, left - dt);
                    let zero = fr(c, 21);
                    let expired = fcc(c, left.le(zero));
                    stf(c, 0, p(m, 0xa4));
                    if expired {
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S0][0];
                        callee::RIDER_METRIX_BANK_COMBO.call(c, 0x0011_7ce0)?;
                    }
                }
            }
        }
        if path == 0 {
            c.cycles += 1;
            ldf(c, 0, p(m, 0x30));
        }
        if path <= 1 {
            c.cycles += 1;
            c.f[21] = 0;
        }

        // Air time.
        c.cycles += 4;
        let air = fr(c, 0);
        let zero = fr(c, 21);
        c.g[A2][0] = 0x46_0000;
        if fcc(c, zero.le(air)) {
            c.cycles += 11;
            let air = setf(c, 0, air + dt);
            c.g[A2][0] = sx(0x0045_9e00);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(p(m, 0x88));
            c.g[A3][0] = 0x1c;
            setf(c, 12, air);
            stf(c, 0, p(m, 0x30));
            add_milestone_score(c, m, 0x0011_7d18)?;
        }

        // Timer +0x40, scoring +0x3c per frame.
        c.cycles += 5;
        let t = ldf(c, 2, p(m, 0x40));
        let zero = fr(c, 21);
        c.g[A2][0] = 0x46_0000;
        if fcc(c, zero.le(t)) {
            c.cycles += 15;
            let score = ldf(c, 0, p(m, 0x14));
            let t = setf(c, 2, t + dt);
            ldf(c, 1, p(m, 0x3c));
            // `+= award` per tick (a held trick's points/sec / 600000, from
            // `RiderMetrix_AddTrick`): the hold pays out in real time.
            per_tick_60(c, 1);
            let bonus = fr(c, 1);
            c.g[A2][0] = sx(0x0045_9e20);
            c.g[A0][0] = c.g[S0][0];
            setf(c, 0, score + bonus);
            setf(c, 12, t);
            stf(c, 2, p(m, 0x40));
            c.g[A1][0] = sx(p(m, 0x8c));
            c.g[A3][0] = 0x20;
            stf(c, 0, p(m, 0x14));
            add_milestone_score(c, m, 0x0011_7d68)?;
        }

        // Distance +0x24 (speed * dt), and +0x1c while descending.
        c.cycles += 5;
        let t = ldf(c, 2, p(m, 0x24));
        let zero = fr(c, 21);
        if fcc(c, zero.le(t)) {
            c.cycles += 16;
            let step = setf(c, 3, speed * dt);
            let rate = ldf(c, 0, gp(c, -0x7afc));
            let score = ldf(c, 1, p(m, 0x14));
            c.g[V1][0] = lw(c, p(m, 0x1ac));
            let rv = r32(c, V1);
            let inc = setf(c, 0, step * rate);
            setf(c, 2, t + step);
            setf(c, 1, score + inc);
            stf(c, 2, p(m, 0x24));
            stf(c, 1, p(m, 0x14));
            let sp = sp(c);
            qcopy(c, V0, p(rv, 0x1c0), sp);
            let vz = ldf(c, 0, p(sp, 8));
            let zero = fr(c, 21);
            let down = fcc(c, vz.lt(zero));
            c.g[A2][0] = 0x46_0000;
            if down {
                c.cycles += 5;
                let k = ldf(c, 0, gp(c, -0x7af8));
                let d = ldf(c, 1, p(m, 0x1c));
                let k = setf(c, 0, step * k);
                setf(c, 1, d + k);
                stf(c, 1, p(m, 0x1c));
            }
            c.cycles += 10;
            ldf(c, 12, p(m, 0x24));
            c.g[A2][0] = sx(0x0045_9f68);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(p(m, 0x98));
            c.g[A3][0] = 0x1d;
            add_milestone_score(c, m, 0x0011_7df4)?;
        }
        ldf(c, 2, p(m, 0x2c));

        // Timer +0x2c.
        c.cycles += 5;
        c.f[21] = 0;
        let t = fr(c, 2);
        let zero = fr(c, 21);
        c.g[A2][0] = 0x46_0000;
        if fcc(c, zero.le(t)) {
            timed_milestone(c, m, 0x2c, -0x7af4, 0x0045_9e40, 0x90, 0x1e, 0x0011_7e4c)?;
        }

        // Timer +0x6c (score only).
        c.cycles += 5;
        let t = ldf(c, 2, p(m, 0x6c));
        let zero = fr(c, 21);
        if fcc(c, zero.le(t)) {
            c.cycles += 8;
            let rate = ldf(c, 0, gp(c, -0x7af0));
            setf(c, 2, t + dt);
            let score = ldf(c, 1, p(m, 0x14));
            let inc = setf(c, 0, dt * rate);
            stf(c, 2, p(m, 0x6c));
            setf(c, 1, score + inc);
            stf(c, 1, p(m, 0x14));
        }
        ldf(c, 2, p(m, 0x78));

        // Timer +0x78.
        c.cycles += 4;
        let t = fr(c, 2);
        let zero = fr(c, 21);
        c.g[A2][0] = 0x46_0000;
        if fcc(c, zero.le(t)) {
            timed_milestone(c, m, 0x78, -0x7aec, 0x0045_9f08, 0x94, 0x1f, 0x0011_7ed0)?;
        }

        // Statistics: smoothed speed peak, distance, frames, top speed.
        c.cycles += 18;
        let avg = ldf(c, 3, p(m, 0x1a8));
        let k_keep = ldf(c, 0, gp(c, -0x7ae8));
        let k_new = ldf(c, 1, gp(c, -0x7ae4));
        // `avg = avg*0.9 + speed*0.1` per tick is a smoothing filter (`a` = 0.1):
        // at another rate both factors follow from `a' = smoothing_60(a)`, so
        // the average speed keeps its time constant.
        let (k_keep, k_new) = if crate::tick::rate(c) == 60 {
            (k_keep, k_new)
        } else {
            let a = smoothing_60(c, k_new);
            let keep = FpuF32::ONE - a;
            c.f[0] = keep.0;
            c.f[1] = a.0;
            (keep, a)
        };
        let avg = setf(c, 3, avg * k_keep);
        let dist = ldf(c, 2, p(m, 0x164));
        let add = setf(c, 1, speed * k_new);
        let frames = ldf(c, 0, p(m, 0x168));
        let one = li_f(c, 4, 0x3f80_0000);
        setf(c, 2, dist + speed);
        let peak = ldf(c, 5, p(m, 0x160));
        let avg = setf(c, 3, avg + add);
        setf(c, 0, frames + one);
        stf(c, 2, p(m, 0x164));
        let new_peak = fcc(c, peak.lt(avg));
        stf(c, 3, p(m, 0x1a8));
        stf(c, 0, p(m, 0x168));
        if new_peak {
            c.cycles += 1;
            stf(c, 3, p(m, 0x160));
        }
        c.cycles += 5;
        let top = ldf(c, 0, p(m, 0x15c));
        if fcc(c, top.lt(speed)) {
            stf(c, 22, p(m, 0x15c));
        }

        // HUD.
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        c.g[S3][0] = sx(p(m, 0x1b8));
        callee::RIDER_METRIX_UPDATE_TRICK_HUD.call(c, 0x0011_7f48)?;
        c.g[V0][0] = lw(c, p(m, 0x1b0));
        let has_fx = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S3][0];
        if has_fx {
            c.cycles += 3;
            c.g[V0][0] = lw(c, p(m, 0x1b4));
            let none = (c.g[V0][0] as i64) <= 0;
            c.g[S1][0] = 0;
            if !none {
                c.cycles += 2;
                c.g[S2][0] = 0;
                loop {
                    c.cycles += 8;
                    let m = r32(c, S0);
                    c.g[A0][0] = lw(c, p(m, 0x1b0));
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    let fx = r32(c, A0).wrapping_add(r32(c, S2));
                    c.g[A0][0] = sx(fx);
                    callee::CHUDFX_UPDATE.call(c, 0x0011_7f78)?;
                    c.g[V0][0] = lw(c, p(m, 0x1b4));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(0x9c));
                    if !more {
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S3][0];
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0011_7f68);
                    }
                }
            }
        }

        // memset(m + 0x1b8, 0, 8); m->hud_dirty = 1; audio trick score.
        c.cycles += 20;
        c.g[A1][0] = 0;
        c.g[A2][0] = 8;
        call_ptr(c, callee::MEMSET, 0x0011_7f98)?;
        c.g[V1][0] = 1;
        c.sw(p(m, 0x1c0), 1);
        call_ptr(c, callee::SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x0011_7fa4)?;
        ldf(c, 12, p(m, 0x14));
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = lw(c, p(m, 0x1ac));
        call_ptr(
            c,
            callee::SSX_AUDIO_SYSTEM_SET_PLAYER_TRICK_SCORE,
            0x0011_7fb4,
        )?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 22, 0x70);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// `RiderMetrix_CheckScoreMilestone(...)`, then `m->bonus (+0x84) += v0`.
pub(crate) fn add_milestone_score(c: &mut Cpu, m: u32, ret: u32) -> Flow {
    callee::RIDER_METRIX_CHECK_SCORE_MILESTONE.call(c, ret)?;
    c.g[V1][0] = lw(c, p(m, 0x84));
    let bonus = r32(c, V1).wrapping_add(r32(c, V0));
    c.g[V1][0] = sx(bonus);
    c.sw(p(m, 0x84), sx(bonus));
    Ok(())
}

/// A running timer (f2 = its value, non-negative; 16-cycle block):
/// `timer += dt`, `score += dt * rate`, then the milestone check on the
/// timer (`CheckScoreMilestone(m, m + slot, table, kind)`).
#[allow(clippy::too_many_arguments)]
pub(crate) fn timed_milestone(
    c: &mut Cpu,
    m: u32,
    timer: u32,
    rate_gp: i32,
    table: u32,
    slot: u32,
    kind: u64,
    ret: u32,
) -> Flow {
    c.cycles += 16;
    let dt = fr(c, 20);
    let t = fr(c, 2);
    let rate = ldf(c, 0, gp(c, rate_gp));
    let t = setf(c, 2, t + dt);
    let score = ldf(c, 1, p(m, 0x14));
    let inc = setf(c, 0, dt * rate);
    c.g[A2][0] = sx(table);
    setf(c, 12, t);
    stf(c, 2, p(m, timer));
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = sx(p(m, slot));
    setf(c, 1, score + inc);
    c.g[A3][0] = kind;
    stf(c, 1, p(m, 0x14));
    add_milestone_score(c, m, ret)
}

/// `int RiderMetrix_CheckScoreMilestone(Rider* self, int* idx,
/// const GradeStep* grades, int type)` (0x119210): when the step at `*idx`
/// has been reached, advance the index, drop the five HUD messages of the
/// previous tier and show the new score; returns the awarded value or 0.
pub fn rider_metrix_check_score_milestone(c: &mut Cpu) {
    c.cycles += 17;
    let frame = Frame::open(c, 0x50);
    let idx = abi::arg(c, 1);
    frame.save(c, S1, 0x20);
    frame.save(c, S2, 0x10);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x30);
    frame.save_ra(c, 0);
    frame.save_fpr(c, 20, 0x40);
    c.mtc1(0, 0);
    c.g[V1][0] = c.lw(idx);
    c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(abi::arg(c, 2)));
    c.lwc1(20, r32(c, V0));
    c.c_lt_s(20, 0);
    let below_zero = c.fcc();
    c.g[S2][0] = c.g[A3][0];
    if !below_zero {
        c.cycles += 4;
        c.c_le_s(20, 12);
        if c.fcc() {
            // The step's threshold has been reached: award the milestone.
            c.lwc1(1, p(r32(c, S1), rider::METRIC_SCALE));
            c.cycles += 33;
            c.g[A0][0] = c.g[S1][0];
            c.lwc1(0, p(r32(c, V0), 0x04));
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 2);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.mul_s(0, 0, 1);
            c.sw(idx, c.g[V0][0]);
            c.add_s(0, 0, 2);
            c.cvt_w_s(1, 0);
            c.mfc1(16, 1);
            // Drop the five HUD messages of the old tier.
            c.g[A1][0] = 0x1c;
            c.g[A0][0] = c.g[S1][0];
            if callee::REMOVE_HUD_MESSAGE.call(c, 0x0011_929c).is_err() {
                return;
            }
            c.g[A1][0] = 0x1d;
            c.g[A0][0] = c.g[S1][0];
            if callee::REMOVE_HUD_MESSAGE.call(c, 0x0011_92a8).is_err() {
                return;
            }
            c.g[A1][0] = 0x1e;
            c.g[A0][0] = c.g[S1][0];
            if callee::REMOVE_HUD_MESSAGE.call(c, 0x0011_92b4).is_err() {
                return;
            }
            c.g[A1][0] = 0x1f;
            c.g[A0][0] = c.g[S1][0];
            if callee::REMOVE_HUD_MESSAGE.call(c, 0x0011_92c0).is_err() {
                return;
            }
            c.g[A1][0] = 0x20;
            c.g[A0][0] = c.g[S1][0];
            if callee::REMOVE_HUD_MESSAGE.call(c, 0x0011_92cc).is_err() {
                return;
            }
            c.cvt_w_s(0, 20);
            c.mfc1(7, 0);
            c.g[AT][0] = 0x3fc0_0000;
            c.mtc1(1, 12);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S0][0];
            if call_ptr(c, callee::SHOW_HUD_NUMBER, 0x0011_92ec).is_err() {
                return;
            }
            c.g[V0][0] = c.g[S0][0];
            return rider_metrix_leave(c, &frame);
        }
    }
    // Threshold negative, or not reached yet.
    c.cycles += 2;
    c.g[V0][0] = 0;
    rider_metrix_leave(c, &frame);
}

/// The epilogue of [`rider_metrix_check_score_milestone`].
#[inline(always)]
pub(crate) fn rider_metrix_leave(c: &mut Cpu, frame: &Frame) {
    c.cycles += 7;
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x40);
    frame.close(c);
}

/// `void RiderMetrix_ClearPeakA4(RiderMetrix* m)` (0x1176f8): resets the peak
/// air metric at +0xa4 to −1 (an unset marker) and leaves −1.0f in f0.
pub fn rider_metrix_clear_peak_a4(c: &mut Cpu) {
    c.cycles += 4;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(1, 0); // f0 = -1.0f
    let selfp = abi::arg(c, 0);
    c.swc1(0, selfp.wrapping_add(0xa4));
    abi::ret(c);
}

/// `void RiderMetrix_UpdatePeakA4(RiderMetrix* m, float seconds)` (0x117708):
/// raises the peak air metric at +0xa4 to the new value.
pub fn rider_metrix_update_peak_a4(c: &mut Cpu) {
    c.cycles += 4;
    let selfp = abi::arg(c, 0);
    let peak = selfp.wrapping_add(0xa4);
    c.lwc1(0, peak);
    c.max_s(0, 0, 12); // max(current, seconds)
    c.swc1(0, peak);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Forwards with extra arguments or a result the wrapper re-reads.

/// `float RiderMetrix_OnRaceFinished(this)` (0x001193e0): bank the combo, then
/// return 0.0 (the float is materialised with `mtc1`).
pub fn rider_metrix_on_race_finished(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, callee::RIDER_METRIX_BANK_COMBO_2, 0x0011_93f0)?;
        c.mtc1(0, 0);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderMetrix_SetPressSpin(RiderMetrix* self, int on)` (0x119ad8):
/// record the press flag (+0x28) and add pi to the spin amount (+0x34).
pub fn rider_metrix_set_press_spin(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let base = c.g[V0][0] as u32;
        c.lwc1(0, glo(c, -0x7a0c));
        c.lwc1(12, at(base, 0x34));
        let on = c.g[A1][0];
        Addr(at(base, 0x28)).sw(c, on);
        let sum = crate::ps2f::FpuF32(c.f[12]) + crate::ps2f::FpuF32(c.f[0]);
        c.f[12] = sum.0;
        callee::RIDER_METRIX_SET_SPIN_AMOUNT.call(c, 0x0011_9af8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `RiderMetrix_LoadState(self, iface)` (0x1174e8): `iface->v02(self, 0x1ac)`.
pub fn rider_metrix_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0011_7510, Some(0x1ac)))
}

/// `float RiderMetrix_StartBoardPress(RiderMetrix* m, int a1)` (0x1199f8):
/// record the press (`+0x10`, `+0x28`), keep the peak at `+0x2c` at least
/// zero, clear the peak tracker and return 0.0 (f0 and `f20`).
pub fn rider_metrix_start_board_press(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x10);
        c.mtc1(0, 20);
        let this = crate::abi::r32(c, V0);
        c.lwc1(0, p(this, 0x2c));
        Addr(p(this, 0x28)).sw(c, c.g[A1][0]);
        c.max_s(0, 0, 20);
        Addr(p(this, 0x10)).sw(c, c.g[A1][0]);
        c.swc1(0, p(this, 0x2c));
        callee::RIDER_METRIX_CLEAR_PEAK_A4.call(c, 0x0011_9a24)?;
        c.mov_s(0, 20);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider metrics / BE.

/// `float RiderMetrix_OnKnockdownOpponent(RiderMetrix* self)` (0x119400):
/// bump the knockdown count at `+0x128` and pop it up as HUD number 0x2c.
pub fn rider_metrix_on_knockdown_opponent(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[AT][0] = 0x3fc0_0000;
        c.mtc1(1, 12);
        c.g[A1][0] = 0x2c;
        c.g[V0][0] = Addr(r32(c, V1)).off(0x128).lw(c);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        c.sw(p(r32(c, V1), 0x128), c.g[V0][0]);
        callee::RIDER_METRIX_SHOW_HUD_NUMBER.call(c, 0x0011_9430)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        frame.restore_ra(c, 0);
        leave(c, 0x10);
        Ok(())
    })
}

/// `float RiderMetrix_StartHandplant(RiderMetrix* self, u32 arg)` (0x119bf0):
/// arm the handplant timer (`+0x7c`, `+0x78`), set `+0x30` to -1 and advance
/// `+0x14` by the frame step at gp-0x7a04.
pub fn rider_metrix_start_handplant(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.lwc1(2, gp(c, -0x7a04));
        c.lwc1(0, p(r32(c, V0), 0x14));
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 1);
        c.add_s(0, 0, 2);
        c.sw(p(r32(c, V0), 0x7c), c.g[A1][0]);
        c.swc1(1, p(r32(c, V0), 0x30));
        c.sw(p(r32(c, V0), 0x78), 0);
        c.swc1(0, p(r32(c, V0), 0x14));
        callee::RIDER_METRIX_CLEAR_PEAK_A4_2.call(c, 0x0011_9c24)?;
        c.mtc1(0, 0);
        frame.restore_ra(c, 0);
        leave(c, 0x10);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider metrics and state.

/// `void RiderMetrix_ClearHUDMessages(RiderMetrix* m)` (0x1173b8): mark every
/// HUD message (`m+0x1b0`, `m+0x1b4` entries of 0x9c bytes) as state 0x34.
pub fn rider_metrix_clear_hud_messages(c: &mut Cpu) {
    c.cycles += 3;
    let m = r32(c, A0);
    c.g[V0][0] = c.lw(m.wrapping_add(0x1b0));
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = c.lw(m.wrapping_add(0x1b4));
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[V1][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[A2][0] = 0x34;
            c.g[A1][0] = 0;
            loop {
                c.cycles += 8;
                c.g[V0][0] = c.lw(m.wrapping_add(0x1b0));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                c.sw(r32(c, V0), c.g[A2][0]);
                c.g[V0][0] = c.lw(m.wrapping_add(0x1b4));
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x9c));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0011_73d8);
                }
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void RiderMetrix_OnMissionCompleted(RiderMetrix* m, int n)` (0x11a110):
/// show the HUD number (1.5f), award 4 points and return 0.0f.
pub fn rider_metrix_on_mission_completed(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x30);
    c.g[AT][0] = 0x3fc0_0000;
    c.mtc1(1, 12);
    frame.save(c, S0, 0x20);
    c.g[A3][0] = 0;
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    c.g[S1][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[A2][0] = c.g[S1][0];
    c.g[A1][0] = 0x1b;
    if callee::RIDER_METRIX_SHOW_HUD_NUMBER_2
        .call(c, 0x0011_a140)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A1][0] = 4;
    if callee::RIDER_METRIX_AWARD_POINTS
        .call(c, 0x0011_a150)
        .is_err()
    {
        return;
    }
    c.mtc1(0, 0);
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void Rider_FinishHandplant(Rider* rider)` (0x119c38): finish a handplant
/// trick, reset the trick metrix, and return the trick result in `f0`.
pub fn rider_metrix_finish_handplant(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        frame.save_fpr(c, 20, 0x20);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[V0][0] = Addr(at_2(c, S0, 0x80)).lw(c);
        c.g[T1][0] = 0x1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        c.g[RA][0] = sx(0x0011_9c70);
        Addr(at_2(c, S0, 0x80)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0011_a228, 0x0011_9c70)?;
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0011_7838, 0x0011_9c7c)?;
        Addr(at_2(c, S0, 0x30)).sw(c, 0);
        c.mov_s(0, 20);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `float RiderMetrix_GetTrickPoints(RiderMetrix* m)` (0x117900): the trick
/// point total held at +0x14.
pub fn rider_metrix_get_trick_points(c: &mut Cpu) {
    c.cycles += 2;
    let m = abi::arg(c, 0);
    c.lwc1(0, m.wrapping_add(0x14));
    abi::ret(c);
}

/// `RiderMetrix_RecordBestJump` (0x119448): count a jump; when the rider beat its
/// best jump height, show the number and remember it. Returns 0.0f.
pub fn rider_metrix_record_best_jump(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        c.g[V0][0] = lw_at(c, S0, 0x130);
        c.g[A0][0] = lw_at(c, S0, 0x1b0);
        c.g[V0][0] = low(c, V0).wrapping_add(1);
        let no_hud = c.g[A0][0] == 0;
        sw_at(c, S0, 0x130, c.g[V0][0]);
        if !no_hud {
            c.cycles += 5;
            c.lwc1(0, r32(c, S0).wrapping_add(0x18));
            c.c_lt_s(0, 20);
            let not_better = !c.fcc();
            c.g[A0][0] = low(c, A0).wrapping_add(0x270);
            if !not_better {
                c.cycles += 7;
                c.cvt_w_s(0, 20);
                c.mfc1(6, 0);
                c.mtc1(ZERO, 12);
                c.g[A1][0] = 4;
                c.g[A3][0] = 0;
                call_ptr(c, addr::HUD_SHOW_NUMBER_DELAYED, 0x0011_94a0)?;
                c.swc1(20, r32(c, S0).wrapping_add(0x18));
            }
        }
        c.cycles += 6;
        c.mtc1(ZERO, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `lwc1 f, off(reg)`.
#[inline]
pub(crate) fn lwc1_r(c: &mut Cpu, f: usize, reg: usize, off: u32) {
    let a = low_2(c, reg).wrapping_add(off);
    c.lwc1(f, a);
}

/// `swc1 f, off(reg)`.
#[inline]
pub(crate) fn swc1_r(c: &mut Cpu, f: usize, reg: usize, off: u32) {
    let a = low_2(c, reg).wrapping_add(off);
    c.swc1(f, a);
}

/// `void RiderMetrix_Reset(self)` (0x117540): reset every metrics counter,
/// HUD message, trick and combo, and the two float/flag fields.
pub fn rider_metrix_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low_2(c, SP).wrapping_sub(0x20));
        c.sq(16, sp_2(c).wrapping_add(0x10));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        callee::RIDER_METRIX_RESET_STATS.call(c, 0x0011_7554)?;
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_METRIX_CLEAR_HUD_MESSAGES.call(c, 0x0011_755c)?;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x50;
        c.g[A0][0] = sx(low_2(c, S0).wrapping_add(0xa8));
        call_ptr(c, callee::MEMSET_3E6448, 0x0011_756c)?;
        sw_r(c, S0, 0xf8, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_METRIX_RESET_TRICK.call(c, 0x0011_7578)?;
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_METRIX_RESET_COMBO.call(c, 0x0011_7580)?;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = sx(low_2(c, S0).wrapping_add(0x1b8));
        sw_r(c, S0, 0x1a8, 0);
        c.g[A1][0] = 0;
        swc1_r(c, 0, S0, 0xa4);
        c.g[A2][0] = 0x8;
        call_ptr(c, callee::MEMSET_3E6448, 0x0011_75a0)?;
        c.g[V0][0] = 0x1;
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        sw_r(c, S0, 0x1c0, c.g[V0][0]);
        c.lq(16, sp_2(c).wrapping_add(0x10));
        let t = low_2(c, RA);
        c.g[SP][0] = sx(low_2(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `float RiderMetrix_OnRiderReset(self, a1)` (0x119368): bank the pending
/// score, reset trick/combo, count the reset and return the penalty multiplier
/// applied to the model's speed value.
pub fn rider_metrix_on_rider_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx(low_2(c, SP).wrapping_sub(0x30));
        c.sq(17, sp_2(c).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, sp_2(c).wrapping_add(0x20));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A1][0];
        callee::RIDER_METRIX_BANK_PENDING_SCORE.call(c, 0x0011_9384)?;
        c.g[V1][0] = lw_r(c, S1, 0x1a4);
        c.g[V1][0] = sx(low_2(c, V1).wrapping_add(low_2(c, V0)));
        sw_r(c, S1, 0x1a4, c.g[V1][0]);
        c.g[A0][0] = c.g[S1][0];
        callee::RIDER_METRIX_RESET_TRICK.call(c, 0x0011_9398)?;
        c.g[A0][0] = c.g[S1][0];
        callee::RIDER_METRIX_RESET_COMBO.call(c, 0x0011_93a0)?;
        c.g[V0][0] = lw_r(c, S1, 0x120);
        c.g[V0][0] = sx(low_2(c, V0).wrapping_add(1));
        let first = c.g[S0][0] == 0;
        sw_r(c, S1, 0x120, c.g[V0][0]);
        if first {
            c.cycles += 2;
            c.g[V0][0] = lw_r(c, S1, 0x1ac);
            c.lwc1(1, gpo(c, -0x7a24));
        } else {
            c.cycles += 3;
            c.lwc1(1, gpo(c, -0x7a28));
            c.g[V0][0] = lw_r(c, S1, 0x1ac);
        }
        c.cycles += 7;
        lwc1_r(c, 0, V0, 0x2f8);
        c.mul_s(0, 0, 1);
        c.lq(16, sp_2(c).wrapping_add(0x20));
        c.lq(17, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low_2(c, RA);
        c.g[SP][0] = sx(low_2(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider metrics HUD slots.

/// `int RiderMetrix_FindHUDSlot(RiderMetrix* self, int type)` (0x117a58): the
/// index type itself below 34; otherwise the first free slot at or after 35,
/// else the slot with the smallest value/total ratio (-1 when none).
pub fn rider_metrix_find_hud_slot(c: &mut Cpu) {
    c.cycles += 3;
    let this = abi::arg(c, 0);
    c.g[V0][0] = ((c.g[A1][0] as i64) < 34i64) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = c.g[A1][0];
        abi::ret(c);
        return;
    }
    c.g[A1][0] = Addr(p(this, 0x1b4)).lw(c);
    c.cycles += 6;
    c.g[V1][0] = 0x23;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 2); // f2 = 1.0
    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
    let empty = c.g[V0][0] == 0;
    c.g[A2][0] = u64::MAX;
    if !empty {
        c.cycles += 3;
        c.g[V0][0] = Addr(p(this, 0x1b0)).lw(c);
        c.g[A0][0] = sx(r32(c, V0).wrapping_add(0x1554));
        loop {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, A0)).lw(c);
            c.g[V0][0] = c.g[V0][0] ^ 0x34;
            if c.g[V0][0] == 0 {
                // Free slot: take it.
                c.cycles += 2;
                c.g[V0][0] = c.g[V1][0];
                abi::ret(c);
                return;
            }
            c.cycles += 7;
            c.lwc1(1, p(r32(c, A0), 8));
            c.lwc1(0, p(r32(c, A0), 4));
            c.div_s(1, 1, 0);
            c.c_lt_s(1, 2);
            if !c.fcc() {
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            } else {
                c.cycles += 3;
                c.mov_s(2, 1);
                c.g[A2][0] = c.g[V1][0];
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx(p(r32(c, A0), 0x9c));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0011_7a98);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Rider metrics.

/// `void RiderMetrix_ChainUberGrind(RiderMetrix* self, int points)` (0x119958):
/// count the chained uber, finish the trick with no extra bonus and reset it,
/// then record the points as the trick's value.
pub fn rider_metrix_chain_uber_grind(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        c.g[A1][0] = 0;
        frame.save_fpr(c, 22, 0x40);
        frame.save_fpr(c, 21, 0x38);
        c.g[T0][0] = 0;
        c.lwc1(21, p(c.g[S0][0] as u32, 0x30));
        c.g[T1][0] = 0;
        c.g[V1][0] = lw(c, p(c.g[S0][0] as u32, 0x54));
        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.lwc1(22, p(c.g[S0][0] as u32, 0x24));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        sw(c, p(c.g[S0][0] as u32, 0x54), c.g[V1][0]);
        sw(c, p(c.g[S0][0] as u32, 0x74), c.g[V0][0]);
        call_ptr(c, 0x0011_a228, 0x0011_99b4)?; // RiderMetrix_FinishTrick
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0011_7838, 0x0011_99c0)?; // RiderMetrix_ResetTrick
        c.swc1(21, p(c.g[S0][0] as u32, 0x30));
        c.mov_s(0, 20);
        sw(c, p(c.g[S0][0] as u32, 0x20), c.g[S1][0]);
        c.swc1(22, p(c.g[S0][0] as u32, 0x24));
        sw(c, p(c.g[S0][0] as u32, 0xc), c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    });
}

/// `void RiderMetrix_FinishBoardPress(RiderMetrix* self)` (0x119a38): finish
/// the board press trick (with a zero bonus unless the press value at
/// `+0x30` is negative) and reset it.
pub fn rider_metrix_finish_board_press(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.mtc1(0, 0);
        frame.save(c, S0, 0x20);
        c.g[T1][0] = 1;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 22, 0x40);
        frame.save_fpr(c, 20, 0x30);
        frame.save_fpr(c, 21, 0x38);
        c.lwc1(21, p(c.g[S0][0] as u32, 0x30));
        c.lwc1(22, p(c.g[S0][0] as u32, 0x24));
        c.c_le_s(0, 21);
        let not_negative = c.fcc();
        c.g[S1][0] = lw(c, p(c.g[S0][0] as u32, 0x20));
        if !not_negative {
            c.cycles += 1;
            c.g[T1][0] = 0;
        }
        c.cycles += 22;
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        c.g[T0][0] = 0;
        call_ptr(c, 0x0011_a228, 0x0011_9a94)?;
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0011_7838, 0x0011_9aa0)?;
        c.swc1(21, p(c.g[S0][0] as u32, 0x30));
        c.mov_s(0, 20);
        sw(c, p(c.g[S0][0] as u32, 0x20), c.g[S1][0]);
        c.swc1(22, p(c.g[S0][0] as u32, 0x24));
        sw(c, p(c.g[S0][0] as u32, 0xc), c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Rider tricks and effects.

/// `float RiderMetrix_StartSpinTrick(self, int trick)` (0x119c98): announce a
/// spin trick, and when the rider is spinning start the trick's timer.
pub fn rider_metrix_start_spin_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[T0][0] = 0;
        call_ptr(c, 0x0011_a168, 0x00119cc8)?;
        c.mtc1(0, 20);
        c.lwc1(0, p(r32(c, S0), 0x14));
        c.c_lt_s(20, 0);
        let spinning = c.fcc();
        c.g[V0][0] = 1;
        if spinning {
            c.cycles += 14;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 0);
            Addr(r32(c, S0)).sw(c, c.g[S1][0]);
            c.g[A0][0] = sx(p(r32(c, S0), 0x60));
            c.swc1(20, p(r32(c, S0), 0x34));
            c.g[A1][0] = 0;
            c.swc1(20, p(r32(c, S0), 0x38));
            c.g[A2][0] = 0xc;
            Addr(p(r32(c, S0), 4)).sw(c, 0);
            Addr(p(r32(c, S0), 0xc)).sw(c, 0);
            Addr(p(r32(c, S0), 0x10)).sw(c, 0);
            c.swc1(0, p(r32(c, S0), 0x40));
            Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x003e_6448, 0x00119d18)?;
        }
        c.cycles += 9;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0011_76f8, 0x00119d20)?;
        c.mov_s(0, 20);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderMetrix_OnSplitMarker (0x00001194c0).

/// `RiderMetrix_OnSplitMarker`: record a split time for the rider and update the split HUD.
pub fn rider_metrix_on_split_marker(c: &mut Cpu) {
    guest(c, |c| {
        'b_119588: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S2][0] = c.g[A1][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ac));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[RA][0] = sx(0x001194f8);
            c.pc = t;
            ssx3_ee_rt::funcs::call_indirect(c, t);
            if c.pc != 0x001194f8 {
                return Err(crate::call::Unwound);
            }
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if t {
                break 'b_119588;
            } else {
                c.cycles += 14;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ac));
                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x38));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x3c));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[RA][0] = sx(0x00119520);
                c.pc = t;
                ssx3_ee_rt::funcs::call_indirect(c, t);
                if c.pc != 0x00119520 {
                    return Err(crate::call::Unwound);
                }
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x00238510, 0x00119530)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.g[S1][0];
                if t {
                    break 'b_119588;
                } else {
                    'b_119568: {
                        c.cycles += 10;
                        c.g[AT][0] = 0x40200000;
                        c.mtc1(1, 12);
                        c.g[A2][0] = c.g[S2][0];
                        c.g[A1][0] = 0x29;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00117b88, 0x00119550)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ac));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x874));
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = 0u64;
                        if t {
                            break 'b_119568;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x87c));
                            c.g[A0][0] = (0u64 < c.g[V0][0]) as u64;
                            break 'b_119568;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[A0][0] == 0u64;

                    if t {
                        break 'b_119588;
                    } else {
                        c.cycles += 6;
                        call_ptr(c, 0x0028b180, 0x00119578)?;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ac));
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A2][0] = 0x1;
                        call_ptr(c, 0x002a3ce8, 0x00119588)?;
                        break 'b_119588;
                    }
                }
            }
        }
        c.cycles += 7;
        c.mtc1(0, 0);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Memory helpers (the same widths and sign extension as the original's).

/// `base + off` as a 32-bit address.
#[inline(always)]
pub(crate) fn at_3(c: &Cpu, base: usize, off: u32) -> u32 {
    (c.g[base][0] as u32).wrapping_add(off)
}

/// `lw_3 rd, off(base)`.
#[inline(always)]
pub(crate) fn lw_3(c: &mut Cpu, rd: usize, base: usize, off: u32) {
    let a = at_3(c, base, off);
    c.g[rd][0] = c.lw(a);
}

/// `sw_2 rs, off(base)`.
#[inline(always)]
pub(crate) fn sw_2(c: &mut Cpu, base: usize, off: u32, rs: usize) {
    let a = at_3(c, base, off);
    c.sw(a, c.g[rs][0]);
}

/// `swc1 fs, off(base)`.
#[inline(always)]
pub(crate) fn swc1(c: &mut Cpu, base: usize, off: u32, fs: usize) {
    let a = at_3(c, base, off);
    c.swc1(fs, a);
}

/// `lwc1 fs, off(base)`.
#[inline(always)]
pub(crate) fn lwc1(c: &mut Cpu, fs: usize, base: usize, off: u32) {
    let a = at_3(c, base, off);
    c.lwc1(fs, a);
}

/// `addiu rd, rs, imm` (32-bit, sign-extended).
#[inline(always)]
pub(crate) fn addiu(c: &mut Cpu, rd: usize, rs: usize, imm: u32) {
    c.g[rd][0] = sx((c.g[rs][0] as u32).wrapping_add(imm));
}

/// `addu rd, rd, rs`.
#[inline(always)]
pub(crate) fn add_reg(c: &mut Cpu, rd: usize, rs: usize) {
    c.g[rd][0] = sx((c.g[rd][0] as u32).wrapping_add(c.g[rs][0] as u32));
}

/// `sw_2 zero, off(base)`.
#[inline(always)]
pub(crate) fn sw_zero(c: &mut Cpu, base: usize, off: u32) {
    let a = at_3(c, base, off);
    c.sw(a, 0);
}

/// `lui at_3, hi(bits); mtc1 at_3, fs`: a float constant through `at_3`.
#[inline(always)]
pub(crate) fn fc(c: &mut Cpu, fs: usize, bits: u32) {
    c.g[AT][0] = sx(bits);
    c.f[fs] = bits;
}

/// A `gp`-relative float threshold the HUD compares against.
#[inline(always)]
pub(crate) fn gpf(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

/// The trick HUD's number animation steps toward its target by at most one
/// `1/30` entry per tick (two 60 Hz frames of progress): [`rate_step_60`].
#[inline]
pub(crate) fn hud_step_30(c: &mut Cpu) -> FpuF32 {
    rate_step_60(c, FpuF32::from_f32(1.0f32 / 30.0))
}

/// The trick-name banner's alpha oscillates with `sin(frameCounter * 6°)`:
/// 6° per 60 Hz frame, so the phase per tick is [`rate_step_60`] of it (exact
/// at 60, where nothing is written; the 120/240 factors are 1/2, 1/4).
#[inline]
pub(crate) fn hud_wave_step(c: &mut Cpu, six_degrees: FpuF32) -> FpuF32 {
    rate_step_60(c, six_degrees)
}

/// The boost-level bar's slew step: `1/150` per 60 Hz tick (the game's entry
/// at gp-0x7a64).
#[inline]
pub(crate) fn hud_step_150(c: &mut Cpu) -> FpuF32 {
    rate_step_60(c, FpuF32(0x3bda_740f))
}

/// The band `1 ± step` around a bar's 1.0 target in which the step snaps to it
/// (`above`: `1 + step`, else `1 - step`). At 60 the baked thresholds are used
/// instead, so this is only called on the other branch.
#[inline]
pub(crate) fn hud_band(step: FpuF32, above: bool) -> FpuF32 {
    if above {
        FpuF32::ONE + step
    } else {
        FpuF32::ONE - step
    }
}

/// A HUD record address: slot `n` of the table at_3 `+0x1b0`.
#[inline(always)]
pub(crate) fn slot(n: u32) -> u32 {
    n * mx::SLOT_SIZE
}

/// `RiderMetrix_RemoveHUDMessage(m, id)`.
#[inline(always)]
pub(crate) fn remove_message(c: &mut Cpu, id: u64, ret: u32) -> Flow {
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = id;
    callee::REMOVE_HUD_MESSAGE_2.call(c, ret)
}

/// `RiderMetrix_UpdateTrickHUD(RiderMetrix* m)` (0x117fe0).
///
/// The numbers it shows move toward their targets by at most one step per
/// tick: seven `1/60` `.lit4` entries (which follow the rate), the banner's
/// tracking step (a `1/30` entry, [`hud_step_30`]) and the boost-level bar's
/// step (a `1/150` entry, [`hud_step_150`]); the bands the steps snap inside
/// are `1 ± step` ([`hud_band`]). The banner's alpha oscillates
/// `sin(frameCounter * 6°)`, a phase per tick ([`hud_wave_step`]), and the
/// trick history accumulates `points/1200` per tick ([`per_tick_60`]).
pub fn rider_metrix_update_trick_hud(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S1, 0x80);
        frame.save(c, S0, 0x90);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 27, 0xd8);
        frame.save_fpr(c, 26, 0xd0);
        frame.save_fpr(c, 25, 0xc8);
        frame.save_fpr(c, 24, 0xc0);
        frame.save_fpr(c, 23, 0xb8);
        frame.save_fpr(c, 22, 0xb0);
        frame.save_fpr(c, 21, 0xa8);
        frame.save_fpr(c, 20, 0xa0);
        lw_3(c, V0, S1, mx::SLOTS);
        let has_hud = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x90);
        if has_hud {
            // ---------------------------------------------------------------
            // The pending trick's points and the score-tier number.
            {
                c.cycles += 5;
                callee::GET_ROUNDED_TRICK_POINTS.call(c, 0x0011_8040)?;
                c.g[S0][0] = c.g[V0][0];
                let positive = (c.g[S0][0] as i64) > 0;
                c.g[A0][0] = c.g[S1][0];
                if positive {
                    c.cycles += 15;
                    c.g[A1][0] = c.g[S0][0];
                    callee::GET_SCORE_TIER.call(c, 0x0011_8054)?;
                    fc(c, 12, 0xbf80_0000);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = 1;
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A3][0] = c.g[V0][0];
                    callee::SHOW_HUD_NUMBER_2.call(c, 0x0011_8070)?;
                    call_ptr(c, table::AUDIO_SYSTEM, 0x0011_8078)?;
                    lw_3(c, A1, S1, mx::RIDER);
                    c.g[A0][0] = c.g[V0][0];
                    // (The points again: the calls above clobbered a2.)
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, table::AUDIO_TRICK_TICK, 0x0011_8088)?;
                }
            }
            // ---------------------------------------------------------------
            // The boost meter's "delayed number" (message 0xb) while the
            // board is in its boost mode.
            'b_1180f0: {
                'b_1180e4: {
                    c.cycles += 5;
                    lw_3(c, A0, S1, mx::RIDER);
                    callee::GET_SUB_STATE.call(c, 0x0011_8090)?;
                    c.g[V1][0] = 8;
                    if c.g[V0][0] != c.g[V1][0] {
                        c.g[A0][0] = c.g[S1][0];
                        break 'b_1180e4;
                    } else {
                        'b_1180e0: {
                            c.cycles += 6;
                            lw_3(c, V1, S1, mx::RIDER);
                            lw_3(c, V1, V1, rider::OWNER);
                            lw_3(c, V0, V1, rider::OWNER_MODE);
                            c.g[V0][0] ^= 0x3;
                            let t = c.g[V0][0] == 0;
                            c.g[A1][0] = 0xb;
                            if t {
                                break 'b_1180e0;
                            } else {
                                c.cycles += 11;
                                lwc1(c, 12, V1, rider::OWNER_METER);
                                fc(c, 0, 0x3f80_0000);
                                c.g[A2][0] = 0;
                                lw_3(c, A0, S1, mx::SLOTS);
                                c.g[A3][0] = 0;
                                c.min_s(12, 12, 0);
                                addiu(c, A0, A0, slot(11));
                                callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_80d8)?;
                                c.g[A0][0] = c.g[S1][0];
                                break 'b_1180f0;
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S1][0];
                        break 'b_1180e4;
                    }
                }
                c.cycles += 3;
                remove_message(c, 0xb, 0x0011_80ec)?;
                c.g[A0][0] = c.g[S1][0];
                break 'b_1180f0;
            }
            // ---------------------------------------------------------------
            // The trick messages. `'b_1188fc` is the "HUD is up" merge point;
            // `break 'b_1188fc` skips to the next section.
            'b_1188fc: {
                'b_1188f8: {
                    'b_11889c: {
                        'b_118880: {
                            // Fixed messages 0x10..0x17 (the eight trick names
                            // of the running combo): clear them all.
                            'b_118194: {
                                c.cycles += 28;
                                for (i, ret) in [
                                    0x0011_80f8u32,
                                    0x0011_8104,
                                    0x0011_8110,
                                    0x0011_811c,
                                    0x0011_8128,
                                    0x0011_8134,
                                    0x0011_8140,
                                    0x0011_814c,
                                ]
                                .into_iter()
                                .enumerate()
                                {
                                    remove_message(c, 0x10 + i as u64, ret)?;
                                }
                                lw_3(c, A0, S1, mx::RIDER);
                                callee::GET_SUB_STATE.call(c, 0x0011_8154)?;
                                c.g[V1][0] = 2;
                                if c.g[V0][0] == c.g[V1][0] {
                                    lw_3(c, A0, S1, mx::RIDER);
                                    break 'b_118194;
                                } else {
                                    // Sub-state 2: the "replay" messages 0xc and
                                    // 0xd are live, otherwise clear them.
                                    'b_118190: {
                                        c.cycles += 5;
                                        lw_3(c, A0, S1, mx::RIDER);
                                        callee::GET_SUB_STATE.call(c, 0x0011_8168)?;
                                        c.g[V1][0] = 5;
                                        let replay = c.g[V0][0] == c.g[V1][0];
                                        c.g[A0][0] = c.g[S1][0];
                                        if replay {
                                            break 'b_118190;
                                        } else {
                                            c.cycles += 7;
                                            remove_message(c, 0xc, 0x0011_817c)?;
                                            c.g[A0][0] = c.g[S1][0];
                                            remove_message(c, 0xd, 0x0011_8188)?;
                                            c.g[A0][0] = c.g[S1][0];
                                            break 'b_118880;
                                        }
                                    }
                                    c.cycles += 1;
                                    lw_3(c, A0, S1, mx::RIDER);
                                    break 'b_118194;
                                }
                            }
                            // The HUD angle: slot 0xd while in sub-state 2 (the
                            // prewind), else the board's own method.
                            'b_1181d4: {
                                'b_1181c0: {
                                    c.cycles += 5;
                                    sw_zero(c, SP, 0);
                                    callee::GET_SUB_STATE.call(c, 0x0011_819c)?;
                                    c.g[V1][0] = 2;
                                    let t = c.g[V0][0] != c.g[V1][0];
                                    lw_3(c, V0, S1, mx::RIDER);
                                    if t {
                                        break 'b_1181c0;
                                    } else {
                                        c.cycles += 6;
                                        c.g[A1][0] = c.g[SP][0];
                                        lw_3(c, A0, V0, rider::OWNER);
                                        addiu(c, A0, A0, 0x1f0);
                                        callee::GET_HUD_ANGLE.call(c, 0x0011_81b8)?;
                                        c.mov_s(27, 0);
                                        break 'b_1181d4;
                                    }
                                }
                                c.cycles += 5;
                                c.g[A1][0] = c.g[SP][0];
                                lw_3(c, A0, V0, rider::OWNER);
                                addiu(c, A0, A0, 0x230);
                                call_ptr(c, table::BOARD_DONE, 0x0011_81d0)?;
                                c.mov_s(27, 0);
                                break 'b_1181d4;
                            }
                            // ---------------------------------------------------
                            // The trick-name banner (slot 13, id 0xd) fades in
                            // and out with the HUD-effect timer in f12.
                            'b_11838c: {
                                'b_1181f0: {
                                    c.cycles += 3;
                                    lw_3(c, V0, SP, 0);
                                    let t = c.g[V0][0] == 0;
                                    c.g[A0][0] = c.g[S1][0];
                                    if t {
                                        break 'b_1181f0;
                                    } else {
                                        c.cycles += 4;
                                        remove_message(c, 0xd, 0x0011_81e8)?;
                                        c.g[A0][0] = c.g[S1][0];
                                        break 'b_11838c;
                                    }
                                }
                                // The timer: a triangle wave over the frame
                                // counter, scaled by two gp constants.
                                'b_118240: {
                                    c.cycles += 17;
                                    callee::GET_FRAME_COUNTER.call(c, 0x0011_81f8)?;
                                    c.mtc1(2, 0);
                                    c.cvt_s_w(0, 0);
                                    c.lwc1(12, gpf(c, 0x7ae0));
                                    // 6° per 60 Hz frame: the phase per tick
                                    // follows the rate.
                                    let six_degrees = FpuF32(c.f[12]);
                                    c.f[12] = hud_wave_step(c, six_degrees).0;
                                    c.mul_s(12, 0, 12);
                                    call_ptr(c, table::SIN, 0x0011_820c)?;
                                    c.lwc1(1, gpf(c, 0x7adc));
                                    c.lwc1(2, gpf(c, 0x7ad8));
                                    c.mul_s(1, 27, 1);
                                    c.mul_s(0, 0, 2);
                                    c.mtc1(0, 3);
                                    c.add_s(12, 1, 0);
                                    c.c_lt_s(12, 3);
                                    let t = !c.fcc();
                                    lw_3(c, V1, S1, mx::SLOTS);
                                    if t {
                                        break 'b_118240;
                                    } else {
                                        c.cycles += 3;
                                        fc(c, 0, 0x3f80_0000);
                                        c.add_s(12, 12, 0);
                                        break 'b_118240;
                                    }
                                }
                                'b_118378: {
                                    c.cycles += 4;
                                    lw_3(c, V0, V1, slot(13));
                                    c.g[V0][0] ^= 0x34;
                                    if c.g[V0][0] == 0 {
                                        lw_3(c, A0, S1, mx::SLOTS);
                                        break 'b_118378;
                                    } else {
                                        'b_118374: {
                                            'b_1182d4: {
                                                c.cycles += 9;
                                                lwc1(c, 1, V1, slot(13) + 8);
                                                lwc1(c, 0, V1, slot(13) + 4);
                                                fc(c, 2, 0x3f00_0000);
                                                c.div_s(3, 1, 0);
                                                c.sub_s(1, 3, 12);
                                                c.c_lt_s(2, 1);
                                                let t = !c.fcc();
                                                if t {
                                                    break 'b_1182d4;
                                                } else {
                                                    'b_1182b0: {
                                                        'b_1182a0: {
                                                            c.cycles += 9;
                                                            fc(c, 1, 0x3f80_0000);
                                                            c.lwc1(2, gpf(c, 0x7ad4));
                                                            // The 1/30 entry is a
                                                            // per-tick step.
                                                            if crate::tick::rate(c) != 60 {
                                                                let step = hud_step_30(c);
                                                                c.f[2] = step.0;
                                                            }
                                                            c.sub_s(1, 3, 1);
                                                            c.add_s(0, 12, 2);
                                                            c.c_lt_s(0, 1);
                                                            if !c.fcc() {
                                                                c.sub_s(0, 12, 2);
                                                                break 'b_1182a0;
                                                            } else {
                                                                c.cycles += 2;
                                                                c.sub_s(12, 1, 2);
                                                                break 'b_1182b0;
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.c_lt_s(1, 0);
                                                        if c.fcc() {
                                                            c.add_s(12, 1, 2);
                                                            break 'b_1182b0;
                                                        } else {
                                                            break 'b_1182b0;
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.mtc1(0, 0);
                                                    c.c_lt_s(12, 0);
                                                    if !c.fcc() {
                                                        lw_3(c, A0, S1, mx::SLOTS);
                                                        break 'b_118378;
                                                    } else {
                                                        c.cycles += 4;
                                                        fc(c, 0, 0x3f80_0000);
                                                        c.add_s(12, 12, 0);
                                                        break 'b_118374;
                                                    }
                                                }
                                            }
                                            'b_118344: {
                                                c.cycles += 5;
                                                fc(c, 0, 0xbf00_0000);
                                                c.c_lt_s(1, 0);
                                                let t = !c.fcc();
                                                if t {
                                                    break 'b_118344;
                                                } else {
                                                    'b_118324: {
                                                        'b_118314: {
                                                            c.cycles += 9;
                                                            fc(c, 1, 0x3f80_0000);
                                                            c.lwc1(2, gpf(c, 0x7ad0));
                                                            if crate::tick::rate(c) != 60 {
                                                                let step = hud_step_30(c);
                                                                c.f[2] = step.0;
                                                            }
                                                            c.add_s(1, 3, 1);
                                                            c.add_s(0, 12, 2);
                                                            c.c_lt_s(0, 1);
                                                            if !c.fcc() {
                                                                c.sub_s(0, 12, 2);
                                                                break 'b_118314;
                                                            } else {
                                                                c.cycles += 2;
                                                                c.sub_s(12, 1, 2);
                                                                break 'b_118324;
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.c_lt_s(1, 0);
                                                        if c.fcc() {
                                                            c.add_s(12, 1, 2);
                                                            break 'b_118324;
                                                        } else {
                                                            break 'b_118324;
                                                        }
                                                    }
                                                    c.cycles += 6;
                                                    fc(c, 0, 0x3f80_0000);
                                                    c.c_lt_s(0, 12);
                                                    if c.fcc() {
                                                        c.sub_s(12, 12, 0);
                                                        break 'b_118374;
                                                    } else {
                                                        c.cycles += 2;
                                                        lw_3(c, A0, S1, mx::SLOTS);
                                                        break 'b_118378;
                                                    }
                                                }
                                            }
                                            'b_118364: {
                                                c.cycles += 6;
                                                c.lwc1(1, gpf(c, 0x7acc));
                                                if crate::tick::rate(c) != 60 {
                                                    let step = hud_step_30(c);
                                                    c.f[1] = step.0;
                                                }
                                                c.add_s(0, 12, 1);
                                                c.c_lt_s(0, 3);
                                                if !c.fcc() {
                                                    c.sub_s(0, 12, 1);
                                                    break 'b_118364;
                                                } else {
                                                    c.cycles += 2;
                                                    c.sub_s(12, 3, 1);
                                                    break 'b_118374;
                                                }
                                            }
                                            c.cycles += 4;
                                            c.c_lt_s(3, 0);
                                            if c.fcc() {
                                                c.add_s(12, 3, 1);
                                                break 'b_118374;
                                            } else {
                                                break 'b_118374;
                                            }
                                        }
                                        c.cycles += 1;
                                        lw_3(c, A0, S1, mx::SLOTS);
                                        break 'b_118378;
                                    }
                                }
                                c.cycles += 5;
                                c.g[A1][0] = 0xd;
                                c.g[A2][0] = 0;
                                c.g[A3][0] = 0;
                                addiu(c, A0, A0, slot(13));
                                callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_838c)?;
                                break 'b_11838c;
                            }
                            // ---------------------------------------------------
                            // Sub-state 2 / replay: the combo counter's
                            // messages 0xc and 0xd.
                            'b_1183dc: {
                                c.cycles += 5;
                                lw_3(c, A0, S1, mx::RIDER);
                                callee::GET_SUB_STATE.call(c, 0x0011_8394)?;
                                c.g[V1][0] = 2;
                                if c.g[V0][0] != c.g[V1][0] {
                                    lw_3(c, V0, S1, mx::RIDER);
                                    break 'b_1183dc;
                                } else {
                                    'b_1183bc: {
                                        c.cycles += 3;
                                        lw_3(c, V0, SP, 0);
                                        let t = c.g[V0][0] == 0;
                                        c.g[A0][0] = c.g[S1][0];
                                        if t {
                                            break 'b_1183bc;
                                        } else {
                                            c.cycles += 4;
                                            remove_message(c, 0xc, 0x0011_83b4)?;
                                            lw_3(c, V1, S1, mx::SLOTS);
                                            break 'b_1188fc;
                                        }
                                    }
                                    c.cycles += 8;
                                    fc(c, 12, 0xbf80_0000);
                                    c.g[A1][0] = 0xc;
                                    c.g[A2][0] = 0;
                                    c.g[A3][0] = 0;
                                    callee::SHOW_HUD_NUMBER_2.call(c, 0x0011_83d4)?;
                                    lw_3(c, V1, S1, mx::SLOTS);
                                    break 'b_1188fc;
                                }
                            }
                            // The board-wake effect (vtable `rider+0x230`): clear
                            // messages 0xc/0xd when it reports a wake.
                            'b_11841c: {
                                c.cycles += 8;
                                c.g[A1][0] = c.g[SP][0] | 0x4;
                                sw_zero(c, SP, 0x4);
                                lw_3(c, A0, V0, rider::OWNER);
                                addiu(c, A0, A0, 0x230);
                                call_ptr(c, table::BOARD_WAKE_CLEAR, 0x0011_83f0)?;
                                lw_3(c, V0, SP, 0x4);
                                let t = c.g[V0][0] == 0;
                                c.mov_s(21, 0);
                                if t {
                                    break 'b_11841c;
                                } else {
                                    c.cycles += 8;
                                    c.g[A0][0] = c.g[S1][0];
                                    remove_message(c, 0xc, 0x0011_8408)?;
                                    c.g[A0][0] = c.g[S1][0];
                                    remove_message(c, 0xd, 0x0011_8414)?;
                                    c.g[A0][0] = c.g[S1][0];
                                    break 'b_118880;
                                }
                            }
                            // ---------------------------------------------------
                            // The wake/difficulty bars: pick the five message
                            // ids for the current peak-air value (f21).
                            'b_118534: {
                                'b_118468: {
                                    c.cycles += 17;
                                    fc(c, 12, 0xbf80_0000);
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A1][0] = 0xc;
                                    c.g[A2][0] = 0;
                                    c.g[A3][0] = 0;
                                    c.g[S2][0] = 0x34;
                                    callee::SHOW_HUD_NUMBER_2.call(c, 0x0011_843c)?;
                                    c.g[S3][0] = 0x34;
                                    c.lwc1(1, gpf(c, 0x7ac8));
                                    c.abs_s(0, 21);
                                    c.mov_s(25, 0);
                                    c.g[S4][0] = 0x34;
                                    c.c_lt_s(1, 0);
                                    let t = !c.fcc();
                                    c.g[S5][0] = 0x34;
                                    if t {
                                        break 'b_118468;
                                    } else {
                                        c.cycles += 2;
                                        c.g[S0][0] = 0x10;
                                        break 'b_118534;
                                    }
                                }
                                'b_118490: {
                                    c.cycles += 5;
                                    c.lwc1(0, gpf(c, 0x7ac4));
                                    c.c_lt_s(0, 21);
                                    let t = !c.fcc();
                                    c.g[S0][0] = 0x17;
                                    if t {
                                        break 'b_118490;
                                    } else {
                                        c.cycles += 5;
                                        c.g[S2][0] = 0x15;
                                        c.g[S3][0] = 0x16;
                                        c.g[S4][0] = 0x11;
                                        c.g[S5][0] = 0x10;
                                        break 'b_118534;
                                    }
                                }
                                c.cycles += 5;
                                c.lwc1(0, gpf(c, 0x7ac0));
                                c.c_lt_s(0, 21);
                                let t = c.fcc();
                                c.g[S0][0] = 0x16;
                                if t {
                                    break 'b_118534;
                                } else {
                                    'b_1184cc: {
                                        c.cycles += 5;
                                        c.lwc1(0, gpf(c, 0x7abc));
                                        c.c_lt_s(0, 21);
                                        let t = !c.fcc();
                                        c.g[S0][0] = 0x15;
                                        if t {
                                            break 'b_1184cc;
                                        } else {
                                            c.cycles += 5;
                                            c.g[S2][0] = 0x17;
                                            c.g[S3][0] = 0x16;
                                            c.g[S4][0] = 0x13;
                                            c.g[S5][0] = 0x14;
                                            break 'b_118534;
                                        }
                                    }
                                    'b_1184f4: {
                                        c.cycles += 5;
                                        c.lwc1(0, gpf(c, 0x7ab8));
                                        c.c_lt_s(21, 0);
                                        let t = !c.fcc();
                                        c.g[S0][0] = 0x11;
                                        if t {
                                            break 'b_1184f4;
                                        } else {
                                            c.cycles += 5;
                                            c.g[S2][0] = 0x13;
                                            c.g[S3][0] = 0x12;
                                            c.g[S4][0] = 0x17;
                                            c.g[S5][0] = 0x10;
                                            break 'b_118534;
                                        }
                                    }
                                    c.cycles += 5;
                                    c.lwc1(0, gpf(c, 0x7ab4));
                                    c.c_lt_s(21, 0);
                                    let t = c.fcc();
                                    c.g[S0][0] = 0x12;
                                    if t {
                                        break 'b_118534;
                                    } else {
                                        'b_118530: {
                                            c.cycles += 5;
                                            c.lwc1(0, gpf(c, 0x7ab0));
                                            c.c_lt_s(21, 0);
                                            let t = !c.fcc();
                                            c.g[S0][0] = 0x13;
                                            if t {
                                                break 'b_118530;
                                            } else {
                                                c.cycles += 5;
                                                c.g[S2][0] = 0x11;
                                                c.g[S3][0] = 0x12;
                                                c.g[S4][0] = 0x15;
                                                c.g[S5][0] = 0x14;
                                                break 'b_118534;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[S0][0] = 0x14;
                                        break 'b_118534;
                                    }
                                }
                            }
                            // The five bars: slots S0, S2..S5 of the message
                            // table, at_3 the offset the board reports.
                            'b_1185b0: {
                                c.cycles += 24;
                                lw_3(c, V0, S1, mx::RIDER);
                                c.g[S6][0] = 0x9c;
                                fc(c, 26, 0x3f80_0000);
                                c.g[S7][0] = 0x34;
                                lw_3(c, A0, V0, rider::OWNER);
                                addiu(c, A0, A0, 0x230);
                                call_ptr(c, table::BOARD_WAKE_A, 0x0011_8554)?;
                                lw_3(c, V0, S1, mx::RIDER);
                                c.mov_s(24, 0);
                                lw_3(c, A0, V0, rider::OWNER);
                                addiu(c, A0, A0, 0x230);
                                call_ptr(c, table::BOARD_WAKE_B, 0x0011_8568)?;
                                c.mult(0, 2, 16, 22);
                                lw_3(c, A0, S1, mx::SLOTS);
                                c.mov_s(23, 0);
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A2][0] = 0;
                                c.mov_s(12, 26);
                                c.g[A3][0] = 0;
                                add_reg(c, A0, V0);
                                callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_858c)?;
                                let t = c.g[S2][0] == c.g[S7][0];
                                c.mult(0, 2, 18, 22);
                                if t {
                                    break 'b_1185b0;
                                } else {
                                    c.cycles += 7;
                                    lw_3(c, A0, S1, mx::SLOTS);
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[A2][0] = 0;
                                    c.mov_s(12, 24);
                                    c.g[A3][0] = 1;
                                    add_reg(c, A0, V0);
                                    callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_85b0)?;
                                    break 'b_1185b0;
                                }
                            }
                            'b_1185d4: {
                                c.cycles += 2;
                                let t = c.g[S3][0] == c.g[S7][0];
                                c.mult(0, 2, 19, 22);
                                if t {
                                    break 'b_1185d4;
                                } else {
                                    c.cycles += 7;
                                    lw_3(c, A0, S1, mx::SLOTS);
                                    c.g[A1][0] = c.g[S3][0];
                                    c.g[A2][0] = 0;
                                    c.mov_s(12, 24);
                                    c.g[A3][0] = 1;
                                    add_reg(c, A0, V0);
                                    callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_85d4)?;
                                    break 'b_1185d4;
                                }
                            }
                            'b_1185f8: {
                                c.cycles += 2;
                                let t = c.g[S4][0] == c.g[S7][0];
                                c.mult(0, 2, 20, 22);
                                if t {
                                    break 'b_1185f8;
                                } else {
                                    c.cycles += 7;
                                    lw_3(c, A0, S1, mx::SLOTS);
                                    c.g[A1][0] = c.g[S4][0];
                                    c.g[A2][0] = 0;
                                    c.mov_s(12, 23);
                                    c.g[A3][0] = 2;
                                    add_reg(c, A0, V0);
                                    callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_85f8)?;
                                    break 'b_1185f8;
                                }
                            }
                            'b_11861c: {
                                c.cycles += 2;
                                let t = c.g[S5][0] == c.g[S7][0];
                                c.mult(0, 2, 21, 22);
                                if t {
                                    break 'b_11861c;
                                } else {
                                    c.cycles += 7;
                                    lw_3(c, A0, S1, mx::SLOTS);
                                    c.g[A1][0] = c.g[S5][0];
                                    c.g[A2][0] = 0;
                                    c.mov_s(12, 23);
                                    c.g[A3][0] = 2;
                                    add_reg(c, A0, V0);
                                    callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_861c)?;
                                    break 'b_11861c;
                                }
                            }
                            // The fourth wake counter (`slot 14`).
                            'b_118634: {
                                c.cycles += 5;
                                lw_3(c, V1, S1, mx::SLOTS);
                                lw_3(c, V0, V1, slot(14));
                                c.g[V0][0] ^= 0x34;
                                let t = c.g[V0][0] == 0;
                                c.g[S2][0] = 0;
                                if t {
                                    break 'b_118634;
                                } else {
                                    c.cycles += 1;
                                    lw_3(c, S2, V1, slot(14) + 0xc);
                                    break 'b_118634;
                                }
                            }
                            // ---------------------------------------------------
                            // The four bar/percentage messages, chosen from the
                            // wake peak (f25 = |f21|).
                            c.cycles += 3;
                            lw_3(c, V0, SP, 0);
                            let t = c.g[V0][0] != 0;
                            c.g[S0][0] = 0;
                            if t {
                                break 'b_1188f8;
                            } else {
                                'b_118670: {
                                    c.cycles += 12;
                                    c.sub_s(2, 21, 27);
                                    c.lwc1(0, gpf(c, 0x7aac));
                                    fc(c, 1, 0x3f00_0000);
                                    c.mul_s(0, 2, 0);
                                    c.add_s(0, 0, 1);
                                    c.cvt_w_s(1, 0);
                                    c.cvt_s_w(1, 1);
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.sub_s(1, 1, 26);
                                        break 'b_118670;
                                    } else {
                                        break 'b_118670;
                                    }
                                }
                                'b_11886c: {
                                    'b_118714: {
                                        c.cycles += 10;
                                        c.lwc1(0, gpf(c, 0x7aa8));
                                        c.lwc1(22, gpf(c, 0x7aa4));
                                        c.mul_s(0, 1, 0);
                                        c.mtc1(0, 20);
                                        c.sub_s(2, 2, 0);
                                        c.abs_s(1, 2);
                                        c.c_lt_s(22, 1);
                                        let t = !c.fcc();
                                        c.g[A2][0] = 0x45_0000;
                                        if t {
                                            break 'b_118714;
                                        } else {
                                            'b_1186dc: {
                                                'b_1186d4: {
                                                    c.cycles += 11;
                                                    fc(c, 12, 0xbf80_0000);
                                                    addiu(c, A2, A2, 0x7820);
                                                    c.g[A0][0] = c.g[S1][0];
                                                    c.g[A1][0] = 0xe;
                                                    c.g[A3][0] = 1;
                                                    callee::SHOW_HUD_TEXT.call(c, 0x0011_86b4)?;
                                                    c.c_lt_s(22, 25);
                                                    let t = c.fcc();
                                                    c.g[S0][0] = 1;
                                                    if t {
                                                        break 'b_1186d4;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.lwc1(0, gpf(c, 0x7aa0));
                                                        c.c_lt_s(25, 0);
                                                        let t = !c.fcc();
                                                        if t {
                                                            break 'b_1186dc;
                                                        } else {
                                                            break 'b_1186d4;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                c.mov_s(20, 26);
                                                break 'b_11886c;
                                            }
                                            c.cycles += 6;
                                            c.lwc1(0, gpf(c, 0x7a9c));
                                            fc(c, 20, 0x4000_0000);
                                            c.c_lt_s(0, 25);
                                            let t = !c.fcc();
                                            if t {
                                                break 'b_11886c;
                                            } else {
                                                c.cycles += 4;
                                                c.lwc1(0, gpf(c, 0x7a98));
                                                c.c_lt_s(25, 0);
                                                let t = !c.fcc();
                                                if t {
                                                    break 'b_11886c;
                                                } else {
                                                    c.cycles += 4;
                                                    fc(c, 20, 0x40a0_0000);
                                                    break 'b_11886c;
                                                }
                                            }
                                        }
                                    }
                                    'b_1187e0: {
                                        c.cycles += 4;
                                        c.lwc1(0, gpf(c, 0x7a94));
                                        c.c_lt_s(0, 1);
                                        let t = !c.fcc();
                                        if t {
                                            break 'b_1187e0;
                                        } else {
                                            'b_1187a0: {
                                                'b_118760: {
                                                    'b_118744: {
                                                        c.cycles += 4;
                                                        c.mul_s(1, 2, 21);
                                                        c.c_lt_s(20, 1);
                                                        let t = !c.fcc();
                                                        if t {
                                                            break 'b_118744;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.lwc1(0, gpf(c, 0x7a90));
                                                            c.c_lt_s(0, 25);
                                                            let t = c.fcc();
                                                            if t {
                                                                break 'b_118760;
                                                            } else {
                                                                break 'b_118744;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 3;
                                                    c.c_lt_s(1, 20);
                                                    let t = !c.fcc();
                                                    if t {
                                                        break 'b_1187a0;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.lwc1(0, gpf(c, 0x7a8c));
                                                        c.c_lt_s(25, 0);
                                                        let t = !c.fcc();
                                                        if t {
                                                            break 'b_1187a0;
                                                        } else {
                                                            break 'b_118760;
                                                        }
                                                    }
                                                }
                                                c.cycles += 5;
                                                c.lwc1(0, gpf(c, 0x7a88));
                                                c.c_lt_s(0, 23);
                                                let t = !c.fcc();
                                                c.g[A2][0] = 0x45_0000;
                                                if t {
                                                    break 'b_11886c;
                                                } else {
                                                    c.cycles += 11;
                                                    fc(c, 12, 0xbf80_0000);
                                                    addiu(c, A2, A2, 0x7830);
                                                    c.g[A0][0] = c.g[S1][0];
                                                    c.g[A1][0] = 0xe;
                                                    c.g[A3][0] = 2;
                                                    callee::SHOW_HUD_TEXT.call(c, 0x0011_8790)?;
                                                    fc(c, 20, 0x4040_0000);
                                                    c.g[S0][0] = 2;
                                                    break 'b_11886c;
                                                }
                                            }
                                            c.cycles += 5;
                                            c.lwc1(0, gpf(c, 0x7a84));
                                            c.c_lt_s(0, 24);
                                            let t = !c.fcc();
                                            c.g[A2][0] = 0x45_0000;
                                            if t {
                                                break 'b_11886c;
                                            } else {
                                                c.cycles += 11;
                                                fc(c, 12, 0xbf80_0000);
                                                addiu(c, A2, A2, 0x7840);
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[A1][0] = 0xe;
                                                c.g[A3][0] = 3;
                                                callee::SHOW_HUD_TEXT.call(c, 0x0011_87d0)?;
                                                fc(c, 20, 0x4000_0000);
                                                c.g[S0][0] = 3;
                                                break 'b_11886c;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.lwc1(0, gpf(c, 0x7a80));
                                    c.c_lt_s(0, 1);
                                    let t = !c.fcc();
                                    if t {
                                        break 'b_11886c;
                                    } else {
                                        'b_118844: {
                                            'b_11881c: {
                                                c.cycles += 6;
                                                c.lwc1(0, gpf(c, 0x7a7c));
                                                c.abs_s(1, 27);
                                                c.c_lt_s(1, 0);
                                                let t = c.fcc();
                                                c.g[A2][0] = 0x45_0000;
                                                if t {
                                                    break 'b_11881c;
                                                } else {
                                                    c.cycles += 5;
                                                    c.lwc1(0, gpf(c, 0x7a78));
                                                    c.c_lt_s(0, 1);
                                                    if !c.fcc() {
                                                        c.g[A2][0] = 0x45_0000;
                                                        break 'b_118844;
                                                    } else {
                                                        break 'b_11881c;
                                                    }
                                                }
                                            }
                                            c.cycles += 10;
                                            fc(c, 12, 0xbf80_0000);
                                            addiu(c, A2, A2, 0x7850);
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[A1][0] = 0xe;
                                            callee::SHOW_HUD_TEXT.call(c, 0x0011_8838)?;
                                            c.mov_s(20, 26);
                                            c.g[S0][0] = 4;
                                            break 'b_11886c;
                                        }
                                        c.cycles += 10;
                                        fc(c, 12, 0xbf80_0000);
                                        addiu(c, A2, A2, 0x7860);
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[A1][0] = 0xe;
                                        fc(c, 20, 0x4000_0000);
                                        c.g[A3][0] = 5;
                                        callee::SHOW_HUD_TEXT.call(c, 0x0011_8868)?;
                                        c.g[S0][0] = 5;
                                        break 'b_11886c;
                                    }
                                }
                                c.cycles += 5;
                                c.mtc1(0, 21);
                                c.c_eq_s(20, 21);
                                let t = !c.fcc();
                                c.g[A0][0] = c.g[S1][0];
                                if t {
                                    break 'b_11889c;
                                } else {
                                    break 'b_118880;
                                }
                            }
                        }
                        c.cycles += 7;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = 0xf;
                        callee::REMOVE_HUD_MESSAGE_2.call(c, 0x0011_8888)?;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = 0xe;
                        callee::REMOVE_HUD_MESSAGE_2.call(c, 0x0011_8894)?;
                        lw_3(c, V1, S1, mx::SLOTS);
                        break 'b_1188fc;
                    }
                    // -------------------------------------------------------
                    // The trick history: its number (message 0xf), the score
                    // scale, then the combo's bonus points (slot 2).
                    c.cycles += 13;
                    c.cvt_w_s(0, 20);
                    c.mfc1(A2, 0);
                    fc(c, 12, 0xbf80_0000);
                    c.g[A1][0] = 0xf;
                    c.g[A3][0] = 0;
                    callee::SHOW_HUD_NUMBER_2.call(c, 0x0011_88b8)?;
                    c.lwc1(0, gpf(c, 0x7a74));
                    lwc1(c, 1, S1, 0x1c);
                    c.mul_s(0, 20, 0);
                    // `+= points/1200` per tick: the trick history counts up
                    // in real time.
                    per_tick_60(c, 0);
                    c.add_s(1, 1, 0);
                    let t = c.g[S0][0] == c.g[S2][0];
                    swc1(c, S1, 0x1c, 1);
                    if t {
                        break 'b_1188f8;
                    } else {
                        c.cycles += 10;
                        c.g[A0][0] = c.g[S1][0];
                        callee::GET_TRICK_BONUS.call(c, 0x0011_88d8)?;
                        lw_3(c, A2, S1, mx::BONUS);
                        c.mov_s(12, 21);
                        lw_3(c, A0, S1, mx::SLOTS);
                        c.g[A3][0] = c.g[V0][0];
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A1][0] = 2;
                        addiu(c, A0, A0, slot(2));
                        callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_88f8)?;
                        break 'b_1188f8;
                    }
                }
                c.cycles += 1;
                lw_3(c, V1, S1, mx::SLOTS);
                break 'b_1188fc;
            }
            // ---------------------------------------------------------------
            // The boost meter bar: slot 5's fraction (f4), slot 6's value.
            'b_11898c: {
                'b_118918: {
                    c.cycles += 4;
                    lw_3(c, V0, V1, slot(5));
                    c.g[V0][0] ^= 0x34;
                    let t = c.g[V0][0] != 0;
                    c.g[A1][0] = c.g[V1][0];
                    if t {
                        break 'b_118918;
                    } else {
                        c.cycles += 3;
                        lw_3(c, V0, S1, mx::RIDER);
                        lwc1(c, 4, V0, rider::BOOST_STRENGTH);
                        break 'b_11898c;
                    }
                }
                'b_11893c: {
                    'b_118938: {
                        c.cycles += 5;
                        lw_3(c, V0, S1, mx::RIDER);
                        c.g[A0][0] = 3;
                        lw_3(c, V1, V0, rider::BOOST_MODE);
                        let t = c.g[V1][0] == c.g[A0][0];
                        lwc1(c, 1, V0, rider::BOOST_STRENGTH);
                        if t {
                            break 'b_118938;
                        } else {
                            c.cycles += 3;
                            lwc1(c, 0, S1, mx::TRICK_BOOST);
                            c.add_s(3, 1, 0);
                            break 'b_11893c;
                        }
                    }
                    c.cycles += 1;
                    c.mov_s(3, 1);
                    break 'b_11893c;
                }
                'b_118988: {
                    'b_118974: {
                        c.cycles += 12;
                        lwc1(c, 0, A1, slot(5) + 4);
                        lwc1(c, 2, A1, slot(5) + 8);
                        fc(c, 1, 0x3f80_0000);
                        c.div_s(2, 2, 0);
                        c.lwc1(4, gpf(c, 0x7a70));
                        c.min_s(1, 3, 1);
                        c.add_s(0, 1, 4);
                        c.c_lt_s(0, 2);
                        if !c.fcc() {
                            c.sub_s(0, 1, 4);
                            break 'b_118974;
                        } else {
                            c.cycles += 2;
                            c.sub_s(12, 2, 4);
                            break 'b_118988;
                        }
                    }
                    c.cycles += 4;
                    c.c_lt_s(2, 0);
                    let t = !c.fcc();
                    c.mov_s(12, 1);
                    if t {
                        break 'b_118988;
                    } else {
                        c.cycles += 1;
                        c.add_s(12, 2, 4);
                        break 'b_118988;
                    }
                }
                c.cycles += 1;
                c.mov_s(4, 12);
                break 'b_11898c;
            }
            'b_1189f0: {
                'b_1189a8: {
                    c.cycles += 4;
                    lw_3(c, V0, A1, slot(6));
                    c.g[V0][0] ^= 0x34;
                    if c.g[V0][0] != 0 {
                        lwc1(c, 1, A1, slot(6) + 4);
                        break 'b_1189a8;
                    } else {
                        c.cycles += 3;
                        lw_3(c, V0, S1, mx::RIDER);
                        lwc1(c, 20, V0, rider::BOOST_STRENGTH);
                        break 'b_1189f0;
                    }
                }
                'b_1189ec: {
                    'b_1189d8: {
                        c.cycles += 10;
                        lwc1(c, 0, A1, slot(6) + 8);
                        lw_3(c, V0, S1, mx::RIDER);
                        c.div_s(2, 0, 1);
                        c.lwc1(3, gpf(c, 0x7a6c));
                        lwc1(c, 1, V0, rider::BOOST_STRENGTH);
                        c.add_s(0, 1, 3);
                        c.c_lt_s(0, 2);
                        if !c.fcc() {
                            c.sub_s(0, 1, 3);
                            break 'b_1189d8;
                        } else {
                            c.cycles += 2;
                            c.sub_s(12, 2, 3);
                            break 'b_1189ec;
                        }
                    }
                    c.cycles += 4;
                    c.c_lt_s(2, 0);
                    let t = !c.fcc();
                    c.mov_s(12, 1);
                    if t {
                        break 'b_1189ec;
                    } else {
                        c.cycles += 1;
                        c.add_s(12, 2, 3);
                        break 'b_1189ec;
                    }
                }
                c.cycles += 1;
                c.mov_s(20, 12);
                break 'b_1189f0;
            }
            // ---------------------------------------------------------------
            // The fourth counter (messages 5 and 6, slot 8).
            'b_118afc: {
                'b_118a4c: {
                    c.cycles += 18;
                    lw_3(c, A0, S1, mx::SLOTS);
                    c.mov_s(12, 4);
                    c.g[A1][0] = 5;
                    c.g[A2][0] = 0;
                    addiu(c, A0, A0, slot(5));
                    c.g[A3][0] = 0;
                    callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8a0c)?;
                    lw_3(c, A0, S1, mx::SLOTS);
                    c.mov_s(12, 20);
                    c.g[A1][0] = 6;
                    c.g[A2][0] = 0;
                    addiu(c, A0, A0, slot(6));
                    c.g[A3][0] = 0;
                    callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8a28)?;
                    lw_3(c, V0, S1, mx::RIDER);
                    lw_3(c, V1, V0, rider::BOOST_LEVEL);
                    if c.g[V1][0] != 0 {
                        lw_3(c, V1, S1, mx::SLOTS);
                        break 'b_118a4c;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = 8;
                        callee::REMOVE_HUD_MESSAGE_2.call(c, 0x0011_8a44)?;
                        lw_3(c, A0, S1, mx::RIDER);
                        break 'b_118afc;
                    }
                }
                'b_118a70: {
                    c.cycles += 4;
                    lw_3(c, V0, V1, slot(8));
                    c.g[V0][0] ^= 0x34;
                    let t = c.g[V0][0] != 0;
                    addiu(c, A0, V1, slot(8));
                    if t {
                        break 'b_118a70;
                    } else {
                        c.cycles += 5;
                        c.mtc1(0, 12);
                        c.g[A1][0] = 8;
                        c.g[A2][0] = 0;
                        c.g[A3][0] = 0;
                        callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8a70)?;
                        break 'b_118a70;
                    }
                }
                // The boost level as a fraction of its maximum.
                'b_118ae8: {
                    'b_118ad4: {
                        c.cycles += 23;
                        lw_3(c, V1, S1, mx::RIDER);
                        c.g[A1][0] = 0xa;
                        lw_3(c, A0, S1, mx::SLOTS);
                        lw_3(c, V0, V1, rider::BOOST_LEVEL);
                        c.lwc1(4, gpf(c, 0x7a68));
                        c.g[V1][0] = ((c.g[V0][0] as i64) < 11) as u64;
                        lwc1(c, 1, A0, slot(8) + 8);
                        if c.g[V1][0] == 0 {
                            c.g[V0][0] = c.g[A1][0];
                        }
                        lwc1(c, 3, A0, slot(8) + 4);
                        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        fc(c, 2, 0x3f80_0000);
                        c.mtc1(7, 0);
                        c.cvt_s_w(0, 0);
                        c.lwc1(5, gpf(c, 0x7a64));
                        // The bar's step is 1/150 per 60 Hz tick.
                        if crate::tick::rate(c) != 60 {
                            let step = hud_step_150(c);
                            c.f[5] = step.0;
                        }
                        c.div_s(3, 1, 3);
                        c.mul_s(0, 0, 4);
                        c.min_s(12, 0, 2);
                        c.add_s(1, 12, 5);
                        c.c_lt_s(1, 3);
                        if !c.fcc() {
                            c.sub_s(0, 12, 5);
                            break 'b_118ad4;
                        } else {
                            c.cycles += 2;
                            c.sub_s(12, 3, 5);
                            break 'b_118ae8;
                        }
                    }
                    'b_118ae4: {
                        c.cycles += 4;
                        c.c_lt_s(3, 0);
                        if c.fcc() {
                            c.add_s(12, 3, 5);
                            break 'b_118ae4;
                        } else {
                            break 'b_118ae4;
                        }
                    }
                    c.cycles += 1;
                    lw_3(c, A0, S1, mx::SLOTS);
                    break 'b_118ae8;
                }
                c.cycles += 5;
                c.g[A1][0] = 8;
                c.g[A2][0] = 0;
                addiu(c, A0, A0, slot(8));
                callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8af8)?;
                lw_3(c, A0, S1, mx::RIDER);
                break 'b_118afc;
            }
            // ---------------------------------------------------------------
            // Trick-name banner slot 10 (messages 0xa) and its history.
            'b_118c30: {
                'b_118bfc: {
                    'b_118b24: {
                        c.cycles += 7;
                        c.g[A1][0] = 0xa;
                        lw_3(c, A3, S1, mx::TRICK_FLAG);
                        lw_3(c, V0, A0, rider::BOOST_LEVEL);
                        c.g[V1][0] = ((c.g[V0][0] as i64) < 11) as u64;
                        if c.g[V1][0] == 0 {
                            c.g[V0][0] = c.g[A1][0];
                        }
                        let t = c.g[A3][0] != 0;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        if t {
                            break 'b_118b24;
                        } else {
                            c.cycles += 3;
                            lw_3(c, V0, S1, mx::TRICK_COUNT);
                            if c.g[V0][0] == 0 {
                                lw_3(c, V1, S1, mx::SLOTS);
                                break 'b_118bfc;
                            } else {
                                break 'b_118b24;
                            }
                        }
                    }
                    c.cycles += 2;
                    if (c.g[A1][0] as i64) <= 0 {
                        lw_3(c, V1, S1, mx::SLOTS);
                        break 'b_118bfc;
                    } else {
                        'b_118b80: {
                            c.cycles += 10;
                            lw_3(c, S0, S1, mx::TRICK_COUNT);
                            lw_3(c, A0, S1, mx::SLOTS);
                            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                            if c.g[A3][0] != 0 {
                                c.g[S0][0] = c.g[V1][0];
                            }
                            lw_3(c, V0, A0, slot(10));
                            c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[S0][0] as i64)) as u64;
                            c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(slot(10)));
                            c.g[V0][0] ^= 0x34;
                            let t = c.g[V0][0] != 0;
                            if c.g[V1][0] != 0 {
                                c.g[S0][0] = c.g[A1][0];
                            }
                            if t {
                                break 'b_118b80;
                            } else {
                                c.cycles += 11;
                                c.g[A0][0] = c.g[A2][0];
                                c.mtc1(0, 12);
                                c.g[A1][0] = 0xa;
                                c.g[A2][0] = 0;
                                c.g[A3][0] = 0;
                                callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8b6c)?;
                                call_ptr(c, table::AUDIO_SYSTEM, 0x0011_8b74)?;
                                lw_3(c, A1, S1, mx::RIDER);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, table::AUDIO_CALL_A, 0x0011_8b80)?;
                                break 'b_118b80;
                            }
                        }
                        // The trick-name timer (slot 10's fraction).
                        'b_118bdc: {
                            'b_118bcc: {
                                c.cycles += 17;
                                lw_3(c, V0, S1, mx::SLOTS);
                                c.mtc1(16, 1);
                                c.cvt_s_w(1, 1);
                                c.lwc1(0, gpf(c, 0x7a60));
                                lwc1(c, 4, V0, slot(10) + 4);
                                lwc1(c, 3, V0, slot(10) + 8);
                                c.mul_s(1, 1, 0);
                                fc(c, 2, 0x3f80_0000);
                                c.div_s(3, 3, 4);
                                c.lwc1(4, gpf(c, 0x7a5c));
                                c.min_s(12, 1, 2);
                                c.add_s(0, 12, 4);
                                c.c_lt_s(0, 3);
                                if !c.fcc() {
                                    c.sub_s(0, 12, 4);
                                    break 'b_118bcc;
                                } else {
                                    c.cycles += 2;
                                    c.sub_s(12, 3, 4);
                                    break 'b_118bdc;
                                }
                            }
                            c.cycles += 4;
                            c.c_lt_s(3, 0);
                            if c.fcc() {
                                c.add_s(12, 3, 4);
                                break 'b_118bdc;
                            } else {
                                break 'b_118bdc;
                            }
                        }
                        c.cycles += 8;
                        lw_3(c, A0, S1, mx::SLOTS);
                        c.g[A3][0] = c.g[S0][0];
                        c.g[A1][0] = 0xa;
                        c.g[A2][0] = 0;
                        addiu(c, A0, A0, slot(10));
                        callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8bf4)?;
                        lw_3(c, V1, S1, mx::SLOTS);
                        break 'b_118c30;
                    }
                }
                'b_118c24: {
                    c.cycles += 4;
                    lw_3(c, V0, V1, slot(10));
                    c.g[V0][0] ^= 0x34;
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = c.g[S1][0];
                        break 'b_118c24;
                    } else {
                        c.cycles += 6;
                        call_ptr(c, table::AUDIO_SYSTEM, 0x0011_8c14)?;
                        lw_3(c, A1, S1, mx::RIDER);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, table::AUDIO_CALL_B, 0x0011_8c20)?;
                        c.g[A0][0] = c.g[S1][0];
                        break 'b_118c24;
                    }
                }
                c.cycles += 3;
                c.g[A1][0] = 0xa;
                callee::REMOVE_HUD_MESSAGE_2.call(c, 0x0011_8c2c)?;
                lw_3(c, V1, S1, mx::SLOTS);
                break 'b_118c30;
            }
            // The board-wake meter: slot 9 (message 9) and its fraction.
            'b_118d54: {
                'b_118d50: {
                    'b_118c84: {
                        c.cycles += 4;
                        lw_3(c, V0, V1, slot(9));
                        c.g[V0][0] ^= 0x34;
                        let t = c.g[V0][0] != 0;
                        addiu(c, A0, V1, slot(9));
                        if t {
                            break 'b_118c84;
                        } else {
                            c.cycles += 7;
                            lw_3(c, V0, S1, mx::RIDER);
                            c.mtc1(0, 12);
                            lwc1(c, 0, V0, rider::BOOST_METER);
                            c.c_lt_s(12, 0);
                            let t = !c.fcc();
                            c.g[A1][0] = 9;
                            if t {
                                break 'b_118d50;
                            } else {
                                c.cycles += 10;
                                c.g[A2][0] = 0;
                                c.g[A3][0] = 0;
                                callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8c68)?;
                                call_ptr(c, table::AUDIO_SYSTEM, 0x0011_8c70)?;
                                lw_3(c, A1, S1, mx::RIDER);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, table::AUDIO_CALL_C, 0x0011_8c7c)?;
                                lw_3(c, A0, S1, mx::SLOTS);
                                break 'b_118d54;
                            }
                        }
                    }
                    'b_118ce4: {
                        c.cycles += 9;
                        lwc1(c, 0, V1, slot(9) + 4);
                        lwc1(c, 1, V1, slot(9) + 8);
                        fc(c, 2, 0x3f80_0000);
                        c.div_s(3, 1, 0);
                        c.c_eq_s(3, 2);
                        let t = !c.fcc();
                        lw_3(c, V1, S1, mx::RIDER);
                        if t {
                            break 'b_118ce4;
                        } else {
                            c.cycles += 5;
                            lwc1(c, 1, V1, rider::BOOST_METER);
                            c.mtc1(0, 0);
                            c.c_eq_s(1, 0);
                            let t = !c.fcc();
                            if t {
                                break 'b_118ce4;
                            } else {
                                c.cycles += 10;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = 9;
                                callee::REMOVE_HUD_MESSAGE_2.call(c, 0x0011_8cc8)?;
                                call_ptr(c, table::AUDIO_SYSTEM, 0x0011_8cd0)?;
                                lw_3(c, A1, S1, mx::RIDER);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, table::AUDIO_CALL_D, 0x0011_8cdc)?;
                                lw_3(c, A0, S1, mx::SLOTS);
                                break 'b_118d54;
                            }
                        }
                    }
                    'b_118d34: {
                        'b_118d20: {
                            c.cycles += 13;
                            c.lwc1(1, gpf(c, 0x7a58));
                            lwc1(c, 0, V1, rider::BOOST_METER);
                            fc(c, 2, 0x3f80_0000);
                            c.mul_s(0, 0, 1);
                            lw_3(c, V0, S1, mx::SLOTS);
                            c.lwc1(1, gpf(c, 0x7a54));
                            c.sub_s(12, 2, 0);
                            c.add_s(0, 12, 1);
                            c.c_lt_s(0, 3);
                            let t = !c.fcc();
                            addiu(c, A0, V0, slot(9));
                            if t {
                                break 'b_118d20;
                            } else {
                                c.cycles += 2;
                                c.sub_s(12, 3, 1);
                                break 'b_118d34;
                            }
                        }
                        c.cycles += 5;
                        c.sub_s(0, 12, 1);
                        c.c_lt_s(3, 0);
                        if c.fcc() {
                            c.add_s(12, 3, 1);
                            break 'b_118d34;
                        } else {
                            break 'b_118d34;
                        }
                    }
                    c.cycles += 7;
                    lw_3(c, A3, V1, rider::BOOST_LEVEL);
                    c.g[A1][0] = 9;
                    c.g[V1][0] = 0xb;
                    c.g[A2][0] = 0;
                    c.g[V0][0] = ((c.g[A3][0] as i64) < 12) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[A3][0] = c.g[V1][0];
                    }
                    callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8d50)?;
                    break 'b_118d50;
                }
                c.cycles += 1;
                lw_3(c, A0, S1, mx::SLOTS);
                break 'b_118d54;
            }
            // ---------------------------------------------------------------
            // The combo timer's slot 3 (message 3, its count at_3 +0x10) and
            // slot 4's fraction.
            'b_118e30: {
                c.cycles += 11;
                c.g[A1][0] = 7;
                lw_3(c, A2, S1, 0x198);
                c.g[A3][0] = 0;
                c.mtc1(0, 12);
                addiu(c, A0, A0, slot(7));
                callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8d6c)?;
                lw_3(c, V1, S1, mx::SLOTS);
                lw_3(c, V0, V1, slot(3));
                c.g[V0][0] ^= 0x34;
                let t = c.g[V0][0] == 0;
                addiu(c, A0, V1, slot(3));
                if t {
                    break 'b_118e30;
                } else {
                    'b_118db4: {
                        c.cycles += 9;
                        lw_3(c, A3, S1, mx::COMBO_POINTS);
                        lwc1(c, 0, V1, slot(3) + 4);
                        lw_3(c, A1, V1, slot(3) + 0x14);
                        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x46));
                        lwc1(c, 1, V1, slot(3) + 8);
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                        c.div_s(1, 1, 0);
                        let t = c.g[V0][0] != 0;
                        c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x46));
                        if t {
                            break 'b_118db4;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x46));
                            c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x46));
                            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            if c.g[V0][0] == 0 {
                                c.g[A2][0] = c.g[A3][0];
                            }
                            break 'b_118db4;
                        }
                    }
                    'b_118dec: {
                        'b_118dd0: {
                            c.cycles += 5;
                            c.lwc1(0, gpf(c, 0x7a50));
                            c.lwc1(2, gpf(c, 0x7a4c));
                            // `1 + step`: the band around the target must be
                            // one tick wide, and the step follows the rate.
                            if crate::tick::rate(c) != 60 {
                                let band = hud_band(FpuF32(c.f[2]), true);
                                c.f[0] = band.0;
                            }
                            c.c_lt_s(0, 1);
                            let t = !c.fcc();
                            if t {
                                break 'b_118dd0;
                            } else {
                                c.cycles += 2;
                                c.sub_s(12, 1, 2);
                                break 'b_118dec;
                            }
                        }
                        c.cycles += 7;
                        c.lwc1(0, gpf(c, 0x7a48));
                        fc(c, 12, 0x3f80_0000);
                        // `1 - step`.
                        if crate::tick::rate(c) != 60 {
                            let band = hud_band(FpuF32(c.f[2]), false);
                            c.f[0] = band.0;
                        }
                        c.c_lt_s(1, 0);
                        if c.fcc() {
                            c.add_s(12, 1, 2);
                            break 'b_118dec;
                        } else {
                            break 'b_118dec;
                        }
                    }
                    'b_118e1c: {
                        c.cycles += 9;
                        lw_3(c, A3, S1, mx::COMBO_COUNT);
                        c.g[A1][0] = 3;
                        callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8df8)?;
                        lwc1(c, 1, S1, mx::COMBO_TIMER);
                        c.mtc1(0, 0);
                        c.c_lt_s(1, 0);
                        let t = !c.fcc();
                        c.g[V0][0] = 4;
                        if t {
                            break 'b_118e1c;
                        } else {
                            c.cycles += 3;
                            lw_3(c, V1, S1, mx::SLOTS);
                            sw_2(c, V1, slot(3) + 0x10, V0);
                            break 'b_118e30;
                        }
                    }
                    c.cycles += 5;
                    c.lwc1(0, gpf(c, 0x7a44));
                    lw_3(c, V0, S1, mx::SLOTS);
                    c.mul_s(0, 1, 0);
                    c.cvt_w_s(1, 0);
                    swc1(c, V0, slot(3) + 0x10, 1);
                    break 'b_118e30;
                }
            }
            // ---------------------------------------------------------------
            // The combo's bonus number (slot 2 / message 2): the slot's
            // fraction is its own progress, else the new bonus.
            'b_118ee0: {
                'b_118ecc: {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S1][0];
                    callee::GET_TRICK_BONUS.call(c, 0x0011_8e38)?;
                    lw_3(c, A1, S1, mx::SLOTS);
                    c.g[A3][0] = c.g[V0][0];
                    lw_3(c, T0, S1, mx::BONUS);
                    lw_3(c, V0, A1, slot(2));
                    addiu(c, A0, A1, slot(2));
                    c.g[V0][0] ^= 0x34;
                    let t = c.g[V0][0] == 0;
                    c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    if t {
                        break 'b_118ecc;
                    } else {
                        'b_118ebc: {
                            'b_118eb8: {
                                c.cycles += 5;
                                lw_3(c, V0, A1, slot(2) + 0x14);
                                lw_3(c, V1, A1, slot(2) + 0xc);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                                let t = c.g[V0][0] != c.g[T0][0];
                                if t {
                                    break 'b_118eb8;
                                } else {
                                    'b_118e94: {
                                        c.cycles += 8;
                                        lwc1(c, 0, A1, slot(2) + 4);
                                        lwc1(c, 1, A1, slot(2) + 8);
                                        c.lwc1(2, gpf(c, 0x7a40));
                                        c.div_s(1, 1, 0);
                                        c.lwc1(3, gpf(c, 0x7a3c));
                                        // `1 + step`.
                                        if crate::tick::rate(c) != 60 {
                                            let band = hud_band(FpuF32(c.f[3]), true);
                                            c.f[2] = band.0;
                                        }
                                        c.c_lt_s(2, 1);
                                        let t = !c.fcc();
                                        if t {
                                            break 'b_118e94;
                                        } else {
                                            c.cycles += 2;
                                            c.sub_s(12, 1, 3);
                                            break 'b_118ebc;
                                        }
                                    }
                                    c.cycles += 7;
                                    c.lwc1(0, gpf(c, 0x7a38));
                                    fc(c, 12, 0x3f80_0000);
                                    // `1 - step`.
                                    if crate::tick::rate(c) != 60 {
                                        let band = hud_band(FpuF32(c.f[3]), false);
                                        c.f[0] = band.0;
                                    }
                                    c.c_lt_s(1, 0);
                                    if c.fcc() {
                                        c.add_s(12, 1, 3);
                                        break 'b_118ebc;
                                    } else {
                                        c.cycles += 2;
                                        break 'b_118ebc;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.mtc1(0, 12);
                            break 'b_118ebc;
                        }
                        c.cycles += 4;
                        c.g[A1][0] = 2;
                        callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8ec4)?;
                        break 'b_118ee0;
                    }
                }
                c.cycles += 2;
                if (c.g[A2][0] as i64) <= 0 {
                    break 'b_118ee0;
                } else {
                    c.cycles += 3;
                    c.mtc1(0, 12);
                    c.g[A1][0] = 2;
                    callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8ee0)?;
                    break 'b_118ee0;
                }
            }
            // ---------------------------------------------------------------
            // The "special" message: the BE interface's name goes to slot 25
            // (message 0x19), slot 4's fraction to message 4.
            'b_118fa8: {
                c.cycles += 28;
                call_ptr(c, table::BE_GET_BE, 0x0011_8ee8)?;
                c.g[A1][0] = 0xb;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, table::BE_GET_INTERFACE, 0x0011_8ef4)?;
                lw_3(c, A0, S1, mx::RIDER);
                c.g[S0][0] = c.g[V0][0];
                lw_3(c, V1, A0, rider::BOARD);
                addiu(c, A0, A0, rider::BOARD);
                c.g[A1][0] = c.lh(at_3(c, V1, 0x38));
                lw_3(c, V0, V1, 0x3c);
                let t = c.g[V0][0] as u32;
                addiu(c, A0, A0, c.g[A1][0] as u32);
                call_ptr(c, t, 0x0011_8f14)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, table::BE_CALL, 0x0011_8f20)?;
                lw_3(c, A0, S1, mx::SLOTS);
                c.g[A2][0] = c.g[V0][0];
                c.mtc1(0, 12);
                c.g[A1][0] = 0x19;
                addiu(c, A0, A0, slot(25));
                c.g[A3][0] = 0;
                callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8f3c)?;
                lw_3(c, A0, S1, mx::SLOTS);
                lw_3(c, V0, A0, slot(4));
                c.g[V0][0] ^= 0x34;
                let t = c.g[V0][0] == 0;
                addiu(c, V1, A0, slot(4));
                if t {
                    break 'b_118fa8;
                } else {
                    'b_118f98: {
                        'b_118f7c: {
                            c.cycles += 9;
                            lwc1(c, 0, A0, slot(4) + 4);
                            lwc1(c, 1, A0, slot(4) + 8);
                            c.lwc1(2, gpf(c, 0x7a34));
                            c.div_s(1, 1, 0);
                            c.lwc1(3, gpf(c, 0x7a30));
                            // `1 + step`.
                            if crate::tick::rate(c) != 60 {
                                let band = hud_band(FpuF32(c.f[3]), true);
                                c.f[2] = band.0;
                            }
                            c.c_lt_s(2, 1);
                            let t = !c.fcc();
                            lw_3(c, A2, A0, slot(4) + 0x14);
                            if t {
                                break 'b_118f7c;
                            } else {
                                c.cycles += 2;
                                c.sub_s(12, 1, 3);
                                break 'b_118f98;
                            }
                        }
                        c.cycles += 7;
                        c.lwc1(0, gpf(c, 0x7a2c));
                        fc(c, 12, 0x3f80_0000);
                        // `1 - step`.
                        if crate::tick::rate(c) != 60 {
                            let band = hud_band(FpuF32(c.f[3]), false);
                            c.f[0] = band.0;
                        }
                        c.c_lt_s(1, 0);
                        if c.fcc() {
                            c.add_s(12, 1, 3);
                            break 'b_118f98;
                        } else {
                            break 'b_118f98;
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[V1][0];
                    c.g[A1][0] = 4;
                    c.g[A3][0] = 0;
                    callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8fa8)?;
                    break 'b_118fa8;
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x90);
        }
        c.cycles += 18;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 27, 0xd8);
        frame.restore_fpr(c, 26, 0xd0);
        frame.restore_fpr(c, 25, 0xc8);
        frame.restore_fpr(c, 24, 0xc0);
        frame.restore_fpr(c, 23, 0xb8);
        frame.restore_fpr(c, 22, 0xb0);
        frame.restore_fpr(c, 21, 0xa8);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod target_speed {

    /// `BELibrary_GetInstance()` (through the function table).
    pub const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;

    /// `BELibrary_GetInterface(lib, 3)`.
    pub const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;

    /// `float BECharInt_GetAttrib0Factor(iface, slot, override)`: the
    /// rider's speed attribute as a factor.
    pub const BE_CHAR_INT_GET_ATTRIB0_FACTOR: u32 = 0x0014_94c0;

    /// `Rider_GetMainState(rider)`.
    pub const RIDER_GET_MAIN_STATE: u32 = 0x0011_fe98;

    /// Rider: back-end player slot.
    pub const RIDER_SLOT: u32 = 0x86c;

    /// Rider: attribute override passed to the factor query.
    pub const ATTRIB_OVERRIDE: u32 = 0xb34;

    /// Rider: index of the course segment it is on.
    pub const SEGMENT_INDEX: u32 = 0x438;

    /// Rider: normalised speed (picks the speed band).
    pub const SPEED: u32 = 0x2fc;

    /// Rider: column blend of the speed table (used when the main state is 0).
    pub const COLUMN_BLEND: u32 = 0x220;

    /// Rider: an int state; from 11 up the target has a floor.
    pub const STATE: u32 = 0x2f4;

    /// Rider: two words compared for equality (no slow-down when equal).
    pub const LINE_A: u32 = 0x320;

    pub const LINE_B: u32 = 0x324;

    /// Rider: word cleared on every normal update.
    pub const CLEARED: u32 = 0x5a0;

    /// Rider: the smoothed target speed (written).
    pub const TARGET: u32 = 0x2e4;

    /// Rider: the target's floor when the state is 11 or more.
    pub const TARGET_FLOOR: u32 = 0x30c;

    /// The world (`gp-0x848`), its course (+0x84), the course's segment
    /// array (+0x44) of 0xb0-byte segments; a segment's +4 is the float
    /// that picks and blends the speed-table rows.
    pub const WORLD_GP: u32 = 0x848;

    pub const WORLD_COURSE: u32 = 0x84;

    pub const COURSE_SEGMENTS: u32 = 0x44;

    pub const SEGMENT_SIZE: u32 = 0xb0;

    pub const SEGMENT_SLOPE: u32 = 0x4;

    /// The speed table: 2 pages (0x60 apart) of rows (0x20) of 4 bands of
    /// 2 floats (columns).
    pub const SPEED_TABLE: u32 = 0x004a_6310;

    pub const PAGE: u32 = 0x60;

    /// `.lit4` (gp-relative): band limits 0.1, 0.4, 0.8; the fixed target
    /// 3333.33 (main state 1); the table-to-speed scale 27.78; smoothing
    /// 0.9 (rising) / 0.97 (falling); the stopped slow-down 0.98.
    pub const LIT_BAND_1: u32 = 0x79dc;

    pub const LIT_BAND_2: u32 = 0x79d8;

    pub const LIT_BAND_3: u32 = 0x79d4;

    pub const LIT_FIXED_TARGET: u32 = 0x79d0;

    pub const LIT_SCALE: u32 = 0x79cc;

    pub const LIT_KEEP_RISING: u32 = 0x79c8;

    pub const LIT_STOPPED: u32 = 0x79c4;

    pub const LIT_KEEP_FALLING: u32 = 0x79c0;
}

/// `void Rider_UpdateTargetSpeed(Rider* r)` (0x11b3f8): the rider's target
/// speed. A bilinear lookup in the speed table (rows by the slope of the
/// segment the rider is on, columns by `COLUMN_BLEND`, band by the
/// normalised speed) gives one value per table page; the two pages are
/// blended by the back end's speed attribute `a` (`factor + 1 - 1`), scaled
/// by 27.78 (x0.98 when not moving and `LINE_A != LINE_B`), and the stored
/// target eases toward that (keeping 0.9 of the old value when rising, 0.97
/// when falling). Main state 1 sets the target to 3333.33 instead. From
/// `STATE` 11 up, a moving rider's target is at least `TARGET_FLOOR`.
pub fn rider_update_target_speed(c: &mut Cpu) {
    use self::target_speed::*;
    use crate::abi;
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;
    const ONE_BITS: u64 = 0x3f80_0000;
    let one = FpuF32::ONE;
    let zero = FpuF32::ZERO;
    guest(c, |c| {
        let lit = |c: &mut Cpu, off: u32| Addr(r32(c, GP).wrapping_sub(off)).read_fpu(c);

        c.cycles += 31;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_fpr(c, 21, 0x48);
        frame.save_fpr(c, 20, 0x40);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let rider = Addr(r32(c, S0));

        // The speed attribute from the back end.
        call_ptr(c, BE_LIBRARY_GET_INSTANCE, 0x0011_b418)?;
        c.g[AT][0] = ONE_BITS;
        c.f[20] = one.0;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        call_ptr(c, BE_LIBRARY_GET_INTERFACE, 0x0011_b42c)?;
        c.g[A2][0] = rider.off(ATTRIB_OVERRIDE).lw(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = rider.off(RIDER_SLOT).lw(c);
        call_ptr(c, BE_CHAR_INT_GET_ATTRIB0_FACTOR, 0x0011_b43c)?;

        // The segment the rider is on.
        let world = Addr(r32(c, GP).wrapping_sub(WORLD_GP)).lw(c);
        c.g[V0][0] = world;
        c.g[A2][0] = SEGMENT_SIZE as u64;
        let index = rider.off(SEGMENT_INDEX).lw(c) as u32;
        let factor = FpuF32(c.f[0]) + one;
        c.f[0] = factor.0;
        let course = Addr(world as u32).off(WORLD_COURSE).lw(c);
        c.g[A1][0] = course;
        c.g[A0][0] = c.g[S0][0];
        let seg_off = abi::mult(c, index, SEGMENT_SIZE);
        c.g[V1][0] = sx(seg_off);
        let segments = Addr(course as u32).off(COURSE_SEGMENTS).lw(c);
        c.g[V0][0] = segments;
        let attrib = factor - one;
        c.f[21] = attrib.0;
        let segment = Addr((segments as u32).wrapping_add(seg_off));
        c.g[S1][0] = sx(segment.0);
        Callee {
            addr: RIDER_GET_MAIN_STATE,
            f: ee::s00::Rider_GetMainState,
        }
        .call(c, 0x0011_b468)?;
        let state = c.g[V0][0];
        c.g[A2][0] = state;

        // Row position: slope > 0 picks rows (1, 2) at weight slope, else
        // rows (0, 1) at weight slope + 1. Main states other than 0 use 1.
        let mut slope = one;
        if state == 0 {
            c.cycles += 1;
            slope = segment.off(SEGMENT_SLOPE).read_fpu(c);
        }
        c.f[1] = slope.0;
        c.cycles += 5;
        c.f[0] = zero.0;
        let upper = zero.lt(slope);
        abi::set_fcc(c, upper);
        if !upper {
            c.cycles += 1;
        }
        c.cycles += 3;
        let row = upper as u64;
        c.g[V0][0] = row;
        c.g[V1][0] = row;
        let mut t = slope;
        if !upper {
            c.cycles += 1;
            t = t + one;
        }
        c.f[3] = t.0;

        // Speed band: < 0.1, < 0.4, < 0.8, else 3.
        c.cycles += 6;
        let speed = rider.off(SPEED).read_fpu(c);
        c.f[1] = speed.0;
        let limit = lit(c, LIT_BAND_1);
        c.f[0] = limit.0;
        let mut band = 0u32;
        let mut cond = limit.le(speed);
        if cond {
            c.cycles += 5;
            let limit = lit(c, LIT_BAND_2);
            c.f[0] = limit.0;
            cond = limit.le(speed);
            band = 1;
            if cond {
                c.cycles += 5;
                let limit = lit(c, LIT_BAND_3);
                c.f[0] = limit.0;
                cond = speed.lt(limit);
                band = if cond {
                    c.cycles += 1;
                    2
                } else {
                    3
                };
            }
        }
        abi::set_fcc(c, cond);
        c.g[A0][0] = band as u64;

        // Column blend.
        c.cycles += 4;
        c.g[AT][0] = ONE_BITS;
        let s = if state == 0 {
            rider.off(COLUMN_BLEND).read_fpu(c)
        } else {
            one
        };
        c.f[2] = s.0;

        // Bilinear weights and the two row pointers.
        c.cycles += 21;
        let one_minus_s = one - s;
        let one_minus_t = one - t;
        c.f[0] = one_minus_t.0;
        c.f[1] = one_minus_s.0;
        c.f[7] = (t * s).0;
        c.f[6] = (t * one_minus_s).0;
        c.f[5] = (one_minus_t * s).0;
        c.f[4] = (one_minus_t * one_minus_s).0;
        let col = band << 3;
        c.g[V1][0] = sx(col
            .wrapping_add((row as u32) << 5)
            .wrapping_add(SPEED_TABLE));
        c.g[A0][0] = sx(col
            .wrapping_add(((row as u32) + 1) << 5)
            .wrapping_add(SPEED_TABLE));
        c.g[V0][0] = sx(sp(c));
        c.g[A1][0] = 1;

        // One bilinear value per table page into sp[0], sp[4].
        loop {
            c.cycles += 17;
            let lo_row = Addr(r32(c, V1));
            let hi_row = Addr(r32(c, A0));
            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
            let p0 = FpuF32(c.f[4]) * lo_row.read_fpu(c);
            let p3 = FpuF32(c.f[6]) * hi_row.read_fpu(c);
            let p2 = FpuF32(c.f[5]) * lo_row.off(4).read_fpu(c);
            let p1 = FpuF32(c.f[7]) * hi_row.off(4).read_fpu(c);
            c.g[A0][0] = sx(hi_row.0.wrapping_add(PAGE));
            c.g[V1][0] = sx(lo_row.0.wrapping_add(PAGE));
            let value = p0 + p3 + p2 + p1;
            c.f[0] = value.0;
            c.f[1] = p1.0;
            c.f[2] = p2.0;
            c.f[3] = p3.0;
            Addr(r32(c, V0)).write_fpu(c, value);
            let more = (c.g[A1][0] as i64) >= 0;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0011_b548);
            }
        }

        c.cycles += 3;
        c.g[V0][0] = 1;
        let page0 = Addr(sp(c)).read_fpu(c);
        c.f[2] = page0.0;
        let target = if state == 1 {
            c.cycles += 3;
            let fixed = lit(c, LIT_FIXED_TARGET);
            c.f[2] = fixed.0;
            fixed
        } else {
            // Pages blended by the attribute, scaled to a speed.
            c.cycles += 14;
            c.g[AT][0] = ONE_BITS;
            let page1 = Addr(sp(c).wrapping_add(4)).read_fpu(c);
            let scale = lit(c, LIT_SCALE);
            c.f[3] = scale.0;
            let b = attrib * page1;
            c.g[V1][0] = rider.off(LINE_A).lw(c);
            let a = (one - attrib) * page0;
            c.f[0] = a.0;
            c.g[V0][0] = rider.off(LINE_B).lw(c);
            rider.off(CLEARED).sw(c, 0);
            let mut keep = lit(c, LIT_KEEP_RISING);
            let blended = b + a;
            c.f[1] = blended.0;
            let mut wanted = blended * scale;
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 6;
                let speed = rider.off(SPEED).read_fpu(c);
                c.f[1] = speed.0;
                c.f[0] = zero.0;
                let stopped = speed.le(zero);
                abi::set_fcc(c, stopped);
                if stopped {
                    c.cycles += 3;
                    let k = lit(c, LIT_STOPPED);
                    c.f[0] = k.0;
                    wanted = wanted * k;
                }
            } else {
                c.cycles += 1;
            }
            let current = rider.off(TARGET).read_fpu(c);

            // Ease toward it.
            c.cycles += 3;
            let falling = wanted.lt(current);
            abi::set_fcc(c, falling);
            if falling {
                c.cycles += 1;
                keep = lit(c, LIT_KEEP_FALLING);
            }
            c.f[4] = keep.0;
            c.cycles += 7;
            c.g[AT][0] = ONE_BITS;
            let kept = keep * current;
            c.f[1] = kept.0;
            let moved = (one - keep) * wanted;
            c.f[0] = moved.0;
            let next = moved + kept;
            c.f[2] = next.0;
            next
        };
        c.g[V0][0] = rider.off(STATE).lw(c);

        c.cycles += 3;
        let below = ((c.g[V0][0] as i64) < 11) as u64;
        c.g[V0][0] = below;
        rider.off(TARGET).write_fpu(c, target);
        if below == 0 {
            c.cycles += 6;
            let speed = rider.off(SPEED).read_fpu(c);
            c.f[1] = speed.0;
            c.f[0] = zero.0;
            let moving = zero.lt(speed);
            abi::set_fcc(c, moving);
            if moving {
                c.cycles += 4;
                let floored = rider.off(TARGET_FLOOR).read_fpu(c).max(target);
                c.f[0] = floored.0;
                rider.off(TARGET).write_fpu(c, floored);
            }
        } else {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 6;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderMetrix_SetFlipAmount (0x1198d8)

/// RiderMetrix: the trick's accumulated rotation total and the flip amount.
pub(crate) const METRIX_ROTATION_TOTAL: u32 = 0x14;

pub(crate) const METRIX_FLIP_AMOUNT: u32 = 0x38;

/// `gp-0x7a10`: the flip weight in the rotation total (0.0286..., `.lit4`).
pub(crate) const FLIP_WEIGHT_GP: u32 = 0x7a10;

/// `float RiderMetrix_SetFlipAmount(RiderMetrix* m, float flip)` (0x1198d8):
/// replace the flip amount, swapping its weighted magnitude in the
/// rotation total (`total - |old| * k + |flip| * k`, not below 0). Returns
/// 0.0 in f0.
pub fn rider_metrix_set_flip_amount(c: &mut Cpu) {
    c.cycles += 15;
    let m = Addr(abi::arg(c, 0));
    let gp = r32(c, GP);
    let k = Addr(gp.wrapping_sub(FLIP_WEIGHT_GP)).read_fpu(c);
    c.f[2] = k.0;
    let flip = FpuF32(c.f[12]);
    let old = m.off(METRIX_FLIP_AMOUNT).read_fpu(c);
    let total = m.off(METRIX_ROTATION_TOTAL).read_fpu(c);
    let removed = old.abs() * k;
    m.off(METRIX_FLIP_AMOUNT).write_fpu(c, flip);
    let added = flip.abs() * k;
    let less = total - removed;
    let new_total = (less + added).max(FpuF32::ZERO);
    c.f[3] = less.0;
    c.f[2] = 0;
    c.f[0] = 0;
    c.f[1] = new_total.0;
    m.off(METRIX_ROTATION_TOTAL).write_fpu(c, new_total);
    ret(c);
}

// ---------------------------------------------------------------------------
// RiderMetrix_SetSpinAmount (0x119898)

/// RiderMetrix: the spin amount of the current trick (float).
pub(crate) const SPIN_METRIX_SPIN_AMOUNT: u32 = 0x34;

/// RiderMetrix: the running rotation total (spin and flip, weighted).
pub(crate) const SPIN_METRIX_ROTATION_TOTAL: u32 = 0x14;

/// `gp`-relative weight of a spin degree in the rotation total (.sdata).
pub(crate) const SPIN_WEIGHT_GP: u32 = 0x7a14;

/// `void RiderMetrix_SetSpinAmount(RiderMetrix* m, float spin)` (0x119898):
/// replaces the stored spin amount and moves the rotation total by the
/// weighted difference of the magnitudes, clamped at zero:
/// `total = max(total - |old|*k + |spin|*k, 0)`. Leaves f0 = f2 = 0,
/// f3 = the total minus the old share, f1 = the new total.
pub fn rider_metrix_set_spin_amount(c: &mut Cpu) {
    c.cycles += 15;
    let m = Addr(abi::arg(c, 0));
    let gp = r32(c, GP);
    let k = Addr(gp.wrapping_sub(SPIN_WEIGHT_GP)).read_fpu(c);
    let spin = FpuF32(c.f[12]);
    let old = m.off(SPIN_METRIX_SPIN_AMOUNT).read_fpu(c);
    let total = m.off(SPIN_METRIX_ROTATION_TOTAL).read_fpu(c);
    let removed = old.abs() * k;
    m.off(SPIN_METRIX_SPIN_AMOUNT).write_fpu(c, spin);
    let added = spin.abs() * k;
    let less = total - removed;
    let new_total = (less + added).max(FpuF32::ZERO);
    c.f[3] = less.0;
    c.f[2] = 0;
    c.f[0] = 0;
    c.f[1] = new_total.0;
    m.off(SPIN_METRIX_ROTATION_TOTAL).write_fpu(c, new_total);
    ret(c);
}

// ---------------------------------------------------------------------------
// RiderMetrix_GetTrickBonus (0x117908)

/// The metrix's trick bonus factor (float).
pub(crate) const RIDER_METRIX_TRICK_BONUS: u32 = 0x1c;

/// The scale applied to it (gp-0x7b0c, 10000.0 in the game).
pub(crate) const RIDER_METRIX_BONUS_SCALE_GP: u32 = 0x7b0c;

/// `int RiderMetrix_GetTrickBonus(RiderMetrix *self)` (0x117908): the trick
/// bonus factor scaled to points, `(int)(factor * scale + 5)` rounded down
/// to a multiple of 10 (toward zero).
pub fn rider_metrix_get_trick_bonus(c: &mut Cpu) {
    c.cycles += 7;
    let factor = Addr(r32(c, A0).wrapping_add(RIDER_METRIX_TRICK_BONUS)).read_fpu(c);
    let gp = c.g[GP][0] as u32;
    let scale = Addr(gp.wrapping_sub(RIDER_METRIX_BONUS_SCALE_GP)).read_fpu(c);
    c.f[2] = scale.0;
    c.g[A0][0] = 0xa;
    c.g[AT][0] = 0x40a0_0000;
    let five = FpuF32(0x40a0_0000);
    c.cycles += 8;
    let rounded = factor * scale + five;
    c.f[0] = rounded.0;
    let points = rounded.to_i32();
    c.f[1] = points as u32;
    c.g[V0][0] = sx(points as u32);
    let (_, rem) = abi::div(c, points as u32, 10);
    c.g[V1][0] = sx(rem);
    c.g[V0][0] = sx((points as u32).wrapping_sub(rem));
    ret(c);
}

pub(crate) mod finish_trick_callee {
    use super::*;

    pub const REMOVE_HUD_MESSAGE: Callee = Callee {
        addr: 0x0011_79e0,
        f: ee::s00::f_001179e0,
    };

    pub const ALL_PLAYERS_FINISHED: Callee = Callee {
        addr: 0x0012_a250,
        f: ee::s00::f_0012a250,
    };

    pub const BUILD_TRICK_DESC: Callee = Callee {
        addr: 0x0011_a8c8,
        f: ee::s00::f_0011a8c8,
    };

    pub const GET_ROUNDED_TRICK_POINTS: Callee = Callee {
        addr: 0x0011_7948,
        f: ee::s00::f_00117948,
    };

    pub const LOOKUP_BONUS_TRICK: Callee = Callee {
        addr: 0x0011_b1a8,
        f: ee::s00::f_0011b1a8,
    };

    pub const SHOW_HUD_NUMBER: Callee = Callee {
        addr: 0x0011_7b88,
        f: ee::s00::f_00117b88,
    };

    pub const COUNT_TRICK_REPEATS: Callee = Callee {
        addr: 0x0011_90f0,
        f: ee::s00::f_001190f0,
    };

    pub const SHOW_TRICK_NAME: Callee = Callee {
        addr: 0x0011_8ff8,
        f: ee::s00::f_00118ff8,
    };

    pub const GET_TRICK_SCORE: Callee = Callee {
        addr: 0x0011_7990,
        f: ee::s00::f_00117990,
    };

    pub const GET_SCORE_TIER: Callee = Callee {
        addr: 0x0011_9310,
        f: ee::s00::f_00119310,
    };

    pub const GET_TRICK_BONUS: Callee = Callee {
        addr: 0x0011_7908,
        f: ee::s00::f_00117908,
    };

    pub const ADD_COMBO_POINTS: Callee = Callee {
        addr: 0x0011_7638,
        f: ee::s00::f_00117638,
    };

    pub const GET_TRICK_POINTS: Callee = Callee {
        addr: 0x0011_7900,
        f: ee::s00::f_00117900,
    };

    pub const AWARD_POINTS: Callee = Callee {
        addr: 0x0011_9ef8,
        f: ee::s00::f_00119ef8,
    };

    pub const UPDATE_PEAK_A4: Callee = Callee {
        addr: 0x0011_7708,
        f: ee::s00::f_00117708,
    };

    /// Cross-shard calls (through the function table).
    pub const AUDIO_GET_INSTANCE: u32 = 0x0028_b180;

    pub const AUDIO_ON_BONUS_TRICK: u32 = 0x0029_b7e0;

    pub const AUDIO_ON_TRICK_LANDED: u32 = 0x0029_b430;

    pub const BE_GET_INSTANCE: u32 = 0x0014_dc80;

    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;
}

/// The event settings (`D_00535BC8`): kind at +0x48, game type at +0x49.
pub(crate) const FINISH_TRICK_EVENT_SETTINGS: u32 = 0x0053_5bc8;

/// `float RiderMetrix_FinishTrick(RiderMetrix* m, int stance, int alternate,
/// int style, int flag, int takeoff)` (0x11a228): clear the trick HUD
/// messages; unless every player has finished, build the landed trick's id
/// and, when it scored, bank it: a bonus-trick bonus (score `+0x14`, HUD
/// and audio), repeats (dividing the points), combo points, the per-run
/// statistics (points `+0x198`, bonus `+0x19c`, count `+0x110`, grade
/// counts `+0x16c`, best trick `+0x18c/+0x190`, spin/flip degrees, air
/// time, rails, peaks), the landing sound, then the HUD numbers (or, in a
/// Conquer-the-Mountain free ride, career points). Finally peaks/`+0xa4`
/// when the landing is clean. Returns the trick points over the repeats.
pub fn rider_metrix_finish_trick(c: &mut Cpu) {
    use finish_trick_callee as f;
    guest(c, |c| {
        c.cycles += 47;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xb0);
        frame.save(c, S1, 0xa0);
        frame.save(c, S2, 0x90);
        frame.save(c, S7, 0x40);
        frame.save(c, FP, 0x30);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save(c, S6, 0x50);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 20, 0xc0);
        c.g[S0][0] = c.g[A1][0]; // stance
        c.g[S1][0] = c.g[A0][0]; // m
        c.g[S2][0] = c.g[A2][0]; // alternate
        c.g[S7][0] = c.g[A3][0]; // style
        c.g[FP][0] = c.g[T0][0]; // flag
        Addr(sp(c) + 0x10).sw(c, c.g[T1][0]); // takeoff
        let m = r32(c, S1);

        // Clear the trick HUD messages.
        let codes: [(u64, u32); 8] = [
            (0x1, 0x0011_a278),
            (0x2, 0x0011_a284),
            (0x4, 0x0011_a290),
            (0x1c, 0x0011_a29c),
            (0x1d, 0x0011_a2a8),
            (0x1e, 0x0011_a2b4),
            (0x1f, 0x0011_a2c0),
            (0x20, 0x0011_a2cc),
        ];
        for (code, ret_addr) in codes {
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = code;
            f::REMOVE_HUD_MESSAGE.call(c, ret_addr)?;
        }

        // Nothing is scored once everybody has finished.
        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0) + 0x84).lw(c);
        c.g[A0][0] = Addr(r32(c, V1) + 0xc).lw(c);
        f::ALL_PLAYERS_FINISHED.call(c, 0x0011_a2dc)?;
        let finished = c.g[V0][0] != 0;
        c.g[A2][0] = c.g[S0][0];
        if finished {
            c.cycles += 3;
            c.mtc1(0, 0);
            frame.restore(c, S0, 0xb0);
            finish_trick_epilogue(c, frame);
            return Ok(());
        }

        c.cycles += 12;
        c.g[A0][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[A1][0] = c.g[SP][0]; // &id
        c.g[T0][0] = c.g[S7][0];
        c.g[T1][0] = c.g[FP][0];
        f::BUILD_TRICK_DESC.call(c, 0x0011_a308)?;
        c.mtc1(0, 20); // delta = 0
        c.g[S0][0] = c.g[V0][0]; // valid
        c.g[A0][0] = c.g[S1][0];
        f::GET_ROUNDED_TRICK_POINTS.call(c, 0x0011_a318)?;
        let scored = (c.g[V0][0] as i64) > 0;
        c.g[V0][0] = Addr(sp(c) + 0x10).lw(c);
        if scored {
            finish_trick_bank(c, m)?;
        }

        // The clean-landing check (v0 = takeoff).
        c.cycles += 2;
        let check_peaks;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = Addr(m + 0x20).lw(c);
            c.cycles += 2;
            let busy = c.g[V0][0] != 0;
            c.mov_s(0, 20);
            if busy {
                check_peaks = false;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(m + 0x28).lw(c);
                let busy = c.g[V0][0] != 0;
                frame.restore(c, S0, 0xb0);
                if busy {
                    finish_trick_epilogue(c, frame);
                    return Ok(());
                }
                c.cycles += 1;
                c.lwc1(0, m + 0x14);
                check_peaks = true;
            }
        } else {
            c.cycles += 2;
            let styled = c.g[S7][0] != 0;
            c.mov_s(0, 20);
            if styled {
                check_peaks = false;
            } else {
                c.cycles += 2;
                if c.g[FP][0] == 0 {
                    c.lwc1(0, m + 0x14);
                    check_peaks = true;
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0xb0);
                    finish_trick_epilogue(c, frame);
                    return Ok(());
                }
            }
        }
        if check_peaks {
            c.cycles += 4;
            c.mtc1(0, 1);
            c.c_lt_s(1, 0);
            let mut update = c.fcc(); // score > 0
            if !update {
                c.cycles += 5;
                c.lwc1(0, m + 0xa4);
                c.c_lt_s(0, 1);
                update = c.fcc(); // +0xa4 < 0
                c.mov_s(0, 20);
            }
            if update {
                c.cycles += 5;
                c.g[AT][0] = 0x3fc0_0000;
                c.mtc1(1, 12); // 1.5
                c.g[A0][0] = c.g[S1][0];
                f::UPDATE_PEAK_A4.call(c, 0x0011_a76c)?;
                c.mov_s(0, 20);
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0xb0);
        finish_trick_epilogue(c, frame);
        Ok(())
    })
}

/// RiderMetrix_FinishTrick's scoring part (0x11a320..0x11a700): bank the
/// trick; leaves v0 = takeoff (reloaded from the frame).
pub(crate) fn finish_trick_bank(c: &mut Cpu, m: u32) -> crate::call::Flow {
    use finish_trick_callee as f;
    c.cycles += 2;
    c.g[S6][0] = 0; // repeats
    if c.g[S0][0] != 0 {
        c.cycles += 5;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[SP][0];
        f::LOOKUP_BONUS_TRICK.call(c, 0x0011_a334)?;
        let bonus = (c.g[V0][0] as i64) > 0;
        c.g[A1][0] = 0x32;
        if bonus {
            c.cycles += 18;
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.lwc1(2, r32(c, GP).wrapping_sub(0x79f4));
            c.lwc1(1, m + 0x14);
            c.g[A0][0] = c.g[S1][0];
            c.g[AT][0] = 0x3fc0_0000;
            c.mtc1(1, 12); // 1.5
            c.mul_s(0, 0, 2);
            c.g[A2][0] = c.g[V0][0];
            c.g[A3][0] = 0;
            c.add_s(1, 1, 0);
            c.swc1(1, m + 0x14);
            f::SHOW_HUD_NUMBER.call(c, 0x0011_a370)?;
            call_ptr(c, f::AUDIO_GET_INSTANCE, 0x0011_a378)?;
            c.g[A1][0] = Addr(m + 0x1ac).lw(c);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, f::AUDIO_ON_BONUS_TRICK, 0x0011_a384)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[SP][0];
        f::COUNT_TRICK_REPEATS.call(c, 0x0011_a390)?;
        c.g[S6][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = (0 < (c.g[S6][0] as i64)) as u64;
        f::SHOW_TRICK_NAME.call(c, 0x0011_a3a4)?;
    }

    c.cycles += 18;
    c.g[A0][0] = c.g[S1][0];
    c.g[S0][0] = sx(r32(c, S6).wrapping_add(1)); // divisor
    f::GET_ROUNDED_TRICK_POINTS.call(c, 0x0011_a3b0)?;
    c.g[S3][0] = c.g[V0][0]; // pending
    c.g[A0][0] = c.g[S1][0];
    f::GET_TRICK_SCORE.call(c, 0x0011_a3bc)?;
    c.g[S2][0] = c.g[V0][0]; // points
    c.g[A1][0] = c.g[S3][0];
    c.g[A0][0] = c.g[S1][0];
    f::GET_SCORE_TIER.call(c, 0x0011_a3cc)?;
    c.g[S5][0] = c.g[V0][0]; // grade
    c.g[A0][0] = c.g[S1][0];
    f::GET_TRICK_BONUS.call(c, 0x0011_a3d8)?;
    c.div(0, S3, S0);
    c.g[S4][0] = c.g[V0][0]; // inverted
    c.g[A0][0] = c.g[S1][0];

    c.cycles += 10;
    c.g[A1][0] = c.lo[0];
    f::ADD_COMBO_POINTS.call(c, 0x0011_a3f8)?;
    c.g[A0][0] = c.g[S1][0];
    f::GET_TRICK_POINTS.call(c, 0x0011_a400)?;
    c.div(0, S2, S0);
    c.g[A2][0] = 0xa;
    c.g[V0][0] = sx(r32(c, S5) << 2);

    // Totals, grade counts, delta = points / divisor.
    c.cycles += 27;
    c.g[A1][0] = sx(m.wrapping_add(0x16c).wrapping_add(r32(c, V0)));
    c.g[V1][0] = Addr(m + 0x19c).lw(c);
    c.g[V0][0] = Addr(m + 0x110).lw(c);
    c.mtc1(S0, 1);
    c.cvt_s_w(1, 1);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, S4)));
    c.g[A0][0] = Addr(m + 0x198).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    Addr(m + 0x19c).sw(c, c.g[V1][0]);
    Addr(m + 0x110).sw(c, c.g[V0][0]);
    c.div_s(20, 0, 1);
    c.g[A3][0] = c.lo[0];
    c.g[S2][0] = c.g[A3][0];
    c.div(0, S2, A2);
    c.g[V0][0] = c.hi[0];
    c.g[S2][0] = sx(r32(c, S2).wrapping_sub(r32(c, V0)));
    c.g[V1][0] = sx(r32(c, S2).wrapping_add(r32(c, S4)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    Addr(m + 0x198).sw(c, c.g[A0][0]);
    let grade_count = r32(c, A1);
    c.g[V0][0] = Addr(grade_count).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    Addr(grade_count).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(m + 0x18c).lw(c);
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[S3][0] as i64)) as u64;
    if c.g[V1][0] != 0 {
        // New best trick: points and id.
        c.cycles += 5;
        Addr(m + 0x18c).sw(c, c.g[S3][0]);
        let s = sp(c);
        c.ldl(2, s + 7);
        c.ldr(2, s);
        c.sdl(2, m + 0x197);
        c.sdr(2, m + 0x190);
    }

    // Spin and flip in whole degrees, rounded to half turns.
    c.cycles += 5;
    c.lwc1(1, r32(c, GP).wrapping_sub(0x79f0));
    c.g[V1][0] = 0xb4;
    c.lwc1(0, m + 0x34);
    c.cycles += 8;
    c.mul_s(0, 0, 1);
    c.cvt_w_s(1, 0);
    c.mfc1(A0, 1);
    c.div(0, A0, V1);
    c.g[V1][0] = c.hi[0];
    let low = (c.g[V1][0] as i64) < -90;
    c.g[V0][0] = sx(r32(c, A0).wrapping_sub(r32(c, V1)));
    if low {
        c.cycles += 2;
        c.g[A1][0] = sx(r32(c, V0).wrapping_sub(0xb4));
    } else {
        c.cycles += 3;
        let small = (c.g[V1][0] as i64) < 91;
        c.g[V0][0] = small as u64;
        c.g[A1][0] = sx(r32(c, A0).wrapping_sub(r32(c, V1)));
        if !small {
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, A0).wrapping_sub(r32(c, V1)));
            c.g[A1][0] = sx(r32(c, V0).wrapping_add(0xb4));
        }
    }
    c.cycles += 5;
    c.lwc1(1, r32(c, GP).wrapping_sub(0x79ec));
    c.g[V1][0] = 0xb4;
    c.lwc1(0, m + 0x38);
    c.cycles += 8;
    c.mul_s(0, 0, 1);
    c.cvt_w_s(1, 0);
    c.mfc1(A0, 1);
    c.div(0, A0, V1);
    c.g[A3][0] = c.hi[0];
    let low = (c.g[A3][0] as i64) < -90;
    c.g[V0][0] = sx(r32(c, A0).wrapping_sub(r32(c, A3)));
    if low {
        c.cycles += 2;
        c.g[A3][0] = sx(r32(c, V0).wrapping_sub(0xb4));
    } else {
        c.cycles += 3;
        let small = (c.g[A3][0] as i64) < 91;
        c.g[V0][0] = small as u64;
        if small {
            c.g[A3][0] = sx(r32(c, A0).wrapping_sub(r32(c, A3)));
        } else {
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, A0).wrapping_sub(r32(c, A3)));
            c.g[A3][0] = sx(r32(c, V0).wrapping_add(0xb4));
        }
    }
    // abs(spin) in v0, abs(flip) in a3.
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    if (c.g[A1][0] as i64) < 0 {
        c.cycles += 1;
        c.g[V0][0] = sx(0u32.wrapping_sub(r32(c, V0)));
    }
    c.cycles += 4;
    c.g[V1][0] = Addr(m + 0x100).lw(c);
    c.g[A1][0] = Addr(m + 0xfc).lw(c);
    if (c.g[A3][0] as i64) < 0 {
        c.g[A3][0] = sx(0u32.wrapping_sub(r32(c, A3)));
    }

    c.cycles += 18;
    c.lwc1(1, m + 0x144);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A3)));
    c.lwc1(0, m + 0x44);
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
    c.g[A2][0] = Addr(m + 0x104).lw(c);
    c.g[A0][0] = Addr(m + 0x108).lw(c);
    c.add_s(1, 1, 0);
    c.g[V0][0] = Addr(m + 0x4c).lw(c);
    c.g[A3][0] = Addr(m + 0x80).lw(c);
    c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, V0)));
    c.g[T0][0] = Addr(m + 0x54).lw(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A3)));
    Addr(m + 0xfc).sw(c, c.g[A1][0]);
    Addr(m + 0x100).sw(c, c.g[V1][0]);
    c.swc1(1, m + 0x144);
    Addr(m + 0x104).sw(c, c.g[A2][0]);
    let rails = (c.g[T0][0] as i64) > 0;
    Addr(m + 0x108).sw(c, c.g[A0][0]);
    if rails {
        c.cycles += 7;
        call_ptr(c, f::AUDIO_GET_INSTANCE, 0x0011_a5a0)?;
        c.g[A1][0] = Addr(m + 0x1ac).lw(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = Addr(m + 0x54).lw(c);
        c.g[A3][0] = Addr(m + 0x114).lw(c);
        call_ptr(c, f::AUDIO_ON_TRICK_LANDED, 0x0011_a5b4)?;
    }

    // Running sums and peaks.
    c.cycles += 44;
    c.lwc1(4, m + 0x14c);
    c.lwc1(5, m + 0x150);
    c.lwc1(6, m + 0x154);
    c.lwc1(7, m + 0x158);
    c.g[T1][0] = Addr(m + 0x84).lw(c);
    c.g[V1][0] = Addr(m + 0x58).lw(c);
    c.g[A1][0] = Addr(m + 0x114).lw(c);
    c.g[T2][0] = sx(r32(c, S2).wrapping_add(r32(c, T1)));
    c.g[A2][0] = Addr(m + 0x118).lw(c);
    c.g[S0][0] = sx(r32(c, T2).wrapping_add(r32(c, S4))); // total
    c.g[A3][0] = Addr(m + 0x11c).lw(c);
    c.g[T0][0] = Addr(m + 0x198).lw(c);
    c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, V1)));
    c.g[A0][0] = Addr(m + 0x74).lw(c);
    c.g[V0][0] = Addr(m + 0x54).lw(c);
    c.g[T0][0] = sx(r32(c, T0).wrapping_add(r32(c, T1)));
    c.lwc1(3, m + 0x30);
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
    c.lwc1(2, m + 0x48);
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
    c.lwc1(1, m + 0x24);
    c.max_s(3, 3, 4);
    c.lwc1(0, m + 0x78);
    c.max_s(2, 2, 5);
    c.max_s(1, 1, 6);
    Addr(m + 0x114).sw(c, c.g[A1][0]);
    c.max_s(0, 0, 7);
    Addr(m + 0x118).sw(c, c.g[A2][0]);
    Addr(m + 0x11c).sw(c, c.g[A3][0]);
    c.swc1(3, m + 0x14c);
    c.swc1(2, m + 0x150);
    c.swc1(1, m + 0x154);
    c.swc1(0, m + 0x158);
    Addr(m + 0x198).sw(c, c.g[T0][0]);
    call_ptr(c, f::BE_GET_INSTANCE, 0x0011_a640)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, f::BE_GET_INTERFACE, 0x0011_a64c)?;
    c.g[V1][0] = sx(FINISH_TRICK_EVENT_SETTINGS);
    c.g[A0][0] = 0x4;
    c.g[V0][0] = Addr(FINISH_TRICK_EVENT_SETTINGS + 0x48).lb(c);
    let mountain = c.g[V0][0] == c.g[A0][0];
    c.g[A1][0] = 0;
    if mountain {
        c.cycles += 2;
        c.g[V0][0] = Addr(FINISH_TRICK_EVENT_SETTINGS + 0x49).lb(c);
        c.g[A1][0] = (c.g[V0][0] < 1) as u64;
    }

    c.cycles += 2;
    let career = c.g[A1][0] != 0;
    c.g[V0][0] = 0x1f4;
    if !career {
        c.cycles += 2;
        let repeated = (c.g[S6][0] as i64) > 0;
        c.g[A3][0] = c.g[S5][0];
        if repeated {
            c.cycles += 8;
            c.g[AT][0] = 0x4020_0000;
            c.mtc1(1, 12); // 2.5
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x24;
            c.g[A2][0] = c.g[S0][0];
            f::SHOW_HUD_NUMBER.call(c, 0x0011_a694)?;
        } else {
            c.cycles += 6;
            c.g[AT][0] = 0x4020_0000;
            c.mtc1(1, 12); // 2.5
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x23;
            c.g[A2][0] = c.g[S0][0];
            f::SHOW_HUD_NUMBER.call(c, 0x0011_a6b4)?;
        }
        c.cycles += 8;
        c.lwc1(12, r32(c, GP).wrapping_sub(0x79e8));
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0x18;
        c.g[A3][0] = 0;
        f::SHOW_HUD_NUMBER.call(c, 0x0011_a6cc)?;
    } else {
        c.cycles += 4;
        c.g[V1][0] = 0x14;
        c.div(0, S0, V0);
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = c.lo[0];
        c.g[V0][0] = ((c.g[A2][0] as i64) < 21) as u64;
        if c.g[V0][0] == 0 {
            c.g[A2][0] = c.g[V1][0];
        }
        f::AWARD_POINTS.call(c, 0x0011_a6fc)?;
    }
    c.g[V0][0] = Addr(sp(c) + 0x10).lw(c);
    Ok(())
}

/// RiderMetrix_FinishTrick's epilogue (0x11a774).
pub(crate) fn finish_trick_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 12;
    frame.restore(c, S1, 0xa0);
    frame.restore(c, S2, 0x90);
    frame.restore(c, S3, 0x80);
    frame.restore(c, S4, 0x70);
    frame.restore(c, S5, 0x60);
    frame.restore(c, S6, 0x50);
    frame.restore(c, S7, 0x40);
    frame.restore(c, FP, 0x30);
    frame.restore_ra(c, 0x20);
    frame.restore_fpr(c, 20, 0xc0);
    frame.close(c);
}

/// `void RiderMetrix::SaveState(cStream* s)` (0x1174b0): writes the
/// 0x1ac-byte metrics block.
pub fn rider_metrix_save_state(c: &mut Cpu) {
    stream_write_member(c, 0, 0x1ac, 0x0011_74d8);
}

pub(crate) mod land_trick_callee {
    use super::*;

    pub const FINISH_TRICK: Callee = Callee {
        addr: 0x0011_a228,
        f: ee::s00::RiderMetrix_FinishTrick,
    };

    pub const RESET_TRICK: Callee = Callee {
        addr: 0x0011_7838,
        f: ee::s00::RiderMetrix_ResetTrick,
    };
}

pub(crate) mod add_trick_callee {
    use super::*;

    /// `cBE_getBE`, `BELibrary_GetInterface`, `BEBAGTInt_GetTrickPoints`,
    /// `BEBAGTInt_GetTrickPoints2` (other shard: through the function table).
    pub const BE_GET_BE: u32 = 0x0014_dc80;

    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;

    pub const GET_TRICK_POINTS: u32 = 0x0015_0528;

    pub const GET_TRICK_POINTS_2: u32 = 0x0015_0540;

    pub const CLEAR_PEAK_A4: Callee = Callee {
        addr: 0x0011_76f8,
        f: ee::s00::RiderMetrix_ClearPeakA4,
    };
}

// ---------------------------------------------------------------------------
// RiderMetrix_InterruptTrick (0x119e38)

/// RiderMetrix fields used by InterruptTrick.
pub(crate) const METRIX_INT_STANCE: u32 = 0x0;

pub(crate) const METRIX_INT_STANCE_2: u32 = 0x4;

pub(crate) const METRIX_INT_PREV_TRICKING: u32 = 0xc;

pub(crate) const METRIX_INT_PREV_28: u32 = 0x10;

pub(crate) const METRIX_INT_TRICKING: u32 = 0x20;

pub(crate) const METRIX_INT_TIMER: u32 = 0x24;

pub(crate) const METRIX_INT_28: u32 = 0x28;

pub(crate) const METRIX_INT_30: u32 = 0x30;

pub(crate) const METRIX_INT_34: u32 = 0x34;

/// Non-zero while the metrix is idle (nothing to bank).
pub(crate) const METRIX_INT_IDLE: u32 = 0x70;

/// `float RiderMetrix::InterruptTrick(int a, int b)` (0x119e38): keeps the
/// current tricking flag (+0x20) and +0x28 as the previous ones (+0xc,
/// +0x10). When idle (+0x70 set) it only clears +0x34, sets the timer +0x24
/// to -1 and clears +0x20; otherwise it banks the trick
/// (`FinishTrick(this, 0, 0, 0, 0, 1)`) and resets it. Then stores `a` at
/// +0x0, `b` at +0x4, clears +0x30 and returns the banked points (0 when
/// idle).
pub fn rider_metrix_interrupt_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x50);
        frame.save(c, S3, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save_fpr(c, 20, 0x60);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0);
        c.mtc1(0, 20); // points = 0.0
        let m = c.g[S0][0] as u32;
        c.g[V0][0] = c.lw(m.wrapping_add(METRIX_INT_IDLE));
        c.g[S1][0] = c.lw(m.wrapping_add(METRIX_INT_TRICKING));
        c.g[S2][0] = c.lw(m.wrapping_add(METRIX_INT_28));
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
            c.mtc1(AT, 0);
            c.swc1(20, m.wrapping_add(METRIX_INT_34));
            c.swc1(0, m.wrapping_add(METRIX_INT_TIMER));
            c.sw(m.wrapping_add(METRIX_INT_TRICKING), 0);
        } else {
            c.cycles += 10;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            c.g[T1][0] = 1;
            Callee {
                addr: 0x0011_a228,
                f: ee::s00::RiderMetrix_FinishTrick,
            }
            .call(c, 0x0011_9eac)?;
            c.mov_s(20, 0);
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x0011_7838,
                f: ee::s00::RiderMetrix_ResetTrick,
            }
            .call(c, 0x0011_9eb8)?;
        }
        c.cycles += 15;
        let m = c.g[S0][0] as u32;
        c.sw(m.wrapping_add(METRIX_INT_PREV_TRICKING), c.g[S1][0]);
        c.mov_s(0, 20);
        c.sw(m.wrapping_add(METRIX_INT_PREV_28), c.g[S2][0]);
        c.sw(m.wrapping_add(METRIX_INT_STANCE_2), c.g[S3][0]);
        c.sw(m.wrapping_add(METRIX_INT_STANCE), c.g[S4][0]);
        c.sw(m.wrapping_add(METRIX_INT_30), 0);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// RiderMetrix trick record fields written on landing.
pub(crate) const METRIX_STANCE: u32 = 0x0;

pub(crate) const METRIX_ALTERNATE: u32 = 0x4;

pub(crate) const METRIX_STYLE: u32 = 0xc;

pub(crate) const METRIX_FLAG: u32 = 0x10;

pub(crate) const METRIX_SCORE: u32 = 0x14;

pub(crate) const METRIX_LAST_STYLE: u32 = 0x20;

pub(crate) const METRIX_LAST_STYLE_TIME: u32 = 0x24;

pub(crate) const METRIX_LAST_FLAG: u32 = 0x28;

pub(crate) const METRIX_LAST_FLAG_TIME: u32 = 0x2c;

pub(crate) const METRIX_CHAIN_ACTIVE: u32 = 0x5c;

pub(crate) const METRIX_CHAIN_TIME: u32 = 0x6c;

pub(crate) const METRIX_CHAIN: u32 = 0x70;

/// `float RiderMetrix_LandTrick(RiderMetrix* m, int stance, int alternate,
/// int style, int flag)` (0x119d40): a styled landing with a positive score
/// adds the style bonus (`gp-0x7a00`, 0.13) to the score; then the trick is
/// banked (`RiderMetrix_FinishTrick(m, stance, alternate, style, flag, 0)`)
/// and reset, and the landing's stance/alternate/style/flag are recorded
/// (style and flag also as "last" with their timers cleared). A trick chain
/// (`+0x70`) running before the landing survives the reset and is marked
/// active again. Returns FinishTrick's points.
pub fn rider_metrix_land_trick(c: &mut Cpu) {
    use land_trick_callee as f;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save(c, S3, 0x30);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x70);
        c.g[S0][0] = c.g[A0][0]; // m
        c.g[S1][0] = c.g[A3][0]; // style
        c.g[S2][0] = c.g[T0][0]; // flag
        c.g[S4][0] = c.g[A1][0]; // stance
        c.g[S5][0] = c.g[A2][0]; // alternate
        let m = r32(c, S0);

        // Style bonus on a positive score.
        if c.g[S1][0] == 0 {
            c.cycles += 1;
        } else {
            c.cycles += 6;
            c.lwc1(1, m + METRIX_SCORE);
            c.mtc1(0, 0);
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.cycles += 4;
                c.lwc1(0, r32(c, GP).wrapping_sub(0x7a00));
                c.add_s(0, 1, 0);
                c.swc1(0, m + METRIX_SCORE);
            }
        }
        c.g[A0][0] = c.g[S0][0];

        c.cycles += 15;
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S5][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[T0][0] = c.g[S2][0];
        c.g[T1][0] = 0; // takeoff
        c.g[S3][0] = Addr(m + METRIX_CHAIN).lw(c);
        f::FINISH_TRICK.call(c, 0x0011_9dc0)?;
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S0][0];
        f::RESET_TRICK.call(c, 0x0011_9dcc)?;

        let m = r32(c, S0);
        Addr(m + METRIX_STANCE).sw(c, c.g[S4][0]);
        Addr(m + METRIX_ALTERNATE).sw(c, c.g[S5][0]);
        Addr(m + METRIX_STYLE).sw(c, c.g[S1][0]);
        Addr(m + METRIX_FLAG).sw(c, c.g[S2][0]);
        if c.g[S1][0] != 0 {
            c.cycles += 2;
            Addr(m + METRIX_LAST_STYLE).sw(c, c.g[S1][0]);
            Addr(m + METRIX_LAST_STYLE_TIME).sw(c, 0);
        }
        c.cycles += 2;
        if c.g[S2][0] != 0 {
            c.cycles += 2;
            Addr(m + METRIX_LAST_FLAG).sw(c, c.g[S2][0]);
            Addr(m + METRIX_LAST_FLAG_TIME).sw(c, 0);
        }
        c.cycles += 2;
        c.g[V0][0] = 1;
        if c.g[S3][0] != 0 {
            c.cycles += 3;
            Addr(m + METRIX_CHAIN).sw(c, c.g[S3][0]);
            Addr(m + METRIX_CHAIN_ACTIVE).sw(c, 1);
            Addr(m + METRIX_CHAIN_TIME).sw(c, 0);
        }

        c.cycles += 11;
        c.mov_s(0, 20);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// RiderMetrix fields `RiderMetrix_AddTrick` updates.
pub(crate) const METRIX_TRICK_RATE: u32 = 0x3c;

pub(crate) const METRIX_TRICK_TIMER: u32 = 0x40;

/// `float RiderMetrix_AddTrick(RiderMetrix* m, int id)` (0x119708): look the
/// trick up in the BE trick table (interface 6): its points times 0.0001
/// (`gp-0x7a1c`) are added to the running score (`+0x14`), its second
/// value times 1/600000 (`gp-0x7a18`) becomes the rate at `+0x3c`; a
/// negative timer at `+0x40` is clamped to 0, an uber trick (id >= 35) sets
/// the chain flag `+0x5c`, then `+0xa4` is cleared. Returns 0.
pub fn rider_metrix_add_trick(c: &mut Cpu) {
    use add_trick_callee as f;
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save_fpr(c, 21, 0x48);
        frame.save_fpr(c, 20, 0x40);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0]; // m
        c.g[S2][0] = c.g[A1][0]; // id
        call_ptr(c, f::BE_GET_BE, 0x0011_9730)?;
        c.mtc1(0, 21); // zero
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 6;
        call_ptr(c, f::BE_GET_INTERFACE, 0x0011_9740)?;
        c.g[S0][0] = c.g[V0][0]; // trick table interface
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, f::GET_TRICK_POINTS, 0x0011_9750)?;
        c.mtc1(2, 20);
        c.cvt_s_w(20, 20);
        c.lwc1(0, r32(c, GP).wrapping_sub(0x7a1c));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        c.mul_s(20, 20, 0); // points * 0.0001
        call_ptr(c, f::GET_TRICK_POINTS_2, 0x0011_976c)?;

        let m = r32(c, S1);
        c.mtc1(2, 1);
        c.cvt_s_w(1, 1);
        c.lwc1(0, m + METRIX_SCORE);
        c.lwc1(2, r32(c, GP).wrapping_sub(0x7a18));
        c.lwc1(3, m + METRIX_TRICK_TIMER);
        c.add_s(0, 0, 20);
        c.mul_s(1, 1, 2);
        c.c_lt_s(3, 21);
        c.swc1(0, m + METRIX_SCORE);
        c.swc1(1, m + METRIX_TRICK_RATE);
        if c.fcc() {
            c.cycles += 1;
            c.swc1(21, m + METRIX_TRICK_TIMER);
        }

        c.cycles += 3;
        let uber = (c.g[S2][0] as i64) >= 35;
        c.g[V0][0] = 1;
        if uber {
            c.cycles += 1;
            Addr(m + METRIX_CHAIN_ACTIVE).sw(c, 1);
        }

        c.cycles += 11;
        c.g[A0][0] = c.g[S1][0];
        f::CLEAR_PEAK_A4.call(c, 0x0011_97b4)?;
        c.mov_s(0, 21);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderMetrix_BankCombo (0x117718)

/// RiderMetrix -> tricks in the running combo.
pub(crate) const METRIX_COMBO_COUNT: u32 = 0x9c;

/// RiderMetrix -> points of the running combo.
pub(crate) const METRIX_COMBO_POINTS: u32 = 0xa0;

/// RiderMetrix -> tricks banked over all combos.
pub(crate) const METRIX_BANKED_TRICKS: u32 = 0x134;

/// RiderMetrix -> longest combo (tricks).
pub(crate) const METRIX_BEST_COMBO_COUNT: u32 = 0x184;

/// RiderMetrix -> best combo (points).
pub(crate) const METRIX_BEST_COMBO_POINTS: u32 = 0x188;

/// RiderMetrix -> points banked over all combos.
pub(crate) const METRIX_BANKED_POINTS: u32 = 0x198;

/// The event settings (`mode` byte +0x48, `online` byte +0x49).
pub(crate) const BANK_COMBO_EVENT_SETTINGS: u32 = 0x0053_5bc8;

pub(crate) const BANK_COMBO_SHOW_HUD_NUMBER: Callee = Callee {
    addr: 0x0011_7b88,
    f: ee::s00::RiderMetrix_ShowHUDNumber,
};

pub(crate) const BANK_COMBO_AWARD_POINTS: Callee = Callee {
    addr: 0x0011_9ef8,
    f: ee::s00::RiderMetrix_AwardPoints,
};

pub(crate) const BANK_COMBO_RESET_COMBO: Callee = Callee {
    addr: 0x0011_75f8,
    f: ee::s00::RiderMetrix_ResetCombo,
};

/// `void RiderMetrix::BankCombo()` (0x117718): a combo of two or more
/// tricks with positive points is banked: points and trick count are added
/// to the totals, the best-combo records updated, and the back end's
/// interface 0 is touched; then either the combo number (HUD id 0x26, 1.5 s,
/// with the trick count) and the points (id 0x18, gp-0x7b10 s) are shown,
/// or, offline in mode 4, `AwardPoints(2, min(points / 500, 20))`. The combo
/// is always reset (`ResetCombo`). Frame 0x20 (s0).
pub fn rider_metrix_bank_combo(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let m = Addr(c.g[S0][0] as u32);
        c.g[A2][0] = m.off(METRIX_COMBO_COUNT).lw(c);
        c.g[V0][0] = ((c.g[A2][0] as i64) < 2) as u64;
        'bank: {
            if c.g[V0][0] != 0 {
                break 'bank;
            }
            c.cycles += 3;
            c.g[A1][0] = m.off(METRIX_COMBO_POINTS).lw(c);
            if (c.g[A1][0] as i64) <= 0 {
                break 'bank;
            }
            // Bank the combo into the totals and records.
            c.cycles += 9;
            c.g[V0][0] = m.off(METRIX_BANKED_POINTS).lw(c);
            c.g[V1][0] = m.off(METRIX_BANKED_TRICKS).lw(c);
            c.g[A0][0] = m.off(METRIX_BEST_COMBO_POINTS).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
            let total = c.g[V0][0];
            m.off(METRIX_BANKED_POINTS).sw(c, total);
            c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let tricks = c.g[V1][0];
            m.off(METRIX_BANKED_TRICKS).sw(c, tricks);
            if c.g[A0][0] != 0 {
                c.cycles += 1;
                let pts = c.g[A1][0];
                m.off(METRIX_BEST_COMBO_POINTS).sw(c, pts);
            }
            c.cycles += 5;
            c.g[V1][0] = m.off(METRIX_COMBO_COUNT).lw(c);
            c.g[V0][0] = m.off(METRIX_BEST_COMBO_COUNT).lw(c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                let count = c.g[V1][0];
                m.off(METRIX_BEST_COMBO_COUNT).sw(c, count);
            }

            // cBE_getBE()->GetInterface(0), result unused.
            c.cycles += 11;
            call_ptr(c, 0x0014_dc80, 0x0011_7788)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_dd58, 0x0011_7794)?;

            // Offline in mode 4?
            c.g[V1][0] = sx(BANK_COMBO_EVENT_SETTINGS);
            c.g[A0][0] = 4;
            c.g[V0][0] = Addr(BANK_COMBO_EVENT_SETTINGS + 0x48).lb(c);
            c.g[A1][0] = 0;
            if c.g[V0][0] == c.g[A0][0] {
                c.cycles += 2;
                c.g[V0][0] = Addr(BANK_COMBO_EVENT_SETTINGS + 0x49).lb(c);
                c.g[A1][0] = (c.g[V0][0] < 1) as u64;
            }
            c.cycles += 2;
            let s0 = c.g[S0][0] as u32;
            c.g[A2][0] = Addr(s0.wrapping_add(METRIX_COMBO_POINTS)).lw(c);
            if c.g[A1][0] == 0 {
                // Show the combo count and the points on the HUD.
                c.cycles += 14;
                c.g[A0][0] = c.g[S0][0];
                c.g[AT][0] = 0x3fc0_0000;
                c.f[12] = 0x3fc0_0000;
                c.g[A1][0] = 0x26;
                c.g[A3][0] = Addr(s0.wrapping_add(METRIX_COMBO_COUNT)).lw(c);
                BANK_COMBO_SHOW_HUD_NUMBER.call(c, 0x0011_77d4)?;
                let s0 = c.g[S0][0] as u32;
                c.g[A2][0] = Addr(s0.wrapping_add(METRIX_COMBO_POINTS)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                let gp = c.g[GP][0] as u32;
                c.f[12] = Addr(gp.wrapping_sub(0x7b10)).read_u32(c);
                c.g[A1][0] = 0x18;
                c.g[A3][0] = 0;
                BANK_COMBO_SHOW_HUD_NUMBER.call(c, 0x0011_77ec)?;
            } else {
                // Award min(points / 500, 20) as an offline bonus.
                c.cycles += 3;
                c.g[V0][0] = 0x1f4;
                c.cycles += 8;
                c.g[V1][0] = 0x14;
                c.div(0, A2, V0);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 2;
                c.g[A2][0] = c.lo[0];
                c.g[V0][0] = ((c.g[A2][0] as i64) < 21) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A2][0] = c.g[V1][0];
                }
                BANK_COMBO_AWARD_POINTS.call(c, 0x0011_7820)?;
            }
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        BANK_COMBO_RESET_COMBO.call(c, 0x0011_7828)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod combo_points {
    use crate::call::Callee;
    use ssx3_ee as ee;

    /// RiderMetrix: +0x9c combo count, +0xa0 combo score, +0xa4 peak (-1:
    /// reset), +0x1b0 the HUD effects object.
    pub const COMBO_COUNT: u32 = 0x9c;

    pub const COMBO_SCORE: u32 = 0xa0;

    pub const PEAK: u32 = 0xa4;

    pub const HUD: u32 = 0x1b0;

    /// HUD effects: +0x1d4 the combo-number effect (its first word the
    /// state, 0x34 while showing), +0x1e8 its current value.
    pub const HUD_COMBO_FX: u32 = 0x1d4;

    pub const HUD_COMBO_VALUE: u32 = 0x1e8;

    pub const FX_SHOWING: u64 = 0x34;

    /// `gp-0x7b14`: 0.05, the multiplier step per combo.
    pub const GP_STEP: u32 = 0x7b14;

    pub const SHOW_NUMBER_DELAYED: Callee = Callee {
        addr: 0x0011_71a8,
        f: ee::s00::cHUDFX_ShowNumberDelayed,
    };
}

/// `void RiderMetrix_AddComboPoints(RiderMetrix* self, int points)`
/// (0x117638): for positive points, bump the combo count `n`, add `points *
/// clamp((n + 10) * 0.05, 0.5, 2.0)` (truncated) to the combo score, reset
/// the peak to -1, and show the combo count on the HUD (type 3, with the
/// effect's value unless it is already showing, no delay).
pub fn rider_metrix_add_combo_points(c: &mut Cpu) {
    use combo_points as k;
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        if (c.g[A1][0] as i64) > 0 {
            c.cycles += 18;
            let this = abi::arg(c, 0);
            let gp = r32(c, GP);
            // f0 = (n + 10) * 0.05 with n the new combo count (a3)
            c.g[V0][0] = Addr(this.wrapping_add(k::COMBO_COUNT)).lw(c);
            c.g[AT][0] = 0x4120_0000; // 10.0
            c.mtc1(AT, 3);
            c.g[A3][0] = sx(r32(c, V0).wrapping_add(1));
            c.lwc1(2, gp.wrapping_sub(k::GP_STEP));
            c.mtc1(A3, 0);
            c.cvt_s_w(0, 0);
            c.g[AT][0] = 0x3f00_0000; // 0.5
            c.mtc1(AT, 1);
            Addr(this.wrapping_add(k::COMBO_COUNT)).sw(c, c.g[A3][0]);
            c.g[AT][0] = 0x4000_0000; // 2.0
            c.mtc1(AT, 4);
            c.add_s(0, 0, 3);
            c.mul_s(0, 0, 2);
            // f5 = clamp(f0, 0.5, 2.0)
            c.c_le_s(1, 0);
            c.mov_s(5, 1);
            if c.fcc() {
                c.cycles += 1;
                c.min_s(5, 0, 4);
            }
            // score += (int)(points * f5); peak = -1
            c.cycles += 13;
            c.mtc1(A1, 0);
            c.cvt_s_w(0, 0);
            c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0
            c.mtc1(AT, 1);
            c.g[V1][0] = Addr(this.wrapping_add(k::COMBO_SCORE)).lw(c);
            c.swc1(1, this.wrapping_add(k::PEAK));
            c.mul_s(0, 0, 5);
            c.g[A1][0] = Addr(this.wrapping_add(k::HUD)).lw(c);
            c.cvt_w_s(1, 0);
            c.mfc1(V0, 1);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            Addr(this.wrapping_add(k::COMBO_SCORE)).sw(c, c.g[V1][0]);
            if c.g[A1][0] != 0 {
                c.cycles += 5;
                let hud = r32(c, A1);
                c.g[V0][0] = Addr(hud.wrapping_add(k::HUD_COMBO_FX)).lw(c) ^ k::FX_SHOWING;
                c.g[A2][0] = 0;
                c.g[A0][0] = sx(hud.wrapping_add(k::HUD_COMBO_FX));
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    c.g[A2][0] = Addr(hud.wrapping_add(k::HUD_COMBO_VALUE)).lw(c);
                }
                c.cycles += 3;
                c.mtc1(0, 12);
                c.g[A1][0] = 3;
                k::SHOW_NUMBER_DELAYED.call(c, 0x0011_76e8)?;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------- RiderMetrix_AddTrickHistory

/// RiderMetrix: +0x60 the three most recent trick ids (0: empty).
pub(crate) const TRICK_HISTORY: u32 = 0x60;

/// The loop head of `RiderMetrix_AddTrickHistory` (the back-edge event pc).
pub(crate) const TRICK_HISTORY_LOOP: u32 = 0x0011_908c;

/// `void RiderMetrix_AddTrickHistory(RiderMetrix* self, int trick)`
/// (0x119068): record `trick` in the first empty one of the three recent
/// slots at `+0x60`; a repeat of the last recorded trick (the slot holding
/// it is followed by an empty one) is stored over itself instead. A full
/// history is left unchanged.
pub fn rider_metrix_add_trick_history(c: &mut Cpu) {
    c.cycles += 3;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this.wrapping_add(TRICK_HISTORY)).lw(c);
    c.g[T1][0] = 0;
    if c.g[V0][0] == 0 {
        // empty history: the trick goes into the first slot
        c.cycles += 2;
        Addr(this.wrapping_add(TRICK_HISTORY)).sw(c, c.g[A1][0]);
        ret(c);
        return;
    }
    c.cycles += 1;
    c.g[V0][0] = sx(this.wrapping_add(TRICK_HISTORY));
    // t1 = slot index, t0 = the history, a3 = &history[t1], a2 = t1 + 1
    loop {
        c.cycles += 6;
        c.g[V1][0] = sx(r32(c, T1) << 2);
        c.g[A3][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[T0][0] = c.g[V0][0];
        c.g[V1][0] = Addr(r32(c, A3)).lw(c);
        c.g[A2][0] = sx(r32(c, T1).wrapping_add(1));
        if c.g[V1][0] == c.g[A1][0] {
            // the same trick again: if the next slot is empty, store over it
            c.cycles += 3;
            let not_last = (c.g[T1][0] as i64) < 2;
            c.g[V0][0] = sx(r32(c, A2) << 2);
            if not_last {
                c.cycles += 4;
                c.g[V0][0] = sx(r32(c, T0).wrapping_add(r32(c, V0)));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                if c.g[V1][0] == 0 {
                    c.cycles += 2;
                    Addr(r32(c, A3)).sw(c, c.g[A1][0]);
                    ret(c);
                    return;
                }
            }
        }
        c.cycles += 3;
        let more = (c.g[A2][0] as i64) < 3;
        c.g[V0][0] = more as u64;
        c.g[T1][0] = c.g[A2][0];
        if !more {
            break;
        }
        c.cycles += 5;
        c.g[V0][0] = sx(r32(c, T0).wrapping_add(r32(c, A2) << 2));
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        if c.g[V1][0] == 0 {
            // the next slot is free: record the trick there
            c.cycles += 1;
            Addr(r32(c, V0)).sw(c, c.g[A1][0]);
            break;
        }
        c.g[V0][0] = sx(r32(c, A0).wrapping_add(TRICK_HISTORY));
        if event_due(c) {
            deliver_event_here(c, TRICK_HISTORY_LOOP);
        }
    }
    c.cycles += 2;
    ret(c);
}

/// The bonus-trick table (24 entries of 16 bytes): +0 grade (stored into
/// the trick id's top 5 bits), +4 points, +8..+0xe the trick-id fields to
/// match.
pub(crate) const BONUS_TRICKS: u32 = 0x0043_d608;

pub(crate) const BONUS_TRICK_COUNT: i64 = 24;

pub(crate) const BONUS_TRICK_SIZE: u32 = 0x10;

pub(crate) const BONUS_TRICK_GRADE: u32 = 0x0;

pub(crate) const BONUS_TRICK_POINTS: u32 = 0x4;

/// `EA_MemSet(dst, value, n)` (0x3e6448, through the function table).
pub(crate) const BONUS_TRICK_MEM_SET: u32 = 0x003e_6448;

/// The bonus-table loop head (event checks resume there).
pub(crate) const BONUS_TRICK_LOOP: u32 = 0x0011_b1d8;

/// How one bonus-table entry compared with the trick id.
pub(crate) enum BonusTrickCmp {
    /// A field before the last one differed.
    Miss,
    /// Only the last field (word 1 bits 11..17) differed.
    LastMiss,
    /// Every field matched.
    Hit,
}

/// Compare the entry at s0 with the trick id at s1 (a1 counts entries).
pub(crate) fn bonus_trick_compare(c: &mut Cpu) -> BonusTrickCmp {
    c.cycles += 6;
    let entry = r32(c, S0);
    let w0 = Addr(r32(c, S1)).lw(c);
    c.g[A0][0] = w0;
    // Word 0: bits 10..11, 12..15, 16..18, 19..21 against bytes +8..+0xb.
    for (i, &(shift, mask, off)) in [
        (10, 0x3, 0x8),
        (12, 0xf, 0x9),
        (16, 0x7, 0xa),
        (19, 0x7, 0xb),
    ]
    .iter()
    .enumerate()
    {
        if i > 0 {
            c.cycles += 5;
        }
        c.g[V1][0] = Addr(entry.wrapping_add(off)).lbu(c);
        c.g[V0][0] = sx((w0 as u32) >> shift) & mask;
        if c.g[V0][0] != c.g[V1][0] {
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
            return BonusTrickCmp::Miss;
        }
    }
    // Bits 28..31 against +0xc (the counter steps either way).
    c.cycles += 4;
    c.g[V1][0] = Addr(entry.wrapping_add(0xc)).lbu(c);
    c.g[V0][0] = sx((w0 as u32) >> 28);
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
    if c.g[V0][0] != c.g[V1][0] {
        return BonusTrickCmp::Miss;
    }
    // Word 1: bits 3..9 against +0xd, bits 11..17 against +0xe.
    c.cycles += 6;
    let w1 = Addr(r32(c, S1).wrapping_add(4)).lw(c);
    c.g[A0][0] = w1;
    c.g[V1][0] = Addr(entry.wrapping_add(0xd)).lbu(c);
    c.g[V0][0] = sx((w1 as u32) >> 3) & 0x7f;
    if c.g[V0][0] != c.g[V1][0] {
        return BonusTrickCmp::Miss;
    }
    c.cycles += 5;
    c.g[V0][0] = sx((w1 as u32) >> 11) & 0x7f;
    c.g[V1][0] = Addr(entry.wrapping_add(0xe)).lbu(c);
    if c.g[V0][0] != c.g[V1][0] {
        return BonusTrickCmp::LastMiss;
    }
    BonusTrickCmp::Hit
}

/// `int Trick_LookupBonusTrick(RiderMetrix *self, sTrickId *id)` (0x11b1a8):
/// look the trick id up in the 24-entry bonus-trick table; on a match clear
/// the id's first 8 bytes, store the entry's grade into the id's top 5 bits
/// and return the entry's points; 0 when no entry matches.
pub fn trick_lookup_bonus_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x44_0000;
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(BONUS_TRICKS);
        c.g[A1][0] = 0;
        c.g[S2][0] = 0x07ff_ffff;
        loop {
            match bonus_trick_compare(c) {
                BonusTrickCmp::Hit => {
                    c.cycles += 12;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0x8;
                    call_ptr(c, BONUS_TRICK_MEM_SET, 0x0011_b278)?;
                    let entry = r32(c, S0);
                    let id = r32(c, S1);
                    let grade = Addr(entry.wrapping_add(BONUS_TRICK_GRADE)).lw(c);
                    let w1 = Addr(id.wrapping_add(4)).lw(c);
                    c.g[V1][0] = sx((grade as u32) << 27);
                    let w1 = (w1 & c.g[S2][0]) | c.g[V1][0];
                    c.g[V0][0] = w1;
                    Addr(id.wrapping_add(4)).sw(c, w1);
                    c.g[V0][0] = Addr(entry.wrapping_add(BONUS_TRICK_POINTS)).lw(c);
                    break;
                }
                cmp => {
                    if let BonusTrickCmp::Miss = cmp {
                        c.cycles += 1;
                    }
                    c.g[V0][0] = ((c.g[A1][0] as i64) < BONUS_TRICK_COUNT) as u64;
                }
            }
            c.cycles += 2;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(BONUS_TRICK_SIZE));
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 0;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, BONUS_TRICK_LOOP);
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

/// `RiderMetrix` fields: the running trick's score +0x40 (-1 when none),
/// the score total +0x44, the best single score +0x48, completed-trick
/// counters (small +0x4c, medium +0x50, big +0x54, big while the rider's
/// level is >= 6 +0x58), a combo flag +0x5c, the rider +0x1ac.
pub(crate) const METRIX_TRICK_SCORE: u32 = 0x40;

pub(crate) const METRIX_SCORE_TOTAL: u32 = 0x44;

pub(crate) const METRIX_BEST_SCORE: u32 = 0x48;

pub(crate) const METRIX_SMALL_TRICKS: u32 = 0x4c;

pub(crate) const METRIX_MEDIUM_TRICKS: u32 = 0x50;

pub(crate) const METRIX_BIG_TRICKS: u32 = 0x54;

pub(crate) const METRIX_BIG_TRICKS_HIGH: u32 = 0x58;

pub(crate) const METRIX_COMBO_FLAG: u32 = 0x5c;

pub(crate) const METRIX_RIDER: u32 = 0x1ac;

/// The rider's level field and the level from which big tricks also
/// count in +0x58.
pub(crate) const METRIX_RIDER_LEVEL: u32 = 0x2f4;

pub(crate) const METRIX_HIGH_LEVEL: i64 = 6;

/// Trick ids from 19 are medium, from 35 big.
pub(crate) const METRIX_MEDIUM_ID: i64 = 19;

pub(crate) const METRIX_BIG_ID: i64 = 35;

/// `RiderMetrix_AddTrickHistory(metrix, id)` (0x119068).
pub(crate) const METRIX_ADD_TRICK_HISTORY: Callee = Callee {
    addr: 0x0011_9068,
    f: ee::s00::f_00119068,
};

/// `float RiderMetrix_CompleteTrick(RiderMetrix *m, int id)` (0x1197d8):
/// record the trick in the history, count it as small / medium / big (big
/// ones also in +0x58 when the rider's level is at least 6), then fold the
/// running score into the best score and the total, reset it to -1, clear
/// the combo flag; returns 0.0.
pub fn rider_metrix_complete_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        METRIX_ADD_TRICK_HISTORY.call(c, 0x0011_97f4)?;
        let m = r32(c, S0);
        let id = c.g[S1][0] as i64;
        c.g[V0][0] = (id < METRIX_MEDIUM_ID) as u64;
        // Bump the counter at m + off (v0 = the new count).
        let bump = |c: &mut Cpu, off: u32| {
            let n = sx((Addr(m.wrapping_add(off)).lw(c) as u32).wrapping_add(1));
            c.g[V0][0] = n;
            Addr(m.wrapping_add(off)).sw(c, n);
        };
        let mut joined = true;
        if id >= METRIX_BIG_ID {
            c.cycles += 8;
            let rider = Addr(m.wrapping_add(METRIX_RIDER)).lw(c);
            c.g[A0][0] = rider;
            bump(c, METRIX_BIG_TRICKS);
            let level = Addr((rider as u32).wrapping_add(METRIX_RIDER_LEVEL)).lw(c) as i64;
            c.g[V1][0] = (level < METRIX_HIGH_LEVEL) as u64;
            if level < METRIX_HIGH_LEVEL {
                joined = false;
            } else {
                c.cycles += 4;
                bump(c, METRIX_BIG_TRICKS_HIGH);
            }
        } else {
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                bump(c, METRIX_SMALL_TRICKS);
            } else {
                c.cycles += 4;
                bump(c, METRIX_MEDIUM_TRICKS);
            }
        }
        if joined {
            c.cycles += 1;
        }
        let score = Addr(m.wrapping_add(METRIX_TRICK_SCORE)).read_fpu(c);

        c.cycles += 16;
        let total = Addr(m.wrapping_add(METRIX_SCORE_TOTAL)).read_fpu(c);
        let best = Addr(m.wrapping_add(METRIX_BEST_SCORE)).read_fpu(c);
        c.f[2] = best.0;
        let total = total + score;
        c.f[1] = total.0;
        // -1.0
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.f[3] = 0xbf80_0000;
        let best = score.max(best);
        Addr(m.wrapping_add(METRIX_COMBO_FLAG)).sw(c, 0);
        Addr(m.wrapping_add(METRIX_BEST_SCORE)).write_fpu(c, best);
        Addr(m.wrapping_add(METRIX_SCORE_TOTAL)).write_fpu(c, total);
        Addr(m.wrapping_add(METRIX_TRICK_SCORE)).write_u32(c, 0xbf80_0000);
        c.f[0] = 0;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderMetrix_CountTrickRepeats (0x1190f0)

/// Rider metrics: the 10-entry history of recent trick ids (8 bytes each).
pub(crate) const COUNT_TRICK_REPEATS_HIST: u32 = 0xa8;

/// Rider metrics: next history slot (0..9).
pub(crate) const COUNT_TRICK_REPEATS_HIST_IDX: u32 = 0xf8;

/// Rider metrics: the state words that suspend trick counting when set.
pub(crate) const COUNT_TRICK_REPEATS_BUSY: [u32; 5] = [0x08, 0x20, 0x70, 0x7c, 0x28];

/// `int RiderMetrix_CountTrickRepeats(RiderMetrix* self, sTrickId* id)`
/// (0x1190f0): how often the 8-byte trick id already appears in the
/// rider's 10-entry recent-trick history (0 for a few trick classes that
/// never count as repeats), then records it at the history cursor and
/// advances the cursor modulo 10. Returns 0 without recording while any of
/// the busy state words is set.
pub fn rider_metrix_count_trick_repeats(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A3][0] = c.g[A0][0];
    c.g[T3][0] = c.g[A1][0];
    let me = c.g[A3][0] as u32;
    let id = c.g[T3][0] as u32;
    let mut busy = 0u64;
    for (k, off) in COUNT_TRICK_REPEATS_BUSY.into_iter().enumerate() {
        if k > 0 {
            c.cycles += 3;
        }
        busy = Addr(me.wrapping_add(off)).lw(c);
        if busy != 0 {
            break;
        }
    }
    if busy != 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }

    // Trick classes that are never counted as repeats.
    c.cycles += 7;
    let word1 = Addr(id.wrapping_add(4)).lw(c);
    c.g[V1][0] = 0x3f800;
    c.g[T2][0] = 0; // repeats found
    c.g[V0][0] = word1 & 0x3f800;
    c.g[A0][0] = 0x800;
    c.g[T5][0] = Addr(me.wrapping_add(COUNT_TRICK_REPEATS_HIST_IDX)).lw(c);
    if word1 & 0x3f800 != 0x800 {
        c.cycles += 6;
        let word0 = Addr(id).lw(c);
        c.g[A0][0] = 0x400000;
        c.g[V0][0] = word0 & 0xfc0_0000;
        c.g[V1][0] = 0; // history index
        if word0 & 0xfc0_0000 != 0x40_0000 {
            c.cycles += 3;
            c.g[T4][0] = u64::MAX;
            c.g[V0][0] = 0; // index * 8
            count_trick_repeats_scan(c);
        }
    }

    // Record the id at the cursor; cursor = (cursor + 1) % 10.
    c.cycles += 14;
    let me = c.g[A3][0] as u32;
    let src = c.g[T3][0] as u32;
    let entry = ((c.g[T5][0] as u32) << 3).wrapping_add(me);
    c.g[A0][0] = sx(entry);
    c.g[A1][0] = 10;
    c.g[V0][0] = c.g[T2][0];
    let trick = Addr(src).read_u64_unaligned(c);
    c.g[V1][0] = trick;
    Addr(entry.wrapping_add(COUNT_TRICK_REPEATS_HIST)).write_u64_unaligned(c, trick);
    let cursor = (Addr(me.wrapping_add(COUNT_TRICK_REPEATS_HIST_IDX)).lw(c) as u32).wrapping_add(1);
    c.g[V1][0] = sx(cursor);
    abi::div(c, cursor, 10);
    Addr(me.wrapping_add(COUNT_TRICK_REPEATS_HIST_IDX)).sw(c, sx(cursor));
    c.cycles += 3;
    let wrapped = c.hi[0];
    c.g[A0][0] = wrapped;
    Addr(me.wrapping_add(COUNT_TRICK_REPEATS_HIST_IDX)).sw(c, wrapped);
    abi::ret(c);
}

/// The repeat scan of [`rider_metrix_count_trick_repeats`]: byte-compares
/// the id (t3) with each of the 10 history entries, counting matches in t2.
/// Works on the registers (v1 = index, v0 = index * 8, a3 = self) so the
/// back-edge event checks see the original's state.
pub(crate) fn count_trick_repeats_scan(c: &mut Cpu) {
    loop {
        c.cycles += 6;
        c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[A1][0] = c.g[T3][0];
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(COUNT_TRICK_REPEATS_HIST));
        c.g[A2][0] = 7;
        c.g[T0][0] = sx((c.g[T2][0] as u32).wrapping_add(1));
        // 8-byte compare (a2 counts 7..-1).
        loop {
            c.cycles += 4;
            let a = c.lbu(c.g[A1][0] as u32);
            let b = c.lbu(c.g[A0][0] as u32);
            c.g[V1][0] = a;
            c.g[V0][0] = 0;
            if a != b {
                break;
            }
            c.cycles += 4;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            if c.g[A2][0] != c.g[T4][0] {
                if event_due(c) {
                    deliver_event_here(c, 0x0011_9190);
                }
                continue;
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
            break;
        }
        c.cycles += 5;
        c.g[V1][0] = c.g[T1][0];
        if c.g[V0][0] != 0 {
            c.g[T2][0] = c.g[T0][0];
        }
        let more = (c.g[V1][0] as i64) < 10;
        c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0011_9178);
        }
    }
}

// ---------------------------------------------------------------------------
// RiderMetrix_ShowHUDText (0x117ae8).

/// `RiderMetrix` field: the HUD message slot table (0: no messages).
pub(crate) const RIDER_METRIX_HUD_TABLE: u32 = 0x1b0;

/// A HUD message slot is 0x9c bytes.
pub(crate) const RIDER_METRIX_HUD_SLOT_SIZE: u32 = 0x9c;

/// `RiderMetrix_FindHUDSlot(RiderMetrix*, int type)`.
pub(crate) const RIDER_METRIX_FIND_HUD_SLOT: Callee = Callee {
    addr: 0x0011_7a58,
    f: ee::s00::f_00117a58,
};

/// `cHUDFX_Show(fx, int type, const char *text, int style, float time)`.
pub(crate) const C_HUDFX_SHOW: Callee = Callee {
    addr: 0x0011_7048,
    f: ee::s00::f_00117048,
};

/// `int RiderMetrix_ShowHUDText(RiderMetrix* self, int type, const char*
/// text, int style, float time)` (0x117ae8): show a HUD message in the slot
/// `RiderMetrix_FindHUDSlot` picks, and return that slot index (-1 when the
/// rider has no message table).
pub fn rider_metrix_show_hud_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save_fpr(c, 20, 0x60);
        c.g[S4][0] = c.g[A3][0];
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(RIDER_METRIX_HUD_TABLE)).lw(c);
        let has_table = c.g[V0][0] != 0;
        c.f[20] = c.f[12]; // the decay time
        if !has_table {
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
        } else {
            c.cycles += 14;
            RIDER_METRIX_FIND_HUD_SLOT.call(c, 0x0011_7b2c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = RIDER_METRIX_HUD_SLOT_SIZE as u64;
            c.mult(0, A0, S0, A0); // slot index * slot size
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(RIDER_METRIX_HUD_TABLE)).lw(c);
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            c.f[12] = c.f[20];
            c.g[A3][0] = c.g[S4][0];
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            C_HUDFX_SHOW.call(c, 0x0011_7b54)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// `RiderMetrix_RemoveHUDMessage` (0x1179e0).
pub(crate) const RIDER_METRIX_REMOVE_HUD: u32 = 0x0011_79e0;

/// `void cRiderMetrix::resetComboChain(cRiderMetrix* this)` (0x1175f8):
/// removes the combo HUD message (id 3), then clears the combo chain count
/// (+0x9c), the combo points (+0xa0) and the combo timer (+0xa4 = -1.0).
pub fn rider_metrix_reset_combo(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x3;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, RIDER_METRIX_REMOVE_HUD, 0x0011_7610)?;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 0);
        let this = r32(c, S0);
        Addr(this).off(0x9c).sw(c, 0);
        c.swc1(0, this.wrapping_add(0xa4));
        Addr(this).off(0xa0).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderMetrix_ShowTrickName (0x118ff8)

/// `Trick_BuildName(out, id)` (0x116950).
pub(crate) const OC320_TRICK_BUILD_NAME: Callee = Callee {
    addr: 0x0011_6950,
    f: ee::s00::Trick_BuildName,
};

/// `RiderMetrix_ShowHUDText(self, 0, text, 3.0f, repeat)` (0x117ae8).
pub(crate) const OC320_SHOW_HUD_TEXT: Callee = Callee {
    addr: 0x0011_7ae8,
    f: ee::s00::RiderMetrix_ShowHUDText,
};

/// `void RiderMetrix_ShowTrickName(RiderMetrix* self, TrickId* id, int
/// repeat)` (0x118ff8): build the trick's display name into a 0x100-byte
/// stack buffer and show it as a HUD effect at scale 3, then post the trick
/// to the script manager.
pub fn rider_metrix_show_trick_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x140);
        frame.save(c, S0, 0x130);
        frame.save(c, S1, 0x120);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x110);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0x100);
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = 0xff;
        OC320_TRICK_BUILD_NAME.call(c, 0x0011_9024)?;
        c.g[AT][0] = 0x40400000;
        c.mtc1(1, 12);
        c.g[A3][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = c.g[SP][0];
        OC320_SHOW_HUD_TEXT.call(c, 0x0011_9040)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0030_9918, 0x0011_904c)?;
        frame.restore(c, S0, 0x130);
        frame.restore(c, S1, 0x120);
        frame.restore(c, S2, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderMetrix_BankPendingScore (0x11a7a8)
// ---------------------------------------------------------------------------

/// The per-slot HUD numbers `RiderMetrix_BankPendingScore` refreshes, and the
/// "1.5x" size it shows them at.
pub(crate) const RIDER_METRIX_BANK_SCALE: u32 = 0x3fc0_0000;

// 1.5f

/// `int RiderMetrix_BankPendingScore(cRiderMetrix* self)` (0x11a7a8): refresh
/// the HUD numbers for the tricks landed this jump (slot 0x25), the pending
/// base score (0x27) and the banked total (0x28), each only when its value is
/// positive, and always drop the "wiped out" message (slot 4). Returns the
/// rounded trick points.
pub fn ridermetrix_bank_pending_score(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::RIDER_METRIX_ROUNDED_TRICK_POINTS.call(c, 0x0011_a7c4)?;
        c.g[S1][0] = c.g[V0][0];
        let no_tricks = (c.g[S1][0] as i64) <= 0;
        c.g[A0][0] = c.g[S0][0];
        if !no_tricks {
            c.cycles += 9;
            c.g[A1][0] = 1;
            callee::RIDER_METRIX_REMOVE_HUD_MESSAGE.call(c, 0x0011_a7d8)?;
            c.g[AT][0] = RIDER_METRIX_BANK_SCALE as u64;
            c.mtc1(1, 12);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x25;
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = 0;
            callee::RIDER_METRIX_SHOW_HUD_NUMBER_3.call(c, 0x0011_a7f4)?;
        }

        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S0)).off(0xa0).lw(c);
        let no_pending = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = c.g[S0][0];
        if !no_pending {
            c.cycles += 9;
            c.g[A1][0] = 3;
            callee::RIDER_METRIX_REMOVE_HUD_MESSAGE.call(c, 0x0011_a808)?;
            c.g[A2][0] = Addr(r32(c, S0)).off(0xa0).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[AT][0] = RIDER_METRIX_BANK_SCALE as u64;
            c.mtc1(1, 12);
            c.g[A1][0] = 0x27;
            c.g[A3][0] = 0;
            callee::RIDER_METRIX_SHOW_HUD_NUMBER_3.call(c, 0x0011_a824)?;
        }

        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_METRIX_TRICK_BONUS.call(c, 0x0011_a82c)?;
        c.g[V1][0] = Addr(r32(c, S0)).off(0x84).lw(c);
        c.g[S2][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        let no_total = (c.g[S2][0] as i64) <= 0;
        c.g[A0][0] = c.g[S0][0];
        if !no_total {
            c.cycles += 24;
            // The combo messages this total replaces.
            for (slot, ret) in [
                (0x2u64, 0x0011_a844u32),
                (0x1d, 0x0011_a850),
                (0x1c, 0x0011_a85c),
                (0x1e, 0x0011_a868),
                (0x1f, 0x0011_a874),
                (0x20, 0x0011_a880),
            ] {
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = slot;
                callee::RIDER_METRIX_REMOVE_HUD_MESSAGE.call(c, ret)?;
            }
            c.g[AT][0] = RIDER_METRIX_BANK_SCALE as u64;
            c.mtc1(1, 12);
            c.g[A2][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x28;
            c.g[A3][0] = 0;
            callee::RIDER_METRIX_SHOW_HUD_NUMBER_3.call(c, 0x0011_a89c)?;
        }

        c.cycles += 10;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 4;
        callee::RIDER_METRIX_REMOVE_HUD_MESSAGE.call(c, 0x0011_a8a8)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        c.g[RA][0] = c.ld(frame.at(c, 0));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x00117990 RiderMetrix_GetTrickScore

/// `int RiderMetrix_GetTrickScore(RiderMetrix *self)` (0x117990): the
/// accumulated score at 0.01 points a unit, rounded to whole tens:
/// `(int)(v * 10000 + 5)` floored to a multiple of 10.
pub fn rider_metrix_get_trick_score(c: &mut Cpu) {
    c.cycles += 5;
    c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x18));
    c.g[A1][0] = 0xa;
    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x1c4));
    // (A branch on the multiplier that goes both ways in the original.)
    c.cycles += 14;
    c.lwc1(2, (c.g[A0][0] as u32).wrapping_add(0x14));
    c.mul_s(0, 0, 1);
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x7b04));
    c.g[AT][0] = 0x40a0_0000;
    c.mtc1(AT, 3);
    c.mul_s(0, 0, 2);
    c.mul_s(0, 0, 4);
    c.add_s(0, 0, 3);
    c.cvt_w_s(1, 0);
    c.mfc1(V0, 1);
    c.div(0, V0, A1);
    c.g[V1][0] = c.hi[0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    ret(c);
}

// ---------------------------------------------------------------------------
// RiderMetrix_OnWipeout (0x119b08)

pub(crate) const WIPEOUT_SHOW_HUD_NUMBER: Callee = Callee {
    addr: 0x0011_7b88,
    f: ee::s00::f_00117b88,
};

pub(crate) const WIPEOUT_BANK_PENDING_SCORE: Callee = Callee {
    addr: 0x0011_a7a8,
    f: ee::s00::f_0011a7a8,
};

pub(crate) const WIPEOUT_ROUNDED_TRICKS: Callee = Callee {
    addr: 0x0011_7948,
    f: ee::s00::f_00117948,
};

pub(crate) const WIPEOUT_RESET_TRICK: Callee = Callee {
    addr: 0x0011_7838,
    f: ee::s00::f_00117838,
};

pub(crate) const WIPEOUT_RESET_COMBO: Callee = Callee {
    addr: 0x0011_75f8,
    f: ee::s00::f_001175f8,
};

/// `float RiderMetrix_OnWipeout(cRiderMetrix* m, int hudEffect)` (0x119b08):
/// count the wipeout — in the HUD effect counter `+0x12c` (and flash HUD
/// number 0x2d at 1.5x) when `hudEffect` is set, else in the per-jump
/// counter `+0x124`; add the pending trick points to `+0x1a0`; return
/// -0.25 (the wipeout penalty) when the rounded trick points are positive,
/// else 0; then reset the trick and the combo chain.
pub fn rider_metrix_on_wipeout(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let hud_effect = c.g[A1][0] != 0;
        frame.save_fpr(c, 20, 0x20);
        if hud_effect {
            c.cycles += 11;
            let count = Addr(r32(c, S0)).off(0x12c).lw(c);
            c.g[V0][0] = count;
            c.g[AT][0] = 0x3fc0_0000; // 1.5
            c.mtc1(1, 12);
            c.g[A1][0] = 0x2d;
            c.g[V0][0] = sx((count as u32).wrapping_add(1));
            c.g[A2][0] = 0;
            let at = r32(c, S0);
            Addr(at).off(0x12c).sw(c, c.g[V0][0]);
            c.g[A3][0] = 0;
            WIPEOUT_SHOW_HUD_NUMBER.call(c, 0x0011_9b44)?;
        } else {
            c.cycles += 3;
            let count = Addr(r32(c, S0)).off(0x124).lw(c);
            c.g[V0][0] = count;
            c.g[V0][0] = sx((count as u32).wrapping_add(1));
            let at = r32(c, S0);
            Addr(at).off(0x124).sw(c, c.g[V0][0]);
        }
        // The wipeout penalty: f20 stays 0 unless the trick points are
        // positive.
        c.cycles += 10;
        c.mtc1(0, 20);
        c.g[A0][0] = c.g[S0][0];
        WIPEOUT_BANK_PENDING_SCORE.call(c, 0x0011_9b64)?;
        c.g[V1][0] = Addr(r32(c, S0)).off(0x1a0).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let at = r32(c, S0);
        Addr(at).off(0x1a0).sw(c, c.g[V1][0]);
        WIPEOUT_ROUNDED_TRICKS.call(c, 0x0011_9b78)?;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 2;
            c.g[AT][0] = 0xffff_ffff_be80_0000; // -0.25
            c.mtc1(1, 20);
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S0][0];
        WIPEOUT_RESET_TRICK.call(c, 0x0011_9b90)?;
        c.g[A0][0] = c.g[S0][0];
        WIPEOUT_RESET_COMBO.call(c, 0x0011_9b98)?;
        c.mov_s(0, 20);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `float RiderMetrix_GetRecoveryBoost(RiderMetrix* self, int show)`
/// (0x119bb0): when `show`, flash HUD number 0x21 at 1.5 and return the boost
/// constant at `gp-0x7a08`; 0.0 otherwise.
pub fn rider_metrix_get_recovery_boost(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        if c.g[A1][0] != 0 {
            c.cycles += 9;
            c.g[AT][0] = 0x3fc0_0000;
            c.mtc1(AT, 12);
            c.g[A1][0] = 0x21;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            callee::RIDER_METRIX_SHOW_HUD_NUMBER_4.call(c, 0x0011_9bd4)?;
            let boost = (c.g[GP][0] as u32).wrapping_sub(0x7a08);
            c.f[0] = Addr(boost).read_u32(c);
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 2;
            c.mtc1(ZERO, 0);
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `RiderMetrix_BuildTrickDesc(self, id, stance, alternate, style, flag)`
/// (0x0011a8c8).
pub(crate) const RIDER_METRIX_BUILD_TRICK_DESC: Callee = Callee {
    addr: 0x0011_a8c8,
    f: ee::s00::RiderMetrix_BuildTrickDesc,
};

/// `int Trick_LookupBonusTrick(RiderMetrix* self, sTrickId* id)` (0x0011b1a8).
pub(crate) const TRICK_LOOKUP_BONUS_TRICK: Callee = Callee {
    addr: 0x0011_b1a8,
    f: ee::s00::Trick_LookupBonusTrick,
};

/// `RiderMetrix_ShowHUDNumber(self, kind, value, arg, duration)` (0x00117b88).
pub(crate) const RIDER_METRIX_SHOW_HUD_NUMBER: Callee = Callee {
    addr: 0x0011_7b88,
    f: ee::s00::RiderMetrix_ShowHUDNumber,
};

/// `int RiderMetrix_CountTrickRepeats(RiderMetrix* self, sTrickId* id)`
/// (0x001190f0).
pub(crate) const RIDER_METRIX_COUNT_TRICK_REPEATS: Callee = Callee {
    addr: 0x0011_90f0,
    f: ee::s00::RiderMetrix_CountTrickRepeats,
};

/// `void RiderMetrix_ShowTrickName(RiderMetrix* self, sTrickId* id, int
/// repeat)` (0x00118ff8).
pub(crate) const RIDER_METRIX_SHOW_TRICK_NAME: Callee = Callee {
    addr: 0x0011_8ff8,
    f: ee::s00::RiderMetrix_ShowTrickName,
};

/// Rider metrics: the score float at +0x14.
pub(crate) const RIDER_METRIX_SCORE: u32 = 0x14;

/// Rider metrics: the rider object pointer at +0x1ac.
pub(crate) const RIDER_METRIX_RIDER: u32 = 0x1ac;

#[cfg(test)]
/// Rider metrics: the trick-class word at +0x70 (set while a special trick is
/// in progress; `BuildTrickDesc` then short-circuits).
pub(crate) const RIDER_METRIX_TRICK_STATE: u32 = 0x70;

/// `void RiderMetrix_AnnounceTrick(RiderMetrix* self, int stance, int
/// alternate, int style, int flag)` (0x0011a168): build the 8-byte trick id,
/// look up its bonus points; if it has any, award `points * 1e-4` to the
/// score and show the HUD number (plus the bonus-trick sound), then show the
/// trick name with its repeat count.
pub fn rider_metrix_announce_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = c.g[A2][0];
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[V1][0] = c.g[A3][0];
        c.g[T1][0] = c.g[T0][0];
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = c.g[A1][0];
        c.sd(sp(c).wrapping_add(0x10), c.g[RA][0]);
        c.g[A3][0] = c.g[V0][0];
        c.g[T0][0] = c.g[V1][0];
        c.g[A1][0] = c.g[SP][0];
        RIDER_METRIX_BUILD_TRICK_DESC.call(c, 0x0011_a198)?;
        let no_trick = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !no_trick {
            c.cycles += 4;
            c.g[A1][0] = c.g[SP][0];
            TRICK_LOOKUP_BONUS_TRICK.call(c, 0x0011_a1a8)?;
            let points = c.g[V0][0];
            let no_points = (points as i64) <= 0;
            c.g[A0][0] = c.g[S0][0];
            if !no_points {
                c.cycles += 18;
                // score += points * 1e-4f
                c.mtc1(V0, 0);
                c.cvt_s_w(0, 0);
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x79f8));
                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(RIDER_METRIX_SCORE));
                c.g[AT][0] = 0x3fc0_0000; // 1.5f
                c.mtc1(1, 12);
                c.g[A1][0] = 0x32;
                c.mul_s(0, 0, 2);
                c.g[A2][0] = c.g[V0][0];
                c.g[A3][0] = 0;
                c.add_s(1, 1, 0);
                c.swc1(1, (c.g[S0][0] as u32).wrapping_add(RIDER_METRIX_SCORE));
                RIDER_METRIX_SHOW_HUD_NUMBER.call(c, 0x0011_a1e4)?;
                call_ptr(c, 0x0028_b180, 0x0011_a1ec)?;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(RIDER_METRIX_RIDER).lw(c);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0029_b7e0, 0x0011_a1f8)?;
            }
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            RIDER_METRIX_COUNT_TRICK_REPEATS.call(c, 0x0011_a204)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = ((0u64 as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[A1][0] = c.g[SP][0];
            RIDER_METRIX_SHOW_TRICK_NAME.call(c, 0x0011_a214)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `float RiderMetrix_ReturnZero_a0c0(void)` (0x0011a0c0): return 0.0f (the
/// FPU result is the only one the caller reads; v0 is left alone).
pub fn rider_metrix_return_zero_a0c0(c: &mut Cpu) {
    c.cycles += 3;
    c.mtc1(0, 0);
    abi::ret(c);
}

/// `RiderMetrix` trick-state fields `RiderMetrix_BuildTrickDesc` reads:
/// riding switch +0x00, use the alternate table +0x04, +0x08/+0x0c/+0x10
/// (board-state flags behind the descriptor's 3-bit "k2" field), the trick
/// kind +0x20, its speed +0x24, the rail/air class +0x28 with its amount
/// +0x2c, spin and flip angles in radians +0x34/+0x38, up to three grabs
/// +0x60.., an override trick +0x70 (its low 3 bits), extra bits +0x7c, the
/// rider +0x1ac (whose +0x324 swaps the two kind codes).
pub(crate) const BTD_SWITCH: u32 = 0x00;

pub(crate) const BTD_ALT_TABLE: u32 = 0x04;

pub(crate) const BTD_FLAG_8: u32 = 0x08;

pub(crate) const BTD_FLAG_C: u32 = 0x0c;

pub(crate) const BTD_FLAG_10: u32 = 0x10;

pub(crate) const BTD_KIND: u32 = 0x20;

pub(crate) const BTD_SPEED: u32 = 0x24;

pub(crate) const BTD_CLASS: u32 = 0x28;

pub(crate) const BTD_AMOUNT: u32 = 0x2c;

pub(crate) const BTD_SPIN: u32 = 0x34;

pub(crate) const BTD_FLIP: u32 = 0x38;

pub(crate) const BTD_GRABS: u32 = 0x60;

pub(crate) const BTD_OVERRIDE: u32 = 0x70;

pub(crate) const BTD_EXTRA: u32 = 0x7c;

pub(crate) const BTD_RIDER: u32 = 0x1ac;

/// The rider's "mirrored kinds" flag.
pub(crate) const BTD_RIDER_SWAP: u32 = 0x324;

/// `.lit4` radians -> degrees for the spin and the flip angle.
pub(crate) const BTD_SPIN_DEG_LIT: u32 = 0x79e4;

pub(crate) const BTD_FLIP_DEG_LIT: u32 = 0x79e0;

/// The two 29 x 11 byte trick tables (by spin and flip index): the normal
/// one and the alternate one.
pub(crate) const BTD_TABLE: u32 = 0x0043_d388;

pub(crate) const BTD_TABLE_ALT: u32 = 0x0043_d4c8;

/// `MEM_fill(dst, byte, n)` (0x3e6448), through the function table.
pub(crate) const BTD_MEM_FILL: u32 = 0x003e_6448;

#[inline]
pub(crate) fn btd_lw(c: &mut Cpu, base: u32, off: u32) -> u64 {
    Addr(base.wrapping_add(off)).lw(c)
}

#[inline]
pub(crate) fn btd_sw(c: &mut Cpu, base: u32, off: u32, reg: usize) {
    let v = c.g[reg][0];
    Addr(base.wrapping_add(off)).sw(c, v);
}

/// The inlined `func_roundDegrees((int)(angle * 57.29578f))`: the angle at
/// `field` in whole degrees, rounded to the nearest multiple of 180, into
/// `out`. `raw` receives the truncated degrees, `rem` the remainder mod 180
/// (the registers the original uses for this angle).
pub(crate) fn btd_round_degrees(
    c: &mut Cpu,
    field: u32,
    lit: u32,
    raw: usize,
    rem: usize,
    out: usize,
) {
    c.cycles += 5;
    let scale = Addr(r32(c, GP).wrapping_sub(lit)).read_fpu(c);
    c.g[V1][0] = 0xb4;
    let angle = Addr(r32(c, S1).wrapping_add(field)).read_fpu(c);
    c.cycles += 8;
    let degrees = angle * scale;
    abi::set_fpr(c, 0, degrees);
    let whole = degrees.to_i32() as u32;
    c.f[1] = whole;
    c.g[raw][0] = sx(whole);
    let (_, r) = abi::div(c, whole, 0xb4);
    c.g[rem][0] = sx(r);
    let base = sx(whole.wrapping_sub(r));
    let r = r as i32;
    if r < -90 {
        c.cycles += 2;
        c.g[V0][0] = base;
        c.g[out][0] = sx((base as u32).wrapping_sub(0xb4));
    } else {
        c.cycles += 3;
        c.g[out][0] = base;
        if r < 91 {
            c.g[V0][0] = 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = base;
            c.g[out][0] = sx((base as u32).wrapping_add(0xb4));
        }
    }
}

/// The air-trick grade from the amount at +0x2c (>= 3): thresholds 12, 9
/// and 6 give `top`, `mid`, `low` or `base` (the C++'s 11/9/7/5 for the
/// class that matches the single spin, 12/10/8/6 otherwise).
pub(crate) fn btd_amount_grade(c: &mut Cpu, grades: [u64; 4], last_cycles: u64) {
    let amount = crate::ps2f::FpuF32(c.f[1]);
    for (limit, grade) in [(0x4140_0000u32, grades[0]), (0x4110_0000, grades[1])] {
        c.cycles += 6;
        c.g[AT][0] = limit as u64;
        c.f[0] = limit;
        let below = amount.lt(crate::ps2f::FpuF32(limit));
        abi::set_fcc(c, below);
        c.g[A0][0] = grade;
        if !below {
            return;
        }
    }
    c.cycles += 6;
    c.g[AT][0] = 0x40c0_0000;
    c.f[0] = 0x40c0_0000;
    let at_least = crate::ps2f::FpuF32(0x40c0_0000).le(amount);
    abi::set_fcc(c, at_least);
    c.g[A0][0] = grades[2];
    if !at_least {
        return;
    }
    c.cycles += last_cycles;
    c.g[A0][0] = grades[3];
}

/// `int RiderMetrix_BuildTrickDesc(TrickState *self, sTrickId *id, int
/// stance, int alternate, int style, int flag)` (0x11a8c8): clear the 8-byte
/// trick descriptor and pack the trick the rider just did into it. An
/// override trick (+0x70) is copied as is; rail/air classes (+0x28) and
/// special kinds (+0x20) get a grade from the spin count or the amount;
/// otherwise the spin and flip angles, rounded to half and whole turns,
/// pick an entry of the 29 x 11 trick table (out of range: the "unknown
/// trick" descriptor), and the spin direction, flip count and direction,
/// grabs, style/flag quality and extra bits are filled in. Returns whether
/// anything was done (1 on the early paths).
pub fn rider_metrix_build_trick_desc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S5, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[T1][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[T0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x8;
        call_ptr(c, BTD_MEM_FILL, 0x0011_a910)?;
        let me = r32(c, S1);
        let id = r32(c, S0);
        c.g[V0][0] = btd_lw(c, me, BTD_OVERRIDE);
        c.g[A1][0] = 0xffff_ffff_ffff_fff8;
        'done: {
            if c.g[V0][0] != 0 {
                // Override trick: kind and trick from +0x0c / +0x70.
                c.cycles += 13;
                c.g[V0][0] = btd_lw(c, me, BTD_FLAG_C);
                c.g[V1][0] = btd_lw(c, id, 0);
                c.g[V0][0] &= 0x7;
                c.g[A0][0] = btd_lw(c, id, 4);
                c.g[V1][0] = (c.g[V1][0] & c.g[A1][0]) | c.g[V0][0];
                c.g[A0][0] &= c.g[A1][0];
                btd_sw(c, id, 0, V1);
                c.g[V0][0] = btd_lw(c, me, BTD_OVERRIDE) & 0x7;
                c.g[A0][0] |= c.g[V0][0];
                btd_sw(c, id, 4, A0);
                if c.g[S4][0] != 0 {
                    c.cycles += 7;
                    c.g[V1][0] = 0x300_0000;
                    c.g[V0][0] = (c.g[A0][0] & 0xffff_ffff_f8ff_ffff) | 0x300_0000;
                    btd_sw(c, id, 4, V0);
                }
                c.cycles += 2;
                c.g[V0][0] = 1;
                break 'done;
            }

            // Riding switch mirrors the spin, kind 4 the flip.
            c.cycles += 3;
            c.g[V0][0] = btd_lw(c, me, BTD_SWITCH);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                let a = Addr(me.wrapping_add(BTD_SPIN));
                let v = -a.read_fpu(c);
                abi::set_fpr(c, 0, v);
                a.write_fpu(c, v);
            }
            c.g[V1][0] = btd_lw(c, me, BTD_KIND);
            c.cycles += 3;
            c.g[V0][0] = 0x4;
            if c.g[V1][0] == 0x4 {
                c.cycles += 3;
                let a = Addr(me.wrapping_add(BTD_FLIP));
                let v = -a.read_fpu(c);
                abi::set_fpr(c, 0, v);
                a.write_fpu(c, v);
            }
            // Spin to half turns (t2), flip to whole turns (t0).
            btd_round_degrees(c, BTD_SPIN, BTD_SPIN_DEG_LIT, A0, A1, T2);
            btd_round_degrees(c, BTD_FLIP, BTD_FLIP_DEG_LIT, A1, A0, T0);
            c.cycles += 5;
            c.g[V0][0] = 0x168;
            c.g[V1][0] = 0xb4;
            let flip_deg = r32(c, T0);
            let (flip_turns, _) = abi::div(c, flip_deg, 0x168);
            c.cycles += 5;
            c.g[A2][0] = btd_lw(c, me, BTD_CLASS);
            c.g[A0][0] = sx(flip_turns);
            let spin_deg = r32(c, T2);
            let (spin_halves, _) = abi::div(c, spin_deg, 0xb4);
            // flips (t1) and spins (a3): absolute counts.
            c.g[T1][0] = c.g[A0][0];
            if (flip_turns as i32) < 0 {
                c.cycles += 1;
                c.g[T1][0] = sx(flip_turns.wrapping_neg());
            }
            c.cycles += 3;
            c.g[A1][0] = sx(spin_halves);
            c.g[A3][0] = c.g[A1][0];
            if (spin_halves as i32) < 0 {
                c.cycles += 1;
                c.g[A3][0] = sx(spin_halves.wrapping_neg());
            }
            c.cycles += 2;
            let spins = c.g[A3][0] as i64;
            c.g[V0][0] = (spins < 2) as u64;

            if c.g[A2][0] != 0 {
                // Rail/air class: a grade from the spin count or the amount.
                c.cycles += 2;
                c.g[V0][0] = 0x1;
                let graded = if spins >= 2 {
                    c.cycles += 3;
                    c.g[V0][0] = (spins < 12) as u64;
                    c.g[V1][0] = (spins < 6) as u64;
                    if spins < 12 {
                        c.cycles += 4;
                        c.g[A0][0] = 0xd;
                        c.g[V0][0] = 0xe;
                        if c.g[V1][0] == 0 {
                            c.g[A0][0] = 0xe;
                        }
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = 0xf;
                    }
                    true
                } else {
                    c.cycles += 2;
                    let mut big = false;
                    if spins == 1 {
                        c.cycles += 7;
                        let amount = Addr(me.wrapping_add(BTD_AMOUNT)).read_fpu(c);
                        abi::set_fpr(c, 1, amount);
                        c.g[AT][0] = 0x4040_0000;
                        c.f[0] = 0x4040_0000;
                        big = crate::ps2f::FpuF32(0x4040_0000).le(amount);
                        abi::set_fcc(c, big);
                    }
                    if big {
                        c.cycles += 2;
                        if c.g[A2][0] != c.g[A3][0] {
                            btd_amount_grade(c, [0xc, 0xa, 0x6, 0x8], 1);
                        } else {
                            btd_amount_grade(c, [0xb, 0x9, 0x5, 0x7], 2);
                        }
                    }
                    big
                };
                if graded {
                    c.cycles += 8;
                    c.g[V0][0] = btd_lw(c, id, 4);
                    c.g[V1][0] = 0xffff_ffff_ff0f_ffff;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 20);
                    c.g[V0][0] = (c.g[V0][0] & c.g[V1][0]) | c.g[A0][0];
                    btd_sw(c, id, 4, V0);
                } else {
                    // Small spin: kind by class and +0x20, spin count.
                    c.cycles += 2;
                    if c.g[A2][0] != 1 {
                        c.g[A2][0] = btd_lw(c, me, BTD_KIND);
                        c.cycles += 8;
                        c.g[A1][0] = 0x4;
                        c.g[A0][0] = btd_lw(c, id, 4);
                        c.g[V1][0] = 0xffff_ffff_ff0f_ffff;
                        c.g[V0][0] = if c.g[A2][0] != 0 { 4 } else { 2 };
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 20);
                        c.g[A0][0] &= c.g[V1][0];
                    } else {
                        c.cycles += 10;
                        c.g[A1][0] = btd_lw(c, me, BTD_KIND);
                        c.g[A0][0] = btd_lw(c, id, 4);
                        c.g[V1][0] = 0xffff_ffff_ff0f_ffff;
                        c.g[V0][0] = if c.g[A1][0] == 0 { c.g[A2][0] } else { 3 };
                        c.g[V0][0] &= 0xf;
                        c.g[A0][0] &= c.g[V1][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 20);
                    }
                    c.cycles += 13;
                    c.g[A0][0] |= c.g[V0][0];
                    btd_sw(c, id, 4, A0);
                    c.g[A0][0] = (spins < 11) as u64;
                    c.g[V0][0] = if c.g[A0][0] != 0 { c.g[A3][0] } else { 0xa };
                    c.g[V1][0] = btd_lw(c, id, 0);
                    c.g[V0][0] &= 0xf;
                    c.g[A0][0] = 0xffff_ffff_ffff_ff87;
                    c.g[V1][0] &= c.g[A0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                    c.g[V1][0] |= c.g[V0][0];
                    btd_sw(c, id, 0, V1);
                }
                c.cycles += 2;
                c.g[V0][0] = 1;
                break 'done;
            }

            c.cycles += 3;
            c.g[A1][0] = btd_lw(c, me, BTD_KIND);
            c.g[A2][0] = c.g[A3][0];
            if c.g[A1][0] != 0 {
                // Special kind: slow ones are kind 5, else the kind (swapped
                // 1 <-> 2 for a mirrored rider) and the spin count.
                c.cycles += 7;
                let speed = Addr(me.wrapping_add(BTD_SPEED)).read_fpu(c);
                abi::set_fpr(c, 1, speed);
                c.g[AT][0] = 0x4248_0000;
                c.f[0] = 0x4248_0000;
                let slow = speed.le(crate::ps2f::FpuF32(0x4248_0000));
                abi::set_fcc(c, slow);
                c.g[V1][0] = 0xffff_ffff_ffff_fff8;
                if slow {
                    c.cycles += 4;
                    c.g[V0][0] = (btd_lw(c, id, 0) & c.g[V1][0]) | 0x5;
                } else {
                    c.cycles += 18;
                    c.g[A0][0] = btd_lw(c, id, 0);
                    c.g[V1][0] = (spins < 11) as u64;
                    c.g[A2][0] = 0xffff_ffff_ffff_fff8;
                    c.g[V0][0] = if c.g[V1][0] != 0 { c.g[A3][0] } else { 0xa };
                    c.g[A1][0] &= 0x7;
                    c.g[A0][0] &= c.g[A2][0];
                    c.g[V1][0] = 0xffff_ffff_ffff_ff87;
                    c.g[V0][0] &= 0xf;
                    c.g[A0][0] |= c.g[A1][0];
                    c.g[A0][0] &= c.g[V1][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                    c.g[A0][0] |= c.g[V0][0];
                    btd_sw(c, id, 0, A0);
                    let rider = btd_lw(c, me, BTD_RIDER) as u32;
                    c.g[V0][0] = sx(rider);
                    c.g[V1][0] = btd_lw(c, rider, BTD_RIDER_SWAP);
                    c.g[V0][0] = 1;
                    if c.g[V1][0] == 0 {
                        c.cycles += 2;
                        c.g[V0][0] = 1;
                        break 'done;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = btd_lw(c, me, BTD_KIND);
                    if c.g[V1][0] == 1 {
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[A0][0] & c.g[A2][0]) | 0x2;
                    } else {
                        c.g[V0][0] = 0x2;
                        c.cycles += 2;
                        if c.g[V1][0] != 2 {
                            c.g[V0][0] = 1;
                            break 'done;
                        }
                        c.cycles += 2;
                        c.g[V0][0] = (c.g[A0][0] & c.g[A2][0]) | 0x1;
                    }
                }
                c.cycles += 1;
                btd_sw(c, id, 0, V0);
                c.cycles += 2;
                c.g[V0][0] = 1;
                break 'done;
            }

            // A plain trick. cnt (t3): how many flips came with three
            // half spins each.
            c.cycles += 5;
            c.g[A0][0] = c.g[T1][0];
            c.g[T3][0] = 0;
            c.g[V1][0] = btd_lw(c, me, BTD_GRABS);
            c.g[A1][0] = btd_lw(c, me, BTD_FLAG_10);
            loop {
                c.cycles += 2;
                c.g[V0][0] = ((c.g[A3][0] as i64) < 3) as u64;
                if (c.g[T1][0] as i64) <= 0 {
                    break;
                }
                c.cycles += 2;
                let few_spins = c.g[V0][0] != 0;
                c.g[T1][0] = sx(r32(c, T1).wrapping_sub(1));
                if few_spins {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0011_ace8);
                }
                c.cycles += 2;
                c.g[A3][0] = sx(r32(c, A3).wrapping_sub(3));
                c.g[T3][0] = sx(r32(c, T3).wrapping_add(1));
            }
            // Table indices: spin idx (a2), flip idx (a0); restore the
            // counts.
            c.cycles += 3;
            c.g[T1][0] = c.g[A0][0];
            c.g[A3][0] = c.g[A2][0];
            let spin_right = (c.g[T2][0] as i64) > 0;
            if !spin_right {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A3][0] as i64) < 15) as u64;
            }
            if spin_right || c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[A2][0] = sx(r32(c, A3).wrapping_add(0xe));
            }
            c.cycles += 2;
            c.g[V0][0] = ((c.g[A0][0] as i64) < 6) as u64;
            let flip_back = (c.g[T0][0] as i64) < 0;
            if !flip_back {
                c.cycles += 2;
                c.g[T0][0] = 0;
            }
            if flip_back || c.g[V0][0] == 0 {
                c.cycles += 2;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x5));
                c.g[T0][0] = 0;
            }
            // Grab count (t0).
            c.cycles += 2;
            c.g[T4][0] = ((c.g[A2][0] as i64) < 29) as u64;
            if c.g[V1][0] != 0 {
                c.cycles += 3;
                c.g[V1][0] = sx(me.wrapping_add(BTD_GRABS));
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[T0][0] as i64) < 3) as u64;
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.cycles += 3;
                    let grab = r32(c, V1);
                    c.g[V0][0] = btd_lw(c, grab, 0);
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                    if event_due(c) {
                        deliver_event_here(c, 0x0011_ad48);
                    }
                }
            }
            // k2 (bits 7..9) from the board-state flags.
            c.cycles += 3;
            c.g[V0][0] = 0x1;
            let k2_bits = if c.g[A1][0] == 1 {
                c.cycles += 5;
                Some(0x280)
            } else {
                c.g[V0][0] = 0x2;
                c.cycles += 2;
                if c.g[A1][0] == 2 {
                    c.cycles += 5;
                    Some(0x300)
                } else {
                    c.g[V0][0] = btd_lw(c, me, BTD_FLAG_C);
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.cycles += 3;
                        c.g[V0][0] = btd_lw(c, me, BTD_SWITCH);
                        c.g[V1][0] = 0xffff_ffff_ffff_fc7f;
                        c.cycles += 4;
                        Some(if c.g[V0][0] != 0 { 0x180 } else { 0x100 })
                    } else {
                        c.g[V0][0] = btd_lw(c, me, BTD_FLAG_8);
                        c.cycles += 2;
                        c.g[V1][0] = 0xffff_ffff_ffff_fc7f;
                        if c.g[V0][0] != 0 {
                            c.cycles += 4;
                            Some(0x200)
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = btd_lw(c, me, BTD_SWITCH);
                            if c.g[V0][0] != 0 {
                                c.cycles += 3;
                                Some(0x80)
                            } else {
                                None
                            }
                        }
                    }
                }
            };
            if let Some(bits) = k2_bits {
                c.g[V1][0] = 0xffff_ffff_ffff_fc7f;
                c.g[V0][0] = (btd_lw(c, id, 0) & c.g[V1][0]) | bits;
                c.cycles += 1;
                btd_sw(c, id, 0, V0);
            }

            // The table entry (bits 22..27), or the "unknown trick".
            c.cycles += 2;
            c.g[V0][0] = ((c.g[A0][0] as i64) < 11) as u64;
            let in_range = c.g[T4][0] != 0 && {
                c.cycles += 2;
                c.g[V0][0] != 0
            };
            if !in_range {
                if c.g[T4][0] == 0 {
                    c.cycles += 1;
                }
                c.g[A0][0] = c.g[S0][0];
                c.cycles += 12;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x8;
                call_ptr(c, BTD_MEM_FILL, 0x0011_ae80)?;
                let id = r32(c, S0);
                c.g[V1][0] = btd_lw(c, id, 0);
                c.g[V0][0] = 0xffff_ffff_f03f_ffff;
                c.g[A0][0] = 0x40_0000;
                c.g[V1][0] = (c.g[V1][0] & c.g[V0][0]) | c.g[A0][0];
                c.g[V0][0] = 0x1;
                btd_sw(c, id, 0, V1);
                break 'done;
            }
            c.cycles += 3;
            c.g[V0][0] = btd_lw(c, me, BTD_ALT_TABLE);
            c.g[V1][0] = 0xb;
            let mut table = BTD_TABLE;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[A1][0] = 0x44_0000;
                if c.g[S2][0] != 0 {
                    table = BTD_TABLE_ALT;
                }
            }
            c.cycles += 3;
            c.g[A1][0] = sx(table);
            let spin_idx = r32(c, A2);
            c.g[V0][0] = sx(abi::mult(c, spin_idx, 0xb));
            c.cycles += 12;
            c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[A0][0] = btd_lw(c, id, 0);
            c.g[V0][0] = 0xffff_ffff_f03f_ffff;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(table));
            c.g[A0][0] &= c.g[V0][0];
            c.g[V0][0] = Addr(r32(c, V1)).lbu(c) & 0x3f;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 22);
            c.g[A0][0] |= c.g[V0][0];
            btd_sw(c, id, 0, A0);

            // No table entry: spin direction (bits 10..11), spin count
            // (12..15) when no flips, flip count (16..18) and direction
            // (19..21).
            c.cycles += 5;
            c.g[V1][0] = btd_lw(c, id, 0);
            c.g[V0][0] = c.g[V1][0] & 0xfc0_0000;
            let mut extra_spins = true;
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                if (c.g[A3][0] as i64) > 0 {
                    c.cycles += 2;
                    c.g[V0][0] = 0xffff_ffff_ffff_f3ff;
                    let dir = if (c.g[T2][0] as i64) > 0 {
                        c.cycles += 3;
                        0x800
                    } else {
                        c.cycles += 2;
                        0x400
                    };
                    c.g[V0][0] = (c.g[V1][0] & c.g[V0][0]) | dir;
                    c.cycles += 1;
                    btd_sw(c, id, 0, V0);
                }
                c.cycles += 2;
                c.g[V0][0] = btd_lw(c, id, 0);
                if c.g[T1][0] == 0 {
                    c.cycles += 8;
                    c.g[V1][0] = 0xffff_ffff_ffff_0fff;
                    c.g[A0][0] = c.g[A3][0] & 0xf;
                    c.g[V0][0] &= c.g[V1][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 12);
                    c.g[V0][0] |= c.g[A0][0];
                    btd_sw(c, id, 0, V0);
                    c.g[V0][0] = btd_lw(c, id, 0);
                }
                c.cycles += 8;
                c.g[V1][0] = 0xffff_ffff_fff8_ffff;
                c.g[A0][0] = c.g[T1][0] & 0x7;
                c.g[A1][0] = c.g[V0][0] & c.g[V1][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 16);
                c.g[A1][0] |= c.g[A0][0];
                btd_sw(c, id, 0, A1);
                if (c.g[T1][0] as i64) <= 0 {
                    extra_spins = false;
                } else {
                    c.cycles += 2;
                    let flip = Addr(me.wrapping_add(BTD_FLIP)).read_fpu(c);
                    abi::set_fpr(c, 1, flip);
                    let all_combined = (c.g[T3][0] as i64) > 0 && {
                        c.cycles += 2;
                        c.g[T3][0] == c.g[T1][0]
                    };
                    c.cycles += 5;
                    c.f[0] = 0;
                    let forward = crate::ps2f::FpuF32::ZERO.lt(flip);
                    abi::set_fcc(c, forward);
                    if all_combined {
                        c.g[V1][0] = 0x20_0000;
                        if forward {
                            c.cycles += 5;
                        } else {
                            c.cycles += 6;
                            c.g[V1][0] = 0x18_0000;
                        }
                        c.g[V0][0] = (c.g[A1][0] & 0xffff_ffff_ffc7_ffff) | c.g[V1][0];
                    } else {
                        c.g[A0][0] = 0x8_0000;
                        c.cycles += 4;
                        c.g[V0][0] = btd_lw(c, id, 0);
                        c.g[V1][0] = 0xffff_ffff_ffc7_ffff;
                        if !forward {
                            c.g[A0][0] = 0x10_0000;
                        }
                        c.cycles += 2;
                        c.g[V0][0] = (c.g[V0][0] & c.g[V1][0]) | c.g[A0][0];
                    }
                    c.cycles += 1;
                    btd_sw(c, id, 0, V0);
                }
            }
            // Leftover half spins (bits 28..31) next to the flips.
            if extra_spins {
                c.cycles += 2;
                c.g[V0][0] = sx(r32(c, T1) << 1);
                if (c.g[T1][0] as i64) > 0 {
                    c.cycles += 3;
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T1)));
                    let three_each = c.g[A3][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[A3][0] as i64) < 15) as u64;
                    if !three_each {
                        c.cycles += 2;
                        c.g[V1][0] = 0xfff_0000;
                        if c.g[V0][0] != 0 {
                            c.cycles += 6;
                            c.g[V0][0] = btd_lw(c, id, 0);
                            c.g[V1][0] = 0xfff_ffff;
                            c.g[A0][0] = sx(r32(c, A3) << 28);
                            c.g[V0][0] = (c.g[V0][0] & c.g[V1][0]) | c.g[A0][0];
                            btd_sw(c, id, 0, V0);
                        }
                    }
                }
            }

            // Quality (bits 24..26 of the second word).
            c.cycles += 3;
            c.g[V0][0] = 0x2;
            let grabs = c.g[T0][0] as i64;
            let mut skip_grab_check = false;
            'quality: {
                if c.g[S3][0] == 1 {
                    c.cycles += 5;
                    c.g[V0][0] = btd_lw(c, id, 4);
                    c.g[V1][0] = 0xffff_ffff_f8ff_ffff;
                    c.g[A0][0] = 0x400_0000;
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = 0x500_0000;
                    if c.g[S3][0] == 2 {
                        c.cycles += 4;
                        c.g[V0][0] = btd_lw(c, id, 4);
                        c.g[V1][0] = 0xffff_ffff_f8ff_ffff;
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = 0x300_0000;
                        if c.g[S4][0] != 0 {
                            c.cycles += 4;
                            c.g[V0][0] = btd_lw(c, id, 4);
                            c.g[V1][0] = 0xffff_ffff_f8ff_ffff;
                        } else {
                            c.cycles += 5;
                            c.g[V1][0] = btd_lw(c, id, 0);
                            c.g[V0][0] = c.g[V1][0] & 0xfc0_0000;
                            if c.g[V0][0] != 0 {
                                break 'quality;
                            }
                            c.cycles += 2;
                            if c.g[T3][0] != 0 {
                                c.cycles += 4;
                                c.g[V0][0] = c.g[V1][0] & 0xffff_ffff_f000_0000;
                                if c.g[V0][0] == 0 {
                                    break 'quality;
                                }
                            }
                            c.cycles += 2;
                            let mut stance_q = false;
                            if c.g[S5][0] != 0 {
                                c.cycles += 3;
                                c.g[V0][0] = btd_lw(c, me, BTD_SWITCH);
                                c.g[A0][0] = 0x200_0000;
                                if c.g[V0][0] == 0 {
                                    c.cycles += 4;
                                    c.g[V0][0] = btd_lw(c, id, 4);
                                    c.g[V1][0] = 0xffff_ffff_f8ff_ffff;
                                    stance_q = true;
                                }
                            }
                            if !stance_q {
                                c.cycles += 2;
                                if c.g[A3][0] != 0 {
                                    break 'quality;
                                }
                                c.cycles += 2;
                                if c.g[T1][0] != 0 {
                                    break 'quality;
                                }
                                c.cycles += 2;
                                c.g[A0][0] = 0x100_0000;
                                if grabs <= 0 {
                                    skip_grab_check = true;
                                    break 'quality;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = btd_lw(c, id, 4);
                                c.g[V1][0] = 0xffff_ffff_f8ff_ffff;
                            }
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] & c.g[V1][0]) | c.g[A0][0];
                btd_sw(c, id, 4, V0);
            }

            // Grabs: the first (bits 3..9), a second one (bit 10, and its
            // id in bits 11..17, or bit 11 for three).
            if !skip_grab_check {
                c.cycles += 2;
                c.g[A0][0] = 0xffff_ffff_ffff_fc07;
                if grabs > 0 {
                    c.cycles += 9;
                    c.g[V0][0] = btd_lw(c, me, BTD_GRABS);
                    c.g[V1][0] = btd_lw(c, id, 4);
                    c.g[A1][0] = (grabs < 2) as u64;
                    c.g[V0][0] &= 0x7f;
                    c.g[V1][0] &= c.g[A0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                    c.g[V1][0] |= c.g[V0][0];
                    btd_sw(c, id, 4, V1);
                    if c.g[A1][0] == 0 {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[V1][0] | 0x400;
                        c.g[V0][0] = 0x2;
                        btd_sw(c, id, 4, A0);
                        if grabs == 2 {
                            c.cycles += 8;
                            c.g[V1][0] = btd_lw(c, me, BTD_GRABS + 4) & 0x7f;
                            c.g[V0][0] = c.g[A0][0] & 0xffff_ffff_fffc_07ff;
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 11);
                            c.g[V0][0] |= c.g[V1][0];
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = (c.g[A0][0] & 0xffff_ffff_fffc_07ff) | 0x800;
                        }
                        c.cycles += 1;
                        btd_sw(c, id, 4, V0);
                    }
                }
            }

            // Extra bits (18..19 of the second word).
            c.cycles += 3;
            let extra = btd_lw(c, me, BTD_EXTRA);
            c.g[A0][0] = extra & 0x3;
            if extra != 0 {
                c.cycles += 7;
                c.g[V0][0] = btd_lw(c, id, 4);
                c.g[V1][0] = 0xffff_ffff_fff3_ffff;
                c.g[V0][0] &= c.g[V1][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 18);
                c.g[V0][0] |= c.g[A0][0];
                btd_sw(c, id, 4, V0);
            }

            // Anything at all?
            c.cycles += 2;
            c.g[V1][0] = 0;
            if (c.g[T1][0] as i64) > 0 {
                c.cycles += 1;
                c.g[V1][0] = 1;
            } else {
                c.cycles += 2;
                if (c.g[A3][0] as i64) > 0 {
                    c.g[V1][0] = 1;
                } else {
                    c.cycles += 2;
                    if grabs > 0 {
                        c.g[V1][0] = 1;
                    } else {
                        c.cycles += 3;
                        let extra = btd_lw(c, me, BTD_EXTRA);
                        c.g[V0][0] = c.g[V1][0];
                        if extra == 0 {
                            break 'done;
                        }
                        c.cycles += 1;
                        c.g[V1][0] = 1;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[V1][0];
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::elf_image;

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::belibrary;
    pub(crate) use crate::game::ai::ai::tests::leaf;

    pub(crate) use crate::game::ai::ai::tests::virtual_setup;
    pub(crate) use crate::game::ai::ai::tests::wf;
    pub(crate) use crate::game::ai::ai::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::ai::rider::tests::f;
    pub(crate) use crate::game::ai::rider::tests::fb;
    pub(crate) use crate::game::ai::rider::tests::put_state;
    pub(crate) use crate::game::ai::rider::tests::rand_vec;
    pub(crate) use crate::game::ai::rider::tests::uf;
    pub(crate) use crate::game::ai::rider::tests::RIDER;
    pub(crate) use crate::game::ai::rider::tests::SEEDS;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;

    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;

    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_7;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_6;

    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;

    use ssx3_ee as ee;
    use ssx3_ee::s00;

    #[test]
    fn rider_metrix_set_base_score_matches() {
        check(
            ssx3_ee::s00::f_00117520,
            rider_metrix_set_base_score,
            200,
            &|c, seed| {
                let mut r = Rng(0x5566_7788 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.bus.write32(DATA + 0x1c8, r.next_u32());
                c.bus.write32(DATA + 0x198, r.next_u32());
            },
        );
    }

    #[test]
    fn rider_metrix_get_rounded_trick_points_matches() {
        check_lohi(
            ssx3_ee::s00::f_00117948,
            rider_metrix_get_rounded_trick_points,
            300,
            &|c, seed| {
                let mut r = Rng(0x6677_8899 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x14, r.edge_f32());
                c.bus.write32(DATA + 0x1c4, r.edge_f32());
                c.bus.write32(GAME_GP - 0x7b08, r.edge_f32());
            },
        );
    }

    #[test]
    fn rider_metrix_update_matches() {
        check(
            ee::s00::f_00117c28__generated,
            rider_metrix_update,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x7c28, 0);
                elf_image(c);
                let m = DATA + 0x8000;
                // The BELibrary singleton (gp-0x1f04) already exists: otherwise
                // `BELibrary_GetInstance` (reached from the trick HUD and
                // `RiderMetrix_BankCombo` → `RiderMetrix_AwardPoints`) builds it
                // and waits forever on its rewards file (`EAFile_QueueOpen` /
                // `EAFile_AllocRequest` with no request slots on a test machine).
                c.bus.write32(GAME_GP - 0x1f04, DATA + 0xa000);
                c.bus.write32(m + 0x1ac, RIDER);
                rand_vec(c, &mut r, RIDER + 0x1e0, 30.0);
                c.bus.write32(RIDER + 0x300, uf(&mut r, 0.5, 2.0));
                rand_vec(c, &mut r, RIDER + 0x1c0, 5.0);
                put_state(c, (seed % 3 == 1) as u32, if seed % 4 == 2 { 1 } else { 3 });
                for off in [0xa4, 0x30, 0x40, 0x24, 0x2c, 0x6c, 0x78] {
                    let v = if r.next_u32() % 3 == 0 {
                        r.f32(0.05)
                    } else {
                        r.f32(10.0)
                    };
                    c.bus.write32(m + off, v);
                }
                for off in [0x14, 0x3c, 0x1c, 0x1a8, 0x164, 0x168, 0x160, 0x15c] {
                    c.bus.write32(m + off, r.f32(50.0));
                }
                c.bus.write32(m + 0x84, r.next_u32() % 1000);
                let n = [0, 1, 3, 0xffff_fffe][(seed % 4) as usize];
                c.bus
                    .write32(m + 0x1b0, if seed % 5 == 0 { 0 } else { DATA + 0x9000 });
                c.bus.write32(m + 0x1b4, n);
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn rider_metrix_check_score_milestone_matches() {
        check(
            ee::s00::f_00119210__generated,
            rider_metrix_check_score_milestone,
            120,
            &|c, seed| {
                let mut r = Rng(0x1192_10 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let this = DATA;
                let idx = DATA + 0x100;
                let table = DATA + 0x200;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(idx);
                c.g[A2][0] = sx(table);
                c.g[A3][0] = sx(2);
                let i = if seed % 3 == 0 { 0 } else { r.next_u32() % 3 };
                c.bus.write32(idx, i);
                // Seeds cover a negative threshold, an unreached step and a hit.
                let (threshold, x) = match seed % 3 {
                    0 => ((-1.0f32).to_bits(), 1.0f32.to_bits()),
                    1 => (5.0f32.to_bits(), 1.0f32.to_bits()),
                    _ => (0.5f32.to_bits(), 1.0f32.to_bits()),
                };
                c.bus.write32(table + i * 8, threshold);
                c.bus.write32(table + i * 8 + 4, r.f32(100.0));
                c.bus.write32(this + rider::METRIC_SCALE, r.edge_f32());
                c.f[12] = x;
            },
        );
    }

    #[test]
    fn rider_metrix_clear_peak_a4_matches() {
        check(
            ee::s00::f_001176f8__generated,
            rider_metrix_clear_peak_a4,
            SEEDS_2,
            &|c, seed| {
                let mut r = rng(0x0011_76f8, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0xa4, r.next_u32());
            },
        );
    }

    #[test]
    fn rider_metrix_update_peak_a4_matches() {
        check(
            ee::s00::f_00117708__generated,
            rider_metrix_update_peak_a4,
            SEEDS_2,
            &|c, seed| {
                let mut r = rng(0x0011_7708, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                // Half the seeds take a larger "seconds", half a smaller one.
                let (cur, new) = if seed % 2 == 0 {
                    (r.f32(10.0), r.f32(100.0))
                } else {
                    (r.f32(100.0), r.f32(10.0))
                };
                c.bus.write32(DATA + 0xa4, cur);
                c.f[12] = new;
            },
        );
    }

    #[test]
    fn rider_metrix_on_race_finished_matches() {
        check(
            ee::s00::f_001193e0__generated,
            rider_metrix_on_race_finished,
            200,
            &|c, seed| {
                machine(c, seed, 0x11_93e0, 0);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                // Both exits of `RiderMetrix_BankCombo` are early: no combo banked
                // (trick count < 2) or no points.
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x1000 + 0x9c, 0);
                } else {
                    c.bus.write32(DATA + 0x1000 + 0x9c, 5);
                    c.bus.write32(DATA + 0x1000 + 0xa0, 0);
                }
            },
        );
    }

    #[test]
    fn rider_metrix_set_press_spin_matches() {
        check(
            s00::f_00119ad8__generated,
            rider_metrix_set_press_spin,
            150,
            &|c, seed| {
                machine(c, seed, 0x119ad8, 1);
                clear(c, OBJ, 0x80);
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(seed as u32 & 1);
                // The pi constant lives at `gp-0x7a0c` in `.lit4`; both bodies load
                // it from there, so a zeroed page is fine (and deterministic).
                c.bus.write32(GAME_GP.wrapping_sub(0x7a0c), 0x4049_0fdb);
                c.bus.write32(OBJ + 0x34, seed.wrapping_mul(0x3dcc_cccd));
                c.bus.write32(OBJ + 0x14, 0);
            },
        );
    }

    #[test]
    fn rider_metrix_load_state_matches() {
        check(
            ee::s00::f_001174e8__generated,
            rider_metrix_load_state,
            SEEDS_3,
            &|c, seed| {
                virtual_setup(c, seed, 0x1174_e8, 0x10);
            },
        );
    }

    #[test]
    fn rider_metrix_start_board_press_matches() {
        check(
            ssx3_ee::s00::f_001199f8__generated,
            rider_metrix_start_board_press,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x06, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                // The press peak `+0x2c` can be negative or positive.
                c.bus.write32(
                    DATA + 0x2c,
                    if seed % 3 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(1.0)
                    },
                );
            },
        );
    }

    // ------------------------------------------------------------------
    // Rider metrics / BE.

    #[test]
    fn rider_metrix_on_knockdown_opponent_matches() {
        check(
            ee::s00::f_00119400__generated,
            rider_metrix_on_knockdown_opponent,
            SEEDS_4,
            &|c, seed| {
                let _ = machine(c, seed, 0x119400, 0);
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at + 0x128, (seed % 7) as u32);
            },
        );
    }

    #[test]
    fn rider_metrix_start_handplant_matches() {
        check(
            ee::s00::f_00119bf0__generated,
            rider_metrix_start_handplant,
            SEEDS_4,
            &|c, seed| {
                let _ = machine(c, seed, 0x119bf0, 0);
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.bus.write32(self_at + 0x14, 0x4120_0000 | (seed & 0xfff));
                c.bus.write32(GAME_GP - 0x7a04, 0x3ca3_d70a);
            },
        );
    }

    #[test]
    fn rider_metrix_clear_hud_messages_matches() {
        check(
            ee::s00::f_001173b8__generated,
            rider_metrix_clear_hud_messages,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11_73b8, 0);
                let m = DATA + 0x1000;
                let entries = DATA + 0x2000;
                match seed % 4 {
                    0 => {}
                    1 => c.bus.write32(m + 0x1b0, entries),
                    2 => {
                        c.bus.write32(m + 0x1b0, entries);
                        c.bus.write32(m + 0x1b4, u32::MAX);
                    }
                    _ => {
                        c.bus.write32(m + 0x1b0, entries);
                        c.bus.write32(m + 0x1b4, 1 + r.next_u32() % 6);
                    }
                }
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn rider_metrix_on_mission_completed_matches() {
        check(
            ssx3_ee::s00::f_0011a110__generated,
            rider_metrix_on_mission_completed,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11a110, 2);
                belibrary(c, &mut r);
                let mtx = DATA + 0x500;
                // The HUD slot array is empty: `ShowHUDNumber` returns at once.
                c.bus.write32(mtx + 0x1b0, 0);
                c.g[A0][0] = mtx as u64;
                // A zero amount makes `RiderMetrix_AwardPoints` return at once.
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn rider_metrix_get_trick_points_matches() {
        leaf(
            ssx3_ee::s00::f_00117900__generated,
            rider_metrix_get_trick_points,
            31,
        );
    }

    #[test]
    fn rider_metrix_record_best_jump_matches() {
        check(
            ee::s00::f_00119448__generated,
            rider_metrix_record_best_jump,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11_9448, 2);
                c.f[12] = r.f32(500.0);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                w(c, DATA + 0x2000 + 0x130, seed % 9);
                w(
                    c,
                    DATA + 0x2000 + 0x1b0,
                    if seed % 3 == 0 { 0 } else { DATA + 0x2200 },
                );
                w(
                    c,
                    DATA + 0x2000 + 0x18,
                    if seed % 2 == 0 { fb(-100.0) } else { fb(1e6) },
                );
                c.pc = 0x0011_9448;
            },
        );
    }

    #[test]
    fn rider_metrix_reset_matches() {
        check(
            ee::s00::f_00117540__generated,
            rider_metrix_reset,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x117540, 6);
                be_library_2(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0x114, 0);
                w(c, obj + 0x118, 0);
                for i in 0..16u32 {
                    w(c, obj + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn rider_metrix_on_rider_reset_matches() {
        check(
            ee::s00::f_00119368__generated,
            rider_metrix_on_rider_reset,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x119368, 3);
                be_library_2(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, obj + 0x1ac, DATA + 0x1000);
                w(c, DATA + 0x1000 + 0x2f8, r.edge_f32());
            },
        );
    }

    #[test]
    fn rider_metrix_find_hud_slot_matches() {
        check(
            ee::s00::f_00117a58__generated,
            rider_metrix_find_hud_slot,
            200,
            &|c, seed| {
                let mut r = base(c, seed, 0x11_7a58);
                globals(c);
                const SLOTS: u32 = DATA + 0x1554;
                let count = 34 + (seed % 5);
                w(c, DATA + 0x1b4, count);
                w(c, DATA + 0x1b0, SLOTS - 0x1554);
                for i in 0..39u32 {
                    if i < 35 {
                        w(c, SLOTS + 0x9c * i, 0);
                        continue;
                    }
                    // 0x34 marks a free slot; otherwise value/total drives the
                    // "smallest ratio" search.
                    let free = (seed + i) % 5 == 0;
                    w(c, SLOTS + 0x9c * i, if free { 0x34 } else { 0 });
                    c.bus.write32(
                        SLOTS + 0x9c * i + 4,
                        0x3f80_0000u32 | (r.next_u32() & 0x003f_ffff),
                    );
                    c.bus.write32(
                        SLOTS + 0x9c * i + 8,
                        0x3f00_0000u32 | (r.next_u32() & 0x007f_ffff),
                    );
                }
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = [0u64, 17, 33, 34, 36][seed as usize % 5];
            },
        );
    }

    // ---------------------------------------------------------- rider metrics

    #[test]
    fn rider_metrix_chain_uber_grind_matches() {
        check(
            ssx3_ee::s00::f_00119958__generated,
            rider_metrix_chain_uber_grind,
            8,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x0011_9958);
                let m = DATA + 0x100;
                clear_2(c, m, 0x200);
                w(c, m + 0x54, r.next_u32() % 8);
                w(c, m + 0x74, r.next_u32() % 8);
                w(c, m + 0x30, r.f32(4.0));
                w(c, m + 0x24, r.f32(4.0));
                w(c, m + 0x20, r.next_u32());
                w(c, m + 0xc, r.next_u32());
                c.g[A0][0] = sx(m);
                c.g[A1][0] = (seed % 500) as u64;
            },
        );
    }

    #[test]
    fn rider_metrix_finish_board_press_matches() {
        check(
            ssx3_ee::s00::f_00119a38__generated,
            rider_metrix_finish_board_press,
            8,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x0011_9a38);
                let m = DATA + 0x100;
                clear_2(c, m, 0x200);
                w(
                    c,
                    m + 0x30,
                    if seed % 2 == 0 {
                        r.f32(1.0)
                    } else {
                        r.next_u32() | 0x8000_0000
                    },
                );
                w(c, m + 0x24, r.f32(4.0));
                w(c, m + 0x20, r.next_u32());
                w(c, m + 0xc, r.next_u32());
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn rider_metrix_start_spin_trick_matches() {
        check(
            ee::s00::f_00119c98__generated,
            rider_metrix_start_spin_trick,
            8,
            &|c, seed| {
                let _ = base_3(c, seed, 0x119c98, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 6) as u64;
                wf(c, OBJ + 0x14, if seed % 3 == 0 { 0.0 } else { 1.0 });
                w(c, OBJ + 0x60, 0);
            },
        );
    }

    #[test]
    fn rider_metrix_on_split_marker_matches() {
        diff(
            ee::s00::f_001194c0__generated,
            rider_metrix_on_split_marker,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x11_94c0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                w(c, DATA + 0x1ac, DATA + 0x2000);
                w(c, DATA + 0x2000 + 0x6c0, 0);
                w(c, DATA + 0x2000 + 0x6c4, r.next_u32());
                w(c, DATA + 0x2000 + 0x6c8, r.next_u32());
                w(c, DATA + 0x2000 + 0x6cc, r.next_u32());
            },
        );
    }

    /// The metrix, its rider and the objects the HUD callees walk through.
    pub(crate) const M: u32 = DATA + 0x4000;

    pub(crate) const RIDER_2: u32 = DATA + 0x6000;

    pub(crate) const OWNER: u32 = DATA + 0x6800;

    pub(crate) const BOARD: u32 = DATA + 0x6a00;

    pub(crate) const ANGLE_OBJ: u32 = DATA + 0x6c00;

    pub(crate) const BOARD_VT: u32 = DATA + 0x6d00;

    pub(crate) const SLOTS: u32 = DATA + 0x10000;

    pub(crate) const SLOT_COUNT: u32 = 0x2c;

    /// The BE-library singleton and the audio system the HUD calls into.
    pub(crate) const BELIB: u32 = DATA + 0x30000;

    pub(crate) const AUDIO: u32 = DATA + 0x31000;

    /// `world -> +0x84 manager -> +0xc frames`, and the audio "playing" list
    /// `manager+0x28 -> (a value < 10)` that makes every audio helper return
    /// at_3 once.
    pub(crate) const WORLD_3: u32 = DATA + 0x32000;

    pub(crate) const MANAGER: u32 = DATA + 0x32100;

    pub(crate) const FRAMES: u32 = DATA + 0x32200;

    pub(crate) const PLAYING: u32 = DATA + 0x32300;

    #[test]
    fn rider_metrix_update_trick_hud_matches() {
        check_lohi(
            ee::s00::f_00117fe0__generated,
            rider_metrix_update_trick_hud,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x7fe0, 0);
                elf_image(c);
                // Globals: the BE library (0x14dc80), the audio system
                // (0x28b180) and the world chain (0x1298c8).
                c.bus.write32(GAME_GP.wrapping_sub(0x1f04), BELIB);
                c.bus.write32(GAME_GP.wrapping_add(0x410), AUDIO);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD_3);
                c.bus.write32(WORLD_3 + 0x84, MANAGER);
                c.bus.write32(MANAGER + 0xc, FRAMES);
                c.bus.write32(MANAGER + 0x28, PLAYING);
                c.bus.write32(PLAYING, 1);
                c.bus.write32(FRAMES + 8, r.next_u32() % 4096);

                // The metrix: a slot table (so the body runs) and the
                // trick-score inputs (`GetRoundedTrickPoints` +0x1c4 * +0x14).
                c.bus.write32(M + mx::SLOTS, SLOTS);
                c.bus.write32(M + 0x1b4, SLOT_COUNT);
                c.bus.write32(M + mx::RIDER, RIDER_2);
                c.bus.write32(M + mx::TRICK_BOOST, f(1.0));
                let scale: f32 = match seed % 3 {
                    0 => 0.0,
                    1 => 0.001,
                    _ => -0.001,
                };
                c.bus.write32(M + 0x1c4, f(scale));
                c.bus.write32(M + 0x1c, r.f32(2.0));
                c.bus.write32(M + mx::TRICK_COUNT, r.next_u32() % 4);
                c.bus.write32(M + mx::TRICK_FLAG, (seed % 3 == 0) as u32);
                c.bus.write32(M + mx::BONUS, r.next_u32() % 5000);
                c.bus.write32(M + mx::COMBO_COUNT, r.next_u32() % 5);
                c.bus.write32(M + mx::COMBO_POINTS, r.next_u32() % 8000);
                c.bus.write32(M + mx::COMBO_TIMER, r.f32(1.0));
                c.bus.write32(M + 0x198, r.next_u32() % 5000);

                // The HUD records: some are free (id 0x34), some are live.
                for i in 0..SLOT_COUNT {
                    let rec = SLOTS + i * mx::SLOT_SIZE;
                    let id = if r.next_u32() % 3 == 0 { 0x34 } else { i };
                    c.bus.write32(rec, id);
                    c.bus.write32(rec + 4, r.f32(1.0));
                    c.bus.write32(rec + 8, r.f32(1.0));
                    c.bus.write32(rec + 0xc, r.next_u32() % 10);
                    c.bus.write32(rec + 0x14, r.next_u32() % 1000);
                    c.bus.write32(rec + 0x1c, r.f32(1.0));
                    c.bus.write32(rec + 0x20, r.f32(2.0));
                    c.bus.write32(rec + 0x98, i);
                }

                // The rider and its owner: sub-state 2/5, boost mode 3 and
                // the meter/blend inputs the board and HUD-angle calls read.
                c.bus.write32(RIDER_2 + rider::OWNER, OWNER);
                c.bus.write32(RIDER_2 + rider::BOARD, BOARD_VT);
                c.bus
                    .write32(RIDER_2 + rider::BOOST_LEVEL, r.next_u32() % 14);
                c.bus.write32(RIDER_2 + rider::BOOST_MODE, r.next_u32() % 8);
                c.bus.write32(RIDER_2 + rider::BOOST_METER, r.f32(1.0));
                c.bus.write32(RIDER_2 + rider::BOOST_STRENGTH, r.f32(1.0));
                c.bus.write16(BOARD_VT + 0x38, 0);
                c.bus.write32(BOARD_VT + 0x3c, FAKE);

                let sub = [2, 5, 8, 0][(seed % 4) as usize];
                c.bus.write32(OWNER + 0xde4, sub);
                c.bus.write32(OWNER + rider::OWNER_MODE, r.next_u32() % 8);
                c.bus.write32(OWNER + rider::OWNER_METER, r.f32(1.0));
                c.bus.write32(OWNER + 0x1f0, ANGLE_OBJ);
                c.bus.write32(OWNER + 0x230, BOARD);
                c.bus.write32(ANGLE_OBJ + 0x2a4, r.f32(0.5));
                c.bus.write32(ANGLE_OBJ + 0x2b0, r.f32(0.5));
                for off in [0x18, 0x1c, 0x50] {
                    c.bus.write32(BOARD + off, r.f32(1.0));
                }

                c.g[A0][0] = sx(M);
            },
        );
    }

    /// The trick HUD's two hand-converted per-tick quantities: the banner's
    /// tracking step (a `1/30` `.lit4` entry: two 60 Hz frames of progress per
    /// tick) and its oscillation phase (`6°` per 60 Hz frame). A tick at `r` Hz
    /// must carry `60 / r` of the 60 Hz amount, so a second of ticks covers as
    /// much of the animation as a second at 60. At 60 both keep the game's
    /// values (the entries are loaded unchanged).
    #[test]
    fn rider_metrix_update_trick_hud_follows_the_rate() {
        use crate::tick;
        // The game's `6°` in radians (the entry at gp-0x7ae0).
        const SIX_DEGREES: FpuF32 = FpuF32(0x3dd6_7751);
        let mut c = crate::difftest::fresh();
        tick::set_for_test(&mut c, 60);
        assert_eq!(
            hud_wave_step(&mut c, SIX_DEGREES),
            SIX_DEGREES,
            "60 Hz phase"
        );
        assert_eq!(hud_step_30(&mut c).to_f32(), 1.0f32 / 30.0, "60 Hz step");
        assert_eq!(hud_step_150(&mut c), FpuF32(0x3bda_740f), "60 Hz bar step");
        for hz in [120u32, 240] {
            tick::set_for_test(&mut c, hz);
            // The same progress per tick, measured in 60 Hz frames.
            let frames = hz / 60;
            let step = hud_step_30(&mut c).to_f32() * frames as f32;
            assert!(
                (step - 1.0f32 / 30.0).abs() < 1e-9,
                "{hz} Hz: {frames} ticks of the step cover {step}, not 1/30"
            );
            // ... and the same phase per tick, so the wave keeps its period.
            let phase = hud_wave_step(&mut c, SIX_DEGREES).to_f32() * frames as f32;
            assert!(
                (phase - SIX_DEGREES.to_f32()).abs() < 1e-7,
                "{hz} Hz: {frames} ticks of the wave cover {phase} rad, not 6°"
            );
            // The boost bar's step, and the band it snaps inside: one tick
            // wide, so a second of ticks moves the bar as far as at 60.
            let bar = hud_step_150(&mut c).to_f32() * frames as f32;
            let want_bar = FpuF32(0x3bda_740f).to_f32();
            assert!(
                (bar - want_bar).abs() < 1e-9,
                "{hz} Hz: {frames} ticks of the bar step cover {bar}, not {want_bar}"
            );
            let one_tick = FpuF32(tick::step_bits(hz)); // the 1/60 entry, patched
            for (above, want) in [
                (true, 1.0f32 + 1.0 / hz as f32),
                (false, 1.0 - 1.0 / hz as f32),
            ] {
                let band = hud_band(one_tick, above).to_f32();
                assert!(
                    (band - want).abs() < 1e-6,
                    "{hz} Hz: band {band}, not {want}"
                );
            }
        }
    }

    /// `RiderMetrix_Update`'s smoothed speed (`avg = avg*0.9 + speed*0.1` per
    /// tick): after one second of ticks the average must be the same at any
    /// rate, so the filter's time constant (and the peak speed it feeds) does
    /// not change with the tick rate.
    #[test]
    fn rider_metrix_update_smoothed_speed_follows_the_rate() {
        use crate::tick;
        const TARGET: f32 = 100.0;
        let mut c = crate::difftest::fresh();
        let run = |c: &mut Cpu, hz: u32| -> f32 {
            tick::set_for_test(c, hz);
            // The two factors the function multiplies by (`0.9` and `0.1`).
            let a = smoothing_60(c, FpuF32::from_f32(0.1));
            let keep = FpuF32::ONE - a;
            let mut avg = FpuF32::ZERO;
            for _ in 0..hz {
                avg = avg * keep + FpuF32::from_f32(TARGET) * a;
            }
            avg.to_f32()
        };
        let at_60 = run(&mut c, 60);
        // One time constant (~0.158 s) of a 100 unit/s step.
        assert!(at_60 > 99.0 && at_60 < 100.0, "{at_60}");
        for hz in [120u32, 240] {
            let at = run(&mut c, hz);
            assert!(
                (at - at_60).abs() < 1e-3,
                "{hz} Hz: {at} after 1 s, 60 Hz: {at_60}"
            );
        }
    }

    /// `RiderMetrix_Update`'s continuous held-trick award (+0x3c into the
    /// score +0x14 while the +0x40 timer runs): the award is per tick, so two
    /// ticks at 120 Hz (four at 240) must add what one tick at 60 adds.
    #[test]
    fn rider_metrix_update_held_bonus_follows_the_rate() {
        use crate::tick;
        const HELD: f32 = 400.0 / 600_000.0;
        // A quiet metrix: only the held-trick timer runs, so the score
        // moves only through the held award.
        let run = |hz: u32, ticks: u32| -> f32 {
            let mut c = crate::difftest::fresh();
            machine(&mut c, 0x3c40, 0x3c, 0);
            elf_image(&mut c);
            tick::set_for_test(&mut c, hz);
            let m = DATA + 0x8000;
            // The BELibrary singleton (gp-0x1f04) already exists: otherwise
            // `BELibrary_GetInstance` builds it and waits forever on its
            // rewards file.
            c.bus.write32(GAME_GP - 0x1f04, DATA + 0xa000);
            c.bus.write32(m + 0x1ac, RIDER);
            for k in 0..3 {
                c.bus.write32(RIDER + 0x1e0 + k * 4, 10.0f32.to_bits());
            }
            // dt = time scale * the `.lit4` step (gp-0x7b00).
            c.bus.write32(RIDER + 0x300, 1.0f32.to_bits());
            c.bus.write32(GAME_GP - 0x7b00, tick::step_bits(hz));
            // Only the held-trick timer runs.
            for off in [0xa4, 0x30, 0x24, 0x2c, 0x6c, 0x78] {
                c.bus.write32(m + off, (-1.0f32).to_bits());
            }
            c.bus.write32(m + 0x40, 0.0f32.to_bits());
            c.bus.write32(m + 0x14, 0.0f32.to_bits());
            // A 400 point/sec hold (`RiderMetrix_AddTrick` stores
            // points/sec / 600000).
            c.bus.write32(m + 0x3c, HELD.to_bits());
            c.bus.write32(m + 0x84, 0);
            c.bus.write32(m + 0x8c, 0);
            c.bus.write32(m + 0x1b0, 0);
            c.bus.write32(m + 0x1b4, 0);
            for _ in 0..ticks {
                c.g[A0][0] = sx(m);
                rider_metrix_update(&mut c);
            }
            f32::from_bits(c.bus.read32(m + 0x14))
        };
        let at_60 = run(60, 1);
        assert_eq!(at_60, HELD, "60 Hz");
        for hz in [120u32, 240] {
            let ticks = hz / 60;
            let at = run(hz, ticks);
            assert!(
                (at - at_60).abs() < at_60 * 1e-6,
                "{hz} Hz: {ticks} ticks add {at}, 60 Hz adds {at_60}"
            );
        }
    }

    // ------------------------------------------------ Rider_UpdateTargetSpeed

    pub(crate) mod target_speed_fixture {
        use super::super::target_speed::*;
        use crate::difftest::events::{schedule, Events};
        use crate::difftest::fakes::{machine, vtable};
        use crate::difftest::{fill, put_words, Rng, DATA, GAME_GP};
        use ssx3_ee_rt::cpu::{reg::*, sx};
        use ssx3_ee_rt::Cpu;

        pub const RIDER: u32 = DATA + 0x20_0000;
        const WORLD: u32 = DATA + 0x21_0000;
        const COURSE: u32 = DATA + 0x21_0100;
        const OWNER: u32 = DATA + 0x21_0200;
        const SEGMENTS: u32 = DATA + 0x22_0000;
        const BE_LIBRARY_GP: u32 = 0x1f04;
        const BE_LIBRARY_INDEX_GP: u32 = 0x1efc;
        const BE_LIBRARIES: u32 = 0x0053_08a8;
        /// The `.lit4` values at gp-0x79dc .. gp-0x79c0.
        const LITS: [u32; 8] = [
            0x3dcc_cccd,
            0x3ecc_cccd,
            0x3f4c_cccd,
            0x4550_5556,
            0x41de_38e4,
            0x3f66_6666,
            0x3f7a_e148,
            0x3f78_51ec,
        ];

        fn pick(r: &mut Rng, xs: &[f32]) -> u32 {
            xs[(r.next_u32() as usize) % xs.len()].to_bits()
        }

        /// A rider on a course segment, the back end (library singleton,
        /// an interface whose query method is a fake, player and character
        /// tables for the attribute factor), the speed table and literals.
        /// `calls`: fake calls that may unwind (0 for the event test).
        pub fn setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
            let mut r = machine(c, seed, 0x11_b3f8, calls);

            // Back end: the library exists; interface 0's vtable slot 8 is
            // a fake (it returns the interface).
            c.bus.write32(GAME_GP - BE_LIBRARY_GP, DATA + 0x30_0000);
            c.bus.write32(GAME_GP - BE_LIBRARY_INDEX_GP, 0);
            fill(c, &mut r, 0x0053_0000, 0x1100 / 4);
            fill(c, &mut r, 0x0053_4f00, 0x1100 / 4);
            let lib = DATA + 0x31_0000;
            c.bus.write32(BE_LIBRARIES, lib);
            let vt = vtable(c, &mut r, &[8]);
            c.bus.write32(lib + 4, vt);
            // Player slots -> rider records (some without a profile).
            for p in 0..8 {
                c.bus.write32(0x0053_05b0 + 4 * p, r.next_u32() % 8);
            }
            for s in 0..8 {
                if r.next_u32() % 3 == 0 {
                    c.bus.write32(0x0053_5b20 + s * 0x1c + 0xc, u32::MAX);
                }
            }

            // The rider.
            fill(c, &mut r, RIDER, 0xc00 / 4);
            c.g[A0][0] = sx(RIDER);
            c.bus.write32(RIDER + RIDER_SLOT, r.next_u32() % 8);
            let ov = if r.next_u32() % 2 == 0 {
                r.next_u32() % 4
            } else {
                0u32.wrapping_sub(r.next_u32() % 3)
            };
            c.bus.write32(RIDER + ATTRIB_OVERRIDE, ov);
            let index = r.next_u32() % 16;
            c.bus.write32(RIDER + SEGMENT_INDEX, index);
            c.bus.write32(RIDER + 0x77c, OWNER);
            // Main state: 0 (table rows by slope), 1 (fixed target), other.
            let state = match seed % 4 {
                0 | 1 => 0,
                2 => 1,
                _ => 2 + r.next_u32() % 20,
            };
            c.bus.write32(OWNER + 0xde0, state);
            let speed = pick(&mut r, &[-0.5, 0.0, 0.05, 0.1, 0.2, 0.4, 0.6, 0.8, 1.2]);
            c.bus.write32(RIDER + SPEED, speed);
            c.bus
                .write32(RIDER + COLUMN_BLEND, r.f32(1.0) & 0x7fff_ffff);
            c.bus.write32(RIDER + STATE, r.next_u32() % 22);
            let line = r.next_u32();
            c.bus.write32(RIDER + LINE_A, line);
            c.bus.write32(
                RIDER + LINE_B,
                if r.next_u32() % 2 == 0 {
                    line
                } else {
                    line ^ 1
                },
            );
            c.bus.write32(RIDER + TARGET, r.f32(40.0) & 0x7fff_ffff);
            c.bus
                .write32(RIDER + TARGET_FLOOR, r.f32(40.0) & 0x7fff_ffff);

            // The world, course and segment slope.
            c.bus.write32(GAME_GP - WORLD_GP, WORLD);
            c.bus.write32(WORLD + WORLD_COURSE, COURSE);
            c.bus.write32(COURSE + COURSE_SEGMENTS, SEGMENTS);
            let slope = if r.next_u32() % 5 == 0 {
                r.edge_f32()
            } else {
                r.f32(1.0)
            };
            c.bus
                .write32(SEGMENTS + index * SEGMENT_SIZE + SEGMENT_SLOPE, slope);

            // Speed table and literals.
            for i in 0..0x100 / 4 {
                let v = r.f32(1.5);
                c.bus.write32(SPEED_TABLE + 4 * i, v);
            }
            put_words(c, GAME_GP - LIT_BAND_1, &LITS);
            c.pc = 0x0011_b3f8;
            r
        }

        /// [`setup`] entered on a running thread with an event due at the
        /// loop's back edge (off / unwind request / delivered).
        pub fn setup_events(c: &mut Cpu, seed: u32) -> Events {
            let mut r = setup(c, seed, 0);
            schedule(c, &mut r, seed, 0x0011_b3f8)
        }
    }

    /// `Rider_UpdateTargetSpeed`: every path (main state 0/1/other, slope
    /// sign, the four speed bands, lines equal or not, stopped, rising or
    /// falling, the state-11 floor), the back-end calls unwinding, LO/HI of
    /// the segment `mult`, and a real event at the table loop's back edge.
    #[test]
    fn rider_update_target_speed_matches() {
        use crate::difftest::check_lohi;
        use crate::difftest::events::check_events;
        use crate::difftest::fakes::covered;
        covered(
            ee::s00::f_0011b3f8__generated,
            super::rider_update_target_speed,
            300,
            &|c, seed| {
                target_speed_fixture::setup(c, seed, 1);
            },
        );
        check_lohi(
            ee::s00::f_0011b3f8__generated,
            super::rider_update_target_speed,
            100,
            &|c, seed| {
                target_speed_fixture::setup(c, seed, 1);
            },
        );
        check_events(
            ee::s00::f_0011b3f8__generated,
            super::rider_update_target_speed,
            90,
            &target_speed_fixture::setup_events,
        );
    }

    /// `RiderMetrix_SetFlipAmount`: random and edge-case old/new flip
    /// amounts and totals (including totals that would go negative), the
    /// game's weight constant or an edge case.
    #[test]
    fn rider_metrix_set_flip_amount_matches() {
        check(
            ee::s00::f_001198d8__generated,
            rider_metrix_set_flip_amount,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0011_98d8, 0);
                c.pc = 0x0011_98d8;
                let k = if seed % 5 == 0 {
                    r.edge_f32()
                } else {
                    0x3cea_aef8
                };
                c.bus.write32(GAME_GP - FLIP_WEIGHT_GP, k);
                let pick = |r: &mut Rng, i: u32| match i % 3 {
                    0 => r.edge_f32(),
                    _ => r.f32(400.0),
                };
                c.bus.write32(DATA + METRIX_FLIP_AMOUNT, pick(&mut r, seed));
                c.bus
                    .write32(DATA + METRIX_ROTATION_TOTAL, pick(&mut r, seed / 3));
                c.f[12] = pick(&mut r, seed / 9);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    /// `RiderMetrix_SetSpinAmount`: random and edge-case old/new spin
    /// amounts and totals (including totals that would go negative), the
    /// game's weight constant or an edge case.
    #[test]
    fn rider_metrix_set_spin_amount_matches() {
        check(
            ee::s00::f_00119898__generated,
            rider_metrix_set_spin_amount,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0011_9898, 0);
                c.pc = 0x0011_9898;
                let k = if seed % 5 == 0 {
                    r.edge_f32()
                } else {
                    0x3c8e_fa35
                };
                c.bus.write32(GAME_GP - SPIN_WEIGHT_GP, k);
                let pick = |r: &mut Rng, i: u32| match i % 3 {
                    0 => r.edge_f32(),
                    _ => r.f32(400.0),
                };
                c.bus
                    .write32(DATA + SPIN_METRIX_SPIN_AMOUNT, pick(&mut r, seed));
                c.bus
                    .write32(DATA + SPIN_METRIX_ROTATION_TOTAL, pick(&mut r, seed / 3));
                c.f[12] = pick(&mut r, seed / 9);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    /// Random and edge-case bonus factors (both signs, huge values that
    /// saturate the conversion), the game's scale and random ones.
    #[test]
    fn rider_metrix_get_trick_bonus_matches() {
        crate::difftest::check_lohi(
            ee::s00::f_00117908__generated,
            rider_metrix_get_trick_bonus,
            300,
            &|c, seed| {
                let mut r = Rng(0x0011_7908 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.pc = 0x0011_7908;
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                let factor = match seed % 4 {
                    0 => r.edge_f32(),
                    1 => r.f32(1.0),
                    _ => r.f32(10.0),
                };
                c.bus.write32(DATA + RIDER_METRIX_TRICK_BONUS, factor);
                let scale = if seed % 5 == 0 {
                    r.edge_f32()
                } else {
                    10000.0f32.to_bits()
                };
                c.bus.write32(GAME_GP - RIDER_METRIX_BONUS_SCALE_GP, scale);
            },
        );
    }

    /// RiderMetrix_FinishTrick with its real callees: a world whose single
    /// player has (not) finished, a zeroed BELibrary dummy and audio system,
    /// a metrix record (cleared, then random score/points/spin/flip/peaks,
    /// best trick, landing flags), the event settings (Conquer-the-Mountain
    /// free ride or not) and every takeoff/style/flag combination.
    #[test]
    fn rider_metrix_finish_trick_matches() {
        crate::difftest::check(
            ee::s00::f_0011a228__generated,
            rider_metrix_finish_trick,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11_a228, 0);
                elf_image(c);
                let be = DATA + 0x20_0000;
                let world = DATA + 0x21_0000;
                let game = world + 0x1000;
                let race = world + 0x2000;
                let player = world + 0x3000;
                let player_state = world + 0x4000;
                let audio = DATA + 0x22_0000;
                for a in [be, world, audio] {
                    for i in 0..0x1000 / 4 {
                        c.bus.write32(a + 4 * i, 0);
                    }
                }
                c.bus.write32(GAME_GP.wrapping_sub(0x1f04), be);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(GAME_GP + 0x410, audio);
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0xc, race);
                // One player; finished when its +0x470 is >= 0.
                c.bus.write32(race + 0x7c, 1);
                c.bus.write32(race + 0x40, player);
                c.bus.write32(player + 0x18, player_state);
                let finished = seed % 10 == 0;
                c.bus.write32(
                    player_state + 0x470,
                    (if finished { 1.0f32 } else { -1.0 }).to_bits(),
                );
                // The event settings.
                let kind = if seed % 3 == 0 {
                    4
                } else {
                    (r.next_u32() % 4) as u8
                };
                c.bus.write8(FINISH_TRICK_EVENT_SETTINGS + 0x48, kind);
                c.bus
                    .write8(FINISH_TRICK_EVENT_SETTINGS + 0x49, (seed / 3 % 2) as u8);

                let m = DATA + 0x1000;
                for i in 0..0x200 / 4 {
                    c.bus.write32(m + 4 * i, 0);
                }
                let score = match seed % 5 {
                    0 => 0.0f32,
                    1 => -((r.next_u32() % 50) as f32),
                    _ => (r.next_u32() % 500) as f32,
                };
                c.bus.write32(m + 0x14, score.to_bits());
                // Trick points (0 on some seeds: nothing scored).
                let points = if seed % 7 == 0 {
                    0.0f32
                } else {
                    (r.next_u32() % 4000) as f32
                };
                c.bus.write32(m + 0x1c4, points.to_bits());
                c.bus.write32(m + 0x34, r.f32(20.0));
                c.bus.write32(m + 0x38, r.f32(20.0));
                for off in [
                    0x44, 0x30, 0x48, 0x24, 0x78, 0x14c, 0x150, 0x154, 0x158, 0x144,
                ] {
                    c.bus.write32(m + off, r.f32(10.0));
                }
                for off in [
                    0x4c, 0x80, 0x58, 0x74, 0x84, 0xfc, 0x100, 0x104, 0x108, 0x114, 0x118, 0x11c,
                ] {
                    c.bus.write32(m + off, r.next_u32() % 100);
                }
                for off in [0x198, 0x19c, 0x110, 0x18c] {
                    c.bus.write32(m + off, r.next_u32() % 3000);
                }
                c.bus.write32(m + 0xa4, r.f32(2.0));
                c.bus.write32(m + 0x20, if seed % 4 == 1 { 1 } else { 0 });
                c.bus.write32(m + 0x28, if seed % 6 == 2 { 1 } else { 0 });
                c.g[A0][0] = sx(m);
                c.g[A1][0] = (r.next_u32() % 2) as u64;
                c.g[A2][0] = (r.next_u32() % 2) as u64;
                c.g[A3][0] = if seed % 4 == 3 { 1 } else { 0 };
                c.g[T0][0] = if seed % 8 == 5 { 1 } else { 0 };
                c.g[T1][0] = if seed % 2 == 0 { 1 } else { 0 };
                c.pc = 0x0011_a228;
            },
        );
    }

    pub(crate) use crate::game::ai::ai::tests::stream_save_setup;

    /// `RiderMetrix_SaveState`.
    #[test]
    fn rider_metrix_save_state_matches() {
        covered(
            ee::s00::f_001174b0__generated,
            rider_metrix_save_state,
            200,
            &|c, seed| stream_save_setup(c, seed, 0x0011_74b0),
        );
    }

    /// `RiderMetrix_InterruptTrick`: idle metrix (no calls) or the real
    /// FinishTrick/ResetTrick on the RiderMetrix_FinishTrick fixture (a
    /// world with one player, finished or not, zeroed BELibrary dummy and
    /// audio system, random trick record).
    #[test]
    fn rider_metrix_interrupt_trick_matches() {
        check(
            ee::s00::f_00119e38__generated,
            rider_metrix_interrupt_trick,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0011_9e38, 0);
                elf_image(c);
                c.pc = 0x0011_9e38;
                let be = DATA + 0x20_0000;
                let world = DATA + 0x21_0000;
                let game = world + 0x1000;
                let race = world + 0x2000;
                let player = world + 0x3000;
                let player_state = world + 0x4000;
                let audio = DATA + 0x22_0000;
                for a in [be, world, audio] {
                    for i in 0..0x1000 / 4 {
                        c.bus.write32(a + 4 * i, 0);
                    }
                }
                c.bus.write32(GAME_GP.wrapping_sub(0x1f04), be);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(GAME_GP + 0x410, audio);
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x7b0c), 1.0f32.to_bits());
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0xc, race);
                c.bus.write32(race + 0x7c, 1);
                c.bus.write32(race + 0x40, player);
                c.bus.write32(player + 0x18, player_state);
                let finished = seed % 5 == 1;
                c.bus.write32(
                    player_state + 0x470,
                    (if finished { 1.0f32 } else { -1.0 }).to_bits(),
                );
                c.bus.write8(0x0053_5bc8 + 0x48, (r.next_u32() % 5) as u8);
                c.bus.write8(0x0053_5bc8 + 0x49, (seed / 3 % 2) as u8);
                let m = DATA + 0x1000;
                for i in 0..0x200 / 4 {
                    c.bus.write32(m + 4 * i, 0);
                }
                c.bus
                    .write32(m + 0x14, ((r.next_u32() % 500) as f32).to_bits());
                let points = if seed % 7 == 0 {
                    0.0f32
                } else {
                    (r.next_u32() % 4000) as f32
                };
                c.bus.write32(m + 0x1c4, points.to_bits());
                for off in [
                    0x34, 0x38, 0x44, 0x30, 0x48, 0x24, 0x78, 0x14c, 0x150, 0x154, 0x158, 0x144,
                ] {
                    c.bus.write32(m + off, r.f32(10.0));
                }
                for off in [
                    0x4c, 0x80, 0x58, 0x74, 0x84, 0xfc, 0x100, 0x104, 0x108, 0x114, 0x118, 0x11c,
                ] {
                    c.bus.write32(m + off, r.next_u32() % 100);
                }
                for off in [0x198, 0x19c, 0x110, 0x18c] {
                    c.bus.write32(m + off, r.next_u32() % 3000);
                }
                c.bus.write32(m + 0xa4, r.f32(2.0));
                c.bus.write32(m + METRIX_INT_TRICKING, r.next_u32() % 2);
                c.bus.write32(m + METRIX_INT_28, r.next_u32() % 2);
                c.bus
                    .write32(m + METRIX_INT_IDLE, if seed % 3 == 0 { 1 } else { 0 });
                c.g[A0][0] = sx(m);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    /// A RiderMetrix record at `DATA + 0x1000` that `RiderMetrix_FinishTrick`
    /// can bank for real: a zeroed BELibrary dummy and audio system, a world
    /// with one race player (finished on some seeds), the event settings, and
    /// random score/points/statistics. Returns the record.
    pub(crate) fn land_trick_metrix_setup(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        elf_image(c);
        let be = DATA + 0x20_0000;
        let world = DATA + 0x21_0000;
        let game = world + 0x1000;
        let race = world + 0x2000;
        let player = world + 0x3000;
        let player_state = world + 0x4000;
        let audio = DATA + 0x22_0000;
        for a in [be, world, audio] {
            for i in 0..0x1000 / 4 {
                c.bus.write32(a + 4 * i, 0);
            }
        }
        for i in 0..0x5000 / 4 {
            c.bus.write32(world + 4 * i, 0);
        }
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), be);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(GAME_GP + 0x410, audio);
        c.bus.write32(world + 0x84, game);
        c.bus.write32(game + 0xc, race);
        c.bus.write32(race + 0x7c, 1);
        c.bus.write32(race + 0x40, player);
        c.bus.write32(player + 0x18, player_state);
        let finished = seed % 10 == 0;
        c.bus.write32(
            player_state + 0x470,
            (if finished { 1.0f32 } else { -1.0 }).to_bits(),
        );
        // The event settings (`D_00535BC8`): kind, game type.
        c.bus.write8(0x0053_5bc8 + 0x48, (r.next_u32() % 5) as u8);
        c.bus.write8(0x0053_5bc8 + 0x49, (seed / 3 % 2) as u8);

        let m = DATA + 0x1000;
        for i in 0..0x200 / 4 {
            c.bus.write32(m + 4 * i, 0);
        }
        let points = if seed % 7 == 0 {
            0.0f32
        } else {
            (r.next_u32() % 4000) as f32
        };
        c.bus.write32(m + 0x1c4, points.to_bits());
        c.bus.write32(m + 0x34, r.f32(20.0));
        c.bus.write32(m + 0x38, r.f32(20.0));
        for off in [
            0x44, 0x30, 0x48, 0x24, 0x78, 0x14c, 0x150, 0x154, 0x158, 0x144,
        ] {
            c.bus.write32(m + off, r.f32(10.0));
        }
        for off in [
            0x4c, 0x80, 0x58, 0x74, 0x84, 0xfc, 0x100, 0x104, 0x108, 0x114, 0x118, 0x11c,
        ] {
            c.bus.write32(m + off, r.next_u32() % 100);
        }
        for off in [0x198, 0x19c, 0x110, 0x18c] {
            c.bus.write32(m + off, r.next_u32() % 3000);
        }
        c.bus.write32(m + 0xa4, r.f32(2.0));
        m
    }

    /// `RiderMetrix_LandTrick`: styled and plain landings, positive, zero
    /// and negative scores (the style bonus), flags on/off, and a trick
    /// chain (`+0x70`) running or not, banked by the real FinishTrick and
    /// ResetTrick.
    #[test]
    fn rider_metrix_land_trick_matches() {
        check(
            ee::s00::f_00119d40__generated,
            rider_metrix_land_trick,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11_9d40, 0);
                let m = land_trick_metrix_setup(c, &mut r, seed);
                let score = match seed % 5 {
                    0 => 0.0f32,
                    1 => -((r.next_u32() % 50) as f32),
                    _ => (r.next_u32() % 500) as f32,
                };
                c.bus.write32(m + METRIX_SCORE, score.to_bits());
                c.bus.write32(
                    m + METRIX_CHAIN,
                    if seed % 3 == 0 {
                        0
                    } else {
                        r.next_u32() % 40 + 1
                    },
                );
                c.bus.write32(m + 0x20, if seed % 4 == 1 { 1 } else { 0 });
                c.bus.write32(m + 0x28, if seed % 6 == 2 { 1 } else { 0 });
                c.g[A0][0] = sx(m);
                c.g[A1][0] = (r.next_u32() % 2) as u64;
                c.g[A2][0] = (r.next_u32() % 2) as u64;
                c.g[A3][0] = if seed % 2 == 0 {
                    0
                } else {
                    (r.next_u32() % 8 + 1) as u64
                };
                c.g[T0][0] = if seed % 8 < 3 {
                    (r.next_u32() % 8 + 1) as u64
                } else {
                    0
                };
                c.pc = 0x0011_9d40;
            },
        );
    }

    /// `RiderMetrix_AddTrick`: a BELibrary whose interface 6 is a fake
    /// (unwinding on some seeds), a trick table at 0x530600 with random
    /// points, ids on both sides of the uber threshold (35), and timers
    /// `+0x40` negative or not.
    #[test]
    fn rider_metrix_add_trick_matches() {
        use crate::difftest::fakes::covered;
        use crate::game::ai::ai::tests::belibrary;
        covered(
            ee::s00::f_00119708__generated,
            rider_metrix_add_trick,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11_9708, 1);
                elf_image(c);
                belibrary(c, &mut r);
                let id = match seed % 4 {
                    0 => 34,
                    1 => 35,
                    _ => r.next_u32() % 60,
                };
                c.bus.write32(0x0053_0600 + id * 8, r.next_u32() % 20000);
                c.bus
                    .write32(0x0053_0604 + id * 8, r.next_u32() % 2_000_000);
                let m = DATA + 0x2000;
                c.bus.write32(m + METRIX_SCORE, r.f32(100.0));
                c.bus.write32(m + METRIX_TRICK_TIMER, r.f32(5.0));
                c.bus.write32(m + 0xa4, r.f32(5.0));
                c.g[A0][0] = sx(m);
                c.g[A1][0] = id as u64;
                c.pc = 0x0011_9708;
            },
        );
    }

    /// RiderMetrix_BankCombo: short combos, no points, banked combos with
    /// and without new records, online/offline in mode 4 (AwardPoints on
    /// its early-out), the HUD calls on an empty message table, and the
    /// back end's interface query as a fake that may unwind.
    #[test]
    fn rider_metrix_bank_combo_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::GAME_GP;
        covered(
            ee::s00::f_00117718__generated,
            rider_metrix_bank_combo,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x117718, 1);
                // Back end: library exists, interface 0 -> fake method.
                let lib = DATA + 0x20_0000;
                let vt = DATA + 0x20_1000;
                c.bus.write32(GAME_GP - 0x1f04, DATA + 0x21_0000);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, lib);
                c.bus.write32(lib + 4, vt);
                c.bus
                    .write16(vt + 8, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0xc, FAKE);
                // World: AwardPoints takes its early-out (rank word 5).
                let world = DATA + 0x22_0000;
                let game = world + 0x1000;
                let rank = world + 0x2000;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0x28, rank);
                c.bus.write32(rank, 5);
                c.bus.write32(GAME_GP - 0x7b10, 0.7f32.to_bits());
                // Event settings: mode 4 online/offline, or another mode.
                let mode = if seed % 3 == 0 {
                    4
                } else {
                    (r.next_u32() % 6) as u8
                };
                c.bus.write8(BANK_COMBO_EVENT_SETTINGS + 0x48, mode);
                c.bus
                    .write8(BANK_COMBO_EVENT_SETTINGS + 0x49, (seed / 3 % 2) as u8);
                // The metrix record; empty HUD message table.
                let m = DATA;
                fill(c, &mut r, m, 0x200 / 4);
                c.bus.write32(m + 0x1b0, 0);
                let count = match seed % 8 {
                    0 => r.next_u32() % 2,
                    1 => 0u32.wrapping_sub(r.next_u32() % 4),
                    _ => 2 + r.next_u32() % 40,
                };
                c.bus.write32(m + METRIX_COMBO_COUNT, count);
                let points = match seed % 7 {
                    0 => 0u32.wrapping_sub(r.next_u32() % 100),
                    1 => 0,
                    2 => 20_000 + r.next_u32() % 50_000,
                    _ => r.next_u32() % 12_000,
                };
                c.bus.write32(m + METRIX_COMBO_POINTS, points);
                c.bus
                    .write32(m + METRIX_BEST_COMBO_POINTS, r.next_u32() % 15_000);
                c.bus
                    .write32(m + METRIX_BEST_COMBO_COUNT, r.next_u32() % 40);
                c.g[A0][0] = sx(m);
                c.pc = 0x0011_7718;
            },
        );
    }

    /// Zero/negative/positive points, combo counts on both sides of the
    /// clamp (multiplier below 0.5, inside, above 2.0), with no HUD, or a
    /// HUD whose combo effect is showing (0x34) or not.
    #[test]
    fn rider_metrix_add_combo_points_matches() {
        use crate::difftest::GAME_GP;
        use combo_points as k;
        check(
            ee::s00::f_00117638__generated,
            rider_metrix_add_combo_points,
            300,
            &|c, seed| {
                let mut r = Rng(0x1176_3800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0011_7638;
                c.g[GP][0] = sx(GAME_GP);
                c.bus.write32(GAME_GP - k::GP_STEP, 0x3d4c_cccd); // 0.05
                let this = DATA + 0x100;
                let hud = DATA + 0x400;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = match seed % 5 {
                    0 => 0,
                    1 => sx(r.next_u32() | 0x8000_0000),
                    2 => sx(r.next_u32() % 100 + 1),
                    _ => sx(r.next_u32() % 100_000 + 1),
                };
                let count = match (seed / 5) % 4 {
                    0 => (r.next_u32() % 20) as i32 - 20,   // multiplier < 0.5
                    1 => (r.next_u32() % 30) as i32,        // inside
                    2 => 30 + (r.next_u32() % 1000) as i32, // > 2.0
                    _ => r.next_u32() as i32,
                };
                c.bus.write32(this + k::COMBO_COUNT, count as u32);
                c.bus.write32(this + k::COMBO_SCORE, r.next_u32());
                c.bus.write32(this + k::PEAK, r.f32(10.0));
                c.bus
                    .write32(this + k::HUD, if seed % 3 == 0 { 0 } else { hud });
                c.bus.write32(
                    hud + k::HUD_COMBO_FX,
                    if seed % 2 == 0 {
                        0x34
                    } else {
                        r.next_u32() % 0x40
                    },
                );
                c.bus.write32(hud + k::HUD_COMBO_VALUE, r.next_u32());
            },
        );
    }

    /// Histories of 0..3 entries drawn from a small id set (so repeats of the
    /// trick, at the last entry or earlier, happen), and a full history.
    #[test]
    fn rider_metrix_add_trick_history_matches() {
        check(
            ee::s00::f_00119068__generated,
            rider_metrix_add_trick_history,
            300,
            &|c, seed| {
                let mut r = Rng(0x1190_6800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0011_9068;
                let this = DATA + 0x100;
                let filled = seed % 4;
                for i in 0..3 {
                    let v = if i < filled { 1 + r.next_u32() % 3 } else { 0 };
                    c.bus.write32(this + TRICK_HISTORY + 4 * i, v);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(1 + r.next_u32() % 3);
            },
        );
    }

    /// The same with events falling due at the loop's back edge (off,
    /// delivered, or a VBlank requesting an unwind): two or three entries,
    /// mostly a new trick so the walk reaches the back edges.
    #[test]
    fn rider_metrix_add_trick_history_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s00::f_00119068__generated,
            rider_metrix_add_trick_history,
            200,
            &|c, seed| {
                let mut r = Rng(0x1190_6801 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA + 0x100;
                for i in 0..3 {
                    let v = if i < 2 + (seed / 3) % 2 {
                        1 + r.next_u32() % 2
                    } else {
                        0
                    };
                    c.bus.write32(this + TRICK_HISTORY + 4 * i, v);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(if seed % 4 == 0 { 1 } else { 3 });
                schedule(c, &mut r, seed, 0x0011_9068)
            },
        );
    }

    #[test]
    fn trick_lookup_bonus_trick_matches() {
        check(
            ee::s00::f_0011b1a8__generated,
            trick_lookup_bonus_trick,
            300,
            &|c, seed| {
                let mut r = Rng(0x11b1_a800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0011_b1a8;
                fill(c, &mut r, crate::difftest::SP - 0x80, 0x20);
                let id = DATA + 0x40;
                let (w0, w1) = (r.next_u32(), r.next_u32());
                c.bus.write32(id, w0);
                c.bus.write32(id + 4, w1);
                let fields = [
                    (w0 >> 10) & 0x3,
                    (w0 >> 12) & 0xf,
                    (w0 >> 16) & 0x7,
                    (w0 >> 19) & 0x7,
                    w0 >> 28,
                    (w1 >> 3) & 0x7f,
                    (w1 >> 11) & 0x7f,
                ];
                // The matching entry (24: none); the others match a prefix
                // of the fields and differ at the next one.
                let target = seed % 25;
                for i in 0..24u32 {
                    let e = BONUS_TRICKS + i * BONUS_TRICK_SIZE;
                    c.bus.write32(e + BONUS_TRICK_GRADE, r.next_u32());
                    c.bus.write32(e + BONUS_TRICK_POINTS, r.next_u32());
                    let same = if i == target { 7 } else { r.next_u32() % 7 };
                    for (k, &f) in fields.iter().enumerate() {
                        let k = k as u32;
                        let b = if k < same {
                            f
                        } else if k == same {
                            f ^ 1
                        } else {
                            r.next_u32() & 0xff
                        };
                        c.bus.write8(e + 8 + k, b as u8);
                    }
                    c.bus.write8(e + 0xf, r.next_u32() as u8);
                }
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = id as u64;
            },
        );
    }

    #[test]
    fn rider_metrix_complete_trick_matches() {
        check(
            ee::s00::f_001197d8__generated,
            rider_metrix_complete_trick,
            300,
            &|c, seed| {
                let mut r = Rng(0x1197_d800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0011_97d8;
                fill(c, &mut r, crate::difftest::SP - 0x80, 0x20);
                let m = DATA;
                let rider = DATA + 0x400;
                fill(c, &mut r, m, 0x80);
                // Trick history (+0x60..+0x6c): small ids, some empty slots.
                for i in 0..4 {
                    let v = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        r.next_u32() % 50
                    };
                    c.bus.write32(m + 0x60 + 4 * i, v);
                }
                for off in [METRIX_TRICK_SCORE, METRIX_SCORE_TOTAL, METRIX_BEST_SCORE] {
                    let v = if r.next_u32() % 4 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(5000.0)
                    };
                    c.bus.write32(m + off, v);
                }
                c.bus.write32(m + METRIX_RIDER, rider);
                c.bus.write32(
                    rider + METRIX_RIDER_LEVEL,
                    (r.next_u32() % 12).wrapping_sub(2),
                );
                let id = match seed % 5 {
                    0 => r.next_u32() % 19,
                    1 => 19 + r.next_u32() % 16,
                    2 => r.next_u32(),
                    _ => 35 + r.next_u32() % 20,
                };
                c.g[A0][0] = m as u64;
                c.g[A1][0] = sx(id);
            },
        );
    }

    /// RiderMetrix_CountTrickRepeats: busy states, the two never-counted
    /// trick classes, and histories with exact copies, near misses (one byte
    /// off) and random entries; events delivered inside the byte-compare
    /// loops.
    #[test]
    fn rider_metrix_count_trick_repeats_matches() {
        use crate::difftest::events::{check_events, schedule, Events};
        use crate::difftest::{scramble, Rng, DATA};
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0x0011_90f0 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let me = DATA;
            let id = DATA + 0x200 + 4 * (seed % 2);
            let mut w0 = r.next_u32();
            let mut w1 = r.next_u32();
            match r.next_u32() % 8 {
                0 => w0 = (w0 & !0x0fc0_0000) | 0x0040_0000,
                1 => w1 = (w1 & !0x0003_f800) | 0x800,
                _ => {
                    // Keep both classes countable.
                    if w0 & 0x0fc0_0000 == 0x0040_0000 {
                        w0 ^= 0x0100_0000;
                    }
                    if w1 & 0x0003_f800 == 0x800 {
                        w1 ^= 0x1000;
                    }
                }
            }
            c.bus.write32(id, w0);
            c.bus.write32(id + 4, w1);
            let trick = (w1 as u64) << 32 | w0 as u64;
            for off in COUNT_TRICK_REPEATS_BUSY {
                c.bus.write32(me + off, 0);
            }
            if r.next_u32() % 6 == 0 {
                let off = COUNT_TRICK_REPEATS_BUSY[(r.next_u32() % 5) as usize];
                c.bus.write32(me + off, r.next_u32() | 1);
            }
            for i in 0..10u32 {
                let e = me + COUNT_TRICK_REPEATS_HIST + 8 * i;
                let v = match r.next_u32() % 3 {
                    0 => trick,
                    1 => trick ^ (1u64 << (8 * (r.next_u32() % 8) + r.next_u32() % 8)),
                    _ => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                };
                c.bus.write64(e, v);
            }
            c.bus
                .write32(me + COUNT_TRICK_REPEATS_HIST_IDX, r.next_u32() % 10);
            c.g[A0][0] = sx(me);
            c.g[A1][0] = sx(id);
            schedule(c, &mut r, seed, 0x0011_90f0)
        };
        check_events(
            ee::s00::f_001190f0__generated,
            rider_metrix_count_trick_repeats,
            150,
            &setup,
        );
    }

    use crate::difftest::RA as SENTINEL;

    // -- 0x00117ae8 RiderMetrix_ShowHUDText -----------------------------------

    /// `RiderMetrix_ShowHUDText`: no message table (-1), a fixed type below
    /// 34 and a searched one (a free slot, or the smallest ratio), the real
    /// `RiderMetrix_FindHUDSlot` and `cHUDFX_Show` with a text or a null
    /// one.
    #[test]
    fn rider_metrix_show_hud_text_matches() {
        use crate::difftest::{check_lohi, scramble, Rng, DATA};
        const SELF: u32 = DATA;
        const TABLE: u32 = DATA + 0x2000;
        const TEXT: u32 = DATA + 0x4000;
        check_lohi(
            ee::s00::f_00117ae8__generated,
            rider_metrix_show_hud_text,
            240,
            &|c, seed| {
                let mut r = Rng(0x117a_e800 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0011_7ae8;
                if seed % 5 == 0 {
                    c.bus.write32(SELF + RIDER_METRIX_HUD_TABLE, 0);
                } else {
                    c.bus.write32(SELF + RIDER_METRIX_HUD_TABLE, TABLE);
                    let count = 0x23 + r.next_u32() % 4;
                    c.bus.write32(SELF + 0x1b4, count);
                    // A free slot among the searched ones (35..count), if any.
                    let free = if seed % 3 == 2 && count > 0x23 {
                        0x23 + r.next_u32() % (count - 0x23)
                    } else {
                        u32::MAX
                    };
                    for i in 0..count {
                        let slot = TABLE + RIDER_METRIX_HUD_SLOT_SIZE * i;
                        c.bus.write32(slot, if i == free { 0x34 } else { 0x35 });
                        c.bus.write32(slot + 4, r.edge_f32());
                        c.bus.write32(slot + 8, r.edge_f32());
                        c.bus.write32(slot + 0xc, r.next_u32());
                        c.bus.write32(slot + 0x18, r.next_u32());
                    }
                }
                let n = r.next_u32() % 30;
                for i in 0..n {
                    c.bus.write8(TEXT + i, (r.next_u32() % 255 + 1) as u8);
                }
                c.bus.write8(TEXT + n, 0);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = if seed % 3 == 0 {
                    (34 + r.next_u32() % 4) as u64
                } else {
                    (r.next_u32() % 34) as u64
                };
                c.g[A2][0] = if r.next_u32() % 8 == 0 { 0 } else { sx(TEXT) };
                c.g[A3][0] = r.next_u32() as u64;
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn rider_metrix_reset_combo_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, DATA};
        check(
            ee::s00::f_001175f8__generated,
            rider_metrix_reset_combo,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0011_75f8, 0);
                let this = DATA + 0x200;
                fill(c, &mut r, this + 0x90, 0x10);
                // The HUD table: absent in half the seeds, else an offset
                // into valid RAM so the fixed-slot write lands on RAM.
                let table = if seed % 2 == 0 { 0 } else { 0x4000 };
                c.bus.write32(this + 0x1b0, table);
                c.bus.write32(this + 0x1b4, (seed % 0x28) as u32);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32());
                c.pc = 0x0011_75f8;
            },
        );
    }

    // -- 0x00118ff8 RiderMetrix_ShowTrickName --------------------------------

    /// A trick id (random bit fields) and name-part tables whose slots point
    /// at empty or short strings, so the real `Trick_BuildName` copies both
    /// shapes; the real `RiderMetrix_ShowHUDText` stops at its null slot
    /// (self+0x1b0), and the real script-manager post runs.
    #[test]
    fn rider_metrix_show_trick_name_matches() {
        check(
            ee::s00::f_00118ff8__generated,
            rider_metrix_show_trick_name,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0011_8ff8, 0);
                let this = DATA + 0x1000;
                let id = DATA + 0x2000;
                let strings = DATA + 0x3000;
                c.bus.write32(this + 0x1b0, 0);
                c.bus.write32(id, r.next_u32());
                c.bus.write32(id + 4, r.next_u32());
                for k in 0..4u32 {
                    let s = strings + 0x10 * k;
                    let len = if k == 0 { 0 } else { k + r.next_u32() % 6 };
                    for i in 0..len {
                        c.bus.write8(s + i, b'a' + (r.next_u32() % 26) as u8);
                    }
                    c.bus.write8(s + len, 0);
                }
                for t in [
                    0x0043_d118u32,
                    0x0043_d130,
                    0x0043_cf40,
                    0x0043_cf60,
                    0x0043_cf70,
                    0x0043_cfb0,
                    0x0043_cfc8,
                    0x0043_cfe0,
                    0x0043_d0c0,
                    0x0043_d160,
                    0x0043_d2b8,
                    0x0043_d2d0,
                    0x0043_d2e0,
                    0x0043_d100,
                    0x0043_d320,
                    GAME_GP - 0x2100,
                ] {
                    for slot in 0..0x80u32 {
                        c.bus
                            .write32(t + 4 * slot, strings + 0x10 * (r.next_u32() % 4));
                    }
                }
                c.bus.write32(GAME_GP + 0xce8, DATA + 0x4000);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(id);
                c.g[A2][0] = sx(r.next_u32() % 4);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0011_8ff8;
            },
        );
    }

    /// RiderMetrix_BankPendingScore: the trick points, the pending score and
    /// the banked total each negative, zero or positive (the real HUD
    /// functions return at once on an empty message list).
    #[test]
    fn ridermetrix_bank_pending_score_matches() {
        check(
            ee::s00::f_0011a7a8__generated,
            ridermetrix_bank_pending_score,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11_a7a8, 0);
                let this = DATA;
                c.g[A0][0] = sx(this);
                // No HUD message list: Remove/Show return immediately.
                c.bus.write32(this + 0x1b0, 0);
                c.bus.write32(this + 0x1b4, 0);
                // Trick points = count (+0x14) * factor (+0x1c4) * gp const.
                c.bus.write32(
                    this + 0x14,
                    if seed % 3 == 0 {
                        0
                    } else {
                        (r.next_u32() % 0x1000) as f32 as u32
                    },
                );
                c.bus.write32(this + 0x1c4, 1.0f32.to_bits());
                c.bus
                    .write32(this + 0x18, (r.next_u32() % 0x1000) as f32 as u32);
                c.bus.write32(GAME_GP - 0x7b08, 1.0f32.to_bits());
                c.bus
                    .write32(this + 0xa0, [0, 1, 0xffff_ffff, 77][(seed % 4) as usize]);
                c.bus.write32(this + 0x84, (seed % 3) as u32);
            },
        );
    }

    /// Random score factors (edge-case floats included) and the 10000 /
    /// 10 constants the function loads from the game image.
    #[test]
    fn rider_metrix_get_trick_score_matches() {
        use crate::difftest::elf_image;
        use crate::difftest::fakes::machine;
        const SELF: u32 = DATA + 0x100;
        check(
            ee::s00::f_00117990__generated,
            rider_metrix_get_trick_score,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11_7990, 0);
                elf_image(c);
                c.pc = 0x0011_7990;
                c.bus.write32(GAME_GP - 0x7b04, 10_000.0f32.to_bits());
                c.bus.write32(SELF + 0x18, r.edge_f32());
                c.bus.write32(SELF + 0x14, r.edge_f32());
                c.bus.write32(SELF + 0x1c4, r.f32(2.0));
                c.g[A0][0] = sx(SELF);
            },
        );
    }

    use crate::difftest::fakes::fake;

    pub(crate) use crate::game::object::movenode::tests::oc336_fake;
    pub(crate) use crate::game::object::movenode::tests::oc336_fall_through;
    pub(crate) use crate::game::object::movenode::tests::oc336_install;
    pub(crate) use crate::game::object::movenode::tests::OC336_CTRL;
    pub(crate) use crate::game::object::movenode::tests::OC336_MAGIC;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;
    use std::sync::Once;

    // -- RiderMetrix_OnWipeout -------------------------------------------

    #[test]
    fn rider_metrix_on_wipeout_matches() {
        check(
            ee::s00::f_00119b08__generated,
            rider_metrix_on_wipeout,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x11_9b08, 0);
                c.pc = 0x0011_9b08;
                let m = DATA + 0x100;
                // No HUD message table: the HUD helpers take their null
                // path, whatever the score fields hold.
                c.bus.write32(m + 0x1b0, 0);
                c.bus.write32(m + 0x124, seed % 5);
                c.bus.write32(m + 0x12c, seed % 7);
                c.bus
                    .write32(m + 0x1a0, if seed % 4 == 0 { 0 } else { 1000 });
                c.bus.write32(m + 0x84, if seed % 3 == 0 { 0 } else { 250 });
                c.bus.write32(m + 0xa0, if seed % 3 == 1 { 0 } else { 40 });
                // Trick scale 1.0, points 0 or 10: the rounded trick points
                // straddle zero, so the -0.25 penalty branch happens.
                c.bus.write32(m + 0x14, 0x3f80_0000);
                c.bus
                    .write32(m + 0x1c4, if seed % 2 == 0 { 0 } else { 10.0f32.to_bits() });
                c.bus.write32(m + 0x1c, 0);
                c.bus.write32(GAME_GP - 0x7b08, 0x3f80_0000); // 1.0
                c.bus.write32(GAME_GP - 0x7b0c, 0x3f80_0000);
                c.g[A0][0] = sx(m);
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    // -------------------------------------- RiderMetrix_GetRecoveryBoost

    #[test]
    fn rider_metrix_get_recovery_boost_matches() {
        check(
            ee::s00::f_00119bb0__generated,
            rider_metrix_get_recovery_boost,
            300,
            &|c, seed| {
                let mut r = Rng(0x0011_9bb0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0011_9bb0;
                let this = DATA;
                // No HUD table: ShowHUDNumber returns -1 without calling.
                c.bus.write32(this + 0x1b0, 0);
                c.bus.write32(
                    GAME_GP - 0x7a08,
                    if seed % 5 == 0 {
                        r.edge_f32()
                    } else {
                        r.next_u32()
                    },
                );
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }

    pub(crate) static ANNOUNCE_PREV_LOOKUP: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static ANNOUNCE_PREV_REPEATS: AtomicUsize = AtomicUsize::new(0);

    /// `Trick_LookupBonusTrick`: log the call, then return the setup's points
    /// (0, -1 or a positive count) so both branches of the caller are
    /// reached.
    pub(crate) fn announce_fake_lookup_bonus(c: &mut Cpu) {
        if c.bus.read32(OC336_CTRL) != OC336_MAGIC {
            return oc336_fall_through(c, &ANNOUNCE_PREV_LOOKUP, ee::s00::f_0011b1a8__generated);
        }
        let ra = c.g[RA][0] as u32;
        let points = c.bus.read32(OC336_CTRL + 4) as i32;
        fake(c, 0x0011_b1a8, ra);
        if c.pc == ra {
            c.g[V0][0] = sx(points as u32);
        }
    }

    /// `RiderMetrix_CountTrickRepeats`: log the call, then return the setup's
    /// repeat count (zero or non-zero).
    pub(crate) fn announce_fake_count_repeats(c: &mut Cpu) {
        if c.bus.read32(OC336_CTRL) != OC336_MAGIC {
            return oc336_fall_through(c, &ANNOUNCE_PREV_REPEATS, ee::s00::f_001190f0__generated);
        }
        let ra = c.g[RA][0] as u32;
        let repeats = c.bus.read32(OC336_CTRL + 8);
        fake(c, 0x0011_90f0, ra);
        if c.pc == ra {
            c.g[V0][0] = sx(repeats);
        }
    }

    oc336_fake!(
        announce_fake_show_hud,
        ANNOUNCE_PREV_HUD,
        0x0011_7b88,
        ee::s00::f_00117b88__generated
    );

    oc336_fake!(
        announce_fake_bonus_trick,
        ANNOUNCE_PREV_BONUS_TRICK,
        0x0029_b7e0,
        ee::s08::f_0029b7e0__generated
    );

    oc336_fake!(
        announce_fake_show_name,
        ANNOUNCE_PREV_NAME,
        0x0011_8ff8,
        ee::s00::f_00118ff8__generated
    );

    pub(crate) fn rider_metrix_announce_trick_install() {
        oc336_install!(
            ee::s00::OVERRIDE_0011B1A8 => announce_fake_lookup_bonus => ANNOUNCE_PREV_LOOKUP,
            ee::s00::OVERRIDE_001190F0 => announce_fake_count_repeats => ANNOUNCE_PREV_REPEATS,
            ee::s00::OVERRIDE_00117B88 => announce_fake_show_hud => ANNOUNCE_PREV_HUD,
            ee::s08::OVERRIDE_0029B7E0 => announce_fake_bonus_trick => ANNOUNCE_PREV_BONUS_TRICK,
            ee::s00::OVERRIDE_00118FF8 => announce_fake_show_name => ANNOUNCE_PREV_NAME,
        );
    }

    /// `RiderMetrix_AnnounceTrick`: no trick (the descriptor builder returns
    /// 0), a trick with no bonus points, and one with points (the score, the
    /// HUD number and the bonus-trick sound); the repeat count feeds the
    /// trick-name display. The descriptor builder is real (`self+0x70` makes
    /// it short-circuit), the other callees are logging fakes.
    #[test]
    fn rider_metrix_announce_trick_matches() {
        rider_metrix_announce_trick_install();
        const SELF: u32 = DATA + 0x100;
        const AUDIO: u32 = DATA + 0x1000;
        covered(
            ee::s00::f_0011a168__generated,
            rider_metrix_announce_trick,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0011_a168, 5);
                c.pc = 0x0011_a168;
                c.bus.write32(OC336_CTRL, OC336_MAGIC);
                c.bus.write32(
                    OC336_CTRL + 4,
                    match seed % 3 {
                        0 => 0,
                        1 => 0xffff_ffff,
                        _ => 1 + r.next_u32() % 0x1000,
                    },
                );
                c.bus.write32(OC336_CTRL + 8, (seed % 2) as u32);
                c.bus.write32(GAME_GP.wrapping_add(0x410), AUDIO);
                // The real .lit4 constants the descriptor builder and the
                // score use: degrees -> radians, and 1e-4f.
                c.bus.write32(GAME_GP.wrapping_sub(0x79e4), 0x4265_2ee0);
                c.bus.write32(GAME_GP.wrapping_sub(0x79f8), 0x38d1_b717);
                c.bus.write32(SELF + 0x14, r.edge_f32());
                c.bus.write32(SELF + 0x1ac, AUDIO + 0x100);
                // The descriptor builder short-circuits on +0x70; with 0 it
                // walks the trick bits: no rotation at all (no trick), or a
                // small angle (a spin, so the trick path runs).
                let state = if seed % 4 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 0x10
                };
                c.bus.write32(SELF + RIDER_METRIX_TRICK_STATE, state);
                c.bus.write32(SELF + 0xc, r.next_u32() % 8);
                let no_rotation = state == 0 && seed % 8 == 0;
                c.bus
                    .write32(SELF + 0x34, if no_rotation { 0 } else { r.f32(700.0) });
                c.bus
                    .write32(SELF + 0x38, if no_rotation { 0 } else { r.f32(1400.0) });
                // Fields that would take the descriptor builder's other
                // (state-machine) paths stay clear.
                for off in [
                    0x0u32, 0x4, 0x8, 0x10, 0x20, 0x24, 0x28, 0x2c, 0x60, 0x64, 0x68, 0x7c,
                ] {
                    c.bus.write32(SELF + off, 0);
                }
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = (seed % 2) as u64; // stance
                c.g[A2][0] = ((seed / 2) % 2) as u64; // alternate
                c.g[A3][0] = ((seed / 4) % 2) as u64; // style
                c.g[T0][0] = (seed % 3) as u64; // flag
            },
        );
    }

    /// `RiderMetrix_ReturnZero_a0c0`: the FPU result is 0.0f.
    #[test]
    fn rider_metrix_return_zero_a0c0_matches() {
        check(
            ee::s00::f_0011a0c0__generated,
            rider_metrix_return_zero_a0c0,
            50,
            &|c, seed| {
                let mut r = Rng(0x0011_a0c0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
            },
        );
    }

    /// `RiderMetrix_BuildTrickDesc` over its four shapes: an override trick
    /// (+0x70), a rail/air class (+0x28: spin grades, amount grades, small
    /// spins by kind), a special kind (+0x20: slow, mirrored rider) and the
    /// plain trick (table lookup in and out of range, spin/flip fields,
    /// leftover spins, quality, grabs, extra bits, the "nothing done"
    /// return). Angles are built from whole half/full turns plus up to 100
    /// degrees, so all three rounding cases occur.
    #[test]
    fn rider_metrix_build_trick_desc_matches() {
        const ME: u32 = DATA + 0x100;
        const ID: u32 = DATA + 0x800;
        const RIDER: u32 = DATA + 0x1000;
        check_lohi(
            ee::s00::f_0011a8c8__generated,
            rider_metrix_build_trick_desc,
            800,
            &|c, seed| {
                let mut r = Rng(0x11a8_c800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0011_a8c8;
                c.g[GP][0] = sx(GAME_GP);
                fill(c, &mut r, crate::difftest::SP - 0x80, 0x20);
                fill(c, &mut r, ID, 2);
                fill(c, &mut r, ME, 0x80);
                c.bus
                    .write32(GAME_GP.wrapping_sub(BTD_SPIN_DEG_LIT), 0x4265_2ee0);
                c.bus
                    .write32(GAME_GP.wrapping_sub(BTD_FLIP_DEG_LIT), 0x4265_2ee0);
                // Both trick tables; half the entries 0 (no named trick).
                for i in 0..0x280u32 {
                    let b = if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        r.next_u32() as u8
                    };
                    c.bus.write8(BTD_TABLE + i, b);
                }
                let mode = seed % 8;
                let some = |r: &mut Rng| {
                    if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        r.next_u32() | 1
                    }
                };
                let pick = |r: &mut Rng, xs: &[u32]| xs[(r.next_u32() as usize) % xs.len()];

                let switch = some(&mut r);
                c.bus.write32(ME + BTD_SWITCH, switch);
                let alt = some(&mut r);
                c.bus.write32(ME + BTD_ALT_TABLE, alt);
                let f8 = some(&mut r);
                c.bus.write32(ME + BTD_FLAG_8, f8);
                let fc = some(&mut r);
                c.bus.write32(ME + BTD_FLAG_C, fc);
                let f10 = pick(&mut r, &[0, 1, 2, 3]);
                c.bus.write32(ME + BTD_FLAG_10, f10);
                let kind = match mode {
                    0 => r.next_u32(),
                    1 | 2 => pick(&mut r, &[0, 1, 4, 7]),
                    3 => pick(&mut r, &[1, 2, 3, 4]),
                    _ => 0,
                };
                c.bus.write32(ME + BTD_KIND, kind);
                let speed = r.f32(100.0);
                c.bus.write32(ME + BTD_SPEED, speed);
                let class = if mode == 1 || mode == 2 {
                    pick(&mut r, &[1, 1, 2, 3])
                } else {
                    0
                };
                c.bus.write32(ME + BTD_CLASS, class);
                let amount = ((r.next_u32() % 32) as f32 * 0.5).to_bits();
                c.bus.write32(ME + BTD_AMOUNT, amount);

                // Spin in half turns, flip in whole turns.
                let halves = if mode == 1 || mode == 2 {
                    pick(&mut r, &[0, 1, 1, 1, 2, 3, 5, 6, 8, 11, 12, 14])
                } else if r.next_u32() % 4 == 0 {
                    0
                } else {
                    r.next_u32() % 34
                };
                let turns = if r.next_u32() % 3 == 0 {
                    0
                } else {
                    r.next_u32() % 13
                };
                let jitter = |r: &mut Rng| (r.next_u32() % 201) as f32 - 100.0;
                let sign = |r: &mut Rng| if r.next_u32() % 2 == 0 { 1.0 } else { -1.0f32 };
                let spin_deg = sign(&mut r) * (halves as f32 * 180.0 + jitter(&mut r));
                let flip_deg = sign(&mut r) * (turns as f32 * 360.0 + jitter(&mut r));
                c.bus
                    .write32(ME + BTD_SPIN, (spin_deg / 57.295_78).to_bits());
                c.bus
                    .write32(ME + BTD_FLIP, (flip_deg / 57.295_78).to_bits());

                // Grabs: the first `n` non-zero, then a 0.
                let n = r.next_u32() % 4;
                for k in 0..3u32 {
                    let v = if k < n {
                        r.next_u32() | 1
                    } else if k == n {
                        0
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(ME + BTD_GRABS + 4 * k, v);
                }
                let over = if mode == 0 { r.next_u32() | 1 } else { 0 };
                c.bus.write32(ME + BTD_OVERRIDE, over);
                let extra = some(&mut r);
                c.bus.write32(ME + BTD_EXTRA, extra);
                c.bus.write32(ME + BTD_RIDER, RIDER);
                let swap = some(&mut r);
                c.bus.write32(RIDER + BTD_RIDER_SWAP, swap);

                c.g[A0][0] = sx(ME);
                c.g[A1][0] = sx(ID);
                c.g[A2][0] = some(&mut r) as u64;
                c.g[A3][0] = some(&mut r) as u64;
                c.g[T0][0] = if r.next_u32() % 4 == 0 {
                    r.next_u32() as u64 | 1
                } else {
                    0
                };
                c.g[T1][0] = pick(&mut r, &[0, 0, 0, 1, 2, 3]) as u64;
            },
        );
    }
}
