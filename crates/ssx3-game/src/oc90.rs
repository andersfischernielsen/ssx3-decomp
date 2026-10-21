//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for fifty small front-end, option, sound, script, menu,
//! memcard and object helpers, each with a differential test against the
//! generated body.
//!
//! - **Front-end widget dispatch** (`cFEState*_OnWidgetNotify`,
//!   `OVScreenPauseFreeride_OnWidgetNotify`): each maps a widget id (and, for
//!   the audio options, the widget kind) onto one of the `WIDGET_*` results.
//! - **Option blocks** (`BENetworkInt_*`, `BEOptionInt_*`): fixed-size copies
//!   of a live option block onto the committed one, as gcc expands a struct
//!   assignment (word loop plus an overlapping `ldl`/`ldr` tail).
//! - **Pre/post-race visibility flags** (`BE*Race_Enter`/`Exit`): set or clear
//!   the `visible` word of a run of `BE*` singletons.
//! - **Comparators and predicates** (`GameAnimMap_CompareNameIndex`,
//!   `Avalanche_CompareFloat`/`CompareU16`, `AnimObject_IsTimeInWindow`).
//! - **Audio** (`PS2SoundManager_dtor`, `SpeechBank_dtor`,
//!   `SSXAudioSystem_*`, `SongInstance_GetMsToNextBeat`, `PATH_PlatFindBankSlot`,
//!   `DataInstanceMan_dtor`).
//! - **Script / menu / memcard / object** (`WScriptMan_DeactivateProcess`,
//!   `ScriptEngine_*`, `MenuSys*Item_*`, `MemCard_SjisToAsciiString`,
//!   `cPS2MemCard_*`, `ModObj_*`, `RiderSphereTree_Init`, `Boost_ctor_2`,
//!   `Floating_v15`, `PS2BezierMan_v22`, the `NetEvent*_Clone` nodes).
//!
//! Bodies that make calls follow [`crate::call`] (same frame, same registers
//! at each call, unwind check after it); loops that check events follow
//! [`crate::events`]. FPU code issues the original's `c.*_s` operations in the
//! same order, so `f0`/`f1` and the FPU condition bit end as the original
//! leaves them.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

#[rustfmt::skip]
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0019_52e8, fe_state_main_menu_on_widget_notify, "cFEStateMainMenu_OnWidgetNotify"),
    (0x0014_e130, be_network_int_commit, "BENetworkInt_Commit"),
    (0x0014_e1f8, be_network_int_reset, "BENetworkInt_Reset"),
    (0x001f_8908, ov_screen_pause_freeride_on_widget_notify, "OVScreenPauseFreeride_OnWidgetNotify"),
    (0x0038_b190, ps2_bezier_man_v22, "PS2BezierMan_v22"),
    (0x0018_59d8, fe_state_uber_trick_on_widget_notify, "cFEStateUberTrick_OnWidgetNotify"),
    (0x001f_3cc8, fe_state_lodge_on_widget_notify, "cFEStateLodge_OnWidgetNotify"),
    (0x001f_43d8, fe_state_lodge_rider_detail_on_widget_notify, "cFEStateLodgeRiderDetail_OnWidgetNotify"),
    (0x0018_6478, fe_options_credits_on_widget_notify, "cFEOptionsCredits_OnWidgetNotify"),
    (0x0014_f2a8, be_option_int_commit, "BEOptionInt_Commit"),
    (0x0014_f360, be_option_int_reset, "BEOptionInt_Reset"),
    (0x0031_4d60, game_anim_map_compare_name_index, "GameAnimMap_CompareNameIndex"),
    (0x0018_2c68, fe_state_rider_detail_on_widget_notify, "cFEStateRiderDetail_OnWidgetNotify"),
    (0x0019_6b08, fe_state_audio_options_on_widget_notify, "cFEStateAudioOptions_OnWidgetNotify"),
    (0x0015_64b0, be_post_race_exit, "BEPostRace_Exit"),
    (0x0015_6510, be_post_race_enter, "BEPostRace_Enter"),
    (0x0015_66e8, be_pre_race_exit, "BEPreRace_Exit"),
    (0x0015_6750, be_pre_race_enter, "BEPreRace_Enter"),
    (0x0034_f930, floating_v15, "Floating_v15"),
    (0x002d_5718, avalanche_compare_float, "Avalanche_CompareFloat"),
    (0x002d_5750, avalanche_compare_u16, "Avalanche_CompareU16"),
    (0x0019_55b8, fe_state_bonus_material_on_widget_notify, "cFEStateBonusMaterial_OnWidgetNotify"),
    (0x0034_ebe0, anim_object_is_time_in_window, "AnimObject_IsTimeInWindow"),
    (0x0036_10e0, mod_obj_get_matrix_at, "ModObj_GetMatrixAt"),
    (0x0029_2ae8, ssx_audio_system_kill_rider_timers, "SSXAudioSystem_KillRiderTimers"),
    (0x002a_4168, ssx_audio_system_is_track_group2_event, "SSXAudioSystem_IsTrackGroup2Event"),
    (0x002a_41d8, ssx_audio_system_is_track_group3_event, "SSXAudioSystem_IsTrackGroup3Event"),
    (0x002a_8c50, ps2_sound_manager_dtor, "PS2SoundManager_dtor"),
    (0x0027_d790, script_obj_cam_is_bound_to_process, "ScriptObjCam_IsBoundToProcess"),
    (0x002a_e230, path_plat_find_bank_slot, "PATH_PlatFindBankSlot"),
    (0x002a_f900, speech_bank_dtor, "SpeechBank_dtor"),
    (0x002b_2718, song_instance_get_ms_to_next_beat, "SongInstance_GetMsToNextBeat"),
    (0x002c_cf98, menu_sys_action_item_ctor_3, "MenuSysActionItem_ctor_3"),
    (0x002c_d7f0, menu_sys_int8_item_draw, "MenuSysInt8Item_Draw"),
    (0x002c_dc28, menu_sys_int_item_draw, "MenuSysIntItem_Draw"),
    (0x002c_2268, memcard_sjis_to_ascii_string, "MemCard_SjisToAsciiString"),
    (0x002c_4480, ps2_memcard_request_get_info, "cPS2MemCard_RequestGetInfo"),
    (0x002c_50e0, ps2_memcard_do_get_info, "cPS2MemCard_DoGetInfo"),
    (0x003a_ca70, wscript_cache_dtor, "tWScriptCache_dtor"),
    (0x0030_9aa0, wscript_man_deactivate_process, "WScriptMan_DeactivateProcess"),
    (0x0035_58b8, mod_obj_add_position_modifier, "ModObj_AddPositionModifier"),
    (0x0026_9dc0, net_event_fd_clone, "NetEventFD_Clone"),
    (0x0026_9e40, net_event_fe_clone, "NetEventFE_Clone"),
    (0x0028_f558, ssx_audio_system_set_selected_peak, "SSXAudioSystem_SetSelectedPeak"),
    (0x0028_2390, script_engine_stop_all_process_seqs, "ScriptEngine_StopAllProcessSeqs"),
    (0x0028_23f0, script_engine_kill_all_processes, "ScriptEngine_KillAllProcesses"),
    (0x0028_2798, script_engine_get_process_state, "ScriptEngine_GetProcessState"),
    (0x0034_1548, boost_ctor_2, "Boost_ctor_2"),
    (0x0032_99c8, rider_sphere_tree_init, "RiderSphereTree_Init"),
    (0x0027_58c0, data_instance_man_dtor, "DataInstanceMan_dtor"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const SCRIPT_ENGINE_GET_INSTANCE: Callee =
        Callee { addr: 0x0027_d2e8, f: ee::s07::f_0027d2e8 };
    pub const SCRIPT_ENGINE_BINDING_HAS_PROCESS: Callee =
        Callee { addr: 0x0027_d2c8, f: ee::s07::f_0027d2c8 };
    pub const MENU_SYS_ITEM_CTOR: Callee = Callee { addr: 0x002c_a258, f: ee::s09::f_002ca258 };
    pub const MENU_SYS_ITEM_DRAW_ROW: Callee = Callee { addr: 0x002c_a988, f: ee::s09::f_002ca988 };
    pub const MEMCARD_SJIS_CHAR_TO_ASCII: Callee =
        Callee { addr: 0x002c_2430, f: ee::s08::f_002c2430 };
    pub const PS2_MEMCARD_SET_STATE: Callee = Callee { addr: 0x002c_48c0, f: ee::s08::f_002c48c0 };
    pub const WSCRIPT_PROCESS_LIST_CONTAINS: Callee =
        Callee { addr: 0x0030_b428, f: ee::s10::f_0030b428 };
    pub const WSCRIPT_PROCESS_LIST_REMOVE: Callee =
        Callee { addr: 0x0030_b2d0, f: ee::s10::f_0030b2d0 };
    pub const WSCRIPT_PROCESS_LIST_PUSH_FRONT: Callee =
        Callee { addr: 0x0030_b260, f: ee::s10::f_0030b260 };
    pub const POSITION_MODIFIER_CTOR: Callee = Callee { addr: 0x0035_6eb8, f: ee::s11::f_00356eb8 };
    pub const MOD_OBJ_SET_TRANSFORM_MODIFIER: Callee =
        Callee { addr: 0x0035_54b0, f: ee::s11::f_003554b0 };
    pub const SSX_AUDIO_SYSTEM_RESTART_PEAK_MUSIC: Callee =
        Callee { addr: 0x0028_ef90, f: ee::s07::f_0028ef90 };
    pub const SCRIPT_ENGINE_STOP_PROCESS_SEQ: Callee =
        Callee { addr: 0x0028_2338, f: ee::s07::f_00282338 };
    pub const SCRIPT_ENGINE_KILL_PROCESS: Callee =
        Callee { addr: 0x0028_2540, f: ee::s07::f_00282540 };
    pub const SCRIPT_ENGINE_GET_PROCESS: Callee =
        Callee { addr: 0x0028_2cb0, f: ee::s07::f_00282cb0 };
    pub const SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS: Callee =
        Callee { addr: 0x0028_2908, f: ee::s07::f_00282908 };
    pub const WORLD_OBJ_CTOR_2: Callee = Callee { addr: 0x0034_fb90, f: ee::s11::f_0034fb90 };
    pub const SONG_INSTANCE_GET_BEAT_MS: Callee =
        Callee { addr: 0x002b_2690, f: ee::s08::f_002b2690 };
    pub const SONG_INSTANCE_GET_POSITION_MS: Callee =
        Callee { addr: 0x002b_2600, f: ee::s08::f_002b2600 };
    pub const SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP2: Callee =
        Callee { addr: 0x002a_41c8, f: ee::s08::f_002a41c8 };
    pub const SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP3: Callee =
        Callee { addr: 0x002a_4238, f: ee::s08::f_002a4238 };
}

