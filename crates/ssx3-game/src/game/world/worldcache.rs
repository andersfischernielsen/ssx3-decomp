//! `src/world/worldcache.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use crate::call::sp;
use ssx3_ee_rt::funcs::call_indirect;
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call;
use crate::call::Unwound;
pub(crate) use crate::game::sound::bankmonitor::p;
pub(crate) use crate::game::sound::bankmonitor::add32;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::world::wscriptcache::DELETE_2 as DELETE;
pub(crate) use crate::game::animation::animsequencer::dtor;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_3;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_4;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::be::beintplayer::mul32;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_5;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw as lw_2;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_7598, world_block_pool_init, "WorldBlockPool_Init"),
    (0x003a_76c0, world_block_pool_dtor, "WorldBlockPool_dtor"),
    (0x003a_7768, world_block_pool_alloc_block, "WorldBlockPool_AllocBlock"),
    (0x003a_7790, world_block_pool_free_block, "WorldBlockPool_FreeBlock"),
    (0x003a_77b0, section_allocator_ctor, "SectionAllocator_ctor"),
    (0x003a_77d0, section_allocator_dtor, "SectionAllocator_dtor"),
    (0x003a_7a20, world_memman_ctor, "WorldMemman_ctor"),
    (0x003a_7aa8, world_memman_dtor, "WorldMemman_dtor"),
    (0x003a_7b18, worldmemman_create_section_allocator, "WorldMemman_CreateSectionAllocator"),
    (0x003a_7b98, world_memman_free_section_allocator, "WorldMemman_FreeSectionAllocator"),
    (0x003a_7bd8, c_hull_page_ctor, "cHullPage_ctor"),
    (0x003a_7c30, c_hull_page_dtor, "cHullPage_dtor"),
    (0x003a_7c88, section_table_ctor, "SectionTable_ctor"),
    (0x003a_7d80, section_table_dtor, "SectionTable_dtor"),
    (0x003a_7e38, world_cache_dtor, "cWorldCache_dtor"),
    (0x003a_7f90, c_world_cache_reset, "cWorldCache_Reset"),
    (0x003a_81a8, c_world_cache_create_section_table, "cWorldCache_CreateSectionTable"),
    (0x003a_82c8, c_world_cache_init_hull_slots, "cWorldCache_InitHullSlots"),
    (0x003a_8448, c_world_cache_request_hull, "cWorldCache_RequestHull"),
    (0x003a_84c8, c_world_cache_expire_hull, "cWorldCache_ExpireHull"),
    (0x003a_84f8, c_world_cache_set_hull_pinned, "cWorldCache_SetHullPinned"),
    (0x003a_8528, cworldcache_free_hull_page, "cWorldCache_FreeHullPage"),
    (0x003a_8618, c_world_cache_is_hull_resident, "cWorldCache_IsHullResident"),
    (0x003a_8638, c_world_cache_is_hull_free, "cWorldCache_IsHullFree"),
    (0x003a_8650, c_world_cache_set_hull_resident, "cWorldCache_SetHullResident"),
    (0x003a_8f10, c_world_cache_flag_section_objects, "cWorldCache_FlagSectionObjects"),
    (0x003a_8fb8, c_world_cache_streamer_dtor, "cWorldCache_StreamerDtor"),
    (0x003a_9188, cworldcache_streamer_reset, "cWorldCache_StreamerReset"),
    (0x003a_9558, cworld_cache_streamer_release_node_hulls, "cWorldCache_StreamerReleaseNodeHulls"),
    (0x003a_95c0, world_cache_streamer_are_node_hulls_free, "cWorldCache_StreamerAreNodeHullsFree"),
    (0x003a_9770, c_world_cache_streamer_get_view_clearance_all, "cWorldCache_StreamerGetViewClearanceAll"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::ai::ai::callee::INTC_SAVE_REGS;
    pub(crate) use crate::game::ai::ai::callee::INTC_RESTORE_REGS;
    pub(crate) use crate::game::ai::ai::callee::IRRADIANCE_FIND_INDEX;
    pub(crate) use crate::game::ai::ai::callee::IRRADIANCE_GET_ENTRY;
    pub(crate) use crate::game::ai::ai::callee::WORLD_CACHE_FREE;
    pub(crate) use crate::game::ai::ai::callee::MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::ai::ai::callee::MPEG2_SHOW_BITS;
    pub(crate) use crate::game::ai::ai::callee::MPEG2_FLUSH_BUFFER32;
    pub(crate) use crate::game::ai::ai::callee::AI_RESET_RIDERS;
    pub(crate) use crate::game::ai::ai::callee::AI_SET_RIDER_ORDER_MODE;
    pub(crate) use crate::game::ai::ai::callee::RIDER_FIND_NEARBY_LIGHTS;
    pub(crate) use crate::game::ai::ai::callee::BE_COPY_PROFILE_CHARACTERS;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_STATE_GOTO_NEXT_2 as GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PS2_MEMCARD_CREATE;
    pub(crate) use crate::game::sound::bankmonitor::callee::LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUDDY_MAN_CONNECT;
    pub(crate) use crate::game::sound::bankmonitor::callee::STR_CASE_CMP;
    pub(crate) use crate::game::sound::bankmonitor::callee::SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMCARD_REQUEST_GET_INFO_NOW;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_VEC3;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_FLOAT_2 as PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_LIST_ADD_CHOICE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_END_SCRIPT_MUSIC;
    pub(crate) use crate::game::sound::bankmonitor::callee::WORLD_PAINTER_SET_SCRIPT_SURFACE_MODE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_MEMORY_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::sound::bankmonitor::callee::SPEECH_SYSTEM_STOP_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BANK_MONITOR_TIMER_KILL;
    pub(crate) use crate::game::sound::bankmonitor::callee::SNDSTRM_SET_PREBUFFER;
    pub(crate) use crate::game::sound::bankmonitor::callee::AUDIO_RAND;
    pub(crate) use crate::game::sound::bankmonitor::callee::SONG_INSTANCE_GET_REQUEST_STATUS;
    pub(crate) use crate::game::sound::bankmonitor::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::sound::bankmonitor::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_MAN_CALL_TASK_FUNC;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_REMOVE;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_MISSION_INT_GET_COLLECTED_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::ADF_DATA_SOURCE_SET_DATA;
    pub(crate) use crate::game::sound::bankmonitor::callee::EA_FILE_LOAD_DECOMPRESSED;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_ANIM_MAP_FIND_HASH;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMSET_2 as MEMSET;
    pub(crate) use crate::game::sound::bankmonitor::callee::CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::sound::bankmonitor::callee::TEX_MAN_FREE_TEXTURE;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_MENU_SET_NAV_BUTTONS;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::sound::bankmonitor::callee::SECTION_ALLOCATOR_DTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::STREAM_FILE_RELEASE_CHUNK;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_GET_EVENT_RECORD_INDEX;
    pub(crate) use crate::game::sound::bankmonitor::callee::BOLTS_FREE_DB;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_SOUND_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_PAUSE;
    pub(crate) use crate::game::world::wscriptcache::callee::BANK_MONITOR_DELETE_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_NEW_RACE_INT_GET_TRACK_INFO;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP5;
    pub(crate) use crate::game::world::wscriptcache::callee::CROWD_AUDIO_FREE_SAMPLE_LIST;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_BASE_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_INIT;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_GET_COUNT;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_ADD_ITEMS;
    pub(crate) use crate::game::world::wscriptcache::callee::EA_MEM_FREE_2 as EA_MEM_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::MDF_FILE_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::world::wscriptcache::callee::INPUT_PREPROC_WRITE_STRING;
    pub(crate) use crate::game::world::wscriptcache::callee::CINSTANCE_NODE_CTOR_V;
    pub(crate) use crate::game::world::wscriptcache::callee::FONT_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::SECTION_ALLOCATOR_FREE_ALL;
    pub(crate) use crate::game::world::wscriptcache::callee::C_WORLD_CACHE_STREAMER_DTOR_STUB;
    pub(crate) use crate::game::world::wscriptcache::callee::RCMP_SYSTEM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::PS2_SONY_CODEC_INTERNAL_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_GET_BITS1;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_FLUSH_BUFFER_3 as MPEG2_FLUSH_BUFFER_2;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_SUB_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_MAIN_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::world::wscriptcache::callee::CHASE_CAMERA_ALGORITHM_CTOR_2 as CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_RESET_CHASE_CONTROLLER_SWITCHES;
    pub(crate) use crate::game::world::wscriptcache::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::world::wscriptcache::callee::DELETE;
    pub(crate) use crate::game::world::wscriptcache::callee::SND_BANK_MONITOR_MEM_SIZE;
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
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INTERFACE_3 as BE_LIBRARY_GET_INTERFACE_2;
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
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
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
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4 as EA_MEM_FREE_2;
    pub(crate) use crate::game::main::gamestate::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_FIRST_2 as UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamestate::callee::C_GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_ALLOC_NAMED_3 as EA_MEM_ALLOC_NAMED;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_3 as SCRIPT_CMD_QUEUE_AT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM_2 as UI_OBJECT_UPDATE_ANIM;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_9 as NET_FE_STATE_ON_CHILD_EVENT_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_CTOR as SCRIPT_BINDING_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_MEM_CARD_CLEAR_DIR_CACHE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST as UI_LIST_FIRST_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_2 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_HIDE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SHOW;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP_2 as NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_GET_CHARACTER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_FILL_PLAYER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_MESSAGE_CENTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_COUNT_2 as BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_ENTRY_2 as BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_DATA_GET_BANK_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3 as SCRIPT_ENGINE_GET_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_3 as SCRIPT_OBJ_ACTIVATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2 as UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3 as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_STREAMER_UPDATE_VIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SET_INPUT_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_SET_OWNER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_3 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_READ_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_FLAG_MANAGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_MANAGER_ADD_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY as UI_ANIM_APPLY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE as UI_ANIM_ADVANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2 as GAME_PROCESS_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2 as BE_GET_BE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_5 as SPRINTF_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::EDITOR_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_SELECT_ALGORITHM;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_FREE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_LOAD_BANK_FROM_MEMORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_MUSIC_TIER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_TRACK_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2 as SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2 as SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE_2 as SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COMPUTE_OFFSET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_AABB_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_PAD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_NODE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME_2 as UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBE_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3 as NET_FE_STATE_GET_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6 as UI_LIST_FIRST_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_DRAW_RECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COPY_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2 as AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2 as AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2 as MEMCPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCROLL_SET_PARTS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ADD_SCREEN_BY_HASH_NAME;
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

/// Cross-shard call targets (the generator emits `call_indirect` for them).
pub(crate) mod table {
    pub(crate) use crate::game::wscript::wscriptman::table::IS_REPLAY_PLAYING;
    pub(crate) use crate::game::wscript::wscriptman::table::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_BANK_REMOVE;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_MIDI_RESUME;
    pub(crate) use crate::game::wscript::wscriptman::table::SCE_MC_FLUSH;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_VEC_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::MEM_FREE;
    pub(crate) use crate::game::wscript::wscriptman::table::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::wscript::wscriptman::table::WSCRIPT_PROCESS_LIST_POP_FRONT;
    pub(crate) use crate::game::wscript::wscriptman::table::STR_HASH;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32_2;
    pub(crate) use crate::game::wscript::wscriptman::table::BXFILE_EXISTS;
    pub(crate) use crate::game::wscript::wscriptman::table::STREAM_FILE_DESTROY;
    pub(crate) use crate::game::wscript::wscriptman::table::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::ai::computer::table::DELETE;
    pub(crate) use crate::game::ai::computer::table::EA_MEM_FREE;
    pub(crate) use crate::game::ai::computer::table::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::ai::computer::table::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::ai::computer::table::WAIT_SEMA;
    pub(crate) use crate::game::ai::computer::table::WAKEUP_THREAD;
    pub(crate) use crate::game::ai::computer::table::SLEEP_THREAD;
    pub(crate) use crate::game::ai::computer::table::SSX_AUDIO_SYSTEM_GET_INSTANCE;
}


// ---------------------------------------------------------------------------
// World block pool

/// `void *WorldBlockPool_AllocBlock(WorldBlockPool *pool)` (0x3a7768): pop a
/// block off the free list at +0x10, mark it used and return it in v0.
pub fn world_block_pool_alloc_block(c: &mut Cpu) {
    c.cycles += 10;
    let pool = Addr(abi::arg(c, 0));
    c.g[A1][0] = pool.off(0x10).lw(c);
    c.g[A3][0] = 0x1;
    c.g[V1][0] = pool.off(0x8).lw(c);
    c.g[A2][0] = Addr(c.g[A1][0] as u32).off(0x14).lw(c);
    c.g[V0][0] = c.g[A1][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
    pool.off(0x8).sw(c, c.g[V1][0]);
    pool.off(0x10).sw(c, c.g[A2][0]);
    Addr(c.g[A1][0] as u32).off(0x10).sw(c, c.g[A3][0]);
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// World block pool / section allocator.

/// `void WorldBlockPool_FreeBlock(WorldBlockPool* pool, void* block)`
/// (0x3a7790): push `block` onto the pool's free list at +0x10, recording the
/// old head at block+0x14 and bumping the free count at pool+0x8.
pub fn world_block_pool_free_block(c: &mut Cpu) {
    c.cycles += 8;
    let pool = Addr(abi::arg(c, 0));
    let block = Addr(abi::arg(c, 1));
    block.off(0x10).sw(c, 0);
    c.g[V0][0] = pool.off(0x10).lw(c);
    block.off(0x14).sw(c, c.g[V0][0]);
    pool.off(0x10).sw(c, c.g[A1][0]);
    c.g[V0][0] = pool.off(0x8).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    pool.off(0x8).sw(c, c.g[V0][0]);
    abi::ret(c);
}


/// `SectionAllocator* SectionAllocator_ctor(SectionAllocator* self, void*
/// begin, void* end)` (0x3a77b0): initialise the allocator's first/current/
/// end pointers and clear its counters.
pub fn section_allocator_ctor(c: &mut Cpu) {
    c.cycles += 7;
    let this = Addr(abi::arg(c, 0));
    this.off(0x4).sw(c, c.g[A2][0]);
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, c.g[A1][0]);
    this.off(0x8).sw(c, 0);
    this.off(0xc).sw(c, 0);
    this.off(0x10).sw(c, 0);
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// World cache hulls.

/// `void cWorldCache_RequestHull(cWorldCache* self, int index)` (0x3a8448):
/// claim the hull slot `index` (0x18 bytes each, table at +0x3f0), fill in the
/// free-list linkage and set the slot's reference count to 1 (0 stays 0).
pub fn c_world_cache_request_hull(c: &mut Cpu) {
    c.cycles += 16;
    let self_ = abi::arg(c, 0);
    c.g[V0][0] = 0x18;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c);
    c.mult(0, A1, A1, V0);
    c.g[A3][0] = sx(self_.wrapping_add(0x3f0));
    c.g[A2][0] = Addr(c.g[V1][0] as u32).off(0x1c).lw(c);
    c.g[V0][0] = sx(self_.wrapping_add(c.g[A1][0] as u32));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x400), c.g[A2][0]);
    c.g[V1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[V0][0];
    c.swc1(12, (c.g[V1][0] as u32).wrapping_add(0x3fc));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x3f4), 0);
    c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
    c.g[V0][0] = ((c.g[V1][0] as i64) < 4) as u64;
    if c.g[V0][0] == 0 {
        // Slot already had a count >= 4: leave it alone unless exactly 4.
        c.g[V0][0] = 0x4;
        c.cycles += 2;
        let is_four = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x3;
        if is_four {
            c.cycles += 1;
            c.sw(c.g[A3][0] as u32, c.g[V0][0]);
        } else {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
    } else {
        c.cycles += 2;
        if (c.g[V1][0] as i64) > 0 {
            // 1..3: already in use.
        } else {
            c.cycles += 2;
            let is_zero = c.g[V1][0] == 0;
            c.g[V0][0] = 0x1;
            if is_zero {
                c.cycles += 2;
                c.sw(c.g[A3][0] as u32, c.g[V0][0]);
            } else {
                c.cycles += 2;
            }
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `void cWorldCache_ExpireHull(cWorldCache* self, int index)` (0x3a84c8):
/// release the hull slot `index`: clear its in-use flag and refresh its
/// last-use stamp from the global frame counter.
pub fn c_world_cache_expire_hull(c: &mut Cpu) {
    c.cycles += 11;
    let self_ = abi::arg(c, 0);
    c.g[V0][0] = 0x18;
    c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c);
    c.mult(0, A1, A1, V0);
    c.g[V1][0] = sx(self_.wrapping_add(c.g[A1][0] as u32));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x3f4), 0);
    c.g[A1][0] = c.g[V1][0];
    c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x1c).lw(c);
    c.g[V1][0] = Addr(self_.wrapping_add(0x1bf0)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    c.sw((c.g[A1][0] as u32).wrapping_add(0x400), c.g[V0][0]);
    abi::ret(c);
}


