//! Staging module oc1505 (wave wave276): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1505`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001d_fd28,
        c_fe_state_reward_gallery_concept_arts_get_item_record_001dfd28,
        "cFEStateRewardGalleryConceptArts_GetItemRecord",
    ),
    (
        0x001d_fd50,
        c_fe_state_reward_gallery_concept_arts_get_item_field0_e_001dfd50,
        "cFEStateRewardGalleryConceptArts_GetItemField0E",
    ),
    (
        0x001d_fd58,
        c_fe_state_reward_gallery_concept_arts_get_item_name_001dfd58,
        "cFEStateRewardGalleryConceptArts_GetItemName",
    ),
    (
        0x001d_fdf8,
        c_fe_state_reward_gallery_videos_get_item_record_001dfdf8,
        "cFEStateRewardGalleryVideos_GetItemRecord",
    ),
    (
        0x001d_fe20,
        c_fe_state_reward_gallery_videos_get_item_field0_e_001dfe20,
        "cFEStateRewardGalleryVideos_GetItemField0E",
    ),
    (
        0x001d_fe28,
        c_fe_state_reward_gallery_videos_get_item_name_001dfe28,
        "cFEStateRewardGalleryVideos_GetItemName",
    ),
    (
        0x001d_fec8,
        c_fe_state_reward_gallery_toys_get_item_record_001dfec8,
        "cFEStateRewardGalleryToys_GetItemRecord",
    ),
    (
        0x001d_fef0,
        c_fe_state_reward_gallery_toys_get_item_field0_e_001dfef0,
        "cFEStateRewardGalleryToys_GetItemField0E",
    ),
    (
        0x001d_fef8,
        c_fe_state_reward_gallery_toys_get_item_name_001dfef8,
        "cFEStateRewardGalleryToys_GetItemName",
    ),
    (
        0x001d_ff98,
        c_fe_state_reward_gallery_cheat_chars_get_item_record_001dff98,
        "cFEStateRewardGalleryCheatChars_GetItemRecord",
    ),
    (
        0x001d_ffc0,
        c_fe_state_reward_gallery_cheat_chars_get_item_field0_e_001dffc0,
        "cFEStateRewardGalleryCheatChars_GetItemField0E",
    ),
    (
        0x001d_ffc8,
        c_fe_state_reward_gallery_cheat_chars_get_item_name_001dffc8,
        "cFEStateRewardGalleryCheatChars_GetItemName",
    ),
    (0x001e_0014, leaf_001e0014, "leaf_001e0014"),
    (
        0x001e_0020,
        c_fe_state_video_player_transition_in_001e0020,
        "cFEStateVideoPlayer_TransitionIn",
    ),
    (
        0x001e_0028,
        c_fe_state_video_player_transition_out_001e0028,
        "cFEStateVideoPlayer_TransitionOut",
    ),
    (
        0x001e_04f8,
        c_fe_popup_select_multiplayer_mode_v23_001e04f8,
        "cFEPopupSelectMultiplayerMode_v23",
    ),
    (
        0x001e_0500,
        c_fe_popup_select_multiplayer_mode_transition_in_001e0500,
        "cFEPopupSelectMultiplayerMode_TransitionIn",
    ),
    (
        0x001e_0508,
        c_fe_popup_select_multiplayer_mode_transition_out_001e0508,
        "cFEPopupSelectMultiplayerMode_TransitionOut",
    ),
    (0x001e_0770, feui_sound_v03_001e0770, "FEUISound_v03"),
    (0x001e_0778, feui_sound_v04_001e0778, "FEUISound_v04"),
    (0x001e_07b0, ovui_sound_v03_001e07b0, "OVUISound_v03"),
    (0x001e_07b8, ovui_sound_v04_001e07b8, "OVUISound_v04"),
    (
        0x001e_08d0,
        c_fe_use_existing_account_v29_001e08d0,
        "cFEUseExistingAccount_v29",
    ),
    (
        0x001e_0940,
        c_fe_state_select_persona_v29_001e0940,
        "cFEStateSelectPersona_v29",
    ),
    (
        0x001e_0948,
        c_fe_state_select_persona_v32_001e0948,
        "cFEStateSelectPersona_v32",
    ),
    (
        0x001e_0a00,
        c_fe_state_challenge_popup_transition_out_001e0a00,
        "cFEStateChallengePopup_TransitionOut",
    ),
    (
        0x001e_0ab8,
        c_fe_state_challenge_received_popup_transition_out_001e0ab8,
        "cFEStateChallengeReceivedPopup_TransitionOut",
    ),
    (
        0x001e_0ba0,
        c_fe_state_net_config_v32_001e0ba0,
        "cFEStateNetConfig_v32",
    ),
    (
        0x001e_0ba8,
        c_fe_state_net_config_v29_001e0ba8,
        "cFEStateNetConfig_v29",
    ),
    (
        0x001e_0c78,
        c_fe_state_auth_dnas_error_transition_out_001e0c78,
        "cFEStateAuthDNASError_TransitionOut",
    ),
    (
        0x001e_0c80,
        c_fe_state_auth_dnas_error_v32_001e0c80,
        "cFEStateAuthDNASError_v32",
    ),
    (
        0x001e_0c88,
        c_fe_state_auth_dnas_error_v29_001e0c88,
        "cFEStateAuthDNASError_v29",
    ),
    (
        0x001e_0c90,
        c_fe_state_net_account_mem_transition_in_001e0c90,
        "cFEStateNetAccountMem_TransitionIn",
    ),
    (
        0x001e_0c98,
        c_fe_state_net_account_mem_transition_out_001e0c98,
        "cFEStateNetAccountMem_TransitionOut",
    ),
    (
        0x001e_0d60,
        c_fe_state_create_step1_v29_001e0d60,
        "cFEStateCreateStep1_v29",
    ),
    (
        0x001e_0dd0,
        c_fe_state_create_step2_v29_001e0dd0,
        "cFEStateCreateStep2_v29",
    ),
    (
        0x001e_0e40,
        c_fe_state_create_duplicate_v29_001e0e40,
        "cFEStateCreateDuplicate_v29",
    ),
    (
        0x001e_0e88,
        c_fe_state_login_v32_001e0e88,
        "cFEStateLogin_v32",
    ),
    (
        0x001e_0e90,
        c_fe_state_login_v29_001e0e90,
        "cFEStateLogin_v29",
    ),
    (
        0x001e_0ec0,
        c_fe_state_stats_v13_001e0ec0,
        "cFEStateStats_v13",
    ),
    (
        0x001e_0ef0,
        c_fe_state_net_news_transition_out_001e0ef0,
        "cFEStateNetNews_TransitionOut",
    ),
    (
        0x001e_0f58,
        c_fe_state_relogin_transition_out_001e0f58,
        "cFEStateRelogin_TransitionOut",
    ),
    (
        0x001e_0fb8,
        c_fe_state_net_messenger_transition_out_001e0fb8,
        "cFEStateNetMessenger_TransitionOut",
    ),
    (
        0x001e_1040,
        c_fe_state_read_message_popup_transition_out_001e1040,
        "cFEStateReadMessagePopup_TransitionOut",
    ),
    (
        0x001e_1078,
        ui_sound_base_play_sound_001e1078,
        "UISoundBase_PlaySound",
    ),
];

