//! `src/be/beintbagt.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::beintmission::lw;
pub(crate) use crate::game::be::beintmission::BUILTIN_DELETE;
pub(crate) use crate::game::be::beintmission::EA_MEM_ALLOC_NAMED;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::scripter::ssxscriptengine::p;
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
        0x0014_f900,
        be_bagt_int_get_instance,
        "BEBAGTInt_GetInstance",
    ),
    (
        0x0014_fd80,
        bebagt_int_set_uber_trick_button,
        "BEBAGTInt_SetUberTrickButton",
    ),
    (
        0x0014_fe08,
        be_bagt_int_clear_uber_trick_bit,
        "BEBAGTInt_ClearUberTrickBit",
    ),
    (
        0x0014_ff90,
        be_bagt_int_get_trick_table_entry,
        "BEBAGTInt_GetTrickTableEntry",
    ),
    (
        0x0014_ffb8,
        bebagt_int_test_uber_trick_bit_a,
        "BEBAGTInt_TestUberTrickBitA",
    ),
    (
        0x0015_0048,
        bebagt_int_test_uber_trick_bit_b,
        "BEBAGTInt_TestUberTrickBitB",
    ),
    (
        0x0015_00d8,
        bebagtint_get_trick_field0,
        "BEBAGTInt_GetTrickField0",
    ),
    (
        0x0015_00f8,
        bebagtint_get_trick_field2,
        "BEBAGTInt_GetTrickField2",
    ),
    (
        0x0015_0118,
        bebagtint_get_trick_field4,
        "BEBAGTInt_GetTrickField4",
    ),
    (
        0x0015_0138,
        bebagtint_get_trick_field6,
        "BEBAGTInt_GetTrickField6",
    ),
    (
        0x0015_0158,
        bebagtint_get_trick_field8,
        "BEBAGTInt_GetTrickField8",
    ),
    (
        0x0015_0178,
        bebagtint_get_trick_fielda,
        "BEBAGTInt_GetTrickFieldA",
    ),
    (
        0x0015_c6e8,
        be_bagt_int_destroy_instance,
        "BEBAGTInt_DestroyInstance",
    ),
];

// ---------------------------------------------------------------------------
// Callees the generator emitted as direct calls (the readable alias, which
// dispatches through the override slot) and as cross-shard `call_indirect`s.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::FE_ASYNC_SET_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2 as GAME_MODULE_REQUEST_STATE;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_IS_TRACK_GROUP0_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_RESET_RACE_RESULT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STORE_RACE_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COLLECTED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COMPLETED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_CHARACTER_2 as BE_GET_RIDER_CHARACTER_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_SLOT_3 as BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHALLENGE_POPUP_SET_SETTINGS_LOCKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_GROUP_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEAD_NODE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_ABORT_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_EMAIL_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_LOGIN_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_LOAD_UPDATE_PLAYER_RANKING;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_LOAD_STATE_2 as GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_SKIP_WHITESPACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LIGHT_MAN_EMPTY_HOOK_D;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_ON_CARD_SCANNED;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_CARD_STATE_BASE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS_2 as MPEG2_GET_BITS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT_2 as MPEG2_MARKER_BIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_11 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_4 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_12 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_3 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID_2 as POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_GET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_BUILD_WORLD_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_UPDATE_BOARD_LEAN;
    pub(crate) use crate::game::fe::feasyncfile::callee::SETUP_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_FLUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_TREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_DEACTIVATE;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_CURSOR_2 as KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_ADD_VERTEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_CLOSE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MATH_EVAL_CUBIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_MAN_FREE_ENTRY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_DRAW_ROW_4 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4 as SCRIPT_DATA_GET_INFO;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_FIRST_5 as UI_LIST_FIRST_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_SCREEN_FIND_OBJECT_3 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
}

/// `void BEBAGTInt_DestroyInstance(void)` (0x15c6e8).
pub fn be_bagt_int_destroy_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1ee0));
        frame.save_ra(c, 0);
        call_ptr(c, BUILTIN_DELETE, 0x0015_c6fc)?;
        frame.restore_ra(c, 0);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1ee0), 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BAGT singleton.