/// `void cWorldCache_SetHullPinned(cWorldCache* self, int index, int pinned)`
/// (0x3a84f8): store `pinned` in the hull slot's flag word.
pub fn c_world_cache_set_hull_pinned(c: &mut Cpu) {
    c.cycles += 5;
    let self_ = abi::arg(c, 0);
    c.g[V0][0] = 0x18;
    c.mult(0, V1, A1, V0);
    c.g[A0][0] = sx(self_.wrapping_add(c.g[V1][0] as u32));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x3f4), c.g[A2][0]);
    abi::ret(c);
}


/// `bool cWorldCache_IsHullResident(cWorldCache* self, int index)` (0x3a8618):
/// true when the hull slot's ref count is exactly 3.
pub fn c_world_cache_is_hull_resident(c: &mut Cpu) {
    c.cycles += 7;
    let self_ = abi::arg(c, 0);
    c.g[V0][0] = 0x18;
    c.mult(0, V1, A1, V0);
    c.g[A0][0] = sx(self_.wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x3f0).lw(c);
    c.g[V0][0] = c.g[V0][0] ^ 0x3;
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    abi::ret(c);
}


/// `bool cWorldCache_IsHullFree(cWorldCache* self, int index)` (0x3a8638):
/// true when the hull slot's ref count is zero.
pub fn c_world_cache_is_hull_free(c: &mut Cpu) {
    c.cycles += 6;
    let self_ = abi::arg(c, 0);
    c.g[V0][0] = 0x18;
    c.mult(0, V1, A1, V0);
    c.g[A0][0] = sx(self_.wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x3f0).lw(c);
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    abi::ret(c);
}


