//! `src/fe/fepopup3.cpp`: hand-written replacements (moved from the
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
pub(crate) use crate::game::ai::ai::GAME_MODULE_GP;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::animation::rideranimbase::sw as sw_3;
pub(crate) use crate::game::be::be::vt_call;
pub(crate) use crate::game::dirtysock::tags2::gp as gp_3;
pub(crate) use crate::game::dirtysock::tags2::gp_8 as gp;
pub(crate) use crate::game::dirtysock::tags2::leave;
pub(crate) use crate::game::dirtysock::tags2::lw;
pub(crate) use crate::game::dirtysock::tags2::vslot;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::feasyncfile::at as at_4;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::feasyncfile::at_5;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_4;
pub(crate) use crate::game::fe::feasyncfile::gpr;
pub(crate) use crate::game::fe::feasyncfile::low_2;
pub(crate) use crate::game::fe::feasyncfile::lw_3;
pub(crate) use crate::game::fe::feasyncfile::lw_5;
pub(crate) use crate::game::fe::feasyncfile::lw_6;
pub(crate) use crate::game::fe::feasyncfile::lw_7;
pub(crate) use crate::game::fe::feasyncfile::lw_8;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_3;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::sb_r;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw_2;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::feasyncfile::u32_at;
pub(crate) use crate::game::fe::feasyncfile::ui_init_screen;
pub(crate) use crate::game::fe::fepopup::at as at_7;
pub(crate) use crate::game::fe::fepopup::hide_show_widget;
pub(crate) use crate::game::fe::fepopup::lb as lb_2;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::sw_4;
pub(crate) use crate::game::fe::festaterewards::at_4 as at_3;
pub(crate) use crate::game::fe::festaterewards::gp_4 as gp_5;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_4;
pub(crate) use crate::game::fe::ovstates::at as at_6;
pub(crate) use crate::game::fe::ovstates::close;
pub(crate) use crate::game::fe::ovstates::open;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at_4;
pub(crate) use crate::game::main::ssxappdtor::p;
pub(crate) use crate::game::main::ssxappdtor::vtable_dtor;
pub(crate) use crate::game::movie::movieplayer::lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at_3;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
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
        0x001d_f028,
        fe_popup_screen_pos_v23,
        "cFEPopupScreenPos_v23",
    ),
    (
        0x001d_f440,
        rule_select_member_dtor,
        "RuleSelectMember_dtor",
    ),
    (
        0x001d_f4e8,
        cfe_state_bragging_rights_dtor,
        "cFEStateBraggingRights_dtor",
    ),
    (
        0x001d_f7c8,
        cfe_state_lobby_menu_dtor,
        "cFEStateLobbyMenu_dtor",
    ),
    (
        0x001d_f878,
        c_fe_state_net_create_room_popup_dtor,
        "cFEStateNetCreateRoomPopup_dtor",
    ),
    (
        0x001d_fa10,
        cfe_state_reward_gallery_dtor,
        "cFEStateRewardGallery_dtor",
    ),
    (
        0x001d_fb28,
        cfe_state_reward_gallery_posters_dtor,
        "cFEStateRewardGalleryPosters_dtor",
    ),
    (
        0x001d_fb98,
        fe_state_reward_gallery_posters_get_item_price,
        "cFEStateRewardGalleryPosters_GetItemPrice",
    ),
    (
        0x001d_fbf8,
        cfe_state_reward_gallery_trading_cards_dtor,
        "cFEStateRewardGalleryTradingCards_dtor",
    ),
    (
        0x001d_fc68,
        fe_state_reward_gallery_trading_cards_get_item_price,
        "cFEStateRewardGalleryTradingCards_GetItemPrice",
    ),
    (
        0x001d_fcc8,
        cfe_state_reward_gallery_concept_arts_dtor,
        "cFEStateRewardGalleryConceptArts_dtor",
    ),
    (
        0x001d_fd38,
        fe_state_reward_gallery_concept_arts_get_item_price,
        "cFEStateRewardGalleryConceptArts_GetItemPrice",
    ),
    (
        0x001d_fd98,
        cfe_state_reward_gallery_videos_dtor,
        "cFEStateRewardGalleryVideos_dtor",
    ),
    (
        0x001d_fe08,
        fe_state_reward_gallery_videos_get_item_price,
        "cFEStateRewardGalleryVideos_GetItemPrice",
    ),
    (
        0x001d_fe68,
        cfe_state_reward_gallery_toys_dtor,
        "cFEStateRewardGalleryToys_dtor",
    ),
    (
        0x001d_fed8,
        fe_state_reward_gallery_toys_get_item_price,
        "cFEStateRewardGalleryToys_GetItemPrice",
    ),
    (
        0x001d_ff38,
        cfe_state_reward_gallery_cheat_chars_dtor,
        "cFEStateRewardGalleryCheatChars_dtor",
    ),
    (
        0x001d_ffa8,
        fe_state_reward_gallery_cheat_chars_get_item_price,
        "cFEStateRewardGalleryCheatChars_GetItemPrice",
    ),
    (
        0x001e_0210,
        c_fe_state_trophy_room_dtor,
        "cFEStateTrophyRoom_dtor",
    ),
    (
        0x001e_02f8,
        cfe_popup_select_multiplayer_mode_init,
        "cFEPopupSelectMultiplayerMode_Init",
    ),
    (
        0x001e_03f8,
        c_fe_popup_select_multiplayer_mode_on_widget_action,
        "cFEPopupSelectMultiplayerMode_OnWidgetAction",
    ),
    (
        0x001e_0470,
        c_fe_popup_select_multiplayer_mode_on_widget_notify,
        "cFEPopupSelectMultiplayerMode_OnWidgetNotify",
    ),
    (0x001e_0740, fe_ui_sound_v01, "FEUISound_v01"),
    (0x001e_0780, ov_ui_sound_v01, "OVUISound_v01"),
    (
        0x001e_09a0,
        cfe_state_challenge_popup_dtor,
        "cFEStateChallengePopup_dtor",
    ),
    (
        0x001e_0a50,
        c_fe_state_challenge_received_popup_dtor,
        "cFEStateChallengeReceivedPopup_dtor",
    ),
    (
        0x001e_0cf8,
        c_fe_state_create_step1_dtor,
        "cFEStateCreateStep1_dtor",
    ),
    (
        0x001e_0fc0,
        c_fe_state_net_messenger_transition_in,
        "cFEStateNetMessenger_TransitionIn",
    ),
    (0x001e_1048, ui_sound_base_dtor, "UISoundBase_dtor"),
    (0x001e_1090, fe_locale_set_flag27, "FELocale_SetFlag27"),
    (0x001e_10b8, ui_custom_base_dtor, "UICustomBase_dtor"),
    (0x001e_1108, ui_input_base_dtor, "UIInputBase_dtor"),
    (
        0x001e_14c0,
        message_center_trigger_groups,
        "MessageCenter_TriggerGroups",
    ),
    (
        0x001e_1ad0,
        message_center_pick_foreshadow_message,
        "MessageCenter_PickForeshadowMessage",
    ),
    (
        0x001e_2370,
        message_center_set_peak_foreshadow_done,
        "MessageCenter_SetPeakForeshadowDone",
    ),
    (
        0x001e_2428,
        message_center_post_peak_unlock_message,
        "MessageCenter_PostPeakUnlockMessage",
    ),
    (
        0x001e_2518,
        message_center_post_once_3c,
        "MessageCenter_PostOnce3C",
    ),
    (
        0x001e_2578,
        message_center_post_once_38,
        "MessageCenter_PostOnce38",
    ),
    (
        0x001e_25d8,
        message_center_on_bc_finish,
        "MessageCenter_OnBCFinish",
    ),
    (
        0x001e_2bb8,
        message_center_count_group_messages,
        "MessageCenter_CountGroupMessages",
    ),
    (
        0x001e_2d58,
        message_center_is_message_of_type,
        "MessageCenter_IsMessageOfType",
    ),
    (
        0x001e_2dc0,
        message_center_get_index_in_type,
        "MessageCenter_GetIndexInType",
    ),
    (
        0x001e_31b8,
        message_center_remove_message,
        "MessageCenter_RemoveMessage",
    ),
    (
        0x001e_3268,
        message_center_remove_message_by_id,
        "MessageCenter_RemoveMessageById",
    ),
    (
        0x001e_32c8,
        message_center_remove_messages_of_group,
        "MessageCenter_RemoveMessagesOfGroup",
    ),
    (
        0x001e_3388,
        message_center_find_message,
        "MessageCenter_FindMessage",
    ),
    (
        0x001e_3408,
        message_center_get_unread_count,
        "MessageCenter_GetUnreadCount",
    ),
    (
        0x001e_3510,
        message_center_reset_new_player_flag,
        "MessageCenter_ResetNewPlayerFlag",
    ),
    (
        0x001e_3570,
        message_center_is_new_player_flag_clear,
        "MessageCenter_IsNewPlayerFlagClear",
    ),
    (
        0x001e_3620,
        message_center_get_message_id,
        "MessageCenter_GetMessageId",
    ),
    (
        0x001e_3650,
        message_center_get_message_variant,
        "MessageCenter_GetMessageVariant",
    ),
    (
        0x001e_3680,
        messagecenter_get_list_index,
        "MessageCenter_GetListIndex",
    ),
    (
        0x001e_3a30,
        message_center_is_flag_set,
        "MessageCenter_IsFlagSet",
    ),
    (
        0x001e_3a78,
        message_center_set_flag,
        "MessageCenter_SetFlag",
    ),
    (
        0x001e_3c00,
        ov_screen_message_center_ctor,
        "OVScreenMessageCenter_ctor",
    ),
    (
        0x001e_3e30,
        ov_screen_message_center_shutdown,
        "OVScreenMessageCenter_Shutdown",
    ),
    (
        0x001e_4168,
        ov_screen_message_center_on_activate,
        "OVScreenMessageCenter_OnActivate",
    ),
    (
        0x001e_4610,
        ov_screen_message_center_on_child_event,
        "OVScreenMessageCenter_OnChildEvent",
    ),
    (
        0x001e_4758,
        ov_screen_message_center_update_count_text,
        "OVScreenMessageCenter_UpdateCountText",
    ),
    (
        0x001e_4840,
        ov_message_center_hide_widget,
        "OVScreenMessageCenter_HideWidget",
    ),
    (
        0x001e_4878,
        ov_message_center_show_widget,
        "OVScreenMessageCenter_ShowWidget",
    ),
    (
        0x001e_50c8,
        ov_screen_message_center_open_view_popup,
        "OVScreenMessageCenter_OpenViewPopup",
    ),
    (
        0x001e_5148,
        ovscreenmessagecenter_delete_message,
        "OVScreenMessageCenter_DeleteMessage",
    ),
    (
        0x001e_51d0,
        cov_state_view_popup_ctor,
        "cOVState_ViewPopup_ctor",
    ),
    (
        0x001e_5248,
        cov_state_view_popup_init,
        "cOVState_ViewPopup_Init",
    ),
    (
        0x001e_52d0,
        ov_state_view_popup_transition_in,
        "cOVState_ViewPopup_TransitionIn",
    ),
    (
        0x001e_55d0,
        cov_state_view_popup_set_subject,
        "cOVState_ViewPopup_SetSubject",
    ),
    (
        0x001e_5780,
        c_ov_state_view_popup_set_body,
        "cOVState_ViewPopup_SetBody",
    ),
    (
        0x001e_5800,
        c_ov_state_view_popup_on_widget_action,
        "cOVState_ViewPopup_OnWidgetAction",
    ),
    (
        0x001e_5a18,
        ov_state_view_popup_on_widget_notify,
        "cOVState_ViewPopup_OnWidgetNotify",
    ),
    (
        0x001e_5aa0,
        c_ov_state_view_popup_get_group_variant,
        "cOVState_ViewPopup_GetGroupVariant",
    ),
    (
        0x001e_65b8,
        ov_screen_freestyle_standings_handle_input,
        "OVScreenFreestyleStandings_HandleInput",
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
        0x001e_6668,
        ov_screen_final_standings_init,
        "OVScreenFinalStandings_Init",
    ),
    (
        0x001e_7558,
        ov_screen_final_standings_on_widget_created,
        "OVScreenFinalStandings_OnWidgetCreated",
    ),
    (
        0x001e_7aa8,
        ov_screen_final_standings_handle_input,
        "OVScreenFinalStandings_HandleInput",
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
        0x001e_8098,
        ov_screen_finish_ov_update,
        "OVScreenFinishOv_Update",
    ),
    (
        0x001e_8160,
        ov_screen_finish_ov_on_widget_action,
        "OVScreenFinishOv_OnWidgetAction",
    ),
    (
        0x001e_81d0,
        ov_screen_finish_ov_on_event,
        "OVScreenFinishOv_OnEvent",
    ),
    (
        0x001e_87b8,
        ov_screen_finish_ov2_update,
        "OVScreenFinishOv2_Update",
    ),
    (
        0x001e_8880,
        ov_screen_finish_ov2_on_widget_action,
        "OVScreenFinishOv2_OnWidgetAction",
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
        0x001e_8ed0,
        ov_screen_peak_challenge_results_on_activate,
        "OVScreenPeakChallengeResults_OnActivate",
    ),
    (
        0x001e_8f00,
        ov_screen_multiplayer_standings_init,
        "OVScreenMultiplayerStandings_Init",
    ),
    (
        0x001e_8fc8,
        ov_screen_multiplayer_standings_on_widget_action,
        "OVScreenMultiplayerStandings_OnWidgetAction",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    use super::*;
    pub(crate) use crate::game::ai::rider::callee::BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_COUNT as BUDDY_MAN_GET_ROSTER_COUNT_3;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_ENTRY as BUDDY_MAN_GET_ROSTER_ENTRY_2;
    pub(crate) use crate::game::ai::rider::callee::BUY_POPUP_INIT_BUY_BOLT;
    pub(crate) use crate::game::ai::rider::callee::CODEC_POP_FREE_FRAME;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_ALL_FIELDS_FILLED;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_BACKUP_ACCOUNT;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_ACCOUNT_NAME;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_CONFIRM_PASSWORD;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_EMAIL;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_INFORM;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_PASSWORD;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_SHARE_INFO;
    pub(crate) use crate::game::ai::rider::callee::CROWD_MAN_GROUP_FREE;
    pub(crate) use crate::game::ai::rider::callee::GAME_UI_OPEN_SCREEN;
    pub(crate) use crate::game::ai::rider::callee::IN_SEQ_PLAY;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_GET_LANGUAGE;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_SEND_REQUEST as LOBBY_MAN_SEND_REQUEST_2;
    pub(crate) use crate::game::ai::rider::callee::LOGIN_ON_ACCOUNT_MEM_POPUP;
    pub(crate) use crate::game::ai::rider::callee::LOGIN_ON_DISCONNECT_POPUP;
    pub(crate) use crate::game::ai::rider::callee::MESSAGE_CENTER_GET_LIST_INDEX;
    pub(crate) use crate::game::ai::rider::callee::MESSAGE_CENTER_OPEN_VIEW_POPUP;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::ai::rider::callee::NET_ACCOUNT_MEM_CTOR;
    pub(crate) use crate::game::ai::rider::callee::RAIL_MAN_ENABLE_RAIL;
    pub(crate) use crate::game::ai::rider::callee::RIDER_GET_START_DIST_TO_FINISH;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_2;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_OBJ_CTOR as SCRIPT_OBJ_CTOR_2;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_OBJ_DEACTIVATE;
    pub(crate) use crate::game::ai::rider::callee::SHOW_CHAR_STATS;
    pub(crate) use crate::game::ai::rider::callee::SHOW_CHECKING_CARD_POPUP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_GET_SURFACE_SOUND_TYPE;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_PLAY_FE_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_BASE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_EDGE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_SURFACE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_TRICK_METER_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::ai::rider::callee::START_CARD_SCAN;
    pub(crate) use crate::game::ai::rider::callee::STREAMER_GET_SECTION_COUNT;
    pub(crate) use crate::game::ai::rider::callee::SYNC_PROCESS_DTOR;
    pub(crate) use crate::game::ai::rider::callee::TEX_MAN_EVICT_TEXTURE;
    pub(crate) use crate::game::ai::rider::callee::VRAM_HEAP_FREE_TEXTURE_BLOCKS;
    pub(crate) use crate::game::ai::rider::callee::WORLD_CACHE_LOAD;
    pub(crate) use crate::game::ai::rider::callee::WORLD_CACHE_RESET;
    pub(crate) use crate::game::ai::rider::callee::WSCRIPT_CACHE_ALLOC;
    pub(crate) use crate::game::be::beintmission::callee::BUY_ATTRIB_UPDATE_BARS;
    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_EMPTY_STUB;
    pub(crate) use crate::game::be::beintmission::callee::CAREER_STATS_SHOW_HIGHLIGHTS;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::be::beintmission::callee::MEMCARD_SCAN_STEP;
    pub(crate) use crate::game::be::beintmission::callee::MISSION_COUNT;
    pub(crate) use crate::game::be::beintmission::callee::MISSION_NTH;
    pub(crate) use crate::game::be::beintmission::callee::MPEG2_ERROR;
    pub(crate) use crate::game::be::beintmission::callee::NET_FE_STATE_ON_WIDGET_ACTION;
    pub(crate) use crate::game::be::beintmission::callee::REWARDS_FILE_LOAD;
    pub(crate) use crate::game::be::beintmission::callee::UBER_TRICK_REFRESH;
    pub(crate) use crate::game::be::beintmission::callee::UITEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::be::beintmission::callee::VIEW_POPUP_SHOW_MESSAGE;
    pub(crate) use crate::game::be::beintmission::callee::WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
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
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_FREE as EA_MEM_FREE_3;
    pub(crate) use crate::game::bx::bxstring2::callee::FLAG_MANAGER_GROUP_BUILD_MESH;
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_SKIP_WHITESPACE;
    pub(crate) use crate::game::bx::bxstring2::callee::MEMSET;
    pub(crate) use crate::game::bx::bxstring2::callee::MESSAGE_CENTER_GET_INBOX as MESSAGE_CENTER_GET_INBOX_4;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_MAN_CLEAR;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_SET_EVENT_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_GET_POPUP as NET_FE_STATE_GET_POPUP_3;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_SHOW_POPUP as NET_FE_STATE_SHOW_POPUP_2;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_MANAGER;
    pub(crate) use crate::game::bx::bxstring2::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::bx::bxstring2::callee::OPTIONS_SET_VIBRATION;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_SCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_READ_FLOAT_3 as PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WIDGET_ALIGN;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WRAPPED_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::QSORT;
    pub(crate) use crate::game::bx::bxstring2::callee::READ_PRIVATE_MESSAGE_SET_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::RIDER_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_LIST_START as SCRIPT_ENGINE_LIST_START_2;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_SET_SKIP_DELAY;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_DEQUEUE_SECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_POP_FREE_NODE;
    pub(crate) use crate::game::bx::bxstring2::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT as SPEECH_SYSTEM_SET_EVENT_CONTEXT_2;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_SPEECH_CONTEXT_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::STRLEN;
    pub(crate) use crate::game::bx::bxstring2::callee::STR_HASH as STR_HASH_2;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_DESIRED_EYE;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_LOOK_AT;
    pub(crate) use crate::game::bx::bxstring2::callee::UITEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_ADD_ITEM_STR_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_FIND_ITEM_BY_VALUE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_FIND_FIRST_SELECTABLE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_SET_SELECTION as UI_MENU_SET_SELECTION_2;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_SCREEN_FIND_OBJECT as UI_SCREEN_FIND_OBJECT_3;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_FIND_STATE as UI_STATE_MGR_FIND_STATE_2;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_SHUTDOWN as UI_STATE_SHUTDOWN_3;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_CPY;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_FROM_ASCII_2 as USTR_FROM_ASCII;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_LEN;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_SPRINTF;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_MISSION_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT_2 as WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::dirtysock::tags2::callee::ANIM_GET_LAYER_SEQUENCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::APP_SOMETHING;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_APPLY_OUTPUT_MODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_SET_OUTPUT;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_BAKE_GROUP;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_EXPORT_HISTORY_FILES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_FREE_HISTORIES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_LOAD_HISTORY_FILE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_ADD_TIMER_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BENEW_PLAYER_INT_IS_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_BAGT_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_CHAR_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_BE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE_RIDER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_INTERFACE_CALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_NEW_PLAYER_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_RESET_PROFILE_CHARACTERS_2 as BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_REWARD_INT_IS_UNLOCKED;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_GET_NEXT_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOUNDED_CAMERA_ALGORITHM_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_DESTROY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_SET_PRESENCE_ADDR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAM_CONTROLLER_SET_ALGORITHM;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBE_GET_BE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBE_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBXSTRING_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_BUY_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_EQUIP_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY_2 as CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_MAN2D_INIT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_BX_STRING_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_LUNO_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_GET_SAVE_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_LIST_DIR_ALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_READ_SAVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_CREATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEMSET;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_2 as EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EFFECT_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::EFFECT_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_SET_PREVIEW_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_ON_ACTIVATE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_STATS_REFRESH_RESULTS_2 as FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_UPDATE_BUTTONS_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::FLOAT_TO_UINT;
    pub(crate) use crate::game::dirtysock::tags2::callee::FORMAT_TAGGED;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_00152948;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_00152BA8;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_002C2540;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_002C26D0;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_0039E2A0;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_COMM_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_PRE_RACE_MENU_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_ALT_CHAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_CHAR_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_FINISH_SPEECH_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_LANGUAGE_MASK as GET_LANGUAGE_MASK_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_RIDER_VOICE_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_TRANSITION_MAN;
    pub(crate) use crate::game::dirtysock::tags2::callee::GM_PICK_OPPONENTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_EVENT_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_RAMP_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_SPAN_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_LOCAL_PLAYER;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_PA_SPEECH_ON;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_RACE_TYPE_4OR5;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_REPLAY_PLAYING;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_SHOW;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBYMAN_SEND_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_API_ADD_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT_3 as LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LISTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LOBBY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_PEER_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOC_STRING_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_GET_PRODUCT_CODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_GET_SAVE_TYPE_TAG;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_END_OPERATION_2 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_SET_MODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEASURE_PLAIN;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEASURE_WRAP;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMORY_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMSET_3 as MEMSET_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_ASCII_CHAR_TO_SJIS;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_SET_USE_SAVE_TITLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT0;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT1;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT3;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT4;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT5;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT6;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_GET_INBOX as MESSAGE_CENTER_GET_INBOX_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_IS_FLAG_SET_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_MESSAGE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_SET_FLAG_2 as MESSAGE_CENTER_SET_FLAG_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_1;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_4;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_5;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_2 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FIND;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FLUSH;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_MESSENGER_SET_PLAYER_MENU_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_REMOVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::ON_QUIT_POPUP_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::PATH_PLAT_REFRESH_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_DECODE_FRAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_FRAME_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_ADD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_CACHE_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_END_PLAYBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR_2 as REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RECORD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RESTORE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SNAPSHOT_EMPTY_HOOK_B;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_STREAM_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_REFRESH_LIST;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_NOW_PLAYING;
    pub(crate) use crate::game::dirtysock::tags2::callee::REWARDS_FILE_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::RIDER_123168;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCE_MC_FORMAT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ANIM_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_DATA_GET_INFO_2 as SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CANCEL_BANK_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_FIND_BANK_SLOT_BY_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2 as SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_IS_BANK_SLOT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_OBJ_TABLE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS_2 as SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_FIELD_B0;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_CURRENT_SKIPPABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MOVIE_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_OBJ_CAM_SET_OWNER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_IS_EVENT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_PUSH_FREE_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUPMAN_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::dirtysock::tags2::callee::SHARED_LOGIN_MAN_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::SLOGIN_MC_GET_PORT_SLOT_2 as SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY_2 as SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLINK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_2 as SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNOW_BOX_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNPRINTF;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_SET_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPEECH_QUEUE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPEECH_SET_EVENT_CONTEXT as SPEECH_SET_EVENT_CONTEXT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_6 as SPRINTF_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_APP_START_FE_MODULE;
    pub(crate) use crate::game::dirtysock::tags2::callee::STATS_SELECT_CATEGORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::STREAM_BUFFER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::STR_HASH_4 as STR_HASH_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::STR_INTERN;
    pub(crate) use crate::game::dirtysock::tags2::callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::dirtysock::tags2::callee::TAG_FIELD_GET_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::TEXT_OBJ;
    pub(crate) use crate::game::dirtysock::tags2::callee::TRANSITION_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::TRANSITION_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::TROPHY_ROOM_RELEASE_IMAGES;
    pub(crate) use crate::game::dirtysock::tags2::callee::UIMENU_SET_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_LIST_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_GET_FONT;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_0;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_1;
    pub(crate) use crate::game::dirtysock::tags2::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_DEACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_MAN_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_STATE_RESTORE;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_VIEW_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::ACTIVE_CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE_2 as AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_MUSIC_TIER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_TRACK_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_PAUSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_SOMETHING;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_CLEAR_EMITTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_INIT_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_COUNT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_ENTRY_2 as BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_SET_OWNER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_FREE_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_PARSE_2 as CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_STACK_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBE_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_FILL_PLAYER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_GET_CHARACTER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_DAY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_GENDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_LEGAL;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_MONTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_YEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::EDITOR_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_CREATED_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_ERROR_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_SHOW_ROOM_TAB;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_UPDATE_PLAYERS_IN_ROOM;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_MANAGER_ADD_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_READ_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_BUILD_MESH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_SPRINGS_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_UPDATE_VERTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_ANIM_MAP_SET_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2 as GAME_PROCESS_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_FINISH_CLEAR_FLAG40;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_FLAG_MANAGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_LANGUAGE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_NEXT_SECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_EMPTY_STUB;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_NEXT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_PAUSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_HIDE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SHOW;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_SELECT_ALGORITHM;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_UPDATE_MODE1;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_UPDATE_STATES;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_EMPTY_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_IS_FLAG_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_DISCONNECT_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_12 as NET_FE_STATE_CTOR_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_4 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_4 as NET_FE_STATE_DTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_SELECTED_USER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT_5;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_4 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_9 as NET_FE_STATE_ON_CHILD_EVENT_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP_2 as NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_UPDATE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPTIONS_HUD_REFRESH_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_MESSAGE_CENTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_MESSAGE_CENTER_REFRESH_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_MESSAGE_CENTER_UPDATE_COUNT_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_MEM_CARD_CLEAR_DIR_CACHE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_PAD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_CACHE_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_INIT_PAGES;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_REQUEST_SHAPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SAY_20D3;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SAY_20D4;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SET_INPUT_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_BIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_3 as SCRIPT_CMD_QUEUE_AT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_IS_FULL;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_DATA_GET_BANK_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ACTIVATE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ALLOC_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_2 as SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3 as SCRIPT_ENGINE_GET_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2 as SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_2 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_3 as SCRIPT_OBJ_ACTIVATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CAM_BUILD_MATRIX_FROM_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_LOAD_BANK_FROM_MEMORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_FREE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_AABB_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COMPUTE_OFFSET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3 as SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRACK_TO_CATEGORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME_2 as UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2 as UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST as UI_LIST_FIRST_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_4 as UI_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3 as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM_2 as UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_2 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::VEC3_MAX_COMPONENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_BLOCK_POOL_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_STREAMER_UPDATE_VIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_MEMMAN_CREATE_SECTION_ALLOCATOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2_2 as WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_3 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_WRITE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_MANAGER_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_NODE_DTOR;
    pub(crate) use crate::game::fe::fepopup::callee::C00113f88;
    pub(crate) use crate::game::fe::fepopup::callee::C00115ab0;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe78;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe98;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fec8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a080;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a0e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0014bdb8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d448;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d988;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d998;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dc80;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dd58;
    pub(crate) use crate::game::fe::fepopup::callee::C00150b48;
    pub(crate) use crate::game::fe::fepopup::callee::C00151c48;
    pub(crate) use crate::game::fe::fepopup::callee::C001535c0;
    pub(crate) use crate::game::fe::fepopup::callee::C001543e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00156a60;
    pub(crate) use crate::game::fe::fepopup::callee::C001577e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00157bf0;
    pub(crate) use crate::game::fe::fepopup::callee::C001589b0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158a50;
    pub(crate) use crate::game::fe::fepopup::callee::C00158af0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158c80;
    pub(crate) use crate::game::fe::fepopup::callee::C00158d58;
    pub(crate) use crate::game::fe::fepopup::callee::C00159cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C0015a2e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c6b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c718;
    pub(crate) use crate::game::fe::fepopup::callee::C0017f500;
    pub(crate) use crate::game::fe::fepopup::callee::C0019e2b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0019f878;
    pub(crate) use crate::game::fe::fepopup::callee::C001bb9b0;
    pub(crate) use crate::game::fe::fepopup::callee::C001c84a8;
    pub(crate) use crate::game::fe::fepopup::callee::C001c8568;
    pub(crate) use crate::game::fe::fepopup::callee::C001c9b68;
    pub(crate) use crate::game::fe::fepopup::callee::C001cb9b0;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbc48;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbce8;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbfc0;
    pub(crate) use crate::game::fe::fepopup::callee::C001ce560;
    pub(crate) use crate::game::fe::fepopup::callee::C001d32e0;
    pub(crate) use crate::game::fe::fepopup::callee::C001e1a30;
    pub(crate) use crate::game::fe::fepopup::callee::C001e3388;
    pub(crate) use crate::game::fe::fepopup::callee::C001e91a8;
    pub(crate) use crate::game::fe::fepopup::callee::C001e91f8;
    pub(crate) use crate::game::fe::fepopup::callee::C001e9290;
    pub(crate) use crate::game::fe::fepopup::callee::C001f10f8;
    pub(crate) use crate::game::fe::fepopup::callee::C0020e900;
    pub(crate) use crate::game::fe::fepopup::callee::C00224c78;
    pub(crate) use crate::game::fe::fepopup::callee::C00224da0;
    pub(crate) use crate::game::fe::fepopup::callee::C00224df0;
    pub(crate) use crate::game::fe::fepopup::callee::C002259a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00227f80;
    pub(crate) use crate::game::fe::fepopup::callee::C00231f80;
    pub(crate) use crate::game::fe::fepopup::callee::C00238bf8;
    pub(crate) use crate::game::fe::fepopup::callee::C0026d4d8;
    pub(crate) use crate::game::fe::fepopup::callee::C0026db88;
    pub(crate) use crate::game::fe::fepopup::callee::C0026e448;
    pub(crate) use crate::game::fe::fepopup::callee::C0026f4a0;
    pub(crate) use crate::game::fe::fepopup::callee::C002702f8;
    pub(crate) use crate::game::fe::fepopup::callee::C002703f0;
    pub(crate) use crate::game::fe::fepopup::callee::C002758c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00275b98;
    pub(crate) use crate::game::fe::fepopup::callee::C00275cf8;
    pub(crate) use crate::game::fe::fepopup::callee::C00278e50;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f38;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f68;
    pub(crate) use crate::game::fe::fepopup::callee::C00281ed8;
    pub(crate) use crate::game::fe::fepopup::callee::C002837c8;
    pub(crate) use crate::game::fe::fepopup::callee::C0028b180;
    pub(crate) use crate::game::fe::fepopup::callee::C0028bb10;
    pub(crate) use crate::game::fe::fepopup::callee::C0028e8c0;
    pub(crate) use crate::game::fe::fepopup::callee::C002913d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003457c8;
    pub(crate) use crate::game::fe::fepopup::callee::C003625c0;
    pub(crate) use crate::game::fe::fepopup::callee::C003626d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003627a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00364cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C00366ce0;
    pub(crate) use crate::game::fe::fepopup::callee::C003684f0;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f460;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f668;
    pub(crate) use crate::game::fe::fepopup::callee::C0039c870;
    pub(crate) use crate::game::fe::fepopup::callee::C0039ff78;
    pub(crate) use crate::game::fe::fepopup::callee::C003b1300;
    pub(crate) use crate::game::fe::fepopup::callee::C003b2360;
    pub(crate) use crate::game::fe::fepopup::callee::C00125108;
    pub(crate) use crate::game::fe::fepopup::callee::C00150928;
    pub(crate) use crate::game::fe::fepopup::callee::C00156988;
    pub(crate) use crate::game::fe::fepopup::callee::C00158910;
    pub(crate) use crate::game::fe::fepopup::callee::C00159170;
    pub(crate) use crate::game::fe::fepopup::callee::C00176890;
    pub(crate) use crate::game::fe::fepopup::callee::C00225068;
    pub(crate) use crate::game::fe::fepopup::callee::C00275498;
    pub(crate) use crate::game::fe::fepopup::callee::C00278308;
    pub(crate) use crate::game::fe::fepopup::callee::C00283228;
    pub(crate) use crate::game::fe::fepopup::callee::C00283518;
    pub(crate) use crate::game::fe::fepopup::callee::C00358020;
    pub(crate) use crate::game::fe::fepopup::callee::C00362478;
    pub(crate) use crate::game::fe::fepopup::callee::C00362660;
    pub(crate) use crate::game::fe::fepopup::callee::C00368170;
    pub(crate) use crate::game::fe::fepopup::callee::C00368970;
    pub(crate) use crate::game::fe::festaterewards::callee::BECHAR_GET_OVERALL_RATING;
    pub(crate) use crate::game::fe::festaterewards::callee::BENEWPLAYER_GET_SLOT;
    pub(crate) use crate::game::fe::festaterewards::callee::BENEWPLAYER_IS_SHARING_CHAR_RECORD;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_ECON_GET_MONEY;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_ECON_SPEND_MONEY;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_LIBRARY_INSTANCE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_LIBRARY_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_REWARD_GET_FILE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_REWARD_SET_BIT;
    pub(crate) use crate::game::fe::festaterewards::callee::BRAGGING_RIGHTS_UPDATE_ROW_COLORS;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_OPPONENT_DISCONNECTED;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SERVER_TIMEOUT;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SHOW_CANCEL;
    pub(crate) use crate::game::fe::festaterewards::callee::GET_LANGUAGE_MASK_2;
    pub(crate) use crate::game::fe::festaterewards::callee::IS_ARCADE_SPEECH_ON;
    pub(crate) use crate::game::fe::festaterewards::callee::IS_DJ_SPEECH_ON_2;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_IS_FLAG_SET;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_POST;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_SET_FLAG;
    pub(crate) use crate::game::fe::festaterewards::callee::MOD_OBJ_RUN_SCRIPT_EVENT;
    pub(crate) use crate::game::fe::festaterewards::callee::REWARD_GALLERY_REGISTER_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_POP_FREE_NODE;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_PUSH_FREE_NODE;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_START_SCRIPT;
    pub(crate) use crate::game::fe::festaterewards::callee::SPEECH_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::festaterewards::callee::SPEECH_STOP_INSTANCE;
    pub(crate) use crate::game::fe::festaterewards::callee::TAG_FIELD_COPY_TAG;
    pub(crate) use crate::game::fe::festaterewards::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::fe::festaterewards::callee::TEXFLIP_SET_FRAME;
    pub(crate) use crate::game::fe::festaterewards::callee::VFX_PLANTS_GET_SWAY_MATRIX;
    pub(crate) use crate::game::main::gamestate::callee::ADF_DATA_SOURCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::ANIM_DATA_BASE_FREE_CHAN_CACHES;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_STOP_HANDLE as BANK_MONITOR_STOP_HANDLE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_ECON_INT_GET_TOTAL_EARNED;
    pub(crate) use crate::game::main::gamestate::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INTERFACE_3 as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::main::gamestate::callee::BOLTS_FIND_BOLT_INDEX;
    pub(crate) use crate::game::main::gamestate::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::main::gamestate::callee::C_GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::main::gamestate::callee::C_KEYBOARD_POPUP_SET_KEY_ENABLED;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4 as EA_MEM_FREE_2;
    pub(crate) use crate::game::main::gamestate::callee::GAME_ANIM_MAP_UNREGISTER_BANK;
    pub(crate) use crate::game::main::gamestate::callee::GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_CLEAR_SLOTS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_INSTALL_VIF_VU_HANDLERS;
    pub(crate) use crate::game::main::gamestate::callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS;
    pub(crate) use crate::game::main::gamestate::callee::MEM_FREE;
    pub(crate) use crate::game::main::gamestate::callee::MEM_MOVE;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_IS_DISABLED as MENU_SYS_ITEM_IS_DISABLED_2;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_NEXT as MENU_SYS_PAGE_SELECT_NEXT_2;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_PREV;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_COUNT_GROUP_MESSAGES;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_GET_MESSAGE_COUNT;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_REMOVE_MESSAGE;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_REFRESH_LIST;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_COUNT_TEXT;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::main::gamestate::callee::RIDER_CLEAR_NEARBY_LIGHTS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_PAUSE_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_RESUME_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_CMD_QUEUE_AT_2 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_SCRIPT_OBJ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_IS_OBJ_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_STOP_3 as SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_HAS_BINDING;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_IS_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SECTION_ALLOCATOR_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::SNDSTRM_CLOSE;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_MAKE_ID;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_INSTANCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_AABB;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_SPHERE_TREE;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_2 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_RESET_RACE_SOUNDS;
    pub(crate) use crate::game::main::gamestate::callee::STR_COPY;
    pub(crate) use crate::game::main::gamestate::callee::STR_HASH;
    pub(crate) use crate::game::main::gamestate::callee::STR_N_COPY;
    pub(crate) use crate::game::main::gamestate::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_FIRST_2 as UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_CTOR_3;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2;
    pub(crate) use crate::game::main::gamestate::callee::VD_TEXTURE_SINK_CTOR;
    pub(crate) use crate::game::main::ssxappdtor::callee::APP_MODULE_UPDATE;
    pub(crate) use crate::game::main::ssxappdtor::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::main::ssxappdtor::callee::AUDIO_ON_EVENT_SELECT_ENTER;
    pub(crate) use crate::game::main::ssxappdtor::callee::AUDIO_PLAY_FE_SOUND;
    pub(crate) use crate::game::main::ssxappdtor::callee::BUY_ATTRIB_COMMIT_PURCHASE;
    pub(crate) use crate::game::main::ssxappdtor::callee::CAREER_STATS_SELECT_PAGE;
    pub(crate) use crate::game::main::ssxappdtor::callee::DELETE;
    pub(crate) use crate::game::main::ssxappdtor::callee::EVENT_SELECT_SET_MODE;
    pub(crate) use crate::game::main::ssxappdtor::callee::EVENT_SELECT_SHUTDOWN;
    pub(crate) use crate::game::main::ssxappdtor::callee::FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::main::ssxappdtor::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::main::ssxappdtor::callee::NET_FE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::main::ssxappdtor::callee::OV_STATE_OVERLAY_SHUTDOWN;
    pub(crate) use crate::game::main::ssxappdtor::callee::REWARD_LIST_SHOW_PAGE;
    pub(crate) use crate::game::main::ssxappdtor::callee::SHARED_LOGIN_SET_ACCOUNT_NAME;
    pub(crate) use crate::game::main::ssxappdtor::callee::SHARED_LOGIN_SET_ACCOUNT_PASSWORD;
    pub(crate) use crate::game::main::ssxappdtor::callee::UI_ENGINE_UPDATE;
    pub(crate) use crate::game::main::ssxappdtor::callee::UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AUDIO_SYSTEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUILD_BUDDY_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHANNEL_GET_TRACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FIND_FREE_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_RACE_MAN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_SELECTED_USER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GOTO_NEXT_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::HANDLE_NET_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LEVEL_TO_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LIST_GET_ENTRY_TYPE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LOAD_BANK_ASYNC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_BASE_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_ON_CHILD_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ONLINE_CHARACTER_SELECT_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ON_CHILD_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_END_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SET_CONTROLLER_INDEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REQUEST_IMAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_HAS_RENDER_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_SAY_20B9;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SEND_PACKET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SET_SELECTED_USER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SHOW_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD as SND_BANK_UNLOAD_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::START_RIDER_BOARD_LOOPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_BOARD_LOOPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_STATE_CTOR_2_2 as UI_STATE_CTOR_2;

    // Same-shard direct calls (`jal`, emitted by the generator as the shard's
    // readable alias).
    pub const FE_LOCALE_UNLOAD_TABLE: Callee = Callee {
        addr: 0x0019_5a50,
        f: ee::s02::FELocale_UnloadTable,
    };

    pub const FE_MODULE_RIDER_PREVIEW_CTOR: Callee = Callee {
        addr: 0x0019_e3d0,
        f: ee::s03::FEModule_RiderPreviewCtor,
    };

    pub const FE_ASYNC_MANAGER_GET_STATE: Callee = Callee {
        addr: 0x001a_39c0,
        f: ee::s03::FEAsyncManager_GetState,
    };

    pub const NET_FE_STATE_GET_POPUP: Callee = Callee {
        addr: 0x001a_9c50,
        f: ee::s03::NetFEState_GetPopup,
    };

    pub const NET_FE_STATE_SETUP_BUDDY_ACTION_POPUP: Callee = Callee {
        addr: 0x001a_b478,
        f: ee::s03::NetFEState_SetupBuddyActionPopup,
    };

    pub const CFE_STATE_CREATE_STEP1_ON_SAVE_ACCOUNT_ANSWER: Callee = Callee {
        addr: 0x001b_91c8,
        f: ee::s03::f_001b91c8,
    };

    pub const CFE_STATE_CREATE_STEP1_ON_EDIT_DONE_POPUP_EVENT: Callee = Callee {
        addr: 0x001b_92c8,
        f: ee::s03::f_001b92c8,
    };

    pub const CFE_STATE_LOBBY_MENU_ON_CREATE_ROOM_POPUP_EVENT: Callee = Callee {
        addr: 0x001b_dcf8,
        f: ee::s03::f_001bdcf8,
    };

    pub const CFE_STATE_LOBBY_MENU_PROMPT_ROOM_PASSWORD: Callee = Callee {
        addr: 0x001b_dd60,
        f: ee::s03::f_001bdd60,
    };

    pub const CFE_STATE_ONLINE_MAIN_MENU_ON_LEAVE_POPUP_EVENT: Callee = Callee {
        addr: 0x001c_0138,
        f: ee::s03::f_001c0138,
    };

    pub const CFE_STATE_ONLINE_MAIN_MENU_ON_DISCONNECT_POPUP_EVENT: Callee = Callee {
        addr: 0x001c_0240,
        f: ee::s03::f_001c0240,
    };

    pub const CFE_STATE_NET_QUICK_MESSAGES_ON_SAVE_POPUP_EVENT: Callee = Callee {
        addr: 0x001c_2c50,
        f: ee::s03::f_001c2c50,
    };

    pub const CFE_STATE_NET_QUICK_MESSAGES_ON_RESET_POPUP_EVENT: Callee = Callee {
        addr: 0x001c_2ba8,
        f: ee::s03::f_001c2ba8,
    };

    pub const CFE_STATE_REWARD_GALLERY_V29: Callee = Callee {
        addr: 0x001d_fac8,
        f: ee::s04::cFEStateRewardGallery_v29,
    };

    pub const CFE_STATE_REWARD_GALLERY_RELEASE_TEXTURES: Callee = Callee {
        addr: 0x001d_00c0,
        f: ee::s04::cFEStateRewardGallery_ReleaseTextures,
    };

    pub const MESSAGE_CENTER_GET_INBOX_3: Callee = Callee {
        addr: 0x001e_39f8,
        f: ee::s04::MessageCenter_GetInbox,
    };

    pub const HUD_LAYOUT_ICON_TEXT: Callee = Callee {
        addr: 0x001e_92a8,
        f: ee::s04::HUD_LayoutIconText,
    };

    pub const GAME_MODULE_CTOR: Callee = Callee {
        addr: 0x0022_e968,
        f: ee::s05::GameModule_ctor,
    };

    pub const MC_MANAGER_END_PROGRESS: Callee = Callee {
        addr: 0x0024_1180,
        f: ee::s06::f_00241180,
    };

    pub const MC_MANAGER_SET_MODE: Callee = Callee {
        addr: 0x0023_d570,
        f: ee::s06::MCManager_SetMode,
    };

    pub const MC_MANAGER_END_OPERATION_2: Callee = Callee {
        addr: 0x0023_ea30,
        f: ee::s06::MCManager_EndOperation,
    };

    pub const MC_MANAGER_REQUEST_CARD_INFO_2: Callee = Callee {
        addr: 0x0023_fb20,
        f: ee::s06::MCManager_RequestCardInfo,
    };

    pub const LOBBY_MAN_GET_ROOM_NAME: Callee = Callee {
        addr: 0x0025_f150,
        f: ee::s06::LobbyMan_GetRoomName,
    };

    pub const LOBBY_MAN_MOVE_TO_ROOM: Callee = Callee {
        addr: 0x0025_d860,
        f: ee::s06::LobbyMan_MoveToRoom,
    };

    pub const LOBBY_MAN_LOOKUP_PEER_ADDR: Callee = Callee {
        addr: 0x0025_a730,
        f: ee::s06::LobbyMan_LookupPeerAddr,
    };
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BANK_MONITOR_ADD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE_9;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CLOSE_UNPINNED_POPUPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::FE_NET_SEND;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::HUD_SHOW_NUMBER_DELAYED;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MC_STATUS_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEMSET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEM_MAN_ALLOC;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_POP_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_PUSH_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::POSITION_MODIFIER_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW_BOLTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPATIAL_INSERT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPRINTF;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCASECMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRLEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STR_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_TEARDOWN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_OBJECT_BY_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_SCREEN_START_THREAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_PAINTER_2D;
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

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {
    pub(crate) use crate::game::fe::feasyncfile::table::ASYNC_FILE_GET_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::table::AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_EVENT_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_GAME_MODE;
    pub(crate) use crate::game::fe::feasyncfile::table::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE2;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_CALC;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_TREE_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_GET_UNPACKED_SIZE;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_UNPACK;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::table::OPERATOR_NEW_TAG;
    pub(crate) use crate::game::fe::feasyncfile::table::PATH_STOP_TRACKS;
    pub(crate) use crate::game::fe::feasyncfile::table::PS2_MEM_CARD_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_BEGIN_PLAYBACK;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_END_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::table::RIDER_MODEL_FIND;
    pub(crate) use crate::game::fe::feasyncfile::table::SCRIPT_ENGINE_GET_RESULT_VTABLE;
    pub(crate) use crate::game::fe::feasyncfile::table::SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::table::STR_CASECMP;
    pub(crate) use crate::game::fe::feasyncfile::table::STR_HASH;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_MENU_RESET_SCROLL;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_START;
}

// ---------------------------------------------------------------------------
// Message center.

/// `bool MessageCenter_IsMessageOfType(uint type, int index)` (0x1e2d58):
/// follows the two lookup tables at 0x4c6c08 (24-byte records, word +4) and
/// the 20-byte records at 0x441630 (word +0xc), and reports whether the
/// resulting type equals `type`.
pub fn message_center_is_message_of_type(c: &mut Cpu) {
    c.cycles += 15;
    c.g[V0][0] = 0x18;
    c.g[A2][0] = 0x4c0000;
    let first = abi::mult(c, abi::r32(c, A1), 0x18);
    c.g[A1][0] = sx(first);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x6c08));
    c.g[A3][0] = 0x14;
    c.g[V0][0] = sx(0x0044_0000u32.wrapping_add(0x1630));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = Addr(c.g[A1][0] as u32).off(4).lw(c);
    let second = abi::mult(c, abi::r32(c, V1), abi::r32(c, A3));
    c.g[V1][0] = sx(second);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
    c.g[V0][0] ^= c.g[A0][0];
    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// FE state transition hooks: `if (on) refresh(this); return 0;`.

