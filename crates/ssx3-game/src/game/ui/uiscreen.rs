//! `src/ui/uiscreen.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
use ssx3_ee as ee;
use crate::abi::r32;
use crate::call::Callee;
use crate::call::Flow;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::call::VSlot;
use crate::call::sp;
use crate::events::back_edge_unwinds;
use callee::*;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::Unwound;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
pub(crate) use crate::game::dirtysock::tags2::p;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_3;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::feasyncfile::at_2;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::fe::feasyncfile::low_2;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::festaterewards2::sh_r;
pub(crate) use crate::game::fe::feasyncfile::sb_r;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::main::gamestate::vslot;
pub(crate) use crate::game::camera::cameracontroller::delete_owner;
pub(crate) use crate::game::fe::feasyncfile::at as at_3;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_4;
pub(crate) use crate::game::camera::script::scriptcontroller::vslot as vslot_2;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_3;
pub(crate) use crate::game::camera::cameraalgorithms::lh;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_c558, ui_screen_dtor, "UIScreen_dtor"),
    (0x0039_c688, ui_screen_set_data, "UIScreen_SetData"),
    (0x0039_c728, uiscreen_find_keyframe, "UIScreen_FindKeyframe"),
    (0x0039_c7c0, ui_screen_get_primary_thread, "UIScreen_GetPrimaryThread"),
    (0x0039_c978, ui_screen_run_thread_frame, "UIScreen_RunThreadFrame"),
    (0x0039_cbc8, uiscreen_cmd_stop_thread, "UIScreen_CmdStopThread"),
    (0x0039_cbe0, ui_screen_cmd_goto_label, "UIScreen_CmdGotoLabel"),
    (0x0039_cc38, ui_screen_cmd_start_thread, "UIScreen_CmdStartThread"),
    (0x0039_cca8, ui_screen_cmd_play_anim, "UIScreen_CmdPlayAnim"),
    (0x0039_cd30, ui_screen_cmd_set_object_param, "UIScreen_CmdSetObjectParam"),
    (0x0039_cda0, uiscreen_cmd_send_event, "UIScreen_CmdSendEvent"),
    (0x0039_ce20, uiscreen_cmd_end_transition_in, "UIScreen_CmdEndTransitionIn"),
    (0x0039_ce48, ui_screen_cmd_begin_transition_out, "UIScreen_CmdBeginTransitionOut"),
    (0x0039_ce98, uiscreen_cmd_end_transition_out, "UIScreen_CmdEndTransitionOut"),
    (0x0039_d860, ui_screen_find_object, "UIScreen_FindObject"),
    (0x0039_d8c0, c_ui_scroll_dtor, "cUIScroll_dtor"),
    (0x0039_d968, cui_scroll_render, "cUIScroll_Render"),
    (0x0039_de68, ui_scroll_set_parts_visible, "cUIScroll_SetPartsVisible"),
    (0x0039_dfb0, cui_slider_load, "cUISlider_Load"),
    (0x0039_e2a0, ui_state_ctor, "UIState_ctor"),
    (0x0039_e318, ui_state_ctor_2, "UIState_ctor_2"),
    (0x0039_e390, ui_state_dtor, "UIState_dtor"),
    (0x0039_e688, ui_state_v13, "UIState_v13"),
    (0x0039_e6b8, ui_state_render, "UIState_Render"),
    (0x0039_e758, ui_state_add_screen, "UIState_AddScreen"),
    (0x0039_fcc8, ui_object_set_anim, "UIObject_SetAnim"),
    (0x0039_fd38, ui_object_clear_anim, "UIObject_ClearAnim"),
    (0x003a_4868, ui_list_node_dtor_body, "UIListNode_dtorBody"),
    (0x003a_5338, ui_menu_flags_ctor, "UIMenuFlags_ctor"),
    (0x003a_5508, ui_pair_flags_ctor, "UIPairFlags_ctor"),
];

// ---------------------------------------------------------------------------
// Callees.

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
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SOUND_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR_2 as UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_REMOVE_TICK_CALLBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::CW_SCRIPT_CACHE_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE_2 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_CAMERA_SET_CONTROLLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MENU_ON_LOBBY_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_FORWARD_EVENT_TO_UI;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LUNO_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_PLAY_CHARACTER_TUNE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_GAME_UTIL_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::CSETUP_MAN_SEND_PACKET;
    pub(crate) use crate::game::dirtysock::tags2::callee::CVOIP_MAN_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SAVE_EMIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_HANDLE_MUSIC_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_ARCADE_SAY;
    pub(crate) use crate::game::dirtysock::tags2::callee::EATIMER_REMOVE_CALLBACK;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_STATE_CTOR_2_2 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_BASE_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_ON_CHILD_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_HAS_RENDER_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCARD_INSTALL_CALLBACKS;
    pub(crate) use crate::game::fe::feasyncfile::callee::PLAYBACK_RESTORE_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CHUNK_RESOLVE_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_RENDER_FADE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_RAIL_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_MODIFIER_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUTOSAVE_UPDATE_ICON;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_ANIM_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_DTOR_2 as WORLD_OBJ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_RUN_KILL_SCRIPT_2 as WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MOD_OBJ_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_LOCALE_LOAD_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_9 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_READ_QUOTED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_TRICK_METER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MENU_HIDE_ROOM_TABS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_3 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_2 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_2 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::BX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_INIT_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_CLEAR_EMITTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_BLOCK_POOL_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_MEMMAN_CREATE_SECTION_ALLOCATOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_4 as UI_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2_2 as WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_SPRINGS_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_BUILD_MESH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_UPDATE_VERTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_FINISH_CLEAR_FLAG40;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_ANIM_MAP_SET_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::ACTIVE_CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_STACK_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_ERROR_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_CREATED_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_9 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_SHOW_ROOM_TAB;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_UPDATE_PLAYERS_IN_ROOM;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_UPDATE_3 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_IS_FLAG_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_EMPTY_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_NEXT_SECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_2 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CAM_BUILD_MATRIX_FROM_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_MEM_CARD_CLEAR_DIR_CACHE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_CHANNEL_GET_TRACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TYPE0;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TYPE1;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TYPE2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TYPE3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_CAMERA_ALGORITHM_CTOR;


    // UI (shard 13).
    pub const C_UI_GROUP_SET_FOCUS: Callee = Callee {
        addr: 0x0039_8638,
        f: ee::s13::f_00398638,
    };
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
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
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {
    pub(crate) use crate::game::fe::feasyncfile::table::STR_HASH;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_MENU_RESET_SCROLL;
    pub(crate) use crate::game::fe::feasyncfile::table::PS2_MEM_CARD_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::table::PATH_STOP_TRACKS;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::table::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_START;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::table::AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::feasyncfile::table::ASYNC_FILE_GET_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::table::SCRIPT_ENGINE_GET_RESULT_VTABLE;
    pub(crate) use crate::game::fe::feasyncfile::table::RIDER_MODEL_FIND;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_GAME_MODE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_EVENT_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_END_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_BEGIN_PLAYBACK;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::table::OPERATOR_NEW_TAG;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::feasyncfile::table::SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_GET_UNPACKED_SIZE;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_UNPACK;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_TREE_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE2;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_CALC;
    pub(crate) use crate::game::fe::feasyncfile::table::STR_CASECMP;
}


// ---------------------------------------------------------------------------
// UI screen commands

/// `void UIScreen_CmdStopThread(void *ctx)` (0x39cbc8): clear the "thread
/// running" bit (bit 16) of the flag word at `a3 + 0x10`.
pub fn uiscreen_cmd_stop_thread(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = Addr(c.g[A3][0] as u32).off(0x10).lw(c);
    c.g[V0][0] = 0xffff_ffff_feff_0000;
    c.g[V0][0] = c.g[V0][0] | 0xffff;
    c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
    Addr(c.g[A3][0] as u32).off(0x10).sw(c, c.g[V1][0]);
    abi::ret(c);
}


/// `void UIScreen_CmdEndTransitionIn(void *ctx)` (0x39ce20): set the transition
/// state bits (0x300) of the screen's flag word when the screen exists.
pub fn uiscreen_cmd_end_transition_in(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A0][0] = Addr(abi::arg(c, 0)).off(0xd0).lw(c);
    let present = c.g[A0][0] != 0;
    c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
    if present {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x1c).lw(c);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | 0x300;
        Addr(c.g[A0][0] as u32).off(0x1c).sw(c, c.g[V0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `void UIScreen_CmdEndTransitionOut(void *ctx)` (0x39ce98): clear the thread
/// bit of `a3 + 0x10` and set state bits 0x700 on the screen's flag word.
pub fn uiscreen_cmd_end_transition_out(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V1][0] = Addr(c.g[A3][0] as u32).off(0x10).lw(c);
    c.g[V0][0] = 0xffff_ffff_feff_0000;
    c.g[V0][0] = c.g[V0][0] | 0xffff;
    c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
    Addr(c.g[A3][0] as u32).off(0x10).sw(c, c.g[V1][0]);
    c.g[A0][0] = Addr(abi::arg(c, 0)).off(0xd0).lw(c);
    let present = c.g[A0][0] != 0;
    c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
    if present {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x1c).lw(c);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | 0x700;
        Addr(c.g[A0][0] as u32).off(0x1c).sw(c, c.g[V0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `UIMenuFlags* UIMenuFlags_ctor(UIMenuFlags* f)` (0x3a5338): clears the low
/// nibble of the flag byte and returns the object.
pub fn ui_menu_flags_ctor(c: &mut Cpu) {
    c.cycles += 5;
    let selfp = abi::arg(c, 0);
    c.g[V1][0] = c.lbu(selfp);
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] &= 0xf0;
    c.sb(selfp, c.g[V1][0]);
    abi::ret(c);
}


/// `void UIState_AddScreen(this, cListNode *node)` (0x0039e758): push the
/// node onto the screen list at `+0x24`.
pub fn ui_state_add_screen(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24)),
        callee::UI_LIST_PUSH_BACK,
        0x0039_e768,
    )
}


