//! `src/fe/fepopup.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::animation::rideranimbase::sw as sw_3;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at as lw_at_3;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_4;
pub(crate) use crate::game::fe::feasyncfile::ldf;
pub(crate) use crate::game::fe::feasyncfile::lh;
pub(crate) use crate::game::fe::feasyncfile::lw_3;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw_7;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_8;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_4;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_5;
pub(crate) use crate::game::fe::feasyncfile::ui_init_screen;
pub(crate) use crate::game::fe::feasyncfile::widget_action_v24;
pub(crate) use crate::game::fe::fepopupmisc::p_2 as p_3;
pub(crate) use crate::game::fe::fepopupmisc::sw;
#[cfg(test)]
pub(crate) use crate::game::fe::festateoptions::fe_setup;
pub(crate) use crate::game::fe::festateoptions::fe_vptr;
pub(crate) use crate::game::fe::festateoptions::FE_KEYBOARD_SET_KEY_ENABLED;
#[cfg(test)]
pub(crate) use crate::game::fe::festateoptions::FE_SEEDS;
pub(crate) use crate::game::fe::festateoptions::FE_STRLEN;
pub(crate) use crate::game::fe::festateoptions::FE_UI_STATE_CTOR_2;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_5;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at_2;
pub(crate) use crate::game::main::gamestate::vslot;
pub(crate) use crate::game::movie::movieplayer::at;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::sound::bankmonitor::p;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw_6;
pub(crate) use crate::game::wscript::wscriptman::sw as sw_2;
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
        0x001c_7258,
        c_fe_popup_confirm_layout_body_text,
        "cFEPopupConfirm_LayoutBodyText",
    ),
    (
        0x001c_5a90,
        c_fe_popup_confirm_base_dtor,
        "cFEPopupConfirm_BaseDtor",
    ),
    (0x001c_5af8, c_fe_popup_confirm_v04, "cFEPopupConfirm_v04"),
    (0x001c_5b88, cfe_popup_confirm_v06, "cFEPopupConfirm_v06"),
    (
        0x001c_5c30,
        cfe_popup_confirm_transition_in,
        "cFEPopupConfirm_v07",
    ),
    (
        0x001c_7040,
        cfe_popup_confirm_layout_post_text,
        "cFEPopupConfirm_LayoutPostText",
    ),
    (
        0x001c_70f0,
        cfe_popup_confirm_resize_text_widgets,
        "cFEPopupConfirm_ResizeTextWidgets",
    ),
    (
        0x001c_75c8,
        c_fe_popup_confirm_select_option,
        "cFEPopupConfirm_SelectOption",
    ),
    (
        0x001c_7738,
        c_fe_popup_confirm_apply_background_scale,
        "cFEPopupConfirm_ApplyBackgroundScale",
    ),
    (
        0x001c_78c0,
        fe_popup_confirm_position_background,
        "cFEPopupConfirm_PositionBackground",
    ),
    (
        0x001c_7be0,
        c_fe_popup_confirm_get_centered_x,
        "cFEPopupConfirm_GetCenteredX",
    ),
    (
        0x001c_8050,
        cfe_popup_confirm_set_text_from_desc,
        "cFEPopupConfirm_SetTextFromDesc",
    ),
    (
        0x001c_8258,
        cfe_popup_confirm_get_widget_text,
        "cFEPopupConfirm_GetWidgetText",
    ),
    (
        0x001c_84a8,
        fe_popup_confirm_measure_line,
        "cFEPopupConfirm_MeasureLine",
    ),
    (
        0x001c_8568,
        fe_popup_confirm_measure_line_w,
        "cFEPopupConfirm_MeasureLineW",
    ),
    (
        0x001c_8628,
        c_fe_popup_confirm_get_widget_size,
        "cFEPopupConfirm_GetWidgetSize",
    ),
    (
        0x001c_8758,
        c_fe_popup_confirm_layout_menu,
        "cFEPopupConfirm_LayoutMenu",
    ),
    (
        0x001c_88c8,
        c_fe_popup_confirm_position_buttons,
        "cFEPopupConfirm_PositionButtons",
    ),
    (
        0x001c_9038,
        c_fe_popup_confirm_get_text_box_size_w,
        "cFEPopupConfirm_GetTextBoxSizeW",
    ),
    (
        0x001c_9210,
        c_fe_popup_confirm_get_text_box_size,
        "cFEPopupConfirm_GetTextBoxSize",
    ),
    (
        0x001c_9b28,
        fe_popup_confirm_set_text_measured,
        "cFEPopupConfirm_SetTextMeasured",
    ),
    (
        0x001c_9b68,
        c_fe_popup_confirm_accumulate_size,
        "cFEPopupConfirm_AccumulateSize",
    ),
    (
        0x001c_a150,
        c_fe_popup_confirm_get_selection,
        "cFEPopupConfirm_GetSelection",
    ),
    (
        0x001c_a488,
        fe_popup_confirm_hide_widget,
        "cFEPopupConfirm_HideWidget",
    ),
    (
        0x001c_a4c0,
        fe_popup_confirm_show_widget,
        "cFEPopupConfirm_ShowWidget",
    ),
    (0x001c_a4f8, c_fe_popup_confirm_v24, "cFEPopupConfirm_v24"),
    (
        0x001c_a528,
        c_fe_popup_confirm_update,
        "cFEPopupConfirm_Update",
    ),
    (
        0x001c_a8a0,
        cui_state_buy_popup_reset_params,
        "cUIStateBuyPopup_ResetParams",
    ),
    (
        0x001c_a900,
        c_ui_state_buy_popup_set_song_credit_params,
        "cUIStateBuyPopup_SetSongCreditParams",
    ),
    (
        0x001c_a980,
        cuistate_buy_popup_set_music_track_params,
        "cUIStateBuyPopup_SetMusicTrackParams",
    ),
    (
        0x001c_ab10,
        cuistate_buy_popup_set_item_params,
        "cUIStateBuyPopup_SetItemParams",
    ),
    (
        0x001c_aba8,
        c_ui_state_buy_popup_ctor,
        "cUIStateBuyPopup_ctor",
    ),
    (
        0x001c_ac30,
        cui_state_buy_popup_init,
        "cUIStateBuyPopup_Init",
    ),
    (
        0x001c_aed8,
        c_ui_state_buy_popup_on_widget_notify,
        "cUIStateBuyPopup_OnWidgetNotify",
    ),
    (
        0x001c_af00,
        cui_state_buy_popup_on_widget_action,
        "cUIStateBuyPopup_OnWidgetAction",
    ),
    (
        0x001c_af58,
        c_ui_state_buy_popup_set_uber_trick_params,
        "cUIStateBuyPopup_SetUberTrickParams",
    ),
    (
        0x001c_afc0,
        c_ui_state_buy_popup_set_attrib_params,
        "cUIStateBuyPopup_SetAttribParams",
    ),
    (0x001c_b138, keyboard_popup_dtor, "cKeyboardPopup_dtor"),
    (0x001c_b1a0, keyboard_popup_init, "cKeyboardPopup_Init"),
    (0x001c_b208, ckeyboardpopup_update, "cKeyboardPopup_Update"),
    (
        0x001c_b290,
        ckeyboard_popup_on_widget_action,
        "cKeyboardPopup_OnWidgetAction",
    ),
    (0x001c_b3d0, c_keyboard_popup_v24, "cKeyboardPopup_v24"),
    (
        0x001c_b9b0,
        c_keyboard_popup_move_down,
        "cKeyboardPopup_MoveDown",
    ),
    (
        0x001c_bc48,
        c_keyboard_popup_move_down_from_top_row,
        "cKeyboardPopup_MoveDownFromTopRow",
    ),
    (
        0x001c_bce8,
        c_keyboard_popup_move_down_from_bottom_row,
        "cKeyboardPopup_MoveDownFromBottomRow",
    ),
    (
        0x001c_bf20,
        c_keyboard_popup_move_up_from_bottom_row,
        "cKeyboardPopup_MoveUpFromBottomRow",
    ),
    (
        0x001c_bfc0,
        ckeyboard_popup_move_down_from_row4,
        "cKeyboardPopup_MoveDownFromRow4",
    ),
    (
        0x001c_c090,
        ckeyboard_popup_find_enabled_in_column_from_top,
        "cKeyboardPopup_FindEnabledInColumnFromTop",
    ),
    (
        0x001c_c0e0,
        c_keyboard_popup_find_enabled_in_column_from_bottom,
        "cKeyboardPopup_FindEnabledInColumnFromBottom",
    ),
    (
        0x001c_c140,
        ckeyboard_popup_step_up_to_enabled_key,
        "cKeyboardPopup_StepUpToEnabledKey",
    ),
    (
        0x001c_c190,
        ckeyboard_popup_step_down_to_enabled_key,
        "cKeyboardPopup_StepDownToEnabledKey",
    ),
    (
        0x001c_c620,
        c_keyboard_popup_insert_char,
        "cKeyboardPopup_InsertChar",
    ),
    (
        0x001c_c6e8,
        ckeyboardpopup_handle_key_char,
        "cKeyboardPopup_HandleKeyChar",
    ),
    (
        0x001c_c7c8,
        c_keyboard_popup_type_current_key,
        "cKeyboardPopup_TypeCurrentKey",
    ),
    (
        0x001c_c848,
        c_keyboard_popup_refresh_entry_text,
        "cKeyboardPopup_RefreshEntryText",
    ),
    (
        0x001c_ce98,
        keyboard_popup_mask_text,
        "cKeyboardPopup_MaskText",
    ),
    (
        0x001c_cf70,
        keyboard_popup_set_static_wide_text,
        "cKeyboardPopup_SetStaticWideText",
    ),
    (
        0x001c_d020,
        keyboard_popup_set_static_text_id,
        "cKeyboardPopup_SetStaticTextId",
    ),
    (
        0x001c_d0b0,
        c_keyboard_popup_set_min_length,
        "cKeyboardPopup_SetMinLength",
    ),
    (
        0x001c_d2f0,
        c_keyboard_popup_on_activate,
        "cKeyboardPopup_OnActivate",
    ),
    (
        0x001c_e408,
        c_keyboard_popup_set_digit_keys_disabled,
        "cKeyboardPopup_SetDigitKeysDisabled",
    ),
    (
        0x001c_e4c8,
        c_keyboard_popup_apply_font_scale,
        "cKeyboardPopup_ApplyFontScale",
    ),
    (
        0x001c_e520,
        c_keyboard_popup_ensure_cursor_on_enabled_key,
        "cKeyboardPopup_EnsureCursorOnEnabledKey",
    ),
    (
        0x001c_e560,
        c_keyboard_popup_is_key_enabled,
        "cKeyboardPopup_IsKeyEnabled",
    ),
    (
        0x001c_e6f0,
        cfe_state_trophy_room_count_event_entries,
        "cFEStateTrophyRoom_CountEventEntries",
    ),
    (
        0x001d_dbd0,
        c_fe_popup_confirm_base_ctor_2,
        "cFEPopupConfirm_BaseCtor_2",
    ),
    (
        0x001d_dc48,
        c_ui_state_buy_popup_ctor_2,
        "cUIStateBuyPopup_ctor_2",
    ),
    (
        0x001d_dc90,
        c_ui_state_buy_popup_dtor,
        "cUIStateBuyPopup_dtor",
    ),
    (
        0x001d_dcc8,
        c_ui_state_buy_popup_set_song_credit_params_thunk,
        "cUIStateBuyPopup_SetSongCreditParams_Thunk",
    ),
    (
        0x001d_dce8,
        c_ui_state_buy_popup_set_music_track_params_thunk,
        "cUIStateBuyPopup_SetMusicTrackParams_Thunk",
    ),
    (
        0x001d_dd08,
        c_ui_state_buy_popup_set_reward_item_params_thunk,
        "cUIStateBuyPopup_SetRewardItemParams_Thunk",
    ),
    (
        0x001d_de78,
        c_fe_popup_confirm_ctor_2,
        "cFEPopupConfirm_ctor_2",
    ),
    (0x001d_deb8, c_fe_popup_confirm_dtor, "cFEPopupConfirm_dtor"),
    (0x001d_e288, ckeyboard_popup_ctor_2, "cKeyboardPopup_ctor_2"),
    (
        0x001d_e2e8,
        ckeyboard_popup_get_text_length,
        "cKeyboardPopup_GetTextLength",
    ),
    (
        0x001d_e350,
        ckeyboard_popup_set_key4b_enabled,
        "cKeyboardPopup_SetKey4BEnabled",
    ),
    (
        0x001d_e378,
        c_keyboard_popup_set_key_1c_enabled,
        "cKeyboardPopup_SetKey1CEnabled",
    ),
    (
        0x001d_e3a0,
        c_keyboard_popup_set_key_45_enabled,
        "cKeyboardPopup_SetKey45Enabled",
    ),
    (
        0x001d_e3c8,
        c_keyboard_popup_set_key_4f_enabled,
        "cKeyboardPopup_SetKey4FEnabled",
    ),
    (
        0x001d_e3f0,
        c_keyboard_popup_set_key_29_enabled,
        "cKeyboardPopup_SetKey29Enabled",
    ),
    (
        0x001d_e418,
        c_keyboard_popup_set_key_42_enabled,
        "cKeyboardPopup_SetKey42Enabled",
    ),
    (
        0x001c_58e8,
        c_fe_popup_confirm_base_ctor,
        "cFEPopupConfirm_BaseCtor",
    ),
    (0x001c_5b68, c_fe_popup_confirm_v05, "cFEPopupConfirm_v05"),
    (
        0x001c_5e58,
        c_fe_popup_confirm_set_content_visible,
        "cFEPopupConfirm_SetContentVisible",
    ),
    (
        0x001c_5f20,
        c_fe_popup_confirm_set_fade,
        "cFEPopupConfirm_SetFade",
    ),
    (
        0x001c_6230,
        c_fe_popup_confirm_find_widgets,
        "cFEPopupConfirm_FindWidgets",
    ),
    (
        0x001c_66e8,
        c_fe_popup_confirm_apply_background_style,
        "cFEPopupConfirm_ApplyBackgroundStyle",
    ),
    (
        0x001c_6b08,
        c_fe_popup_confirm_apply_config,
        "cFEPopupConfirm_ApplyConfig",
    ),
    (
        0x001c_6d30,
        c_fe_popup_confirm_layout_text_entry,
        "cFEPopupConfirm_LayoutTextEntry",
    ),
    (
        0x001c_7388,
        c_fe_popup_confirm_layout_options,
        "cFEPopupConfirm_LayoutOptions",
    ),
    (
        0x001c_7620,
        c_fe_popup_confirm_compute_background_scale,
        "cFEPopupConfirm_ComputeBackgroundScale",
    ),
    (
        0x001c_7c20,
        c_fe_popup_confirm_layout_frame,
        "cFEPopupConfirm_LayoutFrame",
    ),
    (
        0x001c_7eb0,
        c_fe_popup_confirm_layout_text_entry_box,
        "cFEPopupConfirm_LayoutTextEntryBox",
    ),
    (
        0x001c_80e8,
        c_fe_popup_confirm_desc_to_string,
        "cFEPopupConfirm_DescToString",
    ),
    (
        0x001c_8320,
        c_fe_popup_confirm_measure_desc,
        "cFEPopupConfirm_MeasureDesc",
    ),
    (
        0x001c_8678,
        c_fe_popup_confirm_layout_title,
        "cFEPopupConfirm_LayoutTitle",
    ),
    (
        0x001c_8930,
        c_fe_popup_confirm_set_widget_align,
        "cFEPopupConfirm_SetWidgetAlign",
    ),
    (
        0x001c_8a38,
        c_fe_popup_confirm_reset_layout,
        "cFEPopupConfirm_ResetLayout",
    ),
    (
        0x001c_9b98,
        c_fe_popup_confirm_open_confirm_popup,
        "cFEPopupConfirm_OpenConfirmPopup",
    ),
    (
        0x001c_9cd0,
        c_fe_popup_confirm_adjust_draw_layer,
        "cFEPopupConfirm_AdjustDrawLayer",
    ),
    (
        0x001c_a0f0,
        fepopup_confirm_set_option_enabled,
        "cFEPopupConfirm_SetOptionEnabled",
    ),
    (
        0x001c_a180,
        c_fe_popup_confirm_on_widget_notify,
        "cFEPopupConfirm_OnWidgetNotify",
    ),
    (
        0x001c_a298,
        c_fe_popup_confirm_on_widget_action,
        "cFEPopupConfirm_OnWidgetAction",
    ),
    (
        0x001c_a578,
        fepopup_confirm_render,
        "cFEPopupConfirm_Render",
    ),
    (
        0x001c_a598,
        c_fe_popup_confirm_on_child_event,
        "cFEPopupConfirm_OnChildEvent",
    ),
    (
        0x001c_aa18,
        c_ui_state_buy_popup_set_reward_item_params,
        "cUIStateBuyPopup_SetRewardItemParams",
    ),
    (
        0x001c_ac98,
        c_ui_state_buy_popup_on_widget_created,
        "cUIStateBuyPopup_OnWidgetCreated",
    ),
    (0x001c_b030, keyboard_popup_ctor, "cKeyboardPopup_ctor"),
    (
        0x001c_b2c8,
        keyboard_popup_on_widget_created,
        "cKeyboardPopup_OnWidgetCreated",
    ),
    (
        0x001c_bb70,
        keyboard_popup_move_up_from_top_row,
        "cKeyboardPopup_MoveUpFromTopRow",
    ),
    (
        0x001c_bdb0,
        keyboard_popup_move_up_from_digit_row,
        "cKeyboardPopup_MoveUpFromDigitRow",
    ),
    (
        0x001c_c1e0,
        c_keyboard_popup_press_key,
        "cKeyboardPopup_PressKey",
    ),
    (
        0x001c_c8d8,
        c_keyboard_popup_update_scroll_window,
        "cKeyboardPopup_UpdateScrollWindow",
    ),
    (
        0x001c_cc30,
        c_keyboard_popup_update_cursor,
        "cKeyboardPopup_UpdateCursor",
    ),
    (
        0x001c_d0d8,
        c_keyboard_popup_get_key_widget_name,
        "cKeyboardPopup_GetKeyWidgetName",
    ),
    (
        0x001c_d348,
        c_keyboard_popup_transition_in,
        "cKeyboardPopup_TransitionIn",
    ),
    (
        0x001c_d708,
        c_keyboard_popup_refresh_keys,
        "cKeyboardPopup_RefreshKeys",
    ),
    (
        0x001c_d9a0,
        c_keyboard_popup_set_special_key_label,
        "cKeyboardPopup_SetSpecialKeyLabel",
    ),
    (
        0x001c_db98,
        c_keyboard_popup_handle_input,
        "cKeyboardPopup_HandleInput",
    ),
    (
        0x001c_e2a8,
        keyboard_popup_build_keys,
        "cKeyboardPopup_BuildKeys",
    ),
    (
        0x001c_e5e0,
        keyboard_popup_append_quick_message,
        "cKeyboardPopup_AppendQuickMessage",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::bx::bxstringctor::callee::POPUP_SET_WIDGET_ALIGN;
    pub(crate) use crate::game::bx::bxstringctor::callee::POPUP_SET_WRAPPED_TEXT;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUY_POPUP_RESET;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::KEYBOARD_POPUP_IS_KEY_ENABLED as KEYBOARD_POPUP_IS_KEY_ENABLED_2;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::KEYBOARD_POPUP_UPDATE_CURSOR;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::MEASURE_PLAIN;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::MEASURE_WRAP;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SPRINTF_6 as SPRINTF_4;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::TEXT_OBJ;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::UIMENU_SET_SELECTION;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_STATE_ON_ACTIVATE as UI_STATE_ON_ACTIVATE_2;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_TEXT_GET_FONT;

    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_ASSIGN_CSTR;

    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_FIND_ENABLED_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_FIND_ENABLED_IN_COLUMN_FROM_TOP;

    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED_4 as OPERATOR_NEW_NAMED_3;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_ID_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_OWNED_STRING;

    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_LEN;

    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_TO_ASCII;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::POPUP_CONFIRM_GET_TEXT_BOX_SIZE;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::UI_TEXT_SET_STRING;

    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_MOVE_DOWN;

    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_MASK_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_CURSOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW;

    pub const C00113F88: Callee = Callee {
        addr: 0x00113f88,
        f: ee::s00::Rider_SetSlew3And4Targets,
    };

    pub const C00115AB0: Callee = Callee {
        addr: 0x00115ab0,
        f: ee::s00::Rider_GetGateChargeAngle,
    };

    pub const C0011FE78: Callee = Callee {
        addr: 0x0011fe78,
        f: ee::s00::Rider_SetMainState,
    };

    pub const C0011FE98: Callee = Callee {
        addr: 0x0011fe98,
        f: ee::s00::Rider_GetMainState,
    };

    pub const C0011FEC8: Callee = Callee {
        addr: 0x0011fec8,
        f: ee::s00::Rider_SetSubState,
    };

    pub const C00125108: Callee = Callee {
        addr: 0x00125108,
        f: ee::s00::Rider_OnFinishRace,
    };

    pub const C0014A080: Callee = Callee {
        addr: 0x0014a080,
        f: ee::s01::BE_GetRiderCharacter,
    };

    pub const C0014A0E0: Callee = Callee {
        addr: 0x0014a0e0,
        f: ee::s01::BE_GetRiderProfile,
    };

    pub const C0014BDB8: Callee = Callee {
        addr: 0x0014bdb8,
        f: ee::s01::Bolts_GetDB,
    };

    pub const C0014D448: Callee = Callee {
        addr: 0x0014d448,
        f: ee::s01::Bolts_FindBoltIndex,
    };

    pub const C0014D988: Callee = Callee {
        addr: 0x0014d988,
        f: ee::s01::Bolts_GetCharBoltCount,
    };

    pub const C0014D998: Callee = Callee {
        addr: 0x0014d998,
        f: ee::s01::Bolts_GetCharBolts,
    };

    pub const C0014DC80: Callee = Callee {
        addr: 0x0014dc80,
        f: ee::s01::BELibrary_GetInstance,
    };

    pub const C0014DD58: Callee = Callee {
        addr: 0x0014dd58,
        f: ee::s01::BELibrary_GetInterface,
    };

    pub const C00150928: Callee = Callee {
        addr: 0x00150928,
        f: ee::s01::BEEconInt_GetMoney,
    };

    pub const C00150B48: Callee = Callee {
        addr: 0x00150b48,
        f: ee::s01::BEEconInt_SpendMoney,
    };

    pub const C00151C48: Callee = Callee {
        addr: 0x00151c48,
        f: ee::s01::BE_SetCharProfileBoltOwned,
    };

    pub const C001535C0: Callee = Callee {
        addr: 0x001535c0,
        f: ee::s01::BEMissionInt_CountCollectedOnPeak,
    };

    pub const C001543E0: Callee = Callee {
        addr: 0x001543e0,
        f: ee::s01::BEMissionInt_CountCompletedOnPeak,
    };

    pub const C00156988: Callee = Callee {
        addr: 0x00156988,
        f: ee::s01::BE_RestoreCharacter,
    };

    pub const C00156A60: Callee = Callee {
        addr: 0x00156a60,
        f: ee::s01::BERewardInt_IsBetterPlace,
    };

    pub const C001577E0: Callee = Callee {
        addr: 0x001577e0,
        f: ee::s01::BERewardInt_IsPeakFullyCompleted,
    };

    pub const C00157BF0: Callee = Callee {
        addr: 0x00157bf0,
        f: ee::s01::BERewardInt_IsPeakGoalComplete,
    };

    pub const C00158910: Callee = Callee {
        addr: 0x00158910,
        f: ee::s01::BERewardInt_GetEventBestPlace,
    };

    pub const C001589B0: Callee = Callee {
        addr: 0x001589b0,
        f: ee::s01::BERewardInt_GetCollectibleTier,
    };

    pub const C00158A50: Callee = Callee {
        addr: 0x00158a50,
        f: ee::s01::BERewardInt_GetMissionTier,
    };

    pub const C00158AF0: Callee = Callee {
        addr: 0x00158af0,
        f: ee::s01::BERewardInt_HasPointsTier,
    };

    pub const C00158C80: Callee = Callee {
        addr: 0x00158c80,
        f: ee::s01::BERewardInt_GetNextCollectibleThreshold,
    };

    pub const C00158D58: Callee = Callee {
        addr: 0x00158d58,
        f: ee::s01::BERewardInt_GetNextMissionThreshold,
    };

    pub const C00159170: Callee = Callee {
        addr: 0x00159170,
        f: ee::s01::BERewardInt_GrantMedalReward,
    };

    pub const C00159CD0: Callee = Callee {
        addr: 0x00159cd0,
        f: ee::s01::BERewardInt_GrantReward,
    };

    pub const C0015A2E0: Callee = Callee {
        addr: 0x0015a2e0,
        f: ee::s01::BERewardInt_IsRewardGranted,
    };

    pub const C0016C6B0: Callee = Callee {
        addr: 0x0016c6b0,
        f: ee::s02::ActiveCameraTriggerList_Find,
    };

    pub const C0016C718: Callee = Callee {
        addr: 0x0016c718,
        f: ee::s02::ActiveCameraTriggerList_Add,
    };

    pub const C00176890: Callee = Callee {
        addr: 0x00176890,
        f: ee::s02::CameraTriggerMan_CollectOctreeNodeTriggers,
    };

    pub const C0017F500: Callee = Callee {
        addr: 0x0017f500,
        f: ee::s02::DebugMenuModule_DrawCursorTrail,
    };

    pub const C0019E2B0: Callee = Callee {
        addr: 0x0019e2b0,
        f: ee::s03::BoltFile_GetData,
    };

    pub const C0019F878: Callee = Callee {
        addr: 0x0019f878,
        f: ee::s03::FEModule_RiderPreviewFindTexture,
    };

    pub const C001BB9B0: Callee = Callee {
        addr: 0x001bb9b0,
        f: ee::s03::cFEStateCreateStep2_GetDaysInMonth,
    };

    pub const C001C84A8: Callee = Callee {
        addr: 0x001c84a8,
        f: ee::s03::cFEPopupConfirm_MeasureLine,
    };

    pub const C001C8568: Callee = Callee {
        addr: 0x001c8568,
        f: ee::s03::cFEPopupConfirm_MeasureLineW,
    };

    pub const C001C9B68: Callee = Callee {
        addr: 0x001c9b68,
        f: ee::s03::cFEPopupConfirm_AccumulateSize,
    };

    pub const C001CB9B0: Callee = Callee {
        addr: 0x001cb9b0,
        f: ee::s04::cKeyboardPopup_MoveDown,
    };

    pub const C001CBC48: Callee = Callee {
        addr: 0x001cbc48,
        f: ee::s04::cKeyboardPopup_MoveDownFromTopRow,
    };

    pub const C001CBCE8: Callee = Callee {
        addr: 0x001cbce8,
        f: ee::s04::cKeyboardPopup_MoveDownFromBottomRow,
    };

    pub const C001CBFC0: Callee = Callee {
        addr: 0x001cbfc0,
        f: ee::s04::cKeyboardPopup_MoveDownFromRow4,
    };

    pub const C001CE560: Callee = Callee {
        addr: 0x001ce560,
        f: ee::s04::cKeyboardPopup_IsKeyEnabled,
    };

    pub const C001D32E0: Callee = Callee {
        addr: 0x001d32e0,
        f: ee::s04::cFEStateMountainRoom_ShowImage,
    };

    pub const C001E1A30: Callee = Callee {
        addr: 0x001e1a30,
        f: ee::s04::MessageCenter_MatchCharacter,
    };

    pub const C001E3388: Callee = Callee {
        addr: 0x001e3388,
        f: ee::s04::MessageCenter_FindMessage,
    };

    pub const C001E91A8: Callee = Callee {
        addr: 0x001e91a8,
        f: ee::s04::HUD_GetElemColor,
    };

    pub const C001E91F8: Callee = Callee {
        addr: 0x001e91f8,
        f: ee::s04::HUD_GetElemPos,
    };

    pub const C001E9290: Callee = Callee {
        addr: 0x001e9290,
        f: ee::s04::HUD_GetElemScale,
    };

    pub const C001F10F8: Callee = Callee {
        addr: 0x001f10f8,
        f: ee::s04::HUD_AlignElemPos,
    };

    pub const C0020E900: Callee = Callee {
        addr: 0x0020e900,
        f: ee::s05::cOVStateOverlay_Update,
    };

    pub const C00224C78: Callee = Callee {
        addr: 0x00224c78,
        f: ee::s05::LunoVMRegister_ReadPtr,
    };

    pub const C00224DA0: Callee = Callee {
        addr: 0x00224da0,
        f: ee::s05::cLunoTable_ctor,
    };

    pub const C00224DF0: Callee = Callee {
        addr: 0x00224df0,
        f: ee::s05::cLunoTable_dtor,
    };

    pub const C00225068: Callee = Callee {
        addr: 0x00225068,
        f: ee::s05::cLunoTable_Assign,
    };

    pub const C002259A8: Callee = Callee {
        addr: 0x002259a8,
        f: ee::s05::cLunoTable_Read,
    };

    pub const C00227F80: Callee = Callee {
        addr: 0x00227f80,
        f: ee::s05::MCManager_Get,
    };

    pub const C00231F80: Callee = Callee {
        addr: 0x00231f80,
        f: ee::s06::LoadIndicator_Advance,
    };

    pub const C00238BF8: Callee = Callee {
        addr: 0x00238bf8,
        f: ee::s06::GameModeMan_SortByTime,
    };

    pub const C0026D4D8: Callee = Callee {
        addr: 0x0026d4d8,
        f: ee::s07::RunList_Seek,
    };

    pub const C0026DB88: Callee = Callee {
        addr: 0x0026db88,
        f: ee::s07::Replay_RestoreSnapshot,
    };

    pub const C0026E448: Callee = Callee {
        addr: 0x0026e448,
        f: ee::s07::Replay_FindSnapshot,
    };

    pub const C0026F4A0: Callee = Callee {
        addr: 0x0026f4a0,
        f: ee::s07::Replay_SetState,
    };

    pub const C002702F8: Callee = Callee {
        addr: 0x002702f8,
        f: ee::s07::Replay_SeekToSnapshot,
    };

    pub const C002703F0: Callee = Callee {
        addr: 0x002703f0,
        f: ee::s07::Replay_StepFrame,
    };

    pub const C00275498: Callee = Callee {
        addr: 0x00275498,
        f: ee::s07::ScriptAnimBankMan_dtor,
    };

    pub const C002758C0: Callee = Callee {
        addr: 0x002758c0,
        f: ee::s07::DataInstanceMan_dtor,
    };

    pub const C00275B98: Callee = Callee {
        addr: 0x00275b98,
        f: ee::s07::DataInstanceMan_AddBigEntries,
    };

    pub const C00275CF8: Callee = Callee {
        addr: 0x00275cf8,
        f: ee::s07::DataInstanceMan_AllocInfo,
    };

    pub const C00278308: Callee = Callee {
        addr: 0x00278308,
        f: ee::s07::ScriptEngine_UnloadMaster,
    };

    pub const C00278E50: Callee = Callee {
        addr: 0x00278e50,
        f: ee::s07::ScriptEngine_ListAddChoice,
    };

    pub const C00278F38: Callee = Callee {
        addr: 0x00278f38,
        f: ee::s07::ScriptEngine_ListPrepare,
    };

    pub const C00278F68: Callee = Callee {
        addr: 0x00278f68,
        f: ee::s07::ScriptEngine_ListStart,
    };

    pub const C00281ED8: Callee = Callee {
        addr: 0x00281ed8,
        f: ee::s07::ScriptEngine_ScriptDbDtor,
    };

    pub const C00283228: Callee = Callee {
        addr: 0x00283228,
        f: ee::s07::ScriptObj_dtor,
    };

    pub const C00283518: Callee = Callee {
        addr: 0x00283518,
        f: ee::s07::ScriptSoundBankMan_dtor,
    };

    pub const C002837C8: Callee = Callee {
        addr: 0x002837c8,
        f: ee::s07::ScriptMovie_dtor,
    };

    pub const C0028B180: Callee = Callee {
        addr: 0x0028b180,
        f: ee::s07::SSXAudioSystem_GetInstance,
    };

    pub const C0028BB10: Callee = Callee {
        addr: 0x0028bb10,
        f: ee::s07::SndBankMan_dtor,
    };

    pub const C0028E8C0: Callee = Callee {
        addr: 0x0028e8c0,
        f: ee::s07::SSXAudioSystem_HandleMusicEvent,
    };

    pub const C002913D8: Callee = Callee {
        addr: 0x002913d8,
        f: ee::s07::SSXAudioSystem_StopAmbienceLoop,
    };

    pub const C003457C8: Callee = Callee {
        addr: 0x003457c8,
        f: ee::s11::ParticleModifier_BaseCtor_2,
    };

    pub const C00358020: Callee = Callee {
        addr: 0x00358020,
        f: ee::s11::MultiParticle_CreateEmitter,
    };

    pub const C00362478: Callee = Callee {
        addr: 0x00362478,
        f: ee::s12::GsPkt_AddAlpha,
    };

    pub const C003625C0: Callee = Callee {
        addr: 0x003625c0,
        f: ee::s12::GsPkt_AddClamp,
    };

    pub const C00362660: Callee = Callee {
        addr: 0x00362660,
        f: ee::s12::GsPkt_AddZbuf,
    };

    pub const C003626D8: Callee = Callee {
        addr: 0x003626d8,
        f: ee::s12::GsPkt_AddTest,
    };

    pub const C003627A8: Callee = Callee {
        addr: 0x003627a8,
        f: ee::s12::GsPkt_Finish,
    };

    pub const C00364CD0: Callee = Callee {
        addr: 0x00364cd0,
        f: ee::s12::DeferMan_EmitMaterialSetup,
    };

    pub const C00366CE0: Callee = Callee {
        addr: 0x00366ce0,
        f: ee::s12::VramHeap_Alloc,
    };

    pub const C00368170: Callee = Callee {
        addr: 0x00368170,
        f: ee::s12::TexMan_EmitImageUpload,
    };

    pub const C003684F0: Callee = Callee {
        addr: 0x003684f0,
        f: ee::s12::TexMan_EmitClutUpload,
    };

    pub const C00368970: Callee = Callee {
        addr: 0x00368970,
        f: ee::s12::TexMan_EmitTex0,
    };

    pub const C0038F460: Callee = Callee {
        addr: 0x0038f460,
        f: ee::s12::SprStream_BeginBlock,
    };

    pub const C0038F668: Callee = Callee {
        addr: 0x0038f668,
        f: ee::s12::SprStream_EndBlockAligned,
    };

    pub const C0039C870: Callee = Callee {
        addr: 0x0039c870,
        f: ee::s13::UIScreen_StartThread,
    };

    pub const C0039FF78: Callee = Callee {
        addr: 0x0039ff78,
        f: ee::s13::UIObject_GetEventThread,
    };

    pub const C003B1300: Callee = Callee {
        addr: 0x003b1300,
        f: ee::s13::MPEG2_Get_Bits,
    };

    pub const C003B2360: Callee = Callee {
        addr: 0x003b2360,
        f: ee::s13::MPEG2_marker_bit,
    };
}

/// Cross-shard call targets (the generator emits `call_indirect` for them).
pub(crate) mod table {

    pub(crate) use crate::game::ai::computer::table::STR_HASH;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {

    pub(crate) use crate::game::fe::festaterewards::table_callee::GET_HASH_VALUE32;

    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_RICH_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {

    pub(crate) use crate::game::fe::feasyncfile::addr::EA_MEM_FREE;
}

// ---------------------------------------------------------------------------
// Front-end popup confirm.

/// `float cFEPopupConfirm_GetCenteredX(cFEPopupConfirm *this)` (0x1c7be0):
/// if the override at +0x2cc is set, the centred X is 0; otherwise it is
/// `((640.0 - this->w) * 0.5) - this->x` (the fields at +0x260 and +0x1b8).
pub fn c_fe_popup_confirm_get_centered_x(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    c.cycles += 3;
    c.g[V0][0] = this.off(0x2cc).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        // mtc1 $zero -> f0
        c.f[0] = 0;
        abi::ret(c);
    } else {
        c.cycles += 10;
        let w = this.off(0x260).read_fpu(c);
        abi::set_fpr(c, 2, w);
        // mtc1 $at (=0x44200000 = 640.0) -> f0
        let mut f0 = FpuF32(0x4420_0000);
        // mtc1 $at (=0x3f000000 = 0.5) -> f3
        let half = FpuF32(0x3f00_0000);
        c.g[AT][0] = 0x3f00_0000;
        f0 = f0 - w;
        let x = this.off(0x1b8).read_fpu(c);
        abi::set_fpr(c, 1, x);
        f0 = f0 * half;
        abi::set_fpr(c, 3, half);
        f0 = f0 - x;
        abi::set_fpr(c, 0, f0);
        abi::ret(c);
    }
}

/// `void cFEPopupConfirm_PositionButtons(cFEPopupConfirm *this)` (0x1c88c8):
/// when `this->+0x70` holds a button container, adds the two offsets at +0x290
/// and +0x294 to the pair at the loaded doubleword near +0x1ac and copies the
/// result (plus the word at +0x1b4) into the container at +0x44.
pub fn c_fe_popup_confirm_position_buttons(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    c.cycles += 3;
    let child = this.off(0x70).lw(c);
    c.g[V0][0] = child;
    c.g[SP][0] = sx(abi::r32(c, SP).wrapping_sub(0x10));
    if child != 0 {
        c.cycles += 20;
        let sp = Addr(abi::r32(c, SP));
        let dword = this.off(0x1ac).read_u64_unaligned(c);
        c.g[V1][0] = dword;
        let extra = this.off(0x1b4).lw(c);
        c.g[A1][0] = extra;
        sp.write_u64_unaligned(c, dword);
        sp.off(8).sw(c, extra);
        let f3v = this.off(0x290).read_fpu(c);
        abi::set_fpr(c, 3, f3v);
        let f2v = this.off(0x294).read_fpu(c);
        abi::set_fpr(c, 2, f2v);
        let f1r = sp.off(4).read_fpu(c);
        let f0r = sp.read_fpu(c);
        let f1 = f1r + f2v;
        abi::set_fpr(c, 1, f1);
        let f0 = f0r + f3v;
        abi::set_fpr(c, 0, f0);
        sp.off(4).write_fpu(c, f1);
        sp.write_fpu(c, f0);
        let out = sp.read_u64_unaligned(c);
        c.g[V1][0] = out;
        let extra2 = sp.off(8).lw(c);
        c.g[A0][0] = extra2;
        Addr(child as u32).off(0x44).write_u64_unaligned(c, out);
        Addr(child as u32).off(0x4c).sw(c, extra2);
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(abi::r32(c, SP).wrapping_add(0x10));
    c.pc = t;
}

/// `void cFEPopupConfirm_AccumulateSize(void *unused, float *dst, const float *src)`
/// (0x1c9b68): `if (dst[0] < src[2]) dst[0] = src[2]; dst[1] += src[3];`.
pub fn c_fe_popup_confirm_accumulate_size(c: &mut Cpu) {
    let dst = Addr(abi::arg(c, 1));
    let src = Addr(abi::arg(c, 2));
    c.cycles += 6;
    let f1 = src.off(8).read_fpu(c);
    abi::set_fpr(c, 1, f1);
    let f0 = dst.read_fpu(c);
    abi::set_fpr(c, 0, f0);
    if abi::c_lt_s(c, f0, f1) {
        dst.write_fpu(c, f1);
    }
    c.cycles += 5;
    let f1b = src.off(0xc).read_fpu(c);
    abi::set_fpr(c, 1, f1b);
    let f0b = dst.off(4).read_fpu(c) + f1b;
    abi::set_fpr(c, 0, f0b);
    dst.off(4).write_fpu(c, f0b);
    abi::ret(c);
}

/// `int cFEPopupConfirm_GetSelection(cFEPopupConfirm *this)` (0x1ca150): if
/// the current selection at +0x6c is set, returns its byte at +0x95 (or one
/// less when the +0x30c sentinel is -1); otherwise the id at +0x168.
pub fn c_fe_popup_confirm_get_selection(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    c.cycles += 3;
    let sel = this.off(0x6c).lw(c);
    c.g[A1][0] = sel;
    if sel != 0 {
        let sentinel = this.off(0x30c).lw(c);
        c.g[V1][0] = sentinel;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        let is_minus_one = sentinel == c.g[V0][0];
        c.cycles += 3;
        let byte = Addr(sel as u32).off(0x95).lbu(c);
        c.g[A1][0] = byte;
        c.cycles += 2;
        if is_minus_one {
            c.g[V0][0] = byte;
        } else {
            c.g[V0][0] = sx((byte as u32).wrapping_sub(1));
        }
    } else {
        c.cycles += 2;
        c.g[V0][0] = this.off(0x168).lw(c);
    }
    abi::ret(c);
}

/// `void cKeyboardPopup_SetMinLength(cKeyboardPopup *this, int min)` (0x1cd0b0):
/// same as [`c_keyboard_popup_set_max_length`] but the field is at +0x64.
pub fn c_keyboard_popup_set_min_length(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    let min = c.g[A1][0];
    c.cycles += 2;
    let non_negative = (min as i64) >= 0;
    c.g[V0][0] = ((min as i64) < 64) as u64;
    if non_negative {
        c.cycles += 2;
        let in_range = c.g[V0][0] != 0;
        c.g[V0][0] = 0x3f;
        c.cycles += 2;
        if in_range {
            this.off(0x64).sw(c, min);
        } else {
            this.off(0x64).sw(c, c.g[V0][0]);
        }
    } else {
        c.cycles += 2;
        this.off(0x64).sw(c, 0);
    }
    abi::ret(c);
}

/// `int cKeyboardPopup_IsKeyEnabled(cKeyboardPopup *this, int index, int *out)`
/// (0x1ce560): picks the enabled flag for `index` from the +0x148/+0x298
/// tables (or returns the table word when `out` is null), combining it with
/// the popup's own enabled state.
pub fn c_keyboard_popup_is_key_enabled(c: &mut Cpu) {
    let self_ptr = c.g[A0][0];
    let index = c.g[A1][0];
    let out = c.g[A2][0];
    c.cycles += 5;
    c.g[A3][0] = self_ptr;
    let obj = Addr(self_ptr as u32);
    c.g[V1][0] = obj.off(0x68).lw(c);
    c.g[V0][0] = obj.off(0x6c).lw(c);
    c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V1][0] ^= 1;
    } else {
        c.cycles += 1;
        c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
    }
    c.cycles += 4;
    c.g[V0][0] = sx((index as u32).wrapping_sub(0xf));
    c.g[V0][0] = (c.g[V0][0] < 0xc) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = obj.off(0x6c).lw(c);
        c.g[V1][0] = c.g[V0][0] ^ 1;
    }
    c.cycles += 2;
    c.g[A0][0] = sx((index as u32) << 2);
    if c.g[V1][0] == 0 {
        c.cycles += 2;
        if out != 0 {
            c.cycles += 3;
            c.g[V0][0] = sx((self_ptr as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x148).lw(c);
            Addr(out as u32).sw(c, c.g[V1][0]);
        }
        c.cycles += 3;
        c.g[V1][0] = sx((self_ptr as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x298).lw(c);
        abi::ret(c);
    } else {
        c.cycles += 2;
        if out != 0 {
            c.cycles += 3;
            c.g[V0][0] = sx((self_ptr as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x298).lw(c);
            Addr(out as u32).sw(c, c.g[V1][0]);
        }
        c.cycles += 3;
        c.g[V1][0] = sx((self_ptr as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x148).lw(c);
        abi::ret(c);
    }
}

/// `cFEPopupConfirm_v24(this, a1, a2)` 0x1ca4f8: forward the widget action.
pub fn c_fe_popup_confirm_v24(c: &mut Cpu) {
    widget_action_v24(c, 0x001c_a518)
}

/// `void cFEPopupConfirm_HideWidget(cFEPopupConfirm* self, Widget* w)`
/// (0x1ca488): forward `Show(false)` to the widget's virtual method
/// (vtable slot +0x4c, receiver offset at +0x48).
pub fn fe_popup_confirm_hide_widget(c: &mut Cpu) {
    hide_show_widget(c, 0, 0x001c_a4b0);
}

/// `void cFEPopupConfirm_ShowWidget(cFEPopupConfirm* self, Widget* w)`
/// (0x1ca4c0): forward `Show(true)` to the widget's virtual method.
pub fn fe_popup_confirm_show_widget(c: &mut Cpu) {
    hide_show_widget(c, 1, 0x001c_a4e8);
}

/// The shared body of the `*_HideWidget` / `*_ShowWidget` forwarders: call
/// the widget's virtual `Show(visible)` (slot +0x4c, `this` adjusted by the
/// signed halfword at +0x48).
pub(crate) fn hide_show_widget(c: &mut Cpu, visible: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A1][0];
        let null = c.g[A2][0] == 0;
        frame.save_ra(c, 0);
        if !null {
            c.cycles += 6;
            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).lw(c);
            c.g[A1][0] = visible;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, ret)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cKeyboardPopup_OnWidgetAction(cKeyboardPopup* self, int, unsigned act)`
/// (0x1cb290): a widget action of 5 or 6 closes the popup's top menu states.
pub fn ckeyboard_popup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = (c.g[A2][0] < 7u64) as u64;
        let below_seven = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if below_seven {
            c.cycles += 3;
            c.g[V0][0] = (c.g[A2][0] < 5u64) as u64;
            let below_five = c.g[V0][0] != 0;
            if below_five {
                frame.restore_ra(c, 0);
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            c.cycles += 4;
            c.g[A0][0] = Addr(abi::arg(c, 0)).off(0x10).lw(c);
            c.g[A1][0] = 1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            callee::UI_STATE_MGR_CLOSE_TOP_STATES.call(c, 0x001c_b2bc)?;
        }
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void cFEPopupConfirm_SetTextMeasured(self, cUIText* text, const char* str,
/// int flags)` (0x1C9B28): size the confirm popup's text box for the string,
/// then set the ASCII string on it.
pub fn fe_popup_confirm_set_text_measured(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        callee::POPUP_CONFIRM_GET_TEXT_BOX_SIZE.call(c, 0x001c_9b44)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::UI_TEXT_SET_STRING.call(c, 0x001c_9b50)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cKeyboardPopup_EnsureCursorOnEnabledKey(Popup* p)` (0x1ce520):
/// when the key under the cursor is disabled, move down one row.
pub fn c_keyboard_popup_ensure_cursor_on_enabled_key(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[A1][0] = lw(c, this, 0x140);
        callee::KEYBOARD_POPUP_IS_KEY_ENABLED.call(c, 0x001c_e53c)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            callee::KEYBOARD_POPUP_MOVE_DOWN.call(c, 0x001c_e550)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// The `v24` widget-forwarding popups.

/// `cKeyboardPopup_v24`: the keyboard popup's virtual method 24 wrapper: when
/// the popup is open (`+0x438`) and owns a widget (`+0x20`), forward to the
/// widget's own slot 24 (vtable at `widget+8`, delta at `+0xc0`, method at
/// `+0xc4`); otherwise return 0.
pub fn c_keyboard_popup_v24(c: &mut Cpu) {
    guest(c, |c| widget_v24(c, 0x438, 0x001c_b404))
}

/// See [`c_keyboard_popup_v24`]: `flag_off` is the "popup is open" field.
pub(crate) fn widget_v24(c: &mut Cpu, flag_off: u32, ret: u32) -> Flow {
    c.cycles += 5;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    let self_ = abi::r32(c, A0);
    c.g[V0][0] = Addr(self_.wrapping_add(flag_off)).lw(c);
    let closed = c.g[V0][0] == 0;
    c.g[V0][0] = 0;
    if closed {
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        return Ok(());
    }
    c.cycles += 3;
    c.g[A3][0] = Addr(self_.wrapping_add(0x20)).lw(c);
    let no_widget = c.g[A3][0] == 0;
    frame.restore_ra(c, 0);
    if no_widget {
        c.cycles += 2;
        frame.close(c);
        return Ok(());
    }
    c.cycles += 7;
    let widget = abi::r32(c, A3);
    c.g[V1][0] = Addr(widget.wrapping_add(8)).lw(c);
    let vtable = c.g[V1][0] as u32;
    c.g[A0][0] = Addr(vtable.wrapping_add(0xc0)).lh(c);
    c.g[V0][0] = Addr(vtable.wrapping_add(0xc4)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx(widget.wrapping_add(abi::r32(c, A0)));
    call_ptr(c, target, ret)?;
    frame.restore_ra(c, 0);
    c.cycles += 2;
    frame.close(c);
    Ok(())
}

/// `void cFEPopupConfirm_GetWidgetSize(self, widget, out)` (0x1c8628): ask the
/// widget for its size (vtable 0xa0/0xa4, written through the stack
/// argument), then store the two floats at `out+8`/`out+0xc`.
pub fn c_fe_popup_confirm_get_widget_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A2][0];
        let widget = c.g[A0][0] as u32;
        c.g[V0][0] = Addr(widget).off(8).lw(c);
        let vt = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(vt).off(0xa0).lh(c);
        c.g[A2][0] = Addr(vt).off(0xa4).lw(c);
        let target = c.g[A2][0] as u32;
        c.g[RA][0] = sx(0x001c_8654);
        c.g[A0][0] = sx(widget.wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x001c_8654)?;
        c.lwc1(0, c.g[SP][0] as u32);
        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(4));
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(8));
        c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
        frame.restore(c, S0, 0x20);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        frame.close(c);
        Ok(())
    })
}

/// `void cFEPopupConfirm_Update(self)` (0x1ca528): if the popup has a body
/// widget (`+0x2c0`), let it lay itself out (vtable slot 0x60/0x64), then run
/// the base UI update.
pub fn c_fe_popup_confirm_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(this).off(0x2c0).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.g[A1][0] = Addr(this).off(0x20).lw(c);
            let widget = c.g[A1][0] as u32;
            c.g[V0][0] = Addr(widget).off(8).lw(c);
            let vt = c.g[V0][0] as u32;
            c.g[A0][0] = Addr(vt).off(0x60).lh(c);
            c.g[V1][0] = Addr(vt).off(0x64).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001c_a55c);
            c.g[A0][0] = sx(widget.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001c_a55c)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_e510, 0x001c_a564)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Keyboard popup: enabled-key steppers.

/// The body of the three steppers below: start at `cur + first`, add `step`
/// while `KeyboardPopup_IsKeyEnabled(popup, index, 0)` is false and store the
/// found index at +0x140. `head` is the event-check address the original
/// resumes at; `call_ret` the return address after the enabled check.
pub(crate) fn keyboard_popup_step(
    c: &mut Cpu,
    first: i32,
    step: u32,
    head: u32,
    call_ret: u32,
) -> Flow {
    if c.pc != head {
        c.cycles += 8;
        let sp = sp(c).wrapping_sub(0x30);
        c.g[SP][0] = sx(sp);
        c.sq(16, p(sp, 0x20));
        c.sq(17, p(sp, 0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sd(p(sp, 0), c.g[RA][0]);
        let self_ = r32(c, A0);
        c.g[V0][0] = c.lw(p(self_, 0x140));
        c.g[S0][0] = sx(r32(c, V0).wrapping_add(first as u32));
    } else {
        c.cycles += 1;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(step));
    }
    loop {
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 0;
        callee::KEYBOARD_POPUP_IS_KEY_ENABLED_2.call(c, call_ret)?;
        let disabled = c.g[V0][0] == 0;
        let sp = sp(c);
        c.g[RA][0] = c.ld(p(sp, 0));
        if disabled {
            if back_edge_unwinds(c, head) {
                return Ok(());
            }
            c.cycles += 1;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(step));
            continue;
        }
        c.cycles += 5;
        c.sw(p(r32(c, S1), 0x140), c.g[S0][0]);
        c.lq(16, p(sp, 0x20));
        c.lq(17, p(sp, 0x10));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp.wrapping_add(0x30));
        c.pc = ra;
        return Ok(());
    }
}

/// `void cKeyboardPopup_FindEnabledInColumnFromTop(cKeyboardPopup* p)`
/// (0x1cc090): from 0x38 keys above the current index, step down 14 keys at a
/// time to the first enabled one.
pub fn ckeyboard_popup_find_enabled_in_column_from_top(c: &mut Cpu) {
    guest(c, |c| {
        keyboard_popup_step(c, -0x38, 0xe, 0x001c_c0b0, 0x001c_c0c4)
    })
}

/// `void cKeyboardPopup_StepUpToEnabledKey(cKeyboardPopup* p)` (0x1cc140):
/// step up (14 keys) from the current index to the first enabled key.
pub fn ckeyboard_popup_step_up_to_enabled_key(c: &mut Cpu) {
    guest(c, |c| {
        keyboard_popup_step(c, -0xe, 0xffff_fff2, 0x001c_c160, 0x001c_c174)
    })
}

/// `void cKeyboardPopup_StepDownToEnabledKey(cKeyboardPopup* p)` (0x1cc190):
/// step down (14 keys) from the current index to the first enabled key.
pub fn ckeyboard_popup_step_down_to_enabled_key(c: &mut Cpu) {
    guest(c, |c| {
        keyboard_popup_step(c, 0xe, 0xe, 0x001c_c1b0, 0x001c_c1c4)
    })
}

/// `void cFEPopupConfirm_SelectOption(menu, option, idx, max)` (0x1c75c8):
/// clamp `idx + 1` into `[lo, max]` (with `max == -1` meaning `[0, 1]`) and
/// select it in the menu.
pub fn c_fe_popup_confirm_select_option(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = u64::MAX;
        let has_max = c.g[A3][0] != c.g[V0][0];
        let idx = sx(r32(c, A2).wrapping_add(1));
        let lo;
        let mut hi = c.g[A3][0];
        if has_max {
            c.cycles += 1;
            c.g[V1][0] = 1;
            lo = 1u64;
        } else {
            c.cycles += 3;
            c.g[V1][0] = 0;
            c.g[A3][0] = 1;
            lo = 0;
            hi = 1;
        }
        c.cycles += 3;
        c.g[V0][0] = ((idx as i64) < (lo as i64)) as u64;
        let below = c.g[V0][0] != 0;
        c.g[V0][0] = ((hi as i64) < (idx as i64)) as u64;
        c.g[A1][0] = sx(idx as u32);
        if below {
            c.cycles += 2;
            c.g[A1][0] = sx(lo as u32);
        } else {
            c.cycles += 1;
            if c.g[V0][0] != 0 {
                c.g[A1][0] = hi;
            }
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x001c_7610);
        c.g[A1][0] = c.g[A1][0] & 0xff;
        call_ptr(c, callee::UIMENU_SET_SELECTION, 0x001c_7610)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cKeyboardPopup_OnActivate(self, arg)` (0x1cd2f0): step the cursor
/// counter down at its limit, update the cursor, then activate.
pub fn c_keyboard_popup_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, S0)).off(0x134).lw(c);
        c.g[V0][0] = Addr(r32(c, S0)).off(0x60).lw(c);
        let at_limit = c.g[V1][0] == c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        if at_limit {
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, V1).wrapping_sub(1));
            Addr(p(r32(c, S0), 0x134)).sw(c, c.g[V0][0]);
        }
        c.cycles += 10;
        c.g[RA][0] = sx(0x001c_d324);
        c.g[A0][0] = c.g[S0][0];
        callee::KEYBOARD_POPUP_UPDATE_CURSOR.call(c, 0x001c_d324)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x001c_d330);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, callee::UI_STATE_ON_ACTIVATE_2, 0x001c_d330)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cKeyboardPopup_ApplyFontScale(obj, scale)` (0x1ce4c8): save the
/// 12-byte scale vector at `+0x50`, scale the font's x/y size into its
/// `+0x38`/`+0x3c` size.
pub fn c_keyboard_popup_apply_font_scale(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        let obj = r32(c, A1);
        c.ldl(2, p(obj, 0x57));
        c.ldr(2, p(obj, 0x50));
        c.g[V1][0] = Addr(p(obj, 0x58)).lw(c);
        let sp = c.g[SP][0] as u32;
        c.sdl(2, p(sp, 7));
        c.sdr(2, sp);
        c.sw(p(sp, 8), c.g[V1][0]);
        c.g[RA][0] = sx(0x001c_e4f0);
        c.g[A0][0] = c.g[A1][0];
        call_ptr(c, callee::UI_TEXT_GET_FONT, 0x001c_e4f0)?;
        let font = r32(c, V0);
        c.lwc1(2, c.g[SP][0] as u32);
        c.lwc1(3, p(c.g[SP][0] as u32, 4));
        c.lwc1(0, Addr(font).off(0x30).0);
        c.lwc1(1, Addr(font).off(0x34).0);
        c.mul_s(0, 0, 2);
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x10));
        c.mul_s(1, 1, 3);
        c.swc1(0, Addr(font).off(0x38).0);
        c.swc1(1, Addr(font).off(0x3c).0);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(p(c.g[SP][0] as u32, 0x20));
        c.pc = t;
        Ok(())
    })
}

/// `void cUIStateBuyPopup_OnWidgetAction(cUIStateBuyPopup* s, UIWidget* w,
/// int event)` (0x1caf00): when the "purchase" button is released, set the
/// confirmation flag at `+0x6c`.
pub fn cui_state_buy_popup_on_widget_action(c: &mut Cpu) {
    c.cycles += 6;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S1, 0x10);
    frame.save(c, S0, 0x20);
    c.g[S1][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    let mut skip_one = false;
    if c.g[A1][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = 5;
        let not_purchase = c.g[A2][0] != c.g[V0][0];
        frame.restore(c, S0, 0x20);
        if not_purchase {
            skip_one = true;
        } else {
            c.cycles += 6;
            c.g[A0][0] = sx(0x004a_14e8);
            c.g[S0][0] = Addr(p_2(c, A1, 0x38)).lw(c);
            if call_ptr(c, table::STR_HASH, 0x001c_af34).is_err() {
                return;
            }
            if c.g[S0][0] != c.g[V0][0] {
                c.sw(p_2(c, S1, 0x6c), 0);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 1;
                c.sw(p_2(c, S1, 0x6c), c.g[V0][0]);
            }
        }
    }
    if !skip_one {
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void cUIStateBuyPopup_ResetParams(cUIStateBuyPopup* self)` (0x1ca8a0):
/// zero the three 8-byte parameter blocks at +0xc, +0x14, +0x1c and the three
/// words at +0, +4, +8.
pub fn cui_state_buy_popup_reset_params(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 8;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
        call_ptr(c, 0x003e_6448, 0x001c_a8c0)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = 0;
        c.g[A2][0] = 8;
        call_ptr(c, 0x003e_6448, 0x001c_a8d0)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[A1][0] = 0;
        c.g[A2][0] = 8;
        call_ptr(c, 0x003e_6448, 0x001c_a8e0)?;
        sw(c, p_3(c.g[S0][0], 8), 0);
        sw(c, p_3(c.g[S0][0], 0), 0);
        sw(c, p_3(c.g[S0][0], 4), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUIStateBuyPopup_OnWidgetNotify` (0x001caed8): the occlusion key class
/// (`1,1` for keys below 7, `1,0` for 7..9, `1,1` for 10 and up) — 0x101 /
/// 0x100.
pub fn c_ui_state_buy_popup_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    let key = abi::arg64(c, 2);
    c.g[V1][0] = (key < 10) as u64;
    let far = c.g[V1][0] == 0;
    c.g[V0][0] = 0x101;
    if !far {
        c.cycles += 4;
        c.g[A0][0] = (key < 7) as u64;
        c.g[V1][0] = 0x101;
        c.g[V0][0] = 0x100;
        if c.g[A0][0] != 0 {
            c.g[V0][0] = c.g[V1][0];
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int cKeyboardPopup_FindEnabledInColumnFromBottom(cKeyboardPopup* self,
/// int start)` (0x1cc0e0): walk up a key column until an enabled key is
/// found; remember it at `self+0x140`. Dispatcher: 0x1cc110 is a resume entry.
pub fn c_keyboard_popup_find_enabled_in_column_from_bottom(c: &mut Cpu) {
    guest(c, |c| {
        let frame;
        let mut stage;
        if c.pc == 0x001c_c0e0 {
            c.cycles += 7;
            frame = Frame::open(c, 0x30);
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            frame.save(c, S1, 0x10);
            frame.save(c, S0, 0x20);
            c.g[S1][0] = c.g[A0][0];
            let have_start = c.g[A1][0] != c.g[V0][0];
            frame.save_ra(c, 0);
            if have_start {
                c.cycles += 2;
                c.g[S0][0] = c.g[A1][0];
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw_3(c, p_4(c.g[S1][0], 0x140));
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x38));
            }
            stage = 0u32;
        } else if c.pc == 0x001c_c110 {
            frame = Frame { size: 0x30 };
            stage = 1;
        } else {
            let pc = c.pc;
            c.jump_out(pc, 0x001c_c0e0, 0x001c_c140);
            return Ok(());
        }
        loop {
            if stage == 1 {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xe));
            }
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 0;
            call_ptr(c, 0x001c_e560, 0x001c_c124)?;
            let disabled = c.g[V0][0] == 0;
            frame.restore_ra(c, 0);
            if !disabled {
                break;
            }
            if back_edge_unwinds(c, 0x001c_c110) {
                return Ok(());
            }
            stage = 1;
        }
        c.cycles += 5;
        Addr(p_4(c.g[S1][0], 0x140)).sw(c, c.g[S0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cKeyboardPopup_SetDigitKeysDisabled(cKeyboardPopup* self, int on)`
/// (0x1ce408): enable/disable the ten digit keys (indices 0xf..0x18).
pub fn c_keyboard_popup_set_digit_keys_disabled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0xf;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 8;
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S1][0] ^ 1;
            c.g[A3][0] = 1;
            call_ptr(c, 0x001c_e3c8, 0x001c_e440)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 25i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001c_e430);
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cUIStateBuyPopup_Init(void* self)` (0x1cac30).
pub fn cui_state_buy_popup_init(c: &mut Cpu) {
    guest(c, |c| ui_init_screen(c, 0x001c_ac30, 0x0046_62c8))
}

/// `void cKeyboardPopup_dtor(cKeyboardPopup* self, int flags)` (0x1cb138).
pub fn keyboard_popup_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0047_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4138));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(c.g[S0][0] as u32, 8)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x428)).lw(c);
        let no_buf = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_buf {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x001c_b16c)?;
            Addr(p(c.g[S0][0] as u32, 0x428)).sw(c, 0);
        }
        c.cycles += 11;
        c.g[A0][0] = sx(p(c.g[S0][0] as u32, 0x424));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0031_8260, 0x001c_b17c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_e390, 0x001c_b188)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cKeyboardPopup_Init(void* self)` (0x1cb1a0).
pub fn keyboard_popup_init(c: &mut Cpu) {
    guest(c, |c| ui_init_screen(c, 0x001c_b1a0, 0x0046_6388))
}

/// `void cKeyboardPopup_MaskText(cKeyboardPopup* self, const char* str)`
/// (0x1cce98): fill the text field with `*` and terminate it.
pub fn keyboard_popup_mask_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        call_ptr(c, 0x0041_6810, 0x001c_ceb0)?;
        c.g[A0][0] = c.g[V0][0];
        let empty = (c.g[A0][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !empty {
            c.cycles += 3;
            c.g[A2][0] = sx(p(c.g[S0][0] as u32, 0xf4));
            c.g[A3][0] = 0x2a;
            loop {
                c.cycles += 7;
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.sb(c.g[V1][0] as u32, c.g[A3][0]);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001c_cec8);
                }
            }
        }
        c.cycles += 6;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lq(S0, p(c.g[SP][0] as u32, 0x10));
        c.sb(p(c.g[V0][0] as u32, 0xf4), 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

/// `void cKeyboardPopup_SetStaticTextId(cKeyboardPopup* self, int id)`
/// (0x1cd020).
pub fn keyboard_popup_set_static_text_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x40)).lw(c);
        let no_screen = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_screen {
            c.cycles += 2;
            Addr(p(c.g[S0][0] as u32, 0x42c)).sw(c, c.g[S1][0]);
        } else {
            c.cycles += 8;
            c.g[A0][0] = 0x0046_0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6580));
            call_ptr(c, 0x0031_7670, 0x001c_d054)?;
            c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001c_d060)?;
            let missing = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !missing {
                c.cycles += 2;
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x003a_1000, 0x001c_d070)?;
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateTrophyRoom_CountEventEntries(int a, int b)` (0x1ce6f0): count
/// the leading non-negative ids in the 5-entry event table.
pub fn cfe_state_trophy_room_count_event_entries(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0xa;
    c.g[A2][0] = 0x28;
    c.mult(0, 5, 5, 2);
    c.mult(0, 4, 4, 6);
    c.g[V1][0] = 0x0046_0000;
    c.g[A3][0] = 0;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5528));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = c.lb(c.g[A1][0] as u32);
    let negative = (c.g[V0][0] as i64) < 0;
    c.g[A2][0] = 0;
    if !negative {
        c.cycles += 2;
        c.g[A3][0] = 0x1;
        loop {
            c.cycles += 4;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[A2][0] as i64) < 5) as u64;
            let done = c.g[V0][0] == 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(2));
            if done {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lb(c.g[A1][0] as u32);
            if (c.g[V0][0] as i64) < 0 {
                break;
            }
            c.cycles += 2;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            if event_due(c) {
                deliver_event_here(c, 0x001c_e728);
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = c.g[A3][0];
    c.pc = ra;
}

/// `cUIStateBuyPopup_SetUberTrickParams(popup, trick, a2, a3)` 0x1caf58:
/// remember the four parameters and the two hashed labels for the buy popup.
pub fn c_ui_state_buy_popup_set_uber_trick_params(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        frame.save_ra(c, 0);
        c.g[V0][0] = 2;
        c.g[S0][0] = this64;
        c.g[A0][0] = 0x460000;
        Addr(p(this, 0x48)).sw(c, c.g[A2][0]);
        c.g[A0][0] = sx(0x0046_6338);
        Addr(p(this, 0x4c)).sw(c, c.g[A3][0]);
        c.g[S1][0] = 3;
        Addr(p(this, 0x58)).sw(c, c.g[A1][0]);
        Addr(p(this, 0x54)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7670, 0x001c_af90)?;
        Addr(p(this, 0x5c)).sw(c, c.g[S1][0]);
        c.g[A0][0] = 0x460000;
        Addr(p(this, 0x60)).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx(0x0046_6350);
        call_ptr(c, 0x0031_7670, 0x001c_afa4)?;
        Addr(p(this, 0x64)).sw(c, c.g[S1][0]);
        Addr(p(this, 0x68)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEPopupConfirm_BaseDtor(this, flags)` 0x1c5a90: restore the base vtable,
/// release the two label strings and the popup configuration, then run the UI
/// state destructor.
pub fn c_fe_popup_confirm_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        c.g[V0][0] = sx(0x0046_cbd8);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = this64;
        frame.save_ra(c, 0);
        c.g[S1][0] = abi::arg64(c, 1);
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx(p(this, 0x2b4));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001c_5ac0)?;
        c.g[A0][0] = sx(p(this, 0x2b0));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001c_5acc)?;
        c.g[A0][0] = sx(p(this, 0xbc));
        c.g[A1][0] = 2;
        call_ptr(c, 0x001c_5750, 0x001c_5ad8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_e390, 0x001c_5ae4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEPopupConfirm_v04(self)` (0x1c5af8): create the popup screen and
/// start it, marking the "popup" flag.
pub fn c_fe_popup_confirm_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c40));
        c.g[S0][0] = Addr(p(c.g[S1][0] as u32, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001c_5b1c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001c_5b30)?;
        c.g[A0][0] = c.g[V0][0];
        let missing = c.g[A0][0] == 0;
        Addr(p(c.g[S1][0] as u32, 0x40)).sw(c, c.g[A0][0]);
        if !missing {
            c.cycles += 5;
            c.g[V0][0] = 1;
            c.g[A1][0] = 0;
            Addr(p(c.g[S1][0] as u32, 0x2b8)).sw(c, c.g[V0][0]);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001c_5b50)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUIStateBuyPopup::SetAttribParams(self, a1, value, label)` (0x1cafc0):
/// fill the two labelled parameter rows and their hashes.
pub fn c_ui_state_buy_popup_set_attrib_params(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = 0x4a0000;
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1408));
        c.g[S2][0] = 0x2;
        Addr(p(c.g[S0][0] as u32, 0x58)).sw(c, c.g[S1][0]);
        Addr(p(c.g[S0][0] as u32, 0x54)).sw(c, c.g[S2][0]);
        c.g[A0][0] = 0x460000;
        Addr(p(c.g[S0][0] as u32, 0x48)).sw(c, c.g[A2][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6368));
        Addr(p(c.g[S0][0] as u32, 0x4c)).sw(c, c.g[A3][0]);
        call_ptr(c, 0x0031_7670, 0x001c_b000)?;
        c.g[V1][0] = 0x3;
        Addr(p(c.g[S0][0] as u32, 0x64)).sw(c, c.g[S2][0]);
        Addr(p(c.g[S0][0] as u32, 0x68)).sw(c, c.g[S1][0]);
        Addr(p(c.g[S0][0] as u32, 0x60)).sw(c, c.g[V0][0]);
        Addr(p(c.g[S0][0] as u32, 0x5c)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIStateBuyPopup_SetSongCreditParams(self, song, credit)`
/// (0x001ca900): fill the buy-popup info with the song's credit price.
pub fn c_ui_state_buy_popup_set_song_credit_params(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[A0][0] = sx(0x460000);
        c.g[S1][0] = c.g[A1][0];
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        c.g[A0][0] = sx(0x460000u32.wrapping_add(0x61b8));
        frame.save_ra(c, 0);
        c.g[S2][0] = 0x3;
        callee::GET_HASH_VALUE_32.call(c, 0x001c_a934)?;
        Addr(at(c, S0, 0x10)).sw(c, c.g[S1][0]);
        c.g[V1][0] = 0x2;
        Addr(at(c, S0, 0x14)).sw(c, c.g[S2][0]);
        c.g[A0][0] = sx(0x460000);
        Addr(at(c, S0, 0xc)).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx(0x460000u32.wrapping_add(0x61c8));
        c.g[RA][0] = sx(0x001c_a954);
        Addr(at(c, S0, 0x18)).sw(c, c.g[V0][0]);
        callee::GET_HASH_VALUE_32.call(c, 0x001c_a954)?;
        Addr(at(c, S0, 0x8)).sw(c, c.g[S3][0]);
        Addr(at(c, S0, 0x1c)).sw(c, c.g[S2][0]);
        Addr(at(c, S0, 0x20)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `lhu`/`lbu`-free byte load helpers used below.
#[inline(always)]
pub(crate) fn lb(c: &mut Cpu, a: u32) -> u64 {
    c.lb(a)
}

/// `void cKeyboardPopup_TypeCurrentKey(cKeyboardPopup* p)` (0x001cc7c8):
/// insert the current keyboard character (the shifted row when the key is
/// shifted) into the popup's text field.
pub fn c_keyboard_popup_type_current_key(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
        c.sd(sp(c), c.g[RA][0]);
        let base = c.g[A0][0] as u32;
        c.g[V0][0] = c.lw(base.wrapping_add(0x68));
        c.g[V1][0] = c.lw(base.wrapping_add(0x6c));
        let no_shift = c.g[V1][0] == 0;
        c.g[V0][0] = (0 < c.g[V0][0]) as u64;
        if no_shift {
            c.cycles += 1;
            c.g[V1][0] = (0 < c.g[V0][0]) as u64;
        } else {
            c.cycles += 2;
            c.g[V1][0] = c.g[V0][0] ^ 1;
        }
        c.cycles += 5;
        c.g[A1][0] = c.lw(base.wrapping_add(0x140));
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf));
        c.g[V0][0] = (c.g[V0][0] < 0xc) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = c.lw(base.wrapping_add(0x6c));
            c.g[V1][0] = c.g[V0][0] ^ 1;
        }
        c.cycles += 2;
        let shifted = c.g[V1][0] == 0;
        c.g[V0][0] = sx(0x0044_0000);
        if !shifted {
            c.cycles += 6;
            c.g[V0][0] = sx(0x0044_1390);
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = c.lb(c.g[V0][0] as u32);
            call_ptr(c, 0x001c_c620, 0x001c_c820)?; // cKeyboardPopup_InsertChar
        } else {
            c.cycles += 6;
            c.g[V0][0] = sx(0x0044_13e8);
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = c.lb(c.g[V0][0] as u32);
            call_ptr(c, 0x001c_c620, 0x001c_c83c)?; // cKeyboardPopup_InsertChar
        }
        c.g[RA][0] = c.ld(sp(c));
        c.cycles += 2;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        abi::ret(c);
        Ok(())
    })
}

/// `void cKeyboardPopup_Update(self)` (0x1cb208): update the popup's base
/// state and its key-list widget.
pub fn ckeyboardpopup_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e510, 0x001c_b21c)?;
        c.g[V0][0] = lw_at_2(c, gw(c, -0x204));
        let v0;
        if c.g[V0][0] == 0 {
            v0 = lw_at_2(c, p(r32(c, S0), 0x440));
        } else {
            c.cycles += 3;
            c.g[V0][0] = lw_at_2(c, p(c.g[V0][0] as u32, 0x64));
            if c.g[V0][0] != 0 {
                v0 = lw_at_2(c, p(r32(c, S0), 0x440));
            } else {
                c.cycles += 4;
                c.g[V0][0] = lw_at_2(c, gw(c, -0x848));
                c.g[V1][0] = lw_at_2(c, p(c.g[V0][0] as u32, 0x84));
                let no_race = c.g[V1][0] == 0;
                c.g[A1][0] = 1;
                if !no_race {
                    c.cycles += 3;
                    c.g[A0][0] = lw_at_2(c, p(r32(c, S0), 0x10));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f190, 0x001c_b250)?;
                }
                c.cycles += 1;
                v0 = lw_at_2(c, p(r32(c, S0), 0x440));
            }
        }
        c.g[V0][0] = v0;
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = lw_at_2(c, p(r32(c, S0), 0x20));
            let none = c.g[A1][0] == 0;
            frame.restore(c, 16, 0x10);
            if !none {
                c.cycles += 6;
                c.g[V0][0] = lw_at_2(c, p(c.g[A1][0] as u32, 8));
                let func = vslot(c, c.g[V0][0], 0x60, V1);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, 0x001c_b27c)?;
                frame.restore(c, 16, 0x10);
            }
        } else {
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FE / UI constructors and helpers.

/// `cUIStateBuyPopup* cUIStateBuyPopup_ctor(self, owner, engine, uchar unk)`
/// (0x1caba8): construct the UI state, install the buy-popup vtable and reset
/// the popup parameters; when an owner is attached, propagate the player
/// index (`owner[0]`, a `FEModule*`) into the state's selection byte.
pub fn c_ui_state_buy_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        open(c, 0x40);
        let sp0 = spv(c);
        c.g[A3][0] = sx((c.g[A3][0] as u32) << 24);
        c.sq(16, at_2(sp0, 0x30));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, at_2(sp0, 0x20));
        c.sq(18, at_2(sp0, 0x10));
        c.sd(sp0, c.g[RA][0]);
        c.g[S1][0] = sx((((c.g[A3][0] as u32) as i32) >> 24) as u32);
        callee::UI_STATE_CTOR_2_2.call(c, 0x001c_abcc)?;
        let this = c.g[S0][0];
        c.g[S2][0] = sx(at_2(this as u32, 0x48));
        c.g[V1][0] = 0x470000;
        c.g[A0][0] = c.g[S2][0];
        c.g[V1][0] = sx(at_2(c.g[V1][0] as u32, 0xffff_cb08));
        c.sw(at_2(this as u32, 8), c.g[V1][0]);
        callee::BUY_POPUP_RESET.call(c, 0x001c_abe4)?;
        c.g[V1][0] = 7;
        c.sb(at_2(this as u32, 0x44), c.g[S1][0]);
        c.sw(at_2(this as u32, 0xc), c.g[V1][0]);
        c.g[V0][0] = lw_at_3(c, this, 0x10);
        let owner = c.g[V0][0];
        c.g[A0][0] = lw_at_3(c, owner, 0);
        let no_owner = c.g[A0][0] == 0;
        if !no_owner {
            c.cycles += 3;
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x001a_1cd0, 0x001c_ac08)?;
            c.sb(at_2(this as u32, 0x15), c.g[V0][0]);
        }
        c.cycles += 9;
        c.g[A0][0] = c.g[S2][0];
        callee::BUY_POPUP_RESET.call(c, 0x001c_ac14)?;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, at_2(sp0, 0x20));
        c.lq(16, at_2(sp0, 0x30));
        c.lq(18, at_2(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_2(sp0, 0x40));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end keyboard.

/// `void cKeyboardPopup_RefreshEntryText(cKeyboardPopup* self)`
/// (0x1cc848): refresh the popup's scroll window, look its text object up by
/// the entry's hash name and push the (masked) entry text into it.
pub fn c_keyboard_popup_refresh_entry_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW.call(c, 0x001c_c860)?;
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6530));
        call_ptr(c, 0x0031_7670, 0x001c_c86c)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001c_c878)?;
        c.g[S1][0] = c.g[V0][0];
        if c.g[S1][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x70)).lw(c);
            c.g[A0][0] = c.g[S1][0];
            if c.g[V0][0] != 0 {
                c.cycles += 8;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx(p(r32(c, S0), 0xb4));
                callee::KEYBOARD_POPUP_MASK_TEXT.call(c, 0x001c_c89c)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = sx(p(r32(c, S0), 0xf4));
                call_ptr(c, 0x003a_0c50, 0x001c_c8a8)?;
            } else {
                c.cycles += 2;
                c.g[A1][0] = sx(p(r32(c, S0), 0xb4));
                call_ptr(c, 0x003a_0c50, 0x001c_c8b8)?;
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S0][0];
        callee::KEYBOARD_POPUP_UPDATE_CURSOR_2.call(c, 0x001c_c8c0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEPopupConfirm_SetTextFromDesc(cUIText* text, void* unused,
/// const int* desc)` (0x1c8050): set the popup text from the description
/// record (0 = default string, 1 = rich text, 2 = ASCII, 3 = string id).
pub fn cfe_popup_confirm_set_text_from_desc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
        frame.save_ra(c, 0);
        c.g[V0][0] = 1;
        let kind = c.g[V1][0];
        c.g[A0][0] = c.g[A1][0];
        // 0,1,2,3 = the switch cases, 4 = no matching case.
        let mut case = 4u8;
        if kind == c.g[V0][0] {
            case = 1;
        } else {
            c.cycles += 3;
            c.g[V0][0] = (kind < 2) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 2;
                c.cycles += 2;
                let is_two = kind == c.g[V0][0];
                c.g[V0][0] = 3;
                if is_two {
                    case = 2;
                } else {
                    c.cycles += 2;
                    if kind == c.g[V0][0] {
                        case = 3;
                    } else {
                        c.cycles += 2;
                    }
                }
            } else {
                c.cycles += 2;
                if kind == 0 {
                    case = 0;
                } else {
                    c.cycles += 2;
                }
            }
        }
        match case {
            1 => {
                c.cycles += 4;
                c.g[A1][0] = lw_5(c, A2, 0x4);
                call_ptr(c, table_callee::UI_TEXT_SET_RICH_STRING, 0x001c_80a4)?;
            }
            2 => {
                c.cycles += 4;
                c.g[A1][0] = lw_5(c, A2, 0x4);
                call_ptr(c, table_callee::UI_TEXT_SET_ASCII_STRING, 0x001c_80b4)?;
            }
            3 => {
                c.cycles += 4;
                c.g[A1][0] = lw_5(c, A2, 0x4);
                call_ptr(
                    c,
                    table_callee::UI_TEXT_SET_UNICODE_STRING_BY_ID,
                    0x001c_80c4,
                )?;
            }
            0 => {
                c.cycles += 4;
                c.g[A1][0] = 0x4a0000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                call_ptr(c, table_callee::UI_TEXT_SET_ASCII_STRING, 0x001c_80d8)?;
            }
            _ => {}
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// Shared body of the two `cUIStateBuyPopup_Set*Params` helpers: fill the five
/// hash/parameter slots of the popup state.
pub(crate) fn ui_state_buy_popup_set_params(
    c: &mut Cpu,
    entry: u32,
    first_hash: u32,
    second_hash: u32,
) -> Flow {
    c.cycles += 37;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S0, 0x50);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S1, 0x40);
    frame.save(c, S3, 0x20);
    c.g[A0][0] = 0x460000;
    frame.save(c, S4, 0x10);
    c.g[S1][0] = c.g[A1][0];
    c.g[S4][0] = c.g[A2][0];
    c.g[S3][0] = c.g[A3][0];
    frame.save(c, S2, 0x30);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(first_hash));
    frame.save_ra(c, 0);
    c.g[S2][0] = 3;
    call_ptr(c, table_callee::GET_HASH_VALUE32, entry + 0x3c)?;
    Addr(c.g[S0][0] as u32).off(0x10).sw(c, c.g[S1][0]);
    c.g[V1][0] = 2;
    Addr(c.g[S0][0] as u32).off(0x14).sw(c, c.g[S2][0]);
    c.g[A0][0] = 0x460000;
    Addr(c.g[S0][0] as u32).off(0xc).sw(c, c.g[V1][0]);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(second_hash));
    Addr(c.g[S0][0] as u32).off(0x18).sw(c, c.g[V0][0]);
    call_ptr(c, table_callee::GET_HASH_VALUE32, entry + 0x5c)?;
    c.g[V1][0] = u64::MAX;
    Addr(c.g[S0][0] as u32).off(0x4).sw(c, c.g[S3][0]);
    Addr(c.g[S0][0] as u32).off(0x1c).sw(c, c.g[S2][0]);
    Addr(c.g[S0][0] as u32).sw(c, c.g[S4][0]);
    Addr(c.g[S0][0] as u32).off(0x20).sw(c, c.g[V0][0]);
    Addr(c.g[S0][0] as u32).off(0x8).sw(c, c.g[V1][0]);
    frame.restore(c, S0, 0x50);
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore(c, S4, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void cUIStateBuyPopup_SetMusicTrackParams(self, a1, a2, a3)` (0x1ca980).
pub fn cuistate_buy_popup_set_music_track_params(c: &mut Cpu) {
    guest(c, |c| {
        ui_state_buy_popup_set_params(c, 0x001c_a980, 0x61b8, 0x61e0)
    })
}

/// `void cUIStateBuyPopup_SetItemParams(self, a1, a2, a3)` (0x1cab10).
pub fn cuistate_buy_popup_set_item_params(c: &mut Cpu) {
    guest(c, |c| {
        ui_state_buy_popup_set_params(c, 0x001c_ab10, 0x62b8, 0x61f8)
    })
}

// ---------------------------------------------------------------------------
// On-screen keyboard.

/// `int cKeyboardPopup_MoveDownFromTopRow(KeyboardPopup* popup, int arg)`
/// (0x1cbc48): from the top row (state 4) or the row below it (state 5), move
/// down to the first enabled key: if the key below the current one is enabled
/// select it directly, otherwise scan with `StepDownToEnabledKey`. Returns 1
/// when the selection moved.
pub fn c_keyboard_popup_move_down_from_top_row(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 4;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[V1][0] = lw_6(c, p(c.g[S0][0] as u32, 0x140));
        let not_top = c.g[V1][0] != c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        let mut step = false;
        let mut matched = !not_top;
        if !not_top {
            c.cycles += 5;
            c.g[A1][0] = 0x10;
            c.g[A2][0] = c.g[SP][0];
            call_ptr(c, 0x001c_e560, 0x001c_bc78)?; // IsKeyEnabled
            let disabled = c.g[V0][0] == 0;
            c.g[V0][0] = 0x10;
            if disabled {
                step = true;
            } else {
                c.cycles += 2;
                sw_2(c, p(c.g[S0][0] as u32, 0x140), c.g[V0][0]);
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = 5;
            let below = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 0;
            matched = !below;
            if !below {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x12;
                c.g[A2][0] = c.g[SP][0];
                call_ptr(c, 0x001c_e560, 0x001c_bca4)?;
                let disabled = c.g[V0][0] == 0;
                c.g[V0][0] = 0x12;
                if disabled {
                    step = true;
                } else {
                    c.cycles += 2;
                    sw_2(c, p(c.g[S0][0] as u32, 0x140), c.g[V0][0]);
                }
            }
        }
        if step {
            c.cycles += 3;
            c.g[V0][0] = lw_6(c, c.g[SP][0] as u32);
            let out_empty = c.g[V0][0] == 0;
            c.g[V0][0] = (c.g[S1][0] < 1) as u64;
            if !out_empty {
                c.cycles += 1;
                sw_2(c, p(c.g[S0][0] as u32, 0x144), c.g[V0][0]);
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001c_c190, 0x001c_bccc)?; // StepDownToEnabledKey
        }
        if matched {
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `int cKeyboardPopup_MoveUpFromBottomRow(KeyboardPopup* popup, int arg)`
/// (0x1cbf20): the mirror of `MoveDownFromTopRow`, from state 0x4c/0x4f.
pub fn c_keyboard_popup_move_up_from_bottom_row(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x4c;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[V1][0] = lw_6(c, p(c.g[S0][0] as u32, 0x140));
        let not_bottom = c.g[V1][0] != c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        let mut matched = !not_bottom;
        if !not_bottom {
            c.cycles += 5;
            c.g[A1][0] = 0x42;
            c.g[A2][0] = c.g[SP][0];
            call_ptr(c, 0x001c_e560, 0x001c_bf50)?;
            let enabled = c.g[V0][0] != 0;
            c.g[V0][0] = 0x42;
            if enabled {
                c.cycles += 2;
                sw_2(c, p(c.g[S0][0] as u32, 0x140), c.g[V0][0]);
                c.cycles += 1;
                c.g[V0][0] = 1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = lw_6(c, c.g[SP][0] as u32);
                c.cycles += 2;
                let out_empty = c.g[V0][0] == 0;
                c.g[V0][0] = (c.g[S1][0] < 1) as u64;
                if !out_empty {
                    c.cycles += 1;
                    sw_2(c, p(c.g[S0][0] as u32, 0x144), c.g[V0][0]);
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x001c_c140, 0x001c_bfa4)?;
                c.cycles += 1;
                c.g[V0][0] = 1;
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x4f;
            let other = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 0;
            matched = !other;
            if !other {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x42;
                c.g[A2][0] = c.g[SP][0];
                call_ptr(c, 0x001c_e560, 0x001c_bf7c)?;
                let enabled = c.g[V0][0] != 0;
                c.g[V0][0] = 0x42;
                if enabled {
                    c.cycles += 2;
                    sw_2(c, p(c.g[S0][0] as u32, 0x140), c.g[V0][0]);
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = lw_6(c, c.g[SP][0] as u32);
                    c.cycles += 2;
                    let out_empty = c.g[V0][0] == 0;
                    c.g[V0][0] = (c.g[S1][0] < 1) as u64;
                    if !out_empty {
                        c.cycles += 1;
                        sw_2(c, p(c.g[S0][0] as u32, 0x144), c.g[V0][0]);
                    }
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x001c_c140, 0x001c_bfa4)?;
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                }
            }
        }
        let _ = matched;
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Popup confirm.

/// `void cFEPopupConfirm_v06(self)` (0x1c5b88): with a live popup, fade it
/// out, select its zero-th option and hand its result to the screen.
pub fn cfe_popup_confirm_v06(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        c.cycles += 7;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_7(c, at_4(c.g[S0][0], 0x6c));
        if c.g[V0][0] != 0 {
            c.cycles += 32;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 12);
            c.g[RA][0] = sx(0x001c_5bb4);
            call_ptr(c, 0x001c_5f20, 0x001c_5bb4)?;
            c.g[A2][0] = lw_7(c, at_4(c.g[S0][0], 0x6c));
            c.g[A1][0] = 0;
            c.g[V0][0] = lw_7(c, at_4(c.g[A2][0], 8));
            c.g[A0][0] = lh(c, at_4(c.g[V0][0], 0x40));
            c.g[V1][0] = lw_7(c, at_4(c.g[V0][0], 0x44));
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001c_5bd0);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001c_5bd0)?;
            c.g[A2][0] = lw_7(c, at_4(c.g[S0][0], 0x2c4));
            c.g[A0][0] = c.g[S0][0];
            c.g[A3][0] = lw_7(c, at_4(c.g[S0][0], 0x30c));
            c.g[RA][0] = sx(0x001c_5be4);
            c.g[A1][0] = lw_7(c, at_4(c.g[S0][0], 0x6c));
            call_ptr(c, 0x001c_75c8, 0x001c_5be4)?;
            c.g[A2][0] = lw_7(c, at_4(c.g[S0][0], 0x6c));
            c.g[A1][0] = 1;
            c.g[V0][0] = lw_7(c, at_4(c.g[A2][0], 8));
            c.g[A0][0] = lh(c, at_4(c.g[V0][0], 0x38));
            c.g[V1][0] = lw_7(c, at_4(c.g[V0][0], 0x3c));
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001c_5c00);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001c_5c00)?;
            c.g[T0][0] = lw_7(c, at_4(c.g[S0][0], 8));
            c.g[A2][0] = 5;
            c.g[A1][0] = lw_7(c, at_4(c.g[S0][0], 0x6c));
            c.g[A3][0] = 0;
            c.g[A0][0] = lh(c, at_4(c.g[T0][0], 0xa0));
            c.g[V0][0] = lw_7(c, at_4(c.g[T0][0], 0xa4));
            let target = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x001c_5c20);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001c_5c20)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEPopupConfirm_LayoutPostText` (0x1c7040): re-lay a confirm popup's wrapped
// message text after its text object (`+0x78`) exists: copy the popup's
// position, bump y by `+0x264`, and update the label (and its alignment when
// `+0x2cc` is set). Called with the popup and a 3-vector source.
pub fn cfe_popup_confirm_layout_post_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
        if c.g[V1][0] == 0 {
            c.lq(16, frame.at(c, 0x20));
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
            let empty = c.g[V0][0] == 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4c));
            if !empty {
                c.cycles += 25;
                // v = self->pos; v.y += self->+0x264; text->+0x44 = v.
                c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1bf));
                c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1b8));
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(2, c.g[SP][0] as u32);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.g[AT][0] = 0x41200000;
                c.mtc1(1, 12);
                c.g[A0][0] = c.g[S0][0];
                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x264));
                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x260));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.add_s(0, 0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(8, c.g[SP][0] as u32);
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x4b));
                c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4c), c.g[T1][0]);
                c.g[A3][0] = 0u64;
                callee::POPUP_SET_WRAPPED_TEXT.call(c, 0x001c_70c0)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2cc));
                let no_align = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S0][0];
                if !no_align {
                    c.cycles += 4;
                    c.mtc1(0, 12);
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x78));
                    c.g[A2][0] = 0x11;
                    callee::POPUP_SET_WIDGET_ALIGN.call(c, 0x001c_70dc)?;
                }
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x20));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cKeyboardPopup_SetStaticWideText` (0x1ccf70): cache the static wide text on
// the popup: either clone it through the popup's text object or allocate a
// private copy.
pub fn keyboard_popup_set_static_wide_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
        let has_screen = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if !has_screen {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x428));
            let none = c.g[A0][0] == 0;
            if !none {
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x001c_cfa4)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x428), 0u64);
            }
            c.cycles += 15;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003d_bca0, 0x001c_cfb0)?;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6590));
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x0031_7e30, 0x001c_cfcc)?;
            c.g[A1][0] = c.g[S1][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x428), c.g[V0][0]);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x003d_be20, 0x001c_cfdc)?;
            c.lq(16, frame.at(c, 0x20));
        } else {
            c.cycles += 8;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6580));
            call_ptr(c, 0x0031_7670, 0x001c_cff0)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001c_cffc)?;
            let no_text = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !no_text {
                c.cycles += 2;
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x003a_0e90, 0x001c_d00c)?;
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x20));
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEPopupConfirm_MeasureLine` (0x001c84a8).
//
// `void cFEPopupConfirm::MeasureLine(self, TextMetrics* m, float* out, void* dst)`:
// measure one line of the popup text and scale it by the font metrics.

pub fn fe_popup_confirm_measure_line(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 48;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        let metrics = c.g[A1][0] as u32;
        frame.save(c, 18, 0x20);
        let out = c.g[A2][0];
        frame.save_ra(c, 0x10);
        let dst = c.g[A3][0];
        c.g[S0][0] = sx(metrics);
        c.g[S1][0] = out;
        c.g[S2][0] = dst;
        c.ldl(2, metrics.wrapping_add(0x57));
        c.ldr(2, metrics.wrapping_add(0x50));
        c.g[V1][0] = lw_8(c, metrics, 0x58);
        c.sdl(2, sp(c).wrapping_add(7));
        c.sdr(2, sp(c));
        sw_3(c, sp(c), 8, c.g[V1][0]);
        c.g[A0][0] = sx(metrics);
        call_ptr(c, 0x003a_04f0, 0x001c_84e8)?;
        ldf(c, 2, sp(c), 0);
        c.g[A0][0] = sx(metrics);
        ldf(c, 3, sp(c), 4);
        let v0 = c.g[V0][0] as u32;
        ldf(c, 0, v0, 0x30);
        ldf(c, 1, v0, 0x34);
        c.mul_s(0, 0, 2);
        c.mul_s(1, 1, 3);
        c.swc1(0, v0.wrapping_add(0x38));
        c.swc1(1, v0.wrapping_add(0x3c));
        call_ptr(c, 0x003a_04f0, 0x001c_8510)?;
        let v0 = c.g[V0][0] as u32;
        c.g[A0][0] = sx(v0);
        c.g[A1][0] = c.g[S1][0];
        ldf(c, 12, v0, 0x38);
        c.g[A2][0] = c.g[S2][0];
        ldf(c, 13, v0, 0x3c);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_1fb0, 0x001c_852c)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003a_04f0, 0x001c_8534)?;
        ldf(c, 0, c.g[V0][0] as u32, 0x14);
        c.cvt_s_w(0, 0);
        ldf(c, 1, c.g[V0][0] as u32, 0x34);
        ldf(c, 2, sp(c), 4);
        frame.restore(c, 16, 0x40);
        c.mul_s(0, 0, 1);
        frame.restore(c, 17, 0x30);
        c.mul_s(2, 2, 0);
        c.swc1(2, c.g[S2][0] as u32 + 0xc);
        frame.restore(c, 18, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEPopupConfirm_MeasureLineW` (0x001c8568): the wide-string variant of
// [`fe_popup_confirm_measure_line`].
pub fn fe_popup_confirm_measure_line_w(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 48;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        let metrics = c.g[A1][0] as u32;
        frame.save(c, 18, 0x20);
        let out = c.g[A2][0];
        frame.save_ra(c, 0x10);
        let dst = c.g[A3][0];
        c.g[S0][0] = sx(metrics);
        c.g[S1][0] = out;
        c.g[S2][0] = dst;
        c.ldl(2, metrics.wrapping_add(0x57));
        c.ldr(2, metrics.wrapping_add(0x50));
        c.g[V1][0] = lw_8(c, metrics, 0x58);
        c.sdl(2, sp(c).wrapping_add(7));
        c.sdr(2, sp(c));
        sw_3(c, sp(c), 8, c.g[V1][0]);
        c.g[A0][0] = sx(metrics);
        call_ptr(c, 0x003a_04f0, 0x001c_85a8)?;
        ldf(c, 2, sp(c), 0);
        c.g[A0][0] = sx(metrics);
        ldf(c, 3, sp(c), 4);
        let v0 = c.g[V0][0] as u32;
        ldf(c, 0, v0, 0x30);
        ldf(c, 1, v0, 0x34);
        c.mul_s(0, 0, 2);
        c.mul_s(1, 1, 3);
        c.swc1(0, v0.wrapping_add(0x38));
        c.swc1(1, v0.wrapping_add(0x3c));
        call_ptr(c, 0x003a_04f0, 0x001c_85d0)?;
        let v0 = c.g[V0][0] as u32;
        c.g[A0][0] = sx(v0);
        c.g[A1][0] = c.g[S1][0];
        ldf(c, 12, v0, 0x38);
        c.g[A2][0] = c.g[S2][0];
        ldf(c, 13, v0, 0x3c);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_21f0, 0x001c_85ec)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003a_04f0, 0x001c_85f4)?;
        ldf(c, 0, c.g[V0][0] as u32, 0x14);
        c.cvt_s_w(0, 0);
        ldf(c, 1, c.g[V0][0] as u32, 0x34);
        ldf(c, 2, sp(c), 4);
        frame.restore(c, 16, 0x40);
        c.mul_s(0, 0, 1);
        frame.restore(c, 17, 0x30);
        c.mul_s(2, 2, 0);
        c.swc1(2, c.g[S2][0] as u32 + 0xc);
        frame.restore(c, 18, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cKeyboardPopup_MoveDownFromBottomRow(this, a1)` (0x1cbce8): from the
/// bottom row, move the cursor to the first enabled key of the next row
/// (wrapping from the last row back to the top), or find a key in the
/// current column.
pub fn c_keyboard_popup_move_down_from_bottom_row(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let this = abi::arg(c, 0);
        let _col = abi::arg(c, 1);
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0x10);
        c.g[V1][0] = Addr(this).off(0x140).lw(c);
        let state = c.g[V1][0];
        c.g[S1][0] = abi::arg64(c, 1);

        // 0: fell through to the epilogue with the original's v0,
        // 1: the column search runs, 2: a new state was stored (return 1).
        let mut path = 0u32;
        if state == 0x4c {
            c.cycles += 5;
            c.g[A1][0] = 0;
            c.g[A2][0] = frame.at(c, 0) as u64;
            callee::KEYBOARD_POPUP_IS_KEY_ENABLED.call(c, 0x001c_bd18)?;
            let disabled = c.g[V0][0] == 0;
            c.g[V0][0] = 9;
            if !disabled {
                c.cycles += 2;
                Addr(this).off(0x140).sw(c, c.g[V0][0]);
                path = 2;
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x17;
                c.g[A2][0] = frame.at(c, 0) as u64;
                callee::KEYBOARD_POPUP_IS_KEY_ENABLED.call(c, 0x001c_bd38)?;
                let disabled = c.g[V0][0] == 0;
                c.g[V0][0] = 0x17;
                if !disabled {
                    c.cycles += 2;
                    Addr(this).off(0x140).sw(c, c.g[V0][0]);
                    path = 2;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(c.g[SP][0] as u32).lw(c);
                    path = 1;
                }
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x4b;
            let other_row = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 0;
            if !other_row {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x13;
                c.g[A2][0] = sx(sp(c) | 4);
                callee::KEYBOARD_POPUP_IS_KEY_ENABLED.call(c, 0x001c_bd6c)?;
                let disabled = c.g[V0][0] == 0;
                c.g[V0][0] = 0x13;
                if !disabled {
                    c.cycles += 2;
                    Addr(this).off(0x140).sw(c, c.g[V0][0]);
                    path = 2;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(c.g[SP][0] as u32).off(4).lw(c);
                    path = 1;
                }
            }
        }

        if path == 1 {
            c.cycles += 2;
            let empty = c.g[V0][0] == 0;
            c.g[V0][0] = (c.g[S1][0] < 1) as u64;
            if !empty {
                c.cycles += 1;
                Addr(this).off(0x144).sw(c, c.g[V0][0]);
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            callee::KEYBOARD_POPUP_FIND_ENABLED_IN_COLUMN_FROM_TOP.call(c, 0x001c_bd94)?;
        }
        if path != 0 {
            c.cycles += 1;
            c.g[V0][0] = 1;
        }

        c.cycles += 5;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cKeyboardPopup_InsertChar(this, ch)` (0x1cc620): insert `ch` at the
/// cursor in the typed buffer at +0x74, keeping the cursor inside the field
/// and re-terminating the string.
pub fn c_keyboard_popup_insert_char(c: &mut Cpu) {
    'b_1cc6cc: {
        'b_1cc6b0: {
            c.cycles += 15;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = sx((c.g[A1][0] as u32) << 24);
            c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
            c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x001cc64c);
            c.g[A0][0] = c.g[S2][0];
            if call_ptr(c, 0x00416810, 0x001cc64c).is_err() {
                return;
            }
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x134));
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            let t = c.g[A0][0] != c.g[V0][0];
            c.sb(c.g[V1][0] as u32, c.g[S0][0]);
            if t {
                break 'b_1cc6b0;
            } else {
                'b_1cc688: {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x134));
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    if c.g[A1][0] != c.g[A2][0] {
                        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                        break 'b_1cc688;
                    } else {
                        c.cycles += 6;
                        c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sb(c.g[V1][0] as u32, 0u64);
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x134));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x13c), c.g[V0][0]);
                        if t {
                            break 'b_1cc6cc;
                        } else {
                            break 'b_1cc688;
                        }
                    }
                }
                'b_1cc694: {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    if t {
                        break 'b_1cc694;
                    } else {
                        c.cycles += 1;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x134), c.g[V0][0]);
                        break 'b_1cc694;
                    }
                }
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x134));
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sb(c.g[V0][0] as u32, 0u64);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x13c));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                let t = true;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x13c), c.g[V1][0]);
                if t {
                    break 'b_1cc6cc;
                } else {
                    break 'b_1cc6b0;
                }
            }
        }
        c.cycles += 6;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x134));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        if t {
            break 'b_1cc6cc;
        } else {
            c.cycles += 1;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x134), c.g[V0][0]);
            break 'b_1cc6cc;
        }
    }
    c.cycles += 6;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = t;
    return;
}

/// `cFEPopupConfirm_GetWidgetText(this, dst, text)` (0x1c8258): copy the
/// widget's text (ASCII or wide) into the string, or the default prompt.
pub fn cfe_popup_confirm_get_widget_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let this = abi::arg(c, 0);
        let _dst = abi::arg(c, 1);
        let _text = abi::arg(c, 2);
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        c.g[S0][0] = abi::arg64(c, 2);
        frame.save(c, 18, 0x10);
        c.g[S2][0] = abi::arg64(c, 1);
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::UI_TEXT_GET_OWNED_STRING.call(c, 0x001c_827c)?;

        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[V0][0];
            callee::C_BX_STRING_ASSIGN_CSTR.call(c, 0x001c_828c)?;
            c.lq(16, frame.at(c, 0x30));
            c.cycles += 5;
            frame.restore(c, 17, 0x20);
            frame.restore(c, 18, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        // No ASCII text: try the wide string.
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        callee::UI_TEXT_GET_ID_STRING.call(c, 0x001c_829c)?;
        c.g[S1][0] = c.g[V0][0];
        if c.g[S1][0] == 0 {
            c.g[A1][0] = 0x004a_0000;
            // -> the shared default-string tail
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = sx(0x004a_0000u32.wrapping_add(0x1408));
            callee::C_BX_STRING_ASSIGN_CSTR.call(c, 0x001c_8308)?;
            c.lq(16, frame.at(c, 0x30));
            c.cycles += 5;
            frame.restore(c, 17, 0x20);
            frame.restore(c, 18, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 17;
        c.g[A0][0] = c.g[S1][0];
        callee::U_STR_LEN.call(c, 0x001c_82b0)?;
        c.g[A1][0] = 0x0046_0000;
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        c.g[A1][0] = sx(0x0046_0000u32.wrapping_add(0x60b8));
        callee::OPERATOR_NEW_NAMED_3.call(c, 0x001c_82c8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        callee::U_STR_TO_ASCII.call(c, 0x001c_82d8)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        callee::C_BX_STRING_ASSIGN_CSTR.call(c, 0x001c_82e4)?;
        if c.g[S0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, addr::EA_MEM_FREE, 0x001c_82f4)?;
        }
        c.lq(16, frame.at(c, 0x30));
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        let _ = this;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Keyboard popup (a resumable menu cursor mover).

/// `void cKeyboardPopup_MoveDownFromRow4(cKeyboardPopup *self, int step)`
/// (0x1cbfc0): move the cursor down, wrapping within the enabled keys of the
/// column. The 0x1cbffc back edge is a re-entry point.
pub fn ckeyboard_popup_move_down_from_row4(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        // Opened at the entry arm, reused by the 0x1cbffc resume arm.
        let mut frame = Frame { size: 0x40 };
        loop {
            match pc {
                0x001c_bfc0 => {
                    c.cycles += 9;
                    frame = Frame::open(c, 0x40);
                    frame.save(c, S0, 0x30);
                    frame.save(c, S1, 0x20);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_ra(c, 0x10);
                    c.g[V1][0] = Addr(p_5(c.g[S0][0], 0x140)).lw(c);
                    let not_row = c.g[V1][0] != 0x39;
                    c.g[S1][0] = c.g[A1][0];
                    if not_row {
                        pc = 0x001c_c004;
                    } else {
                        pc = 0x001c_bfe4;
                    }
                }
                0x001c_bfe4 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0x4b;
                    c.g[A2][0] = c.g[SP][0];
                    callee::KEYBOARD_IS_KEY_ENABLED.call(c, 0x001c_bff0)?;
                    let missing = c.g[V0][0] == 0;
                    c.g[V0][0] = Addr(c.g[SP][0] as u32).lw(c);
                    if missing {
                        pc = 0x001c_c060;
                    } else {
                        pc = 0x001c_bff8;
                    }
                }
                0x001c_bff8 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x4b;
                    pc = 0x001c_bffc;
                }
                0x001c_bffc => {
                    c.cycles += 2;
                    Addr(p_5(c.g[S0][0], 0x140)).sw(c, c.g[V0][0]);
                    pc = 0x001c_c074;
                }
                0x001c_c004 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3a));
                    c.g[V0][0] = (c.g[V0][0] < 0x8) as u64;
                    let not_grid = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    if not_grid {
                        pc = 0x001c_c030;
                    } else {
                        pc = 0x001c_c014;
                    }
                }
                0x001c_c014 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0x4b;
                    c.g[A2][0] = c.g[SP][0];
                    callee::KEYBOARD_IS_KEY_ENABLED.call(c, 0x001c_c020)?;
                    let enabled = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x4b;
                    if enabled {
                        if c.cycles >= c.next_event {
                            c.pc = 0x001c_bffc;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x001c_bffc;
                    } else {
                        pc = 0x001c_c028;
                    }
                }
                0x001c_c028 => {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(c.g[SP][0] as u32).lw(c);
                    pc = 0x001c_c060;
                }
                0x001c_c030 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x42;
                    let not_last = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0;
                    if not_last {
                        pc = 0x001c_c078;
                    } else {
                        pc = 0x001c_c03c;
                    }
                }
                0x001c_c03c => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x4c;
                    c.g[A2][0] = c.g[SP][0];
                    callee::KEYBOARD_IS_KEY_ENABLED.call(c, 0x001c_c04c)?;
                    let missing = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x4c;
                    if missing {
                        pc = 0x001c_c05c;
                    } else {
                        pc = 0x001c_c054;
                    }
                }
                0x001c_c054 => {
                    c.cycles += 2;
                    Addr(p_5(c.g[S0][0], 0x140)).sw(c, c.g[V0][0]);
                    pc = 0x001c_c074;
                }
                0x001c_c05c => {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(c.g[SP][0] as u32).lw(c);
                    pc = 0x001c_c060;
                }
                0x001c_c060 => {
                    c.cycles += 2;
                    let empty = c.g[V0][0] == 0;
                    c.g[V0][0] = (c.g[S1][0] < 1) as u64;
                    if empty {
                        pc = 0x001c_c06c;
                    } else {
                        pc = 0x001c_c068;
                    }
                }
                0x001c_c068 => {
                    c.cycles += 1;
                    Addr(p_5(c.g[S0][0], 0x144)).sw(c, c.g[V0][0]);
                    pc = 0x001c_c06c;
                }
                0x001c_c06c => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    callee::KEYBOARD_FIND_ENABLED_FROM_TOP.call(c, 0x001c_c074)?;
                    pc = 0x001c_c074;
                }
                0x001c_c074 => {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                    pc = 0x001c_c078;
                }
                0x001c_c078 => {
                    c.cycles += 5;
                    frame.restore(c, S0, 0x30);
                    frame.restore(c, S1, 0x20);
                    frame.restore_ra(c, 0x10);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x001c_bfc0, 0x001c_c08c);
                    return Ok(());
                }
            }
        }
    });
}

// ---------------------------------------------------------------------------
// Popup confirm.

/// `void cFEPopupConfirm_LayoutTitle(self, Desc* desc)` (0x1c8678): lay out
/// the confirm dialog's title / body from the descriptor and relayout it.
pub fn cfepopupconfirm_layout_title(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A3][0] = Addr(p(r32(c, S0), 0x7c)).lw(c);
        if c.g[A3][0] == 0 {
            c.lq(S0, frame.at(c, 0x30));
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, A1), 0xc)).lw(c);
            let no_desc = c.g[V0][0] == 0;
            c.g[S1][0] = sx(p(r32(c, A1), 0xc));
            if !no_desc {
                c.cycles += 26;
                c.g[A1][0] = c.g[A3][0];
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, 0x001c_8050, 0x001c_86b0)?;
                c.g[A2][0] = Addr(p(r32(c, S0), 0x7c)).lw(c);
                c.g[A1][0] = 1;
                c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
                c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
                let f = r32(c, V1);
                c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                call_ptr(c, f, 0x001c_86cc)?;
                c.g[A1][0] = Addr(p(r32(c, S0), 0x7c)).lw(c);
                c.g[A3][0] = c.g[SP][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x001c_8320, 0x001c_86e0)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx(p(r32(c, S0), 0x260));
                c.g[A2][0] = c.g[SP][0];
                call_ptr(c, 0x001c_9b68, 0x001c_86f0)?;
                c.lwc1(0, p(r32(c, S0), 0x264));
                c.g[AT][0] = 0x40a00000;
                c.mtc1(1, 1);
                c.g[V0][0] = Addr(p(r32(c, S0), 0x2cc)).lw(c);
                c.add_s(0, 0, 1);
                let no_align = c.g[V0][0] == 0;
                c.swc1(0, p(r32(c, S0), 0x264));
                if !no_align {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    c.mtc1(0, 12);
                    c.g[A1][0] = Addr(p(r32(c, A0), 0x7c)).lw(c);
                    c.g[A2][0] = 0x11;
                    call_ptr(c, 0x001c_8930, 0x001c_8720)?;
                    c.lq(S0, frame.at(c, 0x30));
                }
            } else {
                c.cycles += 6;
                c.g[V0][0] = Addr(p(r32(c, A3), 8)).lw(c);
                c.g[A1][0] = 0;
                c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
                c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
                let f = r32(c, V1);
                c.g[A0][0] = sx(p(r32(c, A3), r32(c, A0)));
                call_ptr(c, f, 0x001c_8740)?;
            }
            c.cycles += 1;
            c.lq(S0, frame.at(c, 0x30));
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Keyboard popup.

/// `int cKeyboardPopup_HandleKeyChar(self, char ch)` (0x1cc6e8): map the
/// character to a key index and select it when enabled.
pub fn ckeyboardpopup_handle_key_char(c: &mut Cpu) {
    let mut pc = c.pc;
    loop {
        match pc {
            0x001c_c6e8 => {
                c.cycles += 9;
                c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x30));
                c.g[A1][0] = sx(r32(c, A1) << 24);
                c.sq(S1, p(r32(c, SP), 0x10));
                c.g[A1][0] = sx(((r32(c, A1) as i32) >> 24) as u32);
                c.sq(S0, p(r32(c, SP), 0x20));
                c.g[S1][0] = c.g[A0][0];
                c.g[V0][0] = 0x60;
                let is_shift = c.g[A1][0] != c.g[V0][0];
                c.sd(r32(c, SP), c.g[RA][0]);
                if is_shift {
                    pc = 0x001c_c728;
                } else {
                    pc = 0x001c_c70c;
                }
            }
            0x001c_c70c => {
                c.cycles += 4;
                c.g[V1][0] = 0xe;
                c.g[V0][0] = 1;
                Addr(p(r32(c, S1), 0x140)).sw(c, c.g[V1][0]);
                pc = 0x001c_c7b0;
            }
            0x001c_c71c => {
                c.cycles += 3;
                Addr(p(r32(c, S1), 0x140)).sw(c, c.g[S0][0]);
                c.g[V0][0] = 1;
                pc = 0x001c_c7b0;
            }
            0x001c_c728 => {
                c.cycles += 8;
                c.g[V0][0] = 0x440000;
                c.g[V1][0] = 0x440000;
                c.g[T0][0] = sx(r32(c, V0).wrapping_add(0x1390));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x13e8));
                c.g[S0][0] = 0;
                c.g[A3][0] = 0x2a;
                c.g[A2][0] = 0x40;
                pc = 0x001c_c748;
            }
            0x001c_c748 => {
                c.cycles += 4;
                c.g[V0][0] = sx(p(r32(c, S0), r32(c, T0)));
                c.g[A0][0] = Addr(r32(c, V0)).lb(c);
                if c.g[A0][0] == c.g[A1][0] {
                    pc = 0x001c_c764;
                } else {
                    pc = 0x001c_c758;
                }
            }
            0x001c_c758 => {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, V1)).lb(c);
                if c.g[V0][0] != c.g[A1][0] {
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    pc = 0x001c_c7a0;
                } else {
                    pc = 0x001c_c764;
                }
            }
            0x001c_c764 => {
                c.cycles += 2;
                if c.g[A0][0] == c.g[A3][0] {
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    pc = 0x001c_c7a0;
                } else {
                    pc = 0x001c_c76c;
                }
            }
            0x001c_c76c => {
                c.cycles += 2;
                if c.g[A0][0] != c.g[A2][0] {
                    c.g[A0][0] = c.g[S1][0];
                    pc = 0x001c_c780;
                } else {
                    pc = 0x001c_c774;
                }
            }
            0x001c_c774 => {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, V1)).lb(c);
                let same = c.g[V0][0] == c.g[A0][0];
                c.g[A0][0] = c.g[S1][0];
                if same {
                    pc = 0x001c_c79c;
                } else {
                    pc = 0x001c_c780;
                }
            }
            0x001c_c780 => {
                c.cycles += 5;
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 0;
                if call_ptr(c, 0x001c_e560, 0x001c_c78c).is_err() {
                    return;
                }
                let enabled = c.g[V0][0] != 0;
                c.g[V0][0] = 0;
                if enabled {
                    if back_edge_unwinds(c, 0x001c_c71c) {
                        return;
                    }
                    pc = 0x001c_c71c;
                } else {
                    pc = 0x001c_c794;
                }
            }
            0x001c_c794 => {
                c.cycles += 2;
                c.lq(S0, p(r32(c, SP), 0x20));
                pc = 0x001c_c7b4;
            }
            0x001c_c79c => {
                c.cycles += 1;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                pc = 0x001c_c7a0;
            }
            0x001c_c7a0 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < 84i64) as u64;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                if c.g[V0][0] != 0 {
                    if back_edge_unwinds(c, 0x001c_c748) {
                        return;
                    }
                    pc = 0x001c_c748;
                } else {
                    pc = 0x001c_c7ac;
                }
            }
            0x001c_c7ac => {
                c.cycles += 1;
                c.g[V0][0] = 0;
                pc = 0x001c_c7b0;
            }
            0x001c_c7b0 => {
                c.cycles += 1;
                c.lq(S0, p(r32(c, SP), 0x20));
                pc = 0x001c_c7b4;
            }
            0x001c_c7b4 => {
                c.cycles += 4;
                c.lq(S1, p(r32(c, SP), 0x10));
                c.g[RA][0] = c.ld(r32(c, SP));
                let ra = r32(c, RA);
                c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x30));
                c.pc = ra;
                return;
            }
            _ => {
                c.jump_out(pc, 0x001c_c6e8, 0x001c_c7c4);
                return;
            }
        }
    }
}

/// `void cFEPopupConfirm_ApplyBackgroundScale(self, const sColor* c)`
/// (0x1c7738): copy a colour to the popup's background objects, chosen by
/// the popup style at `+0x31c`: two objects for style 0, five for style 1,
/// one for style 2. Each object gets the colour at `+0x50`.
pub fn c_fe_popup_confirm_apply_background_scale(c: &mut Cpu) {
    'b_1c78b4: {
        'b_1c7894: {
            'b_1c77d8: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.g[V1][0] = c.ldw_2(A0, 0x31c);
                c.ldl(6, c.pa(A1, 0x7));
                c.ldr(6, c.pa(A1, 0));
                c.g[A3][0] = c.ldw_2(A1, 0x8);
                c.sdl(6, c.pa(SP, 0x7));
                c.sdr(6, c.pa(SP, 0));
                c.g[V0][0] = 0x1;
                let t = c.g[V1][0] == c.g[V0][0];
                c.stw_2(SP, 0x8, c.g[A3][0]);
                if t {
                    break 'b_1c77d8;
                } else {
                    'b_1c778c: {
                        'b_1c777c: {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = 0x2;
                                break 'b_1c777c;
                            } else {
                                c.cycles += 2;
                                if c.g[V1][0] == 0u64 {
                                    c.g[V0][0] = c.ldw_2(A0, 0x48);
                                    break 'b_1c778c;
                                } else {
                                    c.cycles += 2;
                                    let t = true;

                                    if t {
                                        break 'b_1c78b4;
                                    } else {
                                        break 'b_1c777c;
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        if c.g[V1][0] == c.g[V0][0] {
                            c.g[A0][0] = c.ldw_2(A0, 0x68);
                            break 'b_1c7894;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_1c78b4;
                            } else {
                                break 'b_1c778c;
                            }
                        }
                    }
                    'b_1c77b0: {
                        c.cycles += 2;
                        if c.g[V0][0] == 0u64 {
                            c.g[A0][0] = c.ldw_2(A0, 0x4c);
                            break 'b_1c77b0;
                        } else {
                            c.cycles += 7;
                            c.ldl(3, c.pa(SP, 0x7));
                            c.ldr(3, c.pa(SP, 0));
                            c.g[A1][0] = c.ldw_2(SP, 0x8);
                            c.sdl(3, c.pa(V0, 0x57));
                            c.sdr(3, c.pa(V0, 0x50));
                            c.stw_2(V0, 0x58, c.g[A1][0]);
                            c.g[A0][0] = c.ldw_2(A0, 0x4c);
                            break 'b_1c77b0;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[A0][0] == 0u64;

                    if t {
                        break 'b_1c78b4;
                    } else {
                        c.cycles += 8;
                        c.ldl(8, c.pa(SP, 0x7));
                        c.ldr(8, c.pa(SP, 0));
                        c.g[T1][0] = c.ldw_2(SP, 0x8);
                        c.sdl(8, c.pa(A0, 0x57));
                        c.sdr(8, c.pa(A0, 0x50));
                        c.stw_2(A0, 0x58, c.g[T1][0]);
                        let t = true;

                        if t {
                            break 'b_1c78b4;
                        } else {
                            break 'b_1c77d8;
                        }
                    }
                }
            }
            'b_1c7800: {
                c.cycles += 3;
                c.g[V0][0] = c.ldw_2(A0, 0x58);
                if c.g[V0][0] == 0u64 {
                    c.g[V0][0] = c.ldw_2(A0, 0x5c);
                    break 'b_1c7800;
                } else {
                    c.cycles += 7;
                    c.ldl(5, c.pa(SP, 0x7));
                    c.ldr(5, c.pa(SP, 0));
                    c.g[A2][0] = c.ldw_2(SP, 0x8);
                    c.sdl(5, c.pa(V0, 0x57));
                    c.sdr(5, c.pa(V0, 0x50));
                    c.stw_2(V0, 0x58, c.g[A2][0]);
                    c.g[V0][0] = c.ldw_2(A0, 0x5c);
                    break 'b_1c7800;
                }
            }
            'b_1c7824: {
                c.cycles += 2;
                if c.g[V0][0] == 0u64 {
                    c.g[V0][0] = c.ldw_2(A0, 0x50);
                    break 'b_1c7824;
                } else {
                    c.cycles += 7;
                    c.ldl(9, c.pa(SP, 0x7));
                    c.ldr(9, c.pa(SP, 0));
                    c.g[V1][0] = c.ldw_2(SP, 0x8);
                    c.sdl(9, c.pa(V0, 0x57));
                    c.sdr(9, c.pa(V0, 0x50));
                    c.stw_2(V0, 0x58, c.g[V1][0]);
                    c.g[V0][0] = c.ldw_2(A0, 0x50);
                    break 'b_1c7824;
                }
            }
            'b_1c7848: {
                c.cycles += 2;
                if c.g[V0][0] == 0u64 {
                    c.g[V0][0] = c.ldw_2(A0, 0x54);
                    break 'b_1c7848;
                } else {
                    c.cycles += 7;
                    c.ldl(7, c.pa(SP, 0x7));
                    c.ldr(7, c.pa(SP, 0));
                    c.g[T0][0] = c.ldw_2(SP, 0x8);
                    c.sdl(7, c.pa(V0, 0x57));
                    c.sdr(7, c.pa(V0, 0x50));
                    c.stw_2(V0, 0x58, c.g[T0][0]);
                    c.g[V0][0] = c.ldw_2(A0, 0x54);
                    break 'b_1c7848;
                }
            }
            'b_1c786c: {
                c.cycles += 2;
                if c.g[V0][0] == 0u64 {
                    c.g[A0][0] = c.ldw_2(A0, 0x60);
                    break 'b_1c786c;
                } else {
                    c.cycles += 7;
                    c.ldl(5, c.pa(SP, 0x7));
                    c.ldr(5, c.pa(SP, 0));
                    c.g[A2][0] = c.ldw_2(SP, 0x8);
                    c.sdl(5, c.pa(V0, 0x57));
                    c.sdr(5, c.pa(V0, 0x50));
                    c.stw_2(V0, 0x58, c.g[A2][0]);
                    c.g[A0][0] = c.ldw_2(A0, 0x60);
                    break 'b_1c786c;
                }
            }
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;

            if t {
                break 'b_1c78b4;
            } else {
                c.cycles += 8;
                c.ldl(9, c.pa(SP, 0x7));
                c.ldr(9, c.pa(SP, 0));
                c.g[V0][0] = c.ldw_2(SP, 0x8);
                c.sdl(9, c.pa(A0, 0x57));
                c.sdr(9, c.pa(A0, 0x50));
                c.stw_2(A0, 0x58, c.g[V0][0]);
                let t = true;

                if t {
                    break 'b_1c78b4;
                } else {
                    break 'b_1c7894;
                }
            }
        }
        c.cycles += 2;
        let t = c.g[A0][0] == 0u64;

        if t {
            break 'b_1c78b4;
        } else {
            c.cycles += 6;
            c.ldl(6, c.pa(SP, 0x7));
            c.ldr(6, c.pa(SP, 0));
            c.g[A3][0] = c.ldw_2(SP, 0x8);
            c.sdl(6, c.pa(A0, 0x57));
            c.sdr(6, c.pa(A0, 0x50));
            c.stw_2(A0, 0x58, c.g[A3][0]);
            break 'b_1c78b4;
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
    return;
}

// ---------------------------------------------------------------------------
// 0x001c70f0 cFEPopupConfirm_ResizeTextWidgets
// ---------------------------------------------------------------------------

/// One child-widget resize pass of `cFEPopupConfirm_ResizeTextWidgets`: asks
/// the widget for its two sizes, stores the non-zero ones into the stack size
/// vector and calls the widget's resize method.
pub(crate) fn resize_child(
    c: &mut Cpu,
    widget_slot: u32,
    first_charge: u64,
    resize_arg: u64,
    ret1: u32,
    ret2: u32,
) -> crate::call::Flow {
    let obj = c.g[S1][0] as u32;
    c.g[A2][0] = c.lw(p(obj, widget_slot));
    c.g[A1][0] = c.g[SP][0];
    if c.g[A2][0] == 0 {
        return Ok(());
    }
    c.cycles += first_charge;
    c.g[V0][0] = c.lw(p(c.g[A2][0] as u32, 8));
    c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0xa0));
    c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0xa4));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx(c.g[A2][0] as u32 + c.g[A0][0] as u32);
    call_ptr(c, t, ret1)?;
    let size = c.g[S0][0] as u32;
    c.lwc1(0, size);
    c.mtc1(0, 1);
    c.c_eq_s(0, 1);
    if !c.fcc() {
        c.swc1(0, c.g[SP][0] as u32);
    }
    c.cycles += 5;
    c.lwc1(0, p(size, 4));
    c.c_eq_s(0, 1);
    if !c.fcc() {
        c.swc1(0, p(c.g[SP][0] as u32, 4));
    }
    c.cycles += 7;
    c.g[A2][0] = c.lw(p(obj, widget_slot));
    c.g[A1][0] = resize_arg;
    c.g[V0][0] = c.lw(p(c.g[A2][0] as u32, 8));
    c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0xa8));
    c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0xac));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx(c.g[A2][0] as u32 + c.g[A0][0] as u32);
    call_ptr(c, t, ret2)
}

/// `void cFEPopupConfirm_ResizeTextWidgets(cFEPopupConfirm* self,
/// const Vec2* size)` (0x1c70f0).
///
/// Re-lays out the popup's three text widgets to the given size, then asks the
/// popup to resize itself. The size vector is rebuilt on the stack, skipping
/// zero components.
pub fn cfe_popup_confirm_resize_text_widgets(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        let obj = c.g[A0][0] as u32;
        frame.save_ra(c, 0x10);
        c.g[S1][0] = sx(obj);
        c.g[A2][0] = c.lw(p(obj, 0x74));
        let size = c.g[A1][0] as u32;
        c.g[S0][0] = sx(size);
        if c.g[A2][0] != 0 {
            c.cycles += 12;
            c.g[V0][0] = c.lw(p(c.g[A2][0] as u32, 8));
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0xa0));
            c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0xa4));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx(c.g[A2][0] as u32 + c.g[A0][0] as u32);
            call_ptr(c, t, 0x001c_7128)?;
            c.lwc1(0, size);
            c.mtc1(0, 1);
            c.c_eq_s(0, 1);
            if !c.fcc() {
                c.swc1(0, c.g[SP][0] as u32);
            }
            c.cycles += 5;
            c.lwc1(0, p(size, 4));
            c.c_eq_s(0, 1);
            if !c.fcc() {
                c.swc1(0, p(c.g[SP][0] as u32, 4));
            }
            c.cycles += 7;
            c.g[A2][0] = c.lw(p(obj, 0x74));
            c.g[A1][0] = sx(size);
            c.g[V0][0] = c.lw(p(c.g[A2][0] as u32, 8));
            c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0xa8));
            c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0xac));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx(c.g[A2][0] as u32 + c.g[A0][0] as u32);
            call_ptr(c, t, 0x001c_7170)?;
        }

        c.cycles += 3;
        let size_vec = c.g[SP][0];
        resize_child(c, 0x78, 11, size_vec, 0x001c_7190, 0x001c_71d8)?;
        c.cycles += 3;
        resize_child(c, 0x7c, 11, c.g[S0][0], 0x001c_71f8, 0x001c_7240)?;

        c.cycles += 5;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end popup background placement.

/// Copy a 12-byte position from `obj+vec` to the stack temporary, add `a`
/// (f20) to x, set y from f13 and store it at `child+0x44` (the trailing word
/// is at `child+0x4c`).  `load`/`store` select the scratch registers the
/// original happens to use.
#[allow(clippy::too_many_arguments)]
pub(crate) fn position_child(
    c: &mut Cpu,
    obj: u32,
    vec: u32,
    child: u32,
    load: usize,
    load_z: usize,
    store: usize,
    store_z: usize,
) {
    c.ldl(load, obj.wrapping_add(vec + 0xb));
    c.ldr(load, obj.wrapping_add(vec));
    c.g[load_z][0] = c.lw(obj.wrapping_add(vec + 8));
    c.sdl(load, sp(c).wrapping_add(0x17));
    c.sdr(load, sp(c).wrapping_add(0x10));
    c.sw(sp(c).wrapping_add(0x18), c.g[load_z][0]);
    c.swc1(13, sp(c).wrapping_add(0x14));
    c.lwc1(0, sp(c).wrapping_add(0x10));
    c.add_s(0, 0, 20);
    c.swc1(0, sp(c).wrapping_add(0x10));
    c.ldl(store, sp(c).wrapping_add(0x17));
    c.ldr(store, sp(c).wrapping_add(0x10));
    c.g[store_z][0] = c.lw(sp(c).wrapping_add(0x18));
    c.sdl(store, child.wrapping_add(0x4b));
    c.sdr(store, child.wrapping_add(0x44));
    c.sw(child.wrapping_add(0x4c), c.g[store_z][0]);
}

/// `cFEPopupConfirm_PositionBackground(self, const Vec3 *v, float a, float
/// b)` (0x1c78c0): position the popup's buttons from the stored button
/// vectors.  `self+0x31c` selects the layout: 0 = one column, 1 = five
/// buttons (with a rounded y), 2 = reset the slide values, anything else =
/// leave untouched.
pub fn fe_popup_confirm_position_background(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        c.mov_s(4, 13); // b
        frame.save(c, S0, 0x30);
        c.g[V0][0] = 1;
        c.swc1(20, frame.at(c, 0x40));
        let obj = c.g[A0][0] as u32;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.mov_s(20, 12); // a
        c.mtc1(0, 13); // y = 0
        let v = c.g[A1][0] as u32;
        c.g[V1][0] = Addr(obj).off(0x31c).lw(c);
        // Copy the incoming Vec3 to the stack temporary at sp+0.
        c.ldl(4, v.wrapping_add(0x7));
        c.ldr(4, v);
        c.g[A2][0] = c.lw(v.wrapping_add(0x8));
        c.sdl(4, frame.at(c, 0x7));
        c.sdr(4, frame.at(c, 0x0));
        let state = c.g[V1][0]; // sign-extended `lw`
        let is_one = c.g[V1][0] == c.g[V0][0];
        c.sw(frame.at(c, 0x8), c.g[A2][0]);
        if !is_one {
            c.cycles += 3;
            c.g[V0][0] = ((state as i64) < 2) as u64;
            if c.g[V0][0] == 0 {
                // state >= 2: only state 2 has work.
                c.cycles += 2;
                c.g[V0][0] = 2;
                if state == 2 {
                    c.cycles += 5;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                    c.swc1(0, Addr(obj).off(0x27c).0);
                    c.swc1(0, Addr(obj).off(0x278).0);
                } else {
                    c.cycles += 2;
                }
            } else if state == 0 {
                // Layout 0: two stacked buttons.
                c.cycles += 2;
                c.g[V0][0] = Addr(obj).off(0x48).lw(c);
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.cycles += 27;
                    c.ldl(3, obj.wrapping_add(0x18f));
                    c.ldr(3, obj.wrapping_add(0x188));
                    c.g[A0][0] = c.lw(obj.wrapping_add(0x190));
                    c.sdl(3, frame.at(c, 0x17));
                    c.sdr(3, frame.at(c, 0x10));
                    c.sw(frame.at(c, 0x18), c.g[A0][0]);
                    c.g[AT][0] = 0x3f00_0000;
                    c.mtc1(1, 3); // f3 = 0.5
                    c.lwc1(1, obj.wrapping_add(0x1bc));
                    c.lwc1(2, frame.at(c, 0x4)); // v.y
                    c.add_s(1, 1, 4); // base + b
                    c.lwc1(0, frame.at(c, 0x10)); // p.x
                    c.mul_s(2, 2, 3); // v.y * 0.5
                    c.g[AT][0] = 0x40a0_0000;
                    c.mtc1(1, 4); // f4 = 5.0
                    c.add_s(0, 0, 20); // p.x += a
                    c.add_s(1, 1, 2);
                    c.swc1(0, frame.at(c, 0x10));
                    c.sub_s(13, 1, 4); // y = base + b + v.y*0.5 - 5
                    c.swc1(13, frame.at(c, 0x14));
                    c.ldl(7, frame.at(c, 0x17));
                    c.ldr(7, frame.at(c, 0x10));
                    c.g[T0][0] = c.lw(frame.at(c, 0x18));
                    let child = c.g[V0][0] as u32;
                    c.sdl(7, child.wrapping_add(0x4b));
                    c.sdr(7, child.wrapping_add(0x44));
                    c.sw(child.wrapping_add(0x4c), c.g[T0][0]);
                    c.g[V0][0] = Addr(obj).off(0x4c).lw(c);
                } else {
                    c.g[V0][0] = Addr(obj).off(0x4c).lw(c);
                }
                // Second button at 0x4c.
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.cycles += 18;
                    let child = c.g[V0][0] as u32;
                    position_child(c, obj, 0x194, child, 5, A2, 3, A0);
                }
            } else {
                // Negative state: nothing to do (the two +2 charges of
                // 0x1c7930 and its fall-through).
                c.cycles += 4;
            }
        } else {
            // Layout 1: five buttons; the first one computes the shaped y.
            c.cycles += 3;
            c.g[V0][0] = Addr(obj).off(0x58).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 33;
                let child = c.g[V0][0] as u32;
                c.ldl(7, obj.wrapping_add(0x243));
                c.ldr(7, obj.wrapping_add(0x23c));
                c.g[T0][0] = c.lw(obj.wrapping_add(0x244));
                c.sdl(7, frame.at(c, 0x17));
                c.sdr(7, frame.at(c, 0x10));
                c.sw(frame.at(c, 0x18), c.g[T0][0]);
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(1, 1);
                c.lwc1(12, frame.at(c, 0x4));
                c.lwc1(0, obj.wrapping_add(0x1bc));
                c.mul_s(12, 12, 1);
                c.add_s(0, 0, 4);
                c.add_s(12, 0, 12);
                call_ptr(c, 0x0041_3af8, 0x001c_7a38)?; // __extendsfdf2
                c.g[A1][0] = 0x803c;
                c.g[A1][0] <<= 47; // 7.5
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0041_1f70, 0x001c_7a48)?; // __subdf3
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0041_27a8, 0x001c_7a50)?; // __truncdfsf2
                c.lwc1(1, frame.at(c, 0x10));
                c.mov_s(13, 0);
                c.g[V0][0] = Addr(obj).off(0x58).lw(c);
                c.add_s(1, 1, 20);
                c.swc1(13, frame.at(c, 0x14));
                c.swc1(1, frame.at(c, 0x10));
                c.ldl(4, frame.at(c, 0x17));
                c.ldr(4, frame.at(c, 0x10));
                c.g[A1][0] = c.lw(frame.at(c, 0x18));
                c.sdl(4, child.wrapping_add(0x4b));
                c.sdr(4, child.wrapping_add(0x44));
                c.sw(child.wrapping_add(0x4c), c.g[A1][0]);
                c.g[V0][0] = Addr(obj).off(0x5c).lw(c);
            } else {
                c.g[V0][0] = Addr(obj).off(0x5c).lw(c);
            }
            // Four more buttons, all with the y computed above (f13).
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 17;
                let child = c.g[V0][0] as u32;
                position_child(c, obj, 0x248, child, 8, V1, 6, A3);
            }
            c.g[V0][0] = Addr(obj).off(0x50).lw(c);
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 17;
                let child = c.g[V0][0] as u32;
                position_child(c, obj, 0x224, child, 4, A1, 8, V1);
            }
            c.g[V0][0] = Addr(obj).off(0x54).lw(c);
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 17;
                let child = c.g[V0][0] as u32;
                position_child(c, obj, 0x230, child, 6, A3, 4, A1);
            }
            c.g[V0][0] = Addr(obj).off(0x60).lw(c);
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 18;
                let child = c.g[V0][0] as u32;
                position_child(c, obj, 0x254, child, 8, V1, 6, A3);
            }
        }
        c.lq(16, frame.at(c, 0x30));
        c.cycles += 4;
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        c.lwc1(20, frame.at(c, 0x40));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
        abi::ret(c);
        Ok(())
    })
}

/// `cFEPopupConfirm_LayoutMenu` (0x001c8758): lay out the popup's menu.  The
/// text widget at `self+0x6c` is centred horizontally ((40 - `self+0x29c`) /
/// 2) and moved by `self+0x288`, the widget at `self+0x88` gets the mirrored
/// placement (+5), and -- when `self+0x2c8` is set -- the five widgets at
/// `self+0x90..` are aligned with `cFEPopupConfirm_SetWidgetAlign`; the final
/// placement is recorded at `self+0x290`/`+0x294`.
pub fn c_fe_popup_confirm_layout_menu(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 21, 0x78);
        frame.save_fpr(c, 20, 0x70);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
        if c.g[V1][0] == 0 {
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        } else {
            c.cycles += 57;
            c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x29c));
            c.g[AT][0] = 0x4420_0000;
            c.mtc1(1, 1); // f1 = 40.0
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x90));
            c.sub_s(1, 1, 0); // f1 = 40.0 - self->29c
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 5); // f5 = 0.5
            c.ldl(4, (c.g[V0][0] as u32).wrapping_add(0x4b));
            c.ldr(4, (c.g[V0][0] as u32).wrapping_add(0x44));
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
            c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.sdr(4, c.g[SP][0] as u32);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
            c.g[AT][0] = 0x40a0_0000;
            c.mtc1(1, 4); // f4 = 5.0
            c.mul_s(1, 1, 5); // f1 = (40 - f0) * 0.5
            c.lwc1(3, (c.g[S0][0] as u32).wrapping_add(0x288));
            c.lwc1(2, c.g[SP][0] as u32);
            c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1a7));
            c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1a0));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
            c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
            c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
            c.sub_s(20, 1, 2); // f20 = centring offset
            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.add_s(0, 0, 3);
            c.add_s(1, 1, 20);
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x17));
            c.ldr(7, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
            c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x4b));
            c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x44));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
            c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x4b));
            c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x44));
            c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
            c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x27));
            c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A3][0]);
            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sub_s(0, 0, 20);
            c.add_s(0, 0, 4); // f0 = mirrored placement + 5
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x27));
            c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
            c.sdl(5, (c.g[V1][0] as u32).wrapping_add(0x4b));
            c.sdr(5, (c.g[V1][0] as u32).wrapping_add(0x44));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4c), c.g[A2][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c8));
            let no_align = c.g[V0][0] == 0;
            c.mov_s(21, 5); // f21 = 0.5
            if !no_align {
                c.cycles += 2;
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x90));
                c.g[S2][0] = 0x4;
                loop {
                    c.cycles += 3;
                    c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                    let absent = c.g[A1][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    if !absent {
                        c.cycles += 4;
                        c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x29c));
                        c.g[A2][0] = 0x12;
                        c.g[RA][0] = sx(0x001c_888c);
                        c.mul_s(12, 12, 21);
                        call_ptr(c, 0x001c_8930, 0x001c_888c)?;
                    }
                    c.cycles += 3;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                    let more = (c.g[S2][0] as i64) >= 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x001c_8870);
                    }
                }
            }
            c.cycles += 4;
            c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x288));
            c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x290));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x294));
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 21, 0x78);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    });
}

/// `cFEPopupConfirm_LayoutBodyText` (0x001c7258): lay out the popup's body
/// text: copy the body rect at `a1` into position (+0x260..+0x26c), add the
/// line-spacing at `self+0x264` to its `y`, and store the result in the text
/// widget `self+0x74`; then, when the optional second widget at `self+0x2cc`
/// exists, re-wrap the text to the width from `cFEPopupConfirm_GetDescLineWrapSize`
/// (`cFEPopupConfirm_ResizeTextWidgets` when the height overflowed), write it
/// with `cFEPopupConfirm_SetWrappedText` and align it with
/// `cFEPopupConfirm_SetWidgetAlign`.
pub fn c_fe_popup_confirm_layout_body_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x20);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
        let no_widget = c.g[V0][0] == 0;
        if !no_widget {
            c.cycles += 19;
            c.ldl(3, (c.g[S0][0] as u32).wrapping_add(0x1bf));
            c.ldr(3, (c.g[S0][0] as u32).wrapping_add(0x1b8));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
            c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.sdr(3, c.g[SP][0] as u32);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x264));
            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
            c.add_s(1, 1, 0);
            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
            c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.ldr(8, c.g[SP][0] as u32);
            c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
            c.sdl(8, (c.g[V0][0] as u32).wrapping_add(0x4b));
            c.sdr(8, (c.g[V0][0] as u32).wrapping_add(0x44));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[T1][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2cc));
            let has_second = c.g[V0][0] != 0;
            c.g[S2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
            if has_second {
                c.cycles += 19;
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = 0x50_0000;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf28));
                c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(7, c.g[V1][0] as u32);
                c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x17));
                c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                c.g[RA][0] = sx(0x001c_72fc);
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, 0x001c_93b0, 0x001c_72fc)?;
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x260));
                c.c_lt_s(0, 1);
                let skip_resize = !c.fcc();
                c.g[A1][0] = c.g[S1][0];
                if !skip_resize {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x001c_731c);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x001c_70f0, 0x001c_731c)?;
                }
            }
            c.cycles += 10;
            c.g[AT][0] = 0x4120_0000;
            c.mtc1(1, 12); // f12 = 10.0
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x260));
            c.g[RA][0] = sx(0x001c_7338);
            c.g[A3][0] = 0x1;
            call_ptr(c, 0x001c_97e0, 0x001c_7338)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2cc));
            let done = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !done {
                c.cycles += 4;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                c.mtc1(0, 12);
                c.g[RA][0] = sx(0x001c_7354);
                c.g[A2][0] = 0x11;
                call_ptr(c, 0x001c_8930, 0x001c_7354)?;
            }
        }
        c.cycles += 12;
        c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x267));
        c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x260));
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x268));
        c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0x28b));
        c.sdr(4, (c.g[S0][0] as u32).wrapping_add(0x284));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28c), c.g[A1][0]);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// `lw_9` (sign-extended to 64 bits).
#[inline(always)]
pub(crate) fn lw_9(c: &mut Cpu, a: u32) -> u64 {
    c.lw(a)
}

/// `sw_4`.
#[inline(always)]
pub(crate) fn sw_4(c: &mut Cpu, a: u32, v: u64) {
    c.sw(a, v);
}

/// `lhu`.
#[inline(always)]
pub(crate) fn lhu(c: &mut Cpu, a: u32) -> u64 {
    c.lhu(a)
}

/// `sh`.
#[inline(always)]
pub(crate) fn sh(c: &mut Cpu, a: u32, v: u64) {
    c.sh(a, v);
}

/// `sb`.
#[inline(always)]
pub(crate) fn sb(c: &mut Cpu, a: u32, v: u64) {
    c.sb(a, v);
}

/// `ld`.
#[inline(always)]
pub(crate) fn ld(c: &mut Cpu, a: u32) -> u64 {
    c.ld(a)
}

/// `lq`.
#[inline(always)]
pub(crate) fn lq(c: &mut Cpu, rt: usize, a: u32) {
    c.lq(rt, a);
}

/// `sq`.
#[inline(always)]
pub(crate) fn sq(c: &mut Cpu, rt: usize, a: u32) {
    c.sq(rt, a);
}

/// `void func_001CB9B0(void* popup, int a1)` (0x1cb9b0): move the on-screen
/// keyboard's selection down one key. Reads the popup's "shifted" flag
/// (`+0x68`/`+0x6c`), remembers the old row (`+0x434`) and whether the key
/// was enabled (`+0x144`), and for a1 != 0 stores the row (when
/// `IsKeyEnabled(old, 0)` finds the key). Then, depending on the row index at
/// `+0x140`, calls the row/column stepping helper (rows 56..69, 70..83 or
/// below 14), advances `+0x140` by 14 (or wraps 70..83 back to 0..13),
/// skips disabled keys marked 0x2a in `D_00441390`, and finally re-checks the
/// key with `IsKeyEnabled(idx, &value)`; if it stays disabled it re-enters
/// itself and sets `+0x144` from the flag.
pub fn c_keyboard_popup_move_down(c: &mut Cpu) {
    let frame = Frame::open(c, 0x50);
    'b_1cb9e4: {
        'b_1cb9e0: {
            c.cycles += 10;
            frame.save(c, S0, 0x40);
            frame.save(c, S1, 0x30);
            c.g[S0][0] = c.g[A0][0];
            frame.save(c, S2, 0x20);
            frame.save_ra(c, 0x10);
            c.g[V0][0] = lw_9(c, at(c, S0, 0x68));
            c.g[V1][0] = lw_9(c, at(c, S0, 0x6c));
            let t = c.g[V1][0] == 0u64;
            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
            if t {
                break 'b_1cb9e0;
            } else {
                c.cycles += 2;
                let t = true;
                c.g[S1][0] = c.g[V0][0] ^ 0x1;
                if t {
                    break 'b_1cb9e4;
                } else {
                    break 'b_1cb9e0;
                }
            }
        }
        c.cycles += 1;
        c.g[S1][0] = (0u64 < c.g[V0][0]) as u64;
        break 'b_1cb9e4;
    }
    'b_1cba00: {
        c.cycles += 5;
        c.g[A0][0] = lw_9(c, at(c, S0, 0x140));
        c.g[V0][0] = sx(r32(c, A0).wrapping_sub(0xf));
        c.g[V0][0] = (c.g[V0][0] < 0xcu64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[S2][0] = lw_9(c, at(c, S0, 0x434));
        if t {
            break 'b_1cba00;
        } else {
            c.cycles += 2;
            c.g[V0][0] = lw_9(c, at(c, S0, 0x6c));
            c.g[S1][0] = c.g[V0][0] ^ 0x1;
            break 'b_1cba00;
        }
    }
    'b_1cbb5c: {
        'b_1cbb58: {
            'b_1cba6c: {
                'b_1cba68: {
                    c.cycles += 2;
                    let t = c.g[A1][0] == 0u64;
                    c.g[V1][0] = lw_9(c, at(c, S0, 0x430));
                    if t {
                        break 'b_1cba68;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = 0x1;
                        sw_4(c, at(c, S0, 0x434), c.g[A0][0]);
                        let t = c.g[V1][0] != 0u64;
                        sw_4(c, at(c, S0, 0x430), c.g[V0][0]);
                        if t {
                            break 'b_1cba68;
                        } else {
                            'b_1cba4c: {
                                c.cycles += 4;
                                c.g[V1][0] = lw_9(c, at(c, S0, 0x144));
                                c.g[V0][0] = 0x2;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.g[A0][0] = c.g[S0][0];
                                    break 'b_1cba4c;
                                } else {
                                    'b_1cba40: {
                                        c.cycles += 2;
                                        let t = c.g[S1][0] == 0u64;

                                        if t {
                                            break 'b_1cba40;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == 0u64;
                                            c.g[A0][0] = c.g[S0][0];
                                            if t {
                                                break 'b_1cba4c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[V1][0] = lw_9(c, at(c, S0, 0x140));
                                                if t {
                                                    break 'b_1cba6c;
                                                } else {
                                                    break 'b_1cba40;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    if c.g[V1][0] != 0u64 {
                                        c.g[V1][0] = lw_9(c, at(c, S0, 0x140));
                                        break 'b_1cba6c;
                                    } else {
                                        c.cycles += 1;
                                        c.g[A0][0] = c.g[S0][0];
                                        break 'b_1cba4c;
                                    }
                                }
                            }
                            c.cycles += 5;
                            c.g[A1][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001cba58);
                            c.g[A2][0] = 0u64;
                            if callee::C001CE560.call(c, 0x001cba58).is_err() {
                                return;
                            }
                            if c.g[V0][0] == 0u64 {
                                c.g[V1][0] = lw_9(c, at(c, S0, 0x140));
                                break 'b_1cba6c;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                sw_4(c, at(c, S0, 0x140), c.g[S2][0]);
                                if t {
                                    break 'b_1cbb58;
                                } else {
                                    break 'b_1cba68;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V1][0] = lw_9(c, at(c, S0, 0x140));
                break 'b_1cba6c;
            }
            'b_1cbac4: {
                'b_1cbabc: {
                    'b_1cba8c: {
                        c.cycles += 4;
                        c.g[V0][0] = sx(r32(c, V1).wrapping_sub(0x38));
                        c.g[V0][0] = (c.g[V0][0] < 0xeu64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_1cba8c;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001cba84);
                            c.g[A1][0] = c.g[S1][0];
                            if callee::C001CBFC0.call(c, 0x001cba84).is_err() {
                                return;
                            }
                            let t = true;

                            if t {
                                break 'b_1cbabc;
                            } else {
                                break 'b_1cba8c;
                            }
                        }
                    }
                    'b_1cbaac: {
                        c.cycles += 4;
                        c.g[V0][0] = sx(r32(c, V1).wrapping_sub(0x46));
                        c.g[V0][0] = (c.g[V0][0] < 0xeu64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = (c.g[V1][0] < 0xeu64) as u64;
                        if t {
                            break 'b_1cbaac;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001cbaa4);
                            c.g[A1][0] = c.g[S1][0];
                            if callee::C001CBCE8.call(c, 0x001cbaa4).is_err() {
                                return;
                            }
                            let t = true;

                            if t {
                                break 'b_1cbabc;
                            } else {
                                break 'b_1cbaac;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_1cbac4;
                    } else {
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x001cbabc);
                        c.g[A1][0] = c.g[S1][0];
                        if callee::C001CBC48.call(c, 0x001cbabc).is_err() {
                            return;
                        }
                        break 'b_1cbabc;
                    }
                }
                c.cycles += 2;
                if c.g[V0][0] != 0u64 {
                    frame.restore(c, S0, 0x40);
                    break 'b_1cbb5c;
                } else {
                    break 'b_1cbac4;
                }
            }
            'b_1cbae4: {
                'b_1cbae0: {
                    c.cycles += 5;
                    c.g[A0][0] = lw_9(c, at(c, S0, 0x140));
                    c.g[V1][0] = sx(r32(c, A0).wrapping_sub(0x46));
                    c.g[V0][0] = (c.g[V1][0] < 0xeu64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx(at(c, A0, 0xe));
                    if t {
                        break 'b_1cbae0;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        sw_4(c, at(c, S0, 0x140), c.g[V1][0]);
                        if t {
                            break 'b_1cbae4;
                        } else {
                            break 'b_1cbae0;
                        }
                    }
                }
                c.cycles += 1;
                sw_4(c, at(c, S0, 0x140), c.g[V0][0]);
                break 'b_1cbae4;
            }
            'b_1cbb2c: {
                c.cycles += 8;
                c.g[V0][0] = 0x440000;
                c.g[A1][0] = lw_9(c, at(c, S0, 0x140));
                c.g[A2][0] = sx(at(c, V0, 0x1390));
                c.g[A0][0] = 0x2a;
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, A2)));
                c.g[V1][0] = lb(c, r32(c, V0));
                if c.g[V1][0] != c.g[A0][0] {
                    c.g[A0][0] = c.g[S0][0];
                    break 'b_1cbb2c;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = sx(at(c, A1, 0x1));
                    'l_1cbb08: loop {
                        c.cycles += 7;
                        c.g[A1][0] = c.g[V0][0];
                        sw_4(c, at(c, S0, 0x140), c.g[V0][0]);
                        c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, A2)));
                        c.g[V0][0] = lb(c, r32(c, V1));
                        let t = c.g[V0][0] == c.g[A0][0];
                        c.g[V0][0] = sx(at(c, A1, 0x1));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x001cbb08);
                            }
                            continue 'l_1cbb08;
                        } else {
                            c.cycles += 2;
                            c.g[A1][0] = lw_9(c, at(c, S0, 0x140));
                            c.g[A0][0] = c.g[S0][0];
                            break 'b_1cbb2c;
                        }
                    }
                }
            }
            'b_1cbb54: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001cbb34);
                c.g[A2][0] = c.g[SP][0];
                if callee::C001CE560.call(c, 0x001cbb34).is_err() {
                    return;
                }
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x2;
                if t {
                    break 'b_1cbb54;
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001cbb48);
                    c.g[A1][0] = 0u64;
                    if callee::C001CB9B0.call(c, 0x001cbb48).is_err() {
                        return;
                    }
                    c.g[V0][0] = lw_9(c, r32(c, SP));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = (c.g[S1][0] < 0x1u64) as u64;
                    if t {
                        break 'b_1cbb58;
                    } else {
                        break 'b_1cbb54;
                    }
                }
            }
            c.cycles += 1;
            sw_4(c, at(c, S0, 0x144), c.g[V0][0]);
            break 'b_1cbb58;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x40);
        break 'b_1cbb5c;
    }
    c.cycles += 5;
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    c.g[RA][0] = ld(c, at(c, SP, 0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x50));
    c.pc = t;
    return;
}

/// `void func_001C9210(void* self, void* text, const char* str, float* size)`
/// (0x1c9210): measure a popup text box for a `\n`/`\\`-separated string: split
/// it into lines, measure each line with the text object, accumulate the size
/// (adding 3 pixels per line break) and free the temporary buffer.
pub fn c_fe_popup_confirm_get_text_box_size(c: &mut Cpu) {
    let frame = Frame::open(c, 0xc0);
    'b_1c933c: {
        c.cycles += 33;
        frame.save(c, S4, 0x60);
        frame.save(c, S7, 0x30);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S2, 0x80);
        c.g[S7][0] = c.g[A2][0];
        frame.save(c, S5, 0x50);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S0, 0xa0);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S1, 0x90);
        c.g[A0][0] = c.g[S7][0];
        frame.save(c, S3, 0x70);
        c.g[S0][0] = 0u64;
        frame.save(c, S6, 0x40);
        c.g[S3][0] = 0u64;
        sq(c, 30, at(c, SP, 0x20));
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x001c9260);
        c.swc1(20, at(c, SP, 0xb0));
        if call_ptr(c, 0x00416810, 0x001c9260).is_err() {
            return;
        }
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = sx(at(c, V0, 0x1));
        c.g[A1][0] = sx(at(c, A1, 0x60d8));
        c.g[A2][0] = 0x100;
        c.g[RA][0] = sx(0x001c9278);
        c.g[A3][0] = 0u64;
        if call_ptr(c, 0x00317e30, 0x001c9278).is_err() {
            return;
        }
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S7][0];
        c.g[RA][0] = sx(0x001c9288);
        sb(c, r32(c, S1), 0u64);
        if call_ptr(c, 0x00416810, 0x001c9288).is_err() {
            return;
        }
        c.g[S6][0] = c.g[V0][0];
        let t = (c.g[S6][0] as i64) <= 0;
        c.g[V0][0] = sx(r32(c, S7).wrapping_add(r32(c, S3)));
        if t {
            break 'b_1c933c;
        } else {
            c.cycles += 3;
            c.g[AT][0] = 0x40400000;
            c.mtc1(1, 20);
            c.g[FP][0] = 0xd;
            'l_1c92a0: loop {
                'b_1c9330: {
                    'b_1c9324: {
                        'b_1c9300: {
                            'b_1c92dc: {
                                c.cycles += 5;
                                c.g[A0][0] = lb(c, at(c, V0, 0x1));
                                c.g[V1][0] = lb(c, r32(c, V0));
                                c.g[V0][0] = 0x5c;
                                let t = c.g[V1][0] != c.g[V0][0];

                                if t {
                                    break 'b_1c92dc;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[A0][0] != c.g[V1][0];
                                    c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, S0)));
                                    if t {
                                        break 'b_1c92dc;
                                    } else {
                                        c.cycles += 8;
                                        c.g[A0][0] = c.g[S4][0];
                                        sb(c, r32(c, V0), 0u64);
                                        c.g[A1][0] = c.g[S5][0];
                                        c.g[A2][0] = c.g[S1][0];
                                        c.g[RA][0] = sx(0x001c92d4);
                                        c.g[A3][0] = c.g[SP][0];
                                        if callee::C001C84A8.call(c, 0x001c92d4).is_err() {
                                            return;
                                        }
                                        let t = true;
                                        c.g[S3][0] = sx(at(c, S3, 0x2));
                                        if t {
                                            break 'b_1c9300;
                                        } else {
                                            break 'b_1c92dc;
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[V1][0] != c.g[FP][0];
                            c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, S0)));
                            if t {
                                break 'b_1c9324;
                            } else {
                                c.cycles += 7;
                                c.g[A0][0] = c.g[S4][0];
                                sb(c, r32(c, V0), 0u64);
                                c.g[A1][0] = c.g[S5][0];
                                c.g[A2][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x001c92fc);
                                c.g[A3][0] = c.g[SP][0];
                                if callee::C001C84A8.call(c, 0x001c92fc).is_err() {
                                    return;
                                }
                                c.g[S3][0] = sx(at(c, S3, 0x1));
                                break 'b_1c9300;
                            }
                        }
                        c.cycles += 9;
                        c.g[A0][0] = c.g[S4][0];
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x001c9314);
                        c.g[S0][0] = 0u64;
                        if callee::C001C9B68.call(c, 0x001c9314).is_err() {
                            return;
                        }
                        c.lwc1(0, at(c, S2, 0x4));
                        c.add_s(0, 0, 20);
                        let t = true;
                        c.swc1(0, at(c, S2, 0x4));
                        if t {
                            break 'b_1c9330;
                        } else {
                            break 'b_1c9324;
                        }
                    }
                    c.cycles += 3;
                    c.g[S3][0] = sx(at(c, S3, 0x1));
                    sb(c, r32(c, V0), c.g[V1][0]);
                    c.g[S0][0] = sx(at(c, S0, 0x1));
                    break 'b_1c9330;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S6][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = sx(r32(c, S7).wrapping_add(r32(c, S3)));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x001c92a0);
                    }
                    continue 'l_1c92a0;
                } else {
                    break 'b_1c933c;
                }
            }
        }
    }
    'b_1c936c: {
        c.cycles += 2;
        let t = c.g[S0][0] == 0u64;
        c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, S0)));
        if t {
            break 'b_1c936c;
        } else {
            c.cycles += 10;
            c.g[A1][0] = c.g[S5][0];
            c.g[A0][0] = c.g[S4][0];
            c.g[A2][0] = c.g[S1][0];
            sb(c, r32(c, V0), 0u64);
            c.g[RA][0] = sx(0x001c935c);
            c.g[A3][0] = c.g[SP][0];
            if callee::C001C84A8.call(c, 0x001c935c).is_err() {
                return;
            }
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001c936c);
            c.g[A2][0] = c.g[SP][0];
            if callee::C001C9B68.call(c, 0x001c936c).is_err() {
                return;
            }
            break 'b_1c936c;
        }
    }
    'b_1c9380: {
        c.cycles += 2;
        let t = c.g[S1][0] == 0u64;
        frame.restore(c, S0, 0xa0);
        if t {
            break 'b_1c9380;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001c937c);
            c.g[A0][0] = c.g[S1][0];
            if call_ptr(c, 0x00317e98, 0x001c937c).is_err() {
                return;
            }
            frame.restore(c, S0, 0xa0);
            break 'b_1c9380;
        }
    }
    c.cycles += 12;
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    frame.restore(c, S7, 0x30);
    lq(c, 30, at(c, SP, 0x20));
    c.g[RA][0] = ld(c, at(c, SP, 0x10));
    c.lwc1(20, at(c, SP, 0xb0));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0xc0));
    c.pc = t;
    return;
}

/// `void func_001C9038(void* self, void* text, const u16* str, float* size)`
/// (0x1c9038): the wide-character twin of `cFEPopupConfirm_GetTextBoxSize`:
/// split the UTF-16 string at `\r`/`\\`, measure each line, accumulate the size
/// and free the temporary buffer.
pub fn c_fe_popup_confirm_get_text_box_size_w(c: &mut Cpu) {
    let frame = Frame::open(c, 0xc0);
    'b_1c9180: {
        c.cycles += 34;
        frame.save(c, S0, 0xa0);
        frame.save(c, S5, 0x50);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S3, 0x70);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S7, 0x30);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S2, 0x80);
        c.g[S7][0] = c.g[A1][0];
        frame.save(c, S4, 0x60);
        c.g[A0][0] = c.g[S0][0];
        frame.save(c, S6, 0x40);
        c.g[S2][0] = 0u64;
        sq(c, 30, at(c, SP, 0x20));
        c.g[S6][0] = 0u64;
        frame.save(c, S1, 0x90);
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x001c9088);
        c.swc1(20, at(c, SP, 0xb0));
        if call_ptr(c, 0x003dbca0, 0x001c9088).is_err() {
            return;
        }
        c.g[V0][0] = sx(at(c, V0, 0x1));
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = sx(r32(c, V0) << 1);
        c.g[A1][0] = sx(at(c, A1, 0x60d8));
        c.g[A2][0] = 0x100;
        c.g[RA][0] = sx(0x001c90a4);
        c.g[A3][0] = 0u64;
        if call_ptr(c, 0x00317e30, 0x001c90a4).is_err() {
            return;
        }
        c.g[S4][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x001c90b4);
        sh(c, r32(c, S4), 0u64);
        if call_ptr(c, 0x003dbca0, 0x001c90b4).is_err() {
            return;
        }
        c.g[FP][0] = c.g[V0][0];
        let t = (c.g[FP][0] as i64) <= 0;

        if t {
            break 'b_1c9180;
        } else {
            c.cycles += 6;
            c.g[S1][0] = c.g[S0][0];
            c.g[AT][0] = 0x40400000;
            c.mtc1(1, 20);
            c.g[S0][0] = c.g[S4][0];
            c.g[V1][0] = lhu(c, r32(c, S1));
            'l_1c90d8: loop {
                'b_1c9174: {
                    'b_1c9164: {
                        'b_1c913c: {
                            'b_1c9114: {
                                'b_1c9110: {
                                    c.cycles += 3;
                                    c.g[V0][0] = lb(c, r32(c, GP).wrapping_sub(0x14b8));
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[A0][0] = lhu(c, at(c, S1, 0x2));
                                    if t {
                                        break 'b_1c9110;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[A0][0] != c.g[V1][0];
                                        c.g[V0][0] = 0xd;
                                        if t {
                                            break 'b_1c9114;
                                        } else {
                                            c.cycles += 9;
                                            sh(c, r32(c, S0), 0u64);
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S7][0];
                                            c.g[A2][0] = c.g[S4][0];
                                            c.g[A3][0] = c.g[SP][0];
                                            c.g[RA][0] = sx(0x001c9108);
                                            c.g[S1][0] = sx(at(c, S1, 0x4));
                                            if callee::C001C8568.call(c, 0x001c9108).is_err() {
                                                return;
                                            }
                                            let t = true;
                                            c.g[S6][0] = sx(at(c, S6, 0x2));
                                            if t {
                                                break 'b_1c913c;
                                            } else {
                                                break 'b_1c9110;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[V0][0] = 0xd;
                                break 'b_1c9114;
                            }
                            c.cycles += 2;
                            if c.g[V1][0] != c.g[V0][0] {
                                sh(c, r32(c, S0), c.g[V1][0]);
                                break 'b_1c9164;
                            } else {
                                c.cycles += 8;
                                sh(c, r32(c, S0), 0u64);
                                c.g[A0][0] = c.g[S5][0];
                                c.g[A1][0] = c.g[S7][0];
                                c.g[A2][0] = c.g[S4][0];
                                c.g[A3][0] = c.g[SP][0];
                                c.g[RA][0] = sx(0x001c9138);
                                c.g[S1][0] = sx(at(c, S1, 0x2));
                                if callee::C001C8568.call(c, 0x001c9138).is_err() {
                                    return;
                                }
                                c.g[S6][0] = sx(at(c, S6, 0x1));
                                break 'b_1c913c;
                            }
                        }
                        c.cycles += 10;
                        c.g[A0][0] = c.g[S5][0];
                        c.g[A1][0] = c.g[S3][0];
                        c.g[A2][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x001c9150);
                        c.g[S0][0] = c.g[S4][0];
                        if callee::C001C9B68.call(c, 0x001c9150).is_err() {
                            return;
                        }
                        c.g[S2][0] = 0u64;
                        c.lwc1(0, at(c, S3, 0x4));
                        c.add_s(0, 0, 20);
                        let t = true;
                        c.swc1(0, at(c, S3, 0x4));
                        if t {
                            break 'b_1c9174;
                        } else {
                            break 'b_1c9164;
                        }
                    }
                    c.cycles += 4;
                    c.g[S1][0] = sx(at(c, S1, 0x2));
                    c.g[S6][0] = sx(at(c, S6, 0x1));
                    c.g[S0][0] = sx(at(c, S0, 0x2));
                    c.g[S2][0] = sx(at(c, S2, 0x1));
                    break 'b_1c9174;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[FP][0] as i64)) as u64;
                if c.g[V0][0] != 0u64 {
                    c.g[V1][0] = lhu(c, r32(c, S1));
                    if event_due(c) {
                        deliver_event_here(c, 0x001c90d8);
                    }
                    continue 'l_1c90d8;
                } else {
                    break 'b_1c9180;
                }
            }
        }
    }
    'b_1c91c8: {
        c.cycles += 2;
        let t = c.g[S2][0] == 0u64;
        c.g[V0][0] = sx(r32(c, S2) << 1);
        if t {
            break 'b_1c91c8;
        } else {
            c.cycles += 16;
            c.g[A1][0] = c.g[S7][0];
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S4)));
            c.g[A0][0] = c.g[S5][0];
            sh(c, r32(c, V0), 0u64);
            c.g[A2][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x001c91a4);
            c.g[A3][0] = c.g[SP][0];
            if callee::C001C8568.call(c, 0x001c91a4).is_err() {
                return;
            }
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x001c91b4);
            c.g[A2][0] = c.g[SP][0];
            if callee::C001C9B68.call(c, 0x001c91b4).is_err() {
                return;
            }
            c.lwc1(0, at(c, S3, 0x4));
            c.g[AT][0] = 0x40400000;
            c.mtc1(1, 1);
            c.add_s(0, 0, 1);
            c.swc1(0, at(c, S3, 0x4));
            break 'b_1c91c8;
        }
    }
    'b_1c91dc: {
        c.cycles += 2;
        let t = c.g[S4][0] == 0u64;
        frame.restore(c, S0, 0xa0);
        if t {
            break 'b_1c91dc;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001c91d8);
            c.g[A0][0] = c.g[S4][0];
            if call_ptr(c, 0x00317e98, 0x001c91d8).is_err() {
                return;
            }
            frame.restore(c, S0, 0xa0);
            break 'b_1c91dc;
        }
    }
    c.cycles += 12;
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    frame.restore(c, S7, 0x30);
    lq(c, 30, at(c, SP, 0x20));
    c.g[RA][0] = ld(c, at(c, SP, 0x10));
    c.lwc1(20, at(c, SP, 0xb0));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0xc0));
    c.pc = t;
    return;
}

/// `void cFEPopupConfirm_MeasureDesc(TextObj *text, Vec3 *scale, int *info,
/// float *out)` (0x1c8320): scale the text object's layout size by the
/// caller's scale and measure the description: a plain or a wrapped single
/// line for info[0] 1/2, a looked-up string (or sprintf'd number) for 3, and
/// two zero words for 0.
pub fn cfepopupconfirm_measure_desc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, 17, 0x80);
        frame.save(c, 18, 0x70);
        let text64 = abi::arg64(c, 1);
        frame.save(c, 19, 0x60);
        c.g[S1][0] = text64;
        c.g[S2][0] = abi::arg64(c, 2);
        frame.save(c, 16, 0x90);
        let text = text64 as u32;
        c.g[A0][0] = text64;
        frame.save_ra(c, 0x50);
        // Copy the text object's layout scale (three words from +0x50).
        c.ldl(2, text.wrapping_add(0x57));
        c.ldr(2, text.wrapping_add(0x50));
        c.g[V1][0] = c.lw(text.wrapping_add(0x58));
        c.sdl(2, (c.g[SP][0] as u32).wrapping_add(7));
        c.sdr(2, c.g[SP][0] as u32);
        c.sw((c.g[SP][0] as u32).wrapping_add(8), c.g[V1][0]);
        c.g[S3][0] = abi::arg64(c, 3);
        callee::TEXT_OBJ.call(c, 0x001c_8364)?;

        // g->size.x *= v.x, g->size.y *= v.y
        c.lwc1(2, c.g[SP][0] as u32);
        c.g[V1][0] = 1;
        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(4));
        let g = c.g[V0][0] as u32;
        c.lwc1(0, g.wrapping_add(0x30));
        c.lwc1(1, g.wrapping_add(0x34));
        c.mul_s(0, 0, 2);
        c.mul_s(1, 1, 3);
        c.swc1(0, g.wrapping_add(0x38));
        c.swc1(1, g.wrapping_add(0x3c));

        let info = c.g[S2][0] as u32;
        let out = c.g[S3][0];
        c.g[A0][0] = c.lw(info);
        let sel = c.g[A0][0];
        let v0 = ((c.g[A0][0] as i64) < 2) as u64;
        c.g[V0][0] = v0;
        if sel == c.g[V1][0] {
            // case 1
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            callee::TEXT_OBJ.call(c, 0x001c_83cc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.lw(info.wrapping_add(4));
            measure_line(c, callee::MEASURE_PLAIN, out, 0x001c_843c)?;
        } else {
            c.cycles += 2;
            if v0 == 0 {
                c.g[V0][0] = 2;
            } else {
                c.cycles += 2;
                if sel == 0 {
                    frame.restore(c, 16, 0x90);
                    // case 0: two zero words.
                    c.cycles += 2;
                    c.sw((out as u32).wrapping_add(0xc), 0);
                    c.sw((out as u32).wrapping_add(8), 0);
                    c.cycles += 1;
                    frame.restore(c, 17, 0x80);
                    return epilogue(c, frame);
                }
                c.cycles += 2;
                frame.restore(c, 17, 0x80);
                return epilogue(c, frame);
            }
            c.cycles += 2;
            let is_two = sel == c.g[V0][0];
            c.g[V0][0] = 3;
            if is_two {
                // case 2
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                callee::TEXT_OBJ.call(c, 0x001c_83e0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw(info.wrapping_add(4));
                measure_line(c, callee::MEASURE_WRAP, out, 0x001c_847c)?;
            } else {
                c.cycles += 2;
                let is_three = sel == c.g[V0][0];
                frame.restore(c, 16, 0x90);
                if !is_three {
                    c.cycles += 2;
                    frame.restore(c, 17, 0x80);
                    return epilogue(c, frame);
                }
                // case 3: look the description up in the world's UI object.
                c.cycles += 11;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A1][0] = c.lw(info.wrapping_add(4));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(4));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                let f = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                crate::call::call_ptr(c, f, 0x001c_840c)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] == 0 {
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    // sprintf(buf, "%d", info[1]) then measure wrapped.
                    c.cycles += 9;
                    c.g[A2][0] = c.lw(info.wrapping_add(4));
                    c.g[A1][0] = 0x0046_0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60a8));
                    callee::SPRINTF_4.call(c, 0x001c_8458)?;
                    c.g[A0][0] = c.g[S1][0];
                    callee::TEXT_OBJ.call(c, 0x001c_8460)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    measure_line(c, callee::MEASURE_WRAP, out, 0x001c_847c)?;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    callee::TEXT_OBJ.call(c, 0x001c_8420)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    measure_line(c, callee::MEASURE_PLAIN, out, 0x001c_843c)?;
                }
            }
        }

        frame.restore(c, 16, 0x90);
        c.cycles += 1;
        frame.restore(c, 17, 0x80);
        epilogue(c, frame)
    })
}

/// Measure one line into `out`: `f(g, str, out, 0, g->size.x, g->size.y)`.
pub(crate) fn measure_line(c: &mut Cpu, f: Callee, out: u64, ret: u32) -> crate::call::Flow {
    c.cycles += 7;
    c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
    c.g[A2][0] = out;
    c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
    c.g[A3][0] = 0;
    f.call(c, ret)
}

/// The shared tail of [`cfepopupconfirm_measure_desc`].
pub(crate) fn epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 5;
    frame.restore(c, 18, 0x70);
    frame.restore(c, 19, 0x60);
    frame.restore_ra(c, 0x50);
    frame.close(c);
    Ok(())
}

const PCF_APPLY_CONFIG: Callee = Callee {
    addr: 0x001c_6b08,
    f: ee::s03::f_001c6b08,
};
const PCF_APPLY_BACKGROUND_SCALE: Callee = Callee {
    addr: 0x001c_7738,
    f: ee::s03::f_001c7738,
};
const PCF_SET_FADE: Callee = Callee {
    addr: 0x001c_5f20,
    f: ee::s03::f_001c5f20,
};

/// `base(reg) + off`.
#[inline(always)]
fn pcf_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `int cFEPopupConfirm_v07(cFEPopupConfirm* self, int first)` (0x1c5c30),
/// the popup's `TransitionIn` (state vtable slot `+0x38`): on the first call
/// apply its config (`+0xbc`) at the start scale, fade it in when it has a
/// background (`+0x31c == 1`) and set up the grow-in: `+0xa8` steps (the
/// screen's value, 25 without one, at least 2), per-step scale `+0xac/+0xb0`
/// toward the final scale `+0x278..`. Each call (`UIStateMgr_ProcessStates`,
/// once per tick until it returns nonzero) applies `step * per-step + base`
/// and takes the next step (`+0xa4`), then returns 1 at the last one (or at
/// once with the final scale without a background).
///
/// Tick rate: the grow-in is a 60 Hz timeline: the step counter goes up
/// once per 60 Hz frame of the popup ([`crate::tick::frame60_due`]); exact
/// at 60.
pub fn cfe_popup_confirm_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        c.g[S1][0] = c.g[A0][0];
        let first = c.g[A1][0] != 0;
        frame.save_ra(c, 0x20);
        if first {
            c.cycles += 16;
            let s = sp(c);
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5908));
            c.g[S0][0] = 1;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.sw(pcf_at(c, S1, 0x2bc), c.g[S0][0]);
            c.g[A1][0] = sx(pcf_at(c, S1, 0xbc));
            c.swc1(1, s.wrapping_add(4));
            c.swc1(0, s.wrapping_add(8));
            c.swc1(1, s);
            PCF_APPLY_CONFIG.call(c, 0x001c_5c70)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[SP][0];
            PCF_APPLY_BACKGROUND_SCALE.call(c, 0x001c_5c7c)?;
            c.g[V0][0] = c.lw(pcf_at(c, S1, 0x31c));
            if c.g[V0][0] == c.g[S0][0] {
                c.cycles += 4;
                c.mtc1(0, 12);
                c.g[A0][0] = c.g[S1][0];
                PCF_SET_FADE.call(c, 0x001c_5c94)?;
            }
            // The step count: the screen's value for the popup's name hash.
            c.g[A0][0] = 0x4a_0000;
            c.cycles += 9;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c48));
            call_ptr(c, 0x0031_7670, 0x001c_5ca0)?; // StrHash
            c.g[A0][0] = c.lw(pcf_at(c, S1, 0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_c778, 0x001c_5cac)?; // UIScreen_LookupValue
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = 0xffff;
            let missing = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x19;
            if missing {
                c.cycles += 1;
                c.sw(pcf_at(c, S1, 0xa8), c.g[V0][0]);
            } else {
                c.cycles += 2;
                c.sw(pcf_at(c, S1, 0xa8), c.g[V1][0]);
            }
            c.cycles += 4;
            c.g[V0][0] = c.lw(pcf_at(c, S1, 0xa8));
            c.g[V1][0] = 1;
            if c.g[V0][0] == c.g[V1][0] {
                c.cycles += 3;
                c.g[V0][0] = 2;
                c.sw(pcf_at(c, S1, 0xa8), c.g[V0][0]);
                c.g[V0][0] = c.lw(pcf_at(c, S1, 0xa8));
            }
            // Per-step scale = (final - start) / (steps - 1); step 1.
            c.cycles += 13;
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5904));
            c.lwc1(1, pcf_at(c, S1, 0x278));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            c.lwc1(0, pcf_at(c, S1, 0x27c));
            c.mtc1(2, 2);
            c.cvt_s_w(2, 2);
            c.sub_s(1, 1, 3);
            c.sw(pcf_at(c, S1, 0xa4), c.g[V1][0]);
            c.sub_s(0, 0, 3);
            c.div_s(1, 1, 2);
            c.div_s(0, 0, 2);
            c.swc1(1, pcf_at(c, S1, 0xac));
            c.swc1(0, pcf_at(c, S1, 0xb0));
        }

        c.cycles += 4;
        c.g[V1][0] = c.lw(pcf_at(c, S1, 0x31c));
        c.g[V0][0] = 1;
        let growing = c.g[V1][0] == c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        'out: {
            if growing {
                c.cycles += 25;
                let s = sp(c);
                c.lwc1(1, pcf_at(c, S1, 0xa4));
                c.cvt_s_w(1, 1);
                c.lwc1(0, pcf_at(c, S1, 0xb0));
                c.lwc1(3, pcf_at(c, S1, 0xac));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5900));
                c.g[A1][0] = sx(s.wrapping_add(0x10));
                c.mul_s(0, 1, 0);
                c.g[V0][0] = c.lw(pcf_at(c, S1, 0x58));
                c.mul_s(1, 1, 3);
                c.ldl(3, pcf_at(c, V0, 0x4b));
                c.ldr(3, pcf_at(c, V0, 0x44));
                c.g[A2][0] = c.lw(pcf_at(c, V0, 0x4c));
                c.sdl(3, s.wrapping_add(7));
                c.sdr(3, s);
                c.sw(s.wrapping_add(8), c.g[A2][0]);
                c.add_s(0, 0, 2);
                c.add_s(1, 1, 2);
                c.swc1(0, s.wrapping_add(0x14));
                c.swc1(1, s.wrapping_add(0x10));
                PCF_APPLY_BACKGROUND_SCALE.call(c, 0x001c_5d78)?;
                c.g[V1][0] = c.lw(pcf_at(c, S1, 0xa4));
                c.g[V0][0] = c.lw(pcf_at(c, S1, 0xa8));
                let more = (c.g[V1][0] as i64) < (c.g[V0][0] as i64);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                    break 'out;
                }
                c.cycles += 3;
                c.g[V0][0] = 0;
                // The next step once per 60 Hz frame of this popup.
                let key = pcf_at(c, S1, 0xa4);
                let step = c.g[V1][0] as u32;
                if crate::tick::frame60_due(c, key, step.wrapping_sub(1)) {
                    c.sw(key, c.g[V1][0]);
                    crate::tick::frame60_ran(c, key, step);
                }
                break 'out;
            }
            // No background: the final scale at once.
            c.cycles += 8;
            let s = sp(c);
            c.ldl(2, pcf_at(c, S1, 0x27f));
            c.ldr(2, pcf_at(c, S1, 0x278));
            c.g[V1][0] = c.lw(pcf_at(c, S1, 0x280));
            c.sdl(2, s.wrapping_add(7));
            c.sdr(2, s);
            c.sw(s.wrapping_add(8), c.g[V1][0]);
            c.g[A1][0] = c.g[SP][0];
            PCF_APPLY_BACKGROUND_SCALE.call(c, 0x001c_5db8)?;
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// The guest address `c.g[reg] + off` (the generated
/// `(c.g[reg][0] as u32).wrapping_add(off)`).
pub(crate) fn oc508_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// The destructor shape of the FE states and popups: `this->vtable` (the
/// word at +8) is set to `vt`, then the base destructor `base(this)` is
/// called, returning to `ret`.
pub(crate) fn oc508_vtable_dtor(c: &mut Cpu, vt: u64, base: u32, ret: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = vt;
    c.sw(oc508_at(c, V1, 0x8), c.g[V0][0]);
    call_ptr(c, base, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cFEPopupConfirm` base-constructor variant (0x1ddbd0): the base UI state
/// (0x39e318) with no arguments, the vtable 0x46cbd8 at +8, the member at
/// +0xbc set up (0x1c55f8), then the words at +0x2b0 and +0x2b4 take the
/// game word at gp+0xda0. Returns `this`.
pub fn c_fe_popup_confirm_base_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e318, 0x001d_dbec)?;
        c.g[V1][0] = sx(0x0047_0000 - 0x3428);
        c.g[A0][0] = sx(oc508_at(c, S0, 0xbc));
        c.sw(oc508_at(c, S0, 0x8), c.g[V1][0]);
        call_ptr(c, 0x001c_55f8, 0x001d_dc00)?;
        c.g[V1][0] = c.lw(oc508_at(c, GP, 0xda0));
        c.g[V0][0] = c.g[S0][0];
        c.sw(oc508_at(c, S0, 0x2b4), c.g[V1][0]);
        c.sw(oc508_at(c, S0, 0x2b0), c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUIStateBuyPopup::cUIStateBuyPopup(this)` variant (0x1ddc48): the base
/// UI state (0x39e318), the vtable 0x46cb08 at +8, then the parameter block
/// at +0x48 is reset (0x1ca8a0). Returns `this`.
pub fn c_ui_state_buy_popup_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e318, 0x001d_dc64)?;
        c.g[V1][0] = sx(0x0047_0000 - 0x34f8);
        c.g[A0][0] = sx(oc508_at(c, S0, 0x48));
        c.sw(oc508_at(c, S0, 0x8), c.g[V1][0]);
        call_ptr(c, 0x001c_a8a0, 0x001d_dc78)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUIStateBuyPopup::~cUIStateBuyPopup(this)` (0x1ddc90).
pub fn c_ui_state_buy_popup_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc508_vtable_dtor(c, sx(0x0047_0000 - 0x2f30), 0x0039_e390, 0x001d_dcac)
    })
}

/// The parameter thunks of the buy popup (+0x48 is the block the
/// `cBuyPopupInfo` setter fills): `target(this + 0x48)`, returning to `ret`.
pub(crate) fn oc508_params_thunk(c: &mut Cpu, target: u32, ret: u32) -> Flow {
    c.cycles += 7;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A0][0] = sx(oc508_at(c, A0, 0x48));
    call_ptr(c, target, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cUIStateBuyPopup::SetSongCreditParams` thunk (0x1ddcc8).
pub fn c_ui_state_buy_popup_set_song_credit_params_thunk(c: &mut Cpu) {
    guest(c, |c| oc508_params_thunk(c, 0x001c_a900, 0x001d_dcd8))
}

/// `cUIStateBuyPopup::SetMusicTrackParams` thunk (0x1ddce8).
pub fn c_ui_state_buy_popup_set_music_track_params_thunk(c: &mut Cpu) {
    guest(c, |c| oc508_params_thunk(c, 0x001c_a980, 0x001d_dcf8))
}

/// `cUIStateBuyPopup::SetRewardItemParams` thunk (0x1ddd08).
pub fn c_ui_state_buy_popup_set_reward_item_params_thunk(c: &mut Cpu) {
    guest(c, |c| oc508_params_thunk(c, 0x001c_aa18, 0x001d_dd18))
}

/// `cFEPopupConfirm::cFEPopupConfirm(this)` variant (0x1dde78): the base
/// popup (0x1c58e8) with no arguments, the vtable 0x46c890 at +8. Returns
/// `this`.
pub fn c_fe_popup_confirm_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001c_58e8, 0x001d_de94)?;
        c.g[V1][0] = sx(0x0047_0000 - 0x3770);
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        c.sw(oc508_at(c, S0, 0x8), c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cFEPopupConfirm::~cFEPopupConfirm(this)` (0x1ddeb8): vtable 0x46c890,
/// then the base popup destructor (0x1c5a90).
pub fn c_fe_popup_confirm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc508_vtable_dtor(c, sx(0x0047_0000 - 0x3770), 0x001c_5a90, 0x001d_ded4)
    })
}

/// `cKeyboardPopup::cKeyboardPopup` (0x1de288, `_ctor_2`, 76 bytes): the
/// `UIState_ctor_2` base constructor with a1 = a2 = 0, then the vptr, the
/// popup's character limit (0x3f at +0x60) and the `gp`-relative default
/// (`lw gp+0xda0`) at +0x424.
pub fn ckeyboard_popup_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, FE_UI_STATE_CTOR_2, 0x001d_e2a4)?;
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xda0)).lw(c);
        c.g[V1][0] = fe_vptr(0x4138);
        c.g[A0][0] = 0x3f;
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x60)).sw(c, c.g[A0][0]);
        Addr((c.g[S0][0] as u32).wrapping_add(0x424)).sw(c, c.g[A1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cKeyboardPopup::GetTextLength` (0x1de2e8): `strlen(this + 0x74)`, its
/// result returned as is.
pub fn ckeyboard_popup_get_text_length(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
        call_ptr(c, FE_STRLEN, 0x001d_e2f8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cKeyboardPopup::SetKey4BEnabled(popup, enabled)` (0x1de350): the key
/// 0x4b (the 4B key) set through `cKeyboardPopup_SetKeyEnabled` with a2 = the
/// enable flag and a3 = 2.
pub fn ckeyboard_popup_set_key4b_enabled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x4b;
        c.g[A3][0] = 0x2;
        call_ptr(c, FE_KEYBOARD_SET_KEY_ENABLED, 0x001d_e368)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cKeyboardPopup_SetKeyEnabled(this, index, value, key)` (0x1ce3c8), the
/// callee of the five `SetKeyNNEnabled` wrappers below.
pub(crate) const FE_KEYBOARD_SET_KEY_ENABLED_2: Callee = Callee {
    addr: 0x001c_e3c8,
    f: ee::s04::cKeyboardPopup_SetKeyEnabled,
};

/// The keyboard-popup wrappers (`cKeyboardPopup_setIncludeKey` family): a
/// 0x10 frame holding `ra`, `a2 = a1` (the caller's flag), `a1 = index`,
/// `a3 = 2` (the key), then `cKeyboardPopup_SetKeyEnabled` returning to `ret`.
pub(crate) fn keyboard_popup_set_key_wrapper(c: &mut Cpu, index: u64, ret: u32) -> Flow {
    c.cycles += 9;
    let frame = Frame::open(c, 0x10);
    c.g[A2][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[A1][0] = index;
    c.g[A3][0] = 2;
    FE_KEYBOARD_SET_KEY_ENABLED_2.call(c, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cKeyboardPopup_SetKey1CEnabled(this, flag)` (0x1de378): key 0x1c (the
/// "include tab" key) enabled from `flag`.
pub fn c_keyboard_popup_set_key_1c_enabled(c: &mut Cpu) {
    guest(c, |c| keyboard_popup_set_key_wrapper(c, 0x1c, 0x001d_e390));
}

/// `cKeyboardPopup_SetKey45Enabled(this, flag)` (0x1de3a0).
pub fn c_keyboard_popup_set_key_45_enabled(c: &mut Cpu) {
    guest(c, |c| keyboard_popup_set_key_wrapper(c, 0x45, 0x001d_e3b8));
}

/// `cKeyboardPopup_SetKey4FEnabled(this, flag)` (0x1de3c8).
pub fn c_keyboard_popup_set_key_4f_enabled(c: &mut Cpu) {
    guest(c, |c| keyboard_popup_set_key_wrapper(c, 0x4f, 0x001d_e3e0));
}

/// `cKeyboardPopup_SetKey29Enabled(this, flag)` (0x1de3f0).
pub fn c_keyboard_popup_set_key_29_enabled(c: &mut Cpu) {
    guest(c, |c| keyboard_popup_set_key_wrapper(c, 0x29, 0x001d_e408));
}

/// `cKeyboardPopup_SetKey42Enabled(this, flag)` (0x1de418).
pub fn c_keyboard_popup_set_key_42_enabled(c: &mut Cpu) {
    guest(c, |c| keyboard_popup_set_key_wrapper(c, 0x42, 0x001d_e430));
}

/// `cFEPopupConfirm_OnWidgetNotify` (0x001ca180): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_popup_confirm_on_widget_notify(c: &mut Cpu) {
    if c.pc != 0x001ca180 {
        return ee::s03::f_001ca180__generated(c);
    }
    guest(c, |c| 'l_1ca180: loop {
        'b_1ca278: {
            'b_1ca274: {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S2][0] = c.g[A1][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[V0][0] == 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_1ca274;
                } else {
                    'b_1ca264: {
                        'b_1ca254: {
                            'b_1ca24c: {
                                c.cycles += 7;
                                c.g[V0][0] = 0x460000;
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6130));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                let t = c.g[A0][0] as u32;
                                match t {
                                    0x001ca180 => {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x001ca180);
                                        }
                                        continue 'l_1ca180;
                                    }
                                    0x001ca1c8 => {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x96));
                                        c.g[S0][0] = 0u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[S3][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x95));
                                        if t {
                                            break 'b_1ca274;
                                        } else {
                                            c.cycles += 2;
                                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x74));
                                            'l_1ca1e0: loop {
                                                'b_1ca1f8: {
                                                    c.cycles += 6;
                                                    c.g[RA][0] = sx(0x001ca1e8);
                                                    c.g[A1][0] = c.g[S0][0];
                                                    call_ptr(c, 0x00397870, 0x001ca1e8)?;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0x88));
                                                    c.g[A2][0] = c.g[V0][0];
                                                    if c.g[A2][0] == c.g[V1][0] {
                                                        c.g[A2][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x84));
                                                        break 'b_1ca1f8;
                                                    } else {
                                                        break 'b_1ca1f8;
                                                    }
                                                }
                                                'b_1ca234: {
                                                    'b_1ca21c: {
                                                        c.cycles += 2;
                                                        let t = c.g[S0][0] != c.g[S3][0];
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x8));
                                                        if t {
                                                            break 'b_1ca21c;
                                                        } else {
                                                            c.cycles += 7;
                                                            c.g[A1][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x32c));
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x58));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x5c));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[RA][0] = sx(0x001ca214);
                                                            c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            call_ptr(c, t, 0x001ca214)?;
                                                            let t = true;
                                                            c.g[V0][0] = c.lbu(
                                                                (c.g[S2][0] as u32)
                                                                    .wrapping_add(0x96),
                                                            );
                                                            if t {
                                                                break 'b_1ca234;
                                                            } else {
                                                                break 'b_1ca21c;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 6;
                                                    c.g[A1][0] =
                                                        sx((c.g[S1][0] as u32).wrapping_add(0x33c));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[RA][0] = sx(0x001ca230);
                                                    c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x001ca230)?;
                                                    c.g[V0][0] = c.lbu(
                                                        (c.g[S2][0] as u32).wrapping_add(0x96),
                                                    );
                                                    break 'b_1ca234;
                                                }
                                                c.cycles += 4;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                c.g[V0][0] = ((c.g[S0][0] as i64)
                                                    < (c.g[V0][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[A0][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_add(0x74));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x001ca1e0);
                                                    }
                                                    continue 'l_1ca1e0;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[V0][0] = 0x101;
                                                    if t {
                                                        break 'b_1ca278;
                                                    } else {
                                                        break 'b_1ca24c;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    0x001ca24c => {
                                        break 'b_1ca24c;
                                    }
                                    0x001ca254 => {
                                        break 'b_1ca254;
                                    }
                                    0x001ca264 => {
                                        break 'b_1ca264;
                                    }
                                    _ => {
                                        c.jump_out(t, 0x001ca180, 0x001ca294);
                                        return Ok(());
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_1ca278;
                            } else {
                                break 'b_1ca254;
                            }
                        }
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x310));
                        c.g[V0][0] = 0x101;
                        let t = true;
                        if c.g[V1][0] == 0 {
                            c.g[V0][0] = 0u64;
                        }
                        if t {
                            break 'b_1ca278;
                        } else {
                            break 'b_1ca264;
                        }
                    }
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xb8));
                    c.g[V0][0] = 0x101;
                    let t = true;
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = 0u64;
                    }
                    if t {
                        break 'b_1ca278;
                    } else {
                        break 'b_1ca274;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x101;
            break 'b_1ca278;
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEPopupConfirm_OnWidgetAction` (0x001ca298): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_popup_confirm_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001ca298 {
        return ee::s03::f_001ca298__generated(c);
    }
    guest(c, |c| 'l_1ca298: loop {
        'b_1ca478: {
            'b_1ca474: {
                'b_1ca458: {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = (c.g[V1][0] < 0x9u64) as u64;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A0][0];
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    let t = c.g[V0][0] == 0u64;
                    c.g[S1][0] = c.g[A1][0];
                    if t {
                        break 'b_1ca458;
                    } else {
                        'b_1ca41c: {
                            'b_1ca3ec: {
                                c.cycles += 7;
                                c.g[V0][0] = 0x460000;
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6150));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                let t = c.g[A0][0] as u32;
                                match t {
                                    0x001ca298 => {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x001ca298);
                                        }
                                        continue 'l_1ca298;
                                    }
                                    0x001ca2d8 => {
                                        'b_1ca3e0: {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x30c));
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A2][0] = 0u64;
                                            if t {
                                                break 'b_1ca3e0;
                                            } else {
                                                c.cycles += 5;
                                                c.sw((c.g[S0][0] as u32).wrapping_add(0xb8), 0u64);
                                                c.g[A1][0] = 0u64;
                                                c.g[A0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x2d0));
                                                c.g[V1][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x90));
                                                'l_1ca2f8: loop {
                                                    'b_1ca354: {
                                                        'b_1ca344: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                                            if c.g[S1][0] != c.g[V0][0] {
                                                                c.g[A1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                break 'b_1ca344;
                                                            } else {
                                                                'b_1ca320: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] =
                                                                        c.lw(c.g[A0][0] as u32);
                                                                    if c.g[V0][0] == 0u64 {
                                                                        c.g[A3][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        break 'b_1ca320;
                                                                    } else {
                                                                        c.cycles += 4;
                                                                        c.g[RA][0] = sx(0x001ca318);
                                                                        c.g[A0][0] = c.g[S0][0];
                                                                        Callee { addr: 0x001c9b98, f: ee::s03::cFEPopupConfirm_OpenConfirmPopup }.call(c, 0x001ca318)?;
                                                                        let t = true;
                                                                        c.g[A2][0] = 0x1;
                                                                        if t {
                                                                            break 'b_1ca354;
                                                                        } else {
                                                                            break 'b_1ca320;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 9;
                                                                c.g[A2][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_add(0xf));
                                                                c.g[A1][0] = c.g[S0][0];
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A3][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                c.g[A0][0] = c
                                                                    .lh((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x88));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x8c));
                                                                let t = c.g[V1][0] as u32;
                                                                c.g[RA][0] = sx(0x001ca33c);
                                                                c.g[A0][0] = sx((c.g[A3][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[A0][0] as u32,
                                                                    ));
                                                                call_ptr(c, t, 0x001ca33c)?;
                                                                let t = true;
                                                                c.g[A2][0] = 0x1;
                                                                if t {
                                                                    break 'b_1ca354;
                                                                } else {
                                                                    break 'b_1ca344;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0xc));
                                                        c.g[V0][0] =
                                                            ((c.g[A1][0] as i64) < 5i64) as u64;
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[V1][0] =
                                                            sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x4));
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x001ca2f8);
                                                            }
                                                            continue 'l_1ca2f8;
                                                        } else {
                                                            break 'b_1ca354;
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    if c.g[A2][0] != 0u64 {
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        break 'b_1ca478;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x88));
                                                        if c.g[S1][0] != c.g[V0][0] {
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x20),
                                                            );
                                                            break 'b_1ca478;
                                                        } else {
                                                            c.cycles += 25;
                                                            c.g[A1][0] = 0x460000;
                                                            c.g[A0][0] = 0x444;
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_sub(0x2728));
                                                            c.g[A2][0] = 0x100;
                                                            c.g[RA][0] = sx(0x001ca380);
                                                            c.g[A3][0] = 0u64;
                                                            call_ptr(c, 0x00317d70, 0x001ca380)?;
                                                            c.g[A1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[A2][0] = c.g[S0][0];
                                                            c.g[T0][0] = 0xf;
                                                            c.g[A3][0] = 0u64;
                                                            c.g[RA][0] = sx(0x001ca398);
                                                            c.g[A0][0] = c.g[V0][0];
                                                            call_ptr(c, 0x001cb030, 0x001ca398)?;
                                                            c.g[A1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x328));
                                                            c.g[A0][0] = c.g[V0][0];
                                                            c.g[RA][0] = sx(0x001ca3a8);
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x2a8),
                                                                c.g[V0][0],
                                                            );
                                                            call_ptr(c, 0x001cd088, 0x001ca3a8)?;
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[A1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x2a8));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0xd0));
                                                            c.g[A0][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[RA][0] = sx(0x001ca3c0);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x18));
                                                            call_ptr(c, 0x0039f290, 0x001ca3c0)?;
                                                            c.g[A2][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x84));
                                                            if c.g[A2][0] == 0u64 {
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                break 'b_1ca478;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.g[A0][0] = c.g[S0][0];
                                                                c.g[RA][0] = sx(0x001ca3d8);
                                                                c.g[A1][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x2b4));
                                                                Callee { addr: 0x001c8258, f: ee::s03::cFEPopupConfirm_GetWidgetText }.call(c, 0x001ca3d8)?;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                if t {
                                                                    break 'b_1ca478;
                                                                } else {
                                                                    break 'b_1ca3e0;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x1;
                                        let t = true;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xb8), c.g[V0][0]);
                                        if t {
                                            break 'b_1ca474;
                                        } else {
                                            break 'b_1ca3ec;
                                        }
                                    }
                                    0x001ca3ec => {
                                        break 'b_1ca3ec;
                                    }
                                    0x001ca41c => {
                                        break 'b_1ca41c;
                                    }
                                    0x001ca458 => {
                                        break 'b_1ca458;
                                    }
                                    0x001ca474 => {
                                        break 'b_1ca474;
                                    }
                                    _ => {
                                        c.jump_out(t, 0x001ca298, 0x001ca488);
                                        return Ok(());
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x310));
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.g[S0][0];
                            if t {
                                break 'b_1ca474;
                            } else {
                                c.cycles += 9;
                                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                                c.g[A2][0] = 0x14;
                                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x88));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001ca414);
                                c.g[A0][0] =
                                    sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001ca414)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_1ca478;
                                } else {
                                    break 'b_1ca41c;
                                }
                            }
                        }
                        c.cycles += 7;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2a8));
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b0));
                        c.g[RA][0] = sx(0x001ca42c);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x74));
                        call_ptr(c, 0x003184e8, 0x001ca42c)?;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                        if c.g[A1][0] == 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            break 'b_1ca478;
                        } else {
                            c.cycles += 4;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2b0));
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_sub(0x8));
                            if c.g[V0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                break 'b_1ca478;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001ca450);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x001ca6a0,
                                    f: ee::s03::cFEPopupConfirm_CheckAndFixTextEntry,
                                }
                                .call(c, 0x001ca450)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_1ca478;
                                } else {
                                    break 'b_1ca458;
                                }
                            }
                        }
                    }
                }
                c.cycles += 7;
                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                c.g[A1][0] = c.g[S1][0];
                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001ca474);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001ca474)?;
                break 'b_1ca474;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1ca478;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEPopupConfirm_OnChildEvent` (0x001ca598): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_popup_confirm_on_child_event(c: &mut Cpu) {
    if c.pc != 0x001ca598 {
        return ee::s03::f_001ca598__generated(c);
    }
    guest(c, |c| 'l_1ca598: loop {
        'b_1ca690: {
            'b_1ca68c: {
                c.cycles += 8;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0xf));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A3][0] = c.g[A1][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[V0][0] = (c.g[V1][0] < 0x8u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[S0][0] = c.g[A0][0];
                if t {
                    break 'b_1ca68c;
                } else {
                    'b_1ca648: {
                        'b_1ca634: {
                            'b_1ca630: {
                                'b_1ca620: {
                                    'b_1ca600: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x460000;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6180));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x001ca598 => {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x001ca598);
                                                }
                                                continue 'l_1ca598;
                                            }
                                            0x001ca5d4 => {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x2a8));
                                                if c.g[A3][0] != c.g[V0][0] {
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x10),
                                                    );
                                                    break 'b_1ca690;
                                                } else {
                                                    c.cycles += 8;
                                                    c.g[A1][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x2b0)
                                                        );
                                                    c.g[RA][0] = sx(0x001ca5ec);
                                                    c.g[A0][0] = c.g[A3][0];
                                                    call_ptr(c, 0x001cb418, 0x001ca5ec)?;
                                                    c.g[A1][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x2b4)
                                                        );
                                                    c.g[RA][0] = sx(0x001ca5f8);
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x2a8)
                                                        );
                                                    call_ptr(c, 0x001ccf00, 0x001ca5f8)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x10),
                                                    );
                                                    if t {
                                                        break 'b_1ca690;
                                                    } else {
                                                        break 'b_1ca600;
                                                    }
                                                }
                                            }
                                            0x001ca600 => {
                                                break 'b_1ca600;
                                            }
                                            0x001ca620 => {
                                                break 'b_1ca620;
                                            }
                                            0x001ca630 => {
                                                break 'b_1ca630;
                                            }
                                            0x001ca648 => {
                                                break 'b_1ca648;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x001ca598, 0x001ca69c);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 8;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                    c.g[A1][0] = 0x1;
                                    c.g[RA][0] = sx(0x001ca610);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f190, 0x001ca610)?;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x324));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf));
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x320), c.g[V1][0]);
                                    if t {
                                        break 'b_1ca68c;
                                    } else {
                                        break 'b_1ca620;
                                    }
                                }
                                c.cycles += 4;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                c.g[V0][0] = 0xffffffffffffffff;
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x320), c.g[V0][0]);
                                if t {
                                    break 'b_1ca634;
                                } else {
                                    break 'b_1ca630;
                                }
                            }
                            c.cycles += 1;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                            break 'b_1ca634;
                        }
                        c.cycles += 5;
                        c.g[A1][0] = 0x1;
                        c.g[RA][0] = sx(0x001ca640);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0039f190, 0x001ca640)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        if t {
                            break 'b_1ca690;
                        } else {
                            break 'b_1ca648;
                        }
                    }
                    'b_1ca654: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2a8));
                        if c.g[A3][0] == c.g[V0][0] {
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x2a8), 0u64);
                            break 'b_1ca654;
                        } else {
                            break 'b_1ca654;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ac));
                    if c.g[A3][0] != c.g[V0][0] {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        break 'b_1ca690;
                    } else {
                        c.cycles += 4;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x320));
                        c.g[V0][0] = 0xffffffffffffffff;
                        let t = c.g[A2][0] == c.g[V0][0];
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x2ac), 0u64);
                        if t {
                            break 'b_1ca68c;
                        } else {
                            c.cycles += 7;
                            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x88));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001ca68c);
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001ca68c)?;
                            break 'b_1ca68c;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1ca690;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `cUIStateBuyPopup_SetRewardItemParams` (0x001caa18): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_ui_state_buy_popup_set_reward_item_params(c: &mut Cpu) {
    if c.pc != 0x001caa18 {
        return ee::s04::f_001caa18__generated(c);
    }
    guest(c, |c| {
        'b_1caaf4: {
            c.cycles += 26;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.g[V0][0] = 0x2;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S3][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = 0x460000;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[A1][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S1][0] = c.g[A3][0];
            c.g[S2][0] = c.g[T0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x61f8));
            c.sw((c.g[S3][0] as u32).wrapping_add(0x10), c.g[A2][0]);
            c.g[RA][0] = sx(0x001caa58);
            c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            call_ptr(c, 0x00317670, 0x001caa58)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3));
            c.g[V1][0] = 0x3;
            c.g[A0][0] = 0xffffffffffffffff;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x20), c.g[V0][0]);
            c.sw((c.g[S3][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
            c.g[V0][0] = (c.g[S0][0] < 0x6u64) as u64;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.sw(c.g[S3][0] as u32, c.g[S1][0]);
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x4), c.g[S2][0]);
            if t {
                break 'b_1caaf4;
            } else {
                'b_1caae0: {
                    'b_1caad8: {
                        'b_1caacc: {
                            'b_1caac0: {
                                'b_1caab4: {
                                    'b_1caaa8: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x460000;
                                        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x62a0));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x001caa9c => {
                                                c.cycles += 3;
                                                c.g[A0][0] = 0x460000;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x6210));
                                                if t {
                                                    break 'b_1caae0;
                                                } else {
                                                    break 'b_1caaa8;
                                                }
                                            }
                                            0x001caaa8 => {
                                                break 'b_1caaa8;
                                            }
                                            0x001caab4 => {
                                                break 'b_1caab4;
                                            }
                                            0x001caac0 => {
                                                break 'b_1caac0;
                                            }
                                            0x001caacc => {
                                                break 'b_1caacc;
                                            }
                                            0x001caad8 => {
                                                break 'b_1caad8;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x001caa18, 0x001cab10);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x460000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6228));
                                    if t {
                                        break 'b_1caae0;
                                    } else {
                                        break 'b_1caab4;
                                    }
                                }
                                c.cycles += 3;
                                c.g[A0][0] = 0x460000;
                                let t = true;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6240));
                                if t {
                                    break 'b_1caae0;
                                } else {
                                    break 'b_1caac0;
                                }
                            }
                            c.cycles += 3;
                            c.g[A0][0] = 0x460000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6258));
                            if t {
                                break 'b_1caae0;
                            } else {
                                break 'b_1caacc;
                            }
                        }
                        c.cycles += 3;
                        c.g[A0][0] = 0x460000;
                        let t = true;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6270));
                        if t {
                            break 'b_1caae0;
                        } else {
                            break 'b_1caad8;
                        }
                    }
                    c.cycles += 2;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6288));
                    break 'b_1caae0;
                }
                c.cycles += 5;
                c.g[RA][0] = sx(0x001caae8);
                call_ptr(c, 0x00317670, 0x001caae8)?;
                c.g[V1][0] = 0x3;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                c.sw((c.g[S3][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1caaf4;
            }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `void cKeyboardPopup_PressKey(cKeyboardPopup* self, int flag)` (0x1cc1e0): the
/// on-screen keyboard presses the key under the cursor (`self->key`, +0x140).
/// A pending accept (+0x54) is first completed by pressing key 0x46 (recursion).
/// With `flag == 0` and the key enabled it runs the key's action: backspace
/// (0x1b), caps (0x2a), shift (0x38), clear (0x46), left (0x4c), right (0x53),
/// accept (0x37), quick messages (4..13) and the default typed character. It
/// then refreshes the entry text, the accept/clear key states (min length) and
/// re-highlights the clear key (colour through the key widget's vtable).
pub fn c_keyboard_popup_press_key(c: &mut Cpu) {
    if c.pc != 0x001cc1e0 {
        return ee::s04::f_001cc1e0__generated(c);
    }
    guest(c, |c| {
        'b_1cc244: {
            'b_1cc240: {
                c.cycles += 12;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S1][0] = c.g[A0][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S2][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
                let t = c.g[V0][0] == 0u64;
                c.g[S4][0] = 0u64;
                if t {
                    break 'b_1cc240;
                } else {
                    c.cycles += 4;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
                    c.g[V0][0] = 0x37;
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[V0][0] = 0x46;
                    if t {
                        break 'b_1cc240;
                    } else {
                        c.cycles += 2;
                        let t = c.g[S0][0] == c.g[V0][0];
                        c.g[S3][0] = 0u64;
                        if t {
                            break 'b_1cc244;
                        } else {
                            c.cycles += 6;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x54), 0u64);
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x140), c.g[V0][0]);
                            c.g[RA][0] = sx(0x001cc238);
                            c.g[A1][0] = 0u64;
                            Callee {
                                addr: 0x001cc1e0,
                                f: ee::s04::cKeyboardPopup_PressKey,
                            }
                            .call(c, 0x001cc238)?;
                            c.g[S4][0] = 0x1;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x140), c.g[S0][0]);
                            break 'b_1cc240;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[S3][0] = 0u64;
            break 'b_1cc244;
        }
        'b_1cc500: {
            'b_1cc4fc: {
                c.cycles += 2;
                let t = c.g[S2][0] != 0u64;
                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                if t {
                    break 'b_1cc4fc;
                } else {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001cc25c);
                    c.g[A2][0] = 0u64;
                    Callee {
                        addr: 0x001ce560,
                        f: ee::s04::cKeyboardPopup_IsKeyEnabled,
                    }
                    .call(c, 0x001cc25c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
                    if t {
                        break 'b_1cc500;
                    } else {
                        'b_1cc4f0: {
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[V1][0] < 0x54u64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x460000;
                            if t {
                                break 'b_1cc4f0;
                            } else {
                                'b_1cc4d4: {
                                    'b_1cc4b8: {
                                        'b_1cc498: {
                                            'b_1cc46c: {
                                                'b_1cc438: {
                                                    'b_1cc3e8: {
                                                        'b_1cc3b0: {
                                                            'b_1cc388: {
                                                                'b_1cc368: {
                                                                    'b_1cc348: {
                                                                        c.cycles += 6;
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x63e0,
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[A0][0] =
                                                                            c.lw(c.g[V1][0] as u32);
                                                                        let t = c.g[A0][0] as u32;
                                                                        match t {
                                                                            0x001cc288 => {
                                                                                'b_1cc314: {
                                                                                    c.cycles += 7;
                                                                                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                                                                                    c.g[A0][0] =
                                                                                        c.g[V0][0];
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001cc298,
                                                                                    );
                                                                                    c.g[S0][0] =
                                                                                        c.g[V0][0];
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x00416810,
                                                                                        0x001cc298,
                                                                                    )?;
                                                                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x134));
                                                                                    if c.g[V1][0]
                                                                                        == c.g[V0]
                                                                                            [0]
                                                                                    {
                                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x138));
                                                                                        break 'b_1cc314;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        if c.g[V1]
                                                                                            [0]
                                                                                            == 0u64
                                                                                        {
                                                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
                                                                                            break 'b_1cc500;
                                                                                        } else {
                                                                                            'b_1cc2b8: {
                                                                                                c.cycles += 2;
                                                                                                let t = (c.g[V1][0] as i64) <= 0;
                                                                                                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_1cc2b8;
                                                                                                } else {
                                                                                                    c.cycles += 1;
                                                                                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x134), c.g[V0][0]);
                                                                                                    break 'b_1cc2b8;
                                                                                                }
                                                                                            }
                                                                                            'b_1cc2ec: {
                                                                                                c.cycles += 11;
                                                                                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x134));
                                                                                                c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x75));
                                                                                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
                                                                                                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                                                                                c.g[RA][0] = sx(0x001cc2d0);
                                                                                                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                                                                                call_ptr(c, 0x004166f4, 0x001cc2d0)?;
                                                                                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x134));
                                                                                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x138));
                                                                                                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                                                                                let t = c.g[V1][0] != c.g[V0][0];

                                                                                                if t
                                                                                                {
                                                                                                    break 'b_1cc2ec;
                                                                                                } else {
                                                                                                    c.cycles += 2;
                                                                                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x138), c.g[A0][0]);
                                                                                                    c.g[S3][0] = 0x1;
                                                                                                    break 'b_1cc2ec;
                                                                                                }
                                                                                            }
                                                                                            c.cycles += 6;
                                                                                            c.g[RA][0] = sx(0x001cc2f4);
                                                                                            c.g[A0][0] = c.g[S0][0];
                                                                                            call_ptr(c, 0x00416810, 0x001cc2f4)?;
                                                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x13c));
                                                                                            c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                                                                                            if c.g[V0][0] == 0u64 {
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
break 'b_1cc500;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x001cc30c);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x00416810, 0x001cc30c)?;
let t = true;
c.sw((c.g[S1][0] as u32).wrapping_add(0x13c), c.g[V0][0]);
if t {
break 'b_1cc4fc;
} else {
break 'b_1cc314;
}
}
                                                                                        }
                                                                                    }
                                                                                }
                                                                                'b_1cc324: {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[V1]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_1cc324;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t =
                                                                                            true;
                                                                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x138), 0u64);
                                                                                        if t {
                                                                                            break 'b_1cc4fc;
                                                                                        } else {
                                                                                            break 'b_1cc324;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                'b_1cc330: {
                                                                                    c.cycles += 2;
                                                                                    let t = (c.g[V1]
                                                                                        [0]
                                                                                        as i64)
                                                                                        <= 0;
                                                                                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                                                                    if t {
                                                                                        break 'b_1cc330;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            1;
                                                                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x134), c.g[V0][0]);
                                                                                        break 'b_1cc330;
                                                                                    }
                                                                                }
                                                                                c.cycles += 6;
                                                                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x134));
                                                                                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                c.sb(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                    0u64,
                                                                                );
                                                                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x134));
                                                                                let t = true;
                                                                                c.sw((c.g[S1][0] as u32).wrapping_add(0x13c), c.g[V1][0]);
                                                                                if t {
                                                                                    break 'b_1cc4fc;
                                                                                } else {
                                                                                    break 'b_1cc348;
                                                                                }
                                                                            }
                                                                            0x001cc348 => {
                                                                                break 'b_1cc348;
                                                                            }
                                                                            0x001cc368 => {
                                                                                break 'b_1cc368;
                                                                            }
                                                                            0x001cc388 => {
                                                                                break 'b_1cc388;
                                                                            }
                                                                            0x001cc3b0 => {
                                                                                break 'b_1cc3b0;
                                                                            }
                                                                            0x001cc3e8 => {
                                                                                break 'b_1cc3e8;
                                                                            }
                                                                            0x001cc438 => {
                                                                                break 'b_1cc438;
                                                                            }
                                                                            0x001cc46c => {
                                                                                break 'b_1cc46c;
                                                                            }
                                                                            0x001cc498 => {
                                                                                break 'b_1cc498;
                                                                            }
                                                                            0x001cc4d4 => {
                                                                                break 'b_1cc4d4;
                                                                            }
                                                                            0x001cc4f0 => {
                                                                                break 'b_1cc4f0;
                                                                            }
                                                                            0x001cc4fc => {
                                                                                break 'b_1cc4fc;
                                                                            }
                                                                            _ => {
                                                                                c.jump_out(
                                                                                    t, 0x001cc1e0,
                                                                                    0x001cc620,
                                                                                );
                                                                                return Ok(());
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 8;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x68));
                                                                    c.g[A0][0] = c.g[S1][0];
                                                                    c.g[S0][0] = sx((c.g[S1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x74));
                                                                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                                    c.g[RA][0] = sx(0x001cc360);
                                                                    c.sw(
                                                                        (c.g[S1][0] as u32)
                                                                            .wrapping_add(0x68),
                                                                        c.g[V0][0],
                                                                    );
                                                                    Callee { addr: 0x001cd708, f: ee::s04::cKeyboardPopup_RefreshKeys }.call(c, 0x001cc360)?;
                                                                    let t = true;
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x140));
                                                                    if t {
                                                                        break 'b_1cc500;
                                                                    } else {
                                                                        break 'b_1cc368;
                                                                    }
                                                                }
                                                                c.cycles += 8;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x6c));
                                                                c.g[A0][0] = c.g[S1][0];
                                                                c.g[S0][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x74));
                                                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                                c.g[RA][0] = sx(0x001cc380);
                                                                c.sw(
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0x6c),
                                                                    c.g[V0][0],
                                                                );
                                                                Callee { addr: 0x001cd708, f: ee::s04::cKeyboardPopup_RefreshKeys }.call(c, 0x001cc380)?;
                                                                let t = true;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x140));
                                                                if t {
                                                                    break 'b_1cc500;
                                                                } else {
                                                                    break 'b_1cc388;
                                                                }
                                                            }
                                                            c.cycles += 10;
                                                            c.g[S0][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x74));
                                                            c.g[A1][0] = 0u64;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x001cc39c);
                                                            c.g[A2][0] = 0x40;
                                                            call_ptr(c, 0x00416210, 0x001cc39c)?;
                                                            c.sb(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x74),
                                                                0u64,
                                                            );
                                                            c.sw(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x134),
                                                                0u64,
                                                            );
                                                            c.sw(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x13c),
                                                                0u64,
                                                            );
                                                            let t = true;
                                                            c.sw(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x138),
                                                                0u64,
                                                            );
                                                            if t {
                                                                break 'b_1cc4fc;
                                                            } else {
                                                                break 'b_1cc3b0;
                                                            }
                                                        }
                                                        'b_1cc3d4: {
                                                            c.cycles += 4;
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x138));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x134));
                                                            let t = c.g[V1][0] != c.g[V0][0];

                                                            if t {
                                                                break 'b_1cc3d4;
                                                            } else {
                                                                'b_1cc3d0: {
                                                                    c.cycles += 2;
                                                                    let t = c.g[V1][0] == 0u64;
                                                                    c.g[V0][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_sub(0x1));
                                                                    if t {
                                                                        break 'b_1cc3d0;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.sw(
                                                                            (c.g[S1][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x138,
                                                                                ),
                                                                            c.g[V0][0],
                                                                        );
                                                                        c.g[V1][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x134));
                                                                        break 'b_1cc3d0;
                                                                    }
                                                                }
                                                                c.cycles += 1;
                                                                c.g[S3][0] = 0x1;
                                                                break 'b_1cc3d4;
                                                            }
                                                        }
                                                        c.cycles += 2;
                                                        let t = (c.g[V1][0] as i64) <= 0;
                                                        c.g[S0][0] =
                                                            sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x74));
                                                        if t {
                                                            break 'b_1cc4fc;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x1));
                                                            let t = true;
                                                            c.sw(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x134),
                                                                c.g[V0][0],
                                                            );
                                                            if t {
                                                                break 'b_1cc4fc;
                                                            } else {
                                                                break 'b_1cc3e8;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 7;
                                                    c.g[V0][0] =
                                                        sx((c.g[S1][0] as u32).wrapping_add(0x74));
                                                    c.g[A0][0] = c.g[V0][0];
                                                    c.g[RA][0] = sx(0x001cc3f8);
                                                    c.g[S0][0] = c.g[V0][0];
                                                    call_ptr(c, 0x00416810, 0x001cc3f8)?;
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x134)
                                                        );
                                                    if c.g[V1][0] == c.g[V0][0] {
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x140));
                                                        break 'b_1cc500;
                                                    } else {
                                                        'b_1cc41c: {
                                                            c.cycles += 5;
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x60));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x1));
                                                            c.g[V0][0] = ((c.g[V1][0] as i64)
                                                                < (c.g[V0][0] as i64))
                                                                as u64;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x1));
                                                            if t {
                                                                break 'b_1cc41c;
                                                            } else {
                                                                c.cycles += 1;
                                                                c.sw(
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0x134),
                                                                    c.g[V0][0],
                                                                );
                                                                break 'b_1cc41c;
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x13c));
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x134));
                                                        if c.g[V1][0] != c.g[V0][0] {
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x140));
                                                            break 'b_1cc500;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x1));
                                                            let t = true;
                                                            c.sw(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x13c),
                                                                c.g[V0][0],
                                                            );
                                                            if t {
                                                                break 'b_1cc4fc;
                                                            } else {
                                                                break 'b_1cc438;
                                                            }
                                                        }
                                                    }
                                                }
                                                'b_1cc450: {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x420)
                                                        );
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A1][0] = 0x1;
                                                    if t {
                                                        break 'b_1cc450;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.g[RA][0] = sx(0x001cc450);
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x18));
                                                        call_ptr(c, 0x0039f190, 0x001cc450)?;
                                                        break 'b_1cc450;
                                                    }
                                                }
                                                c.cycles += 7;
                                                c.g[V0][0] = 0x1;
                                                c.g[A3][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                c.sw(
                                                    (c.g[S1][0] as u32).wrapping_add(0x5c),
                                                    c.g[V0][0],
                                                );
                                                c.g[A1][0] = 0u64;
                                                c.g[A2][0] = 0xd;
                                                let t = true;
                                                c.g[S0][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x74));
                                                if t {
                                                    break 'b_1cc4b8;
                                                } else {
                                                    break 'b_1cc46c;
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[A1][0] = 0x4b;
                                            c.g[A2][0] = 0u64;
                                            c.g[RA][0] = sx(0x001cc480);
                                            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                                            Callee {
                                                addr: 0x001ce560,
                                                f: ee::s04::cKeyboardPopup_IsKeyEnabled,
                                            }
                                            .call(c, 0x001cc480)?;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
                                                break 'b_1cc500;
                                            } else {
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x001cc490);
                                                c.g[A0][0] = c.g[S1][0];
                                                Callee {
                                                    addr: 0x001cc7c8,
                                                    f: ee::s04::cKeyboardPopup_TypeCurrentKey,
                                                }
                                                .call(c, 0x001cc490)?;
                                                let t = true;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
                                                if t {
                                                    break 'b_1cc500;
                                                } else {
                                                    break 'b_1cc498;
                                                }
                                            }
                                        }
                                        c.cycles += 8;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                        c.g[A1][0] = 0x1;
                                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                                        c.g[RA][0] = sx(0x001cc4ac);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                        call_ptr(c, 0x0039f190, 0x001cc4ac)?;
                                        c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                        c.g[A1][0] = 0u64;
                                        c.g[A2][0] = 0xe;
                                        break 'b_1cc4b8;
                                    }
                                    c.cycles += 7;
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001cc4cc);
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001cc4cc)?;
                                    let t = true;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
                                    if t {
                                        break 'b_1cc500;
                                    } else {
                                        break 'b_1cc4d4;
                                    }
                                }
                                c.cycles += 7;
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
                                c.g[A0][0] = c.g[S1][0];
                                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                                c.g[RA][0] = sx(0x001cc4e8);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4));
                                Callee {
                                    addr: 0x001ce5e0,
                                    f: ee::s04::cKeyboardPopup_AppendQuickMessage,
                                }
                                .call(c, 0x001cc4e8)?;
                                let t = true;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
                                if t {
                                    break 'b_1cc500;
                                } else {
                                    break 'b_1cc4f0;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x001cc4f8);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x001cc7c8,
                            f: ee::s04::cKeyboardPopup_TypeCurrentKey,
                        }
                        .call(c, 0x001cc4f8)?;
                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                        break 'b_1cc4fc;
                    }
                }
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
            break 'b_1cc500;
        }
        'b_1cc528: {
            c.cycles += 3;
            c.g[V0][0] = 0x38;
            let t = c.g[V1][0] == c.g[V0][0];
            c.g[A1][0] = c.g[S3][0];
            if t {
                break 'b_1cc528;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                if t {
                    break 'b_1cc528;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001cc524);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x6c), c.g[V0][0]);
                    Callee {
                        addr: 0x001cd708,
                        f: ee::s04::cKeyboardPopup_RefreshKeys,
                    }
                    .call(c, 0x001cc524)?;
                    c.g[A1][0] = c.g[S3][0];
                    break 'b_1cc528;
                }
            }
        }
        'b_1cc558: {
            c.cycles += 5;
            c.g[RA][0] = sx(0x001cc530);
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: 0x001cc848,
                f: ee::s04::cKeyboardPopup_RefreshEntryText,
            }
            .call(c, 0x001cc530)?;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x58));
            let t = c.g[V0][0] == 0u64;

            if t {
                break 'b_1cc558;
            } else {
                c.cycles += 7;
                c.g[RA][0] = sx(0x001cc544);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00416810, 0x001cc544)?;
                c.g[A2][0] = (0u64 < c.g[V0][0]) as u64;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0x4b;
                c.g[RA][0] = sx(0x001cc558);
                c.g[A3][0] = 0x2;
                Callee {
                    addr: 0x001ce3c8,
                    f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                }
                .call(c, 0x001cc558)?;
                break 'b_1cc558;
            }
        }
        'b_1cc598: {
            'b_1cc588: {
                c.cycles += 6;
                c.g[RA][0] = sx(0x001cc560);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00416810, 0x001cc560)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x37;
                if t {
                    break 'b_1cc588;
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001cc580);
                    c.g[A3][0] = 0x2;
                    Callee {
                        addr: 0x001ce3c8,
                        f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                    }
                    .call(c, 0x001cc580)?;
                    let t = true;

                    if t {
                        break 'b_1cc598;
                    } else {
                        break 'b_1cc588;
                    }
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 0x1;
            c.g[RA][0] = sx(0x001cc598);
            c.g[A3][0] = 0x2;
            Callee {
                addr: 0x001ce3c8,
                f: ee::s04::cKeyboardPopup_SetKeyEnabled,
            }
            .call(c, 0x001cc598)?;
            break 'b_1cc598;
        }
        'b_1cc604: {
            'b_1cc600: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001cc5a0);
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x001cd708,
                    f: ee::s04::cKeyboardPopup_RefreshKeys,
                }
                .call(c, 0x001cc5a0)?;
                let t = c.g[S4][0] == 0u64;
                c.g[A2][0] = c.g[SP][0];
                if t {
                    break 'b_1cc600;
                } else {
                    c.cycles += 11;
                    c.g[A1][0] = 0x46;
                    c.g[A3][0] = 0x1;
                    c.g[RA][0] = sx(0x001cc5b8);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x001cd0d8,
                        f: ee::s04::cKeyboardPopup_GetKeyWidgetName,
                    }
                    .call(c, 0x001cc5b8)?;
                    c.g[RA][0] = sx(0x001cc5c0);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001cc5c0)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001cc5cc);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001cc5cc)?;
                    let t = c.g[V0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    if t {
                        break 'b_1cc604;
                    } else {
                        c.cycles += 11;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x58fc));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001cc600);
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        call_ptr(c, t, 0x001cc600)?;
                        break 'b_1cc600;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            break 'b_1cc604;
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `int cKeyboardPopup_GetKeyWidgetName(self, int key, char* out, int shifted)`
/// (0x1cd0d8): writes the UI object name of a key (a name prefix, then the key's
/// suffix: special keys by table, F-keys by `sprintf`, characters from the key
/// character table) into `out`; returns 0 for keys whose character is `*`.
pub fn c_keyboard_popup_get_key_widget_name(c: &mut Cpu) {
    if c.pc != 0x001cd0d8 {
        return ee::s04::f_001cd0d8__generated(c);
    }
    guest(c, |c| {
        'b_1cd13c: {
            'b_1cd110: {
                c.cycles += 6;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.g[T0][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                let t = c.g[A3][0] == 0u64;
                c.g[S0][0] = c.g[A2][0];
                if t {
                    break 'b_1cd110;
                } else {
                    c.cycles += 8;
                    c.g[V0][0] = 0x4a0000;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1368));
                    c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                    c.ldr(3, c.g[A2][0] as u32);
                    c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x7));
                    c.sdr(3, c.g[S0][0] as u32);
                    let t = true;
                    c.g[V0][0] = (c.g[T0][0] < 0x54u64) as u64;
                    if t {
                        break 'b_1cd13c;
                    } else {
                        break 'b_1cd110;
                    }
                }
            }
            c.cycles += 11;
            c.g[V0][0] = 0x460000;
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2ab0));
            c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
            c.ldr(3, c.g[A2][0] as u32);
            c.g[A0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x9));
            c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x7));
            c.sdr(3, c.g[S0][0] as u32);
            c.sb((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.sb((c.g[S0][0] as u32).wrapping_add(0x9), c.g[A1][0]);
            c.g[V0][0] = (c.g[T0][0] < 0x54u64) as u64;
            break 'b_1cd13c;
        }
        'b_1cd2e0: {
            'b_1cd2a4: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x460000;
                if t {
                    break 'b_1cd2a4;
                } else {
                    'b_1cd27c: {
                        'b_1cd264: {
                            'b_1cd24c: {
                                'b_1cd234: {
                                    'b_1cd21c: {
                                        'b_1cd204: {
                                            'b_1cd1ec: {
                                                'b_1cd1d4: {
                                                    'b_1cd1bc: {
                                                        'b_1cd1a4: {
                                                            'b_1cd18c: {
                                                                'b_1cd174: {
                                                                    c.cycles += 6;
                                                                    c.g[V1][0] = sx((c.g[T0][0]
                                                                        as u32)
                                                                        << 2);
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x65b0));
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[V0][0] as u32,
                                                                        ));
                                                                    c.g[A0][0] =
                                                                        c.lw(c.g[V1][0] as u32);
                                                                    let t = c.g[A0][0] as u32;
                                                                    match t {
                                                                        0x001cd15c => {
                                                                            c.cycles += 6;
                                                                            c.g[A1][0] = 0x4a0000;
                                                                            c.g[A0][0] = c.g[S0][0];
                                                                            c.g[RA][0] =
                                                                                sx(0x001cd16c);
                                                                            c.g[A1][0] =
                                                                                sx((c.g[A1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x12a8,
                                                                                    ));
                                                                            call_ptr(
                                                                                c, 0x004162d0,
                                                                                0x001cd16c,
                                                                            )?;
                                                                            let t = true;
                                                                            c.g[V0][0] = 0x1;
                                                                            if t {
                                                                                break 'b_1cd2e0;
                                                                            } else {
                                                                                break 'b_1cd174;
                                                                            }
                                                                        }
                                                                        0x001cd174 => {
                                                                            break 'b_1cd174;
                                                                        }
                                                                        0x001cd18c => {
                                                                            break 'b_1cd18c;
                                                                        }
                                                                        0x001cd1a4 => {
                                                                            break 'b_1cd1a4;
                                                                        }
                                                                        0x001cd1bc => {
                                                                            break 'b_1cd1bc;
                                                                        }
                                                                        0x001cd1d4 => {
                                                                            break 'b_1cd1d4;
                                                                        }
                                                                        0x001cd1ec => {
                                                                            break 'b_1cd1ec;
                                                                        }
                                                                        0x001cd204 => {
                                                                            break 'b_1cd204;
                                                                        }
                                                                        0x001cd21c => {
                                                                            break 'b_1cd21c;
                                                                        }
                                                                        0x001cd234 => {
                                                                            break 'b_1cd234;
                                                                        }
                                                                        0x001cd24c => {
                                                                            break 'b_1cd24c;
                                                                        }
                                                                        0x001cd264 => {
                                                                            break 'b_1cd264;
                                                                        }
                                                                        0x001cd27c => {
                                                                            break 'b_1cd27c;
                                                                        }
                                                                        0x001cd2a4 => {
                                                                            break 'b_1cd2a4;
                                                                        }
                                                                        _ => {
                                                                            c.jump_out(
                                                                                t, 0x001cd0d8,
                                                                                0x001cd2f0,
                                                                            );
                                                                            return Ok(());
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 6;
                                                                c.g[A1][0] = 0x4a0000;
                                                                c.g[A0][0] = c.g[S0][0];
                                                                c.g[RA][0] = sx(0x001cd184);
                                                                c.g[A1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1c80));
                                                                call_ptr(
                                                                    c, 0x004162d0, 0x001cd184,
                                                                )?;
                                                                let t = true;
                                                                c.g[V0][0] = 0x1;
                                                                if t {
                                                                    break 'b_1cd2e0;
                                                                } else {
                                                                    break 'b_1cd18c;
                                                                }
                                                            }
                                                            c.cycles += 6;
                                                            c.g[A1][0] = 0x4a0000;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x001cd19c);
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x1c88));
                                                            call_ptr(c, 0x004162d0, 0x001cd19c)?;
                                                            let t = true;
                                                            c.g[V0][0] = 0x1;
                                                            if t {
                                                                break 'b_1cd2e0;
                                                            } else {
                                                                break 'b_1cd1a4;
                                                            }
                                                        }
                                                        c.cycles += 6;
                                                        c.g[A1][0] = 0x4a0000;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x001cd1b4);
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x1c90));
                                                        call_ptr(c, 0x004162d0, 0x001cd1b4)?;
                                                        let t = true;
                                                        c.g[V0][0] = 0x1;
                                                        if t {
                                                            break 'b_1cd2e0;
                                                        } else {
                                                            break 'b_1cd1bc;
                                                        }
                                                    }
                                                    c.cycles += 6;
                                                    c.g[A1][0] = 0x4a0000;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.g[RA][0] = sx(0x001cd1cc);
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x1350));
                                                    call_ptr(c, 0x004162d0, 0x001cd1cc)?;
                                                    let t = true;
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_1cd2e0;
                                                    } else {
                                                        break 'b_1cd1d4;
                                                    }
                                                }
                                                c.cycles += 6;
                                                c.g[A1][0] = 0x460000;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[RA][0] = sx(0x001cd1e4);
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_sub(0x2aa0));
                                                call_ptr(c, 0x004162d0, 0x001cd1e4)?;
                                                let t = true;
                                                c.g[V0][0] = 0x1;
                                                if t {
                                                    break 'b_1cd2e0;
                                                } else {
                                                    break 'b_1cd1ec;
                                                }
                                            }
                                            c.cycles += 6;
                                            c.g[A1][0] = 0x4a0000;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x001cd1fc);
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x1c98));
                                            call_ptr(c, 0x004162d0, 0x001cd1fc)?;
                                            let t = true;
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_1cd2e0;
                                            } else {
                                                break 'b_1cd204;
                                            }
                                        }
                                        c.cycles += 6;
                                        c.g[A1][0] = 0x4a0000;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001cd214);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1ca0));
                                        call_ptr(c, 0x004162d0, 0x001cd214)?;
                                        let t = true;
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_1cd2e0;
                                        } else {
                                            break 'b_1cd21c;
                                        }
                                    }
                                    c.cycles += 6;
                                    c.g[A1][0] = 0x4a0000;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x001cd22c);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1ca8));
                                    call_ptr(c, 0x004162d0, 0x001cd22c)?;
                                    let t = true;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_1cd2e0;
                                    } else {
                                        break 'b_1cd234;
                                    }
                                }
                                c.cycles += 6;
                                c.g[A1][0] = 0x4a0000;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x001cd244);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1cb0));
                                call_ptr(c, 0x004162d0, 0x001cd244)?;
                                let t = true;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_1cd2e0;
                                } else {
                                    break 'b_1cd24c;
                                }
                            }
                            c.cycles += 6;
                            c.g[A1][0] = 0x4a0000;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001cd25c);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1cb8));
                            call_ptr(c, 0x004162d0, 0x001cd25c)?;
                            let t = true;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_1cd2e0;
                            } else {
                                break 'b_1cd264;
                            }
                        }
                        c.cycles += 6;
                        c.g[A1][0] = 0x4a0000;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x001cd274);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1cc0));
                        call_ptr(c, 0x004162d0, 0x001cd274)?;
                        let t = true;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_1cd2e0;
                        } else {
                            break 'b_1cd27c;
                        }
                    }
                    c.cycles += 10;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1cc8));
                    c.g[RA][0] = sx(0x001cd290);
                    c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x3));
                    call_ptr(c, 0x00417828, 0x001cd290)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001cd29c);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x004162d0, 0x001cd29c)?;
                    let t = true;
                    c.g[V0][0] = 0x1;
                    if t {
                        break 'b_1cd2e0;
                    } else {
                        break 'b_1cd2a4;
                    }
                }
            }
            'b_1cd2c8: {
                c.cycles += 7;
                c.g[V0][0] = 0x440000;
                c.g[A0][0] = 0x2a;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1390));
                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                let t = c.g[V1][0] != c.g[A0][0];
                c.g[A1][0] = c.lbu(c.g[V0][0] as u32);
                if t {
                    break 'b_1cd2c8;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_1cd2e0;
                    } else {
                        break 'b_1cd2c8;
                    }
                }
            }
            c.cycles += 6;
            c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
            c.g[A0][0] = c.g[S0][0];
            c.sb((c.g[SP][0] as u32).wrapping_add(0x11), 0u64);
            c.g[RA][0] = sx(0x001cd2dc);
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x004162d0, 0x001cd2dc)?;
            c.g[V0][0] = 0x1;
            break 'b_1cd2e0;
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void cKeyboardPopup_SetSpecialKeyLabel(self, cUIText* text, int key)` (0x1cd9a0):
/// colours the key's text (enabled or disabled colour), then sets the label of
/// the special keys (backspace, caps, shift, clear, left, right, accept...) from
/// a text id; the digit keys 4..13 are shown/hidden by the caps state and get
/// their number as an ASCII string.
pub fn c_keyboard_popup_set_special_key_label(c: &mut Cpu) {
    if c.pc != 0x001cd9a0 {
        return ee::s04::f_001cd9a0__generated(c);
    }
    guest(c, |c| {
        'b_1cdb84: {
            'b_1cdb80: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = c.g[A1][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                let t = c.g[S0][0] == 0u64;
                c.g[S2][0] = c.g[A2][0];
                if t {
                    break 'b_1cdb80;
                } else {
                    'b_1cda14: {
                        'b_1cd9f8: {
                            c.cycles += 5;
                            c.g[A1][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001cd9d0);
                            c.g[A2][0] = 0u64;
                            Callee {
                                addr: 0x001ce560,
                                f: ee::s04::cKeyboardPopup_IsKeyEnabled,
                            }
                            .call(c, 0x001cd9d0)?;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            if t {
                                break 'b_1cd9f8;
                            } else {
                                c.cycles += 8;
                                c.g[A1][0] = 0x4c0000;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x67b8));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001cd9f0);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001cd9f0)?;
                                let t = true;
                                c.g[V0][0] = (c.g[S2][0] < 0x54u64) as u64;
                                if t {
                                    break 'b_1cda14;
                                } else {
                                    break 'b_1cd9f8;
                                }
                            }
                        }
                        c.cycles += 7;
                        c.g[A1][0] = 0x4c0000;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x67c8));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001cda10);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001cda10)?;
                        c.g[V0][0] = (c.g[S2][0] < 0x54u64) as u64;
                        break 'b_1cda14;
                    }
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        break 'b_1cdb80;
                    } else {
                        'b_1cdb20: {
                            'b_1cdb04: {
                                'b_1cdabc: {
                                    'b_1cdab0: {
                                        'b_1cdaa4: {
                                            'b_1cda98: {
                                                'b_1cda8c: {
                                                    'b_1cda54: {
                                                        'b_1cda4c: {
                                                            'b_1cda40: {
                                                                c.cycles += 6;
                                                                c.g[V1][0] =
                                                                    sx((c.g[S2][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x67f0));
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[V0][0] as u32,
                                                                    ));
                                                                c.g[A0][0] =
                                                                    c.lw(c.g[V1][0] as u32);
                                                                let t = c.g[A0][0] as u32;
                                                                match t {
                                                                    0x001cda34 => {
                                                                        c.cycles += 3;
                                                                        c.g[A0][0] = 0x460000;
                                                                        let t = true;
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x6720,
                                                                                ));
                                                                        if t {
                                                                            break 'b_1cdb04;
                                                                        } else {
                                                                            break 'b_1cda40;
                                                                        }
                                                                    }
                                                                    0x001cda40 => {
                                                                        break 'b_1cda40;
                                                                    }
                                                                    0x001cda4c => {
                                                                        break 'b_1cda4c;
                                                                    }
                                                                    0x001cda8c => {
                                                                        break 'b_1cda8c;
                                                                    }
                                                                    0x001cda98 => {
                                                                        break 'b_1cda98;
                                                                    }
                                                                    0x001cdaa4 => {
                                                                        break 'b_1cdaa4;
                                                                    }
                                                                    0x001cdab0 => {
                                                                        break 'b_1cdab0;
                                                                    }
                                                                    0x001cdabc => {
                                                                        break 'b_1cdabc;
                                                                    }
                                                                    0x001cdb20 => {
                                                                        break 'b_1cdb20;
                                                                    }
                                                                    0x001cdb80 => {
                                                                        break 'b_1cdb80;
                                                                    }
                                                                    _ => {
                                                                        c.jump_out(
                                                                            t, 0x001cd9a0,
                                                                            0x001cdb98,
                                                                        );
                                                                        return Ok(());
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 3;
                                                            c.g[A0][0] = 0x460000;
                                                            let t = true;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x6740));
                                                            if t {
                                                                break 'b_1cda54;
                                                            } else {
                                                                break 'b_1cda4c;
                                                            }
                                                        }
                                                        c.cycles += 2;
                                                        c.g[A0][0] = 0x460000;
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x6760));
                                                        break 'b_1cda54;
                                                    }
                                                    c.cycles += 14;
                                                    c.g[RA][0] = sx(0x001cda5c);
                                                    call_ptr(c, 0x00317670, 0x001cda5c)?;
                                                    c.g[A1][0] = c.g[V0][0];
                                                    c.g[RA][0] = sx(0x001cda68);
                                                    c.g[A0][0] = c.g[S0][0];
                                                    call_ptr(c, 0x003a1000, 0x001cda68)?;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A1][0] = 0x4c0000;
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x67c8));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                                    c.g[V0][0] = c
                                                        .lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[RA][0] = sx(0x001cda84);
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x001cda84)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x40),
                                                    );
                                                    if t {
                                                        break 'b_1cdb84;
                                                    } else {
                                                        break 'b_1cda8c;
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[A0][0] = 0x460000;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x6778));
                                                if t {
                                                    break 'b_1cdb04;
                                                } else {
                                                    break 'b_1cda98;
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[A0][0] = 0x460000;
                                            let t = true;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x6790));
                                            if t {
                                                break 'b_1cdb04;
                                            } else {
                                                break 'b_1cdaa4;
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x460000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x67a8));
                                        if t {
                                            break 'b_1cdb04;
                                        } else {
                                            break 'b_1cdab0;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x460000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x67c0));
                                    if t {
                                        break 'b_1cdb04;
                                    } else {
                                        break 'b_1cdabc;
                                    }
                                }
                                'b_1cdb00: {
                                    'b_1cdae8: {
                                        c.cycles += 4;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x43c));
                                        c.g[V0][0] = 0x1;
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        if t {
                                            break 'b_1cdae8;
                                        } else {
                                            c.cycles += 7;
                                            c.g[A1][0] = 0u64;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x001cdae0);
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x001cdae0)?;
                                            let t = true;
                                            c.g[A0][0] = 0x460000;
                                            if t {
                                                break 'b_1cdb00;
                                            } else {
                                                break 'b_1cdae8;
                                            }
                                        }
                                    }
                                    c.cycles += 6;
                                    c.g[A1][0] = 0x1;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001cdafc);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001cdafc)?;
                                    c.g[A0][0] = 0x460000;
                                    break 'b_1cdb00;
                                }
                                c.cycles += 1;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x67d8));
                                break 'b_1cdb04;
                            }
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x001cdb0c);
                            call_ptr(c, 0x00317670, 0x001cdb0c)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001cdb18);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x001cdb18)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            if t {
                                break 'b_1cdb84;
                            } else {
                                break 'b_1cdb20;
                            }
                        }
                        'b_1cdb64: {
                            'b_1cdb4c: {
                                c.cycles += 4;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x43c));
                                c.g[V0][0] = 0x1;
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                if t {
                                    break 'b_1cdb4c;
                                } else {
                                    c.cycles += 7;
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001cdb44);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001cdb44)?;
                                    let t = true;
                                    c.g[A1][0] = 0x4a0000;
                                    if t {
                                        break 'b_1cdb64;
                                    } else {
                                        break 'b_1cdb4c;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001cdb60);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001cdb60)?;
                            c.g[A1][0] = 0x4a0000;
                            break 'b_1cdb64;
                        }
                        c.cycles += 7;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1cc8));
                        c.g[RA][0] = sx(0x001cdb74);
                        c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x3));
                        call_ptr(c, 0x00417828, 0x001cdb74)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x001cdb80);
                        c.g[A1][0] = c.g[SP][0];
                        call_ptr(c, 0x003a0c50, 0x001cdb80)?;
                        break 'b_1cdb80;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_1cdb84;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `int cKeyboardPopup_HandleInput(self, InputEvent* ev)` (0x1cdb98): the
/// keyboard popup's input handler. Moves the cursor key (left/right/up/down),
/// presses a key, opens the help popup, closes, toggles the two modifier flags,
/// or maps a typed (PC) key to a keyboard key; highlights the old and new key
/// widgets and plays the feedback sound. Always returns 1.
pub fn c_keyboard_popup_handle_input(c: &mut Cpu) {
    if c.pc != 0x001cdb98 {
        return ee::s04::f_001cdb98__generated(c);
    }
    guest(c, |c| 'l_1cdb98: loop {
        'b_1ce284: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S0][0] = c.g[A1][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S2][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x001cdbd4);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x001cdbd4)?;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_1ce284;
            } else {
                'b_1cdc44: {
                    c.cycles += 14;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = 0x1;
                    c.g[RA][0] = sx(0x001cdbf0);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001cd0d8,
                        f: ee::s04::cKeyboardPopup_GetKeyWidgetName,
                    }
                    .call(c, 0x001cdbf0)?;
                    c.g[S3][0] = 0u64;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x001cdc00);
                    c.g[S4][0] = 0x1;
                    call_ptr(c, 0x00317670, 0x001cdc00)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001cdc0c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001cdc0c)?;
                    if c.g[V0][0] == 0u64 {
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        break 'b_1cdc44;
                    } else {
                        c.cycles += 12;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x58f8));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001cdc40);
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        call_ptr(c, t, 0x001cdc40)?;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        break 'b_1cdc44;
                    }
                }
                'b_1ce1f4: {
                    'b_1cdc74: {
                        c.cycles += 6;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x98));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x9c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001cdc54);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001cdc54)?;
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            break 'b_1cdc74;
                        } else {
                            c.cycles += 6;
                            c.g[S3][0] = 0x1;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001cdc6c);
                            c.g[A1][0] = 0x1;
                            Callee {
                                addr: 0x001cb500,
                                f: ee::s04::cKeyboardPopup_MoveLeft,
                            }
                            .call(c, 0x001cdc6c)?;
                            let t = true;
                            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                            if t {
                                break 'b_1ce1f4;
                            } else {
                                break 'b_1cdc74;
                            }
                        }
                    }
                    'b_1cdca4: {
                        c.cycles += 6;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001cdc84);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001cdc84)?;
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            break 'b_1cdca4;
                        } else {
                            c.cycles += 6;
                            c.g[S3][0] = 0x1;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001cdc9c);
                            c.g[A1][0] = 0x1;
                            Callee {
                                addr: 0x001cb650,
                                f: ee::s04::cKeyboardPopup_MoveRight,
                            }
                            .call(c, 0x001cdc9c)?;
                            let t = true;
                            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                            if t {
                                break 'b_1ce1f4;
                            } else {
                                break 'b_1cdca4;
                            }
                        }
                    }
                    'b_1cdcd4: {
                        c.cycles += 6;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001cdcb4);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001cdcb4)?;
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            break 'b_1cdcd4;
                        } else {
                            c.cycles += 6;
                            c.g[S3][0] = 0x1;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001cdccc);
                            c.g[A1][0] = 0x1;
                            Callee {
                                addr: 0x001cb7f0,
                                f: ee::s04::cKeyboardPopup_MoveUp,
                            }
                            .call(c, 0x001cdccc)?;
                            let t = true;
                            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                            if t {
                                break 'b_1ce1f4;
                            } else {
                                break 'b_1cdcd4;
                            }
                        }
                    }
                    'b_1cdd04: {
                        c.cycles += 6;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x90));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001cdce4);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001cdce4)?;
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            break 'b_1cdd04;
                        } else {
                            c.cycles += 6;
                            c.g[S3][0] = 0x1;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001cdcfc);
                            c.g[A1][0] = 0x1;
                            Callee {
                                addr: 0x001cb9b0,
                                f: ee::s04::cKeyboardPopup_MoveDown,
                            }
                            .call(c, 0x001cdcfc)?;
                            let t = true;
                            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                            if t {
                                break 'b_1ce1f4;
                            } else {
                                break 'b_1cdd04;
                            }
                        }
                    }
                    'b_1cdd34: {
                        c.cycles += 6;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001cdd14);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001cdd14)?;
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            break 'b_1cdd34;
                        } else {
                            c.cycles += 6;
                            c.g[S3][0] = 0x1;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001cdd2c);
                            c.g[A1][0] = 0u64;
                            Callee {
                                addr: 0x001cc1e0,
                                f: ee::s04::cKeyboardPopup_PressKey,
                            }
                            .call(c, 0x001cdd2c)?;
                            let t = true;
                            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                            if t {
                                break 'b_1ce1f4;
                            } else {
                                break 'b_1cdd34;
                            }
                        }
                    }
                    'b_1ce1f0: {
                        'b_1cdea8: {
                            c.cycles += 6;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x001cdd44);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001cdd44)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                            if t {
                                break 'b_1cdea8;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                if c.g[V0][0] != 0u64 {
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                                    break 'b_1ce1f4;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                    let t = c.g[V1][0] != 0u64;
                                    c.g[S4][0] = 0x5;
                                    if t {
                                        break 'b_1ce1f0;
                                    } else {
                                        c.cycles += 80;
                                        c.g[A1][0] = 0x460000;
                                        c.g[A3][0] = 0u64;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a70));
                                        c.g[A0][0] = 0x140;
                                        c.g[A2][0] = 0x100;
                                        c.g[RA][0] = sx(0x001cdd84);
                                        c.g[S3][0] = 0x1;
                                        call_ptr(c, 0x00317d70, 0x001cdd84)?;
                                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[A2][0] = c.g[S2][0];
                                        c.g[RA][0] = sx(0x001cdd94);
                                        c.g[A0][0] = c.g[V0][0];
                                        Callee {
                                            addr: 0x001d9d48,
                                            f: ee::s04::cFEStateNetHelpPopup_ctor,
                                        }
                                        .call(c, 0x001cdd94)?;
                                        c.g[S1][0] = c.g[V0][0];
                                        c.g[A1][0] = 0x6;
                                        c.g[RA][0] = sx(0x001cdda4);
                                        c.g[A0][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x001da528,
                                            f: ee::s04::cFEStateNetHelpPopup_SetNumButtons,
                                        }
                                        .call(c, 0x001cdda4)?;
                                        c.g[A0][0] = 0x460000;
                                        c.g[RA][0] = sx(0x001cddb0);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6940));
                                        call_ptr(c, 0x00317670, 0x001cddb0)?;
                                        c.g[A0][0] = 0x460000;
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                                        c.g[RA][0] = sx(0x001cddc0);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6950));
                                        call_ptr(c, 0x00317670, 0x001cddc0)?;
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[A1][0] = 0u64;
                                        c.g[RA][0] = sx(0x001cddd0);
                                        c.g[A0][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x001da530,
                                            f: ee::s04::cFEStateNetHelpPopup_SetBodyText,
                                        }
                                        .call(c, 0x001cddd0)?;
                                        c.g[S0][0] = 0x460000;
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6968));
                                        c.g[RA][0] = sx(0x001cdde0);
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x00317670, 0x001cdde0)?;
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[A1][0] = 0u64;
                                        c.g[A3][0] = 0x4;
                                        c.g[RA][0] = sx(0x001cddf4);
                                        c.g[A0][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x001da510,
                                            f: ee::s04::cFEStateNetHelpPopup_SetButtonHelp,
                                        }
                                        .call(c, 0x001cddf4)?;
                                        c.g[RA][0] = sx(0x001cddfc);
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x00317670, 0x001cddfc)?;
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[A1][0] = 0x1;
                                        c.g[A3][0] = 0x3;
                                        c.g[RA][0] = sx(0x001cde10);
                                        c.g[A0][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x001da510,
                                            f: ee::s04::cFEStateNetHelpPopup_SetButtonHelp,
                                        }
                                        .call(c, 0x001cde10)?;
                                        c.g[A0][0] = 0x460000;
                                        c.g[RA][0] = sx(0x001cde1c);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6988));
                                        call_ptr(c, 0x00317670, 0x001cde1c)?;
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[A1][0] = 0x2;
                                        c.g[A3][0] = 0x5;
                                        c.g[RA][0] = sx(0x001cde30);
                                        c.g[A0][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x001da510,
                                            f: ee::s04::cFEStateNetHelpPopup_SetButtonHelp,
                                        }
                                        .call(c, 0x001cde30)?;
                                        c.g[A0][0] = 0x460000;
                                        c.g[RA][0] = sx(0x001cde3c);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x69a0));
                                        call_ptr(c, 0x00317670, 0x001cde3c)?;
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[A1][0] = 0x3;
                                        c.g[A3][0] = 0x6;
                                        c.g[RA][0] = sx(0x001cde50);
                                        c.g[A0][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x001da510,
                                            f: ee::s04::cFEStateNetHelpPopup_SetButtonHelp,
                                        }
                                        .call(c, 0x001cde50)?;
                                        c.g[A0][0] = 0x460000;
                                        c.g[RA][0] = sx(0x001cde5c);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x69b8));
                                        call_ptr(c, 0x00317670, 0x001cde5c)?;
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[A1][0] = 0x4;
                                        c.g[A3][0] = 0u64;
                                        c.g[RA][0] = sx(0x001cde70);
                                        c.g[A0][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x001da510,
                                            f: ee::s04::cFEStateNetHelpPopup_SetButtonHelp,
                                        }
                                        .call(c, 0x001cde70)?;
                                        c.g[A0][0] = 0x460000;
                                        c.g[RA][0] = sx(0x001cde7c);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x69d8));
                                        call_ptr(c, 0x00317670, 0x001cde7c)?;
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[A1][0] = 0x5;
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001cde90);
                                        c.g[A3][0] = 0x1;
                                        Callee {
                                            addr: 0x001da510,
                                            f: ee::s04::cFEStateNetHelpPopup_SetButtonHelp,
                                        }
                                        .call(c, 0x001cde90)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[A1][0] = c.g[S1][0];
                                        c.g[RA][0] = sx(0x001cdea0);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                        call_ptr(c, 0x0039f290, 0x001cdea0)?;
                                        let t = true;
                                        c.g[S4][0] = 0x5;
                                        if t {
                                            break 'b_1ce1f0;
                                        } else {
                                            break 'b_1cdea8;
                                        }
                                    }
                                }
                            }
                        }
                        'b_1cdf00: {
                            c.cycles += 7;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x30));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x001cdebc);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001cdebc)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = 0x1;
                            if t {
                                break 'b_1cdf00;
                            } else {
                                c.cycles += 15;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[S3][0] = 0x1;
                                c.g[S4][0] = 0x6;
                                c.g[RA][0] = sx(0x001cded8);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f190, 0x001cded8)?;
                                c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                                c.g[A1][0] = 0u64;
                                c.g[A2][0] = 0xe;
                                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001cdef8);
                                c.g[A0][0] =
                                    sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001cdef8)?;
                                let t = true;
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                                if t {
                                    break 'b_1ce1f4;
                                } else {
                                    break 'b_1cdf00;
                                }
                            }
                        }
                        'b_1cdf3c: {
                            c.cycles += 7;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc8));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xcc));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x001cdf14);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001cdf14)?;
                            if c.g[V0][0] == 0u64 {
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                break 'b_1cdf3c;
                            } else {
                                c.cycles += 8;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
                                c.g[S3][0] = 0x1;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                c.g[RA][0] = sx(0x001cdf34);
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x68), c.g[V0][0]);
                                Callee {
                                    addr: 0x001cd708,
                                    f: ee::s04::cKeyboardPopup_RefreshKeys,
                                }
                                .call(c, 0x001cdf34)?;
                                let t = true;
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                                if t {
                                    break 'b_1ce1f4;
                                } else {
                                    break 'b_1cdf3c;
                                }
                            }
                        }
                        'b_1cdf74: {
                            c.cycles += 6;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xd8));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xdc));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x001cdf4c);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001cdf4c)?;
                            if c.g[V0][0] == 0u64 {
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                break 'b_1cdf74;
                            } else {
                                c.cycles += 8;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6c));
                                c.g[S3][0] = 0x1;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                c.g[RA][0] = sx(0x001cdf6c);
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x6c), c.g[V0][0]);
                                Callee {
                                    addr: 0x001cd708,
                                    f: ee::s04::cKeyboardPopup_RefreshKeys,
                                }
                                .call(c, 0x001cdf6c)?;
                                let t = true;
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                                if t {
                                    break 'b_1ce1f4;
                                } else {
                                    break 'b_1cdf74;
                                }
                            }
                        }
                        c.cycles += 7;
                        c.g[S1][0] = 0u64;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x148));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001cdf88);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001cdf88)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[S5][0] = 0u64;
                        if t {
                            break 'b_1ce1f0;
                        } else {
                            'b_1ce1d8: {
                                'b_1ce138: {
                                    c.cycles += 9;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x001cdfa4);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001cdfa4)?;
                                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8));
                                    c.g[V0][0] = (c.g[V1][0] < 0x72u64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x460000;
                                    if t {
                                        break 'b_1ce138;
                                    } else {
                                        'b_1ce130: {
                                            'b_1ce0f8: {
                                                'b_1ce0d8: {
                                                    'b_1ce0b8: {
                                                        'b_1ce098: {
                                                            'b_1ce078: {
                                                                'b_1ce058: {
                                                                    'b_1ce038: {
                                                                        'b_1ce018: {
                                                                            'b_1cdff8: {
                                                                                'b_1cdfec: {
                                                                                    c.cycles += 6;
                                                                                    c.g[V1][0] = sx(
                                                                                        (c.g[V1][0]
                                                                                            as u32)
                                                                                            << 2,
                                                                                    );
                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x69f0));
                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                    c.g[A0][0] = c
                                                                                        .lw(
                                                                                        c.g[V1][0]
                                                                                            as u32,
                                                                                    );
                                                                                    let t = c.g[A0]
                                                                                        [0]
                                                                                        as u32;
                                                                                    match t {
0x001cdb98 => {
if event_due(c) { deliver_event_here(c, 0x001cdb98); } continue 'l_1cdb98;
}
0x001cdfcc => {
c.cycles += 6;
c.g[A0][0] = c.g[S2][0];
c.g[A1][0] = 0x1b;
c.g[RA][0] = sx(0x001cdfdc);
c.g[A2][0] = 0u64;
Callee { addr: 0x001ce560, f: ee::s04::cKeyboardPopup_IsKeyEnabled }.call(c, 0x001cdfdc)?;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x1b;
if t {
break 'b_1ce130;
} else {
c.cycles += 2;
let t = true;
c.sw((c.g[S2][0] as u32).wrapping_add(0x140), c.g[V0][0]);
if t {
break 'b_1ce1d8;
} else {
break 'b_1cdfec;
}
}
}
0x001cdfec => {
break 'b_1cdfec;
}
0x001cdff8 => {
break 'b_1cdff8;
}
0x001ce018 => {
break 'b_1ce018;
}
0x001ce038 => {
break 'b_1ce038;
}
0x001ce058 => {
break 'b_1ce058;
}
0x001ce078 => {
break 'b_1ce078;
}
0x001ce098 => {
break 'b_1ce098;
}
0x001ce0b8 => {
break 'b_1ce0b8;
}
0x001ce0d8 => {
break 'b_1ce0d8;
}
0x001ce0f8 => {
break 'b_1ce0f8;
}
0x001ce138 => {
break 'b_1ce138;
}
_ => { c.jump_out(t, 0x001cdb98, 0x001ce2a8); return Ok(()); }
}
                                                                                }
                                                                                c.cycles += 3;
                                                                                c.g[V0][0] = 0x37;
                                                                                let t = true;
                                                                                c.sw((c.g[S2][0] as u32).wrapping_add(0x140), c.g[V0][0]);
                                                                                if t {
                                                                                    break 'b_1ce1d8;
                                                                                } else {
                                                                                    break 'b_1cdff8;
                                                                                }
                                                                            }
                                                                            c.cycles += 6;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            c.g[A1][0] = 0x38;
                                                                            c.g[RA][0] =
                                                                                sx(0x001ce008);
                                                                            c.g[A2][0] = 0u64;
                                                                            Callee { addr: 0x001ce560, f: ee::s04::cKeyboardPopup_IsKeyEnabled }.call(c, 0x001ce008)?;
                                                                            let t =
                                                                                c.g[V0][0] == 0u64;
                                                                            c.g[V0][0] = 0x38;
                                                                            if t {
                                                                                break 'b_1ce130;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                let t = true;
                                                                                c.sw((c.g[S2][0] as u32).wrapping_add(0x140), c.g[V0][0]);
                                                                                if t {
                                                                                    break 'b_1ce1d8;
                                                                                } else {
                                                                                    break 'b_1ce018;
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 6;
                                                                        c.g[A0][0] = c.g[S2][0];
                                                                        c.g[A1][0] = 0x2a;
                                                                        c.g[RA][0] = sx(0x001ce028);
                                                                        c.g[A2][0] = 0u64;
                                                                        Callee { addr: 0x001ce560, f: ee::s04::cKeyboardPopup_IsKeyEnabled }.call(c, 0x001ce028)?;
                                                                        let t = c.g[V0][0] == 0u64;
                                                                        c.g[V0][0] = 0x2a;
                                                                        if t {
                                                                            break 'b_1ce130;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            let t = true;
                                                                            c.sw(
                                                                                (c.g[S2][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x140,
                                                                                    ),
                                                                                c.g[V0][0],
                                                                            );
                                                                            if t {
                                                                                break 'b_1ce1d8;
                                                                            } else {
                                                                                break 'b_1ce038;
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 6;
                                                                    c.g[A0][0] = c.g[S2][0];
                                                                    c.g[A1][0] = 0u64;
                                                                    c.g[RA][0] = sx(0x001ce048);
                                                                    c.g[A2][0] = 0u64;
                                                                    Callee { addr: 0x001ce560, f: ee::s04::cKeyboardPopup_IsKeyEnabled }.call(c, 0x001ce048)?;
                                                                    if c.g[V0][0] == 0u64 {
                                                                        c.g[S1][0] = 0x1;
                                                                        break 'b_1ce1d8;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        let t = true;
                                                                        c.sw(
                                                                            (c.g[S2][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x140,
                                                                                ),
                                                                            0u64,
                                                                        );
                                                                        if t {
                                                                            break 'b_1ce1d8;
                                                                        } else {
                                                                            break 'b_1ce058;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 6;
                                                                c.g[A0][0] = c.g[S2][0];
                                                                c.g[A1][0] = 0x4c;
                                                                c.g[RA][0] = sx(0x001ce068);
                                                                c.g[A2][0] = 0u64;
                                                                Callee { addr: 0x001ce560, f: ee::s04::cKeyboardPopup_IsKeyEnabled }.call(c, 0x001ce068)?;
                                                                let t = c.g[V0][0] == 0u64;
                                                                c.g[V0][0] = 0x4c;
                                                                if t {
                                                                    break 'b_1ce130;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = true;
                                                                    c.sw(
                                                                        (c.g[S2][0] as u32)
                                                                            .wrapping_add(0x140),
                                                                        c.g[V0][0],
                                                                    );
                                                                    if t {
                                                                        break 'b_1ce1d8;
                                                                    } else {
                                                                        break 'b_1ce078;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 6;
                                                            c.g[A0][0] = c.g[S2][0];
                                                            c.g[A1][0] = 0x53;
                                                            c.g[RA][0] = sx(0x001ce088);
                                                            c.g[A2][0] = 0u64;
                                                            Callee { addr: 0x001ce560, f: ee::s04::cKeyboardPopup_IsKeyEnabled }.call(c, 0x001ce088)?;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[V0][0] = 0x53;
                                                            if t {
                                                                break 'b_1ce130;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.sw(
                                                                    (c.g[S2][0] as u32)
                                                                        .wrapping_add(0x140),
                                                                    c.g[V0][0],
                                                                );
                                                                if t {
                                                                    break 'b_1ce1d8;
                                                                } else {
                                                                    break 'b_1ce098;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 6;
                                                        c.g[A0][0] = c.g[S2][0];
                                                        c.g[A1][0] = 0x45;
                                                        c.g[RA][0] = sx(0x001ce0a8);
                                                        c.g[A2][0] = 0u64;
                                                        Callee {
                                                            addr: 0x001ce560,
                                                            f: ee::s04::cKeyboardPopup_IsKeyEnabled,
                                                        }
                                                        .call(c, 0x001ce0a8)?;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = 0x45;
                                                        if t {
                                                            break 'b_1ce130;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.sw(
                                                                (c.g[S2][0] as u32)
                                                                    .wrapping_add(0x140),
                                                                c.g[V0][0],
                                                            );
                                                            if t {
                                                                break 'b_1ce1d8;
                                                            } else {
                                                                break 'b_1ce0b8;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 6;
                                                    c.g[A0][0] = c.g[S2][0];
                                                    c.g[A1][0] = 0x4f;
                                                    c.g[RA][0] = sx(0x001ce0c8);
                                                    c.g[A2][0] = 0u64;
                                                    Callee {
                                                        addr: 0x001ce560,
                                                        f: ee::s04::cKeyboardPopup_IsKeyEnabled,
                                                    }
                                                    .call(c, 0x001ce0c8)?;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x4f;
                                                    if t {
                                                        break 'b_1ce130;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.sw(
                                                            (c.g[S2][0] as u32).wrapping_add(0x140),
                                                            c.g[V0][0],
                                                        );
                                                        if t {
                                                            break 'b_1ce1d8;
                                                        } else {
                                                            break 'b_1ce0d8;
                                                        }
                                                    }
                                                }
                                                c.cycles += 6;
                                                c.g[A0][0] = c.g[S2][0];
                                                c.g[A1][0] = 0x1c;
                                                c.g[RA][0] = sx(0x001ce0e8);
                                                c.g[A2][0] = 0u64;
                                                Callee {
                                                    addr: 0x001ce560,
                                                    f: ee::s04::cKeyboardPopup_IsKeyEnabled,
                                                }
                                                .call(c, 0x001ce0e8)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = 0x1c;
                                                if t {
                                                    break 'b_1ce130;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[S2][0] as u32).wrapping_add(0x140),
                                                        c.g[V0][0],
                                                    );
                                                    if t {
                                                        break 'b_1ce1d8;
                                                    } else {
                                                        break 'b_1ce0f8;
                                                    }
                                                }
                                            }
                                            c.cycles += 12;
                                            c.g[V1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x001ce10c);
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x001ce10c)?;
                                            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6c));
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[A1][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x001ce120);
                                            c.g[A2][0] = 0u64;
                                            Callee {
                                                addr: 0x001ce560,
                                                f: ee::s04::cKeyboardPopup_IsKeyEnabled,
                                            }
                                            .call(c, 0x001ce120)?;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[S1][0] = 0x1;
                                                break 'b_1ce1d8;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x140),
                                                    c.g[S0][0],
                                                );
                                                if t {
                                                    break 'b_1ce1d8;
                                                } else {
                                                    break 'b_1ce130;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[S1][0] = 0x1;
                                        if t {
                                            break 'b_1ce1d8;
                                        } else {
                                            break 'b_1ce138;
                                        }
                                    }
                                }
                                c.cycles += 8;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x158));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x15c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001ce14c);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001ce14c)?;
                                c.g[V0][0] = ((c.g[V0][0] as i64) < 32i64) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[S5][0] = 0x1;
                                if t {
                                    break 'b_1ce1d8;
                                } else {
                                    c.cycles += 13;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x158));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x15c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x001ce16c);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001ce16c)?;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[S0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                                    c.g[RA][0] = sx(0x001ce180);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x001cc6e8,
                                        f: ee::s04::cKeyboardPopup_HandleKeyChar,
                                    }
                                    .call(c, 0x001ce180)?;
                                    c.g[S1][0] = c.g[V0][0] ^ 0x1;
                                    let t = c.g[S1][0] != 0u64;
                                    c.g[S5][0] = 0x1;
                                    if t {
                                        break 'b_1ce1d8;
                                    } else {
                                        'b_1ce1d0: {
                                            c.cycles += 6;
                                            c.g[A1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[RA][0] = sx(0x001ce19c);
                                            c.g[A2][0] = 0u64;
                                            Callee {
                                                addr: 0x001ce560,
                                                f: ee::s04::cKeyboardPopup_IsKeyEnabled,
                                            }
                                            .call(c, 0x001ce19c)?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x60;
                                            if t {
                                                break 'b_1ce1d0;
                                            } else {
                                                'b_1ce1c0: {
                                                    c.cycles += 2;
                                                    let t = c.g[S0][0] != c.g[V0][0];
                                                    c.g[A1][0] = c.g[S0][0];
                                                    if t {
                                                        break 'b_1ce1c0;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.g[A0][0] = c.g[S2][0];
                                                        c.g[RA][0] = sx(0x001ce1b8);
                                                        c.g[A1][0] = 0x7e;
                                                        Callee {
                                                            addr: 0x001cc620,
                                                            f: ee::s04::cKeyboardPopup_InsertChar,
                                                        }
                                                        .call(c, 0x001ce1b8)?;
                                                        let t = true;
                                                        c.g[S5][0] = 0x1;
                                                        if t {
                                                            break 'b_1ce1d8;
                                                        } else {
                                                            break 'b_1ce1c0;
                                                        }
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x001ce1c8);
                                                c.g[A0][0] = c.g[S2][0];
                                                Callee {
                                                    addr: 0x001cc620,
                                                    f: ee::s04::cKeyboardPopup_InsertChar,
                                                }
                                                .call(c, 0x001ce1c8)?;
                                                let t = true;
                                                c.g[S5][0] = 0x1;
                                                if t {
                                                    break 'b_1ce1d8;
                                                } else {
                                                    break 'b_1ce1d0;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        c.g[S1][0] = 0x1;
                                        c.g[S5][0] = 0x1;
                                        break 'b_1ce1d8;
                                    }
                                }
                            }
                            c.cycles += 2;
                            if c.g[S1][0] != 0u64 {
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                                break 'b_1ce1f4;
                            } else {
                                c.cycles += 4;
                                c.g[A1][0] = c.g[S5][0];
                                c.g[S3][0] = 0x1;
                                c.g[RA][0] = sx(0x001ce1f0);
                                c.g[A0][0] = c.g[S2][0];
                                Callee {
                                    addr: 0x001cc1e0,
                                    f: ee::s04::cKeyboardPopup_PressKey,
                                }
                                .call(c, 0x001ce1f0)?;
                                break 'b_1ce1f0;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                    break 'b_1ce1f4;
                }
                'b_1ce254: {
                    c.cycles += 11;
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = 0x1;
                    c.g[RA][0] = sx(0x001ce204);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001cd0d8,
                        f: ee::s04::cKeyboardPopup_GetKeyWidgetName,
                    }
                    .call(c, 0x001ce204)?;
                    c.g[RA][0] = sx(0x001ce20c);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001ce20c)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001ce218);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001ce218)?;
                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                        break 'b_1ce254;
                    } else {
                        c.cycles += 13;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001ce250);
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        call_ptr(c, t, 0x001ce250)?;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                        break 'b_1ce254;
                    }
                }
                'b_1ce280: {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    let t = c.g[S3][0] == 0u64;
                    if c.g[A2][0] == 0 {
                        c.g[A2][0] = c.g[V1][0];
                    }
                    if t {
                        break 'b_1ce280;
                    } else {
                        c.cycles += 2;
                        let t = c.g[A2][0] == 0u64;
                        c.g[A1][0] = c.g[S4][0];
                        if t {
                            break 'b_1ce280;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001ce280);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001ce280)?;
                            break 'b_1ce280;
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0x1;
                break 'b_1ce284;
            }
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEPopupConfirm_SetFade(cFEPopupConfirm* self, float t)` (0x1c5f20): set the alpha of the popup's text and background widgets (`+0x70/+0x74/+0x78/+0x7c/+0x88`, the three widgets looked up by hash name, and the five option widgets at `+0x90`) to `t`: each non-null widget gets its colour block (16 bytes at widget+0x1c) copied with `t` as the first float and its vtable slot 0x58 called with it.
/// Converted from the generated body: every register, cycle and call kept.
pub fn c_fe_popup_confirm_set_fade(c: &mut Cpu) {
    guest(c, |c| {
        'b_1c5f80: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
            let t = c.g[A2][0] == 0u64;
            c.mov_s(20, 12);
            if t {
                break 'b_1c5f80;
            } else {
                c.cycles += 15;
                c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x23));
                c.ldr(2, (c.g[A2][0] as u32).wrapping_add(0x1c));
                c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x2b));
                c.ldr(3, (c.g[A2][0] as u32).wrapping_add(0x24));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(2, c.g[SP][0] as u32);
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(20, c.g[SP][0] as u32);
                c.g[A1][0] = c.g[SP][0];
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c5f80);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001c5f80)?;
                break 'b_1c5f80;
            }
        }
        'b_1c5fc4: {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[SP][0];
            if t {
                break 'b_1c5fc4;
            } else {
                c.cycles += 14;
                c.ldl(7, (c.g[A2][0] as u32).wrapping_add(0x23));
                c.ldr(7, (c.g[A2][0] as u32).wrapping_add(0x1c));
                c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x2b));
                c.ldr(2, (c.g[A2][0] as u32).wrapping_add(0x24));
                c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(7, c.g[SP][0] as u32);
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(20, c.g[SP][0] as u32);
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c5fc4);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001c5fc4)?;
                break 'b_1c5fc4;
            }
        }
        'b_1c6010: {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
            if c.g[A2][0] == 0u64 {
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                break 'b_1c6010;
            } else {
                c.cycles += 16;
                c.ldl(5, (c.g[A2][0] as u32).wrapping_add(0x23));
                c.ldr(5, (c.g[A2][0] as u32).wrapping_add(0x1c));
                c.ldl(7, (c.g[A2][0] as u32).wrapping_add(0x2b));
                c.ldr(7, (c.g[A2][0] as u32).wrapping_add(0x24));
                c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(5, c.g[SP][0] as u32);
                c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(20, c.g[SP][0] as u32);
                c.g[A1][0] = c.g[SP][0];
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c600c);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001c600c)?;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                break 'b_1c6010;
            }
        }
        'b_1c6058: {
            c.cycles += 2;
            if c.g[A2][0] == 0u64 {
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
                break 'b_1c6058;
            } else {
                c.cycles += 16;
                c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0x23));
                c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x1c));
                c.ldl(5, (c.g[A2][0] as u32).wrapping_add(0x2b));
                c.ldr(5, (c.g[A2][0] as u32).wrapping_add(0x24));
                c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(4, c.g[SP][0] as u32);
                c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(20, c.g[SP][0] as u32);
                c.g[A1][0] = c.g[SP][0];
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c6054);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001c6054)?;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
                break 'b_1c6058;
            }
        }
        'b_1c60a0: {
            c.cycles += 2;
            if c.g[A2][0] == 0u64 {
                c.g[A0][0] = 0x4a0000;
                break 'b_1c60a0;
            } else {
                c.cycles += 16;
                c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x23));
                c.ldr(3, (c.g[A2][0] as u32).wrapping_add(0x1c));
                c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0x2b));
                c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x24));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(3, c.g[SP][0] as u32);
                c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(20, c.g[SP][0] as u32);
                c.g[A1][0] = c.g[SP][0];
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c609c);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001c609c)?;
                c.g[A0][0] = 0x4a0000;
                break 'b_1c60a0;
            }
        }
        'b_1c6100: {
            c.cycles += 8;
            c.g[RA][0] = sx(0x001c60a8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c50));
            call_ptr(c, 0x00317670, 0x001c60a8)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c60b4);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c60b4)?;
            c.g[A2][0] = c.g[V0][0];
            if c.g[A2][0] == 0u64 {
                c.g[A0][0] = 0x4a0000;
                break 'b_1c6100;
            } else {
                c.cycles += 16;
                c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x23));
                c.ldr(2, (c.g[A2][0] as u32).wrapping_add(0x1c));
                c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x2b));
                c.ldr(3, (c.g[A2][0] as u32).wrapping_add(0x24));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(2, c.g[SP][0] as u32);
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(20, c.g[SP][0] as u32);
                c.g[A1][0] = c.g[SP][0];
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c60fc);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001c60fc)?;
                c.g[A0][0] = 0x4a0000;
                break 'b_1c6100;
            }
        }
        'b_1c6158: {
            c.cycles += 8;
            c.g[RA][0] = sx(0x001c6108);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c58));
            call_ptr(c, 0x00317670, 0x001c6108)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c6114);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c6114)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[SP][0];
            if t {
                break 'b_1c6158;
            } else {
                c.cycles += 14;
                c.ldl(7, (c.g[A2][0] as u32).wrapping_add(0x23));
                c.ldr(7, (c.g[A2][0] as u32).wrapping_add(0x1c));
                c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x2b));
                c.ldr(2, (c.g[A2][0] as u32).wrapping_add(0x24));
                c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(7, c.g[SP][0] as u32);
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(20, c.g[SP][0] as u32);
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c6158);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001c6158)?;
                break 'b_1c6158;
            }
        }
        'b_1c61bc: {
            c.cycles += 9;
            c.g[A0][0] = 0x4a0000;
            c.g[RA][0] = sx(0x001c6164);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x15e0));
            call_ptr(c, 0x00317670, 0x001c6164)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c6170);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c6170)?;
            c.g[A2][0] = c.g[V0][0];
            if c.g[A2][0] == 0u64 {
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x90));
                break 'b_1c61bc;
            } else {
                c.cycles += 16;
                c.ldl(5, (c.g[A2][0] as u32).wrapping_add(0x23));
                c.ldr(5, (c.g[A2][0] as u32).wrapping_add(0x1c));
                c.ldl(7, (c.g[A2][0] as u32).wrapping_add(0x2b));
                c.ldr(7, (c.g[A2][0] as u32).wrapping_add(0x24));
                c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(5, c.g[SP][0] as u32);
                c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(20, c.g[SP][0] as u32);
                c.g[A1][0] = c.g[SP][0];
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c61b8);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001c61b8)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x90));
                break 'b_1c61bc;
            }
        }
        c.cycles += 3;
        c.g[S1][0] = 0x4;
        c.g[A2][0] = c.lw(c.g[S0][0] as u32);
        'l_1c61c8: loop {
            'b_1c6210: {
                c.cycles += 4;
                c.g[A1][0] = c.g[SP][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                let t = c.g[A2][0] == 0u64;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                if t {
                    break 'b_1c6210;
                } else {
                    c.cycles += 14;
                    c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0x23));
                    c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x1c));
                    c.ldl(7, (c.g[A2][0] as u32).wrapping_add(0x2b));
                    c.ldr(7, (c.g[A2][0] as u32).wrapping_add(0x24));
                    c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
                    c.sdr(4, c.g[SP][0] as u32);
                    c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0xf));
                    c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x8));
                    c.swc1(20, c.g[SP][0] as u32);
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6210);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001c6210)?;
                    break 'b_1c6210;
                }
            }
            c.cycles += 2;
            if (c.g[S1][0] as i64) >= 0 {
                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x001c61c8);
                }
                continue 'l_1c61c8;
            } else {
                c.cycles += 6;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void cFEPopupConfirm_FindWidgets(cFEPopupConfirm* self)` (0x1c6230): look up the popup screen's widgets by hash name (`UIScreen_FindObject(self->screen +0x40, StrHash(name))`), keep their pointers in `+0x48..+0x8c`, copy the positions into `+0x188..` and get the rects of three of them; the five option widgets are found by `sprintf(name, fmt, i)` into `+0x90..`.
/// Converted from the generated body: every register, cycle and call kept.
pub fn c_fe_popup_confirm_find_widgets(c: &mut Cpu) {
    guest(c, |c| {
        'b_1c62a8: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[A0][0] = 0x4a0000;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[RA][0] = sx(0x001c6258);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c60));
            call_ptr(c, 0x00317670, 0x001c6258)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c6264);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c6264)?;
            c.g[S1][0] = c.g[V0][0];
            if c.g[S1][0] == 0u64 {
                c.g[A0][0] = 0x4a0000;
                break 'b_1c62a8;
            } else {
                c.cycles += 14;
                c.ldl(2, (c.g[S1][0] as u32).wrapping_add(0x4b));
                c.ldr(2, (c.g[S1][0] as u32).wrapping_add(0x44));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4c));
                c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x18f));
                c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0x188));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x190), c.g[V1][0]);
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa0));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c62a0);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001c62a0)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[S1][0]);
                c.g[A0][0] = 0x4a0000;
                break 'b_1c62a8;
            }
        }
        'b_1c62e4: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c62b0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c68));
            call_ptr(c, 0x00317670, 0x001c62b0)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c62bc);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c62bc)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c62e4;
            } else {
                c.cycles += 8;
                c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(6, (c.g[S0][0] as u32).wrapping_add(0x19b));
                c.sdr(6, (c.g[S0][0] as u32).wrapping_add(0x194));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x19c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c62e4;
            }
        }
        'b_1c6300: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c62ec);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5fa8));
            call_ptr(c, 0x00317670, 0x001c62ec)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c62f8);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c62f8)?;
            if c.g[V0][0] != 0u64 {
                c.sw((c.g[S0][0] as u32).wrapping_add(0x64), c.g[V0][0]);
                break 'b_1c6300;
            } else {
                break 'b_1c6300;
            }
        }
        'b_1c6340: {
            c.cycles += 8;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x001c630c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5fb8));
            call_ptr(c, 0x00317670, 0x001c630c)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c6318);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c6318)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c6340;
            } else {
                c.cycles += 8;
                c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(6, (c.g[S0][0] as u32).wrapping_add(0x237));
                c.sdr(6, (c.g[S0][0] as u32).wrapping_add(0x230));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x238), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c6340;
            }
        }
        'b_1c637c: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c6348);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5fc8));
            call_ptr(c, 0x00317670, 0x001c6348)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c6354);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c6354)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c637c;
            } else {
                c.cycles += 8;
                c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(6, (c.g[S0][0] as u32).wrapping_add(0x22b));
                c.sdr(6, (c.g[S0][0] as u32).wrapping_add(0x224));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x22c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x50), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c637c;
            }
        }
        'b_1c63b8: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c6384);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5fd8));
            call_ptr(c, 0x00317670, 0x001c6384)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c6390);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c6390)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c63b8;
            } else {
                c.cycles += 8;
                c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(6, (c.g[S0][0] as u32).wrapping_add(0x24f));
                c.sdr(6, (c.g[S0][0] as u32).wrapping_add(0x248));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x250), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c63b8;
            }
        }
        'b_1c6408: {
            c.cycles += 8;
            c.g[RA][0] = sx(0x001c63c0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5fe8));
            call_ptr(c, 0x00317670, 0x001c63c0)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c63cc);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c63cc)?;
            c.g[S1][0] = c.g[V0][0];
            let t = c.g[S1][0] == 0u64;
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x218));
            if t {
                break 'b_1c6408;
            } else {
                c.cycles += 12;
                c.ldl(6, (c.g[S1][0] as u32).wrapping_add(0x4b));
                c.ldr(6, (c.g[S1][0] as u32).wrapping_add(0x44));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4c));
                c.sdl(6, (c.g[S0][0] as u32).wrapping_add(0x243));
                c.sdr(6, (c.g[S0][0] as u32).wrapping_add(0x23c));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x244), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa0));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c6404);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001c6404)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[S1][0]);
                break 'b_1c6408;
            }
        }
        'b_1c6448: {
            c.cycles += 8;
            c.g[A0][0] = 0x4a0000;
            c.g[RA][0] = sx(0x001c6414);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c70));
            call_ptr(c, 0x00317670, 0x001c6414)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c6420);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c6420)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c6448;
            } else {
                c.cycles += 8;
                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x25b));
                c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x254));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x25c), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x60), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c6448;
            }
        }
        'b_1c6464: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c6450);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5ff8));
            call_ptr(c, 0x00317670, 0x001c6450)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c645c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c645c)?;
            if c.g[V0][0] != 0u64 {
                c.sw((c.g[S0][0] as u32).wrapping_add(0x68), c.g[V0][0]);
                break 'b_1c6464;
            } else {
                break 'b_1c6464;
            }
        }
        'b_1c64a4: {
            c.cycles += 8;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x001c6470);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2470));
            call_ptr(c, 0x00317670, 0x001c6470)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c647c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c647c)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x4a0000;
                break 'b_1c64a4;
            } else {
                c.cycles += 8;
                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x1a7));
                c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x1a0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1a8), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x6c), c.g[V0][0]);
                c.g[A0][0] = 0x4a0000;
                break 'b_1c64a4;
            }
        }
        'b_1c64e0: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c64ac);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c78));
            call_ptr(c, 0x00317670, 0x001c64ac)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c64b8);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c64b8)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c64e0;
            } else {
                c.cycles += 8;
                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x1b3));
                c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x1ac));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1b4), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x70), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c64e0;
            }
        }
        'b_1c651c: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c64e8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6008));
            call_ptr(c, 0x00317670, 0x001c64e8)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c64f4);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c64f4)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c651c;
            } else {
                c.cycles += 8;
                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x1cb));
                c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x1c4));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1cc), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x74), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c651c;
            }
        }
        'b_1c6558: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c6524);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6018));
            call_ptr(c, 0x00317670, 0x001c6524)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c6530);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c6530)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c6558;
            } else {
                c.cycles += 8;
                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x1bf));
                c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x1b8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c0), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c6558;
            }
        }
        'b_1c6594: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c6560);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6028));
            call_ptr(c, 0x00317670, 0x001c6560)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c656c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c656c)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c6594;
            } else {
                c.cycles += 8;
                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x1d7));
                c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x1d0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1d8), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c6594;
            }
        }
        'b_1c65d0: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c659c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6038));
            call_ptr(c, 0x00317670, 0x001c659c)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c65a8);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c65a8)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c65d0;
            } else {
                c.cycles += 8;
                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x1e3));
                c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x1dc));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1e4), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x80), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c65d0;
            }
        }
        'b_1c660c: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c65d8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6048));
            call_ptr(c, 0x00317670, 0x001c65d8)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c65e4);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c65e4)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c660c;
            } else {
                c.cycles += 8;
                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x1ef));
                c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x1e8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1f0), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x88), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c660c;
            }
        }
        'b_1c6628: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c6614);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6058));
            call_ptr(c, 0x00317670, 0x001c6614)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c6620);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c6620)?;
            if c.g[V0][0] != 0u64 {
                c.sw((c.g[S0][0] as u32).wrapping_add(0x84), c.g[V0][0]);
                break 'b_1c6628;
            } else {
                break 'b_1c6628;
            }
        }
        'b_1c6684: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x001c6634);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6070));
            call_ptr(c, 0x00317670, 0x001c6634)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c6640);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c6640)?;
            c.g[S1][0] = c.g[V0][0];
            if c.g[S1][0] == 0u64 {
                c.g[S2][0] = 0u64;
                break 'b_1c6684;
            } else {
                c.cycles += 14;
                c.ldl(5, (c.g[S1][0] as u32).wrapping_add(0x4b));
                c.ldr(5, (c.g[S1][0] as u32).wrapping_add(0x44));
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4c));
                c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x207));
                c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x200));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x208), c.g[A2][0]);
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20c));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa0));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c667c);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001c667c)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8c), c.g[S1][0]);
                c.g[S2][0] = 0u64;
                break 'b_1c6684;
            }
        }
        c.cycles += 3;
        c.g[S3][0] = 0x460000;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x90));
        'l_1c6690: loop {
            'b_1c66c4: {
                c.cycles += 12;
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6080));
                c.g[RA][0] = sx(0x001c66a4);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x00417828, 0x001c66a4)?;
                c.g[RA][0] = sx(0x001c66ac);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x001c66ac)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001c66b8);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001c66b8)?;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = ((c.g[S2][0] as i64) < 5i64) as u64;
                if t {
                    break 'b_1c66c4;
                } else {
                    c.cycles += 1;
                    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                    break 'b_1c66c4;
                }
            }
            c.cycles += 2;
            let t = c.g[V1][0] != 0u64;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001c6690);
                }
                continue 'l_1c6690;
            } else {
                c.cycles += 7;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void cFEPopupConfirm_ApplyBackgroundStyle(cFEPopupConfirm* self, int style)` (0x1c66e8): show/hide the background widgets for the popup style (0: online/white, 1: blue, 2: black; stored at `+0x31c`) and recolour the four text widgets (`+0x74/+0x78/+0x7c/+0x88`).
/// Converted from the generated body: every register, cycle and call kept.
pub fn c_fe_popup_confirm_apply_background_style(c: &mut Cpu) {
    guest(c, |c| {
        'b_1c6af4: {
            'b_1c69b8: {
                'b_1c6874: {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = 0x1;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A0][0];
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x31c));
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                    if t {
                        break 'b_1c6874;
                    } else {
                        'b_1c6734: {
                            'b_1c6724: {
                                c.cycles += 2;
                                if c.g[V0][0] == 0u64 {
                                    c.g[V0][0] = 0x2;
                                    break 'b_1c6724;
                                } else {
                                    c.cycles += 2;
                                    if c.g[V1][0] == 0u64 {
                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
                                        break 'b_1c6734;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        if t {
                                            break 'b_1c6af4;
                                        } else {
                                            break 'b_1c6724;
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                                break 'b_1c69b8;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_1c6af4;
                                } else {
                                    break 'b_1c6734;
                                }
                            }
                        }
                        'b_1c67c0: {
                            c.cycles += 30;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001c6740);
                            c.g[S1][0] = 0x4c0000;
                            Callee {
                                addr: 0x001ca4c0,
                                f: ee::s03::cFEPopupConfirm_ShowWidget,
                            }
                            .call(c, 0x001c6740)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                            c.g[RA][0] = sx(0x001c674c);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001ca4c0,
                                f: ee::s03::cFEPopupConfirm_ShowWidget,
                            }
                            .call(c, 0x001c674c)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                            c.g[RA][0] = sx(0x001c6758);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001ca4c0,
                                f: ee::s03::cFEPopupConfirm_ShowWidget,
                            }
                            .call(c, 0x001c6758)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                            c.g[RA][0] = sx(0x001c6764);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001ca488,
                                f: ee::s03::cFEPopupConfirm_HideWidget,
                            }
                            .call(c, 0x001c6764)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                            c.g[RA][0] = sx(0x001c6770);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001ca488,
                                f: ee::s03::cFEPopupConfirm_HideWidget,
                            }
                            .call(c, 0x001c6770)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                            c.g[RA][0] = sx(0x001c677c);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001ca488,
                                f: ee::s03::cFEPopupConfirm_HideWidget,
                            }
                            .call(c, 0x001c677c)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                            c.g[RA][0] = sx(0x001c6788);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001ca488,
                                f: ee::s03::cFEPopupConfirm_HideWidget,
                            }
                            .call(c, 0x001c6788)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
                            c.g[RA][0] = sx(0x001c6794);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001ca488,
                                f: ee::s03::cFEPopupConfirm_HideWidget,
                            }
                            .call(c, 0x001c6794)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                            c.g[RA][0] = sx(0x001c67a0);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001ca488,
                                f: ee::s03::cFEPopupConfirm_HideWidget,
                            }
                            .call(c, 0x001c67a0)?;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                            let t = c.g[A2][0] == 0u64;
                            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6798));
                            if t {
                                break 'b_1c67c0;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001c67c0);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001c67c0)?;
                                break 'b_1c67c0;
                            }
                        }
                        'b_1c67e0: {
                            c.cycles += 3;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                            let t = c.g[A2][0] == 0u64;
                            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6798));
                            if t {
                                break 'b_1c67e0;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001c67e0);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001c67e0)?;
                                break 'b_1c67e0;
                            }
                        }
                        'b_1c6800: {
                            c.cycles += 3;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                            let t = c.g[A2][0] == 0u64;
                            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6798));
                            if t {
                                break 'b_1c6800;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001c6800);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001c6800)?;
                                break 'b_1c6800;
                            }
                        }
                        'b_1c6820: {
                            c.cycles += 3;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                            let t = c.g[A2][0] == 0u64;
                            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6798));
                            if t {
                                break 'b_1c6820;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001c6820);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001c6820)?;
                                break 'b_1c6820;
                            }
                        }
                        c.cycles += 21;
                        c.g[V0][0] = 0x4c0000;
                        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6788));
                        c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                        c.ldr(3, c.g[A2][0] as u32);
                        c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                        c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                        c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x333));
                        c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x32c));
                        c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0x33b));
                        c.sdr(4, (c.g[S0][0] as u32).wrapping_add(0x334));
                        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6798));
                        c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                        c.ldr(2, c.g[A1][0] as u32);
                        c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
                        c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
                        c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x343));
                        c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0x33c));
                        c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x34b));
                        c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x344));
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        if t {
                            break 'b_1c6af4;
                        } else {
                            break 'b_1c6874;
                        }
                    }
                }
                'b_1c6904: {
                    c.cycles += 31;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001c6884);
                    c.g[S1][0] = 0x4c0000;
                    Callee {
                        addr: 0x001ca4c0,
                        f: ee::s03::cFEPopupConfirm_ShowWidget,
                    }
                    .call(c, 0x001c6884)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                    c.g[RA][0] = sx(0x001c6890);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001ca4c0,
                        f: ee::s03::cFEPopupConfirm_ShowWidget,
                    }
                    .call(c, 0x001c6890)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                    c.g[RA][0] = sx(0x001c689c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001ca4c0,
                        f: ee::s03::cFEPopupConfirm_ShowWidget,
                    }
                    .call(c, 0x001c689c)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                    c.g[RA][0] = sx(0x001c68a8);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001ca4c0,
                        f: ee::s03::cFEPopupConfirm_ShowWidget,
                    }
                    .call(c, 0x001c68a8)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                    c.g[RA][0] = sx(0x001c68b4);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001ca4c0,
                        f: ee::s03::cFEPopupConfirm_ShowWidget,
                    }
                    .call(c, 0x001c68b4)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
                    c.g[RA][0] = sx(0x001c68c0);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001ca4c0,
                        f: ee::s03::cFEPopupConfirm_ShowWidget,
                    }
                    .call(c, 0x001c68c0)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x001c68cc);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001ca488,
                        f: ee::s03::cFEPopupConfirm_HideWidget,
                    }
                    .call(c, 0x001c68cc)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                    c.g[RA][0] = sx(0x001c68d8);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001ca488,
                        f: ee::s03::cFEPopupConfirm_HideWidget,
                    }
                    .call(c, 0x001c68d8)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                    c.g[RA][0] = sx(0x001c68e4);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001ca488,
                        f: ee::s03::cFEPopupConfirm_HideWidget,
                    }
                    .call(c, 0x001c68e4)?;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6798));
                    if t {
                        break 'b_1c6904;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001c6904);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001c6904)?;
                        break 'b_1c6904;
                    }
                }
                'b_1c6924: {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6798));
                    if t {
                        break 'b_1c6924;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001c6924);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001c6924)?;
                        break 'b_1c6924;
                    }
                }
                'b_1c6944: {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6798));
                    if t {
                        break 'b_1c6944;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001c6944);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001c6944)?;
                        break 'b_1c6944;
                    }
                }
                'b_1c6964: {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6798));
                    if t {
                        break 'b_1c6964;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001c6964);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001c6964)?;
                        break 'b_1c6964;
                    }
                }
                c.cycles += 21;
                c.g[V0][0] = 0x4c0000;
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6788));
                c.ldl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
                c.ldr(6, c.g[A1][0] as u32);
                c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
                c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.sdl(6, (c.g[S0][0] as u32).wrapping_add(0x333));
                c.sdr(6, (c.g[S0][0] as u32).wrapping_add(0x32c));
                c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x33b));
                c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x334));
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6798));
                c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x7));
                c.ldr(6, c.g[A0][0] as u32);
                c.ldl(2, (c.g[A0][0] as u32).wrapping_add(0xf));
                c.ldr(2, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.sdl(6, (c.g[S0][0] as u32).wrapping_add(0x343));
                c.sdr(6, (c.g[S0][0] as u32).wrapping_add(0x33c));
                c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x34b));
                c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0x344));
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                if t {
                    break 'b_1c6af4;
                } else {
                    break 'b_1c69b8;
                }
            }
            'b_1c6a44: {
                c.cycles += 30;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x001c69c4);
                c.g[S1][0] = 0x4c0000;
                Callee {
                    addr: 0x001ca488,
                    f: ee::s03::cFEPopupConfirm_HideWidget,
                }
                .call(c, 0x001c69c4)?;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                c.g[RA][0] = sx(0x001c69d0);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001ca488,
                    f: ee::s03::cFEPopupConfirm_HideWidget,
                }
                .call(c, 0x001c69d0)?;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                c.g[RA][0] = sx(0x001c69dc);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001ca488,
                    f: ee::s03::cFEPopupConfirm_HideWidget,
                }
                .call(c, 0x001c69dc)?;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                c.g[RA][0] = sx(0x001c69e8);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001ca488,
                    f: ee::s03::cFEPopupConfirm_HideWidget,
                }
                .call(c, 0x001c69e8)?;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                c.g[RA][0] = sx(0x001c69f4);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001ca488,
                    f: ee::s03::cFEPopupConfirm_HideWidget,
                }
                .call(c, 0x001c69f4)?;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                c.g[RA][0] = sx(0x001c6a00);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001ca488,
                    f: ee::s03::cFEPopupConfirm_HideWidget,
                }
                .call(c, 0x001c6a00)?;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
                c.g[RA][0] = sx(0x001c6a0c);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001ca488,
                    f: ee::s03::cFEPopupConfirm_HideWidget,
                }
                .call(c, 0x001c6a0c)?;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
                c.g[RA][0] = sx(0x001c6a18);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001ca488,
                    f: ee::s03::cFEPopupConfirm_HideWidget,
                }
                .call(c, 0x001c6a18)?;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                c.g[RA][0] = sx(0x001c6a24);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001ca4c0,
                    f: ee::s03::cFEPopupConfirm_ShowWidget,
                }
                .call(c, 0x001c6a24)?;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                let t = c.g[A2][0] == 0u64;
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6788));
                if t {
                    break 'b_1c6a44;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6a44);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001c6a44)?;
                    break 'b_1c6a44;
                }
            }
            'b_1c6a64: {
                c.cycles += 3;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                let t = c.g[A2][0] == 0u64;
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6788));
                if t {
                    break 'b_1c6a64;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6a64);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001c6a64)?;
                    break 'b_1c6a64;
                }
            }
            'b_1c6a84: {
                c.cycles += 3;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                let t = c.g[A2][0] == 0u64;
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6788));
                if t {
                    break 'b_1c6a84;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6a84);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001c6a84)?;
                    break 'b_1c6a84;
                }
            }
            'b_1c6aa4: {
                c.cycles += 3;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                let t = c.g[A2][0] == 0u64;
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6788));
                if t {
                    break 'b_1c6aa4;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6aa4);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001c6aa4)?;
                    break 'b_1c6aa4;
                }
            }
            c.cycles += 20;
            c.g[V0][0] = 0x4c0000;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x67a8));
            c.ldl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
            c.ldr(5, c.g[A0][0] as u32);
            c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
            c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
            c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x333));
            c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x32c));
            c.sdl(6, (c.g[S0][0] as u32).wrapping_add(0x33b));
            c.sdr(6, (c.g[S0][0] as u32).wrapping_add(0x334));
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6788));
            c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.ldr(5, c.g[V1][0] as u32);
            c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
            c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x343));
            c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x33c));
            c.sdl(6, (c.g[S0][0] as u32).wrapping_add(0x34b));
            c.sdr(6, (c.g[S0][0] as u32).wrapping_add(0x344));
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1c6af4;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEPopupConfirm_ApplyConfig(cFEPopupConfirm* self, Config* cfg)` (0x1c6b08): find the widgets, copy the configuration (fields, title/body/option text slots, size and fade) into the popup, lay out title, body, options, menu, buttons and post text, apply the background style and frame, and lay out the text entry box when the config has no selection (`cfg[0] == -1`).
/// Converted from the generated body: every register, cycle and call kept.
pub fn c_fe_popup_confirm_apply_config(c: &mut Cpu) {
    guest(c, |c| {
        'b_1c6c04: {
            'b_1c6ba0: {
                c.cycles += 27;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[RA][0] = sx(0x001c6b2c);
                c.g[S3][0] = c.g[A1][0];
                Callee {
                    addr: 0x001c6230,
                    f: ee::s03::cFEPopupConfirm_FindWidgets,
                }
                .call(c, 0x001c6b2c)?;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x94));
                c.mtc1(0, 1);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x98));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x2c0), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xa8));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x310), c.g[V0][0]);
                c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x30c), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xb8));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x2c8), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xbc));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x2cc), c.g[V0][0]);
                c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0xb0));
                c.c_eq_s(0, 1);
                let t = c.fcc();
                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x318));
                if t {
                    break 'b_1c6ba0;
                } else {
                    c.cycles += 11;
                    c.swc1(0, c.g[SP][0] as u32);
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[SP][0] as u32);
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S2][0] as u32).wrapping_add(0x267));
                    c.sdr(2, (c.g[S2][0] as u32).wrapping_add(0x260));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x268), c.g[V1][0]);
                    let t = true;
                    c.g[S1][0] = 0u64;
                    if t {
                        break 'b_1c6c04;
                    } else {
                        break 'b_1c6ba0;
                    }
                }
            }
            'b_1c6bd8: {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64;
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                if t {
                    break 'b_1c6bd8;
                } else {
                    c.cycles += 12;
                    c.g[AT][0] = 0x43ec0000;
                    c.mtc1(1, 0);
                    c.swc1(0, c.g[SP][0] as u32);
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
                    c.ldr(6, c.g[SP][0] as u32);
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.sdl(6, (c.g[S2][0] as u32).wrapping_add(0x267));
                    c.sdr(6, (c.g[S2][0] as u32).wrapping_add(0x260));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x268), c.g[A3][0]);
                    let t = true;
                    c.g[S1][0] = 0u64;
                    if t {
                        break 'b_1c6c04;
                    } else {
                        break 'b_1c6bd8;
                    }
                }
            }
            c.cycles += 11;
            c.g[AT][0] = 0x43960000;
            c.mtc1(1, 0);
            c.swc1(0, c.g[SP][0] as u32);
            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
            c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.ldr(4, c.g[SP][0] as u32);
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
            c.sdl(4, (c.g[S2][0] as u32).wrapping_add(0x267));
            c.sdr(4, (c.g[S2][0] as u32).wrapping_add(0x260));
            c.sw((c.g[S2][0] as u32).wrapping_add(0x268), c.g[A1][0]);
            c.g[S1][0] = 0u64;
            break 'b_1c6c04;
        }
        c.cycles += 1;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2d0));
        'l_1c6c08: loop {
            c.cycles += 13;
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001c6c14);
            c.g[A0][0] = c.g[S3][0];
            Callee {
                addr: 0x001c58d0,
                f: ee::s03::cFEPopupConfirm_CfgGetOptionConfirm,
            }
            .call(c, 0x001c6c14)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
            c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
            c.ldr(3, c.g[V0][0] as u32);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
            c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x7));
            c.sdr(3, c.g[S0][0] as u32);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.g[V0][0] = ((c.g[S1][0] as i64) < 5i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001c6c08);
                }
                continue 'l_1c6c08;
            } else {
                'b_1c6cc0: {
                    c.cycles += 31;
                    c.ldl(7, (c.g[S3][0] as u32).wrapping_add(0xa3));
                    c.ldr(7, (c.g[S3][0] as u32).wrapping_add(0x9c));
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xa4));
                    c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                    c.sdr(7, c.g[SP][0] as u32);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[A1][0] = c.g[S3][0];
                    c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
                    c.ldr(6, c.g[SP][0] as u32);
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.sdl(6, (c.g[S2][0] as u32).wrapping_add(0x273));
                    c.sdr(6, (c.g[S2][0] as u32).wrapping_add(0x26c));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x274), c.g[A3][0]);
                    c.g[RA][0] = sx(0x001c6c78);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001c8678,
                        f: ee::s03::cFEPopupConfirm_LayoutTitle,
                    }
                    .call(c, 0x001c6c78)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001c6c84);
                    c.g[A1][0] = c.g[S3][0];
                    Callee {
                        addr: 0x001c7258,
                        f: ee::s03::cFEPopupConfirm_LayoutBodyText,
                    }
                    .call(c, 0x001c6c84)?;
                    c.g[A1][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x001c6c90);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001c7388,
                        f: ee::s03::cFEPopupConfirm_LayoutOptions,
                    }
                    .call(c, 0x001c6c90)?;
                    c.g[RA][0] = sx(0x001c6c98);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001c8758,
                        f: ee::s03::cFEPopupConfirm_LayoutMenu,
                    }
                    .call(c, 0x001c6c98)?;
                    c.g[RA][0] = sx(0x001c6ca0);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001c88c8,
                        f: ee::s03::cFEPopupConfirm_PositionButtons,
                    }
                    .call(c, 0x001c6ca0)?;
                    c.g[A1][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x001c6cac);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001c7040,
                        f: ee::s03::cFEPopupConfirm_LayoutPostText,
                    }
                    .call(c, 0x001c6cac)?;
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x90));
                    let t = c.g[A1][0] == 0u64;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x314), c.g[A1][0]);
                    if t {
                        break 'b_1c6cc0;
                    } else {
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x001c6cc0);
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x001c9cd0,
                            f: ee::s03::cFEPopupConfirm_AdjustDrawLayer,
                        }
                        .call(c, 0x001c6cc0)?;
                        break 'b_1c6cc0;
                    }
                }
                'b_1c6d18: {
                    c.cycles += 15;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc0));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001c6cd4);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x31c), c.g[V0][0]);
                    Callee {
                        addr: 0x001c66e8,
                        f: ee::s03::cFEPopupConfirm_ApplyBackgroundStyle,
                    }
                    .call(c, 0x001c6cd4)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xac));
                    c.g[A0][0] = c.g[S2][0];
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x2c4), c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc4));
                    c.g[RA][0] = sx(0x001c6cec);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x328), c.g[V0][0]);
                    Callee {
                        addr: 0x001c7c20,
                        f: ee::s03::cFEPopupConfirm_LayoutFrame,
                    }
                    .call(c, 0x001c6cec)?;
                    c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = 0xffffffffffffffff;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    if t {
                        break 'b_1c6d18;
                    } else {
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x001c6d04);
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x001c7eb0,
                            f: ee::s03::cFEPopupConfirm_LayoutTextEntryBox,
                        }
                        .call(c, 0x001c6d04)?;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x80));
                        c.g[RA][0] = sx(0x001c6d14);
                        c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x2b0));
                        Callee {
                            addr: 0x001ca6a0,
                            f: ee::s03::cFEPopupConfirm_CheckAndFixTextEntry,
                        }
                        .call(c, 0x001c6d14)?;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                        break 'b_1c6d18;
                    }
                }
                c.cycles += 6;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `int cFEPopupConfirm_LayoutTextEntry(self, info)` (0x1c6d30): lay out the text-entry widgets (`+0x80/+0x84/+0x88/+0x8c`) on the first call (`info[0] == -1`, then set to 1, returns 1), else hide them again (returns 0).
/// Converted from the generated body: every register, cycle and call kept.
pub fn c_fe_popup_confirm_layout_text_entry(c: &mut Cpu) {
    guest(c, |c| {
        'b_1c7024: {
            'b_1c6f28: {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                c.g[V0][0] = 0xffffffffffffffff;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S2][0] = c.g[A1][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[S1][0] = c.g[A0][0];
                if t {
                    break 'b_1c6f28;
                } else {
                    'b_1c6e10: {
                        c.cycles += 12;
                        c.g[V0][0] = 0x500000;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                        c.mtc1(0, 20);
                        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf28));
                        c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
                        c.ldr(3, c.g[A3][0] as u32);
                        c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x17));
                        c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                        let t = c.g[A1][0] == 0u64;
                        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x14));
                        if t {
                            break 'b_1c6e10;
                        } else {
                            c.cycles += 33;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x001c6d98);
                            c.g[A2][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001c8050,
                                f: ee::s03::cFEPopupConfirm_SetTextFromDesc,
                            }
                            .call(c, 0x001c6d98)?;
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                            c.g[A2][0] = c.g[SP][0];
                            c.g[RA][0] = sx(0x001c6da8);
                            c.g[A0][0] = c.g[S1][0];
                            Callee {
                                addr: 0x001c8628,
                                f: ee::s03::cFEPopupConfirm_GetWidgetSize,
                            }
                            .call(c, 0x001c6da8)?;
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                            c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                            c.g[A2][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001c6dbc);
                            c.g[A0][0] = c.g[S1][0];
                            Callee {
                                addr: 0x001c8320,
                                f: ee::s03::cFEPopupConfirm_MeasureDesc,
                            }
                            .call(c, 0x001c6dbc)?;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                            c.g[A1][0] = 0u64;
                            c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x28));
                            c.ldl(8, (c.g[A0][0] as u32).wrapping_add(0x4b));
                            c.ldr(8, (c.g[A0][0] as u32).wrapping_add(0x44));
                            c.g[T1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4c));
                            c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0x17));
                            c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[A2][0] as u32;
                            c.g[RA][0] = sx(0x001c6df4);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            call_ptr(c, t, 0x001c6df4)?;
                            c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                            c.g[A1][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001c6e10);
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001c6e10)?;
                            break 'b_1c6e10;
                        }
                    }
                    'b_1c6ebc: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2b0));
                        if t {
                            break 'b_1c6ebc;
                        } else {
                            c.cycles += 40;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x001c6e28);
                            c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1c));
                            Callee {
                                addr: 0x001c80e8,
                                f: ee::s03::cFEPopupConfirm_DescToString,
                            }
                            .call(c, 0x001c6e28)?;
                            c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x17));
                            c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                            c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x27));
                            c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A1][0]);
                            c.g[AT][0] = 0x41200000;
                            c.mtc1(1, 1);
                            c.g[A0][0] = c.g[S1][0];
                            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.g[A2][0] = c.g[SP][0];
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                            c.add_s(0, 0, 20);
                            c.add_s(0, 0, 1);
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x27));
                            c.ldr(8, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                            c.sdl(8, (c.g[V0][0] as u32).wrapping_add(0x4b));
                            c.sdr(8, (c.g[V0][0] as u32).wrapping_add(0x44));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[T1][0]);
                            c.g[RA][0] = sx(0x001c6e84);
                            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x260));
                            Callee {
                                addr: 0x001c9b68,
                                f: ee::s03::cFEPopupConfirm_AccumulateSize,
                            }
                            .call(c, 0x001c6e84)?;
                            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001c6ea0);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001c6ea0)?;
                            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                            c.g[A1][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001c6ebc);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001c6ebc)?;
                            break 'b_1c6ebc;
                        }
                    }
                    'b_1c6ef8: {
                        c.cycles += 3;
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x88));
                        let t = c.g[A2][0] == 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_1c6ef8;
                        } else {
                            c.cycles += 12;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001c6edc);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001c6edc)?;
                            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x88));
                            c.g[A1][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001c6ef8);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001c6ef8)?;
                            break 'b_1c6ef8;
                        }
                    }
                    'b_1c6f18: {
                        c.cycles += 3;
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8c));
                        let t = c.g[A2][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_1c6f18;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001c6f18);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001c6f18)?;
                            break 'b_1c6f18;
                        }
                    }
                    c.cycles += 4;
                    c.g[V1][0] = 0x1;
                    c.g[V0][0] = 0x1;
                    let t = true;
                    c.sw(c.g[S2][0] as u32, c.g[V1][0]);
                    if t {
                        break 'b_1c7024;
                    } else {
                        break 'b_1c6f28;
                    }
                }
            }
            'b_1c6f64: {
                c.cycles += 3;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x88));
                let t = c.g[A2][0] == 0u64;
                c.g[A1][0] = 0x1;
                if t {
                    break 'b_1c6f64;
                } else {
                    c.cycles += 12;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6f48);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001c6f48)?;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x88));
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6f64);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001c6f64)?;
                    break 'b_1c6f64;
                }
            }
            'b_1c6fa0: {
                c.cycles += 3;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                let t = c.g[A2][0] == 0u64;
                c.g[A1][0] = 0x1;
                if t {
                    break 'b_1c6fa0;
                } else {
                    c.cycles += 12;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6f84);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001c6f84)?;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6fa0);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001c6fa0)?;
                    break 'b_1c6fa0;
                }
            }
            'b_1c6fdc: {
                c.cycles += 3;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                let t = c.g[A2][0] == 0u64;
                c.g[A1][0] = 0x1;
                if t {
                    break 'b_1c6fdc;
                } else {
                    c.cycles += 12;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6fc0);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001c6fc0)?;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6fdc);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001c6fdc)?;
                    break 'b_1c6fdc;
                }
            }
            'b_1c7004: {
                c.cycles += 3;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8c));
                if c.g[A2][0] == 0u64 {
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                    break 'b_1c7004;
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c7000);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001c7000)?;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                    break 'b_1c7004;
                }
            }
            c.cycles += 8;
            c.g[A2][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x001c7010);
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: 0x001c8628,
                f: ee::s03::cFEPopupConfirm_GetWidgetSize,
            }
            .call(c, 0x001c7010)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x001c7020);
            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x260));
            Callee {
                addr: 0x001c9b68,
                f: ee::s03::cFEPopupConfirm_AccumulateSize,
            }
            .call(c, 0x001c7020)?;
            c.g[V0][0] = 0u64;
            break 'b_1c7024;
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEPopupConfirm_LayoutOptions(self, info)` (0x1c7388): lay out the
/// option buttons of a confirm popup. `info[0]` is the option count: the
/// background object's flag 8 (`self+0x6c -> +0x90`) is set for 0 or 1 options
/// and cleared otherwise; the text entry is laid out
/// (`cFEPopupConfirm_LayoutTextEntry`), the `+0x70` widget is shown only when
/// there are options, unused option widgets (`self+0x90 + 4*i`, i >= count) are
/// made static and hidden, and every used one gets its text, size and measured
/// width (the widest goes to `+0x29c`, the sizes are accumulated into
/// `+0x260`) before it becomes non-static and visible. Finally `+0x264` gets
/// 10.0 added and, when the text entry layout asked for it, `info[0] = -1`.
pub fn c_fe_popup_confirm_layout_options(c: &mut Cpu) {
    if c.pc != 0x001c7388 {
        return ee::s03::f_001c7388__generated(c);
    }
    guest(c, |c| {
        'b_1c73ec: {
            'b_1c73dc: {
                'b_1c73c8: {
                    c.cycles += 13;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S5][0] = c.g[A1][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    c.g[V1][0] = c.lw(c.g[S5][0] as u32);
                    let t = c.g[V1][0] == 0u64;
                    c.g[S2][0] = c.g[A0][0];
                    if t {
                        break 'b_1c73c8;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6c));
                            break 'b_1c73dc;
                        } else {
                            break 'b_1c73c8;
                        }
                    }
                }
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6c));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x90));
                c.g[V0][0] = c.g[V0][0] | 0x8;
                let t = true;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x90), c.g[V0][0]);
                if t {
                    break 'b_1c73ec;
                } else {
                    break 'b_1c73dc;
                }
            }
            c.cycles += 4;
            c.g[V1][0] = 0xfffffffffffffff7;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x90));
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.sw((c.g[A0][0] as u32).wrapping_add(0x90), c.g[V0][0]);
            break 'b_1c73ec;
        }
        'b_1c744c: {
            'b_1c7430: {
                c.cycles += 9;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x290), 0u64);
                c.g[A0][0] = c.g[S2][0];
                c.sw((c.g[S2][0] as u32).wrapping_add(0x29c), 0u64);
                c.g[RA][0] = sx(0x001c7400);
                c.g[A1][0] = c.g[S5][0];
                Callee {
                    addr: 0x001c6d30,
                    f: ee::s03::cFEPopupConfirm_LayoutTextEntry,
                }
                .call(c, 0x001c7400)?;
                c.g[S6][0] = c.g[V0][0];
                c.g[V0][0] = c.lw(c.g[S5][0] as u32);
                let t = c.g[V0][0] != 0u64;
                c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x70));
                if t {
                    break 'b_1c7430;
                } else {
                    c.cycles += 8;
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c7428);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001c7428)?;
                    let t = true;
                    c.g[S1][0] = c.lw(c.g[S5][0] as u32);
                    if t {
                        break 'b_1c744c;
                    } else {
                        break 'b_1c7430;
                    }
                }
            }
            c.cycles += 7;
            c.g[A1][0] = 0x1;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001c7448);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x001c7448)?;
            c.g[S1][0] = c.lw(c.g[S5][0] as u32);
            break 'b_1c744c;
        }
        'b_1c74ac: {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 5i64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
            if t {
                break 'b_1c74ac;
            } else {
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x90));
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                'l_1c7460: loop {
                    'b_1c749c: {
                        c.cycles += 3;
                        c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                        let t = c.g[A2][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_1c749c;
                        } else {
                            c.cycles += 12;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001c7480);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001c7480)?;
                            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001c749c);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001c749c)?;
                            break 'b_1c749c;
                        }
                    }
                    c.cycles += 4;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 5i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001c7460);
                        }
                        continue 'l_1c7460;
                    } else {
                        break 'b_1c74ac;
                    }
                }
            }
        }
        'b_1c757c: {
            c.cycles += 3;
            c.g[V0][0] = c.lw(c.g[S5][0] as u32);
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S3][0] = 0u64;
            if t {
                break 'b_1c757c;
            } else {
                c.cycles += 2;
                c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x90));
                'l_1c74c0: loop {
                    'b_1c756c: {
                        c.cycles += 3;
                        c.g[S1][0] = c.lw(c.g[S4][0] as u32);
                        if c.g[S1][0] == 0u64 {
                            c.g[V0][0] = c.lw(c.g[S5][0] as u32);
                            break 'b_1c756c;
                        } else {
                            'b_1c7528: {
                                c.cycles += 23;
                                c.g[A0][0] = c.g[S5][0];
                                c.g[RA][0] = sx(0x001c74d8);
                                c.g[A1][0] = c.g[S3][0];
                                Callee {
                                    addr: 0x001c5800,
                                    f: ee::s03::cFEPopupConfirm_CfgGetOption,
                                }
                                .call(c, 0x001c74d8)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A1][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x001c74ec);
                                c.g[A2][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x001c8050,
                                    f: ee::s03::cFEPopupConfirm_SetTextFromDesc,
                                }
                                .call(c, 0x001c74ec)?;
                                c.g[A2][0] = c.g[SP][0];
                                c.g[A0][0] = c.g[S2][0];
                                c.g[RA][0] = sx(0x001c74fc);
                                c.g[A1][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x001c8628,
                                    f: ee::s03::cFEPopupConfirm_GetWidgetSize,
                                }
                                .call(c, 0x001c74fc)?;
                                c.g[A2][0] = c.g[S0][0];
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A1][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x001c7510);
                                c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                Callee {
                                    addr: 0x001c8320,
                                    f: ee::s03::cFEPopupConfirm_MeasureDesc,
                                }
                                .call(c, 0x001c7510)?;
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
                                c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x29c));
                                c.c_lt_s(0, 1);
                                if c.fcc() {
                                    c.swc1(1, (c.g[S2][0] as u32).wrapping_add(0x29c));
                                    break 'b_1c7528;
                                } else {
                                    break 'b_1c7528;
                                }
                            }
                            c.cycles += 17;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x260));
                            c.g[RA][0] = sx(0x001c7538);
                            c.g[A2][0] = c.g[SP][0];
                            Callee {
                                addr: 0x001c9b68,
                                f: ee::s03::cFEPopupConfirm_AccumulateSize,
                            }
                            .call(c, 0x001c7538)?;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001c7550);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001c7550)?;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001c7568);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001c7568)?;
                            c.g[V0][0] = c.lw(c.g[S5][0] as u32);
                            break 'b_1c756c;
                        }
                    }
                    c.cycles += 4;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001c74c0);
                        }
                        continue 'l_1c74c0;
                    } else {
                        break 'b_1c757c;
                    }
                }
            }
        }
        'b_1c759c: {
            c.cycles += 6;
            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x264));
            c.g[AT][0] = 0x41200000;
            c.mtc1(1, 1);
            c.add_s(0, 0, 1);
            let t = c.g[S6][0] == 0u64;
            c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x264));
            if t {
                break 'b_1c759c;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0xffffffffffffffff;
                c.sw(c.g[S5][0] as u32, c.g[V0][0]);
                break 'b_1c759c;
            }
        }
        c.cycles += 10;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEPopupConfirm_LayoutFrame(self)` (0x1c7c20): position the popup's
/// frame: compute the centred x (`GetCenteredX + self+0x26c`) and y
/// (`self+0x270`), rescale and position the background, then shift the stored
/// positions (`+0x44` vec3) of the widgets at `+0x6c`/`+0x70` by (x, y) and of
/// the text widgets at `+0x74`, `+0x7c`, `+0x78` by (x, y) (x set to
/// `centred + 320.0` when `+0x2cc` is set, else added).
pub fn c_fe_popup_confirm_layout_frame(c: &mut Cpu) {
    if c.pc != 0x001c7c20 {
        return ee::s03::f_001c7c20__generated(c);
    }
    guest(c, |c| {
        'b_1c7cf4: {
            c.cycles += 33;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[RA][0] = sx(0x001c7c3c);
            c.g[S0][0] = c.g[A0][0];
            Callee {
                addr: 0x001c7be0,
                f: ee::s03::cFEPopupConfirm_GetCenteredX,
            }
            .call(c, 0x001c7c3c)?;
            c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x26c));
            c.g[A1][0] = c.g[SP][0];
            c.lwc1(21, (c.g[S0][0] as u32).wrapping_add(0x270));
            c.g[A0][0] = c.g[S0][0];
            c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x267));
            c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x260));
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x268));
            c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.sdr(2, c.g[SP][0] as u32);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
            c.g[RA][0] = sx(0x001c7c6c);
            c.add_s(20, 0, 1);
            Callee {
                addr: 0x001c7620,
                f: ee::s03::cFEPopupConfirm_ComputeBackgroundScale,
            }
            .call(c, 0x001c7c6c)?;
            c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x26c));
            c.g[A0][0] = c.g[S0][0];
            c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x267));
            c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x260));
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x268));
            c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.sdr(2, c.g[SP][0] as u32);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
            c.g[A1][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x001c7c98);
            c.mov_s(13, 21);
            Callee {
                addr: 0x001c78c0,
                f: ee::s03::cFEPopupConfirm_PositionBackground,
            }
            .call(c, 0x001c7c98)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                break 'b_1c7cf4;
            } else {
                c.cycles += 20;
                c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(3, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(3, c.g[SP][0] as u32);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x26c));
                c.lwc1(1, c.g[SP][0] as u32);
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.add_s(1, 1, 0);
                c.add_s(2, 2, 21);
                c.swc1(1, c.g[SP][0] as u32);
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(7, c.g[SP][0] as u32);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                break 'b_1c7cf4;
            }
        }
        'b_1c7d4c: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                break 'b_1c7d4c;
            } else {
                c.cycles += 20;
                c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(6, c.g[SP][0] as u32);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x26c));
                c.lwc1(1, c.g[SP][0] as u32);
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.add_s(1, 1, 0);
                c.add_s(2, 2, 21);
                c.swc1(1, c.g[SP][0] as u32);
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(5, c.g[SP][0] as u32);
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A2][0]);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                break 'b_1c7d4c;
            }
        }
        'b_1c7dbc: {
            c.cycles += 2;
            if c.g[V1][0] == 0u64 {
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                break 'b_1c7dbc;
            } else {
                'b_1c7d8c: {
                    'b_1c7d88: {
                        c.cycles += 9;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2cc));
                        c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x4b));
                        c.ldr(4, (c.g[V1][0] as u32).wrapping_add(0x44));
                        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                        c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.sdr(4, c.g[SP][0] as u32);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                        if c.g[V0][0] == 0u64 {
                            c.lwc1(0, c.g[SP][0] as u32);
                            break 'b_1c7d88;
                        } else {
                            c.cycles += 4;
                            c.g[AT][0] = 0x43a00000;
                            c.mtc1(1, 0);
                            let t = true;
                            c.add_s(0, 20, 0);
                            if t {
                                break 'b_1c7d8c;
                            } else {
                                break 'b_1c7d88;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.add_s(0, 0, 20);
                    break 'b_1c7d8c;
                }
                c.cycles += 12;
                c.swc1(0, c.g[SP][0] as u32);
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                c.add_s(0, 0, 21);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(3, c.g[SP][0] as u32);
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A0][0]);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                break 'b_1c7dbc;
            }
        }
        'b_1c7e2c: {
            c.cycles += 2;
            if c.g[V1][0] == 0u64 {
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                break 'b_1c7e2c;
            } else {
                'b_1c7dfc: {
                    'b_1c7df8: {
                        c.cycles += 9;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2cc));
                        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x4b));
                        c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x44));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                        c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.sdr(7, c.g[SP][0] as u32);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                        if c.g[V0][0] == 0u64 {
                            c.lwc1(0, c.g[SP][0] as u32);
                            break 'b_1c7df8;
                        } else {
                            c.cycles += 4;
                            c.g[AT][0] = 0x43a00000;
                            c.mtc1(1, 0);
                            let t = true;
                            c.add_s(0, 20, 0);
                            if t {
                                break 'b_1c7dfc;
                            } else {
                                break 'b_1c7df8;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.add_s(0, 0, 20);
                    break 'b_1c7dfc;
                }
                c.cycles += 12;
                c.swc1(0, c.g[SP][0] as u32);
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                c.add_s(0, 0, 21);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(7, c.g[SP][0] as u32);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                break 'b_1c7e2c;
            }
        }
        'b_1c7e9c: {
            c.cycles += 2;
            if c.g[V1][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_1c7e9c;
            } else {
                'b_1c7e6c: {
                    'b_1c7e68: {
                        c.cycles += 9;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2cc));
                        c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x4b));
                        c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x44));
                        c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                        c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.sdr(6, c.g[SP][0] as u32);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                        if c.g[V0][0] == 0u64 {
                            c.lwc1(0, c.g[SP][0] as u32);
                            break 'b_1c7e68;
                        } else {
                            c.cycles += 4;
                            c.g[AT][0] = 0x43a00000;
                            c.mtc1(1, 0);
                            let t = true;
                            c.add_s(0, 20, 0);
                            if t {
                                break 'b_1c7e6c;
                            } else {
                                break 'b_1c7e68;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.add_s(0, 0, 20);
                    break 'b_1c7e6c;
                }
                c.cycles += 12;
                c.swc1(0, c.g[SP][0] as u32);
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                c.add_s(0, 0, 21);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(6, c.g[SP][0] as u32);
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A3][0]);
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_1c7e9c;
            }
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEPopupConfirm_ResetLayout(self)` (0x1c8a38): put the popup's
/// widgets back to their stored layout positions: widgets at `+0x48..+0x60` and
/// `+0x8c` get scale (1,1,1) and the position at `+0x188..`/`+0x200`; `+0x6c`,
/// `+0x70`, `+0x80`, `+0x88` get only their position; the text widgets at
/// `+0x74`, `+0x7c`, `+0x78` get their position and `SetWidgetAlign(9)` (the
/// generated code passes `+0x74` for all three); the option widgets at
/// `+0x90..` are aligned (10) at half `+0x29c`. Then `ResizeTextWidgets` with
/// the global viewport vec (x = 472.0), the scale at `+0x278` is reset to 1 and
/// `AdjustDrawLayer(self, -self+0x314)` runs.
pub fn c_fe_popup_confirm_reset_layout(c: &mut Cpu) {
    if c.pc != 0x001c8a38 {
        return ee::s03::f_001c8a38__generated(c);
    }
    guest(c, |c| {
        'b_1c8aac: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                break 'b_1c8aac;
            } else {
                c.cycles += 19;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(0, c.g[SP][0] as u32);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(3, c.g[SP][0] as u32);
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x57));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x50));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x18f));
                c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x188));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x190));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                break 'b_1c8aac;
            }
        }
        'b_1c8b00: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                break 'b_1c8b00;
            } else {
                c.cycles += 19;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(0, c.g[SP][0] as u32);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(6, c.g[SP][0] as u32);
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x57));
                c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x50));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A3][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                c.ldl(5, (c.g[S0][0] as u32).wrapping_add(0x19b));
                c.ldr(5, (c.g[S0][0] as u32).wrapping_add(0x194));
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x19c));
                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A2][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                break 'b_1c8b00;
            }
        }
        'b_1c8b54: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                break 'b_1c8b54;
            } else {
                c.cycles += 19;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(0, c.g[SP][0] as u32);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(4, c.g[SP][0] as u32);
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x57));
                c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x50));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                c.ldl(3, (c.g[S0][0] as u32).wrapping_add(0x22b));
                c.ldr(3, (c.g[S0][0] as u32).wrapping_add(0x224));
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x22c));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                break 'b_1c8b54;
            }
        }
        'b_1c8ba8: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                break 'b_1c8ba8;
            } else {
                c.cycles += 19;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(0, c.g[SP][0] as u32);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(7, c.g[SP][0] as u32);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x57));
                c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x50));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                c.ldl(6, (c.g[S0][0] as u32).wrapping_add(0x237));
                c.ldr(6, (c.g[S0][0] as u32).wrapping_add(0x230));
                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x238));
                c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A3][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                break 'b_1c8ba8;
            }
        }
        'b_1c8bfc: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                break 'b_1c8bfc;
            } else {
                c.cycles += 19;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(0, c.g[SP][0] as u32);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(5, c.g[SP][0] as u32);
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x57));
                c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x50));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A2][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x243));
                c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x23c));
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x244));
                c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                break 'b_1c8bfc;
            }
        }
        'b_1c8c50: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
                break 'b_1c8c50;
            } else {
                c.cycles += 19;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(0, c.g[SP][0] as u32);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(3, c.g[SP][0] as u32);
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x57));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x50));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x24f));
                c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x248));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x250));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
                break 'b_1c8c50;
            }
        }
        'b_1c8ca4: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8c));
                break 'b_1c8ca4;
            } else {
                c.cycles += 19;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(0, c.g[SP][0] as u32);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(6, c.g[SP][0] as u32);
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x57));
                c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x50));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A3][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
                c.ldl(5, (c.g[S0][0] as u32).wrapping_add(0x25b));
                c.ldr(5, (c.g[S0][0] as u32).wrapping_add(0x254));
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x25c));
                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A2][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8c));
                break 'b_1c8ca4;
            }
        }
        'b_1c8cf8: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                break 'b_1c8cf8;
            } else {
                c.cycles += 19;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(0, c.g[SP][0] as u32);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(4, c.g[SP][0] as u32);
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x57));
                c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x50));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8c));
                c.ldl(3, (c.g[S0][0] as u32).wrapping_add(0x207));
                c.ldr(3, (c.g[S0][0] as u32).wrapping_add(0x200));
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x208));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                break 'b_1c8cf8;
            }
        }
        'b_1c8d1c: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                break 'b_1c8d1c;
            } else {
                c.cycles += 7;
                c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1a7));
                c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1a0));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                break 'b_1c8d1c;
            }
        }
        'b_1c8d40: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                break 'b_1c8d40;
            } else {
                c.cycles += 7;
                c.ldl(6, (c.g[S0][0] as u32).wrapping_add(0x1b3));
                c.ldr(6, (c.g[S0][0] as u32).wrapping_add(0x1ac));
                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b4));
                c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A3][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                break 'b_1c8d40;
            }
        }
        'b_1c8d70: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[S0][0];
            if t {
                break 'b_1c8d70;
            } else {
                c.cycles += 10;
                c.ldl(5, (c.g[S0][0] as u32).wrapping_add(0x1cb));
                c.ldr(5, (c.g[S0][0] as u32).wrapping_add(0x1c4));
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1cc));
                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A2][0]);
                c.mtc1(0, 12);
                c.g[A2][0] = 0x9;
                c.g[RA][0] = sx(0x001c8d70);
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                Callee {
                    addr: 0x001c8930,
                    f: ee::s03::cFEPopupConfirm_SetWidgetAlign,
                }
                .call(c, 0x001c8d70)?;
                break 'b_1c8d70;
            }
        }
        'b_1c8dac: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                break 'b_1c8dac;
            } else {
                c.cycles += 12;
                c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1bf));
                c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1b8));
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A1][0]);
                c.mtc1(0, 12);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                c.g[RA][0] = sx(0x001c8da8);
                c.g[A2][0] = 0x9;
                Callee {
                    addr: 0x001c8930,
                    f: ee::s03::cFEPopupConfirm_SetWidgetAlign,
                }
                .call(c, 0x001c8da8)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                break 'b_1c8dac;
            }
        }
        'b_1c8de4: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                break 'b_1c8de4;
            } else {
                c.cycles += 12;
                c.ldl(3, (c.g[S0][0] as u32).wrapping_add(0x1d7));
                c.ldr(3, (c.g[S0][0] as u32).wrapping_add(0x1d0));
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1d8));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A0][0]);
                c.mtc1(0, 12);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                c.g[RA][0] = sx(0x001c8de0);
                c.g[A2][0] = 0x9;
                Callee {
                    addr: 0x001c8930,
                    f: ee::s03::cFEPopupConfirm_SetWidgetAlign,
                }
                .call(c, 0x001c8de0)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                break 'b_1c8de4;
            }
        }
        'b_1c8e08: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
                break 'b_1c8e08;
            } else {
                c.cycles += 7;
                c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1e3));
                c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1dc));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1e4));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
                break 'b_1c8e08;
            }
        }
        'b_1c8e28: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;

            if t {
                break 'b_1c8e28;
            } else {
                c.cycles += 6;
                c.ldl(6, (c.g[S0][0] as u32).wrapping_add(0x1ef));
                c.ldr(6, (c.g[S0][0] as u32).wrapping_add(0x1e8));
                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f0));
                c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A3][0]);
                break 'b_1c8e28;
            }
        }
        c.cycles += 4;
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 20);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x90));
        c.g[S2][0] = 0x4;
        'l_1c8e38: loop {
            'b_1c8e54: {
                c.cycles += 3;
                c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                let t = c.g[A1][0] == 0u64;
                c.g[A0][0] = c.g[S0][0];
                if t {
                    break 'b_1c8e54;
                } else {
                    c.cycles += 4;
                    c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x29c));
                    c.g[A2][0] = 0xa;
                    c.g[RA][0] = sx(0x001c8e54);
                    c.mul_s(12, 12, 20);
                    Callee {
                        addr: 0x001c8930,
                        f: ee::s03::cFEPopupConfirm_SetWidgetAlign,
                    }
                    .call(c, 0x001c8e54)?;
                    break 'b_1c8e54;
                }
            }
            c.cycles += 3;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
            let t = (c.g[S2][0] as i64) >= 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001c8e38);
                }
                continue 'l_1c8e38;
            } else {
                c.cycles += 36;
                c.g[V0][0] = 0x500000;
                c.g[AT][0] = 0x43ec0000;
                c.mtc1(1, 0);
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf28));
                c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(5, c.g[V1][0] as u32);
                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(5, c.g[SP][0] as u32);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x001c8e98);
                c.swc1(0, c.g[SP][0] as u32);
                Callee {
                    addr: 0x001c70f0,
                    f: ee::s03::cFEPopupConfirm_ResizeTextWidgets,
                }
                .call(c, 0x001c8e98)?;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[A0][0] = c.g[S0][0];
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x17));
                c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x27f));
                c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x278));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x280), c.g[A2][0]);
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x314));
                c.g[RA][0] = sx(0x001c8ed4);
                c.g[A1][0] = sx(0u32.wrapping_sub(c.g[A1][0] as u32));
                Callee {
                    addr: 0x001c9cd0,
                    f: ee::s03::cFEPopupConfirm_AdjustDrawLayer,
                }
                .call(c, 0x001c8ed4)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void cFEPopupConfirm_AdjustDrawLayer(self, delta)` (0x1c9cd0): add `delta`
/// to the 5-bit draw layer (bits 8..12 of the word at `obj+0x14`, wrapping) of
/// each popup widget (`+0x48..+0x68`, `+0x74/+0x7c/+0x78/+0x80/+0x84/+0x8c`),
/// of the three screen objects found by hash name (`screen = self+0x40`) and
/// of the five option widgets at `+0x90`.
pub fn c_fe_popup_confirm_adjust_draw_layer(c: &mut Cpu) {
    if c.pc != 0x001c9cd0 {
        return ee::s03::f_001c9cd0__generated(c);
    }
    guest(c, |c| {
        'b_1c9d18: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S1][0] = c.g[A1][0];
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9d18;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9d18;
            }
        }
        'b_1c9d48: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9d48;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9d48;
            }
        }
        'b_1c9d78: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9d78;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9d78;
            }
        }
        'b_1c9da8: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9da8;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9da8;
            }
        }
        'b_1c9dd8: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9dd8;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9dd8;
            }
        }
        'b_1c9e08: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9e08;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9e08;
            }
        }
        'b_1c9e38: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9e38;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9e38;
            }
        }
        'b_1c9e68: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9e68;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9e68;
            }
        }
        'b_1c9e98: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9e98;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9e98;
            }
        }
        'b_1c9ec8: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9ec8;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9ec8;
            }
        }
        'b_1c9ef8: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9ef8;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9ef8;
            }
        }
        'b_1c9f28: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9f28;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9f28;
            }
        }
        'b_1c9f58: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9f58;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9f58;
            }
        }
        'b_1c9f88: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9f88;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9f88;
            }
        }
        'b_1c9fb8: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8c));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9fb8;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9fb8;
            }
        }
        'b_1ca000: {
            c.cycles += 9;
            c.g[A0][0] = 0x4a0000;
            c.g[RA][0] = sx(0x001c9fc4);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c50));
            call_ptr(c, 0x00317670, 0x001c9fc4)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c9fd0);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c9fd0)?;
            c.g[A1][0] = c.g[V0][0];
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1ca000;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1ca000;
            }
        }
        'b_1ca048: {
            c.cycles += 9;
            c.g[A0][0] = 0x4a0000;
            c.g[RA][0] = sx(0x001ca00c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c58));
            call_ptr(c, 0x00317670, 0x001ca00c)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001ca018);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001ca018)?;
            c.g[A1][0] = c.g[V0][0];
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1ca048;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1ca048;
            }
        }
        'b_1ca090: {
            c.cycles += 9;
            c.g[A0][0] = 0x4a0000;
            c.g[RA][0] = sx(0x001ca054);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x15e0));
            call_ptr(c, 0x00317670, 0x001ca054)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001ca060);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001ca060)?;
            c.g[A1][0] = c.g[V0][0];
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1ca090;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1ca090;
            }
        }
        c.cycles += 4;
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x90));
        c.g[T0][0] = 0xffffffffffffe0ff;
        c.g[A3][0] = 0x4;
        c.g[A1][0] = c.lw(c.g[A2][0] as u32);
        'l_1ca0a0: loop {
            'b_1ca0d0: {
                c.cycles += 3;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
                let t = c.g[A1][0] == 0u64;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                if t {
                    break 'b_1ca0d0;
                } else {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                    c.g[A0][0] = sx((c.g[V0][0] as u32) >> 8);
                    c.g[A0][0] = c.g[A0][0] & 0x1f;
                    c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[V1][0] = c.g[V1][0] & 0x1f;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 8);
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    break 'b_1ca0d0;
                }
            }
            c.cycles += 2;
            if (c.g[A3][0] as i64) >= 0 {
                c.g[A1][0] = c.lw(c.g[A2][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x001ca0a0);
                }
                continue 'l_1ca0a0;
            } else {
                c.cycles += 5;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void cFEPopupConfirm_CheckAndFixTextEntry(self, text, str)` (0x1ca6a0):
/// make a text entry fit its widget: measure `str` and an ellipsis string with
/// the text object's font scale; while the string (minus the ellipsis width and
/// 1.0) is wider than the widget (`text` vtable 0xa0 size), cut one character
/// off a private copy. If anything was cut, append the ellipsis and set the
/// copy as the text, else set the original; free the copy.
///
/// Not in `OVERRIDES`, no test: its callees (`UIText_GetFont`, `Font_MeasureText`,
/// `operator_new_named`, `strcpy`, `strcat`, `UIText_SetString`, `EA_MemFree`)
/// are ported or have no slot, so they run for real in a unit test and walk
/// the allocator (runaway at pc 0x31ef40) on fixture data. Exact otherwise.
#[allow(dead_code)]
pub fn c_fe_popup_confirm_check_and_fix_text_entry(c: &mut Cpu) {
    if c.pc != 0x001ca6a0 {
        return ee::s03::f_001ca6a0__generated(c);
    }
    guest(c, |c| {
        'b_1ca838: {
            c.cycles += 80;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S1][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = c.g[A2][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
            c.g[S5][0] = 0u64;
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.ldl(2, (c.g[S1][0] as u32).wrapping_add(0x57));
            c.ldr(2, (c.g[S1][0] as u32).wrapping_add(0x50));
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x58));
            c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.sdr(2, c.g[SP][0] as u32);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x001ca704);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x001ca704)?;
            c.g[RA][0] = sx(0x001ca70c);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003a04f0, 0x001ca70c)?;
            c.lwc1(2, c.g[SP][0] as u32);
            c.g[A1][0] = 0x460000;
            c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[A2][0] = 0x100;
            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x30));
            c.g[A3][0] = 0u64;
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x34));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x61a0));
            c.mul_s(1, 1, 2);
            c.mul_s(0, 0, 3);
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x38));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x3c));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x328));
            c.g[RA][0] = sx(0x001ca748);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
            call_ptr(c, 0x00317e30, 0x001ca748)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001ca754);
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x00416810, 0x001ca754)?;
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x001ca764);
            c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            call_ptr(c, 0x004166f4, 0x001ca764)?;
            c.g[V1][0] = 0x4a0000;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001ca774);
            c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1bd0));
            call_ptr(c, 0x003a04f0, 0x001ca774)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
            c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
            c.g[A1][0] = c.g[S4][0];
            c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
            c.g[RA][0] = sx(0x001ca790);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00391fb0, 0x001ca790)?;
            c.g[RA][0] = sx(0x001ca798);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003a04f0, 0x001ca798)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
            c.g[RA][0] = sx(0x001ca7b4);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00391fb0, 0x001ca7b4)?;
            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x38));
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 3);
            c.sub_s(0, 0, 1);
            c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x28));
            c.sub_s(0, 0, 3);
            c.c_lt_s(0, 2);
            let t = !c.fcc();
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            if t {
                break 'b_1ca838;
            } else {
                c.cycles += 2;
                c.mov_s(20, 3);
                c.g[S5][0] = 0x1;
                'l_1ca7e8: loop {
                    c.cycles += 20;
                    c.g[A0][0] = c.g[S1][0];
                    c.sb(c.g[V0][0] as u32, 0u64);
                    c.g[RA][0] = sx(0x001ca7f8);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    call_ptr(c, 0x003a04f0, 0x001ca7f8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                    c.g[RA][0] = sx(0x001ca814);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00391fb0, 0x001ca814)?;
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x38));
                    c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x28));
                    c.sub_s(1, 1, 0);
                    c.sub_s(1, 1, 20);
                    c.c_lt_s(1, 2);
                    let t = c.fcc();
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001ca7e8);
                        }
                        continue 'l_1ca7e8;
                    } else {
                        break 'b_1ca838;
                    }
                }
            }
        }
        'b_1ca868: {
            'b_1ca85c: {
                c.cycles += 2;
                let t = c.g[S5][0] == 0u64;
                c.g[A1][0] = c.g[S4][0];
                if t {
                    break 'b_1ca85c;
                } else {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x001ca848);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x004162d0, 0x001ca848)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001ca854);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x003a0c50, 0x001ca854)?;
                    let t = true;

                    if t {
                        break 'b_1ca868;
                    } else {
                        break 'b_1ca85c;
                    }
                }
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001ca868);
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x003a0c50, 0x001ca868)?;
            break 'b_1ca868;
        }
        'b_1ca87c: {
            c.cycles += 2;
            if c.g[S0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                break 'b_1ca87c;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001ca878);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00317e98, 0x001ca878)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                break 'b_1ca87c;
            }
        }
        c.cycles += 9;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        return Ok(());
    });
}

// ---------------------------------------------------------------------------
// Small helpers shared by the functions below (register-level, as the
// generated code does them).

/// The low 32 bits of a register (an address).
pub(crate) fn kbd_r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// `off(reg)` as an address.
pub(crate) fn kbd_ea(c: &Cpu, reg: usize, off: u32) -> u32 {
    kbd_r(c, reg).wrapping_add(off)
}

/// `addiu dst, src, off`.
pub(crate) fn kbd_addi(c: &mut Cpu, dst: usize, src: usize, off: u32) {
    c.g[dst][0] = sx(kbd_ea(c, src, off));
}

/// `lw dst, off(base)`.
pub(crate) fn kbd_lw(c: &mut Cpu, dst: usize, base: usize, off: u32) {
    let a = kbd_ea(c, base, off);
    c.g[dst][0] = c.lw(a);
}

/// `lwc1 f, off(base)`.
pub(crate) fn kbd_lwc1(c: &mut Cpu, f: usize, base: usize, off: u32) {
    let a = kbd_ea(c, base, off);
    c.lwc1(f, a);
}

/// The widget virtual call the compiler emits for `obj->vtbl[slot]`: the
/// vtable is `*(obj + 8)`, each entry is `{i16 delta, fn at +4}` at `slot`.
/// Leaves `v0` (vtable), `a0` (this + delta) and `v1` (function) as the
/// generated code does, then calls it returning to `ret`.
pub(crate) fn kbd_vcall(c: &mut Cpu, obj: u32, slot: u32, ret: u32) -> Flow {
    let ea = obj.wrapping_add(8);
    c.g[V0][0] = c.lw(ea);
    let ea = kbd_ea(c, V0, slot);
    c.g[A0][0] = c.lh(ea);
    let ea = kbd_ea(c, V0, slot + 4);
    c.g[V1][0] = c.lw(ea);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `GetHashValue32` (StrHash).
pub(crate) const KBD_STR_HASH: u32 = 0x0031_7670;

/// `cUIScreen_getObjectByHashName(screen, hash)`.
pub(crate) const KBD_FIND_OBJECT: u32 = 0x0039_d860;

/// `strlen`.
pub(crate) const KBD_STRLEN: u32 = 0x0041_6810;

/// `strncpy`.
pub(crate) const KBD_STRNCPY: u32 = 0x0041_6ce0;

/// `cUIText_getFont`.
pub(crate) const KBD_GET_FONT: u32 = 0x003a_04f0;

/// `Font::Measure(font, str, rect *, flag)`.
pub(crate) const KBD_MEASURE: u32 = 0x0039_1fb0;

/// `cUIText_setAsciiString`.
pub(crate) const KBD_SET_ASCII: u32 = 0x003a_0c50;

pub(crate) fn kbd_apply_font_scale() -> Callee {
    Callee {
        addr: 0x001c_e4c8,
        f: ee::s04::f_001ce4c8,
    }
}

// ---------------------------------------------------------------------------
// cUIStateBuyPopup_OnWidgetCreated

/// Offsets in `cUIStateBuyPopup` (the popup info block).
pub(crate) const BUY_POPUP_ITEM_NAME_TYPE: u32 = 0x54;

pub(crate) const BUY_POPUP_ITEM_TYPE_TYPE: u32 = 0x5c;

pub(crate) const BUY_POPUP_QUESTION_TYPE: u32 = 0x64;

pub(crate) const BUY_POPUP_CREDITS_LEFT: u32 = 0x50;

/// The text-source switch of one string widget: the string type word at
/// `type_off` selects `setAsciiString` (2), `setUnicodeString` (1) or
/// `setUnicodeStringByID` (3) with the value at `ptr_off`; any other type does
/// nothing. `question` is the third string, whose branches are one cycle
/// shorter (the delay slots sit in the same block).
pub(crate) fn buy_popup_string_case(
    c: &mut Cpu,
    frame: Frame,
    type_off: u32,
    ptr_off: u32,
    question: bool,
) -> Flow {
    let slot = if question { 1 } else { 2 };
    c.cycles += 4;
    kbd_lw(c, V1, S0, type_off);
    c.g[V0][0] = 1;
    let ty = c.g[V1][0];
    let (target, ret) = if ty == 2 {
        c.cycles += slot;
        kbd_lw(c, A1, S0, ptr_off);
        (0x003a_0c50, 0x001c_ad78)
    } else {
        c.cycles += 2;
        c.g[V0][0] = 3;
        if ty == 1 {
            c.cycles += slot;
            kbd_lw(c, A1, S0, ptr_off);
            (0x003a_0e90, 0x001c_ad94)
        } else {
            c.cycles += 2;
            if ty != 3 {
                frame.restore(c, S0, 0x40);
                return Ok(());
            }
            c.cycles += slot;
            kbd_lw(c, A1, S0, ptr_off);
            (0x003a_1000, 0x001c_adb0)
        }
    };
    c.cycles += 4;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, target, ret)?;
    frame.restore(c, S0, 0x40);
    Ok(())
}

/// Money answer: `setAsciiString(text, getDollarString(value))`, `value`
/// already in `a0`.
pub(crate) fn buy_popup_dollar_text(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 7;
    call_ptr(c, 0x0019_8af0, 0x001c_ae04)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, KBD_SET_ASCII, 0x001c_ae10)?;
    frame.restore(c, S0, 0x40);
    Ok(())
}

/// `void cUIStateBuyPopup_OnWidgetCreated(cUIStateBuyPopup *this, cUIText *widget)`
/// (0x1cac98): fills a freshly created widget of the buy popup by the hash of
/// its name: the item type, item name and question strings (ascii, wide or
/// by text id), the cost and the money in dollars (cash purchases), the
/// remaining credits (`"%d"`, credit purchases) and hides the group that does
/// not apply.
pub fn c_ui_state_buy_popup_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        c.g[A0][0] = 0x46_0000;
        kbd_lw(c, S1, S2, 0x38);
        kbd_addi(c, A0, A0, 0x62d8);
        call_ptr(c, KBD_STR_HASH, 0x001c_acc4)?;
        'end: {
            // item type
            if c.g[S1][0] == c.g[V0][0] {
                buy_popup_string_case(c, frame, BUY_POPUP_ITEM_TYPE_TYPE, 0x60, false)?;
                break 'end;
            }
            c.g[A0][0] = 0x46_0000;
            // item name
            c.cycles += 4;
            kbd_addi(c, A0, A0, 0x62e8);
            call_ptr(c, KBD_STR_HASH, 0x001c_ad0c)?;
            if c.g[S1][0] == c.g[V0][0] {
                buy_popup_string_case(c, frame, BUY_POPUP_ITEM_NAME_TYPE, 0x58, false)?;
                break 'end;
            }
            c.g[A0][0] = 0x46_0000;
            // question
            c.cycles += 4;
            kbd_addi(c, A0, A0, 0x62f8);
            call_ptr(c, KBD_STR_HASH, 0x001c_ad54)?;
            if c.g[S1][0] == c.g[V0][0] {
                buy_popup_string_case(c, frame, BUY_POPUP_QUESTION_TYPE, 0x68, true)?;
                break 'end;
            }
            c.g[A0][0] = 0x46_0000;
            // cost answer (cash only)
            c.cycles += 4;
            kbd_addi(c, A0, A0, 0x5f0);
            call_ptr(c, KBD_STR_HASH, 0x001c_adc0)?;
            if c.g[S1][0] == c.g[V0][0] {
                c.cycles += 3;
                kbd_lw(c, V0, S0, BUY_POPUP_CREDITS_LEFT);
                if (c.g[V0][0] as i64) > 0 {
                    frame.restore(c, S0, 0x40);
                    break 'end;
                }
                c.cycles += 2;
                kbd_lw(c, A0, S0, 0x48);
                buy_popup_dollar_text(c, frame)?;
                break 'end;
            }
            c.g[A0][0] = 0x46_0000;
            // "you have" answer (cash only)
            c.cycles += 4;
            kbd_addi(c, A0, A0, 0x610);
            call_ptr(c, KBD_STR_HASH, 0x001c_ade4)?;
            if c.g[S1][0] == c.g[V0][0] {
                c.cycles += 3;
                kbd_lw(c, V0, S0, BUY_POPUP_CREDITS_LEFT);
                if (c.g[V0][0] as i64) > 0 {
                    frame.restore(c, S0, 0x40);
                    break 'end;
                }
                c.cycles += 1;
                kbd_lw(c, A0, S0, 0x4c);
                buy_popup_dollar_text(c, frame)?;
                break 'end;
            }
            c.g[A0][0] = 0x46_0000;
            // credits remaining ("%d")
            c.cycles += 4;
            kbd_addi(c, A0, A0, 0x6308);
            call_ptr(c, KBD_STR_HASH, 0x001c_ae20)?;
            if c.g[S1][0] == c.g[V0][0] {
                c.cycles += 3;
                kbd_lw(c, A2, S0, BUY_POPUP_CREDITS_LEFT);
                let none = (c.g[A2][0] as i64) <= 0;
                c.g[A1][0] = 0x4a_0000;
                if none {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x40);
                    break 'end;
                }
                c.cycles += 8;
                c.g[A0][0] = c.g[SP][0];
                kbd_addi(c, A1, A1, 0x12c0);
                call_ptr(c, 0x0041_7828, 0x001c_ae40)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, KBD_SET_ASCII, 0x001c_ae4c)?;
                frame.restore(c, S0, 0x40);
                break 'end;
            }
            c.g[A0][0] = 0x46_0000;
            // group "credit": hidden when paying cash
            c.cycles += 4;
            kbd_addi(c, A0, A0, 0x6318);
            call_ptr(c, KBD_STR_HASH, 0x001c_ae5c)?;
            if c.g[S1][0] == c.g[V0][0] {
                c.cycles += 3;
                kbd_lw(c, V0, S0, BUY_POPUP_CREDITS_LEFT);
                let visible = (c.g[V0][0] as i64) > 0;
                frame.restore(c, S0, 0x40);
                if visible {
                    break 'end;
                }
                c.cycles += 8;
                let widget = kbd_r(c, S2);
                c.g[A1][0] = 0;
                kbd_vcall(c, widget, 0x48, 0x001c_ae88)?;
                frame.restore(c, S0, 0x40);
                break 'end;
            }
            c.g[A0][0] = 0x46_0000;
            // group "cash": hidden when paying with credits
            c.cycles += 4;
            kbd_addi(c, A0, A0, 0x6328);
            call_ptr(c, KBD_STR_HASH, 0x001c_ae98)?;
            if c.g[S1][0] != c.g[V0][0] {
                frame.restore(c, S0, 0x40);
                break 'end;
            }
            c.cycles += 3;
            kbd_lw(c, V0, S0, BUY_POPUP_CREDITS_LEFT);
            let none = (c.g[V0][0] as i64) <= 0;
            c.g[A1][0] = 0;
            if none {
                c.cycles += 1;
                frame.restore(c, S0, 0x40);
                break 'end;
            }
            c.cycles += 5;
            let widget = kbd_r(c, S2);
            kbd_vcall(c, widget, 0x48, 0x001c_aec0)?;
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cKeyboardPopup

/// Offsets in `cKeyboardPopup`.
pub(crate) const KBD_POPUP_SCREEN: u32 = 0x40;

pub(crate) const KBD_POPUP_MAX_LEN: u32 = 0x60;

pub(crate) const KBD_POPUP_MASKED: u32 = 0x70;

pub(crate) const KBD_POPUP_TEXT: u32 = 0x74;

pub(crate) const KBD_POPUP_BUF: u32 = 0xb4;

pub(crate) const KBD_POPUP_MASK_BUF: u32 = 0xf4;

pub(crate) const KBD_POPUP_CURSOR: u32 = 0x134;

pub(crate) const KBD_POPUP_START: u32 = 0x138;

pub(crate) const KBD_POPUP_END: u32 = 0x13c;

pub(crate) const KBD_POPUP_WIDTH: u32 = 0x3f4;

pub(crate) const KBD_POPUP_MARGIN: u32 = 0x400;

/// The tail every re-measure of the scroll window shares: terminate the copied
/// window text at `buf + n` (`s0 + s1`), apply the font scale, fetch the font
/// of the text object (`s3`) and measure the window into the rect on the
/// stack (`sp`).
pub(crate) fn kbd_scroll_measure(
    c: &mut Cpu,
    ret_scale: u32,
    ret_font: u32,
    ret_measure: u32,
) -> Flow {
    c.g[V1][0] = sx(kbd_r(c, S0).wrapping_add(kbd_r(c, S1)));
    c.g[A1][0] = c.g[S3][0];
    let ea = kbd_r(c, V1);
    c.sb(ea, 0);
    c.g[A0][0] = c.g[S2][0];
    kbd_apply_font_scale().call(c, ret_scale)?;
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, KBD_GET_FONT, ret_font)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    kbd_lwc1(c, 12, A0, 0x38);
    c.g[A2][0] = c.g[SP][0];
    kbd_lwc1(c, 13, A0, 0x3c);
    c.g[A3][0] = 0;
    call_ptr(c, KBD_MEASURE, ret_measure)
}

/// `void cKeyboardPopup_UpdateScrollWindow(cKeyboardPopup *this, int dir)`
/// (0x1cc8d8): re-fits the visible window `[start, end)` of the edit text so
/// that the measured text fits the field width (shrinking from the end when
/// `dir` is set, from the start otherwise; then growing while it still
/// fits), copies it to the display buffer, and shows or hides the left and
/// right scroll arrows.
pub fn c_keyboard_popup_update_scroll_window(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S2, 0x80);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        kbd_addi(c, S4, S2, KBD_POPUP_TEXT);
        frame.save(c, S0, 0xa0);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S1, 0x90);
        c.g[A0][0] = c.g[S4][0];
        frame.save(c, S3, 0x70);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 20, 0xb0);
        call_ptr(c, KBD_STRLEN, 0x001c_c91c)?;
        let empty = c.g[V0][0] == 0;
        kbd_addi(c, S0, S2, KBD_POPUP_BUF);
        'fitted: {
            if empty {
                c.cycles += 3;
                let ea = kbd_ea(c, S2, KBD_POPUP_BUF);
                c.sb(ea, 0);
                c.g[S7][0] = 0x46_0000;
                c.g[FP][0] = 0x46_0000;
                break 'fitted;
            }
            c.cycles += 36;
            c.g[A0][0] = 0x46_0000;
            kbd_addi(c, A0, A0, 0x6530);
            c.g[S6][0] = c.g[S0][0];
            call_ptr(c, KBD_STR_HASH, 0x001c_c934)?;
            kbd_lw(c, A0, S2, KBD_POPUP_SCREEN);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, KBD_FIND_OBJECT, 0x001c_c940)?;
            c.g[S3][0] = c.g[V0][0];
            // Copy the current window and measure it.
            kbd_lw(c, V1, S2, KBD_POPUP_END);
            kbd_lw(c, V0, S2, KBD_POPUP_START);
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = sx(kbd_r(c, V1).wrapping_sub(kbd_r(c, V0)));
            c.g[A1][0] = sx(kbd_r(c, V0).wrapping_add(KBD_POPUP_TEXT));
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = sx(kbd_r(c, S2).wrapping_add(kbd_r(c, A1)));
            call_ptr(c, KBD_STRNCPY, 0x001c_c964)?;
            kbd_scroll_measure(c, 0x001c_c978, 0x001c_c980, 0x001c_c99c)?;
            c.g[A0][0] = c.g[S4][0];
            kbd_lwc1(c, 20, S2, KBD_POPUP_WIDTH);
            call_ptr(c, KBD_STRLEN, 0x001c_c9a8)?;
            kbd_lw(c, V1, S2, KBD_POPUP_CURSOR);
            let cursor_not_at_end = c.g[V1][0] != c.g[V0][0];
            kbd_lwc1(c, 0, SP, 8);
            if cursor_not_at_end {
                c.cycles += 1;
                kbd_addi(c, S4, S2, KBD_POPUP_TEXT);
            } else {
                // The cursor is at the end: leave room for the margin.
                c.cycles += 6;
                kbd_lwc1(c, 1, S2, KBD_POPUP_MARGIN);
                c.sub_s(20, 20, 1);
                c.c_lt_s(20, 0);
                let fits = !c.fcc();
                kbd_addi(c, S4, S2, KBD_POPUP_TEXT);
                if !fits {
                    c.cycles += 2;
                    if c.g[S5][0] != 0 {
                        c.add_s(20, 20, 1);
                    } else {
                        c.cycles += 1;
                        kbd_lwc1(c, 0, SP, 8);
                        c.cycles += 1;
                        kbd_addi(c, S4, S2, KBD_POPUP_TEXT);
                    }
                }
            }
            // Shrink while too wide.
            c.cycles += 5;
            c.g[S7][0] = 0x46_0000;
            c.c_lt_s(20, 0);
            let too_wide = c.fcc();
            c.g[FP][0] = 0x46_0000;
            if too_wide {
                c.cycles += 2;
                c.g[S0][0] = c.g[S6][0];
                loop {
                    c.cycles += 2;
                    if c.g[S5][0] == 0 {
                        kbd_lw(c, V0, S2, KBD_POPUP_START);
                        c.cycles += 2;
                        c.g[V0][0] = sx(kbd_r(c, V0).wrapping_add(1));
                        let ea = kbd_ea(c, S2, KBD_POPUP_START);
                        c.sw(ea, c.g[V0][0]);
                    } else {
                        c.cycles += 4;
                        kbd_lw(c, V0, S2, KBD_POPUP_END);
                        c.g[V0][0] = sx(kbd_r(c, V0).wrapping_sub(1));
                        let ea = kbd_ea(c, S2, KBD_POPUP_END);
                        c.sw(ea, c.g[V0][0]);
                    }
                    c.cycles += 26;
                    kbd_lw(c, V1, S2, KBD_POPUP_START);
                    c.g[A0][0] = c.g[S0][0];
                    kbd_lw(c, V0, S2, KBD_POPUP_END);
                    c.g[A1][0] = sx(kbd_r(c, V1).wrapping_add(KBD_POPUP_TEXT));
                    c.g[S1][0] = sx(kbd_r(c, V0).wrapping_sub(kbd_r(c, V1)));
                    c.g[A1][0] = sx(kbd_r(c, S2).wrapping_add(kbd_r(c, A1)));
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, KBD_STRNCPY, 0x001c_ca38)?;
                    kbd_scroll_measure(c, 0x001c_ca4c, 0x001c_ca54, 0x001c_ca70)?;
                    kbd_lwc1(c, 1, SP, 8);
                    c.c_lt_s(20, 1);
                    if c.fcc() {
                        if event_due(c) {
                            deliver_event_here(c, 0x001c_c9f8);
                        }
                        continue;
                    }
                    break;
                }
            }
            // Grow while the window still fits.
            c.cycles += 7;
            kbd_lwc1(c, 0, S2, KBD_POPUP_WIDTH);
            kbd_lwc1(c, 1, S2, KBD_POPUP_MARGIN);
            kbd_lwc1(c, 2, SP, 8);
            c.sub_s(0, 0, 1);
            c.c_lt_s(2, 0);
            if !c.fcc() {
                break 'fitted;
            }
            c.cycles += 6;
            kbd_lw(c, V0, S2, KBD_POPUP_END);
            kbd_lw(c, V1, S2, KBD_POPUP_MAX_LEN);
            c.g[A0][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            let full = c.g[V0][0] == 0;
            c.g[S0][0] = c.g[S6][0];
            if full {
                break 'fitted;
            }
            c.cycles += 1;
            kbd_lw(c, A1, S2, KBD_POPUP_START);
            loop {
                c.cycles += 29;
                c.g[V0][0] = sx(kbd_r(c, A0).wrapping_add(1));
                c.g[A0][0] = c.g[S0][0];
                let ea = kbd_ea(c, S2, KBD_POPUP_END);
                c.sw(ea, c.g[V0][0]);
                c.g[S1][0] = sx(kbd_r(c, V0).wrapping_sub(kbd_r(c, A1)));
                c.g[A1][0] = sx(kbd_r(c, A1).wrapping_add(KBD_POPUP_TEXT));
                c.g[A2][0] = c.g[S1][0];
                c.g[A1][0] = sx(kbd_r(c, S2).wrapping_add(kbd_r(c, A1)));
                call_ptr(c, KBD_STRNCPY, 0x001c_cad8)?;
                kbd_scroll_measure(c, 0x001c_caec, 0x001c_caf4, 0x001c_cb10)?;
                kbd_lwc1(c, 1, S2, KBD_POPUP_WIDTH);
                kbd_lwc1(c, 0, S2, KBD_POPUP_MARGIN);
                kbd_lwc1(c, 2, SP, 8);
                c.sub_s(1, 1, 0);
                c.c_lt_s(2, 1);
                if !c.fcc() {
                    break 'fitted;
                }
                c.cycles += 6;
                kbd_lw(c, V0, S2, KBD_POPUP_END);
                kbd_lw(c, V1, S2, KBD_POPUP_MAX_LEN);
                c.g[A0][0] = c.g[V0][0];
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    kbd_lw(c, A1, S2, KBD_POPUP_START);
                    if event_due(c) {
                        deliver_event_here(c, 0x001c_cab8);
                    }
                    continue;
                }
                c.cycles += 2;
                break 'fitted;
            }
        }
        // Left arrow: shown when the window does not start at 0.
        c.cycles += 23;
        c.g[A0][0] = sx(kbd_r(c, S7).wrapping_add(0x6540));
        call_ptr(c, KBD_STR_HASH, 0x001c_cb60)?;
        kbd_lw(c, A0, S2, KBD_POPUP_SCREEN);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, KBD_FIND_OBJECT, 0x001c_cb6c)?;
        c.g[S3][0] = c.g[V0][0];
        kbd_lw(c, A1, S2, KBD_POPUP_START);
        c.g[A1][0] = (c.g[A1][0] != 0) as u64;
        let left = kbd_r(c, S3);
        kbd_vcall(c, left, 0x48, 0x001c_cb8c)?;
        // Right arrow: shown when the window ends before the text does.
        c.g[A0][0] = sx(kbd_r(c, FP).wrapping_add(0x6558));
        call_ptr(c, KBD_STR_HASH, 0x001c_cb94)?;
        kbd_lw(c, A0, S2, KBD_POPUP_SCREEN);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, KBD_FIND_OBJECT, 0x001c_cba0)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, KBD_STRLEN, 0x001c_cbac)?;
        if c.g[V0][0] == 0 {
            kbd_lw(c, V0, S3, 8);
            c.cycles += 6;
            c.g[A1][0] = 0;
            let right = kbd_r(c, S3);
            kbd_vcall(c, right, 0x48, 0x001c_cbf8)?;
            frame.restore(c, S0, 0xa0);
        } else {
            c.cycles += 12;
            kbd_lw(c, S1, S3, 8);
            c.g[A0][0] = c.g[S4][0];
            let ea = kbd_ea(c, S1, 0x48);
            c.g[S0][0] = c.lh(ea);
            c.g[S0][0] = sx(kbd_r(c, S3).wrapping_add(kbd_r(c, S0)));
            call_ptr(c, KBD_STRLEN, 0x001c_cbc8)?;
            kbd_lw(c, A1, S2, KBD_POPUP_END);
            c.g[A0][0] = c.g[S0][0];
            kbd_lw(c, V1, S1, 0x4c);
            let target = kbd_r(c, V1);
            c.g[A1][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
            call_ptr(c, target, 0x001c_cbdc)?;
            frame.restore(c, S0, 0xa0);
        }
        c.cycles += 12;
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    });
}

/// `void cKeyboardPopup_UpdateCursor(cKeyboardPopup *this)` (0x1ccc30): puts the
/// cursor widget after the visible text (`+0x3e8` is the origin of the
/// text object, the cursor x is the measured width of the characters up to
/// the cursor, `+0x40c` the origin of the cursor widget) and the blink
/// marker after the character under the cursor (`*` when the field is masked).
pub fn c_keyboard_popup_update_cursor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        c.g[S0][0] = 0x46_0000;
        kbd_addi(c, S0, S0, 0x6530);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x90);
        frame.save(c, S2, 0xa0);
        frame.save_ra(c, 0x80);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, KBD_STR_HASH, 0x001c_cc5c)?;
        kbd_lw(c, A0, S1, KBD_POPUP_SCREEN);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, KBD_FIND_OBJECT, 0x001c_cc68)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, KBD_STR_HASH, 0x001c_cc74)?;
        kbd_lw(c, A0, S1, KBD_POPUP_SCREEN);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, KBD_FIND_OBJECT, 0x001c_cc80)?;
        c.g[S0][0] = c.g[V0][0];
        let no_text = c.g[S0][0] == 0;
        kbd_addi(c, A1, S1, KBD_POPUP_WIDTH);
        if !no_text {
            // text->getPos(&this->origin) (vtable slot 20), origin = text->pos.
            c.cycles += 11;
            kbd_lw(c, V0, S0, 8);
            let ea = kbd_ea(c, V0, 0xa0);
            c.g[A0][0] = c.lh(ea);
            kbd_lw(c, V1, V0, 0xa4);
            let target = kbd_r(c, V1);
            c.g[A0][0] = sx(kbd_r(c, S0).wrapping_add(kbd_r(c, A0)));
            call_ptr(c, target, 0x001c_cca0)?;
            let ea = kbd_ea(c, S0, 0x4b);
            c.ldl(V0, ea);
            let ea = kbd_ea(c, S0, 0x44);
            c.ldr(V0, ea);
            kbd_lw(c, V1, S0, 0x4c);
            let ea = kbd_ea(c, S1, 0x3ef);
            c.sdl(V0, ea);
            let ea = kbd_ea(c, S1, 0x3e8);
            c.sdr(V0, ea);
            let ea = kbd_ea(c, S1, 0x3f0);
            c.sw(ea, c.g[V1][0]);
        }
        // cursor y and the character under the cursor.
        c.cycles += 21;
        kbd_lwc1(c, 0, S1, 0x3f8);
        kbd_lwc1(c, 1, S1, 0x3ec);
        kbd_lwc1(c, 2, S1, 0x404);
        c.add_s(0, 0, 1);
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(1, 3);
        kbd_lw(c, V0, S1, KBD_POPUP_CURSOR);
        c.sub_s(0, 0, 2);
        c.g[V0][0] = sx(kbd_r(c, S1).wrapping_add(kbd_r(c, V0)));
        c.sub_s(0, 0, 3);
        let ea = kbd_ea(c, S1, 0x410);
        c.swc1(0, ea);
        let ea = kbd_ea(c, V0, KBD_POPUP_TEXT);
        c.g[A0][0] = c.lbu(ea);
        let ea = kbd_ea(c, V0, KBD_POPUP_TEXT);
        c.g[V1][0] = c.lb(ea);
        let ea = kbd_ea(c, S1, 0x407);
        c.ldl(A2, ea);
        let ea = kbd_ea(c, S1, 0x400);
        c.ldr(A2, ea);
        kbd_lw(c, A3, S1, 0x408);
        let ea = kbd_ea(c, SP, 0x7);
        c.sdl(A2, ea);
        let ea = kbd_ea(c, SP, 0);
        c.sdr(A2, ea);
        let ea = kbd_ea(c, SP, 8);
        c.sw(ea, c.g[A3][0]);
        if c.g[V1][0] != 0 {
            // Measure the character under the cursor (`*` when masked).
            c.cycles += 3;
            kbd_lw(c, V0, S1, KBD_POPUP_MASKED);
            let masked = c.g[V0][0] != 0;
            let ea = kbd_ea(c, SP, 0x21);
            c.sb(ea, 0);
            if masked {
                c.cycles += 3;
                c.g[V0][0] = 0x2a;
                let ea = kbd_ea(c, SP, 0x20);
                c.sb(ea, c.g[V0][0]);
            } else {
                c.cycles += 1;
                let ea = kbd_ea(c, SP, 0x20);
                c.sb(ea, c.g[A0][0]);
            }
            c.cycles += 17;
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = c.g[S1][0];
            kbd_apply_font_scale().call(c, 0x001c_cd34)?;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, KBD_GET_FONT, 0x001c_cd3c)?;
            let ea = kbd_ea(c, SP, 0x20);
            c.g[A3][0] = c.lb(ea);
            c.g[A0][0] = c.g[V0][0];
            kbd_lwc1(c, 12, A0, 0x38);
            kbd_addi(c, A1, SP, 0x20);
            c.g[A3][0] ^= 0x20;
            kbd_lwc1(c, 13, A0, 0x3c);
            c.g[A3][0] = (c.g[A3][0] < 1) as u64;
            kbd_addi(c, A2, SP, 0x10);
            call_ptr(c, KBD_MEASURE, 0x001c_cd60)?;
            let ea = kbd_ea(c, SP, 0x18);
            c.lwc1(1, ea);
            let ea = kbd_ea(c, SP, 0);
            c.swc1(1, ea);
        }
        c.g[A0][0] = 0x46_0000;
        // cursor widget position (vtable slot 21) from the window text.
        c.cycles += 15;
        kbd_addi(c, A0, A0, 0x6570);
        call_ptr(c, KBD_STR_HASH, 0x001c_cd74)?;
        kbd_lw(c, A0, S1, KBD_POPUP_SCREEN);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, KBD_FIND_OBJECT, 0x001c_cd80)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[SP][0];
        let obj = kbd_r(c, S0);
        c.g[V0][0] = c.lw(obj.wrapping_add(8));
        let ea = kbd_ea(c, V0, 0xa8);
        c.g[A0][0] = c.lh(ea);
        let ea = kbd_ea(c, V0, 0xac);
        c.g[V1][0] = c.lw(ea);
        let target = kbd_r(c, V1);
        c.g[A0][0] = sx(obj.wrapping_add(kbd_r(c, A0)));
        call_ptr(c, target, 0x001c_cd9c)?;
        kbd_lw(c, V0, S1, KBD_POPUP_MASKED);
        if c.g[V0][0] == 0 {
            kbd_lw(c, A2, S1, KBD_POPUP_CURSOR);
            c.cycles += 8;
            c.g[V0][0] = sx(kbd_r(c, SP).wrapping_add(0x30));
            kbd_lw(c, V1, S1, KBD_POPUP_START);
            c.g[A0][0] = c.g[V0][0];
            kbd_addi(c, A1, S1, KBD_POPUP_BUF);
            c.g[A2][0] = sx(kbd_r(c, A2).wrapping_sub(kbd_r(c, V1)));
            c.g[S2][0] = c.g[V0][0];
            call_ptr(c, KBD_STRNCPY, 0x001c_cdf8)?;
            kbd_lw(c, V1, S1, KBD_POPUP_START);
        } else {
            c.cycles += 13;
            c.g[A0][0] = c.g[S1][0];
            kbd_addi(c, A1, S1, KBD_POPUP_BUF);
            Callee {
                addr: 0x001c_ce98,
                f: ee::s04::f_001cce98,
            }
            .call(c, 0x001c_cdb4)?;
            kbd_lw(c, A2, S1, KBD_POPUP_CURSOR);
            c.g[V0][0] = sx(kbd_r(c, SP).wrapping_add(0x30));
            kbd_lw(c, V1, S1, KBD_POPUP_START);
            c.g[A0][0] = c.g[V0][0];
            kbd_addi(c, A1, S1, KBD_POPUP_MASK_BUF);
            c.g[S2][0] = c.g[V0][0];
            c.g[A2][0] = sx(kbd_r(c, A2).wrapping_sub(kbd_r(c, V1)));
            call_ptr(c, KBD_STRNCPY, 0x001c_cdd4)?;
            kbd_lw(c, V1, S1, KBD_POPUP_START);
        }
        // Measure the text up to the cursor; the cursor x is its width.
        c.cycles += 39;
        c.g[A1][0] = c.g[S3][0];
        kbd_lw(c, V0, S1, KBD_POPUP_CURSOR);
        c.g[A0][0] = c.g[S1][0];
        c.g[V0][0] = sx(kbd_r(c, V0).wrapping_sub(kbd_r(c, V1)));
        c.g[V0][0] = sx(kbd_r(c, S2).wrapping_add(kbd_r(c, V0)));
        let ea = kbd_r(c, V0);
        c.sb(ea, 0);
        kbd_apply_font_scale().call(c, 0x001c_ce18)?;
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, KBD_GET_FONT, 0x001c_ce20)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        kbd_lwc1(c, 12, A0, 0x38);
        kbd_addi(c, A2, SP, 0x10);
        kbd_lwc1(c, 13, A0, 0x3c);
        c.g[A3][0] = 1;
        call_ptr(c, KBD_MEASURE, 0x001c_ce3c)?;
        let ea = kbd_ea(c, S1, 0x413);
        c.ldl(A2, ea);
        let ea = kbd_ea(c, S1, 0x40c);
        c.ldr(A2, ea);
        kbd_lw(c, A3, S1, 0x414);
        let ea = kbd_ea(c, SP, 0x77);
        c.sdl(A2, ea);
        let ea = kbd_ea(c, SP, 0x70);
        c.sdr(A2, ea);
        let ea = kbd_ea(c, SP, 0x78);
        c.sw(ea, c.g[A3][0]);
        frame.restore(c, S2, 0xa0);
        kbd_lwc1(c, 1, S1, 0x3e8);
        kbd_lwc1(c, 0, SP, 0x18);
        frame.restore(c, S1, 0xb0);
        c.add_s(0, 0, 1);
        frame.restore(c, S3, 0x90);
        frame.restore_ra(c, 0x80);
        let ea = kbd_ea(c, SP, 0x70);
        c.swc1(0, ea);
        let ea = kbd_ea(c, SP, 0x77);
        c.ldl(A0, ea);
        let ea = kbd_ea(c, SP, 0x70);
        c.ldr(A0, ea);
        kbd_lw(c, A1, SP, 0x78);
        let ea = kbd_ea(c, S0, 0x4b);
        c.sdl(A0, ea);
        let ea = kbd_ea(c, S0, 0x44);
        c.sdr(A0, ea);
        let ea = kbd_ea(c, S0, 0x4c);
        c.sw(ea, c.g[A1][0]);
        frame.restore(c, S0, 0xc0);
        frame.close(c);
        Ok(())
    });
}

/// A call of `obj->vtbl[9]` (`setVisible(flag)`) the keyboard popup does when
/// `obj` (in `a2`) is not null: `a1` already holds the flag.
pub(crate) fn kbd_set_visible(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 5;
    let obj = kbd_r(c, A2);
    kbd_vcall(c, obj, 0x48, ret)
}

/// `int cKeyboardPopup_TransitionIn(cKeyboardPopup *this, int first)` (0x1cd348):
/// when shown for the first time resets the edit state (shift flag from the
/// caps state, empty window), lays out the keys, shows the key set of the
/// keyboard type (`+0x43c`: ascii, accents or symbols) and starts the
/// matching screen frame, caches the text and cursor origins of the screen
/// objects, loads the initial text (the text, text id or wide string given
/// before) and places the cursor. Returns 1.
pub fn c_keyboard_popup_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        let not_first = c.g[A1][0] == 0; // `first` is 0: nothing to do
        frame.save_ra(c, 0x20);
        'epilogue: {
            if not_first {
                break 'epilogue;
            }
            c.cycles += 5;
            c.g[S1][0] = 1;
            kbd_lw(c, V0, GP, 0x300);
            let ea = kbd_ea(c, S0, 0x41c);
            c.sw(ea, c.g[S1][0]);
            let no_ctl = c.g[V0][0] == 0;
            let ea = kbd_ea(c, S0, 0x68);
            c.sw(ea, c.g[S1][0]);
            if !no_ctl {
                c.cycles += 4;
                kbd_lw(c, V0, V0, 0x88);
                c.g[V0][0] = sx(kbd_r(c, V0) >> 1);
                c.g[V0][0] &= 1;
                let ea = kbd_ea(c, S0, 0x68);
                c.sw(ea, c.g[V0][0]);
            }
            c.cycles += 3;
            kbd_lw(c, V0, S0, 0x418);
            let has_text = c.g[V0][0] != 0;
            let ea = kbd_ea(c, S0, 0x6c);
            c.sw(ea, 0);
            if !has_text {
                c.cycles += 1;
                let ea = kbd_ea(c, S0, 0x74);
                c.sb(ea, 0);
            }
            c.cycles += 11;
            for off in [0x138, 0x13c] {
                let ea = kbd_ea(c, S0, off);
                c.sw(ea, 0);
            }
            c.g[A0][0] = c.g[S0][0];
            let ea = kbd_ea(c, S0, 0x134);
            c.sw(ea, 0);
            c.g[S2][0] = 0;
            c.g[S4][0] = 0;
            c.g[S3][0] = 0;
            Callee {
                addr: 0x001c_d708,
                f: ee::s04::f_001cd708,
            }
            .call(c, 0x001c_d3c0)?;
            // Key set by keyboard type: 0 symbols (s4), 2 accents (s3), else
            // ascii (s2); the matching name picks the screen frame.
            kbd_lw(c, V1, S0, 0x43c);
            let ty = c.g[V1][0];
            c.g[V0][0] = ((ty as i64) < 2) as u64;
            let ascii_default;
            if ty == c.g[S1][0] {
                c.cycles += 1;
                c.g[A1][0] = 0x4a_0000;
                ascii_default = true;
                c.cycles += 1;
                c.g[S2][0] = 1;
            } else {
                c.cycles += 2;
                let ge2 = c.g[V0][0] == 0;
                c.g[V0][0] = 2;
                if ge2 {
                    c.cycles += 2;
                    let accents = ty == 2;
                    c.g[A1][0] = 0x4a_0000;
                    if accents {
                        c.cycles += 3;
                        c.g[A1][0] = 0x46_0000;
                        c.g[S3][0] = 1;
                        kbd_addi(c, A1, A1, 0x6700);
                        ascii_default = false;
                    } else {
                        c.cycles += 2;
                        c.g[S2][0] = 1;
                        ascii_default = true;
                    }
                } else {
                    c.cycles += 2;
                    let symbols = ty == 0;
                    c.g[A1][0] = 0x4a_0000;
                    if symbols {
                        c.cycles += 3;
                        c.g[S4][0] = 1;
                        kbd_addi(c, A1, A1, 0x1cd0);
                        ascii_default = false;
                    } else {
                        c.cycles += 2;
                        c.g[A1][0] = 0x4a_0000;
                        c.cycles += 1;
                        c.g[S2][0] = 1;
                        ascii_default = true;
                    }
                }
            }
            if ascii_default {
                c.cycles += 4;
                kbd_addi(c, A1, A1, 0x1cd8);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x0041_7828, 0x001c_d430)?;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x0041_7828, 0x001c_d414)?;
            }
            // Show the key sets: objects at +0x48 (ascii), +0x50 (accents),
            // +0x4c (symbols).
            kbd_lw(c, A2, S0, 0x48);
            c.cycles += 2;
            c.g[A1][0] = c.g[S2][0];
            if c.g[A2][0] != 0 {
                kbd_set_visible(c, 0x001c_d450)?;
            }
            c.cycles += 3;
            kbd_lw(c, A2, S0, 0x50);
            c.g[A1][0] = c.g[S3][0];
            if c.g[A2][0] != 0 {
                kbd_set_visible(c, 0x001c_d470)?;
            }
            c.cycles += 3;
            kbd_lw(c, A2, S0, 0x4c);
            c.g[A1][0] = c.g[S4][0];
            if c.g[A2][0] != 0 {
                kbd_set_visible(c, 0x001c_d490)?;
            }
            // Play the screen frame for the keyboard type.
            c.cycles += 9;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, KBD_STR_HASH, 0x001c_d498)?;
            kbd_lw(c, A0, S0, KBD_POPUP_SCREEN);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_c778, 0x001c_d4a4)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = 0xffff;
            let no_frame = c.g[A1][0] == c.g[V0][0];
            c.g[A1][0] &= 0xffff;
            if !no_frame {
                c.cycles += 3;
                kbd_lw(c, A0, S0, KBD_POPUP_SCREEN);
                c.g[A2][0] = 1;
                call_ptr(c, 0x0039_c870, 0x001c_d4c0)?;
            }
            // Cursor widget origin -> this+0x40c.
            c.cycles += 9;
            c.g[A0][0] = 0x46_0000;
            kbd_addi(c, A0, A0, 0x6570);
            call_ptr(c, KBD_STR_HASH, 0x001c_d4cc)?;
            kbd_lw(c, A0, S0, KBD_POPUP_SCREEN);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, KBD_FIND_OBJECT, 0x001c_d4d8)?;
            c.g[S1][0] = c.g[V0][0];
            let none = c.g[S1][0] == 0;
            kbd_addi(c, A1, S0, KBD_POPUP_MARGIN);
            if !none {
                c.cycles += 11;
                let obj = kbd_r(c, S1);
                c.g[V0][0] = c.lw(obj.wrapping_add(8));
                let ea = kbd_ea(c, V0, 0xa0);
                c.g[A0][0] = c.lh(ea);
                kbd_lw(c, V1, V0, 0xa4);
                let target = kbd_r(c, V1);
                c.g[A0][0] = sx(obj.wrapping_add(kbd_r(c, A0)));
                call_ptr(c, target, 0x001c_d4f8)?;
                let ea = kbd_ea(c, S1, 0x4b);
                c.ldl(V0, ea);
                let ea = kbd_ea(c, S1, 0x44);
                c.ldr(V0, ea);
                kbd_lw(c, V1, S1, 0x4c);
                let ea = kbd_ea(c, S0, 0x413);
                c.sdl(V0, ea);
                let ea = kbd_ea(c, S0, 0x40c);
                c.sdr(V0, ea);
                let ea = kbd_ea(c, S0, 0x414);
                c.sw(ea, c.g[V1][0]);
            }
            // Text object origin -> this+0x3e8, and its extent.
            c.cycles += 9;
            c.g[A0][0] = 0x46_0000;
            kbd_addi(c, A0, A0, 0x6530);
            call_ptr(c, KBD_STR_HASH, 0x001c_d51c)?;
            kbd_lw(c, A0, S0, KBD_POPUP_SCREEN);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, KBD_FIND_OBJECT, 0x001c_d528)?;
            c.g[S1][0] = c.g[V0][0];
            let none = c.g[S1][0] == 0;
            kbd_addi(c, A1, S0, KBD_POPUP_WIDTH);
            if !none {
                c.cycles += 15;
                let obj = kbd_r(c, S1);
                c.g[V0][0] = c.lw(obj.wrapping_add(8));
                let ea = kbd_ea(c, V0, 0xa0);
                c.g[A0][0] = c.lh(ea);
                kbd_lw(c, V1, V0, 0xa4);
                let target = kbd_r(c, V1);
                c.g[A0][0] = sx(obj.wrapping_add(kbd_r(c, A0)));
                call_ptr(c, target, 0x001c_d548)?;
                let ea = kbd_ea(c, S1, 0x4b);
                c.ldl(A2, ea);
                let ea = kbd_ea(c, S1, 0x44);
                c.ldr(A2, ea);
                kbd_lw(c, V0, S1, 0x4c);
                let ea = kbd_ea(c, S0, 0x3ef);
                c.sdl(A2, ea);
                let ea = kbd_ea(c, S0, 0x3e8);
                c.sdr(A2, ea);
                let ea = kbd_ea(c, S0, 0x3f0);
                c.sw(ea, c.g[V0][0]);
                kbd_lwc1(c, 0, S0, 0x3f8);
                kbd_lwc1(c, 1, S0, 0x3ec);
                c.add_s(0, 0, 1);
                let ea = kbd_ea(c, S0, 0x410);
                c.swc1(0, ea);
            }
            // Offset object: add its position to the text origin.
            c.cycles += 8;
            c.g[A0][0] = 0x46_0000;
            kbd_addi(c, A0, A0, 0x6710);
            call_ptr(c, KBD_STR_HASH, 0x001c_d57c)?;
            kbd_lw(c, A0, S0, KBD_POPUP_SCREEN);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, KBD_FIND_OBJECT, 0x001c_d588)?;
            if c.g[V0][0] != 0 {
                c.cycles += 19;
                let ea = kbd_ea(c, V0, 0x4b);
                c.ldl(A1, ea);
                let ea = kbd_ea(c, V0, 0x44);
                c.ldr(A1, ea);
                kbd_lw(c, A2, V0, 0x4c);
                let ea = kbd_ea(c, SP, 0x17);
                c.sdl(A1, ea);
                let ea = kbd_ea(c, SP, 0x10);
                c.sdr(A1, ea);
                let ea = kbd_ea(c, SP, 0x18);
                c.sw(ea, c.g[A2][0]);
                kbd_lwc1(c, 3, SP, 0x10);
                kbd_lwc1(c, 4, SP, 0x14);
                kbd_lwc1(c, 5, SP, 0x18);
                kbd_lwc1(c, 0, S0, 0x3e8);
                kbd_lwc1(c, 1, S0, 0x3ec);
                kbd_lwc1(c, 2, S0, 0x3f0);
                c.add_s(0, 0, 3);
                c.add_s(1, 1, 4);
                c.add_s(2, 2, 5);
                for (f, off) in [(0, 0x3e8), (1, 0x3ec), (2, 0x3f0)] {
                    let ea = kbd_ea(c, S0, off);
                    c.swc1(f, ea);
                }
            }
            c.g[A0][0] = 0x46_0000;
            // Hide the two scroll arrows.
            for (hash_off, ret_hash, ret_find, ret_call) in [
                (0x6540, 0x001c_d5e4, 0x001c_d5f0, 0x001c_d610),
                (0x6558, 0x001c_d61c, 0x001c_d628, 0x001c_d648),
            ] {
                c.cycles += if hash_off == 0x6540 { 8 } else { 9 };
                if hash_off != 0x6540 {
                    c.g[A0][0] = 0x46_0000;
                }
                kbd_addi(c, A0, A0, hash_off);
                call_ptr(c, KBD_STR_HASH, ret_hash)?;
                kbd_lw(c, A0, S0, KBD_POPUP_SCREEN);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, KBD_FIND_OBJECT, ret_find)?;
                c.g[A2][0] = c.g[V0][0];
                let none = c.g[A2][0] == 0;
                c.g[A1][0] = 0;
                if !none {
                    kbd_set_visible(c, ret_call)?;
                }
            }
            // Initial text.
            c.cycles += 3;
            kbd_lw(c, V0, S0, 0x418);
            let has_text = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if has_text {
                c.cycles += 2;
                kbd_addi(c, A1, S0, KBD_POPUP_TEXT);
                Callee {
                    addr: 0x001c_b418,
                    f: ee::s04::f_001cb418,
                }
                .call(c, 0x001c_d65c)?;
            }
            c.cycles += 3;
            kbd_lw(c, A1, S0, 0x42c);
            if c.g[A1][0] != 0 {
                // A text id: set it and clear it.
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001c_d020,
                    f: ee::s04::f_001cd020,
                }
                .call(c, 0x001c_d670)?;
                let ea = kbd_ea(c, S0, 0x42c);
                c.sw(ea, 0);
            } else {
                kbd_lw(c, A1, S0, 0x428);
                c.cycles += 2;
                if c.g[A1][0] != 0 {
                    // A wide string: set it, then free it.
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001c_cf70,
                        f: ee::s04::f_001ccf70,
                    }
                    .call(c, 0x001c_d688)?;
                    kbd_lw(c, A0, S0, 0x428);
                    if c.g[A0][0] != 0 {
                        c.cycles += 4;
                        call_ptr(c, 0x0031_7e98, 0x001c_d69c)?;
                    }
                    let ea = kbd_ea(c, S0, 0x428);
                    c.sw(ea, 0);
                } else {
                    kbd_lw(c, A1, S0, 0x424);
                    // A string object: set its text and release it.
                    c.cycles += 3;
                    let ea = kbd_r(c, A1).wrapping_sub(8);
                    c.g[V0][0] = c.lw(ea);
                    let empty = c.g[V0][0] == 0;
                    kbd_addi(c, S1, S0, 0x424);
                    if !empty {
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S0][0];
                        Callee {
                            addr: 0x001c_cf00,
                            f: ee::s04::f_001ccf00,
                        }
                        .call(c, 0x001c_d6b8)?;
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x0031_80b8, 0x001c_d6c0)?;
                    } else {
                        c.cycles += 4;
                        c.g[A1][0] = 0x4a_0000;
                        c.g[A0][0] = c.g[S0][0];
                        kbd_addi(c, A1, A1, 0x1408);
                        Callee {
                            addr: 0x001c_cf00,
                            f: ee::s04::f_001ccf00,
                        }
                        .call(c, 0x001c_d6d8)?;
                    }
                }
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x001c_cc30,
                f: ee::s04::f_001ccc30,
            }
            .call(c, 0x001c_d6e0)?;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x70);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// `void cKeyboardPopup_RefreshKeys(cKeyboardPopup *this)` (0x1cd708): redraws the
/// labels of the 0x54 keys: each key text gets its character (the upper or the
/// lower table by shift and caps, symbols keys always upper; `@` and space
/// entries take their special label), is greyed when the key is disabled,
/// and the key's frame is shown or hidden for the keyboard type; finally the
/// frame of the current cursor key is highlighted.
pub fn c_keyboard_popup_refresh_keys(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S2, 0xa0);
        frame.save(c, S0, 0xc0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0xb0);
        frame.save(c, S3, 0x90);
        frame.save(c, S4, 0x80);
        frame.save(c, S5, 0x70);
        frame.save(c, S6, 0x60);
        frame.save(c, S7, 0x50);
        frame.save(c, FP, 0x40);
        frame.save_ra(c, 0x30);
        kbd_lw(c, V0, S2, KBD_POPUP_SCREEN);
        let no_screen = c.g[V0][0] == 0;
        c.g[V1][0] = 0x44_0000;
        'epilogue: {
            if no_screen {
                break 'epilogue;
            }
            c.cycles += 9;
            c.g[V0][0] = 0x44_0000;
            let ea = kbd_ea(c, SP, 0x21);
            c.sb(ea, 0);
            c.g[FP][0] = sx(kbd_r(c, V0).wrapping_add(0x13e8));
            c.g[S4][0] = sx(kbd_r(c, V1).wrapping_add(0x1390));
            c.g[S3][0] = 0;
            c.g[S7][0] = 0x4c_0000;
            c.g[S6][0] = 0x4c_0000;
            c.g[S5][0] = 0x4c_0000;
            loop {
                'key: {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = 0;
                    key_widget_name().call(c, 0x001c_d77c)?;
                    if c.g[V0][0] == 0 {
                        // No key with this index.
                        c.g[S3][0] = sx(kbd_r(c, S3).wrapping_add(1));
                        break 'key;
                    }
                    // The key's text object.
                    c.cycles += 10;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, KBD_STR_HASH, 0x001c_d78c)?;
                    kbd_lw(c, A0, S2, KBD_POPUP_SCREEN);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, KBD_FIND_OBJECT, 0x001c_d798)?;
                    c.g[S1][0] = c.g[V0][0];
                    let ea = kbd_r(c, S4);
                    c.g[V1][0] = c.lb(ea);
                    c.g[V0][0] = 0x40;
                    let mut special = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x20;
                    if !special {
                        c.cycles += 2;
                        if c.g[V1][0] != c.g[V0][0] {
                            kbd_lw(c, V0, S2, 0x68);
                        } else {
                            special = true;
                        }
                    }
                    if special {
                        // `@` and space: the special key label.
                        c.cycles += 6;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S3][0];
                        Callee {
                            addr: 0x001c_d9a0,
                            f: ee::s04::f_001cd9a0,
                        }
                        .call(c, 0x001c_d7c4)?;
                        c.g[A1][0] = c.g[S3][0];
                    } else {
                        // Upper or lower case by caps and shift.
                        c.cycles += 3;
                        kbd_lw(c, V1, S2, 0x6c);
                        let no_shift = c.g[V1][0] == 0;
                        c.g[V0][0] = (0 < c.g[V0][0]) as u64;
                        if no_shift {
                            c.cycles += 1;
                            c.g[S0][0] = (0 < c.g[V0][0]) as u64;
                        } else {
                            c.cycles += 2;
                            c.g[S0][0] = c.g[V0][0] ^ 1;
                        }
                        c.cycles += 4;
                        c.g[V0][0] = sx(kbd_r(c, S3).wrapping_sub(0xf));
                        c.g[V0][0] = (c.g[V0][0] < 0xc) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[S0][0] = c.g[V1][0] ^ 1;
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x001c_e520,
                            f: ee::s04::f_001ce520,
                        }
                        .call(c, 0x001c_d7fc)?;
                        let lower = c.g[S0][0] == 0;
                        c.g[V0][0] = sx(kbd_r(c, S3).wrapping_add(kbd_r(c, FP)));
                        let sea = kbd_ea(c, SP, 0x20);
                        if lower {
                            c.cycles += 2;
                            let ea = kbd_r(c, V0);
                            c.g[V1][0] = c.lbu(ea);
                            c.sb(sea, c.g[V1][0]);
                        } else {
                            c.cycles += 3;
                            let ea = kbd_r(c, S4);
                            c.g[V0][0] = c.lbu(ea);
                            c.sb(sea, c.g[V0][0]);
                        }
                        // Set the character, grey it when the key is disabled.
                        c.cycles += 2;
                        let text_none = c.g[S1][0] == 0;
                        c.g[A0][0] = c.g[S1][0];
                        if text_none {
                            c.cycles += 1;
                            c.g[A1][0] = c.g[S3][0];
                        } else {
                            c.cycles += 8;
                            kbd_addi(c, A1, SP, 0x20);
                            call_ptr(c, KBD_SET_ASCII, 0x001c_d828)?;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A1][0] = c.g[S3][0];
                            c.g[A2][0] = 0;
                            Callee {
                                addr: 0x001c_e560,
                                f: ee::s04::f_001ce560,
                            }
                            .call(c, 0x001c_d838)?;
                            let enabled = c.g[V0][0] != 0;
                            kbd_lw(c, V0, S1, 8);
                            if enabled {
                                c.cycles += 5;
                                kbd_addi(c, A1, S6, 0x67c8);
                                let obj = kbd_r(c, S1);
                                kbd_vcall(c, obj, 0x58, 0x001c_d870)?;
                                c.cycles += 1;
                                c.g[A1][0] = c.g[S3][0];
                            } else {
                                c.cycles += 7;
                                kbd_addi(c, A1, S7, 0x67b8);
                                let obj = kbd_r(c, S1);
                                kbd_vcall(c, obj, 0x58, 0x001c_d854)?;
                                c.g[A1][0] = c.g[S3][0];
                            }
                        }
                    }
                    // The key's frame object.
                    c.cycles += 12;
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = 1;
                    c.g[A0][0] = c.g[S2][0];
                    key_widget_name().call(c, 0x001c_d884)?;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, KBD_STR_HASH, 0x001c_d88c)?;
                    kbd_lw(c, A0, S2, KBD_POPUP_SCREEN);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, KBD_FIND_OBJECT, 0x001c_d898)?;
                    c.g[S0][0] = c.g[V0][0];
                    let none = c.g[S0][0] == 0;
                    c.g[V0][0] = 1;
                    if none {
                        c.cycles += 1;
                        c.g[S3][0] = sx(kbd_r(c, S3).wrapping_add(1));
                        break 'key;
                    }
                    c.cycles += 3;
                    kbd_lw(c, V1, S2, 0x43c);
                    if c.g[V1][0] != c.g[V0][0] {
                        // Not the ascii keyboard: colour the frame, then show it.
                        let obj = kbd_r(c, S0);
                        c.g[V0][0] = c.lw(obj.wrapping_add(8));
                        c.cycles += 11;
                        kbd_addi(c, A1, S5, 0x67f8);
                        kbd_vcall(c, obj, 0x58, 0x001c_d8f8)?;
                        let obj = kbd_r(c, S0);
                        c.g[V0][0] = c.lw(obj.wrapping_add(8));
                        c.g[A1][0] = 1;
                        kbd_vcall(c, obj, 0x48, 0x001c_d910)?;
                        c.cycles += 1;
                        c.g[S3][0] = sx(kbd_r(c, S3).wrapping_add(1));
                    } else {
                        // Ascii keyboard: only the first 14 keys' frames are hidden.
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[S3][0] < 0xe) as u64;
                        let show = c.g[V0][0] == 0;
                        c.g[A1][0] = 0;
                        c.cycles += 2;
                        if show {
                            c.g[A1][0] = 1;
                        }
                        let obj = kbd_r(c, S0);
                        c.cycles += 6;
                        kbd_vcall(c, obj, 0x48, 0x001c_d8dc)?;
                        c.g[S3][0] = sx(kbd_r(c, S3).wrapping_add(1));
                    }
                }
                // Next key.
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S3][0] as i64) < 84) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S4][0] = sx(kbd_r(c, S4).wrapping_add(1));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x001c_d768);
                    }
                    continue;
                }
                break;
            }
            // Highlight the cursor key.
            c.cycles += 13;
            kbd_lw(c, A1, S2, 0x140);
            c.g[A2][0] = c.g[SP][0];
            c.g[A3][0] = 1;
            c.g[A0][0] = c.g[S2][0];
            key_widget_name().call(c, 0x001c_d934)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, KBD_STR_HASH, 0x001c_d93c)?;
            kbd_lw(c, A0, S2, KBD_POPUP_SCREEN);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, KBD_FIND_OBJECT, 0x001c_d948)?;
            c.g[A2][0] = c.g[V0][0];
            let none = c.g[A2][0] == 0;
            c.g[A1][0] = 0x4c_0000;
            if !none {
                c.cycles += 6;
                let obj = kbd_r(c, A2);
                c.g[V1][0] = c.lw(obj.wrapping_add(8));
                kbd_addi(c, A1, A1, 0x67e8);
                let ea = kbd_ea(c, V1, 0x58);
                c.g[A0][0] = c.lh(ea);
                let ea = kbd_ea(c, V1, 0x5c);
                c.g[V0][0] = c.lw(ea);
                let target = kbd_r(c, V0);
                c.g[A0][0] = sx(obj.wrapping_add(kbd_r(c, A0)));
                call_ptr(c, target, 0x001c_d96c)?;
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S5, 0x70);
        frame.restore(c, S6, 0x60);
        frame.restore(c, S7, 0x50);
        frame.restore(c, FP, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    });
}

pub(crate) fn key_widget_name() -> Callee {
    Callee {
        addr: 0x001c_d0d8,
        f: ee::s04::f_001cd0d8,
    }
}

/// `base + off` as a guest address (32-bit wrap, as the generated code does).
#[inline]
pub(crate) fn at_937(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `cFEPopupConfirm_BaseCtor(this)` (0x1c58e8): the popup's base constructor.
/// Runs the UI-state constructor on `this`, the config constructor on
/// `this+0xbc`, memsets the five confirmation elements at `this+0x2d0`, and
/// clears the pointers, flags and the default colours.
pub fn c_fe_popup_confirm_base_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = sx(at_937(c.g[S0][0], 0x2d0));
        call_ptr(c, 0x0039_e318, 0x001c_5908)?;
        c.g[S2][0] = 4;
        c.g[V1][0] = sx(0x0047_0000u32.wrapping_sub(0x3428));
        c.g[A0][0] = sx(at_937(c.g[S0][0], 0xbc));
        let v1 = c.g[V1][0];
        let this = at_937(c.g[S0][0], 0);
        Addr(this.wrapping_add(8)).sw(c, v1);
        call_ptr(c, 0x001c_55f8, 0x001c_5920)?;
        let this = at_937(c.g[S0][0], 0);
        c.g[V1][0] = Addr(at_937(c.g[GP][0], 0xda0)).lw(c);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        let v0 = c.g[V0][0];
        Addr(this.wrapping_add(0x2c4)).sw(c, v0);
        let v1 = c.g[V1][0];
        Addr(this.wrapping_add(0x2b4)).sw(c, v1);
        Addr(this.wrapping_add(0x2b0)).sw(c, v1);
        for off in [0x2a8u32, 0x2b8, 0x2bc, 0x30c, 0x2c0, 0x314] {
            Addr(this.wrapping_add(off)).sw(c, 0);
        }
        c.g[A0][0] = c.g[S1][0];
        loop {
            c.cycles += 7;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0xc;
            c.g[S1][0] = sx(at_937(c.g[S1][0], 0xc));
            call_ptr(c, 0x0041_6210, 0x001c_5960)?;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[A0][0] = c.g[S1][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001c_5950);
            }
        }
        c.cycles += 23;
        let this = at_937(c.g[S0][0], 0);
        c.g[V0][0] = 1;
        Addr(this.wrapping_add(0x48)).sw(c, 0);
        let v0 = c.g[V0][0];
        Addr(this.wrapping_add(0x310)).sw(c, v0);
        c.g[V1][0] = 4;
        Addr(this.wrapping_add(0x4c)).sw(c, 0);
        c.g[V0][0] = sx(this.wrapping_add(0xa0));
        for off in [
            0x6cu32, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x88, 0x84, 0x8c, 0x64, 0x50, 0x54, 0x58, 0x5c,
            0x60, 0x68,
        ] {
            Addr(this.wrapping_add(off)).sw(c, 0);
        }
        loop {
            c.cycles += 7;
            Addr(c.g[V0][0] as u32).sw(c, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001c_59c8);
            }
        }
        c.cycles += 43;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A1][0] = 0x3f;
        Addr(this.wrapping_add(0x31c)).sw(c, 0);
        c.g[V0][0] = c.g[S0][0];
        let spv = sp(c);
        c.swc1(0, spv.wrapping_add(8));
        c.swc1(0, spv);
        c.swc1(0, spv.wrapping_add(4));
        frame.restore(c, S1, 0x30);
        c.ldl(V1, spv.wrapping_add(7));
        c.ldr(V1, spv);
        c.g[A0][0] = Addr(spv.wrapping_add(8)).lw(c);
        c.sdl(V1, this.wrapping_add(0x27f));
        c.sdr(V1, this.wrapping_add(0x278));
        let a0 = c.g[A0][0];
        Addr(this.wrapping_add(0x280)).sw(c, a0);
        Addr(this.wrapping_add(0x2c8)).sw(c, 0);
        c.g[V1][0] = 0x4c_0000;
        Addr(this.wrapping_add(0x2cc)).sw(c, 0);
        c.g[A3][0] = sx(at_937(c.g[V1][0], 0x6788));
        let a3 = at_937(c.g[A3][0], 0);
        c.ldl(T0, a3.wrapping_add(7));
        c.ldr(T0, a3);
        c.ldl(A0, a3.wrapping_add(0xf));
        c.ldr(A0, a3.wrapping_add(8));
        c.sdl(T0, this.wrapping_add(0x333));
        c.sdr(T0, this.wrapping_add(0x32c));
        c.sdl(A0, this.wrapping_add(0x33b));
        c.sdr(A0, this.wrapping_add(0x334));
        c.g[A0][0] = 0x4c_0000;
        frame.restore(c, S2, 0x20);
        c.g[A3][0] = sx(at_937(c.g[A0][0], 0x6798));
        let a3 = at_937(c.g[A3][0], 0);
        c.ldl(T0, a3.wrapping_add(7));
        c.ldr(T0, a3);
        c.ldl(V1, a3.wrapping_add(0xf));
        c.ldr(V1, a3.wrapping_add(8));
        c.sdl(T0, this.wrapping_add(0x343));
        c.sdr(T0, this.wrapping_add(0x33c));
        c.sdl(V1, this.wrapping_add(0x34b));
        c.sdr(V1, this.wrapping_add(0x344));
        let a1 = c.g[A1][0];
        Addr(this.wrapping_add(0x328)).sw(c, a1);
        frame.restore(c, S0, 0x40);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cFEPopupConfirm_v05(this)` (0x1c5b68): clears the word at `this+0x2b8`
/// and forwards to the base-class slot at 0x39e4a0.
pub fn c_fe_popup_confirm_v05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let v0 = at_937(c.g[V0][0], 0x2b8);
        Addr(v0).sw(c, 0);
        call_ptr(c, 0x0039_e4a0, 0x001c_5b7c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// One block of `cFEPopupConfirm_SetContentVisible`: if the child at `this+field`
/// exists, call its slot 9 (`vt[9](child, visible)`) returning to `ret`.
pub(crate) fn set_content_visible_child_937(c: &mut Cpu, field: u32, ret: u32, head: u64) -> Flow {
    c.cycles += head;
    let this = at_937(c.g[S0][0], 0);
    c.g[A2][0] = Addr(this.wrapping_add(field)).lw(c);
    c.g[A1][0] = c.g[S1][0];
    if c.g[A2][0] == 0 {
        return Ok(());
    }
    c.cycles += 5;
    let child = c.g[A2][0] as u32;
    c.g[V0][0] = Addr(child.wrapping_add(8)).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
    let vt = c.g[V0][0] as u32;
    c.g[V1][0] = Addr(vt.wrapping_add(0x4c)).lw(c);
    c.g[A0][0] = sx(child.wrapping_add(c.g[A0][0] as u32));
    let target = c.g[V1][0] as u32;
    call_ptr(c, target, ret)
}

/// `cFEPopupConfirm_SetContentVisible(this, visible)` (0x1c5e58): passes the
/// visibility flag to the five child popups' slot-9 method.
pub fn c_fe_popup_confirm_set_content_visible(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        set_content_visible_child_937(c, 0x70, 0x001c_5e8c, 0)?;
        set_content_visible_child_937(c, 0x74, 0x001c_5eac, 3)?;
        set_content_visible_child_937(c, 0x78, 0x001c_5ecc, 3)?;
        set_content_visible_child_937(c, 0x7c, 0x001c_5eec, 3)?;
        set_content_visible_child_937(c, 0x88, 0x001c_5f0c, 3)?;
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEPopupConfirm_ComputeBackgroundScale(this, pos)` (0x1c7620): sets the
/// background scale for the popup's background type (online, blue, black).
pub fn c_fe_popup_confirm_compute_background_scale(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = Addr(at_937(c.g[S0][0], 0x31c)).lw(c);
        let src = at_937(c.g[A1][0], 0);
        c.ldl(A0, src.wrapping_add(7));
        c.ldr(A0, src);
        c.g[A2][0] = Addr(src.wrapping_add(8)).lw(c);
        let spv = sp(c);
        c.sdl(A0, spv.wrapping_add(7));
        c.sdr(A0, spv);
        let a2 = c.g[A2][0];
        Addr(spv.wrapping_add(8)).sw(c, a2);
        let t = c.g[V1][0] == c.g[V0][0];
        if t {
            // Type 1 (blue): the 1.05 bump through the float helpers.
            return cbs_blue_937(c, &frame);
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 2;
            c.cycles += 2;
            let is_two = c.g[V1][0] == c.g[V0][0];
            if is_two {
                // Black: scale 1.0.
                cbs_black_937(c);
                return cbs_b_tail_937(c, &frame);
            }
            c.cycles += 2;
            frame.restore(c, S0, 0x20);
            return cbs_epilogue_937(c, &frame);
        }
        c.cycles += 2;
        let is_zero = c.g[V1][0] == 0;
        c.lwc1(1, spv);
        if is_zero {
            // Online: scale from the position and the white background size.
            c.cycles += 14;
            c.lwc1(0, spv.wrapping_add(4));
            c.g[AT][0] = 0x4170_0000;
            c.mtc1(1, 2);
            c.g[AT][0] = 0x4120_0000;
            c.mtc1(1, 3);
            let v = fpr_937(c, 1) + fpr_937(c, 2);
            abi::set_fpr(c, 1, v);
            c.lwc1(4, at_937(c.g[S0][0], 0x1f4));
            let v = fpr_937(c, 0) + fpr_937(c, 3);
            abi::set_fpr(c, 0, v);
            c.lwc1(2, at_937(c.g[S0][0], 0x1f8));
            let v = fpr_937(c, 1).div(fpr_937(c, 4));
            abi::set_fpr(c, 1, v);
            let v = fpr_937(c, 0).div(fpr_937(c, 2));
            abi::set_fpr(c, 0, v);
            c.swc1(1, at_937(c.g[S0][0], 0x278));
            c.swc1(0, at_937(c.g[S0][0], 0x27c));
            return cbs_b_tail_937(c, &frame);
        }
        c.cycles += 2;
        frame.restore(c, S0, 0x20);
        cbs_epilogue_937(c, &frame)
    })
}

/// The `ComputeBackgroundScale` type-1 (blue) block: the scale through the
/// fixed-point helpers at 0x413af8, 0x411fd8 and 0x4127a8.
pub(crate) fn cbs_blue_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 23;
    c.lwc1(12, sp(c));
    c.g[AT][0] = 0x41a0_0000;
    c.mtc1(1, 0);
    c.lwc1(1, at_937(c.g[S0][0], 0x218));
    let v = fpr_937(c, 12) + fpr_937(c, 0);
    abi::set_fpr(c, 12, v);
    let v = fpr_937(c, 12).div(fpr_937(c, 1));
    abi::set_fpr(c, 12, v);
    call_ptr(c, 0x0041_3af8, 0x001c_76d8)?;
    c.g[AT][0] = 0x0046_0000;
    c.g[A1][0] = Addr(at_937(c.g[AT][0], 0x6090)).ld(c);
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0041_1fd8, 0x001c_76e8)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0041_27a8, 0x001c_76f0)?;
    c.lwc1(1, sp(c).wrapping_add(4));
    c.g[AT][0] = 0x4170_0000;
    c.mtc1(1, 2);
    c.lwc1(3, at_937(c.g[S0][0], 0x21c));
    let v = fpr_937(c, 1) + fpr_937(c, 2);
    abi::set_fpr(c, 1, v);
    c.swc1(0, at_937(c.g[S0][0], 0x278));
    let v = fpr_937(c, 1).div(fpr_937(c, 3));
    abi::set_fpr(c, 1, v);
    c.swc1(1, at_937(c.g[S0][0], 0x27c));
    cbs_b_tail_937(c, frame)
}

/// The black (type 2) block: scale 1.0 in both axes.
pub(crate) fn cbs_black_937(c: &mut Cpu) {
    c.cycles += 4;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    c.swc1(0, at_937(c.g[S0][0], 0x27c));
    c.swc1(0, at_937(c.g[S0][0], 0x278));
}

/// The `B` tail: `cycles += 1; lq16`, then the epilogue.
pub(crate) fn cbs_b_tail_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 1;
    frame.restore(c, S0, 0x20);
    cbs_epilogue_937(c, frame)
}

pub(crate) fn cbs_epilogue_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 3;
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// The float at FPU register `r`.
#[inline]
pub(crate) fn fpr_937(c: &Cpu, r: usize) -> FpuF32 {
    FpuF32(c.f[r])
}

/// `cFEPopupConfirm_LayoutTextEntryBox(this)` (0x1c7eb0): places the text entry
/// box and its background from the body text and the two child widgets.
pub fn c_fe_popup_confirm_layout_text_entry_box(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 62;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = sx(at_937(sp(c) as u64, 0x30));
        frame.save_ra(c, 0x60);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        let this = at_937(c.g[S1][0], 0);
        c.g[A2][0] = Addr(this.wrapping_add(0x80)).lw(c);
        c.g[V0][0] = Addr(this.wrapping_add(0x6c)).lw(c);
        c.g[V1][0] = Addr(this.wrapping_add(0x84)).lw(c);
        let a2 = at_937(c.g[A2][0], 0);
        c.ldl(A0, a2.wrapping_add(0x4b));
        c.ldr(A0, a2.wrapping_add(0x44));
        c.g[A3][0] = Addr(a2.wrapping_add(0x4c)).lw(c);
        let spv = sp(c);
        c.sdl(A0, spv.wrapping_add(7));
        c.sdr(A0, spv);
        let a3 = c.g[A3][0];
        Addr(spv.wrapping_add(8)).sw(c, a3);
        let v0 = at_937(c.g[V0][0], 0);
        c.ldl(A0, v0.wrapping_add(0x4b));
        c.ldr(A0, v0.wrapping_add(0x44));
        c.g[A3][0] = Addr(v0.wrapping_add(0x4c)).lw(c);
        c.sdl(A0, spv.wrapping_add(0x27));
        c.sdr(A0, spv.wrapping_add(0x20));
        let a3 = c.g[A3][0];
        Addr(spv.wrapping_add(0x28)).sw(c, a3);
        let v1 = at_937(c.g[V1][0], 0);
        c.ldl(V0, v1.wrapping_add(0x4b));
        c.ldr(V0, v1.wrapping_add(0x44));
        c.g[A0][0] = Addr(v1.wrapping_add(0x4c)).lw(c);
        c.sdl(V0, spv.wrapping_add(0x17));
        c.sdr(V0, spv.wrapping_add(0x10));
        let a0 = c.g[A0][0];
        Addr(spv.wrapping_add(0x18)).sw(c, a0);
        c.g[V0][0] = Addr(a2.wrapping_add(8)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xa0)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xa4)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(a2.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001c_7f34)?;
        c.lwc1(2, spv.wrapping_add(0x10));
        c.g[A1][0] = c.g[S0][0];
        c.lwc1(1, spv);
        c.lwc1(0, at_937(c.g[S1][0], 0x260));
        let v = fpr_937(c, 1) - fpr_937(c, 2);
        abi::set_fpr(c, 1, v);
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(1, 3);
        c.g[A2][0] = Addr(at_937(c.g[S1][0], 0x80)).lw(c);
        let v = fpr_937(c, 0) - fpr_937(c, 1);
        abi::set_fpr(c, 0, v);
        let v = fpr_937(c, 0) - fpr_937(c, 3);
        abi::set_fpr(c, 0, v);
        c.swc1(0, spv.wrapping_add(0x30));
        let a2 = at_937(c.g[A2][0], 0);
        c.g[V0][0] = Addr(a2.wrapping_add(8)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xa8)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xac)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(a2.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001c_7f74)?;
        c.g[AT][0] = 0x40c0_0000;
        c.mtc1(1, 6);
        c.lwc1(0, spv.wrapping_add(0x30));
        c.lwc1(2, at_937(c.g[S1][0], 0x20c));
        let v = fpr_937(c, 0) + fpr_937(c, 6);
        abi::set_fpr(c, 0, v);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        c.g[V0][0] = Addr(at_937(c.g[S1][0], 0x8c)).lw(c);
        c.swc1(1, spv.wrapping_add(0x48));
        let v = fpr_937(c, 0).div(fpr_937(c, 2));
        abi::set_fpr(c, 0, v);
        c.swc1(1, spv.wrapping_add(0x44));
        let is_zero = c.g[V0][0] == 0;
        c.swc1(0, spv.wrapping_add(0x40));
        if is_zero {
            return layout_text_entry_epilogue_937(c, &frame);
        }
        c.cycles += 37;
        c.lwc1(2, at_937(c.g[S1][0], 0x210));
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 4);
        c.ldl(T1, spv.wrapping_add(7));
        c.ldr(T1, spv);
        c.g[V1][0] = Addr(spv.wrapping_add(8)).lw(c);
        c.sdl(T1, spv.wrapping_add(0x57));
        c.sdr(T1, spv.wrapping_add(0x50));
        Addr(spv.wrapping_add(0x58)).sw(c, c.g[V1][0]);
        let v = fpr_937(c, 2) * fpr_937(c, 4);
        abi::set_fpr(c, 2, v);
        c.g[AT][0] = 0x4040_0000;
        c.mtc1(1, 5);
        c.lwc1(0, spv.wrapping_add(0x54));
        c.lwc1(1, spv.wrapping_add(0x30));
        let v = fpr_937(c, 0) + fpr_937(c, 2);
        abi::set_fpr(c, 0, v);
        c.lwc1(3, spv.wrapping_add(0x24));
        let v = fpr_937(c, 1) + fpr_937(c, 6);
        abi::set_fpr(c, 1, v);
        c.lwc1(2, spv.wrapping_add(0x50));
        let v = fpr_937(c, 0) + fpr_937(c, 3);
        abi::set_fpr(c, 0, v);
        let v = fpr_937(c, 1) * fpr_937(c, 4);
        abi::set_fpr(c, 1, v);
        let v = fpr_937(c, 0) - fpr_937(c, 5);
        abi::set_fpr(c, 0, v);
        let v = fpr_937(c, 2) + fpr_937(c, 1);
        abi::set_fpr(c, 2, v);
        c.swc1(0, spv.wrapping_add(0x54));
        c.swc1(2, spv.wrapping_add(0x50));
        c.ldl(A2, spv.wrapping_add(0x57));
        c.ldr(A2, spv.wrapping_add(0x50));
        c.g[A3][0] = Addr(spv.wrapping_add(0x58)).lw(c);
        let dst = at_937(c.g[V0][0], 0);
        c.sdl(A2, dst.wrapping_add(0x4b));
        c.sdr(A2, dst.wrapping_add(0x44));
        Addr(dst.wrapping_add(0x4c)).sw(c, c.g[A3][0]);
        c.g[V0][0] = Addr(at_937(c.g[S1][0], 0x8c)).lw(c);
        c.ldl(V1, spv.wrapping_add(0x47));
        c.ldr(V1, spv.wrapping_add(0x40));
        c.g[A0][0] = Addr(spv.wrapping_add(0x48)).lw(c);
        let dst = at_937(c.g[V0][0], 0);
        c.sdl(V1, dst.wrapping_add(0x57));
        c.sdr(V1, dst.wrapping_add(0x50));
        Addr(dst.wrapping_add(0x58)).sw(c, c.g[A0][0]);
        layout_text_entry_epilogue_937(c, &frame)
    })
}

/// Epilogue of `LayoutTextEntryBox`: `cycles += 5`, restore s0/s1/ra, return.
pub(crate) fn layout_text_entry_epilogue_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 5;
    frame.restore(c, S0, 0x80);
    frame.restore(c, S1, 0x70);
    frame.restore_ra(c, 0x60);
    frame.close(c);
    Ok(())
}

/// `cFEPopupConfirm_DescToString(this, dst, desc)` (0x1c80e8): the popup's
/// description element (kind 0 empty, 1 unicode string, 2 char string, 3 a
/// localised string or a printf of its id) copied into the string `dst`.
pub fn c_fe_popup_confirm_desc_to_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 1;
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        frame.save_ra(c, 0x40);
        c.g[V1][0] = Addr(c.g[S2][0] as u32).lw(c);
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[S3][0] = c.g[A1][0];
        if t {
            return desc_case1_937(c, frame);
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            let is_zero = c.g[V1][0] == 0;
            frame.restore(c, S0, 0x80);
            if is_zero {
                return desc_case0_937(c, frame);
            }
            c.cycles += 2;
            frame.restore(c, S1, 0x70);
            return desc_epilogue_937(c, frame);
        }
        c.g[V0][0] = 2;
        c.cycles += 2;
        let is_two = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 3;
        if is_two {
            return desc_case2_937(c, frame);
        }
        c.cycles += 2;
        let is_three = c.g[V1][0] == c.g[V0][0];
        frame.restore(c, S0, 0x80);
        if is_three {
            return desc_case3_937(c, frame);
        }
        c.cycles += 2;
        frame.restore(c, S1, 0x70);
        desc_epilogue_937(c, frame)
    })
}

/// The `L1` tail of DescToString: `cycles += 1; lq17`, then the epilogue.
pub(crate) fn desc_l1_tail_937(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    frame.restore(c, S1, 0x70);
    desc_epilogue_937(c, frame)
}

/// The epilogue of DescToString: `cycles += 5`, restore s2/s3/ra, return.
pub(crate) fn desc_epilogue_937(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    frame.restore(c, S2, 0x60);
    frame.restore(c, S3, 0x50);
    frame.restore_ra(c, 0x40);
    frame.close(c);
    Ok(())
}

/// Kind 0: the empty string.
pub(crate) fn desc_case0_937(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    c.g[A1][0] = sx(0x004a_1408);
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x0031_84e8, 0x001c_823c)?;
    desc_l1_tail_937(c, frame)
}

/// Kind 2: a char string, copied straight into `dst`.
pub(crate) fn desc_case2_937(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    c.g[A1][0] = Addr(at_937(c.g[S2][0], 4)).lw(c);
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x0031_84e8, 0x001c_8180)?;
    frame.restore(c, S0, 0x80);
    desc_l1_tail_937(c, frame)
}

/// Kind 1: a unicode string converted into a new buffer.
pub(crate) fn desc_case1_937(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 11;
    c.g[A0][0] = Addr(at_937(c.g[S2][0], 4)).lw(c);
    call_ptr(c, 0x003d_bca0, 0x001c_8150)?;
    c.g[A1][0] = 0x0046_0000;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6098));
    c.g[A2][0] = 0x100;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7e30, 0x001c_8168)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = Addr(at_937(c.g[S2][0], 4)).lw(c);
    desc_copy_tail_937(c, frame)
}

/// Kind 3: a localised string through the string table (`vt[4]` of the
/// world's object); when there is none, a printf of its id.
pub(crate) fn desc_case3_937(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 11;
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[A1][0] = Addr(at_937(c.g[S2][0], 4)).lw(c);
    c.g[A2][0] = Addr(at_937(c.g[V0][0], 0x8c)).lw(c);
    c.g[V1][0] = Addr(at_937(c.g[A2][0], 4)).lw(c);
    c.g[A0][0] = Addr(at_937(c.g[V1][0], 0x20)).lh(c);
    c.g[V0][0] = Addr(at_937(c.g[V1][0], 0x24)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001c_81a8)?;
    c.g[S1][0] = c.g[V0][0];
    if c.g[S1][0] == 0 {
        c.g[A2][0] = Addr(at_937(c.g[S2][0], 4)).lw(c);
        // printf the id into a stack buffer, then copy it.
        c.cycles += 9;
        c.g[A1][0] = 0x0046_0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60a8));
        call_ptr(c, 0x0041_7828, 0x001c_8218)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0031_84e8, 0x001c_8224)?;
        frame.restore(c, S0, 0x80);
        return desc_l1_tail_937(c, frame);
    }
    c.cycles += 10;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x003d_bca0, 0x001c_81bc)?;
    c.g[A1][0] = 0x0046_0000;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6098));
    c.g[A2][0] = 0x100;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7e30, 0x001c_81d4)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S1][0];
    desc_copy_tail_937(c, frame)
}

/// The shared tail of kinds 1 and 3: copy the source into the new buffer
/// (`S0`), set `dst` from it, free the buffer, and return.
pub(crate) fn desc_copy_tail_937(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 7;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x002c_2580, 0x001c_81e4)?;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, 0x0031_84e8, 0x001c_81f0)?;
    if c.g[S0][0] != 0 {
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_7e98, 0x001c_8200)?;
    }
    frame.restore(c, S0, 0x80);
    desc_l1_tail_937(c, frame)
}

/// `cFEPopupConfirm_MeasureDesc(self, text, desc, out)` (0x1c8320): scales the
/// text's size vector and measures the description into `out` (width, height).
pub fn c_fe_popup_confirm_measure_desc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x60);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x90);
        c.g[A0][0] = c.g[S1][0];
        frame.save_ra(c, 0x50);
        let text = c.g[S1][0] as u32;
        c.ldl(V0, text.wrapping_add(0x57));
        c.ldr(V0, text.wrapping_add(0x50));
        c.g[V1][0] = Addr(text.wrapping_add(0x58)).lw(c);
        let spv = sp(c);
        c.sdl(V0, spv.wrapping_add(7));
        c.sdr(V0, spv);
        let v1 = c.g[V1][0];
        Addr(spv.wrapping_add(8)).sw(c, v1);
        c.g[RA][0] = sx(0x001c_8364);
        c.g[S3][0] = c.g[A3][0];
        call_ptr(c, 0x003a_04f0, 0x001c_8364)?;
        let g = c.g[V0][0] as u32;
        c.lwc1(2, spv);
        c.g[V1][0] = 1;
        c.lwc1(3, spv.wrapping_add(4));
        c.lwc1(0, g.wrapping_add(0x30));
        c.lwc1(1, g.wrapping_add(0x34));
        let v = fpr_937(c, 0) * fpr_937(c, 2);
        abi::set_fpr(c, 0, v);
        let v = fpr_937(c, 1) * fpr_937(c, 3);
        abi::set_fpr(c, 1, v);
        c.swc1(0, g.wrapping_add(0x38));
        c.swc1(1, g.wrapping_add(0x3c));
        c.g[A0][0] = Addr(c.g[S2][0] as u32).lw(c);
        let t = c.g[A0][0] == c.g[V1][0];
        c.g[V0][0] = ((c.g[A0][0] as i64) < 2) as u64;
        if t {
            return measure_case1_937(c, &frame);
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 2;
        } else {
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            frame.restore(c, S0, 0x90);
            if is_zero {
                return measure_case0_937(c, &frame);
            }
            c.cycles += 2;
            frame.restore(c, S1, 0x80);
            return measure_epilogue_937(c, &frame);
        }
        c.cycles += 2;
        let is_two = c.g[A0][0] == c.g[V0][0];
        c.g[V0][0] = 3;
        if is_two {
            return measure_case2_937(c, &frame);
        }
        c.cycles += 2;
        let is_three = c.g[A0][0] == c.g[V0][0];
        frame.restore(c, S0, 0x90);
        if is_three {
            return measure_case3_937(c, &frame);
        }
        c.cycles += 2;
        frame.restore(c, S1, 0x80);
        measure_epilogue_937(c, &frame)
    })
}

/// The measure epilogue: `cycles += 5`, restore s2/s3/ra, return.
pub(crate) fn measure_epilogue_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 5;
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore_ra(c, 0x50);
    frame.close(c);
    Ok(())
}

/// The `L1` tail of MeasureDesc: `cycles += 1; lq17`, then the epilogue.
pub(crate) fn measure_l1_tail_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 1;
    frame.restore(c, S1, 0x80);
    measure_epilogue_937(c, frame)
}

/// Kind 0: an empty description measures 0 x 0.
pub(crate) fn measure_case0_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 2;
    Addr(at_937(c.g[S3][0], 0xc)).sw(c, 0);
    Addr(at_937(c.g[S3][0], 8)).sw(c, 0);
    measure_l1_tail_937(c, frame)
}

/// Kind 1: a unicode string drawn by the text helper at 0x3921f0.
pub(crate) fn measure_case1_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 5;
    c.g[RA][0] = sx(0x001c_83cc);
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x003a_04f0, 0x001c_83cc)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = Addr(at_937(c.g[S2][0], 4)).lw(c);
    measure_draw1_937(c, frame)
}

/// The kind-1 draw (0x3921f0) and its tail.
pub(crate) fn measure_draw1_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 7;
    c.lwc1(12, at_937(c.g[A0][0], 0x38));
    c.g[A2][0] = c.g[S3][0];
    c.lwc1(13, at_937(c.g[A0][0], 0x3c));
    c.g[A3][0] = 0;
    call_ptr(c, 0x0039_21f0, 0x001c_843c)?;
    frame.restore(c, S0, 0x90);
    measure_l1_tail_937(c, frame)
}

/// Kind 2: a char string drawn by the text helper at 0x391fb0.
pub(crate) fn measure_case2_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 5;
    c.g[RA][0] = sx(0x001c_83e0);
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x003a_04f0, 0x001c_83e0)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = Addr(at_937(c.g[S2][0], 4)).lw(c);
    measure_draw2_937(c, frame)
}

/// The kind-2 draw (0x391fb0) and its tail.
pub(crate) fn measure_draw2_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 7;
    c.lwc1(12, at_937(c.g[A0][0], 0x38));
    c.g[A2][0] = c.g[S3][0];
    c.lwc1(13, at_937(c.g[A0][0], 0x3c));
    c.g[A3][0] = 0;
    call_ptr(c, 0x0039_1fb0, 0x001c_847c)?;
    frame.restore(c, S0, 0x90);
    measure_l1_tail_937(c, frame)
}

/// Kind 3: a localised string through the string table; a printf of the id
/// when the table has none.
pub(crate) fn measure_case3_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 11;
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[A1][0] = Addr(at_937(c.g[S2][0], 4)).lw(c);
    c.g[A2][0] = Addr(at_937(c.g[V0][0], 0x8c)).lw(c);
    c.g[V1][0] = Addr(at_937(c.g[A2][0], 4)).lw(c);
    c.g[A0][0] = Addr(at_937(c.g[V1][0], 0x20)).lh(c);
    c.g[V0][0] = Addr(at_937(c.g[V1][0], 0x24)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[RA][0] = sx(0x001c_840c);
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001c_840c)?;
    c.g[S0][0] = c.g[V0][0];
    if c.g[S0][0] == 0 {
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        return measure_sprintf_937(c, frame);
    }
    c.cycles += 4;
    c.g[RA][0] = sx(0x001c_8420);
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x003a_04f0, 0x001c_8420)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    measure_draw1_937(c, frame)
}

/// The printf path of kind 3: format the id into the stack buffer at `S0`,
/// then draw it.
pub(crate) fn measure_sprintf_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 9;
    c.g[A2][0] = Addr(at_937(c.g[S2][0], 4)).lw(c);
    c.g[A1][0] = 0x0046_0000;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60a8));
    call_ptr(c, 0x0041_7828, 0x001c_8458)?;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x003a_04f0, 0x001c_8460)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    measure_draw2_937(c, frame)
}

/// `cFEPopupConfirm_LayoutTitle(this, info)` (0x1c8678): sets the title text
/// from the info element, then lays it out and positions it when the popup
/// has a second line.
pub fn c_fe_popup_confirm_layout_title(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A3][0] = Addr(at_937(c.g[S0][0], 0x7c)).lw(c);
        if c.g[A3][0] == 0 {
            frame.restore(c, S0, 0x30);
            return layout_title_tail_937(c, &frame);
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(at_937(c.g[A1][0], 0xc)).lw(c);
        c.g[S1][0] = sx(at_937(c.g[A1][0], 0xc));
        if c.g[V0][0] == 0 {
            // No info: the text gets an empty value (vt[9](text, 0)).
            c.cycles += 6;
            c.g[V0][0] = Addr(at_937(c.g[A3][0], 8)).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001c_8740);
            c.g[A0][0] = sx(at_937(c.g[A3][0], 0).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001c_8740)?;
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
            return layout_title_tail_937(c, &frame);
        }
        c.cycles += 26;
        c.g[A1][0] = c.g[A3][0];
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, 0x001c_8050, 0x001c_86b0)?;
        c.g[A2][0] = Addr(at_937(c.g[S0][0], 0x7c)).lw(c);
        c.g[A1][0] = 1;
        c.g[V0][0] = Addr(at_937(c.g[A2][0], 8)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at_937(c.g[A2][0], 0).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001c_86cc)?;
        c.g[A1][0] = Addr(at_937(c.g[S0][0], 0x7c)).lw(c);
        c.g[A3][0] = c.g[SP][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001c_8320, 0x001c_86e0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(at_937(c.g[S0][0], 0x260));
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x001c_9b68, 0x001c_86f0)?;
        let s0 = at_937(c.g[S0][0], 0);
        c.lwc1(0, s0.wrapping_add(0x264));
        c.g[AT][0] = 0x40a0_0000;
        c.mtc1(1, 1);
        c.g[V0][0] = Addr(s0.wrapping_add(0x2cc)).lw(c);
        let v = fpr_937(c, 0) + fpr_937(c, 1);
        abi::set_fpr(c, 0, v);
        let is_zero = c.g[V0][0] == 0;
        c.swc1(0, s0.wrapping_add(0x264));
        if is_zero {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
            return layout_title_tail_937(c, &frame);
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S0][0];
        c.mtc1(0, 12);
        c.g[A1][0] = Addr(at_937(c.g[A0][0], 0x7c)).lw(c);
        c.g[RA][0] = sx(0x001c_8720);
        c.g[A2][0] = 0x11;
        call_ptr(c, 0x001c_8930, 0x001c_8720)?;
        frame.restore(c, S0, 0x30);
        layout_title_tail_937(c, &frame)
    })
}

/// Epilogue of `LayoutTitle`: `cycles += 4`, restore s1/ra, return.
pub(crate) fn layout_title_tail_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 4;
    frame.restore(c, S1, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// `void cFEPopupConfirm_SetWidgetAlign(void *unused, cWidget *obj, uint8 state, float t)`
/// (0x1c8930; the peak3 fork calls it `cScreenPopup_changePivot`). The word at
/// `obj+0x14` holds the alignment in bits 13..18. When it already equals
/// `state` nothing changes. Otherwise the widget's size comes from its
/// vtable method 20, the new alignment is stored, and the x of the origin
/// (`obj+0x44`) moves by `t` (`size.x / 2` when `t` is zero): right for states
/// 17 and 18, left for 9 and 10. The origin and the alignment word go through
/// the stack frame, as the generated code copies them.
pub fn c_fe_popup_confirm_set_widget_align(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x50);
        c.g[S1][0] = c.g[A2][0] & 0xff;
        frame.save_ra(c, 0x20);
        let obj = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(obj.wrapping_add(0x14)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 13);
        c.g[V0][0] &= 0x3f;
        // mov.s f20, f12
        c.f[20] = c.f[12];
        if c.g[V0][0] != c.g[S1][0] {
            set_widget_align_move(c, frame, obj)?;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    });
}

/// The changing branch of [`c_fe_popup_confirm_set_widget_align`]: calls the
/// widget's size method, stores the alignment word, scales `t` when it is
/// zero, moves the origin's x for the state, and copies the origin and the
/// z word back from the frame.
pub(crate) fn set_widget_align_move(c: &mut Cpu, frame: Frame, obj: u32) -> Flow {
    c.cycles += 23;
    let vt = Addr(obj.wrapping_add(8)).lw(c);
    c.g[V0][0] = vt;
    c.g[A1][0] = c.g[SP][0];
    c.g[A0][0] = Addr((vt as u32).wrapping_add(0xa0)).lh(c);
    c.g[V1][0] = Addr((vt as u32).wrapping_add(0xa4)).lw(c);
    let method = c.g[V1][0] as u32;
    c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
    // vtable method 20 (the widget's size into the frame at sp).
    call_ptr(c, method, 0x001c_8978)?;
    // Read the widget back from s0 after the call, as the generated code does.
    let obj = c.g[S0][0] as u32;

    // The alignment word: clear bits 13..18, insert the new state.
    c.g[A0][0] = Addr(obj.wrapping_add(0x14)).lw(c);
    c.g[V0][0] = 0xffff_ffff_fff8_0000 | 0x1fff;
    c.f[0] = 0; // mtc1 $zero, $f0
    c.g[V1][0] = c.g[S1][0] & 0x3f;
    c.g[A0][0] &= c.g[V0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 13);
    c.c_eq_s(20, 0);
    c.g[A0][0] |= c.g[V1][0];

    // The origin (8 bytes at +0x44) and its z word, through the frame.
    c.ldl(V0, obj.wrapping_add(0x4b));
    c.ldr(V0, obj.wrapping_add(0x44));
    c.g[V1][0] = Addr(obj.wrapping_add(0x4c)).lw(c);
    let at_lo = frame.at(c, 0x17);
    c.sdl(V0, at_lo);
    let at_hi = frame.at(c, 0x10);
    c.sdr(V0, at_hi);
    let at_z = frame.at(c, 0x18);
    let z = c.g[V1][0];
    c.sw(at_z, z);

    // `t` is zero: use size.x / 2 (size.x is the first float of the frame).
    let word = c.g[A0][0];
    Addr(obj.wrapping_add(0x14)).sw(c, word);
    if c.fcc() {
        c.cycles += 4;
        let size_x = frame.at(c, 0);
        c.lwc1(1, size_x);
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 0);
        c.mul_s(20, 1, 0);
    }

    // Move the origin's x for the state (17/18 right, 9/10 left).
    c.cycles += 3;
    let state = c.g[S1][0] as i64;
    if state >= 9 {
        c.cycles += 2;
        let x_slot = frame.at(c, 0x10);
        c.lwc1(0, x_slot);
        let moved = if state < 11 {
            c.cycles += 1;
            c.sub_s(0, 0, 20);
            true
        } else {
            c.cycles += 3;
            if state < 19 {
                c.cycles += 2;
                if state >= 17 {
                    c.cycles += 2;
                    c.add_s(0, 0, 20);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        if moved {
            c.cycles += 1;
            c.swc1(0, x_slot);
        }
    }

    // Copy the origin and z back into the widget.
    c.cycles += 6;
    let lo = frame.at(c, 0x17);
    c.ldl(V0, lo);
    let hi = frame.at(c, 0x10);
    c.ldr(V0, hi);
    let z_at = frame.at(c, 0x18);
    c.g[V1][0] = Addr(z_at).lw(c);
    c.sdl(V0, obj.wrapping_add(0x4b));
    c.sdr(V0, obj.wrapping_add(0x44));
    let z_back = c.g[V1][0];
    Addr(obj.wrapping_add(0x4c)).sw(c, z_back);
    Ok(())
}

/// `void cFEPopupConfirm_OpenConfirmPopup(cFEPopupConfirm *self, void *unused, int idx)`
/// (0x1c9b98; the peak3 fork's `cScreenPopup_createConfirmationPopup`). Allocates
/// a 0x360-byte popup and constructs it (`cFEPopupConfirm_BaseCtor`), keeps it
/// at `self+0x2ac`, sets its two option texts from the hashes of two strings,
/// places it at (30, 40, 0), copies entry `idx` (12 bytes per entry) into it,
/// and calls the popup's setup method.
pub fn c_fe_popup_confirm_open_confirm_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 77;
        let frame = Frame::open(c, 0x60);
        c.g[A3][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[A2][0] = 0x100;
        frame.save(c, S1, 0x40);
        c.g[S3][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = sx(0x0046_0000u32.wrapping_add(0x6110));
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x10);
        // popup = cMemMan_alloc(0x360, D_00466110, 0x100, 0)
        c.g[A0][0] = 0x360;
        call_ptr(c, 0x0031_7d70, 0x001c_9bd0)?;

        // popup = cFEPopupConfirm_BaseCtor(popup, self->0x10, self)
        let self_ = c.g[S1][0] as u32;
        c.g[A1][0] = Addr(self_.wrapping_add(0x10)).lw(c);
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x001c_58e8,
            f: ee::s03::f_001c58e8,
        }
        .call(c, 0x001c_9be0)?;
        c.g[S0][0] = c.g[V0][0];
        let self_ = c.g[S1][0] as u32;
        let popup = c.g[S0][0] as u32;

        let s0_value = c.g[S0][0];
        Addr(self_.wrapping_add(0x2ac)).sw(c, s0_value);
        // Option texts: CfgSetOptionTextId(slots, 0, GetHashValue32(D_0045DCC8))
        // and CfgSetOptionTextId(slots, 1, GetHashValue32(D_0045FCD8)).
        let slots = popup.wrapping_add(0xbc);
        c.g[V0][0] = 2;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_sub(0x2338));
        let count = c.g[V0][0];
        Addr(slots).sw(c, count);
        call_ptr(c, 0x0031_7670, 0x001c_9bfc)?;
        c.g[S2][0] = sx(slots);
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        Callee {
            addr: 0x001c_57c0,
            f: ee::s03::f_001c57c0,
        }
        .call(c, 0x001c_9c10)?;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_sub(0x328));
        call_ptr(c, 0x0031_7670, 0x001c_9c1c)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 1;
        Callee {
            addr: 0x001c_57c0,
            f: ee::s03::f_001c57c0,
        }
        .call(c, 0x001c_9c2c)?;
        let self_ = c.g[S1][0] as u32;
        let popup = c.g[S0][0] as u32;

        // The popup's origin: (30, 40, 0), stored as 12 bytes at +0x158.
        c.g[AT][0] = 0x41f0_0000;
        c.mtc1(AT, 1);
        c.g[A0][0] = 0xc;
        c.g[AT][0] = 0x4220_0000;
        c.mtc1(AT, 0);
        c.mult(0, A0, S3, A0); // a0 = idx * 12
        let s0_at = frame.at(c, 0);
        c.swc1(1, s0_at);
        let s4_at = frame.at(c, 4);
        c.swc1(0, s4_at);
        let s8_at = frame.at(c, 8);
        c.sw(s8_at, 0);
        let p_lo = frame.at(c, 7);
        c.ldl(V0, p_lo);
        let p_hi = frame.at(c, 0);
        c.ldr(V0, p_hi);
        let z_at = frame.at(c, 8);
        c.g[V1][0] = Addr(z_at).lw(c);
        c.sdl(V0, popup.wrapping_add(0x15f));
        c.sdr(V0, popup.wrapping_add(0x158));
        let z = c.g[V1][0];
        Addr(popup.wrapping_add(0x160)).sw(c, z);

        // The entry idx of self: its count and its two fields into the popup.
        c.g[A1][0] = sx(self_.wrapping_add(c.g[A0][0] as u32) as u32);
        c.g[V0][0] = Addr(self_.wrapping_add(0x314)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(self_));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(3));
        let v0 = c.g[V0][0];
        Addr(popup.wrapping_add(0x14c)).sw(c, v0);
        let entry = c.g[A1][0] as u32;
        c.g[V1][0] = Addr(entry.wrapping_add(0x2d8)).lw(c);
        let v1 = c.g[V1][0];
        Addr(popup.wrapping_add(0x168)).sw(c, v1);
        let key = c.g[A0][0] as u32;
        c.ldl(V0, key.wrapping_add(0x2d7));
        c.ldr(V0, key.wrapping_add(0x2d0));
        c.sdl(V0, popup.wrapping_add(0xc7));
        c.sdr(V0, popup.wrapping_add(0xc0));
        let idx = c.g[S3][0];
        Addr(self_.wrapping_add(0x324)).sw(c, idx);

        // Setup call: method 0x18 of the popup manager's vtable (object at self+0x40).
        c.g[V1][0] = Addr(self_.wrapping_add(0x40)).lw(c);
        c.g[A1][0] = Addr(self_.wrapping_add(0x2ac)).lw(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xd0)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0039_f290, 0x001c_9cb0)?;

        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `cKeyboardPopup::IsKeyEnabled(self, key, &val)` (0x1ce560).
pub(crate) const KB_IS_KEY_ENABLED: Callee = Callee {
    addr: 0x001c_e560,
    f: ee::s04::f_001ce560,
};

/// `cKeyboardPopup::FindEnabledInColumnFromBottom(self, -1)` (0x1cc0e0).
pub(crate) const KB_FIND_ENABLED_FROM_BOTTOM: Callee = Callee {
    addr: 0x001c_c0e0,
    f: ee::s04::f_001cc0e0,
};

/// `cKeyboardPopup::BuildKeys(self)` (0x1ce2a8).
pub(crate) const KB_BUILD_KEYS: Callee = Callee {
    addr: 0x001c_e2a8,
    f: ee::s04::f_001ce2a8,
};

/// `cFEPopupConfirm::SetOptionEnabled(self, idx, a)` (0x1ca0f0): the option
/// object at `self + 0x90 + 4 * idx`, when there is one, gets its vtable's
/// slot 8 called with `a` and its slot 25 with 1. Each method is reached
/// through its vtable entry (`lh` delta added to the object, `lw` function).
pub fn fepopup_confirm_set_option_enabled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let f = Frame::open(c, 0x20);
        c.g[A1][0] = sx(r32(c, A1) << 2);
        f.save_ra(c, 0);
        let slot = r32(c, A0).wrapping_add(r32(c, A1));
        c.g[A0][0] = sx(slot);
        f.save(c, S0, 0x10);
        c.g[S0][0] = Addr(slot.wrapping_add(0x90)).lw(c);
        let present = c.g[S0][0] != 0;
        c.g[A1][0] = c.g[A2][0];
        if present {
            c.cycles += 11;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x40)).lh(c);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x44)).lw(c);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            let target = r32(c, V1);
            call_ptr(c, target, 0x001c_a124)?;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
            c.g[A1][0] = 1;
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0xc8)).lh(c);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xcc)).lw(c);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            let target = r32(c, V1);
            call_ptr(c, target, 0x001c_a13c)?;
        }
        c.cycles += 4;
        f.restore(c, S0, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `cFEPopupConfirm::Render` (0x1ca578): forwards to `UIState_Render`.
pub fn fepopup_confirm_render(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x0039_e6b8), 0x001c_a588);
}

/// `cKeyboardPopup::cKeyboardPopup(self, engine, owner, a3, ctrl)` (0x1cb030),
/// the constructor: the `cUIState` base (0x39e318), then the popup's fields
/// (+0x15 the control byte, +0x70 `a3`, +0x424 the text object reset by
/// 0x3180b8, the vtable, the defaults), then `BuildKeys`.
pub fn keyboard_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 50;
        let f = Frame::open(c, 0x50);
        f.save(c, S3, 0x10);
        c.g[S3][0] = c.g[A0][0];
        f.save(c, S1, 0x30);
        f.save(c, S2, 0x20);
        c.g[S1][0] = c.g[T0][0] & 0xff;
        f.save(c, S0, 0x40);
        f.save_ra(c, 0);
        c.g[S2][0] = c.g[A3][0];
        call_ptr(c, 0x0039_e318, 0x001c_b058)?;
        c.g[S0][0] = 1;
        c.g[A2][0] = Addr(r32(c, GP).wrapping_add(0xda0)).lw(c);
        c.g[V0][0] = sx(0x0046_bec8);
        c.g[V1][0] = 0x1d;
        c.g[A1][0] = 0x3f;
        let this = r32(c, S3);
        {
            let a = this.wrapping_add(0x15);
            let v = c.g[S1][0];
            Addr(a).sb(c, v);
        }
        {
            let a = this.wrapping_add(0x70);
            let v = c.g[S2][0];
            Addr(a).sw(c, v);
        }
        c.g[A0][0] = sx(this.wrapping_add(0x424));
        Addr(this.wrapping_add(0x418)).sw(c, 0);
        Addr(this.wrapping_add(0x41c)).sw(c, 0);
        {
            let a = this.wrapping_add(0x420);
            let v = c.g[S0][0];
            Addr(a).sw(c, v);
        }
        Addr(this.wrapping_add(0x64)).sw(c, 0);
        Addr(this.wrapping_add(0x6c)).sw(c, 0);
        {
            let a = this.wrapping_add(0x68);
            let v = c.g[S0][0];
            Addr(a).sw(c, v);
        }
        Addr(this.wrapping_add(0x74)).sb(c, 0);
        Addr(this.wrapping_add(0x134)).sw(c, 0);
        Addr(this.wrapping_add(0x138)).sw(c, 0);
        Addr(this.wrapping_add(0x13c)).sw(c, 0);
        Addr(this.wrapping_add(0x54)).sw(c, 0);
        Addr(this.wrapping_add(0x58)).sw(c, 0);
        {
            let a = this.wrapping_add(0x8);
            let v = c.g[V0][0];
            Addr(a).sw(c, v);
        }
        {
            let a = this.wrapping_add(0x140);
            let v = c.g[V1][0];
            Addr(a).sw(c, v);
        }
        {
            let a = this.wrapping_add(0x424);
            let v = c.g[A2][0];
            Addr(a).sw(c, v);
        }
        {
            let a = this.wrapping_add(0x60);
            let v = c.g[A1][0];
            Addr(a).sw(c, v);
        }
        call_ptr(c, 0x0031_80b8, 0x001c_b0c0)?;
        let this = r32(c, S3);
        c.g[A0][0] = Addr(r32(c, GP).wrapping_add(0x300)).lw(c);
        c.g[V0][0] = 2;
        c.g[V1][0] = 0x2b;
        {
            let a = this.wrapping_add(0x144);
            let v = c.g[V0][0];
            Addr(a).sw(c, v);
        }
        {
            let a = this.wrapping_add(0x434);
            let v = c.g[V1][0];
            Addr(a).sw(c, v);
        }
        {
            let a = this.wrapping_add(0x43c);
            let v = c.g[S0][0];
            Addr(a).sw(c, v);
        }
        Addr(this.wrapping_add(0x428)).sw(c, 0);
        Addr(this.wrapping_add(0x42c)).sw(c, 0);
        Addr(this.wrapping_add(0x430)).sw(c, 0);
        Addr(this.wrapping_add(0x438)).sw(c, 0);
        Addr(this.wrapping_add(0x48)).sw(c, 0);
        Addr(this.wrapping_add(0x4c)).sw(c, 0);
        let has_gp300 = c.g[A0][0] != 0;
        Addr(this.wrapping_add(0x50)).sw(c, 0);
        if has_gp300 {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x88)).lw(c);
            c.g[V0][0] = sx(r32(c, V0) >> 1);
            c.g[V0][0] &= 0x1;
            {
                let a = this.wrapping_add(0x68);
                let v = c.g[V0][0];
                Addr(a).sw(c, v);
            }
        }
        c.cycles += 12;
        c.g[A0][0] = c.g[S3][0];
        KB_BUILD_KEYS.call(c, 0x001c_b110)?;
        let this = r32(c, S3);
        c.g[V0][0] = c.g[S3][0];
        Addr(this.wrapping_add(0x440)).sw(c, 0);
        Addr(this.wrapping_add(0x5c)).sw(c, 0);
        f.restore(c, S0, 0x40);
        f.restore(c, S1, 0x30);
        f.restore(c, S2, 0x20);
        f.restore(c, S3, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// One lookup of `cKeyboardPopup::onWidgetCreate` (0x317670 on the text
/// object at `a0 + add`, compared with the widget's +0x38 word). Returns
/// whether they match.
pub(crate) fn widget_created_lookup(c: &mut Cpu, add: u32, ret_addr: u32) -> Result<bool, Unwound> {
    c.cycles += 5;
    c.g[S0][0] = Addr(r32(c, S1).wrapping_add(0x38)).lw(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(add));
    call_ptr(c, 0x0031_7670, ret_addr)?;
    Ok(c.g[S0][0] == c.g[V0][0])
}

/// The matched arm of the first lookup in `cKeyboardPopup::onWidgetCreate`:
/// two notifications through the state's vtable, then `0x1cb360` when the
/// popup is open. Returns `true` when it already restored `s0` and goes
/// straight to the epilogue.
pub(crate) fn widget_created_matched(c: &mut Cpu, f: &Frame) -> Result<bool, Unwound> {
    c.cycles += 10;
    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
    c.g[A1][0] = 0;
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x48)).lh(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x4c)).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
    call_ptr(c, target, 0x001c_b314)?;
    c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x1680)).lw(c);
    c.g[V0][0] ^= 0x1;
    let skip_second = c.g[V0][0] != 0;
    c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
    if !skip_second {
        c.cycles += 7;
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x48)).lh(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x4c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x001c_b33c)?;
        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
    }
    c.cycles += 3;
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
    let closed = c.g[V1][0] == 0;
    c.g[A1][0] = 0;
    if closed {
        return Ok(false);
    }
    c.cycles += 7;
    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x48)).lh(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x4c)).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
    call_ptr(c, target, 0x001c_b360)?;
    f.restore(c, S0, 0x30);
    Ok(true)
}

/// `cKeyboardPopup::onWidgetCreate(self, widget)` (0x1cb2c8): when the text
/// object of the widget matches one of the three lookups, the matching
/// field of the popup (+0x48, +0x50 or +0x4c) takes the widget.
pub fn keyboard_popup_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let f = Frame::open(c, 0x40);
        f.save(c, S2, 0x10);
        f.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        f.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        f.save(c, S0, 0x30);
        c.g[S0][0] = Addr(r32(c, S1).wrapping_add(0x38)).lw(c);
        c.g[A0][0] = sx(0x0046_6398);
        call_ptr(c, 0x0031_7670, 0x001c_b2f4)?;
        let direct_exit = if c.g[S0][0] == c.g[V0][0] {
            widget_created_matched(c, &f)?
        } else {
            c.g[A0][0] = 0x0046_0000;
            if widget_created_lookup(c, 0x63a8, 0x001c_b374)? {
                c.cycles += 2;
                {
                    let a = r32(c, S2).wrapping_add(0x48);
                    let v = c.g[S1][0];
                    Addr(a).sw(c, v);
                }
            } else {
                c.g[A0][0] = 0x0046_0000;
                if widget_created_lookup(c, 0x63b8, 0x001c_b390)? {
                    c.cycles += 2;
                    {
                        let a = r32(c, S2).wrapping_add(0x50);
                        let v = c.g[S1][0];
                        Addr(a).sw(c, v);
                    }
                } else {
                    c.g[A0][0] = 0x0046_0000;
                    if widget_created_lookup(c, 0x63c8, 0x001c_b3ac)? {
                        {
                            let a = r32(c, S2).wrapping_add(0x4c);
                            let v = c.g[S1][0];
                            Addr(a).sw(c, v);
                        }
                    }
                }
            }
            false
        };
        if !direct_exit {
            c.cycles += 1;
            f.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        f.restore(c, S1, 0x20);
        f.restore(c, S2, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// The `else` arm of one quick-key state of `handleMovementUpFromQuickKeyRow`:
/// `IsKeyEnabled(self, key, &val)` (the key is `ret_addr`'s call). When the
/// key is enabled the state becomes `key`; otherwise `val` decides whether
/// the popup's flag at +0x144 changes, and the enabled key in the column from
/// the bottom is found. Either way the result is 1.
pub(crate) fn move_up_top_key(c: &mut Cpu, key: u32, ret_addr: u32) -> Flow {
    c.g[A1][0] = key as u64;
    c.g[A2][0] = c.g[SP][0];
    KB_IS_KEY_ENABLED.call(c, ret_addr)?;
    let enabled = c.g[V0][0] != 0;
    c.g[V0][0] = key as u64;
    if enabled {
        c.cycles += 2;
        {
            let a = r32(c, S0).wrapping_add(0x140);
            let v = c.g[V0][0];
            Addr(a).sw(c, v);
        }
    } else {
        c.cycles += 3;
        c.g[V0][0] = Addr(sp(c)).lw(c);
        let val_set = c.g[V0][0] != 0;
        c.g[V0][0] = (c.g[S1][0] < 1) as u64;
        if val_set {
            c.cycles += 1;
            {
                let a = r32(c, S0).wrapping_add(0x144);
                let v = c.g[V0][0];
                Addr(a).sw(c, v);
            }
        }
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = u64::MAX;
        KB_FIND_ENABLED_FROM_BOTTOM.call(c, 0x001c_bc24)?;
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    Ok(())
}

/// `cKeyboardPopup::handleMovementUpFromQuickKeyRow` (0x1cbb70): on the
/// quick-key row (popup states 5-12, one key each) the cursor moves to that
/// row's key when it is enabled, else to the enabled key in the column from
/// the bottom. Returns 1 when it handled the move, 0 for any other state.
pub fn keyboard_popup_move_up_from_top_row(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let f = Frame::open(c, 0x40);
        f.save(c, S0, 0x30);
        f.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        f.save_ra(c, 0x10);
        c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x140)).lw(c);
        c.g[V0][0] = (sx(r32(c, V1).wrapping_sub(5)) < 4) as u64;
        c.g[S1][0] = c.g[A1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            move_up_top_key(c, 0x4b, 0x001c_bba4)?;
        } else {
            c.cycles += 4;
            c.g[V0][0] = (sx(r32(c, V1).wrapping_sub(9)) < 2) as u64;
            c.g[A0][0] = c.g[S0][0];
            if c.g[V0][0] != 0 {
                c.cycles += 5;
                move_up_top_key(c, 0x4c, 0x001c_bbd0)?;
            } else {
                c.cycles += 4;
                c.g[V0][0] = (sx(r32(c, V1).wrapping_sub(0xb)) < 2) as u64;
                c.g[A1][0] = 0x4f;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 4;
                    move_up_top_key(c, 0x4f, 0x001c_bbf8)?;
                }
            }
        }
        c.cycles += 5;
        f.restore(c, S0, 0x30);
        f.restore(c, S1, 0x20);
        f.restore_ra(c, 0x10);
        f.close(c);
        Ok(())
    })
}

/// `cKeyboardPopup_IsKeyEnabled(self, key, &out)` (0x1ce560).
pub(crate) const KEY_ENABLED: Callee = Callee {
    addr: 0x001c_e560,
    f: ee::s04::f_001ce560,
};

/// `cKeyboardPopup_FindEnabledInColumnFromBottom(self, row)` (0x1cc0e0).
pub(crate) const FIND_ENABLED_IN_COLUMN_FROM_BOTTOM: Callee = Callee {
    addr: 0x001c_c0e0,
    f: ee::s04::f_001cc0e0,
};

/// `cKeyboardPopup_SetKeyEnabled(self, key, enabled, flags)` (0x1ce3c8).
pub(crate) const SET_KEY_ENABLED: Callee = Callee {
    addr: 0x001c_e3c8,
    f: ee::s04::f_001ce3c8,
};

/// `cKeyboardPopup_SetFunctionKeysEnabled(self, enabled)` (0x1ce468).
pub(crate) const SET_FUNCTION_KEYS_ENABLED: Callee = Callee {
    addr: 0x001c_e468,
    f: ee::s04::f_001ce468,
};

/// Where `BuildKeys` continues after one key: the `'*'`/`0x1c` or `0x45/0x4f/0x29`
/// key paths, the default path, or the strlen-checked `0x4b` key.
#[derive(Clone, Copy)]
pub(crate) enum KeyExit {
    /// Sets A1 = key, then A2 = 0, then the call.
    Star,
    /// Sets A2 = 0, then the call.
    Zero,
    /// The call with the A2 the path chose.
    Call,
    /// The default path: the call with the enabled flag 1 (its own cycles).
    Default,
}

/// `cKeyboardPopup::BuildKeys(this)` (0x1ce2a8): walks the 0x54 keys of the
/// key table at `0x441390`, enabling or disabling each through
/// `SetKeyEnabled(this, key, enabled, 2)`; key `0x4b` is enabled only when its
/// text and value are set. Then `SetFunctionKeysEnabled(this, 0)`.
pub fn keyboard_popup_build_keys(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let f = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x0044_0000;
        f.save(c, 16, 0x90);
        f.save(c, 17, 0x80);
        c.g[S0][0] = c.g[A0][0];
        f.save(c, 18, 0x70);
        c.g[S1][0] = 0;
        f.save(c, 19, 0x60);
        c.g[S2][0] = 0x4b;
        f.save(c, 20, 0x50);
        c.g[S3][0] = 0x29;
        f.save(c, 21, 0x40);
        c.g[S4][0] = 0x4f;
        f.save(c, 22, 0x30);
        c.g[S5][0] = 0x45;
        f.save(c, 23, 0x20);
        c.g[S6][0] = 0x1c;
        f.save(c, 30, 0x10);
        c.g[S7][0] = 0x2a;
        f.save_ra(c, 0);
        c.g[FP][0] = sx(0x0044_0000u32.wrapping_add(0x1390));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[FP][0] as u32));
        let this = c.g[S0][0] as u32;
        loop {
            c.cycles += 3;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lb(c);
            let star = c.g[V1][0] == c.g[S7][0];
            c.g[A0][0] = c.g[S0][0];
            let exit = if star {
                KeyExit::Star
            } else {
                c.cycles += 2;
                let at_1c = c.g[S1][0] == c.g[S6][0];
                c.g[A1][0] = c.g[S1][0];
                if at_1c {
                    KeyExit::Zero
                } else {
                    c.cycles += 2;
                    let at_45 = c.g[S1][0] == c.g[S5][0];
                    c.g[A2][0] = 0;
                    if at_45 {
                        KeyExit::Call
                    } else {
                        c.cycles += 2;
                        if c.g[S1][0] == c.g[S4][0] {
                            KeyExit::Call
                        } else {
                            c.cycles += 2;
                            if c.g[S1][0] == c.g[S3][0] {
                                KeyExit::Call
                            } else {
                                c.cycles += 2;
                                let not_4b = c.g[S1][0] != c.g[S2][0];
                                c.g[A2][0] = 1;
                                if not_4b {
                                    KeyExit::Default
                                } else {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001c_e33c);
                                    c.g[A0][0] = sx(this.wrapping_add(0x74));
                                    call_ptr(c, 0x0041_6810, 0x001c_e33c)?;
                                    let no_name = c.g[V0][0] == 0;
                                    c.g[A0][0] = c.g[S0][0];
                                    if no_name {
                                        KeyExit::Star
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = Addr(this.wrapping_add(0x58)).lw(c);
                                        let no_value = c.g[V0][0] == 0;
                                        c.g[A1][0] = 0x4b;
                                        if no_value {
                                            KeyExit::Star
                                        } else {
                                            c.cycles += 2;
                                            c.g[A2][0] = 1;
                                            KeyExit::Call
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match exit {
                KeyExit::Star => {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[S1][0];
                    c.cycles += 1;
                    c.g[A2][0] = 0;
                    set_key_enabled_call(c)?;
                }
                KeyExit::Zero => {
                    c.cycles += 1;
                    c.g[A2][0] = 0;
                    set_key_enabled_call(c)?;
                }
                KeyExit::Call => set_key_enabled_call(c)?,
                KeyExit::Default => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001c_e378);
                    c.g[A3][0] = 0x2;
                    SET_KEY_ENABLED.call(c, 0x001c_e378)?;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 84) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[FP][0] as u32));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001c_e300);
            }
        }
        c.cycles += 15;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x001c_e394);
        c.g[A1][0] = 0;
        SET_FUNCTION_KEYS_ENABLED.call(c, 0x001c_e394)?;
        f.restore(c, 16, 0x90);
        f.restore(c, 17, 0x80);
        f.restore(c, 18, 0x70);
        f.restore(c, 19, 0x60);
        f.restore(c, 20, 0x50);
        f.restore(c, 21, 0x40);
        f.restore(c, 22, 0x30);
        f.restore(c, 23, 0x20);
        f.restore(c, 30, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// The shared call of `BuildKeys`' per-key paths: `SetKeyEnabled(this, key, A2, 2)`
/// with its own cycles, then the key advances.
pub(crate) fn set_key_enabled_call(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[RA][0] = sx(0x001c_e368);
    c.g[A3][0] = 0x2;
    SET_KEY_ENABLED.call(c, 0x001c_e368)?;
    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
    Ok(())
}

/// `cKeyboardPopup::AppendQuickMessage(this, key)` (0x1ce5e0): fills the
/// quick-message buffer on the stack with the option's text (or the default
/// quick-key message for the key), trims it to the room left after the
/// current text, appends it to `this+0x74`, and records the length in
/// `this+0x134` (clamped to `this+0x60 - 1`).
pub fn keyboard_popup_append_quick_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let f = Frame::open(c, 0x90);
        f.save(c, 18, 0x60);
        f.save(c, 17, 0x70);
        c.g[S2][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        f.save(c, 16, 0x80);
        f.save(c, 19, 0x50);
        f.save_ra(c, 0x40);
        let this = c.g[S2][0] as u32;
        c.g[RA][0] = sx(0x001c_e608);
        c.g[A0][0] = sx(this.wrapping_add(0x74));
        call_ptr(c, 0x0041_6810, 0x001c_e608)?;
        c.g[V1][0] = Addr(this.wrapping_add(0x60)).lw(c);
        c.g[RA][0] = sx(0x001c_e614);
        c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        call_ptr(c, 0x0014_dc80, 0x001c_e614)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001c_e620);
        c.g[A1][0] = 4;
        call_ptr(c, 0x0014_dd58, 0x001c_e620)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001c_e630);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_f6a8, 0x001c_e630)?;
        let buf = sp(c);
        if c.g[V0][0] != 0 {
            c.g[A1][0] = c.g[V0][0];
            // The option text: copy it into the buffer.
            c.cycles += 2;
            c.g[RA][0] = sx(0x001c_e690);
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0041_66f4, 0x001c_e690)?;
            c.cycles += 1;
            c.g[V0][0] = sx(buf.wrapping_add(c.g[S3][0] as u32));
        } else {
            // The default quick-key message of the key.
            c.cycles += 14;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8c)).lw(c);
            c.g[S1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x4)).lw(c);
            c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x20)).lh(c);
            c.g[RA][0] = sx(0x001c_e658);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            call_ptr(c, 0x0014_f678, 0x001c_e658)?;
            c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x24)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001c_e668);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, target, 0x001c_e668)?;
            c.g[A1][0] = c.g[V0][0];
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                Addr(buf).sb(c, 0);
                c.cycles += 1;
                c.g[V0][0] = sx(buf.wrapping_add(c.g[S3][0] as u32));
            } else {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001c_e678);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x003d_cb20, 0x001c_e678)?;
                c.g[V0][0] = sx(buf.wrapping_add(c.g[S3][0] as u32));
            }
        }
        // Trim: the room is the text's cursor, the text is cut at the room.
        c.cycles += 15;
        c.g[S0][0] = sx(this.wrapping_add(0x74));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x001c_e6a8);
        Addr(c.g[V0][0] as u32).sb(c, 0);
        call_ptr(c, 0x0041_62d0, 0x001c_e6a8)?;
        c.g[RA][0] = sx(0x001c_e6b0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0041_6810, 0x001c_e6b0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = Addr(this.wrapping_add(0x60)).lw(c);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        let len_minus_1 = c.g[V0][0];
        Addr(this.wrapping_add(0x134)).sw(c, len_minus_1);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            let len = c.g[A0][0];
            Addr(this.wrapping_add(0x134)).sw(c, len);
        }
        c.cycles += 7;
        f.restore(c, 16, 0x80);
        f.restore(c, 17, 0x70);
        f.restore(c, 18, 0x60);
        f.restore(c, 19, 0x50);
        f.restore_ra(c, 0x40);
        f.close(c);
        Ok(())
    })
}

/// `cKeyboardPopup::MoveUpFromDigitRow(this, dir)` (0x1cbdb0): the key-state
/// machine behind moving the cursor up from the number row. The current
/// state is at `this+0x140`; each branch checks the keys of the states it
/// may move to and stores the new state. The `dir == 0` flag (`S1 < 1`) goes
/// to `this+0x144`.
pub fn keyboard_popup_move_up_from_digit_row(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let f = Frame::open(c, 0x40);
        c.g[A2][0] = 0;
        f.save(c, 16, 0x30);
        f.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        f.save_ra(c, 0x10);
        c.g[A1][0] = 0;
        let this = c.g[S0][0] as u32;
        KEY_ENABLED.call(c, 0x001c_bdd4)?;
        let enabled = c.g[V0][0] != 0;
        c.g[V1][0] = Addr(this.wrapping_add(0x140)).lw(c);
        if enabled {
            move_up_enabled(c, this)?;
        } else {
            move_up_disabled(c, this)?;
        }
        c.cycles += 5;
        f.restore(c, 16, 0x30);
        f.restore(c, 17, 0x20);
        f.restore_ra(c, 0x10);
        f.close(c);
        Ok(())
    })
}

/// The `KeyEnabled` answer was non-zero: states 0x10/0x11 go to 4, state 0x12
/// to 5 (both return 1); any other state returns 0.
pub(crate) fn move_up_enabled(c: &mut Cpu, this: u32) -> Flow {
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x10));
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    let in_10_11 = c.g[V0][0] != 0;
    c.g[V0][0] = 0x12;
    if !in_10_11 {
        c.cycles += 2;
        let differs = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = 0;
        if differs {
            return Ok(());
        }
        c.cycles += 1;
        c.g[V1][0] = 5;
    } else {
        c.cycles += 2;
        c.g[V1][0] = 4;
    }
    c.cycles += 3;
    c.g[V0][0] = 1;
    let state = c.g[V1][0];
    Addr(this.wrapping_add(0x140)).sw(c, state);
    Ok(())
}

/// The `KeyEnabled` answer was zero: the states 0x0f..=0x16, 0x17, 0x18/0x19
/// and 0x1a each check one key, and the other states return 0.
pub(crate) fn move_up_disabled(c: &mut Cpu, this: u32) -> Flow {
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xf));
    c.g[V0][0] = (c.g[V0][0] < 8) as u64;
    let in_0f_16 = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[S0][0];
    if in_0f_16 {
        return move_up_key_0x4b(c, this);
    }
    // 0x17 checks key 0x4c; 0x1a checks 0x53; 0x18/0x19 check 0x4f.
    c.cycles += 3;
    c.g[V0][0] = 0x17;
    let not_17 = c.g[V1][0] != c.g[V0][0];
    c.g[V0][0] = 0x1a;
    if !not_17 {
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x4c;
        c.g[RA][0] = sx(0x001c_be84);
        c.g[A2][0] = c.g[SP][0];
        KEY_ENABLED.call(c, 0x001c_be84)?;
        let key_on = c.g[V0][0] != 0;
        c.g[V0][0] = 0x4c;
        if key_on {
            c.cycles += 2;
            Addr(this.wrapping_add(0x140)).sw(c, 0x4c);
            return finish_move_up(c);
        }
        return move_up_key_failed(c, this);
    }
    c.cycles += 2;
    let not_1a = c.g[V1][0] != c.g[V0][0];
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x18));
    if !not_1a {
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x53;
        c.g[RA][0] = sx(0x001c_beac);
        c.g[A2][0] = c.g[SP][0];
        KEY_ENABLED.call(c, 0x001c_beac)?;
        let key_on = c.g[V0][0] != 0;
        c.g[V0][0] = 0x53;
        if key_on {
            c.cycles += 2;
            Addr(this.wrapping_add(0x140)).sw(c, 0x53);
            return finish_move_up(c);
        }
        return move_up_key_failed(c, this);
    }
    c.cycles += 3;
    c.g[V0][0] = ((c.g[V0][0]) < 2) as u64;
    let in_18_19 = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[S0][0];
    if !in_18_19 {
        c.cycles += 1;
        c.g[V0][0] = 0;
        return Ok(());
    }
    c.cycles += 5;
    c.g[A1][0] = 0x4f;
    c.g[RA][0] = sx(0x001c_bed4);
    c.g[A2][0] = c.g[SP][0];
    KEY_ENABLED.call(c, 0x001c_bed4)?;
    let key_on = c.g[V0][0] != 0;
    c.g[V0][0] = 0x4f;
    if key_on {
        c.cycles += 2;
        Addr(this.wrapping_add(0x140)).sw(c, 0x4f);
        return finish_move_up(c);
    }
    move_up_key_failed(c, this)
}

/// States 0x0f..=0x16: key 0x4b, then the state-0x14 check.
pub(crate) fn move_up_key_0x4b(c: &mut Cpu, this: u32) -> Flow {
    c.cycles += 5;
    c.g[A1][0] = 0x4b;
    c.g[RA][0] = sx(0x001c_be28);
    c.g[A2][0] = c.g[SP][0];
    KEY_ENABLED.call(c, 0x001c_be28)?;
    let key_on = c.g[V0][0] != 0;
    c.g[V0][0] = 0x4b;
    if key_on {
        c.cycles += 2;
        Addr(this.wrapping_add(0x140)).sw(c, 0x4b);
        return finish_move_up(c);
    }
    flag_store_move_up(c, this);
    c.cycles += 4;
    c.g[V1][0] = Addr(this.wrapping_add(0x140)).lw(c);
    c.g[V0][0] = 0x14;
    let not_14 = c.g[V1][0] != c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    if not_14 {
        return find_minus_one_move_up(c);
    }
    c.cycles += 4;
    c.g[RA][0] = sx(0x001c_be60);
    c.g[A1][0] = 0x3e;
    FIND_ENABLED_IN_COLUMN_FROM_BOTTOM.call(c, 0x001c_be60)?;
    c.g[V0][0] = 1;
    Ok(())
}

/// The key check of 0x17, 0x1a or 0x18/0x19 failed: store the flag, then
/// `FindEnabledInColumnFromBottom(this, -1)`.
pub(crate) fn move_up_key_failed(c: &mut Cpu, this: u32) -> Flow {
    flag_store_move_up(c, this);
    c.cycles += 1;
    c.g[A0][0] = c.g[S0][0];
    find_minus_one_move_up(c)
}

/// `this+0x144 = (a1 < 1)` when the key's out-value is non-zero.
pub(crate) fn flag_store_move_up(c: &mut Cpu, this: u32) {
    c.cycles += 3;
    c.g[V0][0] = Addr(sp(c)).lw(c);
    let no_out = c.g[V0][0] == 0;
    c.g[V0][0] = (c.g[S1][0] < 1) as u64;
    if !no_out {
        c.cycles += 1;
        let flag = c.g[V0][0];
        Addr(this.wrapping_add(0x144)).sw(c, flag);
    }
}

/// `FindEnabledInColumnFromBottom(this, -1)` and the return with 1.
pub(crate) fn find_minus_one_move_up(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    c.g[RA][0] = sx(0x001c_bf00);
    c.g[A1][0] = u64::MAX;
    FIND_ENABLED_IN_COLUMN_FROM_BOTTOM.call(c, 0x001c_bf00)?;
    finish_move_up(c)
}

/// The return with 1 after the state store (`+2` cycles).
pub(crate) fn finish_move_up(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    c.g[V0][0] = 1;
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::compare;

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::put_words;
    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_3;
    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::bx::bxstringctor::tests::alloc_base;
    pub(crate) use crate::game::bx::bxstringctor::tests::base_3 as base_11;
    pub(crate) use crate::game::render::particle::tests::f;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::SEEDS_6;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::alloc_ready;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::base_8 as base;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::common;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::zero;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::SEEDS_7 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_12;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_4 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::hash_screen;

    pub(crate) use crate::game::fe::feasyncfile::tests::init_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui_init_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::widget_action_test;
    pub(crate) use crate::game::fe::feasyncfile::tests::world;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_5;

    pub(crate) use crate::game::fe::fememcardcallbacks::tests::hash_screen as hash_screen_2;

    pub(crate) use crate::game::fe::fepopupmisc::tests::setup;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_8;

    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_6;
    pub(crate) use crate::game::main::gamestate::tests::slot_2 as slot_3;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_4;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base_4;

    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_7;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals_5;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::run;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_13;

    pub(crate) use crate::game::ui::uivector::tests::start;
    pub(crate) use crate::game::util::menu::tests::wf_2 as wf;
    pub(crate) use crate::game::world::worldcache::tests::check_all as check_all_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_9;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;

    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn c_fe_popup_confirm_get_centered_x_matches() {
        check(
            ssx3_ee::s03::f_001c7be0,
            c_fe_popup_confirm_get_centered_x,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c10_0001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(
                    DATA + 0x2cc,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(DATA + 0x260, r.edge_f32());
                c.bus.write32(DATA + 0x1b8, r.edge_f32());
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_position_buttons_matches() {
        check(
            ssx3_ee::s03::f_001c88c8,
            c_fe_popup_confirm_position_buttons,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c10_0002 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let child = if seed % 2 == 0 { 0 } else { DATA + 0x400 };
                c.bus.write32(DATA + 0x70, child);
                c.bus.write32(DATA + 0x1ac, r.next_u32());
                c.bus.write32(DATA + 0x1b0, r.next_u32());
                c.bus.write32(DATA + 0x1b4, r.next_u32());
                c.bus.write32(DATA + 0x290, r.edge_f32());
                c.bus.write32(DATA + 0x294, r.edge_f32());
                for i in 0..4 {
                    c.bus.write32(child + 0x44 + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_accumulate_size_matches() {
        check(
            ssx3_ee::s03::f_001c9b68,
            c_fe_popup_confirm_accumulate_size,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c10_0003 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = DATA as u64;
                c.g[A2][0] = (DATA + 0x40) as u64;
                // Force both compare outcomes: -1.0 vs 1.0 and the reverse.
                let a = if seed % 2 == 0 {
                    0xbf80_0000
                } else {
                    0x3f80_0000
                };
                let b = if seed % 2 == 0 {
                    0x3f80_0000
                } else {
                    0xbf80_0000
                };
                c.bus.write32(DATA, a);
                c.bus.write32(DATA + 4, r.edge_f32());
                c.bus.write32(DATA + 0x40 + 8, b);
                c.bus.write32(DATA + 0x40 + 0xc, r.edge_f32());
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_get_selection_matches() {
        check(
            ssx3_ee::s03::f_001ca150,
            c_fe_popup_confirm_get_selection,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c10_0004 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let sel = if seed % 2 == 0 { 0 } else { DATA + 0x400 };
                c.bus.write32(DATA + 0x6c, sel);
                c.bus.write32(DATA + 0x168, r.next_u32());
                c.bus.write32(
                    DATA + 0x30c,
                    if seed % 4 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() & 0x7fff_ffff
                    },
                );
                c.bus.write8(sel + 0x95, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn c_keyboard_popup_set_min_length_matches() {
        check(
            ssx3_ee::s04::f_001cd0b0,
            c_keyboard_popup_set_min_length,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c10_0006 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = match seed % 4 {
                    0 => (r.next_u32() % 64) as u64,
                    1 => 64 + (r.next_u32() % 64) as u64,
                    2 => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                    _ => 0xffff_ffff_ffff_ffff,
                };
                c.bus.write32(DATA + 0x64, r.next_u32());
            },
        );
    }

    #[test]
    fn c_keyboard_popup_is_key_enabled_matches() {
        check(
            ssx3_ee::s04::f_001ce560,
            c_keyboard_popup_is_key_enabled,
            400,
            &|c, seed| {
                let mut r = Rng(0x0c10_0008 ^ seed as u64);
                scramble(c, &mut r);
                let key = [15u64, 20, 5, 30][(seed % 4) as usize];
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = key;
                c.g[A2][0] = if seed % 2 == 0 {
                    0
                } else {
                    (DATA + 0x600) as u64
                };
                let child = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
                c.bus.write32(DATA + 0x6c, child);
                c.bus
                    .write32(DATA + 0x68, if seed % 5 == 0 { 0 } else { r.next_u32() });
                let base = DATA + (key as u32) * 4;
                c.bus.write32(base + 0x148, r.next_u32());
                c.bus.write32(base + 0x298, r.next_u32());
                c.bus.write32(DATA + 0x600, r.next_u32());
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_v24_matches() {
        widget_action_test(
            ssx3_ee::s03::f_001ca4f8__generated,
            c_fe_popup_confirm_v24,
            0xa9,
            0x20,
        );
    }

    pub(crate) fn widget_setup(c: &mut Cpu, seed: u32, salt: u64) -> u32 {
        let mut r = machine(c, seed, salt, 1);
        let w = DATA + 0x100;
        let vt = DATA + 0x200;
        c.bus.write32(w + 8, vt);
        c.bus
            .write16(vt + 0x48, ((r.next_u32() % 0x100) * 4) as u16);
        c.bus.write32(vt + 0x4c, FAKE);
        w
    }

    #[test]
    fn fe_popup_confirm_hide_widget_matches() {
        check(
            ssx3_ee::s03::f_001ca488__generated,
            fe_popup_confirm_hide_widget,
            SEEDS,
            &|c, seed| {
                let w = widget_setup(c, seed, 0xca488);
                c.g[A0][0] = sx(DATA + 0x300);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(w) };
            },
        );
    }

    #[test]
    fn fe_popup_confirm_show_widget_matches() {
        check(
            ssx3_ee::s03::f_001ca4c0__generated,
            fe_popup_confirm_show_widget,
            SEEDS,
            &|c, seed| {
                let w = widget_setup(c, seed, 0xca4c0);
                c.g[A0][0] = sx(DATA + 0x300);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(w) };
            },
        );
    }

    #[test]
    fn ckeyboard_popup_on_widget_action_matches() {
        check(
            ee::s04::f_001cb290__generated,
            ckeyboard_popup_on_widget_action,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1cb2_90, 0);
                let popup = DATA + 0x400;
                fill(c, &mut r, popup, 0x40);
                zero(c, DATA + 0x600, 0x40);
                c.g[A0][0] = sx(popup);
                c.g[A2][0] = sx((seed % 9) as u32);
                c.bus.write32(popup + 0x10, DATA + 0x600);
                // The state-manager list at +0x18 is empty (`head` of the sentinel).
                c.bus.write32(DATA + 0x600 + 0x18 + 4, 0);
            },
        );
    }

    /// A byte of test data.
    pub(crate) fn b(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }

    #[test]
    fn fe_popup_confirm_set_text_measured_matches() {
        check(
            ssx3_ee::s03::f_001c9b28__generated,
            fe_popup_confirm_set_text_measured,
            60,
            &|c, seed| {
                machine(c, seed, 0x1c9b_28, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
                c.g[A2][0] = (DATA + 0x500) as u64;
                b(c, DATA + 0x500, 0);
            },
        );
    }

    #[test]
    fn c_keyboard_popup_ensure_cursor_on_enabled_key_matches() {
        check(
            ssx3_ee::s04::f_001ce520__generated,
            c_keyboard_popup_ensure_cursor_on_enabled_key,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x21, 1);
                c.g[A0][0] = sx(DATA);
                // `cKeyboardPopup_IsKeyEnabled` answers from the per-key words
                // at `popup + index * 4 + 0x148` (shifted grid) and `+0x298`.
                // On a zeroed object every key reads "disabled", and then the
                // cursor walk (`cKeyboardPopup_MoveDown` -> its own recursive
                // tail / `cKeyboardPopup_StepDownToEnabledKey`) keeps stepping
                // down the grid until the guest stack overflows. Enable every
                // key, and disable only the cursor's own slot so that the
                // "move down" call is still exercised.
                for i in 0..0x100u32 {
                    c.bus.write32(DATA + 0x148 + 4 * i, 1);
                    c.bus.write32(DATA + 0x298 + 4 * i, 1);
                }
                // A plain-grid cursor key (>= 0xe skips the top-row walk; the
                // step below is a single `+0xe` into enabled keys).
                let key = 0x20 + seed % 0x10;
                c.bus.write32(DATA + 0x140, key);
                if seed % 2 == 1 {
                    c.bus.write32(DATA + 0x148 + 4 * key, 0);
                    c.bus.write32(DATA + 0x298 + 4 * key, 0);
                }
                // The grid state the walk consults (row marker and shift flag
                // in `+0x68`/`+0x6c`, the "current row" at `+0x430`/`+0x434`).
                c.bus.write32(DATA + 0x68, 0);
                c.bus.write32(DATA + 0x6c, 0);
                c.bus.write32(DATA + 0x430, 0);
                c.bus.write32(DATA + 0x434, 1);
                c.bus.write32(DATA + 0x144, (seed % 3) as u32);
            },
        );
    }

    // ------------------------------------------------------------ v24 popups

    pub(crate) fn popup_v24_machine(c: &mut Cpu, seed: u32, salt: u64, flag_off: u32) {
        let mut r = machine(c, seed, salt, 2);
        let this = DATA + 0x100;
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(r.next_u32() & 0xffff);
        c.g[A2][0] = sx(r.next_u32() & 0xffff);
        c.bus
            .write32(this + flag_off, if seed % 3 == 0 { 0 } else { 1 });
        let widget = if seed % 4 == 1 { 0 } else { DATA + 0x200 };
        c.bus.write32(this + 0x20, widget);
        if widget != 0 {
            let vt = DATA + 0x800;
            c.bus.write32(widget + 8, vt);
            slot(c, &mut r, vt, 0xc0, FAKE + 0x10);
        }
    }

    #[test]
    fn c_keyboard_popup_v24_matches() {
        check(
            ee::s04::f_001cb3d0__generated,
            c_keyboard_popup_v24,
            120,
            &|c, seed| {
                popup_v24_machine(c, seed, 0x1cb3_d0, 0x438);
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x1c8628 cFEPopupConfirm_GetWidgetSize.

    #[test]
    fn c_fe_popup_confirm_get_widget_size_matches() {
        check(
            ee::s03::f_001c8628__generated,
            c_fe_popup_confirm_get_widget_size,
            16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1c86_28, 1);
                let widget = DATA + 0x200;
                let out = DATA + 0x400;
                c.g[A0][0] = 0;
                c.g[A1][0] = sx(widget);
                c.g[A2][0] = sx(out);
                let vt = vtable(c, &mut r, &[0xa0]);
                c.bus.write32(widget + 8, vt);
                c.bus.write32(DATA + 0xff0, r.next_u32());
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x1ca528 cFEPopupConfirm_Update.

    #[test]
    fn c_fe_popup_confirm_update_matches() {
        check(
            ee::s03::f_001ca528__generated,
            c_fe_popup_confirm_update,
            16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1ca5_28, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x2c0, (seed % 2) as u32);
                c.bus.write32(this + 0x20, DATA + 0x300);
                let vt = vtable(c, &mut r, &[0x60]);
                c.bus.write32(DATA + 0x300 + 8, vt);
                // Make the base UI update find an empty object graph.
                c.bus.write32(this + 0x10, 0);
                c.bus.write32(this + 0xc, 0);
                c.bus.write32(this + 0x24, 0);
                c.bus.write8(this + 0x14, 0);
                c.bus.write8(this + 0x15, 0);
            },
        );
    }

    // ------------------------------------------------------ keyboard popup

    /// Enable exactly one key of the popup steppers: the popup's own flags
    /// (+0x68/+0x6c) stay 0 and `index` is enabled in both key tables.
    pub(crate) fn enable_key(c: &mut Cpu, index: i32) {
        let off = (index as u32).wrapping_mul(4);
        c.bus.write32(DATA.wrapping_add(0x148).wrapping_add(off), 1);
        c.bus.write32(DATA.wrapping_add(0x298).wrapping_add(off), 1);
    }

    #[test]
    fn ckeyboard_popup_find_enabled_in_column_from_top_matches() {
        check(
            ee::s04::f_001cc090__generated,
            ckeyboard_popup_find_enabled_in_column_from_top,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x1cc0);
                let cur = (r.next_u32() % 40) as i32;
                let found = cur - 0x38 + 0xe * (seed % 3) as i32;
                c.bus.write32(DATA + 0x140, cur as u32);
                enable_key(c, found);
                c.g[A0][0] = sx(DATA);
                c.pc = 0x001c_c090;
            },
        );
    }

    #[test]
    fn ckeyboard_popup_step_up_to_enabled_key_matches() {
        check(
            ee::s04::f_001cc140__generated,
            ckeyboard_popup_step_up_to_enabled_key,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x1cc1);
                let cur = 0x50 + (r.next_u32() % 40) as i32;
                let found = cur - 0xe - 0xe * (seed % 3) as i32;
                c.bus.write32(DATA + 0x140, cur as u32);
                enable_key(c, found);
                c.g[A0][0] = sx(DATA);
                c.pc = 0x001c_c140;
            },
        );
    }

    #[test]
    fn ckeyboard_popup_step_down_to_enabled_key_matches() {
        check(
            ee::s04::f_001cc190__generated,
            ckeyboard_popup_step_down_to_enabled_key,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x1cc2);
                let cur = (r.next_u32() % 40) as i32;
                let found = cur + 0xe + 0xe * (seed % 3) as i32;
                c.bus.write32(DATA + 0x140, cur as u32);
                enable_key(c, found);
                c.g[A0][0] = sx(DATA);
                c.pc = 0x001c_c190;
            },
        );
    }

    /// A 3-word pseudo-random scale vector at `at`.
    pub(crate) fn put_scale(c: &mut Cpu, r: &mut Rng, at: u32) {
        put_words(c, at, &[r.edge_f32(), r.edge_f32(), r.edge_f32()]);
    }

    #[test]
    fn c_fe_popup_confirm_select_option_matches() {
        check(
            ee::s03::f_001c75c8__generated,
            c_fe_popup_confirm_select_option,
            SEEDS_2,
            &|c, seed| {
                let _ = common(c, seed, 0x1c75c8, 0);
                let menu = DATA + 0x100;
                clear(c, menu, 0x200);
                c.g[A0][0] = sx(DATA + 0x300);
                c.g[A1][0] = sx(menu);
                c.g[A2][0] = (((seed as i32 % 7) - 3) as i64) as u64;
                c.g[A3][0] = match seed % 5 {
                    0 => u64::MAX,
                    1 => 0,
                    n => n as u64,
                };
            },
        );
    }

    #[test]
    fn c_keyboard_popup_on_activate_matches() {
        check(
            ee::s04::f_001cd2f0__generated,
            c_keyboard_popup_on_activate,
            SEEDS_2,
            &|c, seed| {
                let _ = common(c, seed, 0x1cd2f0, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                // The cursor must never go negative: OnActivate decrements it
                // at the limit, and UpdateCursor uses `cursor - length` as the
                // memcpy length. Keep `length` at 0 and the cursor positive.
                let v = 1 + (seed % 15);
                c.bus.write32(this + 0x60, v);
                c.bus
                    .write32(this + 0x134, if seed % 2 == 0 { v } else { v + 1 });
                c.bus.write32(this + 0x138, 0);
                c.bus.write32(this + 0x70, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed as u64) | 1;
            },
        );
    }

    #[test]
    fn c_keyboard_popup_apply_font_scale_matches() {
        check(
            ee::s04::f_001ce4c8__generated,
            c_keyboard_popup_apply_font_scale,
            SEEDS_2,
            &|c, seed| {
                let mut r = common(c, seed, 0x1ce4c8, 1);
                let obj = DATA + 0x1000;
                clear(c, obj, 0x100);
                put_scale(c, &mut r, obj + 0x50);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(obj);
            },
        );
    }

    #[test]
    fn cui_state_buy_popup_on_widget_action_matches() {
        check(
            ssx3_ee::s04::f_001caf00__generated,
            cui_state_buy_popup_on_widget_action,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x1caf00, 1);
                let this = DATA + 0x400;
                let widget = DATA + 0x500;
                let target = run(c, 0x0031_7670, ssx3_ee::s10::StrHash, &[0x004a_14e8]);
                let events = [0u32, 5, 5, 4, 6, 5];
                match seed % 5 {
                    0 => c.g[A1][0] = 0,
                    _ => {
                        c.bus.write32(
                            widget + 0x38,
                            if seed % 2 == 0 {
                                target as u32
                            } else {
                                0x1234_5678
                            },
                        );
                        c.g[A1][0] = widget as u64;
                    }
                }
                c.g[A0][0] = this as u64;
                c.g[A2][0] = events[(seed % 5) as usize] as u64;
            },
        );
    }

    #[test]
    fn cui_state_buy_popup_reset_params_matches() {
        check(
            ssx3_ee::s04::f_001ca8a0__generated,
            cui_state_buy_popup_reset_params,
            150,
            &|c, seed| {
                let _r = setup(c, seed, 0x1ca8a0);
                c.g[A0][0] = DATA as u64;
                for i in 0..0xc {
                    c.bus.write32(DATA + 4 * i, 0x1111_1111);
                }
            },
        );
    }

    #[test]
    fn c_ui_state_buy_popup_on_widget_notify_matches() {
        check(
            ssx3_ee::s04::f_001caed8__generated,
            c_ui_state_buy_popup_on_widget_notify,
            200,
            &|c, seed| {
                let _r = start(c, seed, 0xcaed8);
                c.g[A2][0] = match seed % 6 {
                    0 => 0,
                    1 => 6,
                    2 => 7,
                    3 => 9,
                    4 => 10,
                    5 => 0xffff,
                    _ => u64::MAX,
                };
            },
        );
    }

    #[test]
    fn keyboard_popup_find_enabled_in_column_matches() {
        check(
            ee::s04::f_001cc0e0__generated,
            c_keyboard_popup_find_enabled_in_column_from_bottom,
            SEEDS_3,
            &|c, seed| {
                base_2(c, seed, 0x1cc0_e0);
                let s = DATA + 0x100;
                clear(c, s, 0x400);
                // Index 56 (start 0x38 from a zero base_2) is disabled, 42 enabled:
                // the loop takes two iterations.
                w_2(c, s + 0x378, 0);
                w_2(c, s + 0x340, 1);
                c.pc = 0x001c_c0e0;
                c.g[A0][0] = sx(s);
                c.g[A1][0] = if seed % 2 == 0 {
                    0xffff_ffff_ffff_ffff
                } else {
                    0x38
                };
            },
        );
    }

    #[test]
    fn keyboard_popup_set_digit_keys_disabled_matches() {
        check(
            ee::s04::f_001ce408__generated,
            c_keyboard_popup_set_digit_keys_disabled,
            48,
            &|c, seed| {
                let _ = base_2(c, seed, 0x1ce4_08);
                let s = DATA + 0x100;
                clear(c, s, 0x400);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn cui_state_buy_popup_init_matches() {
        check(
            ee::s04::f_001cac30__generated,
            cui_state_buy_popup_init,
            6,
            &|c, seed| {
                init_setup(c, seed, 0x001c_ac30);
            },
        );
    }

    #[test]
    fn keyboard_popup_dtor_matches() {
        check(
            ee::s04::f_001cb138__generated,
            keyboard_popup_dtor,
            8,
            &|c, seed| {
                machine(c, seed, 0x001c_b138, 2);
                be_library(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.bus.write32(DATA + 0x428, 0);
            },
        );
    }

    #[test]
    fn keyboard_popup_init_matches() {
        check(
            ee::s04::f_001cb1a0__generated,
            keyboard_popup_init,
            6,
            &|c, seed| {
                init_setup(c, seed, 0x001c_b1a0);
            },
        );
    }

    #[test]
    fn keyboard_popup_mask_text_matches() {
        check(
            ee::s04::f_001cce98__generated,
            keyboard_popup_mask_text,
            24,
            &|c, seed| {
                machine(c, seed, 0x001c_ce98, 0);
                be_library(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                let n = seed % 20;
                for i in 0..n {
                    c.bus.write8(DATA + 0x1000 + i, b'q');
                }
                c.bus.write8(DATA + 0x1000 + n, 0);
            },
        );
    }

    #[test]
    fn keyboard_popup_set_static_text_id_matches() {
        check(
            ee::s04::f_001cd020__generated,
            keyboard_popup_set_static_text_id,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x001c_d020, 3);
                be_library(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x40, 0);
                } else {
                    c.bus.write32(DATA + 0x40, DATA + 0x3000);
                    hash_screen(c, DATA + 0x3000, DATA + 0x4200);
                }
            },
        );
    }

    #[test]
    fn cfe_state_trophy_room_count_event_entries_matches() {
        check(
            ee::s04::f_001ce6f0__generated,
            cfe_state_trophy_room_count_event_entries,
            200,
            &|c, seed| {
                let mut r = Rng(0x1ce6_f0d3 ^ seed as u64);
                scramble(c, &mut r);
                let a = (r.next_u32() % 4) as u64;
                let b = (r.next_u32() % 4) as u64;
                c.g[A0][0] = a;
                c.g[A1][0] = b;
                let base = 0x0045_aad8 + (a * 0x28 + b * 0x0a) as u32;
                let neg_from = (r.next_u32() % 6) as u32;
                for i in 0..5u32 {
                    let id: u8 = if i >= neg_from {
                        0x80
                    } else {
                        (r.next_u32() & 0x7f) as u8
                    };
                    c.bus.write8(base + 2 * i, id);
                }
            },
        );
    }

    #[test]
    fn c_ui_state_buy_popup_set_uber_trick_params_matches() {
        check(
            ee::s04::f_001caf58__generated,
            c_ui_state_buy_popup_set_uber_trick_params,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x1caf58, 0);
                globals_2(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = seed as u64;
                c.g[A2][0] = (seed as u64) << 8;
                c.g[A3][0] = (seed as u64) << 16;
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_base_dtor_matches() {
        check(
            ee::s03::f_001c5a90__generated,
            c_fe_popup_confirm_base_dtor,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x1c5a90, 0);
                globals_2(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { 1 };
                w_3(c, OBJ + 0x2b4, 0x0048_dcd4);
                w_3(c, OBJ + 0x2b0, 0x0048_dcd4);
                w_3(c, OBJ + 0xbc, 0);
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_v04_matches() {
        check(
            ee::s03::f_001c5af8__generated,
            c_fe_popup_confirm_v04,
            4,
            &|c, seed| {
                ui_init_setup(c, seed, 0x001c_5af8);
            },
        );
    }

    #[test]
    fn c_ui_state_buy_popup_set_attrib_params_matches() {
        check(
            ee::s04::f_001cafc0__generated,
            c_ui_state_buy_popup_set_attrib_params,
            6,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x001c_afc0, 2);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A2][0] = (r.next_u32() & 0xffff) as u64;
                c.g[A3][0] = (r.next_u32() & 0xffff) as u64;
            },
        );
    }

    #[test]
    fn c_ui_state_buy_popup_set_song_credit_params_matches() {
        check(
            ee::s04::f_001ca900__generated,
            c_ui_state_buy_popup_set_song_credit_params,
            12,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x1ca9_00, 2);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 16) as u64;
                c.g[A2][0] = (r.next_u32() % 16) as u64;
            },
        );
    }

    #[test]
    fn c_keyboard_popup_type_current_key_matches() {
        check(
            ee::s04::f_001cc7c8__generated,
            c_keyboard_popup_type_current_key,
            80,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x1cc7, 2);
                let p = DATA;
                fill(c, &mut r, p + 0x60, 8);
                w(c, p + 0x68, if seed % 3 == 0 { 0 } else { 1 });
                w(c, p + 0x6c, if seed % 2 == 0 { 0 } else { 1 });
                w(
                    c,
                    p + 0x140,
                    match seed % 4 {
                        0 => 0xf,
                        1 => 0x12,
                        2 => 0x1a,
                        _ => 0x20,
                    },
                );
                w(c, p + 0x134, (seed % 8) as u32);
                w(c, p + 0x60, 0x10);
                c.g[A0][0] = sx(p);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    #[test]
    fn ckeyboardpopup_update_matches() {
        covered(
            ee::s04::f_001cb208__generated,
            ckeyboardpopup_update,
            SEEDS_4,
            &|c, seed| {
                base_6(c, seed, 0xcb208, 1);
                let s = DATA + 0x100;
                c.g[A0][0] = sx(s);
                c.bus.write32(s + 0x440, if seed % 3 == 0 { 0 } else { 1 });
                let widget = DATA + 0x300;
                c.bus
                    .write32(s + 0x20, if seed % 4 == 0 { 0 } else { widget });
                let vt = DATA + 0x400;
                slot_3(c, vt, 0x60, 0, FAKE);
                c.bus.write32(widget + 8, vt);
                match seed % 4 {
                    0 => c.bus.write32(GAME_GP - 0x204, 0),
                    1 => {
                        let input = DATA + 0x500;
                        c.bus.write32(GAME_GP - 0x204, input);
                        c.bus.write32(input + 0x64, 1);
                    }
                    _ => {
                        let input = DATA + 0x500;
                        c.bus.write32(GAME_GP - 0x204, input);
                        c.bus.write32(input + 0x64, 0);
                        if seed % 8 == 2 {
                            c.bus.write32(DATA + 0x9000 + 0x84, 0);
                        }
                    }
                }
                c.bus.write32(s + 0x10, 0x10);
            },
        );
    }

    /// `StrHash(addr)` (0x317670), for fixtures that must match a hash name.
    pub(crate) fn str_hash(c: &mut Cpu, addr: u32) -> u64 {
        const RET: u32 = 0xdead_0200;
        c.g[A0][0] = sx(addr);
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_7670;
        ee::s10::f_00317670(c);
        assert_eq!(c.pc, RET, "StrHash did not return");
        c.g[V0][0]
    }

    #[test]
    fn c_keyboard_popup_refresh_entry_text_matches() {
        check(
            ee::s04::f_001cc848__generated,
            c_keyboard_popup_refresh_entry_text,
            120,
            &|c, seed| {
                let _ = base_7(c, seed, 0x1c_c848);
                globals_5(c);
                const SCREEN: u32 = DATA + 0x1000;
                const NODE: u32 = DATA + 0x2000;
                if seed % 2 == 0 {
                    // No screen at all: the text lookup fails.
                    w(c, DATA + 0x40, 0);
                } else {
                    // A one-object screen whose entry matches the hash name.
                    let hash = str_hash(c, 0x0046_6530);
                    w(c, DATA + 0x40, SCREEN);
                    w(c, SCREEN + 0x3c, 1);
                    w(c, SCREEN + 0x38, DATA + 0x2100);
                    w(c, DATA + 0x2100, NODE);
                    w(c, NODE + 0x38, hash as u32);
                    // The list at +0x74 stays empty.
                    w(c, DATA + 0x74, 0);
                }
                w(c, DATA + 0x70, [0, 1][seed as usize % 2]);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn cfe_popup_confirm_set_text_from_desc_matches() {
        check(
            ee::s03::f_001c8050__generated,
            cfe_popup_confirm_set_text_from_desc,
            60,
            &|c, seed| {
                let mut r = base_8(c, seed, 0x1c8050, 4);
                let text = DATA + 0x1000;
                let desc = DATA + 0x2000;
                c.g[A0][0] = text as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = desc as u64;
                w(
                    c,
                    desc,
                    match seed % 5 {
                        0 => 0,
                        1 => 1,
                        2 => 2,
                        3 => 3,
                        _ => r.next_u32() % 8,
                    },
                );
                w(c, desc + 0x4, r.next_u32());
            },
        );
    }

    #[test]
    fn cuistate_buy_popup_set_music_track_params_matches() {
        check(
            ee::s04::f_001ca980__generated,
            cuistate_buy_popup_set_music_track_params,
            40,
            &|c, seed| {
                let mut r = base_8(c, seed, 0x1ca980, 2);
                let obj = DATA + 0x1000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn cuistate_buy_popup_set_item_params_matches() {
        check(
            ee::s04::f_001cab10__generated,
            cuistate_buy_popup_set_item_params,
            40,
            &|c, seed| {
                let mut r = base_8(c, seed, 0x1cab10, 2);
                let obj = DATA + 0x1000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    // ------------------------------------------------------------ keyboard

    /// A keyboard popup whose key tables are all enabled except the given key.
    pub(crate) fn keyboard(c: &mut Cpu, state: u32, disable: &[u32]) -> u32 {
        let popup = DATA + 0x100;
        clear_2(c, popup, 0x700);
        for i in 0..((0x700 - 0x148) / 4) {
            w(c, popup + 0x148 + 4 * i, 1);
        }
        for &k in disable {
            w(c, popup + 0x148 + 4 * k, 0);
            w(c, popup + 0x298 + 4 * k, 0);
        }
        w(c, popup + 0x140, state);
        w(c, popup + 0x144, 0x55);
        popup
    }

    #[test]
    fn c_keyboard_popup_move_down_from_top_row_matches() {
        check(
            ssx3_ee::s04::f_001cbc48__generated,
            c_keyboard_popup_move_down_from_top_row,
            16,
            &|c, seed| {
                let _ = base_9(c, seed, 0x001c_bc48);
                let state = [4u32, 5, 4, 5, 0x40, 0x4c][seed as usize % 6];
                let disable: &[u32] = if seed % 2 == 0 { &[] } else { &[0x10, 0x12] };
                let popup = keyboard(c, state, disable);
                c.g[A0][0] = sx(popup);
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn c_keyboard_popup_move_up_from_bottom_row_matches() {
        check(
            ssx3_ee::s04::f_001cbf20__generated,
            c_keyboard_popup_move_up_from_bottom_row,
            16,
            &|c, seed| {
                let _ = base_9(c, seed, 0x001c_bf20);
                let state = [0x4cu32, 0x4f, 0x4c, 0x4f, 0x40, 4][seed as usize % 6];
                let disable: &[u32] = if seed % 2 == 0 { &[] } else { &[0x42] };
                let popup = keyboard(c, state, disable);
                c.g[A0][0] = sx(popup);
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    // --------------------------------------------------------- popup confirm

    #[test]
    fn cfe_popup_confirm_v06_matches() {
        check(
            ee::s03::f_001c5b88__generated,
            cfe_popup_confirm_v06,
            SEEDS_5,
            &|c, seed| {
                let _ = base_10(c, seed, 0x1c5b_88, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                let popup = if seed % 3 == 0 { 0 } else { obj + 0x2000 };
                w(c, obj + 0x6c, popup);
                w(c, obj + 0x2c4, seed as u32);
                w(c, obj + 0x30c, seed.wrapping_mul(3));
                // The popup's two child objects.
                w(c, popup + 8, obj + 0x2100);
                w(c, obj + 0x2100 + 0x38, 0);
                w(c, obj + 0x2100 + 0x3c, 0);
                slot_5(c, obj + 0x2100, 0x38, 0);
                slot_5(c, obj + 0x2100, 0x3c, 1);
                slot_5(c, obj + 0x2100, 0x40, 2);
                slot_5(c, obj + 0x2100, 0x44, 3);
                // The item object at_4 self+8.
                w(c, obj + 8, obj + 0x2200);
                slot_5(c, obj + 0x2200, 0xa0, 4);
                slot_5(c, obj + 0x2200, 0xa4, 5);
            },
        );
    }

    pub(crate) fn wide(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write16(at + 2 * i as u32, *b as u16);
        }
    }

    #[test]
    fn cfe_popup_confirm_layout_post_text_matches() {
        // No text object, or an empty source word (the full relayout calls
        // into the UI text engine).
        check(
            ee::s03::f_001c7040__generated,
            cfe_popup_confirm_layout_post_text,
            SEEDS_6,
            &|c, seed| {
                let mut r = base_11(c, seed, 0x1c70);
                let src = DATA + 0x800;
                let text = DATA + 0x600;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(src);
                let no_text = seed % 2 == 0;
                c.bus.write32(DATA + 0x78, if no_text { 0 } else { text });
                c.bus.write32(src + 0x4c, if no_text { 1 } else { 0 });
                fill(c, &mut r, DATA + 0x1b8, 4);
                fill(c, &mut r, DATA + 0x260, 4);
                c.bus.write32(DATA + 0x264, f(1.5));
                fill(c, &mut r, text, 0x40);
            },
        );
    }

    #[test]
    fn keyboard_popup_set_static_wide_text_matches() {
        check(
            ee::s04::f_001ccf70__generated,
            keyboard_popup_set_static_wide_text,
            40,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x1ccf);
                let str_ = DATA + 0x1000;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(str_);
                wide(c, str_, b"abc");
                c.bus.write16(str_ + 8, 0);
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x40, 0);
                    c.bus
                        .write32(DATA + 0x428, if seed % 4 == 0 { 0 } else { DATA + 0x3000 });
                } else {
                    let screen = DATA + 0x2000;
                    let node = DATA + 0x2100;
                    c.bus.write32(DATA + 0x40, screen);
                    hash_screen_2(c, screen, node, 0x0046_6580);
                    c.bus.write32(node + 8, DATA + 0x2200);
                }
            },
        );
    }

    #[test]
    fn fe_popup_confirm_measure_line_matches() {
        check(
            ee::s03::f_001c84a8__generated,
            fe_popup_confirm_measure_line,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1c84, 4);
                let metrics = DATA + 0x100;
                let out = DATA + 0x200;
                for k in 0..4 {
                    c.bus.write32(metrics + 0x30 + 4 * k, r.edge_f32());
                    c.bus.write32(metrics + 0x50 + 4 * k, r.edge_f32());
                    c.bus.write32(out + 4 * k, r.edge_f32());
                }
                c.bus.write32(metrics + 0x14, r.next_u32());
                c.bus.write32(metrics + 0x58, r.next_u32());
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(metrics);
                c.g[A2][0] = sx(out);
                c.g[A3][0] = sx(DATA + 0x300);
            },
        );
    }

    #[test]
    fn fe_popup_confirm_measure_line_w_matches() {
        check(
            ee::s03::f_001c8568__generated,
            fe_popup_confirm_measure_line_w,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1c85, 4);
                let metrics = DATA + 0x100;
                let out = DATA + 0x200;
                for k in 0..4 {
                    c.bus.write32(metrics + 0x30 + 4 * k, r.edge_f32());
                    c.bus.write32(metrics + 0x50 + 4 * k, r.edge_f32());
                    c.bus.write32(out + 4 * k, r.edge_f32());
                }
                c.bus.write32(metrics + 0x14, r.next_u32());
                c.bus.write32(metrics + 0x58, r.next_u32());
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(metrics);
                c.g[A2][0] = sx(out);
                c.g[A3][0] = sx(DATA + 0x300);
            },
        );
    }

    #[test]
    fn c_keyboard_popup_move_down_from_bottom_row_matches() {
        check(
            ee::s04::f_001cbce8__generated,
            c_keyboard_popup_move_down_from_bottom_row,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1cbc, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(
                    this + 0x140,
                    match seed % 3 {
                        0 => 0x4c,
                        1 => 0x4b,
                        _ => 0x38,
                    },
                );
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn cfe_popup_confirm_get_widget_text_matches() {
        check(
            ee::s03::f_001c8258__generated,
            cfe_popup_confirm_get_widget_text,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1c82, 3);
                game_globals(c);
                let this = DATA;
                let dst = DATA + 0x2000;
                let text = DATA + 0x2400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = dst as u64;
                c.g[A2][0] = text as u64;
                c.bus.write32(text + 0x0, seed % 4);
                c.bus.write32(text + 0x4, seed % 3);
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn c_keyboard_popup_insert_char_matches() {
        check_all(
            ee::s04::f_001cc620__generated,
            c_keyboard_popup_insert_char,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1cc6, 0);
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 0x41 + (seed % 26) as u64;
                for (i, ch) in b"abcd".iter().enumerate() {
                    c.bus.write8(this + 0x74 + i as u32, *ch);
                }
                c.bus.write8(this + 0x78, 0);
                let max = 3 + (seed % 4);
                c.bus.write32(this + 0x60, max);
                c.bus.write32(this + 0x134, seed % (max + 1));
                c.bus.write32(this + 0x13c, r.next_u32() % 8);
            },
        );
    }

    #[test]
    fn ckeyboard_popup_move_down_from_row4_matches() {
        check(
            ee::s04::f_001cbfc0__generated,
            ckeyboard_popup_move_down_from_row4,
            80,
            &|c, seed| {
                let _r = base_12(c, seed, 0x1cbf);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                let row = [0x39u32, 0x3a, 0x42, 0x10][(seed % 4) as usize];
                c.bus.write32(DATA + 0x140, row);
                c.bus.write32(DATA + 0x40, DATA + 0x1000);
            },
        );
    }

    #[test]
    fn ckeyboardpopup_handle_key_char_matches() {
        check(
            ee::s04::f_001cc6e8__generated,
            ckeyboardpopup_handle_key_char,
            16,
            &|c, seed| {
                let _ = base_13(c, seed, 0x1cc6e8, 2);
                let this = DATA;
                clear_3(c, this, 0x200);
                c.pc = 0x001c_c6e8;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = match seed % 4 {
                    0 => 0x60,
                    1 => 0,
                    2 => 0x2a,
                    _ => (seed % 90) as u64,
                };
                for i in 0..84u32 {
                    w(c, 0x0044_1390 + 4 * i, 0);
                    w(c, 0x0044_13e8 + 4 * i, 0);
                }
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_apply_background_scale_matches() {
        same(
            ssx3_ee::s03::f_001c7738__generated,
            c_fe_popup_confirm_apply_background_scale,
            200,
            &|c, seed| {
                let mut r = Rng(0x1c7738 ^ seed as u64);
                scramble(c, &mut r);
                let self_at = DATA + 0x100;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                for i in 0..3 {
                    c.bus.write32(DATA + 0x300 + 4 * i, r.next_u32());
                }
                c.bus.write32(self_at + 0x31c, (seed % 4) as u32);
                for (i, off) in [0x48u32, 0x4c, 0x50, 0x54, 0x58, 0x5c, 0x60, 0x68]
                    .iter()
                    .enumerate()
                {
                    let p = if (seed as usize + i) % 3 == 0 {
                        0
                    } else {
                        DATA + 0x400 + 0x40 * i as u32
                    };
                    c.bus.write32(self_at + *off, p);
                    for k in 0..4 {
                        c.bus.write32(p + 0x50 + 4 * k, r.next_u32());
                    }
                }
            },
        );
    }

    // -- 0x0018eb40 FEScreenSelectProfile_OnWidgetAction ----------------

    // -- 0x001c7258 cFEPopupConfirm_LayoutBodyText ----------------------

    static LBT_PREV_RESIZE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    const LBT_CTRL: u32 = DATA + 0x6000;
    const LBT_MAGIC: u32 = 0x1c72_5800;

    /// Gated fake of `cFEPopupConfirm_ResizeTextWidgets` (the real body walks
    /// allocator lists on test data): logs the call, returns.
    fn lbt_fake_resize(c: &mut Cpu) {
        if c.bus.read32(LBT_CTRL) != LBT_MAGIC {
            return fakes::fall_back(c, &LBT_PREV_RESIZE, ee::s03::f_001c70f0__generated);
        }
        fakes::fake_call(c, 0x001c_70f0);
    }

    #[test]
    fn c_fe_popup_confirm_layout_body_text_matches() {
        // No text widget; a text widget alone; with the second widget, both
        // with and without the height overflow (`+0x260` against the wrap
        // size). The callees are faked (Resize through a gated hook; its real
        // body walks allocator lists on test data).
        check(
            ee::s03::f_001c7258__generated,
            c_fe_popup_confirm_layout_body_text,
            120,
            &|c, seed| {
                fakes::hook(
                    std::ptr::addr_of_mut!(ee::s03::OVERRIDE_001C70F0),
                    lbt_fake_resize,
                    &LBT_PREV_RESIZE,
                );
                let mut r = fakes::machine_rt(c, seed, 0x1c7258, 4);
                c.bus.write32(LBT_CTRL, LBT_MAGIC);
                let this = DATA + 0x1000;
                let text = DATA + 0x3000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x2000);
                fill(c, &mut r, this + 0x1b8, 4);
                fill(c, &mut r, this + 0x260, 4);
                wf(c, this + 0x264, r.f32(20.0));
                wf(c, this + 0x1bc, r.f32(50.0));
                wf(
                    c,
                    this + 0x260,
                    if seed & 2 == 0 {
                        (-1.0e30f32).to_bits()
                    } else {
                        1.0e30f32.to_bits()
                    },
                );
                fill(c, &mut r, text, 0x40);
                let has_text = seed % 5 != 0;
                w(c, this + 0x74, if has_text { text } else { 0 });
                w(c, this + 0x78, 0);
                w(c, this + 0x7c, 0);
                w(
                    c,
                    this + 0x2cc,
                    if seed & 1 == 0 { 0 } else { DATA + 0x4000 },
                );
                if has_text {
                    let vt = DATA + 0x5000;
                    w(c, text + 8, vt);
                    c.bus.write16(vt + 0xa0, 0);
                    w(c, vt + 0xa4, FAKE);
                    c.bus.write16(vt + 0xa8, 0);
                    w(c, vt + 0xac, FAKE + 0x40);
                }
            },
        );
    }

    // -- 0x001c70f0 cFEPopupConfirm_ResizeTextWidgets -------------------

    #[test]
    fn cfe_popup_confirm_resize_text_widgets_matches() {
        check(
            ssx3_ee::s03::f_001c70f0__generated,
            cfe_popup_confirm_resize_text_widgets,
            120,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1c70f0, 8);
                let this = DATA + 0x1000;
                let size = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(size);
                wf(c, size, r.f32(50.0));
                wf(
                    c,
                    size + 4,
                    if seed % 3 == 0 { 0 } else { 12.0f32.to_bits() },
                );
                for (i, slot) in [0x74u32, 0x78, 0x7c].into_iter().enumerate() {
                    let wid = if (seed >> i) & 1 == 0 {
                        0
                    } else {
                        DATA + 0x3000 + 0x100 * i as u32
                    };
                    w(c, this + slot, wid);
                    if wid != 0 {
                        let vt = DATA + 0x5000 + 0x100 * i as u32;
                        w(c, wid + 8, vt);
                        c.bus.write16(vt + 0xa0, 0);
                        w(c, vt + 0xa4, FAKE + 0x10 * i as u32);
                        c.bus.write16(vt + 0xa8, 0);
                        w(c, vt + 0xac, FAKE + 0x40 + 0x10 * i as u32);
                    }
                }
            },
        );
    }

    #[test]
    fn position_background_matches() {
        check(
            ssx3_ee::s03::f_001c78c0__generated,
            fe_popup_confirm_position_background,
            250,
            &|c, seed| {
                let mut r = Rng(0x001c_78c0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x3000) as u64;
                c.f[12] = r.f32(40.0);
                c.f[13] = r.f32(40.0);
                for k in 0..3u32 {
                    c.bus.write32(DATA + 0x3000 + 4 * k, r.next_u32());
                }
                // Every layout: 0, 1, 2, >2 and negative.
                let state = match seed % 7 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => -1i32,
                    5 => 0x7fff_ffff,
                    _ => 0,
                };
                c.bus.write32(DATA + 0x31c, state as u32);
                let null_odd = seed % 3 == 0;
                let children = [0x48u32, 0x4c, 0x58, 0x5c, 0x50, 0x54, 0x60];
                for (i, off) in children.iter().enumerate() {
                    let p = if null_odd && i % 2 == 0 {
                        0
                    } else {
                        DATA + 0x1000 + 0x100 * i as u32
                    };
                    c.bus.write32(DATA + off, p);
                }
                let sources = [0x188u32, 0x194, 0x23c, 0x248, 0x224, 0x230, 0x254];
                for off in sources {
                    for k in 0..3u32 {
                        c.bus.write32(DATA + off + 4 * k, r.next_u32());
                    }
                }
                c.bus.write32(DATA + 0x1bc, r.f32(100.0));
                c.bus.write32(DATA + 0x278, r.next_u32());
                c.bus.write32(DATA + 0x27c, r.next_u32());
            },
        );
    }

    /// Float bits at `a`.
    pub(crate) fn f_2(c: &mut Cpu, a: u32, bits: u32) {
        c.bus.write32(a, bits);
    }

    /// The zeroed BE-library singleton at `gp-0x1f04` (else the FE helpers walk
    /// into the lazy disc load).
    pub(crate) fn world_2(c: &mut Cpu) {
        // Minimal world_2 singleton: world_2 -> +0x84 game -> +0xc race manager.
        let w = DATA + 0x8_0000;
        c.bus
            .write32(crate::difftest::GAME_GP.wrapping_sub(0x848), w);
        c.bus.write32(w + 0x84, w + 0x100);
        c.bus.write32(w + 0x100 + 0xc, w + 0x200);
    }

    pub(crate) fn be_library_2(c: &mut Cpu) {
        w(
            c,
            crate::difftest::GAME_GP.wrapping_sub(0x1f04),
            DATA + 0x30_0000,
        );
    }

    #[test]
    fn c_fe_popup_confirm_layout_menu_matches() {
        const S: u64 = 0x001c_8758;
        check_all_2(
            ee::s03::f_001c8758__generated,
            c_fe_popup_confirm_layout_menu,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, S, 12);
                be_library_2(c);
                world_2(c);
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                if seed % 6 != 0 {
                    w(c, this + 0x6c, DATA + 0x2000);
                }
                for i in 0..5u32 {
                    let obj = DATA + 0x2000 + i * 0x100;
                    let present = (seed >> i) & 1 != 0;
                    w(c, this + 0x90 + i * 4, if present { obj } else { 0 });
                    w(c, obj + 0x44, r.next_u32());
                    w(c, obj + 0x48, r.next_u32());
                    w(c, obj + 0x4c, r.next_u32());
                    let vt = DATA + 0x3000 + i * 0x100;
                    w(c, obj + 8, vt);
                    c.bus.write16(vt + 0xa0, (r.next_u32() as u16) & 0xfff0);
                    w(c, vt + 0xa4, 0x4000);
                    w(c, obj + 0x14, r.next_u32());
                }
                w(c, this + 0x1a0, r.next_u32());
                w(c, this + 0x1a4, r.next_u32());
                w(c, this + 0x1a8, r.next_u32());
                f_2(c, this + 0x288, r.edge_f32());
                f_2(c, this + 0x29c, r.edge_f32());
                w(c, this + 0x2c8, ((seed / 2) % 2) as u32);
            },
        );
    }

    pub(crate) const BEL: u32 = DATA + 0x10_0000;

    pub(crate) const WORLD_4: u32 = DATA + 0x11_0000;

    pub(crate) const AUX: u32 = DATA + 0x12_0000;

    pub(crate) const SYS: u32 = DATA + 0x13_0000;

    pub(crate) const RACE: u32 = DATA + 0x14_0000;

    pub(crate) const RMAN_2: u32 = DATA + 0x15_0000;

    /// Stricter than `check`: caller-saved registers and LO/HI must
    /// match too (a fake callee hashes every GPR and the cycles).
    pub(crate) fn check_all_3(
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        for seed in 0..crate::difftest::seed_count(seeds) {
            let r = compare(generated, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} (soft): {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} (LO/HI): {:#?}", r.lohi);
        }
    }

    /// The scaffolding most of these functions read: fake callees, the
    /// BELibrary singleton, the world chain (`world -> +0x84 -> race ->
    /// +0xc -> race manager`, rider count at `+0x7c`), the script
    /// engine at `gp-0x84c` and a dummy at `gp-0x854`.
    pub(crate) fn base_14(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.bus.write32(GAME_GP - 0x1f04, BEL);
        c.bus.write32(GAME_GP - 0x848, WORLD_4);
        c.bus.write32(GAME_GP - 0x854, AUX);
        c.bus.write32(GAME_GP - 0x84c, SYS);
        c.bus.write32(WORLD_4 + 0x84, RACE);
        c.bus.write32(RACE + 0xc, RMAN_2);
        c.bus.write32(RMAN_2 + 0x7c, 1);
        r
    }

    pub(crate) const POPUP: u32 = DATA + 0x1_0000;

    #[test]
    fn c_keyboard_popup_move_down_matches() {
        check_all_3(
            ssx3_ee::s04::f_001cb9b0__generated,
            c_keyboard_popup_move_down,
            40,
            &|c, seed| {
                let mut r = base_14(c, seed, 0x1cb9_b0, 8);
                c.g[A0][0] = sx(POPUP);
                c.g[A1][0] = sx(seed % 2);
                c.bus.write32(POPUP + 0x68, (seed % 2) as u32);
                c.bus.write32(POPUP + 0x6c, ((seed / 2) % 2) as u32);
                let idx = [0u32, 14, 56, 70, 100][(seed as usize / 3) % 5];
                c.bus.write32(POPUP + 0x140, idx);
                c.bus
                    .write32(POPUP + 0x144, if seed % 4 == 0 { 2 } else { 1 });
                c.bus.write32(POPUP + 0x430, (seed % 3) as u32);
                c.bus.write32(POPUP + 0x434, r.next_u32() % 0x40);
                // The key table: no 0x2a runs (one seed walks the skip loop once).
                for i in 0..0x100 {
                    c.bus.write8(0x0044_1390 + i, 0);
                }
                if seed % 3 == 0 {
                    c.bus.write8(0x0044_1390 + idx, 0x2a);
                }
                // `IsKeyEnabled` reads `+0x148`/`+0x298` at `idx*4`: keep every
                // slot non-zero so the tail never re-enters the function.
                for i in 0..0x400 {
                    c.bus.write32(POPUP + 0x148 + i * 4, 1);
                }
            },
        );
    }

    pub(crate) const POPUP2: u32 = DATA + 0x1_0000;

    pub(crate) const POPUP2_TEXT: u32 = DATA + 0x1_1000;

    pub(crate) const POPUP2_STR: u32 = DATA + 0x1_2000;

    pub(crate) const POPUP2_SIZE: u32 = DATA + 0x1_3000;

    #[test]
    fn c_fe_popup_confirm_get_text_box_size_matches() {
        check_all_3(
            ee::s03::f_001c9210__generated,
            c_fe_popup_confirm_get_text_box_size,
            20,
            &|c, seed| {
                let mut r = base_14(c, seed, 0x1c92_10, 10);
                c.g[A0][0] = sx(POPUP2);
                c.g[A1][0] = sx(POPUP2_TEXT);
                c.g[A2][0] = sx(POPUP2_STR);
                c.g[A3][0] = sx(POPUP2_SIZE);
                let s: &[u8] = match seed % 4 {
                    0 => b"ab\r\ncd",
                    1 => b"hello",
                    2 => b"a\\\\b",
                    _ => b"\r\r",
                };
                for (i, b) in s.iter().enumerate() {
                    c.bus.write8(POPUP2_STR + i as u32, *b);
                }
                c.bus.write8(POPUP2_STR + s.len() as u32, 0);
                c.bus.write32(POPUP2_SIZE, r.f32(1.0));
                c.bus.write32(POPUP2_SIZE + 4, r.f32(1.0));
            },
        );
    }

    pub(crate) const POPUP3: u32 = DATA + 0x1_0000;

    pub(crate) const POPUP3_TEXT: u32 = DATA + 0x1_1000;

    pub(crate) const POPUP3_STR: u32 = DATA + 0x1_2000;

    pub(crate) const POPUP3_SIZE: u32 = DATA + 0x1_3000;

    #[test]
    fn c_fe_popup_confirm_get_text_box_size_w_matches() {
        check_all_3(
            ee::s03::f_001c9038__generated,
            c_fe_popup_confirm_get_text_box_size_w,
            20,
            &|c, seed| {
                let mut r = base_14(c, seed, 0x1c90_38, 10);
                c.g[A0][0] = sx(POPUP3);
                c.g[A1][0] = sx(POPUP3_TEXT);
                c.g[A2][0] = sx(POPUP3_STR);
                c.g[A3][0] = sx(POPUP3_SIZE);
                let s: &[u16] = match seed % 4 {
                    0 => &[b'a' as u16, b'b' as u16, 13, b'c' as u16, b'd' as u16],
                    1 => &[b'h' as u16, b'i' as u16],
                    2 => &[b'a' as u16, 92, 92, b'b' as u16],
                    _ => &[13, 13],
                };
                for (i, w) in s.iter().enumerate() {
                    c.bus.write16(POPUP3_STR + 2 * i as u32, *w);
                }
                c.bus.write16(POPUP3_STR + 2 * s.len() as u32, 0);
                c.bus.write32(POPUP3_SIZE, r.f32(1.0));
                c.bus.write32(POPUP3_SIZE + 4, r.f32(1.0));
            },
        );
    }

    // -------------------------------------------- cFEPopupConfirm_v07 (grow-in)

    /// A confirm popup at `POPUP` in its grow-in (`a1 = 0`): background
    /// `+0x31c`, step `+0xa4` of `+0xa8`, scales, no widgets.
    fn transition_in_setup(c: &mut Cpu, seed: u32) {
        let mut r = base_14(c, seed, 0x1c_5c30, 3);
        let p = POPUP;
        c.bus.write32(p + 0x31c, if seed % 4 == 0 { 0 } else { 1 });
        let steps = 2 + seed % 30;
        c.bus.write32(p + 0xa8, steps);
        c.bus.write32(
            p + 0xa4,
            if seed % 5 == 0 {
                steps
            } else {
                r.next_u32() % steps
            },
        );
        for off in [0xac, 0xb0, 0x278, 0x27c, 0x280] {
            c.bus.write32(p + off, r.f32(2.0));
        }
        c.bus.write32(GAME_GP - 0x5900, r.f32(1.0));
        c.bus.write32(p + 0x58, DATA + 0x2_0000);
        for k in 0..4 {
            c.bus.write32(DATA + 0x2_0000 + 0x44 + 4 * k, r.next_u32());
        }
        c.g[A0][0] = sx(p);
        c.g[A1][0] = 0;
    }

    #[test]
    fn cfe_popup_confirm_transition_in_matches() {
        check(
            ee::s03::f_001c5c30__generated,
            cfe_popup_confirm_transition_in,
            200,
            &|c, seed| transition_in_setup(c, seed),
        );
    }

    #[test]
    fn cfe_popup_confirm_transition_in_open_matches() {
        // The first call (`a1 != 0`): the popup's config (as
        // `cFEPopupConfirm_Rebuild`'s test lays it out), no screen.
        check(
            ee::s03::f_001c5c30__generated,
            cfe_popup_confirm_transition_in,
            8,
            &|c, seed| {
                let _ = crate::game::movie::movieplayer::tests::alloc_base(c, seed, 0x1c_5c31, 6);
                let p = DATA;
                let child = DATA + 0x1000;
                c.bus.write32(p + 0x8, child);
                crate::game::movie::movieplayer::tests::ret_slot(
                    c,
                    child,
                    0x30,
                    crate::game::movie::movieplayer::tests::RET_ZERO,
                );
                c.bus.write32(p + 0x278, 0x3f00_0000);
                c.bus.write32(p + 0x27c, 0x3fc0_0000);
                c.bus.write32(p + 0x280, 0x3f80_0000);
                c.bus.write32(p + 0xbc, 0);
                // The background style (config word 0x30 -> +0x31c).
                c.bus.write32(p + 0xbc + 0xc0, (seed % 2) as u32);
                c.g[A0][0] = p as u64;
                c.g[A1][0] = 1;
            },
        );
    }

    #[test]
    fn cfe_popup_confirm_transition_in_rate() {
        // The grow-in takes as long at any rate: 25 calls at 60 (24 steps and
        // the last), about rate/60 times as many calls above.
        let calls = |hz: u32| -> u32 {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            transition_in_setup(c, 1);
            crate::difftest::fakes::machine(c, 1, 0x1c_5c30, 0);
            c.bus.write32(GAME_GP - 0x1f04, BEL);
            crate::tick::set_for_test(c, hz);
            let p = POPUP;
            c.bus.write32(p + 0x31c, 1);
            c.bus.write32(p + 0xa8, 25);
            c.bus.write32(p + 0xa4, 1);
            let mut n = 0;
            loop {
                n += 1;
                c.g[A0][0] = sx(p);
                c.g[A1][0] = 0;
                c.g[RA][0] = crate::difftest::RA as u64;
                cfe_popup_confirm_transition_in(c);
                assert_eq!(c.pc, crate::difftest::RA);
                if c.g[V0][0] == 1 {
                    return n;
                }
                assert!(n < 1000);
            }
        };
        assert_eq!(calls(60), 25);
        for hz in [120u32, 240] {
            let n = calls(hz);
            let k = hz / 60;
            assert!(n >= 24 * k - (k - 1) && n <= 25 * k, "{hz} Hz: {n} calls");
        }
    }

    /// Random registers, the game's gp, and the object (`this`) at
    /// `DATA + 0x1000` in a0. The object's memory stays zero.
    pub(crate) fn oc508_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ ((seed as u64) << 8) ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.g[A0][0] = sx(DATA + 0x1000);
        r
    }

    #[test]
    fn c_fe_popup_confirm_base_ctor_2_matches() {
        check(
            ee::s04::f_001ddbd0__generated,
            c_fe_popup_confirm_base_ctor_2,
            200,
            &|c, seed| {
                let mut r = oc508_setup(c, seed, 0x1ddbd0);
                c.bus.write32(GAME_GP + 0xda0, r.next_u32());
            },
        );
    }

    #[test]
    fn c_ui_state_buy_popup_ctor_2_matches() {
        check(
            ee::s04::f_001ddc48__generated,
            c_ui_state_buy_popup_ctor_2,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1ddc48);
            },
        );
    }

    #[test]
    fn c_ui_state_buy_popup_dtor_matches() {
        check(
            ee::s04::f_001ddc90__generated,
            c_ui_state_buy_popup_dtor,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1ddc90);
            },
        );
    }

    #[test]
    fn c_ui_state_buy_popup_set_song_credit_params_thunk_matches() {
        check(
            ee::s04::f_001ddcc8__generated,
            c_ui_state_buy_popup_set_song_credit_params_thunk,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1ddcc8);
            },
        );
    }

    #[test]
    fn c_ui_state_buy_popup_set_music_track_params_thunk_matches() {
        check(
            ee::s04::f_001ddce8__generated,
            c_ui_state_buy_popup_set_music_track_params_thunk,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1ddce8);
            },
        );
    }

    #[test]
    fn c_ui_state_buy_popup_set_reward_item_params_thunk_matches() {
        check(
            ee::s04::f_001ddd08__generated,
            c_ui_state_buy_popup_set_reward_item_params_thunk,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1ddd08);
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_ctor_2_matches() {
        check(
            ee::s04::f_001dde78__generated,
            c_fe_popup_confirm_ctor_2,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1dde78);
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_dtor_matches() {
        check(
            ee::s04::f_001ddeb8__generated,
            c_fe_popup_confirm_dtor,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1ddeb8);
            },
        );
    }

    #[test]
    fn ckeyboard_popup_ctor_2_matches() {
        covered(
            ee::s04::f_001de288__generated,
            ckeyboard_popup_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e288, 18);
            },
        );
    }

    #[test]
    fn ckeyboard_popup_get_text_length_matches() {
        covered(
            ee::s04::f_001de2e8__generated,
            ckeyboard_popup_get_text_length,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e2e8, 19);
            },
        );
    }

    #[test]
    fn ckeyboard_popup_set_key4b_enabled_matches() {
        covered(
            ee::s04::f_001de350__generated,
            ckeyboard_popup_set_key4b_enabled,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e350, 20);
            },
        );
    }

    /// The five wrappers: a `this` object, a random flag in `a1`.
    pub(crate) fn set_key_case(generated: GuestFn, hand: GuestFn, entry: u32) {
        check(generated, hand, 200, &|c, seed| {
            let mut r = machine(c, seed, entry as u64, 0);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.pc = entry;
        });
    }

    #[test]
    fn c_keyboard_popup_set_key_1c_enabled_matches() {
        set_key_case(
            ee::s04::f_001de378__generated,
            c_keyboard_popup_set_key_1c_enabled,
            0x001d_e378,
        );
    }

    #[test]
    fn c_keyboard_popup_set_key_45_enabled_matches() {
        set_key_case(
            ee::s04::f_001de3a0__generated,
            c_keyboard_popup_set_key_45_enabled,
            0x001d_e3a0,
        );
    }

    #[test]
    fn c_keyboard_popup_set_key_4f_enabled_matches() {
        set_key_case(
            ee::s04::f_001de3c8__generated,
            c_keyboard_popup_set_key_4f_enabled,
            0x001d_e3c8,
        );
    }

    #[test]
    fn c_keyboard_popup_set_key_29_enabled_matches() {
        set_key_case(
            ee::s04::f_001de3f0__generated,
            c_keyboard_popup_set_key_29_enabled,
            0x001d_e3f0,
        );
    }

    #[test]
    fn c_keyboard_popup_set_key_42_enabled_matches() {
        set_key_case(
            ee::s04::f_001de418__generated,
            c_keyboard_popup_set_key_42_enabled,
            0x001d_e418,
        );
    }

    use crate::difftest::elf_image;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;
    use crate::difftest::fakes::machine_rt;

    use crate::difftest::run_generated;
    use crate::game::hashvalue::str_hash as str_hash_m2;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Control word of this module's gated fakes (own gate, own magic).
    pub(crate) const OC929_CTRL: u32 = DATA + 0xe_0000;

    pub(crate) const OC929_MAGIC: u32 = 0x0929_f00d;

    /// What the `UIList_GetAt` fake returns for index 0..7.
    pub(crate) const OC929_GETAT: u32 = OC929_CTRL + 0x20;

    pub(crate) const OC929_THIS: u32 = DATA + 0x1000;

    pub(crate) const OC929_ARG: u32 = DATA + 0x2000;

    pub(crate) const OC929_OBJ: u32 = DATA + 0x3000;

    pub(crate) const OC929_VT: u32 = DATA + 0x4000;

    /// Empty UI list: its anchor (+4) is a self-linked sentinel.
    pub(crate) const OC929_SENT: u32 = DATA + 0x5100;

    pub(crate) const OC929_ALLOC: u32 = DATA + 0x6000;

    pub(crate) const OC929_KBD: u32 = DATA + 0x6100;

    /// Logged fake with scripted results: `UIList_GetAt` answers from the
    /// table at `OC929_GETAT`, the allocator and the keyboard constructor
    /// return fixed blocks.
    pub(crate) fn oc929_fake(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let a1 = c.g[A1][0] as u32;
        fakes::fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        match target {
            0x0039_7870 => {
                let v = c.bus.read32(OC929_GETAT + 4 * a1.min(7));
                c.g[V0][0] = v as u64;
            }
            0x0031_7d70 => c.g[V0][0] = OC929_ALLOC as u64,
            0x001c_b030 => c.g[V0][0] = OC929_KBD as u64,
            _ => {}
        }
    }

    macro_rules! oc929_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC929_CTRL) != OC929_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                oc929_fake(c, $addr);
            }
        };
    }

    oc929_gated!(
        oc929_fake_get_at,
        OC929_PREV_GET_AT,
        0x0039_7870,
        ee::s13::f_00397870__generated
    );

    oc929_gated!(
        oc929_fake_alloc,
        OC929_PREV_ALLOC,
        0x0031_7d70,
        ee::s10::f_00317d70__generated
    );

    oc929_gated!(
        oc929_fake_kbd_ctor,
        OC929_PREV_KBD_CTOR,
        0x001c_b030,
        ee::s04::f_001cb030__generated
    );

    oc929_gated!(
        oc929_fake_max_len,
        OC929_PREV_MAX_LEN,
        0x001c_d088,
        ee::s04::f_001cd088__generated
    );

    oc929_gated!(
        oc929_fake_pop,
        OC929_PREV_POP,
        0x0039_f190,
        ee::s13::f_0039f190__generated
    );

    oc929_gated!(
        oc929_fake_set_text,
        OC929_PREV_SET_TEXT,
        0x001c_b418,
        ee::s04::f_001cb418__generated
    );

    oc929_gated!(
        oc929_fake_set_static,
        OC929_PREV_SET_STATIC,
        0x001c_cf00,
        ee::s04::f_001ccf00__generated
    );

    oc929_gated!(
        oc929_fake_assign,
        OC929_PREV_ASSIGN,
        0x0031_84e8,
        ee::s10::f_003184e8__generated
    );

    oc929_gated!(
        oc929_fake_widget_text,
        OC929_PREV_WIDGET_TEXT,
        0x001c_8258,
        ee::s03::f_001c8258__generated
    );

    oc929_gated!(
        oc929_fake_open_confirm,
        OC929_PREV_OPEN_CONFIRM,
        0x001c_9b98,
        ee::s03::f_001c9b98__generated
    );

    oc929_gated!(
        oc929_fake_check_text,
        OC929_PREV_CHECK_TEXT,
        0x001c_a6a0,
        ee::s03::f_001ca6a0__generated
    );

    pub(crate) fn oc929_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00397870),
            oc929_fake_get_at,
            &OC929_PREV_GET_AT,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc929_fake_alloc,
            &OC929_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001CB030),
            oc929_fake_kbd_ctor,
            &OC929_PREV_KBD_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001CD088),
            oc929_fake_max_len,
            &OC929_PREV_MAX_LEN,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039F190),
            oc929_fake_pop,
            &OC929_PREV_POP,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001CB418),
            oc929_fake_set_text,
            &OC929_PREV_SET_TEXT,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001CCF00),
            oc929_fake_set_static,
            &OC929_PREV_SET_STATIC,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003184E8),
            oc929_fake_assign,
            &OC929_PREV_ASSIGN,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C8258),
            oc929_fake_widget_text,
            &OC929_PREV_WIDGET_TEXT,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C9B98),
            oc929_fake_open_confirm,
            &OC929_PREV_OPEN_CONFIRM,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001CA6A0),
            oc929_fake_check_text,
            &OC929_PREV_CHECK_TEXT,
        );
    }

    pub(crate) fn oc929_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    /// Machine with the game image (jump tables, strings), the gated fakes on
    /// and the shared object layout: a vtable at `OC929_VT` whose methods
    /// (`+0x58/0x5c`, `+0x88/0x8c`, `+0x98/0x9c`) are machine fakes, three
    /// objects pointing at it, an empty UI list behind the popup's state chain.
    pub(crate) fn oc929_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) {
        oc929_install_fakes();
        fakes::machine(c, seed, salt, calls);
        elf_image(c);
        oc929_w(c, OC929_CTRL, OC929_MAGIC);
        for i in 0..3 {
            oc929_w(c, OC929_OBJ + 0x100 * i + 8, OC929_VT);
        }
        for (k, off) in [0x58u32, 0x88, 0x98].into_iter().enumerate() {
            c.bus.write16(OC929_VT + off, 0);
            oc929_w(c, OC929_VT + off + 4, FAKE + 0x10 * (k as u32 + 1));
        }
        // this -> +0x40 -> +0xd0 -> +0x10 -> list at +0x18 (empty).
        oc929_w(c, OC929_THIS + 0x40, DATA + 0x5000);
        oc929_w(c, DATA + 0x5000 + 0xd0, DATA + 0x5040);
        oc929_w(c, DATA + 0x5040 + 0x10, DATA + 0x5080);
        oc929_w(c, DATA + 0x5080 + 0x18 + 4, OC929_SENT);
        oc929_w(c, OC929_SENT, OC929_SENT);
        oc929_w(c, OC929_SENT + 4, OC929_SENT);
        oc929_w(c, OC929_THIS + 0x10, DATA + 0x5080);
        oc929_w(c, OC929_THIS + 0x20, OC929_OBJ);
    }

    /// `cFEPopupConfirm_OnWidgetNotify(this, menu, event)`: the scroll-change
    /// event recolours the menu's rows (the selected one highlighted), the
    /// option/misc/previous/next events answer 0 or 0x101 from the popup's
    /// flags.
    #[test]
    fn c_fe_popup_confirm_on_widget_notify_matches() {
        check(
            ee::s03::f_001ca180__generated,
            c_fe_popup_confirm_on_widget_notify,
            216,
            &|c, seed| {
                oc929_base(c, seed, 0x1ca180, 6);
                c.pc = 0x001c_a180;
                let ev = 3 + seed % 9;
                c.g[A0][0] = OC929_THIS as u64;
                c.g[A1][0] = OC929_ARG as u64;
                c.g[A2][0] = ev as u64;
                oc929_w(c, OC929_THIS + 0x84, OC929_OBJ + 0x200);
                oc929_w(c, OC929_THIS + 0x88, OC929_OBJ + 0x100);
                oc929_w(c, OC929_THIS + 0x310, seed / 9 & 1);
                oc929_w(c, OC929_THIS + 0xb8, seed / 18 & 1);
                c.bus.write8(OC929_ARG + 0x96, (seed / 9 % 4) as u8);
                c.bus.write8(OC929_ARG + 0x95, (seed / 36 % 4) as u8);
                for i in 0..8 {
                    let o = OC929_OBJ + 0x100 * (seed >> (i + 2) & 1);
                    oc929_w(c, OC929_GETAT + 4 * i, o);
                }
            },
        );
    }

    /// `cFEPopupConfirm_OnWidgetAction(this, id, event)`: button actions
    /// (open the confirm popup or the keyboard popup, edit text, forward to
    /// the child, ...).
    #[test]
    fn c_fe_popup_confirm_on_widget_action_matches() {
        check(
            ee::s03::f_001ca298__generated,
            c_fe_popup_confirm_on_widget_action,
            420,
            &|c, seed| {
                oc929_base(c, seed, 0x1ca298, 5);
                c.pc = 0x001c_a298;
                let ev = 3 + seed % 12;
                let id = 0x1234u32;
                c.g[A0][0] = OC929_THIS as u64;
                c.g[A1][0] = id as u64;
                c.g[A2][0] = ev as u64;
                let hit = seed / 12 % 7;
                for k in 0..5u32 {
                    let v = if k == hit { id } else { id + 1 + k };
                    oc929_w(c, OC929_THIS + 0x90 + 4 * k, v);
                    oc929_w(c, OC929_THIS + 0x2d0 + 12 * k, seed >> (k + 3) & 1);
                }
                oc929_w(c, OC929_THIS + 0x30c, seed / 84 & 1);
                oc929_w(
                    c,
                    OC929_THIS + 0x88,
                    if seed / 7 & 1 == 0 { id } else { id + 0x40 },
                );
                oc929_w(c, OC929_THIS + 0x84, seed / 3 & 1);
                oc929_w(c, OC929_THIS + 0x80, seed / 5 & 1);
                oc929_w(c, OC929_THIS + 0x310, seed / 11 & 1);
                oc929_w(c, OC929_THIS + 0x328, 12);
                oc929_w(c, OC929_THIS + 0x2a8, OC929_KBD);
                oc929_w(c, OC929_THIS + 0x2b0, DATA + 0x7010);
                oc929_w(c, DATA + 0x7008, seed / 13 & 1);
            },
        );
    }

    /// `cFEPopupConfirm_OnChildEvent(this, child, event)`: the keyboard
    /// popup's events (text entered, closed, cancelled ...).
    #[test]
    fn c_fe_popup_confirm_on_child_event_matches() {
        check(
            ee::s03::f_001ca598__generated,
            c_fe_popup_confirm_on_child_event,
            240,
            &|c, seed| {
                oc929_base(c, seed, 0x1ca598, 3);
                c.pc = 0x001c_a598;
                let ev = 13 + seed % 12;
                let child = DATA + 0x6800;
                c.g[A0][0] = OC929_THIS as u64;
                c.g[A1][0] = child as u64;
                c.g[A2][0] = ev as u64;
                oc929_w(
                    c,
                    OC929_THIS + 0x2a8,
                    if seed / 12 & 1 == 0 { child } else { child + 8 },
                );
                oc929_w(
                    c,
                    OC929_THIS + 0x2ac,
                    if seed / 24 & 1 == 0 { child } else { child + 8 },
                );
                oc929_w(c, OC929_THIS + 0x2b0, DATA + 0x7010);
                oc929_w(c, OC929_THIS + 0x2b4, DATA + 0x7020);
                oc929_w(
                    c,
                    OC929_THIS + 0x320,
                    if seed / 48 & 1 == 0 { 0xffff_ffff } else { 3 },
                );
                oc929_w(c, OC929_THIS + 0x324, 7);
            },
        );
    }

    /// `cUIStateBuyPopup_SetRewardItemParams(this, kind, id, a, b)`: fills
    /// the popup's item record; the image name comes from the kind (3..8)
    /// through a jump table, hashed by the real `StrHash`.
    #[test]
    fn c_ui_state_buy_popup_set_reward_item_params_matches() {
        check(
            ee::s04::f_001caa18__generated,
            c_ui_state_buy_popup_set_reward_item_params,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x1caa18, 0);
                elf_image(c);
                c.pc = 0x001c_aa18;
                c.g[A0][0] = OC929_THIS as u64;
                c.g[A1][0] = (seed % 10) as u64;
                c.g[A2][0] = 0x55 + seed as u64;
                c.g[A3][0] = 0x66 + seed as u64;
                c.g[T0][0] = 0x77 + seed as u64;
            },
        );
    }

    /// The gate word of this module's fakes (in each machine's own RAM); the
    /// per-callee call counters, result mailboxes and the input-event result
    /// table sit behind it.
    pub(crate) const OC930_CTRL: u32 = DATA + 0x2e_0000;

    pub(crate) const OC930_MAGIC: u32 = 0x0930_0930;

    /// Result table of the input-event vtable calls, one word per event code.
    pub(crate) const OC930_EV_RES: u32 = OC930_CTRL + 0x1000;

    /// Fake targets of the input-event vtable slots (`+ 0x10 * code`).
    pub(crate) const OC930_EV_FAKE: u32 = FAKE + 0x1000;

    pub(crate) const OC930_SELF: u32 = DATA + 0x1000;

    pub(crate) const OC930_OWNER: u32 = DATA + 0x2000;

    pub(crate) const OC930_PARENT: u32 = DATA + 0x2400;

    pub(crate) const OC930_PARENT_VT: u32 = DATA + 0x2600;

    pub(crate) const OC930_EV: u32 = DATA + 0x2800;

    pub(crate) const OC930_EV_VT: u32 = DATA + 0x2a00;

    pub(crate) const OC930_OBJ: u32 = DATA + 0x3000;

    pub(crate) const OC930_OBJ_VT: u32 = DATA + 0x3200;

    pub(crate) const OC930_SND: u32 = DATA + 0x3400;

    pub(crate) const OC930_SND_VT: u32 = DATA + 0x3600;

    pub(crate) const OC930_WORLD: u32 = DATA + 0x3800;

    pub(crate) const OC930_POPUP: u32 = DATA + 0x3a00;

    pub(crate) const OC930_SCREEN: u32 = DATA + 0x3c00;

    pub(crate) const OC930_TEXT: u32 = DATA + 0x3e00;

    pub(crate) const OC930_TEXT_VT: u32 = DATA + 0x4000;

    pub(crate) const OC930_OUT: u32 = DATA + 0x4200;

    // Fake slot indexes (the call counter / mailbox of each faked callee).
    pub(crate) const FK_MOVE_LEFT: u32 = 0;

    pub(crate) const FK_MOVE_RIGHT: u32 = 1;

    pub(crate) const FK_MOVE_UP: u32 = 2;

    pub(crate) const FK_MOVE_DOWN: u32 = 3;

    pub(crate) const FK_INSERT_CHAR: u32 = 4;

    pub(crate) const FK_HANDLE_KEY_CHAR: u32 = 5;

    pub(crate) const FK_TYPE_CURRENT_KEY: u32 = 6;

    pub(crate) const FK_REFRESH_ENTRY: u32 = 7;

    pub(crate) const FK_REFRESH_KEYS: u32 = 8;

    pub(crate) const FK_SET_KEY_ENABLED: u32 = 9;

    pub(crate) const FK_IS_KEY_ENABLED: u32 = 10;

    pub(crate) const FK_QUICK_MESSAGE: u32 = 11;

    pub(crate) const FK_HELP_CTOR: u32 = 12;

    pub(crate) const FK_HELP_BUTTON: u32 = 13;

    pub(crate) const FK_HELP_NUM_BUTTONS: u32 = 14;

    pub(crate) const FK_HELP_BODY: u32 = 15;

    pub(crate) const FK_ALLOC: u32 = 16;

    pub(crate) const FK_FIND_OBJECT: u32 = 17;

    pub(crate) const FK_CLOSE_TOP: u32 = 18;

    pub(crate) const FK_SET_ASCII: u32 = 19;

    pub(crate) const FK_PRESS_KEY: u32 = 20;

    /// A fake callee: logs the call like `fakes::fake`, then (unless it
    /// unwound) returns `mail - 1` when the fixture put `mail` (non-zero) in
    /// the mailbox of this fake for this call number (16 calls, then wraps).
    pub(crate) fn oc930_fake_call(c: &mut Cpu, addr: u32, idx: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(OC930_CTRL + 0x20 + 4 * idx);
        c.bus.write32(OC930_CTRL + 0x20 + 4 * idx, n + 1);
        let mail = c.bus.read32(OC930_CTRL + 0x100 + 0x40 * idx + 4 * (n & 15));
        fakes::fake(c, addr, ra);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = (mail - 1) as u64;
        }
    }

    macro_rules! oc930_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $idx:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC930_CTRL) != OC930_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc930_fake_call(c, $addr, $idx);
                }
            )*
            pub(crate) fn oc930_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc930_fakes!(
        (
            oc930_fake_move_left,
            OC930_PREV_MOVE_LEFT,
            ee::s04::OVERRIDE_001CB500,
            ee::s04::f_001cb500__generated,
            0x001c_b500,
            FK_MOVE_LEFT
        ),
        (
            oc930_fake_move_right,
            OC930_PREV_MOVE_RIGHT,
            ee::s04::OVERRIDE_001CB650,
            ee::s04::f_001cb650__generated,
            0x001c_b650,
            FK_MOVE_RIGHT
        ),
        (
            oc930_fake_move_up,
            OC930_PREV_MOVE_UP,
            ee::s04::OVERRIDE_001CB7F0,
            ee::s04::f_001cb7f0__generated,
            0x001c_b7f0,
            FK_MOVE_UP
        ),
        (
            oc930_fake_move_down,
            OC930_PREV_MOVE_DOWN,
            ee::s04::OVERRIDE_001CB9B0,
            ee::s04::f_001cb9b0__generated,
            0x001c_b9b0,
            FK_MOVE_DOWN
        ),
        (
            oc930_fake_insert_char,
            OC930_PREV_INSERT_CHAR,
            ee::s04::OVERRIDE_001CC620,
            ee::s04::f_001cc620__generated,
            0x001c_c620,
            FK_INSERT_CHAR
        ),
        (
            oc930_fake_handle_key_char,
            OC930_PREV_HANDLE_KEY_CHAR,
            ee::s04::OVERRIDE_001CC6E8,
            ee::s04::f_001cc6e8__generated,
            0x001c_c6e8,
            FK_HANDLE_KEY_CHAR
        ),
        (
            oc930_fake_type_current_key,
            OC930_PREV_TYPE_CURRENT_KEY,
            ee::s04::OVERRIDE_001CC7C8,
            ee::s04::f_001cc7c8__generated,
            0x001c_c7c8,
            FK_TYPE_CURRENT_KEY
        ),
        (
            oc930_fake_refresh_entry,
            OC930_PREV_REFRESH_ENTRY,
            ee::s04::OVERRIDE_001CC848,
            ee::s04::f_001cc848__generated,
            0x001c_c848,
            FK_REFRESH_ENTRY
        ),
        (
            oc930_fake_refresh_keys,
            OC930_PREV_REFRESH_KEYS,
            ee::s04::OVERRIDE_001CD708,
            ee::s04::f_001cd708__generated,
            0x001c_d708,
            FK_REFRESH_KEYS
        ),
        (
            oc930_fake_set_key_enabled,
            OC930_PREV_SET_KEY_ENABLED,
            ee::s04::OVERRIDE_001CE3C8,
            ee::s04::f_001ce3c8__generated,
            0x001c_e3c8,
            FK_SET_KEY_ENABLED
        ),
        (
            oc930_fake_is_key_enabled,
            OC930_PREV_IS_KEY_ENABLED,
            ee::s04::OVERRIDE_001CE560,
            ee::s04::f_001ce560__generated,
            0x001c_e560,
            FK_IS_KEY_ENABLED
        ),
        (
            oc930_fake_quick_message,
            OC930_PREV_QUICK_MESSAGE,
            ee::s04::OVERRIDE_001CE5E0,
            ee::s04::f_001ce5e0__generated,
            0x001c_e5e0,
            FK_QUICK_MESSAGE
        ),
        (
            oc930_fake_help_ctor,
            OC930_PREV_HELP_CTOR,
            ee::s04::OVERRIDE_001D9D48,
            ee::s04::f_001d9d48__generated,
            0x001d_9d48,
            FK_HELP_CTOR
        ),
        (
            oc930_fake_help_button,
            OC930_PREV_HELP_BUTTON,
            ee::s04::OVERRIDE_001DA510,
            ee::s04::f_001da510__generated,
            0x001d_a510,
            FK_HELP_BUTTON
        ),
        (
            oc930_fake_help_num_buttons,
            OC930_PREV_HELP_NUM_BUTTONS,
            ee::s04::OVERRIDE_001DA528,
            ee::s04::f_001da528__generated,
            0x001d_a528,
            FK_HELP_NUM_BUTTONS
        ),
        (
            oc930_fake_help_body,
            OC930_PREV_HELP_BODY,
            ee::s04::OVERRIDE_001DA530,
            ee::s04::f_001da530__generated,
            0x001d_a530,
            FK_HELP_BODY
        ),
        (
            oc930_fake_alloc,
            OC930_PREV_ALLOC,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70,
            FK_ALLOC
        ),
        (
            oc930_fake_find_object,
            OC930_PREV_FIND_OBJECT,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860,
            FK_FIND_OBJECT
        ),
        (
            oc930_fake_close_top,
            OC930_PREV_CLOSE_TOP,
            ee::s13::OVERRIDE_0039F190,
            ee::s13::f_0039f190__generated,
            0x0039_f190,
            FK_CLOSE_TOP
        ),
        (
            oc930_fake_set_ascii,
            OC930_PREV_SET_ASCII,
            ee::s13::OVERRIDE_003A0C70,
            ee::s13::f_003a0c70__generated,
            0x003a_0c70,
            FK_SET_ASCII
        ),
        (
            oc930_fake_press_key,
            OC930_PREV_PRESS_KEY,
            ee::s04::OVERRIDE_001CC1E0,
            ee::s04::f_001cc1e0__generated,
            0x001c_c1e0,
            FK_PRESS_KEY
        ),
    );

    /// Mailbox word of fake `idx` for call number `n`: the fake returns `value`.
    pub(crate) fn oc930_mail(c: &mut Cpu, idx: u32, n: u32, value: u32) {
        c.bus
            .write32(OC930_CTRL + 0x100 + 0x40 * idx + 4 * n, value + 1);
    }

    /// The `miss` hook: the input-event vtable slots (`OC930_EV_FAKE + 0x10 *
    /// code`) return the fixture's per-code result, everything else is the
    /// plain logging fake.
    pub(crate) fn oc930_miss(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, target, ra);
        if c.pc == ra && (OC930_EV_FAKE..OC930_EV_FAKE + 0x400).contains(&target) {
            let code = (target - OC930_EV_FAKE) / 0x10;
            let res = c.bus.read32(OC930_EV_RES + 4 * code);
            c.g[V0][0] = res as u64;
        }
    }

    /// A vtable at `vt` whose slots `offs` hold `{delta, FAKE + 0x10 * i}`.
    pub(crate) fn oc930_vtable(c: &mut Cpu, r: &mut Rng, vt: u32, offs: &[u32]) {
        for (i, &off) in offs.iter().enumerate() {
            let delta = (r.next_u32() % 8) * 4;
            c.bus.write16(vt + off, delta as u16);
            c.bus.write32(vt + off + 4, FAKE + 0x10 * i as u32);
        }
    }

    /// A machine with the game image (jump tables, strings, constants), the
    /// fakes' gate on and the event-aware miss hook.
    pub(crate) fn oc930_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc930_install_fakes();
        let r = machine(c, seed, salt, calls);
        elf_image(c);
        c.hooks.miss = oc930_miss;
        c.bus.write32(OC930_CTRL, OC930_MAGIC);
        r
    }

    /// The popup object with its owner (sound object, empty state list), the
    /// parent (vtable slot 0x98), the screen and the world flags.
    pub(crate) fn oc930_popup(c: &mut Cpu, r: &mut Rng, seed: u32, key: u32) {
        let s = OC930_SELF;
        c.bus.write32(s + 0x10, OC930_OWNER);
        c.bus.write32(s + 0x20, OC930_PARENT);
        c.bus.write32(s + 0x40, OC930_SCREEN);
        c.bus.write32(s + 0x140, key);
        c.bus.write32(OC930_PARENT + 8, OC930_PARENT_VT);
        oc930_vtable(c, r, OC930_PARENT_VT, &[0x98]);
        // The owner's state list, as `UIList_Reset` leaves it (empty).
        let l = OC930_OWNER + 0x18;
        c.bus.write32(l, l);
        c.bus.write32(l + 4, l + 0xc);
        c.bus.write32(l + 0xc, l);
        c.bus.write32(l + 0x10, l + 0xc);
        // The feedback sound object: the owner's (+0x14), or the game's own
        // at gp+0x2968 when the owner has none.
        c.bus.write32(
            OC930_OWNER + 0x14,
            if seed % 4 == 0 { 0 } else { OC930_SND },
        );
        c.bus.write32(OC930_SND, OC930_SND_VT);
        c.bus.write32(GAME_GP + 0x2968, OC930_SND_VT);
        oc930_vtable(c, r, OC930_SND_VT, &[0x10]);
        // The found key widget: vtable slot 0x58 (set colour).
        c.bus.write32(OC930_OBJ + 8, OC930_OBJ_VT);
        oc930_vtable(c, r, OC930_OBJ_VT, &[0x58]);
        c.bus.write32(OC930_WORLD + 0x84, seed % 2);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC930_WORLD);
    }

    // ---------------------------------------------------------------------
    // cKeyboardPopup_GetKeyWidgetName
    // ---------------------------------------------------------------------

    /// Every key 0..0x60 (special keys by table, digit keys through
    /// `sprintf`, characters from the key character table, `*` keys that
    /// return 0, keys past the table) with and without the shifted prefix.
    #[test]
    fn c_keyboard_popup_get_key_widget_name_matches() {
        check(
            ee::s04::f_001cd0d8__generated,
            c_keyboard_popup_get_key_widget_name,
            194,
            &|c, seed| {
                let _ = machine(c, seed, 0x1cd0d8, 0);
                elf_image(c);
                c.pc = 0x001c_d0d8;
                c.g[A0][0] = sx(OC930_SELF);
                c.g[A1][0] = (seed % 97) as u64;
                c.g[A2][0] = sx(OC930_OUT);
                c.g[A3][0] = (seed / 97) as u64;
            },
        );
    }

    /// The generated body takes many different paths over the fixtures.
    #[test]
    fn c_keyboard_popup_get_key_widget_name_paths() {
        oc930_paths(ee::s04::f_001cd0d8__generated, 194, 15, &|c, seed| {
            let _ = machine(c, seed, 0x1cd0d8, 0);
            elf_image(c);
            c.pc = 0x001c_d0d8;
            c.g[A0][0] = sx(OC930_SELF);
            c.g[A1][0] = (seed % 97) as u64;
            c.g[A2][0] = sx(OC930_OUT);
            c.g[A3][0] = (seed / 97) as u64;
        });
    }

    /// Distinct cycle totals of the generated body over `seeds` fixtures
    /// (no unwinds): at least `min` paths must be taken.
    pub(crate) fn oc930_paths(
        generated: fn(&mut Cpu),
        seeds: u32,
        min: usize,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= min, "only {} paths: {seen:?}", seen.len());
    }

    // ---------------------------------------------------------------------
    // cKeyboardPopup_SetSpecialKeyLabel
    // ---------------------------------------------------------------------

    pub(crate) const OC930_LABEL_KEYS: [u32; 30] = [
        27, 42, 56, 70, 28, 55, 75, 0, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 83, 1, 2, 3, 14, 26, 29,
        0x54, 0x60, 76, 69, 79,
    ];

    pub(crate) fn oc930_label_fixture(c: &mut Cpu, seed: u32, calls: u32) {
        let mut r = oc930_machine(c, seed, 0x1cd9a0, calls);
        let key = OC930_LABEL_KEYS[(seed % 30) as usize];
        let enabled = (seed / 30) % 2;
        oc930_mail(c, FK_IS_KEY_ENABLED, 0, enabled);
        c.bus
            .write32(OC930_SELF + 0x43c, if (seed / 60) % 2 == 0 { 1 } else { 2 });
        c.bus.write32(OC930_TEXT + 8, OC930_TEXT_VT);
        oc930_vtable(c, &mut r, OC930_TEXT_VT, &[0x28, 0x48, 0x58]);
        c.pc = 0x001c_d9a0;
        c.g[A0][0] = sx(OC930_SELF);
        c.g[A1][0] = if seed % 47 == 3 { 0 } else { sx(OC930_TEXT) };
        c.g[A2][0] = key as u64;
    }

    /// Colour (enabled or disabled), then the label of every special key by
    /// text id, the digit keys by caps state and `sprintf`; null text exits.
    #[test]
    fn c_keyboard_popup_set_special_key_label_matches() {
        check(
            ee::s04::f_001cd9a0__generated,
            c_keyboard_popup_set_special_key_label,
            240,
            &|c, seed| oc930_label_fixture(c, seed, 6),
        );
    }

    #[test]
    fn c_keyboard_popup_set_special_key_label_paths() {
        oc930_paths(ee::s04::f_001cd9a0__generated, 240, 10, &|c, seed| {
            oc930_label_fixture(c, seed, 0)
        });
    }

    // ---------------------------------------------------------------------
    // cKeyboardPopup_PressKey
    // ---------------------------------------------------------------------

    pub(crate) const OC930_PRESS_KEYS: [u32; 32] = [
        27, 28, 69, 79, 42, 56, 70, 76, 83, 55, 75, 0, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 30, 1, 2,
        3, 14, 0x37, 0x46, 0x54, 0x60, 0x38,
    ];

    pub(crate) fn oc930_press_fixture(c: &mut Cpu, seed: u32, calls: u32) {
        let mut r = oc930_machine(c, seed, 0x1cc1e0, calls);
        let v = seed / 32;
        let key = OC930_PRESS_KEYS[(seed % 32) as usize];
        oc930_popup(c, &mut r, seed, key);
        let s = OC930_SELF;
        // The entry text and the cursor, selection mark and end.
        let len = (v + key) % 5;
        for i in 0..len {
            c.bus.write8(s + 0x74 + i, b'a' + i as u8);
        }
        let cursor = r.next_u32() % (len + 2);
        let mark = [cursor, cursor + 1, cursor.saturating_sub(1), 3][(r.next_u32() % 4) as usize];
        c.bus.write32(s + 0x134, cursor);
        c.bus.write32(s + 0x138, mark);
        c.bus.write32(s + 0x13c, r.next_u32() % 7);
        c.bus.write32(s + 0x60, 1 + r.next_u32() % 7);
        c.bus.write32(s + 0x64, r.next_u32() % 4);
        c.bus.write32(s + 0x68, r.next_u32() & 1);
        c.bus.write32(s + 0x6c, r.next_u32() & 1);
        c.bus.write32(s + 0x58, r.next_u32() & 1);
        c.bus.write32(s + 0x5c, r.next_u32() & 1);
        c.bus.write32(s + 0x420, r.next_u32() & 1);
        // A pending accept makes the key press itself first (key 0x46).
        c.bus.write32(s + 0x54, (v % 3 == 1) as u32);
        oc930_mail(c, FK_IS_KEY_ENABLED, 0, (v % 4 != 0) as u32);
        oc930_mail(c, FK_IS_KEY_ENABLED, 1, ((v / 2) % 2) as u32);
        oc930_mail(c, FK_FIND_OBJECT, 0, if v % 5 == 2 { 0 } else { OC930_OBJ });
        c.pc = 0x001c_c1e0;
        c.g[A0][0] = sx(s);
        c.g[A1][0] = (seed % 13 == 0) as u64;
    }

    /// Every key action of the keyboard (backspace, caps, shift, clear, left,
    /// right, accept, quick messages, typed characters), with the key
    /// enabled or not, a pending accept, and the entry-text refresh.
    #[test]
    fn c_keyboard_popup_press_key_matches() {
        check(
            ee::s04::f_001cc1e0__generated,
            c_keyboard_popup_press_key,
            320,
            &|c, seed| oc930_press_fixture(c, seed, 12),
        );
    }

    #[test]
    fn c_keyboard_popup_press_key_paths() {
        oc930_paths(ee::s04::f_001cc1e0__generated, 320, 20, &|c, seed| {
            oc930_press_fixture(c, seed, 0)
        });
    }

    // ---------------------------------------------------------------------
    // cKeyboardPopup_HandleInput
    // ---------------------------------------------------------------------

    /// `switch` values of event 42 (a PC key): the mapped keys, the F-key
    /// range, and unmapped values (default: typed characters).
    pub(crate) const OC930_PC_KEYS: [u32; 30] = [
        8, 0xd, 0x10, 0x14, 0x1b, 0x25, 0x27, 0x26, 0x28, 9, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75,
        0x76, 0x77, 0x78, 0x79, 0, 0x7a, 0x30, 0x7f, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
    ];

    pub(crate) fn oc930_input_fixture(c: &mut Cpu, seed: u32, calls: u32) {
        let mut r = oc930_machine(c, seed, 0x1cdb98, calls);
        let scn = seed % 44;
        let v = seed / 44;
        let key = r.next_u32() % 0x54;
        oc930_popup(c, &mut r, seed, key);
        let s = OC930_SELF;
        c.bus.write32(s + 0x68, r.next_u32() & 1);
        c.bus.write32(s + 0x6c, r.next_u32() & 1);
        // The event: a vtable with one slot per event code (offset 8 * code).
        c.bus.write32(OC930_EV + 8, OC930_EV_VT);
        let codes = [8u32, 19, 20, 17, 18, 5, 7, 6, 25, 27, 41, 42, 43];
        for &code in &codes {
            let delta = (r.next_u32() % 4) * 4;
            c.bus.write16(OC930_EV_VT + 8 * code, delta as u16);
            c.bus
                .write32(OC930_EV_VT + 8 * code + 4, OC930_EV_FAKE + 0x10 * code);
        }
        let mut res = [0u32; 64];
        let mut isonflag = 0;
        match scn {
            0 => res[8] = 1,
            1 => res[19] = 1,
            2 => res[20] = 1,
            3 => res[17] = 1,
            4 => res[18] = 1,
            5 => res[5] = 1,
            6..=9 => {
                // Help: the isOn flag (gp-0x1680) and the world's +0x84 word.
                res[7] = 1;
                isonflag = (scn - 6) & 1;
                c.bus.write32(OC930_WORLD + 0x84, (scn - 6) >> 1);
            }
            10 => res[6] = 1,
            11 => res[25] = 1,
            12 => res[27] = 1,
            13 => {}
            _ => {
                res[41] = 1;
                res[42] = OC930_PC_KEYS[(scn - 14) as usize];
                res[43] = [0x41u32, 0x60, 0x1f, 0x80, 0x7e, 0x2a][((v + scn) % 6) as usize];
            }
        }
        c.bus.write32(GAME_GP.wrapping_sub(0x1680), isonflag);
        for (i, w) in res.iter().enumerate() {
            c.bus.write32(OC930_EV_RES + 4 * i as u32, *w);
        }
        // Callee results: IsKeyEnabled by call number, HandleKeyChar, the
        // help popup object, the found key widget (before / after).
        for n in 0..4 {
            oc930_mail(c, FK_IS_KEY_ENABLED, n, (v >> n) & 1);
        }
        oc930_mail(c, FK_HANDLE_KEY_CHAR, 0, (v >> 1) & 1);
        oc930_mail(c, FK_HELP_CTOR, 0, OC930_POPUP);
        oc930_mail(c, FK_FIND_OBJECT, 0, if v % 2 == 0 { OC930_OBJ } else { 0 });
        oc930_mail(c, FK_FIND_OBJECT, 1, if v % 3 == 0 { 0 } else { OC930_OBJ });
        c.pc = 0x001c_db98;
        c.g[A0][0] = sx(s);
        c.g[A1][0] = sx(OC930_EV);
    }

    /// Cursor moves, press, help popup, close, modifier toggles, PC keys
    /// (mapped, F-keys, typed characters), with and without the owner's
    /// sound object; returns 1.
    #[test]
    fn c_keyboard_popup_handle_input_matches() {
        check(
            ee::s04::f_001cdb98__generated,
            c_keyboard_popup_handle_input,
            264,
            &|c, seed| oc930_input_fixture(c, seed, 24),
        );
    }

    #[test]
    fn c_keyboard_popup_handle_input_paths() {
        oc930_paths(ee::s04::f_001cdb98__generated, 264, 25, &|c, seed| {
            oc930_input_fixture(c, seed, 0)
        });
    }

    /// Gate word of this module's fakes (in the machine's RAM) and its magic;
    /// the mask word after it says which `UIScreen_FindObject` calls find a widget.
    pub(crate) const OC932_CTRL: u32 = DATA + 0x20_4000;

    pub(crate) const OC932_MAGIC: u32 = 0x0932_f00d;

    /// The popup object, its widgets, the shared widget vtable, the config/info block.
    pub(crate) const OC932_SELF: u32 = DATA + 0x20_0000;

    pub(crate) const OC932_WID: u32 = DATA + 0x20_1000;

    pub(crate) const OC932_VT: u32 = DATA + 0x20_2000;

    pub(crate) const OC932_CFG: u32 = DATA + 0x20_3000;

    /// A gated fake callee at `addr`: logs the call like `fakes::fake_call`;
    /// `UIScreen_FindObject` (0x39d860) then answers a widget or null from
    /// the mask, by the index of the call.
    pub(crate) fn oc932_fake_call(c: &mut Cpu, addr: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if addr == 0x0039_d860 && c.pc == ra {
            let mask = c.bus.read32(OC932_CTRL + 4);
            c.g[V0][0] = if (mask >> (n % 32)) & 1 != 0 {
                oc932_widget(n) as u64
            } else {
                0
            };
        }
    }

    pub(crate) fn oc932_widget(i: u32) -> u32 {
        OC932_WID + 0x100 * (i % 8)
    }

    macro_rules! oc932_fakes {
        ($($fake:ident, $prev:ident, $slot:path, $gen:path, $addr:literal;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC932_CTRL) != OC932_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc932_fake_call(c, $addr);
                }
            )*
            fn oc932_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc932_fakes! {
        oc932_f_317670, OC932_P_317670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated, 0x0031_7670;
        oc932_f_39d860, OC932_P_39D860, ee::s13::OVERRIDE_0039D860, ee::s13::f_0039d860__generated, 0x0039_d860;
        oc932_f_417828, OC932_P_417828, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated, 0x0041_7828;
        oc932_f_1ca488, OC932_P_1CA488, ee::s03::OVERRIDE_001CA488, ee::s03::f_001ca488__generated, 0x001c_a488;
        oc932_f_1ca4c0, OC932_P_1CA4C0, ee::s03::OVERRIDE_001CA4C0, ee::s03::f_001ca4c0__generated, 0x001c_a4c0;
        oc932_f_1c6230, OC932_P_1C6230, ee::s03::OVERRIDE_001C6230, ee::s03::f_001c6230__generated, 0x001c_6230;
        oc932_f_1c66e8, OC932_P_1C66E8, ee::s03::OVERRIDE_001C66E8, ee::s03::f_001c66e8__generated, 0x001c_66e8;
        oc932_f_1c58d0, OC932_P_1C58D0, ee::s03::OVERRIDE_001C58D0, ee::s03::f_001c58d0__generated, 0x001c_58d0;
        oc932_f_1c8678, OC932_P_1C8678, ee::s03::OVERRIDE_001C8678, ee::s03::f_001c8678__generated, 0x001c_8678;
        oc932_f_1c7258, OC932_P_1C7258, ee::s03::OVERRIDE_001C7258, ee::s03::f_001c7258__generated, 0x001c_7258;
        oc932_f_1c7388, OC932_P_1C7388, ee::s03::OVERRIDE_001C7388, ee::s03::f_001c7388__generated, 0x001c_7388;
        oc932_f_1c8758, OC932_P_1C8758, ee::s03::OVERRIDE_001C8758, ee::s03::f_001c8758__generated, 0x001c_8758;
        oc932_f_1c88c8, OC932_P_1C88C8, ee::s03::OVERRIDE_001C88C8, ee::s03::f_001c88c8__generated, 0x001c_88c8;
        oc932_f_1c7040, OC932_P_1C7040, ee::s03::OVERRIDE_001C7040, ee::s03::f_001c7040__generated, 0x001c_7040;
        oc932_f_1c9cd0, OC932_P_1C9CD0, ee::s03::OVERRIDE_001C9CD0, ee::s03::f_001c9cd0__generated, 0x001c_9cd0;
        oc932_f_1c7c20, OC932_P_1C7C20, ee::s03::OVERRIDE_001C7C20, ee::s03::f_001c7c20__generated, 0x001c_7c20;
        oc932_f_1c7eb0, OC932_P_1C7EB0, ee::s03::OVERRIDE_001C7EB0, ee::s03::f_001c7eb0__generated, 0x001c_7eb0;
        oc932_f_1ca6a0, OC932_P_1CA6A0, ee::s03::OVERRIDE_001CA6A0, ee::s03::f_001ca6a0__generated, 0x001c_a6a0;
        oc932_f_1c8050, OC932_P_1C8050, ee::s03::OVERRIDE_001C8050, ee::s03::f_001c8050__generated, 0x001c_8050;
        oc932_f_1c80e8, OC932_P_1C80E8, ee::s03::OVERRIDE_001C80E8, ee::s03::f_001c80e8__generated, 0x001c_80e8;
        oc932_f_1c8320, OC932_P_1C8320, ee::s03::OVERRIDE_001C8320, ee::s03::f_001c8320__generated, 0x001c_8320;
        oc932_f_1c8628, OC932_P_1C8628, ee::s03::OVERRIDE_001C8628, ee::s03::f_001c8628__generated, 0x001c_8628;
        oc932_f_1c9b68, OC932_P_1C9B68, ee::s03::OVERRIDE_001C9B68, ee::s03::f_001c9b68__generated, 0x001c_9b68;
    }

    /// The popup world: scrambled machine with the fakes' gate open, eight
    /// widgets (random colour/position data, one shared vtable whose slots
    /// are all fake calls), and a popup object of random words. `mask`
    /// (seed-driven: all, none or random bits) says which widget fields of
    /// the popup and which `FindObject` calls are non-null.
    pub(crate) fn oc932_world(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> (Rng, u32) {
        oc932_install_fakes();
        let mut r = machine(c, seed, salt, calls);
        let mask = match seed % 4 {
            0 => !0,
            1 => 0,
            _ => r.next_u32(),
        };
        c.bus.write32(OC932_CTRL, OC932_MAGIC);
        c.bus.write32(OC932_CTRL + 4, mask);
        for i in 0..0x40u32 {
            c.bus
                .write16(OC932_VT + 8 * i, (0x10 * (r.next_u32() % 3)) as u16);
            c.bus.write32(OC932_VT + 8 * i + 4, FAKE + 0x10 * (i % 4));
        }
        for k in 0..8u32 {
            let w = oc932_widget(k);
            fill(c, &mut r, w, 0x40);
            c.bus.write32(w + 8, OC932_VT);
        }
        fill(c, &mut r, OC932_SELF, 0x100);
        // Globals the popup code copies (colours, default position).
        fill(c, &mut r, 0x004c_6700, 0x80);
        fill(c, &mut r, 0x004f_f0d8, 4);
        c.g[A0][0] = sx(OC932_SELF);
        c.pc = 0;
        (r, mask)
    }

    /// Store widget pointer `i` (or null by the mask) at `OC932_SELF + off`.
    pub(crate) fn oc932_field(c: &mut Cpu, mask: u32, off: u32, i: u32) {
        let v = if (mask >> (i % 32)) & 1 != 0 {
            oc932_widget(i)
        } else {
            0
        };
        c.bus.write32(OC932_SELF + off, v);
    }

    /// `SetFade(self, t)`: the five widgets, three looked-up ones and the
    /// five option widgets each get alpha `t` (null ones are skipped).
    #[test]
    fn c_fe_popup_confirm_set_fade_matches() {
        covered(
            ee::s03::f_001c5f20__generated,
            c_fe_popup_confirm_set_fade,
            96,
            &|c, seed| {
                let (mut r, mask) = oc932_world(c, seed, 0x1c5f20, 22);
                for (i, off) in [
                    0x70u32, 0x74, 0x78, 0x7c, 0x88, 0x90, 0x94, 0x98, 0x9c, 0xa0,
                ]
                .into_iter()
                .enumerate()
                {
                    oc932_field(c, mask.rotate_left(seed), off, i as u32);
                }
                c.f[12] = r.f32(2.0);
                c.pc = 0x001c_5f20;
            },
        );
    }

    /// `FindWidgets(self)`: every named widget found or missing (by call
    /// index), with the hash, find and rect calls unwinding at random.
    #[test]
    fn c_fe_popup_confirm_find_widgets_matches() {
        covered(
            ee::s03::f_001c6230__generated,
            c_fe_popup_confirm_find_widgets,
            120,
            &|c, seed| {
                let _ = oc932_world(c, seed, 0x1c6230, 70);
                c.pc = 0x001c_6230;
            },
        );
    }

    /// `ApplyBackgroundStyle(self, style)`: the style word (+0x31c) takes
    /// each of 0..3, with text widgets present or missing.
    #[test]
    fn c_fe_popup_confirm_apply_background_style_matches() {
        covered(
            ee::s03::f_001c66e8__generated,
            c_fe_popup_confirm_apply_background_style,
            96,
            &|c, seed| {
                let (mut r, mask) = oc932_world(c, seed, 0x1c66e8, 20);
                c.bus.write32(OC932_SELF + 0x31c, (seed / 4) % 4);
                for (i, off) in [0x74u32, 0x78, 0x7c, 0x88].into_iter().enumerate() {
                    oc932_field(c, mask.rotate_left(seed), off, i as u32);
                }
                c.g[A1][0] = r.next_u32() as u64;
                c.pc = 0x001c_66e8;
            },
        );
    }

    /// `ApplyConfig(self, cfg)`: cfg[0] == -1 or not, fade float zero or not,
    /// +0xbc zero or not, +0x90 zero or not (bits of the seed).
    #[test]
    fn c_fe_popup_confirm_apply_config_matches() {
        covered(
            ee::s03::f_001c6b08__generated,
            c_fe_popup_confirm_apply_config,
            128,
            &|c, seed| {
                let (mut r, _) = oc932_world(c, seed, 0x1c6b08, 26);
                fill(c, &mut r, OC932_CFG, 0x40);
                if seed & 1 != 0 {
                    c.bus.write32(OC932_CFG, 0xffff_ffff);
                }
                if seed & 2 != 0 {
                    c.bus.write32(OC932_CFG + 0xb0, 0);
                }
                if seed & 4 != 0 {
                    c.bus.write32(OC932_CFG + 0xbc, 0);
                }
                if seed & 8 != 0 {
                    c.bus.write32(OC932_CFG + 0x90, 0);
                }
                c.g[A1][0] = sx(OC932_CFG);
                c.pc = 0x001c_6b08;
            },
        );
    }

    /// `LayoutTextEntry(self, info)`: first call (info[0] == -1: lay out and
    /// show the four entry widgets) and later calls (hide them), widgets
    /// present or missing.
    #[test]
    fn c_fe_popup_confirm_layout_text_entry_matches() {
        covered(
            ee::s03::f_001c6d30__generated,
            c_fe_popup_confirm_layout_text_entry,
            128,
            &|c, seed| {
                let (mut r, mask) = oc932_world(c, seed, 0x1c6d30, 24);
                fill(c, &mut r, OC932_CFG, 0x40);
                c.bus
                    .write32(OC932_CFG, if seed & 1 != 0 { 0xffff_ffff } else { 0 });
                for (i, off) in [0x80u32, 0x84, 0x88, 0x8c].into_iter().enumerate() {
                    oc932_field(c, mask.rotate_left(seed / 2), off, i as u32);
                }
                c.g[A1][0] = sx(OC932_CFG);
                c.pc = 0x001c_6d30;
            },
        );
    }

    /// Gate word of this module's callee fakes (in the machine's own RAM) and
    /// its magic value; mailbox words follow at `+4`, `+8`, ...
    pub(crate) const OC933_CTRL: u32 = DATA + 0x7f00;

    pub(crate) const OC933_MAGIC: u32 = 0x0933_c0de;

    /// A gated fake of a direct callee: logs the call like `fakes::fake_call`
    /// and, when `$mailbox` (an offset from `OC933_CTRL`, 0 for none) is
    /// given, returns the mailbox word in `v0`.
    macro_rules! oc933_fake {
        ($fake:ident, $prev:ident, $addr:expr, $gen:path, $mailbox:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC933_CTRL) != OC933_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
                let mailbox: u32 = $mailbox;
                if mailbox != 0 {
                    let word = c.bus.read32(OC933_CTRL + mailbox);
                    c.g[V0][0] = sx(word);
                }
            }
        };
    }

    oc933_fake!(
        oc933_fake_layout_text_entry,
        OC933_PREV_LTE,
        0x001c_6d30,
        ee::s03::f_001c6d30__generated,
        4
    );

    oc933_fake!(
        oc933_fake_set_text_from_desc,
        OC933_PREV_STFD,
        0x001c_8050,
        ee::s03::f_001c8050__generated,
        0
    );

    oc933_fake!(
        oc933_fake_measure_desc,
        OC933_PREV_MD,
        0x001c_8320,
        ee::s03::f_001c8320__generated,
        0
    );

    oc933_fake!(
        oc933_fake_compute_bg_scale,
        OC933_PREV_CBS,
        0x001c_7620,
        ee::s03::f_001c7620__generated,
        0
    );

    oc933_fake!(
        oc933_fake_position_bg,
        OC933_PREV_PBG,
        0x001c_78c0,
        ee::s03::f_001c78c0__generated,
        0
    );

    oc933_fake!(
        oc933_fake_set_widget_align,
        OC933_PREV_SWA,
        0x001c_8930,
        ee::s03::f_001c8930__generated,
        0
    );

    oc933_fake!(
        oc933_fake_resize_text,
        OC933_PREV_RTW,
        0x001c_70f0,
        ee::s03::f_001c70f0__generated,
        0
    );

    oc933_fake!(
        oc933_fake_adjust_layer,
        OC933_PREV_ADL,
        0x001c_9cd0,
        ee::s03::f_001c9cd0__generated,
        0
    );

    pub(crate) fn oc933_install() {
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C6D30),
            oc933_fake_layout_text_entry,
            &OC933_PREV_LTE,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C8050),
            oc933_fake_set_text_from_desc,
            &OC933_PREV_STFD,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C8320),
            oc933_fake_measure_desc,
            &OC933_PREV_MD,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C7620),
            oc933_fake_compute_bg_scale,
            &OC933_PREV_CBS,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C78C0),
            oc933_fake_position_bg,
            &OC933_PREV_PBG,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C8930),
            oc933_fake_set_widget_align,
            &OC933_PREV_SWA,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C70F0),
            oc933_fake_resize_text,
            &OC933_PREV_RTW,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C9CD0),
            oc933_fake_adjust_layer,
            &OC933_PREV_ADL,
        );
    }

    /// A vtable at `vt` whose slots `offs` hold `{delta, FAKE + 0x10 * k}`.
    pub(crate) fn oc933_vtable(c: &mut Cpu, vt: u32, offs: &[u32], delta: u16) {
        for (k, off) in offs.iter().enumerate() {
            c.bus.write16(vt + off, delta);
            c.bus.write32(vt + off + 4, FAKE + 0x10 * k as u32);
        }
    }

    /// A widget at `at` with its own vtable (slots 0x40 setStatic, 0x48
    /// setVisible, 0xa0 size) and random words in `+0..0x60`.
    pub(crate) fn oc933_widget(c: &mut Cpu, r: &mut Rng, at: u32, delta: u16) {
        fill(c, r, at, 0x18);
        let vt = at + 0x1000;
        c.bus.write32(at + 8, vt);
        oc933_vtable(c, vt, &[0x40, 0x48, 0xa0], delta);
    }

    // -- 0x001c7388 cFEPopupConfirm_LayoutOptions ----------------------

    #[test]
    fn c_fe_popup_confirm_layout_options_matches() {
        // Option counts 0..6 (flag 8 set / cleared, +0x70 shown or hidden,
        // used and unused widgets), absent widgets, wider / narrower
        // measured widths, and both answers of the text-entry layout.
        check(
            ee::s03::f_001c7388__generated,
            c_fe_popup_confirm_layout_options,
            140,
            &|c, seed| {
                oc933_install();
                let mut r = fakes::machine(c, seed, 0x1c7388, 16);
                c.bus.write32(OC933_CTRL, OC933_MAGIC);
                c.bus
                    .write32(OC933_CTRL + 4, if seed % 2 == 0 { 0 } else { 5 });
                let this = DATA + 0x1000;
                let info = DATA + 0x2000;
                let bg = DATA + 0x2400;
                let counts = [0u32, 1, 2, 3, 4, 5, 6];
                let count = counts[(seed as usize / 2) % counts.len()];
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(info);
                c.bus.write32(info, count);
                fill(c, &mut r, info + 4, 0x40);
                fill(c, &mut r, this + 0x260, 4);
                c.bus.write32(this + 0x264, r.f32(20.0));
                c.bus.write32(this + 0x29c, r.f32(100.0));
                c.bus.write32(this + 0x6c, bg);
                c.bus.write32(bg + 0x90, r.next_u32());
                let shown = DATA + 0x2800;
                oc933_widget(c, &mut r, shown, 0);
                c.bus.write32(this + 0x70, shown);
                for i in 0..5u32 {
                    let present = (seed >> (i + 1)) & 3 != 0;
                    let w = DATA + 0x3000 + 0x100 * i;
                    c.bus
                        .write32(this + 0x90 + 4 * i, if present { w } else { 0 });
                    if present {
                        oc933_widget(c, &mut r, w, (4 * (seed % 3)) as u16);
                    }
                }
                // The measured item size the faked `MeasureDesc` leaves on the stack.
                let frame = c.g[SP][0] as u32 - 0xa0;
                c.bus.write32(frame + 0x18, r.f32(100.0));
                fill(c, &mut r, frame, 4);
            },
        );
    }

    // -- 0x001c7c20 cFEPopupConfirm_LayoutFrame ------------------------

    #[test]
    fn c_fe_popup_confirm_layout_frame_matches() {
        // Each of the five widgets present or absent, `+0x2cc` set (x = centred
        // + 320) or clear (centred x added); the background scaling and
        // positioning callees are faked (gated), `GetCenteredX` runs for real.
        check(
            ee::s03::f_001c7c20__generated,
            c_fe_popup_confirm_layout_frame,
            120,
            &|c, seed| {
                oc933_install();
                let mut r = fakes::machine(c, seed, 0x1c7c20, 3);
                c.bus.write32(OC933_CTRL, OC933_MAGIC);
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                fill(c, &mut r, this + 0x1b8, 4);
                for off in [0x260u32, 0x264, 0x268, 0x26c, 0x270] {
                    c.bus.write32(this + off, r.f32(300.0));
                }
                c.bus.write32(this + 0x2cc, seed % 2);
                for (k, off) in [0x6cu32, 0x70, 0x74, 0x7c, 0x78].into_iter().enumerate() {
                    let w = DATA + 0x3000 + 0x80 * k as u32;
                    let present = (seed >> (k + 1)) % 4 != 0;
                    c.bus.write32(this + off, if present { w } else { 0 });
                    fill(c, &mut r, w + 0x44, 3);
                }
            },
        );
    }

    // -- 0x001c8a38 cFEPopupConfirm_ResetLayout ------------------------

    #[test]
    fn c_fe_popup_confirm_reset_layout_matches() {
        // Every widget slot present or absent (the scale / position copies and
        // the `SetWidgetAlign` calls of the three text widgets and of the
        // option widgets); the aligning, resizing and layer callees are faked.
        check(
            ee::s03::f_001c8a38__generated,
            c_fe_popup_confirm_reset_layout,
            140,
            &|c, seed| {
                oc933_install();
                let mut r = fakes::machine(c, seed, 0x1c8a38, 6);
                c.bus.write32(OC933_CTRL, OC933_MAGIC);
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                fill(c, &mut r, this + 0x180, 0x80);
                c.bus.write32(this + 0x29c, r.f32(80.0));
                c.bus.write32(this + 0x314, r.next_u32());
                let slots = [
                    0x48u32, 0x4c, 0x50, 0x54, 0x58, 0x5c, 0x60, 0x8c, 0x6c, 0x70, 0x74, 0x7c,
                    0x78, 0x80, 0x88, 0x90, 0x94, 0x98, 0x9c, 0xa0,
                ];
                let mask = r.next_u32() ^ seed.wrapping_mul(0x9e37_79b9);
                for (k, off) in slots.into_iter().enumerate() {
                    // Seeds 0 and 1 run with everything present / absent.
                    let present = match seed {
                        0 => true,
                        1 => false,
                        _ => (mask >> k) & 1 != 0,
                    };
                    let w = DATA + 0x3000 + 0x80 * k as u32;
                    c.bus.write32(this + off, if present { w } else { 0 });
                    fill(c, &mut r, w + 0x44, 0x10);
                }
            },
        );
    }

    // -- 0x001c9cd0 cFEPopupConfirm_AdjustDrawLayer --------------------

    /// `miss` hook of the AdjustDrawLayer test: the screen's object finder
    /// (`0x39d860`, call index 1, 3, 5) answers with the mailbox word.
    pub(crate) fn oc933_adjust_miss(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake_call(c, target);
        if target == 0x0039_d860 {
            let found = c.bus.read32(OC933_CTRL + 0x10 + 4 * n);
            c.g[V0][0] = sx(found);
        }
    }

    #[test]
    fn c_fe_popup_confirm_adjust_draw_layer_matches() {
        // Each widget present or absent, each hash-named screen object found
        // or not, positive and negative layer deltas (the 5-bit layer wraps).
        check(
            ee::s03::f_001c9cd0__generated,
            c_fe_popup_confirm_adjust_draw_layer,
            140,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1c9cd0, 6);
                c.hooks.miss = oc933_adjust_miss;
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                let deltas = [1u32, 5, 0, 0xffff_ffff, 31, 0xffff_ffe1, 17, 3];
                c.g[A1][0] = sx(deltas[seed as usize % deltas.len()]);
                c.bus.write32(this + 0x40, DATA + 0x2000);
                let slots = [
                    0x48u32, 0x4c, 0x50, 0x54, 0x58, 0x5c, 0x60, 0x64, 0x68, 0x74, 0x7c, 0x78,
                    0x80, 0x84, 0x8c, 0x90, 0x94, 0x98, 0x9c, 0xa0,
                ];
                let mask = r.next_u32() ^ seed.wrapping_mul(0x9e37_79b9);
                for (k, off) in slots.into_iter().enumerate() {
                    let present = match seed {
                        0 => true,
                        1 => false,
                        _ => (mask >> k) & 1 != 0,
                    };
                    let w = DATA + 0x3000 + 0x40 * k as u32;
                    c.bus.write32(this + off, if present { w } else { 0 });
                    c.bus.write32(w + 0x14, r.next_u32());
                }
                for k in 0..3u32 {
                    let found = seed == 0 || (seed != 1 && (mask >> (20 + k)) & 1 != 0);
                    let obj = DATA + 0x2800 + 0x40 * k;
                    c.bus.write32(obj + 0x14, r.next_u32());
                    c.bus.write32(
                        OC933_CTRL + 0x10 + 4 * (1 + 2 * k),
                        if found { obj } else { 0 },
                    );
                }
            },
        );
    }

    /// The fakes' gate word and its magic (both machines of a diff get their own).
    pub(crate) const OC934_CTRL: u32 = DATA + 0x2f_1000;

    pub(crate) const OC934_MAGIC: u32 = 0x4f43_3934;

    /// Per-run salt of the fakes' dice and the count of fake calls so far.
    pub(crate) const OC934_SALT: u32 = OC934_CTRL + 4;

    pub(crate) const OC934_CALLS: u32 = OC934_CTRL + 8;

    /// The widget every object lookup returns, and its vtable.
    pub(crate) const OC934_OBJ: u32 = DATA + 0x2f_2000;

    pub(crate) const OC934_VT: u32 = DATA + 0x2f_3000;

    /// A fake callee: the logged fake call, then the result the function under
    /// test branches on. Lookups return null, keys report absent or disabled
    /// about a quarter of the time (a die per call, salted per run).
    pub(crate) fn oc934_fake(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let (a1, a2) = (c.g[A1][0] as u32, c.g[A2][0] as u32);
        let n = c.bus.read32(OC934_CALLS);
        c.bus.write32(OC934_CALLS, n + 1);
        let salt = c.bus.read32(OC934_SALT) as u64;
        let roll = fakes::mix(salt << 20 ^ n as u64) & 3;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        match addr {
            // cUIScreen_getObjectByHashName
            0x0039_d860 => c.g[V0][0] = if roll == 0 { 0 } else { OC934_OBJ as u64 },
            // cKeyboardPopup_GetKeyWidgetName: a name, or no such key.
            0x001c_d0d8 => {
                c.bus.write8(a2, b'k');
                c.bus.write8(a2 + 1, 0);
                c.g[V0][0] = (roll != 0) as u64;
            }
            // cKeyboardPopup_IsKeyEnabled
            0x001c_e560 => c.g[V0][0] = (roll != 0) as u64,
            // cUIScreen_getFrameByLabel: 0xffff means no such frame.
            0x0039_c778 => c.g[V0][0] = if roll == 0 { 0xffff } else { 3 },
            // Font::Measure: ten units per character, into rect.w (+8).
            0x0039_1fb0 => {
                let mut len = 0u32;
                while len < 255 && c.bus.read8(a1 + len) != 0 {
                    len += 1;
                }
                c.bus.write32(a2 + 8, (10.0f32 * len as f32).to_bits());
            }
            _ => {}
        }
    }

    macro_rules! oc934_fakes {
        ($install:ident; $(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC934_CTRL) != OC934_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc934_fake(c, $addr);
                }
            )*
            fn $install() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc934_fakes!(
        oc934_install;
        (oc934_fake_317e98, OC934_PREV_317E98, ee::s10::OVERRIDE_00317E98, ee::s10::f_00317e98__generated, 0x0031_7e98),
        (oc934_fake_3180b8, OC934_PREV_3180B8, ee::s10::OVERRIDE_003180B8, ee::s10::f_003180b8__generated, 0x0031_80b8),
        (oc934_fake_198af0, OC934_PREV_198AF0, ee::s03::OVERRIDE_00198AF0, ee::s03::f_00198af0__generated, 0x0019_8af0),
        (oc934_fake_417828, OC934_PREV_417828, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated, 0x0041_7828),
        (oc934_fake_39d860, OC934_PREV_39D860, ee::s13::OVERRIDE_0039D860, ee::s13::f_0039d860__generated, 0x0039_d860),
        (oc934_fake_39c778, OC934_PREV_39C778, ee::s13::OVERRIDE_0039C778, ee::s13::f_0039c778__generated, 0x0039_c778),
        (oc934_fake_39c870, OC934_PREV_39C870, ee::s13::OVERRIDE_0039C870, ee::s13::f_0039c870__generated, 0x0039_c870),
        (oc934_fake_3a0c50, OC934_PREV_3A0C50, ee::s13::OVERRIDE_003A0C50, ee::s13::f_003a0c50__generated, 0x003a_0c50),
        (oc934_fake_3a0e90, OC934_PREV_3A0E90, ee::s13::OVERRIDE_003A0E90, ee::s13::f_003a0e90__generated, 0x003a_0e90),
        (oc934_fake_3a1000, OC934_PREV_3A1000, ee::s13::OVERRIDE_003A1000, ee::s13::f_003a1000__generated, 0x003a_1000),
        (oc934_fake_3a04f0, OC934_PREV_3A04F0, ee::s13::OVERRIDE_003A04F0, ee::s13::f_003a04f0__generated, 0x003a_04f0),
        (oc934_fake_391fb0, OC934_PREV_391FB0, ee::s12::OVERRIDE_00391FB0, ee::s12::f_00391fb0__generated, 0x0039_1fb0),
        (oc934_fake_1cd0d8, OC934_PREV_1CD0D8, ee::s04::OVERRIDE_001CD0D8, ee::s04::f_001cd0d8__generated, 0x001c_d0d8),
        (oc934_fake_1cd9a0, OC934_PREV_1CD9A0, ee::s04::OVERRIDE_001CD9A0, ee::s04::f_001cd9a0__generated, 0x001c_d9a0),
        (oc934_fake_1ce520, OC934_PREV_1CE520, ee::s04::OVERRIDE_001CE520, ee::s04::f_001ce520__generated, 0x001c_e520),
        (oc934_fake_1ce560, OC934_PREV_1CE560, ee::s04::OVERRIDE_001CE560, ee::s04::f_001ce560__generated, 0x001c_e560),
        (oc934_fake_1ce4c8, OC934_PREV_1CE4C8, ee::s04::OVERRIDE_001CE4C8, ee::s04::f_001ce4c8__generated, 0x001c_e4c8),
        (oc934_fake_1cd020, OC934_PREV_1CD020, ee::s04::OVERRIDE_001CD020, ee::s04::f_001cd020__generated, 0x001c_d020),
        (oc934_fake_1ccf70, OC934_PREV_1CCF70, ee::s04::OVERRIDE_001CCF70, ee::s04::f_001ccf70__generated, 0x001c_cf70),
        (oc934_fake_1ccf00, OC934_PREV_1CCF00, ee::s04::OVERRIDE_001CCF00, ee::s04::f_001ccf00__generated, 0x001c_cf00),
        (oc934_fake_1cb418, OC934_PREV_1CB418, ee::s04::OVERRIDE_001CB418, ee::s04::f_001cb418__generated, 0x001c_b418),
        (oc934_fake_1cce98, OC934_PREV_1CCE98, ee::s04::OVERRIDE_001CCE98, ee::s04::f_001cce98__generated, 0x001c_ce98),
        (oc934_fake_1ccc30, OC934_PREV_1CCC30, ee::s04::OVERRIDE_001CCC30, ee::s04::f_001ccc30__generated, 0x001c_cc30),
        (oc934_fake_1cd708, OC934_PREV_1CD708, ee::s04::OVERRIDE_001CD708, ee::s04::f_001cd708__generated, 0x001c_d708),
    );

    /// A scrambled machine with the game image, the fakes' gate open and the
    /// shared widget (random contents, vtable `OC934_VT` whose slots +0x48,
    /// +0x58, +0xa0 and +0xa8 are fake methods).
    pub(crate) fn oc934_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc934_install();
        let mut r = fakes::machine(c, seed, salt, calls);
        elf_image(c);
        c.bus.write32(OC934_CTRL, OC934_MAGIC);
        c.bus.write32(OC934_SALT, r.next_u32());
        c.bus.write32(OC934_CALLS, 0);
        fill(c, &mut r, OC934_OBJ, 0x40);
        c.bus.write32(OC934_OBJ + 8, OC934_VT);
        for (i, off) in [0x48u32, 0x58, 0xa0, 0xa8].into_iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(OC934_VT + off, delta as u16);
            c.bus.write32(OC934_VT + off + 4, FAKE + 0x10 * i as u32);
        }
        r
    }

    pub(crate) fn oc934_put_str(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    pub(crate) fn oc934_text(r: &mut Rng, len: u32) -> Vec<u8> {
        (0..len).map(|_| b'a' + (r.next_u32() % 26) as u8).collect()
    }

    /// Buy popup: the widget's hash picks one of the eight handlers (or none),
    /// the string types, the credit count and the callee results vary.
    #[test]
    fn c_ui_state_buy_popup_on_widget_created_matches() {
        const NAMES: [u32; 8] = [
            0x0046_62d8,
            0x0046_62e8,
            0x0046_62f8,
            0x0046_05f0,
            0x0046_0610,
            0x0046_6308,
            0x0046_6318,
            0x0046_6328,
        ];
        covered(
            ee::s04::f_001cac98__generated,
            c_ui_state_buy_popup_on_widget_created,
            162,
            &|c, seed| {
                let mut r = oc934_machine(c, seed, 0x001c_ac98, 3);
                let (this, widget) = (DATA + 0x1000, DATA + 0x2000);
                let which = (seed % 9) as usize;
                let hash = if which < 8 {
                    let a = NAMES[which] as usize;
                    str_hash_m2(&c.bus.ram[a..a + 32])
                } else {
                    r.next_u32() | 1
                };
                c.bus.write32(widget + 0x38, hash);
                c.bus.write32(widget + 8, OC934_VT);
                fill(c, &mut r, this + 0x40, 0x20);
                let types = [1u32, 2, 3, 0, 5, u32::MAX];
                let sel = ((seed / 9) % 6) as usize;
                for (k, off) in [0x5cu32, 0x54, 0x64].into_iter().enumerate() {
                    c.bus.write32(this + off, types[(sel + k) % 6]);
                }
                c.bus
                    .write32(this + 0x50, [0xffff_fffd, 0, 2][((seed / 54) % 3) as usize]);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = widget as u64;
            },
        );
    }

    /// Scroll window: window text, margins and direction vary; the measured
    /// width is ten units per character (a fake), so the fit loops end.
    #[test]
    fn c_keyboard_popup_update_scroll_window_matches() {
        covered(
            ee::s04::f_001cc8d8__generated,
            c_keyboard_popup_update_scroll_window,
            160,
            &|c, seed| {
                let mut r = oc934_machine(c, seed, 0x001c_c8d8, 24);
                let this = DATA + 0x1000;
                let len = [0u32, 3, 8, 14, 20][((seed / 2) % 5) as usize];
                let text = oc934_text(&mut r, len);
                oc934_put_str(c, this + 0x74, &text);
                for i in 0..0x40 {
                    c.bus.write8(this + 0xb4 + i, 0);
                }
                let start = if len == 0 {
                    0
                } else {
                    r.next_u32() % (len / 2 + 1)
                };
                let end = start + r.next_u32() % (len - start + 1);
                c.bus.write32(this + 0x138, start);
                c.bus.write32(this + 0x13c, end);
                let cursor = if (seed / 10) % 2 == 0 {
                    len
                } else {
                    r.next_u32() % (len + 1)
                };
                c.bus.write32(this + 0x134, cursor);
                c.bus.write32(this + 0x60, len + (seed / 3) % 3);
                c.bus.write32(this + 0x40, DATA + 0x5000);
                let width = [40.0f32, 90.0, 160.0, 400.0][((seed / 5) % 4) as usize];
                let margin = [0.0f32, 6.0, 12.0][((seed / 7) % 3) as usize];
                c.bus.write32(this + 0x3f4, width.to_bits());
                c.bus.write32(this + 0x400, margin.to_bits());
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    /// Cursor: text object found or not, character under the cursor (none,
    /// letter, space), masked or plain text.
    #[test]
    fn c_keyboard_popup_update_cursor_matches() {
        covered(
            ee::s04::f_001ccc30__generated,
            c_keyboard_popup_update_cursor,
            120,
            &|c, seed| {
                let mut r = oc934_machine(c, seed, 0x001c_cc30, 14);
                let this = DATA + 0x1000;
                fill(c, &mut r, this + 0x3e8, 0x20);
                let cursor = r.next_u32() % 18;
                let start = r.next_u32() % (cursor + 1);
                let mut text = oc934_text(&mut r, 20);
                text[cursor as usize] = [0u8, b'a', b' ', b'@', b'Z'][(seed % 5) as usize];
                // The text ends at the cursor character when it is NUL.
                oc934_put_str(c, this + 0x74, &text);
                c.bus.write8(this + 0x74 + cursor, text[cursor as usize]);
                let plain = oc934_text(&mut r, 20);
                oc934_put_str(c, this + 0xb4, &plain);
                let masked = oc934_text(&mut r, 20);
                oc934_put_str(c, this + 0xf4, &masked);
                c.bus.write32(this + 0x70, (seed / 5) % 2);
                c.bus.write32(this + 0x134, cursor);
                c.bus.write32(this + 0x138, start);
                c.bus.write32(this + 0x40, DATA + 0x5000);
                c.g[A0][0] = this as u64;
            },
        );
    }

    /// Transition in: first or not, caps state, keyboard type, which key-set
    /// objects exist, and which initial text (text, text id, wide string,
    /// string object or the empty default) is given.
    #[test]
    fn c_keyboard_popup_transition_in_matches() {
        covered(
            ee::s04::f_001cd348__generated,
            c_keyboard_popup_transition_in,
            192,
            &|c, seed| {
                let mut r = oc934_machine(c, seed, 0x001c_d348, 16);
                let this = DATA + 0x1000;
                fill(c, &mut r, this, 0x110);
                let ty = [1u32, 0, 2, 3, u32::MAX, 9][(seed % 6) as usize];
                let bits = seed / 6;
                c.bus.write32(this + 0x43c, ty);
                c.bus.write32(this + 0x40, DATA + 0x5000);
                for (k, off) in [0x48u32, 0x4c, 0x50].into_iter().enumerate() {
                    let have = (r.next_u32() >> k) & 1;
                    c.bus
                        .write32(this + off, if have != 0 { OC934_OBJ } else { 0 });
                }
                c.bus
                    .write32(GAME_GP_300, if bits & 1 != 0 { DATA + 0x7000 } else { 0 });
                c.bus.write32(DATA + 0x7000 + 0x88, r.next_u32());
                c.bus.write32(this + 0x418, (bits >> 1) & 1);
                c.bus.write32(
                    this + 0x42c,
                    if bits & 4 != 0 { r.next_u32() | 1 } else { 0 },
                );
                c.bus
                    .write32(this + 0x428, if bits & 8 != 0 { DATA + 0x6100 } else { 0 });
                c.bus.write32(this + 0x424, DATA + 0x6008);
                c.bus.write32(DATA + 0x6000, (bits >> 4) & 1);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 24 != 7) as u64;
            },
        );
    }

    /// `gp + 0x300`: the pointer to the game-wide control block whose flag
    /// word (+0x88) gives the caps state.
    pub(crate) const GAME_GP_300: u32 = crate::difftest::GAME_GP + 0x300;

    /// Refresh keys: 84 keys, each present or absent, plain or special, enabled
    /// or not, with and without its text and frame objects.
    #[test]
    fn c_keyboard_popup_refresh_keys_matches() {
        covered(
            ee::s04::f_001cd708__generated,
            c_keyboard_popup_refresh_keys,
            24,
            &|c, seed| {
                let mut r = oc934_machine(c, seed, 0x001c_d708, 300);
                let this = DATA + 0x1000;
                c.bus
                    .write32(this + 0x40, if seed % 12 == 0 { 0 } else { DATA + 0x5000 });
                c.bus.write32(this + 0x68, r.next_u32() & 1);
                c.bus.write32(this + 0x6c, r.next_u32() & 1);
                c.bus.write32(this + 0x140, r.next_u32() % 84);
                c.bus.write32(
                    this + 0x43c,
                    if seed % 3 == 0 { 1 } else { r.next_u32() % 3 },
                );
                c.g[A0][0] = this as u64;
            },
        );
    }

    /// The popup object (0x360 bytes) in the fixture area.
    pub(crate) const POPUP_937: u32 = DATA + 0x20_0000;

    /// Gate word of the LayoutTitle fakes, in the machine's own RAM.
    pub(crate) const LAYOUT_TITLE_GATE_937: u32 = DATA + 0x2e_0000;

    pub(crate) const LAYOUT_TITLE_MAGIC_937: u32 = 0x9370_0c0d;

    pub(crate) static PREV_SET_TEXT_937: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static PREV_ACCUMULATE_937: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static PREV_SET_WIDGET_ALIGN_937: AtomicUsize = AtomicUsize::new(0);

    /// SetTextFromDesc (0x1c8050) with the gate on: returns at once. With the
    /// gate off it runs the body found in the slot when it was hooked.
    pub(crate) fn layout_title_set_text_fake_937(c: &mut Cpu) {
        if c.bus.read32(LAYOUT_TITLE_GATE_937) != LAYOUT_TITLE_MAGIC_937 {
            return fall_back(c, &PREV_SET_TEXT_937, ee::s03::f_001c8050__generated);
        }
        abi::ret(c);
    }

    /// AccumulateSize (0x1c9b68) with the gate on: returns at once.
    pub(crate) fn layout_title_accumulate_fake_937(c: &mut Cpu) {
        if c.bus.read32(LAYOUT_TITLE_GATE_937) != LAYOUT_TITLE_MAGIC_937 {
            return fall_back(c, &PREV_ACCUMULATE_937, ee::s03::f_001c9b68__generated);
        }
        abi::ret(c);
    }

    /// SetWidgetAlign (0x1c8930) with the gate on: returns at once.
    pub(crate) fn layout_title_set_widget_align_fake_937(c: &mut Cpu) {
        if c.bus.read32(LAYOUT_TITLE_GATE_937) != LAYOUT_TITLE_MAGIC_937 {
            return fall_back(
                c,
                &PREV_SET_WIDGET_ALIGN_937,
                ee::s03::f_001c8930__generated,
            );
        }
        abi::ret(c);
    }

    /// Puts the three gated fakes in their override slots (once per process).
    pub(crate) fn layout_title_install_937() {
        hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C8050),
            layout_title_set_text_fake_937,
            &PREV_SET_TEXT_937,
        );
        hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C9B68),
            layout_title_accumulate_fake_937,
            &PREV_ACCUMULATE_937,
        );
        hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C8930),
            layout_title_set_widget_align_fake_937,
            &PREV_SET_WIDGET_ALIGN_937,
        );
    }

    #[test]
    fn c_fe_popup_confirm_base_ctor_matches() {
        check(
            ee::s03::f_001c58e8__generated,
            c_fe_popup_confirm_base_ctor,
            6,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1c58e8, 0);
                elf_image(c);
                fill(c, &mut r, POPUP_937, 0xd8);
                c.bus.write32(GAME_GP.wrapping_add(0xda0), r.next_u32());
                c.g[A0][0] = POPUP_937 as u64;
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_v05_matches() {
        check(
            ee::s03::f_001c5b68__generated,
            c_fe_popup_confirm_v05,
            8,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1c5b68, 0);
                fill(c, &mut r, POPUP_937, 0xd8);
                c.g[A0][0] = POPUP_937 as u64;
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_set_content_visible_matches() {
        check(
            ee::s03::f_001c5e58__generated,
            c_fe_popup_confirm_set_content_visible,
            16,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1c5e58, 5);
                let vt = fakes::vtable(c, &mut r, &[0x48]);
                for (k, field) in [0x70u32, 0x74, 0x78, 0x7c, 0x88].iter().enumerate() {
                    let child = POPUP_937 + 0x1000 + 0x40 * k as u32;
                    c.bus.write32(child + 8, vt);
                    let present = (seed >> k) & 1 == 0;
                    c.bus
                        .write32(POPUP_937 + field, if present { child } else { 0 });
                }
                c.g[A0][0] = POPUP_937 as u64;
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_compute_background_scale_matches() {
        check(
            ee::s03::f_001c7620__generated,
            c_fe_popup_confirm_compute_background_scale,
            24,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1c7620, 0);
                fill(c, &mut r, POPUP_937, 0xd8);
                let ty = match seed % 5 {
                    4 => u32::MAX,
                    k => k,
                };
                c.bus.write32(POPUP_937 + 0x31c, ty);
                c.bus.write32(POPUP_937 + 0x218, r.f32(4.0));
                c.bus.write32(POPUP_937 + 0x21c, r.f32(4.0));
                c.bus.write32(POPUP_937 + 0x1f4, r.f32(4.0));
                c.bus.write32(POPUP_937 + 0x1f8, r.f32(4.0));
                let pos = POPUP_937 + 0x2000;
                c.bus.write32(pos, r.f32(100.0));
                c.bus.write32(pos + 4, r.f32(100.0));
                c.bus.write32(pos + 8, r.next_u32());
                c.g[A0][0] = POPUP_937 as u64;
                c.g[A1][0] = pos as u64;
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_layout_text_entry_box_matches() {
        check(
            ee::s03::f_001c7eb0__generated,
            c_fe_popup_confirm_layout_text_entry_box,
            16,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1c7eb0, 0);
                let vt = fakes::vtable(c, &mut r, &[0xa0, 0xa8]);
                fill(c, &mut r, POPUP_937, 0xd8);
                let o80 = POPUP_937 + 0x1000;
                let o6c = POPUP_937 + 0x1100;
                let o84 = POPUP_937 + 0x1200;
                let o8c = POPUP_937 + 0x1300;
                for o in [o80, o6c, o84, o8c] {
                    fill(c, &mut r, o, 0x40);
                }
                c.bus.write32(o80 + 8, vt);
                c.bus.write32(POPUP_937 + 0x80, o80);
                c.bus.write32(POPUP_937 + 0x6c, o6c);
                c.bus.write32(POPUP_937 + 0x84, o84);
                c.bus
                    .write32(POPUP_937 + 0x8c, if seed % 2 == 0 { 0 } else { o8c });
                c.bus.write32(o8c + 0x4, 0);
                c.g[A0][0] = POPUP_937 as u64;
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_desc_to_string_matches() {
        check(
            ee::s03::f_001c80e8__generated,
            c_fe_popup_confirm_desc_to_string,
            20,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1c80e8, 2);
                elf_image(c);
                let desc = POPUP_937 + 0x2000;
                let text = POPUP_937 + 0x2100;
                let dst = POPUP_937 + 0x2200;
                let ty = match seed % 5 {
                    4 => 9,
                    k => k,
                };
                c.bus.write32(desc, ty);
                c.bus.write32(desc + 4, text);
                c.bus.write16(text, 0x41);
                c.bus.write16(text + 2, 0x42);
                c.bus.write16(text + 4, 0);
                c.bus.write8(text + 8, 0);
                let vt = fakes::vtable(c, &mut r, &[0x20]);
                let world = POPUP_937 + 0x3000;
                let obj = POPUP_937 + 0x3100;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x8c, obj);
                c.bus.write32(obj + 4, vt);
                c.g[A0][0] = POPUP_937 as u64;
                c.g[A1][0] = dst as u64;
                c.g[A2][0] = desc as u64;
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_measure_desc_matches() {
        check(
            ee::s03::f_001c8320__generated,
            c_fe_popup_confirm_measure_desc,
            16,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1c8320, 2);
                elf_image(c);
                let text = POPUP_937 + 0x3000;
                fill(c, &mut r, text, 0x40);
                let desc = POPUP_937 + 0x2000;
                let ty = match seed % 5 {
                    4 => 9,
                    k => k,
                };
                c.bus.write32(desc, ty);
                c.bus.write32(desc + 4, POPUP_937 + 0x2100);
                let vt = fakes::vtable(c, &mut r, &[0x20]);
                let world = POPUP_937 + 0x4000;
                let obj = POPUP_937 + 0x4100;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x8c, obj);
                c.bus.write32(obj + 4, vt);
                c.g[A0][0] = POPUP_937 as u64;
                c.g[A1][0] = text as u64;
                c.g[A2][0] = desc as u64;
                c.g[A3][0] = (POPUP_937 + 0x2800) as u64;
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_layout_title_matches() {
        check(
            ee::s03::f_001c8678__generated,
            c_fe_popup_confirm_layout_title,
            16,
            &|c, seed| {
                layout_title_install_937();
                let mut r = fakes::machine_rt(c, seed, 0x1c8678, 3);
                c.bus.write32(LAYOUT_TITLE_GATE_937, LAYOUT_TITLE_MAGIC_937);
                elf_image(c);
                fill(c, &mut r, POPUP_937, 0xd8);
                let text = POPUP_937 + 0x3000;
                fill(c, &mut r, text, 0x40);
                let vt = fakes::vtable(c, &mut r, &[0x48]);
                c.bus.write32(text + 8, vt);
                c.bus
                    .write32(POPUP_937 + 0x7c, if seed % 7 == 0 { 0 } else { text });
                c.bus
                    .write32(POPUP_937 + 0x2cc, if seed % 3 == 0 { 0 } else { 1 });
                let info = POPUP_937 + 0x2000;
                c.bus
                    .write32(info + 0xc, [0u32, 1, 2, 3][(seed % 4) as usize]);
                c.bus.write32(info + 0x10, POPUP_937 + 0x2100);
                c.g[A0][0] = POPUP_937 as u64;
                c.g[A1][0] = info as u64;
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_set_widget_align_matches() {
        check(
            ee::s03::f_001c8930__generated,
            c_fe_popup_confirm_set_widget_align,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x001c_8930, 0);
                let obj = DATA + 0x20_0000;
                // Every state 0..23 (so 9/10, 17/18 and the no-move cases),
                // the alignment equal to it for half the seeds, and t zero
                // for a quarter of them.
                let state = seed % 24;
                let old = if seed % 2 == 0 {
                    state
                } else {
                    r.next_u32() % 64
                };
                let vt = fakes::vtable(c, &mut r, &[0xa0]);
                let word = (r.next_u32() & !(0x3f << 13)) | (old << 13);
                c.bus.write32(obj + 0x14, word);
                c.bus.write32(obj + 8, vt);
                for k in 0..3 {
                    c.bus.write32(obj + 0x44 + 4 * k, r.edge_f32());
                }
                c.g[A1][0] = obj as u64;
                c.g[A2][0] = (state as u64) | ((r.next_u32() as u64) << 8);
                c.f[12] = if seed % 4 == 0 { 0 } else { r.edge_f32() };
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_open_confirm_popup_matches() {
        check(
            ee::s03::f_001c9b98__generated,
            c_fe_popup_confirm_open_confirm_popup,
            60,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x001c_9b98, 0);
                let this = DATA + 0x20_0000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                // Widget fields the code reads, pointing into the test data.
                fill(c, &mut r, this, 0x340 / 4);
                c.bus.write32(this + 0x40, this + 0x400);
                c.bus.write32(this + 0x400 + 0xd0, this + 0x800);
                c.bus.write32(this + 0x800 + 0x10, this + 0xa00);
                c.bus.write32(this + 0x314, r.next_u32() % 16);
                c.bus.write32(this + 0x10, 0);
            },
        );
    }

    /// The `vt[off]` method of a fixture vtable: `{ delta, fn }`.
    pub(crate) fn put_method(c: &mut Cpu, vt: u32, off: u32, delta: i16, func: u32) {
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, func);
    }

    #[test]
    fn fepopup_confirm_set_option_enabled_matches() {
        check(
            ee::s03::f_001ca0f0__generated,
            fepopup_confirm_set_option_enabled,
            200,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x1ca0, 2);
                let idx = seed % 4;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = r.next_u32() as u64;
                let obj = if seed % 5 == 0 { 0 } else { DATA + 0x1000 };
                c.bus.write32(DATA + 0x90 + 4 * idx, obj);
                let vt = DATA + 0x2000;
                c.bus.write32(DATA + 0x1008, vt);
                put_method(c, vt, 0x40, (r.next_u32() as i16) & !3, FAKE);
                put_method(c, vt, 0xc8, (r.next_u32() as i16) & !3, FAKE + 0x10);
            },
        );
    }

    #[test]
    fn fepopup_confirm_render_matches() {
        check(
            ee::s03::f_001ca578__generated,
            fepopup_confirm_render,
            100,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x1ca5, 0);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn keyboard_popup_ctor_matches() {
        check(
            ee::s04::f_001cb030__generated,
            keyboard_popup_ctor,
            200,
            &|c, seed| {
                let mut r: Rng = machine_rt(c, seed, 0x1cb0, 0);
                let this = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x3000);
                c.g[A2][0] = sx(DATA + 0x3800);
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
                let gp = crate::difftest::GAME_GP;
                c.bus
                    .write32(gp + 0x300, if seed % 3 == 0 { 0 } else { DATA + 0x5000 });
                c.bus.write32(DATA + 0x5000 + 0x88, r.next_u32());
                c.bus.write32(gp + 0xda0, r.next_u32());
            },
        );
    }

    #[test]
    fn keyboard_popup_on_widget_created_matches() {
        check(
            ee::s04::f_001cb2c8__generated,
            keyboard_popup_on_widget_created,
            200,
            &|c, seed| {
                let mut r: Rng = machine_rt(c, seed, 0x1cb2, 0);
                c.g[A0][0] = sx(DATA + 0x2000);
                c.g[A1][0] = sx(DATA + 0x3000);
                let widget = DATA + 0x3000;
                c.bus.write32(widget + 0x38, r.next_u32());
                c.bus.write32(widget + 0x8, DATA + 0x4000);
                put_method(c, DATA + 0x4000, 0x48, 0, FAKE + 0x20);
                c.bus
                    .write32(crate::difftest::GAME_GP - 0x848, DATA + 0x6000);
                c.bus
                    .write32(DATA + 0x6000 + 0x84, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(crate::difftest::GAME_GP - 0x1680, seed % 2);
            },
        );
    }

    #[test]
    fn keyboard_popup_move_up_from_top_row_matches() {
        check(
            ee::s04::f_001cbb70__generated,
            keyboard_popup_move_up_from_top_row,
            200,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x1cbb, 0);
                let this = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(this + 0x140, seed % 16);
            },
        );
    }

    /// The world pointer the reward gallery code reads at `gp - 0x848`.
    pub(crate) const WORLD: u32 = DATA + 0x2000;

    #[test]
    fn keyboard_popup_build_keys_matches() {
        check(
            ee::s04::f_001ce2a8__generated,
            keyboard_popup_build_keys,
            60,
            &|c, seed| {
                elf_image(c);
                let _ = machine_rt(c, seed, 0x1c_e2a8, 0);
                let this = DATA + 0x100;
                // The name text (+0x74) is empty for some seeds, set for others.
                if seed % 3 != 0 {
                    c.bus.write8(this + 0x74, b'x');
                    c.bus.write8(this + 0x75, 0);
                }
                c.bus.write32(this + 0x58, seed % 2);
                c.bus.write32(this + 0x60, 0x40);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn keyboard_popup_append_quick_message_matches() {
        check(
            ee::s04::f_001ce5e0__generated,
            keyboard_popup_append_quick_message,
            40,
            &|c, seed| {
                elf_image(c);
                let _ = machine_rt(c, seed, 0x1c_e5e0, 2);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
                let this = DATA + 0x100;
                c.bus.write8(this + 0x74, b'o');
                c.bus.write8(this + 0x75, 0);
                c.bus.write32(this + 0x60, 0x20 + seed % 8);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 0x54) as u64;
            },
        );
    }

    #[test]
    fn keyboard_popup_move_up_from_digit_row_matches() {
        check(
            ee::s04::f_001cbdb0__generated,
            keyboard_popup_move_up_from_digit_row,
            120,
            &|c, seed| {
                elf_image(c);
                let _ = machine_rt(c, seed, 0x1c_bdb0, 2);
                let this = DATA + 0x100;
                c.bus.write32(this + 0x140, 0x0c + seed % 16);
                c.bus.write32(this + 0x144, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }
}
