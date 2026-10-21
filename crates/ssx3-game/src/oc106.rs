//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold-path front-end, net, audio, menu, codec, replay and
//! renderer helpers, each with a differential test against the generated
//! body.
//!
//! The bodies are transcribed from the generated Rust (the authority for
//! registers, cycles and call order). Calls go through [`crate::call`] with
//! the original return address, so a replaced callee and the fake hooks the
//! differential tests install are both reached. Structured loops make the
//! original's back-edge event checks with [`crate::events::event_due`] and
//! [`crate::events::deliver_event_here`]; the three `match pc` dispatchers
//! accept the loop heads the trampoline can re-enter and use
//! [`crate::events::back_edge_unwinds`].

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001b_1f18,
        c_fe_state_online_character_select_on_quit_popup_event,
        "cFEStateOnlineCharacterSelect_OnQuitPopupEvent",
    ),
    (
        0x001b_6ab8,
        c_fe_state_auth_dnas_error_ctor,
        "cFEStateAuthDNASError_ctor",
    ),
    (
        0x001b_b0e8,
        c_fe_state_create_step2_on_parent_email_entered,
        "cFEStateCreateStep2_OnParentEmailEntered",
    ),
    (
        0x001b_b2d0,
        c_fe_state_create_step2_set_month_selection,
        "cFEStateCreateStep2_SetMonthSelection",
    ),
    (
        0x001b_b340,
        c_fe_state_create_step2_set_day_selection,
        "cFEStateCreateStep2_SetDaySelection",
    ),
    (
        0x001b_b440,
        c_fe_state_create_step2_set_gender_selection,
        "cFEStateCreateStep2_SetGenderSelection",
    ),
    (
        0x001b_cca8,
        c_fe_state_lobby_menu_v32,
        "cFEStateLobbyMenu_v32",
    ),
    (0x001b_eed0, c_fe_state_login_init, "cFEStateLogin_Init"),
    (0x001c_2e00, c_fe_state_stats_init, "cFEStateStats_Init"),
    (0x001c_2e70, c_fe_state_stats_v32, "cFEStateStats_v32"),
    (0x001c_5108, c_fe_state_net_news_v32, "cFEStateNetNews_v32"),
    (
        0x0035_f1a8,
        tex_flip_modifier_ctor_2,
        "TexFlipModifier_ctor_2",
    ),
    (0x0036_1858, floating_v51, "Floating_v51"),
    (0x0036_70e0, tex_man_init_table, "TexMan_InitTable"),
    (0x0025_e0c8, lobby_man_request_news, "LobbyMan_RequestNews"),
    (
        0x0026_0f10,
        lobby_man_reset_account_info,
        "LobbyMan_ResetAccountInfo",
    ),
    (
        0x0026_2280,
        buddy_man_count_permanent_entries,
        "BuddyMan_CountPermanentEntries",
    ),
    (
        0x0026_22f0,
        buddy_man_count_temp_entries,
        "BuddyMan_CountTempEntries",
    ),
    (0x0026_5950, c_shared_login_man_init, "cSharedLoginMan_Init"),
    (
        0x0026_83f0,
        net_event102_down_clone,
        "NetEvent102Down_Clone",
    ),
    (0x0026_8720, net_event_fc_clone, "NetEventFC_Clone"),
    (
        0x002a_6430,
        crowd_audio_instance_remove_all_sources,
        "CrowdAudio_InstanceRemoveAllSources",
    ),
    (
        0x002a_7a20,
        ini_parser_skip_section_headers,
        "IniParser_SkipSectionHeaders",
    ),
    (
        0x0027_b4b8,
        script_engine_cancel_bank_load,
        "ScriptEngine_CancelBankLoad",
    ),
    (0x002a_dee8, sound_manager_ref_dtor, "SoundManagerRef_dtor"),
    (
        0x002b_0348,
        speech_instance_add_bank,
        "SpeechInstance_AddBank",
    ),
    (0x002b_2418, song_instance_fade_out, "SongInstance_FadeOut"),
    (
        0x002c_ce38,
        menu_sys_back_item_handle_input,
        "MenuSysBackItem_HandleInput",
    ),
    (
        0x002c_d1d0,
        menu_sys_callback_item_handle_input,
        "MenuSysCallbackItem_HandleInput",
    ),
    (
        0x002c_e418,
        menu_sys_text_edit_item_ctor,
        "MenuSysTextEditItem_ctor",
    ),
    (0x0027_0e68, replay_begin_save, "Replay_BeginSave"),
    (0x0039_2a60, font_draw_text_right, "Font_DrawTextRight"),
    (
        0x0039_2df0,
        renderer_build_sine_table,
        "Renderer_BuildSineTable",
    ),
    (0x0012_96f8, ai_load_next_rider, "AI_LoadNextRider"),
    (0x002c_3fe0, c_ps2_mem_card_dtor, "cPS2MemCard_dtor"),
    (
        0x0016_21a8,
        chase_cam_controller_enter_spoke_mode,
        "ChaseCamController_EnterSpokeMode",
    ),
    (0x0023_e428, mc_manager_state38, "MCManager_State38"),
    (0x001c_5af8, c_fe_popup_confirm_v04, "cFEPopupConfirm_v04"),
    (
        0x001c_afc0,
        c_ui_state_buy_popup_set_attrib_params,
        "cUIStateBuyPopup_SetAttribParams",
    ),
    (
        0x001c_cf00,
        c_keyboard_popup_set_static_text,
        "cKeyboardPopup_SetStaticText",
    ),
    (0x003b_0430, rcmp_decoder_ctor, "RCMP_DECODER_ctor"),
    (0x003b_1290, mpeg2_flush_buffer, "MPEG2_Flush_Buffer"),
    (
        0x0030_9b00,
        w_script_man_deactivate_mission,
        "WScriptMan_DeactivateMission",
    ),
    (0x0035_7660, magnet_modifier_v09, "MagnetModifier_v09"),
    (0x0017_e2f8, lobby_menu_enter_room, "LobbyMenu_EnterRoom"),
    (
        0x0017_e558,
        lobby_menu_cancel_challenge,
        "LobbyMenu_CancelChallenge",
    ),
    (
        0x001d_24a8,
        c_fe_state_video_player_update,
        "cFEStateVideoPlayer_Update",
    ),
    (
        0x0028_6ca8,
        ssx_audio_system_load_memory_bank,
        "SSXAudioSystem_LoadMemoryBank",
    ),
    (
        0x0028_ef90,
        ssx_audio_system_restart_peak_music,
        "SSXAudioSystem_RestartPeakMusic",
    ),
    (
        0x0018_c128,
        c_fe_state_options_hud_init,
        "cFEStateOptionsHUD_Init",
    ),
];

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
    r32(c, GP).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// Front-end state helpers.

/// `cFEStateOnlineCharacterSelect::OnQuitPopupEvent(self, a1, kind)`
/// (0x1b1f18): a `0xf` child event sends the quit request to the setup
/// manager (when it exists) and resends the event as kind `0xf`.
pub fn c_fe_state_online_character_select_on_quit_popup_event(c: &mut Cpu) {
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
            c.cycles += 3;
            c.g[A0][0] = Addr(gp(c, 0x2dc)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[A1][0] = 0xf9;
                call_ptr(c, 0x0026_5768, 0x001b_1f4c)?;
                c.g[A0][0] = c.g[S0][0];
            } else {
                c.g[A0][0] = c.g[S0][0];
            }
            c.cycles += 5;
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 0xf;
            call_ptr(c, 0x001a_97b8, 0x001b_1f5c)?;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x001a_97b8, 0x001b_1f70)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateAuthDNASError* ctor(self, a1, a2, a3, a4, a5)` (0x1b6ab8): the
/// five-argument net-front-end state constructor with the fields the auth
/// state adds.
pub fn c_fe_state_auth_dnas_error_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[T1][0];
        call_ptr(c, 0x001a_8500, 0x001b_6ae4)?;
        c.g[V1][0] = 0x470000;
        c.g[A0][0] = 0x45;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7720));
        Addr(p(c.g[S0][0] as u32, 0x6d0)).sw(c, c.g[S1][0]);
        Addr(p(c.g[S0][0] as u32, 0x6d4)).sw(c, c.g[S2][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(p(c.g[S0][0] as u32, 0x6d8)).sw(c, c.g[S3][0]);
        Addr(p(c.g[S0][0] as u32, 0x8)).sw(c, c.g[V1][0]);
        Addr(p(c.g[S0][0] as u32, 0xc)).sw(c, c.g[A0][0]);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateCreateStep2::OnParentEmailEntered(self, a1, kind)` (0x1bb0e8):
/// on the `0xf` child event, hand the entered email to the account and
/// submit it.
pub fn c_fe_state_create_step2_on_parent_email_entered(c: &mut Cpu) {
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
            c.cycles += 11;
            c.g[A2][0] = 0xf;
            call_ptr(c, 0x001a_97b8, 0x001b_b110)?;
            c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x6d0)).lw(c);
            c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 0x2b0)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x0031_84e8, 0x001b_b120)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x001b_a9f8, 0x001b_b12c)?;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x001a_97b8, 0x001b_b140)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of the three create-step-2 selection setters: hash the
