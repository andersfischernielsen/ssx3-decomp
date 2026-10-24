//! `src/sound/asyncsys.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::control::handplantcontrol::gp_4 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_2;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::gp_2 as gp_3;
pub(crate) use crate::game::wscript::wscriptman::p;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0028_b180,
        ssx_audio_system_get_instance,
        "SSXAudioSystem_GetInstance",
    ),
    (
        0x0028_b1b0,
        ssx_audio_system_is_in_game,
        "SSXAudioSystem_IsInGame",
    ),
    (
        0x0028_b1c0,
        ssx_audio_system_get_app,
        "SSXAudioSystem_GetApp",
    ),
    (
        0x0028_b1e8,
        ssx_audio_system_get_be_library,
        "SSXAudioSystem_GetBELibrary",
    ),
    (
        0x0028_b1f8,
        ssx_audio_system_get_view_count,
        "SSXAudioSystem_GetViewCount",
    ),
    (
        0x0028_b210,
        ssx_audio_system_get_view,
        "SSXAudioSystem_GetView",
    ),
    (0x0028_b240, audio_get_frame_rate, "Audio_GetFrameRate"),
    (0x0028_b248, snd_bank_ctor, "SndBank_ctor"),
    (0x0028_b278, snd_bank_dtor, "SndBank_dtor"),
    (0x0028_b2d0, snd_bank_unload_linked, "SndBank_UnloadLinked"),
    (0x0028_b528, snd_bank_unload, "SndBank_Unload"),
];

/// The audio app / view objects: `world + 0x84` is the audio system, its
/// `+0x84` the app, and the app's `+0x10` the view count.
pub(crate) mod audio {

    pub const APP: u32 = 0x84;

    pub const LISTENER_COUNT: u32 = 0x10;
}

// ---------------------------------------------------------------------------
// Callees (same-shard direct calls; cross-shard ones stay `call_ptr`s).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_BAKE_GROUP;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_EXPORT_HISTORY_FILES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_FREE_HISTORIES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_LOAD_HISTORY_FILE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_REWARD_INT_IS_UNLOCKED;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_DESTROY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_SET_PRESENCE_ADDR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY_2 as CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_BX_STRING_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_LUNO_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_GET_SAVE_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_LIST_DIR_ALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_READ_SAVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_SEND_QUIT as C_SETUP_MAN_SEND_QUIT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEMSET;
    pub(crate) use crate::game::dirtysock::tags2::callee::EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT_3 as LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_END_OPERATION_2 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_ASCII_CHAR_TO_SJIS;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_SET_USE_SAVE_TITLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT0;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT1;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT3;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT4;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT5;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT6;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_3 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR_2 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE_2 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::ON_QUIT_POPUP_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::PATH_PLAT_REFRESH_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_ADD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RECORD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCE_MC_FORMAT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_OBJ_TABLE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_OBJ_CAM_SET_OWNER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_PUSH_FREE_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SLOGIN_MC_GET_PORT_SLOT_2 as SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY_2 as SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLINK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNOW_BOX_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::dirtysock::tags2::callee::STATS_SELECT_CATEGORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::STREAM_BUFFER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::TROPHY_ROOM_RELEASE_IMAGES;
    pub(crate) use crate::game::dirtysock::tags2::callee::UIMENU_SET_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF as UI_OBJECT_SHOW_CLAMP_ALPHA_HALF_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_GET_FONT;
    pub(crate) use crate::game::dirtysock::tags2::callee::USTR_TO_ASCII;
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

/// `bool SSXAudioSystem_IsInGame(void)` (0x0028b1b0): whether the game module
/// pointer (global at `gp-0x848`) has a non-zero app/module word at +0x84.
/// `v1` is left holding the game module pointer.
pub fn ssx_audio_system_is_in_game(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x84).lw(c);
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    abi::ret(c);
}

/// `BELibrary* SSXAudioSystem_GetBELibrary(void)` (0x0028b1e8): the library
/// pointer at game module +0x78. `v1` is left holding the game module pointer.
pub fn ssx_audio_system_get_be_library(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x78).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Sound bank (a doubly-linked list of banks).