/// `void cWorldCache_SetHullResident(cWorldCache* self, int index)`
/// (0x3a8650): mark the hull slot resident (ref count 3).
pub fn c_world_cache_set_hull_resident(c: &mut Cpu) {
    c.cycles += 6;
    let self_ = abi::arg(c, 0);
    c.g[V0][0] = 0x18;
    c.g[V1][0] = 0x3;
    c.mult(0, A2, A1, V0);
    c.g[A0][0] = sx(self_.wrapping_add(c.g[A2][0] as u32));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x3f0), c.g[V1][0]);
    abi::ret(c);
}


/// `void cWorldCache_Reset(cWorldCache* w)` (0x3a7f90): drop every loaded
/// section (`cWorldCache_Free`) and clear the three cache words.
pub fn c_world_cache_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::WORLD_CACHE_FREE.call(c, 0x003a_7fa4)?;
        let cache = c.g[S0][0];
        Addr(cache as u32).off(0x3e8).sw(c, 0);
        Addr(cache as u32).sw(c, 0);
        Addr(cache as u32).off(0x3ec).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void WorldMemman_FreeSectionAllocator(WorldMemman* self, int i)`
/// (0x3a7b98): destroy and clear section allocator `i` at `self+0x3c+i*4`.
pub fn world_memman_free_section_allocator(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = sx(p(abi::arg(c, 0), 0x3c));
        frame.save(c, S0, 0x10);
        c.g[A1][0] = sx(abi::arg(c, 1) << 2);
        frame.save_ra(c, 0);
        c.g[S0][0] = add32(c.g[A0][0], c.g[A1][0]);
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        if c.g[A0][0] == 0 {
            c.sw(r32(c, S0), 0);
        } else {
            c.cycles += 3;
            c.g[A1][0] = 3;
            callee::SECTION_ALLOCATOR_DTOR.call(c, 0x003a_7bc4)?;
            c.sw(r32(c, S0), 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        leave(c, 0x20);
        Ok(())
    })
}


/// `void SectionAllocator_dtor(SectionAllocator* a, int flags)` (0x3a77d0):
/// free every section and, when `flags & 1`, the allocator.
pub fn section_allocator_dtor(c: &mut Cpu) {
    guest(c, |c| {
        dtor(
            c,
            callee::SECTION_ALLOCATOR_FREE_ALL,
            0x003a_77ec,
            0x003a_7800,
        )
    })
}


/// `void cWorldCache_StreamerDtor(void* p, int flags)` (0x3a8fb8): the streamer
/// destructor and, when `flags & 1`, the object.
pub fn c_world_cache_streamer_dtor(c: &mut Cpu) {
    guest(c, |c| {
        dtor(
            c,
            callee::C_WORLD_CACHE_STREAMER_DTOR_STUB,
            0x003a_8fd4,
            0x003a_8fe8,
        )
    })
}


/// `void SectionTable_dtor(SectionTable* self, int flags)` (0x3a7d80): free
/// the owned table and, when `flags & 1`, the object.
pub fn section_table_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(this).lw(c);
        let none = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !none {
            c.cycles += 2;
            call_ptr(c, table::BUILTIN_VEC_DELETE, 0x003a_7da8)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x003a_7dbc)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cHullPage* cHullPage_ctor(cHullPage* p_2, const u16* data)` (0x3a7bd8): keep
/// the vertex count from `*data`, allocate `4 * count` bytes of index space.
pub fn c_hull_page_ctor(c: &mut Cpu) {
    c.cycles += 21;
    let frame = Frame::open(c, 0x20);
    c.g[V1][0] = c.g[A1][0];
    frame.save(c, S0, 0x10);
    c.g[S0][0] = c.g[A0][0];
    c.sw(p_2(c, S0, 0), c.g[V1][0]);
    c.g[A1][0] = sx(0x0049_4e00);
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    c.g[V0][0] = Addr(p_2(c, V1, 0)).lhu(c);
    c.sw(p_2(c, S0, 8), 0);
    c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
    frame.save_ra(c, 0);
    c.sw(p_2(c, S0, 4), c.g[V0][0]);
    if call_ptr(c, table::OPERATOR_NEW_NAMED, 0x003a_7c14).is_err() {
        return;
    }
    c.sw(p_2(c, S0, 0xc), c.g[V0][0]);
    c.g[V0][0] = c.g[S0][0];
    frame.restore_ra(c, 0);
    frame.restore(c, S0, 0x10);
    frame.close(c);
}


/// `void cHullPage_dtor(cHullPage* p_2, int flags)` (0x3a7c30): clear the count,
/// free the index array and `delete this` for bit 0.
pub fn c_hull_page_dtor(c: &mut Cpu) {
    c.cycles += 9;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.sw(p_2(c, S0, 4), 0);
    c.g[A0][0] = Addr(p_2(c, S0, 0xc)).lw(c);
    let null = c.g[A0][0] == 0;
    c.g[S1][0] = c.g[A1][0];
    if !null {
        c.cycles += 2;
        if call_ptr(c, table::EA_MEM_FREE, 0x003a_7c5c).is_err() {
            return;
        }
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[S1][0] & 1;
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        if call_ptr(c, table::DELETE, 0x003a_7c70).is_err() {
            return;
        }
    }
    frame.restore(c, S0, 0x20);
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}


// ---------------------------------------------------------------------------
// World cache.

/// `void cWorldCache_dtor(cWorldCache* self, int flags)` (0x3a7e38): release
/// the cache's hulls/sections and both streamers, then delete it when owned.
pub fn world_cache_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x003a_7e98, 0x003a_7e54)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x300));
        c.g[A1][0] = 2;
        call_ptr(c, 0x003a_6e20, 0x003a_7e60)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = 2;
        call_ptr(c, 0x003a_8fb8, 0x003a_7e6c)?;
        c.g[S0][0] &= 1;
        let owns = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !owns {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x003a_7e80)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void cWorldCache_FreeSectionTable(cWorldCache* self, int i)` (0x3a8230):
/// destroy section table `i` and clear its slot.
pub fn world_cache_free_section_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[A0][0] = lw(c, p_3(c.g[S0][0], 0xc));
        call_ptr(c, 0x003a_7b98, 0x003a_8250)?;
        c.g[V0][0] = lw(c, p_3(c.g[S0][0], 8));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = lw(c, c.g[V0][0] as u32);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x003a_7d80, 0x003a_826c)?;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = lw(c, p_3(c.g[S0][0], 8));
        frame.restore_ra(c, 0);
        frame.restore(c, S1, 0x10);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        Addr(c.g[V0][0] as u32).sw(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `cWorldCache_InitHullSlots(cWorldCache* self)` (0x3a82c8): ask the streamer
/// how many hulls there are and zero each slot (six words, 0x18 bytes).
pub fn c_world_cache_init_hull_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x10));
        call_ptr(c, 0x003a_9ab0, 0x003a_82e0)?;
        c.g[A0][0] = 0;
        let none = c.g[V0][0] == 0;
        sw(c, p(this as u32, 0x3ec), c.g[V0][0]);
        if !none {
            c.cycles += 1;
            c.g[V1][0] = sx(p(this as u32, 0x3fc));
            loop {
                c.cycles += 11;
                sw(c, (c.g[V1][0] as u32).wrapping_sub(0xc), 0);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                sw(c, (c.g[V1][0] as u32).wrapping_sub(8), 0);
                sw(c, (c.g[V1][0] as u32).wrapping_sub(4), 0);
                sw(c, (c.g[V1][0] as u32).wrapping_add(4), 0);
                sw(c, (c.g[V1][0] as u32).wrapping_add(8), 0);
                sw(c, c.g[V1][0] as u32, 0);
                c.g[V0][0] = lw(c, p(this as u32, 0x3ec));
                c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x18));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_82f0);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cWorldCache_StreamerReleaseNodeHulls(cWorldCache* self, Node* node)`
/// (0x3a9558): recursively expire the node's section hull and both children.
pub fn cworld_cache_streamer_release_node_hulls(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        open(c, 0x30);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
        let no_section = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A0][0];
        if !no_section {
            c.cycles += 3;
            c.g[A1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
            c.g[RA][0] = sx(0x003a_9584);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32));
            call_ptr(c, 0x003a_84c8, 0x003a_9584)?;
        }
        c.cycles += 3;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
        if c.g[A1][0] != 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x003a_9598);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003a_9558, 0x003a_9598)?;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
        } else {
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
        }
        c.cycles += 2;
        let no_right = c.g[A1][0] == 0;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        if !no_right {
            c.cycles += 3;
            c.g[RA][0] = sx(0x003a_95ac);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003a_9558, 0x003a_95ac)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}