/// label, find the list box on the created screen, look the object's value
/// up in it and select that row.
fn step2_set_selection(c: &mut Cpu, entry: u32, tag: u32, value_off: u32) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S1, 0x20);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x30);
    frame.save_ra(c, 0x10);
    c.g[A0][0] = sx(tag);
    call_ptr(c, 0x0031_7670, entry + 0x20)?;
    c.g[A0][0] = Addr(p(c.g[S1][0] as u32, 0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, entry + 0x2c)?;
    c.g[S0][0] = c.g[V0][0];
    let missing = c.g[S0][0] == 0;
    c.g[A2][0] = c.g[SP][0];
    if !missing {
        c.cycles += 8;
        c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0x6d0)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        Addr(c.g[SP][0] as u32).sb(c, 0);
        c.g[A1][0] = Addr(p(c.g[V0][0] as u32, value_off)).lw(c);
        call_ptr(c, 0x0039_a768, entry + 0x4c)?;
        c.g[A1][0] = Addr(c.g[SP][0] as u32).lbu(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_a7a8, entry + 0x58)?;
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// `cFEStateCreateStep2::SetMonthSelection(self)` (0x1bb2d0).
pub fn c_fe_state_create_step2_set_month_selection(c: &mut Cpu) {
    guest(c, |c| {
        step2_set_selection(c, 0x001b_b2d0, 0x004a_1bd8, 0x28)
    })
}

/// `cFEStateCreateStep2::SetDaySelection(self)` (0x1bb340).
pub fn c_fe_state_create_step2_set_day_selection(c: &mut Cpu) {
    guest(c, |c| {
        step2_set_selection(c, 0x001b_b340, 0x004a_1be0, 0x2c)
    })
}

/// `cFEStateCreateStep2::SetGenderSelection(self)` (0x1bb440).
pub fn c_fe_state_create_step2_set_gender_selection(c: &mut Cpu) {
    guest(c, |c| {
        step2_set_selection(c, 0x001b_b440, 0x0046_4a18, 0x34)
    })
}

/// The shared body of the `*_v32` presence pushers: make sure the buddy
/// manager exists, reconnect when it is disconnected, disconnect the lobby
/// when there is still no manager, then push the presence (kind `a2`).
fn presence_v32(c: &mut Cpu, entry: u32, kind: u64) -> Flow {
    c.cycles += 4;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
    let have_manager = c.g[V0][0] != 0;
    frame.save_ra(c, 0);
    if !have_manager {
        c.cycles += 2;
        call_ptr(c, 0x0026_0f80, entry + 0x18)?;
    }
    c.cycles += 4;
    c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
    call_ptr(c, 0x0026_1460, entry + 0x20)?;
    let connected = c.g[V0][0] != 0;
    c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
    if !connected {
        c.cycles += 3;
        c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
        call_ptr(c, 0x0026_13c8, entry + 0x30)?;
        c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
    }
    c.cycles += 2;
    let have_manager = c.g[V0][0] != 0;
    c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
    if !have_manager {
        c.cycles += 3;
        c.g[A0][0] = Addr(gp(c, 0xc8)).lw(c);
        call_ptr(c, 0x0025_b800, entry + 0x44)?;
        c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
    }
    c.cycles += 9;
    c.g[A1][0] = 1;
    c.g[A2][0] = kind;
    c.g[A3][0] = 0;
    c.g[T0][0] = 0;
    c.g[T1][0] = 0;
    call_ptr(c, 0x0026_2768, entry + 0x60)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cFEStateLobbyMenu_v32(self)` (0x1bcca8): push presence kind 2.
pub fn c_fe_state_lobby_menu_v32(c: &mut Cpu) {
    guest(c, |c| presence_v32(c, 0x001b_cca8, 2))
}

/// `cFEStateStats_v32(self)` (0x1c2e70): push presence kind 5.
pub fn c_fe_state_stats_v32(c: &mut Cpu) {
    guest(c, |c| presence_v32(c, 0x001c_2e70, 5))
}

/// `cFEStateNetNews_v32(self)` (0x1c5108): push presence kind 4.
pub fn c_fe_state_net_news_v32(c: &mut Cpu) {
    guest(c, |c| presence_v32(c, 0x001c_5108, 4))
}

/// `cFEStateLogin::Init(self)` (0x1beed0): create the state's screen by tag,
/// start its thread and initialise the net state.
pub fn c_fe_state_login_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x52b0));
        c.g[S0][0] = Addr(p(c.g[S1][0] as u32, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001b_eef4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001b_ef08)?;
        let missing = c.g[V0][0] == 0;
        Addr(p(c.g[S1][0] as u32, 0x40)).sw(c, c.g[V0][0]);
        if !missing {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001b_ef20)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001a_8768, 0x001b_ef28)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateStats::Init(self)` (0x1c2e00): create the stats screen, start
/// its thread and refresh the result rows.
pub fn c_fe_state_stats_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5c68));
        c.g[S0][0] = Addr(p(c.g[S1][0] as u32, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001c_2e24)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001c_2e38)?;
        let missing = c.g[V0][0] == 0;
        Addr(p(c.g[S1][0] as u32, 0x40)).sw(c, c.g[V0][0]);
        if !missing {
            c.cycles += 6;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001c_2e50)?;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x001c_3310, 0x001c_2e58)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEPopupConfirm_v04(self)` (0x1c5af8): create the popup screen and
/// start it, marking the "popup" flag.
pub fn c_fe_popup_confirm_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c40));
        c.g[S0][0] = Addr(p(c.g[S1][0] as u32, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001c_5b1c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001c_5b30)?;
        c.g[A0][0] = c.g[V0][0];
        let missing = c.g[A0][0] == 0;
        Addr(p(c.g[S1][0] as u32, 0x40)).sw(c, c.g[A0][0]);
        if !missing {
            c.cycles += 5;
            c.g[V0][0] = 1;
            c.g[A1][0] = 0;
            Addr(p(c.g[S1][0] as u32, 0x2b8)).sw(c, c.g[V0][0]);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001c_5b50)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateOptionsHUD::Init(self)` (0x18c128): create the options HUD screen,
/// start it and refresh the selected row.
pub fn c_fe_state_options_hud_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fc8));
        c.g[S0][0] = Addr(p(c.g[S1][0] as u32, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0018_c14c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0018_c160)?;
        let missing = c.g[V0][0] == 0;
        Addr(p(c.g[S1][0] as u32, 0x40)).sw(c, c.g[V0][0]);
        if !missing {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0018_c178)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0018_c478, 0x0018_c180)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUIStateBuyPopup::SetAttribParams(self, a1, value, label)` (0x1cafc0):
/// fill the two labelled parameter rows and their hashes.
pub fn c_ui_state_buy_popup_set_attrib_params(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = 0x4a0000;
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1408));
        c.g[S2][0] = 0x2;
        Addr(p(c.g[S0][0] as u32, 0x58)).sw(c, c.g[S1][0]);
        Addr(p(c.g[S0][0] as u32, 0x54)).sw(c, c.g[S2][0]);
        c.g[A0][0] = 0x460000;
        Addr(p(c.g[S0][0] as u32, 0x48)).sw(c, c.g[A2][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6368));
        Addr(p(c.g[S0][0] as u32, 0x4c)).sw(c, c.g[A3][0]);
        call_ptr(c, 0x0031_7670, 0x001c_b000)?;
        c.g[V1][0] = 0x3;
        Addr(p(c.g[S0][0] as u32, 0x64)).sw(c, c.g[S2][0]);
        Addr(p(c.g[S0][0] as u32, 0x68)).sw(c, c.g[S1][0]);
        Addr(p(c.g[S0][0] as u32, 0x60)).sw(c, c.g[V0][0]);
        Addr(p(c.g[S0][0] as u32, 0x5c)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cKeyboardPopup::SetStaticText(self, text)` (0x1ccf00): with no screen yet,
/// keep the string in the popup; otherwise set it on the found text object.
pub fn c_keyboard_popup_set_static_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x40)).lw(c);
        let have_screen = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if !have_screen {
            c.cycles += 4;
            c.g[A0][0] = sx(p(c.g[S0][0] as u32, 0x424));
            call_ptr(c, 0x0031_84e8, 0x001c_cf28)?;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 8;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6580));
            call_ptr(c, 0x0031_7670, 0x001c_cf3c)?;
            c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001c_cf48)?;
            let missing = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !missing {
                c.cycles += 2;
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x003a_0c50, 0x001c_cf58)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Net and buddy manager.

/// `cSetupMan` quit-event hook helpers.

