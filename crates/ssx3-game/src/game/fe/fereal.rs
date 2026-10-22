//! `src/fe/fereal.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use crate::call::sp;
use crate::call::Flow;
use crate::events::back_edge_unwinds;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::vu0::X;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw;
pub(crate) use crate::game::animation::rideranimbase::sw;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001a_0720, feui_custom_ctor, "FEUICustom_ctor"),
    (0x001a_07c8, feuicustom_v03, "FEUICustom_v03"),
    (0x001a_1cb8, fe_module_set_player_pad_mask, "FEModule_SetPlayerPadMask"),
    (0x001a_1cd0, fe_module_get_player_pad_mask, "FEModule_GetPlayerPadMask"),
    (0x001a_1d50, fe_module_dtor, "FEModule_dtor"),
    (0x001a_2128, femodule_try_exit, "FEModule_TryExit"),
    (0x001a_2190, fe_module_unload, "FEModule_Unload"),
    (0x001a_2bc0, femodule_unload_anim_data, "FEModule_UnloadAnimData"),
    (0x001a_2e20, feui_sound_ctor, "FEUISound_ctor"),
    (0x001a_2f38, ovui_sound_ctor, "OVUISound_ctor"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE_2 as BUILTIN_DELETE;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
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
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4 as EA_MEM_FREE_2;
    pub(crate) use crate::game::main::gamestate::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_STOP_HANDLE as BANK_MONITOR_STOP_HANDLE_2;
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
}


/// `void FEModule_SetPlayerPadMask(FEModule* m, int slot, u8 mask)` (0x1a1cb8):
/// the per-player pad mask bytes at +8, indexed by the sign-extended `slot`.
pub fn fe_module_set_player_pad_mask(c: &mut Cpu) {
    c.cycles += 5;
    let selfp = abi::arg(c, 0);
    let slot = abi::arg(c, 1) as u8 as i8 as i32;
    c.g[A1][0] = sx(slot as u32);
    c.g[A0][0] = sx(selfp.wrapping_add(slot as u32));
    let mask = c.g[A2][0];
    Addr(c.g[A0][0] as u32).off(8).sb(c, mask);
    abi::ret(c);
}


/// `u8 FEModule_GetPlayerPadMask(FEModule* m, int slot)` (0x1a1cd0): the
/// per-player pad mask byte at +8.
pub fn fe_module_get_player_pad_mask(c: &mut Cpu) {
    c.cycles += 5;
    let selfp = abi::arg(c, 0);
    let slot = abi::arg(c, 1) as u8 as i8 as i32;
    c.g[A1][0] = sx(slot as u32);
    c.g[A0][0] = sx(selfp.wrapping_add(slot as u32));
    let addr = (c.g[A0][0] as u32).wrapping_add(8);
    c.g[V0][0] = c.lbu(addr);
    abi::ret(c);
}


