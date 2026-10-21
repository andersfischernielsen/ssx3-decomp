//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! Script engine sequence notifications and binding/variable bookkeeping, the
//! menu item/page input and drawing paths, and a handful of constructors,
//! destructors and resets on the cold side (load-state transitions, crowd,
//! particle, world-cache and memcard code).
//!
//! Every function reproduces the generated body's cycles, memory accesses and
//! the registers it leaves behind; call-making bodies run inside
//! [`crate::call::guest`] and propagate the host-thread unwind.

use crate::abi::{self, r32};
use crate::call::{self, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0027_a6b0, script_engine_on_seq_start, "ScriptEngine_OnSeqStart"),
    (0x0027_a7f0, script_engine_on_seq_stopped, "ScriptEngine_OnSeqStopped"),
    (0x0027_d050, script_engine_remove_all_script_objs, "ScriptEngine_RemoveAllScriptObjs"),
    (0x0027_d390, script_engine_remove_binding, "ScriptEngine_RemoveBinding"),
    (0x0027_d578, script_engine_collect_free_bindings, "ScriptEngine_CollectFreeBindings"),
    (0x002c_a2b0, menu_sys_item_handle_input, "MenuSysItem_HandleInput"),
    (0x002c_bf60, menu_sys_page_on_activate, "MenuSysPage_OnActivate"),
    (0x002c_cd20, menu_sys_submenu_item_handle_input, "MenuSysSubmenuItem_HandleInput"),
    (0x002c_e100, menu_sys_float_item_draw, "MenuSysFloatItem_Draw"),
    (0x0027_2938, in_seq_start_sequence, "InSeq_StartSequence"),
    (0x0012_c0c0, rider_gate_sub_update_wait_push, "RiderGateSub_UpdateWaitPush"),
    (0x0038_97a0, irradiance_scale, "Irradiance_Scale"),
    (0x0024_5698, c_game_load_state_out_lodge_transition_in, "cGameLoadStateOutLodge_TransitionIn"),
    (0x0024_58b8, c_game_load_state_conquer_transition_in, "cGameLoadStateConquer_TransitionIn"),
    (0x0024_5ae0, c_fe_load_hint_state_transition_in, "cFELoadHintState_TransitionIn"),
    (0x0024_5c60, c_fe_load_state_transition_in, "cFELoadState_TransitionIn"),
    (0x0024_5dc0, c_fe_load_state_in_lodge_transition_in, "cFELoadStateInLodge_TransitionIn"),
    (0x0035_7a78, multi_particle_man_create_slot, "MultiParticleMan_CreateSlot"),
    (0x0026_af00, path_data_free, "PathData_Free"),
    (0x0013_a8f8, rider_air_state_play_land_anim, "RiderAirState_PlayLandAnim"),
    (0x0023_57f8, game_state_reset_update, "GameStateReset_Update"),
    (0x0012_5038, rider_stop_transition, "Rider_StopTransition"),
    (0x0030_8f70, wscript_man_unload_section, "WScriptMan_UnloadSection"),
    (0x0018_8800, c_fe_state_options_init, "cFEStateOptions_Init"),
    (0x001d_9ca8, c_fe_popup_confirm_v27, "cFEPopupConfirm_v27"),
    (0x0028_2720, script_engine_resume_process, "ScriptEngine_ResumeProcess"),
    (0x0014_7898, benew_player_int_clear_shared_char_record, "BENewPlayerInt_ClearSharedCharRecord"),
    (0x0023_82d8, game_mode_man_reset_mode, "GameModeMan_ResetMode"),
    (0x002b_5e68, world_trigger_manager_stop_all, "WorldTriggerManager_StopAll"),
    (0x0027_5b08, data_instance_man_remove_all, "DataInstanceMan_RemoveAll"),
    (0x0032_6678, input_preproc_skip_line, "InputPreproc_SkipLine"),
    (0x0032_6cf0, pad_device_stop_actuators, "PadDevice_StopActuators"),
    (0x002f_03c8, ps2_ai_visual_effects_man_ctor, "PS2AIVisualEffectsMan_ctor"),
    (0x0038_df98, light_man_v11, "LightMan_v11"),
    (0x003a_7aa8, world_memman_dtor, "WorldMemman_dtor"),
    (0x003a_8f10, c_world_cache_flag_section_objects, "cWorldCache_FlagSectionObjects"),
    (0x0022_9398, crowd_man2d_remove_all, "CrowdMan2d_RemoveAll"),
    (0x003a_99d8, c_world_cache_streamer_activate_section, "cWorldCache_StreamerActivateSection"),
    (0x003a_1148, c_ui_text_scroll_ctor, "cUITextScroll_ctor"),
    (0x003a_3280, ui_vector_ctor, "UIVector_ctor"),
    (0x0035_81f0, multi_particle_remove_instance, "MultiParticle_RemoveInstance"),
    (0x002d_9fb8, avalanche_set_history_data, "Avalanche_SetHistoryData"),
    (0x0026_e4b8, replay_snap_to_snapshot, "Replay_SnapToSnapshot"),
    (0x0015_cba0, cam_controller_ctor, "CamController_ctor"),
    (0x0018_3a98, c_fe_state_uber_trick_ctor, "cFEStateUberTrick_ctor"),
    (0x0037_c830, ps2_graphics_man_free_all_textures, "PS2GraphicsMan_FreeAllTextures"),
    (0x001d_1128, c_fe_state_reward_gallery_posters_ctor, "cFEStateRewardGalleryPosters_ctor"),
];

/// `a + o`, 32-bit.
#[inline(always)]
fn p(a: u32, o: u32) -> u32 {
    a.wrapping_add(o)
}

/// `sq reg, off(sp)`.
#[inline(always)]
fn sqsp(c: &mut Cpu, reg: usize, off: u32) {
    let a = (c.g[SP][0] as u32).wrapping_add(off);
    c.sq(reg, a);
}

/// `lq reg, off(sp)`.
#[inline(always)]
fn lqsp(c: &mut Cpu, reg: usize, off: u32) {
    let a = (c.g[SP][0] as u32).wrapping_add(off);
    c.lq(reg, a);
}

/// `sd v, off(sp)`.
#[inline(always)]
fn sdsp(c: &mut Cpu, off: u32, v: u64) {
    let a = (c.g[SP][0] as u32).wrapping_add(off);
    c.sd(a, v);
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// Callees and fixed addresses.
mod callee {
    use super::{ee, Callee};

    pub const SEQ_LISTENER_ON_START: Callee = Callee { addr: 0x0028_29d0, f: ee::s07::f_002829d0 };
    pub const SEQ_LISTENER_ON_STOP: Callee = Callee { addr: 0x0028_2a18, f: ee::s07::f_00282a18 };
    pub const GET_SEQ_PROCESS_ID: Callee = Callee { addr: 0x0028_2be0, f: ee::s07::f_00282be0 };
    pub const SCRIPT_LIST_ON_SEQ_START: Callee = Callee { addr: 0x0027_72c0, f: ee::s07::f_002772c0 };
    pub const SCRIPT_LIST_ON_SEQ_STOPPED: Callee = Callee { addr: 0x0027_7400, f: ee::s07::f_00277400 };
    pub const SCRIPT_BINDING_CLEAR_LISTENER: Callee =
        Callee { addr: 0x0028_2f90, f: ee::s07::f_00282f90 };
    pub const MENU_ITEM_GET_INPUT_CODE: Callee =
        Callee { addr: 0x002c_a3e8, f: ee::s09::f_002ca3e8 };
    pub const MENU_ITEM_IS_DISABLED: Callee = Callee { addr: 0x002c_a3b0, f: ee::s09::f_002ca3b0 };
    pub const MENU_ITEM_HANDLE_INPUT: Callee = Callee { addr: 0x002c_a2b0, f: ee::s09::f_002ca2b0 };
    pub const MENU_PAGE_VALIDATE_SELECTION: Callee =
        Callee { addr: 0x002c_aab0, f: ee::s09::f_002caab0 };
    pub const MENU_PUSH_PAGE: Callee = Callee { addr: 0x002c_c3b8, f: ee::s09::f_002cc3b8 };
    pub const MENU_FORMAT_FLOAT: Callee = Callee { addr: 0x002c_dfd8, f: ee::s09::f_002cdfd8 };
    pub const MENU_ITEM_DRAW_ROW: Callee = Callee { addr: 0x002c_a988, f: ee::s09::f_002ca988 };
    pub const IN_SEQ_INSTANCE_SETUP: Callee = Callee { addr: 0x0027_45c0, f: ee::s07::f_002745c0 };
    pub const LIGHT_MAN_EMPTY_HOOK: Callee = Callee { addr: 0x0038_dc60, f: ee::s12::f_0038dc60 };
    pub const UI_OBJECT_CTOR: Callee = Callee { addr: 0x0039_fb30, f: ee::s13::f_0039fb30 };
    pub const UI_LIST_CTOR: Callee = Callee { addr: 0x0039_77e8, f: ee::s13::f_003977e8 };
    pub const CUI_TEXT_SCROLL_SET_NAV_BUTTONS: Callee =
        Callee { addr: 0x003a_1310, f: ee::s13::f_003a1310 };
    pub const FE_STATE_CTOR: Callee = Callee { addr: 0x0039_e2a0, f: ee::s13::f_0039e2a0 };
    pub const REWARD_GALLERY_CTOR: Callee = Callee { addr: 0x001c_eea0, f: ee::s04::f_001ceea0 };
    pub const AI_VISUAL_EFFECTS_MAN_CTOR: Callee =
        Callee { addr: 0x002f_0368, f: ee::s09::f_002f0368 };
}

/// `void ScriptEngine_OnSeqStart(ScriptEngine* e, ScriptList* list)`
/// (0x27a6b0): tell the listener a sequence started, and if the engine is not
/// in "stopped" mode (state 2) also notify the script list entry `r` of the
/// sequence table (`+0x54c`, `0xcc` bytes each).
pub fn script_engine_on_seq_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 17, 0x20);
        sqsp(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        sqsp(c, 16, 0x30);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        callee::SEQ_LISTENER_ON_START.call(c, 0x0027_a6d0)?;
        c.g[V1][0] = Addr(p(r32(c, S1), 0x550)).lw(c);
        c.g[V0][0] = 2;
        let stopped = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0xcc;
        if !stopped {
            c.cycles += 10;
            c.g[S0][0] = Addr(p(r32(c, S1), 0x54c)).lw(c);
            c.mult(0, V0, V1, V0);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[S0][0] = abi::addu(c.g[S0][0], c.g[V0][0]);
            callee::GET_SEQ_PROCESS_ID.call(c, 0x0027_a6f8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S2][0];
            callee::SCRIPT_LIST_ON_SEQ_START.call(c, 0x0027_a708)?;
        }
        c.cycles += 6;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `void ScriptEngine_OnSeqStopped(ScriptEngine* e, ScriptList* list)`
/// (0x27a7f0): like [`script_engine_on_seq_start`] but tells the listener the
/// sequence stopped, before notifying the script list.
pub fn script_engine_on_seq_stopped(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 2;
        sqsp(c, 17, 0x20);
        sqsp(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        sqsp(c, 16, 0x30);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(r32(c, S1), 0x550)).lw(c);
        let stopped = c.g[V1][0] == c.g[V0][0];
        c.g[S2][0] = c.g[A1][0];
        if !stopped {
            c.cycles += 9;
            c.g[V0][0] = 0xcc;
            c.g[S0][0] = Addr(p(r32(c, S1), 0x54c)).lw(c);
            c.mult(0, V0, V1, V0);
            c.g[S0][0] = abi::addu(c.g[S0][0], c.g[V0][0]);
            callee::GET_SEQ_PROCESS_ID.call(c, 0x0027_a82c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S2][0];
            callee::SCRIPT_LIST_ON_SEQ_STOPPED.call(c, 0x0027_a83c)?;
        }
        c.cycles += 9;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        callee::SEQ_LISTENER_ON_STOP.call(c, 0x0027_a848)?;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void ScriptEngine_RemoveAllScriptObjs(ScriptEngine* e)` (0x27d050):
/// remove the first 20 script objects, calling virtual slot 3 on each.
pub fn script_engine_remove_all_script_objs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 18, 0x10);
        sqsp(c, 16, 0x30);
        c.g[S2][0] = c.g[A0][0];
        sqsp(c, 17, 0x20);
        c.g[S0][0] = c.g[S2][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        c.g[A1][0] = 0;
        loop {
            c.cycles += 4;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            let empty = c.g[V0][0] == 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !empty {
                c.cycles += 5;
                c.g[V1][0] = Addr(p(r32(c, S2), 0x2a8)).lw(c);
                c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x18)).lh(c);
                c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x1c)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = abi::addu(c.g[S2][0], c.g[A0][0]);
                call::call_ptr(c, target, 0x0027_d09c)?;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 20) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S1][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_d078);
            }
        }
        c.cycles += 6;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int ScriptEngine_RemoveBinding(ScriptEngine* e, ScriptMessage* msg)`
/// (0x27d390): clear the binding for the variable index in `msg+4`, returning
/// 1.
pub fn script_engine_remove_binding(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = c.g[A1][0];
        sqsp(c, 16, 0x10);
        c.g[V1][0] = 0x14;
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        c.g[V0][0] = Addr(p(r32(c, A2), 4)).lw(c);
        c.mult(0, V0, V0, V1);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x50));
        c.g[S0][0] = abi::addu(c.g[A0][0], c.g[V0][0]);
        c.g[V1][0] = Addr(p(r32(c, S0), 0xc)).lw(c);
        if c.g[V1][0] != c.g[A1][0] {
            c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[A2][0];
            callee::SCRIPT_BINDING_CLEAR_LISTENER.call(c, 0x0027_d3cc)?;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        }
        c.cycles += 12;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        c.g[V0][0] = 1;
        Addr(p(c.g[A0][0] as u32, 4)).sw(c, c.g[V1][0]);
        Addr(p(r32(c, S0), 0x10)).sw(c, c.g[V1][0]);
        Addr(p(r32(c, S0), 4)).sw(c, c.g[V1][0]);
        Addr(c.g[S0][0] as u32).sw(c, 0);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        Addr(p(r32(c, S0), 0xc)).sw(c, 0);
        lqsp(c, 16, 0x10);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `int ScriptEngine_CollectFreeBindings(ScriptEngine* e, char* end,
/// void** out, int max)` (0x27d578): walk the 30 variable slots at `e+0x50`,
/// collecting live, non-type-1 variables that are unbound (`link < 0`) or
/// bound to `end`; returns how many were collected.
pub fn script_engine_collect_free_bindings(c: &mut Cpu) {
    c.cycles += 4;
    let mut a0 = (c.g[A0][0] as u32).wrapping_add(0x50);
    c.g[A0][0] = a0 as u64;
    let a1 = c.g[A1][0];
    let mut a2 = c.g[A2][0] as u32;
    let a3 = c.g[A3][0];
    let mut t1: u64 = 0;
    let mut t2: u64 = 0;
    if a3 != 0 {
        c.cycles += 2;
        let t3: u64 = 1;
        let mut t0 = a0;
        loop {
            c.cycles += 3;
            let v1 = Addr(a0).lw(c) as u64;
            c.g[V1][0] = v1;
            let mut tail = false;
            if v1 == 0 {
                t2 = sx((t2 as u32).wrapping_add(1));
            } else {
                c.cycles += 3;
                let ty = Addr(p(a0, 0xc)).lw(c) as u64;
                if ty == t3 {
                    t2 = sx((t2 as u32).wrapping_add(1));
                } else {
                    c.cycles += 3;
                    // `lw` sign-extends: the bound test and the `end`
                    // comparison are 64-bit, exactly as the original.
                    let link = Addr(p(a0, 0x10)).lw(c);
                    c.g[A0][0] = link;
                    if (link as i64) < 0 {
                        Addr(a2).sw(c, v1);
                        tail = true;
                    } else {
                        c.cycles += 2;
                        if link != a1 {
                            t2 = sx((t2 as u32).wrapping_add(1));
                        } else {
                            c.cycles += 1;
                            Addr(a2).sw(c, v1);
                            tail = true;
                        }
                    }
                }
            }
            if tail {
                c.cycles += 3;
                t1 = sx((t1 as u32).wrapping_add(1));
                a2 = a2.wrapping_add(4);
                c.g[A2][0] = a2 as u64;
                t2 = sx((t2 as u32).wrapping_add(1));
            }
            c.cycles += 3;
            let more_slots = (t2 as i64) < 30;
            t0 = t0.wrapping_add(0x14);
            if !more_slots {
                break;
            }
            c.cycles += 2;
            let cont = t1 != a3;
            a0 = t0;
            c.g[A0][0] = a0 as u64;
            if !cont {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_d590);
            }
        }
    }
    c.cycles += 2;
    c.g[T1][0] = t1;
    c.g[T2][0] = t2;
    c.g[V0][0] = t1;
    abi::ret(c);
}