/// `void WorldMemman_dtor(WorldMemman* m, int flags)` (0x3a7aa8): destroy the
/// two world block pools, then optionally free the manager itself.
pub fn world_memman_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 17, 0x20);
        sqsp(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        sqsp(c, 16, 0x30);
        c.g[S2][0] = c.g[A1][0];
        let null = c.g[S1][0] == 0;
        frame.save_ra(c, 0);
        if !null {
            c.cycles += 3;
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3c));
            let single = c.g[S1][0] == c.g[S0][0];
            c.g[V0][0] = c.g[S2][0] & 1;
            if !single {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x14));
                loop {
                    c.cycles += 5;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S0][0];
                    call::call_ptr(c, 0x003a_76c0, 0x003a_7ae4)?;
                    let more = c.g[S1][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x14));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_7ad8);
                    }
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[S2][0] & 1;
        c.cycles += 2;
        let no_free = c.g[V0][0] == 0;
        lqsp(c, 16, 0x30);
        if !no_free {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call::call_ptr(c, 0x0031_7e50, 0x003a_7b00)?;
            lqsp(c, 16, 0x30);
        }
        c.cycles += 5;
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}


/// `void cWorldCache_FlagSectionObjects(cWorldCache* cache, int idx)`
/// (0x3a8f10): set bit 6 of the flags word of every object in a section.
pub fn c_world_cache_flag_section_objects(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0x58;
    c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.mult(0, V0, A1, V0);
    c.g[A2][0] = Addr(p_4(r32(c, A0), 8)).lw(c);
    c.g[A0][0] = Addr(p_4(c.g[V1][0] as u32, 0x14)).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A1][0] = abi::addu(c.g[A1][0], c.g[A2][0]);
    c.g[A2][0] = 0;
    c.g[V0][0] = abi::addu(c.g[V0][0], c.g[A0][0]);
    c.g[A3][0] = Addr(p_4(c.g[V0][0] as u32, 0x22)).lh(c);
    let none = c.g[A3][0] == 0;
    c.g[A1][0] = Addr(c.g[A1][0] as u32).lw(c);
    if !none {
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(p_4(c.g[A1][0] as u32, 0xc)).lw(c);
            c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
            c.g[V1][0] = abi::addu(c.g[V1][0], c.g[V0][0]);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32) >> 8);
            let nonzero = c.g[A0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            if !nonzero {
                c.cycles += 1;
                c.g[A0][0] = 0;
            }
            c.cycles += 6;
            c.g[V0][0] = Addr(p_4(c.g[A0][0] as u32, 0xa)).lhu(c);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            c.g[V1][0] = (c.g[A2][0] < c.g[A3][0]) as u64;
            c.g[V0][0] = c.g[V0][0] | 0x40;
            let more = c.g[V1][0] != 0;
            Addr(p_4(c.g[A0][0] as u32, 0xa)).sh(c, c.g[V0][0]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_8f40);
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
}


/// `void WorldMemman_CreateSectionAllocator(self, index, section)`
/// (0x3a7b18): `self->allocators[index] = new SectionAllocator(
/// EA_MemAllocNamed(0x14, ...), &sections[section], index)`.
pub fn worldmemman_create_section_allocator(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x50);
        c.g[A3][0] = 0;
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, 16, 0x40);
        frame.save(c, 19, 0x10);
        c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
        c.g[S3][0] = c.g[A2][0];
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3c));
        c.g[A1][0] = 0x490000;
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4de8));
        c.g[A0][0] = 0x14;
        c.g[A2][0] = 0x20000000;
        call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x003a_7b60)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x14;
        mul32(c, V0, S3, A1);
        c.g[A2][0] = c.g[S2][0];
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        callee::SECTION_ALLOCATOR_CTOR.call(c, 0x003a_7b78)?;
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// World memory manager and cache.

/// `WorldMemman* WorldMemman_ctor(WorldMemman* self)` (0x3a7a20): initialise
/// the three block pools and clear the 64-entry block table.
pub fn world_memman_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0x45_0000;
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(0x0044_c1e0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = Addr(p(r32(c, S0), 4)).lw(c);
        c.g[RA][0] = sx(0x003a_7a48);
        c.g[A2][0] = Addr(p(r32(c, S0), 8)).lw(c);
        callee::WORLD_BLOCK_POOL_INIT.call(c, 0x003a_7a48)?;
        c.g[A1][0] = Addr(p(r32(c, S0), 0x10)).lw(c);
        c.g[A0][0] = sx(p(r32(c, S1), 0x14));
        c.g[RA][0] = sx(0x003a_7a58);
        c.g[A2][0] = Addr(p(r32(c, S0), 0x14)).lw(c);
        callee::WORLD_BLOCK_POOL_INIT.call(c, 0x003a_7a58)?;
        c.g[A2][0] = Addr(p(r32(c, S0), 0x20)).lw(c);
        c.g[A0][0] = sx(p(r32(c, S1), 0x28));
        c.g[RA][0] = sx(0x003a_7a68);
        c.g[A1][0] = Addr(p(r32(c, S0), 0x1c)).lw(c);
        callee::WORLD_BLOCK_POOL_INIT.call(c, 0x003a_7a68)?;
        c.g[V1][0] = 0x3f;
        c.g[V0][0] = sx(p(r32(c, S1), 0x138));
        loop {
            c.cycles += 7;
            Addr(r32(c, V0)).sw(c, 0);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_7a70);
            }
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cWorldCache_CreateSectionTable(cWorldCache* self, int sections,
/// void* section)` (0x3a81a8): create the section allocator, allocate the
/// section table and construct it.
pub fn c_world_cache_create_section_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x003a_81cc);
        c.g[A0][0] = Addr(p(r32(c, S2), 0xc)).lw(c);
        callee::WORLD_MEMMAN_CREATE_SECTION_ALLOCATOR.call(c, 0x003a_81cc)?;
        c.g[V0][0] = Addr(p(r32(c, S2), 8)).lw(c);
        c.g[S1][0] = sx(r32(c, S0) << 2);
        c.g[A1][0] = 0x49_0000;
        c.g[A3][0] = 0;
        c.g[S1][0] = sx(p(r32(c, S1), r32(c, V0)));
        c.g[A1][0] = sx(0x0049_4e60);
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x003a_81f0);
        c.g[A0][0] = 0xe8;
        call_ptr(c, 0x0031_7d70, 0x003a_81f0)?;
        c.g[V1][0] = 0x58;
        c.g[A2][0] = Addr(r32(c, S2)).lw(c);
        c.mult(0, 16, 16, 3);
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = Addr(p(r32(c, A2), 0x14)).lw(c);
        c.g[A1][0] = sx(p(r32(c, A1), r32(c, S0)));
        c.g[RA][0] = sx(0x003a_8210);
        c.g[A1][0] = sx(p(r32(c, A1), 0x20));
        callee::SECTION_TABLE_CTOR.call(c, 0x003a_8210)?;
        Addr(r32(c, S1)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int cWorldCache_StreamerGetViewClearanceAll(cWorldCache* cache, int idx,
/// Vec4* out_pos, float* out_t)` (0x3a9770): update the view, copy the view
/// quad, and report whether the view's fade value is past the halfway point.
pub fn c_world_cache_streamer_get_view_clearance_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        let self_ = abi::arg(c, 0);
        let idx = abi::arg64(c, 1);
        let out_pos = abi::arg64(c, 2);
        let out_t = abi::arg64(c, 3);
        c.g[V0][0] = 0x50;
        c.mult(0, 3, 5, 2);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = out_pos;
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
        c.g[V0][0] = c.lw(at(c, S0, 0x240));
        let present = c.g[V0][0] != 0;
        c.g[S2][0] = out_t;
        if !present {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 6;
            frame.restore(c, S0, 0x30);
            frame.restore(c, S1, 0x20);
            frame.restore(c, S2, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 14;
        call_ptr(c, callee::WORLD_CACHE_STREAMER_UPDATE_VIEW, 0x003a_97b0)?;
        c.lq(2, at(c, S0, 0x250));
        c.g[V1][0] = 1;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 2);
        c.sq(2, r32(c, S1));
        c.mtc1(ZERO, 1);
        c.lwc1(0, at(c, S0, 0x288));
        c.sub_s(0, 0, 2);
        c.c_lt_s(1, 0);
        let positive = c.fcc();
        c.swc1(0, r32(c, S2));
        if !positive {
            c.cycles += 1;
            c.g[V1][0] = 0;
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[V1][0];
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cWorldCache_StreamerAreNodeHullsFree` (0x3a95c0): walk the streamer node
/// chain and return whether all hulls it references are free.
pub fn world_cache_streamer_are_node_hulls_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        let world = r(c, A0);
        c.g[V0][0] = Addr(r(c, S1)).lw(c);
        c.g[S2][0] = sx(world);

        let mut plus_one = false;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r(c, S0), 0x58)).lw(c);
            let mut a1;
            let mut finished = false;
            if c.g[V0][0] == 0 {
                a1 = Addr(p(r(c, S0), 0x50)).lw(c);
            } else {
                c.cycles += 5;
                a1 = Addr(p(r(c, V0), 2)).lhu(c);
                c.g[A1][0] = a1;
                c.g[RA][0] = sx(0x003a_9600);
                c.g[A0][0] = Addr(r(c, S2)).lw(c);
                callee::WORLD_CACHE_IS_HULL_FREE.call(c, 0x003a_9600)?;
                if c.g[V0][0] != 0 {
                    a1 = Addr(p(r(c, S0), 0x50)).lw(c);
                } else {
                    c.cycles += 2;
                    Addr(r(c, S1)).sw(c, 0);
                    plus_one = true;
                    finished = true;
                }
            }
            if !finished {
                c.cycles += 2;
                c.g[A1][0] = a1;
                c.g[A0][0] = c.g[S2][0];
                if a1 != 0 {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x003a_9620);
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, 0x003a_95c0, 0x003a_9620)?;
                }
                c.cycles += 3;
                c.g[A1][0] = Addr(p(r(c, S0), 0x54)).lw(c);
                let no_next = c.g[A1][0] == 0;
                frame.restore(c, S0, 0x30);
                if !no_next {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r(c, S1)).lw(c);
                    let keep = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S2][0];
                    if !keep {
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x003a_9640);
                        c.g[A2][0] = c.g[S1][0];
                        call_ptr(c, 0x003a_95c0, 0x003a_9640)?;
                        plus_one = true;
                    }
                }
            }
        } else {
            plus_one = true;
        }
        if plus_one {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void WorldBlockPool_dtor(self, flags)` (0x3a76c0): free every resident
/// block and the block array, then delete the pool itself when asked.
pub fn world_block_pool_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S1), 4)).lw(c);
        let none = c.g[V0][0] == 0;
        c.g[S3][0] = c.g[A1][0];
        if !none {
            c.cycles += 1;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(r32(c, S1), 0xc)).lw(c);
                c.g[V0][0] = sx(p(r32(c, S2), r32(c, V0)));
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                if c.g[A0][0] == 0 {
                    c.g[V0][0] = Addr(p(r32(c, S1), 4)).lw(c);
                } else {
                    c.cycles += 3;
                    call_ptr(c, 0x0031_7e98, 0x003a770c)?;
                    c.g[V0][0] = Addr(p(r32(c, S1), 4)).lw(c);
                }
                c.cycles += 4;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx(p(r32(c, S2), 0x18));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_76f0);
                }
            }
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(p(r32(c, S1), 0xc)).lw(c);
        let no_array = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S3][0] & 1;
        if !no_array {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x003a7734)?;
            c.g[V0][0] = c.g[S3][0] & 1;
        }
        c.cycles += 2;
        let keep = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x40);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x003a7748)?;
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


