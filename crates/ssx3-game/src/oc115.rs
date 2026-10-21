//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for 50 small cold-path functions, each with a differential
//! test against its generated body.
//!
//! The functions fall into a few families:
//!
//! - constructor/destructor prologues that install a vtable and hand off to a
//!   base constructor (`UIState_ctor_2`, `MemCardStateBase_ctor`, `DeadNode_ctor`,
//!   `Hide_ctor`, `cFEStateStats_ctor`, `NetEventE5_Clone`, ...);
//! - `Save` methods that serialise an object through its own vtable
//!   (`FloatRail_Save`, `SpringRail_Save`, `Floating_Save`);
//! - UI screen code: `Render` methods that bounce through `CAppModule` slots,
//!   `OnActivate`/`OnWidgetAction` handlers that forward to a widget's vtable
//!   method once a hash lookup finds it, and `Init` methods that build a
//!   child widget;
//! - list walks with back-edge event checks (`PadQueue_Shutdown`,
//!   `UIFontSet_FreeFonts`, `UIList_DeleteAll`, `MultiParticle_RemoveSectionInstances`,
//!   `cFEStateLobbyMenu_ShowRoomTab`, `NetFEState_MakePasswordMask`,
//!   `cFEStateStats_ctor`, `IniParser_SkipToEol`) and the one dispatcher-shaped
//!   worker loop (`PS2ExecMan_WorkerThread`);
//! - string/parser/audio helpers (`cBXString_*`, `IniParser_*`,
//!   `SSXAudioSystem_StopRiderLoops`, `CrowdAudio_AddSampleFile`).
//!
//! Every port keeps the generated body's cycle charges at the same points, its
//! exact register writes (including v0/v1 scratch values a caller may read) and
//! its call order. Calls use the same protocol as the original: a `Callee` for
//! the intra-shard `jal` the generator wrote as a direct call, `call_ptr` for
//! the cross-shard/virtual calls it wrote as `call_indirect`.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_e318, ui_state_ctor_2, "UIState_ctor_2"),
    (0x0032_6a68, pad_queue_shutdown, "PadQueue_Shutdown"),
    (
        0x0031_8040,
        cbx_string_alloc_buffer,
        "cBXString_AllocBuffer",
    ),
    (0x0031_90a8, cbx_string_left, "cBXString_Left"),
    (
        0x002f_0438,
        ps2ai_visual_effects_man_dtor,
        "PS2AIVisualEffectsMan_dtor",
    ),
    (
        0x001d_53b0,
        mem_card_state_base_ctor,
        "MemCardStateBase_ctor",
    ),
    (
        0x0026_cd20,
        playback_man_stop_playback,
        "PlaybackMan_StopPlayback",
    ),
    (0x003a_abd8, cps2_chunk_resolve_v20, "cPS2ChunkResolve_v20"),
    (0x0023_2da8, start_screen_render, "StartScreen_Render"),
    (
        0x0023_3390,
        c_game_load_screen_render,
        "cGameLoadScreen_Render",
    ),
    (0x0039_8438, ui_font_set_free_fonts, "UIFontSet_FreeFonts"),
    (
        0x0031_ac60,
        ps2_exec_man_worker_thread,
        "PS2ExecMan_WorkerThread",
    ),
    (
        0x0035_3020,
        modifier_block_forward_v43,
        "ModifierBlock_ForwardV43",
    ),
    (
        0x001f_5650,
        c_fe_state_career_stats_on_widget_action,
        "cFEStateCareerStats_OnWidgetAction",
    ),
    (0x0034_9ad0, float_rail_save, "FloatRail_Save"),
    (0x0034_ac10, spring_rail_save, "SpringRail_Save"),
    (0x0023_39f8, c_fe_load_screen_render, "cFELoadScreen_Render"),
    (0x0035_9eb8, spline_modifier_v05, "SplineModifier_v05"),
    (
        0x0035_8140,
        multi_particle_remove_section_instances,
        "MultiParticle_RemoveSectionInstances",
    ),
    (
        0x0020_8b00,
        ov_screen_session_on_activate,
        "OVScreenSession_OnActivate",
    ),
    (0x0039_c1c8, ui_pair_update_anim, "UIPair_UpdateAnim"),
    (
        0x0036_7150,
        tex_man_alloc_nb_tex_info,
        "TexMan_AllocNBTexInfo",
    ),
    (
        0x001f_7198,
        ov_screen_enter_lodge_init,
        "OVScreenEnterLodge_Init",
    ),
    (0x0021_22e8, ov_screen_autosave_v26, "OVScreenAutosave_v26"),
    (0x001a_2190, fe_module_unload, "FEModule_Unload"),
    (
        0x0020_cbe8,
        game_ui_can_open_pause_menu,
        "GameUI_CanOpenPauseMenu",
    ),
    (
        0x0020_d4f0,
        ov_screen_yn_dialog_on_widget_created,
        "OVScreenYNDialog_OnWidgetCreated",
    ),
    (
        0x0023_05c8,
        game_module_end_replay_playback,
        "GameModule_EndReplayPlayback",
    ),
    (
        0x001e_9130,
        ov_screen_hiscore_list_on_widget_action,
        "OVScreenHiscoreList_OnWidgetAction",
    ),
    (
        0x0035_2708,
        mesh_anim_kill_restorable,
        "MeshAnim_KillRestorable",
    ),(
        0x0014_ea90,
        be_reset_net_interfaces,
        "BE_ResetNetInterfaces",
    ),
    (
        0x0015_5190,
        be_score_int_get_high_score_owner,
        "BEScoreInt_GetHighScoreOwner",
    ),
    (0x0035_06d8, dead_node_ctor, "DeadNode_ctor"),
    (0x0035_0e90, hide_ctor, "Hide_ctor"),
    (0x0039_7948, ui_list_delete_all, "UIList_DeleteAll"),
    (0x0034_7f90, flex_rail_dtor, "FlexRail_dtor"),
    (0x0034_fa88, floating_save, "Floating_Save"),
    (
        0x001a_98b8,
        net_fe_state_make_password_mask,
        "NetFEState_MakePasswordMask",
    ),
    (
        0x001b_00b8,
        c_fe_state_challenge_received_popup_v32,
        "cFEStateChallengeReceivedPopup_v32",
    ),
    (
        0x001b_cc30,
        c_fe_state_lobby_menu_init,
        "cFEStateLobbyMenu_Init",
    ),
    (
        0x001b_df60,
        c_fe_state_lobby_menu_show_room_tab,
        "cFEStateLobbyMenu_ShowRoomTab",
    ),
    (
        0x001c_0fc0,
        c_fe_state_net_quick_play_popup_init,
        "cFEStateNetQuickPlayPopup_Init",
    ),
    (0x001c_2d18, c_fe_state_stats_ctor, "cFEStateStats_ctor"),
    (
        0x001e_03f8,
        c_fe_popup_select_multiplayer_mode_on_widget_action,
        "cFEPopupSelectMultiplayerMode_OnWidgetAction",
    ),
    (0x0026_85e0, net_event_e5_clone, "NetEventE5_Clone"),
    (
        0x0029_ed18,
        ssx_audio_system_stop_rider_loops,
        "SSXAudioSystem_StopRiderLoops",
    ),(0x002a_7a90, ini_parser_skip_to_eol, "IniParser_SkipToEol"),
    (0x002a_7ff8, ini_parser_get_string, "IniParser_GetString"),
];

// ---------------------------------------------------------------------------
// Helpers.

/// The low 32 bits of a register (an object pointer or a 32-bit value).
#[inline]
fn low(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// `lw off(reg)`.
#[inline]
fn lw_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lw(c)
}

/// `lh off(reg)` (sign-extended).
#[inline]
fn lh_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lh(c)
}

/// `lhu off(reg)`.
#[inline]
fn lhu_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lhu(c)
}

/// `lb off(reg)` (sign-extended).
#[inline]
fn lb_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lb(c)
}

/// `lbu off(reg)`.
#[inline]
fn lbu_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lbu(c)
}

/// `sw off(reg)`.
#[inline]
fn sw_r(c: &mut Cpu, reg: usize, off: u32, v: u64) {
    let a = low(c, reg).wrapping_add(off);
    c.sw(a, v);
}

/// `sh off(reg)`.
#[inline]
fn sh_r(c: &mut Cpu, reg: usize, off: u32, v: u64) {
    let a = low(c, reg).wrapping_add(off);
    c.sh(a, v);
}

/// `sb off(reg)`.
#[inline]
fn sb_r(c: &mut Cpu, reg: usize, off: u32, v: u64) {
    let a = low(c, reg).wrapping_add(off);
    c.sb(a, v);
}

/// `lwc1 f, off(reg)`.
#[inline]
fn lwc1_r(c: &mut Cpu, f: usize, reg: usize, off: u32) {
    let a = low(c, reg).wrapping_add(off);
    c.lwc1(f, a);
}

/// `swc1 f, off(reg)`.
#[inline]
fn swc1_r(c: &mut Cpu, f: usize, reg: usize, off: u32) {
    let a = low(c, reg).wrapping_add(off);
    c.swc1(f, a);
}

// ---------------------------------------------------------------------------
// Intra-shard callees the generator called directly (its `jal` aliases).

mod callee {
    use super::{ee, Callee};

    pub const UI_LIST_CTOR: Callee = Callee {
        addr: 0x0039_77e8,
        f: ee::s13::f_003977e8,
    };
    pub const UI_LIST_RESET: Callee = Callee {
        addr: 0x0039_7930,
        f: ee::s13::f_00397930,
    };
    pub const OPERATOR_NEW_NAMED: Callee = Callee {
        addr: 0x0031_7e30,
        f: ee::s10::f_00317e30,
    };
    pub const CBX_STRING_ALLOC_COPY: Callee = Callee {
        addr: 0x0031_81e8,
        f: ee::s10::f_003181e8,
    };
    pub const CBX_STRING_CTOR_COPY: Callee = Callee {
        addr: 0x0031_7fe8,
        f: ee::s10::f_00317fe8,
    };
    pub const CBX_STRING_DTOR: Callee = Callee {
        addr: 0x0031_8260,
        f: ee::s10::f_00318260,
    };
    pub const MEMCARD_INSTALL_CALLBACKS: Callee = Callee {
        addr: 0x001d_5488,
        f: ee::s04::f_001d5488,
    };
    pub const REPLAY_SET_STATE: Callee = Callee {
        addr: 0x0026_f4a0,
        f: ee::s07::f_0026f4a0,
    };
    pub const PLAYBACK_RESTORE_WORLD: Callee = Callee {
        addr: 0x0026_cc18,
        f: ee::s07::f_0026cc18,
    };
    pub const WORLD_CHUNK_RESOLVE_HOOK: Callee = Callee {
        addr: 0x003a_d230,
        f: ee::s13::f_003ad230,
    };
    pub const APP_MODULE_RENDER_FADE: Callee = Callee {
        addr: 0x0023_1d60,
        f: ee::s06::f_00231d60,
    };
    pub const FLEX_RAIL_SAVE: Callee = Callee {
        addr: 0x0034_8b40,
        f: ee::s11::f_00348b40,
    };
    pub const SPLINE_MODIFIER_SET_SPEED: Callee = Callee {
        addr: 0x0035_9688,
        f: ee::s11::f_00359688,
    };
    pub const AUTOSAVE_UPDATE_ICON: Callee = Callee {
        addr: 0x0021_2360,
        f: ee::s05::f_00212360,
    };
    pub const FE_UNLOAD_ANIM_DATA: Callee = Callee {
        addr: 0x001a_2bc0,
        f: ee::s03::f_001a2bc0,
    };
    pub const FE_UNLOAD_RIDER_PREVIEWS: Callee = Callee {
        addr: 0x001a_04c8,
        f: ee::s03::f_001a04c8,
    };
    pub const BOLT_FILE_CACHE_SHUTDOWN: Callee = Callee {
        addr: 0x0019_cf40,
        f: ee::s03::f_0019cf40,
    };
    pub const WORLD_OBJ_CTOR: Callee = Callee {
        addr: 0x0034_fb00,
        f: ee::s11::f_0034fb00,
    };
    pub const WORLD_OBJ_DTOR: Callee = Callee {
        addr: 0x0034_fbf0,
        f: ee::s11::f_0034fbf0,
    };
    pub const WORLD_OBJ_RUN_KILL_SCRIPT: Callee = Callee {
        addr: 0x0034_fcc0,
        f: ee::s11::f_0034fcc0,
    };
    pub const WORLD_OBJ_REPLACE_WITH_DEAD_NODE: Callee = Callee {
        addr: 0x0034_fe28,
        f: ee::s11::f_0034fe28,
    };
    pub const MOD_OBJ_SAVE: Callee = Callee {
        addr: 0x0035_6b30,
        f: ee::s11::f_00356b30,
    };
    pub const FE_LOCALE_LOAD_TABLE: Callee = Callee {
        addr: 0x0019_59a8,
        f: ee::s02::f_001959a8,
    };
    pub const BE_LIBRARY_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };
    pub const BE_LIBRARY_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::f_0014dd58,
    };
    pub const BE_SCORE_INT_GET_HIGH_SCORE_TABLE: Callee = Callee {
        addr: 0x0014_ab20,
        f: ee::s01::f_0014ab20,
    };
    pub const BE_OPTION_INT_SWAP_PLAYER_FLAGS: Callee = Callee {
        addr: 0x0014_eb08,
        f: ee::s01::f_0014eb08,
    };
    pub const NET_FE_STATE_CTOR: Callee = Callee {
        addr: 0x001a_8500,
        f: ee::s03::f_001a8500,
    };
    pub const UI_OBJECT_UPDATE_ANIM: Callee = Callee {
        addr: 0x003a_0000,
        f: ee::s13::f_003a0000,
    };
    pub const INI_PARSER_GET_CHAR: Callee = Callee {
        addr: 0x002a_78e0,
        f: ee::s08::f_002a78e0,
    };
    pub const INI_PARSER_EXPECT: Callee = Callee {
        addr: 0x002a_7ef0,
        f: ee::s08::f_002a7ef0,
    };
    pub const INI_PARSER_READ_QUOTED_STRING: Callee = Callee {
        addr: 0x002a_7cf0,
        f: ee::s08::f_002a7cf0,
    };
    pub const AUDIO_STOP_WIPEOUT_LOOP: Callee = Callee {
        addr: 0x0029_6e20,
        f: ee::s08::f_00296e20,
    };
    pub const AUDIO_STOP_WIPEOUT_LOOP2: Callee = Callee {
        addr: 0x0029_7438,
        f: ee::s08::f_00297438,
    };
    pub const AUDIO_STOP_TRICK_METER: Callee = Callee {
        addr: 0x0029_b3c0,
        f: ee::s08::f_0029b3c0,
    };
    pub const BANK_MONITOR_STOP_HANDLE: Callee = Callee {
        addr: 0x002a_d5f0,
        f: ee::s08::f_002ad5f0,
    };
    pub const LOBBY_MENU_HIDE_ROOM_TABS: Callee = Callee {
        addr: 0x001b_dfd8,
        f: ee::s03::f_001bdfd8,
    };
}

// ---------------------------------------------------------------------------
// Constructors.

