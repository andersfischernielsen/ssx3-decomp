//! `src/be/bedbreward.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call;
use crate::call::call_ptr;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use crate::call::guest;
use crate::abi::r32;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::sp;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw;
pub(crate) use crate::game::fe::feasyncfile::gp_16 as gp;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_2;
pub(crate) use crate::game::animation::rideranimbase::sw;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0015_ae00, rewards_file_unload, "RewardsFile_Unload"),
    (0x0015_c228, be_clear_option_state_5308b8, "BE_ClearOptionState5308b8"),
    (0x0015_c830, be_pre_race_destroy_instance, "BEPreRace_DestroyInstance"),
    (0x0015_c870, be_race_destroy, "BERace_Destroy"),
    (0x0015_c8b0, be_post_race_destroy_instance, "BEPostRace_DestroyInstance"),
    (0x0015_c8f0, be_global_ctors, "BE_GlobalCtors"),
    (0x0015_c910, be_global_dtors, "BE_GlobalDtors"),
    (0x0015_c930, algorithm_list_ctor, "AlgorithmList_ctor"),
    (0x0015_c940, algorithm_list_dtor, "AlgorithmList_dtor"),
    (0x0031_84e8, c_bx_string_assign_cstr, "cBXString_assign_cstr"),
];

/// Cross-shard calls the generator emitted as `call_indirect(c, addr)`: they
/// resolve through the function table like the original's `jal`.
pub(crate) mod table_callee {

    /// `operator delete(p)` (`__builtin_delete`).
    pub const OPERATOR_DELETE: u32 = 0x0031_7e50;

    /// `memset(dst, value, size)` (`EA_MemSet`).
    pub const EA_MEMSET: u32 = 0x003e_6448;

    /// `World_CreateDeadNode(world, position)`.
    pub const WORLD_CREATE_DEAD_NODE: u32 = 0x002d_1cc0;
}

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
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
    pub(crate) use crate::game::render::particle::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
    pub(crate) use crate::game::render::particle::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_RESET;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_PUSH_BACK_2 as UI_LIST_PUSH_BACK;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH_5 as STR_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_START_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE_2 as BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_DISPLAY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_7 as SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_GET_ACTIVE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_NET_CONFIG_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE_2 as MENU_SYS_SET_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_DRAW_ROW_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_POP_FREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_START_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_PUSH_FREE_NODE_2 as SCRIPT_ENGINE_PUSH_FREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ADD_SCRIPT_OBJ;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ADD_CAM;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_SET_TRANSITION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_INSTANCE_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_FIRE_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_NEXT_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_LEN_2 as USTR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_CPY_2 as USTR_CPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_ALLOC_NAMED_5 as EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_5 as EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_GET_PATH_SIZE_CHECKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_LOAD_FILE_INTO;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_ACQUIRE;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARDS_FILE_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_BACKUP_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_ON_RIDER_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_ADD_WIPEOUT_RECOVERY_BOOST;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_CHECK_RESET_PATH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_FRAME_DUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_DECODE_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_AUDIO_DONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_3 as UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_4 as UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_3 as UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_FIND_STATE_2 as UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_ALL_TIMERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_STOP_SONG_2 as MUSIC_PLAYER_STOP_SONG;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_FADE_OUT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_PLAY_SONG;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_TRY_EXIT_2 as APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CLEAR_SKIP_DELAY_ON_EXIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_INDICATOR_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GRAPHICS_MAN_FREE_ALL_TEXTURES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_4 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_GET_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_ADVERT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_POPUP_CONFIRM_EMPTY_INIT_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_LIST_GET_AT_2 as LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATOI;
    pub(crate) use crate::game::fe::feasyncfile::callee::TAG_FIELD_GET_BINARY;
}


