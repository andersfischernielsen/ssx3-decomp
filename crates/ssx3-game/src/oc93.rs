//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for small front-end, net, audio, replay, camera and codec
//! functions, each with a differential test against the generated body.
//!
//! The bodies are transcribed from the generated Rust (the authority for
//! registers, cycles and call order). Calls go through [`crate::call`]:
//! fixed targets through the function table with the original return
//! address, which also reaches a replaced callee and the fake hooks the
//! differential tests install. Structured loops make the original's
//! back-edge event checks with [`crate::events::event_due`] and
//! [`crate::events::deliver_event_here`].

use crate::call::{call_ptr, guest, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0038_adb0, irradiance_free, "Irradiance_Free"),
    (0x001a_bfa8, fe_screen_blank_init, "FEScreenBlank_Init"),
    (
        0x001a_d9c8,
        cfe_state_select_persona_v33,
        "cFEStateSelectPersona_v33",
    ),
    (
        0x001b_6b50,
        cfe_state_auth_dnas_error_init,
        "cFEStateAuthDNASError_Init",
    ),
    (
        0x001b_7bc0,
        cfe_state_net_account_mem_update_state16,
        "cFEStateNetAccountMem_UpdateState16",
    ),
    (
        0x001b_9610,
        cfe_state_create_step1_set_password_text,
        "cFEStateCreateStep1_SetPasswordText",
    ),
    (
        0x001b_9678,
        cfe_state_create_step1_set_confirm_password_text,
        "cFEStateCreateStep1_SetConfirmPasswordText",
    ),
    (
        0x001b_dcf8,
        cfe_state_lobby_menu_on_create_room_popup_event,
        "cFEStateLobbyMenu_OnCreateRoomPopupEvent",
    ),
    (
        0x001b_e828,
        cfe_state_net_create_room_popup_init,
        "cFEStateNetCreateRoomPopup_Init",
    ),
    (
        0x001e_5248,
        cov_state_view_popup_init,
        "cOVState_ViewPopup_Init",
    ),
    (
        0x001e_55d0,
        cov_state_view_popup_set_subject,
        "cOVState_ViewPopup_SetSubject",
    ),
    (
        0x0036_2340,
        intc_table_vif1_handler,
        "IntcTable_Vif1Handler",
    ),
    (0x0036_7078, texman_reset_vram, "TexMan_ResetVram"),
    (
        0x0025_cfe0,
        lobbyman_query_own_stats,
        "LobbyMan_QueryOwnStats",
    ),
    (0x0026_67e8, c_usb_keyboard_ctor, "cUsbKeyboard_ctor"),
    (0x0026_8688, net_event_fb_clone, "NetEventFB_Clone"),
    (
        0x0029_ce70,
        ssx_audio_system_resume_sfx,
        "SSXAudioSystem_ResumeSfx",
    ),
    (
        0x002a_72d8,
        crowd_audio_remove_source,
        "CrowdAudio_RemoveSource",
    ),
    (
        0x002a_7340,
        crowd_audio_set_source_param,
        "CrowdAudio_SetSourceParam",
    ),
    (0x002a_8070, ini_parser_get_float, "IniParser_GetFloat"),
    (0x002a_80d8, ini_parser_get_int, "IniParser_GetInt"),
    (
        0x0027_8ed0,
        script_engine_list_add_movie_by_id,
        "ScriptEngine_ListAddMovieById",
    ),
    (
        0x002b_3bc0,
        music_player_set_event_state,
        "MusicPlayer_SetEventState",
    ),
    (
        0x0027_1620,
        replay_save_compute_padding,
        "Replay_SaveComputePadding",
    ),
    (
        0x0027_1688,
        replay_save_write_padding,
        "Replay_SaveWritePadding",
    ),
    (
        0x0027_4d08,
        in_seq_instance_get_remaining_steps,
        "InSeq_InstanceGetRemainingSteps",
    ),
    (0x0039_1c48, font_draw_text_f, "Font_DrawTextF"),
    (
        0x0013_6100,
        rider_spin_sub_get_spin_hud_blend,
        "RiderSpinSub_GetSpinHUDBlend",
    ),
    (0x002c_69a0, cps2_mem_card_do_open, "cPS2MemCard_DoOpen"),
    (
        0x0016_19a8,
        chase_cam_controller_ctor,
        "ChaseCamController_ctor",
    ),
    (0x0037_2660, rail_tube_dtor, "RailTube_dtor"),
    (0x0037_cc30, ps2_graphics_man_v60, "PS2GraphicsMan_v60"),
    (
        0x0024_1cd8,
        mc_manager_poll_card_status,
        "MCManager_PollCardStatus",
    ),
    (
        0x001c_ac30,
        cui_state_buy_popup_init,
        "cUIStateBuyPopup_Init",
    ),
    (0x001c_b138, keyboard_popup_dtor, "cKeyboardPopup_dtor"),
    (0x001c_b1a0, keyboard_popup_init, "cKeyboardPopup_Init"),
    (
        0x001c_ce98,
        keyboard_popup_mask_text,
        "cKeyboardPopup_MaskText",
    ),
    (
        0x001c_d020,
        keyboard_popup_set_static_text_id,
        "cKeyboardPopup_SetStaticTextId",
    ),
    (
        0x001c_e6f0,
        cfe_state_trophy_room_count_event_entries,
        "cFEStateTrophyRoom_CountEventEntries",
    ),
    (
        0x003b_08e0,
        ps2_sony_codec_internal_free_frame,
        "PS2_SONY_CODEC_INTERNAL_FreeFrame",
    ),
    (0x003b_1340, mpeg2_flush_buffer32, "MPEG2_Flush_Buffer32"),
    (0x0024_4410, vfx_process_dtor, "VfxProcess_dtor"),
    (
        0x0024_5628,
        c_game_load_state_out_lodge_init,
        "cGameLoadStateOutLodge_Init",
    ),
    (
        0x0017_3678,
        bounded_camera_algorithm_base_reset,
        "BoundedCameraAlgorithmBase_Reset",
    ),
    (
        0x0017_47a0,
        bounded_camera_algorithm_reset,
        "BoundedCameraAlgorithm_Reset",
    ),
    (
        0x0017_8520,
        superpipe_chase_camera_algorithm_ctor,
        "SuperpipeChaseCameraAlgorithm_ctor",
    ),
    (
        0x0030_b388,
        wscript_process_list_find_mission_by_id,
        "WScriptProcessList_FindMissionById",
    ),
    (0x0035_7210, parent_modifier_v22, "ParentModifier_v22"),
    (0x0035_7278, parent_modifier_save, "ParentModifier_Save"),
    (
        0x0035_7b90,
        multi_particle_man_on_section_unload,
        "MultiParticleMan_OnSectionUnload",
    ),
];

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
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// The shared "init screen by hash" body of the front-end state/overlay
/// constructors: hash the name, look the screen up in the engine, store it at
/// `+0x40` and play its frame when it was created.
fn ui_init_screen(c: &mut Cpu, base: u32, tag: u32) -> Flow {
    c.cycles += 16;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save_ra(c, 0);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    c.g[A0][0] = sx(tag);
    c.g[S1][0] = Addr(p(c.g[S0][0] as u32, 0x10)).lw(c);
    call_ptr(c, 0x0031_7670, base + 0x24)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[A3][0] = 0;
    call_ptr(c, 0x0039_7cc0, base + 0x38)?;
    let screen = c.g[V0][0];
    Addr(p(c.g[S0][0] as u32, 0x40)).sw(c, screen);
    if screen != 0 {
        c.cycles += 4;
        c.g[A0][0] = screen;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        call_ptr(c, 0x0039_c870, base + 0x50)?;
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// One of the two password-text setters: hash the label, find the UI text
/// object on the created screen and mask the string stored at `self + 0x5c`
/// into it.
fn set_password_text(c: &mut Cpu, base: u32, tag: u32, list_off: u32) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save_ra(c, 0);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    c.g[A0][0] = sx(tag);
    call_ptr(c, 0x0031_7670, base + 0x20)?;
    c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, base + 0x2c)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    if c.g[S1][0] != 0 {
        c.cycles += 6;
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x6d0)).lw(c);
        c.g[A1][0] = Addr(p(c.g[V0][0] as u32, list_off)).lw(c);
        call_ptr(c, 0x001a_98b8, base + 0x44)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = sx(p(c.g[S0][0] as u32, 0x5c));
        call_ptr(c, 0x003a_0c50, base + 0x50)?;
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// Render / FE.