/// `void UIListNode_dtorBody(cListNode *node)` (0x003a4868): unlink the node.
pub fn ui_list_node_dtor_body(c: &mut Cpu) {
    forward(c, 7, 0x10, |_| {}, callee::UI_LIST_NODE_UNLINK, 0x003a_4878)
}


/// `UIPairFlags* UIPairFlags_ctor(UIPairFlags* f)` (0x3a5508): mask the two
/// pair-flag bytes (`+0` and `+1`) down to their top two bits and return the
/// object.
pub fn ui_pair_flags_ctor(c: &mut Cpu) {
    c.cycles += 8;
    let flags = r32(c, A0);
    c.g[A1][0] = Addr(flags.wrapping_add(1)).lbu(c);
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = Addr(flags).lbu(c);
    c.g[A1][0] = c.g[A1][0] & 0xc0;
    c.g[V1][0] = c.g[V1][0] & 0xc0;
    Addr(flags.wrapping_add(1)).sb(c, c.g[A1][0]);
    Addr(flags).sb(c, c.g[V1][0]);
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// UIState_v13.

/// `void UIState_v13(UIState* s, UIState* target)` (0x39e688): `target`'s
/// seventh virtual method called with 1 (`s`'s `+8` sub-object supplies the
/// vtable).
pub fn ui_state_v13(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x1;
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x8)).lw(c);
        let slot = VSlot::read(c, c.g[V0][0], 0x38);
        c.g[V1][0] = slot.delta;
        c.g[A2][0] = slot.func;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(slot.delta as u32));
        call_ptr(c, slot.func as u32, 0x0039_e6ac)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cUISlider_Load(cUISlider* self)` (0x39dfb0): same as the progress
/// bar, with the slider's range fields at `+0x78`/`+0x80`.
pub fn cui_slider_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_OBJECT_LOAD.call(c, 0x0039_dfc4)?;
        c.g[V1][0] = 0xa;
        c.g[V0][0] = 0x64;
        let s0 = c.g[S0][0];
        let v0 = c.g[V0][0];
        let v1 = c.g[V1][0];
        Addr(s0 as u32).off(0x80).sw(c, v0);
        Addr(s0 as u32).off(0x78).sw(c, v1);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void UIObject_ClearAnim(UIObject* o)` (0x39fd38): detach (`this, 3`) and
/// drop the animation object at `+0xc`.
pub fn ui_object_clear_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = c.lw(r32(c, S0).wrapping_add(0xc));
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !none {
            c.cycles += 6;
            c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x10));
            let vt = r32(c, V0);
            c.g[A0][0] = c.lh(vt.wrapping_add(8));
            c.g[V1][0] = c.lw(vt.wrapping_add(0xc));
            let method = r32(c, V1);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x0039_fd68)?;
            c.sw(r32(c, S0).wrapping_add(0xc), 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void UIScreen_CmdBeginTransitionOut(screen)` (0x39ce48): clear the
/// screen's focus and set the "transition out" mode (6) on its owner
/// (`+0xd0`), if any.
pub fn ui_screen_cmd_begin_transition_out(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
        C_UI_GROUP_SET_FOCUS.call(c, 0x0039_ce64)?;
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xd0)).lw(c);
        let owner = c.g[A0][0] as u32;
        c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
        if owner != 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(owner).off(0x1c).lw(c);
            c.g[V0][0] &= c.g[V1][0];
            c.g[V0][0] |= 0x600;
            Addr(owner).off(0x1c).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void UIScreen_RunThreadFrame(UIScreen* s, UIThread* t)` (0x39c978):
/// one screen-script frame of a thread (called by `UIScreen_Update` while
/// the thread's frame `+0xc` differs from its last run `+0xe`): step the
/// frame by `+0x12` towards the end frame `+0x10`, then run the commands of
/// the keyframe at the old frame.
///
/// Screen scripts are authored at 60 fps: above 60 Hz each thread runs one
/// frame per `rate / 60` ticks of its own (`tick::frame60_due`; its first
/// frame on the tick it starts, as at 60), so transitions and screen
/// animations keep their wall-clock speed. The screen update itself (the
/// widgets' layout, `cUIGroup_UpdateAnim` at the held frame) still runs
/// every tick. At 60 every call runs the original.
pub fn ui_screen_run_thread_frame(c: &mut Cpu) {
    let thread = r32(c, A1);
    let frame = c.bus.read16(thread.wrapping_add(0xc)) as u32;
    if !crate::tick::frame60_due(c, thread, frame) {
        abi::ret(c);
        return;
    }
    ee::s13::f_0039c978__generated(c);
    let frame = c.bus.read16(thread.wrapping_add(0xc)) as u32;
    crate::tick::frame60_ran(c, thread, frame);
}


/// `UIKeyframe* UIScreen_FindKeyframe(UIScreen* s, unsigned short frame)`
/// (0x39c728): walk the variable-length keyframe list at `s+0x34` until an
/// entry's `+8` matches `frame`; null when the list ends. 6 (+2, then
/// 3 (+4) per miss, 2 on a hit) + 2 cycles.
pub fn uiscreen_find_keyframe(c: &mut Cpu) {
    c.cycles += 6;
    let list = c.lw(p(r32(c, A0), 0x34)) as u32;
    c.g[A1][0] = c.g[A1][0] & 0xffff;
    c.g[A2][0] = 0;
    let count = c.lw(list);
    c.g[V1][0] = count;
    let mut entry = p(list, 4);
    c.g[A0][0] = sx(entry);
    if count != 0 {
        c.cycles += 2;
        c.g[A3][0] = count;
        loop {
            c.cycles += 3;
            let frame = Addr(p(entry, 8)).lhu(c);
            c.g[V0][0] = frame;
            if frame != c.g[A1][0] {
                let stride = c.lw(p(entry, 4));
                c.g[V1][0] = stride;
                c.cycles += 4;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                c.g[V0][0] = (c.g[A2][0] < c.g[A3][0]) as u64;
                entry = p(entry, stride as u32);
                c.g[A0][0] = sx(entry);
                if c.g[V0][0] != 0 {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_c748);
                    }
                    continue;
                }
                break;
            } else {
                c.cycles += 2;
                c.g[V0][0] = sx(entry);
                abi::ret(c);
                return;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// UI widgets.

/// `void UIScreen_CmdGotoLabel(UIScreen* s, void* key, void* out)` (0x39cbe0):
/// walk the screen's label list (count at `+0`, entries `{label, stride,
/// value}` at `+4`) and store the matching entry's `u16` at `out+0xc`.
pub fn ui_screen_cmd_goto_label(c: &mut Cpu) {
    c.cycles += 6;
    c.g[T1][0] = c.g[A0][0];
    c.g[T0][0] = 0;
    c.g[V1][0] = Addr(p_2(c, T1, 0x34)).lw(c);
    c.g[V0][0] = Addr(p_2(c, V1, 0)).lw(c);
    let empty = c.g[V0][0] == 0;
    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
    if !empty {
        loop {
            c.cycles += 4;
            c.g[V1][0] = Addr(p_2(c, A1, 0)).lw(c);
            c.g[V0][0] = Addr(p_2(c, A2, 4)).lw(c);
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A0][0] = Addr(p_2(c, T1, 0x34)).lw(c);
                c.cycles += 6;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                c.g[V1][0] = Addr(p_2(c, A1, 4)).lw(c);
                c.g[V0][0] = Addr(p_2(c, A0, 0)).lw(c);
                c.g[V0][0] = (c.g[T0][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_cbf8);
                }
                continue;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p_2(c, A1, 8)).lhu(c);
            c.sh(p_2(c, A3, 0xc), c.g[V0][0]);
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// UI screens and groups.

/// `void* UIScreen_FindObject(UIScreen* self, uint hash)` (0x39d860): walk the
/// screen's object array and return the first live object whose hash matches.
pub fn ui_screen_find_object(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = lw(c, p_3(c.g[A0][0], 0x3c));
    if c.g[V1][0] != 0 {
        c.cycles += 4;
        c.g[V0][0] = lw(c, p_3(c.g[A0][0], 0x38));
        c.g[V0][0] = lw(c, c.g[V0][0] as u32);
        c.g[A3][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[V1][0];
            loop {
                c.cycles += 3;
                c.g[V1][0] = lw(c, c.g[A2][0] as u32);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                let mut more;
                if c.g[V1][0] == 0 {
                    c.cycles += 1;
                    c.g[V0][0] = (c.g[A3][0] < c.g[A0][0]) as u64;
                    more = c.g[V0][0] != 0;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, p_3(c.g[V1][0], 0x38));
                    if c.g[V0][0] != c.g[A1][0] {
                        c.g[V0][0] = (c.g[A3][0] < c.g[A0][0]) as u64;
                        more = c.g[V0][0] != 0;
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = c.g[V1][0];
                        abi::ret(c);
                        return;
                    }
                }
                c.cycles += 2;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_d888);
                    }
                    continue;
                }
                break;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// UI screens.

/// `UIThread* UIScreen_GetPrimaryThread(UIScreen* screen)` (0x39c7c0): walk
/// the screen's UI list and return the first non-sentinel node that is not
/// hidden (bit 25 of its flags clear), or null.
pub fn ui_screen_get_primary_thread(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0039_c7c0 => {
                    c.cycles += 8;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_7840, 0x0039_c7d4)?;
                    c.g[S0][0] = c.g[V0][0];
                    let none = c.g[S0][0] == 0;
                    c.g[V0][0] = 0;
                    if none {
                        pc = 0x0039_c810;
                        continue;
                    }
                    pc = 0x0039_c7e0;
                }
                0x0039_c7e0 => {
                    c.cycles += 2;
                    pc = 0x0039_c800;
                }
                0x0039_c7e8 => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(at(c, S0, 0x10)).lw(c);
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 25) as u32);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let hidden = c.g[V0][0] != 0;
                    c.g[V0][0] = c.g[S0][0];
                    if hidden {
                        pc = 0x0039_c810;
                        continue;
                    }
                    pc = 0x0039_c7fc;
                }
                0x0039_c7fc => {
                    c.cycles += 1;
                    c.g[S0][0] = Addr(at(c, S0, 0x4)).lw(c);
                    pc = 0x0039_c800;
                }
                0x0039_c800 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039_7718, 0x0039_c808)?;
                    let sentinel = c.g[V0][0] == 0;
                    c.g[V0][0] = 0;
                    if sentinel {
                        if back_edge_unwinds(c, 0x0039_c7e8) {
                            return Ok(());
                        }
                        pc = 0x0039_c7e8;
                        continue;
                    }
                    pc = 0x0039_c810;
                }
                0x0039_c810 => {
                    c.cycles += 4;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = Addr(r32_2(c, SP)).ld(c);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0039_c7c0, 0x0039_c820);
                    return Ok(());
                }
            }
        }
    })
}


