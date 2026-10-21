//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for a batch of small, cold-path functions (input map file,
//! wipe transitions, menu/UI destructors and state helpers, net/chat FE
//! states, lobby/buddy list teardown, replay save buffer, music/sound queue,
//! script and world-state helpers), each with a differential test against the
//! generated body.
//!
//! These are literal ports of the generated code: every block charge, every
//! register the original leaves behind (notably `v0`/`v1` and the scratch
//! registers a callee may observe) and the call protocol of [`crate::call`]
//! are preserved. Loop back edges deliver events with
//! [`crate::events::event_due`] + [`deliver_event_here`], the two dispatcher
//! shaped functions (`BuddyMan_DestroyApi`, `BuddyMan_GetChallengerByName`)
//! accept the same resume points as the generated `match pc` loop.

use crate::abi::{self};
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0032_0f10,
        input_map_finalize_opcodes,
        "InputMap_FinalizeOpcodes",
    ),
    (0x002e_bba8, transition_wipe_ctor, "TransitionWipe_ctor"),
    (0x0039_73a8, ui_anim_dtor, "UIAnim_dtor"),
    (0x0022_e730, game_view_man_dtor, "GameViewMan_dtor"),
    (
        0x001a_4d90,
        fe_net_messenger_show_looking_for_self,
        "cFEStateNetMessenger_ShowLookingForSelf",
    ),
    (
        0x001a_56d0,
        fe_net_messenger_has_name,
        "cFEStateNetMessenger_HasName",
    ),
    (
        0x001a_5800,
        fe_net_messenger_find_name_str,
        "cFEStateNetMessenger_FindNameStr",
    ),
    (
        0x001a_94e0,
        net_fe_state_on_keyboard_popup_event,
        "NetFEState_OnKeyboardPopupEvent",
    ),
    (
        0x001b_3490,
        fe_state_chat_room_transition_in,
        "cFEStateChatRoom_TransitionIn",
    ),
    (
        0x001c_04a8,
        fe_state_online_agreement_init,
        "cFEStateOnlineAgreement_Init",
    ),
    (
        0x001c_0ea8,
        fe_state_online_agreement_v25,
        "cFEStateOnlineAgreement_v25",
    ),
    (
        0x0025_a8b0,
        lobby_man_destroy_lists,
        "LobbyMan_DestroyLists",
    ),
    (0x0026_1530, buddy_man_destroy_api, "BuddyMan_DestroyApi"),(0x002a_f7a0, path_plat_set_paused, "PATH_PlatSetPaused"),
    (
        0x002b_43d8,
        music_player_load_loop_bank,
        "MusicPlayer_LoadLoopBank",
    ),
    (
        0x0027_0fc0,
        replay_flush_save_output,
        "Replay_FlushSaveOutput",
    ),
    (
        0x0027_1228,
        replay_save_build_header,
        "Replay_SaveBuildHeader",
    ),
    (
        0x0027_4c70,
        in_seq_instance_get_length,
        "InSeq_InstanceGetLength",
    ),
    (
        0x0016_1e58,
        chase_cam_controller_push_algorithm,
        "ChaseCamController_PushAlgorithm",
    ),(0x0023_fc70, mc_manager_state_24, "MCManager_State24"),
    (0x0025_0090, tube_menu_dtor, "TubeMenu_dtor"),
    (
        0x0028_7488,
        ssx_audio_system_set_sfx_volume,
        "SSXAudioSystem_SetSfxVolume",
    ),
    (
        0x0023_6bb0,
        game_state_prerace_menu_enter,
        "GameStatePreRaceMenu_Enter",
    ),
    (
        0x0028_bd10,
        snd_bank_man_load_bank_async,
        "SndBankMan_LoadBankAsync",
    ),
    (
        0x0028_d740,
        ssx_audio_system_confirm_music_trigger,
        "SSXAudioSystem_ConfirmMusicTrigger",
    ),
    (
        0x0030_7ec0,
        w_script_process_jump_to_task,
        "WScriptProcess_JumpToTask",
    ),(0x002d_f3b0, dynamic_spray_fx_reset, "cDynamicSprayFX_Reset"),
    (
        0x0014_5dd0,
        be_new_player_int_set_char_profile_flag,
        "BENewPlayerInt_SetCharProfileFlag",
    ),
    (0x0023_c770, game_rand_shuffle, "GameRand_Shuffle"),
    (
        0x002b_6740,
        world_trigger_manager_any_trigger_flagged,
        "WorldTriggerManager_AnyTriggerFlagged",
    ),
    (0x0027_5828, data_instance_man_ctor, "DataInstanceMan_ctor"),
    (
        0x0027_76e0,
        script_list_check_next_ready,
        "ScriptList_CheckNextReady",
    ),
    (
        0x002e_cb28,
        bolt_on_fx_interface_dtor,
        "cBoltOnFXInterface_dtor",
    ),
    (
        0x003a_95c0,
        world_cache_streamer_are_node_hulls_free,
        "cWorldCache_StreamerAreNodeHullsFree",
    ),
    (
        0x003a_18c0,
        ui_text_scroll_clear_lines,
        "cUITextScroll_ClearLines",
    ),
    (0x0035_84f0, rail_man_ctor, "RailMan_ctor"),
    (0x002d_6378, vfx_avalanche_dtor, "VfxAvalanche_dtor"),
    (
        0x0031_4c00,
        mdf_file_create_model_by_name,
        "MdfFile_CreateModelByName",
    ),(0x0018_74e8, fe_state_mem_card_v33, "cFEStateMemCard_v33"),(0x001f_30d8, load_indicator_init, "LoadIndicator_Init"),
    (
        0x0039_f600,
        ui_state_mgr_close_unpinned_popups,
        "UIStateMgr_CloseUnpinnedPopups",
    ),
    (
        0x002e_bb10,
        transition_effect_draw_fade,
        "TransitionEffect_DrawFade",
    ),
    (
        0x0014_e9f8,
        be_prepare_net_profiles,
        "BE_PrepareNetProfiles",
    ),
    (
        0x0015_0b88,
        be_set_char_state_flag_cheat,
        "BE_SetCharStateFlagCheat",
    ),
    (
        0x0026_ddc0,
        world_state_read_objects,
        "WorldState_ReadObjects",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

mod callee {
    use super::{ee, Callee};

    pub const BUILTIN_DELETE: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::f_00317e50,
    };
    pub const OPERATOR_NEW_NAMED: Callee = Callee {
        addr: 0x0031_7e30,
        f: ee::s10::f_00317e30,
    };
    pub const OPERATOR_NEW: Callee = Callee {
        addr: 0x0031_7e30,
        f: ee::s10::f_00317e30,
    };
    pub const EA_MEM_FREE: Callee = Callee {
        addr: 0x0031_7e98,
        f: ee::s10::f_00317e98,
    };
    pub const TRANSITION_EFFECT_CTOR: Callee = Callee {
        addr: 0x002e_4d70,
        f: ee::s09::f_002e4d70,
    };
    pub const NET_FE_STATE_GET_POPUP: Callee = Callee {
        addr: 0x001a_9c50,
        f: ee::s03::f_001a9c50,
    };
    pub const POPUP_CFG_SET_OPTION_TEXT_ID: Callee = Callee {
        addr: 0x001c_57c0,
        f: ee::s03::f_001c57c0,
    };
    pub const NET_FE_STATE_SHOW_POPUP: Callee = Callee {
        addr: 0x001a_9bc0,
        f: ee::s03::f_001a9bc0,
    };
    pub const NET_FE_STATE_ON_CHILD_EVENT_DEFAULT: Callee = Callee {
        addr: 0x001a_93d0,
        f: ee::s03::f_001a93d0,
    };
    pub const CHAT_ROOM_SELECT_PLAYER_BY_NAME: Callee = Callee {
        addr: 0x001b_3978,
        f: ee::s03::f_001b3978,
    };
    pub const LOAD_AGREEMENT_TERMS: Callee = Callee {
        addr: 0x001c_0540,
        f: ee::s03::f_001c0540,
    };
    pub const LOBBY_MAN_RELEASE_SNAPSHOT_LISTS: Callee = Callee {
        addr: 0x0025_a850,
        f: ee::s06::f_0025a850,
    };
    pub const LOBBY_MAN_SEND_DECLINE: Callee = Callee {
        addr: 0x0025_fc28,
        f: ee::s06::f_0025fc28,
    };
    pub const SONG_INSTANCE_PICK_RANDOM_LOOP: Callee = Callee {
        addr: 0x002b_24c0,
        f: ee::s08::f_002b24c0,
    };
    pub const REPLAY_COUNT_SAVE_SNAPSHOTS: Callee = Callee {
        addr: 0x0027_12f8,
        f: ee::s07::f_002712f8,
    };
    pub const IN_SEQ_CHANNEL_GET_LENGTH: Callee = Callee {
        addr: 0x0027_4348,
        f: ee::s07::f_00274348,
    };
    pub const ALGORITHM_LIST_PUSH: Callee = Callee {
        addr: 0x0015_c988,
        f: ee::s01::f_0015c988,
    };
    pub const ALGORITHM_LIST_CLEAR: Callee = Callee {
        addr: 0x0015_cb08,
        f: ee::s01::f_0015cb08,
    };
    pub const SND_BANK_IS_LOADED_WITH_NAME: Callee = Callee {
        addr: 0x0028_b830,
        f: ee::s07::f_0028b830,
    };
    pub const SND_LOAD_QUEUE_CANCEL_BANK: Callee = Callee {
        addr: 0x0028_a728,
        f: ee::s07::f_0028a728,
    };
    pub const SND_LOAD_QUEUE_PUSH: Callee = Callee {
        addr: 0x0028_a558,
        f: ee::s07::f_0028a558,
    };
    pub const CLEAR_MUSIC_TRIGGER: Callee = Callee {
        addr: 0x0028_d7d8,
        f: ee::s07::f_0028d7d8,
    };
    pub const SPRAY_CONFIGURE_FOR_RIDER: Callee = Callee {
        addr: 0x002e_26b8,
        f: ee::s09::f_002e26b8,
    };
    pub const SPRAY_UPDATE_EMITTER_ENABLES: Callee = Callee {
        addr: 0x002e_2550,
        f: ee::s09::f_002e2550,
    };
    pub const BE_GET_RIDER_PROFILE: Callee = Callee {
        addr: 0x0014_a0e0,
        f: ee::s01::f_0014a0e0,
    };
    pub const BENEW_PLAYER_INT_GET_SLOT: Callee = Callee {
        addr: 0x0014_7798,
        f: ee::s01::f_00147798,
    };
    pub const BE_SET_CHAR_PROFILE_FLAG: Callee = Callee {
        addr: 0x0015_2430,
        f: ee::s01::f_00152430,
    };
    pub const GAME_RAND_NEXT: Callee = Callee {
        addr: 0x0023_7cd8,
        f: ee::s06::f_00237cd8,
    };
    pub const WORLD_TRIGGER_IS_FLAG4: Callee = Callee {
        addr: 0x002b_67d8,
        f: ee::s08::f_002b67d8,
    };
    pub const DATA_INSTANCE_FREE_INFO: Callee = Callee {
        addr: 0x0027_5cd0,
        f: ee::s07::f_00275cd0,
    };
    pub const SCRIPT_CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::f_00283d70,
    };
    pub const SCRIPT_LIST_IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::f_00277dd8,
    };
    pub const SCRIPT_ENGINE_GET_PROCESS_STATE: Callee = Callee {
        addr: 0x0028_2798,
        f: ee::s07::f_00282798,
    };
    pub const SCRIPT_LIST_SET_PAUSED: Callee = Callee {
        addr: 0x0027_7c08,
        f: ee::s07::f_00277c08,
    };
    pub const AVALANCHE_DESTROY_ALL: Callee = Callee {
        addr: 0x002d_9b40,
        f: ee::s09::f_002d9b40,
    };
    pub const ATTACHED_EMITTER_FX_DESTROY_EMITTER: u32 = 0x002f_2218;
    pub const WORLD_CACHE_IS_HULL_FREE: Callee = Callee {
        addr: 0x003a_8638,
        f: ee::s13::f_003a8638,
    };
    pub const SYNC_PROCESS_CTOR: Callee = Callee {
        addr: 0x0035_4850,
        f: ee::s11::f_00354850,
    };
    pub const MDF_FILE_CREATE_MODEL: Callee = Callee {
        addr: 0x0031_4cb0,
        f: ee::s10::f_00314cb0,
    };
    pub const FE_MEMCARD_START_PROFILE_SEARCH: Callee = Callee {
        addr: 0x0018_77b0,
        f: ee::s02::f_001877b0,
    };
    pub const FE_MEMCARD_FINISH: Callee = Callee {
        addr: 0x0018_7318,
        f: ee::s02::f_00187318,
    };
    pub const UI_LIST_FIRST: Callee = Callee {
        addr: 0x0039_7840,
        f: ee::s13::f_00397840,
    };
    pub const TRANSITION_EFFECT_DRAW_RECT: Callee = Callee {
        addr: 0x002e_b938,
        f: ee::s09::f_002eb938,
    };
    pub const BE_RESET_PROFILE_CHARACTERS: Callee = Callee {
        addr: 0x0015_67b8,
        f: ee::s01::f_001567b8,
    };
    pub const BE_COPY_PROFILE: Callee = Callee {
        addr: 0x0014_e9c0,
        f: ee::s01::f_0014e9c0,
    };
    pub const BE_OPTION_INT_SWAP_PLAYER_FLAGS: Callee = Callee {
        addr: 0x0014_eb08,
        f: ee::s01::f_0014eb08,
    };
    pub const WORLD_STATE_CREATE_OBJECT: Callee = Callee {
        addr: 0x0026_ded8,
        f: ee::s07::f_0026ded8,
    };
    pub const AUDIO_LEVEL_TO_VOLUME: Callee = Callee {
        addr: 0x0028_73d8,
        f: ee::s07::f_002873d8,
    };
    pub const AUDIO_SET_CHANNEL_VOLUME: Callee = Callee {
        addr: 0x0028_7700,
        f: ee::s07::f_00287700,
    };

    // `call_indirect` targets the generator left as table calls.
    pub const MEMCPY: u32 = 0x003e_6574;
    pub const STRCASECMP: u32 = 0x0041_aa88;
    pub const SPRINTF: u32 = 0x0041_7828;
    pub const FE_STRING_HASH: u32 = 0x0031_7670;
    pub const UI_SCROLL_SET_PARTS_VISIBLE: u32 = 0x0039_de68;
    pub const UI_ADD_SCREEN_BY_HASH_NAME: u32 = 0x0039_7cc0;
    pub const UI_SCREEN_PLAY_FRAME: u32 = 0x0039_c870;
    pub const UI_SCREEN_GET_OBJECT_BY_HASH: u32 = 0x0039_d860;
    pub const LOBBY_API_DESTROY_LIST: u32 = 0x003e_8688;
    pub const LOBBY_API_FLUSH_ROSTER: u32 = 0x003f_0be0;
    pub const BUDDY_API_DESTROY: u32 = 0x003f_07a0;
    pub const TOOLTIP_MANAGER_HIDE_POPUP: u32 = 0x001d_a510;
    pub const TOOLTIP_MANAGER_SET_STATE: u32 = 0x001d_a528;
    pub const TOOLTIP_MANAGER_SET_TEXT: u32 = 0x001d_a530;
    pub const TUBE_MENU_ITEM_DTOR: u32 = 0x002c_a280;
    pub const TUBE_MENU_BASE_DTOR: u32 = 0x002c_aa80;
    pub const OVERLAY_RESET: u32 = 0x002e_a820;
    pub const BE_SET_PHASE: u32 = 0x0014_de68;
    pub const GAME_MODULE_SET_STATE: u32 = 0x0020_a8f8;
    pub const WORLD_SOUND_GET_TRIGGER_IDS: Callee = Callee {
        addr: 0x002b_8190,
        f: ee::s08::cWorldSound_GetTriggerIds,
    };
    pub const AUDIO_GET_GAME_MODULE: u32 = 0x0028_b1c8;
    pub const MC_GET_WORLD: u32 = 0x0022_7f80;
    pub const MC_MANAGER_GET: u32 = 0x0023_fb58;
    pub const OVERLAY_UPDATE_CALLEE: Callee = Callee {
        addr: 0x0020_e900,
        f: ee::s05::f_0020e900,
    };
    pub const SND_BANK_LOAD: u32 = 0x0028_bd10;
    pub const AVALANCHE_RESET_ONE: u32 = 0x0037_15b0;
    pub const REPLAY_GET_WORLD: u32 = 0x0012_baf0;
    pub const GAME_STATE_IS_LOADING: u32 = 0x0027_8da0;
    pub const WORLD_STATE_FINALIZE: u32 = 0x0035_4c98;
    pub const UI_SCREEN_LOOKUP_VALUE: u32 = 0x0039_c778;
    pub const UI_SCREEN_LAUNCH_THREAD: u32 = 0x0039_f190;
    pub const PATH_PLAT_RESUME: u32 = 0x003b_58a0;
    pub const PATH_PLAT_SUSPEND: u32 = 0x003b_58d8;
    pub const PATH_PLAT_SET_RUNNING: u32 = 0x003b_7fb8;
    pub const PATH_PLAT_STOP: u32 = 0x003b_8790;
    pub const OVERLAY_UPDATE: u32 = 0x0020_e900;
    pub const UI_SCREEN_SET_OBJECT_VALUE: u32 = 0x0039_83f0;
}