/// `BEBAGTInt *BEBAGTInt_GetInstance(void)` (0x14f900): lazily allocate the
/// singleton, install its vtable and run the vtable's constructor slot (+0x10).
pub fn be_bagt_int_get_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1ee0));
        let have = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !have {
            c.cycles += 16;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = 0x10;
            c.g[A1][0] = sx(0x460000u32.wrapping_sub(0x5928));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, EA_MEM_ALLOC_NAMED, 0x0014_f928)?;
            c.g[V1][0] = 0x460000;
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1ee0), c.g[V0][0]);
            c.g[V1][0] = sx(0x460000u32.wrapping_sub(0x5328));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = lw(c, (c.g[V1][0] as u32).wrapping_add(0x14));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            let target = c.g[A1][0] as u32;
            call_ptr(c, target, 0x0014_f948)?;
            c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1ee0));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
        }
        c.cycles += 4;
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1ee0));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `D_0045AEB8`: the BE BAGT trick table (0x14-byte entries).
pub(crate) const TRICK_TABLE: u32 = 0x0045_aeb8;

/// `u16 BEBAGTInt_GetTrickField0(int unused, int unused, int index)`
/// (0x1500d8): the first halfword of a BAGT trick entry (0x14-byte records in
/// `D_0045AEB8`).
pub fn bebagtint_get_trick_field0(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = 0x14;
    c.g[V0][0] = 0x46_0000;
    c.mult(0, A2, A2, V1); // a2 = index * 0x14
    c.g[V0][0] = sx(TRICK_TABLE);
    let base = c.g[V0][0] as u32;
    c.g[A2][0] = sx(base.wrapping_add(c.g[A2][0] as u32));
    let entry = c.g[A2][0] as u32;
    c.g[V0][0] = c.lhu(entry);
    abi::ret(c);
}

/// `u16 BEBAGTInt_GetTrickField2(...)` (0x1500f8): halfword +2 of a trick
/// entry.
pub fn bebagtint_get_trick_field2(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = 0x14;
    c.g[V0][0] = 0x46_0000;
    c.mult(0, A2, A2, V1);
    c.g[V0][0] = sx(TRICK_TABLE);
    let base = c.g[V0][0] as u32;
    c.g[A2][0] = sx(base.wrapping_add(c.g[A2][0] as u32));
    let entry = c.g[A2][0] as u32;
    c.g[V0][0] = c.lhu(entry.wrapping_add(2));
    abi::ret(c);
}

/// `u16 BEBAGTInt_GetTrickField4(...)` (0x150118): halfword +4 of a trick
/// entry.
pub fn bebagtint_get_trick_field4(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = 0x14;
    c.g[V0][0] = 0x46_0000;
    c.mult(0, A2, A2, V1);
    c.g[V0][0] = sx(TRICK_TABLE);
    let base = c.g[V0][0] as u32;
    c.g[A2][0] = sx(base.wrapping_add(c.g[A2][0] as u32));
    let entry = c.g[A2][0] as u32;
    c.g[V0][0] = c.lhu(entry.wrapping_add(4));
    abi::ret(c);
}

/// `u16 BEBAGTInt_GetTrickField6(...)` (0x150138): halfword +6 of a trick
/// entry.
pub fn bebagtint_get_trick_field6(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = 0x14;
    c.g[V0][0] = 0x46_0000;
    c.mult(0, A2, A2, V1);
    c.g[V0][0] = sx(TRICK_TABLE);
    let base = c.g[V0][0] as u32;
    c.g[A2][0] = sx(base.wrapping_add(c.g[A2][0] as u32));
    let entry = c.g[A2][0] as u32;
    c.g[V0][0] = c.lhu(entry.wrapping_add(6));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// BAGT trick records: 20-byte records at `D_0045AEB8`.

/// `D_0045AEB8`: the BAGT trick table.
pub(crate) const BAGT_TRICKS: u32 = 0x0045_aeb8;

/// `unsigned short BEBAGTInt_GetTrickField8(int idx)` (0x00150158): the u16
/// field at `+0x8` of trick record `idx`.
pub fn bebagtint_get_trick_field8(c: &mut Cpu) {
    bagt_trick_field(c, 0x8);
}

/// `unsigned short BEBAGTInt_GetTrickFieldA(int idx)` (0x00150178): the u16
/// field at `+0xa` of trick record `idx`.
pub fn bebagtint_get_trick_fielda(c: &mut Cpu) {
    bagt_trick_field(c, 0xa);
}

/// `p = &D_0045AEB8 + idx * 0x14; return *(u16 *)(p + off)`.
pub(crate) fn bagt_trick_field(c: &mut Cpu, off: u32) {
    c.cycles += 7;
    c.g[V1][0] = 0x14;
    c.g[V0][0] = 0x46_0000;
    // a2 = idx * 20 (the multiply leaves LO/HI set, as the original does).
    c.mult(0, A2, A2, V1);
    // `v0 = 0x460000 - 0x5148`: the low half of `&D_0045AEB8`.
    c.g[V0][0] = sx(BAGT_TRICKS);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(off));
    c.pc = ra;
}

