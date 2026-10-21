//! Phase 2 wave 16 module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for fifty small game functions, each with a differential test
//! against the generated body.
//!
//! Groups:
//! - **Game state / mode dispatch**: `GameStatePreRaceMenu_Update`, the three
//!   `GameModeMan_HandlerV*` vtable thunks.
//! - **Lobby / buddy / login**: room-user flags, reconnect, roster compare,
//!   the shared-login memcard query.
//! - **Paths**: the two `PathData` field readers and the segment reader.
//! - **Script engine**: list choices, bank slots, leaving script mode, the
//!   camera binding constructor, object registration, sound-bank entries,
//!   movie pause counting.
//! - **Audio**: system teardown, grunt banks, speech on reset, bank-monitor
//!   timers, random loop pick, song position, PS2 memcard directory listing.
//! - **Menus / UI**: action-item draw, menu load, list-box lookup, input
//!   visitor, transition-effect data.
//! - **World / render / BE**: world memory section allocators, chunk release,
//!   crowd groups, anim nodes, texture freeing, WScript tasks, ADF data
//!   sources, anim-map handles, input-parser nodes, and the BE score /
//!   profile helpers.

use crate::abi::{self, r32};
use crate::call::{guest, sp, Callee, Flow, Frame, VSlot};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0023_6c48,
        game_state_pre_race_menu_update,
        "GameStatePreRaceMenu_Update",
    ),
    (
        0x0023_8510,
        game_mode_man_handler_v06,
        "GameModeMan_HandlerV06",
    ),
    (
        0x0023_8550,
        game_mode_man_handler_v07,
        "GameModeMan_HandlerV07",
    ),
    (
        0x0023_8590,
        game_mode_man_handler_v09,
        "GameModeMan_HandlerV09",
    ),
    (0x0023_d5a8, mc_manager_create_card, "MCManager_CreateCard"),
    (
        0x0025_f248,
        lobby_man_is_room_user_flagged,
        "LobbyMan_IsRoomUserFlagged",
    ),
    (0x0026_13c8, buddy_man_reconnect, "BuddyMan_Reconnect"),
    (
        0x0026_34c0,
        buddy_man_roster_compare,
        "BuddyMan_RosterCompare",
    ),
    (
        0x0026_6060,
        c_shared_login_man_mc_get_info,
        "cSharedLoginMan_McGetInfo",
    ),
    (0x0026_9ed8, path_read_a_i_field, "Path_ReadAIField"),
    (0x0026_a338, path_read_segment, "Path_ReadSegment"),
    (
        0x0027_8e50,
        script_engine_list_add_choice,
        "ScriptEngine_ListAddChoice",
    ),
    (
        0x0027_bf50,
        script_engine_is_bank_slot_loaded,
        "ScriptEngine_IsBankSlotLoaded",
    ),
    (
        0x0027_bf90,
        script_engine_exit_script_mode,
        "ScriptEngine_ExitScriptMode",
    ),
    (
        0x0028_3088,
        script_cam_binding_ctor,
        "ScriptCamBinding_ctor",
    ),
    (0x0028_3298, script_obj_register, "ScriptObj_Register"),
    (
        0x0028_3720,
        script_sound_bank_man_free_entry,
        "ScriptSoundBankMan_FreeEntry",
    ),
    (
        0x0028_3b78,
        script_movie_set_paused,
        "ScriptMovie_SetPaused",
    ),
    (
        0x0028_4c28,
        ssx_audio_system_destroy,
        "SSXAudioSystem_Destroy",
    ),
    (
        0x0029_f5e0,
        ssx_audio_system_unload_grunt_banks,
        "SSXAudioSystem_UnloadGruntBanks",
    ),
    (
        0x002a_4b68,
        ssx_audio_system_stop_speech_on_reset,
        "SSXAudioSystem_StopSpeechOnReset",
    ),
    (
        0x002a_dda0,
        bank_monitor_kill_all_timers,
        "BankMonitor_KillAllTimers",
    ),
    (
        0x002a_e768,
        path_plat_set_track_file,
        "PATH_PlatSetTrackFile",
    ),
    (
        0x002a_e9f8,
        path_plat_clear_prebuffer,
        "PATH_PlatClearPrebuffer",
    ),
    (
        0x002b_24c0,
        song_instance_pick_random_loop,
        "SongInstance_PickRandomLoop",
    ),
    (
        0x002b_2600,
        song_instance_get_position_ms,
        "SongInstance_GetPositionMs",
    ),
    (
        0x002c_4198,
        c_ps2_mem_card_request_list_dir_w,
        "cPS2MemCard_RequestListDirW",
    ),
    (
        0x002c_d0e8,
        menu_sys_action_item_draw,
        "MenuSysActionItem_Draw",
    ),
    (
        0x002e_4d30,
        transition_effect_free_data,
        "TransitionEffect_FreeData",
    ),
    (0x0030_86b0, wscript_task_start, "WScriptTask_Start"),
    (0x0030_8720, wscript_task_stop, "WScriptTask_Stop"),
    (
        0x0030_9a60,
        wscript_man_activate_process,
        "WScriptMan_ActivateProcess",
    ),
    (
        0x0030_c468,
        wscript_man_collect_set_get_count,
        "WScriptMan_CollectSetGetCount",
    ),
    (
        0x0031_4ef0,
        game_anim_map_set_handle,
        "GameAnimMap_SetHandle",
    ),
    (
        0x0032_15f8,
        input_parser_alloc_node,
        "InputParser_AllocNode",
    ),
    (0x0034_4368, crowd_man3d_group_init, "CrowdMan3d_GroupInit"),
    (
        0x0035_0698,
        instance_has_anim_nodes,
        "Instance_HasAnimNodes",
    ),
    (
        0x0037_6578,
        ps2_graphics_man_set_default_display_mode,
        "PS2GraphicsMan_SetDefaultDisplayMode",
    ),
    (
        0x0037_cb50,
        ps2_graphics_man_free_texture,
        "PS2GraphicsMan_FreeTexture",
    ),
    (
        0x0039_a768,
        ui_list_box_find_item_by_value,
        "UIListBox_FindItemByValue",
    ),
    (0x0039_aac0, uimenu_load, "UIMenu_Load"),
    (0x003a_4d00, ui_input_visitor_visit, "UIInputVisitor_Visit"),
    (
        0x003a_7b98,
        world_memman_free_section_allocator,
        "WorldMemman_FreeSectionAllocator",
    ),
    (
        0x003a_ece0,
        av_player_release_chunk,
        "AV_Player_ReleaseChunk",
    ),
    (
        0x0011_9400,
        rider_metrix_on_knockdown_opponent,
        "RiderMetrix_OnKnockdownOpponent",
    ),
    (
        0x0011_9bf0,
        rider_metrix_start_handplant,
        "RiderMetrix_StartHandplant",
    ),
    (
        0x0014_ab20,
        be_score_int_get_high_score_table,
        "BEScoreInt_GetHighScoreTable",
    ),
    (0x0014_c658, bolts_destroy_db, "Bolts_DestroyDB"),
    (
        0x0015_15b8,
        be_restore_char_profile_equip_flags,
        "BE_RestoreCharProfileEquipFlags",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const GAME_STATE_GOTO_NEXT: Callee = Callee {
        addr: 0x0023_3aa0,
        f: ee::s06::f_00233aa0,
    };
    pub const PS2_MEMCARD_CREATE: Callee = Callee {
        addr: 0x002c_22c8,
        f: ee::s08::f_002c22c8,
    };
    pub const LOBBY_LIST_GET_AT: Callee = Callee {
        addr: 0x003e_e560,
        f: ee::s14::f_003ee560,
    };
    pub const BUDDY_MAN_CONNECT: Callee = Callee {
        addr: 0x0026_1248,
        f: ee::s06::f_00261248,
    };
    pub const STR_CASE_CMP: Callee = Callee {
        addr: 0x0041_aa88,
        f: ee::s15::f_0041aa88,
    };
    pub const SLOGIN_MC_GET_PORT_SLOT: Callee = Callee {
        addr: 0x003f_a1d0,
        f: ee::s14::f_003fa1d0,
    };
    pub const MEMCARD_REQUEST_GET_INFO_NOW: Callee = Callee {
        addr: 0x002c_67e8,
        f: ee::s08::f_002c67e8,
    };
    pub const PATH_DATA_READ_INT: Callee = Callee {
        addr: 0x0026_ba68,
        f: ee::s07::f_0026ba68,
    };
    pub const PATH_DATA_READ_VEC3: Callee = Callee {
        addr: 0x0026_ba48,
        f: ee::s07::f_0026ba48,
    };
    pub const PATH_DATA_READ_FLOAT: Callee = Callee {
        addr: 0x0026_ba88,
        f: ee::s07::f_0026ba88,
    };
    pub const SCRIPT_LIST_ADD_CHOICE: Callee = Callee {
        addr: 0x0027_5ed0,
        f: ee::s07::f_00275ed0,
    };
    pub const SCRIPT_ENGINE_POLL_ASYNC_LOADS: Callee = Callee {
        addr: 0x0027_8918,
        f: ee::s07::f_00278918,
    };
    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: Callee = Callee {
        addr: 0x0028_b180,
        f: ee::s07::f_0028b180,
    };
    pub const SSX_AUDIO_SYSTEM_END_SCRIPT_MUSIC: Callee = Callee {
        addr: 0x0028_e888,
        f: ee::s07::f_0028e888,
    };
    pub const WORLD_PAINTER_SET_SCRIPT_SURFACE_MODE: Callee = Callee {
        addr: 0x002e_f368,
        f: ee::s09::f_002ef368,
    };
    pub const SCRIPT_BINDING_CTOR: Callee = Callee {
        addr: 0x0028_2f60,
        f: ee::s07::f_00282f60,
    };
    pub const SCRIPT_ENGINE_GET_INSTANCE: Callee = Callee {
        addr: 0x0027_d2e8,
        f: ee::s07::f_0027d2e8,
    };
    pub const SND_BANK_MAN_UNLOAD_MEMORY_BANK: Callee = Callee {
        addr: 0x0028_be90,
        f: ee::s07::f_0028be90,
    };
    pub const SND_BANK_MAN_UNLOAD_BANK: Callee = Callee {
        addr: 0x0028_bce8,
        f: ee::s07::f_0028bce8,
    };
    pub const MUSIC_PLAYER_GET_SONG_TAG: Callee = Callee {
        addr: 0x002b_49e0,
        f: ee::s08::f_002b49e0,
    };
    pub const SPEECH_SYSTEM_STOP_INSTANCE: Callee = Callee {
        addr: 0x002b_11b0,
        f: ee::s08::f_002b11b0,
    };
    pub const BANK_MONITOR_TIMER_KILL: Callee = Callee {
        addr: 0x002a_d940,
        f: ee::s08::f_002ad940,
    };
    pub const SNDSTRM_SET_PREBUFFER: Callee = Callee {
        addr: 0x003b_7d80,
        f: ee::s13::f_003b7d80,
    };
    pub const AUDIO_RAND: Callee = Callee {
        addr: 0x002a_df60,
        f: ee::s08::f_002adf60,
    };
    pub const SONG_INSTANCE_GET_REQUEST_STATUS: Callee = Callee {
        addr: 0x002b_25c8,
        f: ee::s08::f_002b25c8,
    };
    pub const USTR_TO_ASCII: Callee = Callee {
        addr: 0x002c_2580,
        f: ee::s08::f_002c2580,
    };
    pub const MENU_SYS_ITEM_DRAW_ROW: Callee = Callee {
        addr: 0x002c_a988,
        f: ee::s09::f_002ca988,
    };
    pub const WSCRIPT_MAN_CALL_TASK_FUNC: Callee = Callee {
        addr: 0x0030_ada8,
        f: ee::s10::f_0030ada8,
    };
    pub const WSCRIPT_PROCESS_LIST_REMOVE: Callee = Callee {
        addr: 0x0030_b2d0,
        f: ee::s10::f_0030b2d0,
    };
    pub const WSCRIPT_PROCESS_LIST_PUSH_FRONT: Callee = Callee {
        addr: 0x0030_b260,
        f: ee::s10::f_0030b260,
    };
    pub const BE_LIBRARY_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };
    pub const BE_LIBRARY_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::f_0014dd58,
    };
    pub const BE_MISSION_INT_GET_COLLECTED_COUNT: Callee = Callee {
        addr: 0x0015_3498,
        f: ee::s01::f_00153498,
    };
    pub const ADF_DATA_SOURCE_SET_DATA: Callee = Callee {
        addr: 0x0031_1290,
        f: ee::s10::f_00311290,
    };
    pub const EA_FILE_LOAD_DECOMPRESSED: Callee = Callee {
        addr: 0x003e_22f0,
        f: ee::s14::f_003e22f0,
    };
    pub const GAME_ANIM_MAP_FIND_HASH: Callee = Callee {
        addr: 0x0031_4e88,
        f: ee::s10::f_00314e88,
    };
    pub const MEMSET: Callee = Callee {
        addr: 0x0041_6210,
        f: ee::s15::f_00416210,
    };
    pub const CROWD_MAN3D_GROUP_CREATE_MESHES: Callee = Callee {
        addr: 0x0034_43a8,
        f: ee::s11::f_003443a8,
    };
    pub const TEX_MAN_FREE_TEXTURE: Callee = Callee {
        addr: 0x0036_7bc0,
        f: ee::s12::f_00367bc0,
    };
    pub const UI_LIST_COUNT: Callee = Callee {
        addr: 0x0039_79c0,
        f: ee::s13::f_003979c0,
    };
    pub const UI_MENU_SET_NAV_BUTTONS: Callee = Callee {
        addr: 0x0039_ab00,
        f: ee::s13::f_0039ab00,
    };
    pub const UI_OBJECT_LOAD: Callee = Callee {
        addr: 0x0039_fe00,
        f: ee::s13::f_0039fe00,
    };
    pub const SECTION_ALLOCATOR_DTOR: Callee = Callee {
        addr: 0x003a_77d0,
        f: ee::s13::f_003a77d0,
    };
    pub const STREAM_FILE_RELEASE_CHUNK: Callee = Callee {
        addr: 0x003e_13e8,
        f: ee::s14::f_003e13e8,
    };
    pub const BUILTIN_DELETE: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::f_00317e50,
    };
    pub const RIDER_METRIX_SHOW_HUD_NUMBER: Callee = Callee {
        addr: 0x0011_7b88,
        f: ee::s00::f_00117b88,
    };
    pub const RIDER_METRIX_CLEAR_PEAK_A4: Callee = Callee {
        addr: 0x0011_76f8,
        f: ee::s00::f_001176f8,
    };
    pub const BE_GET_EVENT_RECORD_INDEX: Callee = Callee {
        addr: 0x0014_ab68,
        f: ee::s01::f_0014ab68,
    };
    pub const BOLTS_FREE_DB: Callee = Callee {
        addr: 0x0014_d240,
        f: ee::s01::f_0014d240,
    };
}

