//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold-path front-end, net, audio, codec, replay, menu and
//! renderer helpers, each with a differential test against the generated
//! body.
//!
//! Bodies are transcribed from the generated Rust (the authority for
//! registers, cycles and call order). Calls go through [`crate::call`] with
//! the original return address, so a replaced callee and the fake hooks the
//! differential tests install are both reached. Structured loops make the
//! original's back-edge event checks with [`crate::events::event_due`] and
//! [`crate::events::deliver_event_here`].

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, sp, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
        0x001b_1760,
        cfe_online_character_select_show_opponent_char,
        "cFEStateOnlineCharacterSelect_ShowOpponentChar",
    ),
    (
        0x001b_b798,
        cfe_create_step2_fill_year_list,
        "cFEStateCreateStep2_FillYearList",
    ),
    (
        0x001b_fbf8,
        cfe_online_main_menu_init,
        "cFEStateOnlineMainMenu_Init",
    ),
    (
        0x001c_13e0,
        cfe_net_quick_play_popup_set_options_hint_visible,
        "cFEStateNetQuickPlayPopup_SetOptionsHintVisible",
    ),
    (
        0x001c_2ba8,
        cfe_net_quick_messages_on_reset_popup_event,
        "cFEStateNetQuickMessages_OnResetPopupEvent",
    ),
    (
        0x001e_2bb8,
        message_center_count_group_messages,
        "MessageCenter_CountGroupMessages",
    ),
    (
        0x0025_f388,
        lobby_man_get_rank_list_entry,
        "LobbyMan_GetRankListEntry",
    ),
    (0x0026_49b0, c_setup_man_pop_packet, "cSetupMan_PopPacket"),
    (0x002a_1778, ssx_audio_system_rider_say_20bd, "SSXAudioSystem_RiderSay20BD"),
    (0x002a_2b88, ssx_audio_system_dj_say_212c, "SSXAudioSystem_DJSay212C"),
    (0x002a_32b0, ssx_audio_system_pa_say_20c1, "SSXAudioSystem_PASay20C1"),
    (0x002a_3358, ssx_audio_system_pa_say_20c2, "SSXAudioSystem_PASay20C2"),(0x0027_b410, script_engine_unload_bank, "ScriptEngine_UnloadBank"),
    (0x0013_2620, rider_railslide_sub_set_rail, "RiderRailslideSub_SetRail"),(
        0x0038_6dd0,
        ps2_graphics_man_draw_shadow_edge_pass,
        "PS2GraphicsMan_DrawShadowEdgePass",
    ),(0x0035_7dd8, multi_particle_ctor, "MultiParticle_ctor"),
    (0x0024_fe00, snow_surface_menu_dtor, "SnowSurfaceMenu_dtor"),
    (
        0x0028_6c00,
        ssx_audio_system_unload_world_banks,
        "SSXAudioSystem_UnloadWorldBanks",
    ),
    (
        0x0028_7108,
        ssx_audio_system_on_reset_race,
        "SSXAudioSystem_OnResetRace",
    ),
    (
        0x0028_fc58,
        ssx_audio_system_play_char_select_music,
        "SSXAudioSystem_PlayCharSelectMusic",
    ),(0x0011_9c98, rider_metrix_start_spin_trick, "RiderMetrix_StartSpinTrick"),
    (0x002e_3930, c_player_pulse_fx_reset, "cPlayerPulseFX_Reset"),
    (0x002e_4228, c_transition_man_ctor, "cTransitionMan_ctor"),
    (0x0034_1770, boost_v48, "Boost_v48"),
    (0x0034_29e0, boost_modifier_v08, "BoostModifier_v08"),
    (0x0014_c320, bolts_index_table2_by_char, "Bolts_IndexTable2ByChar"),
    (
        0x002b_5bc0,
        world_trigger_manager_find_instance,
        "WorldTriggerManager_FindInstance",
    ),
    (0x003a_76c0, world_block_pool_dtor, "WorldBlockPool_dtor"),
    (
        0x001d_4f90,
        cfe_rewards_room_set_count_text,
        "cFEStateRewardsRoom_SetCountText",
    ),
    (0x0026_f180, replay_init, "Replay_Init"),
    (0x003a_a520, cps2_chunk_resolve_v07, "cPS2ChunkResolve_v07"),
    (0x003a_a830, cps2_chunk_resolve_v11, "cPS2ChunkResolve_v11"),
    (0x003a_32f0, ui_vector_dtor, "UIVector_dtor"),(0x0020_4ff0, event_select_get_medal_text, "EventSelect_GetMedalText"),(0x0039_bd68, ui_pair_v03, "UIPair_v03"),
    (0x001a_0720, feui_custom_ctor, "FEUICustom_ctor"),
    (0x0018_6bf0, cfe_mem_card_update, "cFEStateMemCard_Update"),
    (
        0x0018_7270,
        cfe_mem_card_on_scan_done,
        "cFEStateMemCard_OnScanDone",
    ),
    (
        0x0026_b7d8,
        path_data_read_float_int_pairs,
        "PathData_ReadFloatIntPairs",
    ),(
        0x0018_4780,
        cfe_uber_trick_show_buy_buttons,
        "cFEStateUberTrick_ShowBuyButtons",
    ),];

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
// Front end.

/// `void cFEStateChallengeReceivedPopup_OnWidgetAction(self, a1, int msg)`
/// (0x1b0538): route a widget message. Kind 5 shows the waiting text (popup
/// state 0xb) or accepts (state 0xc), kind 7 is ignored, anything else is
/// handed to the base net state handler.
pub fn cfe_challenge_received_popup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let msg = c.g[A2][0];
        if c.g[A1][0] == 0 {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001a_8918, 0x001b_05cc)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 5;
            let kind5 = msg == c.g[V0][0];
            c.g[V0][0] = 7;
            if kind5 {
                let this = r32(c, S0);
                c.cycles += 4;
                c.g[A1][0] = Addr(p(this, 0x18)).lw(c);
                c.g[V0][0] = 0xb;
                let state_b = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = 0xc;
                if state_b {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(this, 0x6d4)).lw(c);
                    if c.g[V0][0] == 0 {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x001b_0c08, 0x001b_059c)?;
                        c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
                        c.g[A2][0] = Addr(p(this, 0x14)).lb(c);
                        c.g[A1][0] = Addr(p(r32(c, A0), 0x98)).lw(c);
                        call_ptr(c, 0x0025_fb58, 0x001b_05ac)?;
                    }
                } else {
                    c.cycles += 2;
                    if c.g[A1][0] == c.g[V0][0] {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x001b_0e10, 0x001b_05bc)?;
                    }
                }
            } else {
                c.cycles += 2;
                if msg != c.g[V0][0] {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x001a_8918, 0x001b_05cc)?;
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateOnlineCharacterSelect_ShowOpponentChar(self, int player)`
/// (0x1b1760): look the opponent's name text up on the screen and set it from
/// the character's display name, then flag the text visible.
pub fn cfe_online_character_select_show_opponent_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(r32(c, S2), 0x40)).lw(c);
        let have_screen = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        if have_screen {
            c.cycles += 26;
            let this = r32(c, S2);
            call_ptr(c, 0x0014_dc80, 0x001b_178c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 2;
            call_ptr(c, 0x0014_dd58, 0x001b_1798)?;
            c.g[A2][0] = 0;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_eec8, 0x001b_17a8)?;
            c.g[A0][0] = sx(0x0046_3830);
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, 0x0031_7670, 0x001b_17b8)?;
            c.g[A0][0] = Addr(p(this, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001b_17c4)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
            let setter = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, setter, 0x001b_17e0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x003a_0c50, 0x001b_17ec)?;
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

/// `void cFEStateCreateStep2_FillYearList(box, value)` (0x1bb798): format the
/// value into the list box's year label and select its row.
pub fn cfe_create_step2_fill_year_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_a670, 0x001bb7b8)?;
        let this = r32(c, S0);
        c.g[V0][0] = Addr(p(this, 0x14)).lw(c);
        c.g[V1][0] = sx(0xffff_fffe);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x67;
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | 0x80;
        Addr(p(this, 0x14)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0039_a8d8, 0x001bb7d8)?;
        c.g[A1][0] = sx(0x004a_12c0);
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, 0x0041_7828, 0x001bb7ec)?;
        c.g[A3][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[SP][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0039_a7f0, 0x001bb800)?;
        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(0x6c));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0] & 0xff;
        call_ptr(c, 0x0039_a7a8, 0x001bb810)?;
        c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
        let setter = r32(c, V1);
        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
        call_ptr(c, setter, 0x001bb828)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateOnlineMainMenu_Init(self)` (0x1bfbf8): create the state's
/// screen, play its first frame, and update the net/menu systems.
pub fn cfe_online_main_menu_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(0x0046_53f0);
        c.g[S0][0] = Addr(p(r32(c, S1), 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001bfc1c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001bfc30)?;
        let no_screen = c.g[V0][0] == 0;
        Addr(p(r32(c, S1), 0x40)).sw(c, c.g[V0][0]);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001bfc48)?;
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(gp(c, -0xc8)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        let connected = c.g[V1][0] != 0;
        c.g[V0][0] = Addr(gp(c, -0x204)).lw(c);
        if !connected {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x001a_9930, 0x001bfc60)?;
            c.g[V0][0] = Addr(gp(c, -0x204)).lw(c);
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, 0x0025_6c50, 0x001bfc74)?;
        }
        c.cycles += 10;
        call_ptr(c, 0x0028_b180, 0x001bfc7c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0028_f140, 0x001bfc88)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateNetQuickPlayPopup_SetOptionsHintVisible(self, int show)`
/// (0x1c13e0): show/hide the popup's two text objects.
pub fn cfe_net_quick_play_popup_set_options_hint_visible(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
        let have_screen = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if have_screen {
            c.cycles += 9;
            c.g[A0][0] = sx(0x0046_3ca0);
            call_ptr(c, 0x0031_7670, 0x001c140c)?;
            c.g[A0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001c1418)?;
            c.g[A2][0] = c.g[V0][0];
            let missing = c.g[A2][0] == 0;
            c.g[A1][0] = c.g[S1][0];
            if !missing {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
                c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
                let setter = r32(c, V1);
                c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                call_ptr(c, setter, 0x001c1438)?;
            }
            c.cycles += 9;
            c.g[A0][0] = sx(0x0046_58f0);
            call_ptr(c, 0x0031_7670, 0x001c1444)?;
            c.g[A0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001c1450)?;
            c.g[A2][0] = c.g[V0][0];
            let missing = c.g[A2][0] == 0;
            c.g[A1][0] = c.g[S1][0];
            if !missing {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
                c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
                let setter = r32(c, V1);
                c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                call_ptr(c, setter, 0x001c1470)?;
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

/// `void cFEStateNetQuickMessages_OnResetPopupEvent(self, a1, int kind)`
/// (0x1c2ba8): on kind 0xf clear the pending message, refresh the list and
/// resend the event as 0xf; other kinds go to the base handler.
pub fn cfe_net_quick_messages_on_reset_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0xf;
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        let kind = c.g[A2][0];
        let other = kind != c.g[V0][0];
        frame.save_ra(c, 0);
        if other {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, 0x001a_97b8, 0x001c2c34)?;
        } else {
            c.cycles += 23;
            call_ptr(c, 0x0014_dc80, 0x001c2bd4)?;
            c.g[A1][0] = 4;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_dd58, 0x001c2be0)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0014_f758, 0x001c2bec)?;
            c.g[V0][0] = 1;
            Addr(p(r32(c, S1), 0x6d0)).sw(c, c.g[V0][0]);
            c.g[V1][0] = Addr(p(r32(c, S0), 0xc)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 8)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0xc)).lw(c);
            let refresh = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, refresh, 0x001c2c08)?;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x001c_2418, 0x001c2c10)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = 0xf;
            call_ptr(c, 0x001a_97b8, 0x001c2c20)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MessageCenter_CountGroupMessages(int type, int all)` (0x1e2bb8):
/// total the counts of the message-table entries of `type`, treating
/// entries with no count/flag as one.
pub fn message_center_count_group_messages(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = sx(0x0044_1630);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A0][0];
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            if c.g[V0][0] != c.g[S4][0] {
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            } else {
                c.cycles += 2;
                let mut count = false;
                if c.g[S3][0] != 0 {
                    c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
                    count = true;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(r32(c, S0), 0x10)).lw(c);
                    if c.g[V0][0] == 0 {
                        c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x001e_3a30, 0x001e_2c18)?;
                        if c.g[V0][0] == 0 {
                            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
                            count = true;
                        }
                    }
                }
                if count {
                    c.cycles += 1;
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
                }
                c.cycles += 1;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 61) as u64;
            c.g[S0][0] = sx(p(r32(c, S0), 0x14));
            if c.g[V0][0] != 0 {
                if event_due(c) {
                    deliver_event_here(c, 0x001e_2bf0);
                }
                continue;
            }
            c.cycles += 9;
            c.g[V0][0] = c.g[S2][0];
            frame.restore(c, S0, 0x50);
            frame.restore(c, S1, 0x40);
            frame.restore(c, S2, 0x30);
            frame.restore(c, S3, 0x20);
            frame.restore(c, S4, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            break;
        }
        Ok(())
    })
}