/// `sTrick *BEBAGTInt_GetTrickTableEntry(void *this, int rider, int idx,
/// int *count)` (0x14ff90): entry `idx` of the 0x14-byte trick table
/// (`D_0045AEB8`), its `+0x10` value in v0 and its `+0xc` count stored in
/// `*count`.
pub fn be_bagt_int_get_trick_table_entry(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = 0x14; // entry stride
    c.g[V1][0] = 0x0046_0000;
    c.mult(0, A2, A2, V0);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5148)); // &D_0045AEB8
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lhu(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x10)).lw(c);
    let ret = c.g[RA][0] as u32;
    c.sw(c.g[A3][0] as u32, c.g[A0][0]);
    c.pc = ret;
}

/// `void BEBAGTInt_SetUberTrickButton(BEBAGTInt* self, int player, u32 id,
/// u8 button)` (0x14fd80): write the button byte into the per-character row
/// of the bag of tricks (`0x52eec0 + profile*0x13ec + player*6 + id`).
pub fn bebagt_int_set_uber_trick_button(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        open(c, 0x50);
        let sp0 = spv(c);
        c.sq(18, at(sp0, 0x20));
        c.g[S2][0] = c.g[A1][0];
        c.sq(16, at(sp0, 0x40));
        c.sq(19, at(sp0, 0x10));
        c.g[S0][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A3][0];
        c.sq(17, at(sp0, 0x30));
        c.sd(sp0, c.g[RA][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::BE_GET_RIDER_PROFILE.call(c, 0x0014_fdac)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[S1][0] = c.g[V0][0];
        callee::BE_GET_RIDER_CHARACTER.call(c, 0x0014_fdb8)?;
        c.g[V1][0] = 0x1fe;
        c.g[A0][0] = 6;
        c.mult(0, 5, 2, 3);
        c.mult(0, 16, 16, 4);
        c.g[V0][0] = 0x13ec;
        c.g[V1][0] = 0x530000;
        c.mult(0, 17, 17, 2);
        c.g[V1][0] = sx(at(c.g[V1][0] as u32, 0xec0));
        c.g[V0][0] = 1;
        c.g[S0][0] = sx(at(c.g[S0][0] as u32, c.g[A1][0] as u32));
        c.g[RA][0] = c.ld(sp0);
        c.g[S0][0] = sx(at(c.g[S0][0] as u32, c.g[S1][0] as u32));
        c.g[S0][0] = sx(at(c.g[S0][0] as u32, c.g[V1][0] as u32));
        c.lq(18, at(sp0, 0x20));
        c.sb(at(c.g[S0][0] as u32, 1), c.g[S3][0]);
        c.lq(17, at(sp0, 0x30));
        c.lq(16, at(sp0, 0x40));
        c.lq(19, at(sp0, 0x10));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x50));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE bag tricks.

/// `int BEBAGTInt_TestUberTrickBitA(void*, int rider, int idx, int bit)`
/// (0x14ffb8): bit `bit` of `flags` in the rider's character bag-trick table.
pub fn bebagt_int_test_uber_trick_bit_a(c: &mut Cpu) {
    bebagt_test_bit(c, 2);
}

/// `int BEBAGTInt_TestUberTrickBitB(void*, int rider, int idx, int bit)`
/// (0x150048): the same for the second flag word.
pub fn bebagt_int_test_uber_trick_bit_b(c: &mut Cpu) {
    bebagt_test_bit(c, 4);
}

/// The shared body of the two "test a bag-trick bit" wrappers.
pub(crate) fn bebagt_test_bit(c: &mut Cpu, flag_offset: u32) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S2][0];
        callee::BE_GET_RIDER_PROFILE_2.call(c, 0x0014_ffe4)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[S1][0] = c.g[V0][0];
        callee::BE_GET_RIDER_CHARACTER_2.call(c, 0x0014_fff0)?;
        c.g[A1][0] = 6;
        c.g[V1][0] = 0x1fe;
        c.mult(0, V0, V0, V1); // character * 510
        c.mult(0, S0, S0, A1); // index * 6
        c.g[A0][0] = 0x13ec;
        c.g[V1][0] = 0x0053_0000;
        c.mult(0, S1, S1, A0); // profile * 0x13ec
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0xec0));
        frame.restore(c, S2, 0x20);
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
        frame.restore_ra(c, 0);
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, S1)));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, S0)));
        frame.restore(c, S1, 0x30);
        c.g[V0][0] = Addr(p(r32(c, V1), flag_offset)).lhu(c);
        frame.restore(c, S0, 0x40);
        c.g[V0][0] = sx(((r32(c, V0) as i32) >> (r32(c, S3) & 31)) as u32);
        frame.restore(c, S3, 0x10);
        c.g[V0][0] = c.g[V0][0] & 1;
        frame.close(c);
        Ok(())
    })
}

