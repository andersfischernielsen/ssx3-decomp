//! `src/util/menu.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::ai::gp as gp_2;
pub(crate) use crate::game::ai::computer::lh_2;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::dirtysock::tags2::lw;
pub(crate) use crate::game::dirtysock::tags2::p;
pub(crate) use crate::game::fe::feasyncfile::at as at_3;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_4;
pub(crate) use crate::game::fe::feasyncfile::gp_3;
pub(crate) use crate::game::fe::feasyncfile::gp_7 as gp_4;
pub(crate) use crate::game::fe::feasyncfile::lh;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2 as lw_at;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_3;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw_2;
pub(crate) use crate::game::fe::fepopup::at as at_5;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lq;
pub(crate) use crate::game::fe::fepopup::sq;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::festateloadhint::sdsp;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::main::gamemode::gpo;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at_2;
pub(crate) use crate::game::main::gamestate::vslot;
pub(crate) use crate::game::movie::movieplayer::at as at_2;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::replay::replay2::destroy_if_flag;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::visualfx::boardtrailfx::gp;
pub(crate) use crate::game::visualfx::worldlightman::deleting_dtor;
pub(crate) use crate::game::visualfx::worldlightman::DELETE;
pub(crate) use crate::game::wscript::wscriptman::sw;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
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
    (0x002c_a258, menu_sys_item_ctor, "MenuSysItem_ctor"),
    (0x002c_a280, menu_sys_item_dtor, "MenuSysItem_dtor"),
    (
        0x002c_a2b0,
        menu_sys_item_handle_input,
        "MenuSysItem_HandleInput",
    ),
    (
        0x002c_a320,
        menu_sys_item_is_selectable,
        "MenuSysItem_IsSelectable",
    ),
    (
        0x002c_a378,
        menu_sys_item_is_visible,
        "MenuSysItem_IsVisible",
    ),
    (
        0x002c_a388,
        menu_sys_item_set_visible,
        "MenuSysItem_SetVisible",
    ),
    (
        0x002c_a3b0,
        menu_sys_item_is_disabled,
        "MenuSysItem_IsDisabled",
    ),
    (
        0x002c_a3e8,
        menu_sys_item_get_input_code,
        "MenuSysItem_GetInputCode",
    ),
    (0x002c_aa58, menu_sys_page_ctor, "MenuSysPage_ctor"),
    (0x002c_aa80, menu_sys_page_dtor, "MenuSysPage_dtor"),
    (
        0x002c_aab0,
        menu_sys_page_validate_selection,
        "MenuSysPage_ValidateSelection",
    ),
    (
        0x002c_ab08,
        menu_sys_page_select_next,
        "MenuSysPage_SelectNext",
    ),
    (
        0x002c_abe0,
        menu_sys_page_select_prev,
        "MenuSysPage_SelectPrev",
    ),
    (
        0x002c_acc0,
        menusyspage_handle_input,
        "MenuSysPage_HandleInput",
    ),
    (
        0x002c_b180,
        menu_sys_page_begin_draw,
        "MenuSysPage_BeginDraw",
    ),
    (0x002c_b2f8, menu_sys_page_end_draw, "MenuSysPage_EndDraw"),
    (0x002c_b350, menu_sys_build_quad, "MenuSys_BuildQuad"),
    (
        0x002c_b880,
        menu_sys_page_set_row_color,
        "MenuSysPage_SetRowColor",
    ),
    (
        0x002c_bcc8,
        menusyspage_insert_item,
        "MenuSysPage_InsertItem",
    ),
    (
        0x002c_be10,
        menu_sys_page_remove_all_items,
        "MenuSysPage_RemoveAllItems",
    ),
    (
        0x002c_be68,
        menu_sys_page_set_selection,
        "MenuSysPage_SetSelection",
    ),
    (
        0x002c_bf08,
        menu_sys_page_get_selected_index,
        "MenuSysPage_GetSelectedIndex",
    ),
    (
        0x002c_bf30,
        menu_sys_page_get_selected_item,
        "MenuSysPage_GetSelectedItem",
    ),
    (
        0x002c_bf60,
        menu_sys_page_on_activate,
        "MenuSysPage_OnActivate",
    ),
    (
        0x002c_bfd0,
        menu_sys_page_on_deactivate,
        "MenuSysPage_OnDeactivate",
    ),
    (0x002c_c048, menu_sys_dtor, "MenuSys_dtor"),
    (0x002c_c070, menu_sys_reset, "MenuSys_Reset"),
    (0x002c_c0c0, menu_sys_update, "MenuSys_Update"),
    (0x002c_c350, menu_sys_draw, "MenuSys_Draw"),
    (0x002c_c3b8, menu_sys_push_page, "MenuSys_PushPage"),
    (0x002c_c460, menu_sys_pop_page, "MenuSys_PopPage"),
    (
        0x002c_c578,
        menu_sys_bool_item_handle_input,
        "MenuSysBoolItem_HandleInput",
    ),
    (0x002c_c758, menu_sys_bool_item_draw, "MenuSysBoolItem_Draw"),
    (0x002c_cc38, menu_sys_enum_item_draw, "MenuSysEnumItem_Draw"),
    (
        0x002c_ccc8,
        menu_sys_submenu_item_ctor,
        "MenuSysSubmenuItem_ctor",
    ),
    (
        0x002c_cd20,
        menu_sys_submenu_item_handle_input,
        "MenuSysSubmenuItem_HandleInput",
    ),
    (
        0x002c_cd90,
        menu_sys_submenu_item_measure,
        "MenuSysSubmenuItem_Measure",
    ),
    (0x002c_cdf0, menu_sys_back_item_ctor, "MenuSysBackItem_ctor"),
    (
        0x002c_ce38,
        menu_sys_back_item_handle_input,
        "MenuSysBackItem_HandleInput",
    ),
    (
        0x002c_cea8,
        menu_sys_back_item_measure,
        "MenuSysBackItem_Measure",
    ),
    (
        0x002c_cf08,
        menu_sys_action_item_ctor,
        "MenuSysActionItem_ctor",
    ),
    (
        0x002c_cf98,
        menu_sys_action_item_ctor_3,
        "MenuSysActionItem_ctor_3",
    ),
    (
        0x002c_d008,
        menusysactionitem_handle_input,
        "MenuSysActionItem_HandleInput",
    ),
    (
        0x002c_d090,
        menu_sys_action_item_measure,
        "MenuSysActionItem_Measure",
    ),
    (
        0x002c_d0e8,
        menu_sys_action_item_draw,
        "MenuSysActionItem_Draw",
    ),
    (
        0x002c_d1d0,
        menu_sys_callback_item_handle_input,
        "MenuSysCallbackItem_HandleInput",
    ),
    (
        0x002c_d240,
        menu_sys_callback_item_measure,
        "MenuSysCallbackItem_Measure",
    ),
    (
        0x002c_d2f0,
        menu_sys_label_item_measure,
        "MenuSysLabelItem_Measure",
    ),
    (
        0x002c_d3a0,
        menu_sys_spacer_item_measure,
        "MenuSysSpacerItem_Measure",
    ),
    (
        0x002c_d578,
        menu_sys_int8_item_reset_value,
        "MenuSysInt8Item_ResetValue",
    ),
    (
        0x002c_d7b8,
        menu_sys_int8_item_measure,
        "MenuSysInt8Item_Measure",
    ),
    (0x002c_d7f0, menu_sys_int8_item_draw, "MenuSysInt8Item_Draw"),
    (
        0x002c_d8f8,
        menu_sys_int_item_ctor_3,
        "MenuSysIntItem_ctor_3",
    ),
    (
        0x002c_d9b8,
        menu_sys_int_item_reset_value,
        "MenuSysIntItem_ResetValue",
    ),
    (
        0x002c_dbf0,
        menu_sys_int_item_measure,
        "MenuSysIntItem_Measure",
    ),
    (0x002c_dc28, menu_sys_int_item_draw, "MenuSysIntItem_Draw"),
    (
        0x002c_dce8,
        menu_sys_float_item_ctor_2,
        "MenuSysFloatItem_ctor_2",
    ),
    (
        0x002c_df78,
        menu_sys_float_item_reset_value,
        "MenuSysFloatItem_ResetValue",
    ),
    (
        0x002c_dfa0,
        menu_sys_float_item_measure,
        "MenuSysFloatItem_Measure",
    ),
    (
        0x002c_e100,
        menu_sys_float_item_draw,
        "MenuSysFloatItem_Draw",
    ),
    (
        0x002c_e368,
        menu_sys_float_range_item_measure,
        "MenuSysFloatRangeItem_Measure",
    ),
    (
        0x002c_e3a0,
        menu_sys_float_range_item_draw,
        "MenuSysFloatRangeItem_Draw",
    ),
    (
        0x002c_e418,
        menu_sys_text_edit_item_ctor,
        "MenuSysTextEditItem_ctor",
    ),
    (
        0x002c_ed20,
        menu_sys_color_channel_item_ctor,
        "MenuSysColorChannelItem_ctor",
    ),
    (0x002c_f8d8, menu_sys_group_item_v08, "MenuSysGroupItem_v08"),
    (0x002c_fb28, menu_sys_group_item_v10, "MenuSysGroupItem_v10"),
    (
        0x002c_fd28,
        menu_sys_group_item_draw,
        "MenuSysGroupItem_Draw",
    ),
    (
        0x002c_fe78,
        menu_sys_group_item_on_focus,
        "MenuSysGroupItem_OnFocus",
    ),
    (
        0x002c_ff28,
        menu_sys_color_swatch_item_ctor,
        "MenuSysColorSwatchItem_ctor",
    ),
    (
        0x002c_ff88,
        menu_sys_color_swatch_item_measure,
        "MenuSysColorSwatchItem_Measure",
    ),
    (
        0x002d_0450,
        menu_sys_vector_label_item_measure,
        "MenuSysVectorLabelItem_Measure",
    ),
    (
        0x002d_0500,
        menu_sys_vector_label_item_draw,
        "MenuSysVectorLabelItem_Draw",
    ),
    (
        0x002d_08e0,
        menu_sys_point2d_item_on_focus,
        "MenuSysPoint2DItem_OnFocus",
    ),
    (
        0x002d_0d48,
        menu_sys_point2d_item_measure,
        "MenuSysPoint2DItem_Measure",
    ),
    (
        0x002d_19b8,
        wscript_man_obj_trigger_hook,
        "WScriptMan_ObjTriggerHook",
    ),
    (
        0x002d_1a30,
        wscript_man_try_run_obj_script3,
        "WScriptMan_TryRunObjScript3",
    ),
    (
        0x002d_1ac8,
        wscript_man_run_obj_script5_hook,
        "WScriptMan_RunObjScript5Hook",
    ),
    (0x002d_1af0, world_get_rider_count, "World_GetRiderCount"),
    (0x002d_1b08, world_get_rider, "World_GetRider"),
    (0x002d_1b30, mod_obj_call_slot7, "ModObj_CallSlot7"),
    (
        0x002d_1b58,
        world_get_rider_position,
        "World_GetRiderPosition",
    ),
    (
        0x002d_1c98,
        world_get_ai_frame_count,
        "World_GetAIFrameCount",
    ),
    (0x002d_1cb0, world_get_flag_manager, "World_GetFlagManager"),
];

/// Same-shard direct callees used above.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::camera::camera::callee::BEBAGT_RESET_PROFILE;
    pub(crate) use crate::game::camera::camera::callee::BENEWPLAYER_GET_SLOT;
    pub(crate) use crate::game::camera::camera::callee::BE_LIBRARY_RESET_PROFILE;
    pub(crate) use crate::game::camera::camera::callee::BE_OPTION_GET_MAX_HANDICAP;
    pub(crate) use crate::game::camera::camera::callee::CAMERA_TRIGGER_FREE_TRIGGERS;
    pub(crate) use crate::game::camera::camera::callee::CAMERA_TRIGGER_RESET_HEADER;
    pub(crate) use crate::game::camera::camera::callee::CAM_CONTROLLER_RESET;
    pub(crate) use crate::game::camera::camera::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::camera::camera::callee::CHASE_CAM_SET_ALGORITHM_2 as CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::camera::camera::callee::FE_REFRESH_PROFILE_LIST;
    pub(crate) use crate::game::camera::camera::callee::FE_REFRESH_SLOTS;
    pub(crate) use crate::game::camera::camera::callee::MANUAL_CAM_ALGO_BASE_CTOR;
    pub(crate) use crate::game::camera::camera::callee::MEASURE_ROW;
    pub(crate) use crate::game::camera::camera::callee::MEMCARD_SET_STATE;
    pub(crate) use crate::game::camera::camera::callee::SPHERE_TREE_RESET_TRANSFORM;
    pub(crate) use crate::game::camera::camera::callee::TRANSITION_MAN_START;
    pub(crate) use crate::game::camera::camera::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::camera::camera::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::camera::camera::callee::WSCRIPT_DELETE_ALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_BAKE_GROUP;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_EXPORT_HISTORY_FILES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_FREE_HISTORIES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_LOAD_HISTORY_FILE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE as BE_LIBRARY_GET_INSTANCE_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_REWARD_INT_IS_UNLOCKED;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_DESTROY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_SET_PRESENCE_ADDR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY_2 as CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_BX_STRING_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_LUNO_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_GET_SAVE_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_LIST_DIR_ALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_READ_SAVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEMSET;
    pub(crate) use crate::game::dirtysock::tags2::callee::EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT_3 as LOBBY_LIST_GET_AT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_END_OPERATION_2 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR_2 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE_2 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::ON_QUIT_POPUP_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::PATH_PLAT_REFRESH_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_ADD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RECORD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCE_MC_FORMAT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_OBJ_TABLE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_OBJ_CAM_SET_OWNER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_PUSH_FREE_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SLOGIN_MC_GET_PORT_SLOT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY_2 as SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLINK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_2 as SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNOW_BOX_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::dirtysock::tags2::callee::STATS_SELECT_CATEGORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY as STRCPY_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::STREAM_BUFFER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::TROPHY_ROOM_RELEASE_IMAGES;
    pub(crate) use crate::game::dirtysock::tags2::callee::UIMENU_SET_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_GET_FONT;
    pub(crate) use crate::game::fe::feasyncfile::callee::ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_TRY_EXIT_2 as APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATOI;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_IS_TRACK_GROUP0_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_RESET_RACE_RESULT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STORE_RACE_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_DECODE_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_AUDIO_DONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_FRAME_DUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_ALL_TIMERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_PAUSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_BACKUP_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COLLECTED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COMPLETED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_DISPLAY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE_2 as BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_ACQUIRE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_GET_NEXT_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_GET_PREV_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_NODE_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILD_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_NET_CONFIG_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHALLENGE_POPUP_SET_SETTINGS_LOCKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_GROUP_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_POPUP_CONFIRM_EMPTY_INIT_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEAD_NODE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_GET_PATH_SIZE_CHECKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_LOAD_FILE_INTO;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_ALLOC_NAMED_5 as EA_MEM_ALLOC_NAMED_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_5 as EA_MEM_FREE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_GET_RIDER_PREVIEW_2 as FEMODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_ABORT_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_EMAIL_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_LOGIN_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_LOAD_UPDATE_PLAYER_RANKING;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_LOAD_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GRAPHICS_MAN_FREE_ALL_TEXTURES;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_SKIP_WHITESPACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUTSYNC_IS_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_FIRE_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_NEXT_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_INSTANCE_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_PAUSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_HEAP_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_SPAN_TRACK_ACTIVATE_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_SPAN_TRACK_NEXT_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_FIND_ENABLED_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::LIGHT_MAN_EMPTY_HOOK_D;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_INDICATOR_ADVANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_INDICATOR_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_GET_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_ON_CARD_SCANNED;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_UPDATE_MODE1;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_UPDATE_STATES;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_CARD_STATE_BASE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_DISABLED_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS_2 as MPEG2_GET_BITS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT_2 as MPEG2_MARKER_BIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_FADE_OUT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_PLAY_SONG;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_STOP_SONG_2 as MUSIC_PLAYER_STOP_SONG;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_ACCOUNT_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_DISCONNECT_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_11 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_4 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_4 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_12 as NET_FE_STATE_ON_CHILD_EVENT_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_4 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_3 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_ADVERT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_MESSAGE_CENTER_REFRESH_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_MESSAGE_CENTER_UPDATE_COUNT_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_FIND_BANK_SLOT as PATH_PLAT_FIND_BANK_SLOT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID_2 as POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_GET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_GRID_MESH_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::REFRESH_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_CACHE_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARDS_FILE_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_ADD_WIPEOUT_RECOVERY_BOOST;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_BASE_RESET_FX;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_BUILD_WORLD_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_CHECK_RESET_PATH;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_ON_RIDER_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SAY_20D3;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SAY_20D4;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_UPDATE_BOARD_LEAN;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_IS_FULL;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ACTIVATE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ADD_SCRIPT_OBJ;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ALLOC_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CLEAR_SKIP_DELAY_ON_EXIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_POP_FREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_PUSH_FREE_NODE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_START_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ADD_CAM;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_SET_TRANSITION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SETUP_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SET_CATEGORY_TITLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SHOW_BUY_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SHOW_EQUIP_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_FLUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_TREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_GET_PATH_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_7 as SPRINTF_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::START_SCREEN_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH_5 as STR_HASH_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::TAG_FIELD_GET_BINARY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_GET_ACTIVE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_START_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_3 as UI_LIST_DELETE_ALL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_4 as UI_LIST_NODE_DTOR_BODY_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_3 as UI_OBJECT_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_FIND_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_CPY_2 as USTR_CPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_LEN_2 as USTR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_DEACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_WRITE;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_SAVE_DATA_SIZE;
    pub(crate) use crate::game::fe::festateload::callee::BOLT_FILE_CACHE_INIT;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SELECT_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SETUP_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::EA_MEM_FREE;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_UPDATE;
    pub(crate) use crate::game::fe::festateload::callee::I_SIGNAL_SEMA;
    pub(crate) use crate::game::fe::festateload::callee::MEM_CARD_BASE_DTOR;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V42;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V43;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_HANDLE_EVENT;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_METRIX_SET_SPIN_AMOUNT;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_PREVIEW_APPLY_TRANSFORM;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_PAUSE;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_RESUME;
    pub(crate) use crate::game::fe::festateload::callee::STRLEN_COPY;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::UI_STATE_V13;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_TRIGGER_MANAGER_DTOR;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_CACHE_SET_DATA;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_MAN_RESET_COMMAND_QUEUE;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_GOTO_TASK;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_JUMP_TO_TASK;
    pub(crate) use crate::game::fe::festaterewards::callee::AI_VISUAL_EFFECTS_MAN_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::CUI_TEXT_SCROLL_SET_NAV_BUTTONS;
    pub(crate) use crate::game::fe::festaterewards::callee::FE_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::fe::festaterewards::callee::IN_SEQ_INSTANCE_SETUP;
    pub(crate) use crate::game::fe::festaterewards::callee::LIGHT_MAN_EMPTY_HOOK;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_FORMAT_FLOAT;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_DRAW_ROW;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_PUSH_PAGE;
    pub(crate) use crate::game::fe::festaterewards::callee::REWARD_GALLERY_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_BINDING_CLEAR_LISTENER;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_LIST_ON_SEQ_START;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_LIST_ON_SEQ_STOPPED;
    pub(crate) use crate::game::fe::festaterewards::callee::SEQ_LISTENER_ON_START;
    pub(crate) use crate::game::fe::festaterewards::callee::SEQ_LISTENER_ON_STOP;
    pub(crate) use crate::game::fe::festaterewards::callee::UI_LIST_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MEMCARD_SJIS_CHAR_TO_ASCII;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MENU_SYS_ITEM_CTOR as MENU_SYS_ITEM_CTOR_3;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MENU_SYS_ITEM_DRAW_ROW as MENU_SYS_ITEM_DRAW_ROW_3;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER as MOD_OBJ_SET_TRANSFORM_MODIFIER_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::POSITION_MODIFIER_CTOR as POSITION_MODIFIER_CTOR_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::PS2_MEMCARD_SET_STATE;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_BINDING_HAS_PROCESS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_GET_INSTANCE as SCRIPT_ENGINE_GET_INSTANCE_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_KILL_PROCESS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_STOP_PROCESS_SEQ;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SONG_INSTANCE_GET_BEAT_MS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SONG_INSTANCE_GET_POSITION_MS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP3;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_RESTART_PEAK_MUSIC;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WSCRIPT_PROCESS_LIST_CONTAINS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT as WSCRIPT_PROCESS_LIST_PUSH_FRONT_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WSCRIPT_PROCESS_LIST_REMOVE as WSCRIPT_PROCESS_LIST_REMOVE_2;
    pub(crate) use crate::game::main::gamemode::callee::AI_RAND;
    pub(crate) use crate::game::main::gamemode::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamemode::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GET_MEDAL_REWARD_ID;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GRANT_REWARD;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_IS_REWARD_GRANTED;
    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE as BUILTIN_DELETE_5;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UNLOAD;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2;
    pub(crate) use crate::game::main::gamemode::callee::CBE_GET_BE;
    pub(crate) use crate::game::main::gamemode::callee::CBX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::main::gamemode::callee::CPS2_MEM_CARD_SET_STATE as CPS2_MEM_CARD_SET_STATE_2;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_FREE_BUFFERS;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_RESET;
    pub(crate) use crate::game::main::gamemode::callee::CUI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::main::gamemode::callee::CVT_FLOAT_TO_INT;
    pub(crate) use crate::game::main::gamemode::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE as EA_MEM_FREE_2;
    pub(crate) use crate::game::main::gamemode::callee::FE_ASYNC_MANAGER_SET_FILE_STATUS;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_RIDER_PREVIEW_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::main::gamemode::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::main::gamemode::callee::FUNC_0028F140;
    pub(crate) use crate::game::main::gamemode::callee::MEMCPY;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET as MEMSET_2;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;
    pub(crate) use crate::game::main::gamemode::callee::MENU_SYS_ITEM_DRAW_ROW as MENU_SYS_ITEM_DRAW_ROW_4;
    pub(crate) use crate::game::main::gamemode::callee::MODIFIER_BLOCK_HAS_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_CTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_SET_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::main::gamemode::callee::RAIL_MODIFIER_CTOR;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_FIND_SNAPSHOT;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_REQUEST_INSTANT_REPLAY;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_GET_SYNC_CHECKSUM;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_BANK_PENDING_SCORE;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_CLEAR_HUD_MESSAGES;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_COMBO;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_TRICK;
    pub(crate) use crate::game::main::gamemode::callee::RUN_LIST_SEEK;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_LIST_ON_SEQ_ADVANCE;
    pub(crate) use crate::game::main::gamemode::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE as SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
    pub(crate) use crate::game::main::gamemode::callee::STRCPY as STRCPY_3;
    pub(crate) use crate::game::main::gamemode::callee::STR_HASH;
    pub(crate) use crate::game::main::gamemode::callee::TEX_MAN_UNLOCK_VRAM;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_ACCEPT_VISITOR;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_FIRST as UI_LIST_FIRST_2;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::main::gamemode::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF as UI_OBJECT_SHOW_CLAMP_ALPHA_HALF_2;
    pub(crate) use crate::game::main::gamemode::callee::USTR_TO_ASCII as USTR_TO_ASCII_2;
    pub(crate) use crate::game::main::gamemode::callee::WSCRIPT_MAN_RESET_COLLECT_SET;
    pub(crate) use crate::game::main::gamestate::callee::ADF_DATA_SOURCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::ANIM_DATA_BASE_FREE_CHAN_CACHES;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_STOP_HANDLE as BANK_MONITOR_STOP_HANDLE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_ECON_INT_GET_TOTAL_EARNED;
    pub(crate) use crate::game::main::gamestate::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE_4;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INTERFACE_3;
    pub(crate) use crate::game::main::gamestate::callee::BE_NEW_PLAYER_INT_GET_SLOT as BE_NEW_PLAYER_INT_GET_SLOT_2;
    pub(crate) use crate::game::main::gamestate::callee::BOLTS_FIND_BOLT_INDEX;
    pub(crate) use crate::game::main::gamestate::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_7;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::main::gamestate::callee::C_GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::main::gamestate::callee::C_KEYBOARD_POPUP_SET_KEY_ENABLED;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_ALLOC_NAMED_3 as EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4 as EA_MEM_FREE_3;
    pub(crate) use crate::game::main::gamestate::callee::GAME_ANIM_MAP_UNREGISTER_BANK;
    pub(crate) use crate::game::main::gamestate::callee::GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_CLEAR_SLOTS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_INSTALL_VIF_VU_HANDLERS;
    pub(crate) use crate::game::main::gamestate::callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS;
    pub(crate) use crate::game::main::gamestate::callee::MEM_FREE as MEM_FREE_3;
    pub(crate) use crate::game::main::gamestate::callee::MEM_MOVE;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_IS_DISABLED as MENU_SYS_ITEM_IS_DISABLED_2;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_NEXT as MENU_SYS_PAGE_SELECT_NEXT_2;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_PREV;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_VALIDATE_SELECTION as MENU_SYS_PAGE_VALIDATE_SELECTION_2;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_COUNT_GROUP_MESSAGES;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_GET_MESSAGE_COUNT;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_REMOVE_MESSAGE;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_REFRESH_LIST;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_COUNT_TEXT;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::main::gamestate::callee::RIDER_CLEAR_NEARBY_LIGHTS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_PAUSE_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_RESUME_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_CMD_QUEUE_AT_2 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ as SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_2;
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
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_2 as SSX_AUDIO_SYSTEM_GET_INSTANCE_3;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_RESET_RACE_SOUNDS;
    pub(crate) use crate::game::main::gamestate::callee::STR_COPY;
    pub(crate) use crate::game::main::gamestate::callee::STR_N_COPY;
    pub(crate) use crate::game::main::gamestate::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_FIRST_2 as UI_LIST_FIRST_3;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_NODE_DTOR_BODY as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR_4;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2 as UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::VD_TEXTURE_SINK_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_1;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BEREWARD_INT_IS_PEAK_COMPLETE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_SCORE_INT_UPDATE_BEST_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_IS_CONNECTED;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_RECONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_6;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_CTOR_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_DTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_BE as CBE_GET_BE_2;
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
    pub(crate) use crate::game::movie::movieplayer::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES as FLEX_BRIDGE_ALLOC_SPRING_NODES_2;
    pub(crate) use crate::game::movie::movieplayer::callee::FONT_PARSE_GLYPHS;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_OVERLAY_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_CREATE_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_JOIN_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_ON_OPERATION_DONE;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_SET_MODE_2 as MC_MANAGER_SET_MODE;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_ALLOC_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_ITEM_ON_FOCUS as MENU_SYS_ITEM_ON_FOCUS_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_ON_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_SET_SELECTION;
    pub(crate) use crate::game::movie::movieplayer::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER as MOD_OBJ_SET_TRANSFORM_MODIFIER_3;
    pub(crate) use crate::game::movie::movieplayer::callee::MULTI_SPLINE_MODIFIER_ADD_REF;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_SET_QUALITY_ICONS;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::movie::movieplayer::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PDA_TEMPLATE_OPEN;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PROFILE_SAVE_PDA_INIT_SLOT_LABELS;
    pub(crate) use crate::game::movie::movieplayer::callee::PARENT_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_ON_SPLIT_MARKER;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_RESET_STATS as RIDER_METRIX_RESET_STATS_2;
    pub(crate) use crate::game::movie::movieplayer::callee::ROLLER_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_FIND_SOUND_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS as SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_RUN_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_UNLOAD_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::SPRINTF_2;
    pub(crate) use crate::game::movie::movieplayer::callee::STRCPY_2 as STRCPY_4;
    pub(crate) use crate::game::movie::movieplayer::callee::STREAM_BUFFER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_LOAD_TEXTURES;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_STORE_WORD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR_3;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_MGR_ADD_BACKGROUND_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESSES;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESS_COUNT;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_MAN_CREATE_PROCESS;
    pub(crate) use crate::game::render::particle::callee::ALGORITHM_LIST_DTOR;
    pub(crate) use crate::game::render::particle::callee::ANIM_OBJECT_BASE_DTOR;
    pub(crate) use crate::game::render::particle::callee::AVA_SPLINE_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::BANK_MONITOR_INSTANCE_PAUSE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::render::particle::callee::BENEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::render::particle::callee::BENEW_RACE_INT_IS_STATION_TRACK_BY_INDEX;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_COLLECTIBLE_COLLECTED;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::BUILTIN_DELETE_4;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_COPY;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_CONCAT_IN_PLACE;
    pub(crate) use crate::game::render::particle::callee::CFE_STATE_CHAR_SELECT_UPDATE_ATTRIBUTE_BARS;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAMERA_ALGORITHM_CTOR as CHASE_CAMERA_ALGORITHM_CTOR_2;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::render::particle::callee::CPS2_CHUNK_RESOLVE_DTOR_STUB;
    pub(crate) use crate::game::render::particle::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::render::particle::callee::CROWD_MAN3D_GROUP_REMOVE_SECTION;
    pub(crate) use crate::game::render::particle::callee::CUT_SCENE_TRANSITION_SAVE_VIEWPORT;
    pub(crate) use crate::game::render::particle::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::render::particle::callee::FLEX_BRIDGE_SPRINGS_SAVE;
    pub(crate) use crate::game::render::particle::callee::INI_PARSER_SKIP_TO_EOL;
    pub(crate) use crate::game::render::particle::callee::ISIGNAL_SEMA;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_POP;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_SELECT;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_CTOR as MENU_SYS_ITEM_CTOR_2;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_DRAW_ROW_2;
    pub(crate) use crate::game::render::particle::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::render::particle::callee::PATH_POST_EVENT;
    pub(crate) use crate::game::render::particle::callee::PATH_SET_PAUSED;
    pub(crate) use crate::game::render::particle::callee::POSITION_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_FINISH_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_SETTLE_SLEW;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_FINISH_BOARD_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_DELETE;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_MKDIR;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_READ;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_SEEK;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_WRITE;
    pub(crate) use crate::game::render::particle::callee::SNDSTRM_SET_PITCH;
    pub(crate) use crate::game::render::particle::callee::SND_LOCK;
    pub(crate) use crate::game::render::particle::callee::SND_SPU_MEM_SET_BASE;
    pub(crate) use crate::game::render::particle::callee::SND_UNLOCK;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_MS_TO_NEXT_BEAT;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_STREAM_HANDLE;
    pub(crate) use crate::game::render::particle::callee::STRLEN;
    pub(crate) use crate::game::render::particle::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_PUSH_BACK_2;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_RESET;
    pub(crate) use crate::game::render::particle::callee::WORLD_CREATE_DEAD_NODE;
    pub(crate) use crate::game::render::particle::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_MAN_FREE_ENTRY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR as MENU_SYS_ITEM_CTOR_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_DRAW_ROW_4 as MENU_SYS_ITEM_DRAW_ROW_5;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4 as SCRIPT_DATA_GET_INFO_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2 as SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3 as SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3 as SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_FIRST_5 as UI_LIST_FIRST_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_SCREEN_FIND_OBJECT_3 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::sound::bankmonitor::callee::ADF_DATA_SOURCE_SET_DATA;
    pub(crate) use crate::game::sound::bankmonitor::callee::AUDIO_RAND;
    pub(crate) use crate::game::sound::bankmonitor::callee::BANK_MONITOR_TIMER_KILL;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_GET_EVENT_RECORD_INDEX;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_MISSION_INT_GET_COLLECTED_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BOLTS_FREE_DB;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUDDY_MAN_CONNECT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_3;
    pub(crate) use crate::game::sound::bankmonitor::callee::CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::sound::bankmonitor::callee::EA_FILE_LOAD_DECOMPRESSED;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_ANIM_MAP_FIND_HASH;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_STATE_GOTO_NEXT_2 as GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::sound::bankmonitor::callee::LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMCARD_REQUEST_GET_INFO_NOW;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMSET_2 as MEMSET;
    pub(crate) use crate::game::sound::bankmonitor::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::sound::bankmonitor::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_FLOAT_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_VEC3;
    pub(crate) use crate::game::sound::bankmonitor::callee::PS2_MEMCARD_CREATE;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_LIST_ADD_CHOICE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SECTION_ALLOCATOR_DTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::sound::bankmonitor::callee::SNDSTRM_SET_PREBUFFER;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_MEMORY_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::SONG_INSTANCE_GET_REQUEST_STATUS;
    pub(crate) use crate::game::sound::bankmonitor::callee::SPEECH_SYSTEM_STOP_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_END_SCRIPT_MUSIC;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::STREAM_FILE_RELEASE_CHUNK;
    pub(crate) use crate::game::sound::bankmonitor::callee::STR_CASE_CMP;
    pub(crate) use crate::game::sound::bankmonitor::callee::TEX_MAN_FREE_TEXTURE;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_MENU_SET_NAV_BUTTONS;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::sound::bankmonitor::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::sound::bankmonitor::callee::WORLD_PAINTER_SET_SCRIPT_SURFACE_MODE;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_MAN_CALL_TASK_FUNC;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_REMOVE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::CBXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::C_WORLD_SOUND_UNLINK_EMITTERS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EA_MEM_COPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EVENT_SELECT_HANDLE_INPUT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::GRAPHICS_MAN_EMPTY_HOOK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::IRRADIANCE_REF_GET_ENTRY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::LOBBY_MAN_REQUEST_STATS_PAGE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MC_MANAGER_GET_INSTANCE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MENU_SYS_GET_INPUT_CODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_SET_TRACK_CONTROL_VALUE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RIDER_METRIX_BANK_COMBO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_DUMMY_OBJ_READ_FIELDS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_ENGINE_PUSH_FREE_NODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_OBJ_DEACTIVATE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SSX_APP_BOOT_LOAD;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_UPDATE_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STREAM_BUFFER_GET_STATUS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STRLEN_AND_STRCPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::TEX_MAN_SET_ALLOC_FLAGS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_NODE_UNLINK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_STATE_MGR_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::USTR_FROM_ASCII;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_RESET_SECTIONS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_INFO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_NAME;
    pub(crate) use crate::game::worldpainter::quadtree::callee::AI_VFX_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::BUILTIN_DELETE_2;
    pub(crate) use crate::game::worldpainter::quadtree::callee::CONSOLE_CONFIG_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MEM_ALLOC;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EMITTER_RENDER;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GET_TRANSFORM_BOUNDS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GRAPHICS_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GRAPHICS_ON_VIF1_DMA_END;
    pub(crate) use crate::game::worldpainter::quadtree::callee::INTC_REMOVE_HANDLER;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_INIT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_SET_STATE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MENU_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MOD_OBJ_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::PS2_SOUND_MANAGER_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SNDSTRM_GET_REQUEST_STATUS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_FADE_OUT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_GET_STREAM_STATUS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_POST_EVENT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SPEECH_INSTANCE_STOP_STREAM;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STATS_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STRCPY;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STREAM_BUFFER_SET_FILE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STREAM_BUFFER_STOP;
    pub(crate) use crate::game::worldpainter::quadtree::callee::VFX_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::WORLD_GET_SECTION_SCRIPT_FUNC;
    pub(crate) use crate::game::worldpainter::quadtree::callee::WORLD_OBJ_CTOR;
}