/// The shared shape of the four `TransitionIn` hooks below.
pub(crate) fn transition_in(c: &mut Cpu, ret: u32, refresh: Callee) -> Flow {
    c.cycles += 3;
    let frame = Frame::open(c, 0x10);
    let on = c.g[A1][0] != 0;
    frame.save_ra(c, 0);
    if on {
        c.cycles += 2;
        refresh.call(c, ret)?;
    }
    c.cycles += 4;
    frame.restore_ra(c, 0);
    c.g[V0][0] = 0;
    frame.close(c);
    Ok(())
}

/// `bool cOVState_ViewPopup_TransitionIn(cOVState_ViewPopup* s, int on)`
/// (0x1e52d0).
pub fn ov_state_view_popup_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        transition_in(c, 0x001e_52e4, callee::VIEW_POPUP_SHOW_MESSAGE)
    })
}

// ---------------------------------------------------------------------------
// FELocale_SetFlag27.

/// `void FELocale_SetFlag27(FELocale* l, int on)` (0x1e1090): set bit 27 of
/// the locale's flags word (`0xf7ffffff` masked, the flag shifted into place).
pub fn fe_locale_set_flag27(c: &mut Cpu) {
    c.cycles += 9;
    let this = r32(c, A0);
    c.g[V0][0] = c.lw(this);
    c.g[V1][0] = 0xffff_ffff_f7ff_0000;
    c.g[A1][0] = (0u64 < c.g[A1][0]) as u64;
    c.g[V1][0] = c.g[V1][0] | 0xffff;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 27);
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
    Addr(this).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `cFEStateNetMessenger_TransitionIn(this)` 0x1e0fc0: same shape as the
