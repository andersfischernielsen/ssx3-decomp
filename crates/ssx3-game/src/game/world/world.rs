//! `src/world/world.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
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
use crate::events::back_edge_unwinds;
use crate::call::Flow;
use crate::abi::r32;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
pub(crate) use crate::game::fe::festaterewards::at_4 as at;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw;
pub(crate) use crate::game::wscript::wscriptman::p;
pub(crate) use crate::game::wscript::wscriptman::gp;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw_2;
pub(crate) use crate::game::wscript::wscriptman::sw;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_2;
pub(crate) use crate::game::fe::feasyncfile::r;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_6688, world_ctor, "World_ctor"),
    (0x003a_6740, world_dtor, "World_dtor"),
    (0x003a_67e0, world_on_game_enter, "World_OnGameEnter"),
    (0x003a_6b78, world_get_object_script_func, "World_GetObjectScriptFunc"),
    (0x003a_6cc8, world_get_object_section_data, "World_GetObjectSectionData"),
    (0x003a_6ce8, world_get_sphere_tree_at, "World_GetSphereTreeAt"),
    (0x003a_6d00, world_get_table_entry, "World_GetTableEntry"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::object::debouncenode::callee::HAS_OBJECT_SCRIPT1;
    pub(crate) use crate::game::object::debouncenode::callee::HAS_OBJECT_SCRIPT3;
    pub(crate) use crate::game::object::debouncenode::callee::GET_OBJECT_SLOT;
    pub(crate) use crate::game::object::debouncenode::callee::GET_PLAYER_INSTANCE;
    pub(crate) use crate::game::object::debouncenode::callee::GET_HUMAN_RIDER;
    pub(crate) use crate::game::object::debouncenode::callee::GET_CHARACTER;
    pub(crate) use crate::game::object::debouncenode::callee::GET_TRACK_PEAK;
    pub(crate) use crate::game::object::debouncenode::callee::GET_TRANSFORM_BOUNDS;
    pub(crate) use crate::game::object::debouncenode::callee::EVAL_SEGMENT;
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
    pub(crate) use crate::game::ai::rider::callee::NET_FE_STATE_ON_CHILD_EVENT;
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
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_GET_LANGUAGE;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_GET_SURFACE_SOUND_TYPE;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_BASE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_SURFACE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_EDGE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_PLAY_FE_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_TRICK_METER_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_OBJ_DEACTIVATE as SCRIPT_OBJ_DEACTIVATE_2;
    pub(crate) use crate::game::ai::rider::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::ai::rider::callee::CODEC_POP_FREE_FRAME;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::ai::rider::callee::BUY_POPUP_INIT_BUY_BOLT;
    pub(crate) use crate::game::ai::rider::callee::RIDER_GET_START_DIST_TO_FINISH;
    pub(crate) use crate::game::ai::rider::callee::BE_GET_RIDER_PROFILE_2 as BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::ai::rider::callee::BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_ENGINE_GET_PROCESS;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::MALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::VEC3_MAX_COMPONENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_12 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_4 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_3 as NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_SELECTED_USER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_MANAGER_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST_2 as PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT_2 as PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_FREE_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_PARSE_2 as CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_MONTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_DAY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_YEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_GENDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_LEGAL;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_INIT_PAGES;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_REQUEST_SHAPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPTIONS_HUD_REFRESH_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG_2 as MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_BIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_NEXT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_EMPTY_STUB;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_LANGUAGE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRACK_TO_CATEGORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE_2 as AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_SOMETHING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_NEW;
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


/// `int World_GetObjectScriptFunc(WScriptCache* cache, WorldObj* obj,
/// int flag)` (0x3a6b78): the object's script slot — the script table is at
/// `*(*(cache + 4) + 4)`, records are 12 bytes (`(unsigned char)
/// *(obj + 0x78)` indexes them), and the returned value is
/// `tWScriptCache_GetObjectSlot(table + id * 12, obj, flag)`.
pub fn world_get_object_script_func(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = 0xc;
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(abi::arg(c, 1)).off(0x78).lbu(c);
        c.g[V1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
        // v0 = id * 12 (record stride), in the delay slot.
        c.mult(0, 2, 2, 7);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        callee::GET_OBJECT_SLOT.call(c, 0x003a_6b9c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `SphereTree* World_GetSphereTreeAt(World* w, int index)` (0x3a6ce8): the
/// index-th sphere tree (0x140-byte records) of the world's tree array.
pub fn world_get_sphere_tree_at(c: &mut Cpu) {
    c.cycles += 5;
    let selfp = abi::arg(c, 0);
    c.g[V1][0] = 0x140;
    c.g[V0][0] = c.lw(selfp);
    c.mult(0, A1, A1, V1); // a1 = index * 0x140
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    abi::ret(c);
}


/// `void World_OnGameEnter(this)` (0x003a67e0): reset the world's section
/// state.
pub fn world_on_game_enter(c: &mut Cpu) {
    forward(c, 7, 0x10, |_| {}, callee::WORLD_RESET_SECTIONS, 0x003a_67f0)
}


/// `void* World_GetTableEntry(void** table, int index)` (0x3a6d00): entry
/// `index` of the pointer table.
pub fn world_get_table_entry(c: &mut Cpu) {
    c.cycles += 5;
    let table = abi::arg(c, 0);
    c.g[V0][0] = Addr(table).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let entry = Addr(c.g[A1][0] as u32).lw(c);
    c.g[V0][0] = entry;
    abi::ret(c);
}


/// `void* World_GetObjectSectionData(World* w, WorldObject* obj)`
/// (0x3a6cc8): the section data of the object's current model: `obj->+0x88`
/// → `[+0xc]` → `[+8]`.
pub fn world_get_object_section_data(c: &mut Cpu) {
    c.cycles += 4;
    let obj = Addr(abi::arg(c, 1));
    c.g[V0][0] = obj.off(0x88).lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
    let v = Addr(c.g[V1][0] as u32).off(0x8).lw(c);
    c.g[V0][0] = v;
    abi::ret(c);
}


/// `void World_Load(World* self, char* name, u32 flags)` (0x3a6ae0): reset and
/// load the world cache, allocate the script cache from the section count and
/// release the name string.
pub fn world_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[SP][0];
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, table_callee::BXSTRING_CTOR, 0x003a_6b08)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        callee::WORLD_CACHE_RESET.call(c, 0x003a_6b10)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A2][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        callee::WORLD_CACHE_LOAD.call(c, 0x003a_6b20)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        callee::STREAMER_GET_SECTION_COUNT.call(c, 0x003a_6b2c)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
        c.g[A1][0] = c.g[V0][0];
        callee::WSCRIPT_CACHE_ALLOC.call(c, 0x003a_6b38)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        callee::STREAMER_GET_SECTION_COUNT.call(c, 0x003a_6b44)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x8).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, table_callee::SECTION_TABLE_CTOR, 0x003a_6b50)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call_ptr(c, table_callee::BXSTRING_DTOR, 0x003a_6b5c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// World.

/// `void World_dtor(World* self, int flags)` (0x3a6740): stop the global game
/// module, destroy the world cache, script cache and painter, free the sphere
/// tree cache and delete the world when `flags & 1`.
pub fn world_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = lw_2(c, gp(c, 0x2a90));
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = lw_2(c, p(c.g[A2][0] as u32, 0x10d8));
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x358)).lh(c);
        c.g[V1][0] = lw_2(c, p(c.g[V0][0] as u32, 0x35c));
        c.g[V1][0] = sx(c.g[V1][0] as u32);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V1][0] as u32, 0x003a_6770)?;
        c.g[A0][0] = lw_2(c, c.g[S0][0] as u32);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x003a_7e38, 0x003a_6784)?; // cWorldCache_dtor
        }
        c.g[A0][0] = lw_2(c, p(c.g[S0][0] as u32, 4));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x003a_c8f0, 0x003a_6798)?; // cWScriptCache_dtor
        }
        c.g[A0][0] = lw_2(c, p(c.g[S0][0] as u32, 8));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = 3;
            call_ptr(c, 0x002b_b140, 0x003a_67ac)?; // cWorldPainterMan_dtor
        }
        c.cycles += 5;
        call_ptr(c, 0x0032_de20, 0x003a_67b4)?; // World_FreeSphereTreeCache
        c.g[V0][0] = c.g[S1][0] & 1;
        let no_delete = c.g[V0][0] == 0;
        sw(c, gp(c, 0x16c8), 0);
        if !no_delete {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x003a_67c8)?; // __builtin_delete
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// World.