/// `cFEStateRewardGalleryConceptArts_GetItemRecord` (0x001d_fd28): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_concept_arts_get_item_record_001dfd28(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa28));
    ret(c);
}

/// `cFEStateRewardGalleryConceptArts_GetItemField0E` (0x001d_fd50): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_concept_arts_get_item_field0_e_001dfd50(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0xe));
    ret(c);
}

/// `cFEStateRewardGalleryConceptArts_GetItemName` (0x001d_fd58): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_concept_arts_get_item_name_001dfd58(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A1][0] as u32));
    ret(c);
}

/// `cFEStateRewardGalleryVideos_GetItemRecord` (0x001d_fdf8): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_videos_get_item_record_001dfdf8(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa28));
    ret(c);
}

/// `cFEStateRewardGalleryVideos_GetItemField0E` (0x001d_fe20): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_videos_get_item_field0_e_001dfe20(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0xe));
    ret(c);
}

/// `cFEStateRewardGalleryVideos_GetItemName` (0x001d_fe28): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_videos_get_item_name_001dfe28(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A1][0] as u32));
    ret(c);
}

/// `cFEStateRewardGalleryToys_GetItemRecord` (0x001d_fec8): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_toys_get_item_record_001dfec8(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa28));
    ret(c);
}

/// `cFEStateRewardGalleryToys_GetItemField0E` (0x001d_fef0): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_toys_get_item_field0_e_001dfef0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0xe));
    ret(c);
}