/// challenge popup's transition-in (vtable at `*(this + 8)`, slot +0x100).
pub fn c_fe_state_net_messenger_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let vt = lw_at(c, this as u64, 8);
        c.g[V0][0] = vt;
        let m = VSlot::read(c, vt, 0x100);
        c.g[V1][0] = m.delta;
        c.g[A1][0] = m.func;
        c.g[A0][0] = sx(this.wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, 0x001e_0fdc)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Message center.

/// `int MessageCenter_GetMessageId(MessageCenter* mc, int index)` (0x1e3620):
/// the id field (`+0`) of the `index`th entry of the message center's inbox
/// (entries are 8 bytes wide).
pub fn message_center_get_message_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::MESSAGE_CENTER_GET_INBOX, 0x001e_3634)?;
        let index = r32(c, S0) << 3;
        c.g[S0][0] = sx(index);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
        c.g[V0][0] = c.lw(r32(c, V0));
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int MessageCenter_GetMessageVariant(MessageCenter* mc, int index)`
/// (0x1e3650): the variant byte (`+4`) of the `index`th inbox entry.
pub fn message_center_get_message_variant(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::MESSAGE_CENTER_GET_INBOX, 0x001e_3664)?;
        let index = r32(c, S0) << 3;
        c.g[S0][0] = sx(index);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
        c.g[V0][0] = c.lbu(p(r32(c, V0), 4));
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Screen activation / shutdown hooks.

/// `void OVScreenPeakChallengeResults_OnActivate(self)` (0x1e8ed0): activate
/// the base screen state, then play the peak-challenge screen's front-end
/// sound (id 0xe) on the audio system.
pub fn ov_screen_peak_challenge_results_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, callee::UI_STATE_ON_ACTIVATE, 0x001e_8ee0)?;
        call_ptr(c, callee::AUDIO_GET_INSTANCE, 0x001e_8ee8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xe;
        call_ptr(c, callee::AUDIO_PLAY_FE_SOUND, 0x001e_8ef4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RuleSelectMember` destructor (vtable at `+0x30`).
pub fn rule_select_member_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_b078, 0x30, 0x001d_f464);
}

/// `FEUISound` destructor (vtable at `+0`).
pub fn fe_ui_sound_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_dd60, 0, 0x001e_0764);
}

/// `OVUISound` destructor (vtable at `+0`).
pub fn ov_ui_sound_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_dd60, 0, 0x001e_07a4);
}

/// `UISoundBase` destructor (vtable at `+0`).
pub fn ui_sound_base_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_dd60, 0, 0x001e_106c);
}

/// `UICustomBase` destructor (vtable at `+4`).
pub fn ui_custom_base_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_dd28, 4, 0x001e_10dc);
}

/// `UIInputBase` destructor (vtable at `+8`).
pub fn ui_input_base_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_dba8, 8, 0x001e_112c);
}

/// `void OVScreenMessageCenter_HideWidget(OVScreenMessageCenter* self,
/// Widget* w)` (0x1e4840).
pub fn ov_message_center_hide_widget(c: &mut Cpu) {
    hide_show_widget(c, 0, 0x001e_4868);
}

/// `void OVScreenMessageCenter_ShowWidget(OVScreenMessageCenter* self,
/// Widget* w)` (0x1e4878).
pub fn ov_message_center_show_widget(c: &mut Cpu) {
    hide_show_widget(c, 1, 0x001e_48a0);
}

// ---------------------------------------------------------------------------
// Message center and menu screens.

/// `bool MessageCenter_IsFlagSet(self, flag /*a1*/, key /*a2*/)`
/// (0x1e3a30): true when the BE message flag `key` is set for rider slot 1.
pub fn message_center_is_flag_set(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x001e_3a44)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x001e_3a50)?;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, callee::BENEW_PLAYER_INT_IS_MESSAGE_FLAG_SET, 0x001e_3a60)?;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.restore_ra(c, 0);
        leave(c, 0x20);
        Ok(())
    });
}

/// `void OVScreenMessageCenter_OnChildEvent(self, sender, event, arg)`
/// (0x1e4610): on child event 0x14 the message center rebuilds its list and
/// its counter and scrolls the list (`self+0x10` is the list widget).
pub fn ov_screen_message_center_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x14;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let other = c.g[A2][0] != 0x14;
        c.g[S0][0] = c.g[A0][0];
        let self_ = abi::r32(c, S0);
        if other {
            c.cycles += 4;
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 8;
        callee::OV_MESSAGE_CENTER_REFRESH_LIST.call(c, 0x001e_4630)?;
        c.g[A0][0] = c.g[S0][0];
        callee::OV_MESSAGE_CENTER_UPDATE_COUNT_TEXT.call(c, 0x001e_4638)?;
        c.g[A0][0] = Addr(self_.wrapping_add(0x10)).lw(c);
        c.g[A1][0] = 1;
        let list = (c.g[A0][0] as u32).wrapping_add(0x18);
        c.g[A0][0] = sx(list);
        call_ptr(c, 0x0039_f190, 0x001e_4648)?;
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Message center.

/// `void MessageCenter_SetFlag(int a, int b)` (0x1e3a78): get the BE
/// interface and post the message through its handler with the two arguments.
/// 20 cycles plus the callees.
pub fn message_center_set_flag(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        let a = c.g[A0][0];
        c.g[S0][0] = a;
        frame.save_ra(c, 0);
        let b = c.g[A1][0];
        c.g[S1][0] = b;
        call_ptr(c, callee::BE_GET_BE, 0x001e_3a94)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, callee::BE_GET_INTERFACE, 0x001e_3aa0)?;
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, callee::BE_INTERFACE_CALL, 0x001e_3ab4)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Overlay screens.

/// The shared body of the two `OnWidgetAction` handlers (0x1e8160 and
/// 0x1e8880): widget 5 with a live source and a module hides the current
/// widget and refreshes the world HUD. `ret0`/`ret1` are the return addresses
/// of the two calls.
pub(crate) fn widget_action_finish(c: &mut Cpu, ret0: u32, ret1: u32) -> Flow {
    c.cycles += 4;
    let frame = Frame::open(c, 0x20);
    frame.save(c, 16, 0x10);
    let source_null = c.g[A1][0] == 0;
    frame.save_ra(c, 0);
    let mut extra = source_null;
    if !source_null {
        c.cycles += 3;
        c.g[V0][0] = 5;
        if c.g[A2][0] == c.g[V0][0] {
            c.cycles += 3;
            let field = c.lw(p(r32(c, A0), 0xa0));
            c.g[V0][0] = field;
            let module = c.lw(gp(c, GAME_MODULE_GP));
            c.g[V0][0] = module;
            if field != 0 {
                c.cycles += 6;
                let world = c.lw(p(module as u32, 0x84));
                c.g[S0][0] = world;
                let widget = c.lw(p(world as u32, 0x48));
                c.g[A0][0] = sx((widget as u32).wrapping_add(0x18));
                call_ptr(c, callee::UI_WIDGET_SLOT_0, ret0)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::UI_WIDGET_SLOT_1, ret1)?;
                extra = true;
            }
        }
    }
    if extra {
        c.cycles += 1;
    }
    frame.restore(c, 16, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void OVScreenFinishOv_OnWidgetAction(s, void* source, int action)`
/// (0x1e8160).
pub fn ov_screen_finish_ov_on_widget_action(c: &mut Cpu) {
    guest(c, |c| widget_action_finish(c, 0x001e_8198, 0x001e_81a0))
}

/// `void OVScreenFinishOv2_OnWidgetAction(s, void* source, int action)`
/// (0x1e8880).
pub fn ov_screen_finish_ov2_on_widget_action(c: &mut Cpu) {
    guest(c, |c| widget_action_finish(c, 0x001e_88b8, 0x001e_88c0))
}

/// `void cFEStateTrophyRoom_dtor(self, flags)` (0x1e0210): release the trophy
/// images, install the rewards-room vtable and run the base dtor.
pub fn c_fe_state_trophy_room_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x47_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6948));
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x001e_0238);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        callee::TROPHY_ROOM_RELEASE_IMAGES.call(c, 0x001e_0238)?;
        c.g[V0][0] = 0x47_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2f30));
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001e_0250);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        call_ptr(c, callee::UI_STATE_DTOR, 0x001e_0250)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MessageCenter_GetUnreadCount(void)` (0x1e3408): count the inbox bits
/// that are not set.
pub fn message_center_get_unread_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MESSAGE_CENTER_GET_INBOX_2.call(c, 0x001e_3418)?;
        let inbox = r32(c, V0);
        c.g[A3][0] = Addr(inbox).off(0xcc).lbu(c);
        c.g[A2][0] = 0;
        c.g[A1][0] = 0;
        if c.g[A3][0] != 0 {
            c.cycles += 2;
            c.g[T0][0] = Addr(inbox).off(0xc8).lw(c);
            c.g[T1][0] = 1;
            loop {
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[T1][0] as u32) << ((c.g[A2][0] as u32) & 31));
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[V0][0] = c.g[T0][0] & c.g[V0][0];
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                c.g[V1][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = c.g[A0][0];
                }
                if c.g[V1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001e_3430);
                }
            }
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.g[A1][0];
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(p(c.g[SP][0] as u32, 0x10));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Small helpers.

/// The address `a + off` (32-bit guest address arithmetic).
#[inline(always)]
pub(crate) fn p_2(a: u64, off: u32) -> u32 {
    (a as u32).wrapping_add(off)
}

#[inline(always)]
pub(crate) fn lw_2(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lw(c)
}

#[inline(always)]
pub(crate) fn sw(c: &mut Cpu, a: u32, v: u64) {
    Addr(a).sw(c, v);
}

#[inline(always)]
pub(crate) fn lh(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lh(c)
}

#[inline(always)]
pub(crate) fn lb(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lb(c)
}

#[inline(always)]
pub(crate) fn lbu(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lbu(c)
}

/// `gp_2 + off` (globals).
#[inline(always)]
pub(crate) fn gp_2(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `cFEStateLobbyMenu_dtor` (0x1df7c8): destroy the two embedded strings and
/// then the net-FE state base.
pub fn cfe_state_lobby_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0047_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5808));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        sw(c, p_2(c.g[S0][0], 8), c.g[V0][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb80));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001d_f7f8)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x76c));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001d_f804)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001a_85d0, 0x001d_f810)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of the seven reward-gallery destructors: gallery vtable,
/// release the gallery's textures, base vtable, then the UI-list destructor.
macro_rules! reward_gallery_dtor {
    ($fname:ident, $doc:expr, $a:expr, $b:expr, $c:expr) => {
        #[doc = $doc]
        pub fn $fname(c: &mut Cpu) {
            guest(c, |c| {
                c.cycles += 23;
                let frame = Frame::open(c, 0x30);
                c.g[V0][0] = 0x0047_0000;
                frame.save(c, S0, 0x20);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5c38));
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S1, 0x10);
                frame.save_ra(c, 0);
                c.g[S1][0] = c.g[A1][0];
                sw(c, p_2(c.g[S0][0], 8), c.g[V0][0]);
                callee::CFE_STATE_REWARD_GALLERY_V29.call(c, $a)?;
                c.g[A0][0] = c.g[S0][0];
                callee::CFE_STATE_REWARD_GALLERY_RELEASE_TEXTURES.call(c, $b)?;
                c.g[V0][0] = 0x0047_0000;
                c.g[A0][0] = c.g[S0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2f30));
                c.g[A1][0] = c.g[S1][0];
                sw(c, p_2(c.g[S0][0], 8), c.g[V0][0]);
                call_ptr(c, 0x0039_e390, $c)?;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                Ok(())
            })
        }
    };
}
pub(crate) use reward_gallery_dtor;

reward_gallery_dtor!(
    cfe_state_reward_gallery_dtor,
    "`cFEStateRewardGallery_dtor` (0x1dfa10): destroy the reward-gallery screen.",
    0x001d_fa38,
    0x001d_fa40,
    0x001d_fa58
);

reward_gallery_dtor!(
    cfe_state_reward_gallery_posters_dtor,
    "`cFEStateRewardGalleryPosters_dtor` (0x1dfb28): destroy the posters screen.",
    0x001d_fb50,
    0x001d_fb58,
    0x001d_fb70
);

reward_gallery_dtor!(
    cfe_state_reward_gallery_trading_cards_dtor,
    "`cFEStateRewardGalleryTradingCards_dtor` (0x1dfbf8): destroy the trading-cards screen.",
    0x001d_fc20,
    0x001d_fc28,
    0x001d_fc40
);

reward_gallery_dtor!(
    cfe_state_reward_gallery_concept_arts_dtor,
    "`cFEStateRewardGalleryConceptArts_dtor` (0x1dfcc8): destroy the concept-arts screen.",
    0x001d_fcf0,
    0x001d_fcf8,
    0x001d_fd10
);

reward_gallery_dtor!(
    cfe_state_reward_gallery_videos_dtor,
    "`cFEStateRewardGalleryVideos_dtor` (0x1dfd98): destroy the videos screen.",
    0x001d_fdc0,
    0x001d_fdc8,
    0x001d_fde0
);

reward_gallery_dtor!(
    cfe_state_reward_gallery_toys_dtor,
    "`cFEStateRewardGalleryToys_dtor` (0x1dfe68): destroy the toys screen.",
    0x001d_fe90,
    0x001d_fe98,
    0x001d_feb0
);

reward_gallery_dtor!(
    cfe_state_reward_gallery_cheat_chars_dtor,
    "`cFEStateRewardGalleryCheatChars_dtor` (0x1dff38): destroy the cheat-characters screen.",
    0x001d_ff60,
    0x001d_ff68,
    0x001d_ff80
);

/// `void cFEPopupSelectMultiplayerMode_Init(cFEPopupSelectMultiplayerMode* self)`
/// (0x1e02f8): name-hash the popup, add its screen to the UI engine and play
/// its first frame.
pub fn cfe_popup_select_multiplayer_mode_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(8));
        call_ptr(c, 0x0031_7670, 0x001e_0314)?;
        c.g[A0][0] = lw_2(c, p_2(c.g[S0][0], 0x10));
        c.g[A2][0] = c.g[V0][0];
        sw(c, p_2(c.g[S0][0], 0xc), c.g[V0][0]);
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001e_032c)?;
        let no_screen = c.g[V0][0] == 0;
        sw(c, p_2(c.g[S0][0], 0x40), c.g[V0][0]);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001e_0344)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateChallengePopup_dtor` (0x1e09a0): notify the owned object (vtable
/// slot 0x100), then destroy the net-FE state base.
pub fn cfe_state_challenge_popup_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0047_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x71c8));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        sw(c, p_2(c.g[S0][0], 8), c.g[V0][0]);
        c.g[A2][0] = lw_2(c, p_2(c.g[S0][0], 0x20));
        c.g[V0][0] = lw_2(c, p_2(c.g[A2][0], 8));
        c.g[A0][0] = lh(c, p_2(c.g[V0][0], 0x100));
        c.g[V1][0] = lw_2(c, p_2(c.g[V0][0], 0x104));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001e_09dc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001a_85d0, 0x001e_09e8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MessageCenter_FindMessage(int id)` (0x1e3388): index of the message
/// with that id in the inbox list, or -1.
pub fn message_center_find_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MESSAGE_CENTER_GET_INBOX_3.call(c, 0x001e_339c)?;
        c.g[A0][0] = lbu(c, p_2(c.g[V0][0], 0xcc));
        let empty = c.g[A0][0] == 0;
        c.g[V1][0] = 0;
        let mut found = false;
        if !empty {
            c.g[A1][0] = c.g[A0][0];
            c.g[A0][0] = c.g[V0][0];
            loop {
                c.cycles += 3;
                c.g[V0][0] = lw_2(c, p_2(c.g[A0][0], 0));
                if c.g[S0][0] == c.g[V0][0] {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V1][0];
                    found = true;
                    break;
                }
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(8));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001e_33b0);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MessageCenter_ResetNewPlayerFlag()` (0x1e3510): clear the "new
/// player" interface state, and if the global new-player flag is set, clear
/// it and enable the new-player interface's slot-3 flag.
pub fn message_center_reset_new_player_flag(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0014_dc80, 0x001e_3520)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x001e_352c)?;
        c.g[V1][0] = 0x0053_0000;
        c.g[V0][0] = lb(c, (c.g[V1][0] as u32).wrapping_add(0x5c11));
        let set = c.g[V0][0] != 0;
        if !set {
            c.cycles += 10;
            sw(c, gp_2(c, -0x1024), 0);
            call_ptr(c, 0x0014_dc80, 0x001e_3544)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x0014_dd58, 0x001e_3550)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 1;
            call_ptr(c, 0x0014_75c0, 0x001e_3560)?;
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The three `OVScreen*_OnWidgetNotify` handlers: the action key is xored
/// with 6; 0x101 only when it *was* 6, else 0.
pub(crate) fn ov_screen_notify_action_six(c: &mut Cpu) {
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
/// action 5 stores 1 in `gp_3-0x9f4` and the row value at `menu+0x18` in
/// `gp_3-0x9f0` (v0 = 1, v1 = the row value).
pub(crate) fn ov_screen_action_row(c: &mut Cpu) {
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
            Addr(gp_3(c, -0x9f4)).sw(c, one);
            Addr(gp_3(c, -0x9f0)).sw(c, row);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// The six reward-gallery `GetItemPrice` methods: the owned count at
/// `item+0xc` (a signed short) times ten credits — or the count itself when
/// it is zero or negative.
pub(crate) fn gallery_item_price(c: &mut Cpu) {
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
pub(crate) fn ov_screen_finish_on_event(c: &mut Cpu) {
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
pub(crate) const POPUP_ENTRIES: u32 = 0x004c_6c08;

/// The per-entry target table (`D_00441630`), stride 0x14.
pub(crate) const POPUP_TARGETS: u32 = 0x0044_1630;

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
// Message center.

/// `void MessageCenter_PostOnce3C(void)` (0x1e2518): post the "friends" hint
/// once, when the buddy list is quiet.
pub fn message_center_post_once_3c(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = lw_3(c, p_3(c.g[GP][0], -0x848i32 as u32));
        frame.save_ra(c, 0);
        c.g[V1][0] = lw_3(c, p_3(c.g[V0][0], 0x84));
        c.g[A0][0] = lw_3(c, p_3(c.g[V1][0], 0x28));
        c.g[A0][0] = lw_3(c, c.g[A0][0] as u32);
        let none = c.g[A0][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 1;
            c.g[A1][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
        }
        c.cycles += 2;
        let busy = c.g[A1][0] != 0;
        frame.restore_ra(c, 0);
        if !busy {
            c.cycles += 4;
            c.g[A0][0] = 0x3c;
            call_ptr(c, 0x001e_3a30, 0x001e_254c)?;
            let flagged = c.g[V0][0] != 0;
            frame.restore_ra(c, 0);
            if !flagged {
                c.cycles += 7;
                c.g[A0][0] = 0x3c;
                c.g[A1][0] = 1;
                call_ptr(c, 0x001e_3a78, 0x001e_2560)?;
                c.g[A0][0] = 0xff;
                c.g[A1][0] = 0;
                call_ptr(c, 0x001e_2fe0, 0x001e_256c)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void MessageCenter_PostOnce38(void)` (0x1e2578): post the "lodge" hint
