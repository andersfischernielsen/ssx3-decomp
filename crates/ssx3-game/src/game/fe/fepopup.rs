//! `src/fe/fepopup.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::animation::rideranimbase::sw as sw_3;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at as lw_at_3;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::dirtysock::tags2::lw as lw_2;
pub(crate) use crate::game::dirtysock::tags2::p;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_4;
pub(crate) use crate::game::fe::feasyncfile::ldf;
pub(crate) use crate::game::fe::feasyncfile::lh;
pub(crate) use crate::game::fe::feasyncfile::lw as lw_4;
pub(crate) use crate::game::fe::feasyncfile::lw_3;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw_7;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_8;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_4;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_5;
pub(crate) use crate::game::fe::feasyncfile::ui_init_screen;
pub(crate) use crate::game::fe::feasyncfile::widget_action_v24;
pub(crate) use crate::game::fe::fepopup3::p_2 as p_3;
pub(crate) use crate::game::fe::fepopup3::sw;
pub(crate) use crate::game::fe::festaterewards::at_4 as at_3;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_5;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at_2;
pub(crate) use crate::game::main::gamestate::vslot;
pub(crate) use crate::game::movie::movieplayer::at;
pub(crate) use crate::game::movie::movieplayer::lw_at;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::ui::uivector::r;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw_6;
pub(crate) use crate::game::wscript::wscriptman::sw as sw_2;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
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
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_CTOR_COPY;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_DTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_INIT_FROM_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAM_CONTROLLER_RESTORE_DEFAULT;
    pub(crate) use crate::game::bx::bxstring2::callee::CHUNK_RESOLVE_RELOCATE_DMA;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN3D_GROUP_BUILD_MATRICES;
    pub(crate) use crate::game::bx::bxstring2::callee::CUTSCENE_TRANSITION_MAN_FORCE_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::bx::bxstring2::callee::FLAG_MANAGER_GROUP_BUILD_MESH;
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_SKIP_WHITESPACE;
    pub(crate) use crate::game::bx::bxstring2::callee::MEMSET as MEMSET_2;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_MAN_CLEAR;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_SET_EVENT_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_MANAGER;
    pub(crate) use crate::game::bx::bxstring2::callee::OPTIONS_SET_VIBRATION;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_SCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_READ_FLOAT_3 as PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WIDGET_ALIGN;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WRAPPED_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::QSORT as QSORT_2;
    pub(crate) use crate::game::bx::bxstring2::callee::READ_PRIVATE_MESSAGE_SET_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::RIDER_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_SET_SKIP_DELAY;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_DEQUEUE_SECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_POP_FREE_NODE;
    pub(crate) use crate::game::bx::bxstring2::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_SPEECH_CONTEXT_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::STRLEN as STRLEN_3;
    pub(crate) use crate::game::bx::bxstring2::callee::STR_HASH as STR_HASH_3;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_DESIRED_EYE;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_LOOK_AT;
    pub(crate) use crate::game::bx::bxstring2::callee::UITEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_ADD_ITEM_STR_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_FIND_ITEM_BY_VALUE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_FIND_FIRST_SELECTABLE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_UPDATE as UI_STATE_UPDATE_3;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_CPY as USTR_CPY_2;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_FROM_ASCII_2 as USTR_FROM_ASCII;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_LEN as USTR_LEN_2;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_SPRINTF;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_MISSION_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT_2 as WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INSTANCE as BE_LIBRARY_GET_INSTANCE_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INTERFACE as BE_LIBRARY_GET_INTERFACE_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUY_POPUP_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::CLEAR_NEW_REWARDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::DYNAMIC_EMITTER_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::FE_ASYNC_SET_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2 as GAME_MODULE_REQUEST_STATE_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_PROCESS_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GROW_LINES;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INIT_FROM_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOAD_INDICATOR_RENDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_API_SEND;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_GET_ROOM_NAME;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_END_OPERATION;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_ON_OPERATION_DONE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MEM_REPORT_STATS_LOCKED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MSG_PRINTF;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::POPUP_CONFIRM_REBUILD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::QSORT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RESET_FOR_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_ADD_BOOST_2 as RIDER_ADD_BOOST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_APPLY_SCRIPT_TRACK;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GATHER_CAM_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_RECORD_BEST_JUMP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_UPDATE_SCRIPT_SOUNDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SECTION_SELECT_PEAK_LIGHTING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SERIALIZE_READ_U32;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SET_ITEM_YEAR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SNOW_BOX_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_GET_INSTANCE as SSX_AUDIO_GET_INSTANCE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STRLEN as STRLEN_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_HASH as STR_HASH_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_COPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TEX_MAN_INIT_TABLE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_CTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_ASCII;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_EVENT_STAT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING as UI_TEXT_SET_STRING_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::VRAM_HEAP_INIT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::ANIM_GET_LAYER_SEQUENCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_APPLY_OUTPUT_MODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_SET_OUTPUT;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_BAKE_GROUP;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_EXPORT_HISTORY_FILES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_FREE_HISTORIES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_LOAD_HISTORY_FILE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_ADD_TIMER_2 as BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_BAGT_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_CHAR_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_BE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_INTERFACE_CALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE_2 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_NEW_PLAYER_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_RESET_PROFILE_CHARACTERS_2 as BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_REWARD_INT_IS_UNLOCKED;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_GET_NEXT_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_DESTROY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_SET_PRESENCE_ADDR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE as BUILTIN_DELETE_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE_2 as BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAM_CONTROLLER_SET_ALGORITHM;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBE_GET_BE as CBE_GET_BE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBE_GET_INTERFACE as CBE_GET_INTERFACE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBXSTRING_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_BUY_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_EQUIP_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_MAN2D_INIT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::CSETUP_MAN_SEND_PACKET;
    pub(crate) use crate::game::dirtysock::tags2::callee::CVOIP_MAN_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CW_SCRIPT_CACHE_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_BX_STRING_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_LUNO_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_GET_SAVE_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_LIST_DIR_ALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_READ_SAVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::EATIMER_REMOVE_CALLBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEMSET;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_3 as EA_MEM_FREE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::EFFECT_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::EFFECT_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_ON_ACTIVATE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_STATS_REFRESH_RESULTS_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_UPDATE_BUTTONS_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::FLOAT_TO_UINT;
    pub(crate) use crate::game::dirtysock::tags2::callee::FORMAT_TAGGED;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_00152948;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_00152BA8;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_002C2540;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_002C26D0;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_0039E2A0;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_CAMERA_SET_CONTROLLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_COMM_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_PRE_RACE_MENU_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_ALT_CHAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_CHAR_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_FINISH_SPEECH_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_LANGUAGE_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_RIDER_VOICE_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_TRANSITION_MAN;
    pub(crate) use crate::game::dirtysock::tags2::callee::GM_PICK_OPPONENTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_EVENT_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_RAMP_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_SPAN_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_LOCAL_PLAYER;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_PA_SPEECH_ON;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_RACE_TYPE_4OR5;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_REPLAY_PLAYING;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_IS_KEY_ENABLED as KEYBOARD_POPUP_IS_KEY_ENABLED_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_SHOW;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBYMAN_SEND_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT_3 as LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_FORWARD_EVENT_TO_UI;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MENU_ON_LOBBY_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LUNO_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_GET_PRODUCT_CODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_GET_SAVE_TYPE_TAG;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_END_OPERATION_2 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_SET_MODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEASURE_PLAIN;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEASURE_WRAP;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMORY_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMSET_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_ASCII_CHAR_TO_SJIS;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_SET_USE_SAVE_TITLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT0;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT1;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT3;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT4;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT5;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT6;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_IS_FLAG_SET_2 as MESSAGE_CENTER_IS_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_MESSAGE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_SET_FLAG_2 as MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_1;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_4;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_5;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_3 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE_2 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FIND;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FLUSH;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_GAME_UTIL_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_MESSENGER_SET_PLAYER_MENU_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_REMOVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::ON_QUIT_POPUP_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::PATH_PLAT_REFRESH_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_DECODE_FRAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_FRAME_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_ADD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RECORD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SAVE_EMIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_REFRESH_LIST;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_NOW_PLAYING;
    pub(crate) use crate::game::dirtysock::tags2::callee::RIDER_123168;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCE_MC_FORMAT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_DATA_GET_INFO_2 as SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2 as SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_OBJ_TABLE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_2 as SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MOVIE_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_OBJ_CAM_SET_OWNER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_PUSH_FREE_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUPMAN_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::dirtysock::tags2::callee::SLOGIN_MC_GET_PORT_SLOT_2 as SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY_2 as SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLINK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_2 as SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_REMOVE_TICK_CALLBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNOW_BOX_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNPRINTF;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_SET_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPEECH_QUEUE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPEECH_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_6 as SPRINTF_4;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_ARCADE_SAY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_HANDLE_MUSIC_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_PLAY_CHARACTER_TUNE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::STATS_SELECT_CATEGORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY as STRCPY_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY as STRCPY_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::STREAM_BUFFER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::STR_HASH_4;
    pub(crate) use crate::game::dirtysock::tags2::callee::STR_INTERN;
    pub(crate) use crate::game::dirtysock::tags2::callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::dirtysock::tags2::callee::TAG_FIELD_GET_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::TEXT_OBJ;
    pub(crate) use crate::game::dirtysock::tags2::callee::TRANSITION_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::TRANSITION_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::TROPHY_ROOM_RELEASE_IMAGES;
    pub(crate) use crate::game::dirtysock::tags2::callee::UIMENU_SET_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SOUND_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR as UI_STATE_DTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR_2 as UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_ON_ACTIVATE as UI_STATE_ON_ACTIVATE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_GET_FONT;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_0;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_1;
    pub(crate) use crate::game::dirtysock::tags2::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_DEACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_INSTANCE_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_TIMERS_BY_CALLBACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_MISSION_INT_GET_SESSION_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_HUMAN_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_SPECIAL_EVENT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_TRACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_GET_NEXT_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_GET_PREV_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_NODE_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILD_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_ASSIGN_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_CTOR_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_RECV;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_SEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_CUT_SCENE_TRANSITION_MAN_SAVE_VIEWPORT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_BUY_SELECTED_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REFRESH_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REMOVE_LIST_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_UPDATE_MONEY_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_LOAD_CONFIG_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SET_LIST_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SHOW_CARD_ERROR_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_MESSENGER_CLEAR_NAME_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_HINT_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_SSX_WORLD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_OPTIONS_DEVICE_SELECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_IMAGE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_USE_EXISTING_ACCOUNT_CHECK_EMAIL;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_ALLOC_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_FREE_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START_CAPTURED;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_GET_RIDER_PREVIEW_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ASYNC_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_PLAYER_PAD_MASK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_ADVANCE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_QUIT_TO_LOAD_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUTSYNC_IS_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_BINDING_EVAL_FLOAT;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA_2 as IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_START_SEQUENCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_HEAP_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_SPAN_TRACK_ACTIVATE_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_SPAN_TRACK_NEXT_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_FIND_ENABLED_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_FIND_ENABLED_IN_COLUMN_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_INDICATOR_ADVANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_CONNECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_TABLE_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_CALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_PASS_THROUGH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_AS_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR_2 as LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_FORMAT_SLOT_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC_NAMED_128;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_DISABLED_3 as MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_EXTENSION_AND_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_ACCOUNT_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_5 as NET_FE_STATE_DTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP as NET_FE_STATE_GET_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_13 as NET_FE_STATE_ON_CHILD_EVENT_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_5 as NET_FE_STATE_SHOW_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED_4 as OPERATOR_NEW_NAMED_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_DATA_FIND_MARKER;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_FIND_BANK_SLOT as PATH_PLAT_FIND_BANK_SLOT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_GRID_MESH_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::REFRESH_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_BASE_RESET_FX;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_START_POSE_ANGLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_IS_LOADED;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_SHOW_HUD_MSG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_PLACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SCRIPT_OBJ_MATCHES_RIDER_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_3 as SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_TRANSITION_TARGET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_TICK_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_RIDER_GET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SET_CATEGORY_TITLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SHOW_BUY_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SHOW_EQUIP_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPCH_MAKE_ID_2 as SPCH_MAKE_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_IS_INSTANCE_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT as SPEECH_SYSTEM_SET_EVENT_CONTEXT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_GET_PATH_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_CHARACTER_TO_SPEECH_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_FADE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_IS_SPEECH_VOLUME_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_FADE_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::START_SCREEN_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_MENU_SET_SELECTION_3 as UI_MENU_SET_SELECTION_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_LOAD as UI_OBJECT_LOAD_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_5 as UI_SCREEN_FIND_OBJECT_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_CTOR_2_4 as UI_STATE_CTOR_2_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_TOP_STATES as UI_STATE_MGR_CLOSE_TOP_STATES_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_SWITCH_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_ID_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_OWNED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_TEXT_ID_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF_ASCII_FMT;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_TO_ASCII;
    pub(crate) use crate::game::fe::festaterewards2::callee::ANIM_OBJECT_V36;
    pub(crate) use crate::game::fe::festaterewards2::callee::APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::festaterewards2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::fe::festaterewards2::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::fe::festaterewards2::callee::DL_HEAP_USABLE_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::DYNAMIC_EMITTER_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_LOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_UNLOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEMSET;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_GET_POOL_BLOCK_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_MAN_FREE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_REPORT_POOL_STATS;
    pub(crate) use crate::game::fe::festaterewards2::callee::MOUNTAIN_ROOM_UPDATE_EVENT_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_DISCONNECTED_POPUP_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_QUICK_PLAY_POPUP_ON_OPTIONS_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::POPUP_CONFIRM_GET_TEXT_BOX_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::PREVIEW_REWARD_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::festaterewards2::callee::PS2_PAD_CREATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_APPLY_CAMERA;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_NEXT_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_PREV_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SET_STATE_2;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SNAP_TO_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_STREAM_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_COUNT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_NAMES;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_ARRANGEMENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_READ_MODE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_UNLOAD_WORLD_SCRIPTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_ADD_MOVIE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_POLL_LOADING;
    pub(crate) use crate::game::fe::festaterewards2::callee::SECTION_MAN_FIND_LOGICAL_SECTION;
    pub(crate) use crate::game::fe::festaterewards2::callee::SONG_INSTANCE_GET_DUCK_TO_LOOPS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SPRINTF;
    pub(crate) use crate::game::fe::festaterewards2::callee::S_LOGIN_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::TROPHY_ROOM_LOAD_TROPHY_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards2::callee::TWSCRIPT_CACHE_RELOCATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::USB_KB_DEFAULT_CONNECT_CB;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_STATE_STREAM_RESET;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_TRIGGER_MANAGER_GET_TRIGGER_INFO;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_CREATE_COMPUTER_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_DESTROY_MISSION_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_REWARD_INT_COUNT_SET_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_ACCOUNT_NAME;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_PASSWORD;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_SHUTDOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::ovstatepause::callee::EVENT_SELECT_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_ASYNC_MANAGER_CANCEL;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_FORMAT_MONEY;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_MOVE_DOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_BEGIN_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::fe::ovstatepause::callee::MEM_CARD_STATE_BASE_CTOR_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::MOUNTAIN_ROOM_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_SHOW_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_ON_CHILD_EVENT as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::OV_SCREEN_SESSION_DESTROY_INDICATORS;
    pub(crate) use crate::game::fe::ovstatepause::callee::PLAY_FE_SOUND;
    pub(crate) use crate::game::fe::ovstatepause::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::fe::ovstatepause::callee::SECTION_MAN_REQUEST_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRCPY;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRLEN;
    pub(crate) use crate::game::fe::ovstatepause::callee::STR_HASH;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_CTOR as UI_STATE_CTOR_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATES as UI_STATE_MGR_CLOSE_TOP_STATES_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATE_AND_POPUPS;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_ON_ACTIVATE_2 as UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_RENDER;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_UPDATE_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXT_SET_STRING as UI_TEXT_SET_STRING_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::VID_ENGINE_PLAYER_RENDER;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_1;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BEREWARD_INT_IS_PEAK_COMPLETE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_SCORE_INT_UPDATE_BEST_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_IS_CONNECTED;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_RECONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_CTOR_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_DTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_BE;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_INTERFACE;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_BUY_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_EQUIP_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FLAG_MANAGER_GROUP_INIT_KEY;
    pub(crate) use crate::game::movie::movieplayer::callee::C_OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::EVENT_SELECT_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_BACKGROUND_SCALE;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_CONFIG;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_RESET_LAYOUT;
    pub(crate) use crate::game::movie::movieplayer::callee::FILE_LOAD;
    pub(crate) use crate::game::movie::movieplayer::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::movie::movieplayer::callee::FONT_PARSE_GLYPHS;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_OVERLAY_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_STATE as GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN_DISCONNECT as LOBBY_MAN_DISCONNECT_2;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_CREATE_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_JOIN_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_ON_OPERATION_DONE;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_SET_MODE_2 as MC_MANAGER_SET_MODE;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_ALLOC_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_ON_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_SET_SELECTION;
    pub(crate) use crate::game::movie::movieplayer::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::movie::movieplayer::callee::MULTI_SPLINE_MODIFIER_ADD_REF;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_SET_QUALITY_ICONS;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_CTOR_5 as NET_FE_STATE_CTOR_3;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_HANDLE_NET_EVENT as NET_FE_STATE_HANDLE_NET_EVENT_2;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PDA_TEMPLATE_OPEN;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PROFILE_SAVE_PDA_INIT_SLOT_LABELS;
    pub(crate) use crate::game::movie::movieplayer::callee::PARENT_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_ON_SPLIT_MARKER;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::ROLLER_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_DATA_GET_INFO as SCRIPT_DATA_GET_INFO_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_FIND_SOUND_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT as SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_RUN_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_UNLOAD_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::SPRINTF_2 as SPRINTF_3;
    pub(crate) use crate::game::movie::movieplayer::callee::STRCPY_2 as STRCPY_4;
    pub(crate) use crate::game::movie::movieplayer::callee::STREAM_BUFFER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_LOAD_TEXTURES;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_STORE_WORD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR_4;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_MGR_ADD_BACKGROUND_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESSES;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESS_COUNT;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_MAN_CREATE_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER as BE_GET_RIDER_CHARACTER_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_ALGORITHM_DEFAULT_FOV;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_PROCESS_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_PACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_UNPACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_MASK_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_CURSOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_ADD_VERTEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_CLOSE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MATH_EVAL_CUBIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET as MC_MANAGER_GET_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_MAN_FREE_ENTRY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_DRAW_ROW_4 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2 as SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3 as SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_FIRST_5 as UI_LIST_FIRST_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_SCREEN_FIND_OBJECT_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;

    pub const C00113f88: Callee = Callee {
        addr: 0x00113f88,
        f: ee::s00::Rider_SetSlew3And4Targets,
    };

    pub const C00115ab0: Callee = Callee {
        addr: 0x00115ab0,
        f: ee::s00::Rider_GetGateChargeAngle,
    };

    pub const C0011fe78: Callee = Callee {
        addr: 0x0011fe78,
        f: ee::s00::Rider_SetMainState,
    };

    pub const C0011fe98: Callee = Callee {
        addr: 0x0011fe98,
        f: ee::s00::Rider_GetMainState,
    };

    pub const C0011fec8: Callee = Callee {
        addr: 0x0011fec8,
        f: ee::s00::Rider_SetSubState,
    };

    pub const C00125108: Callee = Callee {
        addr: 0x00125108,
        f: ee::s00::Rider_OnFinishRace,
    };

    pub const C0014a080: Callee = Callee {
        addr: 0x0014a080,
        f: ee::s01::BE_GetRiderCharacter,
    };

    pub const C0014a0e0: Callee = Callee {
        addr: 0x0014a0e0,
        f: ee::s01::BE_GetRiderProfile,
    };

    pub const C0014bdb8: Callee = Callee {
        addr: 0x0014bdb8,
        f: ee::s01::Bolts_GetDB,
    };

    pub const C0014d448: Callee = Callee {
        addr: 0x0014d448,
        f: ee::s01::Bolts_FindBoltIndex,
    };

    pub const C0014d988: Callee = Callee {
        addr: 0x0014d988,
        f: ee::s01::Bolts_GetCharBoltCount,
    };

    pub const C0014d998: Callee = Callee {
        addr: 0x0014d998,
        f: ee::s01::Bolts_GetCharBolts,
    };

    pub const C0014dc80: Callee = Callee {
        addr: 0x0014dc80,
        f: ee::s01::BELibrary_GetInstance,
    };

    pub const C0014dd58: Callee = Callee {
        addr: 0x0014dd58,
        f: ee::s01::BELibrary_GetInterface,
    };

    pub const C00150928: Callee = Callee {
        addr: 0x00150928,
        f: ee::s01::BEEconInt_GetMoney,
    };

    pub const C00150b48: Callee = Callee {
        addr: 0x00150b48,
        f: ee::s01::BEEconInt_SpendMoney,
    };

    pub const C00151c48: Callee = Callee {
        addr: 0x00151c48,
        f: ee::s01::BE_SetCharProfileBoltOwned,
    };

    pub const C001535c0: Callee = Callee {
        addr: 0x001535c0,
        f: ee::s01::BEMissionInt_CountCollectedOnPeak,
    };

    pub const C001543e0: Callee = Callee {
        addr: 0x001543e0,
        f: ee::s01::BEMissionInt_CountCompletedOnPeak,
    };

    pub const C00156988: Callee = Callee {
        addr: 0x00156988,
        f: ee::s01::BE_RestoreCharacter,
    };

    pub const C00156a60: Callee = Callee {
        addr: 0x00156a60,
        f: ee::s01::BERewardInt_IsBetterPlace,
    };

    pub const C001577e0: Callee = Callee {
        addr: 0x001577e0,
        f: ee::s01::BERewardInt_IsPeakFullyCompleted,
    };

    pub const C00157bf0: Callee = Callee {
        addr: 0x00157bf0,
        f: ee::s01::BERewardInt_IsPeakGoalComplete,
    };

    pub const C00158910: Callee = Callee {
        addr: 0x00158910,
        f: ee::s01::BERewardInt_GetEventBestPlace,
    };

    pub const C001589b0: Callee = Callee {
        addr: 0x001589b0,
        f: ee::s01::BERewardInt_GetCollectibleTier,
    };

    pub const C00158a50: Callee = Callee {
        addr: 0x00158a50,
        f: ee::s01::BERewardInt_GetMissionTier,
    };

    pub const C00158af0: Callee = Callee {
        addr: 0x00158af0,
        f: ee::s01::BERewardInt_HasPointsTier,
    };

    pub const C00158c80: Callee = Callee {
        addr: 0x00158c80,
        f: ee::s01::BERewardInt_GetNextCollectibleThreshold,
    };

    pub const C00158d58: Callee = Callee {
        addr: 0x00158d58,
        f: ee::s01::BERewardInt_GetNextMissionThreshold,
    };

    pub const C00159170: Callee = Callee {
        addr: 0x00159170,
        f: ee::s01::BERewardInt_GrantMedalReward,
    };

    pub const C00159cd0: Callee = Callee {
        addr: 0x00159cd0,
        f: ee::s01::BERewardInt_GrantReward,
    };

    pub const C0015a2e0: Callee = Callee {
        addr: 0x0015a2e0,
        f: ee::s01::BERewardInt_IsRewardGranted,
    };

    pub const C0016c6b0: Callee = Callee {
        addr: 0x0016c6b0,
        f: ee::s02::ActiveCameraTriggerList_Find,
    };

    pub const C0016c718: Callee = Callee {
        addr: 0x0016c718,
        f: ee::s02::ActiveCameraTriggerList_Add,
    };

    pub const C00176890: Callee = Callee {
        addr: 0x00176890,
        f: ee::s02::CameraTriggerMan_CollectOctreeNodeTriggers,
    };

    pub const C0017f500: Callee = Callee {
        addr: 0x0017f500,
        f: ee::s02::DebugMenuModule_DrawCursorTrail,
    };

    pub const C0019e2b0: Callee = Callee {
        addr: 0x0019e2b0,
        f: ee::s03::BoltFile_GetData,
    };

    pub const C0019f878: Callee = Callee {
        addr: 0x0019f878,
        f: ee::s03::FEModule_RiderPreviewFindTexture,
    };

    pub const C001bb9b0: Callee = Callee {
        addr: 0x001bb9b0,
        f: ee::s03::cFEStateCreateStep2_GetDaysInMonth,
    };

    pub const C001c84a8: Callee = Callee {
        addr: 0x001c84a8,
        f: ee::s03::cFEPopupConfirm_MeasureLine,
    };

    pub const C001c8568: Callee = Callee {
        addr: 0x001c8568,
        f: ee::s03::cFEPopupConfirm_MeasureLineW,
    };

    pub const C001c9b68: Callee = Callee {
        addr: 0x001c9b68,
        f: ee::s03::cFEPopupConfirm_AccumulateSize,
    };

    pub const C001cb9b0: Callee = Callee {
        addr: 0x001cb9b0,
        f: ee::s04::cKeyboardPopup_MoveDown,
    };

    pub const C001cbc48: Callee = Callee {
        addr: 0x001cbc48,
        f: ee::s04::cKeyboardPopup_MoveDownFromTopRow,
    };

    pub const C001cbce8: Callee = Callee {
        addr: 0x001cbce8,
        f: ee::s04::cKeyboardPopup_MoveDownFromBottomRow,
    };

    pub const C001cbfc0: Callee = Callee {
        addr: 0x001cbfc0,
        f: ee::s04::cKeyboardPopup_MoveDownFromRow4,
    };

    pub const C001ce560: Callee = Callee {
        addr: 0x001ce560,
        f: ee::s04::cKeyboardPopup_IsKeyEnabled,
    };

    pub const C001d32e0: Callee = Callee {
        addr: 0x001d32e0,
        f: ee::s04::cFEStateMountainRoom_ShowImage,
    };

    pub const C001e1a30: Callee = Callee {
        addr: 0x001e1a30,
        f: ee::s04::MessageCenter_MatchCharacter,
    };

    pub const C001e3388: Callee = Callee {
        addr: 0x001e3388,
        f: ee::s04::MessageCenter_FindMessage,
    };

    pub const C001e91a8: Callee = Callee {
        addr: 0x001e91a8,
        f: ee::s04::HUD_GetElemColor,
    };

    pub const C001e91f8: Callee = Callee {
        addr: 0x001e91f8,
        f: ee::s04::HUD_GetElemPos,
    };

    pub const C001e9290: Callee = Callee {
        addr: 0x001e9290,
        f: ee::s04::HUD_GetElemScale,
    };

    pub const C001f10f8: Callee = Callee {
        addr: 0x001f10f8,
        f: ee::s04::HUD_AlignElemPos,
    };

    pub const C0020e900: Callee = Callee {
        addr: 0x0020e900,
        f: ee::s05::cOVStateOverlay_Update,
    };

    pub const C00224c78: Callee = Callee {
        addr: 0x00224c78,
        f: ee::s05::LunoVMRegister_ReadPtr,
    };

    pub const C00224da0: Callee = Callee {
        addr: 0x00224da0,
        f: ee::s05::cLunoTable_ctor,
    };

    pub const C00224df0: Callee = Callee {
        addr: 0x00224df0,
        f: ee::s05::cLunoTable_dtor,
    };

    pub const C00225068: Callee = Callee {
        addr: 0x00225068,
        f: ee::s05::cLunoTable_Assign,
    };

    pub const C002259a8: Callee = Callee {
        addr: 0x002259a8,
        f: ee::s05::cLunoTable_Read,
    };

    pub const C00227f80: Callee = Callee {
        addr: 0x00227f80,
        f: ee::s05::MCManager_Get,
    };

    pub const C00231f80: Callee = Callee {
        addr: 0x00231f80,
        f: ee::s06::LoadIndicator_Advance,
    };

    pub const C00238bf8: Callee = Callee {
        addr: 0x00238bf8,
        f: ee::s06::GameModeMan_SortByTime,
    };

    pub const C0026d4d8: Callee = Callee {
        addr: 0x0026d4d8,
        f: ee::s07::RunList_Seek,
    };

    pub const C0026db88: Callee = Callee {
        addr: 0x0026db88,
        f: ee::s07::Replay_RestoreSnapshot,
    };

    pub const C0026e448: Callee = Callee {
        addr: 0x0026e448,
        f: ee::s07::Replay_FindSnapshot,
    };

    pub const C0026f4a0: Callee = Callee {
        addr: 0x0026f4a0,
        f: ee::s07::Replay_SetState,
    };

    pub const C002702f8: Callee = Callee {
        addr: 0x002702f8,
        f: ee::s07::Replay_SeekToSnapshot,
    };

    pub const C002703f0: Callee = Callee {
        addr: 0x002703f0,
        f: ee::s07::Replay_StepFrame,
    };

    pub const C00275498: Callee = Callee {
        addr: 0x00275498,
        f: ee::s07::ScriptAnimBankMan_dtor,
    };

    pub const C002758c0: Callee = Callee {
        addr: 0x002758c0,
        f: ee::s07::DataInstanceMan_dtor,
    };

    pub const C00275b98: Callee = Callee {
        addr: 0x00275b98,
        f: ee::s07::DataInstanceMan_AddBigEntries,
    };

    pub const C00275cf8: Callee = Callee {
        addr: 0x00275cf8,
        f: ee::s07::DataInstanceMan_AllocInfo,
    };

    pub const C00278308: Callee = Callee {
        addr: 0x00278308,
        f: ee::s07::ScriptEngine_UnloadMaster,
    };

    pub const C00278e50: Callee = Callee {
        addr: 0x00278e50,
        f: ee::s07::ScriptEngine_ListAddChoice,
    };

    pub const C00278f38: Callee = Callee {
        addr: 0x00278f38,
        f: ee::s07::ScriptEngine_ListPrepare,
    };

    pub const C00278f68: Callee = Callee {
        addr: 0x00278f68,
        f: ee::s07::ScriptEngine_ListStart,
    };

    pub const C00281ed8: Callee = Callee {
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

    pub const C002837c8: Callee = Callee {
        addr: 0x002837c8,
        f: ee::s07::ScriptMovie_dtor,
    };

    pub const C0028b180: Callee = Callee {
        addr: 0x0028b180,
        f: ee::s07::SSXAudioSystem_GetInstance,
    };

    pub const C0028bb10: Callee = Callee {
        addr: 0x0028bb10,
        f: ee::s07::SndBankMan_dtor,
    };

    pub const C0028e8c0: Callee = Callee {
        addr: 0x0028e8c0,
        f: ee::s07::SSXAudioSystem_HandleMusicEvent,
    };

    pub const C002913d8: Callee = Callee {
        addr: 0x002913d8,
        f: ee::s07::SSXAudioSystem_StopAmbienceLoop,
    };

    pub const C003457c8: Callee = Callee {
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

    pub const C003625c0: Callee = Callee {
        addr: 0x003625c0,
        f: ee::s12::GsPkt_AddClamp,
    };

    pub const C00362660: Callee = Callee {
        addr: 0x00362660,
        f: ee::s12::GsPkt_AddZbuf,
    };

    pub const C003626d8: Callee = Callee {
        addr: 0x003626d8,
        f: ee::s12::GsPkt_AddTest,
    };

    pub const C003627a8: Callee = Callee {
        addr: 0x003627a8,
        f: ee::s12::GsPkt_Finish,
    };

    pub const C00364cd0: Callee = Callee {
        addr: 0x00364cd0,
        f: ee::s12::DeferMan_EmitMaterialSetup,
    };

    pub const C00366ce0: Callee = Callee {
        addr: 0x00366ce0,
        f: ee::s12::VramHeap_Alloc,
    };

    pub const C00368170: Callee = Callee {
        addr: 0x00368170,
        f: ee::s12::TexMan_EmitImageUpload,
    };

    pub const C003684f0: Callee = Callee {
        addr: 0x003684f0,
        f: ee::s12::TexMan_EmitClutUpload,
    };

    pub const C00368970: Callee = Callee {
        addr: 0x00368970,
        f: ee::s12::TexMan_EmitTex0,
    };

    pub const C0038f460: Callee = Callee {
        addr: 0x0038f460,
        f: ee::s12::SprStream_BeginBlock,
    };

    pub const C0038f668: Callee = Callee {
        addr: 0x0038f668,
        f: ee::s12::SprStream_EndBlockAligned,
    };

    pub const C0039c870: Callee = Callee {
        addr: 0x0039c870,
        f: ee::s13::UIScreen_StartThread,
    };

    pub const C0039ff78: Callee = Callee {
        addr: 0x0039ff78,
        f: ee::s13::UIObject_GetEventThread,
    };

    pub const C003b1300: Callee = Callee {
        addr: 0x003b1300,
        f: ee::s13::MPEG2_Get_Bits,
    };

    pub const C003b2360: Callee = Callee {
        addr: 0x003b2360,
        f: ee::s13::MPEG2_marker_bit,
    };
}

/// Cross-shard call targets (the generator emits `call_indirect` for them).
pub(crate) mod table {
    pub(crate) use crate::game::ai::computer::table::DELETE;
    pub(crate) use crate::game::ai::computer::table::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::ai::computer::table::EA_MEM_FREE;
    pub(crate) use crate::game::ai::computer::table::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::ai::computer::table::SLEEP_THREAD;
    pub(crate) use crate::game::ai::computer::table::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::ai::computer::table::STR_HASH;
    pub(crate) use crate::game::ai::computer::table::WAIT_SEMA;
    pub(crate) use crate::game::ai::computer::table::WAKEUP_THREAD;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {
    pub(crate) use crate::game::fe::festaterewards::table_callee::AI_SET_AI_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::APP_MAN_SET_NEXT_MODULE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BENEW_PLAYER_INT_GET_CHAR_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_INTERFACE_FV;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_TEXT_FROM_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_SET_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUDDY_PRODUCT_COMPARE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUY_POPUP_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::CODEC_DECODE_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_BIG_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_SMALL_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::festaterewards::table_callee::MUSIC_SLOT_RELEASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CHEAT_CHARS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CONCEPT_ARTS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_GET_BASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_POSTERS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TOYS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TRADING_CARDS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_VIDEOS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_REMOVE_SCRIPT_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SCRIPT_CLEANUP;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SET_SCRIPT_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SSX_AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRCPY;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRLEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_GET_STRING4;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_GET_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OBJECT_PUSH;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OPEN_FINISH_SCREEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_ADD_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_STATE_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_RICH_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UPLOAD_TEXTURE_PIXELS;
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {
    pub(crate) use crate::game::fe::feasyncfile::addr::__BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::addr::__FIXUNSSFSI;
    pub(crate) use crate::game::fe::feasyncfile::addr::C_BX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::addr::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::addr::FONT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::addr::STRLEN;
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
                c.sdr(2, (c.g[SP][0] as u32));
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
                c.ldr(8, (c.g[SP][0] as u32));
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
        let col = abi::arg(c, 1);
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
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[RA][0] = sx(0x001cc64c);
            c.g[A0][0] = c.g[S2][0];
            if call_ptr(c, 0x00416810, 0x001cc64c).is_err() {
                return;
            }
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x134));
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            let t = c.g[A0][0] != c.g[V0][0];
            c.sb((c.g[V1][0] as u32), c.g[S0][0]);
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
                        c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.sb((c.g[V1][0] as u32), 0u64);
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
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sb((c.g[V0][0] as u32), 0u64);
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
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
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
        let dst = abi::arg(c, 1);
        let text = abi::arg(c, 2);
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
                c.ldr(7, (c.g[V1][0] as u32));
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
                let overflow = !c.fcc();
                c.g[A1][0] = c.g[S1][0];
                if overflow {
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
                            if callee::C001ce560.call(c, 0x001cba58).is_err() {
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
                            if callee::C001cbfc0.call(c, 0x001cba84).is_err() {
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
                            if callee::C001cbce8.call(c, 0x001cbaa4).is_err() {
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
                        if callee::C001cbc48.call(c, 0x001cbabc).is_err() {
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
                if callee::C001ce560.call(c, 0x001cbb34).is_err() {
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
                    if callee::C001cb9b0.call(c, 0x001cbb48).is_err() {
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
                                        if callee::C001c84a8.call(c, 0x001c92d4).is_err() {
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
                                if callee::C001c84a8.call(c, 0x001c92fc).is_err() {
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
                        if callee::C001c9b68.call(c, 0x001c9314).is_err() {
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
            if callee::C001c84a8.call(c, 0x001c935c).is_err() {
                return;
            }
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001c936c);
            c.g[A2][0] = c.g[SP][0];
            if callee::C001c9b68.call(c, 0x001c936c).is_err() {
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
                                            if callee::C001c8568.call(c, 0x001c9108).is_err() {
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
                                if callee::C001c8568.call(c, 0x001c9138).is_err() {
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
                        if callee::C001c9b68.call(c, 0x001c9150).is_err() {
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
            if callee::C001c8568.call(c, 0x001c91a4).is_err() {
                return;
            }
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x001c91b4);
            c.g[A2][0] = c.g[SP][0];
            if callee::C001c9b68.call(c, 0x001c91b4).is_err() {
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
        let mut v0 = ((c.g[A0][0] as i64) < 2) as u64;
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
                v0 = 2;
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fakes::*;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_3;
    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::bx::bxstring2::tests::alloc_base;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_11;
    pub(crate) use crate::game::bx::bxstring2::tests::f;
    pub(crate) use crate::game::bx::bxstring2::tests::run as run_3;
    pub(crate) use crate::game::bx::bxstring2::tests::slot_2 as slot_6;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE_5;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2 as W_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::SEEDS_6;
    pub(crate) use crate::game::dirtysock::tags2::tests::alloc_ready;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::dirtysock::tags2::tests::clear;
    pub(crate) use crate::game::dirtysock::tags2::tests::common;
    pub(crate) use crate::game::dirtysock::tags2::tests::dummy_world;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::load;
    pub(crate) use crate::game::dirtysock::tags2::tests::zero;
    pub(crate) use crate::game::dirtysock::tags2::tests::SEEDS_7 as SEEDS_2;
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
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_5 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_6 as globals_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::hash_screen;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_10 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::init_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_10 as pool_allocator_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui_init_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::widget_action_test;
    pub(crate) use crate::game::fe::feasyncfile::tests::world;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_4;
    pub(crate) use crate::game::fe::fememcard2::tests::hash_screen as hash_screen_2;
    pub(crate) use crate::game::fe::fememcard2::tests::str_hash as str_hash_2;
    pub(crate) use crate::game::fe::fepopup3::tests::setup;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_8;
    pub(crate) use crate::game::fe::festaterewards::tests::globals_3 as globals_6;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_init_3 as heap_init;
    pub(crate) use crate::game::fe::festaterewards::tests::pool_4;
    pub(crate) use crate::game::fe::festaterewards::tests::slot_2 as slot_4;
    pub(crate) use crate::game::fe::festaterewards::tests::w16;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_6;
    pub(crate) use crate::game::main::gamestate::tests::slot_2 as slot_3;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_4;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base_4;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_LIBRARY;
    pub(crate) use crate::game::movie::movieplayer::tests::W;
    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_7;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals_5;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::run;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_13;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7 as globals_8;
    pub(crate) use crate::game::sound::soundsys::tests::load_2;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool_5;
    pub(crate) use crate::game::ui::uivector::tests::start;
    pub(crate) use crate::game::util::menu::tests::wf_2 as wf;
    pub(crate) use crate::game::world::worldcache::tests::check_all as check_all_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_9;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals_7;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD_2;
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
        let mut r = machine(c, seed, salt, calls);
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

    pub(crate) const LOBBY: u32 = DATA + 0x1_0000;

    pub(crate) const LOBBY_MSG: u32 = DATA + 0x1_1000;

    pub(crate) const LOBBY_NAMES: u32 = DATA + 0x1_2000;

    pub(crate) const DBG_VT: u32 = DATA + 0x1_2000;

    pub(crate) const LOBBY2: u32 = DATA + 0x1_0000;

    pub(crate) const LOBBY2_MSG: u32 = DATA + 0x1_1000;

    pub(crate) const SCRIPT_SYS: u32 = DATA + 0x1_0000;

    pub(crate) const REPLAY: u32 = DATA + 0x1_0000;

    pub(crate) const REPLAY_FRAME: u32 = DATA + 0x1_1000;

    pub(crate) const REPLAY_VIEW: u32 = DATA + 0x1_2000;

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

    pub(crate) const PTR_VEC: u32 = DATA + 0x1_0000;

    pub(crate) const PTR_BUF: u32 = DATA + 0x1_1000;

    pub(crate) const CHAL_VEC: u32 = DATA + 0x1_0000;

    pub(crate) const CHAL_BUF: u32 = DATA + 0x1_1000;

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
}