/// `int MenuSysItem_HandleInput(MenuSysItem* item)` (0x2ca2b0): map the
/// current input code to an item action (8: 1, 7: 2, 0: 4, 1: 3; otherwise 0).
pub fn menu_sys_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MENU_ITEM_GET_INPUT_CODE.call(c, 0x002c_a2c0)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 1;
        let code_is_1 = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        let result: u64;
        if code_is_1 {
            // b_2ca310 -> +1, v0 = 3 -> b_2ca314
            c.cycles += 1;
            result = 3;
        } else if c.g[V0][0] == 0 {
            // code >= 2: fall into b_2ca300 with v0 = 7.
            c.cycles += 2;
            c.g[V0][0] = 7;
            c.cycles += 2;
            let is_7 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 8;
            if is_7 {
                c.cycles += 2;
                result = 2;
            } else {
                c.cycles += 2;
                let not_8 = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 0;
                if not_8 {
                    result = 0;
                } else {
                    c.cycles += 2;
                    result = 1;
                }
            }
        } else {
            // code == 0: b_2ca308 -> v0 = 4
            c.cycles += 2;
            c.g[V0][0] = 7;
            c.cycles += 2;
            c.cycles += 2;
            result = 4;
        }
        c.g[V0][0] = result;
        c.cycles += 1;
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}

/// `void MenuSysPage_OnActivate(MenuSysPage* page, int wrap)` (0x2cbf60):
/// store the wrap flag, clamp the selection to the last item, activate the
/// selected item (virtual `+0x34` with argument 2) and validate the selection.
pub fn menu_sys_page_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(p(r32(c, S0), 0x124)).sw(c, c.g[A1][0]);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            Addr(p(r32(c, S0), 4)).sw(c, c.g[V0][0]);
            c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
        }
        c.cycles += 15;
        c.g[A1][0] = 2;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = abi::addu(c.g[S0][0], c.g[V0][0]);
        c.g[A2][0] = Addr(p(c.g[V0][0] as u32, 0xc)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, A2), 0x10)).lw(c);
        c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x30)).lh(c);
        c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x34)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = abi::addu(c.g[A2][0], c.g[A0][0]);
        call::call_ptr(c, target, 0x002c_bfb8)?;
        c.g[A0][0] = c.g[S0][0];
        callee::MENU_PAGE_VALIDATE_SELECTION.call(c, 0x002c_bfc0)?;
        lqsp(c, 16, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `int MenuSysSubmenuItem_HandleInput(MenuSysSubmenuItem* item)`
/// (0x2ccd20): run the base item input, and when the item is enabled, produced
/// no action and the input code is 6 (confirm), push the submenu page named
/// at `item+0x18`.
pub fn menu_sys_submenu_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        sqsp(c, 16, 0x20);
        sqsp(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_ITEM_HANDLE_INPUT.call(c, 0x002c_cd38)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        callee::MENU_ITEM_IS_DISABLED.call(c, 0x002c_cd44)?;
        let disabled = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S1][0];
        let mut restore_16 = false;
        if !disabled {
            c.cycles += 2;
            if c.g[S1][0] != 0 {
                restore_16 = true;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                callee::MENU_ITEM_GET_INPUT_CODE.call(c, 0x002c_cd5c)?;
                c.g[V1][0] = 6;
                let not_confirm = c.g[V0][0] != c.g[V1][0];
                c.g[V0][0] = c.g[S1][0];
                if !not_confirm {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[A1][0] = Addr(p(r32(c, S0), 0x18)).lw(c);
                    c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x124)).lw(c);
                    callee::MENU_PUSH_PAGE.call(c, 0x002c_cd78)?;
                    c.g[V0][0] = c.g[S1][0];
                }
            }
        }
        if !restore_16 {
            c.cycles += 1;
        }
        lqsp(c, 16, 0x20);
        c.cycles += 4;
        lqsp(c, 17, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `void MenuSysFloatItem_Draw(MenuSysFloatItem* item, int a1)` (0x2ce100):
/// format `*item->value * item->scale + item->bias` into a stack string, draw
/// the row, and destroy the temporary string.
pub fn menu_sys_float_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 16, 0x30);
        sqsp(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        sdsp(c, 0x10, c.g[RA][0]);
        c.g[A0][0] = c.g[SP][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 0x18)).lw(c);
        c.lwc1(1, p(r32(c, S0), 0x24));
        c.lwc1(12, c.g[V0][0] as u32);
        c.lwc1(0, p(r32(c, S0), 0x28));
        c.mul_s(12, 12, 1);
        c.add_s(12, 12, 0);
        callee::MENU_FORMAT_FLOAT.call(c, 0x002c_e138)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[T0][0] = Addr(c.g[SP][0] as u32).lw(c);
        c.g[A2][0] = Addr(p(r32(c, A0), 0x14)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        callee::MENU_ITEM_DRAW_ROW.call(c, 0x002c_e150)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call::call_ptr(c, 0x0031_8260, 0x002c_e15c)?;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        c.g[RA][0] = Addr(p(c.g[SP][0] as u32, 0x10)).ld(c);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `int InSeq_StartSequence(InSeq* seq, void* src)` (0x272938): take the first
/// free channel node, detach it from the free list, set up the instance and
/// push it onto the used list; returns the channel id.
pub fn in_seq_start_sequence(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        sqsp(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        sqsp(c, 16, 0x20);
        c.g[S0][0] = Addr(p(r32(c, S1), 0x18)).lw(c);
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x50)).lw(c);
        Addr(p(r32(c, S1), 0x18)).sw(c, c.g[V0][0]);
        Addr(p(r32(c, S0), 0x50)).sw(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S1), 0x18)).lw(c);
        if c.g[V0][0] != 0 {
            Addr(p(c.g[V0][0] as u32, 0x54)).sw(c, 0);
        }
        c.cycles += 9;
        c.g[A3][0] = Addr(p(r32(c, S0), 4)).lw(c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(8));
        c.g[A2][0] = c.g[S1][0];
        callee::IN_SEQ_INSTANCE_SETUP.call(c, 0x0027_2978)?;
        c.g[V1][0] = Addr(p(r32(c, S1), 0x1c)).lw(c);
        Addr(p(r32(c, S0), 0x50)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(p(r32(c, S1), 0x1c)).lw(c);
        if c.g[V0][0] != 0 {
            Addr(p(c.g[V0][0] as u32, 0x54)).sw(c, c.g[S0][0]);
        }
        c.cycles += 7;
        Addr(p(r32(c, S1), 0x1c)).sw(c, c.g[S0][0]);
        lqsp(c, 17, 0x10);
        c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
        lqsp(c, 16, 0x20);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `void RiderGateSub_UpdateWaitPush(RiderGateSub* g, AiBits* bits)`
/// (0x12c0c0): when the signed 6-bit field at `bits+0` scaled by 0.032258 is
/// positive, play the landing animation (id 2) and set `g->state = 3`.
pub fn rider_gate_sub_update_wait_push(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.lwc1(2, gp(c, -0x783c));
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.mtc1(0, 1);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 14);
        let v = c.g[V0][0] as u32;
        c.g[V0][0] = sx(((v as i32) >> 26) as u32);
        c.mtc1(V0, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 2);
        c.c_lt_s(1, 0);
        let not_positive = !c.fcc();
        c.g[S0][0] = c.g[A0][0];
        if !not_positive {
            c.cycles += 9;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x14)).lw(c);
            c.g[A1][0] = 2;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 12);
            c.g[A2][0] = 0;
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x784)).lw(c);
            call::call_ptr(c, 0x0031_28e8, 0x0012_c118)?;
            c.g[V1][0] = 3;
            Addr(c.g[S0][0] as u32).sw(c, c.g[V1][0]);
        }
        c.cycles += 4;
        lqsp(c, 16, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `void Irradiance_Scale(Vec4* out, const Vec4* in, float scale)` (0x3897a0):
/// scale each of ten vectors by `scale`, first into a stack buffer (through
/// VU0, as the original does), then copy the buffer to `out`.
pub fn irradiance_scale(c: &mut Cpu) {
    c.cycles += 4;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
    c.g[A2][0] = 9;
    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
    c.g[V1][0] = c.g[SP][0];
    loop {
        c.cycles += 12;
        c.mfc1(2, 12);
        c.lqc2(4, c.g[A1][0] as u32);
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, p(c.g[SP][0] as u32, 0xb0));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
        lqsp(c, 2, 0xb0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
        sqsp(c, 2, 0xa0);
        c.sq(2, c.g[V1][0] as u32);
        let more = (c.g[A2][0] as i64) >= 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0038_97b0);
        }
    }
    c.cycles += 2;
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = c.g[SP][0];
    loop {
        c.cycles += 7;
        c.lq(5, c.g[V0][0] as u32);
        c.lq(6, p(c.g[V0][0] as u32, 0x10));
        c.sq(5, c.g[V1][0] as u32);
        c.sq(6, p(c.g[V1][0] as u32, 0x10));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
        let more = c.g[V0][0] != c.g[A3][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0038_97e8);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
    c.pc = ra;
}