/// once, when the buddy list is quiet.
pub fn message_center_post_once_38(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = lw_3(c, p_3(c.g[GP][0], -0x848i32 as u32));
        frame.save_ra(c, 0);
        c.g[V1][0] = lw_3(c, p_3(c.g[V0][0], 0x84));
        c.g[A0][0] = lw_3(c, p_3(c.g[V1][0], 0x28));
        c.g[A0][0] = lw_3(c, c.g[A0][0] as u32);
        let none = c.g[A0][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 1;
            c.g[A1][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
        }
        c.cycles += 2;
        let busy = c.g[A1][0] != 0;
        frame.restore_ra(c, 0);
        if !busy {
            c.cycles += 4;
            c.g[A0][0] = 0x38;
            call_ptr(c, 0x001e_3a30, 0x001e_25ac)?;
            let flagged = c.g[V0][0] != 0;
            frame.restore_ra(c, 0);
            if !flagged {
                c.cycles += 7;
                c.g[A0][0] = 0x38;
                c.g[A1][0] = 1;
                call_ptr(c, 0x001e_3a78, 0x001e_25c0)?;
                c.g[A0][0] = 0xfb;
                c.g[A1][0] = 0;
                call_ptr(c, 0x001e_2fe0, 0x001e_25cc)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void MessageCenter_RemoveMessageById(int id)` (0x1e3268): find the inbox
/// entry with `id` and remove it. A `match pc` dispatcher: the loop head
/// 0x1e3290 is a resume entry.
pub fn message_center_remove_message_by_id(c: &mut Cpu) {
    guest(c, |c| {
        let frame;
        let mut increment;
        if c.pc == 0x001e_3268 {
            c.cycles += 9;
            frame = Frame::open(c, 0x20);
            frame.save(c, S0, 0x10);
            frame.save_ra(c, 0);
            c.g[RA][0] = sx(0x001e_327c);
            c.g[S0][0] = c.g[A0][0];
            call_ptr(c, 0x001e_39f8, 0x001e_327c)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = 0;
            c.g[V1][0] = c.g[A1][0];
            increment = false;
        } else if c.pc == 0x001e_3290 {
            frame = Frame { size: 0x20 };
            increment = true;
        } else {
            let pc = c.pc;
            c.jump_out(pc, 0x001e_3268, 0x001e_32c8);
            return Ok(());
        }
        loop {
            if increment {
                c.cycles += 1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            }
            increment = true;
            c.cycles += 4;
            c.g[V0][0] = Addr(p_3(c.g[A1][0], 0xcc)).lbu(c);
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = lw_3(c, c.g[V1][0] as u32);
            let differs = c.g[S0][0] != c.g[V0][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(8));
            if differs {
                if back_edge_unwinds(c, 0x001e_3290) {
                    return Ok(());
                }
                continue;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x001e_32b8);
            call_ptr(c, 0x001e_31b8, 0x001e_32b8)?;
            frame.restore(c, S0, 0x10);
            break;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cOVState_ViewPopup_Init(void* self)` (0x1e5248).
pub fn cov_state_view_popup_init(c: &mut Cpu) {
    guest(c, |c| ui_init_screen(c, 0x001e_5248, 0x0046_e258))
}

/// `void cOVState_ViewPopup_SetSubject(void* self, int idx)` (0x1e55d0): either
/// create the variant subject object or set a text's Unicode string id from
/// the table entry.
pub fn cov_state_view_popup_set_subject(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[A3][0] = c.g[A1][0];
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[A1][0] = 0x18;
        c.g[V0][0] = 0x004c_0000;
        c.mult(0, 5, 7, 5);
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6c08));
        c.g[A2][0] = c.g[A0][0];
        c.g[A0][0] = 0x1;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[V1][0] = Addr(p(c.g[A1][0] as u32, 8)).lw(c);
        if c.g[V1][0] == c.g[A0][0] {
            c.cycles += 5;
            c.g[A0][0] = c.g[A2][0];
            c.g[A1][0] = c.g[A3][0];
            call_ptr(c, 0x001e_5638, 0x001e_5610)?;
        } else {
            c.g[A0][0] = Addr(p(c.g[A2][0] as u32, 0x54)).lw(c);
            c.cycles += 2;
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[A1][0] = Addr(p(c.g[A1][0] as u32, 0x10)).lw(c);
                call_ptr(c, 0x003a_1000, 0x001e_5628)?;
            }
        }
        c.cycles += 2;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end destructors, modifiers and screen initialisation.

/// `cFEStateChallengeReceivedPopup_dtor(this, flags)` 0x1e0a50: restore the
/// base vtable, destroy the received-challenge popup's `+0x20` object, release
/// its string and run the base destructor.
pub fn c_fe_state_challenge_received_popup_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        c.g[V0][0] = sx(0x0046_8d20);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = this64;
        frame.save_ra(c, 0);
        c.g[S1][0] = abi::arg64(c, 1);
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        let obj = u32_at(c, this, 0x20);
        c.g[A2][0] = sx(obj);
        let vt = u32_at(c, obj, 8);
        c.g[V0][0] = sx(vt);
        c.g[A0][0] = Addr(p(vt, 0x100)).lh(c);
        c.g[V1][0] = lw_at_2(c, vt, 0x104);
        let func = r32(c, V1);
        c.g[A0][0] = sx(obj.wrapping_add(r32(c, A0)));
        call_ptr(c, func, 0x001e_0a8c)?;
        c.g[A0][0] = sx(p(this, 0x6d0));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001e_0a98)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001a_85d0, 0x001e_0aa4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetCreateRoomPopup_dtor(this, flags)` 0x1df878: restore the base
/// vtable, release the three strings and run the base destructor.
pub fn c_fe_state_net_create_room_popup_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        c.g[V0][0] = sx(0x0046_a6e8);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = this64;
        frame.save_ra(c, 0);
        c.g[S1][0] = abi::arg64(c, 1);
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        for (off, ret) in [
            (0x6e4u32, 0x001d_f8a8u32),
            (0x6e0, 0x001d_f8b4),
            (0x6dc, 0x001d_f8c0),
        ] {
            c.g[A0][0] = sx(p(this, off));
            c.g[A1][0] = 2;
            call_ptr(c, 0x0031_8260, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001a_85d0, 0x001d_f8cc)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateCreateStep1_dtor(this, flags)` 0x1e0cf8: restore the base vtable,
/// release the three account strings and run the base destructor.
pub fn c_fe_state_create_step1_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        c.g[V0][0] = sx(0x0046_86c0);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = this64;
        frame.save_ra(c, 0);
        c.g[S1][0] = abi::arg64(c, 1);
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx(p(this, 0x6dc));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001e_0d28)?;
        c.g[A0][0] = sx(p(this, 0x6d8));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001e_0d34)?;
        c.g[A0][0] = sx(p(this, 0x6d4));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001e_0d40)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001a_85d0, 0x001e_0d4c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MessageCenter_OnBCFinish(MessageCenter* self, int key, void* a2)`
/// (0x1e25d8): when no broadcast is running, dispatch the finish message by
/// key (4/5).
pub fn message_center_on_bc_finish(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(gpr(c, -0x848)).lw(c);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(at(c, V0, 0x84)).lw(c);
        c.g[V0][0] = Addr(at(c, V1, 0x28)).lw(c);
        c.g[V0][0] = Addr(at(c, V0, 0)).lw(c);
        let no_bc = c.g[V0][0] == 0;
        c.g[A3][0] = 0;
        if !no_bc {
            c.cycles += 1;
            c.g[A3][0] = ((c.g[V0][0] as i64) < 10) as u64;
        }
        c.cycles += 2;
        let busy = c.g[A3][0] != 0;
        frame.restore_ra(c, 0);
        if !busy {
            let mut key4 = false;
            c.cycles += 3;
            c.g[V0][0] = 4;
            key4 = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 5;
            if !key4 {
                c.cycles += 2;
                let key5 = c.g[A0][0] == c.g[V0][0];
                c.g[A0][0] = c.g[A1][0];
                if key5 {
                    c.cycles += 3;
                    c.g[A1][0] = c.g[A2][0];
                    call_ptr(c, 0x001e_2828, 0x001e_263c)?;
                    frame.restore_ra(c, 0);
                } else {
                    c.cycles += 2;
                }
            }
            if key4 {
                c.cycles += 5;
                c.g[A0][0] = c.g[A1][0];
                c.g[A1][0] = c.g[A2][0];
                call_ptr(c, 0x001e_2648, 0x001e_262c)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `cOVState_ViewPopup_ctor` (0x1E51D0): the base constructor of the
/// full-screen overlay popup state; runs the state constructor and clears the
/// popup's id/box/flag fields.
pub fn cov_state_view_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[T0][0];
        callee::UI_STATE_CTOR_2.call(c, 0x001e_51f4)?;
        c.g[V1][0] = 0x47_0000;
        sw_at(c, S0, 0x68, c.g[S1][0]);
        c.g[V1][0] = low(c, V1).wrapping_add(0x4248);
        sw_at(c, S0, 0x6c, c.g[S2][0]);
        sw_at(c, S0, 0x8, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        for off in [0x48u32, 0x4c, 0x50, 0x54, 0x5c, 0x60, 0x64, 0x58] {
            sw_at(c, S0, off, 0);
        }
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenFreestyleStandings_HandleInput` (0x1E65B8): if the scoreboard's
/// freestyle row is selected (byte +0x95 == 2), feed the pressed controller
/// index to the replay's controller setting. Returns 0.
pub fn ov_screen_freestyle_standings_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x47_0000;
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = low(c, A0).wrapping_sub(0x1af0);
        call_ptr(c, addr::STR_HASH, 0x001e_65dc)?;
        c.g[A0][0] = lw_at_3(c, S0, 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, addr::UI_OBJECT_BY_HASH, 0x001e_65e8)?;
        let found = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x20);
        let mut skip_tail = false;
        if found {
            c.cycles += 4;
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x95)).lbu(c);
            c.g[V0][0] = 2;
            let not_selected = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 0;
            if not_selected {
                skip_tail = true;
            } else {
                c.cycles += 6;
                c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
                c.g[A1][0] = Addr(r32(c, S1).wrapping_add(0x4)).lbu(c);
                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
                c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x28)).lw(c);
                callee::REPLAY_SET_CONTROLLER_INDEX.call(c, 0x001e_6614)?;
                frame.restore(c, S0, 0x20);
            }
        }
        if !skip_tail {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenFinalStandings_OnWidgetCreated` (0x1E7558): when the created
/// widget is the scoreboard's menu, set its selected row when the "final"
/// flag is on.
pub fn ov_screen_final_standings_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x47_0000;
        frame.save(c, S1, 0x10);
        c.g[A0][0] = low(c, A0).wrapping_sub(0x1af0);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = lw_at_3(c, S1, 0x38);
        call_ptr(c, addr::STR_HASH, 0x001e_757c)?;
        let not_menu = c.g[S0][0] != c.g[V0][0];
        frame.restore(c, S0, 0x20);
        if !not_menu {
            c.cycles += 10;
            call_ptr(c, addr::BE_GET_BE, 0x001e_758c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, addr::BE_GET_INTERFACE, 0x001e_7598)?;
            c.g[V1][0] = 0x53_0000;
            c.g[A0][0] = 1;
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x5c11)).lb(c);
            let not_final = c.g[V0][0] != c.g[A0][0];
            frame.restore(c, S0, 0x20);
            if !not_final {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 1;
                callee::UI_MENU_SET_SELECTION.call(c, 0x001e_75b8)?;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_001E03F8` (0x1e03f8): `cFEPopupSelectMultiplayerMode::OnWidgetAction`.
/// Action 5 selects: `this->+0x48 = 1`, the "multiplayer selected" global
/// (`gp-0x171c`) becomes `widget->+0x18 > 0`, and the dialog at
/// `this->+0x10 + 0x18` is told (`0x39f190(dialog, 1)`). Action 6 clears
/// `this->+0x48` and tells the dialog with 0. Anything else returns.
pub fn c_fe_popup_select_multiplayer_mode_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[SP][0] = sx(low_2(c, SP).wrapping_sub(0x10));
        c.g[V1][0] = c.g[A0][0];
        c.sd(low_2(c, SP), c.g[RA][0]);
        c.g[V0][0] = 0x5;
        let five = c.g[A2][0] == c.g[V0][0];
        c.g[A3][0] = c.g[A1][0];
        if !five {
            c.cycles += 3;
            c.g[V0][0] = 0x6;
            let six = c.g[A2][0] == c.g[V0][0];
            c.g[RA][0] = c.ld(low_2(c, SP));
            if six {
                c.cycles += 6;
                c.g[A0][0] = lw_r(c, V1, 0x10);
                c.g[A1][0] = 0x1;
                sb_r(c, V1, 0x48, 0);
                c.g[RA][0] = sx(0x001e_0464);
                c.g[A0][0] = sx(low_2(c, A0).wrapping_add(0x18));
                call_ptr(c, 0x0039_f190, 0x001e_0464)?;
                c.g[RA][0] = c.ld(low_2(c, SP));
            } else {
                c.cycles += 2;
            }
        } else {
            c.cycles += 11;
            c.g[V0][0] = 0x1;
            c.g[A0][0] = lw_r(c, V1, 0x10);
            sb_r(c, V1, 0x48, c.g[V0][0]);
            c.g[A1][0] = 0x1;
            c.g[A0][0] = sx(low_2(c, A0).wrapping_add(0x18));
            c.g[V0][0] = lw_r(c, A3, 0x18);
            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
            c.g[RA][0] = sx(0x001e_0448);
            sw_r(c, GP, 0u32.wrapping_sub(0x171c), c.g[V0][0]);
            call_ptr(c, 0x0039_f190, 0x001e_0448)?;
            c.g[RA][0] = c.ld(low_2(c, SP));
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(low_2(c, SP).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `bool MessageCenter_IsNewPlayerFlagClear(void)` (0x001e3570): false when
/// the new-player marker (`0x535bc8+0x49`) is set or the save type is 6/9;
/// otherwise the slot-0 new-player flag is clear.
pub fn message_center_is_new_player_flag_clear(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
        c.sd(sp(c), c.g[RA][0]);
        call_ptr(c, 0x0014_dc80, 0x001e_3580)?; // BELibrary_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x001e_358c)?; // BELibrary_GetInterface
        c.g[V1][0] = sx(0x0053_5bc8);
        c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
        let marked = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !marked {
            c.cycles += 4;
            c.g[V1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x4a));
            c.g[V0][0] = 6;
            let six = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 9;
            if six {
                c.cycles += 2;
                c.g[V0][0] = 0;
            } else if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 2;
                c.cycles += 9;
                call_ptr(c, 0x0014_dc80, 0x001e_35c8)?; // BELibrary_GetInstance
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                call_ptr(c, 0x0014_dd58, 0x001e_35d4)?; // BELibrary_GetInterface
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x0014_7580, 0x001e_35e0)?; // BENewPlayerInt_GetSlotFlag3
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
            } else {
                c.cycles += 2;
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        abi::ret(c);
        Ok(())
    })
}

/// `void OVScreenMessageCenter_OpenViewPopup(OVScreenMessageCenter* s,
/// void* a, void* b, void* c)` (0x001e50c8): allocate a 0x70-byte view popup,
/// construct it in place and push it on the state manager's popup stack.
pub fn ov_screen_message_center_open_view_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.g[A3][0] = 0;
        c.sq(S2, sp(c).wrapping_add(0x10));
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.g[S2][0] = c.g[A1][0];
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = sx(0x0046_e240);
        c.sd(sp(c), c.g[RA][0]);
        c.g[A0][0] = 0x70;
        c.g[A2][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x001e_5100)?; // EA_MemAllocNamed
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[T0][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, 0x001e_51d0, 0x001e_5118)?; // cOVState_ViewPopup_ctor
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x17c));
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0039_f290, 0x001e_5130)?; // UIStateMgr_PushPopup
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.lq(S2, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

/// `void cOVState_ViewPopup_SetBody(cOVState_ViewPopup* s, ViewPopupData* d)`
/// (0x001e5780): when the body index is >= 2, flag the text as dirty, set its
/// id and refresh the object; otherwise leave the view unchanged.
pub fn c_ov_state_view_popup_set_body(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[V1][0] = 0x18;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.mult(0, 5, 5, 3);
        c.g[V0][0] = sx(0x004c_6c08);
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sd(sp(c), c.g[RA][0]);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(8));
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        let ge2 = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if ge2 {
            c.cycles += 1;
            c.lq(S0, sp(c).wrapping_add(0x10));
        } else {
            c.cycles += 2;
            if (c.g[V1][0] as i64) < 0 {
                c.lq(S0, sp(c).wrapping_add(0x10));
            } else {
                c.cycles += 14;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                c.g[V0][0] = c.g[V0][0] | 0x80;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                call_ptr(c, 0x003a_1000, 0x001e_57d4)?; // UIText_SetTextId
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                c.g[A1][0] = 1;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(8));
                let vt = c.g[V0][0];
                let obj = c.g[A2][0];
                vt_call(c, vt, 0x48, obj, V1, 0x001e_57f0)?;
                c.cycles += 1;
                c.lq(S0, sp(c).wrapping_add(0x10));
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

/// `void OVScreenMessageCenter_DeleteMessage(self, index)` (0x1e5148):
/// remove the message, fix up the group's seen counter and refresh the list,
/// count and scroll arrows.
pub fn ovscreenmessagecenter_delete_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 18, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        call_ptr(c, callee::MESSAGE_CENTER_REMOVE_MESSAGE.addr, 0x001e_5168)?;
        c.g[V0][0] = lw_at_4(c, p(r32(c, S2), 0x144));
        c.g[A1][0] = 1;
        c.g[A0][0] = 4;
        c.g[S1][0] = Addr(p(c.g[V0][0] as u32, 0x95)).lbu(c);
        call_ptr(
            c,
            callee::MESSAGE_CENTER_COUNT_GROUP_MESSAGES.addr,
            0x001e_517c,
        )?;
        c.g[S0][0] = c.g[V0][0];
        call_ptr(
            c,
            callee::MESSAGE_CENTER_GET_MESSAGE_COUNT.addr,
            0x001e_5184,
        )?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = lw_at_4(c, p(r32(c, S2), 0x144));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
        c.g[S0][0] = ((c.g[S1][0] as i64) < (c.g[S0][0] as i64)) as u64;
        if c.g[S0][0] == 0 {
            c.g[S1][0] = c.g[V0][0];
        }
        c.g[A0][0] = c.g[S2][0];
        c.sb(p(c.g[V1][0] as u32, 0x95), c.g[S1][0]);
        call_ptr(
            c,
            callee::OV_SCREEN_MESSAGE_CENTER_REFRESH_LIST.addr,
            0x001e_51a4,
        )?;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(
            c,
            callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_COUNT_TEXT.addr,
            0x001e_51ac,
        )?;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(
            c,
            callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_SCROLL_ARROWS.addr,
            0x001e_51b4,
        )?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEPopupSelectMultiplayerMode_OnWidgetNotify(Popup* self, void* menu,
/// int key, int idx)` (0x1e0470): move the menu selection by `idx` for the
/// arrow keys; always returns 0x101.
pub fn c_fe_popup_select_multiplayer_mode_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x3;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        let down = c.g[A2][0] == c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        let mut skip_e4 = false;
        if down {
            select_next(c)?;
            c.cycles += 1;
        } else if c.g[A2][0] >= 0x4 {
            c.cycles += 3;
            if c.g[A2][0] < 0x8 {
                c.cycles += 3;
                c.cycles += 1;
            } else {
                c.cycles += 3;
                c.cycles += 2;
            }
        } else {
            c.cycles += 3;
            c.cycles += 2;
            c.g[V0][0] = 0x101;
            if c.g[A2][0] == 0x2 {
                select_prev(c)?;
                select_next(c)?;
                c.cycles += 1;
            } else {
                c.cycles += 2;
                skip_e4 = true;
            }
        }
        if !skip_e4 {
            c.cycles += 1;
        }
        c.g[V0][0] = 0x101;
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The `setSelectedByIndex(menu, idx - 1)` block of
/// [`c_fe_popup_select_multiplayer_mode_on_widget_notify`].
pub(crate) fn select_prev(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A1][0] = sx(r32(c, S0).wrapping_sub(1));
    c.g[A0][0] = c.g[S1][0];
    c.g[RA][0] = sx(0x001e_04d0);
    c.g[A1][0] = c.g[A1][0] & 0xff;
    call_ptr(c, 0x0039_b960, 0x001e_04d0)
}

/// The `setSelectedByIndex(menu, idx + 1)` block of
/// [`c_fe_popup_select_multiplayer_mode_on_widget_notify`].
pub(crate) fn select_next(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A1][0] = sx(r32(c, S0).wrapping_add(1));
    c.g[A0][0] = c.g[S1][0];
    c.g[RA][0] = sx(0x001e_04e0);
    c.g[A1][0] = c.g[A1][0] & 0xff;
    call_ptr(c, 0x0039_b960, 0x001e_04e0)
}

/// `void MessageCenter_TriggerGroups(MessageCenter* self, int type)`
/// (0x1e14c0): empty every trigger the group table has for `type`.
pub fn message_center_trigger_groups(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[V0][0] = 0x44_0000;
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x20);
        c.g[S1][0] = sx(0x0044_1630);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = 0x5;
        loop {
            c.cycles += 4;
            c.g[RA][0] = sx(0x001e_14f8);
            c.g[A0][0] = c.g[S0][0];
            callee::MESSAGE_CENTER_IS_FLAG_SET.call(c, 0x001e_14f8)?;
            let mut skip = false;
            if c.g[V0][0] != 0 {
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S1)).lw(c);
                if c.g[V0][0] != c.g[S2][0] {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[S3][0] {
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                        skip = true;
                    }
                }
                if !skip {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001e_151c);
                    c.g[A0][0] = c.g[S0][0];
                    callee::MESSAGE_CENTER_EMPTY_TRIGGER.call(c, 0x001e_151c)?;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 61) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x14));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001e_14f0);
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Popup / screen destructors.

/// `void cFEStateBraggingRights_dtor(self, int flags)` (0x1df4e8): run the
/// destructors of the twelve embedded widgets at_2 `self + 0x6d0` (virtual
/// `dtor(0)`, slot 1 of the vtable at_2 `widget + 0x30`), then the FE state's
/// own destructor.
pub fn cfe_state_bragging_rights_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        let self_ = abi::arg(c, 0);
        let flags = abi::arg64(c, 1);
        c.g[V0][0] = 0x0047_0000;
        frame.save(c, S1, 0x30);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x5098));
        frame.save(c, S3, 0x10);
        c.g[S1][0] = sx(self_);
        frame.save(c, S0, 0x40);
        c.g[V1][0] = sx(p(self_, 0x6d0));
        frame.save(c, S2, 0x20);
        c.g[S3][0] = flags;
        frame.save_ra(c, 0);
        let no_widgets = c.g[V1][0] == 0;
        c.sw(at_2(c, S1, 8), c.g[V0][0]);
        if !no_widgets {
            c.cycles += 3;
            c.g[S0][0] = sx(p(r32(c, S1), 0x738));
            let empty = c.g[V1][0] == c.g[S0][0];
            c.g[S2][0] = c.g[V1][0];
            if !empty {
                c.cycles += 2;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x34));
                loop {
                    c.cycles += 8;
                    c.g[A1][0] = 0;
                    c.g[V0][0] = c.lw(at_2(c, S0, 0x30));
                    c.g[A0][0] = c.lh(at_2(c, V0, 8));
                    c.g[V1][0] = c.lw(at_2(c, V0, 0xc));
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                    let target = r32(c, V1);
                    call_ptr(c, target, 0x001d_f548)?;
                    let more = c.g[S2][0] != c.g[S0][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x34));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x001d_f530);
                    }
                }
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, callee::GAME_MODULE_DTOR, 0x001d_f55c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenMessageCenter_Shutdown(OVScreenMessageCenter* self)`
/// (0x1e3e30): pop the screen off the message centre's stack in the
/// direction its state says, then destroy the screen itself.
pub fn ov_screen_message_center_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(self_);
        c.g[V1][0] = c.lw(p(self_, 0x18));
        let is_enter = c.g[V1][0] == c.g[V0][0];
        c.g[A0][0] = 7;
        let push;
        if is_enter {
            c.cycles += 2;
            c.g[A1][0] = 0;
            push = true;
        } else {
            c.cycles += 2;
            let zero = c.g[V1][0] == 0;
            c.g[A1][0] = 1;
            push = zero;
        }
        if push {
            c.cycles += 18;
            call_ptr(c, callee::MESSAGE_CENTER_SET_FLAG, 0x001e_3e68)?;
            call_ptr(c, callee::OVSCREEN_PDA_TEMPLATE_CLOSE, 0x001e_3e70)?;
            c.g[V0][0] = c.lw(gp(c, -0x69c));
            c.g[A0][0] = 0x0053_0000;
            c.g[A0][0] = sx(p(r32(c, A0), 0x66e8));
            c.g[A1][0] = 0x0044_0000;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            c.g[A1][0] = sx(p(r32(c, A1), 0x28f0));
            c.g[V1][0] = sx(r32(c, V0) << 2);
            c.sw(gp(c, -0x69c), c.g[V0][0]);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
            c.g[V0][0] = c.lw(r32(c, V1));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
            c.g[V1][0] = c.lw(r32(c, V0));
            c.sw(gp(c, -0x6a0), c.g[V1][0]);
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::OVSCREEN_MESSAGE_CENTER_DTOR, 0x001e_3eb0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenMessageCenter_OnActivate(self)` (0x1e4168): when the message
/// centre opens (state 1) select the first entry of the message list, refresh
/// the menu and open the view popup; otherwise just refresh the menu.
pub fn ov_screen_message_center_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = lw_4(c, S0, 0x18);
        c.g[V0][0] = 1;
        let mut open = c.g[V1][0] == 0;
        if !open {
            c.cycles += 2;
            if c.g[V1][0] != c.g[V0][0] {
                // The non-reset arm re-reads the menu pointer.
                c.g[A2][0] = lw_4(c, S0, 0x144);
            } else {
                open = true;
            }
        }
        if open {
            c.cycles += 18;
            c.g[A0][0] = 0x25;
            callee::MESSAGE_CENTER_GET_LIST_INDEX.call(c, 0x001e_4194)?;
            c.g[A0][0] = lw_4(c, S0, 0x144);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, table_callee::UI_MENU_SET_SELECTED_BY_INDEX, 0x001e_41a0)?;
            c.g[A2][0] = lw_4(c, S0, 0x144);
            c.g[A1][0] = 1;
            c.g[V0][0] = lw_4(c, A2, 0x8);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x38).lh(c);
            c.g[V1][0] = lw_4(c, V0, 0x3c);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x001e_41bc)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x25;
            c.g[A2][0] = u64::MAX;
            callee::MESSAGE_CENTER_OPEN_VIEW_POPUP.call(c, 0x001e_41cc)?;
        } else {
            c.cycles += 7;
            c.g[A1][0] = 1;
            c.g[V0][0] = lw_4(c, A2, 0x8);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x38).lh(c);
            c.g[V1][0] = lw_4(c, V0, 0x3c);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x001e_41ec)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEPopupSelectMultiplayerMode_OnWidgetCreated(self, item)` (0x1e0358):
/// the item hash selects one of three widget ids: show/refresh the
/// multiplayer list, clear a flag, or set it.
pub fn cfe_popup_select_multiplayer_mode_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x0046_2000;
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let item = abi::arg(c, 1);
        c.g[S1][0] = sx(item);
        frame.save_ra(c, 0);
        c.g[S0][0] = lw_5(c, S1, 0x38);
        call_ptr(c, table::STR_HASH, 0x001e_037c)?;
        let first = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x004a_0000;
        if !first {
            c.cycles += 10;
            c.g[V0][0] = lw_5(c, S1, 0x14);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 2;
            c.g[V0][0] |= 1;
            sw_2(c, S1, 0x14, c.g[V0][0]);
            call_ptr(c, table::UI_MENU_RESET_SCROLL, 0x001e_039c)?;
            c.g[V0][0] = lw_5(c, S1, 0x14) | 0x80;
            sw_2(c, S1, 0x14, c.g[V0][0]);
        } else {
            c.cycles += 5;
            c.g[S0][0] = lw_5(c, S1, 0x38);
            c.g[A0][0] = 0x004a_13a0;
            call_ptr(c, table::STR_HASH, 0x001e_03b8)?;
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 2;
                sw_2(c, S1, 0x18, 0);
            } else {
                c.cycles += 5;
                c.g[S0][0] = lw_5(c, S1, 0x38);
                c.g[A0][0] = 0x004a_14f8;
                call_ptr(c, table::STR_HASH, 0x001e_03d4)?;
                let other = c.g[S0][0] != c.g[V0][0];
                frame.restore(c, S0, 0x20);
                if other {
                    c.cycles += 4;
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 2;
                c.g[V0][0] = 1;
                sw_2(c, S1, 0x18, c.g[V0][0]);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MessageCenter_CountGroupMessages(int type, int all)` (0x1e2bb8):
