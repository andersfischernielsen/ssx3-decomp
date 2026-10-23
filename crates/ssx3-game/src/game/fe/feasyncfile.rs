//! `src/fe/feasyncfile.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::ps2f::FpuF32;
use crate::tick;
use ssx3_ee as ee;
use crate::abi::r32;
use crate::call::Flow;
use crate::call::sp;
use crate::events::back_edge_unwinds;
use callee::*;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::Unwound;
use crate::events::deliver_event;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::gp_2 as gp;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
pub(crate) use crate::game::movie::movieplayer::lw_at;
pub(crate) use crate::game::main::ssxappdtor::p;
pub(crate) use crate::game::main::ssxappdtor::vtable_dtor;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::dirtysock::tags2::at;
pub(crate) use crate::game::dirtysock::tags2::p_2;
pub(crate) use crate::game::dirtysock::tags2::gp_5 as gp_2;
pub(crate) use crate::game::dirtysock::tags2::sw;
pub(crate) use crate::game::dirtysock::tags2::const_reg;
pub(crate) use crate::game::wscript::wscriptman::gp as gp_4;
pub(crate) use crate::game::dirtysock::tags2::gp_9 as gp_5;
pub(crate) use crate::game::dirtysock::tags2::lw;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p_3;
pub(crate) use crate::game::scripter::ssxscriptengine::gp_5 as gp_6;
pub(crate) use crate::game::fe::fepopup3::p_2 as p_4;
pub(crate) use crate::game::fe::fepopup3::lw_2;
pub(crate) use crate::game::fe::fepopup3::sw as sw_2;
pub(crate) use crate::game::fe::fepopup3::lh;
pub(crate) use crate::game::scripter::ssxscriptengine::gp as gp_9;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at_3;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::main::gamemode::low as low_3;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::gpo;
pub(crate) use crate::game::movie::movieplayer::gp as gp_10;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at_4;
pub(crate) use crate::game::be::beintplayer::frame_restore_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_3;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at as lw_at_5;
pub(crate) use crate::game::dirtysock::tags2::lb_at;
pub(crate) use crate::game::fe::festaterewards::at_4;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_4;
pub(crate) use crate::game::wscript::wscriptman::gp as gp_14;
pub(crate) use crate::game::wscript::wscriptman::sw as sw_3;
pub(crate) use crate::game::fe::ovstates::at as at_6;
pub(crate) use crate::game::fe::ovstates::close;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_8;
pub(crate) use crate::game::ai::computer::lh_2 as lh_4;
pub(crate) use crate::game::fe::fepopup::sb;
pub(crate) use crate::game::fe::fepopup::ld;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001a_36e8, fe_async_manager_request_texture, "FEAsyncManager_RequestTexture"),
    (0x001a_3768, fe_async_manager_request_peak_map, "FEAsyncManager_RequestPeakMap"),
    (0x001a_37f8, fe_async_manager_request_session_texture, "FEAsyncManager_RequestSessionTexture"),
    (0x001a_39c0, fe_async_manager_get_state, "FEAsyncManager_GetState"),
    (0x001a_39d8, fe_async_manager_set_state, "FEAsyncManager_SetState"),
    (0x001a_3bf0, fe_async_manager_find_free_slot, "FEAsyncManager_FindFreeSlot"),
    (0x001a_3c50, fe_async_manager_free_all, "FEAsyncManager_FreeAll"),
    (0x001a_3dc8, c_fe_state_net_messenger_ctor, "cFEStateNetMessenger_ctor"),
    (0x001a_3f68, c_fe_state_net_messenger_init, "cFEStateNetMessenger_Init"),
    (0x001a_3fe0, fe_state_net_messenger_update, "cFEStateNetMessenger_Update"),
    (0x001a_4170, cfe_state_net_messenger_handle_input, "cFEStateNetMessenger_HandleInput"),
    (0x001a_4670, cfe_state_net_messenger_get_buddy_status, "cFEStateNetMessenger_GetBuddyStatus"),
    (0x001a_4740, cfe_state_net_messenger_get_list_index, "cFEStateNetMessenger_GetListIndex"),
    (0x001a_4928, c_fe_state_net_messenger_v33, "cFEStateNetMessenger_v33"),
    (0x001a_4c58, cfe_state_net_messenger_on_child_event, "cFEStateNetMessenger_OnChildEvent"),
    (0x001a_4ce8, net_fe_state_show_user_found, "cFEStateNetMessenger_ShowUserFound"),
    (0x001a_4d90, fe_net_messenger_show_looking_for_self, "cFEStateNetMessenger_ShowLookingForSelf"),
    (0x001a_4fb0, fe_state_net_messenger_on_find_user_result, "cFEStateNetMessenger_OnFindUserResult"),
    (0x001a_5450, cfestatenetmessenger_clear_name_list, "cFEStateNetMessenger_ClearNameList"),
    (0x001a_56d0, fe_net_messenger_has_name, "cFEStateNetMessenger_HasName"),
    (0x001a_5800, fe_net_messenger_find_name_str, "cFEStateNetMessenger_FindNameStr"),
    (0x001a_5fb8, cfe_state_net_messenger_highlight_row, "cFEStateNetMessenger_HighlightRow"),
    (0x001a_6618, c_fe_state_net_messenger_set_row_name, "cFEStateNetMessenger_SetRowName"),
    (0x001a_6c40, fe_state_net_messenger_set_player_menu_selection, "cFEStateNetMessenger_SetPlayerMenuSelection"),
    (0x001a_7ed8, c_fe_state_read_message_popup_add_sent_message, "cFEStateReadMessagePopup_AddSentMessage"),
    (0x001a_86e8, fe_set_online_mode, "FE_SetOnlineMode"),
    (0x001a_8770, net_fe_state_update, "NetFEState_Update"),
    (0x001a_87d0, net_fe_state_on_activate, "NetFEState_OnActivate"),
    (0x001a_8830, net_fe_state_v32, "NetFEState_v32"),
    (0x001a_89b0, net_fe_state_v26, "NetFEState_v26"),
    (0x001a_8ae8, net_fe_state_queue_net_event, "NetFEState_QueueNetEvent"),
    (0x001a_9090, net_fe_state_open_keyboard_str, "NetFEState_OpenKeyboardStr"),
    (0x001a_9150, net_fe_state_open_keyboard_text_id, "NetFEState_OpenKeyboardTextId"),
    (0x001a_91b0, net_fe_state_create_keyboard_popup, "NetFEState_CreateKeyboardPopup"),
    (0x001a_9268, net_fe_state_on_messenger_popup_event, "NetFEState_OnMessengerPopupEvent"),
    (0x001a_9438, net_fe_state_on_mem_card_login_popup_event, "NetFEState_OnMemCardLoginPopupEvent"),
    (0x001a_94e0, net_fe_state_on_keyboard_popup_event, "NetFEState_OnKeyboardPopupEvent"),
    (0x001a_9578, net_fe_state_on_session_lost_popup_event, "NetFEState_OnSessionLostPopupEvent"),
    (0x001a_9638, net_fe_state_on_child_event_ignore_close, "NetFEState_OnChildEventIgnoreClose"),
    (0x001a_9668, net_fe_state_on_disconnected_popup_event, "NetFEState_OnDisconnectedPopupEvent"),
    (0x001a_9710, net_fe_state_on_net_interrupted_popup_event, "NetFEState_OnNetInterruptedPopupEvent"),
    (0x001a_9890, netfe_state_set_selected_user, "NetFEState_SetSelectedUser"),
    (0x001a_98b8, net_fe_state_make_password_mask, "NetFEState_MakePasswordMask"),
    (0x001a_9930, net_fe_state_show_disconnected_popup, "NetFEState_ShowDisconnectedPopup"),
    (0x001a_9a18, net_fe_state_fill_yes_no_list, "NetFEState_FillYesNoList"),
    (0x001a_9ab8, net_fe_state_fill_on_off_list, "NetFEState_FillOnOffList"),
    (0x001a_9b58, net_fe_state_open_messenger, "NetFEState_OpenMessenger"),
    (0x001a_b108, net_fe_state_show_wait_player_popup, "NetFEState_ShowWaitPlayerPopup"),
    (0x001a_b370, net_fe_state_show_buddy_action_popup, "NetFEState_ShowBuddyActionPopup"),
    (0x001a_b3d0, net_fe_state_refresh_buddy_action_popup, "NetFEState_RefreshBuddyActionPopup"),
    (0x001a_bc50, net_fe_state_toggle_block_user, "NetFEState_ToggleBlockUser"),
    (0x001a_be58, net_fe_state_toggle_messenger_buddy, "NetFEState_ToggleMessengerBuddy"),
    (0x001a_bed8, net_fe_state_show_max_buddies_popup, "NetFEState_ShowMaxBuddiesPopup"),
    (0x001a_bf70, fe_screen_blank_ctor, "FEScreenBlank_ctor"),
    (0x001a_bfa8, fe_screen_blank_init, "FEScreenBlank_Init"),
    (0x001a_c010, c_fe_use_existing_account_ctor, "cFEUseExistingAccount_ctor"),
    (0x001a_c068, cfe_use_existing_account_dtor, "cFEUseExistingAccount_dtor"),
    (0x001a_c0b8, c_fe_use_existing_account_init, "cFEUseExistingAccount_Init"),
    (0x001a_c2b8, cfe_use_existing_account_check_password, "cFEUseExistingAccount_CheckPassword"),
    (0x001a_c360, cfe_use_existing_account_check_account_name, "cFEUseExistingAccount_CheckAccountName"),
    (0x001a_c408, c_fe_use_existing_account_validate_fields, "cFEUseExistingAccount_ValidateFields"),
    (0x001a_c448, cfe_use_existing_account_check_email, "cFEUseExistingAccount_CheckEmail"),
    (0x001a_c4f0, fe_use_existing_account_v25, "cFEUseExistingAccount_v25"),
    (0x001a_c5a8, fe_use_existing_account_on_widget_notify, "cFEUseExistingAccount_OnWidgetNotify"),
    (0x001a_c9c0, cfe_use_existing_account_on_email_popup_event, "cFEUseExistingAccount_OnEmailPopupEvent"),
    (0x001a_ca88, fe_use_existing_account_on_abort_popup_event, "cFEUseExistingAccount_OnAbortPopupEvent"),
    (0x001a_d130, fe_state_select_persona_ctor, "cFEStateSelectPersona_ctor"),
    (0x001a_d308, c_fe_state_select_persona_on_widget_created, "cFEStateSelectPersona_OnWidgetCreated"),
    (0x001a_d930, cfe_state_select_persona_on_save_account_popup_event, "cFEStateSelectPersona_OnSaveAccountPopupEvent"),
    (0x001a_d9c8, cfe_state_select_persona_v33, "cFEStateSelectPersona_v33"),
    (0x001a_ddd0, fe_state_challenge_popup_ctor, "cFEStateChallengePopup_ctor"),
    (0x001a_dfb8, c_fe_state_challenge_popup_transition_in, "cFEStateChallengePopup_TransitionIn"),
    (0x001a_e100, fe_state_challenge_popup_on_widget_notify, "cFEStateChallengePopup_OnWidgetNotify"),
    (0x001a_ebf0, c_fe_state_challenge_popup_v32, "cFEStateChallengePopup_v32"),
    (0x001a_f3e0, fe_state_challenge_popup_v33, "cFEStateChallengePopup_v33"),
    (0x001a_f428, c_fe_state_challenge_popup_on_child_event, "cFEStateChallengePopup_OnChildEvent"),
    (0x001a_f688, cfe_state_challenge_popup_show_set_options, "cFEStateChallengePopup_ShowSetOptions"),
    (0x001a_fd88, net_fe_show_player_conn_quality, "NetFE_ShowPlayerConnQuality"),
    (0x001a_fe08, cfe_state_challenge_received_popup_ctor, "cFEStateChallengeReceivedPopup_ctor"),
    (0x001b_00b8, c_fe_state_challenge_received_popup_v32, "cFEStateChallengeReceivedPopup_v32"),
    (0x001b_05e0, c_fe_state_challenge_received_popup_on_child_event, "cFEStateChallengeReceivedPopup_OnChildEvent"),
    (0x001b_0628, fe_state_challenge_received_popup_v33, "cFEStateChallengeReceivedPopup_v33"),
    (0x001b_0670, c_fe_state_challenge_received_popup_transition_in, "cFEStateChallengeReceivedPopup_TransitionIn"),
    (0x001b_06e0, fe_state_challenge_received_popup_on_widget_notify, "cFEStateChallengeReceivedPopup_OnWidgetNotify"),
    (0x001b_0f28, char_to_list_index, "cFEStateOnlineCharacterSelect_CharToListIndex"),
    (0x001b_0f60, c_fe_state_online_character_select_ctor, "cFEStateOnlineCharacterSelect_ctor"),
    (0x001b_0fb0, cfe_state_online_character_select_init, "cFEStateOnlineCharacterSelect_Init"),
    (0x001b_14d8, fe_state_online_character_select_v25, "cFEStateOnlineCharacterSelect_v25"),
    (0x001b_1590, fe_state_online_character_select_handle_input, "cFEStateOnlineCharacterSelect_HandleInput"),
    (0x001b_1760, cfe_online_character_select_show_opponent_char, "cFEStateOnlineCharacterSelect_ShowOpponentChar"),
    (0x001b_1b10, cfestateonlinecharacterselect_show_character, "cFEStateOnlineCharacterSelect_ShowCharacter"),
    (0x001b_1ec0, c_fe_state_online_character_select_on_child_event, "cFEStateOnlineCharacterSelect_OnChildEvent"),
    (0x001b_1f18, c_fe_state_online_character_select_on_quit_popup_event, "cFEStateOnlineCharacterSelect_OnQuitPopupEvent"),
    (0x001b_22a0, c_fe_state_chat_room_v32, "cFEStateChatRoom_v32"),
    (0x001b_2320, c_fe_state_chat_room_update, "cFEStateChatRoom_Update"),
    (0x001b_33d8, fe_state_chat_room_on_keyboard_done, "cFEStateChatRoom_OnKeyboardDone"),
    (0x001b_3490, fe_state_chat_room_transition_in, "cFEStateChatRoom_TransitionIn"),
    (0x001b_3528, fe_state_chat_room_on_activate, "cFEStateChatRoom_OnActivate"),
    (0x001b_3568, c_fe_state_chat_room_on_widget_notify, "cFEStateChatRoom_OnWidgetNotify"),
    (0x001b_3760, c_fe_state_chat_room_on_go_to_char_select_event, "cFEStateChatRoom_OnGoToCharSelectEvent"),
    (0x001b_37d8, c_fe_state_chat_room_on_go_to_char_select_event_2, "cFEStateChatRoom_OnGoToCharSelectEvent_2"),
    (0x001b_3850, c_fe_state_chat_room_on_child_event, "cFEStateChatRoom_OnChildEvent"),
    (0x001b_38d0, cfe_state_chat_room_handle_input, "cFEStateChatRoom_HandleInput"),
    (0x001b_3978, fe_state_chat_room_select_player_by_name, "cFEStateChatRoom_SelectPlayerByName"),
    (0x001b_3f78, cfe_state_net_config_ctor, "cFEStateNetConfig_ctor"),
    (0x001b_4168, c_fe_state_net_config_shutdown, "cFEStateNetConfig_Shutdown"),
    (0x001b_43a8, cfe_state_net_config_check_card, "cFEStateNetConfig_CheckCard"),
    (0x001b_47c0, cfe_state_net_config_show_checking_card_popup, "cFEStateNetConfig_ShowCheckingCardPopup"),
    (0x001b_4c00, c_fe_state_net_config_on_popup_event_passthrough, "cFEStateNetConfig_OnPopupEventPassthrough"),
    (0x001b_4fa0, cfe_state_net_config_handle_load_error, "cFEStateNetConfig_HandleLoadError"),
    (0x001b_5268, fe_state_net_config_start_card_scan, "cFEStateNetConfig_StartCardScan"),
    (0x001b_5cf8, cfe_state_net_config_on_info_popup_closed, "cFEStateNetConfig_OnInfoPopupClosed"),
    (0x001b_6288, net_config_auth_popup_ctor, "NetConfigAuthPopup_ctor"),
    (0x001b_6368, net_config_auth_popup_dtor, "NetConfigAuthPopup_dtor"),
    (0x001b_66b8, net_config_auth_popup_handle_input, "NetConfigAuthPopup_HandleInput"),
    (0x001b_6740, net_config_auth_popup_v24, "NetConfigAuthPopup_v24"),
    (0x001b_6770, net_config_auth_popup_on_widget_action, "NetConfigAuthPopup_OnWidgetAction"),
    (0x001b_6a90, net_config_auth_popup_on_widget_notify, "NetConfigAuthPopup_OnWidgetNotify"),
    (0x001b_6ab8, c_fe_state_auth_dnas_error_ctor, "cFEStateAuthDNASError_ctor"),
    (0x001b_6b50, cfe_state_auth_dnas_error_init, "cFEStateAuthDNASError_Init"),
    (0x001b_6bb8, c_fe_state_auth_dnas_error_on_widget_created, "cFEStateAuthDNASError_OnWidgetCreated"),
    (0x001b_6e10, c_fe_state_auth_dnas_error_v24, "cFEStateAuthDNASError_v24"),
    (0x001b_6e40, c_fe_state_auth_dnas_error_on_widget_action, "cFEStateAuthDNASError_OnWidgetAction"),
    (0x001b_6ea8, fe_state_auth_dnas_error_on_widget_notify, "cFEStateAuthDNASError_OnWidgetNotify"),
    (0x001b_7030, c_fe_state_net_account_mem_dtor, "cFEStateNetAccountMem_dtor"),
    (0x001b_7778, cfe_state_net_account_mem_store_account_strings, "cFEStateNetAccountMem_StoreAccountStrings"),
    (0x001b_77c8, cfe_state_net_account_mem_update_state02, "cFEStateNetAccountMem_UpdateState02"),
    (0x001b_7820, cfe_state_net_account_mem_update_state06, "cFEStateNetAccountMem_UpdateState06"),
    (0x001b_78f0, c_fe_state_net_account_mem_update_state08, "cFEStateNetAccountMem_UpdateState08"),
    (0x001b_7938, cfe_state_net_account_mem_update_state0c, "cFEStateNetAccountMem_UpdateState0C"),
    (0x001b_7a08, c_fe_state_net_account_mem_update_state0e, "cFEStateNetAccountMem_UpdateState0E"),
    (0x001b_7a50, c_fe_state_net_account_mem_update_state10, "cFEStateNetAccountMem_UpdateState10"),
    (0x001b_7bc0, cfe_state_net_account_mem_update_state16, "cFEStateNetAccountMem_UpdateState16"),
    (0x001b_7c28, c_fe_state_net_account_mem_update_state1a, "cFEStateNetAccountMem_UpdateState1A"),
    (0x001b_7c70, c_fe_state_net_account_mem_update_state1b, "cFEStateNetAccountMem_UpdateState1B"),
    (0x001b_8860, c_fe_state_net_account_mem_v24, "cFEStateNetAccountMem_v24"),
    (0x001b_8890, fe_state_net_account_mem_popup_id_to_state, "cFEStateNetAccountMem_PopupIdToState"),
    (0x001b_88f0, c_fe_state_create_step1_ctor, "cFEStateCreateStep1_ctor"),
    (0x001b_8960, c_fe_state_create_step1_init, "cFEStateCreateStep1_Init"),
    (0x001b_8a08, cfe_state_create_step1_transition_in, "cFEStateCreateStep1_TransitionIn"),
    (0x001b_9428, cfe_state_create_step1_on_child_event, "cFEStateCreateStep1_OnChildEvent"),
    (0x001b_9558, cfe_state_create_step1_all_fields_filled, "cFEStateCreateStep1_AllFieldsFilled"),
    (0x001b_95c0, cfe_state_create_step1_set_account_name_text, "cFEStateCreateStep1_SetAccountNameText"),
    (0x001b_9610, cfe_state_create_step1_set_password_text, "cFEStateCreateStep1_SetPasswordText"),
    (0x001b_9678, cfe_state_create_step1_set_confirm_password_text, "cFEStateCreateStep1_SetConfirmPasswordText"),
    (0x001b_96e0, fe_state_create_step1_set_email_text, "cFEStateCreateStep1_SetEmailText"),
    (0x001b_9740, c_fe_state_create_step1_set_inform_selection, "cFEStateCreateStep1_SetInformSelection"),
    (0x001b_97b0, c_fe_state_create_step1_set_share_info_selection, "cFEStateCreateStep1_SetShareInfoSelection"),
    (0x001b_9cd8, c_fe_state_create_step1_is_email_format_valid, "cFEStateCreateStep1_IsEmailFormatValid"),
    (0x001b_9f68, cfe_state_create_step1_char_in_range, "cFEStateCreateStep1_CharInRange"),
    (0x001b_a110, c_fe_state_create_step1_validate_fields, "cFEStateCreateStep1_ValidateFields"),
    (0x001b_a178, cfe_state_create_step1_backup_account_info, "cFEStateCreateStep1_BackupAccountInfo"),
    (0x001b_a218, c_fe_state_create_step1_restore_account_info, "cFEStateCreateStep1_RestoreAccountInfo"),
    (0x001b_a358, c_fe_state_create_step1_detect_changes, "cFEStateCreateStep1_DetectChanges"),
    (0x001b_a3e0, fe_state_create_step2_ctor, "cFEStateCreateStep2_ctor"),
    (0x001b_a428, fe_state_create_step2_init, "cFEStateCreateStep2_Init"),
    (0x001b_a500, cfe_state_create_step2_on_widget_created, "cFEStateCreateStep2_OnWidgetCreated"),
    (0x001b_a9f8, cfe_state_create_step2_submit_account, "cFEStateCreateStep2_SubmitAccount"),
    (0x001b_b0e8, c_fe_state_create_step2_on_parent_email_entered, "cFEStateCreateStep2_OnParentEmailEntered"),
    (0x001b_b2d0, c_fe_state_create_step2_set_month_selection, "cFEStateCreateStep2_SetMonthSelection"),
    (0x001b_b340, c_fe_state_create_step2_set_day_selection, "cFEStateCreateStep2_SetDaySelection"),
    (0x001b_b3b0, c_fe_state_create_step2_set_year_selection, "cFEStateCreateStep2_SetYearSelection"),
    (0x001b_b440, c_fe_state_create_step2_set_gender_selection, "cFEStateCreateStep2_SetGenderSelection"),
    (0x001b_b4b0, c_fe_state_create_step2_set_legal_selection, "cFEStateCreateStep2_SetLegalSelection"),
    (0x001b_b520, cfe_state_create_step2_fill_gender_list, "cFEStateCreateStep2_FillGenderList"),
    (0x001b_b798, cfe_create_step2_fill_year_list, "cFEStateCreateStep2_FillYearList"),
    (0x001b_b840, c_fe_state_create_step2_fill_day_list, "cFEStateCreateStep2_FillDayList"),
    (0x001b_b968, cfe_state_create_step2_is_leap_year, "cFEStateCreateStep2_IsLeapYear"),
    (0x001b_b9b0, c_fe_state_create_step2_get_days_in_month, "cFEStateCreateStep2_GetDaysInMonth"),
    (0x001b_bd08, c_fe_state_create_duplicate_ctor, "cFEStateCreateDuplicate_ctor"),
    (0x001b_bd60, c_fe_state_create_duplicate_init, "cFEStateCreateDuplicate_Init"),
    (0x001b_c290, c_fe_state_create_duplicate_submit_account, "cFEStateCreateDuplicate_SubmitAccount"),
    (0x001b_c6a0, c_fe_state_create_duplicate_on_child_event, "cFEStateCreateDuplicate_OnChildEvent"),
    (0x001b_cc30, c_fe_state_lobby_menu_init, "cFEStateLobbyMenu_Init"),
    (0x001b_cca8, c_fe_state_lobby_menu_v32, "cFEStateLobbyMenu_v32"),
    (0x001b_cd18, cfestate_lobby_menu_transition_in, "cFEStateLobbyMenu_TransitionIn"),
    (0x001b_d428, fe_lobby_menu_on_keyboard_done, "cFEStateLobbyMenu_OnKeyboardDone"),
    (0x001b_d460, c_fe_state_lobby_menu_enter_room, "cFEStateLobbyMenu_EnterRoom"),
    (0x001b_d848, c_fe_state_lobby_menu_on_net_result, "cFEStateLobbyMenu_OnNetResult"),
    (0x001b_dcf8, cfe_state_lobby_menu_on_create_room_popup_event, "cFEStateLobbyMenu_OnCreateRoomPopupEvent"),
    (0x001b_dd60, cfestatelobbymenu_prompt_room_password, "cFEStateLobbyMenu_PromptRoomPassword"),
    (0x001b_dde8, cfe_state_lobby_menu_on_child_event, "cFEStateLobbyMenu_OnChildEvent"),
    (0x001b_df60, c_fe_state_lobby_menu_show_room_tab, "cFEStateLobbyMenu_ShowRoomTab"),
    (0x001b_dfd8, cfe_state_lobby_menu_hide_room_tabs, "cFEStateLobbyMenu_HideRoomTabs"),
    (0x001b_e038, c_fe_state_lobby_menu_clamp_room_selections, "cFEStateLobbyMenu_ClampRoomSelections"),
    (0x001b_e510, c_fe_state_lobby_menu_refresh_player_list, "cFEStateLobbyMenu_RefreshPlayerList"),
    (0x001b_e7d8, c_fe_state_net_create_room_popup_ctor, "cFEStateNetCreateRoomPopup_ctor"),
    (0x001b_e828, cfe_state_net_create_room_popup_init, "cFEStateNetCreateRoomPopup_Init"),
    (0x001b_ec78, net_create_room_popup_validate_room_name, "cFEStateNetCreateRoomPopup_ValidateRoomName"),
    (0x001b_ed28, c_fe_state_net_create_room_popup_v31, "cFEStateNetCreateRoomPopup_v31"),
    (0x001b_ee30, c_fe_state_login_ctor, "cFEStateLogin_ctor"),
    (0x001b_ee78, cfe_state_login_dtor, "cFEStateLogin_dtor"),
    (0x001b_eed0, c_fe_state_login_init, "cFEStateLogin_Init"),
    (0x001b_f378, fe_state_login_v33, "cFEStateLogin_v33"),
    (0x001b_fbf8, cfe_online_main_menu_init, "cFEStateOnlineMainMenu_Init"),
    (0x001c_0138, c_fe_state_online_main_menu_on_leave_popup_event, "cFEStateOnlineMainMenu_OnLeavePopupEvent"),
    (0x001c_0240, cfe_online_main_menu_on_disconnect_popup_event, "cFEStateOnlineMainMenu_OnDisconnectPopupEvent"),
    (0x001c_02f8, cfe_state_online_main_menu_on_child_event, "cFEStateOnlineMainMenu_OnChildEvent"),
    (0x001c_0358, c_fe_state_online_main_menu_v31, "cFEStateOnlineMainMenu_v31"),
    (0x001c_0418, c_fe_state_online_agreement_ctor, "cFEStateOnlineAgreement_ctor"),
    (0x001c_04a8, fe_state_online_agreement_init, "cFEStateOnlineAgreement_Init"),
    (0x001c_0a28, cfe_state_online_agreement_dtor, "cFEStateOnlineAgreement_dtor"),
    (0x001c_0a80, c_fe_state_online_agreement_update, "cFEStateOnlineAgreement_Update"),
    (0x001c_0b08, fe_state_online_agreement_on_widget_notify, "cFEStateOnlineAgreement_OnWidgetNotify"),
    (0x001c_0ce0, c_fe_state_online_agreement_on_widget_created, "cFEStateOnlineAgreement_OnWidgetCreated"),
    (0x001c_0ea8, fe_state_online_agreement_v25, "cFEStateOnlineAgreement_v25"),
    (0x001c_0f40, c_fe_state_net_quick_play_popup_ctor, "cFEStateNetQuickPlayPopup_ctor"),
    (0x001c_0fc0, c_fe_state_net_quick_play_popup_init, "cFEStateNetQuickPlayPopup_Init"),
    (0x001c_1038, fe_state_net_quick_play_popup_update, "cFEStateNetQuickPlayPopup_Update"),
    (0x001c_13e0, cfe_net_quick_play_popup_set_options_hint_visible, "cFEStateNetQuickPlayPopup_SetOptionsHintVisible"),
    (0x001c_17e0, fe_state_net_quick_play_popup_on_widget_notify, "cFEStateNetQuickPlayPopup_OnWidgetNotify"),
    (0x001c_17f8, net_quick_play_popup_set_options_enabled, "cFEStateNetQuickPlayPopup_SetOptionsEnabled"),
    (0x001c_1b48, fe_state_net_quick_play_popup_on_child_event, "cFEStateNetQuickPlayPopup_OnChildEvent"),
    (0x001c_1b88, cfe_state_net_quick_play_popup_on_options_popup_event, "cFEStateNetQuickPlayPopup_OnOptionsPopupEvent"),
    (0x001c_1c50, c_fe_state_net_ssx_world_ctor, "cFEStateNetSSXWorld_ctor"),
    (0x001c_1c90, c_fe_state_net_ssx_world_init, "cFEStateNetSSXWorld_Init"),
    (0x001c_1e08, c_fe_state_net_ssx_world_on_widget_created, "cFEStateNetSSXWorld_OnWidgetCreated"),
    (0x001c_20f8, c_fe_state_net_quick_messages_ctor, "cFEStateNetQuickMessages_ctor"),
    (0x001c_2138, c_fe_state_net_quick_messages_init, "cFEStateNetQuickMessages_Init"),
    (0x001c_23c0, c_fe_state_net_quick_messages_transition_in, "cFEStateNetQuickMessages_TransitionIn"),
    (0x001c_2b48, cfe_state_net_quick_messages_on_child_event, "cFEStateNetQuickMessages_OnChildEvent"),
    (0x001c_2ba8, cfe_net_quick_messages_on_reset_popup_event, "cFEStateNetQuickMessages_OnResetPopupEvent"),
    (0x001c_2c50, cfe_state_net_quick_messages_on_save_popup_event, "cFEStateNetQuickMessages_OnSavePopupEvent"),
    (0x001c_2d18, c_fe_state_stats_ctor, "cFEStateStats_ctor"),
    (0x001c_2e00, c_fe_state_stats_init, "cFEStateStats_Init"),
    (0x001c_2e70, c_fe_state_stats_v32, "cFEStateStats_v32"),
    (0x001c_32b8, c_fe_state_stats_transition_in, "cFEStateStats_TransitionIn"),
    (0x001c_4b78, cfe_state_stats_reset_results_page, "cFEStateStats_ResetResultsPage"),
    (0x001c_4b98, fe_state_stats_on_widget_notify, "cFEStateStats_OnWidgetNotify"),
    (0x001c_4cd8, cfe_state_stats_v31, "cFEStateStats_v31"),
    (0x001c_4f58, fe_state_stats_format_time, "cFEStateStats_FormatTime"),
    (0x001c_5018, c_fe_state_net_news_ctor, "cFEStateNetNews_ctor"),
    (0x001c_5098, c_fe_state_net_news_init, "cFEStateNetNews_Init"),
    (0x001c_5108, c_fe_state_net_news_v32, "cFEStateNetNews_v32"),
    (0x001c_5310, cfe_state_net_news_on_widget_created, "cFEStateNetNews_OnWidgetCreated"),
    (0x001c_5420, fe_state_net_news_on_widget_notify, "cFEStateNetNews_OnWidgetNotify"),
    (0x001c_5540, fe_state_net_news_v25, "cFEStateNetNews_v25"),
    (0x001c_55f8, fe_popup_confirm_cfg_ctor, "cFEPopupConfirm_CfgCtor"),
    (0x001c_5750, c_fe_popup_confirm_cfg_dtor, "cFEPopupConfirm_CfgDtor"),
    (0x001c_5780, cfepopupconfirm_cfg_set_option_string, "cFEPopupConfirm_CfgSetOptionString"),
    (0x001c_57a0, cfepopupconfirm_cfg_set_option_wide_string, "cFEPopupConfirm_CfgSetOptionWideString"),
    (0x001c_57c0, cfepopupconfirm_cfg_set_option_text_id, "cFEPopupConfirm_CfgSetOptionTextId"),
    (0x001c_5800, cfepopupconfirm_cfg_get_option, "cFEPopupConfirm_CfgGetOption"),
    (0x001c_5838, cfepopupconfirm_cfg_set_option_confirm_wide, "cFEPopupConfirm_CfgSetOptionConfirmWide"),
    (0x001c_5860, cfepopupconfirm_cfg_clear_option_confirm, "cFEPopupConfirm_CfgClearOptionConfirm"),
    (0x001c_5878, cfepopupconfirm_cfg_set_option_confirm_text_id, "cFEPopupConfirm_CfgSetOptionConfirmTextId"),
    (0x001c_58d0, cfepopupconfirm_cfg_get_option_confirm, "cFEPopupConfirm_CfgGetOptionConfirm"),
    (0x002c_2718, ustr_sprintf_ascii_fmt, "UStr_SprintfAsciiFmt"),
    (0x002c_27c0, ustr_sprintf_ascii_fmt_2, "UStr_SprintfAsciiFmt_2"),
    (0x003d_ef40, async_file_assign_serial, "AsyncFile_AssignSerial"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EVENT_SELECT_HANDLE_INPUT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MC_MANAGER_GET_INSTANCE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_UPDATE_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_ENGINE_PUSH_FREE_NODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_OBJ_DEACTIVATE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_DUMMY_OBJ_READ_FIELDS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EA_MEM_COPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::TEX_MAN_SET_ALLOC_FLAGS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::GRAPHICS_MAN_EMPTY_HOOK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_STATE_MGR_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_NODE_UNLINK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_RESET_SECTIONS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STRLEN_AND_STRCPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::CBXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::LOBBY_MAN_REQUEST_STATS_PAGE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SSX_APP_BOOT_LOAD;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::USTR_FROM_ASCII;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_SET_TRACK_CONTROL_VALUE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_INFO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_NAME;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STREAM_BUFFER_GET_STATUS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MENU_SYS_GET_INPUT_CODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::IRRADIANCE_REF_GET_ENTRY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::C_WORLD_SOUND_UNLINK_EMITTERS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RIDER_METRIX_BANK_COMBO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::be::beintmission::callee::WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::be::beintmission::callee::UITEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::be::beintmission::callee::MPEG2_ERROR;
    pub(crate) use crate::game::be::beintmission::callee::MISSION_COUNT;
    pub(crate) use crate::game::be::beintmission::callee::MISSION_NTH;
    pub(crate) use crate::game::be::beintmission::callee::REWARDS_FILE_LOAD;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_EMPTY_STUB;
    pub(crate) use crate::game::be::beintmission::callee::UBER_TRICK_REFRESH;
    pub(crate) use crate::game::be::beintmission::callee::MEMCARD_SCAN_STEP;
    pub(crate) use crate::game::be::beintmission::callee::NET_FE_STATE_ON_WIDGET_ACTION;
    pub(crate) use crate::game::be::beintmission::callee::VIEW_POPUP_SHOW_MESSAGE;
    pub(crate) use crate::game::be::beintmission::callee::BUY_ATTRIB_UPDATE_BARS;
    pub(crate) use crate::game::be::beintmission::callee::CAREER_STATS_SHOW_HIGHLIGHTS;
    pub(crate) use crate::game::bx::bxstring2::callee::FE_MODULE_RIDER_PREVIEW_REQUEST_CHAR;
    pub(crate) use crate::game::bx::bxstring2::callee::C_FE_STATE_LOBBY_MENU_FILL_PLAYER_LIST;
    pub(crate) use crate::game::bx::bxstring2::callee::C_FE_STATE_LOBBY_MENU_UPDATE_PLAYER_SCROLL_ARROWS;
    pub(crate) use crate::game::bx::bxstring2::callee::EVENT_SELECT_ON_ACTIVATE;
    pub(crate) use crate::game::bx::bxstring2::callee::EVENT_SELECT_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::C_LUNO_TABLE_ALLOC;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN2D_REMOVE_ALL;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN2D_RESET;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_INIT;
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::VID_ENGINE_PLAYER_START_PLAYBACK;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_LIST_PREPARE;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_LIST_SET_PAUSED_NESTED;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_LIST_GET_ENTRY_TYPE;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_STOP_MAIN_LISTS;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_STOP_AUX_LISTS;
    pub(crate) use crate::game::bx::bxstring2::callee::SNAPSHOT_LIST_REMOVE;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_PARSE;
    pub(crate) use crate::game::bx::bxstring2::callee::WORLD_STATE_RESTORE;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::main::ssxappdtor::callee::NET_FE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::main::ssxappdtor::callee::SHARED_LOGIN_SET_ACCOUNT_NAME;
    pub(crate) use crate::game::main::ssxappdtor::callee::SHARED_LOGIN_SET_ACCOUNT_PASSWORD;
    pub(crate) use crate::game::main::ssxappdtor::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::main::ssxappdtor::callee::UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::main::ssxappdtor::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::main::ssxappdtor::callee::AUDIO_PLAY_FE_SOUND;
    pub(crate) use crate::game::main::ssxappdtor::callee::AUDIO_ON_EVENT_SELECT_ENTER;
    pub(crate) use crate::game::main::ssxappdtor::callee::BUY_ATTRIB_COMMIT_PURCHASE;
    pub(crate) use crate::game::main::ssxappdtor::callee::FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::main::ssxappdtor::callee::CAREER_STATS_SELECT_PAGE;
    pub(crate) use crate::game::main::ssxappdtor::callee::REWARD_LIST_SHOW_PAGE;
    pub(crate) use crate::game::main::ssxappdtor::callee::EVENT_SELECT_SET_MODE;
    pub(crate) use crate::game::main::ssxappdtor::callee::EVENT_SELECT_SHUTDOWN;
    pub(crate) use crate::game::main::ssxappdtor::callee::OV_STATE_OVERLAY_SHUTDOWN;
    pub(crate) use crate::game::main::ssxappdtor::callee::UI_ENGINE_UPDATE;
    pub(crate) use crate::game::main::ssxappdtor::callee::APP_MODULE_UPDATE;
    pub(crate) use crate::game::main::ssxappdtor::callee::DELETE;
    pub(crate) use crate::game::be::beintmission::callee::FIND_MISSION;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_RESET_STATE;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM_2;
    pub(crate) use crate::game::be::beintmission::callee::GET_RIDER_PREVIEW;
    pub(crate) use crate::game::be::beintmission::callee::RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::be::beintmission::callee::NET_FE_STATE_CTOR;
    pub(crate) use crate::game::be::beintmission::callee::LOBBY_MENU_REQUEST_JOIN_ROOM;
    pub(crate) use crate::game::be::beintmission::callee::POPUP_CONFIRM_CFG_RESET;
    pub(crate) use crate::game::be::beintmission::callee::REWARD_GALLERY_CHECK_REWARDS_READY;
    pub(crate) use crate::game::be::beintmission::callee::REWARD_GALLERY_POLL_SHAPE_LOAD;
    pub(crate) use crate::game::be::beintmission::callee::BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::be::beintmission::callee::REPLAY_APPLY_CAMERA;
    pub(crate) use crate::game::be::beintmission::callee::RACE_PROGRESS_RESET_ALL;
    pub(crate) use crate::game::be::beintmission::callee::MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_MAIN_LISTS as SCRIPT_ENGINE_STOP_MAIN_LISTS_2;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_AUX_LISTS as SCRIPT_ENGINE_STOP_AUX_LISTS_2;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_DEACTIVATE_BINDING;
    pub(crate) use crate::game::be::beintmission::callee::CROWD_AUDIO_STOP_ALL;
    pub(crate) use crate::game::be::beintmission::callee::BANK_MONITOR_STOP_ALL;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::SND_BANK_SET_MEMORY;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_IS_GAME_MUSIC_ENABLED;
    pub(crate) use crate::game::be::beintmission::callee::MUSIC_PLAYER_STOP_SONG;
    pub(crate) use crate::game::be::beintmission::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::be::beintmission::callee::BE_NEW_PLAYER_GET_INBOX;
    pub(crate) use crate::game::be::beintmission::callee::WSCRIPT_MAN_EVENT_QUEUE_POP;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_START_PRERACE_MUSIC;
    pub(crate) use crate::game::be::beintmission::callee::SLOGIN_DELETE;
    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_MAN_UPDATE_MASK1;
    pub(crate) use crate::game::be::beintmission::callee::EA_MEMSET;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_ON_ACTIVATE as FE_STATE_ON_ACTIVATE_2;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_NAMES;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_COUNT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_DISCONNECTED_POPUP_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_QUICK_PLAY_POPUP_ON_OPTIONS_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::POPUP_CONFIRM_GET_TEXT_BOX_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::PREVIEW_REWARD_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::festaterewards2::callee::MOUNTAIN_ROOM_UPDATE_EVENT_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::TROPHY_ROOM_LOAD_TROPHY_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards2::callee::SECTION_MAN_FIND_LOGICAL_SECTION;
    pub(crate) use crate::game::fe::festaterewards2::callee::GAME_MODULE_REQUEST_STATE as GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::fe::festaterewards2::callee::APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::festaterewards2::callee::S_LOGIN_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::USB_KB_DEFAULT_CONNECT_CB;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_STATE_STREAM_RESET;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_STREAM_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_NEXT_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_PREV_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SNAP_TO_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SET_STATE_2 as REPLAY_SET_STATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_ADD_MOVIE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_POLL_LOADING;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_UNLOAD_WORLD_SCRIPTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::fe::festaterewards2::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::fe::festaterewards2::callee::SONG_INSTANCE_GET_DUCK_TO_LOOPS;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_TRIGGER_MANAGER_GET_TRIGGER_INFO;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_MAN_FREE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_GET_POOL_BLOCK_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::DL_HEAP_USABLE_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_REPORT_POOL_STATS;
    pub(crate) use crate::game::fe::festaterewards2::callee::PS2_PAD_CREATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::ANIM_OBJECT_V36;
    pub(crate) use crate::game::fe::festaterewards2::callee::DYNAMIC_EMITTER_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_UPDATE as UI_STATE_UPDATE_2;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::TWSCRIPT_CACHE_RELOCATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_LOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_UNLOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_ARRANGEMENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_READ_MODE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEMSET;
    pub(crate) use crate::game::fe::festaterewards2::callee::SPRINTF;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_SHOW_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_DESTROY_MISSION_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_CREATE_COMPUTER_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_REWARD_INT_COUNT_SET_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::fe::ovstatepause::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_BEGIN_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_CTOR as UI_STATE_CTOR_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_FORMAT_MONEY;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXT_SET_STRING as UI_TEXT_SET_STRING_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_ACCOUNT_NAME;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_PASSWORD;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_ON_CHILD_EVENT as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_MOVE_DOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::EVENT_SELECT_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::STR_HASH;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::ovstatepause::callee::VID_ENGINE_PLAYER_RENDER;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_RENDER;
    pub(crate) use crate::game::fe::ovstatepause::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::ovstatepause::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::ovstatepause::callee::MOUNTAIN_ROOM_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_UPDATE_2 as UI_STATE_UPDATE_3;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_ON_ACTIVATE_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::PLAY_FE_SOUND;
    pub(crate) use crate::game::fe::ovstatepause::callee::MEM_CARD_STATE_BASE_CTOR_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATE_AND_POPUPS;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::fe::ovstatepause::callee::OV_SCREEN_SESSION_DESTROY_INDICATORS;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_SHUTDOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_ASYNC_MANAGER_CANCEL;
    pub(crate) use crate::game::fe::ovstatepause::callee::SECTION_MAN_REQUEST_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRCPY;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRLEN;
    pub(crate) use crate::game::dirtysock::tags2::callee::REWARDS_FILE_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOUNDED_CAMERA_ALGORITHM_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_2 as NET_FE_STATE_ON_CHILD_EVENT_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_IS_EVENT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LISTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LOBBY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_PEER_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SHARED_LOGIN_MAN_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_MAN_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_CACHE_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SNAPSHOT_EMPTY_HOOK_B;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RESTORE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_END_PLAYBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ANIM_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS_2 as SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_IS_BANK_SLOT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_CURRENT_SKIPPABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_FIELD_B0;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_FIND_BANK_SLOT_BY_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CANCEL_BANK_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_LIST_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_2 as EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_SET_PREVIEW_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_APP_START_FE_MODULE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOC_STRING_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BENEW_PLAYER_INT_IS_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE_RIDER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_VIEW_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_STREAM_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_API_ADD_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_CREATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::APP_SOMETHING;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_DECODE_FRAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_FRAME_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAM_CONTROLLER_SET_ALGORITHM;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_ON_ACTIVATE_2 as FE_STATE_ON_ACTIVATE_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_REFRESH_LIST;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_NOW_PLAYING;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_BUY_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_EQUIP_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_GET_NEXT_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_MESSENGER_SET_PLAYER_MENU_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_STATS_REFRESH_RESULTS_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_IS_KEY_ENABLED as KEYBOARD_POPUP_IS_KEY_ENABLED_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_MAN2D_INIT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_PRE_RACE_MENU_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_COMM_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_DATA_GET_INFO_2 as SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2 as SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MOVIE_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR_2 as REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_SPAN_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_EVENT_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_RAMP_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FLOAT_TO_UINT;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY as STRCPY_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_0;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_1;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_UPDATE_BUTTONS_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_MESSAGE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_APPLY_OUTPUT_MODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_SET_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_BE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_INTERFACE_CALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBXSTRING_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_SHOW;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMORY_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_SET_OUTPUT;
    pub(crate) use crate::game::wscript::wscriptman::callee::SET_FADE_GAIN;
    pub(crate) use crate::game::wscript::wscriptman::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND as CROWD_AUDIO_INSTANCE_STOP_SOUND_2;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_POLL_FREE;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_SET_PITCH;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_MEMCARD_SET_STATE;
    pub(crate) use crate::game::wscript::wscriptman::callee::AVALANCHE_BAKE_NODE_HISTORY;
    pub(crate) use crate::game::wscript::wscriptman::callee::WSCRIPT_MAN_CALL_FUNC;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_FREE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_CREATE_MESHES_2 as CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLEX_RAIL_UPDATE_SPACE_NODE;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FLAG_MANAGER_GROUP_LOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLOATING_V49;
    pub(crate) use crate::game::wscript::wscriptman::callee::MOD_OBJ_DTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::RAIL_MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::DISABLE_INTC;
    pub(crate) use crate::game::wscript::wscriptman::callee::REMOVE_INTC_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::wscript::wscriptman::callee::LIGHT_MAN_EMPTY_HOOK_A;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_GET_STATE_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_CHUNK_RESOLVE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::AV_PLAYER_INIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::FOCUS_MAN_ADD_FOCUS_BOX;
    pub(crate) use crate::game::wscript::wscriptman::callee::CAM_CONTROLLER_CLEAR_ALGORITHMS;
    pub(crate) use crate::game::wscript::wscriptman::callee::ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::wscript::wscriptman::callee::APP_MODULE_UPDATE_FADE;
    pub(crate) use crate::game::wscript::wscriptman::callee::DEBUG_MENU_MODULE_UPDATE_CURSOR_TRAIL;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_ON_CHILD_EVENT as NET_FE_STATE_ON_CHILD_EVENT_5;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SET_USER_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_FILE_MOUNT_BIG;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_CTOR as NET_FE_STATE_CTOR_6;
    pub(crate) use crate::game::dirtysock::tags2::callee::ON_QUIT_POPUP_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT_6;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_3 as NET_FE_STATE_CTOR_7;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::STATS_SELECT_CATEGORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_SET_USE_SAVE_TITLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::TROPHY_ROOM_RELEASE_IMAGES;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_GET_INBOX as MESSAGE_CENTER_GET_INBOX_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_PUSH_FREE_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_END_OPERATION_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_SET_PRESENCE_ADDR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_DESTROY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_OBJ_CAM_SET_OWNER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_OBJ_TABLE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLINK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY_2 as SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RECORD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_ADD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::STREAM_BUFFER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::PATH_PLAT_REFRESH_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_ASCII_CHAR_TO_SJIS;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_GET_SAVE_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_READ_SAVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_FREE_HISTORIES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_BAKE_GROUP;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_EXPORT_HISTORY_FILES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_LOAD_HISTORY_FILE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNOW_BOX_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT0;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT1;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT3;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT4;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT5;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT6;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::UIMENU_SET_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_GET_FONT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_REWARD_INT_IS_UNLOCKED;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR_4;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_LIST_DIR_ALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_BX_STRING_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT_3 as LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SLOGIN_MC_GET_PORT_SLOT_2 as SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCE_MC_FORMAT;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY as STRCPY_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_SEND_QUIT as C_SETUP_MAN_SEND_QUIT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF as UI_OBJECT_SHOW_CLAMP_ALPHA_HALF_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_LUNO_TABLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::APP_VSYNC_CALLBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_TEX_FLIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::DYNAMIC_EMITTER_FREE_DATA_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RCMP_DECODER_RELEASE_CODEC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_BONUS_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_SHOW_HUD_NUMBER_2 as RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AI_CREATE_ACTOR_RIDERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHASE_CAM_CONTROLLER_RESET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_STACK_CLEAR_2 as CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_CLEAR_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_ALL_TABLES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::NET_FE_STATE_DTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_LOAD_STATE_SETUP_RESULT_WIDGET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::EVENT_SELECT_ON_WIDGET_ACTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_PLAYBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_RECORDING;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_CACHE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_STOP_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_EXIT_SCRIPT_MODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_ON_SEQ_END;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_TARGET_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_OBJ_TABLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_PROCESS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD_MEMORY_2 as SND_BANK_UNLOAD_MEMORY;
    pub(crate) use crate::game::fe::fepopup3::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::fe::fepopup3::callee::FE_MODULE_RIDER_PREVIEW_CTOR;
    pub(crate) use crate::game::fe::fepopup3::callee::FE_ASYNC_MANAGER_GET_STATE;
    pub(crate) use crate::game::fe::fepopup3::callee::NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::fepopup3::callee::NET_FE_STATE_SETUP_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::fe::fepopup3::callee::NET_FE_STATE_CTOR as NET_FE_STATE_CTOR_8;
    pub(crate) use crate::game::fe::fepopup3::callee::NET_FE_STATE_ON_CHILD_EVENT as NET_FE_STATE_ON_CHILD_EVENT_7;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_CREATE_STEP1_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_CREATE_STEP1_ON_EDIT_DONE_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_LOBBY_MENU_ON_CREATE_ROOM_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_LOBBY_MENU_PROMPT_ROOM_PASSWORD;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_ONLINE_MAIN_MENU_ON_LEAVE_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_ONLINE_MAIN_MENU_ON_DISCONNECT_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_NET_QUICK_MESSAGES_ON_SAVE_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_NET_QUICK_MESSAGES_ON_RESET_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_REWARD_GALLERY_V29;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_REWARD_GALLERY_RELEASE_TEXTURES;
    pub(crate) use crate::game::fe::fepopup3::callee::MESSAGE_CENTER_GET_INBOX_3;
    pub(crate) use crate::game::fe::fepopup3::callee::HUD_LAYOUT_ICON_TEXT;
    pub(crate) use crate::game::fe::fepopup3::callee::GAME_MODULE_CTOR;
    pub(crate) use crate::game::fe::fepopup3::callee::MC_MANAGER_END_PROGRESS;
    pub(crate) use crate::game::fe::fepopup3::callee::MC_MANAGER_SET_MODE;
    pub(crate) use crate::game::fe::fepopup3::callee::MC_MANAGER_END_OPERATION_2 as MC_MANAGER_END_OPERATION_3;
    pub(crate) use crate::game::fe::fepopup3::callee::MC_MANAGER_REQUEST_CARD_INFO_2;
    pub(crate) use crate::game::fe::fepopup3::callee::LOBBY_MAN_GET_ROOM_NAME;
    pub(crate) use crate::game::fe::fepopup3::callee::LOBBY_MAN_MOVE_TO_ROOM;
    pub(crate) use crate::game::fe::fepopup3::callee::LOBBY_MAN_LOOKUP_PEER_ADDR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUILD_BUDDY_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SET_SELECTED_USER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_SELECTED_USER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SHOW_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ON_CHILD_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GOTO_NEXT_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::HANDLE_NET_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ONLINE_CHARACTER_SELECT_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHANNEL_GET_TRACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LIST_GET_ENTRY_TYPE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FIND_FREE_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LOAD_BANK_ASYNC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REQUEST_IMAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SEND_PACKET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_SAY_20B9;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_BOARD_LOOPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::START_RIDER_BOARD_LOOPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_RACE_MAN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LEVEL_TO_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AUDIO_SYSTEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_END_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SET_CONTROLLER_INDEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD as SND_BANK_UNLOAD_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_STATE_CTOR_2_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_BASE_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_ON_CHILD_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_HAS_RENDER_SLOT;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_CLEAR_HUD_MESSAGES;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_TRICK;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_COMBO;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_BANK_PENDING_SCORE;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_GET_SYNC_CHECKSUM;
    pub(crate) use crate::game::main::gamemode::callee::CBE_GET_BE;
    pub(crate) use crate::game::main::gamemode::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamemode::callee::BE_NEW_PLAYER_INT_GET_SLOT as BE_NEW_PLAYER_INT_GET_SLOT_2;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GET_MEDAL_REWARD_ID;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_IS_REWARD_GRANTED;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GRANT_REWARD;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UNLOAD;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2;
    pub(crate) use crate::game::main::gamemode::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF as UI_OBJECT_SHOW_CLAMP_ALPHA_HALF_3;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_RIDER_PREVIEW_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::FE_ASYNC_MANAGER_SET_FILE_STATUS;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_FIND_SNAPSHOT;
    pub(crate) use crate::game::main::gamemode::callee::RUN_LIST_SEEK;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_LIST_ON_SEQ_ADVANCE;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID as SCRIPT_ENGINE_GET_SEQ_PROCESS_ID_2;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_REQUEST_INSTANT_REPLAY;
    pub(crate) use crate::game::main::gamemode::callee::USTR_TO_ASCII as USTR_TO_ASCII_2;
    pub(crate) use crate::game::main::gamemode::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::main::gamemode::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::main::gamemode::callee::WSCRIPT_MAN_RESET_COLLECT_SET;
    pub(crate) use crate::game::main::gamemode::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::main::gamemode::callee::CBX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE as EA_MEM_FREE_2;
    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE as BUILTIN_DELETE_3;
    pub(crate) use crate::game::main::gamemode::callee::MODIFIER_BLOCK_HAS_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::RAIL_MODIFIER_CTOR;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_SET_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MULTI_PARTICLE_DTOR as MULTI_PARTICLE_DTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_CTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::main::gamemode::callee::TEX_MAN_UNLOCK_VRAM;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_FIRST as UI_LIST_FIRST_2;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_ACCEPT_VISITOR;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_DTOR_BODY as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::main::gamemode::callee::CUI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_RESET;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_FREE_BUFFERS;
    pub(crate) use crate::game::main::gamemode::callee::STR_HASH as STR_HASH_2;
    pub(crate) use crate::game::main::gamemode::callee::SPRINTF as SPRINTF_3;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET as MEMSET_2;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;
    pub(crate) use crate::game::main::gamemode::callee::MEMCPY;
    pub(crate) use crate::game::main::gamemode::callee::STRLEN as STRLEN_2;
    pub(crate) use crate::game::main::gamemode::callee::STRCPY as STRCPY_4;
    pub(crate) use crate::game::main::gamemode::callee::CVT_FLOAT_TO_INT;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::main::gamemode::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE as SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
    pub(crate) use crate::game::main::gamemode::callee::FUNC_0028F140;
    pub(crate) use crate::game::main::gamemode::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::main::gamemode::callee::AI_RAND;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_IS_CONNECTED;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_RECONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN_DISCONNECT as LOBBY_MAN_DISCONNECT_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_SET_QUALITY_ICONS;
    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_HANDLE_NET_EVENT as NET_FE_STATE_HANDLE_NET_EVENT_2;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_CREATE_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_JOIN_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_CTOR_5 as NET_FE_STATE_CTOR_10;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;
    pub(crate) use crate::game::movie::movieplayer::callee::STRCPY_2 as STRCPY_5;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_1;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_UNLOAD_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT as SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_RUN_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_DATA_GET_INFO as SCRIPT_DATA_GET_INFO_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_FIND_SOUND_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_SET_SELECTION;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_ON_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_SCORE_INT_UPDATE_BEST_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_RESET_STATS as RIDER_METRIX_RESET_STATS_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_ON_OPERATION_DONE;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_END_OPERATION_4;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_SET_MODE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_RESET_LAYOUT;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_CONFIG;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_BACKGROUND_SCALE;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILTIN_DELETE_4;
    pub(crate) use crate::game::movie::movieplayer::callee::PARENT_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::ROLLER_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::movie::movieplayer::callee::MULTI_SPLINE_MODIFIER_ADD_REF;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_ALLOC_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_EQUIP_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_BUY_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::STREAM_BUFFER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_MAN_CREATE_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESS_COUNT;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESSES;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_ON_SPLIT_MARKER;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_CTOR_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_DTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::FONT_PARSE_GLYPHS;
    pub(crate) use crate::game::movie::movieplayer::callee::SPRINTF_2 as SPRINTF_4;
    pub(crate) use crate::game::movie::movieplayer::callee::FILE_LOAD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_STORE_WORD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_LOAD_TEXTURES;
    pub(crate) use crate::game::movie::movieplayer::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES as FLEX_BRIDGE_ALLOC_SPRING_NODES_2;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_BE as CBE_GET_BE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_INTERFACE;
    pub(crate) use crate::game::movie::movieplayer::callee::C_OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::EVENT_SELECT_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FLAG_MANAGER_GROUP_INIT_KEY;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PDA_TEMPLATE_OPEN;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PROFILE_SAVE_PDA_INIT_SLOT_LABELS;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_STATE as GAME_MODULE_REQUEST_STATE_3;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_OVERLAY_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR_5;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_MGR_ADD_BACKGROUND_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::BEREWARD_INT_IS_PEAK_COMPLETE;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_REMOVE_MESSAGE;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_COUNT_GROUP_MESSAGES;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_GET_MESSAGE_COUNT;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_REFRESH_LIST;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_COUNT_TEXT;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_CLEAR_SLOTS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_INSTALL_VIF_VU_HANDLERS;
    pub(crate) use crate::game::main::gamestate::callee::GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::main::gamestate::callee::VD_TEXTURE_SINK_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INSTANCE_3;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INTERFACE_3;
    pub(crate) use crate::game::main::gamestate::callee::BE_ECON_INT_GET_TOTAL_EARNED;
    pub(crate) use crate::game::main::gamestate::callee::BOLTS_FIND_BOLT_INDEX;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_CMD_QUEUE_AT_2;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ as SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_2;
    pub(crate) use crate::game::main::gamestate::callee::SECTION_ALLOCATOR_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR_6;
    pub(crate) use crate::game::main::gamestate::callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_STOP_3 as SCRIPT_ENGINE_LIST_STOP_2;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_RESET_RACE_SOUNDS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_SCRIPT_OBJ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_IS_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_IS_OBJ_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_HAS_BINDING;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_INSTANCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_NEXT as MENU_SYS_PAGE_SELECT_NEXT_2;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_PREV;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_IS_DISABLED as MENU_SYS_ITEM_IS_DISABLED_2;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_PAUSE_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_RESUME_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::GAME_ANIM_MAP_UNREGISTER_BANK;
    pub(crate) use crate::game::main::gamestate::callee::ANIM_DATA_BASE_FREE_CHAN_CACHES;
    pub(crate) use crate::game::main::gamestate::callee::ADF_DATA_SOURCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::C_KEYBOARD_POPUP_SET_KEY_ENABLED;
    pub(crate) use crate::game::main::gamestate::callee::RIDER_CLEAR_NEARBY_LIGHTS;
    pub(crate) use crate::game::main::gamestate::callee::SNDSTRM_CLOSE;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4 as EA_MEM_FREE_3;
    pub(crate) use crate::game::main::gamestate::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_5;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT_8;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_STOP_HANDLE as BANK_MONITOR_STOP_HANDLE_2;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_FIRST_2 as UI_LIST_FIRST_3;
    pub(crate) use crate::game::main::gamestate::callee::C_GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_ALLOC_NAMED_3;
    pub(crate) use crate::game::main::gamestate::callee::MEM_MOVE;
    pub(crate) use crate::game::main::gamestate::callee::STR_COPY;
    pub(crate) use crate::game::main::gamestate::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::main::gamestate::callee::STR_N_COPY;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_SPHERE_TREE;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_AABB;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_MAKE_ID;
    pub(crate) use crate::game::main::gamestate::callee::MEM_FREE as MEM_FREE_3;
    pub(crate) use crate::game::main::gamestate::callee::STR_HASH as STR_HASH_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RESET_FOR_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INIT_FROM_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::DYNAMIC_EMITTER_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_END_OPERATION;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_ON_OPERATION_DONE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUY_POPUP_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::CLEAR_NEW_REWARDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_RECORD_BEST_JUMP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_ADD_BOOST_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_APPLY_SCRIPT_TRACK;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_UPDATE_SCRIPT_SOUNDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SNOW_BOX_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_CTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::VRAM_HEAP_INIT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TEX_MAN_INIT_TABLE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_COPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GROW_LINES;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_EVENT_STAT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2 as UI_STATE_CTOR_2_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOAD_INDICATOR_RENDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST as UI_LIST_FIRST_5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SECTION_SELECT_PEAK_LIGHTING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INSTANCE_GET_NODE_MATRIX as INSTANCE_GET_NODE_MATRIX_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_PROCESS_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_GET_ROOM_NAME;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::FE_ASYNC_SET_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::POPUP_CONFIRM_REBUILD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_HASH as STR_HASH_4;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING as UI_TEXT_SET_STRING_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT as UI_SCREEN_FIND_OBJECT_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM as UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM as UI_OBJECT_SET_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SET_ITEM_YEAR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_ASCII;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2 as UI_ANIM_SET_CREATE_ANIM_2_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MSG_PRINTF;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STRLEN as STRLEN_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::QSORT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2 as GAME_MODULE_REQUEST_STATE_4;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_API_SEND;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GATHER_CAM_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MEM_REPORT_STATS_LOCKED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED_4;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SERIALIZE_READ_U32;
    pub(crate) use crate::game::ai::rider::callee::NET_FE_STATE_ON_CHILD_EVENT as NET_FE_STATE_ON_CHILD_EVENT_11;
    pub(crate) use crate::game::ai::rider::callee::NET_ACCOUNT_MEM_CTOR;
    pub(crate) use crate::game::ai::rider::callee::SHOW_CHAR_STATS;
    pub(crate) use crate::game::ai::rider::callee::START_CARD_SCAN;
    pub(crate) use crate::game::ai::rider::callee::SHOW_CHECKING_CARD_POPUP;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_ACCOUNT_NAME;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_PASSWORD;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_CONFIRM_PASSWORD;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_EMAIL;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_SHARE_INFO;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_INFORM;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_BACKUP_ACCOUNT;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_ALL_FIELDS_FILLED;
    pub(crate) use crate::game::ai::rider::callee::LOGIN_ON_ACCOUNT_MEM_POPUP;
    pub(crate) use crate::game::ai::rider::callee::LOGIN_ON_DISCONNECT_POPUP;
    pub(crate) use crate::game::ai::rider::callee::MESSAGE_CENTER_GET_LIST_INDEX;
    pub(crate) use crate::game::ai::rider::callee::MESSAGE_CENTER_OPEN_VIEW_POPUP;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_SEND_REQUEST as LOBBY_MAN_SEND_REQUEST_2;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_GET_LANGUAGE;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_COUNT as BUDDY_MAN_GET_ROSTER_COUNT_3;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_ENTRY as BUDDY_MAN_GET_ROSTER_ENTRY_3;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_GET_SURFACE_SOUND_TYPE;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_BASE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_SURFACE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_EDGE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_PLAY_FE_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_TRICK_METER_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_OBJ_CTOR as SCRIPT_OBJ_CTOR_2;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_OBJ_DEACTIVATE as SCRIPT_OBJ_DEACTIVATE_2;
    pub(crate) use crate::game::ai::rider::callee::CODEC_POP_FREE_FRAME;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::ai::rider::callee::BUY_POPUP_INIT_BUY_BOLT;
    pub(crate) use crate::game::ai::rider::callee::RIDER_GET_START_DIST_TO_FINISH;
    pub(crate) use crate::game::ai::rider::callee::BE_GET_RIDER_PROFILE_2 as BE_GET_RIDER_PROFILE_4;
    pub(crate) use crate::game::ai::rider::callee::BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_3;
    pub(crate) use crate::game::ai::rider::callee::IN_SEQ_PLAY;
    pub(crate) use crate::game::ai::rider::callee::CROWD_MAN_GROUP_FREE;
    pub(crate) use crate::game::ai::rider::callee::RAIL_MAN_ENABLE_RAIL;
    pub(crate) use crate::game::ai::rider::callee::SYNC_PROCESS_DTOR;
    pub(crate) use crate::game::ai::rider::callee::TEX_MAN_EVICT_TEXTURE;
    pub(crate) use crate::game::ai::rider::callee::VRAM_HEAP_FREE_TEXTURE_BLOCKS;
    pub(crate) use crate::game::ai::rider::callee::GAME_UI_OPEN_SCREEN;
    pub(crate) use crate::game::ai::rider::callee::WORLD_CACHE_RESET;
    pub(crate) use crate::game::ai::rider::callee::WORLD_CACHE_LOAD;
    pub(crate) use crate::game::ai::rider::callee::STREAMER_GET_SECTION_COUNT;
    pub(crate) use crate::game::ai::rider::callee::WSCRIPT_CACHE_ALLOC;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_FROM_ASCII_2;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_SPRINTF;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WIDGET_ALIGN;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WRAPPED_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::RIDER_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_MISSION_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED_6;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT_2 as WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_MAN_CLEAR;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_SET_EVENT_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_UPDATE as UI_STATE_UPDATE_4;
    pub(crate) use crate::game::bx::bxstring2::callee::OPTIONS_SET_VIBRATION;
    pub(crate) use crate::game::bx::bxstring2::callee::STR_HASH as STR_HASH_6;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_FIND_FIRST_SELECTABLE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_SET_SELECTION as UI_MENU_SET_SELECTION_2;
    pub(crate) use crate::game::bx::bxstring2::callee::STRLEN as STRLEN_4;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_DEQUEUE_SECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_POP_FREE_NODE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_ADD_ITEM_STR_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_FIND_ITEM_BY_VALUE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_GET_POPUP as NET_FE_STATE_GET_POPUP_5;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_SHOW_POPUP as NET_FE_STATE_SHOW_POPUP_4;
    pub(crate) use crate::game::bx::bxstring2::callee::MESSAGE_CENTER_GET_INBOX as MESSAGE_CENTER_GET_INBOX_4;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_CTOR_COPY;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_DTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT as SPEECH_SYSTEM_SET_EVENT_CONTEXT_2;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_SPEECH_CONTEXT_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_SKIP_WHITESPACE;
    pub(crate) use crate::game::bx::bxstring2::callee::CHUNK_RESOLVE_RELOCATE_DMA;
    pub(crate) use crate::game::bx::bxstring2::callee::FLAG_MANAGER_GROUP_BUILD_MESH;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_6;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD as BE_GET_CHAR_PROFILE_EVENT_RECORD_2;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::READ_PRIVATE_MESSAGE_SET_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_MANAGER;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_SCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_READ_FLOAT_3 as PATH_DATA_READ_FLOAT_2;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_DESIRED_EYE;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_LOOK_AT;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_INIT_FROM_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::MEMSET as MEMSET_3;
    pub(crate) use crate::game::bx::bxstring2::callee::QSORT as QSORT_2;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN3D_GROUP_BUILD_MATRICES;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAM_CONTROLLER_RESTORE_DEFAULT;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_LIST_START as SCRIPT_ENGINE_LIST_START_2;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_SET_SKIP_DELAY;
    pub(crate) use crate::game::bx::bxstring2::callee::CUTSCENE_TRANSITION_MAN_FORCE_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_FIND_STATE as UI_STATE_MGR_FIND_STATE_3;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_SHUTDOWN as UI_STATE_SHUTDOWN_3;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_FREE as EA_MEM_FREE_6;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_LEN as USTR_LEN_3;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_CPY as USTR_CPY_3;
    pub(crate) use crate::game::bx::bxstring2::callee::UITEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::fepopup::callee::C00113f88;
    pub(crate) use crate::game::fe::fepopup::callee::C00115ab0;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe78;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe98;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fec8;
    pub(crate) use crate::game::fe::fepopup::callee::C00125108;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a080;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a0e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0014bdb8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d448;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d988;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d998;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dc80;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dd58;
    pub(crate) use crate::game::fe::fepopup::callee::C00150928;
    pub(crate) use crate::game::fe::fepopup::callee::C00150b48;
    pub(crate) use crate::game::fe::fepopup::callee::C00151c48;
    pub(crate) use crate::game::fe::fepopup::callee::C001535c0;
    pub(crate) use crate::game::fe::fepopup::callee::C001543e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00156988;
    pub(crate) use crate::game::fe::fepopup::callee::C00156a60;
    pub(crate) use crate::game::fe::fepopup::callee::C001577e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00157bf0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158910;
    pub(crate) use crate::game::fe::fepopup::callee::C001589b0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158a50;
    pub(crate) use crate::game::fe::fepopup::callee::C00158af0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158c80;
    pub(crate) use crate::game::fe::fepopup::callee::C00158d58;
    pub(crate) use crate::game::fe::fepopup::callee::C00159170;
    pub(crate) use crate::game::fe::fepopup::callee::C00159cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C0015a2e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c6b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c718;
    pub(crate) use crate::game::fe::fepopup::callee::C00176890;
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
    pub(crate) use crate::game::fe::fepopup::callee::C00225068;
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
    pub(crate) use crate::game::fe::fepopup::callee::C00275498;
    pub(crate) use crate::game::fe::fepopup::callee::C002758c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00275b98;
    pub(crate) use crate::game::fe::fepopup::callee::C00275cf8;
    pub(crate) use crate::game::fe::fepopup::callee::C00278308;
    pub(crate) use crate::game::fe::fepopup::callee::C00278e50;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f38;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f68;
    pub(crate) use crate::game::fe::fepopup::callee::C00281ed8;
    pub(crate) use crate::game::fe::fepopup::callee::C00283228;
    pub(crate) use crate::game::fe::fepopup::callee::C00283518;
    pub(crate) use crate::game::fe::fepopup::callee::C002837c8;
    pub(crate) use crate::game::fe::fepopup::callee::C0028b180;
    pub(crate) use crate::game::fe::fepopup::callee::C0028bb10;
    pub(crate) use crate::game::fe::fepopup::callee::C0028e8c0;
    pub(crate) use crate::game::fe::fepopup::callee::C002913d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003457c8;
    pub(crate) use crate::game::fe::fepopup::callee::C00358020;
    pub(crate) use crate::game::fe::fepopup::callee::C00362478;
    pub(crate) use crate::game::fe::fepopup::callee::C003625c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00362660;
    pub(crate) use crate::game::fe::fepopup::callee::C003626d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003627a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00364cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C00366ce0;
    pub(crate) use crate::game::fe::fepopup::callee::C00368170;
    pub(crate) use crate::game::fe::fepopup::callee::C003684f0;
    pub(crate) use crate::game::fe::fepopup::callee::C00368970;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f460;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f668;
    pub(crate) use crate::game::fe::fepopup::callee::C0039c870;
    pub(crate) use crate::game::fe::fepopup::callee::C0039ff78;
    pub(crate) use crate::game::fe::fepopup::callee::C003b1300;
    pub(crate) use crate::game::fe::fepopup::callee::C003b2360;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMSET_3 as MEMSET_4;
    pub(crate) use crate::game::dirtysock::tags2::callee::STR_HASH_4 as STR_HASH_7;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_6 as SPRINTF_8;
    pub(crate) use crate::game::dirtysock::tags2::callee::FORMAT_TAGGED;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_RESET_PROFILE_CHARACTERS_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_NEW_PLAYER_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_CHAR_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_BAGT_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_MEM_MAN_ALLOC as C_MEM_MAN_ALLOC_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::dirtysock::tags2::callee::TAG_FIELD_GET_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBE_GET_INTERFACE as CBE_GET_INTERFACE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_CHAR_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_ALT_CHAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::GM_PICK_OPPONENTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_SET_MODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_GET_PRODUCT_CODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_GET_SAVE_TYPE_TAG;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_00152BA8;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_00152948;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_002C2540;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_002C26D0;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_IS_FLAG_SET_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_SET_FLAG_2 as MESSAGE_CENTER_SET_FLAG_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::STR_INTERN;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_0039E2A0;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNPRINTF;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBYMAN_SEND_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_1;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_4;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FLUSH;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_5;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_PA_SPEECH_ON;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_REPLAY_PLAYING;
    pub(crate) use crate::game::dirtysock::tags2::callee::RIDER_123168;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_RACE_TYPE_4OR5;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_ADD_TIMER_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPEECH_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_LANGUAGE_MASK as GET_LANGUAGE_MASK_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_RIDER_VOICE_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_FINISH_SPEECH_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPEECH_QUEUE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FIND;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_REMOVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_DEACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUPMAN_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::TEXT_OBJ;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEASURE_PLAIN;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEASURE_WRAP;
    pub(crate) use crate::game::dirtysock::tags2::callee::ANIM_GET_LAYER_SEQUENCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_LOCAL_PLAYER;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_TRANSITION_MAN;
    pub(crate) use crate::game::dirtysock::tags2::callee::TRANSITION_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::EFFECT_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::EFFECT_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::TRANSITION_START;


    // Front-end state machine (shard 3).
    pub const NET_FE_STATE_ON_CHILD_EVENT_4: Callee = Callee {
        addr: 0x001a_97b8,
        f: ee::s03::f_001a97b8,
    };

    pub const NET_FE_STATE_CTOR_4: Callee = Callee {
        addr: 0x001a_8500,
        f: ee::s03::f_001a8500,
    };

    pub const CHAT_ROOM_REFRESH_PLAYER_LIST: Callee = Callee {
        addr: 0x001b_2378,
        f: ee::s03::f_001b2378,
    };

    pub const NET_ACCOUNT_MEM_SET_STATE: Callee = Callee {
        addr: 0x001b_70d0,
        f: ee::s03::f_001b70d0,
    };

    pub const OV_MESSAGE_CENTER_REFRESH_LIST: Callee = Callee {
        addr: 0x001e_48b0,
        f: ee::s04::f_001e48b0,
    };

    pub const OV_MESSAGE_CENTER_UPDATE_COUNT_TEXT: Callee = Callee {
        addr: 0x001e_4758,
        f: ee::s04::f_001e4758,
    };


    // Memory card / app modules (shard 6).
    pub const APP_MODULE_CTOR: Callee = Callee {
        addr: 0x0023_1cd0,
        f: ee::s06::f_00231cd0,
    };

    pub const MC_MANAGER_UPDATE_STATES: Callee = Callee {
        addr: 0x0023_d7d8,
        f: ee::s06::f_0023d7d8,
    };

    pub const MC_MANAGER_UPDATE_MODE1: Callee = Callee {
        addr: 0x0023_d660,
        f: ee::s06::f_0023d660,
    };

    pub const NET_DISCONNECT_ALL: Callee = Callee {
        addr: 0x0025_5f50,
        f: ee::s06::f_00255f50,
    };


    // Replay / script engine (shard 7).
    pub const REPLAY_CACHE_RESET: Callee = Callee {
        addr: 0x0026_d168,
        f: ee::s07::f_0026d168,
    };

    pub const WORLD_STATE_WRITE: Callee = Callee {
        addr: 0x0026_d818,
        f: ee::s07::f_0026d818,
    };

    pub const SCRIPT_LIST_STOP: Callee = Callee {
        addr: 0x0027_6868,
        f: ee::s07::f_00276868,
    };

    pub const SCRIPT_ENGINE_GET_PROCESS: Callee = Callee {
        addr: 0x0028_2cb0,
        f: ee::s07::f_00282cb0,
    };

    pub const IN_SEQ_INSTANCE_UPDATE: Callee = Callee {
        addr: 0x0027_4a30,
        f: ee::s07::f_00274a30,
    };

    pub const IN_SEQ_INSTANCE_PAUSE: Callee = Callee {
        addr: 0x0027_49e8,
        f: ee::s07::f_002749e8,
    };

    pub const SCRIPT_ENGINE_ALLOC_PROCESS: Callee = Callee {
        addr: 0x0028_2c38,
        f: ee::s07::f_00282c38,
    };

    pub const IN_SEQ_INSTANCE_SET_USER_DATA: Callee = Callee {
        addr: 0x0027_4c10,
        f: ee::s07::f_00274c10,
    };

    pub const SCRIPT_ENGINE_GET_INSTANCE_2: Callee = Callee {
        addr: 0x0027_d2e8,
        f: ee::s07::f_0027d2e8,
    };

    pub const SCRIPT_ENGINE_ACTIVATE_BINDING: Callee = Callee {
        addr: 0x0027_d400,
        f: ee::s07::f_0027d400,
    };

    pub const SCRIPT_CMD_QUEUE_IS_FULL: Callee = Callee {
        addr: 0x0028_3c58,
        f: ee::s07::f_00283c58,
    };


    // Audio (shard 8).
    pub const BANK_MONITOR_PAUSE_ALL: Callee = Callee {
        addr: 0x002a_d2a8,
        f: ee::s08::f_002ad2a8,
    };

    pub const RIDER_SAY_20D3: Callee = Callee {
        addr: 0x002a_1138,
        f: ee::s08::f_002a1138,
    };

    pub const RIDER_SAY_20D4: Callee = Callee {
        addr: 0x002a_1280,
        f: ee::s08::f_002a1280,
    };

    pub const USTR_COPY: Callee = Callee {
        addr: 0x002c_2508,
        f: ee::s08::f_002c2508,
    };


    // Menu items (shard 9).
    pub const MENU_SYS_ITEM_IS_VISIBLE: Callee = Callee {
        addr: 0x002c_a378,
        f: ee::s09::f_002ca378,
    };

    pub const MENU_SYS_ITEM_IS_DISABLED: Callee = Callee {
        addr: 0x002c_a3b0,
        f: ee::s09::f_002ca3b0,
    };

    pub const MENU_SYS_ITEM_CTOR: Callee = Callee {
        addr: 0x002c_a258,
        f: ee::s09::f_002ca258,
    };

    pub const NET_FE_STATE_CTOR_5: Callee = Callee {
        addr: 0x001a_8500,
        f: ee::s03::f_001a8500,
    };


    pub const UI_LIST_CTOR_2: Callee = Callee {
        addr: 0x0039_77e8,
        f: ee::s13::f_003977e8,
    };

    pub const UI_LIST_RESET: Callee = Callee {
        addr: 0x0039_7930,
        f: ee::s13::f_00397930,
    };

    pub const OPERATOR_NEW_NAMED: Callee = Callee {
        addr: 0x0031_7e30,
        f: ee::s10::f_00317e30,
    };

    pub const CBX_STRING_ALLOC_COPY: Callee = Callee {
        addr: 0x0031_81e8,
        f: ee::s10::f_003181e8,
    };

    pub const CBX_STRING_CTOR_COPY: Callee = Callee {
        addr: 0x0031_7fe8,
        f: ee::s10::f_00317fe8,
    };

    pub const CBX_STRING_DTOR: Callee = Callee {
        addr: 0x0031_8260,
        f: ee::s10::f_00318260,
    };

    pub const MEMCARD_INSTALL_CALLBACKS: Callee = Callee {
        addr: 0x001d_5488,
        f: ee::s04::f_001d5488,
    };

    pub const REPLAY_SET_STATE_2: Callee = Callee {
        addr: 0x0026_f4a0,
        f: ee::s07::f_0026f4a0,
    };

    pub const PLAYBACK_RESTORE_WORLD: Callee = Callee {
        addr: 0x0026_cc18,
        f: ee::s07::f_0026cc18,
    };

    pub const WORLD_CHUNK_RESOLVE_HOOK: Callee = Callee {
        addr: 0x003a_d230,
        f: ee::s13::f_003ad230,
    };

    pub const APP_MODULE_RENDER_FADE: Callee = Callee {
        addr: 0x0023_1d60,
        f: ee::s06::f_00231d60,
    };

    pub const FLEX_RAIL_SAVE: Callee = Callee {
        addr: 0x0034_8b40,
        f: ee::s11::f_00348b40,
    };

    pub const SPLINE_MODIFIER_SET_SPEED: Callee = Callee {
        addr: 0x0035_9688,
        f: ee::s11::f_00359688,
    };

    pub const AUTOSAVE_UPDATE_ICON: Callee = Callee {
        addr: 0x0021_2360,
        f: ee::s05::f_00212360,
    };

    pub const FE_UNLOAD_ANIM_DATA: Callee = Callee {
        addr: 0x001a_2bc0,
        f: ee::s03::f_001a2bc0,
    };

    pub const FE_UNLOAD_RIDER_PREVIEWS: Callee = Callee {
        addr: 0x001a_04c8,
        f: ee::s03::f_001a04c8,
    };

    pub const BOLT_FILE_CACHE_SHUTDOWN: Callee = Callee {
        addr: 0x0019_cf40,
        f: ee::s03::f_0019cf40,
    };

    pub const WORLD_OBJ_CTOR_2: Callee = Callee {
        addr: 0x0034_fb00,
        f: ee::s11::f_0034fb00,
    };

    pub const WORLD_OBJ_DTOR_2: Callee = Callee {
        addr: 0x0034_fbf0,
        f: ee::s11::f_0034fbf0,
    };

    pub const WORLD_OBJ_RUN_KILL_SCRIPT_2: Callee = Callee {
        addr: 0x0034_fcc0,
        f: ee::s11::f_0034fcc0,
    };

    pub const WORLD_OBJ_REPLACE_WITH_DEAD_NODE: Callee = Callee {
        addr: 0x0034_fe28,
        f: ee::s11::f_0034fe28,
    };

    pub const MOD_OBJ_SAVE: Callee = Callee {
        addr: 0x0035_6b30,
        f: ee::s11::f_00356b30,
    };

    pub const FE_LOCALE_LOAD_TABLE: Callee = Callee {
        addr: 0x0019_59a8,
        f: ee::s02::f_001959a8,
    };

    pub const BE_LIBRARY_GET_INSTANCE_2: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };

    pub const BE_LIBRARY_GET_INTERFACE_2: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::f_0014dd58,
    };

    pub const BE_SCORE_INT_GET_HIGH_SCORE_TABLE: Callee = Callee {
        addr: 0x0014_ab20,
        f: ee::s01::f_0014ab20,
    };

    pub const BE_OPTION_INT_SWAP_PLAYER_FLAGS: Callee = Callee {
        addr: 0x0014_eb08,
        f: ee::s01::f_0014eb08,
    };

    pub const NET_FE_STATE_CTOR_9: Callee = Callee {
        addr: 0x001a_8500,
        f: ee::s03::f_001a8500,
    };

    pub const UI_OBJECT_UPDATE_ANIM: Callee = Callee {
        addr: 0x003a_0000,
        f: ee::s13::f_003a0000,
    };

    pub const INI_PARSER_GET_CHAR: Callee = Callee {
        addr: 0x002a_78e0,
        f: ee::s08::f_002a78e0,
    };

    pub const INI_PARSER_EXPECT: Callee = Callee {
        addr: 0x002a_7ef0,
        f: ee::s08::f_002a7ef0,
    };

    pub const INI_PARSER_READ_QUOTED_STRING: Callee = Callee {
        addr: 0x002a_7cf0,
        f: ee::s08::f_002a7cf0,
    };

    pub const AUDIO_STOP_WIPEOUT_LOOP: Callee = Callee {
        addr: 0x0029_6e20,
        f: ee::s08::f_00296e20,
    };

    pub const AUDIO_STOP_WIPEOUT_LOOP2: Callee = Callee {
        addr: 0x0029_7438,
        f: ee::s08::f_00297438,
    };

    pub const AUDIO_STOP_TRICK_METER: Callee = Callee {
        addr: 0x0029_b3c0,
        f: ee::s08::f_0029b3c0,
    };

    pub const LOBBY_MENU_HIDE_ROOM_TABS: Callee = Callee {
        addr: 0x001b_dfd8,
        f: ee::s03::f_001bdfd8,
    };


    pub const SCRIPT_CMD_QUEUE_AT_3: Callee = Callee { addr: 0x0028_3d70, f: ee::s07::f_00283d70 };

    pub const SCRIPT_LIST_IS_MOVIE_ENTRY_2: Callee =
        Callee { addr: 0x0027_7dd8, f: ee::s07::f_00277dd8 };

    pub const SCRIPT_ENGINE_GET_PROCESS_STATE: Callee =
        Callee { addr: 0x0028_2798, f: ee::s07::f_00282798 };


    pub const UI_SCREEN_FIND_OBJECT_2: Callee = Callee { addr: 0x0039_d860, f: ee::s13::f_0039d860 };

    pub const UI_ANIM_SET_CREATE_ANIM: Callee =
        Callee { addr: 0x0039_7140, f: ee::s13::f_00397140 };

    pub const UI_OBJECT_SET_ANIM: Callee = Callee { addr: 0x0039_fcc8, f: ee::s13::f_0039fcc8 };


    pub const BX_STRING_ALLOC_BUFFER: Callee =
        Callee { addr: 0x0031_8040, f: ee::s10::f_00318040 };


    pub const BOLT_ON_FX_INIT_HANDLE: Callee =
        Callee { addr: 0x002e_c9d0, f: ee::s09::f_002ec9d0 };

    pub const BOLT_ON_FX_CLEAR_EMITTERS: Callee =
        Callee { addr: 0x002e_caf0, f: ee::s09::f_002ecaf0 };


    pub const WORLD_BLOCK_POOL_INIT: Callee = Callee { addr: 0x003a_7598, f: ee::s13::f_003a7598 };

    pub const WORLD_MEMMAN_CREATE_SECTION_ALLOCATOR: Callee =
        Callee { addr: 0x003a_7b18, f: ee::s13::f_003a7b18 };

    pub const SECTION_TABLE_CTOR: Callee = Callee { addr: 0x003a_7c88, f: ee::s13::f_003a7c88 };


    pub const UI_LIST_FIRST_4: Callee = Callee { addr: 0x0039_7840, f: ee::s13::f_00397840 };

    pub const UI_OBJECT_UPDATE_ANIM_2: Callee = Callee { addr: 0x003a_0000, f: ee::s13::f_003a0000 };


    pub const WORLD_OBJ_CTOR_2_2: Callee = Callee { addr: 0x0034_fb90, f: ee::s11::f_0034fb90 };

    pub const FLEX_BRIDGE_SPRINGS_LOAD: Callee =
        Callee { addr: 0x0035_3e80, f: ee::s11::f_00353e80 };

    pub const FLEX_BRIDGE_BUILD_MESH: Callee =
        Callee { addr: 0x0034_66a0, f: ee::s11::f_003466a0 };

    pub const FLEX_BRIDGE_UPDATE_VERTS: Callee =
        Callee { addr: 0x0034_75d8, f: ee::s11::f_003475d8 };

    pub const INSTANCE_GET_NODE_MATRIX: Callee =
        Callee { addr: 0x0034_fed8, f: ee::s11::f_0034fed8 };


    pub const GAME_STATE_FINISH_CLEAR_FLAG40: Callee =
        Callee { addr: 0x0024_4880, f: ee::s06::f_00244880 };


    pub const LUNO_VM_REGISTER_DTOR: Callee = Callee { addr: 0x0022_5b90, f: ee::s05::f_00225b90 };


    pub const UI_OBJECT_DTOR: Callee = Callee { addr: 0x0039_fc48, f: ee::s13::f_0039fc48 };


    pub const CAM_CONTROLLER_DTOR: Callee = Callee { addr: 0x0015_cc10, f: ee::s01::f_0015cc10 };


    pub const GAME_ANIM_MAP_SET_HANDLE: Callee =
        Callee { addr: 0x0031_4ef0, f: ee::s10::f_00314ef0 };


    pub const CAMERA_TRIGGER_LIST_CTOR: Callee =
        Callee { addr: 0x0016_bee8, f: ee::s02::f_0016bee8 };

    pub const ACTIVE_CAMERA_TRIGGER_LIST_CTOR: Callee =
        Callee { addr: 0x0016_c5d0, f: ee::s02::f_0016c5d0 };

    pub const CAMERA_TRIGGER_STACK_CTOR: Callee =
        Callee { addr: 0x0016_d210, f: ee::s02::f_0016d210 };


    pub const NET_FE_STATE_INIT: Callee = Callee { addr: 0x001a_8768, f: ee::s03::f_001a8768 };


    pub const FE_STATE_CREATE_DUPLICATE_ON_SAVE_ACCOUNT_ANSWER: Callee =
        Callee { addr: 0x001b_c818, f: ee::s03::f_001bc818 };

    pub const FE_STATE_CREATE_DUPLICATE_ON_ERROR_POPUP_EVENT: Callee =
        Callee { addr: 0x001b_c728, f: ee::s03::f_001bc728 };

    pub const FE_STATE_CREATE_DUPLICATE_ON_CREATED_POPUP_EVENT: Callee =
        Callee { addr: 0x001b_c990, f: ee::s03::f_001bc990 };

    pub const NET_FE_STATE_ON_CHILD_EVENT_9: Callee =
        Callee { addr: 0x001a_97b8, f: ee::s03::f_001a97b8 };


    pub const FE_STATE_LOBBY_MENU_SHOW_ROOM_TAB: Callee =
        Callee { addr: 0x001b_df60, f: ee::s03::f_001bdf60 };

    pub const FE_STATE_LOBBY_MENU_UPDATE_PLAYERS_IN_ROOM: Callee =
        Callee { addr: 0x001b_e3a8, f: ee::s03::f_001be3a8 };

    pub const FE_STATE_LOBBY_MENU_REFRESH_PLAYER_LIST: Callee =
        Callee { addr: 0x001b_e510, f: ee::s03::f_001be510 };

    pub const NET_FE_STATE_UPDATE_3: Callee = Callee { addr: 0x001a_8770, f: ee::s03::f_001a8770 };


    pub const MESSAGE_CENTER_IS_FLAG_SET: Callee =
        Callee { addr: 0x001e_3a30, f: ee::s04::f_001e3a30 };

    pub const MESSAGE_CENTER_EMPTY_TRIGGER: Callee =
        Callee { addr: 0x001e_1548, f: ee::s04::f_001e1548 };


    pub const LOBBY_MAN_SEND_REQUEST: Callee = Callee { addr: 0x0025_b608, f: ee::s06::f_0025b608 };


    pub const INI_PARSER_NEXT_SECTION: Callee = Callee { addr: 0x002a_7b08, f: ee::s08::f_002a7b08 };


    pub const SCRIPT_OBJ_ACTIVATE_2: Callee = Callee { addr: 0x0028_3430, f: ee::s07::f_00283430 };

    pub const SCRIPT_OBJ_CAM_BUILD_MATRIX_FROM_SCRIPT: Callee =
        Callee { addr: 0x0027_d970, f: ee::s07::f_0027d970 };


    pub const SCRIPT_BINDING_CTOR: Callee = Callee { addr: 0x0028_2f60, f: ee::s07::f_00282f60 };

    pub const SCRIPT_OBJ_CTOR: Callee = Callee { addr: 0x0028_3200, f: ee::s07::f_00283200 };


    pub const PS2_MEM_CARD_CLEAR_DIR_CACHE: Callee =
        Callee { addr: 0x002c_5278, f: ee::s08::f_002c5278 };


    pub const CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER: Callee =
        Callee { addr: 0x0016_6c60, f: ee::s02::f_00166c60 };

    pub const CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER: Callee =
        Callee { addr: 0x0016_6550, f: ee::s02::f_00166550 };

    pub const UI_LIST_NODE_IS_SENTINEL_2: u32 = 0x0039_7718;

    pub const UI_SCREEN_FIND_OBJECT_4: u32 = 0x0039_d860;


    pub const PROCESS_LIST_FIRST: u32 = 0x0035_4ea8;

    pub const PROCESS_LIST_NEXT: u32 = 0x0035_4ef8;


    pub const UI_SCREEN_PLAY_FRAME_2: u32 = 0x0039_c870;

    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE_3: u32 = 0x0028_b180;

    pub const AUDIO_SET_TRIGGER: u32 = 0x0029_4f78;


    pub const NET_FE_STATE_ON_CHILD_EVENT_10: u32 = 0x001a_97b8;

    pub const NET_FE_STATE_GET_POPUP_2: u32 = 0x001a_9c50;

    pub const NET_FE_STATE_SHOW_POPUP: u32 = 0x001a_9bc0;

    pub const LOBBY_MAN_HIDE: u32 = 0x0025_b6d8;

    pub const LOBBY_MAN_SHOW: u32 = 0x0025_b9c8;

    pub const NET_FE_STATE_SHOW_DISCONNECTED_POPUP_2: u32 = 0x001a_9930;


    pub const CFS_GET_CHARACTER_INFO: u32 = 0x0039_a738;

    pub const CFS_SHOW_CHARACTER: u32 = 0x001b_1b10;

    pub const CFS_SHOW_OPPONENT_CHAR: u32 = 0x001b_1760;

    pub const CFS_FILL_PLAYER_INFO: u32 = 0x001b_1808;


    pub const MESSAGE_CENTER_SET_FLAG: u32 = 0x001e_3a78;

    pub const OVSCREEN_PDA_TEMPLATE_CLOSE: u32 = 0x0020_d190;

    pub const OVSCREEN_MESSAGE_CENTER_DTOR: u32 = 0x0020_a430;


    pub const BUDDY_MAN_GET_ROSTER_COUNT_2: u32 = 0x0026_2598;

    pub const BUDDY_MAN_GET_ROSTER_ENTRY_2: u32 = 0x0026_25c8;

    pub const STRCMP: u32 = 0x0041_65a8;


    pub const SCRIPT_DATA_GET_BANK_LIST: u32 = 0x0027_c078;

    pub const SCRIPT_ENGINE_UNREGISTER_SCRIPT: u32 = 0x0028_2178;

    pub const SCRIPT_ANIM_BANK_MAN_REMOVE_BANK: u32 = 0x0027_5650;

    pub const SCRIPT_SOUND_BANK_MAN_REMOVE_BANK: u32 = 0x0028_3610;


    pub const SCRIPT_ENGINE_GET_INSTANCE_3: u32 = 0x0027_d2e8;

    pub const SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER: u32 = 0x0027_d0c0;

    pub const SCRIPT_VIEW_OBJ_RESET: u32 = 0x0028_1a50;

    pub const SCRIPT_ENGINE_GET_SEQ_INSTANCE: u32 = 0x0028_2c18;

    pub const SCRIPT_ENGINE_IS_PLAYING_SCRIPT: u32 = 0x0027_9370;

    pub const SCRIPT_VIEW_OBJ_ON_ACTIVATED: u32 = 0x0010_33b0;

    pub const SCRIPT_OBJ_ACTIVATE_3: u32 = 0x0028_3430;


    pub const CUI_TEXT_SCROLL_CLEAR_LINES: u32 = 0x003a_18c0;

    pub const UI_LIST_DELETE_ALL_2: u32 = 0x0039_7948;

    pub const UI_LIST_NODE_DTOR_BODY_3: u32 = 0x003a_4868;

    pub const UI_OBJECT_DTOR_2: u32 = 0x0039_fc48;


    pub const WORLD_CACHE_STREAMER_UPDATE_VIEW: u32 = 0x003a_9d60;

    pub const GAME_MODULE_DTOR: u32 = 0x001a_85d0;

    pub const CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS: u32 = 0x0016_d1b8;

    pub const RIDER_SET_INPUT_ENABLED: u32 = 0x0015_dfd8;

    pub const CAMERA_SET_OWNER: u32 = 0x0016_1f50;


    pub const WORLD_OBJ_CTOR_3: u32 = 0x0034_fb00;

    pub const FLAG_READ_PARAMS: u32 = 0x0034_ac88;

    pub const GET_FLAG_MANAGER: u32 = 0x002d_1cb0;

    pub const FLAG_MANAGER_ADD_FLAG: u32 = 0x0034_c548;


    pub const UI_ANIM_APPLY: u32 = 0x0039_75e0;

    pub const UI_ANIM_ADVANCE: u32 = 0x0039_74b0;

    pub const GAME_PROCESS_DTOR_2: u32 = 0x0035_46c8;


    pub const GAME_MODULE_RESET_RACE: u32 = 0x0023_0180;

    pub const GAME_MODE_MAN_RESET_MODE: u32 = 0x0023_82d8;

    pub const GAME_MODULE_REQUEST_STATE_5: u32 = 0x0023_1250;

    pub const GAME_MODULE_START_NEXT_MODE: u32 = 0x0028_70a0;


    pub const UI_OBJECT_CTOR: u32 = 0x0039_e2a0;

    pub const BE_GET_APP: u32 = 0x0014_dc80;

    pub const BE_INTERFACE_GET_INDEX: u32 = 0x0014_7318;

    pub const BE_GET_BE_2: u32 = 0x001a_1cb8;


    pub const SPRINTF_5: u32 = 0x0041_7828;

    pub const VSPRINTF: u32 = 0x0041_86c8;


    pub const CAM_CONTROLLER_CTOR: u32 = 0x0015_cba0;

    pub const MEM_ALLOC: u32 = 0x0031_7d70;

    pub const MANUAL_CAM_ALGO_CTOR: u32 = 0x0015_fd08;

    pub const EDITOR_CAM_ALGO_CTOR: u32 = 0x0015_ffb0;

    pub const MANUAL_CAM_SELECT_ALGORITHM: u32 = 0x0015_cf80;


    pub const SND_LIST_FIND_BANK: u32 = 0x0028_36d8;

    pub const SND_LIST_FIND_FREE_ENTRY: u32 = 0x0028_3760;

    pub const SND_BANK_MAN_LOAD_BANK_FROM_MEMORY: u32 = 0x0028_be60;


    pub const UI_OBJECT_CTOR_2: u32 = 0x0039_e318;

    pub const UI_STRING_CTOR: u32 = 0x0025_3390;


    pub const AUDIO_GET_MUSIC_TIER: u32 = 0x002b_49e0;

    pub const AUDIO_GET_TRACK_TYPE: u32 = 0x002a_1e20;

    pub const AUDIO_PLAY_MUSIC: u32 = 0x0028_cf98;


    pub const SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2: u32 = 0x0028_2390;

    pub const SCRIPT_ENGINE_KILL_ALL_PROCESSES: u32 = 0x0028_23f0;

    pub const SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2: u32 = 0x0027_d050;

    pub const IN_SEQ_DTOR: u32 = 0x0027_28d0;

    pub const SCRIPT_ENGINE_RESET_PROCESS_TABLE_2: u32 = 0x0028_28b8;


    pub const SPHERE_TREE_COMPUTE_OFFSET: u32 = 0x0032_df28;

    pub const SPHERE_TREE_COLLIDE_SPHERE_NODE: u32 = 0x0032_cdb0;

    pub const SPHERE_TREE_COLLIDE_AABB_NODE: u32 = 0x0032_d028;

    pub const SPHERE_TREE_COLLIDE_SPHERE: u32 = 0x0032_c898;


    pub const WORLD_TRIGGER_INSTANCE_DTOR: u32 = 0x002b_4be0;

    pub const PS2_PAD_CTOR: u32 = 0x0032_6df0;


    pub const WSCRIPT_CACHE_NODE_DTOR: u32 = 0x003a_ca70;


    pub const UI_ENGINE_ADD_SCREEN_BY_HASH_NAME_2: u32 = 0x0039_7cc0;


    pub const SCRIPT_ENGINE_GET_SEQ_INSTANCE_2: u32 = 0x001d_9d48;

    pub const CBE_STRING_CTOR: u32 = 0x0031_84e8;

    pub const NET_FE_STATE_INIT_2: u32 = 0x001a_8768;

    pub const POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID: u32 = 0x001c_57c0;

    pub const UI_OBJECT_ADD_CHILD: u32 = 0x0039_f290;


    pub const BUILTIN_DELETE_6: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::f_00317e50,
    };

    pub const OPERATOR_NEW: Callee = Callee {
        addr: 0x0031_7e30,
        f: ee::s10::f_00317e30,
    };

    pub const EA_MEM_FREE_4: Callee = Callee {
        addr: 0x0031_7e98,
        f: ee::s10::f_00317e98,
    };

    pub const TRANSITION_EFFECT_CTOR: Callee = Callee {
        addr: 0x002e_4d70,
        f: ee::s09::f_002e4d70,
    };

    pub const NET_FE_STATE_GET_POPUP_3: Callee = Callee {
        addr: 0x001a_9c50,
        f: ee::s03::f_001a9c50,
    };

    pub const POPUP_CFG_SET_OPTION_TEXT_ID: Callee = Callee {
        addr: 0x001c_57c0,
        f: ee::s03::f_001c57c0,
    };

    pub const NET_FE_STATE_SHOW_POPUP_2: Callee = Callee {
        addr: 0x001a_9bc0,
        f: ee::s03::f_001a9bc0,
    };

    pub const NET_FE_STATE_ON_CHILD_EVENT_DEFAULT: Callee = Callee {
        addr: 0x001a_93d0,
        f: ee::s03::f_001a93d0,
    };

    pub const CHAT_ROOM_SELECT_PLAYER_BY_NAME: Callee = Callee {
        addr: 0x001b_3978,
        f: ee::s03::f_001b3978,
    };

    pub const LOAD_AGREEMENT_TERMS: Callee = Callee {
        addr: 0x001c_0540,
        f: ee::s03::f_001c0540,
    };

    pub const LOBBY_MAN_RELEASE_SNAPSHOT_LISTS: Callee = Callee {
        addr: 0x0025_a850,
        f: ee::s06::f_0025a850,
    };

    pub const LOBBY_MAN_SEND_DECLINE: Callee = Callee {
        addr: 0x0025_fc28,
        f: ee::s06::f_0025fc28,
    };

    pub const SONG_INSTANCE_PICK_RANDOM_LOOP: Callee = Callee {
        addr: 0x002b_24c0,
        f: ee::s08::f_002b24c0,
    };

    pub const REPLAY_COUNT_SAVE_SNAPSHOTS: Callee = Callee {
        addr: 0x0027_12f8,
        f: ee::s07::f_002712f8,
    };

    pub const IN_SEQ_CHANNEL_GET_LENGTH: Callee = Callee {
        addr: 0x0027_4348,
        f: ee::s07::f_00274348,
    };

    pub const ALGORITHM_LIST_PUSH_2: Callee = Callee {
        addr: 0x0015_c988,
        f: ee::s01::f_0015c988,
    };

    pub const ALGORITHM_LIST_CLEAR: Callee = Callee {
        addr: 0x0015_cb08,
        f: ee::s01::f_0015cb08,
    };

    pub const SND_BANK_IS_LOADED_WITH_NAME: Callee = Callee {
        addr: 0x0028_b830,
        f: ee::s07::f_0028b830,
    };

    pub const SND_LOAD_QUEUE_CANCEL_BANK: Callee = Callee {
        addr: 0x0028_a728,
        f: ee::s07::f_0028a728,
    };

    pub const SND_LOAD_QUEUE_PUSH: Callee = Callee {
        addr: 0x0028_a558,
        f: ee::s07::f_0028a558,
    };

    pub const CLEAR_MUSIC_TRIGGER: Callee = Callee {
        addr: 0x0028_d7d8,
        f: ee::s07::f_0028d7d8,
    };

    pub const SPRAY_CONFIGURE_FOR_RIDER: Callee = Callee {
        addr: 0x002e_26b8,
        f: ee::s09::f_002e26b8,
    };

    pub const SPRAY_UPDATE_EMITTER_ENABLES: Callee = Callee {
        addr: 0x002e_2550,
        f: ee::s09::f_002e2550,
    };

    pub const BE_GET_RIDER_PROFILE_5: Callee = Callee {
        addr: 0x0014_a0e0,
        f: ee::s01::f_0014a0e0,
    };

    pub const BENEW_PLAYER_INT_GET_SLOT_2: Callee = Callee {
        addr: 0x0014_7798,
        f: ee::s01::f_00147798,
    };

    pub const BE_SET_CHAR_PROFILE_FLAG: Callee = Callee {
        addr: 0x0015_2430,
        f: ee::s01::f_00152430,
    };

    pub const GAME_RAND_NEXT: Callee = Callee {
        addr: 0x0023_7cd8,
        f: ee::s06::f_00237cd8,
    };

    pub const WORLD_TRIGGER_IS_FLAG4: Callee = Callee {
        addr: 0x002b_67d8,
        f: ee::s08::f_002b67d8,
    };

    pub const DATA_INSTANCE_FREE_INFO: Callee = Callee {
        addr: 0x0027_5cd0,
        f: ee::s07::f_00275cd0,
    };

    pub const SCRIPT_CMD_QUEUE_AT_4: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::f_00283d70,
    };

    pub const SCRIPT_LIST_IS_MOVIE_ENTRY_3: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::f_00277dd8,
    };

    pub const SCRIPT_ENGINE_GET_PROCESS_STATE_2: Callee = Callee {
        addr: 0x0028_2798,
        f: ee::s07::f_00282798,
    };

    pub const SCRIPT_LIST_SET_PAUSED: Callee = Callee {
        addr: 0x0027_7c08,
        f: ee::s07::f_00277c08,
    };

    pub const AVALANCHE_DESTROY_ALL: Callee = Callee {
        addr: 0x002d_9b40,
        f: ee::s09::f_002d9b40,
    };

    pub const ATTACHED_EMITTER_FX_DESTROY_EMITTER: u32 = 0x002f_2218;

    pub const WORLD_CACHE_IS_HULL_FREE: Callee = Callee {
        addr: 0x003a_8638,
        f: ee::s13::f_003a8638,
    };

    pub const SYNC_PROCESS_CTOR: Callee = Callee {
        addr: 0x0035_4850,
        f: ee::s11::f_00354850,
    };

    pub const MDF_FILE_CREATE_MODEL: Callee = Callee {
        addr: 0x0031_4cb0,
        f: ee::s10::f_00314cb0,
    };

    pub const FE_MEMCARD_START_PROFILE_SEARCH: Callee = Callee {
        addr: 0x0018_77b0,
        f: ee::s02::f_001877b0,
    };

    pub const FE_MEMCARD_FINISH: Callee = Callee {
        addr: 0x0018_7318,
        f: ee::s02::f_00187318,
    };

    pub const UI_LIST_FIRST_6: Callee = Callee {
        addr: 0x0039_7840,
        f: ee::s13::f_00397840,
    };

    pub const TRANSITION_EFFECT_DRAW_RECT: Callee = Callee {
        addr: 0x002e_b938,
        f: ee::s09::f_002eb938,
    };

    pub const BE_RESET_PROFILE_CHARACTERS: Callee = Callee {
        addr: 0x0015_67b8,
        f: ee::s01::f_001567b8,
    };

    pub const BE_COPY_PROFILE: Callee = Callee {
        addr: 0x0014_e9c0,
        f: ee::s01::f_0014e9c0,
    };

    pub const WORLD_STATE_CREATE_OBJECT: Callee = Callee {
        addr: 0x0026_ded8,
        f: ee::s07::f_0026ded8,
    };

    pub const AUDIO_LEVEL_TO_VOLUME_2: Callee = Callee {
        addr: 0x0028_73d8,
        f: ee::s07::f_002873d8,
    };

    pub const AUDIO_SET_CHANNEL_VOLUME_2: Callee = Callee {
        addr: 0x0028_7700,
        f: ee::s07::f_00287700,
    };


    // `call_indirect` targets the generator left as table calls.
    pub const MEMCPY_2: u32 = 0x003e_6574;

    pub const STRCASECMP: u32 = 0x0041_aa88;

    pub const SPRINTF_6: u32 = 0x0041_7828;

    pub const FE_STRING_HASH: u32 = 0x0031_7670;

    pub const UI_SCROLL_SET_PARTS_VISIBLE: u32 = 0x0039_de68;

    pub const UI_ADD_SCREEN_BY_HASH_NAME: u32 = 0x0039_7cc0;

    pub const UI_SCREEN_PLAY_FRAME_3: u32 = 0x0039_c870;

    pub const UI_SCREEN_GET_OBJECT_BY_HASH: u32 = 0x0039_d860;

    pub const LOBBY_API_DESTROY_LIST: u32 = 0x003e_8688;

    pub const LOBBY_API_FLUSH_ROSTER: u32 = 0x003f_0be0;

    pub const BUDDY_API_DESTROY: u32 = 0x003f_07a0;

    pub const TOOLTIP_MANAGER_HIDE_POPUP: u32 = 0x001d_a510;

    pub const TOOLTIP_MANAGER_SET_STATE: u32 = 0x001d_a528;

    pub const TOOLTIP_MANAGER_SET_TEXT: u32 = 0x001d_a530;

    pub const TUBE_MENU_ITEM_DTOR: u32 = 0x002c_a280;

    pub const TUBE_MENU_BASE_DTOR: u32 = 0x002c_aa80;

    pub const OVERLAY_RESET: u32 = 0x002e_a820;

    pub const BE_SET_PHASE: u32 = 0x0014_de68;

    pub const GAME_MODULE_SET_STATE: u32 = 0x0020_a8f8;

    pub const WORLD_SOUND_GET_TRIGGER_IDS: Callee = Callee {
        addr: 0x002b_8190,
        f: ee::s08::cWorldSound_GetTriggerIds,
    };

    pub const AUDIO_GET_GAME_MODULE: u32 = 0x0028_b1c8;

    pub const MC_GET_WORLD: u32 = 0x0022_7f80;

    pub const MC_MANAGER_GET_2: u32 = 0x0023_fb58;

    pub const OVERLAY_UPDATE_CALLEE: Callee = Callee {
        addr: 0x0020_e900,
        f: ee::s05::f_0020e900,
    };

    pub const SND_BANK_LOAD: u32 = 0x0028_bd10;

    pub const AVALANCHE_RESET_ONE: u32 = 0x0037_15b0;

    pub const REPLAY_GET_WORLD: u32 = 0x0012_baf0;

    pub const GAME_STATE_IS_LOADING: u32 = 0x0027_8da0;

    pub const WORLD_STATE_FINALIZE: u32 = 0x0035_4c98;

    pub const UI_SCREEN_LOOKUP_VALUE: u32 = 0x0039_c778;

    pub const UI_SCREEN_LAUNCH_THREAD: u32 = 0x0039_f190;

    pub const PATH_PLAT_RESUME: u32 = 0x003b_58a0;

    pub const PATH_PLAT_SUSPEND: u32 = 0x003b_58d8;

    pub const PATH_PLAT_SET_RUNNING: u32 = 0x003b_7fb8;

    pub const PATH_PLAT_STOP: u32 = 0x003b_8790;

    pub const OVERLAY_UPDATE: u32 = 0x0020_e900;

    pub const UI_SCREEN_SET_OBJECT_VALUE: u32 = 0x0039_83f0;


    pub const FE_ON_ABORT_POPUP_EVENT: Callee = Callee { addr: 0x001a_ca88, f: ee::s03::f_001aca88 };

    pub const FE_ON_EMAIL_POPUP_EVENT: Callee = Callee { addr: 0x001a_c9c0, f: ee::s03::f_001ac9c0 };

    pub const FE_ON_LOGIN_POPUP_EVENT: Callee = Callee { addr: 0x001a_cb48, f: ee::s03::f_001acb48 };

    pub const FE_ON_SAVE_ACCOUNT_ANSWER: Callee = Callee { addr: 0x001a_ccd0, f: ee::s03::f_001accd0 };

    pub const NET_FE_STATE_ON_CHILD_EVENT_12: Callee = Callee { addr: 0x001a_97b8, f: ee::s03::f_001a97b8 };

    pub const CHALLENGE_POPUP_SET_SETTINGS_LOCKED: Callee =
        Callee { addr: 0x001a_ec60, f: ee::s03::f_001aec60 };

    pub const NET_FE_STATE_CTOR_11: Callee = Callee { addr: 0x001a_8500, f: ee::s03::f_001a8500 };

    pub const VOIP_MAN_DEACTIVATE: Callee = Callee { addr: 0x0026_6e88, f: ee::s07::f_00266e88 };

    pub const VOIP_MAN_ACTIVATE: Callee = Callee { addr: 0x0026_6e30, f: ee::s07::f_00266e30 };

    pub const AUDIO_RESET_RACE_RESULT_STATE: Callee =
        Callee { addr: 0x002a_4590, f: ee::s08::f_002a4590 };

    pub const AUDIO_IS_TRACK_GROUP0_EVENT: Callee =
        Callee { addr: 0x002a_4078, f: ee::s08::f_002a4078 };

    pub const AUDIO_STORE_RACE_RESULT: Callee = Callee { addr: 0x002a_4660, f: ee::s08::f_002a4660 };

    pub const INI_SKIP_WHITESPACE: Callee = Callee { addr: 0x002a_78f8, f: ee::s08::f_002a78f8 };

    pub const INI_GET_CHAR: Callee = Callee { addr: 0x002a_78e0, f: ee::s08::f_002a78e0 };

    pub const BE_GET_RIDER_CHARACTER_2: Callee = Callee { addr: 0x0014_a080, f: ee::s01::f_0014a080 };

    pub const BE_COUNT_COLLECTED_ON_PEAK: Callee = Callee { addr: 0x0015_3708, f: ee::s01::f_00153708 };

    pub const BE_COUNT_COMPLETED_ON_PEAK: Callee = Callee { addr: 0x0015_44d0, f: ee::s01::f_001544d0 };

    pub const POPUP_CONFIRM_GET_SELECTION: Callee = Callee { addr: 0x001c_a150, f: ee::s03::f_001ca150 };

    pub const SETUP_BUDDY_ACTION_POPUP: Callee = Callee { addr: 0x001a_b478, f: ee::s03::f_001ab478 };

    pub const NET_FE_STATE_GET_POPUP_4: Callee = Callee { addr: 0x001a_9c50, f: ee::s03::f_001a9c50 };

    pub const NET_FE_STATE_SHOW_POPUP_3: Callee = Callee { addr: 0x001a_9bc0, f: ee::s03::f_001a9bc0 };

    pub const POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID_2: Callee =
        Callee { addr: 0x001c_57c0, f: ee::s03::f_001c57c0 };

    pub const MEM_CARD_STATE_BASE_SHUTDOWN: Callee =
        Callee { addr: 0x001d_58b8, f: ee::s04::f_001d58b8 };

    pub const DEAD_NODE_CTOR: Callee = Callee { addr: 0x0035_06d8, f: ee::s11::f_003506d8 };

    pub const CROWD_MAN3D_RESET: Callee = Callee { addr: 0x0034_3bc0, f: ee::s11::f_00343bc0 };

    pub const CROWD_MAN3D_GROUP_LOAD: Callee = Callee { addr: 0x0034_4ff8, f: ee::s11::f_00344ff8 };

    pub const SPHERE_TREE_COLLIDE_TREE_NODE: Callee =
        Callee { addr: 0x0032_cbf8, f: ee::s10::f_0032cbf8 };

    pub const FEMODULE_RIDER_PREVIEW_UNLOAD: Callee =
        Callee { addr: 0x0019_eba0, f: ee::s03::f_0019eba0 };

    pub const RIDER_UPDATE_BOARD_LEAN: Callee = Callee { addr: 0x0011_fa10, f: ee::s00::f_0011fa10 };

    pub const RIDER_BUILD_WORLD_MATRIX: Callee = Callee { addr: 0x0011_e098, f: ee::s00::f_0011e098 };

    pub const GAME_STATE_LOAD_STATE_2: Callee = Callee { addr: 0x0023_3b28, f: ee::s06::f_00233b28 };

    pub const MC_MANAGER_ON_CARD_SCANNED: Callee = Callee { addr: 0x0024_1fd0, f: ee::s06::f_00241fd0 };

    pub const GAME_LOAD_UPDATE_PLAYER_RANKING: Callee =
        Callee { addr: 0x0024_6cf8, f: ee::s06::f_00246cf8 };

    pub const LIGHT_MAN_EMPTY_HOOK_D: Callee = Callee { addr: 0x0038_dc68, f: ee::s12::f_0038dc68 };

    pub const BE_GET_CHAR_PROFILE_EVENT_RECORD: Callee =
        Callee { addr: 0x0015_2398, f: ee::s01::f_00152398 };

    pub const BE_NEW_PLAYER_INT_GET_SLOT_3: Callee = Callee { addr: 0x0014_7798, f: ee::s01::f_00147798 };

    pub const MENU_SYS_SET_ACTIVE: Callee = Callee { addr: 0x002c_c0b8, f: ee::s09::f_002cc0b8 };

    pub const MPEG2_GET_BITS_2: Callee = Callee { addr: 0x003b_1300, f: ee::s13::f_003b1300 };

    pub const MPEG2_MARKER_BIT_2: Callee = Callee { addr: 0x003b_2360, f: ee::s13::f_003b2360 };

    pub const SND_LOAD_QUEUE_FLUSH: Callee = Callee { addr: 0x0028_a230, f: ee::s07::f_0028a230 };


    pub const STR_HASH_5: Callee = Callee {
        addr: 0x0031_7670,
        f: ee::s10::f_00317670,
    };

    pub const UIENGINE_CREATE_SCREEN: Callee = Callee {
        addr: 0x0039_7cc0,
        f: ee::s13::f_00397cc0,
    };

    pub const UISCREEN_START_THREAD: Callee = Callee {
        addr: 0x0039_c870,
        f: ee::s13::f_0039c870,
    };

    pub const UISCREEN_FIND_OBJECT: Callee = Callee {
        addr: 0x0039_d860,
        f: ee::s13::f_0039d860,
    };

    pub const BE_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };

    pub const BE_GET_INTERFACE_2: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::f_0014dd58,
    };

    pub const BE_GET_CHAR_DISPLAY_NAME: Callee = Callee {
        addr: 0x0014_eec8,
        f: ee::s01::f_0014eec8,
    };

    pub const SPRINTF_7: Callee = Callee {
        addr: 0x0041_7828,
        f: ee::s15::f_00417828,
    };

    pub const UITEXTSCROLL_CLEAR_LINES: Callee = Callee {
        addr: 0x003a_18c0,
        f: ee::s13::f_003a18c0,
    };

    pub const UITEXTSCROLL_SET_TEXT_ID: Callee = Callee {
        addr: 0x003a_1f18,
        f: ee::s13::f_003a1f18,
    };

    pub const UIENGINE_CLOSE_ALL_POPUPS: Callee = Callee {
        addr: 0x0039_f698,
        f: ee::s13::f_0039f698,
    };

    pub const UIENGINE_GET_ACTIVE_STATE: Callee = Callee {
        addr: 0x0039_fa88,
        f: ee::s13::f_0039fa88,
    };

    pub const NETFE_STATE_GET_POPUP: Callee = Callee {
        addr: 0x001a_9c50,
        f: ee::s03::f_001a9c50,
    };

    pub const NETFE_STATE_SHOW_POPUP: Callee = Callee {
        addr: 0x001a_9bc0,
        f: ee::s03::f_001a9bc0,
    };

    pub const POPUP_SET_OPTION_STRING: Callee = Callee {
        addr: 0x001c_5780,
        f: ee::s03::f_001c5780,
    };

    pub const POPUP_SET_OPTION_TEXT_ID: Callee = Callee {
        addr: 0x001c_57c0,
        f: ee::s03::f_001c57c0,
    };

    pub const NETFE_STATE_ON_CHILD_EVENT_DEFAULT: Callee = Callee {
        addr: 0x001a_93d0,
        f: ee::s03::f_001a93d0,
    };

    pub const NETFE_STATE_OPEN_KEYBOARD_TEXT_ID: Callee = Callee {
        addr: 0x001a_9150,
        f: ee::s03::f_001a9150,
    };

    pub const NETFE_STATE_GOTO_NEXT_STATE: Callee = Callee {
        addr: 0x001a_8a40,
        f: ee::s03::f_001a8a40,
    };

    pub const CFE_STATE_LOGIN_CTOR: Callee = Callee {
        addr: 0x001b_ee30,
        f: ee::s03::f_001bee30,
    };

    pub const CFE_STATE_NET_CONFIG_CTOR: Callee = Callee {
        addr: 0x001b_3f78,
        f: ee::s03::f_001b3f78,
    };

    pub const CBXSTRING_ASSIGN: Callee = Callee {
        addr: 0x0031_8448,
        f: ee::s10::f_00318448,
    };

    pub const MENU_SYS_SET_ACTIVE_2: Callee = Callee {
        addr: 0x002c_c0b8,
        f: ee::s09::f_002cc0b8,
    };

    pub const MENU_SYS_ITEM_DRAW_ROW_2: Callee = Callee {
        addr: 0x002c_a988,
        f: ee::s09::f_002ca988,
    };

    pub const SCRIPT_ENGINE_CHOOSE_SCRIPT: Callee = Callee {
        addr: 0x0027_b0c0,
        f: ee::s07::f_0027b0c0,
    };

    pub const SCRIPT_ENGINE_POP_FREE_NODE: Callee = Callee {
        addr: 0x0027_bd78,
        f: ee::s07::f_0027bd78,
    };

    pub const SCRIPT_ENGINE_START_SCRIPT: Callee = Callee {
        addr: 0x0028_21a0,
        f: ee::s07::f_002821a0,
    };

    pub const SCRIPT_ENGINE_PUSH_FREE_NODE_2: Callee = Callee {
        addr: 0x0027_bd90,
        f: ee::s07::f_0027bd90,
    };

    pub const SCRIPT_ENGINE_ADD_SCRIPT_OBJ: Callee = Callee {
        addr: 0x0027_cea8,
        f: ee::s07::f_0027cea8,
    };

    pub const SCRIPT_VIEW_OBJ_ADD_CAM: Callee = Callee {
        addr: 0x0028_1190,
        f: ee::s07::f_00281190,
    };

    pub const SCRIPT_VIEW_OBJ_SET_TRANSITION: Callee = Callee {
        addr: 0x0028_12e0,
        f: ee::s07::f_002812e0,
    };

    pub const SCRIPT_ENGINE_SET_BINDING_FILTER: Callee = Callee {
        addr: 0x0027_d540,
        f: ee::s07::f_0027d540,
    };

    pub const INSEQ_STOP: Callee = Callee {
        addr: 0x0027_2b58,
        f: ee::s07::f_00272b58,
    };

    pub const INSEQ_INSTANCE_RELEASE: Callee = Callee {
        addr: 0x0027_46f8,
        f: ee::s07::f_002746f8,
    };

    pub const INSEQ_EVENT_TRACK_FIRE_KEY: Callee = Callee {
        addr: 0x0027_3418,
        f: ee::s07::f_00273418,
    };

    pub const INSEQ_EVENT_TRACK_NEXT_KEY: Callee = Callee {
        addr: 0x0027_3468,
        f: ee::s07::f_00273468,
    };

    pub const USTR_VSPRINTF: Callee = Callee {
        addr: 0x003d_c450,
        f: ee::s14::f_003dc450,
    };

    pub const USTR_LEN_2: Callee = Callee {
        addr: 0x003d_bca0,
        f: ee::s14::f_003dbca0,
    };

    pub const USTR_CPY_2: Callee = Callee {
        addr: 0x003d_be20,
        f: ee::s14::f_003dbe20,
    };

    pub const EA_MEM_ALLOC_NAMED_5: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::f_00317d70,
    };

    pub const EA_MEM_FREE_5: Callee = Callee {
        addr: 0x0031_7e98,
        f: ee::s10::f_00317e98,
    };

    pub const EAFILE_GET_PATH_SIZE_CHECKED: Callee = Callee {
        addr: 0x0031_7f60,
        f: ee::s10::f_00317f60,
    };

    pub const EAFILE_LOAD_FILE_INTO: Callee = Callee {
        addr: 0x003e_1bb8,
        f: ee::s14::f_003e1bb8,
    };

    pub const BOLT_FILE_ACQUIRE: Callee = Callee {
        addr: 0x0019_da10,
        f: ee::s03::f_0019da10,
    };

    pub const REWARDS_FILE_PARSE: Callee = Callee {
        addr: 0x0015_a818,
        f: ee::s01::f_0015a818,
    };

    pub const BE_BACKUP_CHARACTER: Callee = Callee {
        addr: 0x0015_6950,
        f: ee::s01::f_00156950,
    };

    pub const RIDER_METRIX_ON_RIDER_RESET: Callee = Callee {
        addr: 0x0011_9368,
        f: ee::s00::f_00119368,
    };

    pub const RIDER_ADD_WIPEOUT_RECOVERY_BOOST: Callee = Callee {
        addr: 0x0010_f280,
        f: ee::s00::f_0010f280,
    };

    pub const RIDER_CHECK_RESET_PATH: Callee = Callee {
        addr: 0x0011_6120,
        f: ee::s00::f_00116120,
    };

    pub const ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED: Callee = Callee {
        addr: 0x0031_2ae8,
        f: ee::s10::f_00312ae8,
    };

    pub const AV_PLAYER_IS_FRAME_DUE: Callee = Callee {
        addr: 0x003a_e780,
        f: ee::s13::f_003ae780,
    };

    pub const AV_PLAYER_DECODE_FRAME: Callee = Callee {
        addr: 0x003a_e6c8,
        f: ee::s13::f_003ae6c8,
    };

    pub const AV_PLAYER_IS_AUDIO_DONE: Callee = Callee {
        addr: 0x003a_e860,
        f: ee::s13::f_003ae860,
    };

    pub const UI_LIST_DELETE_ALL_3: Callee = Callee {
        addr: 0x0039_7948,
        f: ee::s13::f_00397948,
    };

    pub const UI_LIST_NODE_DTOR_BODY_4: Callee = Callee {
        addr: 0x003a_4868,
        f: ee::s13::f_003a4868,
    };

    pub const UI_ENGINE_DTOR: Callee = Callee {
        addr: 0x0039_7b70,
        f: ee::s13::f_00397b70,
    };

    pub const UI_OBJECT_CTOR_3: Callee = Callee {
        addr: 0x0039_fb30,
        f: ee::s13::f_0039fb30,
    };

    pub const UI_STATE_MGR_FIND_STATE_2: Callee = Callee {
        addr: 0x0039_f9d8,
        f: ee::s13::f_0039f9d8,
    };

    pub const SSX_AUDIO_SYSTEM_SET_MODE: Callee = Callee {
        addr: 0x0028_5be0,
        f: ee::s07::f_00285be0,
    };

    pub const SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN: Callee = Callee {
        addr: 0x0028_7930,
        f: ee::s07::f_00287930,
    };

    pub const SND_BANK_MAN_UNLOAD_BANK: Callee = Callee {
        addr: 0x0028_bce8,
        f: ee::s07::f_0028bce8,
    };

    pub const BANK_MONITOR_KILL_ALL_TIMERS: Callee = Callee {
        addr: 0x002a_dda0,
        f: ee::s08::f_002adda0,
    };

    pub const MUSIC_PLAYER_STOP_SONG_2: Callee = Callee {
        addr: 0x002b_3ac0,
        f: ee::s08::f_002b3ac0,
    };

    pub const MUSIC_PLAYER_FADE_OUT: Callee = Callee {
        addr: 0x002b_3d48,
        f: ee::s08::f_002b3d48,
    };

    pub const MUSIC_PLAYER_PLAY_SONG: Callee = Callee {
        addr: 0x002b_35a0,
        f: ee::s08::f_002b35a0,
    };

    pub const SPEECH_INSTANCE_CTOR: Callee = Callee {
        addr: 0x002a_fa68,
        f: ee::s08::f_002afa68,
    };

    pub const APP_MODULE_TRY_EXIT_2: Callee = Callee {
        addr: 0x0023_1cf0,
        f: ee::s06::f_00231cf0,
    };

    pub const SCRIPT_ENGINE_CLEAR_SKIP_DELAY_ON_EXIT: Callee = Callee {
        addr: 0x0027_8718,
        f: ee::s07::f_00278718,
    };

    pub const LOAD_INDICATOR_CLEAR: Callee = Callee {
        addr: 0x001f_3170,
        f: ee::s04::f_001f3170,
    };

    pub const GRAPHICS_MAN_FREE_ALL_TEXTURES: Callee = Callee {
        addr: 0x0036_a020,
        f: ee::s12::f_0036a020,
    };

    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE_4: Callee = Callee {
        addr: 0x0028_b180,
        f: ee::s07::f_0028b180,
    };

    pub const SSX_AUDIO_SYSTEM_STOP_LOADING_SCREEN_LOOP: Callee = Callee {
        addr: 0x0028_fa98,
        f: ee::s07::f_0028fa98,
    };

    pub const LOBBY_API_GET_RECORD: Callee = Callee {
        addr: 0x003e_8a70,
        f: ee::s14::f_003e8a70,
    };

    pub const NET_GAME_UTIL_CREATE: Callee = Callee {
        addr: 0x003e_6e08,
        f: ee::s14::f_003e6e08,
    };

    pub const NET_GAME_UTIL_ADVERT: Callee = Callee {
        addr: 0x003e_7120,
        f: ee::s14::f_003e7120,
    };

    pub const C_FE_POPUP_CONFIRM_EMPTY_INIT_HOOK: Callee = Callee {
        addr: 0x001a_34a0,
        f: ee::s03::f_001a34a0,
    };

    pub const DEBUG_MENU_DISPATCH_ITEM_OP: Callee = Callee {
        addr: 0x0017_a158,
        f: ee::s02::f_0017a158,
    };

    pub const LOBBY_LIST_GET_AT_2: Callee = Callee {
        addr: 0x003e_e560,
        f: ee::s14::f_003ee560,
    };

    pub const ATOI: Callee = Callee {
        addr: 0x0041_87c0,
        f: ee::s15::f_004187c0,
    };

    pub const TAG_FIELD_GET_BINARY: Callee = Callee {
        addr: 0x003e_d678,
        f: ee::s14::f_003ed678,
    };


    /// The engine allocator (`cMemMan_alloc`), reached as a table call.
    pub const MALLOC: u32 = 0x0031_7d70;

    /// `cMemMan_free`.
    pub const FREE: u32 = 0x0031_7e50;

    /// `cMemMan_free2` (the second form used by the FE states).
    pub const FREE2: u32 = 0x0031_7e98;


    pub const VEC3_MAX_COMPONENT: Callee = Callee {
        addr: 0x0017_31c0,
        f: ee::s02::Vec3_MaxComponent,
    };

    pub const SONG_INSTANCE_CTOR: Callee = Callee {
        addr: 0x002b_1c50,
        f: ee::s08::SongInstance_ctor,
    };

    pub const NET_FE_STATE_CTOR_12: Callee = Callee {
        addr: 0x001a_8500,
        f: ee::s03::f_001a8500,
    };

    pub const NET_FE_STATE_DTOR_4: Callee = Callee {
        addr: 0x001a_85d0,
        f: ee::s03::NetFEState_dtor,
    };

    pub const NET_FE_STATE_INIT_3: Callee = Callee {
        addr: 0x001a_8768,
        f: ee::s03::NetFEState_Init,
    };

    pub const NET_FE_STATE_GET_SELECTED_USER: Callee = Callee {
        addr: 0x001a_98b0,
        f: ee::s03::NetFEState_GetSelectedUser,
    };

    pub const WORLD_CACHE_CTOR: Callee = Callee {
        addr: 0x003a_7dd0,
        f: ee::s13::cWorldCache_ctor,
    };

    pub const WSCRIPT_CACHE_CTOR: Callee = Callee {
        addr: 0x003a_c7c8,
        f: ee::s13::cWScriptCache_ctor,
    };

    pub const WORLD_TRIGGER_INSTANCE_CTOR: Callee = Callee {
        addr: 0x002b_4b70,
        f: ee::s08::WorldTriggerInstance_ctor,
    };

    pub const WORLD_TRIGGER_MANAGER_LOAD: Callee = Callee {
        addr: 0x002b_5a18,
        f: ee::s08::WorldTriggerManager_LoadTriggerInfo,
    };

    pub const PROCESS_LIST_FIRST_2: Callee = Callee {
        addr: 0x0035_4ea8,
        f: ee::s11::ProcessList_First,
    };

    pub const PROCESS_LIST_NEXT_2: Callee = Callee {
        addr: 0x0035_4ef8,
        f: ee::s11::ProcessList_Next,
    };

    pub const CAMERA_TRIGGER_MAN_FREE_TRIGGERS: Callee = Callee {
        addr: 0x0016_bf40,
        f: ee::s02::CameraTriggerMan_FreeTriggers,
    };

    pub const CAMERA_TRIGGER_MAN_PARSE_2: Callee = Callee {
        addr: 0x0016_c228,
        f: ee::s02::CameraTriggerMan_Parse,
    };

    pub const CREATE_STEP2_SET_MONTH: Callee = Callee {
        addr: 0x001b_b2d0,
        f: ee::s03::cFEStateCreateStep2_SetMonthSelection,
    };

    pub const CREATE_STEP2_SET_DAY: Callee = Callee {
        addr: 0x001b_b340,
        f: ee::s03::cFEStateCreateStep2_SetDaySelection,
    };

    pub const CREATE_STEP2_SET_YEAR: Callee = Callee {
        addr: 0x001b_b3b0,
        f: ee::s03::cFEStateCreateStep2_SetYearSelection,
    };

    pub const CREATE_STEP2_SET_GENDER: Callee = Callee {
        addr: 0x001b_b440,
        f: ee::s03::cFEStateCreateStep2_SetGenderSelection,
    };

    pub const CREATE_STEP2_SET_LEGAL: Callee = Callee {
        addr: 0x001b_b4b0,
        f: ee::s03::cFEStateCreateStep2_SetLegalSelection,
    };

    pub const REWARD_GALLERY_INIT_PAGES: Callee = Callee {
        addr: 0x001d_0180,
        f: ee::s04::cFEStateRewardGallery_InitPages,
    };

    pub const REWARD_GALLERY_REQUEST_SHAPE: Callee = Callee {
        addr: 0x001c_fe60,
        f: ee::s04::cFEStateRewardGallery_RequestShapeLoad,
    };

    pub const OV_STATE_OVERLAY_INIT: Callee = Callee {
        addr: 0x0020_a380,
        f: ee::s05::cOVStateOverlay_Init,
    };

    pub const OPTIONS_HUD_REFRESH_SELECTION: Callee = Callee {
        addr: 0x0018_c478,
        f: ee::s02::cFEStateOptionsHUD_RefreshSelection,
    };

    pub const MESSAGE_CENTER_SET_FLAG_2: Callee = Callee {
        addr: 0x001e_3a78,
        f: ee::s04::MessageCenter_SetFlag,
    };

    pub const SCRIPT_BINDING_BIND: Callee = Callee {
        addr: 0x0028_2fb8,
        f: ee::s07::ScriptBinding_Bind,
    };

    pub const INPUT_PREPROC_NEXT_CHAR: Callee = Callee {
        addr: 0x0032_6478,
        f: ee::s10::InputPreproc_NextChar,
    };

    pub const INPUT_PREPROC_EMPTY_STUB: Callee = Callee {
        addr: 0x0032_6a20,
        f: ee::s10::InputPreproc_EmptyStub,
    };

    pub const BXSTRING_CTOR_COPY: Callee = Callee {
        addr: 0x0031_7fe8,
        f: ee::s10::cBXString_ctor_copy,
    };

    pub const BXSTRING_RELEASE: Callee = Callee {
        addr: 0x0031_80b8,
        f: ee::s10::cBXString_Release,
    };

    pub const BXSTRING_ALLOC_BUFFER: Callee = Callee {
        addr: 0x0031_8040,
        f: ee::s10::cBXString_AllocBuffer,
    };

    pub const BXSTRING_DTOR: Callee = Callee {
        addr: 0x0031_8260,
        f: ee::s10::cBXString_dtor,
    };

    /// `SSXAudioSystem_IsDJSpeechOn` — returns non-zero when DJ speech plays.
    pub const IS_DJ_SPEECH_ON: Callee = Callee {
        addr: 0x0029_f0d8,
        f: ee::s08::f_0029f0d8,
    };

    pub const GET_LANGUAGE_MASK: Callee = Callee {
        addr: 0x0029_f198,
        f: ee::s08::f_0029f198,
    };

    pub const SET_EVENT_CONTEXT: Callee = Callee {
        addr: 0x002b_1458,
        f: ee::s08::f_002b1458,
    };

    pub const TRACK_TO_CATEGORY: Callee = Callee {
        addr: 0x002a_1e20,
        f: ee::s08::f_002a1e20,
    };


    // `call_indirect` targets the generator left as table calls.
    pub const AUDIO_GET_GAME_MODULE_2: u32 = 0x0028_b1e8;

    pub const BE_GET_SOMETHING: u32 = 0x0014_4bc0;

    pub const SPEECH_NEW: u32 = 0x003d_8008;


    pub const TAG_FIELD_COPY_TAG: Callee = Callee { addr: 0x003e_b198, f: ee::s14::f_003eb198 };

    pub const TAG_FIELD_SET_RAW_2: Callee = Callee { addr: 0x003e_b1e0, f: ee::s14::f_003eb1e0 };


    pub const C_FE_STATE_NET_MESSENGER_CLEAR_NAME_LIST: Callee = Callee {
        addr: 0x001a_5450,
        f: ee::s03::cFEStateNetMessenger_ClearNameList,
    };

    pub const NET_FE_STATE_DTOR_5: Callee = Callee {
        addr: 0x001a_85d0,
        f: ee::s03::NetFEState_dtor,
    };

    pub const NET_FE_STATE_ON_CHILD_EVENT_13: Callee = Callee {
        addr: 0x001a_97b8,
        f: ee::s03::NetFEState_OnChildEvent,
    };

    pub const NET_FE_STATE_SHOW_POPUP_5: Callee = Callee {
        addr: 0x001a_9bc0,
        f: ee::s03::NetFEState_ShowPopup,
    };

    pub const C_FE_USE_EXISTING_ACCOUNT_CHECK_EMAIL: Callee = Callee {
        addr: 0x001a_c448,
        f: ee::s03::cFEUseExistingAccount_CheckEmail,
    };

    pub const LOBBY_MAN_CONNECT: Callee = Callee {
        addr: 0x0025_b6d8,
        f: ee::s06::LobbyMan_Connect,
    };

    pub const C_BX_STRING_ASSIGN_CSTR: Callee = Callee {
        addr: 0x0031_84e8,
        f: ee::s10::cBXString_assign_cstr,
    };


    pub const C_FE_STATE_NET_CONFIG_SET_LIST_VISIBLE: Callee = Callee {
        addr: 0x001b_5068,
        f: ee::s03::cFEStateNetConfig_SetListVisible,
    };

    pub const C_FE_STATE_NET_CONFIG_SHOW_CARD_ERROR_POPUP: Callee = Callee {
        addr: 0x001b_4888,
        f: ee::s03::cFEStateNetConfig_ShowCardErrorPopup,
    };

    pub const C_FE_STATE_NET_CONFIG_LOAD_CONFIG_LIST: Callee = Callee {
        addr: 0x001b_4cd8,
        f: ee::s03::cFEStateNetConfig_LoadConfigList,
    };


    pub const C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_HINT_VISIBLE: Callee = Callee {
        addr: 0x001c_13e0,
        f: ee::s03::cFEStateNetQuickPlayPopup_SetOptionsHintVisible,
    };

    pub const C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_ENABLED: Callee = Callee {
        addr: 0x001c_17f8,
        f: ee::s03::cFEStateNetQuickPlayPopup_SetOptionsEnabled,
    };

    pub const UI_TEXT_GET_OWNED_STRING: Callee = Callee {
        addr: 0x003a_0c00,
        f: ee::s13::UIText_GetOwnedString,
    };

    pub const UI_TEXT_GET_ID_STRING: Callee = Callee {
        addr: 0x003a_0c28,
        f: ee::s13::UIText_GetIdString,
    };

    pub const UI_TEXT_SET_TEXT_ID_2: Callee = Callee {
        addr: 0x003a_1000,
        f: ee::s13::UIText_SetTextId,
    };


    pub const BE_LIBRARY_GET_INSTANCE_4: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::BELibrary_GetInstance,
    };

    pub const BE_LIBRARY_GET_INTERFACE_4: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::BELibrary_GetInterface,
    };

    pub const BE_NEW_PLAYER_INT_GET_HUMAN_RIDER: Callee = Callee {
        addr: 0x0014_6e98,
        f: ee::s01::BENewPlayerInt_GetHumanRider,
    };

    pub const UI_STATE_MGR_CLOSE_ALL: Callee = Callee {
        addr: 0x0039_f840,
        f: ee::s13::UIStateMgr_CloseAll,
    };

    pub const GAME_MODULE_ADVANCE_STATE: Callee = Callee {
        addr: 0x0023_1320,
        f: ee::s06::GameModule_AdvanceState,
    };


    pub const SCRIPT_ENGINE_TICK_PROCESS: Callee = Callee {
        addr: 0x0028_2450,
        f: ee::s07::ScriptEngine_TickProcess,
    };

    pub const SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_3: Callee = Callee {
        addr: 0x0028_2bf0,
        f: ee::s07::ScriptEngine_GetProcessInSeq,
    };

    pub const SCRIPT_VIEW_OBJ_UPDATE: Callee = Callee {
        addr: 0x0028_1100,
        f: ee::s07::ScriptViewObj_Update,
    };

    pub const SCRIPT_ENGINE_REMOVE_BINDING: Callee = Callee {
        addr: 0x0027_d390,
        f: ee::s07::ScriptEngine_RemoveBinding,
    };


    pub const BANK_MONITOR_KILL_TIMERS_BY_CALLBACK: Callee = Callee {
        addr: 0x002a_dde0,
        f: ee::s08::BankMonitor_KillTimersByCallback,
    };

    pub const SND_LOAD_QUEUE_CANCEL_BANK_2: Callee = Callee {
        addr: 0x0028_a728,
        f: ee::s07::SndLoadQueue_CancelBank,
    };

    pub const SND_BANK_MAN_UNLOAD_BANK_2: Callee = Callee {
        addr: 0x0028_bce8,
        f: ee::s07::SndBankMan_UnloadBank,
    };

    pub const SONG_INSTANCE_DTOR: Callee = Callee {
        addr: 0x002b_1f78,
        f: ee::s08::SongInstance_dtor,
    };

    pub const KEYBOARD_POPUP_FIND_ENABLED_IN_COLUMN_FROM_TOP: Callee = Callee {
        addr: 0x001c_c090,
        f: ee::s04::cKeyboardPopup_FindEnabledInColumnFromTop,
    };


    pub const UI_STATE_CTOR_2_4: Callee = Callee {
        addr: 0x0039_e318,
        f: ee::s13::UIState_ctor_2,
    };

    pub const FE_MODULE_GET_PLAYER_PAD_MASK_2: Callee = Callee {
        addr: 0x001a_1cd0,
        f: ee::s03::FEModule_GetPlayerPadMask,
    };

    pub const BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT: Callee = Callee {
        addr: 0x0014_7488,
        f: ee::s01::BENewPlayerInt_GetCharacterBySlot,
    };

    pub const SSX_AUDIO_SYSTEM_GET_FADE_MASK: Callee = Callee {
        addr: 0x0028_4ba0,
        f: ee::s07::SSXAudioSystem_GetFadeMask,
    };

    pub const SSX_AUDIO_SYSTEM_SET_FADE_GAIN: Callee = Callee {
        addr: 0x0028_7a10,
        f: ee::s07::SSXAudioSystem_SetFadeGain,
    };

    pub const LUNO_VM_REGISTER_AS_ID: Callee = Callee {
        addr: 0x0022_6620,
        f: ee::s05::LunoVMRegister_AsId,
    };

    pub const LUNO_TABLE_FIND: Callee = Callee {
        addr: 0x0022_5248,
        f: ee::s05::cLunoTable_Find,
    };

    pub const LUNO_VM_PASS_THROUGH: Callee = Callee {
        addr: 0x0022_24a8,
        f: ee::s05::LunoVM_PassThrough,
    };

    pub const LUNO_VM_CALL: Callee = Callee {
        addr: 0x0022_24b8,
        f: ee::s05::LunoVM_Call,
    };

    pub const LUNO_VM_REGISTER_DTOR_2: Callee = Callee {
        addr: 0x0022_5b90,
        f: ee::s05::LunoVMRegister_dtor,
    };

    pub const MPEG2_EXTENSION_AND_USER_DATA: Callee = Callee {
        addr: 0x003b_1988,
        f: ee::s13::MPEG2_extension_and_user_data,
    };

    pub const RIDER_GET_START_POSE_ANGLE: Callee = Callee {
        addr: 0x0011_5b08,
        f: ee::s00::Rider_GetStartPoseAngle,
    };

    pub const RIDER_PLACE: Callee = Callee {
        addr: 0x0011_d660,
        f: ee::s00::Rider_Place,
    };

    pub const PATH_DATA_FIND_MARKER: Callee = Callee {
        addr: 0x0026_b5e0,
        f: ee::s07::PathData_FindMarker,
    };

    pub const MEM_ALLOC_NAMED_128: Callee = Callee {
        addr: 0x0025_2fa0,
        f: ee::s06::MEM_AllocNamed128,
    };

    pub const UI_STATE_MGR_CLOSE_ALL_POPUPS_2: Callee = Callee {
        addr: 0x0039_f698,
        f: ee::s13::UIStateMgr_CloseAllPopups,
    };

    pub const UI_STATE_MGR_SWITCH_STATE: Callee = Callee {
        addr: 0x0039_f400,
        f: ee::s13::UIStateMgr_SwitchState,
    };

    pub const UI_SCREEN_FIND_OBJECT_5: Callee = Callee {
        addr: 0x0039_d860,
        f: ee::s13::UIScreen_FindObject,
    };

    pub const UI_MENU_SET_SELECTION_3: Callee = Callee {
        addr: 0x0039_b960,
        f: ee::s13::UIMenu_SetSelection,
    };

    pub const GAME_MODULE_QUIT_TO_LOAD_SCREEN: Callee = Callee {
        addr: 0x0023_0488,
        f: ee::s06::GameModule_QuitToLoadScreen,
    };


    pub const IN_SEQ_START_SEQUENCE: Callee = Callee {
        addr: 0x0027_2938,
        f: ee::s07::InSeq_StartSequence,
    };

    pub const IN_SEQ_GET_INSTANCE: Callee = Callee {
        addr: 0x0027_2cc0,
        f: ee::s07::InSeq_GetInstance,
    };

    pub const IN_SEQ_INSTANCE_SET_USER_DATA_2: Callee = Callee {
        addr: 0x0027_4c10,
        f: ee::s07::InSeq_InstanceSetUserData,
    };

    pub const IN_SEQ_INSTANCE_SET_SPEED: Callee = Callee {
        addr: 0x0027_4808,
        f: ee::s07::InSeq_InstanceSetSpeed,
    };

    pub const SCRIPT_ENGINE_GET_TRANSITION_TARGET: Callee = Callee {
        addr: 0x0027_a4a0,
        f: ee::s07::ScriptEngine_GetTransitionTarget,
    };

    pub const C_TRANSITION_MAN_START: Callee = Callee {
        addr: 0x002e_4370,
        f: ee::s09::cTransitionMan_Start,
    };

    pub const C_TRANSITION_MAN_START_CAPTURED: Callee = Callee {
        addr: 0x002e_4540,
        f: ee::s09::cTransitionMan_StartCaptured,
    };

    pub const C_CUT_SCENE_TRANSITION_MAN_SAVE_VIEWPORT: Callee = Callee {
        addr: 0x002e_aa28,
        f: ee::s09::cCutSceneTransitionMan_SaveViewport,
    };


    pub const BE_NEW_RACE_INT_GET_TRACK: Callee = Callee {
        addr: 0x0014_4be0,
        f: ee::s01::BENewRaceInt_GetTrack,
    };

    pub const BE_NEW_RACE_INT_GET_SPECIAL_EVENT_VALUE: Callee = Callee {
        addr: 0x0014_5668,
        f: ee::s01::BENewRaceInt_GetSpecialEventValue,
    };

    pub const RIDER_METRIX_SHOW_HUD_MSG_2: Callee = Callee {
        addr: 0x0011_95a8,
        f: ee::s00::RiderMetrix_ShowHUDMsg2A,
    };


    pub const FE_MODULE_RIDER_PREVIEW_SET_VISIBLE: Callee = Callee {
        addr: 0x0019_e538,
        f: ee::s03::FEModule_RiderPreviewSetVisible,
    };

    pub const FE_MODULE_GET_RIDER_PREVIEW: Callee = Callee {
        addr: 0x001a_0548,
        f: ee::s03::FEModule_GetRiderPreview,
    };

    pub const FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES: Callee = Callee {
        addr: 0x0019_ec68,
        f: ee::s03::FEModule_RiderPreviewReleaseBoltFiles,
    };

    pub const FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR: Callee = Callee {
        addr: 0x0030_d540,
        f: ee::s10::RiderModel_dtor,
    };


    pub const UI_OBJECT_LOAD: Callee = Callee {
        addr: 0x0039_fe00,
        f: ee::s13::UIObject_Load,
    };

    pub const INPUT_BINDING_EVAL_FLOAT: Callee = Callee {
        addr: 0x0032_0bf0,
        f: ee::s10::InputBinding_EvalFloat,
    };

    pub const MC_MANAGER_FORMAT_SLOT_NAME: Callee = Callee {
        addr: 0x0024_1dc8,
        f: ee::s06::MCManager_FormatSlotName,
    };

    pub const U_STR_SPRINTF: Callee = Callee {
        addr: 0x002c_26d0,
        f: ee::s08::UStr_Sprintf,
    };

    pub const U_STR_SPRINTF_ASCII_FMT: Callee = Callee {
        addr: 0x002c_2718,
        f: ee::s08::UStr_SprintfAsciiFmt,
    };

    pub const U_STR_TO_ASCII: Callee = Callee {
        addr: 0x002c_2580,
        f: ee::s08::UStr_ToAscii,
    };

    pub const U_STR_LEN: Callee = Callee {
        addr: 0x003d_bca0,
        f: ee::s14::UStr_Len,
    };


    pub const FE_ASYNC_MANAGER_SET_STATE: Callee = Callee {
        addr: 0x001a_39d8,
        f: ee::s03::FEAsyncManager_SetState,
    };

    pub const BE_MISSION_INT_GET_SESSION_NAME: Callee = Callee {
        addr: 0x0015_4760,
        f: ee::s01::BEMissionInt_GetSessionName,
    };

    pub const SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_TOP: Callee = Callee {
        addr: 0x0028_2bf0,
        f: ee::s07::ScriptEngine_GetProcessInSeq,
    };


    pub const C_SETUP_MAN_ALLOC_PACKET: Callee = Callee {
        addr: 0x0026_4960,
        f: ee::s07::cSetupMan_AllocPacket,
    };

    pub const C_SETUP_MAN_FREE_PACKET: Callee = Callee {
        addr: 0x0026_4990,
        f: ee::s07::cSetupMan_FreePacket,
    };

    pub const C_COMM_SYSTEM_CHANNEL_RECV: Callee = Callee {
        addr: 0x0025_5dc0,
        f: ee::s06::cCommSystem_ChannelRecv,
    };

    pub const C_COMM_SYSTEM_CHANNEL_SEND: Callee = Callee {
        addr: 0x0025_5e00,
        f: ee::s06::cCommSystem_ChannelSend,
    };

    pub const C_BX_STRING_CTOR_CSTR: Callee = Callee {
        addr: 0x0031_82d8,
        f: ee::s10::cBXString_ctor_cstr,
    };

    pub const OPERATOR_NEW_NAMED_4: Callee = Callee {
        addr: 0x0031_7e30,
        f: ee::s10::operator_new_named,
    };


    pub const NET_FE_STATE_GOTO_NEXT_STATE: Callee = Callee {
        addr: 0x001a_8a40,
        f: ee::s03::NetFEState_GotoNextState,
    };

    pub const C_FE_STATE_NET_SSX_WORLD_CTOR: Callee = Callee {
        addr: 0x001c_1c50,
        f: ee::s03::cFEStateNetSSXWorld_ctor,
    };

    pub const C_FE_STATE_OPTIONS_DEVICE_SELECT_CTOR: Callee = Callee {
        addr: 0x0018_d7f0,
        f: ee::s02::cFEStateOptionsDeviceSelect_ctor,
    };


    pub const SSX_AUDIO_SYSTEM_IS_SPEECH_VOLUME_ON: Callee = Callee {
        addr: 0x0029_f0a0,
        f: ee::s08::SSXAudioSystem_IsSpeechVolumeOn,
    };

    pub const SPEECH_SYSTEM_IS_INSTANCE_READY: Callee = Callee {
        addr: 0x002b_0e28,
        f: ee::s08::SpeechSystem_IsInstanceReady,
    };

    pub const SSX_AUDIO_SYSTEM_CHARACTER_TO_SPEECH_MASK: Callee = Callee {
        addr: 0x0029_f2b0,
        f: ee::s08::SSXAudioSystem_CharacterToSpeechMask,
    };

    pub const SPCH_MAKE_ID_2: Callee = Callee {
        addr: 0x003d_8008,
        f: ee::s14::SPCH_MakeId,
    };


    pub const RIDER_IS_LOADED: Callee = Callee {
        addr: 0x0011_d640,
        f: ee::s00::Rider_IsLoaded,
    };

    pub const RIDER_GET_CHARACTER: Callee = Callee {
        addr: 0x0012_3128,
        f: ee::s00::Rider_GetCharacter,
    };

    pub const SCRIPT_RIDER_GET_PARAMS: Callee = Callee {
        addr: 0x0027_c958,
        f: ee::s07::ScriptRider_GetParams,
    };

    pub const RIDER_SCRIPT_OBJ_MATCHES_RIDER_SLOT: Callee = Callee {
        addr: 0x0028_0560,
        f: ee::s07::RiderScriptObj_MatchesRiderSlot,
    };

    pub const BANK_MONITOR_INSTANCE_RESET: Callee = Callee {
        addr: 0x002a_bb38,
        f: ee::s08::BankMonitorInstance_Reset,
    };


    pub const C_FE_STATE_CHAR_EQUIP_BUY_SELECTED_ITEM: Callee = Callee {
        addr: 0x0019_be80,
        f: ee::s03::cFEStateCharEquip_BuySelectedItem,
    };

    pub const C_FE_STATE_CHAR_EQUIP_UPDATE_MONEY_TEXT: Callee = Callee {
        addr: 0x0019_bba0,
        f: ee::s03::cFEStateCharEquip_UpdateMoneyText,
    };

    pub const C_FE_STATE_CHAR_EQUIP_REMOVE_LIST_ITEM: Callee = Callee {
        addr: 0x0019_b458,
        f: ee::s03::cFEStateCharEquip_RemoveListItem,
    };

    pub const C_FE_STATE_CHAR_EQUIP_REFRESH_ITEM_LIST: Callee = Callee {
        addr: 0x0019_b618,
        f: ee::s03::cFEStateCharEquip_RefreshItemList,
    };

    pub const C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_IMAGE: Callee = Callee {
        addr: 0x001d_47a0,
        f: ee::s04::cFEStateTrophyRoom_ShowTrophyImage,
    };

    pub const C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_INFO: Callee = Callee {
        addr: 0x001d_4890,
        f: ee::s04::cFEStateTrophyRoom_ShowTrophyInfo,
    };


    pub const SHOW_EQUIP_MODE: Callee = Callee {
        addr: 0x0019_a238,
        f: ee::s03::cFEStateCharEquip_ShowEquipMode,
    };

    pub const SHOW_BUY_MODE: Callee = Callee {
        addr: 0x0019_a308,
        f: ee::s03::cFEStateCharEquip_ShowBuyMode,
    };

    pub const SET_CATEGORY_TITLE: Callee = Callee {
        addr: 0x0019_bb_e0,
        f: ee::s03::cFEStateCharEquip_SetCategoryTitle,
    };

    pub const BUILD_ITEM_LIST: Callee = Callee {
        addr: 0x0019_b180,
        f: ee::s03::cFEStateCharEquip_BuildItemList,
    };

    pub const REFRESH_ITEM_LIST: Callee = Callee {
        addr: 0x0019_b618,
        f: ee::s03::cFEStateCharEquip_RefreshItemList,
    };

    pub const SPLINE_GET_PATH_LENGTH: Callee = Callee {
        addr: 0x0034_54e8,
        f: ee::s11::SplineModifier_GetPathLength,
    };

    pub const NET_ACCOUNT_SET_STATE: Callee = Callee {
        addr: 0x001b_70d0,
        f: ee::s03::cFEStateNetAccountMem_SetState,
    };

    pub const PATH_PLAT_FIND_BANK_SLOT: Callee = Callee {
        addr: 0x002a_e230,
        f: ee::s08::PATH_PlatFindBankSlot,
    };

    pub const PATH_PLAT_REMOVE_BANK: Callee = Callee {
        addr: 0x002a_e328,
        f: ee::s08::PATH_PlatRemoveBank,
    };

    pub const MENU_SYS_ITEM_IS_DISABLED_3: Callee = Callee {
        addr: 0x002c_a3b0,
        f: ee::s09::MenuSysItem_IsDisabled,
    };

    pub const ISEQ_HEAP_ALLOC: Callee = Callee {
        addr: 0x0027_2308,
        f: ee::s07::InSeq_HeapAlloc,
    };

    pub const ISEQ_SPAN_TRACK_ACTIVATE_KEY: Callee = Callee {
        addr: 0x0027_30c0,
        f: ee::s07::InSeq_SpanTrackActivateKey,
    };

    pub const ISEQ_SPAN_TRACK_NEXT_KEY: Callee = Callee {
        addr: 0x0027_3140,
        f: ee::s07::InSeq_SpanTrackNextKey,
    };

    pub const PS2_GRID_MESH_CTOR: Callee = Callee {
        addr: 0x0038_5138,
        f: ee::s12::cPS2GridMesh_ctor,
    };

    pub const KEYBOARD_IS_KEY_ENABLED: Callee = Callee {
        addr: 0x001c_e560,
        f: ee::s04::cKeyboardPopup_IsKeyEnabled,
    };

    pub const KEYBOARD_FIND_ENABLED_FROM_TOP: Callee = Callee {
        addr: 0x001c_c090,
        f: ee::s04::cKeyboardPopup_FindEnabledInColumnFromTop,
    };

    pub const FEMODULE_GET_RIDER_PREVIEW_2: Callee = Callee {
        addr: 0x001a_0548,
        f: ee::s03::FEModule_GetRiderPreview,
    };

    pub const FEMODULE_RIDER_PREVIEW_SET_VISIBLE: Callee = Callee {
        addr: 0x0019_e538,
        f: ee::s03::FEModule_RiderPreviewSetVisible,
    };

    pub const INPUTSYNC_IS_READY: Callee = Callee {
        addr: 0x0023_7950,
        f: ee::s06::InputSync_IsReady,
    };

    pub const BOLT_GET_NEXT_NODE: Callee = Callee {
        addr: 0x0019_c8f8,
        f: ee::s03::BoltFileIndexManager_GetNextNode,
    };

    pub const BOLT_GET_PREV_NODE: Callee = Callee {
        addr: 0x0019_c8d0,
        f: ee::s03::BoltFileIndexManager_GetPrevNode,
    };

    pub const BOLT_NODE_INDEX: Callee = Callee {
        addr: 0x0019_c920,
        f: ee::s03::BoltFileIndexManager_NodeIndex,
    };

    pub const RIDER_BASE_RESET_FX: Callee = Callee {
        addr: 0x0011_1890,
        f: ee::s00::RiderBase_ResetFX,
    };

    pub const LOAD_INDICATOR_ADVANCE: Callee = Callee {
        addr: 0x0023_1f80,
        f: ee::s06::LoadIndicator_Advance,
    };

    pub const START_SCREEN_CTOR: Callee = Callee {
        addr: 0x0023_2ae0,
        f: ee::s06::StartScreen_ctor,
    };
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {
    pub(crate) use crate::game::movie::movieplayer::table::UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::table::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::movie::movieplayer::table::UI_LIST_SET_FIRST_FLAGS;
    pub(crate) use crate::game::movie::movieplayer::table::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::movie::movieplayer::table::SSX_AUDIO_SYSTEM_SET_GAME_STATE;
    pub(crate) use crate::game::movie::movieplayer::table::LOBBY_LIST_GET_NAME;
    pub(crate) use crate::game::movie::movieplayer::table::VOIP_RPC_CALL;
    pub(crate) use crate::game::movie::movieplayer::table::MC_CARD_DESTROY;
    pub(crate) use crate::game::ai::computer::table::DELETE;
    pub(crate) use crate::game::ai::computer::table::EA_MEM_FREE;
    pub(crate) use crate::game::ai::computer::table::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::ai::computer::table::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::ai::computer::table::WAIT_SEMA;
    pub(crate) use crate::game::ai::computer::table::WAKEUP_THREAD;
    pub(crate) use crate::game::ai::computer::table::SLEEP_THREAD;
    pub(crate) use crate::game::ai::computer::table::STR_HASH;

    pub const UI_SCREEN_FIND_OBJECT: u32 = 0x0039_d860;

    pub const UI_TEXT_SET_TEXT_ID: u32 = 0x003a_1000;

    pub const BXSTRING_ASSIGN: u32 = 0x0031_8448;

    pub const BXSTRING_RELEASE: u32 = 0x0031_80b8;

    pub const UI_MENU_RESET_SCROLL: u32 = 0x0039_b760;

    pub const PS2_MEM_CARD_CREATE: u32 = 0x002c_22c8;

    pub const PATH_STOP_TRACKS: u32 = 0x003d_2a68;

    pub const BUILTIN_DELETE: u32 = 0x0031_7e50;

    pub const VOIP_RPC_START: u32 = 0x003f_6ab8;

    pub const VOIP_RPC_SET_PARAMS: u32 = 0x003f_6c10;

    pub const AUDIO_GET_RACE_MAN: u32 = 0x0028_b1d8;

    pub const ASYNC_FILE_GET_RESULT: u32 = 0x003d_f9d8;

    pub const SCRIPT_ENGINE_GET_RESULT_VTABLE: u32 = 0;

    pub const RIDER_MODEL_FIND: u32 = 0;

    pub const BE_GET_INSTANCE: u32 = 0x0014_dc80;

    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;

    pub const BE_NEW_RACE_SET_GAME_MODE: u32 = 0x0014_5108;

    pub const BE_NEW_RACE_SET_EVENT_TYPE: u32 = 0x0014_4df0;

    pub const REPLAY_END_SAVE: u32 = 0x0027_0ed8;

    pub const REPLAY_BEGIN_PLAYBACK: u32 = 0x0026_f8a0;

    pub const OPERATOR_NEW_TAG: u32 = 0x0031_7e30;

    pub const UI_LIST_FIRST: u32 = 0x0039_7840;

    pub const UI_LIST_NODE_IS_SENTINEL: u32 = 0x0039_7718;

    pub const UI_TEXT_SET_WIDE_STRING: u32 = 0x003a_0e90;

    pub const SPRINTF: u32 = 0x002c_27c0;

    pub const EAC_GET_UNPACKED_SIZE: u32 = 0x003b_4818;

    pub const EAC_UNPACK: u32 = 0x003b_47f8;

    pub const COLL_SPHERE_TREE_TRIANGLE: u32 = 0x0032_ca78;

    pub const COLL_HULL_TRIANGLE: u32 = 0x0032_8030;

    pub const COLL_HULL_TRIANGLE2: u32 = 0x0032_aa28;

    pub const COLL_SPHERE_CALC: u32 = 0x0032_df28;

    pub const STR_CASECMP: u32 = 0x0041_aa88;
}

// ---------------------------------------------------------------------------
// Callees the generator emits as `call_indirect` (cross-shard or indirect).

pub(crate) mod addr {
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STR_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_SCREEN_START_THREAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CLOSE_UNPINNED_POPUPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEM_MAN_ALLOC;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCASECMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRLEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPRINTF;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEMSET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_POP_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_PUSH_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::FE_NET_SEND;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MC_STATUS_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW_BOLTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE_9;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::HUD_SHOW_NUMBER_DELAYED;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPATIAL_INSERT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::POSITION_MODIFIER_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_PAINTER_2D;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_OBJECT_BY_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BANK_MONITOR_ADD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_TEARDOWN;

    pub const __BUILTIN_DELETE: u32 = 0x0031_7e50;

    pub const EA_MEM_FREE: u32 = 0x0031_7e98;

    pub const C_BX_STRING_DTOR: u32 = 0x0031_8260;

    pub const __FIXUNSSFSI: u32 = 0x0041_39f8;

    pub const FONT_DTOR: u32 = 0x0039_16c0;

    pub const OPERATOR_NEW_NAMED: u32 = 0x0031_7e30;

    pub const MEM_FREE: u32 = 0x0031_7e98;

    pub const MEMCPY: u32 = 0x0041_605c;

    pub const FILE_LOAD: u32 = 0x003e_18c8;

    pub const UI_TEXT_SET_ASCII: u32 = 0x003a_0c50;

    pub const UI_TEXT_SET_UNICODE: u32 = 0x003a_1000;

    pub const USTR_LENGTH: u32 = 0x003d_bca0;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {
    pub(crate) use crate::game::fe::festaterewards::table_callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::festaterewards::table_callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRLEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRCPY;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OBJECT_PUSH;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_ADD_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_GET_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_GET_STRING4;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUDDY_PRODUCT_COMPARE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SSX_AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_REMOVE_SCRIPT_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SET_SCRIPT_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SCRIPT_CLEANUP;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_RICH_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::CODEC_DECODE_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_INTERFACE_FV;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_GET_BASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUY_POPUP_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_POSTERS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TRADING_CARDS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CONCEPT_ARTS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_VIDEOS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TOYS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CHEAT_CHARS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OPEN_FINISH_SCREEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::AI_SET_AI_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_SET_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_TEXT_FROM_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BENEW_PLAYER_INT_GET_CHAR_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_SMALL_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_BIG_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::APP_MAN_SET_NEXT_MODULE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UPLOAD_TEXTURE_PIXELS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::MUSIC_SLOT_RELEASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_STATE_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SECTION_TABLE_CTOR;
}

// ---------------------------------------------------------------------------
// cFEStateSelectPersona_OnWidgetCreated (0x001ad308).

/// The name-hash table of the persona screen (title, locked label, name and
/// description strings).
pub(crate) mod select_persona {

    pub const TITLE: u32 = 0x0046_2e00;

    pub const LOCKED: u32 = 0x0046_2e10;

    pub const NAME: u32 = 0x0046_2e20;

    pub const DESC: u32 = 0x0046_2e30;

    pub const DESC_ID: u32 = 0x0046_2e40;

    /// `cUIText.+0x38` is the widget's name hash.
    pub const HASH: u32 = 0x38;
}

// ---------------------------------------------------------------------------
// NetConfigAuthPopup_ctor (0x001b6288).

/// `NetConfigAuthPopup` fields.
pub(crate) mod net_config_auth {

    /// Base class (`NetFEState` vtable).
    pub const VTABLE: u32 = 0x8;

    /// The popup id.
    pub const ID: u32 = 0xc;

    /// The title text buffer.
    pub const TITLE: u32 = 0x6a8;

    pub const F6D0: u32 = 0x6d0;

    pub const F6D4: u32 = 0x6d4;

    pub const F6E4: u32 = 0x6e4;

    pub const F6F4: u32 = 0x6f4;

    pub const F6F8: u32 = 0x6f8;

    /// The `cBXString`-like buffer freed and reallocated here.
    pub const BUF: u32 = 0x6fc;
}

// ---------------------------------------------------------------------------
// cFEStateAuthDNASError_OnWidgetCreated (0x001b6bb8).

/// Name-hash table of the DNAS error screen.
pub(crate) mod dnas_error {

    pub const STATUS: u32 = 0x004a_1ba8;

    pub const STATUS2: u32 = 0x004a_1bb0;

    pub const STATUS3: u32 = 0x004a_1bb8;

    pub const RETRY: u32 = 0x0046_4250;

    pub const MESSAGE: u32 = 0x0046_42c0;
}


/// Offset of the async manager's state word.
pub(crate) const FE_ASYNC_MANAGER_STATE: u32 = 0x114;


/// The `FEAsyncManager_GetState` / `_SetState` prologue scale: the index is
/// multiplied by this before being added to `this`.
pub(crate) const FE_ASYNC_MANAGER_STRIDE: u32 = 0x11c;


/// `u32 FEAsyncManager_GetState(FEAsyncManager* m, int index)` (0x001a39c0):
/// returns the state word for slot `index`. The original computes the element
/// address with a signed `mult` of `index` by 0x11c (leaving `v1` = LO).
pub fn fe_async_manager_get_state(c: &mut Cpu) {
    c.cycles += 5;
    let base = abi::arg(c, 0) as u64;
    let index = abi::arg(c, 1);
    c.g[V0][0] = FE_ASYNC_MANAGER_STRIDE as u64;
    let lo = abi::mult(c, index, FE_ASYNC_MANAGER_STRIDE);
    c.g[V1][0] = sx(lo);
    c.g[A0][0] = sx(lo.wrapping_add(base as u32));
    c.g[V0][0] = Addr(c.g[A0][0] as u32)
        .off(FE_ASYNC_MANAGER_STATE)
        .lw(c);
    abi::ret(c);
}


/// `void FEAsyncManager_SetState(FEAsyncManager* m, int index, u32 state)`
/// (0x001a39d8): stores `state` into slot `index`. Same stride as the getter;
/// `v0` is left holding 0x11c (the mult writes `v1`).
pub fn fe_async_manager_set_state(c: &mut Cpu) {
    c.cycles += 5;
    let base = abi::arg(c, 0) as u64;
    let index = abi::arg(c, 1);
    c.g[V0][0] = FE_ASYNC_MANAGER_STRIDE as u64;
    let lo = abi::mult(c, index, FE_ASYNC_MANAGER_STRIDE);
    c.g[V1][0] = sx(lo);
    c.g[A0][0] = sx(lo.wrapping_add(base as u32));
    Addr(c.g[A0][0] as u32)
        .off(FE_ASYNC_MANAGER_STATE)
        .sw(c, c.g[A2][0]);
    abi::ret(c);
}


/// `int cFEStateNetMessenger_GetListIndex(cFEStateNetMessenger *self, int i)`
/// (0x1a4740): `*(u32*)(self+0x6f4) + i`.
pub fn cfe_state_net_messenger_get_list_index(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x6f4).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Front-end "create account" flow.

/// `bool cFEStateCreateStep1_AllFieldsFilled(cFEStateCreateStep1 *self)`
/// (0x1b9558): true when the four pointers at `*(u32*)(self+0x6d0) + 0/4/8/0xc`
/// all point at a block whose word at `-8` is non-zero.
pub fn cfe_state_create_step1_all_fields_filled(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A0][0] = Addr(abi::arg(c, 0)).off(0x6d0).lw(c);
    c.g[A2][0] = 0x1;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xffff_fff8).lw(c); // -8
    let mut a1 = 0u64;
    if c.g[V1][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x4).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xffff_fff8).lw(c);
        if c.g[V1][0] != 0 {
            a1 = c.g[A2][0];
        }
    }
    c.cycles += 3;
    c.g[A1][0] = a1;
    c.g[V1][0] = c.g[A1][0];
    let t = c.g[V1][0] == 0;
    c.g[V0][0] = 0;
    if !t {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x8).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xffff_fff8).lw(c);
        c.g[V0][0] = 0;
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[A2][0];
        }
    }
    c.cycles += 3;
    c.g[V1][0] = c.g[V0][0];
    let t = c.g[V1][0] == 0;
    c.g[V0][0] = 0;
    if !t {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0xc).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xffff_fff8).lw(c);
        c.g[V0][0] = 0;
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[A2][0];
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `bool cFEStateCreateStep1_CharInRange(void *self, int lo, int ch, int hi)`
/// (0x1b9f68): treats `lo`, `ch` and `hi` as signed bytes and returns whether
/// `lo <= ch <= hi`.
pub fn cfe_state_create_step1_char_in_range(c: &mut Cpu) {
    c.cycles += 7;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 24);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 24);
    c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 24) as u32);
    c.g[A3][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
    c.g[V1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
    if c.g[A3][0] != 0 {
        c.cycles += 1;
        c.g[V0][0] = 0;
    } else {
        c.cycles += 3;
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
        c.g[V0][0] = 0x1;
        if c.g[V1][0] != 0 {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `bool cFEStateCreateStep2_IsLeapYear(void *self, int year)` (0x1bb968):
/// true for years divisible by 4 that are not divisible by 100 (unless also by
/// 400), computed with the original's `div`/`mfhi` sequence.
pub fn cfe_state_create_step2_is_leap_year(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A1][0] & 0x3;
    let mut a0 = 0u64;
    if c.g[V0][0] == 0 {
        c.cycles += 5;
        c.g[V0][0] = 0x64;
        a0 = 1;
        c.div(0, A1, V0);
        c.cycles += 3;
        c.g[V1][0] = c.hi[0];
        c.g[V0][0] = 0x190;
        if c.g[V1][0] == 0 {
            c.cycles += 3;
            c.div(0, A1, V0);
            c.cycles += 2;
            c.g[V1][0] = c.hi[0];
            a0 = (c.g[V1][0] < 0x1u64) as u64;
        }
    }
    c.cycles += 2;
    c.g[A0][0] = a0;
    c.g[V0][0] = a0;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Front-end popup confirm widget (option rows).

/// `void cFEPopupConfirm_CfgSetOptionString(Cfg* cfg, int row, void *s)`
/// (0x1c5780): stores the string `s` in an option row of kind 2.
pub fn cfepopupconfirm_cfg_set_option_string(c: &mut Cpu) {
    c.cycles += 7;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
    c.g[V0][0] = 0x2;
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[A0][0] = c.g[V1][0];
    Addr(c.g[V1][0] as u32).off(0x28).sw(c, c.g[A2][0]);
    Addr(c.g[A0][0] as u32).off(0x24).sw(c, c.g[V0][0]);
    abi::ret(c);
}


/// `void cFEPopupConfirm_CfgSetOptionWideString(Cfg* cfg, int row, void *s)`
/// (0x1c57a0): same as [`cfepopupconfirm_cfg_set_option_string`] with kind 1.
pub fn cfepopupconfirm_cfg_set_option_wide_string(c: &mut Cpu) {
    c.cycles += 7;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
    c.g[V0][0] = 0x1;
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[A0][0] = c.g[V1][0];
    Addr(c.g[V1][0] as u32).off(0x28).sw(c, c.g[A2][0]);
    Addr(c.g[A0][0] as u32).off(0x24).sw(c, c.g[V0][0]);
    abi::ret(c);
}


/// `void cFEPopupConfirm_CfgSetOptionTextId(Cfg* cfg, int row, int id)`
/// (0x1c57c0): same as [`cfepopupconfirm_cfg_set_option_string`] with kind 3.
pub fn cfepopupconfirm_cfg_set_option_text_id(c: &mut Cpu) {
    c.cycles += 7;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
    c.g[V0][0] = 0x3;
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[A0][0] = c.g[V1][0];
    Addr(c.g[V1][0] as u32).off(0x28).sw(c, c.g[A2][0]);
    Addr(c.g[A0][0] as u32).off(0x24).sw(c, c.g[V0][0]);
    abi::ret(c);
}


/// `void* cFEPopupConfirm_CfgGetOption(Cfg* cfg, int row)` (0x1c5800):
/// `cfg + row*8 + 0x24`.
pub fn cfepopupconfirm_cfg_get_option(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x24));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}


/// `void cFEPopupConfirm_CfgSetOptionConfirmWide(Cfg* cfg, int row, void *s, int arg)`
/// (0x1c5838): sets up a kind-1 option row (`row*0xc` stride).
pub fn cfepopupconfirm_cfg_set_option_confirm_wide(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = 0xc;
    c.g[T0][0] = 0x1;
    c.mult(0, A1, A1, V0); // row *= 0xc
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    Addr(c.g[V0][0] as u32).off(0x58).sw(c, c.g[A2][0]);
    c.g[V1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[V0][0];
    Addr(c.g[V1][0] as u32).off(0x54).sw(c, c.g[T0][0]);
    Addr(c.g[A0][0] as u32).off(0x5c).sw(c, c.g[A3][0]);
    abi::ret(c);
}


/// `void cFEPopupConfirm_CfgClearOptionConfirm(Cfg* cfg, int row)`
/// (0x1c5860): zeroes the option-confirm kind field of `row*0xc`.
pub fn cfepopupconfirm_cfg_clear_option_confirm(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0xc;
    c.mult(0, V1, A1, V0);
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    Addr(c.g[A0][0] as u32).off(0x54).sw(c, 0);
    abi::ret(c);
}


/// `void cFEPopupConfirm_CfgSetOptionConfirmTextId(Cfg* cfg, int row, int id, int arg)`
/// (0x1c5878): like [`cfepopupconfirm_cfg_set_option_confirm_wide`] with
/// kind 3.
pub fn cfepopupconfirm_cfg_set_option_confirm_text_id(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = 0xc;
    c.g[T0][0] = 0x3;
    c.mult(0, A1, A1, V0);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    Addr(c.g[V0][0] as u32).off(0x58).sw(c, c.g[A2][0]);
    c.g[V1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[V0][0];
    Addr(c.g[V1][0] as u32).off(0x54).sw(c, c.g[T0][0]);
    Addr(c.g[A0][0] as u32).off(0x5c).sw(c, c.g[A3][0]);
    abi::ret(c);
}


/// `void* cFEPopupConfirm_CfgGetOptionConfirm(Cfg* cfg, int row)`
/// (0x1c58d0): `cfg + row*0xc + 0x54`.
pub fn cfepopupconfirm_cfg_get_option_confirm(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = 0xc;
    c.mult(0, V0, A1, V1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x54));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// AsyncFile pool / serial.

/// The serial counter the file system hands out.
pub(crate) const ASYNC_SERIAL: u32 = 0x0052_0000 - 0x6400;


/// `void AsyncFile_AssignSerial(AsyncFile *f)` (0x3def40): bump the pool's
/// serial counter by 0x100 (or reset it to 0x100 after the wrap to 0) and OR
/// its low byte's zero-extended... it stores `serial | f->serial_byte(+0)`.
pub fn async_file_assign_serial(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = 0x0052_0000;
    let value = Addr(ASYNC_SERIAL).lw(c) as u32;
    let next = value.wrapping_add(0x100);
    c.g[V0][0] = sx(next);
    if next != 0 {
        Addr(ASYNC_SERIAL).sw(c, sx(next));
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0x100;
        Addr(ASYNC_SERIAL).sw(c, 0x100);
    }
    c.cycles += 5;
    let serial = Addr(ASYNC_SERIAL).lw(c);
    c.g[V1][0] = serial;
    let base = Addr(abi::arg(c, 0));
    let b = base.lbu(c);
    c.g[V0][0] = b | serial;
    base.sw(c, c.g[V0][0]);
    abi::ret(c);
}


/// `void NetFEState_SetSelectedUser(this, const char *name)` (0x001a9890):
/// assign the name to the `cBXString` at `+0x6c0`.
pub fn netfe_state_set_selected_user(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0)),
        callee::CBXSTRING_ASSIGN_CSTR,
        0x001a_98a0,
    )
}


/// `void cFEStateStats_ResetResultsPage(this, int slot)` (0x001c4b78): clear
/// the page field at `+0x6e8`, then request the stats page from the lobby
/// manager (`gp - 0xc8`).
pub fn cfe_state_stats_reset_results_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x6e8), 0);
        c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
        call_ptr(c, callee::LOBBY_MAN_REQUEST_STATS_PAGE, 0x001c_4b8c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// NetConfigAuthPopup_OnWidgetAction.

/// `void NetConfigAuthPopup_OnWidgetAction(Popup* p, int a1, int msg)`
/// (0x1b6770): forward every action but the "auth ok" one (0xd) to the base
/// state.
pub fn net_config_auth_popup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xd;
        let handled = c.g[A2][0] == 0xd;
        frame.save_ra(c, 0);
        if !handled {
            c.cycles += 2;
            callee::NET_FE_STATE_ON_WIDGET_ACTION.call(c, 0x001b_6788)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Front-end state hooks: virtual methods on a member object.

/// `cFEStateChallengePopup_TransitionIn(this)` 0x1adfb8: slot +0x100 of the
/// object's vtable at `*(this + 8)`, then `true`.
pub fn c_fe_state_challenge_popup_transition_in(c: &mut Cpu) {
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
        call_ptr(c, m.func as u32, 0x001a_dfd4)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateNetConfig_Shutdown(this)` 0x1b4168: tear down the state's widget
/// list, then hand the audio system's singleton to its game-state teardown.
pub fn c_fe_state_net_config_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, table::UI_STATE_SHUTDOWN, 0x001b_4178)?;
        call_ptr(c, table::SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x001b_4180)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, table::SSX_AUDIO_SYSTEM_SET_GAME_STATE, 0x001b_4188)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// The `OnWidgetAction` hooks: `A1` is forwarded from the caller, the method
/// is slot +0xc0 of the vtable at `*(*(this + 0x20) + 8)`.
pub(crate) fn widget_action_v24(c: &mut Cpu, ret: u32) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let obj = lw_at(c, this as u64, 0x20);
        c.g[A3][0] = obj;
        let vt = lw_at(c, obj, 8);
        c.g[V1][0] = vt;
        let m = VSlot::read(c, vt, 0xc0);
        c.g[A0][0] = m.delta;
        c.g[V0][0] = m.func;
        c.g[A0][0] = sx((obj as u32).wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `NetConfigAuthPopup_v24(this, a1, a2)` 0x1b6740: forward the widget action.
pub fn net_config_auth_popup_v24(c: &mut Cpu) {
    widget_action_v24(c, 0x001b_6760)
}


/// `cFEStateAuthDNASError_v24(this, a1, a2)` 0x1b6e10: forward the widget
/// action.
pub fn c_fe_state_auth_dnas_error_v24(c: &mut Cpu) {
    widget_action_v24(c, 0x001b_6e30)
}


/// `cFEStateNetAccountMem_v24(this, a1, a2)` 0x1b8860: forward the widget
/// action.
pub fn c_fe_state_net_account_mem_v24(c: &mut Cpu) {
    widget_action_v24(c, 0x001b_8880)
}


/// `cFEStateNetCreateRoomPopup_v31(this, a1, a2)` 0x1bed28: forward the widget
/// action.
pub fn c_fe_state_net_create_room_popup_v31(c: &mut Cpu) {
    widget_action_v24(c, 0x001b_ed48)
}


/// `cFEStateLobbyMenu_RefreshPlayerList(this)` 0x1be510: rebuild the lobby's
/// player list, then refresh the scroll arrows.
pub fn c_fe_state_lobby_menu_refresh_player_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        callee::C_FE_STATE_LOBBY_MENU_FILL_PLAYER_LIST.call(c, 0x001b_e524)?;
        c.g[A0][0] = c.g[S0][0];
        callee::C_FE_STATE_LOBBY_MENU_UPDATE_PLAYER_SCROLL_ARROWS.call(c, 0x001b_e52c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Net / memcard hooks.

/// `void NetFEState_OnChildEventIgnoreClose(self, arg, msg, arg3)` (0x1a9638):
/// the child-event handler of the states that ignore a "close" message
/// (0xf) and a "destroy" message (0x14); everything else goes to the base
/// handler `NetFEState_OnChildEventDefault`.
pub fn net_fe_state_on_child_event_ignore_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xf;
        let msg = c.g[A2][0];
        frame.save_ra(c, 0);
        if msg == 0xf {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x14;
            if msg == 0x14 {
                frame.restore_ra(c, 0);
            } else {
                c.cycles += 2;
                call_ptr(c, callee::NET_FE_ON_CHILD_EVENT_DEFAULT, 0x001a_965c)?;
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateNetAccountMem_StoreAccountStrings(self)` (0x1b7778): copy the
/// account name (`self+0x6e0`) and password (`self+0x6f1`) into the shared
/// login manager.
pub fn cfe_state_net_account_mem_store_account_strings(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        c.g[A0][0] = sx(p(this, 0x6e0));
        call_ptr(c, callee::SHARED_LOGIN_SET_ACCOUNT_NAME, 0x001b_7790)?;
        c.g[A0][0] = sx(p(this, 0x6f1));
        call_ptr(c, callee::SHARED_LOGIN_SET_ACCOUNT_PASSWORD, 0x001b_7798)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEPopupConfirm` config destructor (vtable at `+0xc8`).
pub fn c_fe_popup_confirm_cfg_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_cca8, 0xc8, 0x001c_5774);
}


/// `FEScreenBlank* FEScreenBlank_ctor(FEScreenBlank* self)` (0x1abf70):
/// construct the net FE state and install the FEScreenBlank vtable.
pub fn fe_screen_blank_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::NET_FE_STATE_CTOR.call(c, 0x001a_bf84)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4908));
        frame.restore_ra(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(8)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}


/// `void cFEStateLobbyMenu_OnKeyboardDone(cFEStateLobbyMenu* self)`
/// (0x1bd428): when the pending room request is in state 8, join the room
/// with the typed name and room slot.
pub fn fe_lobby_menu_on_keyboard_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 8;
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x69c)).lw(c);
        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x18)).lw(c);
        let pending = c.g[V0][0] != c.g[V1][0];
        frame.restore_ra(c, 0);
        if !pending {
            c.cycles += 4;
            c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xb80)).lw(c);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x74));
            callee::LOBBY_MENU_REQUEST_JOIN_ROOM.call(c, 0x001b_d450)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}


/// `cFEPopupConfirm* cFEPopupConfirm_CfgCtor(cFEPopupConfirm* self)`
/// (0x1c55f8): install the vtable and reset the configuration fields.
pub fn fe_popup_confirm_cfg_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0047_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3358));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0xc8)).sw(c, c.g[V0][0]);
        callee::POPUP_CONFIRM_CFG_RESET.call(c, 0x001c_5618)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}


/// `int cFEStateOnlineCharacterSelect_CharToListIndex(int character)` (0x1b0f28):
/// index of `character` in the ten-entry character list, or -1.
pub fn char_to_list_index(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x0044_0000;
    c.g[V1][0] = 0;
    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf68));
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lb(c);
        if c.g[A0][0] != c.g[V0][0] {
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 10i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            if !more {
                c.cycles += 2;
                c.g[V0][0] = u64::MAX;
                abi::ret(c);
                return;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001b_0f38);
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[V1][0];
            abi::ret(c);
            return;
        }
    }
}


/// `void cFEStateChatRoom_OnActivate(cFEStateChatRoom* self, void* arg)`
/// (0x1B3528): refresh the player quality, then run the network state's own
/// activation.
pub fn fe_state_chat_room_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY.call(c, 0x001b_3544)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::NET_FE_STATE_ON_ACTIVATE.call(c, 0x001b_3550)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int cFEStateNetQuickPlayPopup_OnChildEvent(self, child, event)` (0x1C1B48):
/// the options popup (id 0x82F) handles the event itself, every other child
/// goes to the network state.
pub fn fe_state_net_quick_play_popup_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x82f;
        frame.save_ra(c, 0);
        let event = Addr(r(c, A1).wrapping_add(0x18)).lw(c);
        c.g[V1][0] = event;
        if event == c.g[V0][0] {
            c.cycles += 4;
            callee::NET_QUICK_PLAY_POPUP_ON_OPTIONS_EVENT.call(c, 0x001c_1b68)?;
        } else {
            c.cycles += 3;
            callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001c_1b78)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// `int cFEStateStats_OnWidgetNotify(self, widget, kind, value)` (0x1C4B98):
/// a stats-tab change (kind 4) records the tab and rebuilds the results list;
/// the other kinds only report their widget id.
pub fn fe_state_stats_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 4;
        let kind = c.g[A2][0];
        frame.save_ra(c, 0);
        if kind == c.g[V0][0] {
            c.cycles += 3;
            let tab = c.g[A3][0];
            Addr(r(c, A0).wrapping_add(0x6e8)).sw(c, tab);
            callee::FE_STATE_STATS_REFRESH_RESULTS.call(c, 0x001c_4bc4)?;
            c.g[V0][0] = 0x101;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 6;
            if kind == c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = 0x100;
            } else {
                c.g[V0][0] = 0x101;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// The four `NetFEState` constructors: base constructor, class id, vtable,
/// and (for the online main menu) an explicit "no parent" argument.
pub(crate) fn net_fe_state_ctor(c: &mut Cpu, id: u32, vt: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::NET_FE_STATE_CTOR_2.call(c, ret)?;
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = id as u64;
        c.g[V1][0] = 0x47_0000;
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx(vt);
        Addr(p(this, 8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void* cFEStateNetSSXWorld_ctor(void* self)` (0x1c1c50): class 0x4a.
pub fn c_fe_state_net_ssx_world_ctor(c: &mut Cpu) {
    net_fe_state_ctor(c, 0x4a, 0x46_aa18, 0x001c_1c68)
}


/// `void* cFEStateNetQuickMessages_ctor(void* self)` (0x1c20f8): class 0x4b.
pub fn c_fe_state_net_quick_messages_ctor(c: &mut Cpu) {
    net_fe_state_ctor(c, 0x4b, 0x46_a908, 0x001c_2110)
}


/// `void* cFEStateNetNews_ctor(void* self)` (0x1c5018): class 0x43.
pub fn c_fe_state_net_news_ctor(c: &mut Cpu) {
    net_fe_state_ctor(c, 0x43, 0x46_8168, 0x001c_5030)
}


/// `int cFEUseExistingAccount_ValidateFields(State* s)` (0x1ac408): true
/// when both the account name and the password are non-empty.
pub fn c_fe_use_existing_account_validate_fields(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CHECK_ACCOUNT_NAME.call(c, 0x001a_c41c)?;
        let have_name = c.g[V0][0] != 0;
        c.g[V1][0] = 0;
        if have_name {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::CHECK_PASSWORD.call(c, 0x001a_c42c)?;
            c.g[V1][0] = 1;
            if c.g[V0][0] == 0 {
                c.g[V1][0] = 0;
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = c.g[V1][0];
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int cFEStateNetConfig_OnPopupEventPassthrough(State* s, int a1, int a2)`
/// (0x1b4c00): forward child events, except that the two popup event ids
/// (0xf and 0x14) take the "own popup" path with one extra cycle.
pub fn c_fe_state_net_config_on_popup_event_passthrough(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xf;
        let popup = c.g[A2][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        if !popup {
            c.cycles += 3;
            c.g[V0][0] = 0x14;
            if c.g[A2][0] != c.g[V0][0] {
                // The passthrough path.
                c.cycles += 3;
                callee::NET_FE_STATE_ON_CHILD_EVENT_2.call(c, 0x001b_4c34)?;
                frame.restore_ra(c, 0);
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 4;
        callee::NET_FE_STATE_ON_CHILD_EVENT_2.call(c, 0x001b_4c24)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateSelectPersona* cFEStateSelectPersona_ctor(self)` (0x1ad130): net
/// front-end state constructor (with a null engine argument), then the
/// persona-select vtable/state and one cleared field.
pub fn fe_state_select_persona_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::NET_FE_STATE_CTOR_3.call(c, 0x001a_d148)?;
        let this = r32(c, S0);
        c.g[V0][0] = 0x38;
        const_reg(c, V1, 0x47_0000, -0x70b0);
        sw(c, this, 0x0c, c.g[V0][0]);
        sw(c, this, 0x08, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        sw(c, this, 0x6d0, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void cFEStateChallengePopup_v33(self, event, code)` (0x1af3e0): on the
/// two popup codes close the state manager's popup list, otherwise pass the
/// event to the net front-end state.
pub fn fe_state_challenge_popup_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xf;
        let is_f = c.g[A2][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        let popup_code = if is_f {
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x14;
            c.g[A2][0] == c.g[V0][0]
        };
        if popup_code {
            c.cycles += 5;
            c.g[A0][0] = c.lw(at(c, A0, 0x10));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, callee::UI_LIST_CLOSE_ALL_POPUPS, 0x001a_f408)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            callee::NET_FE_STATE_ON_CHILD_EVENT_3.call(c, 0x001a_f418)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}


/// `void cFEStateChallengeReceivedPopup_v33(self, event, code)` (0x1b0628):
/// same dispatch as [`fe_state_challenge_popup_v33`] for the "challenge
/// received" popup.
pub fn fe_state_challenge_received_popup_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xf;
        let is_f = c.g[A2][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        let popup_code = if is_f {
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x14;
            c.g[A2][0] == c.g[V0][0]
        };
        if popup_code {
            c.cycles += 5;
            c.g[A0][0] = c.lw(at(c, A0, 0x10));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, callee::UI_LIST_CLOSE_ALL_POPUPS, 0x001b_0650)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            callee::NET_FE_STATE_ON_CHILD_EVENT_3.call(c, 0x001b_0660)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}


/// `bool cFEStateOnlineCharacterSelect_HandleInput(self, input)`
/// (0x1b1590): unless the menu is disabled (or the pad global is clear),
/// let the input object handle the event and return its inverted result.
pub fn fe_state_online_character_select_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(at(c, A0, 0x6dc));
        if c.g[V0][0] != 0 {
            c.g[V1][0] = c.lw(at(c, A1, 0x08));
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw(gp_2(c, 0x2dc));
            if c.g[V0][0] != 0 {
                c.g[V0][0] = 0;
                frame.restore_ra(c, 0);
                c.cycles += 3;
                frame.close(c);
                return Ok(());
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw(at(c, A1, 0x08));
        }
        c.cycles += 5;
        c.g[A0][0] = c.lh(p_2(r32(c, V1), 0x30));
        c.g[V0][0] = c.lw(p_2(r32(c, V1), 0x34));
        let target = r32(c, V0);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001b_15c4)?;
        c.g[V0][0] = c.g[V0][0] ^ 0x1;
        frame.restore_ra(c, 0);
        c.cycles += 3;
        frame.close(c);
        Ok(())
    });
}


/// `cFEStateCreateStep2* cFEStateCreateStep2_ctor(self)` (0x1ba3e0): net
/// front-end state constructor with a null engine, then the create-step-2
/// vtable/state and one cleared field.
pub fn fe_state_create_step2_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::NET_FE_STATE_CTOR_3.call(c, 0x001b_a3f8)?;
        let this = r32(c, S0);
        c.g[V0][0] = 0x4e;
        const_reg(c, V1, 0x47_0000, -0x7a50);
        sw(c, this, 0x0c, c.g[V0][0]);
        sw(c, this, 0x08, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        sw(c, this, 0x6a8, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// The quick-play popup's retry interval: 30 frames (0.5 s at 60 Hz). The
/// countdown consumes one tick per update, so the value reloaded is the
/// interval in ticks.
const QUICK_PLAY_RETRY_FRAMES: u32 = 30;


/// `void cFEStateNetQuickPlayPopup_Update(self)` (0x1c1038): count the
/// popup's timeout down (reloading the interval when it passes 0) while it is
/// armed, then run the net front-end state update.
pub fn fe_state_net_quick_play_popup_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(at(c, A0, 0x6ec));
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = c.lw(at(c, A0, 0x6e8));
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let nonzero = c.g[V1][0] != 0;
            sw(c, r32(c, A0), 0x6e8, c.g[V0][0]);
            if !nonzero {
                c.cycles += 2;
                c.g[V0][0] = tick::ticks_from_frames60(c, QUICK_PLAY_RETRY_FRAMES) as u64;
                sw(c, r32(c, A0), 0x6e8, c.g[V0][0]);
            }
        }
        c.cycles += 5;
        callee::NET_FE_STATE_UPDATE.call(c, 0x001c_1070)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// A `gp_3`-relative global.
#[inline(always)]
pub(crate) fn gp_3(c: &Cpu, off: i32) -> u32 {
    abi::r32(c, GP).wrapping_add(off as u32)
}


// ---------------------------------------------------------------------------
// NetFEState child events.

/// A `NetFEState` child event handler: an event whose kind (`event+0x18`) is
/// `0x835` is delivered to the state's own virtual handler (slot 0x108 of the
/// vtable at `self+8`, called with `self` adjusted by the slot's delta), every
/// other kind to `NetFEState_OnChildEvent`. `ret_virtual`/`ret_other` are the
/// original's return addresses (its 0x10 frame holds only `ra`).
pub(crate) fn child_event_dispatch(c: &mut Cpu, ret_virtual: u32, ret_other: u32) -> Flow {
    c.cycles += 6;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V0][0] = 0x835;
    let event = abi::r32(c, A1);
    c.g[V1][0] = Addr(event.wrapping_add(0x18)).lw(c);
    let own_handler = c.g[V1][0] == 0x835;
    c.g[A3][0] = c.g[A0][0];
    if own_handler {
        c.cycles += 7;
        let this = abi::r32(c, A3);
        c.g[V0][0] = Addr(this.wrapping_add(8)).lw(c);
        let vtable = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(vtable.wrapping_add(0x108)).lh(c);
        c.g[V1][0] = Addr(vtable.wrapping_add(0x10c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(this.wrapping_add(abi::r32(c, A0)));
        call_ptr(c, target, ret_virtual)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        return Ok(());
    }
    c.cycles += 3;
    c.g[A0][0] = c.g[A3][0];
    callee::NET_FE_STATE_ON_CHILD_EVENT_4.call(c, ret_other)?;
    frame.restore_ra(c, 0);
    c.cycles += 2;
    frame.close(c);
    Ok(())
}


/// `void cFEStateChallengePopup_OnChildEvent(self, event, arg)` (0x1af428):
/// the challenge popup's child-event handler; a `0x835` event is the popup's
/// own (see [`child_event_dispatch`]).
pub fn c_fe_state_challenge_popup_on_child_event(c: &mut Cpu) {
    guest(c, |c| child_event_dispatch(c, 0x001a_f454, 0x001a_f464))
}


/// `void cFEStateChallengeReceivedPopup_OnChildEvent(self, event, arg)`
/// (0x1b05e0): the "challenge received" popup's child-event handler, the same
/// shape as [`c_fe_state_challenge_popup_on_child_event`].
pub fn c_fe_state_challenge_received_popup_on_child_event(c: &mut Cpu) {
    guest(c, |c| child_event_dispatch(c, 0x001b_060c, 0x001b_061c))
}


/// `int cFEStateChatRoom_OnWidgetNotify(self, sender, event, value)`
/// (0x1b3568): the chat room's widget notification. Only event 4 (a list
/// selection) counts: from the player list (`+0x6fc`) it stores the selected
/// value at `+0x704` and rebuilds the list; from the buddy list (`+0x700`) it
/// stores it at `+0x708`. Returns the handled code 0x101.
pub fn c_fe_state_chat_room_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 4;
        let selection = c.g[A2][0] == 4;
        frame.save_ra(c, 0);
        let self_ = abi::r32(c, A0);
        let sender = c.g[A1][0];
        let value = c.g[A3][0];
        if selection {
            c.cycles += 3;
            c.g[V0][0] = Addr(self_.wrapping_add(0x6fc)).lw(c);
            if sender == c.g[V0][0] {
                c.cycles += 5;
                Addr(self_.wrapping_add(0x704)).sw(c, value);
                c.g[A1][0] = 0;
                callee::CHAT_ROOM_REFRESH_PLAYER_LIST.call(c, 0x001b_3590)?;
                c.g[V0][0] = 0x101;
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.g[V0][0] = Addr(self_.wrapping_add(0x700)).lw(c);
            c.cycles += 2;
            if sender == c.g[V0][0] {
                Addr(self_.wrapping_add(0x708)).sw(c, value);
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 0x101;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// cFEStateNetAccountMem state stepping.

/// The account-memory hand-off shared by the five `UpdateState..` wrappers:
/// the state machine's mode is `*(*(gp_3+0x2e0))`; mode `from_a` advances it to
/// `to_a` (returning to `ret_a`), else mode `from_b` advances it to `to_b`
/// (returning to `ret_b`). Nothing happens for any other mode.
pub(crate) fn net_account_mem_step(
    c: &mut Cpu,
    from_a: u64,
    to_a: u64,
    ret_a: u32,
    from_b: u64,
    to_b: u64,
    ret_b: u32,
) -> Flow {
    c.cycles += 7;
    let frame = Frame::open(c, 0x10);
    c.g[V1][0] = Addr(gp_3(c, 0x2e0)).lw(c);
    frame.save_ra(c, 0);
    c.g[V1][0] = Addr(c.g[V1][0] as u32).lw(c);
    let mode = c.g[V1][0];
    c.g[V0][0] = mode ^ from_a;
    let first = c.g[V0][0] == 0;
    c.g[V0][0] = mode ^ from_b;
    if first {
        c.cycles += 4;
        c.g[A1][0] = to_a;
        callee::NET_ACCOUNT_MEM_SET_STATE.call(c, ret_a)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        return Ok(());
    }
    let second = c.g[V0][0] == 0;
    c.cycles += 2;
    frame.restore_ra(c, 0);
    if second {
        c.cycles += 3;
        c.g[A1][0] = to_b;
        callee::NET_ACCOUNT_MEM_SET_STATE.call(c, ret_b)?;
        frame.restore_ra(c, 0);
    }
    c.cycles += 2;
    frame.close(c);
    Ok(())
}


/// `void cFEStateNetAccountMem_UpdateState08(self)` (0x1b78f0): mode 8 →
/// state 0xa, mode 9 → state 9.
pub fn c_fe_state_net_account_mem_update_state08(c: &mut Cpu) {
    guest(c, |c| {
        net_account_mem_step(c, 8, 0xa, 0x001b_7914, 9, 9, 0x001b_792c)
    })
}


/// `void cFEStateNetAccountMem_UpdateState0E(self)` (0x1b7a08): mode 0x15 →
/// state 0x10, mode 0x16 → state 0xf.
pub fn c_fe_state_net_account_mem_update_state0e(c: &mut Cpu) {
    guest(c, |c| {
        net_account_mem_step(c, 0x15, 0x10, 0x001b_7a2c, 0x16, 0xf, 0x001b_7a44)
    })
}


/// `void cFEStateNetAccountMem_UpdateState10(self)` (0x1b7a50): mode 0xc →
/// state 0x12, mode 0xd → state 0x11.
pub fn c_fe_state_net_account_mem_update_state10(c: &mut Cpu) {
    guest(c, |c| {
        net_account_mem_step(c, 0xc, 0x12, 0x001b_7a74, 0xd, 0x11, 0x001b_7a8c)
    })
}


/// `void cFEStateNetAccountMem_UpdateState1A(self)` (0x1b7c28): mode 0x10 →
/// state 0x1b, mode 0x11 → state 0x1c.
pub fn c_fe_state_net_account_mem_update_state1a(c: &mut Cpu) {
    guest(c, |c| {
        net_account_mem_step(c, 0x10, 0x1b, 0x001b_7c4c, 0x11, 0x1c, 0x001b_7c64)
    })
}


/// `void cFEStateNetAccountMem_UpdateState1B(self)` (0x1b7c70): mode 0xc →
/// state 0x1d, mode 0xd → state 0x1c.
pub fn c_fe_state_net_account_mem_update_state1b(c: &mut Cpu) {
    guest(c, |c| {
        net_account_mem_step(c, 0xc, 0x1d, 0x001b_7c94, 0xd, 0x1c, 0x001b_7cac)
    })
}


// ---------------------------------------------------------------------------
// cFEStateLogin / cFEStateOnlineAgreement constructors.

/// A `NetFEState`-derived constructor: run `NetFEState_ctor`, install the
/// derived vtable at `+8` and the class id at `+0xc`, and clear two fields.
/// `zero_a`/`zero_b` are the fields the derived class clears.
pub(crate) fn net_fe_state_ctor_2(
    c: &mut Cpu,
    class_id: u32,
    vtable: u32,
    zero_a: u32,
    zero_b: u32,
    ret: u32,
) -> Flow {
    c.cycles += 18;
    let frame = Frame::open(c, 0x20);
    c.g[A2][0] = 0;
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    callee::NET_FE_STATE_CTOR_4.call(c, ret)?;
    let self_ = abi::r32(c, S0);
    c.g[V0][0] = sx(class_id);
    c.g[V1][0] = 0x47_0000;
    Addr(self_.wrapping_add(0xc)).sw(c, c.g[V0][0]);
    c.g[V1][0] = sx(vtable);
    Addr(self_.wrapping_add(8)).sw(c, c.g[V1][0]);
    c.g[V0][0] = c.g[S0][0];
    Addr(self_.wrapping_add(zero_a)).sw(c, 0);
    Addr(self_.wrapping_add(zero_b)).sw(c, 0);
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}


/// `cFEStateLogin* cFEStateLogin_ctor(self, arg)` (0x1bee30): the login
/// state's constructor (class id 0x33, vtable 0x468388; clears `+0x6d0` and
/// `+0x6a8`).
pub fn c_fe_state_login_ctor(c: &mut Cpu) {
    guest(c, |c| {
        net_fe_state_ctor_2(c, 0x33, 0x0046_8388, 0x6d0, 0x6a8, 0x001b_ee48)
    })
}


/// `cFEStateOnlineAgreement* cFEStateOnlineAgreement_ctor(self, arg)`
/// (0x1c0418): the online-agreement state's constructor (class id 0x43,
/// vtable 0x46ac38; clears `+0x6d0` and `+0x6d8`).
pub fn c_fe_state_online_agreement_ctor(c: &mut Cpu) {
    guest(c, |c| {
        net_fe_state_ctor_2(c, 0x43, 0x0046_ac38, 0x6d0, 0x6d8, 0x001c_0430)
    })
}


/// `void cFEStateOnlineAgreement_OnWidgetCreated(self, item)` (0x1c0ce0):
/// if the item's hash (`+0x38`) is the hash of the agreement text the item
/// gets the "needs update" flag (`+0x14 |= 1`).
pub fn c_fe_state_online_agreement_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x46_0000;
        frame.save(c, S1, 0x10);
        c.g[A0][0] = sx(0x46_0000u32.wrapping_add(0x19b8));
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        let item = abi::r32(c, S1);
        c.g[S0][0] = Addr(item.wrapping_add(0x38)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001c_0d04)?;
        let differs = c.g[S0][0] != c.g[V0][0];
        frame.restore(c, S0, 0x20);
        if !differs {
            c.cycles += 3;
            c.g[V0][0] = Addr(item.wrapping_add(0x14)).lw(c);
            c.g[V0][0] |= 1;
            Addr(item.wrapping_add(0x14)).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateOnlineCharacterSelect* cFEStateOnlineCharacterSelect_ctor(this,
/// a1)` (0x1b0f60): construct the NetFE base (child-event handler in `a2 =
/// 0`), install the class vtable and id 0x41, and clear the selection state.
pub fn c_fe_state_online_character_select_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        NET_FE_STATE_CTOR_5.call(c, 0x001b_0f78)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[A1][0] = 0x1;
        c.g[V1][0] = sx(0x0047_0000u32.wrapping_sub(0x73f0)); // &D_00468C10
        c.g[A0][0] = 0x41;
        let this = c.g[S0][0] as u32;
        Addr(this).off(8).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(this).off(0x6dc).sw(c, c.g[A1][0]);
        Addr(this).off(0xc).sw(c, c.g[A0][0]);
        Addr(this).off(0x6d8).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateNetCreateRoomPopup* cFEStateNetCreateRoomPopup_ctor(this, a1)`
/// (0x1be7d8): construct the NetFE base, install the class vtable and id 0x47,
/// and copy the current player count into the three popup fields.
pub fn c_fe_state_net_create_room_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        NET_FE_STATE_CTOR_5.call(c, 0x001b_e7ec)?;
        c.g[V0][0] = 0x0047_0000;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xda0)).lw(c);
        c.g[V0][0] = sx(0x0047_0000u32.wrapping_sub(0x5918)); // &D_0046A6E8
        c.g[A0][0] = 0x47;
        let this = c.g[S0][0] as u32;
        Addr(this).off(8).sw(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(this).off(0x6e4).sw(c, c.g[V1][0]);
        Addr(this).off(0xc).sw(c, c.g[A0][0]);
        Addr(this).off(0x6dc).sw(c, c.g[V1][0]);
        Addr(this).off(0x6e0).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int cFEStateNetMessenger_HandleInput(cFEStateNetMessenger* s, void* menu)`
/// (0x1a4170): when the menu's `+8` vtable slot 0xd8 reports that it handled
/// the input, push the messenger's selection (+0x6f8) into the player menu and
/// return 1; otherwise return 0. 11 (+3, +2) + 4 cycles plus the callees.
pub fn cfe_state_net_messenger_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        let arg = r32(c, A1);
        let vt = c.lw(p(arg, 8));
        c.g[V1][0] = vt;
        c.g[V0][0] = c.lw(p(vt as u32, 0xdc));
        let m = VSlot::read(c, vt, 0xd8);
        c.g[A0][0] = m.this(sx(arg));
        call_ptr(c, m.func as u32, 0x001a_4194)?;
        let handled = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S0][0];
        let mut result = 0u64;
        if handled {
            c.cycles += 3;
            c.g[A1][0] = c.lw(p(self_, 0x6f8));
            callee::NET_MESSENGER_SET_PLAYER_MENU_SELECTION.call(c, 0x001a_41ac)?;
            c.g[V0][0] = 1;
            result = 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        c.g[V0][0] = result;
        Ok(())
    })
}


/// `void cFEUseExistingAccount_dtor(cFEUseExistingAccount* s, int flags)`
/// (0x1ac068): install the vtable at +8, destruct the account-name string at
/// +0x6dc and run the base destructor. 20 cycles plus the callees.
pub fn cfe_use_existing_account_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(0x0046_9068);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.sw(p(self_, 8), c.g[V0][0]);
        c.g[A0][0] = sx(p(self_, 0x6dc));
        c.g[A1][0] = 2;
        call_ptr(c, callee::CBXSTRING_CTOR, 0x001a_c098)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::NET_FE_STATE_DTOR.call(c, 0x001a_c0a4)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateCreateStep1_SetAccountNameText(cFEStateCreateStep1* s)`
/// (0x1b95c0): look the account-name text object up by hash in the screen at
/// +0x40 and set it to the string held at `s+0x6d0`. 13 (+4, +3) + 3 cycles
/// plus the callees.
pub fn cfe_state_create_step1_set_account_name_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x0046_2b88);
        call_ptr(c, callee::GET_HASH_VALUE32, 0x001b_95dc)?;
        c.g[A0][0] = c.lw(p(self_, 0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME, 0x001b_95e8)?;
        c.g[A0][0] = c.g[V0][0];
        if c.g[A0][0] == 0 {
            frame.restore(c, 16, 0x10);
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw(p(self_, 0x6d0));
            c.g[A1][0] = c.lw(r32(c, V0));
            call_ptr(c, callee::UI_TEXT_SET_ASCII_STRING, 0x001b_9600)?;
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int cFEStateStats_v31(cFEStateStats* s, int msg)` (0x1c4cd8): message
/// 0xe1 refreshes the results, selects the first entry of the table at +0x6dc
/// and returns 0; every other message goes to the net front-end handler.
/// 6 (+7 or +4) + 3 cycles plus the callees.
pub fn cfe_state_stats_v31(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xe1;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let msg = c.g[A1][0];
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        if msg == c.g[V0][0] {
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            callee::FE_STATE_STATS_REFRESH_RESULTS_2.call(c, 0x001c_4d08)?;
            c.g[A0][0] = c.lw(p(self_, 0x6dc));
            c.g[A1][0] = 0;
            call_ptr(c, callee::UI_MENU_SET_SELECTED_BY_INDEX, 0x001c_4d14)?;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 4;
            callee::NET_FE_STATE_HANDLE_NET_EVENT.call(c, 0x001c_4cf8)?;
        }
        c.cycles += 3;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void NetFEState_OnActivate(NetFEState* self, int arg)` (0x1a87d0): run
/// the state's activation hook and mark it active.
pub fn net_fe_state_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = Addr(p(this, 8)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x100).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x104).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x001a_87fc)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_e4c0, 0x001a_8808)?;
        c.g[V0][0] = 0x1;
        frame.restore(c, S1, 0x10);
        Addr(p(this, 0x48)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void* cFEUseExistingAccount_ctor(cFEUseExistingAccount* self, int a1)`
/// (0x1ac010): construct the account screen's net state and install its
/// fields.
pub fn c_fe_use_existing_account_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::NET_FE_STATE_CTOR_6.call(c, 0x001a_c028)?;
        c.g[V0][0] = 0x46_9068; // D_00469068
        c.g[A0][0] = Addr(gp_4(c, 0xda0)).lw(c);
        c.g[V1][0] = 0x35;
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(p(this, 0x6dc)).sw(c, c.g[A0][0]);
        Addr(p(this, 0xc)).sw(c, c.g[V1][0]);
        Addr(p(this, 0x6d0)).sw(c, 0);
        Addr(p(this, 0x6e0)).sw(c, 0);
        Addr(p(this, 0x6a8)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Front end.

/// `void cFEStateOnlineCharacterSelect_OnChildEvent(self, child, event)`
/// (0x1b1ec0): dispatch the character-select screen's child events: 0x838 is
/// the quit popup, 0x835 empties the child, anything else forwards.
pub fn c_fe_state_online_character_select_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x835;
        frame.save_ra(c, 0);
        let child = r32(c, A1);
        c.g[V1][0] = Addr(p(child, 0x18)).lw(c);
        let is_empty = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x838;
        if is_empty {
            c.cycles += 4;
            callee::EMPTY_CHILD_HANDLER.call(c, 0x001b_1ef8)?;
        } else {
            c.cycles += 2;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 4;
                callee::ON_QUIT_POPUP_EVENT.call(c, 0x001b_1ee8)?;
            } else {
                c.cycles += 3;
                callee::NET_FE_STATE_ON_CHILD_EVENT_6.call(c, 0x001b_1f08)?;
            }
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// The chat room's player-quality poll period: 10000 frames (166.7 s at
/// 60 Hz). The divisor is applied to the VSync counter `cAppMan+0x18`, so at
/// 120 Hz it is 20000 VSyncs.
const CHAT_ROOM_POLL_FRAMES: u32 = 10000;


/// `void cFEStateChatRoom_Update(self)` (0x1b2320): every 10000th VSync
/// update the player quality, then run the net state's update.
pub fn c_fe_state_chat_room_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = tick::ticks_from_frames60(c, CHAT_ROOM_POLL_FRAMES) as u64;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = lw(c, gp_5(c, 0x2a74));
        c.cycles += 5;
        c.g[V0][0] = Addr(r32(c, A1)).off(0x18).lw(c);
        c.div(0, V0, V1);
        c.g[A0][0] = c.hi[0];
        if c.g[A0][0] == 0 {
            c.cycles += 2;
            callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY_2.call(c, 0x001b_235c)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::NET_FE_STATE_UPDATE_2.call(c, 0x001b_2364)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateNetAccountMem_dtor(self, flags)` (0x1b7030): destroy the
/// shared login manager, install the net-state base vtable and run its dtor.
pub fn c_fe_state_net_account_mem_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x47_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7830));
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x001b_7058);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0026_58b8, 0x001b_7058)?;
        c.g[V0][0] = 0x47_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4908));
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001b_7070);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        callee::NET_FE_STATE_DTOR_2.call(c, 0x001b_7070)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void* cFEStateCreateDuplicate_ctor(self, a1, a2)` (0x1bbd08): run the net
/// state ctor, install the create-duplicate class and store the arguments.
pub fn c_fe_state_create_duplicate_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x001b_bd28);
        c.g[A2][0] = 0;
        callee::NET_FE_STATE_CTOR_7.call(c, 0x001b_bd28)?;
        c.g[V1][0] = 0x47_0000;
        c.g[A0][0] = 0x4f;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7b60));
        Addr(p(r32(c, S0), 0x6d0)).sw(c, c.g[S1][0]);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[A0][0]);
        Addr(p(r32(c, S0), 0x6a8)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateNetQuickMessages_TransitionIn(self, arg)` (0x1c23c0): call
/// the state's transition hook, then refresh all messages when requested.
pub fn c_fe_state_net_quick_messages_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S1)).off(8).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x100).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x104).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x001c_23ec);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x001c_23ec)?;
        let no_refresh = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !no_refresh {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001c_23fc);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x001c_2418, 0x001c_23fc)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 5;
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateStats_TransitionIn(self, arg)` (0x1c32b8): call the state's
/// transition hook, then select the first category when requested.
pub fn c_fe_state_stats_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S1)).off(8).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x100).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x104).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x001c_32e4);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x001c_32e4)?;
        c.g[A0][0] = c.g[S1][0];
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[RA][0] = sx(0x001c_32f4);
            c.g[A1][0] = 0;
            callee::STATS_SELECT_CATEGORY.call(c, 0x001c_32f4)?;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateNetAccountMem_UpdateState02(cFEStateNetAccountMem* s)`
/// (0x1b77c8): leave the sub-state machine of the "account memory" screen:
/// state 4 is accepted directly, state 5 depending on `+0x6d4`, anything else
/// is ignored.
pub fn cfe_state_net_account_mem_update_state02(c: &mut Cpu) {
    c.cycles += 7;
    let frame = Frame::open(c, 0x10);
    c.g[V1][0] = Addr(gp_6(c, 0x2e0)).lw(c);
    frame.save_ra(c, 0);
    c.g[V1][0] = Addr(p_3(c, V1, 0)).lw(c);
    let mode = abi::r32(c, V1);
    c.g[V0][0] = c.g[V1][0] ^ 0x4;
    let is4 = c.g[V0][0] == 0;
    c.g[V0][0] = c.g[V1][0] ^ 0x5;
    // 0: nothing, 1: state 4, 2: state 3.
    let mut which = 0u8;
    if is4 {
        which = 1;
    } else {
        c.cycles += 2;
        let not5 = (mode ^ 5) != 0;
        frame.restore_ra(c, 0);
        if not5 {
            // Return without touching the state.
        } else {
            c.cycles += 4;
            c.g[V1][0] = Addr(abi::arg(c, 0)).off(0x6d4).lw(c);
            let field = abi::r32(c, V1);
            c.g[V0][0] = 1;
            which = if field != 1 { 2 } else { 1 };
        }
    }
    if which == 1 {
        c.cycles += 4;
        c.g[RA][0] = sx(0x001b_7804);
        c.g[A1][0] = 4;
        if callee::CFE_STATE_NET_ACCOUNT_MEM_SET_STATE
            .call(c, 0x001b_7804)
            .is_err()
        {
            return;
        }
        frame.restore_ra(c, 0);
    } else if which == 2 {
        c.cycles += 3;
        c.g[RA][0] = sx(0x001b_7814);
        c.g[A1][0] = 3;
        if callee::CFE_STATE_NET_ACCOUNT_MEM_SET_STATE
            .call(c, 0x001b_7814)
            .is_err()
        {
            return;
        }
        frame.restore_ra(c, 0);
    }
    c.cycles += 2;
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `void cFEStateLogin_dtor(cFEStateLogin* s, int flags)` (0x1bee78): set the
/// vtable, free the buffer at `+0x6d0`, then run the NetFEState destructor.
pub fn cfe_state_login_dtor(c: &mut Cpu) {
    c.cycles += 11;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(0x0046_8388);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.sw(p_3(c, S0, 8), c.g[V0][0]);
    c.g[A0][0] = Addr(p_3(c, S0, 0x6d0)).lw(c);
    let null = c.g[A0][0] == 0;
    c.g[S1][0] = c.g[A1][0];
    if !null {
        c.cycles += 3;
        if call_ptr(c, table::EA_MEM_FREE, 0x001b_eeac).is_err() {
            return;
        }
        c.sw(p_3(c, S0, 0x6d0), 0);
    }
    c.cycles += 8;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    if callee::NET_FE_STATE_DTOR_3.call(c, 0x001b_eebc).is_err() {
        return;
    }
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `void cFEStateOnlineAgreement_dtor(cFEStateOnlineAgreement* s, int flags)`
/// (0x1c0a28): the same shape as the login-state destructor.
pub fn cfe_state_online_agreement_dtor(c: &mut Cpu) {
    c.cycles += 11;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(0x0046_ac38);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.sw(p_3(c, S0, 8), c.g[V0][0]);
    c.g[A0][0] = Addr(p_3(c, S0, 0x6d0)).lw(c);
    let null = c.g[A0][0] == 0;
    c.g[S1][0] = c.g[A1][0];
    if !null {
        c.cycles += 3;
        if call_ptr(c, table::EA_MEM_FREE, 0x001c_0a5c).is_err() {
            return;
        }
        c.sw(p_3(c, S0, 0x6d0), 0);
    }
    c.cycles += 8;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    if callee::NET_FE_STATE_DTOR_3.call(c, 0x001c_0a6c).is_err() {
        return;
    }
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `void cFEStateNetNews_OnWidgetCreated(cFEStateNetNews* s, UIWidget* w)`
/// (0x1c5310): when the created widget is the news list, enable its second
/// behaviour.
pub fn cfe_state_net_news_on_widget_created(c: &mut Cpu) {
    c.cycles += 11;
    let frame = Frame::open(c, 0x30);
    c.g[A0][0] = sx(0x0046_5f48);
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A1][0];
    frame.save(c, S0, 0x20);
    c.g[S0][0] = Addr(p_3(c, S1, 0x38)).lw(c);
    if call_ptr(c, table::STR_HASH, 0x001c_5334).is_err() {
        return;
    }
    let same = c.g[S0][0] == c.g[V0][0];
    frame.restore(c, S0, 0x20);
    if same {
        c.cycles += 7;
        c.g[V0][0] = Addr(p_3(c, S1, 8)).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = Addr(p_3(c, V0, 0x38)).lh(c);
        c.g[V1][0] = Addr(p_3(c, V0, 0x3c)).lw(c);
        let target = abi::r32(c, V1);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        if call_ptr(c, target, 0x001c_5354).is_err() {
            return;
        }
        frame.restore(c, S0, 0x20);
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `int FEAsyncManager_FindFreeSlot(FEAsyncManager* m)` (0x1a3bf0): the first
/// of the five async slots whose state word is 0, or -1.
pub fn fe_async_manager_find_free_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A0][0] = c.g[S1][0];
        loop {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            callee::FE_ASYNC_MANAGER_GET_STATE.call(c, 0x001a_3c18)?;
            if c.g[V0][0] != 0 {
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[S0][0];
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 5i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S1][0];
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001a_3c10);
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


// ---------------------------------------------------------------------------
// Net FE popups and child-event handlers.

/// `void NetFEState_ShowBuddyActionPopup(NetFEState* self, int a1, int a2)`
/// (0x1ab370): mark the buddy-action popup as shown and set it up.
pub fn net_fe_state_show_buddy_action_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        callee::NET_FE_STATE_GET_POPUP.call(c, 0x001a_b394)?;
        c.g[V1][0] = 1;
        c.g[A0][0] = c.g[S0][0];
        sw_2(c, p_4(c.g[S0][0], 0x6c4), c.g[V1][0]);
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[T0][0] = 0;
        callee::NET_FE_STATE_SETUP_BUDDY_ACTION_POPUP.call(c, 0x001a_b3b4)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateChallengeReceivedPopup_ctor` (0x1afe08): the net-FE state base
/// plus the challenge-received popup's vtable, name string, and the global
/// "popup shown" word cleared.
pub fn cfe_state_challenge_received_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A3][0];
        callee::NET_FE_STATE_CTOR_8.call(c, 0x001a_fe24)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[A1][0] = c.g[S1][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x72e0));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6d0));
        sw_2(c, p_4(c.g[S0][0], 8), c.g[V1][0]);
        call_ptr(c, 0x0031_82d8, 0x001a_fe3c)?;
        c.g[V1][0] = 0x40;
        sw_2(c, p_4(c.g[S0][0], 0x6d4), 0);
        sw_2(c, p_4(c.g[S0][0], 0xc), c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        sw_2(c, gp(c, 0x1d98), 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateNetConfig_OnInfoPopupClosed(NetFEState* self, void* ev,
/// int arg)` (0x1b5cf8): on the info-popup event (0x16) also close the popup
/// object's info, then pass the event to the base handler.
pub fn cfe_state_net_config_on_info_popup_closed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x16;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let info = c.g[A2][0] == c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if info {
            c.cycles += 11;
            c.g[A2][0] = 0x16;
            callee::NET_FE_STATE_ON_CHILD_EVENT_7.call(c, 0x001b_5d18)?;
            c.g[A2][0] = lw_2(c, p_4(c.g[S0][0], 0x6ec));
            c.g[A1][0] = 0;
            c.g[V0][0] = lw_2(c, p_4(c.g[A2][0], 8));
            c.g[A0][0] = lh(c, p_4(c.g[V0][0], 0x40));
            c.g[V1][0] = lw_2(c, p_4(c.g[V0][0], 0x44));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001b_5d34)?;
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::NET_FE_STATE_ON_CHILD_EVENT_7.call(c, 0x001b_5d44)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// Dispatch shape shared by the four net-FE child-event handlers below:
/// compare the event id at `ev+0x18` with two ids and call the matching
/// handler (or the base handler), each returning to its own address.
macro_rules! child_event_switch {
    ($fname:ident, $doc:expr, $ev:expr, $a:expr, $b:expr, $ra:expr, $rb:expr,
     $base:expr, $fa:expr, $fb:expr) => {
        #[doc = $doc]
        pub fn $fname(c: &mut Cpu) {
            guest(c, |c| {
                c.cycles += 6;
                let frame = Frame::open(c, 0x10);
                c.g[V0][0] = $a;
                frame.save_ra(c, 0);
                c.g[V1][0] = lw_2(c, p_4(c.g[$ev][0], 0x18));
                let first = c.g[V1][0] == c.g[V0][0];
                let second;
                if first {
                    second = false;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = $b;
                    second = c.g[V1][0] == c.g[V0][0];
                    if !second {
                        c.cycles += 2;
                    }
                }
                if first {
                    c.cycles += 4;
                    $fa.call(c, $ra)?;
                } else if second {
                    c.cycles += 4;
                    $fb.call(c, $rb)?;
                } else {
                    c.cycles += 3;
                    callee::NET_FE_STATE_ON_CHILD_EVENT_7.call(c, $base)?;
                }
                frame.restore_ra(c, 0);
                c.cycles += 2;
                frame.close(c);
                Ok(())
            })
        }
    };
}
pub(crate) use child_event_switch;


child_event_switch!(
    cfe_state_create_step1_on_child_event,
    "`void cFEStateCreateStep1_OnChildEvent(NetFEState* self, void* ev, int arg)` \
     (0x1b9428): 0x81e -> save-account answer, 0x82c -> edit-done popup, else \
     the base handler.",
    A1,
    0x81e,
    0x82c,
    0x001b_9458,
    0x001b_9468,
    0x001b_9478,
    callee::CFE_STATE_CREATE_STEP1_ON_SAVE_ACCOUNT_ANSWER,
    callee::CFE_STATE_CREATE_STEP1_ON_EDIT_DONE_POPUP_EVENT
);


child_event_switch!(
    cfe_state_lobby_menu_on_child_event,
    "`void cFEStateLobbyMenu_OnChildEvent(NetFEState* self, void* ev, int arg)` \
     (0x1bdde8): 0x823 -> create-room popup, 0x82a -> room password prompt, \
     else the base handler.",
    A1,
    0x823,
    0x82a,
    0x001b_de18,
    0x001b_de28,
    0x001b_de38,
    callee::CFE_STATE_LOBBY_MENU_ON_CREATE_ROOM_POPUP_EVENT,
    callee::CFE_STATE_LOBBY_MENU_PROMPT_ROOM_PASSWORD
);


child_event_switch!(
    cfe_state_online_main_menu_on_child_event,
    "`void cFEStateOnlineMainMenu_OnChildEvent(NetFEState* self, void* ev, int arg)` \
     (0x1c02f8): 0x82d -> leave popup, 0x846 -> disconnect popup, else the \
     base handler.",
    A1,
    0x82d,
    0x846,
    0x001c_0328,
    0x001c_0338,
    0x001c_0348,
    callee::CFE_STATE_ONLINE_MAIN_MENU_ON_LEAVE_POPUP_EVENT,
    callee::CFE_STATE_ONLINE_MAIN_MENU_ON_DISCONNECT_POPUP_EVENT
);


child_event_switch!(
    cfe_state_net_quick_messages_on_child_event,
    "`void cFEStateNetQuickMessages_OnChildEvent(NetFEState* self, void* ev, int arg)` \
     (0x1c2b48): 0x83a -> save popup, 0x83b -> reset popup, else the base \
     handler.",
    A1,
    0x83a,
    0x83b,
    0x001c_2b78,
    0x001c_2b88,
    0x001c_2b98,
    callee::CFE_STATE_NET_QUICK_MESSAGES_ON_SAVE_POPUP_EVENT,
    callee::CFE_STATE_NET_QUICK_MESSAGES_ON_RESET_POPUP_EVENT
);


/// `void cFEStateLobbyMenu_HideRoomTabs(cFEStateLobbyMenu* self)` (0x1bdfd8):
/// hide the four room tabs at `self+0x730` through their vtable slot 0x48.
pub fn cfe_state_lobby_menu_hide_room_tabs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x730));
        frame.save_ra(c, 0);
        c.g[S1][0] = 3;
        c.g[A2][0] = lw_2(c, p_4(c.g[S0][0], 0));
        loop {
            c.cycles += 10;
            c.g[A1][0] = 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            c.g[V0][0] = lw_2(c, p_4(c.g[A2][0], 8));
            c.g[A0][0] = lh(c, p_4(c.g[V0][0], 0x48));
            c.g[V1][0] = lw_2(c, p_4(c.g[V0][0], 0x4c));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001b_e018)?;
            if (c.g[S1][0] as i64) < 0 {
                break;
            }
            c.g[A2][0] = lw_2(c, p_4(c.g[S0][0], 0));
            if event_due(c) {
                deliver_event_here(c, 0x001b_dff8);
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


// ---------------------------------------------------------------------------
// Shared bodies of the repeated handler shapes (the generated code is
// identical for each group; only the address differs).

/// The four `cFEState*_OnWidgetNotify` handlers whose action key is tested
/// against 10 and 8: below 8 (and 10 and up) give 0x101, 8 and 9 give 0x100.
pub(crate) fn state_notify_low_action(c: &mut Cpu) {
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


// ---------------------------------------------------------------------------
// Small register helpers.

/// `base + off` as a 32-bit address.
#[inline(always)]
pub(crate) fn p_5(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}


/// `lw_3 a` through the CPU's load path.
#[inline(always)]
pub(crate) fn lw_3(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lw(c)
}


// ---------------------------------------------------------------------------
// Front-end state: net messenger.

/// The messenger's player-list refresh period: one second, i.e. 60 frames of
/// the VSync counter (`cAppMan+0x18`, bumped once per VSync). The update
/// compares `elapsed - 1 < 59`, i.e. refreshes once `elapsed` reaches the
/// period.
const NET_MESSENGER_REFRESH_FRAMES: u32 = 60;


/// `void cFEStateNetMessenger_Update(self)` (0x1a3fe0): refresh the player
/// list when a second has passed since the last refresh, then poll the net
/// state.
///
/// `now` and the remembered `self+0x6dc` are both counts of the VSync counter,
/// so the threshold is the seconds' worth of VSyncs, not 60 ticks: at 120 Hz
/// the list refreshes every 120 VSyncs.
pub fn fe_state_net_messenger_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = lw_3(c, p_5(c.g[GP][0], 0x2a74));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_3(c, p_5(c.g[S0][0], 0x6dc));
        c.g[V1][0] = lw_3(c, p_5(c.g[V1][0], 0x18));
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        let period = tick::ticks_from_frames60(c, NET_MESSENGER_REFRESH_FRAMES) as u64;
        c.g[V0][0] = (c.g[V0][0] < period - 1) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            Addr(p_5(c.g[S0][0], 0x6dc)).sw(c, c.g[V1][0]);
            call_ptr(c, 0x001a_6978, 0x001a_4028)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001a_8770, 0x001a_4030)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 4;
            call_ptr(c, 0x001a_8770, 0x001a_4018)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void cFEStateNetMessenger_OnFindUserResult(self, a1, a2)` (0x1a4fb0):
/// show the find-user error for the "user not found" reply, otherwise hand
/// the event to the base state.
pub fn fe_state_net_messenger_on_find_user_result(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_3(c, p_5(c.g[A3][0], 0x1c));
        c.g[V0][0] &= 6;
        let no_flags = c.g[V0][0] == 0;
        c.g[A0][0] = 0x807;
        if !no_flags {
            c.cycles += 4;
            c.g[V0][0] = lw_3(c, p_5(c.g[A3][0], 0x6a0));
            c.g[V1][0] = lw_3(c, p_5(c.g[V0][0], 0x18));
            let not_found = c.g[V1][0] == c.g[A0][0];
            c.g[A0][0] = c.g[A3][0];
            if not_found {
                c.cycles += 4;
                c.g[A1][0] = c.g[A2][0];
                call_ptr(c, 0x001a_4e28, 0x001a_4fe4)?;
                frame.restore_ra(c, 0);
                c.cycles += 2;
                c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
                c.pc = c.g[RA][0] as u32;
                return Ok(());
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[A3][0];
            call_ptr(c, 0x001a_8e40, 0x001a_4ff4)?;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
            c.pc = c.g[RA][0] as u32;
            return Ok(());
        }
        c.cycles += 3;
        c.g[A0][0] = c.g[A3][0];
        call_ptr(c, 0x001a_8e40, 0x001a_5004)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        c.pc = c.g[RA][0] as u32;
        Ok(())
    });
}


/// `void cFEStateNetMessenger_SetPlayerMenuSelection(self, int idx)`
/// (0x1a6c40): select entry `idx` in the menu named by the messenger's hash.
pub fn fe_state_net_messenger_set_player_menu_selection(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_3(c, p_5(c.g[S0][0], 0x40));
        let no_screen = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_screen {
            c.cycles += 2;
            let negative = (c.g[S1][0] as i64) < 0;
            c.g[A0][0] = 0x0046_0000;
            if !negative {
                c.cycles += 7;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1590));
                call_ptr(c, 0x0031_7670, 0x001a_6c70)?;
                c.g[A0][0] = lw_3(c, p_5(c.g[S0][0], 0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001a_6c7c)?;
                let no_menu = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[V0][0];
                if !no_menu {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S1][0] & 0xff;
                    call_ptr(c, 0x0039_b960, 0x001a_6c8c)?;
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void NetFEState_Update(NetFEState* self)` (0x1a8770): two virtual hooks,
/// then the shared UI update and the net event pump.
pub fn net_fe_state_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_3(c, p_5(c.g[S0][0], 8));
        c.g[A0][0] = Addr(p_5(c.g[V0][0], 0xd8)).lh(c);
        c.g[V1][0] = lw_3(c, p_5(c.g[V0][0], 0xdc));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001a_8794)?;
        c.g[V0][0] = lw_3(c, p_5(c.g[S0][0], 8));
        c.g[A0][0] = Addr(p_5(c.g[V0][0], 0xe0)).lh(c);
        c.g[V1][0] = lw_3(c, p_5(c.g[V0][0], 0xe4));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001a_87a8)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_e510, 0x001a_87b0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x114;
        c.g[A2][0] = 0;
        call_ptr(c, 0x001a_8b88, 0x001a_87c0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void NetFEState_OpenKeyboardStr(self, a1, str, a3, a4, a5)` (0x1a9090):
/// create the popup, then set its static text and its initial text.
pub fn net_fe_state_open_keyboard_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = c.g[T0][0];
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[T0][0] = c.g[T2][0];
        c.g[A3][0] = c.g[T1][0];
        call_ptr(c, 0x001a_91b0, 0x001a_90c0)?;
        c.g[A0][0] = lw_3(c, p_5(c.g[S1][0], 0x69c));
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x001c_cf00, 0x001a_90cc)?;
        c.g[A0][0] = lw_3(c, p_5(c.g[S1][0], 0x69c));
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x001c_b418, 0x001a_90d8)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void NetFEState_OpenKeyboardTextId(self, a1, id, a3, a4, a5)` (0x1a9150):
/// the same popup setup with a static text id instead of a string.
pub fn net_fe_state_open_keyboard_text_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = c.g[T0][0];
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[T0][0] = c.g[T2][0];
        c.g[A3][0] = c.g[T1][0];
        call_ptr(c, 0x001a_91b0, 0x001a_9180)?;
        c.g[A0][0] = lw_3(c, p_5(c.g[S1][0], 0x69c));
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x001c_d020, 0x001a_918c)?;
        c.g[A0][0] = lw_3(c, p_5(c.g[S1][0], 0x69c));
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x001c_b418, 0x001a_9198)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void cFEStateNetConfig_StartCardScan(self)` (0x1b5268): reset the card
/// status widgets and open the "checking card" popup.
pub fn fe_state_net_config_start_card_scan(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 1;
        c.g[A0][0] = lw_3(c, p_5(c.g[S0][0], 0x6f4));
        call_ptr(c, 0x0039_b960, 0x001b_528c)?;
        Addr(p_5(c.g[GP][0], -0x1570i32 as u32)).sw(c, 0);
        c.g[A0][0] = lw_3(c, p_5(c.g[S0][0], 0x6e4));
        Addr(p_5(c.g[S0][0], 0x70c)).sw(c, c.g[S1][0]);
        call_ptr(c, 0x0039_a670, 0x001b_529c)?;
        c.g[A0][0] = lw_3(c, p_5(c.g[S0][0], 0x6e8));
        call_ptr(c, 0x0039_a670, 0x001b_52a4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x001b_47c0, 0x001b_52b0)?;
        Addr(p_5(c.g[S0][0], 0x708)).sw(c, c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `int cFEStateNetAccountMem_PopupIdToState(int id)` (0x1b8890): map an
/// account-memory popup id to the front-end state that owns it.
pub fn fe_state_net_account_mem_popup_id_to_state(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0xd;
    let thirteen = c.g[A1][0] == c.g[V0][0];
    c.g[V0][0] = ((c.g[A1][0] as i64) < 14i64) as u64;
    if thirteen {
        c.cycles += 2;
        c.g[V0][0] = 0xf;
        abi::ret(c);
        return;
    }
    let below14 = c.g[V0][0] != 0;
    c.cycles += 2;
    c.g[V0][0] = 7;
    if below14 {
        c.cycles += 2;
        let seven = c.g[A1][0] == c.g[V0][0];
        c.g[V0][0] = 0x1c;
        if seven {
            c.cycles += 2;
            c.g[V0][0] = 9;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = 0x15;
    let twentyone = c.g[A1][0] == c.g[V0][0];
    c.g[V0][0] = 0x19;
    if twentyone {
        c.cycles += 2;
        c.g[V0][0] = 0x17;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0x1c;
    c.cycles += 2;
    abi::ret(c);
}


/// `void cFEStateCreateStep1_SetEmailText(self)` (0x1b96e0): find the email
/// widget and push the current email string into it.
pub fn fe_state_create_step1_set_email_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4838));
        call_ptr(c, 0x0031_7670, 0x001b_96fc)?;
        c.g[A0][0] = lw_3(c, p_5(c.g[S0][0], 0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001b_9708)?;
        c.g[A1][0] = c.g[V0][0];
        if c.g[A1][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = lw_3(c, p_5(c.g[S0][0], 0x6d0));
            c.g[A2][0] = lw_3(c, p_5(c.g[V0][0], 0xc));
            c.g[V1][0] = lw_3(c, (c.g[A2][0] as u32).wrapping_sub(8));
            if c.g[V1][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x001b_9950, 0x001b_9730)?;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// The shared "init screen by hash" body of the front-end state/overlay
/// constructors: hash the name, look the screen up in the engine, store it at
/// `+0x40` and play its frame when it was created.
pub(crate) fn ui_init_screen(c: &mut Cpu, base: u32, tag: u32) -> Flow {
    c.cycles += 16;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save_ra(c, 0);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    c.g[A0][0] = sx(tag);
    c.g[S1][0] = Addr(p(c.g[S0][0] as u32, 0x10)).lw(c);
    call_ptr(c, 0x0031_7670, base + 0x24)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[A3][0] = 0;
    call_ptr(c, 0x0039_7cc0, base + 0x38)?;
    let screen = c.g[V0][0];
    Addr(p(c.g[S0][0] as u32, 0x40)).sw(c, screen);
    if screen != 0 {
        c.cycles += 4;
        c.g[A0][0] = screen;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        call_ptr(c, 0x0039_c870, base + 0x50)?;
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}


/// One of the two password-text setters: hash the label, find the UI text
/// object on the created screen and mask the string stored at `self + 0x5c`
/// into it.
pub(crate) fn set_password_text(c: &mut Cpu, base: u32, tag: u32, list_off: u32) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save_ra(c, 0);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    c.g[A0][0] = sx(tag);
    call_ptr(c, 0x0031_7670, base + 0x20)?;
    c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, base + 0x2c)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    if c.g[S1][0] != 0 {
        c.cycles += 6;
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x6d0)).lw(c);
        c.g[A1][0] = Addr(p(c.g[V0][0] as u32, list_off)).lw(c);
        call_ptr(c, 0x001a_98b8, base + 0x44)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = sx(p(c.g[S0][0] as u32, 0x5c));
        call_ptr(c, 0x003a_0c50, base + 0x50)?;
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}


/// `void FEScreenBlank_Init(void* self)` (0x1abfa8).
pub fn fe_screen_blank_init(c: &mut Cpu) {
    guest(c, |c| ui_init_screen(c, 0x001a_bfa8, 0x004a_1a78))
}


/// `void cFEStateSelectPersona_v33(void* self, void* a1, int a2)` (0x1ad9c8):
/// on the "create" child event ask the lobby manager for the persona list,
/// then forward the event to the net front-end state.
pub fn cfe_state_select_persona_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0xf;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        if c.g[A2][0] == c.g[V0][0] {
            c.cycles += 9;
            c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
            c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x6d4)).lw(c);
            call_ptr(c, 0x0025_d428, 0x001a_d9f4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 0xf;
            call_ptr(c, 0x001a_97b8, 0x001a_da04)?;
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x001a_97b8, 0x001a_da18)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateAuthDNASError_Init(void* self)` (0x1b6b50).
pub fn cfe_state_auth_dnas_error_init(c: &mut Cpu) {
    guest(c, |c| ui_init_screen(c, 0x001b_6b50, 0x0046_42b0))
}


/// `void cFEStateNetAccountMem_UpdateState16(void* self)` (0x1b7bc0):
/// 0x10 clears the account strings and moves to state 0x18; 0x11 moves to
/// 0x17.
pub fn cfe_state_net_account_mem_update_state16(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(gp(c, 0x2e0)).lw(c);
        c.g[V1][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[V0][0] = c.g[V1][0] ^ 0x10;
        c.g[S0][0] = c.g[A0][0];
        if c.g[V0][0] == 0 {
            c.cycles += 7;
            call_ptr(c, 0x001b_70a8, 0x001b_7be8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x18;
            call_ptr(c, 0x001b_70d0, 0x001b_7bf4)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] ^ 0x11;
            if c.g[V0][0] == 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x17;
                call_ptr(c, 0x001b_70d0, 0x001b_7c14)?;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateCreateStep1_SetPasswordText(void* self)` (0x1b9610).
pub fn cfe_state_create_step1_set_password_text(c: &mut Cpu) {
    guest(c, |c| set_password_text(c, 0x001b_9610, 0x0046_2ba0, 4))
}


/// `void cFEStateCreateStep1_SetConfirmPasswordText(void* self)` (0x1b9678).
pub fn cfe_state_create_step1_set_confirm_password_text(c: &mut Cpu) {
    guest(c, |c| set_password_text(c, 0x001b_9678, 0x0046_47f8, 8))
}


/// `void cFEStateLobbyMenu_OnCreateRoomPopupEvent(void* self, void* a1, int a2)`
/// (0x1bdcf8).
pub fn cfe_state_lobby_menu_on_create_room_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0xf;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        if c.g[A2][0] == c.g[V0][0] {
            c.cycles += 9;
            c.g[A2][0] = 0xf;
            call_ptr(c, 0x001a_97b8, 0x001b_dd20)?;
            c.g[A3][0] = Addr(p(c.g[S0][0] as u32, 0x6e4)).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x6dc)).lw(c);
            c.g[A2][0] = Addr(p(c.g[S0][0] as u32, 0x6e0)).lw(c);
            call_ptr(c, 0x001b_d610, 0x001b_dd34)?;
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x001a_97b8, 0x001b_dd48)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateNetCreateRoomPopup_Init(void* self)` (0x1be828).
pub fn cfe_state_net_create_room_popup_init(c: &mut Cpu) {
    guest(c, |c| ui_init_screen(c, 0x001b_e828, 0x0046_51e0))
}


/// A `gp_7`-relative global address.
#[inline(always)]
pub(crate) fn gp_7(c: &Cpu, off: i32) -> u32 {
    abi::r32(c, GP).wrapping_add(off as u32)
}


/// `lw` at `base + off`.
#[inline(always)]
pub(crate) fn lw_at_2(c: &mut Cpu, base: u32, off: u32) -> u64 {
    Addr(p(base, off)).lw(c)
}


/// `lw` at `base + off` as a 32-bit value.
#[inline(always)]
pub(crate) fn u32_at(c: &mut Cpu, base: u32, off: u32) -> u32 {
    lw_at_2(c, base, off) as u32
}


// ---------------------------------------------------------------------------
// Front end: net messenger creation and screen initialisation.

/// `cFEStateNetMessenger_ctor(this, engine, owner)` 0x1a3dc8: base
/// `NetFEState_ctor`, then install the derived vtable and clear the messenger
/// fields (`this + 0x6d0..0x6f8`), storing the shared empty object pointer.
pub fn c_fe_state_net_messenger_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        frame.save_ra(c, 0x10);
        c.g[S0][0] = this64;
        call_ptr(c, 0x001a_8500, 0x001a_3ddc)?;
        let empty = Addr(gp_7(c, 0xda0)).lw(c);
        c.g[V0][0] = sx(0x0046_7f28);
        c.g[V1][0] = empty;
        c.g[A0][0] = u64::MAX;
        Addr(p(this, 0x8)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0x46;
        Addr(p(this, 0x6e0)).sw(c, c.g[V1][0]);
        c.g[V0][0] = this64;
        Addr(p(this, 0x6f8)).sw(c, c.g[A0][0]);
        Addr(p(this, 0xc)).sw(c, c.g[A1][0]);
        Addr(p(this, 0x6d0)).sw(c, c.g[V1][0]);
        Addr(p(this, 0x6d4)).sw(c, c.g[V1][0]);
        for off in [0x6dc, 0x6e8, 0x6ec, 0x6f0, 0x6f4] {
            Addr(p(this, off)).sw(c, 0);
        }
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `NetFEState_OpenMessenger(state)` 0x1a9b58: allocate and construct the
/// net messenger, tag it, and add it to the state's messenger list.
pub fn net_fe_state_open_messenger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        c.g[A1][0] = 0x460000;
        frame.save(c, S0, 0x10);
        c.g[A3][0] = 0;
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(0x0046_1d40);
        c.g[A0][0] = 0x6fc;
        c.g[A2][0] = 0x100;
        call_ptr(c, 0x0031_7d70, 0x001a_9b80)?;
        c.g[A1][0] = lw_at_2(c, this, 0x10);
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = this as u64;
        call_ptr(c, 0x001a_3dc8, 0x001a_9b90)?;
        let msg = c.g[V0][0];
        c.g[V1][0] = 0x833;
        c.g[A1][0] = msg;
        Addr(p(msg as u32, 0x18)).sw(c, c.g[V1][0]);
        let list = lw_at_2(c, this, 0x40) as u32;
        c.g[V0][0] = list as u64;
        let vt = lw_at_2(c, list, 0xd0) as u32;
        c.g[V1][0] = vt as u64;
        c.g[A0][0] = lw_at_2(c, vt, 0x10);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
        call_ptr(c, 0x0039_f290, 0x001a_9bb0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateCreateStep1_RestoreAccountInfo(this)` 0x1ba218: copy the three
/// saved account strings back into the form object and restore its two
/// scalar fields.
pub fn c_fe_state_create_step1_restore_account_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = abi::arg(c, 0);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(this);
        c.g[A1][0] = sx(p(this, 0x6d4));
        c.g[A0][0] = lw_at_2(c, this, 0x6d0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
        call_ptr(c, 0x0031_8448, 0x001b_a238)?;
        c.g[A0][0] = lw_at_2(c, this, 0x6d0);
        c.g[A1][0] = sx(p(this, 0x6d8));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(8));
        call_ptr(c, 0x0031_8448, 0x001b_a248)?;
        c.g[A0][0] = lw_at_2(c, this, 0x6d0);
        c.g[A1][0] = sx(p(this, 0x6dc));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0xc));
        call_ptr(c, 0x0031_8448, 0x001b_a258)?;
        c.g[V0][0] = lw_at_2(c, this, 0x6e0);
        c.g[V1][0] = lw_at_2(c, this, 0x6d0);
        frame.restore_ra(c, 0);
        Addr(p(r32(c, V1), 0x38)).sw(c, c.g[V0][0]);
        c.g[A0][0] = lw_at_2(c, this, 0x6e4);
        c.g[V0][0] = lw_at_2(c, this, 0x6d0);
        frame.restore(c, S0, 0x10);
        Addr(p(r32(c, V0), 0x3c)).sw(c, c.g[A0][0]);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateNetQuickMessages_Init(this)` 0x1c2138: add the quick-messages
/// screen, play its first frame and clear the message list.
pub fn c_fe_state_net_quick_messages_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        frame.save(c, S1, 0x10);
        c.g[S1][0] = this64;
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx(0x0046_5b20);
        c.g[S0][0] = lw_at_2(c, this, 0x10);
        call_ptr(c, 0x0031_7670, 0x001c_215c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001c_2170)?;
        let screen = c.g[V0][0];
        Addr(p(this, 0x40)).sw(c, screen);
        if screen != 0 {
            c.cycles += 4;
            c.g[A0][0] = screen;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001c_2188)?;
        }
        c.cycles += 6;
        Addr(p(this, 0x6d0)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateAuthDNASError_OnWidgetAction(self, a1, msg)` 0x1b6e40: 5/6 pick the
/// error dialog's option 0xf, 7/8 do nothing, everything else forwards to the
/// net state's widget action.
pub fn c_fe_state_auth_dnas_error_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        let a1 = abi::arg64(c, 1);
        let msg = abi::arg64(c, 2);
        if msg < 5 {
            c.cycles += 3;
            c.g[A0][0] = this;
            c.g[A1][0] = a1;
            c.g[A2][0] = msg;
            call_ptr(c, 0x001a_8918, 0x001b_6e9c)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            if msg < 7 {
                let obj = u32_at(c, this as u32, 0x20);
                c.g[A3][0] = sx(obj);
                c.cycles += 9;
                c.g[A1][0] = this;
                c.g[A2][0] = 0xf;
                let vt = u32_at(c, obj, 8);
                c.g[V0][0] = sx(vt);
                c.g[A0][0] = Addr(p(vt, 0x88)).lh(c);
                c.g[V1][0] = lw_at_2(c, vt, 0x8c);
                let func = r32(c, V1);
                c.g[A0][0] = sx(obj.wrapping_add(r32(c, A0)));
                call_ptr(c, func, 0x001b_6e8c)?;
                frame.restore_ra(c, 0);
            } else if msg < 9 {
                c.cycles += 3;
                c.cycles += 2;
                c.g[V0][0] = 1;
                frame.restore_ra(c, 0);
            } else {
                c.cycles += 3;
                c.cycles += 3;
                c.g[A0][0] = this;
                c.g[A1][0] = a1;
                c.g[A2][0] = msg;
                call_ptr(c, 0x001a_8918, 0x001b_6e9c)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateCreateStep1_ValidateFields(self)` 0x1ba110: validate account name,
/// password and e-mail in that order, stopping at the first failure.
pub fn c_fe_state_create_step1_validate_fields(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        call_ptr(c, 0x001b_9d68, 0x001b_a128)?;
        c.g[S1][0] = 1;
        let name_ok = c.g[V0][0] != 0;
        c.g[V1][0] = 0;
        if name_ok {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001b_9fa0, 0x001b_a13c)?;
            c.g[V1][0] = 0;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[S1][0];
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[V1][0];
        let password_ok = c.g[V0][0] != 0;
        c.g[V1][0] = 0;
        if password_ok {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001b_9b18, 0x001b_a158)?;
            c.g[V1][0] = 0;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[S1][0];
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = c.g[V1][0];
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateCreateStep2_GetDaysInMonth(self, month, year)` 0x1bb9b0: days in
/// the month, 29 for a leap February.
pub fn c_fe_state_create_step2_get_days_in_month(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let month = abi::arg(c, 1);
        frame.save_ra(c, 0);
        c.g[V0][0] = 2;
        if month == 2 {
            c.cycles += 6;
            c.g[A1][0] = abi::arg64(c, 2);
            call_ptr(c, 0x001b_b968, 0x001b_b9fc)?;
            c.g[A0][0] = 0x1d;
            c.g[V1][0] = 0x1c;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[A0][0];
            }
            c.g[V0][0] = c.g[V1][0];
        } else {
            c.cycles += 13;
            let mut m = c.g[A1][0];
            c.g[V1][0] = ((m as i64) < 8) as u64;
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(1));
            if c.g[V1][0] == 0 {
                m = c.g[V0][0];
                c.g[A1][0] = m;
            }
            c.g[A0][0] = 0x1f;
            c.g[V1][0] = sx(r32(c, A1) >> 31);
            c.g[V0][0] = 0x1e;
            c.g[V1][0] = sx((m as u32).wrapping_add(r32(c, V1)));
            c.g[V1][0] = sx(((r32(c, V1) as i32) >> 1) as u32);
            c.g[V1][0] = sx(r32(c, V1) << 1);
            c.g[V1][0] = sx((m as u32).wrapping_sub(r32(c, V1)));
            c.g[V1][0] ^= 1;
            if c.g[V1][0] == 0 {
                c.g[V0][0] = c.g[A0][0];
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// A `gp_8`-relative global.
#[inline(always)]
pub(crate) fn gp_8(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}


// ---------------------------------------------------------------------------
// Front-end state helpers.

/// `cFEStateOnlineCharacterSelect::OnQuitPopupEvent(self, a1, kind)`
/// (0x1b1f18): a `0xf` child event sends the quit request to the setup
/// manager (when it exists) and resends the event as kind `0xf`.
pub fn c_fe_state_online_character_select_on_quit_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0xf;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        if c.g[A2][0] == c.g[V0][0] {
            c.cycles += 3;
            c.g[A0][0] = Addr(gp_8(c, 0x2dc)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[A1][0] = 0xf9;
                call_ptr(c, 0x0026_5768, 0x001b_1f4c)?;
                c.g[A0][0] = c.g[S0][0];
            } else {
                c.g[A0][0] = c.g[S0][0];
            }
            c.cycles += 5;
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 0xf;
            call_ptr(c, 0x001a_97b8, 0x001b_1f5c)?;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x001a_97b8, 0x001b_1f70)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateAuthDNASError* ctor(self, a1, a2, a3, a4, a5)` (0x1b6ab8): the
/// five-argument net-front-end state constructor with the fields the auth
/// state adds.
pub fn c_fe_state_auth_dnas_error_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[T1][0];
        call_ptr(c, 0x001a_8500, 0x001b_6ae4)?;
        c.g[V1][0] = 0x470000;
        c.g[A0][0] = 0x45;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7720));
        Addr(p(c.g[S0][0] as u32, 0x6d0)).sw(c, c.g[S1][0]);
        Addr(p(c.g[S0][0] as u32, 0x6d4)).sw(c, c.g[S2][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(p(c.g[S0][0] as u32, 0x6d8)).sw(c, c.g[S3][0]);
        Addr(p(c.g[S0][0] as u32, 0x8)).sw(c, c.g[V1][0]);
        Addr(p(c.g[S0][0] as u32, 0xc)).sw(c, c.g[A0][0]);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateCreateStep2::OnParentEmailEntered(self, a1, kind)` (0x1bb0e8):
/// on the `0xf` child event, hand the entered email to the account and
/// submit it.
pub fn c_fe_state_create_step2_on_parent_email_entered(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0xf;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        if c.g[A2][0] == c.g[V0][0] {
            c.cycles += 11;
            c.g[A2][0] = 0xf;
            call_ptr(c, 0x001a_97b8, 0x001b_b110)?;
            c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x6d0)).lw(c);
            c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 0x2b0)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x0031_84e8, 0x001b_b120)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x001b_a9f8, 0x001b_b12c)?;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x001a_97b8, 0x001b_b140)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// The shared body of the three create-step-2 selection setters: hash the
/// label, find the list box on the created screen, look the object's value
/// up in it and select that row.
pub(crate) fn step2_set_selection(c: &mut Cpu, entry: u32, tag: u32, value_off: u32) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S1, 0x20);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x30);
    frame.save_ra(c, 0x10);
    c.g[A0][0] = sx(tag);
    call_ptr(c, 0x0031_7670, entry + 0x20)?;
    c.g[A0][0] = Addr(p(c.g[S1][0] as u32, 0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, entry + 0x2c)?;
    c.g[S0][0] = c.g[V0][0];
    let missing = c.g[S0][0] == 0;
    c.g[A2][0] = c.g[SP][0];
    if !missing {
        c.cycles += 8;
        c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0x6d0)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        Addr(c.g[SP][0] as u32).sb(c, 0);
        c.g[A1][0] = Addr(p(c.g[V0][0] as u32, value_off)).lw(c);
        call_ptr(c, 0x0039_a768, entry + 0x4c)?;
        c.g[A1][0] = Addr(c.g[SP][0] as u32).lbu(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_a7a8, entry + 0x58)?;
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}


/// `cFEStateCreateStep2::SetMonthSelection(self)` (0x1bb2d0).
pub fn c_fe_state_create_step2_set_month_selection(c: &mut Cpu) {
    guest(c, |c| {
        step2_set_selection(c, 0x001b_b2d0, 0x004a_1bd8, 0x28)
    })
}


/// `cFEStateCreateStep2::SetDaySelection(self)` (0x1bb340).
pub fn c_fe_state_create_step2_set_day_selection(c: &mut Cpu) {
    guest(c, |c| {
        step2_set_selection(c, 0x001b_b340, 0x004a_1be0, 0x2c)
    })
}


/// `cFEStateCreateStep2::SetGenderSelection(self)` (0x1bb440).
pub fn c_fe_state_create_step2_set_gender_selection(c: &mut Cpu) {
    guest(c, |c| {
        step2_set_selection(c, 0x001b_b440, 0x0046_4a18, 0x34)
    })
}


/// The shared body of the `*_v32` presence pushers: make sure the buddy
/// manager exists, reconnect when it is disconnected, disconnect the lobby
/// when there is still no manager, then push the presence (kind `a2`).
pub(crate) fn presence_v32(c: &mut Cpu, entry: u32, kind: u64) -> Flow {
    c.cycles += 4;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = Addr(gp_8(c, 0x238)).lw(c);
    let have_manager = c.g[V0][0] != 0;
    frame.save_ra(c, 0);
    if !have_manager {
        c.cycles += 2;
        call_ptr(c, 0x0026_0f80, entry + 0x18)?;
    }
    c.cycles += 4;
    c.g[A0][0] = Addr(gp_8(c, 0x238)).lw(c);
    call_ptr(c, 0x0026_1460, entry + 0x20)?;
    let connected = c.g[V0][0] != 0;
    c.g[V0][0] = Addr(gp_8(c, 0x238)).lw(c);
    if !connected {
        c.cycles += 3;
        c.g[A0][0] = Addr(gp_8(c, 0x238)).lw(c);
        call_ptr(c, 0x0026_13c8, entry + 0x30)?;
        c.g[V0][0] = Addr(gp_8(c, 0x238)).lw(c);
    }
    c.cycles += 2;
    let have_manager = c.g[V0][0] != 0;
    c.g[A0][0] = Addr(gp_8(c, 0x238)).lw(c);
    if !have_manager {
        c.cycles += 3;
        c.g[A0][0] = Addr(gp_8(c, 0xc8)).lw(c);
        call_ptr(c, 0x0025_b800, entry + 0x44)?;
        c.g[A0][0] = Addr(gp_8(c, 0x238)).lw(c);
    }
    c.cycles += 9;
    c.g[A1][0] = 1;
    c.g[A2][0] = kind;
    c.g[A3][0] = 0;
    c.g[T0][0] = 0;
    c.g[T1][0] = 0;
    call_ptr(c, 0x0026_2768, entry + 0x60)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}


/// `cFEStateLobbyMenu_v32(self)` (0x1bcca8): push presence kind 2.
pub fn c_fe_state_lobby_menu_v32(c: &mut Cpu) {
    guest(c, |c| presence_v32(c, 0x001b_cca8, 2))
}


/// `cFEStateStats_v32(self)` (0x1c2e70): push presence kind 5.
pub fn c_fe_state_stats_v32(c: &mut Cpu) {
    guest(c, |c| presence_v32(c, 0x001c_2e70, 5))
}


/// `cFEStateNetNews_v32(self)` (0x1c5108): push presence kind 4.
pub fn c_fe_state_net_news_v32(c: &mut Cpu) {
    guest(c, |c| presence_v32(c, 0x001c_5108, 4))
}


/// `cFEStateLogin::Init(self)` (0x1beed0): create the state's screen by tag,
/// start its thread and initialise the net state.
pub fn c_fe_state_login_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x52b0));
        c.g[S0][0] = Addr(p(c.g[S1][0] as u32, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001b_eef4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001b_ef08)?;
        let missing = c.g[V0][0] == 0;
        Addr(p(c.g[S1][0] as u32, 0x40)).sw(c, c.g[V0][0]);
        if !missing {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001b_ef20)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001a_8768, 0x001b_ef28)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateStats::Init(self)` (0x1c2e00): create the stats screen, start
/// its thread and refresh the result rows.
pub fn c_fe_state_stats_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5c68));
        c.g[S0][0] = Addr(p(c.g[S1][0] as u32, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001c_2e24)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001c_2e38)?;
        let missing = c.g[V0][0] == 0;
        Addr(p(c.g[S1][0] as u32, 0x40)).sw(c, c.g[V0][0]);
        if !missing {
            c.cycles += 6;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001c_2e50)?;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x001c_3310, 0x001c_2e58)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Small register helpers.

/// The address `reg + off` (32-bit address arithmetic).
#[inline(always)]
pub(crate) fn at_2(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}


/// A `gp`-relative global.
#[inline(always)]
pub(crate) fn gpr(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}


/// `void NetFEState_OnMessengerPopupEvent(NetFEState* self, void* a1, int
/// event)` (0x1a9268): the messenger popup's confirm event allocates and
/// opens the online-character-select state; every other event is forwarded.
pub fn net_fe_state_on_messenger_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xf;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        if c.g[A2][0] != c.g[V0][0] {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001a_93d0, 0x001a_92c8)?;
        } else {
            c.cycles += 16;
            c.g[A2][0] = 0xf;
            call_ptr(c, 0x001a_93d0, 0x001a_9288)?;
            c.g[A1][0] = 0x461c_60;
            c.g[A3][0] = 0;
            c.g[A0][0] = 0x6e4;
            c.g[A2][0] = 0x100;
            call_ptr(c, 0x0031_7d70, 0x001a_92a0)?;
            c.g[A1][0] = Addr(at_2(c, S0, 0x10)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x001b_0f60, 0x001a_92ac)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x001a_8a40, 0x001a_92b8)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void NetFEState_ShowWaitPlayerPopup(NetFEState* self)` (0x1ab108):
/// configure and show the "waiting for players" popup.
pub fn net_fe_state_show_wait_player_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x001a_9c50, 0x001a_b120)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = 0x836;
        Addr(at_2(c, S0, 0xbc)).sw(c, 0);
        c.g[V0][0] = 6;
        Addr(at_2(c, S0, 0x170)).sw(c, 0);
        c.g[A0][0] = 0x4627_50;
        Addr(at_2(c, S0, 0x150)).sw(c, c.g[V1][0]);
        Addr(at_2(c, S0, 0x14c)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7670, 0x001a_b148)?;
        c.g[V1][0] = 3;
        Addr(at_2(c, S0, 0x164)).sw(c, 0);
        Addr(at_2(c, S0, 0xc4)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        Addr(at_2(c, S0, 0xc0)).sw(c, c.g[V1][0]);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x001a_9bc0, 0x001a_b164)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateChallengePopup_v32(cFEStateChallengePopup* self)` (0x1aebf0):
/// ensure the buddy manager exists and is connected, disconnect the lobby
/// when it vanished, and publish the player's presence.
pub fn c_fe_state_challenge_popup_v32(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(gpr(c, 0x238)).lw(c);
        let exists = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !exists {
            c.cycles += 2;
            call_ptr(c, 0x0026_0f80, 0x001a_ec08)?;
        }
        c.cycles += 4;
        c.g[A0][0] = Addr(gpr(c, 0x238)).lw(c);
        call_ptr(c, 0x0026_1460, 0x001a_ec10)?;
        let connected = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(gpr(c, 0x238)).lw(c);
        if !connected {
            c.cycles += 3;
            c.g[A0][0] = Addr(gpr(c, 0x238)).lw(c);
            call_ptr(c, 0x0026_13c8, 0x001a_ec20)?;
            c.g[V0][0] = Addr(gpr(c, 0x238)).lw(c);
        }
        c.cycles += 2;
        let have_buddy = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(gpr(c, -0xc8)).lw(c);
        if !have_buddy {
            c.cycles += 3;
            c.g[A0][0] = Addr(gpr(c, -0xc8)).lw(c);
            call_ptr(c, 0x0025_b800, 0x001a_ec34)?;
            c.g[V0][0] = Addr(gpr(c, -0xc8)).lw(c);
        }
        c.cycles += 10;
        c.g[A1][0] = 1;
        c.g[A0][0] = Addr(gpr(c, 0x238)).lw(c);
        c.g[A2][0] = 7;
        c.g[A3][0] = Addr(at_2(c, V0, 0x98)).lw(c);
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        call_ptr(c, 0x0026_2768, 0x001a_ec54)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateChallengeReceivedPopup_TransitionIn(cFEState* self, void*
/// a1)` (0x1b0670): finish the popup transition and, when a challenge is
/// given, fill in its text.
pub fn c_fe_state_challenge_received_popup_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(at_2(c, S1, 8)).lw(c);
        c.g[A0][0] = Addr(at_2(c, V0, 0x100)).lh(c);
        c.g[V1][0] = Addr(at_2(c, V0, 0x104)).lw(c);
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at_2(c, S1, 0).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x001b_069c)?;
        let none = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !none {
            c.cycles += 10;
            call_ptr(c, 0x0014_dc80, 0x001b_06ac)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_dd58, 0x001b_06b8)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = Addr(at_2(c, A0, 0x40)).lw(c);
            call_ptr(c, 0x001b_0850, 0x001b_06c8)?;
        }
        c.cycles += 5;
        c.g[V0][0] = 1;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void* cFEStateCreateStep1_ctor(cFEStateCreateStep1* self, void* a1, int
/// a2)` (0x1b88f0): construct the net front-end state and install the
/// create-step-1 class and mode.
pub fn c_fe_state_create_step1_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        call_ptr(c, 0x001a_8500, 0x001b_8910)?;
        c.g[V1][0] = Addr(gpr(c, 0xda0)).lw(c);
        c.g[V0][0] = 0x4686_c0;
        Addr(at_2(c, S0, 0x6f0)).sw(c, 0);
        Addr(at_2(c, S0, 8)).sw(c, c.g[V0][0]);
        Addr(at_2(c, S0, 0x6dc)).sw(c, c.g[V1][0]);
        Addr(at_2(c, S0, 0x6d4)).sw(c, c.g[V1][0]);
        Addr(at_2(c, S0, 0x6d8)).sw(c, c.g[V1][0]);
        let none = c.g[S1][0] == 0;
        c.g[V0][0] = 0x50;
        if !none {
            c.cycles += 1;
            c.g[V0][0] = 0x4d;
        }
        c.cycles += 9;
        Addr(at_2(c, S0, 0xc)).sw(c, c.g[V0][0]);
        Addr(at_2(c, S0, 0x6ec)).sw(c, c.g[S1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(at_2(c, S0, 0x6a8)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateCreateStep1_SetSelection(cFEState* self, u32 name_hash,
/// u32 field)` (0x1b9740): select the list-box item matching the state's
/// boolean flag at_2 `field`.
pub(crate) fn create_step1_selection(
    c: &mut Cpu,
    name: u32,
    field: u32,
    ret_str: u32,
    ret_find: u32,
    ret_find2: u32,
) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S1, 0x20);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x30);
    c.g[A0][0] = sx(name);
    frame.save_ra(c, 0x10);
    call_ptr(c, 0x0031_7670, ret_str)?;
    c.g[A0][0] = Addr(at_2(c, S1, 0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, ret_find)?;
    c.g[S0][0] = c.g[V0][0];
    let none = c.g[S0][0] == 0;
    c.g[A2][0] = c.g[SP][0];
    if !none {
        c.cycles += 9;
        c.g[V0][0] = Addr(at_2(c, S1, 0x6d0)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        Addr(at_2(c, SP, 0)).sb(c, 0);
        c.g[A1][0] = Addr(at_2(c, V0, field)).lw(c);
        c.g[A1][0] = (0u64 < c.g[A1][0]) as u64;
        call_ptr(c, 0x0039_a768, ret_find2)?;
        c.g[A1][0] = Addr(at_2(c, SP, 0)).lbu(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_a7a8, ret_find2 + 0xc)?;
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}


/// `void cFEStateCreateStep1_SetInformSelection(cFEState* self)` (0x1b9740).
pub fn c_fe_state_create_step1_set_inform_selection(c: &mut Cpu) {
    guest(c, |c| {
        create_step1_selection(c, 0x4647_80, 0x38, 0x001b_9760, 0x001b_976c, 0x001b_9790)
    })
}


/// `void cFEStateCreateStep1_SetShareInfoSelection(cFEState* self)`
/// (0x1b97b0).
pub fn c_fe_state_create_step1_set_share_info_selection(c: &mut Cpu) {
    guest(c, |c| {
        create_step1_selection(c, 0x4647_90, 0x3c, 0x001b_97d0, 0x001b_97dc, 0x001b_9800)
    })
}


/// `void cFEStateCreateStep2_SetLegalSelection(cFEState* self)` (0x1bb4b0).
pub fn c_fe_state_create_step2_set_legal_selection(c: &mut Cpu) {
    guest(c, |c| {
        create_step1_selection(c, 0x464a_08, 0x40, 0x001b_b4d0, 0x001b_b4dc, 0x001b_b500)
    })
}


/// `void cFEStateCreateDuplicate_SubmitAccount(cFEState* self)` (0x1bc290):
/// configure and show the account-exists popup, then create the account.
pub fn c_fe_state_create_duplicate_submit_account(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x001a_9c50, 0x001b_c2a8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x464a_38;
        Addr(at_2(c, S0, 0xbc)).sw(c, 0);
        call_ptr(c, 0x0031_7670, 0x001b_c2bc)?;
        c.g[V1][0] = 3;
        c.g[A2][0] = 0x820;
        Addr(at_2(c, S0, 0x164)).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        Addr(at_2(c, S0, 0xc4)).sw(c, c.g[V0][0]);
        Addr(at_2(c, S0, 0xc0)).sw(c, c.g[V1][0]);
        Addr(at_2(c, S0, 0x150)).sw(c, c.g[A2][0]);
        call_ptr(c, 0x001a_9bc0, 0x001b_c2e0)?;
        c.g[A0][0] = Addr(gpr(c, -0xc8)).lw(c);
        c.g[A1][0] = sx(at_2(c, A0, 0x260));
        call_ptr(c, 0x0025_b9c8, 0x001b_c2ec)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateLobbyMenu_ClampRoomSelections(cFEState* self)` (0x1be038):
/// keep each of the four room-list selections inside its item count.
pub fn c_fe_state_lobby_menu_clamp_room_selections(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(at_2(c, A0, 0x708));
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(at_2(c, S0, 0)).lw(c);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.g[V1][0] = Addr(at_2(c, S0, 0x48)).lw(c);
            c.g[A0][0] = Addr(at_2(c, V0, 0x318)).lbu(c);
            c.g[V1][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
            let in_range = c.g[V1][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
            if !in_range {
                c.cycles += 1;
                Addr(at_2(c, S0, 0x48)).sw(c, c.g[A0][0]);
            }
            c.cycles += 4;
            c.g[V1][0] = Addr(at_2(c, S0, 0)).lw(c);
            c.g[V0][0] = Addr(at_2(c, V1, 0x318)).lbu(c);
            let empty = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V1][0];
            if !empty {
                c.cycles += 2;
                c.g[A1][0] = Addr(at_2(c, S0, 0x48)).lbu(c);
                call_ptr(c, 0x0039_a7a8, 0x001b_e088)?;
            }
            c.cycles += 3;
            c.g[V0][0] = (c.g[S1][0] < 0x4u64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001b_e050);
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


/// `void cFEStateNetNews_Init(cFEStateNetNews* self)` (0x1c5098): create the
/// net-news screen, start its thread and request the news list.
pub fn c_fe_state_net_news_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x4a1c_30;
        c.g[S1][0] = Addr(at_2(c, S0, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001c_50bc)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001c_50d0)?;
        let t = c.g[V0][0] == 0;
        Addr(at_2(c, S0, 0x40)).sw(c, c.g[V0][0]);
        if !t {
            c.cycles += 7;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001c_50e8)?;
            c.g[A0][0] = Addr(gpr(c, -0xc8)).lw(c);
            c.g[A1][0] = 1;
            call_ptr(c, 0x0025_e0c8, 0x001c_50f4)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Front-end states.

/// `cFEStateNetMessenger_Init` (0x1A3F68): look up the net-messenger screen by
/// hash name in the UI engine, store it at +0x40, and when it exists play its
/// frame and build the buddy/player lists.
pub fn c_fe_state_net_messenger_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = low(c, A0).wrapping_add(0x1448);
        c.g[S0][0] = lw_at_3(c, S1, 0x10);
        call_ptr(c, addr::STR_HASH, 0x001a_3f8c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, addr::UI_ENGINE_CREATE_SCREEN, 0x001a_3fa0)?;
        let screen = c.g[V0][0];
        let created = screen != 0;
        sw_at(c, S1, 0x40, screen);
        if created {
            c.cycles += 8;
            c.g[A0][0] = screen;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, addr::UI_SCREEN_START_THREAD, 0x001a_3fb8)?;
            c.g[A0][0] = c.g[S1][0];
            callee::BUILD_BUDDY_LIST.call(c, 0x001a_3fc0)?;
            c.g[A0][0] = c.g[S1][0];
            callee::REFRESH_PLAYER_LIST.call(c, 0x001a_3fc8)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateNetMessenger_v33` (0x1A4928): on a child event with key 0xF, open
/// the buddy action popup for the selected user if their name matches the
/// "me" name; every other event goes to the base child-event handler.
pub fn c_fe_state_net_messenger_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xf;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let child_event = c.g[A2][0] != c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if child_event {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::ON_CHILD_EVENT.call(c, 0x001a_498c)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 6;
            c.g[V0][0] = Addr(gp_9(c, -0xc8)).lw(c);
            c.g[A1][0] = lw_at_3(c, S0, 0x6d0);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x80)).lw(c);
            call_ptr(c, addr::STRCASECMP, 0x001a_4950)?;
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 11;
                c.g[A1][0] = lw_at_3(c, S0, 0x6d0);
                c.g[A0][0] = c.g[S0][0];
                callee::SET_SELECTED_USER.call(c, 0x001a_4964)?;
                c.g[A0][0] = c.g[S0][0];
                callee::GET_SELECTED_USER.call(c, 0x001a_496c)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = 0;
                callee::SHOW_BUDDY_ACTION_POPUP.call(c, 0x001a_497c)?;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateChatRoom_OnGoToCharSelectEvent` (0x1B3760): close unpinned popups,
/// allocate and construct the character-select screen, and switch to it.
pub fn c_fe_state_chat_room_on_go_to_char_select_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xf;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let child_event = c.g[A2][0] != c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if child_event {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::ON_CHILD_EVENT.call(c, 0x001b_37c4)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 17;
            c.g[A0][0] = lw_at_3(c, S0, 0x10);
            c.g[A0][0] = low(c, A0).wrapping_add(0x18);
            call_ptr(c, addr::CLOSE_UNPINNED_POPUPS, 0x001b_3784)?;
            c.g[A1][0] = 0x46_0000;
            c.g[A3][0] = 0;
            c.g[A1][0] = low(c, A1).wrapping_add(0x1c60);
            c.g[A0][0] = 0x6e4;
            c.g[A2][0] = 0x100;
            call_ptr(c, addr::MEM_MAN_ALLOC, 0x001b_379c)?;
            c.g[A1][0] = lw_at_3(c, S0, 0x10);
            c.g[A0][0] = c.g[V0][0];
            callee::ONLINE_CHARACTER_SELECT_CTOR.call(c, 0x001b_37a8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            callee::GOTO_NEXT_STATE.call(c, 0x001b_37b4)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateChatRoom_OnGoToCharSelectEvent_2` (0x1B37D8): a second entry point
/// of the same chat-room transition (identical body).
pub fn c_fe_state_chat_room_on_go_to_char_select_event_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xf;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let child_event = c.g[A2][0] != c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if child_event {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::ON_CHILD_EVENT.call(c, 0x001b_383c)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 17;
            c.g[A0][0] = lw_at_3(c, S0, 0x10);
            c.g[A0][0] = low(c, A0).wrapping_add(0x18);
            call_ptr(c, addr::CLOSE_UNPINNED_POPUPS, 0x001b_37fc)?;
            c.g[A1][0] = 0x46_0000;
            c.g[A3][0] = 0;
            c.g[A1][0] = low(c, A1).wrapping_add(0x1c60);
            c.g[A0][0] = 0x6e4;
            c.g[A2][0] = 0x100;
            call_ptr(c, addr::MEM_MAN_ALLOC, 0x001b_3814)?;
            c.g[A1][0] = lw_at_3(c, S0, 0x10);
            c.g[A0][0] = c.g[V0][0];
            callee::ONLINE_CHARACTER_SELECT_CTOR.call(c, 0x001b_3820)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            callee::GOTO_NEXT_STATE.call(c, 0x001b_382c)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateOnlineMainMenu_v31` (0x1C0358): the online main menu's event
/// filter: 0xEF is swallowed, 0xF3 opens the character select screen, and
/// everything else falls through to the net event handler. Returns 0.
pub fn c_fe_state_online_main_menu_v31(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xef;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let is_ef = c.g[A1][0] == c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        let mut handled = true;
        if !is_ef {
            c.cycles += 3;
            c.g[V0][0] = 0xf3;
            if c.g[A1][0] == c.g[V0][0] {
                c.g[A1][0] = 0x46_0000;
            } else {
                c.cycles += 4;
                callee::HANDLE_NET_EVENT.call(c, 0x001c_0384)?;
                handled = false;
            }
            if handled {
                c.cycles += 11;
                c.g[A0][0] = 0x6e4;
                c.g[A1][0] = low(c, A1).wrapping_add(0x1c60);
                c.g[A2][0] = 0x100;
                c.g[A3][0] = 0;
                call_ptr(c, addr::MEM_MAN_ALLOC, 0x001c_03a0)?;
                c.g[A1][0] = lw_at_3(c, S0, 0x10);
                c.g[A0][0] = c.g[V0][0];
                callee::ONLINE_CHARACTER_SELECT_CTOR.call(c, 0x001c_03ac)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                callee::GOTO_NEXT_STATE.call(c, 0x001c_03b8)?;
            }
        }
        if handled {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Helpers.

/// The low_2 32 bits of a register (an object pointer or a 32-bit value).
#[inline]
pub(crate) fn low_2(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}


/// `lw off(reg)`.
#[inline]
pub(crate) fn lw_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low_2(c, reg).wrapping_add(off)).lw(c)
}


/// `lh off(reg)` (sign-extended).
#[inline]
pub(crate) fn lh_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low_2(c, reg).wrapping_add(off)).lh(c)
}


/// `sw off(reg)`.
#[inline]
pub(crate) fn sw_r(c: &mut Cpu, reg: usize, off: u32, v: u64) {
    let a = low_2(c, reg).wrapping_add(off);
    c.sw(a, v);
}


/// `sb off(reg)`.
#[inline]
pub(crate) fn sb_r(c: &mut Cpu, reg: usize, off: u32, v: u64) {
    let a = low_2(c, reg).wrapping_add(off);
    c.sb(a, v);
}


/// `func_001C2D18` (0x1c2d18): the network statistics popup constructor. Runs
/// `NetFEState_ctor`, installs vtable `0x468278`, sets `+0xc = 0x42` and
/// `+0x6ec = 1`, clears four words at `+0x6e0` and then the four-slot array at
/// `+0x6dc` (a `for (i = 3; i >= 0; i--)` countdown loop with an event check).
/// Signature: `void ctor(this)`.
pub fn c_fe_state_stats_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x001c_2d30);
        c.g[S0][0] = c.g[A0][0];
        callee::NET_FE_STATE_CTOR_9.call(c, 0x001c_2d30)?;
        c.g[V1][0] = 0x47_0000;
        c.g[V0][0] = 0x1;
        c.g[V1][0] = sx(low_2(c, V1).wrapping_sub(0x7d88));
        c.g[A0][0] = 0x42;
        sw_r(c, S0, 0x8, c.g[V1][0]);
        sw_r(c, S0, 0x6ec, c.g[V0][0]);
        c.g[V1][0] = 0x3;
        sw_r(c, S0, 0xc, c.g[A0][0]);
        c.g[V0][0] = sx(low_2(c, S0).wrapping_add(0x6dc));
        sw_r(c, S0, 0x6e0, 0);
        sw_r(c, S0, 0x6e4, 0);
        sw_r(c, S0, 0x6e8, 0);
        loop {
            c.cycles += 7;
            sw_r(c, V0, 0x0, 0);
            c.g[V1][0] = sx(low_2(c, V1).wrapping_sub(0x1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(low_2(c, V0).wrapping_sub(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001c_2d60);
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(low_2(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Front-end state helpers.

/// `func_001B00B8` (0x1b00b8): `cFEStateChallengeReceivedPopup`. Makes sure
/// the network popup manager exists (`0x260f80`), refreshes it (`0x261460` /
/// `0x2613c8`), then posts the challenge message: `0x262768(manager, 1, 8,
/// this->+0x6d0, 0, 0)`. Signature: `void popup(this)`.
pub fn c_fe_state_challenge_received_popup_v32(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = lw_r(c, GP, 0x238);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let have = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        if !have {
            c.cycles += 2;
            c.g[RA][0] = sx(0x001b_00d8);
            call_ptr(c, 0x0026_0f80, 0x001b_00d8)?;
        }
        c.cycles += 4;
        c.g[RA][0] = sx(0x001b_00e0);
        c.g[A0][0] = lw_r(c, GP, 0x238);
        call_ptr(c, 0x0026_1460, 0x001b_00e0)?;
        let busy = c.g[V0][0] != 0;
        c.g[V0][0] = lw_r(c, GP, 0x238);
        if !busy {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001b_00f0);
            c.g[A0][0] = lw_r(c, GP, 0x238);
            call_ptr(c, 0x0026_13c8, 0x001b_00f0)?;
            c.g[V0][0] = lw_r(c, GP, 0x238);
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.g[A3][0] = lw_r(c, S0, 0x6d0);
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001b_0104);
            c.g[A0][0] = lw_r(c, GP, 0u32.wrapping_sub(0xc8));
            call_ptr(c, 0x0025_b800, 0x001b_0104)?;
            c.g[A3][0] = lw_r(c, S0, 0x6d0);
        }
        c.cycles += 10;
        c.g[A1][0] = 0x1;
        c.g[A0][0] = lw_r(c, GP, 0x238);
        c.g[A2][0] = 0x8;
        c.g[T0][0] = 0;
        c.g[RA][0] = sx(0x001b_0120);
        c.g[T1][0] = 0;
        call_ptr(c, 0x0026_2768, 0x001b_0120)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `func_001BCC30` (0x1bcc30): `cFEStateLobbyMenu::Init`. Creates the screen
/// widget named `0x464da0` in `this->+0x10` (`0x397cc0`), stores it at `+0x40`
/// and, if created, shows it (`0x39c870`) and calls the state's `vtable[0xe0]`
/// refresh. Signature: `void init(this)`.
pub fn c_fe_state_lobby_menu_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = sx(low_2(c, A0).wrapping_add(0x4da0));
        c.g[RA][0] = sx(0x001b_cc54);
        c.g[S0][0] = lw_r(c, S1, 0x10);
        call_ptr(c, 0x0031_7670, 0x001b_cc54)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001b_cc68);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001b_cc68)?;
        let missing = c.g[V0][0] == 0;
        sw_r(c, S1, 0x40, c.g[V0][0]);
        if !missing {
            c.cycles += 9;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001b_cc80);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001b_cc80)?;
            c.g[V1][0] = lw_r(c, S1, 0x8);
            c.g[A0][0] = lh_r(c, V1, 0xe0);
            c.g[V0][0] = lw_r(c, V1, 0xe4);
            let target = low_2(c, V0);
            c.g[RA][0] = sx(0x001b_cc94);
            c.g[A0][0] = sx(low_2(c, S1).wrapping_add(low_2(c, A0)));
            call_ptr(c, target, 0x001b_cc94)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `func_001BDF60` (0x1bdf60): `cFEStateLobbyMenu::ShowRoomTab`. Hides all
/// room tabs (`0x1bdfd8`) and then walks the four tab slots at `this+0x730`
/// (each `+0x18` back-pointer compared with `a1`) showing the matching one
/// (`vtable[0x48]` with 1). The loop's back edge carries an event check.
pub fn c_fe_state_lobby_menu_show_room_tab(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x001b_df7c);
        c.g[S1][0] = c.g[A1][0];
        callee::LOBBY_MENU_HIDE_ROOM_TABS.call(c, 0x001b_df7c)?;
        c.g[A2][0] = sx(low_2(c, S0).wrapping_add(0x730));
        c.g[V1][0] = 0;
        loop {
            c.cycles += 3;
            c.g[V0][0] = lw_r(c, A2, 0u32.wrapping_sub(0x18));
            let other = c.g[S1][0] != c.g[V0][0];
            c.g[V1][0] = sx(low_2(c, V1).wrapping_add(1));
            if !other {
                c.cycles += 9;
                c.g[A2][0] = Addr(low_2(c, A2)).lw(c);
                c.g[A1][0] = 0x1;
                c.g[V0][0] = lw_r(c, A2, 0x8);
                c.g[A0][0] = lh_r(c, V0, 0x48);
                c.g[V1][0] = lw_r(c, V0, 0x4c);
                let target = low_2(c, V1);
                c.g[RA][0] = sx(0x001b_dfb0);
                c.g[A0][0] = sx(low_2(c, A2).wrapping_add(low_2(c, A0)));
                call_ptr(c, target, 0x001b_dfb0)?;
                break;
            }
            c.cycles += 3;
            let more = (c.g[V1][0] as i64) < 4;
            c.g[V0][0] = more as u64;
            c.g[A2][0] = sx(low_2(c, A2).wrapping_add(0x4));
            if !more {
                c.cycles += 1;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001b_df88);
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `func_001C0FC0` (0x1c0fc0): `cFEStateNetQuickPlayPopup::Init`. Creates the
/// popup named `0x4657e0` in `this->+0x10`, stores it at `+0x40`, shows it
/// (`0x39c870`) and copies the 0x20-byte default string at `0x5308b8` into the
/// popup (`0x3e6448(dst, 0, 0x20)`). Signature: `void init(this)`.
pub fn c_fe_state_net_quick_play_popup_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = sx(low_2(c, A0).wrapping_add(0x57e0));
        c.g[RA][0] = sx(0x001c_0fe4);
        c.g[S1][0] = lw_r(c, S0, 0x10);
        call_ptr(c, 0x0031_7670, 0x001c_0fe4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x001c_0ff8);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001c_0ff8)?;
        let missing = c.g[V0][0] == 0;
        sw_r(c, S0, 0x40, c.g[V0][0]);
        if !missing {
            c.cycles += 9;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001c_1010);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001c_1010)?;
            c.g[A0][0] = 0x53_0000;
            c.g[A1][0] = 0;
            c.g[A0][0] = sx(low_2(c, A0).wrapping_add(0x8b8));
            c.g[RA][0] = sx(0x001c_1024);
            c.g[A2][0] = 0x20;
            call_ptr(c, 0x003e_6448, 0x001c_1024)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `func_001A98B8` (0x1a98b8): `NetFEState_MakePasswordMask`. Masks the
/// password at `this+0x5c` with `'*'`: every character of the `strlen`
/// password (at most 63) is overwritten with `0x2a`, the string is then
/// terminated at the index the loop stopped at, and a zero-length password
/// becomes a single space. Signature:
/// `void make_mask(NetFEState* this, const char* password)`.
pub fn net_fe_state_make_password_mask(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x001a_98d0);
        c.g[A0][0] = c.g[A1][0];
        call_ptr(c, 0x0041_6810, 0x001a_98d0)?;
        c.g[A0][0] = c.g[V0][0];
        let none = (c.g[A0][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[A2][0] = sx(low_2(c, S0).wrapping_add(0x5c));
            c.g[A3][0] = 0x2a;
            c.g[V1][0] = sx(low_2(c, A2).wrapping_add(low_2(c, A1)));
            loop {
                c.cycles += 4;
                c.g[A1][0] = sx(low_2(c, A1).wrapping_add(1));
                let more = (c.g[A1][0] as i64) < (c.g[A0][0] as i64);
                c.g[V0][0] = more as u64;
                sb_r(c, V1, 0, c.g[A3][0]);
                if !more {
                    break;
                }
                c.cycles += 3;
                let limit = (c.g[A1][0] as i64) < 63;
                c.g[V0][0] = limit as u64;
                if !limit {
                    break;
                }
                c.g[V1][0] = sx(low_2(c, A2).wrapping_add(low_2(c, A1)));
                if event_due(c) {
                    deliver_event_here(c, 0x001a_98e8);
                }
            }
        }
        c.cycles += 2;
        let terminated = c.g[A0][0] != 0;
        c.g[V0][0] = sx(low_2(c, S0).wrapping_add(low_2(c, A0)));
        if !terminated {
            c.cycles += 4;
            c.g[V0][0] = 0x20;
            c.g[A0][0] = 0x1;
            sb_r(c, S0, 0x5c, c.g[V0][0]);
            c.g[V0][0] = sx(low_2(c, S0).wrapping_add(low_2(c, A0)));
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld(low_2(c, SP));
        frame.restore(c, S0, 0x10);
        sb_r(c, V0, 0x5c, 0);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(low_2(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}


/// `void cFEAsyncManager_Load3PeakPic(self, slot, param)` (0x1a36e8): name the
/// slot's 0x11c-byte file entry, store the parameter and mark it loading.
pub fn fe_async_manager_request_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        c.g[SP][0] = sx(low_3(c, SP).wrapping_sub(0x50));
        c.g[A3][0] = 0x4a_0000;
        c.sq(16, sp_2(c).wrapping_add(0x40));
        c.g[A3][0] = sx(low_3(c, A3).wrapping_add(0x1a00));
        c.sq(18, sp_2(c).wrapping_add(0x20));
        c.g[S0][0] = 0x11c;
        c.g[S2][0] = c.g[A1][0];
        c.sq(19, sp_2(c).wrapping_add(0x10));
        c.mult(0, S0, S2, S0); // lo = slot * 0x11c
        c.sq(17, sp_2(c).wrapping_add(0x30));
        c.g[S3][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = Addr(gpo(c, -0x1700)).lw(c);
        c.g[A1][0] = 0x4a_0000;
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[A1][0] = sx(low_3(c, A1).wrapping_add(0x19f8));
        c.g[A0][0] = sx(low_3(c, S1).wrapping_add(low_3(c, S0)));
        call_ptr(c, callee::SPRINTF_3, 0x001a_3730)?;
        c.g[S0][0] = sx(low_3(c, S1).wrapping_add(low_3(c, S0)));
        c.g[A0][0] = c.g[S1][0];
        sw_r(c, S0, 0x108, c.g[S3][0]);
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = 0x1;
        callee::FE_ASYNC_MANAGER_SET_FILE_STATUS.call(c, 0x001a_3748)?;
        c.lq(16, sp_2(c).wrapping_add(0x40));
        c.lq(17, sp_2(c).wrapping_add(0x30));
        c.lq(18, sp_2(c).wrapping_add(0x20));
        c.lq(19, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low_3(c, RA);
        c.g[SP][0] = sx(low_3(c, SP).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Front end / net.

/// `void FE_SetOnlineMode(int mode)` (0x001a86e8): record the mode at
/// `gp_10-0x1680`; going offline disconnects the lobby and buddy session.
pub fn fe_set_online_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        Addr(gp_10(c, -0x1680)).sw(c, c.g[A0][0]);
        let online = c.g[A0][0] != 0;
        frame.save_ra(c, 0);
        if online {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            c.g[A0][0] = Addr(gp_10(c, 0x238)).lw(c);
            let absent = c.g[A0][0] == 0;
            frame.restore_ra(c, 0);
            if absent {
                // Nothing to disconnect.
            } else {
                c.cycles += 4;
                callee::BUDDY_MAN_IS_CONNECTED.call(c, 0x001a_870c)?;
                let connected = c.g[V0][0];
                c.g[V0][0] = Addr(gp_10(c, 0x238)).lw(c);
                if connected != 0 {
                    // The session is up: make sure the lobby manager exists,
                    // reconnect the buddy session if it dropped out.
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.cycles += 2;
                        callee::LOBBY_MAN.call(c, 0x001a_8724)?;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = Addr(gp_10(c, 0x238)).lw(c);
                    callee::BUDDY_MAN_IS_CONNECTED.call(c, 0x001a_872c)?;
                    let still = c.g[V0][0];
                    c.g[V0][0] = Addr(gp_10(c, 0x238)).lw(c);
                    if still == 0 {
                        c.cycles += 3;
                        c.g[A0][0] = Addr(gp_10(c, 0x238)).lw(c);
                        callee::BUDDY_MAN_RECONNECT.call(c, 0x001a_873c)?;
                        c.g[V0][0] = Addr(gp_10(c, 0x238)).lw(c);
                    }
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.cycles += 2;
                        c.g[A0][0] = Addr(gp_10(c, -0xc8)).lw(c);
                        callee::LOBBY_MAN_DISCONNECT_2.call(c, 0x001a_8750)?;
                    }
                    c.cycles += 2;
                    c.g[A0][0] = Addr(gp_10(c, 0x238)).lw(c);
                    callee::BUDDY_MAN_DISCONNECT.call(c, 0x001a_8758)?;
                }
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// `void NetFE_ShowPlayerConnQuality(cUIScreen* screen, player*)`
/// (0x001afd88): fill the connection-quality text and icons.
pub fn net_fe_show_player_conn_quality(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[A0][0] = sx(0x460000);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.g[A0][0] = sx(0x460000u32.wrapping_add(0x3538));
        callee::GET_HASH_VALUE_32.call(c, 0x001a_fdb0)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        callee::UI_SCREEN_FIND_OBJECT.call(c, 0x001a_fdbc)?;
        c.g[S0][0] = c.g[V0][0];
        let absent = c.g[S0][0] == 0;
        c.g[A0][0] = sx(0x460000);
        if !absent {
            c.cycles += 9;
            c.g[A0][0] = sx(0x460000u32.wrapping_add(0x35c0));
            callee::GET_HASH_VALUE_32.call(c, 0x001a_fdd0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            callee::UI_TEXT_SET_UNICODE_STRING_BY_ID.call(c, 0x001a_fddc)?;
            c.g[A2][0] = Addr(at(c, S2, 0xc)).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            callee::NET_FE_SET_QUALITY_ICONS.call(c, 0x001a_fdec)?;
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


/// `void cFEStateLobbyMenu_OnNetResult(self, msg, a2)` (0x001bd848): dispatch
/// a net result to the create/join room handlers or the generic handler.
pub fn c_fe_state_lobby_menu_on_net_result(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at(c, A0, 0x1c)).lw(c);
        c.g[V0][0] &= 6;
        let none = c.g[V0][0] == 0;
        c.g[V1][0] = 0x828;
        if none {
            c.cycles += 3;
            // a0 still holds the message (self), as at the original call.
            callee::NET_FE_STATE_HANDLE_NET_EVENT_2.call(c, 0x001b_d8b8)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(at(c, A0, 0x6a0)).lw(c);
            c.g[A3][0] = Addr(at(c, V0, 0x18)).lw(c);
            c.g[V0][0] = 0x829;
            if c.g[A3][0] == 0x828 {
                c.cycles += 4;
                c.g[A1][0] = c.g[A2][0];
                callee::LOBBY_MENU_ON_CREATE_ROOM_RESULT.call(c, 0x001b_d888)?;
                frame.restore_ra(c, 0);
            } else if c.g[A3][0] == 0x829 {
                c.cycles += 2;
                c.cycles += 4;
                c.g[A1][0] = c.g[A2][0];
                callee::LOBBY_MENU_ON_JOIN_ROOM_RESULT.call(c, 0x001b_d898)?;
                frame.restore_ra(c, 0);
            } else {
                c.cycles += 2;
                c.cycles += 2;
                c.cycles += 4;
                callee::NET_FE_STATE_HANDLE_NET_EVENT_2.call(c, 0x001b_d8a8)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateNetQuickPlayPopup* ctor(self)` (0x001c0f40): the net quick-play
/// popup state's constructor.
pub fn c_fe_state_net_quick_play_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::NET_FE_STATE_CTOR_10.call(c, 0x001c_0f54)?;
        c.g[V0][0] = 0x48;
        c.g[V1][0] = 0x470000;
        Addr(at(c, S0, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx(0x470000u32.wrapping_sub(0x54d8));
        Addr(at(c, S0, 0x8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        for off in [
            0x6ec, 0x720, 0x724, 0x728, 0x72c, 0x730, 0x6f8, 0x6fc, 0x700, 0x704, 0x708, 0x70c,
            0x710, 0x714, 0x718, 0x71c,
        ] {
            Addr(at(c, S0, off)).sw(c, 0);
        }
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateNetSSXWorld_Init(self)` (0x001c1c90): create the net SSX
/// World screen and (when disconnected) its popup.
pub fn c_fe_state_net_ssx_world_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(0x460000);
        c.g[A0][0] = sx(0x460000u32.wrapping_add(0x5a38));
        c.g[S0][0] = Addr(at(c, S1, 0x10)).lw(c);
        callee::GET_HASH_VALUE_32.call(c, 0x001c_1cb4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME.call(c, 0x001c_1cc8)?;
        let absent = c.g[V0][0] == 0;
        Addr(at(c, S1, 0x40)).sw(c, c.g[V0][0]);
        if !absent {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            callee::UI_SCREEN_PLAY_FRAME.call(c, 0x001c_1ce0)?;
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(gp_10(c, -0xc8)).lw(c);
        c.g[V1][0] = Addr(at(c, V0, 0)).lw(c);
        let online = c.g[V1][0] != 0;
        c.lq(16, frame.at(c, 0x20));
        if !online {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP.call(c, 0x001c_1cf8)?;
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
// Small helpers mirroring the generated code's idioms.

/// A `gp`-relative global address.
#[inline(always)]
pub(crate) fn gpa(c: &Cpu, off: u32) -> u32 {
    abi::r32(c, GP).wrapping_add(off)
}


/// A `gp`-relative global written with a negative offset.
#[inline(always)]
pub(crate) fn gpn(c: &Cpu, off: u32) -> u32 {
    abi::r32(c, GP).wrapping_sub(off)
}


/// `void NetFEState_ToggleBlockUser(NetFEState* s, void* a, void* b)`
/// (0x001abc50): make sure the buddy manager exists and is connected, then
/// change roster entry `a` to block (kind 1).
pub fn net_fe_state_toggle_block_user(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.g[V0][0] = c.lw(gpa(c, 0x238));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A2][0];
        let have = c.g[V0][0] != 0;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x6c4), 0);
        if !have {
            c.cycles += 2;
            c.g[RA][0] = sx(0x001a_bc7c);
            call_ptr(c, 0x0026_0f80, 0x001a_bc7c)?; // BuddyMan_Create
        }
        c.cycles += 4;
        c.g[A0][0] = c.lw(gpa(c, 0x238));
        call_ptr(c, 0x0026_1460, 0x001a_bc84)?; // BuddyMan_IsConnected
        let connected = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(gpa(c, 0x238));
        if !connected {
            c.cycles += 3;
            c.g[A0][0] = c.lw(gpa(c, 0x238));
            call_ptr(c, 0x0026_13c8, 0x001a_bc94)?; // BuddyMan_Reconnect
            c.g[V0][0] = c.lw(gpa(c, 0x238));
        }
        c.cycles += 2;
        let present = c.g[V0][0] != 0;
        c.g[A0][0] = c.lw(gpa(c, 0x238));
        if !present {
            c.cycles += 3;
            c.g[A0][0] = c.lw(gpn(c, 0xc8));
            call_ptr(c, 0x0025_b800, 0x001a_bca8)?; // LobbyMan_Disconnect
            c.g[A0][0] = c.lw(gpa(c, 0x238));
        }
        c.cycles += 9;
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = 1;
        call_ptr(c, 0x0026_1ef8, 0x001a_bcbc)?; // BuddyMan_ChangeRosterEntry
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}


/// `void NetFEState_ToggleMessengerBuddy(NetFEState* s, void* a, void* b)`
/// (0x001abe58): as [`net_fe_state_toggle_block_user`] but changing the entry
/// to kind 0 (a messenger buddy).
pub fn net_fe_state_toggle_messenger_buddy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.g[V0][0] = c.lw(gpa(c, 0x238));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A2][0];
        let have = c.g[V0][0] != 0;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x6c4), 0);
        if !have {
            c.cycles += 2;
            c.g[RA][0] = sx(0x001a_be84);
            call_ptr(c, 0x0026_0f80, 0x001a_be84)?; // BuddyMan_Create
        }
        c.cycles += 4;
        c.g[A0][0] = c.lw(gpa(c, 0x238));
        call_ptr(c, 0x0026_1460, 0x001a_be8c)?; // BuddyMan_IsConnected
        let connected = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(gpa(c, 0x238));
        if !connected {
            c.cycles += 3;
            c.g[A0][0] = c.lw(gpa(c, 0x238));
            call_ptr(c, 0x0026_13c8, 0x001a_be9c)?; // BuddyMan_Reconnect
            c.g[V0][0] = c.lw(gpa(c, 0x238));
        }
        c.cycles += 2;
        let present = c.g[V0][0] != 0;
        c.g[A0][0] = c.lw(gpa(c, 0x238));
        if !present {
            c.cycles += 3;
            c.g[A0][0] = c.lw(gpn(c, 0xc8));
            call_ptr(c, 0x0025_b800, 0x001a_beb0)?; // LobbyMan_Disconnect
            c.g[A0][0] = c.lw(gpa(c, 0x238));
        }
        c.cycles += 9;
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = 0;
        call_ptr(c, 0x0026_1ef8, 0x001a_bec4)?; // BuddyMan_ChangeRosterEntry
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}


/// `void cFEStateChatRoom_v32(cFEStateChatRoom* s)` (0x001b22a0): ensure the
/// buddy manager exists and is connected, then set the local presence to
/// "chat room" (kind 1/3) using the lobby's kick reason as the payload.
pub fn c_fe_state_chat_room_v32(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.g[V0][0] = c.lw(gpa(c, 0x238));
        c.sq(S0, sp(c).wrapping_add(0x10));
        let have = c.g[V0][0] != 0;
        c.sd(sp(c), c.g[RA][0]);
        if !have {
            c.cycles += 2;
            c.g[RA][0] = sx(0x001b_22bc);
            call_ptr(c, 0x0026_0f80, 0x001b_22bc)?; // BuddyMan_Create
        }
        c.cycles += 4;
        c.g[A0][0] = c.lw(gpa(c, 0x238));
        call_ptr(c, 0x0026_1460, 0x001b_22c4)?; // BuddyMan_IsConnected
        let connected = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(gpa(c, 0x238));
        if !connected {
            c.cycles += 3;
            c.g[A0][0] = c.lw(gpa(c, 0x238));
            call_ptr(c, 0x0026_13c8, 0x001b_22d4)?; // BuddyMan_Reconnect
            c.g[V0][0] = c.lw(gpa(c, 0x238));
        }
        c.cycles += 2;
        let present = c.g[V0][0] != 0;
        c.g[A0][0] = c.lw(gpn(c, 0xc8));
        if !present {
            c.cycles += 3;
            c.g[A0][0] = c.lw(gpn(c, 0xc8));
            call_ptr(c, 0x0025_b800, 0x001b_22e8)?; // LobbyMan_Disconnect
            c.g[A0][0] = c.lw(gpn(c, 0xc8));
        }
        c.cycles += 13;
        c.g[RA][0] = sx(0x001b_22f4);
        c.g[S0][0] = c.lw(gpa(c, 0x238));
        call_ptr(c, 0x0025_da90, 0x001b_22f4)?; // LobbyMan_GetKickReason
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        c.g[A2][0] = 3;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        call_ptr(c, 0x0026_2768, 0x001b_2310)?; // BuddyMan_SetPresence
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}


/// `void cFEStateChatRoom_OnChildEvent(cFEStateChatRoom* s, UIEvent* ev)`
/// (0x001b3850): dispatch the event id at `ev+0x18` to the buddy-action
/// handler (0x80b), to char select (0x800/0x801) or to the base net state.
pub fn c_fe_state_chat_room_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
        c.g[V0][0] = 0x801;
        c.sd(sp(c), c.g[RA][0]);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
        let is_801 = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = (c.g[V1][0] < 0x802) as u64;
        if is_801 {
            // `break 'b_1b38ac`: the char-select-2 handler.
            c.cycles += 4;
            c.g[RA][0] = sx(0x001b_38b4);
            call_ptr(c, 0x001b_37d8, 0x001b_38b4)?;
            c.g[RA][0] = c.ld(sp(c));
        } else {
            // Head of `'b_1b389c`/`'b_1b3880`: both arms charge it.
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                // `break 'b_1b3880`: only 0x80b reaches the buddy handler.
                c.cycles += 3;
                c.g[V0][0] = 0x80b;
                if c.g[V1][0] != c.g[V0][0] {
                    // `break 'b_1b38bc`.
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001b_38c4);
                    call_ptr(c, 0x001a_97b8, 0x001b_38c4)?;
                    c.g[RA][0] = c.ld(sp(c));
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001b_3894);
                    call_ptr(c, 0x001b_35b0, 0x001b_3894)?;
                    c.g[RA][0] = c.ld(sp(c));
                }
            } else {
                c.g[V0][0] = 0x800;
                c.cycles += 2;
                if c.g[V1][0] == c.g[V0][0] {
                    // `break 'b_1b389c`: the char-select handler.
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001b_38a4);
                    call_ptr(c, 0x001b_3760, 0x001b_38a4)?;
                    c.g[RA][0] = c.ld(sp(c));
                } else {
                    // `break 'b_1b38bc`.
                    c.cycles += 2;
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001b_38c4);
                    call_ptr(c, 0x001a_97b8, 0x001b_38c4)?;
                    c.g[RA][0] = c.ld(sp(c));
                }
            }
        }
        c.cycles += 2;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        abi::ret(c);
        Ok(())
    })
}


/// `void NetConfigAuthPopup_dtor(NetConfigAuthPopup* p, int flags)`
/// (0x001b6368): install the class vtable, free the auth buffers, then free
/// the popup's own buffer and run the base destructor.
pub fn net_config_auth_popup_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.g[V0][0] = sx(0x0046_89f0);
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(8), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d0));
        let no_auth = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        let mut charged = false;
        if !no_auth {
            c.cycles += 5;
            call_ptr(c, 0x0025_63a8, 0x001b_639c)?; // DNASAuth_FreeBuffers
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d0));
            if c.g[A0][0] != 0 {
                c.cycles += 2;
                c.g[A1][0] = 3;
                call_ptr(c, 0x0025_61e0, 0x001b_63b0)?; // DNASAuth_dtor
                charged = true;
            }
            if !charged {
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6fc));
            }
        }
        if no_auth || charged {
            c.cycles += 1;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6fc));
        }
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            call_ptr(c, 0x0031_7e98, 0x001b_63c4)?; // EA_MemFree
            c.sw((c.g[S0][0] as u32).wrapping_add(0x6fc), 0);
        }
        c.g[A0][0] = c.g[S0][0];
        c.cycles += 7;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001a_85d0, 0x001b_63d4)?; // NetFEState_dtor
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}


/// `void cFEStateCreateDuplicate_Init(cFEStateCreateDuplicate* s)`
/// (0x001bbd60): hash the screen name, create the screen through the engine,
/// start its thread, then fill in the name suggestions and activate.
pub fn c_fe_state_create_duplicate_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.g[V0][0] = c.lw(gpn(c, 0xc8));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x260));
        c.g[A0][0] = sx(0x0046_4c50);
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x001b_bd90);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x6d8), c.g[V0][0]);
        call_ptr(c, 0x0031_7670, 0x001b_bd90)?; // StrHash
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001b_bda4)?; // UIEngine_CreateScreen
        c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001b_bdbc)?; // UIScreen_StartThread
        }
        c.cycles += 9;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001b_c400, 0x001b_bdc4)?; // cFEStateCreateDuplicate_FillNameSuggestions
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001a_8768, 0x001b_bdcc)?; // NetFEState_Init
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}


/// `void cFEStateNetMessenger_ClearNameList(self)` (0x1a5450): drop the
/// queued player names from `self+0x6e8 .. +0x6ec`, one `lw` at a time.
///
/// Dispatcher-shaped in the original (`0x1a5470` is its loop head): it can
/// be re-entered there after an event unwind, so the port accepts every
/// resume entry and keeps its state in the registers.
pub fn cfestatenetmessenger_clear_name_list(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x001a_5450 => {
                    c.cycles += 7;
                    c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
                    c.sq(18, p(sp(c), 0x10));
                    c.sq(16, p(sp(c), 0x30));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(17, p(sp(c), 0x20));
                    c.sd(p(sp(c), 0), c.g[RA][0]);
                    pc = 0x001a_5484;
                }
                0x001a_5470 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
                    let done = c.g[A0][0] == 0;
                    c.sw(p(r32(c, S2), 0x6ec), c.g[V0][0]);
                    pc = if done { 0x001a_5484 } else { 0x001a_547c };
                }
                0x001a_547c => {
                    c.cycles += 2;
                    c.g[A1][0] = 3;
                    call_ptr(c, 0x0031_8260, 0x001a_5484)?;
                    pc = 0x001a_5484;
                }
                0x001a_5484 => {
                    c.cycles += 4;
                    c.g[V1][0] = lw_at_4(c, p(r32(c, S2), 0x6e8));
                    c.g[V0][0] = lw_at_4(c, p(r32(c, S2), 0x6ec));
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A0][0] = lw_at_4(c, (c.g[V0][0] as u32).wrapping_sub(4));
                        if event_due(c) && back_edge_unwinds(c, 0x001a_5470) {
                            return Ok(());
                        }
                        pc = 0x001a_5470;
                    } else {
                        pc = 0x001a_5494;
                    }
                }
                0x001a_5494 => {
                    c.cycles += 16;
                    c.g[S1][0] = lw_at_4(c, p(r32(c, S2), 0x6e8));
                    c.g[A2][0] = 0;
                    c.g[S0][0] = lw_at_4(c, p(r32(c, S2), 0x6ec));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, callee::MEM_MOVE, 0x001a_54ac)?;
                    c.g[V1][0] = lw_at_4(c, p(r32(c, S2), 0x6ec));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    frame_restore_at(c, 17, 0x20);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                    c.g[RA][0] = c.ld(sp(c));
                    c.sw(p(r32(c, S2), 0x6ec), c.g[V1][0]);
                    frame_restore_at(c, 16, 0x30);
                    frame_restore_at(c, 18, 0x10);
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x001a_5450, 0x001a_54d4);
                    return Ok(());
                }
            }
        }
    })
}


/// `void cFEStateOnlineCharacterSelect_ShowCharacter(self, char, opponent)`
/// (0x1b1b10): show the character rows and their stats, showing the opponent
/// char on the current slot when one is selected.
pub fn cfestateonlinecharacterselect_show_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, 19, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        loop {
            c.cycles += 2;
            c.g[A3][0] = c.g[S0][0] ^ c.g[S2][0];
            let mut plus_one = true;
            if c.g[S0][0] == c.g[S2][0] {
                c.cycles += 3;
                c.g[A1][0] = c.g[S0][0];
                let no_opponent = c.g[S1][0] == 0;
                c.g[A0][0] = c.g[S3][0];
                if no_opponent {
                    plus_one = false;
                } else {
                    c.cycles += 3;
                    call_ptr(
                        c,
                        callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR.addr,
                        0x001b_1b54,
                    )?;
                    c.g[A3][0] = c.g[S0][0] ^ c.g[S2][0];
                }
            }
            if plus_one {
                c.cycles += 1;
                c.g[A1][0] = c.g[S0][0];
            }
            c.cycles += 8;
            c.g[A3][0] = (c.g[A3][0] < 1) as u64;
            c.g[A0][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            call_ptr(
                c,
                callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS.addr,
                0x001b_1b70,
            )?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 30) as u64;
            let again = c.g[V0][0] != 0;
            c.g[RA][0] = c.ld(sp(c));
            if !again {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001b_1b38);
            }
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateLobbyMenu_PromptRoomPassword(self, a1, event)` (0x1bdd60):
/// when the child event is 0xf, open the room-password keyboard; otherwise
/// forward the event to the state base.
pub fn cfestatelobbymenu_prompt_room_password(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xf;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let not_id = c.g[A2][0] != c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if !not_id {
            c.cycles += 21;
            c.g[A2][0] = 0xf;
            callee::NET_FE_STATE_ON_CHILD_EVENT_8.call(c, 0x001b_dd80)?;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x50d0));
            call_ptr(c, callee::STR_HASH_3, 0x001b_dd8c)?;
            c.g[A3][0] = 0x4a0000;
            c.g[A2][0] = c.g[V0][0];
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1408));
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.g[T0][0] = 0;
            c.g[T1][0] = 8;
            c.g[T2][0] = 0x3f;
            c.g[RA][0] = sx(0x001b_ddb0);
            c.pc = callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID.addr;
            call_ptr(
                c,
                callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID.addr,
                0x001b_ddb0,
            )?;
            c.g[A0][0] = lw_at_4(c, p(r32(c, S0), 0x69c));
            c.g[A1][0] = 0x35;
            c.g[A2][0] = 0;
            c.g[A3][0] = 2;
            call_ptr(
                c,
                callee::C_KEYBOARD_POPUP_SET_KEY_ENABLED.addr,
                0x001b_ddc4,
            )?;
            frame.restore(c, 16, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        callee::NET_FE_STATE_ON_CHILD_EVENT_8.call(c, 0x001b_ddd4)?;
        frame.restore(c, 16, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// A `gp_11`-relative global.
#[inline(always)]
pub(crate) fn gp_11(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}


// ---------------------------------------------------------------------------
// Front-end async manager.

/// `void FEAsyncManager_FreeAll(FEAsyncManager* self)` (0x1a3c50): release the
/// five file slots.
pub fn fe_async_manager_free_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx(p(r32(c, A0), 0x110));
        frame.save(c, S2, 0x10);
        c.g[S1][0] = 0x4;
        frame.save_ra(c, 0);
        c.g[S2][0] = u64::MAX;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 0xffff_fff0)).lw(c);
            let no_data = c.g[V0][0] == 0;
            c.g[A2][0] = Addr(gp_11(c, -0x854)).lw(c);
            if !no_data {
                c.cycles += 9;
                c.g[A1][0] = Addr(r32(c, S0)).lw(c);
                c.g[V0][0] = Addr(p(r32(c, A2), 0x10d8)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V0), 0x190)).lh(c);
                c.g[V1][0] = Addr(p(r32(c, V0), 0x194)).lw(c);
                let target = r32(c, V1);
                c.g[RA][0] = sx(0x001a_3c94);
                c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                call_ptr(c, target, 0x001a_3c94)?;
                c.g[A0][0] = Addr(p(r32(c, S0), 0xffff_fff0)).lw(c);
                if c.g[A0][0] != 0 {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001a_3ca8);
                    call_ptr(c, 0x0031_7e98, 0x001a_3ca8)?;
                }
                Addr(p(r32(c, S0), 0xffff_fff0)).sw(c, 0);
                c.cycles += 2;
                Addr(p(r32(c, S0), 8)).sw(c, 0);
                Addr(r32(c, S0)).sw(c, c.g[S2][0]);
            }
            c.cycles += 3;
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(p(r32(c, S0), 0x11c));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001a_3c70);
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


// ---------------------------------------------------------------------------
// Network configuration popup.

/// `int NetConfigAuthPopup_HandleInput(NetConfigAuthPopup* self, void* obj)`
/// (0x1b66b8): when the popup waits for input (state 2), start the transfer
/// as soon as the object reports ready and advance to state 4; always
/// returns 1.
pub fn net_config_auth_popup_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x2;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V1][0] = Addr(p(r32(c, S1), 0x6e4)).lw(c);
        let waiting = c.g[V1][0] == c.g[V0][0];
        c.g[S0][0] = c.g[A1][0];
        let mut plus_one = true;
        if waiting {
            c.cycles += 7;
            c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x28)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x2c)).lw(c);
            let target = r32(c, V0);
            c.g[RA][0] = sx(0x001b_66f0);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, target, 0x001b_66f0)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(p(r32(c, S1), 0x6d0)).lw(c);
            } else {
                c.cycles += 7;
                c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V1), 0x30)).lh(c);
                c.g[V0][0] = Addr(p(r32(c, V1), 0x34)).lw(c);
                let target = r32(c, V0);
                c.g[RA][0] = sx(0x001b_670c);
                c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
                call_ptr(c, target, 0x001b_670c)?;
                if c.g[V0][0] == 0 {
                    plus_one = false;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(p(r32(c, S1), 0x6d0)).lw(c);
                }
            }
            if plus_one {
                c.cycles += 4;
                c.g[V1][0] = 0x1;
                c.g[A0][0] = 0x4;
                Addr(p(r32(c, V0), 0x14)).sw(c, c.g[V1][0]);
                Addr(p(r32(c, S1), 0x6e4)).sw(c, c.g[A0][0]);
            }
        }
        if plus_one {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 5;
        c.g[V0][0] = 0x1;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end network states.

/// `void cFEStateCreateStep1_Init(cFEStateCreateStep1* self)` (0x1b8960):
/// pick the create-screen step by the "net mode" word, then look the screen up
/// and start it.
pub fn c_fe_state_create_step1_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S1), 0x6ec)).lw(c);
        let first = c.g[V0][0] == 0;
        c.g[V0][0] = Addr(gp_11(c, -0xc8)).lw(c);
        if first {
            c.cycles += 1;
            c.g[V0][0] = sx(p(r32(c, V0), 0x2b0));
        } else {
            c.cycles += 2;
            c.g[V0][0] = sx(p(r32(c, V0), 0x260));
        }
        c.cycles += 12;
        Addr(p(r32(c, S1), 0x6d0)).sw(c, c.g[V0][0]);
        c.g[A0][0] = 0x46_0000;
        c.g[S0][0] = Addr(p(r32(c, S1), 0x10)).lw(c);
        c.g[RA][0] = sx(0x001b_89a0);
        c.g[A0][0] = sx(0x0046_4740);
        call_ptr(c, 0x0031_7670, 0x001b_89a0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001b_89b4);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001b_89b4)?;
        let missing = c.g[V0][0] == 0;
        Addr(p(r32(c, S1), 0x40)).sw(c, c.g[V0][0]);
        if !missing {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001b_89cc);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001b_89cc)?;
        }
        c.cycles += 7;
        c.g[RA][0] = sx(0x001b_89d4);
        c.g[A0][0] = c.g[S1][0];
        callee::NET_FE_STATE_INIT.call(c, 0x001b_89d4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateCreateStep1_DetectChanges(cFEStateCreateStep1* self)`
/// (0x1ba358): compare the two account names and ids; set the matching
/// changed-flags.
pub fn c_fe_state_create_step1_detect_changes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 0x6d0)).lw(c);
        c.g[A1][0] = Addr(p(r32(c, S0), 0x6d4)).lw(c);
        c.g[RA][0] = sx(0x001b_a378);
        c.g[A0][0] = Addr(p(r32(c, V0), 4)).lw(c);
        call_ptr(c, 0x0041_65a8, 0x001b_a378)?;
        let same = c.g[V0][0] == 0;
        c.g[V0][0] = 0x1;
        if !same {
            c.cycles += 2;
            c.g[V1][0] = Addr(p(r32(c, S0), 0x6d0)).lw(c);
            Addr(p(r32(c, V1), 0x44)).sw(c, c.g[V0][0]);
        }
        c.cycles += 6;
        c.g[V0][0] = Addr(p(r32(c, S0), 0x6d0)).lw(c);
        c.g[A1][0] = Addr(p(r32(c, S0), 0x6dc)).lw(c);
        c.g[RA][0] = sx(0x001b_a398);
        c.g[A0][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
        call_ptr(c, 0x0041_65a8, 0x001b_a398)?;
        let same = c.g[V0][0] == 0;
        c.g[V0][0] = 0x1;
        if !same {
            c.cycles += 2;
            c.g[V1][0] = Addr(p(r32(c, S0), 0x6d0)).lw(c);
            Addr(p(r32(c, V1), 0x4c)).sw(c, c.g[V0][0]);
        }
        c.cycles += 5;
        c.g[A1][0] = Addr(p(r32(c, S0), 0x6d0)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, S0), 0x6e0)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, A1), 0x38)).lw(c);
        let differs = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = 0x1;
        if !differs {
            c.cycles += 4;
            c.g[V1][0] = Addr(p(r32(c, S0), 0x6e4)).lw(c);
            c.g[V0][0] = Addr(p(r32(c, A1), 0x3c)).lw(c);
            if c.g[V0][0] != c.g[V1][0] {
                c.g[V0][0] = 0x1;
                c.cycles += 1;
                Addr(p(r32(c, A1), 0x48)).sw(c, c.g[V0][0]);
            }
        } else {
            c.cycles += 1;
            Addr(p(r32(c, A1), 0x48)).sw(c, c.g[V0][0]);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateCreateDuplicate_OnChildEvent(cFEStateCreateDuplicate* self,
/// ChildEvent* ev, int, int)` (0x1bc6a0): dispatch the child's event word to
/// the account, error, created-callbacks.
pub fn c_fe_state_create_duplicate_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x81f;
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(r32(c, A1), 0x18)).lw(c);
        let is_account = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = (c.g[V1][0] < 0x820) as u64;
        if is_account {
            c.cycles += 4;
            c.g[RA][0] = sx(0x001b_c6ec);
            callee::FE_STATE_CREATE_DUPLICATE_ON_SAVE_ACCOUNT_ANSWER.call(c, 0x001b_c6ec)?;
        } else {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = 0x82c;
                let is_created = c.g[V1][0] == c.g[V0][0];
                if is_created {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001b_c70c);
                    callee::FE_STATE_CREATE_DUPLICATE_ON_CREATED_POPUP_EVENT.call(c, 0x001b_c70c)?;
                } else {
                    c.cycles += 2;
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001b_c71c);
                    callee::NET_FE_STATE_ON_CHILD_EVENT_9.call(c, 0x001b_c71c)?;
                }
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x80e;
                let is_error = c.g[V1][0] == c.g[V0][0];
                if is_error {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001b_c6fc);
                    callee::FE_STATE_CREATE_DUPLICATE_ON_ERROR_POPUP_EVENT.call(c, 0x001b_c6fc)?;
                } else {
                    c.cycles += 2;
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001b_c71c);
                    callee::NET_FE_STATE_ON_CHILD_EVENT_9.call(c, 0x001b_c71c)?;
                }
            }
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        let target = r32(c, RA);
        c.g[SP][0] = sx(p(r32(c, SP), 0x10));
        c.pc = target;
        Ok(())
    })
}


/// `int cFEStateLobbyMenu_OnWidgetNotify(cFEState* self, void* obj, int msg,
/// int)` (0x1bdc18): apply lobby-room updates for the two child kinds and
/// always returns 0x101.
pub fn c_fe_state_lobby_menu_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(r32(c, S1), 0x6d4)).lw(c);
        if c.g[S0][0] != c.g[V0][0] {
            c.g[V0][0] = Addr(p(r32(c, S1), 0x6d0)).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x5;
            let wrong = c.g[A2][0] != c.g[V0][0];
            c.g[V0][0] = 0x101;
            if !wrong {
                c.cycles += 7;
                c.g[RA][0] = sx(0x001b_dc50);
                c.g[A1][0] = Addr(p(r32(c, S0), 0xa0)).lw(c);
                callee::FE_STATE_LOBBY_MENU_SHOW_ROOM_TAB.call(c, 0x001b_dc50)?;
                c.g[A1][0] = Addr(p(r32(c, S0), 0xa0)).lw(c);
                c.g[RA][0] = sx(0x001b_dc5c);
                c.g[A0][0] = c.g[S1][0];
                callee::FE_STATE_LOBBY_MENU_UPDATE_PLAYERS_IN_ROOM.call(c, 0x001b_dc5c)?;
                c.g[V0][0] = 0x101;
            }
        }
        c.cycles += 2;
        let other = c.g[S0][0] != c.g[V0][0];
        c.g[V0][0] = 0x101;
        if !other {
            c.cycles += 3;
            c.g[V0][0] = 0x4;
            let wrong = c.g[A2][0] != c.g[V0][0];
            c.g[V0][0] = 0x101;
            if !wrong {
                c.cycles += 5;
                Addr(p(r32(c, S1), 0x778)).sw(c, c.g[A3][0]);
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x001b_dc88);
                c.g[A1][0] = Addr(p(r32(c, A0), 0x774)).lw(c);
                callee::FE_STATE_LOBBY_MENU_REFRESH_PLAYER_LIST.call(c, 0x001b_dc88)?;
                c.g[V0][0] = 0x101;
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


/// `void cFEStateOnlineAgreement_Update(cFEStateOnlineAgreement* self)`
/// (0x1c0a80): while the agreement has queued ids, look the text object up
/// and set the next id; then update the base state.
pub fn c_fe_state_online_agreement_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 0x6d8)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, S0), 0x6d4)).lw(c);
        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x6d0)).lw(c);
            let none = c.g[V0][0] == 0;
            c.g[A0][0] = 0x46_0000;
            if !none {
                c.cycles += 8;
                c.g[RA][0] = sx(0x001c_0ab8);
                c.g[A0][0] = sx(0x0046_5710);
                call_ptr(c, 0x0031_7670, 0x001c_0ab8)?;
                c.g[A0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
                c.g[RA][0] = sx(0x001c_0ac4);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001c_0ac4)?;
                c.g[A0][0] = c.g[V0][0];
                if c.g[A0][0] != 0 {
                    c.cycles += 8;
                    c.g[V0][0] = Addr(p(r32(c, S0), 0x6d8)).lw(c);
                    c.g[A1][0] = Addr(p(r32(c, S0), 0x6d0)).lw(c);
                    c.g[V1][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                    c.g[V1][0] = sx(p(r32(c, V1), r32(c, A1)));
                    Addr(p(r32(c, S0), 0x6d8)).sw(c, c.g[V0][0]);
                    c.g[RA][0] = sx(0x001c_0af0);
                    c.g[A1][0] = Addr(r32(c, V1)).lw(c);
                    call_ptr(c, 0x003a_1f18, 0x001c_0af0)?;
                }
            }
        }
        c.cycles += 6;
        c.g[RA][0] = sx(0x001c_0af8);
        c.g[A0][0] = c.g[S0][0];
        callee::NET_FE_STATE_UPDATE_3.call(c, 0x001c_0af8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `lbu` of `base + off`.
#[inline(always)]
pub(crate) fn lbu_at(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr(at_3(base as u32, off)).lbu(c)
}


/// `void FEAsyncManager_RequestPeakMap(FEAsyncManager* self, int index,
/// u32 map, int state)` (0x1a3768): format the peak-map path into the request
/// slot and arm the async state machine.
pub fn fe_async_manager_request_peak_map(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        open(c, 0x50);
        let sp0 = spv(c);
        c.g[V0][0] = 0x440000;
        c.sq(16, at_3(sp0, 0x40));
        c.g[V0][0] = sx(at_3(c.g[V0][0] as u32, 0x1258));
        c.sq(19, at_3(sp0, 0x10));
        c.g[S0][0] = 0x11c;
        c.g[S3][0] = c.g[A2][0];
        c.sq(17, at_3(sp0, 0x30));
        c.mult(0, 16, 19, 16);
        c.sq(18, at_3(sp0, 0x20));
        c.sd(sp0, c.g[RA][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[A1][0] = sx(at_3(c.g[A1][0] as u32, c.g[V0][0] as u32));
        c.g[S2][0] = c.g[A3][0];
        c.g[A2][0] = lw_at_5(c, c.g[A1][0], 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = 0x4a0000;
        c.g[A3][0] = 0x4a0000;
        c.g[A0][0] = sx(at_3(c.g[S1][0] as u32, c.g[S0][0] as u32));
        c.g[A3][0] = sx(at_3(c.g[A3][0] as u32, 0x1a00));
        c.g[A1][0] = sx(at_3(c.g[A1][0] as u32, 0x19f8));
        call_ptr(c, callee::MSG_PRINTF, 0x001a_37c0)?;
        let rec = sx(at_3(c.g[S1][0] as u32, c.g[S0][0] as u32));
        c.g[S0][0] = rec;
        c.g[A0][0] = c.g[S1][0];
        c.sw(at_3(rec as u32, 0x108), c.g[S2][0]);
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = 1;
        callee::FE_ASYNC_SET_STATE.call(c, 0x001a_37d8)?;
        c.lq(16, at_3(sp0, 0x40));
        c.lq(17, at_3(sp0, 0x30));
        c.lq(18, at_3(sp0, 0x20));
        c.lq(19, at_3(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_3(sp0, 0x50));
        c.pc = ra;
        Ok(())
    })
}


/// `void cFEStateNetMessenger_SetRowName(cFEStateNetMessenger* self, u32 row,
/// char* text, u32 id)` (0x1a6618): label a net-messenger list row.
pub fn c_fe_state_net_messenger_set_row_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        open(c, 0x60);
        let sp0 = spv(c);
        c.g[V1][0] = c.g[A1][0];
        c.sq(16, at_3(sp0, 0x50));
        c.sq(17, at_3(sp0, 0x40));
        c.g[S0][0] = c.g[A0][0];
        c.sq(18, at_3(sp0, 0x30));
        c.g[S1][0] = c.g[A2][0];
        c.sd(at_3(sp0, 0x20), c.g[RA][0]);
        c.g[V0][0] = lw_at_5(c, c.g[S0][0], 0x40);
        let nolist = c.g[V0][0] == 0;
        c.g[S2][0] = c.g[A3][0];
        if !nolist {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V1][0] < 0xfu64) as u64;
            let bad_row = c.g[V0][0] == 0;
            c.g[A1][0] = 0x460000;
            if !bad_row {
                c.cycles += 12;
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.g[V1][0];
                c.g[A1][0] = sx(at_3(c.g[A1][0] as u32, 0x15a0));
                call_ptr(c, callee::MSG_PRINTF, 0x001a_6660)?;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, callee::STR_HASH_4, 0x001a_6668)?;
                c.g[A0][0] = lw_at_5(c, c.g[S0][0], 0x40);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, callee::UI_SCREEN_FIND_OBJECT_3, 0x001a_6674)?;
                c.g[S0][0] = c.g[V0][0];
                let nohit = c.g[S0][0] == 0;
                c.g[A1][0] = c.g[S1][0];
                if !nohit {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, callee::UI_TEXT_SET_STRING_3, 0x001a_6688)?;
                    c.sw(at_3(c.g[S0][0] as u32, 0x18), c.g[S2][0]);
                }
            }
        }
        c.cycles += 6;
        c.lq(16, at_3(sp0, 0x50));
        c.lq(17, at_3(sp0, 0x40));
        c.lq(18, at_3(sp0, 0x30));
        c.g[RA][0] = c.ld(at_3(sp0, 0x20));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_3(sp0, 0x60));
        c.pc = ra;
        Ok(())
    })
}


/// `int cFEStateCreateStep1_IsEmailFormatValid(cFEStateCreateStep1* self,
/// char* email)` (0x1b9cd8): true when the address has both an `@` and a
/// `.` (the game's own, lax check).
pub fn c_fe_state_create_step1_is_email_format_valid(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        open(c, 0x40);
        let sp0 = spv(c);
        c.sq(17, at_3(sp0, 0x20));
        c.g[S1][0] = c.g[A1][0];
        c.sq(16, at_3(sp0, 0x30));
        c.sq(18, at_3(sp0, 0x10));
        c.g[A0][0] = c.g[S1][0];
        c.sd(sp0, c.g[RA][0]);
        c.g[S2][0] = 0;
        call_ptr(c, callee::STRLEN_3, 0x001b_9cfc)?;
        c.g[S0][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        let empty = (c.g[A0][0] as i64) <= 0;
        c.g[V1][0] = 0;
        if !empty {
            c.cycles += 3;
            c.g[A2][0] = 0x40;
            c.g[A1][0] = 1;
            c.g[V0][0] = sx(at_3(c.g[S1][0] as u32, c.g[V1][0] as u32));
            loop {
                c.cycles += 3;
                let at_p = c.g[V0][0] as u32;
                c.g[V0][0] = lb_at(c, at_p as u64, 0);
                let not_at = c.g[V0][0] != c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] ^ 0x2e;
                if !not_at {
                    // An `@`: mark it and skip the `.` test for this byte.
                    c.cycles += 2;
                    c.g[S2][0] = 1;
                } else {
                    c.cycles += 1;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[A1][0];
                    }
                }
                c.cycles += 4;
                c.g[V1][0] = sx(at_3(c.g[V1][0] as u32, 1));
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx(at_3(c.g[S1][0] as u32, c.g[V1][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001b_9d18);
                }
            }
        }
        c.cycles += 2;
        let no_at = c.g[S2][0] == 0;
        c.g[V0][0] = 0;
        if !no_at {
            c.cycles += 1;
            c.g[V0][0] = (0u64 < c.g[S0][0]) as u64;
        }
        c.cycles += 6;
        c.lq(16, at_3(sp0, 0x30));
        c.lq(17, at_3(sp0, 0x20));
        c.lq(18, at_3(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_3(sp0, 0x40));
        c.pc = ra;
        Ok(())
    })
}


/// `void cFEStateCreateStep2_SetYearSelection(cFEStateCreateStep2* self)`
/// (0x1bb3b0): label the year list box with the current year and select the
/// rider's year.
pub fn c_fe_state_create_step2_set_year_selection(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        open(c, 0x40);
        let sp0 = spv(c);
        c.sq(17, at_3(sp0, 0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, at_3(sp0, 0x30));
        c.g[A0][0] = 0x4a0000;
        c.sd(at_3(sp0, 0x10), c.g[RA][0]);
        c.g[A0][0] = sx(at_3(c.g[A0][0] as u32, 0x1be8));
        call_ptr(c, callee::STR_HASH_4, 0x001b_b3d0)?;
        c.g[A0][0] = lw_at_5(c, c.g[S1][0], 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_FIND_OBJECT_3, 0x001b_b3dc)?;
        c.g[S0][0] = c.g[V0][0];
        let nohit = c.g[S0][0] == 0;
        c.g[A0][0] = c.g[SP][0];
        if !nohit {
            c.cycles += 16;
            c.g[V0][0] = lw_at_5(c, c.g[S1][0], 0x6d0);
            c.g[A1][0] = 0x4a0000;
            c.g[A1][0] = sx(at_3(c.g[A1][0] as u32, 0x12c0));
            c.g[A2][0] = lw_at_5(c, c.g[V0][0], 0x30);
            call_ptr(c, callee::MSG_PRINTF, 0x001b_b3fc)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[S0][0];
            c.g[A3][0] = 0;
            c.g[A2][0] = c.g[SP][0];
            call_ptr(c, callee::UI_LISTBOX_SET_ITEM_TEXT, 0x001b_b410)?;
            c.g[V0][0] = lw_at_5(c, c.g[S1][0], 0x6d0);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = lbu_at(c, c.g[V0][0], 0x30);
            c.g[A1][0] = sx(at_3(c.g[A1][0] as u32, 0xffff_ff94));
            c.g[A1][0] = c.g[A1][0] & 0xff;
            call_ptr(c, callee::SET_ITEM_YEAR, 0x001b_b428)?;
        }
        c.cycles += 5;
        c.lq(16, at_3(sp0, 0x30));
        c.lq(17, at_3(sp0, 0x20));
        c.g[RA][0] = c.ld(at_3(sp0, 0x10));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_3(sp0, 0x40));
        c.pc = ra;
        Ok(())
    })
}


/// A `gp_12`-relative global.
#[inline(always)]
pub(crate) fn gp_12(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}


/// `void cFEStateNetMessenger_OnChildEvent(self, msg, a2)` (0x1a4c58):
/// dispatch a child message by its kind: 0x809/0x80b/0x80c go to the
/// messenger's own virtual handlers (slots 0x10c/0x114/0x11c), anything else
/// to [`NetFEState_OnChildEvent`].
pub fn cfe_state_net_messenger_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        let self_ = abi::arg(c, 0);
        let msg = abi::arg(c, 1);
        let a2 = abi::arg64(c, 2);
        frame.save_ra(c, 0);
        let kind = c.lw(p(msg, 0x18));
        c.g[A3][0] = sx(self_);
        let mut call_default = false;
        // The dispatch: 0x80b skips straight to its slot block (4 cycles),
        // 0x809 costs 3+2 in the dispatch and 3 in its slot block, 0x80c
        // costs 3+3 and 2, and the default paths 3+2+2 or 3+3+2.
        let (disp, target_off) = if kind == 0x80b {
            c.cycles += 4;
            (0x110u32, 0x114u32)
        } else if kind < 0x80c {
            if kind == 0x809 {
                c.cycles += 8;
                (0x108u32, 0x10cu32)
            } else {
                c.cycles += 7;
                call_default = true;
                (0u32, 0u32)
            }
        } else if kind == 0x80c {
            c.cycles += 8;
            (0x118u32, 0x11cu32)
        } else {
            c.cycles += 8;
            call_default = true;
            (0u32, 0u32)
        };
        if call_default {
            c.cycles += 3;
            c.g[A0][0] = c.g[A3][0];
            c.g[A1][0] = sx(msg);
            c.g[A2][0] = a2;
            call_ptr(c, callee::NET_FE_STATE_ON_CHILD_EVENT_10, 0x001a_4cdc)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw(p(r32(c, A3), 8));
            c.g[A0][0] = c.lh(at(c, V0, disp));
            c.g[V1][0] = c.lw(at(c, V0, target_off));
            c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
            let target = r32(c, V1);
            call_ptr(c, target, 0x001a_4ccc)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// `void* NetFEState_v26(self, msg)` (0x1a89b0): unless the message's kind is
/// one of the messenger's own (0x31, 0x33, 0x4d, 0x4e) or the FE library is
/// busy, drop the message (virtual `delete(3)`), show the disconnected popup
/// and return 0; otherwise pass the message through.
pub fn net_fe_state_v26(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        let msg = abi::arg(c, 1);
        c.g[A2][0] = sx(msg);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(self_);
        c.g[V0][0] = c.lw(p(self_, 0x6a8));
        let idle = c.g[V0][0] == 0;
        c.g[V1][0] = c.lw(p(msg, 0xc));
        let mut pass = idle;
        let mut busy = false;
        if !idle {
            c.cycles += 3;
            c.g[V0][0] = 0x31;
            let t = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x33;
            pass = t;
            if !pass {
                c.cycles += 2;
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x4d;
                pass = t;
                if !pass {
                    c.cycles += 2;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x4e;
                    pass = t;
                    if !pass {
                        c.cycles += 2;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = c.lw(gp_12(c, -0xc8));
                        pass = t;
                        if !pass {
                            c.cycles += 3;
                            c.g[V1][0] = c.lw(r32(c, V0));
                            busy = c.g[V1][0] != 0;
                            pass = false;
                            if busy {
                                c.g[V0][0] = c.g[A2][0];
                            } else {
                                // delete msg through its vtable, then the popup
                                c.cycles += 2;
                                c.g[A1][0] = 3;
                                if c.g[A2][0] != 0 {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw(at(c, A2, 8));
                                    c.g[A0][0] = c.lh(at(c, V0, 8));
                                    c.g[V1][0] = c.lw(at(c, V0, 0xc));
                                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                                    let target = r32(c, V1);
                                    call_ptr(c, target, 0x001a_8a1c)?;
                                }
                                c.cycles += 4;
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(
                                    c,
                                    callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP_2,
                                    0x001a_8a24,
                                )?;
                                c.g[V0][0] = 0;
                            }
                        }
                    }
                }
            }
        }
        if pass {
            c.cycles += 1;
            c.g[V0][0] = c.g[A2][0];
        }
        let _ = busy;
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int cFEStateOnlineCharacterSelect_TransitionIn(self, int a1)` (0x1b1068):
/// if entering, show the local character, ask the BE interface for
/// interface 7, and if a remote player is present also show their character;
/// then fill the player info panel and clear the "entering" flag.
pub fn cfe_state_online_character_select_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        let entering = abi::arg64(c, 1);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(self_);
        frame.save_ra(c, 0);
        if entering != 0 {
            c.cycles += 15;
            c.g[A0][0] = c.lw(p(self_, 0x6d0));
            call_ptr(c, callee::CFS_GET_CHARACTER_INFO, 0x001b_1088)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = 0;
            call_ptr(c, callee::CFS_SHOW_CHARACTER, 0x001b_1098)?;
            c.g[V0][0] = c.lw(gp_12(c, -0x848));
            c.g[A1][0] = 0x7;
            c.g[A0][0] = c.lw(at(c, V0, 0x78));
            call_ptr(c, callee::BE_GET_INTERFACE, 0x001b_10a8)?;
            c.g[V1][0] = 0x0053_0000;
            c.g[S1][0] = sx(p(r32(c, V1), 0x4b30));
            c.g[A1][0] = c.lw(at(c, S1, 0x32c));
            let absent = (c.g[A1][0] as i64) < 0;
            if !absent {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::CFS_SHOW_OPPONENT_CHAR, 0x001b_10c4)?;
                c.g[A1][0] = c.lw(at(c, S1, 0x32c));
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = 1;
                call_ptr(c, callee::CFS_SHOW_CHARACTER, 0x001b_10d4)?;
            }
        }
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::CFS_FILL_PLAYER_INFO, 0x001b_10dc)?;
        c.sw(at(c, S0, 0x6dc), 0);
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateCreateStep2_SubmitAccount(self, int a1)` (0x1ba9f8): fill in
/// and show the network popup (a "connecting" message), then either hide the
/// lobby overlay or show it.
pub fn cfe_state_create_step2_submit_account(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        let self_ = abi::arg(c, 0);
        let a1 = abi::arg64(c, 1);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = sx(self_);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S2][0] = a1;
        call_ptr(c, callee::NET_FE_STATE_GET_POPUP_2, 0x001b_aa18)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x0046_0000;
        c.sw(at(c, S0, 0xbc), 0);
        c.g[A0][0] = sx(p(r32(c, A0), 0x4a38));
        call_ptr(c, callee::GET_HASH_VALUE32, 0x001b_aa2c)?;
        c.g[V1][0] = 3;
        c.g[A2][0] = 0x820;
        c.sw(at(c, S0, 0x164), 0);
        c.g[A0][0] = c.g[S1][0];
        c.sw(at(c, S0, 0xc4), c.g[V0][0]);
        c.g[A1][0] = c.g[S0][0];
        c.sw(at(c, S0, 0xc0), c.g[V1][0]);
        c.sw(at(c, S0, 0x150), c.g[A2][0]);
        call_ptr(c, callee::NET_FE_STATE_SHOW_POPUP, 0x001b_aa50)?;
        let hidden = c.g[S2][0] == 0;
        c.g[A0][0] = c.lw(gp_12(c, -0xc8));
        if hidden {
            c.cycles += 3;
            c.g[A1][0] = sx(p(r32(c, A0), 0x260));
            call_ptr(c, callee::LOBBY_MAN_SHOW, 0x001b_aa70)?;
        } else {
            c.cycles += 4;
            c.g[A1][0] = 0;
            call_ptr(c, callee::LOBBY_MAN_HIDE, 0x001b_aa60)?;
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void NetFEState_ShowMaxBuddiesPopup(self)` (0x1abed8): fill and show the
/// "maximum buddies" network popup.
pub fn net_fe_state_show_max_buddies_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x40);
        let self_ = abi::arg(c, 0);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(self_);
        call_ptr(c, callee::NET_FE_STATE_GET_POPUP_2, 0x001a_bef4)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x0046_0000;
        c.g[V0][0] = 1;
        c.g[A0][0] = sx(p(r32(c, A0), 0x1618));
        c.sw(at(c, S0, 0xbc), c.g[V0][0]);
        c.g[S1][0] = sx(p(r32(c, S0), 0xbc));
        call_ptr(c, callee::GET_HASH_VALUE32, 0x001a_bf10)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID, 0x001a_bf20)?;
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(p(r32(c, A0), 0x2b00));
        call_ptr(c, callee::GET_HASH_VALUE32, 0x001a_bf2c)?;
        c.g[A1][0] = 0x845;
        c.g[V1][0] = 3;
        c.g[A2][0] = 6;
        c.sw(at(c, S0, 0x150), c.g[A1][0]);
        c.sw(at(c, S0, 0xc4), c.g[V0][0]);
        c.g[A0][0] = c.g[S2][0];
        c.sw(at(c, S0, 0xc0), c.g[V1][0]);
        c.g[A1][0] = c.g[S0][0];
        c.sw(at(c, S0, 0x14c), c.g[A2][0]);
        call_ptr(c, callee::NET_FE_STATE_SHOW_POPUP, 0x001a_bf54)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEUseExistingAccount_Init(self)` (0x1ac0b8): create the account
/// screen, play it, clear the two global strings and re-init the net state.
pub fn c_fe_use_existing_account_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = sx(self_);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(p(r32(c, A0), 0x2b20));
        c.g[S0][0] = c.lw(at(c, S1, 0x10));
        call_ptr(c, callee::GET_HASH_VALUE32, 0x001a_c0dc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME_2, 0x001a_c0f0)?;
        let no_screen = c.g[V0][0] == 0;
        c.sw(at(c, S1, 0x40), c.g[V0][0]);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, callee::UI_SCREEN_PLAY_FRAME_2, 0x001a_c108)?;
        }
        c.cycles += 17;
        c.g[A0][0] = c.lw(gp_12(c, -0xc8));
        c.g[S0][0] = 0x004a_0000;
        c.g[S0][0] = sx(p(r32(c, S0), 0x1408));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x48));
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, callee::CBE_STRING_CTOR, 0x001a_c120)?;
        c.g[A0][0] = c.lw(gp_12(c, -0xc8));
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x4c));
        call_ptr(c, callee::CBE_STRING_CTOR, 0x001a_c130)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::NET_FE_STATE_INIT_2, 0x001a_c138)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// A `gp_13`-relative global.
#[inline(always)]
pub(crate) fn gp_13(c: &Cpu, off: i32) -> u32 {
    c.g[GP][0] as u32 + off as u32
}


// ---------------------------------------------------------------------------
// Front-end / online state handlers.

/// `void func_001AD930(void* self, void* child, int event)` (0x1ad930): when
/// the save-account popup closes with event 0xF, allocate and post a
/// `cFEStateNetAccountMem` (0xB10 bytes) to the screen's object queue;
/// otherwise forward the child event.
pub fn cfe_state_select_persona_on_save_account_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xf;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        if c.g[A2][0] == c.g[V0][0] {
            c.cycles += 25;
            c.g[A2][0] = 0xf;
            callee::NET_FE_STATE_ON_CHILD_EVENT_11.call(c, 0x001a_d950)?;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = 0xb10;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2d10));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, table_callee::EA_MEM_ALLOC_NAMED, 0x001a_d968)?;
            let mem = c.g[V0][0];
            c.g[A1][0] = lw_4(c, S0, 0x10);
            c.g[A0][0] = mem;
            c.g[T0][0] = 0;
            c.g[T1][0] = 0;
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = 0;
            callee::NET_ACCOUNT_MEM_CTOR.call(c, 0x001a_d984)?;
            c.g[V1][0] = 0x832;
            c.g[A1][0] = c.g[V0][0];
            Addr(c.g[V0][0] as u32).off(0x18).sw(c, c.g[V1][0]);
            c.g[V0][0] = lw_4(c, S0, 0x40);
            c.g[V1][0] = lw_4(c, V0, 0xd0);
            c.g[A0][0] = lw_4(c, V1, 0x10);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, table_callee::UI_OBJECT_PUSH, 0x001a_d9a4)?;
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::NET_FE_STATE_ON_CHILD_EVENT_11.call(c, 0x001a_d9b4)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateOnlineCharacterSelect_Init(self)` (0x1b0fb0): add the screen
/// by hash, play its first frame and show the stat line of the 30 characters.
pub fn cfe_state_online_character_select_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x460000;
        Addr(c.g[S1][0] as u32).off(0x6e0).sw(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3800));
        c.g[S0][0] = lw_4(c, S1, 0x10);
        call_ptr(c, table_callee::GET_HASH_VALUE32, 0x001b_0fd8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(
            c,
            table_callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME,
            0x001b_0fec,
        )?;
        let screen = c.g[V0][0];
        Addr(c.g[S1][0] as u32).off(0x40).sw(c, screen);
        if screen != 0 {
            c.cycles += 4;
            c.g[A0][0] = screen;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, table_callee::UI_SCREEN_PLAY_FRAME, 0x001b_1004)?;
        }
        c.cycles += 3;
        c.g[S0][0] = 0;
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 8;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 1;
            c.g[A3][0] = 0;
            callee::SHOW_CHAR_STATS.call(c, 0x001b_1020)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 30) as u64;
            c.g[A1][0] = c.g[S0][0];
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001b_1010);
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


/// `void cFEStateNetConfig_Init(self)` (0x1b40d0): add the screen by hash,
/// optionally drop the network adapter interface, then start the card scan
/// and the network config module.
pub fn cfe_state_net_config_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3ed0));
        c.g[S0][0] = lw_4(c, S1, 0x10);
        call_ptr(c, table_callee::GET_HASH_VALUE32, 0x001b_40f4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(
            c,
            table_callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME,
            0x001b_4108,
        )?;
        let screen = c.g[V0][0];
        Addr(c.g[S1][0] as u32).off(0x40).sw(c, screen);
        if screen != 0 {
            c.cycles += 4;
            c.g[A0][0] = screen;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, table_callee::UI_SCREEN_PLAY_FRAME, 0x001b_4120)?;
        }
        // A loaded network adapter gets dropped before the scan restarts.
        c.cycles += 3;
        c.g[V0][0] = Addr(gp_13(c, -0x204)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, table_callee::UI_ENGINE_GET_INTERFACE, 0x001b_4134)?;
            c.g[A0][0] = c.g[S1][0];
        }
        c.cycles += 11;
        c.g[A1][0] = 0;
        callee::START_CARD_SCAN.call(c, 0x001b_4140)?;
        call_ptr(c, 0x0028_b180, 0x001b_4148)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0028_fc38, 0x001b_4150)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateNetConfig_OnRetryPopupEvent(self, child, event)` (0x1b4c40):
/// the checking-card popup's outcome selects whether the config state is
/// flagged complete (`0x70C`/`0x708`) or the child event is forwarded.
pub fn cfe_state_net_config_on_retry_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x10;
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        let event = c.g[A2][0];
        let at_1b4ca8 = event == c.g[V0][0];
        frame.save_ra(c, 0);
        // 1 = case 0x0F, 2 = fall through the first switch arm,
        // 3 = the 0x10/0x14 arm, 4 = the default (forward) arm.
        let mut stage = 0u8;
        if at_1b4ca8 {
            stage = 3;
        } else {
            c.cycles += 3;
            c.g[V0][0] = (event < 0x11) as u64;
            let below = c.g[V0][0] == 0;
            c.g[V0][0] = 0xf;
            if below {
                stage = 2;
            } else if event == c.g[V0][0] {
                Addr(gp_13(c, -0x1570)).sw(c, 0);
                stage = 1;
            } else {
                c.cycles += 2;
                stage = 4;
            }
        }
        if stage == 2 {
            c.cycles += 3;
            c.g[V0][0] = 0x14;
            if event == c.g[V0][0] {
                stage = 3;
            } else {
                c.cycles += 2;
                stage = 4;
            }
        }
        match stage {
            1 => {
                c.cycles += 7;
                c.g[S0][0] = 1;
                Addr(c.g[S1][0] as u32).off(0x70c).sw(c, c.g[S0][0]);
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 1;
                callee::SHOW_CHECKING_CARD_POPUP.call(c, 0x001b_4ca0)?;
                Addr(c.g[S1][0] as u32).off(0x708).sw(c, c.g[S0][0]);
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
            3 => {
                c.cycles += 2;
                c.g[A0][0] = c.g[S1][0];
                callee::NET_FE_STATE_ON_CHILD_EVENT_11.call(c, 0x001b_4cc0)?;
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
            _ => {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                callee::NET_FE_STATE_ON_CHILD_EVENT_11.call(c, 0x001b_4cb0)?;
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


/// `void cFEStateCreateStep1_TransitionIn(self, enabled)` (0x1b8a08): refresh
/// the account text fields and, if the state owns a create-step popup, tell
/// it whether every field is filled.
pub fn cfe_state_create_step1_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        let enabled = c.g[A1][0] != 0;
        frame.save_ra(c, 0);
        if enabled {
            c.cycles += 17;
            callee::CREATE_STEP1_SET_ACCOUNT_NAME.call(c, 0x001b_8a28)?;
            c.g[A0][0] = c.g[S1][0];
            callee::CREATE_STEP1_SET_PASSWORD.call(c, 0x001b_8a30)?;
            c.g[A0][0] = c.g[S1][0];
            callee::CREATE_STEP1_SET_CONFIRM_PASSWORD.call(c, 0x001b_8a38)?;
            c.g[A0][0] = c.g[S1][0];
            callee::CREATE_STEP1_SET_EMAIL.call(c, 0x001b_8a40)?;
            c.g[A0][0] = c.g[S1][0];
            callee::CREATE_STEP1_SET_SHARE_INFO.call(c, 0x001b_8a48)?;
            c.g[A0][0] = c.g[S1][0];
            callee::CREATE_STEP1_SET_INFORM.call(c, 0x001b_8a50)?;
            c.g[A0][0] = c.g[S1][0];
            callee::CREATE_STEP1_BACKUP_ACCOUNT.call(c, 0x001b_8a58)?;
            c.g[V0][0] = lw_4(c, S1, 0x6f0);
            c.g[A0][0] = c.g[S1][0];
            if c.g[V0][0] != 0 {
                c.cycles += 8;
                c.g[S0][0] = lw_4(c, V0, 0x8);
                c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x40).lh(c);
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                callee::CREATE_STEP1_ALL_FIELDS_FILLED.call(c, 0x001b_8a74)?;
                c.g[V1][0] = lw_4(c, S0, 0x44);
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[V0][0] ^ 1;
                call_ptr(c, c.g[V1][0] as u32, 0x001b_8a84)?;
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateLogin_OnChildEvent(self, child, event)` (0x1bf6c0): dispatch
/// the login state's child events (account-memory popup, disconnect popup,
/// keyboard popup) and forward everything else.
pub fn cfe_state_login_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x82c;
        let event = Addr(c.g[A1][0] as u32).off(0x18).lw(c);
        c.g[V1][0] = event;
        let is_82c = event == c.g[V0][0];
        c.g[A3][0] = c.g[A0][0];
        // 0 = default (forward), 1 = 0x82C, 2 = 0x80E, 3 = 0x848.
        let mut stage = 0u8;
        if is_82c {
            stage = 1;
        } else {
            c.cycles += 3;
            c.g[V0][0] = (event < 0x82d) as u64;
            let above = c.g[V0][0] == 0;
            c.g[V0][0] = 0x80e;
            if above {
                c.cycles += 3;
                c.g[V0][0] = 0x848;
                if event == c.g[V0][0] {
                    stage = 3;
                } else {
                    c.cycles += 2;
                    stage = 0;
                }
            } else if event == c.g[V0][0] {
                c.g[V0][0] = lw_4(c, A3, 0x8);
                stage = 2;
            } else {
                c.cycles += 2;
                stage = 0;
            }
        }
        match stage {
            1 => {
                c.cycles += 4;
                c.g[A0][0] = c.g[A3][0];
                callee::LOGIN_ON_ACCOUNT_MEM_POPUP.call(c, 0x001b_f728)?;
            }
            2 => {
                c.cycles += 6;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x108).lh(c);
                c.g[V1][0] = lw_4(c, V0, 0x10c);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V1][0] as u32, 0x001b_f718)?;
            }
            3 => {
                c.cycles += 4;
                c.g[A0][0] = c.g[A3][0];
                callee::LOGIN_ON_DISCONNECT_POPUP.call(c, 0x001b_f738)?;
            }
            _ => {
                c.cycles += 3;
                c.g[A0][0] = c.g[A3][0];
                callee::NET_FE_STATE_ON_CHILD_EVENT_11.call(c, 0x001b_f748)?;
            }
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// `sll x, n` (32-bit, sign-extended).
#[inline(always)]
pub(crate) fn shl32(x: u64, n: u32) -> u64 {
    sx((x as u32) << n)
}


/// `sra x, n`.
#[inline(always)]
pub(crate) fn sra32(x: u64, n: u32) -> u64 {
    sx(((x as u32) as i32 >> n) as u32)
}


/// `addu x, y` (32-bit, sign-extended).
#[inline(always)]
pub(crate) fn add32(x: u64, y: u64) -> u64 {
    sx((x as u32).wrapping_add(y as u32))
}


/// `cFEStateNetMessenger_ShowLookingForSelf` (0x1a4d90): open the
/// "looking for yourself" popup, configure its two option labels and show it
/// with a fixed 0x808 option mask.
pub fn fe_net_messenger_show_looking_for_self(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        let this = r(c, A0);
        c.g[S3][0] = sx(this);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);

        c.g[RA][0] = sx(0x001a_4db4);
        c.g[S2][0] = 3;
        callee::NET_FE_STATE_GET_POPUP_3.call(c, 0x001a_4db4)?;
        let popup = r(c, V0);
        c.g[S0][0] = sx(popup);
        c.g[A0][0] = 0x0046_0000;
        c.g[V0][0] = 1;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x1660));
        Addr(p(popup, 0xbc)).sw(c, c.g[V0][0]);
        c.g[RA][0] = sx(0x001a_4dd0);
        c.g[S1][0] = sx(popup.wrapping_add(0xbc));
        call_ptr(c, callee::FE_STRING_HASH, 0x001a_4dd0)?;
        c.g[S2][0] = 3;
        Addr(p(popup, 0xc8)).sw(c, c.g[S2][0]);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x1618));
        c.g[RA][0] = sx(0x001a_4de4);
        Addr(p(popup, 0xcc)).sw(c, c.g[V0][0]);
        call_ptr(c, callee::FE_STRING_HASH, 0x001a_4de4)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001a_4df4);
        c.g[A1][0] = 0;
        callee::POPUP_CFG_SET_OPTION_TEXT_ID.call(c, 0x001a_4df4)?;
        Addr(p(popup, 0x14c)).sw(c, c.g[S2][0]);
        c.g[V0][0] = 0x808;
        Addr(p(popup, 0x150)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x001a_4e0c);
        c.g[A1][0] = c.g[S0][0];
        callee::NET_FE_STATE_SHOW_POPUP_2.call(c, 0x001a_4e0c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateNetMessenger_HasName` (0x1a56d0): true when the name string is
/// one of the entries in the vector at `+0x6e8` (case-insensitive).
pub fn fe_net_messenger_has_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(this, 0x6ec)).lw(c);
        c.g[V1][0] = Addr(p(this, 0x6e8)).lw(c);
        c.g[V0][0] = sx(r(c, V0).wrapping_sub(r(c, V1)));
        c.g[V0][0] = sra32(c.g[V0][0], 2);
        let empty = c.g[V0][0] == 0;
        let mut found = false;
        if !empty {
            c.cycles += 1;
            c.g[V0][0] = Addr(p(this, 0x6e8)).lw(c);
            loop {
                c.cycles += 8;
                c.g[A0][0] = shl32(c.g[S0][0], 2);
                c.g[A1][0] = c.g[S2][0];
                c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
                c.g[V1][0] = Addr(r(c, V0)).lw(c);
                c.g[RA][0] = sx(0x001a_5720);
                c.g[A0][0] = Addr(r(c, V1)).lw(c);
                call_ptr(c, callee::STRCASECMP, 0x001a_5720)?;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(p(this, 0x6ec)).lw(c);
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    found = true;
                    break;
                }
                c.cycles += 7;
                c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                c.g[V1][0] = Addr(p(this, 0x6e8)).lw(c);
                c.g[V0][0] = sx(r(c, V0).wrapping_sub(r(c, V1)));
                c.g[V0][0] = sra32(c.g[V0][0], 2);
                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(p(this, 0x6e8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x001a_5708);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0;
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


/// `cFEStateNetMessenger_FindNameStr` (0x1a5800): index of the name string in
/// the vector at `+0x6e8` (case-insensitive), or -1.
pub fn fe_net_messenger_find_name_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(this, 0x6ec)).lw(c);
        c.g[V1][0] = Addr(p(this, 0x6e8)).lw(c);
        c.g[V0][0] = sx(r(c, V0).wrapping_sub(r(c, V1)));
        c.g[V0][0] = sra32(c.g[V0][0], 2);
        let empty = c.g[V0][0] == 0;
        let mut found = false;
        if !empty {
            c.cycles += 1;
            c.g[V0][0] = Addr(p(this, 0x6e8)).lw(c);
            loop {
                c.cycles += 8;
                c.g[A0][0] = shl32(c.g[S0][0], 2);
                c.g[A1][0] = Addr(r(c, S2)).lw(c);
                c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
                c.g[V1][0] = Addr(r(c, V0)).lw(c);
                c.g[RA][0] = sx(0x001a_5850);
                c.g[A0][0] = Addr(r(c, V1)).lw(c);
                call_ptr(c, callee::STRCASECMP, 0x001a_5850)?;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(p(this, 0x6ec)).lw(c);
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    found = true;
                    break;
                }
                c.cycles += 7;
                c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                c.g[V1][0] = Addr(p(this, 0x6e8)).lw(c);
                c.g[V0][0] = sx(r(c, V0).wrapping_sub(r(c, V1)));
                c.g[V0][0] = sra32(c.g[V0][0], 2);
                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(p(this, 0x6e8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x001a_5838);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
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


/// `NetFEState_OnKeyboardPopupEvent` (0x1a94e0): keyboard-popup child events
/// 0x15/0x16 dispatch to the text field's virtual handler (0x16 also clears
/// the pending popup flag), everything else to the default child handler.
pub fn net_fe_state_on_keyboard_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x15;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        if c.g[A2][0] == c.g[V0][0] {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001a_9524);
            c.g[A2][0] = 0x15;
            callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT.call(c, 0x001a_9524)?;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x16;
            let is16 = c.g[A2][0] == c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            if is16 {
                c.cycles += 12;
                c.g[V0][0] = Addr(p(this, 8)).lw(c);
                c.g[A0][0] = Addr(p(r(c, V0), 0x100)).lh(c);
                c.g[V1][0] = Addr(p(r(c, V0), 0x104)).lw(c);
                let target = r(c, V1);
                c.g[RA][0] = sx(0x001a_9540);
                c.g[A0][0] = sx(this.wrapping_add(r(c, A0)));
                call_ptr(c, target, 0x001a_9540)?;
                Addr(p(this, 0x69c)).sw(c, 0);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x001a_9554);
                c.g[A2][0] = 0x16;
                callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT.call(c, 0x001a_9554)?;
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 2;
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a_9564);
                c.g[A1][0] = c.g[S1][0];
                callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT.call(c, 0x001a_9564)?;
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


/// `cFEStateChatRoom_TransitionIn` (0x1b3490): hide the chat room's scroll
/// parts and run each player list entry's virtual "transition in" handler,
/// then select the local player.
pub fn fe_state_chat_room_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save(c, S2, 0x10);
        let has_arg = c.g[A1][0] != 0;
        frame.save_ra(c, 0);
        if has_arg {
            c.cycles += 9;
            c.g[A0][0] = Addr(p(this, 0x6ec)).lw(c);
            c.g[A1][0] = 0;
            c.g[S0][0] = sx(this.wrapping_add(0x6fc));
            c.g[RA][0] = sx(0x001b_34c0);
            c.g[S2][0] = 1;
            call_ptr(c, callee::UI_SCROLL_SET_PARTS_VISIBLE, 0x001b_34c0)?;
            c.g[A0][0] = Addr(p(this, 0x6f4)).lw(c);
            c.g[RA][0] = sx(0x001b_34cc);
            c.g[A1][0] = 0;
            call_ptr(c, callee::UI_SCROLL_SET_PARTS_VISIBLE, 0x001b_34cc)?;
            loop {
                c.cycles += 3;
                let entry = Addr(r(c, S0)).lw(c);
                c.g[A2][0] = entry;
                c.g[A1][0] = 1;
                if entry != 0 {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p(entry as u32, 8)).lw(c);
                    c.g[A0][0] = Addr(p(r(c, V0), 0x38)).lh(c);
                    c.g[V1][0] = Addr(p(r(c, V0), 0x3c)).lw(c);
                    let target = r(c, V1);
                    c.g[RA][0] = sx(0x001b_34f0);
                    c.g[A0][0] = sx((entry as u32).wrapping_add(r(c, A0)));
                    call_ptr(c, target, 0x001b_34f0)?;
                }
                c.cycles += 3;
                c.g[S2][0] = sx(r(c, S2).wrapping_sub(1));
                let more = (c.g[S2][0] as i64) >= 0;
                c.g[S0][0] = sx(r(c, S0).wrapping_add(4));
                if !more {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(gp(c, -0xc8)).lw(c);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001b_350c);
                    c.g[A1][0] = Addr(p(r(c, V0), 0x80)).lw(c);
                    callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME.call(c, 0x001b_350c)?;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001b_34d0);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateOnlineAgreement_Init` (0x1c04a8): add the agreement screen by its
/// hash name, start its thread and mark the terms text object dirty.
pub fn fe_state_online_agreement_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x5568));
        c.g[RA][0] = sx(0x001c_04cc);
        c.g[S0][0] = Addr(p(this, 0x10)).lw(c);
        call_ptr(c, callee::FE_STRING_HASH, 0x001c_04cc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001c_04e0);
        c.g[A3][0] = 0;
        call_ptr(c, callee::UI_ADD_SCREEN_BY_HASH_NAME, 0x001c_04e0)?;
        let screen = r(c, V0);
        let none = c.g[V0][0] == 0;
        Addr(p(this, 0x40)).sw(c, c.g[V0][0]);
        if !none {
            c.cycles += 6;
            c.g[A0][0] = sx(screen);
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001c_04f8);
            c.g[A2][0] = 0;
            call_ptr(c, callee::UI_SCREEN_PLAY_FRAME_3, 0x001c_04f8)?;
            c.g[RA][0] = sx(0x001c_0500);
            c.g[A0][0] = c.g[S1][0];
            callee::LOAD_AGREEMENT_TERMS.call(c, 0x001c_0500)?;
        }
        c.cycles += 9;
        c.g[A0][0] = 0x0046_0000;
        c.g[RA][0] = sx(0x001c_050c);
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x5580));
        call_ptr(c, callee::FE_STRING_HASH, 0x001c_050c)?;
        c.g[A0][0] = Addr(p(this, 0x40)).lw(c);
        c.g[RA][0] = sx(0x001c_0518);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_GET_OBJECT_BY_HASH, 0x001c_0518)?;
        let obj = r(c, V0);
        c.g[V1][0] = sx(obj);
        let missing = c.g[V1][0] == 0;
        frame.restore(c, S0, 0x20);
        if !missing {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(obj, 0x90)).lw(c);
            c.g[V0][0] = c.g[V0][0] | 8;
            Addr(p(obj, 0x90)).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateOnlineAgreement_v25` (0x1c0ea8): populate the agreement dialog's
/// terms text and the two option rows.
pub fn fe_state_online_agreement_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 2;
        c.g[RA][0] = sx(0x001c_0ec4);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::TOOLTIP_MANAGER_SET_STATE, 0x001c_0ec4)?;
        c.g[A0][0] = 0x0046_0000;
        c.g[RA][0] = sx(0x001c_0ed0);
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x5778));
        call_ptr(c, callee::FE_STRING_HASH, 0x001c_0ed0)?;
        c.g[A0][0] = 0x0046_0000;
        Addr(p(r(c, S0), 0x54)).sw(c, c.g[V0][0]);
        c.g[RA][0] = sx(0x001c_0ee0);
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x5798));
        call_ptr(c, callee::FE_STRING_HASH, 0x001c_0ee0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001c_0ef0);
        c.g[A1][0] = 0;
        call_ptr(c, callee::TOOLTIP_MANAGER_SET_TEXT, 0x001c_0ef0)?;
        c.g[A0][0] = 0x0046_0000;
        c.g[RA][0] = sx(0x001c_0efc);
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x57c0));
        call_ptr(c, callee::FE_STRING_HASH, 0x001c_0efc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x001c_0f10);
        c.g[A3][0] = 8;
        call_ptr(c, callee::TOOLTIP_MANAGER_HIDE_POPUP, 0x001c_0f10)?;
        c.g[A0][0] = 0x0046_0000;
        c.g[RA][0] = sx(0x001c_0f1c);
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x14e8));
        call_ptr(c, callee::FE_STRING_HASH, 0x001c_0f1c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        c.g[RA][0] = sx(0x001c_0f30);
        c.g[A3][0] = 1;
        call_ptr(c, callee::TOOLTIP_MANAGER_HIDE_POPUP, 0x001c_0f30)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Network front end.

/// `void NetFEState_QueueNetEvent(NetFEState* self, NetEvent* event)`
/// (0x1a8ae8): turn the event object into a queued entry through its virtual
/// conversion at vtable slot 0x10 (returning the payload), allocate a 12-byte
/// queue node from `self+0x6cc`'s list and push it on both the node list and
/// the pending list.
pub fn net_fe_state_queue_net_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        c.g[S1][0] = c.g[A0][0];
        let no_event = c.g[A1][0] == 0;
        frame.save_ra(c, 0x20);
        if !no_event {
            c.cycles += 8;
            c.g[V1][0] = lw(c, p(c.g[A1][0] as u32, 4));
            c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x10)).lh(c);
            c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x14));
            c.g[V0][0] = sx(c.g[V0][0] as u32);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V0][0] as u32, 0x001a_8b14)?;
            c.g[S0][0] = c.g[V0][0];
            let skip = c.g[S0][0] == 0;
            c.g[A1][0] = 0x0046_0000;
            if !skip {
                c.cycles += 11;
                c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x6cc));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3218));
                c.g[A0][0] = 0xc;
                sw_3(c, p(c.g[SP][0] as u32, 0x10), c.g[V0][0]);
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x001a_8b3c)?; // EA_MemAllocNamed
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
                if c.g[V0][0] != 0 {
                    sw_3(c, p(c.g[A1][0] as u32, 8), c.g[S0][0]);
                }
                c.cycles += 9;
                c.g[V0][0] = lw(c, p(c.g[SP][0] as u32, 0x10));
                sw_3(c, c.g[A1][0] as u32, c.g[V0][0]);
                c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 4));
                sw_3(c, p(c.g[A1][0] as u32, 4), c.g[V1][0]);
                c.g[A0][0] = lw(c, p(c.g[V0][0] as u32, 4));
                sw_3(c, c.g[A0][0] as u32, c.g[A1][0]);
                c.g[V0][0] = lw(c, p(c.g[SP][0] as u32, 0x10));
                sw_3(c, p(c.g[V0][0] as u32, 4), c.g[A1][0]);
                sw_3(c, c.g[SP][0] as u32, c.g[A1][0]);
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}


/// `void NetFEState_FillYesNoList(NetFEState* self, UIListBox* box, int mode)`
/// (0x1a9a18): mark the box as having a background (bit 0x80 of `+0x14`), add
/// the "yes" and "no" string ids (the first as the initial item) and select
/// the mode, then refresh the box's parent list.
pub fn net_fe_state_fill_yes_no_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = 0xffff_ffff_ffff_fffe;
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x0046_0000;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x14));
        c.g[V0][0] &= c.g[V1][0];
        c.g[V0][0] |= 0x80;
        sw_3(c, p(c.g[S0][0] as u32, 0x14), c.g[V0][0]);
        call_ptr(c, 0x0031_7670, 0x001a_9a50)?; // StrHash
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 1;
        call_ptr(c, 0x0039_a568, 0x001a_9a60)?; // UIListBox_AddItemStrId
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
        call_ptr(c, 0x0031_7670, 0x001a_9a6c)?; // StrHash
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_a568, 0x001a_9a7c)?; // UIListBox_AddItemStrId
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = (c.g[S1][0] < 1) as u64;
        call_ptr(c, 0x0039_a7a8, 0x001a_9a88)?; // UIListBox_SetSelection
        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 8));
        c.g[A1][0] = 1;
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x48)).lh(c);
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x4c));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V1][0] as u32, 0x001a_9aa0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void NetFEState_FillOnOffList(NetFEState* self, UIListBox* box, int mode)`
/// (0x1a9ab8): the same fill as `NetFEState_FillYesNoList` with the "on"/"off"
/// string ids.
pub fn net_fe_state_fill_on_off_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = 0xffff_ffff_ffff_fffe;
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x0046_0000;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fd8));
        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x14));
        c.g[V0][0] &= c.g[V1][0];
        c.g[V0][0] |= 0x80;
        sw_3(c, p(c.g[S0][0] as u32, 0x14), c.g[V0][0]);
        call_ptr(c, 0x0031_7670, 0x001a_9af0)?; // StrHash
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 1;
        call_ptr(c, 0x0039_a568, 0x001a_9b00)?; // UIListBox_AddItemStrId
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20c0));
        call_ptr(c, 0x0031_7670, 0x001a_9b0c)?; // StrHash
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_a568, 0x001a_9b1c)?; // UIListBox_AddItemStrId
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = (c.g[S1][0] < 1) as u64;
        call_ptr(c, 0x0039_a7a8, 0x001a_9b28)?; // UIListBox_SetSelection
        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 8));
        c.g[A1][0] = 1;
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x48)).lh(c);
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x4c));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V1][0] as u32, 0x001a_9b40)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// Front-end widgets.

/// `void cFEStateNetSSXWorld_OnWidgetCreated(cFEState*, void* widget)`
/// (0x1c1e08): compare the widget's name (`widget+0x38`) against the four
/// known names at `0x465a80/90/a8/c0`; the matching one selects the page
/// stored at `widget+0x18` (0..3), otherwise the field is left alone.
pub fn c_fe_state_net_ssx_world_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x0046_0000;
        frame.save(c, S1, 0x10);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5a80));
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = lw(c, p(c.g[S1][0] as u32, 0x38));
        call_ptr(c, 0x0031_7670, 0x001c_1e2c)?; // StrHash
        let different = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x0046_0000;
        if !different {
            c.cycles += 2;
            sw_3(c, p(c.g[S1][0] as u32, 0x18), 0);
            c.cycles += 1;
        } else {
            c.cycles += 5;
            c.g[S0][0] = lw(c, p(c.g[S1][0] as u32, 0x38));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5a90));
            call_ptr(c, 0x0031_7670, 0x001c_1e48)?;
            if c.g[S0][0] != c.g[V0][0] {
                c.g[A0][0] = 0x0046_0000;
                c.cycles += 5;
                c.g[S0][0] = lw(c, p(c.g[S1][0] as u32, 0x38));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5aa8));
                call_ptr(c, 0x0031_7670, 0x001c_1e64)?;
                if c.g[S0][0] != c.g[V0][0] {
                    c.g[A0][0] = 0x0046_0000;
                    c.cycles += 5;
                    c.g[S0][0] = lw(c, p(c.g[S1][0] as u32, 0x38));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5ac0));
                    call_ptr(c, 0x0031_7670, 0x001c_1e80)?;
                    if c.g[S0][0] == c.g[V0][0] {
                        c.cycles += 1;
                        c.g[V0][0] = 3;
                        c.cycles += 1;
                        sw_3(c, p(c.g[S1][0] as u32, 0x18), c.g[V0][0]);
                        c.cycles += 1;
                    }
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 2;
                    c.cycles += 1;
                    sw_3(c, p(c.g[S1][0] as u32, 0x18), c.g[V0][0]);
                    c.cycles += 1;
                }
            } else {
                c.cycles += 2;
                c.g[V0][0] = 1;
                c.cycles += 1;
                sw_3(c, p(c.g[S1][0] as u32, 0x18), c.g[V0][0]);
                c.cycles += 1;
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void NetFEState_OnMemCardLoginPopupEvent(NetFEState*, void* event, int id)`
/// (0x1a9438): on the login-popup result event (0x16), create a login state
/// through the vtable at `self+8`, switch to it and forward the event;
/// otherwise forward the event to the base handler.
pub fn net_fe_state_on_mem_card_login_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x16;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let other = c.g[A2][0] != c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        if other {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x001a_93d0, 0x001a_94c0)?; // NetFEState_OnChildEventDefault
        } else {
            c.cycles += 23;
            c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 8));
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x100)).lh(c);
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x104));
            c.g[V1][0] = sx(c.g[V1][0] as u32);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x001a_946c)?;
            c.g[A1][0] = 0x0046_0000;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1ca0));
            c.g[A0][0] = 0x6d4;
            call_ptr(c, 0x0031_7d70, 0x001a_9484)?; // EA_MemAllocNamed
            c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0x10));
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x001b_ee30, 0x001a_9490)?; // cFEStateLogin_ctor
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001a_8a40, 0x001a_949c)?; // NetFEState_GotoNextState
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 0x16;
            call_ptr(c, 0x001a_93d0, 0x001a_94ac)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// Read-message popup.

/// `void cFEStateReadMessagePopup_AddSentMessage(cFEState*, char* text)`
/// (0x1a7ed8): build a 16-byte message (sender from `gp_14-0xc8`, the text and a
/// timestamp string) and add it to the popup's message ring.
pub fn c_fe_state_read_message_popup_add_sent_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0x2000_0000;
        frame.save(c, S2, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        c.g[S3][0] = c.g[A0][0];
        c.g[A1][0] = 0x0046_0000;
        frame.save(c, S0, 0x40);
        c.g[A0][0] = 0x10;
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a08));
        call_ptr(c, 0x0031_7d70, 0x001a_7f10)?; // EA_MemAllocNamed
        c.g[A0][0] = lw(c, gp_14(c, -0xc8));
        c.g[V1][0] = 0x0047_0000;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2740));
        c.g[A1][0] = lw(c, p(c.g[A0][0] as u32, 0x80));
        c.g[S0][0] = 0x004a_0000;
        sw_3(c, p(c.g[S1][0] as u32, 0xc), c.g[V1][0]);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1408));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0031_82d8, 0x001a_7f38)?; // cBXString_ctor_cstr
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
        call_ptr(c, 0x0031_82d8, 0x001a_7f44)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(8));
        call_ptr(c, 0x0031_82d8, 0x001a_7f50)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001a_7dd8, 0x001a_7f5c)?; // AddMessage
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `lw_5 reg + off`.
#[inline(always)]
pub(crate) fn lw_5(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(at(c, reg, off)).lw(c)
}


/// `sw_4 reg + off, value`.
#[inline(always)]
pub(crate) fn sw_4(c: &mut Cpu, reg: usize, off: u32, v: u64) {
    Addr(at(c, reg, off)).sw(c, v);
}


// ---------------------------------------------------------------------------
// Front-end / online state handlers.

/// `void cFEUseExistingAccount_OnChildEvent(self, child)` (0x1ac920): the
/// child event's id at `child+0x18` selects one of the four account popup
/// handlers (0x80E abort, 0x80F email, 0x81E login, 0x82C save answer);
/// anything else is forwarded to `NetFEState_OnChildEvent`.
pub fn cfe_use_existing_account_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x80f;
        frame.save_ra(c, 0);
        let event = Addr(abi::arg(c, 1)).off(0x18).lw(c);
        c.g[V1][0] = event;
        let below = (event < 0x810) as u64;
        c.g[V0][0] = below;
        if event == 0x80f {
            c.cycles += 4;
            callee::FE_ON_EMAIL_POPUP_EVENT.call(c, 0x001a_c984)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0x80e;
            if below != 0 {
                c.cycles += 2;
                if event == 0x80e {
                    c.cycles += 4;
                    callee::FE_ON_ABORT_POPUP_EVENT.call(c, 0x001a_c974)?;
                    frame.restore_ra(c, 0);
                } else {
                    c.cycles += 2;
                    c.cycles += 3;
                    callee::NET_FE_STATE_ON_CHILD_EVENT_12.call(c, 0x001a_c9b4)?;
                    frame.restore_ra(c, 0);
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x81e;
                let login = event == 0x81e;
                c.g[V0][0] = 0x82c;
                if login {
                    c.cycles += 4;
                    callee::FE_ON_LOGIN_POPUP_EVENT.call(c, 0x001a_c994)?;
                    frame.restore_ra(c, 0);
                } else {
                    c.cycles += 2;
                    if event == 0x82c {
                        c.cycles += 4;
                        callee::FE_ON_SAVE_ACCOUNT_ANSWER.call(c, 0x001a_c9a4)?;
                        frame.restore_ra(c, 0);
                    } else {
                        c.cycles += 2;
                        c.cycles += 3;
                        callee::NET_FE_STATE_ON_CHILD_EVENT_12.call(c, 0x001a_c9b4)?;
                        frame.restore_ra(c, 0);
                    }
                }
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateChallengePopup_ShowSetOptions(self)` (0x1af688): reset the
/// settings lock, refresh the popup's unicode strings and re-apply the
/// settings-locked state.
pub fn cfe_state_challenge_popup_show_set_options(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x0046_3468;
        Addr(this).off(0x6d0).sw(c, 0);
        call_ptr(c, table::STR_HASH, 0x001a_f6ac)?;
        c.g[A0][0] = lw_5(c, S1, 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, table::UI_SCREEN_FIND_OBJECT, 0x001a_f6b8)?;
        c.g[S0][0] = c.g[V0][0];
        let found = c.g[S0][0] == 0;
        c.g[A0][0] = 0x0046_0000;
        if !found {
            c.cycles += 20;
            c.g[A0][0] = 0x0046_3498;
            call_ptr(c, table::STR_HASH, 0x001a_f6cc)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, table::UI_TEXT_SET_TEXT_ID, 0x001a_f6d8)?;
            c.g[A0][0] = 0x0046_31c8;
            call_ptr(c, table::STR_HASH, 0x001a_f6e4)?;
            c.g[A0][0] = lw_5(c, S1, 0x40);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, table::UI_SCREEN_FIND_OBJECT, 0x001a_f6f0)?;
            c.g[V1][0] = lw_5(c, V0, 0x8);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(at(c, V1, 0x40)).lh(c);
            c.g[A2][0] = lw_5(c, V1, 0x44);
            let target = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001a_f708)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            callee::CHALLENGE_POPUP_SET_SETTINGS_LOCKED.call(c, 0x001a_f714)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateNetConfig* cFEStateNetConfig_ctor(this)` (0x1b3f78): base
/// constructor, the state's vtable and lock fields, the mem-card handler.
pub fn cfe_state_net_config_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        callee::NET_FE_STATE_CTOR_11.call(c, 0x001b_3f90)?;
        c.g[V1][0] = 0x0046_8b00;
        sw_4(c, S0, 0x6d0, 0);
        c.g[V0][0] = 0x31;
        sw_4(c, S0, 0x6d4, 0);
        sw_4(c, S0, 0x6d8, 0);
        sw_4(c, S0, 0x6e4, 0);
        sw_4(c, S0, 0x6e8, 0);
        sw_4(c, S0, 0x6f4, 0);
        sw_4(c, S0, 0x6f8, 0);
        sw_4(c, S0, 0x6fc, 0);
        sw_4(c, S0, 0x700, 0);
        sw_4(c, S0, 0x704, 0);
        sw_4(c, S0, 0x708, 0);
        sw_4(c, S0, 0x710, 0);
        sw_4(c, S0, 0x714, 0);
        sw_4(c, S0, 0x718, 0);
        sw_4(c, S0, 0x6a8, 0);
        sw_4(c, S0, 0x8, c.g[V1][0]);
        sw_4(c, S0, 0xc, c.g[V0][0]);
        call_ptr(c, table::PS2_MEM_CARD_CREATE, 0x001b_3fe4)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        Addr(gp(c, -0x1578)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(at(c, V1, 0x10)).lh(c);
        c.g[A1][0] = lw_5(c, V1, 0x14);
        let target = c.g[A1][0] as u32;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001b_3ffc)?;
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = c.g[S0][0];
        sw_4(c, S0, 0x6e0, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateCreateStep1_BackupAccountInfo(self)` (0x1ba178): copy the
/// three account strings out of `self->account` (or release them when
/// already backed up) and clear the account's flags.
pub fn cfe_state_create_step1_backup_account_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        let already = lw_5(c, S0, 0x6ec) != 0;
        c.g[A0][0] = sx(this.wrapping_add(0x6d4));
        if already {
            c.cycles += 6;
            c.g[A0][0] = sx(this.wrapping_add(0x6d4));
            call_ptr(c, table::BXSTRING_RELEASE, 0x001b_a1e0)?;
            c.g[A0][0] = sx(this.wrapping_add(0x6d8));
            call_ptr(c, table::BXSTRING_RELEASE, 0x001b_a1e8)?;
            c.g[A0][0] = sx(this.wrapping_add(0x6dc));
            call_ptr(c, table::BXSTRING_RELEASE, 0x001b_a1f0)?;
        } else {
            c.cycles += 17;
            c.g[A1][0] = sx((lw_5(c, S0, 0x6d0) as u32).wrapping_add(0x4));
            call_ptr(c, table::BXSTRING_ASSIGN, 0x001b_a1a0)?;
            c.g[A1][0] = sx((lw_5(c, S0, 0x6d0) as u32).wrapping_add(0x8));
            c.g[A0][0] = sx(this.wrapping_add(0x6d8));
            call_ptr(c, table::BXSTRING_ASSIGN, 0x001b_a1b0)?;
            c.g[A1][0] = sx((lw_5(c, S0, 0x6d0) as u32).wrapping_add(0xc));
            c.g[A0][0] = sx(this.wrapping_add(0x6dc));
            call_ptr(c, table::BXSTRING_ASSIGN, 0x001b_a1c0)?;
            c.g[A0][0] = lw_5(c, S0, 0x6d0);
            c.g[V0][0] = lw_5(c, A0, 0x38);
            sw_4(c, S0, 0x6e0, c.g[V0][0]);
            c.g[V1][0] = lw_5(c, A0, 0x3c);
            sw_4(c, S0, 0x6e4, c.g[V1][0]);
        }
        c.cycles += 10;
        c.g[V0][0] = lw_5(c, S0, 0x6d0);
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        sw_4(c, V0, 0x4c, 0);
        c.g[V1][0] = lw_5(c, S0, 0x6d0);
        sw_4(c, V1, 0x44, 0);
        c.g[V0][0] = lw_5(c, S0, 0x6d0);
        frame.restore(c, S0, 0x10);
        sw_4(c, V0, 0x48, 0);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = c.g[RA][0] as u32;
        return Ok(());
    })
}


/// `void NetFEState_RefreshBuddyActionPopup(self, a1, a2)` (0x1ab3d0): if the
/// buddy popup is open and selected, rebuild it from the stored popup.
pub fn net_fe_state_refresh_buddy_action_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        c.g[V0][0] = lw_5(c, S0, 0x1c);
        c.g[V0][0] &= 6;
        let inactive = c.g[V0][0] == 0;
        if inactive {
            refresh_popup_return(c, frame, 1);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A0][0] = lw_5(c, S0, 0x6a0);
        if c.g[A0][0] == 0 {
            refresh_popup_return(c, frame, 0);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = lw_5(c, S0, 0x6c4);
        let no_selection = c.g[V0][0] == 0;
        c.g[V0][0] = 0x80b;
        if no_selection {
            refresh_popup_return(c, frame, 1);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x18).lw(c);
        let known = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x80c;
        if !known {
            c.cycles += 2;
            if c.g[V1][0] != c.g[V0][0] {
                refresh_popup_return(c, frame, 0);
                return Ok(());
            }
        }
        c.cycles += 5;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x1d).lbu(c);
        c.g[V0][0] &= 0x3f;
        c.g[V0][0] = (c.g[V0][0] < 6) as u64;
        if c.g[V0][0] == 0 {
            refresh_popup_return(c, frame, 0);
            return Ok(());
        }
        c.cycles += 8;
        callee::POPUP_CONFIRM_GET_SELECTION.call(c, 0x001a_b444)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = lw_5(c, A0, 0x6a0);
        c.g[A3][0] = c.g[S2][0];
        c.g[T0][0] = c.g[V0][0];
        callee::SETUP_BUDDY_ACTION_POPUP.call(c, 0x001a_b45c)?;
        refresh_popup_return(c, frame, 1);
        Ok(())
    })
}


/// The shared epilogue of `NetFEState_RefreshBuddyActionPopup`: the inline
/// `lq s0` plus the function's `+5` epilogue.
pub(crate) fn refresh_popup_return(c: &mut Cpu, frame: Frame, before: u64) {
    c.cycles += before;
    frame.restore(c, S0, 0x30);
    c.cycles += 5;
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `u32 cFEUseExistingAccount_CheckEmail(self)` (0x1ac448): if the account
/// email string is empty, configure and show the login popup; otherwise
/// report already handled.
pub fn cfe_use_existing_account_check_email(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        let this = abi::arg(c, 0);
        c.g[S2][0] = sx(this);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_5(c, S2, 0x6dc);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_sub(8)).lw(c);
        let handled = c.g[V1][0] != 0;
        c.g[V0][0] = 1;
        if !handled {
            c.cycles += 25;
            c.g[A0][0] = c.g[S2][0];
            callee::NET_FE_STATE_GET_POPUP_4.call(c, 0x001a_c478)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 0x0046_2bf0;
            c.g[V0][0] = 1;
            sw_4(c, S0, 0xbc, c.g[V0][0]);
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
            call_ptr(c, table::STR_HASH, 0x001a_c494)?;
            c.g[V1][0] = 3;
            c.g[A0][0] = 0x0046_1618;
            sw_4(c, S0, 0xc0, c.g[V1][0]);
            sw_4(c, S0, 0xc4, c.g[V0][0]);
            call_ptr(c, table::STR_HASH, 0x001a_c4ac)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID_2.call(c, 0x001a_c4bc)?;
            c.g[V0][0] = 0x80d;
            c.g[A0][0] = c.g[S2][0];
            sw_4(c, S0, 0x150, c.g[V0][0]);
            c.g[A1][0] = c.g[S0][0];
            callee::NET_FE_STATE_SHOW_POPUP_3.call(c, 0x001a_c4d0)?;
            c.g[V0][0] = 0;
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


/// A `gp_15`-relative global.
#[inline(always)]
pub(crate) fn gp_15(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}


// ---------------------------------------------------------------------------
// Front end.

/// `void cFEStateChallengeReceivedPopup_OnWidgetAction(self, a1, int msg)`
/// (0x1b0538): route a widget message. Kind 5 shows the waiting text (popup
/// state 0xb) or accepts (state 0xc), kind 7 is ignored, anything else is
/// handed to the base net state handler.
pub fn cfe_challenge_received_popup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let msg = c.g[A2][0];
        if c.g[A1][0] == 0 {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001a_8918, 0x001b_05cc)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 5;
            let kind5 = msg == c.g[V0][0];
            c.g[V0][0] = 7;
            if kind5 {
                let this = r32(c, S0);
                c.cycles += 4;
                c.g[A1][0] = Addr(p(this, 0x18)).lw(c);
                c.g[V0][0] = 0xb;
                let state_b = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = 0xc;
                if state_b {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(this, 0x6d4)).lw(c);
                    if c.g[V0][0] == 0 {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x001b_0c08, 0x001b_059c)?;
                        c.g[A0][0] = Addr(gp_15(c, -0xc8)).lw(c);
                        c.g[A2][0] = Addr(p(this, 0x14)).lb(c);
                        c.g[A1][0] = Addr(p(r32(c, A0), 0x98)).lw(c);
                        call_ptr(c, 0x0025_fb58, 0x001b_05ac)?;
                    }
                } else {
                    c.cycles += 2;
                    if c.g[A1][0] == c.g[V0][0] {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x001b_0e10, 0x001b_05bc)?;
                    }
                }
            } else {
                c.cycles += 2;
                if msg != c.g[V0][0] {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x001a_8918, 0x001b_05cc)?;
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateOnlineCharacterSelect_ShowOpponentChar(self, int player)`
/// (0x1b1760): look the opponent's name text up on the screen and set it from
/// the character's display name, then flag the text visible.
pub fn cfe_online_character_select_show_opponent_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(r32(c, S2), 0x40)).lw(c);
        let have_screen = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        if have_screen {
            c.cycles += 26;
            let this = r32(c, S2);
            call_ptr(c, 0x0014_dc80, 0x001b_178c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 2;
            call_ptr(c, 0x0014_dd58, 0x001b_1798)?;
            c.g[A2][0] = 0;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_eec8, 0x001b_17a8)?;
            c.g[A0][0] = sx(0x0046_3830);
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, 0x0031_7670, 0x001b_17b8)?;
            c.g[A0][0] = Addr(p(this, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001b_17c4)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
            let setter = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, setter, 0x001b_17e0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x003a_0c50, 0x001b_17ec)?;
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


/// `void cFEStateCreateStep2_FillYearList(box, value)` (0x1bb798): format the
/// value into the list box's year label and select its row.
pub fn cfe_create_step2_fill_year_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_a670, 0x001bb7b8)?;
        let this = r32(c, S0);
        c.g[V0][0] = Addr(p(this, 0x14)).lw(c);
        c.g[V1][0] = sx(0xffff_fffe);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x67;
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | 0x80;
        Addr(p(this, 0x14)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0039_a8d8, 0x001bb7d8)?;
        c.g[A1][0] = sx(0x004a_12c0);
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, 0x0041_7828, 0x001bb7ec)?;
        c.g[A3][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[SP][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0039_a7f0, 0x001bb800)?;
        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(0x6c));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0] & 0xff;
        call_ptr(c, 0x0039_a7a8, 0x001bb810)?;
        c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
        let setter = r32(c, V1);
        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
        call_ptr(c, setter, 0x001bb828)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateOnlineMainMenu_Init(self)` (0x1bfbf8): create the state's
/// screen, play its first frame, and update the net/menu systems.
pub fn cfe_online_main_menu_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(0x0046_53f0);
        c.g[S0][0] = Addr(p(r32(c, S1), 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001bfc1c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001bfc30)?;
        let no_screen = c.g[V0][0] == 0;
        Addr(p(r32(c, S1), 0x40)).sw(c, c.g[V0][0]);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001bfc48)?;
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(gp_15(c, -0xc8)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        let connected = c.g[V1][0] != 0;
        c.g[V0][0] = Addr(gp_15(c, -0x204)).lw(c);
        if !connected {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x001a_9930, 0x001bfc60)?;
            c.g[V0][0] = Addr(gp_15(c, -0x204)).lw(c);
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, 0x0025_6c50, 0x001bfc74)?;
        }
        c.cycles += 10;
        call_ptr(c, 0x0028_b180, 0x001bfc7c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0028_f140, 0x001bfc88)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateNetQuickPlayPopup_SetOptionsHintVisible(self, int show)`
/// (0x1c13e0): show/hide the popup's two text objects.
pub fn cfe_net_quick_play_popup_set_options_hint_visible(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
        let have_screen = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if have_screen {
            c.cycles += 9;
            c.g[A0][0] = sx(0x0046_3ca0);
            call_ptr(c, 0x0031_7670, 0x001c140c)?;
            c.g[A0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001c1418)?;
            c.g[A2][0] = c.g[V0][0];
            let missing = c.g[A2][0] == 0;
            c.g[A1][0] = c.g[S1][0];
            if !missing {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
                c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
                let setter = r32(c, V1);
                c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                call_ptr(c, setter, 0x001c1438)?;
            }
            c.cycles += 9;
            c.g[A0][0] = sx(0x0046_58f0);
            call_ptr(c, 0x0031_7670, 0x001c1444)?;
            c.g[A0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001c1450)?;
            c.g[A2][0] = c.g[V0][0];
            let missing = c.g[A2][0] == 0;
            c.g[A1][0] = c.g[S1][0];
            if !missing {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
                c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
                let setter = r32(c, V1);
                c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                call_ptr(c, setter, 0x001c1470)?;
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


/// `void cFEStateNetQuickMessages_OnResetPopupEvent(self, a1, int kind)`
/// (0x1c2ba8): on kind 0xf clear the pending message, refresh the list and
/// resend the event as 0xf; other kinds go to the base handler.
pub fn cfe_net_quick_messages_on_reset_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0xf;
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        let kind = c.g[A2][0];
        let other = kind != c.g[V0][0];
        frame.save_ra(c, 0);
        if other {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, 0x001a_97b8, 0x001c2c34)?;
        } else {
            c.cycles += 23;
            call_ptr(c, 0x0014_dc80, 0x001c2bd4)?;
            c.g[A1][0] = 4;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_dd58, 0x001c2be0)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0014_f758, 0x001c2bec)?;
            c.g[V0][0] = 1;
            Addr(p(r32(c, S1), 0x6d0)).sw(c, c.g[V0][0]);
            c.g[V1][0] = Addr(p(r32(c, S0), 0xc)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 8)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0xc)).lw(c);
            let refresh = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, refresh, 0x001c2c08)?;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x001c_2418, 0x001c2c10)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = 0xf;
            call_ptr(c, 0x001a_97b8, 0x001c2c20)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `base + off` (32-bit).
#[inline(always)]
pub(crate) fn at_5(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}


/// `lw_6` of a 32-bit object field.
#[inline(always)]
pub(crate) fn lw_6(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lw(c)
}


/// A `gp_16`-relative global.
#[inline(always)]
pub(crate) fn gp_16(c: &Cpu, off: i32) -> u32 {
    abi::r32(c, GP).wrapping_add(off as u32)
}


// ---------------------------------------------------------------------------
// Net messenger: "user found" popup.

/// `void cFEStateNetMessenger_ShowUserFound(self, src)` (0x1a4ce8): copy the
/// found user's name into the state's string, configure a three-option popup
/// and show it.
pub fn net_fe_state_show_user_found(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        c.cycles += 42;
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[A1][0] = sx(at_5(c.g[S0][0], 8));
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(at_5(c.g[S2][0], 0x6d0));
        callee::CBXSTRING_ASSIGN.call(c, 0x001a_4d14)?;
        c.g[S3][0] = 3;
        c.g[V0][0] = lw_6(c, at_5(c.g[S0][0], 0x10));
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001a_4d28);
        Addr(at_5(c.g[S2][0], 0x6d8)).sw(c, c.g[V0][0]);
        callee::NETFE_STATE_GET_POPUP.call(c, 0x001a_4d28)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = sx(0x0046_1648);
        c.g[V0][0] = 1;
        c.g[A0][0] = sx(0x0046_1648);
        Addr(at_5(c.g[S0][0], 0xbc)).sw(c, c.g[V0][0]);
        c.g[RA][0] = sx(0x001a_4d44);
        c.g[S1][0] = sx(at_5(c.g[S0][0], 0xbc));
        call_ptr(c, 0x0031_7670, 0x001a_4d44)?;
        Addr(at_5(c.g[S0][0], 0xcc)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        Addr(at_5(c.g[S0][0], 0xc8)).sw(c, c.g[S3][0]);
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x001a_4d5c);
        c.g[A2][0] = lw_6(c, at_5(c.g[S2][0], 0x6d0));
        callee::POPUP_SET_OPTION_STRING.call(c, 0x001a_4d5c)?;
        Addr(at_5(c.g[S0][0], 0x14c)).sw(c, c.g[S3][0]);
        c.g[V0][0] = sx(0x809);
        Addr(at_5(c.g[S0][0], 0x150)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001a_4d74);
        c.g[A1][0] = c.g[S0][0];
        callee::NETFE_STATE_SHOW_POPUP.call(c, 0x001a_4d74)?;
        c.lq(16, frame.at(c, 0x40));
        c.lq(17, frame.at(c, 0x30));
        c.lq(18, frame.at(c, 0x20));
        c.lq(19, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Net FE state: disconnected / interrupted popup events.

/// `void NetFEState_OnDisconnectedPopupEvent(self, a1, event)` (0x1a9668):
/// for popup events 15 and 20 close every popup and go to the login state,
/// for any other event fall through to the default child-event handler.
pub fn net_fe_state_on_disconnected_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        'end: {
            'default: {
                'body: {
                    c.cycles += 6;
                    c.g[V0][0] = 0xf;
                    frame.save(c, S0, 0x10);
                    frame.save_ra(c, 0);
                    let t = c.g[A2][0] == c.g[V0][0];
                    c.g[S0][0] = c.g[A0][0];
                    if t {
                        break 'body;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x14;
                    if c.g[A2][0] != c.g[V0][0] {
                        break 'default;
                    }
                    break 'body;
                }
                c.cycles += 27;
                c.g[A0][0] = c.g[S0][0];
                callee::NETFE_STATE_ON_CHILD_EVENT_DEFAULT.call(c, 0x001a_9694)?;
                c.g[A0][0] = lw_6(c, at_5(c.g[S0][0], 0x10));
                c.g[RA][0] = sx(0x001a_96a0);
                c.g[A0][0] = sx(at_5(c.g[A0][0], 0x18));
                call_ptr(c, 0x0039_f698, 0x001a_96a0)?;
                c.g[A0][0] = lw_6(c, at_5(c.g[S0][0], 0x10));
                c.g[V0][0] = 1;
                Addr(at_5(c.g[S0][0], 0x58)).sw(c, c.g[V0][0]);
                c.g[RA][0] = sx(0x001a_96b4);
                c.g[A0][0] = sx(at_5(c.g[A0][0], 0x18));
                call_ptr(c, 0x0039_fa88, 0x001a_96b4)?;
                c.g[V1][0] = lw_6(c, at_5(c.g[V0][0], 0x1c));
                c.g[A1][0] = sx(0x0046_0000);
                c.g[A3][0] = 0;
                c.g[A2][0] = 0;
                c.g[V1][0] |= 0x80;
                c.g[A1][0] = sx(0x0046_1cd8);
                Addr(at_5(c.g[V0][0], 0x1c)).sw(c, c.g[V1][0]);
                c.g[RA][0] = sx(0x001a_96d8);
                c.g[A0][0] = 0x6d4;
                call_ptr(c, 0x0031_7d70, 0x001a_96d8)?;
                c.g[A1][0] = lw_6(c, at_5(c.g[S0][0], 0x10));
                c.g[RA][0] = sx(0x001a_96e4);
                c.g[A0][0] = c.g[V0][0];
                callee::CFE_STATE_LOGIN_CTOR.call(c, 0x001a_96e4)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001a_96f0);
                c.g[A0][0] = c.g[S0][0];
                callee::NETFE_STATE_GOTO_NEXT_STATE.call(c, 0x001a_96f0)?;
                Addr(at_5(c.g[S0][0], 0x6ac)).sw(c, 0);
                break 'end;
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            callee::NETFE_STATE_ON_CHILD_EVENT_DEFAULT.call(c, 0x001a_9700)?;
            break 'end;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void NetFEState_OnNetInterruptedPopupEvent(self, a1, event)` (0x1a9710):
/// the same for a network interruption, but to the net-config state.
pub fn net_fe_state_on_net_interrupted_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        'end: {
            'default: {
                'body: {
                    c.cycles += 6;
                    c.g[V0][0] = 0xf;
                    frame.save(c, S0, 0x10);
                    frame.save_ra(c, 0);
                    let t = c.g[A2][0] == c.g[V0][0];
                    c.g[S0][0] = c.g[A0][0];
                    if t {
                        break 'body;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x14;
                    if c.g[A2][0] != c.g[V0][0] {
                        break 'default;
                    }
                    break 'body;
                }
                c.cycles += 27;
                c.g[A0][0] = c.g[S0][0];
                callee::NETFE_STATE_ON_CHILD_EVENT_DEFAULT.call(c, 0x001a_973c)?;
                c.g[A0][0] = lw_6(c, at_5(c.g[S0][0], 0x10));
                c.g[RA][0] = sx(0x001a_9748);
                c.g[A0][0] = sx(at_5(c.g[A0][0], 0x18));
                call_ptr(c, 0x0039_f698, 0x001a_9748)?;
                c.g[A0][0] = lw_6(c, at_5(c.g[S0][0], 0x10));
                c.g[V0][0] = 1;
                Addr(at_5(c.g[S0][0], 0x58)).sw(c, c.g[V0][0]);
                c.g[RA][0] = sx(0x001a_975c);
                c.g[A0][0] = sx(at_5(c.g[A0][0], 0x18));
                call_ptr(c, 0x0039_fa88, 0x001a_975c)?;
                c.g[V1][0] = lw_6(c, at_5(c.g[V0][0], 0x1c));
                c.g[A1][0] = sx(0x0046_0000);
                c.g[A3][0] = 0;
                c.g[A2][0] = 0;
                c.g[V1][0] |= 0x80;
                c.g[A1][0] = sx(0x0046_0bf0);
                Addr(at_5(c.g[V0][0], 0x1c)).sw(c, c.g[V1][0]);
                c.g[RA][0] = sx(0x001a_9780);
                c.g[A0][0] = 0x91c;
                call_ptr(c, 0x0031_7d70, 0x001a_9780)?;
                c.g[A1][0] = lw_6(c, at_5(c.g[S0][0], 0x10));
                c.g[RA][0] = sx(0x001a_978c);
                c.g[A0][0] = c.g[V0][0];
                callee::CFE_STATE_NET_CONFIG_CTOR.call(c, 0x001a_978c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001a_9798);
                c.g[A0][0] = c.g[S0][0];
                callee::NETFE_STATE_GOTO_NEXT_STATE.call(c, 0x001a_9798)?;
                Addr(at_5(c.g[S0][0], 0x6ac)).sw(c, 0);
                break 'end;
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            callee::NETFE_STATE_ON_CHILD_EVENT_DEFAULT.call(c, 0x001a_97a8)?;
            break 'end;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Use-existing-account: password / account-name validation popups.

/// `int cFEUseExistingAccount_CheckPassword(self)` (0x1ac2b8): if the typed
/// password is empty, show the "password missing" popup and return 0.
pub fn cfe_use_existing_account_check_password(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        c.cycles += 11;
        c.g[V0][0] = lw_6(c, gp_16(c, -0xc8));
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x001a_c2dc);
        c.g[A0][0] = lw_6(c, at_5(c.g[V0][0], 0x4c));
        call_ptr(c, 0x0041_6810, 0x001a_c2dc)?;
        let t = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !t {
            c.cycles += 25;
            c.g[A0][0] = c.g[S2][0];
            callee::NETFE_STATE_GET_POPUP.call(c, 0x001a_c2ec)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = sx(0x0046_0000);
            c.g[V0][0] = 1;
            c.g[A0][0] = sx(0x0046_2bb0);
            Addr(at_5(c.g[S0][0], 0xbc)).sw(c, c.g[V0][0]);
            c.g[RA][0] = sx(0x001a_c308);
            c.g[S1][0] = sx(at_5(c.g[S0][0], 0xbc));
            call_ptr(c, 0x0031_7670, 0x001a_c308)?;
            c.g[V1][0] = 3;
            c.g[A0][0] = sx(0x0046_0000);
            Addr(at_5(c.g[S0][0], 0xc0)).sw(c, c.g[V1][0]);
            c.g[A0][0] = sx(0x0046_1618);
            c.g[RA][0] = sx(0x001a_c320);
            Addr(at_5(c.g[S0][0], 0xc4)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x0031_7670, 0x001a_c320)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001a_c330);
            c.g[A1][0] = 0;
            callee::POPUP_SET_OPTION_TEXT_ID.call(c, 0x001a_c330)?;
            c.g[V0][0] = sx(0x80d);
            c.g[A0][0] = c.g[S2][0];
            Addr(at_5(c.g[S0][0], 0x150)).sw(c, c.g[V0][0]);
            c.g[RA][0] = sx(0x001a_c344);
            c.g[A1][0] = c.g[S0][0];
            callee::NETFE_STATE_SHOW_POPUP.call(c, 0x001a_c344)?;
            c.g[V0][0] = 0;
        }
        c.cycles += 6;
        c.lq(16, frame.at(c, 0x30));
        c.lq(17, frame.at(c, 0x20));
        c.lq(18, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int cFEUseExistingAccount_CheckAccountName(self)` (0x1ac360): the same
/// for the account name (option text from the interface's `+0x48` string).
pub fn cfe_use_existing_account_check_account_name(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        c.cycles += 11;
        c.g[V0][0] = lw_6(c, gp_16(c, -0xc8));
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x001a_c384);
        c.g[A0][0] = lw_6(c, at_5(c.g[V0][0], 0x48));
        call_ptr(c, 0x0041_6810, 0x001a_c384)?;
        let t = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !t {
            c.cycles += 25;
            c.g[A0][0] = c.g[S2][0];
            callee::NETFE_STATE_GET_POPUP.call(c, 0x001a_c394)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = sx(0x0046_0000);
            c.g[V0][0] = 1;
            c.g[A0][0] = sx(0x0046_2bd0);
            Addr(at_5(c.g[S0][0], 0xbc)).sw(c, c.g[V0][0]);
            c.g[RA][0] = sx(0x001a_c3b0);
            c.g[S1][0] = sx(at_5(c.g[S0][0], 0xbc));
            call_ptr(c, 0x0031_7670, 0x001a_c3b0)?;
            c.g[V1][0] = 3;
            c.g[A0][0] = sx(0x0046_0000);
            Addr(at_5(c.g[S0][0], 0xc0)).sw(c, c.g[V1][0]);
            c.g[A0][0] = sx(0x0046_1618);
            c.g[RA][0] = sx(0x001a_c3c8);
            Addr(at_5(c.g[S0][0], 0xc4)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x0031_7670, 0x001a_c3c8)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001a_c3d8);
            c.g[A1][0] = 0;
            callee::POPUP_SET_OPTION_TEXT_ID.call(c, 0x001a_c3d8)?;
            c.g[V0][0] = sx(0x80d);
            c.g[A0][0] = c.g[S2][0];
            Addr(at_5(c.g[S0][0], 0x150)).sw(c, c.g[V0][0]);
            c.g[RA][0] = sx(0x001a_c3ec);
            c.g[A1][0] = c.g[S0][0];
            callee::NETFE_STATE_SHOW_POPUP.call(c, 0x001a_c3ec)?;
            c.g[V0][0] = 0;
        }
        c.cycles += 6;
        c.lq(16, frame.at(c, 0x30));
        c.lq(17, frame.at(c, 0x20));
        c.lq(18, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Chat room: open the keyboard for the message field.

/// `int cFEStateChatRoom_HandleInput(self, ev)` (0x1b38d0): if the event
/// target's virtual "is pressed" says so, open the text-entry keyboard with
/// the config popup's current option, then enable the field.
pub fn cfe_state_chat_room_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        c.cycles += 11;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = lw_6(c, at_5(c.g[A1][0], 8));
        c.g[A0][0] = lh(c, at_5(c.g[V1][0], 0xd8));
        c.g[V0][0] = lw_6(c, at_5(c.g[V1][0], 0xdc));
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x001b_38f4);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001b_38f4)?;
        let t = c.g[V0][0] != 0;
        c.g[A0][0] = sx(0x0046_0000);
        if !t {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 25;
            c.g[RA][0] = sx(0x001b_390c);
            c.g[A0][0] = sx(0x0046_3db8);
            call_ptr(c, 0x0031_7670, 0x001b_390c)?;
            c.g[A3][0] = sx(0x004a_0000);
            c.g[A2][0] = c.g[V0][0];
            c.g[T0][0] = 0;
            c.g[T1][0] = 4;
            c.g[T2][0] = 0x3f;
            c.g[A3][0] = sx(0x004a_1408);
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x001b_3930);
            c.g[A1][0] = 0;
            callee::NETFE_STATE_OPEN_KEYBOARD_TEXT_ID.call(c, 0x001b_3930)?;
            c.g[V0][0] = lw_6(c, at_5(c.g[S0][0], 0x69c));
            c.g[A1][0] = 1;
            Addr(at_5(c.g[V0][0], 0x43c)).sw(c, 0);
            c.g[RA][0] = sx(0x001b_3944);
            c.g[A0][0] = lw_6(c, at_5(c.g[S0][0], 0x69c));
            call_ptr(c, 0x001c_e468, 0x001b_3944)?;
            c.g[A0][0] = lw_6(c, at_5(c.g[S0][0], 0x69c));
            c.g[A1][0] = 0x4b;
            c.g[A2][0] = 1;
            c.g[RA][0] = sx(0x001b_3958);
            c.g[A3][0] = 2;
            call_ptr(c, 0x001c_e3c8, 0x001b_3958)?;
            c.g[A0][0] = lw_6(c, at_5(c.g[S0][0], 0x69c));
            c.g[V1][0] = 1;
            c.g[V0][0] = 1;
            Addr(at_5(c.g[A0][0], 0x438)).sw(c, c.g[V1][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// ASCII sprintf wrappers (UTF-8 argument to UTF-16 output).

/// `void UStr_SprintfAsciiFmt(char* buf, const char* fmt, ...)` (0x2c2718):
/// widen the (signed-char) format bytes into a stack UTF-16 buffer and run
/// `UStr_vsprintf` with the vararg save area.
pub fn ustr_sprintf_ascii_fmt(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x8b0);
        'end: {
            c.cycles += 22;
            frame.save(c, S1, 0x820);
            frame.save(c, S2, 0x810);
            c.g[S1][0] = c.g[A1][0];
            frame.save(c, S0, 0x830);
            c.g[S2][0] = c.g[A0][0];
            Addr(frame.at(c, 0x880)).sd(c, c.g[A2][0]);
            c.g[S0][0] = sp_of(c);
            frame.save_ra(c, 0x800);
            c.g[A0][0] = c.g[S1][0];
            Addr(frame.at(c, 0x888)).sd(c, c.g[A3][0]);
            Addr(frame.at(c, 0x890)).sd(c, c.g[T0][0]);
            Addr(frame.at(c, 0x898)).sd(c, c.g[T1][0]);
            Addr(frame.at(c, 0x8a0)).sd(c, c.g[T2][0]);
            Addr(frame.at(c, 0x8a8)).sd(c, c.g[T3][0]);
            c.swc1(12, frame.at(c, 0x870));
            c.swc1(14, frame.at(c, 0x874));
            c.swc1(16, frame.at(c, 0x878));
            c.g[RA][0] = sx(0x002c_2768);
            c.swc1(18, frame.at(c, 0x87c));
            call_ptr(c, 0x0041_6810, 0x002c_2768)?;
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A2][0] = sx(frame.at(c, 0x880));
            if t {
                break 'end;
            }
            c.cycles += 2;
            c.g[V1][0] = c.g[V0][0];
            loop {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).lbu(c);
                c.g[V1][0] = sx(at_5(c.g[V1][0], 0xffff_ffff));
                c.g[S1][0] = sx(at_5(c.g[S1][0], 1));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                Addr(c.g[S0][0] as u32).sh(c, c.g[V0][0]);
                let t = c.g[V1][0] != 0;
                c.g[S0][0] = sx(at_5(c.g[S0][0], 2));
                if !t {
                    break 'end;
                }
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x002c_2778);
                }
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S2][0];
        Addr(c.g[S0][0] as u32).sh(c, 0);
        c.g[RA][0] = sx(0x002c_27a8);
        c.g[A1][0] = sp_of(c);
        call_ptr(c, 0x003d_c450, 0x002c_27a8)?;
        frame.restore(c, S0, 0x830);
        frame.restore(c, S1, 0x820);
        frame.restore(c, S2, 0x810);
        frame.restore_ra(c, 0x800);
        frame.close(c);
        Ok(())
    })
}


/// `int UStr_SprintfAsciiFmt_2(char* buf, const char* fmt, ...)` (0x2c27c0):
/// the same as [`ustr_sprintf_ascii_fmt`].
pub fn ustr_sprintf_ascii_fmt_2(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x8b0);
        'end: {
            c.cycles += 22;
            frame.save(c, S1, 0x820);
            frame.save(c, S2, 0x810);
            c.g[S1][0] = c.g[A1][0];
            frame.save(c, S0, 0x830);
            c.g[S2][0] = c.g[A0][0];
            Addr(frame.at(c, 0x880)).sd(c, c.g[A2][0]);
            c.g[S0][0] = sp_of(c);
            frame.save_ra(c, 0x800);
            c.g[A0][0] = c.g[S1][0];
            Addr(frame.at(c, 0x888)).sd(c, c.g[A3][0]);
            Addr(frame.at(c, 0x890)).sd(c, c.g[T0][0]);
            Addr(frame.at(c, 0x898)).sd(c, c.g[T1][0]);
            Addr(frame.at(c, 0x8a0)).sd(c, c.g[T2][0]);
            Addr(frame.at(c, 0x8a8)).sd(c, c.g[T3][0]);
            c.swc1(12, frame.at(c, 0x870));
            c.swc1(14, frame.at(c, 0x874));
            c.swc1(16, frame.at(c, 0x878));
            c.g[RA][0] = sx(0x002c_2810);
            c.swc1(18, frame.at(c, 0x87c));
            call_ptr(c, 0x0041_6810, 0x002c_2810)?;
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A2][0] = sx(frame.at(c, 0x880));
            if t {
                break 'end;
            }
            c.cycles += 2;
            c.g[V1][0] = c.g[V0][0];
            loop {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).lbu(c);
                c.g[V1][0] = sx(at_5(c.g[V1][0], 0xffff_ffff));
                c.g[S1][0] = sx(at_5(c.g[S1][0], 1));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                Addr(c.g[S0][0] as u32).sh(c, c.g[V0][0]);
                let t = c.g[V1][0] != 0;
                c.g[S0][0] = sx(at_5(c.g[S0][0], 2));
                if !t {
                    break 'end;
                }
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x002c_2820);
                }
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S2][0];
        Addr(c.g[S0][0] as u32).sh(c, 0);
        c.g[RA][0] = sx(0x002c_2850);
        c.g[A1][0] = sp_of(c);
        call_ptr(c, 0x003d_c450, 0x002c_2850)?;
        frame.restore(c, S0, 0x830);
        frame.restore(c, S1, 0x820);
        frame.restore(c, S2, 0x810);
        frame.restore_ra(c, 0x800);
        frame.close(c);
        Ok(())
    })
}


/// The current stack pointer.
#[inline(always)]
pub(crate) fn sp_of(c: &Cpu) -> u64 {
    sx(crate::call::sp(c))
}


// ---------------------------------------------------------------------------
// `cFEStateCreateStep2_FillGenderList` (0x1bb520): add the two gender strings
// to the list box, select the entry matching `flag == 0`, and let the box
// apply its selection.
pub fn cfe_state_create_step2_fill_gender_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = 0xffff_ffff_ffff_fffe;
        frame.save(c, S0, 0x30);
        c.g[A0][0] = 0x460000;
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A2][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4b10));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.sb((c.g[SP][0] as u32), 0u64);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | 0x80;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        call_ptr(c, 0x00317670, 0x001b_b55c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x0039a568, 0x001b_b56c)?;
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4b20));
        call_ptr(c, 0x00317670, 0x001b_b578)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = 0x1;
        call_ptr(c, 0x0039a568, 0x001b_b588)?;
        c.g[A1][0] = (c.g[S1][0] < 0x1u64) as u64;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x0039a768, 0x001b_b598)?;
        c.g[A1][0] = c.lbu((c.g[SP][0] as u32));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039a7a8, 0x001b_b5a4)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0x1;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, target, 0x001b_b5bc)?;
        c.lq(16, frame.at(c, 0x30));
        c.lq(17, frame.at(c, 0x20));
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// `cFEStateNetCreateRoomPopup_ValidateRoomName` (0x1bec78): if the room-name
// field is empty, configure the confirm popup for it and show it; return 1
// when a name is present.
pub fn net_create_room_popup_validate_room_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6dc));
        call_ptr(c, 0x00416810, 0x001b_ec9c)?;
        let has_name = c.g[V0][0] != 0;
        c.g[V0][0] = 0x1;
        if has_name {
            // return 1
        } else {
            c.cycles += 26;
            c.g[A0][0] = c.g[S3][0];
            c.g[S2][0] = 0x3;
            callee::NET_FE_STATE_GET_POPUP_5.call(c, 0x001b_ecb0)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 0x460000;
            c.g[V0][0] = 0x1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4fc0));
            c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
            call_ptr(c, 0x00317670, 0x001b_eccc)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[S2][0]);
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
            call_ptr(c, 0x00317670, 0x001b_ece0)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0u64;
            callee::POPUP_CFG_SET_OPTION_TEXT_ID.call(c, 0x001b_ecf0)?;
            c.g[V0][0] = 0x847;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14c), c.g[S2][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S0][0];
            callee::NET_FE_STATE_SHOW_POPUP_4.call(c, 0x001b_ed08)?;
            c.g[V0][0] = 0u64;
        }
        c.cycles += 7;
        c.lq(16, frame.at(c, 0x40));
        c.lq(17, frame.at(c, 0x30));
        c.lq(18, frame.at(c, 0x20));
        c.lq(19, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Front end.

/// `void cFEStateNetNews_OnWidgetAction(self, widget, int msg)` (0x1c5368):
/// the net-news screen's widget router. Message 8 opens the messenger when
/// no other popup is up, 6 builds and enters the SSX World state, 5 is
/// ignored and everything else (and a missing widget) goes to the base net
/// state handler.
pub fn cfe_state_net_news_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let msg = c.g[A2][0];
        let mut tail = 1; // the `cycles += 1` only the shared exit charges
        if c.g[A1][0] == 0 {
            c.cycles += 2;
            c.g[A0][0] = this;
            call_ptr(c, 0x001a_8918, 0x001c_540c)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 6;
            let is6 = msg == 6;
            c.g[V0][0] = (msg < 7) as u64;
            if is6 {
                // message 6: enter a fresh SSX World state
                net_news_enter_ssx_world(c)?;
                tail = 0;
            } else {
                c.cycles += 2;
                let ge7 = c.g[V0][0] == 0;
                c.g[V0][0] = 5;
                if !ge7 {
                    c.cycles += 2;
                    if msg == 5 {
                        tail = 0;
                    } else {
                        c.cycles += 2;
                        c.cycles += 2;
                        c.g[A0][0] = this;
                        call_ptr(c, 0x001a_8918, 0x001c_540c)?;
                    }
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 8;
                    if msg != 8 {
                        c.cycles += 2;
                        c.g[A0][0] = this;
                        call_ptr(c, 0x001a_8918, 0x001c_540c)?;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x1c)).lw(c);
                        c.g[V0][0] = c.g[V0][0] & 0x6;
                        if c.g[V0][0] == 0 {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x001a_9b58, 0x001c_53c4)?;
                        }
                        tail = 0;
                    }
                }
            }
        }
        c.cycles += tail;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// The `case 6` body of [`cfe_state_net_news_on_widget_action`]: allocate a
/// 0x6d0-byte SSX World state, construct it and enter it.
pub(crate) fn net_news_enter_ssx_world(c: &mut Cpu) -> Flow {
    c.cycles += 14;
    c.g[A1][0] = 0x460000;
    c.g[A0][0] = 0x6d0;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x11a8));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, 0x001c_53e4)?;
    c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x10)).lw(c);
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x001c_1c50, 0x001c_53f0)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x001a_8a40, 0x001c_53fc)?;
    Ok(())
}


/// `void cFEStateOnlineMainMenu_OnDisconnectPopupEvent(self, item, msg)`
/// (0x1c0240): on message 0xf tear down the current net connection and push
/// the account memory-card popup, otherwise offer the event to the base net
/// state handler.
pub fn cfe_online_main_menu_on_disconnect_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        if c.g[A2][0] != 0xf {
            c.cycles += 3;
            c.g[A0][0] = this;
            call_ptr(c, 0x001a_97b8, 0x001c_02e4)?;
        } else {
            c.cycles += 33;
            c.g[A2][0] = 0xf;
            call_ptr(c, 0x001a_97b8, 0x001c_0260)?;
            c.g[A0][0] = Addr(p(r32(c, GP), 0x238)).lw(c);
            call_ptr(c, 0x0026_1408, 0x001c_0268)?;
            c.g[A0][0] = Addr(p(r32(c, GP), 0x238)).lw(c);
            call_ptr(c, 0x0026_1070, 0x001c_0270)?;
            c.g[A0][0] = Addr(p(r32(c, GP), 0x238)).lw(c);
            call_ptr(c, 0x0026_1058, 0x001c_0278)?;
            c.g[A0][0] = Addr(p(r32(c, GP), 0xffff_ff38)).lw(c);
            call_ptr(c, 0x0025_b800, 0x001c_0280)?;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = 0xb10;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2d10));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x001c_0298)?;
            c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x10)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[T0][0] = 0;
            c.g[T1][0] = 0;
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = 0;
            call_ptr(c, 0x001b_6ed0, 0x001c_02b4)?;
            c.g[V1][0] = 0x832;
            c.g[A1][0] = c.g[V0][0];
            c.sw(p(c.g[V0][0] as u32, 0x18), c.g[V1][0]);
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x40)).lw(c);
            c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0xd0)).lw(c);
            c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x10)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039_f290, 0x001c_02d4)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateLobbyMenu_UpdatePlayerScrollArrows(self)` (0x1be720): enable
/// the up arrow's widget when there are entries, and the down arrow's widget
/// unless the player is already at the bottom of the list.
pub fn cfe_lobby_menu_update_player_scroll_arrows(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let count = Addr(this as u32).off(0x778).lw(c);
        c.g[A2][0] = Addr(this as u32).off(0x728).lw(c);
        if count == 0 {
            c.cycles += 8;
            set_widget_enabled(c, 0, 0x001b_e754)?;
            c.cycles += 5;
        } else {
            c.cycles += 7;
            set_widget_enabled(c, 1, 0x001b_e774)?;
            c.cycles += 5;
        }
        c.g[V0][0] = Addr(this as u32).off(0x778).lw(c);
        c.g[A2][0] = Addr(this as u32).off(0x72c).lw(c);
        c.g[V1][0] = Addr(this as u32).off(0x77c).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xa));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            set_widget_enabled(c, 1, 0x001b_e7c4)?;
        } else {
            c.cycles += 8;
            set_widget_enabled(c, 0, 0x001b_e7a4)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// Call `widget->v09(enabled)` through the vtable at `*(A2 + 8)` (slot 0x48).
pub(crate) fn set_widget_enabled(c: &mut Cpu, enabled: u32, ret: u32) -> Flow {
    c.g[A1][0] = enabled as u64;
    c.g[V0][0] = Addr(p(c.g[A2][0] as u32, 8)).lw(c);
    c.g[A0][0] = Addr(c.g[V0][0] as u32).lh(c);
    c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x4c)).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}


/// `void cFEStateNetMessenger_HighlightRow(self, int row)` (0x1a5fb8): colour
/// the 15 roster rows, highlighting `row`.
pub fn cfe_state_net_messenger_highlight_row(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S0, 0x70);
        frame.save_ra(c, 0x20);
        let this = c.g[A0][0];
        c.g[S3][0] = this;
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = 0;
        c.g[S4][0] = 0x460000;
        c.g[A2][0] = 0;
        loop {
            c.cycles += 13;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x15b0));
            c.g[S0][0] = c.g[S1][0] ^ c.g[S2][0];
            call_ptr(c, 0x0041_7828, 0x001a_5ff8)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x001a_6004)?;
            c.g[A0][0] = Addr(p(c.g[S3][0] as u32, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001a_6010)?;
            c.g[A2][0] = c.g[V0][0];
            let missing = c.g[A2][0] == 0;
            c.g[A1][0] = (c.g[S0][0] < 1) as u64;
            if !missing {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(c.g[A2][0] as u32, 8)).lw(c);
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x48)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x4c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x001a_6030)?;
            }
            c.cycles += 7;
            c.g[A2][0] = (c.g[S0][0] < 1) as u64;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, 0x001a_66a8, 0x001a_6040)?;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 15) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A2][0] = c.g[S1][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001a_5fe8);
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}


/// `void* NetFEState_CreateKeyboardPopup(self, a1, a2, a3, int a4)`
/// (0x1a91b0): allocate and construct the keyboard popup and push it on the
/// state stack.
pub fn net_fe_state_create_keyboard_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 45;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S4, 0x10);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        frame.save(c, S2, 0x30);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[S4][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A3][0];
        c.g[S2][0] = c.g[T0][0];
        c.g[A1][0] = 0x460000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2728));
        c.g[A0][0] = 0x444;
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x001a_91f8)?;
        c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x10)).lw(c);
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[T0][0] = 0xf;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x001c_b030, 0x001a_9210)?;
        c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        c.sw(p(c.g[S0][0] as u32, 0x69c), c.g[V0][0]);
        c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0xd0)).lw(c);
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x10)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0039_f290, 0x001a_922c)?;
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x69c)).lw(c);
        c.g[A1][0] = c.g[S2][0];
        c.sw(p(c.g[V0][0] as u32, 0x18), c.g[S3][0]);
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x69c)).lw(c);
        call_ptr(c, 0x001c_d088, 0x001a_9240)?;
        c.sw(p(c.g[S0][0] as u32, 0x6a4), c.g[S4][0]);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// FE state: two-row popup screens (the `_v25` handlers).

/// The shared body of the three `_v25` handlers below: set the popup's state,
/// then three rows of (label hash, text hash, icon) built from the FE string
/// table at `0x460000`.
pub(crate) fn fe_v25_populate(c: &mut Cpu, base: u32, strings: [u32; 5], icons: [u64; 3]) -> Flow {
    c.cycles += 46;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    c.g[S0][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[A1][0] = 3;
    c.g[RA][0] = sx(base.wrapping_add(0x1c));
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x001d_a528, base.wrapping_add(0x1c))?;

    c.g[A0][0] = sx(0x0046_0000);
    c.g[RA][0] = sx(base.wrapping_add(0x28));
    c.g[A0][0] = sx(strings[0]);
    call_ptr(c, callee::FE_STRING_HASH, base.wrapping_add(0x28))?;
    c.g[A0][0] = sx(0x0046_0000);
    Addr(p(r(c, S0), 0x54)).sw(c, c.g[V0][0]);
    c.g[RA][0] = sx(base.wrapping_add(0x38));
    c.g[A0][0] = sx(strings[1]);
    call_ptr(c, callee::FE_STRING_HASH, base.wrapping_add(0x38))?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[RA][0] = sx(base.wrapping_add(0x48));
    c.g[A1][0] = 0;
    call_ptr(c, 0x001d_a530, base.wrapping_add(0x48))?;

    c.g[A0][0] = sx(0x0046_0000);
    c.g[RA][0] = sx(base.wrapping_add(0x54));
    c.g[A0][0] = sx(strings[2]);
    call_ptr(c, callee::FE_STRING_HASH, base.wrapping_add(0x54))?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    c.g[RA][0] = sx(base.wrapping_add(0x68));
    c.g[A3][0] = icons[0];
    call_ptr(c, 0x001d_a510, base.wrapping_add(0x68))?;

    c.g[A0][0] = sx(0x0046_0000);
    c.g[RA][0] = sx(base.wrapping_add(0x74));
    c.g[A0][0] = sx(strings[3]);
    call_ptr(c, callee::FE_STRING_HASH, base.wrapping_add(0x74))?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    c.g[RA][0] = sx(base.wrapping_add(0x88));
    c.g[A3][0] = icons[1];
    call_ptr(c, 0x001d_a510, base.wrapping_add(0x88))?;

    c.g[A0][0] = sx(0x0046_0000);
    c.g[RA][0] = sx(base.wrapping_add(0x94));
    c.g[A0][0] = sx(strings[4]);
    call_ptr(c, callee::FE_STRING_HASH, base.wrapping_add(0x94))?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 2;
    c.g[RA][0] = sx(base.wrapping_add(0xa8));
    c.g[A3][0] = icons[2];
    call_ptr(c, 0x001d_a510, base.wrapping_add(0xa8))?;

    frame.restore(c, 16, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}


/// `void cFEUseExistingAccount_v25(cFEState* s, void* tooltip)` (0x1ac4f0).
pub fn fe_use_existing_account_v25(c: &mut Cpu) {
    guest(c, |c| {
        fe_v25_populate(
            c,
            0x001a_c4f0,
            [0x0046_2c08, 0x0046_2c28, 0x0045_fe60, 0x0046_2c50, 0x0046_14e8],
            [3, 0, 1],
        )
    })
}


/// `void cFEStateOnlineCharacterSelect_v25(cFEState* s, void* tooltip)`
/// (0x1b14d8).
pub fn fe_state_online_character_select_v25(c: &mut Cpu) {
    guest(c, |c| {
        fe_v25_populate(
            c,
            0x001b_14d8,
            [0x0046_3848, 0x0046_3868, 0x0046_3890, 0x0046_38b0, 0x0046_14e8],
            [4, 0, 1],
        )
    })
}


/// `void cFEStateNetNews_v25(cFEState* s, void* tooltip)` (0x1c5540).
pub fn fe_state_net_news_v25(c: &mut Cpu) {
    guest(c, |c| {
        fe_v25_populate(
            c,
            0x001c_5540,
            [0x0046_5f58, 0x0046_5f70, 0x0046_5f90, 0x0046_14e8, 0x0046_1458],
            [3, 1, 2],
        )
    })
}


// ---------------------------------------------------------------------------
// FE state: challenge popup.

/// `cFEStateChallengePopup* cFEStateChallengePopup_ctor(cFEStateChallengePopup*
/// s)` (0x1addd0): construct the network sub-state, install the popup vtable
/// and the two option rows.
pub fn fe_state_challenge_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 46;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = 1;
        callee::NET_FE_STATE_CTOR_12.call(c, 0x001a_ddf0)?;

        c.g[V0][0] = sx(0x0046_8e38);
        c.g[V1][0] = sx(0x0044_1268);
        c.g[A0][0] = 0x3f;
        Addr(p(r(c, S1), 0x6d8)).sw(c, c.g[V1][0]);
        Addr(p(r(c, S1), 0xc)).sw(c, c.g[A0][0]);
        Addr(p(r(c, S1), 8)).sw(c, c.g[V0][0]);
        Addr(p(r(c, S1), 0x6d0)).sw(c, 0);
        Addr(p(r(c, S1), 0x6dc)).sw(c, 0);
        c.g[RA][0] = sx(0x001a_de20);
        Addr(gp(c, 0x1d98)).sw(c, 0);
        call_ptr(c, 0x0014_dc80, 0x001a_de20)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001a_de2c);
        c.g[A1][0] = 0;
        call_ptr(c, callee::BE_GET_INTERFACE, 0x001a_de2c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[V0][0] = sx(0x0053_0000);
        c.g[A0][0] = c.g[S0][0];
        Addr(gp(c, -0x1f38)).sw(c, c.g[S2][0]);
        c.g[RA][0] = sx(0x001a_de48);
        Addr(p(r(c, V0), 0x5c08)).sw(c, 0);
        call_ptr(c, 0x0014_4df0, 0x001a_de48)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x001a_de54);
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_51e8, 0x001a_de54)?;
        c.g[V1][0] = Addr(p(r(c, S0), 0xc)).lw(c);
        c.g[A0][0] = Addr(p(r(c, V1), 8)).lh(c);
        c.g[V0][0] = Addr(p(r(c, V1), 0xc)).lw(c);
        let target = r(c, V0);
        c.g[RA][0] = sx(0x001a_de68);
        c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
        call_ptr(c, target, 0x001a_de68)?;
        Addr(p(r(c, S1), 0x6d4)).sw(c, c.g[S2][0]);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// FE state: chat room.

/// `void cFEStateChatRoom_OnKeyboardDone(cFEStateChatRoom* s)` (0x1b33d8):
/// when the keyboard popup was accepted, post the edited text to the lobby.
pub fn fe_state_chat_room_on_keyboard_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        'outer: {
            'accepted: {
                c.g[V0][0] = 2;
                frame.save(c, S1, 0x10);
                frame.save(c, S0, 0x20);
                let this = r(c, A0);
                c.g[S1][0] = sx(this);
                frame.save_ra(c, 0);
                c.g[A1][0] = Addr(p(this, 0x69c)).lw(c);
                c.g[V1][0] = Addr(p(r(c, A1), 0x18)).lw(c);
                let ok = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 4;
                if ok {
                    break 'accepted;
                }
                c.cycles += 2;
                let cancel = c.g[V1][0] != c.g[V0][0];
                frame.restore(c, 16, 0x20);
                if cancel {
                    break 'outer;
                }
                c.cycles += 5;
                c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
                c.g[RA][0] = sx(0x001b_3414);
                c.g[A1][0] = sx(p(r(c, A1), 0x74));
                call_ptr(c, 0x0025_aae0, 0x001b_3414)?;
                frame.restore(c, 16, 0x20);
                break 'outer;
            }
            'lobby: {
                c.cycles += 3;
                c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
                let present = c.g[V0][0] != 0;
                if present {
                    break 'lobby;
                }
                c.cycles += 2;
                c.g[RA][0] = sx(0x001b_3430);
                call_ptr(c, 0x0026_0f80, 0x001b_3430)?;
                break 'lobby;
            }
            'update: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001b_3438);
                c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
                call_ptr(c, 0x0026_1460, 0x001b_3438)?;
                let busy = c.g[V0][0] != 0;
                c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
                if busy {
                    break 'update;
                }
                c.cycles += 3;
                c.g[RA][0] = sx(0x001b_3448);
                c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
                call_ptr(c, 0x0026_13c8, 0x001b_3448)?;
                c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
                break 'update;
            }
            'bind: {
                c.cycles += 2;
                let present = c.g[V0][0] != 0;
                c.g[S0][0] = Addr(gp(c, 0x238)).lw(c);
                if present {
                    break 'bind;
                }
                c.cycles += 3;
                c.g[RA][0] = sx(0x001b_345c);
                c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
                call_ptr(c, 0x0025_b800, 0x001b_345c)?;
                c.g[S0][0] = Addr(gp(c, 0x238)).lw(c);
                break 'bind;
            }
            c.cycles += 8;
            c.g[RA][0] = sx(0x001b_3468);
            c.g[A0][0] = c.g[S1][0];
            callee::NET_FE_STATE_GET_SELECTED_USER.call(c, 0x001b_3468)?;
            c.g[A2][0] = Addr(p(r(c, S1), 0x69c)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001b_347c);
            c.g[A2][0] = sx(p(r(c, A2), 0x74));
            call_ptr(c, 0x0026_2690, 0x001b_347c)?;
            frame.restore(c, 16, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// FE state: chat room player selection.

/// `void cFEStateChatRoom_SelectPlayerByName(cFEStateChatRoom* s, const char*
/// name)` (0x1b3978): find the room user whose name matches and select that
/// player in the chat room's screen.
pub fn fe_state_chat_room_select_player_by_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        'outer: {
            frame.save(c, S3, 0x20);
            frame.save(c, S4, 0x10);
            frame.save(c, S0, 0x50);
            frame.save(c, S1, 0x40);
            frame.save(c, S2, 0x30);
            frame.save_ra(c, 0);
            let this = r(c, A0);
            c.g[S4][0] = sx(this);
            c.g[V0][0] = Addr(p(this, 0x40)).lw(c);
            let no_screen = c.g[V0][0] == 0;
            c.g[S3][0] = c.g[A1][0];
            if no_screen {
                break 'outer;
            }
            c.cycles += 5;
            c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
            c.g[S2][0] = u64::MAX;
            c.g[RA][0] = sx(0x001b_39b4);
            c.g[S0][0] = 0;
            call_ptr(c, 0x0025_f1b0, 0x001b_39b4)?;
            c.g[S1][0] = c.g[V0][0];
            'scan: loop {
                'search: {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    let end = c.g[V0][0] == 0;
                    c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
                    if end {
                        break 'search;
                    }
                    c.cycles += 8;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001b_39d0);
                    c.g[A2][0] = 0;
                    call_ptr(c, 0x0025_f1e0, 0x001b_39d0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001b_39dc);
                    c.g[A1][0] = c.g[S3][0];
                    call_ptr(c, 0x0041_65a8, 0x001b_39dc)?;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                        if event_due(c) {
                            deliver_event_here(c, 0x001b_39b8);
                        }
                        continue 'scan;
                    }
                    c.cycles += 1;
                    c.g[S2][0] = c.g[S0][0];
                    break 'search;
                }
                c.cycles += 2;
                let found = (c.g[S2][0] as i64) < 0;
                c.g[A0][0] = sx(0x0046_0000);
                if found {
                    break 'outer;
                }
                c.cycles += 8;
                c.g[RA][0] = sx(0x001b_39f8);
                c.g[A0][0] = sx(0x0046_3a38);
                call_ptr(c, callee::FE_STRING_HASH, 0x001b_39f8)?;
                c.g[A0][0] = Addr(p(r(c, S4), 0x40)).lw(c);
                c.g[RA][0] = sx(0x001b_3a04);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001b_3a04)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001b_3a10);
                c.g[A1][0] = c.g[S2][0] & 0xff;
                call_ptr(c, 0x0039_b960, 0x001b_3a10)?;
                break 'outer;
            }
        }
        c.cycles += 8;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// FE state: network config teardown.

/// `void cFEStateNetConfig_dtor(cFEStateNetConfig* s, int free)` (0x1b4018):
/// release the four optional sub-objects, then the net state itself.
pub fn fe_state_net_config_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(0x0046_8b00);
        frame.save(c, S0, 0x20);
        let this = r(c, A0);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(p(this, 0x710)).lw(c);
        let absent = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !absent {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001b_404c);
            call_ptr(c, callee::FREE2, 0x001b_404c)?;
            Addr(p(r(c, S0), 0x710)).sw(c, 0);
        }
        'first: {
            c.cycles += 3;
            c.g[A0][0] = Addr(p(r(c, S0), 0x714)).lw(c);
            if c.g[A0][0] == 0 {
                c.g[A0][0] = Addr(p(r(c, S0), 0x718)).lw(c);
                break 'first;
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x001b_4064);
            call_ptr(c, callee::FREE2, 0x001b_4064)?;
            Addr(p(r(c, S0), 0x714)).sw(c, 0);
            c.g[A0][0] = Addr(p(r(c, S0), 0x718)).lw(c);
        }
        'second: {
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                c.g[A0][0] = Addr(p(r(c, S0), 0x704)).lw(c);
                break 'second;
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x001b_407c);
            call_ptr(c, callee::FREE2, 0x001b_407c)?;
            Addr(p(r(c, S0), 0x718)).sw(c, 0);
            c.g[A0][0] = Addr(p(r(c, S0), 0x704)).lw(c);
        }
        'third: {
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                c.g[A0][0] = Addr(gp(c, -0x1578)).lw(c);
                break 'third;
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x001b_4094);
            call_ptr(c, callee::FREE2, 0x001b_4094)?;
            Addr(p(r(c, S0), 0x704)).sw(c, 0);
            c.g[A0][0] = Addr(gp(c, -0x1578)).lw(c);
        }
        'fourth: {
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                c.g[A0][0] = c.g[S0][0];
                break 'fourth;
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x001b_40ac);
            call_ptr(c, 0x002c_2300, 0x001b_40ac)?;
            Addr(gp(c, -0x1578)).sw(c, 0);
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 7;
        c.g[RA][0] = sx(0x001b_40bc);
        c.g[A1][0] = c.g[S1][0];
        callee::NET_FE_STATE_DTOR_4.call(c, 0x001b_40bc)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// FE state: create step 2.

/// `void cFEStateCreateStep2_Init(cFEStateCreateStep2* s)` (0x1ba428): create
/// the "create rider, step 2" screen, play its first frame, clear the date
/// selection array and hand the six option widgets their initial state.
pub fn fe_state_create_step2_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        'outer: {
            c.g[V0][0] = Addr(gp(c, -0xc8)).lw(c);
            frame.save(c, S1, 0x10);
            let this = r(c, A0);
            c.g[S1][0] = sx(this);
            frame.save(c, S0, 0x20);
            frame.save_ra(c, 0);
            c.g[V0][0] = sx(r(c, V0).wrapping_add(0x260));
            Addr(p(this, 0x6d4)).sw(c, 0);
            c.g[A0][0] = sx(0x0046_49f8);
            c.g[S0][0] = Addr(p(this, 0x10)).lw(c);
            c.g[RA][0] = sx(0x001b_a45c);
            Addr(p(this, 0x6d0)).sw(c, c.g[V0][0]);
            call_ptr(c, callee::FE_STRING_HASH, 0x001b_a45c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001b_a470);
            c.g[A3][0] = 0;
            call_ptr(c, 0x0039_7cc0, 0x001b_a470)?;
            let none = c.g[V0][0] == 0;
            Addr(p(r(c, S1), 0x40)).sw(c, c.g[V0][0]);
            if none {
                break 'outer;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001b_a488);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001b_a488)?;
            break 'outer;
        }
        c.cycles += 22;
        c.g[V0][0] = Addr(p(r(c, S1), 0x6d0)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        Addr(p(r(c, V0), 0x40)).sw(c, 0);
        c.g[RA][0] = sx(0x001b_a49c);
        Addr(p(r(c, S1), 0x6dc)).sw(c, 0);
        callee::CREATE_STEP2_SET_MONTH.call(c, 0x001b_a49c)?;
        c.g[RA][0] = sx(0x001b_a4a4);
        c.g[A0][0] = c.g[S1][0];
        callee::CREATE_STEP2_SET_DAY.call(c, 0x001b_a4a4)?;
        c.g[RA][0] = sx(0x001b_a4ac);
        c.g[A0][0] = c.g[S1][0];
        callee::CREATE_STEP2_SET_YEAR.call(c, 0x001b_a4ac)?;
        c.g[RA][0] = sx(0x001b_a4b4);
        c.g[A0][0] = c.g[S1][0];
        callee::CREATE_STEP2_SET_GENDER.call(c, 0x001b_a4b4)?;
        c.g[RA][0] = sx(0x001b_a4bc);
        c.g[A0][0] = c.g[S1][0];
        callee::CREATE_STEP2_SET_LEGAL.call(c, 0x001b_a4bc)?;
        c.g[RA][0] = sx(0x001b_a4c4);
        c.g[A0][0] = c.g[S1][0];
        callee::NET_FE_STATE_INIT_3.call(c, 0x001b_a4c4)?;
        c.g[V0][0] = 1;
        frame.restore(c, 16, 0x20);
        Addr(p(r(c, S1), 0x6d4)).sw(c, c.g[V0][0]);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// FE state constructors.

/// `cFEStateChatRoom* cFEStateChatRoom_ctor(cFEStateChatRoom* self, int a1)`
/// (0x1b1f88): run the net FE-state constructor, retag the vtable, clear the
/// two item pairs and the four colour floats (1,1,1,1), then copy the global
/// player name into +0x72c.
pub fn cfe_state_chat_room_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        open(c, 0x30);
        c.g[A2][0] = 0;
        c.sq(16, at_6(c.g[SP][0] as u32, 0x20));
        c.sd(at_6(c.g[SP][0] as u32, 0x10), c.g[RA][0]);
        c.g[RA][0] = sx(0x001b_1fa0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        call_ptr(c, 0x001a_8500, 0x001b_1fa0)?;
        c.g[V1][0] = 0x470000;
        c.g[V0][0] = 0x3d;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x51a8));
        Addr(this as u32).off(0xc).sw(c, c.g[V0][0]);
        Addr(this as u32).off(8).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx((this as u32).wrapping_add(0x704));
        Addr(this as u32).off(0x6f8).sw(c, 0);
        c.g[V1][0] = 1;
        loop {
            c.cycles += 7;
            Addr((c.g[V0][0] as u32).wrapping_sub(8)).sw(c, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            Addr(c.g[V0][0] as u32).sw(c, 0);
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001b_1fc0);
            }
        }
        c.cycles += 26;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x70c));
        Addr(this as u32).off(0x6ec).sw(c, 0);
        c.swc1(0, at_6(c.g[SP][0] as u32, 0xc));
        c.swc1(0, c.g[SP][0] as u32);
        c.swc1(0, at_6(c.g[SP][0] as u32, 4));
        c.swc1(0, at_6(c.g[SP][0] as u32, 8));
        Addr(this as u32).off(0x6f0).sw(c, 0);
        Addr(this as u32).off(0x6f4).sw(c, 0);
        c.ldl(2, at_6(c.g[SP][0] as u32, 7));
        c.ldr(2, c.g[SP][0] as u32);
        c.ldl(3, at_6(c.g[SP][0] as u32, 0xf));
        c.ldr(3, at_6(c.g[SP][0] as u32, 8));
        c.sdl(2, at_6(this as u32, 0x733));
        c.sdr(2, at_6(this as u32, 0x72c));
        c.sdl(3, at_6(this as u32, 0x73b));
        c.sdr(3, at_6(this as u32, 0x734));
        c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0xc8));
        c.g[RA][0] = sx(0x001b_2030);
        c.g[A1][0] = Addr(c.g[V0][0] as u32).off(0x80).lw(c);
        call_ptr(c, 0x0041_66f4, 0x001b_2030)?;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld(at_6(c.g[SP][0] as u32, 0x10));
        c.lq(16, at_6(c.g[SP][0] as u32, 0x20));
        close(c, 0x30);
        Ok(())
    })
}


/// A `gp_17`-relative global.
#[inline(always)]
pub(crate) fn gp_17(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}


/// `lw_7` (sign-extended) of `a + off`.
#[inline(always)]
pub(crate) fn lw_7(c: &mut Cpu, a: u32, off: u32) -> u64 {
    Addr(p(a, off)).lw(c)
}


/// `lwc1 fN, a+off`.
#[inline(always)]
pub(crate) fn ldf(c: &mut Cpu, n: usize, a: u32, off: u32) -> FpuF32 {
    let v = Addr(p(a, off)).read_fpu(c);
    c.f[n] = v.0;
    v
}


/// FPU register `n`.
#[inline(always)]
pub(crate) fn fr(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}


// ---------------------------------------------------------------------------
// `TagFieldSetBinary` (0x003ec300).
//
// `int TagFieldSetBinary(Record* r, int len, int, const u8* src, int size)`:
// append the field name, '$', then the two-character hex encoding of every
// source byte, NUL-terminate, and store the raw text in the record.

pub(crate) const HEX_HIGH: u32 = 0x0049_5fc0;
 // D_00495FC0: high nibble -> character
pub(crate) const HEX_LOW: u32 = 0x0049_60c0;


// ---------------------------------------------------------------------------
// `NetFEState_OnSessionLostPopupEvent` (0x001a9578).
//
// `int NetFEState::OnSessionLostPopupEvent(NetFEState* self, int, int key)`:
// keys 0xf/0x14 close the messenger and move on to the chat/online menu,
// everything else forwards to the default child handler.

pub fn net_fe_state_on_session_lost_popup_event(c: &mut Cpu) {
    guest(c, |c| {

        'b_1a962c: {
        'b_1a9620: {
        'b_1a959c: {
        c.cycles += 6;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.g[V0][0] = 0xf;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        let t = c.g[A2][0] == c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if t {
        break 'b_1a959c;
        } else {
        c.cycles += 3;
        c.g[V0][0] = 0x14;
        let t = c.g[A2][0] != c.g[V0][0];

        if t {
        break 'b_1a9620;
        } else {
        break 'b_1a959c;
        }
        }
        }
        'b_1a9610: {
        'b_1a95ec: {
        'b_1a95e8: {
        c.cycles += 6;
        c.g[RA][0] = sx(0x001a95a4);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001a93d0, 0x001a95a4)?;
        c.g[RA][0] = sx(0x001a95ac);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        call_ptr(c, 0x0025da90, 0x001a95ac)?;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1d98));
        if t {
        break 'b_1a95e8;
        } else {
        c.cycles += 2;
        let t = c.g[V0][0] != 0u64;
        c.g[A1][0] = 0x460000;
        if t {
        break 'b_1a95ec;
        } else {
        c.cycles += 11;
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = 0x73c;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf0));
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x001a95d4);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x001a95d4)?;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x001a95e0);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x001b1f88, 0x001a95e0)?;
        let t = true;
        c.g[A0][0] = c.g[S0][0];
        if t {
        break 'b_1a9610;
        } else {
        break 'b_1a95e8;
        }
        }
        }
        }
        c.cycles += 1;
        c.g[A1][0] = 0x460000;
        break 'b_1a95ec;
        }
        c.cycles += 9;
        c.g[A0][0] = 0x6d0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd8));
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x001a9600);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x001a9600)?;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x001a960c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x001bfbb8, 0x001a960c)?;
        c.g[A0][0] = c.g[S0][0];
        break 'b_1a9610;
        }
        c.cycles += 4;
        c.g[RA][0] = sx(0x001a9618);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x001a8a40, 0x001a9618)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        if t {
        break 'b_1a962c;
        } else {
        break 'b_1a9620;
        }
        }
        c.cycles += 3;
        c.g[RA][0] = sx(0x001a9628);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001a93d0, 0x001a9628)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        break 'b_1a962c;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t; return Ok(());

        Ok(())
    })
}


// ---------------------------------------------------------------------------
// `cFEUseExistingAccount_OnAbortPopupEvent` (0x001aca88).
//
// `int cFEUseExistingAccount::OnAbortPopupEvent(self, widget, key)`: 0xf/0x14
// close the network messenger popups, everything else goes to the default
// child event handler.

pub fn fe_use_existing_account_on_abort_popup_event(c: &mut Cpu) {
    guest(c, |c| {

        'b_1acb34: {
        'b_1acb24: {
        'b_1acabc: {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[V0][0] = 0xf;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A2][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        let t = c.g[S0][0] == c.g[V0][0];
        c.g[S2][0] = c.g[A1][0];
        if t {
        break 'b_1acabc;
        } else {
        c.cycles += 3;
        c.g[V0][0] = 0x14;
        let t = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        if t {
        break 'b_1acb24;
        } else {
        break 'b_1acabc;
        }
        }
        }
        'b_1acad0: {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        let t = c.g[V0][0] != 0u64;

        if t {
        break 'b_1acad0;
        } else {
        c.cycles += 2;
        c.g[RA][0] = sx(0x001acad0);
        call_ptr(c, 0x00260f80, 0x001acad0)?;
        break 'b_1acad0;
        }
        }
        'b_1acaec: {
        c.cycles += 4;
        c.g[RA][0] = sx(0x001acad8);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        call_ptr(c, 0x00261460, 0x001acad8)?;
        let t = c.g[V0][0] != 0u64;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        if t {
        break 'b_1acaec;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x001acae8);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        call_ptr(c, 0x002613c8, 0x001acae8)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        break 'b_1acaec;
        }
        }
        'b_1acafc: {
        c.cycles += 2;
        let t = c.g[V0][0] != 0u64;

        if t {
        break 'b_1acafc;
        } else {
        c.cycles += 2;
        c.g[RA][0] = sx(0x001acafc);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        call_ptr(c, 0x0025b800, 0x001acafc)?;
        break 'b_1acafc;
        }
        }
        c.cycles += 10;
        c.g[RA][0] = sx(0x001acb04);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        call_ptr(c, 0x00261408, 0x001acb04)?;
        c.g[RA][0] = sx(0x001acb0c);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        call_ptr(c, 0x0025b800, 0x001acb0c)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001acb1c);
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, 0x001a97b8, 0x001acb1c)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        if t {
        break 'b_1acb34;
        } else {
        break 'b_1acb24;
        }
        }
        c.cycles += 4;
        c.g[A1][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001acb30);
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, 0x001a97b8, 0x001acb30)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        break 'b_1acb34;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t; return Ok(());

        Ok(())
    })
}


// ---------------------------------------------------------------------------
// `cFEStateLogin_v33` (0x001bf378): the same popup-close sequence as
// `cFEUseExistingAccount::OnAbortPopupEvent`.
pub fn fe_state_login_v33(c: &mut Cpu) {
    guest(c, |c| {

        'b_1bf424: {
        'b_1bf414: {
        'b_1bf3ac: {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[V0][0] = 0xf;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A2][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        let t = c.g[S0][0] == c.g[V0][0];
        c.g[S2][0] = c.g[A1][0];
        if t {
        break 'b_1bf3ac;
        } else {
        c.cycles += 3;
        c.g[V0][0] = 0x14;
        let t = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        if t {
        break 'b_1bf414;
        } else {
        break 'b_1bf3ac;
        }
        }
        }
        'b_1bf3c0: {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        let t = c.g[V0][0] != 0u64;

        if t {
        break 'b_1bf3c0;
        } else {
        c.cycles += 2;
        c.g[RA][0] = sx(0x001bf3c0);
        call_ptr(c, 0x00260f80, 0x001bf3c0)?;
        break 'b_1bf3c0;
        }
        }
        'b_1bf3dc: {
        c.cycles += 4;
        c.g[RA][0] = sx(0x001bf3c8);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        call_ptr(c, 0x00261460, 0x001bf3c8)?;
        let t = c.g[V0][0] != 0u64;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        if t {
        break 'b_1bf3dc;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x001bf3d8);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        call_ptr(c, 0x002613c8, 0x001bf3d8)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        break 'b_1bf3dc;
        }
        }
        'b_1bf3ec: {
        c.cycles += 2;
        let t = c.g[V0][0] != 0u64;

        if t {
        break 'b_1bf3ec;
        } else {
        c.cycles += 2;
        c.g[RA][0] = sx(0x001bf3ec);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        call_ptr(c, 0x0025b800, 0x001bf3ec)?;
        break 'b_1bf3ec;
        }
        }
        c.cycles += 10;
        c.g[RA][0] = sx(0x001bf3f4);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        call_ptr(c, 0x00261408, 0x001bf3f4)?;
        c.g[RA][0] = sx(0x001bf3fc);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        call_ptr(c, 0x0025b800, 0x001bf3fc)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001bf40c);
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, 0x001a97b8, 0x001bf40c)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        if t {
        break 'b_1bf424;
        } else {
        break 'b_1bf414;
        }
        }
        c.cycles += 4;
        c.g[A1][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001bf420);
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, 0x001a97b8, 0x001bf420)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        break 'b_1bf424;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t; return Ok(());

        Ok(())
    })
}


// ---------------------------------------------------------------------------
// `cFEStateStats_FormatTime` (0x001c4f58).
//
// `void FormatTime(void* self, char* buf, int frames)`: format a frame count
// as "h:mm:ss.mmm" (or "m:ss.mmm" when under an hour).

pub fn fe_state_stats_format_time(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.mtc1(6, 0);
        c.cvt_s_w(0, 0);
        ldf(c, 1, gp_17(c, -0x590c), 0);
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x3c;
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x3c;
        c.mul_s(0, 0, 1);
        c.cycles += 22;
        c.g[AT][0] = 0x447a0000;
        c.mtc1(1, 2);
        let buf = c.g[A1][0];
        c.g[T2][0] = buf;
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);
        c.g[T0][0] = c.g[V0][0];
        c.mtc1(2, 1);
        c.cvt_s_w(1, 1);
        c.div(0, 8, 3);
        c.sub_s(0, 0, 1);
        c.mul_s(0, 0, 2);
        c.cvt_w_s(1, 0);
        c.mfc1(9, 1);
        c.g[A0][0] = c.lo[0];
        c.g[A3][0] = c.g[A0][0];
        c.div(0, 7, 3);
        c.g[A2][0] = c.lo[0];
        c.mult(0, 2, 7, 3);
        c.mult(0, 3, 6, 3);
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        let hours = c.g[A2][0] as i64;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        if hours > 0 {
            c.cycles += 6;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = c.g[T2][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f00));
            call_ptr(c, 0x0041_7828, 0x001c_4fe8)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 8;
            c.g[A2][0] = c.g[A3][0];
            c.g[A1][0] = 0x460000;
            c.g[A3][0] = c.g[T0][0];
            c.g[A0][0] = c.g[T2][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f18));
            c.g[T0][0] = c.g[T1][0];
            call_ptr(c, 0x0041_7828, 0x001c_500c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// `cFEStateNetMessenger_SubmitKeyboardInput` (0x001a4750).
//
// `void cFEStateNetMessenger::SubmitKeyboardInput(self)`: if the messenger
// is in the "typing" or "waiting" state, submit the text through the active
// dialog.

pub fn fe_state_net_messenger_submit_keyboard_input(c: &mut Cpu) {
    guest(c, |c| {

        'b_1a4804: {
        'b_1a4800: {
        'b_1a479c: {
        'b_1a4788: {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V0][0] = 0x1;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x69c));
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x2;
        if t {
        break 'b_1a4788;
        } else {
        c.cycles += 2;
        let t = c.g[V1][0] == c.g[V0][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        if t {
        break 'b_1a479c;
        } else {
        c.cycles += 2;
        let t = true;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        if t {
        break 'b_1a4804;
        } else {
        break 'b_1a4788;
        }
        }
        }
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001a4794);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x74));
        call_ptr(c, 0x001a4810, 0x001a4794)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        if t {
        break 'b_1a4800;
        } else {
        break 'b_1a479c;
        }
        }
        'b_1a47b0: {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        let t = c.g[V0][0] != 0u64;

        if t {
        break 'b_1a47b0;
        } else {
        c.cycles += 2;
        c.g[RA][0] = sx(0x001a47b0);
        call_ptr(c, 0x00260f80, 0x001a47b0)?;
        break 'b_1a47b0;
        }
        }
        'b_1a47cc: {
        c.cycles += 4;
        c.g[RA][0] = sx(0x001a47b8);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        call_ptr(c, 0x00261460, 0x001a47b8)?;
        let t = c.g[V0][0] != 0u64;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        if t {
        break 'b_1a47cc;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x001a47c8);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        call_ptr(c, 0x002613c8, 0x001a47c8)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        break 'b_1a47cc;
        }
        }
        'b_1a47e0: {
        c.cycles += 2;
        let t = c.g[V0][0] != 0u64;
        c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        if t {
        break 'b_1a47e0;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x001a47dc);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        call_ptr(c, 0x0025b800, 0x001a47dc)?;
        c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        break 'b_1a47e0;
        }
        }
        c.cycles += 8;
        c.g[RA][0] = sx(0x001a47e8);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001a98b0, 0x001a47e8)?;
        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x69c));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001a47fc);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x74));
        call_ptr(c, 0x00262690, 0x001a47fc)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        break 'b_1a4800;
        }
        c.cycles += 1;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        break 'b_1a4804;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t; return Ok(());

        Ok(())
    })
}


// ---------------------------------------------------------------------------
// `LobbyMan_GetErrorString` (0x00260e50).
//
// `char* LobbyMan_GetErrorString(LobbyMan* self, char* buf, int code)`:
// decode the packed error/status word into text.

pub(crate) const MSG_LOBBY_DUPLICATE: u32 = 0x6475_706c;
 // "udpl"
pub(crate) const MSG_LOBBY_ITEM: u32 = 0x7469_6d65;
 // "item"

pub fn lobby_man_get_error_string(c: &mut Cpu) {
    guest(c, |c| {

        'b_260f00: {
        'b_260eb8: {
        'b_260ea0: {
        'b_260e88: {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V0][0] = 0x64750000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[T1][0] = c.g[A2][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[V0][0] = c.g[V0][0] | 0x706c;
        let t = c.g[T1][0] == c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if t {
        break 'b_260e88;
        } else {
        c.cycles += 4;
        c.g[V0][0] = 0x74690000;
        c.g[V0][0] = c.g[V0][0] | 0x6d65;
        let t = c.g[T1][0] == c.g[V0][0];
        c.g[A2][0] = c.g[T1][0];
        if t {
        break 'b_260ea0;
        } else {
        c.cycles += 2;
        let t = true;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xda0));
        if t {
        break 'b_260eb8;
        } else {
        break 'b_260e88;
        }
        }
        }
        }
        c.cycles += 6;
        c.g[A1][0] = 0x480000;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x00260e98);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe10));
        call_ptr(c, 0x003182d8, 0x00260e98)?;
        let t = true;
        c.g[V0][0] = c.g[S0][0];
        if t {
        break 'b_260f00;
        } else {
        break 'b_260ea0;
        }
        }
        c.cycles += 6;
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x00260eb0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3320));
        call_ptr(c, 0x003182d8, 0x00260eb0)?;
        let t = true;
        c.g[V0][0] = c.g[S0][0];
        if t {
        break 'b_260f00;
        } else {
        break 'b_260eb8;
        }
        }
        c.cycles += 18;
        c.g[T0][0] = sx((c.g[T1][0] as u32) >> 16);
        c.g[A1][0] = 0x480000;
        c.g[T1][0] = sx((c.g[T1][0] as u32) >> 8);
        c.g[A0][0] = c.g[SP][0];
        c.g[T0][0] = c.g[T0][0] & 0xff;
        c.g[T1][0] = c.g[T1][0] & 0xff;
        c.g[A3][0] = sx((c.g[A2][0] as u32) >> 24);
        c.g[T2][0] = c.g[A2][0] & 0xff;
        c.sw((c.g[SP][0] as u32), c.g[V0][0]);
        c.g[RA][0] = sx(0x00260ee4);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe20));
        call_ptr(c, 0x003194a8, 0x00260ee4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x00260ef0);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x00317fe8, 0x00260ef0)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x00260efc);
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x00318260, 0x00260efc)?;
        c.g[V0][0] = c.g[S0][0];
        break 'b_260f00;
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t; return Ok(());

        Ok(())
    })
}


// ---------------------------------------------------------------------------
// `cSetupMan_ClientRecvMsg0` (0x00269240) and `cSetupMan_ClientRecvMsg2`
// (0x00269300): store the received remote character and queue a reply.

/// The shared reply tail: link the freshly allocated packet at `a1` into the
/// pending queue and restore the frame.
pub(crate) fn client_recv_reply(c: &mut Cpu) {
    c.cycles += 15;
    c.g[V0][0] = lw_7(c, sp(c), 0x10);
    let a16 = sp(c).wrapping_add(0x50);
    c.lq(16, a16);
    sw(c, c.g[A1][0] as u32, 0, c.g[V0][0]);
    let a17 = sp(c).wrapping_add(0x40);
    let v0 = c.g[V0][0] as u32;
    c.g[V1][0] = lw_7(c, v0, 4);
    let a18 = sp(c).wrapping_add(0x30);
    c.lq(18, a18);
    sw(c, c.g[A1][0] as u32, 4, c.g[V1][0]);
    c.g[A0][0] = lw_7(c, v0, 4);
    sw(c, c.g[A0][0] as u32, 0, c.g[A1][0]);
    c.g[V0][0] = lw_7(c, sp(c), 0x10);
    sw(c, c.g[V0][0] as u32, 4, c.g[A1][0]);
    sw(c, sp(c), 0, c.g[A1][0]);
    let _ = a17;
}


pub(crate) fn recv_msg(c: &mut Cpu, reply: u64, character: u64, ret1: u32, ret2: u32, ret3: u32) -> Flow {
    c.cycles += 33;
    let frame = Frame::open(c, 0x60);
    frame.save(c, 16, 0x50);
    frame.save(c, 17, 0x40);
    c.g[S0][0] = reply;
    frame.save(c, 18, 0x30);
    frame.save_ra(c, 0x20);
    let this = c.g[A0][0] as u32;
    sw(c, this, 0, c.g[A2][0]);
    c.g[A2][0] = character;
    call_ptr(c, 0x0026_4d80, ret1)?;
    c.g[A1][0] = 0x480000;
    let s1 = Addr(gp_17(c, -0xc8)).lw(c) as u32;
    c.g[S1][0] = sx(s1);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
    c.g[A0][0] = 0x8;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, ret2)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[A1][0] = 0x480000;
    c.g[V0][0] = 0x480000;
    sw(c, c.g[S2][0] as u32, 0, c.g[S0][0]);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
    sw(c, c.g[S2][0] as u32, 4, c.g[V0][0]);
    c.g[A0][0] = 0xc;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    c.g[V0][0] = lw_7(c, s1, 0xf0);
    sw(c, sp(c), 0x10, c.g[V0][0]);
    call_ptr(c, 0x0031_7d70, ret3)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
    if c.g[V0][0] != 0 {
        sw(c, c.g[A1][0] as u32, 8, c.g[S2][0]);
    }
    client_recv_reply(c);
    let a17 = sp(c).wrapping_add(0x40);
    c.lq(17, a17);
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}


pub fn setup_man_client_recv_msg0(c: &mut Cpu) {
    guest(c, |c| {

        'b_2692c4: {
        c.cycles += 33;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S0][0] = 0xf3;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.sw((c.g[A0][0] as u32), c.g[A2][0]);
        c.g[RA][0] = sx(0x00269264);
        c.g[A2][0] = c.g[A3][0];
        call_ptr(c, 0x00264d80, 0x00269264)?;
        c.g[A1][0] = 0x480000;
        c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A0][0] = 0x8;
        c.g[A2][0] = 0x20000000;
        c.g[RA][0] = sx(0x00269280);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x00269280)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = 0x480000;
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S2][0] as u32), c.g[S0][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xf0));
        c.g[RA][0] = sx(0x002692b4);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        call_ptr(c, 0x00317d70, 0x002692b4)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
        if c.g[V0][0] != 0u64 {
        c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S2][0]);
        break 'b_2692c4;
        } else {
        break 'b_2692c4;
        }
        }
        c.cycles += 15;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sw((c.g[A1][0] as u32), c.g[V0][0]);
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.sw((c.g[A0][0] as u32), c.g[A1][0]);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
        c.sw((c.g[SP][0] as u32), c.g[A1][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t; return Ok(());

        Ok(())
    })
}


pub fn setup_man_client_recv_msg2(c: &mut Cpu) {
    guest(c, |c| {

        'b_269384: {
        c.cycles += 33;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S0][0] = 0xf5;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.sw((c.g[A0][0] as u32), c.g[A2][0]);
        c.g[RA][0] = sx(0x00269324);
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x00264d80, 0x00269324)?;
        c.g[A1][0] = 0x480000;
        c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A0][0] = 0x8;
        c.g[A2][0] = 0x20000000;
        c.g[RA][0] = sx(0x00269340);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x00269340)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = 0x480000;
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S2][0] as u32), c.g[S0][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xf0));
        c.g[RA][0] = sx(0x00269374);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        call_ptr(c, 0x00317d70, 0x00269374)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
        if c.g[V0][0] != 0u64 {
        c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S2][0]);
        break 'b_269384;
        } else {
        break 'b_269384;
        }
        }
        c.cycles += 15;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sw((c.g[A1][0] as u32), c.g[V0][0]);
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.sw((c.g[A0][0] as u32), c.g[A1][0]);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
        c.sw((c.g[SP][0] as u32), c.g[A1][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t; return Ok(());

        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Helpers.

/// A `gp_18`-relative global.
#[inline(always)]
pub(crate) fn gp_18(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}


/// `gpr = 0xNN0000 + lo` as the `lui`/`addiu` pair in the generated code does
/// it (through `v0`, as the original does).
#[inline(always)]
pub(crate) fn lui_add(c: &mut Cpu, hi: u32, lo: u32) -> u64 {
    c.g[V0][0] = hi as u64;
    c.g[V0][0] = sx(hi.wrapping_add(lo));
    c.g[V0][0]
}


// ---------------------------------------------------------------------------

/// `cFEStateNetMessenger::~cFEStateNetMessenger()` (0x1a3ea0): restore the
/// vtable, release the captured child object through its vtable slot 0x100,
/// free the name-list buffer (one element is deleted directly, more through
/// `EA_MemFree`), destroy the three strings and chain to the base dtor.
pub fn cfe_state_net_messenger_dtor(c: &mut Cpu) {
    'b_1a3f24: {
        'b_1a3f20: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.g[V0][0] = 0x460000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7f28));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S1][0] = c.g[A1][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x100));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x104));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001a3edc);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            if call_ptr(c, t, 0x001a3edc).is_err() {
                return;
            }
            c.g[RA][0] = sx(0x001a3ee4);
            c.g[A0][0] = c.g[S0][0];
            c.pc = 0x001a5450;
            ee::s03::cFEStateNetMessenger_ClearNameList(c);
            if c.pc != 0x001a3ee4 {
                return;
            }
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6e8));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6f0));
            c.g[V1][0] = c.g[A0][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
            let t = c.g[A0][0] == 0u64;
            c.g[V1][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
            if t {
                break 'b_1a3f20;
            } else {
                'b_1a3f18: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] != c.g[V0][0];

                    if t {
                        break 'b_1a3f18;
                    } else {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001a3f10);
                        if call_ptr(c, 0x00317e50, 0x001a3f10).is_err() {
                            return;
                        }
                        let t = true;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6e0));
                        if t {
                            break 'b_1a3f24;
                        } else {
                            break 'b_1a3f18;
                        }
                    }
                }
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a3f20);
                if call_ptr(c, 0x00317e98, 0x001a3f20).is_err() {
                    return;
                }
                break 'b_1a3f20;
            }
        }
        c.cycles += 1;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6e0));
        break 'b_1a3f24;
    }
    c.cycles += 16;
    c.g[RA][0] = sx(0x001a3f2c);
    c.g[A1][0] = 0x2;
    if call_ptr(c, 0x00318260, 0x001a3f2c).is_err() {
        return;
    }
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6d4));
    c.g[RA][0] = sx(0x001a3f38);
    c.g[A1][0] = 0x2;
    if call_ptr(c, 0x00318260, 0x001a3f38).is_err() {
        return;
    }
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6d0));
    c.g[RA][0] = sx(0x001a3f44);
    c.g[A1][0] = 0x2;
    if call_ptr(c, 0x00318260, 0x001a3f44).is_err() {
        return;
    }
    c.g[A0][0] = c.g[S0][0];
    c.g[RA][0] = sx(0x001a3f50);
    c.g[A1][0] = c.g[S1][0];
    c.pc = 0x001a85d0;
    ee::s03::NetFEState_dtor(c);
    if c.pc != 0x001a3f50 {
        return;
    }
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = t;
    return;
}


/// `cFEUseExistingAccount::OnEmailPopupEvent(this, args, id)`
/// (0x1ac9c0): when the popup id is 0xf, fill the e-mail popup from the
/// logged-in account, show it, and connect the lobby if the address checks
/// out; otherwise just forward the event to the base state.
pub fn cfe_use_existing_account_on_email_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let this = abi::arg(c, 0);
        let args = abi::arg(c, 1);
        let id = abi::arg64(c, 2);
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = abi::arg64(c, 1);
        c.g[S1][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);

        if id == 0xf {
            c.cycles += 25;
            c.g[A2][0] = 0xf;
            c.g[S2][0] = 3;
            callee::NET_FE_STATE_ON_CHILD_EVENT_13.call(c, 0x001a_c9f0)?;

            // The e-mail widget's text: copy the account's address into it.
            c.g[A1][0] = Addr(args).off(0x2b0).lw(c);
            c.g[A0][0] = sx(this.wrapping_add(0x6dc));
            callee::C_BX_STRING_ASSIGN_CSTR.call(c, 0x001a_c9fc)?;

            callee::NET_FE_STATE_GET_POPUP.call(c, 0x001a_ca04)?;
            let popup = abi::r32(c, V0);
            c.g[S0][0] = c.g[V0][0];
            let hash = lui_add(c, 0x0046_0000, 0x2d48);
            Addr(popup).off(0xbc).sw(c, 0);
            c.g[A0][0] = hash;
            callee::STR_HASH.call(c, 0x001a_ca18)?;
            c.g[V1][0] = 0x810;
            Addr(popup).off(0xc0).sw(c, c.g[S2][0]);
            Addr(popup).off(0x164).sw(c, 0);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            Addr(popup).off(0xc4).sw(c, c.g[V0][0]);
            Addr(popup).off(0x150).sw(c, c.g[V1][0]);
            callee::NET_FE_STATE_SHOW_POPUP_5.call(c, 0x001a_ca38)?;

            c.g[A0][0] = c.g[S1][0];
            callee::C_FE_USE_EXISTING_ACCOUNT_CHECK_EMAIL.call(c, 0x001a_ca40)?;
            let checked = c.g[V0][0] == 0;
            c.lq(16, frame.at(c, 0x30));
            if !checked {
                c.cycles += 6;
                Addr(this).off(0x6d8).sw(c, c.g[S2][0]);
                c.g[A1][0] = 0;
                c.g[A0][0] = Addr(gp_18(c, -0xc8)).lw(c);
                callee::LOBBY_MAN_CONNECT.call(c, 0x001a_ca58)?;
                c.lq(16, frame.at(c, 0x30));
            }
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            callee::NET_FE_STATE_ON_CHILD_EVENT_13.call(c, 0x001a_ca6c)?;
            c.lq(16, frame.at(c, 0x30));
        }

        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateNetConfig::CheckCard(this)` (0x1b43a8): query the network
/// library's card state (v53/v55) and either load the config list or show
/// the card error popup (0x840/0x14 when there is no card, 0x840/2 when the
/// card is not readable).
pub fn cfe_state_net_config_check_card(c: &mut Cpu) {
    'b_1b445c: {
        'b_1b4420: {
            c.cycles += 20;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1578));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1a8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1ac));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x001b43d4);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            if call_ptr(c, t, 0x001b43d4).is_err() {
                return;
            }
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1578));
            c.g[S1][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[A1][0] as u32));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1bc));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x001b43f0);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            if call_ptr(c, t, 0x001b43f0).is_err() {
                return;
            }
            let t = c.g[V0][0] != 0u64;
            c.g[A0][0] = c.g[S0][0];
            if t {
                break 'b_1b4420;
            } else {
                c.cycles += 10;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x708), 0u64);
                c.g[RA][0] = sx(0x001b4404);
                c.g[A1][0] = 0xffffffffffffffff;
                c.pc = 0x001b5068;
                ee::s03::cFEStateNetConfig_SetListVisible(c);
                if c.pc != 0x001b4404 {
                    return;
                }
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x840;
                c.g[A2][0] = 0x14;
                c.g[RA][0] = sx(0x001b4418);
                c.g[A3][0] = 0xffffffffffffffff;
                c.pc = 0x001b4888;
                ee::s03::cFEStateNetConfig_ShowCardErrorPopup(c);
                if c.pc != 0x001b4418 {
                    return;
                }
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                if t {
                    break 'b_1b445c;
                } else {
                    break 'b_1b4420;
                }
            }
        }
        'b_1b4438: {
            c.cycles += 2;
            if c.g[S1][0] == 0u64 {
                c.sw((c.g[S0][0] as u32).wrapping_add(0x708), 0u64);
                break 'b_1b4438;
            } else {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001b4430);
                c.g[A0][0] = c.g[S0][0];
                c.pc = 0x001b4cd8;
                ee::s03::cFEStateNetConfig_LoadConfigList(c);
                if c.pc != 0x001b4430 {
                    return;
                }
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                if t {
                    break 'b_1b445c;
                } else {
                    break 'b_1b4438;
                }
            }
        }
        c.cycles += 9;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x001b4444);
        c.g[A1][0] = 0xffffffffffffffff;
        c.pc = 0x001b5068;
        ee::s03::cFEStateNetConfig_SetListVisible(c);
        if c.pc != 0x001b4444 {
            return;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x840;
        c.g[A2][0] = 0x2;
        c.g[RA][0] = sx(0x001b4458);
        c.g[A3][0] = 0xffffffffffffffff;
        c.pc = 0x001b4888;
        ee::s03::cFEStateNetConfig_ShowCardErrorPopup(c);
        if c.pc != 0x001b4458 {
            return;
        }
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        break 'b_1b445c;
    }
    c.cycles += 4;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = t;
    return;
}


/// `cFEStateNetQuickPlayPopup::OnOptionsPopupEvent(this, args, id)`
/// (0x1c1b88): for the confirm/cancel ids, close the options list, toggle the
/// options hint, re-label the hint text and forward the event; otherwise
/// forward it unchanged.
pub fn cfe_state_net_quick_play_popup_on_options_popup_event(c: &mut Cpu) {
    'b_1c1c38: {
        'b_1c1c28: {
            'b_1c1bbc: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.g[V0][0] = 0xf;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.g[A2][0];
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                let t = c.g[S1][0] == c.g[V0][0];
                c.g[S2][0] = c.g[A1][0];
                if t {
                    break 'b_1c1bbc;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x14;
                    let t = c.g[S1][0] != c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_1c1c28;
                    } else {
                        break 'b_1c1bbc;
                    }
                }
            }
            c.cycles += 27;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6e4));
            c.g[A1][0] = 0u64;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x6ec), 0u64);
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001c1bdc);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            if call_ptr(c, t, 0x001c1bdc).is_err() {
                return;
            }
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6ec));
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x001c1bec);
            c.g[A1][0] = c.g[A1][0] ^ 0x1;
            c.pc = 0x001c13e0;
            ee::s03::cFEStateNetQuickPlayPopup_SetOptionsHintVisible(c);
            if c.pc != 0x001c1bec {
                return;
            }
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x001c1bf8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
            if call_ptr(c, 0x00317670, 0x001c1bf8).is_err() {
                return;
            }
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6f4));
            c.g[RA][0] = sx(0x001c1c04);
            c.g[A1][0] = c.g[V0][0];
            if call_ptr(c, 0x003a1000, 0x001c1c04).is_err() {
                return;
            }
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x001c1c10);
            c.g[A1][0] = 0u64;
            c.pc = 0x001c17f8;
            ee::s03::cFEStateNetQuickPlayPopup_SetOptionsEnabled(c);
            if c.pc != 0x001c1c10 {
                return;
            }
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001c1c20);
            c.g[A2][0] = c.g[S1][0];
            c.pc = 0x001a97b8;
            ee::s03::NetFEState_OnChildEvent(c);
            if c.pc != 0x001c1c20 {
                return;
            }
            let t = true;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            if t {
                break 'b_1c1c38;
            } else {
                break 'b_1c1c28;
            }
        }
        c.cycles += 4;
        c.g[A1][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001c1c34);
        c.g[A2][0] = c.g[S1][0];
        c.pc = 0x001a97b8;
        ee::s03::NetFEState_OnChildEvent(c);
        if c.pc != 0x001c1c34 {
            return;
        }
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        break 'b_1c1c38;
    }
    c.cycles += 5;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = t;
    return;
}


/// `FEAsyncManager_RequestSessionTexture(this, id, slot, param)` (0x1a37f8):
/// format the request file name and mark the slot as pending.
pub fn fe_async_manager_request_session_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let this = abi::arg(c, 0);
        let id = abi::arg(c, 1);
        let slot = abi::arg(c, 2);
        let param = abi::arg(c, 3);
        let frame = Frame::open(c, 0x80);
        frame.save(c, 19, 0x40);
        frame.save(c, 20, 0x30);
        c.g[S3][0] = abi::arg64(c, 0);
        frame.save(c, 21, 0x20);
        c.g[S4][0] = abi::arg64(c, 1);
        frame.save(c, 22, 0x10);
        c.g[S5][0] = abi::arg64(c, 2);
        frame.save(c, 16, 0x70);
        c.g[S6][0] = abi::arg64(c, 3);
        frame.save(c, 17, 0x60);
        frame.save(c, 18, 0x50);
        let no_id = (c.g[S4][0] as i64) < 0;
        frame.save_ra(c, 0);
        if no_id {
            // fall through to the epilogue
        } else {
            c.cycles += 26;
            c.g[S0][0] = 0x11c;
            callee::BE_LIBRARY_GET_INSTANCE_4.call(c, 0x001a_3838)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0xa;
            callee::BE_LIBRARY_GET_INTERFACE_4.call(c, 0x001a_3844)?;
            c.g[S2][0] = 0x004a_0000;
            c.mult(0, 16, 21, 16);
            c.g[S1][0] = 0x004a_0000;
            c.g[S2][0] = sx(0x004a_0000u32.wrapping_add(0x1a08));
            c.g[S1][0] = sx(0x004a_0000u32.wrapping_add(0x1a10));
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S4][0];
            callee::BE_MISSION_INT_GET_SESSION_NAME.call(c, 0x001a_3864)?;
            c.g[T0][0] = 0x004a_0000;
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[V0][0];
            c.g[T0][0] = sx(0x004a_0000u32.wrapping_add(0x1a00));
            call_ptr(c, 0x0041_7828, 0x001a_3880)?; // sprintf
            c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A0][0] = c.g[S3][0];
            Addr(c.g[S0][0] as u32).off(0x108).sw(c, c.g[S6][0]);
            c.g[A1][0] = c.g[S5][0];
            c.g[A2][0] = 1;
            callee::FE_ASYNC_MANAGER_SET_STATE.call(c, 0x001a_3898)?;
        }

        c.cycles += 10;
        frame.restore(c, 16, 0x70);
        frame.restore(c, 17, 0x60);
        frame.restore(c, 18, 0x50);
        frame.restore(c, 19, 0x40);
        frame.restore(c, 20, 0x30);
        frame.restore(c, 21, 0x20);
        frame.restore(c, 22, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        let _ = (this, slot, param);
        Ok(())
    })
}


/// `cFEStateNetConfig_ShowCheckingCardPopup(this, name)` (0x1b47c0):
/// format the "checking card" line and show the 0x83F popup.
pub fn cfe_state_net_config_show_checking_card_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 50;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x70);
        c.g[A2][0] = abi::arg64(c, 1);
        frame.save(c, 18, 0x40);
        c.g[V0][0] = 0x004a_0000;
        c.g[S2][0] = abi::arg64(c, 0);
        frame.save(c, 16, 0x60);
        c.g[A0][0] = sp(c) as u64;
        frame.save(c, 17, 0x50);
        frame.save(c, 19, 0x30);
        c.g[A1][0] = sx(0x004a_0000u32.wrapping_add(0x12c0));
        frame.save_ra(c, 0x20);
        c.g[S3][0] = sx(this.wrapping_add(0x71c));
        callee::U_STR_SPRINTF_ASCII_FMT.call(c, 0x001b_47f4)?;

        c.g[V0][0] = Addr(gp_18(c, -0x848)).lw(c);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x3f58));
        c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x8c).lw(c);
        c.g[S1][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x20).lh(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        callee::STR_HASH.call(c, 0x001b_4814)?;
        c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x24).lw(c);
        c.g[A0][0] = c.g[S0][0];
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, target, 0x001b_4824)?;

        c.g[A2][0] = sp(c) as u64;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S3][0];
        callee::U_STR_SPRINTF.call(c, 0x001b_4834)?;

        c.g[A0][0] = c.g[S2][0];
        callee::NET_FE_STATE_GET_POPUP.call(c, 0x001b_483c)?;
        c.g[A2][0] = 1;
        c.g[V1][0] = 0x83f;
        Addr(abi::r32(c, V0)).off(0xc4).sw(c, c.g[S3][0]);
        c.g[A0][0] = c.g[S2][0];
        Addr(abi::r32(c, V0)).off(0x154).sw(c, c.g[A2][0]);
        c.g[A1][0] = c.g[V0][0];
        Addr(abi::r32(c, V0)).off(0x150).sw(c, c.g[V1][0]);
        Addr(abi::r32(c, V0)).off(0xbc).sw(c, 0);
        Addr(abi::r32(c, V0)).off(0xc0).sw(c, c.g[A2][0]);
        Addr(abi::r32(c, V0)).off(0x164).sw(c, 0);
        Addr(abi::r32(c, V0)).off(0x168).sw(c, 0);
        callee::NET_FE_STATE_SHOW_POPUP_5.call(c, 0x001b_486c)?;

        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateNetConfig_HandleLoadError(this, err)` (0x1b4fa0): map a network
/// load error to the config popup's message and hide the list.
pub fn cfe_state_net_config_handle_load_error(c: &mut Cpu) {
    'b_1b5050: {
        'b_1b5038: {
            'b_1b501c: {
                c.cycles += 6;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.g[V0][0] = 0xfffffffffffffff1;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                let t = c.g[A1][0] == c.g[V0][0];
                c.g[S0][0] = c.g[A0][0];
                if t {
                    break 'b_1b501c;
                } else {
                    'b_1b5004: {
                        'b_1b4fe8: {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[A1][0] as i64) < -14i64) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.g[V0][0] = 0xffffffffffffff38;
                                break 'b_1b4fe8;
                            } else {
                                c.cycles += 2;
                                let t = (c.g[A1][0] as i64) < 0;
                                c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1570));
                                if t {
                                    break 'b_1b5004;
                                } else {
                                    c.cycles += 2;
                                    if c.g[A1][0] != 0u64 {
                                        c.g[A0][0] = c.g[S0][0];
                                        break 'b_1b5050;
                                    } else {
                                        c.cycles += 5;
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                        c.g[RA][0] = sx(0x001b4fe0);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                        if call_ptr(c, 0x0039f698, 0x001b4fe0).is_err() {
                                            return;
                                        }
                                        let t = true;
                                        c.g[A0][0] = c.g[S0][0];
                                        if t {
                                            break 'b_1b5050;
                                        } else {
                                            break 'b_1b4fe8;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[A1][0] == c.g[V0][0];
                        c.g[V0][0] = ((c.g[A1][0] as i64) < -200i64) as u64;
                        if t {
                            break 'b_1b5038;
                        } else {
                            c.cycles += 2;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_1b5050;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[A1][0] as i64) < -21i64) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1570));
                                if t {
                                    break 'b_1b5050;
                                } else {
                                    break 'b_1b5004;
                                }
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x842;
                    c.g[RA][0] = sx(0x001b5014);
                    c.g[A2][0] = 0x18;
                    c.pc = 0x001b4888;
                    ee::s03::cFEStateNetConfig_ShowCardErrorPopup(c);
                    if c.pc != 0x001b5014 {
                        return;
                    }
                    let t = true;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_1b5050;
                    } else {
                        break 'b_1b501c;
                    }
                }
            }
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x841;
            c.g[A2][0] = 0x6;
            c.g[RA][0] = sx(0x001b5030);
            c.g[A3][0] = 0xffffffffffffffff;
            c.pc = 0x001b4888;
            ee::s03::cFEStateNetConfig_ShowCardErrorPopup(c);
            if c.pc != 0x001b5030 {
                return;
            }
            let t = true;
            c.g[A0][0] = c.g[S0][0];
            if t {
                break 'b_1b5050;
            } else {
                break 'b_1b5038;
            }
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x841;
        c.g[A2][0] = 0x14;
        c.g[RA][0] = sx(0x001b504c);
        c.g[A3][0] = 0u64;
        c.pc = 0x001b4888;
        ee::s03::cFEStateNetConfig_ShowCardErrorPopup(c);
        if c.pc != 0x001b504c {
            return;
        }
        c.g[A0][0] = c.g[S0][0];
        break 'b_1b5050;
    }
    c.cycles += 6;
    c.g[RA][0] = sx(0x001b5058);
    c.g[A1][0] = 0xffffffffffffffff;
    c.pc = 0x001b5068;
    ee::s03::cFEStateNetConfig_SetListVisible(c);
    if c.pc != 0x001b5058 {
        return;
    }
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = t;
    return;
}


/// `cFEStateNetQuickMessages_OnSavePopupEvent(this, args, id)` (0x1c2c50):
/// forward the save-popup event and open the matching next state.
pub fn cfe_state_net_quick_messages_on_save_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let this = abi::arg(c, 0);
        let args = abi::arg(c, 1);
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xf;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let is_f = c.g[A2][0] == c.g[V0][0];
        c.g[S0][0] = abi::arg64(c, 0);
        let mut next: u32 = 0;
        if !is_f {
            c.cycles += 3;
            c.g[V0][0] = 0x10;
            let is_10 = c.g[A2][0] == c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            if is_10 {
                next = 0x10;
            } else {
                c.cycles += 2;
                next = 0xdead;
            }
        }

        if next == 0 {
            // case 0xf
            c.cycles += 17;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = 0xf;
            callee::NET_FE_STATE_ON_CHILD_EVENT_13.call(c, 0x001c_2c88)?;
            c.g[A1][0] = 0x0046_0000;
            c.g[A0][0] = 0x23c;
            c.g[A1][0] = sx(0x0046_0000u32.wrapping_add(0xd90));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x001c_2ca0)?;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[T0][0] = 1;
            c.g[A2][0] = 2;
            c.g[A3][0] = 1;
            callee::C_FE_STATE_OPTIONS_DEVICE_SELECT_CTOR.call(c, 0x001c_2cb8)?;
        } else if next == 0x10 {
            c.cycles += 12;
            c.g[A2][0] = 0x10;
            callee::NET_FE_STATE_ON_CHILD_EVENT_13.call(c, 0x001c_2cc8)?;
            c.g[A1][0] = 0x0046_0000;
            c.g[A0][0] = 0x6d0;
            c.g[A1][0] = sx(0x0046_0000u32.wrapping_add(0x11a8));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x001c_2ce0)?;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
            c.g[A0][0] = c.g[V0][0];
            callee::C_FE_STATE_NET_SSX_WORLD_CTOR.call(c, 0x001c_2cec)?;
            c.g[A0][0] = c.g[S0][0];
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::NET_FE_STATE_ON_CHILD_EVENT_13.call(c, 0x001c_2d08)?;
            c.cycles += 3;
            frame.restore(c, 16, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 4;
        c.g[A1][0] = c.g[V0][0];
        callee::NET_FE_STATE_GOTO_NEXT_STATE.call(c, 0x001c_2cf8)?;
        c.lq(16, frame.at(c, 0x10));
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `a + off`.
#[inline(always)]
pub(crate) fn p_6(a: u64, off: u32) -> u32 {
    (a as u32).wrapping_add(off)
}


// ---------------------------------------------------------------------------
// Network front-end state machines.

/// `int cFEStateNetMessenger_GetBuddyStatus(cFEStateNetMessenger *self,
/// int index)` (0x1a4670): 0 if the buddy is not connected, 2 if it is not
/// online at that index, else 1.
pub fn cfe_state_net_messenger_get_buddy_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let have = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A1][0];
        if !have {
            c.cycles += 2;
            call_ptr(c, 0x0026_0f80, 0x001a_4690)?;
        }
        c.cycles += 4;
        c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
        call_ptr(c, 0x0026_1460, 0x001a_4698)?;
        let connected = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
        if !connected {
            c.cycles += 3;
            c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
            call_ptr(c, 0x0026_13c8, 0x001a_46a8)?;
            c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
        }
        c.cycles += 2;
        let have2 = c.g[V0][0] != 0;
        c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
        if !have2 {
            c.cycles += 3;
            c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
            call_ptr(c, 0x0025_b800, 0x001a_46bc)?;
            c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
        }
        c.cycles += 4;
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0026_2400, 0x001a_46c8)?;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                call_ptr(c, 0x0026_0f80, 0x001a_46e4)?;
            }
            c.cycles += 4;
            c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
            call_ptr(c, 0x0026_1460, 0x001a_46ec)?;
            let connected = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
            if !connected {
                c.cycles += 3;
                c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
                call_ptr(c, 0x0026_13c8, 0x001a_46fc)?;
                c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
            }
            c.cycles += 2;
            let have2 = c.g[V0][0] != 0;
            c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
            if !have2 {
                c.cycles += 3;
                c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
                call_ptr(c, 0x0025_b800, 0x001a_4710)?;
                c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
            }
            c.cycles += 6;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0026_3338, 0x001a_471c)?;
            c.g[A0][0] = 0x2;
            c.g[V1][0] = 0x1;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[A0][0];
            }
            c.g[V0][0] = c.g[V1][0];
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// The shared body of the two net-account substate updates: on substate 5,
/// drive the session through the `+0x6d4` sub-substate and set the next
/// state; otherwise do nothing. `final_state` is 9 (state 0x06) or 0x11
/// (state 0x0C).
pub(crate) fn net_account_mem_update(c: &mut Cpu, final_state: u64) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x20);
    c.g[A1][0] = Addr(gp(c, 0x2e0)).lw(c);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
    let is4 = (c.g[V1][0] ^ 0x4) != 0;
    c.g[S0][0] = c.g[A0][0];
    if !is4 {
        c.cycles += 4;
        c.g[A1][0] = 0x7;
        callee::NET_ACCOUNT_SET_STATE.call(c, 0x001b_7848)?;
        c.cycles += 3;
        c.lq(16, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[V1][0] ^ 0x5;
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.lq(16, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        return Ok(());
    }
    // Substate 5: look at `self+0x6d4`.
    c.cycles += 4;
    c.g[V1][0] = Addr(p_6(c.g[S0][0], 0x6d4)).lw(c);
    c.g[V0][0] = 0x2;
    if c.g[V1][0] == c.g[V0][0] {
        c.cycles += 4;
        c.g[A0][0] = c.g[A1][0];
        call_ptr(c, 0x0026_5cb0, 0x001b_7874)?;
        let no = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !no {
            c.cycles += 4;
            c.g[A1][0] = 0xd;
            callee::NET_ACCOUNT_SET_STATE.call(c, 0x001b_7884)?;
            c.cycles += 3;
            c.lq(16, frame.at(c, 0x10));
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        c.g[V1][0] = Addr(p_6(c.g[S0][0], 0x6d4)).lw(c);
        c.g[V0][0] = 0x1;
    } else {
        c.g[V0][0] = 0x1;
    }
    // `v1` is the sub-substate, `v0` = 1.
    c.cycles += 2;
    let not_one = c.g[V1][0] != c.g[V0][0];
    c.g[V0][0] = 0x5;
    if !not_one {
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x10;
        callee::NET_ACCOUNT_SET_STATE.call(c, 0x001b_78a8)?;
        c.cycles += 3;
        c.lq(16, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        return Ok(());
    }
    c.cycles += 2;
    let is_five = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = 0x6;
    let mut set_19 = is_five;
    if !is_five {
        c.cycles += 2;
        let not_six = c.g[V1][0] != c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        if not_six {
            // final state
            c.cycles += 3;
            c.g[A1][0] = final_state;
            callee::NET_ACCOUNT_SET_STATE.call(c, 0x001b_78dc)?;
            c.cycles += 3;
            c.lq(16, frame.at(c, 0x10));
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        set_19 = true;
    }
    if set_19 {
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x19;
        callee::NET_ACCOUNT_SET_STATE.call(c, 0x001b_78cc)?;
        c.cycles += 3;
        c.lq(16, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        return Ok(());
    }
    Ok(())
}


/// `void cFEStateNetAccountMem_UpdateState06(self)` (0x1b7820).
pub fn cfe_state_net_account_mem_update_state06(c: &mut Cpu) {
    guest(c, |c| net_account_mem_update(c, 9))
}


/// `void cFEStateNetAccountMem_UpdateState0C(self)` (0x1b7938).
pub fn cfe_state_net_account_mem_update_state0c(c: &mut Cpu) {
    guest(c, |c| net_account_mem_update(c, 0x11))
}


/// `addiu a, b, off` on a 64-bit register.
#[inline(always)]
pub(crate) fn sub(a: u64, off: u32) -> u64 {
    sx((a as u32).wrapping_sub(off))
}


/// `addu a, b` on 64-bit registers.
#[inline(always)]
pub(crate) fn addu(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}


/// `lw_8` at `a`.
#[inline(always)]
pub(crate) fn lw_8(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lw(c)
}


/// `sw_5 v, a`.
#[inline(always)]
pub(crate) fn sw_5(c: &mut Cpu, a: u32, v: u64) {
    c.sw(a, v);
}


/// `void cFEStateSelectPersona_OnWidgetCreated(cFEStateSelectPersona* self,
/// cUIText* text)`: fill the persona list for the title, blank the lock icon,
/// or set the name/description of the new text widget.
pub fn c_fe_state_select_persona_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        'b_1ad3d0: {
            'b_1ad358: {
                c.cycles += 15;
                c.g[SP][0] = sub(c.g[SP][0], 0x50);
                c.sq(18, p(c.g[SP][0] as u32, 0x20));
                c.sq(17, p(c.g[SP][0] as u32, 0x30));
                c.g[S2][0] = c.g[A0][0];
                c.sq(19, p(c.g[SP][0] as u32, 0x10));
                c.g[S1][0] = c.g[A1][0];
                c.sd(p(c.g[SP][0] as u32, 0), c.g[RA][0]);
                c.g[A0][0] = sx(select_persona::TITLE);
                c.sq(16, p(c.g[SP][0] as u32, 0x40));
                c.g[S0][0] = lw_8(c, p(c.g[S1][0] as u32, select_persona::HASH));
                c.g[S3][0] = lw_8(c, gp(c, -0xc8));
                call_ptr(c, addr::STR_HASH, 0x001a_d33c)?;
                let matched = c.g[S0][0] == c.g[V0][0];
                c.g[A0][0] = sx(0x0046_0000);
                if !matched {
                    break 'b_1ad358;
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x001a_d3e8, 0x001a_d350)?;
                c.lq(16, p(c.g[SP][0] as u32, 0x40));
                break 'b_1ad3d0;
            }
            'b_1ad3cc: {
                'b_1ad378: {
                    c.cycles += 5;
                    c.g[S0][0] = lw_8(c, p(c.g[S1][0] as u32, select_persona::HASH));
                    c.g[A0][0] = sx(select_persona::LOCKED);
                    call_ptr(c, addr::STR_HASH, 0x001a_d364)?;
                    let matched = c.g[S0][0] == c.g[V0][0];
                    c.g[A0][0] = sx(0x0046_0000);
                    if !matched {
                        break 'b_1ad378;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    sw_5(c, p(c.g[S1][0] as u32, 0x18), c.g[V0][0]);
                    break 'b_1ad3cc;
                }
                'b_1ad3a0: {
                    c.cycles += 5;
                    c.g[S0][0] = lw_8(c, p(c.g[S1][0] as u32, select_persona::HASH));
                    c.g[A0][0] = sx(select_persona::NAME);
                    call_ptr(c, addr::STR_HASH, 0x001a_d384)?;
                    let matched = c.g[S0][0] == c.g[V0][0];
                    c.g[A0][0] = sx(0x0046_0000);
                    if !matched {
                        break 'b_1ad3a0;
                    }
                    c.cycles += 5;
                    c.g[A1][0] = lw_8(c, p(c.g[S3][0] as u32, 0x48));
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, addr::UI_TEXT_SET_ASCII, 0x001a_d398)?;
                    c.lq(16, p(c.g[SP][0] as u32, 0x40));
                    break 'b_1ad3d0;
                }
                c.cycles += 5;
                c.g[S0][0] = lw_8(c, p(c.g[S1][0] as u32, select_persona::HASH));
                c.g[A0][0] = sx(select_persona::DESC);
                call_ptr(c, addr::STR_HASH, 0x001a_d3ac)?;
                let mismatched = c.g[S0][0] != c.g[V0][0];
                c.lq(16, p(c.g[SP][0] as u32, 0x40));
                if !mismatched {
                    c.cycles += 6;
                    c.g[A0][0] = sx(select_persona::DESC_ID);
                    call_ptr(c, addr::STR_HASH, 0x001a_d3c0)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, addr::UI_TEXT_SET_UNICODE, 0x001a_d3cc)?;
                    c.cycles += 1;
                    c.lq(16, p(c.g[SP][0] as u32, 0x40));
                }
                break 'b_1ad3cc;
            }
            c.cycles += 1;
            c.lq(16, p(c.g[SP][0] as u32, 0x40));
        }
        c.cycles += 6;
        c.lq(17, p(c.g[SP][0] as u32, 0x30));
        c.lq(18, p(c.g[SP][0] as u32, 0x20));
        c.lq(19, p(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    });
}


/// `void* NetConfigAuthPopup_ctor(NetConfigAuthPopup* self, void* a1,
/// void* a2)`: initialise the online popup state, then allocate the title
/// buffer from the measured widget text and return `self`.
pub fn net_config_auth_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        c.g[SP][0] = sub(c.g[SP][0], 0x40);
        c.sq(18, p(c.g[SP][0] as u32, 0x10));
        c.sq(16, p(c.g[SP][0] as u32, 0x30));
        c.sq(17, p(c.g[SP][0] as u32, 0x20));
        c.sd(p(c.g[SP][0] as u32, 0), c.g[RA][0]);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x001a_8500, 0x001b_62a4)?;
        c.g[V0][0] = 2;
        c.g[V1][0] = sx(0x0046_89f0);
        sw_5(c, p(c.g[S2][0] as u32, net_config_auth::F6F4), c.g[V0][0]);
        c.g[A2][0] = 0x44;
        c.g[A1][0] = 0x1e;
        sw_5(c, p(c.g[S2][0] as u32, net_config_auth::ID), c.g[A2][0]);
        c.g[A0][0] = sx(0x0046_4208);
        sw_5(c, p(c.g[S2][0] as u32, net_config_auth::VTABLE), c.g[V1][0]);
        sw_5(c, p(c.g[S2][0] as u32, net_config_auth::F6F8), c.g[A1][0]);
        sw_5(c, p(c.g[S2][0] as u32, net_config_auth::F6D0), 0);
        sw_5(c, p(c.g[S2][0] as u32, net_config_auth::F6D4), 0);
        sw_5(c, p(c.g[S2][0] as u32, net_config_auth::F6E4), 0);
        sw_5(c, p(c.g[S2][0] as u32, net_config_auth::BUF), 0);
        // Measure the title string of the current UI object through its vtable.
        c.g[V0][0] = lw_8(c, gp(c, -0x848));
        c.g[S0][0] = lw_8(c, p(c.g[V0][0] as u32, 0x8c));
        c.g[S1][0] = lw_8(c, p(c.g[S0][0] as u32, 4));
        c.g[V0][0] = c.lh(p(c.g[S1][0] as u32, 0x20));
        c.g[S0][0] = addu(c.g[S0][0], c.g[V0][0]);
        call_ptr(c, addr::STR_HASH, 0x001b_62f8)?;
        c.g[V1][0] = lw_8(c, p(c.g[S1][0] as u32, 0x24));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        let func = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001b_6308);
        c.pc = func;
        ssx3_ee_rt::funcs::call_indirect(c, func);
        if c.pc != 0x001b_6308 {
            return Err(crate::call::Unwound);
        }
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, addr::USTR_LENGTH, 0x001b_6310)?;
        c.g[A0][0] = lw_8(c, p(c.g[S2][0] as u32, net_config_auth::BUF));
        let empty = c.g[A0][0] == 0;
        c.g[S0][0] = c.g[V0][0];
        if !empty {
            c.cycles += 3;
            call_ptr(c, addr::MEM_FREE, 0x001b_6324)?;
            sw_5(c, p(c.g[S2][0] as u32, net_config_auth::BUF), 0);
        }
        c.cycles += 16;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(5));
        c.g[A1][0] = sx(0x0046_4220);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        call_ptr(c, addr::OPERATOR_NEW_NAMED, 0x001b_6344)?;
        sw_5(c, p(c.g[S2][0] as u32, net_config_auth::BUF), c.g[V0][0]);
        c.g[V0][0] = c.g[S2][0];
        sw_5(c, p(c.g[S2][0] as u32, net_config_auth::TITLE), 0);
        c.lq(16, p(c.g[SP][0] as u32, 0x30));
        c.lq(17, p(c.g[SP][0] as u32, 0x20));
        c.lq(18, p(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    });
}


/// `void cFEStateAuthDNASError_OnWidgetCreated(cFEStateAuthDNASError* self,
/// cUIWidget* obj)`: keep the status widgets, press the retry button when the
/// retry widget appears, or make the message text scroll.
pub fn c_fe_state_auth_dnas_error_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b6c84: {
            'b_1b6c80: {
                'b_1b6bf4: {
                    c.cycles += 13;
                    c.g[SP][0] = sub(c.g[SP][0], 0x40);
                    c.sq(18, p(c.g[SP][0] as u32, 0x10));
                    c.sq(17, p(c.g[SP][0] as u32, 0x20));
                    c.g[S2][0] = c.g[A0][0];
                    c.sd(p(c.g[SP][0] as u32, 0), c.g[RA][0]);
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, p(c.g[SP][0] as u32, 0x30));
                    c.g[A0][0] = sx(dnas_error::STATUS);
                    c.g[S0][0] = lw_8(c, p(c.g[S1][0] as u32, 0x38));
                    call_ptr(c, addr::STR_HASH, 0x001b_6be4)?;
                    if c.g[S0][0] == c.g[V0][0] {
                        c.cycles += 2;
                        sw_5(c, p(c.g[S2][0] as u32, 0x6dc), c.g[S1][0]);
                        break 'b_1b6c80;
                    }
                    c.g[A0][0] = sx(dnas_error::STATUS2);
                    break 'b_1b6bf4;
                }
                'b_1b6c10: {
                    c.cycles += 5;
                    c.g[S0][0] = lw_8(c, p(c.g[S1][0] as u32, 0x38));
                    c.g[A0][0] = sx(dnas_error::STATUS2);
                    call_ptr(c, addr::STR_HASH, 0x001b_6c00)?;
                    if c.g[S0][0] == c.g[V0][0] {
                        c.cycles += 2;
                        sw_5(c, p(c.g[S2][0] as u32, 0x6e0), c.g[S1][0]);
                        break 'b_1b6c80;
                    }
                    c.g[A0][0] = sx(dnas_error::STATUS3);
                    break 'b_1b6c10;
                }
                'b_1b6c2c: {
                    c.cycles += 5;
                    c.g[S0][0] = lw_8(c, p(c.g[S1][0] as u32, 0x38));
                    c.g[A0][0] = sx(dnas_error::STATUS3);
                    call_ptr(c, addr::STR_HASH, 0x001b_6c1c)?;
                    if c.g[S0][0] == c.g[V0][0] {
                        c.cycles += 2;
                        sw_5(c, p(c.g[S2][0] as u32, 0x6e4), c.g[S1][0]);
                        break 'b_1b6c80;
                    }
                    c.g[A0][0] = sx(0x0046_0000);
                    break 'b_1b6c2c;
                }
                'b_1b6c60: {
                    c.cycles += 5;
                    c.g[S0][0] = lw_8(c, p(c.g[S1][0] as u32, 0x38));
                    c.g[A0][0] = sx(dnas_error::RETRY);
                    call_ptr(c, addr::STR_HASH, 0x001b_6c38)?;
                    if c.g[S0][0] == c.g[V0][0] {
                        c.cycles += 8;
                        c.g[V0][0] = lw_8(c, p(c.g[S1][0] as u32, 8));
                        c.g[A1][0] = 1;
                        c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0x48));
                        c.g[V1][0] = lw_8(c, p(c.g[V0][0] as u32, 0x4c));
                        let func = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001b_6c58);
                        c.g[A0][0] = addu(c.g[S1][0], c.g[A0][0]);
                        c.pc = func;
                        ssx3_ee_rt::funcs::call_indirect(c, func);
                        if c.pc != 0x001b_6c58 {
                            return Err(crate::call::Unwound);
                        }
                        c.lq(16, p(c.g[SP][0] as u32, 0x30));
                        break 'b_1b6c84;
                    }
                    c.g[A0][0] = sx(0x0046_0000);
                    break 'b_1b6c60;
                }
                c.cycles += 5;
                c.g[S0][0] = lw_8(c, p(c.g[S1][0] as u32, 0x38));
                c.g[A0][0] = sx(dnas_error::MESSAGE);
                call_ptr(c, addr::STR_HASH, 0x001b_6c6c)?;
                let mismatched = c.g[S0][0] != c.g[V0][0];
                c.lq(16, p(c.g[SP][0] as u32, 0x30));
                if mismatched {
                    break 'b_1b6c84;
                }
                c.cycles += 3;
                c.g[V0][0] = lw_8(c, p(c.g[S1][0] as u32, 0x90));
                c.g[V0][0] = c.g[V0][0] | 8;
                sw_5(c, p(c.g[S1][0] as u32, 0x90), c.g[V0][0]);
                break 'b_1b6c80;
            }
            c.cycles += 1;
            c.lq(16, p(c.g[SP][0] as u32, 0x30));
        }
        c.cycles += 5;
        c.lq(17, p(c.g[SP][0] as u32, 0x20));
        c.lq(18, p(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// cFEStateLobbyMenu_EnterRoom (0x00001bd460).

/// `cFEStateLobbyMenu_EnterRoom`: open the room popup, store the lobby name (or clear it when empty) and copy the room name into the label.
pub fn c_fe_state_lobby_menu_enter_room(c: &mut Cpu) {
    guest(c, |c| {
    'b_1bd50c: {
    'b_1bd4f4: {
    'b_1bd4dc: {
    c.cycles += 26;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[S2][0] = c.g[A0][0];
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[S1][0] = c.g[A2][0];
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[S3][0] = c.g[A1][0];
    call_ptr(c, 0x001a9c50, 0x001bd488)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = 0x460000;
    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), 0u64);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4fa8));
    call_ptr(c, 0x00317670, 0x001bd49c)?;
    c.g[V1][0] = 0x3;
    c.g[A2][0] = 0x829;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x164), 0u64);
    c.g[A1][0] = c.g[S0][0];
    c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
    c.g[A0][0] = c.g[S2][0];
    c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[A2][0]);
    call_ptr(c, 0x001a9bc0, 0x001bd4c0)?;
    if c.g[S1][0] == 0u64 {
    c.sw((c.g[S2][0] as u32).wrapping_add(0x770), 0u64);
    break 'b_1bd4dc;
    } else {
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x00416810, 0x001bd4d0)?;
    let t = c.g[V0][0] != 0u64;
    c.g[V0][0] = 0x1;
    if t {
    break 'b_1bd4f4;
    } else {
    c.cycles += 1;
    c.sw((c.g[S2][0] as u32).wrapping_add(0x770), 0u64);
    break 'b_1bd4dc;
    }
    }
    }
    c.cycles += 6;
    c.g[A1][0] = c.g[S3][0];
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    c.g[A2][0] = 0u64;
    call_ptr(c, 0x0025d800, 0x001bd4ec)?;
    let t = true;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    if t {
    break 'b_1bd50c;
    } else {
    break 'b_1bd4f4;
    }
    }
    c.cycles += 6;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    c.sw((c.g[S2][0] as u32).wrapping_add(0x770), c.g[V0][0]);
    c.g[A2][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S3][0];
    call_ptr(c, 0x0025d800, 0x001bd508)?;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    break 'b_1bd50c;
    }
    c.cycles += 13;
    c.g[A1][0] = c.g[S3][0];
    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xb80));
    call_ptr(c, 0x0025f150, 0x001bd518)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x003184e8, 0x001bd524)?;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = t; return Ok(());
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// cFEStateOnlineMainMenu_OnLeavePopupEvent (0x00001c0138).

/// `cFEStateOnlineMainMenu_OnLeavePopupEvent`: forward a popup event; when the "join" popup (id 0xf) closes, queue the lobby entry and tear the old session down.
pub fn c_fe_state_online_main_menu_on_leave_popup_event(c: &mut Cpu) {
    guest(c, |c| {
    'b_1c020c: {
    'b_1c0208: {
    'b_1c0200: {
    c.cycles += 6;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    c.g[V0][0] = 0xf;
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    let t = c.g[A2][0] != c.g[V0][0];
    c.g[S0][0] = c.g[A0][0];
    if t {
    break 'b_1c0200;
    } else {
    c.cycles += 18;
    c.g[A2][0] = 0xf;
    call_ptr(c, 0x001a97b8, 0x001c0158)?;
    c.g[V0][0] = 0x1;
    c.g[A1][0] = 0x460000;
    c.g[A3][0] = 0u64;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xba8));
    c.g[A0][0] = 0x58;
    c.g[A2][0] = 0u64;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x6b0), c.g[V0][0]);
    call_ptr(c, 0x00317d70, 0x001c0178)?;
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x00194a60, 0x001c0184)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x001a8a40, 0x001c0190)?;
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    if t {
    break 'b_1c0208;
    } else {
    'b_1c01a8: {
    c.cycles += 2;
    let t = c.g[V0][0] != 0u64;

    if t {
    break 'b_1c01a8;
    } else {
    c.cycles += 2;
    call_ptr(c, 0x00260f80, 0x001c01a8)?;
    break 'b_1c01a8;
    }
    }
    'b_1c01c4: {
    c.cycles += 4;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    call_ptr(c, 0x00261460, 0x001c01b0)?;
    let t = c.g[V0][0] != 0u64;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    if t {
    break 'b_1c01c4;
    } else {
    c.cycles += 3;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    call_ptr(c, 0x002613c8, 0x001c01c0)?;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    break 'b_1c01c4;
    }
    }
    'b_1c01d4: {
    c.cycles += 2;
    let t = c.g[V0][0] != 0u64;

    if t {
    break 'b_1c01d4;
    } else {
    c.cycles += 2;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    call_ptr(c, 0x0025b800, 0x001c01d4)?;
    break 'b_1c01d4;
    }
    }
    c.cycles += 11;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    call_ptr(c, 0x00261408, 0x001c01dc)?;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    call_ptr(c, 0x0025b800, 0x001c01e4)?;
    call_ptr(c, 0x002591b8, 0x001c01ec)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0u64;
    call_ptr(c, 0x002636f0, 0x001c01f8)?;
    let t = true;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    if t {
    break 'b_1c020c;
    } else {
    break 'b_1c0200;
    }
    }
    }
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x001a97b8, 0x001c0208)?;
    break 'b_1c0208;
    }
    c.cycles += 1;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    break 'b_1c020c;
    }
    c.cycles += 3;
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = t; return Ok(());
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// PS2SoundManager_Init (0x00002a8cb0).

/// `PS2SoundManager_Init`: allocate the voice buffer, register it with the PS2 sound library and initialise the 32 voice slots.
pub fn ps2_sound_manager_init(c: &mut Cpu) {
    guest(c, |c| {
    c.cycles += 34;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
    c.g[A2][0] = 0u64;
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[A3][0] = 0u64;
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[S0][0] = c.g[A1][0];
    c.g[S1][0] = c.g[A0][0];
    c.g[A1][0] = 0x480000;
    c.g[A0][0] = 0x30000;
    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
    c.g[A0][0] = c.g[A0][0] | 0x2000;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3158));
    call_ptr(c, 0x00317e30, 0x002a8ce4)?;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    call_ptr(c, 0x003b5478, 0x002a8cec)?;
    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x002a8d04);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x002a8d04 { return Err(crate::call::Unwound); }
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
    c.g[A1][0] = 0x30000;
    c.g[A2][0] = 0x80000;
    c.g[A1][0] = c.g[A1][0] | 0x2000;
    c.g[A2][0] = c.g[A2][0] | 0x303;
    call_ptr(c, 0x003b5158, 0x002a8d1c)?;
    c.g[V1][0] = 0xffffffffffffffff;
    c.g[V0][0] = 0x1f;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x318), c.g[V1][0]);
    c.g[A1][0] = 0xffffffffffffffff;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x310), c.g[V0][0]);
    c.g[A0][0] = 0x1f;
    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x308));
    'l_2a8d38: loop {
    c.cycles += 10;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
    c.sw((c.g[V1][0] as u32).wrapping_sub(0x14), c.g[A0][0]);
    c.sw((c.g[V1][0] as u32).wrapping_sub(0x10), c.g[A1][0]);
    c.g[A0][0] = c.g[V0][0];
    c.sw((c.g[V1][0] as u32).wrapping_sub(0xc), c.g[A1][0]);
    c.sw((c.g[V1][0] as u32).wrapping_sub(0x8), 0u64);
    c.sw((c.g[V1][0] as u32).wrapping_sub(0x4), 0u64);
    c.sw((c.g[V1][0] as u32), c.g[V0][0]);
    let t = (c.g[A0][0] as i64) >= 0;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x18));
    if t {
    if event_due(c) { deliver_event_here(c, 0x002a8d38); } continue 'l_2a8d38;
    } else {
    c.cycles += 12;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x31c), 0u64);
    c.g[A0][0] = c.g[SP][0];
    c.sw((c.g[SP][0] as u32), 0u64);
    call_ptr(c, 0x003b5b68, 0x002a8d70)?;
    c.g[V1][0] = c.lw((c.g[SP][0] as u32));
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
    c.sw((c.g[S1][0] as u32).wrapping_add(0x30c), c.g[V1][0]);
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = t; return Ok(());
    }
    }
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// cBXString_ConcatInPlace (0x0000318830).

/// `cBXString_ConcatInPlace`: append a C string to a cBXString in place, growing the buffer when needed.
pub fn c_bx_string_concat_in_place(c: &mut Cpu) {
    guest(c, |c| {
    'b_3188f8: {
    'b_3188c4: {
    'b_318880: {
    c.cycles += 14;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[S1][0] = c.g[A0][0];
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[S2][0] = c.g[A1][0];
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[S3][0] = c.g[A2][0];
    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
    c.g[A3][0] = c.lw((c.g[S1][0] as u32));
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_sub(0xc));
    c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xc));
    if t {
    break 'b_318880;
    } else {
    c.cycles += 6;
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_sub(0x8));
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_sub(0x4));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[V1][0] == 0u64 {
    c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    break 'b_3188c4;
    } else {
    break 'b_318880;
    }
    }
    }
    c.cycles += 17;
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, 0x00317fe8, 0x0031888c)?;
    c.g[S0][0] = c.lw((c.g[S1][0] as u32));
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x003180b8, 0x00318898)?;
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x8));
    c.g[A0][0] = c.g[S1][0];
    c.g[A3][0] = c.g[S2][0];
    c.g[T0][0] = c.g[S3][0];
    c.g[A2][0] = c.g[S0][0];
    call_ptr(c, 0x00318540, 0x003188b0)?;
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x00318260, 0x003188bc)?;
    let t = true;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    if t {
    break 'b_3188f8;
    } else {
    break 'b_3188c4;
    }
    }
    c.cycles += 13;
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    call_ptr(c, 0x0041605c, 0x003188d4)?;
    c.g[A0][0] = c.lw((c.g[S1][0] as u32));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_sub(0x8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
    c.sw((c.g[A0][0] as u32).wrapping_sub(0x8), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sb((c.g[V1][0] as u32), 0u64);
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    break 'b_3188f8;
    }
    c.cycles += 6;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.pc = t; return Ok(());
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// PS2_SONY_CODEC_INTERNAL_InitFrame (0x00003b07f8).

/// `PS2_SONY_CODEC_INTERNAL_InitFrame`: initialise one Sony codec frame descriptor.
pub fn ps2_sony_codec_internal_init_frame(c: &mut Cpu) {
    guest(c, |c| {
    'b_3b08ac: {
    'b_3b08a8: {
    c.cycles += 41;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
    c.g[V1][0] = 0x3;
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = 0x2;
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[S2][0] = c.g[A0][0];
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[A3][0] = 0x510000;
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[S0][0] = c.g[A1][0];
    c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V1][0]);
    c.g[S1][0] = c.g[A2][0];
    c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
    c.g[A0][0] = c.g[S0][0];
    c.sw((c.g[S2][0] as u32), c.g[V0][0]);
    c.g[A1][0] = c.g[S1][0];
    c.sw((c.g[S2][0] as u32).wrapping_add(0x10), 0u64);
    c.g[A2][0] = 0x20;
    c.g[T0][0] = 0u64;
    c.g[T2][0] = 0u64;
    c.g[T1][0] = c.lw((c.g[A3][0] as u32).wrapping_sub(0x6bc8));
    c.g[T3][0] = 0xf0;
    c.g[A3][0] = 0u64;
    call_ptr(c, 0x003b2a68, 0x003b0858)?;
    c.g[A0][0] = c.g[V0][0];
    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    call_ptr(c, 0x003b3d40, 0x003b0864)?;
    c.g[A0][0] = 0x32;
    c.g[A1][0] = 0x34;
    c.g[V1][0] = 0x30;
    c.sb((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A0][0]);
    c.sb((c.g[V0][0] as u32).wrapping_add(0x6), c.g[V1][0]);
    c.g[S0][0] = sx((c.g[S0][0] as u32) >> 4);
    c.sb((c.g[V0][0] as u32).wrapping_add(0x5), c.g[A1][0]);
    c.mult(0, 16, 16, 17);
    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
    c.g[S0][0] = sx((c.g[S0][0] as u32) >> 4);
    c.g[V0][0] = c.g[V0][0] & 0x1000;
    let t = c.g[V0][0] == 0u64;
    c.g[A2][0] = sx((c.g[S0][0] as u32) << 10);
    if t {
    break 'b_3b08a8;
    } else {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
    let t = true;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    if t {
    break 'b_3b08ac;
    } else {
    break 'b_3b08a8;
    }
    }
    }
    c.cycles += 1;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
    break 'b_3b08ac;
    }
    c.cycles += 12;
    c.g[A1][0] = 0u64;
    call_ptr(c, 0x003e6448, 0x003b08b4)?;
    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.g[S2][0];
    c.g[V1][0] = 0x5;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    c.sb((c.g[A0][0] as u32), c.g[V1][0]);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = t; return Ok(());
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// NetFEState_v32 (0x00001a8830).

/// `NetFEState_v32`: net front-end state helper: refresh the connection status and popup.
pub fn net_fe_state_v32(c: &mut Cpu) {
    guest(c, |c| {
    'b_1a8910: {
    'b_1a88b4: {
    c.cycles += 9;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xe8));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xec));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[RA][0] = sx(0x001a884c);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x001a884c { return Err(crate::call::Unwound); }
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    if t {
    break 'b_1a88b4;
    } else {
    'b_1a8864: {
    c.cycles += 2;
    let t = c.g[V0][0] != 0u64;

    if t {
    break 'b_1a8864;
    } else {
    c.cycles += 2;
    call_ptr(c, 0x00260f80, 0x001a8864)?;
    break 'b_1a8864;
    }
    }
    'b_1a8880: {
    c.cycles += 4;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    call_ptr(c, 0x00261460, 0x001a886c)?;
    let t = c.g[V0][0] != 0u64;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    if t {
    break 'b_1a8880;
    } else {
    c.cycles += 3;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    call_ptr(c, 0x002613c8, 0x001a887c)?;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    break 'b_1a8880;
    }
    }
    'b_1a8894: {
    c.cycles += 2;
    let t = c.g[V0][0] != 0u64;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    if t {
    break 'b_1a8894;
    } else {
    c.cycles += 3;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    call_ptr(c, 0x0025b800, 0x001a8890)?;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    break 'b_1a8894;
    }
    }
    c.cycles += 8;
    c.g[A1][0] = 0x1;
    c.g[A2][0] = 0u64;
    c.g[A3][0] = 0u64;
    c.g[T0][0] = 0u64;
    c.g[T1][0] = 0u64;
    call_ptr(c, 0x00262768, 0x001a88ac)?;
    let t = true;
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    if t {
    break 'b_1a8910;
    } else {
    break 'b_1a88b4;
    }
    }
    }
    'b_1a88c4: {
    c.cycles += 2;
    let t = c.g[V0][0] != 0u64;

    if t {
    break 'b_1a88c4;
    } else {
    c.cycles += 2;
    call_ptr(c, 0x00260f80, 0x001a88c4)?;
    break 'b_1a88c4;
    }
    }
    'b_1a88e0: {
    c.cycles += 4;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    call_ptr(c, 0x00261460, 0x001a88cc)?;
    let t = c.g[V0][0] != 0u64;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    if t {
    break 'b_1a88e0;
    } else {
    c.cycles += 3;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    call_ptr(c, 0x002613c8, 0x001a88dc)?;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    break 'b_1a88e0;
    }
    }
    'b_1a88f4: {
    c.cycles += 2;
    let t = c.g[V0][0] != 0u64;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    if t {
    break 'b_1a88f4;
    } else {
    c.cycles += 3;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    call_ptr(c, 0x0025b800, 0x001a88f0)?;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    break 'b_1a88f4;
    }
    }
    c.cycles += 7;
    c.g[A1][0] = 0u64;
    c.g[A2][0] = 0x1;
    c.g[A3][0] = 0u64;
    c.g[T0][0] = 0u64;
    c.g[T1][0] = 0u64;
    call_ptr(c, 0x00262768, 0x001a890c)?;
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    break 'b_1a8910;
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = t; return Ok(());
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// NetFEState_ShowDisconnectedPopup (0x00001a9930).

/// `NetFEState_ShowDisconnectedPopup`: show the "disconnected" popup and stop the network session.
pub fn net_fe_state_show_disconnected_popup(c: &mut Cpu) {
    guest(c, |c| {
    'b_1a9a00: {
    c.cycles += 10;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[S3][0] = c.g[A0][0];
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6ac));
    let t = c.g[V0][0] != 0u64;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    if t {
    break 'b_1a9a00;
    } else {
    'b_1a99dc: {
    'b_1a99d0: {
    c.cycles += 25;
    c.g[S1][0] = 0x1;
    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
    c.sw((c.g[S3][0] as u32).wrapping_add(0x6ac), c.g[S1][0]);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    call_ptr(c, 0x0039f698, 0x001a996c)?;
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x001a9c50, 0x001a9974)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[A0][0] = 0x460000;
    c.sw((c.g[S2][0] as u32).wrapping_add(0xbc), c.g[S1][0]);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xbc));
    call_ptr(c, 0x00317670, 0x001a998c)?;
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 0u64;
    call_ptr(c, 0x001c57c0, 0x001a999c)?;
    c.g[V0][0] = 0x6;
    c.sw((c.g[S2][0] as u32).wrapping_add(0x170), c.g[S1][0]);
    c.sw((c.g[S2][0] as u32).wrapping_add(0x164), 0u64);
    c.sw((c.g[S2][0] as u32).wrapping_add(0x14c), c.g[V0][0]);
    call_ptr(c, 0x002591b8, 0x001a99b0)?;
    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
    let t = c.g[V1][0] != 0u64;
    c.g[V0][0] = 0x83d;
    if t {
    break 'b_1a99d0;
    } else {
    c.cycles += 5;
    c.g[V0][0] = 0x83e;
    c.g[A0][0] = 0x460000;
    c.sw((c.g[S2][0] as u32).wrapping_add(0x150), c.g[V0][0]);
    let t = true;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c40));
    if t {
    break 'b_1a99dc;
    } else {
    break 'b_1a99d0;
    }
    }
    }
    c.cycles += 3;
    c.g[A0][0] = 0x460000;
    c.sw((c.g[S2][0] as u32).wrapping_add(0x150), c.g[V0][0]);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d28));
    break 'b_1a99dc;
    }
    c.cycles += 9;
    call_ptr(c, 0x00317670, 0x001a99e4)?;
    c.g[V1][0] = 0x3;
    c.sw((c.g[S2][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
    c.sw((c.g[S2][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, 0x001a9bc0, 0x001a99fc)?;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    break 'b_1a9a00;
    }
    }
    c.cycles += 6;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = t; return Ok(());
        Ok(())
    });
}


/// `cFEStateStats_v25(self, state)` (0x1c3570): the stats screen's second
/// constructor vtable entry: set the numeric field, hash and store the title
/// string, then hash and install the five statistic rows.
pub fn cfestate_stats_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 62;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 5;
        c.g[RA][0] = sx(0x001c_358c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_a528, 0x001c_358c)?; // set number 5
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5dc0));
        call_ptr(c, 0x0031_7670, 0x001c_3598)?; // StrHash(title)
        c.g[A0][0] = 0x46_0000;
        c.sw(p(c.g[S0][0] as u32, 0x54), c.g[V0][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5dd8));
        call_ptr(c, 0x0031_7670, 0x001c_35a8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001c_35b8);
        c.g[A1][0] = 0;
        call_ptr(c, 0x001d_a530, 0x001c_35b8)?;
        // the five statistic rows: (label string, row, a3)
        for (i, (name, row, a3)) in [
            (0x45_fe60u32, 0u64, 3u64),
            (0x45_fe40, 1, 4),
            (0x46_1458, 2, 2),
            (0x46_5df8, 3, 8),
            (0x46_14e8, 4, 1),
        ]
        .into_iter()
        .enumerate()
        {
            let ret = 0x001c_35c4 + 0x1c * i as u32;
            c.g[A0][0] = sx(name);
            call_ptr(c, 0x0031_7670, ret)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = row;
            c.g[A3][0] = a3;
            call_ptr(c, 0x001d_a510, ret + 0x14)?;
        }
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `lh_2` (sign-extended).
#[inline(always)]
pub(crate) fn lh_2(c: &mut Cpu, a: u32) -> u64 {
    c.lh(a)
}


/// `lbu` (zero-extended).
#[inline(always)]
pub(crate) fn lbu(c: &mut Cpu, a: u32) -> u64 {
    c.lbu(a)
}


/// `sw_6`.
#[inline(always)]
pub(crate) fn sw_6(c: &mut Cpu, a: u32, v: u64) {
    c.sw(a, v)
}


/// The low 32 bits of a register as an address.
#[inline(always)]
pub(crate) fn a(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}


// ---------------------------------------------------------------------------
// Functions.

/// `cFEStateLobbyMenu_TransitionIn(self, reset)` (0x1bcd18): entering the
/// lobby, reset the room tab and refresh the room lists; when called with
/// `reset == 0` and a room already chosen, clamp the selected player to the
/// room's player count and re-show the tab.
pub fn cfestate_lobby_menu_transition_in(c: &mut Cpu) {
    const REFRESH_ROOM_LISTS: Callee =
        Callee { addr: 0x001b_e0a8, f: ee::s03::cFEStateLobbyMenu_RefreshRoomLists };
    const UPDATE_PLAYER_ARROWS: Callee = Callee { addr: 0x001b_e720, f: ee::s03::cFEStateLobbyMenu_UpdatePlayerScrollArrows };
    const UPDATE_PLAYERS_IN_ROOM: Callee = Callee { addr: 0x001b_e3a8, f: ee::s03::cFEStateLobbyMenu_UpdatePlayersInRoom };
    const SHOW_ROOM_TAB: Callee = Callee { addr: 0x001b_df60, f: ee::s03::cFEStateLobbyMenu_ShowRoomTab };
    const UI_LISTBOX_SET_SELECTION: u32 = 0x0039_a7a8;
    const UI_MENU_SET_SELECTION: u32 = 0x0039_b960;

    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        let this = abi::arg(c, 0);
        frame.save(c, 17, 0x10);
        frame.save(c, 16, 0x20);
        c.g[S1][0] = c.g[A0][0];
        let reset = c.g[A1][0] != 0;
        frame.save_ra(c, 0);
        if reset {
            c.cycles += 15;
            let vt = Addr(this).off(8).lw(c);
            c.g[V0][0] = vt;
            c.g[A0][0] = Addr(vt as u32).off(0x100).lh(c);
            c.g[V1][0] = Addr(vt as u32).off(0x104).lw(c);
            c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x001b_cd44)?;
            sw_6(c, p(this, 0x760), 0);
            c.g[A0][0] = c.g[S1][0];
            REFRESH_ROOM_LISTS.call(c, 0x001b_cd50)?;
            c.g[A0][0] = c.g[S1][0];
            UPDATE_PLAYER_ARROWS.call(c, 0x001b_cd58)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = lw(c, p(this, 0x718));
            UPDATE_PLAYERS_IN_ROOM.call(c, 0x001b_cd64)?;
            c.lq(16, frame.at(c, 0x20));
        } else {
            c.cycles += 3;
            c.g[V0][0] = lw(c, p(this, 0x760));
            let no_room = c.g[V0][0] == 0;
            c.lq(16, frame.at(c, 0x20));
            if !no_room {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                REFRESH_ROOM_LISTS.call(c, 0x001b_cd80)?;
                c.g[V1][0] = lw(c, p(this, 0x764));
                c.g[V0][0] = ((c.g[V1][0] as i64) < 4) as u64;
                let out_of_range = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                if !out_of_range {
                    c.cycles += 7;
                    c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = lw(c, p(this, 0x768));
                    c.g[A0][0] = lw(c, p(c.g[V0][0] as u32, 0x708));
                    c.g[V0][0] = lbu(c, p(c.g[A0][0] as u32, 0x318));
                    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let clamp = c.g[V1][0] != 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    if !clamp {
                        c.cycles += 1;
                        sw_6(c, p(this, 0x768), c.g[V0][0]);
                    }
                    c.cycles += 2;
                    c.g[A1][0] = lbu(c, p(this, 0x768));
                    call_ptr(c, UI_LISTBOX_SET_SELECTION, 0x001b_cdb8)?;
                }
                c.cycles += 19;
                c.g[A0][0] = lw(c, p(this, 0x6d4));
                c.g[S0][0] = sx(this.wrapping_add(0x718));
                c.g[A1][0] = lbu(c, p(this, 0x764));
                call_ptr(c, UI_MENU_SET_SELECTION, 0x001b_cdc8)?;
                c.g[V0][0] = lw(c, p(this, 0x764));
                c.g[A0][0] = c.g[S1][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = lw(c, c.g[V0][0] as u32);
                UPDATE_PLAYERS_IN_ROOM.call(c, 0x001b_cde0)?;
                c.g[A0][0] = c.g[S1][0];
                UPDATE_PLAYER_ARROWS.call(c, 0x001b_cde8)?;
                c.g[V0][0] = lw(c, p(this, 0x764));
                c.g[A0][0] = c.g[S1][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = lw(c, c.g[S0][0] as u32);
                SHOW_ROOM_TAB.call(c, 0x001b_ce00)?;
                c.lq(16, frame.at(c, 0x20));
            }
        }
        c.cycles += 5;
        c.g[V0][0] = 0;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateSelectPersona_FillPersonaList(self, list)` (0x1ad3e8): fill the
/// list box with one entry per persona in the global persona manager, giving
/// each entry its index, then the four fixed persona slots.
pub fn cfestate_select_persona_fill_persona_list(c: &mut Cpu) {
    const LIST_GET_AT: u32 = 0x0039_7870;
    const LISTBOX_ADD_ENTRY: u32 = 0x003a_0c50;
    const STR_ID: u32 = 0x0031_7670;
    const LISTBOX_SET_STR: u32 = 0x003a_1000;

    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        c.g[S3][0] = lw(c, gp(c, -0xc8));
        frame.save(c, 19, 0x20);
        frame.save(c, 17, 0x40);
        frame.save(c, 20, 0x10);
        frame.save(c, 16, 0x50);
        frame.save(c, 18, 0x30);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, p(c.g[S3][0] as u32, 0x6c));
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S4][0] = c.g[A1][0];
        if !empty {
            c.cycles += 2;
            c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x70));
            c.g[A1][0] = c.g[S1][0];
            loop {
                c.cycles += 19;
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x74));
                call_ptr(c, LIST_GET_AT, 0x001a_d428)?;
                c.g[S0][0] = c.g[V0][0];
                sw_6(c, p(c.g[S0][0] as u32, 0x18), c.g[S1][0]);
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[A1][0] = lw(c, c.g[S2][0] as u32);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
                call_ptr(c, LISTBOX_ADD_ENTRY, 0x001a_d444)?;
                c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 8));
                c.g[A1][0] = 1;
                c.g[A0][0] = lh_2(c, p(c.g[V0][0] as u32, 0x48));
                c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x4c));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V1][0] as u32, 0x001a_d45c)?;
                c.g[V0][0] = lw(c, p(c.g[S3][0] as u32, 0x6c));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S1][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001a_d420);
                }
            }
        }
        // Four fixed persona slots.
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S1][0] as i64) < 4) as u64;
        let full = c.g[V0][0] == 0;
        c.g[S2][0] = 0x0046_0000;
        if !full {
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            loop {
                c.cycles += 19;
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x74));
                call_ptr(c, LIST_GET_AT, 0x001a_d488)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2e58));
                sw_6(c, p(c.g[S0][0] as u32, 0x18), c.g[S1][0]);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                call_ptr(c, STR_ID, 0x001a_d49c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, LISTBOX_SET_STR, 0x001a_d4a8)?;
                c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 8));
                c.g[A1][0] = 1;
                c.g[A0][0] = lh_2(c, p(c.g[V0][0] as u32, 0x48));
                c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x4c));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V1][0] as u32, 0x001a_d4c0)?;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 4) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S1][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001a_d480);
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Addresses and helpers.

/// `StrHash(c)` (0x00317670): hashes the C string at `c` in place and returns
/// the hash.
pub(crate) const STR_HASH: u32 = 0x0031_7670;

/// `UIText_SetTextId(text, id)` (0x003a1000).
pub(crate) const UI_TEXT_SET_TEXT_ID: u32 = 0x003a_1000;

/// `UIText_SetString(text, str)` (0x003a0c50).
pub(crate) const UI_TEXT_SET_STRING: u32 = 0x003a_0c50;

/// `sprintf` (0x00417828).
pub(crate) const SPRINTF: u32 = 0x0041_7828;


/// `a + off`.
#[inline(always)]
pub(crate) fn p_7(a: u64, off: u32) -> u32 {
    (a as u32).wrapping_add(off)
}


/// `self->field` for a word field.
#[inline(always)]
pub(crate) fn lw_9(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr(p_7(base, off)).lw(c)
}


/// `self->field` for a signed halfword field.
#[inline(always)]
pub(crate) fn lh_3(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr(p_7(base, off)).lh(c)
}


/// `self->field = v` for a word field.
#[inline(always)]
pub(crate) fn sw_7(c: &mut Cpu, base: u64, off: u32, v: u64) {
    Addr(p_7(base, off)).sw(c, v);
}


/// A virtual call of slot `slot` at `*(obj + vt_off)`, with the method pointer
/// left in `v1` (the vtable is read into `v0` first), as gcc emits for
/// `obj->vt->fn(...)`.
pub(crate) fn vcall_v1(c: &mut Cpu, obj: u64, vt_off: u32, slot: u32, ret: u32) -> Flow {
    let vt = lw_9(c, obj, vt_off);
    c.g[V0][0] = vt;
    let delta = lh_3(c, vt, slot);
    let func = lw_9(c, vt, slot + 4);
    c.g[V1][0] = func;
    c.g[A0][0] = sx((obj as u32).wrapping_add(delta as u32));
    call_ptr(c, func as u32, ret)
}


/// A virtual call of slot `slot` at `*(obj + vt_off)`, with the method pointer
/// left in `v0` (the vtable is read into `v1` first), as gcc emits for
/// `((Vt*)*(void**)(obj + vt_off))->fn(...)` when the vtable is reloaded.
pub(crate) fn vcall_v0(c: &mut Cpu, obj: u64, vt_off: u32, slot: u32, ret: u32) -> Flow {
    let vt = lw_9(c, obj, vt_off);
    c.g[V1][0] = vt;
    let delta = lh_3(c, vt, slot);
    let func = lw_9(c, vt, slot + 4);
    c.g[V0][0] = func;
    c.g[A0][0] = sx((obj as u32).wrapping_add(delta as u32));
    call_ptr(c, func as u32, ret)
}


// ---------------------------------------------------------------------------
// Front end: widget-created handlers and stats column navigation.

/// `cFEStateCreateStep2_OnWidgetCreated(self, widget)` (0x001ba500): the
/// create-rider screen created one of its five list boxes (month, day,
/// weekday, year, gender); fill the list that belongs to the widget just
/// created. The widget is identified by the hash of its name (`widget+0x38`)
/// against the five constant names.
pub fn cfe_state_create_step2_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        let widget = c.g[S1][0];
        let this = c.g[S2][0];

        'out: {
            // Month.
            c.g[S0][0] = lw_9(c, widget, 0x38);
            c.g[A0][0] = sx(0x0046_4a08);
            call_ptr(c, STR_HASH, 0x001b_a52c)?;
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 6;
                c.g[A0][0] = this;
                c.g[A1][0] = widget;
                c.g[A2][0] = 0;
                Callee {
                    addr: 0x001a_9a18,
                    f: ee::s03::NetFEState_FillYesNoList,
                }
                .call(c, 0x001b_a544)?;
                frame.restore(c, S0, 0x30);
                break 'out;
            }

            // Day of the month.
            c.cycles += 5;
            c.g[S0][0] = lw_9(c, widget, 0x38);
            c.g[A0][0] = sx(0x004a_1bd8);
            call_ptr(c, STR_HASH, 0x001b_a558)?;
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 6;
                c.g[A0][0] = this;
                c.g[A1][0] = widget;
                c.g[A2][0] = 1;
                Callee {
                    addr: 0x001b_b5d0,
                    f: ee::s03::cFEStateCreateStep2_FillMonthList,
                }
                .call(c, 0x001b_a570)?;
                frame.restore(c, S0, 0x30);
                break 'out;
            }

            // Weekday.
            c.cycles += 5;
            c.g[S0][0] = lw_9(c, widget, 0x38);
            c.g[A0][0] = sx(0x004a_1be0);
            call_ptr(c, STR_HASH, 0x001b_a584)?;
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 8;
                c.g[A0][0] = this;
                c.g[A1][0] = widget;
                c.g[A2][0] = 1;
                c.g[A3][0] = 0x7c1;
                c.g[T0][0] = 1;
                Callee {
                    addr: 0x001b_b840,
                    f: ee::s03::cFEStateCreateStep2_FillDayList,
                }
                .call(c, 0x001b_a5a4)?;
                frame.restore(c, S0, 0x30);
                break 'out;
            }

            // Year.
            c.cycles += 5;
            c.g[S0][0] = lw_9(c, widget, 0x38);
            c.g[A0][0] = sx(0x004a_1be8);
            call_ptr(c, STR_HASH, 0x001b_a5b8)?;
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 6;
                c.g[A0][0] = this;
                c.g[A1][0] = widget;
                c.g[A2][0] = 0x7c1;
                Callee {
                    addr: 0x001b_b798,
                    f: ee::s03::cFEStateCreateStep2_FillYearList,
                }
                .call(c, 0x001b_a5d0)?;
                frame.restore(c, S0, 0x30);
                break 'out;
            }

            // Gender.
            c.cycles += 5;
            c.g[S0][0] = lw_9(c, widget, 0x38);
            c.g[A0][0] = sx(0x0046_4a18);
            call_ptr(c, STR_HASH, 0x001b_a5e4)?;
            let mismatch = c.g[S0][0] != c.g[V0][0];
            frame.restore(c, S0, 0x30);
            if !mismatch {
                c.cycles += 5;
                c.g[A0][0] = this;
                c.g[A1][0] = widget;
                c.g[A2][0] = 1;
                Callee {
                    addr: 0x001b_b520,
                    f: ee::s03::cFEStateCreateStep2_FillGenderList,
                }
                .call(c, 0x001b_a5fc)?;
                frame.restore(c, S0, 0x30);
            }
        }

        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateCreateDuplicate_OnWidgetCreated(self, text)` (0x001bbe00): one of
/// the duplicate-rider widgets was created; give its text the right label (a
/// fixed caption string, or one of the four weekday names).
pub fn cfe_state_create_duplicate_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0x40);
        frame.save(c, S0, 0x80);
        let this = c.g[S1][0];
        let text = c.g[S2][0];

        // A fixed caption.
        c.g[S0][0] = lw_9(c, text, 0x38);
        c.g[A0][0] = sx(0x0046_4c68);
        call_ptr(c, STR_HASH, 0x001b_be30)?;
        if c.g[S0][0] == c.g[V0][0] {
            c.cycles += 3;
            c.g[V0][0] = 4;
            sw_7(c, text, 0x18, c.g[V0][0]);
            c.cycles += 1;
            frame.restore(c, S0, 0x80);
        } else {
            // Two captions only apply while no rider slot is selected.
            let mut label = 0u64;
            let mut weekday_loop = false;
            c.g[V0][0] = lw_9(c, this, 0x6d0);
            c.cycles += 2;
            c.g[S1][0] = 0;
            if c.g[V0][0] != 0 {
                weekday_loop = true;
            } else {
                c.cycles += 6;
                c.g[S0][0] = lw_9(c, text, 0x38);
                c.g[A0][0] = sx(0x0046_1520);
                call_ptr(c, STR_HASH, 0x001b_be5c)?;
                if c.g[S0][0] == c.g[V0][0] {
                    c.cycles += 3;
                    label = 0x0046_4c78;
                } else {
                    c.g[V0][0] = lw_9(c, this, 0x6d0);
                    c.cycles += 2;
                    c.g[S1][0] = 0;
                    if c.g[V0][0] != 0 {
                        weekday_loop = true;
                    } else {
                        c.cycles += 6;
                        c.g[S0][0] = lw_9(c, text, 0x38);
                        c.g[A0][0] = sx(0x0046_4c90);
                        call_ptr(c, STR_HASH, 0x001b_be88)?;
                        let mismatch = c.g[S0][0] != c.g[V0][0];
                        c.g[S1][0] = 0;
                        if mismatch {
                            weekday_loop = true;
                        } else {
                            c.cycles += 2;
                            label = 0x0046_4ca0;
                        }
                    }
                }
            }
            if label != 0 {
                // `UIText_SetTextId(text, GetHashValue32(name))`.
                c.cycles += 7;
                c.g[A0][0] = sx(label as u32);
                call_ptr(c, STR_HASH, 0x001b_bea0)?;
                c.g[A0][0] = text;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, UI_TEXT_SET_TEXT_ID, 0x001b_beac)?;
                frame.restore(c, S0, 0x80);
            } else {
                debug_assert!(weekday_loop);
                // Try the four weekday names.
                c.cycles += 1;
                c.g[S3][0] = 0x0046_0000;
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < 4) as u64;
                    let done = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[SP][0];
                    if done {
                        break;
                    }
                    c.cycles += 8;
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4cb8));
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, SPRINTF, 0x001b_bed4)?;
                    c.g[S0][0] = lw_9(c, text, 0x38);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, STR_HASH, 0x001b_bee0)?;
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                        if event_due(c) {
                            deliver_event_here(c, 0x001b_bebc);
                        }
                        continue;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = text;
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, UI_TEXT_SET_STRING, 0x001b_bef4)?;
                    sw_7(c, text, 0x18, c.g[S1][0]);
                    break;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x80);
            }
        }

        c.cycles += 6;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateNetConfig_OnChildEvent(self, child, kind)` (0x001bf5a8): a widget
/// of the network-config screen fired. Kind 15 ("OK") allocates and enters the
/// network-config state; 16 and 20 (and everything else) go to the base
/// handler.
pub fn cfe_state_net_config_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let kind = c.g[A2][0];
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0];

        if kind == 0x10 {
            c.cycles += 4;
            c.g[A0][0] = this;
            Callee {
                addr: 0x001a_97b8,
                f: ee::s03::NetFEState_OnChildEvent,
            }
            .call(c, 0x001b_f69c)?;
        } else if kind < 0x11 {
            if kind == 0xf {
                c.cycles += 3 + 2 + 15;
                c.g[V0][0] = 1; // set by the switch
                sw_7(c, this, 0x6b0, c.g[V0][0]);
                c.g[A1][0] = sx(0x0046_0bf0);
                c.g[A0][0] = 0x91c;
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x001b_f60c)?;
                c.g[A1][0] = lw_9(c, this, 0x10);
                c.g[A0][0] = c.g[V0][0];
                Callee {
                    addr: 0x001b_3f78,
                    f: ee::s03::cFEStateNetConfig_ctor,
                }
                .call(c, 0x001b_f618)?;
                c.g[A0][0] = this;
                c.g[A1][0] = c.g[V0][0];
                Callee {
                    addr: 0x001a_8a40,
                    f: ee::s03::NetFEState_GotoNextState,
                }
                .call(c, 0x001b_f624)?;
                let failed = c.g[V0][0] == 0;
                c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
                if !failed {
                    // Wait for the session, then drop it.
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.cycles += 2;
                        call_ptr(c, 0x0026_0f80, 0x001b_f63c)?;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
                    call_ptr(c, 0x0026_1460, 0x001b_f644)?;
                    c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
                    if c.g[V0][0] == 0 {
                        c.cycles += 3;
                        c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
                        call_ptr(c, 0x0026_13c8, 0x001b_f654)?;
                        c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
                    }
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.cycles += 2;
                        c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
                        call_ptr(c, 0x0025_b800, 0x001b_f668)?;
                    }
                    c.cycles += 11;
                    c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
                    call_ptr(c, 0x0026_1408, 0x001b_f670)?;
                    c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
                    call_ptr(c, 0x0025_b800, 0x001b_f678)?;
                    call_ptr(c, 0x0025_91b8, 0x001b_f680)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0026_36f0, 0x001b_f68c)?;
                }
                c.cycles += 1;
            } else {
                c.cycles += 3 + 2 + 2 + 2 + 1;
                c.g[A0][0] = this;
                Callee {
                    addr: 0x001a_97b8,
                    f: ee::s03::NetFEState_OnChildEvent,
                }
                .call(c, 0x001b_f6ac)?;
            }
        } else {
            c.cycles += 3 + 3;
            if kind == 0x14 {
                c.cycles += 4;
                c.g[A0][0] = this;
                Callee {
                    addr: 0x001a_97b8,
                    f: ee::s03::NetFEState_OnChildEvent,
                }
                .call(c, 0x001b_f69c)?;
            } else {
                c.cycles += 2 + 2 + 1;
                c.g[A0][0] = this;
                Callee {
                    addr: 0x001a_97b8,
                    f: ee::s03::NetFEState_OnChildEvent,
                }
                .call(c, 0x001b_f6ac)?;
            }
        }

        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateStats_FocusNextColumn(self)` (0x001c4d28): move to the next stats
/// column: unfocus the old column, show the new column's items and advance the
/// index, recursing while the columns are exhausted.
pub fn cfe_state_stats_focus_next_column(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = c.g[S0][0];

        'out: {
            // Unfocus the widget that had focus (the global one if none).
            let panel = lw_9(c, this, 0x10);
            let old = lw_9(c, panel, 0x14);
            c.g[V1][0] = sx(gp(c, 0x2968));
            c.g[A2][0] = old;
            if c.g[A2][0] == 0 {
                c.g[A2][0] = c.g[V1][0];
            }
            c.g[A1][0] = 1;
            if c.g[A2][0] != 0 {
                c.cycles += 5;
                vcall_v1(c, c.g[A2][0], 0, 0x10, 0x001c_4d68)?;
            }

            // Hide the current column unless it is the last one.
            c.cycles += 4;
            c.g[S1][0] = 3;
            let column = lw_9(c, this, 0x6e4);
            c.g[V0][0] = sx((column as u32) << 2);
            if column != c.g[S1][0] {
                c.cycles += 8;
                c.g[A1][0] = 0;
                let item = lw_9(
                    c,
                    (this as u32).wrapping_add((column as u32) << 2) as u64,
                    0x6d0,
                );
                c.g[A2][0] = item;
                vcall_v0(c, item, 8, 0x38, 0x001c_4d98)?;
            }

            // Advance the column, wrapping 4 -> 0.
            c.cycles += 5;
            let column = lw_9(c, this, 0x6e4);
            c.g[V0][0] = sx((column as u32).wrapping_add(1));
            c.g[V1][0] = (c.g[V0][0] < 4) as u64;
            let in_range = c.g[V1][0] != 0;
            let next = c.g[V0][0];
            sw_7(c, this, 0x6e4, next);
            if !in_range {
                c.cycles += 1;
                sw_7(c, this, 0x6e4, 0);
            }

            // Show the new column.
            c.cycles += 3;
            let column = lw_9(c, this, 0x6e4);
            c.g[V0][0] = sx((column as u32) << 2);
            if column == c.g[S1][0] {
                // Past the last column: show the "next page" item and recurse.
                c.cycles += 11;
                let item = lw_9(c, this, 0x6dc);
                c.g[A2][0] = item;
                c.g[A1][0] = 1;
                vcall_v0(c, item, 8, 0x38, 0x001c_4dd8)?;
                c.g[A0][0] = this;
                Callee {
                    addr: 0x001c_4d28,
                    f: ee::s03::f_001c4d28,
                }
                .call(c, 0x001c_4de0)?;
                frame.restore(c, S0, 0x20);
                break 'out;
            }
            c.cycles += 7;
            let item = lw_9(
                c,
                (this as u32).wrapping_add((column as u32) << 2) as u64,
                0x6d0,
            );
            c.g[A2][0] = item;
            c.g[V1][0] = lw_9(c, item, 0x14);
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 6) as u32);
            c.g[V1][0] = c.g[V1][0] & 1;
            let focused = c.g[V1][0] != 0;
            c.g[A1][0] = 1;
            if focused {
                c.cycles += 7;
                vcall_v0(c, item, 8, 0x38, 0x001c_4e18)?;
                frame.restore(c, S0, 0x20);
                break 'out;
            }
            // Not focusable: skip past it to the next column.
            c.cycles += 3;
            c.g[A0][0] = this;
            Callee {
                addr: 0x001c_4d28,
                f: ee::s03::f_001c4d28,
            }
            .call(c, 0x001c_4e28)?;
            frame.restore(c, S0, 0x20);
        }

        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateStats_FocusPrevColumn(self)` (0x001c4e40): the mirror image of
/// [`cfe_state_stats_focus_next_column`]; the column index wraps 0 -> 3.
pub fn cfe_state_stats_focus_prev_column(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = c.g[S0][0];

        'out: {
            let panel = lw_9(c, this, 0x10);
            let old = lw_9(c, panel, 0x14);
            c.g[V1][0] = sx(gp(c, 0x2968));
            c.g[A2][0] = old;
            if c.g[A2][0] == 0 {
                c.g[A2][0] = c.g[V1][0];
            }
            c.g[A1][0] = 1;
            if c.g[A2][0] != 0 {
                c.cycles += 5;
                vcall_v1(c, c.g[A2][0], 0, 0x10, 0x001c_4e80)?;
            }

            c.cycles += 4;
            c.g[S1][0] = 3;
            let column = lw_9(c, this, 0x6e4);
            c.g[V0][0] = sx((column as u32) << 2);
            if column != c.g[S1][0] {
                c.cycles += 8;
                c.g[A1][0] = 0;
                let item = lw_9(
                    c,
                    (this as u32).wrapping_add((column as u32) << 2) as u64,
                    0x6d0,
                );
                c.g[A2][0] = item;
                vcall_v0(c, item, 8, 0x38, 0x001c_4eb0)?;
            }

            // Retreat the column, wrapping 0 -> 3 (the unsigned compare also
            // catches the 0xffffffff result).
            c.cycles += 5;
            let column = lw_9(c, this, 0x6e4);
            c.g[V0][0] = sx((column as u32).wrapping_sub(1));
            c.g[V1][0] = (c.g[V0][0] < 4) as u64;
            let in_range = c.g[V1][0] != 0;
            let next = c.g[V0][0];
            sw_7(c, this, 0x6e4, next);
            if !in_range {
                c.cycles += 1;
                sw_7(c, this, 0x6e4, c.g[S1][0]);
            }

            c.cycles += 3;
            let column = lw_9(c, this, 0x6e4);
            c.g[V0][0] = sx((column as u32) << 2);
            if column == c.g[S1][0] {
                c.cycles += 11;
                let item = lw_9(c, this, 0x6dc);
                c.g[A2][0] = item;
                c.g[A1][0] = 1;
                vcall_v0(c, item, 8, 0x38, 0x001c_4ef0)?;
                c.g[A0][0] = this;
                Callee {
                    addr: 0x001c_4e40,
                    f: ee::s03::f_001c4e40,
                }
                .call(c, 0x001c_4ef8)?;
                frame.restore(c, S0, 0x20);
                break 'out;
            }
            c.cycles += 7;
            let item = lw_9(
                c,
                (this as u32).wrapping_add((column as u32) << 2) as u64,
                0x6d0,
            );
            c.g[A2][0] = item;
            c.g[V1][0] = lw_9(c, item, 0x14);
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 6) as u32);
            c.g[V1][0] = c.g[V1][0] & 1;
            let focused = c.g[V1][0] != 0;
            c.g[A1][0] = 1;
            if focused {
                c.cycles += 7;
                vcall_v0(c, item, 8, 0x38, 0x001c_4f30)?;
                frame.restore(c, S0, 0x20);
                break 'out;
            }
            c.cycles += 3;
            c.g[A0][0] = this;
            Callee {
                addr: 0x001c_4e40,
                f: ee::s03::f_001c4e40,
            }
            .call(c, 0x001c_4f40)?;
            frame.restore(c, S0, 0x20);
        }

        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// 0x001b7650 cFEStateNetAccountMem_ShowPopup
// ---------------------------------------------------------------------------

/// `void cFEStateNetAccountMem_ShowPopup(void* self, void* state, int kind,
/// int text_id, int* a, int b, int c)` (0x1b7650).
///
/// Builds the account-memory popup: `NetFEState_GetPopup` supplies the popup
/// object, which is configured with one or two option strings and then shown
/// through `NetFEState_ShowPopup`.
pub fn cfe_state_net_account_mem_show_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, 23, 0x20);
        frame.save(c, 18, 0x70);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, 19, 0x60);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, 20, 0x50);
        c.g[S3][0] = c.g[T0][0];
        frame.save(c, 21, 0x40);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, 22, 0x30);
        c.g[S5][0] = c.g[T1][0];
        frame.save(c, 30, 0x10);
        c.g[S6][0] = c.g[T2][0];
        frame.save(c, 16, 0x90);
        frame.save(c, 17, 0x80);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x001b_769c);
        c.g[FP][0] = c.g[A1][0];
        call_ptr(c, 0x001a_9c50, 0x001b_769c)?; // NetFEState_GetPopup
        c.g[S1][0] = c.g[V0][0];
        let t = c.g[T0][0] == 0;
        c.g[S0][0] = sx(c.g[S1][0] as u32 + 0xbc);
        'ret: {
            if t {
                break 'ret;
            }
            'two: {
                c.cycles += 2;
                let t = c.g[S3][0] == 0;
                c.g[V0][0] = 2;
                if t {
                    break 'two;
                }
                c.cycles += 11;
                c.g[A2][0] = c.g[S2][0];
                c.sw(p(c.g[S1][0] as u32, 0xbc), c.g[V0][0]);
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x001b_76c4);
                c.g[A1][0] = 0;
                call_ptr(c, 0x001c_57c0, 0x001b_76c4)?;
                c.g[A2][0] = c.g[S3][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x001b_76d4);
                c.g[A1][0] = 1;
                call_ptr(c, 0x001c_57c0, 0x001b_76d4)?;
                let t = true;
                c.sw(p(c.g[S1][0] as u32, 0x168), c.g[S4][0]);
                if t {
                    break 'ret;
                }
                break 'two;
            }
            // One option only.
            c.cycles += 8;
            c.g[V0][0] = 1;
            c.g[A2][0] = c.g[S2][0];
            c.sw(p(c.g[S1][0] as u32, 0xbc), c.g[V0][0]);
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x001b_76f4);
            c.g[A1][0] = 0;
            call_ptr(c, 0x001c_57c0, 0x001b_76f4)?;
            let t = true;
            c.sw(p(c.g[S1][0] as u32, 0x168), 0);
            if t {
                break 'ret;
            }
            c.cycles += 3;
            c.sw(p(c.g[S1][0] as u32, 0xbc), 0);
            c.sw(p(c.g[S1][0] as u32, 0x164), 0);
            c.sw(p(c.g[S1][0] as u32, 0x168), 0);
            break 'ret;
        }

        'opt1: {
            c.cycles += 2;
            let t = c.g[S5][0] == 0;
            c.g[V0][0] = 1;
            if t {
                break 'opt1;
            }
            c.cycles += 2;
            c.sw(p(c.g[S0][0] as u32, 0x10), c.g[S5][0]);
            c.sw(p(c.g[S0][0] as u32, 0xc), c.g[V0][0]);
            break 'opt1;
        }
        'opt2: {
            c.cycles += 2;
            let t = c.g[S6][0] == 0;
            c.g[V0][0] = 1;
            if t {
                break 'opt2;
            }
            c.cycles += 2;
            c.sw(p(c.g[S0][0] as u32, 8), c.g[S6][0]);
            c.sw(p(c.g[S0][0] as u32, 4), c.g[V0][0]);
            break 'opt2;
        }
        c.cycles += 19;
        c.sw(p(c.g[S0][0] as u32, 0x94), c.g[FP][0]);
        c.g[V0][0] = 1;
        c.sw(p(c.g[S0][0] as u32, 0xa8), 0);
        c.g[A0][0] = c.g[S7][0];
        c.sw(p(c.g[S0][0] as u32, 0x98), c.g[V0][0]);
        c.g[RA][0] = sx(0x001b_7744);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001a_9bc0, 0x001b_7744)?; // NetFEState_ShowPopup

        frame.restore(c, 16, 0x90);
        frame.restore(c, 17, 0x80);
        frame.restore(c, 18, 0x70);
        frame.restore(c, 19, 0x60);
        frame.restore(c, 20, 0x50);
        frame.restore(c, 21, 0x40);
        frame.restore(c, 22, 0x30);
        frame.restore(c, 23, 0x20);
        frame.restore(c, 30, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateNetQuickPlayPopup_SetOptionsEnabled(Popup* self, int value)`
/// (0x1c17f8): pass `value` to the setter (slot 0x40/0x44 of the object's
/// vtable) of the fifteen option widgets at +0x6f8..
pub fn net_quick_play_popup_set_options_enabled(c: &mut Cpu) {
    guest(c, |c| {
        const RET: u32 = 0x001c_1828;
        /// `set(value)` through `[[self + off] + 8]` slot 0x40.
        fn set_option(c: &mut Cpu, self_: u32, off: u32, value: u64) -> Flow {
            c.g[A2][0] = Addr(self_.wrapping_add(off)).lw(c);
            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).lw(c);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x40)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x44)).lw(c);
            let func = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A1][0] = value;
            call_ptr(c, func, RET)
        }
        c.cycles += 115;
        let f = Frame::open(c, 0x30);
        f.save(c, S0, 0x20);
        f.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        f.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let obj = c.g[S0][0] as u32;
        let value = c.g[S1][0];
        for i in 0..15u32 {
            set_option(c, obj, 0x6f8 + 4 * i, value)?;
        }
        f.restore(c, S0, 0x20);
        f.restore(c, S1, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}


/// `void func_001BB840(void* self, void* box, int month, int year, int sel)`
/// (0x1bb840): fill the creation screen's day list box: clear it, then for
/// every day of the given month add a numbered entry and select the requested
/// day (clamped to the month's length).
pub fn c_fe_state_create_step2_fill_day_list(c: &mut Cpu) {
    let frame = Frame::open(c, 0xb0);
    'b_1bb8ec: {
        c.cycles += 29;
        frame.save(c, S1, 0x90);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S6, 0x40);
        c.g[A1][0] = c.g[A2][0];
        frame.save(c, S2, 0x80);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, S0, 0xa0);
        frame.save(c, S3, 0x70);
        c.g[S2][0] = c.g[T0][0];
        frame.save(c, S7, 0x30);
        c.g[A2][0] = c.g[A3][0];
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save_ra(c, 0x20);
        c.g[RA][0] = sx(0x001bb884);
        c.g[S0][0] = 0x1;
        if callee::C001bb9b0.call(c, 0x001bb884).is_err() {
            return;
        }
        c.g[S3][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001bb890);
        c.g[A0][0] = c.g[S1][0];
        if call_ptr(c, 0x0039a670, 0x001bb890).is_err() {
            return;
        }
        c.g[V0][0] = lw(c, at(c, S1, 0x14));
        c.g[V1][0] = 0xfffffffffffffffe;
        c.g[A1][0] = sx(at(c, S3, 0x1));
        c.g[S7][0] = sx(at(c, SP, 0x10));
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[A0][0] = ((c.g[S0][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.g[V0][0] = c.g[V0][0] | 0x80;
        let t = c.g[A0][0] == 0u64;
        sw_8(c, at(c, S1, 0x14), c.g[V0][0]);
        if t {
            break 'b_1bb8ec;
        } else {
            c.cycles += 3;
            c.g[S4][0] = c.g[A1][0];
            c.g[S5][0] = 0x4a0000;
            c.g[A0][0] = c.g[SP][0];
            'l_1bb8c0: loop {
                c.cycles += 11;
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x001bb8cc);
                c.g[A1][0] = sx(at(c, S5, 0x12c0));
                if call_ptr(c, 0x00417828, 0x001bb8cc).is_err() {
                    return;
                }
                c.g[A2][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x001bb8e0);
                c.g[S0][0] = sx(at(c, S0, 0x1));
                if call_ptr(c, 0x0039a3f8, 0x001bb8e0).is_err() {
                    return;
                }
                c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                let t = c.g[V1][0] != 0u64;
                c.g[A0][0] = c.g[SP][0];
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x001bb8c0);
                    }
                    continue 'l_1bb8c0;
                } else {
                    break 'b_1bb8ec;
                }
            }
        }
    }
    'b_1bb904: {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S2][0] as i64)) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[A2][0] = c.g[S7][0];
        if t {
            break 'b_1bb904;
        } else {
            c.cycles += 3;
            c.g[V0][0] = lw(c, at(c, S6, 0x6d0));
            c.g[S2][0] = c.g[S3][0];
            sw_8(c, at(c, V0, 0x2c), c.g[S3][0]);
            break 'b_1bb904;
        }
    }
    c.cycles += 24;
    c.g[A1][0] = c.g[S2][0];
    sb(c, at(c, SP, 0x10), 0u64);
    c.g[RA][0] = sx(0x001bb914);
    c.g[A0][0] = c.g[S1][0];
    if call_ptr(c, 0x0039a768, 0x001bb914).is_err() {
        return;
    }
    c.g[A1][0] = c.lbu(at(c, SP, 0x10));
    c.g[RA][0] = sx(0x001bb920);
    c.g[A0][0] = c.g[S1][0];
    if call_ptr(c, 0x0039a7a8, 0x001bb920).is_err() {
        return;
    }
    c.g[V0][0] = lw(c, at(c, S1, 0x8));
    c.g[A1][0] = 0x1;
    c.g[A0][0] = lh_4(c, at(c, V0, 0x48));
    c.g[V1][0] = lw(c, at(c, V0, 0x4c));
    let t = c.g[V1][0] as u32;
    c.g[RA][0] = sx(0x001bb938);
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
    if call_ptr(c, t, 0x001bb938).is_err() {
        return;
    }
    frame.restore(c, S0, 0xa0);
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    frame.restore(c, S7, 0x30);
    c.g[RA][0] = ld(c, at(c, SP, 0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0xb0));
    c.pc = t;
    return;
}


// ---------------------------------------------------------------------------
// Functions.

/// `void cFEPopupConfirm_CfgReset(cFEPopupConfirm *self)` (0x1c5630): clear
/// the four 8-byte entries at +4, the five {8-byte, 12-byte} pairs at +0x24
/// and +0x54, then reset the popup's counters, the vector at +0x9c and the
/// "no selection" index (-1).
pub fn cfepopupconfirm_cfg_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        frame.save(c, 19, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = abi::arg64(c, 0);

        // The four 8-byte entries at +4, +0xc, +0x14, +0x1c.
        c.g[A1][0] = 0;
        c.g[A2][0] = 8;
        const CLEAR_RETS: [u32; 4] = [0x001c_5664, 0x001c_5678, 0x001c_5688, 0x001c_5698];
        for (i, off) in [4u32, 0xc, 0x14, 0x1c].into_iter().enumerate() {
            c.g[A0][0] = sx(this.wrapping_add(off));
            callee::MEMSET_4.call(c, CLEAR_RETS[i])?;
        }

        // Five iterations clearing an 8-byte entry (from +0x24, stride 8) and
        // a 12-byte entry (from +0x54, stride 0xc).
        c.g[S1][0] = sx(this.wrapping_add(0x24));
        c.g[S2][0] = sx(this.wrapping_add(0x54));
        c.g[S3][0] = 4;
        loop {
            c.cycles += 12;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 8;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(8));
            callee::MEMSET_4.call(c, 0x001c_56b0)?;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0xc;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
            callee::MEMSET_4.call(c, 0x001c_56c8)?;
            let more = (c.g[S3][0] as i64) >= 0;
            c.g[A0][0] = c.g[S1][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001c_56a0);
            }
        }

        c.cycles += 32;
        c.g[A0][0] = sx(this.wrapping_add(0x4c));
        c.g[A1][0] = 0;
        c.g[A2][0] = 8;
        callee::MEMSET_4.call(c, 0x001c_56e0)?;

        let one = 1;
        c.g[V1][0] = one;
        c.sw(this.wrapping_add(0x98), 0);
        c.sw(this.wrapping_add(0xbc), one);
        c.g[V0][0] = 0x0050_0000;
        c.sw(this.wrapping_add(0xa8), one);
        let minus_one = u64::MAX;
        c.g[A0][0] = minus_one;
        c.sw(this.wrapping_add(0x94), 0);
        c.sw(this.wrapping_add(0xb8), 0);
        c.sw(this.wrapping_add(0xb0), 0);
        c.sw(this, 0);
        c.sw(this.wrapping_add(0x90), 0);
        // *(sVec3*)(self + 0x9c) = D_004FF0D8 (a 12-byte copy, unaligned).
        let src = (c.g[V0][0] as u32).wrapping_sub(0xf28);
        c.g[A3][0] = sx(src);
        c.ldl(3, src.wrapping_add(7));
        c.ldr(3, src);
        c.g[A1][0] = c.lw(src.wrapping_add(8));
        c.sdl(3, this.wrapping_add(0xa3));
        c.sdr(3, this.wrapping_add(0x9c));
        c.sw(this.wrapping_add(0xa4), c.g[A1][0]);
        c.sw(this.wrapping_add(0xac), minus_one);
        c.sw(this.wrapping_add(0xb4), 0);
        c.sw(this.wrapping_add(0xc0), 0);

        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
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
    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use ssx3_ee as ee;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::fakes;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::elf_image;
    use crate::difftest::SP as STACK_TOP;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::RA as RET;
    use crate::difftest::SP;
    use crate::difftest::SP as STACK;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::compare;
    use crate::difftest::seed_count;
    pub(crate) use crate::game::be::beintnetwork::tests::put_str;
    pub(crate) use crate::game::be::beintmission::tests::SEEDS;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_LIBRARY;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;
    pub(crate) use crate::game::main::ssxappdtor::tests::MEM_MUTEX;
    pub(crate) use crate::game::main::ssxappdtor::tests::alloc_setup;
    pub(crate) use crate::game::main::ssxappdtor::tests::dtor_case;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::ovstatepause::tests::WORLD;
    pub(crate) use crate::game::fe::ovstatepause::tests::globals_2 as globals_3;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2 as be_library_3;
    pub(crate) use crate::game::dirtysock::tags2::tests::ZERO_FN;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::dirtysock::tags2::tests::put_vtable;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_4;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::dirtysock::tags2::tests::SEEDS_7 as SEEDS_5;
    pub(crate) use crate::game::dirtysock::tags2::tests::clear as clear_3;
    pub(crate) use crate::game::dirtysock::tags2::tests::heap_block_2 as heap_block;
    pub(crate) use crate::game::dirtysock::tags2::tests::dummy_world;
    pub(crate) use crate::game::dirtysock::tags2::tests::common;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::ARENA;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::ARENA_SIZE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::run as run_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_init as heap_init_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::alloc;
    pub(crate) use crate::game::dirtysock::tags2::tests::run_4 as run_3;
    pub(crate) use crate::game::dirtysock::tags2::tests::heap_init_5 as heap_init_3;
    pub(crate) use crate::game::fe::fepopup3::tests::setup;
    pub(crate) use crate::game::object::railmodifier::tests::SEEDS_7 as SEEDS_6;
    pub(crate) use crate::game::object::railmodifier::tests::rng;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_7;
    pub(crate) use crate::game::dirtysock::tags2::tests::bb;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::belib_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::world_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::string;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_setup;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_9;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_6;
    pub(crate) use crate::game::bx::bxstring::tests::string as string_3;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2 as BE_3;
    pub(crate) use crate::game::movie::movieplayer::tests::W as W_2;
    pub(crate) use crate::game::movie::movieplayer::tests::run_2 as run_5;
    pub(crate) use crate::game::movie::movieplayer::tests::heap_init_2 as heap_init_6;
    pub(crate) use crate::game::movie::movieplayer::tests::pool_allocator as pool_allocator_8;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base_5;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base as alloc_base_2;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_10;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_7;
    pub(crate) use crate::game::fe::festaterewards::tests::w16 as w16_2;
    pub(crate) use crate::game::be::beintplayer::tests::w8;
    pub(crate) use crate::game::fe::festaterewards::tests::slot_2 as slot_7;
    pub(crate) use crate::game::fe::festaterewards::tests::globals_3 as globals_7;
    pub(crate) use crate::game::fe::festaterewards::tests::pool_4 as pool_5;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_init_3 as heap_init_7;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_8;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3 as BE_5;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD_4;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals_8;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_10;
    pub(crate) use crate::game::bx::bxstring::tests::heap;
    pub(crate) use crate::game::dirtysock::tags2::tests::pool_7 as pool_6;
    pub(crate) use crate::game::bx::bxstring::tests::put_str_2 as put_str_5;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2 as W_5;
    pub(crate) use crate::game::bx::bxstring2::tests::RM as RM_2;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE_7;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_14;
    pub(crate) use crate::game::bx::bxstring2::tests::slot_2 as slot_10;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::LEAF;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE_DUMMY;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD_4 as WORLD_9;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base_14 as base_19;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::check3;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_10;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_20;


    #[test]
    fn fe_async_manager_get_state_matches() {
        check_lohi(
            ssx3_ee::s03::f_001a39c0__generated,
            fe_async_manager_get_state,
            300,
            &|c, seed| {
                let mut r = Rng(0x1234_5678 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 16) as u64;
                fill(c, &mut r, DATA, 0x2000);
            },
        );
    }


    #[test]
    fn fe_async_manager_set_state_matches() {
        check_lohi(
            ssx3_ee::s03::f_001a39d8__generated,
            fe_async_manager_set_state,
            300,
            &|c, seed| {
                let mut r = Rng(0x2345_6789 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 16) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                fill(c, &mut r, DATA, 0x2000);
            },
        );
    }


    #[test]
    fn cfe_state_net_messenger_get_list_index_matches() {
        check(
            ssx3_ee::s03::f_001a4740,
            cfe_state_net_messenger_get_list_index,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0009 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.bus.write32(DATA + 0x6f4, r.next_u32());
            },
        );
    }


    #[test]
    fn cfe_state_create_step1_all_fields_filled_matches() {
        check(
            ssx3_ee::s03::f_001b9558,
            cfe_state_create_step1_all_fields_filled,
            300,
            &|c, seed| {
                let mut r = Rng(0x1111_000a ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let s = DATA + 0x100;
                c.bus.write32(DATA + 0x6d0, s);
                for k in 0..4u32 {
                    let q = DATA + 0x200 + k * 0x20;
                    c.bus.write32(s + k * 4, q);
                    let v = if (seed >> k) & 1 == 0 { 0 } else { r.next_u32() | 1 };
                    c.bus.write32(q - 8, v);
                }
            },
        );
    }


    #[test]
    fn cfe_state_create_step1_char_in_range_matches() {
        check(
            ssx3_ee::s03::f_001b9f68,
            cfe_state_create_step1_char_in_range,
            300,
            &|c, seed| {
                let mut r = Rng(0x1111_000b ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }


    #[test]
    fn cfe_state_create_step2_is_leap_year_matches() {
        check_lohi(
            ssx3_ee::s03::f_001bb968,
            cfe_state_create_step2_is_leap_year,
            300,
            &|c, seed| {
                let mut r = Rng(0x1111_000c ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = seed as u64;
            },
        );
    }


    #[test]
    fn cfepopupconfirm_cfg_set_option_string_matches() {
        check(
            ssx3_ee::s03::f_001c5780,
            cfepopupconfirm_cfg_set_option_string,
            100,
            &|c, seed| {
                let mut r = Rng(0x1111_000d ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }


    #[test]
    fn cfepopupconfirm_cfg_set_option_wide_string_matches() {
        check(
            ssx3_ee::s03::f_001c57a0,
            cfepopupconfirm_cfg_set_option_wide_string,
            100,
            &|c, seed| {
                let mut r = Rng(0x1111_000e ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }


    #[test]
    fn cfepopupconfirm_cfg_set_option_text_id_matches() {
        check(
            ssx3_ee::s03::f_001c57c0,
            cfepopupconfirm_cfg_set_option_text_id,
            100,
            &|c, seed| {
                let mut r = Rng(0x1111_000f ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }


    #[test]
    fn cfepopupconfirm_cfg_get_option_matches() {
        check(
            ssx3_ee::s03::f_001c5800,
            cfepopupconfirm_cfg_get_option,
            100,
            &|c, seed| {
                let mut r = Rng(0x1111_0010 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
            },
        );
    }


    #[test]
    fn cfepopupconfirm_cfg_set_option_confirm_wide_matches() {
        check_lohi(
            ssx3_ee::s03::f_001c5838,
            cfepopupconfirm_cfg_set_option_confirm_wide,
            100,
            &|c, seed| {
                let mut r = Rng(0x1111_0011 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.g[A3][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }


    #[test]
    fn cfepopupconfirm_cfg_clear_option_confirm_matches() {
        check_lohi(
            ssx3_ee::s03::f_001c5860,
            cfepopupconfirm_cfg_clear_option_confirm,
            100,
            &|c, seed| {
                let mut r = Rng(0x1111_0012 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }


    #[test]
    fn cfepopupconfirm_cfg_set_option_confirm_text_id_matches() {
        check_lohi(
            ssx3_ee::s03::f_001c5878,
            cfepopupconfirm_cfg_set_option_confirm_text_id,
            100,
            &|c, seed| {
                let mut r = Rng(0x1111_0013 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.g[A3][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }


    #[test]
    fn cfepopupconfirm_cfg_get_option_confirm_matches() {
        check_lohi(
            ssx3_ee::s03::f_001c58d0,
            cfepopupconfirm_cfg_get_option_confirm,
            100,
            &|c, seed| {
                let mut r = Rng(0x1111_0014 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }


    #[test]
    fn async_file_assign_serial_matches() {
        check(ssx3_ee::s14::f_003def40, async_file_assign_serial, 100, &|c, seed| {
            let mut r = Rng(0x3def_4000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
            let serial = match seed % 4 {
                0 => 0xffff_ff00,
                1 => 0,
                2 => r.next_u32(),
                _ => 0xffff_ffff,
            };
            c.bus.write32(ASYNC_SERIAL, serial);
        });
    }


    /// The lazily built BE singleton (`gp - 0x1f04`) pointed at a zeroed dummy:
    /// with it zero some callees run to the file system looking for disc data.
    pub(crate) fn be_library(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6_0000);
        c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
    }


    /// The allocator mutex (`0x538b00`) marked as held by the fake
    /// `GetThreadId` result, so allocations take their fast path.
    pub(crate) fn allocator(c: &mut Cpu) {
        c.bus.write32(0x0053_8b00 + 4, 0x42);
        c.bus.write32(crate::difftest::fakes::LOG_RESULTS, 0x42);
    }


    #[test]
    fn netfe_state_set_selected_user_matches() {
        check(ee::s03::f_001a9890__generated, netfe_state_set_selected_user, 200, &|c, seed| {
            machine(c, seed, 0x1a_9890, 0);
            let src = DATA + 0x800;
            match seed % 3 {
                0 => put_str(c, src, ""),
                1 => put_str(c, src, "abc"),
                _ => put_str(c, src, "selected user"),
            }
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = src as u64;
        });
    }


    #[test]
    fn cfe_state_stats_reset_results_page_matches() {
        check(ee::s03::f_001c4b78__generated, cfe_state_stats_reset_results_page, 50, &|c, seed| {
            let mut r = machine(c, seed, 0x1c_4b78, 0);
            be_library(c);
            allocator(c);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            c.g[A1][0] = (r.next_u32() % 4) as u64;
            c.bus.write32(DATA + 0x1000 + 0x6e8, r.next_u32());
        });
    }


    #[test]
    fn net_config_auth_popup_on_widget_action_matches() {
        crate::difftest::check(ee::s03::f_001b6770__generated, net_config_auth_popup_on_widget_action, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x1b67_70, 2);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x300);
            c.g[A2][0] = if seed % 2 == 0 { 0xd } else { (r.next_u32() & 0xff) as u64 };
            c.bus.write32(DATA + 4, r.next_u32());
        });
    }


    /// A vtable for `*(obj + 8)` whose two slots call the fake (delta 0, so
    /// the callee is `obj` itself).
    pub(crate) fn plain_vtable(c: &mut Cpu, r: &mut Rng) -> u32 {
        vtable(c, r, &[0x40, 0x48])
    }


    #[test]
    fn c_fe_state_challenge_popup_transition_in_matches() {
        covered(
            ssx3_ee::s03::f_001adfb8__generated,
            c_fe_state_challenge_popup_transition_in,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0xa2, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                // Half the seeds get a real vtable (non-zero delta and method);
                // the rest point at zeros (a call through the miss hook).
                if seed % 2 == 0 {
                    let vt = vtable(c, &mut r, &[0x100]);
                    c.bus.write32(DATA + 8, vt);
                }
            },
        );
    }


    #[test]
    fn c_fe_state_net_config_shutdown_matches() {
        check(
            ssx3_ee::s03::f_001b4168__generated,
            c_fe_state_net_config_shutdown,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0xa4, 3);
                globals(c);
                c.g[A0][0] = sx(DATA);
            },
        );
    }


    /// The five `OnWidgetAction`/`v24` wrappers share one shape.
    pub(crate) fn widget_action_test(generated: GuestFn, hand: GuestFn, salt: u64, obj_off: u32) {
        check(generated, hand, 40, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            globals(c);
            c.g[A0][0] = sx(DATA);
            // `*(this + obj_off)` is the widget object; its vtable (at +8) is
            // zero for odd seeds and a fake vtable for even ones.
            c.bus.write32(
                DATA + obj_off,
                if seed % 2 == 0 { DATA + 0x3000 } else { 0 },
            );
        });
    }


    #[test]
    fn net_config_auth_popup_v24_matches() {
        widget_action_test(
            ssx3_ee::s03::f_001b6740__generated,
            net_config_auth_popup_v24,
            0xa5,
            0x20,
        );
    }


    #[test]
    fn c_fe_state_auth_dnas_error_v24_matches() {
        widget_action_test(
            ssx3_ee::s03::f_001b6e10__generated,
            c_fe_state_auth_dnas_error_v24,
            0xa6,
            0x20,
        );
    }


    #[test]
    fn c_fe_state_net_account_mem_v24_matches() {
        widget_action_test(
            ssx3_ee::s03::f_001b8860__generated,
            c_fe_state_net_account_mem_v24,
            0xa7,
            0x20,
        );
    }


    #[test]
    fn c_fe_state_net_create_room_popup_v31_matches() {
        widget_action_test(
            ssx3_ee::s03::f_001bed28__generated,
            c_fe_state_net_create_room_popup_v31,
            0xa8,
            0x20,
        );
    }


    #[test]
    fn c_fe_state_lobby_menu_refresh_player_list_matches() {
        covered(
            ssx3_ee::s03::f_001be510__generated,
            c_fe_state_lobby_menu_refresh_player_list,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0xaa, 30);
                globals(c);
                // `FillPlayerList`'s "-1 = reset" path: 10 list entries whose
                // objects carry the fake vtable at +8 (slots 0x40/0x48).
                c.g[A1][0] = u64::MAX;
                c.g[A0][0] = sx(DATA);
                let vt = plain_vtable(c, &mut r);
                for k in 0..10u32 {
                    let obj = DATA + 0x8000 + 0x100 * k;
                    c.bus.write32(DATA + 0x6d8 + 4 * k, obj);
                    c.bus.write32(obj + 8, vt);
                }
                // `UpdatePlayerScrollArrows`: two list objects behind the two
                // pointer slots, and the rider-count fields that pick the branch.
                for obj in [DATA + 0x9000, DATA + 0xa000] {
                    c.bus.write32(obj + 8, vt);
                }
                c.bus.write32(DATA + 0x728, DATA + 0x9000);
                c.bus.write32(DATA + 0x72c, DATA + 0xa000);
                match seed % 4 {
                    0 => {}
                    1 => c.bus.write32(DATA + 0x77c, 0x100),
                    2 => c.bus.write32(DATA + 0x778, 1),
                    _ => {
                        c.bus.write32(DATA + 0x778, 1);
                        c.bus.write32(DATA + 0x77c, 0x100);
                    }
                }
            },
        );
    }


    /// The behaviour library singleton (a zeroed dummy, so
    /// `BELibrary_GetInstance` neither constructs one nor reads the disc) and
    /// a world whose race manager holds one rider (rider loops in the sound
    /// and UI code would otherwise walk a garbage count and run away).
    pub(crate) fn globals_2(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6_c000);
        let (world, ail, arr) = (DATA + 0x6_8000, DATA + 0x6_8100, DATA + 0x6_8200);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x84, ail);
        c.bus.write32(ail + 0xc, arr);
        c.bus.write32(arr + 8, 20);
        c.bus.write32(arr + 0x28, DATA + 0x6_8400);
        c.bus.write32(arr + 0x7c, 1);
    }


    /// Zero `words` words from the object page, so no callee reads a stale
    /// value.
    pub(crate) fn clear(c: &mut Cpu, base: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(base + 4 * i, 0);
        }
    }


    // ---------------------------------------------------------------- net/fe

    #[test]
    fn net_fe_state_on_child_event_ignore_close_matches() {
        check(ee::s03::f_001a9638__generated, net_fe_state_on_child_event_ignore_close, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x1a9638, 1);
            globals_2(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = r.next_u32() as u64;
            // 0xf (ignored), 0x14 (ignored), and an id the base handler
            // forwards without reaching its jump table.
            c.g[A2][0] = match seed % 3 {
                0 => 0xf,
                1 => 0x14,
                _ => (3 + r.next_u32() % 4) as u64,
            };
            c.g[A3][0] = r.next_u32() as u64;
        });
    }


    #[test]
    fn cfe_state_net_account_mem_store_account_strings_matches() {
        check(
            ee::s03::f_001b7778__generated,
            cfe_state_net_account_mem_store_account_strings,
            60,
            &|c, seed| {
                machine(c, seed, 0x1b7778, 2);
                clear(c, DATA, 0x200);
                c.g[A0][0] = DATA as u64;
                // NUL-terminated name (+0x6e0) and password (+0x6f1).
                for i in 0..8u32 {
                    c.bus.write8(DATA + 0x6e0 + i, b'A' + i as u8);
                }
                c.bus.write8(DATA + 0x6e0 + 8, 0);
                c.bus.write8(DATA + 0x6e8, 0);
                for i in 0..8u32 {
                    c.bus.write8(DATA + 0x6f1 + i, b'k' + i as u8);
                }
                c.bus.write8(DATA + 0x6f1 + 8, 0);
            },
        );
    }


    #[test]
    fn c_fe_popup_confirm_cfg_dtor_matches() {
        dtor_case(ee::s03::f_001c5750__generated, c_fe_popup_confirm_cfg_dtor, 0x1c5750);
    }


    /// `BELibrary_GetInstance` fast path: the singleton must be non-null,
    /// otherwise it allocates and initialises itself.
    pub(crate) fn be_library_2(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3000);
    }


    /// The engine's size-class pool allocator (`gp + 0xdd8` flag and the pool
    /// heads at `gp + 0x2a60..0x2a84`), so constructors that allocate do not
    /// grow the heap from a zeroed header.
    pub(crate) fn pool_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x32000),
            (0x2a7c, 0x25800),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        let nodes = DATA + 0x28_000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes + 0x100 * i as u32;
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                c.bus.write32(node, if n == 3 { 0 } else { node + 0x40 });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
        }
    }


    #[test]
    fn fe_screen_blank_ctor_matches() {
        check(ssx3_ee::s03::f_001abf70__generated, fe_screen_blank_ctor, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0xabf70, 0);
            pool_allocator(c);
            be_library_2(c);
            let self_ = DATA + 0x100;
            fill(c, &mut r, self_, 0x40);
            c.g[A0][0] = sx(self_);
        });
    }


    #[test]
    fn fe_lobby_menu_on_keyboard_done_matches() {
        check(ssx3_ee::s03::f_001bd428__generated, fe_lobby_menu_on_keyboard_done, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0xbd428, 0);
            let self_ = DATA + 0x100;
            let menu = DATA + 0x800;
            c.bus.write32(self_ + 0x69c, menu);
            // Not state 8: leave without requesting a room (the join path
            // needs a live lobby menu; see the report).
            c.bus.write32(menu + 0x18, r.next_u32() % 7);
            c.g[A0][0] = sx(self_);
        });
    }


    #[test]
    fn fe_popup_confirm_cfg_ctor_matches() {
        check(ssx3_ee::s03::f_001c55f8__generated, fe_popup_confirm_cfg_ctor, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0xc55f8, 0);
            let self_ = DATA + 0x100;
            fill(c, &mut r, self_, 0x40);
            c.g[A0][0] = sx(self_);
        });
    }


    #[test]
    fn char_to_list_index_matches() {
        check(ee::s03::f_001b0f28__generated, char_to_list_index, 80, &|c, seed| {
            let _ = machine(c, seed, 0x1b0f_28, 0);
            c.g[A0][0] = sx(seed % 12);
            for i in 0..10u32 {
                let v = if seed % 3 == 0 {
                    (i + 5) as u8
                } else {
                    (i % 12) as u8
                };
                c.bus.write8(0x0044_0f68 + i, v);
            }
        });
    }


    #[test]
    fn fe_state_chat_room_on_activate_matches() {
        check(ssx3_ee::s03::f_001b3528__generated, fe_state_chat_room_on_activate, 60, &|c, seed| {
            machine(c, seed, 0x1b35_28, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
        });
    }


    #[test]
    fn fe_state_net_quick_play_popup_on_child_event_matches() {
        check(ssx3_ee::s03::f_001c1b48__generated, fe_state_net_quick_play_popup_on_child_event, 60, &|c, seed| {
            machine(c, seed, 0x1c1b_48, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x300) as u64;
            let event = if seed % 2 == 0 { 0x82f } else { 0x900 };
            w(c, DATA + 0x300 + 0x18, event);
        });
    }


    #[test]
    fn fe_state_stats_on_widget_notify_matches() {
        check(ssx3_ee::s03::f_001c4b98__generated, fe_state_stats_on_widget_notify, SEEDS_3, &|c, seed| {
            machine(c, seed, 0x1c4b_98, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.g[A2][0] = match seed % 3 {
                0 => 4,
                1 => 6,
                _ => 8,
            };
            c.g[A3][0] = 0x1234;
        });
    }


    pub(crate) fn net_fe_state_ctor_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 30, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            globals_3(c);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x3000);
        });
    }


    #[test]
    fn c_fe_state_net_ssx_world_ctor_matches() {
        net_fe_state_ctor_test(
            ssx3_ee::s03::f_001c1c50__generated,
            c_fe_state_net_ssx_world_ctor,
            0x17,
        );
    }


    #[test]
    fn c_fe_state_net_quick_messages_ctor_matches() {
        net_fe_state_ctor_test(
            ssx3_ee::s03::f_001c20f8__generated,
            c_fe_state_net_quick_messages_ctor,
            0x18,
        );
    }


    #[test]
    fn c_fe_state_net_news_ctor_matches() {
        net_fe_state_ctor_test(ssx3_ee::s03::f_001c5018__generated, c_fe_state_net_news_ctor, 0x19);
    }


    #[test]
    fn c_fe_use_existing_account_validate_fields_matches() {
        check(
            ssx3_ee::s03::f_001ac408__generated,
            c_fe_use_existing_account_validate_fields,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x1f, 1);
                c.g[A0][0] = sx(DATA);
                // The BE library holds the account name (`+0x48`) and password
                // (`+0x4c`) strings; both non-empty keeps the popup path out.
                c.bus.write32(GAME_GP - 0xc8, DATA + 0x3000);
                c.bus.write32(DATA + 0x3048, DATA + 0x5000);
                c.bus.write32(DATA + 0x304c, DATA + 0x5100);
                put_str(c, DATA + 0x5000, if seed % 2 == 0 { "player" } else { "x" });
                put_str(c, DATA + 0x5100, "secret");
            },
        );
    }


    #[test]
    fn c_fe_state_net_config_on_popup_event_passthrough_matches() {
        check(
            ssx3_ee::s03::f_001b4c00__generated,
            c_fe_state_net_config_on_popup_event_passthrough,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x20, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 0;
                c.g[A2][0] = match seed % 3 {
                    0 => 0xf,
                    1 => 0x14,
                    _ => 0x30,
                };
                // Bypass the keyboard-popup shortcut so the child event goes
                // through the dispatcher (which lands in its default case).
                c.bus.write32(DATA + 0x69c, 0x1234);
            },
        );
    }


    /// The game's pool allocator: `gp_2+0xdd8` ready plus four free nodes in
    /// each of the seven size classes (8 to 512 bytes).
    pub(crate) fn pool_allocator_2(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x32000),
            (0x2a7c, 0x25800),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        let nodes = DATA + 0x2_8000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool = DATA + 0x2_0000 + 0x100 * i;
            let base = nodes + 0x100 * i;
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                c.bus.write32(node, if n == 3 { 0 } else { node + 0x40 });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
        }
    }


    /// A dummy movie-player object whose method slot `off` calls the fake
    /// callee.
    pub(crate) fn fake_method(c: &mut Cpu, base: u32, off: u32) {
        c.bus.write16(base + off, 0);
        c.bus.write32(base + off + 4, FAKE);
    }


    /// `NetFEState_ctor` allocates from the pool: the constructors below need
    /// the dummy BE library and the allocator.
    pub(crate) fn net_fe_state_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        let _ = machine(c, seed, salt, 0);
        be_library_3(c);
        pool_allocator_2(c);
        c.g[A0][0] = sx(DATA + 0x100);
        c.g[A1][0] = 0;
    }


    #[test]
    fn fe_state_select_persona_ctor_matches() {
        check(
            ssx3_ee::s03::f_001ad130__generated,
            fe_state_select_persona_ctor,
            40,
            &|c, seed| {
                net_fe_state_fixture(c, seed, 0x1ad130);
            },
        );
    }


    #[test]
    fn fe_state_challenge_popup_v33_matches() {
        check(
            ssx3_ee::s03::f_001af3e0__generated,
            fe_state_challenge_popup_v33,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x1af3e0, 0);
                be_library_3(c);
                let this_ = DATA + 0x100;
                // 0xf/0x14 close the popup list; other codes reach the
                // net-state event dispatcher (which returns at once for them).
                let code: u64 = [0x0f, 0x14, 0, 1][seed as usize % 4];
                c.bus.write32(this_ + 0x10, DATA + 0x1000);
                c.bus.write32(this_ + 0x69c, 0);
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = sx(DATA + 0x1400);
                c.g[A2][0] = code;
            },
        );
    }


    #[test]
    fn fe_state_challenge_received_popup_v33_matches() {
        check(
            ssx3_ee::s03::f_001b0628__generated,
            fe_state_challenge_received_popup_v33,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x1b0628, 0);
                be_library_3(c);
                let this_ = DATA + 0x100;
                let code: u64 = [0x0f, 0x14, 0, 1][seed as usize % 4];
                c.bus.write32(this_ + 0x10, DATA + 0x1000);
                c.bus.write32(this_ + 0x69c, 0);
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = sx(DATA + 0x1400);
                c.g[A2][0] = code;
            },
        );
    }


    #[test]
    fn fe_state_online_character_select_handle_input_matches() {
        check(
            ssx3_ee::s03::f_001b1590__generated,
            fe_state_online_character_select_handle_input,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x1b1590, 0);
                be_library_3(c);
                let this_ = DATA + 0x100;
                let input = DATA + 0x1000;
                let vt = DATA + 0x2000;
                // Menu enabled, or disabled with the pad global set: the early
                // return. Otherwise the input's handler is called.
                let enabled = seed % 4 != 0;
                c.bus.write32(this_ + 0x6dc, if enabled { 1 } else { 0 });
                c.bus
                    .write32(GAME_GP.wrapping_add(0x2dc), if enabled { 0 } else { 1 });
                c.bus.write32(input + 8, vt);
                fake_method(c, vt, 0x30);
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = sx(input);
            },
        );
    }


    #[test]
    fn fe_state_create_step2_ctor_matches() {
        check(
            ssx3_ee::s03::f_001ba3e0__generated,
            fe_state_create_step2_ctor,
            40,
            &|c, seed| {
                net_fe_state_fixture(c, seed, 0x1ba3e0);
            },
        );
    }


    #[test]
    fn fe_state_net_quick_play_popup_update_matches() {
        check(
            ssx3_ee::s03::f_001c1038__generated,
            fe_state_net_quick_play_popup_update,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x1c1038, 0);
                be_library_3(c);
                pool_allocator_2(c);
                let this_ = DATA + 0x100;
                c.bus
                    .write32(this_ + 0x6ec, if seed % 3 == 0 { 0 } else { 1 });
                let left: [u32; 4] = [0, 1, 5, 30];
                c.bus.write32(this_ + 0x6e8, left[seed as usize % 4]);
                c.g[A0][0] = sx(this_);
            },
        );
    }


    /// The main heap's arena header (initialised by [`heap_init`]).
    pub(crate) const MEM_HEAP: u32 = 0x004f_f250;


    pub(crate) fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(
            c.pc,
            crate::difftest::RA,
            "setup call {addr:#x} did not return"
        );
        c.g[V0][0]
    }


    /// `gp_3 + 0xdd8` says the size-class pools are ready, and the pool free
    /// lists are populated: on a fresh machine's zeroed globals the pool
    /// allocator forwards to the heap, which forwards back, forever.
    pub(crate) fn pool_allocator_3(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),
            (0x2a60, 0x2000, 0x10),
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes;
            for n in 0..16u32 {
                let node = base + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
            nodes += block * 16;
        }
    }


    /// A dlmalloc arena over 256 KiB of test RAM (allocations above 0x200
    /// bytes, and the frees of them, come from here).
    pub(crate) fn heap_init(c: &mut Cpu) {
        run(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x40000) as u64, 0x40000],
        );
    }


    /// A machine for functions that call into game code: fake callees for the
    /// misses, the pool free lists, a real heap arena, and (for some seeds) an
    /// unwind at one of the function's first `calls` faked calls.
    pub(crate) fn call_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        pool_allocator_3(c);
        heap_init(c);
        c.bus.write32(fakes::LOG_COUNT, 0);
        c.bus.write32(fakes::LOG_NEXT, fakes::LOG_START);
        let unwind = if calls > 0 && r.next_u32() % 3 == 0 {
            r.next_u32() % calls
        } else {
            u32::MAX
        };
        c.bus.write32(fakes::LOG_UNWIND_AT, unwind);
        r
    }


    /// A `{delta, fn}` vtable entry at `vt + off`.
    pub(crate) fn slot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, target: u32) {
        c.bus
            .write16(vt + off, (r.next_u32() as i16 as i32 & !3) as u16);
        c.bus.write32(vt + off + 4, target);
    }


    // ----------------------------------------------------------------- child events

    #[test]
    fn c_fe_state_challenge_popup_on_child_event_matches() {
        check(
            ee::s03::f_001af428__generated,
            c_fe_state_challenge_popup_on_child_event,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1af4_28, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(if seed % 2 == 0 { 0 } else { 0xe });
                let kind = if seed % 3 == 0 {
                    0x835
                } else {
                    0x900 + seed % 7
                };
                c.bus.write32(DATA + 0x418, kind);
                let vt = DATA + 0x800;
                c.bus.write32(this + 8, vt);
                slot(c, &mut r, vt, 0x108, FAKE + 0x10);
                // Not the keyboard popup (a different child of the state).
                c.bus.write32(this + 0x69c, 0xdead_beef);
            },
        );
    }


    #[test]
    fn c_fe_state_challenge_received_popup_on_child_event_matches() {
        check(
            ee::s03::f_001b05e0__generated,
            c_fe_state_challenge_received_popup_on_child_event,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1b05_e0, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(if seed % 2 == 0 { 0 } else { 0xe });
                let kind = if seed % 3 == 0 {
                    0x835
                } else {
                    0x8ff + seed % 5
                };
                c.bus.write32(DATA + 0x418, kind);
                let vt = DATA + 0x800;
                c.bus.write32(this + 8, vt);
                slot(c, &mut r, vt, 0x108, FAKE + 0x10);
                c.bus.write32(this + 0x69c, 0xdead_beef);
            },
        );
    }


    #[test]
    fn c_fe_state_chat_room_on_widget_notify_matches() {
        check(
            ee::s03::f_001b3568__generated,
            c_fe_state_chat_room_on_widget_notify,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1b35_68, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                let sender = match seed % 4 {
                    0 | 1 => DATA + 0x200,
                    2 => DATA + 0x300,
                    _ => DATA + 0x500,
                };
                c.g[A1][0] = sx(sender);
                c.g[A2][0] = sx(if seed % 3 == 0 {
                    4
                } else {
                    r.next_u32() & 0xff
                });
                c.g[A3][0] = r.next_u32() as u64;
                c.bus.write32(this + 0x6fc, DATA + 0x200);
                c.bus.write32(this + 0x700, DATA + 0x300);
            },
        );
    }


    // -------------------------------------------------- net account mem stepping

    pub(crate) fn net_account_mem_machine(c: &mut Cpu, seed: u32, salt: u64, from_a: u32, from_b: u32) {
        let mut r = call_machine(c, seed, salt, 1);
        c.bus.write32(GAME_GP + 0x2e0, DATA + 0x1000);
        let mode = match seed % 4 {
            0 => from_a,
            1 => from_b,
            _ => r.next_u32() & 0x1f,
        };
        c.bus.write32(DATA + 0x1000, mode);
        c.g[A0][0] = sx(DATA + 0x100);
    }


    #[test]
    fn c_fe_state_net_account_mem_update_state08_matches() {
        check(
            ee::s03::f_001b78f0__generated,
            c_fe_state_net_account_mem_update_state08,
            120,
            &|c, seed| {
                net_account_mem_machine(c, seed, 0x1b78_f0, 8, 9);
            },
        );
    }


    #[test]
    fn c_fe_state_net_account_mem_update_state0e_matches() {
        check(
            ee::s03::f_001b7a08__generated,
            c_fe_state_net_account_mem_update_state0e,
            120,
            &|c, seed| {
                net_account_mem_machine(c, seed, 0x1b7a_08, 0x15, 0x16);
            },
        );
    }


    #[test]
    fn c_fe_state_net_account_mem_update_state10_matches() {
        check(
            ee::s03::f_001b7a50__generated,
            c_fe_state_net_account_mem_update_state10,
            120,
            &|c, seed| {
                net_account_mem_machine(c, seed, 0x1b7a_50, 0xc, 0xd);
            },
        );
    }


    #[test]
    fn c_fe_state_net_account_mem_update_state1a_matches() {
        check(
            ee::s03::f_001b7c28__generated,
            c_fe_state_net_account_mem_update_state1a,
            120,
            &|c, seed| {
                net_account_mem_machine(c, seed, 0x1b7c_28, 0x10, 0x11);
            },
        );
    }


    #[test]
    fn c_fe_state_net_account_mem_update_state1b_matches() {
        check(
            ee::s03::f_001b7c70__generated,
            c_fe_state_net_account_mem_update_state1b,
            120,
            &|c, seed| {
                net_account_mem_machine(c, seed, 0x1b7c_70, 0xc, 0xd);
            },
        );
    }


    // ------------------------------------------------------------ state ctors

    #[test]
    fn c_fe_state_login_ctor_matches() {
        check(
            ee::s03::f_001bee30__generated,
            c_fe_state_login_ctor,
            120,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x1bee_30, 3);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() & 0xffff);
                c.bus.write32(this + 0x10, DATA + 0x200);
            },
        );
    }


    #[test]
    fn c_fe_state_online_agreement_ctor_matches() {
        check(
            ee::s03::f_001c0418__generated,
            c_fe_state_online_agreement_ctor,
            120,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x1c04_18, 3);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() & 0xffff);
                c.bus.write32(this + 0x10, DATA + 0x200);
            },
        );
    }


    #[test]
    fn c_fe_state_online_agreement_on_widget_created_matches() {
        check(
            ee::s03::f_001c0ce0__generated,
            c_fe_state_online_agreement_on_widget_created,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1c0c_e0, 1);
                let item = DATA + 0x100;
                c.g[A0][0] = sx(item);
                c.g[A1][0] = sx(item);
                // The hash of the (zeroed) agreement text, or something else.
                c.bus
                    .write32(item + 0x38, if seed % 2 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(item + 0x14, r.next_u32());
            },
        );
    }


    /// The engine's size-class pool allocator: `gp+0xdd8` ready plus 64 free
    /// nodes in each of the seven size classes, spaced by that class's block
    /// size (so a returned block never overlaps the next free node). Without
    /// it a class with an empty list recurses through `MEM_PoolAlloc`.
    pub(crate) fn pool_allocator_4(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        // (gp slot, free-list head offset in the pool, block stride)
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 0x40),
            (0x2a60, 0x2000, 0x40),
            (0x2a6c, 0x6000, 0x40),
            (0x2a7c, 0x32000, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut pool = DATA + 0x2_0000;
        let mut base = DATA + 0x6_0000;
        for (slot, head, stride) in POOLS {
            for n in 0..64u32 {
                let node = base + stride * n;
                c.bus.write32(node, if n == 63 { 0 } else { node + stride });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
            pool += 0x100;
            base += 0x8000;
        }
    }


    // -----------------------------------------------------------------------
    // 0x1b0f60 cFEStateOnlineCharacterSelect_ctor.

    #[test]
    fn c_fe_state_online_character_select_ctor_matches() {
        check(ee::s03::f_001b0f60__generated, c_fe_state_online_character_select_ctor, 8, &|c, seed| {
            let _r = machine(c, seed, 0x1b0f_60, 1);
            let this = DATA + 0x1000;
            pool_allocator_4(c);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (seed % 4) as u64;
        });
    }


    // -----------------------------------------------------------------------
    // 0x1be7d8 cFEStateNetCreateRoomPopup_ctor.

    #[test]
    fn c_fe_state_net_create_room_popup_ctor_matches() {
        check(ee::s03::f_001be7d8__generated, c_fe_state_net_create_room_popup_ctor, 8, &|c, seed| {
            let _r = machine(c, seed, 0x1be7_d8, 1);
            let this = DATA + 0x1000;
            pool_allocator_4(c);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (seed % 4) as u64;
            c.bus.write32(GAME_GP + 0xda0, seed.wrapping_mul(7));
        });
    }


    #[test]
    fn cfe_state_net_messenger_handle_input_matches() {
        check(ee::s03::f_001a4170__generated, cfe_state_net_messenger_handle_input, 80, &|c, seed| {
            let mut r = base(c, seed, 0x1a41);
            let s = DATA;
            let menu = DATA + 0x400;
            let vt = put_vtable(c, &mut r, DATA + 0x800, &[0xd8]);
            if seed % 3 == 0 {
                c.bus.write32(vt + 0xdc, ZERO_FN);
                c.bus.write16(vt + 0xd8, 0);
            }
            c.bus.write32(menu + 8, vt);
            c.bus.write32(s + 0x6f8, r.next_u32() % 8);
            fill(c, &mut r, menu, 16);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = sx(menu);
        });
    }


    #[test]
    fn cfe_use_existing_account_dtor_matches() {
        check(ee::s03::f_001ac068__generated, cfe_use_existing_account_dtor, 24, &|c, seed| {
            let _r = base(c, seed, 0x1ac0);
            let flags = if seed % 4 == 0 { 1u32 } else { 0 };
            let obj = if flags == 1 { 0x100 } else { DATA };
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = sx(flags);
        });
    }


    #[test]
    fn cfe_state_create_step1_set_account_name_text_matches() {
        check(ee::s03::f_001b95c0__generated, cfe_state_create_step1_set_account_name_text, 60, &|c, seed| {
            base(c, seed, 0x1b95);
            let s = DATA;
            let screen = DATA + 0x400;
            let name = DATA + 0x1000;
            c.bus.write32(s + 0x40, screen);
            c.bus.write32(s + 0x6d0, name);
            c.bus.write32(name, name + 0x10);
            for k in 0..8u32 {
                c.bus.write8(name + 0x10 + k, b'a' + ((seed + k) % 26) as u8);
            }
            c.bus.write8(name + 0x18, 0);
            fill(c, &mut Rng(0x1b95_0000 ^ seed as u64), screen, 4);
            c.bus.write32(s + 0x40, screen);
            c.g[A0][0] = sx(s);
        });
    }


    #[test]
    fn cfe_state_stats_v31_matches() {
        check(ee::s03::f_001c4cd8__generated, cfe_state_stats_v31, 24, &|c, seed| {
            let _r = base(c, seed, 0x1c4c);
            c.bus.write32(DATA + 0x6dc, DATA + 0x1000);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = if seed % 2 == 0 { 0xe1u64 } else { (seed % 5) as u64 };
        });
    }


    #[test]
    fn net_fe_state_on_activate_matches() {
        check(
            ee::s03::f_001a87d0__generated,
            net_fe_state_on_activate,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1a87_d0, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let vt = vtable(c, &mut r, &[0x100]);
                clear_2(c, this, 0x100);
                c.bus.write32(this + 8, vt);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }


    #[test]
    fn c_fe_use_existing_account_ctor_matches() {
        check(
            ee::s03::f_001ac010__generated,
            c_fe_use_existing_account_ctor,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1ac0_10, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_2(c, this, 0x800);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.bus.write32(gp_at(0xda0), r.next_u32());
            },
        );
    }


    // ------------------------------------------------------------------
    // Front end.

    #[test]
    fn c_fe_state_online_character_select_on_child_event_matches() {
        check(
            ee::s03::f_001b1ec0__generated,
            c_fe_state_online_character_select_on_child_event,
            SEEDS_5,
            &|c, seed| {
                let _ = common(c, seed, 0x1b1ec0, 0);
                let this = DATA + 0x100;
                let child = DATA + 0x200;
                clear_3(c, this, 0x200);
                clear_3(c, child, 0x40);
                c.bus.write32(this + 0x69c, 0);
                let kind = [0x835u32, 0x838, 0x800][(seed % 3) as usize];
                c.bus.write32(child + 0x18, kind);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(child);
                c.g[A2][0] = [0xff, 0xf, 0xff][(seed % 3) as usize] as u64;
                c.bus.write32(gp_at(0x2dc), 0);
            },
        );
    }


    #[test]
    fn c_fe_state_chat_room_update_matches() {
        check_lohi(
            ee::s03::f_001b2320__generated,
            c_fe_state_chat_room_update,
            SEEDS_5,
            &|c, seed| {
                let mut r = common(c, seed, 0x1b2320, 2);
                let this = DATA + 0x100;
                clear_3(c, this, 0x400);
                let q = DATA + 0x1000;
                clear_3(c, q, 0x10);
                c.bus.write32(q + 0x18, 1 + (seed % 5) * 10000 + 1);
                c.bus.write32(gp_at(0x2a74), q);
                let vt = vtable(c, &mut r, &[0xd8, 0xe0]);
                c.bus.write32(this + 8, vt);
                c.g[A0][0] = sx(this);
            },
        );
    }


    #[test]
    fn c_fe_state_net_account_mem_dtor_matches() {
        check(
            ee::s03::f_001b7030__generated,
            c_fe_state_net_account_mem_dtor,
            SEEDS_5,
            &|c, seed| {
                let _ = common(c, seed, 0x1b7030, 0);
                alloc_ready(c);
                let this = heap_block(c, 0x800);
                clear_3(c, this, 0x200);
                c.bus.write32(this + 0x6c0, 0x0048_dcd4); // static empty cBXString
                c.bus.write32(gp_at(0x2e0), 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }


    #[test]
    fn c_fe_state_create_duplicate_ctor_matches() {
        check(
            ee::s03::f_001bbd08__generated,
            c_fe_state_create_duplicate_ctor,
            SEEDS_5,
            &|c, seed| {
                let mut r = common(c, seed, 0x1bbd08, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_3(c, this, 0x800);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }


    #[test]
    fn c_fe_state_net_quick_messages_transition_in_matches() {
        check(
            ee::s03::f_001c23c0__generated,
            c_fe_state_net_quick_messages_transition_in,
            SEEDS_5,
            &|c, seed| {
                let mut r = common(c, seed, 0x1c23c0, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_3(c, this, 0x800);
                let vt = vtable(c, &mut r, &[0x100]);
                c.bus.write32(this + 8, vt);
                c.g[A0][0] = sx(this);
                // RefreshAll formats ten messages: take it only on a few seeds.
                c.g[A1][0] = (seed % 5 == 0) as u64;
            },
        );
    }


    #[test]
    fn c_fe_state_stats_transition_in_matches() {
        check(
            ee::s03::f_001c32b8__generated,
            c_fe_state_stats_transition_in,
            SEEDS_5,
            &|c, seed| {
                let mut r = common(c, seed, 0x1c32b8, 1);
                let this = DATA + 0x100;
                clear_3(c, this, 0x800);
                let vt = vtable(c, &mut r, &[0x100]);
                c.bus.write32(this + 8, vt);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 7 == 0 { 1 } else { 0 };
            },
        );
    }


    #[test]
    fn cfe_state_net_account_mem_update_state02_matches() {
        check(
            ssx3_ee::s03::f_001b77c8__generated,
            cfe_state_net_account_mem_update_state02,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1b77c8, 0);
                // The manager's mode. Only the "leave the state alone" paths are
                // exercised: states 4 and 5 re-enter the full screen state
                // machine, which needs the FE fixtures.
                let modes = [0u32, 1, 3, 6, 0x10, 0xffff_ffff];
                c.bus.write32(0x004a_33d0, DATA + 0x100);
                c.bus.write32(DATA + 0x100, modes[(seed % 6) as usize]);
                c.bus.write32(DATA + 0x6d4, (seed % 2) as u32);
                c.g[A0][0] = DATA as u64;
            },
        );
    }


    #[test]
    fn cfe_state_login_dtor_matches() {
        check(
            ssx3_ee::s03::f_001bee78__generated,
            cfe_state_login_dtor,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1bee78, 0);
                heap_init_2(c);
                let this = alloc(c, 0x800);
                let buf = if seed % 3 == 0 { 0 } else { alloc(c, 0x40) };
                c.bus.write32(this + 0x6d0, buf);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }


    #[test]
    fn cfe_state_online_agreement_dtor_matches() {
        check(
            ssx3_ee::s03::f_001c0a28__generated,
            cfe_state_online_agreement_dtor,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1c0a28, 0);
                heap_init_2(c);
                let this = alloc(c, 0x800);
                let buf = if seed % 3 == 0 { 0 } else { alloc(c, 0x40) };
                c.bus.write32(this + 0x6d0, buf);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }


    #[test]
    fn cfe_state_net_news_on_widget_created_matches() {
        check(
            ssx3_ee::s03::f_001c5310__generated,
            cfe_state_net_news_on_widget_created,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1c5310, 2);
                let this = DATA + 0x400;
                let widget = DATA + 0x500;
                // `GetHashValue32` is deterministic: compute the target hash with
                // the same function so both the match and the miss are covered.
                let target = run_2(c, 0x0031_7670, ssx3_ee::s10::StrHash, &[0x0046_5f48]);
                c.bus.write32(
                    widget + 0x38,
                    if seed % 2 == 0 {
                        target as u32
                    } else {
                        r.next_u32()
                    },
                );
                let vt = vtable(c, &mut r, &[0x38]);
                c.bus.write32(widget + 8, vt);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = widget as u64;
            },
        );
    }


    pub(crate) fn put_str_2(c: &mut Cpu, at: u32, s: &str) {
        for (i, b) in s.bytes().enumerate() {
            c.bus.write8(at + i as u32, b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }


    #[test]
    fn fe_async_manager_find_free_slot_matches() {
        check(
            ssx3_ee::s03::f_001a3bf0__generated,
            fe_async_manager_find_free_slot,
            150,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1a3bf0);
                c.g[A0][0] = DATA as u64;
                // Slot index of the free slot; 5 leaves all five busy.
                let free = if seed % 4 == 0 { 5u32 } else { seed % 5 };
                for i in 0..5u32 {
                    let v = if i == free { 0 } else { r.next_u32() | 1 };
                    c.bus.write32(DATA + 0x114 + i * 0x11c, v);
                }
            },
        );
    }


    // ------------------------------------------------------- net FE handlers

    #[test]
    fn net_fe_state_show_buddy_action_popup_matches() {
        check(
            ssx3_ee::s03::f_001ab370__generated,
            net_fe_state_show_buddy_action_popup,
            40,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1ab370);
                heap_init_3(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                // An existing popup, so the getter does not construct one.
                c.bus.write32(this + 0x6a0, DATA + 0x400);
            },
        );
    }


    #[test]
    fn cfe_state_challenge_received_popup_ctor_matches() {
        check(
            ssx3_ee::s03::f_001afe08__generated,
            cfe_state_challenge_received_popup_ctor,
            100,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1afe08);
                heap_init_3(c);
                let this = DATA + 0x400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = if seed % 3 == 0 {
                    0
                } else {
                    (DATA + 0x100) as u64
                };
                put_str_2(c, DATA + 0x100, "menu");
            },
        );
    }


    #[test]
    fn cfe_state_net_config_on_info_popup_closed_matches() {
        check(
            ssx3_ee::s03::f_001b5cf8__generated,
            cfe_state_net_config_on_info_popup_closed,
            150,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1b5cf8);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.g[A2][0] = if seed % 2 == 0 { 0x16 } else { 0 };
                // The event object's id (unmapped: the base handler's default path).
                c.bus.write32(DATA + 0x318, 0);
                // The info popup object whose virtual "close info" runs.
                c.bus.write32(this + 0x6ec, DATA + 0x500);
                let vt = vtable(c, &mut r, &[0x40]);
                c.bus.write32(DATA + 0x508, vt);
            },
        );
    }


    #[test]
    fn cfe_state_create_step1_on_child_event_matches() {
        check(
            ssx3_ee::s03::f_001b9428__generated,
            cfe_state_create_step1_on_child_event,
            150,
            &|c, seed| {
                let _r = setup(c, seed, 0x1b9428);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.g[A2][0] = (seed % 2) as u64;
                let id = [0u32, 0x81e, 0x82c][(seed % 3) as usize];
                c.bus.write32(DATA + 0x318, id);
                c.bus.write32(DATA + 0x69c, 0);
            },
        );
    }


    #[test]
    fn cfe_state_lobby_menu_on_child_event_matches() {
        check(
            ssx3_ee::s03::f_001bdde8__generated,
            cfe_state_lobby_menu_on_child_event,
            150,
            &|c, seed| {
                let _r = setup(c, seed, 0x1bdde8);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.g[A2][0] = (seed % 2) as u64;
                let id = [0u32, 0x823, 0x82a][(seed % 3) as usize];
                c.bus.write32(DATA + 0x318, id);
                c.bus.write32(DATA + 0x69c, 0);
            },
        );
    }


    #[test]
    fn cfe_state_lobby_menu_hide_room_tabs_matches() {
        check(
            ssx3_ee::s03::f_001bdfd8__generated,
            cfe_state_lobby_menu_hide_room_tabs,
            150,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1bdfd8);
                let this = DATA;
                c.g[A0][0] = this as u64;
                for i in 0..4u32 {
                    let obj = DATA + 0x1000 + 0x100 * i;
                    let vt = vtable(c, &mut r, &[0x48]);
                    c.bus.write32(obj + 8, vt);
                    c.bus.write32(this + 0x730 + 4 * i, obj);
                }
            },
        );
    }


    #[test]
    fn cfe_state_online_main_menu_on_child_event_matches() {
        check(
            ssx3_ee::s03::f_001c02f8__generated,
            cfe_state_online_main_menu_on_child_event,
            150,
            &|c, seed| {
                let _r = setup(c, seed, 0x1c02f8);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.g[A2][0] = (seed % 2) as u64;
                let id = [0u32, 0x82d, 0x846][(seed % 3) as usize];
                c.bus.write32(DATA + 0x318, id);
                c.bus.write32(DATA + 0x69c, 0);
            },
        );
    }


    #[test]
    fn cfe_state_net_quick_messages_on_child_event_matches() {
        check(
            ssx3_ee::s03::f_001c2b48__generated,
            cfe_state_net_quick_messages_on_child_event,
            150,
            &|c, seed| {
                let _r = setup(c, seed, 0x1c2b48);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.g[A2][0] = (seed % 2) as u64;
                let id = [0u32, 0x83a, 0x83b][(seed % 3) as usize];
                c.bus.write32(DATA + 0x318, id);
                c.bus.write32(DATA + 0x69c, 0);
            },
        );
    }


    /// An action/key code covering the small ranges the handlers switch on.
    pub(crate) fn action_key(seed: u32, r: &mut Rng) -> u64 {
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
            SEEDS_6,
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
            SEEDS_6,
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
            SEEDS_6,
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
            SEEDS_6,
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
            SEEDS_6,
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
            SEEDS_6,
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
            SEEDS_6,
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
            SEEDS_6,
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


    /// Zero `words` words at `at`.
    pub(crate) fn clear_4(c: &mut Cpu, at: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(at + 4 * i, 0);
        }
    }


    /// Store a 32-bit word at `at`.
    pub(crate) fn w_2(c: &mut Cpu, at: u32, v: u32) {
        c.bus.write32(at, v);
    }


    /// [`machine`] plus the BELibrary dummy at `gp-0x1f04` (without it
    /// `BELibrary_GetInstance` builds a library and waits on its rewards
    /// file), zeroed once per setup, and a working allocator.
    pub(crate) fn base_2(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        let lib = DATA + 0x7000;
        clear_4(c, lib, 0x100);
        w_2(c, gp_at(-0x1f04), lib as u32);
        // A small world whose race manager's spatial-hash root cell carries
        // level 11 (what `SpatialKey_FromAABB` produces for a zero box).
        // `SpatialHash_Remove` descends into a cell's children until the
        // levels match; with a null cell it recurses on null children forever.
        let world = DATA + 0x8000;
        let man = DATA + 0x8100;
        let cell = DATA + 0x8200;
        clear_4(c, world, 0x40);
        clear_4(c, man, 0x40);
        clear_4(c, cell, 0x40);
        w_2(c, cell, 11);
        w_2(c, gp_at(-0x848), world as u32);
        w_2(c, world + 0x84, man as u32);
        w_2(c, man + 0x20, cell as u32);
        // Allocations up to 0x200 bytes come from the size-class pools.
        // Without the `gp+0xdd8` flag, MEM_PoolAlloc <-> MEM_AllocInternal
        // recurse forever. (Tests that need larger blocks also call
        // `heap_init` for the dlmalloc arena.)
        pool_allocator_5(c);
        r
    }


    /// The engine's size-class pools (`MEM_PoolAlloc`): free lists hung off
    /// `gp + 0x2a60..`, live only when the "pools ready" flag at `gp+0xdd8`
    /// is set. Each list is 16 blocks, enough for the allocations these
    /// callees make.
    pub(crate) fn pool_allocator_5(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),
            (0x2a60, 0x2000, 0x10),
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        // Pool structs stay above the fake-call result pointers (which end
        // at DATA+0x3fff0) and the nodes below the fake-call log.
        let mut nodes = DATA + 0x44_000;
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x40_000 + 0x800 * i as u32;
            let base_node = nodes;
            for n in 0..16u32 {
                let node = base_node + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, base_node);
            c.bus.write32(GAME_GP + slot, pool);
            nodes += block * 16;
        }
    }


    // ------------------------------------------------------------------
    // Front-end.

    #[test]
    fn fe_state_net_messenger_update_matches() {
        check(ee::s03::f_001a3fe0__generated, fe_state_net_messenger_update, 24, &|c, seed| {
            let mut r = base_2(c, seed, 0x1a3f_e0);
            let s = DATA + 0x100;
            clear_4(c, s, 0x800);
            let obj = DATA + 0x1000;
            clear_4(c, obj, 0x40);
            let now = r.next_u32();
            w_2(c, obj + 0x18, now);
            w_2(c, gp_at(0x2a74), obj as u32);
            w_2(
                c,
                s + 0x6dc,
                if seed % 2 == 0 { now.wrapping_sub(1) } else { now.wrapping_sub(100) },
            );
            c.g[A0][0] = sx(s);
        });
    }


    // ------------------------------------------------- rate: front-end timers

    /// A callee outside the translated code: answer null and return at once.
    /// The rate tests below only look at their timer, so callee results and
    /// the arguments passed to them do not matter.
    fn no_callee(c: &mut Cpu, _target: u32) {
        c.g[V0][0] = 0;
        c.pc = c.g[RA][0] as u32;
    }


    /// The app manager at rate `hz` with its VSync counter at `now`, installed
    /// both where `tick::rate` reads it (`tick::APPMAN_PTR`) and where the game
    /// reads it (`gp+0x2a74`, the same `D_004A5B64`).
    fn app_manager(c: &mut Cpu, hz: u32, now: u32) -> u32 {
        crate::tick::set_for_test(c, hz);
        let app = c.bus.read32(crate::tick::APPMAN_PTR);
        c.bus.write32(app + 0x18, now);
        c.bus.write32(GAME_GP.wrapping_add(0x2a74), app);
        app
    }


    /// `cFEStateNetMessenger_Update` (0x1a3fe0) at rate `hz` with the last
    /// refresh `elapsed` VSyncs ago: true when it refreshed. The refresh path
    /// stores the current VSync count at `self+0x6dc`, the wait path leaves it.
    fn messenger_refreshes(hz: u32, elapsed: u32) -> bool {
        const NOW: u32 = 5_000_000;
        let mut c = crate::difftest::fresh();
        let _ = base_2(&mut c, 0, 0x1a3f_e0);
        c.hooks.miss = no_callee;
        app_manager(&mut c, hz, NOW);
        let s = DATA + 0x100;
        clear_4(&mut c, s, 0x800);
        c.bus.write32(s + 0x6dc, NOW - elapsed);
        c.g[A0][0] = sx(s);
        fe_state_net_messenger_update(&mut c);
        assert_eq!(c.pc, crate::difftest::RA, "the update did not return");
        c.bus.read32(s + 0x6dc) == NOW
    }


    /// The player-list refresh is a one-second period of the VSync counter:
    /// 60 VSyncs at 60 Hz, 120 at 120, 240 at 240.
    #[test]
    fn fe_state_net_messenger_refresh_follows_the_rate() {
        for (hz, period) in [(60u32, 60u32), (120, 120), (240, 240)] {
            assert!(
                !messenger_refreshes(hz, period - 1),
                "{hz} Hz: refreshed one VSync early"
            );
            assert!(messenger_refreshes(hz, period), "{hz} Hz: did not refresh");
        }
        // 60 VSyncs are one second only at 60 Hz: at 120 the list waits on.
        assert!(!messenger_refreshes(120, 60));
    }


    /// `cFEStateChatRoom_Update` (0x1b2320) at rate `hz` with the VSync counter
    /// at `now`: true when it ran the player-quality poll (0x1b3bf0), which is
    /// the only thing the `% period` branch adds.
    fn chat_room_polls(hz: u32, now: u32) -> bool {
        let mut c = crate::difftest::fresh();
        let _ = common(&mut c, 0, 0x1b_2320, 0);
        c.hooks.miss = no_callee;
        app_manager(&mut c, hz, now);
        let this = DATA + 0x100;
        clear_4(&mut c, this, 0x400);
        c.g[A0][0] = sx(this);
        ssx3_ee_rt::prof::trace::start();
        c_fe_state_chat_room_update(&mut c);
        let calls = ssx3_ee_rt::prof::trace::take();
        assert_eq!(c.pc, crate::difftest::RA, "the update did not return");
        calls.iter().any(|k| k[0] == 0x001b_3bf0)
    }


    /// The player-quality poll runs every 10000 frames of the VSync counter:
    /// every 10000 VSyncs at 60 Hz, 20000 at 120, 40000 at 240.
    #[test]
    fn c_fe_state_chat_room_poll_follows_the_rate() {
        for (hz, period) in [(60u32, 10_000u32), (120, 20_000), (240, 40_000)] {
            assert!(chat_room_polls(hz, period), "{hz} Hz: no poll at {period}");
            assert!(chat_room_polls(hz, 3 * period), "{hz} Hz: no poll at 3 x {period}");
            assert!(!chat_room_polls(hz, period - 1), "{hz} Hz: poll just before");
            assert!(
                !chat_room_polls(hz, period + period / 2),
                "{hz} Hz: poll half a period late"
            );
        }
        // 10000 VSyncs are the period only at 60 Hz.
        assert!(!chat_room_polls(120, 10_000));
    }


    /// `cFEStateNetQuickPlayPopup_Update` (0x1c1038) at rate `hz`, one update
    /// from `value` in `self+0x6e8`, armed: the counter afterwards.
    fn quick_play_step(hz: u32, value: u32) -> u32 {
        let mut c = crate::difftest::fresh();
        let _ = machine(&mut c, 0, 0x1c1038, 0);
        be_library_3(&mut c);
        pool_allocator_2(&mut c);
        c.hooks.miss = no_callee;
        crate::tick::set_for_test(&mut c, hz);
        let this = DATA + 0x100;
        c.bus.write32(this + 0x6ec, 1); // armed
        c.bus.write32(this + 0x6e8, value);
        c.g[A0][0] = sx(this);
        fe_state_net_quick_play_popup_update(&mut c);
        assert_eq!(c.pc, crate::difftest::RA, "the update did not return");
        c.bus.read32(this + 0x6e8)
    }


    /// The quick-play popup's retry interval is 0.5 s: the reloaded countdown is
    /// 30 ticks at 60 Hz, 60 at 120, 120 at 240, while the countdown itself
    /// still takes one off per update.
    #[test]
    fn fe_state_net_quick_play_popup_interval_follows_the_rate() {
        assert_eq!(quick_play_step(60, 0), 30, "60 Hz reload");
        assert_eq!(quick_play_step(120, 0), 60, "120 Hz reload");
        assert_eq!(quick_play_step(240, 0), 120, "240 Hz reload");
        for hz in [60u32, 120, 240] {
            assert_eq!(quick_play_step(hz, 5), 4, "{hz} Hz: countdown");
        }
    }


    #[test]
    fn fe_state_net_messenger_on_find_user_result_matches() {
        check(ee::s03::f_001a4fb0__generated, fe_state_net_messenger_on_find_user_result, 48, &|c, seed| {
            let mut r = base_2(c, seed, 0x1a4f_b0);
            let s = DATA + 0x100;
            clear_4(c, s, 0x800);
            w_2(c, s + 0x1c, if seed % 2 == 0 { 0 } else { 6 });
            let reply = DATA + 0x400;
            clear_4(c, reply, 0x40);
            w_2(c, s + 0x6a0, if seed % 3 == 0 { 0 } else { reply as u32 });
            w_2(c, reply + 0x18, if seed % 4 == 0 { 0x807 } else { r.next_u32() });
            c.g[A0][0] = sx(s);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = sx(DATA + 0x500);
            clear_4(c, DATA + 0x500, 0x40);
        });
    }


    #[test]
    fn fe_state_net_messenger_set_player_menu_selection_matches() {
        check(ee::s03::f_001a6c40__generated, fe_state_net_messenger_set_player_menu_selection, 48, &|c, seed| {
            let _ = base_2(c, seed, 0x1a6c_40);
            let s = DATA + 0x100;
            clear_4(c, s, 0x40);
            let screen = DATA + 0x300;
            clear_4(c, screen, 0x40);
            if seed % 3 != 0 {
                w_2(c, s + 0x40, screen as u32);
                let objs = DATA + 0x400;
                let count = DATA + 0x440;
                let menu = DATA + 0x480;
                clear_4(c, objs, 0x10);
                clear_4(c, count, 4);
                clear_4(c, menu, 0x100);
                w_2(c, screen + 0x3c, objs as u32);
                w_2(c, screen + 0x38, count as u32);
                if seed % 2 == 0 {
                    w_2(c, count, 0);
                } else {
                    w_2(c, count, 1);
                    w_2(c, objs, menu as u32);
                    w_2(c, menu + 0x38, 0);
                }
            }
            c.g[A0][0] = sx(s);
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { sx(0xffff_ffff) };
        });
    }


    #[test]
    fn net_fe_state_update_matches() {
        check(ee::s03::f_001a8770__generated, net_fe_state_update, 24, &|c, seed| {
            let mut r = base_2(c, seed, 0x1a87_70);
            let s = DATA + 0x100;
            clear_4(c, s, 0x800);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = r.next_u32() as u64;
        });
    }


    #[test]
    fn net_fe_state_open_keyboard_str_matches() {
        check(ee::s03::f_001a9090__generated, net_fe_state_open_keyboard_str, 32, &|c, seed| {
            let mut r = base_2(c, seed, 0x1a90_90);
            let s = DATA + 0x100;
            clear_4(c, s, 0x800);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = sx(DATA + 0x200);
            c.g[A3][0] = sx(DATA + 0x300);
            c.g[T0][0] = r.next_u32() as u64;
            c.g[T1][0] = r.next_u32() as u64;
            c.g[T2][0] = r.next_u32() as u64;
        });
    }


    #[test]
    fn net_fe_state_open_keyboard_text_id_matches() {
        check(ee::s03::f_001a9150__generated, net_fe_state_open_keyboard_text_id, 32, &|c, seed| {
            let mut r = base_2(c, seed, 0x1a91_50);
            let s = DATA + 0x100;
            clear_4(c, s, 0x800);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = sx(DATA + 0x200);
            c.g[A3][0] = sx(DATA + 0x300);
            c.g[T0][0] = r.next_u32() as u64;
            c.g[T1][0] = r.next_u32() as u64;
            c.g[T2][0] = r.next_u32() as u64;
        });
    }


    #[test]
    fn fe_state_net_config_start_card_scan_matches() {
        check(ee::s03::f_001b5268__generated, fe_state_net_config_start_card_scan, 32, &|c, seed| {
            let mut r = base_2(c, seed, 0x1b52_68);
            let s = DATA + 0x100;
            clear_4(c, s, 0x800);
            w_2(c, s + 0x6e4, DATA + 0x400);
            w_2(c, s + 0x6e8, DATA + 0x480);
            w_2(c, s + 0x6f4, DATA + 0x500);
            clear_4(c, DATA + 0x400, 0x40);
            clear_4(c, DATA + 0x480, 0x40);
            clear_4(c, DATA + 0x500, 0x40);
            w_2(c, gp_at(-0x1570), r.next_u32());
            c.g[A0][0] = sx(s);
        });
    }


    #[test]
    fn fe_state_net_account_mem_popup_id_to_state_matches() {
        check(ee::s03::f_001b8890__generated, fe_state_net_account_mem_popup_id_to_state, 120, &|c, seed| {
            base_2(c, seed, 0x1b88_90);
            c.g[A1][0] = match seed % 8 {
                0 => 13,
                1 => 21,
                2 => 7,
                3 => 25,
                4 => 14,
                5 => 0,
                6 => 20,
                _ => sx(0xffff_fff0),
            };
        });
    }


    #[test]
    fn fe_state_create_step1_set_email_text_matches() {
        check(ee::s03::f_001b96e0__generated, fe_state_create_step1_set_email_text, 48, &|c, seed| {
            let mut r = base_2(c, seed, 0x1b96_e0);
            let s = DATA + 0x100;
            clear_4(c, s, 0x800);
            let screen = DATA + 0x300;
            clear_4(c, screen, 0x40);
            if seed % 3 != 0 {
                w_2(c, s + 0x40, screen as u32);
                let objs = DATA + 0x400;
                let count = DATA + 0x440;
                let text = DATA + 0x480;
                clear_4(c, objs, 0x10);
                clear_4(c, count, 4);
                clear_4(c, text, 0x40);
                w_2(c, screen + 0x3c, objs as u32);
                w_2(c, screen + 0x38, count as u32);
                w_2(c, count, 1);
                w_2(c, objs, text as u32);
                w_2(c, text + 0x38, r.next_u32());
                let holder = DATA + 0x500;
                clear_4(c, holder, 0x20);
                w_2(c, s + 0x6d0, holder as u32);
                w_2(c, holder + 0xc, DATA + 0x600);
                // A string with a non-zero length word at `s - 8`.
                w_2(c, DATA + 0x600, 0);
                w_2(c, DATA + 0x5f8, if seed % 2 == 0 { 0 } else { 4 });
            }
            c.g[A0][0] = sx(s);
        });
    }


    /// The lazily built BELibrary singleton (`gp - 0x1f04`): a zeroed dummy,
    /// so nothing tries to read the file system for it.
    pub(crate) fn be_library_4(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3_0000);
    }


    /// A dummy world (`gp - 0x848`) with one rider, so rider loops in the
    /// callees terminate.
    pub(crate) fn world(c: &mut Cpu) {
        let (w, ail, arr) = (DATA + 0x8_0000, DATA + 0x8_1000, DATA + 0x8_2000);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), w);
        c.bus.write32(w + 0x7c, DATA + 0x8_4000);
        c.bus.write32(w + 0x84, ail);
        c.bus.write32(ail + 0xc, arr);
        c.bus.write32(arr + 8, 20);
        c.bus.write32(arr + 0x28, DATA + 0x8_3000);
        c.bus.write32(arr + 0x7c, 1);
    }


    /// The engine's size-class pool allocator: `gp + 0xdd8` ready plus eight
    /// free nodes per size class. The front-end constructors allocate a screen
    /// and its lists through it; a zeroed heap would walk garbage.
    pub(crate) fn pool(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool = DATA + 0x4_0000 + 0x1000 * i;
            let base = DATA + 0x6_0000 + 0x2000 * i;
            for n in 0..8u32 {
                let node = base + 0x400 * n;
                c.bus.write32(node, if n == 7 { 0 } else { node + 0x400 });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
        }
    }


    /// A screen-shaped object whose hash list contains one entry with hash 0
    /// (the test machine's string globals are zero, so `StrHashElf` returns 0).
    pub(crate) fn hash_screen(c: &mut Cpu, screen: u32, node: u32) {
        c.bus.write32(screen + 0x38, DATA + 0x40_10);
        c.bus.write32(DATA + 0x40_10, 1);
        c.bus.write32(screen + 0x3c, DATA + 0x40_00);
        c.bus.write32(DATA + 0x40_00, node);
        c.bus.write32(node + 0x38, 0);
    }


    pub(crate) fn init_setup(c: &mut Cpu, seed: u32, salt: u64) {
        machine(c, seed, salt, 3);
        be_library_4(c);
        world(c);
        pool(c);
        c.g[A0][0] = DATA as u64;
        // A null/zeroed engine makes the hash lookup miss and build a screen.
        c.bus.write32(DATA + 0x10, DATA + 0x1000);
    }


    #[test]
    fn fe_screen_blank_init_matches() {
        check(ee::s03::f_001abfa8__generated, fe_screen_blank_init, 6, &|c, seed| {
            init_setup(c, seed, 0x001a_bfa8);
        });
    }


    #[test]
    fn cfe_state_select_persona_v33_matches() {
        check(
            ee::s03::f_001ad9c8__generated,
            cfe_state_select_persona_v33,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x001a_d9c8, 3);
                be_library_4(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0xf
                } else {
                    (r.next_u32() & 0x1f) as u64
                };
                c.bus.write32(DATA + 0x2000 + 0x18, 0);
            },
        );
    }


    #[test]
    fn cfe_state_auth_dnas_error_init_matches() {
        check(
            ee::s03::f_001b6b50__generated,
            cfe_state_auth_dnas_error_init,
            6,
            &|c, seed| {
                init_setup(c, seed, 0x001b_6b50);
            },
        );
    }


    #[test]
    fn cfe_state_net_account_mem_update_state16_matches() {
        check(
            ee::s03::f_001b7bc0__generated,
            cfe_state_net_account_mem_update_state16,
            12,
            &|c, seed| {
                machine(c, seed, 0x001b_7bc0, 2);
                be_library_4(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(GAME_GP + 0x2e0, DATA + 0x1000);
                c.bus
                    .write32(DATA + 0x1000, if seed % 2 == 0 { 0x10 } else { 0x11 });
            },
        );
    }


    #[test]
    fn cfe_state_create_step1_set_password_text_matches() {
        check(
            ee::s03::f_001b9610__generated,
            cfe_state_create_step1_set_password_text,
            8,
            &|c, seed| {
                machine(c, seed, 0x001b_9610, 4);
                be_library_4(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x40, 0);
                } else {
                    c.bus.write32(DATA + 0x40, DATA + 0x3000);
                    hash_screen(c, DATA + 0x3000, DATA + 0x4200);
                    c.bus.write32(DATA + 0x6d0, DATA + 0x4300);
                    c.bus.write32(DATA + 0x4304, DATA + 0x4400);
                    c.bus.write32(DATA + 0x4400, 0);
                }
            },
        );
    }


    #[test]
    fn cfe_state_create_step1_set_confirm_password_text_matches() {
        check(
            ee::s03::f_001b9678__generated,
            cfe_state_create_step1_set_confirm_password_text,
            8,
            &|c, seed| {
                machine(c, seed, 0x001b_9678, 4);
                be_library_4(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x40, 0);
                } else {
                    c.bus.write32(DATA + 0x40, DATA + 0x3000);
                    hash_screen(c, DATA + 0x3000, DATA + 0x4200);
                    c.bus.write32(DATA + 0x6d0, DATA + 0x4300);
                    c.bus.write32(DATA + 0x4308, DATA + 0x4400);
                    c.bus.write32(DATA + 0x4400, 0);
                }
            },
        );
    }


    #[test]
    fn cfe_state_lobby_menu_on_create_room_popup_event_matches() {
        check(
            ee::s03::f_001bdcf8__generated,
            cfe_state_lobby_menu_on_create_room_popup_event,
            8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x001b_dcf8, 3);
                be_library_4(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0xf
                } else {
                    (r.next_u32() & 0x1f) as u64
                };
                c.bus.write32(DATA + 0x2000 + 0x18, 0);
                c.bus.write32(DATA + 0x2000 + 0x6dc, 0);
                c.bus.write32(DATA + 0x2000 + 0x6e0, 0);
                c.bus.write32(DATA + 0x2000 + 0x6e4, 0);
            },
        );
    }


    #[test]
    fn cfe_state_net_create_room_popup_init_matches() {
        check(
            ee::s03::f_001be828__generated,
            cfe_state_net_create_room_popup_init,
            6,
            &|c, seed| {
                init_setup(c, seed, 0x001b_e828);
            },
        );
    }


    pub(crate) const OBJ: u32 = DATA + 0x1000;

    pub(crate) const OBJ2: u32 = DATA + 0x2000;

    pub(crate) const OBJ3: u32 = DATA + 0x3000;

    pub(crate) const OBJ4: u32 = DATA + 0x4000;

    pub(crate) const VT: u32 = DATA + 0x8000;


    pub(crate) fn w_3(c: &mut Cpu, at: u32, v: u32) {
        c.bus.write32(at, v);
    }


    /// A vtable slot_2 `{ short delta; short; fn }` at `vt + off`.
    pub(crate) fn slot_2(c: &mut Cpu, vt: u32, off: u32, delta: i16, func: u32) {
        c.bus.write16(vt + off, delta as u16);
        w_3(c, vt + off + 4, func);
    }


    /// The BELibrary singleton at `gp_7 - 0x1f04` and the world at `gp_7 - 0x848`
    /// (world -> +0x84 -> +0xc race, rider count at +0x7c).
    pub(crate) fn globals_4(c: &mut Cpu) {
        w_3(c, GAME_GP - 0x1f04, BE_LIBRARY);
        let world = OBJ4;
        w_3(c, GAME_GP - 0x848, world);
        let game = world + 0x1000;
        w_3(c, world + 0x84, game);
        let race = world + 0x2000;
        w_3(c, game + 0xc, race);
        w_3(c, race + 0x7c, 0);
    }


    #[test]
    fn c_fe_state_net_messenger_ctor_matches() {
        check(
            ee::s03::f_001a3dc8__generated,
            c_fe_state_net_messenger_ctor,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x1a3dc8, 0);
                globals_4(c);
                w_3(c, GAME_GP + 0xda0, 0x1234_5678);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { OBJ2 as u64 };
                c.g[A2][0] = if seed % 3 == 0 { 0 } else { OBJ3 as u64 };
            },
        );
    }


    #[test]
    fn net_fe_state_open_messenger_matches() {
        check(
            ee::s03::f_001a9b58__generated,
            net_fe_state_open_messenger,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x1a9b58, 0);
                globals_4(c);
                pool_allocator_6(c);
                c.g[A0][0] = OBJ as u64;
                w_3(c, OBJ + 0x10, OBJ2);
                w_3(c, OBJ + 0x40, OBJ3);
                w_3(c, OBJ3 + 0xd0, OBJ4);
                w_3(c, OBJ4 + 0x10, OBJ2 + 0x40);
                let _ = seed;
            },
        );
    }


    #[test]
    fn c_fe_state_create_step1_restore_account_info_matches() {
        check(
            ee::s03::f_001ba218__generated,
            c_fe_state_create_step1_restore_account_info,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x1ba218, 0);
                globals_4(c);
                c.g[A0][0] = OBJ as u64;
                w_3(c, OBJ + 0x6d0, OBJ2);
                // Same string pointers on both sides: `operator=` is a no-op.
                w_3(c, OBJ2 + 4, if seed % 2 == 0 { 0 } else { OBJ3 });
                w_3(c, OBJ2 + 8, OBJ3);
                w_3(c, OBJ2 + 0xc, 0);
                w_3(c, OBJ + 0x6d4, 0);
                w_3(c, OBJ + 0x6d8, OBJ3);
                w_3(c, OBJ + 0x6dc, 0);
                w_3(c, OBJ + 0x6e0, 0x11);
                w_3(c, OBJ + 0x6e4, 0x22);
            },
        );
    }


    pub(crate) fn init_screen_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 20, &|c, seed| {
            fakes::machine(c, seed, salt, 0);
            globals_4(c);
            c.g[A0][0] = OBJ as u64;
            w_3(c, OBJ + 0x10, if seed % 2 == 0 { 0 } else { OBJ2 });
            if seed % 3 == 0 {
                w_3(c, OBJ + 0x40, 0);
            }
        });
    }


    #[test]
    fn c_fe_state_net_quick_messages_init_matches() {
        init_screen_test(
            ee::s03::f_001c2138__generated,
            c_fe_state_net_quick_messages_init,
            0x1c2138,
        );
    }


    #[test]
    fn c_fe_state_auth_dnas_error_on_widget_action_matches() {
        check(
            ee::s03::f_001b6e40__generated,
            c_fe_state_auth_dnas_error_on_widget_action,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x1b6e40, 0);
                globals_4(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = match seed % 4 {
                    0 => 0,
                    1 => 5,
                    2 => 7,
                    _ => 10,
                };
                w_3(c, OBJ + 0x20, OBJ3);
                w_3(c, OBJ3 + 8, VT);
                slot_2(c, VT, 0x88, 0, fakes::FAKE);
            },
        );
    }


    #[test]
    fn c_fe_state_create_step1_validate_fields_matches() {
        check(
            ee::s03::f_001ba110__generated,
            c_fe_state_create_step1_validate_fields,
            10,
            &|c, seed| {
                fakes::machine(c, seed, 0x1ba110, 0);
                globals_4(c);
                c.g[A0][0] = OBJ as u64;
                w_3(c, OBJ + 0x6d0, if seed % 2 == 0 { 0 } else { OBJ2 });
                w_3(c, OBJ2 + 4, 0);
                w_3(c, OBJ2 + 8, 0);
                w_3(c, OBJ2 + 0xc, 0);
            },
        );
    }


    #[test]
    fn c_fe_state_create_step2_get_days_in_month_matches() {
        check(
            ee::s03::f_001bb9b0__generated,
            c_fe_state_create_step2_get_days_in_month,
            40,
            &|c, seed| {
                fakes::machine(c, seed, 0x1bb9b0, 0);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = match seed % 8 {
                    0 => 2,
                    1 => 1,
                    2 => 7,
                    3 => 8,
                    4 => 12,
                    5 => 13,
                    6 => 0,
                    _ => 0xffff_ffff,
                };
                c.g[A2][0] = match seed % 4 {
                    0 => 2000,
                    1 => 1900,
                    2 => 2023,
                    _ => 2024,
                };
            },
        );
    }


    /// The engine's size-class pool allocator (`gp_7 + 0xdd8` and the free lists
    /// at `gp_7 + 0x2a60..0x2a84`), copied from `oc57`/`oc69`.
    pub(crate) fn pool_allocator_6(c: &mut Cpu) {
        w_3(c, GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x32000),
            (0x2a7c, 0x25800),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        let nodes = DATA + 0x28_000;
        for (i, (slot_off, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes + 0x100 * i as u32;
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                w_3(c, node, if n == 3 { 0 } else { node + 0x40 });
            }
            w_3(c, pool + head, base);
            w_3(c, GAME_GP + slot_off, pool);
        }
    }


    /// The object under test.
    pub(crate) const OBJ_2: u32 = DATA;

    /// A second scratch object.
    pub(crate) const OBJ2_2: u32 = DATA + 0x800;

    /// An event/message object.
    pub(crate) const EV: u32 = DATA + 0x2000;

    /// The zeroed BELibrary dummy.
    pub(crate) const BE: u32 = DATA + 0x3_0000;

    /// The dummy world.
    pub(crate) const WORLD_2: u32 = DATA + 0x8_0000;


    /// The BELibrary singleton (`gp_8 - 0x1f04`, a zeroed dummy so nothing reads
    /// the disc) and the world (`gp_8 - 0x848`, with no riders).
    pub(crate) fn globals_5(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x848), WORLD_2);
        let game = WORLD_2 + 0x1000;
        w(c, WORLD_2 + 0x84, game);
        let race = WORLD_2 + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }


    /// The engine's size-class pool_2 allocator: `gp_8 + 0xdd8` ready plus eight
    /// free nodes per size class (a zeroed heap walks garbage).
    pub(crate) fn pool_2(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            // One 0x40000-byte region per pool_2 (its largest head is at 0x32000)
            // and one node array well away from it, so the head slots never
            // overwrite a free node.
            let pool_2 = DATA + 0x40_0000 + 0x4_0000 * i;
            let base_3 = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base_3 + 0x400 * n;
                w(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w(c, pool_2 + head, base_3);
            w(c, GAME_GP + slot, pool_2);
        }
    }


    /// Scrambled state, fake call hooks, a game-like `gp_8` and the standard
    /// fixtures.
    pub(crate) fn base_3(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        globals_5(c);
        pool_2(c);
        r
    }


    /// A screen whose hash table holds `node` with hash 0 (`StrHash("")` is 0
    /// on the test machine: the tag strings are not loaded).
    pub(crate) fn hash_object(c: &mut Cpu, screen: u32, node: u32) {
        w(c, screen + 0x38, DATA + 0x30_0100);
        w(c, DATA + 0x30_0100, 1);
        w(c, screen + 0x3c, DATA + 0x30_0000);
        w(c, DATA + 0x30_0000, node);
        w(c, node + 0x38, 0);
    }


    /// The UI `*_Init` pattern: a state with a dummy engine, the pool_2 and the
    /// globals_5.
    pub(crate) fn ui_init_setup(c: &mut Cpu, seed: u32, salt: u64) {
        machine(c, seed, salt, 3);
        globals_5(c);
        pool_2(c);
        c.g[A0][0] = OBJ_2 as u64;
        w(c, OBJ_2 + 0x10, DATA + 0x1000);
    }


    // ---------------------------------------------------------------- front end

    #[test]
    fn c_fe_state_online_character_select_on_quit_popup_event_matches() {
        check(
            ee::s03::f_001b1f18__generated,
            c_fe_state_online_character_select_on_quit_popup_event,
            8,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x1b1f18, 2);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = EV as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0xf
                } else {
                    (r.next_u32() & 0x1f) as u64
                };
                w(c, EV + 0x18, 0);
                w(c, OBJ_2 + 0x69c, 0);
                if seed % 4 >= 2 {
                    w(c, GAME_GP.wrapping_add(0x2dc), OBJ2_2);
                    w(c, OBJ2_2, 0xa);
                } else {
                    w(c, GAME_GP.wrapping_add(0x2dc), 0);
                }
            },
        );
    }


    #[test]
    fn c_fe_state_auth_dnas_error_ctor_matches() {
        check(
            ee::s03::f_001b6ab8__generated,
            c_fe_state_auth_dnas_error_ctor,
            6,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x1b6ab8, 2);
                c.g[A0][0] = OBJ_2 as u64;
                for i in 0..5 {
                    c.g[[A1, A2, A3, T0, T1][i]][0] = r.next_u32() as u64;
                }
            },
        );
    }


    #[test]
    fn c_fe_state_create_step2_on_parent_email_entered_matches() {
        check(
            ee::s03::f_001bb0e8__generated,
            c_fe_state_create_step2_on_parent_email_entered,
            8,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x001b_b0e8, 3);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = EV as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0xf
                } else {
                    (r.next_u32() & 0x1f) as u64
                };
                w(c, EV + 0x18, 0);
                let screen = DATA + 0x3800;
                w(c, OBJ_2 + 0x6d0, screen);
                w(c, EV + 0x2b0, DATA + 0x3900);
                c.bus.write8(DATA + 0x3900, b'x');
                c.bus.write8(DATA + 0x3901, 0);
                w(c, screen + 0x10, 0);
            },
        );
    }


    #[test]
    fn c_fe_state_create_step2_set_month_selection_matches() {
        step2_selection_test(
            ee::s03::f_001bb2d0__generated,
            c_fe_state_create_step2_set_month_selection,
            0x001b_b2d0,
            0x28,
        );
    }


    #[test]
    fn c_fe_state_create_step2_set_day_selection_matches() {
        step2_selection_test(
            ee::s03::f_001bb340__generated,
            c_fe_state_create_step2_set_day_selection,
            0x001b_b340,
            0x2c,
        );
    }


    #[test]
    fn c_fe_state_create_step2_set_gender_selection_matches() {
        step2_selection_test(
            ee::s03::f_001bb440__generated,
            c_fe_state_create_step2_set_gender_selection,
            0x001b_b440,
            0x34,
        );
    }


    pub(crate) fn step2_selection_test(gen: GuestFn, hand: GuestFn, salt: u64, value_off: u32) {
        check(gen, hand, 8, &|c, seed| {
            let mut r = base_3(c, seed, salt, 4);
            c.g[A0][0] = OBJ_2 as u64;
            let screen = DATA + 0x3800;
            let list = DATA + 0x3a00;
            w(c, OBJ_2 + 0x40, screen);
            w(c, OBJ_2 + 0x6d0, list);
            let val = 0x11 + (r.next_u32() % 0x40);
            if seed % 2 == 0 {
                let node = DATA + 0x3c00;
                hash_object(c, screen, node);
                w(c, node + 0x318, 1);
                w(c, node + 0xa4, val);
                w(c, list + value_off, val);
            } else {
                w(c, screen + 0x3c, 0);
            }
        });
    }


    pub(crate) fn presence_test(gen: GuestFn, hand: GuestFn, salt: u64) {
        check(gen, hand, 8, &|c, seed| {
            let _ = base_3(c, seed, salt, 6);
            if seed % 2 == 0 {
                w(c, GAME_GP + 0x238, 0);
            } else {
                let bm = DATA + 0x3000;
                w(c, GAME_GP + 0x238, bm);
                w(c, bm + 0xc8, 1);
                w(c, bm + 0xd8, 0);
                w(c, bm + 0xe0, 1);
            }
        });
    }


    #[test]
    fn c_fe_state_lobby_menu_v32_matches() {
        presence_test(ee::s03::f_001bcca8__generated, c_fe_state_lobby_menu_v32, 0x001b_cca8);
    }


    #[test]
    fn c_fe_state_stats_v32_matches() {
        presence_test(ee::s03::f_001c2e70__generated, c_fe_state_stats_v32, 0x001c_2e70);
    }


    #[test]
    fn c_fe_state_net_news_v32_matches() {
        presence_test(ee::s03::f_001c5108__generated, c_fe_state_net_news_v32, 0x001c_5108);
    }


    #[test]
    fn c_fe_state_login_init_matches() {
        check(ee::s03::f_001beed0__generated, c_fe_state_login_init, 4, &|c, seed| {
            ui_init_setup(c, seed, 0x001b_eed0);
        });
    }


    #[test]
    fn c_fe_state_stats_init_matches() {
        check(ee::s03::f_001c2e00__generated, c_fe_state_stats_init, 4, &|c, seed| {
            ui_init_setup(c, seed, 0x001c_2e00);
        });
    }

    /// The test world / race manager / BE_2 singleton live high in `DATA`.
    pub(crate) const W: u32 = DATA + 0x3d_0000;

    pub(crate) const RM: u32 = DATA + 0x3d_0100;

    pub(crate) const BE_2: u32 = DATA + 0x3d_2000;


    pub(crate) fn run_4(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, SENTINEL_RA, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }


    pub(crate) fn heap_init_4(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        run_4(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
    }


    /// The engine's size-class pool allocator, enabled through `gp + 0xdd8`.
    pub(crate) fn pool_allocator_7(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),
            (0x2a60, 0x2000, 0x10),
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base_4 = nodes;
            for n in 0..16u32 {
                let node = base_4 + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, base_4);
            c.bus.write32(GAME_GP + slot, pool);
            nodes += block * 16;
        }
    }


    /// A machine with a plausible game `gp`: the BE_2 singleton, the world,
    /// race manager and frame counter.
    pub(crate) fn base_4(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP - 0x1f04, BE_2);
        c.bus.write32(GAME_GP - 0x1efc, 0);
        c.bus.write32(GAME_GP - 0x848, W);
        c.bus.write32(W + 0x84, RM);
        c.bus.write32(RM + 0xc, DATA + 0x3d_0200);
        c.bus.write32(DATA + 0x3d_0200 + 8, r.next_u32());
        c.bus.write32(RM + 0x7c, 0);
        c.bus.write32(RM + 0x64, 0);
        c.bus.write32(RM + 0x84, 0);
        r
    }


    /// [`base_4`] plus a working pool allocator and dlmalloc arena.
    pub(crate) fn alloc_base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = base_4(c, seed, salt);
        pool_allocator_7(c);
        heap_init_4(c);
        r
    }


    #[test]
    fn net_fe_state_on_messenger_popup_event_matches() {
        check(
            ee::s03::f_001a9268__generated,
            net_fe_state_on_messenger_popup_event,
            30,
            &|c, seed| {
                let mut r = alloc_base(c, seed, 0x1a92_68);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0xf
                } else {
                    (seed % 0xf) as u64
                };
                c.bus.write32(DATA + 0x10, 0);
            },
        );
    }


    #[test]
    fn net_fe_state_show_wait_player_popup_matches() {
        check(
            ee::s03::f_001ab108__generated,
            net_fe_state_show_wait_player_popup,
            20,
            &|c, seed| {
                alloc_base(c, seed, 0x1ab1_08);
                c.g[A0][0] = sx(DATA);
            },
        );
    }


    #[test]
    fn c_fe_state_challenge_popup_v32_matches() {
        check(
            ee::s03::f_001aebf0__generated,
            c_fe_state_challenge_popup_v32,
            20,
            &|c, seed| {
                alloc_base(c, seed, 0x1aeb_f0);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(GAME_GP + 0x238, 0);
                c.bus.write32(GAME_GP - 0xc8, 0);
            },
        );
    }


    #[test]
    fn c_fe_state_challenge_received_popup_transition_in_matches() {
        check(
            ee::s03::f_001b0670__generated,
            c_fe_state_challenge_received_popup_transition_in,
            30,
            &|c, seed| {
                machine(c, seed, 0x1b06_70, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 0;
                c.bus.write32(DATA + 8, 0);
            },
        );
    }


    #[test]
    fn c_fe_state_create_step1_ctor_matches() {
        check(
            ee::s03::f_001b88f0__generated,
            c_fe_state_create_step1_ctor,
            24,
            &|c, seed| {
                let mut r = alloc_base(c, seed, 0x1b88_f0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = (seed % 2) as u64;
            },
        );
    }


    /// Set up a step-1 list-box lookup for `name_addr` and the boolean field.
    pub(crate) fn selection_setup(c: &mut Cpu, name_addr: u32, field: u32) {
        let screen = DATA + 0x1000;
        let cnt = DATA + 0x2000;
        let arr = DATA + 0x2100;
        let item = DATA + 0x2200;
        let flags = DATA + 0x2300;
        c.g[A0][0] = sx(DATA);
        c.bus.write32(DATA + 0x40, screen);
        c.bus.write32(DATA + 0x6d0, flags);
        c.bus.write32(flags + field, 1);
        c.bus.write32(screen + 0x38, cnt);
        c.bus.write32(screen + 0x3c, arr);
        c.bus.write32(cnt, 1);
        c.bus.write32(arr, item);
        let hash = run_4(c, 0x0031_7670, ee::s10::f_00317670, &[name_addr as u64]) as u32;
        c.bus.write32(item + 0x38, hash);
        c.bus.write8(item + 0x318, 3);
        for i in 0..3u32 {
            c.bus.write32(item + 0xa4 + i * 0x14, i);
        }
    }


    #[test]
    fn c_fe_state_create_step1_set_inform_selection_matches() {
        check(
            ee::s03::f_001b9740__generated,
            c_fe_state_create_step1_set_inform_selection,
            40,
            &|c, seed| {
                base_4(c, seed, 0x1b97_40);
                if seed % 2 == 0 {
                    c.g[A0][0] = sx(DATA);
                    c.bus.write32(DATA + 0x40, 0);
                } else {
                    selection_setup(c, 0x0046_4780, 0x38);
                }
            },
        );
    }


    #[test]
    fn c_fe_state_create_step1_set_share_info_selection_matches() {
        check(
            ee::s03::f_001b97b0__generated,
            c_fe_state_create_step1_set_share_info_selection,
            40,
            &|c, seed| {
                base_4(c, seed, 0x1b97_b0);
                if seed % 2 == 0 {
                    c.g[A0][0] = sx(DATA);
                    c.bus.write32(DATA + 0x40, 0);
                } else {
                    selection_setup(c, 0x0046_4790, 0x3c);
                }
            },
        );
    }


    #[test]
    fn c_fe_state_create_step2_set_legal_selection_matches() {
        check(
            ee::s03::f_001bb4b0__generated,
            c_fe_state_create_step2_set_legal_selection,
            40,
            &|c, seed| {
                base_4(c, seed, 0x1bb4_b0);
                if seed % 2 == 0 {
                    c.g[A0][0] = sx(DATA);
                    c.bus.write32(DATA + 0x40, 0);
                } else {
                    selection_setup(c, 0x0046_4a08, 0x40);
                }
            },
        );
    }


    #[test]
    fn c_fe_state_create_duplicate_submit_account_matches() {
        check(
            ee::s03::f_001bc290__generated,
            c_fe_state_create_duplicate_submit_account,
            16,
            &|c, seed| {
                alloc_base(c, seed, 0x1bc2_90);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(GAME_GP - 0xc8, 0);
            },
        );
    }


    #[test]
    fn c_fe_state_lobby_menu_clamp_room_selections_matches() {
        check(
            ee::s03::f_001be038__generated,
            c_fe_state_lobby_menu_clamp_room_selections,
            50,
            &|c, seed| {
                base_4(c, seed, 0x1be0_38);
                c.g[A0][0] = sx(DATA);
                for i in 0..4u32 {
                    let row = DATA + 0x1000 + i * 0x100;
                    c.bus.write32(DATA + 0x708 + i * 4, row);
                    let count = (seed + i) % 4;
                    c.bus.write8(row + 0x318, count as u8);
                    c.bus.write32(row + 0x48, count + 2);
                }
            },
        );
    }


    #[test]
    fn c_fe_state_net_news_init_matches() {
        check(
            ee::s03::f_001c5098__generated,
            c_fe_state_net_news_init,
            20,
            &|c, seed| {
                alloc_base(c, seed, 0x1c50_98);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(GAME_GP - 0xc8, 0);
            },
        );
    }


    // ---------------------------------------------------------- front-end states

    #[test]
    fn c_fe_state_net_messenger_init_matches() {
        check(ee::s03::f_001a3f68__generated, c_fe_state_net_messenger_init, 12, &|c, seed| {
            machine(c, seed, 0x1a_3f68, 5);
            belib_2(c);
            heap_setup(c);
            world_2(c);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            w(c, DATA + 0x2000 + 0x10, DATA + 0x2200);
            c.pc = 0x001a_3f68;
        });
    }


    #[test]
    fn c_fe_state_net_messenger_v33_matches() {
        check(ee::s03::f_001a4928__generated, c_fe_state_net_messenger_v33, SEEDS_7, &|c, seed| {
            machine(c, seed, 0x1a_4928, 4);
            belib_2(c);
            world_2(c);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            w(c, DATA + 0x2000 + 0x6d0, DATA + 0x2400);
            string(c, DATA + 0x2400, "");
            if seed % 2 == 0 {
                c.g[A2][0] = 0xf;
            } else {
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                w(c, DATA + 0x2000 + 0x69c, DATA + 0x2500);
            }
            c.pc = 0x001a_4928;
        });
    }


    #[test]
    fn c_fe_state_chat_room_on_go_to_char_select_event_matches() {
        check(
            ee::s03::f_001b3760__generated,
            c_fe_state_chat_room_on_go_to_char_select_event,
            12,
            &|c, seed| {
                machine(c, seed, 0x1b_3760, 5);
                belib_2(c);
                heap_setup(c);
                world_2(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                if seed % 2 == 0 {
                    c.g[A2][0] = 0xf;
                    w(c, DATA + 0x2000 + 0x10, DATA + 0x2200);
                } else {
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0;
                    w(c, DATA + 0x2000 + 0x69c, DATA + 0x2500);
                }
                c.pc = 0x001b_3760;
            },
        );
    }


    #[test]
    fn c_fe_state_chat_room_on_go_to_char_select_event_2_matches() {
        check(
            ee::s03::f_001b37d8__generated,
            c_fe_state_chat_room_on_go_to_char_select_event_2,
            12,
            &|c, seed| {
                machine(c, seed, 0x1b_37d8, 5);
                belib_2(c);
                heap_setup(c);
                world_2(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                if seed % 2 == 0 {
                    c.g[A2][0] = 0xf;
                    w(c, DATA + 0x2000 + 0x10, DATA + 0x2200);
                } else {
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0;
                    w(c, DATA + 0x2000 + 0x69c, DATA + 0x2500);
                }
                c.pc = 0x001b_37d8;
            },
        );
    }


    #[test]
    fn c_fe_state_online_main_menu_v31_matches() {
        check(ee::s03::f_001c0358__generated, c_fe_state_online_main_menu_v31, SEEDS_7, &|c, seed| {
            machine(c, seed, 0x1c_0358, 5);
            belib_2(c);
            heap_setup(c);
            world_2(c);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = [0xefu64, 0xf3, 0x42][(seed % 3) as usize];
            c.g[A2][0] = 0;
            w(c, DATA + 0x2000 + 0x10, DATA + 0x2200);
            c.pc = 0x001c_0358;
        });
    }


    pub(crate) const SEEDS_8: u32 = 64;

    pub(crate) fn h(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }

    pub(crate) fn b(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }


    /// The zeroed BE-library singleton at `gp-0x1f04` (else the FE helpers walk
    /// into the lazy disc load).
    pub(crate) fn be_library_5(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
    }


    /// The dummy world_3 at `gp-0x848` with the race-manager chain.
    pub(crate) fn world_3(c: &mut Cpu) {
        let wld = DATA + 0x1000;
        w(c, GAME_GP.wrapping_sub(0x848), wld);
        w(c, wld + 0x84, DATA + 0x1200);
        w(c, DATA + 0x1200 + 0x20, DATA + 0x3000);
        w(c, DATA + 0x1200 + 0x28, DATA + 0x1400);
        w(c, DATA + 0x1200 + 0x78, 0);
        w(c, wld + 0x7c, DATA + 0x1500);
    }


    /// `DLHeap_InitArena(&MEM_HEAP, arena, 0x40000)`: the game heap over test
    /// memory, for the paths that allocate.
    pub(crate) fn heap_init_5(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = TEST_RA as u64;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, TEST_RA as u32, "DLHeap_InitArena did not return");
    }


    /// A vtable slot_3 `{delta, fn}` at `vt + off`.
    pub(crate) fn slot_3(c: &mut Cpu, vt: u32, off: u32, delta: i16, f: u32) {
        h(c, vt + off, delta as u16);
        w(c, vt + off + 4, f);
    }


    /// A NUL-terminated string_2 at `a`.
    pub(crate) fn string_2(c: &mut Cpu, a: u32, s: &str) {
        for (i, ch) in s.bytes().enumerate() {
            b(c, a + i as u32, ch);
        }
        b(c, a + s.len() as u32, 0);
    }


    #[test]
    fn net_fe_state_make_password_mask_matches() {
        check(
            ee::s03::f_001a98b8__generated,
            net_fe_state_make_password_mask,
            SEEDS_8,
            &|c, seed| {
                machine(c, seed, 0x1a98b8, 2);
                be_library_5(c);
                world_3(c);
                let this = DATA;
                let s = DATA + 0x400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = s as u64;
                let text = ["", "abc", "abcdefgh", "0123456789abcdef"][(seed % 4) as usize];
                string_2(c, s, text);
            },
        );
    }


    #[test]
    fn c_fe_state_challenge_received_popup_v32_matches() {
        check(
            ee::s03::f_001b00b8__generated,
            c_fe_state_challenge_received_popup_v32,
            SEEDS_8,
            &|c, seed| {
                machine(c, seed, 0x1b00b8, 6);
                be_library_5(c);
                world_3(c);
                heap_init_5(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                w(c, this + 0x6d0, DATA + 0x400);
                w(
                    c,
                    GAME_GP.wrapping_add(0x238),
                    if seed % 2 == 0 { 0 } else { DATA + 0x500 },
                );
                w(c, GAME_GP.wrapping_sub(0xc8), DATA + 0x600);
            },
        );
    }


    #[test]
    fn c_fe_state_lobby_menu_init_matches() {
        check(
            ee::s03::f_001bcc30__generated,
            c_fe_state_lobby_menu_init,
            16,
            &|c, seed| {
                machine(c, seed, 0x1bcc30, 8);
                be_library_5(c);
                world_3(c);
                heap_init_5(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = this as u64;
                w(c, this + 0x10, DATA + 0x4000);
            },
        );
    }


    #[test]
    fn c_fe_state_lobby_menu_show_room_tab_matches() {
        check(
            ee::s03::f_001bdf60__generated,
            c_fe_state_lobby_menu_show_room_tab,
            SEEDS_8,
            &|c, seed| {
                machine(c, seed, 0x1bdf60, 4);
                be_library_5(c);
                world_3(c);
                heap_init_5(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let tab = DATA + 0x400;
                c.g[A1][0] = if seed % 2 == 0 {
                    tab as u64
                } else {
                    0x1234_5678
                };
                for i in 0..4u32 {
                    w(
                        c,
                        this + 0x718 + i * 4,
                        if i as u32 == seed % 4 { tab } else { 0 },
                    );
                }
                w(c, tab + 8, DATA + 0x2000);
                slot_3(c, DATA + 0x2000, 0x48, 0, FAKE);
            },
        );
    }


    #[test]
    fn c_fe_state_net_quick_play_popup_init_matches() {
        check(
            ee::s03::f_001c0fc0__generated,
            c_fe_state_net_quick_play_popup_init,
            16,
            &|c, seed| {
                machine(c, seed, 0x1c0fc0, 8);
                be_library_5(c);
                world_3(c);
                heap_init_5(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = this as u64;
                w(c, this + 0x10, DATA + 0x4000);
            },
        );
    }


    #[test]
    fn c_fe_state_stats_ctor_matches() {
        check(
            ee::s03::f_001c2d18__generated,
            c_fe_state_stats_ctor,
            16,
            &|c, seed| {
                machine(c, seed, 0x1c2d18, 6);
                be_library_5(c);
                world_3(c);
                heap_init_5(c);
                let this = DATA + 0x4000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
            },
        );
    }


    #[test]
    fn fe_async_manager_request_texture_matches() {
        check(ee::s03::f_001a36e8__generated, fe_async_manager_request_texture, SEEDS_9, &|c, seed| {
            let mut r = machine(c, seed, 0x1a36e8, 2);
            be_library_6(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = r.next_u32() as u64;
            w(c, GAME_GP.wrapping_sub(0x1700), DATA + 0x2000);
            string_3(c, DATA + 0x2000, "pic");
        });
    }


    // ------------------------------------------------------------------ front end

    #[test]
    fn fe_set_online_mode_matches() {
        check(ee::s03::f_001a86e8__generated, fe_set_online_mode, 24, &|c, seed| {
            let _ = base_5(c, seed, 0x1a86_e8, 6);
            c.g[A0][0] = if seed % 3 == 0 {
                0
            } else {
                (1 + seed % 2) as u64
            };
            match seed % 4 {
                0 => w(c, GAME_GP.wrapping_add(0x238), 0),
                _ => {
                    let bm = DATA + 0x1000;
                    w(c, GAME_GP.wrapping_add(0x238), bm);
                    w(c, bm + 0xc8, (seed % 3) as u32);
                    w(c, bm + 0xd8, ((seed / 3) % 2) as u32);
                    w(c, bm + 0xe0, ((seed / 5) % 2) as u32);
                }
            }
            w(c, GAME_GP.wrapping_sub(0xc8), DATA + 0x2000);
        });
    }


    #[test]
    fn net_fe_show_player_conn_quality_matches() {
        check(
            ee::s03::f_001afd88__generated,
            net_fe_show_player_conn_quality,
            16,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x1afd_88, 6);
                let screen = DATA;
                let player = DATA + 0x1000;
                w(c, player + 0xc, r.next_u32() % 8);
                if seed % 2 == 0 {
                    w(c, screen + 0x3c, 0);
                    w(c, screen + 0x38, 0);
                } else {
                    let node = DATA + 0x2000;
                    w(c, screen + 0x3c, 1);
                    w(c, screen + 0x38, DATA + 0x1800);
                    w(c, DATA + 0x1800, node);
                    w(c, node + 0x38, 0);
                }
                c.g[A0][0] = screen as u64;
                c.g[A1][0] = player as u64;
            },
        );
    }


    #[test]
    fn c_fe_state_lobby_menu_on_net_result_matches() {
        check(
            ee::s03::f_001bd848__generated,
            c_fe_state_lobby_menu_on_net_result,
            24,
            &|c, seed| {
                let _ = base_5(c, seed, 0x1bd8_48, 8);
                let msg = DATA + 0x1800;
                w(c, DATA + 0x1c, if seed % 2 == 0 { 0 } else { 6 });
                let result = match seed % 4 {
                    0 => 0x828,
                    1 => 0x829,
                    2 => 0x700,
                    _ => [0x828, 0x829, 0x700][(seed as usize / 4) % 3],
                };
                let obj = DATA + 0x1000;
                w(c, DATA + 0x6a0, obj);
                w(c, obj + 0x18, result);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (0xdc + seed % 0x3b) as u64;
                c.g[A2][0] = msg as u64;
            },
        );
    }


    #[test]
    fn c_fe_state_net_quick_play_popup_ctor_matches() {
        check(
            ee::s03::f_001c0f40__generated,
            c_fe_state_net_quick_play_popup_ctor,
            12,
            &|c, seed| {
                let _ = alloc_base_2(c, seed, 0x1c0f_40, 2);
                c.g[A0][0] = DATA as u64;
            },
        );
    }


    #[test]
    fn c_fe_state_net_ssx_world_init_matches() {
        check(
            ee::s03::f_001c1c90__generated,
            c_fe_state_net_ssx_world_init,
            12,
            &|c, seed| {
                let _ = alloc_base_2(c, seed, 0x1c1c_90, 4);
                let engine = DATA + 0x1000;
                w(c, DATA + 0x10, engine);
                w(c, engine, DATA + 0x1100);
                w(c, engine + 4, 0);
                w(c, DATA + 0x1100, 0);
                let lobby = DATA + 0x2000;
                w(c, GAME_GP.wrapping_sub(0xc8), lobby);
                w(c, lobby, (seed % 2) as u32);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    /// The dummy world (`world -> +0x84 -> game -> +0xc -> race`).
    pub(crate) const WORLD_3: u32 = DATA + 0x8_0000;


    pub(crate) fn wh(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }


    /// A vtable slot_4 `{delta = 0, fn}`.
    pub(crate) fn slot_4(c: &mut Cpu, vt: u32, off: u32, func: u32) {
        wh(c, vt + off, 0);
        w(c, vt + off + 4, func);
    }


    /// The BELibrary singleton, the world/race globals_6 and the `gp` slots the
    /// front-end code keeps singletons in.
    pub(crate) fn globals_6(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x848), WORLD_3);
        let game = WORLD_3 + 0x1000;
        w(c, WORLD_3 + 0x84, game);
        let race = WORLD_3 + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }


    /// The engine's size-class pool_3 allocator (`gp + 0xdd8` ready plus eight
    /// free nodes per size class; a zeroed heap walks garbage).
    pub(crate) fn pool_3(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot_off, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let region = DATA + 0x40_0000 + 0x4_0000 * i;
            let base_6 = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base_6 + 0x400 * n;
                w(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w(c, region + head, base_6);
            w(c, GAME_GP + slot_off, region);
        }
    }


    /// Scrambled state, the fake call hooks, a game-like `gp`, the standard
    /// fixtures and a prepared UIEngine phase table (slot_4 0/1/2/3 -> fake
    /// objects whose `+4` is a vtable that calls the fake).
    pub(crate) fn base_6(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        globals_6(c);
        pool_3(c);
        for i in 0..4u32 {
            let obj = DATA + 0x10_000 + 0x40 * i;
            let vt = DATA + 0x11_000 + 0x40 * i;
            w(c, 0x0053_08a8 + 4 * i, obj);
            w(c, obj + 4, vt);
            slot_4(c, vt, 0x18, FAKE);
            slot_4(c, vt, 0x20, FAKE);
            slot_4(c, vt, 0x08, FAKE);
        }
        r
    }


    #[test]
    fn net_fe_state_toggle_block_user_matches() {
        check(
            ee::s03::f_001abc50__generated,
            net_fe_state_toggle_block_user,
            60,
            &|c, seed| {
                let _ = base_6(c, seed, 0x1abc, 4);
                let s = DATA;
                let buddy = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
                w(c, GAME_GP.wrapping_add(0x238), buddy);
                w(c, GAME_GP.wrapping_sub(0xc8), DATA + 0x500);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(DATA + 0x600);
                c.g[A2][0] = sx(DATA + 0x700);
            },
        );
    }


    #[test]
    fn net_fe_state_toggle_messenger_buddy_matches() {
        check(
            ee::s03::f_001abe58__generated,
            net_fe_state_toggle_messenger_buddy,
            60,
            &|c, seed| {
                let _ = base_6(c, seed, 0x1abe, 4);
                let s = DATA;
                let buddy = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
                w(c, GAME_GP.wrapping_add(0x238), buddy);
                w(c, GAME_GP.wrapping_sub(0xc8), DATA + 0x500);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(DATA + 0x600);
                c.g[A2][0] = sx(DATA + 0x700);
            },
        );
    }


    #[test]
    fn c_fe_state_chat_room_v32_matches() {
        check(
            ee::s03::f_001b22a0__generated,
            c_fe_state_chat_room_v32,
            60,
            &|c, seed| {
                let _ = base_6(c, seed, 0x1b22, 4);
                let s = DATA;
                let buddy = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
                w(c, GAME_GP.wrapping_add(0x238), buddy);
                w(c, GAME_GP.wrapping_sub(0xc8), DATA + 0x500);
                c.g[A0][0] = sx(s);
            },
        );
    }


    #[test]
    fn c_fe_state_chat_room_on_child_event_matches() {
        check(
            ee::s03::f_001b3850__generated,
            c_fe_state_chat_room_on_child_event,
            100,
            &|c, seed| {
                let _ = base_6(c, seed, 0x1b38, 2);
                let id = match seed % 5 {
                    0 => 0x800,
                    1 => 0x801,
                    2 => 0x80b,
                    3 => 0x7ff,
                    _ => 0x810,
                };
                w(c, DATA + 0x200 + 0x18, id);
                // The handlers see the caller's `a0` as their state.
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }


    #[test]
    fn net_config_auth_popup_dtor_matches() {
        check(
            ee::s03::f_001b6368__generated,
            net_config_auth_popup_dtor,
            60,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x1b63, 3);
                let p = DATA;
                let auth = if seed % 3 == 0 { 0 } else { DATA + 0x800 };
                w(c, p + 0x6d0, auth);
                w(c, p + 0x6fc, if seed % 2 == 0 { 0 } else { DATA + 0x900 });
                c.g[A0][0] = sx(p);
                c.g[A1][0] = sx(DATA + 0x400);
            },
        );
    }


    #[test]
    fn c_fe_state_create_duplicate_init_matches() {
        check(
            ee::s03::f_001bbd60__generated,
            c_fe_state_create_duplicate_init,
            40,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x1bbd, 3);
                let s = DATA;
                w(c, s + 0x10, if seed % 4 == 0 { 0 } else { DATA + 0x1000 });
                w(c, GAME_GP.wrapping_sub(0xc8), DATA + 0x1200);
                c.g[A0][0] = sx(s);
            },
        );
    }


    // -----------------------------------------------------------------------
    // Net/online front-end states.

    #[test]
    fn cfestatenetmessenger_clear_name_list_matches() {
        for (gen, hand) in [(
            ee::s03::f_001a5450__generated as fn(&mut Cpu),
            cfestatenetmessenger_clear_name_list as fn(&mut Cpu),
        )] {
            check(gen, hand, SEEDS_10, &|c, seed| {
                base_7(c, seed, 0xa5450, 0);
                let s = DATA + 0x100;
                c.g[A0][0] = sx(s);
                c.pc = 0x001a_5450;
                let lo = s + 0x600;
                let (head, tail) = match seed % 4 {
                    0 => (lo, lo),
                    1 => (lo, lo + 8),
                    2 => (lo, lo + 0xc),
                    _ => (lo, lo + 0x10),
                };
                c.bus.write32(s + 0x6e8, head);
                c.bus.write32(s + 0x6ec, tail);
                for k in 0..8u32 {
                    let at = (tail as i64 - 4 * (k as i64 + 1)) as u32;
                    c.bus.write32(at, 0);
                }
            });
        }
    }


    #[test]
    fn cfestateonlinecharacterselect_show_character_matches() {
        check(
            ee::s03::f_001b1b10__generated,
            cfestateonlinecharacterselect_show_character,
            24,
            &|c, seed| {
                base_7(c, seed, 0xb1b10, 0);
                let s = DATA + 0x100;
                c.g[A0][0] = sx(s);
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = ((seed / 3) % 2) as u64;
            },
        );
    }


    #[test]
    fn cfestatelobbymenu_prompt_room_password_matches() {
        check(
            ee::s03::f_001bdd60__generated,
            cfestatelobbymenu_prompt_room_password,
            SEEDS_10,
            &|c, seed| {
                base_7(c, seed, 0xbdd60, 0);
                let s = DATA + 0x100;
                c.g[A0][0] = sx(s);
                c.g[A2][0] = if seed % 2 == 0 {
                    0xf
                } else {
                    (seed % 4) as u64
                };
                let popup = DATA + 0x200;
                c.bus.write32(s + 0x69c, popup);
                c.bus.write32(0x004a_1408, 0x1234);
                c.bus.write32(0x004a_50d0, 0x5678);
            },
        );
    }


    pub(crate) const SEEDS_11: u32 = 40;


    /// A vtable slot_5 `{delta, fn}` at `vt + off`.
    pub(crate) fn slot_5(c: &mut Cpu, vt: u32, off: u32, delta: i16, f: u32) {
        h(c, vt + off, delta as u16);
        w(c, vt + off + 4, f);
    }


    /// The zeroed BE-library singleton at `gp_11-0x1f04` (else the FE helpers walk
    /// into the lazy disc load).
    pub(crate) fn be_library_7(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
    }


    /// The dummy world_4 at `gp_11-0x848` with the race-manager chain.
    pub(crate) fn world_4(c: &mut Cpu) {
        let wld = DATA + 0x1000;
        w(c, GAME_GP.wrapping_sub(0x848), wld);
        w(c, wld + 0x84, DATA + 0x1200);
        w(c, DATA + 0x1200 + 0x20, DATA + 0x3000);
        w(c, DATA + 0x1200 + 0x28, DATA + 0x1400);
        w(c, DATA + 0x1200 + 0x78, 0);
        w(c, wld + 0x7c, DATA + 0x1500);
    }


    /// A NUL-terminated string_4 at `a`.
    pub(crate) fn string_4(c: &mut Cpu, a: u32, s: &str) {
        for (i, ch) in s.bytes().enumerate() {
            b(c, a + i as u32, ch);
        }
        b(c, a + s.len() as u32, 0);
    }


    #[test]
    fn fe_async_manager_free_all_matches() {
        check(ssx3_ee::s03::f_001a3c50__generated, fe_async_manager_free_all, SEEDS_11, &|c, seed| {
            let mut r = machine(c, seed, 0x1a3c50, 4);
            be_library_7(c);
            world_4(c);
            let mgr = DATA + 0x100;
            c.g[A0][0] = mgr as u64;
            // gp_11-0x854: the object whose vtable slot_5 50 releases a file.
            let obj = DATA + 0x200;
            let vt = DATA + 0x600;
            w(c, GAME_GP.wrapping_sub(0x854), obj);
            w(c, obj + 0x10d8, vt);
            slot_5(c, vt, 0x190, 0, FAKE);
            slot_5(c, vt, 0x194, 0, FAKE + 0x10);
            for i in 0..5 {
                let file = mgr + 0x100 + 0x11c * i;
                if seed % 3 != 0 || i == 0 {
                    w(c, file, r.next_u32() & 0xffff);
                    w(c, file + 8, r.next_u32());
                    w(c, file - 0x10, DATA + 0x4000 + 0x40 * i);
                }
            }
        });
    }


    // ------------------------------------------------------------------
    // Front-end network states.

    #[test]
    fn net_config_auth_popup_handle_input_matches() {
        check(
            ssx3_ee::s03::f_001b66b8__generated,
            net_config_auth_popup_handle_input,
            SEEDS_11,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1b66b8, 4);
                be_library_7(c);
                world_4(c);
                let popup = DATA + 0x100;
                let obj = DATA + 0x200;
                c.g[A0][0] = popup as u64;
                c.g[A1][0] = obj as u64;
                w(c, popup + 0x6e4, if seed % 3 == 0 { 2 } else { 1 });
                w(c, popup + 0x6d0, DATA + 0x300);
                let vt = DATA + 0x500;
                w(c, obj + 8, vt);
                slot_5(c, vt, 0x28, 0, FAKE);
                slot_5(c, vt, 0x2c, 0, FAKE + 0x10);
                slot_5(c, vt, 0x30, 0, FAKE + 0x20);
                slot_5(c, vt, 0x34, 0, FAKE + 0x30);
            },
        );
    }


    #[test]
    fn c_fe_state_create_step1_init_matches() {
        check(ssx3_ee::s03::f_001b8960__generated, c_fe_state_create_step1_init, SEEDS_11, &|c, seed| {
            let mut r = machine(c, seed, 0x1b8960, 4);
            be_library_7(c);
            world_4(c);
            let st = DATA + 0x100;
            c.g[A0][0] = st as u64;
            w(c, st + 0x6ec, if seed % 2 == 0 { 0 } else { 1 });
            w(c, st + 0x10, 0);
            w(c, GAME_GP.wrapping_sub(0xc8), DATA + 0x200);
            let _ = r.next_u32();
        });
    }


    #[test]
    fn c_fe_state_create_step1_detect_changes_matches() {
        check(
            ssx3_ee::s03::f_001ba358__generated,
            c_fe_state_create_step1_detect_changes,
            SEEDS_11,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1ba358, 4);
                be_library_7(c);
                world_4(c);
                let st = DATA + 0x100;
                let dev = DATA + 0x200;
                c.g[A0][0] = st as u64;
                let s1 = DATA + 0x300;
                let s2 = DATA + 0x380;
                string_4(c, s1, "abc");
                string_4(c, s2, if seed % 3 == 0 { "abc" } else { "xyz" });
                w(c, st + 0x6d0, dev);
                w(c, st + 0x6d4, s1);
                w(c, st + 0x6dc, s1);
                w(c, dev + 4, if seed % 2 == 0 { s1 } else { s2 });
                w(c, dev + 0xc, if seed % 3 == 0 { s1 } else { s2 });
                w(c, dev + 0x38, r.next_u32());
                w(c, dev + 0x3c, r.next_u32());
                w(c, st + 0x6e0, r.next_u32());
                w(c, st + 0x6e4, r.next_u32());
            },
        );
    }


    #[test]
    fn c_fe_state_create_duplicate_on_child_event_matches() {
        check(
            ssx3_ee::s03::f_001bc6a0__generated,
            c_fe_state_create_duplicate_on_child_event,
            SEEDS_11,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1bc6a0, 2);
                be_library_7(c);
                world_4(c);
                let st = DATA + 0x100;
                let ev = DATA + 0x200;
                c.g[A0][0] = st as u64;
                c.g[A1][0] = ev as u64;
                c.g[A2][0] = r.next_u32() as u64;
                let word = match seed % 5 {
                    0 => 0x81f,
                    1 => 0x80e,
                    2 => 0x82c,
                    3 => 0x800,
                    _ => 0x900,
                };
                w(c, ev + 0x18, word);
            },
        );
    }


        #[test]
    fn c_fe_state_online_agreement_update_matches() {
        check(
            ssx3_ee::s03::f_001c0a80__generated,
            c_fe_state_online_agreement_update,
            SEEDS_11,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1c0a80, 6);
                be_library_7(c);
                world_4(c);
                let st = DATA + 0x100;
                c.g[A0][0] = st as u64;
                w(c, st + 0x6d8, (r.next_u32() % 4) as u32);
                w(c, st + 0x6d4, (r.next_u32() % 4) as u32);
                w(c, st + 0x6d0, if seed % 3 == 0 { 0 } else { DATA + 0x200 });
                w(c, st + 0x40, DATA + 0x300);
                if seed % 4 != 0 {
                    let objs = DATA + 0x400;
                    let obj = DATA + 0x500;
                    w(c, DATA + 0x300 + 0x38, objs);
                    w(c, DATA + 0x300 + 0x3c, 1);
                    w(c, objs, obj);
                    w(c, obj + 0x38, 0);
                }
            },
        );
    }


    /// Call a guest function with `args` in a0.. and return `v0`.
    pub(crate) fn run_6(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = RET as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, RET, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }


    /// The ELF string hash of the string at_3 `s`.
    pub(crate) fn str_hash(c: &mut Cpu, s: u32) -> u32 {
        run_6(c, 0x0031_7670, ee::s10::f_00317670, &[s as u64]) as u32
    }


    /// A NUL-terminated byte string at_3 `addr`.
    pub(crate) fn put_str_3(c: &mut Cpu, addr: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(addr + i as u32, *b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }


    /// A UIScreen at_3 `screen` whose only child (`+0x38` -> count cell, `+0x3c`
    /// -> array) has id `id`; the child is a UI text object whose `+0x8` slot
    /// `+0x28` calls the fake. Returns the child.
    pub(crate) fn screen_with(c: &mut Cpu, r: &mut Rng, screen: u32, id: u32) -> u32 {
        let node = screen + 0x100;
        let vt = vtable(c, r, &[0x28]);
        c.bus.write32(screen + 0x38, screen + 0x10);
        c.bus.write32(screen + 0x3c, screen + 0x20);
        c.bus.write32(screen + 0x10, 1);
        c.bus.write32(screen + 0x20, node);
        c.bus.write32(node + 0x8, vt);
        c.bus.write32(node + 0x38, id);
        node
    }


    #[test]
    fn fe_async_manager_request_peak_map_matches() {
        check(
            ee::s03::f_001a3768__generated,
            fe_async_manager_request_peak_map,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x1a3768, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 0x40) as u64;
                c.g[A2][0] = (seed % 0x100) as u64;
                c.g[A3][0] = (seed % 4) as u64;
            },
        );
    }


    #[test]
    fn c_fe_state_net_messenger_set_row_name_matches() {
        check(
            ee::s03::f_001a6618__generated,
            c_fe_state_net_messenger_set_row_name,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1a6618, 3);
                put_str_3(c, DATA + 0x400, b"row");
                let h = str_hash(c, DATA + 0x400);
                let screen = DATA + 0x1000;
                screen_with(c, &mut r, screen, h);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 0x12) as u64;
                c.g[A2][0] = sx(DATA + 0x400);
                c.g[A3][0] = (seed % 0x100) as u64;
                c.bus
                    .write32(DATA + 0x40, if seed % 4 == 0 { 0 } else { screen });
            },
        );
    }


    #[test]
    fn c_fe_state_create_step1_is_email_format_valid_matches() {
        check(
            ee::s03::f_001b9cd8__generated,
            c_fe_state_create_step1_is_email_format_valid,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x1b9cd8, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                let n = seed % 8;
                for i in 0..n {
                    let ch = match (seed + i) % 5 {
                        0 => b'@',
                        1 => b'.',
                        2 => b'a',
                        3 => b'1',
                        _ => b'x',
                    };
                    c.bus.write8(DATA + 0x400 + i, ch);
                }
                c.bus.write8(DATA + 0x400 + n, 0);
            },
        );
    }


    #[test]
    fn c_fe_state_create_step2_set_year_selection_matches() {
        check(
            ee::s03::f_001bb3b0__generated,
            c_fe_state_create_step2_set_year_selection,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1bb3b0, 3);
                let screen = DATA + 0x1000;
                let node = screen_with(c, &mut r, screen, 0);
                c.bus.write32(LOG_RESULTS, 0x11);
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 0x40, if seed % 4 == 0 { 0 } else { screen });
                // `*(self+0x6d0)+0x30` holds the year index.
                c.bus.write32(DATA + 0x6d0, DATA + 0x2000);
                c.bus.write8(DATA + 0x2000 + 0x30, 0x6c + (seed % 8) as u8);
                let _ = node;
            },
        );
    }


    pub(crate) const SEEDS_12: u32 = 200;

    /// An empty guest function (returns at once, 2 cycles).
    pub(crate) const EMPTY: u32 = 0x0038_dc60;

    pub(crate) fn w16(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write16(a, v as u16);
    }

    pub(crate) fn w32(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// A `gp_12`-relative global address.
    pub(crate) fn gpat(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    /// `{ short delta; short; void (*fn)(); }` at `vt + off`.
    pub(crate) fn slot_6(c: &mut Cpu, vt: u32, off: u32, func: u32) {
        w16(c, vt + off, 0);
        w32(c, vt + off + 4, func);
    }

    /// A zeroed BE-library singleton at `gp_12-0x1f04` (otherwise the UI/FE
    /// helpers fall into the lazy disc load at `0x3de768`).
    pub(crate) fn be_library_8(c: &mut Cpu) {
        w32(c, gpat(-0x1f04), DATA + 0x30_0000);
    }

    /// The engine's size-class pool_4 allocator: the "pools ready" flag plus
    /// sixteen free nodes per size class.
    pub(crate) fn pool_4(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w32(c, gpat(0xdd8), 1);
        for (i, (slot_off, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool_4 = DATA + 0x40_0000 + 0x4_0000 * i;
            let base = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..16u32 {
                let node = base + 0x400 * n;
                w32(c, node, if n == 15 { 0 } else { node + 0x400 });
            }
            w32(c, pool_4 + head, base);
            w32(c, GAME_GP + slot_off, pool_4);
        }
    }

    /// The world_5 singleton at `gp_12-0x848` with an empty race-manager rider
    /// list (a rider loop on junk data runs away).
    pub(crate) fn world_5(c: &mut Cpu) {
        let w = DATA + 0x2_0000;
        w32(c, gpat(-0x848), w);
        w32(c, w + 0x78, DATA + 0x2_1000);
        w32(c, w + 0x84, DATA + 0x2_2000);
        w32(c, DATA + 0x2_2000 + 0xc, DATA + 0x2_3000);
        w32(c, DATA + 0x2_2000 + 0x84, DATA + 0x2_4000);
        w32(c, DATA + 0x2_4000, 0);
        w32(c, w + 0xc0, DATA + 0x2_5000);
    }

    /// All the standard fixtures.
    pub(crate) fn env(c: &mut Cpu) {
        be_library_8(c);
        world_5(c);
        pool_4(c);
    }


    #[test]
    fn cfe_state_net_messenger_on_child_event_matches() {
        check(
            ee::s03::f_001a4c58__generated,
            cfe_state_net_messenger_on_child_event,
            SEEDS_12,
            &|c, seed| {
                machine(c, seed, 0x1a4c, 0);
                env(c);
                let this = DATA + 0x400;
                let msg = DATA + 0x500;
                let vt = DATA + 0x800;
                let kind = [0x80bu32, 0x809, 0x80c, 0x800, 0x900][(seed % 5) as usize];
                for off in [0x108u32, 0x110, 0x118] {
                    slot_6(c, vt, off, EMPTY);
                }
                w32(c, this + 8, vt);
                w32(c, msg + 0x18, kind);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(msg);
                c.g[A2][0] = sx(DATA + 0x600);
            },
        );
    }


    #[test]
    fn net_fe_state_v26_matches() {
        check(ee::s03::f_001a89b0__generated, net_fe_state_v26, SEEDS_12, &|c, seed| {
            machine(c, seed, 0x1a89, 0);
            env(c);
            let this = DATA + 0x400;
            let msg = DATA + 0x500;
            let vt = DATA + 0x800;
            let popup = DATA + 0x900;
            let kind = [0x31u32, 0x33, 0x4d, 0x4e, 0x800, 0x999][(seed % 6) as usize];
            w32(c, this + 0x6a8, if seed % 4 == 0 { 0 } else { 1 });
            // The disconnected popup returns at once when it is already open
            // (its full path walks the UI tree and is not this test's subject).
            w32(c, this + 0x6ac, 1);
            w32(c, this + 0x6a0, popup);
            w32(c, gpat(-0xc8), if seed % 5 == 0 { 1 } else { 0 });
            w32(c, msg + 0xc, kind);
            w32(c, msg + 8, vt);
            slot_6(c, vt, 8, EMPTY);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(msg);
        });
    }


        #[test]
    fn cfe_state_create_step2_submit_account_matches() {
        check(
            ee::s03::f_001ba9f8__generated,
            cfe_state_create_step2_submit_account,
            SEEDS_12,
            &|c, seed| {
                machine(c, seed, 0x1ba9, 0);
                env(c);
                let this = DATA + 0x400;
                let popup = DATA + 0x900;
                w32(c, this + 0x6a0, popup);
                w32(c, this + 0x40, 0);
                w32(c, popup + 0x2b8, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }


    #[test]
    fn net_fe_state_show_max_buddies_popup_matches() {
        check(
            ee::s03::f_001abed8__generated,
            net_fe_state_show_max_buddies_popup,
            SEEDS_12,
            &|c, seed| {
                machine(c, seed, 0x1abe, 0);
                env(c);
                let this = DATA + 0x400;
                let popup = DATA + 0x1000;
                w32(c, this + 0x6a0, popup);
                w32(c, this + 0x40, 0);
                w32(c, popup + 0x2b8, 0);
                c.g[A0][0] = sx(this);
            },
        );
    }


    #[test]
    fn c_fe_use_existing_account_init_matches() {
        check(
            ee::s03::f_001ac0b8__generated,
            c_fe_use_existing_account_init,
            SEEDS_12,
            &|c, seed| {
                machine(c, seed, 0x1ac0, 0);
                env(c);
                let this = DATA + 0x400;
                w32(
                    c,
                    this + 0x10,
                    if seed % 2 == 0 { 0 } else { DATA + 0x1000 },
                );
                w32(c, this + 0x40, 0);
                w32(c, gpat(-0xc8), DATA + 0x1800);
                c.g[A0][0] = sx(this);
            },
        );
    }


    // ------------------------------------------------------------------
    // Front-end / online state handlers.

    #[test]
    fn cfe_state_select_persona_on_save_account_popup_event_matches() {
        check(
            ee::s03::f_001ad930__generated,
            cfe_state_select_persona_on_save_account_popup_event,
            40,
            &|c, seed| {
                let mut r = base_8(c, seed, 0x1ad930, 3);
                let obj = DATA + 0x1000;
                let child = DATA + 0x2000;
                let screen = DATA + 0x3000;
                let holder = DATA + 0x4000;
                let list = DATA + 0x5000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = child as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0xf
                } else {
                    (seed % 0x20) as u64
                };
                w(c, obj + 0x10, holder);
                w(c, obj + 0x40, screen);
                w(c, obj + 0x69c, 0);
                w(c, screen + 0xd0, holder);
                w(c, holder + 0x10, list);
                w(c, child + 0x18, r.next_u32() % 0x900);
            },
        );
    }


    #[test]
    fn cfe_state_online_character_select_init_matches() {
        check(
            ee::s03::f_001b0fb0__generated,
            cfe_state_online_character_select_init,
            30,
            &|c, seed| {
                let mut r = base_8(c, seed, 0x1b0fb0, 3);
                let obj = DATA + 0x1000;
                let engine = DATA + 0x2000;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0x10, if seed % 3 == 0 { 0 } else { engine });
                w(c, obj + 0x6e0, r.next_u32());
                w(c, engine + 0x4, engine + 0x100);
                w(c, engine + 0x10c, 0);
            },
        );
    }


            #[test]
    fn cfe_state_create_step1_transition_in_matches() {
        check(
            ee::s03::f_001b8a08__generated,
            cfe_state_create_step1_transition_in,
            40,
            &|c, seed| {
                let mut r = base_8(c, seed, 0x1b8a08, 3);
                let obj = DATA + 0x1000;
                let popup = DATA + 0x2000;
                let vt = DATA + 0x3000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { 1 };
                w(c, obj + 0x6f0, if seed % 3 == 0 { 0 } else { popup });
                w(c, popup + 0x8, vt);
                slot_7(c, vt, 0x40, 0, FAKE);
                w(c, obj + 0x10, DATA + 0x4000);
                w(c, obj + 0x40, DATA + 0x5000);
                w8(c, obj + 0x15, r.next_u32() as u8);
            },
        );
    }


    pub(crate) const SEEDS_13: u32 = 200;

    /// The zeroed BELibrary dummy and the world singleton.
    pub(crate) const BE_4: u32 = DATA + 0x1_0000;

    pub(crate) const W_3: u32 = DATA + 0x1_4000;

    pub(crate) const RET_TWO: u32 = 0x0014_5854;

    pub(crate) fn wb(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }

    pub(crate) fn put_str_4(c: &mut Cpu, a: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            wb(c, a + i as u32, *b);
        }
        wb(c, a + s.len() as u32, 0);
    }

    /// `{ short delta; void (*fn)(); }` at `vt + off`.
    pub(crate) fn slot_8(c: &mut Cpu, vt: u32, off: u32, target: u32) {
        wh(c, vt + off, 0);
        w(c, vt + off + 4, target);
    }


    /// EAFile's request and file-handle pools (see oc117): with no free entry
    /// the allocators spin forever (`0x3de768`).
    pub(crate) fn eafile_pools(c: &mut Cpu) {
        for (count, table, at, size) in [
            (0x0051_9ab0, 0x0051_9ad8, 0xf_0000, 0x30),
            (0x0051_9ab4, 0x0051_9adc, 0xf_1000, 0x110),
        ] {
            let base_9 = DATA + at;
            w(c, count, 4);
            w(c, table, base_9);
            for i in 0..size {
                w(c, base_9 + 4 * i, 0);
            }
        }
    }


    pub(crate) fn run_7(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = 0x0012_3450;
        c.pc = addr;
        f(c);
        c.g[V0][0]
    }


    pub(crate) fn heap_init_8(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        run_7(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
    }


    /// The engine's size-class pool allocator, enabled through `gp + 0xdd8`.
    pub(crate) fn pool_allocator_9(c: &mut Cpu) {
        w(c, GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),
            (0x2a60, 0x2000, 0x10),
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot_8, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base_9 = nodes;
            for n in 0..16u32 {
                let node = base_9 + block * n;
                w(c, node, if n == 15 { 0 } else { node + block });
            }
            w(c, pool + head, base_9);
            w(c, GAME_GP + slot_8, pool);
            nodes += block * 16;
        }
    }


    /// Scrambled registers, fake hooks, a game-like `gp`, the BE_4 singleton and
    /// a world with no riders.
    pub(crate) fn base_9(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        w(c, GAME_GP.wrapping_sub(0x1f04), BE_4);
        w(c, GAME_GP.wrapping_sub(0x1efc), 0);
        w(c, GAME_GP.wrapping_sub(0x848), W_3);
        let game = W_3 + 0x1000;
        w(c, W_3 + 0x84, game);
        let race = W_3 + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
        r
    }


    /// [`base_9`] plus the pool allocator and dlmalloc arena.
    pub(crate) fn alloc_base_3(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = base_9(c, seed, salt, calls);
        eafile_pools(c);
        pool_allocator_9(c);
        heap_init_8(c);
        r
    }


    #[test]
    fn fe_net_messenger_show_looking_for_self_matches() {
        check(
            ee::s03::f_001a4d90__generated,
            fe_net_messenger_show_looking_for_self,
            SEEDS_13,
            &alloc_base_setup(|c, seed, r| {
                let this = DATA;
                w(c, this + 0x10, DATA + 0x800);
                w(
                    c,
                    this + 0x6a0,
                    if seed & 1 == 0 { 0 } else { DATA + 0x1000 },
                );
                c.g[A0][0] = this as u64;
                let _ = r;
            }),
        );
    }


    #[test]
    fn fe_net_messenger_has_name_matches() {
        check(
            ee::s03::f_001a56d0__generated,
            fe_net_messenger_has_name,
            SEEDS_13,
            &|c, seed| {
                let _ = base_9(c, seed, 0x56d0, 2);
                let this = DATA;
                let count = seed % 4;
                let query = DATA + 0x9000;
                let words: [&[u8]; 4] = [b"alpha", b"beta", b"gamma", b"delta"];
                put_str_4(c, query, words[(seed % 4) as usize]);
                let names = DATA + 0x8000;
                for i in 0..count {
                    let s = DATA + 0x8200 + 0x40 * i;
                    put_str_4(c, s, words[i as usize]);
                    if seed & 1 == 0 && i == 0 {
                        put_str_4(c, s, words[(seed % 4) as usize]);
                    }
                    w(c, names + 4 * i, s);
                }
                w(c, this + 0x6e8, names);
                w(c, this + 0x6ec, names + 4 * count);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = query as u64;
            },
        );
    }


    #[test]
    fn fe_net_messenger_find_name_str_matches() {
        check(
            ee::s03::f_001a5800__generated,
            fe_net_messenger_find_name_str,
            SEEDS_13,
            &|c, seed| {
                let _ = base_9(c, seed, 0x5800, 2);
                let this = DATA;
                let count = seed % 4;
                let query = DATA + 0x9000;
                let words: [&[u8]; 4] = [b"alpha", b"beta", b"gamma", b"delta"];
                put_str_4(c, query, words[(seed % 4) as usize]);
                let names = DATA + 0x8000;
                for i in 0..count {
                    let s = DATA + 0x8200 + 0x40 * i;
                    put_str_4(c, s, words[i as usize]);
                    if seed & 2 == 0 && i == 0 {
                        put_str_4(c, s, words[(seed % 4) as usize]);
                    }
                    w(c, names + 4 * i, s);
                }
                w(c, this + 0x6e8, names);
                w(c, this + 0x6ec, names + 4 * count);
                w(c, DATA + 0x9100, query);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x9100) as u64;
            },
        );
    }


    #[test]
    fn net_fe_state_on_keyboard_popup_event_matches() {
        check(
            ee::s03::f_001a94e0__generated,
            net_fe_state_on_keyboard_popup_event,
            SEEDS_13,
            &alloc_base_setup(|c, seed, r| {
                let this = DATA;
                let sub = DATA + 0x800;
                let vt = vtable(c, r, &[0x100]);
                w(c, this + 8, vt);
                let a2 = [0x15u64, 0x16, 0x17][(seed % 3) as usize];
                c.g[A0][0] = this as u64;
                c.g[A1][0] = sub as u64;
                c.g[A2][0] = a2;
            }),
        );
    }


    #[test]
    fn fe_state_chat_room_transition_in_matches() {
        check(
            ee::s03::f_001b3490__generated,
            fe_state_chat_room_transition_in,
            SEEDS_13,
            &alloc_base_setup(|c, seed, r| {
                let this = DATA;
                let has_arg = seed & 1;
                for i in 0..2u32 {
                    let entry = DATA + 0x800 + 0x40 * i;
                    let vt = vtable(c, r, &[0x38]);
                    w(c, entry + 8, vt);
                    w(
                        c,
                        this + 0x6fc + 4 * i,
                        if (seed >> i) & 1 == 0 { 0 } else { entry },
                    );
                }
                w(c, this + 0x6ec, 0x100);
                w(c, this + 0x6f4, 0x100);
                let audio = DATA + 0x1000;
                w(c, gp(c, -0xc8), audio);
                w(c, audio + 0x80, DATA + 0x2000);
                put_str_4(c, DATA + 0x2000, b"player");
                c.g[A0][0] = this as u64;
                c.g[A1][0] = has_arg as u64;
            }),
        );
    }


    #[test]
    fn fe_state_online_agreement_init_matches() {
        check(
            ee::s03::f_001c04a8__generated,
            fe_state_online_agreement_init,
            SEEDS_13,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let engine = DATA + 0x800;
                w(c, engine + 4, DATA + 0xa00);
                w(c, DATA + 0xa00 + 0xc, 0x100);
                w(c, this + 0x10, engine);
                w(c, this + 0x40, 0);
                let _ = seed;
                c.g[A0][0] = this as u64;
            }),
        );
    }


    #[test]
    fn fe_state_online_agreement_v25_matches() {
        check(
            ee::s03::f_001c0ea8__generated,
            fe_state_online_agreement_v25,
            SEEDS_13,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let args = DATA + 0x800;
                let _ = seed;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = args as u64;
            }),
        );
    }


    // A setup wrapper that also runs `alloc_base_3`.
    pub(crate) fn alloc_base_setup<'a>(
        f: impl Fn(&mut Cpu, u32, &mut Rng) + 'a,
    ) -> impl Fn(&mut Cpu, u32) + 'a {
        move |c, seed| {
            let mut r = alloc_base_3(c, seed, 0x1250, 3);
            f(c, seed, &mut r);
        }
    }


    // --------------------------------------------------------------- net events

    #[test]
    fn net_fe_state_queue_net_event_matches() {
        check(ssx3_ee::s03::f_001a8ae8__generated, net_fe_state_queue_net_event, 16, &|c, seed| {
            let mut r = base_10(c, seed, 0x001a_8ae8);
            elf_image(c);
            heap(c);
            pool_6(c);
            let state = DATA + 0x100;
            let event = DATA + 0x900;
            let vtable = DATA + 0xa00;
            let node = DATA + 0x3000;
            clear_5(c, state, 0x700);
            clear_5(c, node, 0x20);
            w(c, state + 0x6cc, node);
            w(c, node + 4, 0);
            clear_5(c, event, 0x40);
            clear_5(c, vtable, 0x40);
            w(c, event + 4, vtable);
            if seed % 3 == 0 {
                // Virtual conversion returns the payload.
                c.bus.write16(vtable + 0x10, 0);
                w(c, vtable + 0x14, crate::difftest::fakes::FAKE);
                w(c, crate::difftest::fakes::LOG_RESULTS, 0x4242);
            } else {
                // Virtual conversion returns null (MenuSys_IsActive on a
                // zeroed first word), so nothing is queued.
                c.bus.write16(vtable + 0x10, 0);
                w(c, vtable + 0x14, 0x002c_c098);
                w(c, event, 0);
            }
            c.g[A0][0] = sx(state);
            c.g[A1][0] = if seed % 5 == 0 { 0 } else { sx(event) };
        });
    }


    pub(crate) fn fill_setup(c: &mut Cpu, seed: u32, salt: u64) -> (Rng, u32) {
        let mut r = base_10(c, seed, salt);
        let list_box = DATA + 0x100;
        let p1 = DATA + 0x2000;
        let p2 = DATA + 0x2100;
        let p3 = DATA + 0x2200;
        let node = DATA + 0x3000;
        clear_5(c, list_box, 0x340);
        clear_5(c, p1, 0x100);
        clear_5(c, p2, 0x100);
        clear_5(c, p3, 0x100);
        clear_5(c, node, 0x60);
        // The add-item path stops at the null object chain: no allocation.
        w(c, list_box + 0x5c, p1);
        w(c, p1 + 0xd0, p2);
        w(c, p2 + 0x10, p3);
        w(c, p3 + 0x10, 0);
        // The parent list's node: delta 0 and a miss-hook target.
        w(c, list_box + 8, node);
        c.bus.write16(node + 0x48, 0);
        w(c, node + 0x4c, 0);
        w(c, list_box + 0x14, r.next_u32());
        w(c, list_box + 0x318, (seed % 4) as u32);
        (r, list_box)
    }


    #[test]
    fn net_fe_state_fill_yes_no_list_matches() {
        check(ssx3_ee::s03::f_001a9a18__generated, net_fe_state_fill_yes_no_list, 12, &|c, seed| {
            let (_, list_box) = fill_setup(c, seed, 0x001a_9a18);
            c.g[A0][0] = sx(DATA + 0x1000);
            c.g[A1][0] = sx(list_box);
            c.g[A2][0] = (seed % 3) as u64;
        });
    }


    #[test]
    fn net_fe_state_fill_on_off_list_matches() {
        check(ssx3_ee::s03::f_001a9ab8__generated, net_fe_state_fill_on_off_list, 12, &|c, seed| {
            let (_, list_box) = fill_setup(c, seed, 0x001a_9ab8);
            c.g[A0][0] = sx(DATA + 0x1000);
            c.g[A1][0] = sx(list_box);
            c.g[A2][0] = (seed % 3) as u64;
        });
    }


    // ------------------------------------------------------- front-end widget

    #[test]
    fn c_fe_state_net_ssx_world_on_widget_created_matches() {
        check(
            ssx3_ee::s03::f_001c1e08__generated,
            c_fe_state_net_ssx_world_on_widget_created,
            12,
            &|c, seed| {
                let mut r = base_10(c, seed, 0x001c_1e08);
                let widget = DATA + 0x100;
                clear_5(c, widget, 0x40);
                put_str_5(c, 0x0046_5a80, "widget_a");
                put_str_5(c, 0x0046_5a90, "widget_b");
                put_str_5(c, 0x0046_5aa8, "widget_c");
                put_str_5(c, 0x0046_5ac0, "widget_d");
                put_str_5(c, DATA + 0x2000, "other");
                let name = match seed % 5 {
                    0 => 0x0046_5a80,
                    1 => 0x0046_5a90,
                    2 => 0x0046_5aa8,
                    3 => 0x0046_5ac0,
                    _ => DATA + 0x2000,
                };
                w(c, widget + 0x38, name);
                w(c, widget + 0x18, r.next_u32());
                c.g[A0][0] = sx(DATA + 0x9000);
                c.g[A1][0] = sx(widget);
            },
        );
    }


    #[test]
    fn net_fe_state_on_mem_card_login_popup_event_matches() {
        check(
            ssx3_ee::s03::f_001a9438__generated,
            net_fe_state_on_mem_card_login_popup_event,
            8,
            &|c, seed| {
                let _ = base_10(c, seed, 0x001a_9438);
                elf_image(c);
                heap(c);
                pool_6(c);
                let state = DATA + 0x100;
                let vtable = DATA + 0x2000;
                let event_obj = DATA + 0x4000;
                clear_5(c, state, 0x800);
                clear_5(c, vtable, 0x200);
                clear_5(c, event_obj, 0x40);
                w(c, state + 8, vtable);
                w(c, state + 0x10, DATA + 0x3000);
                c.bus.write16(vtable + 0x100, 0);
                w(c, vtable + 0x104, crate::difftest::fakes::FAKE);
                w(c, crate::difftest::fakes::LOG_RESULTS, 1);
                c.g[A0][0] = sx(state);
                c.g[A1][0] = sx(event_obj);
                c.g[A2][0] = if seed % 3 == 0 { 0x16 } else { (seed % 0x20) as u64 };
            },
        );
    }


    // ---------------------------------------------------- read-message popup

    #[test]
    fn c_fe_state_read_message_popup_add_sent_message_matches() {
        check(
            ssx3_ee::s03::f_001a7ed8__generated,
            c_fe_state_read_message_popup_add_sent_message,
            8,
            &|c, seed| {
                let _ = base_10(c, seed, 0x001a_7ed8);
                elf_image(c);
                heap(c);
                pool_6(c);
                let popup = DATA + 0x100;
                let player = DATA + 0x2000;
                clear_5(c, popup, 0x800);
                clear_5(c, player, 0x100);
                put_str_5(c, DATA + 0x3000, "sender");
                put_str_5(c, DATA + 0x3100, "message body");
                w(c, player + 0x80, DATA + 0x3000);
                w(c, GAME_GP.wrapping_sub(0xc8), player);
                c.g[A0][0] = sx(popup);
                c.g[A1][0] = sx(DATA + 0x3100);
            },
        );
    }


    // ------------------------------------------------------------------
    // Shared fixtures.

    pub(crate) fn w_4(c: &mut Cpu, at: u32, v: u32) {
        c.bus.write32(at, v);
    }


    /// The BELibrary dummy, the world/race-manager chain and the interface
    /// table the game normally builds at boot.
    pub(crate) fn globals_9(c: &mut Cpu) {
        let be = DATA + 0x30_0000;
        let world = DATA + 0x8_0000;
        w_4(c, GAME_GP.wrapping_sub(0x1f04), be);
        w_4(c, GAME_GP.wrapping_sub(0x1efc), 0);
        w_4(c, 0x0053_08a8, DATA + 0x30_1000);
        w_4(c, GAME_GP.wrapping_sub(0x848), world);
        let game = world + 0x1000;
        w_4(c, world + 0x84, game);
        let race = world + 0x2000;
        w_4(c, game + 0xc, race);
        w_4(c, race + 0x7c, 0);
        w_4(c, race + 0x28, world + 0x3000);
        w_4(c, DATA + 0x30_1000 + 4, DATA + 0x30_2000);
        w_4(c, DATA + 0x30_2000 + 8, 0);
        w_4(c, DATA + 0x30_2000 + 0xc, fakes::FAKE);
    }


    /// The engine's size-class pool_7 allocator: each pool_7's start is stored in
    /// its `gp` slot, its free-list head at `start + size`, and eight free
    /// blocks inside the pool_7's range (so `MEM_IsPoolPointer` accepts them).
    pub(crate) fn pool_7(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w_4(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, size)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let start = DATA + 0x40_0000 + 0x4_0000 * i;
            for n in 0..8u32 {
                let node = start + 0x40 * (n + 1);
                w_4(c, node, if n == 7 { 0 } else { node + 0x40 });
            }
            w_4(c, start + size, start + 0x40);
            w_4(c, GAME_GP + slot, start);
        }
    }


    pub(crate) fn heap_init_9(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const RET: u64 = 0x0012_3450;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x20_0000;
        c.g[RA][0] = RET;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, RET as u32, "DLHeap_InitArena did not return");
    }


    pub(crate) fn base_11(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        c.g[GP][0] = GAME_GP as u64;
        heap_init_9(c);
        let r = fakes::machine(c, seed, salt, calls);
        globals_9(c);
        pool_7(c);
        r
    }


    /// A screen whose object list returns `node` for hash 0 (`StrHash("")` is 0
    /// on the test machine: the tag strings are not loaded).
    pub(crate) fn hash_object_2(c: &mut Cpu, screen: u32, node: u32) {
        w_4(c, screen + 0x38, DATA + 0x32_0100);
        w_4(c, DATA + 0x32_0100, 1);
        w_4(c, screen + 0x3c, DATA + 0x32_0000);
        w_4(c, DATA + 0x32_0000, node);
        w_4(c, node + 0x38, 0);
    }


    // ------------------------------------------------------------------
    // Front end / online states.

        #[test]
    fn cfe_state_challenge_popup_show_set_options_matches() {
        check(ssx3_ee::s03::f_001af688__generated, cfe_state_challenge_popup_show_set_options, 20, &|c, seed| {
            let _r = base_11(c, seed, 0x1af688, 0);
            let this = DATA + 0x1000;
            let screen = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            w_4(c, this + 0x40, screen);
            w_4(c, this + 0x6d0, 0xdead);
            if seed % 2 == 0 {
                w_4(c, screen + 0x38, DATA + 0x32_0100);
                w_4(c, DATA + 0x32_0100, 0);
                w_4(c, screen + 0x3c, DATA + 0x32_0000);
            } else {
                let node = DATA + 0x3000;
                hash_object_2(c, screen, node);
                w_4(c, node + 8, DATA + 0x4000);
                c.bus.write16(DATA + 0x4000 + 0x40, 0x40);
                w_4(c, DATA + 0x4000 + 0x44, 1);
            }
        });
    }


    #[test]
    fn cfe_state_net_config_ctor_matches() {
        check(ssx3_ee::s03::f_001b3f78__generated, cfe_state_net_config_ctor, 8, &|c, seed| {
            let mut r = base_11(c, seed, 0x1b3f78, 2);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (DATA + 0x2000) as u64;
            c.g[A2][0] = (r.next_u32() & 0xff) as u64;
            w_4(c, this + 0x6ec, r.next_u32());
        });
    }


    #[test]
    fn cfe_state_create_step1_backup_account_info_matches() {
        check(ssx3_ee::s03::f_001ba178__generated, cfe_state_create_step1_backup_account_info, 30, &|c, seed| {
            let _r = base_11(c, seed, 0x1ba178, 2);
            let this = DATA + 0x1000;
            let acct = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            w_4(c, this + 0x6ec, if seed % 2 == 0 { 0 } else { 1 });
            w_4(c, this + 0x6d0, acct);
            w_4(c, acct + 4, 0);
            w_4(c, acct + 8, 0);
            w_4(c, acct + 0xc, 0);
            w_4(c, acct + 0x38, 0x11);
            w_4(c, acct + 0x3c, 0x22);
        });
    }


        #[test]
    fn net_fe_state_refresh_buddy_action_popup_matches() {
        check(ssx3_ee::s03::f_001ab3d0__generated, net_fe_state_refresh_buddy_action_popup, 30, &|c, seed| {
            let _r = base_11(c, seed, 0x1ab3d0, 2);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (DATA + 0x2000) as u64;
            c.g[A2][0] = (DATA + 0x3000) as u64;
            w_4(c, this + 0x1c, match seed % 4 { 0 => 0, 1 => 2, 2 => 4, _ => 6 });
            let popup = DATA + 0x4000;
            w_4(c, this + 0x6a0, if seed % 5 == 0 { 0 } else { popup });
            w_4(c, this + 0x6c4, if seed % 3 == 0 { 0 } else { 1 });
            w_4(c, popup + 0x18, [0x80bu32, 0x80c, 0x900, 0x80b][seed as usize % 4]);
            c.bus.write8(popup + 0x1d, match seed % 3 { 0 => 0, 1 => 5, _ => 0x40 });
        });
    }


    #[test]
    fn cfe_use_existing_account_check_email_matches() {
        check(ssx3_ee::s03::f_001ac448__generated, cfe_use_existing_account_check_email, 20, &|c, seed| {
            let _r = base_11(c, seed, 0x1ac448, 3);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            let email = DATA + 0x2000;
            w_4(c, this + 0x6dc, email + 8);
            w_4(c, email, if seed % 2 == 0 { 0 } else { 0x10 });
            w_4(c, this + 0x6a0, 0);
        });
    }


    /// Objects and buffers in the test data area (below the fixtures'
    /// reserved blocks: BE at +0x30000, WORLD_5 at +0x80000, the fake log at
    /// 0x900000 and the pools above +0x400000).
    pub(crate) const OBJ_3: u32 = DATA;

    pub(crate) const OBJ2_3: u32 = DATA + 0x800;

    pub(crate) const EV_2: u32 = DATA + 0x1000;

    pub(crate) const SCR: u32 = DATA + 0x10000;

    pub(crate) const CHILD: u32 = DATA + 0x11000;

    pub(crate) const BUF: u32 = DATA + 0x12000;

    pub(crate) const ENG: u32 = DATA + 0x16000;

    pub(crate) const WORLD_5: u32 = DATA + 0x8_0000;


    /// The BELibrary singleton (`gp_15 - 0x1f04`, zeroed so nothing waits for
    /// the disc) and the world (`gp_15 - 0x848`, a dummy with no riders).
    pub(crate) fn globals_10(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x848), WORLD_5);
        let game = WORLD_5 + 0x1000;
        w(c, WORLD_5 + 0x84, game);
        let race = WORLD_5 + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }


    /// The engine's size-class pool_8 allocator (same fixture as the other
    /// waves): `gp_15 + 0xdd8` ready plus eight free nodes per size class.
    pub(crate) fn pool_8(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool_8 = DATA + 0x40_0000 + 0x4_0000 * i;
            let base_12 = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base_12 + 0x400 * n;
                w(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w(c, pool_8 + head, base_12);
            w(c, GAME_GP + slot, pool_8);
        }
    }


    /// Scrambled state, fake call hooks, a game-like `gp_15` and the standard
    /// fixtures.
    pub(crate) fn base_12(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        globals_10(c);
        pool_8(c);
        r
    }


    // ---------------------------------------------------------------- front end

        #[test]
    fn cfe_online_character_select_show_opponent_char_matches() {
        check(
            ee::s03::f_001b1760__generated,
            cfe_online_character_select_show_opponent_char,
            8,
            &|c, seed| {
                let _ = base_12(c, seed, 0x1b1760, 2);
                c.g[A0][0] = OBJ_3 as u64;
                c.g[A1][0] = (seed % 4) as u64;
                w(c, OBJ_3 + 0x40, if seed % 3 == 0 { 0 } else { SCR });
                w(c, 0x0046_3830, 0);
            },
        );
    }


    #[test]
    fn cfe_create_step2_fill_year_list_matches() {
        check(
            ee::s03::f_001bb798__generated,
            cfe_create_step2_fill_year_list,
            8,
            &|c, seed| {
                let mut r = base_12(c, seed, 0x1bb798, 3);
                c.g[A0][0] = OBJ_3 as u64;
                c.g[A1][0] = SCR as u64;
                c.g[A2][0] = (seed % 200) as u64;
                w(c, SCR + 8, CHILD);
                w(c, SCR + 0x14, r.next_u32());
                w(c, CHILD + 0x48, 0);
                w(c, CHILD + 0x4c, 0);
                w(c, 0x004a_12c0, 0);
            },
        );
    }


    #[test]
    fn cfe_online_main_menu_init_matches() {
        check(
            ee::s03::f_001bfbf8__generated,
            cfe_online_main_menu_init,
            6,
            &|c, seed| {
                let _ = base_12(c, seed, 0x1bfbf8, 4);
                c.g[A0][0] = OBJ_3 as u64;
                w(c, OBJ_3 + 0x10, ENG);
                w(c, 0x0046_53f0, 0);
                if seed % 2 == 0 {
                    w(c, GAME_GP.wrapping_sub(0xc8), 0);
                } else {
                    w(c, GAME_GP.wrapping_sub(0xc8), OBJ2_3);
                    w(c, OBJ2_3, 1);
                }
                w(c, GAME_GP.wrapping_sub(0x204), 0);
            },
        );
    }


    #[test]
    fn cfe_net_quick_play_popup_set_options_hint_visible_matches() {
        check(
            ee::s03::f_001c13e0__generated,
            cfe_net_quick_play_popup_set_options_hint_visible,
            8,
            &|c, seed| {
                let _ = base_12(c, seed, 0x1c13e0, 2);
                c.g[A0][0] = OBJ_3 as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, OBJ_3 + 0x40, if seed % 4 == 0 { 0 } else { SCR });
                w(c, 0x0046_3ca0, 0);
                w(c, 0x0046_58f0, 0);
            },
        );
    }


    #[test]
    fn cfe_net_quick_messages_on_reset_popup_event_matches() {
        check(
            ee::s03::f_001c2ba8__generated,
            cfe_net_quick_messages_on_reset_popup_event,
            8,
            &|c, seed| {
                let _ = base_12(c, seed, 0x1c2ba8, 3);
                c.g[A0][0] = OBJ_3 as u64;
                c.g[A1][0] = EV_2 as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0xf
                } else {
                    (seed % 8) as u64
                };
                w(c, OBJ_3 + 0x6d0, 0);
                w(c, EV_2 + 0x18, 0);
            },
        );
    }


    pub(crate) const SEEDS_14: u32 = 40;

    /// The zeroed BELibrary / interface dummy.
    pub(crate) const BE_6: u32 = DATA + 0x2000;

    pub(crate) const IFACE: u32 = DATA + 0x4000;

    /// The world and its game / race manager chain.
    pub(crate) const W_4: u32 = DATA + 0x3000;

    /// General scratch (objects, vtables) below the pool area.
    pub(crate) const SCRATCH: u32 = DATA + 0x5000;


    /// A vtable slot_9 `{ short delta; void (*fn)(); }` pointing at_5 fake `i`.
    pub(crate) fn slot_9(c: &mut Cpu, vt: u32, off: u32, i: u32) {
        wh(c, vt + off, 0);
        w(c, vt + off + 4, fakes::FAKE + 0x10 * i);
    }


    /// Run a generated function for setup (entering at_5 its own address, so a
    /// dispatcher-shaped one starts at_5 its entry), returning `v0`.
    pub(crate) fn run_8(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.pc = addr;
        f(c);
        c.g[V0][0]
    }


    pub(crate) fn heap_init_10(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        run_8(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
    }


    /// The engine's size-class pool allocator, enabled through `gp_16 + 0xdd8`.
    pub(crate) fn pool_allocator_10(c: &mut Cpu) {
        w(c, GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),
            (0x2a60, 0x2000, 0x10),
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot_9, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base_13 = nodes;
            for n in 0..16u32 {
                let node = base_13 + block * n;
                w(c, node, if n == 15 { 0 } else { node + block });
            }
            w(c, pool + head, base_13);
            w(c, GAME_GP + slot_9, pool);
            nodes += block * 16;
        }
    }


    /// Scrambled registers, the fake hooks, a game-like `gp_16`, the BE_6/interface
    /// singleton, a world with no riders, a working heap and pool allocator.
    pub(crate) fn base_13(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        pool_allocator_10(c);
        heap_init_10(c);
        w(c, GAME_GP.wrapping_sub(0x1f04), BE_6);
        w(c, GAME_GP.wrapping_sub(0x848), W_4);
        w(c, GAME_GP.wrapping_sub(0xc8), IFACE);
        w(c, GAME_GP.wrapping_sub(0x854), IFACE);
        // world -> +0x84 game -> +0xc race -> +0x7c riders (none)
        w(c, W_4 + 0x84, W_4 + 0x1000);
        w(c, W_4 + 0x1000 + 0xc, W_4 + 0x2000);
        w(c, W_4 + 0x2000 + 0x7c, 0);
        // world +0x7c: the rider slot_9 table (empty) the bolt file scans.
        w(c, W_4 + 0x7c, DATA + 0x15000);
        // The interface's character-name strings and list head.
        w(c, IFACE + 0x48, SCRATCH + 0x1000);
        w(c, IFACE + 0x4c, SCRATCH + 0x1010);
        wb(c, SCRATCH + 0x1000, 0);
        wb(c, SCRATCH + 0x1010, 0);
        w(c, IFACE + 0xf0, SCRATCH + 0x1100);
        w(c, SCRATCH + 0x1100, 0);
        w(c, SCRATCH + 0x1104, 0);
        r
    }


    #[test]
    fn net_fe_state_show_user_found_matches() {
        check(
            ee::s03::f_001a4ce8__generated,
            net_fe_state_show_user_found,
            SEEDS_14,
            &|c, seed| {
                let _ = base_13(c, seed, 0x1a4c_e8, 8);
                let obj = SCRATCH;
                let src = obj + 0x1000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = src as u64;
                // The popup pre-exists so GetPopup does not allocate.
                let popup = obj + 0x2000;
                w(c, obj + 0x6a0, popup);
                for i in 0..0xa0u32 {
                    w(c, popup + 0xbc + 4 * i, 0);
                }
                // The source record: a name pointer at_5 +8 and an id at_5 +0x10.
                w(c, src + 8, SCRATCH + 0x1000);
                w(c, src + 0x10, seed.wrapping_mul(7));
                w(c, obj + 0x6d0, 0);
            },
        );
    }


    #[test]
    fn net_fe_state_on_disconnected_popup_event_matches() {
        check(
            ee::s03::f_001a9668__generated,
            net_fe_state_on_disconnected_popup_event,
            SEEDS_14,
            &|c, seed| {
                let _ = base_13(c, seed, 0x1a96_68, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A2][0] = (seed % 4) as u64;
                // The state's UI manager.
                let ui = obj + 0x400;
                w(c, obj + 0x10, ui);
                let state = ui + 0x18;
                w(c, ui + 0xd0, obj + 0x500);
                w(c, obj + 0x500 + 0x10, state);
                w(c, state + 0x18, obj + 0x600);
                slot_9(c, obj + 0x600, 0, 0);
                w(c, obj + 0x600 + 0x1c, 0);
            },
        );
    }


    #[test]
    fn net_fe_state_on_net_interrupted_popup_event_matches() {
        check(
            ee::s03::f_001a9710__generated,
            net_fe_state_on_net_interrupted_popup_event,
            SEEDS_14,
            &|c, seed| {
                let _ = base_13(c, seed, 0x1a97_10, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A2][0] = (seed % 4) as u64;
                let ui = obj + 0x400;
                w(c, obj + 0x10, ui);
                let state = ui + 0x18;
                w(c, ui + 0xd0, obj + 0x500);
                w(c, obj + 0x500 + 0x10, state);
                w(c, state + 0x18, obj + 0x600);
                slot_9(c, obj + 0x600, 0, 1);
                w(c, obj + 0x600 + 0x1c, 0);
            },
        );
    }


    #[test]
    fn cfe_use_existing_account_check_password_matches() {
        check(
            ee::s03::f_001ac2b8__generated,
            cfe_use_existing_account_check_password,
            SEEDS_14,
            &|c, seed| {
                let _ = base_13(c, seed, 0x1ac2_b8, 6);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                // Empty password for even seeds.
                w(
                    c,
                    IFACE + 0x4c,
                    if seed % 2 == 0 {
                        SCRATCH + 0x1010
                    } else {
                        SCRATCH + 0x1020
                    },
                );
                wb(c, SCRATCH + 0x1020, b'x');
                wb(c, SCRATCH + 0x1021, 0);
                let popup = obj + 0x2000;
                w(c, obj + 0x6a0, popup);
                for i in 0..0xa0u32 {
                    w(c, popup + 0xbc + 4 * i, 0);
                }
            },
        );
    }


    #[test]
    fn cfe_use_existing_account_check_account_name_matches() {
        check(
            ee::s03::f_001ac360__generated,
            cfe_use_existing_account_check_account_name,
            SEEDS_14,
            &|c, seed| {
                let _ = base_13(c, seed, 0x1ac3_60, 6);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                w(
                    c,
                    IFACE + 0x48,
                    if seed % 2 == 0 {
                        SCRATCH + 0x1000
                    } else {
                        SCRATCH + 0x1030
                    },
                );
                wb(c, SCRATCH + 0x1030, b'y');
                wb(c, SCRATCH + 0x1031, 0);
                let popup = obj + 0x2000;
                w(c, obj + 0x6a0, popup);
                for i in 0..0xa0u32 {
                    w(c, popup + 0xbc + 4 * i, 0);
                }
            },
        );
    }


    #[test]
    fn cfe_state_chat_room_handle_input_matches() {
        check(
            ee::s03::f_001b38d0__generated,
            cfe_state_chat_room_handle_input,
            SEEDS_14,
            &|c, seed| {
                let _ = base_13(c, seed, 0x1b38_d0, 8);
                let obj = SCRATCH;
                let ev = obj + 0x1000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = ev as u64;
                // The event target's vtable (`+8` object, slot_9 0xd8).
                let target = obj + 0x1100;
                w(c, ev + 8, target);
                slot_9(c, target, 0xd8, 0);
                // The keyboard object and its two helper calls.
                let kb = obj + 0x2000;
                w(c, obj + 0x69c, kb);
                w(c, kb + 0x38, 0);
                w(c, kb + 0x3c, 0);
                slot_9(c, kb, 0x38, 1);
                slot_9(c, kb, 0x40, 2);
                w(c, kb + 0x438, 0);
                w(c, kb + 0x43c, 1);
            },
        );
    }


    // ------------------------------------------------------- ASCII formatting

    pub(crate) fn format_fixture(c: &mut Cpu, seed: u32, buf: u32, fmt: u32) {
        // A short ASCII format, sometimes with one conversion.
        let s: &[u8] = match seed % 4 {
            0 => b"hello",
            1 => b"%d",
            2 => b"x=%d y=%d",
            _ => b"",
        };
        for (i, b) in s.iter().enumerate() {
            wb(c, fmt + i as u32, *b);
        }
        wb(c, fmt + s.len() as u32, 0);
        // The vararg save area the callee reads through a2.
        w(c, buf + 0x80, seed);
        w(c, buf + 0x84, 0xdead);
        w(c, buf + 0x88, 7);
    }


    #[test]
    fn ustr_sprintf_ascii_fmt_matches() {
        check(
            ee::s08::f_002c2718__generated,
            ustr_sprintf_ascii_fmt,
            SEEDS_14,
            &|c, seed| {
                let _ = base_13(c, seed, 0x2c27_18, 4);
                let buf = SCRATCH + 0x2000;
                let fmt = SCRATCH + 0x3000;
                for i in 0..0x1000u32 {
                    w(c, buf + 4 * i, 0);
                }
                format_fixture(c, seed, buf, fmt);
                c.g[A0][0] = buf as u64;
                c.g[A1][0] = fmt as u64;
                c.g[A2][0] = (seed as u64) << 40 | 3;
                c.g[A3][0] = 4;
                c.g[T0][0] = 5;
                c.g[T1][0] = 6;
                c.g[T2][0] = 7;
                c.g[T3][0] = 8;
            },
        );
    }


    #[test]
    fn ustr_sprintf_ascii_fmt_2_matches() {
        check(
            ee::s08::f_002c27c0__generated,
            ustr_sprintf_ascii_fmt_2,
            SEEDS_14,
            &|c, seed| {
                let _ = base_13(c, seed, 0x2c27_c0, 4);
                let buf = SCRATCH + 0x2000;
                let fmt = SCRATCH + 0x3000;
                for i in 0..0x1000u32 {
                    w(c, buf + 4 * i, 0);
                }
                format_fixture(c, seed, buf, fmt);
                c.g[A0][0] = buf as u64;
                c.g[A1][0] = fmt as u64;
                c.g[A2][0] = (seed as u64) << 40 | 3;
                c.g[A3][0] = 4;
                c.g[T0][0] = 5;
                c.g[T1][0] = 6;
                c.g[T2][0] = 7;
                c.g[T3][0] = 8;
            },
        );
    }


    #[test]
    fn cfe_state_create_step2_fill_gender_list_matches() {
        check(
            ee::s03::f_001bb520__generated,
            cfe_state_create_step2_fill_gender_list,
            60,
            &|c, seed| {
                let mut r = base_14(c, seed, 0x1bb5);
                let box_ = DATA + 0x1000;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(box_);
                c.g[A2][0] = (seed % 2) as u64;
                c.bus.write32(box_ + 8, DATA + 0x2000);
                slot_10(c, DATA + 0x2000, 0x48, 1);
                fill(c, &mut r, box_, 0x40);
            },
        );
    }


    #[test]
    fn net_create_room_popup_validate_room_name_matches() {
        check(
            ee::s03::f_001bec78__generated,
            net_create_room_popup_validate_room_name,
            40,
            &|c, seed| {
                let mut r = base_14(c, seed, 0x1bec);
                let name = DATA + 0x1000;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x6dc, name);
                if seed % 2 == 0 {
                    c.bus.write8(name, 0);
                } else {
                    c.bus.write8(name, b'x');
                    c.bus.write8(name + 1, 0);
                }
                c.bus.write32(DATA + 0x100, DATA + 0x3000);
                fill(c, &mut r, DATA + 0x3000, 0x200);
            },
        );
    }


    // Objects and buffers in the test data area (below the fixtures'
    // reserved blocks: BE at +0x30000, WORLD_6 at +0x80000, the fake log at
    // 0x900000 and the pools above +0x400000).
    pub(crate) const OBJ_4: u32 = DATA;

    pub(crate) const OBJ2_4: u32 = DATA + 0x800;

    pub(crate) const OBJ3_2: u32 = DATA + 0x1000;

    pub(crate) const OBJ4_2: u32 = DATA + 0x1800;

    pub(crate) const WORLD_6: u32 = DATA + 0x8_0000;


    /// The BELibrary singleton (`gp - 0x1f04`, zeroed so nothing waits for
    /// the disc) and the world (`gp - 0x848`, a dummy with no riders).
    pub(crate) fn globals_11(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x848), WORLD_6);
        let game = WORLD_6 + 0x1000;
        w(c, WORLD_6 + 0x84, game);
        let race = WORLD_6 + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }


    /// The engine's size-class pool_9 allocator: `gp + 0xdd8` ready plus eight
    /// free nodes per size class.
    pub(crate) fn pool_9(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool_9 = DATA + 0x40_0000 + 0x4_0000 * i;
            let base_15 = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base_15 + 0x400 * n;
                w(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w(c, pool_9 + head, base_15);
            w(c, GAME_GP + slot, pool_9);
        }
    }


    pub(crate) fn base_15(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        globals_11(c);
        pool_9(c);
        r
    }


    // ---------------------------------------------------------------- front end

        #[test]
    fn cfe_online_main_menu_on_disconnect_popup_event_matches() {
        check(
            ee::s03::f_001c0240__generated,
            cfe_online_main_menu_on_disconnect_popup_event,
            6,
            &|c, seed| {
                let _ = base_15(c, seed, 0x001c_0240, 6);
                c.g[A0][0] = OBJ_4 as u64;
                c.g[A1][0] = OBJ2_4 as u64;
                c.g[A2][0] = if seed % 2 == 0 { 0xf } else { (seed % 16) as u64 };
                w(c, OBJ_4 + 0x10, OBJ3_2);
                w(c, OBJ_4 + 0x40, OBJ4_2);
                w(c, OBJ4_2 + 0xd0, OBJ3_2);
                w(c, OBJ3_2 + 0x10, OBJ2_4);
                w(c, GAME_GP.wrapping_add(0x238), OBJ2_4);
                w(c, GAME_GP.wrapping_sub(0xc8), OBJ2_4);
            },
        );
    }


    /// Two arrow widgets whose `v09(enabled)` dispatches through a fake vtable
    /// slot at +0x48.
    pub(crate) fn arrows(c: &mut Cpu, seed: u32, salt: u64, count_off: u32, max_off: u32, up: u32, down: u32) {
        let mut r = base_15(c, seed, salt, 2);
        c.g[A0][0] = OBJ_4 as u64;
        let count = if seed % 4 == 0 { 0 } else { (seed % 5) as u32 };
        w(c, OBJ_4 + count_off, count);
        w(c, OBJ_4 + max_off, if seed % 3 == 0 { 0x40 } else { count + 4 });
        w(c, OBJ_4 + up, OBJ2_4);
        w(c, OBJ_4 + down, OBJ3_2);
        let vt = vtable(c, &mut r, &[0x48]);
        w(c, OBJ2_4 + 8, vt);
        let vt = vtable(c, &mut r, &[0x48]);
        w(c, OBJ3_2 + 8, vt);
    }


    #[test]
    fn cfe_state_net_messenger_highlight_row_matches() {
        check(
            ee::s03::f_001a5fb8__generated,
            cfe_state_net_messenger_highlight_row,
            4,
            &|c, seed| {
                let _ = base_15(c, seed, 0x001a_5fb8, 3);
                c.g[A0][0] = OBJ_4 as u64;
                c.g[A1][0] = (seed % 16) as u64;
                w(c, OBJ_4 + 0x40, OBJ2_4);
                w(c, 0x0046_15b0, 0);
            },
        );
    }


    #[test]
    fn net_fe_state_create_keyboard_popup_matches() {
        check(
            ee::s03::f_001a91b0__generated,
            net_fe_state_create_keyboard_popup,
            4,
            &|c, seed| {
                let _ = base_15(c, seed, 0x001a_91b0, 3);
                c.g[A0][0] = OBJ_4 as u64;
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = (seed % 5) as u64;
                c.g[A3][0] = (seed % 7) as u64;
                c.g[T0][0] = (seed % 9) as u64;
                w(c, OBJ_4 + 0x10, OBJ2_4);
                w(c, OBJ_4 + 0x40, OBJ3_2);
                w(c, OBJ3_2 + 0xd0, OBJ4_2);
                w(c, OBJ4_2 + 0x10, OBJ2_4);
                w(c, OBJ_4 + 0x69c, 0);
            },
        );
    }


    pub(crate) const SEEDS_15: u32 = 120;

    /// The zeroed BELibrary dummy and the world singleton.
    pub(crate) const BE_8: u32 = DATA + 0x1_0000;

    pub(crate) const W_6: u32 = DATA + 0x1_4000;

    /// `{ short delta; void (*fn)(); }` at `vt + off`.
    pub(crate) fn slot_11(c: &mut Cpu, vt: u32, off: u32, target: u32) {
        c.bus.write16(vt + off, 0);
        w(c, vt + off + 4, target);
    }


    pub(crate) fn run_9(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = 0x0012_3450;
        c.pc = addr;
        f(c);
        c.g[V0][0]
    }


    pub(crate) fn heap_init_11(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        run_9(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
    }


    /// The engine's size-class pool allocator, enabled through `gp + 0xdd8`.
    pub(crate) fn pool_allocator_11(c: &mut Cpu) {
        w(c, GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),
            (0x2a60, 0x2000, 0x10),
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot_11, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base_16 = nodes;
            for n in 0..16u32 {
                let node = base_16 + block * n;
                w(c, node, if n == 15 { 0 } else { node + block });
            }
            w(c, pool + head, base_16);
            w(c, GAME_GP + slot_11, pool);
            nodes += block * 16;
        }
    }


    /// A real heap block of `size` bytes.
    pub(crate) fn alloc_2(c: &mut Cpu, size: u32) -> u32 {
        run_9(
            c,
            0x0031_7d70,
            ee::s10::f_00317d70,
            &[size as u64, 0x0048_0000, 0, 0],
        ) as u32
    }


    /// Scrambled registers, a game-like `gp`, the BE_8 singleton and a world
    /// with no riders.
    pub(crate) fn base_16(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        w(c, GAME_GP.wrapping_sub(0x1f04), BE_8);
        w(c, GAME_GP.wrapping_sub(0x1efc), 0);
        w(c, GAME_GP.wrapping_sub(0x848), W_6);
        let game = W_6 + 0x1000;
        w(c, W_6 + 0x84, game);
        let race = W_6 + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
        r
    }


    /// [`base_16`] plus the pool allocator and dlmalloc arena.
    pub(crate) fn alloc_base_4(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = base_16(c, seed, salt, calls);
        eafile_pools(c);
        pool_allocator_11(c);
        heap_init_11(c);
        r
    }


    #[test]
    fn fe_state_create_step2_init_matches() {
        check(ee::s03::f_001ba428__generated, fe_state_create_step2_init, 40, &|c, seed| {
            alloc_base_4(c, seed, 0x1ba4, 5);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0x10, DATA + 0x400);
            w(c, DATA + 0x6d0, DATA + 0x500);
        });
    }


    #[test]
    fn fe_state_challenge_popup_ctor_matches() {
        check(ee::s03::f_001addd0__generated, fe_state_challenge_popup_ctor, 40, &|c, seed| {
            alloc_base_4(c, seed, 0x1add, 5);
            c.g[A0][0] = DATA as u64;
        });
    }


    #[test]
    fn fe_use_existing_account_v25_matches() {
        check(
            ee::s03::f_001ac4f0__generated,
            fe_use_existing_account_v25,
            SEEDS_15,
            &|c, seed| {
                base_16(c, seed, 0x1ac4, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
            },
        );
    }


    #[test]
    fn fe_state_online_character_select_v25_matches() {
        check(
            ee::s03::f_001b14d8__generated,
            fe_state_online_character_select_v25,
            SEEDS_15,
            &|c, seed| {
                base_16(c, seed, 0x1b14, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
            },
        );
    }


    #[test]
    fn fe_state_net_news_v25_matches() {
        check(ee::s03::f_001c5540__generated, fe_state_net_news_v25, SEEDS_15, &|c, seed| {
            base_16(c, seed, 0x1c55, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
        });
    }


    #[test]
    fn fe_state_chat_room_on_keyboard_done_matches() {
        check(
            ee::s03::f_001b33d8__generated,
            fe_state_chat_room_on_keyboard_done,
            SEEDS_15,
            &|c, seed| {
                let mut r = base_16(c, seed, 0x1b33, 2);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0x69c, DATA + 0x800);
                w(c, DATA + 0x800 + 0x18, (seed % 5) as u32);
                // The buddy-manager teardown paths need a live manager whose
                // lists are intact; keep them on the empty path.
                w(c, GAME_GP + 0x238, 0);
                let _ = r.next_u32();
            },
        );
    }


    #[test]
    fn fe_state_chat_room_select_player_by_name_matches() {
        check(
            ee::s03::f_001b3978__generated,
            fe_state_chat_room_select_player_by_name,
            SEEDS_15,
            &|c, seed| {
                let mut r = base_16(c, seed, 0x1b39, 2);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x900) as u64;
                w(c, DATA + 0x40, if seed % 3 == 0 { 0 } else { DATA + 0x800 });
                w(c, GAME_GP.wrapping_sub(0xc8), if seed % 2 == 0 { 0 } else { DATA + 0xa00 });
                w(c, DATA + 0xa00, 0);
                let _ = r.next_u32();
            },
        );
    }


                    #[test]
    fn fe_state_stats_format_time_matches() {
        check(ee::s03::f_001c4f58__generated, fe_state_stats_format_time, 80, &|c, seed| {
            let mut r = Rng(0x001c_4f58 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            elf_image(c);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP - 0x590c, (1.0f32 / 60.0).to_bits());
            c.g[A1][0] = sx(DATA);
            c.g[A2][0] = ((r.next_u32() % 200_000) as i32) as u64;
        });
    }


    /// Compare against the generated body and require *everything* to match:
    /// hard state, caller-saved registers (`soft`) and LO/HI.
    pub(crate) fn strict(g: GuestFn, h: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seeds {
            let r = crate::difftest::compare(g, h, &|c| setup(c, seed));
            assert!(
                r.hard.is_empty() && r.soft.is_empty() && r.lohi.is_empty(),
                "seed {seed}: hard={:#?} soft={:#?} lohi={:#?}",
                r.hard,
                r.soft,
                r.lohi
            );
        }
    }


                #[test]
    fn net_fe_state_on_session_lost_popup_event_matches() {
        strict(ee::s03::f_001a9578__generated, net_fe_state_on_session_lost_popup_event, 40, &|c, seed| {
            let mut r = Rng(0x001a_9578 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP + 0x1d98, if seed % 2 == 0 { 0 } else { 1 });
            c.bus.write32(GAME_GP - 0xc8, DATA + 0x1000);
            c.g[A0][0] = sx(DATA);
            c.g[A2][0] = [0u64, 0xf, 0x14, 3][(seed % 4) as usize];
        });
    }


    #[test]
    fn fe_use_existing_account_on_abort_popup_event_matches() {
        strict(ee::s03::f_001aca88__generated, fe_use_existing_account_on_abort_popup_event, 40, &|c, seed| {
            let mut r = Rng(0x001a_ca88 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP + 0x238, if seed % 2 == 0 { 0 } else { DATA + 0x1000 });
            c.bus.write32(GAME_GP - 0xc8, DATA + 0x1800);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            c.g[A2][0] = [0u64, 2, 0xf, 0x14][(seed % 4) as usize];
        });
    }


    #[test]
    fn fe_state_login_v33_matches() {
        strict(ee::s03::f_001bf378__generated, fe_state_login_v33, 40, &|c, seed| {
            let mut r = Rng(0x001b_f378 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP + 0x238, if seed % 2 == 0 { 0 } else { DATA + 0x1000 });
            c.bus.write32(GAME_GP - 0xc8, DATA + 0x1800);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            c.g[A2][0] = [0u64, 2, 0xf, 0x14][(seed % 4) as usize];
        });
    }


    pub(crate) const SEEDS_16: u32 = 200;


    /// The BELibrary dummy and the world/race-manager/rider graph the
    /// front-end and overlay functions read.
    pub(crate) const BE_9: u32 = DATA + 0x0f_0000;

    pub(crate) const WORLD_7: u32 = DATA + 0x10_0000;

    pub(crate) const RMAN: u32 = DATA + 0x11_0000;

    pub(crate) const RIDERS_2: u32 = DATA + 0x12_0000;

    pub(crate) const RIDER: u32 = DATA + 0x13_0000;

    /// The BELibrary interface table (`.data` at 0x5308a8, indexed by
    /// `gp_18-0x1efc`).
    pub(crate) const IFACE_TABLE: u32 = 0x0053_08a8;


    pub(crate) fn game_globals(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP - 0x1f04, BE_9);
        c.bus.write32(GAME_GP - 0x848, WORLD_7);
        c.bus.write32(WORLD_7 + 0x84, RMAN);
        c.bus.write32(RMAN + 0xc, RIDERS_2);
        c.bus.write32(RIDERS_2 + 0x28, RIDER);
    }


    /// `BELibrary_GetInstance` returns the dummy; `GetInterface(dummy, 1)`
    /// finds an object whose slot 8 is a fake method.
    pub(crate) fn be_library_9(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(GAME_GP - 0x1efc, 0);
        let iface = DATA + 0x14_0000;
        c.bus.write32(IFACE_TABLE, iface);
        let vt = vtable(c, r, &[8]);
        c.bus.write32(iface + 4, vt);
    }


    /// A fake that returns a small value instead of a pointer, keyed by slot.
    pub(crate) fn fake_small(c: &mut Cpu, target: u32) {
        fake_call(c, target);
        let h = c.g[V1][0];
        c.g[V0][0] = match target.wrapping_sub(FAKE) {
            0 => ((h >> 3) & 1) as u64,
            0x10 => ((h >> 5) & 1) as u64,
            0x20 => match (h >> 9) & 3 {
                0 => (h >> 11) & 7,
                1 => sx(0xffff_fffd_u32),
                2 => sx(0xffff_fffe_u32),
                _ => sx(0xffff_fff0_u32),
            },
            0x30 => ((h >> 11) & 1) as u64,
            _ => 0,
        };
    }


    #[test]
    fn cfe_use_existing_account_on_email_popup_event_matches() {
        check(
            ee::s03::f_001ac9c0__generated,
            cfe_use_existing_account_on_email_popup_event,
            SEEDS_16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1ac9, 4);
                game_globals(c);
                let this = DATA;
                let args = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = args as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0xf
                } else {
                    1 + (seed % 14) as u64
                };
                let name = DATA + 0x3000;
                c.bus.write32(args + 0x2b0, name);
                c.bus.write8(name, 0);
                c.bus.write32(args + 0x18, 0);
            },
        );
    }


    #[test]
    fn fe_async_manager_request_session_texture_matches() {
        check(
            ee::s03::f_001a37f8__generated,
            fe_async_manager_request_session_texture,
            SEEDS_16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1a37, 3);
                game_globals(c);
                be_library_9(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = if seed % 3 == 0 {
                    0xff_ffff
                } else {
                    (seed % 6) as u64
                };
                c.g[A2][0] = (seed % 8) as u64;
                c.g[A3][0] = (seed % 5) as u64;
            },
        );
    }


    #[test]
    fn cfe_state_net_config_show_checking_card_popup_matches() {
        check(
            ee::s03::f_001b47c0__generated,
            cfe_state_net_config_show_checking_card_popup,
            SEEDS_16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1b47, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                let mc = DATA + 0x2400;
                c.bus.write32(WORLD_7 + 0x8c, mc);
                let obj = DATA + 0x2800;
                c.bus.write32(mc + 4, obj);
                let vt = vtable(c, &mut r, &[0x20]);
                c.bus.write32(obj, vt);
                c.bus.write32(obj + 0x20, 0);
            },
        );
    }


    #[test]
    fn cfe_state_net_quick_messages_on_save_popup_event_matches() {
        check(
            ee::s03::f_001c2c50__generated,
            cfe_state_net_quick_messages_on_save_popup_event,
            SEEDS_16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1c2c, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = match seed % 3 {
                    0 => 0xf,
                    1 => 0x10,
                    _ => 3,
                };
                c.bus.write32(this + 0x10, DATA + 0x2400);
                let _ = &mut r;
            },
        );
    }

    /// `check`, but a caller-saved difference fails too: the fake-call log lives
    /// in RAM and every GPR is visible to it, so nothing may differ.
    pub(crate) fn check_all(generated: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..crate::difftest::seed_count(seeds) {
            let r = crate::difftest::compare(generated, hand, &|c| setup(c, seed));
            assert!(
                r.hard.is_empty() && r.soft.is_empty() && r.lohi.is_empty(),
                "seed {seed}: hard {:#?} soft {:#?} lohi {:#?}",
                r.hard,
                r.soft,
                r.lohi
            );
        }
    }


        #[test]
    fn cfe_state_net_config_check_card_matches() {
        check_all(
            ee::s03::f_001b43a8__generated,
            cfe_state_net_config_check_card,
            SEEDS_16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1b43, 3);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let obj = DATA + 0x2000;
                c.bus.write32(GAME_GP - 0x1578, obj);
                let vt = vtable(c, &mut r, &[0x1a8, 0x1b8]);
                c.bus.write32(obj, vt);
                c.bus.write32(this + 0x708, 0);
            },
        );
    }


    #[test]
    fn cfe_state_net_quick_play_popup_on_options_popup_event_matches() {
        check_all(
            ee::s03::f_001c1b88__generated,
            cfe_state_net_quick_play_popup_on_options_popup_event,
            SEEDS_16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1c1b, 4);
                game_globals(c);
                let this = DATA;
                let text = DATA + 0x2000;
                let popup_text = DATA + 0x2400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x3000) as u64;
                c.g[A2][0] = match seed % 3 {
                    0 => 0xf,
                    1 => 0x14,
                    _ => 2,
                };
                c.bus.write32(this + 0x6e4, text);
                c.bus.write32(this + 0x6f4, popup_text);
                let vt = vtable(c, &mut r, &[0x48]);
                c.bus.write32(text + 8, vt);
                c.bus.write32(this + 0x6ec, (seed % 2) as u32);
            },
        );
    }


    #[test]
    fn cfe_state_net_config_handle_load_error_matches() {
        check_all(
            ee::s03::f_001b4fa0__generated,
            cfe_state_net_config_handle_load_error,
            SEEDS_16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1b4f, 2);
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = match seed % 8 {
                    0 => sx(0xffff_fff1_u32),
                    1 => sx(0xffff_ff38_u32),
                    2 => sx(0xffff_ffeb_u32),
                    3 => sx(0xffff_ffea_u32),
                    4 => 0,
                    5 => 1,
                    6 => sx(0xffff_ffec_u32),
                    _ => sx(0xffff_ffd8_u32),
                };
                c.bus.write32(this + 0x10, DATA + 0x2000);
                c.bus.write32(GAME_GP - 0x1570, DATA + 0x2400);
                let _ = &mut r;
            },
        );
    }


    pub(crate) const W_7: u32 = DATA + 0x5_0000;

    pub(crate) const RM_3: u32 = DATA + 0x5_1000;

    pub(crate) const BE_10: u32 = DATA + 0x5_2000;


    /// A dummy world (`gp-0x848`), its rider manager, and the BELibrary
    /// singleton (`gp-0x1f04`).
    pub(crate) fn base_17(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        c.bus.write32(gp_at(-0x1f04), BE_10);
        c.bus.write32(gp_at(-0x848), W_7);
        c.bus.write32(W_7 + 0x7c, RM_3);
        c.bus.write32(W_7 + 0x84, RM_3);
        c.bus.write32(RM_3 + 0x84, RM_3);
        c.bus.write32(RM_3 + 0x10, 0);
        r
    }


    #[test]
    fn cfe_state_net_messenger_get_buddy_status_matches() {
        check(
            ee::s03::f_001a4670__generated,
            cfe_state_net_messenger_get_buddy_status,
            80,
            &|c, seed| {
                let _r = base_17(c, seed, 0x1a46);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(gp_at(0x238), DATA + 0x1000);
                c.bus.write32(gp_at(-0xc8), DATA + 0x1100);
            },
        );
    }


    #[test]
    fn cfe_state_net_account_mem_update_state06_matches() {
        check(
            ee::s03::f_001b7820__generated,
            cfe_state_net_account_mem_update_state06,
            60,
            &|c, seed| {
                let _r = base_17(c, seed, 0x1b78);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(gp_at(0x2e0), DATA + 0x1000);
                // Neither 4 nor 5: only the mode test runs.
                c.bus.write32(DATA + 0x1000, 6 + (seed % 3));
                c.bus.write32(DATA + 0x6d4, seed % 7);
            },
        );
    }


    #[test]
    fn cfe_state_net_account_mem_update_state0c_matches() {
        check(
            ee::s03::f_001b7938__generated,
            cfe_state_net_account_mem_update_state0c,
            60,
            &|c, seed| {
                let _r = base_17(c, seed, 0x1b79);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(gp_at(0x2e0), DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, 6 + (seed % 3));
                c.bus.write32(DATA + 0x6d4, seed % 7);
            },
        );
    }

    /// The dummy world.
    pub(crate) const WORLD_8: u32 = DATA + 0x8_0000;


    pub(crate) fn put_str_6(c: &mut Cpu, a: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(a + i as u32, *b);
        }
        c.bus.write8(a + s.len() as u32, 0);
    }


    /// The BELibrary singleton (`gp - 0x1f04`, a zeroed dummy so nothing reads
    /// the disc) and the world (`gp - 0x848`, with no riders).
    pub(crate) fn globals_12(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x848), WORLD_8);
        let game = WORLD_8 + 0x1000;
        w(c, WORLD_8 + 0x84, game);
        let race = WORLD_8 + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }


    /// The UI manager singleton (`gp - 0xc8`, `D_004A3028`).
    pub(crate) fn ui(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0xc8), OBJ);
    }


    /// `DLHeap_InitArena(&MEM_HEAP, arena, size)` over a test arena.
    pub(crate) fn heap_init_12(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const RET: u32 = 0xdead_0100;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, RET, "DLHeap_InitArena did not return");
    }


    /// The engine's size-class pool_10 allocator: `gp + 0xdd8` ready plus eight
    /// free nodes per size class.
    pub(crate) fn pool_10(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pl = DATA + 0x40_0000 + 0x4_0000 * i;
            let base = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base + 0x400 * n;
                w(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w(c, pl + head, base);
            w(c, GAME_GP.wrapping_add(slot), pl);
        }
    }


    /// `StrHash(s)` (0x317670) on the string at `a`, to build name-hash
    /// fixtures.
    pub(crate) fn str_hash_2(c: &mut Cpu, a: u32) -> u32 {
        const RET: u32 = 0xdead_0200;
        c.g[A0][0] = sx(a);
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_7670;
        ee::s10::StrHash(c);
        assert_eq!(c.pc, RET, "StrUHash did not return");
        c.g[V0][0] as u32
    }


    /// Assert no differences at all (hard, caller-saved and LO/HI).
    pub(crate) fn diff(gen: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seed_count(seeds) {
            let r = compare(gen, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} (caller-saved): {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} (LO/HI): {:#?}", r.lohi);
        }
    }


        #[test]
    fn net_config_auth_popup_ctor_matches() {
        diff(ee::s03::f_001b6288__generated, net_config_auth_popup_ctor, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x1b_6288, 0);
            globals_12(c);
            ui(c);
            heap_init_12(c);
            pool_10(c);
            // `USTR_length` goes through the current UI object's vtable; a zero
            // vtable pointer makes that call a fake.
            w(c, WORLD_8 + 0x8c, DATA + 0x3000);
            w(c, DATA + 0x3004, 0);
            put_str_6(c, DATA + 0x300, b"config");
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(r.next_u32());
            c.g[A2][0] = sx(r.next_u32());
            w(c, DATA + 0x6fc, 0);
            w(c, DATA + 0x6dc, r.next_u32());
        });
    }


    #[test]
    fn auth_dnas_error_on_widget_created_matches() {
        diff(
            ee::s03::f_001b6bb8__generated,
            c_fe_state_auth_dnas_error_on_widget_created,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1b_6bb8, 0);
                globals_12(c);
                put_str_6(c, dnas_error::STATUS, b"status1");
                put_str_6(c, dnas_error::STATUS2, b"status2");
                put_str_6(c, dnas_error::STATUS3, b"status3");
                put_str_6(c, dnas_error::RETRY, b"retry");
                put_str_6(c, dnas_error::MESSAGE, b"message");
                let obj = DATA + 0x2000;
                let hash = match seed % 6 {
                    0 => str_hash_2(c, dnas_error::STATUS),
                    1 => str_hash_2(c, dnas_error::STATUS2),
                    2 => str_hash_2(c, dnas_error::STATUS3),
                    3 => str_hash_2(c, dnas_error::RETRY),
                    4 => str_hash_2(c, dnas_error::MESSAGE),
                    _ => r.next_u32(),
                };
                w(c, obj + 0x38, hash);
                w(c, obj + 8, 0);
                w(c, obj + 0x90, r.next_u32());
                w(c, DATA + 0x6dc, 0);
                w(c, DATA + 0x6e0, 0);
                w(c, DATA + 0x6e4, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(obj);
            },
        );
    }


    /// A scratch world/UI object graph used by the front-end tests.
    pub(crate) fn fe_base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        globals_12(c);
        ui(c);
        heap_init_12(c);
        pool_10(c);
        r
    }


    #[test]
    fn lobby_menu_enter_room_matches() {
        diff(ee::s03::f_001bd460__generated, c_fe_state_lobby_menu_enter_room, 40, &|c, seed| {
            let _r = fe_base(c, seed, 0x1b_d460);
            let s = if seed % 3 == 0 { 0 } else { DATA + 0x300 };
            if s != 0 {
                put_str_6(c, s, b"room name");
            }
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = sx(s);
            w(c, DATA + 0x770, 0);
            w(c, DATA + 0xb80, 0);
        });
    }


    #[test]
    fn online_main_menu_on_leave_popup_event_matches() {
        diff(
            ee::s03::f_001c0138__generated,
            c_fe_state_online_main_menu_on_leave_popup_event,
            40,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x1c_0138);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = if seed % 5 == 0 { 0xf } else { (seed % 0x10) as u64 };
                w(c, DATA + 0x10, r.next_u32());
                w(c, DATA + 0x6b0, 0);
                w(c, DATA + 0x2000, r.next_u32());
                w(c, GAME_GP.wrapping_add(0x238), if seed % 3 == 0 { 0 } else { DATA + 0x3000 });
            },
        );
    }


    #[test]
    fn net_fe_state_v32_matches() {
        diff(ee::s03::f_001a8830__generated, net_fe_state_v32, 20, &|c, seed| {
            let mut r = fe_base(c, seed, 0x1a_8830);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 3) as u64;
            w(c, DATA + 8, 0);
            w(c, DATA + 0x3000, r.next_u32());
            w(c, GAME_GP.wrapping_add(0x238), if seed % 2 == 0 { 0 } else { DATA + 0x3000 });
        });
    }


    #[test]
    fn net_fe_state_show_disconnected_popup_matches() {
        diff(
            ee::s03::f_001a9930__generated,
            net_fe_state_show_disconnected_popup,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x1a_9930);
                c.g[A0][0] = sx(DATA);
                w(c, DATA + 0x6ac, if seed % 2 == 0 { 0 } else { (r.next_u32() | 1) });
                w(c, DATA + 0x6b0, 0);
                w(c, DATA + 0x6dc, 0);
            },
        );
    }


    pub(crate) const SEEDS_17: u32 = 120;


    /// Every compared state must match: hard, caller-saved (`soft`) and LO/HI.
    pub(crate) fn strict_2(g: GuestFn, h: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seeds {
            let r = compare(g, h, &|c| setup(c, seed));
            assert!(
                r.hard.is_empty() && r.soft.is_empty() && r.lohi.is_empty(),
                "seed {seed}: hard={:#?} soft={:#?} lohi={:#?}",
                r.hard,
                r.soft,
                r.lohi
            );
        }
    }


    /// The `BELibrary` singleton (`gp - 0x1f04`) and the world (`gp - 0x848`).
    pub(crate) fn globals_13(c: &mut Cpu) {
        let be = DATA + 0x3_0000;
        let world = DATA + 0x8_0000;
        w(c, GAME_GP.wrapping_sub(0x1f04), be);
        w(c, GAME_GP.wrapping_sub(0x848), world);
        let game = world + 0x1000;
        w(c, world + 0x84, game);
        let race = world + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }


    /// The engine's size-class pool_11 allocator behind `gp + 0xdd8`.
    pub(crate) fn pool_11(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x32000),
            (0x2a7c, 0x25800),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w(c, GAME_GP.wrapping_add(0xdd8), 1);
        let nodes = DATA + 0x28_000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool_11 = DATA + 0x20_000 + 0x100 * i as u32;
            let base_18 = nodes + 0x400 * i as u32;
            for n in 0..8u32 {
                let node = base_18 + 0x40 * n;
                w(c, node, if n == 7 { 0 } else { node + 0x40 });
            }
            w(c, pool_11 + head, base_18);
            w(c, GAME_GP + slot, pool_11);
        }
    }


    /// Scrambled state, the fake call hooks, `gp`, the globals_13 and the pools.
    pub(crate) fn base_18(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        globals_13(c);
        pool_11(c);
        r
    }


    #[test]
    fn cfestate_lobby_menu_transition_in_matches() {
        strict_2(
            ssx3_ee::s03::f_001bcd18__generated,
            cfestate_lobby_menu_transition_in,
            SEEDS_17,
            &|c, seed| {
                let mut r = base_18(c, seed, 0xbcd18);
                let this = DATA + 0x1000;
                let vt = DATA + 0x2000;
                c.bus.write16(vt + 0x100, 0);
                w(c, vt + 0x104, FAKE);
                w(c, this + 8, vt);
                let reset = seed % 2 == 1;
                // Reset runs the vtable method (a fake); stop there. The
                // other half takes the "no room chosen" early return.
                w(c, this + 0x760, if reset { 1 } else { 0 });
                w(c, this + 0x764, r.next_u32() % 4);
                w(c, this + 0x768, r.next_u32() % 4);
                w(c, this + 0x6d4, DATA + 0x3000);
                for i in 0..4 {
                    w(c, this + 0x708 + 4 * i, DATA + 0x4000 + 0x100 * i);
                }
                w(c, this + 0x718, 0);
                w(c, DATA + 0x4000 + 0x318, 0);
                c.bus.write32(LOG_UNWIND_AT, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(if reset { 1 } else { 0 });
            },
        );
    }


    /// `check`, but a caller-saved difference fails too: scratch registers are
    /// visible to fake callees, so nothing may differ.
    pub(crate) fn check_all_2(
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        for seed in 0..crate::difftest::seed_count(seeds) {
            let r = crate::difftest::compare(generated, hand, &|c| setup(c, seed));
            assert!(
                r.hard.is_empty() && r.soft.is_empty() && r.lohi.is_empty(),
                "seed {seed}: hard {:#?} soft {:#?} lohi {:#?}",
                r.hard,
                r.soft,
                r.lohi
            );
        }
    }


    /// Write the one-character string `"A"` (hash 0x41) at a constant-name
    /// address so a `StrHash` comparison can match a name.
    pub(crate) fn name_a(c: &mut Cpu, addr: u32) {
        c.bus.write8(addr, b'A');
        c.bus.write8(addr + 1, 0);
    }


    #[test]
    fn cfe_state_create_step2_on_widget_created_matches() {
        const HASHES: [u32; 5] = [
            0x0046_4a08,
            0x004a_1bd8,
            0x004a_1be0,
            0x004a_1be8,
            0x0046_4a18,
        ];
        check_all_2(
            ee::s03::f_001ba500__generated,
            cfe_state_create_step2_on_widget_created,
            120,
            &|c, seed| {
                machine(c, seed, 0x1ba5, 0);
                let widget = DATA + 0x400;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = widget as u64;
                let k = seed % 6;
                if k < 5 {
                    name_a(c, HASHES[k as usize]);
                    c.bus.write32(widget + 0x38, 0x41);
                } else {
                    // No list matches: the widget name is something else.
                    c.bus.write32(widget + 0x38, 1);
                }
                // The list-box fillers read their widget; leave everything at
                // zero except the name so they take their short paths.
                c.bus.write32(widget + 0x8, 0);
                c.bus.write32(widget + 0x40, 0);
            },
        );
    }


            /// The stats-column fixture: a self object, a panel with an old focus
    /// widget, five column items with vtables of fakes, and a chosen column.
    pub(crate) fn stats_fixture(c: &mut Cpu, seed: u32, salt: u64, forward: bool) -> u64 {
        let mut r = machine(c, seed, salt, 0);
        let this_ = DATA;
        let panel = DATA + 0x100;
        c.bus.write32(this_ + 0x10, panel);
        if seed % 2 == 0 {
            let old = DATA + 0x200;
            c.bus.write32(panel + 0x14, old);
            let vt = vtable(c, &mut r, &[0x10]);
            c.bus.write32(old, vt);
        }
        for i in 0..5u32 {
            let obj = DATA + 0x1000 + 0x100 * i;
            c.bus.write32(this_ + 0x6d0 + 4 * i, obj);
            let vt0 = vtable(c, &mut r, &[0x38]);
            c.bus.write32(obj + 8, vt0);
        }
        // The column the function lands on after its increment/decrement is
        // the one whose focus flag matters.
        let before = seed % 3;
        let after = if forward {
            (before + 1) % 4
        } else {
            (before + 3) % 4
        };
        c.bus.write32(this_ + 0x6e4, before);
        let flag = if seed % 3 == 0 { 0x40u32 } else { 0 };
        c.bus.write32(DATA + 0x1000 + 0x100 * after + 0x14, flag);
        this_ as u64
    }


    #[test]
    fn net_quick_play_popup_set_options_enabled_matches() {
        check3(ee::s03::f_001c17f8__generated, net_quick_play_popup_set_options_enabled, 40, &|c, seed| {
            let mut r = base_19(c, seed, 0x1c17f8);
            let obj = DATA;
            for i in 0..15u32 {
                let w = DATA + 0x2000 + 0x40 * i;
                c.bus.write32(obj + 0x6f8 + 4 * i, w);
                c.bus.write32(w + 8, w + 0x20);
                c.bus.write16(w + 0x20 + 0x40, 0);
                c.bus.write32(w + 0x20 + 0x44, LEAF);
            }
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        });
    }


    pub(crate) const DAY_SELF: u32 = DATA + 0x1_0000;

    pub(crate) const DAY_BOX: u32 = DATA + 0x1_1000;

    pub(crate) const DAY_SEL: u32 = DATA + 0x1_2000;


    #[test]
    fn c_fe_state_create_step2_fill_day_list_matches() {
        check_all_3(
            ee::s03::f_001bb840__generated,
            c_fe_state_create_step2_fill_day_list,
            20,
            &|c, seed| {
                let mut r = base_20(c, seed, 0x1bb8_40, 10);
                c.g[A0][0] = sx(DAY_SELF);
                c.g[A1][0] = sx(DAY_BOX);
                c.g[A2][0] = sx(seed % 12); // month
                c.g[A3][0] = sx(2003 + (seed % 3)); // year
                c.bus.write32(DAY_SELF + 0x6d0, DAY_SEL);
                c.bus.write32(DAY_SEL + 0x2c, seed % 40);
                c.bus.write32(DAY_BOX + 0x14, r.next_u32());
                let _ = &mut r;
            },
        );
    }
}