/// The shared body of the five load-state "transition in" functions: look the
/// screen object up by hash and, when found, format the hint text and set it.
fn load_state_transition_in(c: &mut Cpu, entry: u32) -> call::Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0xf0);
    sqsp(c, 16, 0xe0);
    c.g[S0][0] = c.g[A0][0];
    sdsp(c, 0xd0, c.g[RA][0]);    c.g[A0][0] = 0x004a_2bc8;
    call::call_ptr(c, 0x0031_7670, entry + 0x1c)?;
    c.g[A0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call::call_ptr(c, 0x0039_d860, entry + 0x28)?;
    c.g[S0][0] = c.g[V0][0];
    let missing = c.g[S0][0] == 0;
    c.g[V0][0] = Addr(gp(c, 0x2a74)).lw(c);
    if !missing {
        c.cycles += 10;
        c.g[A1][0] = 0x004a_2bd0;
        c.lwc1(0, p(c.g[V0][0] as u32, 0x38));
        c.cvt_w_s(1, 0);
        c.mfc1(6, 1);
        c.g[A0][0] = c.g[SP][0];
        call::call_ptr(c, 0x002c_27c0, entry + 0x50)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        call::call_ptr(c, 0x003a_0e90, entry + 0x5c)?;
    }
    c.cycles += 5;
    lqsp(c, 16, 0xe0);
    c.g[V0][0] = 0;
    c.g[RA][0] = Addr(p(c.g[SP][0] as u32, 0xd0)).ld(c);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
    c.pc = ra;
    Ok(())
}

/// `int cGameLoadStateOutLodge_TransitionIn(LoadState* s)` (0x245698).
pub fn c_game_load_state_out_lodge_transition_in(c: &mut Cpu) {
    guest(c, |c| load_state_transition_in(c, 0x0024_5698))
}

/// `int cGameLoadStateConquer_TransitionIn(LoadState* s)` (0x2458b8).
pub fn c_game_load_state_conquer_transition_in(c: &mut Cpu) {
    guest(c, |c| load_state_transition_in(c, 0x0024_58b8))
}

/// `int cFELoadHintState_TransitionIn(LoadState* s)` (0x245ae0).
pub fn c_fe_load_hint_state_transition_in(c: &mut Cpu) {
    guest(c, |c| load_state_transition_in(c, 0x0024_5ae0))
}

/// `int cFELoadState_TransitionIn(LoadState* s)` (0x245c60).
pub fn c_fe_load_state_transition_in(c: &mut Cpu) {
    guest(c, |c| load_state_transition_in(c, 0x0024_5c60))
}

/// `int cFELoadStateInLodge_TransitionIn(LoadState* s)` (0x245dc0).
pub fn c_fe_load_state_in_lodge_transition_in(c: &mut Cpu) {
    guest(c, |c| load_state_transition_in(c, 0x0024_5dc0))
}

/// `MultiParticle* MultiParticleMan_CreateSlot(MultiParticleMan* m, int i,
/// void* a2, void* a3)` (0x357a78): create slot `i`'s particle instance on
/// first use.
pub fn multi_particle_man_create_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        sqsp(c, 16, 0x30);
        sqsp(c, 17, 0x20);
        c.g[S0][0] = abi::addu(c.g[A0][0], c.g[A1][0]);
        sqsp(c, 18, 0x10);
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let occupied = c.g[V0][0] != 0;
        c.g[S2][0] = c.g[A2][0];
        if !occupied {
            c.cycles += 11;
            c.g[A1][0] = 0x0048_ea00;
            c.g[A0][0] = 0x50;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call::call_ptr(c, 0x0031_7d70, 0x0035_7abc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S1][0];
            call::call_ptr(c, 0x0035_7dd8, 0x0035_7acc)?;
            Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        }
        c.cycles += 6;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `void PathData_Free(PathData* d)` (0x26af00): free the three owned
/// buffers and clear their pointers.
pub fn path_data_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p(r32(c, S0), 0xc)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call::call_ptr(c, 0x0031_7e98, 0x0026_af24)?;
            c.g[A0][0] = Addr(p(r32(c, S0), 0x14)).lw(c);
        } else {
            c.g[A0][0] = Addr(p(r32(c, S0), 0x14)).lw(c);
        }
        c.cycles += 2;
        let clear = c.g[A0][0] == 0;
        Addr(p(r32(c, S0), 0xc)).sw(c, 0);
        if !clear {
            c.cycles += 2;
            call::call_ptr(c, 0x0031_7e98, 0x0026_af38)?;
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x48)).lw(c);
        let head_empty = c.g[A0][0] == 0;
        Addr(p(r32(c, S0), 0x14)).sw(c, 0);
        let mut tail_clear = false;
        if !head_empty {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x4c)).lw(c);
            if c.g[V0][0] == 0 {
                Addr(p(r32(c, S0), 0x4c)).sw(c, 0);
            } else {
                c.cycles += 2;
                call::call_ptr(c, 0x0031_7e98, 0x0026_af58)?;
                tail_clear = true;
            }
        } else {
            tail_clear = true;
        }
        if tail_clear {
            c.cycles += 1;
            Addr(p(r32(c, S0), 0x4c)).sw(c, 0);
        }
        c.cycles += 5;
        Addr(p(r32(c, S0), 0x48)).sw(c, 0);
        lqsp(c, 16, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `void RiderAirState_PlayLandAnim(Rider* r, float v)` (0x13a8f8): play the
/// hard-landing animation (0x41) and clear the landing flag when `v` is below
/// −2222.22, otherwise the soft landing (0x40).
pub fn rider_air_state_play_land_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.lwc1(0, gp(c, -0x71a0));
        let frame = Frame::open(c, 0x20);
        sqsp(c, 16, 0x10);
        c.c_lt_s(12, 0);
        frame.save_ra(c, 0);
        let hard = !c.fcc();
        c.g[S0][0] = c.g[A0][0];
        if !hard {
            c.cycles += 10;
            c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
            c.g[A1][0] = 0x41;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 12);
            c.g[A2][0] = 0;
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x784)).lw(c);
            call::call_ptr(c, 0x0031_28e8, 0x0013_a930)?;
            c.g[V1][0] = Addr(p(r32(c, S0), 4)).lw(c);
            Addr(p(c.g[V1][0] as u32, 0x2dc)).sw(c, 0);
        } else {
            c.cycles += 7;
            c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
            c.g[A1][0] = 0x40;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 12);
            c.g[A2][0] = 0;
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x784)).lw(c);
            call::call_ptr(c, 0x0031_28e8, 0x0013_a958)?;
        }
        c.cycles += 4;
        lqsp(c, 16, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `void GameStateReset_Update(GameStateReset* s)` (0x2357f8): once the reset
/// flag is armed and the script engine is idle, flag the "reset" field on the
/// race manager and let the state machine advance.
pub fn game_state_reset_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 0x1c)).lw(c);
        let idle = c.g[V0][0] == 0;
        c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
        if !idle {
            c.cycles += 4;
            c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
            call::call_ptr(c, 0x0027_aa80, 0x0023_581c)?;
            let busy = c.g[V0][0] != 0;
            c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
            if !busy {
                c.cycles += 7;
                c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
                c.g[A1][0] = 1;
                c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0x84)).lw(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x78)).lw(c);
                Addr(p(c.g[V1][0] as u32, 0x1a4)).sw(c, c.g[A1][0]);
                Addr(p(r32(c, S0), 0x1c)).sw(c, 0);
                c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
            }
        }
        c.cycles += 4;
        c.g[A1][0] = 1;
        call::call_ptr(c, 0x0027_9220, 0x0023_5848)?;
        if c.g[V0][0] != 0 {
            lqsp(c, 16, 0x10);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call::call_ptr(c, 0x0023_3aa0, 0x0023_5858)?;
            lqsp(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `void Rider_StopTransition(Rider* r)` (0x125038): when the rider is in a
/// view and its view index is valid, stop the transition on the current
/// camera.
pub fn rider_stop_transition(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x84)).lw(c);
        c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0x84)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = Addr(p(r32(c, S0), 0x870)).lw(c);
            if (c.g[A1][0] as i64) >= 0 {
                c.cycles += 6;
                call::call_ptr(c, 0x0023_0698, 0x0012_5070)?;
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x78)).lw(c);
                c.g[V0][0] = 1;
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
                    c.g[A1][0] = Addr(p(r32(c, S0), 0x870)).lw(c);
                    c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x84)).lw(c);
                    call::call_ptr(c, 0x0023_0698, 0x0012_5090)?;
                    c.g[A0][0] = c.g[V0][0];
                    call::call_ptr(c, 0x002e_4578, 0x0012_5098)?;
                }
            }
        }
        lqsp(c, 16, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `void WScriptMan_UnloadSection(WScriptMan* m, int section)` (0x308f70):
/// drop every process bound to the section from its three lists and free the
/// section's object slot.
pub fn wscript_man_unload_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        sqsp(c, 16, 0x20);
        sqsp(c, 17, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2b4));
        call::call_ptr(c, 0x0030_b0e8, 0x0030_8f90)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2b8));
        c.g[A1][0] = c.g[S0][0];
        call::call_ptr(c, 0x0030_b0e8, 0x0030_8f9c)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2bc));
        c.g[A1][0] = c.g[S0][0];
        call::call_ptr(c, 0x0030_b0e8, 0x0030_8fa8)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3cc));
        c.g[S1][0] = abi::addu(c.g[S1][0], c.g[S0][0]);
        c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
        let empty = c.g[A0][0] == 0;
        lqsp(c, 16, 0x20);
        if !empty {
            c.cycles += 4;
            c.g[A1][0] = 3;
            call::call_ptr(c, 0x0022_4df0, 0x0030_8fc8)?;
            Addr(c.g[S1][0] as u32).sw(c, 0);
            lqsp(c, 16, 0x20);
        }
        c.cycles += 4;
        lqsp(c, 17, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `void cFEStateOptions_Init(cFEStateOptions* s)` (0x188800): create the
/// options screen from the hash name and start its frame.
pub fn c_fe_state_options_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        sqsp(c, 17, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        sqsp(c, 16, 0x20);
        c.g[A0][0] = 0x0045_dd30;
        c.g[S0][0] = Addr(p(r32(c, S1), 0x10)).lw(c);
        call::call_ptr(c, 0x0031_7670, 0x0018_8824)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call::call_ptr(c, 0x0039_7cc0, 0x0018_8838)?;
        let no_screen = c.g[V0][0] == 0;
        Addr(p(r32(c, S1), 0x40)).sw(c, c.g[V0][0]);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call::call_ptr(c, 0x0039_c870, 0x0018_8850)?;
        }
        c.cycles += 8;
        c.g[V0][0] = 1;
        Addr(p(r32(c, S1), 0x48)).sw(c, 0);
        Addr(p(r32(c, S1), 0x4c)).sw(c, c.g[V0][0]);
        lqsp(c, 16, 0x20);
        lqsp(c, 17, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `void cFEPopupConfirm_v27(cFEPopupConfirm* self, const u16* src)`
/// (0x1d9ca8): set the popup's text from a source UTF-16 string.
pub fn c_fe_popup_confirm_v27(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 17, 0x20);
        sqsp(c, 18, 0x10);
        c.g[S1][0] = c.g[A1][0];
        sqsp(c, 16, 0x30);
        c.g[S2][0] = c.g[A0][0];
        let no_src = c.g[S1][0] == 0;
        frame.save_ra(c, 0);
        if !no_src {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            call::call_ptr(c, 0x002c_24d0, 0x001d_9cd0)?;
            let empty = c.g[V0][0] == 0;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x360));
            if !empty {
                c.cycles += 8;
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S0][0];
                call::call_ptr(c, 0x002c_2508, 0x001d_9ce4)?;
                c.g[V1][0] = 1;
                c.g[V0][0] = Addr(p(r32(c, S2), 0x40)).lw(c);
                Addr(p(r32(c, S2), 0xc4)).sw(c, c.g[S0][0]);
                let no_refresh = c.g[V0][0] == 0;
                Addr(p(r32(c, S2), 0xc0)).sw(c, c.g[V1][0]);
                if !no_refresh {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S2][0];
                    call::call_ptr(c, 0x001c_5dd8, 0x001d_9d00)?;
                }
            }
        }
        c.cycles += 6;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `void ScriptEngine_ResumeProcess(ScriptEngine* e, int idx)` (0x282720):