/// `cFEStateRewardGalleryToys_GetItemName` (0x001d_fef8): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_toys_get_item_name_001dfef8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A1][0] as u32));
    ret(c);
}

/// `cFEStateRewardGalleryCheatChars_GetItemRecord` (0x001d_ff98): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_cheat_chars_get_item_record_001dff98(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa28));
    ret(c);
}

/// `cFEStateRewardGalleryCheatChars_GetItemField0E` (0x001d_ffc0): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_cheat_chars_get_item_field0_e_001dffc0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0xe));
    ret(c);
}

/// `cFEStateRewardGalleryCheatChars_GetItemName` (0x001d_ffc8): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_cheat_chars_get_item_name_001dffc8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A1][0] as u32));
    ret(c);
}

/// `leaf_001e0014` (0x001e_0014): straight-line leaf. Mechanical port of the generated body.
pub fn leaf_001e0014(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0u64;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEStateVideoPlayer_TransitionIn` (0x001e_0020): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_video_player_transition_in_001e0020(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEStateVideoPlayer_TransitionOut` (0x001e_0028): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_video_player_transition_out_001e0028(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEPopupSelectMultiplayerMode_v23` (0x001e_04f8): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_fe_popup_select_multiplayer_mode_v23_001e04f8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lb((c.g[A0][0] as u32).wrapping_add(0x48));
    ret(c);
}

/// `cFEPopupSelectMultiplayerMode_TransitionIn` (0x001e_0500): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_popup_select_multiplayer_mode_transition_in_001e0500(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEPopupSelectMultiplayerMode_TransitionOut` (0x001e_0508): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_popup_select_multiplayer_mode_transition_out_001e0508(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `FEUISound_v03` (0x001e_0770): straight-line leaf. Mechanical port of the generated body.
pub fn feui_sound_v03_001e0770(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `FEUISound_v04` (0x001e_0778): straight-line leaf. Mechanical port of the generated body.
pub fn feui_sound_v04_001e0778(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `OVUISound_v03` (0x001e_07b0): straight-line leaf. Mechanical port of the generated body.
pub fn ovui_sound_v03_001e07b0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `OVUISound_v04` (0x001e_07b8): straight-line leaf. Mechanical port of the generated body.
pub fn ovui_sound_v04_001e07b8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEUseExistingAccount_v29` (0x001e_08d0): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_use_existing_account_v29_001e08d0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cFEStateSelectPersona_v29` (0x001e_0940): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_select_persona_v29_001e0940(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cFEStateSelectPersona_v32` (0x001e_0948): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_select_persona_v32_001e0948(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateChallengePopup_TransitionOut` (0x001e_0a00): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_challenge_popup_transition_out_001e0a00(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEStateChallengeReceivedPopup_TransitionOut` (0x001e_0ab8): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_challenge_received_popup_transition_out_001e0ab8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEStateNetConfig_v32` (0x001e_0ba0): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_net_config_v32_001e0ba0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateNetConfig_v29` (0x001e_0ba8): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_net_config_v29_001e0ba8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cFEStateAuthDNASError_TransitionOut` (0x001e_0c78): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_auth_dnas_error_transition_out_001e0c78(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEStateAuthDNASError_v32` (0x001e_0c80): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_auth_dnas_error_v32_001e0c80(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateAuthDNASError_v29` (0x001e_0c88): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_auth_dnas_error_v29_001e0c88(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cFEStateNetAccountMem_TransitionIn` (0x001e_0c90): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_net_account_mem_transition_in_001e0c90(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEStateNetAccountMem_TransitionOut` (0x001e_0c98): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_net_account_mem_transition_out_001e0c98(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEStateCreateStep1_v29` (0x001e_0d60): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_create_step1_v29_001e0d60(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cFEStateCreateStep2_v29` (0x001e_0dd0): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_create_step2_v29_001e0dd0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cFEStateCreateDuplicate_v29` (0x001e_0e40): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_create_duplicate_v29_001e0e40(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cFEStateLogin_v32` (0x001e_0e88): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_login_v32_001e0e88(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateLogin_v29` (0x001e_0e90): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_login_v29_001e0e90(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cFEStateStats_v13` (0x001e_0ec0): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_stats_v13_001e0ec0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateNetNews_TransitionOut` (0x001e_0ef0): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_net_news_transition_out_001e0ef0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEStateRelogin_TransitionOut` (0x001e_0f58): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_relogin_transition_out_001e0f58(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEStateNetMessenger_TransitionOut` (0x001e_0fb8): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_net_messenger_transition_out_001e0fb8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEStateReadMessagePopup_TransitionOut` (0x001e_1040): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_read_message_popup_transition_out_001e1040(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `UISoundBase_PlaySound` (0x001e_1078): straight-line leaf. Mechanical port of the generated body.
pub fn ui_sound_base_play_sound_001e1078(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, check_lohi, Rng};

    /// An edge-case 64-bit register value (0, 1, -1, small, random).
    #[allow(dead_code)]
    fn leaf_val(r: &mut Rng) -> u64 {
        match r.next_u32() % 8 {
            0 => 0,
            1 => 1,
            2 => u64::MAX,
            3 => (r.next_u32() % 64) as u64,
            4 => sx(r.next_u32()),
            5 => r.next_u32() as u64,
            6 => 0x8000_0000,
            _ => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
        }
    }

    /// Fill `size` bytes at `start` with zero / small / random words, or, when
    /// `chase` is set, with pointers (16-byte aligned) into `ptr_lo..ptr_lo + 0x200`.
    #[allow(dead_code)]
    fn leaf_fill(c: &mut Cpu, r: &mut Rng, start: u32, size: u32, chase: bool, ptr_lo: u32) {
        for i in 0..size / 4 {
            let w = if chase {
                ptr_lo + 16 * (r.next_u32() % 32)
            } else {
                match r.next_u32() % 4 {
                    0 => 0,
                    1 => r.next_u32() % 16,
                    _ => r.next_u32(),
                }
            };
            c.bus.write32(start + 4 * i, w);
        }
    }

    /// `cFEStateRewardGalleryConceptArts_GetItemRecord` (0x001d_fd28) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_concept_arts_get_item_record_001dfd28_matches() {
        check(
            ee::s04::f_001dfd28__generated,
            c_fe_state_reward_gallery_concept_arts_get_item_record_001dfd28,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dfd28, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[A1][0] = (r.next_u32() % 8 * 4) as u64;
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryConceptArts_GetItemField0E` (0x001d_fd50) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_concept_arts_get_item_field0_e_001dfd50_matches() {
        check(
            ee::s04::f_001dfd50__generated,
            c_fe_state_reward_gallery_concept_arts_get_item_field0_e_001dfd50,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dfd50, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryConceptArts_GetItemName` (0x001d_fd58) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_concept_arts_get_item_name_001dfd58_matches() {
        check(
            ee::s04::f_001dfd58__generated,
            c_fe_state_reward_gallery_concept_arts_get_item_name_001dfd58,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dfd58, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryVideos_GetItemRecord` (0x001d_fdf8) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_videos_get_item_record_001dfdf8_matches() {
        check(
            ee::s04::f_001dfdf8__generated,
            c_fe_state_reward_gallery_videos_get_item_record_001dfdf8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dfdf8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[A1][0] = (r.next_u32() % 8 * 4) as u64;
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryVideos_GetItemField0E` (0x001d_fe20) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_videos_get_item_field0_e_001dfe20_matches() {
        check(
            ee::s04::f_001dfe20__generated,
            c_fe_state_reward_gallery_videos_get_item_field0_e_001dfe20,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dfe20, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryVideos_GetItemName` (0x001d_fe28) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_videos_get_item_name_001dfe28_matches() {
        check(
            ee::s04::f_001dfe28__generated,
            c_fe_state_reward_gallery_videos_get_item_name_001dfe28,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dfe28, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryToys_GetItemRecord` (0x001d_fec8) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_toys_get_item_record_001dfec8_matches() {
        check(
            ee::s04::f_001dfec8__generated,
            c_fe_state_reward_gallery_toys_get_item_record_001dfec8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dfec8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[A1][0] = (r.next_u32() % 8 * 4) as u64;
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryToys_GetItemField0E` (0x001d_fef0) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_toys_get_item_field0_e_001dfef0_matches() {
        check(
            ee::s04::f_001dfef0__generated,
            c_fe_state_reward_gallery_toys_get_item_field0_e_001dfef0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dfef0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryToys_GetItemName` (0x001d_fef8) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_toys_get_item_name_001dfef8_matches() {
        check(
            ee::s04::f_001dfef8__generated,
            c_fe_state_reward_gallery_toys_get_item_name_001dfef8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dfef8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryCheatChars_GetItemRecord` (0x001d_ff98) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_cheat_chars_get_item_record_001dff98_matches() {
        check(
            ee::s04::f_001dff98__generated,
            c_fe_state_reward_gallery_cheat_chars_get_item_record_001dff98,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dff98, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[A1][0] = (r.next_u32() % 8 * 4) as u64;
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryCheatChars_GetItemField0E` (0x001d_ffc0) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_cheat_chars_get_item_field0_e_001dffc0_matches() {
        check(
            ee::s04::f_001dffc0__generated,
            c_fe_state_reward_gallery_cheat_chars_get_item_field0_e_001dffc0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dffc0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryCheatChars_GetItemName` (0x001d_ffc8) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_cheat_chars_get_item_name_001dffc8_matches() {
        check(
            ee::s04::f_001dffc8__generated,
            c_fe_state_reward_gallery_cheat_chars_get_item_name_001dffc8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dffc8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `leaf_001e0014` (0x001e_0014) against the generated body.
    #[test]
    fn leaf_001e0014_matches() {
        check(
            ee::s04::f_001e0014__generated,
            leaf_001e0014,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0014, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateVideoPlayer_TransitionIn` (0x001e_0020) against the generated body.
    #[test]
    fn c_fe_state_video_player_transition_in_001e0020_matches() {
        check(
            ee::s04::f_001e0020__generated,
            c_fe_state_video_player_transition_in_001e0020,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0020, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateVideoPlayer_TransitionOut` (0x001e_0028) against the generated body.
    #[test]
    fn c_fe_state_video_player_transition_out_001e0028_matches() {
        check(
            ee::s04::f_001e0028__generated,
            c_fe_state_video_player_transition_out_001e0028,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0028, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEPopupSelectMultiplayerMode_v23` (0x001e_04f8) against the generated body.
    #[test]
    fn c_fe_popup_select_multiplayer_mode_v23_001e04f8_matches() {
        check(
            ee::s04::f_001e04f8__generated,
            c_fe_popup_select_multiplayer_mode_v23_001e04f8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1e04f8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEPopupSelectMultiplayerMode_TransitionIn` (0x001e_0500) against the generated body.
    #[test]
    fn c_fe_popup_select_multiplayer_mode_transition_in_001e0500_matches() {
        check(
            ee::s04::f_001e0500__generated,
            c_fe_popup_select_multiplayer_mode_transition_in_001e0500,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0500, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEPopupSelectMultiplayerMode_TransitionOut` (0x001e_0508) against the generated body.
    #[test]
    fn c_fe_popup_select_multiplayer_mode_transition_out_001e0508_matches() {
        check(
            ee::s04::f_001e0508__generated,
            c_fe_popup_select_multiplayer_mode_transition_out_001e0508,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0508, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `FEUISound_v03` (0x001e_0770) against the generated body.
    #[test]
    fn feui_sound_v03_001e0770_matches() {
        check(
            ee::s04::f_001e0770__generated,
            feui_sound_v03_001e0770,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0770, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `FEUISound_v04` (0x001e_0778) against the generated body.
    #[test]
    fn feui_sound_v04_001e0778_matches() {
        check(
            ee::s04::f_001e0778__generated,
            feui_sound_v04_001e0778,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0778, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `OVUISound_v03` (0x001e_07b0) against the generated body.
    #[test]
    fn ovui_sound_v03_001e07b0_matches() {
        check(
            ee::s04::f_001e07b0__generated,
            ovui_sound_v03_001e07b0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e07b0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `OVUISound_v04` (0x001e_07b8) against the generated body.
    #[test]
    fn ovui_sound_v04_001e07b8_matches() {
        check(
            ee::s04::f_001e07b8__generated,
            ovui_sound_v04_001e07b8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e07b8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEUseExistingAccount_v29` (0x001e_08d0) against the generated body.
    #[test]
    fn c_fe_use_existing_account_v29_001e08d0_matches() {
        check(
            ee::s04::f_001e08d0__generated,
            c_fe_use_existing_account_v29_001e08d0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e08d0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateSelectPersona_v29` (0x001e_0940) against the generated body.
    #[test]
    fn c_fe_state_select_persona_v29_001e0940_matches() {
        check(
            ee::s04::f_001e0940__generated,
            c_fe_state_select_persona_v29_001e0940,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0940, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateSelectPersona_v32` (0x001e_0948) against the generated body.
    #[test]
    fn c_fe_state_select_persona_v32_001e0948_matches() {
        check(
            ee::s04::f_001e0948__generated,
            c_fe_state_select_persona_v32_001e0948,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0948, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateChallengePopup_TransitionOut` (0x001e_0a00) against the generated body.
    #[test]
    fn c_fe_state_challenge_popup_transition_out_001e0a00_matches() {
        check(
            ee::s04::f_001e0a00__generated,
            c_fe_state_challenge_popup_transition_out_001e0a00,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0a00, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateChallengeReceivedPopup_TransitionOut` (0x001e_0ab8) against the generated body.
    #[test]
    fn c_fe_state_challenge_received_popup_transition_out_001e0ab8_matches() {
        check(
            ee::s04::f_001e0ab8__generated,
            c_fe_state_challenge_received_popup_transition_out_001e0ab8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0ab8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateNetConfig_v32` (0x001e_0ba0) against the generated body.
    #[test]
    fn c_fe_state_net_config_v32_001e0ba0_matches() {
        check(
            ee::s04::f_001e0ba0__generated,
            c_fe_state_net_config_v32_001e0ba0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0ba0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateNetConfig_v29` (0x001e_0ba8) against the generated body.
    #[test]
    fn c_fe_state_net_config_v29_001e0ba8_matches() {
        check(
            ee::s04::f_001e0ba8__generated,
            c_fe_state_net_config_v29_001e0ba8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0ba8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateAuthDNASError_TransitionOut` (0x001e_0c78) against the generated body.
    #[test]
    fn c_fe_state_auth_dnas_error_transition_out_001e0c78_matches() {
        check(
            ee::s04::f_001e0c78__generated,
            c_fe_state_auth_dnas_error_transition_out_001e0c78,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0c78, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateAuthDNASError_v32` (0x001e_0c80) against the generated body.
    #[test]
    fn c_fe_state_auth_dnas_error_v32_001e0c80_matches() {
        check(
            ee::s04::f_001e0c80__generated,
            c_fe_state_auth_dnas_error_v32_001e0c80,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0c80, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateAuthDNASError_v29` (0x001e_0c88) against the generated body.
    #[test]
    fn c_fe_state_auth_dnas_error_v29_001e0c88_matches() {
        check(
            ee::s04::f_001e0c88__generated,
            c_fe_state_auth_dnas_error_v29_001e0c88,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0c88, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateNetAccountMem_TransitionIn` (0x001e_0c90) against the generated body.
    #[test]
    fn c_fe_state_net_account_mem_transition_in_001e0c90_matches() {
        check(
            ee::s04::f_001e0c90__generated,
            c_fe_state_net_account_mem_transition_in_001e0c90,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0c90, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateNetAccountMem_TransitionOut` (0x001e_0c98) against the generated body.
    #[test]
    fn c_fe_state_net_account_mem_transition_out_001e0c98_matches() {
        check(
            ee::s04::f_001e0c98__generated,
            c_fe_state_net_account_mem_transition_out_001e0c98,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0c98, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateCreateStep1_v29` (0x001e_0d60) against the generated body.
    #[test]
    fn c_fe_state_create_step1_v29_001e0d60_matches() {
        check(
            ee::s04::f_001e0d60__generated,
            c_fe_state_create_step1_v29_001e0d60,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0d60, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateCreateStep2_v29` (0x001e_0dd0) against the generated body.
    #[test]
    fn c_fe_state_create_step2_v29_001e0dd0_matches() {
        check(
            ee::s04::f_001e0dd0__generated,
            c_fe_state_create_step2_v29_001e0dd0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0dd0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateCreateDuplicate_v29` (0x001e_0e40) against the generated body.
    #[test]
    fn c_fe_state_create_duplicate_v29_001e0e40_matches() {
        check(
            ee::s04::f_001e0e40__generated,
            c_fe_state_create_duplicate_v29_001e0e40,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0e40, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateLogin_v32` (0x001e_0e88) against the generated body.
    #[test]
    fn c_fe_state_login_v32_001e0e88_matches() {
        check(
            ee::s04::f_001e0e88__generated,
            c_fe_state_login_v32_001e0e88,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0e88, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateLogin_v29` (0x001e_0e90) against the generated body.
    #[test]
    fn c_fe_state_login_v29_001e0e90_matches() {
        check(
            ee::s04::f_001e0e90__generated,
            c_fe_state_login_v29_001e0e90,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0e90, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateStats_v13` (0x001e_0ec0) against the generated body.
    #[test]
    fn c_fe_state_stats_v13_001e0ec0_matches() {
        check(
            ee::s04::f_001e0ec0__generated,
            c_fe_state_stats_v13_001e0ec0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0ec0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateNetNews_TransitionOut` (0x001e_0ef0) against the generated body.
    #[test]
    fn c_fe_state_net_news_transition_out_001e0ef0_matches() {
        check(
            ee::s04::f_001e0ef0__generated,
            c_fe_state_net_news_transition_out_001e0ef0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0ef0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRelogin_TransitionOut` (0x001e_0f58) against the generated body.
    #[test]
    fn c_fe_state_relogin_transition_out_001e0f58_matches() {
        check(
            ee::s04::f_001e0f58__generated,
            c_fe_state_relogin_transition_out_001e0f58,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0f58, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateNetMessenger_TransitionOut` (0x001e_0fb8) against the generated body.
    #[test]
    fn c_fe_state_net_messenger_transition_out_001e0fb8_matches() {
        check(
            ee::s04::f_001e0fb8__generated,
            c_fe_state_net_messenger_transition_out_001e0fb8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e0fb8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateReadMessagePopup_TransitionOut` (0x001e_1040) against the generated body.
    #[test]
    fn c_fe_state_read_message_popup_transition_out_001e1040_matches() {
        check(
            ee::s04::f_001e1040__generated,
            c_fe_state_read_message_popup_transition_out_001e1040,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1040, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UISoundBase_PlaySound` (0x001e_1078) against the generated body.
    #[test]
    fn ui_sound_base_play_sound_001e1078_matches() {
        check(
            ee::s04::f_001e1078__generated,
            ui_sound_base_play_sound_001e1078,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1078, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
