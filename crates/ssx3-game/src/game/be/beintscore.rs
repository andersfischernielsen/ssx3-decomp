//! `src/be/beintscore.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call;
use crate::call::call_ptr;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::abi::r32;
use crate::call::guest;
use crate::call::sp;
use crate::call::Flow;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::world::wscriptcache::DELETE_2 as DELETE;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::scripter::ssxscriptengine::p;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0015_49a8, be_score_int_ctor, "BEScoreInt_ctor"),
    (0x0015_49e8, be_score_int_v02, "BEScoreInt_v02"),
    (0x0015_4a58, be_score_int_update_best_stats, "BEScoreInt_UpdateBestStats"),
    (0x0015_50e8, be_score_int_get_high_score_for, "BEScoreInt_GetHighScoreFor"),
    (0x0015_5130, be_score_int_get_high_score, "BEScoreInt_GetHighScore"),
    (0x0015_5190, be_score_int_get_high_score_owner, "BEScoreInt_GetHighScoreOwner"),
    (0x0015_5208, be_score_int_get_high_score_name, "BEScoreInt_GetHighScoreName"),
    (0x0015_5270, be_score_int_get_player_result, "BEScoreInt_GetPlayerResult"),
    (0x0015_5288, be_score_int_get_best_records, "BEScoreInt_GetBestRecords"),
    (0x0015_5328, be_score_int_place_to_medal, "BEScoreInt_PlaceToMedal"),
    (0x0015_5390, bescore_int_update_record_level, "BEScoreInt_UpdateRecordLevel"),
    (0x0015_57e0, be_score_int_can_submit_score, "BEScoreInt_CanSubmitScore"),
    (0x0015_58f8, bescore_int_get_event_best_place, "BEScoreInt_GetEventBestPlace"),
];

// ---------------------------------------------------------------------------
// Callees (address + the generated shard function, which dispatches through
// its override slot). Entries for calls that the generator emitted as
// `call_indirect` are kept for reference next to the `call_ptr` sites.

#[allow(dead_code)]
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_ENGINE_GET_INSTANCE;
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
    pub(crate) use crate::game::world::wscriptcache::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::SECTION_ALLOCATOR_FREE_ALL;
    pub(crate) use crate::game::world::wscriptcache::callee::C_WORLD_CACHE_STREAMER_DTOR_STUB;
    pub(crate) use crate::game::world::wscriptcache::callee::RCMP_SYSTEM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::PS2_SONY_CODEC_INTERNAL_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_GET_BITS1;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_FLUSH_BUFFER_3 as MPEG2_FLUSH_BUFFER;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCARD_INSTALL_CALLBACKS;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_SET_STATE_2 as REPLAY_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PLAYBACK_RESTORE_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CHUNK_RESOLVE_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_RENDER_FADE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_RAIL_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_MODIFIER_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUTOSAVE_UPDATE_ICON;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_ANIM_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_DTOR_2 as WORLD_OBJ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_RUN_KILL_SCRIPT_2 as WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MOD_OBJ_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_LOCALE_LOAD_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_2 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE;
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
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_FROM_ASCII_2 as USTR_FROM_ASCII;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_SPRINTF;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WIDGET_ALIGN;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WRAPPED_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::RIDER_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_MISSION_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT_2 as WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_MAN_CLEAR;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_SET_EVENT_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::OPTIONS_SET_VIBRATION;
    pub(crate) use crate::game::bx::bxstring2::callee::STR_HASH;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_FIND_FIRST_SELECTABLE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::STRLEN;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_DEQUEUE_SECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_POP_FREE_NODE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_ADD_ITEM_STR_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_FIND_ITEM_BY_VALUE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_CTOR_COPY;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_DTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_SPEECH_CONTEXT_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_SKIP_WHITESPACE;
    pub(crate) use crate::game::bx::bxstring2::callee::CHUNK_RESOLVE_RELOCATE_DMA;
    pub(crate) use crate::game::bx::bxstring2::callee::FLAG_MANAGER_GROUP_BUILD_MESH;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::READ_PRIVATE_MESSAGE_SET_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_MANAGER;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_SCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_READ_FLOAT_3 as PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_DESIRED_EYE;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_LOOK_AT;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_INIT_FROM_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::MEMSET;
    pub(crate) use crate::game::bx::bxstring2::callee::QSORT;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN3D_GROUP_BUILD_MATRICES;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAM_CONTROLLER_RESTORE_DEFAULT;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_SET_SKIP_DELAY;
    pub(crate) use crate::game::bx::bxstring2::callee::CUTSCENE_TRANSITION_MAN_FORCE_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_LEN;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_CPY;
    pub(crate) use crate::game::bx::bxstring2::callee::UITEXT_SET_WIDE_STRING;
}