// ---------------------------------------------------------------------------
// Helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `lw` of a `gp`-relative global, as a register value.
#[inline(always)]
fn gp_lw(c: &mut Cpu, off: i32) -> u64 {
    Addr(gp(c, off)).lw(c)
}

/// `lw reg, gp+off`.
#[inline(always)]
fn load_gp(c: &mut Cpu, reg: usize, off: i32) {
    let a = gp(c, off);
    c.g[reg][0] = Addr(a).lw(c);
}

/// `addu` of two register values (32-bit, sign-extended).
#[inline(always)]
fn add32(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

/// The common epilogue tail: `addiu sp, sp, size; jr ra`.
#[inline(always)]
fn leave(c: &mut Cpu, size: u32) {
    let ra = r32(c, RA);
    c.g[SP][0] = sx(sp(c).wrapping_add(size));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Game state / mode dispatch.

/// `void GameStatePreRaceMenu_Update(GameState* self)` (0x236c48): leave the
/// pre-race menu when the lobby manager (gp-0x204) is gone or its `+4` is
/// clear, and when the state's `+8` is 1, advance to the next state.
pub fn game_state_pre_race_menu_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let gate = Addr(gp(c, -0x204)).lw(c);
        c.g[V0][0] = gate;
        let mut epilogue = false;
        if gate != 0 {
            c.cycles += 3;
            let next = Addr(gate as u32).off(4).lw(c);
            c.g[V0][0] = next;
            epilogue = next != 0;
            frame.restore_ra(c, 0);
        }
        if !epilogue {
            c.cycles += 4;
            c.g[V1][0] = Addr(abi::arg(c, 0)).off(8).lw(c);
            c.g[V0][0] = 1;
            let not_first = c.g[V1][0] != c.g[V0][0];
            frame.restore_ra(c, 0);
            if !not_first {
                c.cycles += 3;
                callee::GAME_STATE_GOTO_NEXT.call(c, 0x0023_6c7c)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        leave(c, 0x10);
        Ok(())
    })
}

/// The body of the `GameModeMan_HandlerV*` thunks: `obj =
/// D_00536668[*(int*)(self+4)]`, then `vt = *(void**)obj` and the vtable slot
/// at `slot` called with `a0 = obj + delta` and the other arguments as given.
fn game_mode_handler(c: &mut Cpu, slot: u32, ret: u32) -> Flow {
    c.cycles += 16;
    let frame = Frame::open(c, 0x10);
    c.g[V1][0] = 0x0053_6668;
    frame.save_ra(c, 0);
    c.g[V0][0] = Addr(p(abi::arg(c, 0), 4)).lw(c);
    c.g[V0][0] = sx((r32(c, V0) as u32) << 2);
    c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
    let index_at = r32(c, V0);
    c.g[A3][0] = Addr(index_at).lw(c);
    c.g[V1][0] = Addr(r32(c, A3)).lw(c);
    let vtable = c.g[V1][0];
    let entry = VSlot::read(c, vtable, slot);
    c.g[A0][0] = sx(entry.delta as u32);
    c.g[V0][0] = entry.func;
    c.g[RA][0] = sx(ret);
    c.g[A0][0] = abi::addu(c.g[A3][0], entry.delta);
    let func = entry.func as u32;
    c.pc = func;
    call_indirect(c, func);
    if c.pc != ret {
        return Err(crate::call::Unwound);
    }
    frame.restore_ra(c, 0);
    leave(c, 0x10);
    Ok(())
}

/// `void GameModeMan_HandlerV06(self, a, b)` (0x238510): vtable slot 6.
pub fn game_mode_man_handler_v06(c: &mut Cpu) {
    guest(c, |c| game_mode_handler(c, 0x30, 0x0023_8544))
}

/// `void GameModeMan_HandlerV07(self, a, b)` (0x238550): vtable slot 7.
pub fn game_mode_man_handler_v07(c: &mut Cpu) {
    guest(c, |c| game_mode_handler(c, 0x38, 0x0023_8584))
}

/// `void GameModeMan_HandlerV09(self, a, b)` (0x238590): vtable slot 9.
pub fn game_mode_man_handler_v09(c: &mut Cpu) {
    guest(c, |c| game_mode_handler(c, 0x48, 0x0023_85c4))
}

/// `void MCManager_CreateCard(MCManager* self)` (0x23d5a8): create the
/// memcard object into `+0x434` and reset the download state (`+0x104` = 0,
/// `+0x110` = 1.0, `+0x114` = the timer step at gp-0x5128).
pub fn mc_manager_create_card(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::PS2_MEMCARD_CREATE.call(c, 0x0023_d5bc)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        c.lwc1(0, gp(c, -0x5128));
        c.sw(p(r32(c, S0), 0x434), c.g[V0][0]);
        c.swc1(0, p(r32(c, S0), 0x114));
        c.swc1(1, p(r32(c, S0), 0x110));
        c.sw(p(r32(c, S0), 0x104), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Lobby / buddies / login.

/// `bool LobbyMan_IsRoomUserFlagged(LobbyMan* self)` (0x25f248): true when the
/// room user at `[self+0x60]` (a lobby list) exists and its `+4` bit 21 is set.
pub fn lobby_man_is_room_user_flagged(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let self_at = abi::arg(c, 0);
        c.g[V0][0] = Addr(self_at).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 4;
            c.g[A0][0] = Addr(self_at).off(0x60).lw(c);
            callee::LOBBY_LIST_GET_AT.call(c, 0x0025_f264)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(r32(c, V0)).off(4).lw(c);
                c.cycles += 2;
                c.g[V0][0] = sx(((r32(c, V0) as i32) >> 21) as u32);
                c.g[V0][0] &= 1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        leave(c, 0x10);
        Ok(())
    })
}

/// `void BuddyMan_Reconnect(BuddyMan* self)` (0x2613c8): when the reconnect
/// index `+0x40` is valid and `+0xd8` is clear, reconnect through
/// `BuddyMan_Connect` (self, index, name at +0x44, message at +0x64, `+0xec`).
pub fn buddy_man_reconnect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let self_at = abi::arg(c, 0);
        c.g[A2][0] = Addr(self_at).off(0x40).lw(c);
        let bad_index = (c.g[A2][0] as i64) < 0;
        frame.restore_ra(c, 0);
        if !bad_index {
            c.cycles += 4;
            c.g[V0][0] = Addr(self_at).off(0xd8).lw(c);
            c.g[V0][0] ^= 1;
            let busy = c.g[V0][0] == 0;
            c.g[A1][0] = c.g[A0][0];
            if !busy {
                c.cycles += 5;
                c.g[T1][0] = Addr(self_at).off(0xec).lw(c);
                c.g[A3][0] = sx(p(self_at, 0x44));
                c.g[T0][0] = sx(p(self_at, 0x64));
                callee::BUDDY_MAN_CONNECT.call(c, 0x0026_13fc)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        leave(c, 0x10);
        Ok(())
    })
}