/// `LobbyMan_RequestNews(self, value)` (0x25e0c8): when the lobby manager is
/// connected, format the tag-encoded "news" request and send it.
pub fn lobby_man_request_news(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x120);
        frame.save(c, S0, 0x110);
        frame.save_ra(c, 0x100);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let idle = c.g[V0][0] == 0;
        c.g[A2][0] = 0x4a0000;
        if !idle {
            c.cycles += 16;
            c.g[A3][0] = c.g[A1][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2f58));
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 0x100;
            Addr(c.g[SP][0] as u32).sb(c, 0);
            call_ptr(c, 0x003e_be40, 0x0025_e0fc)?;
            c.g[T0][0] = 0x270000;
            c.g[A2][0] = 0x6e650000;
            c.g[AT][0] = 0x41a00000;
            c.mtc1(AT, 12);
            c.g[A0][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7228));
            c.g[A1][0] = 0xe;
            c.g[A2][0] = c.g[A2][0] | 0x7773;
            c.g[A3][0] = c.g[SP][0];
            call_ptr(c, 0x0025_b608, 0x0025_e124)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMan_ResetAccountInfo(self)` (0x260f10): release the five account
/// strings and reset the timer/counter fields.
pub fn lobby_man_reset_account_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let base = c.g[S0][0] as u32;
        for (i, off) in [0x260u32, 0x264, 0x268, 0x26c, 0x270]
            .into_iter()
            .enumerate()
        {
            c.g[A0][0] = sx(p(base, off));
            call_ptr(c, 0x0031_80b8, 0x0026_0f28 + 8 * i as u32)?;
        }
        c.g[V0][0] = 1;
        c.g[V1][0] = 0x7c1;
        Addr(p(base, 0x28c)).sw(c, c.g[V0][0]);
        Addr(p(base, 0x290)).sw(c, c.g[V1][0]);
        Addr(p(base, 0x2a0)).sw(c, 0);
        Addr(p(base, 0x298)).sw(c, c.g[V0][0]);
        Addr(p(base, 0x29c)).sw(c, c.g[V0][0]);
        Addr(p(base, 0x288)).sw(c, c.g[V0][0]);
        Addr(p(base, 0x294)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared loop of the two buddy-roster counters: count entries whose
/// `IsTempAt` result equals `want_temp`.
fn buddy_man_count(
    c: &mut Cpu,
    entry: u32,
    at_head: u32,
    at_body: u32,
    at_dead: u32,
    want_temp: bool,
) -> Flow {
    let end = entry + 0x6c;
    let mut stage: u8 = match c.pc {
        pc if pc == entry => 0,
        pc if pc == at_head => 1,
        pc if pc == at_body => 2,
        pc if pc == at_dead => 3,
        pc => {
            c.jump_out(pc, entry, end);
            return Ok(());
        }
    };
    loop {
        match stage {
            0 => {
                c.cycles += 9;
                let frame = Frame::open(c, 0x40);
                frame.save(c, S0, 0x30);
                frame.save(c, S1, 0x20);
                c.g[S0][0] = 0;
                frame.save(c, S2, 0x10);
                c.g[S1][0] = 0;
                frame.save_ra(c, 0);
                c.g[S2][0] = c.g[A0][0];
                stage = 1;
            }
            1 => {
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0026_2598, at_head + 8)?;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S0][0];
                if more {
                    if back_edge_unwinds(c, at_body) {
                        return Ok(());
                    }
                    stage = 2;
                } else {
                    stage = 8;
                }
            }
            2 => {
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0026_2400, at_body + 8)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                let hit = if want_temp {
                    c.g[V0][0] != 0
                } else {
                    c.g[V0][0] == 0
                };
                if hit {
                    c.g[S1][0] = c.g[V1][0];
                }
                stage = 1;
            }
            3 => {
                c.cycles += 1;
                stage = 2;
            }
            _ => {
                c.cycles += 7;
                c.g[V0][0] = c.g[S1][0];
                let frame = Frame { size: 0x40 };
                frame.restore(c, S0, 0x30);
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
    }
}

/// `int BuddyMan_CountPermanentEntries(BuddyMan* self)` (0x262280): count the
/// roster entries that are not temporary.
pub fn buddy_man_count_permanent_entries(c: &mut Cpu) {
    guest(c, |c| {
        buddy_man_count(c, 0x0026_2280, 0x0026_22bc, 0x0026_22a8, 0x0026_22a4, false)
    })
}

/// `int BuddyMan_CountTempEntries(BuddyMan* self)` (0x2622f0): count the
/// roster entries that are temporary.
pub fn buddy_man_count_temp_entries(c: &mut Cpu) {
    guest(c, |c| {
        buddy_man_count(c, 0x0026_22f0, 0x0026_232c, 0x0026_2318, 0x0026_2314, true)
    })
}

/// `void cSharedLoginMan_Init(cSharedLoginMan* self)` (0x265950): publish the
/// empty tag string, bring up the shared login state, zero the card string
/// and create the memory-card object.
pub fn c_shared_login_man_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        c.g[V1][0] = 0x440000;
        Addr(c.g[A0][0] as u32).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2a18));
        Addr(gp(c, 0x2e4)).sw(c, 0);
        Addr(p(c.g[A0][0] as u32, 4)).sw(c, 0);
        Addr(gp(c, 0x2a58)).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx(gp(c, 0x2a58));
        Addr(gp(c, 0x2f0)).sw(c, 0);
        Addr(gp(c, 0x2ec)).sw(c, 0);
        Addr(gp(c, 0x2f4)).sw(c, 0);
        Addr(gp(c, 0x2a5c)).sw(c, 0);
        call_ptr(c, 0x003f_7ee0, 0x0026_5990)?;
        c.g[A0][0] = 0x530000;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x40;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7190));
        call_ptr(c, 0x003e_6448, 0x0026_59a4)?;
        call_ptr(c, 0x002c_22c8, 0x0026_59ac)?;
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        Addr(gp(c, 0x2e8)).sw(c, c.g[V0][0]);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

/// `NetEvent102Down_Clone(ev)` (0x2683f0): allocate a 0x10-byte event clone,
/// tag it and copy the kind string at `+0xc`.
pub fn net_event102_down_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x480000;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x2000_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x10;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        call_ptr(c, 0x0031_7d70, 0x0026_8418)?;
        c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0xc)).lw(c);
        c.g[A0][0] = 0x480000;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x11b0));
        c.g[V1][0] = 0x646f0000;
        Addr(p(c.g[S0][0] as u32, 4)).sw(c, c.g[A0][0]);
        c.g[V1][0] = c.g[V1][0] | 0x776e;
        c.g[V0][0] = 0x102;
        Addr(p(c.g[S0][0] as u32, 8)).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx(p(c.g[S0][0] as u32, 0xc));
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_82d8, 0x0026_8448)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `NetEventFC_Clone(ev)` (0x268720): allocate a 0x14-byte clone and copy the
/// three payload words.
pub fn net_event_fc_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x480000;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x14;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_8748)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[A3][0] = Addr(p(c.g[S0][0] as u32, 0x10)).lw(c);
        c.g[A2][0] = Addr(p(c.g[S0][0] as u32, 8)).lw(c);
        c.g[V0][0] = 0xfc;
        c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0xc)).lw(c);
        c.g[A0][0] = 0x480000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1130));
        Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = c.g[V1][0];
        frame.restore_ra(c, 0);
        Addr(p(c.g[V1][0] as u32, 4)).sw(c, c.g[A0][0]);
        Addr(p(c.g[V1][0] as u32, 8)).sw(c, c.g[A2][0]);
        Addr(p(c.g[V1][0] as u32, 0xc)).sw(c, c.g[A1][0]);
        Addr(p(c.g[V1][0] as u32, 0x10)).sw(c, c.g[A3][0]);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio.

/// `CrowdAudio::InstanceRemoveAllSources(self, fade)` (0x2a6430): remove the
/// fifteen crowd-audio instance slots.
pub fn crowd_audio_instance_remove_all_sources(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x20));
        frame.save_fpr(c, 20, 0x40);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            let empty = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S2][0];
            if !empty {
                c.cycles += 3;
                c.g[A1][0] = c.g[S1][0];
                c.mov_s(12, 20);
                call_ptr(c, 0x002a_62f0, 0x002a_6470)?;
            }
            c.cycles += 4;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S1][0] as i64) < 15i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x002a_6458);
                }
            } else {
                c.cycles += 7;
                frame.restore(c, S0, 0x30);
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.restore_fpr(c, 20, 0x40);
                frame.close(c);
                return Ok(());
            }
        }
    })
}

/// `SoundManagerRef_dtor(self, flags)` (0x2adee8): destroy the referenced
/// sound manager and free the reference.
pub fn sound_manager_ref_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x480000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3c00));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        Addr(p(c.g[S1][0] as u32, 4)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x18)).lh(c);
        c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x1c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002a_df24)?;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
        call_ptr(c, 0x0031_7e50, 0x002a_df2c)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let no_delete = c.g[S0][0] == 0;
        Addr(gp(c, 0x68c)).sw(c, 0);
        if !no_delete {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x002a_df40)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SpeechInstance::AddBank(self, a, b, c)` (0x2b0348): load a speech bank,
