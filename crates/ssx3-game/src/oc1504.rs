//! Staging module oc1504 (wave wave276): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1504`.

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
    (0x001d_c930, app_module_unload_001dc930, "AppModule_Unload"),
    (0x001d_c938, app_module_enter_001dc938, "AppModule_Enter"),
    (
        0x001d_cc38,
        fe_locale_set_field1_001dcc38,
        "FELocale_SetField1",
    ),
    (
        0x001d_cc58,
        fe_locale_v04_cc58_001dcc58,
        "FELocale_v04_cc58",
    ),
    (0x001d_cca8, feui_custom_v02_001dcca8, "FEUICustom_v02"),
    (0x001d_ccf8, ui_input_base_v39_001dccf8, "UIInputBase_v39"),
    (0x001d_d560, feui_input_v41_001dd560, "FEUIInput_v41"),
    (0x001d_d568, feui_input_v42_001dd568, "FEUIInput_v42"),
    (0x001d_d570, feui_input_v43_001dd570, "FEUIInput_v43"),
    (
        0x001d_d918,
        c_fe_state_uber_trick_v13_001dd918,
        "cFEStateUberTrick_v13",
    ),
    (
        0x001d_dfa8,
        c_fe_state_options_game_on_deactivate_001ddfa8,
        "cFEStateOptionsGame_OnDeactivate",
    ),
    (
        0x001d_e010,
        c_fe_state_options_sound_on_deactivate_001de010,
        "cFEStateOptionsSound_OnDeactivate",
    ),
    (
        0x001d_e078,
        c_fe_state_options_hud_on_deactivate_001de078,
        "cFEStateOptionsHUD_OnDeactivate",
    ),
    (
        0x001d_e0e8,
        c_fe_state_options_controller_on_deactivate_001de0e8,
        "cFEStateOptionsController_OnDeactivate",
    ),
    (
        0x001d_e2d8,
        c_keyboard_popup_transition_out_001de2d8,
        "cKeyboardPopup_TransitionOut",
    ),
    (
        0x001d_e550,
        c_fe_state_event_select_v13_001de550,
        "cFEStateEventSelect_v13",
    ),
    (
        0x001d_e908,
        c_fe_state_net_account_mem_v29_001de908,
        "cFEStateNetAccountMem_v29",
    ),
    (
        0x001d_e910,
        c_fe_state_net_account_mem_v32_001de910,
        "cFEStateNetAccountMem_v32",
    ),
    (
        0x001d_e940,
        c_fe_state_mem_card_on_widget_action_001de940,
        "cFEStateMemCard_OnWidgetAction",
    ),
    (
        0x001d_e948,
        c_fe_state_mem_card_v37_001de948,
        "cFEStateMemCard_v37",
    ),
    (
        0x001d_e9f0,
        fe_screen_autosave_popup_transition_in_001de9f0,
        "FEScreenAutosavePopup_TransitionIn",
    ),
    (
        0x001d_eb50,
        c_fe_popup_option_reset_v23_001deb50,
        "cFEPopupOptionReset_v23",
    ),
    (
        0x001d_eb80,
        c_fe_popup_video_calibration_transition_in_001deb80,
        "cFEPopupVideoCalibration_TransitionIn",
    ),
    (
        0x001d_eb88,
        c_fe_popup_video_calibration_transition_out_001deb88,
        "cFEPopupVideoCalibration_TransitionOut",
    ),
    (
        0x001d_f378,
        c_fe_popup_screen_pos_transition_out_001df378,
        "cFEPopupScreenPos_TransitionOut",
    ),
    (
        0x001d_f578,
        c_fe_state_bragging_rights_v13_001df578,
        "cFEStateBraggingRights_v13",
    ),
    (
        0x001d_f5c0,
        c_fe_state_chat_room_v13_001df5c0,
        "cFEStateChatRoom_v13",
    ),
    (
        0x001d_f630,
        c_fe_state_online_agreement_transition_out_001df630,
        "cFEStateOnlineAgreement_TransitionOut",
    ),
    (
        0x001d_f6a0,
        c_fe_state_net_quick_play_popup_transition_out_001df6a0,
        "cFEStateNetQuickPlayPopup_TransitionOut",
    ),
    (
        0x001d_f8e0,
        c_fe_state_net_create_room_popup_transition_out_001df8e0,
        "cFEStateNetCreateRoomPopup_TransitionOut",
    ),
    (
        0x001d_f988,
        c_fe_state_net_help_popup_transition_out_001df988,
        "cFEStateNetHelpPopup_TransitionOut",
    ),
    (
        0x001d_fa88,
        c_fe_state_reward_gallery_get_icon_path_001dfa88,
        "cFEStateRewardGallery_GetIconPath",
    ),
    (
        0x001d_faa0,
        c_fe_state_reward_gallery_v26_001dfaa0,
        "cFEStateRewardGallery_v26",
    ),
    (
        0x001d_faa8,
        c_fe_state_reward_gallery_v27_001dfaa8,
        "cFEStateRewardGallery_v27",
    ),
    (
        0x001d_fac0,
        c_fe_state_reward_gallery_init_buttons_001dfac0,
        "cFEStateRewardGallery_InitButtons",
    ),
    (
        0x001d_fad0,
        c_fe_state_reward_gallery_get_item_record_001dfad0,
        "cFEStateRewardGallery_GetItemRecord",
    ),
    (
        0x001d_fad8,
        c_fe_state_reward_gallery_get_item_price_001dfad8,
        "cFEStateRewardGallery_GetItemPrice",
    ),
    (
        0x001d_fae0,
        c_fe_state_reward_gallery_get_item_field0_e_001dfae0,
        "cFEStateRewardGallery_GetItemField0E",
    ),
    (
        0x001d_fae8,
        c_fe_state_reward_gallery_get_item_name_001dfae8,
        "cFEStateRewardGallery_GetItemName",
    ),
    (
        0x001d_fb88,
        c_fe_state_reward_gallery_posters_get_item_record_001dfb88,
        "cFEStateRewardGalleryPosters_GetItemRecord",
    ),
    (
        0x001d_fbb0,
        c_fe_state_reward_gallery_posters_get_item_field0_e_001dfbb0,
        "cFEStateRewardGalleryPosters_GetItemField0E",
    ),
    (
        0x001d_fbb8,
        c_fe_state_reward_gallery_posters_get_item_name_001dfbb8,
        "cFEStateRewardGalleryPosters_GetItemName",
    ),
    (
        0x001d_fc58,
        c_fe_state_reward_gallery_trading_cards_get_item_record_001dfc58,
        "cFEStateRewardGalleryTradingCards_GetItemRecord",
    ),
    (
        0x001d_fc80,
        c_fe_state_reward_gallery_trading_cards_get_item_field0_e_001dfc80,
        "cFEStateRewardGalleryTradingCards_GetItemField0E",
    ),
    (
        0x001d_fc88,
        c_fe_state_reward_gallery_trading_cards_get_item_name_001dfc88,
        "cFEStateRewardGalleryTradingCards_GetItemName",
    ),
];