/// `void Irradiance_Free(Irradiance* self)` (0x38adb0): release the two
/// allocation lists and clear the owner word.
pub fn irradiance_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let base = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(p(base, 4)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr(p(base, 0xc)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x0038_ade0)?;
            }
            c.g[A0][0] = Addr(p(base, 8)).lw(c);
            c.cycles += 2;
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x0038_adf4)?;
            }
            Addr(p(base, 4)).sw(c, 0);
            c.cycles += 3;
            Addr(p(base, 0xc)).sw(c, 0);
            Addr(p(base, 8)).sw(c, 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FEScreenBlank_Init(void* self)` (0x1abfa8).
pub fn fe_screen_blank_init(c: &mut Cpu) {
    guest(c, |c| ui_init_screen(c, 0x001a_bfa8, 0x004a_1a78))
}

/// `void cFEStateSelectPersona_v33(void* self, void* a1, int a2)` (0x1ad9c8):
/// on the "create" child event ask the lobby manager for the persona list,
/// then forward the event to the net front-end state.
pub fn cfe_state_select_persona_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0xf;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        if c.g[A2][0] == c.g[V0][0] {
            c.cycles += 9;
            c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
            c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x6d4)).lw(c);
            call_ptr(c, 0x0025_d428, 0x001a_d9f4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 0xf;
            call_ptr(c, 0x001a_97b8, 0x001a_da04)?;
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x001a_97b8, 0x001a_da18)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateAuthDNASError_Init(void* self)` (0x1b6b50).
pub fn cfe_state_auth_dnas_error_init(c: &mut Cpu) {
    guest(c, |c| ui_init_screen(c, 0x001b_6b50, 0x0046_42b0))
}

/// `void cFEStateNetAccountMem_UpdateState16(void* self)` (0x1b7bc0):
/// 0x10 clears the account strings and moves to state 0x18; 0x11 moves to
/// 0x17.
pub fn cfe_state_net_account_mem_update_state16(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(gp(c, 0x2e0)).lw(c);
        c.g[V1][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[V0][0] = c.g[V1][0] ^ 0x10;
        c.g[S0][0] = c.g[A0][0];
        if c.g[V0][0] == 0 {
            c.cycles += 7;
            call_ptr(c, 0x001b_70a8, 0x001b_7be8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x18;
            call_ptr(c, 0x001b_70d0, 0x001b_7bf4)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] ^ 0x11;
            if c.g[V0][0] == 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x17;
                call_ptr(c, 0x001b_70d0, 0x001b_7c14)?;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCreateStep1_SetPasswordText(void* self)` (0x1b9610).
pub fn cfe_state_create_step1_set_password_text(c: &mut Cpu) {
    guest(c, |c| set_password_text(c, 0x001b_9610, 0x0046_2ba0, 4))
}

/// `void cFEStateCreateStep1_SetConfirmPasswordText(void* self)` (0x1b9678).
pub fn cfe_state_create_step1_set_confirm_password_text(c: &mut Cpu) {
    guest(c, |c| set_password_text(c, 0x001b_9678, 0x0046_47f8, 8))
}

/// `void cFEStateLobbyMenu_OnCreateRoomPopupEvent(void* self, void* a1, int a2)`
/// (0x1bdcf8).
pub fn cfe_state_lobby_menu_on_create_room_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0xf;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        if c.g[A2][0] == c.g[V0][0] {
            c.cycles += 9;
            c.g[A2][0] = 0xf;
            call_ptr(c, 0x001a_97b8, 0x001b_dd20)?;
            c.g[A3][0] = Addr(p(c.g[S0][0] as u32, 0x6e4)).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x6dc)).lw(c);
            c.g[A2][0] = Addr(p(c.g[S0][0] as u32, 0x6e0)).lw(c);
            call_ptr(c, 0x001b_d610, 0x001b_dd34)?;
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x001a_97b8, 0x001b_dd48)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateNetCreateRoomPopup_Init(void* self)` (0x1be828).
pub fn cfe_state_net_create_room_popup_init(c: &mut Cpu) {
    guest(c, |c| ui_init_screen(c, 0x001b_e828, 0x0046_51e0))
}

/// `void cOVState_ViewPopup_Init(void* self)` (0x1e5248).
pub fn cov_state_view_popup_init(c: &mut Cpu) {
    guest(c, |c| ui_init_screen(c, 0x001e_5248, 0x0046_e258))
}

/// `void cOVState_ViewPopup_SetSubject(void* self, int idx)` (0x1e55d0): either
/// create the variant subject object or set a text's Unicode string id from
/// the table entry.
pub fn cov_state_view_popup_set_subject(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[A3][0] = c.g[A1][0];
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[A1][0] = 0x18;
        c.g[V0][0] = 0x004c_0000;
        c.mult(0, 5, 7, 5);
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6c08));
        c.g[A2][0] = c.g[A0][0];
        c.g[A0][0] = 0x1;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[V1][0] = Addr(p(c.g[A1][0] as u32, 8)).lw(c);
        if c.g[V1][0] == c.g[A0][0] {
            c.cycles += 5;
            c.g[A0][0] = c.g[A2][0];
            c.g[A1][0] = c.g[A3][0];
            call_ptr(c, 0x001e_5638, 0x001e_5610)?;
        } else {
            c.g[A0][0] = Addr(p(c.g[A2][0] as u32, 0x54)).lw(c);
            c.cycles += 2;
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[A1][0] = Addr(p(c.g[A1][0] as u32, 0x10)).lw(c);
                call_ptr(c, 0x003a_1000, 0x001e_5628)?;
            }
        }
        c.cycles += 2;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Interrupts, VRAM and graphics.

/// `int IntcTable_Vif1Handler(void)` (0x362340): save the interrupted
/// registers, bump the VIF1 completion count and acknowledge the VIF1 IRQ.
pub fn intc_table_vif1_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
        c.sd(p(c.g[SP][0] as u32, 0x100), c.g[RA][0]);
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x0036_2120, 0x0036_2350)?;
        c.g[V0][0] = 0x1000_0000;
        c.g[V0][0] = Addr(p(c.g[V0][0] as u32, 0x3c00)).lw(c);
        c.g[V1][0] = 0x0050_0000;
        c.g[V0][0] = c.g[V0][0] & 0x400;
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x59d8));
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[V0][0] = 0x1000_0000;
            c.g[V1][0] = 0x8;
            c.g[V0][0] = c.g[V0][0] | 0x3c10;
            c.sw(c.g[V0][0] as u32, c.g[V1][0]);
        }
        c.cycles += 11;
        c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 0x80)).lw(c);
        c.g[A0][0] = c.g[SP][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        c.sw(p(c.g[A1][0] as u32, 0x80), c.g[V0][0]);
        call_ptr(c, 0x0036_2230, 0x0036_238c)?;
        c.ei();
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x100));
        c.g[V0][0] = 1;
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.pc = ra;
        Ok(())
    })
}

/// `void TexMan_ResetVram(TexMan* self)` (0x367078): free both VRAM arenas and
/// reset the two page tables and the owner word.
pub fn texman_reset_vram(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = sx(p(c.g[S0][0] as u32, 0x4350));
        call_ptr(c, 0x0036_6618, 0x0036_7090)?;
        c.g[A0][0] = sx(p(c.g[S0][0] as u32, 0x1f60));
        call_ptr(c, 0x0036_6618, 0x0036_7098)?;
        c.g[A0][0] = 0xffff_ffff_ffff_ffff;
        c.g[V1][0] = 0x1;
        c.g[V0][0] = sx(p(c.g[S0][0] as u32, 0x1f50));
        loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, c.g[A0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0036_70a8);
                }
                continue;
            }
            c.cycles += 6;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            c.sw(p(c.g[S0][0] as u32, 0x1f54), c.g[V0][0]);
            frame.restore(c, S0, 0x10);
            let ra = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.pc = ra;
            return Ok(());
        }
    })
}

/// `void PS2GraphicsMan_v60(void* self, VEntry* item)` (0x37cc30): unlock the
/// texture's VRAM and notify the texture manager.
pub fn ps2_graphics_man_v60(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = 0xffff_ffff_ffff_ffff;
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 4)).lw(c);
        if c.g[A1][0] != c.g[S2][0] {
            c.cycles += 9;
            c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x18f4)).lw(c);
            call_ptr(c, 0x0036_7b60, 0x0037_cc60)?;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x10d8)).lw(c);
            c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 4)).lw(c);
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x190)).lh(c);
            c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x194)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx(p(c.g[S0][0] as u32, c.g[A0][0] as u32));
            call_ptr(c, target, 0x0037_cc78)?;
            Addr(p(c.g[S1][0] as u32, 4)).sw(c, c.g[S2][0]);
        }
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        c.cycles += 6;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Net / lobby / UI.

/// `void LobbyMan_QueryOwnStats(LobbyMan* self)` (0x25cfe0): format the
/// player's stats into a tag string and queue request 0x1b.
pub fn lobbyman_query_own_stats(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x220));
        c.g[A2][0] = 0x004a_0000;
        c.sq(S0, p(c.g[SP][0] as u32, 0x210));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3108));
        c.sd(p(c.g[SP][0] as u32, 0x200), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x200;
        c.g[A3][0] = Addr(p(c.g[S0][0] as u32, 0x80)).lw(c);
        c.g[RA][0] = sx(0x0025_d00c);
        c.sb(c.g[SP][0] as u32, 0);
        call_ptr(c, 0x003e_c180, 0x0025_d00c)?;
        c.g[T0][0] = 0x0027_0000;
        c.g[A2][0] = 0x6f6e_0000;
        c.g[AT][0] = 0x41a0_0000;
        c.mtc1(AT, 12);
        c.g[A0][0] = c.g[S0][0];
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7340));
        c.g[A1][0] = 0x1b;
        c.g[A2][0] = c.g[A2][0] | 0x6c6e;
        c.g[A3][0] = c.g[SP][0];
        call_ptr(c, 0x0025_b608, 0x0025_d034)?;
        c.lq(S0, p(c.g[SP][0] as u32, 0x210));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x200));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x220));
        c.pc = ra;
        Ok(())
    })
}

/// `void* cUsbKeyboard_ctor(UsbKeyboard* self)` (0x2667e8): zero the status
/// word, register the completion callback and clear the status area.
pub fn c_usb_keyboard_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0x0053_0000;
        frame.save(c, S0, 0x10);
        c.g[A3][0] = 0x0026_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(c.g[S0][0] as u32, 0x460)).sw(c, 0);
        c.g[A2][0] = sx(0x0053_71d0);
        c.g[A3][0] = sx(0x0026_68a8);
        c.g[A0][0] = 0;
        c.g[A1][0] = 0;
        c.g[T0][0] = 0;
        call_ptr(c, 0x003f_61d0, 0x0026_681c)?;
        let failed = (c.g[V0][0] as i64) < 0;
        c.g[V0][0] = 1;
        if !failed {
            c.cycles += 1;
            Addr(p(c.g[S0][0] as u32, 0x460)).sw(c, c.g[V0][0]);
        }
        c.cycles += 9;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x460;
        call_ptr(c, 0x003e_6448, 0x0026_6838)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `NetEventFB* NetEventFB_Clone(NetEventFB* self)` (0x268688): allocate a