/// append its id and bump the global bank counter.
pub fn speech_instance_add_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = c.g[A2][0];
        c.g[A2][0] = c.g[A3][0];
        call_ptr(c, 0x002a_f960, 0x002b_0368)?;
        c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0xf0)).lw(c);
        c.g[A0][0] = sx(p(c.g[S0][0] as u32, 0xf4));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
        c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0xf0)).lw(c);
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
        let empty = c.g[V1][0] == 0;
        c.g[V0][0] = Addr(gp(c, 0x6c8)).lw(c);
        if !empty {
            c.cycles += 4;
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            Addr(p(c.g[S0][0] as u32, 0xf0)).sw(c, c.g[V1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            Addr(gp(c, 0x6c8)).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SongInstance::FadeOut(self, seconds)` (0x2b2418): fade the song's stream
/// over `seconds` (locked against the sound thread) and mark the instance
/// faded.
pub fn song_instance_fade_out(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        call_ptr(c, 0x003b_58a0, 0x002b_2434)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002b_2488, 0x002b_243c)?;
        let no_stream = (c.g[V0][0] as i64) < 0;
        c.g[A0][0] = c.g[V0][0];
        if !no_stream {
            c.cycles += 7;
            c.g[AT][0] = 0x447a0000;
            c.mtc1(AT, 0);
            c.mul_s(0, 20, 0);
            c.cvt_w_s(1, 0);
            c.mfc1(A1, 1);
            c.g[A2][0] = 0;
            call_ptr(c, 0x003b_7eb0, 0x002b_2460)?;
        }
        c.cycles += 9;
        call_ptr(c, 0x003b_58d8, 0x002b_2468)?;
        c.g[V0][0] = 1;
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        Addr(p(c.g[S0][0] as u32, 0x48)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_fpr(c, 20, 0x20);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    })
}

/// `SSXAudioSystem::LoadMemoryBank(self, id, data)` (0x286ca8): load one
/// memory-resident sound bank and mark it in the loaded/sub tables.
pub fn ssx_audio_system_load_memory_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let absent = c.g[A2][0] == 0;
        c.g[S1][0] = sx((c.g[S2][0] as u32) >> 8);
        if !absent {
            c.cycles += 12;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x118)).lw(c);
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(8));
            c.g[A3][0] = 0x20000;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
            call_ptr(c, 0x0028_be60, 0x0028_6ce0)?;
            c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[A1][0] = 1;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = c.g[S2][0] & 0xff;
            c.g[V1][0] = c.g[A0][0];
            Addr(p(c.g[A0][0] as u32, 0x6c70)).sw(c, c.g[A1][0]);
            Addr(p(c.g[V1][0] as u32, 0x6c78)).sw(c, c.g[V0][0]);
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

/// `SSXAudioSystem::RestartPeakMusic(self)` (0x28ef90): reset the peak-music
/// state, pick a random song and arm its timer.
pub fn ssx_audio_system_restart_peak_music(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(c.g[S0][0] as u32, 0x6258)).sw(c, 0);
        Addr(p(c.g[S0][0] as u32, 0x577c)).sw(c, 0);
        call_ptr(c, 0x0028_df08, 0x0028_efac)?;
        Addr(p(c.g[S0][0] as u32, 0x578c)).sw(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0028_d488, 0x0028_efb8)?;
        c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x118)).lw(c);
        c.g[V0][0] = 0x4a0000;
        c.ldl(A2, 0x004a_364f);
        c.ldr(A2, 0x004a_3648);
        c.g[A1][0] = 0xa;
        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0x2;
        c.g[T1][0] = 0x1;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        call_ptr(c, 0x002a_dca0, 0x0028_efec)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Ini parser, script engine, replay and codec.

/// `IniParser_SkipSectionHeaders(self)` (0x2a7a20): skip whitespace, then walk
/// past consecutive `[...]` section headers, ending at the section end.
pub fn ini_parser_skip_section_headers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x002a_78f8, 0x002a_7a38)?;
        c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 8)).lw(c);
        c.g[A0][0] = 0x5b;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lb(c);
        let not_header = c.g[V0][0] != c.g[A0][0];
        c.g[S1][0] = 0x5b;
        if !not_header {
            c.cycles += 1;
            loop {
                c.cycles += 8;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002a_7b08, 0x002a_7a58)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002a_78f8, 0x002a_7a60)?;
                c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 8)).lw(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lb(c);
                let another = c.g[V0][0] == c.g[S1][0];
                if !another {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002a_7a50);
                }
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002a_7f90, 0x002a_7a78)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptEngine::CancelBankLoad(self, index)` (0x27b4b8): cancel and release
/// the async load in slot `index` and clear the slot.
pub fn script_engine_cancel_bank_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 4);
        frame.save(c, S1, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x628));
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 8)).lw(c);
        let empty = c.g[A0][0] == 0;
        if !empty {
            c.cycles += 12;
            call_ptr(c, 0x003d_faf0, 0x0027_b4ec)?;
            c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 8)).lw(c);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x003d_f9d8, 0x0027_b4fc)?;
            c.g[V1][0] = Addr(p(c.g[S1][0] as u32, 0x620)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            Addr(p(c.g[S1][0] as u32, 0x620)).sw(c, c.g[V1][0]);
            call_ptr(c, 0x0027_7f08, 0x0027_b510)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Replay_BeginSave(self)` (0x270e68): allocate the save buffers and clear
/// the replay writer state (ending in the `0xdeadbeef` sentinel).
pub fn replay_begin_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0027_0f20, 0x0027_0e7c)?;
        c.g[V0][0] = 0xffff_ffff_dead_0000;
        Addr(p(c.g[S0][0] as u32, 0x638)).sw(c, 0);
        c.g[V0][0] = c.g[V0][0] | 0xbeef;
        Addr(p(c.g[S0][0] as u32, 0x644)).sw(c, 0);
        Addr(p(c.g[S0][0] as u32, 0x684)).sw(c, c.g[V0][0]);
        for off in [
            0x648u32, 0x64c, 0x650, 0x654, 0x658, 0x65c, 0x660, 0x664, 0x66c, 0x674, 0x678, 0x67c,
            0x680,
        ] {
            Addr(p(c.g[S0][0] as u32, off)).sw(c, 0);
        }
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RCMP_DECODER_ctor(self, key)` (0x3b0430): install the vtable, initialise
/// the callback table and copy the 16-byte key.
pub fn rcmp_decoder_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x450000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x68a8));
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = sx(p(c.g[S0][0] as u32, 4));
        Addr(p(c.g[S0][0] as u32, 0x18)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x003b_0738, 0x003b_045c)?;
        c.ldl(V0, p(c.g[S1][0] as u32, 7));
        c.ldr(V0, c.g[S1][0] as u32);
        c.ldl(V1, p(c.g[S1][0] as u32, 0xf));
        c.ldr(V1, p(c.g[S1][0] as u32, 8));
        c.sdl(V0, p(c.g[S0][0] as u32, 0xb));
        c.sdr(V0, p(c.g[S0][0] as u32, 4));
        c.sdl(V1, p(c.g[S0][0] as u32, 0x13));
        c.sdr(V1, p(c.g[S0][0] as u32, 0xc));
        c.g[V0][0] = c.g[S0][0];
        Addr(p(c.g[S0][0] as u32, 0x14)).sw(c, 0);
        Addr(c.g[S0][0] as u32).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MPEG2_Flush_Buffer(BitReader* unused, int n)` (0x3b1290): shift the
/// bit-reader buffer left by `n` bits and pull whole bytes in until it holds
/// at least 25 bits.
pub fn mpeg2_flush_buffer(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V1][0] = 0x510000;
    c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x5f78)).lw(c);
    c.g[T2][0] = c.g[V1][0];
    c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 4)).lw(c);
    c.g[V1][0] = Addr(p(c.g[A1][0] as u32, 8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A0][0] as u32) & 31));
    c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
    c.g[V1][0] = ((c.g[T0][0] as i64) < 25i64) as u64;
    let no_loop = c.g[V1][0] == 0;
    Addr(p(c.g[A1][0] as u32, 4)).sw(c, c.g[V0][0]);
    if !no_loop {
        c.cycles += 2;
        c.g[A3][0] = c.g[A1][0];
        c.g[T1][0] = 0x18;
        loop {
            c.cycles += 12;
            c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
            c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[T0][0] as u32));
            c.g[A0][0] = Addr(p(c.g[A3][0] as u32, 4)).lw(c);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(8));
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lbu(c);
            c.g[A2][0] = ((c.g[T0][0] as i64) < 25i64) as u64;
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
                deliver_event_here(c, 0x003b_12c0);
            }
        }
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[T2][0] as u32).wrapping_sub(0x5f78)).lw(c);
    let ra = c.g[RA][0] as u32;
    Addr(p(c.g[V0][0] as u32, 8)).sw(c, c.g[T0][0]);
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Menu system.

/// The shared handler of the back and callback menu items: run the base
/// input handler, then (when enabled and not already consumed) pop the page
/// for the "back" code, or run the item's callback.
fn menu_item_handle_input(c: &mut Cpu, entry: u32, callback: bool) -> Flow {
    c.cycles += 11;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    call_ptr(c, 0x002c_a2b0, entry + 0x18)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x002c_a3b0, entry + 0x24)?;
    let disabled = c.g[V0][0] != 0;
    c.g[V0][0] = c.g[S1][0];
    let mut restored_early = false;
    if !disabled {
        c.cycles += 2;
        if c.g[S1][0] != 0 {
            frame.restore(c, S0, 0x20);
            restored_early = true;
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002c_a3e8, entry + 0x3c)?;
            c.g[V1][0] = 6;
            let not_back = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = c.g[S1][0];
            if !not_back {
                c.cycles += 4;
                if callback {
                    c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x18)).lw(c);
                    let target = c.g[V0][0] as u32;
                    call_ptr(c, target, entry + 0x54)?;
                    c.g[V0][0] = c.g[S1][0];
                } else {
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x124)).lw(c);
                    call_ptr(c, 0x002c_c460, entry + 0x54)?;
                    c.g[V0][0] = c.g[S1][0];
                }
            }
        }
    }
    if !restored_early {
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `MenuSysBackItem::HandleInput(self)` (0x2cce38): pop the current page when
/// the "back" input arrives.
pub fn menu_sys_back_item_handle_input(c: &mut Cpu) {
    guest(c, |c| menu_item_handle_input(c, 0x002c_ce38, false))
}

/// `MenuSysCallbackItem::HandleInput(self)` (0x2cd1d0): call the item's
/// callback when the "back" input arrives.
pub fn menu_sys_callback_item_handle_input(c: &mut Cpu) {
    guest(c, |c| menu_item_handle_input(c, 0x002c_d1d0, true))
}

/// `MenuSysTextEditItem* MenuSysTextEditItem_ctor(self, a1, name)`
/// (0x2ce418): base menu item plus the text-edit vtable and the name copy.
pub fn menu_sys_text_edit_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        c.g[A1][0] = u64::MAX;
        call_ptr(c, 0x002c_a258, 0x002c_e440)?;
        c.g[V1][0] = 0x480000;
        Addr(p(c.g[S0][0] as u32, 0x14)).sw(c, c.g[S1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ac0));
        Addr(p(c.g[S0][0] as u32, 0x28)).sw(c, 0);
        Addr(p(c.g[S0][0] as u32, 0x10)).sw(c, c.g[V1][0]);
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = sx(p(c.g[S0][0] as u32, 0x18));
        c.g[A2][0] = 0xf;
        call_ptr(c, 0x0041_6ce0, 0x002c_e464)?;
        c.g[V0][0] = c.g[S0][0];
        Addr(p(c.g[S0][0] as u32, 0x27)).sb(c, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Camera and memory card.

/// `ChaseCamController::EnterSpokeMode(self)` (0x1621a8): set the spoke-mode
/// flag and blend to the spoke camera algorithm at 1/60 s per frame (1.2 for
/// the "stuck" rider state, 0.6 otherwise).
pub fn chase_cam_controller_enter_spoke_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x30)).lw(c);
        c.g[V0][0] = c.g[V0][0] | 0x4;
        c.g[V1][0] = c.g[V0][0] & 0x3;
        let already = c.g[V1][0] != 0;
        Addr(p(c.g[S0][0] as u32, 0x30)).sw(c, c.g[V0][0]);
        if !already {
            c.cycles += 7;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x20)).lw(c);
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 4)).lw(c);
            call_ptr(c, 0x0011_fe98, 0x0016_21d8)?;
            c.g[V1][0] = 0x5;
            c.lwc1(0, gp(c, -0x6b20));
            let stuck = c.g[V0][0] == c.g[V1][0];
            if stuck {
                c.cycles += 1;
                c.lwc1(0, gp(c, -0x6b1c));
            }
            c.cycles += 6;
            c.lwc1(12, gp(c, -0x6b18));
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x42;
            c.g[A2][0] = 0;
            c.mul_s(12, 0, 12);
            call_ptr(c, 0x0015_d078, 0x0016_2204)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cPS2MemCard_dtor(self, flags)` (0x2c3fe0): close the card, free the buffer,
