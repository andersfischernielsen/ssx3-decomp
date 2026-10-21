//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for small leaf helpers — camera tuning tables, path event
//! queries, FE input bindings, reward-gallery item flags, front-end state
//! widget notifications, sound-bank load requests, lens-FX render state and
//! collision primitives — each with a differential test against the
//! generated body.
//!
//! Most of these are leaf accessors (one store or one compare) whose whole
//! meaning is in the field offset; the two interesting ones are
//! [`path_has_event_in_range_reversed`] (signed handling of the range ends,
//! `Path_GetLength` when the upper end is negative) and
//! [`coll_segment_collide_aabb`] (slab test of a segment against an AABB
//! with the two hit records built by VU0 macro ops).
//!
//! The replacements keep their state in the guest registers the originals
//! use where callers can see it, follow the call protocol of [`crate::call`]
//! and the event rules of [`crate::events`]; VU0 code is issued as the
//! original's macro ops.

use crate::abi::{self, r32};
use crate::call::{guest, Callee};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

/// Not listed: `FEUIInput_IsDirection` (0x17fdf0) and
/// `OVScreenRewardList_OnWidgetNotify` (0x1ff748) are switch dispatchers
/// whose generated bodies tail-jump through jump tables into sibling
/// functions (`jump_out` / `call_indirect`), so they are not leaves; porting
/// them needs the whole dispatch (`events::Resume` style), which is out of
/// scope for this wave.
#[rustfmt::skip]
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0017_7ec8, spoke_chase_camera_get_tuning, "SpokeChaseCameraAlgorithm_GetTuning"),
    (0x0017_85b0, superpipe_chase_camera_get_eye_drop, "SuperpipeChaseCameraAlgorithm_GetEyeDrop"),
    (0x0017_85c0, superpipe_chase_camera_get_tuning, "SuperpipeChaseCameraAlgorithm_GetTuning"),
    (0x0035_5858, mod_obj_v41, "ModObj_v41"),
    (0x0035_6ff0, position_modifier_update_transform, "PositionModifier_UpdateTransform"),
    (0x0035_7528, magnet_modifier_update_transform, "MagnetModifier_UpdateTransform"),
    (0x0026_a0b8, path_has_event_in_range_reversed, "Path_HasEventInRangeReversed"),
    (0x0026_ac88, path_has_event_in_range, "Path_HasEventInRange"),
    (0x0018_0478, feui_input_set_port, "FEUIInput_SetPort"),
    (0x0018_0498, feui_input_has_binding, "FEUIInput_HasBinding"),
    (0x001d_1290, reward_gallery_posters_clear_item_flags, "cFEStateRewardGalleryPosters_ClearItemFlags"),
    (0x001d_1428, reward_gallery_trading_cards_clear_item_flags, "cFEStateRewardGalleryTradingCards_ClearItemFlags"),
    (0x001d_15c0, reward_gallery_concept_arts_clear_item_flags, "cFEStateRewardGalleryConceptArts_ClearItemFlags"),
    (0x001d_1758, reward_gallery_videos_clear_item_flags, "cFEStateRewardGalleryVideos_ClearItemFlags"),
    (0x001d_18f0, reward_gallery_toys_clear_item_flags, "cFEStateRewardGalleryToys_ClearItemFlags"),
    (0x001d_1a28, reward_gallery_cheat_chars_clear_item_flags, "cFEStateRewardGalleryCheatChars_ClearItemFlags"),
    (0x001d_1f78, preview_reward_on_widget_notify, "cFEStatePreviewReward_OnWidgetNotify"),
    (0x0014_0bc8, rider_is_computer, "Rider_IsComputer"),
    (0x0023_55b0, game_state_reset_v08, "GameStateReset_v08"),
    (0x0023_6f90, game_state_finish_cinematic_exit, "GameStateFinishCinematic_Exit"),
    (0x0012_78d0, player_pulse_rumble, "Player_PulseRumble"),
    (0x0028_bf38, snd_bank_man_init_bank_request, "SndBankMan_InitBankRequest"),
    (0x0018_8c58, cfe_state_options_handle_input, "cFEStateOptions_HandleInput"),
    (0x0018_9958, cfe_state_options_game_on_widget_notify, "cFEStateOptionsGame_OnWidgetNotify"),
    (0x0018_9970, cfe_state_options_game_v21, "cFEStateOptionsGame_v21"),
    (0x0018_a890, cfe_state_options_sound_on_widget_notify, "cFEStateOptionsSound_OnWidgetNotify"),
    (0x0018_a8c0, cfe_state_options_sound_v21, "cFEStateOptionsSound_v21"),
    (0x0018_c270, cfe_state_options_hud_on_widget_notify, "cFEStateOptionsHUD_OnWidgetNotify"),
    (0x0018_cdb0, cfe_state_options_controller_v21, "cFEStateOptionsController_v21"),
    (0x0018_d438, cfe_state_options_save_load_on_widget_notify, "cFEStateOptionsSaveLoad_OnWidgetNotify"),
    (0x001d_8670, mem_card_state_base_v46, "MemCardStateBase_v46"),
    (0x001d_93d8, mem_card_state_base_transition_out, "MemCardStateBase_TransitionOut"),
    (0x001f_aaf0, ov_screen_be_options_v22, "OVScreenBEOptions_v22"),
    (0x001f_ab08, ov_screen_be_options_v21, "OVScreenBEOptions_v21"),
    (0x001f_bcc8, ov_screen_race_pre_on_deactivate, "OVScreenRacePre_OnDeactivate"),
    (0x001f_db78, ov_screen_rival_pre_on_widget_notify, "OVScreenRivalPre_OnWidgetNotify"),
    (0x002e_3578, lens_fx_base_push_render_state, "LensFXBase_PushRenderState"),
    (0x002e_3668, lens_fx_base_pop_render_state, "LensFXBase_PopRenderState"),
    (0x002e_4c90, c_transition_man_script_is_running, "cTransitionMan_ScriptIsRunning"),
    (0x0014_7fd8, be_char_int_commit, "BECharInt_Commit"),
    (0x0034_0a18, coll_sphere_get_diameter, "CollSphere_GetDiameter"),
    (0x0034_1a30, boost_handle_event, "Boost_HandleEvent"),
    (0x0034_24d0, anim_teeter_v53, "AnimTeeter_v53"),
    (0x0034_2b80, boost_modifier_handle_event, "BoostModifier_HandleEvent"),
    (0x0023_9ce0, gm_handler0_v09, "GMHandler0_v09"),
    (0x0023_a070, gm_handler9_v06, "GMHandler9_v06"),
    (0x0032_e4b8, coll_segment_reset_transform, "CollSegment_ResetTransform"),
    (0x0032_e690, coll_segment_collide_aabb, "CollSegment_CollideAABB"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const PATH_GET_LENGTH: Callee = Callee {
        addr: 0x0026_ac48,
        f: ee::s07::Path_GetLength,
    };
    pub const PATH_HAS_EVENT_IN_RANGE: Callee = Callee {
        addr: 0x0026_ac88,
        f: ee::s07::Path_HasEventInRange,
    };
    pub const SND_BANK_MAN_INIT_LOAD_REQUEST: Callee = Callee {
        addr: 0x0028_b930,
        f: ee::s07::SndBankMan_InitLoadRequest,
    };
}

// ---------------------------------------------------------------------------
// Small helpers.