/// `AppModule_Unload` (0x001d_c930): straight-line leaf. Mechanical port of the generated body.
pub fn app_module_unload_001dc930(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `AppModule_Enter` (0x001d_c938): straight-line leaf. Mechanical port of the generated body.
pub fn app_module_enter_001dc938(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `FELocale_SetField1` (0x001d_cc38): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn fe_locale_set_field1_001dcc38(c: &mut Cpu) {
    c.cycles += 2;
    c.sb((c.g[A0][0] as u32).wrapping_add(0x1), c.g[A1][0]);
    ret(c);
}

/// `FELocale_v04_cc58` (0x001d_cc58): straight-line leaf. Mechanical port of the generated body.
pub fn fe_locale_v04_cc58_001dcc58(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `FEUICustom_v02` (0x001d_cca8): straight-line leaf. Mechanical port of the generated body.
pub fn feui_custom_v02_001dcca8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `UIInputBase_v39` (0x001d_ccf8): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn ui_input_base_v39_001dccf8(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A0][0] as u32), c.g[A1][0]);
    ret(c);
}

/// `FEUIInput_v41` (0x001d_d560): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn feui_input_v41_001dd560(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
    ret(c);
}

/// `FEUIInput_v42` (0x001d_d568): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn feui_input_v42_001dd568(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
    ret(c);
}

/// `FEUIInput_v43` (0x001d_d570): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn feui_input_v43_001dd570(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
    ret(c);
}