/// `void cWorldCache_StreamerReset(cWorldCache *self)` (0x3a9188): clear the
/// streamer's view slots and reset both view descriptors.
pub fn cworldcache_streamer_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x0050_0000;
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = 0;
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[V0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xed0));
        Addr(c.g[S2][0] as u32).sw(c, 0);
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
        Addr(p_5(c.g[S2][0], 0x4)).sw(c, 0);
        Addr(p_5(c.g[S2][0], 0x8)).sw(c, 0);
        Addr(p_5(c.g[S2][0], 0xc)).sw(c, 0);
        Addr(p_5(c.g[S2][0], 0x10)).sw(c, 0);
        c.lq(3, c.g[V0][0] as u32);
        c.sq(3, p_5(c.g[S2][0], 0x220));
        c.lq(4, c.g[V0][0] as u32);
        c.sq(4, p_5(c.g[S2][0], 0x230));
        loop {
            c.cycles += 7;
            Addr((c.g[A1][0] as u32).wrapping_sub(0x4)).sw(c, 0);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            Addr(c.g[A1][0] as u32).sw(c, 0);
            c.g[V0][0] = (c.g[S1][0] < 0x40) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_91e0);
            }
        }
        c.cycles += 3;
        c.g[S1][0] = 0;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x240));
        loop {
            c.cycles += 10;
            Addr(c.g[S0][0] as u32).sw(c, 0);
            c.g[A1][0] = c.g[S1][0];
            c.mtc1(0, 12);
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = sx((c.g[S3][0] as u32).wrapping_sub(0xed0));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            call_ptr(c, 0x003a_9658, 0x003a_9224)?;
            c.g[V0][0] = (c.g[S1][0] < 0x2) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_9208);
            }
        }
        c.cycles += 9;
        Addr(p_5(c.g[S2][0], 0x2e4)).sw(c, 0);
        Addr(p_5(c.g[S2][0], 0x2e0)).sw(c, 0);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cWorldCache_FreeHullPage(self, page)` (0x3a8528): release every resource
/// of a cached hull page, destroy the page object and clear the cache slot
/// and its page table entry.
pub fn cworldcache_free_hull_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x70);
        let this = abi::arg(c, 0);
        c.g[V0][0] = 0x18;
        frame.save(c, 19, 0x30);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, 20, 0x20);
        c.mult(0, V1, S3, V0); // v1 = page * 0x18
        frame.save(c, 16, 0x60);
        frame.save(c, 17, 0x50);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, 21, 0x10);
        c.g[S0][0] = sx(0xffff_0000);
        frame.save_ra(c, 0);
        frame.save(c, 18, 0x40);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S4][0] as u32)));
        c.g[S2][0] = lw_2(c, p(c.g[V0][0] as u32, 0x3f8));
        c.g[V1][0] = lw_2(c, p(c.g[S2][0] as u32, 8));
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        let t = (c.g[S1][0] as i64) < 0;
        c.g[S0][0] |= 0xffff;
        if !t {
            c.cycles += 3;
            c.g[S5][0] = sx(0xffff_ff00);
            c.g[V1][0] = lw_2(c, p(c.g[S2][0] as u32, 0xc));
            loop {
                c.cycles += 17;
                c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[S0][0] &= c.g[S5][0];
                c.g[A0][0] = c.g[S4][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = c.lbu(p(c.g[V0][0] as u32, 1));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
                c.g[V1][0] = c.lhu(p(c.g[V0][0] as u32, 2));
                c.g[S0][0] |= c.g[A2][0];
                c.g[A2][0] = c.lbu(c.g[V0][0] as u32);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 8);
                c.g[V0][0] = c.g[S0][0] & 0xff;
                c.g[S0][0] = c.g[V0][0] | c.g[V1][0];
                c.g[RA][0] = sx(0x003a_85bc);
                c.g[A3][0] = c.g[S0][0];
                c.pc = 0x003a_8e20;
                ee::s13::cWorldCache_ReleaseResource(c);
                if c.pc != 0x003a_85bc {
                    return Err(Unwound);
                }
                if (c.g[S1][0] as i64) < 0 {
                    break;
                }
                c.g[V1][0] = lw_2(c, p(c.g[S2][0] as u32, 0xc));
                if event_due(c) {
                    deliver_event_here(c, 0x003a_8580);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = 0x18;
        c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x3f8));
        c.mult(0, 16, S3, V0); // a0 = page * 0x18
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        c.g[A0][0] = lw_2(c, c.g[S1][0] as u32);
        let t = c.g[A0][0] == 0;
        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        if !t {
            c.cycles += 3;
            c.g[RA][0] = sx(0x003a_85e8);
            c.g[A1][0] = 3;
            c.pc = 0x003a_7c30;
            ee::s13::cHullPage_dtor(c);
            if c.pc != 0x003a_85e8 {
                return Err(Unwound);
            }
            c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        }
        c.cycles += 11;
        c.sw(c.g[S1][0] as u32, 0);
        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        frame.restore(c, 20, 0x20);
        frame.restore(c, 21, 0x10);
        frame.restore_ra(c, 0);
        c.sw(p(c.g[V0][0] as u32, 0x3f0), 0);
        frame.close(c);
        Ok(())
    })
}


/// `void* func_003A7C88(WorldCacheTable* self, short* sizes)` (0x3a7c88):
/// build the world-cache table: keep the 28 class sizes, total them, allocate
/// and clear the entry pool, then point the non-empty classes at their slices.
pub fn section_table_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0xe0;
        frame.save(c, 17, 0x20);
        frame.save(c, 16, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 18, 0x10);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
        frame.save_ra(c, 0);
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(8));
        c.sw(p(c.g[S1][0] as u32, 0xe4), c.g[A1][0]); // the size table
        c.g[A1][0] = 0;
        call_ptr(c, 0x0041_6210, 0x003a_7cb8)?; // memset(entries, 0, 0xe0)
        c.g[S0][0] = 0; // total
        c.g[A2][0] = 0;
        c.g[A1][0] = c.g[S2][0];
        loop {
            c.cycles += 10;
            c.g[V1][0] = c.lw(p(c.g[S1][0] as u32, 0xe4));
            c.g[V0][0] = sx((c.g[A2][0] as u32) << 1);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = ((c.g[A2][0] as i64) < 28) as u64;
            c.g[V1][0] = c.lh(c.g[V0][0] as u32);
            c.sw(c.g[A1][0] as u32, c.g[V1][0]); // counts[i]
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            let more = c.g[A0][0] != 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_7cc8);
            }
        }
        c.cycles += 18;
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[A1][0] = 0x49_0000;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4e18)); // D_00494E18
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x003a_7d0c)?; // operator new(total * 4)
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.sw(c.g[S1][0] as u32, c.g[V0][0]); // self->pool
        call_ptr(c, 0x0041_6210, 0x003a_7d20)?; // clear the pool
        c.g[T0][0] = c.lw(c.g[S1][0] as u32); // pool
        c.g[A0][0] = c.g[S2][0];
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
        c.g[A2][0] = 0x1b;
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(c.g[A0][0] as u32); // count
            let empty = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
            if !empty {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sw(c.g[A1][0] as u32, c.g[V0][0]); // ptr = pool + offset
                c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
            }
            c.cycles += 4;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
            let more = (c.g[A2][0] as i64) >= 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(8));
            if !more {
                c.cycles += 7;
                c.g[V0][0] = c.g[S1][0];
                frame.restore(c, 16, 0x30);
                frame.restore(c, 17, 0x20);
                frame.restore(c, 18, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_7d38);
            }
        }
    })
}


/// `WorldBlockPool_Init` (0x003a7598): initialise a world-block pool:
/// `self+0` = block size, `self+4`/`self+8` = block count, `self+0xc` = the
/// block array (`count * size` bytes), then link every block to the next
/// (`+0x14`) with `+0x4` = the block size, `+0x8`/`+0xc`/`+0x10` cleared and
/// `+0` = a freshly allocated per-block payload, and finally store the array
/// in `self+0x10`.
pub fn world_block_pool_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = c.g[A1][0]; // count
        frame.save(c, S1, 0x50);
        c.g[A1][0] = 0x49_0000;
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A0][0]; // this
        frame.save(c, S0, 0x60);
        c.g[A0][0] = 0x18; // block size
        frame.save(c, S2, 0x40);
        c.mult(0, 0, 2, 4); // blocks * 0x18
        frame.save(c, S4, 0x20);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4dc0));
        frame.save(c, S5, 0x10);
        c.g[A3][0] = 0u64;
        frame.save_ra(c, 0);
        c.g[S3][0] = 0u64;
        c.sw(c.g[S1][0] as u32, c.g[A2][0]);
        c.g[A0][0] = c.lo[0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A2][0] = 0x2000_0000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0031_7e30, 0x003a_75f0)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        let empty = c.g[V1][0] == 0;
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        if !empty {
            c.cycles += 3;
            c.g[S5][0] = 0x49_0000;
            c.g[S4][0] = 0x18;
            c.g[S2][0] = 0u64;
            loop {
                c.cycles += 35;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4dd8));
                c.g[A2][0] = 0x2500_0000;
                c.g[A3][0] = 0u64;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), 0u64);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
                call_ptr(c, 0x0031_7e30, 0x003a_7658)?;
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                c.g[V1][0] = (c.g[S3][0] < c.g[V1][0]) as u64;
                if c.g[V1][0] == 0 {
                    c.g[V0][0] = 0u64;
                }
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = (c.g[S3][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_7608);
                }
            }
        }
        c.cycles += 11;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        frame.restore(c, S0, 0x60);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cWorldCache_FlushHullPages` (0x003a88a8): flush the world cache's hull
