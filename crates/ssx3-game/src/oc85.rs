//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! Small leaf functions, in three groups:
//!
//! - **Front end** (`crates/ssx3-game`, addresses 0x1a..0x1e): the
//!   `cFEState*_OnWidgetNotify` / `_OnWidgetAction` handlers, which map a
//!   widget action key onto the screen-system result codes 0x100/0x101, the
//!   popup layout accessors (0x48/0x4c/0x50), and the reward gallery's
//!   `GetItemPrice` methods (the owned count at `item+0xc` times ten credits).
//! - **Overworld screens**: the `OVScreen*_OnWidgetNotify`/`_OnWidgetAction`
//!   handlers (the action handler publishes the picked row through the two
//!   globals at `gp-0x9f4`/`gp-0x9f0`), the finish screen's `OnEvent` and the
//!   view popup's notify (a switch on the action: `8` looks up the popup
//!   entry at `D_004C6C08 + index*0x18`, its `D_00441630 + field*0x14` target,
//!   and returns whether that word is not 4).
//! - **Rail modifiers, processes and script objects** (0x26..0x36): the
//!   transform-bound quadword copies, the `0.0 <= x` / `17.0 <= x`
//!   predicates, the rail entry accessors (`*(self+0x70) + index*0x50/0x60`),
//!   the 30·x anim-object input scaler, `TypedProcess_IsType`, and the
//!   sound-pool base setter (the one function here that calls out, through the
//!   function table exactly as the original's indirect call does).
//!
//! `mult` goes through [`abi::mult`] (pipeline 0, with LO/HI as the hardware
//! leaves them); FPU arithmetic uses [`FpuF32`] and every compare leaves the
//! FCC bit via [`abi::set_fcc`]. Each replacement ends with [`abi::ret`] (or
//! the frame's `close`, which does the same after restoring `sp`).

use crate::abi;
use crate::call::{call_ptr, guest, Frame};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001a_c5a8,
        fe_use_existing_account_on_widget_notify,
        "cFEUseExistingAccount_OnWidgetNotify",
    ),
    (
        0x001a_e100,
        fe_state_challenge_popup_on_widget_notify,
        "cFEStateChallengePopup_OnWidgetNotify",
    ),
    (
        0x001b_06e0,
        fe_state_challenge_received_popup_on_widget_notify,
        "cFEStateChallengeReceivedPopup_OnWidgetNotify",
    ),
    (
        0x001b_6a90,
        net_config_auth_popup_on_widget_notify,
        "NetConfigAuthPopup_OnWidgetNotify",
    ),
    (
        0x001b_6ea8,
        fe_state_auth_dnas_error_on_widget_notify,
        "cFEStateAuthDNASError_OnWidgetNotify",
    ),
    (
        0x001c_0b08,
        fe_state_online_agreement_on_widget_notify,
        "cFEStateOnlineAgreement_OnWidgetNotify",
    ),
    (
        0x001c_17e0,
        fe_state_net_quick_play_popup_on_widget_notify,
        "cFEStateNetQuickPlayPopup_OnWidgetNotify",
    ),
    (
        0x001c_5420,
        fe_state_net_news_on_widget_notify,
        "cFEStateNetNews_OnWidgetNotify",
    ),
    (
        0x001d_f028,
        fe_popup_screen_pos_v23,
        "cFEPopupScreenPos_v23",
    ),
    (
        0x001d_fb98,
        fe_state_reward_gallery_posters_get_item_price,
        "cFEStateRewardGalleryPosters_GetItemPrice",
    ),
    (
        0x001d_fc68,
        fe_state_reward_gallery_trading_cards_get_item_price,
        "cFEStateRewardGalleryTradingCards_GetItemPrice",
    ),
    (
        0x001d_fd38,
        fe_state_reward_gallery_concept_arts_get_item_price,
        "cFEStateRewardGalleryConceptArts_GetItemPrice",
    ),
    (
        0x001d_fe08,
        fe_state_reward_gallery_videos_get_item_price,
        "cFEStateRewardGalleryVideos_GetItemPrice",
    ),
    (
        0x001d_fed8,
        fe_state_reward_gallery_toys_get_item_price,
        "cFEStateRewardGalleryToys_GetItemPrice",
    ),
    (
        0x001d_ffa8,
        fe_state_reward_gallery_cheat_chars_get_item_price,
        "cFEStateRewardGalleryCheatChars_GetItemPrice",
    ),
    (
        0x001e_5a18,
        ov_state_view_popup_on_widget_notify,
        "cOVState_ViewPopup_OnWidgetNotify",
    ),
    (
        0x001e_6630,
        ov_screen_freestyle_standings_on_widget_notify,
        "OVScreenFreestyleStandings_OnWidgetNotify",
    ),
    (
        0x001e_6640,
        ov_screen_freestyle_standings_on_widget_action,
        "OVScreenFreestyleStandings_OnWidgetAction",
    ),
    (
        0x001e_7b78,
        ov_screen_final_standings_on_widget_notify,
        "OVScreenFinalStandings_OnWidgetNotify",
    ),
    (
        0x001e_7b88,
        ov_screen_final_standings_on_widget_action,
        "OVScreenFinalStandings_OnWidgetAction",
    ),
    (
        0x001e_81d0,
        ov_screen_finish_ov_on_event,
        "OVScreenFinishOv_OnEvent",
    ),
    (
        0x001e_88f0,
        ov_screen_finish_ov2_on_event,
        "OVScreenFinishOv2_OnEvent",
    ),
    (
        0x001e_8e98,
        ov_screen_peak_challenge_results_on_widget_notify,
        "OVScreenPeakChallengeResults_OnWidgetNotify",
    ),
    (
        0x001e_8ea8,
        ov_screen_peak_challenge_results_on_widget_action,
        "OVScreenPeakChallengeResults_OnWidgetAction",
    ),
    (
        0x001e_8fc8,
        ov_screen_multiplayer_standings_on_widget_action,
        "OVScreenMultiplayerStandings_OnWidgetAction",
    ),
    (0x0036_0768, game_process_v11, "GameProcess_v11"),
    (0x0036_07a0, typed_process_is_type, "TypedProcess_IsType"),
    (0x0036_0dd8, mod_obj_v51, "ModObj_v51"),
    (0x0036_1068, anim_object_v30, "AnimObject_v30"),
    (0x0036_1158, anim_object_v53, "AnimObject_v53"),
    (0x0036_1170, anim_object_v31, "AnimObject_v31"),
    (0x0036_1460, anim_teeter_v31, "AnimTeeter_v31"),
    (0x0036_15a8, float_rail_v49, "FloatRail_v49"),
    (0x0036_15c0, spring_rail_v49, "SpringRail_v49"),
    (
        0x0036_1918,
        position_modifier_set_transform_bounds,
        "PositionModifier_SetTransformBounds",
    ),
    (
        0x0036_1a30,
        magnet_modifier_set_transform_bounds,
        "MagnetModifier_SetTransformBounds",
    ),
    (
        0x0036_1ae0,
        ava_spline_modifier_set_transform_bounds,
        "AvaSplineModifier_SetTransformBounds",
    ),
    (
        0x0036_1b60,
        spline_modifier_set_transform_bounds,
        "SplineModifier_SetTransformBounds",
    ),
    (
        0x0036_1bf0,
        multi_spline_modifier_set_transform_bounds,
        "MultiSplineModifier_SetTransformBounds",
    ),
    (
        0x0036_1cb0,
        roller_modifier_is_transform_finished,
        "RollerModifier_IsTransformFinished",
    ),
    (
        0x0036_1d00,
        roller_modifier_set_transform_bounds,
        "RollerModifier_SetTransformBounds",
    ),
    (0x0036_1d78, rail_man_v11, "RailMan_v11"),
    (0x0036_1d90, anim_combo_v31, "AnimCombo_v31"),
    (
        0x0026_6348,
        shared_login_man_mc_cb_state_0f,
        "cSharedLoginMan_McCb_State0F",
    ),
    (
        0x0026_7dd8,
        dsock_channel_base_v05,
        "cDSockChannel_Base_v05",
    ),
    (
        0x0026_7e00,
        dsock_channel_is_connected,
        "cDSockChannel_IsConnected",
    ),
    (
        0x002a_9250,
        ps2_sound_manager_set_spu_mem_base,
        "PS2SoundManager_SetSpuMemBase",
    ),
    (
        0x0027_c040,
        script_dummy_obj_accept_command,
        "ScriptDummyObj_AcceptCommand",
    ),
    (
        0x0028_08d8,
        script_audio_obj_accept_command,
        "ScriptAudioObj_AcceptCommand",
    ),
    (
        0x0028_1170,
        script_view_obj_get_binding,
        "ScriptViewObj_GetBinding",
    ),
];