/// A `gp`-relative global address.
#[inline(always)]
fn gpa(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// Chase-camera algorithms (camera/trigger/cameratriggerfactory.cpp).

/// `void SpokeChaseCameraAlgorithm_GetTuning(SpokeChaseCameraAlgorithm*,
/// CameraTuning* out)` (0x177ec8): copy the four tuning constants of the
/// spoke chase camera (`gp-0x5e1c..-0x5e10`) into `out` (x, y, z, w at
/// `+0x0/+0x4/+0x8/+0xc`), set `out->kind = 1` at `+0x10`, return 1 in `v0`.
pub fn spoke_chase_camera_get_tuning(c: &mut Cpu) {
    c.cycles += 11;
    let gp = r32(c, GP);
    c.lwc1(3, gp.wrapping_sub(0x5e1c));
    c.g[V0][0] = 0x1;
    c.lwc1(2, gp.wrapping_sub(0x5e18));
    c.lwc1(0, gp.wrapping_sub(0x5e14));
    c.lwc1(1, gp.wrapping_sub(0x5e10));
    let out = r32(c, A1);
    c.sw(out.wrapping_add(0x10), c.g[V0][0]);
    c.swc1(3, out.wrapping_add(0x4));
    c.swc1(2, out);
    c.swc1(0, out.wrapping_add(0xc));
    c.swc1(1, out.wrapping_add(0x8));
    abi::ret(c);
}

/// `float SuperpipeChaseCameraAlgorithm_GetEyeDrop(void*)` (0x1785b0):
/// the superpipe camera's eye-drop offset (`gp-0x5dc0`) in `f0`.
pub fn superpipe_chase_camera_get_eye_drop(c: &mut Cpu) {
    c.cycles += 3;
    let gp = r32(c, GP);
    c.lwc1(0, gp.wrapping_sub(0x5dc0));
    abi::ret(c);
}

/// `void SuperpipeChaseCameraAlgorithm_GetTuning(SuperpipeChaseCameraAlgorithm*,
/// CameraTuning* out)` (0x1785c0): the superpipe counterpart of
/// [`spoke_chase_camera_get_tuning`], reading `gp-0x5dbc..-0x5db0`.
pub fn superpipe_chase_camera_get_tuning(c: &mut Cpu) {
    c.cycles += 11;
    let gp = r32(c, GP);
    c.lwc1(3, gp.wrapping_sub(0x5dbc));
    c.g[V0][0] = 0x1;
    c.lwc1(2, gp.wrapping_sub(0x5db8));
    c.lwc1(0, gp.wrapping_sub(0x5db4));
    c.lwc1(1, gp.wrapping_sub(0x5db0));
    let out = r32(c, A1);
    c.sw(out.wrapping_add(0x10), c.g[V0][0]);
    c.swc1(3, out.wrapping_add(0x4));
    c.swc1(2, out);
    c.swc1(0, out.wrapping_add(0xc));
    c.swc1(1, out.wrapping_add(0x8));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Move-node modifiers (object/movenode.cpp).

/// `void ModObj_v41(Modifier* self, float secs)` (0x355858): `self->+0x20 =
/// (int)((float)gp->0x2a74->0x10 * secs)` — an integer field scaled by the
/// frame time and truncated.
pub fn mod_obj_v41(c: &mut Cpu) {
    c.cycles += 8;
    let gp = r32(c, GP);
    let cfg = Addr(gp.wrapping_add(0x2a74)).lw(c);
    c.g[V0][0] = cfg;
    let cfg = cfg as u32;
    c.lwc1(0, cfg.wrapping_add(0x10));
    c.cvt_s_w(0, 0);
    c.mul_s(0, 0, 12);
    c.cvt_w_s(1, 0);
    let this = r32(c, A0);
    c.swc1(1, this.wrapping_add(0x20));
    abi::ret(c);
}

/// `void PositionModifier_UpdateTransform(PositionModifier* self)`
/// (0x356ff0): publish the node's offset (`+0x30`, a quadword) at `+0x80`
/// and clear the pending-flag at `+0x44`.
pub fn position_modifier_update_transform(c: &mut Cpu) {
    c.cycles += 4;
    let this = r32(c, A0);
    c.lq(2, this.wrapping_add(0x30));
    c.sw(this.wrapping_add(0x44), 0);
    c.sq(2, this.wrapping_add(0x80));
    abi::ret(c);
}

/// `void MagnetModifier_UpdateTransform(MagnetModifier* self)` (0x357528):
/// publish the magnet target (`+0x10`) at `+0x90` and clear the pending
/// flag at `+0x50`.
pub fn magnet_modifier_update_transform(c: &mut Cpu) {
    c.cycles += 4;
    let this = r32(c, A0);
    c.lq(2, this.wrapping_add(0x10));
    c.sw(this.wrapping_add(0x50), 0);
    c.sq(2, this.wrapping_add(0x90));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Path events.

/// `int Path_HasEventInRangeReversed(Path* path, float a, float b)` (0x26a0b8):
/// the reversed range query — `lo = a < 0 ? 0 : path->range_end - a`,
/// `hi = b < 0 ? Path_GetLength(path) : path->range_end - b`, then
/// [`path_has_event_in_range`] on `[lo, hi]`.
pub fn path_has_event_in_range_reversed(c: &mut Cpu) {
    guest(c, |c| {
        const RANGE_END: u32 = 0x38;
        c.cycles += 8;
        c.mtc1(0, 0); // f0 = 0.0
        let sp = r32(c, SP).wrapping_sub(0x30);
        c.g[SP][0] = sx(sp);
        c.sq(16, sp.wrapping_add(0x10));
        c.c_le_s(0, 12); // 0.0 <= a
        Addr(sp).sd(c, c.g[RA][0]);
        c.swc1(20, sp.wrapping_add(0x20));
        let upper = !c.fcc();
        c.g[S0][0] = c.g[A0][0];
        if upper {
            // a < 0: the low end clamps to zero.
            c.cycles += 1;
            c.mov_s(20, 0);
        } else {
            c.cycles += 3;
            c.lwc1(0, r32(c, S0).wrapping_add(RANGE_END));
            c.sub_s(20, 0, 12); // f20 = range_end - a
        }
        // Higher end.
        c.cycles += 5;
        c.mtc1(0, 0); // f0 = 0.0
        c.c_lt_s(13, 0);
        if !c.fcc() {
            // b >= 0: the high end is range_end - b.
            c.lwc1(0, r32(c, S0).wrapping_add(RANGE_END));
            c.cycles += 2;
            c.sub_s(0, 0, 13);
            c.g[A0][0] = c.g[S0][0];
        } else {
            // b < 0: the high end is the path length.
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::PATH_GET_LENGTH.call(c, 0x0026_a104)?;
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 8;
        c.mov_s(12, 20); // low
        c.mov_s(13, 0); // high
        callee::PATH_HAS_EVENT_IN_RANGE.call(c, 0x0026_a120)?;
        c.lq(16, sp.wrapping_add(0x10));
        c.g[RA][0] = Addr(sp).ld(c);
        c.lwc1(20, sp.wrapping_add(0x20));
        c.g[SP][0] = sx(sp.wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `int Path_HasEventInRange(Path* path, float lo, float hi)` (0x26ac88):
/// 1 if any segment of the path overlaps `[lo, hi]` and is a scripted event
/// (type 0x12 start / 0x14 end), else 0. `path->count` at `+0x0`, the item
/// array at `+0x4` (0x10 bytes each: type at `+0x0`, start at `+0x8`, end at
/// `+0xc`).
pub fn path_has_event_in_range(c: &mut Cpu) {
    const EVENT_END: u32 = 0x14;
    const EVENT_START: u32 = 0x12;
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, A0)).lw(c);
    c.g[V1][0] = 0;
    if (c.g[V0][0] as i64) <= 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 5;
    c.g[A0][0] = Addr(r32(c, A0).wrapping_add(0x4)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    c.g[A3][0] = EVENT_END as u64;
    c.g[A2][0] = EVENT_START as u64;
    loop {
        let item = r32(c, A0);
        'counted: {
            c.cycles += 5;
            c.lwc1(0, item.wrapping_add(0x8));
            c.c_lt_s(13, 0); // hi < start
            if c.fcc() {
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                break 'counted;
            }
            c.cycles += 5;
            c.lwc1(0, item.wrapping_add(0xc));
            c.c_lt_s(0, 12); // end < lo
            if c.fcc() {
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                break 'counted;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(item).lw(c);
            let event = c.g[V0][0] == c.g[A3][0];
            if !event {
                c.cycles += 2;
                let not_start = c.g[V0][0] != c.g[A2][0];
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                if not_start {
                    break 'counted;
                }
            }
            // A start or end marker inside the range.
            c.cycles += 2;
            c.g[V0][0] = 0x1;
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0026_aca8);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Front-end input (fe/feuiinput.cpp).

/// `void FEUIInput_SetPort(FEUIInput* self, int port)` (0x180478): record the
/// controller port at `self+0x4` and bind the per-port binding object
/// (`world->[port]` at `+0xb0`) at `self+0x18`.
pub fn feui_input_set_port(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = Addr(gpa(c, -0x848)).lw(c);
    c.g[A1][0] &= 0xff;
    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xb0)).lw(c);
    let this = r32(c, A0);
    Addr(this.wrapping_add(0x4)).sb(c, c.g[A1][0]);
    Addr(this.wrapping_add(0x18)).sw(c, c.g[A2][0]);
    abi::ret(c);
}

/// `int FEUIInput_HasBinding(FEUIInput* self)` (0x180498): whether the bound
/// object at `+0x18` (two pointer hops) holds a non-zero action count.
pub fn feui_input_has_binding(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = Addr(r32(c, A0).wrapping_add(0x18)).lw(c);
    if c.g[V1][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.g[V1][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.cycles += 3;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Reward gallery (fe/festaterewardgallery.cpp).

/// Shared body of the six `ClearItemFlags` resets: clear the state word at
/// `+0x68` and the "flags dirty" word at `+0xa24`, then zero `count + 1`
/// item-flag words ending at `+base` (the array is walked downwards).
fn clear_item_flags(c: &mut Cpu, base: u32, count: u32, head: u32) {
    const STATE: u32 = 0x68;
    const FLAGS_DIRTY: u32 = 0xa24;
    c.cycles += 4;
    let this = r32(c, A0);
    c.sw(this.wrapping_add(STATE), 0);
    c.g[V1][0] = count as u64;
    c.sw(this.wrapping_add(FLAGS_DIRTY), 0);
    c.g[V0][0] = sx(this.wrapping_add(base));
    loop {
        c.cycles += 7;
        c.sw(r32(c, V0), 0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void cFEStateRewardGalleryPosters_ClearItemFlags(cFEState* self)`
/// (0x1d1290): 43 poster flags at `+0xad0..+0xa28`.
pub fn reward_gallery_posters_clear_item_flags(c: &mut Cpu) {
    clear_item_flags(c, 0xad0, 0x2a, 0x001d_12a0);
}

/// `void cFEStateRewardGalleryTradingCards_ClearItemFlags(cFEState* self)`
/// (0x1d1428): 116 trading-card slot ids at `+0xbf4..+0xa28`.
pub fn reward_gallery_trading_cards_clear_item_flags(c: &mut Cpu) {
    clear_item_flags(c, 0xbf4, 0x73, 0x001d_1438);
}

/// `void cFEStateRewardGalleryConceptArts_ClearItemFlags(cFEState* self)`
/// (0x1d15c0): 100 concept-art ids at `+0xbb4..+0xa28`.
pub fn reward_gallery_concept_arts_clear_item_flags(c: &mut Cpu) {
    clear_item_flags(c, 0xbb4, 0x63, 0x001d_15d0);
}

/// `void cFEStateRewardGalleryVideos_ClearItemFlags(cFEState* self)`
/// (0x1d1758): the two video ids at `+0xa2c/+0xa28`.
pub fn reward_gallery_videos_clear_item_flags(c: &mut Cpu) {
    clear_item_flags(c, 0xa2c, 0x1, 0x001d_1768);
}

/// `void cFEStateRewardGalleryToys_ClearItemFlags(cFEState* self)`
/// (0x1d18f0): 28 toy ids at `+0xa94..+0xa28`.
pub fn reward_gallery_toys_clear_item_flags(c: &mut Cpu) {
    clear_item_flags(c, 0xa94, 0x1b, 0x001d_1900);
}

/// `void cFEStateRewardGalleryCheatChars_ClearItemFlags(cFEState* self)`
/// (0x1d1a28): 28 cheat-character ids at `+0xa94..+0xa28`.
pub fn reward_gallery_cheat_chars_clear_item_flags(c: &mut Cpu) {
    clear_item_flags(c, 0xa94, 0x1b, 0x001d_1a38);
}

/// `int cFEStatePreviewReward_OnWidgetNotify(cFEState*, int, int key)`
/// (0x1d1f78): the reward preview answers the back/cancel keys (6, 8, 9)
/// with 0x100 (handled) and everything else with 0x101 (ignored), matching
/// the generated code's nested range tests.
pub fn preview_reward_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x6;
    let is6 = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = (c.g[A2][0] < 0x6u64) as u64;
    if !is6 {
        c.cycles += 2;
        let below6 = c.g[V0][0] != 0;
        c.g[V0][0] = (c.g[A2][0] < 0xau64) as u64;
        if !below6 {
            c.cycles += 2;
            let above10 = c.g[V0][0] == 0;
            c.g[V0][0] = (c.g[A2][0] < 0x8u64) as u64;
            if !above10 {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    // 8 or 9: handled.
                    c.cycles += 2;
                    c.g[V0][0] = 0x100;
                    abi::ret(c);
                    return;
                }
            }
        }
    } else {
        // key 6: handled.
        c.cycles += 2;
        c.g[V0][0] = 0x100;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0x101;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Rider and game-state predicates.

/// `int Rider_IsComputer(Rider* rider)` (0x140bc8): `rider->+0x874 ^ 1`
/// (1 for the AI flag).
pub fn rider_is_computer(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x874)).lw(c);
    c.g[V0][0] ^= 0x1;
    abi::ret(c);
}

/// `int GameStateReset_v08(GameState* state)` (0x2355b0): `state->+0x10 == 5`.
pub fn game_state_reset_v08(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x10)).lw(c);
    c.g[V0][0] ^= 0x5;
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    abi::ret(c);
}

/// `void GameStateFinishCinematic_Exit(void)` (0x236f90): clear the cinematic
/// state word (`race->+0x8c`) of the current race.
pub fn game_state_finish_cinematic_exit(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = Addr(gpa(c, -0x848)).lw(c);
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8c), 0);
    abi::ret(c);
}

/// `void Player_PulseRumble(Player* player, float amount)` (0x1278d0):
/// `player->rumble = max(amount, player->rumble)` (leave-set, no decay).
pub fn player_pulse_rumble(c: &mut Cpu) {
    c.cycles += 4;
    let this = r32(c, A0);
    c.lwc1(0, this.wrapping_add(0xdfc));
    c.max_s(12, 12, 0);
    c.swc1(12, this.wrapping_add(0xdfc));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Sound banks.

/// `void SndBankMan_InitBankRequest(SndBankMan* man, int, int, int, int,
/// int, int)` (0x28bf38): forward a bank-load request to
/// `SndBankMan_InitLoadRequest(man->banks + arg1*0x60, ...)` with the
/// arguments shifted up by one (the byte offset is built by the `mult`).
pub fn snd_bank_man_init_bank_request(c: &mut Cpu) {
    const RET: u32 = 0x0028_bf68;
    guest(c, |c| {
        c.cycles += 15;
        c.g[V0][0] = 0x60;
        let sp = r32(c, SP).wrapping_sub(0x10);
        c.g[SP][0] = sx(sp);
        c.mult(0, 3, 5, 2);
        Addr(sp).sd(c, c.g[RA][0]);
        c.g[A1][0] = c.g[A2][0];
        c.g[A2][0] = c.g[A3][0];
        c.g[A0][0] = Addr(r32(c, A0).wrapping_add(0xacc)).lw(c);
        c.g[A3][0] = c.g[T0][0];
        c.g[T0][0] = c.g[T1][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[T1][0] = c.g[T2][0];
        callee::SND_BANK_MAN_INIT_LOAD_REQUEST.call(c, RET)?;
        c.g[RA][0] = Addr(sp).ld(c);
        c.g[SP][0] = sx(sp.wrapping_add(0x10));
        abi::ret(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end option screens (fe/feoptions.cpp).

/// `int cFEStateOptions_HandleInput(cFEStateOptions* self, FEUIInput* input)`
/// (0x188c58): latch the raw pad byte at `input+0x4` into the global input
/// word (`gp+0x394`) and return 0.
pub fn cfe_state_options_handle_input(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = Addr(r32(c, A1).wrapping_add(0x4)).lbu(c);
    c.g[V0][0] = 0;
    c.sw(gpa(c, 0x394), c.g[V1][0]);
    abi::ret(c);
}

/// `int cFEStateOptionsGame_OnWidgetNotify(void*, int, int id)`
/// (0x189958): the game-options page handles the "back" widget (9) with
/// 0x100 and everything else with 0x101.
pub fn cfe_state_options_game_on_widget_notify(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] ^= 0x9;
    c.g[V1][0] = 0x100;
    c.g[V0][0] = 0x101;
    if c.g[A2][0] == 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    abi::ret(c);
}

/// `int cFEStateOptionsGame_v21(void*, int, int id)` (0x189970): 0x100 when
/// an id was passed, 0 otherwise.
pub fn cfe_state_options_game_v21(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x100;
    if c.g[A2][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `int cFEStateOptionsSound_OnWidgetNotify(void*, int, int id)`
/// (0x18a890): id 6 is ignored (0), id 9 is handled (0x100), anything else
/// is not for this page (0x101).
pub fn cfe_state_options_sound_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x6;
    let is6 = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = 0x9;
    if is6 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let not_back = c.g[A2][0] != c.g[V0][0];
    c.g[V0][0] = 0x100;
    if not_back {
        c.cycles += 2;
        c.g[V0][0] = 0x101;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int cFEStateOptionsSound_v21(void*, int, int id)` (0x18a8c0): 0x100 when
/// an id was passed, 0 otherwise.
pub fn cfe_state_options_sound_v21(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x100;
    if c.g[A2][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `int cFEStateOptionsHUD_OnWidgetNotify(void*, int, int id)` (0x18c270):
/// as [`cfe_state_options_sound_on_widget_notify`] for the HUD page.
pub fn cfe_state_options_hud_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x6;
    let is6 = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = 0x9;
    if is6 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let not_back = c.g[A2][0] != c.g[V0][0];
    c.g[V0][0] = 0x100;
    if not_back {
        c.cycles += 2;
        c.g[V0][0] = 0x101;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int cFEStateOptionsController_v21(void*, int, int id)` (0x18cdb0):
/// 0x100 when an id was passed, 0 otherwise.
pub fn cfe_state_options_controller_v21(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x100;
    if c.g[A2][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `int cFEStateOptionsSaveLoad_OnWidgetNotify(void*, int, int key)`
/// (0x18d438): keys 8 and 9 are handled (0x100), everything else ignored
/// (0x101); the two comparisons are done as unsigned `< 10` / `< 8`.
pub fn cfe_state_options_save_load_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = (c.g[A2][0] < 0xau64) as u64;
    let out_of_range = c.g[V1][0] == 0;
    c.g[V0][0] = 0x101;
    if out_of_range {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.g[A0][0] = (c.g[A2][0] < 0x8u64) as u64;
    c.g[V1][0] = 0x101;
    c.g[V0][0] = 0x100;
    if c.g[A0][0] != 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Memory-card and front-end overlay states.

/// `void MemCardStateBase_v46(void*)` (0x1d8670): restart the front-end's
/// state machine if it is in state 4, then clear the two progress words of
/// the memory-card screen (the FE singleton at `gp-0x10c8`).
pub fn mem_card_state_base_v46(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A0][0] = Addr(gpa(c, -0x10c8)).lw(c);
    c.g[V1][0] = 0x4;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c0));
    let not4 = c.g[V0][0] != c.g[V1][0];
    c.g[V0][0] = Addr(gpa(c, -0x10c8)).lw(c);
    if !not4 {
        c.cycles += 3;
        c.g[V0][0] = 0x1;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
        c.g[V0][0] = Addr(gpa(c, -0x10c8)).lw(c);
    }
    c.cycles += 3;
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1e0), 0);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1a8), 0);
    abi::ret(c);
}

/// `int MemCardStateBase_TransitionOut(MemCardStateBase* state)` (0x1d93d8):
/// `state->+0x1c0 == 6`.
pub fn mem_card_state_base_transition_out(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x1c0)).lw(c);
    c.g[V0][0] ^= 0x6;
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    abi::ret(c);
}

/// `int OVScreenBEOptions_v22(void*, int, int id)` (0x1faaf0): id 4 (the
/// "back" widget of the back-end options overlay) is handled (0x100).
pub fn ov_screen_be_options_v22(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] ^= 0x4;
    c.g[V1][0] = 0x100;
    c.g[V0][0] = 0x101;
    if c.g[A2][0] == 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    abi::ret(c);
}

/// `int OVScreenBEOptions_v21(void*, int, int id)` (0x1fab08): 0x100 for the
/// "no id" case, as the other option screens.
pub fn ov_screen_be_options_v21(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0x100;
    c.g[V0][0] = 0x101;
    if c.g[A2][0] == 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    abi::ret(c);
}

/// `void OVScreenRacePre_OnDeactivate(void)` (0x1fbcc8): reset the pending
/// count of the overlay (`gp-0x204 -> +0x64`) to -1, when it exists.
pub fn ov_screen_race_pre_on_deactivate(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = Addr(gpa(c, -0x204)).lw(c);
    let absent = c.g[V1][0] == 0;
    c.g[V0][0] = u64::MAX;
    if absent {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.sw((c.g[V1][0] as u32).wrapping_add(0x64), c.g[V0][0]);
    c.cycles += 2;
    abi::ret(c);
}

/// `int OVScreenRivalPre_OnWidgetNotify(void*, int, int id)` (0x1fdb78):
/// id 6 handled (0x101), everything else 0.
pub fn ov_screen_rival_pre_on_widget_notify(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] ^= 0x6;
    c.g[V0][0] = 0x101;
    if c.g[A2][0] != 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Lens FX (graphics/lensfx.cpp).

/// `void LensFXBase_PushRenderState(LensFXBase* self)` (0x2e3578): push a
/// 0x14-byte copy of the render state (`+0x0..+0x13` of the block at
/// `gp+0x2a90 -> +0xe84`) onto the same stack, copy the default block from
/// `0x500000+0x1420`, then set the render-control bits (GS blend/alpha/test
/// fields at `+0x4`/`+0x8`) as the fixed point pipeline expects.
pub fn lens_fx_base_push_render_state(c: &mut Cpu) {
    c.cycles += 60;
    c.g[A0][0] = Addr(gpa(c, 0x2a90)).lw(c);
    c.g[T0][0] = 0x500000;
    c.g[A2][0] = 0xffff_ffff_ffbf_0000;
    c.g[T1][0] = 0x400000;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
    c.g[A2][0] |= 0xffff;
    c.g[T2][0] = 0xffff_ffff_ffff_fffc;
    c.g[A3][0] = 0xffff_ffff_e000_0000;
    c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
    c.ldr(3, c.g[V0][0] as u32);
    c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0xf));
    c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x8));
    c.g[T3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
    c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x1b));
    c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x14));
    c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x23));
    c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x1c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x24), c.g[T3][0]);
    c.g[A3][0] |= 0x3ff;
    c.g[T3][0] = 0xffff_ffff_ffff_ff83;
    c.g[T4][0] = 0xffff_ffff_ffff_fc1f;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
    c.sw((c.g[A0][0] as u32).wrapping_add(0xe84), c.g[V0][0]);
    c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1420));
    c.ldl(13, (c.g[A1][0] as u32).wrapping_add(0x7));
    c.ldr(13, c.g[A1][0] as u32);
    c.ldl(14, (c.g[A1][0] as u32).wrapping_add(0xf));
    c.ldr(14, (c.g[A1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
    c.sdl(13, (c.g[V1][0] as u32).wrapping_add(0x1b));
    c.sdr(13, (c.g[V1][0] as u32).wrapping_add(0x14));
    c.sdl(14, (c.g[V1][0] as u32).wrapping_add(0x23));
    c.sdr(14, (c.g[V1][0] as u32).wrapping_add(0x1c));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x24), c.g[V0][0]);
    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] &= c.g[A2][0];
    c.g[V0][0] |= c.g[T1][0];
    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
    c.g[V0][0] &= c.g[T2][0];
    c.g[V0][0] |= 0x2;
    c.sw((c.g[A2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] &= c.g[A3][0];
    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] &= c.g[T3][0];
    c.g[V0][0] |= 0x1c;
    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] &= c.g[T4][0];
    c.g[V0][0] |= 0xe0;
    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    abi::ret(c);
}

/// `void LensFXBase_PopRenderState(LensFXBase* self)` (0x2e3668): pop the
/// saved render-state block (`gp-0x854 -> +0xe84`) by 0x14 bytes.
pub fn lens_fx_base_pop_render_state(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = Addr(gpa(c, -0x854)).lw(c);
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
    c.sw((c.g[V1][0] as u32).wrapping_add(0xe84), c.g[V0][0]);
    abi::ret(c);
}

/// `int cTransitionMan_ScriptIsRunning(cTransitionMan* self)` (0x2e4c90):
/// a script is running when the current script pointer (`+0x60`) is set and
/// its step counter (`+0x64`) is non-zero.
pub fn c_transition_man_script_is_running(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x60)).lw(c);
    c.g[V1][0] = 0;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x64)).lw(c);
        c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[V1][0];
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Beast character controller.