/// Cross-shard calls the generator emitted as `call_indirect(c, addr)`: they
/// resolve through the function table like the original's `jal`.
pub(crate) mod table_callee {
    pub(crate) use crate::game::be::bedbreward::table_callee::EA_MEMSET;
    pub(crate) use crate::game::be::bedbreward::table_callee::OPERATOR_DELETE;
    pub(crate) use crate::game::be::bedbreward::table_callee::WORLD_CREATE_DEAD_NODE;
}

/// `gp`-relative globals used here.
pub(crate) mod g {
    pub(crate) use crate::game::fe::festateload::g::BE_OPTION_SINGLETON;
    pub(crate) use crate::game::fe::festateload::g::BOLT_FILE_CACHE;
    pub(crate) use crate::game::fe::festateload::g::FE_BASE_BACK;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER_END;
    pub(crate) use crate::game::fe::festateload::g::WORLD_TRIGGER_MANAGER;
}

pub(crate) mod addr {
    pub(crate) use crate::game::bx::bxstring2::addr::BANK_MONITOR_TIMER_RESET;
    pub(crate) use crate::game::bx::bxstring2::addr::BE_INTERFACE_LOOKUP;
    pub(crate) use crate::game::bx::bxstring2::addr::CBE_GET_BE;
    pub(crate) use crate::game::bx::bxstring2::addr::CBE_GET_INTERFACE;
    pub(crate) use crate::game::bx::bxstring2::addr::FMODF;
    pub(crate) use crate::game::bx::bxstring2::addr::MC_MANAGER_GET_INFO;
    pub(crate) use crate::game::bx::bxstring2::addr::MEM_MAN_ALLOC;
    pub(crate) use crate::game::bx::bxstring2::addr::MEM_MAN_FREE;
    pub(crate) use crate::game::bx::bxstring2::addr::MEM_MAN_FREE_NOSIZE;
    pub(crate) use crate::game::bx::bxstring2::addr::SPEECH_BANK_FREE_CALLBACK;
    pub(crate) use crate::game::bx::bxstring2::addr::SPRINTF;
    pub(crate) use crate::game::bx::bxstring2::addr::UI_ENGINE_TEARDOWN;
}

/// `gp-0x848`: the `World` singleton pointer (global data filled in at
/// startup).
pub(crate) const WORLD_GLOBAL: u32 = 0x848;

/// `World+0x84`: the World's AI manager.
pub(crate) const WORLD_AI_MANAGER: u32 = 0x84;

/// `World+0x84+0x70`: the World's flag manager.
pub(crate) const FLAG_MANAGER: u32 = 0x70;

/// `AIManager+0xc`: the AI currently being processed.
pub(crate) const AI_MANAGER_CURRENT: u32 = 0xc;

/// `AI+0x8`: the AI's frame counter.
pub(crate) const AI_FRAME_COUNT: u32 = 0x8;

// ---------------------------------------------------------------- world

/// `int World_GetAIFrameCount(void)` (0x2d1c98): the current AI's frame
/// counter. `World` at `gp-0x848`, its AI manager at `+0x84`, the current AI
/// at `+0xc`, and the count at `+0x8`.
pub fn world_get_ai_frame_count(c: &mut Cpu) {
    c.cycles += 5;
    let gp = Addr(c.g[GP][0] as u32);
    let world = gp.off(0u32.wrapping_sub(WORLD_GLOBAL)).lw(c);
    c.g[V0][0] = world;
    let ai_manager = Addr(world as u32).off(WORLD_AI_MANAGER).lw(c);
    c.g[V1][0] = ai_manager;
    let ai = Addr(ai_manager as u32).off(AI_MANAGER_CURRENT).lw(c);
    c.g[A0][0] = ai;
    c.g[V0][0] = Addr(ai as u32).off(AI_FRAME_COUNT).lw(c);
    abi::ret(c);
}

/// `FlagManager *World_GetFlagManager(void)` (0x2d1cb0): the World's flag
/// manager (`World+0x84` → `+0x70`).
pub fn world_get_flag_manager(c: &mut Cpu) {
    c.cycles += 4;
    let gp = Addr(c.g[GP][0] as u32);
    let world = gp.off(0u32.wrapping_sub(WORLD_GLOBAL)).lw(c);
    c.g[V0][0] = world;
    let manager = Addr(world as u32).off(WORLD_AI_MANAGER).lw(c);
    c.g[V1][0] = manager;
    c.g[V0][0] = Addr(manager as u32).off(FLAG_MANAGER).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Menu system.

/// `MenuSysItem* MenuSysItem_ctor(MenuSysItem* item, void* data)` (0x2ca258):
/// set the vtable at +0x10, flags at +4, name at +8 and clear +0xc.
pub fn menu_sys_item_ctor(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V0][0] = 0x480000;
    c.g[V1][0] = 0x1;
    c.g[V0][0] = sx(0x480000u32.wrapping_add(0x6f28));
    let item = Addr(abi::arg(c, 0));
    item.off(0x4).sw(c, c.g[V1][0]);
    item.off(0x10).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    item.off(0x8).sw(c, c.g[A1][0]);
    item.off(0xc).sw(c, 0);
    abi::ret(c);
}

/// `bool MenuSysItem_IsVisible(MenuSysItem* item)` (0x2ca378): the visible bit.
pub fn menu_sys_item_is_visible(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x4).lw(c);
    c.g[V0][0] &= 0x1;
    abi::ret(c);
}

/// `void MenuSysItem_SetVisible(MenuSysItem* item, bool visible)` (0x2ca388):
/// set or clear the visible bit.
pub fn menu_sys_item_set_visible(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x4).lw(c);
    if c.g[A1][0] == 0 {
        c.cycles += 4;
        c.g[V1][0] = 0xffff_ffff_ffff_fffe;
        c.g[V0][0] &= c.g[V1][0];
    } else {
        c.cycles += 3;
        c.g[V0][0] |= 0x1;
    }
    Addr(abi::arg(c, 0)).off(0x4).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `bool MenuSysItem_IsDisabled(MenuSysItem* item)` (0x2ca3b0): the disabled bit.
pub fn menu_sys_item_is_disabled(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x4).lw(c);
    c.g[V0][0] &= 0x8;
    abi::ret(c);
}

/// `MenuSysPage* MenuSysPage_ctor(MenuSysPage* page)` (0x2caa58): set the
/// vtable at +0x12c and clear the four state words.
pub fn menu_sys_page_ctor(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V1][0] = 0x480000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx(0x480000u32.wrapping_add(0x6ed0));
    let page = Addr(abi::arg(c, 0));
    page.sw(c, 0);
    page.off(0x12c).sw(c, c.g[V1][0]);
    page.off(0x4).sw(c, 0);
    page.off(0x8).sw(c, 0);
    page.off(0x124).sw(c, 0);
    abi::ret(c);
}

/// `void MenuSysPage_RemoveAllItems(MenuSysPage* page)` (0x2cbe10): clear the
/// item list head/tail.
pub fn menu_sys_page_remove_all_items(c: &mut Cpu) {
    c.cycles += 3;
    let page = Addr(abi::arg(c, 0));
    page.sw(c, 0);
    page.off(0x4).sw(c, 0);
    abi::ret(c);
}

/// `void MenuSys_Reset(MenuSys* menu)` (0x2cc070): reset the menu state.
pub fn menu_sys_reset(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = 0xb;
    let menu = Addr(abi::arg(c, 0));
    menu.off(0x50).sw(c, c.g[V1][0]);
    menu.off(0x48).sw(c, c.g[V0][0]);
    menu.off(0x58).sw(c, 0);
    menu.off(0x54).sw(c, 0);
    menu.sw(c, 0);
    menu.off(0x64).sw(c, 0);
    menu.off(0x4c).sw(c, c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// World.

/// `int World_GetRiderCount(void)` (0x2d1af0): follow the world singleton to
/// the rider manager and return its count at +0x78.
pub fn world_get_rider_count(c: &mut Cpu) {
    c.cycles += 5;
    let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[V0][0] = world;
    let man = Addr(world as u32).off(0x84).lw(c);
    c.g[V1][0] = man;
    let sub = Addr(man as u32).off(0xc).lw(c);
    c.g[A0][0] = sub;
    c.g[V0][0] = Addr(sub as u32).off(0x78).lw(c);
    abi::ret(c);
}

/// `Rider* World_GetRider(World* w, int index)` (0x2d1b08): look up the rider
/// pointer table entry, returning null when the entry is absent.
pub fn world_get_rider(c: &mut Cpu) {
    c.cycles += 9;
    c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x84).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x28).lw(c);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6c0));
    if c.g[V1][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Script hooks.

/// `int WScriptMan_RunObjScript5Hook(int id)` (0x2d1ac8): run object script 5
/// on the current object script manager (`gp+0xce8`).
pub fn wscript_man_run_obj_script5_hook(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[A1][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(gp(c, 0xce8)).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0030_a688, 0x002d_1ae0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MenuSysItem_GetInputCode(MenuSysItem **this)` (0x002ca3e8): the input
/// code of the menu system at `(*this) + 0x124`.
pub fn menu_sys_item_get_input_code(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x124)).lw(c);
        call_ptr(c, callee::MENU_SYS_GET_INPUT_CODE, 0x002c_a3fc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSys_dtor(MenuSys *this, int flags)` (0x2cc048).
pub fn menu_sys_dtor(c: &mut Cpu) {
    guest(c, |c| destroy_if_flag(c, 0x002c_c060))
}

/// `int MenuSysPage_GetSelectedIndex(MenuSysPage* self)` (0x2cbf08): validate
/// the page's selection, then read the selected index (+0x4).
pub fn menu_sys_page_get_selected_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_PAGE_VALIDATE_SELECTION.call(c, 0x002c_bf1c)?;
        c.g[V0][0] = Addr(at(c.g[S0][0] as u32, 4)).lw(c);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void MenuSysPoint2DItem_OnFocus(MenuSysPoint2DItem* self)` (0x2d08e0):
/// run the base item's focus handler and clear the item's focus flag (+0x40).
pub fn menu_sys_point2d_item_on_focus(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_ON_FOCUS.call(c, 0x002d_08f4)?;
        Addr(at(c.g[S0][0] as u32, 0x40)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Table and vtable dispatchers.

/// `void ModObj_CallSlot7(ModObj* obj)` (0x2d1b30): call the object's vtable
/// slot 7 (delta at `+0x38`, method at `+0x3c`) with the adjusted `this`.
pub fn mod_obj_call_slot7(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(abi::arg(c, 0)).lw(c);
        let vt = c.g[V1][0] as u32;
        c.g[A1][0] = Addr(at(vt, 0x38)).lh(c);
        c.g[V0][0] = Addr(at(vt, 0x3c)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, target, 0x002d_1b4c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `MenuSysItem_dtor` (0x2ca280): vtable at `+0x10`.
pub fn menu_sys_item_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x10, 0x0048_6f28, 0x002c_a2a4);
}

/// `MenuSysPage_dtor` (0x2caa80): vtable at `+0x12c`.
pub fn menu_sys_page_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x12c, 0x0048_6ed0, 0x002c_aaa4);
}

// ---------------------------------------------------------------------------
// Menus.

/// `MenuSysPage_GetSelectedItem(MenuSysPage* page)` (0x2cbf30): select the
/// page's current item first, then return `page->items[selected]` (the word
/// at `page + index*4 + 0xc`).
pub fn menu_sys_page_get_selected_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x002c_bf08, 0x002c_bf44)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        frame.restore_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysGroupItem_v08(MenuSysGroupItem* item)` (0x2cf8d8): notify the item
/// that its selected page changed (virtual slot `+0x40`/`+0x44` of the
/// selected item's object).
pub fn menu_sys_group_item_v08(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x002c_bf30, 0x002c_f8e8)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x40).lh(c);
        c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x44).lw(c);
        let target = c.g[A1][0] as u32;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002c_f8fc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysGroupItem_v10(MenuSysGroupItem* item)` (0x2cfb28): the same for the
/// object at `(*item)->page` (slot `+0x20`/`+0x24` of the vtable at `+0x12c`).
pub fn menu_sys_group_item_v10(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x12c).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x20).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x24).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002c_fb4c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script system.

/// `WScriptMan_ObjTriggerHook(this, a, b, c)` (0x2d19b8): forward an
/// object-trigger event to the script manager singleton at `gp+0xce8`.
pub fn wscript_man_obj_trigger_hook(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        c.g[V1][0] = c.g[A1][0];
        c.g[A3][0] = c.g[A2][0];
        c.g[A0][0] = Addr(c.g[GP][0] as u32).off(0xce8).lw(c);
        c.g[A1][0] = c.g[V0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[V1][0];
        call_ptr(c, 0x0030_a060, 0x002d_19dc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menu item measurement.

/// `MenuSysItem* Measure(MenuSysItem* self, sItemLayout* layout)`
/// (0x2ccd90 / 0x2ccea8 / 0x2cd2f0): measure the row with the layout's row
/// count and return `self`.
pub(crate) fn menu_item_measure(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x20);
    c.g[A2][0] = 0;
    frame.save(c, S0, 0x10);
    c.g[T0][0] = 0;
    frame.save_ra(c, 0);
    let item = c.g[A0][0];
    c.g[S0][0] = item;
    c.g[A3][0] = Addr(abi::arg(c, 1)).off(0x14).lw(c);
    callee::MENU_ITEM_MEASURE_ROW.call(c, ret)?;
    c.g[V0][0] = c.g[S0][0];
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `MenuSysSubmenuItem_Measure` (0x2ccd90).
pub fn menu_sys_submenu_item_measure(c: &mut Cpu) {
    guest(c, |c| menu_item_measure(c, 0x002c_cdb0))
}

/// `MenuSysBackItem_Measure` (0x2ccea8).
pub fn menu_sys_back_item_measure(c: &mut Cpu) {
    guest(c, |c| menu_item_measure(c, 0x002c_cec8))
}

/// `MenuSysLabelItem_Measure` (0x2cd2f0).
pub fn menu_sys_label_item_measure(c: &mut Cpu) {
    guest(c, |c| menu_item_measure(c, 0x002c_d310))
}

// ---------------------------------------------------------------------------
// MenuSys*Item_Measure trampolines.
//
// Each is the same 7-instruction wrapper: `t0` receives the static row layout
// table, `a2` the item's value/count field at `+0x14`, `a3` zero, then
// `MenuSysItem_MeasureRow(row, item)` lays the row out; the item is returned.

/// `void* menu_measure(this, item, table, ret)`.
pub(crate) fn menu_measure(c: &mut Cpu, table: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[T0][0] = 0x0048_0000;
        frame.save(c, S0, 0x10);
        c.g[T0][0] = sx(table);
        frame.save_ra(c, 0);
        let row = c.g[A0][0];
        c.g[S0][0] = row;
        c.g[A3][0] = 0;
        c.g[A2][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x14)).lw(c);
        callee::MEASURE_ROW.call(c, ret)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void* MenuSysCallbackItem_Measure(this, item)` (0x2cd240): lay the item
/// out with the callback-item descriptor table.
pub fn menu_sys_callback_item_measure(c: &mut Cpu) {
    menu_measure(c, 0x0048_66c8, 0x002c_d264);
}

/// `void* MenuSysInt8Item_Measure(this, item)` (0x2cd7b8): lay the item out
/// with the integer-item descriptor table.
pub fn menu_sys_int8_item_measure(c: &mut Cpu) {
    menu_measure(c, 0x0048_66e0, 0x002c_d7dc);
}

/// `void* MenuSysIntItem_Measure(this, item)` (0x2cdbf0): lay the item out
/// with the integer-item descriptor table.
pub fn menu_sys_int_item_measure(c: &mut Cpu) {
    menu_measure(c, 0x0048_66e0, 0x002c_dc14);
}

/// `void* MenuSysFloatItem_Measure(this, item)` (0x2cdfa0): lay the item out
/// with the float-item descriptor table.
pub fn menu_sys_float_item_measure(c: &mut Cpu) {
    menu_measure(c, 0x0048_66f0, 0x002c_dfc4);
}

/// `void* MenuSysFloatRangeItem_Measure(this, item)` (0x2ce368): lay the item
/// out with the float-item descriptor table.
pub fn menu_sys_float_range_item_measure(c: &mut Cpu) {
    menu_measure(c, 0x0048_66f0, 0x002c_e38c);
}

/// `void MenuSysActionItem_Draw(Item* self, u32 arg)` (0x2cd0e8): draw the
/// row with the item's icon (`+0x18`) or its label (`+0x14`) as the fourth
/// argument.
pub fn menu_sys_action_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let self_at = abi::arg(c, 0);
        c.g[T0][0] = Addr(self_at).off(0x18).lw(c);
        if c.g[T0][0] != 0 {
            c.cycles += 5;
            c.g[A2][0] = Addr(self_at).off(0x14).lw(c);
            c.g[A3][0] = 0;
            callee::MENU_SYS_ITEM_DRAW_ROW.call(c, 0x002c_d108)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 4;
            c.g[A3][0] = Addr(self_at).off(0x14).lw(c);
            c.g[A2][0] = 0;
            c.g[T0][0] = 0;
            callee::MENU_SYS_ITEM_DRAW_ROW.call(c, 0x002c_d11c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        leave(c, 0x10);
        Ok(())
    })
}