// ---------------------------------------------------------------------------
// Front-end scores.

/// `BEScoreResult *BEScoreInt_GetPlayerResult(BEScoreInt *self, int player, ...)`
/// (0x155270): `self + player*0xb0 + 0x20`.
pub fn be_score_int_get_player_result(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = 0xb0;
    let slot = abi::mult(c, abi::arg(c, 1), 0xb0);
    c.g[V0][0] = sx(slot.wrapping_add(0x20));
    c.g[V0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}


/// `BEScoreRecord *BEScoreInt_GetBestRecords(BEScoreInt *self, int rider, int character)`
/// (0x155288): `rider*0x9b50 + character*0xf88 + 0x4a6ca8 + 0xde4`.
pub fn be_score_int_get_best_records(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V1][0] = 0x9b50;
    c.g[A0][0] = 0xf88;
    let rider_off = abi::mult(c, abi::arg(c, 1), 0x9b50);
    c.g[V0][0] = sx(rider_off);
    let char_off = abi::mult(c, abi::arg(c, 2), 0xf88);
    c.g[A2][0] = sx(char_off);
    c.g[V1][0] = sx(0x4a_0000u32.wrapping_add(0x6ca8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xde4));
    abi::ret(c);
}


/// `int BEScoreInt_PlaceToMedal(int place)` (0x155328): medal grade for a finish
/// place — 1st -> 2, 2nd -> 3, 3rd -> 1, anything else -1.
pub fn be_score_int_place_to_medal(c: &mut Cpu) {
    c.cycles += 3;
    let place = abi::arg64(c, 1);
    if place == 1 {
        c.cycles += 2;
        c.g[V0][0] = 2;
        abi::ret(c);
        return;
    }
    if (place as i64) >= 2 {
        c.cycles += 2;
        c.g[V0][0] = 2;
        c.cycles += 2;
        if place == 2 {
            c.cycles += 2;
            c.g[V0][0] = 3;
            abi::ret(c);
            return;
        }
        c.g[V0][0] = u64::MAX;
        c.cycles += 2;
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    // place < 2 and not 1: either 0, or a negative sign-extended value.
    c.cycles += 2;
    c.cycles += 2;
    c.g[V0][0] = u64::MAX;
    if place == 0 {
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.cycles += 2;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Score singleton.

/// `BEScoreInt_ctor(self)` (0x1549a8): set the name at +0xc and reset.
pub fn be_score_int_ctor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0046_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx(0x0046_0000u32.wrapping_sub(0x5358));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.sw(Addr(c.g[S0][0] as u32 + 0xc).0, c.g[V0][0]);
        c.pc = 0x0015_49e8;
        BE_SCORE_INT_V02.call(c, 0x0015_49c8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `BEScoreInt_v02(self)` (0x1549e8): zero 0x160 bytes from +0x20, set
/// +0x10/+0x14 to 0 and +0x18/+0x1c to -1, then set two stride-0xb0 records
/// to -1.
pub fn be_score_int_v02(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(Addr(c.g[S0][0] as u32 + 0x10).0, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
        c.sw(Addr(c.g[S0][0] as u32 + 0x14).0, 0);
        c.g[A2][0] = 0x160;
        c.sw(Addr(c.g[S0][0] as u32 + 0x1c).0, c.g[V0][0]);
        c.g[RA][0] = 0x0015_4a1c;
        c.sw(Addr(c.g[S0][0] as u32 + 0x18).0, c.g[V0][0]);
        call_ptr(c, 0x003e_6448, 0x0015_4a1c)?;
        c.g[V1][0] = u64::MAX;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x17c));
        c.g[V0][0] = 1;
        loop {
            c.cycles += 7;
            c.sw(Addr(c.g[S0][0] as u32).0, c.g[V1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let t = (c.g[V0][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xb0));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0015_4a28);
                }
                continue;
            }
            c.cycles += 4;
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}


pub(crate) const BE_SCORE_INT_V02: Callee = Callee { addr: 0x0015_49e8, f: ssx3_ee::s01::f_001549e8 };


/// `BEScoreInt_UpdateBestStats(self, rider)` (0x154a58): ask the rider's
/// profile whether stats should update, then `BEScoreInt_UpdateRecords`.
pub fn be_score_int_update_best_stats(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0014_a0e0, 0x0015_4a74)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 0x0053_0000;
        c.g[V1][0] = Addr(0x0053_05f9).lb(c);
        if c.g[V1][0] != 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < 2) as u64;
            let update = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if update {
                c.cycles += 4;
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = u64::MAX;
                c.g[A3][0] = u64::MAX;
                call_ptr(c, 0x0015_5420, 0x0015_4aa4)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int BEScoreInt_GetHighScoreFor(score, int i, int a, int b)` (0x1550e8):
/// entry `i` (0x14 bytes) of the high-score table for `(a, b)`, or 0.
pub fn be_score_int_get_high_score_for(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A2][0];
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A3][0];
        callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE.call(c, 0x0015_5104)?;
        let table = c.g[V0][0];
        c.g[V1][0] = 0x14;
        if table != 0 {
            c.cycles += 3;
            c.mult(0, A0, S0, V1);
            c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
            c.g[V0][0] = c.lw(r32(c, V1));
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int BEScoreInt_GetHighScore(int index)` (0x155130): the high-score entry
/// at `index` for the current player, or 0 when no table exists.
pub fn be_score_int_get_high_score(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let index = c.g[A1][0];
        c.g[S0][0] = index;
        call_ptr(c, 0x0014_dc80, 0x0015_5144)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0015_5150)?;
        c.g[V1][0] = 0x53_0000;
        c.g[V0][0] = 0x53_0000;
        c.g[A0][0] = Addr(r32_2(c, V1)).off(0x5f0).lw(c);
        c.g[A1][0] = Addr(r32_2(c, V0)).off(0x5c12).lb(c);
        call_ptr(c, 0x0014_ab20, 0x0015_5164)?;
        let table = c.g[V0][0] != 0;
        c.g[V1][0] = 0x14;
        if table {
            c.cycles += 3;
            c.mult(0, A0, S0, V1);
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[V0][0] = Addr(r32_2(c, V1)).lw(c);
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// A `gp`-relative global.
#[inline(always)]
pub(crate) fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}


#[inline(always)]
pub(crate) fn lb(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lb(c)
}


/// `char* BEScoreInt_GetHighScoreName(int idx)` (0x155208): resolve the BE
/// interface and the score table, then return entry `idx`'s name (20-byte
/// records, name at +8) or the default name at `gp-0x1eac`.
pub fn be_score_int_get_high_score_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let idx = c.g[A1][0];
        c.g[S0][0] = idx;
        call_ptr(c, 0x0014_dc80, 0x0015_521c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0015_5228)?;
        c.g[V1][0] = 0x0053_0000;
        c.g[V0][0] = 0x0053_0000;
        c.g[A1][0] = lb(c, 0x0053_5c12);
        c.g[A0][0] = lw(c, 0x0053_05f0);
        call_ptr(c, 0x0014_ab20, 0x0015_523c)?;
        c.g[A1][0] = c.g[V0][0];
        let have = c.g[A1][0] != 0;
        c.g[V0][0] = 0x14;
        if have {
            c.cycles += 3;
            c.mult(0, V1, S0, V0);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(8));
        } else {
            c.cycles += 2;
            c.g[V0][0] = lw(c, gp(c, -0x1eac));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `bool BEScoreInt_CanSubmitScore(BEScoreInt* self, int riders)` (0x1557e0):
/// no handicap is set, the BE interface is up, and fewer than two profiles
/// are in career mode.
pub fn be_score_int_can_submit_score(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        open(c, 0x10);
        c.g[A1][0] = ((c.g[A1][0] as i64) < 2i64) as u64;
        let few = c.g[A1][0] == 0;
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        if !few {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0015_57f8);
            c.g[A0][0] = 0u64;
            call_ptr(c, 0x0014_f810, 0x0015_57f8)?;
            let no_handicap = c.g[V0][0] == 0;
            if !no_handicap {
                c.cycles += 2;
                c.g[V0][0] = 0u64;
                c.cycles += 3;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                close(c, 0x10);
                return Ok(());
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0u64;
            c.cycles += 3;
            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
            close(c, 0x10);
            return Ok(());
        }
        c.cycles += 10;
        c.g[RA][0] = sx(0x0015_5810);
        call_ptr(c, 0x0014_dc80, 0x0015_5810)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0015_581c);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x0014_dd58, 0x0015_581c)?;
        c.g[V1][0] = 0x530000;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5bfc));
        c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
        let career = c.g[V0][0] != 0;
        c.g[V0][0] = 0x1;
        if !career {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0015_5838);
            c.g[A0][0] = 0u64;
            call_ptr(c, 0x0014_f810, 0x0015_5838)?;
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x10);
        Ok(())
    })
}