// ---------------------------------------------------------------------------
// Small helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// The low word of a GPR.
#[inline(always)]
fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// `sll x, n` (32-bit, sign-extended).
#[inline(always)]
fn shl32(x: u64, n: u32) -> u64 {
    sx((x as u32) << n)
}

/// `sra x, n`.
#[inline(always)]
fn sra32(x: u64, n: u32) -> u64 {
    sx(((x as u32) as i32 >> n) as u32)
}

/// `addu x, y` (32-bit, sign-extended).
#[inline(always)]
fn add32(x: u64, y: u64) -> u64 {
    sx((x as u32).wrapping_add(y as u32))
}

// ---------------------------------------------------------------------------
// Input map file

/// `InputMap_FinalizeOpcodes` (0x320f10): replace the map's binding array:
/// delete the old list, allocate a copy of the new one, copy it in and free
/// the temporary. `this+8` is the old array, `+0x10` the new array (word
/// count), `+0x14` the source list to copy from.
pub fn input_map_finalize_opcodes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);

        let old = Addr(p(this, 8)).lw(c);
        if old == 0 {
            c.g[A0][0] = Addr(p(this, 0x10)).lw(c);
        } else {
            c.cycles += 3;
            c.g[A0][0] = old;
            c.g[RA][0] = sx(0x0032_0f38);
            callee::BUILTIN_DELETE.call(c, 0x0032_0f38)?;
            c.g[A0][0] = Addr(p(this, 0x10)).lw(c);
        }

        // The new array: `count * 4` bytes from the named pool.
        c.cycles += 16;
        c.g[A1][0] = 0x0049_0000;
        Addr(p(this, 8)).sw(c, 0);
        c.g[A2][0] = 0;
        c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(0x22d0));
        c.g[A0][0] = shl32(c.g[A0][0], 2);
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x0032_0f58);
        callee::OPERATOR_NEW_NAMED.call(c, 0x0032_0f58)?;
        c.g[A2][0] = Addr(p(this, 0x10)).lw(c);
        let array = c.g[V0][0];
        c.g[S1][0] = array;
        c.g[A1][0] = Addr(p(this, 0x14)).lw(c);
        c.g[A0][0] = array;
        c.g[RA][0] = sx(0x0032_0f70);
        c.g[A2][0] = shl32(c.g[A2][0], 2);
        call_ptr(c, callee::MEMCPY, 0x0032_0f70)?;
        c.g[A0][0] = Addr(p(this, 0x14)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0032_0f84);
            callee::EA_MEM_FREE.call(c, 0x0032_0f84)?;
        }
        c.g[V0][0] = Addr(p(this, 0x10)).lw(c);
        c.cycles += 7;
        Addr(p(this, 0x14)).sw(c, array);
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `TransitionWipe_ctor` (0x2ebba8): construct the 16-byte parameter struct
/// passed by value (`a1`) into TransitionEffect's constructor, then install
/// the wipe vtable and store the two trailing words.
pub fn transition_wipe_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        let arg = r(c, A1);
        c.ldl(2, p(arg, 7));
        c.ldr(2, arg);
        c.ldl(3, p(arg, 0xf));
        c.ldr(3, p(arg, 8));
        c.sdl(2, p(r(c, SP), 7));
        c.sdr(2, r(c, SP));
        c.sdl(3, p(r(c, SP), 0xf));
        c.g[RA][0] = sx(0x002e_bbec);
        c.sdr(3, p(r(c, SP), 8));
        callee::TRANSITION_EFFECT_CTOR.call(c, 0x002e_bbec)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[V0][0] = sx(this);
        c.g[V1][0] = sx(0x0049_0000u32.wrapping_sub(0x7f60));
        frame.restore_ra(c, 0x10);
        Addr(this).sw(c, c.g[V1][0]);
        let sp = r(c, SP);
        c.ldl(3, p(sp, 7));
        c.ldr(3, sp);
        c.ldl(4, p(sp, 0xf));
        c.ldr(4, p(sp, 8));
        c.sdl(3, p(this, 0xb));
        c.sdr(3, p(this, 4));
        c.sdl(4, p(this, 0x13));
        c.sdr(4, p(this, 0xc));
        Addr(p(this, 0x14)).sw(c, c.g[S1][0]);
        Addr(p(this, 0x18)).sw(c, c.g[S2][0]);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.close(c);
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

/// `GameViewMan_dtor` (0x22e730): destroy every view in the manager's list
/// through its virtual destructor and optionally free the manager.
pub fn game_view_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        let this = r(c, A0);
        c.g[S2][0] = sx(this);
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(this).lw(c);
        c.g[S3][0] = c.g[A1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[S0][0] = sx(this.wrapping_add(4));
            loop {
                c.cycles += 3;
                let entry = Addr(r(c, S0)).lw(c);
                c.g[A2][0] = entry;
                c.g[A1][0] = 3;
                if entry != 0 {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p(entry as u32, 0x90)).lw(c);
                    c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
                    c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
                    let target = r(c, V1);
                    c.g[RA][0] = sx(0x0022_e780);
                    c.g[A0][0] = sx((entry as u32).wrapping_add(r(c, A0)));
                    call_ptr(c, target, 0x0022_e780)?;
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(r(c, S2)).lw(c);
                c.g[S1][0] = sx(r(c, S1).wrapping_add(1));
                c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(r(c, S0).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0022_e760);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S3][0] & 1;
        let free_self = c.g[V0][0] != 0;
        Addr(r(c, S2)).off(0x10).sw(c, 0);
        if free_self {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0022_e7a8);
            c.g[A0][0] = c.g[S2][0];
            callee::BUILTIN_DELETE.call(c, 0x0022_e7a8)?;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetMessenger_ShowLookingForSelf` (0x1a4d90): open the
/// "looking for yourself" popup, configure its two option labels and show it
/// with a fixed 0x808 option mask.
pub fn fe_net_messenger_show_looking_for_self(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        let this = r(c, A0);
        c.g[S3][0] = sx(this);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);

        c.g[RA][0] = sx(0x001a_4db4);
        c.g[S2][0] = 3;
        callee::NET_FE_STATE_GET_POPUP.call(c, 0x001a_4db4)?;
        let popup = r(c, V0);
        c.g[S0][0] = sx(popup);
        c.g[A0][0] = 0x0046_0000;
        c.g[V0][0] = 1;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x1660));
        Addr(p(popup, 0xbc)).sw(c, c.g[V0][0]);
        c.g[RA][0] = sx(0x001a_4dd0);
        c.g[S1][0] = sx(popup.wrapping_add(0xbc));
        call_ptr(c, callee::FE_STRING_HASH, 0x001a_4dd0)?;
        c.g[S2][0] = 3;
        Addr(p(popup, 0xc8)).sw(c, c.g[S2][0]);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x1618));
        c.g[RA][0] = sx(0x001a_4de4);
        Addr(p(popup, 0xcc)).sw(c, c.g[V0][0]);
        call_ptr(c, callee::FE_STRING_HASH, 0x001a_4de4)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001a_4df4);
        c.g[A1][0] = 0;
        callee::POPUP_CFG_SET_OPTION_TEXT_ID.call(c, 0x001a_4df4)?;
        Addr(p(popup, 0x14c)).sw(c, c.g[S2][0]);
        c.g[V0][0] = 0x808;
        Addr(p(popup, 0x150)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x001a_4e0c);
        c.g[A1][0] = c.g[S0][0];
        callee::NET_FE_STATE_SHOW_POPUP.call(c, 0x001a_4e0c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetMessenger_HasName` (0x1a56d0): true when the name string is
/// one of the entries in the vector at `+0x6e8` (case-insensitive).
pub fn fe_net_messenger_has_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(this, 0x6ec)).lw(c);
        c.g[V1][0] = Addr(p(this, 0x6e8)).lw(c);
        c.g[V0][0] = sx(r(c, V0).wrapping_sub(r(c, V1)));
        c.g[V0][0] = sra32(c.g[V0][0], 2);
        let empty = c.g[V0][0] == 0;
        let mut found = false;
        if !empty {
            c.cycles += 1;
            c.g[V0][0] = Addr(p(this, 0x6e8)).lw(c);
            loop {
                c.cycles += 8;
                c.g[A0][0] = shl32(c.g[S0][0], 2);
                c.g[A1][0] = c.g[S2][0];
                c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
                c.g[V1][0] = Addr(r(c, V0)).lw(c);
                c.g[RA][0] = sx(0x001a_5720);
                c.g[A0][0] = Addr(r(c, V1)).lw(c);
                call_ptr(c, callee::STRCASECMP, 0x001a_5720)?;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(p(this, 0x6ec)).lw(c);
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    found = true;
                    break;
                }
                c.cycles += 7;
                c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                c.g[V1][0] = Addr(p(this, 0x6e8)).lw(c);
                c.g[V0][0] = sx(r(c, V0).wrapping_sub(r(c, V1)));
                c.g[V0][0] = sra32(c.g[V0][0], 2);
                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(p(this, 0x6e8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x001a_5708);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetMessenger_FindNameStr` (0x1a5800): index of the name string in
/// the vector at `+0x6e8` (case-insensitive), or -1.
pub fn fe_net_messenger_find_name_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(this, 0x6ec)).lw(c);
        c.g[V1][0] = Addr(p(this, 0x6e8)).lw(c);
        c.g[V0][0] = sx(r(c, V0).wrapping_sub(r(c, V1)));
        c.g[V0][0] = sra32(c.g[V0][0], 2);
        let empty = c.g[V0][0] == 0;
        let mut found = false;
        if !empty {
            c.cycles += 1;
            c.g[V0][0] = Addr(p(this, 0x6e8)).lw(c);
            loop {
                c.cycles += 8;
                c.g[A0][0] = shl32(c.g[S0][0], 2);
                c.g[A1][0] = Addr(r(c, S2)).lw(c);
                c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
                c.g[V1][0] = Addr(r(c, V0)).lw(c);
                c.g[RA][0] = sx(0x001a_5850);
                c.g[A0][0] = Addr(r(c, V1)).lw(c);
                call_ptr(c, callee::STRCASECMP, 0x001a_5850)?;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(p(this, 0x6ec)).lw(c);
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    found = true;
                    break;
                }
                c.cycles += 7;
                c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                c.g[V1][0] = Addr(p(this, 0x6e8)).lw(c);
                c.g[V0][0] = sx(r(c, V0).wrapping_sub(r(c, V1)));
                c.g[V0][0] = sra32(c.g[V0][0], 2);
                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(p(this, 0x6e8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x001a_5838);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `NetFEState_OnKeyboardPopupEvent` (0x1a94e0): keyboard-popup child events
/// 0x15/0x16 dispatch to the text field's virtual handler (0x16 also clears
/// the pending popup flag), everything else to the default child handler.
pub fn net_fe_state_on_keyboard_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x15;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        if c.g[A2][0] == c.g[V0][0] {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001a_9524);
            c.g[A2][0] = 0x15;
            callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT.call(c, 0x001a_9524)?;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x16;
            let is16 = c.g[A2][0] == c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            if is16 {
                c.cycles += 12;
                c.g[V0][0] = Addr(p(this, 8)).lw(c);
                c.g[A0][0] = Addr(p(r(c, V0), 0x100)).lh(c);
                c.g[V1][0] = Addr(p(r(c, V0), 0x104)).lw(c);
                let target = r(c, V1);
                c.g[RA][0] = sx(0x001a_9540);
                c.g[A0][0] = sx(this.wrapping_add(r(c, A0)));
                call_ptr(c, target, 0x001a_9540)?;
                Addr(p(this, 0x69c)).sw(c, 0);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x001a_9554);
                c.g[A2][0] = 0x16;
                callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT.call(c, 0x001a_9554)?;
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 2;
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a_9564);
                c.g[A1][0] = c.g[S1][0];
                callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT.call(c, 0x001a_9564)?;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateChatRoom_TransitionIn` (0x1b3490): hide the chat room's scroll
/// parts and run each player list entry's virtual "transition in" handler,
/// then select the local player.
pub fn fe_state_chat_room_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save(c, S2, 0x10);
        let has_arg = c.g[A1][0] != 0;
        frame.save_ra(c, 0);
        if has_arg {
            c.cycles += 9;
            c.g[A0][0] = Addr(p(this, 0x6ec)).lw(c);
            c.g[A1][0] = 0;
            c.g[S0][0] = sx(this.wrapping_add(0x6fc));
            c.g[RA][0] = sx(0x001b_34c0);
            c.g[S2][0] = 1;
            call_ptr(c, callee::UI_SCROLL_SET_PARTS_VISIBLE, 0x001b_34c0)?;
            c.g[A0][0] = Addr(p(this, 0x6f4)).lw(c);
            c.g[RA][0] = sx(0x001b_34cc);
            c.g[A1][0] = 0;
            call_ptr(c, callee::UI_SCROLL_SET_PARTS_VISIBLE, 0x001b_34cc)?;
            loop {
                c.cycles += 3;
                let entry = Addr(r(c, S0)).lw(c);
                c.g[A2][0] = entry;
                c.g[A1][0] = 1;
                if entry != 0 {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p(entry as u32, 8)).lw(c);
                    c.g[A0][0] = Addr(p(r(c, V0), 0x38)).lh(c);
                    c.g[V1][0] = Addr(p(r(c, V0), 0x3c)).lw(c);
                    let target = r(c, V1);
                    c.g[RA][0] = sx(0x001b_34f0);
                    c.g[A0][0] = sx((entry as u32).wrapping_add(r(c, A0)));
                    call_ptr(c, target, 0x001b_34f0)?;
                }
                c.cycles += 3;
                c.g[S2][0] = sx(r(c, S2).wrapping_sub(1));
                let more = (c.g[S2][0] as i64) >= 0;
                c.g[S0][0] = sx(r(c, S0).wrapping_add(4));
                if !more {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(gp(c, -0xc8)).lw(c);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001b_350c);
                    c.g[A1][0] = Addr(p(r(c, V0), 0x80)).lw(c);
                    callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME.call(c, 0x001b_350c)?;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001b_34d0);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateOnlineAgreement_Init` (0x1c04a8): add the agreement screen by its
/// hash name, start its thread and mark the terms text object dirty.
pub fn fe_state_online_agreement_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x5568));
        c.g[RA][0] = sx(0x001c_04cc);
        c.g[S0][0] = Addr(p(this, 0x10)).lw(c);
        call_ptr(c, callee::FE_STRING_HASH, 0x001c_04cc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001c_04e0);
        c.g[A3][0] = 0;
        call_ptr(c, callee::UI_ADD_SCREEN_BY_HASH_NAME, 0x001c_04e0)?;
        let screen = r(c, V0);
        let none = c.g[V0][0] == 0;
        Addr(p(this, 0x40)).sw(c, c.g[V0][0]);
        if !none {
            c.cycles += 6;
            c.g[A0][0] = sx(screen);
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001c_04f8);
            c.g[A2][0] = 0;
            call_ptr(c, callee::UI_SCREEN_PLAY_FRAME, 0x001c_04f8)?;
            c.g[RA][0] = sx(0x001c_0500);
            c.g[A0][0] = c.g[S1][0];
            callee::LOAD_AGREEMENT_TERMS.call(c, 0x001c_0500)?;
        }
        c.cycles += 9;
        c.g[A0][0] = 0x0046_0000;
        c.g[RA][0] = sx(0x001c_050c);
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x5580));
        call_ptr(c, callee::FE_STRING_HASH, 0x001c_050c)?;
        c.g[A0][0] = Addr(p(this, 0x40)).lw(c);
        c.g[RA][0] = sx(0x001c_0518);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_GET_OBJECT_BY_HASH, 0x001c_0518)?;
        let obj = r(c, V0);
        c.g[V1][0] = sx(obj);
        let missing = c.g[V1][0] == 0;
        frame.restore(c, S0, 0x20);
        if !missing {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(obj, 0x90)).lw(c);
            c.g[V0][0] = c.g[V0][0] | 8;
            Addr(p(obj, 0x90)).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateOnlineAgreement_v25` (0x1c0ea8): populate the agreement dialog's
/// terms text and the two option rows.
pub fn fe_state_online_agreement_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 2;
        c.g[RA][0] = sx(0x001c_0ec4);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::TOOLTIP_MANAGER_SET_STATE, 0x001c_0ec4)?;
        c.g[A0][0] = 0x0046_0000;
        c.g[RA][0] = sx(0x001c_0ed0);
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x5778));
        call_ptr(c, callee::FE_STRING_HASH, 0x001c_0ed0)?;
        c.g[A0][0] = 0x0046_0000;
        Addr(p(r(c, S0), 0x54)).sw(c, c.g[V0][0]);
        c.g[RA][0] = sx(0x001c_0ee0);
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x5798));
        call_ptr(c, callee::FE_STRING_HASH, 0x001c_0ee0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001c_0ef0);
        c.g[A1][0] = 0;
        call_ptr(c, callee::TOOLTIP_MANAGER_SET_TEXT, 0x001c_0ef0)?;
        c.g[A0][0] = 0x0046_0000;
        c.g[RA][0] = sx(0x001c_0efc);
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x57c0));
        call_ptr(c, callee::FE_STRING_HASH, 0x001c_0efc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x001c_0f10);
        c.g[A3][0] = 8;
        call_ptr(c, callee::TOOLTIP_MANAGER_HIDE_POPUP, 0x001c_0f10)?;
        c.g[A0][0] = 0x0046_0000;
        c.g[RA][0] = sx(0x001c_0f1c);
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x14e8));
        call_ptr(c, callee::FE_STRING_HASH, 0x001c_0f1c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        c.g[RA][0] = sx(0x001c_0f30);
        c.g[A3][0] = 1;
        call_ptr(c, callee::TOOLTIP_MANAGER_HIDE_POPUP, 0x001c_0f30)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMan_DestroyLists` (0x25a8b0): destroy the lobby's three API lists
/// through the LobbyApi list destructor, then release the snapshot lists and
/// any pending list object.
pub fn lobby_man_destroy_lists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        c.g[A2][0] = Addr(p(this, 0x5c)).lw(c);
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = Addr(p(this, 0x54)).lw(c);
            c.g[RA][0] = sx(0x0025_a8d8);
            c.g[A1][0] = 0;
            call_ptr(c, callee::LOBBY_API_DESTROY_LIST, 0x0025_a8d8)?;
            Addr(p(this, 0x5c)).sw(c, 0);
        }
        c.g[A2][0] = Addr(p(this, 0x60)).lw(c);
        c.cycles += 2;
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = Addr(p(this, 0x54)).lw(c);
            c.g[RA][0] = sx(0x0025_a8f4);
            c.g[A1][0] = 1;
            call_ptr(c, callee::LOBBY_API_DESTROY_LIST, 0x0025_a8f4)?;
            Addr(p(this, 0x60)).sw(c, 0);
        }
        c.g[A2][0] = Addr(p(this, 0x64)).lw(c);
        c.cycles += 2;
        if c.g[A2][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = Addr(p(this, 0x54)).lw(c);
            c.g[RA][0] = sx(0x0025_a910);
            c.g[A1][0] = 2;
            call_ptr(c, callee::LOBBY_API_DESTROY_LIST, 0x0025_a910)?;
            Addr(p(this, 0x64)).sw(c, 0);
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x0025_a91c);
        c.g[A0][0] = c.g[S0][0];
        callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS.call(c, 0x0025_a91c)?;
        c.g[A0][0] = Addr(p(this, 0x58)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r(c, A0), 0xc)).lw(c);
            let target = r(c, V0);
            c.g[RA][0] = sx(0x0025_a934);
            call_ptr(c, target, 0x0025_a934)?;
            Addr(p(this, 0x58)).sw(c, 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BuddyMan_DestroyApi` (0x261530): send the outstanding declines, destroy
/// the API object and clear the buddy manager's fields. A dispatcher-shaped
/// loop: it resumes at every block the original can unwind at.
pub fn buddy_man_destroy_api(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0026_1530 => {
                    c.cycles += 8;
                    let frame = Frame::open(c, 0x30);
                    frame.save(c, S1, 0x10);
                    frame.save(c, S0, 0x20);
                    let this = r(c, A0);
                    c.g[S1][0] = sx(this);
                    frame.save_ra(c, 0);
                    c.g[V0][0] = Addr(p(this, 0xc8)).lw(c);
                    let none = c.g[V0][0] == 0;
                    // The original restores s0 here; the save/restore is a no-op.
                    if none {
                        pc = 0x0026_15b8;
                    } else {
                        pc = 0x0026_1550;
                    }
                }
                0x0026_1550 => {
                    c.cycles += 2;
                    c.g[V1][0] = Addr(p(r(c, S1), 0xbc)).lw(c);
                    pc = 0x0026_1590;
                }
                0x0026_1558 => {
                    c.cycles += 7;
                    let v0 = r(c, V0);
                    c.g[S0][0] = Addr(v0.wrapping_sub(4)).lw(c);
                    c.g[RA][0] = sx(0x0026_1564);
                    c.g[A1][0] = Addr(r(c, S0)).lw(c);
                    callee::LOBBY_MAN_SEND_DECLINE.call(c, 0x0026_1564)?;
                    c.g[V0][0] = Addr(p(r(c, S1), 0xc0)).lw(c);
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(4));
                    let no_next = c.g[S0][0] == 0;
                    Addr(p(r(c, S1), 0xc0)).sw(c, c.g[V0][0]);
                    if no_next {
                        pc = 0x0026_158c;
                    } else {
                        pc = 0x0026_1574;
                    }
                }
                0x0026_1574 => {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(p(r(c, S0), 0x3c)).lw(c);
                    c.g[A1][0] = 3;
                    c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
                    c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
                    let target = r(c, V1);
                    c.g[RA][0] = sx(0x0026_158c);
                    c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                    call_ptr(c, target, 0x0026_158c)?;
                    pc = 0x0026_158c;
                }
                0x0026_158c => {
                    c.cycles += 1;
                    c.g[V1][0] = Addr(p(r(c, S1), 0xbc)).lw(c);
                    pc = 0x0026_1590;
                }
                0x0026_1590 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(r(c, S1), 0xc0)).lw(c);
                    let more = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
                    if more {
                        if back_edge_unwinds(c, 0x0026_1558) {
                            return Ok(());
                        }
                        pc = 0x0026_1558;
                    } else {
                        pc = 0x0026_159c;
                    }
                }
                0x0026_159c => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x0026_15a4);
                    c.g[A0][0] = Addr(p(r(c, S1), 0xc8)).lw(c);
                    call_ptr(c, callee::LOBBY_API_FLUSH_ROSTER, 0x0026_15a4)?;
                    c.g[RA][0] = sx(0x0026_15ac);
                    c.g[A0][0] = Addr(p(r(c, S1), 0xc8)).lw(c);
                    call_ptr(c, callee::BUDDY_API_DESTROY, 0x0026_15ac)?;
                    Addr(p(r(c, S1), 0xcc)).sw(c, 0);
                    Addr(p(r(c, S1), 0xc8)).sw(c, 0);
                    c.lq(16, p(r(c, SP), 0x20));
                    pc = 0x0026_15b8;
                }
                0x0026_15b8 => {
                    c.cycles += 4;
                    let sp = r(c, SP);
                    c.lq(17, p(sp, 0x10));
                    c.g[RA][0] = c.ld(sp);
                    let ra = r(c, RA);
                    c.g[SP][0] = sx(sp.wrapping_add(0x30));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0026_1530, 0x0026_15c8);
                    return Ok(());
                }
            }
        }
    })
}