/// `cWorld* cWorld::cWorld()` (0x3a6688): construct the world singleton — the
/// object cache, the script cache, the trigger manager hook object and the
/// path network — and publish it at `gp_2+0x16c8`.
pub fn world_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 45;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        c.g[A1][0] = sx(0x0049_4d58);
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x1c00;
        c.g[RA][0] = sx(0x003a_66b4);
        Addr(gp_2(c, 0x16c8)).sw(c, c.g[S0][0]);
        call_ptr(c, callee::MALLOC, 0x003a_66b4)?;

        c.g[RA][0] = sx(0x003a_66bc);
        c.g[A0][0] = c.g[V0][0];
        callee::WORLD_CACHE_CTOR.call(c, 0x003a_66bc)?;

        c.g[A1][0] = sx(0x0049_4d68);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        Addr(r(c, S0)).sw(c, c.g[V0][0]);
        c.g[RA][0] = sx(0x003a_66d8);
        c.g[A0][0] = 0x10;
        call_ptr(c, callee::MALLOC, 0x003a_66d8)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x003a_66e4);
        c.g[A0][0] = c.g[V0][0];
        callee::WSCRIPT_CACHE_CTOR.call(c, 0x003a_66e4)?;

        c.g[A1][0] = sx(0x0049_4d78);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        Addr(p(r(c, S0), 4)).sw(c, c.g[V0][0]);
        c.g[RA][0] = sx(0x003a_6700);
        c.g[A0][0] = 8;
        call_ptr(c, callee::MALLOC, 0x003a_6700)?;

        c.g[RA][0] = sx(0x003a_6708);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002b_b130, 0x003a_6708)?;

        Addr(p(r(c, S0), 0x10)).sw(c, 0);
        c.g[A0][0] = 0xa;
        Addr(p(r(c, S0), 0x14)).sw(c, 0);
        c.g[A1][0] = 4;
        Addr(p(r(c, S0), 8)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 2;
        c.g[RA][0] = sx(0x003a_6728);
        c.g[A3][0] = 5;
        call_ptr(c, 0x0032_dc10, 0x003a_6728)?;

        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::check;
    use crate::difftest::put_vec;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::check_lohi;
    use ssx3_ee as ee;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::elf_image;
    use crate::difftest::put_words;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::object::debouncenode::tests::SEEDS;
    pub(crate) use crate::game::object::debouncenode::tests::base;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::mixed;
    pub(crate) use crate::game::lib::sif::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::ai::ai::tests::rng as rng_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::w;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3 as BE;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_2;
    pub(crate) use crate::game::bx::bxstring::tests::heap;
    pub(crate) use crate::game::dirtysock::tags2::tests::pool_7 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_8 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_6 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_11 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_9 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_11 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_11 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_4 as alloc_base;


    /// `wscript` fixture addresses.
    pub(crate) const WORLD: u32 = DATA + 0x200;

    pub(crate) const TABLE: u32 = DATA + 0x300;

    pub(crate) const SLOT1: u32 = DATA + 0x400;

    pub(crate) const SLOT2: u32 = DATA + 0x440;

    pub(crate) const BASE: u32 = DATA + 0x600;


    /// `*(man + 0x28c) -> cache -> +4 -> world -> +4 -> table`, with the
    /// object's 12-byte table record's type halfword.
    pub(crate) fn put_script_cache(c: &mut Cpu, man: u32, cache: u32, obj: u32, id: u8, kind: u16) {
        c.bus.write32(man + 0x28c, cache);
        c.bus.write32(cache + 4, WORLD);
        c.bus.write32(WORLD + 4, TABLE);
        c.bus.write8(obj + 0x78, id);
        c.bus.write16(TABLE + 12 * id as u32, kind);
    }


    /// `tWScriptCache_GetObjectSlot` fixture: `obj + 0x88 -> +8` indexes
    /// `BASE + 4 * flag`, through `slot + 8 -> +0x1c -> BASE`.
    pub(crate) fn put_object_slots(c: &mut Cpu, obj: u32, slot: u32, word1: u32, word3: u32) {
        c.bus.write32(obj + 0x88, SLOT1);
        c.bus.write32(SLOT1 + 8, 0);
        c.bus.write32(slot + 8, SLOT2);
        c.bus.write32(SLOT2 + 0x1c, BASE);
        c.bus.write32(BASE + 4, word1);
        c.bus.write32(BASE + 12, word3);
    }


    #[test]
    fn world_get_object_script_func_matches() {
        check(ee::s13::f_003a6b78__generated, world_get_object_script_func, SEEDS, &|c, seed| {
            let _r = base(c, seed, 0x3a6b);
            let (cache, obj) = (DATA, DATA + 0x140);
            let id = (seed % 3) as u8;
            put_script_cache(c, DATA + 0x700, cache, obj, id, 3);
            let slot = TABLE + 12 * id as u32;
            match seed % 4 {
                0 => c.bus.write32(obj + 0x88, 0),
                1 => {
                    c.bus.write32(obj + 0x88, SLOT1);
                    c.bus.write32(SLOT1 + 8, 0);
                }
                2 => {
                    c.bus.write32(obj + 0x88, SLOT1);
                    c.bus.write32(SLOT1 + 8, 0xffff_ffff);
                    c.bus.write32(slot + 8, SLOT2);
                    c.bus.write32(SLOT2 + 0x1c, BASE);
                }
                _ => put_object_slots(c, obj, slot, 0x1234, 0x4321),
            }
            c.g[A0][0] = sx(cache);
            c.g[A1][0] = sx(obj);
            c.g[A2][0] = if seed % 2 == 0 { 1 } else { 3 };
        });
    }


    #[test]
    fn world_get_sphere_tree_at_matches() {
        check_lohi(ee::s13::f_003a6ce8__generated, world_get_sphere_tree_at, SEEDS_2, &|c, seed| {
            let mut r = rng(0x003a_6ce8, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA, DATA + 0x2000);
            c.g[A1][0] = mixed(&mut r, seed, (seed % 8) as u32);
        });
    }


    #[test]
    fn world_on_game_enter_matches() {
        check(ee::s13::f_003a67e0__generated, world_on_game_enter, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x3a_67e0, 0);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            match seed % 3 {
                0 => {}
                1 => {
                    c.bus.write32(DATA + 0x1000, DATA + 0x2000);
                    c.bus.write32(DATA + 0x1000 + 0x10 + 4, 0);
                }
                _ => {
                    c.bus.write32(DATA + 0x1000, DATA + 0x2000);
                    c.bus.write32(DATA + 0x1000 + 0x10 + 4, DATA + 0x3000);
                    c.bus.write32(DATA + 0x3000 + 8, r.next_u32() % 8);
                }
            }
        });
    }


    #[test]
    fn world_get_table_entry_matches() {
        check(ssx3_ee::s13::f_003a6d00__generated, world_get_table_entry, SEEDS_3, &|c, seed| {
            let mut r = rng_2(seed, 32);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(DATA, DATA + 0x800);
            for i in 0..0x400 {
                c.bus.write32(DATA + 0x800 + i * 4, r.next_u32());
            }
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (r.next_u32() & 0x3ff) as u64;
        });
    }


    #[test]
    fn world_get_object_section_data_matches() {
        check(ssx3_ee::s13::f_003a6cc8__generated, world_get_object_section_data, SEEDS_3, &|c, seed| {
            let mut r = rng_2(seed, 33);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(DATA + 0x88, DATA + 0x200);
            c.bus.write32(DATA + 0x20c, DATA + 0x300);
            c.bus.write32(DATA + 0x308, r.next_u32());
            c.g[A1][0] = DATA as u64;
        });
    }


    // -------------------------------------------------------------- world

    #[test]
    fn world_dtor_matches() {
        check(ssx3_ee::s13::f_003a6740__generated, world_dtor, 8, &|c, seed| {
            let _ = base_2(c, seed, 0x003a_6740);
            heap(c);
            pool(c);
            let world = DATA + 0x100;
            let mgr = DATA + 0x5000;
            let vtable = DATA + 0x6000;
            clear(c, world, 0x100);
            clear(c, mgr, 0x1200);
            clear(c, vtable, 0x200);
            clear(c, DATA + 0x2000, 0x400);
            clear(c, DATA + 0x3000, 0x400);
            clear(c, DATA + 0x4000, 0x400);
            if seed % 2 == 0 {
                w(c, world, DATA + 0x2000);
                w(c, world + 4, DATA + 0x3000);
                w(c, world + 8, DATA + 0x4000);
            }
            w(c, GAME_GP.wrapping_add(0x2a90), mgr);
            w(c, mgr + 0x10d8, vtable);
            c.bus.write16(vtable + 0x358, 0);
            w(c, vtable + 0x35c, crate::difftest::fakes::FAKE);
            w(c, crate::difftest::fakes::LOG_RESULTS, 1);
            c.g[A0][0] = sx(world);
            c.g[A1][0] = (seed % 3) as u64;
        });
    }


    // ------------------------------------------------------- world / triggers

    #[test]
    fn world_ctor_matches() {
        check(ee::s13::f_003a6688__generated, world_ctor, 60, &|c, seed| {
            alloc_base(c, seed, 0x3a66, 5);
            c.g[A0][0] = DATA as u64;
        });
    }
}
