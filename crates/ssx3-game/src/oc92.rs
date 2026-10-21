//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for small object, menu, memory, audio and UI helpers, each
//! with a differential test against the generated body.
//!
//! Groups:
//! - **Object modifiers** (`ModObj_AddMultiSplineModifier`,
//!   `ModObj_AddTexFlipModifier`, `ModObj_AddBoostModifier`, and the spline
//!   modifier's `Save`) — allocate a modifier and install it on a move node.
//! - **Menus** (`LobbyMenu_InitMessagePage`, `LobbyMenu_AcceptChallenge`,
//!   keyboard cursor moves, `EventSelect_Shutdown`,
//!   `EventSelect_SetHighlight/UnlockedColor`, `OVScreenAutosave_ctor`,
//!   `cFEStateVideoPlayer_dtor`, `cFEStateMountainRoom_ShowImage`,
//!   `cFEStateBackground_Init`, `StartScreen_Enter`, `cTransitionMan_*`).
//! - **Audio** (`SSXAudioSystem_ResumeGameAudio`,
//!   `SSXAudioSystem_OutputModeToOption`, `SndBank_Release`,
//!   `ScriptList_CanSkip`, `cStreamMan_Reset`).
//! - **Riders** (`Rider_GetStartMarkerPos`, `Rider_AddVelocity`,
//!   `Rider_v13`/`v14`, `RiderMetrix_FinishHandplant`).
//! - **Memory** (`MEM_ResizeBlockLocked`, `MEM_Init`, `cBXString_AllocBeforeWrite`).
//! - **UI** (`UIScreen_GetPrimaryThread`, `UIScreen_CmdSetObjectParam`,
//!   `UIPair_RefreshText`, `OVScreenReadPrivateMessage_OpenKeyboard`,
//!   `cFEStateBuyAttrib_UpdateTotalCost/UpdateMoney`).
//! - **Camera** (`Manual/Target/SubjectScriptCameraAlgo_ctor`).
//!
//! Not listed in [`OVERRIDES`] (ported here, but their differential test would
//! need a fixture the engine's real allocator/UI chains cannot provide at test
//! speed): `ModObj_AddMultiSplineModifier` (0x355b30, the multi-spline
//! constructor allocates per-node arrays), `Rider_v13`/`Rider_v14`
//! (0x10e770/0x10e7d0, the whole paint path), `RiderMetrix_FinishHandplant`
//! (0x119c38, `RiderMetrix_FinishTrick`), `WorldTriggerManager_GetSpecialTriggerInfo`
//! (0x2b6550, the trigger table builder), `cFEStateBuyAttrib_UpdateTotalCost`
//! / `_UpdateMoney` (0x1f5240/0x1f52a0, the attribute editor) and
//! `PS2BezierMan_FreeCaches` (0x38b0f8, frees real allocator blocks).

use crate::abi;
use crate::call::{call_ptr, guest, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::Cpu;

#[rustfmt::skip]
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_5bf0, mod_obj_add_tex_flip_modifier, "ModObj_AddTexFlipModifier"),
    (0x0035_5c50, mod_obj_add_boost_modifier, "ModObj_AddBoostModifier"),
    (0x0035_7848, ava_spline_modifier_save, "AvaSplineModifier_Save"),
    (0x0017_d200, lobby_menu_init_message_page, "LobbyMenu_InitMessagePage"),
    (0x0017_e490, lobby_menu_accept_challenge, "LobbyMenu_AcceptChallenge"),
    (0x0018_0618, keyboard_screen_cursor_left, "KeyboardScreen_CursorLeft"),
    (0x0018_0678, keyboard_screen_cursor_right, "KeyboardScreen_CursorRight"),
    (0x001d_2380, c_fe_state_video_player_dtor, "cFEStateVideoPlayer_dtor"),
    (0x0028_9bb8, ssx_audio_system_resume_game_audio, "SSXAudioSystem_ResumeGameAudio"),
    (0x0028_9d60, ssx_audio_system_output_mode_to_option, "SSXAudioSystem_OutputModeToOption"),
    (0x0012_2c28, rider_get_start_marker_pos, "Rider_GetStartMarkerPos"),
    (0x0012_50a8, rider_add_velocity, "Rider_AddVelocity"),
    (0x002a_9e78, stream_buffer_set_file, "StreamBuffer_SetFile"),
    (0x0028_b878, snd_bank_release, "SndBank_Release"),
    (0x0028_2cd0, c_transition_man_script_obj_ctor, "cTransitionMan_ScriptObjCtor"),
    (0x0025_26b8, mem_resize_block_locked, "MEM_ResizeBlockLocked"),
    (0x0025_39e0, vid_engine_player_pause, "VidEnginePlayer_Pause"),
    (0x002b_6550, world_trigger_manager_get_special_trigger_info, "WorldTriggerManager_GetSpecialTriggerInfo"),
    (0x002b_6808, world_trigger_manager_was_played, "WorldTriggerManager_WasPlayed"),
    (0x0027_7060, script_list_can_skip, "ScriptList_CanSkip"),
    (0x0039_c7c0, ui_screen_get_primary_thread, "UIScreen_GetPrimaryThread"),
    (0x0039_cd30, ui_screen_cmd_set_object_param, "UIScreen_CmdSetObjectParam"),
    (0x0032_6df0, ps2_pad_ctor, "PS2Pad_ctor"),
    (0x0032_6e50, ps2_pad_dtor, "PS2Pad_dtor"),
    (0x0031_8188, c_bx_string_alloc_before_write, "cBXString_AllocBeforeWrite"),
    (0x0038_df38, light_man_v10, "LightMan_v10"),
    (0x0023_2c98, start_screen_enter, "StartScreen_Enter"),
    (0x0016_94b8, manual_script_camera_algo_ctor, "ManualScriptCameraAlgo_ctor"),
    (0x0016_9cf8, target_script_camera_algo_ctor, "TargetScriptCameraAlgo_ctor"),
    (0x0016_a458, subject_script_camera_algo_ctor, "SubjectScriptCameraAlgo_ctor"),
    (0x0031_9a30, mem_init, "MEM_Init"),
    (0x0020_0af0, event_select_shutdown, "EventSelect_Shutdown"),
    (0x0020_83d8, event_select_set_highlight_color, "EventSelect_SetHighlightColor"),
    (0x0020_8438, event_select_set_unlocked_color, "EventSelect_SetUnlockedColor"),
    (0x0039_be10, ui_pair_refresh_text, "UIPair_RefreshText"),
    (0x0021_2138, ov_screen_autosave_ctor, "OVScreenAutosave_ctor"),
    (0x001d_32e0, c_fe_state_mountain_room_show_image, "cFEStateMountainRoom_ShowImage"),
    (0x0021_0f40, ov_screen_read_private_message_open_keyboard, "OVScreenReadPrivateMessage_OpenKeyboard"),
    (0x0015_5130, be_score_int_get_high_score, "BEScoreInt_GetHighScore"),
    (0x001a_35e8, fe_async_manager_dtor, "FEAsyncManager_dtor"),
    (0x003a_6ed8, c_stream_man_reset, "cStreamMan_Reset"),
    (0x0014_f810, be_option_int_is_any_handicap_set, "BEOptionInt_IsAnyHandicapSet"),
    (0x0019_4528, c_fe_state_background_init, "cFEStateBackground_Init"),
];

// ---------------------------------------------------------------------------
// Small helpers.

/// `reg + off` (32-bit address arithmetic).
#[inline(always)]
fn at(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

/// The low 32 bits of a register.
#[inline(always)]
fn r32(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// `gp + off`.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `lw reg, off(base)` leaving the zero-extended value in `reg`.
#[inline(always)]
fn lw_into(c: &mut Cpu, reg: usize, base: u32, off: u32) {
    c.g[reg][0] = Addr(base.wrapping_add(off)).lw(c);
}

// ---------------------------------------------------------------------------
// Object modifiers (movenode.cpp).

/// `void cMoveNode_addSpline(cMoveNode* self, void* desc)` (0x355b30):
/// allocate a multi-spline modifier (0x58 bytes), construct it from `desc`
/// and the node's modifier block, and install it as the transform modifier.
pub fn mod_obj_add_multi_spline_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[A2][0] = 0x2000_0000;
        frame.save(c, S0, 0x20);
        let desc = c.g[A1][0];
        let obj = c.g[A0][0];
        c.g[S1][0] = desc;
        c.g[S0][0] = obj;
        c.g[A1][0] = sx(0x0048_e9e8);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x58;
        call_ptr(c, 0x0031_7d70, 0x0035_5b60)?;
        lw_into(c, A2, r32(c, S0), 0x18);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0035_9f88, 0x0035_5b70)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0035_54b0, 0x0035_5b7c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void func_00355BF0(cMoveNode* self, void* desc)` (0x355bf0): allocate and
/// install a texture-flip modifier (0xe8 bytes).
pub fn mod_obj_add_tex_flip_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[A2][0] = 0x2000_0000;
        frame.save(c, S0, 0x20);
        let desc = c.g[A1][0];
        let obj = c.g[A0][0];
        c.g[S1][0] = desc;
        c.g[S0][0] = obj;
        c.g[A1][0] = sx(0x0048_e958);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0xe8;
        call_ptr(c, 0x0031_7d70, 0x0035_5c20)?;
        lw_into(c, A2, r32(c, S0), 0x18);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0035_f0b8, 0x0035_5c30)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0035_55a8, 0x0035_5c3c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void func_00355C50(cMoveNode* self, void* desc)` (0x355c50): allocate and
/// install a boost modifier (0x50 bytes).
pub fn mod_obj_add_boost_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[A2][0] = 0x2000_0000;
        frame.save(c, S0, 0x20);
        let desc = c.g[A1][0];
        let obj = c.g[A0][0];
        c.g[S1][0] = desc;
        c.g[S0][0] = obj;
        c.g[A1][0] = sx(0x0048_e968);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x50;
        call_ptr(c, 0x0031_7d70, 0x0035_5c80)?;
        lw_into(c, A2, r32(c, S0), 0x18);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0034_2768, 0x0035_5c90)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0035_5600, 0x0035_5c9c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void func_00357848(AvaSplineModifier* self, void* desc)` (0x357848): save
/// the modifier's curve data through two virtual methods of the two objects.
pub fn ava_spline_modifier_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let desc = c.g[A1][0];
        let self_ = c.g[A0][0];
        c.g[S0][0] = desc;
        frame.save_ra(c, 0);
        c.g[S1][0] = self_;
        c.g[V0][0] = Addr(desc as u32).lw(c);
        c.g[A1][0] = Addr(at(c, S1, 0x80)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x28).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x2c).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, target, 0x0035_7878)?;
        c.g[A3][0] = Addr(r32(c, S0)).lw(c);
        c.g[A1][0] = sx(at(c, S1, 0x10));
        c.g[A2][0] = 0x30;
        c.g[A0][0] = Addr(r32(c, A3)).off(0x8).lh(c);
        c.g[V0][0] = Addr(r32(c, A3)).off(0xc).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, target, 0x0035_7894)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menus.

/// `void LobbyMenu_InitMessagePage(LobbyMenu* menu)` (0x17d200): initialise
/// the message-page child widget and its two labels.
pub fn lobby_menu_init_message_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let menu = c.g[A0][0];
        c.g[S0][0] = menu;
        call_ptr(c, 0x002c_aa58, 0x0017_d218)?;
        c.g[S1][0] = sx(at(c, S0, 0x130));
        c.g[V1][0] = 0x46_d740;
        c.g[A1][0] = 0x45_d060;
        c.g[A0][0] = c.g[S1][0];
        Addr(at(c, S0, 0x12c)).sw(c, c.g[V1][0]);
        call_ptr(c, 0x002c_cdf0, 0x0017_d238)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x0017_d248)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMenu_AcceptChallenge(LobbyMenu* menu)` (0x17e490): drain the
