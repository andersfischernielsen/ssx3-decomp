//! `src/object/flexrailnode.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::p as p_3;
pub(crate) use crate::game::fe::feasyncfile::p as p_4;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_2;
pub(crate) use crate::game::movie::movieplayer::at;
pub(crate) use crate::game::movie::movieplayer::gp;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::object::deadfadenode::ctor_with_vtable;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::visualfx::worldlightman::deleting_dtor;
pub(crate) use crate::game::visualfx::worldlightman::DELETE;
pub(crate) use crate::game::world::worldview::p_2 as p;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::float;
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
    (0x0034_90b0, float_rail_v01, "FloatRail_v01"),
    (0x0034_9110, float_rail_v50, "FloatRail_v50"),
    (
        0x0034_9798,
        float_rail_apply_impulse,
        "FloatRail_ApplyImpulse",
    ),
    (
        0x0034_9840,
        float_rail_get_contact_velocity,
        "FloatRail_GetContactVelocity",
    ),
    (0x0034_9ad0, float_rail_save, "FloatRail_Save"),
    (0x0034_9eb8, spring_rail_v01, "SpringRail_v01"),
    (0x0034_9f18, spring_rail_v50, "SpringRail_v50"),
    (
        0x0034_a568,
        spring_rail_apply_impulse,
        "SpringRail_ApplyImpulse",
    ),
    (0x0034_ac10, spring_rail_save, "SpringRail_Save"),
    (0x0034_ac88, flag_read_params, "Flag_ReadParams"),
    (0x0034_add8, flag_ctor, "Flag_ctor"),
    (0x0034_ae68, flag_ctor_2, "Flag_ctor_2"),
    (
        0x0034_af20,
        c_flag_manager_group_init_key,
        "cFlagManager_GroupInitKey",
    ),
    (
        0x0034_af38,
        c_flag_manager_group_ctor,
        "cFlagManager_GroupCtor",
    ),
    (
        0x0034_afb8,
        c_flag_manager_group_dtor,
        "cFlagManager_GroupDtor",
    ),
    (
        0x0034_afe8,
        c_flag_manager_group_matches,
        "cFlagManager_GroupMatches",
    ),
];

// ---------------------------------------------------------------------------
// Callees (addresses the generated body reaches through the function table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INSTANCE as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INTERFACE as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUY_POPUP_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::CLEAR_NEW_REWARDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::DYNAMIC_EMITTER_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STRLEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_HASH;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_COPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TEX_MAN_INIT_TABLE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_CTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM as UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2 as UI_ANIM_SET_CREATE_ANIM_2_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST as UI_LIST_FIRST_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM as UI_OBJECT_SET_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT as UI_SCREEN_FIND_OBJECT_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2 as UI_STATE_CTOR_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_RENDER_FADE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_MUSIC_TIER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_TRACK_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_TRICK_METER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUTOSAVE_UPDATE_ICON;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2 as BE_GET_BE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_2 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_CLEAR_EMITTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_INIT_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_COUNT_2 as BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_ENTRY_2 as BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_SET_OWNER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_STACK_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBE_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_FILL_PLAYER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_GET_CHARACTER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::EDITOR_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_LOCALE_LOAD_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_CREATED_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_ERROR_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_SHOW_ROOM_TAB;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_UPDATE_PLAYERS_IN_ROOM;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_ANIM_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_MANAGER_ADD_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_READ_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_BUILD_MESH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_SPRINGS_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_UPDATE_VERTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_RAIL_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_ANIM_MAP_SET_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_FINISH_CLEAR_FLAG40;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_FLAG_MANAGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_NEXT_SECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_READ_QUOTED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_HIDE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SHOW;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MENU_HIDE_ROOM_TABS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_SELECT_ALGORITHM;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCARD_INSTALL_CALLBACKS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_EMPTY_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_IS_FLAG_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::MOD_OBJ_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_9 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_2 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_9 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_UPDATE_3 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_MESSAGE_CENTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PLAYBACK_RESTORE_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_MEM_CARD_CLEAR_DIR_CACHE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_PAD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_SET_STATE_2 as REPLAY_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SET_INPUT_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_3 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_DATA_GET_BANK_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3 as SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2 as SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE_2 as SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2 as SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_2 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CAM_BUILD_MATRIX_FROM_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_LOAD_BANK_FROM_MEMORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_FREE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_AABB_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COMPUTE_OFFSET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_MODIFIER_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_5 as SPRINTF_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2 as UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST as UI_LIST_FIRST_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_4 as UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3 as UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_BLOCK_POOL_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_STREAMER_UPDATE_VIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CHUNK_RESOLVE_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_MEMMAN_CREATE_SECTION_ALLOCATOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_DTOR_2 as WORLD_OBJ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_RUN_KILL_SCRIPT_2 as WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_NODE_DTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_1;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BEREWARD_INT_IS_PEAK_COMPLETE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_SCORE_INT_UPDATE_BEST_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_IS_CONNECTED;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_RECONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;
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
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PDA_TEMPLATE_OPEN;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PROFILE_SAVE_PDA_INIT_SLOT_LABELS;
    pub(crate) use crate::game::movie::movieplayer::callee::PARENT_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_ON_SPLIT_MARKER;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_RESET_STATS;
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
    pub(crate) use crate::game::movie::movieplayer::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::movie::movieplayer::callee::STRCPY_2;
    pub(crate) use crate::game::movie::movieplayer::callee::STREAM_BUFFER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_LOAD_TEXTURES;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_STORE_WORD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_MGR_ADD_BACKGROUND_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESSES;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESS_COUNT;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_MAN_CREATE_PROCESS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::AI_VFX_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::BUILTIN_DELETE_2 as BUILTIN_DELETE;
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

/// `void cFlagManager_GroupInitKey(cFlagManager_Group *g)` (0x34af20): clear
/// the group's key fields (state -1 at +0x58, +0x5c = 0).
pub fn c_flag_manager_group_init_key(c: &mut Cpu) {
    c.cycles += 6;
    let g = Addr(abi::arg(c, 0));
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = c.g[V1][0] | 0xffff;
    g.off(0x5c).sw(c, 0);
    g.off(0x58).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `cFlagManager_GroupDtor` (0x34afb8): vtable at `+0x184`.
pub fn c_flag_manager_group_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x184, 0x0048_fbf8, 0x0034_afdc);
}

/// `void* Flag_ctor_2(Flag* self)` (0x34ae68).
pub fn flag_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        ctor_with_vtable(c, 0x0034_ae7c, callee::WORLD_OBJ_CTOR, 0x0048_fc10)
    })
}