mod addr {
    /// `cBE_getBE` (cross-shard direct call the generator emitted as
    /// `call_indirect`).
    pub const CBE_GET_BE: u32 = 0x0014_dc80;
    /// `cBE_getInterface` (0x14DD58).
    pub const CBE_GET_INTERFACE: u32 = 0x0014_dd58;
    /// `cBEInterface::getNetwork`-ish lookup at 0x144BC0.
    pub const BE_INTERFACE_LOOKUP: u32 = 0x0014_4bc0;
    /// `BankMonitor_TimerKill` caller at 0x2ADDE0.
    pub const BANK_MONITOR_TIMER_RESET: u32 = 0x002a_dde0;
    /// `free` on a `SpeechBank` voice block.
    pub const SPEECH_BANK_FREE_CALLBACK: u32 = 0x003d_6ca0;
    /// `func_002523A8` (UI engine teardown).
    pub const UI_ENGINE_TEARDOWN: u32 = 0x0025_23a8;
    /// `sprintf`.
    pub const SPRINTF: u32 = 0x0041_7828;
    /// `fmodf`.
    pub const FMODF: u32 = 0x0040_d8d0;
    /// `cMemMan_alloc(size, vtable, flags, align)`.
    pub const MEM_MAN_ALLOC: u32 = 0x0031_7d70;
    /// `operator delete`.
    pub const MEM_MAN_FREE: u32 = 0x0031_7e50;
    /// `cMemMan_free`.
    pub const MEM_MAN_FREE_NOSIZE: u32 = 0x0031_7e98;
    /// The McManager query behind `cPS2MemCard_DoGetInfo`.
    pub const MC_MANAGER_GET_INFO: u32 = 0x0040_a498;
}

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `base + off` as an effective address (`addu` of the low 32 bits).
#[inline(always)]
fn ea(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `lwc1 fN, off(base)`.
#[inline(always)]
fn ldf_at(c: &mut Cpu, n: usize, base: u64, off: u32) {
    c.lwc1(n, ea(base, off));
}

/// `swc1 fN, off(base)`.
#[inline(always)]
fn stf_at(c: &mut Cpu, n: usize, base: u64, off: u32) {
    c.swc1(n, ea(base, off));
}

/// A `gp`-relative global address.
#[inline(always)]
fn gp_rel(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `lw rt, gp+off`.
#[inline(always)]
fn gp_lw(c: &mut Cpu, off: i32) -> u64 {
    c.lw(gp_rel(c, off))
}

/// `sw src, gp+off`.
#[inline(always)]
fn gp_sw(c: &mut Cpu, off: i32, v: u64) {
    c.sw(gp_rel(c, off), v);
}

/// The front-end widget-notify results.
const WIDGET_IGNORED: u64 = 0x101;
const WIDGET_ACTIVATED: u64 = 0x100;

// ---------------------------------------------------------------------------
// Option-block copies (a struct assignment of a fixed size).

/// Copy `size` bytes from `v1` to `a0` the way gcc expands a known-size
/// `memcpy`: 8-byte words while `v1 != v0`, then an overlapping `ldl`/`ldr`
/// tail. `v1`, `v0` (the end address) and `a0` are the original's loop
/// registers and are left with the values the epilogue reads.
fn copy_block(c: &mut Cpu, head: u32, tail_cycles: u64, tail_pairs: u32) {
    if (c.g[V1][0] | c.g[A0][0]) & 7 == 0 {
        let end = c.g[V0][0];
        loop {
            c.cycles += 11;
            c.g[A1][0] = c.ld(ea(c.g[V1][0], 0));
            c.g[A2][0] = c.ld(ea(c.g[V1][0], 8));
            c.g[A3][0] = c.ld(ea(c.g[V1][0], 0x10));
            c.g[T0][0] = c.ld(ea(c.g[V1][0], 0x18));
            c.sd(ea(c.g[A0][0], 0), c.g[A1][0]);
            c.sd(ea(c.g[A0][0], 8), c.g[A2][0]);
            c.sd(ea(c.g[A0][0], 0x10), c.g[A3][0]);
            c.sd(ea(c.g[A0][0], 0x18), c.g[T0][0]);
            c.g[V1][0] = sx(ea(c.g[V1][0], 0x20));
            let more = c.g[V1][0] != end;
            c.g[A0][0] = sx(ea(c.g[A0][0], 0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, head);
            }
        }
    } else {
        let end = c.g[V0][0];
        loop {
            c.cycles += 19;
            c.ldl(5, ea(c.g[V1][0], 7));
            c.ldr(5, ea(c.g[V1][0], 0));
            c.ldl(6, ea(c.g[V1][0], 0xf));
            c.ldr(6, ea(c.g[V1][0], 8));
            c.ldl(7, ea(c.g[V1][0], 0x17));
            c.ldr(7, ea(c.g[V1][0], 0x10));
            c.ldl(8, ea(c.g[V1][0], 0x1f));
            c.ldr(8, ea(c.g[V1][0], 0x18));
            c.sdl(5, ea(c.g[A0][0], 7));
            c.sdr(5, ea(c.g[A0][0], 0));
            c.sdl(6, ea(c.g[A0][0], 0xf));
            c.sdr(6, ea(c.g[A0][0], 8));
            c.sdl(7, ea(c.g[A0][0], 0x17));
            c.sdr(7, ea(c.g[A0][0], 0x10));
            c.sdl(8, ea(c.g[A0][0], 0x1f));
            c.sdr(8, ea(c.g[A0][0], 0x18));
            c.g[V1][0] = sx(ea(c.g[V1][0], 0x20));
            let more = c.g[V1][0] != end;
            c.g[A0][0] = sx(ea(c.g[A0][0], 0x20));
            if !more {
                c.cycles += 2;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, head);
            }
        }
    }
    c.cycles += tail_cycles;
    c.ldl(2, ea(c.g[V1][0], 7));
    c.ldr(2, ea(c.g[V1][0], 0));
    if tail_pairs == 2 {
        c.ldl(5, ea(c.g[V1][0], 0xf));
        c.ldr(5, ea(c.g[V1][0], 8));
    }
    c.sdl(2, ea(c.g[A0][0], 7));
    c.sdr(2, ea(c.g[A0][0], 0));
    if tail_pairs == 2 {
        c.sdl(5, ea(c.g[A0][0], 0xf));
        c.sdr(5, ea(c.g[A0][0], 8));
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Front-end widget notifications.

/// `int cFEStateMainMenu_OnWidgetNotify(cFEStateMainMenu* self, void* widget,
/// int id, void* data)` (0x1952E8): a `Back`/`Continue` widget reports whether
/// the selection it points at (`+0x48`) is null.
pub fn fe_state_main_menu_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 6;
    if c.g[A2][0] == c.g[V0][0] {
        c.g[A0][0] = c.lw(ea(c.g[A0][0], 0x48));
    } else {
        c.cycles += 3;
        c.g[V0][0] = 9;
        let is_nine = c.g[A2][0] == c.g[V0][0];
        c.g[V0][0] = WIDGET_IGNORED;
        if is_nine {
            c.cycles += 2;
            c.g[V0][0] = WIDGET_ACTIVATED;
            abi::ret(c);
            return;
        }
        // (the original's `li t, 1; beqz` tail always returns here)
        c.cycles += 4;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.g[V1][0] = WIDGET_IGNORED;
    c.g[V0][0] = 1;
    if c.g[A0][0] != 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    abi::ret(c);
}

/// `void* BENetworkInt_Commit()` (0x14E130): copy the live network-option
/// block (0x534B30) over the committed one at 0x535088.
pub fn be_network_int_commit(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x530000;
    c.g[V1][0] = 0x530000;
    c.g[A0][0] = sx(ea(c.g[V0][0], 0x5088));
    c.g[V1][0] = sx(ea(c.g[V1][0], 0x4b30));
    c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
    c.g[V0][0] &= 7;
    c.g[V0][0] = sx(ea(c.g[V1][0], 0x4a0));
    copy_block(c, 0x0014_e150, 9, 2);
}

/// `void* BENetworkInt_Reset()` (0x14E1F8): copy the committed network-option
/// block back over the live one.
pub fn be_network_int_reset(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x530000;
    c.g[V1][0] = 0x530000;
    c.g[A0][0] = sx(ea(c.g[V0][0], 0x4b30));
    c.g[V1][0] = sx(ea(c.g[V1][0], 0x5088));
    c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
    c.g[V0][0] &= 7;
    c.g[V0][0] = sx(ea(c.g[V1][0], 0x4a0));
    copy_block(c, 0x0014_e218, 9, 2);
}

/// `int OVScreenPauseFreeride_OnWidgetNotify(...)` (0x1F8908): `Resume`
/// (6) answers 0, `Restart`/`Quit` (8, 9) answer 0x100, anything else
/// 0x101.
pub fn ov_screen_pause_freeride_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 6;
    let is_six = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = (c.g[A2][0] < 6) as u64;
    if is_six {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let below_six = c.g[V0][0] != 0;
    c.g[V0][0] = (c.g[A2][0] < 0xa) as u64;
    if !below_six {
        c.cycles += 2;
        let at_least_ten = c.g[V0][0] == 0;
        c.g[V0][0] = (c.g[A2][0] < 8) as u64;
        if at_least_ten {
            c.cycles += 2;
            c.g[V0][0] = WIDGET_IGNORED;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        let below_eight = c.g[V0][0] != 0;
        c.g[V0][0] = WIDGET_ACTIVATED;
        if below_eight {
            c.cycles += 2;
            c.g[V0][0] = WIDGET_IGNORED;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = WIDGET_IGNORED;
    abi::ret(c);
}

/// `void PS2BezierMan_v22(PS2BezierMan* self, void* obj, u8* types, void*
/// extra)` (0x38B190): queue a bezier patch. Four equal type bytes go to the
/// per-size list for that size (4, 6 or 8); a mixed patch goes to the mix list
/// with kind 2 when any type is 8, else 0.
pub fn ps2_bezier_man_v22(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A3][0] = c.lbu(ea(c.g[A2][0], 0));
    c.g[V0][0] = c.lbu(ea(c.g[A2][0], 2));
    let mut uniform = c.g[A3][0] == c.g[V0][0];
    if uniform {
        c.cycles += 4;
        c.g[V1][0] = c.lbu(ea(c.g[A2][0], 1));
        c.g[V0][0] = c.lbu(ea(c.g[A2][0], 3));
        uniform = c.g[V1][0] == c.g[V0][0];
        if uniform {
            c.cycles += 2;
            uniform = c.g[A3][0] == c.g[V1][0];
            if !uniform {
                c.g[A3][0] = c.lw(ea(c.g[A0][0], 0x4d84));
            }
        } else {
            c.g[A3][0] = c.lw(ea(c.g[A0][0], 0x4d84));
        }
    } else {
        c.g[A3][0] = c.lw(ea(c.g[A0][0], 0x4d84));
    }
    if uniform {
        // One list per patch size: 4 (+0x458, 900 entries), 6 (+0x3c9c,
        // 220) and 8 (+0x4a60, 50); the entry is 16 bytes at the list base.
        c.cycles += 4;
        c.g[A2][0] = c.g[A3][0];
        c.g[V0][0] = 6;
        let is_six = c.g[A2][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[A2][0] as i64) < 7) as u64;
        if is_six {
            c.cycles += 4;
            c.g[V1][0] = c.lw(ea(c.g[A0][0], 0x3c9c));
            c.g[V0][0] = ((c.g[V1][0] as i64) < 220) as u64;
            let full = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
            if full {
                c.cycles += 2;
                abi::ret(c);
                return;
            }
            c.cycles += 11;
            c.g[V1][0] = sx(ea(c.g[V1][0], 1));
            c.g[V0][0] = sx(ea(c.g[V0][0], 0x3ca0));
            c.sw(ea(c.g[A0][0], 0x3c9c), c.g[V1][0]);
            c.g[V0][0] = sx(ea(c.g[A0][0], c.g[V0][0] as u32));
            c.g[V1][0] = 1;
            c.sw(ea(c.g[V0][0], 4), c.g[T0][0]);
            c.sw(ea(c.g[V0][0], 8), c.g[V1][0]);
            c.sw(ea(c.g[V0][0], 0), c.g[A1][0]);
            c.g[V1][0] = c.lhu(ea(c.g[A1][0], 0x1a8));
            c.sh(ea(c.g[V0][0], 0xc), c.g[V1][0]);
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        let at_least_seven = c.g[V0][0] == 0;
        c.g[V0][0] = 4;
        if at_least_seven {
            c.cycles += 3;
            c.g[V0][0] = 8;
            if c.g[A2][0] == c.g[V0][0] {
                c.g[V1][0] = c.lw(ea(c.g[A0][0], 0x4a60));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 50) as u64;
                let full = c.g[V0][0] == 0;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
                if full {
                    c.cycles += 2;
                    abi::ret(c);
                    return;
                }
                c.cycles += 11;
                c.g[V1][0] = sx(ea(c.g[V1][0], 1));
                c.g[V0][0] = sx(ea(c.g[V0][0], 0x4a64));
                c.sw(ea(c.g[A0][0], 0x4a60), c.g[V1][0]);
                c.g[V0][0] = sx(ea(c.g[A0][0], c.g[V0][0] as u32));
                c.g[V1][0] = 2;
                c.sw(ea(c.g[V0][0], 4), c.g[T0][0]);
                c.sw(ea(c.g[V0][0], 8), c.g[V1][0]);
                c.sw(ea(c.g[V0][0], 0), c.g[A1][0]);
                c.g[V1][0] = c.lhu(ea(c.g[A1][0], 0x1aa));
                c.sh(ea(c.g[V0][0], 0xc), c.g[V1][0]);
                abi::ret(c);
                return;
            }
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        if c.g[A2][0] == c.g[V0][0] {
            c.g[V1][0] = c.lw(ea(c.g[A0][0], 0x458));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 900) as u64;
            let full = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
            if full {
                c.cycles += 2;
                abi::ret(c);
                return;
            }
            c.cycles += 10;
            c.g[V1][0] = sx(ea(c.g[V1][0], 1));
            c.g[V0][0] = sx(ea(c.g[V0][0], 0x45c));
            c.sw(ea(c.g[A0][0], 0x458), c.g[V1][0]);
            c.g[V0][0] = sx(ea(c.g[A0][0], c.g[V0][0] as u32));
            c.sw(ea(c.g[V0][0], 4), c.g[T0][0]);
            c.sw(ea(c.g[V0][0], 0), c.g[A1][0]);
            c.sw(ea(c.g[V0][0], 8), 0);
            c.g[V1][0] = c.lhu(ea(c.g[A1][0], 0x1a6));
            c.sh(ea(c.g[V0][0], 0xc), c.g[V1][0]);
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    // Mixed patch: capacity 75, kind 2 when any type is 8.
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A3][0] as i64) < 75) as u64;
    let full = c.g[V0][0] == 0;
    c.g[V0][0] = 0x14;
    if full {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    let mut kind;
    c.cycles += 11;
    c.g[V1][0] = sx(ea(c.g[A3][0], 1));
    // `mult a3, v0` (the generator routes the LO product through v0).
    c.mult(0, V0, A3, V0);
    c.sw(ea(c.g[A0][0], 0x4d84), c.g[V1][0]);
    c.g[A3][0] = 8;
    c.g[V0][0] = sx(ea(c.g[V0][0], 0x4d88));
    c.g[V1][0] = sx(ea(c.g[A0][0], c.g[V0][0] as u32));
    c.sw(ea(c.g[V1][0], 4), c.g[T0][0]);
    c.sw(ea(c.g[V1][0], 0), c.g[A1][0]);
    c.g[V0][0] = c.lbu(ea(c.g[A2][0], 0));
    let is_eight0 = c.g[V0][0] == c.g[A3][0];
    c.g[A0][0] = sx(ea(c.g[V1][0], 8));
    if is_eight0 {
        c.cycles += 1;
        kind = 2;
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.lbu(ea(c.g[A2][0], 1));
        let is_eight1 = c.g[V0][0] == c.g[A3][0];
        c.g[V0][0] = 2;
        if is_eight1 {
            kind = 2;
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lbu(ea(c.g[A2][0], 2));
            let is_eight2 = c.g[V0][0] == c.g[A3][0];
            c.g[V0][0] = 2;
            if is_eight2 {
                kind = 2;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lbu(ea(c.g[A2][0], 3));
                let not_eight3 = c.g[V0][0] != c.g[A3][0];
                c.g[V0][0] = 0;
                if not_eight3 {
                    kind = 0;
                } else {
                    c.cycles += 1;
                    kind = 2;
                }
            }
        }
    }
    c.g[V0][0] = kind;
    c.cycles += 5;
    c.sw(ea(c.g[A0][0], 0), c.g[V0][0]);
    c.g[T0][0] = sx(ea(c.g[V1][0], 0x10));
    c.g[A3][0] = 0;
    c.g[V0][0] = c.lhu(ea(c.g[A1][0], 0x1aa));
    c.sh(ea(c.g[V1][0], 0xc), c.g[V0][0]);
    loop {
        c.cycles += 7;
        c.g[V0][0] = sx(ea(c.g[A2][0], c.g[A3][0] as u32));
        c.g[A0][0] = sx(ea(c.g[T0][0], c.g[A3][0] as u32));
        c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
        c.g[A3][0] = sx(ea(c.g[A3][0], 1));
        c.g[V0][0] = ((c.g[A3][0] as i64) < 4) as u64;
        let more = c.g[V0][0] != 0;
        c.sb(ea(c.g[A0][0], 0), c.g[V1][0]);
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0038_b310);
        }
    }
}

/// `int cFEStateUberTrick_OnWidgetNotify(...)` (0x1859D8): `Ok` (9) answers
/// 0x100 once the widget belongs to the current page (`+0x50`), `Next`/`Prev`
/// (8, 9) on the overview page; everything else 0x101.
pub fn fe_state_uber_trick_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw(ea(c.g[A0][0], 0x50));
    let other_page = c.g[A1][0] != c.g[V0][0];
    c.g[V0][0] = (c.g[A2][0] < 0xa) as u64;
    if other_page {
        c.cycles += 2;
        let below_ten = c.g[V0][0] != 0;
        c.g[V1][0] = (c.g[A2][0] < 8) as u64;
        if !below_ten {
            c.cycles += 2;
            let below_eight = c.g[V1][0] != 0;
            c.g[V0][0] = WIDGET_ACTIVATED;
            if !below_eight {
                c.cycles += 2;
                abi::ret(c);
                return;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = WIDGET_IGNORED;
    } else {
        c.cycles += 3;
        c.g[V0][0] = 9;
        let not_nine = c.g[A2][0] != 9;
        c.g[V0][0] = WIDGET_IGNORED;
        if !not_nine {
            c.cycles += 2;
            c.g[V0][0] = WIDGET_ACTIVATED;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// The two-key widget predicates (0x1F3CC8, 0x1F43D8, 0x1955B8): ids at or
/// above the second threshold are ignored (0x101); below the first threshold
/// also 0x101; in between answers 0x100.
fn widget_range_notify(c: &mut Cpu, low: u64) {
    c.cycles += 3;
    c.g[V1][0] = (c.g[A2][0] < 0xa) as u64;
    let above = c.g[V1][0] == 0;
    c.g[V0][0] = WIDGET_IGNORED;
    if !above {
        c.cycles += 4;
        c.g[A0][0] = (c.g[A2][0] < low) as u64;
        c.g[V1][0] = WIDGET_IGNORED;
        c.g[V0][0] = WIDGET_ACTIVATED;
        if c.g[A0][0] != 0 {
            c.g[V0][0] = c.g[V1][0];
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int cFEStateLodge_OnWidgetNotify(...)` (0x1F3CC8): ids 7..9 answer 0x100.
pub fn fe_state_lodge_on_widget_notify(c: &mut Cpu) {
    widget_range_notify(c, 7);
}

/// `int cFEStateLodgeRiderDetail_OnWidgetNotify(...)` (0x1F43D8): ids 8..9
/// answer 0x100.
pub fn fe_state_lodge_rider_detail_on_widget_notify(c: &mut Cpu) {
    widget_range_notify(c, 8);
}

/// `int cFEStateBonusMaterial_OnWidgetNotify(...)` (0x1955B8): ids 8..9 answer
/// 0x100.
pub fn fe_state_bonus_material_on_widget_notify(c: &mut Cpu) {
    widget_range_notify(c, 8);
}

/// `int cFEOptionsCredits_OnWidgetNotify(...)` (0x186478): `Ok`/`Next`/`Prev`
/// (6, 8, 9) answer 0x100, everything else 0x101.
pub fn fe_options_credits_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 6;
    let is_six = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = (c.g[A2][0] < 6) as u64;
    if !is_six {
        c.cycles += 2;
        let below_six = c.g[V0][0] != 0;
        c.g[V0][0] = (c.g[A2][0] < 0xa) as u64;
        if !below_six {
            c.cycles += 2;
            let at_least_ten = c.g[V0][0] == 0;
            c.g[V0][0] = (c.g[A2][0] < 8) as u64;
            if at_least_ten {
                c.cycles += 2;
                c.g[V0][0] = WIDGET_IGNORED;
                abi::ret(c);
                return;
            }
            c.cycles += 2;
            let below_eight = c.g[V0][0] != 0;
            if below_eight {
                c.cycles += 2;
                c.g[V0][0] = WIDGET_IGNORED;
                abi::ret(c);
                return;
            }
            // 8 <= id < 10
            c.cycles += 2;
            c.g[V0][0] = WIDGET_ACTIVATED;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        c.g[V0][0] = WIDGET_IGNORED;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = WIDGET_ACTIVATED;
    abi::ret(c);
}

/// `void* BEOptionInt_Commit()` (0x14F2A8): copy the live option block at
/// 0x535610 over the committed one at 0x535898 (0x280 bytes).
pub fn be_option_int_commit(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x530000;
    c.g[V1][0] = 0x530000;
    c.g[A0][0] = sx(ea(c.g[V0][0], 0x5898));
    c.g[V1][0] = sx(ea(c.g[V1][0], 0x5610));
    c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
    c.g[V0][0] &= 7;
    c.g[V0][0] = sx(ea(c.g[V1][0], 0x280));
    copy_block(c, 0x0014_f2c8, 5, 1);
}

/// `void* BEOptionInt_Reset()` (0x14F360): copy the committed option block
/// back over the live one.
pub fn be_option_int_reset(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x530000;
    c.g[V1][0] = 0x530000;
    c.g[A0][0] = sx(ea(c.g[V0][0], 0x5610));
    c.g[V1][0] = sx(ea(c.g[V1][0], 0x5898));
    c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
    c.g[V0][0] &= 7;
    c.g[V0][0] = sx(ea(c.g[V1][0], 0x280));
    copy_block(c, 0x0014_f380, 5, 1);
}

/// `int GameAnimMap_CompareNameIndex(int* a, int* b)` (0x314D60): the qsort
/// comparator over the name table at `gp+0xD88`, ordering by index.
pub fn game_anim_map_compare_name_index(c: &mut Cpu) {
    c.cycles += 14;
    c.g[A3][0] = c.lw(c.g[A0][0] as u32);
    c.g[T0][0] = 1;
    c.g[A2][0] = c.lw(c.g[A1][0] as u32);
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    c.g[A0][0] = gp_lw(c, 0xd88);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
    c.g[A3][0] = sx(ea(c.g[A3][0], c.g[A0][0] as u32));
    c.g[A2][0] = sx(ea(c.g[A2][0], c.g[A0][0] as u32));
    c.g[V1][0] = c.lw(c.g[A3][0] as u32);
    c.g[A0][0] = c.lw(c.g[A2][0] as u32);
    c.g[V1][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
    if c.g[V1][0] == 0 {
        c.g[V0][0] = c.g[T0][0];
    }
    abi::ret(c);
}

/// `int cFEStateRiderDetail_OnWidgetNotify(...)` (0x182C68): the `Ok` widget
/// is 9.
pub fn fe_state_rider_detail_on_widget_notify(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] ^= 9;
    c.g[V1][0] = WIDGET_ACTIVATED;
    c.g[V0][0] = WIDGET_IGNORED;
    if c.g[A2][0] == 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    abi::ret(c);
}

/// `int cFEStateAudioOptions_OnWidgetNotify(cFEStateAudioOptions* self,
/// void* widget, int id, int kind)` (0x196B08): the volume sliders (kind
/// 2..4) answer 0x10 when their channel is idle, `Back`/`Ok` (8, 9) answer 0.
pub fn fe_state_audio_options_on_widget_notify(c: &mut Cpu) {
    /// The `0x101` result of a rejected widget.
    fn rejected(c: &mut Cpu) {
        c.cycles += 2;
        c.g[V0][0] = WIDGET_IGNORED;
        abi::ret(c);
    }
    /// The `0x10` result of an accepted slider.
    fn accepted(c: &mut Cpu) {
        c.cycles += 2;
        c.g[V0][0] = 0x10;
        abi::ret(c);
    }
    /// The tail both non-slider outcomes share (`a2 - 8 < 2` is `Back`/`Ok`).
    fn not_a_slider(c: &mut Cpu) {
        c.cycles += 3;
        c.g[V1][0] = (c.g[V1][0] < 2) as u64;
        let back_or_ok = c.g[V1][0] != 0;
        c.g[V0][0] = 0;
        if back_or_ok {
            c.cycles += 2;
            abi::ret(c);
        } else {
            c.cycles += 1;
            c.g[V0][0] = WIDGET_IGNORED;
            c.cycles += 2;
            abi::ret(c);
        }
    }

    c.cycles += 3;
    c.g[V0][0] = 6;
    if c.g[A2][0] != c.g[V0][0] {
        c.g[V1][0] = sx(ea(c.g[A2][0], 0xffff_fff8));
        return not_a_slider(c);
    }
    c.g[V1][0] = sx(ea(c.g[A2][0], 0xffff_fff8));
    c.cycles += 4;
    c.g[V0][0] = sx(ea(c.g[A3][0], 0xffff_fffe));
    c.g[V0][0] = (c.g[V0][0] < 3) as u64;
    let not_a_channel = c.g[V0][0] == 0;
    c.g[V0][0] = 2;
    if not_a_channel {
        return not_a_slider(c);
    }
    // Channel 2: the first slider is idle when +0x7C is clear.
    c.cycles += 2;
    let is_channel_two = c.g[A3][0] == c.g[V0][0];
    c.g[V0][0] = 3;
    if is_channel_two {
        c.cycles += 3;
        c.g[V0][0] = c.lw(ea(c.g[A0][0], 0x7c));
        let idle = c.g[V0][0] == 0;
        c.g[V0][0] = 3;
        if idle {
            return accepted(c);
        }
    }
    // Channel 3.
    c.cycles += 2;
    let is_channel_three = c.g[A3][0] == c.g[V0][0];
    c.g[V0][0] = 4;
    if is_channel_three {
        c.cycles += 3;
        c.g[V0][0] = c.lw(ea(c.g[A0][0], 0x7c));
        let idle = c.g[V0][0] == 0;
        c.g[V0][0] = 4;
        if idle {
            return accepted(c);
        }
    }
    // Channel 4 (both sliders idle).
    c.cycles += 2;
    let is_channel_four = c.g[A3][0] == c.g[V0][0];
    c.g[V0][0] = WIDGET_IGNORED;
    if is_channel_four {
        c.cycles += 3;
        c.g[V0][0] = c.lw(ea(c.g[A0][0], 0x74));
        let busy = c.g[V0][0] != 0;
        c.g[V0][0] = WIDGET_IGNORED;
        if !busy {
            c.cycles += 3;
            c.g[V0][0] = c.lw(ea(c.g[A0][0], 0x7c));
            let busy = c.g[V0][0] != 0;
            c.g[V0][0] = WIDGET_IGNORED;
            if !busy {
                return accepted(c);
            }
        }
    }
    rejected(c);
}

// ---------------------------------------------------------------------------
// Pre/post-race visibility flags.

/// `void BEPostRace_Exit()` (0x1564B0) and friends: clear (or set) the
/// `visible` word at `+8` of a run of `BE*` singletons. The loads are
/// software-pipelined, one register ahead of the stores.
fn be_race_flags(c: &mut Cpu, value: u64, loads: &[(usize, i32)]) {
    c.g[loads[0].0][0] = gp_lw(c, loads[0].1);
    for k in 1..loads.len() {
        c.g[loads[k].0][0] = gp_lw(c, loads[k].1);
        let prev = loads[k - 1].0;
        c.sw(ea(c.g[prev][0], 8), value);
    }
    let last = loads[loads.len() - 1].0;
    c.sw(ea(c.g[last][0], 8), value);
}

pub fn be_post_race_exit(c: &mut Cpu) {
    c.cycles += 23;
    be_race_flags(
        c,
        0,
        &[
            (V0, -0x1f30),
            (V1, -0x1ee8),
            (A0, -0x1f2c),
            (V0, -0x1ee4),
            (V1, -0x1ed4),
            (A0, -0x1eec),
            (V0, -0x1f28),
            (V1, -0x1ee0),
            (A0, -0x1edc),
            (A1, -0x1ea4),
            (V0, -0x1e8c),
        ],
    );
    abi::ret(c);
}

pub fn be_post_race_enter(c: &mut Cpu) {
    c.cycles += 24;
    c.g[V0][0] = 1;
    be_race_flags(
        c,
        1,
        &[
            (V1, -0x1f30),
            (A0, -0x1ee8),
            (A1, -0x1f2c),
            (V1, -0x1ee4),
            (A0, -0x1ed4),
            (A1, -0x1eec),
            (V1, -0x1f28),
            (A0, -0x1ee0),
            (A1, -0x1edc),
            (A2, -0x1ea4),
            (V1, -0x1e8c),
        ],
    );
    abi::ret(c);
}

pub fn be_pre_race_exit(c: &mut Cpu) {
    c.cycles += 25;
    be_race_flags(
        c,
        0,
        &[
            (V0, -0x1f3c),
            (V1, -0x1f30),
            (A0, -0x1ee8),
            (V0, -0x1f2c),
            (V1, -0x1ee4),
            (A0, -0x1ed4),
            (V0, -0x1eec),
            (V1, -0x1f28),
            (A0, -0x1ee0),
            (A1, -0x1edc),
            (V1, -0x1ea4),
            (V0, -0x1e8c),
        ],
    );
    abi::ret(c);
}

pub fn be_pre_race_enter(c: &mut Cpu) {
    c.cycles += 26;
    c.g[V0][0] = 1;
    be_race_flags(
        c,
        1,
        &[
            (V1, -0x1f3c),
            (A0, -0x1f30),
            (A1, -0x1ee8),
            (V1, -0x1f2c),
            (A0, -0x1ee4),
            (A1, -0x1ed4),
            (V1, -0x1eec),
            (A0, -0x1f28),
            (A1, -0x1ee0),
            (A2, -0x1edc),
            (A0, -0x1ea4),
            (V1, -0x1e8c),
        ],
    );
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Comparators and predicates.

/// `float Floating_v15(Floating* self)` (0x34F930): one step of the
/// three-axis damped spring used by `sFloatTilt` (board float), then clamp the
/// lean angles to +-80 degrees and clear the inputs.
pub fn floating_v15(c: &mut Cpu) {
    c.cycles += 68;
    let a0 = A0;
    ldf_at(c, 14, c.g[a0][0], 0x50); // pos1
    c.g[V1][0] = sx(ea(c.g[a0][0], 0x2c));
    ldf_at(c, 2, c.g[a0][0], 0x2c); // k
    ldf_at(c, 0, c.g[a0][0], 0x4c); // rest1
    ldf_at(c, 1, c.g[a0][0], 0x38); // input1
    c.neg_s(2, 2);
    c.sub_s(0, 14, 0);
    ldf_at(c, 13, c.g[a0][0], 0x64); // vel2
    c.neg_s(1, 1);
    ldf_at(c, 12, c.g[a0][0], 0x54); // vel1
    ldf_at(c, 7, c.g[a0][0], 0x40); // gain
    ldf_at(c, 3, c.g[a0][0], 0x60); // k2
    c.mul_s(2, 2, 0);
    c.mul_s(5, 1, 13);
    ldf_at(c, 4, c.g[a0][0], 0x3c); // damp
    c.g[V0][0] = gp_lw(c, 0x2a74); // the frame counter object
    c.mul_s(3, 7, 3);
    ldf_at(c, 9, c.g[a0][0], 0x44); // input2
    c.mul_s(1, 1, 12);
    ldf_at(c, 0, c.g[a0][0], 0x70); // pos3
    c.neg_s(4, 4);
    ldf_at(c, 8, c.g[a0][0], 0x34); // pos2
    ldf_at(c, 15, c.g[V0][0], 0x10); // frame delta
    c.cvt_s_w(15, 15);
    ldf_at(c, 6, c.g[a0][0], 0x30); // damp2
    c.mul_s(7, 7, 0);
    ldf_at(c, 11, c.g[a0][0], 0x6c); // vel3
    c.mul_s(8, 8, 9);
    c.neg_s(6, 6);
    ldf_at(c, 9, c.g[a0][0], 0x48); // input3
    ldf_at(c, 10, c.g[a0][0], 0x5c); // rest2
    c.add_s(1, 1, 3);
    c.g[AT][0] = 0x3f80_0000; // 1.0f
    c.mtc1(1, 0);
    c.mul_s(3, 4, 11);
    c.mul_s(4, 4, 10);
    c.div_s(0, 0, 15); // dt = 1 / fps
    c.add_s(2, 2, 8);
    c.add_s(5, 5, 7);
    c.lwc1(8, gp_rel(c, -0x2a98)); // +80 degrees
    c.mul_s(6, 6, 9);
    c.lwc1(7, gp_rel(c, -0x2a94)); // -80 degrees
    c.add_s(1, 1, 4);
    c.add_s(5, 5, 3);
    c.add_s(2, 2, 6);
    c.mul_s(3, 9, 0);
    c.mul_s(4, 10, 0);
    c.mul_s(2, 2, 0);
    c.mul_s(5, 5, 0);
    c.mul_s(1, 1, 0);
    c.mul_s(0, 11, 0);
    c.add_s(14, 14, 3);
    c.add_s(9, 9, 2);
    c.add_s(13, 13, 0);
    c.add_s(12, 12, 4);
    stf_at(c, 14, c.g[a0][0], 0x50);
    c.add_s(10, 10, 1);
    stf_at(c, 9, c.g[a0][0], 0x48);
    c.add_s(11, 11, 5);
    stf_at(c, 13, c.g[a0][0], 0x64);
    stf_at(c, 12, c.g[a0][0], 0x54);
    stf_at(c, 10, c.g[a0][0], 0x5c);
    stf_at(c, 11, c.g[a0][0], 0x6c);
    ldf_at(c, 0, c.g[V1][0], 0x28);
    c.c_le_s(7, 0);
    let keep = !c.fcc();
    c.mov_s(15, 7);
    if !keep {
        c.cycles += 1;
        c.min_s(15, 0, 8);
    }
    stf_at(c, 15, c.g[a0][0], 0x54);
    c.cycles += 6;
    ldf_at(c, 0, c.g[V1][0], 0x38);
    c.c_le_s(7, 0);
    let keep = !c.fcc();
    c.mov_s(1, 7);
    if !keep {
        c.cycles += 1;
        c.min_s(1, 0, 8);
    }
    c.cycles += 9;
    stf_at(c, 1, c.g[a0][0], 0x64);
    c.g[V1][0] = c.lhu(ea(c.g[a0][0], 0x12));
    c.g[V0][0] = 1;
    c.g[V1][0] |= 1;
    c.sw(ea(c.g[a0][0], 0x44), 0);
    c.sh(ea(c.g[a0][0], 0x12), c.g[V1][0]);
    c.sw(ea(c.g[a0][0], 0x60), 0);
    c.sw(ea(c.g[a0][0], 0x70), 0);
    abi::ret(c);
}

/// `int Avalanche_CompareFloat(float* a, float* b)` (0x2D5718): -1/0/1.
pub fn avalanche_compare_float(c: &mut Cpu) {
    c.cycles += 6;
    c.lwc1(1, c.g[A0][0] as u32);
    c.lwc1(0, c.g[A1][0] as u32);
    c.c_lt_s(1, 0);
    if c.fcc() {
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    } else {
        c.cycles += 4;
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.g[V0][0] = 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int Avalanche_CompareU16(u16* a, u16* b)` (0x2D5750): -1/0/1.
pub fn avalanche_compare_u16(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A0][0] = c.lhu(c.g[A0][0] as u32);
    c.g[V1][0] = c.lhu(c.g[A1][0] as u32);
    c.g[V0][0] = (c.g[A0][0] < c.g[V1][0]) as u64;
    let not_less = c.g[V0][0] == 0;
    c.g[V0][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
    if not_less {
        c.cycles += 2;
        abi::ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        abi::ret(c);
    }
}

/// `int AnimObject_IsTimeInWindow(AnimObject* self, float t)` (0x34EBE0): is
/// `t` inside the object's [start, end) play window? A start of 1e10 means
/// "no window"; a window that ends before it starts wraps.
pub fn anim_object_is_time_in_window(c: &mut Cpu) {
    c.cycles += 6;
    c.lwc1(0, gp_rel(c, -0x2aa0)); // the "no window" sentinel (1e10)
    c.lwc1(1, ea(c.g[A0][0], 8)); // start
    c.c_eq_s(1, 0);
    let no_window = c.fcc();
    c.g[V0][0] = 0;
    if no_window {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.lwc1(0, ea(c.g[A0][0], 0xc)); // end
    c.c_lt_s(1, 0);
    if !c.fcc() {
        // start >= end: the window wraps, so [end, start) is outside.
        c.cycles += 4;
        c.c_le_s(0, 12);
        c.g[V0][0] = 0;
        if c.fcc() {
            c.cycles += 4;
            c.c_lt_s(12, 1);
            c.g[V0][0] = 1;
            if c.fcc() {
                c.cycles += 2;
                abi::ret(c);
                return;
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    // start < end: start <= t < end.
    c.cycles += 3;
    c.c_le_s(1, 12);
    if c.fcc() {
        c.cycles += 3;
        c.c_lt_s(12, 0);
        if c.fcc() {
            c.cycles += 2;
            c.g[V0][0] = 1;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Objects, audio and script.

/// `void* ModObj_GetMatrixAt(ModObj* self, int index)` (0x3610E0): the matrix
/// element array at `+0x44`, or the transform modifier's matrix when bit 0 of
/// the flags at `+0x26` is set.
pub fn mod_obj_get_matrix_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = c.lhu(ea(c.g[S0][0], 0x26));
        c.g[V0][0] &= 1;
        let has_modifier = c.g[V0][0] != 0;
        c.g[A1][0] = sx(ea(c.g[S0][0], 0x14));
        if has_modifier {
            c.cycles += 5;
            c.g[V0][0] = c.lw(ea(c.g[S0][0], 0x20));
            c.g[A0][0] = c.lh(ea(c.g[V0][0], 0x198));
            c.g[V1][0] = c.lw(ea(c.g[V0][0], 0x19c));
            let method = c.g[V1][0] as u32;
            c.g[A0][0] = sx(ea(c.g[A1][0], c.g[A0][0] as u32));
            call_ptr(c, method, 0x0036_111c)?;
        }
        c.cycles += 8;
        c.g[V1][0] = c.lw(ea(c.g[S0][0], 0x44));
        c.g[V0][0] = sx((c.g[S1][0] as u32) << 6);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        c.g[V0][0] = sx(ea(c.g[V1][0], c.g[V0][0] as u32));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_KillRiderTimers(SSXAudioSystem* self, int unused)`
/// (0x292AE8): reset the bank monitor's timer through its vtable slot 4 and
/// then run the monitor's tick reset.
pub fn ssx_audio_system_kill_rider_timers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(ea(c.g[S0][0], 0x118));
        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
        c.g[V1][0] = c.lw(ea(c.g[A0][0], 0xab0));
        c.g[A0][0] = sx(ea(c.g[A0][0], 0x1d8));
        c.g[V0][0] = c.lh(ea(c.g[V1][0], 0x20));
        c.g[A2][0] = c.lw(ea(c.g[V1][0], 0x24));
        let method = c.g[A2][0] as u32;
        c.g[A0][0] = sx(ea(c.g[A0][0], c.g[V0][0] as u32));
        call_ptr(c, method, 0x0029_2b18)?;
        c.g[V1][0] = c.lw(ea(c.g[S0][0], 0x118));
        c.g[V0][0] = 0x4a0000;
        c.ldl(5, ea(c.g[V0][0], 0x36af));
        c.ldr(5, ea(c.g[V0][0], 0x36a8));
        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
        c.g[A0][0] = sx(ea(c.g[A0][0], 0x1d8));
        call_ptr(c, addr::BANK_MONITOR_TIMER_RESET, 0x0029_2b34)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool SSXAudioSystem_IsTrackGroup2Event(SSXAudioSystem* self, int id)`
/// (0x2A4168): is `id` in the given track group, or is that group's flag set?
/// `ret0` is the first call's return address; the others follow at +0xC, +0x14
/// and +0x20 (the next instruction of each call in the original).
fn is_track_group_event(c: &mut Cpu, group: u8, ret0: u32) -> crate::call::Flow {
    c.cycles += 15;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    call_ptr(c, addr::CBE_GET_BE, ret0)?;
    c.g[A1][0] = 0;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, addr::CBE_GET_INTERFACE, ret0 + 0xc)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, addr::BE_INTERFACE_LOOKUP, ret0 + 0x14)?;
    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
    c.g[A0][0] = c.g[S0][0];
    if group == 2 {
        callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP2.call(c, ret0 + 0x20)?;
    } else {
        callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP3.call(c, ret0 + 0x20)?;
    }
    if c.g[V0][0] != 0 {
        c.g[V0][0] = 1;
    } else {
        c.cycles += 4;
        c.g[V1][0] = 0x530000;
        c.g[V0][0] = c.lb(ea(c.g[V1][0], 0x5c10));
        c.g[V0][0] ^= group as u64;
        c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    }
    c.cycles += 4;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `bool SSXAudioSystem_IsTrackGroup2Event(...)` (0x2A4168).
pub fn ssx_audio_system_is_track_group2_event(c: &mut Cpu) {
    guest(c, |c| is_track_group_event(c, 2, 0x002a_417c))
}

/// `bool SSXAudioSystem_IsTrackGroup3Event(...)` (0x2A41D8).
pub fn ssx_audio_system_is_track_group3_event(c: &mut Cpu) {
    guest(c, |c| is_track_group_event(c, 3, 0x002a_41ec))
}

/// `void* PS2SoundManager_dtor(PS2SoundManager* self, int flags)` (0x2A8C50):
/// install the vtable, free the owned buffer at `+4`, and delete the object
/// when bit 0 of `flags` is set.
pub fn ps2_sound_manager_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x480000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(ea(c.g[V0][0], 0x35f8));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(ea(c.g[S0][0], 0), c.g[V0][0]);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = c.lw(ea(c.g[S0][0], 4));
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, addr::MEM_MAN_FREE_NOSIZE, 0x002a_8c84)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, addr::MEM_MAN_FREE, 0x002a_8c98)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool ScriptObjCam_IsBoundToProcess(ScriptObjCam* self, int process)`
/// (0x27D790): does the binding at `+0x14` hold `process`? No binding is
/// false.
pub fn script_obj_cam_is_bound_to_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(ea(c.g[S0][0], 0x14));
        let unbound = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if unbound {
            c.cycles += 2;
            c.g[V0][0] = 0;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 9;
            callee::SCRIPT_ENGINE_GET_INSTANCE.call(c, 0x0027_d7b8)?;
            c.g[V1][0] = c.lw(ea(c.g[S0][0], 0x14));
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = c.lw(ea(c.g[V1][0], 4));
            callee::SCRIPT_ENGINE_BINDING_HAS_PROCESS.call(c, 0x0027_d7cc)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PATH_PlatFindBankSlot(int bank, int id)` (0x2AE230): index of the
/// first of the 24 bank slots (0x4D3EF8, 0x28 bytes each) that matches, or -8.
pub fn path_plat_find_bank_slot(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x4d0000;
    c.g[V1][0] = 0;
    c.g[A2][0] = sx(0x4d_3ef8);
    loop {
        let found;
        c.cycles += 2;
        let bank_non_negative = (c.g[A0][0] as i64) >= 0;
        c.g[V0][0] = c.lw(ea(c.g[A2][0], 8));
        if bank_non_negative {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A0][0] {
                c.g[V1][0] = sx(ea(c.g[V1][0], 1));
                found = false;
            } else {
                c.cycles += 2;
                if (c.g[A1][0] as i64) < 0 {
                    found = true;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    if c.g[V0][0] == c.g[A1][0] {
                        found = true;
                    } else {
                        c.cycles += 1;
                        c.g[V1][0] = sx(ea(c.g[V1][0], 1));
                        found = false;
                    }
                }
            }
        } else {
            c.cycles += 2;
            if (c.g[V0][0] as i64) >= 0 {
                c.g[V1][0] = sx(ea(c.g[V1][0], 1));
                found = false;
            } else {
                found = true;
            }
        }
        if found {
            c.cycles += 2;
            c.g[V0][0] = c.g[V1][0];
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 24) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = sx(ea(c.g[A2][0], 0x28));
        if !more {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_fff8;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002a_e240);
        }
    }
}

/// `void SpeechBank_dtor(SpeechBank* self, int flags)` (0x2AF900): release the
/// voice block at `+0`, free the stream at `+0x44` when the flag at `+0x48` is
/// set, and delete the object when bit 0 of `flags` is set.
pub fn speech_bank_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
        call_ptr(c, addr::SPEECH_BANK_FREE_CALLBACK, 0x002a_f920)?;
        c.g[V0][0] = c.lw(ea(c.g[S0][0], 0x48));
        let no_stream = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if !no_stream {
            c.cycles += 3;
            c.g[A0][0] = c.lw(ea(c.g[S0][0], 0x44));
            call_ptr(c, addr::UI_ENGINE_TEARDOWN, 0x002a_f934)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, addr::MEM_MAN_FREE, 0x002a_f948)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float SongInstance_GetMsToNextBeat(SongInstance* self)` (0x2B2718): the
/// beat length minus the position modulo it, i.e. the time to the next beat.
pub fn song_instance_get_ms_to_next_beat(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x28);
        frame.save_fpr(c, 20, 0x20);
        callee::SONG_INSTANCE_GET_BEAT_MS.call(c, 0x002b_2734)?;
        c.mov_s(21, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::SONG_INSTANCE_GET_POSITION_MS.call(c, 0x002b_2740)?;
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(20, 0);
        callee::SONG_INSTANCE_GET_BEAT_MS.call(c, 0x002b_274c)?;
        c.mov_s(12, 20);
        c.mov_s(13, 0);
        call_ptr(c, addr::FMODF, 0x002b_2758)?;
        c.sub_s(0, 21, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `cMenuItem* MenuSysActionItem_ctor_3(cMenuItem* self, char* text, int a2,
/// int a3)` (0x2CCF98): the base constructor plus the action item's vtable and
/// two fields.
pub fn menu_sys_action_item_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A3][0];
        callee::MENU_SYS_ITEM_CTOR.call(c, 0x002c_cfbc)?;
        c.g[V1][0] = 0x480000;
        c.sw(ea(c.g[S0][0], 0x14), c.g[S1][0]);
        c.g[V1][0] = sx(ea(c.g[V1][0], 0x6d40));
        c.sw(ea(c.g[S0][0], 0x1c), c.g[S2][0]);
        c.sw(ea(c.g[S0][0], 0x10), c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw(ea(c.g[S0][0], 0x20), 0);
        c.sw(ea(c.g[S0][0], 0x18), 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysInt8Item_Draw(MenuSysInt8Item* self, void* gfx)` (0x2CD7F0):
/// format the byte value at `*(self+0x18)` into a stack buffer, then draw the
/// row.
pub fn menu_sys_int8_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x70);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[V0][0] = c.lw(ea(c.g[S0][0], 0x18));
        c.g[A1][0] = sx(ea(c.g[A1][0], 0x3970));
        c.g[A2][0] = c.lb(c.g[V0][0] as u32);
        call_ptr(c, addr::SPRINTF, 0x002c_d820)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.lw(ea(c.g[A0][0], 0x14));
        c.g[A3][0] = 0;
        c.g[T0][0] = c.g[SP][0];
        callee::MENU_SYS_ITEM_DRAW_ROW.call(c, 0x002c_d838)?;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysIntItem_Draw(MenuSysIntItem* self, void* gfx)` (0x2CDC28):
/// the same for the 32-bit value at `*(self+0x18)`.
pub fn menu_sys_int_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x70);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[V0][0] = c.lw(ea(c.g[S0][0], 0x18));
        c.g[A1][0] = sx(ea(c.g[A1][0], 0x3970));
        c.g[A2][0] = c.lw(c.g[V0][0] as u32);
        call_ptr(c, addr::SPRINTF, 0x002c_dc58)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.lw(ea(c.g[A0][0], 0x14));
        c.g[A3][0] = 0;
        c.g[T0][0] = c.g[SP][0];
        callee::MENU_SYS_ITEM_DRAW_ROW.call(c, 0x002c_dc70)?;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCard_SjisToAsciiString(char* dst, char* src)` (0x2C2268): convert
/// the 16-bit SJIS string at `src` into single-byte characters at `dst`.
///
/// The original is a resumable loop (a delivered event may unwind with `pc`
/// at the conversion body), so the entry point also accepts that resume
/// address, as the generated `match pc` does.
pub fn memcard_sjis_to_ascii_string(c: &mut Cpu) {
    const ENTRY: u32 = 0x002c_2268;
    const CONVERT: u32 = 0x002c_2288;
    const TEST: u32 = 0x002c_22a0;
    const EXIT: u32 = 0x002c_22ac;

    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 7;
                    let frame = Frame::open(c, 0x30);
                    frame.save(c, S0, 0x20);
                    frame.save(c, S1, 0x10);
                    c.g[S0][0] = c.g[A2][0];
                    frame.save_ra(c, 0);
                    c.g[S1][0] = c.g[A1][0];
                    pc = TEST;
                }
                CONVERT => {
                    c.cycles += 6;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 16);
                    c.g[S0][0] = sx(ea(c.g[S0][0], 2));
                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 16) as u32);
                    callee::MEMCARD_SJIS_CHAR_TO_ASCII.call(c, 0x002c_2298)?;
                    c.sb(c.g[S1][0] as u32, c.g[V0][0]);
                    c.g[S1][0] = sx(ea(c.g[S1][0], 1));
                    pc = TEST;
                }
                0x002c_2284 => {
                    // The generated block split (never branched to).
                    c.cycles += 1;
                    pc = CONVERT;
                }
                TEST => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lh(c.g[S0][0] as u32);
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = c.lhu(c.g[S0][0] as u32);
                    if !more {
                        pc = EXIT;
                        continue;
                    }
                    if event_due(c) && crate::events::deliver_event(c, CONVERT) {
                        return Ok(());
                    }
                    pc = CONVERT;
                }
                EXIT => {
                    c.cycles += 6;
                    c.sb(c.g[S1][0] as u32, 0);
                    let frame = Frame { size: 0x30 };
                    frame.restore(c, S0, 0x20);
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, ENTRY, 0x002c_22c4);
                    return Ok(());
                }
            }
        }
    })
}

/// `void cPS2MemCard_RequestGetInfo(cPS2MemCard* self, int op)` (0x2C4480):
/// remember the pending operation and, when the card's `v11()` finds nothing,
/// restart the state machine at state 3.
pub fn ps2_memcard_request_get_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.sw(ea(c.g[S0][0], 0xc), c.g[S1][0]);
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A0][0] = c.lh(ea(c.g[V1][0], 0x58));
        c.g[V0][0] = c.lw(ea(c.g[V1][0], 0x5c));
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx(ea(c.g[S0][0], c.g[A0][0] as u32));
        call_ptr(c, method, 0x002c_44b0)?;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.sw(ea(c.g[S0][0], 0x10), c.g[S1][0]);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 3;
            callee::PS2_MEMCARD_SET_STATE.call(c, 0x002c_44c8)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cPS2MemCard_DoGetInfo(cPS2MemCard* self)` (0x2C50E0): ask the memory
/// card manager for the card's info and record success at `+0x40`.
pub fn ps2_memcard_do_get_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        gp_sw(c, 0x848, 0);
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.sw(ea(c.g[S0][0], 0x20), 0);
        c.g[A2][0] = sx(gp_rel(c, 0x848));
        c.g[A0][0] = c.lw(ea(c.g[S0][0], 0xc));
        c.g[A3][0] = sx(gp_rel(c, 0x84c));
        c.g[T0][0] = sx(gp_rel(c, 0x854));
        gp_sw(c, 0x854, 0);
        gp_sw(c, 0x84c, 0);
        call_ptr(c, addr::MC_MANAGER_GET_INFO, 0x002c_5118)?;
        c.g[V1][0] = (c.g[V0][0] < 1) as u64;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 5;
        c.sw(ea(c.g[S0][0], 0x40), c.g[V1][0]);
        c.sw(ea(c.g[S0][0], 0x4c), c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = c.g[V1][0];
        frame.close(c);
        Ok(())
    })
}

/// `void tWScriptCache_dtor(tWScriptCache* self, int flags)` (0x3ACA70):
/// release the cached block at `+8` unless a reference is held, then delete
/// the object when bit 0 of `flags` is set.
pub fn wscript_cache_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(ea(c.g[S0][0], 8));
        let no_block = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if no_block {
            c.cycles += 1;
            c.g[V0][0] = c.g[S1][0] & 1;
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lh(ea(c.g[S0][0], 2));
            let referenced = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[S1][0] & 1;
            if !referenced {
                c.cycles += 2;
                call_ptr(c, addr::MEM_MAN_FREE_NOSIZE, 0x003a_caa4)?;
                c.cycles += 1;
                c.g[V0][0] = c.g[S1][0] & 1;
            }
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, addr::MEM_MAN_FREE, 0x003a_cab8)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_DeactivateProcess(WScriptMan* self, void* node)`
/// (0x309AA0): move a node from the active list to the front of the pending
/// list, if it is active at all.
pub fn wscript_man_deactivate_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = sx(ea(c.g[S2][0], 0x2b8));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        callee::WSCRIPT_PROCESS_LIST_CONTAINS.call(c, 0x0030_9ac8)?;
        let inactive = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S1][0];
        if !inactive {
            c.cycles += 5;
            c.g[A1][0] = c.g[S0][0];
            callee::WSCRIPT_PROCESS_LIST_REMOVE.call(c, 0x0030_9ad8)?;
            c.g[A0][0] = sx(ea(c.g[S2][0], 0x2b4));
            c.g[A1][0] = c.g[S0][0];
            callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT.call(c, 0x0030_9ae4)?;
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

/// `void ModObj_AddPositionModifier(ModObj* self, void* params)` (0x3558B8):
/// allocate and install a position modifier built from `params`.
pub fn mod_obj_add_position_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x2000_0000;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = 0x490000;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x90;
        c.g[A1][0] = sx(ea(c.g[A1][0], 0xffff_e8f0));
        call_ptr(c, addr::MEM_MAN_ALLOC, 0x0035_58e8)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        callee::POSITION_MODIFIER_CTOR.call(c, 0x0035_58f4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        callee::MOD_OBJ_SET_TRANSFORM_MODIFIER.call(c, 0x0035_5900)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `sNode* NetEventFD_Clone(sNode* self)` (0x269DC0): allocate a 12-byte
/// `0xFD` node carrying `self->value`.
pub fn net_event_fd_clone(c: &mut Cpu) {
    net_event_clone(c, 0xfd, 0x13e0, 0x0026_9de8);
}

/// `sNode* NetEventFE_Clone(sNode* self)` (0x269E40): the `0xFE` variant.
pub fn net_event_fe_clone(c: &mut Cpu) {
    net_event_clone(c, 0xfe, 0x13c0, 0x0026_9e68);
}

/// The shared body of the two `NetEvent*_Clone` nodes: `vtable_off` is the
/// node vtable's offset from 0x480000.
fn net_event_clone(c: &mut Cpu, kind: u64, vtable_off: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x480000;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = sx(ea(c.g[A1][0], vtable_off));
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, addr::MEM_MAN_ALLOC, ret)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw(ea(c.g[S0][0], 8));
        c.g[V0][0] = kind;
        c.g[V1][0] = 0x480000;
        c.g[V1][0] = sx(ea(c.g[V1][0], vtable_off));
        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.restore_ra(c, 0);
        c.sw(ea(c.g[A0][0], 4), c.g[V1][0]);
        c.sw(ea(c.g[A0][0], 8), c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_SetSelectedPeak(SSXAudioSystem* self, int id)`
/// (0x28F558): store the peak-music id and restart that peak if the audio
/// manager is already playing it.
pub fn ssx_audio_system_set_selected_peak(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(ea(c.g[S0][0], 0x6284), c.g[A1][0]);
        call_ptr(c, addr::CBE_GET_BE, 0x0028_f570)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, addr::CBE_GET_INTERFACE, 0x0028_f57c)?;
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, addr::BE_INTERFACE_LOOKUP, 0x0028_f58c)?;
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.g[V0][0] = c.lw(ea(c.g[S0][0], 0x6284));
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                callee::SSX_AUDIO_SYSTEM_RESTART_PEAK_MUSIC.call(c, 0x0028_f5a4)?;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_StopAllProcessSeqs(ScriptEngine* self)` (0x282390):
/// stop the sequence of each of the 16 active slots at `+0x2C0` (0x24 bytes
/// apart).
pub fn script_engine_stop_all_process_seqs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S0][0] = sx(ea(c.g[S1][0], 0x2c0));
        frame.save_ra(c, 0);
        c.g[S2][0] = 0xf;
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = c.lw(ea(c.g[S0][0], 0x1c));
                c.g[A0][0] = c.g[S1][0];
                callee::SCRIPT_ENGINE_STOP_PROCESS_SEQ.call(c, 0x0028_23c8)?;
            }
            c.g[S2][0] = sx(ea(c.g[S2][0], 0xffff_ffff));
            c.cycles += 2;
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx(ea(c.g[S0][0], 0x24));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0028_23b0);
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

/// `void ScriptEngine_KillAllProcesses(ScriptEngine* self)` (0x2823F0): kill
/// each of the 16 slots at `+0x2C0` whose state word at `+0x20` is set.
pub fn script_engine_kill_all_processes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S0][0] = sx(ea(c.g[S1][0], 0x2c0));
        frame.save_ra(c, 0);
        c.g[S2][0] = 0xf;
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(ea(c.g[S0][0], 0x20));
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = c.lw(ea(c.g[S0][0], 0x1c));
                c.g[A0][0] = c.g[S1][0];
                callee::SCRIPT_ENGINE_KILL_PROCESS.call(c, 0x0028_2428)?;
            }
            c.g[S2][0] = sx(ea(c.g[S2][0], 0xffff_ffff));
            c.cycles += 2;
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx(ea(c.g[S0][0], 0x24));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0028_2410);
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

/// `int ScriptEngine_GetProcessState(ScriptEngine* self, int id)` (0x282798):
/// a pending process is launched (and its new state returned); a live one
/// just reports its state word; no process is 0.
pub fn script_engine_get_process_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::SCRIPT_ENGINE_GET_PROCESS.call(c, 0x0028_27b0)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.g[V1][0] = c.lw(ea(c.g[S0][0], 4));
            c.cycles += 3;
            c.g[V0][0] = 1;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V0][0] = c.lw(ea(c.g[S0][0], 4));
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS.call(c, 0x0028_27dc)?;
                c.g[V0][0] = c.lw(ea(c.g[S0][0], 4));
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

/// `Boost* Boost_ctor_2(Boost* self, void* a1, void* stream)` (0x341548): the
/// instance-node constructor plus the boost vtable and the stream's
/// transform setup.
pub fn boost_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        callee::WORLD_OBJ_CTOR_2.call(c, 0x0034_1564)?;
        c.g[V1][0] = 0x490000;
        c.g[A1][0] = sx(ea(c.g[S0][0], 0x20));
        c.g[V1][0] = sx(ea(c.g[V1][0], 0x14e0));
        c.g[A2][0] = 0x30;
        c.sw(ea(c.g[S0][0], 0xc), c.g[V1][0]);
        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
        c.g[A0][0] = c.lh(ea(c.g[V0][0], 0x10));
        c.g[V1][0] = c.lw(ea(c.g[V0][0], 0x14));
        let method = c.g[V1][0] as u32;
        c.g[A0][0] = sx(ea(c.g[S1][0], c.g[A0][0] as u32));
        call_ptr(c, method, 0x0034_158c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderSphereTree_Init(RiderSphereTree* self, int type, int count,
/// int* ids, float* radii, int user, int radius)` (0x3299C8): fill `count`
/// (id, radius) entries at `+0x40`/`+0x44`, 0x20 bytes apart.
pub fn rider_sphere_tree_init(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.sw(ea(c.g[A0][0], 0x2c), c.g[A2][0]);
    c.g[V0][0] |= 0xffff;
    c.g[T2][0] = 0;
    let empty = (c.g[A2][0] as i64) <= 0;
    c.sw(ea(c.g[A0][0], 0x28), c.g[V0][0]);
    if !empty {
        c.cycles += 2;
        c.g[V1][0] = c.g[A0][0];
        loop {
            c.cycles += 11;
            c.g[V0][0] = c.lw(c.g[A3][0] as u32);
            c.g[T2][0] = sx(ea(c.g[T2][0], 1));
            c.g[A3][0] = sx(ea(c.g[A3][0], 4));
            c.sw(ea(c.g[V1][0], 0x44), c.g[V0][0]);
            c.lwc1(0, c.g[T0][0] as u32);
            c.g[T0][0] = sx(ea(c.g[T0][0], 4));
            c.swc1(0, ea(c.g[V1][0], 0x40));
            c.g[V0][0] = c.lw(ea(c.g[A0][0], 0x2c));
            c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(ea(c.g[V1][0], 0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_99e8);
            }
        }
    }
    c.cycles += 4;
    c.sw(ea(c.g[A0][0], 0x24), c.g[T1][0]);
    c.sw(ea(c.g[A0][0], 0), c.g[A1][0]);
    c.swc1(12, ea(c.g[A0][0], 0x20));
    abi::ret(c);
}

/// `void DataInstanceMan_dtor(DataInstanceMan* self, int flags)` (0x2758C0):
/// install the vtable, free the owned block at `+4`, and delete when bit 0 of
/// `flags` is set.
pub fn data_instance_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x480000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(ea(c.g[V0][0], 0x25a0));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(ea(c.g[S0][0], 0xc), c.g[V0][0]);
        c.g[A0][0] = c.lw(ea(c.g[S0][0], 4));
        let no_block = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_block {
            c.cycles += 2;
            call_ptr(c, addr::MEM_MAN_FREE_NOSIZE, 0x0027_58f4)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, addr::MEM_MAN_FREE, 0x0027_5908)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Differential tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable};
    use crate::difftest::{check, DATA, GAME_GP};

    const SEEDS: u32 = 40;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    fn h(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }

    fn b(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }

    fn fb(x: f32) -> u32 {
        x.to_bits()
    }

    /// The BELibrary singleton (`gp - 0x1F04`) and a zeroed dummy object it
    /// points at: without it a library lookup tries to build the whole BE
    /// singleton graph (and allocate from a zeroed heap).
    fn belib(c: &mut Cpu) {
        w(c, GAME_GP - 0x1f04, DATA + 0x8000);
    }

    /// A run of distinct `BE*` singletons for the visibility-flag functions.
    fn race_singletons(c: &mut Cpu) {
        const OFFSETS: [i32; 12] = [
            -0x1f3c, -0x1f30, -0x1f2c, -0x1ee8, -0x1ee4, -0x1ed4, -0x1eec, -0x1f28, -0x1ee0,
            -0x1edc, -0x1ea4, -0x1e8c,
        ];
        for (i, off) in OFFSETS.into_iter().enumerate() {
            w(c, (GAME_GP as i32 + off) as u32, DATA + 0x100 + 0x40 * i as u32);
        }
    }

    // ------------------------------------------------------- front end widgets

    #[test]
    fn fe_state_main_menu_on_widget_notify_matches() {
        check(ee::s02::f_001952e8__generated, fe_state_main_menu_on_widget_notify, SEEDS, &|c, seed| {
            machine(c, seed, 0x0019_52e8, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A2][0] = match seed % 4 {
                0 => 6,
                1 => 9,
                2 => 5,
                _ => 12,
            };
            w(c, DATA + 0x48, if seed % 2 == 0 { 0 } else { DATA + 0x100 });
        });
    }

    #[test]
    fn ov_screen_pause_freeride_on_widget_notify_matches() {
        check(
            ee::s04::f_001f8908__generated,
            ov_screen_pause_freeride_on_widget_notify,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x001f_8908, 0);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
    }

    #[test]
    fn fe_state_uber_trick_on_widget_notify_matches() {
        check(ee::s02::f_001859d8__generated, fe_state_uber_trick_on_widget_notify, SEEDS, &|c, seed| {
            machine(c, seed, 0x0018_59d8, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
            w(c, DATA + 0x50, (seed % 2) as u32);
            c.g[A2][0] = (seed % 12) as u64;
        });
    }

    #[test]
    fn fe_state_lodge_on_widget_notify_matches() {
        check(ssx3_ee::s04::f_001f3cc8__generated, fe_state_lodge_on_widget_notify, SEEDS, &|c, seed| {
            machine(c, seed, 0x001f_3cc8, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A2][0] = (seed % 12) as u64;
        });
    }

    #[test]
    fn fe_state_lodge_rider_detail_on_widget_notify_matches() {
        check(ssx3_ee::s04::f_001f43d8__generated, fe_state_lodge_rider_detail_on_widget_notify, SEEDS, &|c, seed| {
            machine(c, seed, 0x001f_43d8, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A2][0] = (seed % 12) as u64;
        });
    }

    #[test]
    fn fe_state_bonus_material_on_widget_notify_matches() {
        check(ee::s02::f_001955b8__generated, fe_state_bonus_material_on_widget_notify, SEEDS, &|c, seed| {
            machine(c, seed, 0x0019_55b8, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A2][0] = (seed % 12) as u64;
        });
    }

    #[test]
    fn fe_options_credits_on_widget_notify_matches() {
        check(ee::s02::f_00186478__generated, fe_options_credits_on_widget_notify, SEEDS, &|c, seed| {
            machine(c, seed, 0x0018_6478, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A2][0] = (seed % 12) as u64;
        });
    }

    #[test]
    fn fe_state_rider_detail_on_widget_notify_matches() {
        check(ee::s02::f_00182c68__generated, fe_state_rider_detail_on_widget_notify, SEEDS, &|c, seed| {
            machine(c, seed, 0x0018_2c68, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A2][0] = match seed % 3 {
                0 => 9,
                1 => 0,
                _ => seed as u64,
            };
        });
    }

    #[test]
    fn fe_state_audio_options_on_widget_notify_matches() {
        check(ssx3_ee::s02::f_00196b08__generated, fe_state_audio_options_on_widget_notify, 100, &|c, seed| {
            machine(c, seed, 0x0019_6b08, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A2][0] = match seed % 3 {
                0 => 6,
                1 => 8 + (seed % 2) as u64,
                _ => 5,
            };
            c.g[A3][0] = (seed % 6) as u64;
            w(c, DATA + 0x74, if seed % 2 == 0 { 0 } else { 1 });
            w(c, DATA + 0x7c, if seed % 4 == 0 { 0 } else { 1 });
        });
    }

    // ------------------------------------------------------- option blocks

    #[test]
    fn be_network_int_commit_matches() {
        check(ee::s01::f_0014e130__generated, be_network_int_commit, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x0014_e130, 0);
            belib(c);
            for i in 0..0x128u32 {
                w(c, 0x0053_4b30 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn be_network_int_reset_matches() {
        check(ee::s01::f_0014e1f8__generated, be_network_int_reset, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x0014_e1f8, 0);
            belib(c);
            for i in 0..0x128u32 {
                w(c, 0x0053_5088 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn be_option_int_commit_matches() {
        check(ee::s01::f_0014f2a8__generated, be_option_int_commit, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x0014_f2a8, 0);
            belib(c);
            for i in 0..0xa0u32 {
                w(c, 0x0053_5610 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn be_option_int_reset_matches() {
        check(ee::s01::f_0014f360__generated, be_option_int_reset, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x0014_f360, 0);
            belib(c);
            for i in 0..0xa0u32 {
                w(c, 0x0053_5898 + 4 * i, r.next_u32());
            }
        });
    }

    // ------------------------------------------------------- race flow flags

    #[test]
    fn be_post_race_exit_matches() {
        check(ee::s01::f_001564b0__generated, be_post_race_exit, SEEDS, &|c, seed| {
            machine(c, seed, 0x0015_64b0, 0);
            belib(c);
            race_singletons(c);
        });
    }

    #[test]
    fn be_post_race_enter_matches() {
        check(ee::s01::f_00156510__generated, be_post_race_enter, SEEDS, &|c, seed| {
            machine(c, seed, 0x0015_6510, 0);
            belib(c);
            race_singletons(c);
        });
    }

    #[test]
    fn be_pre_race_exit_matches() {
        check(ee::s01::f_001566e8__generated, be_pre_race_exit, SEEDS, &|c, seed| {
            machine(c, seed, 0x0015_66e8, 0);
            belib(c);
            race_singletons(c);
        });
    }

    #[test]
    fn be_pre_race_enter_matches() {
        check(ee::s01::f_00156750__generated, be_pre_race_enter, SEEDS, &|c, seed| {
            machine(c, seed, 0x0015_6750, 0);
            belib(c);
            race_singletons(c);
        });
    }

    // ------------------------------------------------------- comparators

    #[test]
    fn game_anim_map_compare_name_index_matches() {
        check(ee::s10::f_00314d60__generated, game_anim_map_compare_name_index, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x0031_4d60, 0);
            belib(c);
            let table = DATA + 0x2000;
            w(c, GAME_GP + 0xd88, table);
            for i in 0..16u32 {
                w(c, table + 4 * i, r.next_u32() % 32);
            }
            c.g[A0][0] = (DATA + 0x100) as u64;
            c.g[A1][0] = (DATA + 0x104) as u64;
            w(c, DATA + 0x100, r.next_u32() % 16);
            w(c, DATA + 0x104, r.next_u32() % 16);
        });
    }

    #[test]
    fn avalanche_compare_float_matches() {
        check(ee::s09::f_002d5718__generated, avalanche_compare_float, 80, &|c, seed| {
            let mut r = machine(c, seed, 0x002d_5718, 0);
            belib(c);
            c.g[A0][0] = (DATA + 0x100) as u64;
            c.g[A1][0] = (DATA + 0x108) as u64;
            let a = match seed % 3 {
                0 => fb(1.5),
                1 => fb(-2.25),
                _ => r.edge_f32(),
            };
            let b = if seed % 4 == 0 { a } else { r.edge_f32() };
            w(c, DATA + 0x100, a);
            w(c, DATA + 0x108, b);
        });
    }

    #[test]
    fn avalanche_compare_u16_matches() {
        check(ee::s09::f_002d5750__generated, avalanche_compare_u16, 80, &|c, seed| {
            let mut r = machine(c, seed, 0x002d_5750, 0);
            belib(c);
            c.g[A0][0] = (DATA + 0x100) as u64;
            c.g[A1][0] = (DATA + 0x102) as u64;
            let a = (r.next_u32() & 0xffff) as u16;
            let b = if seed % 4 == 0 { a } else { (r.next_u32() & 0xffff) as u16 };
            h(c, DATA + 0x100, a);
            h(c, DATA + 0x102, b);
        });
    }

    #[test]
    fn anim_object_is_time_in_window_matches() {
        check(ee::s11::f_0034ebe0__generated, anim_object_is_time_in_window, 150, &|c, seed| {
            let mut r = machine(c, seed, 0x0034_ebe0, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.f[12] = r.edge_f32();
            w(c, GAME_GP - 0x2aa0, fb(1.0e10));
            let (start, end) = match seed % 4 {
                0 => (1.0e10, 2.0),
                1 => (1.0, 2.0),
                2 => (2.0, 1.0),
                _ => (-1.0, 1.0),
            };
            w(c, DATA + 8, fb(start));
            w(c, DATA + 0xc, fb(end));
        });
    }

    // ------------------------------------------------------- object transforms

    #[test]
    fn mod_obj_get_matrix_at_matches() {
        check(ee::s12::f_003610e0__generated, mod_obj_get_matrix_at, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x0036_10e0, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (r.next_u32() % 64) as u64;
            h(c, DATA + 0x26, if seed % 2 == 0 { 0 } else { 1 });
            w(c, DATA + 0x44, DATA + 0x3000);
            let vt = vtable(c, &mut r, &[0x198]);
            w(c, DATA + 0x20, vt);
        });
    }

    #[test]
    fn ssx_audio_system_kill_rider_timers_matches() {
        check(ee::s07::f_00292ae8__generated, ssx_audio_system_kill_rider_timers, 24, &|c, seed| {
            let mut r = machine(c, seed, 0x0029_2ae8, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = 7;
            w(c, DATA + 0x118, DATA + 0x100);
            w(c, DATA + 0x100, DATA + 0x200);
            let vt = vtable(c, &mut r, &[0x20]);
            w(c, DATA + 0x200 + 0xab0, vt);
        });
    }

    #[test]
    fn ssx_audio_system_is_track_group2_event_matches() {
        check(ee::s08::f_002a4168__generated, ssx_audio_system_is_track_group2_event, 64, &|c, seed| {
            machine(c, seed, 0x002a_4168, 4);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 12) as u64;
            b(c, 0x0053_5c10, (seed % 4) as u8);
        });
    }

    #[test]
    fn ssx_audio_system_is_track_group3_event_matches() {
        check(ee::s08::f_002a41d8__generated, ssx_audio_system_is_track_group3_event, 64, &|c, seed| {
            machine(c, seed, 0x002a_41d8, 4);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 12) as u64;
            b(c, 0x0053_5c10, (seed % 4) as u8);
        });
    }

    #[test]
    fn ps2_sound_manager_dtor_matches() {
        check(ee::s08::f_002a8c50__generated, ps2_sound_manager_dtor, SEEDS, &|c, seed| {
            machine(c, seed, 0x002a_8c50, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 3) as u64;
            w(c, DATA + 4, if seed % 2 == 0 { 0 } else { DATA + 0x800 });
        });
    }

    #[test]
    fn data_instance_man_dtor_matches() {
        check(ee::s07::f_002758c0__generated, data_instance_man_dtor, SEEDS, &|c, seed| {
            machine(c, seed, 0x0027_58c0, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 3) as u64;
            w(c, DATA + 4, if seed % 2 == 0 { 0 } else { DATA + 0x800 });
        });
    }

    #[test]
    fn speech_bank_dtor_matches() {
        check(ee::s08::f_002af900__generated, speech_bank_dtor, SEEDS, &|c, seed| {
            machine(c, seed, 0x002a_f900, 3);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 3) as u64;
            w(c, DATA, DATA + 0x600);
            w(c, DATA + 0x44, DATA + 0x800);
            w(c, DATA + 0x48, if seed % 2 == 0 { 0 } else { 1 });
        });
    }

    #[test]
    fn script_obj_cam_is_bound_to_process_matches() {
        check(ee::s07::f_0027d790__generated, script_obj_cam_is_bound_to_process, SEEDS, &|c, seed| {
            machine(c, seed, 0x0027_d790, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 5) as u64;
            let bound = if seed % 2 == 0 { 0 } else { DATA + 0x100 };
            w(c, DATA + 0x14, bound);
            w(c, DATA + 0x104, (seed % 5) as u32);
        });
    }

    #[test]
    fn path_plat_find_bank_slot_matches() {
        check(ee::s08::f_002ae230__generated, path_plat_find_bank_slot, 100, &|c, seed| {
            machine(c, seed, 0x002a_e230, 0);
            belib(c);
            for i in 0..24u32 {
                let base = 0x004d_3ef8 + 0x28 * i;
                w(c, base, i);
                let bank = if i < 20 { 0xffff_ffff } else { i };
                w(c, base + 8, bank);
            }
            c.g[A0][0] = match seed % 5 {
                0 => 0xffff_ffff_ffff_ffff,
                1 => 21,
                2 => 20 + (seed % 4) as u64,
                3 => 0,
                _ => 0xffff_ffff_ffff_fffe,
            };
            c.g[A1][0] = match seed % 3 {
                0 => 0xffff_ffff_ffff_ffff,
                1 => 0xffff_ffff_ffff_fffe,
                _ => (seed % 25) as u64,
            };
        });
    }

    #[test]
    fn song_instance_get_ms_to_next_beat_matches() {
        check(ee::s08::f_002b2718__generated, song_instance_get_ms_to_next_beat, SEEDS, &|c, seed| {
            machine(c, seed, 0x002b_2718, 3);
            belib(c);
            c.g[A0][0] = DATA as u64;
            w(c, DATA, DATA + 0x200);
            w(c, DATA + 0x200 + 0x90, fb(1.0 + (seed % 4) as f32));
            w(c, DATA + 0x200 + 0x4, (seed % 8) as u32);
        });
    }

    // ------------------------------------------------------- menu items

    #[test]
    fn menu_sys_action_item_ctor_3_matches() {
        check(ee::s09::f_002ccf98__generated, menu_sys_action_item_ctor_3, SEEDS, &|c, seed| {
            machine(c, seed, 0x002c_cf98, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            c.g[A2][0] = (seed % 7) as u64;
            c.g[A3][0] = (seed % 11) as u64;
        });
    }

    #[test]
    fn menu_sys_int8_item_draw_matches() {
        check(ee::s09::f_002cd7f0__generated, menu_sys_int8_item_draw, SEEDS, &|c, seed| {
            machine(c, seed, 0x002c_d7f0, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            w(c, DATA + 0x18, DATA + 0x300);
            b(c, DATA + 0x300, (seed % 100) as u8);
            w(c, DATA + 0x14, (seed % 9) as u32);
        });
    }

    #[test]
    fn menu_sys_int_item_draw_matches() {
        check(ee::s09::f_002cdc28__generated, menu_sys_int_item_draw, SEEDS, &|c, seed| {
            machine(c, seed, 0x002c_dc28, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            w(c, DATA + 0x18, DATA + 0x300);
            w(c, DATA + 0x300, (seed.wrapping_mul(0x9e37)) as u32);
            w(c, DATA + 0x14, (seed % 9) as u32);
        });
    }

    // ------------------------------------------------------- memcard

    #[test]
    fn memcard_sjis_to_ascii_string_matches() {
        check(ee::s08::f_002c2268__generated, memcard_sjis_to_ascii_string, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x002c_2268, 1);
            belib(c);
            c.pc = 0x002c_2268;
            c.g[A1][0] = (DATA + 0x400) as u64;
            c.g[A2][0] = (DATA + 0x200) as u64;
            let n = 1 + seed % 6;
            for i in 0..n {
                let ch = if seed % 5 == 0 {
                    0x8140 | (r.next_u32() & 0xff) as u16
                } else if seed % 5 == 1 {
                    0x8260 | (r.next_u32() & 0x1f) as u16
                } else {
                    (r.next_u32() & 0xff) as u16
                };
                h(c, DATA + 0x200 + 2 * i, ch);
            }
            h(c, DATA + 0x200 + 2 * n, 0);
        });
    }

    #[test]
    fn ps2_memcard_request_get_info_matches() {
        check(ee::s08::f_002c4480__generated, ps2_memcard_request_get_info, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x002c_4480, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 5) as u64;
            let vt = vtable(c, &mut r, &[0x58]);
            w(c, DATA, vt);
            if seed % 2 == 1 {
                // A callee that answers 0 goes on to restart the state machine.
                w(c, vt + 0x5c, 0x002a_41c8);
                c.g[A1][0] = 0;
            }
        });
    }

    #[test]
    fn ps2_memcard_do_get_info_matches() {
        check(ee::s08::f_002c50e0__generated, ps2_memcard_do_get_info, SEEDS, &|c, seed| {
            machine(c, seed, 0x002c_50e0, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0xc, seed % 6);
        });
    }

    // ------------------------------------------------------- script engine

    #[test]
    fn wscript_cache_dtor_matches() {
        check(ee::s13::f_003aca70__generated, wscript_cache_dtor, SEEDS, &|c, seed| {
            machine(c, seed, 0x003a_ca70, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 3) as u64;
            w(c, DATA + 8, if seed % 2 == 0 { 0 } else { DATA + 0x800 });
            h(c, DATA + 2, if seed % 3 == 0 { 1 } else { 0 });
        });
    }

    #[test]
    fn wscript_man_deactivate_process_matches() {
        check(ee::s10::f_00309aa0__generated, wscript_man_deactivate_process, SEEDS, &|c, seed| {
            machine(c, seed, 0x0030_9aa0, 3);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            let node = DATA + 0x100;
            if seed % 2 == 0 {
                w(c, DATA + 0x2b8, 0); // inactive list
            } else {
                w(c, DATA + 0x2b8, node);
                w(c, node + 0x18, 0);
            }
        });
    }

    #[test]
    fn script_engine_stop_all_process_seqs_matches() {
        check(ee::s07::f_00282390__generated, script_engine_stop_all_process_seqs, 64, &|c, seed| {
            machine(c, seed, 0x0028_2390, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            for i in 0..16u32 {
                let slot = DATA + 0x2c0 + 0x24 * i;
                for k in 0..9u32 {
                    w(c, slot + 4 * k, 0);
                }
            }
            if seed % 2 == 1 {
                // One active sequence; its process id 0 resolves to the empty
                // slot 0, so `StopProcessSeq` returns at once.
                w(c, DATA + 0x2c0 + 0x24 * (seed % 16) as u32, 1);
            }
        });
    }

    #[test]
    fn script_engine_kill_all_processes_matches() {
        check(ee::s07::f_002823f0__generated, script_engine_kill_all_processes, 64, &|c, seed| {
            machine(c, seed, 0x0028_23f0, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            for i in 0..16u32 {
                let slot = DATA + 0x2c0 + 0x24 * i;
                for k in 0..9u32 {
                    w(c, slot + 4 * k, 0);
                }
            }
        });
    }

    #[test]
    fn script_engine_get_process_state_matches() {
        check(ee::s07::f_00282798__generated, script_engine_get_process_state, SEEDS, &|c, seed| {
            machine(c, seed, 0x0028_2798, 2);
            belib(c);
            let id = seed % 6;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = id as u64;
            let slot = DATA + 0x2c0 + 0x24 * id;
            if seed % 2 == 0 {
                w(c, slot + 0x20, 0); // no such process
            } else {
                w(c, slot + 0x20, 1);
                w(c, slot + 4, 0); // already running: reports its state word
            }
        });
    }

    // ------------------------------------------------------- objects and audio

    #[test]
    fn mod_obj_add_position_modifier_matches() {
        check(ee::s11::f_003558b8__generated, mod_obj_add_position_modifier, 24, &|c, seed| {
            let mut r = machine(c, seed, 0x0035_58b8, 3);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            pool_allocator(c);
            // keep the modifier constructor on its short path
            for i in 0..0x20u32 {
                w(c, DATA + 0x200 + 4 * i, r.next_u32() & 1);
            }
        });
    }

    #[test]
    fn net_event_fd_clone_matches() {
        check(ee::s07::f_00269dc0__generated, net_event_fd_clone, SEEDS, &|c, seed| {
            machine(c, seed, 0x0026_9dc0, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 8, seed.wrapping_mul(0x1234_5678));
            pool_allocator(c);
        });
    }

    #[test]
    fn net_event_fe_clone_matches() {
        check(ee::s07::f_00269e40__generated, net_event_fe_clone, SEEDS, &|c, seed| {
            machine(c, seed, 0x0026_9e40, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 8, seed.wrapping_mul(0x1234_5678));
            pool_allocator(c);
        });
    }

    #[test]
    fn ssx_audio_system_set_selected_peak_matches() {
        check(ee::s07::f_0028f558__generated, ssx_audio_system_set_selected_peak, 64, &|c, seed| {
            machine(c, seed, 0x0028_f558, 3);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 8) as u64;
            w(c, DATA + 0x6284, (seed % 8) as u32);
            // `RestartPeakMusic` reaches the music player's round-robin slot
            // picker, which spins until the occupancy mask names a free slot:
            // give it a two-slot bank whose second slot is free.
            w(c, DATA + 0x4fc, 2); // slot count
            w(c, DATA + 0x508, 0); // current
            c.bus.write64(DATA + 0x510, 2); // slot mask (slot 1 free)
        });
    }

    #[test]
    fn boost_ctor_2_matches() {
        check(ee::s11::f_00341548__generated, boost_ctor_2, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x0034_1548, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            c.g[A2][0] = (DATA + 0x300) as u64;
            let vt = vtable(c, &mut r, &[0x10]);
            w(c, DATA + 0x300, vt);
        });
    }

    #[test]
    fn rider_sphere_tree_init_matches() {
        check(ee::s10::f_003299c8__generated, rider_sphere_tree_init, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x0032_99c8, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = (seed % 5) as u64;
            c.g[A3][0] = (DATA + 0x400) as u64;
            c.g[T0][0] = (DATA + 0x500) as u64;
            c.g[T1][0] = (seed % 9) as u64;
            c.f[12] = r.edge_f32();
            for i in 0..5u32 {
                w(c, DATA + 0x400 + 4 * i, r.next_u32());
                w(c, DATA + 0x500 + 4 * i, r.edge_f32());
            }
        });
    }

    #[test]
    fn floating_v15_matches() {
        check(ee::s11::f_0034f930__generated, floating_v15, 150, &|c, seed| {
            let mut r = machine(c, seed, 0x0034_f930, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            for off in [
                0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40, 0x44, 0x48, 0x4c, 0x50, 0x54, 0x5c, 0x60,
                0x64, 0x6c, 0x70,
            ] {
                w(c, DATA + off, r.edge_f32());
            }
            w(c, GAME_GP + 0x2a74, DATA + 0x900);
            w(c, DATA + 0x910, 24 + (seed % 6));
            w(c, GAME_GP - 0x2a98, fb(1.396_263_5));
            w(c, GAME_GP - 0x2a94, fb(-1.396_263_5));
        });
    }

    #[test]
    fn ps2_bezier_man_v22_matches() {
        check(ee::s12::f_0038b190__generated, ps2_bezier_man_v22, 100, &|c, seed| {
            machine(c, seed, 0x0038_b190, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x1000) as u64;
            c.g[A2][0] = (DATA + 0x2000) as u64;
            c.g[T0][0] = (DATA + 0x3000) as u64;
            let types: [u8; 4] = match seed % 6 {
                0 => [4, 4, 4, 4],
                1 => [6, 6, 6, 6],
                2 => [8, 8, 8, 8],
                3 => [1, 2, 8, 3],
                4 => [1, 2, 3, 4],
                _ => [2, 2, 2, 2],
            };
            for (i, t) in types.into_iter().enumerate() {
                b(c, DATA + 0x2000 + i as u32, t);
            }
            h(c, DATA + 0x1000 + 0x1a6, 0x11);
            h(c, DATA + 0x1000 + 0x1a8, 0x22);
            h(c, DATA + 0x1000 + 0x1aa, 0x33);
            match seed % 4 {
                1 => w(c, DATA + 0x3c9c, 220), // 6-list full
                2 => {
                    w(c, DATA + 0x458, 900); // 4-list full
                    w(c, DATA + 0x4d84, 75); // mix list full
                }
                3 => {
                    w(c, DATA + 0x4a60, 50); // 8-list full
                    w(c, DATA + 0x3c9c, 220);
                }
                _ => {}
            }
        });
    }

    /// The engine's size-class pool allocator: blocks up to 0x400 bytes come
    /// from per-size free lists hanging off `gp + 0x2a60..0x2a84` once the
    /// "pools ready" flag at `gp + 0xdd8` is set. Without it an allocation
    /// grows the DLHeap from a zeroed header and can walk a corrupt bucket
    /// chain.
    fn pool_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),   // 8-byte blocks
            (0x2a60, 0x2000),  // 16
            (0x2a6c, 0x32000), // 32
            (0x2a7c, 0x25800), // 64
            (0x2a84, 0x25800), // 128
            (0x2a78, 0x20000), // 256
            (0x2a68, 0x20000), // 512
        ];
        let nodes = DATA + 0x28_000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes + 0x100 * i as u32;
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                w(c, node, if n == 3 { 0 } else { node + 0x40 });
            }
            w(c, pool + head, base);
            w(c, GAME_GP + slot, pool);
        }
    }
}


