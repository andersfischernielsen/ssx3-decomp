//! `src/ui/uistatestack.cpp`: hand-written replacements (moved from the
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
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::call::sp;
use crate::call::Flow;
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::Unwound;
use ssx3_ee_rt::vu0::X;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
pub(crate) use crate::game::render::particle::this_with_delta;
pub(crate) use crate::game::main::gamestate::p_2 as p;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::main::gamestate::vslot;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::sra32;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_2;
pub(crate) use crate::game::animation::rideranimbase::sw as sw_2;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_f238, ui_state_mgr_add_background_state, "UIStateMgr_AddBackgroundState"),
    (0x0039_f320, uistatemgr_switch_state_immediate, "UIStateMgr_SwitchStateImmediate"),
    (0x0039_f400, ui_state_mgr_switch_state, "UIStateMgr_SwitchState"),
    (0x0039_f4c0, ui_state_mgr_switch_state_close_popups, "UIStateMgr_SwitchStateClosePopups"),
    (0x0039_f600, ui_state_mgr_close_unpinned_popups, "UIStateMgr_CloseUnpinnedPopups"),
    (0x0039_f698, uistatemgr_close_all_popups, "UIStateMgr_CloseAllPopups"),
    (0x0039_f718, ui_state_mgr_close_top_state_and_popups, "UIStateMgr_CloseTopStateAndPopups"),
    (0x0039_fae8, ui_list_first_wrapper, "UIList_First_Wrapper"),
    (0x0039_fc48, uiobject_dtor, "UIObject_dtor"),
    (0x0039_feb8, uiobject_apply_alignment, "UIObject_ApplyAlignment"),
    (0x0039_ff60, uiobject_find_by_id, "UIObject_FindById"),
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
    pub(crate) use crate::game::render::particle::callee::INI_PARSER_SKIP_TO_EOL;
    pub(crate) use crate::game::render::particle::callee::BANK_MONITOR_INSTANCE_PAUSE;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_STREAM_HANDLE;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_MS_TO_NEXT_BEAT;
    pub(crate) use crate::game::render::particle::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_DRAW_ROW_2 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::render::particle::callee::CUT_SCENE_TRANSITION_SAVE_VIEWPORT;
    pub(crate) use crate::game::render::particle::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::render::particle::callee::AVA_SPLINE_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::POSITION_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::render::particle::callee::ANIM_OBJECT_BASE_DTOR;
    pub(crate) use crate::game::render::particle::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::render::particle::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_RESET;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_PUSH_BACK_2;
    pub(crate) use crate::game::render::particle::callee::CPS2_CHUNK_RESOLVE_DTOR_STUB;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_SETTLE_SLEW;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_FINISH_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_FINISH_BOARD_PRESS;
    pub(crate) use crate::game::render::particle::callee::ALGORITHM_LIST_DTOR;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::render::particle::callee::CFE_STATE_CHAR_SELECT_UPDATE_ATTRIBUTE_BARS;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_COPY;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_CONCAT_IN_PLACE;
    pub(crate) use crate::game::render::particle::callee::CROWD_MAN3D_GROUP_REMOVE_SECTION;
    pub(crate) use crate::game::render::particle::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::render::particle::callee::FLEX_BRIDGE_SPRINGS_SAVE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_COLLECTIBLE_COLLECTED;
    pub(crate) use crate::game::render::particle::callee::BENEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::render::particle::callee::BENEW_RACE_INT_IS_STATION_TRACK_BY_INDEX;
    pub(crate) use crate::game::render::particle::callee::SND_LOCK;
    pub(crate) use crate::game::render::particle::callee::SND_UNLOCK;
    pub(crate) use crate::game::render::particle::callee::SND_SPU_MEM_SET_BASE;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_POP;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_SELECT;
    pub(crate) use crate::game::render::particle::callee::SNDSTRM_SET_PITCH;
    pub(crate) use crate::game::render::particle::callee::PATH_SET_PAUSED;
    pub(crate) use crate::game::render::particle::callee::PATH_POST_EVENT;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_SEEK;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_READ;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_WRITE;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_MKDIR;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_DELETE;
    pub(crate) use crate::game::render::particle::callee::ISIGNAL_SEMA;
    pub(crate) use crate::game::render::particle::callee::STRLEN;
    pub(crate) use crate::game::render::particle::callee::WORLD_CREATE_DEAD_NODE;
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
    pub(crate) use crate::game::main::gamestate::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamestate::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INTERFACE_3 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::main::gamestate::callee::BE_ECON_INT_GET_TOTAL_EARNED;
    pub(crate) use crate::game::main::gamestate::callee::BOLTS_FIND_BOLT_INDEX;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_CMD_QUEUE_AT_2 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SECTION_ALLOCATOR_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2 as UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_STOP_3 as SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_2 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
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
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_PREV;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_PAUSE_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_RESUME_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::GAME_ANIM_MAP_UNREGISTER_BANK;
    pub(crate) use crate::game::main::gamestate::callee::ANIM_DATA_BASE_FREE_CHAN_CACHES;
    pub(crate) use crate::game::main::gamestate::callee::ADF_DATA_SOURCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::C_KEYBOARD_POPUP_SET_KEY_ENABLED;
    pub(crate) use crate::game::main::gamestate::callee::RIDER_CLEAR_NEARBY_LIGHTS;
    pub(crate) use crate::game::main::gamestate::callee::SNDSTRM_CLOSE;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4 as EA_MEM_FREE;
    pub(crate) use crate::game::main::gamestate::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_3;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_FIRST_2;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamestate::callee::C_GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_ALLOC_NAMED_3 as EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::main::gamestate::callee::MEM_MOVE;
    pub(crate) use crate::game::main::gamestate::callee::STR_COPY;
    pub(crate) use crate::game::main::gamestate::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::main::gamestate::callee::STR_N_COPY;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_SPHERE_TREE;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_AABB;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_MAKE_ID;
    pub(crate) use crate::game::main::gamestate::callee::MEM_FREE;
    pub(crate) use crate::game::main::gamestate::callee::STR_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_4 as EA_MEM_FREE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_AGREEMENT_TERMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_PICK_RANDOM_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_COUNT_SAVE_SNAPSHOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_CHANNEL_GET_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2 as ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_CONFIGURE_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_UPDATE_EMITTER_ENABLES;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE_5 as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2 as BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_CHAR_PROFILE_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_IS_FLAG4;
    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3 as SCRIPT_LIST_IS_MOVIE_ENTRY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2 as SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6 as UI_LIST_FIRST_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_DRAW_RECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COPY_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2 as AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2 as AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2 as MEMCPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCROLL_SET_PARTS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_3 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_GET_OBJECT_BY_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_DESTROY_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_FLUSH_ROSTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_API_DESTROY;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_HIDE_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_ITEM_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_BASE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_PHASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_SOUND_GET_TRIGGER_IDS;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2 as MC_MANAGER_GET;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_RESET_ONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_IS_LOADING;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_FINALIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SUSPEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SET_RUNNING;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;
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
// UI object