/// `void* LobbyMan_GetRankListEntry(self, outIndex, outSlot, outField)`
/// (0x25f388): resolve the current rank-list handle to its tag structure and
/// hand back its two fields.
pub fn lobby_man_get_rank_list_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        let active = c.g[V0][0] != 0;
        c.g[S3][0] = c.g[A3][0];
        let mut entry = 0u64;
        if !active {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 8;
            c.g[V0][0] = Addr(p(r32(c, S1), 0xf8)).lw(c);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(p(r32(c, S1), r32(c, V0)));
            c.g[A0][0] = Addr(p(r32(c, V0), 0x1ec)).lw(c);
            call_ptr(c, 0x003e_e560, 0x0025f3d0)?;
            c.g[S0][0] = c.g[V0][0];
            let missing = c.g[S0][0] == 0;
            c.g[A3][0] = sx(0x004a_0000);
            entry = c.g[S0][0];
            if !missing {
                c.cycles += 10;
                c.g[A0][0] = sx(p(r32(c, S0), 0x28));
                c.g[A3][0] = sx(0x004a_2f30);
                c.g[A1][0] = c.g[SP][0];
                c.g[A2][0] = 8;
                call_ptr(c, 0x003e_d678, 0x0025f3f0)?;
                c.g[V1][0] = Addr(p(r32(c, SP), 4)).lw(c);
                Addr(r32(c, S3)).sw(c, c.g[V1][0]);
                c.g[V0][0] = Addr(p(r32(c, S1), 0xf8)).lw(c);
                let slot = c.g[V0][0] != 0;
                c.g[V0][0] = Addr(r32(c, SP)).lw(c);
                if !slot {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                }
                c.cycles += 1;
                Addr(r32(c, S2)).sw(c, c.g[V0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = entry;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int cSetupMan_PopPacket(self, void* dst)` (0x2649b0): receive pending
/// packets, dequeue the head packet into `dst`, free it and return its size.
pub fn c_setup_man_pop_packet(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0026_4a58, 0x002649d0)?;
        c.g[A1][0] = Addr(p(r32(c, S2), 0xc)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, S2), 8)).lw(c);
        let empty = c.g[A1][0] == c.g[V0][0];
        c.g[V1][0] = sx(0xffff_ffff);
        let mut len = 0u64;
        if !empty {
            c.cycles += 21;
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(0x1f));
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(1));
            if c.g[V1][0] != 0 {
                c.g[V0][0] = c.g[A1][0];
            }
            Addr(p(r32(c, S2), 0xc)).sw(c, c.g[A0][0]);
            c.g[V0][0] = sx(((r32(c, V0) as i32) >> 5) as u32);
            c.g[A0][0] = c.g[S0][0];
            c.g[V0][0] = sx(r32(c, V0) << 5);
            c.g[V0][0] = sx(r32(c, A1).wrapping_sub(r32(c, V0)));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(p(r32(c, S2), r32(c, V0)));
            c.g[S1][0] = Addr(p(r32(c, V0), 0x10)).lw(c);
            c.g[S0][0] = Addr(p(r32(c, S1), 2)).lhu(c);
            c.g[A1][0] = sx(p(r32(c, S1), 4));
            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(4));
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x003e_6574, 0x00264a2c)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0026_4990, 0x00264a38)?;
            c.g[V0][0] = c.g[S0][0];
            len = c.g[S0][0];
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        let _ = len;
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio.

/// `void SSXAudioSystem_RiderSay20BD(self, rider)` (0x2a1778): when speech is
/// on and the rider's speech system is ready, post the rider's "20bd" event
/// with the character's voice mask.
pub fn ssx_audio_system_rider_say_20bd(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0029_f0a0, 0x002a1794)?;
        let speech_off = c.g[V0][0] == 0;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x5560));
        if !speech_off {
            c.cycles += 5;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002b_0e28, 0x002a17a8)?;
            let not_ready = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !not_ready {
                c.cycles += 10;
                c.mtc1(0, 12);
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20bd;
                c.g[A3][0] = 0;
                c.g[T0][0] = c.g[S1][0];
                c.g[T1][0] = 0;
                c.g[T2][0] = 1;
                call_ptr(c, 0x002b_1458, 0x002a17d0)?;
                let rejected = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S1][0];
                if !rejected {
                    c.cycles += 12;
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0029_f2b0, 0x002a17e0)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = 1;
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0x20bd;
                    call_ptr(c, 0x003d_8008, 0x002a17f4)?;
                    c.g[V1][0] = Addr(gp(c, 0x173c)).lw(c);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.g[S0][0];
                    let post = r32(c, V1);
                    c.g[A1][0] = 1;
                    call_ptr(c, post, 0x002a1808)?;
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

/// `void SSXAudioSystem_DJSay212C(self)` (0x2a2b88): post the DJ's "212c"
/// event with the language mask and the current line, then mark it played.
pub fn ssx_audio_system_dj_say_212c(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0029_f0d8, 0x002a2ba4)?;
        let speech_off = c.g[V0][0] == 0;
        c.g[A0][0] = sx(r32(c, S2).wrapping_add(0x5560));
        let mut last = 0u64;
        if !speech_off {
            c.cycles += 10;
            c.mtc1(0, 12);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x212c;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0xa;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            call_ptr(c, 0x002b_1458, 0x002a2bcc)?;
            let rejected = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
            if !rejected {
                c.cycles += 15;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0029_f198, 0x002a2bdc)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[S1][0] = Addr(p(r32(c, S2), 0x5780)).lw(c);
                c.g[A0][0] = 1;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x212c;
                call_ptr(c, 0x003d_8008, 0x002a2bf4)?;
                c.g[V1][0] = Addr(gp(c, 0x173c)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[A3][0] = c.g[S1][0];
                let post = r32(c, V1);
                c.g[A1][0] = 2;
                call_ptr(c, post, 0x002a2c0c)?;
                c.g[V0][0] = 1;
            }
            last = c.g[V0][0];
            c.cycles += 1;
            Addr(p(r32(c, S2), 0x5780)).sw(c, last);
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

/// The shared body of the two player-announcer lines: check the PA speech
/// switch, post the event with the language and rider-voice masks, and play
/// it on the announcer.
fn pa_say(c: &mut Cpu, code: u32, entry: u32) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    call_ptr(c, 0x0029_f128, entry + 0x18)?;
    let speech_off = c.g[V0][0] == 0;
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x5560));
    let mut skip_one = true;
    if !speech_off {
        c.cycles += 10;
        c.mtc1(0, 12);
        c.g[A1][0] = 0;
        c.g[A2][0] = sx(code);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0xb;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        call_ptr(c, 0x002b_1458, entry + 0x40)?;
        if c.g[V0][0] == 0 {
            skip_one = false;
        } else {
            c.cycles += 18;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0029_f198, entry + 0x50)?;
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, 0x0028_b1d8, entry + 0x58)?;
            c.g[A1][0] = Addr(p(r32(c, V0), 0x28)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_1d48, entry + 0x64)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 1;
            c.g[A1][0] = 0;
            c.g[A2][0] = sx(code);
            call_ptr(c, 0x003d_8008, entry + 0x78)?;
            c.g[V1][0] = Addr(gp(c, 0x173c)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[S0][0];
            let post = r32(c, V1);
            c.g[A1][0] = 2;
            call_ptr(c, post, entry + 0x90)?;
        }
    }
    if skip_one {
        c.cycles += 1;
    }
    frame.restore(c, S0, 0x20);
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void SSXAudioSystem_PASay20C1(self)` (0x2a32b0).
pub fn ssx_audio_system_pa_say_20c1(c: &mut Cpu) {
    guest(c, |c| pa_say(c, 0x20c1, 0x002a_32b0))
}

/// `void SSXAudioSystem_PASay20C2(self)` (0x2a3358).
pub fn ssx_audio_system_pa_say_20c2(c: &mut Cpu) {
    guest(c, |c| pa_say(c, 0x20c2, 0x002a_3358))
}

// ---------------------------------------------------------------------------
// Script engine.

/// `void ScriptEngine_v03(self, int idx)` (0x279da8): drop the script view
/// object of `idx` (removing its camera or setting its transition, by state)
/// and then remove the script object.
pub fn script_engine_v03(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0027_d320, 0x00279dc4)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[V0][0] = 1;
        let state1 = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 2;
        if state1 {
            c.cycles += 8;
            c.g[A0][0] = Addr(p(r32(c, A1), 0x7c)).lw(c);
            c.g[V0][0] = 0x120;
            c.mult(0, 4, 4, 2);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6d0));
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, 0x0028_11e8, 0x00279df0)?;
            c.g[A0][0] = c.g[S0][0];
        } else {
            c.cycles += 2;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A0][0] = c.g[S0][0];
            } else {
                c.cycles += 12;
                c.g[V1][0] = Addr(p(r32(c, A1), 0xc)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V1), 0x48)).lh(c);
                c.g[V0][0] = Addr(p(r32(c, V1), 0x4c)).lw(c);
                let trans = r32(c, V0);
                c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
                call_ptr(c, trans, 0x00279e14)?;
                c.g[V1][0] = 0x120;
                c.g[A1][0] = 0;
                c.mult(0, 2, 2, 3);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x6d0));
                c.g[A0][0] = sx(p(r32(c, S0), r32(c, V0)));
                call_ptr(c, 0x0028_12e0, 0x00279e2c)?;
                c.g[A0][0] = c.g[S0][0];
            }
        }
        c.cycles += 7;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0027_cf88, 0x00279e38)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_UnloadBank(self, int index)` (0x27b410): release the
/// data instance (or the async file) held by bank slot `index` and clear the
/// slot.
pub fn script_engine_unload_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = sx(r32(c, A1) << 4);
        frame.save(c, S0, 0x20);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x628));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(p(r32(c, S0), r32(c, A1)));
        c.g[A1][0] = Addr(r32(c, S1)).lw(c);
        if c.g[A1][0] != 0 {
            c.cycles += 6;
            c.g[A0][0] = sx(p(r32(c, S0), 0x51c));
            call_ptr(c, 0x0027_5a20, 0x0027b444)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V0][0] = Addr(p(r32(c, S1), 0xc)).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                if c.g[A0][0] != 0 {
                    c.cycles += 4;
                    call_ptr(c, 0x0031_7e98, 0x0027b464)?;
                }
            }
        } else {
            c.g[A0][0] = Addr(p(r32(c, S1), 8)).lw(c);
            c.cycles += 2;
            if c.g[A0][0] != 0 {
                c.cycles += 9;
                call_ptr(c, 0x003d_faf0, 0x0027b47c)?;
                c.g[A0][0] = Addr(p(r32(c, S1), 8)).lw(c);
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                call_ptr(c, 0x003d_f9d8, 0x0027b48c)?;
                c.g[V1][0] = Addr(p(r32(c, S0), 0x620)).lw(c);
                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                Addr(p(r32(c, S0), 0x620)).sw(c, c.g[V1][0]);
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0027_7f08, 0x0027b4a0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider.

/// `int RiderRailslideSub_SetRail(self, int id)` (0x132620): remember the
/// rail id, ask the rider's rail state to play its enter animation when the
/// rail flag is set, and return 1 in that case.
pub fn rider_railslide_sub_set_rail(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(0xffff_ffff);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        let none = c.g[S1][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        if none {
            c.cycles += 1;
            Addr(p(r32(c, S0), 4)).sw(c, c.g[S1][0]);
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 7;
            c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
            c.mtc1(0, 1);
            c.lwc1(0, p(r32(c, V1), 0x2f0));
            c.c_lt_s(1, 0);
            if !c.fcc() {
                c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
            } else {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(r32(c, V1), 0xb2c)).lw(c);
                c.g[V0][0] = sx(((r32(c, V0) as i32) >> 1) as u32);
                c.g[V0][0] = c.g[V0][0] & 1;
                let rail_off = c.g[V0][0] == 0;
                c.g[A1][0] = 0xc;
                if !rail_off {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(p(r32(c, V1), 0x77c)).lw(c);
                    Addr(p(r32(c, V0), 0x394)).sw(c, c.g[S1][0]);
                    c.g[A0][0] = Addr(p(r32(c, S0), 8)).lw(c);
                    call_ptr(c, 0x0011_fec8, 0x00132680)?;
                    c.g[V0][0] = 1;
                    c.cycles += 5;
                    frame.restore(c, S0, 0x20);
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 1;
                c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
            }
            c.cycles += 2;
            if c.g[S1][0] == c.g[V0][0] {
                Addr(p(r32(c, S0), 4)).sw(c, c.g[S1][0]);
                c.cycles += 1;
                c.g[V0][0] = 0;
            } else {
                c.cycles += 5;
                call_ptr(c, 0x0028_b180, 0x0013269c)?;
                c.g[A1][0] = Addr(p(r32(c, S0), 8)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0029_9b70, 0x001326a8)?;
                c.cycles += 1;
                Addr(p(r32(c, S0), 4)).sw(c, c.g[S1][0]);
                c.cycles += 1;
                c.g[V0][0] = 0;
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

/// `void RiderRailslideSub_PlayEnterAnim(self, int alt)` (0x1326c8): play
/// the railslide enter animation matching the rider's state and side.
pub fn rider_railslide_sub_play_enter_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 4;
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(r32(c, A0), 8)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x328)).lw(c);
        let not4 = c.g[A0][0] != c.g[V0][0];
        c.g[V0][0] = 3;
        // 1: the `alt` entry (b_132748), 2: the shared 0x12 entry (b_132730).
        let mut tail = 0u32;
        if not4 {
            c.cycles += 2;
            let not3 = c.g[A0][0] != c.g[V0][0];
            if not3 {
                c.cycles += 2;
                let alt0 = c.g[A1][0] == 0;
                c.g[A1][0] = 0x44;
                if alt0 {
                    tail = 1;
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = Addr(p(r32(c, V1), 0x784)).lw(c);
                    tail = 2;
                }
            } else {
                c.cycles += 2;
                let alt0 = c.g[A1][0] == 0;
                c.g[A1][0] = 0x46;
                if alt0 {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(p(r32(c, V1), 0x784)).lw(c);
                    c.g[A1][0] = 0x13;
                    tail = 2;
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = Addr(p(r32(c, V1), 0x784)).lw(c);
                    tail = 2;
                }
            }
        } else {
            c.cycles += 2;
            let alt0 = c.g[A1][0] == 0;
            c.g[A1][0] = 0x45;
            if alt0 {
                c.cycles += 3;
                c.g[A0][0] = Addr(p(r32(c, V1), 0x784)).lw(c);
                c.g[A1][0] = 0x14;
                tail = 2;
            } else {
                c.cycles += 2;
                c.g[A0][0] = Addr(p(r32(c, V1), 0x784)).lw(c);
                tail = 2;
            }
        }
        if tail == 1 {
            c.cycles += 6;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 12);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0031_28e8, 0x00132740)?;
            c.g[RA][0] = Addr(sp(c)).ld(c);
        } else {
            c.cycles += 7;
            c.g[A0][0] = Addr(p(r32(c, V1), 0x784)).lw(c);
            c.g[A1][0] = 0x12;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 12);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0031_28e8, 0x00132760)?;
            c.g[RA][0] = Addr(sp(c)).ld(c);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Renderer.

/// `void PS2GraphicsMan_DrawShadowEdgePass(self, strips, int n, a3, a4, a5)`
/// (0x386dd0): emit every strip block with at least two vertices.
pub fn ps2_graphics_man_draw_shadow_edge_pass(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[T1][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[T0][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = c.g[A3][0];
        c.g[S5][0] = c.g[A0][0];
        let none = (c.g[S1][0] as i64) <= 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 1;
            loop {
                c.cycles += 4;
                c.g[A3][0] = Addr(p(r32(c, S0), 8)).lw(c);
                c.g[V0][0] = ((c.g[A3][0] as i64) < 2) as u64;
                if c.g[V0][0] != 0 {
                    c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                } else {
                    c.cycles += 10;
                    c.g[A1][0] = Addr(r32(c, S0)).lw(c);
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A2][0] = Addr(p(r32(c, S0), 4)).lw(c);
                    c.g[T0][0] = c.g[S4][0];
                    c.lwc1(12, p(r32(c, S0), 0xc));
                    c.g[T1][0] = c.g[S3][0];
                    c.lwc1(13, p(r32(c, S0), 0x10));
                    c.g[T2][0] = c.g[S2][0];
                    call_ptr(c, 0x0038_7ec0, 0x00386e44)?;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                }
                c.cycles += 2;
                let more = (c.g[S1][0] as i64) > 0;
                c.g[S0][0] = sx(p(r32(c, S0), 0x14));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0038_6e10);
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SHP_DXTAlphaIsOpaque(const void* buf, int size, int type)` (0x3b3308):
/// scan 16-byte DXT blocks for a non-opaque alpha pattern.
pub fn shp_dxt_alpha_is_opaque(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 3;
    let other = c.g[A2][0] != c.g[V0][0];
    c.g[V0][0] = sx(0xffff_ffff);
    if other {
        c.cycles += 6;
        c.g[V1][0] = sx(r32(c, A1).wrapping_add(0xf));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[V1][0] = c.g[A1][0];
        }
        c.g[V1][0] = sx(((r32(c, V1) as i32) >> 4) as u32);
        let empty = (c.g[V1][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if empty {
            c.cycles += 2;
            c.g[V0][0] = 1;
            abi::ret(c);
            return;
        }
        c.cycles += 1;
        c.g[A2][0] = 0xffff;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, A0)).lhu(c);
            let mismatch = c.g[V0][0] != c.g[A2][0];
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
            if mismatch {
                c.cycles += 3;
                c.g[V0][0] = 0;
                abi::ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x20));
            if !more {
                c.cycles += 2;
                c.g[V0][0] = 1;
                abi::ret(c);
                return;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_3388);
            }
        }
    }
    c.cycles += 6;
    c.g[V1][0] = sx(r32(c, A1).wrapping_add(0xf));
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
    if c.g[V0][0] != 0 {
        c.g[V1][0] = c.g[A1][0];
    }
    c.g[V1][0] = sx(((r32(c, V1) as i32) >> 4) as u32);
    let empty = (c.g[V1][0] as i64) <= 0;
    c.g[A2][0] = 0;
    if empty {
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[A3][0] = sx(0xffff);
    loop {
        c.cycles += 3;
        c.g[A1][0] = Addr(r32(c, A0)).ld(c);
        let mismatch = c.g[A1][0] != c.g[A3][0];
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(8));
        if mismatch {
            c.cycles += 3;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, A0)).ld(c);
        let mismatch = c.g[V0][0] != c.g[A1][0];
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
        if mismatch {
            c.cycles += 3;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
        if !more {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003b_3338);
        }
    }
}

