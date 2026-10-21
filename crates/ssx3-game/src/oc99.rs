//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold-path front-end, network, audio, menu, replay and
//! camera helpers of shards s01-s13, each with a differential test against the
//! generated body.
//!
//! Most are compiler-shaped wrappers: open a 0x10-0x40 frame, save `ra` (and
//! the `s0`/`s1` they reuse), charge the block's cycles, pass a member or a
//! vtable method along, and return what the callee returned. The ports keep
//! the frame layout, the cycle charges of every block (including the branches
//! that jump into the middle of an epilogue), the scratch values the generated
//! body leaves in `v0`/`v1`/`a*`, and the unwind check through [`crate::call`].
//!
//! A few are real bodies: the replay highlight counter, the text-flip
//! modifier, the INI section test, the mission-type scan, the leap-year day
//! count and the lobby persona hooks.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, sp, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001a_3dc8,
        c_fe_state_net_messenger_ctor,
        "cFEStateNetMessenger_ctor",
    ),
    (
        0x0026_8240,
        net_event102_tooy_clone,
        "NetEvent102Tooy_Clone",
    ),
    (
        0x0028_03f0,
        rider_script_obj_deactivate,
        "RiderScriptObj_Deactivate",
    ),
    (0x002c_c350, menu_sys_draw, "MenuSys_Draw"),
    (0x0037_1318, dynamic_emitter_load, "DynamicEmitter_Load"),
    (0x0026_4030, c_setup_man_create, "cSetupMan_Create"),
    (
        0x001c_af58,
        c_ui_state_buy_popup_set_uber_trick_params,
        "cUIStateBuyPopup_SetUberTrickParams",
    ),
    (0x001a_0648, fe_flow_state_init, "FEFlowState_Init"),
    (
        0x001a_9b58,
        net_fe_state_open_messenger,
        "NetFEState_OpenMessenger",
    ),
    (
        0x001b_a218,
        c_fe_state_create_step1_restore_account_info,
        "cFEStateCreateStep1_RestoreAccountInfo",
    ),
    (
        0x0026_6170,
        c_shared_login_man_mc_mkdir,
        "cSharedLoginMan_McMkdir",
    ),
    (
        0x0026_61d8,
        c_shared_login_man_mc_delete_file,
        "cSharedLoginMan_McDeleteFile",
    ),
    (
        0x0039_29f8,
        font_draw_text_centered_f,
        "Font_DrawTextCenteredF",
    ),
    (0x0039_2b40, font_draw_text_right_f, "Font_DrawTextRightF"),
    (
        0x001e_0a50,
        c_fe_state_challenge_received_popup_dtor,
        "cFEStateChallengeReceivedPopup_dtor",
    ),
    (
        0x001d_f878,
        c_fe_state_net_create_room_popup_dtor,
        "cFEStateNetCreateRoomPopup_dtor",
    ),
    (
        0x001e_0cf8,
        c_fe_state_create_step1_dtor,
        "cFEStateCreateStep1_dtor",
    ),
    (0x0035_e1e0, roller_modifier_save, "RollerModifier_Save"),
    (
        0x0016_1370,
        spline_camera_algorithm_dtor,
        "SplineCameraAlgorithm_dtor",
    ),
    (
        0x001c_5a90,
        c_fe_popup_confirm_base_dtor,
        "cFEPopupConfirm_BaseDtor",
    ),
    (
        0x0016_20d0,
        chase_cam_controller_clear_override,
        "ChaseCamController_ClearOverride",
    ),
    (
        0x0019_54d8,
        c_fe_state_bonus_material_init,
        "cFEStateBonusMaterial_Init",
    ),
    (
        0x001a_3090,
        fe_screen_sel_multi_init,
        "FEScreenSelMulti_Init",
    ),
    (
        0x001c_2138,
        c_fe_state_net_quick_messages_init,
        "cFEStateNetQuickMessages_Init",
    ),
    (
        0x002a_6848,
        crowd_audio_instance_stop_sound,
        "CrowdAudio_InstanceStopSound",
    ),
    (0x0027_2ec0, in_seq_span_track_stop, "InSeq_SpanTrackStop"),
    (0x0027_32c8, in_seq_event_track_stop, "InSeq_EventTrackStop"),
    (0x0027_3630, in_seq_ramp_track_stop, "InSeq_RampTrackStop"),
    (
        0x0035_f3a8,
        tex_flip_modifier_apply_frame,
        "TexFlipModifier_ApplyFrame",
    ),
    (
        0x0025_f1e0,
        lobby_man_get_room_user_name,
        "LobbyMan_GetRoomUserName",
    ),
    (
        0x002a_4078,
        ssx_audio_system_is_track_group0_event,
        "SSXAudioSystem_IsTrackGroup0Event",
    ),
    (
        0x002a_7f90,
        ini_parser_at_section_end,
        "IniParser_AtSectionEnd",
    ),
    (
        0x0027_0070,
        player_alloc_replay_cache,
        "Player_AllocReplayCache",
    ),
    (
        0x0027_0bd8,
        replay_count_highlights,
        "Replay_CountHighlights",
    ),
    (0x0023_e3c0, mc_manager_state37, "MCManager_State37"),
    (0x0024_0960, mc_manager_state1b, "MCManager_State1B"),
    (0x003a_ee30, av_audio_dtor, "AV_Audio_dtor"),
    (
        0x0024_5598,
        c_game_load_state_on_widget_created,
        "cGameLoadState_OnWidgetCreated",
    ),
    (
        0x0030_b8c0,
        w_script_man_is_mission_type2_active,
        "WScriptMan_IsMissionType2Active",
    ),
    (0x0035_60c0, mod_obj_v28, "ModObj_v28"),
    (0x0017_d688, lobby_menu_update, "LobbyMenu_Update"),
    (
        0x0017_e1c0,
        lobby_menu_create_persona,
        "LobbyMenu_CreatePersona",
    ),
    (
        0x0017_e228,
        lobby_menu_delete_persona,
        "LobbyMenu_DeletePersona",
    ),
    (
        0x0017_e290,
        lobby_menu_select_persona,
        "LobbyMenu_SelectPersona",
    ),
    (
        0x0017_e4f0,
        lobby_menu_decline_challenge,
        "LobbyMenu_DeclineChallenge",
    ),
    (
        0x001a_30f8,
        fe_screen_sel_multi_on_widget_created,
        "FEScreenSelMulti_OnWidgetCreated",
    ),
    (
        0x001b_6e40,
        c_fe_state_auth_dnas_error_on_widget_action,
        "cFEStateAuthDNASError_OnWidgetAction",
    ),
    (
        0x001b_a110,
        c_fe_state_create_step1_validate_fields,
        "cFEStateCreateStep1_ValidateFields",
    ),
    (
        0x001b_b9b0,
        c_fe_state_create_step2_get_days_in_month,
        "cFEStateCreateStep2_GetDaysInMonth",
    ),
    (
        0x0026_5700,
        c_setup_man_send_brag_msg12,
        "cSetupMan_SendBragMsg12",
    ),
    // --- end of overrides ---
];

// ---------------------------------------------------------------------------
// Helpers.

/// `a + off`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global address.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    abi::r32(c, GP).wrapping_add(off as u32)
}

/// `lw` at `base + off`.
#[inline(always)]
fn lw_at(c: &mut Cpu, base: u32, off: u32) -> u64 {
    Addr(p(base, off)).lw(c)
}

/// `lw` at `base + off` as a 32-bit value.
#[inline(always)]
fn u32_at(c: &mut Cpu, base: u32, off: u32) -> u32 {
    lw_at(c, base, off) as u32
}

// ---------------------------------------------------------------------------
// Front end: net messenger creation and screen initialisation.

/// `cFEStateNetMessenger_ctor(this, engine, owner)` 0x1a3dc8: base
/// `NetFEState_ctor`, then install the derived vtable and clear the messenger
/// fields (`this + 0x6d0..0x6f8`), storing the shared empty object pointer.
pub fn c_fe_state_net_messenger_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        frame.save_ra(c, 0x10);
        c.g[S0][0] = this64;
        call_ptr(c, 0x001a_8500, 0x001a_3ddc)?;
        let empty = Addr(gp(c, 0xda0)).lw(c);
        c.g[V0][0] = sx(0x0046_7f28);
        c.g[V1][0] = empty;
        c.g[A0][0] = u64::MAX;
        Addr(p(this, 0x8)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0x46;
        Addr(p(this, 0x6e0)).sw(c, c.g[V1][0]);
        c.g[V0][0] = this64;
        Addr(p(this, 0x6f8)).sw(c, c.g[A0][0]);
        Addr(p(this, 0xc)).sw(c, c.g[A1][0]);
        Addr(p(this, 0x6d0)).sw(c, c.g[V1][0]);
        Addr(p(this, 0x6d4)).sw(c, c.g[V1][0]);
        for off in [0x6dc, 0x6e8, 0x6ec, 0x6f0, 0x6f4] {
            Addr(p(this, off)).sw(c, 0);
        }
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `NetEvent102Tooy_Clone(ev)` 0x268240: allocate a 0x10-byte clone, tag it
/// and copy the event kind at `+0xc`.
pub fn net_event102_tooy_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        c.g[A1][0] = 0x480000;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = sx(0x0048_0488);
        c.g[S0][0] = this64;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x10;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_8268)?;
        let clone = c.g[V0][0];
        c.g[V1][0] = clone;
        c.g[A2][0] = lw_at(c, this, 0xc);
        c.g[V0][0] = 0x102;
        c.g[A0][0] = 0x746f_0000;
        c.g[A1][0] = 0x480000;
        c.g[A0][0] |= 0x6f79;
        c.g[A1][0] = sx(0x0048_11f0);
        Addr(clone as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = clone;
        frame.restore_ra(c, 0);
        Addr(p(clone as u32, 8)).sw(c, c.g[A0][0]);
        Addr(p(clone as u32, 4)).sw(c, c.g[A1][0]);
        Addr(p(clone as u32, 0xc)).sw(c, c.g[A2][0]);
        frame.close(c);
        Ok(())
    })
}