/// 16-byte copy and duplicate the two words in the source.
pub fn net_event_fb_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.g[A1][0] = 0x0048_0000;
        c.sq(S0, p(c.g[SP][0] as u32, 0x10));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[S0][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A0][0] = 0x10;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_86b0)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[A2][0] = Addr(p(c.g[S0][0] as u32, 0x0c)).lw(c);
        c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x08)).lw(c);
        c.g[V0][0] = 0xfb;
        c.g[A0][0] = 0x0048_1150;
        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
        c.lq(S0, p(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[V0][0] = c.g[V1][0];
        c.sw(p(c.g[V1][0] as u32, 4), c.g[A0][0]);
        c.sw(p(c.g[V1][0] as u32, 8), c.g[A1][0]);
        c.sw(p(c.g[V1][0] as u32, 0xc), c.g[A2][0]);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

/// `void SSXAudioSystem_ResumeSfx(SSXAudioSystem* self)` (0x29ce70): if a sound
/// was suspended, resume all bank monitors, reset their distances and mark
/// every trigger active again.
pub fn ssx_audio_system_resume_sfx(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x5fb0)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 15;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x118)).lw(c);
            Addr(p(c.g[S0][0] as u32, 0x5fb0)).sw(c, 0);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
            call_ptr(c, 0x002a_d300, 0x0029_cea0)?;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x118)).lw(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            Addr(p(c.g[V1][0] as u32, 0x26c)).sw(c, 0);
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x118)).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
            call_ptr(c, 0x002a_d810, 0x0029_cebc)?;
            c.g[A0][0] = Addr(gp(c, 0x21e4)).lw(c);
            call_ptr(c, 0x002b_6a68, 0x0029_cec4)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CrowdAudio_RemoveSource(CrowdAudio* self, int id)` (0x2a72d8).
pub fn crowd_audio_remove_source(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(S0, p(c.g[SP][0] as u32, 0x20));
        c.swc1(20, p(c.g[SP][0] as u32, 0x30));
        c.g[S0][0] = c.g[A0][0];
        c.sd(p(c.g[SP][0] as u32, 0x10), c.g[RA][0]);
        let negative = (c.g[A1][0] as i64) < 0;
        c.mov_s(20, 12);
        if !negative {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x18)).lw(c);
            let absent = c.g[V0][0] == 0;
            c.g[A2][0] = c.g[SP][0];
            if !absent {
                c.cycles += 10;
                c.g[A3][0] = c.g[SP][0] | 4;
                call_ptr(c, 0x002a_72c0, 0x002a_7308)?;
                c.g[A0][0] = Addr(c.g[SP][0] as u32).lw(c);
                c.g[V0][0] = 0x190;
                c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x18)).lw(c);
                c.mov_s(12, 20);
                c.mult(0, 4, 4, 2);
                c.g[A1][0] = Addr(p(c.g[SP][0] as u32, 4)).lw(c);
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, 0x002a_62f0, 0x002a_7328)?;
            }
        }
        c.cycles += 5;
        c.lq(S0, p(c.g[SP][0] as u32, 0x20));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x10));
        c.lwc1(20, p(c.g[SP][0] as u32, 0x30));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    })
}

/// `void CrowdAudio_SetSourceParam(CrowdAudio* self, int id, int param,
/// float value)` (0x2a7340).
pub fn crowd_audio_set_source_param(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(S0, p(c.g[SP][0] as u32, 0x30));
        c.sq(S1, p(c.g[SP][0] as u32, 0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sd(p(c.g[SP][0] as u32, 0x10), c.g[RA][0]);
        let negative = (c.g[A1][0] as i64) < 0;
        c.g[S1][0] = c.g[A2][0];
        if !negative {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x18)).lw(c);
            let absent = c.g[V0][0] == 0;
            c.g[A2][0] = c.g[SP][0];
            if !absent {
                c.cycles += 10;
                c.g[A3][0] = c.g[SP][0] | 4;
                call_ptr(c, 0x002a_72c0, 0x002a_7370)?;
                c.g[A0][0] = Addr(c.g[SP][0] as u32).lw(c);
                c.g[V0][0] = 0x190;
                c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x18)).lw(c);
                c.g[A2][0] = c.g[S1][0];
                c.mult(0, 4, 4, 2);
                c.g[A1][0] = Addr(p(c.g[SP][0] as u32, 4)).lw(c);
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, 0x002a_5cd8, 0x002a_7390)?;
            }
        }
        c.cycles += 5;
        c.lq(S0, p(c.g[SP][0] as u32, 0x30));
        c.lq(S1, p(c.g[SP][0] as u32, 0x20));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x10));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    })
}

/// `int IniParser_GetFloat(IniParser* self, const char* name, float* out)`
/// (0x2a8070).
pub fn ini_parser_get_float(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        c.g[A0][0] = sx(p(c.g[S0][0] as u32, 0x0c));
        call_ptr(c, 0x0041_aa88, 0x002a_8090)?;
        let mismatch = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !mismatch {
            c.cycles += 6;
            c.g[A1][0] = 0x004a_3740;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_7ef0, 0x002a_80a8)?;
            let bad = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if !bad {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002a_7e60, 0x002a_80b8)?;
                c.swc1(0, c.g[S1][0] as u32);
                c.g[V0][0] = 1;
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

/// `int IniParser_GetInt(IniParser* self, const char* name, int* out)`
/// (0x2a80d8).
pub fn ini_parser_get_int(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        c.g[A0][0] = sx(p(c.g[S0][0] as u32, 0x0c));
        call_ptr(c, 0x0041_aa88, 0x002a_80f8)?;
        let mismatch = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !mismatch {
            c.cycles += 6;
            c.g[A1][0] = 0x004a_3740;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_7ef0, 0x002a_8110)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002a_7ea8, 0x002a_8120)?;
                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                c.g[V0][0] = 1;
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

/// `void ScriptEngine_ListAddMovieById(ScriptEngine* self, int key, ...)`
/// (0x278ed0): scan the movie list for a matching key and add it.
pub fn script_engine_list_add_movie_by_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[T4][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0x52c)).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        let empty = (c.g[V1][0] as i64) <= 0;
        c.g[A2][0] = 0;
        let mut matched = false;
        if !empty {
            c.cycles += 1;
            c.g[T3][0] = 0;
            loop {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0x530)).lw(c);
                c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 4)).lw(c);
                if c.g[V1][0] == c.g[T4][0] {
                    c.cycles += 4;
                    call_ptr(c, 0x0027_8e90, 0x0027_8f0c)?;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    matched = true;
                    break;
                }
                c.g[V1][0] = Addr(p(c.g[A0][0] as u32, 0x52c)).lw(c);
                c.cycles += 5;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x0c));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_8ef0);
                }
            }
        }
        if !matched {
            c.cycles += 1;
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        }
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

