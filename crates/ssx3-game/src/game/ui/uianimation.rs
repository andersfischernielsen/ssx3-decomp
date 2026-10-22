//! `src/ui/uianimation.cpp`: hand-written replacements (moved from the
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
use crate::call::Callee;
use crate::call::Flow;
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use crate::abi::r32;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::shl32;
pub(crate) use crate::game::render::graphicsman::RegMem;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_7140, ui_anim_set_create_anim, "UIAnimSet_CreateAnim"),
    (0x0039_71c8, uianim_eval_channel, "UIAnim_EvalChannel"),
    (0x0039_7278, uianim_get_frame_count_safe, "UIAnim_GetFrameCountSafe"),
    (0x0039_73a8, ui_anim_dtor, "UIAnim_dtor"),
    (0x0039_7440, uianim_is_done, "UIAnim_IsDone"),
    (0x0039_7458, uianim_get_frame_count, "UIAnim_GetFrameCount"),
    (0x0039_7468, uianim_set_play_mode, "UIAnim_SetPlayMode"),
    (0x0039_74b0, ui_anim_advance, "UIAnim_Advance"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_ADD_BOOST_2 as RIDER_ADD_BOOST;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOAD_INDICATOR_RENDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SECTION_SELECT_PEAK_LIGHTING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_PROCESS_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_GET_ROOM_NAME;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::FE_ASYNC_SET_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::POPUP_CONFIRM_REBUILD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_HASH;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SET_ITEM_YEAR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_ASCII;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MSG_PRINTF;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STRLEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::QSORT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2 as GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_API_SEND;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GATHER_CAM_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MEM_REPORT_STATS_LOCKED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SERIALIZE_READ_U32;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_4 as EA_MEM_FREE;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2 as BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_CHAR_PROFILE_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_IS_FLAG4;
    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2 as SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
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


// ---------------------------------------------------------------------------
// UI animation

/// `int UIAnim_GetFrameCountSafe(UIAnim **anim)` (0x397278): frame count at
/// `*anim + 2`, or 0 when the animation is null.
pub fn uianim_get_frame_count_safe(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A0][0] = Addr(abi::arg(c, 0)).lw(c);
    c.g[V0][0] = 0;
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x2).lhu(c);
    abi::ret(c);
}


/// `bool UIAnim_IsDone(UIAnim *anim)` (0x397440): true when the current frame
/// (signed 16-bit at +0xa) has reached the frame count at `*anim + 2`.
pub fn uianim_is_done(c: &mut Cpu) {
    c.cycles += 6;
    let anim = Addr(abi::arg(c, 0));
    c.g[A1][0] = anim.lw(c);
    c.g[V0][0] = anim.off(0xa).lh(c);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x2).lhu(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    c.g[V0][0] = c.g[V0][0] ^ 0x1;
    abi::ret(c);
}


/// `int UIAnim_GetFrameCount(UIAnim *anim)` (0x397458): signed frame count at
/// `*anim + 2`.
pub fn uianim_get_frame_count(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = Addr(abi::arg(c, 0)).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x2).lh(c);
    abi::ret(c);
}


/// `void UIAnim_SetPlayMode(UIAnim *anim, int mode, int loops)` (0x397468):
/// set the play mode byte and initialise the loop counter.
pub fn uianim_set_play_mode(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A3][0] = c.g[A0][0];
    c.g[A1][0] = c.g[A1][0] & 0x7;
    c.g[A2][0] = c.g[A2][0] & 0xffff;
    if c.g[A1][0] == 0 {
        c.cycles += 1;
        Addr(c.g[A3][0] as u32).off(0xc).sh(c, c.g[A2][0]);
    }
    c.cycles += 3;
    c.g[V0][0] = 0x2;
    if c.g[A1][0] != c.g[V0][0] {
        Addr(c.g[A3][0] as u32).off(0xa).sh(c, 0);
        c.cycles += 2;
        Addr(c.g[A3][0] as u32).off(0xe).sb(c, 0);
        abi::ret(c);
        return;
    }
    c.cycles += 7;
    c.g[V0][0] = 0x1;
    c.g[A0][0] = Addr(c.g[A3][0] as u32).lw(c);
    Addr(c.g[A3][0] as u32).off(0xe).sb(c, c.g[V0][0]);
    c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x2).lhu(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
    Addr(c.g[A3][0] as u32).off(0xa).sh(c, c.g[V1][0]);
    abi::ret(c);
}