/// `RiderScriptObj_Deactivate(obj)` 0x2803f0: deactivate the embedded script
/// object, copy the identity transform into `obj + 0x30` and clear the active
/// flags.
pub fn rider_script_obj_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        c.g[S0][0] = this64;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(this, 0xc));
        call_ptr(c, 0x0028_3440, 0x0028_0408)?;
        c.g[V0][0] = 0x500000;
        c.g[V1][0] = sx(p(this, 0x30));
        c.g[V0][0] = sx(0x004f_f1a0);
        let src = c.g[V0][0] as u32;
        let dst = c.g[V1][0] as u32;
        for q in 0..4u32 {
            c.lqc2(q as usize + 1, p(src, q * 0x10));
        }
        for q in 0..4u32 {
            c.sqc2(q as usize + 1, p(dst, q * 0x10));
        }
        Addr(p(this, 0x20)).sw(c, 0);
        for off in [0x70, 0x74, 0x78, 0x1c] {
            Addr(p(this, off)).sw(c, 0);
        }
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSys_Draw(item)` 0x2cc350: compute the item's scaled size into its
/// object (`+0x38`/`+0x3c`) and forward to the draw method of the item's
/// selected object.
pub fn menu_sys_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        c.lwc1(1, gp(c, -0x3ff4));
        frame.save_ra(c, 0x10);
        let obj = lw_at(c, this, 0x58) as u32;
        c.g[V1][0] = obj as u64;
        c.swc1(1, p(sp(c), 4));
        c.swc1(1, sp(c));
        c.lwc1(0, p(obj, 0x30));
        c.lwc1(2, p(obj, 0x34));
        c.mul_s(0, 0, 1);
        c.swc1(0, p(obj, 0x38));
        c.lwc1(1, p(sp(c), 4));
        c.mul_s(2, 2, 1);
        c.swc1(2, p(obj, 0x3c));
        c.g[V0][0] = Addr(this).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, V0)));
        c.g[A1][0] = Addr(p(r32(c, A0), 4)).lw(c);
        let target = Addr(p(r32(c, A1), 0x12c)).lw(c) as u32;
        c.g[V0][0] = target as u64;
        c.g[A0][0] = Addr(p(target, 0x28)).lh(c);
        c.g[V1][0] = lw_at(c, target, 0x2c);
        let func = r32(c, V1);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, func, 0x002c_c3ac)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `DynamicEmitter_Load(emitter, data)` 0x371318: let the data object's
/// loader fill a local parameter block, then copy the position, range and
/// count fields into the emitter.
pub fn dynamic_emitter_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        let data64 = abi::arg64(c, 1);
        let data = data64 as u32;
        c.g[A3][0] = data64;
        frame.save(c, S0, 0x30);
        c.g[A1][0] = sp(c) as u64;
        c.g[A2][0] = 0x14;
        frame.save_ra(c, 0x20);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        let src = lw_at(c, data, 0) as u32;
        c.g[V1][0] = src as u64;
        c.g[A0][0] = Addr(p(src, 0x10)).lh(c);
        c.g[V0][0] = lw_at(c, src, 0x14);
        let func = r32(c, V0);
        c.g[A0][0] = sx(data.wrapping_add(r32(c, A0)));
        call_ptr(c, func, 0x0037_1348)?;
        let base = sp(c);
        c.lwc1(0, base);
        c.lwc1(1, p(base, 4));
        c.g[V1][0] = lw_at(c, base, 0xc);
        c.g[A0][0] = lw_at(c, base, 8);
        c.g[V0][0] = lw_at(c, base, 0x10);
        c.swc1(0, this);
        Addr(p(this, 0x17c)).sw(c, c.g[V0][0]);
        c.swc1(1, p(this, 0x28));
        Addr(p(this, 0x170)).sw(c, c.g[V1][0]);
        Addr(p(this, 0x174)).sw(c, c.g[A0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `cSetupMan_Create(a, b, c)` 0x264030: allocate a 0x12c-byte setup manager,
/// construct it and publish it in the `gp + 0x2dc` singleton.
pub fn c_setup_man_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        let a0 = abi::arg64(c, 0);
        let a1 = abi::arg64(c, 1);
        let a2 = abi::arg64(c, 2);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = a1;
        frame.save(c, S2, 0x10);
        c.g[S0][0] = a0;
        c.g[S2][0] = a2;
        c.g[A1][0] = 0x480000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(0x0048_0f08);
        c.g[A0][0] = 0x12c;
        c.g[A2][0] = 0x2000_0000;
        call_ptr(c, 0x0031_7d70, 0x0026_4068)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        let mem = c.g[V0][0];
        c.g[A0][0] = mem;
        call_ptr(c, 0x0026_40c0, 0x0026_407c)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        Addr(gp(c, 0x2dc)).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `cUIStateBuyPopup_SetUberTrickParams(popup, trick, a2, a3)` 0x1caf58:
/// remember the four parameters and the two hashed labels for the buy popup.
pub fn c_ui_state_buy_popup_set_uber_trick_params(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        frame.save_ra(c, 0);
        c.g[V0][0] = 2;
        c.g[S0][0] = this64;
        c.g[A0][0] = 0x460000;
        Addr(p(this, 0x48)).sw(c, c.g[A2][0]);
        c.g[A0][0] = sx(0x0046_6338);
        Addr(p(this, 0x4c)).sw(c, c.g[A3][0]);
        c.g[S1][0] = 3;
        Addr(p(this, 0x58)).sw(c, c.g[A1][0]);
        Addr(p(this, 0x54)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7670, 0x001c_af90)?;
        Addr(p(this, 0x5c)).sw(c, c.g[S1][0]);
        c.g[A0][0] = 0x460000;
        Addr(p(this, 0x60)).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx(0x0046_6350);
        call_ptr(c, 0x0031_7670, 0x001c_afa4)?;
        Addr(p(this, 0x64)).sw(c, c.g[S1][0]);
        Addr(p(this, 0x68)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `FEFlowState_Init(state, n)` 0x1a0648: free the previous list, remember
/// the sign-extended byte count, allocate the new list and clear it.
pub fn fe_flow_state_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x30);
        let n = (abi::arg(c, 1) as u8 as i8) as i32 as u32;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        c.g[S0][0] = sx(n);
        c.g[S1][0] = this64;
        frame.save_ra(c, 0);
        call_ptr(c, 0x001a_06b0, 0x001a_0668)?;
        Addr(this).sb(c, c.g[S0][0]);
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        c.g[A1][0] = sx(0x0046_0b80);
        c.g[A2][0] = 0x100;
        call_ptr(c, 0x0031_7e30, 0x001a_0684)?;
        let list = c.g[V0][0];
        c.g[A2][0] = c.g[S0][0];
        Addr(p(this, 4)).sw(c, list);
        c.g[A0][0] = list;
        c.g[A1][0] = 0;
        call_ptr(c, 0x0041_6210, 0x001a_0698)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `NetFEState_OpenMessenger(state)` 0x1a9b58: allocate and construct the
/// net messenger, tag it, and add it to the state's messenger list.
pub fn net_fe_state_open_messenger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        c.g[A1][0] = 0x460000;
        frame.save(c, S0, 0x10);
        c.g[A3][0] = 0;
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(0x0046_1d40);
        c.g[A0][0] = 0x6fc;
        c.g[A2][0] = 0x100;
        call_ptr(c, 0x0031_7d70, 0x001a_9b80)?;
        c.g[A1][0] = lw_at(c, this, 0x10);
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = this as u64;
        call_ptr(c, 0x001a_3dc8, 0x001a_9b90)?;
        let msg = c.g[V0][0];
        c.g[V1][0] = 0x833;
        c.g[A1][0] = msg;
        Addr(p(msg as u32, 0x18)).sw(c, c.g[V1][0]);
        let list = lw_at(c, this, 0x40) as u32;
        c.g[V0][0] = list as u64;
        let vt = lw_at(c, list, 0xd0) as u32;
        c.g[V1][0] = vt as u64;
        c.g[A0][0] = lw_at(c, vt, 0x10);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
        call_ptr(c, 0x0039_f290, 0x001a_9bb0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateCreateStep1_RestoreAccountInfo(this)` 0x1ba218: copy the three
/// saved account strings back into the form object and restore its two
/// scalar fields.
pub fn c_fe_state_create_step1_restore_account_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = abi::arg(c, 0);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(this);
        c.g[A1][0] = sx(p(this, 0x6d4));
        c.g[A0][0] = lw_at(c, this, 0x6d0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
        call_ptr(c, 0x0031_8448, 0x001b_a238)?;
        c.g[A0][0] = lw_at(c, this, 0x6d0);
        c.g[A1][0] = sx(p(this, 0x6d8));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(8));
        call_ptr(c, 0x0031_8448, 0x001b_a248)?;
        c.g[A0][0] = lw_at(c, this, 0x6d0);
        c.g[A1][0] = sx(p(this, 0x6dc));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0xc));
        call_ptr(c, 0x0031_8448, 0x001b_a258)?;
        c.g[V0][0] = lw_at(c, this, 0x6e0);
        c.g[V1][0] = lw_at(c, this, 0x6d0);
        frame.restore_ra(c, 0);
        Addr(p(r32(c, V1), 0x38)).sw(c, c.g[V0][0]);
        c.g[A0][0] = lw_at(c, this, 0x6e4);
        c.g[V0][0] = lw_at(c, this, 0x6d0);
        frame.restore(c, S0, 0x10);
        Addr(p(r32(c, V0), 0x3c)).sw(c, c.g[A0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `cSharedLoginMan_McMkdir(self, name, code)` 0x266170: fill the memory-card
/// error record, copy the name into a local buffer and ask the MC layer to
/// create the directory; record the pending request kind.
pub fn c_shared_login_man_mc_mkdir(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        let self_ = abi::arg64(c, 0);
        let name = abi::arg64(c, 1);
        c.g[S1][0] = self_;
        c.g[S2][0] = name;
        frame.save(c, S0, 0x90);
        frame.save_ra(c, 0x60);
        c.g[A1][0] = sp(c) as u64;
        c.g[A0][0] = abi::arg64(c, 2);
        call_ptr(c, 0x003f_a1d0, 0x0026_6198)?;
        let buf = p(sp(c), 0x10);
        c.g[S0][0] = sx(buf);
        c.g[A1][0] = self_;
        c.g[A0][0] = sx(buf);
        call_ptr(c, 0x0041_66f4, 0x0026_61a8)?;
        c.g[A0][0] = Addr(gp(c, 0x2e8)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S2][0];
        call_ptr(c, 0x002c_6c50, 0x0026_61b8)?;
        c.g[V0][0] = 0xb;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore_ra(c, 0x60);
        Addr(gp(c, 0x2e4)).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `cSharedLoginMan_McDeleteFile(self, name, code)` 0x2661d8: fill the error
/// record, format the path and ask the MC layer to remove the file.
pub fn c_shared_login_man_mc_delete_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        let self_ = abi::arg64(c, 0);
        let name = abi::arg64(c, 1);
        c.g[S0][0] = self_;
        c.g[S1][0] = name;
        frame.save_ra(c, 0x60);
        c.g[A1][0] = sp(c) as u64;
        c.g[A0][0] = abi::arg64(c, 2);
        call_ptr(c, 0x003f_a1d0, 0x0026_61fc)?;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = 0x4a0000;
        c.g[S0][0] = sx(p(sp(c), 0x10));
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(0x004a_33e8);
        call_ptr(c, 0x0041_7828, 0x0026_6218)?;
        c.g[A0][0] = Addr(gp(c, 0x2e8)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x002c_6ce0, 0x0026_6224)?;
        c.g[V0][0] = 0xc;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore_ra(c, 0x60);
        Addr(gp(c, 0x2e4)).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `Font_DrawTextCenteredF(font, fmt, ap, x, y)` 0x3929f8: format and draw a
/// centred string.
pub fn font_draw_text_centered_f(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        let font = abi::arg64(c, 0);
        let fmt = abi::arg64(c, 1);
        let ap = abi::arg64(c, 2);
        c.g[V0][0] = Addr(gp(c, 0xda0)).lw(c);
        frame.save(c, S0, 0x20);
        c.swc1(21, p(sp(c), 0x38));
        c.g[S0][0] = font;
        c.swc1(20, p(sp(c), 0x30));
        c.g[A0][0] = sp(c) as u64;
        frame.save_ra(c, 0x10);
        c.mov_s(20, 12);
        c.mov_s(21, 13);
        c.sw(sp(c), c.g[V0][0]);
        c.g[A1][0] = fmt;
        c.g[A2][0] = ap;
        call_ptr(c, 0x0031_91c0, 0x0039_2a28)?;
        c.g[A1][0] = Addr(sp(c)).lw(c);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_2908, 0x0039_2a3c)?;
        c.g[A0][0] = sp(c) as u64;
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0039_2a48)?;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        c.lwc1(21, p(sp(c), 0x38));
        c.lwc1(20, p(sp(c), 0x30));
        frame.close(c);
        Ok(())
    })
}