/// `void World_GetRiderPosition(int i)` (0x2d1b58): call the rider slot `i`
/// object's (`race->riders[i]`) slot-5 method with its `+0x6c0` sub-object.
pub fn world_get_rider_position(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.lw(gp_2(c, -0x848));
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(abi::arg(c, 0) << 2);
        c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0x84));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0xc));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x28));
        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x6c0));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6c0));
        c.g[A1][0] = c.lh(r32(c, V1).wrapping_add(0x28));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x2c));
        let method = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
        call_ptr(c, method, 0x002d_1b90)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool MenuSysItem_IsSelectable(self)` (0x2ca320): visible
/// (`menu_item_is_visible`) and not disabled (`menu_item_is_disabled`).
pub fn menu_sys_item_is_selectable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        callee::MENU_SYS_ITEM_IS_VISIBLE.call(c, 0x002c_a33c)?;
        let hidden = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[S1][0];
        if !hidden {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::MENU_SYS_ITEM_IS_DISABLED.call(c, 0x002c_a34c)?;
            c.g[S1][0] = (c.g[V0][0] < 1) as u64;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPage_SetRowColor(self, row, on)` (0x2cb880): forward to the
/// row widget's colour method (`self+0x12c` is the vtable; entry 8's delta at
/// `+0x40`, its method at `+0x44`), passing the "on" or "off" colour global.
pub fn menu_sys_page_set_row_color(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let self_ = abi::r32(c, A0);
        c.g[V1][0] = Addr(self_.wrapping_add(0x12c)).lw(c);
        let vtable = c.g[V1][0] as u32;
        c.g[V0][0] = Addr(vtable.wrapping_add(0x40)).lh(c);
        c.g[A3][0] = sx(vtable.wrapping_add(0x44));
        let on = c.g[A2][0] != 0;
        c.g[A0][0] = sx(self_.wrapping_add(abi::r32(c, V0)));
        if on {
            c.cycles += 3;
            c.g[V0][0] = 0x4d_0000;
            c.g[A2][0] = sx(0x4d_0000u32.wrapping_add(0x5380));
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0x4d_0000;
            c.g[A2][0] = sx(0x4d_0000u32.wrapping_add(0x5390));
        }
        c.cycles += 6;
        c.g[V0][0] = Addr(abi::r32(c, A3)).lw(c);
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x002c_b8bc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPage_OnDeactivate(self)` (0x2cbfd0): deactivate the selected
/// row (`self->items[self->selected]`, its vtable's entry 7) and re-arm the
/// wrap-around flag (`+0x124 = 0`).
pub fn menu_sys_page_on_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = abi::r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(self_.wrapping_add(4)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        let slot = c.g[V0][0] as u32;
        c.g[V0][0] = sx(self_.wrapping_add(slot));
        let item_slot = c.g[V0][0] as u32;
        c.g[A1][0] = Addr(item_slot.wrapping_add(0xc)).lw(c);
        let item = c.g[A1][0] as u32;
        c.g[V1][0] = Addr(item.wrapping_add(0x10)).lw(c);
        let vtable = c.g[V1][0] as u32;
        c.g[A0][0] = Addr(vtable.wrapping_add(0x38)).lh(c);
        c.g[V0][0] = Addr(vtable.wrapping_add(0x3c)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx(item.wrapping_add(abi::r32(c, A0)));
        call_ptr(c, target, 0x002c_c004)?;
        Addr(self_.wrapping_add(0x124)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysBackItem* MenuSysBackItem_ctor(self, text)` (0x2ccdf0): a menu
/// item (id -1) that keeps `text` at `+0x14` and uses the back-item vtable.
pub fn menu_sys_back_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = u64::MAX;
        callee::MENU_SYS_ITEM_CTOR.call(c, 0x002c_ce10)?;
        let self_ = abi::r32(c, S0);
        c.g[V1][0] = 0x48_0000;
        Addr(self_.wrapping_add(0x14)).sw(c, c.g[S1][0]);
        c.g[V1][0] = sx(0x48_0000u32.wrapping_add(0x6d90));
        c.g[V0][0] = c.g[S0][0];
        Addr(self_.wrapping_add(0x10)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysActionItem* MenuSysActionItem_ctor(self)` (0x2ccf08): a menu item
/// (id -1) that uses the action-item vtable and starts with no action, label
/// or child page.
pub fn menu_sys_action_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = u64::MAX;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_CTOR.call(c, 0x002c_cf20)?;
        let self_ = abi::r32(c, S0);
        c.g[V1][0] = 0x48_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx(0x48_0000u32.wrapping_add(0x6d40));
        Addr(self_.wrapping_add(0x14)).sw(c, 0);
        Addr(self_.wrapping_add(0x10)).sw(c, c.g[V1][0]);
        Addr(self_.wrapping_add(0x1c)).sw(c, 0);
        Addr(self_.wrapping_add(0x20)).sw(c, 0);
        Addr(self_.wrapping_add(0x18)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int WScriptMan_TryRunObjScript3(id)` (0x2d1a30): when the script manager
/// (`gp_3+0xce8`) knows an object script for `id`, run it; returns 1 when it
/// ran.
pub fn wscript_man_try_run_obj_script3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(gp_3(c, 0xce8)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0030_a2e8, 0x002d_1a4c)?;
        let found = c.g[V0][0] != 0;
        c.g[A0][0] = Addr(gp_3(c, 0xce8)).lw(c);
        if !found {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 4;
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0030_a548, 0x002d_1a64)?;
        c.g[V0][0] = 1;
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPage_ValidateSelection(self)` (0x2caab0): when the selected
/// item is not selectable, move to the next one.
pub fn menu_sys_page_validate_selection(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(4).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
        c.g[A1][0] = Addr(r32(c, V0)).off(0xc).lw(c);
        c.g[V1][0] = Addr(r32(c, A1)).off(0x10).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x18).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x002c_aae4);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x002c_aae4)?;
        if c.g[V0][0] != 0 {
            // no-op: return
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x002c_aaf4);
            c.g[A0][0] = c.g[S0][0];
            callee::MENU_SYS_PAGE_SELECT_NEXT.call(c, 0x002c_aaf4)?;
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPage_EndDraw(void)` (0x2cb2f8): end the current menu's draw,
/// end its draw list and undo the 0x14 draw offset.
pub fn menu_sys_page_end_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = lw(c, gp(c, -0x854));
        c.g[V0][0] = Addr(r32(c, A1)).off(0x10d8).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x100).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x104).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x002c_b318);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x002c_b318)?;
        c.g[A1][0] = lw(c, gp(c, -0x854));
        c.g[V0][0] = Addr(r32(c, A1)).off(0x10d8).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0xb0).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0xb4).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x002c_b330);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x002c_b330)?;
        c.g[A0][0] = lw(c, gp(c, -0x854));
        c.g[V0][0] = Addr(r32(c, A0)).off(0xe84).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
        Addr(p(r32(c, A0), 0xe84)).sw(c, c.g[V0][0]);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* MenuSysActionItem_Measure(self, item)` (0x2cd090): measure an
/// action item's row with its icon or its text width.
pub fn menu_sys_action_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[T0][0] = Addr(r32(c, A1)).off(0x18).lw(c);
        let has_icon = c.g[T0][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        if has_icon {
            c.cycles += 5;
            c.g[A2][0] = Addr(r32(c, A1)).off(0x14).lw(c);
            c.g[RA][0] = sx(0x002c_d0b4);
            c.g[A3][0] = 0;
            callee::MENU_SYS_ITEM_MEASURE_ROW.call(c, 0x002c_d0b4)?;
            c.g[V0][0] = c.g[S0][0];
        } else {
            c.cycles += 6;
            c.g[A3][0] = Addr(r32(c, A1)).off(0x14).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = 0;
            c.g[RA][0] = sx(0x002c_d0d0);
            c.g[T0][0] = 0;
            callee::MENU_SYS_ITEM_MEASURE_ROW.call(c, 0x002c_d0d0)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menus (s09).

/// `void MenuSysBoolItem_Draw(MenuSysBoolItem* self, int a1)` (0x2cc758):
/// draw a two-state menu row, picking the on/off variant from the item's
/// image and state fields.
pub fn menu_sys_bool_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[T1][0] = Addr(this).off(0x18).lw(c);
        if c.g[T1][0] == 0 {
            c.g[A3][0] = Addr(this).off(0x14).lw(c);
        } else {
            c.cycles += 3;
            c.g[T0][0] = Addr(this).off(0x1c).lw(c);
            let no_state = c.g[T0][0] == 0;
            c.g[A3][0] = 0;
            if !no_state {
                c.cycles += 7;
                c.g[V1][0] = Addr(this).off(0x20).lw(c);
                c.g[A2][0] = Addr(this).off(0x14).lw(c);
                c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                if c.g[V0][0] != 0 {
                    c.g[T0][0] = c.g[T1][0];
                }
                callee::MENU_SYS_ITEM_DRAW_ROW_2.call(c, 0x002c_c78c)?;
                frame.restore_ra(c, 0);
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            c.cycles += 1;
            c.g[A3][0] = Addr(this).off(0x14).lw(c);
        }
        c.cycles += 4;
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x002c_c7a4);
        c.g[T0][0] = 0;
        callee::MENU_SYS_ITEM_DRAW_ROW_2.call(c, 0x002c_c7a4)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// A menu item constructor: `MenuSysItem_ctor(this, -1)`, then the class
/// vtable and its two data words. Shared by the three below.
pub(crate) fn menu_item_ctor_body(
    c: &mut Cpu,
    this: u64,
    vtable_delta: u32,
    first: u64,
    second: u64,
    ret: u32,
) -> crate::call::Flow {
    callee::MENU_SYS_ITEM_CTOR_2.call(c, ret)?;
    c.g[V1][0] = 0x0048_0000;
    Addr(this as u32).off(0x14).sw(c, first);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(vtable_delta));
    Addr(this as u32).off(0x18).sw(c, second);
    Addr(this as u32).off(0x10).sw(c, c.g[V1][0]);
    c.g[V0][0] = this;
    Ok(())
}

/// `MenuSysSubmenuItem* MenuSysSubmenuItem_ctor(this, a1, a2)` (0x2cccc8).
pub fn menu_sys_submenu_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = u64::MAX;
        let (this, first, second) = (c.g[S0][0], c.g[S1][0], c.g[S2][0]);
        menu_item_ctor_body(c, this, 0x6de0, first, second, 0x002c_ccf0)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysColorChannelItem* MenuSysColorChannelItem_ctor(this, a1, a2)`
/// (0x2ced20).
pub fn menu_sys_color_channel_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = u64::MAX;
        let (this, first, second) = (c.g[S0][0], c.g[S1][0], c.g[S2][0]);
        menu_item_ctor_body(c, this, 0x6a70, first, second, 0x002c_ed48)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysColorSwatchItem* MenuSysColorSwatchItem_ctor(this, a1, a2)`
/// (0x2cff28).
pub fn menu_sys_color_swatch_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = u64::MAX;
        let (this, first, second) = (c.g[S0][0], c.g[S1][0], c.g[S2][0]);
        menu_item_ctor_body(c, this, 0x6968, first, second, 0x002c_ff50)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menu items.

/// `s2CD3A0Vec MenuSysSpacerItem_Measure(self, item)` (0x2cd3a0): the spacer's
/// height (the int at `item + 0x14`) as a vec4 `{0, 0, 0, (float)h}`.
pub fn menu_sys_spacer_item_measure(c: &mut Cpu) {
    c.cycles += 19;
    c.mtc1(0, 0);
    let sp = (c.g[SP][0] as u32).wrapping_sub(0x10);
    c.g[SP][0] = sx(sp);
    c.g[V0][0] = c.g[A0][0];
    let out = c.g[A0][0] as u32;
    let height = Addr(abi::arg(c, 1)).off(0x14).0;
    c.swc1(0, sp);
    c.swc1(0, sp.wrapping_add(0x8));
    c.swc1(0, sp.wrapping_add(0x4));
    c.lwc1(1, height);
    c.cvt_s_w(1, 1);
    c.swc1(1, sp.wrapping_add(0xc));
    // The original copies the four words of the local vector with
    // ldl/ldr + sdl/sdr pairs; keep the exact (possibly misaligned) ops.
    c.ldl(3, sp.wrapping_add(7));
    c.ldr(3, sp);
    c.ldl(5, sp.wrapping_add(0xf));
    c.ldr(5, sp.wrapping_add(8));
    c.sdl(3, out.wrapping_add(7));
    c.sdr(3, out);
    c.sdl(5, out.wrapping_add(0xf));
    c.sdr(5, out.wrapping_add(8));
    c.g[SP][0] = sx(sp.wrapping_add(0x10));
    abi::ret(c);
}

/// `void MenuSysInt8Item_ResetValue(self)` (0x2cd578): store the item's
/// signed-byte value (`+0x1c`) through the pointer at `+0x18`, or 0 when it is
/// not positive.
pub fn menu_sys_int8_item_reset_value(c: &mut Cpu) {
    c.cycles += 3;
    let item = Addr(abi::arg(c, 0));
    c.g[V0][0] = item.off(0x1c).lb(c);
    let positive = (c.g[V0][0] as i64) > 0;
    c.g[V1][0] = item.off(0x1c).lbu(c);
    if positive {
        c.cycles += 3;
        c.g[V0][0] = item.off(0x18).lw(c);
        let dst = c.g[V0][0] as u32;
        let v = c.g[V1][0];
        Addr(dst).sb(c, v);
    } else {
        c.cycles += 3;
        c.g[V0][0] = item.off(0x18).lw(c);
        let dst = c.g[V0][0] as u32;
        Addr(dst).sb(c, 0);
    }
    abi::ret(c);
}

/// `void MenuSysIntItem_ResetValue(self)` (0x2cd9b8): store the item's int
/// value (`+0x1c`) through the pointer at `+0x18`, or 0 when it is not
/// positive.
pub fn menu_sys_int_item_reset_value(c: &mut Cpu) {
    c.cycles += 3;
    let item = Addr(abi::arg(c, 0));
    c.g[V1][0] = item.off(0x1c).lw(c);
    let positive = (c.g[V1][0] as i64) > 0;
    c.g[V0][0] = item.off(0x18).lw(c);
    let dst = c.g[V0][0] as u32;
    if positive {
        c.cycles += 2;
        let v = c.g[V1][0];
        Addr(dst).sw(c, v);
    } else {
        c.cycles += 2;
        Addr(dst).sw(c, 0);
    }
    abi::ret(c);
}

/// `void MenuSysFloatItem_ResetValue(self)` (0x2cdf78): store the item's
/// float value (`+0x1c`) through the pointer at `+0x18`, or +0.0 when it is
/// not positive.
pub fn menu_sys_float_item_reset_value(c: &mut Cpu) {
    c.cycles += 6;
    let item = Addr(abi::arg(c, 0));
    c.mtc1(0, 1);
    c.lwc1(0, item.off(0x1c).0);
    c.c_le_s(0, 1);
    let positive = !c.fcc();
    c.g[V0][0] = item.off(0x18).lw(c);
    let dst = c.g[V0][0] as u32;
    if positive {
        c.cycles += 2;
        c.swc1(0, dst);
    } else {
        c.cycles += 2;
        c.swc1(1, dst);
    }
    abi::ret(c);
}

/// `cMenuItem* MenuSysActionItem_ctor_3(cMenuItem* self, char* text, int a2,
/// int a3)` (0x2CCF98): the base constructor plus the action item's vtable and
/// two fields.
pub fn menu_sys_action_item_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A3][0];
        callee::MENU_SYS_ITEM_CTOR_3.call(c, 0x002c_cfbc)?;
        c.g[V1][0] = 0x480000;
        c.sw(ea(c.g[S0][0], 0x14), c.g[S1][0]);
        c.g[V1][0] = sx(ea(c.g[V1][0], 0x6d40));
        c.sw(ea(c.g[S0][0], 0x1c), c.g[S2][0]);
        c.sw(ea(c.g[S0][0], 0x10), c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw(ea(c.g[S0][0], 0x20), 0);
        c.sw(ea(c.g[S0][0], 0x18), 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysInt8Item_Draw(MenuSysInt8Item* self, void* gfx)` (0x2CD7F0):
/// format the byte value at `*(self+0x18)` into a stack buffer, then draw the
/// row.
pub fn menu_sys_int8_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x70);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[V0][0] = c.lw(ea(c.g[S0][0], 0x18));
        c.g[A1][0] = sx(ea(c.g[A1][0], 0x3970));
        c.g[A2][0] = c.lb(c.g[V0][0] as u32);
        call_ptr(c, addr::SPRINTF, 0x002c_d820)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.lw(ea(c.g[A0][0], 0x14));
        c.g[A3][0] = 0;
        c.g[T0][0] = c.g[SP][0];
        callee::MENU_SYS_ITEM_DRAW_ROW_3.call(c, 0x002c_d838)?;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysIntItem_Draw(MenuSysIntItem* self, void* gfx)` (0x2CDC28):
/// the same for the 32-bit value at `*(self+0x18)`.
pub fn menu_sys_int_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x70);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[V0][0] = c.lw(ea(c.g[S0][0], 0x18));
        c.g[A1][0] = sx(ea(c.g[A1][0], 0x3970));
        c.g[A2][0] = c.lw(c.g[V0][0] as u32);
        call_ptr(c, addr::SPRINTF, 0x002c_dc58)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.lw(ea(c.g[A0][0], 0x14));
        c.g[A3][0] = 0;
        c.g[T0][0] = c.g[SP][0];
        callee::MENU_SYS_ITEM_DRAW_ROW_3.call(c, 0x002c_dc70)?;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSys_Draw(item)` 0x2cc350: compute the item's scaled size into its
/// object (`+0x38`/`+0x3c`) and forward to the draw method of the item's
/// selected object.
pub fn menu_sys_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        c.lwc1(1, gp_4(c, -0x3ff4));
        frame.save_ra(c, 0x10);
        let obj = lw_at(c, this, 0x58) as u32;
        c.g[V1][0] = obj as u64;
        c.swc1(1, p(sp(c), 4));
        c.swc1(1, sp(c));
        c.lwc1(0, p(obj, 0x30));
        c.lwc1(2, p(obj, 0x34));
        c.mul_s(0, 0, 1);
        c.swc1(0, p(obj, 0x38));
        c.lwc1(1, p(sp(c), 4));
        c.mul_s(2, 2, 1);
        c.swc1(2, p(obj, 0x3c));
        c.g[V0][0] = Addr(this).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, V0)));
        c.g[A1][0] = Addr(p(r32(c, A0), 4)).lw(c);
        let target = Addr(p(r32(c, A1), 0x12c)).lw(c) as u32;
        c.g[V0][0] = target as u64;
        c.g[A0][0] = Addr(p(target, 0x28)).lh(c);
        c.g[V1][0] = lw_at(c, target, 0x2c);
        let func = r32(c, V1);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, func, 0x002c_c3ac)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menu system.

/// The shared handler of the back and callback menu items: run the base
/// input handler, then (when enabled and not already consumed) pop the page
/// for the "back" code, or run the item's callback.
pub(crate) fn menu_item_handle_input(c: &mut Cpu, entry: u32, callback: bool) -> Flow {
    c.cycles += 11;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    call_ptr(c, 0x002c_a2b0, entry + 0x18)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x002c_a3b0, entry + 0x24)?;
    let disabled = c.g[V0][0] != 0;
    c.g[V0][0] = c.g[S1][0];
    let mut restored_early = false;
    if !disabled {
        c.cycles += 2;
        if c.g[S1][0] != 0 {
            frame.restore(c, S0, 0x20);
            restored_early = true;
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002c_a3e8, entry + 0x3c)?;
            c.g[V1][0] = 6;
            let not_back = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = c.g[S1][0];
            if !not_back {
                c.cycles += 4;
                if callback {
                    c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x18)).lw(c);
                    let target = c.g[V0][0] as u32;
                    call_ptr(c, target, entry + 0x54)?;
                    c.g[V0][0] = c.g[S1][0];
                } else {
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x124)).lw(c);
                    call_ptr(c, 0x002c_c460, entry + 0x54)?;
                    c.g[V0][0] = c.g[S1][0];
                }
            }
        }
    }
    if !restored_early {
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `MenuSysBackItem::HandleInput(self)` (0x2cce38): pop the current page when
/// the "back" input arrives.
pub fn menu_sys_back_item_handle_input(c: &mut Cpu) {
    guest(c, |c| menu_item_handle_input(c, 0x002c_ce38, false))
}

/// `MenuSysCallbackItem::HandleInput(self)` (0x2cd1d0): call the item's
/// callback when the "back" input arrives.
pub fn menu_sys_callback_item_handle_input(c: &mut Cpu) {
    guest(c, |c| menu_item_handle_input(c, 0x002c_d1d0, true))
}

/// `MenuSysTextEditItem* MenuSysTextEditItem_ctor(self, a1, name)`
/// (0x2ce418): base menu item plus the text-edit vtable and the name copy.
pub fn menu_sys_text_edit_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        c.g[A1][0] = u64::MAX;
        call_ptr(c, 0x002c_a258, 0x002c_e440)?;
        c.g[V1][0] = 0x480000;
        Addr(p(c.g[S0][0] as u32, 0x14)).sw(c, c.g[S1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ac0));
        Addr(p(c.g[S0][0] as u32, 0x28)).sw(c, 0);
        Addr(p(c.g[S0][0] as u32, 0x10)).sw(c, c.g[V1][0]);
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = sx(p(c.g[S0][0] as u32, 0x18));
        c.g[A2][0] = 0xf;
        call_ptr(c, 0x0041_6ce0, 0x002c_e464)?;
        c.g[V0][0] = c.g[S0][0];
        Addr(p(c.g[S0][0] as u32, 0x27)).sb(c, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MenuSysItem_HandleInput(MenuSysItem* item)` (0x2ca2b0): map the
/// current input code to an item action (8: 1, 7: 2, 0: 4, 1: 3; otherwise 0).
pub fn menu_sys_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MENU_ITEM_GET_INPUT_CODE.call(c, 0x002c_a2c0)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 1;
        let code_is_1 = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        let result: u64;
        if code_is_1 {
            // b_2ca310 -> +1, v0 = 3 -> b_2ca314
            c.cycles += 1;
            result = 3;
        } else if c.g[V0][0] == 0 {
            // code >= 2: fall into b_2ca300 with v0 = 7.
            c.cycles += 2;
            c.g[V0][0] = 7;
            c.cycles += 2;
            let is_7 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 8;
            if is_7 {
                c.cycles += 2;
                result = 2;
            } else {
                c.cycles += 2;
                let not_8 = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 0;
                if not_8 {
                    result = 0;
                } else {
                    c.cycles += 2;
                    result = 1;
                }
            }
        } else {
            // code == 0: b_2ca308 -> v0 = 4
            c.cycles += 2;
            c.g[V0][0] = 7;
            c.cycles += 2;
            c.cycles += 2;
            result = 4;
        }
        c.g[V0][0] = result;
        c.cycles += 1;
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}