/// `AlgorithmList* AlgorithmList_ctor(AlgorithmList *self)` (0x15c930):
/// clear the list head/tail and return `self`.
pub fn algorithm_list_ctor(c: &mut Cpu) {
    c.cycles += 4;
    let a0 = abi::arg64(c, 0);
    let this = Addr(a0 as u32);
    c.g[V0][0] = a0;
    this.sw(c, 0);
    this.off(0x4).sw(c, 0);
    abi::ret(c);
}

/// `__builtin_delete(0x317e50)`, cross-shard (through the function table).
pub(crate) const BUILTIN_DELETE_ADDR: u32 = 0x0031_7e50;

pub(crate) const ALGORITHM_LIST_CLEAR: Callee = Callee { addr: 0x0015_cb08, f: ee::s01::f_0015cb08 };


/// `void AlgorithmList_dtor(AlgorithmList *list, uint flags)` (0x15c940):
/// clear the list, then delete it when `flags & 1`.
pub fn algorithm_list_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        ALGORITHM_LIST_CLEAR.call(c, 0x0015_c95c)?;
        c.g[S0][0] &= 1;
        let delete_list = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if delete_list {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, BUILTIN_DELETE_ADDR, 0x0015_c970)?;
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
// More back-end singletons, character profiles and economy.

pub(crate) const BE_STATIC_INIT_DESTROY: Callee = Callee { addr: 0x0015_b028, f: ssx3_ee::s01::f_0015b028 };


/// `void BE_GlobalCtors(void)` (0x15c8f0): `BE_StaticInitDestroy(1, 0xffff)`.
pub fn be_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        BE_STATIC_INIT_DESTROY.call(c, 0x0015_c904)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void BE_GlobalDtors(void)` (0x15c910): `BE_StaticInitDestroy(0, 0xffff)`.
pub fn be_global_dtors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        BE_STATIC_INIT_DESTROY.call(c, 0x0015_c924)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const BUILTIN_DELETE: u32 = 0x0031_7e50;

pub(crate) const PRE_RACE_GP: u32 = 0x1e98;

pub(crate) const RACE_GP: u32 = 0x1ec4;

pub(crate) const POST_RACE_GP: u32 = 0x1e9c;


/// `BEPreRace_DestroyInstance()` (0x15c830): as above for `gp - 0x1e98`.
pub fn be_pre_race_destroy_instance(c: &mut Cpu) {
    destroy_instance(c, PRE_RACE_GP, 0x0015_c844);
}


/// `BERace_Destroy()` (0x15c870): as above for `gp - 0x1ec4`.
pub fn be_race_destroy(c: &mut Cpu) {
    destroy_instance(c, RACE_GP, 0x0015_c884);
}


/// `BEPostRace_DestroyInstance()` (0x15c8b0): as above for `gp - 0x1e9c`.
pub fn be_post_race_destroy_instance(c: &mut Cpu) {
    destroy_instance(c, POST_RACE_GP, 0x0015_c8c4);
}


/// The shared body of the `*_DestroyInstance` family: load the singleton,
/// `__builtin_delete` it, then clear the global. `ret` is the return address
/// the original passes to the delete call.
pub(crate) fn destroy_instance(c: &mut Cpu, gp_off: u32, ret: u32) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        let global = (c.g[GP][0] as u32).wrapping_sub(gp_off);
        c.g[A0][0] = Addr(global).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, BUILTIN_DELETE, ret)?;
        frame.restore_ra(c, 0);
        Addr(global).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void BE_ClearOptionState5308b8(void)` (0x15c228): zero the 0x20-byte option