/// `int cFlagManager_GroupMatches(cFlagManager* self, const void* data,
/// cFlagContext* ctx)` (0x34afe8): true when the group is live, its id matches
/// and the flag bytes compare equal.
pub fn c_flag_manager_group_matches(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        let this = r32(c, A0);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(this).off(0x5c).lw(c);
        let dead = c.g[V0][0] == 0;
        c.g[V0][0] = 0;
        if !dead {
            c.cycles += 5;
            c.g[V0][0] = Addr(r32(c, A2)).off(0x80).lw(c);
            c.g[A0][0] = Addr(this).off(0x58).lw(c);
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            if c.g[V1][0] != c.g[A0][0] {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[A1][0];
                c.g[A2][0] = 0x58;
                c.g[A1][0] = c.g[A3][0];
                call_ptr(c, 0x0041_5fc8, 0x0034_b024)?;
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rails (VU0).

/// `void FloatRail_ApplyImpulse(FloatRail* self, RailNode* node, Vec4* v)`
/// (0x00349798): move the two rail points the node sits between by
/// `v * (1 - t)` and `v * t`.
pub fn float_rail_apply_impulse(c: &mut Cpu) {
    c.cycles += 5;
    let node = abi::arg(c, 1);
    let self_ = abi::arg(c, 0);
    let v = abi::arg(c, 2);
    c.g[A3][0] = sx(node);
    c.g[T0][0] = sx(self_);
    let idx = Addr(p(node, 0x64)).lw(c);
    c.g[A0][0] = idx;
    let negative = (c.g[A0][0] as i64) < 0;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    if negative {
        c.cycles += 2;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        abi::ret(c);
        return;
    }
    c.cycles += 35;
    c.lwc1(1, p(node, 0x68)); // t
    c.g[A1][0] = 0x50;
    c.g[AT][0] = 0x3f80_0000; // 1.0f
    c.mtc1(1, 0);
    c.mult(0, 4, 4, 5);
    c.sub_s(0, 0, 1); // 1 - t
    c.mfc1(3, 0);
    c.lqc2(4, v);
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x70));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sq(2, c.g[SP][0] as u32);
    c.lqc2(3, c.g[A0][0] as u32);
    c.lqc2(4, c.g[SP][0] as u32);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, c.g[A0][0] as u32);
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x64));
    c.lwc1(0, (c.g[A3][0] as u32).wrapping_add(0x68));
    c.mult(0, 3, 3, 5);
    c.mfc1(2, 0);
    c.lqc2(4, v);
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x70));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.sq(2, c.g[SP][0] as u32);
    c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x50));
    c.lqc2(4, c.g[SP][0] as u32);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[V1][0] as u32).wrapping_add(0x50));
    c.cycles += 2;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    abi::ret(c);
}

/// `void FloatRail_GetContactVelocity(FloatRail* self, RailNode* node,
/// Vec4* out)` (0x00349840): blend the two rail point velocities by `t` and
/// add the contact's correction at +0x20.
pub fn float_rail_get_contact_velocity(c: &mut Cpu) {
    c.cycles += 5;
    let node = abi::arg(c, 1);
    let self_ = abi::arg(c, 0);
    c.g[A2][0] = sx(node);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x64));
    let negative = (c.g[V1][0] as i64) < 0;
    c.g[A3][0] = sx(self_);
    if negative {
        c.cycles += 2;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        abi::ret(c);
        return;
    }
    c.cycles += 35;
    c.g[A0][0] = 0x50;
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
    c.mult(0, 3, 3, 4);
    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x68));
    c.g[AT][0] = 0x3f80_0000; // 1.0f
    c.mtc1(1, 1);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.mfc1(5, 0);
    c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x10));
    c.qmtc2(5, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(2, c.g[SP][0] as u32);
    c.lqc2(3, (c.g[A2][0] as u32).wrapping_add(0x20));
    c.lqc2(4, c.g[SP][0] as u32);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[A2][0] as u32).wrapping_add(0x20));
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x64));
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
    c.mult(0, 3, 3, 4);
    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x68));
    c.sub_s(1, 1, 0); // 1 - t
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.mfc1(4, 1);
    c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x60));
    c.qmtc2(4, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(2, c.g[SP][0] as u32);
    c.lqc2(3, (c.g[A2][0] as u32).wrapping_add(0x20));
    c.lqc2(4, c.g[SP][0] as u32);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[A2][0] as u32).wrapping_add(0x20));
    c.cycles += 2;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    abi::ret(c);
}

/// `void SpringRail_ApplyImpulse(SpringRail* self, RailNode* node, Vec4* dir)`
/// (0x0034a568): clamp the direction to length 200 and push every rail
/// point up to the node with a spring force.
pub fn spring_rail_apply_impulse(c: &mut Cpu) {
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.cycles += 17;
    let self_ = abi::arg(c, 0);
    let node = abi::arg(c, 1);
    let dir = abi::arg(c, 2);
    c.g[AT][0] = 0x4348_0000; // 200.0f
    c.mtc1(1, 1);
    c.lqc2(3, dir);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(2, 22);
    c.mtc1(2, 0); // f0 = |dir|
    c.g[A3][0] = sx(self_);
    c.c_lt_s(1, 0); // 200 < |dir|
    let over = c.fcc();
    if over {
        c.cycles += 10;
        c.div_s(0, 1, 0); // 200 / |dir|
        c.mfc1(3, 0);
        c.lqc2(4, dir);
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(2, c.g[SP][0] as u32);
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    } else {
        c.cycles += 2;
        c.lq(2, dir);
        c.sq(2, c.g[SP][0] as u32);
    }
    // node->idx < 0: nothing to push.
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x64)).lw(c);
    if (c.g[V0][0] as i64) < 0 {
        c.cycles += 2;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        abi::ret(c);
        return;
    }
    // a = node->pos - elems[0].pos, c = dir x a, alen = |a|
    c.cycles += 30;
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
    c.g[T0][0] = 0;
    c.lqc2(3, c.g[A1][0] as u32);
    c.lqc2(4, c.g[V0][0] as u32);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[T2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x60));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lqc2(4, c.g[SP][0] as u32);
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x64));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(4, 22);
    c.mtc1(4, 2); // f2 = |a|
    if (c.g[V1][0] as i64) <= 0 {
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x64));
    } else {
        c.cycles += 4;
        c.g[T1][0] = c.g[T2][0];
        c.g[A2][0] = 0;
        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
        loop {
            c.cycles += 69;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
            c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(4, 22);
            c.mtc1(4, 0); // f0 = |vel[i]|
            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(3, 22);
            c.mtc1(3, 1); // f1 = |a|
            c.div_s(0, 0, 1);
            c.mfc1(3, 0);
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(3, 22);
            c.mtc1(3, 0); // f0 = |vel[i]|
            c.div_s(0, 0, 2); // / alen
            c.mfc1(3, 0);
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.lwc1(0, (c.g[T1][0] as u32).wrapping_add(0x8)); // k[2]
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.mfc1(4, 0);
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
            c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x60));
            c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x40));
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, (c.g[V0][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x64));
            c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x64));
                break;
            }
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
            if event_due(c) {
                deliver_event_here(c, 0x0034_a670);
            }
        }
    }
    // The node's own element.
    c.cycles += 42;
    c.g[V0][0] = 0x60;
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
    c.mult(0, 4, 4, 2); // idx * 0x60
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.lqc2(3, (c.g[A1][0] as u32));
    c.lqc2(4, c.g[V1][0] as u32);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(3, 22);
    c.mtc1(3, 0);
    c.div_s(0, 0, 2);
    c.mfc1(3, 0);
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lwc1(0, (c.g[T2][0] as u32).wrapping_add(0x8)); // k[2]
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.mfc1(5, 0);
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.qmtc2(5, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x40));
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[A0][0] as u32).wrapping_add(0x40));
    c.cycles += 2;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Rails.