/// `int BuddyMan_RosterCompare(void* a /*a2*/, void* b /*a3*/)`
/// (0x2634c0): a null `a` sorts after anything but null, a null `b` before
/// everything, else `strcasecmp(a, b)`.
pub fn buddy_man_roster_compare(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A3][0];
        let mut called = false;
        if c.g[A0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = (0u64 < c.g[A1][0]) as u64;
        } else {
            c.cycles += 2;
            let b_null = c.g[A1][0] == 0;
            c.g[V0][0] = u64::MAX;
            if !b_null {
                c.cycles += 4;
                callee::STR_CASE_CMP.call(c, 0x0026_34ec)?;
                called = true;
                frame.restore_ra(c, 0);
            }
        }
        if !called {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        leave(c, 0x10);
        Ok(())
    })
}

/// `void cSharedLoginMan_McGetInfo(u32 code)` (0x266060): ask the login
/// manager for the memcard port slot, then start the memcard info request
/// with the error byte it returned.
pub fn c_shared_login_man_mc_get_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        callee::SLOGIN_MC_GET_PORT_SLOT.call(c, 0x0026_6070)?;
        c.g[V1][0] = Addr(sp(c)).lbu(c);
        c.g[V0][0] = 9;
        load_gp(c, A0, 0x2e8);
        c.sw(gp(c, 0x2f4), c.g[V0][0]);
        c.g[A1][0] = c.g[V1][0];
        c.sw(gp(c, 0x2ec), c.g[V1][0]);
        callee::MEMCARD_REQUEST_GET_INFO_NOW.call(c, 0x0026_608c)?;
        c.g[V0][0] = 1;
        frame.restore_ra(c, 0x10);
        c.sw(gp(c, 0x2e4), c.g[V0][0]);
        leave(c, 0x20);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Path readers.

/// `void Path_ReadAIField(PathData* dst, u32 unused, int id /*a2*/)`
/// (0x269ed8): read the int at `+0x38` for id 0x64 and at `+0x3c` for id 0x65.
pub fn path_read_a_i_field(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let dst = abi::arg(c, 0);
        let id = abi::arg64(c, 2);
        if id == 0x64 {
            c.g[V0][0] = 0x64;
            c.cycles += 4;
            c.g[A0][0] = sx(p(dst, 0x38));
            callee::PATH_DATA_READ_INT.call(c, 0x0026_9ef0)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x65;
            let other = id != 0x65;
            frame.restore_ra(c, 0);
            if !other {
                c.cycles += 3;
                c.g[A0][0] = sx(p(dst, 0x3c));
                callee::PATH_DATA_READ_INT.call(c, 0x0026_9f0c)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        leave(c, 0x10);
        Ok(())
    })
}

/// `void Path_ReadSegment(u32 unused, const void* src /*a1*/, void* dst
/// /*a2*/)` (0x26a338): copy a segment's three floats and its length from
/// `src` to `dst`.
pub fn path_read_segment(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::PATH_DATA_READ_VEC3.call(c, 0x0026_a358)?;
        c.g[A0][0] = sx(p(r32(c, S0), 0xc));
        c.g[A1][0] = sx(p(r32(c, S1), 0xc));
        callee::PATH_DATA_READ_FLOAT.call(c, 0x0026_a364)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script engine.

/// `void ScriptEngine_ListAddChoice(ScriptEngine* self, int index, a, b, c,
/// d, f, g)` (0x278e50): add a choice (0xcc bytes) to the script list at
/// `self+0x54c`.
pub fn script_engine_list_add_choice(c: &mut Cpu) {
    guest(c, |c| {
        let self_at = abi::arg(c, 0);
        c.cycles += 16;
        c.g[V0][0] = 0xcc;
        let frame = Frame::open(c, 0x10);
        c.mult(0, V1, A1, V0);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A2][0] = c.g[A3][0];
        c.g[A0][0] = Addr(self_at).off(0x54c).lw(c);
        c.g[A3][0] = c.g[T0][0];
        c.g[T0][0] = c.g[T1][0];
        c.g[A0][0] = add32(c.g[A0][0], c.g[V1][0]);
        c.g[T1][0] = c.g[T2][0];
        c.g[T2][0] = c.g[T3][0];
        callee::SCRIPT_LIST_ADD_CHOICE.call(c, 0x0027_8e84)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool ScriptEngine_IsBankSlotLoaded(ScriptEngine* self, int slot)`
/// (0x27bf50): pump the async loaders, then report whether bank `slot`
/// (`self + 0x630 + slot*16`) is present.
pub fn script_engine_is_bank_slot_loaded(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS.call(c, 0x0027_bf6c)?;
        c.g[S1][0] = sx((r32(c, S1) as u32) << 4);
        frame.restore_ra(c, 0);
        c.g[S0][0] = add32(c.g[S0][0], c.g[S1][0]);
        c.g[V0][0] = Addr(r32(c, S0)).off(0x630).lw(c);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        leave(c, 0x30);
        Ok(())
    })
}

/// `void ScriptEngine_ExitScriptMode(ScriptEngine* self)` (0x27bf90): once,
/// set the mode to 2, end script music and disable the script surface mode.
pub fn script_engine_exit_script_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 2;
        frame.save_ra(c, 0);
        let self_at = abi::arg(c, 0);
        c.g[V0][0] = Addr(self_at).off(0x550).lw(c);
        let already = c.g[V0][0] == c.g[V1][0];
        frame.restore_ra(c, 0);
        if !already {
            c.cycles += 8;
            c.sw(p(self_at, 0x550), c.g[V1][0]);
            c.sw(p(self_at, 0x554), 0);
            callee::SSX_AUDIO_SYSTEM_GET_INSTANCE.call(c, 0x0027_bfb4)?;
            c.g[A0][0] = c.g[V0][0];
            callee::SSX_AUDIO_SYSTEM_END_SCRIPT_MUSIC.call(c, 0x0027_bfbc)?;
            c.g[A0][0] = 0;
            callee::WORLD_PAINTER_SET_SCRIPT_SURFACE_MODE.call(c, 0x0027_bfc4)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        leave(c, 0x10);
        Ok(())
    })
}

/// `ScriptCamBinding* ScriptCamBinding_ctor(ScriptCamBinding* self)`
/// (0x283088): base constructor, then the camera-binding vtable and the
/// camera index 9 / field 0.
pub fn script_cam_binding_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_BINDING_CTOR.call(c, 0x0028_309c)?;
        c.g[V0][0] = 9;
        c.g[V1][0] = 0x0048_0000;
        c.sw(p(r32(c, S0), 0xc), c.g[V0][0]);
        c.g[V1][0] = sx(0x0048_0000u32.wrapping_add(0x23e0));
        c.sw(r32(c, S0), c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw(p(r32(c, S0), 0x10), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptObj_Register(ScriptObj* self)` (0x283298): register with the
/// script engine (virtual slot 2 at `engine->vtable+0x10`) and store the id
/// it returns at `+4`.
pub fn script_obj_register(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_ENGINE_GET_INSTANCE.call(c, 0x0028_32ac)?;
        c.g[V1][0] = Addr(r32(c, V0)).off(0x2a8).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = Addr(r32(c, V1)).off(0x10).lh(c);
        c.g[A2][0] = Addr(r32(c, V1)).off(0x14).lw(c);
        let func = r32(c, A2);
        c.g[RA][0] = sx(0x0028_32c4);
        c.g[A0][0] = add32(c.g[V0][0], c.g[A0][0]);
        c.pc = func;
        call_indirect(c, func);
        if c.pc != 0x0028_32c4 {
            return Err(crate::call::Unwound);
        }
        c.sw(p(r32(c, S0), 4), c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptSoundBankMan_FreeEntry(u32 unused, SoundBankEntry* e /*a1*/)`
/// (0x283720): unload `e->bank` (`+4`) through the sound bank manager, then
/// mark the entry free (`id = -1`, `data = 0`).
pub fn script_sound_bank_man_free_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::SSX_AUDIO_SYSTEM_GET_INSTANCE.call(c, 0x0028_3734)?;
        c.g[V1][0] = Addr(r32(c, V0)).off(0x118).lw(c);
        c.g[A1][0] = Addr(r32(c, S0)).off(4).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).lw(c);
        callee::SND_BANK_MAN_UNLOAD_MEMORY_BANK.call(c, 0x0028_3744)?;
        c.g[V0][0] = u64::MAX;
        c.sw(p(r32(c, S0), 8), 0);
        c.sw(r32(c, S0), c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptMovie_SetPaused(Movie* m, int inc /*a1*/)` (0x283b78): count
/// pause requests (`+0xc`) up or down, saturating at zero, once the movie
/// (`+0`) and its stream (`+4`) exist.
pub fn script_movie_set_paused(c: &mut Cpu) {
    c.cycles += 3;
    let m = abi::arg(c, 0);
    c.g[V0][0] = Addr(m).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(m).off(4).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let dec = c.g[A1][0] == 0;
    c.g[V0][0] = Addr(m).off(0xc).lw(c);
    if dec {
        c.cycles += 2;
        let zero = c.g[V0][0] == 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        if zero {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        c.cycles += 1;
        c.sw(p(m, 0xc), c.g[V0][0]);
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    c.sw(p(m, 0xc), c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Audio.

/// `void SSXAudioSystem_Destroy()` (0x284c28): tell the audio interface
/// object (gp+0x410) to shut down (virtual slot 1, argument 3), then clear
/// the singleton.
pub fn ssx_audio_system_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        load_gp(c, V0, 0x410);
        let empty = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !empty {
            c.cycles += 8;
            c.g[V1][0] = Addr(r32(c, V0)).off(0x118).lw(c);
            c.g[A1][0] = 3;
            c.g[A2][0] = Addr(r32(c, V1)).lw(c);
            c.g[V0][0] = Addr(r32(c, A2)).off(0x1d4).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(8).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0xc).lw(c);
            let func = r32(c, V1);
            c.g[RA][0] = sx(0x0028_4c58);
            c.g[A0][0] = add32(c.g[A2][0], c.g[A0][0]);
            c.pc = func;
            call_indirect(c, func);
            if c.pc != 0x0028_4c58 {
                return Err(crate::call::Unwound);
            }
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.sw(gp(c, 0x410), 0);
        leave(c, 0x10);
        Ok(())
    })
}

/// `void SSXAudioSystem_UnloadGruntBanks(SSXAudioSystem* self)`
/// (0x29f5e0): unload sound banks 14 and 15.
pub fn ssx_audio_system_unload_grunt_banks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0xe;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(0x118).lw(c);
        let bank_man = r32(c, V0);
        c.g[A0][0] = Addr(bank_man).lw(c);
        callee::SND_BANK_MAN_UNLOAD_BANK.call(c, 0x0029_f600)?;
        c.g[V0][0] = Addr(r32(c, S0)).off(0x118).lw(c);
        c.g[A1][0] = 0xf;
        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
        callee::SND_BANK_MAN_UNLOAD_BANK.call(c, 0x0029_f610)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_StopSpeechOnReset(SSXAudioSystem* self)`
/// (0x2a4b68): when the music player at `+0x118` is on song tag 0x191, stop
/// the speech instance at `+0x5560`.
pub fn ssx_audio_system_stop_speech_on_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(r32(c, S0), 0x118));
        callee::MUSIC_PLAYER_GET_SONG_TAG.call(c, 0x002a_4b80)?;
        c.g[V1][0] = 0x191;
        if c.g[V0][0] != c.g[V1][0] {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 4;
            c.g[A0][0] = sx(p(r32(c, S0), 0x5560));
            c.g[A1][0] = 0;
            callee::SPEECH_SYSTEM_STOP_INSTANCE.call(c, 0x002a_4b98)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        leave(c, 0x20);
        Ok(())
    })
}

/// `void BankMonitor_KillAllTimers(BankMonitor* self)` (0x2adda0): kill the
/// 48 bank timers at `self+0x98`, stride 0x2c.
pub fn bank_monitor_kill_all_timers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(p(abi::arg(c, 0), 0x98));
        frame.save_ra(c, 0);
        c.g[S1][0] = 0x2f;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            callee::BANK_MONITOR_TIMER_KILL.call(c, 0x002a_ddc4)?;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x2c));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_ddb8);
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PATH_PlatSetTrackFile(int index, u32 a, u32 b)` (0x2ae768): store
/// `a`/`b` at `+0x28`/`+0x2c` of platform `index` (0..24) when it is not
/// busy (`+0x64` == 0).
pub fn path_plat_set_track_file(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = (c.g[A0][0] < 0x18u64) as u64;
    c.g[V0][0] = 0x004d_0000;
    if c.g[A0][0] >= 0x18u64 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.g[V1][0] = sx(abi::arg(c, 0) << 2);
    c.g[V0][0] = sx(0x004d_0000u32.wrapping_add(0x3e98));
    c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
    c.g[V1][0] = Addr(r32(c, V1)).lw(c);
    if c.g[V1][0] == 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, V1)).off(0x64).lb(c);
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.sw(p(r32(c, V1), 0x2c), c.g[A2][0]);
    c.sw(p(r32(c, V1), 0x28), c.g[A1][0]);
    c.cycles += 2;
    abi::ret(c);
}

/// `void PATH_PlatClearPrebuffer(int index)` (0x2ae9f8): drop the streaming
/// prebuffer request of platform `index` when it is not busy.
pub fn path_plat_clear_prebuffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x004d_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x004d_0000u32.wrapping_add(0x3e98));
        c.g[A0][0] = sx(abi::arg(c, 0) << 2);
        c.g[A0][0] = add32(c.g[A0][0], c.g[V0][0]);
        c.g[A0][0] = Addr(r32(c, A0)).lw(c);
        c.g[V0][0] = Addr(r32(c, A0)).off(0x64).lb(c);
        let busy = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !busy {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, A0)).off(0x6c).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(4).lw(c);
            callee::SNDSTRM_SET_PREBUFFER.call(c, 0x002a_ea2c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        leave(c, 0x10);
        Ok(())
    })
}