/// `void BEBAGTInt_ClearUberTrickBit(rider, a1, slot, bit)` (0x14fe08): clear
/// the given bit in the rider's bag trick record.
pub fn be_bagt_int_clear_uber_trick_bit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        callee::BE_GET_RIDER_PROFILE_3.call(c, 0x0014_fe34)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[S2][0] = c.g[V0][0];
        callee::BE_GET_RIDER_CHARACTER_3.call(c, 0x0014_fe40)?;
        c.g[V1][0] = 0x1fe;
        c.g[A0][0] = 6;
        c.mult(0, V0, V0, V1);
        c.mult(0, S0, S0, A0);
        c.g[V1][0] = 0x13ec;
        c.g[A1][0] = 0x0053_0ec2;
        c.mult(0, S2, S2, V1);
        c.g[A0][0] = 1;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << (c.g[S3][0] as u32 & 31));
        c.g[A0][0] = !(c.g[A0][0] | 0);
        frame.restore(c, S1, 0x30);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        frame.restore(c, S3, 0x10);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        frame.restore(c, S2, 0x20);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lhu(c);
        c.g[V0][0] = 1;
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        c.g[V1][0] &= c.g[A0][0];
        Addr(c.g[S0][0] as u32).sh(c, c.g[V1][0]);
        frame.restore(c, S0, 0x40);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as RET;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD;
    pub(crate) use crate::game::sound::bankmanager::tests::mixed;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn be_bagt_int_get_instance_matches() {
        check(
            ssx3_ee::s01::f_0014f900,
            be_bagt_int_get_instance,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0xf900, 0);
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x1ee0),
                    if seed % 3 == 0 { 0 } else { DATA + 0x100 },
                );
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn bebagtint_get_trick_field0_matches() {
        check_lohi(
            ee::s01::f_001500d8__generated,
            bebagtint_get_trick_field0,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0015_00d8, seed);
                scramble(c, &mut r);
                let index = (seed % 8) as u32;
                c.g[A2][0] = mixed(&mut r, seed, index);
                c.bus
                    .write16(TRICK_TABLE + index * 0x14, r.next_u32() as u16);
            },
        );
    }

    #[test]
    fn bebagtint_get_trick_field2_matches() {
        check_lohi(
            ee::s01::f_001500f8__generated,
            bebagtint_get_trick_field2,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0015_00f8, seed);
                scramble(c, &mut r);
                let index = (seed % 8) as u32;
                c.g[A2][0] = mixed(&mut r, seed, index);
                c.bus
                    .write16(TRICK_TABLE + index * 0x14 + 2, r.next_u32() as u16);
            },
        );
    }

    #[test]
    fn bebagtint_get_trick_field4_matches() {
        check_lohi(
            ee::s01::f_00150118__generated,
            bebagtint_get_trick_field4,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0015_0118, seed);
                scramble(c, &mut r);
                let index = (seed % 8) as u32;
                c.g[A2][0] = mixed(&mut r, seed, index);
                c.bus
                    .write16(TRICK_TABLE + index * 0x14 + 4, r.next_u32() as u16);
            },
        );
    }

    #[test]
    fn bebagtint_get_trick_field6_matches() {
        check_lohi(
            ee::s01::f_00150138__generated,
            bebagtint_get_trick_field6,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0015_0138, seed);
                scramble(c, &mut r);
                let index = (seed % 8) as u32;
                c.g[A2][0] = mixed(&mut r, seed, index);
                c.bus
                    .write16(TRICK_TABLE + index * 0x14 + 6, r.next_u32() as u16);
            },
        );
    }

    // -----------------------------------------------------------------------
    // BAGT trick records.

    pub(crate) fn trick_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x1501_58 ^ seed as u64);
        scramble(c, &mut r);
        let idx = (r.next_u32() % 40) as i32 - 4;
        c.g[A2][0] = sx(idx as u32);
        let base = 0x0045_aeb8u32.wrapping_add((idx as u32).wrapping_mul(0x14));
        for k in 0..10u32 {
            c.bus.write16(base.wrapping_add(2 * k), r.next_u32() as u16);
        }
    }

    #[test]
    fn bebagtint_get_trick_field8_matches() {
        check_lohi(
            ee::s01::f_00150158__generated,
            bebagtint_get_trick_field8,
            200,
            &trick_setup,
        );
    }

    #[test]
    fn bebagtint_get_trick_fielda_matches() {
        check_lohi(
            ee::s01::f_00150178__generated,
            bebagtint_get_trick_fielda,
            200,
            &trick_setup,
        );
    }

    #[test]
    fn be_bagt_int_get_trick_table_entry_matches() {
        check(
            ee::s01::f_0014ff90__generated,
            be_bagt_int_get_trick_table_entry,
            200,
            &|c, seed| {
                let mut r = Rng(0x14ff_9000 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                let index = seed % 8;
                c.g[A2][0] = index as u64;
                c.g[A3][0] = (SCRATCH + 0x1200) as u64;
                let table = 0x0045_aeb8u32;
                for i in 0..8u32 {
                    let e = table + 0x14 * i;
                    c.bus.write16(e + 0xc, r.next_u32() as u16);
                    c.bus.write32(e + 0x10, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn bebagt_int_set_uber_trick_button_matches() {
        check(
            ee::s01::f_0014fd80__generated,
            bebagt_int_set_uber_trick_button,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x14fd80, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 20) as u64;
                c.g[A2][0] = sx(DATA);
                c.g[A3][0] = (seed % 256) as u64;
                // The race setup table: player 0 in slot 0, whose record has a
                // profile word of 0 and character 0.
                c.bus.write32(0x0053_05b0, 0);
                c.bus.write32(0x0053_5b20 + 0xc, 0);
                c.bus.write8(0x0053_5b20 + 0x11, 0);
            },
        );
    }

    #[test]
    fn bebagt_int_test_uber_trick_bit_a_matches() {
        check(
            ee::s01::f_0014ffb8__generated,
            bebagt_int_test_uber_trick_bit_a,
            100,
            &|c, seed| {
                let _ = base(c, seed, 0x14_ffb8);
                globals(c);
                w(c, DATA, if seed % 2 == 0 { 0xffff } else { 0xaaaa });
                c.g[A0][0] = 0;
                c.g[A1][0] = sx(seed % 8);
                c.g[A2][0] = sx(seed % 4);
                c.g[A3][0] = sx(seed % 16);
            },
        );
    }

    #[test]
    fn bebagt_int_test_uber_trick_bit_b_matches() {
        check(
            ee::s01::f_00150048__generated,
            bebagt_int_test_uber_trick_bit_b,
            100,
            &|c, seed| {
                let _ = base(c, seed, 0x15_0048);
                globals(c);
                w(c, DATA, if seed % 2 == 0 { 0xffff } else { 0x5555 });
                c.g[A0][0] = 0;
                c.g[A1][0] = sx(seed % 8);
                c.g[A2][0] = sx(seed % 4);
                c.g[A3][0] = sx(seed % 16);
            },
        );
    }

    #[test]
    fn be_bagt_int_clear_uber_trick_bit_matches() {
        check_lohi(
            ssx3_ee::s01::f_0014fe08__generated,
            be_bagt_int_clear_uber_trick_bit,
            10,
            &|c, seed| {
                let _r = base_2(c, seed, 0x14fe08, 3);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (seed % 4) as u64;
                c.g[A3][0] = (seed % 16) as u64;
                for i in 0..0x2000u32 {
                    c.bus.write8(0x0053_0ec2 + i, seed.wrapping_add(i) as u8);
                }
            },
        );
    }
}