/// `cFEStateUberTrick_v13` (0x001d_d918): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_uber_trick_v13_001dd918(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateOptionsGame_OnDeactivate` (0x001d_dfa8): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_options_game_on_deactivate_001ddfa8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateOptionsSound_OnDeactivate` (0x001d_e010): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_options_sound_on_deactivate_001de010(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateOptionsHUD_OnDeactivate` (0x001d_e078): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_options_hud_on_deactivate_001de078(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateOptionsController_OnDeactivate` (0x001d_e0e8): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_options_controller_on_deactivate_001de0e8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cKeyboardPopup_TransitionOut` (0x001d_e2d8): straight-line leaf. Mechanical port of the generated body.
pub fn c_keyboard_popup_transition_out_001de2d8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEStateEventSelect_v13` (0x001d_e550): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_event_select_v13_001de550(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateNetAccountMem_v29` (0x001d_e908): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_net_account_mem_v29_001de908(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cFEStateNetAccountMem_v32` (0x001d_e910): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_net_account_mem_v32_001de910(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateMemCard_OnWidgetAction` (0x001d_e940): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_mem_card_on_widget_action_001de940(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateMemCard_v37` (0x001d_e948): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_mem_card_v37_001de948(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `FEScreenAutosavePopup_TransitionIn` (0x001d_e9f0): straight-line leaf. Mechanical port of the generated body.
pub fn fe_screen_autosave_popup_transition_in_001de9f0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEPopupOptionReset_v23` (0x001d_eb50): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_fe_popup_option_reset_v23_001deb50(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lb((c.g[A0][0] as u32).wrapping_add(0x48));
    ret(c);
}

/// `cFEPopupVideoCalibration_TransitionIn` (0x001d_eb80): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_popup_video_calibration_transition_in_001deb80(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEPopupVideoCalibration_TransitionOut` (0x001d_eb88): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_popup_video_calibration_transition_out_001deb88(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEPopupScreenPos_TransitionOut` (0x001d_f378): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_popup_screen_pos_transition_out_001df378(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEStateBraggingRights_v13` (0x001d_f578): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_bragging_rights_v13_001df578(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateChatRoom_v13` (0x001d_f5c0): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_chat_room_v13_001df5c0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateOnlineAgreement_TransitionOut` (0x001d_f630): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_online_agreement_transition_out_001df630(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEStateNetQuickPlayPopup_TransitionOut` (0x001d_f6a0): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_net_quick_play_popup_transition_out_001df6a0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEStateNetCreateRoomPopup_TransitionOut` (0x001d_f8e0): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_net_create_room_popup_transition_out_001df8e0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEStateNetHelpPopup_TransitionOut` (0x001d_f988): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_net_help_popup_transition_out_001df988(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEStateRewardGallery_GetIconPath` (0x001d_fa88): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_get_icon_path_001dfa88(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cFEStateRewardGallery_v26` (0x001d_faa0): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_v26_001dfaa0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0xffffffffffffffff;
    ret(c);
}

/// `cFEStateRewardGallery_v27` (0x001d_faa8): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_v27_001dfaa8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cFEStateRewardGallery_InitButtons` (0x001d_fac0): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_init_buttons_001dfac0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateRewardGallery_GetItemRecord` (0x001d_fad0): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_get_item_record_001dfad0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cFEStateRewardGallery_GetItemPrice` (0x001d_fad8): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_get_item_price_001dfad8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cFEStateRewardGallery_GetItemField0E` (0x001d_fae0): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_get_item_field0_e_001dfae0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cFEStateRewardGallery_GetItemName` (0x001d_fae8): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_get_item_name_001dfae8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cFEStateRewardGalleryPosters_GetItemRecord` (0x001d_fb88): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_posters_get_item_record_001dfb88(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa28));
    ret(c);
}

/// `cFEStateRewardGalleryPosters_GetItemField0E` (0x001d_fbb0): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_posters_get_item_field0_e_001dfbb0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0xe));
    ret(c);
}

/// `cFEStateRewardGalleryPosters_GetItemName` (0x001d_fbb8): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_posters_get_item_name_001dfbb8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A1][0] as u32));
    ret(c);
}

/// `cFEStateRewardGalleryTradingCards_GetItemRecord` (0x001d_fc58): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_trading_cards_get_item_record_001dfc58(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa28));
    ret(c);
}

/// `cFEStateRewardGalleryTradingCards_GetItemField0E` (0x001d_fc80): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_trading_cards_get_item_field0_e_001dfc80(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0xe));
    ret(c);
}