/// resume the in-sequence instance in slot `idx` and, when it was stopped but
/// has re-entered a runnable state, move its slot state to 3 or 4.
pub fn script_engine_resume_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[V0][0] = 0x24;
        let frame = Frame::open(c, 0x20);
        c.mult(0, A1, A1, V0);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c0));
        c.g[S0][0] = abi::addu(c.g[A0][0], c.g[A1][0]);
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        call::call_ptr(c, 0x0027_4a08, 0x0028_2744)?;
        c.g[V1][0] = Addr(p(r32(c, S0), 4)).lw(c);
        c.g[V0][0] = 5;
        if c.g[V1][0] != c.g[V0][0] {
            lqsp(c, 16, 0x10);
        } else {
            c.cycles += 4;
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x2c)).lw(c);
            if (c.g[V0][0] as i64) > 0 {
                lqsp(c, 16, 0x10);
            } else {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x20)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(3));
                c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                let t = c.g[V0][0] != 0;
                c.g[V0][0] = 4;
                if !t {
                    c.cycles += 1;
                    c.g[V0][0] = 3;
                }
                c.cycles += 2;
                Addr(p(r32(c, S0), 4)).sw(c, c.g[V0][0]);
                lqsp(c, 16, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `void BENewPlayerInt_ClearSharedCharRecord(int idx, int player)`
/// (0x147898): clear the character id and the "custom" flag of the player's
/// shared record.
pub fn benew_player_int_clear_shared_char_record(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 17, 0x20);
        c.g[S1][0] = c.g[A1][0];
        sqsp(c, 16, 0x30);
        sqsp(c, 18, 0x10);
        c.g[A0][0] = c.g[S1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = 0x1c;
        call::call_ptr(c, 0x0014_7798, 0x0014_78bc)?;
        c.mult(0, V0, V0, S2);
        c.g[S0][0] = 0x0053_4fe0;
        c.g[A0][0] = c.g[S1][0];
        c.g[V0][0] = abi::addu(c.g[V0][0], c.g[S0][0]);
        Addr(p(c.g[V0][0] as u32, 0x11)).sb(c, 0);
        call::call_ptr(c, 0x0014_7798, 0x0014_78d8)?;
        c.mult(0, V1, V0, S2);
        c.g[A0][0] = 0xffff_ffff_ffff_fffb;
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        c.g[V0][0] = abi::addu(c.g[V1][0], c.g[S0][0]);
        c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x10)).lw(c);
        lqsp(c, 16, 0x30);
        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
        Addr(p(c.g[V0][0] as u32, 0x10)).sw(c, c.g[V1][0]);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `void cGameModeMan_restartHeat(GameModeMan* gm)` (0x2382d8): restart the
/// current mode object (virtual slot 2) and clear the six mode-history words.
pub fn game_mode_man_reset_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x0053_6668;
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(r32(c, A0), 4)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = abi::addu(c.g[V1][0], c.g[V0][0]);
        c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x10)).lh(c);
        c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x14)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = abi::addu(c.g[A1][0], c.g[A0][0]);
        call::call_ptr(c, target, 0x0023_830c)?;
        c.g[V0][0] = 0x0053_66d0;
        c.g[V1][0] = 5;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
        loop {
            c.cycles += 7;
            Addr(c.g[V0][0] as u32).sw(c, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0023_8320);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}

/// `void WorldTriggerManager_StopAll(WorldTriggerManager* m)` (0x2b5e68):
/// stop and clear every active trigger in the 40 trigger slots.
pub fn world_trigger_manager_stop_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        sqsp(c, 16, 0x40);
        sqsp(c, 17, 0x30);
        c.g[S0][0] = 0;
        sqsp(c, 18, 0x20);
        c.g[S1][0] = c.g[A0][0];
        sqsp(c, 19, 0x10);
        c.g[S2][0] = 0x27;
        frame.save_ra(c, 0);
        c.g[S3][0] = 1;
        loop {
            c.cycles += 5;
            c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
            c.g[V1][0] = abi::addu(c.g[S0][0], c.g[V1][0]);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let active = c.g[V0][0] == c.g[S3][0];
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            if active {
                c.cycles += 4;
                Addr(c.g[V1][0] as u32).sw(c, 0);
                c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[A0][0] = abi::addu(c.g[A0][0], c.g[S0][0]);
                call::call_ptr(c, 0x002b_5758, 0x002b_5eb4)?;
            }
            c.cycles += 2;
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x70));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002b_5e90);
            }
        }
        c.cycles += 7;
        lqsp(c, 16, 0x40);
        lqsp(c, 17, 0x30);
        lqsp(c, 18, 0x20);
        lqsp(c, 19, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = ra;
        Ok(())
    });
}

/// `void DataInstanceMan_RemoveAll(DataInstanceMan* m)` (0x275b08): remove
/// every live instance of the manager.
pub fn data_instance_man_remove_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 16, 0x30);
        sqsp(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        sqsp(c, 18, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !none {
            c.cycles += 1;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
                c.g[A1][0] = abi::addu(c.g[S2][0], c.g[V0][0]);
                c.g[V1][0] = Addr(p(c.g[A1][0] as u32, 0x10)).lw(c);
                if c.g[V1][0] != 0 {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    call::call_ptr(c, 0x0027_5a20, 0x0027_5b4c)?;
                }
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_5b30);
                }
            }
        }
        c.cycles += 6;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `void InputPreproc_SkipLine(InputPreproc* p, Stream* s)` (0x326678): consume
/// characters up to and including the next newline (0x0a), or until the
/// stream ends; then advance the stream's line counter.
pub fn input_preproc_skip_line(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 16, 0x30);
        sqsp(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        sqsp(c, 18, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = 0xa;
        c.g[V0][0] = Addr(p(r32(c, S0), 0xc)).lw(c);
        loop {
            c.cycles += 2;
            let eof = (c.g[V0][0] as i64) < 0;
            c.g[A0][0] = c.g[S0][0];
            if eof {
                break;
            }
            c.cycles += 5;
            c.g[A1][0] = c.g[S1][0];
            call::call_ptr(c, 0x0032_6478, 0x0032_66b0)?;
            c.g[V0][0] = Addr(p(r32(c, S0), 0xc)).lw(c);
            let not_newline = c.g[V0][0] != c.g[S2][0];
            c.g[A0][0] = c.g[S0][0];
            if !not_newline {
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                call::call_ptr(c, 0x0032_6478, 0x0032_66c4)?;
                c.g[V0][0] = Addr(p(r32(c, S1), 0xc)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                Addr(p(r32(c, S1), 0xc)).sw(c, c.g[V0][0]);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_66a0);
            }
        }
        c.cycles += 6;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `void PadDevice_StopActuators(PadDevice* d)` (0x326cf0): call the actuator
/// setter with 0.0f for every actuator the device reports.
pub fn pad_device_stop_actuators(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0032_6cf0 => {
                    c.cycles += 7;
                    let frame = Frame::open(c, 0x30);
                    frame.save(c, 16, 0x20);
                    frame.save(c, 17, 0x10);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[S1][0] = 0;
                    pc = 0x0032_6d24;
                }
                0x0032_6d0c => {
                    c.cycles += 1;
                    pc = 0x0032_6d10;
                }
                0x0032_6d10 => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                    c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x20)).lh(c);
                    c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x24)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
                    call::call_ptr(c, target, 0x0032_6d24)?;
                    pc = 0x0032_6d24;
                }
                0x0032_6d24 => {
                    c.cycles += 10;
                    c.g[V1][0] = Addr(r32(c, S0)).lw(c);
                    c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x18)).lh(c);
                    c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x1c)).lw(c);
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
                    call::call_ptr(c, target, 0x0032_6d38)?;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.mtc1(0, 12);
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    if more {
                        if crate::events::back_edge_unwinds(c, 0x0032_6d10) {
                            return Ok(());
                        }
                        pc = 0x0032_6d10;
                    } else {
                        pc = 0x0032_6d4c;
                    }
                }
                0x0032_6d4c => {
                    c.cycles += 5;
                    let frame = Frame { size: 0x30 };
                    frame.restore(c, 16, 0x20);
                    frame.restore(c, 17, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0032_6cf0, 0x0032_6d60);
                    return Ok(());
                }
            }
        }
    });
}

/// `PS2AIVisualEffectsMan* PS2AIVisualEffectsMan_ctor(PS2AIVisualEffectsMan* m)`
/// (0x2f03c8): construct the AI visual-effects manager, allocating and
/// initialising its avalanche effect.
pub fn ps2_ai_visual_effects_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        sqsp(c, 17, 0x10);
        sqsp(c, 16, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::AI_VISUAL_EFFECTS_MAN_CTOR.call(c, 0x002f_03e0)?;
        c.g[V0][0] = 0x0048_7d50;
        c.g[A1][0] = 0x0048_7568;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        Addr(p(r32(c, S1), 4)).sw(c, c.g[V0][0]);
        c.g[A0][0] = 4;
        call::call_ptr(c, 0x0031_7d70, 0x002f_0404)?;
        c.g[V1][0] = 0x0048_8648;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        Addr(c.g[S0][0] as u32).sw(c, c.g[V1][0]);
        call::call_ptr(c, 0x002d_6410, 0x002f_041c)?;
        Addr(c.g[S1][0] as u32).sw(c, c.g[S0][0]);
        c.g[V0][0] = c.g[S1][0];
        lqsp(c, 16, 0x20);
        lqsp(c, 17, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `void LightMan_v11(void* obj, int b, int c, int d, float f)` (0x38df98):
/// look up the light manager's virtual method and hand the arguments on.
pub fn light_man_v11(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x50);
        sqsp(c, 16, 0x30);
        sqsp(c, 17, 0x20);
        c.g[S0][0] = c.g[A3][0];
        sqsp(c, 18, 0x10);
        c.g[S1][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        frame.save_fpr(c, 20, 0x40);
        c.mov_s(20, 12);
        c.g[A2][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[V1][0] = Addr(p(c.g[A2][0] as u32, 0x10)).lh(c);
        c.g[V0][0] = Addr(p(c.g[A2][0] as u32, 0x14)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = abi::addu(c.g[A0][0], c.g[V1][0]);
        call::call_ptr(c, target, 0x0038_dfd4)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S0][0];
        c.mov_s(12, 20);
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        callee::LIGHT_MAN_EMPTY_HOOK.call(c, 0x0038_dfec)?;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = ra;
        Ok(())
    });
}

/// `void WorldMemman_dtor(WorldMemman* m, int flags)` (0x3a7aa8): destroy the
/// two world block pools, then optionally free the manager itself.
pub fn world_memman_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 17, 0x20);
        sqsp(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        sqsp(c, 16, 0x30);
        c.g[S2][0] = c.g[A1][0];
        let null = c.g[S1][0] == 0;
        frame.save_ra(c, 0);
        if !null {
            c.cycles += 3;
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3c));
            let single = c.g[S1][0] == c.g[S0][0];
            c.g[V0][0] = c.g[S2][0] & 1;
            if !single {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x14));
                loop {
                    c.cycles += 5;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S0][0];
                    call::call_ptr(c, 0x003a_76c0, 0x003a_7ae4)?;
                    let more = c.g[S1][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x14));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_7ad8);
                    }
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[S2][0] & 1;
        c.cycles += 2;
        let no_free = c.g[V0][0] == 0;
        lqsp(c, 16, 0x30);
        if !no_free {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call::call_ptr(c, 0x0031_7e50, 0x003a_7b00)?;
            lqsp(c, 16, 0x30);
        }
        c.cycles += 5;
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `void cWorldCache_FlagSectionObjects(cWorldCache* cache, int idx)`
/// (0x3a8f10): set bit 6 of the flags word of every object in a section.
pub fn c_world_cache_flag_section_objects(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0x58;
    c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.mult(0, V0, A1, V0);
    c.g[A2][0] = Addr(p(r32(c, A0), 8)).lw(c);
    c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x14)).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A1][0] = abi::addu(c.g[A1][0], c.g[A2][0]);
    c.g[A2][0] = 0;
    c.g[V0][0] = abi::addu(c.g[V0][0], c.g[A0][0]);
    c.g[A3][0] = Addr(p(c.g[V0][0] as u32, 0x22)).lh(c);
    let none = c.g[A3][0] == 0;
    c.g[A1][0] = Addr(c.g[A1][0] as u32).lw(c);
    if !none {
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 0xc)).lw(c);
            c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
            c.g[V1][0] = abi::addu(c.g[V1][0], c.g[V0][0]);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32) >> 8);
            let nonzero = c.g[A0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            if !nonzero {
                c.cycles += 1;
                c.g[A0][0] = 0;
            }
            c.cycles += 6;
            c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0xa)).lhu(c);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            c.g[V1][0] = (c.g[A2][0] < c.g[A3][0]) as u64;
            c.g[V0][0] = c.g[V0][0] | 0x40;
            let more = c.g[V1][0] != 0;
            Addr(p(c.g[A0][0] as u32, 0xa)).sh(c, c.g[V0][0]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_8f40);
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
}

/// `void CrowdMan2d_RemoveAll(CrowdMan2d* m)` (0x229398): clear all 128 crowd
/// slots that are not already empty.
pub fn crowd_man2d_remove_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        sqsp(c, 18, 0x20);
        sqsp(c, 19, 0x10);
        c.g[S2][0] = 0xffff_ffff_ffff_ffff;
        sqsp(c, 16, 0x40);
        c.g[S3][0] = c.g[A0][0];
        sqsp(c, 17, 0x30);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(8));
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            let empty = c.g[V0][0] == c.g[S2][0];
            c.g[A0][0] = c.g[S3][0];
            if !empty {
                c.cycles += 2;
                c.g[A1][0] = c.g[S1][0];
                call::call_ptr(c, 0x0022_9b90, 0x0022_93dc)?;
            }
            c.cycles += 4;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S1][0] as i64) < 128) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_93c8);
            }
        }
        c.cycles += 7;
        lqsp(c, 16, 0x40);
        lqsp(c, 17, 0x30);
        lqsp(c, 18, 0x20);
        lqsp(c, 19, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = ra;
        Ok(())
    });
}