/// `func_0039E318` (0x39e318): the second half of a UI state's constructor.
/// Installs vtable `0x4946c8` at `+8`, the id byte `0x0f` at `+0x15`, clears
/// the flag words at `+0xc`/`+0x40` and constructs the widget list at `+0x24`
/// (`UIList_ctor`). Signature: `void ctor(this, owner, engine)`.
pub fn ui_state_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = 0xf;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = u64::MAX;
        sb_r(c, S0, 0x14, c.g[S1][0]);
        c.g[V1][0] = 0x218;
        sw_r(c, S0, 0x4, c.g[S0][0]);
        c.g[V0][0] = sx(low(c, V0).wrapping_add(0x46c8));
        sw_r(c, S0, 0x0, c.g[S0][0]);
        c.g[A0][0] = sx(low(c, S0).wrapping_add(0x24));
        sw_r(c, S0, 0xc, 0);
        sw_r(c, S0, 0x10, c.g[A1][0]);
        sb_r(c, S0, 0x15, c.g[A3][0]);
        sh_r(c, S0, 0x1c, c.g[V1][0]);
        sw_r(c, S0, 0x20, c.g[A2][0]);
        c.g[RA][0] = sx(0x0039_e36c);
        sw_r(c, S0, 0x8, c.g[V0][0]);
        callee::UI_LIST_CTOR.call(c, 0x0039_e36c)?;
        sb_r(c, S0, 0x44, c.g[S1][0]);
        c.g[V0][0] = c.g[S0][0];
        sw_r(c, S0, 0x40, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_001D53B0` (0x1d53b0): the memory-card state constructor. Runs the UI
/// state constructor (`0x39e2a0`), installs vtables `0x474e08`/`0x46c980`,
/// clears the callback/word fields at `+0x48`..`+0x98` and calls
/// `MemCardStateBase_InstallCallbacks`. Signature: `void ctor(this, a, b)`.
pub fn mem_card_state_base_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x001d_53cc);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x001d_53cc)?;
        c.g[V1][0] = 0x47_0000;
        sw_r(c, S0, 0x98, 0);
        c.g[V1][0] = sx(low(c, V1).wrapping_add(0x4e08));
        sw_r(c, S0, 0x48, 0);
        sw_r(c, S0, 0x8, c.g[V1][0]);
        c.g[A2][0] = 0x4a_0000;
        sw_r(c, S0, 0x4c, 0);
        c.g[V0][0] = 0x47_0000;
        sw_r(c, S0, 0x54, 0);
        c.g[V0][0] = sx(low(c, V0).wrapping_sub(0x3680));
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = lhu_r(c, A2, 0x1390);
        sw_r(c, S0, 0x8, c.g[V0][0]);
        c.g[RA][0] = sx(0x001d_540c);
        sh_r(c, S0, 0x58, c.g[V1][0]);
        callee::MEMCARD_INSTALL_CALLBACKS.call(c, 0x001d_540c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_003506D8` (0x3506d8): the dead-node object constructor. Runs the
/// world object constructor (`WorldObj_ctor` with kind 8 / flags 6), installs
/// vtable `0x491b00` at `+0xc` and clears bits `0x60`/`0x2` of the node flags
/// word at `+0x8` of both the node and its script object while setting bit 2
/// (`0x4`) on the script object. Signature: `void ctor(this, a, b)`.
pub fn dead_node_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0x8;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0035_06f8);
        c.g[A2][0] = 0x6;
        callee::WORLD_OBJ_CTOR.call(c, 0x0035_06f8)?;
        c.g[V1][0] = 0x49_0000;
        c.g[A2][0] = lw_r(c, S0, 0x18);
        c.g[V1][0] = sx(low(c, V1).wrapping_add(0x1b00));
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        sw_r(c, S0, 0xc, c.g[V1][0]);
        c.g[A0][0] = c.g[A0][0] | 0xff9f;
        c.g[A1][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = lw_r(c, A2, 0x8);
        c.g[A1][0] = c.g[A1][0] | 0xfffd;
        c.g[RA][0] = c.ld(low(c, SP));
        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
        sw_r(c, A2, 0x8, c.g[V1][0]);
        c.g[A0][0] = lw_r(c, S0, 0x18);
        frame.restore(c, S0, 0x10);
        c.g[V1][0] = lw_r(c, A0, 0x8);
        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
        c.g[V1][0] = c.g[V1][0] | 0x4;
        sw_r(c, A0, 0x8, c.g[V1][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `func_00350E90` (0x350e90): the hide-object constructor. Same shape as the
/// dead node's, with `WorldObj_ctor` kind 1 / flags 0x10 and vtable `0x491800`.
/// Signature: `void ctor(this, a, b)`.
pub fn hide_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0x1;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0035_0eb0);
        c.g[A2][0] = 0x10;
        callee::WORLD_OBJ_CTOR.call(c, 0x0035_0eb0)?;
        c.g[V1][0] = 0x49_0000;
        c.g[A2][0] = lw_r(c, S0, 0x18);
        c.g[V1][0] = sx(low(c, V1).wrapping_add(0x1800));
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        sw_r(c, S0, 0xc, c.g[V1][0]);
        c.g[A0][0] = c.g[A0][0] | 0xff9f;
        c.g[A1][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = lw_r(c, A2, 0x8);
        c.g[A1][0] = c.g[A1][0] | 0xfffd;
        c.g[RA][0] = c.ld(low(c, SP));
        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
        sw_r(c, A2, 0x8, c.g[V1][0]);
        c.g[A0][0] = lw_r(c, S0, 0x18);
        frame.restore(c, S0, 0x10);
        c.g[V1][0] = lw_r(c, A0, 0x8);
        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
        c.g[V1][0] = c.g[V1][0] | 0x4;
        sw_r(c, A0, 0x8, c.g[V1][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `func_001C2D18` (0x1c2d18): the network statistics popup constructor. Runs
/// `NetFEState_ctor`, installs vtable `0x468278`, sets `+0xc = 0x42` and
/// `+0x6ec = 1`, clears four words at `+0x6e0` and then the four-slot array at
/// `+0x6dc` (a `for (i = 3; i >= 0; i--)` countdown loop with an event check).
/// Signature: `void ctor(this)`.
pub fn c_fe_state_stats_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x001c_2d30);
        c.g[S0][0] = c.g[A0][0];
        callee::NET_FE_STATE_CTOR.call(c, 0x001c_2d30)?;
        c.g[V1][0] = 0x47_0000;
        c.g[V0][0] = 0x1;
        c.g[V1][0] = sx(low(c, V1).wrapping_sub(0x7d88));
        c.g[A0][0] = 0x42;
        sw_r(c, S0, 0x8, c.g[V1][0]);
        sw_r(c, S0, 0x6ec, c.g[V0][0]);
        c.g[V1][0] = 0x3;
        sw_r(c, S0, 0xc, c.g[A0][0]);
        c.g[V0][0] = sx(low(c, S0).wrapping_add(0x6dc));
        sw_r(c, S0, 0x6e0, 0);
        sw_r(c, S0, 0x6e4, 0);
        sw_r(c, S0, 0x6e8, 0);
        loop {
            c.cycles += 7;
            sw_r(c, V0, 0x0, 0);
            c.g[V1][0] = sx(low(c, V1).wrapping_sub(0x1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(low(c, V0).wrapping_sub(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001c_2d60);
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `func_002685E0` (0x2685e0): `NetEventE5_Clone`. Allocates a 0x10-byte copy
/// with `operator new(0x10, tag=0x480488, 0x20000000, 0)`, sets the event id
/// word to 0xe5, vtable `0x481170` at `+4`, copies the string at `+8` and the
/// payload word at `+0xc`. Signature: `NetEventE5* clone(NetEventE5* self)`.
pub fn net_event_e5_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0x48_0000;
        frame.save(c, S2, 0x10);
        c.g[A2][0] = 0x2000_0000;
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(low(c, A1).wrapping_add(0x488));
        c.g[RA][0] = sx(0x0026_8610);
        c.g[A0][0] = 0x10;
        // the original reaches `EA_MemAllocNamed` directly, not the
        // `operator new` wrapper at 0x317e30
        call_ptr(c, 0x0031_7d70, 0x0026_8610)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = 0x48_0000;
        c.g[S1][0] = lw_r(c, S2, 0xc);
        c.g[V1][0] = sx(low(c, V1).wrapping_add(0x1170));
        c.g[A1][0] = lw_r(c, S2, 0x8);
        c.g[V0][0] = 0xe5;
        sw_r(c, S0, 0x4, c.g[V1][0]);
        c.g[A0][0] = sx(low(c, S0).wrapping_add(0x8));
        c.g[RA][0] = sx(0x0026_8638);
        sw_r(c, S0, 0x0, c.g[V0][0]);
        call_ptr(c, 0x0031_82d8, 0x0026_8638)?;
        sw_r(c, S0, 0xc, c.g[S1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
// ---------------------------------------------------------------------------
// List walks and buffer management.

/// `func_00326A68` (0x326a68): shut down the pad queue. For each of the
/// `count` entries at `this+0x2eec` (a null entry is skipped) call its virtual
/// destructor (`vtable[8]`, flag 3). The loop's back edge carries an event
/// check. Signature: `void shutdown(PadQueue* this)`.
pub fn pad_queue_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_r(c, S2, 0x2ee8);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[S0][0] = sx(low(c, S2).wrapping_add(0x2eec));
            loop {
                c.cycles += 3;
                c.g[A2][0] = lw_r(c, S0, 0);
                let skip = c.g[A2][0] == 0;
                c.g[A1][0] = 0x3;
                if !skip {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(low(c, A2)).lw(c);
                    c.g[A0][0] = lh_r(c, V0, 0x8);
                    c.g[V1][0] = lw_r(c, V0, 0xc);
                    let target = low(c, V1);
                    c.g[RA][0] = sx(0x0032_6ab0);
                    c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
                    call_ptr(c, target, 0x0032_6ab0)?;
                }
                c.cycles += 5;
                c.g[V0][0] = lw_r(c, S2, 0x2ee8);
                c.g[S1][0] = sx(low(c, S1).wrapping_add(1));
                let more = (c.g[S1][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[S0][0] = sx(low(c, S0).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_6a90);
                }
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

/// `cBXString_Realloc` (0x318040): `cBXString_AllocBuffer`. A zero size just
/// points the string at the shared empty buffer (`D_004A3E90`); otherwise
/// allocate `size + 0xd` bytes with `operator new(size, D_0048D850,
/// 0x20000000, 0)`, write the three-word header (refcount 1, size, capacity)
/// in front of the characters and terminate the string.
pub fn cbx_string_alloc_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        if c.g[S0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = lw_r(c, GP, 0xda0);
            sw_r(c, S1, 0, c.g[V0][0]);
        } else {
            c.cycles += 14;
            c.g[A1][0] = 0x49_0000;
            c.g[A0][0] = sx(low(c, S0).wrapping_add(0xd));
            c.g[A1][0] = sx(low(c, A1).wrapping_sub(0x27b0));
            c.g[A2][0] = 0x2000_0000;
            c.g[RA][0] = sx(0x0031_8080);
            c.g[A3][0] = 0;
            callee::OPERATOR_NEW_NAMED.call(c, 0x0031_8080)?;
            c.g[V1][0] = 0x1;
            c.g[A0][0] = sx(low(c, V0).wrapping_add(0xc));
            sw_r(c, V0, 0, c.g[V1][0]);
            c.g[A1][0] = sx(low(c, A0).wrapping_add(low(c, S0)));
            sw_r(c, S1, 0, c.g[A0][0]);
            sb_r(c, A1, 0, 0);
            sw_r(c, V0, 0x8, c.g[S0][0]);
            sw_r(c, V0, 0x4, c.g[S0][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString_Left` (0x3190a8): the first `count` characters of `src` (a
/// negative count means "all"; a longer count is clamped to the source
/// length stored in front of its characters, `*(int*)(*(char**)src - 8)`).
/// The copy is built in a stack temporary and moved into `dst`.
/// Signature: `cBXString* left(cBXString* dst, cBXString* src, int count)`.
pub fn cbx_string_left(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        if (c.g[A2][0] as i64) >= 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(low(c, A1)).lw(c);
            c.g[A0][0] = Addr(low(c, V0).wrapping_sub(8)).lw(c);
            let shorter = (c.g[A0][0] as i64) < (c.g[A2][0] as i64);
            c.g[V1][0] = shorter as u64;
            if shorter {
                c.g[A2][0] = c.g[A0][0];
            }
        } else {
            c.cycles += 2;
            c.g[A2][0] = 0;
        }
        c.cycles += 18;
        c.g[V0][0] = lw_r(c, GP, 0xda0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A3][0] = 0;
        sw_r(c, SP, 0, c.g[V0][0]);
        c.g[RA][0] = sx(0x0031_90f0);
        c.g[T0][0] = 0;
        callee::CBX_STRING_ALLOC_COPY.call(c, 0x0031_90f0)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x0031_90fc);
        c.g[A0][0] = c.g[S0][0];
        callee::CBX_STRING_CTOR_COPY.call(c, 0x0031_90fc)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x0031_9108);
        c.g[A1][0] = 0x2;
        callee::CBX_STRING_DTOR.call(c, 0x0031_9108)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        frame.restore(c, S0, 0x20);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Destructors / save methods.

/// `func_002F0438` (0x2f0438): the PS2 AI visual-effects manager destructor.
/// Installs the deleting-destructor vtable `0x487d78`, destroys the owned
/// object at `+0` through its vtable (`vtable[8]`, flag 3) and, when the
/// delete flag (`a1 & 1`) is set, frees `this` (`0x317e50`).
pub fn ps2ai_visual_effects_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x48_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(low(c, V0).wrapping_add(0x7d50));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        sw_r(c, S0, 0x4, c.g[V0][0]);
        c.g[A2][0] = lw_r(c, S0, 0);
        let none = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !none {
            c.cycles += 6;
            c.g[V0][0] = Addr(low(c, A2)).lw(c);
            c.g[A1][0] = 0x3;
            c.g[A0][0] = lh_r(c, V0, 0x8);
            c.g[V1][0] = lw_r(c, V0, 0xc);
            let target = low(c, V1);
            c.g[RA][0] = sx(0x002f_047c);
            c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x002f_047c)?;
        }
        c.cycles += 5;
        c.g[V0][0] = 0x48_0000;
        c.g[V1][0] = c.g[S1][0] & 0x1;
        c.g[V0][0] = sx(low(c, V0).wrapping_add(0x7d78));
        let keep = c.g[V1][0] == 0;
        sw_r(c, S0, 0x4, c.g[V0][0]);
        if !keep {
            c.cycles += 2;
            c.g[RA][0] = sx(0x002f_0498);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x002f_0498)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_00349AD0` (0x349ad0): `FloatRail_Save`, the float rail's serialiser.
/// Saves the flex rail (0x348b40), writes the 0xc-byte float block through the
/// archive's virtual writer (`vtable[8]`, `this+0x60`), then the variable-size
/// block: `size = *(int*)(this+0x20) * 0xc / ...` -- the original multiplies
/// the count at `+0x20` by 0xc (`mult`), adds it to the object base and writes
/// `this+0x70` through the writer.
fn rail_save(c: &mut Cpu, tail_block: u32, rets: [u32; 3]) -> Flow {
    c.cycles += 29;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[RA][0] = sx(rets[0]);
    c.g[S1][0] = c.g[A1][0];
    callee::FLEX_RAIL_SAVE.call(c, rets[0])?;
    c.g[V1][0] = lw_r(c, S1, 0);
    c.g[A1][0] = sx(low(c, S0).wrapping_add(0x60));
    c.g[A2][0] = 0xc;
    c.g[A0][0] = lh_r(c, V1, 0x8);
    c.g[V0][0] = lw_r(c, V1, 0xc);
    let target = low(c, V0);
    c.g[RA][0] = sx(rets[1]);
    c.g[A0][0] = sx(low(c, S1).wrapping_add(low(c, A0)));
    call_ptr(c, target, rets[1])?;
    c.g[A0][0] = lw_r(c, S0, 0x20);
    c.g[A2][0] = sx(tail_block);
    c.g[V1][0] = lw_r(c, S1, 0);
    c.mult(0, 0, A0, A2);
    c.g[A1][0] = lw_r(c, S0, 0x70);
    c.g[A0][0] = lh_r(c, V1, 0x8);
    c.g[V0][0] = lw_r(c, V1, 0xc);
    c.g[A2][0] = c.lo[0];
    let target = low(c, V0);
    c.g[RA][0] = sx(rets[2]);
    c.g[A0][0] = sx(low(c, S1).wrapping_add(low(c, A0)));
    call_ptr(c, target, rets[2])?;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `FloatRail_Save` (0x349ad0): 0x50-byte variable block.
pub fn float_rail_save(c: &mut Cpu) {
    guest(c, |c| {
        rail_save(c, 0x50, [0x0034_9aec, 0x0034_9b08, 0x0034_9b30])
    })
}

/// `SpringRail_Save` (0x34ac10): same shape, 0x60-byte variable block.
pub fn spring_rail_save(c: &mut Cpu) {
    guest(c, |c| {
        rail_save(c, 0x60, [0x0034_ac2c, 0x0034_ac48, 0x0034_ac70])
    })
}

/// `func_0034FA88` (0x34fa88): `Floating_Save`, the floating-modifier
/// serialiser. Saves the modifier object (0x356b30), writes a 4-byte flag
/// (`this->+0x78 > 0`) plus the base pointer from the stack, then the 0x48-byte
/// block at `this+0x2c` through the archive writer.
pub fn floating_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x0034_faa4);
        c.g[S1][0] = c.g[A0][0];
        callee::MOD_OBJ_SAVE.call(c, 0x0034_faa4)?;
        c.g[V1][0] = lw_r(c, S0, 0);
        c.g[A1][0] = c.g[SP][0];
        c.g[V0][0] = lw_r(c, S1, 0x78);
        c.g[A2][0] = 0x4;
        c.g[A0][0] = lh_r(c, V1, 0x8);
        c.g[A3][0] = lw_r(c, V1, 0xc);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
        let target = low(c, A3);
        c.g[RA][0] = sx(0x0034_facc);
        sw_r(c, SP, 0, c.g[V0][0]);
        call_ptr(c, target, 0x0034_facc)?;
        c.g[V1][0] = lw_r(c, S0, 0);
        c.g[A1][0] = sx(low(c, S1).wrapping_add(0x2c));
        c.g[A2][0] = 0x48;
        c.g[A0][0] = lh_r(c, V1, 0x8);
        c.g[V0][0] = lw_r(c, V1, 0xc);
        let target = low(c, V0);
        c.g[RA][0] = sx(0x0034_fae8);
        c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
        call_ptr(c, target, 0x0034_fae8)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `func_003AABD8` (0x3aabd8): `cPS2ChunkResolve`. Runs the chunk resolve hook
/// on the chunk's flags (`*(this+0xc) & 0x70`), then fills the bounding box at
/// `+0x28`..`+0x3c` and `+0x34`..`+0x44` with the centre (`+0x1c`..`+0x24`) plus
/// and minus the half-extent already in `f0`; `v0` is left 1.
pub fn cps2_chunk_resolve_v20(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[A0][0] = lw_r(c, S0, 0xc);
        c.g[RA][0] = sx(0x003a_abf4);
        c.g[A0][0] = c.g[A0][0] & 0x70;
        callee::WORLD_CHUNK_RESOLVE_HOOK.call(c, 0x003a_abf4)?;
        lwc1_r(c, 3, S0, 0x1c);
        c.g[V0][0] = 0x1;
        lwc1_r(c, 2, S0, 0x20);
        lwc1_r(c, 1, S0, 0x24);
        c.f[4] = (FpuF32(c.f[3]) + FpuF32(c.f[0])).0;
        c.f[5] = (FpuF32(c.f[2]) + FpuF32(c.f[0])).0;
        sw_r(c, S0, 0x44, 0);
        c.f[6] = (FpuF32(c.f[1]) + FpuF32(c.f[0])).0;
        sw_r(c, S0, 0x40, 0);
        c.f[3] = (FpuF32(c.f[3]) - FpuF32(c.f[0])).0;
        swc1_r(c, 4, S0, 0x34);
        c.f[2] = (FpuF32(c.f[2]) - FpuF32(c.f[0])).0;
        swc1_r(c, 5, S0, 0x38);
        c.f[1] = (FpuF32(c.f[1]) - FpuF32(c.f[0])).0;
        swc1_r(c, 6, S0, 0x3c);
        swc1_r(c, 3, S0, 0x28);
        swc1_r(c, 2, S0, 0x2c);
        swc1_r(c, 1, S0, 0x30);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_0026CD20` (0x26cd20): stop playback. When `this+0x70` is 1: stop the
/// two streams at `+0x60`, restore the recorded object states
/// (`Replay_SetState` / `PlaybackMan_RestoreWorldState`), clear `+0x70` and
/// notify the scene at `*(*(this+0x5c)+0x84)`. Signature: `void stop(this)`.
pub fn playback_man_stop_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 0x1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_r(c, S0, 0x70);
        if c.g[V0][0] != c.g[V1][0] {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 18;
            c.g[RA][0] = sx(0x0026_cd48);
            c.g[A0][0] = lw_r(c, S0, 0x60);
            call_ptr(c, 0x0012_af68, 0x0026_cd48)?;
            c.g[RA][0] = sx(0x0026_cd50);
            c.g[A0][0] = lw_r(c, S0, 0x60);
            call_ptr(c, 0x0012_8ac0, 0x0026_cd50)?;
            c.g[V0][0] = lw_r(c, S0, 0x5c);
            c.g[A0][0] = lw_r(c, V0, 0x28);
            c.g[RA][0] = sx(0x0026_cd60);
            c.g[A1][0] = lw_r(c, A0, 0x4);
            callee::REPLAY_SET_STATE.call(c, 0x0026_cd60)?;
            c.g[RA][0] = sx(0x0026_cd68);
            c.g[A0][0] = c.g[S0][0];
            callee::PLAYBACK_RESTORE_WORLD.call(c, 0x0026_cd68)?;
            sw_r(c, S0, 0x70, 0);
            c.g[A0][0] = lw_r(c, S0, 0x5c);
            c.g[V0][0] = lw_r(c, S0, 0x60);
            c.g[V1][0] = lw_r(c, A0, 0x84);
            c.g[A1][0] = lw_r(c, V0, 0x28);
            c.g[RA][0] = sx(0x0026_cd84);
            c.g[A0][0] = lw_r(c, V1, 0x4);
            call_ptr(c, 0x0015_edc8, 0x0026_cd84)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Screen rendering.

/// The shared body of the three `Render` methods (0x232da8, 0x233390,
/// 0x2339f8): look up the app module (`gp-0x854`), call its `canRender` slot
/// (`vtable[0x88]/[0x8c]`); unless it says no, draw the fade
/// (`AppModule_RenderFade`), optionally refresh the widget (`0x397df8`), then
/// call the module's `render` slot (`vtable[0xa0]/[0xa4]`) and return 1.
fn render_screen(c: &mut Cpu, rets: [u32; 4]) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x20);
    c.g[A1][0] = lw_r(c, GP, 0u32.wrapping_sub(0x854));
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[V1][0] = lw_r(c, A1, 0x10d8);
    c.g[A0][0] = lh_r(c, V1, 0x88);
    c.g[V0][0] = lw_r(c, V1, 0x8c);
    let target = low(c, V0);
    c.g[RA][0] = sx(rets[0]);
    c.g[A0][0] = sx(low(c, A1).wrapping_add(low(c, A0)));
    call_ptr(c, target, rets[0])?;
    let hidden = c.g[V0][0] == 0;
    c.g[V0][0] = 0;
    if hidden {
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        return Ok(());
    }
    c.cycles += 4;
    c.g[RA][0] = sx(rets[1]);
    c.g[A0][0] = c.g[S0][0];
    callee::APP_MODULE_RENDER_FADE.call(c, rets[1])?;
    let faded = c.g[V0][0] != 0;
    c.g[A1][0] = lw_r(c, GP, 0u32.wrapping_sub(0x854));
    if !faded {
        c.cycles += 3;
        c.g[RA][0] = sx(rets[2]);
        c.g[A0][0] = lw_r(c, S0, 0xc);
        call_ptr(c, 0x0039_7df8, rets[2])?;
        c.g[A1][0] = lw_r(c, GP, 0u32.wrapping_sub(0x854));
    }
    c.cycles += 6;
    c.g[V0][0] = lw_r(c, A1, 0x10d8);
    c.g[A0][0] = lh_r(c, V0, 0xa0);
    c.g[V1][0] = lw_r(c, V0, 0xa4);
    let target = low(c, V1);
    c.g[RA][0] = sx(rets[3]);
    c.g[A0][0] = sx(low(c, A1).wrapping_add(low(c, A0)));
    call_ptr(c, target, rets[3])?;
    c.g[V0][0] = 0x1;
    c.cycles += 4;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `StartScreen_Render` (0x232da8).
pub fn start_screen_render(c: &mut Cpu) {
    guest(c, |c| {
        render_screen(c, [0x0023_2dd0, 0x0023_2de0, 0x0023_2df0, 0x0023_2e08])
    })
}

/// `cGameLoadScreen_Render` (0x233390).
pub fn c_game_load_screen_render(c: &mut Cpu) {
    guest(c, |c| {
        render_screen(c, [0x0023_33b8, 0x0023_33c8, 0x0023_33d8, 0x0023_33f0])
    })
}

/// `cFELoadScreen_Render` (0x2339f8).
pub fn c_fe_load_screen_render(c: &mut Cpu) {
    guest(c, |c| {
        render_screen(c, [0x0023_3a20, 0x0023_3a30, 0x0023_3a40, 0x0023_3a58])
    })
}
// ---------------------------------------------------------------------------
// Loops and worker threads.

/// `func_00398438` (0x398438): free every font in a font set. Walks the
/// `count` (int8 at `+0x34`) entries of the 0xc-byte table at `+0x4`; each
/// font (`+8`) is destroyed (`vtable[8]`, flag 3) and its slots cleared, then
/// the count byte is zeroed. Signature: `void free_fonts(UIFontSet* this)`.
pub fn ui_font_set_free_fonts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lb_r(c, S2, 0x34);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[S0][0] = sx(low(c, S2).wrapping_add(0x4));
            loop {
                c.cycles += 3;
                c.g[A0][0] = lw_r(c, S0, 0x8);
                if c.g[A0][0] == 0 {
                    sw_r(c, S0, 0x4, 0);
                } else {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0039_8474);
                    c.g[A1][0] = 0x3;
                    call_ptr(c, 0x0039_16c0, 0x0039_8474)?;
                    sw_r(c, S0, 0x4, 0);
                }
                c.cycles += 6;
                c.g[S1][0] = sx(low(c, S1).wrapping_add(1));
                sw_r(c, S0, 0x0, 0);
                c.g[V0][0] = lb_r(c, S2, 0x34);
                let more = (c.g[S1][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[S0][0] = sx(low(c, S0).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_8460);
                }
            }
        }
        c.cycles += 7;
        sb_r(c, S2, 0x34, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_0031AC60` (0x31ac60): the PS/2 execution manager's worker thread.
/// Sets up its frame at `this + 0x10000`, then loops: run the pending job at
/// `+0x4048` through its vtable (`vtable[0x10]`) and clear it, post the
/// semaphore at `+0x4044` (`0x423de0`), and while `this+0x20` is zero go round
/// again (back-edge event check); when it is set, signal (`0x423cd0`) and free
/// (`0x423cc0`) and return. Dispatcher-shaped: the trampoline may re-enter at
/// the loop head after an unwind.
pub fn ps2_exec_man_worker_thread(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0031_ac60 => {
                    c.cycles += 8;
                    let frame = Frame::open(c, 0x30);
                    c.g[V0][0] = 0x1_0000;
                    frame.save(c, S1, 0x10);
                    frame.save(c, S0, 0x20);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[S0][0] = sx(low(c, S1).wrapping_add(low(c, V0)));
                    pc = 0x0031_ac9c;
                }
                0x0031_ac80 => {
                    c.cycles += 7;
                    c.g[A1][0] = lw_r(c, S0, 0x4048);
                    c.g[V0][0] = Addr(low(c, A1)).lw(c);
                    c.g[A0][0] = lh_r(c, V0, 0x10);
                    c.g[V1][0] = lw_r(c, V0, 0x14);
                    let target = low(c, V1);
                    c.g[RA][0] = sx(0x0031_ac98);
                    c.g[A0][0] = sx(low(c, A1).wrapping_add(low(c, A0)));
                    call_ptr(c, target, 0x0031_ac98)?;
                    sw_r(c, S0, 0x4048, 0);
                    pc = 0x0031_ac9c;
                }
                0x0031_ac9c => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x0031_aca4);
                    c.g[A0][0] = lw_r(c, S0, 0x4044);
                    call_ptr(c, 0x0042_3de0, 0x0031_aca4)?;
                    c.g[V1][0] = lw_r(c, S1, 0x20);
                    if c.g[V1][0] == 0 {
                        if back_edge_unwinds(c, 0x0031_ac80) {
                            return Ok(());
                        }
                        pc = 0x0031_ac80;
                    } else {
                        pc = 0x0031_acb0;
                    }
                }
                0x0031_acb0 => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x0031_acb8);
                    c.g[A0][0] = lw_r(c, S1, 0x10);
                    call_ptr(c, 0x0042_3cd0, 0x0031_acb8)?;
                    c.g[RA][0] = sx(0x0031_acc0);
                    call_ptr(c, 0x0042_3cc0, 0x0031_acc0)?;
                    let frame = Frame { size: 0x30 };
                    frame.restore(c, S0, 0x20);
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0031_ac60, 0x0031_acd4);
                    return Ok(());
                }
            }
        }
    })
}

/// `func_00353020` (0x353020): `ModifierBlock_ForwardV43`. Forwards a modifier
/// call to the block's first child (`this->+0` through `vtable[0xc8]`) and then
/// to the block itself (`child->+0xc` through `vtable[0x158]`, with the two
/// saved arguments). Signature: `void forward(this, a1, a2)`.
pub fn modifier_block_forward_v43(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = lw_r(c, A0, 0);
        let none = c.g[A3][0] == 0;
        c.g[S1][0] = c.g[A2][0];
        if none {
            c.cycles += 1;
            c.g[A3][0] = 0;
        } else {
            c.cycles += 7;
            c.g[V1][0] = Addr(low(c, A3)).lw(c);
            c.g[A0][0] = lh_r(c, V1, 0xc8);
            c.g[V0][0] = lw_r(c, V1, 0xcc);
            let target = low(c, V0);
            c.g[RA][0] = sx(0x0035_3054);
            c.g[A0][0] = sx(low(c, A3).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0035_3054)?;
            c.g[A3][0] = c.g[V0][0];
        }
        c.cycles += 2;
        let skip = c.g[A3][0] == 0;
        c.g[A1][0] = c.g[S0][0];
        if !skip {
            c.cycles += 6;
            c.g[V0][0] = lw_r(c, A3, 0xc);
            c.g[A2][0] = c.g[S1][0];
            c.g[A0][0] = lh_r(c, V0, 0x158);
            c.g[V1][0] = lw_r(c, V0, 0x15c);
            let target = low(c, V1);
            c.g[RA][0] = sx(0x0035_3080);
            c.g[A0][0] = sx(low(c, A3).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0035_3080)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI widget event handlers.

/// `func_001F5650` (0x1f5650): `cFEStateCareerStats::OnWidgetAction`. For
/// widget action 6 only (action 5 and a null widget do nothing) it asks the
/// widget (`widget+0x18` through the owner's `vtable[0x28]`) whether it has a
/// selection; if it does, it forwards the event to `this->+0x10 + 0x18`
/// (`0x39f400`). Signature: `void on_widget_action(this, widget, action)`.
pub fn c_fe_state_career_stats_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let no_widget = c.g[V1][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        let mut tail_charge = true;
        if no_widget {
            // nothing to do
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x5;
            let five = c.g[A2][0] == c.g[V0][0];
            c.g[V0][0] = 0x6;
            if five {
                // nothing to do
            } else {
                c.cycles += 2;
                if c.g[A2][0] != c.g[V0][0] {
                    tail_charge = false;
                } else {
                    c.cycles += 11;
                    c.g[V0][0] = lw_r(c, S0, 0x10);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = lw_r(c, V1, 0x18);
                    c.g[A3][0] = Addr(low(c, V0)).lw(c);
                    c.g[V1][0] = lw_r(c, A3, 0x4);
                    c.g[A0][0] = lh_r(c, V1, 0x28);
                    c.g[V0][0] = lw_r(c, V1, 0x2c);
                    let target = low(c, V0);
                    c.g[RA][0] = sx(0x001f_56a0);
                    c.g[A0][0] = sx(low(c, A3).wrapping_add(low(c, A0)));
                    call_ptr(c, target, 0x001f_56a0)?;
                    let no_selection = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[V0][0];
                    if !no_selection {
                        c.cycles += 3;
                        c.g[A0][0] = lw_r(c, S0, 0x10);
                        c.g[RA][0] = sx(0x001f_56b4);
                        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x18));
                        call_ptr(c, 0x0039_f400, 0x001f_56b4)?;
                    }
                }
            }
        }
        if tail_charge {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_0035A0xx` `SplineModifier_v05` (0x359eb8): the spline modifier's
/// event handler. Action 0x190 runs `SplineModifier_SetSpeed` and re-enables
/// the modifier; 0x191 does the same without the call; 0x192 negates the
/// speed at `+0x48`; anything else returns. `+0x50` is cleared and `+0x54` set
/// to 1 whenever it acts. Signature: `void handler(this, action, ...)`.
pub fn spline_modifier_v05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x190;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let not_first = c.g[A1][0] != c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if !not_first {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0035_9ed8);
            callee::SPLINE_MODIFIER_SET_SPEED.call(c, 0x0035_9ed8)?;
            c.g[V0][0] = 0x1;
            c.cycles += 3;
            sw_r(c, S0, 0x50, 0);
            sw_r(c, S0, 0x54, c.g[V0][0]);
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x191;
            let not_second = c.g[A1][0] != c.g[V0][0];
            c.g[V0][0] = 0x192;
            if not_second {
                c.cycles += 2;
                if c.g[A1][0] != c.g[V0][0] {
                    frame.restore(c, S0, 0x10);
                } else {
                    c.cycles += 6;
                    lwc1_r(c, 0, S0, 0x48);
                    c.g[V0][0] = 0x1;
                    sw_r(c, S0, 0x54, c.g[V0][0]);
                    c.f[0] = (-FpuF32(c.f[0])).0;
                    sw_r(c, S0, 0x50, 0);
                    swc1_r(c, 0, S0, 0x48);
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                }
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0x1;
                c.cycles += 3;
                sw_r(c, S0, 0x50, 0);
                sw_r(c, S0, 0x54, c.g[V0][0]);
                c.cycles += 1;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_00358140` (0x358140): remove every instance of section `a1` from a
/// multi-particle object. Walks the instance array (`+0xc`, count at `+4`) and
/// for each match replaces the slot with the last instance (and shrinks the
/// count), then retries the same index. The loop back edge carries an event
/// check. Signature: `void remove(MultiParticle* this, u8 section)`.
pub fn multi_particle_remove_section_instances(c: &mut Cpu) {
    c.cycles += 3;
    c.g[T0][0] = lw_r(c, A0, 0x4);
    let empty = (c.g[T0][0] as i64) <= 0;
    c.g[A2][0] = 0;
    if empty {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[T2][0] = 0xffff_ffff_ffff_0000;
    c.g[T2][0] = c.g[T2][0] | 0xffff;
    c.g[T1][0] = lw_r(c, A0, 0xc);
    loop {
        c.cycles += 5;
        c.g[V0][0] = sx(low(c, A2) << 2);
        c.g[A3][0] = sx(low(c, V0).wrapping_add(low(c, T1)));
        c.g[V1][0] = lbu_r(c, A3, 0);
        if c.g[V1][0] != c.g[A1][0] {
            c.g[T0][0] = lw_r(c, A0, 0x4);
        } else {
            c.cycles += 4;
            c.g[V1][0] = sx(low(c, T0).wrapping_sub(1));
            let less = (c.g[A2][0] as i64) < (c.g[V1][0] as i64);
            c.g[V0][0] = less as u64;
            let last = c.g[V0][0] == 0;
            sw_r(c, A0, 0x4, c.g[V1][0]);
            if last {
                c.cycles += 1;
                sw_r(c, A3, 0, c.g[T2][0]);
            } else {
                c.cycles += 6;
                c.g[V0][0] = sx(low(c, V1) << 2);
                c.g[A2][0] = sx(low(c, A2).wrapping_sub(1));
                c.g[V0][0] = sx(low(c, V0).wrapping_add(low(c, T1)));
                c.g[V1][0] = Addr(low(c, V0)).lw(c);
                sw_r(c, A3, 0, c.g[V1][0]);
            }
            c.cycles += 1;
            c.g[T0][0] = lw_r(c, A0, 0x4);
        }
        c.cycles += 4;
        c.g[A2][0] = sx(low(c, A2).wrapping_add(1));
        let more = (c.g[A2][0] as i64) < (c.g[T0][0] as i64);
        c.g[V0][0] = more as u64;
        if !more {
            break;
        }
        c.g[T1][0] = lw_r(c, A0, 0xc);
        if event_due(c) {
            deliver_event_here(c, 0x0035_8158);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `func_00208B00` (0x208b00): `OVScreenSession::OnActivate`. Activates the
/// screen (`0x39e4c0`), looks up the widget named `0x470a20`
/// (`0x39d860(this->+0xd4, hash)`) and, if found, calls its `vtable[0x38]`
/// (with 0) and `vtable[0x48]` (with 0). Signature: `void on_activate(this)`.
pub fn ov_screen_session_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0020_8b14);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e4c0, 0x0020_8b14)?;
        c.g[A0][0] = 0x47_0000;
        c.g[RA][0] = sx(0x0020_8b20);
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0xa20));
        call_ptr(c, 0x0031_7670, 0x0020_8b20)?;
        c.g[A0][0] = lw_r(c, S0, 0xd4);
        c.g[RA][0] = sx(0x0020_8b2c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0020_8b2c)?;
        c.g[S0][0] = c.g[V0][0];
        let missing = c.g[S0][0] == 0;
        c.g[A1][0] = 0;
        if !missing {
            c.cycles += 11;
            c.g[V0][0] = lw_r(c, S0, 0x8);
            c.g[A0][0] = lh_r(c, V0, 0x38);
            c.g[V1][0] = lw_r(c, V0, 0x3c);
            let target = low(c, V1);
            c.g[RA][0] = sx(0x0020_8b4c);
            c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0020_8b4c)?;
            c.g[V0][0] = lw_r(c, S0, 0x8);
            c.g[A1][0] = 0;
            c.g[A0][0] = lh_r(c, V0, 0x48);
            c.g[V1][0] = lw_r(c, V0, 0x4c);
            let target = low(c, V1);
            c.g[RA][0] = sx(0x0020_8b64);
            c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0020_8b64)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
// ---------------------------------------------------------------------------
// UI pair / texture manager.

/// `func_0039C1C8` (0x39c1c8): `UIPair_UpdateAnim`. Updates the pair itself
/// (`UIObject_UpdateAnim`) and then each of its two children at `+0x78` and
/// `+0x7c` through `vtable[0x98]`. Signature: `void update_anim(this, flags)`.
pub fn ui_pair_update_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0] & 0xffff;
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0039_c1e8);
        c.g[A1][0] = c.g[S1][0];
        callee::UI_OBJECT_UPDATE_ANIM.call(c, 0x0039_c1e8)?;
        c.g[A2][0] = lw_r(c, S0, 0x78);
        let no_first = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !no_first {
            c.cycles += 5;
            c.g[V0][0] = lw_r(c, A2, 0x8);
            c.g[A0][0] = lh_r(c, V0, 0x98);
            c.g[V1][0] = lw_r(c, V0, 0x9c);
            let target = low(c, V1);
            c.g[RA][0] = sx(0x0039_c208);
            c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0039_c208)?;
        }
        c.cycles += 3;
        c.g[A2][0] = lw_r(c, S0, 0x7c);
        let no_second = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !no_second {
            c.cycles += 5;
            c.g[V0][0] = lw_r(c, A2, 0x8);
            c.g[A0][0] = lh_r(c, V0, 0x98);
            c.g[V1][0] = lw_r(c, V0, 0x9c);
            let target = low(c, V1);
            c.g[RA][0] = sx(0x0039_c228);
            c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0039_c228)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_00367150` (0x367150): `TexMan_AllocNBTexInfo`. Pops a free texture
/// info node from the list at `+0x1f48`, works out its slot index
/// (`(node - 8 - this) / 4`), bumps the live count at `+4`, allocates a fresh
/// 0x58-byte node (`EA_MemAllocNamed(size, 0x492068, 0x21000000, 0)`) and
/// stores it in the slot. Returns the slot index. Signature:
/// `int alloc(TexMan* this)`.
pub fn tex_man_alloc_nb_tex_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[A1][0] = sx(low(c, A1).wrapping_add(0x2068));
        frame.save(c, S1, 0x10);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(low(c, A0).wrapping_add(0x8));
        c.g[V0][0] = lw_r(c, A0, 0x1f48);
        c.g[V1][0] = lw_r(c, A0, 0x4);
        c.g[S0][0] = sx(low(c, V0).wrapping_sub(0x8));
        c.g[A2][0] = Addr(low(c, V0)).lw(c);
        c.g[S0][0] = sx(low(c, S0).wrapping_sub(low(c, A0)));
        c.g[V1][0] = sx(low(c, V1).wrapping_add(0x1));
        c.g[S0][0] = sx(((low(c, S0) as i32) >> 2) as u32);
        sw_r(c, A0, 0x1f48, c.g[A2][0]);
        c.g[V0][0] = sx(low(c, S0) << 2);
        sw_r(c, A0, 0x4, c.g[V1][0]);
        c.g[S1][0] = sx(low(c, S1).wrapping_add(low(c, V0)));
        c.g[A0][0] = 0x58;
        c.g[RA][0] = sx(0x0036_71a8);
        c.g[A2][0] = 0x2100_0000;
        call_ptr(c, 0x0031_7d70, 0x0036_71a8)?;
        sw_r(c, S1, 0, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end screens.

/// `func_001F7198` (0x1f7198): `OVScreenEnterLodge::Init`. Creates the screen
/// widget named `0x46f4d0` from `this->+0x10` (`0x397cc0`), stores it at
/// `+0x40`, and if it exists shows it (`0x39c870`); finally runs the two shared
/// finish calls (`0x20d110`, `0x20a380`). Signature: `void init(this)`.
pub fn ov_screen_enter_lodge_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x47_0000;
        c.g[A0][0] = sx(low(c, A0).wrapping_sub(0xb30));
        c.g[RA][0] = sx(0x001f_71bc);
        c.g[S0][0] = lw_r(c, S1, 0x10);
        call_ptr(c, 0x0031_7670, 0x001f_71bc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001f_71d0);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_71d0)?;
        let missing = c.g[V0][0] == 0;
        sw_r(c, S1, 0x40, c.g[V0][0]);
        if !missing {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001f_71e8);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_71e8)?;
        }
        c.cycles += 9;
        c.g[RA][0] = sx(0x001f_71f0);
        call_ptr(c, 0x0020_d110, 0x001f_71f0)?;
        c.g[RA][0] = sx(0x001f_71f8);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0020_a380, 0x001f_71f8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_002122E8` (0x2122e8): `OVScreenAutosave`. When the autosave state at
/// `+0x1c0` is 0 or 6, looks up the widget named `0x471e38`, finds it
/// (`0x39d860(this->+0x40, hash)`) and calls its `vtable[0x48]` with 0.
/// Any other state just refreshes the icon (`0x212360`).
/// Signature: `void update(this)`.
pub fn ov_screen_autosave_v26(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = lw_r(c, S0, 0x1c0);
        let zero = c.g[V1][0] == 0;
        c.g[V0][0] = 0x6;
        if zero {
            c.cycles += 1;
            c.g[A0][0] = 0x47_0000;
        } else {
            c.cycles += 2;
            if c.g[V1][0] == c.g[V0][0] {
                c.g[A0][0] = 0x47_0000;
            } else {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0021_2314);
                callee::AUTOSAVE_UPDATE_ICON.call(c, 0x0021_2314)?;
                frame.restore(c, S0, 0x10);
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 12;
        c.g[RA][0] = sx(0x0021_2328);
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x1e38));
        call_ptr(c, 0x0031_7670, 0x0021_2328)?;
        c.g[A0][0] = lw_r(c, S0, 0x40);
        c.g[RA][0] = sx(0x0021_2334);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0021_2334)?;
        c.g[V1][0] = lw_r(c, V0, 0x8);
        c.g[A1][0] = 0;
        c.g[A0][0] = lh_r(c, V1, 0x48);
        c.g[A2][0] = lw_r(c, V1, 0x4c);
        let target = low(c, A2);
        c.g[RA][0] = sx(0x0021_234c);
        c.g[A0][0] = sx(low(c, V0).wrapping_add(low(c, A0)));
        call_ptr(c, target, 0x0021_234c)?;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
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

/// `func_0020CBE8` (0x20cbe8): `GameUI_CanOpenPauseMenu`. False while the
/// pause menu is open; otherwise looks up the widget at
/// `((world->+0x84)->+0x48) + 0x18` (`0x39fa88`) and returns true only if the
/// widget exists, is not disabled (`+0x1c & 1`) and its `vtable[0xc8]`
/// predicate says so. Signature: `bool can_open(world)`.
pub fn game_ui_can_open_pause_menu(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = lw_r(c, GP, 0u32.wrapping_sub(0x848));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = lw_r(c, V0, 0x84);
        c.g[RA][0] = sx(0x0020_cc04);
        c.g[S0][0] = lw_r(c, V1, 0x48);
        call_ptr(c, 0x0020_cba0, 0x0020_cc04)?;
        let open = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        let mut tail_charge = true;
        if !open {
            c.cycles += 5;
            c.g[RA][0] = sx(0x0020_cc14);
            c.g[A0][0] = sx(low(c, S0).wrapping_add(0x18));
            call_ptr(c, 0x0039_fa88, 0x0020_cc14)?;
            c.g[A1][0] = c.g[V0][0];
            let missing = c.g[A1][0] == 0;
            c.g[V0][0] = 0x1;
            if !missing {
                c.cycles += 4;
                c.g[V0][0] = lw_r(c, A1, 0x1c);
                c.g[V0][0] = c.g[V0][0] & 0x1;
                let disabled = c.g[V0][0] != 0;
                c.g[V0][0] = 0;
                if !disabled {
                    c.cycles += 7;
                    c.g[V1][0] = lw_r(c, A1, 0x8);
                    c.g[A0][0] = lh_r(c, V1, 0xc8);
                    c.g[V0][0] = lw_r(c, V1, 0xcc);
                    let target = low(c, V0);
                    c.g[RA][0] = sx(0x0020_cc44);
                    c.g[A0][0] = sx(low(c, A1).wrapping_add(low(c, A0)));
                    call_ptr(c, target, 0x0020_cc44)?;
                    tail_charge = false;
                }
            }
        }
        if tail_charge {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_0020D4F0` (0x20d4f0): `OVScreenYNDialog::OnWidgetCreated`. If the
/// created widget (`a1+0x38`) is the yes/no dialog's button ("0x46e818"),
/// enables it when `this->+0x9c == 6` (`0x39b960(widget, 0)`) or leaves it
/// disabled otherwise (`0x39b960(widget, 1)`).
/// Signature: `void on_widget_created(this, widget)`.
pub fn ov_screen_yn_dialog_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = 0x47_0000;
        c.g[S0][0] = lw_r(c, S1, 0x38);
        c.g[RA][0] = sx(0x0020_d51c);
        c.g[A0][0] = sx(low(c, A0).wrapping_sub(0x17e8));
        call_ptr(c, 0x0031_7670, 0x0020_d51c)?;
        let other = c.g[S0][0] != c.g[V0][0];
        frame.restore(c, S0, 0x30);
        if other {
            c.cycles += 5;
            frame.restore(c, S1, 0x20);
            frame.restore(c, S2, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 4;
        c.g[V1][0] = lw_r(c, S2, 0x9c);
        c.g[V0][0] = 0x6;
        let enabled = c.g[V1][0] != c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        if enabled {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0020_d54c);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0039_b960, 0x0020_d54c)?;
        } else {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0020_d53c);
            c.g[A1][0] = 0;
            call_ptr(c, 0x0039_b960, 0x0020_d53c)?;
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_002305C8` (0x2305c8): `GameModule_EndReplayPlayback`. Stops the
/// replay (`0x26f980`), hides the replay UI (`0x2cc0b8(ui, 1)`), closes it
/// (`0x2cc460(ui)`), then pops one entry off the two replay stacks at
/// `D_005367`/`D_005368` (a counter at `gp-0x69c` and the saved slot at
/// `gp-0x6a0`). Signature: `void end_replay(GameModule* this)`.
pub fn game_module_end_replay_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x0023_05e0);
        c.g[A0][0] = lw_r(c, S0, 0x28);
        call_ptr(c, 0x0026_f980, 0x0023_05e0)?;
        c.g[A0][0] = lw_r(c, S0, 0xa4);
        c.g[RA][0] = sx(0x0023_05ec);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x002c_c0b8, 0x0023_05ec)?;
        c.g[RA][0] = sx(0x0023_05f4);
        c.g[A0][0] = lw_r(c, S0, 0xa4);
        call_ptr(c, 0x002c_c460, 0x0023_05f4)?;
        c.g[V0][0] = lw_r(c, GP, 0u32.wrapping_sub(0x69c));
        c.g[A0][0] = 0x53_0000;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x66e8));
        c.g[A1][0] = 0x44_0000;
        c.g[V0][0] = sx(low(c, V0).wrapping_sub(0x1));
        c.g[A1][0] = sx(low(c, A1).wrapping_add(0x28f0));
        c.g[V1][0] = sx(low(c, V0) << 2);
        sw_r(c, GP, 0u32.wrapping_sub(0x69c), c.g[V0][0]);
        c.g[V1][0] = sx(low(c, V1).wrapping_add(low(c, A0)));
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = Addr(low(c, V1)).lw(c);
        c.g[RA][0] = c.ld(low(c, SP));
        c.g[V0][0] = sx(low(c, V0) << 2);
        c.g[V0][0] = sx(low(c, V0).wrapping_add(low(c, A1)));
        c.g[V1][0] = Addr(low(c, V0)).lw(c);
        sw_r(c, GP, 0u32.wrapping_sub(0x6a0), c.g[V1][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `func_001E9130` (0x1e9130): `OVScreenHiscoreList::OnWidgetAction`. For
/// action 5 on a widget whose state (`widget+0x18`) is 1: ask the list at
/// `this->+0x10` for the selected entry (`vtable[0x20]` with 1) and forward
/// the action to that entry (`0x39f400(list + 0x18)`).
/// Signature: `void on_widget_action(this, widget, action)`.
pub fn ov_screen_hiscore_list_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let no_widget = c.g[A1][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        let mut tail_charge = true;
        if !no_widget {
            c.cycles += 3;
            c.g[V0][0] = 0x5;
            if c.g[A2][0] != c.g[V0][0] {
                tail_charge = false;
            } else {
                c.cycles += 4;
                c.g[V1][0] = lw_r(c, A1, 0x18);
                c.g[V0][0] = 0x1;
                let not_one = c.g[V1][0] != c.g[V0][0];
                c.g[A1][0] = 0;
                if !not_one {
                    c.cycles += 10;
                    c.g[V0][0] = lw_r(c, S0, 0x10);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = 0x1;
                    c.g[A3][0] = Addr(low(c, V0)).lw(c);
                    c.g[V1][0] = lw_r(c, A3, 0x4);
                    c.g[A0][0] = lh_r(c, V1, 0x20);
                    c.g[V0][0] = lw_r(c, V1, 0x24);
                    let target = low(c, V0);
                    c.g[RA][0] = sx(0x001e_9184);
                    c.g[A0][0] = sx(low(c, A3).wrapping_add(low(c, A0)));
                    call_ptr(c, target, 0x001e_9184)?;
                    c.g[A1][0] = c.g[V0][0];
                }
                c.cycles += 3;
                c.g[A0][0] = lw_r(c, S0, 0x10);
                c.g[RA][0] = sx(0x001e_9194);
                c.g[A0][0] = sx(low(c, A0).wrapping_add(0x18));
                call_ptr(c, 0x0039_f400, 0x001e_9194)?;
            }
        }
        if tail_charge {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_00352708` (0x352708): `MeshAnim_KillRestorable`. If asked to run the
/// kill script (`a1 != 0`) and `WorldObj_RunKillScript` accepts, return;
/// otherwise if the node at `+0x6b0` is still the restorable one replace it
/// with a dead node (`WorldObj_ReplaceWithDeadNode`) and, when it became a
/// dead node (kind 2), destroy it (`vtable[8]`, flag 3).
/// Signature: `void kill_restorable(this, run_script)`.
pub fn mesh_anim_kill_restorable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let no_script = c.g[A1][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if !no_script {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0035_2724);
            callee::WORLD_OBJ_RUN_KILL_SCRIPT.call(c, 0x0035_2724)?;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x10);
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 3;
        c.g[V0][0] = lw_r(c, S0, 0x6b0);
        if c.g[V0][0] != 0 {
            c.g[V1][0] = lw_r(c, S0, 0x6b0);
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0035_2740);
            c.g[A0][0] = c.g[S0][0];
            callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE.call(c, 0x0035_2740)?;
            c.g[V1][0] = lw_r(c, S0, 0x6b0);
        }
        c.cycles += 3;
        c.g[V0][0] = 0x2;
        if c.g[V1][0] != c.g[V0][0] {
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        let null = c.g[S0][0] == 0;
        c.g[A1][0] = 0x3;
        if !null {
            c.cycles += 5;
            c.g[V0][0] = lw_r(c, S0, 0xc);
            c.g[A0][0] = lh_r(c, V0, 0x8);
            c.g[V1][0] = lw_r(c, V0, 0xc);
            let target = low(c, V1);
            c.g[RA][0] = sx(0x0035_276c);
            c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0035_276c)?;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_00195DE0` (0x195de0): `FELocale_LoadGameTables`. Picks the table id
/// `{0: 0xc, 1: 0xd, 2: 0xe, 3: 0xf, else -1}` from the language byte at
/// `this+1`, marks the locale (`+0x58 = 2`) and loads that table (`0x1959a8`).
/// Signature: `void load_game_tables(FELocale* this)`.
pub fn fe_locale_load_game_tables(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x10));
        c.g[V1][0] = 0x2;
        c.sd(low(c, SP), c.g[RA][0]);
        sb_r(c, A0, 0x58, c.g[V1][0]);
        c.g[V0][0] = 0x1;
        c.g[A2][0] = lb_r(c, A0, 0x1);
        let one = c.g[A2][0] == c.g[V0][0];
        c.g[A1][0] = u64::MAX;
        if one {
            c.cycles += 2;
            c.g[A1][0] = 0xd;
        } else {
            c.cycles += 3;
            let below = (c.g[A2][0] as i64) < 2;
            c.g[V0][0] = 0x2;
            if below {
                c.cycles += 2;
                if c.g[A2][0] == 0 {
                    c.g[A1][0] = 0xc;
                } else {
                    c.cycles += 2;
                }
            } else {
                c.cycles += 2;
                let two = c.g[A2][0] == c.g[V0][0];
                c.g[V0][0] = 0x3;
                if two {
                    c.cycles += 1;
                    c.g[A1][0] = 0xe;
                } else {
                    c.cycles += 2;
                    if c.g[A2][0] == c.g[V0][0] {
                        c.g[A1][0] = 0xf;
                    } else {
                        c.cycles += 2;
                    }
                }
            }
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x0019_5e48);
        callee::FE_LOCALE_LOAD_TABLE.call(c, 0x00195e48)?;
        c.g[RA][0] = c.ld(low(c, SP));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}
// ---------------------------------------------------------------------------
// Online (BE) system.

/// `func_0014EA90` (0x14ea90): `BE_ResetNetInterfaces`. Re-initialises two BE
/// interfaces (ids 3 and 0xc) by calling their `vtable[0x10]` method with the
/// argument in `a1`, then refreshes the player-flag option
/// (`BEOptionInt_SwapPlayerFlags(this)`).
pub fn be_reset_net_interfaces(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0014_eaa4);
        c.g[S0][0] = c.g[A0][0];
        callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x0014_eaa4)?;
        c.g[A1][0] = 0x3;
        c.g[RA][0] = sx(0x0014_eab0);
        c.g[A0][0] = c.g[V0][0];
        callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x0014_eab0)?;
        c.g[V1][0] = lw_r(c, V0, 0xc);
        c.g[A0][0] = lh_r(c, V1, 0x10);
        c.g[A1][0] = lw_r(c, V1, 0x14);
        let target = low(c, A1);
        c.g[RA][0] = sx(0x0014_eac4);
        c.g[A0][0] = sx(low(c, V0).wrapping_add(low(c, A0)));
        call_ptr(c, target, 0x0014_eac4)?;
        c.g[RA][0] = sx(0x0014_eacc);
        callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x0014_eacc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0014_ead8);
        c.g[A1][0] = 0xc;
        callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x0014_ead8)?;
        c.g[V1][0] = lw_r(c, V0, 0xc);
        c.g[A0][0] = lh_r(c, V1, 0x10);
        c.g[A1][0] = lw_r(c, V1, 0x14);
        let target = low(c, A1);
        c.g[RA][0] = sx(0x0014_eaec);
        c.g[A0][0] = sx(low(c, V0).wrapping_add(low(c, A0)));
        call_ptr(c, target, 0x0014_eaec)?;
        c.g[RA][0] = sx(0x0014_eaf4);
        c.g[A0][0] = c.g[S0][0];
        callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS.call(c, 0x0014_eaf4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
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
// UI lists and rails.

/// `func_00397948` (0x397948): `UIList_DeleteAll`. Walks the list from
/// `this->+4` (the sentinel links back to itself), destroying every node
/// (`node+8` through `vtable[8]`, flag 3), then resets the list
/// (`UIList_Reset`). The loop's back edge carries an event check.
pub fn ui_list_delete_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = lw_r(c, S1, 0x4);
        c.g[V0][0] = lw_r(c, A2, 0x4);
        let single = c.g[V0][0] == c.g[A2][0];
        if !single {
            c.cycles += 1;
            loop {
                c.cycles += 2;
                let sentinel = c.g[A2][0] == 0;
                c.g[S0][0] = lw_r(c, A2, 0x4);
                if !sentinel {
                    c.cycles += 6;
                    c.g[V0][0] = lw_r(c, A2, 0x8);
                    c.g[A1][0] = 0x3;
                    c.g[A0][0] = lh_r(c, V0, 0x8);
                    c.g[V1][0] = lw_r(c, V0, 0xc);
                    let target = low(c, V1);
                    c.g[RA][0] = sx(0x0039_7990);
                    c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
                    call_ptr(c, target, 0x0039_7990)?;
                }
                c.cycles += 4;
                c.g[A2][0] = c.g[S0][0];
                c.g[V0][0] = lw_r(c, A2, 0x4);
                let more = c.g[V0][0] != c.g[A2][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_7970);
                }
            }
        }
        c.cycles += 7;
        c.g[RA][0] = sx(0x0039_79a8);
        c.g[A0][0] = c.g[S1][0];
        callee::UI_LIST_RESET.call(c, 0x0039_79a8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_00347F90` (0x347f90): `FlexRail_dtor`. Installs vtable `0x4900d0`,
/// frees the two arrays at `+0x50` (`0x317e98`) and destroys the object at
/// `+0x58` (`vtable[8]`, flag 3) before running the world object destructor.
pub fn flex_rail_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(low(c, V0).wrapping_add(0xd0));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        sw_r(c, S0, 0xc, c.g[V0][0]);
        c.g[A0][0] = lw_r(c, S0, 0x50);
        let no_array = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_array {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0034_7fc4);
            call_ptr(c, 0x0031_7e98, 0x0034_7fc4)?;
        }
        c.cycles += 3;
        c.g[A2][0] = lw_r(c, S0, 0x58);
        let no_obj = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !no_obj {
            c.cycles += 5;
            c.g[V0][0] = Addr(low(c, A2)).lw(c);
            c.g[A0][0] = lh_r(c, V0, 0x8);
            c.g[V1][0] = lw_r(c, V0, 0xc);
            let target = low(c, V1);
            c.g[RA][0] = sx(0x0034_7fe4);
            c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0034_7fe4)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0034_7ff0);
        c.g[A1][0] = c.g[S1][0];
        callee::WORLD_OBJ_DTOR.call(c, 0x0034_7ff0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end state helpers.

/// `func_001B00B8` (0x1b00b8): `cFEStateChallengeReceivedPopup`. Makes sure
/// the network popup manager exists (`0x260f80`), refreshes it (`0x261460` /
/// `0x2613c8`), then posts the challenge message: `0x262768(manager, 1, 8,
/// this->+0x6d0, 0, 0)`. Signature: `void popup(this)`.
pub fn c_fe_state_challenge_received_popup_v32(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = lw_r(c, GP, 0x238);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let have = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        if !have {
            c.cycles += 2;
            c.g[RA][0] = sx(0x001b_00d8);
            call_ptr(c, 0x0026_0f80, 0x001b_00d8)?;
        }
        c.cycles += 4;
        c.g[RA][0] = sx(0x001b_00e0);
        c.g[A0][0] = lw_r(c, GP, 0x238);
        call_ptr(c, 0x0026_1460, 0x001b_00e0)?;
        let busy = c.g[V0][0] != 0;
        c.g[V0][0] = lw_r(c, GP, 0x238);
        if !busy {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001b_00f0);
            c.g[A0][0] = lw_r(c, GP, 0x238);
            call_ptr(c, 0x0026_13c8, 0x001b_00f0)?;
            c.g[V0][0] = lw_r(c, GP, 0x238);
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.g[A3][0] = lw_r(c, S0, 0x6d0);
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001b_0104);
            c.g[A0][0] = lw_r(c, GP, 0u32.wrapping_sub(0xc8));
            call_ptr(c, 0x0025_b800, 0x001b_0104)?;
            c.g[A3][0] = lw_r(c, S0, 0x6d0);
        }
        c.cycles += 10;
        c.g[A1][0] = 0x1;
        c.g[A0][0] = lw_r(c, GP, 0x238);
        c.g[A2][0] = 0x8;
        c.g[T0][0] = 0;
        c.g[RA][0] = sx(0x001b_0120);
        c.g[T1][0] = 0;
        call_ptr(c, 0x0026_2768, 0x001b_0120)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_001BCC30` (0x1bcc30): `cFEStateLobbyMenu::Init`. Creates the screen
/// widget named `0x464da0` in `this->+0x10` (`0x397cc0`), stores it at `+0x40`
/// and, if created, shows it (`0x39c870`) and calls the state's `vtable[0xe0]`
/// refresh. Signature: `void init(this)`.
pub fn c_fe_state_lobby_menu_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x4da0));
        c.g[RA][0] = sx(0x001b_cc54);
        c.g[S0][0] = lw_r(c, S1, 0x10);
        call_ptr(c, 0x0031_7670, 0x001b_cc54)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001b_cc68);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001b_cc68)?;
        let missing = c.g[V0][0] == 0;
        sw_r(c, S1, 0x40, c.g[V0][0]);
        if !missing {
            c.cycles += 9;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001b_cc80);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001b_cc80)?;
            c.g[V1][0] = lw_r(c, S1, 0x8);
            c.g[A0][0] = lh_r(c, V1, 0xe0);
            c.g[V0][0] = lw_r(c, V1, 0xe4);
            let target = low(c, V0);
            c.g[RA][0] = sx(0x001b_cc94);
            c.g[A0][0] = sx(low(c, S1).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x001b_cc94)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_001BDF60` (0x1bdf60): `cFEStateLobbyMenu::ShowRoomTab`. Hides all
/// room tabs (`0x1bdfd8`) and then walks the four tab slots at `this+0x730`
/// (each `+0x18` back-pointer compared with `a1`) showing the matching one
/// (`vtable[0x48]` with 1). The loop's back edge carries an event check.
pub fn c_fe_state_lobby_menu_show_room_tab(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x001b_df7c);
        c.g[S1][0] = c.g[A1][0];
        callee::LOBBY_MENU_HIDE_ROOM_TABS.call(c, 0x001b_df7c)?;
        c.g[A2][0] = sx(low(c, S0).wrapping_add(0x730));
        c.g[V1][0] = 0;
        loop {
            c.cycles += 3;
            c.g[V0][0] = lw_r(c, A2, 0u32.wrapping_sub(0x18));
            let other = c.g[S1][0] != c.g[V0][0];
            c.g[V1][0] = sx(low(c, V1).wrapping_add(1));
            if !other {
                c.cycles += 9;
                c.g[A2][0] = Addr(low(c, A2)).lw(c);
                c.g[A1][0] = 0x1;
                c.g[V0][0] = lw_r(c, A2, 0x8);
                c.g[A0][0] = lh_r(c, V0, 0x48);
                c.g[V1][0] = lw_r(c, V0, 0x4c);
                let target = low(c, V1);
                c.g[RA][0] = sx(0x001b_dfb0);
                c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
                call_ptr(c, target, 0x001b_dfb0)?;
                break;
            }
            c.cycles += 3;
            let more = (c.g[V1][0] as i64) < 4;
            c.g[V0][0] = more as u64;
            c.g[A2][0] = sx(low(c, A2).wrapping_add(0x4));
            if !more {
                c.cycles += 1;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001b_df88);
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_001C0FC0` (0x1c0fc0): `cFEStateNetQuickPlayPopup::Init`. Creates the
/// popup named `0x4657e0` in `this->+0x10`, stores it at `+0x40`, shows it
/// (`0x39c870`) and copies the 0x20-byte default string at `0x5308b8` into the
/// popup (`0x3e6448(dst, 0, 0x20)`). Signature: `void init(this)`.
pub fn c_fe_state_net_quick_play_popup_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x57e0));
        c.g[RA][0] = sx(0x001c_0fe4);
        c.g[S1][0] = lw_r(c, S0, 0x10);
        call_ptr(c, 0x0031_7670, 0x001c_0fe4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x001c_0ff8);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001c_0ff8)?;
        let missing = c.g[V0][0] == 0;
        sw_r(c, S0, 0x40, c.g[V0][0]);
        if !missing {
            c.cycles += 9;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001c_1010);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001c_1010)?;
            c.g[A0][0] = 0x53_0000;
            c.g[A1][0] = 0;
            c.g[A0][0] = sx(low(c, A0).wrapping_add(0x8b8));
            c.g[RA][0] = sx(0x001c_1024);
            c.g[A2][0] = 0x20;
            call_ptr(c, 0x003e_6448, 0x001c_1024)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_001E03F8` (0x1e03f8): `cFEPopupSelectMultiplayerMode::OnWidgetAction`.
/// Action 5 selects: `this->+0x48 = 1`, the "multiplayer selected" global
/// (`gp-0x171c`) becomes `widget->+0x18 > 0`, and the dialog at
/// `this->+0x10 + 0x18` is told (`0x39f190(dialog, 1)`). Action 6 clears
/// `this->+0x48` and tells the dialog with 0. Anything else returns.
pub fn c_fe_popup_select_multiplayer_mode_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x10));
        c.g[V1][0] = c.g[A0][0];
        c.sd(low(c, SP), c.g[RA][0]);
        c.g[V0][0] = 0x5;
        let five = c.g[A2][0] == c.g[V0][0];
        c.g[A3][0] = c.g[A1][0];
        if !five {
            c.cycles += 3;
            c.g[V0][0] = 0x6;
            let six = c.g[A2][0] == c.g[V0][0];
            c.g[RA][0] = c.ld(low(c, SP));
            if six {
                c.cycles += 6;
                c.g[A0][0] = lw_r(c, V1, 0x10);
                c.g[A1][0] = 0x1;
                sb_r(c, V1, 0x48, 0);
                c.g[RA][0] = sx(0x001e_0464);
                c.g[A0][0] = sx(low(c, A0).wrapping_add(0x18));
                call_ptr(c, 0x0039_f190, 0x001e_0464)?;
                c.g[RA][0] = c.ld(low(c, SP));
            } else {
                c.cycles += 2;
            }
        } else {
            c.cycles += 11;
            c.g[V0][0] = 0x1;
            c.g[A0][0] = lw_r(c, V1, 0x10);
            sb_r(c, V1, 0x48, c.g[V0][0]);
            c.g[A1][0] = 0x1;
            c.g[A0][0] = sx(low(c, A0).wrapping_add(0x18));
            c.g[V0][0] = lw_r(c, A3, 0x18);
            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
            c.g[RA][0] = sx(0x001e_0448);
            sw_r(c, GP, 0u32.wrapping_sub(0x171c), c.g[V0][0]);
            call_ptr(c, 0x0039_f190, 0x001e_0448)?;
            c.g[RA][0] = c.ld(low(c, SP));
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `func_001A98B8` (0x1a98b8): `NetFEState_MakePasswordMask`. Masks the
/// password at `this+0x5c` with `'*'`: every character of the `strlen`
/// password (at most 63) is overwritten with `0x2a`, the string is then
/// terminated at the index the loop stopped at, and a zero-length password
/// becomes a single space. Signature:
/// `void make_mask(NetFEState* this, const char* password)`.
pub fn net_fe_state_make_password_mask(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x001a_98d0);
        c.g[A0][0] = c.g[A1][0];
        call_ptr(c, 0x0041_6810, 0x001a_98d0)?;
        c.g[A0][0] = c.g[V0][0];
        let none = (c.g[A0][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[A2][0] = sx(low(c, S0).wrapping_add(0x5c));
            c.g[A3][0] = 0x2a;
            c.g[V1][0] = sx(low(c, A2).wrapping_add(low(c, A1)));
            loop {
                c.cycles += 4;
                c.g[A1][0] = sx(low(c, A1).wrapping_add(1));
                let more = (c.g[A1][0] as i64) < (c.g[A0][0] as i64);
                c.g[V0][0] = more as u64;
                sb_r(c, V1, 0, c.g[A3][0]);
                if !more {
                    break;
                }
                c.cycles += 3;
                let limit = (c.g[A1][0] as i64) < 63;
                c.g[V0][0] = limit as u64;
                if !limit {
                    break;
                }
                c.g[V1][0] = sx(low(c, A2).wrapping_add(low(c, A1)));
                if event_due(c) {
                    deliver_event_here(c, 0x001a_98e8);
                }
            }
        }
        c.cycles += 2;
        let terminated = c.g[A0][0] != 0;
        c.g[V0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
        if !terminated {
            c.cycles += 4;
            c.g[V0][0] = 0x20;
            c.g[A0][0] = 0x1;
            sb_r(c, S0, 0x5c, c.g[V0][0]);
            c.g[V0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld(low(c, SP));
        frame.restore(c, S0, 0x10);
        sb_r(c, V0, 0x5c, 0);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio and parser helpers.

/// `func_0029ED18` (0x29ed18): `SSXAudioSystem_StopRiderLoops`. Stops the
/// rider's wipeout loops and trick-meter sound, and when the bank monitor
/// count at `+0x59e4` is positive stops the handle at `+0x59e0` on the monitor
/// `*(this+0x118) + 0x1d8` (with the float argument 0 in f12), clearing the
/// count. Signature: `void stop_rider_loops(this)`.
pub fn ssx_audio_system_stop_rider_loops(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0029_ed2c);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0028_b1d8, 0x0029_ed2c)?;
        c.g[A1][0] = lw_r(c, V0, 0x28);
        c.g[RA][0] = sx(0x0029_ed38);
        c.g[A0][0] = c.g[S0][0];
        callee::AUDIO_STOP_WIPEOUT_LOOP.call(c, 0x0029_ed38)?;
        c.g[RA][0] = sx(0x0029_ed40);
        call_ptr(c, 0x0028_b1d8, 0x0029_ed40)?;
        c.g[A1][0] = lw_r(c, V0, 0x28);
        c.g[RA][0] = sx(0x0029_ed4c);
        c.g[A0][0] = c.g[S0][0];
        callee::AUDIO_STOP_WIPEOUT_LOOP2.call(c, 0x0029_ed4c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0029_ed58);
        c.g[A1][0] = 0;
        callee::AUDIO_STOP_TRICK_METER.call(c, 0x0029_ed58)?;
        c.g[V0][0] = lw_r(c, S0, 0x59e4);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A2][0] = 0x1;
        if !none {
            c.cycles += 7;
            c.g[V0][0] = lw_r(c, S0, 0x118);
            c.g[A1][0] = lw_r(c, S0, 0x59e0);
            c.g[A0][0] = Addr(low(c, V0)).lw(c);
            c.mtc1(0, 12);
            c.g[RA][0] = sx(0x0029_ed7c);
            c.g[A0][0] = sx(low(c, A0).wrapping_add(0x1d8));
            callee::BANK_MONITOR_STOP_HANDLE.call(c, 0x0029_ed7c)?;
            sw_r(c, S0, 0x59e4, 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_002A4C28` (0x2a4c28): `CrowdAudio_AddSampleFile`. Appends a sample
/// file: when fewer than five are registered (`this->+0`), copies/registers
/// `a1` with `a2` (`0x3e1908`), stores the handle at `this+4+4*n`, bumps the
/// count and returns 1 if the slot is non-null. Signature:
/// `int add(CrowdAudio* this, const char* file, int bank)`.
pub fn crowd_audio_add_sample_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.g[A1][0];
        c.g[V0][0] = lw_r(c, S0, 0);
        c.g[V0][0] = ((c.g[V0][0] as i64) < 5) as u64;
        let full = c.g[V0][0] == 0;
        c.g[A1][0] = c.g[A2][0];
        if full {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 13;
            c.g[RA][0] = sx(0x002a_4c54);
            call_ptr(c, 0x003e_1908, 0x002a_4c54)?;
            c.g[V1][0] = lw_r(c, S0, 0);
            c.g[A0][0] = sx(low(c, S0).wrapping_add(0x4));
            c.g[V1][0] = sx(low(c, V1) << 2);
            c.g[V1][0] = sx(low(c, A0).wrapping_add(low(c, V1)));
            sw_r(c, V1, 0, c.g[V0][0]);
            c.g[A1][0] = lw_r(c, S0, 0);
            c.g[V0][0] = sx(low(c, A1) << 2);
            c.g[A0][0] = sx(low(c, A0).wrapping_add(low(c, V0)));
            c.g[V1][0] = Addr(low(c, A0)).lw(c);
            let stored = c.g[V1][0] != 0;
            c.g[V1][0] = sx(low(c, A1).wrapping_add(0x1));
            if stored {
                c.cycles += 2;
                c.g[V0][0] = 0x1;
                sw_r(c, S0, 0, c.g[V1][0]);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_002A7A90` (0x2a7a90): `IniParser_SkipToEol`. Advances the parser
/// (`IniParser_GetChar`) until the character at `this->+8` is a carriage
/// return (0xd), a newline (0xa) or NUL; returns 1 when the buffer ended, 0
/// when a line terminator was consumed. The loop's back edge carries an event
/// check. Signature: `int skip_to_eol(IniParser* this)`.
pub fn ini_parser_skip_to_eol(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = lw_r(c, S0, 0x8);
        c.g[V0][0] = Addr(low(c, V1)).lb(c);
        let empty = c.g[V0][0] == 0;
        c.g[S2][0] = 0xd;
        if empty {
            c.cycles += 1;
            c.g[V0][0] = 0x1;
        } else {
            c.cycles += 2;
            c.g[S1][0] = 0xa;
            loop {
                c.cycles += 3;
                c.g[V0][0] = Addr(low(c, V1)).lb(c);
                if c.g[V0][0] == c.g[S2][0] {
                    c.g[V0][0] = 0;
                    break;
                }
                c.cycles += 2;
                let newline = c.g[V0][0] == c.g[S1][0];
                c.g[V0][0] = 0;
                if newline {
                    break;
                }
                c.cycles += 6;
                c.g[RA][0] = sx(0x002a_7adc);
                c.g[A0][0] = c.g[S0][0];
                callee::INI_PARSER_GET_CHAR.call(c, 0x002a_7adc)?;
                c.g[V1][0] = lw_r(c, S0, 0x8);
                c.g[V0][0] = Addr(low(c, V1)).lb(c);
                let more = c.g[V0][0] != 0;
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002a_7ac0);
                }
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

/// `func_002A7FF8` (0x2a7ff8): `IniParser_GetString`. Expects an `=`-style
/// token (`0x41aa88(this+0xc)`), then the string `"0x4a3740"` (`IniParser_Expect`),
/// then reads a quoted string and copies it to `a2` (`0x4166f4`); returns 1 on
/// success, 0 otherwise. Signature: `int get_string(IniParser* this, void*,
/// char* out)`.
pub fn ini_parser_get_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        c.g[RA][0] = sx(0x002a_8018);
        c.g[A0][0] = sx(low(c, S0).wrapping_add(0xc));
        call_ptr(c, 0x0041_aa88, 0x002a_8018)?;
        let failed = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !failed {
            c.cycles += 6;
            c.g[A1][0] = 0x4a_0000;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002a_8030);
            c.g[A1][0] = sx(low(c, A1).wrapping_add(0x3740));
            callee::INI_PARSER_EXPECT.call(c, 0x002a_8030)?;
            let failed = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if !failed {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002a_8040);
                c.g[A0][0] = c.g[S0][0];
                callee::INI_PARSER_READ_QUOTED_STRING.call(c, 0x002a_8040)?;
                let empty = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S1][0];
                if empty {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002a_8050);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0041_66f4, 0x002a_8050)?;
                    c.g[V0][0] = 0x1;
                }
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
// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, Rng, DATA, GAME_GP, RA as TEST_RA};

    const SEEDS: u32 = 64;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }
    fn h(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }
    fn b(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }

    /// The zeroed BE-library singleton at `gp-0x1f04` (else the FE helpers walk
    /// into the lazy disc load).
    fn be_library(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
    }

    /// The dummy world at `gp-0x848` with the race-manager chain.
    fn world(c: &mut Cpu) {
        let wld = DATA + 0x1000;
        w(c, GAME_GP.wrapping_sub(0x848), wld);
        w(c, wld + 0x84, DATA + 0x1200);
        w(c, DATA + 0x1200 + 0x20, DATA + 0x3000);
        w(c, DATA + 0x1200 + 0x28, DATA + 0x1400);
        w(c, DATA + 0x1200 + 0x78, 0);
        w(c, wld + 0x7c, DATA + 0x1500);
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, 0x40000)`: the game heap over test
    /// memory, for the paths that allocate.
    fn heap_init(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = TEST_RA as u64;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, TEST_RA as u32, "DLHeap_InitArena did not return");
    }

    /// A vtable slot `{delta, fn}` at `vt + off`.
    fn slot(c: &mut Cpu, vt: u32, off: u32, delta: i16, f: u32) {
        h(c, vt + off, delta as u16);
        w(c, vt + off + 4, f);
    }

    /// Run a guest helper during setup; returns its `v0`.
    fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = TEST_RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, TEST_RA as u32, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// `StrHash` (0x317670).
    fn str_hash(c: &mut Cpu, s: u32) -> u32 {
        run(c, 0x0031_7670, ee::s10::f_00317670, &[s as u64]) as u32
    }

    /// A NUL-terminated string at `a`.
    fn string(c: &mut Cpu, a: u32, s: &str) {
        for (i, ch) in s.bytes().enumerate() {
            b(c, a + i as u32, ch);
        }
        b(c, a + s.len() as u32, 0);
    }

    /// A vtable dispatch target: object `obj` with vtable `vt` whose slot `off`
    /// is a fake callee.
    fn fake_slot(c: &mut Cpu, obj: u32, vt: u32, off: u32) {
        w(c, obj, vt);
        slot(c, vt, off, 0, FAKE);
    }

    #[test]
    fn ui_state_ctor_2_matches() {
        check(ee::s13::f_0039e318__generated, ui_state_ctor_2, SEEDS, &|c, seed| {
            machine(c, seed, 0x39e318, 2);
            be_library(c);
            world(c);
            heap_init(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            c.g[A2][0] = (DATA + 0x900) as u64;
        });
    }

    #[test]
    fn pad_queue_shutdown_matches() {
        check(
            ee::s10::f_00326a68__generated,
            pad_queue_shutdown,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x326a68, 6);
                be_library(c);
                world(c);
                let q = DATA;
                c.g[A0][0] = q as u64;
                let n = seed % 4;
                w(c, q + 0x2ee8, n);
                for i in 0..n {
                    let e = q + 0x2f00 + i * 0x40;
                    let ptr = if (seed + i) % 3 == 0 { 0 } else { e };
                    w(c, q + 0x2eec + i * 4, ptr);
                    fake_slot(c, e, DATA + 0x3000, 8);
                }
            },
        );
    }

    #[test]
    fn cbx_string_alloc_buffer_matches() {
        check(
            ee::s10::f_00318040__generated,
            cbx_string_alloc_buffer,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x318040, 2);
                be_library(c);
                world(c);
                heap_init(c);
                c.g[A0][0] = (DATA + 0x100) as u64;
                let size = if seed % 4 == 0 { 0 } else { 1 + seed % 0x30 };
                c.g[A1][0] = size as u64;
            },
        );
    }

    #[test]
    fn cbx_string_left_matches() {
        check(ee::s10::f_003190a8__generated, cbx_string_left, SEEDS, &|c, seed| {
            machine(c, seed, 0x3190a8, 4);
            be_library(c);
            world(c);
            heap_init(c);
            let src = DATA + 0x400;
            string(c, src, "hello");
            w(c, src - 8, 5);
            w(c, DATA + 0x200, src);
            c.g[A0][0] = (DATA + 0x100) as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            c.g[A2][0] = match seed % 4 {
                0 => 0,
                1 => 2,
                2 => 0x10,
                _ => u32::MAX, // -1: "all"
            } as u64;
        });
    }

    #[test]
    fn ps2ai_visual_effects_man_dtor_matches() {
        check(
            ee::s09::f_002f0438__generated,
            ps2ai_visual_effects_man_dtor,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x2f0438, 3);
                be_library(c);
                world(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0; // no operator delete on a test pointer
                if seed % 3 == 0 {
                    w(c, DATA, 0);
                } else {
                    let obj = DATA + 0x400;
                    w(c, DATA, obj);
                    fake_slot(c, obj, DATA + 0x800, 8);
                }
            },
        );
    }

    #[test]
    fn mem_card_state_base_ctor_matches() {
        check(
            ee::s04::f_001d53b0__generated,
            mem_card_state_base_ctor,
            16,
            &|c, seed| {
                machine(c, seed, 0x1d53b0, 8);
                be_library(c);
                heap_init(c);
                world(c);
                c.g[A0][0] = (DATA + 0x2_0000) as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = (DATA + 0xc00) as u64;
            },
        );
    }

    #[test]
    fn playback_man_stop_playback_matches() {
        check(
            ee::s07::f_0026cd20__generated,
            playback_man_stop_playback,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x26cd20, 6);
                be_library(c);
                world(c);
                heap_init(c);
                c.g[A0][0] = DATA as u64;
                if seed % 2 == 0 {
                    w(c, DATA + 0x70, 0);
                } else {
                    w(c, DATA + 0x70, 1);
                    w(c, DATA + 0x60, DATA + 0x400);
                    w(c, DATA + 0x5c, DATA + 0x600);
                    w(c, DATA + 0x600 + 0x28, DATA + 0x700);
                    w(c, DATA + 0x600 + 0x84, DATA + 0x900);
                }
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v20_matches() {
        check(
            ee::s13::f_003aabd8__generated,
            cps2_chunk_resolve_v20,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x3aabd8, 2);
                be_library(c);
                world(c);
                let chunk = DATA;
                c.g[A1][0] = chunk as u64;
                w(c, chunk + 0xc, (seed * 0x11) & 0x70);
                let mut r = Rng(0x5a5a_0000 ^ seed as u64);
                w(c, chunk + 0x1c, r.edge_f32());
                w(c, chunk + 0x20, r.edge_f32());
                w(c, chunk + 0x24, r.edge_f32());
                c.f[0] = r.edge_f32();
            },
        );
    }

    #[test]
    fn start_screen_render_matches() {
        check(
            ee::s06::f_00232da8__generated,
            start_screen_render,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x232da8, 4);
                be_library(c);
                world(c);
                heap_init(c);
                let app = DATA + 0x2000;
                w(c, GAME_GP.wrapping_sub(0x854), app);
                c.g[A0][0] = DATA as u64;
                let vt = DATA + 0x3000;
                w(c, app + 0x10d8, vt);
                slot(c, vt, 0x88, 0, FAKE);
                slot(c, vt, 0xa0, 0, FAKE + 0x10);
                w(c, DATA + 0xc, DATA + 0x400);
            },
        );
    }

    #[test]
    fn c_game_load_screen_render_matches() {
        check(
            ee::s06::f_00233390__generated,
            c_game_load_screen_render,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x233390, 4);
                be_library(c);
                world(c);
                heap_init(c);
                let app = DATA + 0x2000;
                w(c, GAME_GP.wrapping_sub(0x854), app);
                c.g[A0][0] = DATA as u64;
                let vt = DATA + 0x3000;
                w(c, app + 0x10d8, vt);
                slot(c, vt, 0x88, 0, FAKE);
                slot(c, vt, 0xa0, 0, FAKE + 0x10);
                w(c, DATA + 0xc, DATA + 0x400);
            },
        );
    }

    #[test]
    fn c_fe_load_screen_render_matches() {
        check(
            ee::s06::f_002339f8__generated,
            c_fe_load_screen_render,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x2339f8, 4);
                be_library(c);
                world(c);
                heap_init(c);
                let app = DATA + 0x2000;
                w(c, GAME_GP.wrapping_sub(0x854), app);
                c.g[A0][0] = DATA as u64;
                let vt = DATA + 0x3000;
                w(c, app + 0x10d8, vt);
                slot(c, vt, 0x88, 0, FAKE);
                slot(c, vt, 0xa0, 0, FAKE + 0x10);
                w(c, DATA + 0xc, DATA + 0x400);
            },
        );
    }

    #[test]
    fn ui_font_set_free_fonts_matches() {
        check(
            ee::s13::f_00398438__generated,
            ui_font_set_free_fonts,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x398438, 4);
                be_library(c);
                world(c);
                let set = DATA;
                c.g[A0][0] = set as u64;
                let n = (seed % 4) as u8;
                b(c, set + 0x34, n);
                // Font objects are left null: the non-null path destroys them
                // through the fixed font destructor (0x3916c0), which frees the
                // pointer and would need a real heap allocation in the fixture.
                for i in 0..n as u32 {
                    let s = set + 4 + i * 0xc;
                    w(c, s, (seed + i) as u32);
                    w(c, s + 8, 0);
                }
            },
        );
    }

    #[test]
    fn ps2_exec_man_worker_thread_matches() {
        check(
            ee::s10::f_0031ac60__generated,
            ps2_exec_man_worker_thread,
            24,
            &|c, seed| {
                machine(c, seed, 0x31ac60, 4);
                be_library(c);
                world(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.pc = 0x0031_ac60;
                // the loop terminates when the "done" flag at +0x20 is set
                w(c, this + 0x20, 1);
                w(c, this + 0x10, DATA + 0x400);
                if seed % 2 == 0 {
                    let job = DATA + 0x800;
                    w(c, this + 0x1_0000 + 0x4048, job);
                    fake_slot(c, job, DATA + 0x2000, 0x10);
                }
            },
        );
    }

    #[test]
    fn modifier_block_forward_v43_matches() {
        check(
            ee::s11::f_00353020__generated,
            modifier_block_forward_v43,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x353020, 4);
                be_library(c);
                world(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = (DATA + 0x900) as u64;
                if seed % 2 == 0 {
                    w(c, DATA, 0);
                } else {
                    let child = DATA + 0x400;
                    w(c, DATA, child);
                    fake_slot(c, child, DATA + 0x2000, 0xc8);
                }
            },
        );
    }

    #[test]
    fn c_fe_state_career_stats_on_widget_action_matches() {
        check(
            ee::s04::f_001f5650__generated,
            c_fe_state_career_stats_on_widget_action,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1f5650, 4);
                be_library(c);
                world(c);
                heap_init(c);
                c.g[A0][0] = DATA as u64;
                let widget = DATA + 0x400;
                c.g[A1][0] = match seed % 3 {
                    0 => 0,
                    _ => widget as u64,
                };
                c.g[A2][0] = [5, 6, 7][(seed % 3) as usize] as u64;
                w(c, widget + 0x18, 0x10);
                let list = DATA + 0x600;
                w(c, DATA + 0x10, list);
                let owner = DATA + 0x800;
                w(c, list, owner);
                let vt = DATA + 0x2000;
                w(c, owner + 4, vt);
                slot(c, vt, 0x28, 0, FAKE);
            },
        );
    }

    #[test]
    fn float_rail_save_matches() {
        check(ee::s11::f_00349ad0__generated, float_rail_save, SEEDS, &|c, seed| {
            machine(c, seed, 0x349ad0, 4);
            be_library(c);
            world(c);
            let this = DATA;
            let arch = DATA + 0x400;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = arch as u64;
            w(c, arch, DATA + 0x2000);
            slot(c, DATA + 0x2000, 8, 0, FAKE);
            w(c, this + 0x20, seed % 5);
            w(c, this + 0x70, DATA + 0x900);
        });
    }

    #[test]
    fn spring_rail_save_matches() {
        check(ee::s11::f_0034ac10__generated, spring_rail_save, SEEDS, &|c, seed| {
            machine(c, seed, 0x34ac10, 4);
            be_library(c);
            world(c);
            let this = DATA;
            let arch = DATA + 0x400;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = arch as u64;
            w(c, arch, DATA + 0x2000);
            slot(c, DATA + 0x2000, 8, 0, FAKE);
            w(c, this + 0x20, seed % 5);
            w(c, this + 0x70, DATA + 0x900);
        });
    }

    #[test]
    fn spline_modifier_v05_matches() {
        check(
            ee::s11::f_00359eb8__generated,
            spline_modifier_v05,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x359eb8, 2);
                be_library(c);
                world(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = [0x190, 0x191, 0x192, 0x200][(seed % 4) as usize] as u64;
                let mut r = Rng(0x359e_0000 ^ seed as u64);
                w(c, DATA + 0x48, r.edge_f32());
            },
        );
    }

    #[test]
    fn multi_particle_remove_section_instances_matches() {
        check(
            ee::s11::f_00358140__generated,
            multi_particle_remove_section_instances,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x358140, 2);
                be_library(c);
                world(c);
                let this = DATA;
                let arr = DATA + 0x400;
                c.g[A0][0] = this as u64;
                let target = (seed % 3) as u64;
                c.g[A1][0] = target;
                let n = seed % 6;
                w(c, this + 4, n);
                w(c, this + 0xc, arr);
                let mut r = Rng(0x3581_0000 ^ seed as u64);
                for i in 0..n {
                    b(c, arr + i, (r.next_u32() % 3) as u8);
                }
            },
        );
    }

    #[test]
    fn ov_screen_session_on_activate_matches() {
        check(
            ee::s05::f_00208b00__generated,
            ov_screen_session_on_activate,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x208b00, 5);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA;
                let screen = DATA + 0x400;
                let arr = DATA + 0x500;
                let child = DATA + 0x600;
                // `StrHash` clobbers the argument registers: hash first.
                let hash = str_hash(c, 0x0047_0a20);
                c.g[A0][0] = this as u64;
                if seed % 2 == 0 {
                    w(c, this + 0xd4, 0);
                } else {
                    w(c, this + 0xd4, screen);
                    w(c, screen + 0x38, arr);
                    w(c, screen + 0x3c, arr + 8);
                    w(c, arr, 1);
                    w(c, arr + 8, child);
                    w(c, child + 0x38, hash);
                    w(c, child + 0x8, DATA + 0x2000);
                    slot(c, DATA + 0x2000, 0x38, 0, FAKE);
                    slot(c, DATA + 0x2000, 0x48, 0, FAKE + 0x10);
                }
            },
        );
    }

    #[test]
    fn ui_pair_update_anim_matches() {
        check(
            ee::s13::f_0039c1c8__generated,
            ui_pair_update_anim,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x39c1c8, 4);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = seed as u64;
                if seed % 2 == 0 {
                    w(c, this + 0x78, 0);
                } else {
                    // the pair child's vtable is the word after its next links
                    let a = DATA + 0x400;
                    w(c, this + 0x78, a);
                    w(c, a + 8, DATA + 0x2000);
                    slot(c, DATA + 0x2000, 0x98, 0, FAKE);
                }
                if seed % 3 == 0 {
                    w(c, this + 0x7c, 0);
                } else {
                    let b = DATA + 0x500;
                    w(c, this + 0x7c, b);
                    w(c, b + 8, DATA + 0x2000);
                    slot(c, DATA + 0x2000, 0x98, 0, FAKE);
                }
            },
        );
    }

    #[test]
    fn tex_man_alloc_nb_tex_info_matches() {
        check(
            ee::s12::f_00367150__generated,
            tex_man_alloc_nb_tex_info,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x367150, 2);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let node = DATA + 0x2000;
                w(c, this + 0x1f48, node);
                w(c, node, 0);
                w(c, this + 4, seed % 4);
            },
        );
    }

    #[test]
    fn ov_screen_enter_lodge_init_matches() {
        check(
            ee::s04::f_001f7198__generated,
            ov_screen_enter_lodge_init,
            16,
            &|c, seed| {
                machine(c, seed, 0x1f7198, 8);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = this as u64;
                w(c, this + 0x10, DATA + 0x4000);
            },
        );
    }

    #[test]
    fn ov_screen_autosave_v26_matches() {
        check(
            ee::s05::f_002122e8__generated,
            ov_screen_autosave_v26,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x2122e8, 5);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA;
                // `StrHash` clobbers the argument registers: hash first.
                let hash = str_hash(c, 0x0047_1e38);
                c.g[A0][0] = this as u64;
                w(c, this + 0x1c0, [0, 6, 0x11][(seed % 3) as usize]);
                if seed % 2 == 0 {
                    w(c, this + 0x40, 0);
                } else {
                    let screen = DATA + 0x400;
                    let arr = DATA + 0x500;
                    let child = DATA + 0x600;
                    w(c, this + 0x40, screen);
                    w(c, screen + 0x38, arr);
                    w(c, screen + 0x3c, arr + 8);
                    w(c, arr, 1);
                    w(c, arr + 8, child);
                    w(c, child + 0x38, hash);
                    w(c, child + 0x8, DATA + 0x2000);
                    slot(c, DATA + 0x2000, 0x48, 0, FAKE);
                }
            },
        );
    }

    #[test]
    fn fe_module_unload_matches() {
        check(ee::s03::f_001a2190__generated, fe_module_unload, 16, &|c, seed| {
            machine(c, seed, 0x1a2190, 8);
            be_library(c);
            heap_init(c);
            world(c);
            let app = DATA + 0x2000;
            w(c, GAME_GP.wrapping_sub(0x854), app);
            w(c, app + 0x10d8, DATA + 0x3000);
            slot(c, DATA + 0x3000, 0x98, 0, FAKE);
            c.g[A0][0] = (DATA + 0x2_0000) as u64;
        });
    }

    #[test]
    fn game_ui_can_open_pause_menu_matches() {
        check(
            ee::s05::f_0020cbe8__generated,
            game_ui_can_open_pause_menu,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x20cbe8, 5);
                be_library(c);
                heap_init(c);
                world(c);
                let race = DATA + 0x1200;
                if seed % 2 == 0 {
                    w(c, race + 0x48, 0);
                } else {
                    w(c, race + 0x48, DATA + 0x400);
                }
            },
        );
    }

    #[test]
    fn ov_screen_yn_dialog_on_widget_created_matches() {
        check(
            ee::s05::f_0020d4f0__generated,
            ov_screen_yn_dialog_on_widget_created,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x20d4f0, 5);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA;
                let widget = DATA + 0x400;
                // `StrHash` clobbers the argument registers: hash first.
                let want = if seed % 2 == 0 {
                    str_hash(c, 0x0046_e818)
                } else {
                    0x1234
                };
                c.g[A0][0] = this as u64;
                c.g[A1][0] = widget as u64;
                w(c, widget + 0x38, want);
                w(c, this + 0x9c, if seed % 3 == 0 { 6 } else { 5 });
            },
        );
    }

    #[test]
    fn game_module_end_replay_playback_matches() {
        check(
            ee::s06::f_002305c8__generated,
            game_module_end_replay_playback,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x2305c8, 4);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                w(c, this + 0x28, DATA + 0x400);
                w(c, this + 0xa4, DATA + 0x500);
                w(c, GAME_GP.wrapping_sub(0x69c), seed % 4);
            },
        );
    }

    #[test]
    fn ov_screen_hiscore_list_on_widget_action_matches() {
        check(
            ee::s04::f_001e9130__generated,
            ov_screen_hiscore_list_on_widget_action,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1e9130, 4);
                be_library(c);
                world(c);
                heap_init(c);
                c.g[A0][0] = DATA as u64;
                let widget = DATA + 0x400;
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { widget as u64 };
                c.g[A2][0] = [5, 6, 7][(seed % 3) as usize] as u64;
                w(c, widget + 0x18, if seed % 2 == 0 { 1 } else { 2 });
                let list = DATA + 0x600;
                w(c, DATA + 0x10, list);
                let owner = DATA + 0x800;
                w(c, list, owner);
                let vt = DATA + 0x2000;
                w(c, owner + 4, vt);
                slot(c, vt, 0x20, 0, FAKE);
            },
        );
    }

    #[test]
    fn mesh_anim_kill_restorable_matches() {
        check(
            ee::s11::f_00352708__generated,
            mesh_anim_kill_restorable,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x352708, 4);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
                let dead = match seed % 3 {
                    0 => 0,
                    1 => 2,
                    _ => 7,
                };
                w(c, this + 0x6b0, dead);
            },
        );
    }

        #[test]
    fn be_reset_net_interfaces_matches() {
        check(
            ee::s01::f_0014ea90__generated,
            be_reset_net_interfaces,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x14ea90, 6);
                world(c);
                be_library(c);
                c.g[A0][0] = DATA as u64;
            },
        );
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
                be_library(c);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                b(c, 0x0053_5c12, (seed % 6) as u8);
                if seed % 2 == 0 {
                    w(c, 0x0053_05f0, 0);
                } else {
                    let table = DATA + 0x600;
                    w(c, 0x0053_05f0, table);
                    let idx = (seed % 4) as u32;
                    w(c, table + idx * 0x14 + 4, seed % 7);
                }
            },
        );
    }

    #[test]
    fn dead_node_ctor_matches() {
        check(ee::s11::f_003506d8__generated, dead_node_ctor, SEEDS, &|c, seed| {
            machine(c, seed, 0x3506d8, 4);
            be_library(c);
            world(c);
            heap_init(c);
            let this = DATA;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            c.g[A2][0] = (DATA + 0x500) as u64;
            w(c, this + 0x18, DATA + 0x600);
        });
    }

    #[test]
    fn hide_ctor_matches() {
        check(ee::s11::f_00350e90__generated, hide_ctor, SEEDS, &|c, seed| {
            machine(c, seed, 0x350e90, 4);
            be_library(c);
            world(c);
            heap_init(c);
            let this = DATA;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            c.g[A2][0] = (DATA + 0x500) as u64;
            w(c, this + 0x18, DATA + 0x600);
        });
    }

    #[test]
    fn ui_list_delete_all_matches() {
        check(
            ee::s13::f_00397948__generated,
            ui_list_delete_all,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x397948, 4);
                be_library(c);
                world(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                if seed % 3 == 0 {
                    // empty list: the head links back to itself
                    w(c, this + 4, this + 4);
                    w(c, this + 8, this + 4);
                } else {
                    let n = 1 + seed % 3;
                    w(c, this + 4, this + 4);
                    w(c, this + 8, DATA + 0x400);
                    for i in 0..n {
                        let node = DATA + 0x400 + i * 0x100;
                        let next = if i + 1 == n { node } else { node + 0x100 };
                        w(c, node + 4, next);
                        w(c, node + 8, DATA + 0x2000);
                        slot(c, DATA + 0x2000, 8, 0, FAKE);
                    }
                }
            },
        );
    }

    #[test]
    fn flex_rail_dtor_matches() {
        check(ee::s11::f_00347f90__generated, flex_rail_dtor, SEEDS, &|c, seed| {
            machine(c, seed, 0x347f90, 4);
            be_library(c);
            world(c);
            heap_init(c);
            let this = DATA;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = 0;
            w(c, this + 0x50, 0); // no array free
            if seed % 2 == 0 {
                w(c, this + 0x58, 0);
            } else {
                let obj = DATA + 0x400;
                w(c, this + 0x58, obj);
                fake_slot(c, obj, DATA + 0x2000, 8);
            }
        });
    }

    #[test]
    fn floating_save_matches() {
        check(ee::s11::f_0034fa88__generated, floating_save, SEEDS, &|c, seed| {
            machine(c, seed, 0x34fa88, 4);
            be_library(c);
            world(c);
            let this = DATA;
            let arch = DATA + 0x400;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = arch as u64;
            w(c, arch, DATA + 0x2000);
            slot(c, DATA + 0x2000, 8, 0, FAKE);
            w(c, this + 0x78, (seed % 3) as u32);
            w(c, this + 0x70, DATA + 0x900);
        });
    }

    #[test]
    fn net_fe_state_make_password_mask_matches() {
        check(
            ee::s03::f_001a98b8__generated,
            net_fe_state_make_password_mask,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1a98b8, 2);
                be_library(c);
                world(c);
                let this = DATA;
                let s = DATA + 0x400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = s as u64;
                let text = ["", "abc", "abcdefgh", "0123456789abcdef"][(seed % 4) as usize];
                string(c, s, text);
            },
        );
    }

    #[test]
    fn c_fe_state_challenge_received_popup_v32_matches() {
        check(
            ee::s03::f_001b00b8__generated,
            c_fe_state_challenge_received_popup_v32,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1b00b8, 6);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                w(c, this + 0x6d0, DATA + 0x400);
                w(
                    c,
                    GAME_GP.wrapping_add(0x238),
                    if seed % 2 == 0 { 0 } else { DATA + 0x500 },
                );
                w(c, GAME_GP.wrapping_sub(0xc8), DATA + 0x600);
            },
        );
    }

    #[test]
    fn c_fe_state_lobby_menu_init_matches() {
        check(
            ee::s03::f_001bcc30__generated,
            c_fe_state_lobby_menu_init,
            16,
            &|c, seed| {
                machine(c, seed, 0x1bcc30, 8);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = this as u64;
                w(c, this + 0x10, DATA + 0x4000);
            },
        );
    }

    #[test]
    fn c_fe_state_lobby_menu_show_room_tab_matches() {
        check(
            ee::s03::f_001bdf60__generated,
            c_fe_state_lobby_menu_show_room_tab,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1bdf60, 4);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let tab = DATA + 0x400;
                c.g[A1][0] = if seed % 2 == 0 {
                    tab as u64
                } else {
                    0x1234_5678
                };
                for i in 0..4u32 {
                    w(
                        c,
                        this + 0x718 + i * 4,
                        if i as u32 == seed % 4 { tab } else { 0 },
                    );
                }
                w(c, tab + 8, DATA + 0x2000);
                slot(c, DATA + 0x2000, 0x48, 0, FAKE);
            },
        );
    }

    #[test]
    fn c_fe_state_net_quick_play_popup_init_matches() {
        check(
            ee::s03::f_001c0fc0__generated,
            c_fe_state_net_quick_play_popup_init,
            16,
            &|c, seed| {
                machine(c, seed, 0x1c0fc0, 8);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = this as u64;
                w(c, this + 0x10, DATA + 0x4000);
            },
        );
    }

    #[test]
    fn c_fe_state_stats_ctor_matches() {
        check(
            ee::s03::f_001c2d18__generated,
            c_fe_state_stats_ctor,
            16,
            &|c, seed| {
                machine(c, seed, 0x1c2d18, 6);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA + 0x4000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
            },
        );
    }

    #[test]
    fn c_fe_popup_select_multiplayer_mode_on_widget_action_matches() {
        check(
            ee::s04::f_001e03f8__generated,
            c_fe_popup_select_multiplayer_mode_on_widget_action,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1e03f8, 3);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA;
                let widget = DATA + 0x400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = widget as u64;
                c.g[A2][0] = [5, 6, 7][(seed % 3) as usize] as u64;
                w(c, this + 0x10, DATA + 0x600);
                w(c, widget + 0x18, (seed % 2) as u32);
            },
        );
    }

    #[test]
    fn net_event_e5_clone_matches() {
        check(
            ee::s07::f_002685e0__generated,
            net_event_e5_clone,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x2685e0, 3);
                be_library(c);
                world(c);
                heap_init(c);
                let src = DATA;
                c.g[A0][0] = src as u64;
                let s = DATA + 0x400;
                string(c, s, "event");
                w(c, src + 8, s);
                w(c, src + 0xc, seed);
            },
        );
    }

    #[test]
    fn ssx_audio_system_stop_rider_loops_matches() {
        check(
            ee::s08::f_0029ed18__generated,
            ssx_audio_system_stop_rider_loops,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x29ed18, 6);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA + 0x4000;
                c.g[A0][0] = this as u64;
                w(c, this + 0x118, DATA + 0x800);
                w(c, this + 0x800, DATA + 0x900);
                w(c, this + 0x59e0, seed % 8);
                w(
                    c,
                    this + 0x59e4,
                    if seed % 3 == 0 { 0 } else { 1 + seed % 3 },
                );
            },
        );
    }

        #[test]
    fn ini_parser_skip_to_eol_matches() {
        check(
            ee::s08::f_002a7a90__generated,
            ini_parser_skip_to_eol,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x2a7a90, 2);
                be_library(c);
                world(c);
                let this = DATA;
                let text = DATA + 0x400;
                c.g[A0][0] = this as u64;
                w(c, this + 8, text);
                let s = ["abc\ndef", "abcdef", "\r\nrest", "", "no terminator here"]
                    [(seed % 5) as usize];
                string(c, text, s);
            },
        );
    }

    #[test]
    fn ini_parser_get_string_matches() {
        check(
            ee::s08::f_002a7ff8__generated,
            ini_parser_get_string,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x2a7ff8, 4);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA;
                let stream = DATA + 0x400;
                let key = DATA + 0x500;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 0; // compared case-insensitively against `this+0xc`
                c.g[A2][0] = (DATA + 0x800) as u64;
                // `this+0xc` is the key the match compares against address 0 (so
                // it must be the empty string to pass); `this+8` is the character
                // stream. `ReadQuotedString`'s copy loop has no NUL check, so the
                // stream must always contain a closing quote.
                string(c, key, "");
                w(c, this + 0xc, key);
                w(c, this + 8, stream);
                match seed % 4 {
                    0 => string(c, stream, "\""),
                    1 => string(c, stream, "v\""),
                    2 => string(c, stream, "value\""),
                    _ => string(c, stream, "\"\""),
                }
            },
        );
    }
}