/// `sMultiPart* MultiParticle_ctor(mem, src, owner)` (0x357dd8): allocate the
/// particle data, copy it in, set up the emitter and copy the descriptor's
/// vectors.
pub fn multi_particle_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x40);
        c.g[A3][0] = 0;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A1][0];
        Addr(r32(c, S0)).sw(c, c.g[A2][0]);
        c.g[A1][0] = sx(0x0048_ea10);
        Addr(p(r32(c, S0), 4)).sw(c, 0);
        c.g[A0][0] = 0xd8;
        c.g[A2][0] = 0x2000_0000;
        call_ptr(c, 0x0031_7d70, 0x00357e10)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 0xd8;
        call_ptr(c, 0x003e_6574, 0x00357e24)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0035_8020, 0x00357e2c)?;
        c.g[A1][0] = Addr(p(r32(c, S0), 0x10)).lw(c);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        c.lq(3, p(r32(c, A1), 0x160));
        frame.restore_ra(c, 0x10);
        c.sq(3, p(r32(c, S0), 0x20));
        c.lq(4, p(r32(c, A1), 0x170));
        c.sq(4, p(r32(c, S0), 0x30));
        c.lwc1(2, p(r32(c, A1), 0xc8));
        c.lwc1(0, p(r32(c, A1), 0xc0));
        c.lwc1(1, p(r32(c, A1), 0xc4));
        c.swc1(0, sp(c));
        c.swc1(1, p(sp(c), 4));
        c.swc1(2, p(sp(c), 8));
        Addr(p(sp(c), 0xc)).sw(c, 0);
        c.lq(3, sp(c));
        c.sq(3, p(r32(c, S0), 0x40));
        frame.restore(c, S0, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void SnowSurfaceMenu_dtor(self, flags)` (0x24fe00): destroy the menu's
/// nine sub-objects and the base object.
pub fn snow_surface_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        for (off, ret) in [
            (0x294u32, 0x0024_fe24u32),
            (0x260, 0x0024_fe30),
            (0x22c, 0x0024_fe3c),
            (0x1f8, 0x0024_fe48),
            (0x1c4, 0x0024_fe54),
            (0x194, 0x0024_fe60),
            (0x164, 0x0024_fe6c),
            (0x14c, 0x0024_fe78),
            (0x130, 0x0024_fe84),
        ] {
            c.g[A0][0] = sx(p(r32(c, S0), off));
            c.g[A1][0] = 2;
            call_ptr(c, 0x002c_a280, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0024fe90)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_UnloadWorldBanks(self)` (0x286c00): finish the
/// current load, unload the world's banks 0..16 (except 13) and free the
/// bank memory.
pub fn ssx_audio_system_unload_world_banks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = 0;
        call_ptr(c, 0x002a_6f38, 0x00286c20)?;
        c.g[S2][0] = 0xd;
        c.g[V0][0] = Addr(p(r32(c, S1), 0x118)).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
        call_ptr(c, 0x0028_9df0, 0x00286c34)?;
        loop {
            c.cycles += 2;
            let is13 = c.g[S0][0] == c.g[S2][0];
            c.g[A1][0] = c.g[S0][0];
            if !is13 {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(r32(c, S1), 0x118)).lw(c);
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                call_ptr(c, 0x0028_bce8, 0x00286c4c)?;
            }
            c.cycles += 4;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 17) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = 3;
            if !more {
                c.cycles += 18;
                c.g[V0][0] = Addr(p(r32(c, S1), 0x118)).lw(c);
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                call_ptr(c, 0x0028_bde0, 0x00286c68)?;
                c.g[V0][0] = Addr(p(r32(c, S1), 0x118)).lw(c);
                c.g[A1][0] = 0xc;
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                call_ptr(c, 0x0028_bde0, 0x00286c78)?;
                c.g[V0][0] = Addr(p(r32(c, S1), 0x118)).lw(c);
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                call_ptr(c, 0x0028_a230, 0x00286c84)?;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0029_f5e0, 0x00286c8c)?;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0028_6c38);
            }
        }
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_OnResetRace(self)` (0x287108): reset the race sounds
/// and, once the race manager agrees, restart the race music.
pub fn ssx_audio_system_on_reset_race(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0028_71b0, 0x0028711c)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002a_4168, 0x00287124)?;
        let mut proceed = c.g[V0][0] != 0;
        if !proceed {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_41d8, 0x00287134)?;
            proceed = c.g[V0][0] != 0;
        }
        if proceed {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_4040, 0x00287144)?;
            if c.g[V0][0] == 0 {
                c.cycles += 7;
                c.g[A1][0] = Addr(gp(c, -0x848)).lw(c);
                c.g[V0][0] = Addr(p(r32(c, S0), 0x5824)).lw(c);
                c.g[V1][0] = Addr(p(r32(c, A1), 0xc0)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V1), 0x70)).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0;
                    call_ptr(c, 0x0029_5028, 0x00287178)?;
                    if c.g[V0][0] == 0 {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0;
                        c.g[A2][0] = 0;
                        c.g[A3][0] = sx(0xffff_ffff);
                        c.g[T0][0] = 0;
                        c.g[T1][0] = 0;
                        call_ptr(c, 0x0029_5628, 0x0028719c)?;
                    }
                }
            }
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_PlayCharSelectMusic(self)` (0x28fc58): start the
/// character-select music on the sound bank's channel 0x12d and set its
/// volume from the current rider.
pub fn ssx_audio_system_play_char_select_music(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x60);
        c.g[A1][0] = 1;
        frame.save(c, S1, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S2, 0x30);
        c.g[S1][0] = sx(0x004a_0000);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S3][0] = sx(r32(c, S2).wrapping_add(0x118));
        frame.save(c, S4, 0x10);
        c.g[S1][0] = sx(0x004a_3598);
        frame.save_ra(c, 0);
        c.g[S4][0] = Addr(p(r32(c, S2), 0x5558)).lw(c);
        c.g[S0][0] = Addr(p(r32(c, S4), 0x10)).lh(c);
        c.g[S0][0] = sx(p(r32(c, S3), r32(c, S0)));
        call_ptr(c, 0x0028_7968, 0x0028fc9c)?;
        c.g[A3][0] = c.g[V0][0];
        c.g[T0][0] = 1;
        c.g[V0][0] = Addr(p(r32(c, S4), 0x14)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        let play = r32(c, V0);
        c.g[A2][0] = 0xc;
        call_ptr(c, play, 0x0028fcb8)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0x12d;
        call_ptr(c, 0x002b_49c0, 0x0028fcc4)?;
        c.g[V1][0] = Addr(p(r32(c, S2), 0x5558)).lw(c);
        c.g[A1][0] = Addr(p(r32(c, S2), 0x6290)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x20)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x24)).lw(c);
        let set_vol = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, S3), r32(c, A0)));
        call_ptr(c, set_vol, 0x0028fcdc)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VidEnginePlayer_Render(self, int force)` (0x253890): drive the
/// player object's present or its frame-advance method.
pub fn vid_engine_player_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 0x38)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A2][0] = Addr(p(r32(c, S0), 0x3c)).lw(c);
            let no_obj = c.g[A2][0] == 0;
            c.g[A3][0] = c.g[A2][0];
            if !no_obj {
                // 1: the object's vtable slot (a3 + delta); 2: the player's
                // own word field; 3: the object's other slot.
                let mut via = 0u32;
                c.cycles += 2;
                if c.g[A1][0] != 0 {
                    c.g[V0][0] = Addr(p(r32(c, A3), 4)).lw(c);
                    via = 1;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = Addr(p(r32(c, A3), 4)).lw(c);
                        via = 1;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
                        if c.g[V0][0] != 0 {
                            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                            via = 2;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
                            if c.g[V0][0] != 0 {
                                c.g[V0][0] = Addr(p(r32(c, A3), 4)).lw(c);
                                via = 1;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                                via = 2;
                            }
                        }
                        if via == 2 {
                            c.cycles += 2;
                            if c.g[V0][0] == 0 {
                                c.g[V0][0] = Addr(p(r32(c, A2), 4)).lw(c);
                                via = 3;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = Addr(p(r32(c, A3), 4)).lw(c);
                                via = 1;
                            }
                        }
                    }
                }
                if via == 1 {
                    c.cycles += 6;
                    c.g[A0][0] = Addr(p(r32(c, V0), 0x28)).lh(c);
                    c.g[V1][0] = Addr(p(r32(c, V0), 0x2c)).lw(c);
                    let present = r32(c, V1);
                    c.g[A0][0] = sx(p(r32(c, A3), r32(c, A0)));
                    call_ptr(c, present, 0x00253904)?;
                    frame.restore(c, S0, 0x10);
                } else {
                    if via != 0 {
                        c.cycles += 6;
                        c.g[A1][0] = Addr(p(r32(c, S0), 0xc)).lw(c);
                        c.g[A0][0] = Addr(p(r32(c, V0), 0x20)).lh(c);
                        c.g[V1][0] = Addr(p(r32(c, V0), 0x24)).lw(c);
                        let advance = r32(c, V1);
                        c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                        call_ptr(c, advance, 0x00253920)?;
                        Addr(p(r32(c, S0), 0x40)).sw(c, 0);
                    }
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider tricks and effects.

/// `float RiderMetrix_StartSpinTrick(self, int trick)` (0x119c98): announce a
/// spin trick, and when the rider is spinning start the trick's timer.
pub fn rider_metrix_start_spin_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[T0][0] = 0;
        call_ptr(c, 0x0011_a168, 0x00119cc8)?;
        c.mtc1(0, 20);
        c.lwc1(0, p(r32(c, S0), 0x14));
        c.c_lt_s(20, 0);
        let spinning = c.fcc();
        c.g[V0][0] = 1;
        if spinning {
            c.cycles += 14;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 0);
            Addr(r32(c, S0)).sw(c, c.g[S1][0]);
            c.g[A0][0] = sx(p(r32(c, S0), 0x60));
            c.swc1(20, p(r32(c, S0), 0x34));
            c.g[A1][0] = 0;
            c.swc1(20, p(r32(c, S0), 0x38));
            c.g[A2][0] = 0xc;
            Addr(p(r32(c, S0), 4)).sw(c, 0);
            Addr(p(r32(c, S0), 0xc)).sw(c, 0);
            Addr(p(r32(c, S0), 0x10)).sw(c, 0);
            c.swc1(0, p(r32(c, S0), 0x40));
            Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x003e_6448, 0x00119d18)?;
        }
        c.cycles += 9;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0011_76f8, 0x00119d20)?;
        c.mov_s(0, 20);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void cPlayerPulseFX_Reset(self)` (0x2e3930): reset the pulse effect's