/// `void UIScreen_CmdSetObjectParam(UIScreen* screen, int a1, void* cmd)`
/// (0x39cd30): find the named object and apply the parameter command to it.
pub fn ui_screen_cmd_set_object_param(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let cmd = c.g[A2][0];
        c.g[S1][0] = cmd;
        c.g[A1][0] = Addr(at(c, S1, 0x4)).lw(c);
        call_ptr(c, 0x0039_d860, 0x0039_cd4c)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 10;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039_fd38, 0x0039_cd60)?;
            c.g[V0][0] = Addr(at(c, S0, 0x8)).lw(c);
            c.g[A2][0] = sx(at(c, S1, 0xc));
            c.g[A1][0] = Addr(at(c, S1, 0x8)).lw(c);
            c.g[A0][0] = Addr(r32_2(c, V0)).off(0x68).lh(c);
            c.g[V1][0] = Addr(r32_2(c, V0)).off(0x6c).lw(c);
            let target = r32_2(c, V1);
            c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
            call_ptr(c, target, 0x0039_cd7c)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void UIScreen_CmdStartThread(UIScreen* self, void* p2)` (0x39cc38): walk
/// the screen's frame-table entries and play the frame whose label matches
/// `*(p2 + 4)`.
pub fn ui_screen_cmd_start_thread(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[T0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(at_2(c, T0, 0x34)).lw(c);
        c.g[V1][0] = Addr(at_2(c, V0, 0)).lw(c);
        let empty = c.g[V1][0] == 0;
        c.g[A1][0] = sx(at_2(c, V0, 4));
        if !empty {
            loop {
                c.cycles += 4;
                c.g[V1][0] = Addr(at_2(c, A1, 0)).lw(c);
                c.g[V0][0] = Addr(at_2(c, A2, 4)).lw(c);
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[A0][0] = Addr(at_2(c, T0, 0x34)).lw(c);
                    break;
                }
                c.cycles += 6;
                c.g[A1][0] = Addr(at_2(c, A1, 8)).lhu(c);
                c.g[A0][0] = c.g[T0][0];
                c.g[A2][0] = 1;
                call_ptr(c, 0x0039_c870, 0x0039_cc78)?;
                c.g[RA][0] = Addr(at_2(c, SP, 0)).ld(c);
                c.cycles += 2;
                let ra = c.g[RA][0] as u32;
                c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
                c.pc = ra;
                return Ok(());
            }
            loop {
                c.cycles += 6;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V1][0] = Addr(at_2(c, A1, 4)).lw(c);
                c.g[V0][0] = Addr(at_2(c, A0, 0)).lw(c);
                c.g[V0][0] = (c.g[A3][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = sx(at_2(c, A1, 0).wrapping_add(c.g[V1][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_cc58);
                }
                loop {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(at_2(c, A1, 0)).lw(c);
                    c.g[V0][0] = Addr(at_2(c, A2, 4)).lw(c);
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A0][0] = Addr(at_2(c, T0, 0x34)).lw(c);
                        break;
                    }
                    c.cycles += 6;
                    c.g[A1][0] = Addr(at_2(c, A1, 8)).lhu(c);
                    c.g[A0][0] = c.g[T0][0];
                    c.g[A2][0] = 1;
                    call_ptr(c, 0x0039_c870, 0x0039_cc78)?;
                    c.g[RA][0] = Addr(at_2(c, SP, 0)).ld(c);
                    c.cycles += 2;
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
                    c.pc = ra;
                    return Ok(());
                }
            }
        }
        c.cycles += 1;
        c.g[RA][0] = Addr(at_2(c, SP, 0)).ld(c);
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}


/// `void UIObject_SetAnim(UIObject* self, void* anim, u8 mode, u16 flags)`
/// (0x39fcc8): clear the old animation, then set the new one's play mode.
pub fn ui_object_set_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0] & 0xff;
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at_2(c, S0, 0xc)).lw(c);
        let same = c.g[V0][0] == c.g[S1][0];
        c.g[S3][0] = c.g[A3][0] & 0xffff;
        if !same {
            c.cycles += 2;
            call_ptr(c, 0x0039_fd38, 0x0039_fd00)?;
        }
        c.cycles += 13;
        Addr(at_2(c, S0, 0x10)).sb(c, c.g[S2][0]);
        c.g[A0][0] = c.g[S1][0];
        Addr(at_2(c, S0, 0xc)).sw(c, c.g[S1][0]);
        c.g[A2][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S2][0] & 0xff;
        call_ptr(c, 0x0039_7468, 0x0039_fd18)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `UIState_ctor` (0x39E2A0): the base constructor of every UI state object
/// (list links, vtable, engine, id byte and the widget list at +0x24).
pub fn ui_state_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[A2][0] = 0xf;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 0xffff_ffff_ffff_ffff;
        c.sb(r32(c, S0).wrapping_add(0x14), c.g[S1][0]);
        c.g[V1][0] = 0x218;
        sw_at(c, S0, 0x4, c.g[S0][0]);
        c.g[V0][0] = low(c, V0).wrapping_add(0x46c8);
        sw_at(c, S0, 0, c.g[S0][0]);
        c.g[A0][0] = low(c, S0).wrapping_add(0x24);
        sw_at(c, S0, 0xc, 0);
        sw_at(c, S0, 0x20, 0);
        sw_at(c, S0, 0x10, c.g[A1][0]);
        c.sb(r32(c, S0).wrapping_add(0x15), c.g[A2][0]);
        c.sh(r32(c, S0).wrapping_add(0x1c), c.g[V1][0]);
        sw_at(c, S0, 0x8, c.g[V0][0]);
        callee::UI_LIST_CTOR.call(c, 0x0039_e2f4)?;
        c.sb(r32(c, S0).wrapping_add(0x44), c.g[S1][0]);
        c.g[V0][0] = c.g[S0][0];
        sw_at(c, S0, 0x40, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Constructors.

/// `func_0039E318` (0x39e318): the second half of a UI state's constructor.
/// Installs vtable `0x4946c8` at `+8`, the id byte `0x0f` at `+0x15`, clears
/// the flag words at `+0xc`/`+0x40` and constructs the widget list at `+0x24`
/// (`UIList_ctor`). Signature: `void ctor(this, owner, engine)`.
pub fn ui_state_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = 0xf;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = u64::MAX;
        sb_r(c, S0, 0x14, c.g[S1][0]);
        c.g[V1][0] = 0x218;
        sw_r(c, S0, 0x4, c.g[S0][0]);
        c.g[V0][0] = sx(low_2(c, V0).wrapping_add(0x46c8));
        sw_r(c, S0, 0x0, c.g[S0][0]);
        c.g[A0][0] = sx(low_2(c, S0).wrapping_add(0x24));
        sw_r(c, S0, 0xc, 0);
        sw_r(c, S0, 0x10, c.g[A1][0]);
        sb_r(c, S0, 0x15, c.g[A3][0]);
        sh_r(c, S0, 0x1c, c.g[V1][0]);
        sw_r(c, S0, 0x20, c.g[A2][0]);
        c.g[RA][0] = sx(0x0039_e36c);
        sw_r(c, S0, 0x8, c.g[V0][0]);
        callee::UI_LIST_CTOR_2.call(c, 0x0039_e36c)?;
        sb_r(c, S0, 0x44, c.g[S1][0]);
        c.g[V0][0] = c.g[S0][0];
        sw_r(c, S0, 0x40, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// UI screens.

/// `void UIScreen_CmdSendEvent(self, msg, cmd)` (0x39cda0): dispatch a
/// command through the screen's controller (`self+0xd0`), falling back to the
/// controller's child when the first handler declines.
pub fn uiscreen_cmd_send_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = c.g[S0][0];
        let ctrl = lw_at(c, p(r32(c, S0), 0xd0));
        c.g[A3][0] = ctrl;
        c.g[A2][0] = lw_at(c, p(r32(c, S1), 4));
        c.g[V1][0] = lw_at(c, p(ctrl as u32, 8));
        let func = vslot(c, c.g[V1][0], 0x80, V0);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, 0x0039_cdd8)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, 16, 0x20);
        } else {
            c.cycles += 5;
            c.g[V0][0] = lw_at(c, p(r32(c, S0), 0xd0));
            c.g[V1][0] = lw_at(c, p(c.g[V0][0] as u32, 0x10));
            c.g[A3][0] = lw_at(c, c.g[V1][0] as u32);
            let none = c.g[A3][0] == 0;
            c.g[A1][0] = c.g[S0][0];
            if none {
                c.cycles += 1;
                frame.restore(c, 16, 0x20);
            } else {
                c.cycles += 6;
                c.g[V1][0] = lw_at(c, p(c.g[A3][0] as u32, 4));
                c.g[A2][0] = lw_at(c, p(r32(c, S1), 4));
                let func = vslot(c, c.g[V1][0], 0x18, V0);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, 0x0039_ce0c)?;
                c.cycles += 1;
                frame.restore(c, 16, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// UI commands.

/// `void UIScreen_CmdPlayAnim(UIScreen* self, int, UIMsg* msg)` (0x39cca8):
/// play the animation named by `msg[+4]` on the object named by `msg[+8]`.
pub fn ui_screen_cmd_play_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x0039_ccd4);
        c.g[A1][0] = Addr(p(r32(c, S0), 8)).lw(c);
        callee::UI_SCREEN_FIND_OBJECT.call(c, 0x0039_ccd4)?;
        c.g[S2][0] = c.g[V0][0];
        if c.g[S2][0] == 0 {
            frame.restore(c, S0, 0x40);
        } else {
            c.cycles += 7;
            c.g[V0][0] = Addr(p(r32(c, S1), 0xd0)).lw(c);
            c.g[A1][0] = Addr(p(r32(c, S0), 4)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x10)).lw(c);
            c.g[RA][0] = sx(0x0039_ccf4);
            c.g[A0][0] = sx(p(r32(c, A0), 0x50));
            callee::UI_ANIM_SET_CREATE_ANIM.call(c, 0x0039_ccf4)?;
            let missing = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S2][0];
            if !missing {
                c.cycles += 5;
                c.g[A2][0] = Addr(p(r32(c, S0), 0xc)).lbu(c);
                c.g[A3][0] = Addr(p(r32(c, S3), 8)).lhu(c);
                c.g[A1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0039_cd10);
                c.g[T0][0] = 0;
                callee::UI_OBJECT_SET_ANIM.call(c, 0x0039_cd10)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cUIScroll_dtor(cUIScroll* self, int flags)` (0x39d8c0): release the