/// `void* FEUISound_ctor(FEUISound* self)` (0x1a2e20): base UI-sound
/// constructor plus the derived vtable.
pub fn feui_sound_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_SOUND_BASE_CTOR.call(c, 0x001a_2e34)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6cb8));
        let v1 = c.g[V1][0];
        let s0 = c.g[S0][0];
        Addr(s0 as u32).off(4).sw(c, 0);
        Addr(s0 as u32).sw(c, v1);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void* OVUISound_ctor(OVUISound* self)` (0x1a2f38): the overlay variant of
/// the UI-sound constructor.
pub fn ovui_sound_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_SOUND_BASE_CTOR.call(c, 0x001a_2f4c)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6ce8));
        let v1 = c.g[V1][0];
        let s0 = c.g[S0][0];
        Addr(s0 as u32).off(4).sw(c, 0);
        Addr(s0 as u32).sw(c, v1);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// A `gp`-relative global address.
#[inline(always)]
pub(crate) fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}


/// `addiu sp, sp, -n`.
#[inline(always)]
pub(crate) fn open(c: &mut Cpu, n: u32) {
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(n));
}


/// `addiu sp, sp, n; jr ra` (with `ra` already reloaded by the caller).
#[inline(always)]
pub(crate) fn close(c: &mut Cpu, n: u32) {
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(n));
    abi::ret(c);
}


/// `int FEUICustom_v03(FEUIObject* self, int unused, int hash)` (0x1a07c8):
/// true (and the screen object's slot 24 invoked with `(1, 0)`) when `hash`
/// is the hash of the credits object name `D_0045FFF8`.
pub fn feuicustom_v03(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        open(c, 0x30);
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A0][0] = 0x460000;
        c.g[S0][0] = c.g[A2][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x001a_07ec);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x8));
        call_ptr(c, 0x0031_7670, 0x001a_07ec)?;
        if c.g[S0][0] == c.g[V0][0] {
            c.cycles += 3;
            c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            let no_obj = c.g[A3][0] == 0;
            c.g[A1][0] = 0x1;
            if !no_obj {
                c.cycles += 6;
                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = 0u64;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                let target = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x001a_0818);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, target, 0x001a_0818)?;
            }
            c.cycles += 1;
            c.g[V0][0] = 0x1;
        } else {
            c.g[V0][0] = 0u64;
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}


/// `bool FEModule_TryExit(FEModule* self)` (0x1a2128): exit the current flow
/// state, stop the module's UI screens and free the module's flow state.
pub fn femodule_try_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        open(c, 0x20);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x001a_213c);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0023_1cf0, 0x001a_213c)?;
        let can_exit = c.g[V0][0] != 0;
        c.g[V0][0] = 0xb0000;
        if can_exit {
            c.cycles += 4;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5ad4));
            if c.g[A0][0] == 0 {
                c.sw((c.g[S0][0] as u32).wrapping_add(0x5ad8), 0u64);
            } else {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a_2164);
                c.g[A1][0] = 0x3;
                call_ptr(c, 0x0025_3418, 0x001a_2164)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x5ad4), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x5ad8), 0u64);
            }
            c.cycles += 5;
            c.g[A0][0] = sx(gp(c, 0x1d90));
            c.sw(gp(c, -0x1724), 0u64);
            c.g[RA][0] = sx(0x001a_217c);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x5adc), 0u64);
            call_ptr(c, 0x001a_06b0, 0x001a_217c)?;
            c.g[V0][0] = 0x1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0u64;
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x20);
        Ok(())
    })
}


/// `void FEModule_dtor(FEModule* self, int flags)` (0x1a1d50): tear down the
/// two net hooks then the base front-end module.
pub fn fe_module_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = 0x46d7_c0;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = 0xb5abc;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = sx(at(c, S0, 0xb5abc));
        Addr(at(c, S0, 0)).sw(c, c.g[V1][0]);
        c.g[A1][0] = 2;
        call_ptr(c, 0x0026_39b0, 0x001a_1d88)?;
        c.g[A1][0] = 2;
        c.g[A0][0] = sx(at(c, S0, 0xb5ab0));
        call_ptr(c, 0x0026_3678, 0x001a_1d9c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0019_57d0, 0x001a_1da8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `func_001A2190` (0x1a2190): `FEModule_Unload`. Unloads the module's
/// animation data, rider previews and bolt-file cache, then shuts the file
/// system down and clears the world's rider list (`world+0x80`, `+0x7c`)
/// through `0x28b180`/`0x286200`. Signature: `void unload(FEModule* this)`.
pub fn fe_module_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = lw_r(c, GP, 0u32.wrapping_sub(0x854));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_r(c, A1, 0x10d8);
        c.g[A0][0] = lh_r(c, V0, 0x98);
        c.g[V1][0] = lw_r(c, V0, 0x9c);
        let target = low(c, V1);
        c.g[RA][0] = sx(0x001a_21b8);
        c.g[A0][0] = sx(low(c, A1).wrapping_add(low(c, A0)));
        call_ptr(c, target, 0x001a_21b8)?;
        c.g[RA][0] = sx(0x001a_21c0);
        c.g[A0][0] = c.g[S0][0];
        callee::FE_UNLOAD_ANIM_DATA.call(c, 0x001a_21c0)?;
        c.g[RA][0] = sx(0x001a_21c8);
        c.g[A0][0] = sx(low(c, S0).wrapping_add(0xb0));
        callee::FE_UNLOAD_RIDER_PREVIEWS.call(c, 0x001a_21c8)?;
        c.g[RA][0] = sx(0x001a_21d0);
        c.g[A0][0] = sx(low(c, S0).wrapping_add(0x1a70));
        callee::BOLT_FILE_CACHE_SHUTDOWN.call(c, 0x001a_21d0)?;
        c.g[A0][0] = 0x50_0000;
        c.g[RA][0] = sx(0x001a_21dc);
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x47f8));
        call_ptr(c, 0x0038_adb0, 0x001a_21dc)?;
        c.g[V0][0] = lw_r(c, GP, 0u32.wrapping_sub(0x848));
        sw_r(c, V0, 0x80, 0);
        c.g[RA][0] = sx(0x001a_21ec);
        sw_r(c, V0, 0x7c, 0);
        call_ptr(c, 0x0028_b180, 0x001a_21ec)?;
        c.g[RA][0] = sx(0x001a_21f4);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0028_6200, 0x001a_21f4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void FEModule_UnloadAnimData(self)` (0x1a2bc0): drop the first animation