/// `int cWorldCache_StreamerActivateSection(cWorldCache* cache, int i)`
/// (0x3a99d8): move section `i`'s streamer state from 5 (requested) to 6
/// (active), flagging its objects; returns 1 when active.
pub fn c_world_cache_streamer_activate_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, A3), 4)).lw(c);
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(p(c.g[V0][0] as u32, 8)).lw(c);
            c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
            let out_of_range = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
            if !out_of_range {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x14));
                c.g[V1][0] = abi::addu(c.g[V1][0], c.g[V0][0]);
                c.g[A0][0] = 5;
                c.g[A2][0] = Addr(c.g[V1][0] as u32).lw(c);
                let requested = c.g[A2][0] == c.g[A0][0];
                if !requested {
                    c.g[V1][0] = 6;
                    c.cycles += 2;
                    let active = c.g[A2][0] == c.g[V1][0];
                    c.g[V0][0] = 1;
                    if active {
                        // returns 1
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = 6;
                    Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
                    c.g[A0][0] = Addr(c.g[A3][0] as u32).lw(c);
                    call::call_ptr(c, 0x003a_8f10, 0x003a_9a28)?;
                    c.g[V0][0] = 1;
                }
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}

/// `cUITextScroll* cUITextScroll_ctor(cUITextScroll* s, int a1, int a2)`
/// (0x3a1148).
pub fn c_ui_text_scroll_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x20);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_OBJECT_CTOR.call(c, 0x003a_115c)?;
        c.g[V1][0] = Addr(p(r32(c, S0), 0x74)).lbu(c);
        c.g[V0][0] = 0x0049_4278;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8c));
        c.g[V1][0] = c.g[V1][0] & 0xc6;
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        Addr(p(r32(c, S0), 0x74)).sb(c, c.g[V1][0]);
        callee::UI_LIST_CTOR.call(c, 0x003a_117c)?;
        Addr(p(r32(c, S0), 0xb8)).sw(c, 0);
        c.g[A0][0] = c.g[S0][0];
        Addr(p(r32(c, S0), 0xbc)).sw(c, 0);
        c.g[A1][0] = 0;
        Addr(p(r32(c, S0), 0xc0)).sw(c, 0);
        Addr(p(r32(c, S0), 0xc4)).sh(c, 0);
        Addr(p(r32(c, S0), 0xc6)).sh(c, 0);
        Addr(p(r32(c, S0), 0xc8)).sh(c, 0);
        Addr(p(r32(c, S0), 0xcc)).sw(c, 0);
        callee::CUI_TEXT_SCROLL_SET_NAV_BUTTONS.call(c, 0x003a_11a4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        lqsp(c, 16, 0x10);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `UIVector* UIVector_ctor(UIVector* v, int a1, int a2)` (0x3a3280).
pub fn ui_vector_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x20);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_OBJECT_CTOR.call(c, 0x003a_3294)?;
        c.g[V1][0] = Addr(p(r32(c, S0), 0x74)).lw(c);
        c.g[A1][0] = 0xffff_ffff_ffff_fffe;
        c.g[A2][0] = 0xffff_ffff_ffff_fffd;
        c.g[V0][0] = 0x1_0000;
        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.g[V1][0] = c.g[V1][0] & c.g[A2][0];
        c.g[A0][0] = 0x0049_41b8;
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        Addr(p(r32(c, S0), 8)).sw(c, c.g[A0][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(p(r32(c, S0), 0x74)).sw(c, c.g[V1][0]);
        for off in [0x78u32, 0x7c, 0x80, 0x84, 0x88, 0x8c] {
            Addr(p(r32(c, S0), off)).sw(c, 0);
        }
        frame.restore_ra(c, 0);
        lqsp(c, 16, 0x10);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `void MultiParticle_RemoveInstance(MultiParticle* p, int key)`
/// (0x3581f0): remove `key` from the instance array (swap with the last) or
/// mark the slot empty.
pub fn multi_particle_remove_instance(c: &mut Cpu) {
    c.cycles += 3;
    c.g[T0][0] = Addr(p(r32(c, A0), 4)).lw(c);
    let none = (c.g[T0][0] as i64) <= 0;
    c.g[A3][0] = 0;
    if none {
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.pc = ra;
        return;
    }
    c.cycles += 3;
    c.g[T2][0] = 0xffff_ffff_ffff_ffff;
    c.g[T1][0] = Addr(p(r32(c, A0), 0xc)).lw(c);
    loop {
        c.cycles += 5;
        c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
        c.g[A2][0] = abi::addu(c.g[V0][0], c.g[T1][0]);
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
        if c.g[V1][0] != c.g[A1][0] {
            c.g[T0][0] = Addr(p(r32(c, A0), 4)).lw(c);
            c.cycles += 4;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[T1][0] = Addr(p(r32(c, A0), 0xc)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0035_8208);
                }
                continue;
            }
            break;
        } else {
            c.cycles += 4;
            c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_sub(1));
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V1][0] as i64)) as u64;
            let last = c.g[V0][0] == 0;
            Addr(p(r32(c, A0), 4)).sw(c, c.g[V1][0]);
            if last {
                c.cycles += 2;
                Addr(c.g[A2][0] as u32).sw(c, c.g[T2][0]);
            } else {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = abi::addu(c.g[V0][0], c.g[T1][0]);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                Addr(c.g[A2][0] as u32).sw(c, c.g[V1][0]);
            }
            let ra = c.g[RA][0] as u32;
            c.pc = ra;
            return;
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
}

/// `void Avalanche_SetHistoryData(void* data)` (0x2d9fb8): replace the
/// avalanche history data, freeing the old block and re-resolving the new
/// data's pointers when it carries the magic tag.
pub fn avalanche_set_history_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call::call_ptr(c, 0x002d_9b40, 0x002d_9fcc)?;
        c.g[A0][0] = Addr(gp(c, 0x9c4)).lw(c);
        let no_old = c.g[A0][0] == 0;
        c.g[V0][0] = Addr(gp(c, 0x9c8)).lw(c);
        let mut set = false;
        if no_old {
            set = true;
        } else {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                Addr(gp(c, 0x9c4)).sw(c, c.g[S0][0]);
            } else {
                c.cycles += 2;
                call::call_ptr(c, 0x0031_7e98, 0x002d_9fe8)?;
                set = true;
            }
        }
        if set {
            c.cycles += 1;
            Addr(gp(c, 0x9c4)).sw(c, c.g[S0][0]);
        }
        c.cycles += 6;
        c.g[V0][0] = 0x02be_ef00;
        Addr(gp(c, 0x9c8)).sw(c, 0);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        let magic = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 1;
        if !magic {
            c.cycles += 3;
            Addr(gp(c, 0x9c4)).sw(c, 0);
            Addr(gp(c, 0x9d4)).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 2;
            Addr(gp(c, 0x9d4)).sw(c, 0);
            call::call_ptr(c, 0x002d_a028, 0x002d_a018)?;
        }
        c.cycles += 4;
        lqsp(c, 16, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `void Replay_SnapToSnapshot(Replay* r)` (0x26e4b8): seek both replay
/// streams to the snapshot nearest the current frame.
pub fn replay_snap_to_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 16, 0x30);
        sqsp(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        sqsp(c, 18, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = 1;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x48c));
        c.g[A1][0] = Addr(p(r32(c, S0), 0x484)).lw(c);
        call::call_ptr(c, 0x0026_e448, 0x0026_e4e0)?;
        Addr(p(r32(c, S0), 0x488)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x30)).lw(c);
        Addr(p(r32(c, S0), 0x484)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
        loop {
            c.cycles += 4;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
            let empty = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !empty {
                c.cycles += 2;
                c.g[A1][0] = Addr(p(r32(c, S0), 0x484)).lw(c);
                call::call_ptr(c, 0x0026_d4d8, 0x0026_e508)?;
            }
            c.cycles += 2;
            if (c.g[S2][0] as i64) >= 0 {
                c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0026_e4f0);
                }
                continue;
            }
            c.cycles += 6;
            lqsp(c, 16, 0x30);
            lqsp(c, 17, 0x20);
            lqsp(c, 18, 0x10);
            frame.restore_ra(c, 0);
            let ra = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
            c.pc = ra;
            return Ok(());
        }
    });
}

/// `cCameraController* cCameraController_ctor(cCameraController* c, int a)`
/// (0x15cba0).
pub fn cam_controller_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0045_b938;
        sqsp(c, 17, 0x10);
        sqsp(c, 16, 0x20);
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x0045_b220;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        Addr(p(r32(c, S0), 0x14)).sw(c, c.g[V0][0]);
        c.g[A0][0] = 8;
        call::call_ptr(c, 0x0031_7d70, 0x0015_cbdc)?;
        c.g[A0][0] = c.g[V0][0];
        call::call_ptr(c, 0x0015_c930, 0x0015_cbe4)?;
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        Addr(p(r32(c, S0), 0x10)).sw(c, c.g[S1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(p(r32(c, S0), 0xc)).sw(c, 0);
        Addr(c.g[S0][0] as u32).sw(c, 0);
        Addr(p(r32(c, S0), 4)).sw(c, 0);
        lqsp(c, 16, 0x20);
        lqsp(c, 17, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `cFEStateUberTrick* cFEStateUberTrick_ctor(cFEStateUberTrick* s, int a1,
/// signed char idx)` (0x183a98).
pub fn c_fe_state_uber_trick_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        sqsp(c, 16, 0x20);
        sqsp(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = sx((c.g[A2][0] as u32) << 24);
        frame.save_ra(c, 0);
        let s1 = c.g[S1][0] as u32;
        c.g[S1][0] = sx(((s1 as i32) >> 24) as u32);
        callee::FE_STATE_CTOR.call(c, 0x0018_3ab8)?;
        c.g[V0][0] = 0x12;
        c.g[V1][0] = 0x0046_ccc0;
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[V0][0]);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        c.g[A1][0] = c.g[S1][0];
        Addr(p(r32(c, S0), 0x44)).sb(c, c.g[S1][0]);
        c.g[V0][0] = Addr(p(r32(c, S0), 0x10)).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        call::call_ptr(c, 0x001a_1cd0, 0x0018_3ae0)?;
        Addr(p(r32(c, S0), 0x15)).sb(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(p(r32(c, S0), 0x4c)).sw(c, 0);
        Addr(p(r32(c, S0), 0x50)).sw(c, 0);
        Addr(p(r32(c, S0), 0x48)).sw(c, 0);
        lqsp(c, 16, 0x20);
        lqsp(c, 17, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `void PS2GraphicsMan_FreeAllTextures(PS2GraphicsMan* m)` (0x37c830): free
/// the first 2000 non-null texture slots through virtual slot 50.
pub fn ps2_graphics_man_free_all_textures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 18, 0x10);
        sqsp(c, 16, 0x30);
        c.g[S2][0] = c.g[A0][0];
        sqsp(c, 17, 0x20);
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18f8));
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        c.g[A1][0] = 0;
        loop {
            c.cycles += 4;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            let empty = c.g[V0][0] == 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !empty {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(r32(c, S2), 0x10d8)).lw(c);
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x190)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x194)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = abi::addu(c.g[S2][0], c.g[A0][0]);
                call::call_ptr(c, target, 0x0037_c87c)?;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 2000) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S1][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0037_c858);
            }
        }
        c.cycles += 6;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `cFEStateRewardGalleryPosters* cFEStateRewardGalleryPosters_ctor(...)`