/// four vectors (wake vector 0.5/0/0/0, tint 0.7 x3, then the two copies).
pub fn c_player_pulse_fx_reset(c: &mut Cpu) {
    c.cycles += 41;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(1, 0);
    c.swc1(1, p(sp(c), 4));
    c.swc1(0, sp(c));
    Addr(p(sp(c), 8)).sw(c, 0);
    Addr(p(sp(c), 0xc)).sw(c, 0);
    c.lwc1(0, gp(c, -0x3a34));
    c.ldl(2, p(sp(c), 7));
    c.ldr(2, sp(c));
    c.ldl(3, p(sp(c), 0xf));
    c.ldr(3, p(sp(c), 8));
    let obj = r32(c, A0);
    c.sdl(2, p(obj, 0xf));
    c.sdr(2, p(obj, 8));
    c.sdl(3, p(obj, 0x17));
    c.sdr(3, p(obj, 0x10));
    c.swc1(1, p(sp(c), 4));
    c.swc1(0, p(sp(c), 0xc));
    c.swc1(0, sp(c));
    c.swc1(0, p(sp(c), 8));
    c.ldl(2, p(sp(c), 7));
    c.ldr(2, sp(c));
    c.ldl(3, p(sp(c), 0xf));
    c.ldr(3, p(sp(c), 8));
    c.sdl(2, p(obj, 0x1f));
    c.sdr(2, p(obj, 0x18));
    c.sdl(3, p(obj, 0x27));
    c.sdr(3, p(obj, 0x20));
    c.ldl(2, p(obj, 0xf));
    c.ldr(2, p(obj, 8));
    c.ldl(3, p(obj, 0x17));
    c.ldr(3, p(obj, 0x10));
    c.sdl(2, p(obj, 0x2f));
    c.sdr(2, p(obj, 0x28));
    c.sdl(3, p(obj, 0x37));
    c.sdr(3, p(obj, 0x30));
    Addr(p(obj, 4)).sw(c, 0);
    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
    abi::ret(c);
}

/// `cTransitionMan* cTransitionMan_ctor(self, a1, a2)` (0x2e4228): build the
/// transition manager (effect pool, effect list, default fields).
pub fn c_transition_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0035_4648, 0x002e4248)?;
        c.g[S2][0] = sx(p(r32(c, S0), 0x10));
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0028_2cd0, 0x002e4254)?;
        c.g[V1][0] = sx(0x0049_0000);
        c.g[V0][0] = sx(0x0049_0000);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x7f00));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x7ea8));
        Addr(p(r32(c, S0), 0x74)).sw(c, c.g[S1][0]);
        c.g[A0][0] = sx(0xffff_ffff);
        Addr(p(r32(c, S0), 0x40)).sw(c, 0);
        Addr(p(r32(c, S0), 0x44)).sw(c, 0);
        Addr(p(r32(c, S0), 0x4c)).sw(c, 0);
        Addr(p(r32(c, S0), 0x50)).sw(c, 0);
        Addr(p(r32(c, S0), 0x60)).sw(c, 0);
        Addr(p(r32(c, S0), 0x64)).sw(c, 0);
        Addr(p(r32(c, S0), 0x68)).sw(c, 0);
        Addr(p(r32(c, S0), 0x70)).sw(c, 0);
        Addr(p(r32(c, S0), 0x78)).sw(c, 0);
        Addr(p(r32(c, S0), 0x7c)).sw(c, 0);
        Addr(p(r32(c, S0), 0x80)).sw(c, 0);
        Addr(p(r32(c, S0), 0x1c)).sw(c, c.g[V1][0]);
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[V0][0]);
        Addr(p(r32(c, S0), 0x6c)).sw(c, c.g[A0][0]);
        call_ptr(c, 0x002e_4cb0, 0x002e42a8)?;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0028_3298, 0x002e42b0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Modifiers.

/// The shared body of the boost modifiers: scale the object's velocity by
/// `1 + gain` and by the frame scale, then hand it back.
fn boost_scale(c: &mut Cpu, gain_off: u32, entry: u32) -> Flow {
    c.cycles += 41;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S0, 0x50);
    frame.save(c, S1, 0x40);
    c.g[S0][0] = c.g[A1][0];
    frame.save_ra(c, 0x30);
    c.g[S1][0] = c.g[A0][0];
    c.g[V1][0] = Addr(r32(c, S0)).lw(c);
    c.g[A0][0] = Addr(p(r32(c, V1), 0x10)).lh(c);
    c.g[V0][0] = Addr(p(r32(c, V1), 0x14)).lw(c);
    let get = r32(c, V0);
    c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
    call_ptr(c, get, entry + 0x2c)?;
    c.lwc1(0, p(r32(c, S1), gain_off));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1);
    c.add_s(0, 0, 1);
    c.mfc1(3, 0);
    c.lqc2(4, r32(c, V0));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, p(sp(c), 0x20));
    c.lq(2, p(sp(c), 0x20));
    c.sq(2, p(sp(c), 0x10));
    call_ptr(c, 0x002d_1c70, entry + 0x5c)?;
    c.mfc1(2, 0);
    c.lqc2(4, p(sp(c), 0x10));
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, p(sp(c), 0x20));
    c.g[A1][0] = c.g[SP][0];
    c.lq(2, p(sp(c), 0x20));
    c.g[V1][0] = Addr(r32(c, S0)).lw(c);
    c.sq(2, sp(c));
    c.g[A0][0] = Addr(p(r32(c, V1), 0x20)).lh(c);
    c.g[V0][0] = Addr(p(r32(c, V1), 0x24)).lw(c);
    let set = r32(c, V0);
    c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
    call_ptr(c, set, entry + 0x90)?;
    frame.restore(c, S0, 0x50);
    frame.restore(c, S1, 0x40);
    frame.restore_ra(c, 0x30);
    frame.close(c);
    Ok(())
}

/// `void Boost_v48(self, obj)` (0x341770): boost the object's velocity using
/// the effect's `+0x34` gain.
pub fn boost_v48(c: &mut Cpu) {
    guest(c, |c| boost_scale(c, 0x34, 0x0034_1770))
}

/// `void BoostModifier_v08(self, obj)` (0x3429e0): same with `+0x24`.
pub fn boost_modifier_v08(c: &mut Cpu) {
    guest(c, |c| boost_scale(c, 0x24, 0x0034_29e0))
}

// ---------------------------------------------------------------------------
// Table and pool helpers.