/// `void FloatRail_v01(FloatRail* self, int flags)` (0x3490b0): detach the
/// rail from the spatial tree and run the base destructor. The `v01` in the
/// name marks this as the vector-deleting variant.
pub fn float_rail_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xd0));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(p_2(c.g[S0][0], 0xc)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0034_8c48, 0x0034_90d8)?;
        c.g[A0][0] = lw(c, p_2(c.g[S0][0], 0x70));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0034_90ec)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.cycles += 7;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0034_7f90, 0x0034_90f8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SpringRail_v01(SpringRail* self, int flags)` (0x349eb8): the same
/// teardown for a spring rail (its own vtable, base `FlexRail_dtor`).
pub fn spring_rail_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x270));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(p_2(c.g[S0][0], 0xc)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0034_8c48, 0x0034_9ee0)?;
        c.g[A0][0] = lw(c, p_2(c.g[S0][0], 0x70));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0034_9ef4)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.cycles += 7;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0034_7f90, 0x0034_9f00)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `func_00349AD0` (0x349ad0): `FloatRail_Save`, the float rail's serialiser.
/// Saves the flex rail (0x348b40), writes the 0xc-byte float block through the
/// archive's virtual writer (`vtable[8]`, `this+0x60`), then the variable-size
/// block: `size = *(int*)(this+0x20) * 0xc / ...` -- the original multiplies
/// the count at `+0x20` by 0xc (`mult`), adds it to the object base and writes
/// `this+0x70` through the writer.
pub(crate) fn rail_save(c: &mut Cpu, tail_block: u32, rets: [u32; 3]) -> Flow {
    c.cycles += 29;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[RA][0] = sx(rets[0]);
    c.g[S1][0] = c.g[A1][0];
    callee::FLEX_RAIL_SAVE.call(c, rets[0])?;
    c.g[V1][0] = lw_r(c, S1, 0);
    c.g[A1][0] = sx(low(c, S0).wrapping_add(0x60));
    c.g[A2][0] = 0xc;
    c.g[A0][0] = lh_r(c, V1, 0x8);
    c.g[V0][0] = lw_r(c, V1, 0xc);
    let target = low(c, V0);
    c.g[RA][0] = sx(rets[1]);
    c.g[A0][0] = sx(low(c, S1).wrapping_add(low(c, A0)));
    call_ptr(c, target, rets[1])?;
    c.g[A0][0] = lw_r(c, S0, 0x20);
    c.g[A2][0] = sx(tail_block);
    c.g[V1][0] = lw_r(c, S1, 0);
    c.mult(0, 0, A0, A2);
    c.g[A1][0] = lw_r(c, S0, 0x70);
    c.g[A0][0] = lh_r(c, V1, 0x8);
    c.g[V0][0] = lw_r(c, V1, 0xc);
    c.g[A2][0] = c.lo[0];
    let target = low(c, V0);
    c.g[RA][0] = sx(rets[2]);
    c.g[A0][0] = sx(low(c, S1).wrapping_add(low(c, A0)));
    call_ptr(c, target, rets[2])?;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `FloatRail_Save` (0x349ad0): 0x50-byte variable block.
pub fn float_rail_save(c: &mut Cpu) {
    guest(c, |c| {
        rail_save(c, 0x50, [0x0034_9aec, 0x0034_9b08, 0x0034_9b30])
    })
}

/// `SpringRail_Save` (0x34ac10): same shape, 0x60-byte variable block.
pub fn spring_rail_save(c: &mut Cpu) {
    guest(c, |c| {
        rail_save(c, 0x60, [0x0034_ac2c, 0x0034_ac48, 0x0034_ac70])
    })
}

/// `void* cFlagManager_GroupCtor(self)` (0x0034af38): construct a flag group
/// and clear its 60-entry key table.
pub fn c_flag_manager_group_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::C_FLAG_MANAGER_GROUP_INIT_KEY.call(c, 0x0034_af4c)?;
        c.g[V1][0] = sx(0x490000);
        Addr(at(c, S0, 0x60)).sw(c, 0);
        c.g[V1][0] = sx(0x490000u32.wrapping_sub(0x408));
        Addr(at(c, S0, 0x74)).sw(c, 0);
        Addr(at(c, S0, 0x184)).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx(word(c, S0).wrapping_add(0x180));
        c.g[V1][0] = 0x3b;
        loop {
            c.cycles += 7;
            Addr(at(c, V0, 0)).sw(c, 0);
            c.g[V1][0] = sx(word(c, V1).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(word(c, V0).wrapping_sub(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0034_af68);
                }
                continue;
            }
            c.cycles += 6;
            c.g[V0][0] = Addr(gp(c, 0xf20)).lw(c);
            c.g[V1][0] = sx(word(c, V0).wrapping_add(1));
            Addr(at(c, S0, 0x90)).sw(c, c.g[V0][0]);
            c.g[A0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
            let keep = c.g[A0][0] != 0;
            Addr(gp(c, 0xf20)).sw(c, c.g[V1][0]);
            if !keep {
                c.cycles += 1;
                Addr(gp(c, 0xf20)).sw(c, 0);
            }
            c.cycles += 5;
            c.g[V0][0] = c.g[S0][0];
            c.g[RA][0] = c.ld(frame.at(c, 0));
            c.lq(16, frame.at(c, 0x10));
            frame.close(c);
            break;
        }
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Spring rails.

/// `void SpringRail_InitPosFromInstance(SpringRail* node, int bone)` (0x34a028):
/// resolve the node's model and bone, then copy the world matrix translation
/// into the node's position.
pub fn spring_rail_init_pos_from_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x50);
        c.g[RA][0] = sx(0x0034_a03c);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x002d_1bd8, 0x0034_a03c)?;
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        c.g[A1][0] = Addr(p_3(r32(c, S0), 0x50)).lw(c);
        c.g[A0][0] = Addr(p_3(r32(c, V1), 8)).lw(c);
        c.g[V0][0] = c.g[A1][0] & 0xff;
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(p_3(r32(c, V0), r32(c, A0)));
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        let empty = c.g[V1][0] == 0;
        c.g[A0][0] = 0;
        if !empty {
            c.cycles += 8;
            c.g[A0][0] = Addr(p_3(r32(c, V1), 0x1c)).lw(c);
            c.g[V0][0] = sx(r32(c, A1) >> 8);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(p_3(r32(c, V0), r32(c, A0)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            c.g[V0][0] = sx(r32(c, V1) >> 8);
            let empty2 = c.g[V0][0] == 0;
            c.g[A0][0] = 0;
            if !empty2 {
                c.cycles += 1;
                c.g[A0][0] = sx(r32(c, V0) << 2);
            }
        }
        c.cycles += 10;
        c.g[A1][0] = Addr(p_3(r32(c, S0), 0x54)).lw(c);
        c.g[RA][0] = sx(0x0034_a090);
        c.g[A2][0] = c.g[SP][0];
        callee::INSTANCE_GET_NODE_MATRIX.call(c, 0x0034_a090)?;
        c.lq(2, p_3(r32(c, SP), 0x30));
        c.g[RA][0] = c.ld(p_3(r32(c, SP), 0x50));
        c.sq(2, r32(c, S0));
        c.lq(16, p_3(r32(c, SP), 0x60));
        c.sq(2, p_3(r32(c, SP), 0x40));
        let target = r32(c, RA);
        c.g[SP][0] = sx(p_3(r32(c, SP), 0x70));
        c.pc = target;
        Ok(())
    })
}

/// `void FloatRail_ResetPosFromInstance(FloatRail* self, Instance* inst)`
/// (0x349220): look the instance's node up in the world table and copy its
/// node matrix into the rail's transform.
pub fn float_rail_reset_pos_from_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        open(c, 0x70);
        let sp0 = spv(c);
        c.sq(16, at_2(sp0, 0x60));
        c.sd(at_2(sp0, 0x50), c.g[RA][0]);
        c.g[S0][0] = c.g[A1][0];
        callee::WORLD_GET_INSTANCE.call(c, 0x0034_9234)?;
        c.g[V1][0] = lw_at(c, c.g[V0][0], 0);
        let inst = c.g[S0][0];
        c.g[A1][0] = lw_at(c, inst, 0x40);
        c.g[A0][0] = lw_at(c, c.g[V1][0], 8);
        c.g[V0][0] = c.g[A1][0] & 0xff;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx(at_2(c.g[V0][0] as u32, c.g[A0][0] as u32));
        c.g[V1][0] = lw_at(c, c.g[V0][0], 0);
        let none = c.g[V1][0] == 0;
        c.g[A0][0] = 0;
        if !none {
            c.cycles += 8;
            c.g[A0][0] = lw_at(c, c.g[V1][0], 0x1c);
            c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx(at_2(c.g[V0][0] as u32, c.g[A0][0] as u32));
            c.g[V1][0] = lw_at(c, c.g[V0][0], 0);
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            let none2 = c.g[V0][0] == 0;
            c.g[A0][0] = 0;
            if !none2 {
                c.cycles += 1;
                c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
            }
        }
        c.cycles += 11;
        c.g[A1][0] = lw_at(c, inst, 0x44);
        c.g[A2][0] = c.g[SP][0];
        callee::INSTANCE_GET_NODE_MATRIX_2.call(c, 0x0034_9288)?;
        c.lq(2, at_2(sp0, 0x30));
        c.g[RA][0] = c.ld(at_2(sp0, 0x50));
        c.sq(2, at_2(inst as u32, 0x20));
        c.sq(2, at_2(sp0, 0x40));
        c.sq(2, at_2(inst as u32, 0x30));
        c.lq(16, at_2(sp0, 0x60));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_2(sp0, 0x70));
        c.pc = ra;
        Ok(())
    })
}

/// `void* Flag_ctor(Flag* self, void* node, const Params* params, void*
/// owner)` (0x34add8): build the world object, then read the flag parameters
/// and register the flag with the flag manager.
pub fn flag_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0xa0);
        let self_ = abi::arg(c, 0);
        let node = abi::arg64(c, 1);
        let params = abi::arg64(c, 2);
        let owner = abi::arg64(c, 3);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S0][0] = params;
        frame.save(c, S2, 0x70);
        c.g[S1][0] = sx(self_);
        c.g[S2][0] = owner;
        frame.save_ra(c, 0x60);
        c.g[A3][0] = c.g[S0][0];
        c.g[A1][0] = node;
        c.g[A2][0] = 0xa;
        call_ptr(c, callee::WORLD_OBJ_CTOR_3, 0x0034_ae04)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A2][0] = 0xffff_ffff_ffff_0000;
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x3f0));
        c.g[A2][0] = c.g[A2][0] | 0xfffd;
        c.sw(at(c, S1, 0xc), c.g[V1][0]);
        c.g[A0][0] = sx(frame.at(c, 0));
        c.g[A1][0] = c.g[S2][0];
        c.g[V0][0] = c.lw(at(c, S0, 8));
        c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
        c.g[V0][0] = c.g[V0][0] | 4;
        c.sw(at(c, S0, 8), c.g[V0][0]);
        call_ptr(c, callee::FLAG_READ_PARAMS, 0x0034_ae34)?;
        call_ptr(c, callee::GET_FLAG_MANAGER, 0x0034_ae3c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = sx(frame.at(c, 0));
        call_ptr(c, callee::FLAG_MANAGER_ADD_FLAG, 0x0034_ae4c)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// `void FlexRail_RegisterSpaceNode(self, node)` (0x348b90): allocate a rail
/// entry for a space node and register it with the spatial hash.
pub fn flex_rail_register_space_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 45;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0x490000;
        frame.save(c, S0, 0x40);
        c.g[A2][0] = 0x2000_0000;
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17f8));
        c.g[A0][0] = 0x10;
        call_ptr(c, 0x0031_7d70, 0x0034_8bc0)?;
        c.g[V1][0] = 0x3;
        c.sw(p_4(c.g[V0][0] as u32, 0xc), c.g[S0][0]);
        c.sw(p_4(c.g[V0][0] as u32, 8), c.g[V1][0]);
        c.sw(p_4(c.g[S0][0] as u32, 0x54), c.g[V0][0]);
        call_ptr(c, 0x002d_1be0, 0x0034_8bd4)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
        c.g[S2][0] = Addr(p_4(c.g[S0][0] as u32, 0x54)).lw(c);
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x0032_8f28, 0x0034_8be8)?;
        c.g[V0][0] = Addr(p_4(c.g[SP][0] as u32, 4)).lw(c);
        c.g[A3][0] = 0xffff_ffff_ffff_ffff;
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x50));
        c.g[A1][0] = Addr(p_4(c.g[SP][0] as u32, 8)).lw(c);
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[A2][0] = Addr(p_4(c.g[SP][0] as u32, 0xc)).lw(c);
        if c.g[V0][0] != 0 {
            c.g[V1][0] = c.g[S1][0];
        }
        c.g[A1][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[A3][0] = ((c.g[A3][0] as i64) < (c.g[A2][0] as i64)) as u64;
        if c.g[A1][0] != 0 {
            c.g[A0][0] = c.g[V1][0];
        }
        c.g[A2][0] = c.g[S2][0];
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
        c.g[A1][0] = 0x2;
        if c.g[A3][0] == 0 {
            c.g[A0][0] = c.g[V0][0];
        }
        c.g[A3][0] = c.g[SP][0];
        call_ptr(c, 0x0032_8c20, 0x0034_8c2c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
pub(crate) fn at_3(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A 32-bit register value.
#[inline(always)]
pub(crate) fn r32v(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// `off(sp)`.
#[inline(always)]
pub(crate) fn spa(c: &Cpu, off: u32) -> u32 {
    r32v(c, SP).wrapping_add(off)
}

// ---------------------------------------------------------------------------
// Rails.

/// `void FloatRail_v50(FloatRail* rail)` (0x349110): grow the rail's bounding
/// box (`+0x30`: min, `+0x40`: max) over the points at_3 `*(rail+0x70)`
/// (`+0x20` of that array is the count, points are 0x50 bytes each; the first
/// point is skipped), then widen it by 200 units in every direction.
pub fn float_rail_v50(c: &mut Cpu) {
    c.cycles += 9;
    let this = r32v(c, A0);
    // *(rail+0x70): point array; *(rail+0x20): point count.
    c.g[A1][0] = Addr(at_3(this, 0x70)).lw(c);
    c.g[V1][0] = 1;
    c.g[A2][0] = Addr(at_3(this, 0x20)).lw(c);
    let points = r32v(c, A1);
    let count = c.g[A2][0];
    c.g[SP][0] = sx(r32v(c, SP).wrapping_sub(0x10));
    // Seed both corners from the second point (+0x20 of the array).
    c.lq(2, at_3(points, 0x20));
    c.g[V1][0] = (1i64 < count as i64) as u64;
    c.sq(2, at_3(this, 0x30));
    let one_point = c.g[V1][0] == 0;
    c.sq(2, at_3(this, 0x40));
    if !one_point {
        c.cycles += 3;
        c.g[V0][0] = sx(at_3(points, 0x70));
        let mut p = at_3(points, 0x70);
        let mut n = r32v(c, A2).wrapping_sub(1);
        loop {
            // min: keep the smaller of each axis (x, y, z).
            c.cycles += 6;
            c.lwc1(1, p);
            c.lwc1(0, at_3(this, 0x30));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x30));
            }
            c.cycles += 6;
            c.lwc1(1, at_3(p, 4));
            c.lwc1(0, at_3(this, 0x34));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x34));
            }
            c.cycles += 6;
            c.lwc1(1, at_3(p, 8));
            c.lwc1(0, at_3(this, 0x38));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x38));
            }
            // max: keep the larger of each axis.
            c.cycles += 6;
            c.lwc1(1, p);
            c.lwc1(0, at_3(this, 0x40));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x40));
            }
            c.cycles += 6;
            c.lwc1(1, at_3(p, 4));
            c.lwc1(0, at_3(this, 0x44));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x44));
            }
            c.cycles += 6;
            c.lwc1(1, at_3(p, 8));
            c.lwc1(0, at_3(this, 0x48));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x48));
            }
            c.cycles += 3;
            n = n.wrapping_sub(1);
            let more = n != 0;
            c.g[V1][0] = sx(n);
            c.g[V0][0] = sx(p.wrapping_add(0x50));
            if more && event_due(c) {
                deliver_event_here(c, 0x0034_9140);
            }
            p = p.wrapping_add(0x50);
            if !more {
                break;
            }
        }
    }
    c.cycles += 16;
    // Widen by 200 units on each side: one VU0 subtract, one add.
    c.g[AT][0] = 0x4348_0000;
    c.mtc1(1, 0);
    c.sw(spa(c, 0xc), 0);
    c.swc1(0, spa(c, 8));
    c.swc1(0, spa(c, 0));
    c.swc1(0, spa(c, 4));
    c.lqc2(3, at_3(this, 0x30));
    c.lqc2(4, spa(c, 0));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at_3(this, 0x30));
    c.lqc2(3, at_3(this, 0x40));
    c.lqc2(4, spa(c, 0));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at_3(this, 0x40));
    c.g[SP][0] = sx(r32v(c, SP).wrapping_add(0x10));
    abi::ret(c);
}