/// (0x1d1128): construct the reward-gallery posters state (43 poster slots).
pub fn c_fe_state_reward_gallery_posters_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 24);
        sqsp(c, 16, 0x10);
        let a2 = c.g[A2][0] as u32;
        c.g[A2][0] = sx(((a2 as i32) >> 24) as u32);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[T0][0] = 3;
        callee::REWARD_GALLERY_CTOR.call(c, 0x001d_1148)?;
        c.g[V1][0] = 0x0046_a248;
        Addr(p(r32(c, S0), 0x68)).sw(c, 0);
        Addr(p(r32(c, S0), 0xa24)).sw(c, 0);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xad0));
        c.g[V1][0] = 0x2a;
        loop {
            c.cycles += 7;
            Addr(c.g[V0][0] as u32).sw(c, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_1168);
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        lqsp(c, 16, 0x10);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `PlaybackMan* PlaybackMan_dtor(PlaybackMan* p, int flags)` (0x26c588):
/// destroy the owned big file and data, free the two buffers, and optionally
/// free the manager.
pub fn playback_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        sqsp(c, 16, 0x20);
        sqsp(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let no_file = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_file {
            c.cycles += 2;
            c.g[A1][0] = 3;
            call::call_ptr(c, 0x0031_68e8, 0x0026_c5b0)?;
        }
        c.cycles += 11;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x6c)).lw(c);
        call::call_ptr(c, 0x0031_7e50, 0x0026_c5b8)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x68)).lw(c);
        call::call_ptr(c, 0x0031_7e50, 0x0026_c5c0)?;
        c.g[A0][0] = c.g[S0][0];
        call::call_ptr(c, 0x0026_c898, 0x0026_c5c8)?;
        c.g[V0][0] = 0x0048_1898;
        c.g[V1][0] = c.g[S1][0] & 1;
        let no_free = c.g[V1][0] == 0;
        Addr(p(r32(c, S0), 0x14)).sw(c, c.g[V0][0]);
        if !no_free {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call::call_ptr(c, 0x0031_7e50, 0x0026_c5e4)?;
        }
        c.cycles += 5;
        lqsp(c, 16, 0x20);
        lqsp(c, 17, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `void cPS2MemCard_DoDeleteSave(cPS2MemCard* m)` (0x2c4410): build the save
/// file name and prompt string, then move to state 0x36 (delete). The
/// generated body's call into the 1200-line memcard state machine cannot be
/// bounded cheaply, so this port has no differential test.
#[allow(dead_code)]
pub fn c_ps2_mem_card_do_delete_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x78;
        sqsp(c, 16, 0x20);
        sqsp(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb9));
        Addr(p(r32(c, S0), 0x3c)).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = Addr(p(r32(c, S0), 0x1c)).lw(c);
        c.mult(0, V1, A1, V0);
        c.g[A1][0] = abi::addu(c.g[V1][0], c.g[S0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1b0));
        call::call_ptr(c, 0x0041_66f4, 0x002c_4448)?;
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = 0x004a_3948;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xf9));
        c.g[A3][0] = c.g[A2][0];
        call::call_ptr(c, 0x0041_7828, 0x002c_4460)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x36;
        call::call_ptr(c, 0x002c_48c0, 0x002c_446c)?;
        lqsp(c, 16, 0x20);
        lqsp(c, 17, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `int PATH_PlatStopTrackSound(int idx)` (0x2af838): stop the platform's
/// track sound through the sound system and mark the platform stopped.
pub fn path_plat_stop_track_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x004d_3e98;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        sqsp(c, 16, 0x10);
        c.g[A0][0] = abi::addu(c.g[A0][0], c.g[V0][0]);
        c.g[S0][0] = Addr(c.g[A0][0] as u32).lw(c);
        call::call_ptr(c, 0x003b_58a0, 0x002a_f85c)?;
        c.g[V0][0] = Addr(p(r32(c, S0), 0x64)).lb(c);
        let stopped = c.g[V0][0] == 0;
        if !stopped {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x6c)).lw(c);
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 4)).lw(c);
            call::call_ptr(c, 0x003b_89e0, 0x002a_f874)?;
        } else {
            c.cycles += 2;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
            call::call_ptr(c, 0x003b_78f8, 0x002a_f884)?;
        }
        c.cycles += 9;
        call::call_ptr(c, 0x003b_58d8, 0x002a_f88c)?;
        c.g[V1][0] = 1;
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        Addr(p(r32(c, S0), 0x65)).sb(c, c.g[V1][0]);
        c.g[V0][0] = 0;
        lqsp(c, 16, 0x10);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, DATA, GAME_GP};

    /// An empty guest function (returns immediately, 2 cycles).
    const EMPTY: u32 = 0x0038_dc60;
    /// `ScriptList_GetFieldB0`: returns `*(a0 + 0xb0)`; used to hand a vtable
    /// method a controlled non-zero result cheaply.
    const LOAD_B0: u32 = 0x0027_72b8;

    fn w32(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }
    fn w16(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }
    fn w8(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }
    /// The engine's size-class pool allocator, as in `oc106`: the "pools
    /// ready" flag plus eight free nodes per size class.
    fn pool(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w32(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool = DATA + 0x40_0000 + 0x4_0000 * i;
            let base = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base + 0x400 * n;
                w32(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w32(c, pool + head, base);
            w32(c, GAME_GP + slot, pool);
        }
    }
    /// Point every virtual slot a test may reach at [`EMPTY`].
    fn empty_slots(c: &mut Cpu, vt: u32, offs: &[u32]) {
        for off in offs {
            slot(c, vt, *off, 0, EMPTY);
        }
    }
    /// A zeroed BE-library singleton at `gp-0x1f04` (otherwise the UI/FE
    /// helpers fall into the lazy disc load at `0x3de768`).
    fn be_library(c: &mut Cpu) {
        w32(c, GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
    }
    /// `DLHeap_InitArena(&MEM_HEAP, arena, size)` over a test arena, for
    /// paths that allocate beyond the size-class pools.
    fn heap_init(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const RET: u64 = 0x0012_3450;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = RET;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, RET as u32, "DLHeap_InitArena did not return");
    }
    /// `{ short delta; short; void (*fn)(); }` at `vt + off`.
    fn slot(c: &mut Cpu, vt: u32, off: u32, delta: i16, func: u32) {
        w16(c, vt + off, delta as u16);
        w32(c, vt + off + 4, func);
    }

    const SEEDS: u32 = 200;

    #[test]
    fn script_engine_on_seq_start_matches() {
        check(ssx3_ee::s07::f_0027a6b0__generated, script_engine_on_seq_start, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x27a6b0, 0);
            let e = DATA + 0x1000;
            let listener = DATA + 0x2000;
            c.g[A0][0] = e as u64;
            c.g[A1][0] = listener as u64;
            let r_idx = if seed % 2 == 0 { 2 } else { (seed % 3) as u32 };
            w32(c, e + 0x550, r_idx);
            let table = DATA + 0x3000;
            w32(c, e + 0x54c, table);
            let list = table + r_idx * 0xcc;
            w32(c, list + 0xa4, 0); // ScriptList_IsPlayingScript -> false
            let o = DATA + 0x4000;
            w32(c, listener + 0xc, o);
            w32(c, o + 0x1c, r.next_u32());
            w32(c, o + 0x10, 0);
            let vt = DATA + 0x5000;
            w32(c, e + 0x2a8, vt);
            slot(c, vt, 0x50, 0, EMPTY);
        });
    }

    #[test]
    fn script_engine_on_seq_stopped_matches() {
        check(ssx3_ee::s07::f_0027a7f0__generated, script_engine_on_seq_stopped, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x27a7f0, 0);
            let e = DATA + 0x1000;
            let listener = DATA + 0x2000;
            c.g[A0][0] = e as u64;
            c.g[A1][0] = listener as u64;
            let r_idx = if seed % 2 == 0 { 2 } else { (seed % 3) as u32 };
            w32(c, e + 0x550, r_idx);
            let table = DATA + 0x3000;
            w32(c, e + 0x54c, table);
            w32(c, table + r_idx * 0xcc + 0xa4, 0);
            let o = DATA + 0x4000;
            w32(c, listener + 0xc, o);
            w32(c, o + 0x10, 0);
            w32(c, o + 0x1c, r.next_u32());
            let vt = DATA + 0x5000;
            w32(c, e + 0x2a8, vt);
            slot(c, vt, 0x58, 0, EMPTY);
        });
    }

    #[test]
    fn script_engine_remove_all_script_objs_matches() {
        check(
            ssx3_ee::s07::f_0027d050__generated,
            script_engine_remove_all_script_objs,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27d050, 0);
                let e = DATA + 0x1000;
                c.g[A0][0] = e as u64;
                let vt = DATA + 0x5000;
                w32(c, e + 0x2a8, vt);
                slot(c, vt, 0x18, 0, EMPTY);
                for i in 0..20u32 {
                    w32(c, e + 4 * i, if (seed + i) % 4 == 0 { 0 } else { r.next_u32() });
                }
            },
        );
    }

    #[test]
    fn script_engine_remove_binding_matches() {
        check(ssx3_ee::s07::f_0027d390__generated, script_engine_remove_binding, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x27d390, 0);
            let e = DATA + 0x1000;
            let msg = DATA + 0x2000;
            c.g[A0][0] = e as u64;
            c.g[A1][0] = msg as u64;
            let idx = (seed % 4) as u32;
            w32(c, msg + 4, idx);
            let s0 = e + idx * 0x14 + 0x50;
            w32(c, s0 + 0xc, if seed % 3 == 0 { 1 } else { r.next_u32() });
            w32(c, s0, if seed % 2 == 0 { DATA + 0x3000 } else { r.next_u32() });
            w32(c, msg + 8, 0); // ClearListener returns at once
            w32(c, DATA + 0x3010, r.next_u32());
        });
    }

    #[test]
    fn script_engine_collect_free_bindings_matches() {
        check(
            ssx3_ee::s07::f_0027d578__generated,
            script_engine_collect_free_bindings,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27d578, 0);
                let vars = DATA + 0x1000;
                let out = DATA + 0x4000;
                let end = DATA + 0x5000;
                c.g[A0][0] = vars as u64;
                c.g[A1][0] = end as u64;
                c.g[A2][0] = out as u64;
                c.g[A3][0] = (1 + seed % 4) as u64;
                for i in 0..30u32 {
                    let v = vars + 0x50 + i * 0x14;
                    w32(c, v, if (seed + i) % 5 == 0 { 0 } else { r.next_u32() });
                    w32(c, v + 0xc, if (seed + i) % 4 == 0 { 1 } else { 2 });
                    let link = match (seed + i) % 6 {
                        0 => 0x8000_0000u32,
                        1 => end,
                        2 => end + 0x14,
                        _ => r.next_u32() & 0x00ff_ffff,
                    };
                    w32(c, v + 0x10, link);
                }
            },
        );
    }

    #[test]
    fn menu_sys_item_handle_input_matches() {
        check(ssx3_ee::s09::f_002ca2b0__generated, menu_sys_item_handle_input, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2ca2b0, 0);
            let item = DATA + 0x1000;
            let menu = DATA + 0x2000;
            let input = DATA + 0x3000;
            c.g[A0][0] = item as u64;
            w32(c, item, menu);
            w32(c, menu + 0x124, input);
            let code = match seed % 9 {
                0 => 0,
                1 => 1,
                2 => 7,
                3 => 8,
                n => r.next_u32() % 10 + n as u32,
            };
            w32(c, input + 0x48, code);
        });
    }

    #[test]
    fn menu_sys_page_on_activate_matches() {
        check(ssx3_ee::s09::f_002cbf60__generated, menu_sys_page_on_activate, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2cbf60, 0);
            let page = DATA + 0x1000;
            c.g[A0][0] = page as u64;
            c.g[A1][0] = (seed % 3) as u64;
            let count = (seed % 4) as u32;
            let selected = if seed % 3 == 0 { count + 1 } else { (seed % 5) as u32 };
            w32(c, page, count);
            w32(c, page + 4, selected);
            // Item array lives at page + 0xc, indexed by the selection.
            let vt = DATA + 0x5000;
            slot(c, vt, 0x30, 0, LOAD_B0);
            slot(c, vt, 0x18, 0, LOAD_B0);
            for i in 0..8u32 {
                let item = DATA + 0x3000 + i * 0x100;
                w32(c, page + 0xc + i * 4, item);
                w32(c, item + 0x10, vt);
                w32(c, item + 0xb0, 1); // non-zero: no SelectNext walk
            }
            w32(c, DATA + 0x3000 + 0xb0, r.next_u32() | 1);
        });
    }

    #[test]
    fn menu_sys_submenu_item_handle_input_matches() {
        check(
            ssx3_ee::s09::f_002ccd20__generated,
            menu_sys_submenu_item_handle_input,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ccd20, 0);
                let item = DATA + 0x1000;
                let menu = DATA + 0x2000;
                let input = DATA + 0x3000;
                let page = DATA + 0x4000;
                c.g[A0][0] = item as u64;
                w32(c, item, menu);
                w32(c, menu + 0x124, input);
                w32(c, input + 0x48, match seed % 4 {
                    0 => 6,
                    1 => 0,
                    n => n as u32,
                });
                // disabled flag (`item+4 & 8`) and no base action
                w32(c, item + 4, if seed % 5 == 0 { 8 } else { 0 });
                w32(c, item + 0x18, page);
                // PushPage's item: `page+0x12c` vtable.
                let vt = DATA + 0x5000;
                slot(c, vt, 0x30, 0, EMPTY);
                w32(c, page + 0x12c, vt);
                let _ = r;
            },
        );
    }

    #[test]
    fn menu_sys_float_item_draw_matches() {
        check(ssx3_ee::s09::f_002ce100__generated, menu_sys_float_item_draw, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2ce100, 0);
            pool(c);
            heap_init(c);
            be_library(c);
            let item = DATA + 0x1000;
            let value = DATA + 0x2000;
            let page = DATA + 0x3000;
            c.g[A0][0] = item as u64;
            c.g[A1][0] = (seed % 5) as u64;
            w32(c, item + 0x18, value);
            w32(c, value, r.f32(1000.0));
            w32(c, item + 0x24, if seed % 4 == 0 { 0x3f80_0000 } else { r.f32(10.0) });
            w32(c, item + 0x28, r.f32(100.0));
            w32(c, item, page);
            w32(c, item + 0x14, r.next_u32());
            // DrawRow: item is not the selected item (different page selection).
            w32(c, page + 4, 0);
            w32(c, page + 0xc, item);
            let vt = DATA + 0x5000;
            empty_slots(c, vt, &[0x18, 0x1c]);
            w32(c, item + 0x10, vt);
        });
    }

    #[test]
    fn in_seq_start_sequence_matches() {
        check(ssx3_ee::s07::f_00272938__generated, in_seq_start_sequence, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x272938, 0);
            let seq = DATA + 0x1000;
            let src = DATA + 0x2000;
            c.g[A0][0] = seq as u64;
            c.g[A1][0] = src as u64;
            // Free node at seq+0x18, used list at seq+0x1c.
            let node = DATA + 0x3000;
            let used = DATA + 0x4000;
            w32(c, seq + 0x18, node);
            w32(c, seq + 0x1c, if seed % 2 == 0 { 0 } else { used });
            w32(c, node + 0x50, if seed % 3 == 0 { 0 } else { node + 0x100 });
            w32(c, node + 0x4, r.next_u32());
            w32(c, node + 0x54, r.next_u32());
            w32(c, used + 0x54, r.next_u32());
            // InSeq_InstanceSetup needs the source header to look benign.
            w16(c, src + 2, 0);
            w32(c, src + 4, 0);
            w32(c, src + 8, 0);
            w32(c, node, 0);
            w32(c, node + 8, 0);
            w32(c, node + 0x34, 0);
        });
    }

    // ------------------------------------------------------------------
    // Wave 36, batch 2.

    #[test]
    fn rider_gate_sub_update_wait_push_matches() {
        check(ssx3_ee::s00::f_0012c0c0__generated, rider_gate_sub_update_wait_push, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x12c0c0, 0);
            let g = DATA + 0x1000;
            let bits = DATA + 0x2000;
            c.g[A0][0] = g as u64;
            c.g[A1][0] = bits as u64;
            w32(c, GAME_GP.wrapping_sub(0x783c), 0.032_258_064f32.to_bits());
            let signed = (seed as i32 % 17) - 8;
            let raw = ((signed as u32) & 0x3f) << 12 | (r.next_u32() & 0xfff);
            w32(c, bits, raw);
            let model = DATA + 0x3000;
            let anim = DATA + 0x4000;
            w32(c, g + 0x14, model);
            w32(c, model + 0x784, anim);
            let vt = DATA + 0x5000;
            w32(c, anim + 0x58, vt);
            slot(c, vt, 0x20, 0, LOAD_B0);
            w32(c, anim + 0xb0, 0x207);
        });
    }

    #[test]
    fn irradiance_scale_matches() {
        check(ssx3_ee::s12::f_003897a0__generated, irradiance_scale, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3897a0, 0);
            let out = DATA + 0x1000;
            let src = DATA + 0x2000;
            c.g[A0][0] = out as u64;
            c.g[A1][0] = src as u64;
            c.f[12] = r.edge_f32();
            for i in 0..10u32 {
                for k in 0..4u32 {
                    w32(c, src + i * 0x10 + k * 4, r.next_u32());
                }
            }
        });
    }

    fn load_state_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        let _ = machine(c, seed, salt, 0);
        let s = DATA + 0x1000;
        let screen = DATA + 0x2000;
        c.g[A0][0] = s as u64;
        w32(c, s + 0x40, screen);
        // No objects in the screen: the transition returns without touching
        // the string formatting path.
        w32(c, screen + 0x3c, 0);
    }

    #[test]
    fn c_game_load_state_out_lodge_transition_in_matches() {
        check(
            ssx3_ee::s06::f_00245698__generated,
            c_game_load_state_out_lodge_transition_in,
            SEEDS,
            &|c, seed| load_state_fixture(c, seed, 0x245698),
        );
    }

    #[test]
    fn c_game_load_state_conquer_transition_in_matches() {
        check(
            ssx3_ee::s06::f_002458b8__generated,
            c_game_load_state_conquer_transition_in,
            SEEDS,
            &|c, seed| load_state_fixture(c, seed, 0x2458b8),
        );
    }

    #[test]
    fn c_fe_load_hint_state_transition_in_matches() {
        check(
            ssx3_ee::s06::f_00245ae0__generated,
            c_fe_load_hint_state_transition_in,
            SEEDS,
            &|c, seed| load_state_fixture(c, seed, 0x245ae0),
        );
    }

    #[test]
    fn c_fe_load_state_transition_in_matches() {
        check(
            ssx3_ee::s06::f_00245c60__generated,
            c_fe_load_state_transition_in,
            SEEDS,
            &|c, seed| load_state_fixture(c, seed, 0x245c60),
        );
    }

    #[test]
    fn c_fe_load_state_in_lodge_transition_in_matches() {
        check(
            ssx3_ee::s06::f_00245dc0__generated,
            c_fe_load_state_in_lodge_transition_in,
            SEEDS,
            &|c, seed| load_state_fixture(c, seed, 0x245dc0),
        );
    }

    #[test]
    fn multi_particle_man_create_slot_matches() {
        check(
            ssx3_ee::s11::f_00357a78__generated,
            multi_particle_man_create_slot,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x357a78, 0);
                pool(c);
                let man = DATA + 0x1000;
                c.g[A0][0] = man as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = 0;
                // The ctor's first field (its 2nd argument) is an element
                // count: keep it tiny or the emitter free-list walk runs away.
                c.g[A3][0] = (seed % 2) as u64;
                for i in 0..8u32 {
                    w32(c, man + 4 * i, if (seed + i) % 3 == 0 { 0 } else { DATA + 0x3000 + i * 0x100 });
                }
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn path_data_free_matches() {
        check(ssx3_ee::s07::f_0026af00__generated, path_data_free, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x26af00, 0);
            let d = DATA + 0x1000;
            c.g[A0][0] = d as u64;
            // No owned buffers: no frees (a free of an arbitrary pointer walks
            // the heap headers and can run away).
            w32(c, d + 0xc, 0);
            w32(c, d + 0x14, 0);
            w32(c, d + 0x48, if seed % 2 == 0 { 0 } else { DATA + 0x3000 });
            w32(c, d + 0x4c, 0);
            w32(c, DATA + 0x3000, 0);
        });
    }

    #[test]
    fn rider_air_state_play_land_anim_matches() {
        check(ssx3_ee::s01::f_0013a8f8__generated, rider_air_state_play_land_anim, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x13a8f8, 0);
            let rider = DATA + 0x1000;
            let model = DATA + 0x3000;
            let anim = DATA + 0x4000;
            c.g[A0][0] = rider as u64;
            w32(c, GAME_GP.wrapping_sub(0x71a0), (-2222.222_2f32).to_bits());
            c.f[12] = if seed % 2 == 0 {
                (-2222.3f32 + (seed % 5) as f32).to_bits()
            } else {
                r.f32(3000.0)
            };
            w32(c, rider + 4, model);
            w32(c, model + 0x784, anim);
            let vt = DATA + 0x5000;
            w32(c, anim + 0x58, vt);
            slot(c, vt, 0x20, 0, LOAD_B0);
            w32(c, anim + 0xb0, 0x207);
            w32(c, model + 0x2dc, r.next_u32());
        });
    }

    #[test]
    fn game_state_reset_update_matches() {
        check(ssx3_ee::s06::f_002357f8__generated, game_state_reset_update, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x2357f8, 0);
            let s = DATA + 0x1000;
            let engine = DATA + 0x2000;
            let world = DATA + 0x3000;
            let race = DATA + 0x4000;
            let arr = DATA + 0x5000;
            let table = DATA + 0x6000;
            c.g[A0][0] = s as u64;
            w32(c, s + 0x1c, if seed % 2 == 0 { 0 } else { 1 });
            w32(c, s, 0); // GotoNext takes its early-out
            w32(c, GAME_GP.wrapping_sub(0x84c), engine);
            w32(c, GAME_GP.wrapping_sub(0x848), world);
            w32(c, engine + 0x550, 0);
            w32(c, engine + 0x54c, table);
            w32(c, table + 0xcc + 0xa4, 0); // GetPlayStatus short path
            w32(c, world + 0x84, race);
            w32(c, race + 0x78, arr);
            w32(c, arr + 0x1a4, 0);
        });
    }

    #[test]
    fn rider_stop_transition_matches() {
        check(ssx3_ee::s00::f_00125038__generated, rider_stop_transition, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x125038, 0);
            let rider = DATA + 0x1000;
            let world = DATA + 0x3000;
            let ail = DATA + 0x4000;
            let cam = DATA + 0x5000;
            c.g[A0][0] = rider as u64;
            w32(c, GAME_GP.wrapping_sub(0x848), world);
            w32(c, world + 0x84, ail);
            w32(c, ail + 0x84, if seed % 5 == 0 { 0 } else { cam });
            let view = if seed % 3 == 0 { 0x8000_0000u32 } else { (seed % 4) as u32 };
            w32(c, rider + 0x870, view);
            // func_00230698(ail, view) -> *(ail + view*4 + 0x5c)
            for i in 0..4u32 {
                w32(c, ail + 0x5c + 4 * i, DATA + 0x6000 + i * 0x100);
            }
            for i in 0..4u32 {
                let c2 = DATA + 0x6000 + i * 0x100;
                w32(c, c2 + 0x78, if seed % 2 == 0 { 1 } else { 0 });
                w32(c, c2 + 0x44, 0);
            }
            let _ = &mut r;
        });
    }

    #[test]
    fn wscript_man_unload_section_matches() {
        check(ssx3_ee::s10::f_00308f70__generated, wscript_man_unload_section, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x308f70, 0);
            let m = DATA + 0x1000;
            c.g[A0][0] = m as u64;
            c.g[A1][0] = (seed % 4) as u64;
            w32(c, m + 0x2b4, 0);
            w32(c, m + 0x2b8, 0);
            w32(c, m + 0x2bc, 0);
            for i in 0..4u32 {
                w32(c, m + 0x3cc + 4 * i, 0);
            }
        });
    }

    #[test]
    fn c_fe_state_options_init_matches() {
        check(ssx3_ee::s02::f_00188800__generated, c_fe_state_options_init, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x188800, 0);
            pool(c);
            let s = DATA + 0x1000;
            let engine = DATA + 0x2000;
            c.g[A0][0] = s as u64;
            w32(c, s + 0x10, engine);
            w32(c, engine + 4, 0);
            w32(c, engine + 0x38, 0);
            w32(c, engine + 0x3c, 0);
            let _ = seed;
        });
    }

    #[test]
    fn c_fe_popup_confirm_v27_matches() {
        check(ssx3_ee::s04::f_001d9ca8__generated, c_fe_popup_confirm_v27, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x1d9ca8, 0);
            let s = DATA + 0x1000;
            let src = DATA + 0x2000;
            c.g[A0][0] = s as u64;
            c.g[A1][0] = if seed % 4 == 0 { 0 } else { src as u64 };
            // A short UTF-16 string.
            w16(c, src, if seed % 3 == 0 { 0 } else { 0x41 });
            w16(c, src + 2, 0x42);
            w16(c, src + 4, 0);
            w32(c, s + 0x40, if seed % 2 == 0 { 0 } else { 1 });
            w32(c, s + 0x2bc, 0); // ResetLayout path avoided
        });
    }

    #[test]
    fn script_engine_resume_process_matches() {
        check(ssx3_ee::s07::f_00282720__generated, script_engine_resume_process, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x282720, 0);
            let e = DATA + 0x1000;
            let idx = (seed % 4) as u32;
            c.g[A0][0] = e as u64;
            c.g[A1][0] = idx as u64;
            let slotp = e + idx * 0x24 + 0x2c0;
            let inst = DATA + 0x3000 + idx * 0x100;
            w32(c, slotp, inst);
            w32(c, slotp + 4, if seed % 3 == 0 { 5 } else { (seed % 7) as u32 });
            w32(c, inst + 0x20, if seed % 2 == 0 { 0 } else { 3 + (seed % 4) });
            w32(c, inst + 0x2c, if seed % 5 == 0 { 1 } else { 0 });
            let _ = &mut r;
        });
    }

    #[test]
    fn benew_player_int_clear_shared_char_record_matches() {
        check(
            ssx3_ee::s01::f_00147898__generated,
            benew_player_int_clear_shared_char_record,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x147898, 0);
                let idx = (seed % 4) as u32;
                c.g[A1][0] = idx as u64;
                for i in 0..4u32 {
                    w32(c, 0x0053_55b0 + 4 * i, if i == idx { (seed % 4) as u32 } else { 0 });
                }
                for slot in 0..4u32 {
                    let base = 0x0053_4fe0 + slot * 0x1c;
                    w32(c, base + 0x10, r.next_u32());
                    w8(c, base + 0x11, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn game_mode_man_reset_mode_matches() {
        check(ssx3_ee::s06::f_002382d8__generated, game_mode_man_reset_mode, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2382d8, 0);
            let gm = DATA + 0x1000;
            c.g[A0][0] = gm as u64;
            let mode = (seed % 4) as u32;
            w32(c, gm + 4, mode);
            let obj = DATA + 0x3000 + mode * 0x100;
            let vt = DATA + 0x5000;
            w32(c, 0x0053_6668 + mode * 4, obj);
            w32(c, obj, vt);
            slot(c, vt, 0x10, 0, EMPTY);
            for i in 0..6u32 {
                w32(c, 0x0053_66d0 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn world_trigger_manager_stop_all_matches() {
        check(ssx3_ee::s08::f_002b5e68__generated, world_trigger_manager_stop_all, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2b5e68, 0);
            let m = DATA + 0x1000;
            let entries = DATA + 0x2000;
            c.g[A0][0] = m as u64;
            w32(c, m, entries);
            for i in 0..40u32 {
                let e = entries + i * 0x70;
                let active = if (seed as u32 + i) % 5 == 0 { 1 } else { 0 };
                w32(c, e, active);
                w32(c, e + 0x30, r.next_u32());
                w32(c, e + 0x38, 0); // WorldTriggerInstance_Stop early-out
            }
        });
    }

    #[test]
    fn data_instance_man_remove_all_matches() {
        check(ssx3_ee::s07::f_00275b08__generated, data_instance_man_remove_all, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x275b08, 0);
            let m = DATA + 0x1000;
            let entries = DATA + 0x2000;
            let count = (seed % 5) as u32;
            c.g[A0][0] = m as u64;
            w32(c, m, count);
            w32(c, m + 4, entries);
            for i in 0..4u32 {
                w32(c, entries + i * 0x18 + 0x10, 0);
                w32(c, entries + i * 0x18, r.next_u32());
            }
        });
    }

    #[test]
    fn input_preproc_skip_line_matches() {
        check(ssx3_ee::s10::f_00326678__generated, input_preproc_skip_line, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x326678, 0);
            let p = DATA + 0x1000;
            let s = DATA + 0x2000;
            let buf = DATA + 0x3000;
            c.g[A0][0] = p as u64;
            c.g[A1][0] = s as u64;
            // A 8-byte stream with a newline at position seed % 8.
            w32(c, s, 8);
            w32(c, s + 4, 0);
            w32(c, s + 8, buf);
            w32(c, s + 0xc, 0);
            let nl = (seed % 8) as u32;
            w8(c, p + 0xc, b'x');
            for i in 0..8u32 {
                w8(c, buf + i, if i == nl { 0x0a } else { b'a' });
            }
        });
    }

    #[test]
    fn pad_device_stop_actuators_matches() {
        check(ssx3_ee::s10::f_00326cf0__generated, pad_device_stop_actuators, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x326cf0, 0);
            let d = DATA + 0x1000;
            let vt = DATA + 0x5000;
            c.g[A0][0] = d as u64;
            c.pc = 0x0032_6cf0;
            w32(c, d, vt);
            slot(c, vt, 0x18, 0, LOAD_B0);
            slot(c, vt, 0x20, 0, EMPTY);
            w32(c, d + 0xb0, (seed % 4) as u32);
            let _ = &mut r;
        });
    }

    #[test]
    fn ps2_ai_visual_effects_man_ctor_matches() {
        check(
            ssx3_ee::s09::f_002f03c8__generated,
            ps2_ai_visual_effects_man_ctor,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2f03c8, 0);
                pool(c);
                let m = DATA + 0x1000;
                c.g[A0][0] = m as u64;
                let _ = seed;
            },
        );
    }

    #[test]
    fn light_man_v11_matches() {
        check(ssx3_ee::s12::f_0038df98__generated, light_man_v11, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x38df98, 0);
            let obj = DATA + 0x1000;
            let vt = DATA + 0x5000;
            c.g[A0][0] = obj as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = r.next_u32() as u64;
            c.g[T0][0] = r.next_u32() as u64;
            c.f[12] = r.edge_f32();
            w32(c, obj, vt);
            slot(c, vt, 0x10, 0, EMPTY);
        });
    }

    #[test]
    fn world_memman_dtor_matches() {
        check(ssx3_ee::s13::f_003a7aa8__generated, world_memman_dtor, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x3a7aa8, 0);
            let m = DATA + 0x1000;
            c.g[A0][0] = if seed % 7 == 0 { 0 } else { m as u64 };
            c.g[A1][0] = (seed % 2) as u64;
            // The two block pools: empty (no free path).
            for p in [m + 0x28, m + 0x14] {
                w32(c, p + 4, 0);
                w32(c, p + 0xc, 0);
            }
        });
    }

    #[test]
    fn c_world_cache_flag_section_objects_matches() {
        check(
            ssx3_ee::s13::f_003a8f10__generated,
            c_world_cache_flag_section_objects,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a8f10, 0);
                let cache = DATA + 0x1000;
                let idx = (seed % 3) as u32;
                let world = DATA + 0x2000;
                let table = DATA + 0x3000;
                let lists = DATA + 0x4000;
                let list = DATA + 0x5000;
                let entries = DATA + 0x6000;
                c.g[A0][0] = cache as u64;
                c.g[A1][0] = idx as u64;
                w32(c, cache, world);
                w32(c, cache + 8, lists);
                w32(c, world + 0x14, table);
                let n = (seed % 4) as u16;
                w16(c, table + idx * 0x58 + 0x22, n);
                w32(c, lists + idx * 4, list);
                w32(c, list + 0xc, entries);
                for i in 0..4u32 {
                    let target = DATA + 0x7000 + i * 4;
                    if i < n as u32 {
                        w32(c, entries + i * 4, ((target >> 2) << 8) | (r.next_u32() & 0xff));
                    } else {
                        w32(c, entries + i * 4, 0);
                    }
                    w16(c, target + 0xa, r.next_u32() as u16);
                }
            },
        );
    }

    #[test]
    fn crowd_man2d_remove_all_matches() {
        check(ssx3_ee::s05::f_00229398__generated, crowd_man2d_remove_all, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x229398, 0);
            let m = DATA + 0x1000;
            c.g[A0][0] = m as u64;
            for i in 0..128u32 {
                w32(
                    c,
                    m + 8 + 4 * i,
                    if (seed as u32 + i) % 3 == 0 { 0xffff_ffff } else { r.next_u32() },
                );
            }
        });
    }

    #[test]
    fn c_world_cache_streamer_activate_section_matches() {
        check(
            ssx3_ee::s13::f_003a99d8__generated,
            c_world_cache_streamer_activate_section,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a99d8, 0);
                let cache = DATA + 0x1000;
                let idx = (seed % 4) as u32;
                c.g[A0][0] = cache as u64;
                c.g[A1][0] = idx as u64;
                let sections = if seed % 5 == 0 { 0 } else { DATA + 0x2000 };
                w32(c, cache + 4, sections);
                w32(c, sections + 8, if seed % 4 == 0 { 2 } else { 8 });
                for i in 0..8u32 {
                    w32(
                        c,
                        cache + 0x14 + i * 8,
                        if i == idx { (seed % 8) as u32 } else { r.next_u32() % 8 },
                    );
                }
                // The flagging path needs a world whose section has no objects.
                let world = DATA + 0x3000;
                let table = DATA + 0x4000;
                let lists = DATA + 0x5000;
                w32(c, cache, world);
                w32(c, world + 0x14, table);
                w16(c, table + idx * 0x58 + 0x22, 0);
                w32(c, world + 8, lists);
            },
        );
    }

    #[test]
    fn c_ui_text_scroll_ctor_matches() {
        check(ssx3_ee::s13::f_003a1148__generated, c_ui_text_scroll_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3a1148, 0);
            let s = DATA + 0x1000;
            c.g[A0][0] = s as u64;
            w32(c, s + 0x14, r.next_u32());
            w8(c, s + 0x74, r.next_u32() as u8);
        });
    }

    #[test]
    fn ui_vector_ctor_matches() {
        check(ssx3_ee::s13::f_003a3280__generated, ui_vector_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3a3280, 0);
            let v = DATA + 0x1000;
            c.g[A0][0] = v as u64;
            w32(c, v + 0x14, r.next_u32());
            w32(c, v + 0x74, r.next_u32());
        });
    }

    #[test]
    fn multi_particle_remove_instance_matches() {
        check(ssx3_ee::s11::f_003581f0__generated, multi_particle_remove_instance, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3581f0, 0);
            let p = DATA + 0x1000;
            let entries = DATA + 0x2000;
            let count = (seed % 5) as u32;
            c.g[A0][0] = p as u64;
            w32(c, p + 4, count);
            w32(c, p + 0xc, entries);
            for i in 0..5u32 {
                w32(c, entries + 4 * i, if i < count { r.next_u32() } else { 0 });
            }
            // Half of the seeds look for a present key.
            c.g[A1][0] = if seed % 2 == 0 && count > 0 {
                c.bus.read32(entries + 4 * ((seed % count) as u32)) as u64
            } else {
                r.next_u32() as u64
            };
        });
    }

    #[test]
    fn avalanche_set_history_data_matches() {
        check(ssx3_ee::s09::f_002d9fb8__generated, avalanche_set_history_data, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x2d9fb8, 0);
            let data = DATA + 0x1000;
            c.g[A0][0] = data as u64;
            w32(c, data, if seed % 4 == 0 { 0x02be_ef00 } else { 0x1234_5678 });
            w32(c, GAME_GP + 0x9c0, 0); // no live history blocks
            // Never the (freeing) "old data with buffers" combination.
            w32(c, GAME_GP + 0x9c4, if seed % 2 == 0 { 0 } else { DATA + 0x3000 });
            w32(c, GAME_GP + 0x9c8, 0);
        });
    }

    #[test]
    fn replay_snap_to_snapshot_matches() {
        check(ssx3_ee::s07::f_0026e4b8__generated, replay_snap_to_snapshot, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x26e4b8, 0);
            let replay = DATA + 0x1000;
            c.g[A0][0] = replay as u64;
            w32(c, replay + 0x484, r.next_u32());
            // Empty snapshot list and streams.
            w32(c, replay + 0x3b0, 0);
            w32(c, replay + 0x48c, 0);
            w32(c, replay + 0x490, 0);
        });
    }

    #[test]
    fn cam_controller_ctor_matches() {
        check(ssx3_ee::s01::f_0015cba0__generated, cam_controller_ctor, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x15cba0, 0);
            pool(c);
            let cam = DATA + 0x1000;
            c.g[A0][0] = cam as u64;
            c.g[A1][0] = (seed % 5) as u64;
        });
    }

    #[test]
    fn c_fe_state_uber_trick_ctor_matches() {
        check(ssx3_ee::s02::f_00183a98__generated, c_fe_state_uber_trick_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x183a98, 0);
            let s = DATA + 0x1000;
            let engine = DATA + 0x2000;
            let names = DATA + 0x3000;
            c.g[A0][0] = s as u64;
            c.g[A1][0] = engine as u64;
            c.g[A2][0] = (seed % 8) as u64;
            w32(c, s + 0x10, engine);
            w32(c, engine, names);
            for i in 0..8u32 {
                w8(c, names + i + 8, r.next_u32() as u8);
            }
        });
    }

    #[test]
    fn ps2_graphics_man_free_all_textures_matches() {
        check(
            ssx3_ee::s12::f_0037c830__generated,
            ps2_graphics_man_free_all_textures,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x37c830, 0);
                let m = DATA + 0x1000;
                let vt = DATA + 0x5000;
                c.g[A0][0] = m as u64;
                w32(c, m + 0x10d8, vt);
                slot(c, vt, 0x190, 0, EMPTY);
                for i in 0..2000u32 {
                    let v = if (seed as u32 + i) % 37 == 0 { r.next_u32() | 1 } else { 0 };
                    w32(c, m + 0x18f8 + 4 * i, v);
                }
            },
        );
    }

    #[test]
    fn c_fe_state_reward_gallery_posters_ctor_matches() {
        check(
            ssx3_ee::s04::f_001d1128__generated,
            c_fe_state_reward_gallery_posters_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d1128, 0);
                pool(c);
                be_library(c);
                let s = DATA + 0x1000;
                let engine = DATA + 0x2000;
                let names = DATA + 0x3000;
                c.g[A0][0] = s as u64;
                c.g[A1][0] = engine as u64;
                c.g[A2][0] = (seed % 4) as u64;
                // Its 4th argument selects the "load the rider bio from disc"
                // branch: keep it null.
                c.g[A3][0] = 0;
                w32(c, s + 0x10, engine);
                w32(c, engine, names);
                for i in 0..8u32 {
                    w8(c, names + i + 8, r.next_u32() as u8);
                }
            },
        );
    }
}