/// total the counts of the message-table entries of `type`, treating
/// entries with no count/flag as one.
pub fn message_center_count_group_messages(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = sx(0x0044_1630);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A0][0];
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            if c.g[V0][0] != c.g[S4][0] {
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            } else {
                c.cycles += 2;
                let mut count = false;
                if c.g[S3][0] != 0 {
                    c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
                    count = true;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(r32(c, S0), 0x10)).lw(c);
                    if c.g[V0][0] == 0 {
                        c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x001e_3a30, 0x001e_2c18)?;
                        if c.g[V0][0] == 0 {
                            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
                            count = true;
                        }
                    }
                }
                if count {
                    c.cycles += 1;
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
                }
                c.cycles += 1;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 61) as u64;
            c.g[S0][0] = sx(p(r32(c, S0), 0x14));
            if c.g[V0][0] != 0 {
                if event_due(c) {
                    deliver_event_here(c, 0x001e_2bf0);
                }
                continue;
            }
            c.cycles += 9;
            c.g[V0][0] = c.g[S2][0];
            frame.restore(c, S0, 0x50);
            frame.restore(c, S1, 0x40);
            frame.restore(c, S2, 0x30);
            frame.restore(c, S3, 0x20);
            frame.restore(c, S4, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            break;
        }
        Ok(())
    })
}

/// `int OVScreenFinalStandings_Init(self)` (0x1e6668): create the two screens
/// of the final-standings state, start their threads and mark the state in.
pub fn ov_screen_final_standings_init(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        {
            c.cycles += 16;
            frame.save(c, S1, 0x10);
            c.g[S1][0] = c.g[A0][0];
            frame.save_ra(c, 0);
            frame.save(c, S0, 0x20);
            c.g[A0][0] = sx(0x0046_e520);
            c.g[RA][0] = sx(0x001e_668c);
            c.g[S0][0] = lw_6(c, at_5(c.g[S1][0], 0x10));
            call_ptr(c, 0x0031_7670, 0x001e_668c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001e_66a0);
            c.g[A3][0] = 0;
            call_ptr(c, 0x0039_7cc0, 0x001e_66a0)?;
            let t = c.g[V0][0] == 0;
            Addr(at_5(c.g[S1][0], 0x40)).sw(c, c.g[V0][0]);
            if !t {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                c.g[RA][0] = sx(0x001e_66b8);
                c.g[A2][0] = 0;
                call_ptr(c, 0x0039_c870, 0x001e_66b8)?;
            }
        }
        {
            c.cycles += 11;
            c.g[A0][0] = sx(0x0046_0000);
            c.g[S0][0] = lw_6(c, at_5(c.g[S1][0], 0x10));
            c.g[RA][0] = sx(0x001e_66c8);
            c.g[A0][0] = sx(0x0046_e378);
            call_ptr(c, 0x0031_7670, 0x001e_66c8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001e_66dc);
            c.g[A3][0] = 0;
            call_ptr(c, 0x0039_7cc0, 0x001e_66dc)?;
            let t = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !t {
                c.cycles += 3;
                c.g[A1][0] = 0;
                c.g[RA][0] = sx(0x001e_66f0);
                c.g[A2][0] = 0;
                call_ptr(c, 0x0039_c870, 0x001e_66f0)?;
            }
        }
        c.cycles += 9;
        c.g[RA][0] = sx(0x001e_66f8);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0020_a380, 0x001e_66f8)?;
        c.g[V0][0] = 1;
        frame.restore(c, S0, 0x20);
        Addr(at_5(c.g[S1][0], 0x9c)).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `MessageCenter_RemoveMessage` (0x1e31b8): shift the inbox entries after
// `idx` down by one, carry the selection bit down with them and decrement the
// count.
pub fn message_center_remove_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MESSAGE_CENTER_GET_INBOX_4.call(c, 0x001e_31cc)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xcc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A3][0] = 0x1;
            c.g[V0][0] = sx((c.g[S0][0] as u32) << 3);
            loop {
                c.cycles += 12;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[A2][0] = c.g[V1][0];
                c.ldl(4, (c.g[V0][0] as u32).wrapping_add(0xf));
                c.ldr(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x7));
                c.sdr(4, (c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[A3][0] as u32) << ((c.g[V1][0] as u32) & 31));
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc8));
                c.g[V1][0] = c.g[A0][0] & c.g[V1][0];
                let empty = c.g[V1][0] == 0;
                c.g[V0][0] = sx((c.g[A3][0] as u32) << ((c.g[S0][0] as u32) & 31));
                if !empty {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[A0][0] | c.g[V0][0];
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = !(0u64 | c.g[V0][0]);
                    c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
                }
                c.cycles += 7;
                c.sw((c.g[A1][0] as u32).wrapping_add(0xc8), c.g[V0][0]);
                c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xcc));
                c.g[S0][0] = c.g[A2][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[S0][0] as u32) << 3);
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xcc));
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001e_31f0);
                }
            }
        } else {
            c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xcc));
        }
        c.cycles += 6;
        c.lq(16, frame.at(c, 0x10));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.sb((c.g[A1][0] as u32).wrapping_add(0xcc), c.g[V0][0]);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

/// `int MessageCenter_PickVariant(MessageCenter* self)` (0x1e3100): pick a
/// random variant index for a message, with type-specific handling (type 8
/// picks a variant directly, types 3/4/5 pick a random message id).
pub fn message_center_pick_variant(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[S1][0] = 0xff;
        c.g[A0][0] = 0x3;
        c.g[A1][0] = this;
        call_ptr(c, 0x001e_2d58, 0x001e_3124)?;
        let is3 = c.g[V0][0] != 0;
        c.g[A0][0] = 0x5;
        // Types 3 and 4 pick a random message id (the 0xa variant), type 5
        // uses the 0xb variant, type 8 a random variant index.
        let mut rand3 = is3;
        let mut rand5 = false;
        if !is3 {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x001e_2d58, 0x001e_3134)?;
            let not5 = c.g[V0][0] == 0;
            c.g[A0][0] = 0x8;
            if !not5 {
                c.cycles += 4;
                call_ptr(c, 0x0031_77f0, 0x001e_3144)?;
                c.g[V1][0] = 0xb;
                rand5 = true;
            } else {
                c.cycles += 4;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x001e_2d58, 0x001e_3154)?;
                let not8 = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[S0][0];
                if !not8 {
                    c.cycles += 4;
                    call_ptr(c, 0x0031_77f0, 0x001e_3164)?;
                    c.g[S1][0] = c.g[V0][0] & 0x3;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = 0x4;
                    call_ptr(c, 0x001e_2d58, 0x001e_3174)?;
                    let not4 = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[S1][0];
                    rand3 = !not4;
                }
            }
        }
        if rand3 {
            c.cycles += 3;
            call_ptr(c, 0x0031_77f0, 0x001e_317c)?;
            c.g[V1][0] = 0xa;
        }
        if rand3 || rand5 {
            c.cycles += 3;
            c.divu(0, 2, 3);
            c.cycles += 2;
            c.g[A0][0] = c.hi[0];
            c.g[S1][0] = c.g[A0][0] & 0xff;
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        c.cycles += 5;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Message center.

/// `void MessageCenter_SetPeakForeshadowDone(int peak)` (0x1e2370): when the
/// current peak is in the first ten, mark the peak's foreshadow pair of
/// message flags as done (flags 0x1c/0x1d, 0x1e/0x1f or 0x20/0x21).
pub fn message_center_set_peak_foreshadow_done(c: &mut Cpu) {
    guest(c, |c| {
        'preamble: {
            c.cycles += 9;
            c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x10));
            c.g[V0][0] = Addr(gp_4(c, -0x848)).lw(c);
            c.sd(r(c, SP), c.g[RA][0]);
            c.g[A1][0] = c.g[A0][0];
            c.g[V1][0] = Addr(p(r(c, V0), 0x84)).lw(c);
            c.g[A0][0] = Addr(p(r(c, V1), 0x28)).lw(c);
            c.g[A0][0] = Addr(r(c, A0)).lw(c);
            let none = c.g[A0][0] == 0;
            c.g[A2][0] = 0;
            if none {
                break 'preamble;
            }
            c.cycles += 1;
            c.g[A2][0] = ((c.g[A0][0] as i64) < 10) as u64;
        }
        'done: {
            c.cycles += 2;
            let skip = c.g[A2][0] != 0;
            c.g[RA][0] = c.ld(r(c, SP));
            if skip {
                break 'done;
            }
            'tail: {
                'pairs: {
                    'two: {
                        c.cycles += 3;
                        c.g[V0][0] = 1;
                        let is_one = c.g[A1][0] == c.g[V0][0];
                        c.g[V0][0] = ((c.g[A1][0] as i64) < 2) as u64;
                        if is_one {
                            break 'two;
                        }
                        'zero: {
                            'cmp: {
                                c.cycles += 2;
                                if c.g[V0][0] == 0 {
                                    c.g[V0][0] = 2;
                                    break 'cmp;
                                }
                                c.cycles += 2;
                                let is_zero = c.g[A1][0] == 0;
                                c.g[A0][0] = 0x1c;
                                if is_zero {
                                    break 'zero;
                                }
                                c.cycles += 2;
                                break 'done;
                            }
                            c.cycles += 2;
                            let is_two = c.g[A1][0] == c.g[V0][0];
                            c.g[RA][0] = c.ld(r(c, SP));
                            if is_two {
                                break 'tail;
                            }
                            c.cycles += 2;
                            break 'done;
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001e_23dc);
                        c.g[A1][0] = 1;
                        callee::MESSAGE_CENTER_SET_FLAG_2.call(c, 0x001e_23dc)?;
                        c.g[A0][0] = 0x1d;
                        break 'pairs;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = 0x1e;
                    c.g[RA][0] = sx(0x001e_23f0);
                    c.g[A1][0] = 1;
                    callee::MESSAGE_CENTER_SET_FLAG_2.call(c, 0x001e_23f0)?;
                    c.g[A0][0] = 0x1f;
                    break 'pairs;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x001e_23fc);
                c.g[A1][0] = 1;
                callee::MESSAGE_CENTER_SET_FLAG_2.call(c, 0x001e_23fc)?;
                c.g[RA][0] = c.ld(r(c, SP));
                break 'done;
            }
            c.cycles += 7;
            c.g[A0][0] = 0x20;
            c.g[RA][0] = sx(0x001e_2410);
            c.g[A1][0] = 1;
            callee::MESSAGE_CENTER_SET_FLAG_2.call(c, 0x001e_2410)?;
            c.g[A0][0] = 0x21;
            c.g[RA][0] = sx(0x001e_241c);
            c.g[A1][0] = 1;
            callee::MESSAGE_CENTER_SET_FLAG_2.call(c, 0x001e_241c)?;
            c.g[RA][0] = c.ld(r(c, SP));
        }
        c.cycles += 2;
        let ra = r(c, RA);
        c.g[SP][0] = sx(r(c, SP).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FE message centre.

/// `void MessageCenter_RemoveMessagesOfGroup(int group)` (0x1e32c8): walk the
/// inbox message list and drop every message whose group (in the global table
/// at_6 0x4c6c08, stride 0x18, field +4) matches, compacting as it goes.
pub fn message_center_remove_messages_of_group(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        open(c, 0x80);
        c.sq(20, at_6(c.g[SP][0] as u32, 0x30));
        c.sq(17, at_6(c.g[SP][0] as u32, 0x60));
        let group = c.g[A0][0];
        c.g[S4][0] = group;
        c.sq(18, at_6(c.g[SP][0] as u32, 0x50));
        c.g[S1][0] = 0;
        c.sq(19, at_6(c.g[SP][0] as u32, 0x40));
        c.sq(16, at_6(c.g[SP][0] as u32, 0x70));
        c.sq(21, at_6(c.g[SP][0] as u32, 0x20));
        c.sq(22, at_6(c.g[SP][0] as u32, 0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x001e_32fc);
        c.g[S3][0] = 0;
        call_ptr(c, 0x001e_39f8, 0x001e_32fc)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0xcc).lbu(c);
        let empty = c.g[V0][0] == 0;
        c.g[V0][0] = 0x4c0000;
        if !empty {
            c.cycles += 5;
            c.g[S6][0] = 0x18;
            c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6c08));
            c.g[S0][0] = c.g[S2][0];
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            loop {
                c.cycles += 5;
                c.mult(0, 3, 2, 22);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S5][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(4).lw(c);
                if c.g[S4][0] != c.g[V1][0] {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(8));
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e_333c);
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x001e_31b8, 0x001e_333c)?;
                    c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0xcc).lbu(c);
                }
                c.cycles += 2;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0xcc).lbu(c);
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x001e_3320);
                }
            }
        }
        c.cycles += 10;
        c.lq(16, at_6(c.g[SP][0] as u32, 0x70));
        c.lq(17, at_6(c.g[SP][0] as u32, 0x60));
        c.lq(18, at_6(c.g[SP][0] as u32, 0x50));
        c.lq(19, at_6(c.g[SP][0] as u32, 0x40));
        c.lq(20, at_6(c.g[SP][0] as u32, 0x30));
        c.lq(21, at_6(c.g[SP][0] as u32, 0x20));
        c.lq(22, at_6(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x80);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `MessageCenter_GetIndexInType` (0x001e2dc0).
//
// `int MessageCenter::GetIndexInType(int type, int id)`: walk the message
// type's entries to the one holding `id` and return its running index.

pub fn message_center_get_index_in_type(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e2e78: {
            c.cycles += 15;
            c.g[A3][0] = c.g[A0][0];
            c.g[V1][0] = 0x18;
            c.mult(0, 3, 7, 3);
            c.g[V0][0] = 0x4c0000;
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6c08));
            c.g[A0][0] = 0x14;
            c.g[V0][0] = 0x440000;
            c.g[T1][0] = 0u64;
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1630));
            c.g[T0][0] = 0u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
            c.mult(0, 2, 2, 4);
            let t = (c.g[A3][0] as i64) < 0;
            c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            if t {
                break 'b_1e2e78;
            } else {
                c.cycles += 5;
                c.g[T5][0] = c.g[A2][0];
                c.g[T6][0] = c.g[A1][0];
                c.g[T4][0] = 0x18;
                c.g[T3][0] = 0x14;
                'l_1e2e10: loop {
                    'b_1e2e5c: {
                        c.cycles += 12;
                        c.mult(0, 3, 8, 12);
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T5][0] as u32)));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                        c.mult(0, 2, 3, 11);
                        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T6][0] as u32)));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A3][0] as i64)) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                            break 'b_1e2e5c;
                        } else {
                            c.cycles += 4;
                            c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0xc));
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
                            if t {
                                break 'b_1e2e78;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                                let t = true;
                                c.g[T1][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                if t {
                                    break 'b_1e2e78;
                                } else {
                                    break 'b_1e2e5c;
                                }
                            }
                        }
                    }
                    c.cycles += 7;
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.g[A1][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                    let t = c.g[A1][0] == 0u64;
                    if c.g[V0][0] == 0 {
                        c.g[T1][0] = c.g[A0][0];
                    }
                    if t {
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x001e2e10);
                        }
                        continue 'l_1e2e10;
                    } else {
                        break 'b_1e2e78;
                    }
                }
            }
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
        c.pc = t;
        return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `OVScreenMultiplayerStandings_Init` (0x001e8f00).
//
// `void OVScreenMultiplayerStandings::Init(self)`: build the standings and
// header widgets and hook up the audio cue.

pub fn ov_screen_multiplayer_standings_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e8f50: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = 0x470000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1640));
            c.g[RA][0] = sx(0x001e8f24);
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001e8f24)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001e8f38);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001e8f38)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1e8f50;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001e8f50);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001e8f50)?;
                break 'b_1e8f50;
            }
        }
        'b_1e8f88: {
            c.cycles += 11;
            c.g[A0][0] = 0x470000;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = sx(0x001e8f60);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c88));
            call_ptr(c, 0x00317670, 0x001e8f60)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001e8f74);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001e8f74)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[V0][0];
            if t {
                break 'b_1e8f88;
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001e8f88);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001e8f88)?;
                break 'b_1e8f88;
            }
        }
        c.cycles += 14;
        c.g[RA][0] = sx(0x001e8f90);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001e8fc0, 0x001e8f90)?;
        c.g[RA][0] = sx(0x001e8f98);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0020a380, 0x001e8f98)?;
        c.g[RA][0] = sx(0x001e8fa0);
        call_ptr(c, 0x0028b180, 0x001e8fa0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001e8fac);
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x00294f78, 0x001e8fac)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());

        Ok(())
    })
}

/// `OVScreenFinishOv::Update(this)` (0x1e8098).
pub fn ov_screen_finish_ov_update(c: &mut Cpu) {
    'b_1e80fc: {
        c.cycles += 23;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x001e80b0);
        c.g[S0][0] = c.g[A0][0];
        if call_ptr(c, 0x0014dc80, 0x001e80b0).is_err() {
            return;
        }
        c.g[AT][0] = 0x40400000;
        c.mtc1(1, 20);
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001e80c4);
        c.g[A1][0] = 0x1;
        if call_ptr(c, 0x0014dd58, 0x001e80c4).is_err() {
            return;
        }
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001e80d0);
        c.g[A1][0] = 0u64;
        if call_ptr(c, 0x00146e98, 0x001e80d0).is_err() {
            return;
        }
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x480));
        let t = c.g[A0][0] == 0u64;

        if t {
            break 'b_1e80fc;
        } else {
            c.cycles += 2;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 20);
            break 'b_1e80fc;
        }
    }
    'b_1e8148: {
        c.cycles += 8;
        c.g[RA][0] = sx(0x001e8104);
        c.g[A0][0] = c.g[S0][0];
        if call_ptr(c, 0x0020e900, 0x001e8104).is_err() {
            return;
        }
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x9c));
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x578c));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
        c.add_s(0, 0, 1);
        let t = c.g[V0][0] == 0u64;
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x9c));
        if t {
            break 'b_1e8148;
        } else {
            c.cycles += 4;
            c.c_le_s(20, 0);
            let t = !c.fcc();
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if t {
                break 'b_1e8148;
            } else {
                c.cycles += 7;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x48));
                c.g[RA][0] = sx(0x001e813c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                if call_ptr(c, 0x0039f840, 0x001e813c).is_err() {
                    return;
                }
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[RA][0] = sx(0x001e8148);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                if call_ptr(c, 0x00231320, 0x001e8148).is_err() {
                    return;
                }
                break 'b_1e8148;
            }
        }
    }
    c.cycles += 5;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = t;
    return;
}

/// `OVScreenFinishOv2::Update(this)` (0x1e87b8) — same body, one second is
/// read from a different constant (4 bytes higher).
pub fn ov_screen_finish_ov2_update(c: &mut Cpu) {
    'b_1e881c: {
        c.cycles += 23;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x001e87d0);
        c.g[S0][0] = c.g[A0][0];
        if call_ptr(c, 0x0014dc80, 0x001e87d0).is_err() {
            return;
        }
        c.g[AT][0] = 0x40400000;
        c.mtc1(1, 20);
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001e87e4);
        c.g[A1][0] = 0x1;
        if call_ptr(c, 0x0014dd58, 0x001e87e4).is_err() {
            return;
        }
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001e87f0);
        c.g[A1][0] = 0u64;
        if call_ptr(c, 0x00146e98, 0x001e87f0).is_err() {
            return;
        }
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x480));
        let t = c.g[A0][0] == 0u64;

        if t {
            break 'b_1e881c;
        } else {
            c.cycles += 2;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 20);
            break 'b_1e881c;
        }
    }
    'b_1e8868: {
        c.cycles += 8;
        c.g[RA][0] = sx(0x001e8824);
        c.g[A0][0] = c.g[S0][0];
        if call_ptr(c, 0x0020e900, 0x001e8824).is_err() {
            return;
        }
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x9c));
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5788));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
        c.add_s(0, 0, 1);
        let t = c.g[V0][0] == 0u64;
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x9c));
        if t {
            break 'b_1e8868;
        } else {
            c.cycles += 4;
            c.c_le_s(20, 0);
            let t = !c.fcc();
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if t {
                break 'b_1e8868;
            } else {
                c.cycles += 7;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x48));
                c.g[RA][0] = sx(0x001e885c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                if call_ptr(c, 0x0039f840, 0x001e885c).is_err() {
                    return;
                }
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[RA][0] = sx(0x001e8868);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                if call_ptr(c, 0x00231320, 0x001e8868).is_err() {
                    return;
                }
                break 'b_1e8868;
            }
        }
    }
    c.cycles += 5;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = t;
    return;
}

// ---------------------------------------------------------------------------
// Final-standings screen.