/// `BuddyMan_GetChallengerByName` (0x263270): linear search of the buddy
/// manager's list for the challenger with the given name. Dispatcher-shaped
/// like the original.
pub fn buddy_man_get_challenger_by_name(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0026_3270 => {
                    c.cycles += 13;
                    let frame = Frame::open(c, 0x50);
                    frame.save(c, S2, 0x20);
                    frame.save(c, S3, 0x10);
                    let this = r(c, A0);
                    c.g[S2][0] = sx(this);
                    frame.save(c, S0, 0x40);
                    frame.save(c, S1, 0x30);
                    frame.save_ra(c, 0);
                    c.g[A0][0] = Addr(p(this, 0xc0)).lw(c);
                    c.g[V1][0] = Addr(p(this, 0xbc)).lw(c);
                    c.g[V0][0] = sx(r(c, A0).wrapping_sub(r(c, V1)));
                    c.g[V0][0] = sra32(c.g[V0][0], 2);
                    let empty = c.g[V0][0] == 0;
                    c.g[S3][0] = c.g[A1][0];
                    if empty {
                        pc = 0x0026_32a4;
                    } else {
                        pc = 0x0026_32b4;
                    }
                }
                0x0026_32a4 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    pc = 0x0026_32ec;
                }
                0x0026_32ac => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S1][0];
                    pc = 0x0026_32ec;
                }
                0x0026_32b4 => {
                    c.cycles += 3;
                    c.g[S0][0] = c.g[V1][0];
                    let end = c.g[S0][0] == c.g[A0][0];
                    c.g[V0][0] = 0;
                    if end {
                        pc = 0x0026_32ec;
                    } else {
                        pc = 0x0026_32c0;
                    }
                }
                0x0026_32c0 => {
                    c.cycles += 2;
                    c.g[S1][0] = Addr(r(c, S0)).lw(c);
                    pc = 0x0026_32c8;
                }
                0x0026_32c8 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x0026_32d4);
                    c.g[A0][0] = Addr(r(c, S1)).lw(c);
                    call_ptr(c, callee::STRCASECMP, 0x0026_32d4)?;
                    let differ = c.g[V0][0] == 0;
                    c.g[S0][0] = sx(r(c, S0).wrapping_add(4));
                    if differ {
                        if back_edge_unwinds(c, 0x0026_32ac) {
                            return Ok(());
                        }
                        pc = 0x0026_32ac;
                    } else {
                        pc = 0x0026_32dc;
                    }
                }
                0x0026_32dc => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(r(c, S2), 0xc0)).lw(c);
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[S1][0] = Addr(r(c, S0)).lw(c);
                        if back_edge_unwinds(c, 0x0026_32c8) {
                            return Ok(());
                        }
                        pc = 0x0026_32c8;
                    } else {
                        pc = 0x0026_32e8;
                    }
                }
                0x0026_32e8 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    pc = 0x0026_32ec;
                }
                0x0026_32ec => {
                    c.cycles += 7;
                    let sp = r(c, SP);
                    c.lq(16, p(sp, 0x40));
                    c.lq(17, p(sp, 0x30));
                    c.lq(18, p(sp, 0x20));
                    c.lq(19, p(sp, 0x10));
                    c.g[RA][0] = c.ld(sp);
                    let ra = r(c, RA);
                    c.g[SP][0] = sx(sp.wrapping_add(0x50));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0026_3270, 0x0026_3308);
                    return Ok(());
                }
            }
        }
    })
}