/// `SndBank_ctor(SndBank *self)` (0x28b248): initialise an empty bank (`-1`
/// sentinels at +0x4 and +0xc, everything else zero).
pub fn snd_bank_ctor(c: &mut Cpu) {
    c.cycles += 12;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = c.g[A0][0];
    this.off(0xc).sw(c, u64::MAX);
    this.sw(c, 0);
    this.off(0x4).sw(c, u64::MAX);
    this.off(0x8).sw(c, 0);
    this.off(0x10).sw(c, 0);
    this.off(0x14).sw(c, 0);
    this.off(0x18).sw(c, 0);
    this.off(0x1c).sw(c, 0);
    this.off(0x20).sb(c, 0);
    abi::ret(c);
}

/// `SSXAudioSystem` globals: the app singleton (`gp - 0x848`, the same word as
/// the world pointer) and the audio instance (`gp + 0x410`).
pub(crate) const AUDIO_APP_GP: i32 = -0x848;

pub(crate) const AUDIO_INSTANCE_GP: i32 = 0x410;

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `lw` at `addr` (sign-extended into the 64-bit register).
#[inline(always)]
pub(crate) fn lw(c: &mut Cpu, addr: u32) -> u64 {
    c.lw(addr)
}

/// A `gp`-relative global.
#[inline(always)]
pub(crate) fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// SSXAudioSystem singletons.

/// `void* SSXAudioSystem_GetInstance()` (0x28b180): the audio system object
/// stored in the game globals (`gp + 0x410`).
pub fn ssx_audio_system_get_instance(c: &mut Cpu) {
    c.cycles += 2;
    let a = gp(c, AUDIO_INSTANCE_GP);
    c.g[V0][0] = c.lw(a);
    abi::ret(c);
}

/// `void* SSXAudioSystem_GetApp()` (0x28b1c0): the app object (`gp - 0x848`,
/// the same global as the world pointer).
pub fn ssx_audio_system_get_app(c: &mut Cpu) {
    c.cycles += 2;
    let a = gp(c, AUDIO_APP_GP);
    c.g[V0][0] = c.lw(a);
    abi::ret(c);
}
// D_004A1208
pub(crate) const WORLD: i32 = -0x848;

// ---------------------------------------------------------------------------
// Audio system.

/// `int SSXAudioSystem_GetViewCount()` (0x28b1f8): `world->audio->app->count`.
pub fn ssx_audio_system_get_view_count(c: &mut Cpu) {
    c.cycles += 5;
    let world = Addr(gp_2(c, WORLD)).lw(c);
    c.g[V0][0] = world;
    let system = Addr(world as u32).off(audio::APP).lw(c);
    c.g[V1][0] = system;
    let app = Addr(system as u32).off(audio::APP).lw(c);
    c.g[A0][0] = app;
    let count = Addr(app as u32).off(audio::LISTENER_COUNT).lw(c);
    c.g[V0][0] = count;
    abi::ret(c);
}

/// `void *SSXAudioSystem_GetView(int i)` (0x0028b210): rider view `i` from the
/// rider table (world → `+0x84` → `+0x84`, entries of 4 bytes at `+0x4`).
pub fn ssx_audio_system_get_view(c: &mut Cpu) {
    c.cycles += 7;
    let world = Addr(gp_3(c, -0x848)).lw(c);
    c.g[A1][0] = world;
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
    let cameras = Addr((world as u32).wrapping_add(0x84)).lw(c);
    c.g[V0][0] = cameras;
    let riders = Addr((cameras as u32).wrapping_add(0x84)).lw(c);
    c.g[V1][0] = sx((riders as u32).wrapping_add(c.g[A0][0] as u32));
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(4)).lw(c);
    c.pc = ra;
}