/// two child objects and run the base destructor.
pub fn c_ui_scroll_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0049_44c8);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(p(r32(c, S0), 0x74)).lw(c);
        let first_null = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !first_null {
            c.cycles += 6;
            c.g[A1][0] = 0x3;
            delete_owner(c, 8, 0x0039_d904)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(r32(c, S0), 0x78)).lw(c);
        let second_null = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !second_null {
            c.cycles += 5;
            delete_owner(c, 8, 0x0039_d924)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0039_d930);
        c.g[A1][0] = c.g[S1][0];
        callee::UI_OBJECT_DTOR.call(c, 0x0039_d930)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cUISlider_dtor(cUISlider* self, int flags)` (0x39df28): same shape as
/// [`c_ui_scroll_dtor`] with the slider's child slots at +0x88 and +0x8c.
pub fn c_ui_slider_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0049_4348);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(p(r32(c, S0), 0x88)).lw(c);
        let first_null = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !first_null {
            c.cycles += 6;
            delete_owner(c, 8, 0x0039_df6c)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(r32(c, S0), 0x8c)).lw(c);
        let second_null = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !second_null {
            c.cycles += 5;
            delete_owner(c, 8, 0x0039_df8c)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0039_df98);
        c.g[A1][0] = c.g[S1][0];
        callee::UI_OBJECT_DTOR.call(c, 0x0039_df98)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// UI states.

/// `void UIScreen_SetData(self, data)` (0x39c688): replace the screen's data
/// block with an unpacked copy (or clear the unpacked flag).
pub fn ui_screen_set_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, table::EAC_GET_UNPACKED_SIZE, 0x0039_c6a8)?;
        let none = c.g[V0][0] == 0;
        c.g[A1][0] = 0x0049_0000;
        if !none {
            c.cycles += 14;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x0049_3e00;
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            call_ptr(c, table::OPERATOR_NEW_TAG, 0x0039_c6c4)?;
            c.g[A0][0] = c.g[S0][0];
            sw(c, S1, 0xc, c.g[V0][0]);
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, table::EAC_UNPACK, 0x0039_c6d8)?;
            c.g[V1][0] = lw_2(c, S1, 0x14);
            c.g[V1][0] |= 2;
            sw(c, S1, 0x14, c.g[V1][0]);
        } else {
            c.cycles += 4;
            c.g[V0][0] = lw_2(c, S1, 0x14);
            c.g[V1][0] = 0xffff_ffff_ffff_fffd;
            c.g[V0][0] &= c.g[V1][0];
            sw(c, S1, 0x14, c.g[V0][0]);
        }
        c.cycles += 12;
        sw(c, S1, 0x10, c.g[S0][0]);
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(8).lw(c);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        sw(c, S1, 0x34, c.g[V0][0]);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        sw(c, S1, 0x38, c.g[V1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `void UIState_Render(self)` (0x39e6b8): prepare the list and render each
/// visible node.
pub fn ui_state_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x10);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(this.wrapping_add(0x24));
        frame.save_fpr(c, 20, 0x30);
        call_ptr(c, table::UI_LIST_FIRST, 0x0039_e6d8)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 7;
            c.g[V0][0] = lw_2(c, S1, 0x8);
            c.mtc1(0, 20);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x70).lh(c);
            c.g[V1][0] = lw_2(c, V0, 0x74);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0039_e6fc)?;
            c.g[V0][0] = lw_2(c, S0, 0x54);
            'nodes: loop {
                c.cycles += 4;
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
                c.g[V0][0] &= 1;
                let hidden = c.g[V0][0] == 0;
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
                if !hidden {
                    c.cycles += 7;
                    c.g[V0][0] = lw_2(c, S0, 0x48);
                    c.mov_s(12, 20);
                    c.mov_s(13, 20);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x88).lh(c);
                    c.g[V1][0] = lw_2(c, V0, 0x8c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0039_e72c)?;
                }
                c.cycles += 5;
                c.g[S0][0] = lw_2(c, S0, 0x4);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, table::UI_LIST_NODE_IS_SENTINEL, 0x0039_e738)?;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = lw_2(c, S0, 0x54);
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_e700);
                    }
                    continue 'nodes;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
                break 'nodes;
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}


/// `void UIState_dtor(UIState* self, int flags)` (0x39e390): run the list and
/// node destructors, then free the object when `flags & 1`.
pub fn ui_state_dtor(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        'end: {
            c.cycles += 33;
            c.g[V0][0] = sx(0x0049_0000);
            frame.save(c, S3, 0x10);
            c.g[V0][0] = sx(0x0049_46c8);
            c.g[S3][0] = c.g[A0][0];
            frame.save(c, S1, 0x30);
            frame.save(c, S0, 0x40);
            c.g[S1][0] = sx(at_4(c.g[S3][0], 0x24));
            frame.save(c, S2, 0x20);
            c.g[A0][0] = c.g[S1][0];
            frame.save_ra(c, 0);
            c.g[S2][0] = c.g[A1][0];
            c.g[RA][0] = sx(0x0039_e3c8);
            Addr(at_4(c.g[S3][0], 8)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x0039_7948, 0x0039_e3c8)?;
            c.g[V0][0] = sx(0x0049_0000);
            c.g[A0][0] = c.g[S1][0];
            c.g[V0][0] = sx(0x0049_4c98);
            c.g[RA][0] = sx(0x0039_e3dc);
            Addr(at_4(c.g[S3][0], 0x3c)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x0039_7948, 0x0039_e3dc)?;
            c.g[S0][0] = sx(0x0049_0000);
            c.g[A0][0] = sx(at_4(c.g[S3][0], 0x30));
            c.g[S0][0] = sx(0x0049_4cc0);
            c.g[RA][0] = sx(0x0039_e3f0);
            Addr(at_4(c.g[S3][0], 0x38)).sw(c, c.g[S0][0]);
            call_ptr(c, 0x003a_4868, 0x0039_e3f0)?;
            Addr(at_4(c.g[S3][0], 0x2c)).sw(c, c.g[S0][0]);
            c.g[RA][0] = sx(0x0039_e3fc);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003a_4868, 0x0039_e3fc)?;
            Addr(at_4(c.g[S3][0], 8)).sw(c, c.g[S0][0]);
            c.g[RA][0] = sx(0x0039_e408);
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x003a_4868, 0x0039_e408)?;
            c.g[S2][0] &= 1;
            let t = c.g[S2][0] == 0;
            frame.restore(c, S0, 0x40);
            if t {
                break 'end;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x0039_e41c);
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0031_7e50, 0x0039_e41c)?;
            frame.restore(c, S0, 0x40);
            break 'end;
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cUIScroll_Render(self)` (0x39d968): render the scroll thumb's
/// decoration (f12/f13 carry the caller's screen-space offsets).
pub fn cui_scroll_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.swc1(21, p(c.g[SP][0] as u32, 0x28));
        c.swc1(20, p(c.g[SP][0] as u32, 0x20));
        c.g[A1][0] = Addr(p(this as u32, 0x74)).lw(c);
        let no_thumb = c.g[A1][0] == 0;
        if !no_thumb {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(this as u32, 0x78)).lw(c);
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
                c.cycles += 5;
                frame.restore_ra(c, 0);
                frame.restore_fpr(c, 21, 0x28);
                frame.restore_fpr(c, 20, 0x20);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 8;
            c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 0x14)).lw(c);
            c.lwc1(0, p(this as u32, 0x44));
            c.lwc1(1, p(this as u32, 0x48));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
            c.add_s(21, 12, 0);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            let skip = c.g[V0][0] == 0;
            c.add_s(20, 13, 1);
            if !skip {
                c.cycles += 7;
                c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 8)).lw(c);
                c.mov_s(12, 21);
                c.mov_s(13, 20);
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x88)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x8c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0039_d9d4)?;
            }
            c.cycles += 6;
            c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x78)).lw(c);
            c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 0x14)).lw(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            let skip2 = c.g[V0][0] == 0;
            c.mov_s(12, 21);
            if !skip2 {
                c.cycles += 6;
                c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 8)).lw(c);
                c.mov_s(13, 20);
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x88)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x8c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0039_da04)?;
            }
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 5;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// `cUIScroll_SetPartsVisible` (0x0039de68).
//
// `void cUIScroll::SetPartsVisible(self, bool visible)`: forward the
// visibility flag to the scroll bar, track and thumb objects.

