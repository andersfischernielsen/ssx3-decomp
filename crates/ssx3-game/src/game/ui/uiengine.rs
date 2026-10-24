//! `src/ui/uiengine.cpp`: hand-written replacements (moved from the
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
use crate::events::back_edge_unwinds;
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::camera::cameraalgorithms::lh;
pub(crate) use crate::game::camera::script::scriptcontroller::vslot;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_3;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lh_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::dirtysock::tags2::vslot as vslot_2;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_4;
pub(crate) use crate::game::fe::feasyncfile::gp_18 as gp;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_2;
pub(crate) use crate::game::fe::feasyncfile::sb_r;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::fepopup3::gp_6 as gp_2;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::movie::movieplayer::at as at_2;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
pub(crate) use crate::game::wscript::wscriptman::p;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use callee::*;
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
    (0x0039_79f8, ui_list_accept_visitor, "UIList_AcceptVisitor"),
    (0x0039_7b08, ui_engine_ctor, "UIEngine_ctor"),
    (0x0039_7b70, ui_engine_dtor, "UIEngine_dtor"),
    (0x0039_7c40, ui_engine_load_package, "UIEngine_LoadPackage"),
    (
        0x0039_8018,
        ui_engine_render_state_mgr,
        "UIEngine_RenderStateMgr",
    ),
    (
        0x0039_8078,
        ui_engine_texture_set_dtor,
        "UIEngine_TextureSet_dtor",
    ),
    (0x0039_8380, ui_texture_set_find, "UITextureSet_Find"),
    (0x0039_83f0, ui_font_set_find_font, "UIFontSet_FindFont"),
    (0x0039_8438, ui_font_set_free_fonts, "UIFontSet_FreeFonts"),
    (0x0039_84b0, uifont_set_add_font, "UIFontSet_AddFont"),
    (0x0039_8638, cui_group_set_focus, "cUIGroup_SetFocus"),
    (0x0039_86b0, c_ui_group_find_by_id, "cUIGroup_FindById"),
    (0x0039_8738, ui_group_refresh_text, "cUIGroup_RefreshText"),
    (0x0039_8798, ui_group_handle_input, "cUIGroup_HandleInput"),
    (0x0039_87f8, c_ui_group_v18, "cUIGroup_v18"),
    (0x0039_8910, c_ui_group_update_anim, "cUIGroup_UpdateAnim"),
    (0x0039_8998, ui_icon_load, "UIIcon_Load"),
    (0x0039_9730, ui_icon_set_texture, "UIIcon_SetTexture"),
    (0x0039_9820, uilistbox_load, "UIListBox_Load"),
    (
        0x0039_9920,
        ui_list_box_get_mode_entry,
        "UIListBox_GetModeEntry",
    ),
    (
        0x0039_9d80,
        ui_list_box_set_item_wide_string,
        "UIListBox_SetItemWideString",
    ),
    (0x0039_9e28, uilistbox_refresh_text, "UIListBox_RefreshText"),
];

// ---------------------------------------------------------------------------
// Callees.

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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUY_POPUP_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::CLEAR_NEW_REWARDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::DYNAMIC_EMITTER_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::FE_ASYNC_SET_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_PROCESS_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GROW_LINES;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INIT_FROM_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INSTANCE_GET_NODE_MATRIX as INSTANCE_GET_NODE_MATRIX_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOAD_INDICATOR_RENDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_API_SEND;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_GET_ROOM_NAME;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_END_OPERATION;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_ON_OPERATION_DONE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MEM_REPORT_STATS_LOCKED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MSG_PRINTF;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_3;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_COPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TEX_MAN_INIT_TABLE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_CTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM as UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2 as UI_ANIM_SET_CREATE_ANIM_2_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST as UI_LIST_FIRST_4;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM as UI_OBJECT_SET_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT as UI_SCREEN_FIND_OBJECT_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_ASCII;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_EVENT_STAT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::VRAM_HEAP_INIT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::ACTIVE_CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_INSTANCE_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_TIMERS_BY_CALLBACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_4 as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_4 as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_MISSION_INT_GET_SESSION_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_HUMAN_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_SPECIAL_EVENT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_TRACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_CLEAR_EMITTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_INIT_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_STACK_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_ALLOC_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_FREE_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START_CAPTURED;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ASYNC_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_PLAYER_PAD_MASK_2 as FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_CREATED_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_ERROR_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_SHOW_ROOM_TAB;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_UPDATE_PLAYERS_IN_ROOM;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_BUILD_MESH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_SPRINGS_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_UPDATE_VERTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_ANIM_MAP_SET_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_ADVANCE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_QUIT_TO_LOAD_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_FINISH_CLEAR_FLAG40;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_NEXT_SECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_BINDING_EVAL_FLOAT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA_2 as IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_START_SEQUENCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_FIND_ENABLED_IN_COLUMN_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_CONNECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_TABLE_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_CALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_PASS_THROUGH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_AS_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_FORMAT_SLOT_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC_NAMED_128;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_EMPTY_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_IS_FLAG_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_EXTENSION_AND_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_5 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_9 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_5 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_UPDATE_3 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_DATA_FIND_MARKER;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_MEM_CARD_CLEAR_DIR_CACHE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_START_POSE_ANGLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_IS_LOADED;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_SHOW_HUD_MSG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_PLACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SCRIPT_OBJ_MATCHES_RIDER_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_3 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_3 as SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_TRANSITION_TARGET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_TICK_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_2 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CAM_BUILD_MATRIX_FROM_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_RIDER_GET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK_2 as SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK_2 as SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPCH_MAKE_ID_2 as SPCH_MAKE_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_IS_INSTANCE_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_CHARACTER_TO_SPEECH_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_FADE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_IS_SPEECH_VOLUME_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_FADE_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH as STR_HASH_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_4 as UI_LIST_FIRST_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_MENU_SET_SELECTION_3 as UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM_2 as UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_5 as UI_SCREEN_FIND_OBJECT_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_CTOR_2_4 as UI_STATE_CTOR_2_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS_2 as UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_SWITCH_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXTURE_SET_FIND as UI_TEXTURE_SET_FIND_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_ID_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_OWNED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_TEXT_ID_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF_ASCII_FMT;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_TO_ASCII;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_BLOCK_POOL_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_MEMMAN_CREATE_SECTION_ALLOCATOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2_2 as WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::AI_RAND;
    pub(crate) use crate::game::main::gamemode::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamemode::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GET_MEDAL_REWARD_ID;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GRANT_REWARD;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_IS_REWARD_GRANTED;
    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UNLOAD;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2;
    pub(crate) use crate::game::main::gamemode::callee::CBE_GET_BE;
    pub(crate) use crate::game::main::gamemode::callee::CBX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::main::gamemode::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_FREE_BUFFERS;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_RESET;
    pub(crate) use crate::game::main::gamemode::callee::CUI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::main::gamemode::callee::CVT_FLOAT_TO_INT;
    pub(crate) use crate::game::main::gamemode::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE;
    pub(crate) use crate::game::main::gamemode::callee::FE_ASYNC_MANAGER_SET_FILE_STATUS;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_RIDER_PREVIEW_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::main::gamemode::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::main::gamemode::callee::FUNC_0028F140;
    pub(crate) use crate::game::main::gamemode::callee::MEMCPY;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;
    pub(crate) use crate::game::main::gamemode::callee::MENU_SYS_ITEM_DRAW_ROW;
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
    pub(crate) use crate::game::main::gamemode::callee::SPRINTF;
    pub(crate) use crate::game::main::gamemode::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::main::gamemode::callee::STRCPY;
    pub(crate) use crate::game::main::gamemode::callee::STRLEN;
    pub(crate) use crate::game::main::gamemode::callee::STR_HASH;
    pub(crate) use crate::game::main::gamemode::callee::TEX_MAN_UNLOCK_VRAM;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_ACCEPT_VISITOR;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_FIRST as UI_LIST_FIRST_2;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_DTOR_BODY as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::main::gamemode::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF as UI_OBJECT_SHOW_CLAMP_ALPHA_HALF_2;
    pub(crate) use crate::game::main::gamemode::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::main::gamemode::callee::WSCRIPT_MAN_RESET_COLLECT_SET;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_1;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BEREWARD_INT_IS_PEAK_COMPLETE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_SCORE_INT_UPDATE_BEST_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_IS_CONNECTED;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_RECONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_3;
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
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_END_OPERATION_4 as MC_MANAGER_END_OPERATION;
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
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_CTOR_5 as NET_FE_STATE_CTOR_2;
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
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_RUN_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_UNLOAD_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::SPRINTF_2;
    pub(crate) use crate::game::movie::movieplayer::callee::STRCPY_2;
    pub(crate) use crate::game::movie::movieplayer::callee::STREAM_BUFFER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_LOAD_TEXTURES;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_STORE_WORD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_CTOR_2;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_MGR_ADD_BACKGROUND_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESSES;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESS_COUNT;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_MAN_CREATE_PROCESS;
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
    pub(crate) use crate::game::wscript::wscriptman::callee::ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::wscript::wscriptman::callee::APP_MODULE_UPDATE_FADE;
    pub(crate) use crate::game::wscript::wscriptman::callee::AVALANCHE_BAKE_NODE_HISTORY;
    pub(crate) use crate::game::wscript::wscriptman::callee::AV_PLAYER_INIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_POLL_FREE;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_SET_PITCH;
    pub(crate) use crate::game::wscript::wscriptman::callee::CAM_CONTROLLER_CLEAR_ALGORITHMS;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_CHUNK_RESOLVE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_MEMCARD_SET_STATE;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_CREATE_MESHES_2 as CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_FREE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FLAG_MANAGER_GROUP_LOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::DEBUG_MENU_MODULE_UPDATE_CURSOR_TRAIL;
    pub(crate) use crate::game::wscript::wscriptman::callee::DISABLE_INTC;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_FILE_MOUNT_BIG;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLEX_RAIL_UPDATE_SPACE_NODE;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLOATING_V49;
    pub(crate) use crate::game::wscript::wscriptman::callee::FOCUS_MAN_ADD_FOCUS_BOX;
    pub(crate) use crate::game::wscript::wscriptman::callee::LIGHT_MAN_EMPTY_HOOK_A;
    pub(crate) use crate::game::wscript::wscriptman::callee::MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::MOD_OBJ_DTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::wscript::wscriptman::callee::RAIL_MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::REMOVE_INTC_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::SET_FADE_GAIN;
    pub(crate) use crate::game::wscript::wscriptman::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_GET_STATE_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SET_USER_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::wscript::wscriptman::callee::WSCRIPT_MAN_CALL_FUNC;
}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_VEC_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::BXFILE_EXISTS;
    pub(crate) use crate::game::wscript::wscriptman::table::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::wscript::wscriptman::table::IS_REPLAY_PLAYING;
    pub(crate) use crate::game::wscript::wscriptman::table::MEM_FREE;
    pub(crate) use crate::game::wscript::wscriptman::table::SCE_MC_FLUSH;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32_2;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_BANK_REMOVE;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_MIDI_RESUME;
    pub(crate) use crate::game::wscript::wscriptman::table::STREAM_FILE_DESTROY;
    pub(crate) use crate::game::wscript::wscriptman::table::STR_HASH;
    pub(crate) use crate::game::wscript::wscriptman::table::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::wscript::wscriptman::table::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::wscript::wscriptman::table::WSCRIPT_PROCESS_LIST_POP_FRONT;
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
// UI font set