/// `int OVScreenFinalStandings_HandleInput(OVScreenFinalStandings *self,
/// Input *input)` (0x1e7aa8): route the input to the standings list when the
/// "final" object is selected, tell the BE interface about it, and run the
/// pending overlay handler. Always returns 0.
pub fn ov_screen_final_standings_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e7b08: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = 0x470000;
            c.g[S1][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[RA][0] = sx(0x001e7acc);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1af0));
            call_ptr(c, 0x00317670, 0x001e7acc)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001e7ad8);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001e7ad8)?;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                break 'b_1e7b08;
            } else {
                c.cycles += 4;
                c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x95));
                c.g[V0][0] = 0x2;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if t {
                    break 'b_1e7b08;
                } else {
                    c.cycles += 6;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.g[RA][0] = sx(0x001e7b04);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                    call_ptr(c, 0x0026f898, 0x001e7b04)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    break 'b_1e7b08;
                }
            }
        }
        'b_1e7b64: {
            'b_1e7b60: {
                'b_1e7b5c: {
                    c.cycles += 7;
                    c.g[A1][0] = 0x7;
                    c.g[RA][0] = sx(0x001e7b14);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    call_ptr(c, 0x0014dd58, 0x001e7b14)?;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                    if t {
                        break 'b_1e7b5c;
                    } else {
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        if t {
                            break 'b_1e7b60;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_1e7b64;
                            } else {
                                c.cycles += 7;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                vslot(c, c.g[V1][0], 0xd8, V0);
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001e7b4c);
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001e7b4c)?;
                                let t = c.g[V0][0] == 0u64;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_1e7b60;
                                } else {
                                    c.cycles += 2;
                                    c.g[RA][0] = sx(0x001e7b5c);
                                    c.g[A0][0] = 0x28;
                                    call_ptr(c, 0x0020ab50, 0x001e7b5c)?;
                                    break 'b_1e7b5c;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_1e7b60;
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_1e7b64;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Message center.

/// `int MessageCenter_GetListIndex(MessageCenter* self, int id)` (0x1e3680):
/// find the index of message `id` in the visible and grouped message lists
/// (0xff when absent and no list is active).
pub fn messagecenter_get_list_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(gp(c, -0x102c)).lw(c);
        frame.save(c, S2, 0x40);
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        let active = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if active {
            c.cycles += 10;
            c.g[A0][0] = 4;
            c.g[A1][0] = 0;
            call_ptr(c, 0x001e_2bb8, 0x001e_36c0)?;
            c.g[S0][0] = 0;
            c.g[S4][0] = c.g[V0][0];
            call_ptr(c, 0x001e_33e8, 0x001e_36cc)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[V0][0] = sx(p(r32(c, S4), r32(c, S3)));
            let none = (c.g[V0][0] as i64) <= 0;
            c.g[S5][0] = c.g[V0][0];
            if !none {
                c.cycles += 1;
                c.g[V0][0] = sx(r32(c, S0).wrapping_add(1));
                loop {
                    c.cycles += 3;
                    c.g[A0][0] = sx(r32(c, S3).wrapping_sub(r32(c, V0)));
                    let negative = (c.g[A0][0] as i64) < 0;
                    c.g[S1][0] = c.g[V0][0];
                    let mut matched = false;
                    if !negative {
                        c.cycles += 4;
                        call_ptr(c, 0x001e_3620, 0x001e_36f4)?;
                        if c.g[V0][0] == c.g[S2][0] {
                            c.cycles += 2;
                            c.g[V0][0] = c.g[S0][0] & 0xff;
                            matched = true;
                        } else {
                            c.g[S0][0] = c.g[S1][0];
                        }
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = sx(p(r32(c, A0), r32(c, S4)));
                        let group_neg = (c.g[V0][0] as i64) < 0;
                        c.g[A1][0] = !(0u64 | c.g[A0][0]);
                        if !group_neg {
                            c.cycles += 5;
                            c.g[A2][0] = 1;
                            c.g[A0][0] = 4;
                            call_ptr(c, 0x001e_2c80, 0x001e_371c)?;
                            if c.g[V0][0] == c.g[S2][0] {
                                c.g[V0][0] = c.g[S0][0] & 0xff;
                                matched = true;
                            }
                        }
                        if !matched {
                            c.cycles += 1;
                            c.g[S0][0] = c.g[S1][0];
                        }
                    }
                    if matched {
                        c.cycles += 9;
                        frame.restore(c, S0, 0x60);
                        frame.restore(c, S1, 0x50);
                        frame.restore(c, S2, 0x40);
                        frame.restore(c, S3, 0x30);
                        frame.restore(c, S4, 0x20);
                        frame.restore(c, S5, 0x10);
                        frame.restore_ra(c, 0);
                        frame.close(c);
                        return Ok(());
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(1));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x001e_36e0);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0xff;
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[S2][0] & 0xff;
        }
        // (the inactive path returns above without the 0xff arm)
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

// ---------------------------------------------------------------------------
// cOVState_ViewPopup_OnWidgetAction (0x00001e5800).

/// `cOVState_ViewPopup_OnWidgetAction`: handle a popup widget action: close the message (and remove it from the list) or forward the close to the owner.
pub fn c_ov_state_view_popup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e58d4: {
            'b_1e58b0: {
                c.cycles += 6;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.g[V0][0] = 0x6;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                let t = c.g[A2][0] == c.g[V0][0];
                c.g[S0][0] = c.g[A0][0];
                if t {
                    break 'b_1e58b0;
                } else {
                    'b_1e588c: {
                        'b_1e5834: {
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x5;
                            if t {
                                break 'b_1e5834;
                            } else {
                                c.cycles += 2;
                                if c.g[A2][0] == c.g[V0][0] {
                                    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                                    break 'b_1e588c;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                    if t {
                                        break 'b_1e58d4;
                                    } else {
                                        break 'b_1e5834;
                                    }
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 0x7;
                        if c.g[A2][0] != c.g[V0][0] {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                            break 'b_1e58d4;
                        } else {
                            c.cycles += 16;
                            c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                            c.g[V1][0] = 0x18;
                            c.g[A1][0] = 0x4c0000;
                            c.g[A3][0] = 0x14;
                            c.mult(0, 3, 8, 3);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c08));
                            c.g[A0][0] = 0x440000;
                            c.g[A2][0] = 0x4;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1630));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                            c.mult(0, 2, 2, 7);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                            if c.g[V1][0] == c.g[A2][0] {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                break 'b_1e58d4;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.g[T0][0];
                                call_ptr(c, 0x001e3268, 0x001e5888)?;
                                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                                break 'b_1e588c;
                            }
                        }
                    }
                    c.cycles += 9;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = 0x14;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x88));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[RA][0] = sx(0x001e58a8);
                    c.pc = t;
                    ssx3_ee_rt::funcs::call_indirect(c, t);
                    if c.pc != 0x001e58a8 {
                        return Err(crate::call::Unwound);
                    }
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_1e58d4;
                    } else {
                        break 'b_1e58b0;
                    }
                }
            }
            c.cycles += 9;
            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 0x14;
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x88));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[RA][0] = sx(0x001e58d0);
            c.pc = t;
            ssx3_ee_rt::funcs::call_indirect(c, t);
            if c.pc != 0x001e58d0 {
                return Err(crate::call::Unwound);
            }
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1e58d4;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cOVState_ViewPopup_GetGroupVariant (0x00001e5aa0).

/// `cOVState_ViewPopup_GetGroupVariant`: pick the popup group variant (a small modulo of the entry value) used to lay out the message list.
pub fn c_ov_state_view_popup_get_group_variant(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e5b70: {
            'b_1e5ac0: {
                c.cycles += 6;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x102c));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                let t = c.g[V0][0] != 0u64;
                c.g[S0][0] = c.g[A1][0];
                if t {
                    break 'b_1e5ac0;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0xff;
                    if t {
                        break 'b_1e5b70;
                    } else {
                        break 'b_1e5ac0;
                    }
                }
            }
            'b_1e5b60: {
                'b_1e5b5c: {
                    'b_1e5b58: {
                        c.cycles += 18;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x001e2dc0, 0x001e5ac8)?;
                        c.g[V1][0] = 0x18;
                        c.g[A1][0] = 0x4c0000;
                        c.mult(0, 3, 16, 3);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c08));
                        c.g[T1][0] = c.g[V0][0];
                        c.g[A3][0] = 0x14;
                        c.g[A0][0] = 0x440000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1630));
                        c.g[A2][0] = 0x4;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                        c.mult(0, 2, 2, 7);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        let t = c.g[V1][0] == c.g[A2][0];
                        c.g[T0][0] = 0x1;
                        if t {
                            break 'b_1e5b58;
                        } else {
                            'b_1e5b50: {
                                'b_1e5b40: {
                                    'b_1e5b24: {
                                        c.cycles += 3;
                                        c.g[V0][0] = ((c.g[V1][0] as i64) < 5i64) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x3;
                                        if t {
                                            break 'b_1e5b24;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == c.g[V0][0];
                                            c.g[V0][0] = sx((c.g[T1][0] as u32) >> 31);
                                            if t {
                                                break 'b_1e5b40;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.div(0, 9, 8);
                                                if t {
                                                    break 'b_1e5b60;
                                                } else {
                                                    break 'b_1e5b24;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x5;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x8;
                                    if t {
                                        break 'b_1e5b50;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[V1][0] == c.g[V0][0] {
                                            c.g[T0][0] = 0x4;
                                            break 'b_1e5b5c;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.div(0, 9, 8);
                                            if t {
                                                break 'b_1e5b60;
                                            } else {
                                                break 'b_1e5b40;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[T0][0] = 0xa;
                                c.g[V0][0] =
                                    sx((c.g[T1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                let t = true;
                                c.g[T1][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                                if t {
                                    break 'b_1e5b5c;
                                } else {
                                    break 'b_1e5b50;
                                }
                            }
                            c.cycles += 2;
                            let t = true;
                            c.g[T0][0] = 0xb;
                            if t {
                                break 'b_1e5b5c;
                            } else {
                                break 'b_1e5b58;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[T0][0] = 0xa;
                    break 'b_1e5b5c;
                }
                c.cycles += 1;
                c.div(0, 9, 8);
                break 'b_1e5b60;
            }
            'b_1e5b68: {
                c.cycles += 2;
                if c.g[T0][0] == 0u64 {
                    break 'b_1e5b68;
                } else {
                    break 'b_1e5b68;
                }
            }
            c.cycles += 2;
            c.g[V0][0] = c.hi[0];
            c.g[V0][0] = c.g[V0][0] & 0xff;
            break 'b_1e5b70;
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// OVScreenMessageCenter_UpdateCountText (0x00001e4758).

/// `OVScreenMessageCenter_UpdateCountText`: refresh the message-centre counter text.
pub fn ov_screen_message_center_update_count_text(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e47cc: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x13c));
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if t {
                break 'b_1e47cc;
            } else {
                c.cycles += 20;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1f58));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                call_ptr(c, 0x00317670, 0x001e4798)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001e47a8);
                c.pc = t;
                ssx3_ee_rt::funcs::call_indirect(c, t);
                if c.pc != 0x001e47a8 {
                    return Err(crate::call::Unwound);
                }
                c.g[S0][0] = c.g[V0][0];
                call_ptr(c, 0x001e33e8, 0x001e47b0)?;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.g[V0][0];
                call_ptr(c, 0x002c26d0, 0x001e47c0)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x13c));
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x003a0e90, 0x001e47cc)?;
                break 'b_1e47cc;
            }
        }
        'b_1e4828: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if t {
                break 'b_1e4828;
            } else {
                c.cycles += 20;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1f40));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                call_ptr(c, 0x00317670, 0x001e47f4)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001e4804);
                c.pc = t;
                ssx3_ee_rt::funcs::call_indirect(c, t);
                if c.pc != 0x001e4804 {
                    return Err(crate::call::Unwound);
                }
                c.g[S0][0] = c.g[V0][0];
                call_ptr(c, 0x001e3408, 0x001e480c)?;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.g[V0][0];
                call_ptr(c, 0x002c26d0, 0x001e481c)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x003a0e90, 0x001e4828)?;
                break 'b_1e4828;
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Message center.

/// `void MessageCenter_PostPeakUnlockMessage(self, which)` (0x1e2428): while
/// the current peak is one of the first ten, do nothing; otherwise set the
/// per-slot "seen" flag (`0x39`/`0x3a`/`0x3b`) and post the corresponding
/// unlock message (`0xfc`/`0xfd`/`0xfe`).
pub fn message_center_post_peak_unlock_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(gp_5(c, -0x848)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[V1][0] = Addr(r(c, V0)).off(0x84).lw(c);
        c.g[A0][0] = Addr(r(c, V1)).off(0x28).lw(c);
        c.g[A0][0] = Addr(r(c, A0)).lw(c);
        let no_peak = c.g[A0][0] == 0;
        c.g[A2][0] = 0;
        let busy: bool;
        if no_peak {
            busy = false;
        } else {
            c.cycles += 1;
            c.g[A2][0] = ((c.g[A0][0] as i64) < 10) as u64;
            busy = r(c, A2) != 0;
        }

        c.cycles += 2;
        'outer: {
            if busy {
                break 'outer;
            }
            'b_1e24e0: {
                'b_1e24d0: {
                    'b_1e24b0: {
                        c.cycles += 3;
                        c.g[V0][0] = 1;
                        let is_one = c.g[A1][0] == c.g[V0][0];
                        c.g[V0][0] = ((c.g[A1][0] as i64) < 2) as u64;
                        if is_one {
                            break 'b_1e24b0;
                        }
                        'b_1e248c: {
                            'b_1e247c: {
                                c.cycles += 2;
                                if c.g[V0][0] == 0 {
                                    c.g[V0][0] = 2;
                                    break 'b_1e247c;
                                }
                                c.cycles += 2;
                                if c.g[A1][0] == 0 {
                                    break 'b_1e248c;
                                }
                                c.cycles += 2;
                                break 'outer;
                            }
                            c.cycles += 2;
                            let is_two = c.g[A1][0] == c.g[V0][0];
                            if !is_two {
                                c.cycles += 2;
                                break 'outer;
                            }
                            break 'b_1e24e0;
                        }

                        // which == 0: flag 0x39, message 0xfc.
                        c.cycles += 4;
                        c.g[A0][0] = 0x39;
                        callee::MESSAGE_IS_FLAG_SET.call(c, 0x001e_2494)?;
                        if r(c, V0) != 0 {
                            break 'outer;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = 0x39;
                        c.g[A1][0] = 1;
                        callee::MESSAGE_SET_FLAG.call(c, 0x001e_24a8)?;
                        c.g[A0][0] = 0xfc;
                        break 'b_1e24d0;
                    }

                    // which == 1: flag 0x3a, message 0xfd.
                    c.cycles += 4;
                    c.g[A0][0] = 0x3a;
                    callee::MESSAGE_IS_FLAG_SET.call(c, 0x001e_24b8)?;
                    if r(c, V0) != 0 {
                        break 'outer;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = 0x39;
                    c.g[A1][0] = 1;
                    callee::MESSAGE_SET_FLAG.call(c, 0x001e_24cc)?;
                    c.g[A0][0] = 0xfd;
                    break 'b_1e24d0;
                }

                // Post the queued message (a0 holds 0xfc or 0xfd).
                c.cycles += 4;
                c.g[A1][0] = 0;
                callee::MESSAGE_POST.call(c, 0x001e_24d8)?;
                break 'outer;
            }

            // which == 2: flag 0x3b, message 0xfe.
            c.cycles += 4;
            c.g[A0][0] = 0x3b;
            callee::MESSAGE_IS_FLAG_SET.call(c, 0x001e_24e8)?;
            if r(c, V0) != 0 {
                break 'outer;
            }
            c.cycles += 7;
            c.g[A0][0] = 0x3b;
            c.g[A1][0] = 1;
            callee::MESSAGE_SET_FLAG.call(c, 0x001e_24fc)?;
            c.g[A0][0] = 0xfe;
            c.g[A1][0] = 0;
            callee::MESSAGE_POST.call(c, 0x001e_2508)?;
        }
        peak_unlock_tail(c, frame)
    })
}