/// `void UIObject_ApplyAlignment(UIObject *obj, Vec2 *dst, const Vec2 *size)`
/// (0x39feb8): apply the object's alignment flags (bits 13..18 of the word at
/// +0x14) to adjust `dst` against the size.
pub fn uiobject_apply_alignment(c: &mut Cpu) {
    c.cycles += 6;
    let obj = c.g[A0][0] as u32;
    let a1 = c.g[A1][0] as u32;
    let a2 = c.g[A2][0] as u32;
    c.g[V0][0] = Addr(obj).off(0x14).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 13);
    c.g[A0][0] = c.g[V0][0] & 0x3f;
    c.g[V1][0] = c.g[A0][0] & 0x1;
    let centered = c.g[V1][0] != 0;
    c.g[V0][0] = c.g[A0][0] & 0x8;
    if !centered {
        c.cycles += 3;
        c.g[V0][0] = c.g[A0][0] & 0x4;
        if c.g[V0][0] == 0 {
            c.f[0] = Addr(a2).off(0x4).read_fpu(c).0;
            c.cycles += 6;
            c.g[AT][0] = 0x3f00_0000;
            c.f[2] = c.g[1][0] as u32;
            c.f[1] = Addr(a1).off(0x4).read_fpu(c).0;
            c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[2])).0;
            c.f[1] = (FpuF32(c.f[1]) - FpuF32(c.f[0])).0;
            Addr(a1).off(0x4).write_fpu(c, FpuF32(c.f[1]));
        } else {
            c.cycles += 5;
            c.f[1] = Addr(a2).off(0x4).read_fpu(c).0;
            c.f[0] = Addr(a1).off(0x4).read_fpu(c).0;
            c.f[0] = (FpuF32(c.f[0]) - FpuF32(c.f[1])).0;
            Addr(a1).off(0x4).write_fpu(c, FpuF32(c.f[0]));
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0] & 0x8;
    }
    c.cycles += 2;
    let skip_horizontal = c.g[V0][0] != 0;
    c.g[V0][0] = c.g[A0][0] & 0x20;
    if !skip_horizontal {
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.f[0] = Addr(a2).read_fpu(c).0;
            c.cycles += 6;
            c.g[AT][0] = 0x3f00_0000;
            c.f[2] = c.g[1][0] as u32;
            c.f[1] = Addr(a1).read_fpu(c).0;
            c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[2])).0;
            c.f[1] = (FpuF32(c.f[1]) - FpuF32(c.f[0])).0;
            Addr(a1).write_fpu(c, FpuF32(c.f[1]));
        } else {
            c.cycles += 5;
            c.f[1] = Addr(a2).read_fpu(c).0;
            c.f[0] = Addr(a1).read_fpu(c).0;
            c.f[0] = (FpuF32(c.f[0]) - FpuF32(c.f[1])).0;
            Addr(a1).write_fpu(c, FpuF32(c.f[0]));
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `UIObject *UIObject_FindById(UIObject *obj, int id)` (0x39ff60): `obj` when
/// its id (+0x38) equals `id`, else null.
pub fn uiobject_find_by_id(c: &mut Cpu) {
    c.cycles += 5;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = this.off(0x38).lw(c);
    c.g[V0][0] = 0;
    c.g[V1][0] = c.g[V1][0] ^ c.g[A1][0];
    if c.g[V1][0] == 0 {
        c.g[V0][0] = c.g[A0][0];
    }
    abi::ret(c);
}


/// `void *UIList_First_Wrapper(cList *list)` (0x0039fae8): the list's first
/// node.
pub fn ui_list_first_wrapper(c: &mut Cpu) {
    forward(c, 7, 0x10, |_| {}, callee::UI_LIST_FIRST, 0x0039_faf8)
}


/// `void UIStateMgr_AddBackgroundState(UIStateMgr* self, State* state)`
/// (0x39f238): mark the state as a background state and append it to the
/// background list.
pub fn ui_state_mgr_add_background_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(0x1c).lw(c);
        c.g[V1][0] = Addr(r32(c, S0)).off(0x8).lw(c);
        c.g[V0][0] |= 0x60;
        Addr(r32(c, S0)).off(0x1c).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x20).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x24).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0039_f270);
        c.g[A0][0] = this_with_delta(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, target, 0x0039_f270)?;
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0x1c));
        c.g[RA][0] = sx(0x0039_f27c);
        c.g[A1][0] = c.g[S0][0];
        callee::UI_LIST_PUSH_BACK_2.call(c, 0x0039_f27c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void UIStateMgr_CloseAllPopups()` (0x39f698): walk the popup list and
/// clear the "popping" bits (7, 8, and the 3-bit popup kind when it is below
/// 6) of every entry; returns the last node.
pub fn uistatemgr_close_all_popups(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::UI_LIST_FIRST_2.call(c, 0x0039_f6a8)?;
        let node = c.g[V0][0];
        c.g[A0][0] = node;
        let mut v0 = 0u64;
        if node != 0 {
            c.cycles += 4;
            c.g[V0][0] = lw_at(c, p(node as u32, 0x1c));
            c.g[V0][0] &= 1;
            c.g[A2][0] = 0xffff_ffff_ffff_c0ff;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A1][0] = 0xffff_ffff_ffff_ff7f;
                c.g[V1][0] = lw_at(c, p(node as u32, 0x1c));
                loop {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    c.g[V0][0] &= 0x3f;
                    c.g[V0][0] = (c.g[V0][0] < 6) as u64;
                    let small = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[V1][0] & c.g[A2][0];
                    if !small {
                        c.cycles += 4;
                        c.g[V0][0] |= 0x700;
                        c.g[V0][0] &= c.g[A1][0];
                        c.g[V0][0] |= 0x80;
                        c.sw(p(c.g[A0][0] as u32, 0x1c), c.g[V0][0]);
                    }
                    c.cycles += 5;
                    c.g[A0][0] = lw_at(c, p(c.g[A0][0] as u32, 4));
                    c.g[V0][0] = lw_at(c, p(c.g[A0][0] as u32, 0x1c));
                    c.g[V0][0] &= 1;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[V1][0] = lw_at(c, p(c.g[A0][0] as u32, 0x1c));
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_f6d0);
                    }
                }
            }
            c.cycles += 1;
            v0 = c.g[A0][0];
        }
        c.g[V0][0] = v0;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `UIObject_dtor(self, flags)` (0x39fc48): destroy the child object, then
/// the node body, and free the object when `flags & 1`.
pub fn uiobject_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x490000;
        frame.save(c, 16, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4bd8));
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(p(r32(c, S0), 8), c.g[V0][0]);
        c.g[A2][0] = lw_at(c, p(r32(c, S0), 0xc));
        let no_child = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_child {
            c.cycles += 6;
            c.g[V0][0] = lw_at(c, p(c.g[A2][0] as u32, 0x10));
            c.g[A1][0] = 3;
            let func = vslot(c, c.g[V0][0], 8, V1);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, func, 0x0039_fc8c)?;
        }
        c.cycles += 8;
        c.g[V0][0] = 0x490000;
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4cc0));
        c.sw(p(r32(c, S0), 8), c.g[V0][0]);
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x0039_fca0)?;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::BUILTIN_DELETE_3.addr, 0x0039_fcb4)?;
        }
        frame.restore(c, 16, 0x20);
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `UIStateMgr_CloseUnpinnedPopups` (0x39f600): walk the UI list and close
/// every unpinned popup (clearing its pin state bits).
pub fn ui_state_mgr_close_unpinned_popups(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0039_f610);
        callee::UI_LIST_FIRST_3.call(c, 0x0039_f610)?;
        c.g[A0][0] = c.g[V0][0];
        let empty = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 4;
            c.g[V1][0] = Addr(p(r(c, A0), 0x1c)).lw(c);
            c.g[V0][0] = c.g[V1][0] & 1;
            let t = c.g[V0][0] == 0;
            c.g[V0][0] = sra32(c.g[V1][0], 15);
            let mut direct = false;
            if !t {
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] & 1;
                let pinned = c.g[V0][0] != 0;
                c.g[V0][0] = c.g[A0][0];
                if pinned {
                    direct = true;
                } else {
                    c.cycles += 4;
                    c.g[A2][0] = 0xffff_ffff_ffff_c0ff;
                    c.g[A1][0] = 0xffff_ffff_ffff_ff7f;
                    c.g[V1][0] = Addr(p(r(c, A0), 0x1c)).lw(c);
                    c.g[V0][0] = sx(r(c, V1) >> 8);
                    loop {
                        c.cycles += 4;
                        c.g[V0][0] = c.g[V0][0] & 0x3f;
                        c.g[V0][0] = (c.g[V0][0] < 6) as u64;
                        let zero = c.g[V0][0] == 0;
                        c.g[V0][0] = c.g[V1][0] & c.g[A2][0];
                        if !zero {
                            c.cycles += 4;
                            c.g[V0][0] = c.g[V0][0] | 0x700;
                            c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                            c.g[V0][0] = c.g[V0][0] | 0x80;
                            Addr(p(r(c, A0), 0x1c)).sw(c, c.g[V0][0]);
                        }
                        c.cycles += 5;
                        c.g[A0][0] = Addr(p(r(c, A0), 4)).lw(c);
                        c.g[V1][0] = Addr(p(r(c, A0), 0x1c)).lw(c);
                        c.g[V0][0] = c.g[V1][0] & 1;
                        let t = c.g[V0][0] == 0;
                        c.g[V0][0] = sra32(c.g[V1][0], 15);
                        if t {
                            break;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.g[V0][0] & 1;
                        let cont = c.g[V0][0] == 0;
                        c.g[V0][0] = sx(r(c, V1) >> 8);
                        if !cont {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0039_f648);
                        }
                    }
                }
            }
            if !direct {
                c.cycles += 1;
                c.g[V0][0] = c.g[A0][0];
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `UIListNode* UIStateMgr_CloseTopStateAndPopups(list)` (0x39f718): for the
/// list's first node, clear the "visible" bit on each id<6 node until the
/// sentinel and return the sentinel's next.
pub fn ui_state_mgr_close_top_state_and_popups(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, table::UI_LIST_FIRST, 0x0039_f728)?;
        let first = c.g[V0][0];
        c.g[V1][0] = first;
        let empty = c.g[V1][0] == 0;
        c.g[V0][0] = 0;
        if empty {
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1c).lw(c);
        c.g[V0][0] &= 1;
        let clean = c.g[V0][0] == 0;
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1c).lw(c);
        if !clean {
            c.cycles += 3;
            c.g[A2][0] = 0xffff_ffff_ffff_c0ff;
            c.g[A1][0] = 0xffff_ffff_ffff_ff7f;
            'nodes: loop {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[A0][0] as u32) >> 8);
                c.g[V0][0] &= 0x3f;
                c.g[V0][0] = (c.g[V0][0] < 6) as u64;
                let small = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[A0][0] & c.g[A2][0];
                if !small {
                    c.cycles += 4;
                    c.g[V0][0] |= 0x700;
                    c.g[V0][0] &= c.g[A1][0];
                    c.g[V0][0] |= 0x80;
                    Addr(c.g[V1][0] as u32).off(0x1c).sw(c, c.g[V0][0]);
                }
                c.cycles += 5;
                c.g[V1][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1c).lw(c);
                c.g[V0][0] &= 1;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1c).lw(c);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_f750);
                    }
                    continue 'nodes;
                }
                break 'nodes;
            }
        }
        c.cycles += 5;
        c.g[V0][0] = sx((c.g[A0][0] as u32) >> 8);
        c.g[V0][0] &= 0x3f;
        c.g[V0][0] = (c.g[V0][0] < 6) as u64;
        let small = c.g[V0][0] == 0;
        c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
        if !small {
            c.cycles += 3;
            c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
            c.g[V0][0] |= 0x780;
            Addr(c.g[V1][0] as u32).off(0x1c).sw(c, c.g[V0][0]);
        }
        c.cycles += 1;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// `UIStateMgr_SwitchState` (0x0039f400).
//
// `void UIStateMgr::SwitchState(UIStateMgr* m, UIListNode* n)`: push the new
// state node in front of the current one and patch its visibility bits.

pub fn ui_state_mgr_switch_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 16, 0x30);
        let mgr = c.g[A0][0];
        frame.save(c, 18, 0x10);
        frame.save_ra(c, 0);
        let node = c.g[A1][0];
        c.g[S1][0] = mgr;
        c.g[S0][0] = node;
        call_ptr(c, 0x0039_7840, 0x0039_f420)?;
        c.g[A0][0] = c.g[V0][0];
        let a0 = c.g[A0][0];
        c.g[S2][0] = 0;
        let mut v0;
        if a0 == 0 {
            v0 = c.g[V0][0];
        } else {
            c.cycles += 5;
            let v1 = lw_2(c, a0 as u32, 0x1c);
            c.g[V1][0] = v1;
            c.g[V0][0] = sx((((v1 as u32) as i32) >> 5) as u32);
            c.g[V0][0] = c.g[V0][0] & 1;
            let clear5 = c.g[V0][0] == 0;
            c.g[V0][0] = sx((((v1 as u32) as i32) >> 6) as u32);
            if clear5 {
                v0 = c.g[V0][0];
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] & 1;
                let set6 = c.g[V0][0] != 0;
                c.g[V0][0] = sx((v1 as u32) >> 8);
                if set6 {
                    v0 = c.g[V0][0];
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.g[V0][0] & 0x3f;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 6) as u64;
                    let big = c.g[V0][0] == 0;
                    c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
                    if big {
                        v0 = c.g[V0][0];
                    } else {
                        c.cycles += 4;
                        c.g[S2][0] = 1;
                        c.g[V0][0] = v1 & c.g[V0][0];
                        c.g[V0][0] = c.g[V0][0] | 0x680;
                        sw_2(c, a0 as u32, 0x1c, c.g[V0][0]);
                        v0 = c.g[V0][0];
                    }
                }
            }
        }
        // b_39f48c / b_39f498.
        c.cycles += 2;
        let s2 = c.g[S2][0] == 0;
        c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
        if s2 {
            c.cycles += 3;
            c.g[V0][0] = lw_2(c, node as u32, 0x1c);
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x220;
        } else {
            c.cycles += 6;
            c.g[V0][0] = lw_2(c, node as u32, 0x1c);
            c.g[A0][0] = 0xffff_ffff_ffff_ffdf;
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x100;
            c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        }
        let _ = v0;
        c.cycles += 10;
        sw_2(c, node as u32, 0x1c, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0039_78f0, 0x0039_f4a8)?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// UI state stack.

/// `void UIStateMgr_SwitchStateImmediate(mgr, st, a2, a3)` (0x39f320): close
/// the current state when it is a popup of id 5, then push `st` and mark its
/// flags.
pub fn uistatemgr_switch_state_immediate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A2][0];
        call_ptr(c, 0x0039_fa88, 0x0039_f34c)?;
        c.g[A1][0] = c.g[V0][0];
        let mut plus_one = false;
        if c.g[A1][0] == 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x1c)).lw(c);
            c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
            c.g[S3][0] = 1;
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x200;
            Addr(p(r32(c, S0), 0x1c)).sw(c, c.g[V0][0]);
            plus_one = true;
        } else {
            c.cycles += 5;
            c.g[A0][0] = Addr(p(r32(c, A1), 0x1c)).lw(c);
            c.g[V0][0] = sx(((r32(c, A0) as i32) >> 5) as u32);
            c.g[V0][0] = c.g[V0][0] & 1;
            let not_popup = c.g[V0][0] == 0;
            c.g[V0][0] = sx(r32(c, A0) >> 8);
            if not_popup {
                plus_one = true;
            } else {
                c.cycles += 4;
                c.g[V1][0] = 5;
                c.g[V0][0] = c.g[V0][0] & 0x3f;
                if c.g[V0][0] != c.g[V1][0] {
                    // not the buy popup: nothing to close
                } else {
                    c.cycles += 10;
                    c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
                    c.g[V1][0] = c.g[S1][0] & 1;
                    c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
                    c.g[V1][0] = sx(r32(c, V1) << 7);
                    c.g[A0][0] = 0xffff_ffff_ffff_ff7f;
                    c.g[V0][0] = c.g[V0][0] | 0x700;
                    c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    Addr(p(r32(c, A1), 0x1c)).sw(c, c.g[V0][0]);
                    plus_one = true;
                }
            }
        }
        if plus_one {
            c.cycles += 1;
        }
        c.g[A0][0] = c.g[S2][0];
        c.cycles += 16;
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0039_78f0, 0x0039_f3c4)?;
        c.g[V0][0] = Addr(p(r32(c, S0), 0x1c)).lw(c);
        c.g[V1][0] = c.g[S3][0] & 1;
        c.g[A0][0] = 0xffff_ffff_ffff_ffdf;
        c.g[V1][0] = sx(r32(c, V1) << 5);
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        frame.restore(c, S1, 0x30);
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        frame.restore(c, S2, 0x20);
        Addr(p(r32(c, S0), 0x1c)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `UIStateMgr_SwitchStateClosePopups` (0x0039f4c0): close the popup widgets
/// of a UI state switch.  Starting from the first widget of `a0`'s list
/// (`UIList_First`), every widget still marked visible (bit 0 of `+0x1c`)
/// is walked: its `0x3f`-field is raised to `0x7` when below 6, bit 14 is
/// set, and the loop stops at the first hidden one; finally the transition
/// flags at `a1+0x1c` are updated (bit 8 with bit 5 clear, or `0x220` if a
/// visible widget was seen) and the widget is pushed to the front of `a0`'s
/// list.
pub fn ui_state_mgr_switch_state_close_popups(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0039_f4e8);
        c.g[S3][0] = 0u64;
        call_ptr(c, 0x0039_7840, 0x0039_f4e8)?;
        c.g[A0][0] = c.g[V0][0];
        let no_list = c.g[A0][0] == 0;
        c.g[S0][0] = 0u64;
        'outer: {
            if no_list {
                break 'outer;
            }
            'walk: {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = c.g[V0][0] & 0x1;
                let hidden = c.g[V0][0] == 0;
                if hidden {
                    break 'walk;
                }
                c.cycles += 3;
                c.g[A3][0] = 0xffff_ffff_ffff_c0ff;
                c.g[A2][0] = 0xffff_ffff_ffff_ff7f;
                c.g[A1][0] = 0xffff_ffff_ffff_bfff;
                'lv: loop {
                    'level: {
                        c.cycles += 6;
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                        c.g[V0][0] = c.g[V0][0] & 0x3f;
                        c.g[V0][0] = (c.g[V0][0] < 0x6u64) as u64;
                        let low = c.g[V0][0] == 0;
                        c.g[V0][0] = c.g[V1][0] & c.g[A3][0];
                        if !low {
                            c.cycles += 4;
                            c.g[V0][0] = c.g[V0][0] | 0x700;
                            c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                            c.g[V0][0] = c.g[V0][0] | 0x80;
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                        }
                        c.cycles += 9;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                        c.g[V0][0] = c.g[V0][0] | 0x4000;
                        c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = c.g[V0][0] & 0x1;
                        let more = c.g[V0][0] != 0;
                        c.g[S0][0] = 0x1;
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x0039_f510);
                            }
                            continue 'lv;
                        }
                        break 'level;
                    }
                    break 'walk;
                }
            }
            'mark: {
                c.cycles += 2;
                let seen = c.g[S0][0] != 0;
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                if seen {
                    break 'mark;
                }
                c.cycles += 4;
                c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
                c.g[V0][0] = c.g[V0][0] & 0x1;
                let clear = c.g[V0][0] == 0;
                if clear {
                    break 'outer;
                }
            }
            c.cycles += 4;
            c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 6) as u32);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            let set = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            if set {
                break 'outer;
            }
            c.cycles += 4;
            c.g[V0][0] = c.g[V0][0] & 0x3f;
            c.g[V0][0] = ((c.g[V0][0] as i64) < 6i64) as u64;
            let low = c.g[V0][0] == 0;
            c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
            if low {
                break 'outer;
            }
            c.cycles += 4;
            c.g[S3][0] = 0x1;
            c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
            c.g[V0][0] = c.g[V0][0] | 0x680;
            c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        }
        'flags: {
            'plain: {
                c.cycles += 2;
                let no_seen = c.g[S3][0] == 0;
                c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
                if no_seen {
                    break 'plain;
                }
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                c.g[A0][0] = 0xffff_ffff_ffff_ffdf;
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] | 0x100;
                c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                break 'flags;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x220;
            break 'flags;
        }
        c.cycles += 11;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0039_f5e0);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_78f0, 0x0039_f5e0)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
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
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::put_words;
    use crate::difftest::fakes::*;
    use crate::difftest::fakes::vtable;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::SP as STACK;
    use crate::difftest::fakes;
    use crate::difftest::put_vec;
    use crate::difftest::elf_image;
    use crate::difftest::SP;
    use ssx3_ee as ee;
    use crate::difftest::seed_count;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::clear;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::render::particle::tests::vtable_2;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base;
    pub(crate) use crate::game::main::gamestate::tests::slot_2 as slot;
    pub(crate) use crate::game::be::beintplayer::tests::r_u32;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_3;
    pub(crate) use crate::game::sound::soundsys::tests::BE as BE_2;
    pub(crate) use crate::game::sound::soundsys::tests::WORLD;
    pub(crate) use crate::game::ai::rider::tests::clear_2;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7 as globals_2;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool_2;
    pub(crate) use crate::game::sound::soundsys::tests::load_2 as load;
    pub(crate) use crate::game::sound::soundsys::tests::alloc_ready;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_4;
    pub(crate) use crate::game::world::worldcache::tests::check_all;


    #[test]
    fn uiobject_apply_alignment_matches() {
        check(ssx3_ee::s13::f_0039feb8, uiobject_apply_alignment, 300, &|c, seed| {
            let mut r = Rng(0x39fe_b800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.g[A2][0] = (DATA + 0x200) as u64;
            c.bus.write32(DATA + 0x14, (r.next_u32() & 0x3f) << 13);
            c.bus.write32(DATA + 0x100, r.edge_f32());
            c.bus.write32(DATA + 0x104, r.edge_f32());
            c.bus.write32(DATA + 0x200, r.edge_f32());
            c.bus.write32(DATA + 0x204, r.edge_f32());
        });
    }


    #[test]
    fn uiobject_find_by_id_matches() {
        check(ssx3_ee::s13::f_0039ff60, uiobject_find_by_id, 300, &|c, seed| {
            let mut r = Rng(0x39ff_6000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA as u64) | ((r.next_u32() as u64) << 32);
            let id = r.next_u32();
            c.g[A1][0] = if seed % 3 == 0 { id as u64 } else { (id as u64) | ((r.next_u32() as u64) << 32) };
            c.bus.write32(DATA + 0x38, id);
        });
    }


    #[test]
    fn ui_list_first_wrapper_matches() {
        check(ee::s13::f_0039fae8__generated, ui_list_first_wrapper, 200, &|c, seed| {
            machine(c, seed, 0x39_fae8, 0);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            match seed % 3 {
                0 => c.bus.write32(DATA + 0x1000 + 4, 0),
                1 => {
                    let node = DATA + 0x800;
                    c.bus.write32(DATA + 0x1000 + 4, node);
                    c.bus.write32(node + 4, node);
                }
                _ => {
                    let node = DATA + 0x800;
                    let last = DATA + 0x900;
                    c.bus.write32(DATA + 0x1000 + 4, node);
                    c.bus.write32(node + 4, last);
                }
            }
        });
    }


    #[test]
    fn ui_state_mgr_add_background_state_matches() {
        check(ee::s13::f_0039f238__generated, ui_state_mgr_add_background_state, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x39f238, 2);
            let this = DATA + 0x100;
            let state = DATA + 0x200;
            let vt = DATA + 0x300;
            clear(c, this, 0x100);
            clear(c, state, 0x100);
            clear(c, vt, 0x200);
            c.bus.write32(state + 8, vt);
            vtable_2(c, vt, &[(0x20, 0, FAKE)]);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(state);
        });
    }


    #[test]
    fn uistatemgr_close_all_popups_matches() {
        check(
            ee::s13::f_0039f698__generated,
            uistatemgr_close_all_popups,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0xf698, 0);
                let list = DATA + 0x800;
                let n1 = DATA + 0x840;
                let n2 = DATA + 0x880;
                c.g[A0][0] = sx(list);
                c.bus.write32(list + 4, if seed % 5 == 0 { 0 } else { n1 });
                c.bus.write32(n1 + 4, if seed % 2 == 0 { 0 } else { n2 });
                c.bus.write32(n1 + 0x1c, ((seed % 8) << 8) | 1);
                c.bus.write32(n2 + 4, n2);
                // Even: the walk stops after this node.
                c.bus.write32(n2 + 0x1c, (r_u32(seed, 7) << 1) & !1);
            },
        );
    }


    #[test]
    fn uiobject_dtor_matches() {
        covered(ee::s13::f_0039fc48__generated, uiobject_dtor, SEEDS, &|c, seed| {
            base(c, seed, 0xfc48, 1);
            let s = DATA + 0x100;
            let child = DATA + 0x200;
            c.g[A0][0] = sx(s);
            c.g[A1][0] = (seed & 1) as u64;
            c.bus
                .write32(s + 0xc, if seed % 3 == 0 { 0 } else { child });
            let vt = DATA + 0x300;
            slot(c, vt, 8, 0, FAKE);
            c.bus.write32(child + 0x10, vt);
        });
    }


    #[test]
    fn ui_state_mgr_close_unpinned_popups_matches() {
        check(
            ee::s13::f_0039f600__generated,
            ui_state_mgr_close_unpinned_popups,
            SEEDS_2,
            &|c, seed| {
                let _ = base_2(c, seed, 0xf600, 2);
                let head = DATA;
                let entry = DATA + 0x800;
                w(c, head + 4, if seed & 1 == 0 { head } else { entry });
                w(c, entry + 4, head);
                w(c, head + 0x1c, 0);
                let flags = ((seed & 1) as u32) | ((seed & 2) << 14);
                w(c, entry + 0x1c, flags);
                c.g[A0][0] = head as u64;
            },
        );
    }


        #[test]
    fn ui_state_mgr_close_top_state_and_popups_matches() {
        check(ssx3_ee::s13::f_0039f718__generated, ui_state_mgr_close_top_state_and_popups, 30, &|c, seed| {
            let _r = base_3(c, seed, 0x39f718, 1);
            let list = DATA + 0x1000;
            c.g[A0][0] = list as u64;
            let sentinel = DATA + 0x5000;
            w_2(c, sentinel, sentinel);
            w_2(c, sentinel + 4, sentinel);
            if seed % 4 == 0 {
                w_2(c, list + 4, sentinel);
            } else {
                let n0 = DATA + 0x2000;
                let n1 = DATA + 0x3000;
                w_2(c, list + 4, n0);
                w_2(c, n0, sentinel);
                w_2(c, n0 + 4, if seed % 2 == 0 { n1 } else { sentinel });
                w_2(c, n0 + 0x1c, (seed % 2) | ((seed % 5) << 8));
                w_2(c, n1, sentinel);
                w_2(c, n1 + 4, sentinel);
                w_2(c, n1 + 0x1c, ((seed / 2) % 5) << 8);
            }
        });
    }


                    #[test]
    fn ui_state_mgr_switch_state_matches() {
        check(ee::s13::f_0039f400__generated, ui_state_mgr_switch_state, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x39f4, 4);
            let mgr = DATA;
            let node = DATA + 0x1000;
            c.bus.write32(mgr, mgr);
            c.bus.write32(mgr + 4, node);
            c.bus.write32(node + 0x1c, r.next_u32());
            c.bus.write32(node + 4, mgr);
            c.g[A0][0] = sx(mgr);
            c.g[A1][0] = sx(node);
        });
    }


    #[test]
    fn uistatemgr_switch_state_immediate_matches() {
        check(
            ee::s13::f_0039f320__generated,
            uistatemgr_switch_state_immediate,
            8,
            &|c, seed| {
                let _ = base_4(c, seed, 0x39f320, 2);
                let mgr = DATA;
                let st = DATA + 0x200;
                clear_2(c, mgr, 0x200);
                clear_2(c, st, 0x200);
                c.g[A0][0] = mgr as u64;
                c.g[A1][0] = st as u64;
                c.g[A2][0] = (seed % 2) as u64;
                c.g[A3][0] = (seed % 3) as u64;
            },
        );
    }


        #[test]
    fn ui_state_mgr_switch_state_close_popups_matches() {
        const S: u64 = 0x0039_f4c0;
        check_all(
            ee::s13::f_0039f4c0__generated,
            ui_state_mgr_switch_state_close_popups,
            40,
            &|c, seed| {
                machine(c, seed, S, 0);
                let this = DATA + 0x1000;
                let head = DATA + 0x2000;
                let node0 = DATA + 0x3000;
                let node1 = DATA + 0x4000;
                let trans = DATA + 0x5000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(trans);
                w(c, this + 4, head);
                w(c, head, head);
                if seed % 5 == 0 {
                    // An empty list: `UIList_First` returns 0.
                    w(c, head + 4, head);
                } else {
                    w(c, head + 4, node0);
                    w(c, node0 + 4, node1);
                    w(c, node1 + 4, 0);
                    let f0 = (seed as u32) << 4 | 1; // bit 0: visible
                    let f1 = (seed as u32) << 8; // bit 0 clear: hidden
                    w(c, node0 + 0x1c, f0);
                    w(c, node1 + 0x1c, f1);
                    w(c, node0 + 0x10, (seed % 3) as u32);
                    w(c, node1 + 0x10, (seed % 3) as u32);
                }
                let tf = match seed % 4 {
                    0 => 0,
                    1 => 0x20,
                    2 => 0x40,
                    _ => 0x20 | 0x40,
                };
                w(c, trans + 0x1c, tf | ((seed as u32) << 12));
            },
        );
    }
}