/// `void UIFontSet_AddFont(UIFontSet *set, int index, void *a, void *b)`
/// (0x3984b0): while fewer than 4 fonts are registered, store the two words at
/// `set + index*0xc + 4` and bump the count at +0x34.
pub fn uifont_set_add_font(c: &mut Cpu) {
    c.cycles += 4;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(0x34).lb(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 4i64) as u64;
    let full = c.g[V0][0] == 0;
    c.g[V0][0] = 0xc;
    if !full {
        c.cycles += 8;
        c.mult(0, 8, 5, 2);
        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(this));
        Addr(c.g[V1][0] as u32).off(0x8).sw(c, c.g[A2][0]);
        c.g[V0][0] = c.g[V1][0];
        Addr(c.g[V0][0] as u32).off(0x4).sw(c, c.g[A3][0]);
        c.g[V1][0] = Addr(this).off(0x34).lbu(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        Addr(this).off(0x34).sb(c, c.g[V1][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void UIEngine_RenderStateMgr(this)` (0x00398018): render the state
/// manager at `+0x18`.
pub fn ui_engine_render_state_mgr(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18)),
        callee::UI_STATE_MGR_RENDER_STUB,
        0x0039_8028,
    )
}

// ---------------------------------------------------------------------------
// UITextureSet_Find / UIIcon_SetTexture.

/// `Texture* UITextureSet_Find(UITextureSet* set, int id)` (0x398380): linear
/// search of the `count` (`+0x10`) 0x20-byte entries at `*(set + 0xc)` for the
/// one whose id (`+0x1c`) matches; null when there is none.
pub fn ui_texture_set_find(c: &mut Cpu) {
    c.cycles += 3;
    let self_ = c.g[A0][0];
    c.g[A2][0] = Addr((self_ as u32).wrapping_add(0x10)).lw(c);
    let count = c.g[A2][0];
    c.g[V1][0] = 0;
    if (count as i64) > 0 {
        c.cycles += 1;
        c.g[A0][0] = Addr((self_ as u32).wrapping_add(0xc)).lw(c);
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1c)).lw(c);
            let mismatch = c.g[V0][0] != c.g[A1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            if mismatch {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_8390);
                    }
                    continue;
                }
                break;
            }
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `void UIIcon_SetTexture(this, int id)` (0x399730): look the texture up in
/// the icon's texture set (via `+0x5c` -> `+0xd0` -> `+0x10`) and store it at
/// `+0x7c`.
pub fn ui_icon_set_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[V0][0] = Addr((this as u32).wrapping_add(0x5c)).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xd0).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x58));
        callee::UI_TEXTURE_SET_FIND.call(c, 0x0039_9754)?;
        let found = c.g[V0][0];
        c.sw((this as u32).wrapping_add(0x7c), found);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI objects.

/// `int UIFontSet_FindFont(bank, key)` (0x3983f0): linear search of the
/// `u8`-counted array of 0xc-byte font entries at `bank+4`; returns the
/// entry's `+8` value, or 0.
pub fn ui_font_set_find_font(c: &mut Cpu) {
    c.cycles += 3;
    let bank = c.g[A0][0] as u32;
    c.g[V1][0] = Addr(bank).off(0x34).lb(c);
    c.g[A2][0] = c.g[V1][0];
    if (c.g[V1][0] as i64) <= 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        crate::abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[A0][0] = sx(bank.wrapping_add(4));
    c.g[V1][0] = 0x0100_0000;
    c.g[A3][0] = 0x0100_0000;
    loop {
        c.cycles += 3;
        let entry = c.g[A0][0] as u32;
        c.g[V0][0] = Addr(entry).lw(c);
        let found = c.g[V0][0] == c.g[A1][0];
        c.g[V0][0] = c.g[V1][0];
        if !found {
            c.cycles += 5;
            c.g[A0][0] = sx(entry.wrapping_add(0xc));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
            if c.g[V0][0] != 0 {
                if event_due(c) {
                    deliver_event_here(c, 0x0039_8408);
                }
                continue;
            }
            break;
        }
        c.cycles += 2;
        c.g[V0][0] = Addr(entry.wrapping_add(8)).lw(c);
        crate::abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    crate::abi::ret(c);
}

// ---------------------------------------------------------------------------
// UI and streaming.

/// `int UIListBox_GetModeEntry(UIListBox* self)` (0x399920): read the entry
/// table of the list box's state data for the current mode (2-bit index).
pub fn ui_list_box_get_mode_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_OBJECT_GET_STATE_DATA.call(c, 0x0039_9934)?;
        c.g[A0][0] = c.g[V0][0];
        let mut entry = 0u64;
        if c.g[A0][0] != 0 {
            c.g[V1][0] = Addr(p(this, 0x74)).lbu(c);
            c.cycles += 6;
            c.g[V0][0] = 0xc;
            c.g[A0][0] = Addr(r32(c, A0)).off(8).lw(c);
            c.g[V1][0] &= 0x3;
            c.mult(0, V1, V1, V0);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
            entry = Addr(r32(c, V1)).off(0xc).lw(c);
        } else {
            c.cycles += 2;
        }
        c.g[V0][0] = entry;
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIGroup_RefreshText(cUIGroup* self)` (0x398738): call every element
/// of the group's UI list to re-read its text.
pub fn ui_group_refresh_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
        call_ptr(c, 0x0039_7840, 0x0039_874c)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = lw(c, p_2(c.g[S0][0], 8));
            loop {
                c.cycles += 9;
                c.g[A0][0] = Addr(p_2(c.g[V0][0], 0x80)).lh(c);
                c.g[V1][0] = lw(c, p_2(c.g[V0][0], 0x84));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0039_8770)?;
                c.g[S0][0] = lw(c, p_2(c.g[S0][0], 4));
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039_7718, 0x0039_877c)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                    break;
                }
                c.g[V0][0] = lw(c, p_2(c.g[S0][0], 8));
                if event_due(c) {
                    deliver_event_here(c, 0x0039_8760);
                }
            }
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int cUIGroup_HandleInput(cUIGroup* self, void* arg)` (0x398798): unless the
/// group is a passive display, offer the input to every list node through a
/// small functor; true when a node claimed it.
pub fn ui_group_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = lw(c, p_2(c.g[A0][0], 0x14));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
        c.g[V0][0] &= 1;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 11;
            c.g[V0][0] = 0x0049_0000;
            Addr(p_2(c.g[SP][0], 4)).sw(c, c.g[A1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4bb8));
            c.g[S0][0] = 0x0049_0000;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4ce8));
            Addr(p_2(c.g[SP][0], 0)).sw(c, c.g[V0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x0039_79f8, 0x0039_87dc)?;
            Addr(p_2(c.g[SP][0], 0)).sw(c, c.g[S0][0]);
            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Constructors.

/// `UIEngine* UIEngine_ctor(UIEngine* self)` (0x397b08): clear the engine
/// header, construct both UI lists and clear the tail pointers.
pub fn ui_engine_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x20);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.sw((c.g[S0][0] as u32), 0u64);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
        c.g[RA][0] = sx(0x0039_7b38);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
        call_ptr(c, 0x0039_77e8, 0x0039_7b38)?;
        c.g[RA][0] = sx(0x0039_7b40);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x34));
        call_ptr(c, 0x0039_77e8, 0x0039_7b40)?;
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x58), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x60), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x64), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x68), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[S0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6c), c.g[S0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x20);
        Ok(())
    })
}

/// `void cUIGroup_Render(cUIGroup* self, int a1)` (0x3987f8): call every
/// child node's render method. The original's loop head is a resume entry
/// (an event at the back edge returns at 0x398828).
pub fn c_ui_group_v18(c: &mut Cpu) {
    let frame = Frame { size: 0x30 };
    guest(c, |c| {
        match c.pc {
            0x0039_87f8 => {
                c.cycles += 10;
                c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
                c.g[A0][0] = sx(at(c, A0, 0x74));
                c.sq(17, frame.at(c, 0x10));
                c.sq(16, frame.at(c, 0x20));
                frame.save_ra(c, 0);
                c.g[S1][0] = c.g[A1][0];
                call_ptr(c, 0x0039_7840, 0x0039_8814)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] == 0 {
                    frame.restore(c, S0, 0x20);
                    c.cycles += 4;
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 2;
            }
            0x0039_8820 => c.cycles += 2,
            0x0039_8828 => ui_group_dispatch(c)?,
            _ => {
                c.jump_out(c.pc, 0x0039_87f8, 0x0039_8864);
                return Ok(());
            }
        }
        loop {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039_7718, 0x0039_8848)?;
            if c.g[V0][0] != 0 {
                break;
            }
            c.g[V0][0] = Addr(at(c, S0, 8)).lw(c);
            if event_due(c) && back_edge_unwinds(c, 0x0039_8828) {
                return Ok(());
            }
            ui_group_dispatch(c)?;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The render dispatch of one [`c_ui_group_v18`] node (0x398828).
pub(crate) fn ui_group_dispatch(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    c.g[A1][0] = c.g[S1][0];
    c.g[A0][0] = Addr(at(c, V0, 0x90)).lh(c);
    c.g[V1][0] = Addr(at(c, V0, 0x94)).lw(c);
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx(at(c, S0, 0).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x0039_883c)?;
    c.g[S0][0] = Addr(at(c, S0, 4)).lw(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// Lists and screens.

/// `UIList_AcceptVisitor` (0x3979F8): walk the list at `self->head` (self+4)
/// backwards calling each node's visitor (`node+0x10` delta, `node+0x14`
/// function, `this = node + delta`) with `arg`; return the first non-zero
/// result, else 0.
pub fn ui_list_accept_visitor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(r32(c, A0).wrapping_add(0x4)).lw(c);
        c.g[V0][0] = Addr(r32(c, A2).wrapping_add(0x4)).lw(c);
        let at_head = c.g[V0][0] == c.g[A2][0];
        c.g[S1][0] = c.g[A1][0];
        let mut visited = false;
        if !at_head {
            c.cycles += 2;
            c.g[V1][0] = Addr(r32(c, A2).wrapping_add(0x8)).lw(c);
            loop {
                c.cycles += 8;
                c.g[A1][0] = c.g[S1][0];
                c.g[S0][0] = Addr(r32(c, A2).wrapping_add(0x4)).lw(c);
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x10)).lh(c);
                let visitor = Addr(r32(c, V1).wrapping_add(0x14)).lw(c);
                c.g[V0][0] = visitor;
                c.g[A0][0] = abi::addu(c.g[A2][0], c.g[A0][0]);
                call_ptr(c, visitor as u32, 0x0039_7a38)?;
                if c.g[V0][0] != 0 {
                    visited = true;
                    frame.restore(c, S0, 0x20);
                    break;
                }
                c.cycles += 4;
                c.g[A2][0] = c.g[S0][0];
                c.g[V0][0] = Addr(r32(c, A2).wrapping_add(0x4)).lw(c);
                if c.g[V0][0] == c.g[A2][0] {
                    break;
                }
                c.g[V1][0] = Addr(r32(c, A2).wrapping_add(0x8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0039_7a20);
                }
            }
        }
        if !visited {
            c.cycles += 2;
            c.g[V0][0] = 0;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `lb off(reg)` (sign-extended).
#[inline]
pub(crate) fn lb_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lb(c)
}

// ---------------------------------------------------------------------------
// Loops and worker threads.

/// `func_00398438` (0x398438): free every font in a font set. Walks the
/// `count` (int8 at `+0x34`) entries of the 0xc-byte table at `+0x4`; each
/// font (`+8`) is destroyed (`vtable[8]`, flag 3) and its slots cleared, then
/// the count byte is zeroed. Signature: `void free_fonts(UIFontSet* this)`.
pub fn ui_font_set_free_fonts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lb_r(c, S2, 0x34);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[S0][0] = sx(low(c, S2).wrapping_add(0x4));
            loop {
                c.cycles += 3;
                c.g[A0][0] = lw_r(c, S0, 0x8);
                if c.g[A0][0] == 0 {
                    sw_r(c, S0, 0x4, 0);
                } else {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0039_8474);
                    c.g[A1][0] = 0x3;
                    call_ptr(c, 0x0039_16c0, 0x0039_8474)?;
                    sw_r(c, S0, 0x4, 0);
                }
                c.cycles += 6;
                c.g[S1][0] = sx(low(c, S1).wrapping_add(1));
                sw_r(c, S0, 0x0, 0);
                c.g[V0][0] = lb_r(c, S2, 0x34);
                let more = (c.g[S1][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[S0][0] = sx(low(c, S0).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_8460);
                }
            }
        }
        c.cycles += 7;
        sb_r(c, S2, 0x34, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIGroup_SetFocus(self, on)` (0x398638): run the accept-visitor over
/// the group's list with a small functor, then set the group's focus flag.
pub fn cui_group_set_focus(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x50));
        c.g[V0][0] = 0x49_0000;
        c.sq(16, sp_2(c).wrapping_add(0x40));
        c.g[V0][0] = sx(low(c, V0).wrapping_add(0x4b98));
        c.sq(17, sp_2(c).wrapping_add(0x30));
        c.g[S0][0] = c.g[A1][0];
        c.sq(18, sp_2(c).wrapping_add(0x20));
        c.g[S1][0] = 0x49_0000;
        c.g[S2][0] = c.g[A0][0];
        c.sd(sp_2(c).wrapping_add(0x10), c.g[RA][0]);
        c.g[S1][0] = sx(low(c, S1).wrapping_add(0x4ce8));
        sw_r(c, SP, 0x4, c.g[S0][0]);
        c.g[A0][0] = sx(low(c, S2).wrapping_add(0x74));
        sw_r(c, SP, 0, c.g[V0][0]);
        c.g[A1][0] = c.g[SP][0];
        callee::UI_LIST_ACCEPT_VISITOR.call(c, 0x0039_8678)?;
        c.g[V1][0] = lw_r(c, S2, 0x14);
        c.g[V0][0] = 0xffff_ffff_ffff_ffef;
        c.g[S0][0] = (0u64 < c.g[S0][0]) as u64;
        sw_r(c, SP, 0, c.g[S1][0]);
        c.g[S0][0] = sx(low(c, S0) << 4);
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        c.g[V1][0] = c.g[V1][0] | c.g[S0][0];
        c.lq(17, sp_2(c).wrapping_add(0x30));
        sw_r(c, S2, 0x14, c.g[V1][0]);
        c.lq(16, sp_2(c).wrapping_add(0x40));
        c.lq(18, sp_2(c).wrapping_add(0x20));
        c.g[RA][0] = Addr(sp_2(c).wrapping_add(0x10)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `int UIEngine_LoadPackage(engine, name)` (0x00397c40): load a UI package
/// file, store its handle and load its textures.
pub fn ui_engine_load_package(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S1, 0x80);
        frame.save(c, S0, 0x90);
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = sx(0x4a0000);
        frame.save_ra(c, 0x70);
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = sx(0x4a0000u32.wrapping_add(0x4770));
        callee::SPRINTF_2.call(c, 0x0039_7c6c)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0;
        callee::FILE_LOAD.call(c, 0x0039_7c78)?;
        Addr(at_2(c, S0, 0x4)).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx(word(c, S0).wrapping_add(0x50));
        c.g[A1][0] = Addr(at_2(c, V0, 0x8)).lw(c);
        c.g[A1][0] = sx(word(c, V0).wrapping_add(word(c, A1)));
        callee::UI_ENGINE_STORE_WORD.call(c, 0x0039_7c8c)?;
        c.g[V0][0] = Addr(at_2(c, S0, 0x4)).lw(c);
        c.g[A0][0] = sx(word(c, S0).wrapping_add(0x58));
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = Addr(at_2(c, V0, 0x10)).lw(c);
        c.g[A2][0] = sx(word(c, V0).wrapping_add(word(c, A2)));
        callee::UI_ENGINE_LOAD_TEXTURES.call(c, 0x0039_7ca4)?;
        c.lq(16, frame.at(c, 0x90));
        c.g[V0][0] = 0x1;
        c.lq(17, frame.at(c, 0x80));
        c.g[RA][0] = c.ld(frame.at(c, 0x70));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI groups.

pub(crate) const UI_GROUP_UPDATE_ENTRY: u32 = 0x0039_8910;

pub(crate) const UI_GROUP_UPDATE_END: u32 = 0x0039_8994;

/// Resumed after an event in the node walk (the animation applies loop).
pub(crate) const UI_GROUP_UPDATE_NODE: u32 = 0x0039_8948;

/// `void cUIGroup_UpdateAnim(cUIGroup* self, u16 ev)` (0x398910): apply the
/// event to every live node of the group's list, then update the group's own
/// animation.
pub fn c_ui_group_update_anim(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                UI_GROUP_UPDATE_ENTRY => {
                    c.cycles += 12;
                    let frame = Frame::open(c, 0x40);
                    frame.save(c, S2, 0x10);
                    frame.save(c, S1, 0x20);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, S0, 0x30);
                    c.g[S1][0] = c.g[A1][0] & 0xffff;
                    frame.save_ra(c, 0);
                    c.g[RA][0] = sx(0x0039_8934);
                    c.g[A0][0] = sx(p(r32(c, S2), 0x74));
                    callee::UI_LIST_FIRST_3.call(c, 0x0039_8934)?;
                    c.g[S0][0] = c.g[V0][0];
                    let empty = c.g[S0][0] == 0;
                    c.g[A0][0] = c.g[S2][0];
                    if empty {
                        pc = 0x0039_8974;
                        continue;
                    }
                    pc = 0x0039_8940;
                }
                0x0039_8940 => {
                    // The generated code's always-taken branch to the check.
                    c.cycles += 2;
                    pc = 0x0039_8960;
                }
                UI_GROUP_UPDATE_NODE => {
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = Addr(p(r32(c, V0), 0x98)).lh(c);
                    c.g[V1][0] = Addr(p(r32(c, V0), 0x9c)).lw(c);
                    let target = r32(c, V1);
                    c.g[RA][0] = sx(0x0039_895c);
                    c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
                    call_ptr(c, target, 0x0039_895c)?;
                    c.g[S0][0] = Addr(p(r32(c, S0), 4)).lw(c);
                    pc = 0x0039_8960;
                }
                0x0039_8960 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0039_8968);
                    c.g[A0][0] = c.g[S0][0];
                    callee::UI_LIST_NODE_IS_SENTINEL.call(c, 0x0039_8968)?;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
                        if event_due(c) {
                            c.pc = UI_GROUP_UPDATE_NODE;
                            if c.event() {
                                return Err(crate::call::Unwound);
                            }
                        }
                        pc = UI_GROUP_UPDATE_NODE;
                    } else {
                        pc = 0x0039_8970;
                    }
                }
                0x0039_8970 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S2][0];
                    pc = 0x0039_8974;
                }
                0x0039_8974 => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x0039_897c);
                    c.g[A1][0] = c.g[S1][0];
                    callee::UI_OBJECT_UPDATE_ANIM.call(c, 0x0039_897c)?;
                    frame_restore_group(c);
                    return Ok(());
                }
                other => {
                    c.jump_out(other, UI_GROUP_UPDATE_ENTRY, UI_GROUP_UPDATE_END);
                    return Ok(());
                }
            }
        }
    })
}