// ---------------------------------------------------------------------------
// Helpers.

/// `gp + off`, the address of a game global (the code addresses them through
/// `gp`, so the tests must set it).
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `lwc1 fN, a`: load the 32-bit pattern into FPU register `n`.
#[inline(always)]
fn ldf(c: &mut Cpu, n: usize, a: u32) -> FpuF32 {
    let v = Addr(a).read_fpu(c);
    c.f[n] = v.0;
    v
}

/// `li at, bits; mtc1 at, fN` (as gcc materialises a float literal).
#[inline(always)]
fn li_f(c: &mut Cpu, n: usize, bits: u32) -> FpuF32 {
    c.g[AT][0] = sx(bits);
    c.f[n] = bits;
    FpuF32(bits)
}

/// The FPU condition bit after a `c.*.s` compare; returns the condition.
#[inline(always)]
fn fcc(c: &mut Cpu, cond: bool) -> bool {
    abi::set_fcc(c, cond);
    cond
}

/// `x >= 0.0f` as the original tests it: `li at, 0; mtc1 at, f0; c.le.s f0, f1`
/// then a `bc1t`.
#[inline(always)]
fn non_negative(c: &mut Cpu, x: FpuF32) -> bool {
    c.f[0] = 0;
    fcc(c, FpuF32::ZERO.le(x))
}

// ---------------------------------------------------------------------------
// Shared bodies of the repeated handler shapes (the generated code is
// identical for each group; only the address differs).