pub fn ui_scroll_set_parts_visible(c: &mut Cpu) {
    guest(c, |c| {

        'b_39de9c: {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
        let t = c.g[A2][0] == 0u64;
        c.g[S1][0] = c.g[A1][0];
        if t {
        break 'b_39de9c;
        } else {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = vslot_2(c, c.g[V0][0], 0x48, V1);
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039de9c);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039de9c)?;
        break 'b_39de9c;
        }
        }
        'b_39debc: {
        c.cycles += 3;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
        let t = c.g[A2][0] == 0u64;
        c.g[A1][0] = c.g[S1][0];
        if t {
        break 'b_39debc;
        } else {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = vslot_2(c, c.g[V0][0], 0x48, V1);
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039debc);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039debc)?;
        break 'b_39debc;
        }
        }
        'b_39dedc: {
        c.cycles += 3;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
        let t = c.g[A2][0] == 0u64;
        c.g[A1][0] = c.g[S1][0];
        if t {
        break 'b_39dedc;
        } else {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = vslot_2(c, c.g[V0][0], 0x48, V1);
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039dedc);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039dedc)?;
        break 'b_39dedc;
        }
        }
        c.cycles += 19;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = vslot_2(c, c.g[V0][0], 0x48, V1);
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039def8);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039def8)?;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = vslot_2(c, c.g[V0][0], 0x48, V1);
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039df14);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039df14)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t; return Ok(());

        Ok(())
    })
}