/// bank: unregister the bank, free its channel caches and destroy the data
/// source.
pub fn femodule_unload_anim_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 18, 0x10);
        frame.save(c, 16, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[S2][0] & 0xff;
        loop {
            c.cycles += 7;
            let banks = lw_at(c, gw(c, 0xd08));
            c.g[V0][0] = banks;
            c.g[S1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[S0][0] = lw_at(c, c.g[V0][0] as u32);
            let none = c.g[S0][0] == 0;
            c.g[A1][0] = c.g[S0][0];
            if !none {
                c.cycles += 9;
                c.g[A0][0] = lw_at(c, gw(c, 0xd8c));
                call_ptr(c, callee::GAME_ANIM_MAP_UNREGISTER_BANK.addr, 0x001a_2c04)?;
                c.g[A0][0] = lw_at(c, gw(c, 0xd08));
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.sw(c.g[V0][0] as u32, 0);
                call_ptr(c, callee::ANIM_DATA_BASE_FREE_CHAN_CACHES.addr, 0x001a_2c14)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 3;
                call_ptr(c, callee::ADF_DATA_SOURCE_DTOR.addr, 0x001a_2c20)?;
            }
            c.cycles += 2;
            let again = (c.g[S2][0] as i64) <= 0;
            c.g[V1][0] = c.g[S2][0] & 0xff;
            if !again {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001a_2be0);
            }
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void* FEUICustom_ctor(self, owner)` (0x1a0720): build the custom UI
/// object and push it onto the state stack.
pub fn feui_custom_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(0x0047_0000);
        frame.save(c, S1, 0x10);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x27f0));
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x50;
        Addr(r32(c, S1)).sw(c, c.g[A1][0]);
        c.g[A2][0] = 0;
        Addr(p(r32(c, S1), 0x10)).sw(c, 0);
        c.g[A1][0] = sx(0x0046_0b90);
        c.g[A3][0] = 0;
        Addr(p(r32(c, S1), 4)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x001a0760)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_e2a0, 0x001a0770)?;
        c.g[V1][0] = sx(0x0047_0000);
        Addr(p(r32(c, S0), 0x48)).sw(c, 0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x4478));
        Addr(p(r32(c, S0), 0x4c)).sw(c, 0);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        let null = c.g[S0][0] == 0;
        Addr(p(r32(c, S1), 0x10)).sw(c, c.g[S0][0]);
        if !null {
            c.cycles += 8;
            c.g[A0][0] = Addr(p(r32(c, V1), 0x20)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x24)).lw(c);
            let init = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, init, 0x001a079c)?;
            c.g[A0][0] = Addr(r32(c, S1)).lw(c);
            c.g[A1][0] = Addr(p(r32(c, S1), 0x10)).lw(c);
            c.g[A0][0] = sx(p(r32(c, A0), 0x18));
            call_ptr(c, 0x0039_f238, 0x001a07ac)?;
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


// ---------------------------------------------------------------------------
// `FEModule_LoadAnimData` (0x001a2b00).
//
// `void cRealFE::loadCharAnimations()`: load the character-animation table
// file and install its (single) entry.

pub fn fe_module_load_anim_data(c: &mut Cpu) {
    guest(c, |c| {

        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.g[A0][0] = 0x460000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = 0u64;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1330));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A2][0] = 0x64;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[A3][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x001a2b30);
        c.g[S0][0] = 0u64;
        call_ptr(c, 0x003ded50, 0x001a2b30)?;
        c.g[S2][0] = 0x460000;
        c.g[S1][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x1730));
        c.g[A2][0] = 0u64;
        'l_1a2b40: loop {
        c.cycles += 20;
        c.g[A3][0] = 0u64;
        c.g[A0][0] = 0x18;
        c.g[RA][0] = sx(0x001a2b50);
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1348));
        call_ptr(c, 0x00317d70, 0x001a2b50)?;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32));
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001a2b60);
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
        call_ptr(c, 0x00311250, 0x001a2b60)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xd08));
        c.g[A0][0] = c.g[S0][0] & 0xff;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xd8c));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        c.g[RA][0] = sx(0x001a2b88);
        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
        call_ptr(c, 0x00314f30, 0x001a2b88)?;
        let t = (c.g[S0][0] as i64) <= 0;
        c.g[A2][0] = 0u64;
        if t {
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x001a2b40); }
        continue 'l_1a2b40;
        } else {
        c.cycles += 11;
        c.g[RA][0] = sx(0x001a2b98);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xd8c));
        call_ptr(c, 0x003150b0, 0x001a2b98)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32));
        c.g[RA][0] = sx(0x001a2ba4);
        c.g[A1][0] = 0x64;
        call_ptr(c, 0x003dedc0, 0x001a2ba4)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t; return Ok(());
        }
        }

        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::put_vec;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use ssx3_ee as ee;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::fakes::vtable;
    use crate::difftest::RA as RET;
    use crate::difftest::SP as STACK_TOP;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::fakes::*;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_4 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_4 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_7 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3 as slot;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_2;
    pub(crate) use crate::game::main::gamestate::tests::base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_3 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_5 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_4;


    #[test]
    fn fe_module_set_player_pad_mask_matches() {
        check(ee::s03::f_001a1cb8__generated, fe_module_set_player_pad_mask, SEEDS, &|c, seed| {
            let mut r = rng(0x001a_1cb8, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA + 0x100);
            let slot = (seed % 9) as i32 - 4;
            // Odd seeds pass a slot with the sign extension hidden in a wider
            // value, which the original re-sign-extends from the low byte.
            let raw = if seed % 2 == 0 { slot } else { 0x100 + slot };
            c.g[A1][0] = sx(raw as u32);
            c.g[A2][0] = match seed % 4 {
                0 => 0,
                _ => r.next_u32() as u64,
            };
        });
    }


    #[test]
    fn fe_module_get_player_pad_mask_matches() {
        check(ee::s03::f_001a1cd0__generated, fe_module_get_player_pad_mask, SEEDS, &|c, seed| {
            let mut r = rng(0x001a_1cd0, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA + 0x100);
            let slot = (seed % 9) as i32 - 4;
            c.g[A1][0] = sx(slot as u32);
            let at = (DATA + 0x100) as i32 + slot + 8;
            c.bus.write8(at as u32, r.next_u32() as u8);
        });
    }


    #[test]
    fn feui_sound_ctor_matches() {
        check(ee::s03::f_001a2e20__generated, feui_sound_ctor, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x1a2e_20, 0);
            let obj = DATA + 0x400;
            fill(c, &mut r, obj, 0x10);
            c.g[A0][0] = sx(obj);
        });
    }


    #[test]
    fn ovui_sound_ctor_matches() {
        check(ee::s03::f_001a2f38__generated, ovui_sound_ctor, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x1a2f_38, 0);
            let obj = DATA + 0x400;
            fill(c, &mut r, obj, 0x10);
            c.g[A0][0] = sx(obj);
        });
    }


    /// Scrambled registers with a game-like `gp` (no fake call hooks: these
    /// tests only use callees that are already ported or reach no unknown
    /// code).
    pub(crate) fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x31_0102 ^ ((seed as u64) << 9) ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }


    /// `str_hash(s)`: the ELF string hash, used to build the hash-keyed tests.
    pub(crate) fn str_hash(c: &mut Cpu, s: u32) -> u64 {
        c.g[A0][0] = sx(s);
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_7670;
        ee::s10::f_00317670(c);
        assert_eq!(c.pc, RET, "StrHash did not return");
        c.g[V0][0]
    }


    /// A two-word object at `obj` whose `+off` vtable slot calls the fake.
    pub(crate) fn fake_slot(c: &mut Cpu, r: &mut Rng, obj: u32, off: u32, slot: u32) {
        let vt = vtable(c, r, &[slot]);
        c.bus.write32(obj + off, vt);
    }


    #[test]
    fn feuicustom_v03_matches() {
        check(ee::s03::f_001a07c8__generated, feuicustom_v03, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x1a_07c8, 1);
            let hash = str_hash(c, 0x0045_fff8);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = 0;
            c.g[A2][0] = if seed % 2 == 0 { hash } else { hash ^ 1 };
            let obj = DATA + 0x200;
            fake_slot(c, &mut r, obj, 0x8, 0xc0);
            c.bus.write32(DATA + 0x10, obj);
        });
    }


    #[test]
    fn femodule_try_exit_matches() {
        check(ee::s03::f_001a2128__generated, femodule_try_exit, 40, &|c, seed| {
            let mut r = base(c, seed, 0x1a_2128);
            c.g[A0][0] = sx(DATA);
            // `AppModule_TryExit` compares a float at self+4 with the global
            // at gp-0x51b4, so both outcomes are reachable.
            let t = if seed % 2 == 0 { 0.5f32 } else { 1.5f32 };
            c.bus.write32(DATA + 0x4, t.to_bits());
            c.bus.write32(GAME_GP.wrapping_sub(0x51b4), 1.0f32.to_bits());
            c.bus.write32(DATA + 0xb0000 + 0x5ad4, if seed % 3 == 0 { 0 } else { DATA + 0x100 });
            c.bus.write32(DATA + 0xb0000 + 0x5ad8, 0xdead);
            c.bus.write32(DATA + 0xb0000 + 0x5adc, 0xdead);
            c.bus.write32(GAME_GP.wrapping_sub(0x1724), 0xdead);
            c.bus.write32(GAME_GP + 0x1d90, 0xdead);
            let _ = r.next_u32();
        });
    }


    #[test]
    fn fe_module_dtor_matches() {
        check(ee::s03::f_001a1d50__generated, fe_module_dtor, 24, &|c, seed| {
            alloc_base(c, seed, 0x1a1d_50);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = [0u64, 2][seed as usize % 2];
            c.bus.write32(DATA + 0xc, 0);
            c.bus.write32(GAME_GP - 0x854, 0);
            c.bus.write32(DATA + 0xb5aa0, 0);
        });
    }


    #[test]
    fn fe_module_unload_matches() {
        check(ee::s03::f_001a2190__generated, fe_module_unload, 16, &|c, seed| {
            machine(c, seed, 0x1a2190, 8);
            be_library(c);
            heap_init_2(c);
            world(c);
            let app = DATA + 0x2000;
            w(c, GAME_GP.wrapping_sub(0x854), app);
            w(c, app + 0x10d8, DATA + 0x3000);
            slot(c, DATA + 0x3000, 0x98, 0, FAKE);
            c.g[A0][0] = (DATA + 0x2_0000) as u64;
        });
    }


    #[test]
    fn femodule_unload_anim_data_matches() {
        check(
            ee::s03::f_001a2bc0__generated,
            femodule_unload_anim_data,
            SEEDS_2,
            &|c, seed| {
                base_3(c, seed, 0x2bc0, 0);
                let map = DATA + 0x800;
                let banks = DATA + 0x900;
                c.bus.write32(GAME_GP + 0xd8c, map);
                c.bus.write32(GAME_GP + 0xd08, banks);
                let e = DATA + 0xa00;
                c.bus.write32(banks, if seed % 3 == 0 { 0 } else { e });
            },
        );
    }

    pub(crate) const SELF2: u32 = DATA + 0x19000;


    #[test]
    fn feui_custom_ctor_matches() {
        check(
            ee::s03::f_001a0720__generated,
            feui_custom_ctor,
            8,
            &|c, seed| {
                let _ = base_4(c, seed, 0x1a0720, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                w(c, 0x0046_0b90, 0);
                w(c, OBJ2, if seed % 3 == 0 { 0 } else { SELF2 });
            },
        );
    }
}