/// `void SndBank_UnloadLinked(SndBank* self)` (0x28b2d0): walk the list of
/// banks linked from `+0x18` (terminated by `self`) and unload each.
pub fn snd_bank_unload_linked(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let list = c.g[A0][0];
        c.g[S1][0] = list;
        frame.save_ra(c, 0);
        let mut p = Addr(list as u32).off(0x18).lw(c);
        c.g[S0][0] = p;
        let mut only_sentinel = false;
        if p != 0 {
            c.cycles += 2;
            if p == list {
                // the head is already the sentinel: nothing to unload
                only_sentinel = true;
            } else {
                loop {
                    c.cycles += 5;
                    c.g[A0][0] = p;
                    callee::SND_BANK_UNLOAD.call(c, 0x0028_b300)?;
                    c.g[S0][0] = Addr(p as u32).off(0x18).lw(c);
                    p = c.g[S0][0];
                    if p == list {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0028_b2f8);
                    }
                }
            }
        }
        if !only_sentinel {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SndBank_dtor(self, flags)` (0x28b278): unload, unlink and free the
/// bank's memory, then free the bank when asked.
pub fn snd_bank_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x0028_b294);
        callee::SND_BANK_UNLOAD_2.call(c, 0x0028_b294)?;
        c.g[RA][0] = sx(0x0028_b29c);
        c.g[A0][0] = c.g[S1][0];
        callee::SND_BANK_UNLINK.call(c, 0x0028_b29c)?;
        c.g[RA][0] = sx(0x0028_b2a4);
        c.g[A0][0] = c.g[S1][0];
        callee::SND_BANK_FREE_MEMORY.call(c, 0x0028_b2a4)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let free = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !free {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0028_b2b8);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::BUILTIN_DELETE, 0x0028_b2b8)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Sound.

/// `void SndBank_Unload(SndBank* self)` (0x28b528): unload a sound bank once.
pub fn snd_bank_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_2(c, c.g[S0][0] as u32);
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 7;
            Addr(c.g[S0][0] as u32).sw(c, 0);
            c.g[A0][0] = lw_2(c, p_2(c.g[S0][0], 4));
            call_ptr(c, 0x003b_6300, 0x0028_b554)?;
            c.g[V1][0] = 0xffff_ffff_ffff_ffff;
            c.g[V0][0] = lw_2(c, p_2(c.g[S0][0], 8));
            let has_owner = c.g[V0][0] != 0;
            Addr(p_2(c.g[S0][0], 4)).sw(c, c.g[V1][0]);
            if !has_owner {
                c.cycles += 4;
                c.g[A0][0] = lw_2(c, p_2(c.g[S0][0], 0x10));
                call_ptr(c, 0x0025_23a8, 0x0028_b56c)?;
                Addr(p_2(c.g[S0][0], 0x14)).sw(c, 0);
                Addr(p_2(c.g[S0][0], 0x10)).sw(c, 0);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        } else {
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Leaves: constants and field setters.

/// `int Audio_GetFrameRate(void)` (0x28b240): the audio system's fixed 60 Hz
/// frame rate (`0x3c`).
pub fn audio_get_frame_rate(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x3c;
    abi::ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::SP;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::ai::ai::tests::leaf;
    pub(crate) use crate::game::ai::ai::tests::leaf_setup;
    pub(crate) use crate::game::ai::ai::tests::rng;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4;
    pub(crate) use crate::game::camera::cameraalgorithms::tests::world_chain;
    pub(crate) use crate::game::dirtysock::tags2::tests::common;
    pub(crate) use crate::game::dirtysock::tags2::tests::dummy_world;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::heap_block_2 as heap_block;
    pub(crate) use crate::game::dirtysock::tags2::tests::SEEDS_7 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_2;
    use ssx3_ee as ee;

    #[test]
    fn ssx_audio_system_is_in_game_matches() {
        check(
            ssx3_ee::s07::f_0028b1b0__generated,
            ssx_audio_system_is_in_game,
            300,
            &|c, seed| {
                let mut r = Rng(0x4567_89ab ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP - 0x848, DATA);
                let v = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 0x8000_0000,
                    _ => r.next_u32(),
                };
                c.bus.write32(DATA + 0x84, v);
            },
        );
    }

    #[test]
    fn ssx_audio_system_get_be_library_matches() {
        check(
            ssx3_ee::s07::f_0028b1e8__generated,
            ssx_audio_system_get_be_library,
            300,
            &|c, seed| {
                let mut r = Rng(0x5678_9abc ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP - 0x848, DATA);
                c.bus
                    .write32(DATA + 0x78, if seed % 2 == 0 { 0 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn snd_bank_ctor_matches() {
        check(ssx3_ee::s07::f_0028b248, snd_bank_ctor, 50, &|c, seed| {
            let mut r = Rng(0x1111_0009 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..10 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn ssx_audio_system_get_instance_matches() {
        check(
            ssx3_ee::s07::f_0028b180__generated,
            ssx_audio_system_get_instance,
            100,
            &|c, seed| {
                let mut r = Rng(0x2b18_0001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(
                    GAME_GP + 0x410,
                    if seed % 3 == 0 { 0 } else { r.next_u32() },
                );
            },
        );
    }

    #[test]
    fn ssx_audio_system_get_app_matches() {
        check(
            ssx3_ee::s07::f_0028b1c0__generated,
            ssx_audio_system_get_app,
            100,
            &|c, seed| {
                let mut r = Rng(0x2b1c_0001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x848),
                    if seed % 3 == 0 { 0 } else { r.next_u32() },
                );
            },
        );
    }

    // ------------------------------------------------------------------- audio

    /// `world -> +0x84 -> +0x84 -> +0x10` (audio system, app, count).
    pub(crate) fn audio_world(c: &mut Cpu) -> u32 {
        let world = DATA + 0x1000;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        world
    }

    #[test]
    fn ssx_audio_system_get_view_count_matches() {
        check(
            ee::s07::f_0028b1f8__generated,
            ssx_audio_system_get_view_count,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x0028_b1f8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let world = audio_world(c);
                c.bus.write32(world + 0x84, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0x84, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000 + 0x10, r.next_u32());
            },
        );
    }

    #[test]
    fn ssx_audio_system_get_view_matches() {
        check(
            ee::s07::f_0028b210__generated,
            ssx_audio_system_get_view,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28_b210, 0);
                world_chain(c, DATA + 0x1000, DATA + 0x2000, DATA + 0x3000);
                c.g[A0][0] = (r.next_u32() % 8) as u64;
                for i in 0..8u32 {
                    c.bus.write32(DATA + 0x3000 + 4 + 4 * i, ptr(&mut r));
                }
            },
        );
    }

    #[test]
    fn snd_bank_unload_linked_matches() {
        check(
            ee::s07::f_0028b2d0__generated,
            snd_bank_unload_linked,
            SEEDS_2,
            &|c, seed| {
                let _ = machine(c, seed, 0x28b2_d0, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x20);
                let nodes = (seed % 4) as u32;
                let mut next = this;
                for i in (0..nodes).rev() {
                    let node = DATA + 0x200 + 0x100 * i;
                    clear(c, node, 0x40);
                    c.bus.write32(node + 0x18, next);
                    next = node;
                }
                let head = if seed % 5 == 0 { this } else { next };
                c.bus.write32(this + 0x18, head);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn snd_bank_dtor_matches() {
        check(
            ee::s07::f_0028b278__generated,
            snd_bank_dtor,
            SEEDS_3,
            &|c, seed| {
                let _ = common(c, seed, 0x28b278, 0);
                alloc_ready(c);
                let this = heap_block(c, 0x400);
                clear(c, this, 0x100);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // Sound.

    #[test]
    fn snd_bank_unload_matches() {
        check(
            ee::s07::f_0028b528__generated,
            snd_bank_unload,
            SEEDS_4,
            &|c, seed| {
                let _ = base(c, seed, 0x28b5_28);
                let s = DATA + 0x100;
                clear(c, s, 0x40);
                w(c, s, if seed % 3 == 0 { 1 } else { 0 });
                w(c, s + 4, if seed % 2 == 0 { 0 } else { DATA + 0x300 });
                w(c, s + 8, if seed % 4 < 2 { 0 } else { 1 });
                w(c, s + 0x10, DATA + 0x400);
                clear(c, DATA + 0x300, 0x40);
                clear(c, DATA + 0x400, 0x40);
                c.g[A0][0] = sx(s);
            },
        );
    }

    #[test]
    fn audio_get_frame_rate_matches() {
        leaf(
            ssx3_ee::s07::f_0028b240__generated,
            audio_get_frame_rate,
            16,
        );
    }
}