/// `Font_DrawTextRightF(font, fmt, ap, x, y)` 0x392b40: format and draw a
/// right-aligned string.
pub fn font_draw_text_right_f(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        let font = abi::arg64(c, 0);
        let fmt = abi::arg64(c, 1);
        let ap = abi::arg64(c, 2);
        c.g[V0][0] = Addr(gp(c, 0xda0)).lw(c);
        frame.save(c, S0, 0x20);
        c.swc1(21, p(sp(c), 0x38));
        c.g[S0][0] = font;
        c.swc1(20, p(sp(c), 0x30));
        c.g[A0][0] = sp(c) as u64;
        frame.save_ra(c, 0x10);
        c.mov_s(20, 12);
        c.mov_s(21, 13);
        c.sw(sp(c), c.g[V0][0]);
        c.g[A1][0] = fmt;
        c.g[A2][0] = ap;
        call_ptr(c, 0x0031_91c0, 0x0039_2b70)?;
        c.g[A1][0] = Addr(sp(c)).lw(c);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_2a60, 0x0039_2b84)?;
        c.g[A0][0] = sp(c) as u64;
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0039_2b90)?;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        c.lwc1(21, p(sp(c), 0x38));
        c.lwc1(20, p(sp(c), 0x30));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end destructors, modifiers and screen initialisation.

/// `cFEStateChallengeReceivedPopup_dtor(this, flags)` 0x1e0a50: restore the
/// base vtable, destroy the received-challenge popup's `+0x20` object, release
/// its string and run the base destructor.
pub fn c_fe_state_challenge_received_popup_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        c.g[V0][0] = sx(0x0046_8d20);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = this64;
        frame.save_ra(c, 0);
        c.g[S1][0] = abi::arg64(c, 1);
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        let obj = u32_at(c, this, 0x20);
        c.g[A2][0] = sx(obj);
        let vt = u32_at(c, obj, 8);
        c.g[V0][0] = sx(vt);
        c.g[A0][0] = Addr(p(vt, 0x100)).lh(c);
        c.g[V1][0] = lw_at(c, vt, 0x104);
        let func = r32(c, V1);
        c.g[A0][0] = sx(obj.wrapping_add(r32(c, A0)));
        call_ptr(c, func, 0x001e_0a8c)?;
        c.g[A0][0] = sx(p(this, 0x6d0));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001e_0a98)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001a_85d0, 0x001e_0aa4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetCreateRoomPopup_dtor(this, flags)` 0x1df878: restore the base
/// vtable, release the three strings and run the base destructor.
pub fn c_fe_state_net_create_room_popup_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        c.g[V0][0] = sx(0x0046_a6e8);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = this64;
        frame.save_ra(c, 0);
        c.g[S1][0] = abi::arg64(c, 1);
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        for (off, ret) in [
            (0x6e4u32, 0x001d_f8a8u32),
            (0x6e0, 0x001d_f8b4),
            (0x6dc, 0x001d_f8c0),
        ] {
            c.g[A0][0] = sx(p(this, off));
            c.g[A1][0] = 2;
            call_ptr(c, 0x0031_8260, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001a_85d0, 0x001d_f8cc)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateCreateStep1_dtor(this, flags)` 0x1e0cf8: restore the base vtable,
/// release the three account strings and run the base destructor.
pub fn c_fe_state_create_step1_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        c.g[V0][0] = sx(0x0046_86c0);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = this64;
        frame.save_ra(c, 0);
        c.g[S1][0] = abi::arg64(c, 1);
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx(p(this, 0x6dc));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001e_0d28)?;
        c.g[A0][0] = sx(p(this, 0x6d8));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001e_0d34)?;
        c.g[A0][0] = sx(p(this, 0x6d4));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001e_0d40)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001a_85d0, 0x001e_0d4c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RollerModifier_Save(mod, obj)` 0x35e1e0: serialize the modifier body and
/// forward `obj`'s two save hooks (the packed 0x28-byte record and the current
/// value).
pub fn roller_modifier_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg64(c, 0);
        let obj64 = abi::arg64(c, 1);
        let obj = obj64 as u32;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = obj64;
        c.g[S1][0] = self_;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(self_ as u32, 0x30));
        call_ptr(c, 0x0035_da00, 0x0035_e200)?;
        c.g[V0][0] = Addr(obj).lw(c);
        c.g[A1][0] = sx(p(self_ as u32, 4));
        c.g[A2][0] = 0x28;
        let vt = r32(c, V0);
        c.g[A0][0] = Addr(p(vt, 8)).lh(c);
        c.g[V1][0] = lw_at(c, vt, 0xc);
        let func = r32(c, V1);
        c.g[A0][0] = sx(obj.wrapping_add(r32(c, A0)));
        call_ptr(c, func, 0x0035_e21c)?;
        c.g[V0][0] = Addr(obj).lw(c);
        c.g[A1][0] = lw_at(c, self_ as u32, 0xd0);
        let vt = r32(c, V0);
        c.g[A0][0] = Addr(p(vt, 0x28)).lh(c);
        c.g[V1][0] = lw_at(c, vt, 0x2c);
        let func = r32(c, V1);
        c.g[A0][0] = sx(obj.wrapping_add(r32(c, A0)));
        call_ptr(c, func, 0x0035_e234)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SplineCameraAlgorithm_dtor(this, flags)` 0x161370: restore the base
/// vtable, free the three spline segments and run the camera dtor.
pub fn spline_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = this64;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x0045_b968);
        c.g[S1][0] = abi::arg64(c, 1);
        Addr(p(this, 0x10)).sw(c, c.g[V0][0]);
        for (off, ret) in [
            (0x3a8u32, 0x0016_13a0u32),
            (0x39c, 0x0016_13ac),
            (0x390, 0x0016_13b8),
        ] {
            c.g[A0][0] = sx(p(this, off));
            c.g[A1][0] = 2;
            call_ptr(c, 0x0031_d618, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0016_2458, 0x0016_13c4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEPopupConfirm_BaseDtor(this, flags)` 0x1c5a90: restore the base vtable,
/// release the two label strings and the popup configuration, then run the UI
/// state destructor.
pub fn c_fe_popup_confirm_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        c.g[V0][0] = sx(0x0046_cbd8);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = this64;
        frame.save_ra(c, 0);
        c.g[S1][0] = abi::arg64(c, 1);
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx(p(this, 0x2b4));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001c_5ac0)?;
        c.g[A0][0] = sx(p(this, 0x2b0));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001c_5acc)?;
        c.g[A0][0] = sx(p(this, 0xbc));
        c.g[A1][0] = 2;
        call_ptr(c, 0x001c_5750, 0x001c_5ad8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_e390, 0x001c_5ae4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ChaseCamController_ClearOverride(cam)` 0x1620d0: clear the override flag
/// (bit 1 of `+0x30`), pick the algorithm mode (from `+0x24` when the current
/// one is 0x5d), reset the collision flag and blend to the mode.
pub fn chase_cam_controller_clear_override(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        c.g[V0][0] = sx(0xffff_fffe);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = this64;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x5d;
        let mut mode = u32_at(c, this, 0x2c);
        c.g[S1][0] = sx(mode);
        let flags = u32_at(c, this, 0x30) & (c.g[V0][0] as u32);
        c.g[V1][0] = sx(flags);
        c.g[S1][0] = sx(mode);
        Addr(p(this, 0x30)).sw(c, c.g[V1][0]);
        if mode == 0x5d {
            c.cycles += 1;
            mode = u32_at(c, this, 0x24);
            c.g[S1][0] = sx(mode);
        }
        c.cycles += 13;
        call_ptr(c, 0x0015_dac0, 0x0016_210c)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 12);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0;
        call_ptr(c, 0x0015_d078, 0x0016_2124)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateBonusMaterial_Init(this)` 0x1954d8: add the bonus-material screen
/// and play its first frame.
pub fn c_fe_state_bonus_material_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        frame.save(c, S0, 0x20);
        c.g[S0][0] = this64;
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx(0x0046_0168);
        c.g[S1][0] = lw_at(c, this, 0x10);
        call_ptr(c, 0x0031_7670, 0x0019_54fc)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0019_5510)?;
        let screen = c.g[V0][0];
        Addr(p(this, 0x40)).sw(c, screen);
        if screen != 0 {
            c.cycles += 4;
            c.g[A0][0] = screen;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0019_5528)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `FEScreenSelMulti_Init(this)` 0x1a3090: add the multi-rider selection
/// screen and play its first frame.
pub fn fe_screen_sel_multi_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        frame.save(c, S0, 0x20);
        c.g[S0][0] = this64;
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx(0x0046_13b8);
        c.g[S1][0] = lw_at(c, this, 0x10);
        call_ptr(c, 0x0031_7670, 0x001a_30b4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001a_30c8)?;
        let screen = c.g[V0][0];
        Addr(p(this, 0x40)).sw(c, screen);
        if screen != 0 {
            c.cycles += 4;
            c.g[A0][0] = screen;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001a_30e0)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetQuickMessages_Init(this)` 0x1c2138: add the quick-messages
/// screen, play its first frame and clear the message list.
pub fn c_fe_state_net_quick_messages_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        frame.save(c, S1, 0x10);
        c.g[S1][0] = this64;
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx(0x0046_5b20);
        c.g[S0][0] = lw_at(c, this, 0x10);
        call_ptr(c, 0x0031_7670, 0x001c_215c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001c_2170)?;
        let screen = c.g[V0][0];
        Addr(p(this, 0x40)).sw(c, screen);
        if screen != 0 {
            c.cycles += 4;
            c.g[A0][0] = screen;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001c_2188)?;
        }
        c.cycles += 6;
        Addr(p(this, 0x6d0)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `CrowdAudio_InstanceStopSound(self)` 0x2a6848: clamp the instance volume
/// to 0.25, clear the playing flag and, when a sound is live, stop the bank
/// monitor handle.
pub fn crowd_audio_instance_stop_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        c.g[AT][0] = 0x3e80_0000;
        c.mtc1(1, 1);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(this);
        c.lwc1(0, p(this, 0x188));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, p(this, 0x188));
        }
        c.cycles += 3;
        let mut handle = 0u64;
        c.g[A1][0] = lw_at(c, this, 0x174);
        handle = c.g[A1][0];
        Addr(p(this, 0x180)).sw(c, 0);
        if (handle as i64) >= 0 {
            c.cycles += 8;
            c.g[V0][0] = Addr(gp(c, 0x608)).lw(c);
            c.g[A2][0] = 2;
            c.mtc1(0, 12);
            c.g[A0][0] = Addr(p(r32(c, V0), 0)).lw(c);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1d8));
            call_ptr(c, 0x002a_d5f0, 0x002a_6898)?;
            c.g[V0][0] = u64::MAX;
            Addr(p(this, 0x174)).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The three `InSeq_*TrackStop` bodies: release the track object through the