/// `void UIScreen_dtor(UIScreen* self, uint flags)` (0x39c558): free the
/// screen's name/child blocks, tear down its lists and node bodies, then
/// release the object itself if `flags & 1`.
pub fn ui_screen_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let f = Frame::open(c, 0x80);
        c.g[V0][0] = 0x0049_0000;
        f.save(c, S5, 0x20);
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_add(0x4670));
        f.save(c, S6, 0x10);
        c.g[S5][0] = c.g[A0][0];
        f.save(c, S0, 0x70);
        f.save(c, S1, 0x60);
        f.save(c, S2, 0x50);
        f.save(c, S3, 0x40);
        f.save(c, S4, 0x30);
        f.save_ra(c, 0);
        c.sw((c.g[S5][0] as u32).wrapping_add(8), c.g[V0][0]);
        c.g[A0][0] = Addr((c.g[S5][0] as u32).wrapping_add(0x3c)).lw(c);
        let t = c.g[A0][0] == 0;
        c.g[S6][0] = c.g[A1][0];
        if !t {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e98, 0x0039c5a0)?;
        }
        // The two name/child blocks, freed when the flag bit is clear.
        c.cycles += 3;
        c.g[A0][0] = Addr((c.g[S5][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[S1][0] = 0x0049_0000;
        let empty = c.g[A0][0] == 0;
        if empty {
            c.cycles += 1;
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr((c.g[S5][0] as u32).wrapping_add(0x14)).lw(c) & 2;
            let skip = c.g[V0][0] == 0;
            c.g[V0][0] = 0x0049_0000;
            if !skip {
                c.cycles += 2;
                call_ptr(c, 0x0031_7e98, 0x0039c5c4)?;
                c.cycles += 1;
            }
        }
        c.g[V0][0] = 0x0049_0000;
        // Teardown of the embedded lists and nodes.
        c.cycles += 35;
        let obj = c.g[S5][0] as u32;
        c.g[S1][0] = sx(0x0049_0000u32.wrapping_add(0x4c98));
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_add(0x49e8));
        c.g[S2][0] = sx(obj.wrapping_add(0xb4));
        c.sw(obj.wrapping_add(0xcc), c.g[S1][0]);
        c.sw(obj.wrapping_add(0x48), c.g[V0][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::UI_LIST_DELETE_ALL.call(c, 0x0039c5e4)?;
        c.g[S4][0] = sx(obj.wrapping_add(0x40));
        c.g[S0][0] = 0x0049_0000;
        c.g[A0][0] = sx(obj.wrapping_add(0xc0));
        c.g[S0][0] = sx(0x0049_0000u32.wrapping_add(0x4cc0));
        c.g[S3][0] = sx(obj.wrapping_add(0x18));
        c.sw(obj.wrapping_add(0xc8), c.g[S0][0]);
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x0039c600)?;
        c.sw(obj.wrapping_add(0xbc), c.g[S0][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x0039c60c)?;
        c.g[A1][0] = 2;
        c.g[A0][0] = c.g[S4][0];
        callee::UI_OBJECT_DTOR.call(c, 0x0039c618)?;
        c.sw(obj.wrapping_add(0x30), c.g[S1][0]);
        c.g[A0][0] = c.g[S3][0];
        callee::UI_LIST_DELETE_ALL.call(c, 0x0039c624)?;
        c.sw(obj.wrapping_add(0x2c), c.g[S0][0]);
        c.g[A0][0] = sx(obj.wrapping_add(0x24));
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x0039c630)?;
        c.sw(obj.wrapping_add(0x20), c.g[S0][0]);
        c.g[A0][0] = c.g[S3][0];
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x0039c63c)?;
        c.sw(obj.wrapping_add(8), c.g[S0][0]);
        c.g[A0][0] = c.g[S5][0];
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x0039c648)?;
        c.g[V0][0] = c.g[S6][0] & 1;
        let t = c.g[V0][0] == 0;
        f.restore(c, S0, 0x70);
        if !t {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e50, 0x0039c65c)?;
        }
        c.cycles += 9;
        f.restore(c, S1, 0x60);
        f.restore(c, S2, 0x50);
        f.restore(c, S3, 0x40);
        f.restore(c, S4, 0x30);
        f.restore(c, S5, 0x20);
        f.restore(c, S6, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::put_vec;
    use ssx3_ee as ee;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fakes;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::SP as STACK_TOP;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::fakes::*;
    use crate::difftest::SP as STACK;
    use crate::difftest::elf_image;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::ai::ai::tests::SEEDS_2;
    pub(crate) use crate::game::ai::ai::tests::be_library;
    pub(crate) use crate::game::fe::fememcard::tests::put_slot;
    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear_4 as clear;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ3;
    pub(crate) use crate::game::object::movenode::tests::OBJ4;
    pub(crate) use crate::game::fe::ovstatelodge::tests::VT;
    pub(crate) use crate::game::fe::ovstatelodge::tests::slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_4 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_4 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_7 as pool_allocator_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base;
    pub(crate) use crate::game::object::effectlink::tests::rng_word;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init_2;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_6;
    pub(crate) use crate::game::main::gamestate::tests::base_3;
    pub(crate) use crate::game::main::gamestate::tests::slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::b;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_3;
    pub(crate) use crate::game::camera::cameracontroller::tests::fake_child;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run_2;
    pub(crate) use crate::game::ui::uiengine::tests::alloc;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_10 as pool_allocator_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_6 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_11 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_9 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_6;
    pub(crate) use crate::game::dirtysock::tags2::tests::slot_8 as slot_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE_DUMMY;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD_4 as WORLD_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base_14 as base_7;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::check3;


    #[test]
    fn uiscreen_cmd_stop_thread_matches() {
        check(ssx3_ee::s13::f_0039cbc8, uiscreen_cmd_stop_thread, 200, &|c, seed| {
            let mut r = Rng(0x39cb_c800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A3][0] = DATA as u64;
            c.bus.write32(DATA + 0x10, r.next_u32());
        });
    }


    #[test]
    fn uiscreen_cmd_end_transition_in_matches() {
        check(ssx3_ee::s13::f_0039ce20, uiscreen_cmd_end_transition_in, 200, &|c, seed| {
            let mut r = Rng(0x39ce_2000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            if seed % 3 == 0 {
                c.bus.write32(DATA + 0xd0, 0);
            } else {
                let screen = DATA + 0x400;
                c.bus.write32(DATA + 0xd0, screen);
                c.bus.write32(screen + 0x1c, r.next_u32());
            }
        });
    }


    #[test]
    fn uiscreen_cmd_end_transition_out_matches() {
        check(ssx3_ee::s13::f_0039ce98, uiscreen_cmd_end_transition_out, 200, &|c, seed| {
            let mut r = Rng(0x39ce_9800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A3][0] = (DATA + 0x800) as u64;
            c.bus.write32(DATA + 0x810, r.next_u32());
            if seed % 3 == 0 {
                c.bus.write32(DATA + 0xd0, 0);
            } else {
                let screen = DATA + 0x400;
                c.bus.write32(DATA + 0xd0, screen);
                c.bus.write32(screen + 0x1c, r.next_u32());
            }
        });
    }


    #[test]
    fn ui_menu_flags_ctor_matches() {
        check(ee::s13::f_003a5338__generated, ui_menu_flags_ctor, SEEDS, &|c, seed| {
            let mut r = rng(0x003a_5338, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.bus.write8(DATA, r.next_u32() as u8);
        });
    }


    #[test]
    fn ui_state_add_screen_matches() {
        check(ee::s13::f_0039e758__generated, ui_state_add_screen, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x39_e758, 0);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            match seed % 3 {
                0 => c.bus.write32(DATA + 0x1000 + 0x24 + 0xc, 0),
                1 => c.bus.write32(DATA + 0x1000 + 0x24 + 0xc, DATA + 0x900),
                _ => {
                    c.bus.write32(DATA + 0x1000 + 0x24 + 0xc, DATA + 0x900);
                    c.bus.write32(DATA + 0x900 + 4, DATA + 0xa00);
                }
            }
            c.bus.write32(DATA + 0x800 + 4, r.next_u32());
        });
    }


    #[test]
    fn ui_list_node_dtor_body_matches() {
        check(ee::s13::f_003a4868__generated, ui_list_node_dtor_body, 200, &|c, seed| {
            machine(c, seed, 0x3a_4868, 0);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            match seed % 3 {
                // Head of the list, no neighbours.
                0 => {}
                // Linked between two other nodes.
                1 => {
                    c.bus.write32(DATA + 0x1000, DATA + 0x800);
                    c.bus.write32(DATA + 0x1000 + 4, DATA + 0x900);
                }
                // Only a successor.
                _ => {
                    c.bus.write32(DATA + 0x1000, 0);
                    c.bus.write32(DATA + 0x1000 + 4, DATA + 0x1000);
                }
            }
        });
    }


    #[test]
    fn ui_pair_flags_ctor_matches() {
        crate::difftest::check(ee::s13::f_003a5508__generated, ui_pair_flags_ctor, 200, &|c, seed| {
            let mut r = Rng(0x3a55_0801 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA, r.next_u32());
        });
    }


    #[test]
    fn ui_state_v13_matches() {
        check(ee::s13::f_0039e688__generated, ui_state_v13, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0x39e6_88, 2);
            be_library(c);
            let obj = DATA + 0x800;
            let sub = DATA + 0x900;
            c.g[A0][0] = sx(DATA + 0x400);
            c.g[A1][0] = sx(obj);
            c.bus.write32(obj + 8, sub);
            let vt = fakes::vtable(c, &mut r, &[0x38]);
            c.bus.write32(sub, vt);
        });
    }


    #[test]
    fn cui_slider_load_matches() {
        check(ee::s13::f_0039dfb0__generated, cui_slider_load, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x39df_b0, 0);
            let obj = DATA + 0x400;
            let desc = DATA + 0x600;
            fill(c, &mut r, obj, 0x40);
            fill(c, &mut r, desc, 0x20);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = sx(desc);
            c.bus.write32(obj + 8, DATA + 0x700);
            put_slot(c, &mut r, DATA + 0x700, 0x48);
        });
    }


    #[test]
    fn ui_object_clear_anim_matches() {
        covered(ee::s13::f_0039fd38__generated, ui_object_clear_anim, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x39_fd38, 1);
            let obj = DATA + 0x1000;
            let anim = DATA + 0x2000;
            if seed % 2 != 0 {
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(anim + 0x10, vt);
                c.bus.write32(obj + 0xc, anim);
            }
            c.g[A0][0] = sx(obj);
            let _ = r.next_u32();
        });
    }


    // -----------------------------------------------------------------------
    // 0x39ce48 UIScreen_CmdBeginTransitionOut.

    #[test]
    fn ui_screen_cmd_begin_transition_out_matches() {
        check(ee::s13::f_0039ce48__generated, ui_screen_cmd_begin_transition_out, 16, &|c, seed| {
            let mut r = machine(c, seed, 0x39ce_48, 1);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            for i in 0..8u32 {
                c.bus.write32(this + 0x40 + i * 4, r.next_u32());
            }
            if seed % 2 == 0 {
                c.bus.write32(this + 0xd0, 0);
            } else {
                let owner = DATA + 0x300;
                c.bus.write32(this + 0xd0, owner);
                c.bus.write32(owner + 0x1c, r.next_u32());
            }
        });
    }


    // --------------------------------------------------------------- UI screen

    /// A thread at frame `frame` (speed `speed`, end `end`) of a screen whose
    /// keyframe list has an entry at frames 0..4 with `n` no-op commands.
    fn thread_fixture(c: &mut Cpu, r: &mut Rng, frame: u16, speed: i8, end: u16, n: u16) -> (u32, u32) {
        let screen = DATA;
        let thread = DATA + 0x80;
        let list = DATA + 0x100;
        c.bus.write32(list, 5);
        let mut at = list + 4;
        for i in 0..5u32 {
            let size = 0xc + 4 * n as u32;
            c.bus.write32(at + 4, size);
            c.bus.write16(at + 8, i as u16);
            c.bus.write16(at + 0xa, n);
            for k in 0..n as u32 {
                // UIScreen_ThreadNopA/B/C.
                c.bus.write8(at + 0xc + 4 * k, [0x22u8, 0x31, 0x40][(r.next_u32() % 3) as usize]);
                c.bus.write16(at + 0xc + 4 * k + 2, 4);
            }
            at += size;
        }
        c.bus.write32(screen + 0x34, list);
        c.bus.write16(thread + 0xc, frame);
        c.bus.write16(thread + 0xe, frame.wrapping_sub(1));
        c.bus.write16(thread + 0x10, end);
        c.bus.write8(thread + 0x12, speed as u8);
        c.g[A0][0] = sx(screen);
        c.g[A1][0] = sx(thread);
        (screen, thread)
    }

    #[test]
    fn ui_screen_run_thread_frame_matches() {
        check(ee::s13::f_0039c978__generated, ui_screen_run_thread_frame, 120, &|c, seed| {
            let mut r = Rng(0x39c9_7800 ^ seed as u64);
            scramble(c, &mut r);
            let frame = (seed % 7) as u16;
            let speed = [1i8, 2, -1, 0][(seed / 7 % 4) as usize];
            let end = [0xffffu16, 3, 0][(seed / 28 % 3) as usize];
            thread_fixture(c, &mut r, frame, speed, end, (seed % 3) as u16);
            // (Entered by a call: the generated body dispatches on pc.)
            c.pc = 0x0039_c978;
        });
        // The fixture runs the body (the frame steps when speed != 0).
        let mut c = crate::difftest::fresh();
        thread_fixture(&mut c, &mut Rng(1), 2, 1, 0xffff, 1);
        c.pc = 0x0039_c978;
        ui_screen_run_thread_frame(&mut c);
        assert_eq!((c.bus.read16(DATA + 0x80 + 0xc), c.bus.read16(DATA + 0x80 + 0xe)), (3, 2));
    }

    /// At 240 Hz a thread steps one frame per four ticks (60 per second),
    /// at 60 one per tick.
    #[test]
    fn ui_screen_run_thread_frame_rate() {
        for (hz, ticks) in [(60u32, 60u32), (240, 240)] {
            let mut c = crate::difftest::fresh();
            let mut r = Rng(0x39c978);
            crate::tick::set_for_test(&mut c, hz);
            let app = c.bus.read32(crate::tick::APPMAN_PTR);
            let (_, thread) = thread_fixture(&mut c, &mut r, 0, 1, 0xffff, 1);
            for t in 1..=ticks {
                c.bus.write32(app + crate::tick::APPMAN_VSYNC, t);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(thread);
                c.g[RA][0] = sx(0x0010_0000);
                // (Entered by a call, like UIScreen_Update does.)
                c.pc = 0x0039_c978;
                ui_screen_run_thread_frame(&mut c);
            }
            assert_eq!(c.bus.read16(thread + 0xc), 60, "{hz} Hz");
        }
    }

    #[test]
    fn uiscreen_find_keyframe_matches() {
        check(ee::s13::f_0039c728__generated, uiscreen_find_keyframe, 80, &|c, seed| {
            let mut r = Rng(0x39c7_2811 ^ seed as u64);
            scramble(c, &mut r);
            let screen = DATA;
            let list = DATA + 0x100;
            let n = 1 + seed % 4;
            c.bus.write32(list, n);
            let mut at = list + 4;
            for i in 0..n {
                c.bus.write32(at + 4, 0x10);
                c.bus.write16(at + 8, (i * 7 % 5) as u16);
                fill(c, &mut r, at, 4);
                at += 0x10;
            }
            c.bus.write32(screen + 0x34, list);
            c.g[A0][0] = sx(screen);
            // 0..4 hits an entry, 5 never does.
            c.g[A1][0] = ((seed / 4) % 6) as u64;
        });
    }


    // ---------------------------------------------------------------- UI

    #[test]
    fn ui_screen_cmd_goto_label_matches() {
        check(
            ssx3_ee::s13::f_0039cbe0__generated,
            ui_screen_cmd_goto_label,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x39cbe0, 0);
                let screen = DATA;
                let list = DATA + 0x100;
                let key = DATA + 0x200;
                let out = DATA + 0x300;
                let n = (seed % 4) as usize;
                c.bus.write32(screen + 0x34, list);
                c.bus.write32(list, n as u32);
                let mut labels = [0u32; 4];
                for i in 0..n {
                    labels[i] = 0x1000 + 0x11 * i as u32;
                    let e = list + 4 + 0x20 * i as u32;
                    c.bus.write32(e, labels[i]);
                    c.bus.write32(e + 4, 0x20);
                    c.bus.write16(e + 8, 0x200 + 3 * i as u16);
                }
                let k = if n > 0 && seed % 2 == 0 {
                    labels[(seed as usize / 2) % n]
                } else {
                    0xdead_0000
                };
                c.bus.write32(key + 4, k);
                c.g[A0][0] = screen as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = key as u64;
                c.g[A3][0] = out as u64;
            },
        );
    }


    // ------------------------------------------------------------------
    // UI.

    #[test]
    fn ui_screen_find_object_matches() {
        check(ee::s13::f_0039d860__generated, ui_screen_find_object, SEEDS_3, &|c, seed| {
            let mut r = base(c, seed, 0x39d8_60);
            let screen = DATA + 0x100;
            clear(c, screen, 0x40);
            let count = DATA + 0x200;
            let objs = DATA + 0x300;
            clear(c, count, 4);
            clear(c, objs, 0x40);
            let hash = r.next_u32();
            if seed % 7 != 0 {
                let n = if seed % 5 == 0 { 0 } else { 1 + seed % 4 } as u32;
                w(c, count, n);
                w(c, screen + 0x38, count as u32);
                w(c, screen + 0x3c, objs as u32);
                for i in 0..n {
                    let o = DATA + 0x400 + 0x40 * i;
                    clear(c, o, 0x20);
                    if (seed >> (i + 1)) & 1 == 0 {
                        w(c, objs + 4 * i, o);
                        w(c, o + 0x38, if i == n - 1 && seed % 3 == 0 { hash } else { r.next_u32() });
                    } else {
                        w(c, objs + 4 * i, 0);
                    }
                }
            }
            c.g[A0][0] = sx(screen);
            c.g[A1][0] = hash as u64;
        });
    }


    #[test]
    fn ui_screen_get_primary_thread_matches() {
        check(
            ssx3_ee::s13::f_0039c7c0__generated,
            ui_screen_get_primary_thread,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x39c7c0, 0);
                let l = OBJ + 0x18;
                if seed % 3 == 0 {
                    w(c, l + 4, 0);
                } else {
                    let n0 = OBJ2;
                    let n1 = OBJ2 + 0x40;
                    let sentinel = OBJ2 + 0x80;
                    w(c, l + 4, n0);
                    w(c, n0, n0.wrapping_add(0x100));
                    w(c, n0 + 4, n1);
                    w(c, n1, n1.wrapping_add(0x100));
                    w(c, n1 + 4, sentinel);
                    w(c, sentinel, sentinel);
                    w(c, sentinel + 4, sentinel);
                    let hidden = if seed % 2 == 0 { 1u32 << 25 } else { 0 };
                    w(c, n0 + 0x10, hidden);
                    w(c, n1 + 0x10, if seed % 5 == 1 { 1 << 25 } else { 0 });
                }
                c.g[A0][0] = OBJ as u64;
            },
        );
    }


    #[test]
    fn ui_screen_cmd_set_object_param_matches() {
        check(
            ssx3_ee::s13::f_0039cd30__generated,
            ui_screen_cmd_set_object_param,
            40,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x39cd30, 0);
                let name = OBJ3 + 0x100;
                let count = OBJ2 + 0x800;
                for (i, b) in b"button\0".iter().enumerate() {
                    c.bus.write8(name + i as u32, *b);
                }
                if seed % 2 == 0 {
                    // One entry whose name is `name`: the find loop matches on
                    // its first iteration (`screen+0x3c` is the entry array,
                    // `screen+0x38` holds the count and the first entry).
                    w(c, OBJ + 0x3c, OBJ2);
                    w(c, OBJ2, OBJ3);
                    w(c, OBJ + 0x38, count);
                    w(c, count, 1);
                    w(c, OBJ3 + 0x38, name);
                } else {
                    // No entry array at all: the find returns null at once.
                    w(c, OBJ + 0x3c, 0);
                    w(c, OBJ + 0x38, 0);
                }
                w(c, OBJ3 + 0x8, VT);
                slot(c, VT, 0x68, 0);
                w(c, OBJ4 + 0x4, name);
                w(c, OBJ4 + 0x8, r.next_u32());
                w(c, OBJ4 + 0xc, r.next_u32());
                c.g[A0][0] = OBJ as u64;
                c.g[A2][0] = OBJ4 as u64;
            },
        );
    }


    #[test]
    fn ui_screen_cmd_start_thread_matches() {
        check(
            ee::s13::f_0039cc38__generated,
            ui_screen_cmd_start_thread,
            60,
            &|c, seed| {
                alloc_base(c, seed, 0x39cc_38);
                let list = DATA + 0x300;
                c.g[A0][0] = sx(DATA);
                c.g[A2][0] = sx(DATA + 0x200);
                let count = seed % 3;
                c.bus.write32(DATA + 0x34, list);
                c.bus.write32(list, count);
                c.bus.write32(DATA + 0x200 + 4, 1);
                for i in 0..count {
                    let e = list + 4 + i * 0x20;
                    let label = if seed % 2 == 0 { 1 } else { 2 };
                    c.bus.write32(e, label);
                    c.bus.write32(e + 4, 0x20);
                    c.bus.write16(e + 8, 1);
                }
            },
        );
    }


    #[test]
    fn ui_object_set_anim_matches() {
        check(ee::s13::f_0039fcc8__generated, ui_object_set_anim, 60, &|c, seed| {
            machine(c, seed, 0x39fc_c8, 0);
            let anim = DATA + 0x2000;
            let old = DATA + 0x2400;
            let chain = DATA + 0x3000;
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(anim);
            c.g[A2][0] = (seed % 4) as u64;
            c.g[A3][0] = rng_word(seed) as u64 & 0xffff;
            c.bus
                .write32(DATA + 0xc, if seed % 2 == 0 { old } else { anim });
            c.bus.write32(anim, chain);
            c.bus.write16(chain + 2, 5);
        });
    }


    #[test]
    fn ui_state_ctor_matches() {
        check(ee::s13::f_0039e2a0__generated, ui_state_ctor, SEEDS_4, &|c, seed| {
            machine(c, seed, 0x39_e2a0, 2);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = (DATA + 0x2200) as u64;
            c.pc = 0x0039_e2a0;
        });
    }


    #[test]
    fn ui_state_ctor_2_matches() {
        check(ee::s13::f_0039e318__generated, ui_state_ctor_2, SEEDS_5, &|c, seed| {
            machine(c, seed, 0x39e318, 2);
            be_library_2(c);
            world(c);
            heap_init_2(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            c.g[A2][0] = (DATA + 0x900) as u64;
        });
    }

    /// `UIListNode_IsSentinel`: returns 0 for a node whose `+4` and `+0` are
    /// not the node itself. Used as a "returns 0" stub behind a vtable slot_2.
    pub(crate) const RET_ZERO: u32 = 0x0039_7718;


    // -----------------------------------------------------------------------
    // UI screens and states.

    #[test]
    fn uiscreen_cmd_send_event_matches() {
        covered(
            ee::s13::f_0039cda0__generated,
            uiscreen_cmd_send_event,
            SEEDS_6,
            &|c, seed| {
                base_3(c, seed, 0xcda0, 2);
                let s0 = DATA + 0x100;
                let s1 = DATA + 0x180;
                let ctrl = DATA + 0x200;
                c.g[A0][0] = sx(s0);
                c.g[A2][0] = sx(s1);
                c.bus.write32(s1 + 4, 0x99);
                c.bus.write32(s0 + 0xd0, ctrl);
                c.bus.write32(ctrl, 0);
                let vt1 = DATA + 0x400;
                slot_2(c, vt1, 0x80, 0, if seed % 2 == 0 { FAKE } else { RET_ZERO });
                c.bus.write32(ctrl + 8, vt1);
                let holder = DATA + 0x300;
                c.bus.write32(ctrl + 0x10, holder);
                let obj2 = DATA + 0x340;
                c.bus.write32(holder, if seed % 3 == 0 { 0 } else { obj2 });
                let vt2 = DATA + 0x440;
                slot_2(c, vt2, 0x18, 0, FAKE);
                c.bus.write32(obj2 + 4, vt2);
            },
        );
    }


    // ------------------------------------------------------------------
    // UI commands.

    #[test]
    fn ui_screen_cmd_play_anim_matches() {
        check(ssx3_ee::s13::f_0039cca8__generated, ui_screen_cmd_play_anim, SEEDS_7, &|c, seed| {
            let mut r = machine(c, seed, 0x39cca8, 4);
            let screen = DATA + 0x100;
            let p1 = DATA + 0x200;
            let msg = DATA + 0x300;
            c.g[A0][0] = screen as u64;
            c.g[A1][0] = p1 as u64;
            c.g[A2][0] = msg as u64;
            let hash = r.next_u32();
            w_3(c, msg + 8, hash);
            w_3(c, msg + 4, r.next_u32());
            b(c, msg + 0xc, (r.next_u32() % 4) as u8);
            h(c, p1 + 8, (r.next_u32() % 0x10000) as u16);
            // The screen's object array.
            if seed % 3 == 0 {
                w_3(c, screen + 0x38, 0);
                w_3(c, screen + 0x3c, 0);
            } else {
                let obj = DATA + 0x400;
                let objs = DATA + 0x500;
                w_3(c, objs, obj);
                w_3(c, screen + 0x38, objs);
                w_3(c, screen + 0x3c, 1);
                w_3(c, obj + 0x38, hash);
            }
            // The animation bank: an empty list makes CreateAnim return null.
            let anim_set = DATA + 0x600;
            let bank = DATA + 0x650;
            w_3(c, screen + 0xd0, anim_set);
            w_3(c, anim_set + 0x10, bank - 0x50);
            w_3(c, bank, DATA + 0x700);
            w_3(c, DATA + 0x700, 0);
        });
    }


    #[test]
    fn c_ui_scroll_dtor_matches() {
        check(ssx3_ee::s13::f_0039d8c0__generated, c_ui_scroll_dtor, SEEDS_7, &|c, seed| {
            let mut r = machine(c, seed, 0x39d8c0, 3);
            be_library_3(c);
            world(c);
            let obj = DATA + 0x100;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = ((seed % 2) * 2) as u64;
            w_3(c, obj + 0x74, 0);
            w_3(c, obj + 0x78, 0);
            if seed % 3 != 0 {
                fake_child(c, obj, 0x74, 8, 0);
            }
            if seed % 4 != 0 {
                fake_child(c, obj, 0x78, 8, 1);
            }
            let _ = r.next_u32();
        });
    }


    // ------------------------------------------------------------------
    // UI.

    #[test]
    fn ui_screen_set_data_matches() {
        check(ssx3_ee::s13::f_0039c688__generated, ui_screen_set_data, 10, &|c, seed| {
            let _r = base_4(c, seed, 0x39c688, 3);
            let this = DATA + 0x1000;
            let data = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = data as u64;
            c.bus.write8(data, 0);
            c.bus.write8(data + 1, 0);
        });
    }


    #[test]
    fn ui_state_render_matches() {
        check(ssx3_ee::s13::f_0039e6b8__generated, ui_state_render, 20, &|c, seed| {
            let _r = base_4(c, seed, 0x39e6b8, 2);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            let list = this + 0x24;
            let sentinel = DATA + 0x5000;
            w(c, sentinel, sentinel);
            w(c, sentinel + 4, sentinel);
            if seed % 3 == 0 {
                w(c, list + 4, sentinel);
            } else {
                let node = DATA + 0x2000;
                w(c, list + 4, node);
                w(c, node, sentinel);
                w(c, node + 4, sentinel);
                w(c, node + 0x54, if seed % 2 == 0 { 0 } else { 1 << 6 });
                w(c, node + 0x48, DATA + 0x3000);
                w(c, DATA + 0x3000, DATA + 0x3100);
                c.bus.write16(DATA + 0x3100 + 0x88, 0x40);
                w(c, DATA + 0x3100 + 0x8c, 0);
            }
            w(c, this + 8, DATA + 0x4000);
            w(c, DATA + 0x4000, DATA + 0x4100);
            c.bus.write16(DATA + 0x4100 + 0x70, 0);
            w(c, DATA + 0x4100 + 0x74, 0);
        });
    }


    // ------------------------------------------------------------- UI dtor

    #[test]
    fn ui_state_dtor_matches() {
        check(ee::s13::f_0039e390__generated, ui_state_dtor, SEEDS_8, &|c, seed| {
            let _ = base_5(c, seed, 0x39e3_90, 4);
            // A real heap object, so the `flags & 1` free is valid.
            let obj = alloc(c, 0x60);
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 2) as u64;
            // The two lists and the node destructor state.
            w_4(c, obj + 0x24, 0);
            w_4(c, obj + 0x28, 0);
            w_4(c, obj + 0x30, 0);
            w_4(c, obj + 0x34, 0);
        });
    }


        // ------------------------------------------------------------------- UI

    #[test]
    fn cui_scroll_render_matches() {
        check(ee::s13::f_0039d968__generated, cui_scroll_render, 8, &|c, seed| {
            let mut r = base_6(c, seed, 0x0039_d968, 2);
            c.g[A0][0] = OBJ_2 as u64;
            let thumb = seed & 1 != 0;
            w_5(c, OBJ_2 + 0x74, if thumb { OBJ2_2 } else { 0 });
            w_5(c, OBJ_2 + 0x78, if seed % 3 == 0 { 0 } else { OBJ3_2 });
            w_5(c, OBJ_2 + 0x44, r.edge_f32());
            w_5(c, OBJ_2 + 0x48, r.edge_f32());
            for (obj, off) in [(OBJ2_2, 0x14u32), (OBJ3_2, 0x14u32)] {
                let flags = if seed % 4 == 0 { 0 } else { 0x40 };
                w_5(c, obj + off, flags);
                let vt = vtable(c, &mut r, &[0x88]);
                w_5(c, obj + 8, vt);
            }
        });
    }


                #[test]
    fn ui_scroll_set_parts_visible_matches() {
        strict(ee::s13::f_0039de68__generated, ui_scroll_set_parts_visible, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x39de, 12);
            let this = DATA;
            let vt = DATA + 0x1000;
            slot_5(c, vt, 0x48, FAKE);
            for (i, off) in [0x7c, 0x80, 0x84, 0x78, 0x74].into_iter().enumerate() {
                let obj = DATA + 0x2000 + 0x100 * i as u32;
                c.bus.write32(this + off, if (seed as usize >> i) & 1 == 0 { 0 } else { obj });
                c.bus.write32(obj + 8, vt);
            }
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(r.next_u32() & 1);
        });
    }


    #[test]
    fn ui_screen_dtor_matches() {
        check3(ee::s13::f_0039c558__generated, ui_screen_dtor, 40, &|c, seed| {
            let mut r = base_7(c, seed, 0x39c558);
            let obj = DATA;
            // Zeroed screen: no names to free, empty lists.
            for i in 0..0x100u32 {
                c.bus.write32(obj + 4 * i, 0);
            }
            c.bus.write32(obj + 0x14, (r.next_u32() % 4) as u32);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = (seed % 4) as u64;
        });
    }
}