/// The shared epilogue of [`c_ui_group_update_anim`].
pub(crate) fn frame_restore_group(c: &mut Cpu) {
    let frame = Frame { size: 0x40 };
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `UIObject* cUIGroup_FindById(cUIGroup* self, int id)` (0x3986b0): search
/// the group's own objects, then walk the sub-list looking for an object with
/// the id.
pub fn c_ui_group_find_by_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(16, at_3(sp0, 0x20));
        c.sq(17, at_3(sp0, 0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp0, c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        callee::UI_OBJECT_FIND_BY_ID.call(c, 0x0039_86cc)?;
        let hit = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S0][0];
        // The generated code charges the final +1 on every path except the
        // in-list hit, which leaves through the outer block.
        let mut charge_one = true;
        if !hit {
            c.cycles += 5;
            let this = c.g[S0][0];
            c.g[A0][0] = sx(at_3(this as u32, 0x74));
            callee::UI_LIST_FIRST_4.call(c, 0x0039_86dc)?;
            c.g[S0][0] = c.g[V0][0];
            let empty = c.g[S0][0] == 0;
            c.g[V0][0] = 0;
            if !empty {
                c.cycles += 2;
                c.g[V1][0] = lw_at(c, c.g[S0][0], 8);
                loop {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = lh_at(c, c.g[V1][0], 0x70);
                    c.g[V0][0] = lw_at(c, c.g[V1][0], 0x74);
                    let func = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(at_3(c.g[S0][0] as u32, c.g[A0][0] as u32));
                    call_ptr(c, func, 0x0039_8704)?;
                    if c.g[V0][0] != 0 {
                        c.lq(16, at_3(sp0, 0x20));
                        charge_one = false;
                        break;
                    }
                    c.cycles += 5;
                    c.g[S0][0] = lw_at(c, c.g[S0][0], 4);
                    c.g[A0][0] = c.g[S0][0];
                    callee::UI_LIST_NODE_SENTINEL.call(c, 0x0039_8718)?;
                    if c.g[V0][0] != 0 {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                        break;
                    }
                    c.g[V1][0] = lw_at(c, c.g[S0][0], 8);
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_86f0);
                    }
                }
            }
        }
        if charge_one {
            c.cycles += 1;
            c.lq(16, at_3(sp0, 0x20));
        }
        c.cycles += 4;
        c.lq(17, at_3(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_3(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

/// `void cUIGroup_Render(self)` (0x398868): draw every visible node of the
/// group's list at the group's offset.
pub fn cui_group_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x10);
        frame.save_fpr(c, 21, 0x38);
        c.g[S1][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(21, 12);
        frame.save(c, S0, 0x20);
        c.mov_s(20, 13);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(r32(c, S1), 0x74));
        call_ptr(c, 0x0039_7840, 0x00398894)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x14)).lw(c);
            loop {
                c.cycles += 4;
                c.g[V0][0] = sx(((r32(c, V0) as i32) >> 6) as u32);
                c.g[V0][0] = c.g[V0][0] & 1;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = Addr(p(r32(c, S0), 4)).lw(c);
                } else {
                    c.cycles += 10;
                    c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
                    c.lwc1(12, p(r32(c, S1), 0x44));
                    c.lwc1(13, p(r32(c, S1), 0x48));
                    c.g[A0][0] = Addr(p(r32(c, V1), 0x88)).lh(c);
                    c.add_s(12, 12, 21);
                    c.add_s(13, 13, 20);
                    c.g[V0][0] = Addr(p(r32(c, V1), 0x8c)).lw(c);
                    let draw = r32(c, V0);
                    c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
                    call_ptr(c, draw, 0x003988dc)?;
                    c.g[S0][0] = Addr(p(r32(c, S0), 4)).lw(c);
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039_7718, 0x003988e8)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    break;
                }
                c.g[V0][0] = Addr(p(r32(c, S0), 0x14)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0039_88a8);
                }
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 6;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void UIListBox_SetItemWideString(listbox, index, str)` (0x399d80): replace
/// item `index`'s wide string with a copy of `str`.
pub fn ui_list_box_set_item_wide_string(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        c.cycles += 13;
        c.g[A3][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[A1][0] &= 0xff;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = lw(c, at_4(c.g[A3][0], 0x5c));
        c.g[V0][0] = lw(c, at_4(c.g[A0][0], 0xd0));
        c.g[V1][0] = lw(c, at_4(c.g[V0][0], 0x10));
        c.g[A0][0] = lw(c, at_4(c.g[V1][0], 0x10));
        let t = c.g[A0][0] == 0;
        c.g[S2][0] = c.g[A2][0];
        if t {
            c.cycles += 6;
            frame.restore(c, S0, 0x30);
            frame.restore(c, S1, 0x20);
            frame.restore(c, S2, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        {
            c.cycles += 7;
            c.g[V0][0] = 0x14;
            c.g[V1][0] = sx(at_4(c.g[A3][0], 0x9c));
            c.mult(0, 2, 5, 2);
            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = lw(c, c.g[S1][0] as u32);
            let t = c.g[A0][0] == 0;
            if !t {
                c.cycles += 2;
                c.g[RA][0] = sx(0x0039_9dd8);
                call_ptr(c, 0x0031_7e98, 0x0039_9dd8)?;
            }
        }
        c.cycles += 14;
        c.g[RA][0] = sx(0x0039_9de0);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x003d_bca0, 0x0039_9de0)?;
        c.g[V0][0] = sx(at_4(c.g[V0][0], 1));
        c.g[A1][0] = sx(0x0049_0000);
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
        c.g[RA][0] = sx(0x0039_9dfc);
        c.g[A1][0] = sx(0x0049_3dc0);
        call_ptr(c, 0x0031_7e30, 0x0039_9dfc)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0039_9e0c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003d_be20, 0x0039_9e0c)?;
        Addr(c.g[S1][0] as u32).sw(c, c.g[S0][0]);
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIListBox_ctor(self)` (0x399768): construct a list box: 32 empty
/// rows, then the scroll-bar defaults.
pub fn ui_list_box_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        call_ptr(c, 0x0039_fb30, 0x0039_977c)?;
        c.g[V0][0] = Addr(p(this as u32, 0x74)).lbu(c);
        c.g[V1][0] = 0x490000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4798));
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = c.g[V0][0] | 0x4;
        c.sw(p(this as u32, 8), c.g[V1][0]);
        c.g[V0][0] = c.g[V0][0] & 0xc4;
        c.g[A0][0] = sx((this as u32).wrapping_add(0x98));
        c.sb(p(this as u32, 0x74), c.g[V0][0]);
        c.g[V0][0] = 0x1f;
        loop {
            c.cycles += 8;
            c.sw(c.g[A0][0] as u32, 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            c.sw(p(c.g[A0][0] as u32, 4), 0);
            c.sw(p(c.g[A0][0] as u32, 8), 0);
            c.sw(p(c.g[A0][0] as u32, 0xc), 0);
            c.sb(p(c.g[A0][0] as u32, 0x10), 0);
            let more = c.g[V0][0] != c.g[A1][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0039_97a8);
            }
        }
        c.cycles += 21;
        c.sb(p(this as u32, 0x318), 0);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x98));
        c.sb(p(this as u32, 0x319), 0);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x280;
        call_ptr(c, 0x0041_6210, 0x0039_97e0)?;
        c.g[V1][0] = 0x4c0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66c8));
        c.ldl(4, p(c.g[A3][0] as u32, 7));
        c.ldr(4, c.g[A3][0] as u32);
        c.ldl(5, p(c.g[A3][0] as u32, 0xf));
        c.ldr(5, p(c.g[A3][0] as u32, 8));
        c.sdl(4, p(c.g[S0][0] as u32, 0x8f));
        c.sdr(4, p(c.g[S0][0] as u32, 0x88));
        c.sdl(5, p(c.g[S0][0] as u32, 0x97));
        c.sdr(5, p(c.g[S0][0] as u32, 0x90));
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIFontSet_LoadFonts(self)` (0x398568): load each of the font set's
/// fonts and store the resulting font objects.
pub fn ui_font_set_load_fonts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S3][0] = this;
        c.g[V0][0] = Addr(p(this as u32, 0x34)).lb(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !none {
            c.cycles += 4;
            c.g[S5][0] = 0x4a0000;
            c.g[S4][0] = 0xc;
            c.g[S2][0] = sx((this as u32).wrapping_add(4));
            c.g[S0][0] = sx((c.g[S1][0] as u32) << 24);
            loop {
                c.cycles += 19;
                c.g[A0][0] = 0x84;
                c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4780));
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0039_85c0)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = Addr(p(c.g[S2][0] as u32, 4)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x0039_15e8, 0x0039_85d8)?;
                c.mult(0, 3, 16, 20);
                c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                c.sw(p(c.g[S0][0] as u32, 0xc), c.g[V0][0]);
                c.g[V0][0] = Addr(p(c.g[S3][0] as u32, 0x34)).lb(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S1][0] as u32) << 24);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_85a8);
                }
            }
        }
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
// `UIEngine_TextureSet_dtor` (0x00398078).
//
// `void UIEngine::TextureSet::~TextureSet(void* self, int flags)`: release
// every texture through the engine's virtual `textureRelease`, free the two
// arrays, and (flags & 1) delete the set itself.

pub fn ui_engine_texture_set_dtor(c: &mut Cpu) {
    guest(c, |c| {
        'b_3980e0: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = 0u64;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S2][0] = c.g[A1][0];
            if t {
                break 'b_3980e0;
            } else {
                c.cycles += 2;
                c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                'l_3980a8: loop {
                    c.cycles += 14;
                    c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10d8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[A2][0] = vslot(c, c.g[V0][0], 0x190, A2);
                    c.g[A1][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A2][0] as u32;
                    c.g[RA][0] = sx(0x003980d0);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x003980d0)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                    if t {
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x003980a8);
                        }
                        continue 'l_3980a8;
                    } else {
                        break 'b_3980e0;
                    }
                }
            }
        }
        'b_3980f8: {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                break 'b_3980f8;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003980f4);
                call_ptr(c, 0x00317e98, 0x003980f4)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                break 'b_3980f8;
            }
        }
        'b_39810c: {
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;
            c.g[V0][0] = c.g[S2][0] & 0x1;
            if t {
                break 'b_39810c;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00398108);
                call_ptr(c, 0x00317e98, 0x00398108)?;
                c.g[V0][0] = c.g[S2][0] & 0x1;
                break 'b_39810c;
            }
        }
        'b_398120: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            if t {
                break 'b_398120;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0039811c);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x00317e50, 0x0039811c)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_398120;
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());

        Ok(())
    })
}

/// `UIIcon_Load(this, data)` (0x398998): load the icon's UI object, copy the
/// clip/align flags, find the texture in the scene's texture set and store
/// the scaled size.
pub fn ui_icon_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 49;
        let this = abi::arg(c, 0);
        let data = abi::arg(c, 1);
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = abi::arg64(c, 1);
        callee::UI_OBJECT_LOAD.call(c, 0x0039_89b4)?;

        c.g[V0][0] = Addr(data).off(0x20).lw(c);
        c.g[A0][0] = sx(0xffff_fffd_u32);
        c.g[V1][0] = Addr(this).off(0x74).lw(c);
        c.g[A1][0] = sx(0xffff_fff3_u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
        c.g[A2][0] = Addr(this).off(0x5c).lw(c);
        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
        c.g[V0][0] = c.g[V0][0] & 2;
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        Addr(this).off(0x74).sw(c, c.g[V1][0]);
        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
        c.g[V0][0] = Addr(data).off(0x20).lbu(c);
        c.g[V0][0] = c.g[V0][0] & 3;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        Addr(this).off(0x74).sw(c, c.g[V1][0]);

        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0xd0).lw(c);
        c.g[A1][0] = Addr(data).off(0x24).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x58));
        callee::UI_TEXTURE_SET_FIND_2.call(c, 0x0039_8a08)?;
        Addr(this).off(0x7c).sw(c, c.g[V0][0]);

        c.lwc1(2, gp(c, -0x2548));
        c.g[V0][0] = Addr(data).off(0x2a).lhu(c);
        c.g[V1][0] = Addr(data).off(0x28).lhu(c);
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        frame.restore(c, 17, 0x20);
        c.mtc1(3, 1);
        c.cvt_s_w(1, 1);
        frame.restore_ra(c, 0x10);
        c.mul_s(0, 0, 2);
        c.mul_s(1, 1, 2);
        c.swc1(0, sp(c).wrapping_add(4));
        c.swc1(1, sp(c));
        c.ldl(2, sp(c).wrapping_add(7));
        c.ldr(2, sp(c));
        c.sdl(2, this.wrapping_add(0x87));
        c.sdr(2, this.wrapping_add(0x80));
        frame.restore(c, 16, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void UIEngine_dtor(UIEngine *self, int flags)` (0x397b70): destroy the