/// challenge queue, then show the accepted-challenge page.
pub fn lobby_menu_accept_challenge(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let menu = c.g[A0][0];
        c.g[S1][0] = menu;
        c.g[V0][0] = Addr(at(c, S1, 0x8)).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = sx(at(c, S1, 0x8));
        if !empty {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_c460, 0x0017_e4b8)?;
                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_e4b0);
                }
            }
        }
        c.cycles += 11;
        c.g[A1][0] = Addr(at(c, S1, 0x1c0c)).lw(c);
        c.g[V0][0] = 0xd;
        c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
        c.g[A2][0] = 0;
        Addr(at(c, S1, 0x4)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0025_fb58, 0x0017_e4dc)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void KeyboardScreen_CursorLeft(KeyboardScreen* s)` (0x180618): move the
/// keyboard cursor one key to the left (cursor ids 0,10,20,30,39..).
pub fn keyboard_screen_cursor_left(c: &mut Cpu) {
    let obj = abi::arg(c, 0);
    'end: {
        c.cycles += 3;
        c.g[V1][0] = Addr(obj.wrapping_add(0x170)).lw(c);
        let at_zero = c.g[V1][0] == 0;
        c.g[V0][0] = 0xa;
        if at_zero {
            break 'end;
        }
        c.cycles += 2;
        let equal = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x14;
        if equal {
            break 'end;
        }
        c.cycles += 2;
        let equal = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x1e;
        if equal {
            break 'end;
        }
        c.cycles += 2;
        let different = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = 0x27;
        if different {
            'step: {
                c.cycles += 2;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V0][0] = Addr(obj.wrapping_add(0x170)).lw(c);
                    break 'step;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(obj.wrapping_add(0xe0)).lw(c);
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(obj.wrapping_add(0x170)).lw(c);
                    break 'step;
                }
                c.cycles += 3;
                c.g[V0][0] = 0x25;
                Addr(obj.wrapping_add(0x170)).sw(c, c.g[V0][0]);
                abi::ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            Addr(obj.wrapping_add(0x170)).sw(c, c.g[V0][0]);
            abi::ret(c);
            return;
        }
    }
    c.cycles += 4;
    c.g[V0][0] = Addr(obj.wrapping_add(0x170)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(9));
    Addr(obj.wrapping_add(0x170)).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void KeyboardScreen_CursorRight(KeyboardScreen* s)` (0x180678): move the
/// keyboard cursor one key to the right.
pub fn keyboard_screen_cursor_right(c: &mut Cpu) {
    let obj = abi::arg(c, 0);
    'end: {
        c.cycles += 4;
        c.g[V1][0] = Addr(obj.wrapping_add(0x170)).lw(c);
        c.g[V0][0] = 9;
        let mut equal = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x13;
        if equal {
            break 'end;
        }
        c.cycles += 2;
        equal = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x1d;
        if equal {
            break 'end;
        }
        c.cycles += 2;
        equal = c.g[V1][0] == c.g[V0][0];
        c.g[A1][0] = 0x27;
        if equal {
            break 'end;
        }
        c.cycles += 2;
        let different = c.g[V1][0] != c.g[A1][0];
        c.g[V0][0] = 0x25;
        if different {
            'step: {
                c.cycles += 2;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V0][0] = Addr(obj.wrapping_add(0x170)).lw(c);
                    break 'step;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(obj.wrapping_add(0xe0)).lw(c);
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(obj.wrapping_add(0x170)).lw(c);
                    break 'step;
                }
                c.cycles += 2;
                Addr(obj.wrapping_add(0x170)).sw(c, c.g[A1][0]);
                abi::ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            Addr(obj.wrapping_add(0x170)).sw(c, c.g[V0][0]);
            abi::ret(c);
            return;
        }
    }
    c.cycles += 4;
    c.g[V0][0] = Addr(obj.wrapping_add(0x170)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(9));
    Addr(obj.wrapping_add(0x170)).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void cFEStateVideoPlayer_dtor(cFEStateVideoPlayer* self, int free)`
/// (0x1d2380): restore the vtable, free the decoder, then chain to the base
/// destructor.
pub fn c_fe_state_video_player_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x47_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0046_9928);
        frame.save(c, S1, 0x10);
        let self_ = c.g[A0][0];
        let free = c.g[A1][0];
        c.g[S0][0] = self_;
        frame.save_ra(c, 0);
        Addr(at(c, S0, 0x8)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(at(c, S0, 0x248)).lw(c);
        let none = c.g[A0][0] == 0;
        c.g[S1][0] = free;
        if !none {
            c.cycles += 2;
            c.g[A1][0] = 0x3;
            call_ptr(c, 0x0025_3418, 0x001d_23b4)?;
        }
        c.cycles += 11;
        c.g[V0][0] = 0x47_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = sx(0x0046_d0d0);
        c.g[A1][0] = c.g[S1][0];
        Addr(at(c, S0, 0x8)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x001d_23cc)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_ResumeGameAudio(SSXAudioSystem* audio)` (0x289bb8):
/// resume the paused game-audio streams and clear their pause flags.
pub fn ssx_audio_system_resume_game_audio(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let audio = c.g[A0][0];
        c.g[S0][0] = audio;
        c.g[V0][0] = Addr(at(c, S0, 0x5fb4)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            c.g[A0][0] = sx(at(c, S0, 0x118));
            call_ptr(c, 0x002b_3a98, 0x0028_9bdc)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0029_ce70, 0x0028_9be4)?;
            c.g[V0][0] = Addr(at(c, S0, 0x5828)).lw(c);
            let quiet = c.g[V0][0] == 0;
            Addr(at(c, S0, 0x5fb4)).sw(c, 0);
            if !quiet {
                c.cycles += 6;
                c.g[A0][0] = sx(at(c, S0, 0x5560));
                c.g[A1][0] = 0;
                call_ptr(c, 0x002b_11b0, 0x0028_9bfc)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002a_4550, 0x0028_9c04)?;
                Addr(at(c, S0, 0x5828)).sw(c, 0);
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
    })
}

/// `int SSXAudioSystem_OutputModeToOption(int mode)` (0x289d60): map an
/// output-mode enum to its menu option index (1→0, 2→1, 0→2, 3→3).
pub fn ssx_audio_system_output_mode_to_option(c: &mut Cpu) {
    let mode = c.g[A1][0];
    let (option, extra) = if mode == 1 {
        (0u64, 1u64)
    } else if (mode as i64) < 2 {
        if mode == 0 {
            (2, 8)
        } else {
            (0, 7)
        }
    } else if mode == 2 {
        (1, 8)
    } else if mode == 3 {
        (3, 9)
    } else {
        (0, 9)
    };
    c.cycles += 3 + extra + 2;
    c.g[V1][0] = option;
    c.g[V0][0] = c.g[V1][0];
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Riders.

/// `void Rider_GetStartMarkerPos(Rider* rider, Vec3* out, StartMarker* marker)`
/// (0x122c28): copy the marker's position (and a 1.0 w) into `out`.
pub fn rider_get_start_marker_pos(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let out = c.g[A0][0];
        c.g[S0][0] = out;
        c.g[A0][0] = 0x004d_33a0;
        c.g[A2][0] = Addr(at(c, A1, 0x86c)).lw(c);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0026_b5e0, 0x0012_2c4c)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 2);
        c.lwc1(1, at(c, V1, 0x8));
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = Addr(r32(c, SP)).ld(c);
        let dst = r32(c, S0);
        c.swc1(1, dst);
        c.lwc1(0, at(c, V1, 0xc));
        c.swc1(0, dst.wrapping_add(4));
        c.lwc1(1, at(c, V1, 0x10));
        c.swc1(2, dst.wrapping_add(0xc));
        c.swc1(1, dst.wrapping_add(8));
        frame.restore(c, S0, 0x10);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `void Rider_AddVelocity(Rider* rider, Vec4* v)` (0x1250a8): add `v` to the