/// `void MenuSysPage_OnActivate(MenuSysPage* page, int wrap)` (0x2cbf60):
/// store the wrap flag, clamp the selection to the last item, activate the
/// selected item (virtual `+0x34` with argument 2) and validate the selection.
pub fn menu_sys_page_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(p_2(r32(c, S0), 0x124)).sw(c, c.g[A1][0]);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = Addr(p_2(r32(c, S0), 4)).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = Addr(p_2(r32(c, S0), 4)).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            Addr(p_2(r32(c, S0), 4)).sw(c, c.g[V0][0]);
            c.g[V0][0] = Addr(p_2(r32(c, S0), 4)).lw(c);
        }
        c.cycles += 15;
        c.g[A1][0] = 2;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = abi::addu(c.g[S0][0], c.g[V0][0]);
        c.g[A2][0] = Addr(p_2(c.g[V0][0] as u32, 0xc)).lw(c);
        c.g[V1][0] = Addr(p_2(r32(c, A2), 0x10)).lw(c);
        c.g[A0][0] = Addr(p_2(c.g[V1][0] as u32, 0x30)).lh(c);
        c.g[V0][0] = Addr(p_2(c.g[V1][0] as u32, 0x34)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = abi::addu(c.g[A2][0], c.g[A0][0]);
        call::call_ptr(c, target, 0x002c_bfb8)?;
        c.g[A0][0] = c.g[S0][0];
        callee::MENU_PAGE_VALIDATE_SELECTION.call(c, 0x002c_bfc0)?;
        lqsp(c, 16, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `int MenuSysSubmenuItem_HandleInput(MenuSysSubmenuItem* item)`
/// (0x2ccd20): run the base item input, and when the item is enabled, produced
/// no action and the input code is 6 (confirm), push the submenu page named
/// at `item+0x18`.
pub fn menu_sys_submenu_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        sqsp(c, 16, 0x20);
        sqsp(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_ITEM_HANDLE_INPUT.call(c, 0x002c_cd38)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        callee::MENU_ITEM_IS_DISABLED.call(c, 0x002c_cd44)?;
        let disabled = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S1][0];
        let mut restore_16 = false;
        if !disabled {
            c.cycles += 2;
            if c.g[S1][0] != 0 {
                restore_16 = true;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                callee::MENU_ITEM_GET_INPUT_CODE.call(c, 0x002c_cd5c)?;
                c.g[V1][0] = 6;
                let not_confirm = c.g[V0][0] != c.g[V1][0];
                c.g[V0][0] = c.g[S1][0];
                if !not_confirm {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[A1][0] = Addr(p_2(r32(c, S0), 0x18)).lw(c);
                    c.g[A0][0] = Addr(p_2(c.g[V0][0] as u32, 0x124)).lw(c);
                    callee::MENU_PUSH_PAGE.call(c, 0x002c_cd78)?;
                    c.g[V0][0] = c.g[S1][0];
                }
            }
        }
        if !restore_16 {
            c.cycles += 1;
        }
        lqsp(c, 16, 0x20);
        c.cycles += 4;
        lqsp(c, 17, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `void MenuSysFloatItem_Draw(MenuSysFloatItem* item, int a1)` (0x2ce100):
/// format `*item->value * item->scale + item->bias` into a stack string, draw
/// the row, and destroy the temporary string.
pub fn menu_sys_float_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 16, 0x30);
        sqsp(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        sdsp(c, 0x10, c.g[RA][0]);
        c.g[A0][0] = c.g[SP][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = Addr(p_2(r32(c, S0), 0x18)).lw(c);
        c.lwc1(1, p_2(r32(c, S0), 0x24));
        c.lwc1(12, c.g[V0][0] as u32);
        c.lwc1(0, p_2(r32(c, S0), 0x28));
        c.mul_s(12, 12, 1);
        c.add_s(12, 12, 0);
        callee::MENU_FORMAT_FLOAT.call(c, 0x002c_e138)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[T0][0] = Addr(c.g[SP][0] as u32).lw(c);
        c.g[A2][0] = Addr(p_2(r32(c, A0), 0x14)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        callee::MENU_ITEM_DRAW_ROW.call(c, 0x002c_e150)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call::call_ptr(c, 0x0031_8260, 0x002c_e15c)?;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        c.g[RA][0] = Addr(p_2(c.g[SP][0] as u32, 0x10)).ld(c);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `void MenuSysFloatRangeItem_Draw(self, row)` (0x2ce3a0): format the item's
/// float (`*(self+0x18)` scaled by 180/pi) into a 0x70-byte buffer and draw
/// the row.
pub fn menu_sys_float_range_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0xb0));
        c.lwc1(0, gpo(c, -0x3fb4));
        c.sq(16, sp_2(c).wrapping_add(0xa0));
        c.sq(17, sp_2(c).wrapping_add(0x90));
        c.g[S0][0] = 0x4a_0000;
        c.sq(18, sp_2(c).wrapping_add(0x80));
        c.g[S1][0] = c.g[A0][0];
        c.sd(sp_2(c).wrapping_add(0x70), c.g[RA][0]);
        c.g[S2][0] = c.g[A1][0];
        c.g[S0][0] = sx(low(c, S0).wrapping_add(0x39c8));
        c.g[V0][0] = lw_r(c, S1, 0x18);
        c.lwc1(12, low(c, V0));
        c.mul_s(12, 12, 0);
        call_ptr(c, callee::CVT_FLOAT_TO_INT, 0x002c_e3d8)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, callee::SPRINTF, 0x002c_e3e8)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = lw_r(c, A0, 0x14);
        c.g[A3][0] = 0;
        c.g[T0][0] = c.g[SP][0];
        callee::MENU_SYS_ITEM_DRAW_ROW_4.call(c, 0x002c_e400)?;
        c.lq(16, sp_2(c).wrapping_add(0xa0));
        c.lq(17, sp_2(c).wrapping_add(0x90));
        c.lq(18, sp_2(c).wrapping_add(0x80));
        c.g[RA][0] = Addr(sp_2(c).wrapping_add(0x70)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0xb0));
        c.pc = t;
        Ok(())
    })
}

/// `void MenuSysGroupItem_OnFocus(self, mode)` (0x002cfe78): select the
/// group's page (0 = first, 1 = last) and activate it.
pub fn menu_sys_group_item_on_focus(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_ON_FOCUS_2.call(c, 0x002c_fe94)?;
        let mut select: Option<u64> = None;
        let s0 = c.g[S0][0];
        c.g[V0][0] = 0x1;
        if s0 == 0 {
            c.cycles += 4;
            c.g[S0][0] = sx(word(c, S1).wrapping_add(0x18));
            select = Some(0);
        } else {
            c.cycles += 2;
            c.g[A0][0] = sx(word(c, S1).wrapping_add(0x18));
            if s0 == 1 {
                c.cycles += 3;
                c.g[S0][0] = sx(word(c, S1).wrapping_add(0x18));
                select = Some(0xffffffffffffffff);
            } else {
                c.cycles += 2;
                c.g[V0][0] = Addr(at_2(c, S1, 0)).lw(c);
            }
        }
        if let Some(arg) = select {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = arg;
            callee::MENU_SYS_PAGE_SET_SELECTION.call(c, 0x002c_fed0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[V0][0] = Addr(at_2(c, S1, 0)).lw(c);
        }
        c.cycles += 7;
        c.g[RA][0] = sx(0x002c_fee0);
        c.g[A1][0] = Addr(at_2(c, V0, 0x124)).lw(c);
        callee::MENU_SYS_PAGE_ON_ACTIVATE.call(c, 0x002c_fee0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysIntItem* MenuSysIntItem_ctor_3(MenuSysIntItem* p, int a1, int a2,
/// int a3, int t0, int t1)` (0x002cd8f8): base menu item, class vtable, then
/// the extra fields.
pub fn menu_sys_int_item_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x60));
        c.sq(S0, sp(c).wrapping_add(0x50));
        c.g[S0][0] = c.g[A0][0];
        c.sq(S1, sp(c).wrapping_add(0x40));
        c.sq(S2, sp(c).wrapping_add(0x30));
        c.g[S1][0] = c.g[T1][0];
        c.sq(S3, sp(c).wrapping_add(0x20));
        c.g[S2][0] = c.g[A2][0];
        c.sq(S4, sp(c).wrapping_add(0x10));
        c.g[S3][0] = c.g[A3][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S4][0] = c.g[T0][0];
        call_ptr(c, 0x002c_a258, 0x002c_d92c)?; // MenuSysItem_ctor
        c.g[V1][0] = sx(0x0048_6bb0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[S3][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[S4][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), 0);
        c.lq(S0, sp(c).wrapping_add(0x50));
        c.lq(S1, sp(c).wrapping_add(0x40));
        c.lq(S2, sp(c).wrapping_add(0x30));
        c.lq(S3, sp(c).wrapping_add(0x20));
        c.lq(S4, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x60));
        abi::ret(c);
        Ok(())
    })
}

/// `void MenuSysColorSwatchItem_Measure(MenuSysColorSwatchItem* item,
/// MenuSysItem* row)` (0x002cff88): measure the base row, pad its height by 9
/// and floor it at 29, then store the 16-byte extent into the item.
pub fn menu_sys_color_swatch_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.g[A3][0] = 0;
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[T0][0] = 0;
        c.sd(sp(c).wrapping_add(0x10), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x002c_a4c8, 0x002c_ffac)?; // MenuSysItem_MeasureRow
        c.lwc1(2, sp(c).wrapping_add(8));
        c.g[V0][0] = c.g[S0][0];
        c.g[AT][0] = 0x4210_0000;
        c.mtc1(1, 3);
        c.lwc1(1, sp(c).wrapping_add(0xc));
        c.add_s(2, 2, 3);
        c.g[AT][0] = 0x41e8_0000;
        c.mtc1(1, 0);
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0x10));
        c.max_s(1, 1, 0);
        c.swc1(2, sp(c).wrapping_add(8));
        c.swc1(1, sp(c).wrapping_add(0xc));
        c.ldl(3, sp(c).wrapping_add(7));
        c.ldr(3, sp(c));
        c.ldl(4, sp(c).wrapping_add(0xf));
        c.ldr(4, sp(c).wrapping_add(8));
        c.sdl(3, (c.g[S0][0] as u32).wrapping_add(7));
        c.sdr(3, c.g[S0][0] as u32);
        c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0xf));
        c.sdr(4, (c.g[S0][0] as u32).wrapping_add(8));
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `void MenuSysPage_HandleInput(page)` (0x2cacc0): let the selected item
/// handle its input, then move the selection for the "next"/"previous" codes
/// (3 and 4); returns 0 for the unhandled case.
pub fn menusyspage_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_PAGE_VALIDATE_SELECTION_2.call(c, 0x002c_acd4)?;
        let base = r32(c, S0);
        c.g[V0][0] = lw_at_2(c, p(base, 4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = lw_at_2(c, p(c.g[V0][0] as u32, 0xc));
        c.g[V1][0] = lw_at_2(c, p(c.g[A1][0] as u32, 0x10));
        let func = vslot(c, c.g[V1][0], 0x10, V0);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, 0x002c_acf8)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 3;
        let next = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 4;
        let mut handled = true;
        if next {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::MENU_SYS_PAGE_SELECT_NEXT_2.call(c, 0x002c_ad20)?;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 2;
            let prev = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = c.g[V1][0];
            if prev {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                callee::MENU_SYS_PAGE_SELECT_PREV.call(c, 0x002c_ad30)?;
                c.g[V0][0] = 0;
            } else {
                c.cycles += 2;
                frame.restore(c, 16, 0x10);
                handled = false;
            }
        }
        if handled {
            c.cycles += 1;
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPage_InsertItem(page, item, index)` (0x2cbcc8): insert the
/// item into the page's item array at `index`, shifting the rest up, and
/// keep the item count and the selected index consistent.
pub fn menusyspage_insert_item(c: &mut Cpu) {
    c.cycles += 2;
    let item = c.g[A0][0];
    let page = c.g[A1][0];
    let mut index = c.g[A2][0];
    let given = (index as i64) >= 0;
    c.sw(page as u32, item);
    if !given {
        c.cycles += 1;
        index = lw_at_2(c, item as u32);
        c.g[A2][0] = index;
    }
    c.g[A2][0] = index;
    c.cycles += 5;
    let count = lw_at_2(c, item as u32);
    c.g[T0][0] = count;
    c.g[T1][0] = sx((item as u32).wrapping_add(0xc));
    c.g[V0][0] = ((index as i64) < (count as i64)) as u64;
    let fits = c.g[V0][0] != 0;
    c.g[T2][0] = sx((index as u32) << 2);
    if fits {
        c.cycles += 2;
        c.g[V0][0] = sx((count as u32) << 2);
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
        loop {
            c.cycles += 7;
            c.g[V0][0] = lw_at_2(c, (c.g[A3][0] as u32).wrapping_sub(4));
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(1));
            c.g[V1][0] = ((index as i64) < (c.g[T0][0] as i64)) as u64;
            c.sw(c.g[A3][0] as u32, c.g[V0][0]);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(4));
            if c.g[V1][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_bcf0);
            }
        }
    }
    c.cycles += 5;
    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[T2][0] as u32));
    c.sw(c.g[V0][0] as u32, c.g[A1][0]);
    c.g[V1][0] = lw_at_2(c, p(item as u32, 0x124));
    if c.g[V1][0] == 0 {
        c.g[V0][0] = lw_at_2(c, item as u32);
    } else {
        c.cycles += 4;
        c.g[V1][0] = lw_at_2(c, p(item as u32, 4));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (index as i64)) as u64;
        let after = c.g[V0][0] != 0;
        c.g[V0][0] = lw_at_2(c, item as u32);
        if !after {
            c.cycles += 2;
            let empty = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            if !empty {
                c.cycles += 1;
                c.sw(p(item as u32, 4), c.g[V0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = lw_at_2(c, item as u32);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.sw(item as u32, c.g[V0][0]);
    abi::ret(c);
}

/// `void MenuSysActionItem_HandleInput(item)` (0x2cd008): the action item's
/// input: run the item input, then the action when it was triggered.
pub fn menusysactionitem_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_HANDLE_INPUT.call(c, 0x002c_d020)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        callee::MENU_SYS_ITEM_IS_DISABLED_2.call(c, 0x002c_d02c)?;
        let disabled = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S1][0];
        let mut skip_one = false;
        if !disabled {
            c.cycles += 2;
            if c.g[S1][0] != 0 {
                frame.restore(c, 16, 0x20);
                skip_one = true;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                callee::MENU_SYS_ITEM_GET_INPUT_CODE.call(c, 0x002c_d044)?;
                c.g[V1][0] = 6;
                let other = c.g[V0][0] != c.g[V1][0];
                c.g[V0][0] = c.g[S1][0];
                if other {
                } else {
                    c.cycles += 10;
                    c.g[A3][0] = lw_at_2(c, c.g[S0][0] as u32);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = lw_at_2(c, p(c.g[A1][0] as u32, 0x1c));
                    c.g[V1][0] = lw_at_2(c, p(c.g[A3][0] as u32, 0x12c));
                    let func = vslot(c, c.g[V1][0], 0x18, V0);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, func, 0x002c_d070)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V0][0] = c.g[S1][0];
                }
            }
        }
        if !skip_one {
            c.cycles += 1;
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
// Menu items.

/// The entry point and the loop head (where an event resumes) of
/// [`menu_sys_enum_item_draw`].
pub(crate) const ENUM_DRAW_ENTRY: u32 = 0x002c_cc38;

pub(crate) const ENUM_DRAW_HEAD: u32 = 0x002c_cc58;

/// `void MenuSysEnumItem_Draw(MenuSysEnumItem* self, void* a1)` (0x2ccc38):
/// walk the item's enum table and draw the row whose value matches, or the
/// item's own last row when there is no match.
///
/// The original is a `match pc` dispatcher: its loop head is a resume point
/// (an event delivered at the back edge returns and the trampoline re-enters
/// the function there).
pub fn menu_sys_enum_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                ENUM_DRAW_ENTRY => {
                    c.cycles += 7;
                    let frame = Frame::open(c, 0x10);
                    c.g[A2][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[T1][0] = c.g[A1][0];
                    c.g[A3][0] = 0;
                    c.g[T2][0] = 0x0048_0000;
                    pc = 0x002c_cc94;
                }
                0x002c_cc54 => {
                    c.cycles += 1;
                    pc = ENUM_DRAW_HEAD;
                }
                ENUM_DRAW_HEAD => {
                    c.cycles += 7;
                    c.g[V0][0] = sx(r32(c, A3) << 3);
                    c.g[V1][0] = Addr(p(r32(c, A2), 0x20)).lw(c);
                    c.g[T0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
                    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
                    c.g[V0][0] = Addr(p(r32(c, T0), 4)).lw(c);
                    let mismatch = c.g[A0][0] != c.g[V0][0];
                    c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                    pc = if mismatch { 0x002c_cc94 } else { 0x002c_cc74 };
                }
                0x002c_cc74 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[A2][0];
                    c.g[T0][0] = Addr(r32(c, T0)).lw(c);
                    c.g[A2][0] = Addr(p(r32(c, A0), 0x14)).lw(c);
                    c.g[A1][0] = c.g[T1][0];
                    c.g[A3][0] = 0;
                    callee::MENU_SYS_ITEM_DRAW_ROW_5.call(c, 0x002c_cc8c)?;
                    c.g[RA][0] = Addr(r32(c, SP)).ld(c);
                    pc = 0x002c_ccbc;
                }
                0x002c_cc94 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(r32(c, A2), 0x18)).lw(c);
                    if c.g[A3][0] != c.g[V0][0] {
                        c.g[A1][0] = Addr(p(r32(c, A2), 0x1c)).lw(c);
                        if crate::events::back_edge_unwinds(c, ENUM_DRAW_HEAD) {
                            return Ok(());
                        }
                        pc = ENUM_DRAW_HEAD;
                    } else {
                        pc = 0x002c_cca0;
                    }
                }
                0x002c_cca0 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[A2][0];
                    c.g[A1][0] = c.g[T1][0];
                    c.g[A2][0] = Addr(p(r32(c, A0), 0x14)).lw(c);
                    c.g[T0][0] = sx(r32(c, T2).wrapping_add(0x66b8));
                    c.g[A3][0] = 0;
                    callee::MENU_SYS_ITEM_DRAW_ROW_5.call(c, 0x002c_ccb8)?;
                    c.g[RA][0] = Addr(r32(c, SP)).ld(c);
                    pc = 0x002c_ccbc;
                }
                0x002c_ccbc => {
                    c.cycles += 2;
                    let ra = r32(c, RA);
                    c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x10));
                    c.pc = ra;
                    return Ok(());
                }
                other => {
                    c.jump_out(other, ENUM_DRAW_ENTRY, 0x002c_ccc4);
                    return Ok(());
                }
            }
        }
    })
}

/// `cFloatMenuItem* cFloatMenuItem_ctor(cFloatMenuItem* self, void* text,
/// int a2, int a3, float f1, float f2)` (0x2cdce8): base constructor, then the
/// float item's fields, vtable and unit scale.
pub fn menu_sys_float_item_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A3][0];
        c.swc1(21, p(r32(c, SP), 0x48));
        c.g[S2][0] = c.g[A2][0];
        c.swc1(20, p(r32(c, SP), 0x40));
        c.mov_s(21, 13);
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        callee::MENU_SYS_ITEM_CTOR_4.call(c, 0x002c_dd1c)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[V1][0] = 0x0048_0000;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x6b60));
        Addr(p(r32(c, S0), 0x14)).sw(c, c.g[S1][0]);
        Addr(p(r32(c, S0), 0x18)).sw(c, c.g[S2][0]);
        c.g[V0][0] = c.g[S0][0];
        c.swc1(20, p(r32(c, S0), 0x1c));
        c.swc1(21, p(r32(c, S0), 0x20));
        Addr(p(r32(c, S0), 0x10)).sw(c, c.g[V1][0]);
        c.swc1(0, p(r32(c, S0), 0x24));
        Addr(p(r32(c, S0), 0x30)).sw(c, 0);
        Addr(p(r32(c, S0), 0x2c)).sw(c, 0);
        Addr(p(r32(c, S0), 0x28)).sw(c, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        c.lwc1(21, p(r32(c, SP), 0x48));
        c.lwc1(20, p(r32(c, SP), 0x40));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menu pages.

/// `void MenuSysPage_SetSelection(MenuSysPage* page, int index)` (0x2cbe68):
/// normalise a negative index by the item count, and when the index changed,
/// run the old item's deselect method, store the new index, run the new item's
/// select method and validate the page.
pub fn menu_sys_page_set_selection(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        let non_negative = (c.g[S1][0] as i64) >= 0;
        frame.save_ra(c, 0);
        if !non_negative {
            c.cycles += 2;
            c.g[V0][0] = lw(c, c.g[S2][0] as u32);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        }
        c.cycles += 3;
        c.g[V0][0] = lw(c, p(c.g[S2][0] as u32, 4));
        let same = c.g[S1][0] == c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if !same {
            c.cycles += 20;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = lw(c, c.g[V0][0] as u32);
            c.g[V1][0] = lw(c, p(c.g[A1][0] as u32, 0x10));
            c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x38)).lh(c);
            c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x3c));
            c.g[V0][0] = sx(c.g[V0][0] as u32);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V0][0] as u32, 0x002c_bebc)?;
            sw(c, p(c.g[S2][0] as u32, 4), c.g[S1][0]);
            c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = 2;
            c.g[A2][0] = lw(c, c.g[S0][0] as u32);
            c.g[V0][0] = lw(c, p(c.g[A2][0] as u32, 0x10));
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x30)).lh(c);
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x34));
            c.g[V1][0] = sx(c.g[V1][0] as u32);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x002c_bee4)?;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x002c_aab0, 0x002c_beec)?; // MenuSysPage_ValidateSelection
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

/// `void MenuSys_PopPage(self)` (0x2cc460): dispose the top page and, if one
/// remains, activate its parent; otherwise clear the menu's active flag.
pub fn menu_sys_pop_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(this.wrapping_add(4));
        c.g[V0][0] = Addr(this).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        Addr(this).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[V0][0] = lw_2(c, A1, 0x12c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x38).lh(c);
        c.g[V1][0] = lw_2(c, V0, 0x3c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002c_c4a4)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        if !empty {
            c.cycles += 12;
            c.g[A1][0] = c.g[S0][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V1][0] = lw_2(c, A3, 0x12c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x30).lh(c);
            c.g[A2][0] = lw_2(c, V1, 0x34);
            let target = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x002c_c4d4)?;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x20);
                c.cycles += 4;
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        callee::MENU_SYS_SET_ACTIVE.call(c, 0x002c_c4ec)?;
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menu system: push a page item.

/// `void MenuSys_PushPage(MenuSys* menu, item)` (0x2cc3b8): notify the item
/// below the page, append the item to the page's array, notify it with the new
/// index and mark the menu active.
pub fn menu_sys_push_page(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        c.cycles += 8;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_3(c, c.g[S0][0] as u32);
        let t = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !t {
            c.cycles += 9;
            c.g[V0][0] = sx(at_4(c.g[V0][0], 0xffff_ffff));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = lw_3(c, at_4(c.g[V0][0], 4));
            c.g[V1][0] = lw_3(c, at_4(c.g[A1][0], 0x12c));
            c.g[A0][0] = lh(c, at_4(c.g[V1][0], 0x38));
            c.g[V0][0] = lw_3(c, at_4(c.g[V1][0], 0x3c));
            let target = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x002c_c3fc);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x002c_c3fc)?;
        }
        c.cycles += 25;
        c.g[A0][0] = lw_3(c, c.g[S0][0] as u32);
        c.g[A2][0] = sx(at_4(c.g[S0][0], 4));
        c.g[A1][0] = c.g[S0][0];
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = sx(at_4(c.g[A0][0], 1));
        Addr(c.g[V0][0] as u32).sw(c, c.g[S1][0]);
        c.g[V1][0] = sx(at_4(c.g[A0][0], 0xffff_ffff));
        Addr(c.g[S0][0] as u32).sw(c, c.g[A0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A3][0] = lw_3(c, c.g[A2][0] as u32);
        c.g[V0][0] = lw_3(c, at_4(c.g[A3][0], 0x12c));
        c.g[A0][0] = lh(c, at_4(c.g[V0][0], 0x30));
        c.g[V1][0] = lw_3(c, at_4(c.g[V0][0], 0x34));
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x002c_c440);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002c_c440)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x002c_c44c);
        c.g[A1][0] = 1;
        callee::MENU_SYS_SET_ACTIVE_2.call(c, 0x002c_c44c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menu system: vector-label item draw.

/// `void MenuSysVectorLabelItem_Draw(item, ctx)` (0x2d0500): convert the
/// label's position to three doubles for `sprintf`, then draw the row.
pub fn menu_sys_vector_label_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0xe0);
        c.cycles += 42;
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        c.g[S0][0] = sx(0x0048_0000);
        frame.save(c, S2, 0xb0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0xa0);
        c.g[S0][0] = sx(0x0048_6720);
        frame.save(c, S4, 0x90);
        frame.save_ra(c, 0x80);
        c.g[S4][0] = c.g[A1][0];
        c.g[V0][0] = lw_3(c, at_4(c.g[S1][0], 0x14));
        c.g[RA][0] = sx(0x002d_0538);
        c.lwc1(12, c.g[V0][0] as u32);
        call_ptr(c, 0x0041_3af8, 0x002d_0538)?;
        c.g[V1][0] = lw_3(c, at_4(c.g[S1][0], 0x14));
        c.g[S3][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x002d_0548);
        c.lwc1(12, at_4(c.g[V1][0], 4));
        call_ptr(c, 0x0041_3af8, 0x002d_0548)?;
        c.g[V1][0] = lw_3(c, at_4(c.g[S1][0], 0x14));
        c.g[S2][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x002d_0558);
        c.lwc1(12, at_4(c.g[V1][0], 8));
        call_ptr(c, 0x0041_3af8, 0x002d_0558)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[A0][0] = sx(frame.at(c, 0));
        c.g[RA][0] = sx(0x002d_0570);
        c.g[T0][0] = c.g[V0][0];
        call_ptr(c, 0x0041_7828, 0x002d_0570)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = lw_3(c, at_4(c.g[A0][0], 0x18));
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x002d_0588);
        c.g[T0][0] = sx(frame.at(c, 0));
        callee::MENU_SYS_ITEM_DRAW_ROW.call(c, 0x002d_0588)?;
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menu system: vector-label item measure.