/// `void Bolts_IndexTable2ByChar(self)` (0x14c320): clear the per-character
/// group counts and record the first entry of each group.
pub fn bolts_index_table2_by_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x30);
        c.g[A2][0] = 0x78;
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S1][0] = sx(p(r32(c, S0), 0x20c));
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003e_6448, 0x0014c34c)?;
        c.g[S2][0] = 0x1e;
        c.g[V0][0] = Addr(p(r32(c, S0), 0x20)).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A3][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            c.g[T1][0] = sx(p(r32(c, S0), 0x284));
            c.g[T0][0] = 0;
            loop {
                c.cycles += 9;
                c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
                c.g[V0][0] = sx(p(r32(c, T0), r32(c, V0)));
                c.g[A1][0] = Addr(r32(c, V0)).lb(c);
                c.g[A2][0] = sx(r32(c, A1) << 2);
                c.g[V1][0] = sx(p(r32(c, A0), r32(c, A2)));
                c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                let same = c.g[S2][0] == c.g[A1][0];
                Addr(r32(c, V1)).sw(c, c.g[V0][0]);
                if !same {
                    c.cycles += 3;
                    c.g[V0][0] = sx(p(r32(c, T1), r32(c, A2)));
                    c.g[S2][0] = c.g[A1][0];
                    Addr(r32(c, V0)).sw(c, c.g[A3][0]);
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(p(r32(c, S0), 0x20)).lw(c);
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[T0][0] = sx(p(r32(c, T0), 0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_c368);
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

/// `void* WorldTriggerManager_FindInstance(self, a, b, c)` (0x2b5bc0): walk
/// the manager's 40 instance slots and return the first one matching.
pub fn world_trigger_manager_find_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S5][0] = c.g[A1][0];
        loop {
            c.cycles += 5;
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, V0)));
            c.g[V1][0] = Addr(r32(c, A0)).lw(c);
            let empty = c.g[V1][0] == 0;
            c.g[A1][0] = c.g[S5][0];
            let mut found = false;
            if !empty {
                c.cycles += 5;
                c.g[A2][0] = c.g[S4][0];
                c.g[A3][0] = c.g[S3][0];
                call_ptr(c, 0x002b_4c08, 0x002b5c18)?;
                let miss = c.g[V0][0] == 0;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                if !miss {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, S1)).lw(c);
                    c.g[V0][0] = sx(p(r32(c, V0), r32(c, S0)));
                    found = true;
                }
            } else {
                c.cycles += 1;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            }
            if found {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 40) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx(p(r32(c, S0), 0x70));
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 0;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002b_5bf8);
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WorldBlockPool_dtor(self, flags)` (0x3a76c0): free every resident
/// block and the block array, then delete the pool itself when asked.
pub fn world_block_pool_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S1), 4)).lw(c);
        let none = c.g[V0][0] == 0;
        c.g[S3][0] = c.g[A1][0];
        if !none {
            c.cycles += 1;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(r32(c, S1), 0xc)).lw(c);
                c.g[V0][0] = sx(p(r32(c, S2), r32(c, V0)));
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                if c.g[A0][0] == 0 {
                    c.g[V0][0] = Addr(p(r32(c, S1), 4)).lw(c);
                } else {
                    c.cycles += 3;
                    call_ptr(c, 0x0031_7e98, 0x003a770c)?;
                    c.g[V0][0] = Addr(p(r32(c, S1), 4)).lw(c);
                }
                c.cycles += 4;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx(p(r32(c, S2), 0x18));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_76f0);
                }
            }
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(p(r32(c, S1), 0xc)).lw(c);
        let no_array = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S3][0] & 1;
        if !no_array {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x003a7734)?;
            c.g[V0][0] = c.g[S3][0] & 1;
        }
        c.cycles += 2;
        let keep = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x40);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x003a7748)?;
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

/// `void cFEStateRewardsRoom_SetCountText(self, index, text)` (0x1d4f90):
/// format the collected/total count into a buffer and set it on the label.
pub fn cfe_rewards_room_set_count_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x160);
        frame.save(c, S1, 0x140);
        frame.save(c, S3, 0x120);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S4, 0x110);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S0, 0x150);
        frame.save(c, S2, 0x130);
        frame.save_ra(c, 0x100);
        c.g[S4][0] = c.g[A2][0];
        call_ptr(c, 0x0014_dc80, 0x001d4fbc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x0014_dd58, 0x001d4fc8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0015_7210, 0x001d4fd8)?;
        c.g[A1][0] = Addr(p(r32(c, S1), 0x44)).lb(c);
        c.g[S2][0] = c.g[V0][0];
        c.g[A2][0] = Addr(p(r32(c, S1), 0x48)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S3][0];
        call_ptr(c, 0x0015_7518, 0x001d4ff0)?;
        c.g[A1][0] = sx(0x004a_2020);
        c.g[A3][0] = c.g[S2][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, 0x0041_7828, 0x001d5008)?;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x003a_0c50, 0x001d5014)?;
        frame.restore(c, S0, 0x150);
        frame.restore(c, S1, 0x140);
        frame.restore(c, S2, 0x130);
        frame.restore(c, S3, 0x120);
        frame.restore(c, S4, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_Init(self)` (0x26f180): size the replay buffer, allocate its
/// blocks, then reset the replay stream.
pub fn replay_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = 0x1_0000;
        c.g[A1][0] = 0xc_0000;
        c.g[V1][0] = c.g[V1][0] | 0x4000;
        c.g[A1][0] = c.g[A1][0] | 0x9800;
        c.div(0, 5, 3);
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[V0][0] = 0x1_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[V0][0] = c.g[V0][0] | 0x4000;
        frame.save_ra(c, 0);
        c.g[S2][0] = 0;
        Addr(p(r32(c, S0), 0x14)).sw(c, c.g[V1][0]);
        c.g[S1][0] = sx(p(r32(c, S0), 0x2c));
        c.cycles += 4;
        Addr(p(r32(c, S0), 0x18)).sw(c, 0);
        c.g[A1][0] = c.lo[0];
        Addr(p(r32(c, S0), 8)).sw(c, c.g[A1][0]);
        c.g[A1][0] = Addr(p(r32(c, S0), 0x14)).lw(c);
        loop {
            c.cycles += 8;
            c.g[A0][0] = c.g[S1][0];
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            c.g[S1][0] = sx(p(r32(c, S1), 0x38));
            call_ptr(c, 0x0026_e908, 0x0026f1e8)?;
            c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[A1][0] = Addr(p(r32(c, S0), 0x14)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0026_f1d8);
                }
                continue;
            }
            c.cycles += 11;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xa;
            call_ptr(c, 0x0026_f4a0, 0x0026f204)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0026_f228, 0x0026f20c)?;
            break;
        }
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of the two chunk relocators: add `base` to every pointer
/// in the chunk and fix each entry through the object's method at `slot`.
fn chunk_resolve(c: &mut Cpu, slot: u32, entry: u32) -> Flow {
    c.cycles += 17;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S0, 0x50);
    frame.save(c, S1, 0x40);
    frame.save(c, S2, 0x30);
    c.g[S1][0] = 0;
    frame.save(c, S3, 0x20);
    c.g[S2][0] = c.g[A2][0];
    frame.save(c, S4, 0x10);
    c.g[S3][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[S4][0] = c.g[A0][0];
    c.g[V0][0] = Addr(p(r32(c, S3), 0x20)).lw(c);
    c.g[V1][0] = Addr(p(r32(c, S3), 0x1c)).lw(c);
    c.g[V0][0] = sx(p(r32(c, V0), r32(c, S2)));
    c.g[S0][0] = c.g[V0][0];
    let none = (c.g[V1][0] as i64) <= 0;
    Addr(p(r32(c, S3), 0x20)).sw(c, c.g[V0][0]);
    if !none {
        c.cycles += 1;
        c.g[A1][0] = Addr(r32(c, S0)).lw(c);
        loop {
            c.cycles += 14;
            c.g[A2][0] = c.g[S2][0];
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            c.g[A1][0] = sx(p(r32(c, A1), r32(c, S2)));
            Addr(r32(c, S0)).sw(c, c.g[A1][0]);
            c.g[S0][0] = sx(p(r32(c, S0), 4));
            c.g[V1][0] = Addr(p(r32(c, S4), 8)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), slot)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), slot + 4)).lw(c);
            let fix = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, S4), r32(c, A0)));
            call_ptr(c, fix, entry + 0x70)?;
            c.g[V1][0] = Addr(p(r32(c, S3), 0x1c)).lw(c);
            c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[V1][0] == 0 {
                break;
            }
            c.g[A1][0] = Addr(r32(c, S0)).lw(c);
            if event_due(c) {
                deliver_event_here(c, entry + 0x48);
            }
        }
    }
    c.cycles += 9;
    frame.restore(c, S0, 0x50);
    c.g[V0][0] = 1;
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore(c, S4, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `int cPS2ChunkResolve_v07(self, chunk, base)` (0x3aa520).
pub fn cps2_chunk_resolve_v07(c: &mut Cpu) {
    guest(c, |c| chunk_resolve(c, 0x40, 0x003a_a520))
}

/// `int cPS2ChunkResolve_v11(self, chunk, base)` (0x3aa830).
pub fn cps2_chunk_resolve_v11(c: &mut Cpu) {
    guest(c, |c| chunk_resolve(c, 0x60, 0x003a_a830))
}

/// `void UIVector_dtor(self, flags)` (0x3a32f0): free the vector's five
/// buffers and destroy the UI object.
pub fn ui_vector_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(0x0049_41b8);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(p(r32(c, S0), 0x7c)).lw(c);
        let null = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !null {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e98, 0x003a3324)?;
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x80)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x003a3338)?;
            c.g[A0][0] = Addr(p(r32(c, S0), 0x84)).lw(c);
        } else {
            c.g[A0][0] = Addr(p(r32(c, S0), 0x84)).lw(c);
        }
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x003a334c)?;
            c.g[A0][0] = Addr(p(r32(c, S0), 0x88)).lw(c);
        } else {
            c.g[A0][0] = Addr(p(r32(c, S0), 0x88)).lw(c);
        }
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x003a3360)?;
            c.g[A0][0] = Addr(p(r32(c, S0), 0x8c)).lw(c);
        } else {
            c.g[A0][0] = Addr(p(r32(c, S0), 0x8c)).lw(c);
        }
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x003a3374)?;
            c.g[A0][0] = c.g[S0][0];
        } else {
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 7;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_fc48, 0x003a3380)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIGroup_Render(self)` (0x398868): draw every visible node of the
/// group's list at the group's offset.
pub fn cui_group_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x10);
        frame.save_fpr(c, 21, 0x38);
        c.g[S1][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(21, 12);
        frame.save(c, S0, 0x20);
        c.mov_s(20, 13);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(r32(c, S1), 0x74));
        call_ptr(c, 0x0039_7840, 0x00398894)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x14)).lw(c);
            loop {
                c.cycles += 4;
                c.g[V0][0] = sx(((r32(c, V0) as i32) >> 6) as u32);
                c.g[V0][0] = c.g[V0][0] & 1;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = Addr(p(r32(c, S0), 4)).lw(c);
                } else {
                    c.cycles += 10;
                    c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
                    c.lwc1(12, p(r32(c, S1), 0x44));
                    c.lwc1(13, p(r32(c, S1), 0x48));
                    c.g[A0][0] = Addr(p(r32(c, V1), 0x88)).lh(c);
                    c.add_s(12, 12, 21);
                    c.add_s(13, 13, 20);
                    c.g[V0][0] = Addr(p(r32(c, V1), 0x8c)).lw(c);
                    let draw = r32(c, V0);
                    c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
                    call_ptr(c, draw, 0x003988dc)?;
                    c.g[S0][0] = Addr(p(r32(c, S0), 4)).lw(c);
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039_7718, 0x003988e8)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    break;
                }
                c.g[V0][0] = Addr(p(r32(c, S0), 0x14)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0039_88a8);
                }
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 6;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void SkyBox_SetSection(self, int id)` (0x353cf0): set the section id and,
/// when it is valid, look up the model reference's index.
pub fn sky_box_set_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let valid = (c.g[S0][0] as i64) >= 0;
        Addr(p(r32(c, S1), 0x10)).sw(c, c.g[S0][0]);
        let mut one = false;
        if !valid {
            c.cycles += 4;
            call_ptr(c, 0x0035_3d98, 0x00353d18)?;
        } else {
            one = true;
            c.cycles += 11;
            call_ptr(c, 0x002d_1bd8, 0x00353d28)?;
            c.g[A2][0] = Addr(r32(c, V0)).lw(c);
            c.g[V0][0] = 0x58;
            c.g[V1][0] = Addr(r32(c, A2)).lw(c);
            c.mult(0, 2, 16, 2);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x14)).lw(c);
            c.g[V0][0] = sx(p(r32(c, V0), r32(c, A0)));
            c.g[V1][0] = Addr(p(r32(c, V0), 0x26)).lh(c);
            let none = (c.g[V1][0] as i64) <= 0;
            c.g[A1][0] = c.g[S0][0] & 0xff;
            if !none {
                c.cycles += 6;
                c.g[V1][0] = Addr(p(r32(c, A2), 8)).lw(c);
                c.g[V0][0] = sx(r32(c, A1) << 2);
                c.g[V0][0] = sx(p(r32(c, V0), r32(c, V1)));
                c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 5;
                    c.g[V1][0] = Addr(p(r32(c, V0), 0x1c)).lw(c);
                    c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                    c.g[V0][0] = sx(r32(c, V0) >> 8);
                    let nonzero = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    if !nonzero {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                }
                c.cycles += 1;
                Addr(p(r32(c, S1), 0x14)).sw(c, c.g[V0][0]);
            }
        }
        if one {
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

/// `void* EventSelect_GetMedalText(int peak)` (0x204ff0): the medal label for
/// a peak, hashed into the world's UI text table.
pub fn event_select_get_medal_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let is1 = c.g[A1][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        let mut world = 0u64;
        let mut text = 0u64;
        if is1 {
            c.cycles += 4;
            world = Addr(gp(c, -0x848)).lw(c);
            text = sx(0x0047_0e20);
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 2) as u64;
            let lt2 = c.g[V0][0] != 0;
            world = Addr(gp(c, -0x848)).lw(c);
            if lt2 {
                c.cycles += 3;
                text = sx(0x0047_0e10);
            } else {
                c.cycles += 3;
                c.g[V0][0] = 2;
                let is2 = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = 3;
                if is2 {
                    c.cycles += 4;
                    world = Addr(gp(c, -0x848)).lw(c);
                    text = sx(0x0047_0e30);
                } else {
                    c.cycles += 2;
                    let is3 = c.g[A1][0] == c.g[V0][0];
                    world = Addr(gp(c, -0x848)).lw(c);
                    if is3 {
                        c.cycles += 2;
                        text = sx(0x0047_0e40);
                    } else {
                        c.cycles += 3;
                        text = sx(0x0047_0e10);
                    }
                }
            }
        }
        c.g[V0][0] = world;
        c.g[A0][0] = text;
        c.cycles += 14;
        c.g[S0][0] = Addr(p(r32(c, V0), 0x8c)).lw(c);
        c.g[S1][0] = Addr(p(r32(c, S0), 4)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, S1), 0x20)).lh(c);
        c.g[S0][0] = sx(p(r32(c, S0), r32(c, V0)));
        call_ptr(c, 0x0031_7670, 0x00205070)?;
        c.g[V1][0] = Addr(p(r32(c, S1), 0x24)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        let get = r32(c, V1);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, get, 0x00205080)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Avalanche_Stop(self)` (0x2d81b0): free the spawn buffer, release
/// every active instance and tell the weather manager the avalanche ended.
pub fn avalanche_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(p(r32(c, S1), 8)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        } else {
            c.cycles += 4;
            call_ptr(c, 0x0031_7e98, 0x002d81d8)?;
            Addr(p(r32(c, S1), 8)).sw(c, 0);
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        }
        c.cycles += 2;
        let done = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !done {
            c.cycles += 3;
            c.g[S0][0] = Addr(p(r32(c, S1), 4)).lw(c);
            let empty = c.g[S0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !empty {
                c.cycles += 1;
                loop {
                    c.cycles += 5;
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x002d_7dd8, 0x002d8200)?;
                    c.g[S0][0] = Addr(p(r32(c, S0), 0x2e4)).lw(c);
                    let more = c.g[S0][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_81f8);
                    }
                }
            }
            c.cycles += 12;
            Addr(r32(c, S1)).sw(c, 0);
            call_ptr(c, 0x0028_b180, 0x002d8214)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0029_def0, 0x002d8220)?;
            c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x84)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0xc)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, A0), 0xa8)).lw(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x708)).lw(c);
            let zero = c.g[V0][0] == 0;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            if !zero {
                c.cycles += 1;
                Addr(p(r32(c, V1), 0x708)).sw(c, c.g[V0][0]);
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* UIPair_v03(self, pred)` (0x39bd68): return `self` if the pair's
/// predicate matches, else the first matching child or next node.
pub fn ui_pair_v03(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x10)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x14)).lw(c);
        let pred = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, pred, 0x0039bd98)?;
        let hit = c.g[V0][0] != 0;
        let mut tail = false;
        if hit {
            c.g[V0][0] = c.g[S0][0];
        } else {
            c.cycles += 3;
            c.g[A2][0] = Addr(p(r32(c, S0), 0x78)).lw(c);
            let no_child = c.g[A2][0] == 0;
            c.g[A1][0] = c.g[S1][0];
            if no_child {
                c.cycles += 1;
                c.g[A2][0] = Addr(p(r32(c, S0), 0x7c)).lw(c);
                tail = true;
            } else {
                c.cycles += 7;
                c.g[V1][0] = Addr(p(r32(c, A2), 8)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V1), 0x18)).lh(c);
                c.g[V0][0] = Addr(p(r32(c, V1), 0x1c)).lw(c);
                let visit = r32(c, V0);
                c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                call_ptr(c, visit, 0x0039bdc0)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(p(r32(c, S0), 0x78)).lw(c);
                } else {
                    c.g[A2][0] = Addr(p(r32(c, S0), 0x7c)).lw(c);
                    tail = true;
                }
            }
        }
        if tail {
            c.cycles += 2;
            if c.g[A2][0] != 0 {
                c.g[V1][0] = Addr(p(r32(c, A2), 8)).lw(c);
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = Addr(p(r32(c, V1), 0x18)).lh(c);
                c.g[V0][0] = Addr(p(r32(c, V1), 0x1c)).lw(c);
                let visit = r32(c, V0);
                c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                call_ptr(c, visit, 0x0039bdf8)?;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
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

/// `void* FEUICustom_ctor(self, owner)` (0x1a0720): build the custom UI
/// object and push it onto the state stack.
pub fn feui_custom_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(0x0047_0000);
        frame.save(c, S1, 0x10);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x27f0));
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x50;
        Addr(r32(c, S1)).sw(c, c.g[A1][0]);
        c.g[A2][0] = 0;
        Addr(p(r32(c, S1), 0x10)).sw(c, 0);
        c.g[A1][0] = sx(0x0046_0b90);
        c.g[A3][0] = 0;
        Addr(p(r32(c, S1), 4)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x001a0760)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_e2a0, 0x001a0770)?;
        c.g[V1][0] = sx(0x0047_0000);
        Addr(p(r32(c, S0), 0x48)).sw(c, 0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x4478));
        Addr(p(r32(c, S0), 0x4c)).sw(c, 0);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        let null = c.g[S0][0] == 0;
        Addr(p(r32(c, S1), 0x10)).sw(c, c.g[S0][0]);
        if !null {
            c.cycles += 8;
            c.g[A0][0] = Addr(p(r32(c, V1), 0x20)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x24)).lw(c);
            let init = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, init, 0x001a079c)?;
            c.g[A0][0] = Addr(r32(c, S1)).lw(c);
            c.g[A1][0] = Addr(p(r32(c, S1), 0x10)).lw(c);
            c.g[A0][0] = sx(p(r32(c, A0), 0x18));
            call_ptr(c, 0x0039_f238, 0x001a07ac)?;
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMemCard_Update(self)` (0x186bf0): run the memcard state,
/// then refresh its status label when the scan finished or the state is 6.
pub fn cfe_mem_card_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001d_8de0, 0x00186c04)?;
        c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
        call_ptr(c, 0x0022_7f80, 0x00186c0c)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0023_c898, 0x00186c14)?;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[V1][0] = Addr(p(r32(c, S0), 0x1c0)).lw(c);
            c.g[V0][0] = 6;
            if c.g[V1][0] != c.g[V0][0] {
                frame.restore(c, S0, 0x10);
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 1;
        }
        c.g[A0][0] = sx(0x004a_0000);
        c.cycles += 7;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x14c0));
        call_ptr(c, 0x0031_7670, 0x00186c38)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x00186c44)?;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0039_fd38, 0x00186c54)?;
        }
        c.g[A0][0] = sx(0x0046_0000);
        c.cycles += 12;
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x2380));
        call_ptr(c, 0x0031_7670, 0x00186c60)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x00186c6c)?;
        c.g[V1][0] = Addr(p(r32(c, V0), 8)).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr(p(r32(c, V1), 0x48)).lh(c);
        c.g[A2][0] = Addr(p(r32(c, V1), 0x4c)).lw(c);
        let set = r32(c, A2);
        c.g[A0][0] = sx(p(r32(c, V0), r32(c, A0)));
        call_ptr(c, set, 0x00186c84)?;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMemCard_OnScanDone(self)` (0x187270): start the system load