/// The four `cFEState*_OnWidgetNotify` handlers whose action key is tested
/// against 10 and 8: below 8 (and 10 and up) give 0x101, 8 and 9 give 0x100.
fn state_notify_low_action(c: &mut Cpu) {
    c.cycles += 3;
    let key = c.g[A2][0];
    c.g[V1][0] = (key < 10) as u64;
    c.g[V0][0] = 0x101;
    if key < 10 {
        c.cycles += 4;
        let below_eight = key < 8;
        c.g[A0][0] = below_eight as u64;
        c.g[V1][0] = 0x101;
        c.g[V0][0] = 0x100;
        if below_eight {
            c.g[V0][0] = c.g[V1][0];
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// The three `OVScreen*_OnWidgetNotify` handlers: the action key is xored
/// with 6; 0x101 only when it *was* 6, else 0.
fn ov_screen_notify_action_six(c: &mut Cpu) {
    c.cycles += 4;
    let key = c.g[A2][0] ^ 6;
    c.g[A2][0] = key;
    c.g[V0][0] = 0x101;
    if key != 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// The four `OVScreen*_OnWidgetAction` handlers: a null menu is ignored;
/// action 5 stores 1 in `gp-0x9f4` and the row value at `menu+0x18` in
/// `gp-0x9f0` (v0 = 1, v1 = the row value).
fn ov_screen_action_row(c: &mut Cpu) {
    c.cycles += 2;
    let menu = c.g[A1][0];
    c.g[V0][0] = 5;
    if menu != 0 {
        c.cycles += 2;
        let action = c.g[A2][0];
        c.g[V0][0] = 1;
        if action == 5 {
            c.cycles += 3;
            let row = Addr(menu as u32).off(0x18).lw(c);
            c.g[V1][0] = row;
            let one = c.g[V0][0];
            Addr(gp(c, -0x9f4)).sw(c, one);
            Addr(gp(c, -0x9f0)).sw(c, row);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// The six reward-gallery `GetItemPrice` methods: the owned count at
/// `item+0xc` (a signed short) times ten credits — or the count itself when
/// it is zero or negative.
fn gallery_item_price(c: &mut Cpu) {
    c.cycles += 3;
    let count = Addr(abi::arg(c, 1)).off(0xc).lh(c);
    c.g[V1][0] = count;
    c.g[V0][0] = 0xa;
    let price;
    if (count as i64) <= 0 {
        price = count;
    } else {
        c.cycles += 1;
        let product = abi::mult(c, count as u32, 0xa);
        c.g[V1][0] = sx(product);
        price = sx(product);
    }
    c.cycles += 2;
    c.g[V0][0] = price;
    abi::ret(c);
}

/// The two finish-screen `OnEvent` handlers: event 0 clears the "showing"
/// word at `+0xa0`, event 1 sets it to 1, anything else is ignored; v0 = 0.
fn ov_screen_finish_on_event(c: &mut Cpu) {
    c.cycles += 2;
    let event = c.g[A1][0];
    c.g[V0][0] = 1;
    if event == 0 {
        c.cycles += 2;
        Addr(abi::arg(c, 0)).off(0xa0).sw(c, 0);
        c.cycles += 1;
        c.g[V0][0] = 0;
    } else if event == 1 {
        c.cycles += 2;
        c.cycles += 1;
        Addr(abi::arg(c, 0)).off(0xa0).sw(c, event);
        c.cycles += 1;
        c.g[V0][0] = 0;
    } else {
        c.cycles += 2;
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Front-end widget-notify handlers.

/// `int cFEUseExistingAccount_OnWidgetNotify(void* self, int widget, int action)`
/// (0x1ac5a8): 0x100 when the action is 9 (back), 0x101 otherwise. The xor
/// leaves the tested key in a2.
pub fn fe_use_existing_account_on_widget_notify(c: &mut Cpu) {
    c.cycles += 5;
    let key = c.g[A2][0] ^ 9;
    c.g[A2][0] = key;
    c.g[V1][0] = 0x100;
    c.g[V0][0] = 0x101;
    if key == 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    abi::ret(c);
}

/// `int cFEStateChallengePopup_OnWidgetNotify(void* self, int widget, int action)`
/// (0x1ae100): see [`state_notify_low_action`].
pub fn fe_state_challenge_popup_on_widget_notify(c: &mut Cpu) {
    state_notify_low_action(c);
}

/// `int cFEStateChallengeReceivedPopup_OnWidgetNotify(...)` (0x1b06e0): see
/// [`state_notify_low_action`].
pub fn fe_state_challenge_received_popup_on_widget_notify(c: &mut Cpu) {
    state_notify_low_action(c);
}

/// `int NetConfigAuthPopup_OnWidgetNotify(...)` (0x1b6a90): see
/// [`state_notify_low_action`].
pub fn net_config_auth_popup_on_widget_notify(c: &mut Cpu) {
    state_notify_low_action(c);
}

/// `int cFEStateAuthDNASError_OnWidgetNotify(...)` (0x1b6ea8): see
/// [`state_notify_low_action`].
pub fn fe_state_auth_dnas_error_on_widget_notify(c: &mut Cpu) {
    state_notify_low_action(c);
}

/// `int cFEStateOnlineAgreement_OnWidgetNotify(...)` (0x1c0b08): 0x100 when
/// the action is 9 (accept/back), 0x101 otherwise.
pub fn fe_state_online_agreement_on_widget_notify(c: &mut Cpu) {
    c.cycles += 5;
    let key = c.g[A2][0] ^ 9;
    c.g[A2][0] = key;
    c.g[V1][0] = 0x100;
    c.g[V0][0] = 0x101;
    if key == 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    abi::ret(c);
}

/// `int cFEStateNetQuickPlayPopup_OnWidgetNotify(...)` (0x1c17e0): 0x100
/// when the action is 9, 0x101 otherwise.
pub fn fe_state_net_quick_play_popup_on_widget_notify(c: &mut Cpu) {
    c.cycles += 5;
    let key = c.g[A2][0] ^ 9;
    c.g[A2][0] = key;
    c.g[V1][0] = 0x100;
    c.g[V0][0] = 0x101;
    if key == 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    abi::ret(c);
}

/// `int cFEStateNetNews_OnWidgetNotify(void* self, int widget, int action)`
/// (0x1c5420): 0 when the action is 4, else 0x101.
pub fn fe_state_net_news_on_widget_notify(c: &mut Cpu) {
    c.cycles += 4;
    let key = c.g[A2][0] ^ 4;
    c.g[A2][0] = key;
    c.g[V0][0] = 0x101;
    if key == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `int cFEPopupScreenPos_v23(void* self, int which)` (0x1df028): the popup's
/// layout words — 0 → the word at `+0x48`, 1 → `+0x4c`, anything else the
/// signed byte at `+0x50`.
pub fn fe_popup_screen_pos_v23(c: &mut Cpu) {
    c.cycles += 2;
    let which = c.g[A1][0];
    c.g[V0][0] = 1;
    let this = abi::arg(c, 0);
    if which == 0 {
        c.cycles += 2;
        c.g[V0][0] = Addr(this.wrapping_add(0x48)).lw(c);
    } else {
        c.cycles += 2;
        // v0 still holds the 1 the `switch` compares against.
        if which == c.g[V0][0] {
            c.cycles += 2;
            c.g[V0][0] = Addr(this.wrapping_add(0x4c)).lw(c);
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(this.wrapping_add(0x50)).lb(c);
        }
    }
    abi::ret(c);
}

/// `int cFEStateRewardGalleryPosters_GetItemPrice(void* self, void* item)`
/// (0x1dfb98): see [`gallery_item_price`].
pub fn fe_state_reward_gallery_posters_get_item_price(c: &mut Cpu) {
    gallery_item_price(c);
}

/// `..._TradingCards_GetItemPrice` (0x1dfc68): see [`gallery_item_price`].
pub fn fe_state_reward_gallery_trading_cards_get_item_price(c: &mut Cpu) {
    gallery_item_price(c);
}

/// `..._ConceptArts_GetItemPrice` (0x1dfd38): see [`gallery_item_price`].
pub fn fe_state_reward_gallery_concept_arts_get_item_price(c: &mut Cpu) {
    gallery_item_price(c);
}

/// `..._Videos_GetItemPrice` (0x1dfe08): see [`gallery_item_price`].
pub fn fe_state_reward_gallery_videos_get_item_price(c: &mut Cpu) {
    gallery_item_price(c);
}

/// `..._Toys_GetItemPrice` (0x1dfed8): see [`gallery_item_price`].
pub fn fe_state_reward_gallery_toys_get_item_price(c: &mut Cpu) {
    gallery_item_price(c);
}

/// `..._CheatChars_GetItemPrice` (0x1dffa8): see [`gallery_item_price`].
pub fn fe_state_reward_gallery_cheat_chars_get_item_price(c: &mut Cpu) {
    gallery_item_price(c);
}

// ---------------------------------------------------------------------------
// Overworld screens.

/// Popup entry table (`D_004C6C08`), stride 0x18.
const POPUP_ENTRIES: u32 = 0x004c_6c08;
/// The per-entry target table (`D_00441630`), stride 0x14.
const POPUP_TARGETS: u32 = 0x0044_1630;

/// `int cOVState_ViewPopup_OnWidgetNotify(void* self, int widget, int action)`
/// (0x1e5a18): the view popup's notify handler —
/// action 8 looks up `D_004C6C08[index]` (index at `self+0x68`) and returns
/// whether the word through `D_00441630[entry->+0x4]` is not 4; actions below
/// 6 give 0x101, 6/7 give 1, 9 gives 0, 10 and up give 0x101.
pub fn ov_state_view_popup_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    let action = c.g[A2][0];
    c.g[V0][0] = 8;
    if action == 8 {
        let this = abi::arg(c, 0);
        let index = Addr(this.wrapping_add(0x68)).lw(c);
        c.g[A0][0] = index;
        c.cycles += 15;
        c.g[V0][0] = 0x18;
        c.g[A1][0] = 0x4c_0000;
        c.g[A2][0] = 0x14;
        let entry_off = abi::mult(c, index as u32, 0x18);
        c.g[A0][0] = sx(entry_off);
        let entries = 0x4c_0000u32.wrapping_add(0x6c08);
        c.g[A1][0] = sx(entries);
        c.g[V0][0] = 0x44_0000;
        let targets = 0x44_0000u32.wrapping_add(0x1630);
        c.g[V0][0] = sx(targets);
        let entry = entry_off.wrapping_add(entries);
        c.g[A0][0] = sx(entry);
        let field = Addr(entry.wrapping_add(4)).lw(c);
        c.g[V1][0] = field;
        let field_off = abi::mult(c, field as u32, 0x14);
        c.g[V1][0] = sx(field_off);
        let target = field_off.wrapping_add(targets);
        c.g[V1][0] = sx(target);
        let word = Addr(target).lw(c);
        c.g[V0][0] = word;
        c.g[V0][0] = word ^ 4;
        c.g[V0][0] = (c.g[V0][0] != 0) as u64;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    let below_nine = action < 9;
    c.g[V0][0] = below_nine as u64;
    let below_six = action < 6;
    c.g[V0][0] = below_six as u64;
    if below_nine {
        c.cycles += 2;
        if below_six {
            c.g[V0][0] = 0x101;
        } else {
            c.g[V0][0] = 1;
        }
        c.cycles += 2;
        abi::ret(c);
    } else {
        c.cycles += 3;
        c.g[V0][0] = 9;
        let is_nine = action == c.g[V0][0];
        c.g[V0][0] = 0x101;
        if is_nine {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
        } else {
            c.cycles += 2;
            c.cycles += 2;
            abi::ret(c);
        }
    }
}

/// `int OVScreenFreestyleStandings_OnWidgetNotify(...)` (0x1e6630): see
/// [`ov_screen_notify_action_six`].
pub fn ov_screen_freestyle_standings_on_widget_notify(c: &mut Cpu) {
    ov_screen_notify_action_six(c);
}

/// `int OVScreenFreestyleStandings_OnWidgetAction(void* self, void* menu, int action)`
/// (0x1e6640): see [`ov_screen_action_row`].
pub fn ov_screen_freestyle_standings_on_widget_action(c: &mut Cpu) {
    ov_screen_action_row(c);
}

/// `int OVScreenFinalStandings_OnWidgetNotify(...)` (0x1e7b78): see
/// [`ov_screen_notify_action_six`].
pub fn ov_screen_final_standings_on_widget_notify(c: &mut Cpu) {
    ov_screen_notify_action_six(c);
}

/// `int OVScreenFinalStandings_OnWidgetAction(...)` (0x1e7b88): see
/// [`ov_screen_action_row`].
pub fn ov_screen_final_standings_on_widget_action(c: &mut Cpu) {
    ov_screen_action_row(c);
}

/// `int OVScreenFinishOv_OnEvent(void* self, int event)` (0x1e81d0): see
/// [`ov_screen_finish_on_event`].
pub fn ov_screen_finish_ov_on_event(c: &mut Cpu) {
    ov_screen_finish_on_event(c);
}

/// `int OVScreenFinishOv2_OnEvent(void* self, int event)` (0x1e88f0): see
/// [`ov_screen_finish_on_event`].
pub fn ov_screen_finish_ov2_on_event(c: &mut Cpu) {
    ov_screen_finish_on_event(c);
}

/// `int OVScreenPeakChallengeResults_OnWidgetNotify(...)` (0x1e8e98): see
/// [`ov_screen_notify_action_six`].
pub fn ov_screen_peak_challenge_results_on_widget_notify(c: &mut Cpu) {
    ov_screen_notify_action_six(c);
}

/// `int OVScreenPeakChallengeResults_OnWidgetAction(...)` (0x1e8ea8): see
/// [`ov_screen_action_row`].
pub fn ov_screen_peak_challenge_results_on_widget_action(c: &mut Cpu) {
    ov_screen_action_row(c);
}

/// `int OVScreenMultiplayerStandings_OnWidgetAction(...)` (0x1e8fc8): see
/// [`ov_screen_action_row`].
pub fn ov_screen_multiplayer_standings_on_widget_action(c: &mut Cpu) {
    ov_screen_action_row(c);
}

// ---------------------------------------------------------------------------
// Processes, rail modifiers and anim objects.

/// `int GameProcess_v11(GameProcess* this)` (0x360768): a process vtable slot
/// that always returns -1 (no game process).
pub fn game_process_v11(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.g[V0][0] = c.g[V0][0] | 0xffff;
    abi::ret(c);
}

/// `int TypedProcess_IsType(TypedProcess* this, int type)` (0x3607a0):
/// whether the process's type tag (`this+0x10`, a signed short) equals `type`.
pub fn typed_process_is_type(c: &mut Cpu) {
    c.cycles += 4;
    let tag = Addr(abi::arg(c, 0)).off(0x10).lh(c);
    c.g[V0][0] = tag ^ c.g[A1][0];
    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    abi::ret(c);
}

/// `void ModObj_v51(ModObj* this)` (0x360dd8): clear the low flag bit (bit 0)
/// of the 16-bit word at `+0x12`.
pub fn mod_obj_v51(c: &mut Cpu) {
    c.cycles += 4;
    let flags = Addr(abi::arg(c, 0)).off(0x12);
    let v = flags.lhu(c) & 0xfffe;
    c.g[V0][0] = v;
    flags.sh(c, v);
    abi::ret(c);
}

/// `float AnimObject_v30(AnimObject* this)` (0x361068): the anim object's
/// input at `+0x0` scaled by 30 (the literal is materialised through `at`).
pub fn anim_object_v30(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    let input = ldf(c, 1, this);
    let scale = li_f(c, 0, 0x41f0_0000); // 30.0f
    c.f[0] = (input * scale).0;
    abi::ret(c);
}

/// `float AnimObject_v53(AnimObject* this)` (0x361158): the "finished" word
/// at `+0x8` in v0; f0 = the time at `+0x10` when that word is zero, else 0.
pub fn anim_object_v53(c: &mut Cpu) {
    c.cycles += 4;
    let this = abi::arg(c, 0);
    let finished = Addr(this.wrapping_add(8)).lw(c);
    c.g[V0][0] = finished;
    c.f[0] = 0;
    if finished == 0 {
        c.f[0] = Addr(this.wrapping_add(0x10)).read_fpu(c).0;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int AnimObject_v31(AnimObject* this)` (0x361170): 1 when the time at
/// `+0x10` is >= 0, else -1.
pub fn anim_object_v31(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0);
    let x = ldf(c, 1, this.wrapping_add(0x10));
    c.g[V0][0] = u64::MAX;
    if non_negative(c, x) {
        c.cycles += 1;
        c.g[V0][0] = 1;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int AnimTeeter_v31(AnimTeeter* this)` (0x361460): 1 when the time at
/// `+0x4` is >= 0, else -1.
pub fn anim_teeter_v31(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0);
    let x = ldf(c, 1, this.wrapping_add(4));
    c.g[V0][0] = u64::MAX;
    if non_negative(c, x) {
        c.cycles += 1;
        c.g[V0][0] = 1;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int FloatRail_v49(FloatRail* this, int index)` (0x3615a8): the rail
/// entry `(*(this+0x70))[index]`'s field at `+0x44` (stride 0x50).
pub fn float_rail_v49(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0);
    let index = c.g[A1][0] as u32;
    let entries = Addr(this.wrapping_add(0x70)).lw(c);
    c.g[V1][0] = entries;
    c.g[V0][0] = 0x50;
    let entry_off = abi::mult(c, index, 0x50);
    c.g[A1][0] = sx(entry_off);
    let entry = entry_off.wrapping_add(entries as u32);
    c.g[A1][0] = sx(entry);
    c.g[V0][0] = Addr(entry.wrapping_add(0x44)).lw(c);
    abi::ret(c);
}

/// `int SpringRail_v49(SpringRail* this, int index)` (0x3615c0): the rail
/// entry's field at `+0x54` (stride 0x60).
pub fn spring_rail_v49(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0);
    let index = c.g[A1][0] as u32;
    let entries = Addr(this.wrapping_add(0x70)).lw(c);
    c.g[V1][0] = entries;
    c.g[V0][0] = 0x60;
    let entry_off = abi::mult(c, index, 0x60);
    c.g[A1][0] = sx(entry_off);
    let entry = entry_off.wrapping_add(entries as u32);
    c.g[A1][0] = sx(entry);
    c.g[V0][0] = Addr(entry.wrapping_add(0x54)).lw(c);
    abi::ret(c);
}

/// One 128-bit transform copy: `sq` the two quadwords at `src` to `dst` and
/// `dst+0x10` (through GPRs 2 and 3, as the original does, so v0/v1 end as
/// the copied halves).
fn copy_transform_bounds(c: &mut Cpu, dst: u32, src: u32) {
    c.lq(2, src);
    c.sq(2, dst.wrapping_add(0x10));
    c.lq(3, src.wrapping_add(0x10));
    c.sq(3, dst.wrapping_add(0x20));
}

/// `void PositionModifier_SetTransformBounds(PositionModifier* this, void* xf)`
/// (0x361918): copy the transform into `this+0x10`/`+0x20`.
pub fn position_modifier_set_transform_bounds(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    copy_transform_bounds(c, this, src);
    abi::ret(c);
}

/// `void MagnetModifier_SetTransformBounds(...)` (0x361a30): copy into
/// `this+0x20`/`+0x30`.
pub fn magnet_modifier_set_transform_bounds(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    c.lq(2, src);
    c.sq(2, this.wrapping_add(0x20));
    c.lq(3, src.wrapping_add(0x10));
    c.sq(3, this.wrapping_add(0x30));
    abi::ret(c);
}

/// `void AvaSplineModifier_SetTransformBounds(...)` (0x361ae0): copy into
/// `this+0x10`/`+0x20`.
pub fn ava_spline_modifier_set_transform_bounds(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    copy_transform_bounds(c, this, src);
    abi::ret(c);
}

/// `void SplineModifier_SetTransformBounds(...)` (0x361b60): copy into
/// `this+0x10`/`+0x20`.
pub fn spline_modifier_set_transform_bounds(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    copy_transform_bounds(c, this, src);
    abi::ret(c);
}

/// `void MultiSplineModifier_SetTransformBounds(MultiSplineModifier* this, void* xf)`
/// (0x361bf0): copy the transform into the sub-object at `*(this+0x3c)`
/// (`+0x30`/`+0x40`).
pub fn multi_spline_modifier_set_transform_bounds(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    let sub = Addr(this.wrapping_add(0x3c)).lw(c);
    c.g[A2][0] = sub;
    let sub = sub as u32;
    c.lq(2, src);
    c.sq(2, sub.wrapping_add(0x30));
    c.lq(3, src.wrapping_add(0x10));
    c.sq(3, sub.wrapping_add(0x40));
    abi::ret(c);
}

/// `int RollerModifier_IsTransformFinished(RollerModifier* this)`
/// (0x361cb0): 1 when the distance at `+0x20` is >= 17, else 0.
pub fn roller_modifier_is_transform_finished(c: &mut Cpu) {
    c.cycles += 7;
    let this = abi::arg(c, 0);
    let x = ldf(c, 1, this.wrapping_add(0x20));
    let threshold = li_f(c, 0, 0x4188_0000); // 17.0f
    c.g[V0][0] = 1;
    if !fcc(c, threshold.le(x)) {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void RollerModifier_SetTransformBounds(...)` (0x361d00): copy into
/// `this+0x220`/`+0x230`.
pub fn roller_modifier_set_transform_bounds(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    c.lq(2, src);
    c.sq(2, this.wrapping_add(0x220));
    c.lq(3, src.wrapping_add(0x10));
    c.sq(3, this.wrapping_add(0x230));
    abi::ret(c);
}

/// `int RailMan_v11(RailMan* this)` (0x361d78): a rail-man vtable slot that
/// always returns -1.
pub fn rail_man_v11(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.g[V0][0] = c.g[V0][0] | 0xffff;
    abi::ret(c);
}

/// `int AnimCombo_v31(AnimCombo* this)` (0x361d90): 1 when the time chosen by
/// the flag byte at `+0x14` (`+0x28` when zero, `+0x0` otherwise) is >= 0,
/// else -1.
pub fn anim_combo_v31(c: &mut Cpu) {
    c.cycles += 3;
    let this = abi::arg(c, 0);
    let flag = Addr(this.wrapping_add(0x14)).lb(c);
    c.g[V0][0] = flag;
    if flag == 0 {
        let x = ldf(c, 1, this.wrapping_add(0x28));
        c.cycles += 5;
        c.g[V0][0] = u64::MAX;
        if non_negative(c, x) {
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
    } else {
        let x = ldf(c, 1, this);
        c.cycles += 6;
        c.g[V0][0] = u64::MAX;
        if non_negative(c, x) {
            c.cycles += 2;
            c.g[V0][0] = 1;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Network and script objects.

/// `void cSharedLoginMan_McCb_State0F(void)` (0x266348): the login state
/// machine jumps to state 0xf by storing it in `D_004A33D4` (`gp+0x2e4`).
pub fn shared_login_man_mc_cb_state_0f(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0xf;
    let state = c.g[V0][0];
    Addr(gp(c, 0x2e4)).sw(c, state);
    abi::ret(c);
}

/// `float cDSockChannel_Base_v05(void)` (0x267dd8): the channel's base tick
/// (1/60 s) from the constant at `gp-0x4a5c`.
pub fn dsock_channel_base_v05(c: &mut Cpu) {
    c.cycles += 3;
    c.f[0] = Addr(gp(c, -0x4a5c)).read_fpu(c).0;
    abi::ret(c);
}

/// `int cDSockChannel_IsConnected(cDSockChannel* this)` (0x267e00): whether
/// the channel state at `+0xc` is 2 (connected).
pub fn dsock_channel_is_connected(c: &mut Cpu) {
    c.cycles += 4;
    let state = Addr(abi::arg(c, 0)).off(0xc).lw(c);
    c.g[V0][0] = state ^ 2;
    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    abi::ret(c);
}

/// The pool base setter `func_003B5B20` (SND_Lock; SNDI_SpuMemSetBase;
/// SND_Unlock) that the original calls indirectly. Kept a table call so a
/// replaced callee is reached too.
const SNDI_POOL_SET_BASE: u32 = 0x003b_5b20;
/// The return address the original loads into `ra` for that call.
const POOL_SET_BASE_RET: u32 = 0x002a_927c;

/// `void PS2SoundManager_SetSpuMemBase(PS2SoundManager* this, int index)`
/// (0x2a9250): mark the manager (`this+0x31c = 1`) and hand the pool entry
/// `this[0xc + index*0x18]`'s two words to the SPU base setter.
pub fn ps2_sound_manager_set_spu_mem_base(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[V1][0] = 0x18;
        let frame = Frame::open(c, 0x10);
        let this = c.g[A0][0] as u32;
        let index = c.g[A1][0] as u32;
        let entry_off = abi::mult(c, index, 0x18);
        c.g[A1][0] = sx(entry_off);
        frame.save_ra(c, 0);
        c.g[V0][0] = 1;
        let one = c.g[V0][0];
        Addr(this.wrapping_add(0x31c)).sw(c, one);
        let entry = this.wrapping_add(entry_off.wrapping_add(0xc));
        c.g[A1][0] = sx(entry_off.wrapping_add(0xc));
        c.g[A0][0] = sx(entry);
        let flag = Addr(entry.wrapping_add(8)).lw(c);
        c.g[A1][0] = flag;
        let base = Addr(entry.wrapping_add(4)).lw(c);
        c.g[A0][0] = base;
        call_ptr(c, SNDI_POOL_SET_BASE, POOL_SET_BASE_RET)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int ScriptDummyObj_AcceptCommand(void* self, int cmd, int which)`
/// (0x27c040): the dummy script object accepts command 6.
pub fn script_dummy_obj_accept_command(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A2][0] ^ 6;
    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    abi::ret(c);
}

/// `int ScriptAudioObj_AcceptCommand(void* self, int cmd, int which)`
/// (0x2808d8): the audio script object accepts command 7.
pub fn script_audio_obj_accept_command(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A2][0] ^ 7;
    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    abi::ret(c);
}

/// `void* ScriptViewObj_GetBinding(ScriptViewObj* this, int index)`
/// (0x281170): `this+0x10` for index 0, else the binding `this + 0x8 +
/// index*0x14`.
pub fn script_view_obj_get_binding(c: &mut Cpu) {
    c.cycles += 2;
    let this = abi::arg(c, 0);
    let index = c.g[A1][0];
    c.g[V0][0] = 0x14;
    if index == 0 {
        c.cycles += 2;
        c.g[V0][0] = sx(this.wrapping_add(0x10));
    } else {
        c.cycles += 4;
        let index_off = abi::mult(c, index as u32, 0x14);
        c.g[V0][0] = sx(index_off);
        let binding_off = index_off.wrapping_add(8);
        c.g[V0][0] = sx(binding_off);
        c.g[V0][0] = sx(this.wrapping_add(binding_off));
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 300;

    fn rng(tag: u32, seed: u32) -> Rng {
        Rng(((tag as u64) << 20) ^ (seed as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15))
    }

    /// An action/key code covering the small ranges the handlers switch on.
    fn action_key(seed: u32, r: &mut Rng) -> u64 {
        match seed % 8 {
            0 => 0,
            1 => 3,
            2 => 5,
            3 => 6,
            4 => 7,
            5 => 8,
            6 => 9,
            _ => (r.next_u32() as u64) << 8 | (r.next_u32() % 4) as u64,
        }
    }

    #[test]
    fn fe_use_existing_account_on_widget_notify_matches() {
        check(
            ee::s03::f_001ac5a8__generated,
            fe_use_existing_account_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001a_c5a8, seed);
                scramble(c, &mut r);
                c.g[A2][0] = if seed % 2 == 0 {
                    9
                } else {
                    (r.next_u32() | 0x1000) as u64
                };
            },
        );
    }

    #[test]
    fn fe_state_challenge_popup_on_widget_notify_matches() {
        check(
            ee::s03::f_001ae100__generated,
            fe_state_challenge_popup_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001a_e100, seed);
                scramble(c, &mut r);
                c.g[A2][0] = action_key(seed, &mut r);
            },
        );
    }

    #[test]
    fn fe_state_challenge_received_popup_on_widget_notify_matches() {
        check(
            ee::s03::f_001b06e0__generated,
            fe_state_challenge_received_popup_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001b_06e0, seed);
                scramble(c, &mut r);
                c.g[A2][0] = action_key(seed, &mut r);
            },
        );
    }

    #[test]
    fn net_config_auth_popup_on_widget_notify_matches() {
        check(
            ee::s03::f_001b6a90__generated,
            net_config_auth_popup_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001b_6a90, seed);
                scramble(c, &mut r);
                c.g[A2][0] = action_key(seed, &mut r);
            },
        );
    }

    #[test]
    fn fe_state_auth_dnas_error_on_widget_notify_matches() {
        check(
            ee::s03::f_001b6ea8__generated,
            fe_state_auth_dnas_error_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001b_6ea8, seed);
                scramble(c, &mut r);
                c.g[A2][0] = action_key(seed, &mut r);
            },
        );
    }

    #[test]
    fn fe_state_online_agreement_on_widget_notify_matches() {
        check(
            ee::s03::f_001c0b08__generated,
            fe_state_online_agreement_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001c_0b08, seed);
                scramble(c, &mut r);
                c.g[A2][0] = if seed % 2 == 0 {
                    9
                } else {
                    (r.next_u32() | 0x1000) as u64
                };
            },
        );
    }

    #[test]
    fn fe_state_net_quick_play_popup_on_widget_notify_matches() {
        check(
            ee::s03::f_001c17e0__generated,
            fe_state_net_quick_play_popup_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001c_17e0, seed);
                scramble(c, &mut r);
                c.g[A2][0] = if seed % 2 == 0 {
                    9
                } else {
                    (r.next_u32() | 0x1000) as u64
                };
            },
        );
    }

    #[test]
    fn fe_state_net_news_on_widget_notify_matches() {
        check(
            ee::s03::f_001c5420__generated,
            fe_state_net_news_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001c_5420, seed);
                scramble(c, &mut r);
                c.g[A2][0] = if seed % 2 == 0 {
                    4
                } else {
                    (r.next_u32() | 0x1000) as u64
                };
            },
        );
    }

    #[test]
    fn fe_popup_screen_pos_v23_matches() {
        check(
            ee::s04::f_001df028__generated,
            fe_popup_screen_pos_v23,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001d_f028, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    _ => (r.next_u32() as u64) << 32 | (r.next_u32() % 8) as u64,
                };
                c.bus.write32(DATA + 0x48, r.next_u32());
                c.bus.write32(DATA + 0x4c, r.next_u32());
                c.bus.write8(DATA + 0x50, r.next_u32() as u8);
            },
        );
    }

    /// The item (`a1`): a signed owned-count short at `+0xc`.
    fn put_item(c: &mut Cpu, seed: u32, r: &mut Rng) -> u32 {
        let count = match seed % 5 {
            0 => 0,
            1 => -1 - (r.next_u32() % 40) as i32,
            2 => i16::MIN as i32,
            _ => (r.next_u32() % 300) as i32,
        };
        c.bus.write16(DATA + 0xc, count as u16);
        c.g[A0][0] = sx(DATA + 0x1000);
        c.g[A1][0] = sx(DATA);
        count as u32
    }

    #[test]
    fn fe_state_reward_gallery_posters_get_item_price_matches() {
        check_lohi(
            ee::s04::f_001dfb98__generated,
            fe_state_reward_gallery_posters_get_item_price,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001d_fb98, seed);
                scramble(c, &mut r);
                put_item(c, seed, &mut r);
            },
        );
    }

    #[test]
    fn fe_state_reward_gallery_trading_cards_get_item_price_matches() {
        check_lohi(
            ee::s04::f_001dfc68__generated,
            fe_state_reward_gallery_trading_cards_get_item_price,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001d_fc68, seed);
                scramble(c, &mut r);
                put_item(c, seed, &mut r);
            },
        );
    }

    #[test]
    fn fe_state_reward_gallery_concept_arts_get_item_price_matches() {
        check_lohi(
            ee::s04::f_001dfd38__generated,
            fe_state_reward_gallery_concept_arts_get_item_price,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001d_fd38, seed);
                scramble(c, &mut r);
                put_item(c, seed, &mut r);
            },
        );
    }

    #[test]
    fn fe_state_reward_gallery_videos_get_item_price_matches() {
        check_lohi(
            ee::s04::f_001dfe08__generated,
            fe_state_reward_gallery_videos_get_item_price,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001d_fe08, seed);
                scramble(c, &mut r);
                put_item(c, seed, &mut r);
            },
        );
    }

    #[test]
    fn fe_state_reward_gallery_toys_get_item_price_matches() {
        check_lohi(
            ee::s04::f_001dfed8__generated,
            fe_state_reward_gallery_toys_get_item_price,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001d_fed8, seed);
                scramble(c, &mut r);
                put_item(c, seed, &mut r);
            },
        );
    }

    #[test]
    fn fe_state_reward_gallery_cheat_chars_get_item_price_matches() {
        check_lohi(
            ee::s04::f_001dffa8__generated,
            fe_state_reward_gallery_cheat_chars_get_item_price,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001d_ffa8, seed);
                scramble(c, &mut r);
                put_item(c, seed, &mut r);
            },
        );
    }

    #[test]
    fn ov_state_view_popup_on_widget_notify_matches() {
        check_lohi(
            ee::s04::f_001e5a18__generated,
            ov_state_view_popup_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001e_5a18, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A2][0] = match seed % 6 {
                    0 => 5,
                    1 => 7,
                    2 => 8,
                    3 => 9,
                    4 => 12,
                    _ => r.next_u32() as u64,
                };
                // The popup entry table and its per-entry target table.
                let index = (seed % 3) as u32;
                let field = ((seed / 3) % 3) as u32;
                let word = if seed % 2 == 0 { 4 } else { 7 };
                c.bus.write32(DATA + 0x68, index);
                c.bus.write32(POPUP_ENTRIES + index * 0x18 + 4, field);
                c.bus.write32(POPUP_TARGETS + field * 0x14, word);
            },
        );
    }

    #[test]
    fn ov_screen_freestyle_standings_on_widget_notify_matches() {
        check(
            ee::s04::f_001e6630__generated,
            ov_screen_freestyle_standings_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001e_6630, seed);
                scramble(c, &mut r);
                c.g[A2][0] = if seed % 2 == 0 {
                    6
                } else {
                    (r.next_u32() | 0x1000) as u64
                };
            },
        );
    }

    /// The menu (`a1`) and action (`a2`) of an `OnWidgetAction` handler.
    fn put_menu(c: &mut Cpu, seed: u32, r: &mut Rng) {
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(DATA + 0x18, r.next_u32());
        let picked = gp(c, -0x9f4);
        let row_global = gp(c, -0x9f0);
        c.bus.write32(picked, r.next_u32());
        c.bus.write32(row_global, r.next_u32());
        c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA) };
        c.g[A2][0] = match seed % 4 {
            0 => 5,
            1 => 0,
            2 => 4,
            _ => r.next_u32() as u64,
        };
    }

    #[test]
    fn ov_screen_freestyle_standings_on_widget_action_matches() {
        check(
            ee::s04::f_001e6640__generated,
            ov_screen_freestyle_standings_on_widget_action,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001e_6640, seed);
                scramble(c, &mut r);
                put_menu(c, seed, &mut r);
            },
        );
    }

    #[test]
    fn ov_screen_final_standings_on_widget_notify_matches() {
        check(
            ee::s04::f_001e7b78__generated,
            ov_screen_final_standings_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001e_7b78, seed);
                scramble(c, &mut r);
                c.g[A2][0] = if seed % 2 == 0 {
                    6
                } else {
                    (r.next_u32() | 0x1000) as u64
                };
            },
        );
    }

    #[test]
    fn ov_screen_final_standings_on_widget_action_matches() {
        check(
            ee::s04::f_001e7b88__generated,
            ov_screen_final_standings_on_widget_action,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001e_7b88, seed);
                scramble(c, &mut r);
                put_menu(c, seed, &mut r);
            },
        );
    }

    /// The event (`a1`) of a finish-screen `OnEvent` handler.
    fn put_event(c: &mut Cpu, seed: u32, r: &mut Rng) {
        c.g[A0][0] = sx(DATA);
        c.bus.write32(DATA + 0xa0, r.next_u32());
        c.g[A1][0] = match seed % 4 {
            0 => 0,
            1 => 1,
            2 => 2,
            _ => r.next_u32() as u64,
        };
    }

    #[test]
    fn ov_screen_finish_ov_on_event_matches() {
        check(
            ee::s04::f_001e81d0__generated,
            ov_screen_finish_ov_on_event,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001e_81d0, seed);
                scramble(c, &mut r);
                put_event(c, seed, &mut r);
            },
        );
    }

    #[test]
    fn ov_screen_finish_ov2_on_event_matches() {
        check(
            ee::s04::f_001e88f0__generated,
            ov_screen_finish_ov2_on_event,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001e_88f0, seed);
                scramble(c, &mut r);
                put_event(c, seed, &mut r);
            },
        );
    }

    #[test]
    fn ov_screen_peak_challenge_results_on_widget_notify_matches() {
        check(
            ee::s04::f_001e8e98__generated,
            ov_screen_peak_challenge_results_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001e_8e98, seed);
                scramble(c, &mut r);
                c.g[A2][0] = if seed % 2 == 0 {
                    6
                } else {
                    (r.next_u32() | 0x1000) as u64
                };
            },
        );
    }

    #[test]
    fn ov_screen_peak_challenge_results_on_widget_action_matches() {
        check(
            ee::s04::f_001e8ea8__generated,
            ov_screen_peak_challenge_results_on_widget_action,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001e_8ea8, seed);
                scramble(c, &mut r);
                put_menu(c, seed, &mut r);
            },
        );
    }

    #[test]
    fn ov_screen_multiplayer_standings_on_widget_action_matches() {
        check(
            ee::s04::f_001e8fc8__generated,
            ov_screen_multiplayer_standings_on_widget_action,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001e_8fc8, seed);
                scramble(c, &mut r);
                put_menu(c, seed, &mut r);
            },
        );
    }

    #[test]
    fn game_process_v11_matches() {
        check(ee::s12::f_00360768__generated, game_process_v11, SEEDS, &|c, seed| {
            let mut r = rng(0x0036_0768, seed);
            scramble(c, &mut r);
        });
    }

    #[test]
    fn typed_process_is_type_matches() {
        check(
            ee::s12::f_003607a0__generated,
            typed_process_is_type,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0036_07a0, seed);
                scramble(c, &mut r);
                let tag = (r.next_u32() & 0xffff) as u16;
                c.bus.write16(DATA + 0x10, tag);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 3 == 0 {
                    sx(tag as i16 as i32 as u32)
                } else {
                    (r.next_u32() as u64) << 32 | r.next_u32() as u64
                };
            },
        );
    }

    #[test]
    fn mod_obj_v51_matches() {
        check(ee::s12::f_00360dd8__generated, mod_obj_v51, SEEDS, &|c, seed| {
            let mut r = rng(0x0036_0dd8, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.bus.write16(DATA + 0x12, r.next_u32() as u16);
        });
    }

    #[test]
    fn anim_object_v30_matches() {
        check(ee::s12::f_00361068__generated, anim_object_v30, SEEDS, &|c, seed| {
            let mut r = rng(0x0036_1068, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.bus.write32(
                DATA,
                if seed % 2 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(100.0)
                },
            );
        });
    }

    #[test]
    fn anim_object_v53_matches() {
        check(ee::s12::f_00361158__generated, anim_object_v53, SEEDS, &|c, seed| {
            let mut r = rng(0x0036_1158, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.bus
                .write32(DATA + 8, if seed % 3 == 0 { 0 } else { r.next_u32() });
            c.bus.write32(DATA + 0x10, r.edge_f32());
        });
    }

    /// A float that exercises the `>= 0.0` compares: ±0, denormals, the
    /// largest exponent and ordinary values.
    fn put_float(c: &mut Cpu, seed: u32, addr: u32, r: &mut Rng) {
        let bits = match seed % 5 {
            0 => 0,
            1 => 0x8000_0000,
            2 => 0x7f80_0000,
            3 => 0x0000_0001,
            _ => r.edge_f32(),
        };
        c.bus.write32(addr, bits);
    }

    #[test]
    fn anim_object_v31_matches() {
        check(ee::s12::f_00361170__generated, anim_object_v31, SEEDS, &|c, seed| {
            let mut r = rng(0x0036_1170, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            put_float(c, seed, DATA + 0x10, &mut r);
        });
    }

    #[test]
    fn anim_teeter_v31_matches() {
        check(ee::s12::f_00361460__generated, anim_teeter_v31, SEEDS, &|c, seed| {
            let mut r = rng(0x0036_1460, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            put_float(c, seed, DATA + 4, &mut r);
        });
    }

    #[test]
    fn float_rail_v49_matches() {
        check_lohi(ee::s12::f_003615a8__generated, float_rail_v49, SEEDS, &|c, seed| {
            let mut r = rng(0x0036_15a8, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 4) as u64;
            c.bus.write32(DATA + 0x70, DATA + 0x5000);
            for i in 0..4u32 {
                c.bus.write32(DATA + 0x5000 + i * 0x50 + 0x44, r.next_u32());
            }
        });
    }

    #[test]
    fn spring_rail_v49_matches() {
        check_lohi(ee::s12::f_003615c0__generated, spring_rail_v49, SEEDS, &|c, seed| {
            let mut r = rng(0x0036_15c0, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 4) as u64;
            c.bus.write32(DATA + 0x70, DATA + 0x5000);
            for i in 0..4u32 {
                c.bus.write32(DATA + 0x5000 + i * 0x60 + 0x54, r.next_u32());
            }
        });
    }

    /// The transform source (`a1`) and the object (`a0`) of a
    /// `SetTransformBounds` copy.
    fn put_transform(c: &mut Cpu, r: &mut Rng, dest: u32, sub: bool) {
        let src = DATA + 0x5000;
        for i in 0..0x30u32 {
            c.bus.write32(src + i * 4, r.next_u32());
            c.bus.write32(DATA + dest + i * 4, r.next_u32());
        }
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(src);
        if sub {
            c.bus.write32(DATA + 0x3c, DATA + 0x6000);
        }
    }

    #[test]
    fn position_modifier_set_transform_bounds_matches() {
        check(
            ee::s12::f_00361918__generated,
            position_modifier_set_transform_bounds,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0036_1918, seed);
                scramble(c, &mut r);
                put_transform(c, &mut r, 0x10, false);
            },
        );
    }

    #[test]
    fn magnet_modifier_set_transform_bounds_matches() {
        check(
            ee::s12::f_00361a30__generated,
            magnet_modifier_set_transform_bounds,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0036_1a30, seed);
                scramble(c, &mut r);
                put_transform(c, &mut r, 0x20, false);
            },
        );
    }

    #[test]
    fn ava_spline_modifier_set_transform_bounds_matches() {
        check(
            ee::s12::f_00361ae0__generated,
            ava_spline_modifier_set_transform_bounds,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0036_1ae0, seed);
                scramble(c, &mut r);
                put_transform(c, &mut r, 0x10, false);
            },
        );
    }

    #[test]
    fn spline_modifier_set_transform_bounds_matches() {
        check(
            ee::s12::f_00361b60__generated,
            spline_modifier_set_transform_bounds,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0036_1b60, seed);
                scramble(c, &mut r);
                put_transform(c, &mut r, 0x10, false);
            },
        );
    }

    #[test]
    fn multi_spline_modifier_set_transform_bounds_matches() {
        check(
            ee::s12::f_00361bf0__generated,
            multi_spline_modifier_set_transform_bounds,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0036_1bf0, seed);
                scramble(c, &mut r);
                put_transform(c, &mut r, 0x30, true);
            },
        );
    }

    #[test]
    fn roller_modifier_is_transform_finished_matches() {
        check(
            ee::s12::f_00361cb0__generated,
            roller_modifier_is_transform_finished,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0036_1cb0, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let bits = match seed % 4 {
                    0 => 0x4188_0000, // 17.0
                    1 => 0x4188_0001,
                    2 => 0x4187_ffff,
                    _ => r.edge_f32(),
                };
                c.bus.write32(DATA + 0x20, bits);
            },
        );
    }

    #[test]
    fn roller_modifier_set_transform_bounds_matches() {
        check(
            ee::s12::f_00361d00__generated,
            roller_modifier_set_transform_bounds,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0036_1d00, seed);
                scramble(c, &mut r);
                put_transform(c, &mut r, 0x220, false);
            },
        );
    }

    #[test]
    fn rail_man_v11_matches() {
        check(ee::s12::f_00361d78__generated, rail_man_v11, SEEDS, &|c, seed| {
            let mut r = rng(0x0036_1d78, seed);
            scramble(c, &mut r);
        });
    }

    #[test]
    fn anim_combo_v31_matches() {
        check(ee::s12::f_00361d90__generated, anim_combo_v31, SEEDS, &|c, seed| {
            let mut r = rng(0x0036_1d90, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.bus.write8(
                DATA + 0x14,
                if seed % 3 == 0 { 0 } else { r.next_u32() as u8 },
            );
            put_float(c, seed, DATA, &mut r);
            put_float(c, seed / 2, DATA + 0x28, &mut r);
        });
    }

    #[test]
    fn shared_login_man_mc_cb_state_0f_matches() {
        check(
            ee::s07::f_00266348__generated,
            shared_login_man_mc_cb_state_0f,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0026_6348, seed);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let v = r.next_u32();
                let a = gp(c, 0x2e4);
                c.bus.write32(a, v);
            },
        );
    }

    #[test]
    fn dsock_channel_base_v05_matches() {
        check(
            ee::s07::f_00267dd8__generated,
            dsock_channel_base_v05,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0026_7dd8, seed);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let bits = if seed % 2 == 0 {
                    0x3c88_8889
                } else {
                    r.edge_f32()
                };
                let a = gp(c, -0x4a5c);
                c.bus.write32(a, bits);
            },
        );
    }

    #[test]
    fn dsock_channel_is_connected_matches() {
        check(
            ee::s07::f_00267e00__generated,
            dsock_channel_is_connected,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0026_7e00, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 0xc, if seed % 3 == 0 { 2 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn ps2_sound_manager_set_spu_mem_base_matches() {
        check_lohi(
            ee::s08::f_002a9250__generated,
            ps2_sound_manager_set_spu_mem_base,
            200,
            &|c, seed| {
                let mut r = rng(0x002a_9250, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                // The pool entries at +0xc, stride 0x18: unk4 (the new SPU
                // base) and unk8 feed the setter; keep them small so its
                // range check takes the short path.
                for i in 0..4u32 {
                    c.bus.write32(DATA + 0xc + i * 0x18 + 4, (seed + i) % 0x200);
                    c.bus
                        .write32(DATA + 0xc + i * 0x18 + 8, r.next_u32() & 0xffff);
                }
                // SND_Lock / SND_Unlock call their hook through these
                // pointers; point both at a leaf that only returns.
                c.bus.write32(0x0044_c46c, 0x0036_0768);
                c.bus.write32(0x0044_c470, 0x0036_0768);
            },
        );
    }

    #[test]
    fn script_dummy_obj_accept_command_matches() {
        check(
            ee::s07::f_0027c040__generated,
            script_dummy_obj_accept_command,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0027_c040, seed);
                scramble(c, &mut r);
                c.g[A2][0] = if seed % 2 == 0 {
                    6
                } else {
                    (r.next_u32() | 0x1000) as u64
                };
            },
        );
    }

    #[test]
    fn script_audio_obj_accept_command_matches() {
        check(
            ee::s07::f_002808d8__generated,
            script_audio_obj_accept_command,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0028_08d8, seed);
                scramble(c, &mut r);
                c.g[A2][0] = if seed % 2 == 0 {
                    7
                } else {
                    (r.next_u32() | 0x1000) as u64
                };
            },
        );
    }

    #[test]
    fn script_view_obj_get_binding_matches() {
        check_lohi(
            ee::s07::f_00281170__generated,
            script_view_obj_get_binding,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0028_1170, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { (seed % 6) as u64 };
            },
        );
    }
}