/// `void BECharInt_Commit(void)` (0x147fd8): publish the ten control blocks
/// of the three beast characters from the source table at `0x535538`
/// (three rows of 0x7 bytes + 3 trailing bytes) to `0x4a7887`, `0x4b13d7`
/// and `0x4baf27` (stride 0x7 per iteration, three rows +0x0/+0x46/+0x8c).
pub fn be_char_int_commit(c: &mut Cpu) {
    c.cycles += 10;
    c.g[A0][0] = 0x4a0000;
    c.g[A1][0] = 0x530000;
    c.g[V1][0] = 0x4b0000;
    c.g[V0][0] = 0x4c0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x50d9));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x13d7));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7887));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5538));
    c.g[A2][0] = 0x9;
    loop {
        c.cycles += 36;
        let (a0, a1, v0, v1) = (r32(c, A0), r32(c, A1), r32(c, V0), r32(c, V1));
        c.lwl(7, a1.wrapping_add(0x3));
        c.lwr(7, a1);
        c.g[T0][0] = c.lb(a1.wrapping_add(0x4));
        c.g[T1][0] = c.lb(a1.wrapping_add(0x5));
        c.g[T2][0] = c.lb(a1.wrapping_add(0x6));
        c.swl(7, a0.wrapping_add(0x3));
        c.swr(7, a0);
        c.sb(a0.wrapping_add(0x4), c.g[T0][0]);
        c.sb(a0.wrapping_add(0x5), c.g[T1][0]);
        c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
        c.sb(a0.wrapping_add(0x6), c.g[T2][0]);
        c.lwl(7, a1.wrapping_add(0x49));
        c.lwr(7, a1.wrapping_add(0x46));
        c.g[T0][0] = c.lb(a1.wrapping_add(0x4a));
        c.g[T1][0] = c.lb(a1.wrapping_add(0x4b));
        c.g[T2][0] = c.lb(a1.wrapping_add(0x4c));
        c.swl(7, v1.wrapping_add(0x3));
        c.swr(7, v1);
        c.sb(v1.wrapping_add(0x4), c.g[T0][0]);
        c.sb(v1.wrapping_add(0x5), c.g[T1][0]);
        c.sb(v1.wrapping_add(0x6), c.g[T2][0]);
        c.g[A0][0] = sx(a0.wrapping_add(0xf88));
        c.lwl(7, a1.wrapping_add(0x8f));
        c.lwr(7, a1.wrapping_add(0x8c));
        c.g[T0][0] = c.lb(a1.wrapping_add(0x90));
        c.g[T1][0] = c.lb(a1.wrapping_add(0x91));
        c.g[T2][0] = c.lb(a1.wrapping_add(0x92));
        c.swl(7, v0.wrapping_add(0x3));
        c.swr(7, v0);
        c.sb(v0.wrapping_add(0x4), c.g[T0][0]);
        c.sb(v0.wrapping_add(0x5), c.g[T1][0]);
        c.sb(v0.wrapping_add(0x6), c.g[T2][0]);
        c.g[V1][0] = sx(v1.wrapping_add(0xf88));
        c.g[V0][0] = sx(v0.wrapping_add(0xf88));
        let more = (c.g[A2][0] as i64) >= 0;
        c.g[A1][0] = sx(a1.wrapping_add(0x7));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0014_8000);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Collision primitives (intersect/riderspheretree.cpp).