/// `UIAnim* UIAnimSet_CreateAnim(UIAnimSet* self, u32 id)` (0x397140): find
/// the animation entry with `id`, allocate it and run the animation
/// constructor. 0 when there is no such entry.
pub fn ui_anim_set_create_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        open(c, 0x20);
        let sp0 = spv(c);
        c.g[A3][0] = c.g[A0][0];
        c.sq(16, at(sp0, 0x10));
        c.g[A2][0] = 0;
        c.sd(sp0, c.g[RA][0]);
        c.g[V0][0] = lw_at(c, c.g[A3][0], 0);
        let set = c.g[V0][0];
        c.g[V1][0] = lw_at(c, set, 0);
        let empty = c.g[V1][0] == 0;
        c.g[S0][0] = sx(at(set as u32, 4));
        let mut made = false;
        if !empty {
            c.cycles += 1;
            c.g[T0][0] = 0x4a0000;
            loop {
                c.cycles += 3;
                let ent = c.g[S0][0];
                c.g[V0][0] = lw_at(c, ent, 8);
                if c.g[V0][0] != c.g[A1][0] {
                    c.g[A0][0] = lw_at(c, c.g[A3][0], 0);
                } else {
                    c.cycles += 10;
                    c.g[A1][0] = sx(at(c.g[T0][0] as u32, 0x4768));
                    c.g[A0][0] = 0x14;
                    c.g[A2][0] = 0x100;
                    c.g[A3][0] = 0;
                    call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0039_7188)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    callee::UI_ANIM_CTOR.call(c, 0x0039_7194)?;
                    c.lq(16, at(sp0, 0x10));
                    made = true;
                    break;
                }
                c.cycles += 6;
                c.g[A2][0] = sx(at(c.g[A2][0] as u32, 1));
                c.g[V1][0] = lw_at(c, c.g[S0][0], 0xc);
                c.g[V0][0] = lw_at(c, c.g[A0][0], 0);
                c.g[V0][0] = (c.g[A2][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(at(c.g[S0][0] as u32, c.g[V1][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_7168);
                }
            }
        }
        if !made {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.lq(16, at(sp0, 0x10));
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x20));
        c.pc = ra;
        Ok(())
    })
}


/// `UIAnim_dtor` (0x3973a8): release the animation's element array (when
/// present) and optionally free the object itself (`a1 & 1`).
pub fn ui_anim_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_add(0x4d08));
        frame.save(c, S1, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        Addr(p(this, 0x10)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(p(this, 4)).lw(c);
        c.g[S1][0] = c.g[A1][0];
        if c.g[V1][0] != 0 {
            c.cycles += 6;
            let start = r(c, V1);
            c.g[V0][0] = Addr(p(start, 0u32.wrapping_sub(0x10))).lw(c);
            c.g[A0][0] = sx(start);
            c.g[V0][0] = shl32(c.g[V0][0], 2);
            c.g[V0][0] = sx(start.wrapping_add(r(c, V0)));
            let empty = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = sx(r(c, V0).wrapping_sub(4));
            if !empty {
                c.cycles += 1;
                loop {
                    c.cycles += 7;
                    let more = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(4));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0039_73f0);
                        }
                    } else {
                        break;
                    }
                }
            }
            c.cycles += 2;
            c.g[RA][0] = sx(0x0039_7414);
            c.g[A0][0] = sx(start.wrapping_sub(0x10));
            callee::EA_MEM_FREE.call(c, 0x0039_7414)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0039_7428);
            c.g[A0][0] = c.g[S0][0];
            callee::BUILTIN_DELETE.call(c, 0x0039_7428)?;
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
// `UIAnim_EvalChannel` (0x3971c8): evaluate one animation channel at frame
// `t`: find the key interval containing it and interpolate; 0 outside.
pub fn uianim_eval_channel(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A0][0] = c.lw((c.g[A0][0] as u32));
    if c.g[A0][0] == 0 {
        c.cycles += 3;
        c.mtc1(0, 0);
        abi::ret(c);
        return;
    }
    c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0xa));
    c.cycles += 10;
    c.g[T0][0] = 0x1;
    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x6));
    c.g[T2][0] = 0u64;
    c.mtc1(2, 3);
    c.cvt_s_w(3, 3);
    c.g[A3][0] = 0u64;
    c.mtc1(0, 1);
    c.g[V0][0] = (c.g[T0][0] < c.g[V1][0]) as u64;
    let single = c.g[V0][0] == 0;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
    if !single {
        c.cycles += 2;
        c.g[T1][0] = c.g[V1][0];
        loop {
            c.cycles += 8;
            c.g[A2][0] = c.lhu((c.g[A0][0] as u32));
            c.g[V1][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x2));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A2][0] as u32)));
            c.mtc1(3, 1);
            c.cvt_s_w(1, 1);
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let past = c.g[V0][0] == 0;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
            if past {
                c.cycles += 10;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[T2][0] as u32)));
                c.mtc1(6, 2);
                c.cvt_s_w(2, 2);
                c.mtc1(2, 0);
                c.cvt_s_w(0, 0);
                c.sub_s(1, 1, 3);
                c.div_s(0, 0, 2);
                c.mul_s(0, 0, 1);
                c.add_s(0, 0, 3);
                abi::ret(c);
                return;
            }
            c.cycles += 5;
            c.g[T2][0] = c.g[A3][0];
            c.mov_s(3, 1);
            c.g[V0][0] = (c.g[T0][0] < c.g[T1][0]) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0039_7238);
            }
        }
    }
    c.cycles += 2;
    c.mov_s(0, 1);
    abi::ret(c);
}