/// `PATH_PlatSetPaused` (0x2af7a0): pause/resume a platform path (index in
/// `a0`) and remember the old flag. `a1` selects pause (stash the current
/// flag and set the pause byte) or resume.
pub fn path_plat_set_paused(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x004d_0000;
        frame.save(c, S1, 0x10);
        c.g[A0][0] = shl32(c.g[A0][0], 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x004d_0000u32.wrapping_add(0x3e98));
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V0)));
        let pause = c.g[A1][0] != 0;
        c.g[S0][0] = Addr(r(c, A0)).lw(c);
        if pause {
            c.cycles += 4;
            c.g[V0][0] = Addr(p(r(c, S0), 0x3c)).lhu(c);
            c.g[S1][0] = 0;
            Addr(p(r(c, S0), 0x18)).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 1;
            c.g[S1][0] = Addr(p(r(c, S0), 0x18)).lhu(c);
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x002a_f7e4);
        call_ptr(c, callee::PATH_PLAT_RESUME, 0x002a_f7e4)?;
        c.g[V0][0] = Addr(p(r(c, S0), 0x64)).lb(c);
        let active = c.g[V0][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if active {
            c.cycles += 3;
            c.g[A0][0] = Addr(r(c, S0)).lw(c);
            c.g[RA][0] = sx(0x002a_f810);
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, callee::PATH_PLAT_SET_RUNNING, 0x002a_f810)?;
        } else {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r(c, S0), 0x6c)).lw(c);
            c.g[RA][0] = sx(0x002a_f7fc);
            c.g[A0][0] = Addr(p(r(c, V0), 4)).lw(c);
            call_ptr(c, callee::PATH_PLAT_STOP, 0x002a_f7fc)?;
        }
        c.cycles += 10;
        c.g[RA][0] = sx(0x002a_f818);
        call_ptr(c, callee::PATH_PLAT_SUSPEND, 0x002a_f818)?;
        c.g[V1][0] = 1;
        frame.restore(c, S1, 0x10);
        Addr(p(r(c, S0), 0x65)).sb(c, c.g[V1][0]);
        c.g[V0][0] = 0;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MusicPlayer_LoadLoopBank` (0x2b43d8): pick a random loop of the current
/// song, build its bank name and queue the bank load asynchronously.
pub fn music_player_load_loop_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0x120);
        frame.save(c, S1, 0x110);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0x100);
        let song = Addr(p(this, 0x408)).lw(c);
        c.g[A0][0] = song;
        if song != 0 {
            c.cycles += 4;
            c.g[V1][0] = Addr(r(c, A0)).lw(c);
            c.g[V0][0] = Addr(p(r(c, V1), 0x40)).lw(c);
            let no_loop = c.g[V0][0] == 0;
            c.g[S1][0] = Addr(p(r(c, A0), 0x34)).lw(c);
            if !no_loop {
                c.cycles += 21;
                c.g[RA][0] = sx(0x002b_4410);
                callee::SONG_INSTANCE_PICK_RANDOM_LOOP.call(c, 0x002b_4410)?;
                c.g[A2][0] = Addr(p(r(c, S0), 0x408)).lw(c);
                c.g[V0][0] = shl32(c.g[V0][0], 5);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(0x44));
                c.g[A1][0] = 0x004a_0000;
                c.g[A3][0] = Addr(r(c, A2)).lw(c);
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx(0x004a_0000u32.wrapping_add(0x36e0));
                c.g[A2][0] = sx(r(c, A2).wrapping_add(0x60));
                c.g[RA][0] = sx(0x002b_4438);
                c.g[A3][0] = sx(r(c, A3).wrapping_add(r(c, V0)));
                call_ptr(c, callee::SPRINTF, 0x002b_4438)?;
                c.g[V1][0] = Addr(r(c, S0)).lw(c);
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[SP][0];
                c.g[A3][0] = 0x100;
                c.g[RA][0] = sx(0x002b_4450);
                c.g[A0][0] = Addr(r(c, V1)).lw(c);
                call_ptr(c, callee::SND_BANK_LOAD, 0x002b_4450)?;
                c.g[V1][0] = Addr(p(r(c, S0), 0x408)).lw(c);
                c.g[V0][0] = 1;
                Addr(p(r(c, V1), 0x38)).sw(c, c.g[V0][0]);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x120);
        } else {
            frame.restore(c, S0, 0x120);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `Replay_FlushSaveOutput` (0x270fc0): append the pending replay snapshot
/// bytes to the file buffer (at most 0x4000 per call) and advance the source
/// and count fields.
pub fn replay_flush_save_output(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(p(this, 0x644)).lw(c);
        if c.g[A1][0] != 0 {
            c.cycles += 3;
            c.g[A2][0] = Addr(p(this, 0x648)).lw(c);
            let empty = (c.g[A2][0] as i64) <= 0;
            c.g[V1][0] = 0x4000;
            if empty {
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 7;
                c.g[V0][0] = Addr(p(this, 0x64c)).lw(c);
                c.g[V1][0] = sx(0x4000u32.wrapping_sub(r(c, V0)));
                c.g[A0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                c.g[S1][0] = c.g[V1][0];
                if c.g[A0][0] == 0 {
                    c.g[S1][0] = c.g[A2][0];
                }
                if (c.g[S1][0] as i64) <= 0 {
                    frame.restore(c, S0, 0x20);
                } else {
                    c.cycles += 15;
                    c.g[A0][0] = Addr(p(this, 0x650)).lw(c);
                    c.g[RA][0] = sx(0x0027_1014);
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, callee::MEMCPY, 0x0027_1014)?;
                    c.g[V1][0] = Addr(p(this, 0x650)).lw(c);
                    c.g[A0][0] = Addr(p(this, 0x644)).lw(c);
                    c.g[A1][0] = Addr(p(this, 0x648)).lw(c);
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, S1)));
                    c.g[V0][0] = Addr(p(this, 0x64c)).lw(c);
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, S1)));
                    c.g[A1][0] = sx(r(c, A1).wrapping_sub(r(c, S1)));
                    Addr(p(this, 0x650)).sw(c, c.g[V1][0]);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S1)));
                    Addr(p(this, 0x644)).sw(c, c.g[A0][0]);
                    Addr(p(this, 0x64c)).sw(c, c.g[V0][0]);
                    Addr(p(this, 0x648)).sw(c, c.g[A1][0]);
                    c.cycles += 1;
                    frame.restore(c, S0, 0x20);
                }
            }
        } else {
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Replay_SaveBuildHeader` (0x271228): write the replay header (magic, build
/// id, world/rider fields) into the save buffer and reset the write cursor.
pub fn replay_save_build_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save(c, S0, 0x20);
        c.g[RA][0] = sx(0x0027_1244);
        c.g[S0][0] = Addr(p(this, 0x640)).lw(c);
        callee::REPLAY_COUNT_SAVE_SNAPSHOTS.call(c, 0x0027_1244)?;
        let buf = r(c, S0);
        Addr(p(buf, 4)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 0xffff_ffff_abcd_0000;
        c.g[A3][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[A2][0] = c.g[A2][0] | 2;
        c.g[V1][0] = Addr(p(this, 0x18)).lw(c);
        c.g[A1][0] = sx(buf.wrapping_add(0x20));
        Addr(p(buf, 8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(p(this, 0x610)).lw(c);
        Addr(p(buf, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(p(this, 0x3c8)).lw(c);
        Addr(p(buf, 0x10)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(p(this, 0x3cc)).lw(c);
        Addr(p(buf, 0x14)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(p(r(c, A3), 0x84)).lw(c);
        c.g[V0][0] = Addr(p(r(c, V1), 0xc)).lw(c);
        c.g[A0][0] = Addr(p(r(c, V0), 0x7c)).lw(c);
        Addr(buf).sw(c, c.g[A2][0]);
        Addr(p(buf, 0x18)).sw(c, c.g[A0][0]);
        c.g[V0][0] = Addr(p(r(c, A3), 0x84)).lw(c);
        c.g[RA][0] = sx(0x0027_1298);
        c.g[A0][0] = Addr(p(r(c, V0), 0xc)).lw(c);
        call_ptr(c, callee::REPLAY_GET_WORLD, 0x0027_1298)?;
        c.g[V0][0] = Addr(p(r(c, S1), 0x638)).lw(c);
        Addr(p(r(c, S1), 0x644)).sw(c, 0);
        c.g[V0][0] = sx(r(c, V0).wrapping_add(1));
        Addr(p(r(c, S1), 0x648)).sw(c, 0);
        Addr(p(r(c, S1), 0x638)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `InSeq_InstanceGetLength` (0x274c70): total length of the instance's
/// channels (sum over the channel records), cached at `+0x30`.
pub fn in_seq_instance_get_length(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(this, 0x30)).lw(c);
        if (c.g[V0][0] as i64) < 0 {
            c.cycles += 5;
            c.g[V1][0] = Addr(p(this, 0x10)).lw(c);
            c.g[S3][0] = 0;
            c.g[V0][0] = Addr(p(r(c, V1), 1)).lbu(c);
            let no_channels = c.g[V0][0] == 0;
            c.g[S0][0] = 0;
            if !no_channels {
                c.cycles += 1;
                c.g[S2][0] = 0;
                loop {
                    c.cycles += 11;
                    c.g[A0][0] = Addr(p(this, 0x18)).lw(c);
                    c.g[S3][0] = sx(r(c, S3).wrapping_add(1));
                    c.g[RA][0] = sx(0x0027_4cc0);
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, S2)));
                    callee::IN_SEQ_CHANNEL_GET_LENGTH.call(c, 0x0027_4cc0)?;
                    c.g[S2][0] = sx(r(c, S2).wrapping_add(0x3c));
                    c.g[A1][0] = Addr(p(this, 0x10)).lw(c);
                    c.g[A0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[V1][0] = Addr(p(r(c, A1), 1)).lbu(c);
                    c.g[V1][0] = ((c.g[S3][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let more = c.g[V1][0] != 0;
                    if c.g[A0][0] != 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0027_4cb0);
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p(this, 0x20)).lw(c);
            if c.g[V0][0] != 0 {
                Addr(p(this, 0x30)).sw(c, c.g[S0][0]);
            }
            c.cycles += 2;
            c.g[V0][0] = c.g[S0][0];
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

/// `ChaseCamController_PushAlgorithm` (0x161e58): store the algorithm's
/// blend weight, then either push it directly (list empty) or clear the
/// list first when the weight is zero.
pub fn chase_cam_controller_push_algorithm(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save_fpr(c, 20, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(r(c, S1), 0xc)).lw(c);
        c.g[A0][0] = Addr(p(this, 8)).lw(c);
        Addr(p(this, 0x18)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(p(r(c, A0), 4)).lw(c);
        let has_algo = c.g[V0][0] != 0;
        c.mov_s(20, 12);
        if has_algo {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.c_eq_s(20, 0);
            let non_zero = !c.fcc();
            if non_zero {
                c.g[A0][0] = Addr(p(r(c, S0), 8)).lw(c);
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0016_1ec4);
                callee::ALGORITHM_LIST_CLEAR.call(c, 0x0016_1ec4)?;
                c.g[A0][0] = Addr(p(r(c, S0), 8)).lw(c);
            }
            c.cycles += 5;
            c.g[A1][0] = c.g[S1][0];
            c.mov_s(12, 20);
            c.g[RA][0] = sx(0x0016_1ed8);
            c.g[A2][0] = 1;
            callee::ALGORITHM_LIST_PUSH.call(c, 0x0016_1ed8)?;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 12);
            c.g[RA][0] = sx(0x0016_1e9c);
            c.g[A2][0] = 1;
            callee::ALGORITHM_LIST_PUSH.call(c, 0x0016_1e9c)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `MCManager_State0D` (0x23f308): ask the MC manager's object two virtual
/// questions and send state 0xe (or 0xf) to the result object.
pub fn mc_manager_state_0d(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);

        c.g[A2][0] = Addr(p(this, 0x434)).lw(c);
        c.g[A1][0] = Addr(p(this, 0xf8)).lw(c);
        c.g[V1][0] = Addr(r(c, A2)).lw(c);
        c.g[A0][0] = Addr(p(r(c, V1), 0x218)).lh(c);
        c.g[V0][0] = Addr(p(r(c, V1), 0x21c)).lw(c);
        let first_target = r(c, V0);
        c.g[RA][0] = sx(0x0023_f334);
        c.g[A0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
        call_ptr(c, first_target, 0x0023_f334)?;

        let handled = c.g[V0][0] != 0;
        let state = if handled {
            c.g[V0][0] = Addr(p(this, 0x748)).lw(c);
            0xe
        } else {
            c.cycles += 9;
            c.g[A2][0] = Addr(p(this, 0x434)).lw(c);
            c.g[A1][0] = Addr(p(this, 0xf8)).lw(c);
            c.g[V1][0] = Addr(r(c, A2)).lw(c);
            c.g[A0][0] = Addr(p(r(c, V1), 0x208)).lh(c);
            c.g[V0][0] = Addr(p(r(c, V1), 0x20c)).lw(c);
            let target = r(c, V0);
            c.g[RA][0] = sx(0x0023_f358);
            c.g[A0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
            call_ptr(c, target, 0x0023_f358)?;
            c.g[V0][0] = Addr(p(this, 0x748)).lw(c);
            if c.g[V0][0] == 0 {
                0xf
            } else {
                0xe
            }
        };
        if state == 0xe {
            c.cycles += 7;
        } else {
            c.cycles += 6;
        }
        c.g[A1][0] = state;
        c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
        c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
        let target = r(c, V1);
        c.g[RA][0] = sx(if state == 0xe {
            0x0023_f374
        } else {
            0x0023_f390
        });
        c.g[A0][0] = sx(this.wrapping_add(r(c, A0)));
        call_ptr(
            c,
            target,
            if state == 0xe {
                0x0023_f374
            } else {
                0x0023_f390
            },
        )?;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MCManager_State24` (0x23fc70): the manager's 0x24 state: poll the input
/// object, then send 3 or 6 to the result object.
pub fn mc_manager_state_24(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        c.g[A1][0] = Addr(p(this, 0x434)).lw(c);
        c.g[V1][0] = Addr(r(c, A1)).lw(c);
        c.g[A0][0] = Addr(p(r(c, V1), 0x58)).lh(c);
        c.g[V0][0] = Addr(p(r(c, V1), 0x5c)).lw(c);
        let target = r(c, V0);
        c.g[RA][0] = sx(0x0023_fc98);
        c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
        call_ptr(c, target, 0x0023_fc98)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 5;
        c.g[V0][0] = Addr(p(this, 0x6c)).lw(c);
        let target = r(c, V0);
        c.g[RA][0] = sx(0x0023_fcac);
        call_ptr(c, target, 0x0023_fcac)?;
        let zero = c.g[V0][0] == 0;
        c.g[A1][0] = 3;
        if !zero {
            c.cycles += 7;
            c.g[V0][0] = Addr(p(this, 0x748)).lw(c);
            c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
            let target = r(c, V1);
            c.g[RA][0] = sx(0x0023_fcc8);
            c.g[A0][0] = sx(this.wrapping_add(r(c, A0)));
            call_ptr(c, target, 0x0023_fcc8)?;
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 5;
        c.g[V0][0] = Addr(p(this, 0x84)).lw(c);
        let target = r(c, V0);
        c.g[RA][0] = sx(0x0023_fcdc);
        call_ptr(c, target, 0x0023_fcdc)?;
        let zero = c.g[V0][0] == 0;
        c.g[A1][0] = 6;
        if !zero {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(this, 0x748)).lw(c);
            c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
            let target = r(c, V1);
            c.g[RA][0] = sx(0x0023_fcf8);
            c.g[A0][0] = sx(this.wrapping_add(r(c, A0)));
            call_ptr(c, target, 0x0023_fcf8)?;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `TubeMenu_dtor` (0x250090): destroy the menu's eight child objects and
/// then the base menu.
pub fn tube_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        for (off, ret) in [
            (0x25c, 0x0025_00b4),
            (0x22c, 0x0025_00c0),
            (0x1fc, 0x0025_00cc),
            (0x1c8, 0x0025_00d8),
            (0x194, 0x0025_00e4),
            (0x164, 0x0025_00f0),
            (0x14c, 0x0025_00fc),
            (0x130, 0x0025_0108),
        ] {
            c.g[A0][0] = sx(this.wrapping_add(off));
            c.g[RA][0] = sx(ret);
            c.g[A1][0] = 2;
            call_ptr(c, callee::TUBE_MENU_ITEM_DTOR, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0025_0114);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, callee::TUBE_MENU_BASE_DTOR, 0x0025_0114)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_SetSfxVolume` (0x287488): convert the level to a volume
/// and apply it to the SFX channels (plus 9/10 when the extra bank exists).
pub fn ssx_audio_system_set_sfx_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0028_74a0);
        c.g[S0][0] = c.g[A0][0];
        callee::AUDIO_LEVEL_TO_VOLUME.call(c, 0x0028_74a0)?;
        c.mov_s(20, 0);
        for (channel, ret) in [
            (5u64, 0x0028_74b4u32),
            (3, 0x0028_74c4),
            (6, 0x0028_74d4),
            (7, 0x0028_74e4),
        ] {
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = channel;
            c.g[RA][0] = sx(ret);
            c.mov_s(12, 20);
            callee::AUDIO_SET_CHANNEL_VOLUME.call(c, ret)?;
        }
        c.g[V0][0] = Addr(p(r(c, S0), 0x62b4)).lw(c);
        let no_extra = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !no_extra {
            c.cycles += 7;
            c.g[A1][0] = 9;
            c.g[RA][0] = sx(0x0028_74fc);
            c.mov_s(12, 20);
            callee::AUDIO_SET_CHANNEL_VOLUME.call(c, 0x0028_74fc)?;
            c.g[A0][0] = c.g[S0][0];
            c.mov_s(12, 20);
            c.g[RA][0] = sx(0x0028_750c);
            c.g[A1][0] = 0xa;
            callee::AUDIO_SET_CHANNEL_VOLUME.call(c, 0x0028_750c)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `GameStatePreRaceMenu_Enter` (0x236bb0): leave the pause overlay, bump the
/// pre-race menu counter and enter the menu state (or the quick path when a
/// BE library / world is already active).
pub fn game_state_prerace_menu_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(gp(c, -0x204)).lw(c);
        frame.save_ra(c, 0);
        let be = c.g[V0][0] == 0;
        Addr(p(r(c, A0), 8)).sw(c, 0);
        if !be {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r(c, V0), 4)).lw(c);
            let no_lib = c.g[V0][0] == 0;
            c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
            if !no_lib {
                c.cycles += 7;
                c.g[V1][0] = Addr(p(r(c, V0), 0x84)).lw(c);
                c.g[RA][0] = sx(0x0023_6bdc);
                c.g[A0][0] = Addr(p(r(c, V1), 0x64)).lw(c);
                call_ptr(c, callee::OVERLAY_RESET, 0x0023_6bdc)?;
                c.g[RA][0] = sx(0x0023_6be4);
                c.g[A0][0] = 0;
                call_ptr(c, callee::BE_SET_PHASE, 0x0023_6be4)?;
                frame.restore_ra(c, 0);
                c.cycles += 2;
                let ra = r(c, RA);
                c.g[SP][0] = sx(r(c, SP).wrapping_add(0x10));
                c.pc = ra;
                return Ok(());
            }
        }
        c.cycles += 21;
        c.g[RA][0] = sx(0x0023_6bf4);
        c.g[A0][0] = 2;
        call_ptr(c, callee::GAME_MODULE_SET_STATE, 0x0023_6bf4)?;
        c.g[A2][0] = Addr(gp(c, -0x69c)).lw(c);
        c.g[V1][0] = 0x0053_0000;
        c.g[V1][0] = sx(0x0053_0000u32.wrapping_add(0x66e8));
        c.g[A0][0] = 0x0044_0000;
        c.g[A2][0] = sx(r(c, A2).wrapping_add(1));
        c.g[A1][0] = Addr(p(r(c, A0), 0x28f4)).lw(c);
        c.g[V0][0] = shl32(c.g[A2][0], 2);
        c.g[T0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
        c.g[A3][0] = 1;
        Addr(r(c, V0)).sw(c, c.g[A3][0]);
        Addr(gp(c, -0x6a0)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(p(r(c, T0), 0x84)).lw(c);
        Addr(gp(c, -0x69c)).sw(c, c.g[A2][0]);
        c.g[RA][0] = sx(0x0023_6c34);
        c.g[A0][0] = Addr(p(r(c, V0), 0x64)).lw(c);
        call_ptr(c, callee::OVERLAY_RESET, 0x0023_6c34)?;
        c.g[RA][0] = sx(0x0023_6c3c);
        c.g[A0][0] = 0;
        call_ptr(c, callee::BE_SET_PHASE, 0x0023_6c3c)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `SndBankMan_LoadBankAsync` (0x28bd10): if the bank is not loaded, cancel
/// any queued load of the same name and queue it again with the entry's id.
pub fn snd_bank_man_load_bank_async(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x60;
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.mult(0, 19, 17, 2);
        frame.save(c, S2, 0x30);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save(c, S4, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A3][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = Addr(p(this, 0xacc)).lw(c);
        c.g[RA][0] = sx(0x0028_bd54);
        c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, S3)));
        callee::SND_BANK_IS_LOADED_WITH_NAME.call(c, 0x0028_bd54)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x50);
        } else {
            c.cycles += 12;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0028_bd68);
            c.g[A1][0] = c.g[S1][0];
            callee::SND_LOAD_QUEUE_CANCEL_BANK.call(c, 0x0028_bd68)?;
            c.g[V0][0] = Addr(p(r(c, S0), 0xacc)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[V0][0] = sx(r(c, S3).wrapping_add(r(c, V0)));
            c.g[T0][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x0028_bd88);
            c.g[A3][0] = Addr(p(r(c, V0), 8)).lw(c);
            callee::SND_LOAD_QUEUE_PUSH.call(c, 0x0028_bd88)?;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_ConfirmMusicTrigger` (0x28d740): confirm a music trigger
/// when the object's request id matches, clearing the trigger otherwise.
pub fn ssx_audio_system_confirm_music_trigger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[V0][0] = Addr(p(this, 0x6240)).lw(c);
        let same = c.g[V0][0] == c.g[S2][0];
        let mut skip = false;
        if !same {
            c.cycles += 4;
            c.g[V1][0] = Addr(p(this, 0x62a0)).lw(c);
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            let none = c.g[V1][0] == c.g[V0][0];
            c.g[A0][0] = 0;
            if !none {
                c.cycles += 9;
                c.g[V1][0] = Addr(p(r(c, S1), 0x6c0)).lw(c);
                c.g[A1][0] = sx(r(c, S1).wrapping_add(0x6c0));
                c.g[A0][0] = Addr(p(r(c, V1), 0x38)).lh(c);
                c.g[V0][0] = Addr(p(r(c, V1), 0x3c)).lw(c);
                let target = r(c, V0);
                c.g[RA][0] = sx(0x0028_d790);
                c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
                call_ptr(c, target, 0x0028_d790)?;
                c.g[V1][0] = Addr(p(r(c, S0), 0x62a0)).lw(c);
                c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                c.g[A0][0] = (c.g[V0][0] < 1) as u64;
            }
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                frame.restore(c, S0, 0x30);
                skip = true;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(this, 0x623c)).lw(c);
                let different = c.g[V0][0] != c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                if different {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0028_d7c0);
                    c.g[A1][0] = c.g[S1][0];
                    callee::CLEAR_MUSIC_TRIGGER.call(c, 0x0028_d7c0)?;
                } else {
                    c.cycles += 2;
                    Addr(p(this, 0x6240)).sw(c, c.g[S2][0]);
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
            }
        } else {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
            skip = true;
        }
        let _ = skip;
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptProcess_JumpToTask` (0x307ec0): find the task with the given id in
/// the process's 0x44-byte task table, mark it jumped-to and call its start
/// handler.
pub fn w_script_process_jump_to_task(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        let this = r(c, A0);
        c.g[A2][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[A3][0] = Addr(p(this, 4)).lw(c);
        let none = (c.g[A3][0] as i64) <= 0;
        c.g[T0][0] = 0;
        let mut called = false;
        if none {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 4;
            c.g[T2][0] = 0x44;
            c.g[T3][0] = 3;
            c.g[T1][0] = 0;
            loop {
                c.cycles += 5;
                c.g[A0][0] = Addr(p(this, 0x14)).lw(c);
                c.g[V0][0] = sx(r(c, T1).wrapping_add(r(c, A0)));
                c.g[V1][0] = Addr(p(r(c, V0), 4)).lw(c);
                if c.g[V1][0] != c.g[A1][0] {
                    c.g[A3][0] = Addr(p(this, 4)).lw(c);
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(p(this, 8)).lw(c);
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    let in_range = c.g[V0][0] == 0;
                    c.mult(0, 5, 3, 10);
                    if !in_range {
                        c.cycles += 3;
                        c.g[V0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
                        Addr(r(c, V0)).sw(c, c.g[T3][0]);
                        c.g[A0][0] = Addr(p(this, 0x14)).lw(c);
                    }
                    c.cycles += 9;
                    Addr(p(this, 8)).sw(c, c.g[T0][0]);
                    c.g[A0][0] = sx(r(c, T1).wrapping_add(r(c, A0)));
                    c.g[V0][0] = Addr(p(r(c, A0), 0x40)).lw(c);
                    c.g[V1][0] = Addr(p(r(c, V0), 0x20)).lh(c);
                    c.g[A1][0] = Addr(p(r(c, V0), 0x24)).lw(c);
                    let target = r(c, A1);
                    c.g[RA][0] = sx(0x0030_7f34);
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V1)));
                    call_ptr(c, target, 0x0030_7f34)?;
                    frame.restore_ra(c, 0);
                    called = true;
                    break;
                }
                c.cycles += 4;
                c.g[T0][0] = sx(r(c, T0).wrapping_add(1));
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A3][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[T1][0] = sx(r(c, T1).wrapping_add(0x44));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_7ee8);
                }
            }
            if !called {
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenBEOptions_Update` (0x1fa9e0): update the BE options overlay and,
/// when the pause overlay is active, toggle the option thread's value.
pub fn ov_screen_be_options_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x001f_a9f4);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::OVERLAY_UPDATE, 0x001f_a9f4)?;
        c.g[V1][0] = Addr(gp(c, -0x204)).lw(c);
        let no_overlay = c.g[V1][0] == 0;
        if !no_overlay {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r(c, V1), 0x68)).lw(c);
            let zero68 = c.g[V0][0] == 0;
            c.g[A0][0] = 0;
            let active;
            if zero68 {
                c.cycles += 1;
                c.g[A0][0] = 1;
                active = true;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(r(c, V1), 0x64)).lw(c);
                active = c.g[V0][0] != 0;
                if !active {
                    c.cycles += 1;
                    c.g[A0][0] = 1;
                }
            }
            let _ = active;
            c.cycles += 2;
            if c.g[A0][0] != 0 {
                c.cycles += 10;
                c.g[A0][0] = 0x0047_0000;
                c.g[RA][0] = sx(0x001f_aa30);
                c.g[A0][0] = sx(0x0047_0000u32.wrapping_sub(0x1fb0));
                call_ptr(c, callee::FE_STRING_HASH, 0x001f_aa30)?;
                c.g[A0][0] = Addr(p(r(c, S0), 0x40)).lw(c);
                c.g[RA][0] = sx(0x001f_aa3c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, callee::UI_SCREEN_LOOKUP_VALUE, 0x001f_aa3c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = 0xffff;
                let is_ffff = c.g[A1][0] == c.g[V0][0];
                c.g[A1][0] = c.g[A1][0] & 0xffff;
                if !is_ffff {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(p(r(c, S0), 0x40)).lw(c);
                    c.g[RA][0] = sx(0x001f_aa58);
                    c.g[A2][0] = 1;
                    call_ptr(c, callee::UI_SCREEN_LAUNCH_THREAD, 0x001f_aa58)?;
                }
                c.cycles += 5;
                c.g[A0][0] = Addr(p(r(c, S0), 0x10)).lw(c);
                c.g[A1][0] = 1;
                c.g[RA][0] = sx(0x001f_aa68);
                c.g[A0][0] = sx(r(c, A0).wrapping_add(0x18));
                call_ptr(c, callee::UI_SCREEN_SET_OBJECT_VALUE, 0x001f_aa68)?;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cDynamicSprayFX_Reset` (0x2df3b0): clear the spray burst state, reset the
/// emitter colour/scale, and reconfigure for the current rider if needed.
pub fn dynamic_spray_fx_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x30);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        frame.save(c, S0, 0x20);
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 1);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        c.swc1(0, p(r(c, SP), 0xc));
        c.swc1(0, r(c, SP));
        c.swc1(0, p(r(c, SP), 4));
        c.swc1(0, p(r(c, SP), 8));
        c.sd(p(r(c, SP), 0x10), c.g[RA][0]);
        c.swc1(1, p(this, 0x120));
        c.g[V1][0] = Addr(this).lw(c);
        c.lq(2, r(c, SP));
        Addr(p(this, 0x10)).sw(c, 0);
        c.sq(2, p(this, 0x90));
        Addr(p(this, 0x74)).sw(c, 0);
        Addr(p(this, 4)).sw(c, 0);
        Addr(p(this, 0x1c)).sw(c, 0);
        Addr(p(this, 0x80)).sw(c, 0);
        Addr(p(this, 0x78)).sw(c, 0);
        Addr(p(this, 0xe0)).sw(c, 0);
        Addr(p(this, 0x12c)).sw(c, 0);
        c.g[A0][0] = Addr(p(this, 0x124)).lw(c);
        c.g[V0][0] = Addr(p(r(c, V1), 0x870)).lw(c);
        c.g[V0][0] = !(0u64 | c.g[V0][0]);
        c.g[V0][0] = sx(r(c, V0) >> 31);
        let same = c.g[A0][0] == c.g[V0][0];
        if !same {
            c.cycles += 2;
            c.g[RA][0] = sx(0x002d_f430);
            c.g[A0][0] = c.g[S0][0];
            callee::SPRAY_CONFIGURE_FOR_RIDER.call(c, 0x002d_f430)?;
        }
        c.cycles += 6;
        c.g[RA][0] = sx(0x002d_f438);
        c.g[A0][0] = c.g[S0][0];
        callee::SPRAY_UPDATE_EMITTER_ENABLES.call(c, 0x002d_f438)?;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `BENewPlayerInt_SetCharProfileFlag` (0x145dd0): set or clear one player's
/// character-profile flag (a2 = flag bit, a3 selects set/clear).
pub fn be_new_player_int_set_char_profile_flag(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A3][0];
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0014_5dfc);
        c.g[A0][0] = c.g[S0][0];
        callee::BE_GET_RIDER_PROFILE.call(c, 0x0014_5dfc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0014_5e08);
        c.g[S1][0] = c.g[V0][0];
        callee::BENEW_PLAYER_INT_GET_SLOT.call(c, 0x0014_5e08)?;
        c.g[V1][0] = 0x1c;
        c.g[A0][0] = 0x0053_0000;
        c.mult(0, 2, 2, 3);
        c.g[A0][0] = sx(0x0053_0000u32.wrapping_add(0x4fe0));
        c.g[T0][0] = 0xf88;
        c.g[V1][0] = 0x9b50;
        c.mult(0, 17, 17, 3);
        c.g[A3][0] = 0x004a_0000;
        c.g[A3][0] = sx(0x004a_0000u32.wrapping_add(0x6ca8));
        c.g[A1][0] = c.g[S2][0];
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
        c.g[A2][0] = c.g[S3][0] ^ 1;
        c.g[A0][0] = Addr(p(r(c, V0), 0x11)).lb(c);
        c.mult(0, 4, 4, 8);
        c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, A3)));
        c.g[RA][0] = sx(0x0014_5e4c);
        c.g[A0][0] = sx(r(c, S1).wrapping_add(r(c, A0)));
        callee::BE_SET_CHAR_PROFILE_FLAG.call(c, 0x0014_5e4c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GameRand_Shuffle` (0x23c770): Fisher–Yates shuffle of 25 words using the
/// game RNG (two draws per swap).
pub fn game_rand_shuffle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        let this = r(c, A0);
        c.g[S2][0] = sx(this);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = 0x18;
        frame.save_ra(c, 0);
        loop {
            c.cycles += 5;
            c.g[RA][0] = sx(0x0023_c7a0);
            c.g[S3][0] = sx(r(c, S3).wrapping_sub(1));
            callee::GAME_RAND_NEXT.call(c, 0x0023_c7a0)?;
            c.divu(0, 2, 17);
            c.cycles += 6;
            c.g[S0][0] = c.hi[0];
            c.g[RA][0] = sx(0x0023_c7b8);
            callee::GAME_RAND_NEXT.call(c, 0x0023_c7b8)?;
            c.divu(0, 2, 17);
            c.cycles += 10;
            c.g[S0][0] = shl32(c.g[S0][0], 2);
            c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, S2)));
            c.g[A0][0] = Addr(r(c, S0)).lw(c);
            c.g[V1][0] = c.hi[0];
            c.g[V1][0] = shl32(c.g[V1][0], 2);
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, S2)));
            c.g[V0][0] = Addr(r(c, V1)).lw(c);
            Addr(r(c, S0)).sw(c, c.g[V0][0]);
            let more = (c.g[S3][0] as i64) >= 0;
            Addr(r(c, V1)).sw(c, c.g[A0][0]);
            if !more {
                c.cycles += 7;
                frame.restore(c, S0, 0x40);
                frame.restore(c, S1, 0x30);
                frame.restore(c, S2, 0x20);
                frame.restore(c, S3, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x0023_c798);
            }
        }
    })
}

/// `WorldTriggerManager_AnyTriggerFlagged` (0x2b6740): true when any of the
/// four trigger ids the world sound gives for `a1` has flag 4 set.
pub fn world_trigger_manager_any_trigger_flagged(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x002b_675c);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        call_ptr(c, callee::AUDIO_GET_GAME_MODULE, 0x002b_675c)?;
        c.g[A0][0] = Addr(p(r(c, V0), 0x4c)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = c.g[SP][0] | 4;
        c.g[T0][0] = c.g[SP][0] | 8;
        c.g[RA][0] = sx(0x002b_6778);
        c.g[T1][0] = c.g[SP][0] | 0xc;
        callee::WORLD_SOUND_GET_TRIGGER_IDS.call(c, 0x002b_6778)?;
        c.g[A1][0] = Addr(r(c, SP)).lw(c);
        c.g[RA][0] = sx(0x002b_6784);
        c.g[A0][0] = c.g[S1][0];
        callee::WORLD_TRIGGER_IS_FLAG4.call(c, 0x002b_6784)?;
        let any = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !any {
            c.cycles += 5;
            c.g[A1][0] = Addr(p(r(c, SP), 4)).lw(c);
            c.g[RA][0] = sx(0x002b_6798);
            c.g[A0][0] = c.g[S1][0];
            callee::WORLD_TRIGGER_IS_FLAG4.call(c, 0x002b_6798)?;
            let any = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if !any {
                c.cycles += 5;
                c.g[A1][0] = Addr(p(r(c, SP), 8)).lw(c);
                c.g[RA][0] = sx(0x002b_67ac);
                c.g[A0][0] = c.g[S1][0];
                callee::WORLD_TRIGGER_IS_FLAG4.call(c, 0x002b_67ac)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 4;
                    c.g[A1][0] = Addr(p(r(c, SP), 0xc)).lw(c);
                    c.g[RA][0] = sx(0x002b_67c0);
                    c.g[A0][0] = c.g[S1][0];
                    callee::WORLD_TRIGGER_IS_FLAG4.call(c, 0x002b_67c0)?;
                    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                } else {
                    c.g[V0][0] = 1;
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `DataInstanceMan_ctor` (0x275828): allocate and clear a table of `count`
/// 0x18-byte data-instance records.
pub fn data_instance_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S1, 0x20);
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_add(0x25a0));
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = 0x18;
        c.mult(0, 0, 16, 4);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A1][0];
        Addr(this).sw(c, c.g[S0][0]);
        c.g[A1][0] = 0x0048_0000;
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_add(0x1ab8));
        c.g[A0][0] = c.lo[0];
        c.g[RA][0] = sx(0x0027_5874);
        c.g[A3][0] = 0;
        callee::OPERATOR_NEW.call(c, 0x0027_5874)?;
        Addr(p(this, 4)).sw(c, c.g[V0][0]);
        let empty = (c.g[S0][0] as i64) <= 0;
        Addr(p(this, 8)).sw(c, 0);
        if !empty {
            c.cycles += 2;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 7;
                c.g[A1][0] = Addr(p(this, 4)).lw(c);
                c.g[A0][0] = c.g[S1][0];
                c.g[S0][0] = sx(r(c, S0).wrapping_sub(1));
                c.g[RA][0] = sx(0x0027_589c);
                c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, S2)));
                callee::DATA_INSTANCE_FREE_INFO.call(c, 0x0027_589c)?;
                let more = c.g[S0][0] != 0;
                c.g[S2][0] = sx(r(c, S2).wrapping_add(0x18));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_5888);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptList_CheckNextReady` (0x2776e0): if the next script cmd is a movie
/// entry or its process is paused, pause the list.
pub fn script_list_check_next_ready(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(p(this, 0xbc)).lw(c);
        let invalid = (c.g[A1][0] as i64) < 0;
        c.g[S2][0] = sx(this.wrapping_add(0xc));
        if !invalid {
            c.cycles += 8;
            c.g[RA][0] = sx(0x0027_770c);
            c.g[A0][0] = c.g[S2][0];
            callee::SCRIPT_CMD_QUEUE_AT.call(c, 0x0027_770c)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0027_771c);
            c.g[A1][0] = c.g[S1][0];
            callee::SCRIPT_LIST_IS_MOVIE_ENTRY.call(c, 0x0027_771c)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(p(r(c, S1), 0xc)).lw(c);
                c.cycles += 1;
                c.g[V0][0] = sx(r(c, V0) >> 31);
            } else {
                c.cycles += 9;
                c.g[A1][0] = Addr(p(r(c, S0), 0xbc)).lw(c);
                c.g[RA][0] = sx(0x0027_7730);
                c.g[A0][0] = c.g[S2][0];
                callee::SCRIPT_CMD_QUEUE_AT.call(c, 0x0027_7730)?;
                c.g[A1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
                c.g[RA][0] = sx(0x0027_773c);
                c.g[A0][0] = Addr(r(c, S0)).lw(c);
                callee::SCRIPT_ENGINE_GET_PROCESS_STATE.call(c, 0x0027_773c)?;
                c.g[V0][0] = c.g[V0][0] ^ 1;
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
            }
            c.cycles += 2;
            let ready = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !ready {
                c.cycles += 3;
                c.g[A1][0] = 1;
                c.g[RA][0] = sx(0x0027_7760);
                c.g[A2][0] = 1;
                callee::SCRIPT_LIST_SET_PAUSED.call(c, 0x0027_7760)?;
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cBoltOnFXInterface_dtor` (0x2ecb28): destroy the interface's fifteen
/// attached emitter slots and optionally free the object.
pub fn bolt_on_fx_interface_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S2, 0x30);
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_add(0x7ee8));
        frame.save(c, S0, 0x50);
        let this = r(c, A0);
        c.g[S2][0] = sx(this);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = sx(this.wrapping_add(0x10));
        frame.save(c, S3, 0x20);
        c.g[S1][0] = 0xe;
        frame.save(c, S4, 0x10);
        c.g[S3][0] = 0xffff_ffff_ffff_ffff;
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A1][0];
        Addr(p(this, 0xc4)).sw(c, c.g[V0][0]);
        loop {
            c.cycles += 3;
            c.g[A1][0] = Addr(r(c, S0)).lw(c);
            if c.g[A1][0] != c.g[S3][0] {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002e_cb7c);
                c.g[A0][0] = Addr(p(r(c, S2), 0xc)).lw(c);
                call_ptr(c, callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER, 0x002e_cb7c)?;
                Addr(r(c, S0)).sw(c, c.g[S3][0]);
            }
            c.cycles += 2;
            c.g[S1][0] = sx(r(c, S1).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(r(c, S0).wrapping_add(0xc));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002e_cb68);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S4][0] & 1;
        let free_self = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x50);
        if free_self {
            c.cycles += 3;
            c.g[RA][0] = sx(0x002e_cba0);
            c.g[A0][0] = c.g[S2][0];
            callee::BUILTIN_DELETE.call(c, 0x002e_cba0)?;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
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

/// `cUITextScroll_ClearLines` (0x3a18c0): free every stored line string,
/// null the line array and reset the counters.
pub fn ui_text_scroll_clear_lines(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(this, 0xbc)).lw(c);
        if c.g[V0][0] == 0 {
            Addr(p(this, 0xc8)).sh(c, 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(this, 0xc8)).lhu(c);
            let none = c.g[V0][0] == 0;
            c.g[S0][0] = 0;
            if !none {
                c.cycles += 1;
                c.g[V1][0] = Addr(p(this, 0xbc)).lw(c);
                loop {
                    c.cycles += 5;
                    c.g[V0][0] = shl32(c.g[S0][0], 2);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.g[A0][0] = Addr(r(c, V0)).lw(c);
                    let zero = c.g[A0][0] == 0;
                    c.g[V1][0] = sx(r(c, S0).wrapping_add(1));
                    if !zero {
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x003a_190c);
                        callee::EA_MEM_FREE.call(c, 0x003a_190c)?;
                        c.g[V1][0] = sx(r(c, S0).wrapping_add(1));
                    }
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p(this, 0xc8)).lhu(c);
                    c.g[S0][0] = c.g[V1][0] & 0xffff;
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[V1][0] = Addr(p(this, 0xbc)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_18f0);
                    }
                }
            }
            c.cycles += 3;
            c.g[A0][0] = Addr(p(this, 0xbc)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003a_1938);
                callee::EA_MEM_FREE.call(c, 0x003a_1938)?;
            }
            Addr(p(this, 0xbc)).sw(c, 0);
            c.cycles += 1;
            Addr(p(this, 0xc8)).sh(c, 0);
        }
        c.cycles += 6;
        Addr(p(this, 0xc6)).sh(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RailMan_ctor` (0x3584f0): construct the sync process and clear the rail
/// manager's two 128-entry tables.
pub fn rail_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0x12;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0035_8508);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        callee::SYNC_PROCESS_CTOR.call(c, 0x0035_8508)?;
        c.g[V0][0] = 0x0049_0000;
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_sub(0x1220));
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = 0x7f;
        c.g[V0][0] = sx(this.wrapping_add(0x18));
        c.g[A0][0] = c.g[A0][0] | 0xffff;
        loop {
            c.cycles += 7;
            Addr(r(c, V0)).sw(c, c.g[A0][0]);
            c.g[V1][0] = sx(r(c, V1).wrapping_sub(1));
            let more = c.g[V1][0] != c.g[A1][0];
            c.g[V0][0] = sx(r(c, V0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_8528);
            }
        }
        c.cycles += 5;
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        Addr(gp(c, 0xf50)).sw(c, c.g[S0][0]);
        c.g[A0][0] = c.g[A0][0] | 0xffff;
        c.g[V1][0] = 0x7f;
        c.g[V0][0] = sx(this.wrapping_add(0x214));
        loop {
            c.cycles += 7;
            Addr(r(c, V0)).sw(c, c.g[A0][0]);
            c.g[V1][0] = sx(r(c, V1).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(r(c, V0).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_8558);
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        let ra = r(c, RA);
        c.g[SP][0] = sx(r(c, SP).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

/// `VfxAvalanche_dtor` (0x2d6378): destroy every avalanche effect in the
/// static bank, install the base vtable and optionally free the object.
pub fn vfx_avalanche_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S1, 0x30);
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_sub(0x79b8));
        frame.save(c, S0, 0x40);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x002d_63a8);
        Addr(this).sw(c, c.g[V0][0]);
        callee::AVALANCHE_DESTROY_ALL.call(c, 0x002d_63a8)?;
        c.g[V0][0] = 0x004f_0000;
        c.g[V1][0] = 0xbc00;
        c.g[S0][0] = sx(0x004f_0000u32.wrapping_sub(0x17c0));
        c.g[S2][0] = sx(r(c, S0).wrapping_add(r(c, V1)));
        c.g[A0][0] = c.g[S0][0];
        loop {
            c.cycles += 6;
            c.g[RA][0] = sx(0x002d_63c8);
            c.g[A1][0] = 1;
            call_ptr(c, callee::AVALANCHE_RESET_ONE, 0x002d_63c8)?;
            c.g[S0][0] = sx(r(c, S0).wrapping_add(0x2f0));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_63c0);
            }
        }
        c.cycles += 5;
        c.g[V0][0] = 0x0049_0000;
        c.g[V1][0] = c.g[S3][0] & 1;
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_sub(0x7980));
        let free_self = c.g[V1][0] == 0;
        Addr(this).sw(c, c.g[V0][0]);
        if !free_self {
            c.cycles += 2;
            c.g[RA][0] = sx(0x002d_63f4);
            c.g[A0][0] = c.g[S1][0];
            callee::BUILTIN_DELETE.call(c, 0x002d_63f4)?;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MdfFile_CreateModelByName` (0x314c00): find the model whose name matches
/// `a1` (case-insensitive) in the file's 0x60-byte records and create it.
pub fn mdf_file_create_model_by_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, S3, 0x10);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(this, 4)).lw(c);
        c.g[V0][0] = Addr(p(r(c, V1), 4)).lh(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = c.g[A1][0];
        if empty {
            c.cycles += 2;
            c.g[V0][0] = 0;
            frame.restore(c, S0, 0x40);
        } else {
            c.cycles += 2;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 6;
                c.g[A1][0] = Addr(p(this, 8)).lw(c);
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x0031_4c48);
                c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, S2)));
                call_ptr(c, callee::STRCASECMP, 0x0031_4c48)?;
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = Addr(p(this, 4)).lw(c);
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0031_4c5c);
                    c.g[A1][0] = c.g[S0][0];
                    callee::MDF_FILE_CREATE_MODEL.call(c, 0x0031_4c5c)?;
                    frame.restore(c, S0, 0x40);
                    break;
                }
                c.cycles += 5;
                c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                c.g[V0][0] = Addr(p(r(c, V1), 4)).lh(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx(r(c, S2).wrapping_add(0x60));
                if !more {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    frame.restore(c, S0, 0x40);
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_4c38);
                }
            }
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

/// `OVScreenYNDialogQuitGame_Update` (0x20dbd0): the quit-game yes/no dialog
/// update: run the overlay update and toggle the option thread's value.
pub fn ov_screen_yn_dialog_quit_game_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0020_dbe4);
        c.g[S0][0] = c.g[A0][0];
        callee::OVERLAY_UPDATE_CALLEE.call(c, 0x0020_dbe4)?;
        c.g[V1][0] = Addr(gp(c, -0x204)).lw(c);
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r(c, V1), 0x68)).lw(c);
            let zero68 = c.g[V0][0] == 0;
            c.g[A0][0] = 0;
            if zero68 {
                c.cycles += 1;
                c.g[A0][0] = 1;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(r(c, V1), 0x64)).lw(c);
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    c.g[A0][0] = 1;
                }
            }
            c.cycles += 2;
            if c.g[A0][0] != 0 {
                c.cycles += 10;
                c.g[A0][0] = 0x0047_0000;
                c.g[RA][0] = sx(0x0020_dc20);
                c.g[A0][0] = sx(0x0047_0000u32.wrapping_sub(0x1fb0));
                call_ptr(c, callee::FE_STRING_HASH, 0x0020_dc20)?;
                c.g[A0][0] = Addr(p(r(c, S0), 0x40)).lw(c);
                c.g[RA][0] = sx(0x0020_dc2c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, callee::UI_SCREEN_LOOKUP_VALUE, 0x0020_dc2c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = 0xffff;
                let is_ffff = c.g[A1][0] == c.g[V0][0];
                c.g[A1][0] = c.g[A1][0] & 0xffff;
                if !is_ffff {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(p(r(c, S0), 0x40)).lw(c);
                    c.g[RA][0] = sx(0x0020_dc48);
                    c.g[A2][0] = 1;
                    call_ptr(c, callee::UI_SCREEN_LAUNCH_THREAD, 0x0020_dc48)?;
                }
                c.cycles += 5;
                c.g[A0][0] = Addr(p(r(c, S0), 0x10)).lw(c);
                c.g[A1][0] = 1;
                c.g[RA][0] = sx(0x0020_dc58);
                c.g[A0][0] = sx(r(c, A0).wrapping_add(0x18));
                call_ptr(c, callee::UI_SCREEN_SET_OBJECT_VALUE, 0x0020_dc58)?;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateMemCard_v33` (0x1874e8): memory-card state helper: once the card
/// worker is busy, finish; otherwise start the profile search unless a save
/// is in progress.
pub fn fe_state_mem_card_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        c.g[V0][0] = Addr(p(this, 0x1ac)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 6;
            c.g[RA][0] = sx(0x0018_750c);
            c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
            call_ptr(c, callee::MC_GET_WORLD, 0x0018_750c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V0][0] = Addr(p(r(c, A0), 0x440)).lw(c);
            let busy = c.g[V0][0] != 0;
            c.g[V1][0] = 0;
            if !busy {
                c.cycles += 2;
                c.g[V0][0] = Addr(p(r(c, A0), 0x43c)).lw(c);
                c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
            }
            c.cycles += 2;
            let mut early = false;
            if c.g[V1][0] != 0 {
                frame.restore(c, S0, 0x10);
                early = true;
            } else {
                c.cycles += 4;
                c.g[V1][0] = Addr(gp(c, -0x1c34)).lw(c);
                c.g[V0][0] = 4;
                let mut finish = true;
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p(this, 0x238)).lw(c);
                    c.g[V0][0] = c.g[V0][0] ^ 1;
                    c.g[V0][0] = c.g[V0][0] & 1;
                    if c.g[V0][0] != 0 {
                        c.cycles += 6;
                        c.g[RA][0] = sx(0x0018_7558);
                        call_ptr(c, callee::MC_MANAGER_GET, 0x0018_7558)?;
                        c.g[RA][0] = sx(0x0018_7560);
                        callee::FE_MEMCARD_START_PROFILE_SEARCH.call(c, 0x0018_7560)?;
                        frame.restore(c, S0, 0x10);
                        finish = false;
                    }
                }
                if finish {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0018_7570);
                    c.g[A0][0] = c.g[S0][0];
                    callee::FE_MEMCARD_FINISH.call(c, 0x0018_7570)?;
                    frame.restore(c, S0, 0x10);
                }
                let _ = early;
            }
        } else {
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GameModule_CanRestartFromPause` (0x231ab8): true when the game module is
/// not loading and its state is not one of the pause-menu transition states.
pub fn game_module_can_restart_from_pause(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(this, 0x210)).lw(c);
        c.g[V0][0] = 5;
        let zero = c.g[V1][0] == 0;
        let mut skip_check = false;
        if zero {
            c.cycles += 1;
            c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
            skip_check = true;
        } else {
            c.cycles += 2;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = 0;
                frame.restore(c, S1, 0x10);
                frame.restore(c, S0, 0x20);
                c.cycles += 5;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 2;
            c.g[V0][0] = 0xe;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = 0;
                frame.restore(c, S1, 0x10);
                frame.restore(c, S0, 0x20);
                c.cycles += 5;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = 0;
                frame.restore(c, S1, 0x10);
                frame.restore(c, S0, 0x20);
                c.cycles += 5;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 2;
            c.g[V0][0] = 0xa;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = 0;
                frame.restore(c, S1, 0x10);
                frame.restore(c, S0, 0x20);
                c.cycles += 5;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 2;
            c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
        }
        let _ = skip_check;
        c.cycles += 4;
        c.g[RA][0] = sx(0x0023_1b0c);
        c.g[S0][0] = 0;
        call_ptr(c, callee::GAME_STATE_IS_LOADING, 0x0023_1b0c)?;
        let loaded = c.g[V0][0] == 0;
        c.g[V0][0] = 5;
        if loaded {
            c.g[S0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[A0][0] = Addr(p(this, 0x214)).lw(c);
            let five = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0xe;
            if !five {
                c.cycles += 2;
                let fourteen = c.g[A0][0] == c.g[V0][0];
                c.g[V1][0] = 1;
                if !fourteen {
                    c.cycles += 2;
                    let one = c.g[A0][0] == c.g[V1][0];
                    c.g[V0][0] = c.g[A0][0] ^ 0xa;
                    if !one {
                        c.cycles += 2;
                        c.g[S0][0] = c.g[V1][0];
                        if c.g[V0][0] == 0 {
                            c.g[S0][0] = 0;
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[S0][0];
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LoadIndicator_Init` (0x1f30d8): look up the "loading" text object and
/// initialise the indicator's object/thread fields.
pub fn load_indicator_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let this = r(c, A0);
        c.g[S2][0] = sx(this);
        frame.save(c, S1, 0x20);
        c.g[A0][0] = 0x0047_0000;
        frame.save(c, S0, 0x30);
        c.g[A0][0] = sx(0x0047_0000u32.wrapping_sub(0x1288));
        c.g[S0][0] = Addr(p(r(c, V0), 0x8c)).lw(c);
        c.g[S1][0] = Addr(p(r(c, S0), 4)).lw(c);
        c.g[V0][0] = Addr(p(r(c, S1), 0x20)).lh(c);
        c.g[RA][0] = sx(0x001f_3110);
        c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
        call_ptr(c, callee::FE_STRING_HASH, 0x001f_3110)?;
        c.g[V1][0] = Addr(p(r(c, S1), 0x24)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        let target = r(c, V1);
        c.g[RA][0] = sx(0x001f_3120);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, target, 0x001f_3120)?;
        Addr(r(c, S2)).sw(c, c.g[V0][0]);
        c.g[A0][0] = 0x004a_0000;
        c.g[A1][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[A0][0] = sx(0x004a_0000u32.wrapping_add(0x21f0));
        c.g[V0][0] = Addr(p(r(c, A1), 0x84)).lw(c);
        c.g[V1][0] = Addr(p(r(c, V0), 0x48)).lw(c);
        c.g[RA][0] = sx(0x001f_3140);
        c.g[S0][0] = Addr(p(r(c, V1), 8)).lw(c);
        call_ptr(c, callee::FE_STRING_HASH, 0x001f_3140)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x001f_314c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_SET_OBJECT_VALUE, 0x001f_314c)?;
        Addr(p(r(c, S2), 0xc)).sw(c, c.g[V0][0]);
        Addr(p(r(c, S2), 8)).sw(c, 0);
        Addr(p(r(c, S2), 4)).sw(c, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
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
        callee::UI_LIST_FIRST.call(c, 0x0039_f610)?;
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

/// `TransitionEffect_DrawFade` (0x2ebb10): draw the fade rectangle (the
/// effect's 16-byte colour quad, its alpha scaled by `f12`).
pub fn transition_effect_draw_fade(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x40);
        let this = r(c, A0);
        c.g[V0][0] = sx(this);
        frame.save_ra(c, 0x30);
        c.g[A0][0] = sx(r(c, SP).wrapping_add(0x10));
        c.g[AT][0] = 0x4420_0000;
        c.mtc1(1, 6);
        c.lwc1(0, p(this, 4));
        c.ldl(3, p(this, 0xb));
        c.ldr(3, p(this, 4));
        c.ldl(5, p(this, 0x13));
        c.ldr(5, p(this, 0xc));
        c.sdl(3, p(r(c, SP), 7));
        c.sdr(3, r(c, SP));
        c.sdl(5, p(r(c, SP), 0xf));
        c.sdr(5, p(r(c, SP), 8));
        c.mul_s(0, 0, 12);
        Addr(p(r(c, SP), 0x10)).sw(c, 0);
        c.g[AT][0] = 0x43f0_0000;
        c.mtc1(1, 5);
        c.g[A1][0] = sx(r(c, SP).wrapping_add(0x20));
        c.lwc1(1, gp(c, 0xa40));
        c.g[A2][0] = c.g[SP][0];
        c.lwc1(2, gp(c, 0xa4c));
        c.lwc1(3, gp(c, 0xa48));
        c.lwc1(4, gp(c, 0xa54));
        c.lwc1(12, p(r(c, SP), 0x10));
        c.swc1(0, r(c, SP));
        c.swc1(6, p(r(c, SP), 0x18));
        c.swc1(5, p(r(c, SP), 0x1c));
        c.swc1(1, p(r(c, SP), 0x20));
        c.swc1(2, p(r(c, SP), 0x24));
        c.swc1(3, p(r(c, SP), 0x28));
        c.swc1(4, p(r(c, SP), 0x2c));
        c.g[RA][0] = sx(0x002e_bb9c);
        Addr(p(r(c, SP), 0x14)).sw(c, 0);
        callee::TRANSITION_EFFECT_DRAW_RECT.call(c, 0x002e_bb9c)?;
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `BE_PrepareNetProfiles` (0x14e9f8): reset the profile characters and copy
/// the current profile into the second slot unless a net profile exists.
pub fn be_prepare_net_profiles(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x004c_0000;
        c.g[RA][0] = sx(0x0014_ea18);
        c.g[A0][0] = sx(0x004c_0000u32.wrapping_sub(0x5cb8));
        callee::BE_RESET_PROFILE_CHARACTERS.call(c, 0x0014_ea18)?;
        c.g[V0][0] = 0x0053_0000;
        c.g[V1][0] = Addr(0x0053_0000u32.wrapping_add(0x4b38)).lw(c);
        let have_net = c.g[V1][0] != 0;
        c.g[V1][0] = 0x0053_0000;
        if !have_net {
            c.cycles += 20;
            c.g[A0][0] = c.g[S0][0];
            c.g[V0][0] = sx(0x0053_0000u32.wrapping_add(0x5b3c));
            c.g[A1][0] = 0;
            c.g[T2][0] = sx(0x0053_0000u32.wrapping_add(0x5b3c));
            let v0 = r(c, V0);
            let t2 = r(c, T2);
            c.ldl(7, v0.wrapping_sub(0x15));
            c.ldr(7, v0.wrapping_sub(0x1c));
            c.ldl(8, v0.wrapping_sub(0xd));
            c.ldr(8, v0.wrapping_sub(0x14));
            c.ldl(9, v0.wrapping_sub(5));
            c.ldr(9, v0.wrapping_sub(0xc));
            c.sdl(7, t2.wrapping_add(7));
            c.sdr(7, t2);
            c.sdl(8, t2.wrapping_add(0xf));
            c.sdr(8, t2.wrapping_add(8));
            c.sdl(9, t2.wrapping_add(0x17));
            c.sdr(9, t2.wrapping_add(0x10));
            c.g[A3][0] = Addr(v0.wrapping_sub(4)).lw(c);
            Addr(t2.wrapping_add(0x18)).sw(c, c.g[A3][0]);
            c.g[RA][0] = sx(0x0014_ea78);
            c.g[A2][0] = 1;
            callee::BE_COPY_PROFILE.call(c, 0x0014_ea78)?;
        }
        c.cycles += 6;
        c.g[RA][0] = sx(0x0014_ea80);
        c.g[A0][0] = c.g[S0][0];
        callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS.call(c, 0x0014_ea80)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BE_SetCharStateFlagCheat` (0x150b88): set or clear character-state flag
/// 12/13 for player 0 or 1 (cheat helper), returning 1 when handled.
pub fn be_set_char_state_flag_cheat(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        c.g[A0][0] = c.g[A1][0];
        c.g[V0][0] = ((c.g[A0][0] as i64) < 2i64) as u64;
        let player_ok = c.g[V0][0] != 0;
        c.g[A1][0] = c.g[T0][0];
        if player_ok {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let one = c.g[A3][0] != c.g[V0][0];
            c.g[A1][0] = c.g[A1][0] ^ 1;
            if !one {
                c.cycles += 10;
                c.g[V1][0] = 0xf88;
                c.g[V0][0] = 0x9b50;
                c.mult(0, 2, 4, 2);
                c.mult(0, 3, 6, 3);
                c.g[A0][0] = 0x004a_0000;
                c.g[A1][0] = c.g[A1][0] & 1;
                c.g[A0][0] = sx(0x004a_0000u32.wrapping_add(0x6ca8));
                c.g[A2][0] = 0xffff_ffff_ffff_efff;
                c.g[A1][0] = c.g[A1][0] << 12;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 2;
                let two = c.g[A3][0] != c.g[V0][0];
                c.g[V1][0] = 0xf88;
                if two {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    abi::ret(c);
                    return Ok(());
                }
                c.cycles += 8;
                c.g[V0][0] = 0x9b50;
                c.mult(0, 2, 4, 2);
                c.mult(0, 3, 6, 3);
                c.g[A0][0] = 0x004a_0000;
                c.g[A1][0] = c.g[A1][0] & 1;
                c.g[A0][0] = sx(0x004a_0000u32.wrapping_add(0x6ca8));
                c.g[A2][0] = 0xffff_ffff_ffff_dfff;
                c.g[A1][0] = c.g[A1][0] << 13;
            }
            c.cycles += 6;
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
            c.g[V0][0] = c.ld(p(r(c, V1), 0x278));
            c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
            c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
            c.sd(p(r(c, V1), 0x278), c.g[V0][0]);
            c.cycles += 2;
            c.g[V0][0] = 1;
            abi::ret(c);
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
        }
        Ok(())
    })
}

/// `WorldState_ReadObjects` (0x26ddc0): read the object count and create each
/// world object, then finalize the list.
pub fn world_state_read_objects(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        let this = r(c, A0);
        c.g[S3][0] = sx(this);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 8;
        c.g[V0][0] = Addr(r(c, S1)).lw(c);
        c.g[A0][0] = Addr(p(r(c, V0), 0x10)).lh(c);
        c.g[V1][0] = Addr(p(r(c, V0), 0x14)).lw(c);
        let target = r(c, V1);
        c.g[RA][0] = sx(0x0026_de04);
        c.g[A0][0] = sx(r(c, S1).wrapping_add(r(c, A0)));
        call_ptr(c, target, 0x0026_de04)?;
        c.g[V0][0] = Addr(r(c, SP)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = c.g[S3][0];
        if !empty {
            loop {
                c.cycles += 8;
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0026_de20);
                c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                callee::WORLD_STATE_CREATE_OBJECT.call(c, 0x0026_de20)?;
                c.g[V0][0] = Addr(r(c, SP)).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S3][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_de10);
                }
            }
        }
        c.cycles += 10;
        c.g[A0][0] = sx(r(c, GP).wrapping_add(0x2898u32));
        c.g[RA][0] = sx(0x0026_de3c);
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, callee::WORLD_STATE_FINALIZE, 0x0026_de3c)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable, FAKE};
    use crate::difftest::{check, check_lohi, Rng, DATA, GAME_GP};
    use ssx3_ee_rt::funcs::GuestFn;

    const SEEDS: u32 = 200;
    /// The zeroed BELibrary dummy and the world singleton.
    const BE: u32 = DATA + 0x1_0000;
    const W: u32 = DATA + 0x1_4000;
    /// The dlmalloc arena and the pool allocator's nodes.
    const MEM_HEAP: u32 = 0x004f_f250;
    /// Tiny translated functions used as virtual-method targets.
    const RET_ZERO: u32 = 0x001a_8828;
    const RET_ONE: u32 = 0x001c_5dd0;
    const RET_TWO: u32 = 0x0014_5854;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }
    fn wh(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }
    fn wb(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }
    fn put_str(c: &mut Cpu, a: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            wb(c, a + i as u32, *b);
        }
        wb(c, a + s.len() as u32, 0);
    }
    /// `{ short delta; void (*fn)(); }` at `vt + off`.
    fn slot(c: &mut Cpu, vt: u32, off: u32, target: u32) {
        wh(c, vt + off, 0);
        w(c, vt + off + 4, target);
    }
    fn u64v(r: &mut Rng) -> u64 {
        ((r.next_u32() as u64) << 32) | r.next_u32() as u64
    }

    /// EAFile's request and file-handle pools (see oc117): with no free entry
    /// the allocators spin forever (`0x3de768`).
    fn eafile_pools(c: &mut Cpu) {
        for (count, table, at, size) in [
            (0x0051_9ab0, 0x0051_9ad8, 0xf_0000, 0x30),
            (0x0051_9ab4, 0x0051_9adc, 0xf_1000, 0x110),
        ] {
            let base = DATA + at;
            w(c, count, 4);
            w(c, table, base);
            for i in 0..size {
                w(c, base + 4 * i, 0);
            }
        }
    }

    fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = 0x0012_3450;
        c.pc = addr;
        f(c);
        c.g[V0][0]
    }

    fn heap_init(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        run(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
    }

    /// The engine's size-class pool allocator, enabled through `gp + 0xdd8`.
    fn pool_allocator(c: &mut Cpu) {
        w(c, GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),
            (0x2a60, 0x2000, 0x10),
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes;
            for n in 0..16u32 {
                let node = base + block * n;
                w(c, node, if n == 15 { 0 } else { node + block });
            }
            w(c, pool + head, base);
            w(c, GAME_GP + slot, pool);
            nodes += block * 16;
        }
    }

    /// A real heap block of `size` bytes.
    fn alloc(c: &mut Cpu, size: u32) -> u32 {
        run(
            c,
            0x0031_7d70,
            ee::s10::f_00317d70,
            &[size as u64, 0x0048_0000, 0, 0],
        ) as u32
    }

    /// Scrambled registers, fake hooks, a game-like `gp`, the BE singleton and
    /// a world with no riders.
    fn base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x1efc), 0);
        w(c, GAME_GP.wrapping_sub(0x848), W);
        let game = W + 0x1000;
        w(c, W + 0x84, game);
        let race = W + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
        r
    }

    /// [`base`] plus the pool allocator and dlmalloc arena.
    fn alloc_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = base(c, seed, salt, calls);
        eafile_pools(c);
        pool_allocator(c);
        heap_init(c);
        r
    }

    // ------------------------------------------------------------------ input map

    #[test]
    fn input_map_finalize_opcodes_matches() {
        check(
            ee::s10::f_00320f10__generated,
            input_map_finalize_opcodes,
            SEEDS,
            &|c, seed| {
                let mut r = alloc_base(c, seed, 0x320f, 4);
                let this = DATA;
                let count = seed % 5;
                let src = alloc(c, 0x40);
                for i in 0..count {
                    w(c, src + 4 * i, r.next_u32());
                }
                w(c, this + 0x10, count);
                w(c, this + 0x14, src);
                let old = if seed & 1 == 0 { 0 } else { alloc(c, 0x40) };
                w(c, this + 8, old);
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn transition_wipe_ctor_matches() {
        check(
            ee::s09::f_002ebba8__generated,
            transition_wipe_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0xebba, 0);
                let this = DATA;
                let arg = DATA + 0x1000;
                for i in 0..4 {
                    w(c, arg + 4 * i, r.next_u32());
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = arg as u64;
                c.g[A2][0] = u64v(&mut r);
                c.g[A3][0] = u64v(&mut r);
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
    fn game_view_man_dtor_matches() {
        check(
            ee::s05::f_0022e730__generated,
            game_view_man_dtor,
            SEEDS,
            &alloc_base_setup(|c, seed, r| {
                let free_self = seed & 1;
                let this = if free_self != 0 { alloc(c, 0x60) } else { DATA };
                let count = seed % 3;
                w(c, this, count);
                for i in 0..count {
                    let entry = if (seed >> (i + 1)) & 1 == 0 {
                        0
                    } else {
                        DATA + 0x800 + 0x40 * i
                    };
                    if entry != 0 {
                        let vt = vtable(c, r, &[8]);
                        w(c, entry + 0x90, vt);
                    }
                    w(c, this + 4 + 4 * i, entry);
                }
                w(c, this + 0x10, r.next_u32());
                c.g[A0][0] = this as u64;
                c.g[A1][0] = free_self as u64;
            }),
        );
    }

    #[test]
    fn fe_net_messenger_show_looking_for_self_matches() {
        check(
            ee::s03::f_001a4d90__generated,
            fe_net_messenger_show_looking_for_self,
            SEEDS,
            &alloc_base_setup(|c, seed, r| {
                let this = DATA;
                w(c, this + 0x10, DATA + 0x800);
                w(
                    c,
                    this + 0x6a0,
                    if seed & 1 == 0 { 0 } else { DATA + 0x1000 },
                );
                c.g[A0][0] = this as u64;
                let _ = r;
            }),
        );
    }

    #[test]
    fn fe_net_messenger_has_name_matches() {
        check(
            ee::s03::f_001a56d0__generated,
            fe_net_messenger_has_name,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x56d0, 2);
                let this = DATA;
                let count = seed % 4;
                let query = DATA + 0x9000;
                let words: [&[u8]; 4] = [b"alpha", b"beta", b"gamma", b"delta"];
                put_str(c, query, words[(seed % 4) as usize]);
                let names = DATA + 0x8000;
                for i in 0..count {
                    let s = DATA + 0x8200 + 0x40 * i;
                    put_str(c, s, words[i as usize]);
                    if seed & 1 == 0 && i == 0 {
                        put_str(c, s, words[(seed % 4) as usize]);
                    }
                    w(c, names + 4 * i, s);
                }
                w(c, this + 0x6e8, names);
                w(c, this + 0x6ec, names + 4 * count);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = query as u64;
            },
        );
    }

    #[test]
    fn fe_net_messenger_find_name_str_matches() {
        check(
            ee::s03::f_001a5800__generated,
            fe_net_messenger_find_name_str,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x5800, 2);
                let this = DATA;
                let count = seed % 4;
                let query = DATA + 0x9000;
                let words: [&[u8]; 4] = [b"alpha", b"beta", b"gamma", b"delta"];
                put_str(c, query, words[(seed % 4) as usize]);
                let names = DATA + 0x8000;
                for i in 0..count {
                    let s = DATA + 0x8200 + 0x40 * i;
                    put_str(c, s, words[i as usize]);
                    if seed & 2 == 0 && i == 0 {
                        put_str(c, s, words[(seed % 4) as usize]);
                    }
                    w(c, names + 4 * i, s);
                }
                w(c, this + 0x6e8, names);
                w(c, this + 0x6ec, names + 4 * count);
                w(c, DATA + 0x9100, query);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x9100) as u64;
            },
        );
    }

    #[test]
    fn net_fe_state_on_keyboard_popup_event_matches() {
        check(
            ee::s03::f_001a94e0__generated,
            net_fe_state_on_keyboard_popup_event,
            SEEDS,
            &alloc_base_setup(|c, seed, r| {
                let this = DATA;
                let sub = DATA + 0x800;
                let vt = vtable(c, r, &[0x100]);
                w(c, this + 8, vt);
                let a2 = [0x15u64, 0x16, 0x17][(seed % 3) as usize];
                c.g[A0][0] = this as u64;
                c.g[A1][0] = sub as u64;
                c.g[A2][0] = a2;
            }),
        );
    }

    #[test]
    fn fe_state_chat_room_transition_in_matches() {
        check(
            ee::s03::f_001b3490__generated,
            fe_state_chat_room_transition_in,
            SEEDS,
            &alloc_base_setup(|c, seed, r| {
                let this = DATA;
                let has_arg = seed & 1;
                for i in 0..2u32 {
                    let entry = DATA + 0x800 + 0x40 * i;
                    let vt = vtable(c, r, &[0x38]);
                    w(c, entry + 8, vt);
                    w(
                        c,
                        this + 0x6fc + 4 * i,
                        if (seed >> i) & 1 == 0 { 0 } else { entry },
                    );
                }
                w(c, this + 0x6ec, 0x100);
                w(c, this + 0x6f4, 0x100);
                let audio = DATA + 0x1000;
                w(c, gp(c, -0xc8), audio);
                w(c, audio + 0x80, DATA + 0x2000);
                put_str(c, DATA + 0x2000, b"player");
                c.g[A0][0] = this as u64;
                c.g[A1][0] = has_arg as u64;
            }),
        );
    }

    #[test]
    fn fe_state_online_agreement_init_matches() {
        check(
            ee::s03::f_001c04a8__generated,
            fe_state_online_agreement_init,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let engine = DATA + 0x800;
                w(c, engine + 4, DATA + 0xa00);
                w(c, DATA + 0xa00 + 0xc, 0x100);
                w(c, this + 0x10, engine);
                w(c, this + 0x40, 0);
                let _ = seed;
                c.g[A0][0] = this as u64;
            }),
        );
    }

    #[test]
    fn fe_state_online_agreement_v25_matches() {
        check(
            ee::s03::f_001c0ea8__generated,
            fe_state_online_agreement_v25,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let args = DATA + 0x800;
                let _ = seed;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = args as u64;
            }),
        );
    }

    // A setup wrapper that also runs `alloc_base`.
    fn alloc_base_setup<'a>(
        f: impl Fn(&mut Cpu, u32, &mut Rng) + 'a,
    ) -> impl Fn(&mut Cpu, u32) + 'a {
        move |c, seed| {
            let mut r = alloc_base(c, seed, 0x1250, 3);
            f(c, seed, &mut r);
        }
    }

    #[test]
    fn lobby_man_destroy_lists_matches() {
        check(
            ee::s06::f_0025a8b0__generated,
            lobby_man_destroy_lists,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                w(c, this + 0x54, DATA + 0x800);
                w(
                    c,
                    this + 0x5c,
                    if seed & 1 == 0 { 0 } else { DATA + 0x1000 },
                );
                w(
                    c,
                    this + 0x60,
                    if seed & 2 == 0 { 0 } else { DATA + 0x1100 },
                );
                w(
                    c,
                    this + 0x64,
                    if seed & 4 == 0 { 0 } else { DATA + 0x1200 },
                );
                if seed & 8 == 0 {
                    w(c, this + 0x58, 0);
                } else {
                    let pending = DATA + 0x1400;
                    w(c, this + 0x58, pending);
                    slot(c, pending, 0xc, FAKE);
                }
                c.g[A0][0] = this as u64;
            }),
        );
    }

    #[test]
    fn buddy_man_destroy_api_matches() {
        check(
            ee::s06::f_00261530__generated,
            buddy_man_destroy_api,
            SEEDS,
            &alloc_base_setup(|c, seed, r| {
                let this = DATA;
                let api = DATA + 0x800;
                w(c, gp(c, -0xc8), api);
                if seed & 1 == 0 {
                    w(c, this + 0xc8, 0);
                } else {
                    let list = DATA + 0x1000;
                    let n = 1 + (seed % 3);
                    w(c, this + 0xbc, list);
                    w(c, this + 0xc0, list + 4 * n);
                    for i in 0..n {
                        let e = DATA + 0x1200 + 0x40 * i;
                        w(c, e, DATA + 0x2000);
                        let vt = vtable(c, r, &[8]);
                        w(c, e + 0x3c, vt);
                        w(c, list + 4 * i, e);
                    }
                    let roster = DATA + 0x1800;
                    w(c, this + 0xc8, roster);
                }
                c.pc = 0x0026_1530;
                c.g[A0][0] = this as u64;
            }),
        );
    }

        #[test]
    fn path_plat_set_paused_matches() {
        check(
            ee::s08::f_002af7a0__generated,
            path_plat_set_paused,
            SEEDS,
            &alloc_base_setup(|c, seed, r| {
                let path = DATA + 0x800;
                let idx = seed % 4;
                w(c, 0x004d_3e98 + 4 * idx, path);
                wh(c, path + 0x3c, r.next_u32() as u16);
                w(c, path + 0x18, r.next_u32());
                wb(c, path + 0x64, (seed & 1) as u8);
                w(c, path + 0x6c, DATA + 0x900);
                w(c, DATA + 0x900 + 4, DATA + 0xa00);
                c.g[A0][0] = idx as u64;
                c.g[A1][0] = (seed & 2) as u64;
            }),
        );
    }

    #[test]
    fn music_player_load_loop_bank_matches() {
        check(
            ee::s08::f_002b43d8__generated,
            music_player_load_loop_bank,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let song = DATA + 0x800;
                let bankman = DATA + 0x1200;
                w(c, this, bankman);
                w(c, bankman + 0xacc, DATA + 0x1300);
                if seed & 1 == 0 {
                    w(c, this + 0x408, 0);
                } else {
                    w(c, this + 0x408, song);
                    w(c, song, DATA + 0x1000);
                    w(c, DATA + 0x1000 + 0x40, if seed & 2 == 0 { 0 } else { 1 });
                    w(c, song + 0x34, DATA + 0x2000);
                    put_str(c, DATA + 0x2000, b"bank01");
                }
                c.g[S3][0] = 0;
                c.g[A0][0] = this as u64;
            }),
        );
    }

    #[test]
    fn replay_flush_save_output_matches() {
        check(
            ee::s07::f_00270fc0__generated,
            replay_flush_save_output,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x0fc0, 2);
                let this = DATA;
                match seed % 4 {
                    0 => w(c, this + 0x644, 0),
                    1 => {
                        w(c, this + 0x644, DATA + 0x1000);
                        w(c, this + 0x648, 0);
                    }
                    2 => {
                        w(c, this + 0x644, DATA + 0x1000);
                        w(c, this + 0x648, 0x100);
                        w(c, this + 0x64c, 0x4000);
                        w(c, this + 0x650, DATA + 0x8000);
                    }
                    _ => {
                        w(c, this + 0x644, DATA + 0x1000);
                        w(c, this + 0x648, 0x3000);
                        w(c, this + 0x64c, 0x100);
                        w(c, this + 0x650, DATA + 0x8000);
                    }
                }
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn replay_save_build_header_matches() {
        check(
            ee::s07::f_00271228__generated,
            replay_save_build_header,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x1228, 3);
                let this = DATA;
                w(c, this + 0x640, DATA + 0x1000);
                w(c, this + 0x18, 0x1234);
                w(c, this + 0x610, 0x5678);
                w(c, this + 0x3c8, 0x9abc);
                w(c, this + 0x3cc, 0xdef0);
                w(c, this + 0x3b0, 0);
                w(c, this + 0x3d0, 0);
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn in_seq_instance_get_length_matches() {
        check(
            ee::s07::f_00274c70__generated,
            in_seq_instance_get_length,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x4c70, 2);
                let this = DATA;
                if seed & 1 == 0 {
                    w(c, this + 0x30, 0x1234);
                } else {
                    w(c, this + 0x30, 0xffff_ffff);
                    w(c, this + 0x10, DATA + 0x1000);
                    wb(c, DATA + 0x1000 + 1, (seed % 3) as u8);
                    w(c, this + 0x18, DATA + 0x1100);
                    for i in 0..3u32 {
                        let ch = DATA + 0x1100 + 0x3c * i;
                        w(c, ch + 0x1c, DATA + 0x1300 + 0x20 * i);
                        w(c, DATA + 0x1300 + 0x20 * i + 4, 0x100 + i);
                    }
                    w(c, this + 0x20, (seed & 2) as u32);
                }
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn chase_cam_controller_push_algorithm_matches() {
        check(
            ee::s01::f_00161e58__generated,
            chase_cam_controller_push_algorithm,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x1e58, 2);
                let this = DATA;
                let list = DATA + 0x1000;
                w(c, this + 8, list);
                w(c, list + 4, (seed & 1) as u32);
                w(c, this + 0x18, 0);
                w(c, DATA + 0x1100 + 0xc, seed);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x1100) as u64;
                c.f[12] = if seed & 2 == 0 { 0 } else { 0x3f80_0000 };
            },
        );
    }

        #[test]
    fn tube_menu_dtor_matches() {
        check(
            ee::s06::f_00250090__generated,
            tube_menu_dtor,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = alloc(c, 0x400);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed & 1) as u64;
            }),
        );
    }

    #[test]
    fn ssx_audio_system_set_sfx_volume_matches() {
        check(
            ee::s07::f_00287488__generated,
            ssx_audio_system_set_sfx_volume,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                w(c, this + 0x62b4, (seed & 1) as u32);
                c.g[A0][0] = (seed % 100) as u64;
                let _ = this;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 101) as u64;
            }),
        );
    }

    #[test]
    fn game_state_prerace_menu_enter_matches() {
        check(
            ee::s06::f_00236bb0__generated,
            game_state_prerace_menu_enter,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let be = DATA + 0x800;
                match seed % 3 {
                    0 => w(c, gp(c, -0x204), 0),
                    1 => {
                        w(c, gp(c, -0x204), be);
                        w(c, be + 4, 0);
                    }
                    _ => {
                        w(c, gp(c, -0x204), be);
                        w(c, be + 4, DATA + 0x900);
                    }
                }
                w(c, W + 0x1000 + 0x64, 0);
                w(c, DATA + 0x900 + 0x64, 0);
                c.g[A0][0] = this as u64;
            }),
        );
    }

    #[test]
    fn snd_bank_man_load_bank_async_matches() {
        check(
            ee::s07::f_0028bd10__generated,
            snd_bank_man_load_bank_async,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                w(c, this + 0xacc, DATA + 0x1000);
                for i in 0..8 {
                    w(c, DATA + 0x1000 + 4 * i, 0);
                }
                c.g[S3][0] = 0;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = (DATA + 0x2100) as u64;
                c.g[A3][0] = seed as u64;
                put_str(c, DATA + 0x2000, b"music");
                put_str(c, DATA + 0x2100, b"bank");
            }),
        );
    }

    #[test]
    fn ssx_audio_system_confirm_music_trigger_matches() {
        check(
            ee::s07::f_0028d740__generated,
            ssx_audio_system_confirm_music_trigger,
            SEEDS,
            &alloc_base_setup(|c, seed, r| {
                let this = DATA;
                let obj = DATA + 0x800;
                let vt = vtable(c, r, &[0x38]);
                w(c, obj + 0x6c0, vt);
                match seed % 3 {
                    0 => {
                        w(c, this + 0x6240, 7);
                        w(c, this + 0x62a0, 7);
                    }
                    1 => {
                        w(c, this + 0x6240, 7);
                        w(c, this + 0x62a0, 0xffff_ffff);
                    }
                    _ => {
                        w(c, this + 0x6240, 7);
                        w(c, this + 0x62a0, 9);
                        w(c, this + 0x623c, (seed & 2) as u32);
                    }
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 7;
                c.g[A2][0] = obj as u64;
            }),
        );
    }

    #[test]
    fn w_script_process_jump_to_task_matches() {
        check(
            ee::s10::f_00307ec0__generated,
            w_script_process_jump_to_task,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let table = DATA + 0x1000;
                let count = seed % 3;
                w(c, this + 4, count);
                w(c, this + 8, (seed & 2) as u32);
                w(c, this + 0x14, table);
                let target = (seed & 1) as u64;
                let hit = if count > 0 && seed & 4 != 0 {
                    (seed % count) as u32
                } else {
                    u32::MAX
                };
                for i in 0..count {
                    let task = table + 0x44 * i;
                    w(c, task + 4, if i == hit { 0x1234 } else { 0x5678 });
                    let obj = DATA + 0x1200 + 0x40 * i;
                    w(c, task + 0x40, obj);
                    wh(c, obj + 0x20, 0);
                    w(c, obj + 0x24, RET_ZERO);
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = target;
            }),
        );
    }

        #[test]
    fn dynamic_spray_fx_reset_matches() {
        check(
            ee::s09::f_002df3b0__generated,
            dynamic_spray_fx_reset,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                w(c, this, DATA + 0x800);
                w(c, DATA + 0x800 + 0x870, (seed & 1) as u32);
                w(c, this + 0x124, (seed & 1) as u32);
                c.g[A0][0] = this as u64;
            }),
        );
    }

    #[test]
    fn be_new_player_int_set_char_profile_flag_matches() {
        check(
            ee::s01::f_00145dd0__generated,
            be_new_player_int_set_char_profile_flag,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x5dd0, 3);
                let player = seed % 4;
                w(c, 0x0053_05b0 + 4 * player, 0);
                c.g[A0][0] = player as u64;
                c.g[A1][0] = (seed & 2) as u64;
                c.g[A2][0] = player as u64;
                c.g[A3][0] = (seed & 1) as u64;
            },
        );
    }

    #[test]
    fn game_rand_shuffle_matches() {
        check_lohi(ee::s06::f_0023c770__generated, game_rand_shuffle, SEEDS, &|c, seed| {
            let _ = base(c, seed, 0x3c770, 2);
            let array = DATA;
            for i in 0..25 {
                w(c, array + 4 * i, (seed + i) as u32);
            }
            c.g[A0][0] = array as u64;
            c.g[A1][0] = 25;
        });
    }

    #[test]
    fn world_trigger_manager_any_trigger_flagged_matches() {
        check(
            ee::s08::f_002b6740__generated,
            world_trigger_manager_any_trigger_flagged,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let sound = DATA + 0x800;
                w(c, W + 0x1000 + 0x4c, sound);
                w(c, DATA + 0xc00 + 0x8c, DATA + 0x900);
                w(c, DATA + 0x900 + 0x10, DATA + 0xa00);
                w(c, DATA + 0xa00, 1);
                w(c, DATA + 0xa00 + 4, 1);
                w(c, this + 0x33c, DATA + 0x1000);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0xc00) as u64;
            }),
        );
    }

    #[test]
    fn data_instance_man_ctor_matches() {
        check(
            ee::s07::f_00275828__generated,
            data_instance_man_ctor,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = (seed % 4) as u64;
            }),
        );
    }

    #[test]
    fn script_list_check_next_ready_matches() {
        check(
            ee::s07::f_002776e0__generated,
            script_list_check_next_ready,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                w(
                    c,
                    this + 0xbc,
                    if seed & 1 == 0 {
                        0xffff_ffff
                    } else {
                        (seed % 5) as u32
                    },
                );
                w(c, this + 0x8c, 0);
                w(c, this + 0x90, 0);
                w(c, this, DATA + 0x1100);
                let cmd = DATA + 0x1200;
                for i in 0..8 {
                    w(c, cmd + 4 * i, if i == 0 { 0x10 } else { 0 });
                }
                w(c, this + 0xc, cmd);
                c.g[A0][0] = this as u64;
            }),
        );
    }

    #[test]
    fn bolt_on_fx_interface_dtor_matches() {
        check(
            ee::s09::f_002ecb28__generated,
            bolt_on_fx_interface_dtor,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let free_self = seed & 1;
                let this = if free_self != 0 {
                    alloc(c, 0x200)
                } else {
                    DATA
                };
                w(c, this + 0xc, DATA + 0x800);
                for i in 0..15u32 {
                    w(
                        c,
                        this + 0x10 + 0xc * i,
                        if (seed >> (i % 8)) & 1 == 0 {
                            0xffff_ffff
                        } else {
                            0x1
                        },
                    );
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = free_self as u64;
            }),
        );
    }

    #[test]
    fn world_cache_streamer_are_node_hulls_free_matches() {
        check(
            ee::s13::f_003a95c0__generated,
            world_cache_streamer_are_node_hulls_free,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x95c0, 2);
                let world = DATA;
                let flag = DATA + 0x900;
                let n0 = DATA + 0x800;
                let n1 = DATA + 0x880;
                w(c, flag, (seed & 1) as u32);
                w(c, n0 + 0x58, if seed & 2 == 0 { 0 } else { DATA + 0x980 });
                w(c, DATA + 0x980 + 2, 0);
                w(c, n0 + 0x50, (seed & 4) as u32);
                w(c, n0 + 0x54, if seed & 8 == 0 { 0 } else { n1 });
                w(c, n1 + 0x54, 0);
                c.g[A0][0] = world as u64;
                c.g[A1][0] = n0 as u64;
                c.g[A2][0] = flag as u64;
            },
        );
    }

    #[test]
    fn ui_text_scroll_clear_lines_matches() {
        check(
            ee::s13::f_003a18c0__generated,
            ui_text_scroll_clear_lines,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let count = seed % 4;
                if seed & 1 == 0 {
                    w(c, this + 0xbc, 0);
                } else {
                    let array = alloc(c, 0x40);
                    w(c, this + 0xbc, array);
                    wh(c, this + 0xc8, count as u16);
                    for i in 0..count {
                        let entry = if seed & 2 == 0 { 0 } else { alloc(c, 0x40) };
                        w(c, array + 4 * i, entry);
                    }
                }
                c.g[A0][0] = this as u64;
            }),
        );
    }

    #[test]
    fn rail_man_ctor_matches() {
        check(
            ee::s11::f_003584f0__generated,
            rail_man_ctor,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let _ = seed;
                c.g[A0][0] = this as u64;
            }),
        );
    }

    #[test]
    fn vfx_avalanche_dtor_matches() {
        check(
            ee::s09::f_002d6378__generated,
            vfx_avalanche_dtor,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let free_self = seed & 1;
                let this = if free_self != 0 { alloc(c, 0x80) } else { DATA };
                c.g[A0][0] = this as u64;
                c.g[A1][0] = free_self as u64;
            }),
        );
    }

    #[test]
    fn mdf_file_create_model_by_name_matches() {
        check(
            ee::s10::f_00314c00__generated,
            mdf_file_create_model_by_name,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let records = DATA + 0x1000;
                let names = DATA + 0x1100;
                let count = seed % 3;
                w(c, this + 4, records);
                wh(c, records + 4, count as u16);
                w(c, this + 8, names);
                let query = DATA + 0x9000;
                let words: [&[u8]; 3] = [b"alpha", b"beta", b"gamma"];
                put_str(c, query, words[(seed % 3) as usize]);
                for i in 0..count {
                    let s = names + 0x60 * i;
                    put_str(c, s, words[i as usize]);
                    if seed & 1 == 0 && i == 0 {
                        put_str(c, s, words[(seed % 3) as usize]);
                    }
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = query as u64;
            }),
        );
    }

        #[test]
    fn fe_state_mem_card_v33_matches() {
        check(
            ee::s02::f_001874e8__generated,
            fe_state_mem_card_v33,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                w(c, this + 0x1ac, (seed & 1) as u32);
                w(c, gp(c, -0x1c34), (seed % 6) as u32);
                w(c, this + 0x238, (seed & 2) as u32);
                c.g[A0][0] = this as u64;
            }),
        );
    }

        #[test]
    fn load_indicator_init_matches() {
        check(
            ee::s04::f_001f30d8__generated,
            load_indicator_init,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let engine = DATA + 0x800;
                let elem = DATA + 0x900;
                w(c, W + 0x8c, engine);
                w(c, engine + 4, elem);
                wh(c, elem + 0x20, 0);
                w(c, elem + 0x24, RET_ONE);
                w(c, W + 0x1000 + 0x48, DATA + 0xa00);
                w(c, DATA + 0xa00 + 8, 0);
                c.g[A0][0] = this as u64;
                let _ = seed;
            }),
        );
    }

    #[test]
    fn ui_state_mgr_close_unpinned_popups_matches() {
        check(
            ee::s13::f_0039f600__generated,
            ui_state_mgr_close_unpinned_popups,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0xf600, 2);
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
    fn transition_effect_draw_fade_matches() {
        check(
            ee::s09::f_002ebb10__generated,
            transition_effect_draw_fade,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0xbb10, 2);
                let this = DATA;
                for i in 0..4 {
                    w(c, this + 4 + 4 * i, r.next_u32());
                }
                c.g[A0][0] = this as u64;
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn be_prepare_net_profiles_matches() {
        check(
            ee::s01::f_0014e9f8__generated,
            be_prepare_net_profiles,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0xe9f8, 3);
                let this = DATA;
                w(c, 0x0053_4b38, (seed & 1) as u32);
                w(c, 0x0053_5b3c - 0x1c, 0);
                w(c, 0x0053_5b3c - 4, 0);
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn be_set_char_state_flag_cheat_matches() {
        check_lohi(
            ee::s01::f_00150b88__generated,
            be_set_char_state_flag_cheat,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x0b88, 1);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A3][0] = (seed / 4 % 4) as u64;
                c.g[T0][0] = (seed & 1) as u64;
            },
        );
    }

    #[test]
    fn world_state_read_objects_matches() {
        check(
            ee::s07::f_0026ddc0__generated,
            world_state_read_objects,
            SEEDS,
            &alloc_base_setup(|c, seed, r| {
                let this = DATA;
                let obj = DATA + 0x800;
                let vt = DATA + 0x900;
                w(c, obj, vt);
                slot(c, vt, 0x10, FAKE);
                w(
                    c,
                    0x01f0_0000,
                    if seed & 1 == 0 { 0 } else { 1 + (seed % 2) },
                );
                w(c, this, 0);
                let game = DATA + 0xa00;
                w(c, this + 0x84, game);
                let _ = r;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0xb00) as u64;
                c.g[A2][0] = obj as u64;
            }),
        );
    }

    #[test]
    fn mc_manager_state_24_matches() {
        check(
            ee::s06::f_0023fc70__generated,
            mc_manager_state_24,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let obj = DATA + 0x800;
                let vt = DATA + 0x900;
                w(c, obj, vt);
                slot(c, vt, 0x58, if seed & 1 == 0 { RET_ZERO } else { RET_ONE });
                w(c, this + 0x434, obj);
                w(
                    c,
                    this + 0x6c,
                    if seed & 2 == 0 { RET_ZERO } else { RET_ONE },
                );
                w(
                    c,
                    this + 0x84,
                    if seed & 4 == 0 { RET_ZERO } else { RET_ONE },
                );
                let result = DATA + 0xa00;
                slot(c, result, 8, RET_ZERO);
                w(c, this + 0x748, result);
                c.g[A0][0] = this as u64;
            }),
        );
    }
}