/// `void MenuSysVectorLabelItem_Measure(item, ctx)` (0x2d0450): format the
/// label's position and measure the row.
pub fn menu_sys_vector_label_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0xe0);
        c.cycles += 43;
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        c.g[S0][0] = sx(0x0048_0000);
        frame.save(c, S2, 0xb0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0xa0);
        c.g[S0][0] = sx(0x0048_6720);
        frame.save(c, S4, 0x90);
        frame.save_ra(c, 0x80);
        c.g[S4][0] = c.g[A0][0];
        c.g[V0][0] = lw_3(c, at_4(c.g[S1][0], 0x14));
        c.g[RA][0] = sx(0x002d_0488);
        c.lwc1(12, c.g[V0][0] as u32);
        call_ptr(c, 0x0041_3af8, 0x002d_0488)?;
        c.g[V1][0] = lw_3(c, at_4(c.g[S1][0], 0x14));
        c.g[S3][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x002d_0498);
        c.lwc1(12, at_4(c.g[V1][0], 4));
        call_ptr(c, 0x0041_3af8, 0x002d_0498)?;
        c.g[V1][0] = lw_3(c, at_4(c.g[S1][0], 0x14));
        c.g[S2][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x002d_04a8);
        c.lwc1(12, at_4(c.g[V1][0], 8));
        call_ptr(c, 0x0041_3af8, 0x002d_04a8)?;
        c.g[A0][0] = sx(frame.at(c, 0));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x002d_04c0);
        c.g[T0][0] = c.g[V0][0];
        call_ptr(c, 0x0041_7828, 0x002d_04c0)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S4][0];
        c.g[A2][0] = lw_3(c, at_4(c.g[A1][0], 0x18));
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x002d_04d8);
        c.g[T0][0] = sx(frame.at(c, 0));
        callee::MENU_SYS_ITEM_MEASURE_ROW.call(c, 0x002d_04d8)?;
        c.g[V0][0] = c.g[S4][0];
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `int MenuSysBoolItem_HandleInput(MenuSysBoolItem *self)` (0x2cc578):
/// toggles the item's bool when the handled input is confirm (2/3) or the
/// item's own code is 6 (and the item is enabled), then notifies the owner.
pub fn menu_sys_bool_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_HANDLE_INPUT.call(c, 0x002c_c590)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        callee::MENU_SYS_ITEM_IS_DISABLED_3.call(c, 0x002c_c59c)?;
        let disabled = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S1][0];
        if !disabled {
            c.cycles += 2;
            if c.g[S1][0] != 0 {
                c.lq(16, frame.at(c, 0x20));
            } else {
                let mut toggle = false;
                let mut value_ptr = 0u64;
                c.cycles += 3;
                if Addr(p_3(c.g[S0][0], 0x18)).lw(c) != 0 {
                    c.cycles += 3;
                    if Addr(p_3(c.g[S0][0], 0x1c)).lw(c) != 0 {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        callee::MENU_SYS_ITEM_GET_INPUT_CODE.call(c, 0x002c_c5cc)?;
                        c.g[V1][0] = 0x2;
                        if c.g[V0][0] == c.g[V1][0] {
                            c.g[V1][0] = Addr(p_3(c.g[S0][0], 0x20)).lw(c);
                            value_ptr = c.g[V1][0];
                            toggle = true;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S0][0];
                            callee::MENU_SYS_ITEM_GET_INPUT_CODE.call(c, 0x002c_c5e0)?;
                            c.g[V1][0] = 0x3;
                            if c.g[V0][0] == c.g[V1][0] {
                                c.g[V1][0] = Addr(p_3(c.g[S0][0], 0x20)).lw(c);
                                value_ptr = c.g[V1][0];
                                toggle = true;
                            }
                        }
                    }
                }
                if !toggle {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    callee::MENU_SYS_ITEM_GET_INPUT_CODE.call(c, 0x002c_c5f4)?;
                    c.g[V1][0] = 0x6;
                    let other = c.g[V0][0] != c.g[V1][0];
                    c.g[V0][0] = c.g[S1][0];
                    if !other {
                        c.cycles += 1;
                        c.g[V1][0] = Addr(p_3(c.g[S0][0], 0x20)).lw(c);
                        value_ptr = c.g[V1][0];
                        toggle = true;
                    }
                }
                if toggle {
                    c.cycles += 11;
                    c.g[A1][0] = c.g[S0][0];
                    let value = Addr(value_ptr as u32).lw(c) ^ 1;
                    Addr(value_ptr as u32).sw(c, value);
                    c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[V0][0] = Addr(p_3(c.g[A2][0], 0x12c)).lw(c);
                    c.g[A0][0] = Addr(p_3(c.g[V0][0], 0x20)).lh(c);
                    c.g[V1][0] = Addr(p_3(c.g[V0][0], 0x24)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x002c_c62c)?;
                    c.g[V0][0] = c.g[S1][0];
                }
                c.cycles += 1;
                c.lq(16, frame.at(c, 0x20));
            }
        } else {
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

/// `void MenuSysPage_SelectNext(MenuSysPage* self)` (0x2cab08): advance the
/// page cursor modulo the item count and select the next item that accepts
/// input; when the cursor wraps back to the start, do nothing.
pub fn menu_sys_page_select_next(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S3][0] = sx(p(r32(c, S0), 0xc));
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = Addr(p(r32(c, S0), 4)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
        'page: loop {
            c.cycles += 5;
            c.g[V1][0] = Addr(r32(c, S0)).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            c.div(0, 2, 3);
            c.cycles += 3;
            c.g[V0][0] = c.hi[0];
            let wrapped = c.g[V0][0] == c.g[S1][0];
            Addr(p(r32(c, S0), 4)).sw(c, c.g[V0][0]);
            if wrapped {
                break;
            }
            c.cycles += 11;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[S2][0] = sx(p(r32(c, S0), 0xc));
            c.g[V0][0] = sx(p(r32(c, S3), r32(c, V0)));
            c.g[A1][0] = Addr(r32(c, V0)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, A1), 0x10)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x18)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x1c)).lw(c);
            let accepts = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
            call_ptr(c, accepts, 0x002c_ab74)?;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x002c_ab30);
                }
                continue 'page;
            }
            c.cycles += 18;
            c.g[V0][0] = sx(r32(c, S1) << 2);
            c.g[V0][0] = sx(p(r32(c, S2), r32(c, V0)));
            c.g[A2][0] = Addr(r32(c, V0)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, A2), 0x10)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x38)).lh(c);
            c.g[A1][0] = Addr(p(r32(c, V1), 0x3c)).lw(c);
            let sel = r32(c, A1);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, sel, 0x002c_ab9c)?;
            c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
            c.g[A1][0] = 0;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(p(r32(c, S2), r32(c, V0)));
            c.g[A2][0] = Addr(r32(c, V0)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, A2), 0x10)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x30)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x34)).lw(c);
            let hov = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, hov, 0x002c_abc4)?;
            break;
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
// Menu page navigation (previous).

/// `void MenuSysPage_SelectPrev(MenuSysPage* self)` (0x2cabe0): move the page
/// cursor back modulo the item count and select the previous valid item.
pub fn menu_sys_page_select_prev(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S3][0] = sx(p(r32(c, S0), 0xc));
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = Addr(p(r32(c, S0), 4)).lw(c);
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        'page: loop {
            c.cycles += 3;
            c.g[V1][0] = Addr(p(r32(c, S0), 4)).lw(c);
            c.cycles += 6;
            c.g[V1][0] = sx(p(r32(c, V1), r32(c, V0)));
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
            c.div(0, 3, 2);
            c.g[V0][0] = c.hi[0];
            let wrapped = c.g[V0][0] == c.g[S1][0];
            Addr(p(r32(c, S0), 4)).sw(c, c.g[V0][0]);
            if wrapped {
                break;
            }
            c.cycles += 11;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[S2][0] = sx(p(r32(c, S0), 0xc));
            c.g[V0][0] = sx(p(r32(c, S3), r32(c, V0)));
            c.g[A1][0] = Addr(r32(c, V0)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, A1), 0x10)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x18)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x1c)).lw(c);
            let accepts = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
            call_ptr(c, accepts, 0x002c_ac50)?;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x002c_ac08);
                }
                continue 'page;
            }
            c.cycles += 18;
            c.g[V0][0] = sx(r32(c, S1) << 2);
            c.g[V0][0] = sx(p(r32(c, S2), r32(c, V0)));
            c.g[A2][0] = Addr(r32(c, V0)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, A2), 0x10)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x38)).lh(c);
            c.g[A1][0] = Addr(p(r32(c, V1), 0x3c)).lw(c);
            let sel = r32(c, A1);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, sel, 0x002c_ac78)?;
            c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
            c.g[A1][0] = 1;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(p(r32(c, S2), r32(c, V0)));
            c.g[A2][0] = Addr(r32(c, V0)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, A2), 0x10)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x30)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x34)).lw(c);
            let hov = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, hov, 0x002c_aca0)?;
            break;
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

/// `void MenuSys_BuildQuad(sVtx* v, const float* r, const float* t)`
/// (0x2cb350): fill a menu quad's vertices from the rectangle (`r` = x, y,
/// w, h) and the transform (`t` = x, y, z, w), then hand it to the draw
/// module's submit routine (slot at `*(D_004A289C+0x10d8) + 0x23c`, `this`
/// adjusted by the slot's `lh` delta).
pub fn menu_sys_build_quad(c: &mut Cpu) {
    c.cycles += 82;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    c.g[V1][0] = c.g[A1][0];
    c.std(SP, 0x10, c.g[RA][0]);
    c.g[T0][0] = c.g[A2][0];
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 3);
    c.g[A2][0] = c.g[A0][0];
    c.lwc1(0, c.pa(V1, 0));
    c.g[A1][0] = 0x4;
    c.lwc1(1, c.pa(V1, 0x4));
    c.g[A3][0] = 0u64;
    c.swc1(3, c.pa(SP, 0xc));
    c.swc1(0, c.pa(SP, 0));
    c.stw_2(SP, 0x8, 0u64);
    c.swc1(1, c.pa(SP, 0x4));
    c.lq(2, c.pa(SP, 0));
    c.stw_2(SP, 0x8, 0u64);
    c.sq(2, c.pa(A0, 0x20));
    c.swc1(3, c.pa(SP, 0xc));
    c.lwc1(0, c.pa(T0, 0));
    c.swc1(0, c.pa(A0, 0));
    c.lwc1(1, c.pa(T0, 0x4));
    c.swc1(1, c.pa(A0, 0x4));
    c.lwc1(1, c.pa(V1, 0x8));
    c.lwc1(0, c.pa(V1, 0));
    c.lwc1(2, c.pa(V1, 0x4));
    c.add_s(0, 0, 1);
    c.swc1(2, c.pa(SP, 0x4));
    c.swc1(0, c.pa(SP, 0));
    c.lq(2, c.pa(SP, 0));
    c.stw_2(SP, 0x8, 0u64);
    c.sq(2, c.pa(A0, 0x50));
    c.swc1(3, c.pa(SP, 0xc));
    c.lwc1(1, c.pa(T0, 0x8));
    c.lwc1(0, c.pa(T0, 0));
    c.add_s(0, 0, 1);
    c.swc1(0, c.pa(A0, 0x30));
    c.lwc1(1, c.pa(T0, 0x4));
    c.swc1(1, c.pa(A0, 0x34));
    c.lwc1(1, c.pa(V1, 0xc));
    c.lwc1(0, c.pa(V1, 0x4));
    c.lwc1(2, c.pa(V1, 0));
    c.add_s(0, 0, 1);
    c.swc1(2, c.pa(SP, 0));
    c.swc1(0, c.pa(SP, 0x4));
    c.lq(2, c.pa(SP, 0));
    c.sq(2, c.pa(A0, 0x80));
    c.lwc1(0, c.pa(T0, 0));
    c.swc1(3, c.pa(SP, 0xc));
    c.swc1(0, c.pa(A0, 0x60));
    c.stw_2(SP, 0x8, 0u64);
    c.g[T1][0] = c.ldw_2(GP, -0x854);
    c.lwc1(1, c.pa(T0, 0xc));
    c.lwc1(0, c.pa(T0, 0x4));
    c.add_s(0, 0, 1);
    c.swc1(0, c.pa(A0, 0x64));
    c.lwc1(3, c.pa(V1, 0xc));
    c.lwc1(2, c.pa(V1, 0x8));
    c.lwc1(1, c.pa(V1, 0));
    c.lwc1(0, c.pa(V1, 0x4));
    c.add_s(1, 1, 2);
    c.add_s(0, 0, 3);
    c.swc1(1, c.pa(SP, 0));
    c.swc1(0, c.pa(SP, 0x4));
    c.lq(2, c.pa(SP, 0));
    c.sq(2, c.pa(A0, 0xb0));
    c.lwc1(1, c.pa(T0, 0x8));
    c.lwc1(0, c.pa(T0, 0));
    c.add_s(0, 0, 1);
    c.swc1(0, c.pa(A0, 0x90));
    c.lwc1(1, c.pa(T0, 0xc));
    c.lwc1(0, c.pa(T0, 0x4));
    c.add_s(0, 0, 1);
    c.swc1(0, c.pa(A0, 0x94));
    c.g[V0][0] = c.ldw_2(T1, 0x10d8);
    c.g[A0][0] = c.ldh_2(V0, 0x238);
    c.g[V1][0] = c.ldw_2(V0, 0x23c);
    let t = c.g[V1][0] as u32;
    c.g[RA][0] = sx(0x002cb48c);
    c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.pc = t;
    call_indirect(c, t);
    if c.pc != 0x002cb48c {
        return;
    }
    c.g[RA][0] = c.ldd(SP, 0x10);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = ra;
    return;
}

/// A `gp_5`-relative global.
#[inline(always)]
pub(crate) fn gp_5(c: &Cpu, off: i32) -> u32 {
    crate::abi::r32(c, GP).wrapping_add(off as u32)
}

/// The menu-system singleton (`gp_5 - 0x854`).
#[inline(always)]
pub(crate) fn menu_sys(c: &mut Cpu) -> u32 {
    c.lw(gp_5(c, -0x854)) as u32
}

// ---------------------------------------------------------------------------
// 0x002cb180 MenuSysPage_BeginDraw
// ---------------------------------------------------------------------------

/// `void MenuSysPage_BeginDraw(void* page)` (0x2cb180).
///
/// Pushes the twenty-byte draw state of the current menu object (the page
/// pointer at `menu_sys + 0xe84`) onto that object's save stack, then runs the
/// four renderer passes the interface singleton's dispatch table holds:
/// viewport, scissor, background and font. The `v0` result is the caller's
/// (the last pass's return value is left in the register).
pub fn menu_sys_page_begin_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 94;
        let frame = Frame::open(c, 0x10);
        let ms = menu_sys(c);
        c.g[A0][0] = sx(ms);
        frame.save_ra(c, 0);

        // Push the 20-byte state at obj[0..0x14) onto obj[0x14..0x28).
        let obj = c.lw(p(ms, 0xe84)) as u32;
        c.ldl(3, p(obj, 0x7));
        c.ldr(3, obj);
        c.ldl(5, p(obj, 0xf));
        c.ldr(5, p(obj, 8));
        c.g[A2][0] = c.lw(p(obj, 0x10));
        c.sdl(3, p(obj, 0x1b));
        c.sdr(3, p(obj, 0x14));
        c.sdl(5, p(obj, 0x23));
        c.sdr(5, p(obj, 0x1c));
        c.sw(p(obj, 0x24), c.g[A2][0]);
        // ... and advance the save pointer by one slot.
        c.g[V1][0] = c.lw(p(ms, 0xe84));
        c.g[A1][0] = sx(ms);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
        c.sw(p(ms, 0xe84), c.g[V1][0]);

        // Renderer pass at 0xa8/0xac.
        let renderer = c.lw(p(ms, 0x10d8)) as u32;
        c.g[V0][0] = sx(renderer);
        c.g[A0][0] = c.lh(p(renderer, 0xa8));
        c.g[V1][0] = c.lw(p(renderer, 0xac));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(ms.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x002c_b1dc)?;

        // Renderer pass at 0xf8/0xfc.
        c.g[A1][0] = sx(ms);
        c.g[V0][0] = c.lw(p(ms, 0x10d8));
        c.g[A0][0] = c.lh(p(renderer, 0xf8));
        c.g[V1][0] = c.lw(p(renderer, 0xfc));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(ms.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x002c_b1f4)?;

        // The draw-state bit fields: clear and set the pass flags of the
        // current menu object.
        c.g[T0][0] = sx(ms);
        c.g[A1][0] = 0xffff_ffff_ffff_fc1f;
        c.g[V1][0] = 0xffff_ffff_fe7f_0000;
        c.g[T3][0] = 0x0100_0000;
        c.g[A0][0] = c.lw(p(ms, 0xe84));
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        c.g[T1][0] = 0xffff_ffff_ffcf_0000;
        c.g[T4][0] = 0x0030_0000;
        c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 8));
        c.g[T1][0] = c.g[T1][0] | 0xffff;
        c.g[T2][0] = 0xffff_ffff_fff0_0000;
        c.g[A3][0] = 0x0001_0000;
        c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
        c.g[T2][0] = c.g[T2][0] | 0xfff;
        c.g[V0][0] = c.g[V0][0] | 0x220;
        c.g[A3][0] = c.g[A3][0] | 0x4000;
        c.sw(p(c.g[A0][0] as u32, 8), c.g[V0][0]);
        c.g[T5][0] = 0xffff_ffff_ffff_ff83;
        c.mtc1(0, 12); // f12 = 0.0
        c.g[A1][0] = 0;
        c.g[A0][0] = c.lw(p(ms, 0xe84));
        c.g[A2][0] = 0;
        c.g[AT][0] = 0x4420_0000; // 40.0
        c.mtc1(1, 14);
        c.mov_s(13, 12);
        c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 4));
        c.mov_s(16, 12);
        c.g[AT][0] = 0x43f0_0000; // 480.0
        c.mtc1(1, 15);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[AT][0] = 0x3f80_0000; // 1.0
        c.mtc1(1, 17);
        c.g[V0][0] = c.g[V0][0] | c.g[T3][0];
        c.sw(p(c.g[A0][0] as u32, 4), c.g[V0][0]);
        c.g[V1][0] = c.lw(p(ms, 0xe84));
        c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 4));
        c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
        c.g[V0][0] = c.g[V0][0] | c.g[T4][0];
        c.sw(p(c.g[V1][0] as u32, 4), c.g[V0][0]);
        c.g[A0][0] = c.lw(p(ms, 0xe84));
        c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 4));
        c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
        c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
        c.sw(p(c.g[A0][0] as u32, 4), c.g[V0][0]);
        c.g[A3][0] = c.lw(p(ms, 0xe84));
        c.g[V0][0] = c.lw(p(c.g[A3][0] as u32, 4));
        c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
        c.g[V0][0] = c.g[V0][0] | 0x14;
        c.sw(p(c.g[A3][0] as u32, 4), c.g[V0][0]);

        // Renderer pass at 0xd0/0xd4.
        c.g[V1][0] = c.lw(p(ms, 0x10d8));
        c.g[A0][0] = c.lh(p(renderer, 0xd0));
        c.g[V0][0] = c.lw(p(renderer, 0xd4));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx(ms.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x002c_b2cc)?;

        // Renderer pass at 0x110/0x114.
        c.g[A2][0] = sx(menu_sys(c));
        c.g[A1][0] = 0x0050_0000;
        c.g[A1][0] = sx(0x0050_0000u32.wrapping_sub(0xe60));
        c.g[V0][0] = c.lw(p(ms, 0x10d8));
        c.g[A0][0] = c.lh(p(renderer, 0x110));
        c.g[V1][0] = c.lw(p(renderer, 0x114));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(c.g[A2][0] as u32 + c.g[A0][0] as u32);
        call_ptr(c, t, 0x002c_b2ec)?;

        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x002d0d48 MenuSysPoint2DItem_Measure
// ---------------------------------------------------------------------------

/// `void* MenuSysPoint2DItem_Measure(void* item, void* page)` (0x2d0d48).
///
/// Measures a point item: when the item is the page's selected item the height
/// grows by the selection margin, then the item's text is scaled to the
/// available width and the text's box (16 bytes of floats) is written back to
/// the item. `v0` is the item.
pub fn menu_sys_point2d_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        frame.save(c, 16, 0x50);
        frame.save(c, 18, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 19, 0x20);
        c.g[S2][0] = 0;
        frame.save(c, 17, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 21, 0x68);
        frame.save_fpr(c, 20, 0x60);
        c.g[RA][0] = sx(0x002d_0d7c);
        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
        call_ptr(c, 0x002c_bf30, 0x002d_0d7c)?; // MenuSysPage_GetSelectedItem
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        let t = c.g[S0][0] != c.g[V0][0];
        if !t {
            c.cycles += 2;
            c.g[V0][0] = c.lw(p(c.g[S0][0] as u32, 0x40));
            c.g[S2][0] = (0 < c.g[V0][0]) as u64;
        }

        c.cycles += 31;
        c.g[V0][0] = 0x50;
        c.g[S1][0] = 0x14;
        c.g[A1][0] = c.lw(p(c.g[S0][0] as u32, 0x14));
        c.g[A0][0] = c.lw(p(c.g[V1][0] as u32, 0x124));
        if c.g[S2][0] != 0 {
            c.g[S1][0] = c.g[V0][0];
        }
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 0x58));
        c.lwc1(1, p(c.g[V0][0] as u32, 0x14));
        c.cvt_s_w(1, 1);
        c.g[A0][0] = c.g[V0][0];
        c.lwc1(0, p(c.g[V0][0] as u32, 0x34));
        c.lwc1(12, p(c.g[A0][0] as u32, 0x38));
        c.lwc1(13, p(c.g[A0][0] as u32, 0x3c));
        c.g[RA][0] = sx(0x002d_0dcc);
        c.mul_s(21, 1, 0);
        call_ptr(c, 0x0039_1fb0, 0x002d_0dcc)?;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A1][0] = 0x0048_0000;
        c.mov_s(20, 0);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_add(0x6748));
        c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x124));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = c.lw(p(c.g[V0][0] as u32, 0x58));
        c.lwc1(12, p(c.g[A0][0] as u32, 0x38));
        c.g[RA][0] = sx(0x002d_0df8);
        c.lwc1(13, p(c.g[A0][0] as u32, 0x3c));
        call_ptr(c, 0x0039_1fb0, 0x002d_0df8)?;
        c.c_lt_s(20, 0);
        if c.fcc() {
            c.mov_s(20, 0);
        }

        'height: {
            'half: {
                c.cycles += 4;
                c.mtc1(0, 0);
                c.swc1(0, c.g[SP][0] as u32);
                let t = c.g[S2][0] == 0;
                c.swc1(0, p(c.g[SP][0] as u32, 4));
                if t {
                    break 'half;
                }
                'clamp: {
                    c.cycles += 9;
                    c.add_s(1, 21, 21);
                    c.g[AT][0] = 0x40c0_0000;
                    c.mtc1(1, 0);
                    c.mtc1(17, 2);
                    c.cvt_s_w(2, 2);
                    c.add_s(1, 1, 0);
                    c.c_lt_s(1, 2);
                    let t = !c.fcc();
                    if t {
                        break 'clamp;
                    }
                    c.cycles += 4;
                    c.g[AT][0] = 0x41c0_0000;
                    c.mtc1(1, 0);
                    c.add_s(1, 2, 0);
                    break 'clamp;
                }
                c.cycles += 3;
                c.g[AT][0] = 0x41c0_0000;
                c.mtc1(1, 0);
                c.add_s(1, 1, 0);
                c.cycles += 2;
                c.swc1(1, p(c.g[SP][0] as u32, 0xc));
                break 'height;
            }
            // Not selected: the plain text box.
            c.cycles += 11;
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            c.g[AT][0] = 0x4140_0000;
            c.mtc1(1, 2);
            c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 0x124));
            c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x58));
            c.lwc1(0, p(c.g[V1][0] as u32, 0x14));
            c.cvt_s_w(0, 0);
            c.lwc1(1, p(c.g[V1][0] as u32, 0x34));
            c.mul_s(0, 0, 1);
            c.add_s(0, 0, 2);
            c.swc1(0, p(c.g[SP][0] as u32, 0xc));
            break 'height;
        }

        c.cycles += 26;
        c.g[V0][0] = 0x64;
        c.g[AT][0] = 0x4280_0000;
        c.mtc1(1, 1);
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.lq(16, p(c.g[SP][0] as u32, 0x50));
        c.g[V0][0] = c.g[S3][0];
        c.lq(17, p(c.g[SP][0] as u32, 0x40));
        c.lq(18, p(c.g[SP][0] as u32, 0x30));
        c.add_s(0, 0, 20);
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x10));
        c.lwc1(21, p(c.g[SP][0] as u32, 0x68));
        c.lwc1(20, p(c.g[SP][0] as u32, 0x60));
        c.add_s(0, 0, 1);
        c.swc1(0, p(c.g[SP][0] as u32, 8));
        c.ldl(3, p(c.g[SP][0] as u32, 7));
        c.ldr(3, c.g[SP][0] as u32);
        c.ldl(4, p(c.g[SP][0] as u32, 0xf));
        c.ldr(4, p(c.g[SP][0] as u32, 8));
        c.sdl(3, p(c.g[S3][0] as u32, 7));
        c.sdr(3, c.g[S3][0] as u32);
        c.sdl(4, p(c.g[S3][0] as u32, 0xf));
        c.sdr(4, p(c.g[S3][0] as u32, 8));
        c.lq(19, p(c.g[SP][0] as u32, 0x20));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = ra;
        Ok(())
    })
}