/// `void UIAnim_Advance(UIAnim* self, unsigned flags)` (0x3974b0): step the
/// animation by `flags & 7` (mode 1 forward, 2 back, 3 bounce using
/// `self->dir`), clamping to `data->count`; bit 8 wraps instead of clamping.
/// The frame is at `+0xa`, the direction at `+0xe`, the data at `+0`.
pub fn ui_anim_advance(c: &mut Cpu) {
    'b_3974e0: {
        c.cycles += 3;
        c.g[A2][0] = c.ldw_2(A0, 0);
        let t = c.g[A2][0] == 0u64;
        c.g[A1][0] = c.g[A1][0] & 0xff;
        if t {
            break 'b_3974e0;
        } else {
            c.cycles += 3;
            c.g[V1][0] = c.g[A1][0] & 0x7;
            let t = c.g[V1][0] == 0u64;
            c.g[V0][0] = 0x2;
            if t {
                break 'b_3974e0;
            } else {
                'b_39752c: {
                    'b_397520: {
                        c.cycles += 2;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                        if t {
                            c.cycles += 5;
                            c.g[V0][0] = c.ldhu_2(A0, 0xa);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            c.g[V1][0] = sx((c.g[V0][0] as u32) << 16);
                            let t = (c.g[V1][0] as i64) >= 0;
                            c.sth_2(A0, 0xa, c.g[V0][0]);
                            if t {
                                break 'b_3974e0;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.g[A1][0] & 0x8;
                                let t = c.g[V0][0] == 0u64;

                                if t {
                                    break 'b_39752c;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = c.ldhu_2(A2, 0x2);
                                    break 'b_397520;
                                }
                            }
                        } else {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x1;
                            if t {
                                c.cycles += 3;
                                c.g[V0][0] = 0x3;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.g[V0][0] = c.ldb(A0, 0xe);
                                    'b_397578: {
                                        'b_397574: {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[V0][0] = c.ldhu_2(A0, 0xa);
                                            if t {
                                                break 'b_397574;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                if t {
                                                    break 'b_397578;
                                                } else {
                                                    break 'b_397574;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                        break 'b_397578;
                                    }
                                    'b_3975b4: {
                                        'b_3975b0: {
                                            c.cycles += 4;
                                            c.sth_2(A0, 0xa, c.g[V0][0]);
                                            c.g[V0][0] = c.ldb(A0, 0xe);
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[A2][0] = c.ldbu(A0, 0xe);
                                            if t {
                                                break 'b_3975b0;
                                            } else {
                                                c.cycles += 6;
                                                c.g[A1][0] = c.ldw_2(A0, 0);
                                                c.g[V0][0] = c.ldh_2(A0, 0xa);
                                                c.g[V1][0] = c.ldhu_2(A1, 0x2);
                                                c.g[V0][0] = ((c.g[V0][0] as i64)
                                                    < (c.g[V1][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V0][0] = sx((c.g[A2][0] as u32) << 24);
                                                if t {
                                                    break 'b_3975b4;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[V1][0] = 0x1;
                                                    c.stb(A0, 0xe, c.g[V1][0]);
                                                    let t = true;
                                                    c.g[V0][0] = c.ldhu_2(A1, 0x2);
                                                    if t {
                                                        break 'b_397520;
                                                    } else {
                                                        break 'b_3975b0;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = sx((c.g[A2][0] as u32) << 24);
                                        break 'b_3975b4;
                                    }
                                    'b_3975d8: {
                                        c.cycles += 4;
                                        c.g[V1][0] = 0x1;
                                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                                        let t = c.g[V0][0] != c.g[V1][0];

                                        if t {
                                            break 'b_3975d8;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = c.ldh_2(A0, 0xa);
                                            let t = (c.g[V0][0] as i64) >= 0;

                                            if t {
                                                break 'b_3975d8;
                                            } else {
                                                c.cycles += 2;
                                                c.sth_2(A0, 0xa, 0u64);
                                                c.stb(A0, 0xe, 0u64);
                                                break 'b_3975d8;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let ra = c.g[RA][0] as u32;
                                    c.pc = ra;
                                    return;
                                } else {
                                    c.cycles += 2;
                                    let ra = c.g[RA][0] as u32;
                                    c.pc = ra;
                                    return;
                                }
                            } else {
                                c.cycles += 2;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.g[V0][0] = c.ldhu_2(A0, 0xa);
                                    c.cycles += 8;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                    c.sth_2(A0, 0xa, c.g[V0][0]);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                                    c.g[V1][0] = c.ldhu_2(A2, 0x2);
                                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = c.g[A1][0] & 0x8;
                                    if t {
                                        break 'b_3974e0;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                        if t {
                                            break 'b_39752c;
                                        } else {
                                            c.cycles += 2;
                                            let ra = c.g[RA][0] as u32;
                                            c.sth_2(A0, 0xa, c.g[V0][0]);
                                            c.pc = ra;
                                            return;
                                        }
                                    }
                                } else {
                                    break 'b_3974e0;
                                }
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let ra = c.g[RA][0] as u32;
                    c.sth_2(A0, 0xa, c.g[V0][0]);
                    c.pc = ra;
                    return;
                }
                c.cycles += 2;
                let ra = c.g[RA][0] as u32;
                c.sth_2(A0, 0xa, 0u64);
                c.pc = ra;
                return;
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
    return;
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
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::RA as RET;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::fill;
    use crate::difftest::put_words;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::compare;
    use crate::difftest::put_vec;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_7 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_8 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_9 as pool_allocator_2;
    pub(crate) use crate::game::input::inputmap::tests::alloc;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_3 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::gp_at;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_2;
    pub(crate) use crate::game::render::graphicsman::tests::same;


    #[test]
    fn uianim_get_frame_count_safe_matches() {
        check(ssx3_ee::s13::f_00397278, uianim_get_frame_count_safe, 200, &|c, seed| {
            let mut r = Rng(0x3972_7800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            if seed % 3 == 0 {
                c.bus.write32(DATA, 0);
            } else {
                c.bus.write32(DATA, DATA + 0x100);
                c.bus.write16(DATA + 0x102, r.next_u32() as u16);
            }
        });
    }


    #[test]
    fn uianim_is_done_matches() {
        check(ssx3_ee::s13::f_00397440, uianim_is_done, 200, &|c, seed| {
            let mut r = Rng(0x3974_4000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, DATA + 0x100);
            c.bus.write16(DATA + 0xa, if seed % 5 == 0 { 0x8000 } else { (r.next_u32() & 0x7fff) as u16 });
            c.bus.write16(DATA + 0x102, (r.next_u32() & 0x7fff) as u16);
        });
    }


    #[test]
    fn uianim_get_frame_count_matches() {
        check(ssx3_ee::s13::f_00397458, uianim_get_frame_count, 200, &|c, seed| {
            let mut r = Rng(0x3974_5800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, DATA + 0x100);
            c.bus.write16(DATA + 0x102, if seed % 5 == 0 { 0x8000 } else { (r.next_u32() & 0x7fff) as u16 });
        });
    }


    #[test]
    fn uianim_set_play_mode_matches() {
        check(ssx3_ee::s13::f_00397468, uianim_set_play_mode, 200, &|c, seed| {
            let mut r = Rng(0x3974_6800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((seed % 3) as u64) | ((r.next_u32() as u64) << 32);
            c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(DATA, DATA + 0x100);
            c.bus.write16(DATA + 0x102, r.next_u32() as u16);
        });
    }


    /// The engine's size-class pool allocator (see `oc57`): short free lists
    /// behind `gp + 0x2a60..0x2a84`, enabled by the flag at `gp + 0xdd8`.
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
    fn ui_anim_set_create_anim_matches() {
        check(
            ee::s13::f_00397140__generated,
            ui_anim_set_create_anim,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x397140, 3);
                pool_allocator(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 6) as u64;
                // An empty animation set.
                c.bus.write32(DATA, DATA + 0x1000);
                c.bus
                    .write32(DATA + 0x1000, if seed % 3 == 0 { 0 } else { 4 });
                if seed % 3 != 0 {
                    // A single entry with the sought id and stride 0x14.
                    c.bus.write32(DATA + 0x1004 + 8, seed % 6);
                    c.bus.write32(DATA + 0x1004 + 0xc, 0x14);
                    c.bus.write32(DATA + 0x1004 + 4, 0);
                }
            },
        );
    }


    #[test]
    fn ui_anim_dtor_matches() {
        check(
            ee::s13::f_003973a8__generated,
            ui_anim_dtor,
            SEEDS,
            &alloc_base_setup(|c, seed, r| {
                let free_self = seed & 1;
                let this = if free_self != 0 { alloc(c, 0x60) } else { DATA };
                let block = alloc(c, 0x100);
                let array = block + 0x10;
                w(c, array - 0x10, seed % 4);
                w(c, this + 4, array);
                w(c, this + 0x10, r.next_u32());
                c.g[A0][0] = this as u64;
                c.g[A1][0] = free_self as u64;
            }),
        );
    }


    #[test]
    fn uianim_eval_channel_matches() {
        check(ee::s13::f_003971c8__generated, uianim_eval_channel, 120, &|c, seed| {
            let mut r = base_2(c, seed, 0x3971);
            let anim = DATA + 0x100;
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 40) as u64;
            c.bus.write32(DATA, if seed % 7 == 0 { 0 } else { anim });
            c.bus.write16(anim + 6, (seed % 5) as u16);
            c.bus.write16(anim + 0xa, (seed % 20) as u16);
            for i in 0..5u32 {
                c.bus.write16(anim + 0xc + 4 * i, (1 + i) as u16);
                c.bus
                    .write16(anim + 0xc + 4 * i + 2, (r.next_u32() & 0xff) as u16);
            }
        });
    }


    #[test]
    fn ui_anim_advance_matches() {
        same(
            ssx3_ee::s13::f_003974b0__generated,
            ui_anim_advance,
            200,
            &|c, seed| {
                let mut r = Rng(0x3974b0 ^ seed as u64);
                scramble(c, &mut r);
                let anim = DATA + 0x100;
                let data = DATA + 0x200;
                c.g[A0][0] = anim as u64;
                c.g[A1][0] = ((seed % 0x10) | ((seed % 3) << 3)) as u64;
                c.bus.write32(anim, data);
                c.bus.write16(data + 2, 1 + (r.next_u32() % 8) as u16);
                let frame = r.next_u32() % 10;
                c.bus.write16(
                    anim + 0xa,
                    if (seed & 1) == 0 {
                        frame as u16
                    } else {
                        0xffff
                    },
                );
                c.bus.write8(anim + 0xe, (seed % 2) as u8);
            },
        );
    }
}