/// `func_00155190` (0x155190): `BEScoreInt_GetHighScoreOwner`. Looks up the
/// high-score table for entry `a1` (`BEScoreInt_GetHighScoreTable(`D_005305f0`,
/// `D_00535c12`)`); when it exists returns `table[a1].+4 + 0x530990`, else the
/// current player name (`gp-0x1eac`). Signature: `char* owner(int)`, where
/// `a0` is a `this` the original never reads.
pub fn be_score_int_get_high_score_owner(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0015_51a4);
        c.g[S0][0] = c.g[A1][0];
        callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x0015_51a4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0015_51b0);
        c.g[A1][0] = 0;
        callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x0015_51b0)?;
        c.g[V1][0] = 0x53_0000;
        c.g[V0][0] = 0x53_0000;
        c.g[A0][0] = Addr(low(c, V1).wrapping_add(0x5f0)).lw(c);
        c.g[RA][0] = sx(0x0015_51c4);
        c.g[A1][0] = Addr(low(c, V0).wrapping_add(0x5c12)).lb(c);
        callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE.call(c, 0x0015_51c4)?;
        let no_table = c.g[V0][0] == 0;
        c.g[A0][0] = 0x14;
        if no_table {
            c.cycles += 1;
            c.g[V0][0] = lw_r(c, GP, 0u32.wrapping_sub(0x1eac));
        } else {
            c.cycles += 9;
            c.g[A1][0] = 0x88;
            c.mult(0, V1, S0, A0);
            c.g[A0][0] = sx(low(c, V1).wrapping_add(low(c, V0)));
            c.g[V0][0] = lw_r(c, A0, 0x4);
            c.g[V1][0] = 0x53_0000;
            c.g[V1][0] = sx(low(c, V1).wrapping_add(0x990));
            c.mult(0, V0, V0, A1);
            c.g[V0][0] = sx(low(c, V0).wrapping_add(low(c, V1)));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// BE scores.

/// `void BEScoreInt_UpdateRecordLevel(BEScoreInt* self, int idx, int score)`
/// (0x155390): raise the stored record level for `idx` when `score` beats it.
pub fn bescore_int_update_record_level(c: &mut Cpu) {
    c.cycles += 14;
    c.g[V1][0] = 6;
    c.g[V0][0] = 0x0044_0000;
    c.mult(0, V1, A1, V1); // idx * 6
    c.g[T3][0] = c.g[V0][0];
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0xed0));
    c.g[T1][0] = c.g[A0][0];
    c.g[A0][0] = sx(r32(c, T1).wrapping_add(r32(c, A1)));
    c.g[T0][0] = 0;
    c.g[T2][0] = Addr(p(r32(c, A0), 0xbb8)).lb(c);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.g[V0][0] = Addr(p(r32(c, V1), 4)).lh(c);
    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let below = c.g[V0][0] != 0;
    c.g[A3][0] = 2;
    if !below {
        c.cycles += 2;
        c.g[T0][0] = 3;
    } else {
        c.cycles += 1;
        c.g[A3][0] = sx(r32(c, A3).wrapping_sub(1));
        loop {
            c.cycles += 2;
            let negative = (c.g[A3][0] as i64) < 0;
            c.g[V0][0] = 6;
            if negative {
                break;
            }
            c.cycles += 9;
            c.g[V1][0] = sx(r32(c, A3) << 1);
            c.mult(0, V0, A1, V0); // idx * 6
            c.g[A0][0] = sx(r32(c, T3).wrapping_add(0xed0));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
            c.g[V0][0] = Addr(r32(c, V1)).lh(c);
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[T0][0] = sx(r32(c, A3).wrapping_add(1));
                break;
            }
            c.g[A3][0] = sx(r32(c, A3).wrapping_sub(1));
            if event_due(c) {
                deliver_event_here(c, 0x0015_53d4);
            }
        }
    }
    c.cycles += 3;
    c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[T0][0] as i64)) as u64;
    let raise = c.g[V0][0] != 0;
    c.g[V0][0] = sx(r32(c, T1).wrapping_add(r32(c, A1)));
    if raise {
        c.cycles += 1;
        Addr(p(r32(c, V0), 0xbb8)).sb(c, c.g[T0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// `BEScoreInt_GetEventBestPlace` (0x1558f8): return the event record's best
// place for a rider, or -1 when the profile is unavailable or out of range.
pub fn bescore_int_get_event_best_place(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x530000;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5f9));
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S0, 0x40);
        let blocked = c.g[V1][0] != 0;
        frame.save_ra(c, 0);
        if blocked {
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            callee::BE_GET_RIDER_PROFILE.call(c, 0x0015_5930)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = (c.g[S0][0] < 0x3u64) as u64;
            let bad = c.g[V0][0] == 0;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            if bad {
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            } else {
                c.cycles += 15;
                c.g[A0][0] = c.g[S1][0];
                callee::BE_GET_RIDER_CHARACTER.call(c, 0x0015_5948)?;
                c.g[V1][0] = 0xf88;
                c.g[A0][0] = 0x9b50;
                c.mult(0, 2, 2, 3);
                c.mult(0, 4, 16, 4);
                c.g[V1][0] = 0x4a0000;
                c.g[A1][0] = c.g[S2][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
                c.g[A2][0] = c.g[S3][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                callee::BE_GET_CHAR_PROFILE_EVENT_RECORD.call(c, 0x0015_5974)?;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x1));
                }
            }
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::RA as RET;
    use ssx3_ee as ee;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::put_words;
    use crate::difftest::RA as SENTINEL;
    pub(crate) use crate::game::be::beintnetwork::tests::MEM_MUTEX;
    pub(crate) use crate::game::be::beintnetwork::tests::alloc_setup;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ3;
    pub(crate) use crate::game::fe::debugfe::tests::w;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::b;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w as w_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2 as W;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::gp_at;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_2;


    #[test]
    fn be_score_int_get_player_result_matches() {
        check_lohi(ssx3_ee::s01::f_00155270, be_score_int_get_player_result, 200, &|c, seed| {
            let mut r = Rng(0xa000_000b ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[A1][0] = seed as u64;
        });
    }


    #[test]
    fn be_score_int_get_best_records_matches() {
        check_lohi(ssx3_ee::s01::f_00155288, be_score_int_get_best_records, 200, &|c, seed| {
            let mut r = Rng(0xb000_000c ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = (seed % 8) as u64;
            c.g[A2][0] = ((seed / 8) % 8) as u64;
        });
    }


    #[test]
    fn be_score_int_place_to_medal_matches() {
        check(ssx3_ee::s01::f_00155328, be_score_int_place_to_medal, 400, &|c, seed| {
            let mut r = Rng(0xc000_000d ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = match seed % 5 {
                0 => 1,
                1 => 0,
                2 => 2,
                3 => 3 + (seed % 8) as u64,
                _ => 0xffff_ffff_ffff_ff00u64 | (seed as u64 & 0xff),
            };
        });
    }


    #[test]
    fn be_score_int_update_best_stats_match() {
        check(
            ssx3_ee::s01::f_00154a58,
            be_score_int_update_best_stats,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x4a58, 1);
                c.g[A0][0] = 0;
                c.g[A1][0] = DATA as u64;
                c.bus.write32(0x0053_5b0, 0);
                c.bus.write32(0x0053_5b20 + 0xc, if seed % 3 == 0 { 0 } else { 0xffff_ffff });
                c.bus.write32(0x0053_5b20 + 0x10, (seed % 2) as u32);
                c.bus.write8(0x0053_05f9, (seed % 5 == 0) as u8);
            },
        );
    }


    #[test]
    fn be_score_int_match() {
        check(ssx3_ee::s01::f_001549a8, be_score_int_ctor, 60, &|c, seed| {
            let mut r = machine(c, seed, 61, 6);
            alloc_setup(c, &mut r);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            for i in 0..0x80 {
                c.bus.write32(DATA + 0x2000 + i * 4, r.next_u32());
            }
        });
        check(ssx3_ee::s01::f_001549e8, be_score_int_v02, 60, &|c, seed| {
            let mut r = Rng(0x49e8_0000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            for i in 0..0x80 {
                c.bus.write32(DATA + 0x2000 + i * 4, r.next_u32());
            }
        });
    }


    #[test]
    fn be_score_int_get_high_score_for_matches() {
        check(
            ee::s01::f_001550e8__generated,
            be_score_int_get_high_score_for,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15_50e8, 0);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = (r.next_u32() % 0x20) as u64;
                c.g[A2][0] = (seed % 0x1a) as u64;
                c.g[A3][0] = (r.next_u32() % 8) as u64;
            },
        );
    }


    #[test]
    fn be_score_int_get_high_score_matches() {
        check(
            ssx3_ee::s01::f_00155130__generated,
            be_score_int_get_high_score,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x155130, 0);
                // A created BE library singleton.
                w(c, GAME_GP - 0x1f04, OBJ3);
                w(c, GAME_GP - 0x1efc, 0);
                w(c, 0x0053_08a8, 0);
                w(c, 0x0053_05f0, if seed % 2 == 0 { 0 } else { OBJ2 });
                c.bus.write8(0x0053_0000 + 0x5c12, (seed % 8) as u8);
                for i in 0..8u32 {
                    w(c, OBJ2 + 20 * i, seed + i);
                }
                c.g[A1][0] = (seed % 8) as u64;
            },
        );
    }


    // ------------------------------------------------------------- score names

    #[test]
    fn be_score_int_get_high_score_name_matches() {
        check(
            ssx3_ee::s01::f_00155208__generated,
            be_score_int_get_high_score_name,
            60,
            &|c, seed| {
                let _r = setup(c, seed, 0x155208);
                c.g[A0][0] = 0;
                c.g[A1][0] = (seed % 8) as u64;
                c.bus.write8(0x0053_5c12, (seed % 3) as u8);
                c.bus.write32(0x0053_05f0, DATA + 0x600);
                c.bus.write32(0x004a_1244, DATA + 0x700);
            },
        );
    }


    /// The lazily built BE singleton (`gp-0x1f04`) pointed at a zeroed dummy,
    /// and the current interface index (`gp-0x1efc`) at zero.
    pub(crate) fn be_library(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
        c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
    }


    #[test]
    fn be_score_int_can_submit_score_matches() {
        check(ee::s01::f_001557e0__generated, be_score_int_can_submit_score, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x15_57e0, 2);
            be_library(c);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 4) as u64;
            c.bus.write32(0x0053_5bfc, (seed % 3) as u32);
            c.bus.write32(0x0053_08b8, if seed % 3 == 0 { 0 } else { 1 });
            let _ = r.next_u32();
        });
    }


    #[test]
    fn be_score_int_get_high_score_owner_matches() {
        check(
            ee::s01::f_00155190__generated,
            be_score_int_get_high_score_owner,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x155190, 3);
                world(c);
                be_library_2(c);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                b(c, 0x0053_5c12, (seed % 6) as u8);
                if seed % 2 == 0 {
                    w_2(c, 0x0053_05f0, 0);
                } else {
                    let table = DATA + 0x600;
                    w_2(c, 0x0053_05f0, table);
                    let idx = (seed % 4) as u32;
                    w_2(c, table + idx * 0x14 + 4, seed % 7);
                }
            },
        );
    }


    #[test]
    fn bescore_int_update_record_level_matches() {
        check(ee::s01::f_00155390__generated, bescore_int_update_record_level, 150, &|c, seed| {
            let _ = base(c, seed, 0x15_5390);
            globals(c);
            // Three short thresholds per record.
            for i in 0..3u32 {
                c.bus.write16(0x0044_0ed0 + 6 * i, 100u16 + i as u16 * 100);
                c.bus.write16(0x0044_0ed0 + 6 * i + 2, 300u16 + i as u16 * 100);
                c.bus.write16(0x0044_0ed0 + 6 * i + 4, 600u16 + i as u16 * 100);
            }
            let idx = (seed % 3) as u32;
            c.bus.write8(DATA + idx + 0xbb8, (seed % 4) as u8);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(idx);
            c.g[A2][0] = [0u64, 150, 350, 650, 80][seed as usize % 5];
        });
    }


    #[test]
    fn bescore_int_get_event_best_place_matches() {
        check(
            ee::s01::f_001558f8__generated,
            bescore_int_get_event_best_place,
            80,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x1558);
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (seed / 5 % 4) as u64;
                c.g[A3][0] = (seed / 20 % 4) as u64;
                c.bus.write8(0x0053_05f9, (seed % 3 == 0) as u8);
                fill(c, &mut r, 0x004a_6ca8, 0x1000);
            },
        );
    }
}