/// or the profile search according to the pending flags, else finish.
pub fn cfe_mem_card_on_scan_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
        Addr(p(r32(c, S1), 0x234)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0022_7f80, 0x00187294)?;
        c.g[V1][0] = Addr(p(r32(c, S1), 0x238)).lw(c);
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = c.g[V1][0] & 2;
        let flagged = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[V1][0] ^ 1;
        if flagged {
            c.cycles += 3;
            c.g[V0][0] = c.g[V0][0] & 1;
            let finish = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if finish {
                c.cycles += 3;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0018_7318, 0x00187300)?;
            } else {
                c.cycles += 8;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0023_fb18, 0x001872e0)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0023_fae0, 0x001872e8)?;
                call_ptr(c, 0x0018_77b0, 0x001872f0)?;
            }
        } else {
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0023_fb18, 0x001872b4)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0023_fae0, 0x001872bc)?;
            call_ptr(c, 0x0018_7920, 0x001872c4)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PathData_ReadFloatIntPairs(pairs, p)` (0x26b7d8): read a count and
/// then that many (float, int) pairs, returning the bytes consumed.
pub fn path_data_read_float_int_pairs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S2, 0x40);
        c.g[A0][0] = c.g[SP][0];
        frame.save_ra(c, 0x10);
        c.g[S4][0] = c.g[S0][0];
        call_ptr(c, 0x0026_ba68, 0x0026b808)?;
        c.g[S0][0] = sx(p(r32(c, S0), 4));
        c.g[V0][0] = Addr(r32(c, SP)).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[S2][0] = sx(p(r32(c, S1), 0x18));
            c.g[S1][0] = sx(p(r32(c, S1), 0x1c));
            loop {
                c.cycles += 14;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0026_ba88, 0x0026b82c)?;
                c.g[S0][0] = sx(p(r32(c, S0), 4));
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0026_ba68, 0x0026b83c)?;
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                c.g[V0][0] = Addr(r32(c, SP)).lw(c);
                c.g[S0][0] = sx(p(r32(c, S0), 4));
                c.g[S1][0] = sx(p(r32(c, S1), 8));
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx(p(r32(c, S2), 8));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_b820);
                }
            }
        }
        c.cycles += 9;
        c.g[V0][0] = sx(r32(c, S0).wrapping_sub(r32(c, S4)));
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void FELocale_LoadTable(self, int lang)` (0x1959a8): lazily create the
/// locale file object for a language index.
pub fn fe_locale_load_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = sx(r32(c, A1) << 24);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx(((r32(c, A1) as i32) >> 24) as u32);
        frame.save(c, S2, 0x10);
        let negative = (c.g[S0][0] as i64) < 0;
        frame.save_ra(c, 0);
        let mut one = !negative;
        if !negative {
            c.cycles += 6;
            c.g[V1][0] = sx(p(r32(c, A0), 8));
            c.g[S2][0] = sx(r32(c, S0) << 2);
            c.g[S1][0] = sx(p(r32(c, V1), r32(c, S2)));
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 21;
                c.g[A1][0] = sx(0x0046_03f0);
                c.g[A0][0] = 0x14;
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x001959f8)?;
                c.g[V1][0] = sx(0x0044_0000);
                c.g[A0][0] = sx(0x0044_0000);
                Addr(r32(c, S1)).sw(c, c.g[V0][0]);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x1138));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1188));
                c.g[V1][0] = sx(p(r32(c, S2), r32(c, V1)));
                c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
                c.g[A1][0] = Addr(r32(c, V1)).lw(c);
                c.g[A2][0] = Addr(r32(c, A0)).lb(c);
                c.g[A3][0] = sx(0xffff_ffff);
                c.g[A0][0] = c.g[V0][0];
                Addr(r32(c, V0)).sb(c, c.g[A3][0]);
                Addr(p(r32(c, V0), 4)).sw(c, 0);
                Addr(p(r32(c, V0), 0xc)).sw(c, 0);
                call_ptr(c, 0x002c_6e98, 0x00195a34)?;
            } else {
                one = false;
            }
        }
        if one {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateUberTrick_ShowBuyButtons(self, int on)` (0x184780): show or
/// hide the two buy buttons of the uber-trick screen.
pub fn cfe_uber_trick_show_buy_buttons(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = sx(p(r32(c, A0), 0x24));
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0039_7840, 0x0018479c)?;
        c.g[A0][0] = sx(0x004a_1458);
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, 0x0031_7670, 0x001847ac)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_d860, 0x001847b8)?;
        c.g[A2][0] = c.g[V0][0];
        let missing = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !missing {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
            let show = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, show, 0x001847d8)?;
        }
        c.cycles += 9;
        c.g[A0][0] = sx(0x004a_1460);
        call_ptr(c, 0x0031_7670, 0x001847e4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001847f0)?;
        c.g[A2][0] = c.g[V0][0];
        let missing = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !missing {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
            let show = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, show, 0x00184810)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptCmdQueue_Push(self, const Cmd* cmd)` (0x283c80): append the
/// command to the ring and flag the queue full when the index wraps to the
/// tail.
pub fn script_cmd_queue_push(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0028_3c58, 0x00283c9c)?;
        let full = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !full {
            c.cycles += 24;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x90)).lw(c);
            c.g[V1][0] = 0x1c;
            c.g[A0][0] = c.g[S0][0];
            c.mult(0, 5, 2, 3);
            c.g[V0][0] = sx(p(r32(c, A1), r32(c, S0)));
            c.ldl(6, p(r32(c, S1), 7));
            c.ldr(6, r32(c, S1));
            c.ldl(7, p(r32(c, S1), 0xf));
            c.ldr(7, p(r32(c, S1), 8));
            c.ldl(3, p(r32(c, S1), 0x17));
            c.ldr(3, p(r32(c, S1), 0x10));
            c.g[A1][0] = Addr(p(r32(c, S1), 0x18)).lw(c);
            c.sdl(6, p(r32(c, V0), 7));
            c.sdr(6, r32(c, V0));
            c.sdl(7, p(r32(c, V0), 0xf));
            c.sdr(7, p(r32(c, V0), 8));
            c.sdl(3, p(r32(c, V0), 0x17));
            c.sdr(3, p(r32(c, V0), 0x10));
            Addr(p(r32(c, V0), 0x18)).sw(c, c.g[A1][0]);
            c.g[A1][0] = Addr(p(r32(c, S0), 0x90)).lw(c);
            call_ptr(c, 0x0028_3c60, 0x00283cf8)?;
            c.g[V1][0] = Addr(p(r32(c, S0), 0x8c)).lw(c);
            let wrapped = c.g[V0][0] != c.g[V1][0];
            Addr(p(r32(c, S0), 0x90)).sw(c, c.g[V0][0]);
            if !wrapped {
                c.cycles += 2;
                c.g[V0][0] = 1;
                Addr(p(r32(c, S0), 0x94)).sw(c, c.g[V0][0]);
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 1;
        frame.restore(c, S0, 0x20);
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable};
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP};
    use ssx3_ee as ee;

    /// Objects and buffers in the test data area (below the fixtures'
    /// reserved blocks: BE at +0x30000, WORLD at +0x80000, the fake log at
    /// 0x900000 and the pools above +0x400000).
    const OBJ: u32 = DATA;
    const OBJ2: u32 = DATA + 0x800;
    const EV: u32 = DATA + 0x1000;
    const STRIPS: u32 = DATA + 0x2000;
    const PKT: u32 = DATA + 0x3000;
    const SRC: u32 = DATA + 0x4000;
    const DST: u32 = DATA + 0x5000;
    const MEM: u32 = DATA + 0x6000;
    const LIST: u32 = DATA + 0x7000;
    const LISTARR: u32 = DATA + 0x7100;
    const ENTRY: u32 = DATA + 0x7200;
    const ENT: u32 = DATA + 0x7300;
    const SCR: u32 = DATA + 0x10000;
    const CHILD: u32 = DATA + 0x11000;
    const BUF: u32 = DATA + 0x12000;
    const ARR: u32 = DATA + 0x13000;
    const SI: u32 = DATA + 0x13100;
    const RIDER: u32 = DATA + 0x14000;
    const BANKMAN: u32 = DATA + 0x15000;
    const ENG: u32 = DATA + 0x16000;
    const SYS: u32 = DATA + 0x18000;
    const SELF2: u32 = DATA + 0x19000;
    const BE: u32 = DATA + 0x3_0000;
    const WORLD: u32 = DATA + 0x8_0000;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    fn wf(c: &mut Cpu, a: u32, f: f32) {
        c.bus.write32(a, f.to_bits());
    }

    /// The BELibrary singleton (`gp - 0x1f04`, zeroed so nothing waits for
    /// the disc) and the world (`gp - 0x848`, a dummy with no riders).
    fn globals(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x848), WORLD);
        let game = WORLD + 0x1000;
        w(c, WORLD + 0x84, game);
        let race = WORLD + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }

    /// The engine's size-class pool allocator (same fixture as the other
    /// waves): `gp + 0xdd8` ready plus eight free nodes per size class.
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

    // ---------------------------------------------------------------- front end

        #[test]
    fn cfe_online_character_select_show_opponent_char_matches() {
        check(
            ee::s03::f_001b1760__generated,
            cfe_online_character_select_show_opponent_char,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x1b1760, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 4) as u64;
                w(c, OBJ + 0x40, if seed % 3 == 0 { 0 } else { SCR });
                w(c, 0x0046_3830, 0);
            },
        );
    }

    #[test]
    fn cfe_create_step2_fill_year_list_matches() {
        check(
            ee::s03::f_001bb798__generated,
            cfe_create_step2_fill_year_list,
            8,
            &|c, seed| {
                let mut r = base(c, seed, 0x1bb798, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = SCR as u64;
                c.g[A2][0] = (seed % 200) as u64;
                w(c, SCR + 8, CHILD);
                w(c, SCR + 0x14, r.next_u32());
                w(c, CHILD + 0x48, 0);
                w(c, CHILD + 0x4c, 0);
                w(c, 0x004a_12c0, 0);
            },
        );
    }

    #[test]
    fn cfe_online_main_menu_init_matches() {
        check(
            ee::s03::f_001bfbf8__generated,
            cfe_online_main_menu_init,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x1bfbf8, 4);
                c.g[A0][0] = OBJ as u64;
                w(c, OBJ + 0x10, ENG);
                w(c, 0x0046_53f0, 0);
                if seed % 2 == 0 {
                    w(c, GAME_GP.wrapping_sub(0xc8), 0);
                } else {
                    w(c, GAME_GP.wrapping_sub(0xc8), OBJ2);
                    w(c, OBJ2, 1);
                }
                w(c, GAME_GP.wrapping_sub(0x204), 0);
            },
        );
    }

    #[test]
    fn cfe_net_quick_play_popup_set_options_hint_visible_matches() {
        check(
            ee::s03::f_001c13e0__generated,
            cfe_net_quick_play_popup_set_options_hint_visible,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x1c13e0, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, OBJ + 0x40, if seed % 4 == 0 { 0 } else { SCR });
                w(c, 0x0046_3ca0, 0);
                w(c, 0x0046_58f0, 0);
            },
        );
    }

    #[test]
    fn cfe_net_quick_messages_on_reset_popup_event_matches() {
        check(
            ee::s03::f_001c2ba8__generated,
            cfe_net_quick_messages_on_reset_popup_event,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x1c2ba8, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = EV as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0xf
                } else {
                    (seed % 8) as u64
                };
                w(c, OBJ + 0x6d0, 0);
                w(c, EV + 0x18, 0);
            },
        );
    }

    #[test]
    fn message_center_count_group_messages_matches() {
        check(
            ee::s04::f_001e2bb8__generated,
            message_center_count_group_messages,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x1e2bb8, 2);
                c.g[A0][0] = (seed % 2) as u64;
                c.g[A1][0] = ((seed / 2) % 2) as u64;
                for i in 0..61u32 {
                    let e = 0x0044_1630 + i * 0x14;
                    w(c, e, if i % 3 == 0 { 0 } else { 1 });
                    w(c, e + 8, (i + 1) & 0xffff);
                    w(c, e + 0x10, i % 2);
                }
            },
        );
    }

    #[test]
    fn lobby_man_get_rank_list_entry_matches() {
        check(
            ee::s06::f_0025f388__generated,
            lobby_man_get_rank_list_entry,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x25f388, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = OBJ2 as u64;
                c.g[A3][0] = (OBJ2 + 4) as u64;
                w(c, OBJ, (seed % 4 != 0) as u32);
                w(c, OBJ + 0xf8, (seed % 3) as u32);
                w(c, OBJ + 0x1ec, LIST);
                w(c, LIST + 0x18, 4);
                w(c, LIST + 0x40, LISTARR);
                w(c, LISTARR, if seed % 2 == 0 { ENTRY } else { 0 });
                w(c, ENTRY, ENTRY);
                w(c, 0x004a_2f30, 0);
            },
        );
    }

    #[test]
    fn c_setup_man_pop_packet_matches() {
        check(
            ee::s07::f_002649b0__generated,
            c_setup_man_pop_packet,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x2649b0, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = DST as u64;
                w(c, OBJ + 0xc, 5);
                w(c, OBJ + 8, if seed % 3 == 0 { 5 } else { 4 });
                w(c, OBJ + 0x10 + 5 * 4, PKT);
                w(c, PKT + 2, 8);
                fill(c, &mut Rng(seed as u64 ^ 0x99), PKT + 4, 4);
                w(c, OBJ + 4, 0);
                w(c, OBJ + 0x120, 0);
            },
        );
    }

    // ---------------------------------------------------------------- audio

    /// The audio system with speech channels on (a channel's volume is 0,
    /// not 1.0), a ready speech instance and a free event slot.
    fn audio_system(c: &mut Cpu, seed: u32) {
        w(c, SYS + 0x5560 + 4, ARR);
        w(c, ARR, SI);
        w(c, SI + 4, 0);
        w(c, GAME_GP.wrapping_add(0x6c4), 0);
        w(c, GAME_GP.wrapping_add(0x173c), 0);
        if seed % 4 == 0 {
            // speech off (channel volume exactly 1.0) for one channel
            for ch in 0..4u32 {
                wf(c, SYS + 0x62bc + 0x10 * ch, 1.0);
            }
        }
        if seed % 3 == 0 {
            w(c, ARR, 0);
        }
        w(c, SYS + 0x5780, 0);
    }

    #[test]
    fn ssx_audio_system_rider_say_20bd_matches() {
        check(
            ee::s08::f_002a1778__generated,
            ssx_audio_system_rider_say_20bd,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x2a1778, 3);
                audio_system(c, seed);
                c.g[A0][0] = SYS as u64;
                c.g[A1][0] = RIDER as u64;
            },
        );
    }

    #[test]
    fn ssx_audio_system_dj_say_212c_matches() {
        check(
            ee::s08::f_002a2b88__generated,
            ssx_audio_system_dj_say_212c,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x2a2b88, 4);
                audio_system(c, seed);
                c.g[A0][0] = SYS as u64;
                w(c, SYS + 0x5780, (seed % 2) as u32);
            },
        );
    }

    #[test]
    fn ssx_audio_system_pa_say_20c1_matches() {
        check(
            ee::s08::f_002a32b0__generated,
            ssx_audio_system_pa_say_20c1,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x2a32b0, 4);
                audio_system(c, seed);
                c.g[A0][0] = SYS as u64;
                w(c, SYS + 0x6290, RIDER);
            },
        );
    }

    #[test]
    fn ssx_audio_system_pa_say_20c2_matches() {
        check(
            ee::s08::f_002a3358__generated,
            ssx_audio_system_pa_say_20c2,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x2a3358, 4);
                audio_system(c, seed);
                c.g[A0][0] = SYS as u64;
                w(c, SYS + 0x6290, RIDER);
            },
        );
    }

    // ---------------------------------------------------------------- scripts

    #[test]
    fn ssx_audio_system_unload_world_banks_matches() {
        check(
            ee::s07::f_00286c00__generated,
            ssx_audio_system_unload_world_banks,
            4,
            &|c, seed| {
                let _ = base(c, seed, 0x286c00, 4);
                c.g[A0][0] = SYS as u64;
                w(c, SYS + 0x400, BANKMAN);
                w(c, SYS + 0x118, SYS + 0x400);
            },
        );
    }

    #[test]
    fn ssx_audio_system_on_reset_race_matches() {
        check(
            ee::s07::f_00287108__generated,
            ssx_audio_system_on_reset_race,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x287108, 4);
                c.g[A0][0] = SYS as u64;
                w(c, SYS + 0x5824, (seed * 3) as u32);
                w(c, WORLD + 0xc0, OBJ2);
                w(c, OBJ2 + 0x70, (seed % 4) as u32);
            },
        );
    }

    #[test]
    fn ssx_audio_system_play_char_select_music_matches() {
        check(
            ee::s07::f_0028fc58__generated,
            ssx_audio_system_play_char_select_music,
            8,
            &|c, seed| {
                let mut r = base(c, seed, 0x28fc58, 3);
                c.g[A0][0] = SYS as u64;
                w(c, SYS + 0x118, 0x1000);
                w(c, SYS + 0x5558, OBJ2);
                w(c, SYS + 0x6290, (seed % 4) as u32);
                w(c, OBJ2 + 0x10, (seed % 5) as u32);
                let vt0 = vtable(c, &mut r, &[0x20, 0x24]);
                w(c, OBJ2 + 0x14, vt0);
                let vt1 = vtable(c, &mut r, &[0x20, 0x24]);
                w(c, OBJ2 + 0x24, vt1);
            },
        );
    }

        #[test]
    fn script_engine_unload_bank_matches() {
        check(
            ee::s07::f_0027b410__generated,
            script_engine_unload_bank,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x27b410, 3);
                c.g[A0][0] = OBJ as u64;
                let idx = (seed % 3) as u32;
                c.g[A1][0] = idx as u64;
                let slot = OBJ + idx * 16 + 0x628;
                match seed % 3 {
                    0 => {
                        w(c, slot, 0);
                        w(c, slot + 8, DST);
                    }
                    1 => {
                        w(c, slot, 0);
                        w(c, slot + 8, 0);
                    }
                    _ => {
                        w(c, slot, 0x77);
                        w(c, slot + 0xc, 1);
                    }
                }
                w(c, OBJ + 0x620, 1);
                w(c, OBJ + 0x51c, 0);
            },
        );
    }

    // ---------------------------------------------------------------- rider

    #[test]
    fn rider_railslide_sub_set_rail_matches() {
        check(
            ee::s01::f_00132620__generated,
            rider_railslide_sub_set_rail,
            10,
            &|c, seed| {
                let _ = base(c, seed, 0x132620, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = match seed % 4 {
                    0 => u64::MAX,
                    1 => 7,
                    _ => (seed % 5) as u64,
                };
                w(c, OBJ + 8, RIDER);
                w(c, OBJ + 4, (seed % 4) as u32);
                wf(c, RIDER + 0x2f0, if seed % 2 == 0 { 0.0 } else { 1.5 });
                w(c, RIDER + 0xb2c, if seed % 3 == 0 { 2 } else { 0 });
                w(c, RIDER + 0x77c, OBJ2);
            },
        );
    }

        #[test]
    fn rider_metrix_start_spin_trick_matches() {
        check(
            ee::s00::f_00119c98__generated,
            rider_metrix_start_spin_trick,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x119c98, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 6) as u64;
                wf(c, OBJ + 0x14, if seed % 3 == 0 { 0.0 } else { 1.0 });
                w(c, OBJ + 0x60, 0);
            },
        );
    }

    #[test]
    fn c_player_pulse_fx_reset_matches() {
        check(
            ee::s09::f_002e3930__generated,
            c_player_pulse_fx_reset,
            24,
            &|c, seed| {
                let mut r = base(c, seed, 0x2e3930, 0);
                c.g[A0][0] = OBJ as u64;
                fill(c, &mut r, OBJ, 0x14);
                w(c, GAME_GP.wrapping_sub(0x3a34), r.next_u32());
            },
        );
    }

    // ---------------------------------------------------------------- renderer

    #[test]
    fn ps2_graphics_man_draw_shadow_edge_pass_matches() {
        check(
            ee::s12::f_00386dd0__generated,
            ps2_graphics_man_draw_shadow_edge_pass,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x386dd0, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = STRIPS as u64;
                c.g[A2][0] = (1 + seed % 3) as u64;
                c.g[A3][0] = 0x11;
                c.g[T0][0] = 0x22;
                c.g[T1][0] = 0x33;
                for i in 0..3u32 {
                    let s = STRIPS + 0x14 * i;
                    w(c, s, SRC);
                    w(c, s + 4, SRC + 0x100);
                    w(c, s + 8, if (seed + i) % 3 == 0 { 1 } else { 2 });
                    wf(c, s + 0xc, 0.5);
                    wf(c, s + 0x10, 0.75);
                }
            },
        );
    }

        #[test]
    fn multi_particle_ctor_matches() {
        check(
            ee::s11::f_00357dd8__generated,
            multi_particle_ctor,
            8,
            &|c, seed| {
                let mut r = base(c, seed, 0x357dd8, 4);
                c.g[A0][0] = MEM as u64;
                c.g[A1][0] = SRC as u64;
                c.g[A2][0] = (seed % 8) as u64;
                fill(c, &mut r, SRC, 0x40);
                w(c, 0x0048_ea10, 0);
            },
        );
    }

    #[test]
    fn snow_surface_menu_dtor_matches() {
        check(
            ee::s06::f_0024fe00__generated,
            snow_surface_menu_dtor,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x24fe00, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, OBJ + 0x130, 0);
                w(c, OBJ + 0x164, 0);
            },
        );
    }

    #[test]
    fn ui_vector_dtor_matches() {
        check(
            ee::s13::f_003a32f0__generated,
            ui_vector_dtor,
            10,
            &|c, seed| {
                let _ = base(c, seed, 0x3a32f0, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, OBJ + 0x7c, if seed % 2 == 0 { 0 } else { DST });
                w(c, OBJ + 0x80, if seed % 3 == 0 { 0 } else { DST });
                w(c, OBJ + 0x84, 0);
                w(c, OBJ + 0x88, DST);
                w(c, OBJ + 0x8c, 0);
            },
        );
    }

            // ---------------------------------------------------------------- effects

    #[test]
    fn c_transition_man_ctor_matches() {
        check(
            ee::s09::f_002e4228__generated,
            c_transition_man_ctor,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x2e4228, 4);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (seed * 7 % 11) as u64;
                fill(c, &mut Rng(seed as u64 ^ 0x1234), OBJ, 0x90);
            },
        );
    }

    #[test]
    fn boost_v48_matches() {
        check(
            ee::s11::f_00341770__generated,
            boost_v48,
            8,
            &|c, seed| {
                let mut r = base(c, seed, 0x341770, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                let vt4 = vtable(c, &mut r, &[0x10, 0x14, 0x20, 0x24]);
                w(c, OBJ2, vt4);
                wf(c, OBJ + 0x34, 0.5);
                w(c, OBJ + 0x60, 0);
            },
        );
    }

    #[test]
    fn boost_modifier_v08_matches() {
        check(
            ee::s11::f_003429e0__generated,
            boost_modifier_v08,
            8,
            &|c, seed| {
                let mut r = base(c, seed, 0x3429e0, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                let vt5 = vtable(c, &mut r, &[0x10, 0x14, 0x20, 0x24]);
                w(c, OBJ2, vt5);
                wf(c, OBJ + 0x24, 0.25);
                w(c, OBJ + 0x60, 0);
            },
        );
    }

        // ---------------------------------------------------------------- tables

    #[test]
    fn bolts_index_table2_by_char_matches() {
        check(
            ee::s01::f_0014c320__generated,
            bolts_index_table2_by_char,
            6,
            &|c, seed| {
                let mut r = base(c, seed, 0x14c320, 2);
                c.g[A0][0] = OBJ as u64;
                let n = (seed % 4) as u32;
                w(c, OBJ + 0x20, n);
                w(c, OBJ + 8, SRC);
                fill(c, &mut r, SRC, 4 * 4);
                for i in 0..n {
                    c.bus.write8(SRC + 0xc * i, ((i * 3) % 30) as u8);
                }
            },
        );
    }

    #[test]
    fn world_trigger_manager_find_instance_matches() {
        check(
            ee::s08::f_002b5bc0__generated,
            world_trigger_manager_find_instance,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x2b5bc0, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = 1;
                c.g[A2][0] = 2;
                c.g[A3][0] = 3;
                w(c, OBJ, SELF2);
                for i in 0..40u32 {
                    w(c, SELF2 + 0x70 * i, (i % 4 == 0) as u32);
                }
                let hit = (seed % 40) as u32;
                w(c, SELF2 + 0x70 * hit, 1);
                w(c, SELF2 + 0x70 * hit + 4, 1);
                w(c, SELF2 + 0x70 * hit + 8, 2);
                w(c, SELF2 + 0x70 * hit + 0x3c, 3);
            },
        );
    }

    #[test]
    fn world_block_pool_dtor_matches() {
        check(
            ee::s13::f_003a76c0__generated,
            world_block_pool_dtor,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x3a76c0, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 2) as u64;
                let n = (seed % 4) as u32;
                w(c, OBJ + 4, n);
                w(c, OBJ + 0xc, LISTARR);
                for i in 0..4u32 {
                    w(c, LISTARR + 0x18 * i, if (seed + i) % 3 == 0 { 0 } else { DST });
                }
            },
        );
    }

    #[test]
    fn cfe_rewards_room_set_count_text_matches() {
        check(
            ee::s04::f_001d4f90__generated,
            cfe_rewards_room_set_count_text,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x1d4f90, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = OBJ2 as u64;
                w(c, OBJ + 0x44, seed % 10);
                w(c, OBJ + 0x48, seed % 5);
                w(c, 0x004a_2020, 0);
            },
        );
    }

    #[test]
    fn replay_init_matches() {
        check(
            ee::s07::f_0026f180__generated,
            replay_init,
            4,
            &|c, seed| {
                let _ = base(c, seed, 0x26f180, 4);
                c.g[A0][0] = OBJ as u64;
                fill(c, &mut Rng(seed as u64 ^ 0x26f1), OBJ, 0x120 / 4);
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v07_matches() {
        check(
            ee::s13::f_003aa520__generated,
            cps2_chunk_resolve_v07,
            8,
            &|c, seed| {
                let mut r = base(c, seed, 0x3aa520, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = 0x100;
                let vt7 = vtable(c, &mut r, &[0x40, 0x44, 0x60, 0x64]);
                w(c, OBJ + 8, vt7);
                let n = (seed % 3) as u32;
                w(c, OBJ2 + 0x1c, n);
                w(c, OBJ2 + 0x20, 0);
                w(c, OBJ2 + 0x24, 0);
                w(c, OBJ2 + 0x28, 0);
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v11_matches() {
        check(
            ee::s13::f_003aa830__generated,
            cps2_chunk_resolve_v11,
            8,
            &|c, seed| {
                let mut r = base(c, seed, 0x3aa830, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = 0x100;
                let vt8 = vtable(c, &mut r, &[0x40, 0x44, 0x60, 0x64]);
                w(c, OBJ + 8, vt8);
                let n = (seed % 3) as u32;
                w(c, OBJ2 + 0x1c, n);
                w(c, OBJ2 + 0x20, 0);
                w(c, OBJ2 + 0x24, 0);
                w(c, OBJ2 + 0x28, 0);
            },
        );
    }

    #[test]
    fn event_select_get_medal_text_matches() {
        check(
            ee::s05::f_00204ff0__generated,
            event_select_get_medal_text,
            10,
            &|c, seed| {
                let r = base(c, seed, 0x204ff0, 2);
                c.g[A1][0] = (seed % 6) as u64;
                w(c, WORLD + 0x8c, OBJ2);
                w(c, OBJ2 + 4, LIST);
                w(c, LIST + 0x20, 0);
                w(c, LIST + 0x24, 0);
                let _ = r;
            },
        );
    }

        #[test]
    fn ui_pair_v03_matches() {
        check(
            ee::s13::f_0039bd68__generated,
            ui_pair_v03,
            10,
            &|c, seed| {
                let mut r = base(c, seed, 0x39bd68, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                let vt = vtable(c, &mut r, &[0x10, 0x14]);
                if seed % 2 == 1 {
                    // UIList_First on a zeroed list returns 0: take the child
                    // path instead of the predicate-hit one.
                    w(c, vt + 0x10, 0);
                    w(c, vt + 0x14, 0x0039_7840);
                }
                w(c, OBJ2, vt);
                w(c, OBJ + 0x78, if seed % 4 == 0 { 0 } else { CHILD });
                w(c, OBJ + 0x7c, if seed % 3 == 0 { 0 } else { SELF2 });
                let vt9 = vtable(c, &mut r, &[0x18, 0x1c]);
                w(c, CHILD + 8, vt9);
                let vt10 = vtable(c, &mut r, &[0x18, 0x1c]);
                w(c, SELF2 + 8, vt10);
            },
        );
    }

    #[test]
    fn feui_custom_ctor_matches() {
        check(
            ee::s03::f_001a0720__generated,
            feui_custom_ctor,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x1a0720, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                w(c, 0x0046_0b90, 0);
                w(c, OBJ2, if seed % 3 == 0 { 0 } else { SELF2 });
            },
        );
    }

    #[test]
    fn cfe_mem_card_update_matches() {
        check(
            ee::s02::f_00186bf0__generated,
            cfe_mem_card_update,
            4,
            &|c, seed| {
                let _ = base(c, seed, 0x186bf0, 4);
                c.g[A0][0] = OBJ as u64;
                // A manager that is idle (IsIdle of a zeroed manager) and a
                // base update that returns immediately.
                w(c, GAME_GP.wrapping_sub(0x47c), OBJ2);
                w(c, GAME_GP.wrapping_sub(0x848), WORLD);
                w(c, OBJ + 0x218, 1);
                w(c, OBJ + 0x1c0, if seed % 3 == 0 { 6 } else { (seed % 8) as u32 });
                w(c, OBJ + 0x40, SCR);
                w(c, 0x004a_14c0, 0);
                w(c, 0x0045_dc80, 0);
            },
        );
    }

    #[test]
    fn cfe_mem_card_on_scan_done_matches() {
        check(
            ee::s02::f_00187270__generated,
            cfe_mem_card_on_scan_done,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x187270, 4);
                c.g[A0][0] = OBJ as u64;
                w(c, GAME_GP.wrapping_sub(0x47c), OBJ2);
                w(c, OBJ + 0x234, 0);
                w(c, OBJ + 0x238, (seed % 4) as u32);
            },
        );
    }

    #[test]
    fn path_data_read_float_int_pairs_matches() {
        check(
            ee::s07::f_0026b7d8__generated,
            path_data_read_float_int_pairs,
            6,
            &|c, seed| {
                let mut r = base(c, seed, 0x26b7d8, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = SRC as u64;
                w(c, SRC, (seed % 4) as u32);
                fill(c, &mut r, SRC + 4, 16);
                w(c, OBJ + 0x18, 0);
                w(c, OBJ + 0x1c, 0);
                w(c, OBJ + 0x20, 0);
                w(c, OBJ + 0x24, 0);
            },
        );
    }

        #[test]
    fn cfe_uber_trick_show_buy_buttons_matches() {
        check(
            ee::s02::f_00184780__generated,
            cfe_uber_trick_show_buy_buttons,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x184780, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, OBJ + 0x28, ENT);
                w(c, ENT + 4, if seed % 3 == 0 { ENT } else { 0 });
                w(c, 0x004a_1458, 0);
                w(c, 0x004a_1460, 0);
            },
        );
    }

    }