/// install the base vtable and free the object when asked.
pub fn c_ps2_mem_card_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x480000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6f78));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        call_ptr(c, 0x002c_6360, 0x002c_4008)?;
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x30)).lw(c);
        let no_buffer = c.g[A0][0] == 0;
        c.g[V0][0] = 0x480000;
        if !no_buffer {
            c.cycles += 3;
            call_ptr(c, 0x0025_23a8, 0x002c_401c)?;
            c.g[V0][0] = 0x480000;
        }
        c.cycles += 4;
        c.g[V1][0] = c.g[S1][0] & 0x1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x71e8));
        let keep = c.g[V1][0] == 0;
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x002c_4038)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MCManager::State38(self)` (0x23e428): when the movie player is idle and
/// the operation finished, leave the operation and return to the slot state.
pub fn mc_manager_state38(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x434)).lw(c);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x58)).lh(c);
        c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x5c)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_e450)?;
        let busy = c.g[V0][0] != 0;
        if busy {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x6c)).lw(c);
            let target = c.g[V0][0] as u32;
            call_ptr(c, target, 0x0023_e464)?;
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x0023_ea30, 0x0023_e478)?;
                Addr(p(c.g[S0][0] as u32, 0xc8)).sw(c, 0);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0024_2050, 0x0023_e484)?;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rendering and fonts.

/// `Font_DrawTextRight(font, text, x, y)` (0x392a60): measure the string,
/// subtract its width from x and draw it.
pub fn font_draw_text_right(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 21, 0x38);
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(21, 13);
        c.mov_s(20, 12);
        c.lwc1(12, p(c.g[S0][0] as u32, 0x38));
        c.lwc1(13, p(c.g[S0][0] as u32, 0x3c));
        call_ptr(c, 0x0039_1fb0, 0x0039_2a9c)?;
        c.sub_s(12, 20, 0);
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(13, 21);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_1cb0, 0x0039_2ab0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `Renderer_BuildSineTable(self)` (0x392df0): fill 640 floats with
/// `sin(i * 2*pi/512)`.
pub fn renderer_build_sine_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x10);
        frame.save_fpr(c, 20, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[S2][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        c.lwc1(20, gp(c, -0x2694));
        loop {
            c.cycles += 9;
            c.mtc1(S1, 12);
            c.cvt_s_w(12, 12);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.mul_s(12, 12, 20);
            call_ptr(c, 0x0031_bf60, 0x0039_2e2c)?;
            c.swc1(0, c.g[S0][0] as u32);
            c.g[V0][0] = ((c.g[S1][0] as i64) < 640i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0039_2e18);
                }
            } else {
                c.cycles += 8;
                c.g[V0][0] = c.g[S2][0];
                frame.restore(c, S0, 0x30);
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.restore_fpr(c, 20, 0x40);
                frame.close(c);
                return Ok(());
            }
        }
    })
}

/// `TexMan_InitTable(self)` (0x3670e0): zero the table header and link the
/// 499 free-list entries.
pub fn tex_man_init_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x1f40;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(c.g[S0][0] as u32).sw(c, 0);
        c.g[A0][0] = sx(p(c.g[S0][0] as u32, 8));
        Addr(p(c.g[S0][0] as u32, 4)).sw(c, 0);
        call_ptr(c, 0x003e_6448, 0x0036_7108)?;
        c.g[V0][0] = sx(p(c.g[S0][0] as u32, 0x1778));
        c.g[A1][0] = 0;
        Addr(p(c.g[S0][0] as u32, 0x1f48)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 0x4;
        loop {
            c.cycles += 9;
            c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x1f48)).lw(c);
            c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            Addr(c.g[V0][0] as u32).sw(c, c.g[V1][0]);
            c.g[V0][0] = ((c.g[A1][0] as i64) < 499i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
            if !more {
                c.cycles += 4;
                frame.restore(c, S0, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_7118);
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Modifiers and misc leaf-ish helpers.

/// `TexFlipModifier_ctor_2(self, obj)` (0x35f1a8): read the flip mode and
/// frame count through `obj`'s vtable, store the count and finish the base
/// modifier construction.
pub fn tex_flip_modifier_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x18;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x10)).lh(c);
        c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x14)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_f1dc)?;
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x18)).lh(c);
        c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x1c)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_f1f0)?;
        Addr(p(c.g[S1][0] as u32, 0x18)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0035_f218, 0x0035_f1fc)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Floating_v51(self)` (0x361858): copy the transformed value through the
/// object's two vtable entries and clear the "floating" flag bit.
pub fn floating_v51(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = Addr(p(c.g[S2][0] as u32, 0xc)).lw(c);
        c.g[S1][0] = Addr(p(c.g[S0][0] as u32, 0x108)).lh(c);
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0xc0)).lh(c);
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0xc4)).lw(c);
        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S1][0] as u32));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0036_188c)?;
        c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x10c)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = Addr(p(c.g[S2][0] as u32, 0x74)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, target, 0x0036_18a0)?;
        c.g[V0][0] = Addr(p(c.g[S2][0] as u32, 0x12)).lhu(c);
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = c.g[V0][0] & 0xfffe;
        frame.restore(c, S1, 0x20);
        Addr(p(c.g[S2][0] as u32, 0x12)).sh(c, c.g[V0][0]);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptMan::DeactivateMission(self, node)` (0x309b00): move a still-active
/// node to the front of the process list and clear the current mission id.
pub fn w_script_man_deactivate_mission(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = sx(p(c.g[S2][0] as u32, 0x2b8));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0030_b428, 0x0030_9b28)?;
        let missing = c.g[V0][0] == 0;
        if !missing {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0030_b2d0, 0x0030_9b3c)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = sx(p(c.g[S2][0] as u32, 0x2b4));
            call_ptr(c, 0x0030_b260, 0x0030_9b48)?;
        }
        c.cycles += 9;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        frame.restore(c, S1, 0x20);
        Addr(p(c.g[S2][0] as u32, 0x2a0)).sw(c, c.g[V0][0]);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MagnetModifier_v09(self, id)` (0x357660): return the cached value when the