/// state at `0x5308b8`.
pub fn be_clear_option_state_5308b8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x0053_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
        c.g[A2][0] = 0x20;
        call_ptr(c, table_callee::EA_MEMSET, 0x0015_c244)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cBXString* cBXString_assign_cstr(cBXString* self, const char* str)`
/// (0x3184e8): replace the contents with a C string.
pub fn c_bx_string_assign_cstr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let empty = c.g[S0][0] == 0;
        c.g[S1][0] = c.g[A0][0];
        let len;
        if empty {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.g[A1][0] = c.g[V0][0];
            len = c.g[A1][0];
        } else {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0031_850c);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::STRLEN, 0x0031_850c)?;
            c.g[A1][0] = c.g[V0][0];
            len = c.g[A1][0];
        }
        c.cycles += 9;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = len;
        c.g[RA][0] = sx(0x0031_8528);
        callee::BXSTRING_ASSIGN_COPY.call(c, 0x0031_8528)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Reward database.

/// `void cBERewardDB_init(cBERewardDB* db, int use_file)` (0x15ad58): with a
/// bolt-file name given, acquire the bolt file for that rider; otherwise load
/// and parse the rewards file. Store the result in the database's first word.
pub fn rewards_file_load(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        'end: {
            'named: {
                c.cycles += 7;
                frame.save(c, S2, 0x30);
                frame.save(c, S0, 0x20);
                c.g[S2][0] = c.g[A0][0];
                frame.save(c, S1, 0x10);
                let t = c.g[A1][0] != 0;
                frame.save_ra(c, 0);
                if t {
                    break 'named;
                }
                c.cycles += 19;
                c.g[RA][0] = sx(0x0015_ad7c);
                c.g[A0][0] = lw(c, gp(c, -0x1e80));
                call_ptr(c, 0x0031_7f60, 0x0015_ad7c)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = sx(0x0046_0000);
                c.g[A3][0] = 0;
                c.g[A1][0] = sx(0x0045_ab78);
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0015_ad98);
                c.g[A2][0] = 0;
                call_ptr(c, 0x0031_7e30, 0x0015_ad98)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = lw(c, gp(c, -0x1e80));
                c.g[A2][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0015_adac);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003e_1bb8, 0x0015_adac)?;
                c.g[A1][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0015_adb8);
                c.g[A0][0] = c.g[S2][0];
                callee::REWARDS_FILE_PARSE.call(c, 0x0015_adb8)?;
                c.g[V0][0] = sx(0xffff_ffff);
                break 'end;
            }
            c.cycles += 9;
            c.g[V0][0] = lw(c, gp(c, -0x848));
            c.g[T0][0] = c.g[A1][0];
            c.g[A2][0] = lw(c, gp(c, -0x1e80));
            c.g[A1][0] = 0;
            c.g[A0][0] = lw(c, at(c.g[V0][0], 0x7c));
            c.g[A3][0] = 0xa;
            c.g[T1][0] = 0;
            c.g[RA][0] = sx(0x0015_ade4);
            c.g[A0][0] = sx(at(c.g[A0][0], 0x1a70));
            callee::BOLT_FILE_ACQUIRE.call(c, 0x0015_ade4)?;
            break 'end;
        }
        c.cycles += 7;
        Addr(c.g[S2][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// `RewardsFile_Unload` (0x0015ae00).
//
// `void RewardsFile::Unload(RewardsFile* self)`: drop the open sound handle
// from the BE sound object, free the loaded data and clear the file record.

pub fn rewards_file_unload(c: &mut Cpu) {
    guest(c, |c| {

        'b_15ae44: {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[A2][0] = c.lw((c.g[S1][0] as u32));
        let t = (c.g[A2][0] as i64) < 0;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        if t {
        break 'b_15ae44;
        } else {
        c.cycles += 9;
        c.g[A1][0] = 0u64;
        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1a70));
        c.g[RA][0] = sx(0x0015ae34);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0019dc20, 0x0015ae34)?;
        c.g[A2][0] = c.lw((c.g[S1][0] as u32));
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0015ae44);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x0019d4b8, 0x0015ae44)?;
        break 'b_15ae44;
        }
        }
        'b_15ae5c: {
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        let t = c.g[A0][0] == 0u64;
        c.g[V0][0] = 0xffffffffffffffff;
        if t {
        break 'b_15ae5c;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x0015ae58);
        call_ptr(c, 0x00317e98, 0x0015ae58)?;
        c.g[V0][0] = 0xffffffffffffffff;
        break 'b_15ae5c;
        }
        }
        c.cycles += 24;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), 0u64);
        c.sw((c.g[S1][0] as u32), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x10), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x14), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x20), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x24), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x2c), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x30), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x34), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x38), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x3c), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x40), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x44), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x48), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t; return Ok(());

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
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::fakes;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::put_words;
    use crate::difftest::elf_image;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;
    pub(crate) use crate::game::ai::aianimeventmap::tests::MEM_MUTEX;
    pub(crate) use crate::game::ai::aianimeventmap::tests::alloc_setup;
    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;
    pub(crate) use crate::game::bx::bxstring2::tests::game_globals;
    pub(crate) use crate::game::bx::bxstring2::tests::game;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_2;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::clear;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;


    #[test]
    fn algorithm_list_ctor_matches() {
        check(ssx3_ee::s01::f_0015c930, algorithm_list_ctor, 100, &|c, seed| {
            let mut r = Rng(0x15c9 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
        });
    }


    #[test]
    fn algorithm_list_dtor_matches() {
        check(ee::s01::f_0015c940, algorithm_list_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xc940, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
            c.bus.write32(DATA, 0);
        });
    }


    #[test]
    fn be_global_ctors_dtors_match() {
        for (gen, hand, salt) in [
            (ssx3_ee::s01::f_0015c8f0 as fn(&mut Cpu), be_global_ctors as fn(&mut Cpu), 0xc8f0),
            (ssx3_ee::s01::f_0015c910, be_global_dtors, 0xc910),
        ] {
            check(gen, hand, 30, &|c, seed| {
                let _ = machine(c, seed, salt, 0);
            });
        }
    }


    #[test]
    fn be_clear_option_state_5308b8_matches() {
        check(ee::s01::f_0015c228__generated, be_clear_option_state_5308b8, 60, &|c, seed| {
            let mut r = game(c, seed, 0x15c228, 1);
            for i in 0..8u32 {
                c.bus.write32(0x0053_08b8 + 4 * i, r.next_u32());
            }
        });
    }


    pub(crate) fn call_setup(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        const RET: u64 = 0x0012_3450;
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = RET;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, RET as u32, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }


    /// The main heap's arena over 256 KiB of test RAM.
    pub(crate) fn alloc_ready(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[GP][0] = GAME_GP as u64;
        call_setup(
            c,
            0x0031_ff60,
            ssx3_ee::s10::f_0031ff60,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
        c.bus.write32(GAME_GP + 0xdd8, 1);
    }


    #[test]
    fn c_bx_string_assign_cstr_matches() {
        check(ee::s10::f_003184e8__generated, c_bx_string_assign_cstr, SEEDS_2, &|c, seed| {
            let _ = common(c, seed, 0x3184e8, 2);
            alloc_ready(c);
            let this = DATA + 0x100;
            let data = DATA + 0x300;
            let text = DATA + 0x400;
            clear(c, this, 0x40);
            clear(c, data - 0x10, 0x40);
            clear(c, text, 0x40);
            c.bus.write32(this, data);
            c.bus.write32(data - 0xc, 1);
            c.bus.write32(data - 4, 0x40);
            c.bus.write8(text, b'h');
            c.bus.write8(text + 1, b'i');
            c.g[A0][0] = sx(this);
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(text) };
        });
    }


    #[test]
    fn rewards_file_unload_matches() {
        strict(ee::s01::f_0015ae00__generated, rewards_file_unload, 120, &|c, seed| {
            let _r = machine(c, seed, 0x15ae, 8);
            let pool = DATA + 0x20_0000;
            c.bus.write32(GAME_GP + 0x2a70, pool);
            c.bus.write32(pool + 0x800, 0);
            c.bus.write32(GAME_GP + 0xdd8, 1);
            let this = DATA;
            c.bus.write32(this, 0xffff_ffff);
            c.bus.write32(this + 4, if seed % 2 == 0 { 0 } else { pool + 0x100 });
            c.bus.write32(GAME_GP - 0x848, DATA + 0x1000);
            c.g[A0][0] = sx(this);
        });
    }
}