/// engine's child lists, texture set and state manager, then free the object.
pub fn ui_engine_dtor(c: &mut Cpu) {
    guest(c, |c| {
        'b_397ba8: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[A2][0] = c.lw((c.g[S0][0] as u32));
            let t = c.g[A2][0] == 0u64;
            c.g[S1][0] = c.g[A1][0];
            if t {
                break 'b_397ba8;
            } else {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = 0x3;
                vslot_2(c, c.g[V0][0], 0x8, V1);
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00397ba8);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00397ba8)?;
                break 'b_397ba8;
            }
        }
        'b_397bc0: {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            if c.g[A0][0] == 0u64 {
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                break 'b_397bc0;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00397bbc);
                call_ptr(c, 0x00317e98, 0x00397bbc)?;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                break 'b_397bc0;
            }
        }
        'b_397bdc: {
            c.cycles += 2;
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = 0x3;
            if t {
                break 'b_397bdc;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                vslot_2(c, c.g[V0][0], 0x8, V1);
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00397bdc);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00397bdc)?;
                break 'b_397bdc;
            }
        }
        'b_397bfc: {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = 0x3;
            if t {
                break 'b_397bfc;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                vslot_2(c, c.g[V0][0], 0x8, V1);
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00397bfc);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00397bfc)?;
                break 'b_397bfc;
            }
        }
        'b_397c2c: {
            c.cycles += 9;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
            c.g[RA][0] = sx(0x00397c08);
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x00398078, 0x00397c08)?;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
            c.g[RA][0] = sx(0x00397c14);
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x0039e9d8, 0x00397c14)?;
            c.g[V0][0] = c.g[S1][0] & 0x1;
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_397c2c;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00397c28);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00317e50, 0x00397c28)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_397c2c;
            }
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
// List box.