/// rider's velocity with VU0, and prime the air predictor while in the air.
pub fn rider_add_velocity(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let rider = c.g[A0][0];
        let v = c.g[A1][0];
        c.g[S0][0] = rider;
        frame.save_ra(c, 0);
        c.lqc2(3, at(c, S0, 0x1e0));
        c.lqc2(4, v as u32);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, S0, 0x1e0));
        c.g[RA][0] = sx(0x0012_50d4);
        c.g[S1][0] = sx(at(c, S0, 0x1e0));
        call_ptr(c, 0x0011_fe98, 0x0012_50d4)?;
        c.g[V1][0] = 0x1;
        if c.g[V0][0] != c.g[V1][0] {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 6;
            c.g[A0][0] = Addr(at(c, S0, 0x788)).lw(c);
            c.g[A1][0] = sx(at(c, S0, 0x110));
            c.f[12] = Addr(gp(c, -0x7880)).read_fpu(c).0;
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x0011_35b8, 0x0012_50f4)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Rider_FinishHandplant(Rider* rider)` (0x119c38): finish a handplant
/// trick, reset the trick metrix, and return the trick result in `f0`.
pub fn rider_metrix_finish_handplant(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        frame.save_fpr(c, 20, 0x20);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[V0][0] = Addr(at(c, S0, 0x80)).lw(c);
        c.g[T1][0] = 0x1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        c.g[RA][0] = sx(0x0011_9c70);
        Addr(at(c, S0, 0x80)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0011_a228, 0x0011_9c70)?;
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0011_7838, 0x0011_9c7c)?;
        Addr(at(c, S0, 0x30)).sw(c, 0);
        c.mov_s(0, 20);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void Rider_v13(Rider* rider, float dt)` (0x10e770): advance one of the