/// `MenuSysGroupItem_Draw` (0x002cfd28): lay out and draw the rows of a
/// grouped menu list.  Starting from the rect at `a1`, each visible item
/// (`MenuSysItem_IsVisible`) gets a 0x10-byte rect at `self+0x14c+i*16` --
/// `x` = pos.x + (item == current ? 0 : 20), `y` = the running pos.y,
/// `w` = pos.w - the same offset -- and is drawn through its vtable slot
/// `+0x2c` (with the `+0x28` this-adjustment); `pos.y` then advances by the
/// rect's height (`+0x158`).  The selected item is either the group's
/// current item or every item while `self` is its page's selected item.
pub fn menu_sys_group_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S1, 0x70);
        frame.save(c, S4, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x80);
        c.g[S4][0] = 0u64;
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18)); // count
        c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.ldr(3, c.g[A1][0] as u32);
        c.ldl(4, (c.g[A1][0] as u32).wrapping_add(0xf));
        c.ldr(4, (c.g[A1][0] as u32).wrapping_add(0x8));
        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.sdr(3, c.g[SP][0] as u32);
        c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0xf));
        let no_rows = (c.g[V0][0] as i64) <= 0;
        c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x8));
        if !no_rows {
            c.cycles += 5;
            c.g[S6][0] = 0x14; // indent of a non-selected row
            c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14c)); // rects
            c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x24)); // items
            c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x154));
            loop {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002c_fd98);
                c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                call_ptr(c, 0x002c_a378, 0x002c_fd98)?;
                let mut draw = false;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002c_fda8);
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    call_ptr(c, 0x002c_bf30, 0x002c_fda8)?;
                    if c.g[S1][0] == c.g[V0][0] {
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                        draw = true;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        } else {
                            c.cycles += 1;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                            draw = true;
                        }
                    }
                }
                if draw {
                    c.cycles += 28;
                    c.g[V1][0] = c.g[S6][0];
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A1][0] = c.g[S5][0];
                    c.lwc1(3, c.g[SP][0] as u32);
                    c.g[S0][0] = sx((c.g[S4][0] as u32) << 4);
                    c.g[V0][0] = c.g[V0][0] ^ c.g[A0][0];
                    c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.mtc1(3, 1); // f1 = (float)off
                    c.cvt_s_w(1, 1);
                    c.swc1(0, (c.g[S3][0] as u32).wrapping_sub(0x4));
                    c.sub_s(2, 2, 1);
                    c.add_s(3, 3, 1);
                    c.swc1(2, c.g[S3][0] as u32);
                    c.swc1(3, (c.g[S3][0] as u32).wrapping_sub(0x8));
                    c.g[A2][0] = c.lw(c.g[S2][0] as u32);
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x002c_fe1c)?;
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x158));
                    c.add_s(0, 0, 1);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                }
                c.cycles += 6;
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x10));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_fd90);
                }
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `MenuSysGroupItem_Measure` (0x002cfb58): measure a grouped menu list.  For
/// every visible item (`MenuSysItem_IsVisible`) the item's measure method
/// (vtable `+0x24`, with the `+0x20` this-adjustment) fills the 0x10-byte rect
/// at `sp+0x10`, which is copied to `self+0x14c+i*16`; the item's height is
/// added to the running total at `sp+0xc` when it is the selected row, and
/// non-selected rows get the 20-pixel indent added before the tallest height
/// is kept at `sp+8` and finally written to the 16-byte result at `a0`.
pub fn menu_sys_group_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0xc0);
        c.mtc1(0, 0); // f0 = 0.0
        frame.save(c, S0, 0xa0);
        frame.save(c, S4, 0x60);
        c.g[S0][0] = c.g[A1][0]; // the block
        frame.save(c, S7, 0x30);
        c.g[S4][0] = 0u64;
        frame.save(c, S1, 0x90);
        c.g[S7][0] = c.g[A0][0]; // the result rect
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 20, 0xb0);
        c.g[A1][0] = c.lw(c.g[S0][0] as u32); // the page
        c.swc1(0, c.g[SP][0] as u32);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x124));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18)); // count
        let empty = (c.g[V1][0] as i64) <= 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x13c), c.g[V0][0]);
        if !empty {
            c.cycles += 8;
            c.g[AT][0] = 0x41a0_0000;
            c.mtc1(1, 20); // f20 = 20.0
            c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c)); // rects
            c.g[S3][0] = 0u64;
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24)); // items
            c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x158));
            c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x154));
            loop {
                'iter: {
                    'a: {
                        'c: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x002c_fbe0);
                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                            call_ptr(c, 0x002c_a378, 0x002c_fbe0)?;
                            let is_hidden = c.g[V0][0] == 0;
                            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                            if !is_hidden {
                                'measured: {
                                    'selected: {
                                        c.cycles += 18;
                                        c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                                        c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                                        let target = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x002c_fc00);
                                        c.g[A1][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                        call_ptr(c, target, 0x002c_fc00)?;
                                        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
                                        c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x1f));
                                        c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x18));
                                        c.sdl(2, (c.g[S2][0] as u32).wrapping_add(0x7));
                                        c.sdr(2, c.g[S2][0] as u32);
                                        c.sdl(3, (c.g[S2][0] as u32).wrapping_add(0xf));
                                        c.sdr(3, (c.g[S2][0] as u32).wrapping_add(0x8));
                                        c.g[RA][0] = sx(0x002c_fc28);
                                        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                        call_ptr(c, 0x002c_bf30, 0x002c_fc28)?;
                                        let whole_group = c.g[S0][0] == c.g[V0][0];
                                        c.g[V0][0] =
                                            sx((c.g[S6][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                        if whole_group {
                                            break 'selected;
                                        }
                                        c.cycles += 4;
                                        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                        let not_current = c.g[V1][0] != c.g[V0][0];
                                        c.g[V0][0] =
                                            sx((c.g[S5][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                        if not_current {
                                            break 'measured;
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] =
                                            sx((c.g[S6][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                        break 'selected;
                                    }
                                    c.cycles += 5;
                                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
                                    c.lwc1(1, c.g[V0][0] as u32);
                                    c.add_s(0, 0, 1);
                                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
                                    c.g[V0][0] =
                                        sx((c.g[S5][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                    break 'measured;
                                }
                                'row: {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                    let current = c.g[A0][0] == c.g[V1][0];
                                    c.lwc1(1, c.g[V0][0] as u32);
                                    if !current {
                                        c.cycles += 1;
                                        c.add_s(1, 1, 20);
                                    }
                                }
                                c.cycles += 5;
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                                c.c_lt_s(0, 1);
                                if c.fcc() {
                                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                                    break 'a;
                                }
                                c.cycles += 2;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                break 'iter;
                            }
                        }
                    }
                    // A hidden item contributes an empty rect.
                    c.cycles += 12;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), 0u64);
                    c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x17));
                    c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x1f));
                    c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x18));
                    c.sdl(6, (c.g[S2][0] as u32).wrapping_add(0x7));
                    c.sdr(6, c.g[S2][0] as u32);
                    c.sdl(2, (c.g[S2][0] as u32).wrapping_add(0xf));
                    c.sdr(2, (c.g[S2][0] as u32).wrapping_add(0x8));
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                c.cycles += 6;
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_fbd8);
                }
            }
        }
        c.cycles += 21;
        c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(5, c.g[SP][0] as u32);
        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0xf));
        c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.sdl(5, (c.g[S7][0] as u32).wrapping_add(0x7));
        c.sdr(5, c.g[S7][0] as u32);
        c.sdl(6, (c.g[S7][0] as u32).wrapping_add(0xf));
        c.g[V0][0] = c.g[S7][0];
        c.sdr(6, (c.g[S7][0] as u32).wrapping_add(0x8));
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    });
}