/// `u32 SongInstance_PickRandomLoop(SongInstance* self)` (0x2b24c0):
/// `Audio_Rand() % self->song->loop_count` (0 when there is no song).
pub fn song_instance_pick_random_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        load_gp(c, A0, 0x68c);
        callee::AUDIO_RAND.call(c, 0x002b_24d8)?;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        c.g[V1][0] = Addr(r32(c, A0)).off(0x40).lw(c);
        c.divu(0, V0, V1);
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.hi[0];
        leave(c, 0x20);
        Ok(())
    })
}

/// `float SongInstance_GetPositionMs(SongInstance* self)` (0x2b2600): the
/// playback position when the request status is the "playing" kind (2),
/// else 0.
pub fn song_instance_get_position_ms(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        callee::SONG_INSTANCE_GET_REQUEST_STATUS.call(c, 0x002b_2610)?;
        c.g[V1][0] = Addr(sp(c)).lw(c);
        c.g[V0][0] = 2;
        let playing = c.g[V1][0] == c.g[V0][0];
        frame.restore_ra(c, 0x10);
        if playing {
            c.cycles += 3;
            c.g[V0][0] = Addr(sp(c)).off(4).lw(c);
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
        } else {
            c.cycles += 3;
            c.mtc1(0, 0);
        }
        c.cycles += 2;
        leave(c, 0x20);
        Ok(())
    })
}