/// `void MusicPlayer_SetEventState(MusicPlayer* self, int event)` (0x2b3bc0):
/// notify the current sequence object when the event id changes.
pub fn music_player_set_event_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x418)).lw(c);
        let no_seq = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        let mut equal_case = false;
        if !no_seq {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x41c)).lw(c);
            if c.g[S1][0] != c.g[V0][0] {
                c.cycles += 7;
                c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x5440)).lw(c);
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x18)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x1c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x002b_3c04)?;
                Addr(p(c.g[S0][0] as u32, 0x41c)).sw(c, c.g[S1][0]);
                c.g[V1][0] = 1;
            } else {
                equal_case = true;
            }
        }
        if !equal_case {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 5;
        c.g[V0][0] = c.g[V1][0];
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay.

/// `void Replay_SaveComputePadding(Replay* self)` (0x271620): clear the save
/// buffer, measure the payload and leave the padding count in `+0x67c`.
pub fn replay_save_compute_padding(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0xc800;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x640)).lw(c);
        call_ptr(c, 0x003e_6448, 0x0027_1640)?;
        call_ptr(c, 0x0014_dc80, 0x0027_1648)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_e048, 0x0027_1650)?;
        c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x678)).lw(c);
        c.g[V1][0] = 0x0007_0000;
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x638)).lw(c);
        c.g[V1][0] = c.g[V1][0] | 0xfff8;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        Addr(p(c.g[S0][0] as u32, 0x638)).sw(c, c.g[A0][0]);
        Addr(p(c.g[S0][0] as u32, 0x67c)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

/// `void Replay_SaveWritePadding(Replay* self)` (0x271688): emit at most 0x4000
/// bytes of pending padding, or count an empty write.
pub fn replay_save_write_padding(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(c.g[S1][0] as u32, 0x67c)).lw(c);
        let no_padding = (c.g[V1][0] as i64) <= 0;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 16385) as u64;
        if no_padding {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0x638)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            Addr(p(c.g[S1][0] as u32, 0x638)).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 9;
            c.g[S0][0] = 0x4000;
            if c.g[V0][0] != 0 {
                c.g[S0][0] = c.g[V1][0];
            }
            c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 0x640)).lw(c);
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0027_1058, 0x0027_16bc)?;
            c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0x67c)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S0][0] as u32));
            Addr(p(c.g[S1][0] as u32, 0x67c)).sw(c, c.g[V0][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int InSeq_InstanceGetRemainingSteps(InSeq* self)` (0x274d08): the number
/// of remaining steps, rounded up.
pub fn in_seq_instance_get_remaining_steps(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0027_4c70, 0x0027_4d1c)?;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.lwc1(1, p(c.g[S0][0] as u32, 0x28));
        c.lwc1(2, p(c.g[S0][0] as u32, 0x24));
        c.sub_s(0, 0, 1);
        c.div_s(0, 0, 2);
        c.cvt_w_s(1, 0);
        c.cvt_s_w(1, 1);
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.cycles += 3;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.add_s(1, 1, 0);
        }
        c.cycles += 6;
        c.cvt_w_s(0, 1);
        c.mfc1(2, 0);
        frame.restore(c, S0, 0x10);
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Fonts and HUD.

/// `void Font_DrawTextF(void* font, float x, float y, const char* fmt, ...)`
/// (0x391c48).
pub fn font_draw_text_f(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(S0, p(c.g[SP][0] as u32, 0x20));
        c.sq(S1, p(c.g[SP][0] as u32, 0x10));
        c.g[S0][0] = 0x0050_0000;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x0ca0));
        c.swc1(21, p(c.g[SP][0] as u32, 0x38));
        c.swc1(20, p(c.g[SP][0] as u32, 0x30));
        c.g[S1][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.mov_s(20, 12);
        c.mov_s(21, 13);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0041_86c8, 0x0039_1c7c)?;
        c.g[A0][0] = c.g[S1][0];
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0039_1cb0, 0x0039_1c90)?;
        c.lq(S0, p(c.g[SP][0] as u32, 0x20));
        c.lq(S1, p(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(21, p(c.g[SP][0] as u32, 0x38));
        c.lwc1(20, p(c.g[SP][0] as u32, 0x30));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    })
}

/// `float RiderSpinSub_GetSpinHUDBlend(RiderSpinSub* self)` (0x136100): spin
/// amount in turns, clamped to [0, 1], eased by the square root curve.
pub fn rider_spin_sub_get_spin_hud_blend(c: &mut Cpu) {
    c.cycles += 11;
    c.lwc1(3, p(c.g[A0][0] as u32, 0x18));
    c.lwc1(0, gp(c, -0x7304));
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 4);
    c.mul_s(0, 3, 0);
    c.add_s(0, 0, 4);
    c.cvt_w_s(1, 0);
    c.cvt_s_w(1, 1);
    c.c_lt_s(0, 1);
    if c.fcc() {
        c.cycles += 3;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.sub_s(1, 1, 0);
    }
    c.cycles += 11;
    c.lwc1(0, gp(c, -0x7300));
    c.lwc1(2, gp(c, -0x72fc));
    c.mul_s(0, 1, 0);
    c.mtc1(0, 1);
    c.sub_s(0, 3, 0);
    c.abs_s(0, 0);
    c.mul_s(0, 0, 2);
    c.sub_s(0, 0, 4);
    c.max_s(0, 0, 1);
    c.add_s(0, 0, 0);
    c.pc = c.g[RA][0] as u32;
}

// ---------------------------------------------------------------------------
// Memory cards and cameras.

/// `void cPS2MemCard_DoOpen(cPS2MemCard* self)` (0x2c69a0): open the memory
/// card file and enter state 1 (failure) or 0x40 (ready).
pub fn cps2_mem_card_do_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = sx(p(c.g[S0][0] as u32, 0xf9));
        Addr(p(c.g[S0][0] as u32, 0x18)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x0c)).lw(c);
        c.g[A3][0] = Addr(p(c.g[S0][0] as u32, 0x14e0)).lw(c);
        call_ptr(c, 0x0040_9d10, 0x002c_69cc)?;
        let ok = (c.g[V0][0] as i64) >= 0;
        c.sw(gp(c, 0x850), c.g[V0][0]);
        if ok {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x40;
            call_ptr(c, 0x002c_48c0, 0x002c_69f4)?;
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x002c_48c0, 0x002c_69e0)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* ChaseCamController_ctor(void* self, void* a1, void* a2)` (0x1619a8).
pub fn chase_cam_controller_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        call_ptr(c, 0x0015_cba0, 0x0016_19c8)?;
        c.g[V1][0] = 0x0046_0000;
        c.g[A2][0] = 0x2;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x46f8));
        c.g[V0][0] = 0x1;
        Addr(p(c.g[S0][0] as u32, 0x20)).sw(c, 0);
        c.g[A1][0] = c.g[S1][0];
        Addr(p(c.g[S0][0] as u32, 0x14)).sw(c, c.g[V1][0]);
        c.g[A0][0] = c.g[S0][0];
        Addr(c.g[S0][0] as u32).sw(c, c.g[A2][0]);
        Addr(p(c.g[S0][0] as u32, 4)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0016_1950, 0x0016_19f4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RailTube_dtor(RailTube* self, int flags)` (0x372660).
pub fn rail_tube_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3068));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 8)).lw(c);
        let no_buf = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_buf {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e98, 0x0037_2694)?;
        }
        c.cycles += 5;
        c.g[V0][0] = 0x0049_0000;
        c.g[V1][0] = c.g[S1][0] & 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7980));
        let keep = c.g[V1][0] == 0;
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0037_26b0)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        c.cycles += 5;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_PollCardStatus(MCManager* self)` (0x241cd8).
pub fn mc_manager_poll_card_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0024_1b20, 0x0024_1cec)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x440)).lw(c);
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x43c)).lw(c);
        c.g[V0][0] = c.g[V0][0] ^ c.g[A0][0];
        let no_status = c.g[V1][0] == 0;
        c.g[V0][0] = (c.g[V0][0] != 0) as u64;
        let mut changed = c.g[V0][0] != 0;
        if !no_status {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 0;
                changed = false;
            }
        }
        if no_status || changed {
            c.cycles += 2;
            Addr(p(c.g[S0][0] as u32, 0x43c)).sw(c, c.g[A0][0]);
            if changed {
                c.cycles += 4;
                c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0xb0)).lw(c);
                let target = c.g[V0][0] as u32;
                call_ptr(c, target, 0x0024_1d20)?;
                Addr(p(c.g[S0][0] as u32, 0x440)).sw(c, c.g[V0][0]);
            }
            c.cycles += 2;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x440)).lw(c);
            c.g[V0][0] = c.g[V0][0] ^ 1;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIStateBuyPopup_Init(void* self)` (0x1cac30).
pub fn cui_state_buy_popup_init(c: &mut Cpu) {
    guest(c, |c| ui_init_screen(c, 0x001c_ac30, 0x0046_62c8))
}

/// `void cKeyboardPopup_dtor(cKeyboardPopup* self, int flags)` (0x1cb138).
pub fn keyboard_popup_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0047_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4138));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(c.g[S0][0] as u32, 8)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x428)).lw(c);
        let no_buf = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_buf {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x001c_b16c)?;
            Addr(p(c.g[S0][0] as u32, 0x428)).sw(c, 0);
        }
        c.cycles += 11;
        c.g[A0][0] = sx(p(c.g[S0][0] as u32, 0x424));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0031_8260, 0x001c_b17c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_e390, 0x001c_b188)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cKeyboardPopup_Init(void* self)` (0x1cb1a0).
pub fn keyboard_popup_init(c: &mut Cpu) {
    guest(c, |c| ui_init_screen(c, 0x001c_b1a0, 0x0046_6388))
}

/// `void cKeyboardPopup_MaskText(cKeyboardPopup* self, const char* str)`
/// (0x1cce98): fill the text field with `*` and terminate it.
pub fn keyboard_popup_mask_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        call_ptr(c, 0x0041_6810, 0x001c_ceb0)?;
        c.g[A0][0] = c.g[V0][0];
        let empty = (c.g[A0][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !empty {
            c.cycles += 3;
            c.g[A2][0] = sx(p(c.g[S0][0] as u32, 0xf4));
            c.g[A3][0] = 0x2a;
            loop {
                c.cycles += 7;
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.sb(c.g[V1][0] as u32, c.g[A3][0]);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001c_cec8);
                }
            }
        }
        c.cycles += 6;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lq(S0, p(c.g[SP][0] as u32, 0x10));
        c.sb(p(c.g[V0][0] as u32, 0xf4), 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

/// `void cKeyboardPopup_SetStaticTextId(cKeyboardPopup* self, int id)`
/// (0x1cd020).
pub fn keyboard_popup_set_static_text_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x40)).lw(c);
        let no_screen = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_screen {
            c.cycles += 2;
            Addr(p(c.g[S0][0] as u32, 0x42c)).sw(c, c.g[S1][0]);
        } else {
            c.cycles += 8;
            c.g[A0][0] = 0x0046_0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6580));
            call_ptr(c, 0x0031_7670, 0x001c_d054)?;
            c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001c_d060)?;
            let missing = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !missing {
                c.cycles += 2;
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x003a_1000, 0x001c_d070)?;
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