/// magnet already tracks `id`; otherwise ask the rider world object whether the
/// id is valid and remember it.
pub fn magnet_modifier_v09(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0x44)).lw(c);
        let unset = (c.g[V0][0] as i64) < 0;
        c.g[S0][0] = c.g[A1][0];
        if !unset {
            c.cycles += 2;
            let different = c.g[V0][0] != c.g[S0][0];
            c.g[V0][0] = 0;
            if different {
                // cached miss
            } else {
                c.cycles += 2;
                c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0x54)).lw(c);
            }
        } else {
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002d_1b08, 0x0035_7698)?;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x40)).lh(c);
            c.g[A1][0] = Addr(p(c.g[V1][0] as u32, 0x44)).lw(c);
            let target = c.g[A1][0] as u32;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0035_76ac)?;
            if c.g[V0][0] != 0 {
                Addr(p(c.g[S1][0] as u32, 0x44)).sw(c, c.g[S0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
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
// Lobby menu.

/// `LobbyMenu::EnterRoom(self, room)` (0x17e2f8): pop every page off the page
/// stack, move to room `room` and set state 0xb.
pub fn lobby_menu_enter_room(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 8)).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = sx(p(c.g[S1][0] as u32, 8));
        if !empty {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_c460, 0x0017_e328)?;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                let more = c.g[V0][0] != 0;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_e320);
                }
            }
        }
        c.cycles += 12;
        c.g[V0][0] = 0xb;
        c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
        Addr(p(c.g[S1][0] as u32, 4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = 0;
        call_ptr(c, 0x0025_d800, 0x0017_e34c)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu::CancelChallenge(self)` (0x17e558): pop every page off the page
/// stack, push the challenge list, set state 0xc and cancel the challenge.
pub fn lobby_menu_cancel_challenge(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 8)).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = sx(p(c.g[S1][0] as u32, 8));
        if !empty {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_c460, 0x0017_e580)?;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                let more = c.g[V0][0] != 0;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_e578);
                }
            }
        }
        c.cycles += 14;
        c.g[A0][0] = sx(p(c.g[S1][0] as u32, 8));
        c.g[A1][0] = sx(p(c.g[S1][0] as u32, 0x1044));
        call_ptr(c, 0x002c_c3b8, 0x0017_e598)?;
        c.g[V0][0] = 0xc;
        c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
        Addr(p(c.g[S1][0] as u32, 4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 1;
        c.g[A2][0] = 0;
        call_ptr(c, 0x0025_f628, 0x0017_e5b0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Video player, AI riders and renderer misc.

/// `cFEStateVideoPlayer::Update(self)` (0x1d24a8): advance the movie player and
/// finish the state when the movie ends.
pub fn c_fe_state_video_player_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x248)).lw(c);
        let absent = c.g[A0][0] == 0;
        c.g[V0][0] = 1;
        if !absent {
            c.cycles += 5;
            Addr(p(c.g[S0][0] as u32, 0x274)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x0025_3938, 0x001d_24cc)?;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x27c)).lw(c);
            let mut finish;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x248)).lw(c);
                c.cycles += 3;
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                finish = c.g[V1][0] != 0;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x278)).lw(c);
                if c.g[V0][0] != 0 {
                    finish = true;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x248)).lw(c);
                    c.cycles += 3;
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                    finish = c.g[V1][0] != 0;
                }
            }
            if finish {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x001d_2638, 0x001d_24fc)?;
            }
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_e510, 0x001d_2504)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AI_LoadNextRider(self)` (0x1296f8): find the first roster rider that is
/// not loaded and load its step. A `match pc` dispatcher re-enterable at its
/// loop head (0x129720) after an unwind.
pub fn ai_load_next_rider(c: &mut Cpu) {
    const ENTRY: u32 = 0x0012_96f8;
    const END: u32 = 0x0012_9764;
    const AT_HEAD: u32 = 0x0012_9720;
    const AT_DEAD: u32 = 0x0012_971c;
    guest(c, |c| {
        let mut stage = match c.pc {
            ENTRY => 0u8,
            AT_HEAD => 1,
            AT_DEAD => 6,
            pc => {
                c.jump_out(pc, ENTRY, END);
                return Ok(());
            }
        };
        // The frame is opened at the entry stage; resume stages reuse it.
        let mut frame = Frame { size: 0x40 };
        loop {
            match stage {
                0 => {
                    c.cycles += 9;
                    frame = Frame::open(c, 0x40);
                    frame.save(c, S1, 0x20);
                    frame.save(c, S0, 0x30);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, S2, 0x10);
                    c.g[S0][0] = sx(p(c.g[S1][0] as u32, 0x28));
                    frame.save_ra(c, 0);
                    c.g[S2][0] = 0;
                    stage = 2;
                }
                2 => {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0x78)).lw(c);
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        frame.restore(c, S0, 0x30);
                        stage = 5;
                    } else {
                        stage = 3;
                    }
                }
                3 => {
                    c.cycles += 4;
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    call_ptr(c, 0x0011_d640, 0x0012_973c)?;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                        if back_edge_unwinds(c, AT_HEAD) {
                            return Ok(());
                        }
                        stage = 1;
                    } else {
                        stage = 4;
                    }
                }
                4 => {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    call_ptr(c, 0x0011_c298, 0x0012_974c)?;
                    frame.restore(c, S0, 0x30);
                    stage = 5;
                }
                1 => {
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                    stage = 2;
                }
                6 => {
                    c.cycles += 1;
                    stage = 1;
                }
                _ => {
                    c.cycles += 5;
                    let frame = Frame { size: 0x40 };
                    frame.restore(c, S1, 0x20);
                    frame.restore(c, S2, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, vtable, FAKE};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use ssx3_ee as ee;

    /// The object under test.
    const OBJ: u32 = DATA;
    /// A second scratch object.
    const OBJ2: u32 = DATA + 0x800;
    /// An event/message object.
    const EV: u32 = DATA + 0x2000;
    /// The zeroed BELibrary dummy.
    const BE: u32 = DATA + 0x3_0000;
    /// The dummy world.
    const WORLD: u32 = DATA + 0x8_0000;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// The BELibrary singleton (`gp - 0x1f04`, a zeroed dummy so nothing reads
    /// the disc) and the world (`gp - 0x848`, with no riders).
    fn globals(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x848), WORLD);
        let game = WORLD + 0x1000;
        w(c, WORLD + 0x84, game);
        let race = WORLD + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }

    /// The engine's size-class pool allocator: `gp + 0xdd8` ready plus eight
    /// free nodes per size class (a zeroed heap walks garbage).
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
        w(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            // One 0x40000-byte region per pool (its largest head is at 0x32000)
            // and one node array well away from it, so the head slots never
            // overwrite a free node.
            let pool = DATA + 0x40_0000 + 0x4_0000 * i;
            let base = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base + 0x400 * n;
                w(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w(c, pool + head, base);
            w(c, GAME_GP + slot, pool);
        }
    }

    /// Scrambled state, fake call hooks, a game-like `gp` and the standard
    /// fixtures.
    fn base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        globals(c);
        pool(c);
        r
    }

    /// A screen whose hash table holds `node` with hash 0 (`StrHash("")` is 0
    /// on the test machine: the tag strings are not loaded).
    fn hash_object(c: &mut Cpu, screen: u32, node: u32) {
        w(c, screen + 0x38, DATA + 0x30_0100);
        w(c, DATA + 0x30_0100, 1);
        w(c, screen + 0x3c, DATA + 0x30_0000);
        w(c, DATA + 0x30_0000, node);
        w(c, node + 0x38, 0);
    }

    /// The UI `*_Init` pattern: a state with a dummy engine, the pool and the
    /// globals.
    fn ui_init_setup(c: &mut Cpu, seed: u32, salt: u64) {
        machine(c, seed, salt, 3);
        globals(c);
        pool(c);
        c.g[A0][0] = OBJ as u64;
        w(c, OBJ + 0x10, DATA + 0x1000);
    }

    // ---------------------------------------------------------------- front end

    #[test]
    fn c_fe_state_online_character_select_on_quit_popup_event_matches() {
        check(
            ee::s03::f_001b1f18__generated,
            c_fe_state_online_character_select_on_quit_popup_event,
            8,
            &|c, seed| {
                let mut r = base(c, seed, 0x1b1f18, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = EV as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0xf
                } else {
                    (r.next_u32() & 0x1f) as u64
                };
                w(c, EV + 0x18, 0);
                w(c, OBJ + 0x69c, 0);
                if seed % 4 >= 2 {
                    w(c, GAME_GP.wrapping_add(0x2dc), OBJ2);
                    w(c, OBJ2, 0xa);
                } else {
                    w(c, GAME_GP.wrapping_add(0x2dc), 0);
                }
            },
        );
    }

    #[test]
    fn c_fe_state_auth_dnas_error_ctor_matches() {
        check(
            ee::s03::f_001b6ab8__generated,
            c_fe_state_auth_dnas_error_ctor,
            6,
            &|c, seed| {
                let mut r = base(c, seed, 0x1b6ab8, 2);
                c.g[A0][0] = OBJ as u64;
                for i in 0..5 {
                    c.g[[A1, A2, A3, T0, T1][i]][0] = r.next_u32() as u64;
                }
            },
        );
    }

    #[test]
    fn c_fe_state_create_step2_on_parent_email_entered_matches() {
        check(
            ee::s03::f_001bb0e8__generated,
            c_fe_state_create_step2_on_parent_email_entered,
            8,
            &|c, seed| {
                let mut r = base(c, seed, 0x001b_b0e8, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = EV as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0xf
                } else {
                    (r.next_u32() & 0x1f) as u64
                };
                w(c, EV + 0x18, 0);
                let screen = DATA + 0x3800;
                w(c, OBJ + 0x6d0, screen);
                w(c, EV + 0x2b0, DATA + 0x3900);
                c.bus.write8(DATA + 0x3900, b'x');
                c.bus.write8(DATA + 0x3901, 0);
                w(c, screen + 0x10, 0);
            },
        );
    }

    #[test]
    fn c_fe_state_create_step2_set_month_selection_matches() {
        step2_selection_test(
            ee::s03::f_001bb2d0__generated,
            c_fe_state_create_step2_set_month_selection,
            0x001b_b2d0,
            0x28,
        );
    }

    #[test]
    fn c_fe_state_create_step2_set_day_selection_matches() {
        step2_selection_test(
            ee::s03::f_001bb340__generated,
            c_fe_state_create_step2_set_day_selection,
            0x001b_b340,
            0x2c,
        );
    }

    #[test]
    fn c_fe_state_create_step2_set_gender_selection_matches() {
        step2_selection_test(
            ee::s03::f_001bb440__generated,
            c_fe_state_create_step2_set_gender_selection,
            0x001b_b440,
            0x34,
        );
    }

    fn step2_selection_test(gen: GuestFn, hand: GuestFn, salt: u64, value_off: u32) {
        check(gen, hand, 8, &|c, seed| {
            let mut r = base(c, seed, salt, 4);
            c.g[A0][0] = OBJ as u64;
            let screen = DATA + 0x3800;
            let list = DATA + 0x3a00;
            w(c, OBJ + 0x40, screen);
            w(c, OBJ + 0x6d0, list);
            let val = 0x11 + (r.next_u32() % 0x40);
            if seed % 2 == 0 {
                let node = DATA + 0x3c00;
                hash_object(c, screen, node);
                w(c, node + 0x318, 1);
                w(c, node + 0xa4, val);
                w(c, list + value_off, val);
            } else {
                w(c, screen + 0x3c, 0);
            }
        });
    }

    fn presence_test(gen: GuestFn, hand: GuestFn, salt: u64) {
        check(gen, hand, 8, &|c, seed| {
            let _ = base(c, seed, salt, 6);
            if seed % 2 == 0 {
                w(c, GAME_GP + 0x238, 0);
            } else {
                let bm = DATA + 0x3000;
                w(c, GAME_GP + 0x238, bm);
                w(c, bm + 0xc8, 1);
                w(c, bm + 0xd8, 0);
                w(c, bm + 0xe0, 1);
            }
        });
    }

    #[test]
    fn c_fe_state_lobby_menu_v32_matches() {
        presence_test(ee::s03::f_001bcca8__generated, c_fe_state_lobby_menu_v32, 0x001b_cca8);
    }

    #[test]
    fn c_fe_state_stats_v32_matches() {
        presence_test(ee::s03::f_001c2e70__generated, c_fe_state_stats_v32, 0x001c_2e70);
    }

    #[test]
    fn c_fe_state_net_news_v32_matches() {
        presence_test(ee::s03::f_001c5108__generated, c_fe_state_net_news_v32, 0x001c_5108);
    }

    #[test]
    fn c_fe_state_login_init_matches() {
        check(ee::s03::f_001beed0__generated, c_fe_state_login_init, 4, &|c, seed| {
            ui_init_setup(c, seed, 0x001b_eed0);
        });
    }

    #[test]
    fn c_fe_state_stats_init_matches() {
        check(ee::s03::f_001c2e00__generated, c_fe_state_stats_init, 4, &|c, seed| {
            ui_init_setup(c, seed, 0x001c_2e00);
        });
    }

    #[test]
    fn c_fe_popup_confirm_v04_matches() {
        check(
            ee::s03::f_001c5af8__generated,
            c_fe_popup_confirm_v04,
            4,
            &|c, seed| {
                ui_init_setup(c, seed, 0x001c_5af8);
            },
        );
    }

    #[test]
    fn c_fe_state_options_hud_init_matches() {
        check(
            ee::s02::f_0018c128__generated,
            c_fe_state_options_hud_init,
            4,
            &|c, seed| {
                ui_init_setup(c, seed, 0x0018_c128);
            },
        );
    }

    #[test]
    fn c_ui_state_buy_popup_set_attrib_params_matches() {
        check(
            ee::s04::f_001cafc0__generated,
            c_ui_state_buy_popup_set_attrib_params,
            6,
            &|c, seed| {
                let mut r = base(c, seed, 0x001c_afc0, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A2][0] = (r.next_u32() & 0xffff) as u64;
                c.g[A3][0] = (r.next_u32() & 0xffff) as u64;
            },
        );
    }

    #[test]
    fn c_keyboard_popup_set_static_text_matches() {
        check(
            ee::s04::f_001ccf00__generated,
            c_keyboard_popup_set_static_text,
            8,
            &|c, seed| {
                let mut r = base(c, seed, 0x001c_cf00, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (DATA + 0x4000) as u64;
                c.bus.write8(DATA + 0x4000, b'a');
                c.bus.write8(DATA + 0x4001, 0);
                let screen = DATA + 0x3800;
                match seed % 3 {
                    0 => w(c, OBJ + 0x40, 0),
                    1 => {
                        w(c, OBJ + 0x40, screen);
                        w(c, screen + 0x3c, 0);
                    }
                    _ => {
                        w(c, OBJ + 0x40, screen);
                        let node = DATA + 0x3c00;
                        hash_object(c, screen, node);
                        w(c, node + 0xb0, 0);
                        let _ = r.next_u32();
                    }
                }
            },
        );
    }

    // ------------------------------------------------------- net and buddies

    #[test]
    fn lobby_man_request_news_matches() {
        check(
            ee::s06::f_0025e0c8__generated,
            lobby_man_request_news,
            6,
            &|c, seed| {
                let mut r = base(c, seed, 0x0025_e0c8, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = r.next_u32() as u64;
                w(c, OBJ, if seed % 2 == 0 { 0 } else { 1 });
            },
        );
    }

    #[test]
    fn lobby_man_reset_account_info_matches() {
        check(
            ee::s06::f_00260f10__generated,
            lobby_man_reset_account_info,
            4,
            &|c, seed| {
                let _ = base(c, seed, 0x0026_0f10, 0);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    fn roster(c: &mut Cpu, count: u32, seed: u32) {
        let list = DATA + 0x3800;
        let array = DATA + 0x3900;
        let cells = DATA + 0x3a00;
        let entries = DATA + 0x4000;
        w(c, OBJ + 0xcc, list);
        w(c, list + 0x18, count);
        w(c, list + 0x40, array);
        for i in 0..count {
            w(c, array + 4 * i, cells + 4 * i);
            w(c, cells + 4 * i, entries + 0x400 * i);
            let temp = if (i + seed) % 2 == 0 { 0x0010_0000 } else { 0 };
            w(c, entries + 0x400 * i + 0xa8, temp);
        }
    }

    #[test]
    fn buddy_man_count_permanent_entries_matches() {
        check(
            ee::s07::f_00262280__generated,
            buddy_man_count_permanent_entries,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x0026_2280, 4);
                c.g[A0][0] = OBJ as u64;
                roster(c, seed % 5, seed);
            },
        );
    }

    #[test]
    fn buddy_man_count_temp_entries_matches() {
        check(
            ee::s07::f_002622f0__generated,
            buddy_man_count_temp_entries,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x0026_22f0, 4);
                c.g[A0][0] = OBJ as u64;
                roster(c, seed % 5, seed);
            },
        );
    }

    #[test]
    fn c_shared_login_man_init_matches() {
        check(
            ee::s07::f_00265950__generated,
            c_shared_login_man_init,
            4,
            &|c, seed| {
                let _ = base(c, seed, 0x0026_5950, 4);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn net_event102_down_clone_matches() {
        check(
            ee::s07::f_002683f0__generated,
            net_event102_down_clone,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x0026_83f0, 2);
                c.g[A0][0] = OBJ as u64;
                w(c, OBJ + 0xc, DATA + 0x4000);
                c.bus.write8(DATA + 0x4000, b'z');
                c.bus.write8(DATA + 0x4001, 0);
            },
        );
    }

    #[test]
    fn net_event_fc_clone_matches() {
        check(ee::s07::f_00268720__generated, net_event_fc_clone, 6, &|c, seed| {
            let mut r = base(c, seed, 0x0026_8720, 2);
            c.g[A0][0] = OBJ as u64;
            for off in [8u32, 0xc, 0x10] {
                w(c, OBJ + off, r.next_u32());
            }
        });
    }

    // ---------------------------------------------------------------- audio

    #[test]
    fn crowd_audio_instance_remove_all_sources_matches() {
        check(
            ee::s08::f_002a6430__generated,
            crowd_audio_instance_remove_all_sources,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x002a_6430, 2);
                c.g[A0][0] = OBJ as u64;
                c.f[12] = if seed % 2 == 0 { 0x3f80_0000 } else { 0 };
                let filled = seed % 4;
                for i in 0..filled {
                    w(c, OBJ + 0x20 + i * 0xc, 1);
                }
                w(c, OBJ + 0xd4, 0);
                w(c, OBJ + 0xd8, 0);
            },
        );
    }

    #[test]
    fn sound_manager_ref_dtor_matches() {
        check(
            ee::s08::f_002adee8__generated,
            sound_manager_ref_dtor,
            6,
            &|c, seed| {
                let mut r = base(c, seed, 0x002a_dee8, 2);
                c.g[A0][0] = OBJ as u64;
                // Keep the reference's manager null and do not free the object
                // (a non-pool pointer would walk the test heap).
                c.g[A1][0] = 0;
                w(c, OBJ, 0);
                let vt = vtable(c, &mut r, &[0x18]);
                w(c, OBJ2, vt);
                let _ = seed;
            },
        );
    }

    #[test]
    fn speech_instance_add_bank_matches() {
        check(
            ee::s08::f_002b0348__generated,
            speech_instance_add_bank,
            4,
            &|c, seed| {
                let mut r = base(c, seed, 0x002b_0348, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (DATA + 0x4000) as u64;
                c.g[A2][0] = (r.next_u32() % 8) as u64;
                c.g[A3][0] = 0;
                c.bus.write8(DATA + 0x4000, b'q');
                c.bus.write8(DATA + 0x4001, 0);
                w(c, OBJ + 0xf0, seed % 3);
            },
        );
    }

    #[test]
    fn song_instance_fade_out_matches() {
        check(
            ee::s08::f_002b2418__generated,
            song_instance_fade_out,
            6,
            &|c, seed| {
                let mut r = base(c, seed, 0x002b_2418, 3);
                c.g[A0][0] = OBJ as u64;
                c.f[12] = if seed % 2 == 0 {
                    0x3f80_0000
                } else {
                    r.edge_f32()
                };
            },
        );
    }

    #[test]
    fn ssx_audio_system_load_memory_bank_matches() {
        check(
            ee::s07::f_00286ca8__generated,
            ssx_audio_system_load_memory_bank,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x0028_6ca8, 2);
                let id = 0x100 + (seed % 4);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = id as u64;
                c.g[A2][0] = if seed % 2 == 0 { 0 } else { 1 };
                // manager -> bank table: keep the bank header's free link at
                // -1 so the unload path does not walk the test heap.
                let manager = DATA + 0x3200;
                w(c, OBJ + 0x118, DATA + 0x3000);
                w(c, DATA + 0x3000, manager);
                w(c, DATA + 0x3ccc, DATA + 0x4000);
                let handle = (id >> 8) + 8;
                w(c, DATA + 0x4000 + 0x60 * handle + 4, 0xffff_ffff);
            },
        );
    }

    #[test]
    fn ssx_audio_system_restart_peak_music_matches() {
        check(
            ee::s07::f_0028ef90__generated,
            ssx_audio_system_restart_peak_music,
            4,
            &|c, seed| {
                let _ = base(c, seed, 0x0028_ef90, 4);
                c.g[A0][0] = OBJ as u64;
                w(c, OBJ + 0x118, DATA + 0x3000);
                w(c, DATA + 0x3000, DATA + 0x3200);
                w(c, DATA + 0x3200 + 0x1d8, 0);
                // `RestartPeakMusic` reaches the music player's round-robin
                // slot picker (`MusicPlayer_SelectNextSong`, 0x2b42b8), which
                // spins until the occupancy mask names a slot. The bank
                // manager is inline at `self + 0x118`: slot count at +0x3e4,
                // current slot at +0x3f0, 64-bit mask at +0x3f8. A two-slot
                // bank with slot 1 occupied and current 0 exits after one
                // step (same fixture as oc90's SetSelectedPeak test).
                w(c, OBJ + 0x4fc, 2); // slot count
                w(c, OBJ + 0x508, 0); // current
                c.bus.write64(OBJ + 0x510, 2); // occupancy mask (slot 1)
            },
        );
    }

    // -------------------------------------------------- parser, replay, codec

    #[test]
    fn ini_parser_skip_section_headers_matches() {
        check(
            ee::s08::f_002a7a20__generated,
            ini_parser_skip_section_headers,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x002a_7a20, 4);
                c.g[A0][0] = OBJ as u64;
                w(c, OBJ + 8, DATA + 0x4000);
                let text: &[u8] = if seed % 2 == 0 {
                    b"[a]\n[b]\n\0"
                } else {
                    b"key=1\n\0"
                };
                c.bus.ram[DATA as usize + 0x4000..DATA as usize + 0x4000 + text.len()]
                    .copy_from_slice(text);
            },
        );
    }

    #[test]
    fn script_engine_cancel_bank_load_matches() {
        check(
            ee::s07::f_0027b4b8__generated,
            script_engine_cancel_bank_load,
            6,
            &|c, seed| {
                let mut r = base(c, seed, 0x0027_b4b8, 4);
                c.g[A0][0] = OBJ as u64;
                let idx = (r.next_u32() % 4) as u64;
                c.g[A1][0] = idx;
                let slot = OBJ + idx as u32 * 16 + 0x628;
                w(c, slot + 8, if seed % 2 == 0 { 0 } else { DATA + 0x4000 });
                w(c, OBJ + 0x620, seed % 3);
                w(c, DATA + 0x4000, 0);
            },
        );
    }

    #[test]
    fn replay_begin_save_matches() {
        check(ee::s07::f_00270e68__generated, replay_begin_save, 4, &|c, seed| {
            let _ = base(c, seed, 0x0027_0e68, 2);
            c.g[A0][0] = OBJ as u64;
        });
    }

    #[test]
    fn rcmp_decoder_ctor_matches() {
        check(ee::s13::f_003b0430__generated, rcmp_decoder_ctor, 6, &|c, seed| {
            let mut r = base(c, seed, 0x003b_0430, 2);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = (DATA + 0x4000) as u64;
            for i in 0..4 {
                w(c, DATA + 0x4000 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn mpeg2_flush_buffer_matches() {
        check(ee::s13::f_003b1290__generated, mpeg2_flush_buffer, 120, &|c, seed| {
            let mut r = Rng(0x3b12_90e7 ^ seed as u64);
            crate::difftest::scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            w(c, 0x0050_a088, DATA + 0x1000);
            w(c, DATA + 0x1000, DATA + 0x1100);
            w(c, DATA + 0x1004, r.next_u32());
            w(c, DATA + 0x1008, 0x20 + (r.next_u32() % 0x40));
            for i in 0..8 {
                c.bus.write8(DATA + 0x1100 + i, r.next_u32() as u8);
            }
            c.g[A0][0] = (r.next_u32() % 0x30) as u64;
        });
    }

    // ------------------------------------------------------------- menus

    fn menu_input_test(gen: GuestFn, hand: GuestFn, salt: u64, callback: bool) {
        check(gen, hand, 8, &|c, seed| {
            let mut r = base(c, seed, salt, 3);
            c.g[A0][0] = OBJ as u64;
            let menu = DATA + 0x2000;
            let page = DATA + 0x2200;
            let item = DATA + 0x2400;
            let disabled = seed % 4 == 1 || seed % 4 == 3;
            w(c, OBJ + 4, if disabled { 8 } else { 0 });
            w(c, OBJ, menu);
            w(c, menu + 0x124, page);
            let code = match seed % 4 {
                0 | 1 => 6u32,
                2 => 1,
                _ => 8,
            };
            w(c, page + 0x48, code);
            w(c, page, 1);
            w(c, page + 4, item);
            w(c, page + 8, item);
            let vt = vtable(c, &mut r, &[0x38]);
            w(c, item + 0x12c, vt);
            if callback {
                w(c, OBJ + 0x18, FAKE);
            }
        });
    }

    #[test]
    fn menu_sys_back_item_handle_input_matches() {
        menu_input_test(
            ee::s09::f_002cce38__generated,
            menu_sys_back_item_handle_input,
            0x002c_ce38,
            false,
        );
    }

    #[test]
    fn menu_sys_callback_item_handle_input_matches() {
        menu_input_test(
            ee::s09::f_002cd1d0__generated,
            menu_sys_callback_item_handle_input,
            0x002c_d1d0,
            true,
        );
    }

    #[test]
    fn menu_sys_text_edit_item_ctor_matches() {
        check(
            ee::s09::f_002ce418__generated,
            menu_sys_text_edit_item_ctor,
            6,
            &|c, seed| {
                let mut r = base(c, seed, 0x002c_e418, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (r.next_u32() & 0xff) as u64;
                c.g[A2][0] = (DATA + 0x4000) as u64;
                c.bus.write8(DATA + 0x4000, b'n');
                c.bus.write8(DATA + 0x4001, 0);
            },
        );
    }

    // -------------------------------------------------- camera and memcard

    #[test]
    fn chase_cam_controller_enter_spoke_mode_matches() {
        check(
            ee::s01::f_001621a8__generated,
            chase_cam_controller_enter_spoke_mode,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x0016_21a8, 2);
                c.g[A0][0] = OBJ as u64;
                let holder = DATA + 0x2000;
                let rider = DATA + 0x2200;
                w(c, OBJ + 0x20, holder);
                w(c, holder + 4, rider);
                w(c, rider + 0x880, if seed % 3 == 0 { 5 } else { 0 });
                w(c, OBJ + 0x30, if seed % 4 == 0 { 1 } else { 0 });
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_dtor_matches() {
        check(ee::s08::f_002c3fe0__generated, c_ps2_mem_card_dtor, 4, &|c, seed| {
            let _ = base(c, seed, 0x002c_3fe0, 2);
            c.g[A0][0] = OBJ as u64;
            // No buffer and no delete: a non-pool pointer would walk the test
            // heap.
            c.g[A1][0] = 0;
            w(c, OBJ + 0x30, 0);
        });
    }

    #[test]
    fn mc_manager_state38_matches() {
        check(ee::s06::f_0023e428__generated, mc_manager_state38, 8, &|c, seed| {
            let _ = base(c, seed, 0x0023_e428, 2);
            c.g[A0][0] = OBJ as u64;
            let holder = DATA + 0x3000;
            let vt = DATA + 0x3100;
            w(c, OBJ + 0x434, holder);
            w(c, holder, vt);
            // Slot 0x58 returns a zero word from `holder + 4` (the movie player
            // is idle); the object's `+0x6c` hook is the same predicate.
            c.bus.write16(vt + 0x58, 0);
            w(c, vt + 0x5c, 0x002c_a3b0);
            w(c, OBJ + 0x6c, 0x002c_a3b0);
            w(c, OBJ + 4, if seed % 2 == 0 { 0 } else { 8 });
            w(c, OBJ + 0xc8, 1);
        });
    }

    // --------------------------------------------------------- rendering

    #[test]
    fn font_draw_text_right_matches() {
        check(ee::s13::f_00392a60__generated, font_draw_text_right, 4, &|c, seed| {
            let mut r = base(c, seed, 0x0039_2a60, 2);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = (DATA + 0x4000) as u64;
            c.bus.write8(DATA + 0x4000, b't');
            c.bus.write8(DATA + 0x4001, 0);
            c.f[12] = r.edge_f32();
            c.f[13] = r.edge_f32();
            w(c, OBJ + 0x38, r.edge_f32());
            w(c, OBJ + 0x3c, r.edge_f32());
        });
    }

    #[test]
    fn renderer_build_sine_table_matches() {
        check(
            ee::s13::f_00392df0__generated,
            renderer_build_sine_table,
            4,
            &|c, seed| {
                let mut r = Rng(0x392d_f0e1 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                w(c, GAME_GP.wrapping_sub(0x2694), 0x3c49_0fdb);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn tex_man_init_table_matches() {
        check(ee::s12::f_003670e0__generated, tex_man_init_table, 4, &|c, seed| {
            let _ = machine(c, seed, 0x0036_70e0, 0);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = (DATA + 0x1000) as u64;
        });
    }

    #[test]
    fn tex_flip_modifier_ctor_2_matches() {
        covered(
            ee::s12::f_0035f1a8__generated,
            tex_flip_modifier_ctor_2,
            6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_f1a8, 2);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                let vt = vtable(c, &mut r, &[0x10, 0x18]);
                w(c, OBJ2, vt);
                w(c, OBJ + 0x18, 0);
                w(c, OBJ + 0x14, 0);
            },
        );
    }

    #[test]
    fn floating_v51_matches() {
        covered(ee::s12::f_00361858__generated, floating_v51, 6, &|c, seed| {
            let mut r = machine(c, seed, 0x0036_1858, 2);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = OBJ as u64;
            let vt = vtable(c, &mut r, &[0xc0, 0x108]);
            w(c, OBJ + 0xc, vt);
            w(c, OBJ + 0x74, r.next_u32());
            c.bus.write16(OBJ + 0x12, r.next_u32() as u16);
        });
    }

    // ----------------------------------------------------- script and riders

    #[test]
    fn w_script_man_deactivate_mission_matches() {
        check(
            ee::s10::f_00309b00__generated,
            w_script_man_deactivate_mission,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x0030_9b00, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = EV as u64;
                w(c, OBJ + 0x2b8, if seed % 2 == 0 { 0 } else { EV });
                w(c, OBJ + 0x2b4, 0);
                w(c, EV + 0x18, 0);
            },
        );
    }

    #[test]
    fn magnet_modifier_v09_matches() {
        check(ee::s11::f_00357660__generated, magnet_modifier_v09, 8, &|c, seed| {
            let mut r = machine(c, seed, 0x0035_7660, 2);
            globals(c);
            c.g[A0][0] = OBJ as u64;
            let id = (seed % 6) as u64;
            c.g[A1][0] = id;
            w(
                c,
                OBJ + 0x44,
                if seed % 3 == 0 {
                    0xffff_ffff
                } else {
                    (seed % 6) as u32
                },
            );
            w(c, OBJ + 0x54, r.next_u32());
            // A rider for every id: world -> +0x84 -> +0xc -> +0x28 + id*4.
            let race = WORLD + 0x2000;
            let rider = DATA + 0x3000;
            w(c, race + 0x28 + 4 * (id as u32), rider);
            let vt = vtable(c, &mut r, &[0x40]);
            w(c, rider + 0x6c0, vt);
        });
    }

    #[test]
    fn ai_load_next_rider_matches() {
        check(ee::s00::f_001296f8__generated, ai_load_next_rider, 8, &|c, seed| {
            let _ = base(c, seed, 0x0012_96f8, 3);
            c.g[A0][0] = OBJ as u64;
            let count = seed % 4;
            w(c, OBJ + 0x78, count);
            for i in 0..count {
                let rider = DATA + 0x3000 + 0x900 * i;
                w(c, OBJ + 0x28 + 4 * i, rider);
                let loaded = if seed % 2 == 0 { 7 } else { 0 };
                w(c, rider + 0x880, loaded);
            }
        });
    }

    // ------------------------------------------------------------ lobby menu

    #[test]
    fn lobby_menu_enter_room_matches() {
        lobby_menu_test(
            ee::s02::f_0017e2f8__generated,
            lobby_menu_enter_room,
            0x0017_e2f8,
            true,
        );
    }

    #[test]
    fn lobby_menu_cancel_challenge_matches() {
        lobby_menu_test(
            ee::s02::f_0017e558__generated,
            lobby_menu_cancel_challenge,
            0x0017_e558,
            false,
        );
    }

    fn lobby_menu_test(gen: GuestFn, hand: GuestFn, salt: u64, enter: bool) {
        check(gen, hand, 8, &|c, seed| {
            let mut r = base(c, seed, salt, 4);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = (r.next_u32() % 8) as u64;
            let count = seed % 3;
            w(c, OBJ + 8, count);
            let vt = vtable(c, &mut r, &[0x38]);
            for i in 0..count {
                let item = DATA + 0x3000 + 0x100 * i;
                w(c, OBJ + 0xc + 4 * i, item);
                w(c, item + 0x12c, vt);
            }
            if !enter {
                let vt2 = vtable(c, &mut r, &[0x30]);
                w(c, OBJ + 0x1044 + 0x12c, vt2);
            }
        });
    }

    // -------------------------------------------------------- video and misc

    #[test]
    fn c_fe_state_video_player_update_matches() {
        check(
            ee::s04::f_001d24a8__generated,
            c_fe_state_video_player_update,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x001d_24a8, 3);
                c.g[A0][0] = OBJ as u64;
                let player = DATA + 0x3000;
                w(c, OBJ + 0x248, if seed % 2 == 0 { 0 } else { player });
                w(c, OBJ + 0x27c, (seed % 3) as u32);
                w(c, OBJ + 0x278, (seed / 3) as u32);
                w(c, player + 0x40, 0);
                w(c, player + 0x38, 0);
            },
        );
    }
}