/// pages.  For each used page record at `self+0x3f0` (0x18 bytes, count at
/// `self+0x3ec`) the page's resource pointer (`+8`) is freed through
/// `cWorldCache_FreeHullPage(self, i)`, with the page's "keep" flag decided
/// by its `+4`/`+0x10` fields against the cache limit at
/// `(*(gp+0x2a74))+0x1c`; the page then counts as free (`*page = 0`) or as
/// retired (`*page = 4`).
pub fn c_world_cache_flush_hull_pages(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a74));
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        c.g[S1][0] = 0u64; // i
        frame.save(c, S0, 0x80);
        c.g[S2][0] = c.g[A0][0]; // self
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save(c, S7, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3ec));
        let none = c.g[V1][0] == 0;
        c.g[S5][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
        if !none {
            c.cycles += 6;
            c.g[S4][0] = 0x1;
            c.g[S7][0] = 0x2;
            c.g[S3][0] = 0x4;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3f0));
            c.g[S6][0] = 0x3;
            loop {
                'page: {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                    if c.g[V1][0] == 0 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3ec));
                        break 'page;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    let used = c.g[V0][0] != 0;
                    c.g[A0][0] = 0u64;
                    if !used {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = c.g[S4][0];
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                        if c.g[V0][0] == 0 {
                            c.g[A0][0] = 0;
                        }
                    }
                    'kind: {
                        c.cycles += 2;
                        let kind2 = c.g[V1][0] == c.g[S7][0];
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                        if kind2 {
                            // Kind 2 keeps its page: just the +1 below.
                        } else {
                            'callblk: {
                                'pres: {
                                    'keep: {
                                        'ge3: {
                                            c.cycles += 2;
                                            if c.g[V0][0] == 0 {
                                                c.cycles += 2;
                                                if c.g[V1][0] == c.g[S4][0] {
                                                    break 'keep;
                                                }
                                                c.cycles += 2;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x3ec));
                                                break 'page;
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[V1][0] == c.g[S6][0] {
                                            break 'pres;
                                        }
                                        c.cycles += 2;
                                        let four = c.g[V1][0] == c.g[S3][0];
                                        c.g[A0][0] = c.g[S2][0];
                                        if four {
                                            break 'callblk;
                                        }
                                        c.cycles += 2;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3ec));
                                        break 'page;
                                    }
                                    c.cycles += 2;
                                    if c.g[A0][0] == 0 {
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3ec));
                                        break 'page;
                                    }
                                    c.cycles += 2;
                                    c.sw(c.g[S0][0] as u32, 0u64);
                                    break 'kind;
                                }
                                c.cycles += 2;
                                let t = c.g[A0][0] == 0;
                                c.g[A0][0] = c.g[S2][0];
                                if t {
                                    break 'kind;
                                }
                                c.cycles += 1;
                                c.sw(c.g[S0][0] as u32, c.g[S3][0]);
                                break 'callblk;
                            }
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x003a_8984);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x003a_8528, 0x003a_8984)?;
                            c.sw(c.g[S0][0] as u32, 0u64);
                            break 'kind;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3ec));
                }
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_8900);
                }
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
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::ptr;
    use crate::difftest::put_words;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use ssx3_ee as ee;
    use crate::difftest::RA as RET;
    use crate::difftest::fakes::*;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::compare;
    use crate::difftest::put_vec;
    use crate::difftest::seed_count;
    pub(crate) use crate::game::ai::ai::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::ai::ai::tests::be_library;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS_2;
    pub(crate) use crate::game::be::belibrary::tests::load;
    pub(crate) use crate::game::be::belibrary::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::load as load_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::object::animnode::tests::heap_block;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::MEM_HEAP;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::ARENA;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::ARENA_SIZE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::run;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_init;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::alloc;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::fe::fereal::tests::base as base_2;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::ai::ai::tests::w16;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_4;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5;
    pub(crate) use crate::game::main::gamestate::tests::base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world;
    pub(crate) use crate::game::bx::bxstring::tests::heap_init_3 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;
    pub(crate) use crate::game::dirtysock::tags2::tests::DST;
    pub(crate) use crate::game::dirtysock::tags2::tests::LISTARR;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_5 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_6;
    pub(crate) use crate::game::ai::ai::tests::SEEDS as SEEDS_8;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::w as w_5;
    pub(crate) use crate::game::ai::ai::tests::strict;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::globals_3 as globals_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::pool_4 as pool_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6 as base_7;
    pub(crate) use crate::game::be::beintplayer::tests::SEEDS_7 as SEEDS_9;
    pub(crate) use crate::game::be::beintplayer::tests::w as w_6;
    pub(crate) use crate::game::be::beintplayer::tests::globals_5 as globals_3;
    pub(crate) use crate::game::be::beintplayer::tests::base_10 as base_8;


    #[test]
    fn world_block_pool_alloc_block_matches() {
        check(ssx3_ee::s13::f_003a7768, world_block_pool_alloc_block, 200, &|c, seed| {
            let mut r = Rng(0x3a77_6800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let block = DATA + 0x400;
            c.bus.write32(DATA + 0x10, block);
            c.bus.write32(DATA + 0x8, r.next_u32());
            c.bus.write32(block + 0x14, r.next_u32());
            c.bus.write32(block + 0x10, r.next_u32());
        });
    }


    #[test]
    fn world_block_pool_free_block_matches() {
        check(ssx3_ee::s13::f_003a7790, world_block_pool_free_block, 200, &|c, seed| {
            let mut r = Rng(0x1111 ^ seed as u64);
            scramble(c, &mut r);
            let pool = if seed % 3 == 0 { DATA } else { DATA + 0x100 };
            let block = if seed % 3 == 0 { DATA + 0x100 } else { DATA };
            c.g[A0][0] = pool as u64;
            c.g[A1][0] = block as u64;
            c.bus.write32(pool + 0x10, r.next_u32());
            c.bus.write32(pool + 0x8, r.next_u32());
            c.bus.write32(block + 0x10, r.next_u32());
        });
    }


    #[test]
    fn section_allocator_ctor_matches() {
        check(ssx3_ee::s13::f_003a77b0, section_allocator_ctor, 200, &|c, seed| {
            let mut r = Rng(0x2222 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
        });
    }


    #[test]
    fn c_world_cache_request_hull_matches() {
        check_lohi(ssx3_ee::s13::f_003a8448, c_world_cache_request_hull, 200, &|c, seed| {
            let mut r = Rng(0x3333 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            let index = (r.next_u32() % 6) as u32;
            c.g[A1][0] = index as u64;
            let owner = DATA + 0x1000;
            c.bus.write32(GAME_GP + 0x2a74, owner);
            c.bus.write32(owner + 0x1c, r.next_u32());
            let slot = DATA + 0x3f0 + index * 0x18;
            let count = match seed % 6 {
                0 => 4u32,
                1 => 0,
                2 => 1,
                3 => r.next_u32() % 3 + 1,
                4 => 0xffff_ffff,
                _ => r.next_u32() | 0x8000_0000,
            };
            c.bus.write32(slot, count);
        });
    }


    #[test]
    fn c_world_cache_expire_hull_matches() {
        check_lohi(ssx3_ee::s13::f_003a84c8, c_world_cache_expire_hull, 200, &|c, seed| {
            let mut r = Rng(0x4444 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            let index = (r.next_u32() % 6) as u32;
            c.g[A1][0] = index as u64;
            let owner = DATA + 0x1000;
            c.bus.write32(GAME_GP + 0x2a74, owner);
            c.bus.write32(owner + 0x1c, r.next_u32());
            c.bus.write32(DATA + 0x1bf0, r.next_u32());
        });
    }


    #[test]
    fn c_world_cache_set_hull_pinned_matches() {
        check_lohi(ssx3_ee::s13::f_003a84f8, c_world_cache_set_hull_pinned, 200, &|c, seed| {
            let mut r = Rng(0x5555 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (r.next_u32() % 6) as u64;
            c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
        });
    }


    #[test]
    fn c_world_cache_is_hull_resident_matches() {
        check_lohi(ssx3_ee::s13::f_003a8618, c_world_cache_is_hull_resident, 200, &|c, seed| {
            let mut r = Rng(0x6666 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let index = (r.next_u32() % 6) as u32;
            c.g[A1][0] = index as u64;
            let count = if seed % 3 == 0 { 3u32 } else { r.next_u32() };
            c.bus.write32(DATA + 0x3f0 + index * 0x18, count);
        });
    }


    #[test]
    fn c_world_cache_is_hull_free_matches() {
        check_lohi(ssx3_ee::s13::f_003a8638, c_world_cache_is_hull_free, 200, &|c, seed| {
            let mut r = Rng(0x7777 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let index = (r.next_u32() % 6) as u32;
            c.g[A1][0] = index as u64;
            let count = if seed % 3 == 0 { 0u32 } else { r.next_u32() };
            c.bus.write32(DATA + 0x3f0 + index * 0x18, count);
        });
    }


    #[test]
    fn c_world_cache_set_hull_resident_matches() {
        check_lohi(ssx3_ee::s13::f_003a8650, c_world_cache_set_hull_resident, 200, &|c, seed| {
            let mut r = Rng(0x8888 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (r.next_u32() % 6) as u64;
        });
    }


    #[test]
    fn c_world_cache_reset_matches() {
        check(ee::s13::f_003a7f90__generated, c_world_cache_reset, SEEDS, &|c, seed| {
            machine(c, seed, 0x3a7f_90, 2);
            be_library(c);
            // A zeroed cache: no hull pages, no section tables, no streamer.
            c.g[A0][0] = sx(DATA + 0x1000);
        });
    }


    #[test]
    fn world_memman_free_section_allocator_matches() {
        check(
            ee::s13::f_003a7b98__generated,
            world_memman_free_section_allocator,
            SEEDS_2,
            &|c, seed| {
                let _ = machine(c, seed, 0x3a7b98, 0);
                alloc_ready(c);
                let self_at = DATA + 0x200;
                let i = seed % 4;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = i as u64;
                for k in 0..4u32 {
                    let live = k == i && seed % 3 != 0;
                    c.bus.write32(
                        self_at + 0x3c + 4 * k,
                        if live { DATA + 0x400 + 0x100 * k } else { 0 },
                    );
                }
            },
        );
    }


    #[test]
    fn section_allocator_dtor_matches() {
        check(
            ee::s13::f_003a77d0__generated,
            section_allocator_dtor,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a_77d0, 0);
                let obj = DATA + 0x1000;
                let n1 = DATA + 0x2000;
                let n2 = DATA + 0x2100;
                match seed % 3 {
                    0 => {}
                    1 => c.bus.write32(obj + 0xc, n1),
                    _ => {
                        c.bus.write32(obj + 0xc, n1);
                        c.bus.write32(n1 + 0x14, n2);
                    }
                }
                c.bus.write32(obj + 8, 2);
                c.bus.write32(n1, DATA + 0x3000);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = (seed % 4 == 0) as u64;
            },
        );
    }


    #[test]
    fn c_world_cache_streamer_dtor_matches() {
        check(
            ee::s13::f_003a8fb8__generated,
            c_world_cache_streamer_dtor,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a_8fb8, 0);
                let obj = DATA + 0x1000;
                c.bus.write32(obj, r.next_u32());
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }


    #[test]
    fn section_table_dtor_matches() {
        check(
            ee::s13::f_003a7d80__generated,
            section_table_dtor,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x3a7d_80, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x20);
                let owned = if seed % 2 == 0 {
                    0
                } else {
                    heap_block(c, 0x40)
                };
                c.bus.write32(this, owned);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 3 == 0) as u64;
            },
        );
    }


    #[test]
    fn c_hull_page_ctor_matches() {
        check(
            ssx3_ee::s13::f_003a7bd8__generated,
            c_hull_page_ctor,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0xa7bd8, 0);
                heap_init(c);
                let n = 0x81 + seed % 0x700;
                c.bus.write16(DATA, n as u16);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = DATA as u64;
            },
        );
    }


    #[test]
    fn c_hull_page_dtor_matches() {
        check(
            ssx3_ee::s13::f_003a7c30__generated,
            c_hull_page_dtor,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0xa7c30, 0);
                heap_init(c);
                let this = alloc(c, 0x40);
                let idx = if seed % 3 == 0 { 0 } else { alloc(c, 0x40) };
                c.bus.write32(this + 0xc, idx);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }


    // ------------------------------------------------------------------
    // World cache.

    #[test]
    fn world_cache_dtor_matches() {
        check(ee::s13::f_003a7e38__generated, world_cache_dtor, 24, &|c, seed| {
            let _ = base(c, seed, 0x3a7e_38);
            let s = DATA + 0x100;
            clear(c, s, 0x1000);
            w(c, s + 0x3e8, if seed % 5 == 0 { 0 } else { DATA + 0x4000 });
            clear(c, DATA + 0x4000, 0x40);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }


    #[test]
    fn c_world_cache_init_hull_slots_matches() {
        check(
            ssx3_ee::s13::f_003a82c8__generated,
            c_world_cache_init_hull_slots,
            100,
            &|c, seed| {
                let _r = setup(c, seed, 0x3a82c8);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                let n = seed % 3;
                c.bus.write32(DATA + 0x1000 + 0x14, DATA + 0x400);
                c.bus.write32(DATA + 0x400 + 0x10, n);
            },
        );
    }


    #[test]
    fn cworld_cache_streamer_release_node_hulls_matches() {
        check(ee::s13::f_003a9558__generated, cworld_cache_streamer_release_node_hulls, 40, &|c, seed| {
            base_2(c, seed, 0x3a_9558);
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA, DATA + 0x2000);
            c.bus.write32(GAME_GP + 0x2a74, 0);
            let n0 = DATA + 0x1000;
            let section = DATA + 0x1800;
            c.bus.write32(n0 + 0x50, 0);
            c.bus.write32(n0 + 0x54, 0);
            c.bus.write32(n0 + 0x58, if seed % 4 == 0 { 0 } else { section });
            c.bus.write16(section + 0x2, (seed % 3) as u16);
            if seed % 3 == 1 {
                // left child only
                let l = DATA + 0x1200;
                c.bus.write32(n0 + 0x50, l);
                c.bus.write32(l + 0x50, 0);
                c.bus.write32(l + 0x54, 0);
                c.bus.write32(l + 0x58, 0);
            } else if seed % 3 == 2 {
                // both children
                for child in [DATA + 0x1200u32, DATA + 0x1400] {
                    c.bus.write32(n0 + if child == DATA + 0x1200 { 0x50 } else { 0x54 }, child);
                    c.bus.write32(child + 0x50, 0);
                    c.bus.write32(child + 0x54, 0);
                    c.bus.write32(child + 0x58, 0);
                }
            }
            c.g[A1][0] = sx(n0);
        });
    }


    #[test]
    fn world_memman_dtor_matches() {
        check(ssx3_ee::s13::f_003a7aa8__generated, world_memman_dtor, SEEDS_4, &|c, seed| {
            let _ = machine(c, seed, 0x3a7aa8, 0);
            let m = DATA + 0x1000;
            c.g[A0][0] = if seed % 7 == 0 { 0 } else { m as u64 };
            c.g[A1][0] = (seed % 2) as u64;
            // The two block pools: empty (no free path).
            for p_4 in [m + 0x28, m + 0x14] {
                w32(c, p_4 + 4, 0);
                w32(c, p_4 + 0xc, 0);
            }
        });
    }


    #[test]
    fn c_world_cache_flag_section_objects_matches() {
        check(
            ssx3_ee::s13::f_003a8f10__generated,
            c_world_cache_flag_section_objects,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a8f10, 0);
                let cache = DATA + 0x1000;
                let idx = (seed % 3) as u32;
                let world = DATA + 0x2000;
                let table = DATA + 0x3000;
                let lists = DATA + 0x4000;
                let list = DATA + 0x5000;
                let entries = DATA + 0x6000;
                c.g[A0][0] = cache as u64;
                c.g[A1][0] = idx as u64;
                w32(c, cache, world);
                w32(c, cache + 8, lists);
                w32(c, world + 0x14, table);
                let n = (seed % 4) as u16;
                w16(c, table + idx * 0x58 + 0x22, n);
                w32(c, lists + idx * 4, list);
                w32(c, list + 0xc, entries);
                for i in 0..4u32 {
                    let target = DATA + 0x7000 + i * 4;
                    if i < n as u32 {
                        w32(c, entries + i * 4, ((target >> 2) << 8) | (r.next_u32() & 0xff));
                    } else {
                        w32(c, entries + i * 4, 0);
                    }
                    w16(c, target + 0xa, r.next_u32() as u16);
                }
            },
        );
    }


    #[test]
    fn worldmemman_create_section_allocator_matches() {
        check(
            ee::s13::f_003a7b18__generated,
            worldmemman_create_section_allocator,
            SEEDS_5,
            &|c, seed| {
                base_3(c, seed, 0x7b18, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = (seed % 6) as u64;
                c.g[A2][0] = ((seed / 6) % 3) as u64;
            },
        );
    }


    // ------------------------------------------------------------------
    // World memory manager and cache.

    #[test]
    fn world_memman_ctor_matches() {
        check(ssx3_ee::s13::f_003a7a20__generated, world_memman_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3a7a20, 3);
            be_library_2(c);
            world(c);
            heap_init_2(c);
            let obj = DATA + 0x100;
            c.g[A0][0] = obj as u64;
            // The block-pool table: {size, count} triples.
            const TABLE: u32 = 0x0044_c1e0;
            for i in 0..3 {
                let n = (r.next_u32() % 3) as u32;
                w_2(c, TABLE + 0x10 * i + 4, 0x18);
                w_2(c, TABLE + 0x10 * i + 8, n);
            }
        });
    }


    #[test]
    fn c_world_cache_create_section_table_matches() {
        check(
            ssx3_ee::s13::f_003a81a8__generated,
            c_world_cache_create_section_table,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a81a8, 6);
                be_library_2(c);
                world(c);
                heap_init_2(c);
                let cache = DATA + 0x100;
                let sections = DATA + 0x200;
                c.g[A0][0] = cache as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                w_2(c, cache, sections);
                w_2(c, cache + 8, DATA + 0x300);
                w_2(c, cache + 0xc, DATA + 0x400);
                w_2(c, sections + 0x14, DATA + 0x500);
            },
        );
    }


    #[test]
    fn c_world_cache_streamer_get_view_clearance_all_matches() {
        check(
            ee::s13::f_003a9770__generated,
            c_world_cache_streamer_get_view_clearance_all,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a97, 0);
                env(c);
                let this = DATA + 0x400;
                let idx = (seed % 2) as u32;
                let e = this + 0x50 * idx;
                w32(c, e + 0x240, (seed % 3 != 0) as u32);
                for k in 0..4u32 {
                    w32(c, e + 0x250 + 4 * k, r.next_u32());
                }
                w32(c, e + 0x288, r.edge_f32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = sx(DATA + 0x2000);
                c.g[A3][0] = sx(DATA + 0x2100);
            },
        );
    }


    #[test]
    fn world_cache_streamer_are_node_hulls_free_matches() {
        check(
            ee::s13::f_003a95c0__generated,
            world_cache_streamer_are_node_hulls_free,
            SEEDS_7,
            &|c, seed| {
                let _ = base_4(c, seed, 0x95c0, 2);
                let world = DATA;
                let flag = DATA + 0x900;
                let n0 = DATA + 0x800;
                let n1 = DATA + 0x880;
                w_3(c, flag, (seed & 1) as u32);
                w_3(c, n0 + 0x58, if seed & 2 == 0 { 0 } else { DATA + 0x980 });
                w_3(c, DATA + 0x980 + 2, 0);
                w_3(c, n0 + 0x50, (seed & 4) as u32);
                w_3(c, n0 + 0x54, if seed & 8 == 0 { 0 } else { n1 });
                w_3(c, n1 + 0x54, 0);
                c.g[A0][0] = world as u64;
                c.g[A1][0] = n0 as u64;
                c.g[A2][0] = flag as u64;
            },
        );
    }


    #[test]
    fn world_block_pool_dtor_matches() {
        check(
            ee::s13::f_003a76c0__generated,
            world_block_pool_dtor,
            8,
            &|c, seed| {
                let _ = base_5(c, seed, 0x3a76c0, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 2) as u64;
                let n = (seed % 4) as u32;
                w_4(c, OBJ + 4, n);
                w_4(c, OBJ + 0xc, LISTARR);
                for i in 0..4u32 {
                    w_4(c, LISTARR + 0x18 * i, if (seed + i) % 3 == 0 { 0 } else { DST });
                }
            },
        );
    }


    #[test]
    fn cworldcache_streamer_reset_matches() {
        check(
            ee::s13::f_003a9188__generated,
            cworldcache_streamer_reset,
            20,
            &|c, seed| {
                let _r = base_6(c, seed, 0x3a91);
                c.g[A0][0] = DATA as u64;
            },
        );
    }


                #[test]
    fn cworldcache_free_hull_page_matches() {
        strict(ee::s13::f_003a8528__generated, cworldcache_free_hull_page, SEEDS_8, &|c, seed| {
            let mut r = base_7(c, seed, 0x8528, 0);
            let this = DATA + 0x100;
            let page = DATA + 0x2000;
            let page_idx = (seed % 3) as u32;
            w_5(c, this + 0x3f8 + 0x18 * page_idx, page);
            // no resources in the page: the release loop is skipped
            w_5(c, page + 8, 0);
            w_5(c, page + 0xc, DATA + 0x3000);
            w_5(c, page, 0);
            let _ = r;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = page_idx as u64;
        });
    }


    /// The engine's size-class pool_4 allocator behind `gp + 0xdd8`.
    pub(crate) fn pool_4(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x32000),
            (0x2a7c, 0x25800),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w_6(c, GAME_GP.wrapping_add(0xdd8), 1);
        let nodes = DATA + 0x28_000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool_4 = DATA + 0x20_000 + 0x100 * i as u32;
            let base_8 = nodes + 0x400 * i as u32;
            for n in 0..8u32 {
                let node = base_8 + 0x40 * n;
                w_6(c, node, if n == 7 { 0 } else { node + 0x40 });
            }
            w_6(c, pool_4 + head, base_8);
            w_6(c, GAME_GP + slot, pool_4);
        }
    }


        #[test]
    fn section_table_ctor_matches() {
        strict(ee::s13::f_003a7c88__generated, section_table_ctor, SEEDS_9, &|c, seed| {
            let mut r = base_8(c, seed, 0x7c88, 0);
            pool_4(c);
            let this = DATA + 0x1000;
            let sizes = DATA + 0x2000;
            for i in 0..28u32 {
                // some empty classes, some counted ones
                let n = if (seed as u32 + i) % 5 == 0 { 0 } else { (r.next_u32() % 2) as u16 };
                c.bus.write16(sizes + 2 * i, n);
            }
            // at least one class must be non-empty: the allocation is size 0 otherwise
            c.bus.write16(sizes, 3);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(sizes);
        });
    }


    /// Like `difftest::check`, but caller-saved registers and LO/HI must match
    /// too: an indirect call's fake hashes every GPR and both multiplier
    /// halves, so a scratch register left different is a real difference.
    pub(crate) fn check_all(generated: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seed_count(seeds) {
            let r = crate::difftest::compare(generated, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} (soft): {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} (LO/HI): {:#?}", r.lohi);
        }
    }


    pub(crate) fn w_7(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }


    /// `DLHeap_InitArena(&MEM_HEAP, arena, 0x40000)`: the game heap over test
    /// memory, for the allocations larger than the size-class pools.
    pub(crate) fn heap_init_3(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = 0x0031_ff60;
        ee::s10::f_0031ff60(c);
        assert_eq!(
            c.pc,
            crate::difftest::RA as u32,
            "DLHeap_InitArena did not return"
        );
    }


    /// The engine's size-class pool_5 allocator: `gp + 0xdd8` ready plus eight
    /// free nodes per size class (a zeroed heap walks garbage).
    pub(crate) fn pool_5(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w_7(c, crate::difftest::GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool_5 = DATA + 0x40_0000 + 0x4_0000 * i;
            let base = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base + 0x400 * n;
                w_7(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w_7(c, pool_5 + head, base);
            w_7(c, crate::difftest::GAME_GP + slot, pool_5);
        }
    }


        #[test]
    fn world_block_pool_init_matches() {
        const S: u64 = 0x003a_7598;
        check_all(
            ee::s13::f_003a7598__generated,
            world_block_pool_init,
            40,
            &|c, seed| {
                machine(c, seed, S, 8);
                heap_init_3(c);
                pool_5(c);
                let this = DATA + 0x600;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 4) as u64; // block count
                c.g[A2][0] = 3; // block size (the array is a2 * 0x18)
            },
        );
    }
}