/// `void cPS2MemCard_RequestListDirW(cPS2MemCard* self, const u16* name)`
/// (0x2c4198): convert the name to ASCII and ask the virtual slot 6 of
/// `*self` for the directory listing.
pub fn c_ps2_mem_card_request_list_dir_w(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x410);
        frame.save(c, S0, 0x400);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x3f0);
        c.g[A0][0] = c.g[SP][0];
        callee::USTR_TO_ASCII.call(c, 0x002c_41b0)?;
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = Addr(r32(c, V1)).off(0x30).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x34).lw(c);
        let func = r32(c, V0);
        c.g[RA][0] = sx(0x002c_41c8);
        c.g[A0][0] = add32(c.g[S0][0], c.g[A0][0]);
        c.pc = func;
        call_indirect(c, func);
        if c.pc != 0x002c_41c8 {
            return Err(crate::call::Unwound);
        }
        frame.restore(c, S0, 0x400);
        frame.restore_ra(c, 0x3f0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysActionItem_Draw(Item* self, u32 arg)` (0x2cd0e8): draw the
/// row with the item's icon (`+0x18`) or its label (`+0x14`) as the fourth
/// argument.
pub fn menu_sys_action_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let self_at = abi::arg(c, 0);
        c.g[T0][0] = Addr(self_at).off(0x18).lw(c);
        if c.g[T0][0] != 0 {
            c.cycles += 5;
            c.g[A2][0] = Addr(self_at).off(0x14).lw(c);
            c.g[A3][0] = 0;
            callee::MENU_SYS_ITEM_DRAW_ROW.call(c, 0x002c_d108)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 4;
            c.g[A3][0] = Addr(self_at).off(0x14).lw(c);
            c.g[A2][0] = 0;
            c.g[T0][0] = 0;
            callee::MENU_SYS_ITEM_DRAW_ROW.call(c, 0x002c_d11c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        leave(c, 0x10);
        Ok(())
    })
}

/// `void TransitionEffect_FreeData(void* p)` (0x2e4d30): when `p` is one of
/// the four transition-effect buffers (`D_005382C0`, stride 0x64), clear its
/// data slot.
pub fn transition_effect_free_data(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V1][0] = 0x0054_0000;
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.g[V0][0] = 0x0054_0000;
    c.g[A2][0] = sx(0x0054_0000u32.wrapping_sub(0x77b0));
    c.g[V1][0] = sx(0x0054_0000u32.wrapping_sub(0x7d40));
    c.g[A1][0] = 0;
    loop {
        c.cycles += 2;
        let found = c.g[V1][0] == c.g[A0][0];
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        if found {
            c.cycles += 2;
            let ra = r32(c, RA);
            c.sw(r32(c, A2), 0);
            c.pc = ra;
            return;
        }
        c.cycles += 4;
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(4));
        c.g[V0][0] = ((r32(c, A1) as i64) < 4i64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x64));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002e_4d48);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// WScript tasks.

/// `void WScriptTask_Start(WScriptTask* self)` (0x3086b0): set the task state
/// to 1 and tell the script manager (through `self+0x3c`) to start the task.
pub fn wscript_task_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 1;
        frame.save_ra(c, 0);
        let self_at = abi::arg(c, 0);
        c.g[V0][0] = Addr(self_at).lw(c);
        let already = c.g[V0][0] == c.g[V1][0];
        frame.restore_ra(c, 0);
        if !already {
            c.cycles += 3;
            c.g[A2][0] = Addr(self_at).off(8).lw(c);
            let no_script = c.g[A2][0] == 0;
            c.sw(self_at, c.g[V1][0]);
            if !no_script {
                c.cycles += 5;
                c.g[V0][0] = Addr(self_at).off(0x3c).lw(c);
                c.g[A1][0] = c.g[A0][0];
                c.g[A0][0] = Addr(r32(c, V0)).off(0x10).lw(c);
                callee::WSCRIPT_MAN_CALL_TASK_FUNC.call(c, 0x0030_86e4)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        leave(c, 0x10);
        Ok(())
    })
}

/// `void WScriptTask_Stop(WScriptTask* self)` (0x308720): set the task state
/// to 3 and tell the script manager to stop the task at `+0x10`.
pub fn wscript_task_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 3;
        frame.save_ra(c, 0);
        let self_at = abi::arg(c, 0);
        c.g[V0][0] = Addr(self_at).lw(c);
        let already = c.g[V0][0] == c.g[V1][0];
        frame.restore_ra(c, 0);
        if !already {
            c.cycles += 3;
            c.g[A2][0] = Addr(self_at).off(0x10).lw(c);
            let no_script = c.g[A2][0] == 0;
            c.sw(self_at, c.g[V1][0]);
            if !no_script {
                c.cycles += 5;
                c.g[V0][0] = Addr(self_at).off(0x3c).lw(c);
                c.g[A1][0] = c.g[A0][0];
                c.g[A0][0] = Addr(r32(c, V0)).off(0x10).lw(c);
                callee::WSCRIPT_MAN_CALL_TASK_FUNC.call(c, 0x0030_8754)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        leave(c, 0x10);
        Ok(())
    })
}

/// `void WScriptMan_ActivateProcess(WScriptMan* self, Node* node)`
/// (0x309a60): move the process from the waiting list (`+0x2b4`) to the
/// active list (`+0x2b8`).
pub fn wscript_man_activate_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(r32(c, S0), 0x2b4));
        callee::WSCRIPT_PROCESS_LIST_REMOVE.call(c, 0x0030_9a80)?;
        c.g[A0][0] = sx(p(r32(c, S0), 0x2b8));
        c.g[A1][0] = c.g[S1][0];
        callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT.call(c, 0x0030_9a8c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int WScriptMan_CollectSetGetCount(WScriptMan* self, Entry* entry)`
/// (0x30c468): the collected count of mission `entry->id` in the BE mission
/// interface (interface 10 of the BE library).
pub fn wscript_man_collect_set_get_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x0030_c47c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xa;
        callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x0030_c488)?;
        c.g[A2][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::BE_MISSION_INT_GET_COLLECTED_COUNT.call(c, 0x0030_c498)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AdfDataSource* AdfDataSource_ctor(AdfDataSource* self, const char* src
/// /*a1*/)` (0x311250): load the compressed source and adopt the data.
pub fn adf_data_source_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = 0;
        callee::EA_FILE_LOAD_DECOMPRESSED.call(c, 0x0031_126c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        c.sw(r32(c, S0), c.g[V0][0]);
        callee::ADF_DATA_SOURCE_SET_DATA.call(c, 0x0031_127c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void GameAnimMap_SetHandle(GameAnimMap* self, u32 hash, void* value)`
/// (0x314ef0): store `value` in the map entry for `hash` (when it exists).
pub fn game_anim_map_set_handle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        callee::GAME_ANIM_MAP_FIND_HASH.call(c, 0x0031_4f0c)?;
        let missing = (c.g[V0][0] as i64) < 0;
        c.g[V0][0] = sx((r32(c, V0) as u32) << 2);
        if !missing {
            c.cycles += 2;
            c.g[V0][0] = add32(c.g[V0][0], c.g[S0][0]);
            c.sw(p(r32(c, V0), 0x1030), c.g[S1][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* InputParser_AllocNode(InputParser* self)` (0x3215f8): pop the free
/// list at `+0xe0`, clear the node (0x10 bytes) and return it.
pub fn input_parser_alloc_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x10;
        c.g[S0][0] = Addr(r32(c, V0)).off(0xe0).lw(c);
        c.g[V1][0] = Addr(r32(c, S0)).off(4).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.sw(p(r32(c, V0), 0xe0), c.g[V1][0]);
        callee::MEMSET.call(c, 0x0032_1624)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void CrowdMan3d_GroupInit(CrowdGroup* self, u32 arg)` (0x344368): seed
/// the group's inverse count (`1.0 / count` from the crowd manager at
/// gp+0x2a74) and build its meshes.
pub fn crowd_man3d_group_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        load_gp(c, V1, 0x2a74);
        c.sw(p(r32(c, V0), 4), c.g[A1][0]);
        c.sw(p(r32(c, V0), 0x1dc), 0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.lwc1(1, p(r32(c, V1), 0x10));
        c.cvt_s_w(1, 1);
        c.div_s(0, 0, 1);
        c.swc1(0, p(r32(c, V0), 0x1e4));
        callee::CROWD_MAN3D_GROUP_CREATE_MESHES.call(c, 0x0034_439c)?;
        frame.restore_ra(c, 0);
        leave(c, 0x10);
        Ok(())
    })
}

/// `bool Instance_HasAnimNodes(Instance* self)` (0x350698): true when any of
/// the anim-nodes list's entries (`+0x80`, count `+4`, entries `+8`, stride
/// 0x10) has a non-zero field at `+8`.
pub fn instance_has_anim_nodes(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x80).lw(c);
    c.g[A0][0] = 0;
    c.g[A1][0] = Addr(r32(c, V0)).off(4).lw(c);
    let none = (c.g[A1][0] as i64) <= 0;
    c.g[V1][0] = Addr(r32(c, V0)).off(8).lw(c);
    if none {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, V1)).off(8).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
        if !empty {
            c.cycles += 2;
            let ra = r32(c, RA);
            c.g[V0][0] = 1;
            c.pc = ra;
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0035_06b0);
        }
    }
    c.cycles += 2;
    let ra = r32(c, RA);
    c.g[V0][0] = 0;
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Graphics / UI.

/// `void PS2GraphicsMan_SetDefaultDisplayMode(PS2GraphicsMan* self, u32 mode
/// /*a1*/)` (0x376578): call virtual slot 0xf2 with the default 0x200x0x1c0
/// (0x18x0x20x0x18) screen layout and `mode`.
pub fn ps2_graphics_man_set_default_display_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x10);
        c.g[T2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x200;
        c.g[A2][0] = 0x1c0;
        c.g[A3][0] = 0x18;
        c.g[V1][0] = Addr(abi::arg(c, 0)).off(0x10d8).lw(c);
        c.g[T0][0] = 0x20;
        c.g[T1][0] = Addr(r32(c, V1)).off(0x3d0).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x3d4).lw(c);
        c.g[A0][0] = add32(c.g[A0][0], c.g[T1][0]);
        let func = r32(c, V0);
        c.g[RA][0] = sx(0x0037_65ac);
        c.g[T1][0] = 0x18;
        c.pc = func;
        call_indirect(c, func);
        if c.pc != 0x0037_65ac {
            return Err(crate::call::Unwound);
        }
        frame.restore_ra(c, 0);
        leave(c, 0x10);
        Ok(())
    })
}

/// `void PS2GraphicsMan_FreeTexture(PS2GraphicsMan* self, int index)`
/// (0x37cb50): free texture `index` through the texture manager at `+0x18f4`
/// and clear the slot at `+0x18f8 + index*4`.
pub fn ps2_graphics_man_free_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A0][0] = Addr(r32(c, S1)).off(0x18f4).lw(c);
        callee::TEX_MAN_FREE_TEXTURE.call(c, 0x0037_cb70)?;
        c.g[S0][0] = sx((r32(c, S0) as u32) << 2);
        frame.restore_ra(c, 0);
        c.g[S1][0] = add32(c.g[S1][0], c.g[S0][0]);
        c.sw(p(r32(c, S1), 0x18f8), 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        leave(c, 0x30);
        Ok(())
    })
}

/// `bool UIListBox_FindItemByValue(UIListBox* self, int key /*a1*/, u8* out
/// /*a2*/)` (0x39a768): the index of the first item (`+0xa4`, stride 0x14)
/// whose value equals `key`; stores it as a byte at `out`.
pub fn ui_list_box_find_item_by_value(c: &mut Cpu) {
    c.cycles += 3;
    let self_at = abi::arg(c, 0);
    let key = abi::arg64(c, 1);
    c.g[V0][0] = Addr(self_at).off(0x318).lbu(c);
    c.g[V1][0] = 0;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[A3][0] = sx(p(self_at, 0xa4));
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, A3)).lw(c);
        if c.g[V0][0] == key {
            c.cycles += 3;
            c.sb(abi::arg(c, 2), c.g[V1][0]);
            let ra = r32(c, RA);
            c.g[V0][0] = 1;
            c.pc = ra;
            return;
        }
        c.g[V0][0] = Addr(self_at).off(0x318).lbu(c);
        c.cycles += 4;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A3][0] = sx(r32(c, A3).wrapping_add(0x14));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0039_a778);
        }
    }
    c.cycles += 2;
    let ra = r32(c, RA);
    c.g[V0][0] = 0;
    c.pc = ra;
}