/// sequence's heap and clear the track's eight fields when an owner exists.
fn in_seq_track_stop(c: &mut Cpu, field: u32, clear: &[u32], ret: u32) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        let owner = lw_at(c, this, 0xc);
        c.g[V0][0] = owner;
        if owner == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[A1][0] = lw_at(c, this, field);
            let track = c.g[A1][0];
            if track == 0 {
                Addr(p(this, field + 8)).sw(c, 0);
            } else {
                c.cycles += 5;
                c.g[V0][0] = lw_at(c, owner as u32, 8);
                c.g[A0][0] = Addr(p(r32(c, V0), 8)).lw(c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(8));
                call_ptr(c, 0x0027_23a8, ret)?;
                Addr(p(this, field + 8)).sw(c, 0);
            }
            c.cycles += 8;
            for &off in clear {
                Addr(p(this, off)).sw(c, 0);
            }
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `InSeq_SpanTrackStop(self)` 0x272ec0: release the span track at `+0x20`.
pub fn in_seq_span_track_stop(c: &mut Cpu) {
    in_seq_track_stop(
        c,
        0x20,
        &[0x8, 0xc, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28],
        0x0027_2ef8,
    )
}

/// `InSeq_EventTrackStop(self)` 0x2732c8: release the event track at `+0x1c`.
pub fn in_seq_event_track_stop(c: &mut Cpu) {
    in_seq_track_stop(
        c,
        0x1c,
        &[0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20],
        0x0027_3300,
    )
}

/// `InSeq_RampTrackStop(self)` 0x273630: release the ramp track at `+0x1c`.
pub fn in_seq_ramp_track_stop(c: &mut Cpu) {
    in_seq_track_stop(
        c,
        0x1c,
        &[0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20],
        0x0027_3668,
    )
}

// ---------------------------------------------------------------------------
// Replay, audio, INI, mission scan, lobby and widget hooks.

/// `TexFlipModifier_ApplyFrame(mod)` 0x35f3a8: for each entry, copy the
/// current flip table's 16-bit value into the modifier's output record.
pub fn tex_flip_modifier_apply_frame(c: &mut Cpu) {
    c.cycles += 4;
    let this = abi::arg(c, 0);
    c.g[A2][0] = sx(this);
    c.g[V0][0] = lw_at(c, this, 0x1c);
    c.g[A1][0] = 0;
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 2;
        c.g[A3][0] = sx(p(this, 0x48));
        loop {
            c.cycles += 8;
            c.g[V1][0] = lw_at(c, this, 0x18);
            c.g[A0][0] = sx(r32(c, A1) << 2);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x94)).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = Addr(p(r32(c, V0), 4)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, V1), 0x10)).lw(c);
            if c.g[V1][0] == 0 {
                c.g[V0][0] = lw_at(c, this, 0x1c);
            } else {
                c.cycles += 6;
                c.g[V0][0] = lw_at(c, this, 0x14);
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[V1][0] = Addr(p(r32(c, V0), 4)).lhu(c);
                Addr(r32(c, A3)).sh(c, c.g[V1][0]);
                c.g[V0][0] = lw_at(c, this, 0x1c);
            }
            c.cycles += 4;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(0x14));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_f3c0);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `LobbyMan_GetRoomUserName(lobby, idx, outFlag)` 0x25f1e0: the indexed room