/// `int cFEStateTrophyRoom_CountEventEntries(int a, int b)` (0x1ce6f0): count
/// the leading non-negative ids in the 5-entry event table.
pub fn cfe_state_trophy_room_count_event_entries(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0xa;
    c.g[A2][0] = 0x28;
    c.mult(0, 5, 5, 2);
    c.mult(0, 4, 4, 6);
    c.g[V1][0] = 0x0046_0000;
    c.g[A3][0] = 0;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5528));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = c.lb(c.g[A1][0] as u32);
    let negative = (c.g[V0][0] as i64) < 0;
    c.g[A2][0] = 0;
    if !negative {
        c.cycles += 2;
        c.g[A3][0] = 0x1;
        loop {
            c.cycles += 4;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[A2][0] as i64) < 5) as u64;
            let done = c.g[V0][0] == 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(2));
            if done {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lb(c.g[A1][0] as u32);
            if (c.g[V0][0] as i64) < 0 {
                break;
            }
            c.cycles += 2;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            if event_due(c) {
                deliver_event_here(c, 0x001c_e728);
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = c.g[A3][0];
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Codec / effects.

/// `void PS2_SONY_CODEC_INTERNAL_FreeFrame(Frame* self, int flags)`
/// (0x3b08e0).
pub fn ps2_sony_codec_internal_free_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 4)).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !empty {
            c.cycles += 3;
            call_ptr(c, 0x003b_2b68, 0x003b_0908)?;
            Addr(p(c.g[S0][0] as u32, 4)).sw(c, 0);
        }
        c.cycles += 5;
        c.g[V0][0] = 0x7;
        c.g[V1][0] = c.g[S1][0] & 1;
        Addr(p(c.g[S0][0] as u32, 0xc)).sw(c, c.g[V0][0]);
        let no_delete = c.g[V1][0] == 0;
        Addr(p(c.g[S0][0] as u32, 8)).sw(c, c.g[V0][0]);
        if !no_delete {
            c.cycles += 4;
            c.g[V1][0] = 0x0051_0000;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x6bcc)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, target, 0x003b_0930)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        c.cycles += 5;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MPEG2_Flush_Buffer32(BitReader* r)` (0x3b1340): push bytes into the
/// bit buffer until it holds at least 25 bits.
pub fn mpeg2_flush_buffer32(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x0051_0000;
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_sub(0x5f78)).lw(c);
    c.g[T2][0] = c.g[V0][0];
    c.g[V1][0] = Addr(p(c.g[A0][0] as u32, 8)).lw(c);
    c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x20));
    c.g[V0][0] = ((c.g[T0][0] as i64) < 25) as u64;
    let no_loop = c.g[V0][0] == 0;
    Addr(p(c.g[A0][0] as u32, 4)).sw(c, 0);
    if !no_loop {
        c.cycles += 2;
        c.g[A3][0] = c.g[A0][0];
        c.g[T1][0] = 0x18;
        loop {
            c.cycles += 12;
            c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
            c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[T0][0] as u32));
            c.g[A0][0] = Addr(p(c.g[A3][0] as u32, 4)).lw(c);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(8));
            c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
            c.g[A2][0] = ((c.g[T0][0] as i64) < 25) as u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A1][0] as u32) & 31));
            Addr(c.g[A3][0] as u32).sw(c, c.g[V1][0]);
            c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
            let more = c.g[A2][0] != 0;
            Addr(p(c.g[A3][0] as u32, 4)).sw(c, c.g[A0][0]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_1368);
            }
        }
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[T2][0] as u32).wrapping_sub(0x5f78)).lw(c);
    let ra = c.g[RA][0] as u32;
    Addr(p(c.g[V0][0] as u32, 8)).sw(c, c.g[T0][0]);
    c.pc = ra;
}

/// `void VfxProcess_dtor(VfxProcess* self, int flags)` (0x244410).
pub fn vfx_process_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2f90));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(c.g[S0][0] as u32, 0xc)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(p(c.g[S0][0] as u32, 0x10)).lw(c);
        let no_obj = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_obj {
            c.cycles += 6;
            c.g[V0][0] = Addr(p(c.g[A2][0] as u32, 0x44)).lw(c);
            c.g[A1][0] = 0x3;
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 8)).lh(c);
            c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0xc)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0024_4454)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0035_46c8, 0x0024_4460)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cGameLoadStateOutLodge_Init(void* self)` (0x245628).
pub fn c_game_load_state_out_lodge_init(c: &mut Cpu) {
    guest(c, |c| ui_init_screen(c, 0x0024_5628, 0x0047_bcc8))
}

// ---------------------------------------------------------------------------
// Camera algorithms.

fn camera_reset(c: &mut Cpu, base: u32) -> Flow {
    c.cycles += 25;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S1, 0x10);
    c.g[S1][0] = Addr(p(c.g[S0][0] as u32, 0x10)).lw(c);
    c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x30)).lw(c);
    c.g[A2][0] = Addr(p(c.g[S1][0] as u32, 0x18)).lh(c);
    c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 4)).lw(c);
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[A1][0] = Addr(p(c.g[A0][0] as u32, 0x6c0)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[V1][0] = Addr(p(c.g[A1][0] as u32, 0x38)).lh(c);
    c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 0x3c)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    call_ptr(c, target, base + 0x40)?;
    c.g[V1][0] = Addr(p(c.g[S1][0] as u32, 0x1c)).lw(c);
    c.g[A0][0] = c.g[S0][0];
    let target2 = c.g[V1][0] as u32;
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, target2, base + 0x50)?;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void BoundedCameraAlgorithmBase_Reset(BoundedCameraAlgorithmBase* self)`
/// (0x173678).
pub fn bounded_camera_algorithm_base_reset(c: &mut Cpu) {
    guest(c, |c| camera_reset(c, 0x0017_3678))
}

/// `void BoundedCameraAlgorithm_Reset(BoundedCameraAlgorithm* self)`
/// (0x1747a0).
pub fn bounded_camera_algorithm_reset(c: &mut Cpu) {
    guest(c, |c| camera_reset(c, 0x0017_47a0))
}

/// `void* SuperpipeChaseCameraAlgorithm_ctor(void* self, Vec4* a, Vec4* b)`
/// (0x178520).
pub fn superpipe_chase_camera_algorithm_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, 0x0016_2318, 0x0017_8544)?;
        c.g[V1][0] = 0x0046_0000;
        c.g[V0][0] = 0x43;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x33b0));
        Addr(p(c.g[S0][0] as u32, 0xc)).sw(c, c.g[V0][0]);
        Addr(p(c.g[S0][0] as u32, 0x10)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lq(3, c.g[S1][0] as u32);
        frame.restore(c, S1, 0x20);
        c.sq(3, p(c.g[S0][0] as u32, 0x390));
        c.lq(4, c.g[S2][0] as u32);
        frame.restore(c, S2, 0x10);
        c.sq(4, p(c.g[S0][0] as u32, 0x3a0));
        frame.restore(c, S0, 0x30);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    })
}

/// `void* WScriptProcessList_FindMissionById(WScriptProcessList* self, int id)`
/// (0x30b388).
pub fn wscript_process_list_find_mission_by_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        frame.save(c, S1, 0x10);
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        frame.save(c, S0, 0x20);
        c.g[S1][0] = 0;
        let all = c.g[A1][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        let mut found = 0u64;
        let mut plus1 = true;
        if !all {
            c.cycles += 5;
            call_ptr(c, 0x0030_b320, 0x0030_b3b0)?;
            c.g[S0][0] = c.g[V0][0];
            let none = c.g[S0][0] == 0;
            c.g[V0][0] = c.g[S1][0];
            if none {
                plus1 = false;
            } else {
                c.cycles += 6;
                c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x5c)).lw(c);
                c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x18)).lh(c);
                c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x1c)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0030_b3d0)?;
                if c.g[V0][0] != 0 {
                    c.g[S1][0] = c.g[S0][0];
                }
                found = c.g[S1][0];
            }
        }
        if plus1 {
            c.cycles += 1;
        }
        c.g[V0][0] = found;
        frame.restore(c, S0, 0x20);
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Parent (transform) modifiers and particle sections.

/// `void ParentModifier_v22(ParentModifier* self, void* obj)` (0x357210):
/// copy the current transform value into the object around a virtual call.
pub fn parent_modifier_v22(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0xa0)).lw(c);
        let no_parent = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        let mut plus1 = true;
        if !no_parent {
            c.cycles += 3;
            c.g[A2][0] = Addr(p(c.g[A0][0] as u32, 0xc)).lw(c);
            if c.g[A2][0] == 0 {
                plus1 = false;
            } else {
                c.cycles += 9;
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x90)).lw(c);
                c.g[S0][0] = Addr(p(c.g[S1][0] as u32, 0x5c)).lw(c);
                Addr(p(c.g[S1][0] as u32, 0x5c)).sw(c, c.g[V1][0]);
                c.g[V0][0] = Addr(p(c.g[A2][0] as u32, 0xc)).lw(c);
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x150)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x154)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0035_725c)?;
                Addr(p(c.g[S1][0] as u32, 0x5c)).sw(c, c.g[S0][0]);
            }
        }
        if plus1 {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ParentModifier_Save(ParentModifier* self, Stream* stream)` (0x357278).