/// `void UIMenu_Load(UIMenu* self, void* resource /*a1*/)` (0x39aac0): load
/// the object, record the list count in `+0x96`/`+0x97`, and set the default
/// navigation buttons.
pub fn uimenu_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_OBJECT_LOAD.call(c, 0x0039_aad4)?;
        c.g[A0][0] = sx(p(r32(c, S0), 0x74));
        callee::UI_LIST_COUNT.call(c, 0x0039_aadc)?;
        c.g[A0][0] = c.g[S0][0];
        c.sb(p(r32(c, S0), 0x96), c.g[V0][0]);
        c.sb(p(r32(c, S0), 0x97), c.g[V0][0]);
        c.g[A1][0] = 0;
        callee::UI_MENU_SET_NAV_BUTTONS.call(c, 0x0039_aaf0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* UIInputVisitor_Visit(Visitor* unused, UIObject* obj /*a1*/)`
/// (0x3a4d00): call the object's accept method (virtual slot 0x1e) with
/// `obj->+4`; return the object (or null when it was not accepted).
pub fn ui_input_visitor_visit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[V1][0] = Addr(r32(c, S0)).off(8).lw(c);
        c.g[A1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x78).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x7c).lw(c);
        let func = r32(c, V0);
        c.g[RA][0] = sx(0x003a_4d28);
        c.g[A0][0] = add32(c.g[S0][0], c.g[A0][0]);
        c.pc = func;
        call_indirect(c, func);
        if c.pc != 0x003a_4d28 {
            return Err(crate::call::Unwound);
        }
        if c.g[V0][0] == 0 {
            c.g[S0][0] = 0;
        }
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        leave(c, 0x20);
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

/// `void* AV_Player_ReleaseChunk(AV_Player* self, Chunk* chunk /*a1*/)`
/// (0x3aece0): release the chunk's stream block, then run its destructor
/// through `D_00509434[0]`.
pub fn av_player_release_chunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[A0][0] = Addr(abi::arg(c, 0)).off(0x30).lw(c);
        c.g[A1][0] = Addr(r32(c, S0)).lw(c);
        callee::STREAM_FILE_RELEASE_CHUNK.call(c, 0x003a_ecfc)?;
        let null_chunk = c.g[S0][0] == 0;
        c.g[V1][0] = 0x0051_0000;
        if !null_chunk {
            c.cycles += 3;
            c.g[V0][0] = Addr(0x0051_0000u32.wrapping_sub(0x6bcc)).lw(c);
            let func = r32(c, V0);
            c.g[RA][0] = sx(0x003a_ed10);
            c.g[A0][0] = c.g[S0][0];
            c.pc = func;
            call_indirect(c, func);
            if c.pc != 0x003a_ed10 {
                return Err(crate::call::Unwound);
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        leave(c, 0x20);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider metrics / BE.

/// `float RiderMetrix_OnKnockdownOpponent(RiderMetrix* self)` (0x119400):
/// bump the knockdown count at `+0x128` and pop it up as HUD number 0x2c.
pub fn rider_metrix_on_knockdown_opponent(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[AT][0] = 0x3fc0_0000;
        c.mtc1(1, 12);
        c.g[A1][0] = 0x2c;
        c.g[V0][0] = Addr(r32(c, V1)).off(0x128).lw(c);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        c.sw(p(r32(c, V1), 0x128), c.g[V0][0]);
        callee::RIDER_METRIX_SHOW_HUD_NUMBER.call(c, 0x0011_9430)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        frame.restore_ra(c, 0);
        leave(c, 0x10);
        Ok(())
    })
}

/// `float RiderMetrix_StartHandplant(RiderMetrix* self, u32 arg)` (0x119bf0):
/// arm the handplant timer (`+0x7c`, `+0x78`), set `+0x30` to -1 and advance
/// `+0x14` by the frame step at gp-0x7a04.
pub fn rider_metrix_start_handplant(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.lwc1(2, gp(c, -0x7a04));
        c.lwc1(0, p(r32(c, V0), 0x14));
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 1);
        c.add_s(0, 0, 2);
        c.sw(p(r32(c, V0), 0x7c), c.g[A1][0]);
        c.swc1(1, p(r32(c, V0), 0x30));
        c.sw(p(r32(c, V0), 0x78), 0);
        c.swc1(0, p(r32(c, V0), 0x14));
        callee::RIDER_METRIX_CLEAR_PEAK_A4.call(c, 0x0011_9c24)?;
        c.mtc1(0, 0);
        frame.restore_ra(c, 0);
        leave(c, 0x10);
        Ok(())
    })
}

/// `int* BEScoreInt_GetHighScoreTable(int a, int b)` (0x14ab20): the high
/// score table for the event record (`D_00535C18 + type*100`), null for the
/// record type 26.
pub fn be_score_int_get_high_score_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::BE_GET_EVENT_RECORD_INDEX.call(c, 0x0014_ab30)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 0x1a;
        let unknown = c.g[A0][0] == c.g[V0][0];
        c.g[V1][0] = 0x64;
        if !unknown {
            c.cycles += 5;
            c.g[V0][0] = 0x0053_0000;
            c.mult(0, V1, A0, V1);
            c.g[V0][0] = sx(0x0053_0000u32.wrapping_add(0x5c18));
            c.g[V0][0] = add32(c.g[V1][0], c.g[V0][0]);
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        leave(c, 0x10);
        Ok(())
    })
}

/// `void Bolts_DestroyDB(Bolts* self, u32 flags)` (0x14c658): free the bolt
/// database and, with bit 0 of `flags`, the object itself.
pub fn bolts_destroy_db(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::BOLTS_FREE_DB.call(c, 0x0014_c674)?;
        c.g[S0][0] &= 1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            callee::BUILTIN_DELETE.call(c, 0x0014_c688)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BE_RestoreCharProfileEquipFlags(Profile* self)` (0x1515b8): for each
/// item (flags halfword at `+2`, `self+0x290`, stride 4), set bit 4 when bit
/// 2 is set, clear it otherwise.
pub fn be_restore_char_profile_equip_flags(c: &mut Cpu) {
    c.cycles += 3;
    let base = abi::arg(c, 0);
    c.g[V0][0] = Addr(base).off(0x28c).lw(c);
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[A0][0] = sx(p(base, 0x290));
    if none {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = 0xffff_ffff_ffff_ffef;
    loop {
        c.cycles += 4;
        c.g[V1][0] = Addr(r32(c, A0)).off(2).lhu(c);
        c.g[V0][0] = c.g[V1][0] & 4;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = c.g[V1][0] | 0x10;
        } else {
            c.cycles += 1;
            c.g[V0][0] = c.g[V1][0] & c.g[A2][0];
        }
        c.cycles += 4;
        c.sh(p(r32(c, A0), 2), c.g[V0][0]);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
        let more = c.g[A1][0] != 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0015_15d0);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable, FAKE, LOG_RESULTS};
    use crate::difftest::{check, check_lohi, DATA, GAME_GP};

    const SEEDS: u32 = 200;

    /// Call a generated helper as a setup step (arguments in a0..t1).
    fn load(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(
            c.pc,
            crate::difftest::RA,
            "setup call {addr:#x} did not return"
        );
        c.g[V0][0]
    }

    /// The main heap's arena over 256 KiB of test RAM, so constructors and
    /// destructors that allocate or free take their normal path.
    fn alloc_ready(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[GP][0] = GAME_GP as u64;
        load(
            c,
            0x0031_ff60,
            ssx3_ee::s10::f_0031ff60,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
        c.bus.write32(GAME_GP + 0xdd8, 1); // size-class pools ready
    }

    /// A block from the test heap (`alloc_ready` must have run first).
    fn heap_block(c: &mut Cpu, size: u32) -> u32 {
        const MEM_HEAP: u32 = 0x004f_f250;
        load(
            c,
            0x0032_0058,
            ssx3_ee::s10::f_00320058,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32
    }

    /// The allocator mutex: pretend "this thread" already holds it so the
    /// real `EAMutex_Lock` takes its fast path.
    fn allocator_mutex(c: &mut Cpu) {
        c.bus.write32(0x0053_8b00 + 4, 0x42);
        c.bus.write32(LOG_RESULTS, 0x42);
    }

    // ------------------------------------------------------------------
    // Game state / mode dispatch.

    #[test]
    fn game_state_pre_race_menu_update_matches() {
        check(
            ee::s06::f_00236c48__generated,
            game_state_pre_race_menu_update,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x236c48, 0);
                let mode = seed % 4;
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at, 0);
                c.bus
                    .write32(GAME_GP - 0x204, if mode < 2 { 0 } else { DATA + 0x300 });
                c.bus.write32(DATA + 0x304, (mode == 2) as u32);
                c.bus.write32(self_at + 8, (mode % 2 == 0) as u32);
            },
        );
    }

    fn handler_test(generated: GuestFn, hand: GuestFn, salt: u64, slot: u32) {
        check(generated, hand, SEEDS, &move |c, seed| {
            let mut r = machine(c, seed, salt, 0);
            let obj = DATA + 0x100;
            let vt = vtable(c, &mut r, &[slot]);
            let index = seed % 4;
            c.bus.write32(0x0053_6668 + 4 * index, obj);
            c.bus.write32(obj, vt);
            c.bus.write32(obj + 4, r.next_u32());
            c.g[A0][0] = (DATA + 0x200) as u64;
            c.bus.write32(DATA + 0x204, index);
        });
    }

    #[test]
    fn game_mode_man_handler_v06_matches() {
        handler_test(
            ee::s06::f_00238510__generated,
            game_mode_man_handler_v06,
            0x238510,
            0x30,
        );
    }

    #[test]
    fn game_mode_man_handler_v07_matches() {
        handler_test(
            ee::s06::f_00238550__generated,
            game_mode_man_handler_v07,
            0x238550,
            0x38,
        );
    }

    #[test]
    fn game_mode_man_handler_v09_matches() {
        handler_test(
            ee::s06::f_00238590__generated,
            game_mode_man_handler_v09,
            0x238590,
            0x48,
        );
    }

    #[test]
    fn mc_manager_create_card_matches() {
        check(
            ee::s06::f_0023d5a8__generated,
            mc_manager_create_card,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23d5a8, 0);
                alloc_ready(c);
                allocator_mutex(c);
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.bus.write32(GAME_GP - 0x5128, r.next_u32());
            },
        );
    }

    // ------------------------------------------------------------------
    // Lobby / buddies / login.

    #[test]
    fn lobby_man_is_room_user_flagged_matches() {
        check(
            ee::s06::f_0025f248__generated,
            lobby_man_is_room_user_flagged,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x25f248, 0);
                let self_at = DATA + 0x200;
                let list = DATA + 0x300;
                let arr = DATA + 0x400;
                let item = DATA + 0x500;
                let mode = seed % 4;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = 0; // room-user index
                c.bus.write32(self_at, (mode != 0) as u32);
                c.bus
                    .write32(self_at + 0x60, if mode == 1 { 0 } else { list });
                c.bus.write32(list + 0x18, if mode == 2 { 0 } else { 1 });
                c.bus.write32(list + 0x40, arr);
                c.bus.write32(arr, item);
                c.bus.write32(item, 1);
                c.bus.write32(item + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn buddy_man_reconnect_matches() {
        check(
            ee::s06::f_002613c8__generated,
            buddy_man_reconnect,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2613c8, 0);
                let self_at = DATA + 0x200;
                let mode = seed % 3;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(
                    self_at + 0x40,
                    if mode == 0 {
                        0xffff_ffff
                    } else {
                        (seed % 4) as u32
                    },
                );
                c.bus.write32(self_at + 0xd8, (mode <= 1) as u32);
                c.bus.write32(self_at + 0xc8, 1); // the buddy API already exists
                c.bus.write32(self_at + 0xec, r.next_u32());
            },
        );
    }

    #[test]
    fn buddy_man_roster_compare_matches() {
        check(
            ee::s07::f_002634c0__generated,
            buddy_man_roster_compare,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2634c0, 0);
                let a = DATA + 0x200;
                let b = DATA + 0x300;
                c.bus.write_bytes(a, b"alpha\0");
                let rhs: &[u8] = match seed % 5 {
                    0 => &b"alpha\0"[..],
                    1 => &b"ALPHA\0"[..],
                    2 => &b"beta\0"[..],
                    3 => &b"\0"[..],
                    _ => &b"alphan\0"[..],
                };
                c.bus.write_bytes(b, rhs);
                let (pa, pb) = match seed % 6 {
                    0 => (0u32, 0u32),
                    1 => (0, b),
                    2 => (a, 0),
                    _ => (a, b),
                };
                c.g[A2][0] = pa as u64;
                c.g[A3][0] = pb as u64;
            },
        );
    }

    #[test]
    fn c_shared_login_man_mc_get_info_matches() {
        check(
            ee::s07::f_00266060__generated,
            c_shared_login_man_mc_get_info,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x266060, 0);
                c.g[A0][0] = (seed % 5) as u64;
                c.bus.write32(GAME_GP + 0x2e8, DATA + 0x400);
            },
        );
    }

    // ------------------------------------------------------------------
    // Path readers.

    #[test]
    fn path_read_a_i_field_matches() {
        check(
            ee::s07::f_00269ed8__generated,
            path_read_a_i_field,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x269ed8, 0);
                let dst = DATA + 0x200;
                c.g[A0][0] = dst as u64;
                let id = match seed % 3 {
                    0 => 0x64u64,
                    1 => 0x65,
                    _ => r.next_u32() as u64 | 0x100,
                };
                c.g[A2][0] = id;
                c.g[A1][0] = if seed % 5 == 0 {
                    (DATA + 0x400) as u64
                } else {
                    (dst + 0x38) as u64
                };
            },
        );
    }

    #[test]
    fn path_read_segment_matches() {
        check(ee::s07::f_0026a338__generated, path_read_segment, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x26a338, 0);
            let src = DATA + 0x300;
            let dst = if seed % 3 == 0 { src } else { DATA + 0x400 };
            c.g[A1][0] = src as u64;
            c.g[A2][0] = dst as u64;
            for k in 0..4u32 {
                let v = r.next_u32();
                c.bus.write32(src + 4 * k, v);
            }
        });
    }

    // ------------------------------------------------------------------
    // Script engine.

    #[test]
    fn script_engine_list_add_choice_matches() {
        check_lohi(
            ee::s07::f_00278e50__generated,
            script_engine_list_add_choice,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x278e50, 0);
                let self_at = DATA + 0x200;
                let choices = DATA + 0x1000;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = (seed % 4) as u64;
                // The choice's other arguments: keep them null so the listener
                // does not chase scrambled pointers.
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
                c.g[T1][0] = 0;
                c.g[T2][0] = 0;
                c.g[T3][0] = 0;
                c.bus.write32(self_at + 0x54c, choices);
            },
        );
    }

    #[test]
    fn script_engine_is_bank_slot_loaded_matches() {
        check(
            ee::s07::f_0027bf50__generated,
            script_engine_is_bank_slot_loaded,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x27bf50, 0);
                let self_at = DATA + 0x200;
                let slot = seed % 4;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = slot as u64;
                c.bus
                    .write32(self_at + 0x630 + 16 * slot, (seed % 2) as u32);
            },
        );
    }

    #[test]
    fn script_engine_exit_script_mode_matches() {
        check(
            ee::s07::f_0027bf90__generated,
            script_engine_exit_script_mode,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x27bf90, 0);
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(
                    self_at + 0x550,
                    if seed % 2 == 0 { 2 } else { (seed % 4) as u32 },
                );
            },
        );
    }

    #[test]
    fn script_cam_binding_ctor_matches() {
        check(
            ee::s07::f_00283088__generated,
            script_cam_binding_ctor,
            SEEDS,
            &|c, _seed| {
                let _ = machine(c, 0, 0x283088, 0);
                c.g[A0][0] = (DATA + 0x200) as u64;
            },
        );
    }

    #[test]
    fn script_obj_register_matches() {
        check(
            ee::s07::f_00283298__generated,
            script_obj_register,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x283298, 0);
                let engine = DATA + 0x400;
                let vt = vtable(c, &mut r, &[0x10]);
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.bus.write32(GAME_GP + 0x3ec, engine);
                c.bus.write32(engine + 0x2a8, vt);
            },
        );
    }

    #[test]
    fn script_sound_bank_man_free_entry_matches() {
        check(
            ee::s07::f_00283720__generated,
            script_sound_bank_man_free_entry,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x283720, 0);
                let sys = DATA + 0x300;
                let banks = DATA + 0x400;
                let entry = DATA + 0x500;
                c.g[A1][0] = entry as u64;
                c.bus.write32(GAME_GP + 0x410, sys);
                c.bus.write32(sys + 0x118, banks);
                c.bus.write32(banks, DATA + 0x600);
                c.bus.write32(entry + 4, (seed % 16) as u32);
                c.bus.write32(entry, (seed % 9) as u32);
            },
        );
    }

    #[test]
    fn script_movie_set_paused_matches() {
        check(
            ee::s07::f_00283b78__generated,
            script_movie_set_paused,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x283b78, 0);
                let m = DATA + 0x200;
                c.g[A0][0] = m as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(m, if seed % 4 == 0 { 0 } else { 1 });
                c.bus.write32(m + 4, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(
                    m + 0xc,
                    match seed % 3 {
                        0 => 0,
                        1 => 1,
                        _ => seed % 7,
                    },
                );
            },
        );
    }

    // ------------------------------------------------------------------
    // Audio.

    #[test]
    fn ssx_audio_system_destroy_matches() {
        check(
            ee::s07::f_00284c28__generated,
            ssx_audio_system_destroy,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x284c28, 0);
                let sys = DATA + 0x300;
                let inner = DATA + 0x400;
                let obj = DATA + 0x500;
                let vt = vtable(c, &mut r, &[8]);
                c.bus
                    .write32(GAME_GP + 0x410, if seed % 3 != 0 { sys } else { 0 });
                c.bus.write32(sys + 0x118, inner);
                c.bus.write32(inner, obj);
                c.bus.write32(obj + 0x1d4, vt);
            },
        );
    }

    #[test]
    fn ssx_audio_system_unload_grunt_banks_matches() {
        check(
            ee::s08::f_0029f5e0__generated,
            ssx_audio_system_unload_grunt_banks,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x29f5e0, 0);
                let self_at = DATA + 0x200;
                let bank_man = DATA + 0x300;
                let banks = DATA + 0x400;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at + 0x118, bank_man);
                c.bus.write32(bank_man, banks);
                c.bus.write32(banks + 0xacc, (seed % 3) as u32);
            },
        );
    }

    #[test]
    fn ssx_audio_system_stop_speech_on_reset_matches() {
        check(
            ee::s08::f_002a4b68__generated,
            ssx_audio_system_stop_speech_on_reset,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2a4b68, 0);
                let self_at = DATA + 0x200;
                let player = self_at + 0x118;
                c.g[A0][0] = self_at as u64;
                let tagged = seed % 2 == 0;
                c.bus.write32(player + 0x418, tagged as u32);
                if tagged {
                    let song = DATA + 0x400;
                    c.bus.write32(player + 0x408, song);
                    c.bus.write32(song + 0x58, 0x191);
                }
            },
        );
    }

    #[test]
    fn bank_monitor_kill_all_timers_matches() {
        check(
            ee::s08::f_002adda0__generated,
            bank_monitor_kill_all_timers,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2adda0, 0);
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                for k in 0..4u32 {
                    c.bus.write32(self_at + 0x98 + 0x2c * k, seed + k);
                }
            },
        );
    }

    #[test]
    fn path_plat_set_track_file_matches() {
        check(
            ee::s08::f_002ae768__generated,
            path_plat_set_track_file,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2ae768, 0);
                let plat = DATA + 0x300;
                for k in 0..4u32 {
                    let entry = if (seed / 4 + k) % 4 == 0 {
                        0
                    } else {
                        plat + 0x80 * k
                    };
                    c.bus.write32(0x004d_3e98 + 4 * k, entry);
                    c.bus
                        .write32(plat + 0x80 * k + 0x64, ((seed + k) % 3) as u32 & 1);
                }
                let idx = if seed % 6 == 5 {
                    0x18u64
                } else {
                    (seed % 4) as u64
                };
                c.g[A0][0] = idx;
                c.g[A1][0] = (seed % 1000) as u64;
                c.g[A2][0] = (0x1000 + seed) as u64;
            },
        );
    }

    #[test]
    fn path_plat_clear_prebuffer_matches() {
        check(
            ee::s08::f_002ae9f8__generated,
            path_plat_clear_prebuffer,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2ae9f8, 0);
                let plat = DATA + 0x300;
                let idx = seed % 4;
                c.g[A0][0] = idx as u64;
                for k in 0..4u32 {
                    c.bus.write32(0x004d_3e98 + 4 * k, plat);
                }
                c.bus.write32(plat + 0x64, ((seed / 4) % 2) as u32);
                c.bus.write32(plat + 0x6c, DATA + 0x600);
                c.bus.write32(DATA + 0x604, (seed % 8) as u32);
            },
        );
    }

    #[test]
    fn song_instance_pick_random_loop_matches() {
        check_lohi(
            ee::s08::f_002b24c0__generated,
            song_instance_pick_random_loop,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2b24c0, 0);
                let self_at = DATA + 0x200;
                let song = DATA + 0x300;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at, song);
                c.bus.write32(
                    song + 0x40,
                    match seed % 3 {
                        0 => 0,
                        1 => 1,
                        _ => 1 + seed % 7,
                    },
                );
                c.bus.write32(
                    GAME_GP + 0x68c,
                    if seed % 4 == 0 { 0 } else { seed * 0x9e37_79b9 },
                );
            },
        );
    }

    #[test]
    fn song_instance_get_position_ms_matches() {
        check(
            ee::s08::f_002b2600__generated,
            song_instance_get_position_ms,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2b2600, 0);
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at, (seed % 4) as u32);
                c.bus.write32(self_at + 0x14, (seed % 6) as u32);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_request_list_dir_w_matches() {
        check(
            ee::s08::f_002c4198__generated,
            c_ps2_mem_card_request_list_dir_w,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c4198, 0);
                let self_at = DATA + 0x200;
                let vt = vtable(c, &mut r, &[0x30]);
                c.bus.write16(DATA + 0x300, b'd' as u16);
                c.bus.write16(DATA + 0x302, b'i' as u16);
                c.bus.write16(DATA + 0x304, b'r' as u16);
                c.bus.write16(DATA + 0x306, 0);
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.bus.write32(self_at, vt);
            },
        );
    }

    #[test]
    fn menu_sys_action_item_draw_matches() {
        check(
            ee::s09::f_002cd0e8__generated,
            menu_sys_action_item_draw,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2cd0e8, 0);
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at + 0x14, (seed % 5) as u32);
                c.bus
                    .write32(self_at + 0x18, if seed % 2 == 0 { 0 } else { seed % 9 });
            },
        );
    }

    #[test]
    fn transition_effect_free_data_matches() {
        check(
            ee::s09::f_002e4d30__generated,
            transition_effect_free_data,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2e4d30, 0);
                const BASE: u32 = 0x0053_82c0;
                let choices = [0u32, BASE, BASE + 0x64, BASE + 0x64 * 3, 0x1234_5678];
                c.g[A0][0] = choices[(seed as usize) % choices.len()] as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // WScript tasks.

    #[test]
    fn wscript_task_start_matches() {
        check(
            ee::s10::f_003086b0__generated,
            wscript_task_start,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x3086b0, 0);
                let self_at = DATA + 0x200;
                let mgr = DATA + 0x300;
                c.g[A0][0] = self_at as u64;
                c.bus
                    .write32(self_at, if seed % 3 == 0 { 1 } else { (seed % 4) as u32 });
                c.bus
                    .write32(self_at + 8, if seed % 2 == 0 { 0 } else { 1 + seed % 5 });
                c.bus.write32(self_at + 0x3c, mgr);
                c.bus.write32(mgr + 0x10, (seed % 3) as u32);
            },
        );
    }

    #[test]
    fn wscript_task_stop_matches() {
        check(ee::s10::f_00308720__generated, wscript_task_stop, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x308720, 0);
            let self_at = DATA + 0x200;
            let mgr = DATA + 0x300;
            c.g[A0][0] = self_at as u64;
            c.bus
                .write32(self_at, if seed % 3 == 0 { 3 } else { (seed % 4) as u32 });
            c.bus
                .write32(self_at + 0x10, if seed % 2 == 0 { 0 } else { 1 + seed % 5 });
            c.bus.write32(self_at + 0x3c, mgr);
            c.bus.write32(mgr + 0x10, (seed % 3) as u32);
        });
    }

    #[test]
    fn wscript_man_activate_process_matches() {
        check(
            ee::s10::f_00309a60__generated,
            wscript_man_activate_process,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x309a60, 0);
                let self_at = DATA + 0x200;
                let node = DATA + 0x400;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = node as u64;
                c.bus.write32(node, (seed % 3) as u32);
                c.bus
                    .write32(node + 4, if seed % 2 == 0 { 0 } else { node });
            },
        );
    }

    #[test]
    fn wscript_man_collect_set_get_count_matches() {
        check(
            ee::s10::f_0030c468__generated,
            wscript_man_collect_set_get_count,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30c468, 0);
                let lib = DATA + 0x800;
                let obj = DATA + 0x900;
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(GAME_GP - 0x1f04, lib); // BELibrary singleton
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, obj);
                c.bus.write32(obj + 4, vt);
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.bus.write32(DATA + 0x200, (seed % 20) as u32);
                c.bus.write32(GAME_GP + 0x8d8, 0);
            },
        );
    }

    

    #[test]
    fn game_anim_map_set_handle_matches() {
        check(
            ee::s10::f_00314ef0__generated,
            game_anim_map_set_handle,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x314ef0, 0);
                let map = DATA + 0x200;
                c.g[A0][0] = map as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    1 + (seed % 5) as u64
                };
                c.g[A2][0] = (seed as u64) | 0x1000;
            },
        );
    }

    #[test]
    fn input_parser_alloc_node_matches() {
        check(
            ee::s10::f_003215f8__generated,
            input_parser_alloc_node,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x3215f8, 0);
                let self_at = DATA + 0x200;
                let node = DATA + 0x400;
                c.g[A0][0] = self_at as u64;
                if seed % 3 == 0 {
                    c.bus.write32(self_at + 0xe0, 0);
                } else {
                    c.bus.write32(self_at + 0xe0, node);
                    c.bus
                        .write32(node + 4, if seed % 2 == 0 { 0 } else { DATA + 0x500 });
                }
            },
        );
    }

    #[test]
    fn crowd_man3d_group_init_matches() {
        check(
            ee::s11::f_00344368__generated,
            crowd_man3d_group_init,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x344368, 0);
                let mgr = DATA + 0x300;
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(GAME_GP + 0x2a74, mgr);
                c.bus.write32(mgr + 0x10, 1 + seed % 4);
            },
        );
    }

    #[test]
    fn instance_has_anim_nodes_matches() {
        check(
            ee::s11::f_00350698__generated,
            instance_has_anim_nodes,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x350698, 0);
                let self_at = DATA + 0x200;
                let list = DATA + 0x300;
                let entries = DATA + 0x400;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at + 0x80, list);
                c.bus.write32(list + 4, (seed % 4) as u32);
                c.bus.write32(list + 8, entries);
                for k in 0..3u32 {
                    let hit = seed % 2 == 0 && k == seed % 3;
                    c.bus.write32(entries + 0x10 * k + 8, hit as u32);
                }
            },
        );
    }

    // ------------------------------------------------------------------
    // Graphics / UI.

    #[test]
    fn ps2_graphics_man_set_default_display_mode_matches() {
        check(
            ee::s12::f_00376578__generated,
            ps2_graphics_man_set_default_display_mode,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x376578, 0);
                let self_at = DATA + 0x200;
                let vt = vtable(c, &mut r, &[0x3d0]);
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(self_at + 0x10d8, vt);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_free_texture_matches() {
        check(
            ee::s12::f_0037cb50__generated,
            ps2_graphics_man_free_texture,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x37cb50, 0);
                alloc_ready(c);
                let self_at = DATA + 0x200;
                let texman = DATA + 0x300;
                let idx = seed % 4;
                // The texture record is a real (heap) block, so the delete at
                // the end of the free path is legitimate, and its `+0x28` /
                // `+0x2c` / `+0` say "no VRAM block": the VRAM heap (zeroed on
                // a test machine) is then never walked.
                let rec = heap_block(c, 0x400);
                for off in [
                    0u32, 4, 8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x30, 0x34,
                ] {
                    c.bus.write32(rec + off, 0);
                }
                c.bus.write32(rec + 0x28, 0xffff_ffff);
                c.bus.write32(rec + 0x2c, 0xffff_ffff);
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = idx as u64;
                c.bus.write32(self_at + 0x18f4, texman);
                c.bus.write32(self_at + 0x18f8 + 4 * idx, seed % 5);
                c.bus.write32(texman + 8 + 4 * idx, rec);
            },
        );
    }

    #[test]
    fn ui_list_box_find_item_by_value_matches() {
        check(
            ee::s13::f_0039a768__generated,
            ui_list_box_find_item_by_value,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x39a768, 0);
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = (seed % 7) as u64;
                c.g[A2][0] = (DATA + 0x700) as u64;
                c.bus.write8(self_at + 0x318, (seed % 4) as u8);
                for k in 0..3u32 {
                    c.bus.write32(self_at + 0xa4 + 0x14 * k, k * 3);
                }
            },
        );
    }

    #[test]
    fn uimenu_load_matches() {
        check(ee::s13::f_0039aac0__generated, uimenu_load, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x39aac0, 0);
            let self_at = DATA + 0x200;
            let res = DATA + 0x400;
            c.g[A0][0] = self_at as u64;
            c.g[A1][0] = res as u64;
            c.bus.write32(res + 4, (seed % 3) as u32);
            c.bus.write32(res + 8, (seed % 5) as u32);
            c.bus.write32(self_at + 0x14, (seed % 3) as u32);
            c.bus.write32(self_at + 0x74, 0);
        });
    }

    #[test]
    fn ui_input_visitor_visit_matches() {
        check(
            ee::s13::f_003a4d00__generated,
            ui_input_visitor_visit,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x3a4d00, 0);
                let obj = DATA + 0x200;
                let vt = DATA + 0x600;
                // A real, harmless method: the script engine singleton (0 when the
                // global is clear), so both the accept and reject results occur.
                c.bus.write16(vt + 0x78, 0);
                c.bus.write32(vt + 0x7c, 0x0027_d2e8);
                c.g[A0][0] = (DATA + 0x300) as u64;
                c.g[A1][0] = obj as u64;
                c.bus.write32(DATA + 0x304, (seed % 4) as u32);
                c.bus.write32(obj + 8, vt);
                c.bus.write32(
                    GAME_GP + 0x3ec,
                    if seed % 2 == 0 { 0 } else { DATA + 0x500 },
                );
            },
        );
    }

    #[test]
    fn world_memman_free_section_allocator_matches() {
        check(
            ee::s13::f_003a7b98__generated,
            world_memman_free_section_allocator,
            SEEDS,
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
    fn av_player_release_chunk_matches() {
        check(
            ee::s13::f_003aece0__generated,
            av_player_release_chunk,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x3aece0, 0);
                let self_at = DATA + 0x200;
                let chunk = DATA + 0x300;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { chunk as u64 };
                c.bus.write32(self_at + 0x30, DATA + 0x500);
                c.bus.write32(chunk, (seed % 6) as u32);
                c.bus.write32(0x0050_9434, FAKE);
            },
        );
    }

    // ------------------------------------------------------------------
    // Rider metrics / BE.

    #[test]
    fn rider_metrix_on_knockdown_opponent_matches() {
        check(
            ee::s00::f_00119400__generated,
            rider_metrix_on_knockdown_opponent,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x119400, 0);
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at + 0x128, (seed % 7) as u32);
            },
        );
    }

    #[test]
    fn rider_metrix_start_handplant_matches() {
        check(
            ee::s00::f_00119bf0__generated,
            rider_metrix_start_handplant,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x119bf0, 0);
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.bus.write32(self_at + 0x14, 0x4120_0000 | (seed & 0xfff));
                c.bus.write32(GAME_GP - 0x7a04, 0x3ca3_d70a);
            },
        );
    }

    #[test]
    fn be_score_int_get_high_score_table_matches() {
        check_lohi(
            ee::s01::f_0014ab20__generated,
            be_score_int_get_high_score_table,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x14ab20, 0);
                let (a, b) = match seed % 4 {
                    0 => (0u64, 6u64),
                    1 => (0, 7),
                    2 => (16, 7),
                    _ => (0, 12),
                };
                c.g[A0][0] = a;
                c.g[A1][0] = b;
            },
        );
    }

    #[test]
    fn bolts_destroy_db_matches() {
        check(ee::s01::f_0014c658__generated, bolts_destroy_db, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x14c658, 0);
            alloc_ready(c);
            c.g[A0][0] = (DATA + 0x200) as u64;
            c.g[A1][0] = (seed % 3) as u64;
        });
    }

    #[test]
    fn be_restore_char_profile_equip_flags_matches() {
        check(
            ee::s01::f_001515b8__generated,
            be_restore_char_profile_equip_flags,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x1515b8, 0);
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at + 0x28c, (seed % 4) as u32);
                for k in 0..3u32 {
                    let flags = match (seed + k) % 4 {
                        0 => 0x0004u16,
                        1 => 0x0010,
                        2 => 0x0014,
                        _ => 0x00ff,
                    };
                    c.bus.write16(self_at + 0x290 + 4 * k + 2, flags);
                }
            },
        );
    }
}