/// `void UIListBox_RefreshText(UIListBox* self)` (0x399e28): set the text of
/// every row from the localised string table, then relayout the box.
pub fn uilistbox_refresh_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S1), 0x5c)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0xd0)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x10)).lw(c);
        c.g[S2][0] = Addr(p(r32(c, A0), 0x10)).lw(c);
        let no_storage = c.g[S2][0] == 0;
        c.lq(S0, frame.at(c, 0x50));
        if !no_storage {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S1), 0x318)).lbu(c);
            let none = c.g[V0][0] == 0;
            c.g[S0][0] = 0;
            if !none {
                c.cycles += 3;
                c.g[S4][0] = 0x14;
                c.g[S3][0] = sx(p(r32(c, S1), 0x98));
                c.mult(0, 3, 16, 20);
                loop {
                    c.cycles += 4;
                    c.g[V0][0] = sx(p(r32(c, V1), r32(c, S3)));
                    c.g[A1][0] = Addr(r32(c, V0)).lw(c);
                    let empty = c.g[A1][0] == 0;
                    c.g[V1][0] = sx(r32(c, S0).wrapping_add(1));
                    let mut next = 0u64;
                    if !empty {
                        c.cycles += 7;
                        c.g[V1][0] = Addr(p(r32(c, S2), 4)).lw(c);
                        c.g[A0][0] = Addr(p(r32(c, V1), 0x20)).lh(c);
                        c.g[V0][0] = Addr(p(r32(c, V1), 0x24)).lw(c);
                        let get = r32(c, V0);
                        c.g[A0][0] = sx(p(r32(c, S2), r32(c, A0)));
                        call_ptr(c, get, 0x0039_9e9c)?;
                        let is_null = c.g[V0][0] == 0;
                        c.g[A2][0] = c.g[V0][0];
                        if is_null {
                            c.cycles += 1;
                            c.g[V1][0] = sx(r32(c, S0).wrapping_add(1));
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = c.g[S0][0];
                            call_ptr(c, 0x0039_9d80, 0x0039_9eb0)?;
                            c.cycles += 1;
                            c.g[V1][0] = sx(r32(c, S0).wrapping_add(1));
                        }
                    } else {
                        c.cycles += 1;
                        c.g[V1][0] = sx(r32(c, S0).wrapping_add(1));
                    }
                    next = c.g[V1][0];
                    let _ = next;
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p(r32(c, S1), 0x318)).lbu(c);
                    c.g[S0][0] = c.g[V1][0] & 0xff;
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                    let more = c.g[V0][0] != 0;
                    c.mult(0, 3, 16, 20);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_9e78);
                    }
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = Addr(p(r32(c, S1), 8)).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = Addr(p(r32(c, V0), 0x28)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x2c)).lw(c);
        let relayout = r32(c, V1);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, relayout, 0x0039_9ee0)?;
        c.lq(S0, frame.at(c, 0x50));
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void func_00399820(UIListBox* self, sUIData* data)` (0x399820): load the
/// list box's shared UI-object data, take its horizontal alignment and
/// visibility from `data`, and when the "wrap" flag is set compute the four
/// texel-size floats from the byte channels at `data+0x24`.
pub fn uilistbox_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        let data = c.g[A1][0];
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = data;
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x0039_983c);
        c.g[S1][0] = c.g[A0][0];
        c.pc = 0x0039_fe00;
        ee::s13::UIObject_Load(c);
        if c.pc != 0x0039_983c {
            return Err(Unwound);
        }
        c.g[V1][0] = c.lbu(p(c.g[S0][0] as u32, 0x20));
        c.g[A0][0] = 0xffff_ffff_ffff_fffc;
        c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x74));
        c.g[V1][0] = c.g[V1][0] & 0x3;
        c.g[A2][0] = c.lw(p(c.g[S1][0] as u32, 8)); // vtable
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.sw(p(c.g[S1][0] as u32, 0x74), c.g[V0][0]);
        c.g[A0][0] = c.lh(p(c.g[A2][0] as u32, 0xb8));
        c.g[A1][0] = c.lhu(p(c.g[S0][0] as u32, 0x22));
        c.g[V1][0] = c.lw(p(c.g[A2][0] as u32, 0xbc));
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let func = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039_9874);
        c.g[A1][0] = c.g[A1][0] & 0x1;
        call_ptr(c, func, 0x0039_9874)?;
        c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x74));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        c.g[V0][0] = c.g[V0][0] & 0x1;
        if c.g[V0][0] != 0 {
            c.cycles += 30;
            c.g[V0][0] = c.lbu(p(c.g[S0][0] as u32, 0x27));
            c.g[V1][0] = c.lbu(p(c.g[S0][0] as u32, 0x24));
            c.g[A0][0] = c.lbu(p(c.g[S0][0] as u32, 0x25));
            c.mtc1(2, 4);
            c.cvt_s_w(4, 4);
            c.g[A1][0] = c.lbu(p(c.g[S0][0] as u32, 0x26));
            c.mtc1(3, 2);
            c.cvt_s_w(2, 2);
            c.mtc1(4, 3);
            c.cvt_s_w(3, 3);
            c.lwc1(0, gp_2(c, -0x2544)); // texel scale
            c.mtc1(5, 1);
            c.cvt_s_w(1, 1);
            c.mul_s(4, 4, 0);
            c.mul_s(2, 2, 0);
            c.mul_s(3, 3, 0);
            c.mul_s(1, 1, 0);
            c.swc1(4, p(c.g[SP][0] as u32, 0xc));
            c.swc1(2, c.g[SP][0] as u32);
            c.swc1(3, p(c.g[SP][0] as u32, 4));
            c.swc1(1, p(c.g[SP][0] as u32, 8));
            c.ldl(2, p(c.g[SP][0] as u32, 7));
            c.ldr(2, c.g[SP][0] as u32);
            c.ldl(3, p(c.g[SP][0] as u32, 0xf));
            c.ldr(3, p(c.g[SP][0] as u32, 8));
            c.sdl(2, p(c.g[S1][0] as u32, 0x7f));
            c.sdr(2, p(c.g[S1][0] as u32, 0x78));
            c.sdl(3, p(c.g[S1][0] as u32, 0x87));
            c.sdr(3, p(c.g[S1][0] as u32, 0x80));
            c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x14));
        } else {
            c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x14));
        }
        c.cycles += 7;
        c.lq(16, p(c.g[SP][0] as u32, 0x30));
        c.g[V0][0] = c.g[V0][0] | 0x80;
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x10));
        c.sw(p(c.g[S1][0] as u32, 0x14), c.g[V0][0]);
        c.lq(17, p(c.g[SP][0] as u32, 0x20));
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
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as RET;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::computer::tests::wb;
    pub(crate) use crate::game::ai::rider::tests::clear_2;
    pub(crate) use crate::game::ai::rider::tests::hh;
    pub(crate) use crate::game::ai::rider::tests::slot_2 as slot;
    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4 as SEEDS_2;
    pub(crate) use crate::game::be::bedbprofile::tests::strict as strict_2;
    pub(crate) use crate::game::be::beintplayer::tests::base_10 as base_7;
    pub(crate) use crate::game::be::beintplayer::tests::globals_5 as globals_2;
    pub(crate) use crate::game::be::beintplayer::tests::strict as strict_3;
    pub(crate) use crate::game::be::beintplayer::tests::w as w_9;
    pub(crate) use crate::game::be::beintplayer::tests::SEEDS_7 as SEEDS_9;
    pub(crate) use crate::game::camera::cameracontroller::tests::put_vtable;
    pub(crate) use crate::game::dirtysock::tags::tests::put_str;
    pub(crate) use crate::game::dirtysock::tags2::tests::bb;
    pub(crate) use crate::game::dirtysock::tags2::tests::string;
    pub(crate) use crate::game::fe::feasyncfile::tests::b;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_10 as pool_allocator_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2 as RIDERS;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_3;
    pub(crate) use crate::game::fe::fereal::tests::base as base_2;
    pub(crate) use crate::game::main::gamemode::tests::w as w_4;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3;
    pub(crate) use crate::game::movie::movieplayer::tests::heap_init_2 as heap_init;
    pub(crate) use crate::game::movie::movieplayer::tests::pool_allocator as pool_allocator_2;
    pub(crate) use crate::game::movie::movieplayer::tests::run_2 as run;
    pub(crate) use crate::game::movie::movieplayer::tests::w as w_5;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2 as BE;
    pub(crate) use crate::game::movie::movieplayer::tests::MEM_HEAP;
    pub(crate) use crate::game::movie::movieplayer::tests::W;
    pub(crate) use crate::game::object::debouncenode::tests::vtable_2;
    pub(crate) use crate::game::render::lightman::tests::eafile_pools;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w as w_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_3;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_6;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7 as globals;
    pub(crate) use crate::game::sound::soundsys::tests::load_2;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool;
    pub(crate) use crate::game::sound::soundsys::tests::w as w_8;
    pub(crate) use crate::game::sound::soundsys::tests::BE as BE_5;
    pub(crate) use crate::game::sound::soundsys::tests::WORLD as WORLD_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn uifont_set_add_font_matches() {
        check_lohi(
            ssx3_ee::s13::f_003984b0,
            uifont_set_add_font,
            200,
            &|c, seed| {
                let mut r = Rng(0x3984_b000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
                c.g[A3][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
                c.bus.write8(
                    DATA + 0x34,
                    if seed % 4 == 0 { 4 } else { (seed % 4) as u8 },
                );
            },
        );
    }

    #[test]
    fn ui_engine_render_state_mgr_matches() {
        check(
            ee::s13::f_00398018__generated,
            ui_engine_render_state_mgr,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39_8018, 0);
                c.g[A0][0] = ptr(&mut r) as u64;
            },
        );
    }

    // -----------------------------------------------------------------------
    // UITextureSet_Find / UIIcon_SetTexture.

    /// A texture set at `set`: `count` entries of 0x20 bytes with ids at
    /// `+0x1c`; `seed % 4 == 0` asks for an id that is not there.
    pub(crate) fn texture_set(c: &mut Cpu, r: &mut Rng, set: u32, count: u32) -> u32 {
        let entries = DATA + 0x4000;
        c.bus.write32(set + 0x10, count);
        c.bus.write32(set + 0xc, entries);
        for i in 0..8u32 {
            c.bus.write32(entries + i * 0x20, r.next_u32());
            c.bus.write32(entries + i * 0x20 + 0x1c, 0x1000 + i);
        }
        let n = r.next_u32() % (count + 1);
        if n < count {
            0x1000 + n
        } else {
            0x9000
        }
    }

    pub(crate) fn find_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3983_80, 0);
        c.g[A0][0] = DATA as u64;
        let count = 1 + r.next_u32() % 6;
        let id = texture_set(c, &mut r, DATA, count);
        c.g[A1][0] = id as u64;
    }

    driven!(gen_texture_find, ssx3_ee::s13::f_00398380__generated);

    driven!(hand_texture_find, ui_texture_set_find);

    #[test]
    fn ui_texture_set_find_matches() {
        check(
            ssx3_ee::s13::f_00398380__generated,
            ui_texture_set_find,
            64,
            &find_setup,
        );
        check_events(gen_texture_find, hand_texture_find, 64, &|c, seed| {
            find_setup(c, seed);
            let mut r = Rng(0x3983_80 ^ seed as u64);
            let mode = schedule(c, &mut r, seed, 0x0039_8380);
            // A short search may not reach the scheduled event: make the
            // first back edge due, identically for both bodies.
            c.next_event = c.cycles + 4;
            mode
        });
    }

    #[test]
    fn ui_icon_set_texture_matches() {
        check(
            ssx3_ee::s13::f_00399730__generated,
            ui_icon_set_texture,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3997_30, 0);
                let this = DATA;
                let icon = DATA + 0x800;
                let scene = DATA + 0x1000;
                let set = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.bus.write32(this + 0x5c, icon);
                c.bus.write32(icon + 0xd0, scene);
                // The texture set lives at `*(scene + 0x10) + 0x58`.
                c.bus.write32(scene + 0x10, set);
                c.bus.write32(this + 0x7c, r.next_u32());
                let count = 1 + r.next_u32() % 4;
                let id = texture_set(c, &mut r, set + 0x58, count);
                c.g[A1][0] = id as u64;
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x3983f0 UIFontSet_FindFont.

    #[test]
    fn ui_font_set_find_font_matches() {
        check(
            ee::s13::f_003983f0__generated,
            ui_font_set_find_font,
            24,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3983_f0, 0);
                let bank = DATA + 0x100;
                c.g[A0][0] = sx(bank);
                let count = (seed % 5) as u8;
                c.bus.write8(bank + 0x34, count);
                let key = r.next_u32();
                let hit = (seed % 3) as u32;
                for i in 0..4u32 {
                    let entry = bank + 4 + 0xc * i;
                    c.bus
                        .write32(entry, if i == hit { key } else { r.next_u32() });
                    c.bus.write32(entry + 8, r.next_u32());
                }
                c.g[A1][0] = key as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // UI and streaming.

    #[test]
    fn ui_list_box_get_mode_entry_matches() {
        check(
            ee::s13::f_00399920__generated,
            ui_list_box_get_mode_entry,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3999_20, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let p1 = DATA + 0x800;
                let p2 = DATA + 0x820;
                let p3 = DATA + 0x840;
                let table = DATA + 0x900;
                clear(c, this, 0x100);
                clear(c, p1, 0x200);
                clear(c, table, 0x80);
                c.g[A0][0] = sx(this);
                c.bus.write8(this + 0x74, (seed % 4) as u8);
                if seed % 2 == 0 {
                    c.bus.write32(this + 0x5c, p1);
                    c.bus.write32(p1 + 0xd0, p2);
                    c.bus.write32(p2 + 0x10, p3);
                    c.bus.write32(p3 + 8, table);
                    for i in 0..4u32 {
                        c.bus.write32(table + i * 0xc + 0xc, r.next_u32());
                    }
                }
            },
        );
    }

    #[test]
    fn ui_group_refresh_text_matches() {
        check(
            ee::s13::f_00398738__generated,
            ui_group_refresh_text,
            SEEDS_2,
            &|c, seed| {
                let _ = base(c, seed, 0x3987_38);
                let s = DATA + 0x100;
                clear(c, s, 0x40);
                let list = s + 0x74;
                let sent = DATA + 0x500;
                w(c, sent, sent as u32);
                w(c, sent + 4, sent as u32);
                if seed % 3 == 0 {
                    w(c, list + 4, sent as u32);
                } else {
                    let n = DATA + 0x540;
                    clear(c, n, 0x20);
                    w(c, n, list as u32);
                    w(c, n + 4, sent as u32);
                    let vt = DATA + 0x580;
                    vtable_2(c, vt, &[(0x80, FAKE + 0x20)]);
                    w(c, n + 8, vt as u32);
                    w(c, list + 4, n as u32);
                }
                c.g[A0][0] = sx(s);
            },
        );
    }

    #[test]
    fn ui_group_handle_input_matches() {
        check(
            ee::s13::f_00398798__generated,
            ui_group_handle_input,
            SEEDS_2,
            &|c, seed| {
                let mut r = base(c, seed, 0x3987_98);
                let s = DATA + 0x100;
                clear(c, s, 0x40);
                w(c, s + 0x14, if seed % 3 == 0 { 0x20 } else { 0 });
                let list = s + 0x74;
                let sent = DATA + 0x500;
                w(c, sent, sent as u32);
                w(c, sent + 4, sent as u32);
                if seed % 2 == 0 {
                    w(c, list + 4, sent as u32);
                } else {
                    let n = DATA + 0x540;
                    clear(c, n, 0x20);
                    w(c, n, list as u32);
                    w(c, n + 4, sent as u32);
                    let vt = DATA + 0x580;
                    vtable_2(c, vt, &[(0x10, FAKE + 0x30)]);
                    w(c, n + 8, vt as u32);
                    w(c, list + 4, n as u32);
                }
                c.g[A0][0] = sx(s);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn ui_engine_ctor_matches() {
        check(
            ee::s13::f_00397b08__generated,
            ui_engine_ctor,
            30,
            &|c, seed| {
                base_2(c, seed, 0x39_7b08);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn c_ui_group_v18_matches() {
        check(
            ee::s13::f_003987f8__generated,
            c_ui_group_v18,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3987_f8, 0);
                let n0 = DATA + 0x1000;
                let n1 = DATA + 0x1200;
                let obj = DATA + 0x1300;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = r.next_u32() as u64;
                c.bus.write32(DATA + 0x78, n0);
                if seed % 2 == 0 {
                    c.bus.write32(n0, n0);
                    c.bus.write32(n0 + 4, n0);
                } else {
                    c.bus.write32(n0, 0);
                    c.bus.write32(n0 + 4, n1);
                    c.bus.write32(n0 + 8, obj);
                    c.bus.write32(n1, n1);
                    c.bus.write32(n1 + 4, n1);
                    put_vtable(c, obj, &[0x90]);
                }
                c.pc = 0x0039_87f8;
            },
        );
    }

    /// A list of `n` nodes at `base`, the last pointing at itself; every node
    /// points at the visitor table `vt` and holds an empty string at +0x20.
    pub(crate) fn list(c: &mut Cpu, base: u32, n: u32, head_field: u32, vt: u32) {
        w_2(c, head_field, base);
        for i in 0..n {
            let node = base + 0x100 * i;
            let next = if i + 1 == n { node } else { node + 0x100 };
            w_2(c, node + 4, next);
            w_2(c, node + 8, vt);
            string(c, node + 0x20, "");
        }
    }

    // ------------------------------------------------------------ list walks

    #[test]
    fn ui_list_accept_visitor_matches() {
        check(
            ee::s13::f_003979f8__generated,
            ui_list_accept_visitor,
            SEEDS_3,
            &|c, seed| {
                machine(c, seed, 0x39_79f8, 4);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x500) as u64;
                string(c, DATA + 0x500, "");
                list(c, DATA + 0x100, 3, DATA + 4, DATA + 0x600);
                let real = seed % 2 == 0;
                slot(
                    c,
                    DATA + 0x600,
                    0x10,
                    if real { 0x20 } else { 0 },
                    if real { 0x0041_aa88 } else { FAKE },
                );
                c.pc = 0x0039_79f8;
            },
        );
    }

    #[test]
    fn ui_font_set_free_fonts_matches() {
        check(
            ee::s13::f_00398438__generated,
            ui_font_set_free_fonts,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x398438, 4);
                be_library(c);
                world(c);
                let set = DATA;
                c.g[A0][0] = set as u64;
                let n = (seed % 4) as u8;
                b(c, set + 0x34, n);
                // Font objects are left null: the non-null path destroys them
                // through the fixed font destructor (0x3916c0), which frees the
                // pointer and would need a real heap allocation in the fixture.
                for i in 0..n as u32 {
                    let s = set + 4 + i * 0xc;
                    w_3(c, s, (seed + i) as u32);
                    w_3(c, s + 8, 0);
                }
            },
        );
    }

    #[test]
    fn cui_group_set_focus_matches() {
        check(
            ee::s13::f_00398638__generated,
            cui_group_set_focus,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x398638, 1);
                be_library(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                if seed % 2 == 0 {
                    w_4(c, obj + 0x74 + 4, 0);
                } else {
                    let s = DATA + 0x2000;
                    let a = DATA + 0x2100;
                    w_4(c, obj + 0x74 + 4, a);
                    w_4(c, a + 4, s);
                    w_4(c, a, 0);
                    w_4(c, s + 4, s);
                    w_4(c, s, 0);
                }
                w_4(c, obj + 0x14, r.next_u32());
            },
        );
    }

    #[test]
    fn ui_engine_load_package_matches() {
        check(
            ee::s13::f_00397c40__generated,
            ui_engine_load_package,
            8,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x397c_40, 8);
                eafile_pools(c);
                let engine = DATA;
                let name = DATA + 0x1000;
                put_str(c, name, b"package");
                c.g[A0][0] = engine as u64;
                c.g[A1][0] = name as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // UI groups.

    #[test]
    fn c_ui_group_update_anim_matches() {
        check(
            ssx3_ee::s13::f_00398910__generated,
            c_ui_group_update_anim,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x398910, 4);
                be_library(c);
                world(c);
                let group = DATA + 0x100;
                c.g[A0][0] = group as u64;
                c.g[A1][0] = (r.next_u32() & 0xffff) as u64;
                c.pc = 0x0039_8910;
                let list = group + 0x74;
                if seed % 4 == 0 {
                    // An empty list: the sentinel links back to itself.
                    let sent = DATA + 0x300;
                    w_6(c, list + 4, sent);
                    w_6(c, sent, sent);
                    w_6(c, sent + 4, sent);
                } else {
                    let n1 = DATA + 0x300;
                    let sent = DATA + 0x340;
                    let vt = DATA + 0x500;
                    w_6(c, list + 4, n1);
                    w_6(c, n1, r.next_u32());
                    w_6(c, n1 + 4, sent);
                    w_6(c, n1 + 8, vt);
                    slot_2(c, vt, 0x98, 0, FAKE);
                    slot_2(c, vt, 0x9c, 0, FAKE + 0x10);
                    w_6(c, sent, sent);
                    w_6(c, sent + 4, sent);
                }
            },
        );
    }

    #[test]
    fn c_ui_group_find_by_id_matches() {
        check(
            ee::s13::f_003986b0__generated,
            c_ui_group_find_by_id,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3986b0, 2);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 5) as u64;
                // The group's sub-list at_3 `+0x74` is empty.
                c.bus.write32(DATA + 0x74, 0);
                let node = DATA + 0x1000;
                let vt = vtable(c, &mut r, &[0x70]);
                c.bus.write32(node + 8, vt);
                c.bus.write32(node + 4, DATA + 0x1100);
                c.bus
                    .write32(DATA + 0x74, if seed % 3 == 0 { 0 } else { DATA + 0x1200 });
            },
        );
    }

    /// A real heap block of `size` bytes.
    pub(crate) fn alloc(c: &mut Cpu, size: u32) -> u32 {
        run_2(
            c,
            0x0031_7d70,
            ee::s10::f_00317d70,
            &[size as u64, 0x0048_0000, 0x2000_0000, 0],
        ) as u32
    }

    // ----------------------------------------------------- list box wide str

    #[test]
    fn ui_list_box_set_item_wide_string_matches() {
        check(
            ee::s13::f_00399d80__generated,
            ui_list_box_set_item_wide_string,
            SEEDS_7,
            &|c, seed| {
                let _ = base_4(c, seed, 0x399d_80, 6);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (obj + 0x1000) as u64;
                // The list-box chain that gates the update.
                let a = obj + 0x2000;
                let b = a + 0x100;
                let d = b + 0x100;
                w_7(c, obj + 0x5c, a);
                w_7(c, a + 0xd0, b);
                w_7(c, b + 0x10, d);
                w_7(c, d + 0x10, if seed % 3 == 0 { 0 } else { obj + 0x2300 });
                // The item's old string (a real block when present, so the
                // replacement's free of it is valid).
                let old = alloc(c, 8);
                w_7(
                    c,
                    obj + 0x9c + 0x14 * (seed % 4),
                    if seed % 2 == 0 { 0 } else { old },
                );
                // The wide source string.
                wh(c, obj + 0x1000, b'H' as u16);
                wh(c, obj + 0x1002, b'i' as u16);
                wh(c, obj + 0x1004, 0);
            },
        );
    }

    /// First size-class pool region (8-byte blocks): freeds must land here.
    pub(crate) const POOL0: u32 = DATA + 0x40_0000;

    #[test]
    fn ui_engine_texture_set_dtor_matches() {
        strict(
            ee::s13::f_00398078__generated,
            ui_engine_texture_set_dtor,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3980, 12);
                let pool = POOL0;
                let (this, array, extra) = (pool + 0x300, pool + 0x400, pool + 0x500);
                c.bus.write32(GAME_GP + 0x2a70, pool);
                c.bus.write32(pool + 0x800, 0);
                c.bus.write32(GAME_GP + 0xdd8, 1);
                let obj = DATA + 0x2000;
                let vt = DATA + 0x2100;
                c.bus.write16(vt + 0x190, 0);
                c.bus.write32(vt + 0x194, FAKE);
                c.bus.write32(GAME_GP - 0x854, obj);
                c.bus.write32(obj + 0x10d8, vt);
                c.bus.write32(this + 8, (seed % 4) as u32);
                c.bus.write32(this + 4, array);
                c.bus
                    .write32(this + 0xc, if seed % 5 == 0 { 0 } else { extra });
                for i in 0..8u32 {
                    c.bus.write32(array + 4 * i, DATA + 0x800 + 0x40 * i);
                    c.bus.write32(extra + 4 * i, DATA + 0x900 + 0x40 * i);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn ui_icon_load_matches() {
        check(
            ee::s13::f_00398998__generated,
            ui_icon_load,
            SEEDS_8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3989, 3);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                let data = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = data as u64;
                c.bus.write32(this + 0x74, r.next_u32());
                c.bus.write32(this + 0x5c, DATA + 0x2400);
                c.bus.write32(data + 0x20, r.next_u32());
                c.bus.write32(data + 0x24, seed % 4);
                c.bus.write16(data + 0x28, seed as u16);
                c.bus.write16(data + 0x2a, (seed * 3) as u16);
                c.bus.write32(GAME_GP - 0x2548, 0.01f32.to_bits());
                c.bus.write32(DATA + 0x2400 + 0xd0, DATA + 0x2800);
                c.bus.write32(DATA + 0x2800 + 0x10, DATA + 0x2c00);
            },
        );
    }

    #[test]
    fn ui_engine_dtor_matches() {
        strict_2(
            ee::s13::f_00397b70__generated,
            ui_engine_dtor,
            60,
            &|c, seed| {
                let _r = base_5(c, seed, 0x397b);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.bus.write32(DATA, 0);
                c.bus.write32(DATA + 4, 0);
                c.bus
                    .write32(DATA + 0xc, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
                c.bus
                    .write32(DATA + 0x14, if seed % 4 == 0 { 0 } else { DATA + 0x2200 });
                c.bus.write32(DATA + 0x2008, DATA + 0x2100);
                c.bus.write16(DATA + 0x2100 + 8, 0);
                c.bus.write32(DATA + 0x2100 + 0xc, FAKE);
                c.bus.write32(DATA + 0x2200, DATA + 0x2300);
                c.bus.write16(DATA + 0x2300 + 8, 0);
                c.bus.write32(DATA + 0x2300 + 0xc, FAKE + 0x10);
            },
        );
    }

    #[test]
    fn uilistbox_refresh_text_matches() {
        check(
            ee::s13::f_00399e28__generated,
            uilistbox_refresh_text,
            12,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x399e28, 3);
                let this = DATA;
                clear_2(c, this, 0x800);
                c.g[A0][0] = this as u64;
                let a = DATA + 0x200;
                let b = DATA + 0x300;
                let loc = DATA + 0x400;
                clear_2(c, a, 0x100);
                clear_2(c, b, 0x100);
                clear_2(c, loc, 0x100);
                w_8(c, this + 0x5c, a);
                w_8(c, a + 0xd0, b);
                w_8(c, b + 0x10, loc);
                w_8(c, loc + 0x10, DATA + 0x500);
                let get = vtable(c, &mut r, &[0x20]);
                w_8(c, DATA + 0x500 + 4, get);
                let count = if seed % 3 == 0 { 0 } else { 2 };
                w_8(c, this + 0x318, count);
                for i in 0..3u32 {
                    w_8(
                        c,
                        this + 0x98 + 0x14 * i,
                        if i < count { 0x100 + i * 0x10 } else { 0 },
                    );
                }
                let relayout = vtable(c, &mut r, &[0x28]);
                w_8(c, this + 8, relayout);
            },
        );
    }

    /// A UI element whose vtable slots are fake calls, plus its data block.
    pub(crate) fn ui_fixture(c: &mut Cpu, seed: u32, salt: u64, slots: &[u32]) -> (u32, u32) {
        let mut r = base_7(c, seed, salt, 0);
        let this = DATA + 0x1000;
        let data = DATA + 0x2000;
        let vt = crate::difftest::fakes::vtable(c, &mut r, slots);
        w_9(c, this + 8, vt);
        w_9(c, this + 0x14, r.next_u32());
        // bit 2 of +0x74 survives the load and selects the texel block
        w_9(
            c,
            this + 0x74,
            if seed % 2 == 0 {
                r.next_u32() & !4
            } else {
                r.next_u32() | 4
            },
        );
        w_9(c, this + 0x78, r.next_u32());
        w_9(c, this + 0x80, r.next_u32());
        for k in 0..0x30u32 {
            c.bus.write8(data + 0x20 + k, r.next_u32() as u8);
        }
        w_9(c, data + 4, r.next_u32());
        w_9(c, data + 8, r.next_u32());
        w_9(c, data + 0xc, r.next_u32());
        w_9(c, data + 0x24, r.next_u32());
        w_9(c, GAME_GP.wrapping_sub(0x2544), r.f32(4.0));
        w_9(c, GAME_GP.wrapping_sub(0x24f8), r.f32(4.0));
        (this, data)
    }

    #[test]
    fn uilistbox_load_matches() {
        strict_3(
            ee::s13::f_00399820__generated,
            uilistbox_load,
            SEEDS_9,
            &|c, seed| {
                let (this, data) = ui_fixture(c, seed, 0x9820, &[0x48, 0x4c, 0xb8, 0xbc]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(data);
            },
        );
    }
}