/// `cFEStateRewardGalleryTradingCards_GetItemName` (0x001d_fc88): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_trading_cards_get_item_name_001dfc88(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A1][0] as u32));
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

    /// `AppModule_Unload` (0x001d_c930) against the generated body.
    #[test]
    fn app_module_unload_001dc930_matches() {
        check(
            ee::s04::f_001dc930__generated,
            app_module_unload_001dc930,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1dc930, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `AppModule_Enter` (0x001d_c938) against the generated body.
    #[test]
    fn app_module_enter_001dc938_matches() {
        check(
            ee::s04::f_001dc938__generated,
            app_module_enter_001dc938,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1dc938, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `FELocale_SetField1` (0x001d_cc38) against the generated body.
    #[test]
    fn fe_locale_set_field1_001dcc38_matches() {
        check(
            ee::s04::f_001dcc38__generated,
            fe_locale_set_field1_001dcc38,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dcc38, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[A1][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `FELocale_v04_cc58` (0x001d_cc58) against the generated body.
    #[test]
    fn fe_locale_v04_cc58_001dcc58_matches() {
        check(
            ee::s04::f_001dcc58__generated,
            fe_locale_v04_cc58_001dcc58,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1dcc58, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `FEUICustom_v02` (0x001d_cca8) against the generated body.
    #[test]
    fn feui_custom_v02_001dcca8_matches() {
        check(
            ee::s04::f_001dcca8__generated,
            feui_custom_v02_001dcca8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1dcca8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_v39` (0x001d_ccf8) against the generated body.
    #[test]
    fn ui_input_base_v39_001dccf8_matches() {
        check(
            ee::s04::f_001dccf8__generated,
            ui_input_base_v39_001dccf8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dccf8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[A1][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `FEUIInput_v41` (0x001d_d560) against the generated body.
    #[test]
    fn feui_input_v41_001dd560_matches() {
        check(
            ee::s04::f_001dd560__generated,
            feui_input_v41_001dd560,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dd560, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `FEUIInput_v42` (0x001d_d568) against the generated body.
    #[test]
    fn feui_input_v42_001dd568_matches() {
        check(
            ee::s04::f_001dd568__generated,
            feui_input_v42_001dd568,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dd568, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `FEUIInput_v43` (0x001d_d570) against the generated body.
    #[test]
    fn feui_input_v43_001dd570_matches() {
        check(
            ee::s04::f_001dd570__generated,
            feui_input_v43_001dd570,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dd570, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateUberTrick_v13` (0x001d_d918) against the generated body.
    #[test]
    fn c_fe_state_uber_trick_v13_001dd918_matches() {
        check(
            ee::s04::f_001dd918__generated,
            c_fe_state_uber_trick_v13_001dd918,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1dd918, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateOptionsGame_OnDeactivate` (0x001d_dfa8) against the generated body.
    #[test]
    fn c_fe_state_options_game_on_deactivate_001ddfa8_matches() {
        check(
            ee::s04::f_001ddfa8__generated,
            c_fe_state_options_game_on_deactivate_001ddfa8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1ddfa8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateOptionsSound_OnDeactivate` (0x001d_e010) against the generated body.
    #[test]
    fn c_fe_state_options_sound_on_deactivate_001de010_matches() {
        check(
            ee::s04::f_001de010__generated,
            c_fe_state_options_sound_on_deactivate_001de010,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1de010, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateOptionsHUD_OnDeactivate` (0x001d_e078) against the generated body.
    #[test]
    fn c_fe_state_options_hud_on_deactivate_001de078_matches() {
        check(
            ee::s04::f_001de078__generated,
            c_fe_state_options_hud_on_deactivate_001de078,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1de078, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateOptionsController_OnDeactivate` (0x001d_e0e8) against the generated body.
    #[test]
    fn c_fe_state_options_controller_on_deactivate_001de0e8_matches() {
        check(
            ee::s04::f_001de0e8__generated,
            c_fe_state_options_controller_on_deactivate_001de0e8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1de0e8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cKeyboardPopup_TransitionOut` (0x001d_e2d8) against the generated body.
    #[test]
    fn c_keyboard_popup_transition_out_001de2d8_matches() {
        check(
            ee::s04::f_001de2d8__generated,
            c_keyboard_popup_transition_out_001de2d8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1de2d8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateEventSelect_v13` (0x001d_e550) against the generated body.
    #[test]
    fn c_fe_state_event_select_v13_001de550_matches() {
        check(
            ee::s04::f_001de550__generated,
            c_fe_state_event_select_v13_001de550,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1de550, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateNetAccountMem_v29` (0x001d_e908) against the generated body.
    #[test]
    fn c_fe_state_net_account_mem_v29_001de908_matches() {
        check(
            ee::s04::f_001de908__generated,
            c_fe_state_net_account_mem_v29_001de908,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1de908, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateNetAccountMem_v32` (0x001d_e910) against the generated body.
    #[test]
    fn c_fe_state_net_account_mem_v32_001de910_matches() {
        check(
            ee::s04::f_001de910__generated,
            c_fe_state_net_account_mem_v32_001de910,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1de910, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateMemCard_OnWidgetAction` (0x001d_e940) against the generated body.
    #[test]
    fn c_fe_state_mem_card_on_widget_action_001de940_matches() {
        check(
            ee::s04::f_001de940__generated,
            c_fe_state_mem_card_on_widget_action_001de940,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1de940, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateMemCard_v37` (0x001d_e948) against the generated body.
    #[test]
    fn c_fe_state_mem_card_v37_001de948_matches() {
        check(
            ee::s04::f_001de948__generated,
            c_fe_state_mem_card_v37_001de948,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1de948, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `FEScreenAutosavePopup_TransitionIn` (0x001d_e9f0) against the generated body.
    #[test]
    fn fe_screen_autosave_popup_transition_in_001de9f0_matches() {
        check(
            ee::s04::f_001de9f0__generated,
            fe_screen_autosave_popup_transition_in_001de9f0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1de9f0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEPopupOptionReset_v23` (0x001d_eb50) against the generated body.
    #[test]
    fn c_fe_popup_option_reset_v23_001deb50_matches() {
        check(
            ee::s04::f_001deb50__generated,
            c_fe_popup_option_reset_v23_001deb50,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1deb50, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEPopupVideoCalibration_TransitionIn` (0x001d_eb80) against the generated body.
    #[test]
    fn c_fe_popup_video_calibration_transition_in_001deb80_matches() {
        check(
            ee::s04::f_001deb80__generated,
            c_fe_popup_video_calibration_transition_in_001deb80,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1deb80, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEPopupVideoCalibration_TransitionOut` (0x001d_eb88) against the generated body.
    #[test]
    fn c_fe_popup_video_calibration_transition_out_001deb88_matches() {
        check(
            ee::s04::f_001deb88__generated,
            c_fe_popup_video_calibration_transition_out_001deb88,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1deb88, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEPopupScreenPos_TransitionOut` (0x001d_f378) against the generated body.
    #[test]
    fn c_fe_popup_screen_pos_transition_out_001df378_matches() {
        check(
            ee::s04::f_001df378__generated,
            c_fe_popup_screen_pos_transition_out_001df378,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1df378, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateBraggingRights_v13` (0x001d_f578) against the generated body.
    #[test]
    fn c_fe_state_bragging_rights_v13_001df578_matches() {
        check(
            ee::s04::f_001df578__generated,
            c_fe_state_bragging_rights_v13_001df578,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1df578, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateChatRoom_v13` (0x001d_f5c0) against the generated body.
    #[test]
    fn c_fe_state_chat_room_v13_001df5c0_matches() {
        check(
            ee::s04::f_001df5c0__generated,
            c_fe_state_chat_room_v13_001df5c0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1df5c0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateOnlineAgreement_TransitionOut` (0x001d_f630) against the generated body.
    #[test]
    fn c_fe_state_online_agreement_transition_out_001df630_matches() {
        check(
            ee::s04::f_001df630__generated,
            c_fe_state_online_agreement_transition_out_001df630,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1df630, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateNetQuickPlayPopup_TransitionOut` (0x001d_f6a0) against the generated body.
    #[test]
    fn c_fe_state_net_quick_play_popup_transition_out_001df6a0_matches() {
        check(
            ee::s04::f_001df6a0__generated,
            c_fe_state_net_quick_play_popup_transition_out_001df6a0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1df6a0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateNetCreateRoomPopup_TransitionOut` (0x001d_f8e0) against the generated body.
    #[test]
    fn c_fe_state_net_create_room_popup_transition_out_001df8e0_matches() {
        check(
            ee::s04::f_001df8e0__generated,
            c_fe_state_net_create_room_popup_transition_out_001df8e0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1df8e0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateNetHelpPopup_TransitionOut` (0x001d_f988) against the generated body.
    #[test]
    fn c_fe_state_net_help_popup_transition_out_001df988_matches() {
        check(
            ee::s04::f_001df988__generated,
            c_fe_state_net_help_popup_transition_out_001df988,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1df988, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGallery_GetIconPath` (0x001d_fa88) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_get_icon_path_001dfa88_matches() {
        check(
            ee::s04::f_001dfa88__generated,
            c_fe_state_reward_gallery_get_icon_path_001dfa88,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1dfa88, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGallery_v26` (0x001d_faa0) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_v26_001dfaa0_matches() {
        check(
            ee::s04::f_001dfaa0__generated,
            c_fe_state_reward_gallery_v26_001dfaa0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1dfaa0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGallery_v27` (0x001d_faa8) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_v27_001dfaa8_matches() {
        check(
            ee::s04::f_001dfaa8__generated,
            c_fe_state_reward_gallery_v27_001dfaa8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1dfaa8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGallery_InitButtons` (0x001d_fac0) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_init_buttons_001dfac0_matches() {
        check(
            ee::s04::f_001dfac0__generated,
            c_fe_state_reward_gallery_init_buttons_001dfac0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1dfac0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGallery_GetItemRecord` (0x001d_fad0) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_get_item_record_001dfad0_matches() {
        check(
            ee::s04::f_001dfad0__generated,
            c_fe_state_reward_gallery_get_item_record_001dfad0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1dfad0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGallery_GetItemPrice` (0x001d_fad8) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_get_item_price_001dfad8_matches() {
        check(
            ee::s04::f_001dfad8__generated,
            c_fe_state_reward_gallery_get_item_price_001dfad8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1dfad8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGallery_GetItemField0E` (0x001d_fae0) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_get_item_field0_e_001dfae0_matches() {
        check(
            ee::s04::f_001dfae0__generated,
            c_fe_state_reward_gallery_get_item_field0_e_001dfae0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1dfae0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGallery_GetItemName` (0x001d_fae8) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_get_item_name_001dfae8_matches() {
        check(
            ee::s04::f_001dfae8__generated,
            c_fe_state_reward_gallery_get_item_name_001dfae8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1dfae8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryPosters_GetItemRecord` (0x001d_fb88) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_posters_get_item_record_001dfb88_matches() {
        check(
            ee::s04::f_001dfb88__generated,
            c_fe_state_reward_gallery_posters_get_item_record_001dfb88,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dfb88, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[A1][0] = (r.next_u32() % 8 * 4) as u64;
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryPosters_GetItemField0E` (0x001d_fbb0) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_posters_get_item_field0_e_001dfbb0_matches() {
        check(
            ee::s04::f_001dfbb0__generated,
            c_fe_state_reward_gallery_posters_get_item_field0_e_001dfbb0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dfbb0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryPosters_GetItemName` (0x001d_fbb8) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_posters_get_item_name_001dfbb8_matches() {
        check(
            ee::s04::f_001dfbb8__generated,
            c_fe_state_reward_gallery_posters_get_item_name_001dfbb8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dfbb8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryTradingCards_GetItemRecord` (0x001d_fc58) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_trading_cards_get_item_record_001dfc58_matches() {
        check(
            ee::s04::f_001dfc58__generated,
            c_fe_state_reward_gallery_trading_cards_get_item_record_001dfc58,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dfc58, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[A1][0] = (r.next_u32() % 8 * 4) as u64;
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryTradingCards_GetItemField0E` (0x001d_fc80) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_trading_cards_get_item_field0_e_001dfc80_matches() {
        check(
            ee::s04::f_001dfc80__generated,
            c_fe_state_reward_gallery_trading_cards_get_item_field0_e_001dfc80,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dfc80, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryTradingCards_GetItemName` (0x001d_fc88) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_trading_cards_get_item_name_001dfc88_matches() {
        check(
            ee::s04::f_001dfc88__generated,
            c_fe_state_reward_gallery_trading_cards_get_item_name_001dfc88,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dfc88, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