pub fn parent_modifier_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0035_7000, 0x0035_7294)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A1][0] = sx(p(c.g[S1][0] as u32, 0x90));
        c.g[A2][0] = 0x4;
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 8)).lh(c);
        c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0xc)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_72b0)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 0xa0)).lw(c);
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x28)).lh(c);
        c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x2c)).lw(c);
        let target2 = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target2, 0x0035_72c8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MultiParticleMan_OnSectionUnload(MultiParticleMan* self, int arg)`
/// (0x357b90): unload the four section instance lists.
pub fn multi_particle_man_on_section_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = 0x3;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        loop {
            c.cycles += 5;
            c.g[A1][0] = c.g[S2][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            let skip = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !skip {
                c.cycles += 2;
                call_ptr(c, 0x0035_8140, 0x0035_7bd4)?;
            }
            c.cycles += 2;
            if (c.g[S1][0] as i64) >= 0 {
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0035_7bb8);
                }
                continue;
            }
            c.cycles += 6;
            frame.restore(c, S0, 0x30);
            frame.restore(c, S1, 0x20);
            frame.restore(c, S2, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use ssx3_ee as ee;

    /// The lazily built BELibrary singleton (`gp - 0x1f04`): a zeroed dummy,
    /// so nothing tries to read the file system for it.
    fn be_library(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3_0000);
    }

    /// A dummy world (`gp - 0x848`) with one rider, so rider loops in the
    /// callees terminate.
    fn world(c: &mut Cpu) {
        let (w, ail, arr) = (DATA + 0x8_0000, DATA + 0x8_1000, DATA + 0x8_2000);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), w);
        c.bus.write32(w + 0x7c, DATA + 0x8_4000);
        c.bus.write32(w + 0x84, ail);
        c.bus.write32(ail + 0xc, arr);
        c.bus.write32(arr + 8, 20);
        c.bus.write32(arr + 0x28, DATA + 0x8_3000);
        c.bus.write32(arr + 0x7c, 1);
    }

    /// The engine's size-class pool allocator: `gp + 0xdd8` ready plus eight
    /// free nodes per size class. The front-end constructors allocate a screen
    /// and its lists through it; a zeroed heap would walk garbage.
    fn pool(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool = DATA + 0x4_0000 + 0x1000 * i;
            let base = DATA + 0x6_0000 + 0x2000 * i;
            for n in 0..8u32 {
                let node = base + 0x400 * n;
                c.bus.write32(node, if n == 7 { 0 } else { node + 0x400 });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
        }
    }

    /// A screen-shaped object whose hash list contains one entry with hash 0
    /// (the test machine's string globals are zero, so `StrHashElf` returns 0).
    fn hash_screen(c: &mut Cpu, screen: u32, node: u32) {
        c.bus.write32(screen + 0x38, DATA + 0x40_10);
        c.bus.write32(DATA + 0x40_10, 1);
        c.bus.write32(screen + 0x3c, DATA + 0x40_00);
        c.bus.write32(DATA + 0x40_00, node);
        c.bus.write32(node + 0x38, 0);
    }

    fn init_setup(c: &mut Cpu, seed: u32, salt: u64) {
        machine(c, seed, salt, 3);
        be_library(c);
        world(c);
        pool(c);
        c.g[A0][0] = DATA as u64;
        // A null/zeroed engine makes the hash lookup miss and build a screen.
        c.bus.write32(DATA + 0x10, DATA + 0x1000);
    }

    #[test]
    fn irradiance_free_matches() {
        check(ee::s12::f_0038adb0__generated, irradiance_free, 24, &|c, seed| {
            let mut r = machine(c, seed, 0x0038_adb0, 1);
            c.g[A0][0] = DATA as u64;
            c.bus
                .write32(DATA + 4, if seed % 3 == 0 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(DATA + 8, 0);
            c.bus.write32(DATA + 0xc, 0);
        });
    }

    #[test]
    fn fe_screen_blank_init_matches() {
        check(ee::s03::f_001abfa8__generated, fe_screen_blank_init, 6, &|c, seed| {
            init_setup(c, seed, 0x001a_bfa8);
        });
    }

    #[test]
    fn cfe_state_select_persona_v33_matches() {
        check(
            ee::s03::f_001ad9c8__generated,
            cfe_state_select_persona_v33,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x001a_d9c8, 3);
                be_library(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0xf
                } else {
                    (r.next_u32() & 0x1f) as u64
                };
                c.bus.write32(DATA + 0x2000 + 0x18, 0);
            },
        );
    }

    #[test]
    fn cfe_state_auth_dnas_error_init_matches() {
        check(
            ee::s03::f_001b6b50__generated,
            cfe_state_auth_dnas_error_init,
            6,
            &|c, seed| {
                init_setup(c, seed, 0x001b_6b50);
            },
        );
    }

    #[test]
    fn cfe_state_net_account_mem_update_state16_matches() {
        check(
            ee::s03::f_001b7bc0__generated,
            cfe_state_net_account_mem_update_state16,
            12,
            &|c, seed| {
                machine(c, seed, 0x001b_7bc0, 2);
                be_library(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(GAME_GP + 0x2e0, DATA + 0x1000);
                c.bus
                    .write32(DATA + 0x1000, if seed % 2 == 0 { 0x10 } else { 0x11 });
            },
        );
    }

    #[test]
    fn cfe_state_create_step1_set_password_text_matches() {
        check(
            ee::s03::f_001b9610__generated,
            cfe_state_create_step1_set_password_text,
            8,
            &|c, seed| {
                machine(c, seed, 0x001b_9610, 4);
                be_library(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x40, 0);
                } else {
                    c.bus.write32(DATA + 0x40, DATA + 0x3000);
                    hash_screen(c, DATA + 0x3000, DATA + 0x4200);
                    c.bus.write32(DATA + 0x6d0, DATA + 0x4300);
                    c.bus.write32(DATA + 0x4304, DATA + 0x4400);
                    c.bus.write32(DATA + 0x4400, 0);
                }
            },
        );
    }

    #[test]
    fn cfe_state_create_step1_set_confirm_password_text_matches() {
        check(
            ee::s03::f_001b9678__generated,
            cfe_state_create_step1_set_confirm_password_text,
            8,
            &|c, seed| {
                machine(c, seed, 0x001b_9678, 4);
                be_library(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x40, 0);
                } else {
                    c.bus.write32(DATA + 0x40, DATA + 0x3000);
                    hash_screen(c, DATA + 0x3000, DATA + 0x4200);
                    c.bus.write32(DATA + 0x6d0, DATA + 0x4300);
                    c.bus.write32(DATA + 0x4308, DATA + 0x4400);
                    c.bus.write32(DATA + 0x4400, 0);
                }
            },
        );
    }

    #[test]
    fn cfe_state_lobby_menu_on_create_room_popup_event_matches() {
        check(
            ee::s03::f_001bdcf8__generated,
            cfe_state_lobby_menu_on_create_room_popup_event,
            8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x001b_dcf8, 3);
                be_library(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0xf
                } else {
                    (r.next_u32() & 0x1f) as u64
                };
                c.bus.write32(DATA + 0x2000 + 0x18, 0);
                c.bus.write32(DATA + 0x2000 + 0x6dc, 0);
                c.bus.write32(DATA + 0x2000 + 0x6e0, 0);
                c.bus.write32(DATA + 0x2000 + 0x6e4, 0);
            },
        );
    }

    #[test]
    fn cfe_state_net_create_room_popup_init_matches() {
        check(
            ee::s03::f_001be828__generated,
            cfe_state_net_create_room_popup_init,
            6,
            &|c, seed| {
                init_setup(c, seed, 0x001b_e828);
            },
        );
    }

    #[test]
    fn cov_state_view_popup_init_matches() {
        check(
            ee::s04::f_001e5248__generated,
            cov_state_view_popup_init,
            6,
            &|c, seed| {
                init_setup(c, seed, 0x001e_5248);
            },
        );
    }

    #[test]
    fn cov_state_view_popup_set_subject_matches() {
        check(
            ee::s04::f_001e55d0__generated,
            cov_state_view_popup_set_subject,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x001e_55d0, 2);
                be_library(c);
                world(c);
                c.g[A0][0] = DATA as u64;
                let idx = (seed % 3) as u64;
                c.g[A1][0] = idx;
                let entry = 0x004c_6c08u32 + (idx as u32 * idx as u32) * 0x18;
                c.bus.write32(entry + 8, (seed % 2) as u32);
                c.bus.write32(entry + 0x10, r.next_u32());
                c.bus
                    .write32(DATA + 0x54, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
                c.bus.write32(DATA + 0x2000 + 0xb0, 0);
                c.bus.write32(DATA + 0x2000 + 0xb4, 0);
            },
        );
    }

    #[test]
    fn intc_table_vif1_handler_matches() {
        check(
            ee::s12::f_00362340__generated,
            intc_table_vif1_handler,
            16,
            &|c, seed| {
                machine(c, seed, 0x0036_2340, 0);
                be_library(c);
                c.bus
                    .write32(0x1000_3c00, if seed % 2 == 0 { 0 } else { 0x400 });
            },
        );
    }

    #[test]
    fn texman_reset_vram_matches() {
        check(ee::s12::f_00367078__generated, texman_reset_vram, 8, &|c, seed| {
            machine(c, seed, 0x0036_7078, 2);
            be_library(c);
            world(c);
            pool(c);
            c.g[A0][0] = DATA as u64;
            // `VramHeap_FreeAll(self + 0x4350/0x1f60)` walks a list of rail
            // links: `+0x23c0` is the head index and `+0x1ff0` the link
            // array. A zeroed head (0) with a null array makes the inner
            // "find the next free link" walk read `[0].next` as 0 forever.
            // A negative head terminates both free-alls immediately; the
            // function under test only calls them and resets the counters.
            c.bus.write32(DATA + 0x4350 + 0x23c0, 0xffff_ffff);
            c.bus.write32(DATA + 0x1f60 + 0x23c0, 0xffff_ffff);
        });
    }

    #[test]
    fn lobbyman_query_own_stats_matches() {
        check(
            ee::s06::f_0025cfe0__generated,
            lobbyman_query_own_stats,
            8,
            &|c, seed| {
                machine(c, seed, 0x0025_cfe0, 2);
                be_library(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x80, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, 0x78); // "x\0"
                c.bus.write32(DATA + 0x54, 0);
            },
        );
    }

    #[test]
    fn c_usb_keyboard_ctor_matches() {
        check(ee::s07::f_002667e8__generated, c_usb_keyboard_ctor, 8, &|c, seed| {
            machine(c, seed, 0x0026_67e8, 2);
            be_library(c);
            world(c);
            pool(c);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn net_event_fb_clone_matches() {
        check(ee::s07::f_00268688__generated, net_event_fb_clone, 8, &|c, seed| {
            let mut r = machine(c, seed, 0x0026_8688, 1);
            be_library(c);
            world(c);
            pool(c);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 8, r.next_u32());
            c.bus.write32(DATA + 0xc, r.next_u32());
        });
    }

    #[test]
    fn ssx_audio_system_resume_sfx_matches() {
        check(
            ee::s08::f_0029ce70__generated,
            ssx_audio_system_resume_sfx,
            8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0029_ce70, 3);
                be_library(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(DATA + 0x5fb0, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(DATA + 0x118, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0x26c, r.next_u32());
                c.bus.write32(GAME_GP + 0x21e4, DATA + 0x3000);
            },
        );
    }

    #[test]
    fn crowd_audio_remove_source_matches() {
        check(
            ee::s08::f_002a72d8__generated,
            crowd_audio_remove_source,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_72d8, 2);
                be_library(c);
                world(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 3 == 0 {
                    0xffff_ffffu64
                } else {
                    (r.next_u32() & 0x7fff) as u64
                };
                c.bus
                    .write32(DATA + 0x18, if seed % 4 == 0 { 0 } else { DATA + 0x2000 });
                c.f[12] = r.next_u32();
            },
        );
    }

    #[test]
    fn crowd_audio_set_source_param_matches() {
        check(
            ee::s08::f_002a7340__generated,
            crowd_audio_set_source_param,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_7340, 2);
                be_library(c);
                world(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 3 == 0 {
                    0xffff_ffffu64
                } else {
                    (r.next_u32() & 0x7fff) as u64
                };
                c.g[A2][0] = (r.next_u32() % 8) as u64;
                c.bus
                    .write32(DATA + 0x18, if seed % 4 == 0 { 0 } else { DATA + 0x2000 });
                c.f[12] = r.next_u32();
            },
        );
    }

    #[test]
    fn ini_parser_get_float_matches() {
        check(ee::s08::f_002a8070__generated, ini_parser_get_float, 8, &|c, seed| {
            machine(c, seed, 0x002a_8070, 2);
            be_library(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (if seed % 2 == 0 {
                DATA + 0x1200
            } else {
                DATA + 0x1100
            }) as u64;
            c.g[A2][0] = (DATA + 0x1300) as u64;
            c.bus.write32(DATA + 0x1200, 0x79); // "y\0"
            c.bus.write32(DATA + 0x1100, 0); // empty name
            c.bus.write32(DATA + 0x8, DATA + 0x1400); // empty input buffer
            c.bus.write32(DATA + 0x1400, 0);
        });
    }

    #[test]
    fn ini_parser_get_int_matches() {
        check(ee::s08::f_002a80d8__generated, ini_parser_get_int, 8, &|c, seed| {
            machine(c, seed, 0x002a_80d8, 2);
            be_library(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (if seed % 2 == 0 {
                DATA + 0x1200
            } else {
                DATA + 0x1100
            }) as u64;
            c.g[A2][0] = (DATA + 0x1300) as u64;
            c.bus.write32(DATA + 0x1200, 0x79);
            c.bus.write32(DATA + 0x1100, 0);
            c.bus.write32(DATA + 0x8, DATA + 0x1400);
            c.bus.write32(DATA + 0x1400, 0);
        });
    }

    #[test]
    fn script_engine_list_add_movie_by_id_matches() {
        check(
            ee::s07::f_00278ed0__generated,
            script_engine_list_add_movie_by_id,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_8ed0, 1);
                be_library(c);
                world(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x52c, DATA + 0x1000);
                c.bus.write32(DATA + 0x530, DATA + 0x1100);
                c.bus.write32(DATA + 0x1000, 3);
                for i in 0..3u32 {
                    let e = DATA + 0x1100 + 0xc * i;
                    c.bus.write32(e + 4, 0x100 + i);
                }
                let key = match seed % 4 {
                    0 => 0x100,
                    1 => 0x102,
                    2 => 0x999,
                    _ => (0x100 + r.next_u32() % 3) & 0x1ff,
                };
                c.g[A2][0] = key as u64;
                c.bus.write32(DATA + 0x54c, DATA + 0x2000);
            },
        );
    }

    #[test]
    fn music_player_set_event_state_matches() {
        check(
            ee::s08::f_002b3bc0__generated,
            music_player_set_event_state,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002b_3bc0, 1);
                be_library(c);
                c.g[A0][0] = DATA as u64;
                let v = (r.next_u32() & 7) as u64;
                let current = (seed as u32) & 7;
                c.g[A1][0] = if seed % 3 == 0 { current as u64 } else { v };
                c.bus
                    .write32(DATA + 0x418, if seed % 4 == 0 { 0 } else { 1 });
                c.bus.write32(DATA + 0x41c, current);
                c.bus.write32(DATA + 0x5440, DATA + 0x2000);
                c.bus.write16(DATA + 0x2000 + 0x18, 0);
                c.bus.write32(DATA + 0x2000 + 0x1c, FAKE);
            },
        );
    }

    #[test]
    fn replay_save_compute_padding_matches() {
        check(
            ee::s07::f_00271620__generated,
            replay_save_compute_padding,
            8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_1620, 2);
                be_library(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x640, DATA + 0x1_0000);
                c.bus.write32(DATA + 0x638, r.next_u32());
                c.bus.write32(DATA + 0x678, r.next_u32() & 0xffff);
            },
        );
    }

    #[test]
    fn replay_save_write_padding_matches() {
        check(
            ee::s07::f_00271688__generated,
            replay_save_write_padding,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_1688, 1);
                be_library(c);
                c.g[A0][0] = DATA as u64;
                let left: i32 = match seed % 5 {
                    0 => -1,
                    1 => 0,
                    2 => 1 + (r.next_u32() % 0x3fff) as i32,
                    3 => 0x4000,
                    _ => 0x4000 + (r.next_u32() % 0x1000) as i32,
                };
                c.bus.write32(DATA + 0x67c, left as u32);
                c.bus.write32(DATA + 0x640, DATA + 0x1_0000);
                c.bus.write32(DATA + 0x638, r.next_u32());
            },
        );
    }

    #[test]
    fn in_seq_instance_get_remaining_steps_matches() {
        check(
            ee::s07::f_00274d08__generated,
            in_seq_instance_get_remaining_steps,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_4d08, 1);
                be_library(c);
                world(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x24, r.f32(8.0) | 0x0080_0000);
                c.bus.write32(DATA + 0x28, r.f32(4.0));
                c.bus.write32(DATA + 0x30, 0); // length path returns at once
            },
        );
    }

    #[test]
    fn font_draw_text_f_matches() {
        check(ee::s12::f_00391c48__generated, font_draw_text_f, 4, &|c, seed| {
            let mut r = machine(c, seed, 0x0039_1c48, 2);
            be_library(c);
            world(c);
            pool(c);
            c.g[A0][0] = DATA as u64;
            c.f[12] = r.edge_f32();
            c.f[13] = r.edge_f32();
        });
    }

    #[test]
    fn rider_spin_sub_get_spin_hud_blend_matches() {
        check(
            ee::s01::f_00136100__generated,
            rider_spin_sub_get_spin_hud_blend,
            200,
            &|c, seed| {
                let mut r = Rng(0x1361_00d1 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x18, r.edge_f32());
                c.bus.write32(GAME_GP.wrapping_sub(0x7304), r.edge_f32());
                c.bus.write32(GAME_GP.wrapping_sub(0x7300), r.edge_f32());
                c.bus.write32(GAME_GP.wrapping_sub(0x72fc), r.edge_f32());
            },
        );
    }

    #[test]
    fn cps2_mem_card_do_open_matches() {
        check(ee::s08::f_002c69a0__generated, cps2_mem_card_do_open, 8, &|c, seed| {
            machine(c, seed, 0x002c_69a0, 2);
            be_library(c);
            world(c);
            pool(c);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0xc, 0);
            c.bus.write32(DATA + 0x14e0, 0);
            c.bus.write32(0x0052_71c0 + 0x24, 0);
        });
    }

    #[test]
    fn chase_cam_controller_ctor_matches() {
        check(
            ee::s01::f_001619a8__generated,
            chase_cam_controller_ctor,
            8,
            &|c, seed| {
                machine(c, seed, 0x0016_19a8, 2);
                be_library(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = (DATA + 0x3000) as u64;
            },
        );
    }

    #[test]
    fn rail_tube_dtor_matches() {
        check(ee::s12::f_00372660__generated, rail_tube_dtor, 8, &|c, seed| {
            machine(c, seed, 0x0037_2660, 1);
            be_library(c);
            pool(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = 0;
            c.bus.write32(DATA + 8, 0);
        });
    }

    #[test]
    fn ps2_graphics_man_v60_matches() {
        check(ee::s12::f_0037cc30__generated, ps2_graphics_man_v60, 12, &|c, seed| {
            let mut r = machine(c, seed, 0x0037_cc30, 2);
            be_library(c);
            world(c);
            pool(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x1000) as u64;
            c.bus.write32(
                DATA + 0x1004,
                if seed % 3 == 0 {
                    0xffff_ffff
                } else {
                    r.next_u32() & 0x7fff
                },
            );
            c.bus.write32(DATA + 0x18f4, 0);
            c.bus.write32(DATA + 0x10d8, DATA + 0x2000);
            c.bus.write16(DATA + 0x2000 + 0x190, 0);
            c.bus.write32(DATA + 0x2000 + 0x194, FAKE);
        });
    }

    #[test]
    fn mc_manager_poll_card_status_matches() {
        check(
            ee::s06::f_00241cd8__generated,
            mc_manager_poll_card_status,
            16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0024_1cd8, 1);
                be_library(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x43c, r.next_u32());
                c.bus.write32(
                    DATA + 0x440,
                    if seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus
                    .write32(DATA + 0xb0, if seed % 2 == 0 { 0 } else { FAKE });
            },
        );
    }

    #[test]
    fn cui_state_buy_popup_init_matches() {
        check(
            ee::s04::f_001cac30__generated,
            cui_state_buy_popup_init,
            6,
            &|c, seed| {
                init_setup(c, seed, 0x001c_ac30);
            },
        );
    }

    #[test]
    fn keyboard_popup_dtor_matches() {
        check(ee::s04::f_001cb138__generated, keyboard_popup_dtor, 8, &|c, seed| {
            machine(c, seed, 0x001c_b138, 2);
            be_library(c);
            world(c);
            pool(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = 0;
            c.bus.write32(DATA + 0x428, 0);
        });
    }

    #[test]
    fn keyboard_popup_init_matches() {
        check(ee::s04::f_001cb1a0__generated, keyboard_popup_init, 6, &|c, seed| {
            init_setup(c, seed, 0x001c_b1a0);
        });
    }

    #[test]
    fn keyboard_popup_mask_text_matches() {
        check(
            ee::s04::f_001cce98__generated,
            keyboard_popup_mask_text,
            24,
            &|c, seed| {
                machine(c, seed, 0x001c_ce98, 0);
                be_library(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                let n = seed % 20;
                for i in 0..n {
                    c.bus.write8(DATA + 0x1000 + i, b'q');
                }
                c.bus.write8(DATA + 0x1000 + n, 0);
            },
        );
    }

    #[test]
    fn keyboard_popup_set_static_text_id_matches() {
        check(
            ee::s04::f_001cd020__generated,
            keyboard_popup_set_static_text_id,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x001c_d020, 3);
                be_library(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x40, 0);
                } else {
                    c.bus.write32(DATA + 0x40, DATA + 0x3000);
                    hash_screen(c, DATA + 0x3000, DATA + 0x4200);
                }
            },
        );
    }

    #[test]
    fn cfe_state_trophy_room_count_event_entries_matches() {
        check(
            ee::s04::f_001ce6f0__generated,
            cfe_state_trophy_room_count_event_entries,
            200,
            &|c, seed| {
                let mut r = Rng(0x1ce6_f0d3 ^ seed as u64);
                scramble(c, &mut r);
                let a = (r.next_u32() % 4) as u64;
                let b = (r.next_u32() % 4) as u64;
                c.g[A0][0] = a;
                c.g[A1][0] = b;
                let base = 0x0045_aad8 + (a * 0x28 + b * 0x0a) as u32;
                let neg_from = (r.next_u32() % 6) as u32;
                for i in 0..5u32 {
                    let id: u8 = if i >= neg_from {
                        0x80
                    } else {
                        (r.next_u32() & 0x7f) as u8
                    };
                    c.bus.write8(base + 2 * i, id);
                }
            },
        );
    }

    #[test]
    fn ps2_sony_codec_internal_free_frame_matches() {
        check(
            ee::s13::f_003b08e0__generated,
            ps2_sony_codec_internal_free_frame,
            12,
            &|c, seed| {
                machine(c, seed, 0x003b_08e0, 2);
                be_library(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(DATA + 4, 0);
                c.bus.write32(0x0050_9434, FAKE);
            },
        );
    }

    #[test]
    fn mpeg2_flush_buffer32_matches() {
        check(
            ee::s13::f_003b1340__generated,
            mpeg2_flush_buffer32,
            120,
            &|c, seed| {
                let mut r = Rng(0x3b13_40e7 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(0x0050_a088, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, DATA + 0x1100);
                c.bus.write32(DATA + 0x1004, r.next_u32());
                c.bus.write32(DATA + 0x1008, 0x20 + (r.next_u32() % 0x40));
                for i in 0..8 {
                    c.bus.write8(DATA + 0x1100 + i, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn vfx_process_dtor_matches() {
        check(ee::s06::f_00244410__generated, vfx_process_dtor, 12, &|c, seed| {
            machine(c, seed, 0x0024_4410, 2);
            be_library(c);
            world(c);
            pool(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = 0;
            c.bus
                .write32(DATA + 0x10, if seed % 2 == 0 { 0 } else { DATA + 0x1000 });
            c.bus.write32(DATA + 0x1000 + 0x44, DATA + 0x2000);
            c.bus.write16(DATA + 0x2000 + 8, 0);
            c.bus.write32(DATA + 0x2000 + 0xc, FAKE);
        });
    }

    #[test]
    fn c_game_load_state_out_lodge_init_matches() {
        check(
            ee::s06::f_00245628__generated,
            c_game_load_state_out_lodge_init,
            6,
            &|c, seed| {
                init_setup(c, seed, 0x0024_5628);
            },
        );
    }

    fn camera_setup(c: &mut Cpu, seed: u32, salt: u64) {
        machine(c, seed, salt, 2);
        be_library(c);
        c.g[A0][0] = DATA as u64;
        c.bus.write32(DATA + 0x10, DATA + 0x2000);
        c.bus.write16(DATA + 0x2000 + 0x18, 0);
        c.bus.write32(DATA + 0x2000 + 0x1c, FAKE);
        c.bus.write32(DATA + 0x30, DATA + 0x1000);
        c.bus.write32(DATA + 0x1004, DATA + 0x1100);
        c.bus.write32(DATA + 0x1100 + 0x6c0, DATA + 0x1200);
        c.bus.write16(DATA + 0x1200 + 0x38, 0);
        c.bus.write32(DATA + 0x1200 + 0x3c, FAKE);
    }

    #[test]
    fn bounded_camera_algorithm_base_reset_matches() {
        check(
            ee::s02::f_00173678__generated,
            bounded_camera_algorithm_base_reset,
            8,
            &|c, seed| camera_setup(c, seed, 0x0017_3678),
        );
    }

    #[test]
    fn bounded_camera_algorithm_reset_matches() {
        check(
            ee::s02::f_001747a0__generated,
            bounded_camera_algorithm_reset,
            8,
            &|c, seed| camera_setup(c, seed, 0x0017_47a0),
        );
    }

    #[test]
    fn superpipe_chase_camera_algorithm_ctor_matches() {
        check(
            ee::s02::f_00178520__generated,
            superpipe_chase_camera_algorithm_ctor,
            8,
            &|c, seed| {
                machine(c, seed, 0x0017_8520, 1);
                be_library(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = (DATA + 0x1010) as u64;
            },
        );
    }

    #[test]
    fn wscript_process_list_find_mission_by_id_matches() {
        check(
            ee::s10::f_0030b388__generated,
            wscript_process_list_find_mission_by_id,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0030_b388, 1);
                be_library(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x20, 0x55);
                c.bus.write32(DATA + 0x1000 + 0x18, 0);
                c.bus.write32(DATA + 0x1000 + 0x5c, DATA + 0x2000);
                c.bus.write16(DATA + 0x2000 + 0x18, 0);
                c.bus.write32(DATA + 0x2000 + 0x1c, FAKE);
                c.g[A1][0] = match seed % 3 {
                    0 => 0xffff_ffffu64,
                    1 => 0x55,
                    _ => (r.next_u32() & 0xff) as u64,
                };
            },
        );
    }

    #[test]
    fn parent_modifier_v22_matches() {
        check(ee::s11::f_00357210__generated, parent_modifier_v22, 12, &|c, seed| {
            let mut r = machine(c, seed, 0x0035_7210, 1);
            be_library(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x3000) as u64;
            c.bus.write32(DATA + 0x90, r.next_u32());
            c.bus.write32(DATA + 0x3000 + 0x5c, r.next_u32());
            match seed % 3 {
                0 => c.bus.write32(DATA + 0xa0, 0),
                1 => {
                    c.bus.write32(DATA + 0xa0, DATA + 0x1000);
                    c.bus.write32(DATA + 0x1000 + 0xc, 0);
                }
                _ => {
                    c.bus.write32(DATA + 0xa0, DATA + 0x1000);
                    c.bus.write32(DATA + 0x1000 + 0xc, DATA + 0x2000);
                    c.bus.write32(DATA + 0x2000 + 0xc, DATA + 0x2100);
                    c.bus.write16(DATA + 0x2100 + 0x150, 0);
                    c.bus.write32(DATA + 0x2100 + 0x154, FAKE);
                }
            }
        });
    }

    #[test]
    fn parent_modifier_save_matches() {
        check(ee::s11::f_00357278__generated, parent_modifier_save, 8, &|c, seed| {
            let mut r = machine(c, seed, 0x0035_7278, 2);
            be_library(c);
            world(c);
            pool(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x1000) as u64;
            c.bus.write32(DATA + 0xa0, DATA + 0x3000);
            c.bus.write32(DATA + 0x1000, DATA + 0x2000);
            c.bus.write16(DATA + 0x2000 + 8, 0);
            c.bus.write32(DATA + 0x2000 + 0xc, FAKE);
            c.bus.write16(DATA + 0x2000 + 0x28, 0);
            c.bus.write32(DATA + 0x2000 + 0x2c, FAKE);
            c.bus.write32(DATA + 0x90, r.next_u32());
        });
    }

    #[test]
    fn multi_particle_man_on_section_unload_matches() {
        check(
            ee::s11::f_00357b90__generated,
            multi_particle_man_on_section_unload,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_7b90, 2);
                be_library(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                for i in 0..4u32 {
                    let p = DATA + 0x1000 * (i + 1);
                    let v = if (seed >> i) & 1 != 0 { p } else { 0 };
                    c.bus.write32(DATA + 4 * i, v);
                    c.bus.write32(p + 4, 0);
                }
            },
        );
    }
}