/// user's name, or the empty string; the flag receives the "in use" bit.
pub fn lobby_man_get_room_user_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = abi::arg(c, 0);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(this).lw(c);
        let present = c.g[V0][0] != 0;
        c.g[S0][0] = abi::arg64(c, 2);
        if present {
            c.cycles += 5;
            c.g[A0][0] = lw_at(c, this, 0x60);
            call_ptr(c, 0x003e_e560, 0x0025_f208)?;
            let p = c.g[V0][0];
            c.g[V1][0] = p;
            if p == 0 {
                c.g[V0][0] = 0x4a0000;
                c.cycles += 1;
                c.g[V0][0] = sx(0x004a_2ed0);
            } else {
                // The +2 is charged before the out-flag test, so it applies to
                // both non-empty paths.
                c.cycles += 2;
                if c.g[S0][0] == 0 {
                    c.g[V0][0] = sx((p as u32).wrapping_add(8));
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = lw_at(c, p as u32, 4);
                    c.g[V0][0] = sx(((r32(c, V0) as i32) >> 21) as u32);
                    c.g[V0][0] &= 1;
                    Addr(r32(c, S0)).sw(c, c.g[V0][0]);
                    c.g[V0][0] = sx((p as u32).wrapping_add(8));
                }
            }
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

/// `SSXAudioSystem_IsTrackGroup0Event(self)` 0x2a4078: true when the BE
/// interface's current track is in group 0, or when the game state is 0 or 5.
pub fn ssx_audio_system_is_track_group0_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        call_ptr(c, 0x0014_dc80, 0x002a_408c)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x002a_4098)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_4bc0, 0x002a_40a0)?;
        c.g[A1][0] = Addr(r32(c, V0)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002a_40e0, 0x002a_40ac)?;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = 1;
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x530000;
            c.g[V0][0] = Addr(0x0053_5c10).lb(c);
            let was_zero = c.g[V0][0] == 0;
            c.g[V0][0] ^= 5;
            if was_zero {
                c.cycles += 1;
                c.g[V0][0] = 1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `IniParser_AtSectionEnd(parser)` 0x2a7f90: at the end of a section when the
/// cursor has not reached the limit; true when the next character is NUL or
/// `[`.
pub fn ini_parser_at_section_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        c.g[V0][0] = lw_at(c, this, 8);
        c.g[V1][0] = lw_at(c, this, 4);
        c.g[A0][0] = Addr(this).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 1;
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_78f8, 0x002a_7fc4)?;
            c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).lb(c);
            let zero = c.g[A0][0] == 0;
            c.g[A1][0] = 0;
            if zero {
                c.cycles += 2;
                c.g[A1][0] = 1;
                c.g[V0][0] = 1;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x5b;
                let not_bracket = c.g[A0][0] != c.g[V0][0];
                c.g[V0][0] = c.g[A1][0];
                if !not_bracket {
                    c.cycles += 2;
                    c.g[A1][0] = 1;
                    c.g[V0][0] = 1;
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Player_AllocReplayCache(player)` 0x270070: allocate the first empty replay
/// cache slot (two at `+0x48c`), or return 0 when both are taken.
pub fn player_alloc_replay_cache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0x480000;
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(p(this, 0x48c));
        loop {
            let slot = r32(c, S0);
            c.cycles += 3;
            c.g[V0][0] = Addr(slot).lw(c);
            let taken = c.g[V0][0] != 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            if !taken {
                c.cycles += 9;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x17b0));
                c.g[A0][0] = 0x10;
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0027_00a8)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0026_d0a0, 0x0027_00b0)?;
                let cache = c.g[V0][0];
                Addr(slot).sw(c, cache);
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 0;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_0088);
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Replay_CountHighlights(replay, bit, positive)` 0x270bd8: count the nodes
/// whose value bit is set and whose value has the requested sign.
pub fn replay_count_highlights(c: &mut Cpu) {
    c.cycles += 3;
    let this = abi::arg(c, 0);
    let bit = abi::arg(c, 1);
    let positive = abi::arg64(c, 2);
    c.g[A3][0] = lw_at(c, this, 0x3b0);
    c.g[T0][0] = 0;
    if c.g[A3][0] != 0 {
        c.cycles += 3;
        c.g[T1][0] = sx(bit << 2);
        c.g[T2][0] = u64::MAX;
        c.g[V0][0] = Addr(p(r32(c, A3), 0x1e)).lbu(c);
        loop {
            c.cycles += 4;
            c.g[V0][0] = sx(((r32(c, V0) as i32) >> (bit & 31)) as u32);
            c.g[V0][0] &= 1;
            if c.g[V0][0] == 0 {
                c.g[A3][0] = Addr(p(r32(c, A3), 0x14)).lw(c);
            } else {
                c.g[V1][0] = sx(r32(c, A3).wrapping_add(r32(c, T1)));
                if positive == 0 {
                    c.cycles += 2;
                    c.cycles += 5;
                    c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, T1)));
                    c.g[A0][0] = sx(r32(c, T0).wrapping_add(1));
                    c.g[V1][0] = Addr(p(r32(c, V0), 0x20)).lw(c);
                    c.g[V1][0] = ((c.g[T2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V1][0] == 0 {
                        c.g[T0][0] = c.g[A0][0];
                    }
                } else {
                    c.cycles += 2;
                    c.cycles += 4;
                    c.g[V0][0] = Addr(p(r32(c, V1), 0x20)).lw(c);
                    c.g[V0][0] = ((0i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[T0][0] = sx(r32(c, V0).wrapping_add(r32(c, T0)));
                }
                c.cycles += 1;
                c.g[A3][0] = Addr(p(r32(c, A3), 0x14)).lw(c);
            }
            c.cycles += 2;
            if c.g[A3][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr(p(r32(c, A3), 0x1e)).lbu(c);
            if event_due(c) {
                deliver_event_here(c, 0x0027_0bf0);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T0][0];
    abi::ret(c);
}

/// The memory-card manager state hooks: poll the movie object at `+0x434` and,
/// when it is idle, whether `fn` at `self + fn_off` reports busy and then set
/// the load state `state` on the `+0x748` object.
fn mc_manager_state(c: &mut Cpu, fn_off: u32, state: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        let obj = u32_at(c, this, 0x434);
        c.g[A1][0] = sx(obj);
        let vt = u32_at(c, obj, 0);
        c.g[V1][0] = sx(vt);
        c.g[A0][0] = Addr(p(vt, 0x58)).lh(c);
        c.g[V0][0] = lw_at(c, vt, 0x5c);
        let func = r32(c, V0);
        c.g[A0][0] = sx(obj.wrapping_add(r32(c, A0)));
        call_ptr(c, func, ret)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 5;
            c.g[V0][0] = lw_at(c, this, fn_off);
            let func = r32(c, V0);
            call_ptr(c, func, ret + 0x14)?;
            let busy = c.g[V0][0] != 0;
            c.g[A1][0] = state;
            if busy {
                c.cycles += 5;
                c.g[V0][0] = lw_at(c, this, 0x748);
                let vt2 = r32(c, V0);
                c.g[A0][0] = Addr(p(vt2, 8)).lh(c);
                c.g[V1][0] = lw_at(c, vt2, 0xc);
                let func = r32(c, V1);
                c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
                call_ptr(c, func, ret + 0x30)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MCManager_State37(self)` 0x23e3c0.
pub fn mc_manager_state37(c: &mut Cpu) {
    mc_manager_state(c, 0x6c, 3, 0x0023_e3e8)
}

/// `MCManager_State1B(self)` 0x240960.
pub fn mc_manager_state1b(c: &mut Cpu) {
    mc_manager_state(c, 0x30, 6, 0x0024_0988)
}

/// `cGameLoadState_OnWidgetCreated(self, widget)` 0x245598: tell the BE
/// interface about the widget, then run the QP or MP setup hook depending on
/// the game-mode byte at 0x535c11.
pub fn c_game_load_state_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        frame.save_ra(c, 0);
        let widget = abi::arg64(c, 1);
        c.g[S1][0] = widget;
        call_ptr(c, 0x0014_dc80, 0x0024_55b4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0024_55c0)?;
        c.g[V1][0] = 0x530000;
        c.g[V0][0] = Addr(0x0053_5c11).lb(c);
        c.g[V0][0] ^= 2;
        c.g[A0][0] = c.g[S0][0];
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0024_6278, 0x0024_55ec)?;
        } else {
            c.cycles += 4;
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0024_64e8, 0x0024_55dc)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptMan_IsMissionType2Active(man)` 0x30b8c0: walk the node list at
/// `+0x2b8` and report whether any active node has mission type 2.
pub fn w_script_man_is_mission_type2_active(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        let this = abi::arg(c, 0);
        c.g[S0][0] = lw_at(c, this, 0x2b8);
        c.g[S1][0] = 0;
        if c.g[S0][0] != 0 {
            c.cycles += 1;
            c.g[V1][0] = Addr(p(r32(c, S0), 0x5c)).lw(c);
            loop {
                c.cycles += 6;
                let vt = r32(c, V1);
                c.g[A0][0] = Addr(p(vt, 0x18)).lh(c);
                c.g[V0][0] = lw_at(c, vt, 0x1c);
                let func = r32(c, V0);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                call_ptr(c, func, 0x0030_b8f0)?;
                let active = c.g[V0][0] != 0;
                c.g[V1][0] = sx(r32(c, S1).wrapping_add(1));
                if active {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(r32(c, S0), 0x60)).lw(c);
                    c.g[V0][0] ^= 2;
                    if c.g[V0][0] == 0 {
                        c.g[S1][0] = c.g[V1][0];
                    }
                }
                c.cycles += 3;
                c.g[S0][0] = Addr(p(r32(c, S0), 0x18)).lw(c);
                if c.g[S0][0] == 0 {
                    break;
                }
                c.g[V1][0] = Addr(p(r32(c, S0), 0x5c)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0030_b8e0);
                }
            }
        }
        c.cycles += 6;
        c.g[V0][0] = (0u64 < c.g[S1][0]) as u64;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ModObj_v28(self)` 0x3560c0: forward to the held object's slot +0xb8,
/// falling back to the modifier's own slot +0xd8 when that returns 0.
pub fn mod_obj_v28(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        c.g[V1][0] = lw_at(c, this, 0x1c);
        c.g[A2][0] = 0;
        if c.g[V1][0] != 0 {
            c.cycles += 1;
            c.g[A2][0] = Addr(r32(c, V1)).lw(c);
        }
        c.cycles += 2;
        let mut v1;
        if c.g[A2][0] == 0 {
            v1 = lw_at(c, this, 0xc);
            c.g[V1][0] = v1;
        } else {
            c.cycles += 7;
            let obj = r32(c, A2);
            v1 = lw_at(c, obj, 0);
            c.g[V1][0] = v1;
            let vt = v1 as u32;
            c.g[A0][0] = Addr(p(vt, 0xb8)).lh(c);
            c.g[V0][0] = lw_at(c, vt, 0xbc);
            let func = r32(c, V0);
            c.g[A0][0] = sx(obj.wrapping_add(r32(c, A0)));
            call_ptr(c, func, 0x0035_60fc)?;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                frame.restore(c, S0, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 1;
            v1 = lw_at(c, this, 0xc);
            c.g[V1][0] = v1;
        }
        c.cycles += 5;
        let vt = v1 as u32;
        c.g[A0][0] = Addr(p(vt, 0xd8)).lh(c);
        c.g[V0][0] = lw_at(c, vt, 0xdc);
        let func = r32(c, V0);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
        call_ptr(c, func, 0x0035_6118)?;
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_Update(menu)` 0x17d688: tick the two lobby sub-systems, update
/// the top of the menu stack when present, and run the menu's state machine.
pub fn lobby_menu_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(gp(c, -0x238)).lw(c);
        call_ptr(c, 0x0025_5a20, 0x0017_d6a0)?;
        c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
        call_ptr(c, 0x0025_ac50, 0x0017_d6a8)?;
        c.g[V0][0] = lw_at(c, this, 8);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        if empty {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 4;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(this.wrapping_add(r32(c, V0)));
            c.g[V0][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = sx(p(this, 8));
            call_ptr(c, 0x002c_c0c0, 0x0017_d6d8)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0017_db18, 0x0017_d6e0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_CreatePersona(menu, arg)` 0x17e1c0: drain the persona list, then
/// enter state 8 and ask the lobby layer to create the persona.
pub fn lobby_menu_create_persona(c: &mut Cpu) {
    lobby_menu_drain(c, 8, 0x0017_e1f0, 0x0017_e1e8, 0x0017_e210, 0x0025_d1b8)
}

/// `LobbyMenu_DeletePersona(menu, arg)` 0x17e228: drain the list, enter state
/// 9 and ask the lobby layer to delete the persona.
pub fn lobby_menu_delete_persona(c: &mut Cpu) {
    lobby_menu_drain(c, 9, 0x0017_e258, 0x0017_e250, 0x0017_e278, 0x0025_d428)
}

/// `LobbyMenu_SelectPersona(menu, arg)` 0x17e290: drain the list, enter state
/// 7 and ask the lobby layer to select the persona.
pub fn lobby_menu_select_persona(c: &mut Cpu) {
    lobby_menu_drain(c, 7, 0x0017_e2c0, 0x0017_e2b8, 0x0017_e2e0, 0x0025_cd28)
}

/// The shared `list = &self->list; while (*list) pop(list); state = n; ...`
/// body of the three lobby persona hooks.
fn lobby_menu_drain(c: &mut Cpu, state: u32, loop_ret: u32, head: u32, ret: u32, call: u32) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        let this = abi::arg64(c, 0);
        let arg = abi::arg64(c, 1);
        c.g[S1][0] = this;
        frame.save_ra(c, 0);
        c.g[S2][0] = arg;
        c.g[V0][0] = lw_at(c, this as u32, 8);
        c.g[S0][0] = sx(p(this as u32, 8));
        if c.g[V0][0] != 0 {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_c460, loop_ret)?;
                c.g[V0][0] = Addr(p(r32(c, S0), 0)).lw(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, head);
                }
            }
        }
        c.cycles += 11;
        c.g[V0][0] = sx(state);
        c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
        Addr(p(this as u32, 4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, call, ret)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_DeclineChallenge(menu)` 0x17e4f0: drain the list, clear the
/// pending challenge, enter state 12 and send the decline message.
pub fn lobby_menu_decline_challenge(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        c.g[V0][0] = lw_at(c, this, 8);
        c.g[S0][0] = sx(p(this, 8));
        if c.g[V0][0] != 0 {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_c460, 0x0017_e518)?;
                c.g[V0][0] = Addr(p(r32(c, S0), 0)).lw(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_e510);
                }
            }
        }
        c.cycles += 13;
        c.g[A0][0] = sx(p(this, 8));
        c.g[A1][0] = sx(p(this, 0x1044));
        call_ptr(c, 0x002c_c3b8, 0x0017_e530)?;
        c.g[A1][0] = lw_at(c, this, 0x1c0c);
        c.g[V0][0] = 0xc;
        c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
        Addr(p(this, 4)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0025_fc28, 0x0017_e544)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `FEScreenSelMulti_OnWidgetCreated(self, item)` 0x1a30f8: map the widget's
/// id to the player slot (0 or 1) or leave it untouched.
pub fn fe_screen_sel_multi_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        let item = abi::arg(c, 1);
        c.g[A0][0] = 0x4a0000;
        frame.save(c, S1, 0x10);
        c.g[A0][0] = sx(0x004a_14f8);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(item);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = lw_at(c, item, 0x38);
        call_ptr(c, 0x0031_7670, 0x001a_311c)?;
        let first = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x4a0000;
        let mut matched_second = false;
        if !first {
            c.cycles += 2;
            Addr(p(item, 0x18)).sw(c, 0);
        } else {
            c.cycles += 5;
            c.g[S0][0] = lw_at(c, item, 0x38);
            c.g[A0][0] = sx(0x004a_1500);
            call_ptr(c, 0x0031_7670, 0x001a_3138)?;
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = 1;
                Addr(p(item, 0x18)).sw(c, c.g[V0][0]);
                matched_second = true;
            }
        }
        if !first || matched_second {
            c.cycles += 1;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateAuthDNASError_OnWidgetAction(self, a1, msg)` 0x1b6e40: 5/6 pick the
/// error dialog's option 0xf, 7/8 do nothing, everything else forwards to the
/// net state's widget action.
pub fn c_fe_state_auth_dnas_error_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        let a1 = abi::arg64(c, 1);
        let msg = abi::arg64(c, 2);
        if msg < 5 {
            c.cycles += 3;
            c.g[A0][0] = this;
            c.g[A1][0] = a1;
            c.g[A2][0] = msg;
            call_ptr(c, 0x001a_8918, 0x001b_6e9c)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            if msg < 7 {
                let obj = u32_at(c, this as u32, 0x20);
                c.g[A3][0] = sx(obj);
                c.cycles += 9;
                c.g[A1][0] = this;
                c.g[A2][0] = 0xf;
                let vt = u32_at(c, obj, 8);
                c.g[V0][0] = sx(vt);
                c.g[A0][0] = Addr(p(vt, 0x88)).lh(c);
                c.g[V1][0] = lw_at(c, vt, 0x8c);
                let func = r32(c, V1);
                c.g[A0][0] = sx(obj.wrapping_add(r32(c, A0)));
                call_ptr(c, func, 0x001b_6e8c)?;
                frame.restore_ra(c, 0);
            } else if msg < 9 {
                c.cycles += 3;
                c.cycles += 2;
                c.g[V0][0] = 1;
                frame.restore_ra(c, 0);
            } else {
                c.cycles += 3;
                c.cycles += 3;
                c.g[A0][0] = this;
                c.g[A1][0] = a1;
                c.g[A2][0] = msg;
                call_ptr(c, 0x001a_8918, 0x001b_6e9c)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateCreateStep1_ValidateFields(self)` 0x1ba110: validate account name,
/// password and e-mail in that order, stopping at the first failure.
pub fn c_fe_state_create_step1_validate_fields(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        call_ptr(c, 0x001b_9d68, 0x001b_a128)?;
        c.g[S1][0] = 1;
        let name_ok = c.g[V0][0] != 0;
        c.g[V1][0] = 0;
        if name_ok {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001b_9fa0, 0x001b_a13c)?;
            c.g[V1][0] = 0;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[S1][0];
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[V1][0];
        let password_ok = c.g[V0][0] != 0;
        c.g[V1][0] = 0;
        if password_ok {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001b_9b18, 0x001b_a158)?;
            c.g[V1][0] = 0;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[S1][0];
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = c.g[V1][0];
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateCreateStep2_GetDaysInMonth(self, month, year)` 0x1bb9b0: days in
/// the month, 29 for a leap February.
pub fn c_fe_state_create_step2_get_days_in_month(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let month = abi::arg(c, 1);
        frame.save_ra(c, 0);
        c.g[V0][0] = 2;
        if month == 2 {
            c.cycles += 6;
            c.g[A1][0] = abi::arg64(c, 2);
            call_ptr(c, 0x001b_b968, 0x001b_b9fc)?;
            c.g[A0][0] = 0x1d;
            c.g[V1][0] = 0x1c;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[A0][0];
            }
            c.g[V0][0] = c.g[V1][0];
        } else {
            c.cycles += 13;
            let mut m = c.g[A1][0];
            c.g[V1][0] = ((m as i64) < 8) as u64;
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(1));
            if c.g[V1][0] == 0 {
                m = c.g[V0][0];
                c.g[A1][0] = m;
            }
            c.g[A0][0] = 0x1f;
            c.g[V1][0] = sx(r32(c, A1) >> 31);
            c.g[V0][0] = 0x1e;
            c.g[V1][0] = sx((m as u32).wrapping_add(r32(c, V1)));
            c.g[V1][0] = sx(((r32(c, V1) as i32) >> 1) as u32);
            c.g[V1][0] = sx(r32(c, V1) << 1);
            c.g[V1][0] = sx((m as u32).wrapping_sub(r32(c, V1)));
            c.g[V1][0] ^= 1;
            if c.g[V1][0] == 0 {
                c.g[V0][0] = c.g[A0][0];
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AV_Audio_dtor(self, flags)` 0x3aee30: close the stream, release the
/// sub-object and optionally delete the audio object.
pub fn av_audio_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = abi::arg64(c, 0);
        let this32 = this as u32;
        frame.save_ra(c, 0);
        c.g[S0][0] = this;
        c.g[V0][0] = lw_at(c, this32, 4);
        let absent = c.g[V0][0] == 0;
        c.g[S1][0] = abi::arg64(c, 1);
        if !absent {
            c.cycles += 7;
            c.g[A0][0] = lw_at(c, this32, 0xc);
            call_ptr(c, 0x003b_7838, 0x003a_ee58)?;
            c.g[V1][0] = 0x510000;
            c.g[V0][0] = Addr(0x0050_9434).lw(c);
            let func = r32(c, V0);
            c.g[A0][0] = lw_at(c, this32, 4);
            call_ptr(c, func, 0x003a_ee68)?;
            Addr(p(this32, 4)).sw(c, 0);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        let keep = c.g[V0][0] == 0;
        c.g[V1][0] = 0x510000;
        if !keep {
            c.cycles += 3;
            c.g[V0][0] = Addr(0x0050_9434).lw(c);
            let func = r32(c, V0);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, func, 0x003a_ee84)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cSetupMan_SendBragMsg12(self)` 0x265700: send a brag message (kind 12 when
/// a brag is pending, else 13), entering state 7 first when in state 9.
pub fn c_setup_man_send_brag_msg12(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x30);
        let this = abi::arg(c, 0);
        c.g[V1][0] = Addr(this).lw(c);
        c.g[V0][0] = 0xc;
        let state_12 = c.g[V1][0] == c.g[V0][0];
        frame.save_ra(c, 0x20);
        if !state_12 {
            c.cycles += 3;
            c.g[V0][0] = lw_at(c, this, 0x124);
            let no_pending = c.g[V0][0] == 0;
            c.g[V0][0] = 9;
            if no_pending {
                c.cycles += 1;
                c.g[A1][0] = 0xd;
            } else {
                c.cycles += 2;
                let not_nine = c.g[V1][0] != c.g[V0][0];
                c.g[A1][0] = 0xc;
                if not_nine {
                    // fall through to the send block with a1 = 12
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 7;
                    Addr(this).sw(c, c.g[V0][0]);
                }
            }
            c.cycles += 9;
            c.g[V1][0] = Addr(this).lw(c);
            c.g[V0][0] = u64::MAX;
            c.sw(sp(c), c.g[A1][0]);
            c.sw(p(sp(c), 4), c.g[V1][0]);
            c.g[A1][0] = sp(c) as u64;
            c.sw(p(sp(c), 8), c.g[V0][0]);
            c.g[A2][0] = 0x14;
            call_ptr(c, 0x0026_4be8, 0x0026_575c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, DATA, GAME_GP};
    use ssx3_ee as ee;

    const OBJ: u32 = DATA + 0x1000;
    const OBJ2: u32 = DATA + 0x2000;
    const OBJ3: u32 = DATA + 0x3000;
    const OBJ4: u32 = DATA + 0x4000;
    const VT: u32 = DATA + 0x8000;
    const BE_LIBRARY: u32 = DATA + 0x1_f000;

    fn w(c: &mut Cpu, at: u32, v: u32) {
        c.bus.write32(at, v);
    }

    /// A vtable slot `{ short delta; short; fn }` at `vt + off`.
    fn slot(c: &mut Cpu, vt: u32, off: u32, delta: i16, func: u32) {
        c.bus.write16(vt + off, delta as u16);
        w(c, vt + off + 4, func);
    }

    /// The BELibrary singleton at `gp - 0x1f04` and the world at `gp - 0x848`
    /// (world -> +0x84 -> +0xc race, rider count at +0x7c).
    fn globals(c: &mut Cpu) {
        w(c, GAME_GP - 0x1f04, BE_LIBRARY);
        let world = OBJ4;
        w(c, GAME_GP - 0x848, world);
        let game = world + 0x1000;
        w(c, world + 0x84, game);
        let race = world + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }

    #[test]
    fn c_fe_state_net_messenger_ctor_matches() {
        check(
            ee::s03::f_001a3dc8__generated,
            c_fe_state_net_messenger_ctor,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x1a3dc8, 0);
                globals(c);
                w(c, GAME_GP + 0xda0, 0x1234_5678);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { OBJ2 as u64 };
                c.g[A2][0] = if seed % 3 == 0 { 0 } else { OBJ3 as u64 };
            },
        );
    }

    #[test]
    fn net_event102_tooy_clone_matches() {
        check(
            ee::s07::f_00268240__generated,
            net_event102_tooy_clone,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x268240, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                w(c, OBJ + 0xc, if seed % 3 == 0 { 0 } else { 0x102 });
            },
        );
    }

    #[test]
    fn rider_script_obj_deactivate_matches() {
        check(
            ee::s07::f_002803f0__generated,
            rider_script_obj_deactivate,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x2803f0, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                w(c, OBJ + 0xc, if seed % 2 == 0 { 0 } else { 1 });
                // The identity-transform block the copy reads.
                for k in 0..4u32 {
                    w(c, 0x004f_f1a0 + k * 0x10, 0x3f80_0000);
                }
            },
        );
    }

    #[test]
    fn menu_sys_draw_matches() {
        check(ee::s09::f_002cc350__generated, menu_sys_draw, 30, &|c, seed| {
            fakes::machine(c, seed, 0x2cc350, 0);
            globals(c);
            let obj = OBJ2;
            w(c, GAME_GP - 0x3ff4, 0x3f00_0000);
            w(c, OBJ + 0x58, obj);
            w(c, OBJ, if seed % 2 == 0 { 1 } else { 2 });
            w(c, OBJ + 4, OBJ3);
            w(c, obj + 0x30, 0x4000_0000);
            w(c, obj + 0x34, 0x4040_0000);
            w(c, OBJ3 + 0x12c, VT);
            slot(c, VT, 0x28, 0, fakes::FAKE);
            c.g[A0][0] = OBJ as u64;
            c.f[12] = 0x3f80_0000;
            c.f[13] = 0x4000_0000;
        });
    }

    #[test]
    fn dynamic_emitter_load_matches() {
        check(ee::s12::f_00371318__generated, dynamic_emitter_load, 30, &|c, seed| {
            fakes::machine(c, seed, 0x371318, 0);
            globals(c);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = OBJ2 as u64;
            w(c, OBJ2, VT);
            slot(c, VT, 0x10, 0, fakes::FAKE);
            w(c, OBJ2 + 0x100, seed);
        });
    }

    #[test]
    fn c_setup_man_create_matches() {
        check(ee::s07::f_00264030__generated, c_setup_man_create, 30, &|c, seed| {
            fakes::machine(c, seed, 0x264030, 0);
            globals(c);
            pool_allocator(c);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { OBJ2 as u64 };
            c.g[A2][0] = if seed % 3 == 0 { 0 } else { OBJ3 as u64 };
        });
    }

    #[test]
    fn c_ui_state_buy_popup_set_uber_trick_params_matches() {
        check(
            ee::s04::f_001caf58__generated,
            c_ui_state_buy_popup_set_uber_trick_params,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x1caf58, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = seed as u64;
                c.g[A2][0] = (seed as u64) << 8;
                c.g[A3][0] = (seed as u64) << 16;
            },
        );
    }

    #[test]
    fn fe_flow_state_init_matches() {
        check(ee::s03::f_001a0648__generated, fe_flow_state_init, 30, &|c, seed| {
            fakes::machine(c, seed, 0x1a0648, 0);
            globals(c);
            pool_allocator(c);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { 0x24 };
            w(c, OBJ + 4, if seed % 3 == 0 { 0 } else { OBJ2 });
        });
    }

    #[test]
    fn net_fe_state_open_messenger_matches() {
        check(
            ee::s03::f_001a9b58__generated,
            net_fe_state_open_messenger,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x1a9b58, 0);
                globals(c);
                pool_allocator(c);
                c.g[A0][0] = OBJ as u64;
                w(c, OBJ + 0x10, OBJ2);
                w(c, OBJ + 0x40, OBJ3);
                w(c, OBJ3 + 0xd0, OBJ4);
                w(c, OBJ4 + 0x10, OBJ2 + 0x40);
                let _ = seed;
            },
        );
    }

    #[test]
    fn c_fe_state_create_step1_restore_account_info_matches() {
        check(
            ee::s03::f_001ba218__generated,
            c_fe_state_create_step1_restore_account_info,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x1ba218, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                w(c, OBJ + 0x6d0, OBJ2);
                // Same string pointers on both sides: `operator=` is a no-op.
                w(c, OBJ2 + 4, if seed % 2 == 0 { 0 } else { OBJ3 });
                w(c, OBJ2 + 8, OBJ3);
                w(c, OBJ2 + 0xc, 0);
                w(c, OBJ + 0x6d4, 0);
                w(c, OBJ + 0x6d8, OBJ3);
                w(c, OBJ + 0x6dc, 0);
                w(c, OBJ + 0x6e0, 0x11);
                w(c, OBJ + 0x6e4, 0x22);
            },
        );
    }

    #[test]
    fn c_shared_login_man_mc_mkdir_matches() {
        check(
            ee::s07::f_00266170__generated,
            c_shared_login_man_mc_mkdir,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x266170, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = (seed & 7) as u64;
                // Empty name (strcpy stops at the first byte).
                w(c, OBJ2, 0);
                w(c, GAME_GP + 0x2e8, if seed % 2 == 0 { 0 } else { OBJ3 });
            },
        );
    }

    #[test]
    fn c_shared_login_man_mc_delete_file_matches() {
        check(
            ee::s07::f_002661d8__generated,
            c_shared_login_man_mc_delete_file,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x2661d8, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = (seed & 7) as u64;
                w(c, GAME_GP + 0x2e8, if seed % 2 == 0 { 0 } else { OBJ3 });
            },
        );
    }

    #[test]
    fn font_draw_text_centered_f_matches() {
        check(
            ee::s13::f_003929f8__generated,
            font_draw_text_centered_f,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x3929f8, 0);
                globals(c);
                // The empty-string sentinel: the local BXString's destructor then
                // skips the refcount path.
                w(c, GAME_GP + 0xda0, 0x0048_dcd4);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = 0;
                w(c, OBJ2, 0);
                c.f[12] = if seed % 2 == 0 {
                    0x3f80_0000
                } else {
                    0xc000_0000
                };
                c.f[13] = 0x4040_0000;
            },
        );
    }

    #[test]
    fn font_draw_text_right_f_matches() {
        check(
            ee::s13::f_00392b40__generated,
            font_draw_text_right_f,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x392b40, 0);
                globals(c);
                w(c, GAME_GP + 0xda0, 0x0048_dcd4);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = 0;
                w(c, OBJ2, 0);
                c.f[12] = if seed % 2 == 0 {
                    0x3f80_0000
                } else {
                    0xc000_0000
                };
                c.f[13] = 0x4040_0000;
            },
        );
    }

    #[test]
    fn c_fe_state_challenge_received_popup_dtor_matches() {
        check(
            ee::s04::f_001e0a50__generated,
            c_fe_state_challenge_received_popup_dtor,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x1e0a50, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { 1 };
                w(c, OBJ + 0x20, OBJ2);
                w(c, OBJ2 + 8, VT);
                slot(c, VT, 0x100, 0, fakes::FAKE);
                w(c, OBJ + 0x6d0, 0x0048_dcd4);
            },
        );
    }

    #[test]
    fn c_fe_state_net_create_room_popup_dtor_matches() {
        check(
            ee::s04::f_001df878__generated,
            c_fe_state_net_create_room_popup_dtor,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x1df878, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { 1 };
                for off in [0x6e4u32, 0x6e0, 0x6dc] {
                    w(c, OBJ + off, 0x0048_dcd4);
                }
            },
        );
    }

    #[test]
    fn c_fe_state_create_step1_dtor_matches() {
        check(
            ee::s04::f_001e0cf8__generated,
            c_fe_state_create_step1_dtor,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x1e0cf8, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { 1 };
                for off in [0x6dcu32, 0x6d8, 0x6d4] {
                    w(c, OBJ + off, 0x0048_dcd4);
                }
            },
        );
    }

    #[test]
    fn roller_modifier_save_matches() {
        check(ee::s11::f_0035e1e0__generated, roller_modifier_save, 20, &|c, seed| {
            fakes::machine(c, seed, 0x35e1e0, 0);
            globals(c);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = OBJ2 as u64;
            w(c, OBJ2, VT);
            slot(c, VT, 0x8, 0, fakes::FAKE);
            slot(c, VT, 0x28, 0, fakes::FAKE);
            w(c, OBJ + 0xd0, if seed % 2 == 0 { 0 } else { 0x1234 });
        });
    }

    #[test]
    fn spline_camera_algorithm_dtor_matches() {
        check(
            ee::s01::f_00161370__generated,
            spline_camera_algorithm_dtor,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x161370, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { 1 };
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_base_dtor_matches() {
        check(
            ee::s03::f_001c5a90__generated,
            c_fe_popup_confirm_base_dtor,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x1c5a90, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { 1 };
                w(c, OBJ + 0x2b4, 0x0048_dcd4);
                w(c, OBJ + 0x2b0, 0x0048_dcd4);
                w(c, OBJ + 0xbc, 0);
            },
        );
    }

    #[test]
    fn chase_cam_controller_clear_override_matches() {
        check(
            ee::s01::f_001620d0__generated,
            chase_cam_controller_clear_override,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x1620d0, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                // 0x62+ makes `ChaseCamController_SetAlgorithm` return at once.
                if seed % 3 == 0 {
                    w(c, OBJ + 0x2c, 0x5d);
                    w(c, OBJ + 0x24, 0x62 + (seed & 1));
                } else {
                    w(c, OBJ + 0x2c, 0x62 + (seed & 1));
                }
                w(c, OBJ + 0x30, if seed % 2 == 0 { 0 } else { 3 });
            },
        );
    }

    fn init_screen_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 20, &|c, seed| {
            fakes::machine(c, seed, salt, 0);
            globals(c);
            c.g[A0][0] = OBJ as u64;
            w(c, OBJ + 0x10, if seed % 2 == 0 { 0 } else { OBJ2 });
            if seed % 3 == 0 {
                w(c, OBJ + 0x40, 0);
            }
        });
    }

    #[test]
    fn c_fe_state_bonus_material_init_matches() {
        init_screen_test(
            ee::s02::f_001954d8__generated,
            c_fe_state_bonus_material_init,
            0x1954d8,
        );
    }

    #[test]
    fn fe_screen_sel_multi_init_matches() {
        init_screen_test(ee::s03::f_001a3090__generated, fe_screen_sel_multi_init, 0x1a3090);
    }

    #[test]
    fn c_fe_state_net_quick_messages_init_matches() {
        init_screen_test(
            ee::s03::f_001c2138__generated,
            c_fe_state_net_quick_messages_init,
            0x1c2138,
        );
    }

    #[test]
    fn crowd_audio_instance_stop_sound_matches() {
        check(
            ee::s08::f_002a6848__generated,
            crowd_audio_instance_stop_sound,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x2a6848, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                let vol = if seed % 2 == 0 {
                    0x3e80_0000
                } else {
                    0x3f40_0000
                };
                w(c, OBJ + 0x188, vol);
                w(c, OBJ + 0x174, if seed % 3 == 0 { 0xffff_ffff } else { 3 });
                w(c, GAME_GP + 0x608, OBJ2);
                w(c, OBJ2, OBJ3);
            },
        );
    }

    fn in_seq_test(generated: GuestFn, hand: GuestFn, salt: u64, field: u32) {
        check(generated, hand, 20, &|c, seed| {
            fakes::machine(c, seed, salt, 0);
            globals(c);
            c.g[A0][0] = OBJ as u64;
            if seed % 3 == 0 {
                w(c, OBJ + 0xc, 0);
            } else {
                w(c, OBJ + 0xc, OBJ2);
                w(c, OBJ2 + 8, OBJ3);
                w(c, OBJ3 + 8, OBJ4);
                w(c, OBJ4 + 8, 0);
                w(c, OBJ + field, if seed % 2 == 0 { 0 } else { OBJ4 + 0x100 });
            }
        });
    }

    #[test]
    fn in_seq_span_track_stop_matches() {
        in_seq_test(ee::s07::f_00272ec0__generated, in_seq_span_track_stop, 0x272ec0, 0x20);
    }

    #[test]
    fn in_seq_event_track_stop_matches() {
        in_seq_test(ee::s07::f_002732c8__generated, in_seq_event_track_stop, 0x2732c8, 0x1c);
    }

    #[test]
    fn in_seq_ramp_track_stop_matches() {
        in_seq_test(ee::s07::f_00273630__generated, in_seq_ramp_track_stop, 0x273630, 0x1c);
    }

    /// Tiny generated leaves used as deterministic vtable targets.
    const ZERO_STUB: u32 = 0x0020_2768; // `EventSelect_HandleInput`: returns 0
    const ONE_STUB: u32 = 0x0021_cb68; // returns 1

    #[test]
    fn tex_flip_modifier_apply_frame_matches() {
        check(
            ee::s12::f_0035f3a8__generated,
            tex_flip_modifier_apply_frame,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x35f3a8, 0);
                c.g[A0][0] = OBJ as u64;
                let count: u32 = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => 2,
                };
                w(c, OBJ + 0x1c, count);
                w(c, OBJ + 0x18, OBJ2);
                w(c, OBJ + 0x14, 0);
                w(c, OBJ2 + 0x94, OBJ3);
                w(c, OBJ3 + 4, OBJ4);
                w(c, OBJ3 + 8, OBJ2 + 0x200);
                if seed % 2 == 0 {
                    w(c, OBJ4 + 0x10, OBJ3 + 0x400);
                    w(c, OBJ3 + 0x400 + 4, 0x1234);
                } else {
                    w(c, OBJ4 + 0x10, 0);
                    w(c, OBJ2 + 0x200 + 0x10, 0);
                }
            },
        );
    }

    #[test]
    fn lobby_man_get_room_user_name_matches() {
        check(
            ee::s06::f_0025f1e0__generated,
            lobby_man_get_room_user_name,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x25f1e0, 0);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = match seed % 4 {
                    0 => u64::MAX,
                    1 => 0,
                    2 => 1,
                    _ => 2,
                };
                c.g[A2][0] = if seed % 4 == 0 { 0 } else { OBJ2 as u64 };
                w(c, OBJ, if seed % 5 == 0 { 0 } else { 1 });
                w(c, OBJ + 0x60, OBJ3);
                w(c, OBJ3 + 0x18, 2);
                w(c, OBJ3 + 0x40, OBJ4);
                w(c, OBJ4, OBJ2 + 0x100);
                w(c, OBJ2 + 0x100, OBJ2 + 0x300);
                w(c, OBJ2 + 0x300 + 4, 0x0020_0000);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_track_group0_event_matches() {
        check(
            ee::s08::f_002a4078__generated,
            ssx_audio_system_is_track_group0_event,
            40,
            &|c, seed| {
                fakes::machine(c, seed, 0x2a4078, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                // `cBE_getInterface` really indexes the fixed global array at
                // 0x5308a8 with `*(gp - 0x1efc)`; point slot 0 at our object.
                w(c, GAME_GP - 0x1efc, 0);
                w(c, 0x0053_08a8, OBJ2);
                w(c, OBJ2 + 4, VT);
                slot(c, VT, 8, 0, ZERO_STUB);
                w(c, 0x0053_5c08, OBJ3.wrapping_sub(0x0044_26b0));
                w(
                    c,
                    OBJ3,
                    match seed % 3 {
                        0 => 0,
                        1 => 3,
                        _ => 6,
                    },
                );
                w(
                    c,
                    0x0053_5c10,
                    match seed % 4 {
                        0 => 0,
                        1 => 5,
                        2 => 3,
                        _ => 7,
                    },
                );
            },
        );
    }

    #[test]
    fn ini_parser_at_section_end_matches() {
        check(
            ee::s08::f_002a7f90__generated,
            ini_parser_at_section_end,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x2a7f90, 0);
                c.g[A0][0] = OBJ as u64;
                let (limit, start, end, ch) = match seed % 3 {
                    0 => (0u32, 0u32, 0u32, 0u8),
                    1 => (0x7fff_ffff, 0, OBJ + 0x100, 0),
                    _ => (0x7fff_ffff, 0, OBJ + 0x100, b'['),
                };
                w(c, OBJ, limit);
                w(c, OBJ + 4, start);
                w(c, OBJ + 8, end);
                c.bus.write8(OBJ + 0x100, ch);
            },
        );
    }

    #[test]
    fn player_alloc_replay_cache_matches() {
        check(
            ee::s07::f_00270070__generated,
            player_alloc_replay_cache,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x270070, 0);
                globals(c);
                pool_allocator(c);
                c.g[A0][0] = OBJ as u64;
                let k = seed % 3;
                w(c, OBJ + 0x48c, if k == 0 { 0 } else { OBJ2 });
                w(c, OBJ + 0x490, if k == 2 { OBJ3 } else { 0 });
            },
        );
    }

    #[test]
    fn replay_count_highlights_matches() {
        check(
            ee::s07::f_00270bd8__generated,
            replay_count_highlights,
            40,
            &|c, seed| {
                fakes::machine(c, seed, 0x270bd8, 0);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 2) as u64;
                if seed % 5 == 0 {
                    w(c, OBJ + 0x3b0, 0);
                } else {
                    w(c, OBJ + 0x3b0, OBJ2);
                    w(c, OBJ2 + 0x1e, if seed % 3 == 0 { 1 } else { 0xff });
                    w(c, OBJ2 + 0x14, if seed % 4 == 0 { 0 } else { OBJ3 });
                    w(c, OBJ3 + 0x1e, 0xff);
                    w(c, OBJ3 + 0x14, 0);
                    for i in 0..4u32 {
                        w(
                            c,
                            OBJ2 + 0x20 + 4 * i,
                            if seed % 3 == 0 { 0xffff_ffff } else { 1 },
                        );
                        w(c, OBJ3 + 0x20 + 4 * i, 0x8000_0000);
                    }
                }
            },
        );
    }

    fn mc_state_test(generated: GuestFn, hand: GuestFn, salt: u64, fn_off: u32) {
        check(generated, hand, 30, &|c, seed| {
            fakes::machine(c, seed, salt, 0);
            globals(c);
            c.g[A0][0] = OBJ as u64;
            w(c, OBJ + 0x434, OBJ2);
            w(c, OBJ2, VT);
            // The movie object's virtual "idle" query.
            slot(
                c,
                VT,
                0x58,
                0,
                if seed % 3 == 0 { ONE_STUB } else { ZERO_STUB },
            );
            // The state's "busy" function pointer.
            w(
                c,
                OBJ + fn_off,
                if seed % 2 == 0 { ZERO_STUB } else { ONE_STUB },
            );
            w(c, OBJ + 0x748, VT + 0x100);
            slot(c, VT + 0x100, 8, 0, fakes::FAKE);
        });
    }

    #[test]
    fn mc_manager_state37_matches() {
        mc_state_test(ee::s06::f_0023e3c0__generated, mc_manager_state37, 0x23e3c0, 0x6c);
    }

    #[test]
    fn mc_manager_state1b_matches() {
        mc_state_test(ee::s06::f_00240960__generated, mc_manager_state1b, 0x240960, 0x30);
    }

    #[test]
    fn av_audio_dtor_matches() {
        check(ee::s13::f_003aee30__generated, av_audio_dtor, 30, &|c, seed| {
            fakes::machine(c, seed, 0x3aee30, 0);
            globals(c);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = if seed % 3 == 0 { 1 } else { 0 };
            w(c, OBJ + 4, if seed % 2 == 0 { 0 } else { OBJ2 });
            w(c, OBJ + 0xc, OBJ3);
            w(
                c,
                0x0050_9434,
                if seed % 4 == 0 {
                    ZERO_STUB
                } else {
                    fakes::FAKE
                },
            );
        });
    }

    #[test]
    fn c_game_load_state_on_widget_created_matches() {
        check(
            ee::s06::f_00245598__generated,
            c_game_load_state_on_widget_created,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x245598, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                w(c, OBJ + 8, VT);
                slot(c, VT, 0x48, 0, fakes::FAKE);
                w(c, 0x0053_5c11, if seed % 2 == 0 { 2 } else { 0 });
                w(c, OBJ + 0x38, 0x1234);
                w(c, OBJ2 + 0x38, 0x5678);
            },
        );
    }

    #[test]
    fn w_script_man_is_mission_type2_active_matches() {
        check(
            ee::s10::f_0030b8c0__generated,
            w_script_man_is_mission_type2_active,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x30b8c0, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                if seed % 4 == 0 {
                    w(c, OBJ + 0x2b8, 0);
                } else {
                    w(c, OBJ + 0x2b8, OBJ2);
                    w(c, OBJ2 + 0x5c, VT);
                    slot(
                        c,
                        VT,
                        0x18,
                        0,
                        if seed % 2 == 0 { ONE_STUB } else { ZERO_STUB },
                    );
                    w(c, OBJ2 + 0x60, if seed % 3 == 0 { 2 } else { 3 });
                    let more = seed % 4 == 1;
                    w(c, OBJ2 + 0x18, if more { OBJ3 } else { 0 });
                    w(c, OBJ3 + 0x5c, VT);
                    slot(c, VT + 0x40, 0x18, 0, ONE_STUB);
                    w(c, OBJ3 + 0x60, 2);
                    w(c, OBJ3 + 0x18, 0);
                }
            },
        );
    }

    #[test]
    fn mod_obj_v28_matches() {
        check(ee::s11::f_003560c0__generated, mod_obj_v28, 30, &|c, seed| {
            fakes::machine(c, seed, 0x3560c0, 0);
            globals(c);
            c.g[A0][0] = OBJ as u64;
            if seed % 3 == 0 {
                w(c, OBJ + 0x1c, 0);
            } else {
                w(c, OBJ + 0x1c, OBJ2);
                w(c, OBJ2, OBJ3);
                w(c, OBJ3, VT);
                slot(
                    c,
                    VT,
                    0xb8,
                    0,
                    if seed % 2 == 0 { ONE_STUB } else { ZERO_STUB },
                );
            }
            w(c, OBJ + 0xc, OBJ4);
            w(c, OBJ4, VT + 0x100);
            slot(c, VT + 0x100, 0xd8, 0, fakes::FAKE);
        });
    }

    #[test]
    fn lobby_menu_update_matches() {
        check(ee::s02::f_0017d688__generated, lobby_menu_update, 20, &|c, seed| {
            fakes::machine(c, seed, 0x17d688, 0);
            globals(c);
            c.g[A0][0] = OBJ as u64;
            w(c, OBJ + 8, (seed % 3) as u32);
            w(c, OBJ + 0xc, if seed % 2 == 0 { 0 } else { 1 });
            w(c, OBJ + 4, 0x19);
        });
    }

    fn lobby_persona_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 20, &|c, seed| {
            fakes::machine(c, seed, salt, 0);
            globals(c);
            // Draining the list and `LobbyMan_SelectPersona` build strings and
            // tag fields, so the engine allocator has to be usable.
            pool_allocator(c);
            heap_init(c);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { (seed & 7) as u64 };
            if seed % 2 == 0 {
                w(c, OBJ + 8, 0);
            } else {
                w(c, OBJ + 8, 1);
                w(c, OBJ + 0xc, OBJ2);
                w(c, OBJ2 + 0x12c, VT);
                slot(c, VT, 0x38, 0, fakes::FAKE);
            }
        });
    }

    #[test]
    fn lobby_menu_create_persona_matches() {
        lobby_persona_test(ee::s02::f_0017e1c0__generated, lobby_menu_create_persona, 0x17e1c0);
    }

    #[test]
    fn lobby_menu_delete_persona_matches() {
        lobby_persona_test(ee::s02::f_0017e228__generated, lobby_menu_delete_persona, 0x17e228);
    }

    #[test]
    fn lobby_menu_select_persona_matches() {
        lobby_persona_test(ee::s02::f_0017e290__generated, lobby_menu_select_persona, 0x17e290);
    }

    #[test]
    fn lobby_menu_decline_challenge_matches() {
        check(
            ee::s02::f_0017e4f0__generated,
            lobby_menu_decline_challenge,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x17e4f0, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                if seed % 2 == 0 {
                    w(c, OBJ + 8, 0);
                } else {
                    w(c, OBJ + 8, 1);
                    w(c, OBJ + 0xc, OBJ2);
                    w(c, OBJ2 + 0x12c, VT);
                    slot(c, VT, 0x38, 0, fakes::FAKE);
                }
                w(c, OBJ + 0x1044, 0);
                w(c, OBJ + 0x1c0c, 0);
            },
        );
    }

    #[test]
    fn fe_screen_sel_multi_on_widget_created_matches() {
        check(
            ee::s03::f_001a30f8__generated,
            fe_screen_sel_multi_on_widget_created,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x1a30f8, 0);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                // "a" hashes to 0x61, "b" to 0x62.
                c.bus.write8(0x004a_14f8, b'a');
                c.bus.write8(0x004a_14f9, 0);
                c.bus.write8(0x004a_1500, b'b');
                c.bus.write8(0x004a_1501, 0);
                w(
                    c,
                    OBJ2 + 0x38,
                    match seed % 3 {
                        0 => 0x61,
                        1 => 0x62,
                        _ => 0x99,
                    },
                );
                w(c, OBJ2 + 0x18, 0x55);
            },
        );
    }

    #[test]
    fn c_fe_state_auth_dnas_error_on_widget_action_matches() {
        check(
            ee::s03::f_001b6e40__generated,
            c_fe_state_auth_dnas_error_on_widget_action,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x1b6e40, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = match seed % 4 {
                    0 => 0,
                    1 => 5,
                    2 => 7,
                    _ => 10,
                };
                w(c, OBJ + 0x20, OBJ3);
                w(c, OBJ3 + 8, VT);
                slot(c, VT, 0x88, 0, fakes::FAKE);
            },
        );
    }

    #[test]
    fn c_fe_state_create_step1_validate_fields_matches() {
        check(
            ee::s03::f_001ba110__generated,
            c_fe_state_create_step1_validate_fields,
            10,
            &|c, seed| {
                fakes::machine(c, seed, 0x1ba110, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                w(c, OBJ + 0x6d0, if seed % 2 == 0 { 0 } else { OBJ2 });
                w(c, OBJ2 + 4, 0);
                w(c, OBJ2 + 8, 0);
                w(c, OBJ2 + 0xc, 0);
            },
        );
    }

    #[test]
    fn c_fe_state_create_step2_get_days_in_month_matches() {
        check(
            ee::s03::f_001bb9b0__generated,
            c_fe_state_create_step2_get_days_in_month,
            40,
            &|c, seed| {
                fakes::machine(c, seed, 0x1bb9b0, 0);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = match seed % 8 {
                    0 => 2,
                    1 => 1,
                    2 => 7,
                    3 => 8,
                    4 => 12,
                    5 => 13,
                    6 => 0,
                    _ => 0xffff_ffff,
                };
                c.g[A2][0] = match seed % 4 {
                    0 => 2000,
                    1 => 1900,
                    2 => 2023,
                    _ => 2024,
                };
            },
        );
    }

    #[test]
    fn c_setup_man_send_brag_msg12_matches() {
        check(
            ee::s07::f_00265700__generated,
            c_setup_man_send_brag_msg12,
            10,
            &|c, seed| {
                fakes::machine(c, seed, 0x265700, 0);
                globals(c);
                pool_allocator(c);
                c.g[A0][0] = OBJ as u64;
                w(
                    c,
                    OBJ,
                    match seed % 3 {
                        0 => 12,
                        1 => 9,
                        _ => 3,
                    },
                );
                w(c, OBJ + 0x124, if seed % 2 == 0 { 0 } else { 1 });
                w(c, OBJ + 0x90, 0);
                w(c, OBJ + 0x120, 0);
            },
        );
    }

    /// Run one generated function as a setup step, with `args` in a0..t1.
    fn run(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, SENTINEL_RA, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// The main heap's dlmalloc arena over 256 KiB of test RAM (oc69).
    const MEM_HEAP: u32 = 0x004f_f250;
    const SENTINEL_RA: u32 = 0x0012_3450;

    fn heap_init(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        run(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x40000) as u64, 0x40000],
        );
    }

    /// The engine's size-class pool allocator (`gp + 0xdd8` and the free lists
    /// at `gp + 0x2a60..0x2a84`), copied from `oc57`/`oc69`.
    fn pool_allocator(c: &mut Cpu) {
        w(c, GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x32000),
            (0x2a7c, 0x25800),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        let nodes = DATA + 0x28_000;
        for (i, (slot_off, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes + 0x100 * i as u32;
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                w(c, node, if n == 3 { 0 } else { node + 0x40 });
            }
            w(c, pool + head, base);
            w(c, GAME_GP + slot_off, pool);
        }
    }
}