/// `float CollSphere_GetDiameter(CollSphere* sphere)` (0x340a18): twice the
/// radius at `+0x70`, or -1.0 in `f0` when the sphere is empty.
pub fn coll_sphere_get_diameter(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, A0)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 4;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 0);
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.lwc1(0, r32(c, A0).wrapping_add(0x70));
    c.add_s(0, 0, 0);
    abi::ret(c);
}

/// `void Boost_HandleEvent(Boost* self, int event)` (0x341a30): on event 1
/// (start) remember the current boost level as the event's baseline.
pub fn boost_handle_event(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x1;
    if c.g[A1][0] == c.g[V0][0] {
        c.cycles += 2;
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x38)).lw(c);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `float AnimTeeter_v53(AnimTeeter* self)` (0x3424d0): clamp the lean value
/// at `+0x4` to the [lower, upper] limits (`+0x10`, `+0x14`) only while the
/// teeter is heading outwards (`+0x30` is the angular velocity), returning 0
/// when it is already pinned.
pub fn anim_teeter_v53(c: &mut Cpu) {
    c.cycles += 6;
    let this = r32(c, A0);
    c.lwc1(1, this.wrapping_add(0x30)); // velocity
    c.lwc1(0, this.wrapping_add(0x10)); // upper limit
    c.c_le_s(1, 0); // velocity <= upper
    if !c.fcc() {
        // Below the upper limit: test the lower one.
        c.lwc1(0, this.wrapping_add(0x14));
    } else {
        c.cycles += 6;
        c.lwc1(0, this.wrapping_add(0x4)); // current lean
        c.mtc1(0, 1); // f1 = 0.0
        c.c_lt_s(0, 1); // lean < 0
        if !c.fcc() {
            c.lwc1(1, this.wrapping_add(0x30));
            c.cycles += 1;
            c.lwc1(0, this.wrapping_add(0x14));
        } else {
            // Heading back inwards: no clamp.
            c.cycles += 2;
            c.mov_s(0, 1);
            abi::ret(c);
            return;
        }
    }
    // f0 = lower limit, f1 = velocity (or the upper limit).
    c.cycles += 4;
    c.c_le_s(0, 1); // lower <= velocity
    let outwards = !c.fcc();
    c.lwc1(2, this.wrapping_add(0x4));
    if !outwards {
        c.cycles += 4;
        c.mtc1(0, 0); // f0 = 0.0
        c.c_lt_s(0, 2); // 0 < lean
        if c.fcc() {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 1;
    c.mov_s(0, 2);
    c.cycles += 2;
    abi::ret(c);
}

/// `void BoostModifier_HandleEvent(BoostModifier* self, int event)`
/// (0x342b80): on the boost-start event (0x258) remember the boost level as
/// the modifier's baseline.
pub fn boost_modifier_handle_event(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x258;
    if c.g[A1][0] == c.g[V0][0] {
        c.cycles += 2;
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x28)).lw(c);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int GMHandler0_v09(Handler* self, int col, int row)` (0x239ce0): a
/// 0x28-byte-row table lookup `self->rows[row][col]` (word at
/// `+row*0x28 + col*4 + 4`), 0 when `row` is past the count (`gp-0x480`).
pub fn gm_handler0_v09(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = Addr(gpa(c, -0x480)).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
    let out_of_range = c.g[V0][0] != 0;
    c.g[V0][0] = 0x28;
    if out_of_range {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
    c.mult(0, 2, 6, 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
    abi::ret(c);
}

/// `int GMHandler9_v06(Handler9* self, int player, int other)` (0x23a070):
/// bump `self->score[player]` and report whether it is now strictly ahead of
/// the other player; if so and the race is running, accumulate the frame
/// time into `manager->+0x78` (capped at the race's frame limit).
pub fn gm_handler9_v06(c: &mut Cpu) {
    const ONE: u64 = 0x1;
    c.cycles += 17;
    let self_ = r32(c, A0);
    let player = r32(c, A1);
    c.g[A0][0] = sx(self_.wrapping_add(0x4)); // score array
    c.g[A3][0] = sx(player << 1);
    c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.g[T1][0] = ONE;
    c.g[V1][0] = c.lhu(c.g[A3][0] as u32);
    c.g[V0][0] = c.g[T1][0];
    if c.g[A1][0] != 0 {
        c.g[V0][0] = 0;
    }
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
    c.sh(c.g[A3][0] as u32, c.g[V1][0]);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
    c.g[V0][0] = c.lh(c.g[A0][0] as u32);
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    let ahead = c.g[V0][0] != 0;
    c.g[T0][0] = c.g[A2][0];
    if !ahead {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.g[A1][0] = Addr(gpa(c, -0x480)).lw(c);
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
    let not_running = c.g[V0][0] != c.g[T1][0];
    c.g[V0][0] = Addr(gpa(c, -0x848)).lw(c);
    if not_running {
        c.cycles += 2;
        c.g[V0][0] = 0x1;
        abi::ret(c);
        return;
    }
    c.cycles += 7;
    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x78)); // accumulated
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84)); // race manager
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc)); // race
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8)); // frame limit
    c.g[V0][0] = (c.g[A2][0] < c.g[V0][0]) as u64;
    let limited = c.g[V0][0] != 0;
    c.g[V1][0] = 0x3c;
    if limited {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 5;
    c.g[V0][0] = 0x1;
    c.mult(0, 4, 8, 3);
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
    c.sw((c.g[A1][0] as u32).wrapping_add(0x78), c.g[V1][0]);
    abi::ret(c);
}

/// `void CollSegment_ResetTransform(CollSegment* self)` (0x32e4b8): copy the
/// transformed endpoints (`+0x80`, `+0x90`) back over the local frame
/// (`+0x60`, `+0x70`) and clear the flags at `+0x4`.
pub fn coll_segment_reset_transform(c: &mut Cpu) {
    c.cycles += 6;
    let this = r32(c, A0);
    c.lq(2, this.wrapping_add(0x80));
    c.lq(3, this.wrapping_add(0x90));
    c.sq(2, this.wrapping_add(0x60));
    c.sq(3, this.wrapping_add(0x70));
    c.sw(this.wrapping_add(0x4), 0);
    abi::ret(c);
}

/// `int CollSegment_CollideAABB(CollSegment* self, float* bmin, float* bmax,
/// CollHit* hits)` (0x32e690): slab test of the segment (`origin` at `+0x60`,
/// `dir` at `+0x70`) against the box `[bmin, bmax]`, writing up to two hit
/// records (0x80 bytes each: normal quad at `+0x10`, position at `+0x0`,
/// distance at `+0x40`, flags/indices around `+0x44..+0x70`).
pub fn coll_segment_collide_aabb(c: &mut Cpu) {
    // Prologue: constants and loop state.
    c.cycles += 16;
    let this = r32(c, A0);
    c.g[T1][0] = c.g[A0][0]; // the segment
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(1, 4); // f4 = -1.0 (tmin)
    c.g[AT][0] = 0x4000_0000;
    c.mtc1(1, 5); // f5 = 2.0 (tmax)
    let sp = r32(c, SP).wrapping_sub(0x30);
    c.g[SP][0] = sx(sp);
    c.lwc1(8, gpa(c, -0x2d74)); // f8 = epsilon
    c.g[T0][0] = sx(this.wrapping_add(0x70)); // pd = &dir
    c.mtc1(0, 7); // f7 = 0.0
    c.g[A0][0] = sx(this.wrapping_add(0x60)); // po = &origin
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 6); // f6 = 1.0
    c.g[T2][0] = u64::MAX; // fmin = -1
    c.g[T3][0] = u64::MAX; // fmax = -1
    c.g[V1][0] = 0;
    'outer: loop {
        'axis: {
            c.cycles += 6;
            c.lwc1(3, r32(c, T0));
            c.abs_s(0, 3);
            c.c_lt_s(8, 0); // epsilon < |d|
            if !c.fcc() {
                // Parallel to this axis: skip it.
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                break 'axis;
            }
            {
                c.cycles += 11;
                let po = r32(c, A0);
                c.lwc1(0, po);
                c.lwc1(1, r32(c, A1));
                c.lwc1(2, r32(c, A2));
                c.sub_s(1, 1, 0);
                c.sub_s(2, 2, 0);
                c.div_s(1, 1, 3);
                c.div_s(2, 2, 3);
                c.c_lt_s(2, 1);
                let ordered = !c.fcc();
                c.g[V0][0] = sx(r32(c, V1) << 1); // face = i * 2
                if !ordered {
                    c.cycles += 4;
                    c.mov_s(0, 1);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                    c.mov_s(1, 2);
                    c.mov_s(2, 0);
                }
            }
            c.cycles += 3;
            c.c_lt_s(4, 1); // tmin < t1
            if c.fcc() {
                c.cycles += 2;
                c.mov_s(4, 1);
                c.g[T2][0] = c.g[V0][0];
            }
            c.cycles += 3;
            c.c_lt_s(2, 5); // t2 < tmax
            if c.fcc() {
                c.cycles += 2;
                c.mov_s(5, 2);
                c.g[T3][0] = c.g[V0][0] ^ 0x1;
            }
            c.cycles += 4;
            c.c_lt_s(5, 4); // tmax < tmin
            let empty = c.fcc();
            c.g[V0][0] = 0;
            if empty {
                break 'outer; // miss
            }
            c.cycles += 3;
            c.c_lt_s(5, 7); // tmax < 0
            if c.fcc() {
                break 'outer; // behind
            }
            c.cycles += 4;
            c.c_lt_s(6, 4); // 1 < tmin
            let behind = c.fcc();
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            if behind {
                break 'outer; // past the end
            }
        }
        c.cycles += 6;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x4));
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x4));
        c.g[T0][0] = sx(r32(c, T0).wrapping_add(0x4));
        c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
        let next_axis = c.g[V0][0] != 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x4));
        if next_axis {
            if event_due(c) {
                deliver_event_here(c, 0x0032_e6d0);
            }
            continue 'outer;
        }
        // All three axes passed: build the hit records.
        // Lazy initialisation of the six box normals.
        'init: {
            c.cycles += 3;
            c.g[V0][0] = Addr(gpa(c, 0x2888)).lw(c);
            let ready = c.g[V0][0] != 0;
            c.g[V1][0] = 0x500000;
            if ready {
                break 'init;
            }
            c.cycles += 31;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 0); // f0 = -1.0
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1); // f1 = 1.0
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xda0)); // &normals[0]
            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0xda0));
            c.g[A0][0] = 0x1;
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x48));
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x58));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x5c), 0);
            c.sw(gpa(c, 0x2888), c.g[A0][0]);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0);
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x14), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x18), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x20), 0);
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x24));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x28), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x2c), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x30), 0);
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x34));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x38), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x3c), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x40), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x44), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x50), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x54), 0);
        }
        // Entry hit (tmin in [0, 1]).
        'enter: {
            c.cycles += 5;
            c.mtc1(0, 1); // f1 = 0.0
            c.c_le_s(1, 4); // 0 <= tmin
            let miss = !c.fcc();
            c.g[T0][0] = 0;
            if miss {
                break 'enter;
            }
            c.cycles += 41;
            c.g[V0][0] = 0x2;
            c.g[V1][0] = u64::MAX;
            c.sw((c.g[A3][0] as u32).wrapping_add(0x44), c.g[V0][0]);
            c.g[A0][0] = 0x500000;
            c.g[V0][0] = 0x500000;
            c.sw((c.g[A3][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
            c.sw((c.g[A3][0] as u32).wrapping_add(0x54), 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xda0));
            c.sw((c.g[A3][0] as u32).wrapping_add(0x50), 0);
            c.g[A1][0] = sx((c.g[T2][0] as u32) << 4);
            c.sw((c.g[A3][0] as u32).wrapping_add(0x58), 0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xee0));
            c.sw((c.g[A3][0] as u32).wrapping_add(0x5c), c.g[V1][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[A3][0] as u32).wrapping_add(0x60), c.g[V1][0]);
            c.g[T0][0] = 0x1;
            c.sw((c.g[A3][0] as u32).wrapping_add(0x64), c.g[V1][0]);
            c.lq(2, c.g[A0][0] as u32);
            c.sq(2, (c.g[A3][0] as u32).wrapping_add(0x20));
            c.lq(3, c.g[A0][0] as u32);
            c.swc1(1, (c.g[A3][0] as u32).wrapping_add(0x68));
            c.sq(3, (c.g[A3][0] as u32).wrapping_add(0x30));
            c.swc1(1, (c.g[A3][0] as u32).wrapping_add(0x6c));
            c.swc1(1, (c.g[A3][0] as u32).wrapping_add(0x70));
            c.lq(2, c.g[A1][0] as u32);
            c.sq(2, (c.g[A3][0] as u32).wrapping_add(0x10));
            c.mfc1(3, 4); // v1 = tmin bits
            c.lqc2(4, (c.g[T1][0] as u32).wrapping_add(0x70));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp.wrapping_add(0x20));
            c.lq(2, sp.wrapping_add(0x20));
            c.sq(2, sp.wrapping_add(0x10));
            c.lqc2(3, (c.g[T1][0] as u32).wrapping_add(0x60));
            c.lqc2(4, sp.wrapping_add(0x10));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp.wrapping_add(0x20));
            c.lq(2, sp.wrapping_add(0x20));
            c.swc1(4, (c.g[A3][0] as u32).wrapping_add(0x40));
            c.sq(2, c.g[A3][0] as u32);
            c.sq(2, sp);
        }
        // Exit hit (tmax < 1).
        'leave: {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0); // f0 = 1.0
            c.c_lt_s(5, 0); // tmax < 1
            let exit_hit = c.fcc();
            c.g[V1][0] = sx((c.g[T0][0] as u32) << 7);
            if !exit_hit {
                break 'leave;
            }
            c.cycles += 42;
            c.g[V0][0] = 0x2;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
            c.g[A0][0] = u64::MAX;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x44), c.g[V0][0]);
            c.g[A1][0] = 0x500000;
            c.g[V0][0] = 0x500000;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4c), c.g[A0][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x54), 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xda0));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x50), 0);
            c.g[A2][0] = sx((c.g[T3][0] as u32) << 4);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x58), 0);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xee0));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x5c), c.g[A0][0]);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x60), c.g[A0][0]);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x64), c.g[A0][0]);
            c.lq(2, c.g[A1][0] as u32);
            c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x20));
            c.lq(4, c.g[A1][0] as u32);
            c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x70));
            c.sq(4, (c.g[V1][0] as u32).wrapping_add(0x30));
            c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x68));
            c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x6c));
            c.lq(2, c.g[A2][0] as u32);
            c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x10));
            c.mfc1(4, 5); // a0 = tmax bits
            c.lqc2(4, (c.g[T1][0] as u32).wrapping_add(0x70));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp.wrapping_add(0x20));
            c.lq(2, sp.wrapping_add(0x20));
            c.sq(2, sp.wrapping_add(0x10));
            c.lqc2(3, (c.g[T1][0] as u32).wrapping_add(0x60));
            c.lqc2(4, sp.wrapping_add(0x10));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp.wrapping_add(0x20));
            c.lq(2, sp.wrapping_add(0x20));
            c.sq(2, sp);
            c.swc1(5, (c.g[V1][0] as u32).wrapping_add(0x40));
            c.sq(2, c.g[V1][0] as u32);
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[T0][0];
        break 'outer;
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp.wrapping_add(0x30));
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA, GAME_GP};

    /// The object under test.
    const W: u32 = DATA;
    /// Scratch objects, tables and hit records.
    const AUX: u32 = DATA + 0x1000;
    /// Segment table for `Path_GetLength`.
    const SEGMENTS: u32 = AUX + 0x600;

    fn rng(seed: u32, salt: u64) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ ((seed as u64) << 8) ^ salt)
    }

    fn put(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    fn fbits(x: f32) -> u32 {
        x.to_bits()
    }

    /// Scrambled registers (so incidental leftovers are compared) and the
    /// game's `gp`.
    fn setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = rng(seed, salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    /// A `gp`-relative global word.
    fn gpf(c: &mut Cpu, off: i32, v: u32) {
        c.bus.write32(GAME_GP.wrapping_add(off as u32), v);
    }

    /// `n` random bytes at `a`.
    fn fillb(c: &mut Cpu, r: &mut Rng, a: u32, n: u32) {
        for i in 0..n {
            c.bus.write8(a + i, r.next_u32() as u8);
        }
    }

    /// The world fixture: `gp-0x848 -> world -> +0x84 -> manager -> +0xc ->
    /// race`, `race+8` the race's frame limit.
    fn world(c: &mut Cpu, frame_limit: u32) {
        put(c, AUX + 0x100 + 0x84, AUX + 0x200);
        put(c, AUX + 0x200 + 0xc, AUX + 0x300);
        put(c, AUX + 0x300 + 8, frame_limit);
        gpf(c, -0x848, AUX + 0x100);
    }

    // ------------------------------------------------------------------
    // Chase cameras.

    #[test]
    fn spoke_chase_camera_get_tuning_matches() {
        check(ee::s02::f_00177ec8__generated, spoke_chase_camera_get_tuning, 200, &|c, seed| {
            setup(c, seed, 1);
            c.g[A1][0] = AUX as u64;
            gpf(c, -0x5e1c, 0x3f80_0000u32.wrapping_add(seed));
            gpf(c, -0x5e18, 0x4000_0000u32.wrapping_add(seed));
            gpf(c, -0x5e14, 0x4040_0000u32.wrapping_add(seed));
            gpf(c, -0x5e10, 0x4080_0000u32.wrapping_add(seed));
        });
    }

    #[test]
    fn superpipe_chase_camera_get_eye_drop_matches() {
        check(ee::s02::f_001785b0__generated, superpipe_chase_camera_get_eye_drop, 200, &|c, seed| {
            let mut r = setup(c, seed, 2);
            gpf(c, -0x5dc0, r.f32(50.0));
            c.f[0] = r.f32(5.0);
        });
    }

    #[test]
    fn superpipe_chase_camera_get_tuning_matches() {
        check(ee::s02::f_001785c0__generated, superpipe_chase_camera_get_tuning, 200, &|c, seed| {
            setup(c, seed, 3);
            c.g[A1][0] = AUX as u64;
            gpf(c, -0x5dbc, 0x3f80_0000u32.wrapping_add(seed));
            gpf(c, -0x5db8, 0x4000_0000u32.wrapping_add(seed));
            gpf(c, -0x5db4, 0x4040_0000u32.wrapping_add(seed));
            gpf(c, -0x5db0, 0x4080_0000u32.wrapping_add(seed));
        });
    }

    // ------------------------------------------------------------------
    // Move-node modifiers.

    #[test]
    fn mod_obj_v41_matches() {
        check(ee::s11::f_00355858__generated, mod_obj_v41, 200, &|c, seed| {
            let mut r = setup(c, seed, 4);
            gpf(c, 0x2a74, AUX);
            put(c, AUX + 0x10, r.next_u32() % 1000);
            c.g[A0][0] = (AUX + 0x200) as u64;
            c.f[12] = r.edge_f32();
        });
    }

    #[test]
    fn position_modifier_update_transform_matches() {
        check(ee::s11::f_00356ff0__generated, position_modifier_update_transform, 200, &|c, seed| {
            let mut r = setup(c, seed, 5);
            c.g[A0][0] = AUX as u64;
            fillb(c, &mut r, AUX + 0x30, 0x20);
            put(c, AUX + 0x44, r.next_u32());
            fillb(c, &mut r, AUX + 0x80, 0x10);
        });
    }

    #[test]
    fn magnet_modifier_update_transform_matches() {
        check(ee::s11::f_00357528__generated, magnet_modifier_update_transform, 200, &|c, seed| {
            let mut r = setup(c, seed, 6);
            c.g[A0][0] = AUX as u64;
            fillb(c, &mut r, AUX + 0x10, 0x10);
            put(c, AUX + 0x50, r.next_u32());
            fillb(c, &mut r, AUX + 0x90, 0x10);
        });
    }

    // ------------------------------------------------------------------
    // Path events.

    #[test]
    fn path_has_event_in_range_reversed_matches() {
        check(ee::s07::f_0026a0b8__generated, path_has_event_in_range_reversed, 200, &|c, seed| {
            let mut r = setup(c, seed, 7);
            c.g[A0][0] = W as u64;
            put(c, W + 0x38, r.f32(10.0));
            let neg = |x: u32| x | 0x8000_0000;
            c.f[12] = if seed % 3 == 0 { neg(r.f32(2.0)) } else { r.f32(2.0) };
            c.f[13] = if seed % 4 == 0 { neg(r.f32(2.0)) } else { r.f32(2.0) };
            // `Path_HasEventInRange` reads the count at +0 and 0x10-byte
            // items from +4; `Path_GetLength` the count at +8 and 0x10-byte
            // segments (length at +0xc) from the pointer at +0x18.
            let count = seed % 3;
            for i in 0..count {
                let item = W + 4 + 0x10 * i;
                put(c, item, match (seed + i) % 3 { 0 => 0x14, 1 => 0x12, _ => 0 });
                put(c, item + 8, r.f32(2.0));
                put(c, item + 0xc, r.f32(2.0));
                put(c, SEGMENTS + 0x10 * i + 0xc, r.f32(2.0));
            }
            // The same object is both the segment list (`Path_GetLength`
            // reads the count at +8 and the segments at +0x18) and the item
            // list, so set those two words last (`count` is tiny).
            put(c, W, count);
            put(c, W + 8, count);
            put(c, W + 0x18, SEGMENTS);
        });
    }

    #[test]
    fn path_has_event_in_range_matches() {
        check(ee::s07::f_0026ac88__generated, path_has_event_in_range, 250, &|c, seed| {
            let mut r = setup(c, seed, 8);
            c.g[A0][0] = W as u64;
            c.f[12] = r.f32(3.0);
            c.f[13] = r.f32(3.0);
            let count = seed % 4;
            put(c, W, count);
            for i in 0..count {
                let item = W + 4 + 0x10 * i;
                put(c, item, match (seed + i) % 4 { 0 => 0x14, 1 => 0x12, _ => 0 });
                put(c, item + 8, if (seed + i) % 5 == 0 { fbits(10.0) } else { r.f32(2.0) });
                put(c, item + 0xc, if (seed + i) % 7 == 0 { fbits(-10.0) } else { r.f32(2.0) });
            }
        });
    }

    // ------------------------------------------------------------------
    // Front-end input.

    #[test]
    fn feui_input_set_port_matches() {
        check(ee::s02::f_00180478__generated, feui_input_set_port, 200, &|c, seed| {
            let mut r = setup(c, seed, 9);
            c.g[A0][0] = AUX as u64;
            let port = seed % 5;
            c.g[A1][0] = 0xabcd_ff00u64 | port as u64;
            put(c, W + 0xb0 + 4 * port, r.next_u32());
            gpf(c, -0x848, W);
        });
    }

    #[test]
    fn feui_input_has_binding_matches() {
        check(ee::s02::f_00180498__generated, feui_input_has_binding, 200, &|c, seed| {
            let mut r = setup(c, seed, 10);
            c.g[A0][0] = AUX as u64;
            match seed % 4 {
                0 => put(c, AUX + 0x18, 0),
                n => {
                    put(c, AUX + 0x18, AUX + 0x40);
                    put(c, AUX + 0x40, AUX + 0x80);
                    put(c, AUX + 0x80, if n == 1 { 0 } else { r.next_u32() });
                }
            }
        });
    }

    // ------------------------------------------------------------------
    // Reward gallery.

    fn clear_flags_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = setup(c, seed, salt);
        c.g[A0][0] = AUX as u64;
        for w in 0..((0xc00u32 - 0xa20) / 4) {
            put(c, AUX + 0xa20 + 4 * w, r.next_u32());
        }
    }

    #[test]
    fn reward_gallery_posters_clear_item_flags_matches() {
        check(ee::s04::f_001d1290__generated, reward_gallery_posters_clear_item_flags, 200, &|c, seed| {
            clear_flags_fixture(c, seed, 11)
        });
    }

    #[test]
    fn reward_gallery_trading_cards_clear_item_flags_matches() {
        check(ee::s04::f_001d1428__generated, reward_gallery_trading_cards_clear_item_flags, 200, &|c, seed| {
            clear_flags_fixture(c, seed, 12)
        });
    }

    #[test]
    fn reward_gallery_concept_arts_clear_item_flags_matches() {
        check(ee::s04::f_001d15c0__generated, reward_gallery_concept_arts_clear_item_flags, 200, &|c, seed| {
            clear_flags_fixture(c, seed, 13)
        });
    }

    #[test]
    fn reward_gallery_videos_clear_item_flags_matches() {
        check(ee::s04::f_001d1758__generated, reward_gallery_videos_clear_item_flags, 200, &|c, seed| {
            clear_flags_fixture(c, seed, 14)
        });
    }

    #[test]
    fn reward_gallery_toys_clear_item_flags_matches() {
        check(ee::s04::f_001d18f0__generated, reward_gallery_toys_clear_item_flags, 200, &|c, seed| {
            clear_flags_fixture(c, seed, 15)
        });
    }

    #[test]
    fn reward_gallery_cheat_chars_clear_item_flags_matches() {
        check(ee::s04::f_001d1a28__generated, reward_gallery_cheat_chars_clear_item_flags, 200, &|c, seed| {
            clear_flags_fixture(c, seed, 16)
        });
    }

    #[test]
    fn preview_reward_on_widget_notify_matches() {
        check(ee::s04::f_001d1f78__generated, preview_reward_on_widget_notify, 200, &|c, seed| {
            setup(c, seed, 17);
            c.g[A2][0] = (seed % 12) as u64;
        });
    }

    // ------------------------------------------------------------------
    // State predicates.

    #[test]
    fn rider_is_computer_matches() {
        check(ee::s01::f_00140bc8__generated, rider_is_computer, 200, &|c, seed| {
            let mut r = setup(c, seed, 18);
            c.g[A0][0] = AUX as u64;
            put(c, AUX + 0x874, if seed % 3 == 0 { 0 } else if seed % 3 == 1 { 1 } else { r.next_u32() });
        });
    }

    #[test]
    fn game_state_reset_v08_matches() {
        check(ee::s06::f_002355b0__generated, game_state_reset_v08, 200, &|c, seed| {
            setup(c, seed, 19);
            c.g[A0][0] = AUX as u64;
            put(c, AUX + 0x10, [5, 4, 6][(seed % 3) as usize]);
        });
    }

    #[test]
    fn game_state_finish_cinematic_exit_matches() {
        check(ee::s06::f_00236f90__generated, game_state_finish_cinematic_exit, 200, &|c, seed| {
            let mut r = setup(c, seed, 20);
            world(c, 0);
            put(c, AUX + 0x300 + 0x8c, r.next_u32());
        });
    }

    #[test]
    fn player_pulse_rumble_matches() {
        check(ee::s00::f_001278d0__generated, player_pulse_rumble, 200, &|c, seed| {
            let mut r = setup(c, seed, 21);
            c.g[A0][0] = AUX as u64;
            put(c, AUX + 0xdfc, r.f32(2.0));
            c.f[12] = if seed % 2 == 0 { r.edge_f32() } else { r.f32(2.0) };
        });
    }

    // ------------------------------------------------------------------
    // Sound banks.

    #[test]
    fn snd_bank_man_init_bank_request_matches() {
        covered(ee::s07::f_0028bf38__generated, snd_bank_man_init_bank_request, 150, &|c, seed| {
            let mut r = machine(c, seed, 22, 3);
            c.g[A0][0] = W as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = if seed % 2 == 0 { 1 } else { 0 };
            c.g[A3][0] = (AUX + 0x400) as u64;
            c.g[T0][0] = (r.next_u32() % 4) as u64;
            c.g[T1][0] = (r.next_u32() % 4) as u64;
            c.g[T2][0] = (r.next_u32() % 4) as u64;
            put(c, W + 0xacc, AUX + 0x800);
            // `SndBankMan_InitLoadRequest` walks
            // gp+0x524 -> +0x1d8 -> [0] -> [0] and calls the function
            // pointer at +0x3c (an unregistered address, so the test
            // machine's fake records the call).
            gpf(c, 0x524, AUX + 0x900);
            put(c, AUX + 0x900 + 0x1d8, AUX + 0x920);
            put(c, AUX + 0x920, AUX + 0x940);
            put(c, AUX + 0x940, AUX + 0x960);
            c.bus.write16(AUX + 0x960 + 0x38, 0);
            put(c, AUX + 0x960 + 0x3c, FAKE);
            c.bus.write8(AUX + 0x400, 0); // the strcpy source is empty
        });
    }

    // ------------------------------------------------------------------
    // Front-end option screens.

    #[test]
    fn cfe_state_options_handle_input_matches() {
        check(ee::s02::f_00188c58__generated, cfe_state_options_handle_input, 200, &|c, seed| {
            let mut r = setup(c, seed, 23);
            c.g[A1][0] = AUX as u64;
            c.bus.write8(AUX + 4, r.next_u32() as u8);
        });
    }

    #[test]
    fn cfe_state_options_game_on_widget_notify_matches() {
        check(ee::s02::f_00189958__generated, cfe_state_options_game_on_widget_notify, 200, &|c, seed| {
            setup(c, seed, 24);
            c.g[A2][0] = [9u64, 0, 6, 1][(seed % 4) as usize];
        });
    }

    #[test]
    fn cfe_state_options_game_v21_matches() {
        check(ee::s02::f_00189970__generated, cfe_state_options_game_v21, 200, &|c, seed| {
            setup(c, seed, 25);
            c.g[A2][0] = if seed % 2 == 0 { 0 } else { seed as u64 };
        });
    }

    #[test]
    fn cfe_state_options_sound_on_widget_notify_matches() {
        check(ee::s02::f_0018a890__generated, cfe_state_options_sound_on_widget_notify, 200, &|c, seed| {
            setup(c, seed, 26);
            c.g[A2][0] = [6u64, 9, 0, 3][(seed % 4) as usize];
        });
    }

    #[test]
    fn cfe_state_options_sound_v21_matches() {
        check(ee::s02::f_0018a8c0__generated, cfe_state_options_sound_v21, 200, &|c, seed| {
            setup(c, seed, 27);
            c.g[A2][0] = if seed % 2 == 0 { 0 } else { seed as u64 };
        });
    }

    #[test]
    fn cfe_state_options_hud_on_widget_notify_matches() {
        check(ee::s02::f_0018c270__generated, cfe_state_options_hud_on_widget_notify, 200, &|c, seed| {
            setup(c, seed, 28);
            c.g[A2][0] = [6u64, 9, 0, 3][(seed % 4) as usize];
        });
    }

    #[test]
    fn cfe_state_options_controller_v21_matches() {
        check(ee::s02::f_0018cdb0__generated, cfe_state_options_controller_v21, 200, &|c, seed| {
            setup(c, seed, 29);
            c.g[A2][0] = if seed % 2 == 0 { 0 } else { seed as u64 };
        });
    }

    #[test]
    fn cfe_state_options_save_load_on_widget_notify_matches() {
        check(ee::s02::f_0018d438__generated, cfe_state_options_save_load_on_widget_notify, 200, &|c, seed| {
            setup(c, seed, 30);
            c.g[A2][0] = [0u64, 7, 8, 9, 10, 0xffff_ffff][(seed % 6) as usize];
        });
    }

    // ------------------------------------------------------------------
    // Memory-card and overlay states.

    #[test]
    fn mem_card_state_base_v46_matches() {
        check(ee::s04::f_001d8670__generated, mem_card_state_base_v46, 200, &|c, seed| {
            let mut r = setup(c, seed, 31);
            gpf(c, -0x10c8, AUX);
            put(c, AUX + 0x1c0, if seed % 2 == 0 { 4 } else { r.next_u32() % 8 });
            put(c, AUX + 0x1a8, r.next_u32());
            put(c, AUX + 0x1e0, r.next_u32());
        });
    }

    #[test]
    fn mem_card_state_base_transition_out_matches() {
        check(ee::s04::f_001d93d8__generated, mem_card_state_base_transition_out, 200, &|c, seed| {
            setup(c, seed, 32);
            c.g[A0][0] = AUX as u64;
            put(c, AUX + 0x1c0, [6u32, 5, 7, 0x100][(seed % 4) as usize]);
        });
    }

    #[test]
    fn ov_screen_be_options_v22_matches() {
        check(ee::s04::f_001faaf0__generated, ov_screen_be_options_v22, 200, &|c, seed| {
            setup(c, seed, 33);
            c.g[A2][0] = [4u64, 0, 9][(seed % 3) as usize];
        });
    }

    #[test]
    fn ov_screen_be_options_v21_matches() {
        check(ee::s04::f_001fab08__generated, ov_screen_be_options_v21, 200, &|c, seed| {
            setup(c, seed, 34);
            c.g[A2][0] = if seed % 2 == 0 { 0 } else { seed as u64 };
        });
    }

    #[test]
    fn ov_screen_race_pre_on_deactivate_matches() {
        check(ee::s04::f_001fbcc8__generated, ov_screen_race_pre_on_deactivate, 200, &|c, seed| {
            let mut r = setup(c, seed, 35);
            if seed % 3 == 0 {
                gpf(c, -0x204, 0);
            } else {
                gpf(c, -0x204, AUX);
                put(c, AUX + 0x64, r.next_u32());
            }
        });
    }

    #[test]
    fn ov_screen_rival_pre_on_widget_notify_matches() {
        check(ee::s05::f_001fdb78__generated, ov_screen_rival_pre_on_widget_notify, 200, &|c, seed| {
            setup(c, seed, 36);
            c.g[A2][0] = [6u64, 0, 9][(seed % 3) as usize];
        });
    }

    // ------------------------------------------------------------------
    // Lens FX.

    #[test]
    fn lens_fx_base_push_render_state_matches() {
        check(ee::s09::f_002e3578__generated, lens_fx_base_push_render_state, 200, &|c, seed| {
            let mut r = setup(c, seed, 37);
            gpf(c, 0x2a90, AUX + 0x100);
            put(c, AUX + 0x100 + 0xe84, AUX + 0x400);
            fillb(c, &mut r, AUX + 0x400, 0x40);
            fillb(c, &mut r, 0x0050_1420, 0x40);
        });
    }

    #[test]
    fn lens_fx_base_pop_render_state_matches() {
        check(ee::s09::f_002e3668__generated, lens_fx_base_pop_render_state, 200, &|c, seed| {
            let mut r = setup(c, seed, 38);
            gpf(c, -0x854, AUX);
            put(c, AUX + 0xe84, r.next_u32());
        });
    }

    #[test]
    fn c_transition_man_script_is_running_matches() {
        check(ee::s09::f_002e4c90__generated, c_transition_man_script_is_running, 200, &|c, seed| {
            let mut r = setup(c, seed, 39);
            c.g[A0][0] = AUX as u64;
            put(c, AUX + 0x60, if seed % 2 == 0 { 0 } else { AUX + 0x200 });
            put(c, AUX + 0x64, if seed % 4 == 1 { 0 } else { r.next_u32() | 1 });
        });
    }

    // ------------------------------------------------------------------
    // Beast character controller.

    #[test]
    fn be_char_int_commit_matches() {
        check(ee::s01::f_00147fd8__generated, be_char_int_commit, 200, &|c, seed| {
            let mut r = setup(c, seed, 40);
            fillb(c, &mut r, 0x0053_5538, 0x100);
            for i in 0..10u32 {
                fillb(c, &mut r, 0x004a_7887 + i * 0xf88, 8);
                fillb(c, &mut r, 0x004b_13d7 + i * 0xf88, 8);
                fillb(c, &mut r, 0x004b_af27 + i * 0xf88, 8);
            }
        });
    }

    // ------------------------------------------------------------------
    // Collision.

    #[test]
    fn coll_sphere_get_diameter_matches() {
        check(ee::s11::f_00340a18__generated, coll_sphere_get_diameter, 200, &|c, seed| {
            let mut r = setup(c, seed, 41);
            c.g[A0][0] = AUX as u64;
            put(c, AUX, if seed % 3 == 0 { 0 } else { r.next_u32() });
            put(c, AUX + 0x70, r.f32(10.0));
            c.f[0] = r.f32(1.0);
            c.f[1] = r.f32(1.0);
        });
    }

    #[test]
    fn boost_handle_event_matches() {
        check(ee::s11::f_00341a30__generated, boost_handle_event, 200, &|c, seed| {
            let mut r = setup(c, seed, 42);
            c.g[A0][0] = AUX as u64;
            c.g[A1][0] = [1u64, 0, 1, 2][(seed % 4) as usize];
            put(c, AUX + 0x38, r.next_u32());
            put(c, AUX + 0x3c, r.next_u32());
        });
    }

    #[test]
    fn anim_teeter_v53_matches() {
        // (lean +0x4, upper +0x10, lower +0x14, velocity +0x30)
        // Four distinct paths: the early `0.0` (heading inwards), the
        // `0 < lean` clamp, the outwards bypass and the inner `0.0`.
        const CASES: [[f32; 4]; 6] = [
            [1.0, 2.0, -2.0, 1.0],   // velocity <= upper, lean >= 0, 0 < lean -> 0
            [-1.0, 2.0, -2.0, 1.0],  // velocity <= upper, lean < 0 -> 0 (heading in)
            [1.0, 0.5, -2.0, 1.0],   // velocity > upper, lower <= velocity, 0 < lean -> 0
            [-1.0, 0.5, 2.0, 1.0],   // velocity > upper, lower > velocity -> lean
            [-1.0, 2.0, -2.0, -5.0], // velocity <= upper, lean < 0 -> 0
            [0.0, 2.0, -2.0, 1.0],   // velocity <= upper, lean == 0 -> lean
        ];
        check(ee::s11::f_003424d0__generated, anim_teeter_v53, 200, &|c, seed| {
            let mut r = setup(c, seed, 43);
            c.g[A0][0] = AUX as u64;
            let k = CASES[(seed % 6) as usize];
            put(c, AUX + 4, if seed % 7 == 0 { r.f32(2.0) } else { fbits(k[0]) });
            put(c, AUX + 0x10, fbits(k[1]));
            put(c, AUX + 0x14, fbits(k[2]));
            put(c, AUX + 0x30, fbits(k[3]));
        });
    }

    #[test]
    fn boost_modifier_handle_event_matches() {
        check(ee::s11::f_00342b80__generated, boost_modifier_handle_event, 200, &|c, seed| {
            let mut r = setup(c, seed, 44);
            c.g[A0][0] = AUX as u64;
            c.g[A1][0] = [0x258u64, 0, 0x258, 0x100][(seed % 4) as usize];
            put(c, AUX + 0x28, r.next_u32());
            put(c, AUX + 0x2c, r.next_u32());
        });
    }

    #[test]
    fn gm_handler0_v09_matches() {
        check_lohi(ee::s06::f_00239ce0__generated, gm_handler0_v09, 200, &|c, seed| {
            let mut r = setup(c, seed, 45);
            c.g[A0][0] = AUX as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = (seed % 5) as u64;
            put(c, AUX + 0x600, 3); // the row count
            gpf(c, -0x480, AUX + 0x600);
            for w in 0..(3u32 * 0x28 / 4) {
                put(c, AUX + 4 * w, r.next_u32());
            }
        });
    }

    #[test]
    fn gm_handler9_v06_matches() {
        check_lohi(ee::s06::f_0023a070__generated, gm_handler9_v06, 200, &|c, seed| {
            let mut r = setup(c, seed, 46);
            world(c, r.next_u32() % 400);
            c.g[A0][0] = W as u64;
            c.g[A1][0] = (seed % 2) as u64;
            c.g[A2][0] = ((seed % 5) * 3) as u64;
            c.bus.write16(W + 4, if seed % 3 == 0 { 5 } else { (r.next_u32() % 20) as u16 });
            c.bus.write16(W + 6, (r.next_u32() % 20) as u16);
            gpf(c, -0x480, AUX + 0x600);
            put(c, AUX + 0x600 + 8, if seed % 2 == 0 { 1 } else { 0 });
            put(c, AUX + 0x600 + 0x78, r.next_u32() % 400);
        });
    }

    #[test]
    fn coll_segment_reset_transform_matches() {
        check(ee::s11::f_0032e4b8__generated, coll_segment_reset_transform, 200, &|c, seed| {
            let mut r = setup(c, seed, 47);
            c.g[A0][0] = AUX as u64;
            fillb(c, &mut r, AUX + 0x60, 0x40);
            put(c, AUX + 4, r.next_u32());
        });
    }

    #[test]
    fn coll_segment_collide_aabb_matches() {
        check(ee::s11::f_0032e690__generated, coll_segment_collide_aabb, 250, &|c, seed| {
            let mut r = setup(c, seed, 48);
            gpf(c, 0x2888, if seed % 2 == 0 { 0 } else { 1 });
            gpf(c, -0x2d74, fbits(1e-6));
            let a = |x: f32| fbits(x);
            let (o, d, lo, hi): ([u32; 3], [u32; 3], [u32; 3], [u32; 3]) = match seed % 8 {
                // Through the box: an entry and an exit hit.
                0 => ([0, 0, 0], [a(1.0); 3], [a(-0.5); 3], [a(0.5); 3]),
                // Origin inside, exit ahead only: one hit.
                1 => ([0, 0, 0], [a(1.0); 3], [a(-2.0); 3], [a(0.5); 3]),
                // Box behind the origin: miss (tmax < 0).
                2 => ([0, 0, 0], [a(1.0); 3], [a(-1.0); 3], [a(-0.5); 3]),
                // Box beyond the segment: miss (1 < tmin).
                3 => ([0, 0, 0], [a(1.0); 3], [a(2.0); 3], [a(3.0); 3]),
                // Parallel to x.
                4 => ([0, 0, 0], [0, a(1.0), a(1.0)], [a(-0.5); 3], [a(0.5); 3]),
                // Box to the side: miss (tmax < tmin).
                5 => (
                    [0, 0, 0],
                    [a(1.0); 3],
                    [a(5.0), a(-1.0), a(-1.0)],
                    [a(6.0), a(1.0), a(1.0)],
                ),
                // Entering from the far corner.
                6 => ([a(1.0); 3], [a(-1.0); 3], [a(0.2); 3], [a(0.8); 3]),
                // Arbitrary.
                _ => {
                    let mut v = [0u32; 3];
                    for slot in v.iter_mut() {
                        *slot = r.f32(2.0);
                    }
                    let mut mn = [0u32; 3];
                    let mut mx = [0u32; 3];
                    for k in 0..3usize {
                        let x = f32::from_bits(r.f32(2.0));
                        let y = f32::from_bits(r.f32(2.0));
                        mn[k] = fbits(x.min(y));
                        mx[k] = fbits(x.max(y));
                    }
                    (v, [r.f32(2.0), r.f32(2.0), r.f32(2.0)], mn, mx)
                }
            };
            c.g[A0][0] = AUX as u64;
            for k in 0..3usize {
                let off = 4 * k as u32;
                put(c, AUX + 0x60 + off, o[k]);
                put(c, AUX + 0x70 + off, d[k]);
                put(c, AUX + 0x200 + off, lo[k]);
                put(c, AUX + 0x210 + off, hi[k]);
            }
            c.g[A1][0] = (AUX + 0x200) as u64;
            c.g[A2][0] = (AUX + 0x210) as u64;
            c.g[A3][0] = (AUX + 0x400) as u64;
            fillb(c, &mut r, AUX + 0x400, 0x120);
        });
    }
}