/// rider's animation timers (offset 0x2e8) and repaint it.
pub fn rider_v13(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        c.lwc1(0, at(c, S0, 0x2e8));
        c.add_s(0, 0, 12);
        Addr(at(c, S0, 0x2e8)).sw(c, c.f[0] as u64);
        call_ptr(c, 0x0028_b180, 0x0010_e790)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 12);
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0029_ced8, 0x0010_e7a8)?;
        call_ptr(c, 0x0028_b180, 0x0010_e7b0)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = 0x1;
        call_ptr(c, 0x002a_3b18, 0x0010_e7c0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Rider_v14(Rider* rider, float dt)` (0x10e7d0): the same for the
/// rider's second animation timer (offset 0x2ec).
pub fn rider_v14(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        c.lwc1(0, at(c, S0, 0x2ec));
        c.add_s(0, 0, 12);
        Addr(at(c, S0, 0x2ec)).sw(c, c.f[0] as u64);
        call_ptr(c, 0x0028_b180, 0x0010_e7f0)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 12);
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0029_ced8, 0x0010_e808)?;
        call_ptr(c, 0x0028_b180, 0x0010_e810)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = 0x2;
        call_ptr(c, 0x002a_3b18, 0x0010_e820)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio / streaming.

/// `void StreamBuffer_SetFile(StreamBuffer* buf, const char* name)`
/// (0x2a9e78): look the name up and register it on the buffer.
pub fn stream_buffer_set_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        let buf = c.g[A0][0];
        let name = c.g[A1][0];
        c.g[S1][0] = buf;
        c.g[S2][0] = name;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0031_7f38, 0x002a_9e9c)?;
        let missing = c.g[V0][0] == 0;
        c.g[S0][0] = sx(at(c, S1, 0x14));
        if !missing {
            c.cycles += 7;
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0041_66f4, 0x002a_9eb0)?;
            c.g[A1][0] = Addr(at(c, S1, 0x7c)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e_1ec8, 0x002a_9ebc)?;
            Addr(at(c, S1, 0x74)).sw(c, c.g[V0][0]);
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

/// `void SndBank_Release(SndBank* bank)` (0x28b878): unload the bank, mark it
/// released and unlink it from its parent when it was playing.
pub fn snd_bank_release(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let bank = c.g[A0][0];
        c.g[S0][0] = bank;
        call_ptr(c, 0x0028_b528, 0x0028_b88c)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0028_b2d0, 0x0028_b894)?;
        c.g[V0][0] = 0x2;
        c.g[V1][0] = Addr(at(c, S0, 0x8)).lw(c);
        let not_playing = c.g[V1][0] != c.g[V0][0];
        Addr(at(c, S0, 0x0)).sw(c, c.g[V0][0]);
        if !not_playing {
            c.cycles += 9;
            c.g[V1][0] = Addr(gp(c, 0x524)).lw(c);
            c.g[A1][0] = Addr(at(c, S0, 0xc)).lw(c);
            c.g[V0][0] = Addr(r32(c, V1)).off(0x1d8).lw(c);
            c.g[A2][0] = Addr(r32(c, V0)).lw(c);
            c.g[V1][0] = Addr(r32(c, A2)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).off(0x30).lh(c);
            c.g[V0][0] = Addr(r32(c, V1)).off(0x34).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = abi::addu(c.g[A2][0], c.g[A0][0]);
            call_ptr(c, target, 0x0028_b8c8)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BEOptionInt_IsAnyHandicapSet(...)` (0x14f810): true when any of the
/// twelve handicap entries of the option table is set.
pub fn be_option_int_is_any_handicap_set(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0xc;
    c.g[V1][0] = 0x53_0000;
    c.mult(0, 4, 4, 2);
    c.g[A1][0] = sx(0x0053_08b8);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
    let mut any = c.g[V1][0] != 0;
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    if !any {
        c.cycles += 3;
        c.g[V1][0] = Addr(r32(c, V0)).off(0x4).lw(c);
        any = c.g[V1][0] != 0;
        if !any {
            c.cycles += 3;
            c.g[V1][0] = Addr(r32(c, V0)).off(0x8).lw(c);
            any = c.g[V1][0] != 0;
            if !any {
                c.cycles += 3;
                c.g[V0][0] = Addr(at(c, A1, 0x18)).lw(c);
                any = c.g[V0][0] != 0;
                if !any {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(at(c, A1, 0x1c)).lw(c);
                    if c.g[V0][0] == 0 {
                        c.cycles += 2;
                        c.g[V0][0] = 0;
                        abi::ret(c);
                        return;
                    }
                    any = true;
                }
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `void ScriptList_CanSkip(ScriptList* list)` (0x277060): true when the
/// current command is skippable and the list has no pending delay.
pub fn script_list_can_skip(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let list = c.g[A0][0];
        c.g[S0][0] = list;
        frame.save_ra(c, 0);
        c.g[V1][0] = 0x3;
        c.g[V0][0] = Addr(at(c, S0, 0xa4)).lw(c);
        let not_script = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = 0;
        if !not_script {
            'check: {
                c.cycles += 7;
                c.g[A0][0] = sx(at(c, S0, 0xc));
                c.g[A1][0] = 0;
                call_ptr(c, 0x0028_3d70, 0x0027_7090)?;
                c.g[V1][0] = Addr(r32(c, V0)).off(0x8).lw(c);
                c.g[V1][0] = c.g[V1][0] & 0x1;
                let not_skippable = c.g[V1][0] == 0;
                c.g[S1][0] = 0;
                if not_skippable {
                    break 'check;
                }
                c.cycles += 2;
                c.g[V0][0] = Addr(at(c, S0, 0xa8)).lw(c);
                c.g[S1][0] = (c.g[V0][0] < 0x1) as u64;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cStreamMan_Reset(cStreamMan* man)` (0x3a6ed8): clear the streaming
/// queues and re-apply the default rate.
pub fn c_stream_man_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = u64::MAX;
        c.g[A2][0] = 0x1;
        c.g[A1][0] = 0x1_0000;
        Addr(at(c, V0, 0x94)).sw(c, c.g[V1][0]);
        Addr(at(c, V0, 0xd4)).sw(c, c.g[A2][0]);
        c.g[A1][0] = c.g[A1][0] | 0x9000;
        Addr(at(c, V0, 0x98)).sw(c, 0);
        Addr(at(c, V0, 0x4)).sb(c, 0);
        Addr(at(c, V0, 0x90)).sw(c, 0);
        Addr(at(c, V0, 0xc0)).sw(c, 0);
        Addr(at(c, V0, 0xcc)).sw(c, 0);
        Addr(at(c, V0, 0xd0)).sw(c, 0);
        Addr(at(c, V0, 0xc8)).sw(c, 0);
        Addr(at(c, V0, 0xc4)).sw(c, 0);
        Addr(at(c, V0, 0xac)).sw(c, 0);
        Addr(at(c, V0, 0xb0)).sw(c, 0);
        Addr(at(c, V0, 0xb4)).sw(c, 0);
        call_ptr(c, 0x003a_7058, 0x003a_6f2c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Transitions, memory and pads.

/// `void cTransitionMan_ScriptObjCtor(cTransitionMan_ScriptObj* self)`
/// (0x282cd0): construct the script-object base and clear the transition
/// fields.
pub fn c_transition_man_script_obj_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x2;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = c.g[A0][0];
        c.g[S0][0] = self_;
        call_ptr(c, 0x0028_3200, 0x0028_2ce8)?;
        c.g[V1][0] = 0x482_060;
        c.g[A1][0] = u64::MAX;
        c.g[V1][0] = sx(0x0048_2060);
        c.g[A0][0] = 0x9;
        Addr(at(c, S0, 0xc)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(at(c, S0, 0x14)).sw(c, c.g[A1][0]);
        Addr(at(c, S0, 0x18)).sw(c, c.g[A0][0]);
        Addr(at(c, S0, 0x10)).sw(c, 0);
        Addr(at(c, S0, 0x2c)).sw(c, 0);
        Addr(at(c, S0, 0x1c)).sw(c, 0);
        Addr(at(c, S0, 0x20)).sw(c, 0);
        Addr(at(c, S0, 0x24)).sw(c, 0);
        Addr(at(c, S0, 0x28)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* MEM_ResizeBlockLocked(void* block, size_t size)` (0x2526b8):
/// resize a block under the allocator lock.
pub fn mem_resize_block_locked(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = sx(0x0053_8b00);
        frame.save(c, S1, 0x20);
        let block = c.g[A0][0];
        let size = c.g[A1][0];
        c.g[S2][0] = block;
        frame.save_ra(c, 0);
        c.g[S1][0] = size;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_5700, 0x0025_26e4)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0031_9bd8, 0x0025_26f0)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_5760, 0x0025_26fc)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VidEnginePlayer_Pause(VidEnginePlayer* player)` (0x2539e0): pause
/// the player once its stream is ready and idle.
pub fn vid_engine_player_pause(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let player = c.g[A0][0];
        c.g[S0][0] = player;
        c.g[A0][0] = Addr(at(c, S0, 0x38)).lw(c);
        let mut done = c.g[A0][0] == 0;
        if !done {
            c.cycles += 3;
            c.g[V0][0] = Addr(at(c, S0, 0x3c)).lw(c);
            done = c.g[V0][0] == 0;
            if !done {
                c.cycles += 3;
                c.g[V0][0] = Addr(at(c, S0, 0x0)).lw(c);
                done = c.g[V0][0] != 0;
                if !done {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(at(c, S0, 0x8)).lw(c);
                    done = c.g[V0][0] != 0;
                    if !done {
                        c.cycles += 5;
                        call_ptr(c, 0x003a_e938, 0x0025_3a28)?;
                        c.g[V1][0] = 0x1;
                        Addr(at(c, S0, 0x8)).sw(c, c.g[V1][0]);
                    }
                }
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* WorldTriggerManager_GetSpecialTriggerInfo(WorldTriggerManager* m,
/// int id, int* a, int* b, int* c)` (0x2b6550): return the special trigger's
/// info of `id`, zeroing the three output words.
pub fn world_trigger_manager_get_special_trigger_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 0x4d;
        let t1 = c.g[T1][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = t1;
        Addr(r32(c, A2)).sw(c, 0);
        Addr(r32(c, A3)).sw(c, 0);
        let other = c.g[A1][0] != c.g[V0][0];
        Addr(r32(c, T0)).sw(c, 0);
        if !other {
            c.cycles += 9;
            c.g[V0][0] = 0x1;
            Addr(r32(c, T0)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x0028_b180, 0x002b_6584)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A3][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x1;
            call_ptr(c, 0x002a_34d0, 0x002b_6598)?;
            c.g[V1][0] = c.g[V0][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = c.g[V1][0];
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int WorldTriggerManager_WasPlayed(WorldTriggerManager* m, int id)`
/// (0x2b6808): mark `id` as the last played trigger and report whether it
/// had already been played.
pub fn world_trigger_manager_was_played(c: &mut Cpu) {
    c.cycles += 3;
    let mgr = abi::arg(c, 0);
    let id = c.g[A1][0];
    c.g[V0][0] = Addr(mgr.wrapping_add(0x32c)).lw(c);
    if id != c.g[V0][0] {
        c.g[V0][0] = Addr(mgr.wrapping_add(0x324)).lw(c);
        'scan: {
            c.cycles += 3;
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let negative = (c.g[V1][0] as i64) < 0;
            Addr(mgr.wrapping_add(0x32c)).sw(c, id);
            if negative {
                break 'scan;
            }
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(mgr));
            loop {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, A2)).lw(c);
                let hit = c.g[V0][0] == id;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                if hit {
                    c.cycles += 4;
                    c.g[V1][0] = 0x1;
                    c.g[V0][0] = 0x1;
                    Addr(mgr.wrapping_add(0x330)).sw(c, c.g[V1][0]);
                    abi::ret(c);
                    return;
                }
                c.cycles += 2;
                let more = (c.g[V1][0] as i64) >= 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(4));
                if !more {
                    break 'scan;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002b_6848);
                }
            }
        }
        c.cycles += 3;
        Addr(mgr.wrapping_add(0x330)).sw(c, 0);
        c.g[V0][0] = 0;
        abi::ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = Addr(mgr.wrapping_add(0x330)).lw(c);
        abi::ret(c);
    }
}

// ---------------------------------------------------------------------------
// UI screens.

/// `UIThread* UIScreen_GetPrimaryThread(UIScreen* screen)` (0x39c7c0): walk
/// the screen's UI list and return the first non-sentinel node that is not
/// hidden (bit 25 of its flags clear), or null.
pub fn ui_screen_get_primary_thread(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0039_c7c0 => {
                    c.cycles += 8;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_7840, 0x0039_c7d4)?;
                    c.g[S0][0] = c.g[V0][0];
                    let none = c.g[S0][0] == 0;
                    c.g[V0][0] = 0;
                    if none {
                        pc = 0x0039_c810;
                        continue;
                    }
                    pc = 0x0039_c7e0;
                }
                0x0039_c7e0 => {
                    c.cycles += 2;
                    pc = 0x0039_c800;
                }
                0x0039_c7e8 => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(at(c, S0, 0x10)).lw(c);
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 25) as u32);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let hidden = c.g[V0][0] != 0;
                    c.g[V0][0] = c.g[S0][0];
                    if hidden {
                        pc = 0x0039_c810;
                        continue;
                    }
                    pc = 0x0039_c7fc;
                }
                0x0039_c7fc => {
                    c.cycles += 1;
                    c.g[S0][0] = Addr(at(c, S0, 0x4)).lw(c);
                    pc = 0x0039_c800;
                }
                0x0039_c800 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039_7718, 0x0039_c808)?;
                    let sentinel = c.g[V0][0] == 0;
                    c.g[V0][0] = 0;
                    if sentinel {
                        if back_edge_unwinds(c, 0x0039_c7e8) {
                            return Ok(());
                        }
                        pc = 0x0039_c7e8;
                        continue;
                    }
                    pc = 0x0039_c810;
                }
                0x0039_c810 => {
                    c.cycles += 4;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = Addr(r32(c, SP)).ld(c);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0039_c7c0, 0x0039_c820);
                    return Ok(());
                }
            }
        }
    })
}

/// `void UIScreen_CmdSetObjectParam(UIScreen* screen, int a1, void* cmd)`
/// (0x39cd30): find the named object and apply the parameter command to it.
pub fn ui_screen_cmd_set_object_param(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let cmd = c.g[A2][0];
        c.g[S1][0] = cmd;
        c.g[A1][0] = Addr(at(c, S1, 0x4)).lw(c);
        call_ptr(c, 0x0039_d860, 0x0039_cd4c)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 10;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039_fd38, 0x0039_cd60)?;
            c.g[V0][0] = Addr(at(c, S0, 0x8)).lw(c);
            c.g[A2][0] = sx(at(c, S1, 0xc));
            c.g[A1][0] = Addr(at(c, S1, 0x8)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(0x68).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x6c).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
            call_ptr(c, target, 0x0039_cd7c)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIPair_RefreshText(UIPair* pair)` (0x39be10): refresh the two text
/// widgets of the pair.
pub fn ui_pair_refresh_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let pair = c.g[A0][0];
        c.g[S0][0] = pair;
        c.g[A1][0] = Addr(at(c, S0, 0x78)).lw(c);
        if c.g[A1][0] != 0 {
            c.cycles += 6;
            c.g[V0][0] = Addr(at(c, A1, 0x8)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(0x80).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x84).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = abi::addu(c.g[A1][0], c.g[A0][0]);
            call_ptr(c, target, 0x0039_be40)?;
            c.g[A1][0] = Addr(at(c, S0, 0x7c)).lw(c);
        } else {
            c.g[A1][0] = Addr(at(c, S0, 0x7c)).lw(c);
        }
        'second: {
            c.cycles += 2;
            let none = c.g[A1][0] == 0;
            frame.restore(c, S0, 0x10);
            if none {
                break 'second;
            }
            c.cycles += 6;
            c.g[V0][0] = Addr(at(c, A1, 0x8)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(0x80).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x84).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = abi::addu(c.g[A1][0], c.g[A0][0]);
            call_ptr(c, target, 0x0039_be60)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenAutosave_ctor(OVScreenAutosave* self, int a1, void* a2)`
/// (0x212138): construct the autosave overlay widget.
pub fn ov_screen_autosave_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let self_ = c.g[A0][0];
        let a2 = c.g[A2][0];
        c.g[S0][0] = self_;
        c.g[S1][0] = a2;
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        call_ptr(c, 0x001d_53b0, 0x0021_2158)?;
        c.g[V1][0] = 0x471_f80;
        c.g[A1][0] = 0x25;
        c.g[V1][0] = sx(0x0047_1f80);
        c.g[A0][0] = 0x1;
        Addr(at(c, S0, 0x22c)).sw(c, c.g[S1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(at(c, S0, 0x8)).sw(c, c.g[V1][0]);
        Addr(at(c, S0, 0xc)).sw(c, c.g[A1][0]);
        Addr(at(c, S0, 0x208)).sw(c, c.g[A0][0]);
        Addr(at(c, S0, 0x1c4)).sw(c, 0);
        Addr(at(c, S0, 0x230)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMountainRoom_ShowImage(cFEStateMountainRoom* self, int a1)`
/// (0x1d32e0): show the room's image, and the second one when it has been
/// unlocked.
pub fn c_fe_state_mountain_room_show_image(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = c.g[A0][0];
        c.g[S0][0] = self_;
        c.g[A2][0] = Addr(at(c, S0, 0x48)).lw(c);
        c.g[V0][0] = Addr(r32(c, A2)).off(0x8).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x48).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x4c).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = abi::addu(c.g[A2][0], c.g[A0][0]);
        call_ptr(c, target, 0x001d_330c)?;
        c.g[V0][0] = Addr(at(c, S0, 0xc4)).lw(c);
        let locked = (c.g[V0][0] as i64) < 0;
        c.g[A1][0] = 0x1;
        if !locked {
            c.cycles += 6;
            c.g[A2][0] = Addr(at(c, S0, 0x4c)).lw(c);
            c.g[V0][0] = Addr(r32(c, A2)).off(0x8).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(0x48).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x4c).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = abi::addu(c.g[A2][0], c.g[A0][0]);
            call_ptr(c, target, 0x001d_3330)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenReadPrivateMessage_OpenKeyboard(...)` (0x210f40): open the
/// keyboard popup for the private-message overlay.
pub fn ov_screen_read_private_message_open_keyboard(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let a2 = c.g[A2][0];
        frame.save(c, S2, 0x10);
        let self_ = c.g[A0][0];
        c.g[S0][0] = a2;
        c.g[A2][0] = c.g[T0][0];
        c.g[S1][0] = self_;
        let a3 = c.g[A3][0];
        c.g[S2][0] = a3;
        frame.save_ra(c, 0);
        c.g[T0][0] = c.g[T2][0];
        c.g[A3][0] = c.g[T1][0];
        call_ptr(c, 0x0021_0fa0, 0x0021_0f70)?;
        c.g[A0][0] = Addr(at(c, S1, 0xe0)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x001c_cf00, 0x0021_0f7c)?;
        c.g[A0][0] = Addr(at(c, S1, 0xe0)).lw(c);
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x001c_b418, 0x0021_0f88)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
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
        c.g[A0][0] = Addr(r32(c, V1)).off(0x5f0).lw(c);
        c.g[A1][0] = Addr(r32(c, V0)).off(0x5c12).lb(c);
        call_ptr(c, 0x0014_ab20, 0x0015_5164)?;
        let table = c.g[V0][0] != 0;
        c.g[V1][0] = 0x14;
        if table {
            c.cycles += 3;
            c.mult(0, A0, S0, V1);
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[V0][0] = Addr(r32(c, V1)).lw(c);
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

/// `void PS2BezierMan_FreeCaches(PS2BezierMan* man)` (0x38b0f8): free the
/// renderer's three cache blocks.
pub fn ps2_bezier_man_free_caches(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let man = c.g[A0][0];
        c.g[S0][0] = man;
        call_ptr(c, 0x0038_d660, 0x0038_b10c)?;
        c.g[A0][0] = Addr(at(c, S0, 0x440)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0038_b120)?;
        }
        c.g[A0][0] = Addr(at(c, S0, 0x444)).lw(c);
        'second: {
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                c.g[A0][0] = Addr(at(c, S0, 0x448)).lw(c);
                break 'second;
            }
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0038_b134)?;
            c.g[A0][0] = Addr(at(c, S0, 0x448)).lw(c);
        }
        'third: {
            c.cycles += 2;
            let none = c.g[A0][0] == 0;
            frame.restore(c, S0, 0x10);
            if none {
                break 'third;
            }
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0038_b148)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FEAsyncManager_dtor(FEAsyncManager* self, int free)` (0x1a35e8):
/// release the async manager's job list and optionally delete it.
pub fn fe_async_manager_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        let self_ = c.g[A0][0];
        c.g[S0][0] = self_;
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x0046_1418;
        frame.save_ra(c, 0);
        let free = c.g[A1][0];
        c.g[S1][0] = free;
        call_ptr(c, 0x0031_7f38, 0x001a_360c)?;
        let absent = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if !absent {
            c.cycles += 4;
            c.g[A0][0] = Addr(at(c, S0, 0x58c)).lw(c);
            c.g[A1][0] = 0x64;
            call_ptr(c, 0x003d_edc0, 0x001a_3620)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        'delete: {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x20);
                break 'delete;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x001a_3634)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBackground_Init(cFEStateBackground* self)` (0x194528):
/// create the background screen's widget tree.
pub fn c_fe_state_background_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        let self_ = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = self_;
        c.g[A0][0] = 0x0045_ffd8;
        c.g[S1][0] = Addr(at(c, S0, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0019_454c)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0019_4560)?;
        let none = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[V0][0];
        if !none {
            c.cycles += 3;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0019_4574)?;
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
// Pads and strings.

/// `void PS2Pad_ctor(PS2Pad* pad, int port, int slot)` (0x326df0): install the
/// vtable, remember the port/slot and clear the pad state.
pub fn ps2_pad_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x48_e488;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx(0x0048_e488);
        let self_ = c.g[A0][0];
        let port = c.g[A1][0];
        let slot = c.g[A2][0];
        c.g[S0][0] = self_;
        frame.save_ra(c, 0);
        Addr(at(c, S0, 0x4)).sw(c, port);
        c.g[A0][0] = sx(at(c, S0, 0xc));
        Addr(at(c, S0, 0x8)).sw(c, slot);
        c.g[A1][0] = 0;
        Addr(at(c, S0, 0x0)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 0x24;
        call_ptr(c, 0x0041_6210, 0x0032_6e24)?;
        c.g[V0][0] = c.g[S0][0];
        Addr(at(c, S0, 0x54)).sw(c, 0);
        Addr(at(c, S0, 0x58)).sw(c, 0);
        Addr(at(c, S0, 0x50)).sw(c, 0);
        Addr(at(c, S0, 0x5c)).sw(c, 0);
        Addr(at(c, S0, 0x48)).sw(c, 0);
        Addr(at(c, S0, 0x4c)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2Pad_dtor(PS2Pad* pad, int free)` (0x326e50): restore the vtable,
/// tear the pad down and optionally delete it.
pub fn ps2_pad_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x48_e488;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0048_e488);
        frame.save(c, S1, 0x10);
        let free = c.g[A1][0];
        c.g[S0][0] = free;
        frame.save_ra(c, 0);
        let self_ = c.g[A0][0];
        c.g[S1][0] = self_;
        Addr(at(c, S1, 0x0)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(at(c, S1, 0x4)).lw(c);
        c.g[A1][0] = Addr(at(c, S1, 0x8)).lw(c);
        call_ptr(c, 0x003f_f8f0, 0x0032_6e80)?;
        c.g[V1][0] = 0x48_e4b8;
        c.g[S0][0] = c.g[S0][0] & 0x1;
        c.g[V1][0] = sx(0x0048_e4b8);
        let keep = c.g[S0][0] == 0;
        Addr(at(c, S1, 0x0)).sw(c, c.g[V1][0]);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0032_6e9c)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cBXString_AllocBeforeWrite(cBXString* s, int needed)` (0x318188):
/// reallocate the string's buffer when the reference is not unique or the
/// buffer is too small for `needed` bytes.
pub fn c_bx_string_alloc_before_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let s = c.g[A0][0];
        let needed = c.g[A1][0];
        c.g[S0][0] = s;
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0xffff_fff4).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
        c.g[S1][0] = needed;
        let rebuild;
        if c.g[V0][0] == 0 {
            // `refcount >= 2`: the buffer is shared, rebuild without a
            // capacity test (the generated `t = V0 == 0` arm).
            rebuild = true;
        } else {
            // Unique reference: rebuild only when the buffer is too small.
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, V1)).off(0xffff_fffc).lw(c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            rebuild = c.g[V0][0] != 0;
        }
        if rebuild {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_80b8, 0x0031_81c8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0031_8040, 0x0031_81d4)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LightMan_v10(LightMan* man, void* obj, int a2, int a3)` (0x38df38):
/// resolve the object's light hook and run it.
pub fn light_man_v10(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let a3 = c.g[A3][0];
        let a2 = c.g[A2][0];
        c.g[S0][0] = a3;
        frame.save_ra(c, 0);
        c.g[S1][0] = a2;
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(20, 12);
        c.g[A2][0] = Addr(r32(c, A0)).lw(c);
        c.g[V1][0] = Addr(r32(c, A2)).off(0x10).lh(c);
        c.g[V0][0] = Addr(r32(c, A2)).off(0x14).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = abi::addu(c.g[A0][0], c.g[V1][0]);
        call_ptr(c, target, 0x0038_df6c)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0038_dc58, 0x0038_df80)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Start screen and camera algorithms.

/// `void StartScreen_Enter(StartScreen* self)` (0x232c98): start the game
/// from the front end through the world's race manager.
pub fn start_screen_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        let self_ = c.g[A0][0];
        c.g[S0][0] = self_;
        frame.save_ra(c, 0x10);
        c.g[A0][0] = Addr(gp(c, -0x854)).lw(c);
        call_ptr(c, 0x0036_9ff8, 0x0023_2cb0)?;
        Addr(at(c, S0, 0x14)).sw(c, 0);
        Addr(gp(c, 0x1234)).sw(c, 0);
        Addr(gp(c, 0x14e8)).sw(c, 0);
        Addr(gp(c, 0x12d8)).sw(c, 0);
        c.g[V0][0] = Addr(gp(c, -0x854)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x10d8).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x78).lh(c);
        Addr(r32(c, SP)).sw(c, 0);
        Addr(r32(c, SP).wrapping_add(4)).sw(c, 0);
        c.g[A0][0] = abi::addu(c.g[V0][0], c.g[A0][0]);
        Addr(r32(c, SP).wrapping_add(8)).sw(c, 0);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x7c).lw(c);
        let target = r32(c, V0);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, target, 0x0023_2ce8)?;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void ManualScriptCameraAlgo_ctor(ManualScriptCameraAlgo* self)` (0x1694b8).
pub fn manual_script_camera_algo_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = c.g[A0][0];
        c.g[S0][0] = self_;
        call_ptr(c, 0x0016_9418, 0x0016_94cc)?;
        c.g[V1][0] = 0x45_c850;
        c.g[A0][0] = sx(at(c, S0, 0x74));
        c.g[V1][0] = sx(0x0045_c850);
        Addr(at(c, S0, 0x10)).sw(c, c.g[V1][0]);
        call_ptr(c, 0x0015_fff0, 0x0016_94e0)?;
        c.g[V1][0] = 0x4f_f130;
        c.g[A1][0] = 0x56;
        c.g[V1][0] = sx(0x004f_f130);
        c.g[V0][0] = c.g[S0][0];
        c.lq(4, r32(c, V1));
        Addr(at(c, S0, 0xc)).sw(c, c.g[A1][0]);
        c.sq(4, at(c, S0, 0x30));
        Addr(at(c, S0, 0x44)).sw(c, 0);
        Addr(at(c, S0, 0x48)).sw(c, 0);
        Addr(at(c, S0, 0x40)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void TargetScriptCameraAlgo_ctor(TargetScriptCameraAlgo* self)` (0x169cf8).
pub fn target_script_camera_algo_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = c.g[A0][0];
        c.g[S0][0] = self_;
        call_ptr(c, 0x0016_9418, 0x0016_9d0c)?;
        c.g[V1][0] = 0x45_c7e8;
        c.g[A0][0] = sx(at(c, S0, 0x74));
        c.g[V1][0] = sx(0x0045_c7e8);
        Addr(at(c, S0, 0x10)).sw(c, c.g[V1][0]);
        call_ptr(c, 0x0015_fff0, 0x0016_9d20)?;
        c.g[V1][0] = 0x4f_f130;
        c.g[A1][0] = 0x57;
        c.g[V1][0] = sx(0x004f_f130);
        c.g[V0][0] = c.g[S0][0];
        c.lq(4, r32(c, V1));
        Addr(at(c, S0, 0xc)).sw(c, c.g[A1][0]);
        c.sq(4, at(c, S0, 0x30));
        Addr(at(c, S0, 0x40)).sw(c, 0);
        Addr(at(c, S0, 0x44)).sw(c, 0);
        Addr(at(c, S0, 0x48)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SubjectScriptCameraAlgo_ctor(SubjectScriptCameraAlgo* self)` (0x16a458).
pub fn subject_script_camera_algo_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = c.g[A0][0];
        c.g[S0][0] = self_;
        call_ptr(c, 0x0016_9418, 0x0016_a46c)?;
        c.g[V1][0] = 0x45_c780;
        c.g[A0][0] = sx(at(c, S0, 0x70));
        c.g[V1][0] = sx(0x0045_c780);
        Addr(at(c, S0, 0x10)).sw(c, c.g[V1][0]);
        call_ptr(c, 0x0015_fff0, 0x0016_a480)?;
        c.g[V1][0] = 0x4f_f130;
        c.g[A1][0] = 0x58;
        c.g[V1][0] = sx(0x004f_f130);
        c.g[V0][0] = c.g[S0][0];
        c.lq(4, r32(c, V1));
        Addr(at(c, S0, 0xc)).sw(c, c.g[A1][0]);
        c.sq(4, at(c, S0, 0x30));
        Addr(at(c, S0, 0x40)).sw(c, 0);
        Addr(at(c, S0, 0x44)).sw(c, 0);
        Addr(at(c, S0, 0x48)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MEM_Init(void* arena, size_t size)` (0x319a30): initialise the main
/// DLHeap and the small-block pools.
pub fn mem_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        let arena = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x53_8b00;
        let size = c.g[A1][0];
        c.g[S0][0] = arena;
        c.g[S1][0] = size;
        frame.save_ra(c, 0);
        call_ptr(c, 0x003e_5698, 0x0031_9a54)?;
        c.g[A0][0] = 0x4f_f250;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        Addr(gp(c, 0xdcc)).sw(c, c.g[S0][0]);
        Addr(gp(c, 0xdd0)).sw(c, c.g[S1][0]);
        call_ptr(c, 0x0031_ff60, 0x0031_9a70)?;
        c.g[V0][0] = 0x1;
        Addr(gp(c, 0xdc8)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_9d20, 0x0031_9a7c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end buy-attrib and event-select screens.

/// `void cFEStateBuyAttrib_UpdateTotalCost(cFEStateBuyAttrib* self)`
/// (0x1f5240): recompute the total cost label from the selected item.
pub fn c_fe_state_buy_attrib_update_total_cost(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        let self_ = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x46_f090;
        c.g[S1][0] = self_;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x0046_f090);
        call_ptr(c, 0x0031_7670, 0x001f_5260)?;
        c.g[A0][0] = Addr(at(c, S1, 0x48)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_526c)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 6;
            c.g[A0][0] = Addr(at(c, S1, 0xa0)).lw(c);
            call_ptr(c, 0x0019_8af0, 0x001f_5280)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x001f_528c)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBuyAttrib_UpdateMoney(cFEStateBuyAttrib* self)`
/// (0x1f52a0): refresh the money label from the player's balance.
pub fn c_fe_state_buy_attrib_update_money(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        let self_ = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x46_f0a0;
        c.g[S1][0] = self_;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x0046_f0a0);
        call_ptr(c, 0x0031_7670, 0x001f_52c0)?;
        c.g[A0][0] = Addr(at(c, S1, 0x48)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_52cc)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 6;
            c.g[A0][0] = Addr(at(c, S1, 0xa4)).lw(c);
            call_ptr(c, 0x0019_8af0, 0x001f_52e0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x001f_52ec)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EventSelect_Shutdown(EventSelect* self)` (0x200af0): close the
/// event-select screen and its PDA template.
pub fn event_select_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        let self_ = c.g[A0][0];
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr((self_ as u32).wrapping_add(0x2d8)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x11c).lw(c);
        call_ptr(c, 0x001a_38c0, 0x0020_0b08)?;
        c.g[V1][0] = Addr(gp(c, -0xb5c)).lw(c);
        c.g[V0][0] = 0x1;
        let first = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x3;
        let mut close = first;
        if !first {
            c.cycles += 2;
            let second = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x4;
            close = second;
            if !second {
                c.cycles += 2;
                close = c.g[V1][0] == c.g[V0][0];
            }
        }
        if close {
            c.cycles += 2;
            call_ptr(c, 0x0020_d190, 0x0020_0b30)?;
        }
        c.cycles += 8;
        call_ptr(c, 0x0028_b180, 0x0020_0b38)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0028_f678, 0x0020_0b44)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EventSelect_SetHighlightColor(EventSelect* self, void* obj, void* c)`
/// (0x2083d8): tell the row object to use the highlighted colour.
pub fn event_select_set_highlight_color(c: &mut Cpu) {
    event_select_set_color(c, 0x74c8, 0x74d8, 0x0020_8408, 0x0020_842c)
}

/// `void EventSelect_SetUnlockedColor(EventSelect* self, void* obj, void* c)`
/// (0x208438): tell the row object to use the unlocked colour.
pub fn event_select_set_unlocked_color(c: &mut Cpu) {
    event_select_set_color(c, 0x74a8, 0x74b8, 0x0020_8468, 0x0020_848c)
}

/// The shared body of the two `EventSelect_Set*Color` functions: call the
/// row's virtual colour setter with the colour object (or with no argument
/// when it is null).
fn event_select_set_color(c: &mut Cpu, with: u32, without: u32, ret1: u32, ret2: u32) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let obj = c.g[A1][0];
        c.g[A3][0] = obj;
        let none = c.g[A3][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            'set: {
                c.cycles += 2;
                let no_colour = c.g[A2][0] == 0;
                c.g[A1][0] = 0x4d_0000;
                if !no_colour {
                    c.cycles += 8;
                    c.g[V1][0] = Addr(at(c, A3, 0x8)).lw(c);
                    c.g[A1][0] = sx(0x004d_0000u32.wrapping_sub(with));
                    c.g[A0][0] = Addr(r32(c, V1)).off(0x58).lh(c);
                    c.g[V0][0] = Addr(r32(c, V1)).off(0x5c).lw(c);
                    let target = r32(c, V0);
                    c.g[A0][0] = abi::addu(c.g[A3][0], c.g[A0][0]);
                    call_ptr(c, target, ret1)?;
                    frame.restore_ra(c, 0);
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = t;
                    return Ok(());
                }
            }
            c.cycles += 7;
            c.g[V1][0] = Addr(at(c, A3, 0x8)).lw(c);
            c.g[A1][0] = 0x4d_0000;
            c.g[A1][0] = sx(0x004d_0000u32.wrapping_sub(without));
            c.g[A0][0] = Addr(r32(c, V1)).off(0x58).lh(c);
            c.g[V0][0] = Addr(r32(c, V1)).off(0x5c).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = abi::addu(c.g[A3][0], c.g[A0][0]);
            call_ptr(c, target, ret2)?;
        }
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, fakes, fill, put_vec, Rng, DATA, GAME_GP};

    /// The object most fixtures live at.
    const OBJ: u32 = DATA + 0x1000;
    const OBJ2: u32 = DATA + 0x2000;
    const OBJ3: u32 = DATA + 0x3000;
    const OBJ4: u32 = DATA + 0x4000;
    /// A fake vtable page.
    const VT: u32 = DATA + 0x8000;
    /// The main heap's dlmalloc arena over 256 KiB of test RAM.
    const MEM_HEAP: u32 = 0x004f_f250;

    fn w(c: &mut Cpu, at: u32, v: u32) {
        c.bus.write32(at, v);
    }

    /// A vtable slot `{ short delta; short; fn }` at `vt + off`.
    fn slot(c: &mut Cpu, vt: u32, off: u32, delta: i16) {
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, fakes::FAKE);
    }

    /// The engine's size-class pool allocator: blocks up to 0x400 bytes come
    /// from per-size free lists hanging off `gp + 0x2a60..0x2a84` when the
    /// pools-ready flag at `gp + 0xdd8` is set.
    fn pool_allocator(c: &mut Cpu) {
        w(c, GAME_GP + 0xdd8, 0); // (re-written below as 1)
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
        for (i, (slot_off, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes + 0x100 * i as u32;
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                w(c, node, if n == 3 { 0 } else { node + 0x40 });
            }
            c.bus.write32(pool + head, base);
            w(c, GAME_GP + slot_off, pool);
        }
    }

    /// Run one generated function as a setup step, with `args` in a0..t1.
    fn run(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
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

    fn heap_init(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        run(
            c,
            0x0031_ff60,
            ssx3_ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x40000) as u64, 0x40000],
        );
    }

    fn heap_block(c: &mut Cpu, size: u32) -> u32 {
        heap_init(c);
        run(
            c,
            0x0032_0058,
            ssx3_ee::s10::MEM_HeapMemalign,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32
    }

    // ------------------------------------------------------------- keyboard

    #[test]
    fn keyboard_screen_cursor_left_matches() {
        check(
            ssx3_ee::s02::f_00180618__generated,
            keyboard_screen_cursor_left,
            120,
            &|c, seed| {
                fakes::machine(c, seed, 0x180618, 0);
                let cursor = [0u32, 10, 20, 30, 39, 0x25, 0x27, 5][(seed % 8) as usize];
                w(c, OBJ + 0x170, cursor);
                w(c, OBJ + 0xe0, if seed % 3 == 0 { 0 } else { seed });
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn keyboard_screen_cursor_right_matches() {
        check(
            ssx3_ee::s02::f_00180678__generated,
            keyboard_screen_cursor_right,
            120,
            &|c, seed| {
                fakes::machine(c, seed, 0x180678, 0);
                let cursor = [9u32, 19, 29, 39, 0x25, 0x27, 5, 0x1d][(seed % 8) as usize];
                w(c, OBJ + 0x170, cursor);
                w(c, OBJ + 0xe0, if seed % 3 == 0 { 0 } else { seed });
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn ssx_audio_system_output_mode_to_option_matches() {
        check(
            ssx3_ee::s07::f_00289d60__generated,
            ssx_audio_system_output_mode_to_option,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x289d60, 0);
                c.g[A1][0] = match seed % 7 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => 4,
                    5 => 0x1_0000_0001,
                    _ => u64::MAX,
                };
            },
        );
    }

    // ----------------------------------------------------------- triggers

    #[test]
    fn world_trigger_manager_was_played_matches() {
        check(
            ssx3_ee::s08::f_002b6808__generated,
            world_trigger_manager_was_played,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x2b6808, 0);
                let id = (seed % 5) as u32;
                let last = if seed % 2 == 0 { id } else { (seed + 1) % 5 };
                w(c, OBJ + 0x32c, last);
                w(c, OBJ + 0x330, (seed / 2) % 2);
                let count = (seed / 5) % 4;
                w(c, OBJ + 0x324, count);
                for i in 0..count {
                    w(
                        c,
                        OBJ + 4 + 4 * i,
                        if i + 1 == count { id } else { (seed + i) % 5 },
                    );
                }
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = id as u64;
            },
        );
    }

    #[test]
    fn be_option_int_is_any_handicap_set_matches() {
        check_lohi(
            ssx3_ee::s01::f_0014f810__generated,
            be_option_int_is_any_handicap_set,
            100,
            &|c, seed| {
                fakes::machine(c, seed, 0x14f810, 0);
                let index = (seed % 4) as u32;
                for i in 0..4u32 {
                    let set = i <= index && (seed >> (i + 2)) & 1 != 0;
                    w(c, 0x0053_08b8 + 12 * i, set as u32);
                }
                w(c, 0x0053_08b8 + 0x18, (seed >> 1) & 1);
                w(c, 0x0053_08b8 + 0x1c, seed & 1);
                c.g[A0][0] = index as u64;
            },
        );
    }

    // ------------------------------------------------------------- objects

    #[test]
    fn mod_obj_add_tex_flip_modifier_matches() {
        check(
            ssx3_ee::s11::f_00355bf0__generated,
            mod_obj_add_tex_flip_modifier,
            40,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x355bf0, 0);
                pool_allocator(c);
                fill(c, &mut r, OBJ2, 0x40);
                w(c, OBJ2 + 0x4, r.next_u32());
                w(c, OBJ2 + 0x8, r.next_u32());
                w(c, OBJ2 + 0xc, r.next_u32());
                w(c, OBJ2 + 0x10, 1 + (seed % 8));
                w(c, OBJ2 + 0x14, r.next_u32());
                w(c, OBJ + 0x18, OBJ3);
                w(c, OBJ + 0x1c, if seed % 2 == 0 { 0 } else { OBJ4 });
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
            },
        );
    }

    #[test]
    fn mod_obj_add_boost_modifier_matches() {
        check(
            ssx3_ee::s11::f_00355c50__generated,
            mod_obj_add_boost_modifier,
            40,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x355c50, 0);
                pool_allocator(c);
                fill(c, &mut r, OBJ2, 0x40);
                w(c, OBJ + 0x18, OBJ3);
                w(c, OBJ + 0x1c, if seed % 2 == 0 { 0 } else { OBJ4 });
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
            },
        );
    }

    #[test]
    fn ava_spline_modifier_save_matches() {
        check(
            ssx3_ee::s11::f_00357848__generated,
            ava_spline_modifier_save,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x357848, 0);
                w(c, OBJ, VT);
                slot(c, VT, 0x28, 0);
                slot(c, VT, 0x8, 0);
                w(c, OBJ + 0x80, seed);
                w(c, OBJ2 + 0x80, seed.wrapping_mul(3));
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
            },
        );
    }

    #[test]
    fn ps2_pad_ctor_matches() {
        check(ssx3_ee::s10::f_00326df0__generated, ps2_pad_ctor, 60, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x326df0, 0);
            fill(c, &mut r, OBJ, 0x20);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = (seed % 2) as u64;
            c.g[A2][0] = (seed % 4) as u64;
        });
    }

    #[test]
    fn ps2_pad_dtor_matches() {
        check(ssx3_ee::s10::f_00326e50__generated, ps2_pad_dtor, 60, &|c, seed| {
            fakes::machine(c, seed, 0x326e50, 0);
            w(c, OBJ + 0x4, seed.wrapping_mul(7));
            w(c, OBJ + 0x8, seed.wrapping_mul(11));
            w(c, OBJ, 0);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = (seed & 1) as u64;
        });
    }

    #[test]
    fn c_transition_man_script_obj_ctor_matches() {
        check(
            ssx3_ee::s07::f_00282cd0__generated,
            c_transition_man_script_obj_ctor,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x282cd0, 0);
                c.bus.write32(OBJ, seed);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn c_bx_string_alloc_before_write_matches() {
        check(
            ssx3_ee::s10::f_00318188__generated,
            c_bx_string_alloc_before_write,
            40,
            &|c, seed| {
                fakes::machine(c, seed, 0x318188, 0);
                pool_allocator(c);
                let buf = OBJ2;
                // Cover all three paths of the reference/capacity test:
                //  seed%4 == 0/1: unique reference, buffer big enough  -> early
                //                 return after the (charged) size test;
                //  seed%4 == 2:   shared reference (refcount >= 2)     -> rebuild
                //                 with no size test;
                //  seed%4 == 3:   refcount sign-extends negative, so the size
                //                 test runs and rebuilds.  (`refcount == 1`
                //                 would make `cBXString_Release` free the fake
                //                 buffer, so it is left to its own test.)
                let (refcount, capacity) = match seed % 4 {
                    0 => (1u32, 0x1000u32),
                    1 => (0, 0x1000),
                    2 => (5, 0x10),
                    _ => (0xffff_ffff, 0x10),
                };
                w(c, buf - 0xc, refcount);
                w(c, buf - 4, capacity);
                w(c, OBJ, buf);
                w(c, buf, 0x41);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = ((seed * 37) % 200) as u64;
            },
        );
    }

    // -------------------------------------------------------------- riders

    #[test]
    fn rider_get_start_marker_pos_matches() {
        check(
            ssx3_ee::s00::f_00122c28__generated,
            rider_get_start_marker_pos,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x122c28, 0);
                w(c, OBJ2 + 0x86c, seed % 4);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
            },
        );
    }

    #[test]
    fn rider_add_velocity_matches() {
        check(
            ssx3_ee::s00::f_001250a8__generated,
            rider_add_velocity,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x1250a8, 0);
                let state = OBJ2 + 0x800;
                w(c, OBJ + 0x77c, state);
                w(c, state + 0xde0, if seed % 2 == 0 { 1 } else { 2 });
                put_vec(c, OBJ + 0x1e0, [0x3f80_0000, 0x4000_0000, 0, 0x3f80_0000]);
                put_vec(c, OBJ2, [1, 2, 3, 4]);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
            },
        );
    }

    // ------------------------------------------------------------- streaming

    #[test]
    fn stream_buffer_set_file_matches() {
        check(
            ssx3_ee::s08::f_002a9e78__generated,
            stream_buffer_set_file,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x2a9e78, 0);
                for i in 0..8u32 {
                    c.bus
                        .write8(OBJ2 + i, if i < 4 { b"data"[i as usize] } else { 0 });
                }
                w(c, OBJ + 0x7c, seed);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
            },
        );
    }

    #[test]
    fn snd_bank_release_matches() {
        check(
            ssx3_ee::s07::f_0028b878__generated,
            snd_bank_release,
            40,
            &|c, seed| {
                fakes::machine(c, seed, 0x28b878, 0);
                w(c, OBJ + 0x8, (seed % 3) as u32);
                w(c, OBJ + 0xc, seed);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn c_stream_man_reset_matches() {
        check(
            ssx3_ee::s13::f_003a6ed8__generated,
            c_stream_man_reset,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x3a6ed8, 0);
                // The audio frame duration used by cStreamMan_SetRate.
                w(c, GAME_GP + 0x2a74, 50 + seed % 20);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 0x8000) as u64;
            },
        );
    }

    // ---------------------------------------------------------- memory / UI

    #[test]
    fn mem_init_matches() {
        check(ssx3_ee::s10::f_00319a30__generated, mem_init, 12, &|c, seed| {
            fakes::machine(c, seed, 0x319a30, 0);
            let arena = DATA + 0x40000;
            c.g[A0][0] = arena as u64;
            c.g[A1][0] = 0x40000 - (seed as u64 % 0x40) * 16;
        });
    }

    #[test]
    fn mem_resize_block_locked_matches() {
        check(
            ssx3_ee::s06::f_002526b8__generated,
            mem_resize_block_locked,
            10,
            &|c, seed| {
                fakes::machine(c, seed, 0x2526b8, 0);
                let block = heap_block(c, 0x40);
                c.g[A0][0] = block as u64;
                c.g[A1][0] = 0x20 + (seed as u64 % 0x40);
            },
        );
    }

    #[test]
    fn ui_screen_get_primary_thread_matches() {
        check(
            ssx3_ee::s13::f_0039c7c0__generated,
            ui_screen_get_primary_thread,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x39c7c0, 0);
                let l = OBJ + 0x18;
                if seed % 3 == 0 {
                    w(c, l + 4, 0);
                } else {
                    let n0 = OBJ2;
                    let n1 = OBJ2 + 0x40;
                    let sentinel = OBJ2 + 0x80;
                    w(c, l + 4, n0);
                    w(c, n0, n0.wrapping_add(0x100));
                    w(c, n0 + 4, n1);
                    w(c, n1, n1.wrapping_add(0x100));
                    w(c, n1 + 4, sentinel);
                    w(c, sentinel, sentinel);
                    w(c, sentinel + 4, sentinel);
                    let hidden = if seed % 2 == 0 { 1u32 << 25 } else { 0 };
                    w(c, n0 + 0x10, hidden);
                    w(c, n1 + 0x10, if seed % 5 == 1 { 1 << 25 } else { 0 });
                }
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn ui_screen_cmd_set_object_param_matches() {
        check(
            ssx3_ee::s13::f_0039cd30__generated,
            ui_screen_cmd_set_object_param,
            40,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x39cd30, 0);
                let name = OBJ3 + 0x100;
                let count = OBJ2 + 0x800;
                for (i, b) in b"button\0".iter().enumerate() {
                    c.bus.write8(name + i as u32, *b);
                }
                if seed % 2 == 0 {
                    // One entry whose name is `name`: the find loop matches on
                    // its first iteration (`screen+0x3c` is the entry array,
                    // `screen+0x38` holds the count and the first entry).
                    w(c, OBJ + 0x3c, OBJ2);
                    w(c, OBJ2, OBJ3);
                    w(c, OBJ + 0x38, count);
                    w(c, count, 1);
                    w(c, OBJ3 + 0x38, name);
                } else {
                    // No entry array at all: the find returns null at once.
                    w(c, OBJ + 0x3c, 0);
                    w(c, OBJ + 0x38, 0);
                }
                w(c, OBJ3 + 0x8, VT);
                slot(c, VT, 0x68, 0);
                w(c, OBJ4 + 0x4, name);
                w(c, OBJ4 + 0x8, r.next_u32());
                w(c, OBJ4 + 0xc, r.next_u32());
                c.g[A0][0] = OBJ as u64;
                c.g[A2][0] = OBJ4 as u64;
            },
        );
    }

    #[test]
    fn ui_pair_refresh_text_matches() {
        check(
            ssx3_ee::s13::f_0039be10__generated,
            ui_pair_refresh_text,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x39be10, 0);
                w(c, OBJ + 0x8, VT);
                slot(c, VT, 0x80, 0);
                let first = if seed % 3 == 0 { 0 } else { OBJ2 };
                let second = if seed % 4 == 0 { 0 } else { OBJ3 };
                w(c, OBJ + 0x78, first);
                w(c, OBJ + 0x7c, second);
                w(c, OBJ2 + 0x8, VT);
                w(c, OBJ3 + 0x8, VT);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn ov_screen_autosave_ctor_matches() {
        check(
            ssx3_ee::s05::f_00212138__generated,
            ov_screen_autosave_ctor,
            16,
            &|c, seed| {
                fakes::machine(c, seed, 0x212138, 0);
                // `MemCardStateBase_InstallCallbacks` (through the object ctor)
                // reaches BELibrary_GetInstance: a created dummy keeps it from
                // building the real library (and loading from disc).
                w(c, GAME_GP - 0x1f04, OBJ3);
                w(c, GAME_GP - 0x1efc, 0);
                w(c, 0x0053_08a8, 0);
                c.bus.write32(OBJ, seed);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = seed as u64;
                c.g[A2][0] = OBJ2 as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_mountain_room_show_image_matches() {
        check(
            ssx3_ee::s04::f_001d32e0__generated,
            c_fe_state_mountain_room_show_image,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x1d32e0, 0);
                w(c, OBJ + 0x48, OBJ2);
                w(c, OBJ + 0x4c, OBJ3);
                w(c, OBJ2 + 0x8, VT);
                w(c, OBJ3 + 0x8, VT);
                slot(c, VT, 0x48, 0);
                w(
                    c,
                    OBJ + 0xc4,
                    if seed % 2 == 0 { 0xffff_ffff } else { seed },
                );
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_background_init_matches() {
        check(
            ssx3_ee::s02::f_00194528__generated,
            c_fe_state_background_init,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x194528, 0);
                w(c, OBJ + 0x10, if seed % 2 == 0 { 0 } else { OBJ2 });
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn event_select_set_highlight_color_matches() {
        check(
            ssx3_ee::s05::f_002083d8__generated,
            event_select_set_highlight_color,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x2083d8, 0);
                w(c, OBJ + 0x8, VT);
                slot(c, VT, 0x58, 0);
                c.g[A0][0] = OBJ2 as u64;
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { OBJ as u64 };
                c.g[A2][0] = if seed % 4 == 0 { 0 } else { OBJ3 as u64 };
            },
        );
    }

    #[test]
    fn event_select_set_unlocked_color_matches() {
        check(
            ssx3_ee::s05::f_00208438__generated,
            event_select_set_unlocked_color,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x208438, 0);
                w(c, OBJ + 0x8, VT);
                slot(c, VT, 0x58, 0);
                c.g[A0][0] = OBJ2 as u64;
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { OBJ as u64 };
                c.g[A2][0] = if seed % 4 == 0 { 0 } else { OBJ3 as u64 };
            },
        );
    }

    #[test]
    fn event_select_shutdown_matches() {
        check(
            ssx3_ee::s05::f_00200af0__generated,
            event_select_shutdown,
            40,
            &|c, seed| {
                fakes::machine(c, seed, 0x200af0, 0);
                w(c, GAME_GP - 0x848, 0);
                w(
                    c,
                    GAME_GP - 0xb5c,
                    match seed % 4 {
                        0 => 1,
                        1 => 3,
                        2 => 4,
                        _ => seed,
                    },
                );
                w(c, OBJ + 0x2d8, OBJ2);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn ov_screen_read_private_message_open_keyboard_matches() {
        check(
            ssx3_ee::s05::f_00210f40__generated,
            ov_screen_read_private_message_open_keyboard,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x210f40, 0);
                pool_allocator(c);
                w(c, OBJ + 0xe0, 0);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = seed as u64;
                c.g[A2][0] = OBJ2 as u64;
                c.g[A3][0] = OBJ3 as u64;
                c.g[T0][0] = seed.wrapping_mul(3) as u64;
                c.g[T1][0] = seed.wrapping_mul(5) as u64;
                c.g[T2][0] = seed.wrapping_mul(7) as u64;
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

    // ------------------------------------------------------------- cameras

    #[test]
    fn light_man_v10_matches() {
        check(ssx3_ee::s12::f_0038df38__generated, light_man_v10, 80, &|c, seed| {
            fakes::machine(c, seed, 0x38df38, 0);
            w(c, OBJ, VT);
            slot(c, VT, 0x10, 0);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = OBJ2 as u64;
            c.g[A2][0] = seed as u64;
            c.g[A3][0] = OBJ3 as u64;
            c.f[12] = ((seed as f32) * 0.5).to_bits();
        });
    }

    #[test]
    fn start_screen_enter_matches() {
        check(
            ssx3_ee::s06::f_00232c98__generated,
            start_screen_enter,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x232c98, 0);
                w(c, GAME_GP - 0x854, 0);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn manual_script_camera_algo_ctor_matches() {
        check(
            ssx3_ee::s02::f_001694b8__generated,
            manual_script_camera_algo_ctor,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x1694b8, 0);
                c.bus.write32(OBJ, seed);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn target_script_camera_algo_ctor_matches() {
        check(
            ssx3_ee::s02::f_00169cf8__generated,
            target_script_camera_algo_ctor,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x169cf8, 0);
                c.bus.write32(OBJ, seed);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn subject_script_camera_algo_ctor_matches() {
        check(
            ssx3_ee::s02::f_0016a458__generated,
            subject_script_camera_algo_ctor,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x16a458, 0);
                c.bus.write32(OBJ, seed);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    // ------------------------------------------------------- menus / audio

    #[test]
    fn lobby_menu_init_message_page_matches() {
        check(
            ssx3_ee::s02::f_0017d200__generated,
            lobby_menu_init_message_page,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x17d200, 0);
                c.bus.write32(OBJ, seed);
                w(c, OBJ + 0x130, seed);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_accept_challenge_matches() {
        check(
            ssx3_ee::s02::f_0017e490__generated,
            lobby_menu_accept_challenge,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x17e490, 0);
                pool_allocator(c);
                let node = OBJ2;
                w(c, node, 0);
                w(c, node + 0x4, OBJ3);
                w(c, OBJ + 0x8, if seed % 2 == 0 { 0 } else { node });
                w(c, OBJ + 0x1c0c, seed);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn ssx_audio_system_resume_game_audio_matches() {
        check(
            ssx3_ee::s07::f_00289bb8__generated,
            ssx_audio_system_resume_game_audio,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x289bb8, 0);
                w(c, OBJ + 0x5fb4, if seed % 3 == 0 { 0 } else { 1 });
                w(c, OBJ + 0x5828, if seed % 2 == 0 { 0 } else { 1 });
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn script_list_can_skip_matches() {
        check(
            ssx3_ee::s07::f_00277060__generated,
            script_list_can_skip,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x277060, 0);
                w(c, OBJ + 0xa4, if seed % 3 == 0 { 3 } else { seed % 4 });
                w(c, OBJ + 0x8c, seed);
                w(c, OBJ + 0xa8, if seed % 2 == 0 { 0 } else { seed % 4 });
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_video_player_dtor_matches() {
        check(
            ssx3_ee::s04::f_001d2380__generated,
            c_fe_state_video_player_dtor,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x1d2380, 0);
                w(c, OBJ + 0x248, if seed % 2 == 0 { 0 } else { OBJ2 });
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    #[test]
    fn vid_engine_player_pause_matches() {
        check(
            ssx3_ee::s06::f_002539e0__generated,
            vid_engine_player_pause,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x2539e0, 0);
                w(c, OBJ + 0x38, if seed % 2 == 0 { 0 } else { seed });
                w(c, OBJ + 0x3c, if seed % 3 == 0 { 0 } else { seed });
                w(c, OBJ, (seed / 2) % 2);
                w(c, OBJ + 0x8, (seed / 4) % 2);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn fe_async_manager_dtor_matches() {
        check(
            ssx3_ee::s03::f_001a35e8__generated,
            fe_async_manager_dtor,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x1a35e8, 0);
                w(c, OBJ + 0x58c, 0);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    #[test]
    fn world_trigger_manager_get_special_trigger_info_matches() {
        check(
            ssx3_ee::s08::f_002b6550__generated,
            world_trigger_manager_get_special_trigger_info,
            8,
            &|c, seed| {
                fakes::machine(c, seed, 0x2b6550, 0);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    0x4d
                } else {
                    seed as u64 % 0x4e
                };
                c.g[A2][0] = OBJ2 as u64;
                c.g[A3][0] = OBJ3 as u64;
                c.g[T0][0] = OBJ4 as u64;
                c.g[T1][0] = (OBJ2 + 0x100) as u64;
            },
        );
    }
}