/// `void SpringRail_v50(SpringRail* rail)` (0x349f18): as
/// [`float_rail_v50`], but the points live at_3 `*(rail+0x70)` themselves and
/// are 0x60 bytes each (six floats: position and velocity).
pub fn spring_rail_v50(c: &mut Cpu) {
    c.cycles += 9;
    let this = r32v(c, A0);
    c.g[A1][0] = Addr(at_3(this, 0x70)).lw(c);
    c.g[V1][0] = 1;
    c.g[A2][0] = Addr(at_3(this, 0x20)).lw(c);
    let points = r32v(c, A1);
    let count = c.g[A2][0];
    c.g[SP][0] = sx(r32v(c, SP).wrapping_sub(0x10));
    c.lq(2, points);
    c.g[V1][0] = (1i64 < count as i64) as u64;
    c.sq(2, at_3(this, 0x30));
    let one_point = c.g[V1][0] == 0;
    c.sq(2, at_3(this, 0x40));
    if !one_point {
        c.cycles += 3;
        c.g[V0][0] = sx(at_3(points, 0x60));
        let mut p = at_3(points, 0x60);
        let mut n = r32v(c, A2).wrapping_sub(1);
        loop {
            c.cycles += 6;
            c.lwc1(1, p);
            c.lwc1(0, at_3(this, 0x30));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x30));
            }
            c.cycles += 6;
            c.lwc1(1, at_3(p, 4));
            c.lwc1(0, at_3(this, 0x34));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x34));
            }
            c.cycles += 6;
            c.lwc1(1, at_3(p, 8));
            c.lwc1(0, at_3(this, 0x38));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x38));
            }
            c.cycles += 6;
            c.lwc1(1, p);
            c.lwc1(0, at_3(this, 0x40));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x40));
            }
            c.cycles += 6;
            c.lwc1(1, at_3(p, 4));
            c.lwc1(0, at_3(this, 0x44));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x44));
            }
            c.cycles += 6;
            c.lwc1(1, at_3(p, 8));
            c.lwc1(0, at_3(this, 0x48));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x48));
            }
            c.cycles += 3;
            n = n.wrapping_sub(1);
            let more = n != 0;
            c.g[V1][0] = sx(n);
            c.g[V0][0] = sx(p.wrapping_add(0x60));
            if more && event_due(c) {
                deliver_event_here(c, 0x0034_9f48);
            }
            p = p.wrapping_add(0x60);
            if !more {
                break;
            }
        }
    }
    c.cycles += 16;
    c.g[AT][0] = 0x4348_0000;
    c.mtc1(1, 0);
    c.sw(spa(c, 0xc), 0);
    c.swc1(0, spa(c, 8));
    c.swc1(0, spa(c, 0));
    c.swc1(0, spa(c, 4));
    c.lqc2(3, at_3(this, 0x30));
    c.lqc2(4, spa(c, 0));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at_3(this, 0x30));
    c.lqc2(3, at_3(this, 0x40));
    c.lqc2(4, spa(c, 0));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at_3(this, 0x40));
    c.g[SP][0] = sx(r32v(c, SP).wrapping_add(0x10));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// BE profile events.