/// `void func_002CC0C0(void* stack)` (0x2cc0c0): the menu system's per-frame
/// update, driven as a resumable block dispatcher (re-entered at_5 the `pc` the
/// trampoline resumes): walk the eight menu entries of the active page, call
/// each one's update slot, and after the loop run the page's own update.
pub fn menu_sys_update(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x002cc0c0 => {
                c.cycles += 8;
                c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x30));
                sq(c, 16, at_5(c, SP, 0x20));
                sq(c, 17, at_5(c, SP, 0x10));
                c.g[S0][0] = c.g[A0][0];
                sd(c, r32(c, SP), c.g[RA][0]);
                c.g[V0][0] = lw(c, r32(c, S0));

                if c.g[V0][0] == 0u64 {
                    lq(c, 16, at_5(c, SP, 0x20));
                    pc = 0x002cc238;
                    continue;
                }
                pc = 0x002cc0e0;
                continue;
            }
            0x002cc0e0 => {
                c.cycles += 4;
                c.g[A0][0] = lw(c, at_5(c, S0, 0x5c));
                c.g[V0][0] = lw(c, r32(c, A0));
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = 0u64;
                if t {
                    pc = 0x002cc0f8;
                    continue;
                }
                pc = 0x002cc0f0;
                continue;
            }
            0x002cc0f0 => {
                c.cycles += 2;
                c.g[V0][0] = lw(c, r32(c, V0));
                c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
                pc = 0x002cc0f8;
                continue;
            }
            0x002cc0f8 => {
                c.cycles += 2;

                if c.g[V1][0] == 0u64 {
                    lq(c, 16, at_5(c, SP, 0x20));
                    pc = 0x002cc238;
                    continue;
                }
                pc = 0x002cc100;
                continue;
            }
            0x002cc100 => {
                c.cycles += 11;
                c.g[RA][0] = sx(0x002cc108);
                c.g[A1][0] = 0x3d;
                if call_ptr(c, 0x00320bf0, 0x002cc108).is_err() {
                    return;
                }
                c.cvt_w_s(1, 0);
                c.mfc1(17, 1);
                c.g[A2][0] = sx(at_5(c, S0, 0x4c));
                c.g[V0][0] = 0xffffffffffffffff;
                c.g[A3][0] = sx(at_5(c, S0, 0x50));
                sw(c, at_5(c, S0, 0x4c), c.g[V0][0]);
                sw(c, at_5(c, S0, 0x50), c.g[V0][0]);
                let t = c.g[S1][0] != c.g[V0][0];
                sw(c, at_5(c, S0, 0x48), c.g[S1][0]);
                if t {
                    pc = 0x002cc1c4;
                    continue;
                }
                pc = 0x002cc12c;
                continue;
            }
            0x002cc12c => {
                c.cycles += 3;
                c.g[A0][0] = lw(c, at_5(c, GP, 0x300));

                if c.g[A0][0] == 0u64 {
                    c.g[V0][0] = lw(c, r32(c, S0));
                    pc = 0x002cc1c8;
                    continue;
                }
                pc = 0x002cc138;
                continue;
            }
            0x002cc138 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002cc140);
                c.g[A1][0] = 0u64;
                if call_ptr(c, 0x00266ba8, 0x002cc140).is_err() {
                    return;
                }

                if c.g[V0][0] == 0u64 {
                    c.g[V0][0] = lw(c, r32(c, S0));
                    pc = 0x002cc1c8;
                    continue;
                }
                pc = 0x002cc148;
                continue;
            }
            0x002cc148 => {
                c.cycles += 4;
                c.g[V0][0] = lw(c, at_5(c, S0, 0x50));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 32i64) as u64;

                if c.g[V0][0] != 0u64 {
                    sw(c, at_5(c, S0, 0x50), c.g[S1][0]);
                    pc = 0x002cc158;
                    continue;
                }
                pc = 0x002cc158;
                continue;
            }
            0x002cc158 => {
                c.cycles += 5;
                c.g[V0][0] = lw(c, at_5(c, S0, 0x4c));
                c.g[A0][0] = sx(r32(c, V0).wrapping_sub(0xd));
                c.g[V1][0] = (c.g[A0][0] < 0x22u64) as u64;
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = 0x480000;
                if t {
                    pc = 0x002cc1c4;
                    continue;
                }
                pc = 0x002cc16c;
                continue;
            }
            0x002cc16c => {
                c.cycles += 6;
                c.g[V1][0] = sx(r32(c, A0) << 2);
                c.g[V0][0] = sx(at_5(c, V0, 0x6630));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[A0][0] = lw(c, r32(c, V1));
                let t = c.g[A0][0] as u32;

                pc = t;
                continue;
            }
            0x002cc184 => {
                c.cycles += 2;
                let t = true;
                sw(c, at_5(c, S0, 0x48), 0u64);
                if t {
                    pc = 0x002cc1c4;
                    continue;
                }
                pc = 0x002cc18c;
                continue;
            }
            0x002cc18c => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x1;
                if t {
                    pc = 0x002cc1c0;
                    continue;
                }
                pc = 0x002cc194;
                continue;
            }
            0x002cc194 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x2;
                if t {
                    pc = 0x002cc1c0;
                    continue;
                }
                pc = 0x002cc19c;
                continue;
            }
            0x002cc19c => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x3;
                if t {
                    pc = 0x002cc1c0;
                    continue;
                }
                pc = 0x002cc1a4;
                continue;
            }
            0x002cc1a4 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x9;
                if t {
                    pc = 0x002cc1c0;
                    continue;
                }
                pc = 0x002cc1ac;
                continue;
            }
            0x002cc1ac => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0xa;
                if t {
                    pc = 0x002cc1c0;
                    continue;
                }
                pc = 0x002cc1b4;
                continue;
            }
            0x002cc1b4 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x6;
                if t {
                    pc = 0x002cc1c0;
                    continue;
                }
                pc = 0x002cc1bc;
                continue;
            }
            0x002cc1bc => {
                c.cycles += 1;
                c.g[V0][0] = 0x7;
                pc = 0x002cc1c0;
                continue;
            }
            0x002cc1c0 => {
                c.cycles += 1;
                sw(c, at_5(c, S0, 0x48), c.g[V0][0]);
                pc = 0x002cc1c4;
                continue;
            }
            0x002cc1c4 => {
                c.cycles += 1;
                c.g[V0][0] = lw(c, r32(c, S0));
                pc = 0x002cc1c8;
                continue;
            }
            0x002cc1c8 => {
                c.cycles += 13;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1));
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                c.g[A1][0] = lw(c, at_5(c, V0, 0x4));
                c.g[V1][0] = lw(c, at_5(c, A1, 0x12c));
                c.g[A0][0] = lh_2(c, at_5(c, V1, 0x10));
                c.g[V0][0] = lw(c, at_5(c, V1, 0x14));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x002cc1ec);
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x002cc1ec).is_err() {
                    return;
                }
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = 0x1;
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x2;
                if t {
                    pc = 0x002cc20c;
                    continue;
                }
                pc = 0x002cc1fc;
                continue;
            }
            0x002cc1fc => {
                c.cycles += 2;

                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = lw(c, r32(c, S0));
                    pc = 0x002cc22c;
                    continue;
                }
                pc = 0x002cc204;
                continue;
            }
            0x002cc204 => {
                c.cycles += 2;
                let t = true;
                lq(c, 16, at_5(c, SP, 0x20));
                if t {
                    pc = 0x002cc238;
                    continue;
                }
                pc = 0x002cc20c;
                continue;
            }
            0x002cc20c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002cc214);
                c.g[A0][0] = c.g[S0][0];
                if call_ptr(c, 0x002cc460, 0x002cc214).is_err() {
                    return;
                }
                let t = true;
                lq(c, 16, at_5(c, SP, 0x20));
                if t {
                    pc = 0x002cc238;
                    continue;
                }
                pc = 0x002cc21c;
                continue;
            }
            0x002cc21c => {
                c.cycles += 1;
                pc = 0x002cc220;
                continue;
            }
            0x002cc220 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002cc228);
                c.g[A0][0] = c.g[S0][0];
                if call_ptr(c, 0x002cc460, 0x002cc228).is_err() {
                    return;
                }
                c.g[V0][0] = lw(c, r32(c, S0));
                pc = 0x002cc22c;
                continue;
            }
            0x002cc22c => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64;

                if t {
                    if back_edge_unwinds(c, 0x002cc220) {
                        return;
                    }
                    pc = 0x002cc220;
                    continue;
                }
                pc = 0x002cc234;
                continue;
            }
            0x002cc234 => {
                c.cycles += 1;
                lq(c, 16, at_5(c, SP, 0x20));
                pc = 0x002cc238;
                continue;
            }
            0x002cc238 => {
                c.cycles += 4;
                lq(c, 17, at_5(c, SP, 0x10));
                c.g[RA][0] = ld(c, r32(c, SP));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx(at_5(c, SP, 0x30));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x002cc0c0, 0x002cc248);
                return;
            }
        }
    }
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
    use crate::difftest::events::Events;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
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
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::SP as STACK;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::ai::tests::slot as slot_4;
    pub(crate) use crate::game::ai::ai::tests::w16;
    pub(crate) use crate::game::ai::ai::tests::wf_2 as wf;
    pub(crate) use crate::game::ai::ai::tests::CHILD;
    pub(crate) use crate::game::ai::ai::tests::LOAD_B0;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_5;
    pub(crate) use crate::game::be::beintoption::tests::slot;
    pub(crate) use crate::game::be::beintoption::tests::VT;
    pub(crate) use crate::game::be::belibrary::bolts_count_children;
    pub(crate) use crate::game::be::bestate2::tests::belib;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::be::bestate2::tests::SEEDS as SEEDS_6;
    pub(crate) use crate::game::bx::appman::tests::ptr as ptr_2;
    pub(crate) use crate::game::bx::appman::tests::rng;
    pub(crate) use crate::game::bx::appman::tests::SEEDS;
    pub(crate) use crate::game::bx::bxstring::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::bx::bxstring2::tests::game;
    pub(crate) use crate::game::bx::bxstring2::tests::game_globals;
    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;
    pub(crate) use crate::game::camera::cameracontroller::tests::pool as pool_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::alloc_ready;
    pub(crate) use crate::game::dirtysock::tags2::tests::clear as clear_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::common;
    pub(crate) use crate::game::dirtysock::tags2::tests::dummy_world;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::load;
    pub(crate) use crate::game::dirtysock::tags2::tests::SEEDS_7 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::call_machine;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_5 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_6 as globals_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_10 as heap_init_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_2 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_10 as pool_allocator_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_3 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::run;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_2 as slot_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_LIBRARY;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH as SCRATCH_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_3;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_12;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all_2;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2 as RMAN;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_3;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_7;
    pub(crate) use crate::game::fe::festaterewards::tests::be_library;
    pub(crate) use crate::game::input::inputparse::tests::EMPTY;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_8;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_5;
    pub(crate) use crate::game::main::gamestate::tests::slot_2 as slot_6;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_9;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3;
    pub(crate) use crate::game::movie::movieplayer::tests::heap_init_2 as heap_init_3;
    pub(crate) use crate::game::movie::movieplayer::tests::run_2;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2;
    pub(crate) use crate::game::movie::movieplayer::tests::W;
    pub(crate) use crate::game::render::bezierman::tests::b;
    pub(crate) use crate::game::render::font::tests::words;
    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::render::particle::tests::base;
    pub(crate) use crate::game::render::particle::tests::clear as clear_3;
    pub(crate) use crate::game::render::particle::tests::common as common_2;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_5;
    pub(crate) use crate::game::replay::replay2::tests::dtor_setup;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_6;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals_5;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_11;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7 as globals_8;
    pub(crate) use crate::game::sound::soundsys::tests::load_2;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool_5;
    pub(crate) use crate::game::visualfx::worldlightman::tests::alloc_setup;
    pub(crate) use crate::game::visualfx::worldlightman::tests::dtor_case;
    pub(crate) use crate::game::visualfx::worldlightman::tests::MEM_MUTEX;
    pub(crate) use crate::game::world::worldcache::tests::check_all;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_7;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_4;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals_6;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD_2;
    use ssx3_ee as ee;
    use ssx3_ee::s00;
    use ssx3_ee::s01;
    use ssx3_ee::s02;
    use ssx3_ee::s03;
    use ssx3_ee::s08;
    use ssx3_ee::s09;
    use ssx3_ee::s10;
    use ssx3_ee::s11;
    use ssx3_ee::s12;
    use ssx3_ee::s13;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn world_get_ai_frame_count_matches() {
        check(
            s09::f_002d1c98__generated,
            world_get_ai_frame_count,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 1);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let world = ptr_2(&mut r, 0);
                let manager = ptr_2(&mut r, 1);
                let ai = ptr_2(&mut r, 2);
                c.bus.write32(GAME_GP.wrapping_sub(WORLD_GLOBAL), world);
                c.bus.write32(world + WORLD_AI_MANAGER, manager);
                c.bus.write32(manager + AI_MANAGER_CURRENT, ai);
                c.bus.write32(ai + AI_FRAME_COUNT, r.next_u32());
            },
        );
    }

    #[test]
    fn world_get_flag_manager_matches() {
        check(
            s09::f_002d1cb0__generated,
            world_get_flag_manager,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 2);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let world = ptr_2(&mut r, 0);
                let manager = ptr_2(&mut r, 1);
                c.bus.write32(GAME_GP.wrapping_sub(WORLD_GLOBAL), world);
                c.bus.write32(world + WORLD_AI_MANAGER, manager);
                c.bus.write32(manager + FLAG_MANAGER, r.next_u32());
            },
        );
    }

    #[test]
    fn menu_sys_item_ctor_matches() {
        check(
            ssx3_ee::s09::f_002ca258,
            menu_sys_item_ctor,
            50,
            &|c, seed| {
                let mut r = Rng(0xa258 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn menu_sys_item_is_visible_matches() {
        check(
            ssx3_ee::s09::f_002ca378,
            menu_sys_item_is_visible,
            50,
            &|c, seed| {
                let mut r = Rng(0xa378 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x4, r.next_u32());
            },
        );
    }

    #[test]
    fn menu_sys_item_set_visible_matches() {
        check(
            ssx3_ee::s09::f_002ca388,
            menu_sys_item_set_visible,
            100,
            &|c, seed| {
                let mut r = Rng(0xa388 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                if seed % 2 == 0 {
                    c.g[A1][0] = 0;
                } else {
                    c.g[A1][0] = ((r.next_u32() as u64) << 32) | (r.next_u32() as u64 | 1);
                }
                c.bus.write32(DATA + 0x4, r.next_u32());
            },
        );
    }

    #[test]
    fn menu_sys_item_is_disabled_matches() {
        check(
            ssx3_ee::s09::f_002ca3b0,
            menu_sys_item_is_disabled,
            50,
            &|c, seed| {
                let mut r = Rng(0xa3b0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x4, r.next_u32());
            },
        );
    }

    #[test]
    fn menu_sys_page_ctor_matches() {
        check(
            ssx3_ee::s09::f_002caa58,
            menu_sys_page_ctor,
            50,
            &|c, seed| {
                let mut r = Rng(0xaa58 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
            },
        );
    }

    #[test]
    fn menu_sys_page_remove_all_items_matches() {
        check(
            ssx3_ee::s09::f_002cbe10,
            menu_sys_page_remove_all_items,
            50,
            &|c, seed| {
                let mut r = Rng(0xbe10 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
            },
        );
    }

    #[test]
    fn menu_sys_reset_matches() {
        check(ssx3_ee::s09::f_002cc070, menu_sys_reset, 50, &|c, seed| {
            let mut r = Rng(0xc070 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
        });
    }

    #[test]
    fn world_get_rider_count_matches() {
        check(
            ssx3_ee::s09::f_002d1af0,
            world_get_rider_count,
            50,
            &|c, seed| {
                let mut r = Rng(0x1af0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let world = DATA + 0x100;
                let man = DATA + 0x200;
                let sub = DATA + 0x300;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, man);
                c.bus.write32(man + 0xc, sub);
                c.bus.write32(sub + 0x78, r.next_u32());
            },
        );
    }

    #[test]
    fn world_get_rider_matches() {
        check(
            ssx3_ee::s09::f_002d1b08,
            world_get_rider,
            200,
            &|c, seed| {
                let mut r = Rng(0x1b08 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let world = DATA + 0x100;
                let man = DATA + 0x200;
                let table = DATA + 0x400;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, man);
                c.bus.write32(man + 0xc, table);
                let idx = seed % 4;
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | idx as u64;
                for i in 0..4u32 {
                    let e = DATA + 0x800 + i * 0x100;
                    c.bus.write32(table + i * 4, e);
                    let v = if i == idx && seed % 3 == 0 {
                        0
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(e + 0x28, v);
                }
            },
        );
    }

    #[test]
    fn wscript_man_run_obj_script5_hook_matches() {
        check(
            ee::s09::f_002d1ac8__generated,
            wscript_man_run_obj_script5_hook,
            200,
            &|c, seed| {
                machine(c, seed, 0x2d1ac8, 2);
                let man = DATA + 0x2000;
                let id = DATA + 0x1000;
                c.bus.write32(GAME_GP + 0xce8, man);
                c.bus.write32(man + 0x28c, 0);
                c.bus.write32(id + 0x78, 0);
                c.bus.write32(id + 0x88, 0);
                c.g[A0][0] = sx(id);
            },
        );
    }

    #[test]
    fn menu_sys_item_get_input_code_matches() {
        check(
            ee::s09::f_002ca3e8__generated,
            menu_sys_item_get_input_code,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c_a3e8, 0);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, DATA + 0x100);
                c.bus.write32(DATA + 0x100 + 0x124, r.next_u32());
            },
        );
    }

    #[test]
    fn menu_sys_dtor_matches() {
        check(
            ee::s09::f_002cc048__generated,
            menu_sys_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x2cc048),
        );
    }

    #[test]
    fn menu_sys_page_get_selected_index_matches() {
        check(
            s09::f_002cbf08__generated,
            menu_sys_page_get_selected_index,
            150,
            &|c, seed| {
                machine(c, seed, 0x2cbf08, 1);
                c.g[A0][0] = sx(OBJ);
                clear(c, OBJ, 0x40);
                clear(c, CHILD, 0x40);
                // Selection state: no items selected / one item at +0x4.
                c.bus.write32(OBJ + 4, seed as u32 % 4);
                // MenuSysPage_ValidateSelection walks the page's item list at
                // +0xc; a null list is the "nothing to validate" path.
                c.bus.write32(OBJ + 0xc, 0);
            },
        );
    }

    #[test]
    fn menu_sys_point2d_item_on_focus_matches() {
        check(
            s09::f_002d08e0__generated,
            menu_sys_point2d_item_on_focus,
            150,
            &|c, seed| {
                machine(c, seed, 0x2d08e0, 1);
                c.g[A0][0] = sx(OBJ);
                clear(c, OBJ, 0x40);
                c.bus.write32(OBJ + 0x40, seed as u32 & 0xff);
            },
        );
    }

    // -----------------------------------------------------------------
    // Table and vtable dispatchers.

    #[test]
    fn mod_obj_call_slot7_matches() {
        covered(
            s09::f_002d1b30__generated,
            mod_obj_call_slot7,
            120,
            &|c, seed| {
                machine(c, seed, 0x2d1b30, 1);
                clear(c, OBJ, 0x20);
                clear(c, VT, 0x40);
                // `ModObj::v07`: the vtable lives at the object's first word.
                slot(c, VT, 0x38);
                c.bus.write32(OBJ, VT);
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(seed as u32);
                c.g[A2][0] = sx(seed as u32);
                c.g[A3][0] = sx(seed as u32);
            },
        );
    }

    /// The behaviour-library singleton (a zeroed dummy, so
    /// `BELibrary_GetInstance` neither constructs one nor reads the disc) and
    /// a world whose race manager holds one rider.
    pub(crate) fn globals(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6_c000);
        let (world, ail, arr) = (DATA + 0x6_8000, DATA + 0x6_8100, DATA + 0x6_8200);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x84, ail);
        c.bus.write32(ail + 0xc, arr);
        c.bus.write32(arr + 8, 20);
        c.bus.write32(arr + 0x28, DATA + 0x6_8400);
        c.bus.write32(arr + 0x7c, 1);
    }

    #[test]
    fn menu_sys_item_dtor_matches() {
        dtor_case(ee::s09::f_002ca280__generated, menu_sys_item_dtor, 0x2ca280);
    }

    #[test]
    fn menu_sys_page_dtor_matches() {
        dtor_case(ee::s09::f_002caa80__generated, menu_sys_page_dtor, 0x2caa80);
    }

    /// A menu page at `DATA`: item array at `+0xc`, selected index at `+0x4`,
    /// each item's vtable at `+0x10` (slots 0x18/0x1c for ValidateSelection,
    /// 0x40/0x44 for the group-item notify).
    pub(crate) fn menu_page_setup(c: &mut Cpu, seed: u32, salt: u64, page: u32) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        let n = 1 + (seed % 3);
        let vt = vtable(c, &mut r, &[0x18, 0x40]);
        c.bus.write32(page + 4, seed % n);
        for i in 0..3u32 {
            let item = DATA + 0x300 + 0x40 * i;
            c.bus.write32(page + 4 * i + 0xc, item);
            c.bus.write32(item + 0x10, vt);
        }
        r
    }

    #[test]
    fn menu_sys_page_get_selected_item_matches() {
        check(
            ee::s09::f_002cbf30__generated,
            menu_sys_page_get_selected_item,
            SEEDS_2,
            &|c, seed| {
                menu_page_setup(c, seed, 0x2cbf30, DATA);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn menu_sys_group_item_v08_matches() {
        check(
            ee::s09::f_002cf8d8__generated,
            menu_sys_group_item_v08,
            SEEDS_2,
            &|c, seed| {
                // The group item wraps its page at +0x18.
                menu_page_setup(c, seed, 0x2cf8d8, DATA + 0x18);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn menu_sys_group_item_v10_matches() {
        check(
            ee::s09::f_002cfb28__generated,
            menu_sys_group_item_v10,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cfb28, 0);
                let vt = vtable(c, &mut r, &[0x20]);
                let owner = DATA + 0x100;
                c.bus.write32(DATA, owner);
                c.bus.write32(owner + 0x12c, vt);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    // ------------------------------------------------------- script

    #[test]
    fn wscript_man_obj_trigger_hook_matches() {
        check(
            ee::s09::f_002d19b8__generated,
            wscript_man_obj_trigger_hook,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d19b8, 8);
                let manager = DATA + 0x100;
                let trigger = DATA + 0x2000;
                let vt = vtable(c, &mut r, &[0x38, 0x40]);
                c.bus.write32(GAME_GP + 0xce8, manager);
                c.bus.write32(manager, r.next_u32());
                c.bus.write32(trigger, vt);
                // The second half of WScriptMan_OnObjectTrigger must take its
                // early return: the trigger object at a3+0x480 is non-null and
                // the world's race manager reports a mode other than 4.
                c.bus.write32(trigger.wrapping_sub(0x6c0) + 0x480, 1);
                globals(c);
                c.bus.write32(DATA + 0x6_8100 + 0x214, 0);
                c.g[A0][0] = (DATA + 0x3000) as u64;
                c.g[A1][0] = (DATA + 0x4000) as u64;
                c.g[A2][0] = trigger as u64;
                c.bus.write32(DATA + 0x4000 + 0x50, r.next_u32());
            },
        );
    }

    // ---------------------------------------------------------- menu/stats

    #[test]
    fn menu_sys_submenu_item_measure_matches() {
        check(
            ee::s09::f_002ccd90__generated,
            menu_sys_submenu_item_measure,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ccd_90, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(DATA + 0x200);
                c.bus.write32(DATA + 0x200 + 0x14, (seed % 8) as u32);
                fill(c, &mut r, DATA + 0x100, 0x40);
            },
        );
    }

    #[test]
    fn menu_sys_back_item_measure_matches() {
        check(
            ee::s09::f_002ccea8__generated,
            menu_sys_back_item_measure,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cce_a8, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(DATA + 0x200);
                c.bus.write32(DATA + 0x200 + 0x14, (seed % 8) as u32);
                fill(c, &mut r, DATA + 0x100, 0x40);
            },
        );
    }

    #[test]
    fn menu_sys_label_item_measure_matches() {
        check(
            ee::s09::f_002cd2f0__generated,
            menu_sys_label_item_measure,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cd2_f0, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(DATA + 0x200);
                c.bus.write32(DATA + 0x200 + 0x14, (seed % 8) as u32);
                fill(c, &mut r, DATA + 0x100, 0x40);
            },
        );
    }

    // -----------------------------------------------------------------------
    // MenuSys*Item_Measure trampolines.

    /// A zeroed item and row: `*(item + 0x14)` is 0, so `MenuSysItem_MeasureRow`
    /// takes its cheapest path and calls nothing.
    pub(crate) fn menu_item_setup(c: &mut Cpu, seed: u32, salt: u64) {
        machine(c, seed, salt, 0);
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = (DATA + 0x800) as u64;
    }

    #[test]
    fn menu_sys_callback_item_measure_matches() {
        check(
            ssx3_ee::s09::f_002cd240__generated,
            menu_sys_callback_item_measure,
            32,
            &|c, seed| {
                menu_item_setup(c, seed, 0x2cd2_40);
            },
        );
    }

    #[test]
    fn menu_sys_int8_item_measure_matches() {
        check(
            ssx3_ee::s09::f_002cd7b8__generated,
            menu_sys_int8_item_measure,
            32,
            &|c, seed| {
                menu_item_setup(c, seed, 0x2cd7_b8);
            },
        );
    }

    #[test]
    fn menu_sys_int_item_measure_matches() {
        check(
            ssx3_ee::s09::f_002cdbf0__generated,
            menu_sys_int_item_measure,
            32,
            &|c, seed| {
                menu_item_setup(c, seed, 0x2cdb_f0);
            },
        );
    }

    #[test]
    fn menu_sys_float_item_measure_matches() {
        check(
            ssx3_ee::s09::f_002cdfa0__generated,
            menu_sys_float_item_measure,
            32,
            &|c, seed| {
                menu_item_setup(c, seed, 0x2cdf_a0);
            },
        );
    }

    #[test]
    fn menu_sys_float_range_item_measure_matches() {
        check(
            ssx3_ee::s09::f_002ce368__generated,
            menu_sys_float_range_item_measure,
            32,
            &|c, seed| {
                menu_item_setup(c, seed, 0x2ce3_68);
            },
        );
    }

    #[test]
    fn menu_sys_action_item_draw_matches() {
        check(
            ee::s09::f_002cd0e8__generated,
            menu_sys_action_item_draw,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x2cd0e8, 0);
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at + 0x14, (seed % 5) as u32);
                c.bus
                    .write32(self_at + 0x18, if seed % 2 == 0 { 0 } else { seed % 9 });
            },
        );
    }

    #[test]
    fn world_get_rider_position_matches() {
        covered(
            ee::s09::f_002d1b58__generated,
            world_get_rider_position,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d_1b58, 1);
                let world = DATA + 0x4000;
                let mgr = DATA + 0x5000;
                let race = DATA + 0x6000;
                let rider = DATA + 0x7000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, mgr);
                c.bus.write32(mgr + 0xc, race);
                let i = (r.next_u32() % 4) as u32;
                c.bus.write32(race + 4 * i + 0x28, rider);
                let vt = vtable(c, &mut r, &[0x28]);
                c.bus.write32(rider + 0x6c0, vt);
                c.g[A0][0] = i as u64;
            },
        );
    }

    #[test]
    fn menu_sys_item_is_selectable_matches() {
        check(
            ee::s09::f_002ca320__generated,
            menu_sys_item_is_selectable,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ca3_20, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus.write32(
                    this + 4,
                    match seed % 4 {
                        0 => 0,
                        1 => 1,
                        2 => 8,
                        _ => 9,
                    },
                );
                c.bus.write32(this + 0x14, r.next_u32());
            },
        );
    }

    #[test]
    fn menu_sys_page_set_row_color_matches() {
        check(
            ee::s09::f_002cb880__generated,
            menu_sys_page_set_row_color,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cb8_80, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() % 8);
                c.g[A2][0] = sx(if seed % 2 == 0 { 0 } else { 1 + seed % 3 });
                let vt = DATA + 0x400;
                c.bus.write32(this + 0x12c, vt);
                slot_2(c, &mut r, vt, 0x40, FAKE + 0x10);
            },
        );
    }

    #[test]
    fn menu_sys_page_on_deactivate_matches() {
        check(
            ee::s09::f_002cbfd0__generated,
            menu_sys_page_on_deactivate,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cbf_d0, 1);
                let this = DATA + 0x100;
                let index = seed % 4;
                let item = DATA + 0x400;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 4, index);
                c.bus.write32(this + 0xc + 4 * index, item);
                c.bus.write32(this + 0x124, r.next_u32());
                c.bus.write32(item + 0x10, DATA + 0x800);
                slot_2(c, &mut r, DATA + 0x800, 0x38, FAKE + 0x10);
            },
        );
    }

    #[test]
    fn menu_sys_back_item_ctor_matches() {
        check(
            ee::s09::f_002ccdf0__generated,
            menu_sys_back_item_ctor,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ccd_f0, 1);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(if seed % 2 == 0 { DATA + 0x200 } else { 0 });
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn menu_sys_action_item_ctor_matches() {
        check(
            ee::s09::f_002ccf08__generated,
            menu_sys_action_item_ctor,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ccf_08, 1);
                c.g[A0][0] = sx(DATA + 0x100);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn wscript_man_try_run_obj_script3_matches() {
        check(
            ee::s09::f_002d1a30__generated,
            wscript_man_try_run_obj_script3,
            60,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x2d1a_30, 2);
                c.g[A0][0] = sx(seed as u32 & 0xf);
                c.bus.write32(GAME_GP + 0xce8, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x28c, DATA + 0x2000);
                c.bus.write32(DATA + 0x1000 + 0x290, r.next_u32());
            },
        );
    }

    #[test]
    fn menu_sys_page_validate_selection_matches() {
        check(
            ee::s09::f_002caab0__generated,
            menu_sys_page_validate_selection,
            SEEDS_4,
            &|c, seed| {
                let _ = common(c, seed, 0x2caab0, 1);
                let this = DATA + 0x100;
                let item = DATA + 0x2000;
                let obj = DATA + 0x3000;
                clear_2(c, this, 0x200);
                clear_2(c, item, 0x100);
                clear_2(c, obj, 0x100);
                let idx = seed % 3;
                c.bus.write32(this + 4, idx);
                c.bus.write32(this + 0xc + 4 * idx, item);
                c.bus.write32(item + 0x10, obj);
                c.bus.write16(obj + 0x18, 0x20);
                c.bus.write32(obj + 0x1c, FAKE);
                if seed % 2 == 0 {
                    c.bus.write32(LOG_RESULTS, 1);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn menu_sys_page_end_draw_matches() {
        check(
            ee::s09::f_002cb2f8__generated,
            menu_sys_page_end_draw,
            SEEDS_4,
            &|c, seed| {
                let _ = common(c, seed, 0x2cb2f8, 2);
                let mgr = DATA + 0x100;
                let obj = DATA + 0x2000;
                clear_2(c, mgr, 0x2000);
                clear_2(c, obj, 0x200);
                c.bus.write32(mgr + 0x10d8, obj);
                c.bus.write16(obj + 0x100, 0x20);
                c.bus.write32(obj + 0x104, FAKE);
                c.bus.write16(obj + 0xb0, 0x20);
                c.bus.write32(obj + 0xb4, FAKE + 0x10);
                c.bus.write32(mgr + 0xe84, 0x40 + seed % 8);
                c.bus.write32(gp_at(-0x854), mgr);
            },
        );
    }

    #[test]
    fn menu_sys_action_item_measure_matches() {
        check(
            ee::s09::f_002cd090__generated,
            menu_sys_action_item_measure,
            SEEDS_4,
            &|c, seed| {
                let mut r = common(c, seed, 0x2cd090, 2);
                alloc_ready(c);
                let this = DATA + 0x100;
                let item = DATA + 0x2000;
                clear_2(c, this, 0x400);
                clear_2(c, item, 0x100);
                c.bus.write32(item + 0x14, r.next_u32() % 100);
                let icon = seed % 3 != 0;
                c.bus
                    .write32(item + 0x18, if icon { DATA + 0x3000 } else { 0 });
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(item);
            },
        );
    }

    // ------------------------------------------------------------------
    // Menus.

    #[test]
    fn menu_sys_bool_item_draw_matches() {
        check(
            ee::s09::f_002cc758__generated,
            menu_sys_bool_item_draw,
            SEEDS_5,
            &|c, seed| {
                let mut r = common_2(c, seed, 0x2cc758, 1);
                let this = DATA + 0x100;
                clear_3(c, this, 0x200);
                c.bus.write32(
                    this + 0x18,
                    if seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(this + 0x1c, ((seed / 3) % 2) as u32);
                c.bus.write32(this + 0x20, DATA + 0x200);
                c.bus.write32(this + 0x14, r.next_u32());
                c.bus.write32(DATA + 0x200, ((seed / 6) % 2) as u32);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn menu_sys_submenu_item_ctor_matches() {
        check(
            ee::s09::f_002cccc8__generated,
            menu_sys_submenu_item_ctor,
            SEEDS_5,
            &|c, seed| {
                let mut r = base(c, seed, 0x2cccc8);
                let this = DATA + 0x100;
                clear_3(c, this, 0x200);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn menu_sys_color_channel_item_ctor_matches() {
        check(
            ee::s09::f_002ced20__generated,
            menu_sys_color_channel_item_ctor,
            SEEDS_5,
            &|c, seed| {
                let mut r = base(c, seed, 0x2ced20);
                let this = DATA + 0x100;
                clear_3(c, this, 0x200);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn menu_sys_color_swatch_item_ctor_matches() {
        check(
            ee::s09::f_002cff28__generated,
            menu_sys_color_swatch_item_ctor,
            SEEDS_5,
            &|c, seed| {
                let mut r = base(c, seed, 0x2cff28);
                let this = DATA + 0x100;
                clear_3(c, this, 0x200);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn menu_sys_spacer_item_measure_matches() {
        check(
            ssx3_ee::s09::f_002cd3a0__generated,
            menu_sys_spacer_item_measure,
            200,
            &|c, seed| {
                let mut r = Rng(0x2cd3_a000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                words(
                    c,
                    DATA + 0x300,
                    &[
                        r.next_u32(),
                        r.next_u32(),
                        r.next_u32(),
                        r.next_u32(),
                        r.next_u32(),
                        r.next_u32(),
                    ],
                );
            },
        );
    }

    #[test]
    fn menu_sys_int8_item_reset_value_matches() {
        check(
            ssx3_ee::s09::f_002cd578__generated,
            menu_sys_int8_item_reset_value,
            200,
            &|c, seed| {
                let mut r = Rng(0x2cd5_7800 ^ seed as u64);
                scramble(c, &mut r);
                let item = DATA;
                let dst = DATA + 0x400;
                c.g[A0][0] = item as u64;
                c.bus.write32(item + 0x18, dst);
                let byte = match seed % 4 {
                    0 => 0,
                    1 => 0x7f,
                    2 => 0x80,
                    _ => r.next_u32() as u8,
                };
                c.bus.write8(item + 0x1c, byte);
                c.bus.write8(dst, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn menu_sys_int_item_reset_value_matches() {
        check(
            ssx3_ee::s09::f_002cd9b8__generated,
            menu_sys_int_item_reset_value,
            200,
            &|c, seed| {
                let mut r = Rng(0x2cd9_b800 ^ seed as u64);
                scramble(c, &mut r);
                let item = DATA;
                let dst = DATA + 0x400;
                c.g[A0][0] = item as u64;
                c.bus.write32(item + 0x18, dst);
                let v = match seed % 4 {
                    0 => 0,
                    1 => -1i32,
                    2 => r.next_u32() as i32,
                    _ => (r.next_u32() % 1000) as i32,
                };
                c.bus.write32(item + 0x1c, v as u32);
                c.bus.write32(dst, r.next_u32());
            },
        );
    }

    #[test]
    fn menu_sys_float_item_reset_value_matches() {
        check(
            ssx3_ee::s09::f_002cdf78__generated,
            menu_sys_float_item_reset_value,
            200,
            &|c, seed| {
                let mut r = Rng(0x2cdf_7800 ^ seed as u64);
                scramble(c, &mut r);
                let item = DATA;
                let dst = DATA + 0x400;
                c.g[A0][0] = item as u64;
                c.bus.write32(item + 0x18, dst);
                let v = match seed % 5 {
                    0 => 0.0f32.to_bits(),
                    1 => (-1.0f32).to_bits(),
                    2 => 2.5f32.to_bits(),
                    3 => f32::MIN_POSITIVE.to_bits(),
                    _ => r.edge_f32(),
                };
                c.bus.write32(item + 0x1c, v);
                c.bus.write32(dst, r.next_u32());
            },
        );
    }

    // ------------------------------------------------------- menu items

    #[test]
    fn menu_sys_action_item_ctor_3_matches() {
        check(
            ee::s09::f_002ccf98__generated,
            menu_sys_action_item_ctor_3,
            SEEDS_6,
            &|c, seed| {
                machine(c, seed, 0x002c_cf98, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = (seed % 7) as u64;
                c.g[A3][0] = (seed % 11) as u64;
            },
        );
    }

    #[test]
    fn menu_sys_int8_item_draw_matches() {
        check(
            ee::s09::f_002cd7f0__generated,
            menu_sys_int8_item_draw,
            SEEDS_6,
            &|c, seed| {
                machine(c, seed, 0x002c_d7f0, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
                w(c, DATA + 0x18, DATA + 0x300);
                b(c, DATA + 0x300, (seed % 100) as u8);
                w(c, DATA + 0x14, (seed % 9) as u32);
            },
        );
    }

    #[test]
    fn menu_sys_int_item_draw_matches() {
        check(
            ee::s09::f_002cdc28__generated,
            menu_sys_int_item_draw,
            SEEDS_6,
            &|c, seed| {
                machine(c, seed, 0x002c_dc28, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
                w(c, DATA + 0x18, DATA + 0x300);
                w(c, DATA + 0x300, (seed.wrapping_mul(0x9e37)) as u32);
                w(c, DATA + 0x14, (seed % 9) as u32);
            },
        );
    }

    #[test]
    fn menu_sys_draw_matches() {
        check(
            ee::s09::f_002cc350__generated,
            menu_sys_draw,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x2cc350, 0);
                globals_2(c);
                let obj = OBJ2;
                w_2(c, GAME_GP - 0x3ff4, 0x3f00_0000);
                w_2(c, OBJ_2 + 0x58, obj);
                w_2(c, OBJ_2, if seed % 2 == 0 { 1 } else { 2 });
                w_2(c, OBJ_2 + 4, OBJ3);
                w_2(c, obj + 0x30, 0x4000_0000);
                w_2(c, obj + 0x34, 0x4040_0000);
                w_2(c, OBJ3 + 0x12c, VT);
                slot_3(c, VT, 0x28, 0, fakes::FAKE);
                c.g[A0][0] = OBJ_2 as u64;
                c.f[12] = 0x3f80_0000;
                c.f[13] = 0x4000_0000;
            },
        );
    }

    // ------------------------------------------------------------- menus

    pub(crate) fn menu_input_test(gen: GuestFn, hand: GuestFn, salt: u64, callback: bool) {
        check(gen, hand, 8, &|c, seed| {
            let mut r = base_2(c, seed, salt, 3);
            c.g[A0][0] = OBJ as u64;
            let menu = DATA + 0x2000;
            let page = DATA + 0x2200;
            let item = DATA + 0x2400;
            let disabled = seed % 4 == 1 || seed % 4 == 3;
            w(c, OBJ + 4, if disabled { 8 } else { 0 });
            w(c, OBJ, menu);
            w(c, menu + 0x124, page);
            let code = match seed % 4 {
                0 | 1 => 6u32,
                2 => 1,
                _ => 8,
            };
            w(c, page + 0x48, code);
            w(c, page, 1);
            w(c, page + 4, item);
            w(c, page + 8, item);
            let vt = vtable(c, &mut r, &[0x38]);
            w(c, item + 0x12c, vt);
            if callback {
                w(c, OBJ + 0x18, FAKE);
            }
        });
    }

    #[test]
    fn menu_sys_back_item_handle_input_matches() {
        menu_input_test(
            ee::s09::f_002cce38__generated,
            menu_sys_back_item_handle_input,
            0x002c_ce38,
            false,
        );
    }

    #[test]
    fn menu_sys_callback_item_handle_input_matches() {
        menu_input_test(
            ee::s09::f_002cd1d0__generated,
            menu_sys_callback_item_handle_input,
            0x002c_d1d0,
            true,
        );
    }

    #[test]
    fn menu_sys_text_edit_item_ctor_matches() {
        check(
            ee::s09::f_002ce418__generated,
            menu_sys_text_edit_item_ctor,
            6,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x002c_e418, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (r.next_u32() & 0xff) as u64;
                c.g[A2][0] = (DATA + 0x4000) as u64;
                c.bus.write8(DATA + 0x4000, b'n');
                c.bus.write8(DATA + 0x4001, 0);
            },
        );
    }

    /// Point every virtual slot_4 a test may reach at [`EMPTY`].
    pub(crate) fn empty_slots(c: &mut Cpu, vt: u32, offs: &[u32]) {
        for off in offs {
            slot_4(c, vt, *off, 0, EMPTY);
        }
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, size)` over a test arena, for
    /// paths that allocate beyond the size-class pools.
    pub(crate) fn heap_init_2(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const RET: u64 = 0x0012_3450;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = RET;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, RET as u32, "DLHeap_InitArena did not return");
    }

    #[test]
    fn menu_sys_item_handle_input_matches() {
        check(
            ssx3_ee::s09::f_002ca2b0__generated,
            menu_sys_item_handle_input,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ca2b0, 0);
                let item = DATA + 0x1000;
                let menu = DATA + 0x2000;
                let input = DATA + 0x3000;
                c.g[A0][0] = item as u64;
                w32(c, item, menu);
                w32(c, menu + 0x124, input);
                let code = match seed % 9 {
                    0 => 0,
                    1 => 1,
                    2 => 7,
                    3 => 8,
                    n => r.next_u32() % 10 + n as u32,
                };
                w32(c, input + 0x48, code);
            },
        );
    }

    #[test]
    fn menu_sys_page_on_activate_matches() {
        check(
            ssx3_ee::s09::f_002cbf60__generated,
            menu_sys_page_on_activate,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cbf60, 0);
                let page = DATA + 0x1000;
                c.g[A0][0] = page as u64;
                c.g[A1][0] = (seed % 3) as u64;
                let count = (seed % 4) as u32;
                let selected = if seed % 3 == 0 {
                    count + 1
                } else {
                    (seed % 5) as u32
                };
                w32(c, page, count);
                w32(c, page + 4, selected);
                // Item array lives at page + 0xc, indexed by the selection.
                let vt = DATA + 0x5000;
                slot_4(c, vt, 0x30, 0, LOAD_B0);
                slot_4(c, vt, 0x18, 0, LOAD_B0);
                for i in 0..8u32 {
                    let item = DATA + 0x3000 + i * 0x100;
                    w32(c, page + 0xc + i * 4, item);
                    w32(c, item + 0x10, vt);
                    w32(c, item + 0xb0, 1); // non-zero: no SelectNext walk
                }
                w32(c, DATA + 0x3000 + 0xb0, r.next_u32() | 1);
            },
        );
    }

    #[test]
    fn menu_sys_submenu_item_handle_input_matches() {
        check(
            ssx3_ee::s09::f_002ccd20__generated,
            menu_sys_submenu_item_handle_input,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ccd20, 0);
                let item = DATA + 0x1000;
                let menu = DATA + 0x2000;
                let input = DATA + 0x3000;
                let page = DATA + 0x4000;
                c.g[A0][0] = item as u64;
                w32(c, item, menu);
                w32(c, menu + 0x124, input);
                w32(
                    c,
                    input + 0x48,
                    match seed % 4 {
                        0 => 6,
                        1 => 0,
                        n => n as u32,
                    },
                );
                // disabled flag (`item+4 & 8`) and no base action
                w32(c, item + 4, if seed % 5 == 0 { 8 } else { 0 });
                w32(c, item + 0x18, page);
                // PushPage's item: `page+0x12c` vtable.
                let vt = DATA + 0x5000;
                slot_4(c, vt, 0x30, 0, EMPTY);
                w32(c, page + 0x12c, vt);
                let _ = r;
            },
        );
    }

    #[test]
    fn menu_sys_float_item_draw_matches() {
        check(
            ssx3_ee::s09::f_002ce100__generated,
            menu_sys_float_item_draw,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ce100, 0);
                pool_2(c);
                heap_init_2(c);
                be_library(c);
                let item = DATA + 0x1000;
                let value = DATA + 0x2000;
                let page = DATA + 0x3000;
                c.g[A0][0] = item as u64;
                c.g[A1][0] = (seed % 5) as u64;
                w32(c, item + 0x18, value);
                w32(c, value, r.f32(1000.0));
                w32(
                    c,
                    item + 0x24,
                    if seed % 4 == 0 {
                        0x3f80_0000
                    } else {
                        r.f32(10.0)
                    },
                );
                w32(c, item + 0x28, r.f32(100.0));
                w32(c, item, page);
                w32(c, item + 0x14, r.next_u32());
                // DrawRow: item is not the selected item (different page selection).
                w32(c, page + 4, 0);
                w32(c, page + 0xc, item);
                let vt = DATA + 0x5000;
                empty_slots(c, vt, &[0x18, 0x1c]);
                w32(c, item + 0x10, vt);
            },
        );
    }

    #[test]
    fn menu_sys_float_range_item_draw_matches() {
        check(
            ee::s09::f_002ce3a0__generated,
            menu_sys_float_range_item_draw,
            SEEDS_8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ce3a0, 3);
                be_library_2(c);
                let item = DATA;
                let row = DATA + 0x800;
                c.g[A0][0] = item as u64;
                c.g[A1][0] = row as u64;
                w(c, item, 0);
                w(c, item + 0x18, DATA + 0x100);
                w(c, DATA + 0x100, r.edge_f32());
                w(c, item + 0x14, r.next_u32());
            },
        );
    }

    #[test]
    fn menu_sys_group_item_on_focus_matches() {
        check(
            ee::s09::f_002cfe78__generated,
            menu_sys_group_item_on_focus,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x2cfe_78, 4);
                let self_ = DATA;
                let page = DATA + 0x18;
                w(c, self_, 0);
                w(c, self_ + 0x18, 0);
                let mode = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => 2 + (seed % 7) as u64,
                };
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = mode;
                let _ = page;
            },
        );
    }

    #[test]
    fn menu_sys_int_item_ctor_3_matches() {
        check(
            ee::s09::f_002cd8f8__generated,
            menu_sys_int_item_ctor_3,
            80,
            &|c, seed| {
                let _ = base_4(c, seed, 0x2cd8, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = seed as u64;
                c.g[A2][0] = (seed as u64) << 1;
                c.g[A3][0] = (seed as u64) << 2;
                c.g[T0][0] = (seed as u64) << 3;
                c.g[T1][0] = (seed as u64) << 4;
            },
        );
    }

    #[test]
    fn menu_sys_color_swatch_item_measure_matches() {
        check(
            ee::s09::f_002cff88__generated,
            menu_sys_color_swatch_item_measure,
            50,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x2cff, 1);
                fill(c, &mut r, DATA, 0x40);
                w(c, DATA + 0x200 + 0x18, (seed % 4) as u32);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    // -----------------------------------------------------------------------
    // Menu system.

    #[test]
    fn menusyspage_handle_input_matches() {
        covered(
            ee::s09::f_002cacc0__generated,
            menusyspage_handle_input,
            SEEDS_9,
            &|c, seed| {
                base_5(c, seed, 0xcacc0, 1);
                let page = DATA + 0x100;
                let item = DATA + 0x200;
                c.g[A0][0] = sx(page);
                c.bus.write32(page + 4, (seed % 3) as u32);
                c.bus.write32(page + 0xc + 4 * (seed % 3), item);
                let vt = DATA + 0x300;
                slot_6(c, vt, 0x10, 0, FAKE);
                c.bus.write32(item + 0x10, vt);
                // The fake returns a forced code: 3 (next), 4 (prev) or other.
                let code = match seed % 3 {
                    0 => 3,
                    1 => 4,
                    _ => 0x11,
                };
                for n in 0..8u32 {
                    c.bus.write32(LOG_RESULTS + 4 * n, code);
                }
            },
        );
    }

    #[test]
    fn menusyspage_insert_item_matches() {
        check(
            ee::s09::f_002cbcc8__generated,
            menusyspage_insert_item,
            SEEDS_9,
            &|c, seed| {
                base_5(c, seed, 0xbcc8, 0);
                let item = DATA + 0x100;
                let page = DATA + 0x200;
                c.g[A0][0] = sx(item);
                c.g[A1][0] = sx(page);
                let count = 3u32;
                c.bus.write32(item, count);
                c.bus.write32(item + 4, (seed % 4) as u32);
                c.bus
                    .write32(item + 0x124, if seed % 2 == 0 { 0 } else { 1 });
                for i in 0..5u32 {
                    c.bus.write32(item + 0xc + 4 * i, 0x1000 + i);
                }
                c.g[A2][0] = match seed % 5 {
                    0 => sx(0xffff_ffff),
                    1 => sx(count),
                    _ => ((seed / 5) % 4) as u64,
                };
            },
        );
    }

    #[test]
    fn menusysactionitem_handle_input_matches() {
        check(
            ee::s09::f_002cd008__generated,
            menusysactionitem_handle_input,
            SEEDS_9,
            &|c, seed| {
                base_5(c, seed, 0xcd008, 0);
                let item = DATA + 0x100;
                let parent = DATA + 0x200;
                let input = DATA + 0x300;
                c.g[A0][0] = sx(item);
                c.bus.write32(item, parent);
                c.bus.write32(item + 4, if seed % 2 == 0 { 0 } else { 8 });
                c.bus.write32(item + 0x1c, 0x55);
                c.bus.write32(parent + 0x124, input);
                c.bus
                    .write32(input + 0x48, [0u32, 1, 2, 6, 7][(seed / 2 % 5) as usize]);
                c.bus.write32(parent + 0x12c, input);
                slot_6(c, input, 0x18, 0, FAKE);
            },
        );
    }

    /// A scratch object away from `DATA` (where pointers land).
    pub(crate) const OBJ_3: u32 = DATA + 0x1000;

    driven!(enum_draw_generated, ee::s09::f_002ccc38__generated);

    driven!(enum_draw_hand, menu_sys_enum_item_draw);

    #[test]
    fn menu_sys_enum_item_draw_matches() {
        check_events(enum_draw_generated, enum_draw_hand, 90, &|c, seed| {
            let mut r = base_6(c, seed, 0x2c_cc38);
            globals_5(c);
            // The page the row belongs to, and the item's own row.
            w(c, DATA, DATA + 0x6000);
            w(c, DATA + 0x14, 7);
            // Enum table: {name, value} pairs.
            let count = 1 + (seed % 3);
            w(c, DATA + 0x18, count);
            w(c, DATA + 0x1c, DATA + 0x2000);
            for i in 0..3u32 {
                w(c, DATA + 0x2000 + 8 * i, DATA + 0x7000);
                w(c, DATA + 0x2000 + 8 * i + 4, 0x100 + i);
            }
            // The selected value: matching one entry on most seeds.
            w(c, DATA + 0x20, DATA + 0x2100);
            w(
                c,
                DATA + 0x2100,
                if seed % 4 == 0 {
                    0x999
                } else {
                    0x100 + seed % 3
                },
            );
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x3000);
            schedule(c, &mut r, seed, ENUM_DRAW_ENTRY)
        });
    }

    #[test]
    fn menu_sys_float_item_ctor_2_matches() {
        check(
            ee::s09::f_002cdce8__generated,
            menu_sys_float_item_ctor_2,
            120,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x2c_dce8);
                globals_5(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
            },
        );
    }

    driven!(bolts_count_generated, ee::s01::f_0014d4c8);

    driven!(bolts_count_hand, bolts_count_children);

    // ----------------------------------------------------------- menu pages

    #[test]
    fn menu_sys_page_set_selection_matches() {
        check(
            ssx3_ee::s09::f_002cbe68__generated,
            menu_sys_page_set_selection,
            16,
            &|c, seed| {
                let _ = base_7(c, seed, 0x002c_be68);
                let page = DATA + 0x100;
                let items = DATA + 0x2000;
                clear_4(c, page, 0x100);
                let count = 1 + (seed % 4);
                w(c, page, count);
                w(c, page + 4, (seed % count) as u32);
                // The item array starts at +0xc but is indexed with `(index >> 2)`
                // by the original's own convention; fill both layouts with valid
                // item pointers.
                for i in 0..count {
                    let item = DATA + 0x3000 + 0x80 * i;
                    let vt = DATA + 0x4000 + 0x80 * i;
                    clear_4(c, item, 0x40);
                    clear_4(c, vt, 0x40);
                    w(c, item + 0x10, vt);
                    c.bus.write16(vt + 0x30, 0);
                    w(c, vt + 0x34, crate::difftest::fakes::FAKE + 0x10);
                    c.bus.write16(vt + 0x38, 0);
                    w(c, vt + 0x3c, crate::difftest::fakes::FAKE);
                    w(c, items + 4 * i, item);
                    w(c, page + 0xc + 4 * i, item);
                }
                w(c, crate::difftest::fakes::LOG_RESULTS, 1);
                w(c, crate::difftest::fakes::LOG_RESULTS + 4, 1);
                c.g[A0][0] = sx(page);
                c.g[A1][0] = match seed % 5 {
                    0 => 0xffff_ffff_ffff_ffff,
                    1 => 0,
                    _ => (seed % 5) as u64,
                };
            },
        );
    }

    /// An object with a vtable whose `+off` slot is `{delta, fn}`.
    pub(crate) fn vt_slot(c: &mut Cpu, obj: u32, off: u32, delta: i16, f: u32) {
        w_3(c, obj, DATA + 0x34_0000);
        c.bus.write16(DATA + 0x34_0000 + off, delta as u16);
        w_3(c, DATA + 0x34_0000 + off + 4, f);
    }

    #[test]
    fn menu_sys_pop_page_matches() {
        check(
            ssx3_ee::s09::f_002cc460__generated,
            menu_sys_pop_page,
            30,
            &|c, seed| {
                let _r = base_8(c, seed, 0x2cc460, 2);
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                let count = (seed % 3) as u32;
                w_3(c, this, count);
                for i in 0..3u32 {
                    let page = DATA + 0x2000 + i * 0x100;
                    w_3(c, this + 4 + 4 * i, page);
                    vt_slot(c, page + 0x120, 0x38, 0, 0);
                    w_3(c, page + 0x12c, page + 0x120);
                }
            },
        );
    }

    // ----------------------------------------------------------------- menus

    #[test]
    fn menu_sys_push_page_matches() {
        check(
            ee::s09::f_002cc3b8__generated,
            menu_sys_push_page,
            SEEDS_10,
            &|c, seed| {
                let _ = base_9(c, seed, 0x2cc3_b8, 6);
                let obj = SCRATCH_2;
                let item = obj + 0x1000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = item as u64;
                // The item list: 0, 1 or 2 pages, each with a vtable at_4 +0x12c.
                w(c, obj, (seed % 3) as u32);
                for i in 0..3u32 {
                    let e = obj + 0x100 + 4 * i;
                    w(c, e, obj + 0x200 + 0x40 * i);
                    let v = obj + 0x200 + 0x40 * i;
                    w(c, v + 0x100, 0);
                    w(c, v + 0x12c, obj + 0x800 + 0x40 * i);
                    slot_7(c, obj + 0x800 + 0x40 * i, 0x30, i);
                    slot_7(c, obj + 0x800 + 0x40 * i, 0x38, i + 1);
                }
                w(c, obj + 4, item);
                w(c, item + 0x12c, obj + 0x900);
                slot_7(c, obj + 0x900, 0x30, 4);
                slot_7(c, obj + 0x900, 0x38, 5);
            },
        );
    }

    #[test]
    fn menu_sys_vector_label_item_draw_matches() {
        check(
            ee::s09::f_002d0500__generated,
            menu_sys_vector_label_item_draw,
            SEEDS_10,
            &|c, seed| {
                let _ = base_9(c, seed, 0x2d05_00, 6);
                let obj = SCRATCH_2;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (obj + 0x1000) as u64;
                // Three positions converted to doubles, then a formatted row.
                let pos = obj + 0x2000;
                wf(c, pos, (seed as f32) * 0.5);
                wf(c, pos + 4, 1.0);
                wf(c, pos + 8, 2.0);
                w(c, obj + 0x14, pos);
                w(c, obj + 0x18, 0x40);
                w(c, obj + 0x1000 + 0x14, pos);
                w(c, obj + 0x1000 + 0x18, 0);
            },
        );
    }

    // ---------------------------------------------------------- label measure

    #[test]
    fn menu_sys_vector_label_item_measure_matches() {
        check(
            ee::s09::f_002d0450__generated,
            menu_sys_vector_label_item_measure,
            SEEDS_10,
            &|c, seed| {
                let _ = base_9(c, seed, 0x2d04_50, 6);
                let obj = SCRATCH_2;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (obj + 0x1000) as u64;
                let pos = obj + 0x2000;
                wf(c, pos, (seed as f32) * 0.25);
                wf(c, pos + 4, 1.5);
                wf(c, pos + 8, 3.0);
                w(c, obj + 0x14, pos);
                w(c, obj + 0x18, 0x48);
                w(c, obj + 0x1000 + 0x14, pos);
                w(c, obj + 0x1000 + 0x18, 0);
            },
        );
    }

    #[test]
    fn menu_sys_bool_item_handle_input_matches() {
        check(
            ee::s09::f_002cc578__generated,
            menu_sys_bool_item_handle_input,
            80,
            &|c, seed| {
                let _r = base_10(c, seed, 0x2cc5);
                c.g[A0][0] = DATA as u64;
                // Owner -> menu -> input code.
                c.bus.write32(DATA, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x124, DATA + 0x1100);
                let code = [2u32, 3, 6, 5][(seed % 4) as usize];
                c.bus.write32(DATA + 0x1100 + 0x48, code);
                c.bus.write32(DATA + 0x1000 + 0x12c, DATA + 0x1300);
                c.bus.write16(DATA + 0x1300 + 0x20, 0);
                c.bus.write32(DATA + 0x1300 + 0x24, FAKE);
                // Item flags and the bool's address.
                c.bus.write32(DATA + 4, if seed % 8 == 0 { 8 } else { 0 });
                c.bus
                    .write32(DATA + 0x18, if seed % 2 == 0 { 1 } else { 0 });
                c.bus
                    .write32(DATA + 0x1c, if seed % 2 == 0 { 1 } else { 0 });
                c.bus.write32(DATA + 0x20, DATA + 0x1200);
                c.bus.write32(DATA + 0x1200, seed & 1);
            },
        );
    }

    #[test]
    fn menu_sys_page_select_next_matches() {
        check_lohi(
            ee::s09::f_002cab08__generated,
            menu_sys_page_select_next,
            16,
            &|c, seed| {
                let mut r = base_11(c, seed, 0x2cab08, 3);
                let this = DATA;
                clear_5(c, this, 0x1000);
                c.g[A0][0] = this as u64;
                let count = 4u32;
                w(c, this, if seed % 2 == 0 { 0 } else { count - 1 });
                w(c, this + 4, count);
                let vt = vtable(c, &mut r, &[0x18, 0x30, 0x38]);
                for i in 0..count {
                    let item = DATA + 0x200 + 0x80 * i;
                    clear_5(c, item, 0x40);
                    w(c, item + 0x10, vt);
                }
                for i in 0..count {
                    w(c, this + 0xc + 4 * i, DATA + 0x200 + 0x80 * i);
                }
            },
        );
    }

    #[test]
    fn menu_sys_page_select_prev_matches() {
        check_lohi(
            ee::s09::f_002cabe0__generated,
            menu_sys_page_select_prev,
            16,
            &|c, seed| {
                let mut r = base_11(c, seed, 0x2cabe0, 3);
                let this = DATA;
                clear_5(c, this, 0x1000);
                c.g[A0][0] = this as u64;
                let count = 4u32;
                w(c, this, if seed % 2 == 0 { 0 } else { 1 });
                w(c, this + 4, count);
                let vt = vtable(c, &mut r, &[0x18, 0x30, 0x38]);
                for i in 0..count {
                    let item = DATA + 0x200 + 0x80 * i;
                    clear_5(c, item, 0x40);
                    w(c, item + 0x10, vt);
                }
                for i in 0..count {
                    w(c, this + 0xc + 4 * i, DATA + 0x200 + 0x80 * i);
                }
            },
        );
    }

    #[test]
    fn menu_sys_build_quad_matches() {
        same(
            ssx3_ee::s09::f_002cb350__generated,
            menu_sys_build_quad,
            200,
            &|c, seed| {
                let mut r = Rng(0x2cb350 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let v = DATA + 0x100;
                c.g[A0][0] = v as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = (DATA + 0x240) as u64;
                for i in 0..8 {
                    c.bus.write32(DATA + 0x200 + 4 * i, r.f32(1000.0));
                }
                for i in 0..4 {
                    c.bus.write32(DATA + 0x240 + 4 * i, r.f32(1000.0));
                }
                // D_004A289C -> draw module; slot[0x23c] = a short generated leaf
                // (MenuSys_IsActive) called with this = draw + 0x100.
                let draw = DATA + 0x2000;
                c.bus.write32(GAME_GP - 0x854, draw);
                let slot = DATA + 0x4000;
                c.bus.write32(draw + 0x10d8, slot);
                c.bus.write32(slot + 0x238, 0x100);
                c.bus.write32(slot + 0x23c, 0x002c_c098);
                c.bus.write32(draw + 0x100, 0);
            },
        );
    }

    pub(crate) fn wf_2(c: &mut Cpu, a: u32, bits: u32) {
        c.bus.write32(a, bits);
    }

    // -- 0x002cb180 MenuSysPage_BeginDraw --------------------------------

    #[test]
    fn menu_sys_page_begin_draw_matches() {
        covered(
            ssx3_ee::s09::f_002cb180__generated,
            menu_sys_page_begin_draw,
            60,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2cb180, 4);
                let ms = DATA + 0x1000;
                let page = DATA + 0x2000;
                let renderer = DATA + 0x3000;
                w(c, GAME_GP.wrapping_sub(0x854), ms);
                w(c, ms + 0xe84, page);
                w(c, ms + 0x10d8, renderer);
                for (i, off) in [0xa8u32, 0xd0, 0xf8, 0x110].into_iter().enumerate() {
                    c.bus
                        .write16(renderer + off, (r.next_u32() & 0x7ff0) as u16);
                    w(c, renderer + off + 4, FAKE + 0x10 * i as u32);
                }
                for i in 0..0x40u32 {
                    c.bus
                        .write8(page + i, (seed as u8).wrapping_add(i as u8) | 1);
                }
                w(c, page + 0x4, r.next_u32());
                w(c, page + 8, r.next_u32());
            },
        );
    }

    // -- 0x002d0d48 MenuSysPoint2DItem_Measure --------------------------

    #[test]
    fn menu_sys_point2d_item_measure_matches() {
        check(
            ssx3_ee::s09::f_002d0d48__generated,
            menu_sys_point2d_item_measure,
            100,
            &|c, seed| {
                let mut r = Rng(0x2d0d48 ^ (seed as u64) << 4);
                c.g[GP][0] = GAME_GP as u64;
                c.g[ssx3_ee_rt::cpu::reg::SP][0] = crate::difftest::SP as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                let item = DATA + 0x1000;
                let page = DATA + 0x2000;
                let menu = DATA + 0x3000;
                let font = DATA + 0x4000;
                let style = DATA + 0x5000;
                c.g[A0][0] = sx(item);
                c.g[A1][0] = sx(page);
                w(c, item, menu);
                w(c, page, menu);
                w(c, item + 0x14, DATA + 0x6000);
                w(c, item + 0x40, if seed % 2 == 0 { 1 } else { 0 });
                w(c, menu + 0x124, font);
                w(c, font + 0x58, style);
                wf_2(c, style + 0x14, 20.0f32.to_bits());
                wf_2(c, style + 0x34, 1.0f32.to_bits());
                wf_2(c, style + 0x38, 3.0f32.to_bits());
                wf_2(c, style + 0x3c, 4.0f32.to_bits());
                wf_2(c, item + 0x54, r.f32(100.0));
                wf_2(c, item + 0x64, r.f32(100.0));
                wf_2(c, item + 0x74, r.f32(100.0));
                for i in 0..0x20u32 {
                    w(c, item + 0x80 + 4 * i, r.next_u32());
                }
                // an empty string for the text measurement
                c.bus.write8(DATA + 0x6000, 0);
            },
        );
    }

    #[test]
    fn menu_sys_group_item_draw_matches() {
        const S: u64 = 0x002c_fd28;
        check_all(
            ee::s09::f_002cfd28__generated,
            menu_sys_group_item_draw,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, S, 32);
                let this = DATA + 0x2000;
                let menu = DATA + 0x4000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x1000); // the block's starting rect
                let count = 1 + (seed % 3) as u32;
                w(c, this + 0x18, count);
                w(c, this, menu);
                let cur = DATA + 0x5000 + (seed % count) * 0x100;
                w(c, this + 0x14, cur);
                for i in 0..count {
                    let it = DATA + 0x5000 + i * 0x100;
                    // The item list is embedded at self+0x24.
                    w(c, this + 0x24 + i * 4, it);
                    // Visibility bit 0, then arbitrary flags.
                    w(c, it + 0x4, ((r.next_u32() & 1) << 1) | (r.next_u32() & 1));
                    let vt = DATA + 0x8000 + i * 0x100;
                    w(c, it + 0x10, vt);
                    c.bus.write16(vt + 0x28, (r.next_u32() as u16) & 0xfff0);
                    w(c, vt + 0x2c, 0x4000);
                    // The row height the vtable draw would leave.
                    w(c, this + 0x14c + i * 0x10 + 0xc, r.next_u32());
                }
                // The page's selected item: sometimes this group itself.
                let idx = seed % count;
                w(c, menu + 4, idx);
                w(
                    c,
                    menu + 0xc + idx * 4,
                    if seed % 2 == 0 { this } else { DATA + 0x9000 },
                );
                for k in 0..4u32 {
                    w(c, DATA + 0x1000 + k * 4, r.edge_f32());
                }
            },
        );
    }

    pub(crate) const MENU: u32 = DATA + 0x1_0000;

    pub(crate) const MENU_ENTRY: u32 = DATA + 0x1_1000;

    #[test]
    fn menu_sys_update_matches() {
        check_all_2(
            ee::s09::f_002cc0c0__generated,
            menu_sys_update,
            20,
            &|c, seed| {
                let mut r = base_12(c, seed, 0x2cc0_c0, 16);
                c.pc = 0x002c_c0c0; // the dispatcher is entered at_5 its entry block
                c.g[A0][0] = sx(MENU);
                c.g[A1][0] = sx(seed);
                c.bus.write32(MENU + 0xbc, (seed % 4) as u32);
                c.bus
                    .write32(MENU + 0xc0, if seed % 3 == 0 { 0 } else { MENU_ENTRY });
                c.bus.write8(GAME_GP - 0xc88, (seed % 2) as u8);
                for i in 0..8u32 {
                    c.bus.write32(
                        MENU + 0xd8 + 4 * i,
                        if seed % 2 == 0 {
                            0
                        } else {
                            MENU_ENTRY + 0x100 * i
                        },
                    );
                }
                c.bus.write32(MENU_ENTRY + 0x8, 0);
                c.bus.write32(MENU + 0x9c, DATA + 0x1_2000);
                let _ = &mut r;
            },
        );
    }
}