/// The `ld ra; addiu sp; jr ra` tail.
pub(crate) fn peak_unlock_tail(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// A `gp_6`-relative global.
#[inline(always)]
pub(crate) fn gp_6(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// An `s32` expression of `v`.
#[inline(always)]
pub(crate) fn s32(v: u64) -> i32 {
    v as u32 as i32
}

// ---------------------------------------------------------------------------
// Functions.

/// `int func_001E4B80(int idx)` (0x1e4b80): the message-center group variant
/// byte for popup entry `idx`: 0xff when the message center is not up or the
/// entry is not of the "shown" kind, else `MessageCenter_GetIndexInType`
/// modulo a per-group divisor (entries of type 3 are halved first), masked to
/// a byte.
pub fn ovscreen_message_center_get_group_variant(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        let idx = c.g[A1][0];
        c.g[V0][0] = c.lw(gp_6(c, -0x102c)); // D_004A20C4: the message center
        frame.save(c, 16, 0x10);
        c.g[A2][0] = idx;
        frame.save_ra(c, 0);
        let up = c.g[V0][0] != 0;
        c.g[A3][0] = 0xff;
        if up {
            c.cycles += 9;
            c.g[A0][0] = 0x18;
            c.g[V0][0] = 0x4c0000;
            // a = &D_004C6C08[idx] (0x18-byte popup entries)
            c.mult(0, 4, 6, 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6c08));
            c.g[A1][0] = 0x1;
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[V1][0] = c.lw(p(c.g[S0][0] as u32, 8));
            let other_kind = c.g[V1][0] != c.g[A1][0];
            c.g[V0][0] = c.g[A3][0];
            if !other_kind {
                // b = &D_00441630[a->field_0x4] (0x14-byte group records)
                c.cycles += 13;
                c.g[RA][0] = sx(0x001e_4bd0);
                c.g[A0][0] = c.g[A2][0];
                c.pc = 0x001e_2dc0;
                ee::s04::MessageCenter_GetIndexInType(c);
                if c.pc != 0x001e_4bd0 {
                    return Err(Unwound);
                }
                c.g[V1][0] = c.lw(p(c.g[S0][0] as u32, 4));
                c.g[A1][0] = 0x14;
                c.g[A0][0] = 0x44_0000;
                c.g[A2][0] = c.g[V0][0];
                c.mult(0, 3, 3, 5);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1630));
                c.g[V0][0] = 0x4;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[A0][0] = c.lw(p(c.g[V1][0] as u32, 0xc));
                let case4 = c.g[A0][0] == c.g[V0][0];
                c.g[A1][0] = 0x1;
                if case4 {
                    c.cycles += 2;
                    c.g[A1][0] = 0xa;
                } else {
                    c.cycles += 3;
                    let ge5 = (s32(c.g[A0][0]) as i64) >= 5;
                    if ge5 {
                        c.cycles += 3;
                        if c.g[A0][0] == 0x5 {
                            c.cycles += 5;
                            c.g[A1][0] = 0xb;
                        } else if c.g[A0][0] == 0x8 {
                            c.cycles += 3;
                            c.g[A1][0] = 0x4;
                        } else {
                            c.cycles += 4;
                        }
                    } else if c.g[A0][0] == 0x3 {
                        // case 3: halve the index (rounding toward zero)
                        c.cycles += 2 + 5;
                        c.g[A1][0] = 0xa;
                        c.g[V0][0] = sx((c.g[A2][0] as u32) >> 31);
                        let v = (c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32);
                        c.g[A2][0] = sx(((v as i32) >> 1) as u32);
                    } else {
                        c.cycles += 2 + 2;
                    }
                }
                // the (signed) remainder, low byte
                c.div(0, A2, A1);
                c.cycles += 2;
                c.cycles += 3;
                c.g[V0][0] = c.hi[0];
                c.g[A3][0] = c.g[V0][0] & 0xff;
                c.g[V0][0] = c.g[A3][0];
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0xff;
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int func_001E1AD0(sRangeK1E1AD0* r, const char* arg)` (0x1e1ad0): pick a
/// random foreshadow candidate message. Allocates an index list, walks
/// `r->start .. r->start + r->count` entries of `D_004C6C08`, keeping those
/// whose character (`+0xc`) does not match `arg` and which
/// `MessageCenter_FindMessage` reports as unused (`-1`), then returns
/// `0x100` when none matched or `list[BXrand() % n]` otherwise.
pub fn message_center_pick_foreshadow_message(c: &mut Cpu) {
    let frame = Frame::open(c, 0x90);
    'b_1e1ba4: {
        c.cycles += 27;
        c.g[A2][0] = 0x100;
        frame.save(c, S0, 0x80);
        c.g[A3][0] = 0u64;
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x30);
        c.g[S4][0] = 0u64;
        frame.save(c, S6, 0x20);
        frame.save(c, S1, 0x70);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, S2, 0x60);
        c.g[A1][0] = 0x470000;
        frame.save(c, S7, 0x10);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x2208));
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[A0][0] = lw(c, at_7(c, S3, 0x8));
        c.g[RA][0] = sx(0x001e1b20);
        c.g[A0][0] = sx(r32(c, A0) << 2);
        if call_ptr(c, 0x00317e30, 0x001e1b20).is_err() {
            return;
        }
        c.g[S5][0] = c.g[V0][0];
        c.g[S0][0] = lw(c, at_7(c, S3, 0x4));
        c.g[V0][0] = lw(c, at_7(c, S3, 0x8));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x18;
        if t {
            break 'b_1e1ba4;
        } else {
            c.cycles += 7;
            c.g[V1][0] = 0x4c0000;
            c.mult(0, V0, S0, V0);
            c.g[V1][0] = sx(at_7(c, V1, 0x6c08));
            c.g[S7][0] = 0xffffffffffffffff;
            c.g[S2][0] = c.g[S5][0];
            c.g[V0][0] = sx(at_7(c, V0, 0xc));
            c.g[S1][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            'l_1e1b58: loop {
                'b_1e1b8c: {
                    c.cycles += 5;
                    c.g[A0][0] = lw(c, r32(c, S1));
                    c.g[RA][0] = sx(0x001e1b64);
                    c.g[A1][0] = c.g[S6][0];
                    if callee::C001e1a30.call(c, 0x001e1b64).is_err() {
                        return;
                    }
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = lw(c, at_7(c, S3, 0x8));
                        break 'b_1e1b8c;
                    } else {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001e1b74);
                        c.g[A0][0] = c.g[S0][0];
                        if callee::C001e3388.call(c, 0x001e1b74).is_err() {
                            return;
                        }
                        if c.g[V0][0] != c.g[S7][0] {
                            c.g[V0][0] = lw(c, at_7(c, S3, 0x8));
                            break 'b_1e1b8c;
                        } else {
                            c.cycles += 4;
                            sw_4(c, r32(c, S2), c.g[S0][0]);
                            c.g[S4][0] = sx(at_7(c, S4, 0x1));
                            c.g[S2][0] = sx(at_7(c, S2, 0x4));
                            c.g[V0][0] = lw(c, at_7(c, S3, 0x8));
                            break 'b_1e1b8c;
                        }
                    }
                }
                c.cycles += 6;
                c.g[S0][0] = sx(at_7(c, S0, 0x1));
                c.g[V1][0] = lw(c, at_7(c, S3, 0x4));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S1][0] = sx(at_7(c, S1, 0x18));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x001e1b58);
                    }
                    continue 'l_1e1b58;
                } else {
                    break 'b_1e1ba4;
                }
            }
        }
    }
    'b_1e1be0: {
        c.cycles += 2;
        let t = c.g[S4][0] == 0u64;
        c.g[V0][0] = 0x100;
        if t {
            break 'b_1e1be0;
        } else {
            'b_1e1bc0: {
                c.cycles += 5;
                c.g[RA][0] = sx(0x001e1bb4);
                if call_ptr(c, 0x003177f0, 0x001e1bb4).is_err() {
                    return;
                }
                c.divu(0, 2, 20);
                if c.g[S4][0] == 0u64 {
                    break 'b_1e1bc0;
                } else {
                    break 'b_1e1bc0;
                }
            }
            'b_1e1bdc: {
                c.cycles += 5;
                c.g[V1][0] = c.hi[0];
                c.g[V1][0] = sx(r32(c, V1) << 2);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, S5)));
                let t = c.g[S5][0] == 0u64;
                c.g[S0][0] = lw(c, r32(c, V1));
                if t {
                    break 'b_1e1bdc;
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x001e1bdc);
                    c.g[A0][0] = c.g[S5][0];
                    if call_ptr(c, 0x00317e98, 0x001e1bdc).is_err() {
                        return;
                    }
                    break 'b_1e1bdc;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
            break 'b_1e1be0;
        }
    }
    c.cycles += 11;
    frame.restore(c, S0, 0x80);
    frame.restore(c, S1, 0x70);
    frame.restore(c, S2, 0x60);
    frame.restore(c, S3, 0x50);
    frame.restore(c, S4, 0x40);
    frame.restore(c, S5, 0x30);
    frame.restore(c, S6, 0x20);
    frame.restore(c, S7, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_7(c, SP, 0x90));
    c.pc = t;
    return;
}

/// `OVScreenMessageCenter *OVScreenMessageCenter_ctor(void *self, void *owner,
/// int arg)` (0x1e3c00): build a message-center screen over the base screen
/// constructor: install both vtables, clear the eight 5-word rows at +0x9c
/// and the counters, look up three string hashes in the owner's engine
/// dictionary, then record the "message center shown" flag.
pub fn ov_screen_message_center_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 17, 0x10);
        let this64 = abi::arg64(c, 0);
        frame.save(c, 16, 0x20);
        frame.save_ra(c, 0);
        let this = this64 as u32;
        c.g[S1][0] = this64;
        c.g[S0][0] = abi::arg64(c, 2);
        callee::F_0039E2A0.call(c, 0x001e_3c1c)?;

        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x98), c.g[S0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4e08));
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(8), c.g[V1][0]);
        c.g[A0][0] = 0x004a_0000;
        c.sw(this.wrapping_add(0x4c), 0);
        c.g[V0][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4318));
        c.g[A1][0] = sx(this.wrapping_add(0x11c));
        c.g[A2][0] = 7;
        c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(8), c.g[V0][0]);
        c.sh(this.wrapping_add(0x58), c.g[V1][0]);

        // Eight rows of five words, at +0x9c, +0xbc, +0xdc, +0xfc and +0x11c.
        loop {
            c.cycles += 8;
            c.sw((c.g[A1][0] as u32).wrapping_sub(0x80), 0);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
            c.sw((c.g[A1][0] as u32).wrapping_sub(0x60), 0);
            c.sw((c.g[A1][0] as u32).wrapping_sub(0x40), 0);
            c.sw((c.g[A1][0] as u32).wrapping_sub(0x20), 0);
            c.sw(c.g[A1][0] as u32, 0);
            let more = (c.g[A2][0] as i64) >= 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001e_3c58);
            }
        }

        c.cycles += 36;
        c.sw(this.wrapping_add(0x13c), 0);
        c.g[A0][0] = 0x004a_0000;
        c.sw(this.wrapping_add(0x140), 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20d8));
        c.sw(this.wrapping_add(0x144), 0);
        c.sw(this.wrapping_add(0x14c), 0);
        c.sw(this.wrapping_add(0x150), 0);
        c.sw(this.wrapping_add(0x154), 0);
        c.sw(this.wrapping_add(0x158), 0);
        c.sw(this.wrapping_add(0x15c), 0);
        c.sw(this.wrapping_add(0x160), 0);

        // The three keys: hash the name, intern it in the engine dictionary.
        c.g[S0][0] = c.lw(this.wrapping_add(0x10));
        callee::STR_HASH_3.call(c, 0x001e_3cac)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
        c.g[A1][0] = c.g[V0][0];
        callee::STR_INTERN.call(c, 0x001e_3cb8)?;
        c.g[A0][0] = 0x0047_0000;
        c.g[S0][0] = c.lw(this.wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2098));
        c.sw(this.wrapping_add(0x164), c.g[V0][0]);
        callee::STR_HASH_3.call(c, 0x001e_3ccc)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
        c.g[A1][0] = c.g[V0][0];
        callee::STR_INTERN.call(c, 0x001e_3cd8)?;
        c.g[A0][0] = 0x0047_0000;
        c.g[S0][0] = c.lw(this.wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2088));
        c.sw(this.wrapping_add(0x168), c.g[V0][0]);
        callee::STR_HASH_3.call(c, 0x001e_3cec)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
        c.g[A1][0] = c.g[V0][0];
        callee::STR_INTERN.call(c, 0x001e_3cf8)?;

        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1024));
        c.g[V1][0] = !(0 | c.g[V1][0]);
        let unset = c.g[V1][0] == 0;
        c.sw(this.wrapping_add(0x16c), c.g[V0][0]);
        if unset {
            // Already shown: 2 means "no".
            c.cycles += 3;
            c.g[V0][0] = 2;
            c.sw(this.wrapping_add(0x18), c.g[V0][0]);
            c.g[V0][0] = c.g[S1][0];
        } else {
            c.cycles += 4;
            c.g[A0][0] = 7;
            callee::MESSAGE_CENTER_IS_FLAG_SET_2.call(c, 0x001e_3d10)?;
            let first = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
            if first {
                c.cycles += 1;
                c.sw(this.wrapping_add(0x18), c.g[V0][0]);
            } else {
                c.cycles += 2;
                c.sw(this.wrapping_add(0x18), 0);
            }
            c.cycles += 5;
            c.g[A0][0] = 7;
            c.g[A1][0] = 1;
            callee::MESSAGE_CENTER_SET_FLAG_3.call(c, 0x001e_3d30)?;
            c.g[V0][0] = c.g[S1][0];
        }

        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
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
    use crate::difftest::compare;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
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
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::RA as RET;
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_4;
    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4;
    pub(crate) use crate::game::be::bedbprofile::tests::strict as strict_2;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_9;
    pub(crate) use crate::game::bx::bxstring2::tests::be_library;
    pub(crate) use crate::game::bx::bxstring2::tests::f;
    pub(crate) use crate::game::bx::bxstring2::tests::run as run_3;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE_5;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::RM as RM_2;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2 as W_3;
    pub(crate) use crate::game::dirtysock::tags2::tests::alloc_ready;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::dirtysock::tags2::tests::bb;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::belib_3 as belib_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::clear as clear_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::common;
    pub(crate) use crate::game::dirtysock::tags2::tests::dummy_world as dummy_world_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::heap_block_2 as heap_block;
    pub(crate) use crate::game::dirtysock::tags2::tests::load;
    pub(crate) use crate::game::dirtysock::tags2::tests::world_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::BE as BE_12;
    pub(crate) use crate::game::dirtysock::tags2::tests::SEEDS_7 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_11;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_4 as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_9 as be_library_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::call_machine;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear_4 as clear_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_12 as globals_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_6 as globals_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_10 as heap_init_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12 as heap_init_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::init_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_10 as pool_allocator_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_3 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::run;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_6 as slot_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_11;
    pub(crate) use crate::game::fe::feasyncfile::tests::w16;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::world;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_8 as BE_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::EMPTY;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_15 as SEEDS_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::VT;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_3 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_6 as W_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_5;
    pub(crate) use crate::game::fe::fepopup::tests::base_14;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all_2;
    pub(crate) use crate::game::fe::fepopup::tests::widget_setup;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_3;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_6;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_13;
    pub(crate) use crate::game::fe::festaterewards::tests::globals_3 as globals_5;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_init_3;
    pub(crate) use crate::game::fe::festaterewards::tests::hook;
    pub(crate) use crate::game::fe::festaterewards::tests::pool_4;
    pub(crate) use crate::game::fe::festaterewards::tests::slot_2 as slot_5;
    pub(crate) use crate::game::fe::festaterewards::tests::strict as strict_3;
    pub(crate) use crate::game::fe::festaterewards::tests::w as w_6;
    pub(crate) use crate::game::fe::festaterewards::tests::w as w_12;
    pub(crate) use crate::game::fe::festaterewards::tests::w16 as w16_2;
    pub(crate) use crate::game::fe::festaterewards::tests::BE_7 as BE_11;
    pub(crate) use crate::game::fe::festaterewards::tests::HOOKS;
    pub(crate) use crate::game::fe::festaterewards::tests::KEEP_F0;
    pub(crate) use crate::game::fe::festaterewards::tests::W as W_6;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_5;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_7;
    pub(crate) use crate::game::main::ssxappdtor::tests::alloc_setup;
    pub(crate) use crate::game::main::ssxappdtor::tests::dtor_case;
    pub(crate) use crate::game::main::ssxappdtor::tests::MEM_MUTEX;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_LIBRARY;
    pub(crate) use crate::game::object::railmodifier::tests::rng;
    pub(crate) use crate::game::object::railmodifier::tests::SEEDS_7 as SEEDS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::belib_2 as belib;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w as w_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_5;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_12;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7;
    pub(crate) use crate::game::sound::soundsys::tests::load_2;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool_6;
    pub(crate) use crate::game::sound::soundsys::tests::w as w_10;
    pub(crate) use crate::game::sound::soundsys::tests::BE as BE_9;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn message_center_is_message_of_type_matches() {
        check_lohi(
            ssx3_ee::s04::f_001e2d58,
            message_center_is_message_of_type,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c10_000b ^ seed as u64);
                scramble(c, &mut r);
                let index = (seed % 4) as u32;
                c.g[A1][0] = index as u64;
                let entry = 0x004c_6c08u32.wrapping_add(index.wrapping_mul(0x18));
                let p = 0x10u32 + (seed % 3) * 4;
                c.bus.write32(entry + 4, p);
                let q = p.wrapping_mul(0x14).wrapping_add(0x0044_1630);
                let value = r.next_u32();
                c.bus.write32(q + 0xc, value);
                c.g[A0][0] = if seed % 2 == 0 {
                    sx(value)
                } else {
                    sx(value ^ 0xdead_beef)
                };
            },
        );
    }

    #[test]
    fn fe_locale_set_flag27_matches() {
        crate::difftest::check(
            ee::s04::f_001e1090__generated,
            fe_locale_set_flag27,
            200,
            &|c, seed| {
                let mut r = Rng(0x1e10_9001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA, r.next_u32());
                c.g[A1][0] = if seed % 3 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }

    #[test]
    fn ov_state_view_popup_transition_in_matches() {
        crate::difftest::check(
            ee::s04::f_001e52d0__generated,
            ov_state_view_popup_transition_in,
            40,
            &|c, seed| {
                machine(c, seed, 0x1e52_d0, 2);
                be_library(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_net_messenger_transition_in_matches() {
        covered(
            ssx3_ee::s04::f_001e0fc0__generated,
            c_fe_state_net_messenger_transition_in,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0xa3, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                if seed % 2 == 0 {
                    let vt = vtable(c, &mut r, &[0x100]);
                    c.bus.write32(DATA + 8, vt);
                }
            },
        );
    }

    /// The behaviour library interface slot: `BELibrary_GetInterface(1)`
    /// reads `D_00530 8A8[index]`, whose `+4` is a vtable with a fake method.
    pub(crate) fn be_interface(c: &mut Cpu, r: &mut Rng) {
        globals_2(c);
        c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
        let iface = DATA + 0x400;
        c.bus.write32(0x0053_08a8, iface);
        let vt = vtable(c, r, &[0x8]);
        c.bus.write32(iface + 4, vt);
    }

    // -------------------------------------------------------- message center

    #[test]
    fn message_center_get_message_id_matches() {
        check(
            ee::s04::f_001e3620__generated,
            message_center_get_message_id,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1e3620, 1);
                clear(c, DATA, 0x400);
                be_interface(c, &mut r);
                c.g[A0][0] = (seed % 6) as u64;
            },
        );
    }

    #[test]
    fn message_center_get_message_variant_matches() {
        check(
            ee::s04::f_001e3650__generated,
            message_center_get_message_variant,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1e3650, 1);
                clear(c, DATA, 0x400);
                be_interface(c, &mut r);
                c.g[A0][0] = (seed % 6) as u64;
            },
        );
    }

    // ------------------------------------------------------------- screens

    #[test]
    fn ov_screen_peak_challenge_results_on_activate_matches() {
        check(
            ee::s04::f_001e8ed0__generated,
            ov_screen_peak_challenge_results_on_activate,
            60,
            &|c, seed| {
                machine(c, seed, 0x1e8ed0, 2);
                globals_2(c);
                clear(c, DATA, 0x400);
                c.g[A0][0] = DATA as u64;
                // The audio singleton: null (the dispatcher's jump table is
                // empty on test data) or a dummy.
                c.bus.write32(
                    GAME_GP.wrapping_add(0x410),
                    if seed % 2 == 0 { 0 } else { DATA + 0x800 },
                );
            },
        );
    }

    #[test]
    fn rule_select_member_dtor_matches() {
        dtor_case(
            ee::s04::f_001df440__generated,
            rule_select_member_dtor,
            0x1df440,
        );
    }

    #[test]
    fn fe_ui_sound_v01_matches() {
        dtor_case(ee::s04::f_001e0740__generated, fe_ui_sound_v01, 0x1e0740);
    }

    #[test]
    fn ov_ui_sound_v01_matches() {
        dtor_case(ee::s04::f_001e0780__generated, ov_ui_sound_v01, 0x1e0780);
    }

    #[test]
    fn ui_sound_base_dtor_matches() {
        dtor_case(ee::s04::f_001e1048__generated, ui_sound_base_dtor, 0x1e1048);
    }

    #[test]
    fn ui_custom_base_dtor_matches() {
        dtor_case(
            ee::s04::f_001e10b8__generated,
            ui_custom_base_dtor,
            0x1e10b8,
        );
    }

    #[test]
    fn ui_input_base_dtor_matches() {
        dtor_case(ee::s04::f_001e1108__generated, ui_input_base_dtor, 0x1e1108);
    }

    #[test]
    fn ov_message_center_hide_widget_matches() {
        check(
            ssx3_ee::s04::f_001e4840__generated,
            ov_message_center_hide_widget,
            SEEDS,
            &|c, seed| {
                let w = widget_setup(c, seed, 0xe4840);
                c.g[A0][0] = sx(DATA + 0x300);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(w) };
            },
        );
    }

    #[test]
    fn ov_message_center_show_widget_matches() {
        check(
            ssx3_ee::s04::f_001e4878__generated,
            ov_message_center_show_widget,
            SEEDS,
            &|c, seed| {
                let w = widget_setup(c, seed, 0xe4878);
                c.g[A0][0] = sx(DATA + 0x300);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(w) };
            },
        );
    }

    #[test]
    fn message_center_is_flag_set_matches() {
        check(
            ssx3_ee::s04::f_001e3a30__generated,
            message_center_is_flag_set,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x1e3a30, 0);
                be_library_2(c);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x1400);
            },
        );
    }

    /// A dummy world at `DATA+0x1000` (the `gp-0x848` global), its race
    /// manager at `+0x84 -> +0x8c -> +0x4`, so callees that walk it stop, and
    /// the BE library singleton (a zeroed object at `DATA+0x6000`): without it
    /// `BELibrary_GetInterface` builds one through `EA_MemAllocNamed` and the
    /// file layer spins in `EAFile_AllocRequest`.
    pub(crate) fn dummy_world(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x1000);
        c.bus.write32(DATA + 0x1084, DATA + 0x2000);
        c.bus.write32(DATA + 0x208c, DATA + 0x3000);
        c.bus.write32(DATA + 0x3004, DATA + 0x3400);
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6000);
    }

    #[test]
    fn ov_screen_message_center_on_child_event_matches() {
        check(
            ee::s04::f_001e4610__generated,
            ov_screen_message_center_on_child_event,
            24,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x1e46_10, 3);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() & 0xffff);
                c.g[A2][0] = sx(if seed % 2 == 0 {
                    0x14
                } else {
                    r.next_u32() & 0xff
                });
                dummy_world(c);
                c.bus.write32(this + 0x10, DATA + 0x400);
            },
        );
    }

    // ------------------------------------------------------- message center

    #[test]
    fn message_center_set_flag_matches() {
        check(
            ee::s04::f_001e3a78__generated,
            message_center_set_flag,
            24,
            &|c, seed| {
                let mut r = base(c, seed, 0x1e3a);
                c.bus.write32(gp_at(-0x1f04), DATA + 0x3_e000);
                c.bus.write32(DATA + 0x3_e000, 0);
                c.g[A0][0] = sx(r.next_u32());
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    // ------------------------------------------------------ overlay screens

    #[test]
    fn ov_screen_finish_ov_on_widget_action_matches() {
        check(
            ee::s04::f_001e8160__generated,
            ov_screen_finish_ov_on_widget_action,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x1e81);
                let s = DATA;
                let world = DATA + 0x1000;
                let module = DATA + 0x2000;
                c.bus
                    .write32(gp_at(-0x848), if seed % 5 == 0 { 0 } else { module });
                c.bus.write32(module + 0x84, world);
                c.bus.write32(world + 0x48, DATA + 0x3000);
                c.bus.write32(s + 0xa0, if seed % 3 == 0 { 0 } else { 1 });
                c.g[A0][0] = sx(s);
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x4000) };
                c.g[A2][0] = match seed % 4 {
                    0 => 0,
                    1 => 5,
                    _ => (seed % 9) as u64,
                };
            },
        );
    }

    #[test]
    fn ov_screen_finish_ov2_on_widget_action_matches() {
        check(
            ee::s04::f_001e8880__generated,
            ov_screen_finish_ov2_on_widget_action,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x1e88);
                let s = DATA;
                let world = DATA + 0x1000;
                let module = DATA + 0x2000;
                c.bus
                    .write32(gp_at(-0x848), if seed % 5 == 0 { 0 } else { module });
                c.bus.write32(module + 0x84, world);
                c.bus.write32(world + 0x48, DATA + 0x3000);
                c.bus.write32(s + 0xa0, if seed % 3 == 0 { 0 } else { 1 });
                c.g[A0][0] = sx(s);
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x4000) };
                c.g[A2][0] = match seed % 4 {
                    0 => 0,
                    1 => 5,
                    _ => (seed % 9) as u64,
                };
            },
        );
    }

    #[test]
    fn c_fe_state_trophy_room_dtor_matches() {
        check(
            ee::s04::f_001e0210__generated,
            c_fe_state_trophy_room_dtor,
            SEEDS_2,
            &|c, seed| {
                let _ = common(c, seed, 0x1e0210, 0);
                alloc_ready(c);
                let this = heap_block(c, 0x800);
                clear_2(c, this, 0x200);
                // The trophy release walks five image slots and hands each
                // handle to BoltFile_Release; a large refcount keeps it on the
                // early-out path.
                c.bus.write32(DATA + 0x8000 + 0x7c, DATA + 0x3000);
                c.bus.write32(DATA + 0x4ca4, 100);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn message_center_get_unread_count_matches() {
        check(
            ee::s04::f_001e3408__generated,
            message_center_get_unread_count,
            SEEDS_2,
            &|c, seed| {
                let _ = common(c, seed, 0x1e3408, 0);
                c.bus.write32(gp_at(-0x1efc), 0);
                c.bus.write32(0x0053_08a8, 0);
            },
        );
    }

    /// The `cBXString` static-empty-string marker (the value the string dtor
    /// compares against before touching the heap).
    pub(crate) const EMPTY_STRING: u32 = 0x0048_dcd4;

    /// Scrambled registers, the fake hooks, a game-like `gp_2`, and the BELibrary
    /// singleton replaced by a zeroed dummy (so `cBE_getBE` takes its cheap
    /// path).
    pub(crate) fn setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x1000);
        c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
        r
    }

    /// The BELibrary singleton with interface 0's vtable slot 8 pointing at a
    /// fake method.
    pub(crate) fn belibrary(c: &mut Cpu, r: &mut Rng) {
        let vt = vtable(c, r, &[8]);
        c.bus.write32(0x0053_08a8, DATA + 0x1100);
        c.bus.write32(DATA + 0x1104, vt);
    }

    /// One test per reward-gallery destructor.
    macro_rules! dtor_test {
        ($tname:ident, $addr:expr, $gen:expr, $hand:expr) => {
            #[test]
            fn $tname() {
                check($gen, $hand, 100, &|c, seed| {
                    let _r = setup(c, seed, $addr);
                    c.g[A0][0] = DATA as u64;
                    c.g[A1][0] = 0;
                    c.bus.write32(DATA + 0x78, 0);
                    c.bus.write32(DATA + 0x8ec, 0xffff_ffff);
                });
            }
        };
    }
    pub(crate) use dtor_test;

    #[test]
    fn cfe_state_lobby_menu_dtor_matches() {
        check(
            ssx3_ee::s04::f_001df7c8__generated,
            cfe_state_lobby_menu_dtor,
            120,
            &|c, seed| {
                let _r = setup(c, seed, 0x1df7c8);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.bus.write32(DATA + 0xb80, EMPTY_STRING);
                c.bus.write32(DATA + 0x76c, EMPTY_STRING);
            },
        );
    }

    dtor_test!(
        cfe_state_reward_gallery_dtor_matches,
        0x1dfa10,
        ssx3_ee::s04::f_001dfa10__generated,
        cfe_state_reward_gallery_dtor
    );

    dtor_test!(
        cfe_state_reward_gallery_posters_dtor_matches,
        0x1dfb28,
        ssx3_ee::s04::f_001dfb28__generated,
        cfe_state_reward_gallery_posters_dtor
    );

    dtor_test!(
        cfe_state_reward_gallery_trading_cards_dtor_matches,
        0x1dfbf8,
        ssx3_ee::s04::f_001dfbf8__generated,
        cfe_state_reward_gallery_trading_cards_dtor
    );

    dtor_test!(
        cfe_state_reward_gallery_concept_arts_dtor_matches,
        0x1dfcc8,
        ssx3_ee::s04::f_001dfcc8__generated,
        cfe_state_reward_gallery_concept_arts_dtor
    );

    dtor_test!(
        cfe_state_reward_gallery_videos_dtor_matches,
        0x1dfd98,
        ssx3_ee::s04::f_001dfd98__generated,
        cfe_state_reward_gallery_videos_dtor
    );

    dtor_test!(
        cfe_state_reward_gallery_toys_dtor_matches,
        0x1dfe68,
        ssx3_ee::s04::f_001dfe68__generated,
        cfe_state_reward_gallery_toys_dtor
    );

    dtor_test!(
        cfe_state_reward_gallery_cheat_chars_dtor_matches,
        0x1dff38,
        ssx3_ee::s04::f_001dff38__generated,
        cfe_state_reward_gallery_cheat_chars_dtor
    );

    #[test]
    fn cfe_popup_select_multiplayer_mode_init_matches() {
        check(
            ssx3_ee::s04::f_001e02f8__generated,
            cfe_popup_select_multiplayer_mode_init,
            100,
            &|c, seed| {
                let _r = setup(c, seed, 0x1e02f8);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.bus.write32(DATA + 0x10, 0);
            },
        );
    }

    #[test]
    fn cfe_state_challenge_popup_dtor_matches() {
        check(
            ssx3_ee::s04::f_001e09a0__generated,
            cfe_state_challenge_popup_dtor,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1e09a0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.bus.write32(DATA + 0x20, DATA + 0x900);
                let vt = vtable(c, &mut r, &[0x100]);
                c.bus.write32(DATA + 0x908, vt);
            },
        );
    }

    #[test]
    fn message_center_find_message_matches() {
        check(
            ssx3_ee::s04::f_001e3388__generated,
            message_center_find_message,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1e3388);
                belibrary(c, &mut r);
                c.g[A0][0] = (r.next_u32() % 8) as u64;
            },
        );
    }

    #[test]
    fn message_center_reset_new_player_flag_matches() {
        check(
            ssx3_ee::s04::f_001e3510__generated,
            message_center_reset_new_player_flag,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1e3510);
                belibrary(c, &mut r);
                c.bus.write8(0x0053_5c11, (seed % 2) as u8);
            },
        );
    }

    #[test]
    fn fe_popup_screen_pos_v23_matches() {
        check(
            ee::s04::f_001df028__generated,
            fe_popup_screen_pos_v23,
            SEEDS_3,
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
    pub(crate) fn put_item(c: &mut Cpu, seed: u32, r: &mut Rng) -> u32 {
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
            SEEDS_3,
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
            SEEDS_3,
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
            SEEDS_3,
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
            SEEDS_3,
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
            SEEDS_3,
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
            SEEDS_3,
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
            SEEDS_3,
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
            SEEDS_3,
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
    pub(crate) fn put_menu(c: &mut Cpu, seed: u32, r: &mut Rng) {
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(DATA + 0x18, r.next_u32());
        let picked = gp_3(c, -0x9f4);
        let row_global = gp_3(c, -0x9f0);
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
            SEEDS_3,
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
            SEEDS_3,
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
            SEEDS_3,
            &|c, seed| {
                let mut r = rng(0x001e_7b88, seed);
                scramble(c, &mut r);
                put_menu(c, seed, &mut r);
            },
        );
    }

    /// The event (`a1`) of a finish-screen `OnEvent` handler.
    pub(crate) fn put_event(c: &mut Cpu, seed: u32, r: &mut Rng) {
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
            SEEDS_3,
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
            SEEDS_3,
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
            SEEDS_3,
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
            SEEDS_3,
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
            SEEDS_3,
            &|c, seed| {
                let mut r = rng(0x001e_8fc8, seed);
                scramble(c, &mut r);
                put_menu(c, seed, &mut r);
            },
        );
    }

    /// An inbox with `count` entries whose ids come from `ids`.
    pub(crate) fn inbox(c: &mut Cpu, at: u32, count: u8, ids: &[u32]) {
        clear_3(c, at, 0x40);
        c.bus.write8(at + 0xcc, count);
        for (i, &id) in ids.iter().enumerate() {
            w(c, at + 8 * i as u32, id);
        }
    }

    // ------------------------------------------------------------------
    // Message center.

    #[test]
    fn message_center_post_once_3c_matches() {
        check(
            ee::s04::f_001e2518__generated,
            message_center_post_once_3c,
            SEEDS_4,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x1e25_18);
                let world = DATA + 0x1000;
                let man = DATA + 0x2000;
                let list = DATA + 0x3000;
                clear_3(c, world, 0x100);
                clear_3(c, man, 0x80);
                clear_3(c, list, 0x40);
                w(c, gp_at(-0x848), world as u32);
                w(c, world + 0x84, man as u32);
                w(c, man + 0x28, list as u32);
                w(
                    c,
                    list,
                    match seed % 4 {
                        0 => 0,
                        1 => 3,
                        2 => 12,
                        _ => r.next_u32(),
                    },
                );
            },
        );
    }

    #[test]
    fn message_center_post_once_38_matches() {
        check(
            ee::s04::f_001e2578__generated,
            message_center_post_once_38,
            SEEDS_4,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x1e25_78);
                let world = DATA + 0x1000;
                let man = DATA + 0x2000;
                let list = DATA + 0x3000;
                clear_3(c, world, 0x100);
                clear_3(c, man, 0x80);
                clear_3(c, list, 0x40);
                w(c, gp_at(-0x848), world as u32);
                w(c, world + 0x84, man as u32);
                w(c, man + 0x28, list as u32);
                w(
                    c,
                    list,
                    match seed % 4 {
                        0 => 0,
                        1 => 3,
                        2 => 12,
                        _ => r.next_u32(),
                    },
                );
            },
        );
    }

    #[test]
    fn message_center_remove_message_by_id_matches() {
        check(
            ee::s04::f_001e3268__generated,
            message_center_remove_message_by_id,
            SEEDS_4,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x1e32_68);
                // `MessageCenter_GetInbox` walks the BELibrary singleton.
                let lib = DATA + 0x1000;
                clear_3(c, lib, 0x100);
                w(c, gp_at(-0x1f04), lib as u32);
                let box_ = DATA + 0x2000;
                let id = r.next_u32();
                let n = (seed % 5) as u8;
                inbox(
                    c,
                    box_,
                    n,
                    &[
                        r.next_u32(),
                        if seed % 3 == 0 { id } else { r.next_u32() },
                        r.next_u32(),
                        r.next_u32(),
                    ],
                );
                // The inbox pointer the (faked) `MessageCenter_GetInbox` returns.
                w(c, LOG_RESULTS, box_);
                c.pc = 0x001e_3268;
                c.g[A0][0] = id as u64;
            },
        );
    }

    #[test]
    fn cov_state_view_popup_init_matches() {
        check(
            ee::s04::f_001e5248__generated,
            cov_state_view_popup_init,
            6,
            &|c, seed| {
                init_setup(c, seed, 0x001e_5248);
            },
        );
    }

    #[test]
    fn cov_state_view_popup_set_subject_matches() {
        check(
            ee::s04::f_001e55d0__generated,
            cov_state_view_popup_set_subject,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x001e_55d0, 2);
                be_library_3(c);
                world(c);
                c.g[A0][0] = DATA as u64;
                let idx = (seed % 3) as u64;
                c.g[A1][0] = idx;
                let entry = 0x004c_6c08u32 + (idx as u32 * idx as u32) * 0x18;
                c.bus.write32(entry + 8, (seed % 2) as u32);
                c.bus.write32(entry + 0x10, r.next_u32());
                c.bus
                    .write32(DATA + 0x54, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
                c.bus.write32(DATA + 0x2000 + 0xb0, 0);
                c.bus.write32(DATA + 0x2000 + 0xb4, 0);
            },
        );
    }

    #[test]
    fn c_fe_state_challenge_received_popup_dtor_matches() {
        check(
            ee::s04::f_001e0a50__generated,
            c_fe_state_challenge_received_popup_dtor,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x1e0a50, 0);
                globals_3(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { 1 };
                w(c, OBJ + 0x20, OBJ2);
                w(c, OBJ2 + 8, VT);
                slot_2(c, VT, 0x100, 0, fakes::FAKE);
                w(c, OBJ + 0x6d0, 0x0048_dcd4);
            },
        );
    }

    #[test]
    fn c_fe_state_net_create_room_popup_dtor_matches() {
        check(
            ee::s04::f_001df878__generated,
            c_fe_state_net_create_room_popup_dtor,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x1df878, 0);
                globals_3(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { 1 };
                for off in [0x6e4u32, 0x6e0, 0x6dc] {
                    w(c, OBJ + off, 0x0048_dcd4);
                }
            },
        );
    }

    #[test]
    fn c_fe_state_create_step1_dtor_matches() {
        check(
            ee::s04::f_001e0cf8__generated,
            c_fe_state_create_step1_dtor,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x1e0cf8, 0);
                globals_3(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { 1 };
                for off in [0x6dcu32, 0x6d8, 0x6d4] {
                    w(c, OBJ + off, 0x0048_dcd4);
                }
            },
        );
    }

    #[test]
    fn message_center_on_bc_finish_matches() {
        check(
            ee::s04::f_001e25d8__generated,
            message_center_on_bc_finish,
            60,
            &|c, seed| {
                base_3(c, seed, 0x1e25_d8);
                let p = DATA + 0x2000;
                c.bus.write32(RM + 0x28, p);
                c.bus.write32(p, [0u32, 5, 15][seed as usize % 3]);
                c.g[A0][0] = (6 + seed % 3) as u64;
                c.g[A1][0] = sx(DATA + 0x3000);
                c.g[A2][0] = sx(DATA + 0x3400);
            },
        );
    }

    /// Run a guest helper during setup; its `v0`.
    pub(crate) fn call0(c: &mut Cpu, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3][i]][0] = *a;
        }
        c.g[RA][0] = crate::difftest::RA as u64;
        f(c);
        c.g[V0][0]
    }

    #[test]
    fn cov_state_view_popup_ctor_matches() {
        check(
            ee::s04::f_001e51d0__generated,
            cov_state_view_popup_ctor,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x1e_51d0, 1);
                belib(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (DATA + 0x2200) as u64;
                c.g[A2][0] = (DATA + 0x2400) as u64;
                c.g[A3][0] = (seed % 7) as u64;
                c.g[T0][0] = (seed % 5) as u64;
                c.pc = 0x001e_51d0;
            },
        );
    }

    #[test]
    fn ov_screen_freestyle_standings_handle_input_matches() {
        check(
            ee::s04::f_001e65b8__generated,
            ov_screen_freestyle_standings_handle_input,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x1e_65b8, 3);
                belib(c);
                world_2(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (DATA + 0x2400) as u64;
                bb(c, DATA + 0x2404, (seed % 64) as u8);
                w_2(c, DATA + 0x2000 + 0x40, DATA + 0x2600);
                // Give the screen one object whose name hash is what the
                // function asks for: the search returns it when the list is
                // walked (the hash itself comes from the name table below).
                w_2(c, DATA + 0x2600 + 4, DATA + 0x2800);
                w_2(c, DATA + 0x2800 + 4, DATA + 0x2800);
                c.pc = 0x001e_65b8;
            },
        );
    }

    #[test]
    fn ov_screen_final_standings_on_widget_created_matches() {
        check(
            ee::s04::f_001e7558__generated,
            ov_screen_final_standings_on_widget_created,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x1e_7558, 4);
                belib(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (DATA + 0x2400) as u64;
                let h = call0(c, ee::s10::f_00317670, &[0x0046_e510]);
                w_2(c, DATA + 0x2400 + 0x38, h as u32);
                bb(c, 0x0053_5c11, (seed % 2) as u8);
                c.pc = 0x001e_7558;
            },
        );
    }

    #[test]
    fn c_fe_popup_select_multiplayer_mode_on_widget_action_matches() {
        check(
            ee::s04::f_001e03f8__generated,
            c_fe_popup_select_multiplayer_mode_on_widget_action,
            SEEDS_6,
            &|c, seed| {
                machine(c, seed, 0x1e03f8, 3);
                be_library_4(c);
                world_3(c);
                heap_init_2(c);
                let this = DATA;
                let widget = DATA + 0x400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = widget as u64;
                c.g[A2][0] = [5, 6, 7][(seed % 3) as usize] as u64;
                w_3(c, this + 0x10, DATA + 0x600);
                w_3(c, widget + 0x18, (seed % 2) as u32);
            },
        );
    }

    pub(crate) fn wb(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }

    #[test]
    fn message_center_is_new_player_flag_clear_matches() {
        check(
            ee::s04::f_001e3570__generated,
            message_center_is_new_player_flag_clear,
            80,
            &|c, seed| {
                let _ = base_4(c, seed, 0x1e35, 3);
                // The interface singleton the front-end reads at 0x535bc8.
                let flag = 0x0053_5bc8;
                wb(c, flag + 0x49, (seed % 3 == 0) as u8);
                wb(
                    c,
                    flag + 0x4a,
                    match seed % 4 {
                        0 => 6,
                        1 => 9,
                        _ => (seed % 0x20) as u8,
                    },
                );
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn ov_screen_message_center_open_view_popup_matches() {
        check(
            ee::s04::f_001e50c8__generated,
            ov_screen_message_center_open_view_popup,
            50,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x1e50, 3);
                let s = DATA;
                w_4(c, s + 0x10, DATA + 0x200);
                if seed % 4 == 0 {
                    w_4(c, s + 0x17c, 0);
                } else {
                    w_4(c, s + 0x17c, DATA + 0x500);
                    w_4(c, DATA + 0x5d0, DATA + 0x520);
                    w_4(c, DATA + 0x530, DATA + 0x540);
                }
                fill(c, &mut r, DATA + 0x500, 3);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(DATA + 0x300);
                c.g[A2][0] = sx(DATA + 0x400);
                c.g[A3][0] = sx(DATA + 0x600);
            },
        );
    }

    #[test]
    fn c_ov_state_view_popup_set_body_matches() {
        check(
            ee::s04::f_001e5780__generated,
            c_ov_state_view_popup_set_body,
            80,
            &|c, seed| {
                let _ = base_4(c, seed, 0x1e57, 2);
                let s = DATA;
                let d = DATA + 0x200;
                // `d + 0x4c6c08` is the view popup's data record; +8 is the body
                // index (0/1 select it, >= 2 leaves the view, negative skips).
                let at = d.wrapping_add(0x4c_6c08);
                w_4(
                    c,
                    at + 8,
                    match seed % 4 {
                        0 => 0,
                        1 => 1,
                        2 => 2,
                        _ => 0xffff_ffff,
                    },
                );
                w_4(c, at + 0x14, seed);
                w_4(c, s + 0x58, DATA + 0x600);
                w_4(c, DATA + 0x608, DATA + 0x680);
                slot_3(c, DATA + 0x680, 0x48, FAKE);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(d);
            },
        );
    }

    // -----------------------------------------------------------------------
    // Message center, interrupt table, tags.

    #[test]
    fn ovscreenmessagecenter_delete_message_matches() {
        check(
            ee::s04::f_001e5148__generated,
            ovscreenmessagecenter_delete_message,
            SEEDS_7,
            &|c, seed| {
                base_5(c, seed, 0xe5148, 0);
                let s = DATA + 0x100;
                let mc = DATA + 0x800;
                c.g[A0][0] = sx(s);
                c.g[A1][0] = (seed % 6) as u64;
                c.bus.write32(s + 0x144, mc);
                c.bus.write8(mc + 0x95, (seed % 5) as u8);
                c.bus.write32(GAME_GP - 0x5f8, DATA + 0x900);
            },
        );
    }

    #[test]
    fn c_fe_popup_select_multiplayer_mode_on_widget_notify_matches() {
        check(
            ssx3_ee::s04::f_001e0470__generated,
            c_fe_popup_select_multiplayer_mode_on_widget_notify,
            SEEDS_8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1e0470, 2);
                be_library_5(c);
                world_4(c);
                let st = DATA + 0x100;
                let menu = DATA + 0x200;
                c.g[A0][0] = st as u64;
                c.g[A1][0] = menu as u64;
                c.g[A2][0] = (seed % 9) as u64;
                c.g[A3][0] = (r.next_u32() % 8) as u64;
            },
        );
    }

    #[test]
    fn message_center_trigger_groups_matches() {
        check(
            ssx3_ee::s04::f_001e14c0__generated,
            message_center_trigger_groups,
            SEEDS_8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1e14c0, 4);
                be_library_5(c);
                world_4(c);
                let st = DATA + 0x100;
                c.g[A0][0] = (seed % 6) as u64;
                let _ = st;
                // The trigger table at 0x441630, 0x14 bytes per group.
                const TABLE: u32 = 0x0044_1630;
                for i in 0..0x40 {
                    let kind = (r.next_u32() % 7) as u32;
                    w_5(c, TABLE + 0x14 * i, kind);
                }
            },
        );
    }

    #[test]
    fn cfe_state_bragging_rights_dtor_matches() {
        check(
            ee::s04::f_001df4e8__generated,
            cfe_state_bragging_rights_dtor,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1df4, 0);
                env(c);
                let this = DATA + 0x400;
                let vt = DATA + 0x2000;
                slot_4(c, vt, 8, EMPTY);
                for i in 0..12u32 {
                    w32(c, this + 0x6d0 + 0x34 * i + 0x30, vt);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn ov_screen_message_center_shutdown_matches() {
        check(
            ee::s04::f_001e3e30__generated,
            ov_screen_message_center_shutdown,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1e3e, 0);
                env(c);
                let this = DATA + 0x400;
                w32(c, this + 0x18, [0, 1, 2, 0x7fff_ffff][(seed % 4) as usize]);
                w32(c, gpat(-0x69c), 4);
                c.g[A0][0] = sx(this);
            },
        );
    }

    // ------------------------------------------------------------------
    // UI / lobby.

    #[test]
    fn ov_screen_message_center_on_activate_matches() {
        check(
            ee::s04::f_001e4168__generated,
            ov_screen_message_center_on_activate,
            60,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x1e4168, 3);
                let obj = DATA + 0x1000;
                let menu = DATA + 0x2000;
                let vt = DATA + 0x3000;
                c.g[A0][0] = obj as u64;
                w_6(c, obj + 0x18, (seed % 4) as u32);
                w_6(c, obj + 0x144, if seed % 7 == 0 { 0 } else { menu });
                w_6(c, menu + 0x8, vt);
                slot_5(c, vt, 0x38, 4, FAKE);
                w_6(c, obj + 0xa0, r.next_u32() % 8);
            },
        );
    }

    #[test]
    fn message_center_count_group_messages_matches() {
        check(
            ee::s04::f_001e2bb8__generated,
            message_center_count_group_messages,
            6,
            &|c, seed| {
                let _ = base_7(c, seed, 0x1e2bb8, 2);
                c.g[A0][0] = (seed % 2) as u64;
                c.g[A1][0] = ((seed / 2) % 2) as u64;
                for i in 0..61u32 {
                    let e = 0x0044_1630 + i * 0x14;
                    w_7(c, e, if i % 3 == 0 { 0 } else { 1 });
                    w_7(c, e + 8, (i + 1) & 0xffff);
                    w_7(c, e + 0x10, i % 2);
                }
            },
        );
    }

    // -------------------------------------------------- final standings init

    #[test]
    fn ov_screen_final_standings_init_matches() {
        check(
            ee::s04::f_001e6668__generated,
            ov_screen_final_standings_init,
            SEEDS_9,
            &|c, seed| {
                let _ = base_8(c, seed, 0x1e66_68, 6);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                w_8(c, obj + 0x10, obj + 0x400);
                w_8(c, obj + 0x400 + 4, obj + 0x500);
                w_8(c, obj + 0x500 + 0xc, 0);
                w_8(c, obj + 0x9c, 0);
            },
        );
    }

    #[test]
    fn message_center_remove_message_matches() {
        check(
            ee::s04::f_001e31b8__generated,
            message_center_remove_message,
            80,
            &|c, seed| {
                let mut r = base_9(c, seed, 0x1e31);
                let inbox = DATA + 0x1000;
                c.g[A0][0] = (seed % 5) as u64;
                c.bus.write32(DATA + 0x2000, inbox);
                fill(c, &mut r, inbox, 0x100);
                c.bus.write8(inbox + 0xcc, (seed % 4) as u8);
                c.bus.write32(inbox + 0xc8, r.next_u32());
            },
        );
    }

    // --------------------------------------------------------- message center

    #[test]
    fn message_center_set_peak_foreshadow_done_matches() {
        check(
            ee::s04::f_001e2370__generated,
            message_center_set_peak_foreshadow_done,
            SEEDS_10,
            &|c, seed| {
                base_10(c, seed, 0x1e23, 0);
                let game = W_4 + 0x1000;
                w_9(c, game + 0x28, DATA + 0x300);
                w_9(c, DATA + 0x300, (seed % 13) as u32);
                c.g[A0][0] = (seed % 4) as u64;
            },
        );
    }

    /// The lazily built BE singleton (`gp-0x1f04`) pointed at_6 a zeroed dummy,
    /// and the current interface index (`gp-0x1efc`) at_6 zero.
    pub(crate) fn be_library_7(c: &mut Cpu) {
        c.bus
            .write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
        c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
    }

    // -------------------------------------------------- FE state constructor.

    // ----------------------------------------------------------- Message centre.

    #[test]
    fn message_center_remove_messages_of_group_matches() {
        check(
            ee::s04::f_001e32c8__generated,
            message_center_remove_messages_of_group,
            100,
            &|c, seed| {
                machine(c, seed, 0x1e_32c8, 4);
                be_library_7(c);
                c.g[A0][0] = sx((seed % 8) as u32);
            },
        );
    }

    #[test]
    fn message_center_get_index_in_type_matches() {
        strict(
            ee::s04::f_001e2dc0__generated,
            message_center_get_index_in_type,
            40,
            &|c, seed| {
                let mut r = Rng(0x001e_2dc0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.bus.write32(0x004c_6c08 + 4, 0);
                c.bus.write32(0x0044_1630 + 4, 0);
                c.bus.write32(0x0044_1630 + 8, 1);
                c.bus.write32(0x0044_1630 + 0xc, r.next_u32());
                c.g[A0][0] = 0;
            },
        );
    }

    #[test]
    fn ov_screen_multiplayer_standings_init_matches() {
        strict(
            ee::s04::f_001e8f00__generated,
            ov_screen_multiplayer_standings_init,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1e8f, 8);
                let this = DATA;
                c.bus.write32(this + 0x10, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x38, r.next_u32());
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn ov_screen_finish_ov_update_matches() {
        check_all(
            ee::s04::f_001e8098__generated,
            ov_screen_finish_ov_update,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1e80, 3);
                game_globals(c);
                be_library_8(c, &mut r);
                c.bus.write32(GAME_GP - 0x578c, 0.05f32.to_bits());
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus.write32(this + 0x9c, r.f32(4.0));
                c.bus.write32(this + 0xa0, (seed % 2) as u32);
                c.bus.write32(RIDER + 0x480, (seed % 3 == 0) as u32);
                c.bus.write32(RMAN + 0x48, DATA + 0x15_0000);
            },
        );
    }

    #[test]
    fn ov_screen_finish_ov2_update_matches() {
        check_all(
            ee::s04::f_001e87b8__generated,
            ov_screen_finish_ov2_update,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1e87, 3);
                game_globals(c);
                be_library_8(c, &mut r);
                c.bus.write32(GAME_GP - 0x5788, 0.05f32.to_bits());
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus.write32(this + 0x9c, r.f32(4.0));
                c.bus.write32(this + 0xa0, (seed % 2) as u32);
                c.bus.write32(RIDER + 0x480, (seed % 3 == 0) as u32);
                c.bus.write32(RMAN + 0x48, DATA + 0x15_0000);
            },
        );
    }

    #[test]
    fn ov_screen_final_standings_handle_input_matches() {
        strict_2(
            ee::s04::f_001e7aa8__generated,
            ov_screen_final_standings_handle_input,
            80,
            &|c, seed| {
                let _r = base_11(c, seed, 0x1e7a);
                let hash = run_3(c, 0x0031_7670, ee::s10::f_00317670, &[0x0046_e510]) as u32;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = DATA as u64;
                c.bus.write32(DATA + 0x40, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0x38, DATA + 0x2010);
                c.bus.write32(DATA + 0x2000 + 0x3c, DATA + 0x2020);
                c.bus.write32(DATA + 0x2010, 1);
                c.bus.write32(DATA + 0x2020, DATA + 0x2100);
                c.bus.write32(
                    DATA + 0x2100 + 0x38,
                    if seed % 2 == 0 { hash } else { hash ^ 1 },
                );
                c.bus
                    .write8(DATA + 0x2100 + 0x95, if seed % 3 == 0 { 2 } else { 1 });
                c.bus.write8(DATA + 4, seed as u8);
                c.bus
                    .write32(0x0053_4b30, if seed % 4 == 0 { 1 } else { 0 });
                c.bus
                    .write32(gp_at(-0x204), if seed % 8 == 0 { DATA + 0x3000 } else { 0 });
                c.bus
                    .write32(DATA + 0x3000, if seed % 16 == 0 { 1 } else { 0 });
                c.bus.write32(DATA + 8, DATA + 0x3100);
                c.bus.write16(DATA + 0x3100 + 0xd8, 0);
                c.bus.write32(DATA + 0x3100 + 0xdc, FAKE);
            },
        );
    }

    #[test]
    fn messagecenter_get_list_index_matches() {
        check(
            ee::s04::f_001e3680__generated,
            messagecenter_get_list_index,
            8,
            &|c, seed| {
                let _ = base_12(c, seed, 0x1e3680, 2);
                clear_4(c, DATA, 0x400);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 5) as u64;
                w_10(
                    c,
                    GAME_GP.wrapping_sub(0x102c),
                    if seed % 4 == 0 { 0 } else { 1 },
                );
            },
        );
    }

    #[test]
    fn view_popup_on_widget_action_matches() {
        diff(
            ee::s04::f_001e5800__generated,
            c_ov_state_view_popup_on_widget_action,
            40,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x1e_5800);
                let id = (seed % 4) as u32;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = [5u64, 6, 7, 8][seed as usize % 4];
                w_11(c, DATA + 0x20, 0);
                w_11(c, DATA + 0x68, id);
                w_11(c, 0x004c_6c08 + id * 0x18 + 4, (seed % 3) as u32);
                w_11(
                    c,
                    0x0044_1630 + (seed % 3) * 0x14,
                    [4u32, 0, 2][seed as usize % 3],
                );
                w_11(c, 0x004c_6c08 + id * 0x18 + 8, r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_message_center_update_count_text_matches() {
        diff(
            ee::s04::f_001e4758__generated,
            ov_screen_message_center_update_count_text,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x1e_4758);
                c.g[A0][0] = sx(DATA);
                w_11(
                    c,
                    DATA + 0x13c,
                    if seed % 2 == 0 { 0 } else { DATA + 0x2000 },
                );
                w_11(c, DATA + 0x2000 + 0x68, (seed % 4) as u32);
                w_11(c, DATA + 0x2000 + 0x20, 0);
                w_11(c, DATA + 0x2000 + 0x8, r.next_u32());
            },
        );
    }

    // ------------------------------------------------------ message center

    #[test]
    fn message_center_post_peak_unlock_message_matches() {
        strict_3(
            ee::s04::f_001e2428__generated,
            message_center_post_peak_unlock_message,
            90,
            &|c, seed| {
                let _ = base_13(c, seed, 0x1e24, 6);
                let peak = DATA + 0x900;
                c.g[A0][0] = (seed % 4) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                // world->+0x84->+0x28 = peak, *peak = index
                w_12(c, W_6 + 0x1000 + 0x28, peak);
                w_12(c, peak, if seed % 3 == 0 { 0 } else { (seed % 12) as u32 });
            },
        );
    }

    pub(crate) const RANGE: u32 = DATA + 0x1_0000;

    pub(crate) const NAMES: u32 = DATA + 0x2_0000;

    #[test]
    fn message_center_pick_foreshadow_message_matches() {
        check_all_2(
            ssx3_ee::s04::f_001e1ad0__generated,
            message_center_pick_foreshadow_message,
            30,
            &|c, seed| {
                let mut r = base_14(c, seed, 0x1e1a_d0, 10);
                c.g[A0][0] = sx(RANGE);
                c.g[A1][0] = sx(NAMES);
                c.bus.write32(RANGE + 0x4, (seed % 4) as u32);
                c.bus.write32(RANGE + 0x8, (seed % 3) as u32);
                // The candidate table: 0x18-byte entries whose character pointer
                // at_7 +0xc is a string in the test area.
                for i in 0..6u32 {
                    c.bus
                        .write32(0x004c_6c08 + 0x18 * i + 0xc, NAMES + 0x40 * i);
                    c.bus
                        .write8(NAMES + 0x40 * i, b'a' + (r.next_u32() % 4) as u8);
                    c.bus.write8(NAMES + 0x40 * i + 1, 0);
                }
            },
        );
    }

    #[test]
    fn ov_screen_message_center_ctor_matches() {
        diff(
            ee::s04::f_001e3c00__generated,
            ov_screen_message_center_ctor,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1e_3c00, 4);
                belib_2(c);
                let this = DATA + 0x1_0000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x2_0000);
                c.g[A2][0] = r.next_u32() as u64;
                c.bus.write32(this + 0x10, DATA + 0x2_1000);
                c.bus
                    .write32(GAME_GP - 0x1024, if seed % 2 == 0 { u32::MAX } else { 0 });
                c.bus.write32(0x004a_2088, r.next_u32());
            },
        );
    }
}