/// The per-event-record table `BE_GetEventRecordIndex` scans (2 entries of
/// `{id, index}` per event id), and the per-place record table this function
/// indexes by place (`+0x54` of each 100-byte record).
pub(crate) const EVENT_TABLE: u32 = 0x0045_a2f8;

pub(crate) const PLACE_TABLE: u32 = 0x0043_d950;

/// `sFrRailDesc* Flag_ReadParams(sFrRailDesc* out, const FlagSrc* src)`
/// (0x34ac88): convert a rail modifier's source (flag words, floats) into
/// the runtime descriptor: four flag bytes, p/q, r0/r1 and each `t[]` triple,
/// with the rates and the `a` components scaled by `1/framerate`
/// (`*(int*)(gp+0x2a74 + 0x10)`).
pub fn flag_read_params(c: &mut Cpu) {
    c.cycles += 83;
    c.g[A2][0] = c.ldw_2(A1, 0x4);
    c.g[V0][0] = c.g[A0][0];
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 3);
    c.g[A2][0] = (0u64 < c.g[A2][0]) as u64;
    c.stb(A0, 0, c.g[A2][0]);
    c.g[V1][0] = c.ldw_2(A1, 0x8);
    c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
    c.stb(A0, 0x1, c.g[V1][0]);
    c.g[A2][0] = c.ldw_2(A1, 0xc);
    c.g[A2][0] = (0u64 < c.g[A2][0]) as u64;
    c.stb(A0, 0x2, c.g[A2][0]);
    c.g[V1][0] = c.ldw_2(A1, 0x64);
    c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
    c.stb(A0, 0x3, c.g[V1][0]);
    c.lwc1(0, c.pa(A1, 0x40));
    c.lwc1(1, c.pa(A1, 0x44));
    c.lwc1(2, c.pa(A1, 0x48));
    c.swc1(0, c.pa(A0, 0x34));
    c.swc1(1, c.pa(A0, 0x38));
    c.swc1(2, c.pa(A0, 0x3c));
    c.g[V1][0] = c.ldw_2(GP, 0x2a74);
    c.lwc1(0, c.pa(A1, 0x4c));
    c.lwc1(1, c.pa(A1, 0x50));
    c.lwc1(2, c.pa(A1, 0x54));
    c.swc1(0, c.pa(A0, 0x40));
    c.swc1(2, c.pa(A0, 0x48));
    c.swc1(1, c.pa(A0, 0x44));
    c.lwc1(1, c.pa(A1, 0x58));
    c.swc1(1, c.pa(A0, 0x4c));
    c.lwc1(0, c.pa(V1, 0x10));
    c.cvt_s_w(0, 0);
    c.lwc1(1, c.pa(A1, 0x5c));
    c.div_s(0, 3, 0);
    c.mul_s(1, 1, 0);
    c.swc1(1, c.pa(A0, 0x50));
    c.lwc1(0, c.pa(V1, 0x10));
    c.cvt_s_w(0, 0);
    c.lwc1(1, c.pa(A1, 0x60));
    c.div_s(0, 3, 0);
    c.mul_s(1, 1, 0);
    c.swc1(1, c.pa(A0, 0x54));
    c.lwc1(0, c.pa(V1, 0x10));
    c.cvt_s_w(0, 0);
    c.lwc1(1, c.pa(A1, 0x10));
    c.div_s(0, 3, 0);
    c.mul_s(1, 1, 0);
    c.swc1(1, c.pa(A0, 0x4));
    c.lwc1(0, c.pa(A1, 0x14));
    c.swc1(0, c.pa(A0, 0x14));
    c.lwc1(1, c.pa(A1, 0x18));
    c.swc1(1, c.pa(A0, 0x24));
    c.lwc1(0, c.pa(V1, 0x10));
    c.cvt_s_w(0, 0);
    c.lwc1(1, c.pa(A1, 0x1c));
    c.div_s(0, 3, 0);
    c.mul_s(1, 1, 0);
    c.swc1(1, c.pa(A0, 0x8));
    c.lwc1(0, c.pa(A1, 0x20));
    c.swc1(0, c.pa(A0, 0x18));
    c.lwc1(1, c.pa(A1, 0x24));
    c.swc1(1, c.pa(A0, 0x28));
    c.lwc1(0, c.pa(V1, 0x10));
    c.cvt_s_w(0, 0);
    c.lwc1(1, c.pa(A1, 0x28));
    c.div_s(0, 3, 0);
    c.mul_s(1, 1, 0);
    c.swc1(1, c.pa(A0, 0xc));
    c.lwc1(0, c.pa(A1, 0x2c));
    c.swc1(0, c.pa(A0, 0x1c));
    c.lwc1(1, c.pa(A1, 0x30));
    c.swc1(1, c.pa(A0, 0x2c));
    c.lwc1(2, c.pa(V1, 0x10));
    c.cvt_s_w(2, 2);
    c.lwc1(0, c.pa(A1, 0x34));
    c.div_s(3, 3, 2);
    c.mul_s(0, 0, 3);
    c.swc1(0, c.pa(A0, 0x10));
    c.lwc1(1, c.pa(A1, 0x38));
    c.swc1(1, c.pa(A0, 0x20));
    c.lwc1(0, c.pa(A1, 0x3c));
    let ra = c.g[RA][0] as u32;
    c.swc1(0, c.pa(A0, 0x30));
    c.pc = ra;
    return;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
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
    use crate::difftest::SP as STACK_TOP;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_2;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base_2;
    pub(crate) use crate::game::movie::movieplayer::tests::w as w_3;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2 as BE;
    pub(crate) use crate::game::movie::movieplayer::tests::W;
    pub(crate) use crate::game::object::deadfadenode::tests::ctor_setup;
    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS;
    pub(crate) use crate::game::visualfx::worldlightman::tests::alloc_setup;
    pub(crate) use crate::game::visualfx::worldlightman::tests::dtor_case;
    pub(crate) use crate::game::visualfx::worldlightman::tests::MEM_MUTEX;
    pub(crate) use crate::game::world::worldview::tests::word;
    pub(crate) use crate::game::worldpainter::quadtree::tests::put_vtable;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn c_flag_manager_group_init_key_matches() {
        check(
            ssx3_ee::s11::f_0034af20,
            c_flag_manager_group_init_key,
            200,
            &|c, seed| {
                let mut r = Rng(0x1014 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x58, r.next_u32());
                c.bus.write32(DATA + 0x5c, r.next_u32());
            },
        );
    }

    #[test]
    fn c_flag_manager_group_dtor_matches() {
        dtor_case(
            ee::s11::f_0034afb8__generated,
            c_flag_manager_group_dtor,
            0x34afb8,
        );
    }

    #[test]
    fn flag_ctor_2_matches() {
        check(
            ee::s11::f_0034ae68__generated,
            flag_ctor_2,
            80,
            &|c, seed| {
                ctor_setup(c, seed, 0x34ae_68, 0x20, DATA + 0x300);
            },
        );
    }

    #[test]
    fn c_flag_manager_group_matches_matches() {
        check(
            ee::s11::f_0034afe8__generated,
            c_flag_manager_group_matches,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34af_e8, 1);
                let this = DATA + 0x100;
                let data = DATA + 0x400;
                let ctx = DATA + 0x600;
                clear(c, this, 0x80);
                clear(c, data, 0x20);
                clear(c, ctx, 0x90);
                c.bus
                    .write32(this + 0x5c, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(this + 0x58, r.next_u32());
                let group = c.bus.read32(this + 0x58);
                c.bus.write32(ctx + 0x80, DATA + 0x700);
                c.bus.write32(
                    DATA + 0x700,
                    if seed % 3 == 1 { group } else { r.next_u32() },
                );
                // The 0x58-byte flag block lives at `self`, so the copy at
                // `data` compares equal on some seeds.
                for i in 0..0x16u32 {
                    let v = if seed % 4 == 0 { 0 } else { r.next_u32() };
                    c.bus.write32(this + 4 * i, v);
                    c.bus.write32(data + 4 * i, v);
                }
                if seed % 4 != 0 {
                    c.bus.write32(data + 0x30, r.next_u32());
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(data);
                c.g[A2][0] = sx(ctx);
            },
        );
    }

    // ------------------------------------------------------------------- rails

    /// A rail: `elems` at +0x70 holding `n` 0x60-byte points, plus a node
    /// (idx at +0x64, blend at +0x68).
    pub(crate) fn rail(c: &mut Cpu, r: &mut Rng, self_: u32, node: u32, elems: u32, n: u32) {
        word(c, self_ + 0x70, elems);
        // the spring stiffness coefficients at +0x60..+0x70
        for k in (0..0x10).step_by(4) {
            word(c, self_ + 0x60 + k, r.edge_f32());
        }
        word(c, node + 0x64, 0xffff_ffff);
        word(c, node + 0x68, r.edge_f32());
        for i in 0..n {
            for k in (0..0x60).step_by(4) {
                word(c, elems + 0x60 * i + k, r.edge_f32());
            }
        }
        // the node's own vectors
        for k in 0..8 {
            word(c, node + 4 * k, r.edge_f32());
        }
        for k in 0..8 {
            word(c, node + 0x20 + 4 * k, r.edge_f32());
        }
        for k in 0..8 {
            word(c, node + 0x40 + 4 * k, r.edge_f32());
        }
        // the trailing k[] coefficient the spring reads at elems + n*0x60 + 8
        word(c, elems + 0x60 * n + 8, r.edge_f32());
    }

    #[test]
    fn float_rail_apply_impulse_matches() {
        check_lohi(
            ssx3_ee::s11::f_00349798__generated,
            float_rail_apply_impulse,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x349798, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A2][0] = sx(DATA + 0x1800);
                rail(c, &mut r, DATA, DATA + 0x1000, DATA + 0x2000, 3);
                if seed % 4 != 0 {
                    word(c, DATA + 0x1064, (seed % 3) as u32);
                }
                for k in 0..4 {
                    word(c, DATA + 0x1800 + 4 * k, r.f32(10.0));
                }
            },
        );
    }

    #[test]
    fn float_rail_get_contact_velocity_matches() {
        check_lohi(
            ssx3_ee::s11::f_00349840__generated,
            float_rail_get_contact_velocity,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x349840, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A2][0] = sx(DATA + 0x1800);
                rail(c, &mut r, DATA, DATA + 0x1000, DATA + 0x2000, 3);
                if seed % 4 != 0 {
                    word(c, DATA + 0x1064, (seed % 3) as u32);
                }
            },
        );
    }

    #[test]
    fn spring_rail_apply_impulse_matches() {
        check_lohi(
            ssx3_ee::s11::f_0034a568__generated,
            spring_rail_apply_impulse,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34a568, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A2][0] = sx(DATA + 0x1800);
                rail(c, &mut r, DATA, DATA + 0x1000, DATA + 0x2000, 4);
                let idx = match seed % 5 {
                    0 => 0xffff_ffff,
                    1 => 0,
                    2 => 1,
                    3 => 2,
                    _ => 3,
                };
                word(c, DATA + 0x1064, idx);
                // directions: tiny, medium and over the 200 length clamp
                let scale: f32 = match seed % 4 {
                    0 => 1.0e-6,
                    1 => 1.0,
                    2 => 100.0,
                    _ => 1000.0,
                };
                for k in 0..4 {
                    word(c, DATA + 0x1800 + 4 * k, r.f32(scale));
                }
            },
        );
    }

    #[test]
    fn float_rail_v01_matches() {
        check(
            ee::s11::f_003490b0__generated,
            float_rail_v01,
            24,
            &|c, seed| {
                let _ = base(c, seed, 0x3490_b0);
                let s = DATA + 0x100;
                clear(c, s, 0x400);
                // `FlexRail_UnregisterSpaceNode` -> `World_GetNash()` and
                // `SpatialHash_Remove`: if the cell's level word does not match
                // the key's (11, from `SpatialKey_FromAABB`), the latter descends
                // into the (null) child forever. Give it a real cell whose level
                // matches, so the non-recursive path is taken.
                let world = DATA + 0x1000;
                let man = DATA + 0x2000;
                let cell = DATA + 0x3000;
                clear(c, world, 0x40);
                clear(c, man, 0x40);
                clear(c, cell, 0x40);
                w(c, cell, 11);
                w(c, gp_at(-0x848), world as u32);
                w(c, world + 0x84, man as u32);
                w(c, man + 0x20, cell as u32);
                // No node and no owned memory: no allocator at all on this path.
                w(c, s + 0x54, 0);
                w(c, s + 0x70, 0);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn spring_rail_v01_matches() {
        check(
            ee::s11::f_00349eb8__generated,
            spring_rail_v01,
            24,
            &|c, seed| {
                let _ = base(c, seed, 0x349e_b8);
                let s = DATA + 0x100;
                clear(c, s, 0x400);
                // See `float_rail_v01_matches`: match the spatial hash's level.
                let world = DATA + 0x1000;
                let man = DATA + 0x2000;
                let cell = DATA + 0x3000;
                clear(c, world, 0x40);
                clear(c, man, 0x40);
                clear(c, cell, 0x40);
                w(c, cell, 11);
                w(c, gp_at(-0x848), world as u32);
                w(c, world + 0x84, man as u32);
                w(c, man + 0x20, cell as u32);
                w(c, s + 0x54, 0);
                w(c, s + 0x70, 0);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn float_rail_save_matches() {
        check(
            ee::s11::f_00349ad0__generated,
            float_rail_save,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x349ad0, 4);
                be_library(c);
                world(c);
                let this = DATA;
                let arch = DATA + 0x400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = arch as u64;
                w_2(c, arch, DATA + 0x2000);
                slot(c, DATA + 0x2000, 8, 0, FAKE);
                w_2(c, this + 0x20, seed % 5);
                w_2(c, this + 0x70, DATA + 0x900);
            },
        );
    }

    #[test]
    fn spring_rail_save_matches() {
        check(
            ee::s11::f_0034ac10__generated,
            spring_rail_save,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x34ac10, 4);
                be_library(c);
                world(c);
                let this = DATA;
                let arch = DATA + 0x400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = arch as u64;
                w_2(c, arch, DATA + 0x2000);
                slot(c, DATA + 0x2000, 8, 0, FAKE);
                w_2(c, this + 0x20, seed % 5);
                w_2(c, this + 0x70, DATA + 0x900);
            },
        );
    }

    #[test]
    fn c_flag_manager_group_ctor_matches() {
        check(
            ee::s11::f_0034af38__generated,
            c_flag_manager_group_ctor,
            12,
            &|c, seed| {
                let _ = base_2(c, seed, 0x34af_38, 4);
                w_3(c, GAME_GP.wrapping_add(0xf20), (seed % 3) as u32);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn flag_ctor_matches() {
        check(
            ee::s11::f_0034add8__generated,
            flag_ctor,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34ad, 0);
                env(c);
                let this = DATA + 0x400;
                let node = DATA + 0x800;
                let src = DATA + 0x1000;
                w32(c, gpat(-0xc8), 0);
                w32(c, node + 8, r.next_u32());
                for k in 0..8u32 {
                    w32(c, src + 4 * k, r.next_u32());
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(node);
                c.g[A2][0] = sx(src);
                c.g[A3][0] = sx(DATA + 0x2000);
            },
        );
    }

    /// Scrambled registers and a game-like `gp`.
    pub(crate) fn base_3(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x143_0000 ^ ((seed as u64) << 11) ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    /// Compare hard state, caller-saved "soft" state and LO/HI for every seed.
    pub(crate) fn check_all(
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        for seed in 0..seed_count(seeds) {
            let r = compare(generated, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed} hard: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} soft: {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} lohi: {:#?}", r.lohi);
        }
    }

    pub(crate) fn floats(c: &mut Cpu, r: &mut Rng, addr: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(addr + 4 * i, r.edge_f32());
        }
    }

    // -----------------------------------------------------------------------

    #[test]
    fn float_rail_v50_matches() {
        check_all(
            ee::s11::f_00349110__generated,
            float_rail_v50,
            300,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x34_9110);
                c.g[A0][0] = sx(DATA);
                let count = if seed % 5 == 0 {
                    seed % 2
                } else {
                    2 + r.next_u32() % 40
                };
                c.bus.write32(DATA + 0x70, DATA + 0x2000);
                c.bus.write32(DATA + 0x20, count);
                // The first point comes from +0x20 of the array, the rest from
                // +0x70 with a 0x50 stride.
                floats(c, &mut r, DATA + 0x2000, count * 0x50 / 4 + 8);
                floats(c, &mut r, DATA, 2); // the `lq` seeds from +0x30/from +0x40
                floats(c, &mut r, DATA + 0x10, 2);
                for off in [0x30u32, 0x34, 0x38, 0x40, 0x44, 0x48] {
                    c.bus.write32(DATA + off, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn spring_rail_v50_matches() {
        check_all(
            ee::s11::f_00349f18__generated,
            spring_rail_v50,
            300,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x34_9f18);
                c.g[A0][0] = sx(DATA);
                let count = if seed % 5 == 0 {
                    seed % 2
                } else {
                    2 + r.next_u32() % 40
                };
                c.bus.write32(DATA + 0x70, DATA + 0x2000);
                c.bus.write32(DATA + 0x20, count);
                floats(c, &mut r, DATA + 0x2000, count * 0x60 / 4 + 8);
                for off in [0x30u32, 0x34, 0x38, 0x40, 0x44, 0x48] {
                    c.bus.write32(DATA + off, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn flag_read_params_matches() {
        same(
            ssx3_ee::s11::f_0034ac88__generated,
            flag_read_params,
            200,
            &|c, seed| {
                let mut r = Rng(0x34ac88 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let dst = DATA + 0x100;
                let src = DATA + 0x300;
                c.g[A0][0] = dst as u64;
                c.g[A1][0] = src as u64;
                for i in 0..0x68u32 / 4 {
                    c.bus.write32(src + 4 * i, r.next_u32());
                }
                let hz = DATA + 0x500;
                c.bus.write32(GAME_GP + 0x2a74, hz);
                let rate = if seed % 5 == 0 {
                    0
                } else {
                    1 + (r.next_u32() % 240) as i32
                };
                c.bus.write32(hz + 0x10, rate as u32);
            },
        );
    }
}
