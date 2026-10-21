//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold-path front-end, UI, font, script, audio, net and
//! data helpers, each with a differential test against the generated body.
//!
//! Bodies are transcribed from the generated Rust (the authority for
//! registers, cycles and call order). Calls go through [`crate::call`] with
//! the original return address. Loops make the original's back-edge event
//! checks with [`crate::events::event_due`] / `deliver_event_here` exactly
//! where the generated code does.

use crate::abi::r32;
use crate::call::{call_ptr, guest, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
        0x001c_0240,
        cfe_online_main_menu_on_disconnect_popup_event,
        "cFEStateOnlineMainMenu_OnDisconnectPopupEvent",
    ),(0x0039_2ba8, font_get_char_width, "Font_GetCharWidth"),
    (0x0039_2c60, font_get_char_width_w, "Font_GetCharWidthW"),
    (0x002c_6de0, loc_string_table_find, "LocStringTable_Find"),
    (0x001d_5280, cfe_relogin_v33, "cFEStateRelogin_v33"),
    (
        0x0019_bbe0,
        cfe_state_char_equip_set_category_title,
        "cFEStateCharEquip_SetCategoryTitle",
    ),
    (0x0020_a380, cov_state_overlay_init, "cOVStateOverlay_Init"),
    (0x0030_88d8, wscript_man_dtor, "WScriptMan_dtor"),
    (
        0x0030_7e10,
        wscript_process_next_task,
        "WScriptProcess_NextTask",
    ),
    (0x0030_8278, wscript_task_init, "WScriptTask_Init"),
    (
        0x0017_abe0,
        debug_menu_free_backgrounds,
        "DebugMenu_FreeBackgrounds",
    ),(0x0025_ee20, lobby_man_start_quick_play, "LobbyMan_StartQuickPlay"),(
        0x0029_7eb8,
        ssx_audio_system_stop_object_loop,
        "SSXAudioSystem_StopObjectLoop",
    ),(0x0039_d968, cui_scroll_render, "cUIScroll_Render"),(
        0x0019_bc90,
        cfe_state_char_equip_update_stat_bar,
        "cFEStateCharEquip_UpdateStatBar",
    ),
    (
        0x0032_17d0,
        input_parser_lookup_modifier,
        "InputParser_LookupModifier",
    ),
    (
        0x0014_7980,
        benew_player_int_is_message_flag_set,
        "BENewPlayerInt_IsMessageFlagSet",
    ),
    (
        0x0019_cc30,
        bolt_file_index_manager_set_model_slot,
        "BoltFileIndexManager_SetModelSlot",
    ),
    (
        0x0019_cd38,
        bolt_file_index_manager_set_texture_slot,
        "BoltFileIndexManager_SetTextureSlot",
    ),
    (
        0x0030_8aa0,
        wscript_man_create_local_tables,
        "WScriptMan_CreateLocalTables",
    ),
    (
        0x002b_6630,
        world_trigger_manager_count_active_others,
        "WorldTriggerManager_CountActiveOthers",
    ),(0x001d_99c8, cfe_popup_confirm_ctor, "cFEPopupConfirm_ctor"),(0x001f_3790, cfe_state_lodge_init, "cFEStateLodge_Init"),
    (
        0x0028_cd48,
        ssx_audio_system_play_peak_transition_music,
        "SSXAudioSystem_PlayPeakTransitionMusic",
    ),
    (
        0x0019_88d8,
        cfe_request_line_buy_song,
        "cFEStateRequestLine_BuySong",
    ),
    (
        0x0019_47f8,
        cfe_state_title_transition_in,
        "cFEStateTitle_TransitionIn",
    ),
    (
        0x002a_4660,
        ssx_audio_system_store_race_result,
        "SSXAudioSystem_StoreRaceResult",
    ),
    (0x0011_de60, rider_place_at_marker, "Rider_PlaceAtMarker"),
    (
        0x002c_42b0,
        cps2_mem_card_do_get_save_dir_entry,
        "cPS2MemCard_DoGetSaveDirEntry",
    ),
    (
        0x001a_5fb8,
        cfe_state_net_messenger_highlight_row,
        "cFEStateNetMessenger_HighlightRow",
    ),
    (
        0x0021_2080,
        ov_screen_profile_save_pda_init_slot_labels,
        "OVScreenProfileSavePDA_InitSlotLabels",
    ),(0x0034_c548, cflag_manager_add_flag, "cFlagManager_AddFlag"),
    (0x0031_4cb0, mdf_file_create_model, "MdfFile_CreateModel"),
    (
        0x001a_91b0,
        net_fe_state_create_keyboard_popup,
        "NetFEState_CreateKeyboardPopup",
    ),
    (
        0x002a_24b0,
        ssx_audio_system_dj_say_20cc,
        "SSXAudioSystem_DJSay20CC",
    ),
    (0x0023_c560, gm_handler7_v06, "GMHandler7_v06"),(
        0x0018_eeb8,
        cfe_state_profile_load_on_widget_notify,
        "cFEStateProfileLoad_OnWidgetNotify",
    ),(0x0035_95d8, spline_modifier_ctor_2, "SplineModifier_ctor_2"),
];

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

// ---------------------------------------------------------------------------
// Front end.

/// `void cFEStateNetNews_OnWidgetAction(self, widget, int msg)` (0x1c5368):
/// the net-news screen's widget router. Message 8 opens the messenger when
/// no other popup is up, 6 builds and enters the SSX World state, 5 is
/// ignored and everything else (and a missing widget) goes to the base net
/// state handler.
pub fn cfe_state_net_news_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let msg = c.g[A2][0];
        let mut tail = 1; // the `cycles += 1` only the shared exit charges
        if c.g[A1][0] == 0 {
            c.cycles += 2;
            c.g[A0][0] = this;
            call_ptr(c, 0x001a_8918, 0x001c_540c)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 6;
            let is6 = msg == 6;
            c.g[V0][0] = (msg < 7) as u64;
            if is6 {
                // message 6: enter a fresh SSX World state
                net_news_enter_ssx_world(c)?;
                tail = 0;
            } else {
                c.cycles += 2;
                let ge7 = c.g[V0][0] == 0;
                c.g[V0][0] = 5;
                if !ge7 {
                    c.cycles += 2;
                    if msg == 5 {
                        tail = 0;
                    } else {
                        c.cycles += 2;
                        c.cycles += 2;
                        c.g[A0][0] = this;
                        call_ptr(c, 0x001a_8918, 0x001c_540c)?;
                    }
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 8;
                    if msg != 8 {
                        c.cycles += 2;
                        c.g[A0][0] = this;
                        call_ptr(c, 0x001a_8918, 0x001c_540c)?;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x1c)).lw(c);
                        c.g[V0][0] = c.g[V0][0] & 0x6;
                        if c.g[V0][0] == 0 {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x001a_9b58, 0x001c_53c4)?;
                        }
                        tail = 0;
                    }
                }
            }
        }
        c.cycles += tail;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The `case 6` body of [`cfe_state_net_news_on_widget_action`]: allocate a
/// 0x6d0-byte SSX World state, construct it and enter it.
fn net_news_enter_ssx_world(c: &mut Cpu) -> Flow {
    c.cycles += 14;
    c.g[A1][0] = 0x460000;
    c.g[A0][0] = 0x6d0;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x11a8));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, 0x001c_53e4)?;
    c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x10)).lw(c);
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x001c_1c50, 0x001c_53f0)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x001a_8a40, 0x001c_53fc)?;
    Ok(())
}

/// `void cFEStateOnlineMainMenu_OnDisconnectPopupEvent(self, item, msg)`
/// (0x1c0240): on message 0xf tear down the current net connection and push
/// the account memory-card popup, otherwise offer the event to the base net
/// state handler.
pub fn cfe_online_main_menu_on_disconnect_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        if c.g[A2][0] != 0xf {
            c.cycles += 3;
            c.g[A0][0] = this;
            call_ptr(c, 0x001a_97b8, 0x001c_02e4)?;
        } else {
            c.cycles += 33;
            c.g[A2][0] = 0xf;
            call_ptr(c, 0x001a_97b8, 0x001c_0260)?;
            c.g[A0][0] = Addr(p(r32(c, GP), 0x238)).lw(c);
            call_ptr(c, 0x0026_1408, 0x001c_0268)?;
            c.g[A0][0] = Addr(p(r32(c, GP), 0x238)).lw(c);
            call_ptr(c, 0x0026_1070, 0x001c_0270)?;
            c.g[A0][0] = Addr(p(r32(c, GP), 0x238)).lw(c);
            call_ptr(c, 0x0026_1058, 0x001c_0278)?;
            c.g[A0][0] = Addr(p(r32(c, GP), 0xffff_ff38)).lw(c);
            call_ptr(c, 0x0025_b800, 0x001c_0280)?;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = 0xb10;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2d10));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x001c_0298)?;
            c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x10)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[T0][0] = 0;
            c.g[T1][0] = 0;
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = 0;
            call_ptr(c, 0x001b_6ed0, 0x001c_02b4)?;
            c.g[V1][0] = 0x832;
            c.g[A1][0] = c.g[V0][0];
            c.sw(p(c.g[V0][0] as u32, 0x18), c.g[V1][0]);
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x40)).lw(c);
            c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0xd0)).lw(c);
            c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x10)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039_f290, 0x001c_02d4)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateLobbyMenu_UpdatePlayerScrollArrows(self)` (0x1be720): enable
/// the up arrow's widget when there are entries, and the down arrow's widget
/// unless the player is already at the bottom of the list.
pub fn cfe_lobby_menu_update_player_scroll_arrows(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let count = Addr(this as u32).off(0x778).lw(c);
        c.g[A2][0] = Addr(this as u32).off(0x728).lw(c);
        if count == 0 {
            c.cycles += 8;
            set_widget_enabled(c, 0, 0x001b_e754)?;
            c.cycles += 5;
        } else {
            c.cycles += 7;
            set_widget_enabled(c, 1, 0x001b_e774)?;
            c.cycles += 5;
        }
        c.g[V0][0] = Addr(this as u32).off(0x778).lw(c);
        c.g[A2][0] = Addr(this as u32).off(0x72c).lw(c);
        c.g[V1][0] = Addr(this as u32).off(0x77c).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xa));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            set_widget_enabled(c, 1, 0x001b_e7c4)?;
        } else {
            c.cycles += 8;
            set_widget_enabled(c, 0, 0x001b_e7a4)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Call `widget->v09(enabled)` through the vtable at `*(A2 + 8)` (slot 0x48).
fn set_widget_enabled(c: &mut Cpu, enabled: u32, ret: u32) -> Flow {
    c.g[A1][0] = enabled as u64;
    c.g[V0][0] = Addr(p(c.g[A2][0] as u32, 8)).lw(c);
    c.g[A0][0] = Addr(c.g[V0][0] as u32).lh(c);
    c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x4c)).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `void cFEStateRequestLine_UpdateScrollArrows(self)` (0x197db8): the same
/// two-arrow update for the jukebox request list (widgets at +0x1ec/+0x1f0).
pub fn cfe_request_line_update_scroll_arrows(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let count = Addr(this as u32).off(0x150).lw(c);
        c.g[A2][0] = Addr(this as u32).off(0x1ec).lw(c);
        if count == 0 {
            c.cycles += 8;
            set_widget_enabled(c, 0, 0x0019_7dec)?;
            c.cycles += 5;
        } else {
            c.cycles += 7;
            set_widget_enabled(c, 1, 0x0019_7e0c)?;
            c.cycles += 5;
        }
        c.g[V0][0] = Addr(this as u32).off(0x150).lw(c);
        c.g[A2][0] = Addr(this as u32).off(0x1f0).lw(c);
        c.g[V1][0] = Addr(this as u32).off(0x14c).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(8));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            set_widget_enabled(c, 1, 0x0019_7e5c)?;
        } else {
            c.cycles += 8;
            set_widget_enabled(c, 0, 0x0019_7e3c)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRelogin_v33(self, item)` (0x1d5280): widget event 0xf/0x14
/// tears down the relogin UI, marks the current UI state (0x80) and enters the
/// "relogin failed" state; every other event goes to the base handler.
pub fn cfe_relogin_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let msg = c.g[A2][0];
        let big = if msg == 0xf {
            true
        } else {
            c.cycles += 3;
            msg == 0x14
        };
        if !big {
            c.cycles += 3;
            c.g[A0][0] = this;
            call_ptr(c, 0x001a_97b8, 0x001d_5320)?;
        } else {
            c.cycles += 29;
            c.g[A0][0] = this;
            call_ptr(c, 0x001a_97b8, 0x001d_52ac)?;
            c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x10)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039_f698, 0x001d_52b8)?;
            c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x10)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039_fa88, 0x001d_52c4)?;
            c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x1c)).lw(c);
            c.g[V1][0] = c.g[V1][0] | 0x80;
            c.sw(p(c.g[V0][0] as u32, 0x1c), c.g[V1][0]);
            call_ptr(c, 0x0025_91b8, 0x001d_52d4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0026_36f0, 0x001d_52e0)?;
            c.g[A1][0] = 0x460000;
            c.g[A3][0] = 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xbf0));
            c.g[A0][0] = 0x91c;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x001d_52f8)?;
            c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x10)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x001b_3f78, 0x001d_5304)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x001a_8a40, 0x001d_5310)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Fonts.

/// `float Font_GetCharWidth(sGlyphTable* t, char c)` (0x392ba8): binary
/// search the glyph table for `c`; a found glyph's width, else 0.0f.
pub fn font_get_char_width(c: &mut Cpu) {
    c.cycles += 7;
    let font = c.g[A0][0];
    c.g[V1][0] = font;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
    c.g[A3][0] = Addr(p(font as u32, 0xc)).lw(c);
    c.g[A0][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A3][0] as i64)) as u64;
    let below_range = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    'done: {
        if below_range {
            break 'done;
        }
        'search: {
            c.cycles += 4;
            c.g[A1][0] = Addr(p(font as u32, 0x10)).lw(c);
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[T0][0] = Addr(font as u32).lw(c);
                break 'search;
            }
            c.cycles += 7;
            c.g[V0][0] = 0xc;
            c.g[V1][0] = Addr(p(font as u32, 4)).lw(c);
            c.mult(0, 4, 4, 2);
            c.mult(0, 2, 7, 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
            break 'done;
        }
        c.cycles += 4;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[A3][0] as u32)));
        c.g[T2][0] = Addr(p(c.g[V1][0] as u32, 4)).lw(c);
        c.g[T1][0] = 0xc;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[T0][0] as u32)));
        'l: loop {
            c.cycles += 6;
            c.g[A3][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
            c.mult(0, 6, 7, 9);
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[T2][0] as u32)));
            c.g[A2][0] = c.lhu(c.g[V0][0] as u32);
            let hit = c.g[A0][0] == c.g[A2][0];
            if hit {
                break 'done;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            } else {
                c.cycles += 1;
                c.g[T0][0] = c.g[A3][0];
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[T0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[T0][0] as u32)));
            if !more {
                c.cycles += 1;
                c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 8)).lw(c);
                break 'done;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0039_2c00);
            }
        }
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.g[V0][0] = Addr(p(c.g[V0][0] as u32, 8)).lb(c);
        c.cycles += 4;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
    } else {
        c.cycles += 3;
        c.mtc1(0, 0);
    }
    c.pc = c.g[RA][0] as u32;
}

/// `float Font_GetCharWidthW(sGlyphTable* t, unsigned short c)` (0x392c60):
/// the same lookup for wide characters.
pub fn font_get_char_width_w(c: &mut Cpu) {
    c.cycles += 6;
    let font = c.g[A0][0];
    c.g[V1][0] = font;
    c.g[A3][0] = Addr(p(font as u32, 0xc)).lw(c);
    c.g[A0][0] = c.g[A1][0] & 0xffff;
    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A3][0] as i64)) as u64;
    let below_range = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    'done: {
        if below_range {
            break 'done;
        }
        'search: {
            c.cycles += 4;
            c.g[A2][0] = Addr(p(font as u32, 0x10)).lw(c);
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[A1][0] = Addr(font as u32).lw(c);
                break 'search;
            }
            c.cycles += 7;
            c.g[V0][0] = 0xc;
            c.g[V1][0] = Addr(p(font as u32, 4)).lw(c);
            c.mult(0, 4, 4, 2);
            c.mult(0, 2, 7, 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
            break 'done;
        }
        c.cycles += 5;
        c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[A3][0] as u32)));
        c.g[T2][0] = Addr(p(c.g[V1][0] as u32, 4)).lw(c);
        c.g[T1][0] = 0xc;
        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        'l: loop {
            c.cycles += 6;
            c.g[A3][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
            c.mult(0, 6, 7, 9);
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[T2][0] as u32)));
            c.g[A2][0] = c.lhu(c.g[V0][0] as u32);
            let hit = c.g[A0][0] == c.g[A2][0];
            if hit {
                break 'done;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            } else {
                c.cycles += 1;
                c.g[A1][0] = c.g[A3][0];
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            if !more {
                c.cycles += 1;
                c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 8)).lw(c);
                break 'done;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0039_2cb8);
            }
        }
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.g[V0][0] = Addr(p(c.g[V0][0] as u32, 8)).lb(c);
        c.cycles += 4;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
    } else {
        c.cycles += 3;
        c.mtc1(0, 0);
    }
    c.pc = c.g[RA][0] as u32;
}

// ---------------------------------------------------------------------------
// Localisation.

/// `char* LocStringTable_Find(LocStringTable* t, int key)` (0x2c6de0): binary
/// search the sorted (key, string) table; 0 when the key is absent or the
/// table is empty.
pub fn loc_string_table_find(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0xc)).lw(c);
    c.g[T2][0] = 0;
    let empty = c.g[V0][0] == 0;
    c.g[T0][0] = Addr(p(c.g[A0][0] as u32, 8)).lw(c);
    if empty {
        c.cycles += 2;
        c.g[V0][0] = 0;
        c.pc = c.g[RA][0] as u32;
        return;
    }
    'miss: {
        c.cycles += 7;
        c.g[A0][0] = Addr(p(c.g[A0][0] as u32, 0x10)).lw(c);
        c.g[V1][0] = sx((c.g[T0][0] as u32) << 3);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
        let past_end = c.g[V0][0] != 0;
        c.g[T1][0] = c.g[A0][0];
        if past_end {
            break 'miss;
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[T1][0] as u32).lw(c);
        c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
        let before_start = c.g[V0][0] == 0;
        c.g[A2][0] = sx(((c.g[T0][0] as u32) as i32 >> 1) as u32);
        if !before_start {
            break 'miss;
        }
        'found: {
            c.cycles += 5;
            c.g[A0][0] = sx((c.g[A2][0] as u32) << 3);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T1][0] as u32)));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            let hit = c.g[V1][0] == c.g[A1][0];
            c.g[A3][0] = c.g[T1][0];
            if hit {
                break 'found;
            }
            c.cycles += 2;
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
            loop {
                c.cycles += 4;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] == 0 {
                    c.g[T2][0] = c.g[A2][0];
                } else {
                    c.cycles += 1;
                    c.g[T0][0] = c.g[A2][0];
                }
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub((c.g[T2][0] as u32)));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                let converged = c.g[V0][0] == c.g[A2][0];
                c.g[A2][0] = c.g[V0][0];
                if converged {
                    break 'miss;
                }
                c.cycles += 5;
                c.g[A0][0] = sx((c.g[A2][0] as u32) << 3);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                let different = c.g[V1][0] != c.g[A1][0];
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                if !different {
                    break 'found;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_6e40);
                }
            }
        }
        c.cycles += 4;
        c.g[V1][0] = sx((c.g[A2][0] as u32) << 3);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T1][0] as u32)));
        let ret = c.g[RA][0] as u32;
        c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 4)).lw(c);
        c.pc = ret;
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    c.pc = c.g[RA][0] as u32;
}


/// `void cFEStateCharEquip_SetCategoryTitle(self, int id)` (0x19bbe0): write
/// the category's localised title into the screen's text widget; `id` picks a
/// table entry by hash, with a special case for the "default" category id.
pub fn cfe_state_char_equip_set_category_title(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[V0][0] = Addr(p(this as u32, 0x58)).lw(c);
        let no_text = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if no_text {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, GP), 0xffff_e0e0)).lw(c);
            let default_id = c.g[S1][0] != c.g[V0][0];
            c.g[A0][0] = 0x460000;
            if !default_id {
                // the "default" category's own name
                c.cycles += 7;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x9b0));
                call_ptr(c, 0x0031_7670, 0x0019_bc18)?;
                c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x58)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_1000, 0x0019_bc24)?;
                frame.restore(c, S0, 0x30);
            } else {
                // look `id` up in the BE entry table and hash its name
                c.cycles += 10;
                call_ptr(c, 0x0014_bdb8, 0x0019_bc34)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0xbc)).lw(c);
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, 0x0014_d448, 0x0019_bc48)?;
                c.g[A0][0] = c.g[V0][0];
                let found = (c.g[A0][0] as i64) >= 0;
                c.g[V0][0] = 0;
                if found {
                    c.cycles += 4;
                    c.g[V0][0] = 0x38;
                    c.g[V1][0] = Addr(p(c.g[S2][0] as u32, 4)).lw(c);
                    c.mult(0, 2, 4, 2);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                }
                c.cycles += 5;
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x14)).lw(c);
                call_ptr(c, 0x0031_7670, 0x0019_bc6c)?;
                c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x58)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_1000, 0x0019_bc78)?;
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cOVStateOverlay_Init(self)` (0x20a380): initialise the online
/// overlay state, asking the current race manager's overlay object (if any)
/// to take effect.
pub fn cov_state_overlay_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        c.g[V0][0] = Addr(p(r32(c, GP), 0xffff_f7b8)).lw(c);
        c.sw(p(this as u32, 0x50), 0);
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x84)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[S0][0] = Addr(p(this as u32, 0x10)).lw(c);
        } else {
            c.cycles += 4;
            c.g[A0][0] = Addr(p(c.g[A0][0] as u32, 0x64)).lw(c);
            c.g[S0][0] = Addr(p(c.g[A0][0] as u32, 0x10)).lw(c);
            if c.g[S0][0] == 0 {
                c.g[S0][0] = Addr(p(this as u32, 0x10)).lw(c);
            } else {
                c.cycles += 5;
                call_ptr(c, 0x002e_a820, 0x0020_a3c0)?;
                c.g[V1][0] = 0x1;
                let one = c.g[S0][0] == c.g[V1][0];
                c.g[V0][0] = 0x3;
                let mut enable = one;
                if !one {
                    c.cycles += 2;
                    if c.g[S0][0] == c.g[V0][0] {
                        enable = true;
                    } else {
                        c.g[S0][0] = Addr(p(this as u32, 0x10)).lw(c);
                    }
                }
                if enable {
                    c.cycles += 2;
                    c.sw(p(this as u32, 0x50), c.g[V1][0]);
                    c.g[S0][0] = Addr(p(this as u32, 0x10)).lw(c);
                }
            }
        }
        c.cycles += 10;
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20d0));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0031_7670, 0x0020_a3ec)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_f9d8, 0x0020_a3f8)?;
        c.g[A3][0] = c.g[V0][0];
        let missing = c.g[A3][0] == 0;
        c.g[A1][0] = 0x3;
        if !missing {
            c.cycles += 6;
            c.g[V1][0] = Addr(p(c.g[A3][0] as u32, 8)).lw(c);
            c.g[A2][0] = 0;
            c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0xc0)).lh(c);
            c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0xc4)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0020_a41c)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_dtor(self, int flags)` (0x3088d8): free the script
/// manager's local tables and process lists, then optionally the object.
pub fn wscript_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x490000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6400));
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[S1][0] = c.g[A1][0];
        c.sw(p(this as u32, 0x494), c.g[V0][0]);
        call_ptr(c, 0x0030_8a48, 0x0030_8900)?;
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c0));
        let no_list = c.g[V1][0] == 0;
        c.sw(p(r32(c, GP), 0xce8), 0);
        let mut skip_one = false;
        if !no_list {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3cc));
            let single = c.g[V1][0] == c.g[V0][0];
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2bc));
            if single {
                skip_one = true;
            } else {
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10c));
                loop {
                    c.cycles += 7;
                    let more = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10c));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0030_8920);
                    }
                }
            }
        }
        if !skip_one {
            c.cycles += 1;
        }
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2bc));
        c.cycles += 11;
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0030_b068, 0x0030_8948)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b8));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0030_b068, 0x0030_8954)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b4));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0030_b068, 0x0030_8960)?;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0030_8974)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptProcess_NextTask(self)` (0x307e10): advance the script
/// process to its next instruction, calling the current and next elements'
/// handlers; a finished process (state 3) is left alone.
pub fn wscript_process_next_task(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[V1][0] = 0x3;
        c.g[V0][0] = Addr(this as u32).lw(c);
        let finished = c.g[V0][0] == c.g[V1][0];
        c.g[S1][0] = 0x44;
        let mut skip_one = false;
        if finished {
        } else {
            c.cycles += 15;
            c.g[A0][0] = Addr(p(this as u32, 8)).lw(c);
            c.g[V1][0] = Addr(p(this as u32, 0x14)).lw(c);
            c.mult(0, 4, 4, 17);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0x40)).lw(c);
            c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x30)).lh(c);
            c.g[A1][0] = Addr(p(c.g[V0][0] as u32, 0x34)).lw(c);
            let target = c.g[A1][0] as u32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            call_ptr(c, target, 0x0030_7e58)?;
            c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 8)).lw(c);
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 4)).lw(c);
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let at_end = c.g[V0][0] == 0;
            c.mult(0, 3, 5, 17);
            if at_end {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x5c)).lw(c);
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x10)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x14)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0030_7eac)?;
            } else {
                c.cycles += 10;
                c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x14)).lw(c);
                c.sw(p(c.g[S0][0] as u32, 8), c.g[A1][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0x40)).lw(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x20)).lh(c);
                c.g[A1][0] = Addr(p(c.g[V0][0] as u32, 0x24)).lw(c);
                let target = c.g[A1][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                call_ptr(c, target, 0x0030_7e90)?;
                skip_one = true;
            }
        }
        if !skip_one {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptTask_Init(self, owner, def)` (0x308278): look the task's
/// script functions up (three in the first table, ten in the second) and
/// store the process-relative pointers.
pub fn wscript_task_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S2][0] = this;
        c.g[S1][0] = sx((this as u32).wrapping_add(8));
        c.g[S3][0] = 0x2;
        c.g[S4][0] = c.g[A2][0];
        c.sw(p(this as u32, 0x3c), c.g[A1][0]);
        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(4));
        c.g[V0][0] = Addr(c.g[S4][0] as u32).lw(c);
        c.sw(p(this as u32, 4), c.g[V0][0]);
        loop {
            c.cycles += 8;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
            c.g[A0][0] = Addr(p(c.g[S2][0] as u32, 0x3c)).lw(c);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            call_ptr(c, 0x0030_8018, 0x0030_82cc)?;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            let more = (c.g[S3][0] as i64) >= 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0030_82b8);
            }
        }
        c.cycles += 4;
        c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x14));
        c.g[S3][0] = 0x9;
        loop {
            c.cycles += 8;
            c.g[A1][0] = Addr(c.g[S1][0] as u32).lw(c);
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
            c.g[A0][0] = Addr(p(c.g[S2][0] as u32, 0x3c)).lw(c);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            call_ptr(c, 0x0030_8018, 0x0030_82fc)?;
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            let more = (c.g[S3][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0030_82e8);
            }
        }
        c.cycles += 8;
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

/// `void DebugMenu_FreeBackgrounds(self, int flags)` (0x17abe0): release the
/// debug menu's background objects, then the menu itself when `flags & 1`.
pub fn debug_menu_free_backgrounds(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        c.g[V0][0] = Addr(this as u32).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[S2][0] = c.g[A1][0];
        if !empty {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(this as u32, 4)).lw(c);
            let none = (c.g[V0][0] as i64) <= 0;
            c.g[S0][0] = 0;
            if !none {
                c.cycles += 2;
                c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
                loop {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
                    let removed = (c.g[A1][0] as i64) < 0;
                    c.g[A2][0] = Addr(p(r32(c, GP), 0xffff_f7ac)).lw(c);
                    if !removed {
                        c.cycles += 5;
                        c.g[V0][0] = Addr(p(c.g[A2][0] as u32, 0x10d8)).lw(c);
                        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x190)).lh(c);
                        c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x194)).lw(c);
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0017_ac40)?;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 4)).lw(c);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0017_ac18);
                    }
                }
            }
            c.cycles += 3;
            c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
            let no_array = c.g[A0][0] == 0;
            c.g[V0][0] = c.g[S2][0] & 0x1;
            if !no_array {
                c.cycles += 2;
                call_ptr(c, 0x0031_7e98, 0x0017_ac68)?;
                c.cycles += 1;
                c.g[V0][0] = c.g[S2][0] & 0x1;
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0] & 0x1;
        }
        c.cycles += 2;
        let keep = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x30);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0017_ac7c)?;
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

/// `void cFEStateRewardGallery_InitPages(self)` (0x1d0180): set the
/// "has next / has previous" bits of the reward gallery from the loaded
/// rewards and compute the page count.
pub fn cfe_reward_gallery_init_pages(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        call_ptr(c, 0x0014_dc80, 0x001d_0198)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x0014_dd58, 0x001d_01a4)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0015_6a38, 0x001d_01b0)?;
        let ready = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x48)).lw(c);
        if !ready {
            c.cycles += 8;
            c.g[V1][0] = 0xffff_fffd;
            c.g[A0][0] = c.g[S1][0];
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x1;
            c.sw(p(c.g[S0][0] as u32, 0x48), c.g[V0][0]);
            call_ptr(c, 0x0015_6a10, 0x001d_01d0)?;
        } else {
        c.cycles += 13;
        c.g[V1][0] = 0xffff_fffe;
        c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 8)).lw(c);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | 0x2;
        c.sw(p(c.g[S0][0] as u32, 0x48), c.g[V0][0]);
        c.g[A0][0] = Addr(p(c.g[A1][0] as u32, 0xe0)).lh(c);
        c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 0xe4)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_01fc)?;
        c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x5c)).lw(c);
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x68)).lw(c);
        c.cycles += 6;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.div(0, 2, 3);
        c.g[V0][0] = c.lo[0];
        c.sw(p(c.g[S0][0] as u32, 0x60), c.g[V0][0]);
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void LobbyMan_StartQuickPlay(self, request, int mode)` (0x25ee20): when
/// the lobby manager is idle, build the quick-play request tags and send it.
pub fn lobby_man_start_quick_play(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0x120);
        frame.save(c, S1, 0x110);
        c.g[V1][0] = c.g[A2][0];
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0x100);
        c.g[V0][0] = Addr(this as u32).lw(c);
        let idle = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        let mut tail = 1;
        if !idle {
            c.cycles += 3;
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            if c.g[V0][0] != 0 {
                tail = 0;
            } else {
                c.cycles += 28;
                c.sw(p(c.g[S0][0] as u32, 0xa4), c.g[V1][0]);
                c.g[A2][0] = 0x4a0000;
                c.g[A3][0] = 0x480000;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3200));
                c.g[A0][0] = c.g[SP][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x8d0));
                c.sb(c.g[SP][0] as u32, 0);
                c.g[A1][0] = 0x100;
                call_ptr(c, 0x003e_c180, 0x0025_ee74)?;
                c.g[A2][0] = 0x4a0000;
                c.g[T1][0] = 0x4a0000;
                c.g[T0][0] = 0x14;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3208));
                c.g[A3][0] = c.g[S1][0];
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = 0x100;
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2f30));
                call_ptr(c, 0x003e_c3c0, 0x0025_ee98)?;
                c.g[T0][0] = 0x270000;
                c.g[A2][0] = 0x71750000;
                c.g[AT][0] = 0x41a00000;
                c.mtc1(1, 12);
                c.g[A0][0] = c.g[S0][0];
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7188));
                c.g[A1][0] = 0x12;
                c.g[A2][0] = c.g[A2][0] | 0x696b;
                c.g[A3][0] = c.g[SP][0];
                call_ptr(c, 0x0025_b608, 0x0025_eec0)?;
            }
        }
        c.cycles += tail;
        frame.restore(c, S0, 0x120);
        c.cycles += 4;
        frame.restore(c, S1, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `int BuddyMan_CountMessagesFromAt(BuddyMan* self, int index)` (0x262950):
/// count the roster's messages whose sender address equals the roster entry
/// at `index`.
pub fn buddy_man_count_messages_from_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        call_ptr(c, 0x0026_25c8, 0x0026_2970)?;
        c.g[S3][0] = c.g[V0][0];
        let no_entry = c.g[S3][0] == 0;
        c.g[V0][0] = 0xffff_ffff;
        if no_entry {
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
            frame.restore(c, S0, 0x40);
            c.cycles += 7;
            frame.restore(c, S1, 0x30);
            frame.restore(c, S2, 0x20);
            frame.restore(c, S3, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 7;
        c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0xb0)).lw(c);
        c.g[S2][0] = 0;
        c.g[V1][0] = Addr(p(c.g[S1][0] as u32, 0xac)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        let none = c.g[V0][0] == 0;
        c.g[S0][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            loop {
                c.cycles += 4;
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x0026_3630, 0x0026_29a8)?;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0xb0)).lw(c);
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[A1][0] = c.g[S3][0];
                    call_ptr(c, 0x0041_65a8, 0x0026_29bc)?;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[V1][0];
                    }
                    c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0xb0)).lw(c);
                }
                c.cycles += 7;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = Addr(p(c.g[S1][0] as u32, 0xac)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S1][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_29a0);
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x40);
        c.cycles += 7;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_StopObjectLoop(self, objectId, int context)`
/// (0x297eb8): find the loop slot for (object, context), stop its bank
/// handle and clear the slot.
pub fn ssx_audio_system_stop_object_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S0, 0x60);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        c.g[S0][0] = this;
        c.g[S2][0] = sx((this as u32).wrapping_add(0x5a0));
        c.g[S3][0] = c.g[A2][0];
        c.g[S4][0] = c.g[A1][0];
        c.g[S5][0] = 0x1;
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x5a00)).lw(c);
        loop {
            c.cycles += 2;
            let found = if c.g[V0][0] != c.g[S5][0] {
                false
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x5a04)).lw(c);
                if c.g[V0][0] != c.g[S4][0] {
                    false
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x5a20)).lw(c);
                    if c.g[V0][0] != c.g[S3][0] {
                        false
                    } else {
                        c.cycles += 9;
                        c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0x118)).lw(c);
                        c.g[A2][0] = 0x1;
                        c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x5a24)).lw(c);
                        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                        c.mtc1(0, 12);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                        call_ptr(c, 0x002a_d5f0, 0x0029_7f34)?;
                        c.sw(p(c.g[S0][0] as u32, 0x5a00), 0);
                        true
                    }
                }
            };
            let _ = found;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x5a00)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0029_7ef8);
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

/// `int MessageCenter_PickVariant(MessageCenter* self)` (0x1e3100): pick a
/// random variant index for a message, with type-specific handling (type 8
/// picks a variant directly, types 3/4/5 pick a random message id).
pub fn message_center_pick_variant(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[S1][0] = 0xff;
        c.g[A0][0] = 0x3;
        c.g[A1][0] = this;
        call_ptr(c, 0x001e_2d58, 0x001e_3124)?;
        let is3 = c.g[V0][0] != 0;
        c.g[A0][0] = 0x5;
        // Types 3 and 4 pick a random message id (the 0xa variant), type 5
        // uses the 0xb variant, type 8 a random variant index.
        let mut rand3 = is3;
        let mut rand5 = false;
        if !is3 {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x001e_2d58, 0x001e_3134)?;
            let not5 = c.g[V0][0] == 0;
            c.g[A0][0] = 0x8;
            if !not5 {
                c.cycles += 4;
                call_ptr(c, 0x0031_77f0, 0x001e_3144)?;
                c.g[V1][0] = 0xb;
                rand5 = true;
            } else {
                c.cycles += 4;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x001e_2d58, 0x001e_3154)?;
                let not8 = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[S0][0];
                if !not8 {
                    c.cycles += 4;
                    call_ptr(c, 0x0031_77f0, 0x001e_3164)?;
                    c.g[S1][0] = c.g[V0][0] & 0x3;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = 0x4;
                    call_ptr(c, 0x001e_2d58, 0x001e_3174)?;
                    let not4 = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[S1][0];
                    rand3 = !not4;
                }
            }
        }
        if rand3 {
            c.cycles += 3;
            call_ptr(c, 0x0031_77f0, 0x001e_317c)?;
            c.g[V1][0] = 0xa;
        }
        if rand3 || rand5 {
            c.cycles += 3;
            c.divu(0, 2, 3);
            c.cycles += 2;
            c.g[A0][0] = c.hi[0];
            c.g[S1][0] = c.g[A0][0] & 0xff;
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        c.cycles += 5;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIScroll_Render(self)` (0x39d968): render the scroll thumb's
/// decoration (f12/f13 carry the caller's screen-space offsets).
pub fn cui_scroll_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.swc1(21, p(c.g[SP][0] as u32, 0x28));
        c.swc1(20, p(c.g[SP][0] as u32, 0x20));
        c.g[A1][0] = Addr(p(this as u32, 0x74)).lw(c);
        let no_thumb = c.g[A1][0] == 0;
        if !no_thumb {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(this as u32, 0x78)).lw(c);
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
                c.cycles += 5;
                frame.restore_ra(c, 0);
                frame.restore_fpr(c, 21, 0x28);
                frame.restore_fpr(c, 20, 0x20);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 8;
            c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 0x14)).lw(c);
            c.lwc1(0, p(this as u32, 0x44));
            c.lwc1(1, p(this as u32, 0x48));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
            c.add_s(21, 12, 0);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            let skip = c.g[V0][0] == 0;
            c.add_s(20, 13, 1);
            if !skip {
                c.cycles += 7;
                c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 8)).lw(c);
                c.mov_s(12, 21);
                c.mov_s(13, 20);
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x88)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x8c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0039_d9d4)?;
            }
            c.cycles += 6;
            c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x78)).lw(c);
            c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 0x14)).lw(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            let skip2 = c.g[V0][0] == 0;
            c.mov_s(12, 21);
            if !skip2 {
                c.cycles += 6;
                c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 8)).lw(c);
                c.mov_s(13, 20);
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x88)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x8c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0039_da04)?;
            }
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 5;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void UIListBox_ctor(self)` (0x399768): construct a list box: 32 empty
/// rows, then the scroll-bar defaults.
pub fn ui_list_box_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        call_ptr(c, 0x0039_fb30, 0x0039_977c)?;
        c.g[V0][0] = Addr(p(this as u32, 0x74)).lbu(c);
        c.g[V1][0] = 0x490000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4798));
        c.g[A1][0] = 0xffff_ffff;
        c.g[V0][0] = c.g[V0][0] | 0x4;
        c.sw(p(this as u32, 8), c.g[V1][0]);
        c.g[V0][0] = c.g[V0][0] & 0xc4;
        c.g[A0][0] = sx((this as u32).wrapping_add(0x98));
        c.sb(p(this as u32, 0x74), c.g[V0][0]);
        c.g[V0][0] = 0x1f;
        loop {
            c.cycles += 8;
            c.sw(c.g[A0][0] as u32, 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            c.sw(p(c.g[A0][0] as u32, 4), 0);
            c.sw(p(c.g[A0][0] as u32, 8), 0);
            c.sw(p(c.g[A0][0] as u32, 0xc), 0);
            c.sb(p(c.g[A0][0] as u32, 0x10), 0);
            let more = c.g[V0][0] != c.g[A1][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0039_97a8);
            }
        }
        c.cycles += 21;
        c.sb(p(this as u32, 0x318), 0);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x98));
        c.sb(p(this as u32, 0x319), 0);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x280;
        call_ptr(c, 0x0041_6210, 0x0039_97e0)?;
        c.g[V1][0] = 0x4c0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66c8));
        c.ldl(4, p(c.g[A3][0] as u32, 7));
        c.ldr(4, c.g[A3][0] as u32);
        c.ldl(5, p(c.g[A3][0] as u32, 0xf));
        c.ldr(5, p(c.g[A3][0] as u32, 8));
        c.sdl(4, p(c.g[S0][0] as u32, 0x8f));
        c.sdr(4, p(c.g[S0][0] as u32, 0x88));
        c.sdl(5, p(c.g[S0][0] as u32, 0x97));
        c.sdr(5, p(c.g[S0][0] as u32, 0x90));
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_UpdateStatBar(self)` (0x19bc90): animate the
/// stat-bar fill (0..1 normalised by the global max) and hand the value to
/// the bar widget.
pub fn cfe_state_char_equip_update_stat_bar(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.swc1(20, p(c.g[SP][0] as u32, 0x30));
        c.g[A2][0] = Addr(p(this as u32, 0x70)).lw(c);
        let no_bar = c.g[A2][0] == 0;
        c.g[V1][0] = Addr(p(r32(c, GP), 0xffff_e7e8)).lw(c);
        if !no_bar {
            c.cycles += 13;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0xc4)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5dc));
            c.mtc1(0, 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5dc));
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.mtc1(3, 0);
            c.cvt_s_w(0, 0);
            c.div_s(20, 1, 0);
            c.c_lt_s(20, 2);
            if c.fcc() {
                c.mov_s(20, 2);
            }
            c.cycles += 19;
            c.g[V0][0] = Addr(p(c.g[A2][0] as u32, 8)).lw(c);
            c.g[A1][0] = 0x1;
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x48)).lh(c);
            c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_bcfc)?;
            c.neg_s(1, 20);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x70)).lw(c);
            c.swc1(0, p(c.g[SP][0] as u32, 8));
            c.swc1(1, p(c.g[SP][0] as u32, 4));
            c.swc1(0, c.g[SP][0] as u32);
            c.ldl(3, p(c.g[SP][0] as u32, 7));
            c.ldr(3, c.g[SP][0] as u32);
            c.g[A0][0] = Addr(p(c.g[SP][0] as u32, 8)).lw(c);
            c.sdl(3, p(c.g[V0][0] as u32, 0x57));
            c.sdr(3, p(c.g[V0][0] as u32, 0x50));
            c.sw(p(c.g[V0][0] as u32, 0x58), c.g[A0][0]);
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 5;
        frame.restore_ra(c, 0x10);
        c.lwc1(20, p(c.g[SP][0] as u32, 0x30));
        frame.close(c);
        Ok(())
    })
}


/// `int InputParser_LookupModifier(const char* name, void* parser)` (0x3217d0):
/// map a modifier name to its index (0..3), the "any" modifier (4), a
/// fallback (6) or "no modifier" (0xff... never: 0 for the first name).
pub fn input_parser_lookup_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let name = c.g[A1][0];
        c.g[S0][0] = name;
        c.g[A0][0] = name;
        c.g[A1][0] = 0x4a3ef8;
        call_ptr(c, 0x0041_aa88, 0x0032_17f0)?;
        let differs = c.g[V0][0] != 0;
        if !differs {
            c.cycles += 2;
            c.g[V0][0] = 0;
            frame.restore(c, S0, 0x10);
            c.cycles += 4;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = name;
        c.g[A1][0] = 0x4a3f00;
        call_ptr(c, 0x0041_aa88, 0x0032_180c)?;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 1;
            frame.restore(c, S0, 0x10);
            c.cycles += 4;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = name;
        c.g[A1][0] = 0x4a3f08;
        call_ptr(c, 0x0041_aa88, 0x0032_1828)?;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 2;
            frame.restore(c, S0, 0x10);
            c.cycles += 4;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = name;
        c.g[A1][0] = 0x48ddd8;
        call_ptr(c, 0x0041_aa88, 0x0032_1844)?;
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = 3;
            frame.restore(c, S0, 0x10);
            c.cycles += 4;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 8;
        c.g[A0][0] = name;
        c.g[A1][0] = 0x4a3f10;
        call_ptr(c, 0x0041_aa88, 0x0032_1858)?;
        c.g[A0][0] = 0x4;
        c.g[V1][0] = 0x6;
        if c.g[V0][0] == 0 {
            c.g[V1][0] = c.g[A0][0];
        }
        c.g[V0][0] = c.g[V1][0];
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BENewPlayerInt_IsMessageFlagSet(self, int index, int bit)` (0x147980):
/// test one bit of the per-profile/per-character message-flag bitmap.
pub fn benew_player_int_is_message_flag_set(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        let index = c.g[A1][0];
        c.g[S1][0] = index;
        c.g[S2][0] = c.g[A2][0];
        c.g[A0][0] = index;
        call_ptr(c, 0x0014_a0e0, 0x0014_79a4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_7798, 0x0014_79b0)?;
        c.g[V1][0] = 0x1c;
        c.g[A1][0] = 0x530000;
        c.mult(0, 2, 2, 3);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4fe0));
        c.g[A0][0] = 0x9b50;
        c.g[A2][0] = 0xf88;
        c.mult(0, 16, 16, 4);
        c.g[V1][0] = 0xffff_ffff;
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[S2][0] as i64)) as u64;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1f));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        if c.g[V1][0] != 0 {
            c.g[A0][0] = c.g[S2][0];
        }
        c.g[A1][0] = Addr(p(c.g[V0][0] as u32, 0x11)).lb(c);
        c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 5) as u32);
        c.g[V0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        c.mult(0, 5, 5, 6);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
        c.g[V1][0] = 0x1;
        frame.restore(c, S1, 0x20);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << ((c.g[S2][0] as u32) & 31));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
        frame.restore(c, S2, 0x10);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0xf08)).lw(c);
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ret;
        Ok(())
    })
}

/// The shared body of [`bolt_file_index_manager_set_model_slot`] and
/// [`bolt_file_index_manager_set_texture_slot`]: set (`value >= 0`) or clear
/// a per-node slot bit, dropping the node when it has no more references.
fn bolt_set_slot(c: &mut Cpu, entry: u32, flags_off: u32, value_off: u32, ref_off: u32) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S1, 0x20);
    frame.save(c, S0, 0x30);
    frame.save(c, S2, 0x10);
    let manager = c.g[A0][0];
    c.g[S2][0] = manager;
    let bit = c.g[A1][0];
    c.g[S0][0] = bit;
    c.g[S1][0] = c.g[A3][0];
    c.g[A3][0] = !(0u64 | c.g[S1][0]);
    frame.save_ra(c, 0);
    c.g[A3][0] = (0u64 < c.g[A3][0]) as u64;
    call_ptr(c, 0x0019_c938, entry + 0x2c)?;
    c.g[A1][0] = c.g[V0][0];
    let mut skip_one = c.g[A1][0] == 0;
    if !skip_one {
        c.cycles += 2;
        let store = (c.g[S1][0] as i64) >= 0;
        c.g[V1][0] = Addr(p(c.g[A1][0] as u32, flags_off)).lhu(c);
        if !store {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(c.g[A1][0] as u32, flags_off)).lh(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[S0][0] as u32) & 31)) as u32);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            let clear = c.g[V0][0] == 0;
            c.g[V0][0] = 0x1;
            if !clear {
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[S0][0] as u32) & 31));
                c.g[V0][0] = !(0u64 | c.g[V0][0]);
                c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
                c.g[V1][0] = sx((c.g[V0][0] as u32) << 16);
                let other = c.g[V1][0] != 0;
                c.sh(p(c.g[A1][0] as u32, flags_off), c.g[V0][0]);
                if !other {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(c.g[A1][0] as u32, ref_off)).lh(c);
                    let referenced = (c.g[V0][0] as i64) >= 0;
                    if !referenced {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0019_ca70, entry + 0x24)?;
                        skip_one = true;
                    }
                }
            }
        } else {
            c.cycles += 5;
            c.g[V0][0] = 0x1;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[S0][0] as u32) & 31));
            c.sh(p(c.g[A1][0] as u32, value_off), c.g[S1][0]);
            c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
            c.sh(p(c.g[A1][0] as u32, flags_off), c.g[V1][0]);
        }
    }
    if !skip_one {
        c.cycles += 1;
    }
    frame.restore(c, S0, 0x30);
    c.cycles += 5;
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void BoltFileIndexManager_SetModelSlot(self, int bit, int a2, int value)`
/// (0x19cc30): set or clear a model-slot bit on the node for (bit, a2).
pub fn bolt_file_index_manager_set_model_slot(c: &mut Cpu) {
    guest(c, |c| bolt_set_slot(c, 0x0019_cc30, 6, 0xa, 8))
}

/// `void BoltFileIndexManager_SetTextureSlot(self, int bit, int a2, int value)`
/// (0x19cd38): the same for texture slots.
pub fn bolt_file_index_manager_set_texture_slot(c: &mut Cpu) {
    guest(c, |c| bolt_set_slot(c, 0x0019_cd38, 4, 8, 0xa))
}

/// `void WScriptMan_CreateLocalTables(self)` (0x308aa0): recreate the script
/// manager's local tables from the current process list.
pub fn wscript_man_create_local_tables(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        c.g[S0][0] = 0;
        call_ptr(c, 0x0030_8a48, 0x0030_8ac8)?;
        c.g[V0][0] = Addr(p(this as u32, 0x28c)).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x003a_9800, 0x0030_8ad8)?;
        c.g[S2][0] = c.g[V0][0];
        let none = (c.g[S2][0] as i64) <= 0;
        c.g[S4][0] = 0x1;
        if !none {
            c.cycles += 1;
            c.g[S3][0] = 0;
            loop {
                c.cycles += 8;
                c.g[A0][0] = Addr(p(c.g[S1][0] as u32, 0x28c)).lw(c);
                c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 4)).lw(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 4)).lw(c);
                c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lh(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
                let skip = c.g[V0][0] != 0;
                c.g[A1][0] = 0;
                if !skip {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 8)).lw(c);
                    c.g[A1][0] = c.g[S4][0];
                    if c.g[V0][0] == 0 {
                        c.g[A1][0] = 0;
                    }
                }
                c.cycles += 2;
                let have = c.g[A1][0] != 0;
                c.g[A0][0] = c.g[S1][0];
                if have {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0030_8b78, 0x0030_8b24)?;
                }
                c.cycles += 4;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_8ae8);
                }
            }
        }
        c.cycles += 8;
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

/// `int WorldTriggerManager_CountActiveOthers(self, int kind)` (0x2b6630):
/// count active triggers of the same kind whose name is not the excluded one.
pub fn world_trigger_manager_count_active_others(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = 0;
        c.g[S1][0] = 0;
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = 0x27;
        c.g[S4][0] = c.g[A1][0];
        c.g[S5][0] = 0x4a0000;
        c.g[S6][0] = 0x1;
        loop {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
            if c.g[V1][0] != c.g[S6][0] {
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
            } else {
                c.cycles += 5;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x35a8));
                call_ptr(c, 0x0041_65a8, 0x002b_6690)?;
                let same_name = c.g[V0][0] == 0;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                if !same_name {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0xc)).lw(c);
                    c.g[V1][0] = c.g[V1][0] ^ c.g[S4][0];
                    if c.g[V1][0] != 0 {
                        c.g[S1][0] = c.g[A0][0];
                    }
                }
                c.cycles += 1;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
            }
            c.cycles += 2;
            let more = (c.g[S3][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x70));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002b_6670);
            }
        }
        c.cycles += 11;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptCamController_Update(self)` (0x169368): blend every script
/// camera's weight towards 1.0 and update its algorithm.
pub fn script_cam_controller_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.swc1(21, p(c.g[SP][0] as u32, 0x28));
        c.swc1(20, p(c.g[SP][0] as u32, 0x20));
        c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 8)).lw(c);
        c.g[S0][0] = Addr(c.g[V0][0] as u32).lw(c);
        if c.g[S0][0] != 0 {
            c.cycles += 5;
            c.g[AT][0] = 0x4040_0000;
            c.mtc1(1, 21);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 20);
            loop {
                c.cycles += 12;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                call_ptr(c, 0x0016_94b0, 0x0016_93a8)?;
                c.lwc1(2, p(c.g[V0][0] as u32, 0x18));
                c.mul_s(1, 2, 2);
                c.add_s(0, 1, 1);
                c.mul_s(1, 1, 21);
                c.mul_s(0, 0, 2);
                c.sub_s(1, 1, 0);
                c.c_lt_s(20, 1);
                let above = !c.fcc();
                c.swc1(2, p(c.g[S0][0] as u32, 4));
                if above {
                    c.cycles += 2;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 1);
                }
                c.cycles += 10;
                c.swc1(1, p(c.g[S0][0] as u32, 0xc));
                c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 0x10)).lw(c);
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x28)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x2c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0016_93f4)?;
                c.g[S0][0] = Addr(p(c.g[S0][0] as u32, 0x14)).lw(c);
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0016_93a0);
                }
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 5;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void* cFEPopupConfirm_ctor(self, engine, owner, text, a4..a8)` (0x1d99c8):
/// construct a confirmation popup, choosing the button count from the global
/// popup mode.
pub fn cfe_popup_confirm_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[S1][0] = c.g[A3][0];
        c.g[S2][0] = c.g[T0][0];
        c.g[S3][0] = c.g[T1][0];
        c.g[S4][0] = c.g[T2][0];
        c.g[S5][0] = c.g[T3][0];
        call_ptr(c, 0x001c_58e8, 0x001d_9a04)?;
        c.g[V1][0] = 0x470000;
        c.g[V0][0] = Addr(p(r32(c, GP), 0xffff_ef4c)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3770));
        let already = c.g[V0][0] != 0;
        c.sw(p(c.g[S0][0] as u32, 8), c.g[V1][0]);
        if already {
            c.cycles += 4;
            c.g[V0][0] = 0x2;
            c.g[V1][0] = 0x4;
            c.sw(p(c.g[S0][0] as u32, 0x17c), c.g[V0][0]);
            c.sw(p(c.g[S0][0] as u32, 0x14c), c.g[V1][0]);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x1;
            c.sw(p(c.g[S0][0] as u32, 0x17c), c.g[V0][0]);
        }
        c.cycles += 18;
        c.g[T2][0] = Addr(p(c.g[SP][0] as u32, 0x70)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[T0][0] = c.g[S4][0];
        c.g[T1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_9a80, 0x001d_9a54)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateEventSelect_Init(self)` (0x186658): create and enter the
/// event-select screen.
pub fn cfe_state_event_select_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14b0));
        c.g[S0][0] = Addr(p(this as u32, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0018_667c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0018_6690)?;
        let missing = c.g[V0][0] == 0;
        c.sw(p(c.g[S1][0] as u32, 0x40), c.g[V0][0]);
        if !missing {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0018_66a8)?;
        }
        // (The generated code also carries a second, unreachable screen
        // creation block here: both branches of the creation above leave the
        // block, so it never runs.)
        c.cycles += 10;
        c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 0x40)).lw(c);
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x48));
        c.sw(p(r32(c, GP), 0xffff_f4a4), 0);
        call_ptr(c, 0x0020_0700, 0x0018_66f4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateLodge_Init(self)` (0x1f3790): create the lodge screen, play
/// its first frame, select the lodge bank and start its music.
pub fn cfe_state_lodge_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2318));
        c.g[S0][0] = Addr(p(this as u32, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001f_37b4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_37c8)?;
        let missing = c.g[V0][0] == 0;
        c.sw(p(c.g[S1][0] as u32, 0x40), c.g[V0][0]);
        if !missing {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_37e0)?;
        }
        c.cycles += 24;
        call_ptr(c, 0x0014_dc80, 0x001f_37e8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x001f_37f4)?;
        c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 0x44)).lb(c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_74c8, 0x001f_3800)?;
        c.sw(p(c.g[S1][0] as u32, 0x48), c.g[V0][0]);
        c.g[V0][0] = 0x440000;
        c.sw(p(c.g[V0][0] as u32, 0x1b14), 0);
        call_ptr(c, 0x0028_b180, 0x001f_3810)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0028_f140, 0x001f_381c)?;
        c.g[V0][0] = 0x12;
        c.sw(p(c.g[S1][0] as u32, 0x54), 0);
        c.sw(p(c.g[S1][0] as u32, 0x50), c.g[V0][0]);
        c.sw(p(c.g[S1][0] as u32, 0x4c), 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void SSXAudioSystem_PlayPeakTransitionMusic(self)` (0x28cd48): start the
/// peak transition music on the music channel and set its volume.
pub fn ssx_audio_system_play_peak_transition_music(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x60);
        c.g[A1][0] = 0x1;
        frame.save(c, S1, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0x4a0000;
        frame.save(c, S3, 0x20);
        let this = c.g[A0][0];
        c.g[S2][0] = this;
        c.g[S3][0] = sx((this as u32).wrapping_add(0x118));
        frame.save(c, S0, 0x50);
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3628));
        frame.save(c, S4, 0x10);
        c.g[S4][0] = Addr(p(this as u32, 0x5558)).lw(c);
        c.g[S0][0] = Addr(p(c.g[S4][0] as u32, 0x10)).lh(c);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        call_ptr(c, 0x0028_7968, 0x0028_cd8c)?;
        c.g[A3][0] = c.g[V0][0];
        c.g[T0][0] = 0x1;
        c.g[V0][0] = Addr(p(c.g[S4][0] as u32, 0x14)).lw(c);
        c.g[A2][0] = 0xc;
        c.g[A0][0] = c.g[S0][0];
        let target = c.g[V0][0] as u32;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, target, 0x0028_cda8)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0x191;
        call_ptr(c, 0x002b_49c0, 0x0028_cdb4)?;
        c.g[V1][0] = Addr(p(c.g[S2][0] as u32, 0x5558)).lw(c);
        c.g[A1][0] = 0x1;
        c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x20)).lh(c);
        c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x24)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0028_cdcc)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0x7f;
        call_ptr(c, 0x002b_3c28, 0x0028_cdd8)?;
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

/// `void cFEStateRequestLine_BuySong(self, int songIndex)` (0x1988d8): mark
/// the song bought (only once at least six songs are owned) and refresh the
/// store's owned/bought bitmaps.
pub fn cfe_request_line_buy_song(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.g[A0][0] = 0;
        c.g[A1][0] = Addr(p(this as u32, 0x160)).ld(c);
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.g[A1][0] >> (c.g[A0][0] & 63);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            c.g[V1][0] = ((c.g[A0][0] as i64) < 64) as u64;
            let more = c.g[V1][0] != 0;
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_8900);
            }
        }
        c.cycles += 27;
        c.g[S0][0] = ((c.g[S0][0] as i64) < 6) as u64;
        c.g[S0][0] = c.g[S0][0] ^ 0x1;
        call_ptr(c, 0x0014_dc80, 0x0019_8928)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x0014_dd58, 0x0019_8934)?;
        c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 0x44)).lb(c);
        c.g[T0][0] = c.g[S0][0];
        c.g[A2][0] = Addr(p(c.g[S1][0] as u32, 0x48)).lw(c);
        c.g[A3][0] = c.g[S2][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0015_8558, 0x0019_894c)?;
        c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 0x160)).ld(c);
        c.g[V1][0] = 0x1;
        c.g[A0][0] = Addr(p(c.g[S1][0] as u32, 0x158)).ld(c);
        c.g[V1][0] = c.g[V1][0] << (c.g[S2][0] & 63);
        c.g[A1][0] = c.g[A1][0] | c.g[V1][0];
        c.sw(p(c.g[S1][0] as u32, 0x168), c.g[V0][0]);
        c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
        c.sd(p(c.g[S1][0] as u32, 0x160), c.g[A1][0]);
        c.sd(p(c.g[S1][0] as u32, 0x158), c.g[A0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool cFEStateTitle_TransitionIn(self)` (0x1947f8): count the transition
/// in over a few frames, then fade in the title's two text widgets.
pub fn cfe_state_title_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[V0][0] = Addr(p(r32(c, GP), 0xffff_e6d0)).lw(c);
        let mut fade = c.g[V0][0] == 0;
        if !fade {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let still = c.g[V0][0] != 0;
            c.sw(p(r32(c, GP), 0xffff_e6d0), c.g[V0][0]);
            if still {
                // one more frame to wait
                c.cycles += 1;
                c.g[V0][0] = 0;
                frame.restore(c, S0, 0x10);
                c.cycles += 4;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            fade = true;
        }
        if fade {
            // create the screen and fade its title texts in
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2470));
            call_ptr(c, 0x0031_7670, 0x0019_4828)?;
            c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0019_4834)?;
            c.g[A2][0] = c.g[V0][0];
            let missing = c.g[A2][0] == 0;
            c.g[A1][0] = 0x1;
            if !missing {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(c.g[A2][0] as u32, 8)).lw(c);
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x38)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x3c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0019_4854)?;
            }
            c.cycles += 9;
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x17d0));
            call_ptr(c, 0x0031_7670, 0x0019_4860)?;
            c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0019_486c)?;
            c.g[A2][0] = c.g[V0][0];
            let missing = c.g[A2][0] == 0;
            c.g[A1][0] = 0x1;
            if !missing {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(c.g[A2][0] as u32, 8)).lw(c);
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x48)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x4c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0019_488c)?;
            }
            c.cycles += 2;
            c.g[V0][0] = 0x1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_StoreRaceResult(self, result, int a2, int a3)`
/// (0x2a4660): remember a race result in the audio system's result buffers.
pub fn ssx_audio_system_store_race_result(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = Addr(p(r32(c, GP), 0xffff_f7b8)).lw(c);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0xc0)).lw(c);
        c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0x98)).lw(c);
        let finished = c.g[V0][0] != 0;
        c.g[S2][0] = c.g[A2][0];
        let mut tail = false;
        if !finished {
            c.cycles += 5;
            c.g[V1][0] = Addr(p(c.g[A0][0] as u32, 0x70)).lw(c);
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x5804)).lw(c);
                tail = true;
            }
        }
        if !tail {
            c.cycles += 3;
            c.g[A2][0] = 0x1;
            let no_track = c.g[T0][0] == 0;
            c.sw(p(c.g[S0][0] as u32, 0x57f8), c.g[A2][0]);
            if !no_track {
                c.cycles += 4;
                c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
                c.g[V0][0] = 0x3;
                if c.g[V1][0] == c.g[V0][0] {
                    c.sw(p(c.g[S0][0] as u32, 0x57fc), c.g[A2][0]);
                }
            }
            c.cycles += 10;
            c.sw(p(c.g[S0][0] as u32, 0x5800), c.g[A1][0]);
            call_ptr(c, 0x0028_b1e8, 0x002a_46c8)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_dd58, 0x002a_46d4)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_4bc0, 0x002a_46dc)?;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.sw(p(c.g[S0][0] as u32, 0x5810), c.g[V1][0]);
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x5804)).lw(c);
        }
        c.cycles += 11;
        c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x5808)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S1][0] as u32)));
        c.sw(p(c.g[S0][0] as u32, 0x5804), c.g[V0][0]);
        c.sw(p(c.g[S0][0] as u32, 0x5808), c.g[V1][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = c.g[RA][0] as u32;
        Ok(())
    })
}

/// `void Rider_PlaceAtMarker(rider, marker, int a2)` (0x11de60): place the
/// rider at a marker transform, reset its matrix and state.
pub fn rider_place_at_marker(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 45;
        let frame = Frame::open(c, 0x40);
        let arg1 = c.g[A1][0];
        c.g[V0][0] = arg1;
        frame.save(c, S0, 0x30);
        c.g[A1][0] = c.g[A2][0];
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0x20);
        c.g[A0][0] = 0x4d0000;
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x33a0));
        call_ptr(c, 0x0026_b5e0, 0x0011_de88)?;
        c.lwc1(2, p(c.g[V0][0] as u32, 8));
        c.g[A1][0] = 0x1;
        c.lwc1(0, p(c.g[V0][0] as u32, 0xc));
        c.lwc1(1, p(c.g[V0][0] as u32, 0x10));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 3);
        c.swc1(2, c.g[SP][0] as u32);
        c.swc1(0, p(c.g[SP][0] as u32, 4));
        c.swc1(1, p(c.g[SP][0] as u32, 8));
        c.swc1(3, p(c.g[SP][0] as u32, 0xc));
        c.lwc1(2, p(c.g[V0][0] as u32, 0x1c));
        c.lwc1(0, p(c.g[V0][0] as u32, 0x14));
        c.lwc1(1, p(c.g[V0][0] as u32, 0x18));
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x790)).lw(c);
        c.swc1(0, p(c.g[SP][0] as u32, 0x10));
        c.swc1(1, p(c.g[SP][0] as u32, 0x14));
        c.swc1(2, p(c.g[SP][0] as u32, 0x18));
        c.swc1(3, p(c.g[SP][0] as u32, 0x1c));
        call_ptr(c, 0x0011_9368, 0x0011_ded4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0011_fec8, 0x0011_dee0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0011_fe78, 0x0011_deec)?;
        c.mtc1(0, 12);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A3][0] = 0x5;
        call_ptr(c, 0x0011_d660, 0x0011_df04)?;
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoGetSaveDirEntry(self)` (0x2c42b0): issue the
/// sceMcGetDir for the save directory and update the state machine.
pub fn cps2_mem_card_do_get_save_dir_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x78;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S2][0] = this;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xb9));
        c.sw(p(c.g[S2][0] as u32, 0x3c), 0);
        c.g[A1][0] = Addr(p(c.g[S2][0] as u32, 0x1c)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xf9));
        c.mult(0, 3, 5, 2);
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S2][0] as u32)));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1b0));
        call_ptr(c, 0x0041_66f4, 0x002c_42f0)?;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = 0x4a0000;
        c.g[A3][0] = c.g[A2][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3948));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0041_7828, 0x002c_4308)?;
        c.g[A0][0] = Addr(p(c.g[S2][0] as u32, 0xc)).lw(c);
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0x1;
        c.g[T1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x480));
        call_ptr(c, 0x0040_a688, 0x002c_4324)?;
        let ok = (c.g[V0][0] as i64) >= 0;
        c.sw(p(r32(c, GP), 0x850), c.g[V0][0]);
        if ok {
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0x23;
            call_ptr(c, 0x002c_48c0, 0x002c_434c)?;
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x002c_48c0, 0x002c_4338)?;
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

/// `void cFEStateNetMessenger_HighlightRow(self, int row)` (0x1a5fb8): colour
/// the 15 roster rows, highlighting `row`.
pub fn cfe_state_net_messenger_highlight_row(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S0, 0x70);
        frame.save_ra(c, 0x20);
        let this = c.g[A0][0];
        c.g[S3][0] = this;
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = 0;
        c.g[S4][0] = 0x460000;
        c.g[A2][0] = 0;
        loop {
            c.cycles += 13;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x15b0));
            c.g[S0][0] = c.g[S1][0] ^ c.g[S2][0];
            call_ptr(c, 0x0041_7828, 0x001a_5ff8)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x001a_6004)?;
            c.g[A0][0] = Addr(p(c.g[S3][0] as u32, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001a_6010)?;
            c.g[A2][0] = c.g[V0][0];
            let missing = c.g[A2][0] == 0;
            c.g[A1][0] = (c.g[S0][0] < 1) as u64;
            if !missing {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(c.g[A2][0] as u32, 8)).lw(c);
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x48)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x4c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x001a_6030)?;
            }
            c.cycles += 7;
            c.g[A2][0] = (c.g[S0][0] < 1) as u64;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, 0x001a_66a8, 0x001a_6040)?;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 15) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A2][0] = c.g[S1][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001a_5fe8);
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenProfileSavePDA_InitSlotLabels(self)` (0x212080): set the
/// seven save-slot labels ("Slot 1".."Slot 6" and the empty slot).
pub fn ov_screen_profile_save_pda_init_slot_labels(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = 0x1;
        frame.save(c, S2, 0x30);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0x470000;
        frame.save_ra(c, 0x10);
        c.g[S3][0] = 0x4a0000;
        c.g[A0][0] = c.g[SP][0];
        loop {
            c.cycles += 26;
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2138));
            call_ptr(c, 0x0041_7828, 0x0021_20bc)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x0021_20c4)?;
            c.g[A0][0] = Addr(p(c.g[S1][0] as u32, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0021_20d0)?;
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x0021_20dc)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1d68));
            call_ptr(c, 0x0041_7828, 0x0021_20ec)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x0021_20f4)?;
            c.g[A0][0] = Addr(p(c.g[S1][0] as u32, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0021_2100)?;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            c.sw(p(c.g[V0][0] as u32, 0x18), c.g[V1][0]);
            c.g[V0][0] = ((c.g[S0][0] as i64) < 7) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[SP][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0021_20b0);
            }
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


/// `void* MdfFile_CreateModel(self)` (0x314cb0): name the model after the
/// MDF file, allocate and copy its 0x60-byte header.
pub fn mdf_file_create_model(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = 0x60;
        frame.save(c, S0, 0x70);
        c.mult(0, 16, 5, 2);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        c.g[A1][0] = 0x490000;
        frame.save_ra(c, 0x40);
        let this = c.g[A0][0];
        c.g[S2][0] = this;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5a30));
        c.g[A2][0] = Addr(p(this as u32, 8)).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        call_ptr(c, 0x0041_7828, 0x0031_4ce8)?;
        c.g[V0][0] = Addr(p(c.g[S2][0] as u32, 8)).lw(c);
        c.g[A1][0] = c.g[SP][0];
        c.g[A3][0] = 0;
        c.g[A2][0] = 0x3000000;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x14)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
        call_ptr(c, 0x0031_7e30, 0x0031_4d08)?;
        c.g[A1][0] = Addr(p(c.g[S2][0] as u32, 8)).lw(c);
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = 0x60;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        call_ptr(c, 0x003e_6574, 0x0031_4d20)?;
        c.g[V0][0] = Addr(p(c.g[S2][0] as u32, 8)).lw(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x14)).lw(c);
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x60));
        if c.g[V1][0] != 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(p(c.g[S2][0] as u32, 0xc)).lw(c);
            c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x10)).lw(c);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, 0x003b_47f8, 0x0031_4d44)?;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void* NetFEState_CreateKeyboardPopup(self, a1, a2, a3, int a4)`
/// (0x1a91b0): allocate and construct the keyboard popup and push it on the
/// state stack.
pub fn net_fe_state_create_keyboard_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 45;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S4, 0x10);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        frame.save(c, S2, 0x30);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[S4][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A3][0];
        c.g[S2][0] = c.g[T0][0];
        c.g[A1][0] = 0x460000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2728));
        c.g[A0][0] = 0x444;
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x001a_91f8)?;
        c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x10)).lw(c);
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[T0][0] = 0xf;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x001c_b030, 0x001a_9210)?;
        c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        c.sw(p(c.g[S0][0] as u32, 0x69c), c.g[V0][0]);
        c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0xd0)).lw(c);
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x10)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0039_f290, 0x001a_922c)?;
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x69c)).lw(c);
        c.g[A1][0] = c.g[S2][0];
        c.sw(p(c.g[V0][0] as u32, 0x18), c.g[S3][0]);
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x69c)).lw(c);
        call_ptr(c, 0x001c_d088, 0x001a_9240)?;
        c.sw(p(c.g[S0][0] as u32, 0x6a4), c.g[S4][0]);
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

/// `void SSXAudioSystem_DJSay20CC(self, int context)` (0x2a24b0): have the DJ
/// say line 0x20cc in the given speech context, if DJ speech is on.
pub fn ssx_audio_system_dj_say_20cc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        call_ptr(c, 0x0029_f0d8, 0x002a_24d0)?;
        let off = c.g[V0][0] == 0;
        c.g[A0][0] = sx((this as u32).wrapping_add(0x5560));
        let mut tail = 1;
        if !off {
            c.cycles += 10;
            c.mtc1(0, 12);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x20cc;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0xa;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            call_ptr(c, 0x002b_1458, 0x002a_24f8)?;
            if c.g[V0][0] == 0 {
                tail = 0;
            } else {
                c.cycles += 19;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0029_f198, 0x002a_2508)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = 0xffff_ffff;
                call_ptr(c, 0x002a_1bd8, 0x002a_251c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = 0x1;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20cc;
                call_ptr(c, 0x003d_8008, 0x002a_2530)?;
                c.g[V1][0] = Addr(p(r32(c, GP), 0x173c)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = c.g[S0][0];
                c.g[A1][0] = 0x3;
                let target = c.g[V1][0] as u32;
                c.g[T0][0] = 0x3;
                call_ptr(c, target, 0x002a_254c)?;
            }
        }
        c.cycles += tail;
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int GMHandler7_v06(self)` (0x23c560): the "7" GM handler: dispatch an
/// input event to the active game (or its HUD) for the 17..21 range.
pub fn gm_handler7_v06(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        call_ptr(c, 0x0014_dc80, 0x0023_c578)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_c584)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0014_4be0, 0x0023_c590)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 17) as u64;
        c.g[V0][0] = c.g[V0][0] ^ 0x1;
        c.g[A0][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[A0][0] = ((c.g[V1][0] as i64) < 22) as u64;
        }
        c.cycles += 2;
        c.g[V0][0] = 0x530000;
        if c.g[A0][0] != 0 {
            c.cycles += 14;
            c.g[A2][0] = Addr(p(c.g[S0][0] as u32, 0xa)).lh(c);
            c.g[A1][0] = Addr(p(c.g[V0][0] as u32, 0x5c12)).lb(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A3][0] = Addr(p(c.g[S0][0] as u32, 8)).lhu(c);
            call_ptr(c, 0x0014_5668, 0x0023_c5c4)?;
            c.g[A0][0] = Addr(p(r32(c, GP), 0xffff_f7b8)).lw(c);
            c.g[V1][0] = 0x64;
            c.mult(0, 6, 2, 3);
            c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0x84)).lw(c);
            c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0xc)).lw(c);
            c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x28)).lw(c);
            c.g[A0][0] = Addr(p(c.g[A0][0] as u32, 0x790)).lw(c);
            let none = c.g[A2][0] == 0;
            c.g[A1][0] = Addr(p(c.g[A0][0] as u32, 0x198)).lw(c);
            if !none {
                c.cycles += 2;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[A2][0] as u32)));
                call_ptr(c, 0x0011_95d8, 0x0023_c5f0)?;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 8)).lhu(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sh(p(c.g[S0][0] as u32, 8), c.g[V0][0]);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = 0x1;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FlexRail_RegisterSpaceNode(self, node)` (0x348b90): allocate a rail
/// entry for a space node and register it with the spatial hash.
pub fn flex_rail_register_space_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 45;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0x490000;
        frame.save(c, S0, 0x40);
        c.g[A2][0] = 0x2000_0000;
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17f8));
        c.g[A0][0] = 0x10;
        call_ptr(c, 0x0031_7d70, 0x0034_8bc0)?;
        c.g[V1][0] = 0x3;
        c.sw(p(c.g[V0][0] as u32, 0xc), c.g[S0][0]);
        c.sw(p(c.g[V0][0] as u32, 8), c.g[V1][0]);
        c.sw(p(c.g[S0][0] as u32, 0x54), c.g[V0][0]);
        call_ptr(c, 0x002d_1be0, 0x0034_8bd4)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
        c.g[S2][0] = Addr(p(c.g[S0][0] as u32, 0x54)).lw(c);
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x0032_8f28, 0x0034_8be8)?;
        c.g[V0][0] = Addr(p(c.g[SP][0] as u32, 4)).lw(c);
        c.g[A3][0] = 0xffff_ffff;
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x50));
        c.g[A1][0] = Addr(p(c.g[SP][0] as u32, 8)).lw(c);
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[A2][0] = Addr(p(c.g[SP][0] as u32, 0xc)).lw(c);
        if c.g[V0][0] != 0 {
            c.g[V1][0] = c.g[S1][0];
        }
        c.g[A1][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[A3][0] = ((c.g[A3][0] as i64) < (c.g[A2][0] as i64)) as u64;
        if c.g[A1][0] != 0 {
            c.g[A0][0] = c.g[V1][0];
        }
        c.g[A2][0] = c.g[S2][0];
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
        c.g[A1][0] = 0x2;
        if c.g[A3][0] == 0 {
            c.g[A0][0] = c.g[V0][0];
        }
        c.g[A3][0] = c.g[SP][0];
        call_ptr(c, 0x0032_8c20, 0x0034_8c2c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateProfileLoad_OnWidgetNotify(self, item, unsigned int msg)`
/// (0x18eeb8): map a profile-load widget notification to the state that
/// should handle it.
pub fn cfe_state_profile_load_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(p(r32(c, GP), 0xffff_f7b8)).lw(c);
        let msg = c.g[A2][0];
        c.g[S0][0] = msg;
        let mut result = 0;
        'outer: {
            'b58: {
                'b54: {
                    'b4c: {
                        'b38: {
                            call_ptr(c, 0x0022_7f80, 0x0018_eed8)?;
                            c.g[V1][0]=c.g[V0][0];
                            c.g[V0][0] = 8;
                            let is8 = c.g[S0][0] == 8;
                            c.g[V0][0] = (c.g[S0][0] < 9) as u64;
                            if is8 {
                                break 'b38;
                            }
                            'b14: {
                                'b00: {
                                    c.cycles += 2;
                                    let big = c.g[V0][0] == 0;
                                    c.g[V0][0] = 6;
                                    if big {
                                        break 'b00;
                                    }
                                    c.cycles += 2;
                                    let is6 = c.g[S0][0] == 6;
                                    c.g[V0][0] = 0x101;
                                    if is6 {
                                        break 'b14;
                                    }
                                    c.cycles += 2;
                                    result = c.g[V0][0];
                                    frame.restore(c, S0, 0x20);
                                    break 'outer;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 9;
                                let is9 = c.g[S0][0] == 9;
                                c.g[V0][0] = 0x101;
                                if is9 {
                                    break 'b54;
                                }
                                c.cycles += 2;
                                result = c.g[V0][0];
                                frame.restore(c, S0, 0x20);
                                break 'outer;
                            }
                            c.cycles += 9;
                            c.g[A2][0] = Addr(p(c.g[V1][0] as u32, 0x434)).lw(c);
                            c.g[A1][0] = Addr(p(c.g[V1][0] as u32, 0x428)).lw(c);
                            c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                            c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x1b0)).lh(c);
                            c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x1b4)).lw(c);
                            let target = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, target, 0x0018_ef30)?;
                            break 'b4c;
                        }
                        c.cycles += 4;
                        c.g[V1][0] = Addr(p(r32(c, GP), 0xffff_e4f8)).lw(c);
                        c.g[V0][0] = 2;
                        let two = c.g[V1][0] == 2;
                        c.g[V0][0] = 0x100;
                        if two {
                            break 'b58;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0x220)).lw(c);
                        break 'b4c;
                    }
                    c.cycles += 2;
                    let nonzero = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x101;
                    if nonzero {
                        break 'b58;
                    }
                    break 'b54;
                }
                c.cycles += 1;
                c.g[V0][0] = 0x100;
                break 'b58;
            }
            c.cycles += 1;
            result = c.g[V0][0];
            frame.restore(c, S0, 0x20);
        }
        c.g[V0][0] = result as u64;
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIFontSet_LoadFonts(self)` (0x398568): load each of the font set's
/// fonts and store the resulting font objects.
pub fn ui_font_set_load_fonts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S3][0] = this;
        c.g[V0][0] = Addr(p(this as u32, 0x34)).lb(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !none {
            c.cycles += 4;
            c.g[S5][0] = 0x4a0000;
            c.g[S4][0] = 0xc;
            c.g[S2][0] = sx((this as u32).wrapping_add(4));
            c.g[S0][0] = sx((c.g[S1][0] as u32) << 24);
            loop {
                c.cycles += 19;
                c.g[A0][0] = 0x84;
                c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4780));
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0039_85c0)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = Addr(p(c.g[S2][0] as u32, 4)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x0039_15e8, 0x0039_85d8)?;
                c.mult(0, 3, 16, 20);
                c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                c.sw(p(c.g[S0][0] as u32, 0xc), c.g[V0][0]);
                c.g[V0][0] = Addr(p(c.g[S3][0] as u32, 0x34)).lb(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S1][0] as u32) << 24);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_85a8);
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

/// `void SplineModifier_ctor_2(self, path)` (0x3595d8): construct a spline
/// modifier for a path and load its spline.
pub fn spline_modifier_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x490000;
        frame.save(c, S0, 0x30);
        c.g[V1][0] = 0xffff_ffff_ffff_0000;
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xdb0));
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        c.g[S1][0] = c.g[A1][0];
        c.sw(p(c.g[S0][0] as u32, 0xd8), c.g[V1][0]);
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.sw(p(c.g[S0][0] as u32, 0xe0), 0);
        c.g[A2][0] = 0x50;
        c.sw(p(c.g[S0][0] as u32, 0xdc), 0);
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xd8));
        c.sw(p(c.g[S0][0] as u32, 0xe4), 0);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x10)).lh(c);
        c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x14)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_9638)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0034_5570, 0x0035_9644)?;
        c.g[A1][0] = 0x500000;
        c.g[A2][0] = 0x1;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xee0));
        c.g[V0][0] = c.g[S0][0];
        c.lq(3, c.g[A1][0] as u32);
        frame.restore(c, S1, 0x20);
        c.sq(3, p(c.g[S0][0] as u32, 0xb0));
        frame.restore(c, S2, 0x10);
        c.lq(4, c.g[A1][0] as u32);
        c.sw(p(c.g[S0][0] as u32, 0x58), c.g[A2][0]);
        c.sq(4, p(c.g[S0][0] as u32, 0xc0));
        c.sw(p(c.g[S0][0] as u32, 0xd0), 0);
        c.sw(p(c.g[S0][0] as u32, 0xd4), 0);
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void PS2ExecMan_v03(self)` (0x31a920): shut down the PS2 exec manager's
/// interrupts, threads and semaphores.
pub fn ps2_exec_man_v03(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 45;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x3;
        call_ptr(c, 0x0042_4880, 0x0031_a93c)?;
        c.g[S0][0] = 0x10000;
        c.g[A0][0] = 0x3;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x4290)).lw(c);
        call_ptr(c, 0x0042_3aa0, 0x0031_a950)?;
        c.g[A0][0] = 0x3;
        call_ptr(c, 0x0042_48e8, 0x0031_a958)?;
        c.g[V1][0] = 0x1;
        c.sw(p(c.g[S1][0] as u32, 0x20), c.g[V1][0]);
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x4044)).lw(c);
        call_ptr(c, 0x0042_3dc0, 0x0031_a968)?;
        c.g[A0][0] = Addr(p(c.g[S1][0] as u32, 0x4034)).lw(c);
        call_ptr(c, 0x0042_3dc0, 0x0031_a970)?;
        c.g[A0][0] = Addr(p(c.g[S1][0] as u32, 0x4030)).lw(c);
        call_ptr(c, 0x0042_3bf0, 0x0031_a988)?;
        c.g[A0][0] = Addr(p(c.g[S1][0] as u32, 0x4030)).lw(c);
        call_ptr(c, 0x0042_3bb0, 0x0031_a990)?;
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x4040)).lw(c);
        call_ptr(c, 0x0042_3bf0, 0x0031_a998)?;
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x4040)).lw(c);
        call_ptr(c, 0x0042_3bb0, 0x0031_a9a0)?;
        c.g[A0][0] = Addr(p(c.g[S1][0] as u32, 0x4034)).lw(c);
        call_ptr(c, 0x0042_3db0, 0x0031_a9a8)?;
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x4044)).lw(c);
        call_ptr(c, 0x0042_3db0, 0x0031_a9b0)?;
        c.g[A0][0] = Addr(p(c.g[S1][0] as u32, 0x18)).lw(c);
        call_ptr(c, 0x0042_3db0, 0x0031_a9b8)?;
        c.g[A0][0] = Addr(p(c.g[S1][0] as u32, 0xc)).lw(c);
        call_ptr(c, 0x0042_3db0, 0x0031_a9c0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFlagManager_AddFlag(self, group, flag)` (0x34c548): add a flag to
/// the matching group, or to the first group with room.
pub fn cflag_manager_add_flag(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0034_c548 => {
                    c.cycles += 12;
                    let frame = Frame::open(c, 0x60);
                    frame.save(c, S4, 0x10);
                    frame.save(c, S0, 0x50);
                    frame.save(c, S1, 0x40);
                    frame.save(c, S2, 0x30);
                    frame.save(c, S3, 0x20);
                    frame.save_ra(c, 0);
                    let this = c.g[A0][0];
                    c.g[S4][0] = this;
                    c.g[S0][0] = sx((this as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A2][0];
                    c.g[S2][0] = 0;
                    c.g[S3][0] = c.g[A1][0];
                    pc = 0x0034_c578;
                }
                0x0034_c578 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x5c)).lw(c);
                    let empty = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    pc = if empty { 0x0034_c598 } else { 0x0034_c584 };
                }
                0x0034_c584 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x0034_afe8, 0x0034_c590)?;
                    let matched = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    pc = if matched { 0x0034_c5bc } else { 0x0034_c598 };
                }
                0x0034_c598 => {
                    c.cycles += 4;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 15) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x188));
                    if !more {
                        pc = 0x0034_c5a8;
                    } else {
                        if crate::events::back_edge_unwinds(c, 0x0034_c578) {
                            return Ok(());
                        }
                        pc = 0x0034_c578;
                    }
                }
                0x0034_c5a8 => {
                    c.cycles += 2;
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = 0;
                    pc = 0x0034_c5b0;
                }
                0x0034_c5b0 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0x5c)).lw(c);
                    let busy = c.g[V0][0] != 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    pc = if busy { 0x0034_c5d0 } else { 0x0034_c5bc };
                }
                0x0034_c5bc => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x0034_b038, 0x0034_c5c8)?;
                    c.lq(16, p(c.g[SP][0] as u32, 0x50));
                    pc = 0x0034_c5e0;
                }
                0x0034_c5d0 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 15) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x188));
                    if !more {
                        pc = 0x0034_c5dc;
                    } else {
                        if crate::events::back_edge_unwinds(c, 0x0034_c5b0) {
                            return Ok(());
                        }
                        pc = 0x0034_c5b0;
                    }
                }
                0x0034_c5dc => {
                    c.cycles += 1;
                    c.lq(16, p(c.g[SP][0] as u32, 0x50));
                    pc = 0x0034_c5e0;
                }
                0x0034_c5e0 => {
                    c.cycles += 7;
                    c.lq(17, p(c.g[SP][0] as u32, 0x40));
                    c.lq(18, p(c.g[SP][0] as u32, 0x30));
                    c.lq(19, p(c.g[SP][0] as u32, 0x20));
                    c.lq(20, p(c.g[SP][0] as u32, 0x10));
                    c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
                    let ret = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.pc = ret;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0034_c548, 0x0034_c5fc);
                    return Ok(());
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable, FAKE};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use ssx3_ee as ee;

    // Objects and buffers in the test data area (below the fixtures'
    // reserved blocks: BE at +0x30000, WORLD at +0x80000, the fake log at
    // 0x900000 and the pools above +0x400000).
    const OBJ: u32 = DATA;
    const OBJ2: u32 = DATA + 0x800;
    const OBJ3: u32 = DATA + 0x1000;
    const OBJ4: u32 = DATA + 0x1800;
    const TBL: u32 = DATA + 0x2000;
    const BE: u32 = DATA + 0x3_0000;
    const WORLD: u32 = DATA + 0x8_0000;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
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

    /// The engine's size-class pool allocator: `gp + 0xdd8` ready plus eight
    /// free nodes per size class.
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

    fn base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        globals(c);
        pool(c);
        r
    }

    // ---------------------------------------------------------------- front end

        #[test]
    fn cfe_online_main_menu_on_disconnect_popup_event_matches() {
        check(
            ee::s03::f_001c0240__generated,
            cfe_online_main_menu_on_disconnect_popup_event,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x001c_0240, 6);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = if seed % 2 == 0 { 0xf } else { (seed % 16) as u64 };
                w(c, OBJ + 0x10, OBJ3);
                w(c, OBJ + 0x40, OBJ4);
                w(c, OBJ4 + 0xd0, OBJ3);
                w(c, OBJ3 + 0x10, OBJ2);
                w(c, GAME_GP.wrapping_add(0x238), OBJ2);
                w(c, GAME_GP.wrapping_sub(0xc8), OBJ2);
            },
        );
    }

    /// Two arrow widgets whose `v09(enabled)` dispatches through a fake vtable
    /// slot at +0x48.
    fn arrows(c: &mut Cpu, seed: u32, salt: u64, count_off: u32, max_off: u32, up: u32, down: u32) {
        let mut r = base(c, seed, salt, 2);
        c.g[A0][0] = OBJ as u64;
        let count = if seed % 4 == 0 { 0 } else { (seed % 5) as u32 };
        w(c, OBJ + count_off, count);
        w(c, OBJ + max_off, if seed % 3 == 0 { 0x40 } else { count + 4 });
        w(c, OBJ + up, OBJ2);
        w(c, OBJ + down, OBJ3);
        let vt = vtable(c, &mut r, &[0x48]);
        w(c, OBJ2 + 8, vt);
        let vt = vtable(c, &mut r, &[0x48]);
        w(c, OBJ3 + 8, vt);
    }

            #[test]
    fn cfe_relogin_v33_matches() {
        check(
            ee::s04::f_001d5280__generated,
            cfe_relogin_v33,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x001d_5280, 6);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = match seed % 4 {
                    0 => 0xf,
                    1 => 0x14,
                    _ => (seed % 32) as u64,
                };
                w(c, OBJ + 0x10, OBJ3);
                w(c, OBJ3 + 0x18, OBJ4);
                w(c, OBJ3 + 0x40, OBJ4);
            },
        );
    }

    // -------------------------------------------------------------------- fonts

    /// A glyph table whose count/max fields let both the "outside the range"
    /// and the search paths run: `+0xc` count, `+0x10` max code, `+0` first
    /// glyph, `+4` glyph table pointer, `+8` width table pointer.
    fn font_fixture(c: &mut Cpu, seed: u32, salt: u64, wide: bool) {
        let _ = base(c, seed, salt, 0);
        c.g[A0][0] = OBJ as u64;
        let ch = if wide {
            (seed % 0x60) as u64
        } else {
            ((seed % 0x60) as i8 as i32) as u32 as u64
        };
        c.g[A1][0] = ch;
        w(c, OBJ + 0xc, if seed % 5 == 0 { 0 } else { 100 });
        w(c, OBJ + 0x10, if seed % 3 == 0 { 0x10 } else { 0x200 });
        w(c, OBJ, OBJ2);
        w(c, OBJ + 4, OBJ2);
        w(c, OBJ + 8, OBJ3);
        // A sorted little glyph table at OBJ2 (codes) and widths at OBJ3.
        for i in 0..0x20u32 {
            c.bus.write16(OBJ2 + i * 2, (i * 2) as u16);
            c.bus.write8(OBJ3 + i, (i as u8).wrapping_add(1));
        }
    }

    #[test]
    fn font_get_char_width_matches() {
        check(ee::s13::f_00392ba8__generated, font_get_char_width, 12, &|c, seed| {
            font_fixture(c, seed, 0x0039_2ba8, false)
        });
    }

    #[test]
    fn font_get_char_width_w_matches() {
        check(ee::s13::f_00392c60__generated, font_get_char_width_w, 12, &|c, seed| {
            font_fixture(c, seed, 0x0039_2c60, true)
        });
    }

    // ------------------------------------------------------------- localisation

    #[test]
    fn loc_string_table_find_matches() {
        check(ee::s08::f_002c6de0__generated, loc_string_table_find, 18, &|c, seed| {
            let _ = base(c, seed, 0x002c_6de0, 0);
            c.g[A0][0] = OBJ as u64;
            let count = if seed % 7 == 0 { 0 } else { 1 + (seed % 6) };
            w(c, OBJ + 0xc, count);
            w(c, OBJ + 8, count.saturating_sub(1));
            w(c, OBJ + 0x10, TBL);
            let mut keys = [0u32; 8];
            for i in 0..count {
                keys[i as usize] = 0x10 + i * 4;
                w(c, TBL + i * 8, keys[i as usize]);
                w(c, TBL + i * 8 + 4, 0x1000 + i);
            }
            let i = (seed as usize) % count.max(1) as usize;
            let delta = (seed % 5) as i64 - 2;
            c.g[A1][0] = (keys[i] as i64 + delta) as u32 as u64;
        });
    }
    // ------------------------------------------------ more front end / scripts

    #[test]
    fn cfe_state_char_equip_set_category_title_matches() {
        check(
            ee::s03::f_0019bbe0__generated,
            cfe_state_char_equip_set_category_title,
            9,
            &|c, seed| {
                let _ = base(c, seed, 0x0019_bbe0, 3);
                w(c, GAME_GP.wrapping_sub(0x1f20), 0x1234);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = if seed % 3 == 0 {
                    0x1234
                } else {
                    (0x1000 + seed * 7) as u64
                };
                w(c, OBJ + 0x58, if seed % 4 == 0 { 0 } else { OBJ2 });
                w(c, OBJ + 0xbc, (seed % 5) as u32);
                w(c, OBJ2 + 0x18, 0);
            },
        );
    }

    #[test]
    fn cov_state_overlay_init_matches() {
        check(ee::s05::f_0020a380__generated, cov_state_overlay_init, 6, &|c, seed| {
            let _ = base(c, seed, 0x0020_a380, 3);
            c.g[A0][0] = OBJ as u64;
            w(c, OBJ + 0x50, 0);
            w(c, OBJ + 0x10, OBJ2);
            let game = WORLD + 0x1000;
            w(c, game + 0x64, if seed % 3 == 0 { 0 } else { OBJ3 });
            w(c, OBJ3 + 0x10, match seed % 4 {
                0 => 1,
                1 => 3,
                _ => OBJ4,
            });
            w(c, OBJ2 + 0x18, 0);
        });
    }

    #[test]
    fn wscript_man_dtor_matches() {
        check(
            ee::s10::f_003088d8__generated,
            wscript_man_dtor,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x0030_88d8, 4);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed & 1) as u64;
                w(c, OBJ + 0x28c, OBJ2);
                w(c, OBJ + 0x2bc, 0);
                w(c, OBJ + 0x2b8, 0);
                w(c, OBJ + 0x2b4, 0);
                w(c, OBJ2 + 4, OBJ3);
            },
        );
    }

    #[test]
    fn wscript_process_next_task_matches() {
        check(
            ee::s10::f_00307e10__generated,
            wscript_process_next_task,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x0030_7e10, 3);
                c.g[A0][0] = OBJ as u64;
                w(c, OBJ, if seed % 4 == 0 { 3 } else { (seed % 3) as u32 });
                w(c, OBJ + 8, (seed % 4) as u32);
                w(c, OBJ + 4, 4);
                w(c, OBJ + 0x14, OBJ2);
                w(c, OBJ2 + 0x40, OBJ3);
                c.bus.write16(OBJ3 + 0x30, (seed as u16) & !3);
                w(c, OBJ3 + 0x34, FAKE);
                c.bus.write16(OBJ3 + 0x20, (seed as u16) & !3);
                w(c, OBJ3 + 0x24, FAKE + 0x10);
                w(c, OBJ + 0x5c, OBJ3);
                c.bus.write16(OBJ3 + 0x10, (seed as u16) & !3);
                w(c, OBJ3 + 0x14, FAKE + 0x20);
            },
        );
    }

    #[test]
    fn wscript_task_init_matches() {
        check(
            ee::s10::f_00308278__generated,
            wscript_task_init,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x0030_8278, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ3 as u64;
                c.g[A2][0] = OBJ2 as u64;
                w(c, OBJ2, (seed % 8) as u32);
                for i in 0..4 {
                    w(c, OBJ2 + 4 + 4 * i, 0x100 + i);
                }
                for i in 0..10 {
                    w(c, OBJ2 + 0x10 + 4 * i, 0x200 + i);
                }
                w(c, OBJ + 0x28c, OBJ3);
            },
        );
    }

    #[test]
    fn debug_menu_free_backgrounds_matches() {
        check(
            ee::s02::f_0017abe0__generated,
            debug_menu_free_backgrounds,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x0017_abe0, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed & 1) as u64;
                let count = if seed % 4 == 0 { 0 } else { 1 + (seed % 3) };
                w(c, OBJ, OBJ2);
                w(c, OBJ + 4, count);
                for i in 0..count {
                    w(c, OBJ2 + 4 * i, if seed % 3 == 0 && i == 0 { -1i32 as u32 } else { 1 });
                }
                w(c, GAME_GP.wrapping_sub(0x854), OBJ3);
                w(c, OBJ3 + 0x10d8, OBJ4);
                c.bus.write16(OBJ4 + 0x190, (seed as u16) & !3);
                w(c, OBJ4 + 0x194, FAKE);
            },
        );
    }

        // ------------------------------------------------------- lobbies and audio

    #[test]
    fn lobby_man_start_quick_play_matches() {
        check(
            ee::s06::f_0025ee20__generated,
            lobby_man_start_quick_play,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x0025_ee20, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = (seed % 3) as u64;
                w(c, OBJ, if seed % 4 == 0 { 1 } else { (seed % 3) as u32 });
                w(c, OBJ + 0xa4, 0);
                w(c, OBJ + 0xa0, 0);
            },
        );
    }

        #[test]
    fn ssx_audio_system_stop_object_loop_matches() {
        check(
            ee::s08::f_00297eb8__generated,
            ssx_audio_system_stop_object_loop,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x0029_7eb8, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (seed % 3) as u64;
                for slot in 0..4u32 {
                    let s = OBJ + slot * 0x30;
                    let hit = slot == seed % 4;
                    w(c, s + 0x5a00, if hit { 1 } else { 0 });
                    w(c, s + 0x5a04, if hit { seed % 5 } else { 99 });
                    w(c, s + 0x5a20, if hit { seed % 3 } else { 99 });
                    w(c, s + 0x5a24, 0x1234 + slot);
                }
                w(c, OBJ + 0x118, OBJ2);
                w(c, OBJ2, OBJ3);
            },
        );
    }

        // ------------------------------------------------------------------- UI

    #[test]
    fn cui_scroll_render_matches() {
        check(ee::s13::f_0039d968__generated, cui_scroll_render, 8, &|c, seed| {
            let mut r = base(c, seed, 0x0039_d968, 2);
            c.g[A0][0] = OBJ as u64;
            let thumb = seed & 1 != 0;
            w(c, OBJ + 0x74, if thumb { OBJ2 } else { 0 });
            w(c, OBJ + 0x78, if seed % 3 == 0 { 0 } else { OBJ3 });
            w(c, OBJ + 0x44, r.edge_f32());
            w(c, OBJ + 0x48, r.edge_f32());
            for (obj, off) in [(OBJ2, 0x14u32), (OBJ3, 0x14u32)] {
                let flags = if seed % 4 == 0 { 0 } else { 0x40 };
                w(c, obj + off, flags);
                let vt = vtable(c, &mut r, &[0x88]);
                w(c, obj + 8, vt);
            }
        });
    }

        #[test]
    fn cfe_state_char_equip_update_stat_bar_matches() {
        check(
            ee::s03::f_0019bc90__generated,
            cfe_state_char_equip_update_stat_bar,
            8,
            &|c, seed| {
                let mut r = base(c, seed, 0x0019_bc90, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[GP][0] = GAME_GP as u64;
                w(c, GAME_GP.wrapping_sub(0x1818), 0x5dc + (seed % 200));
                w(c, OBJ + 0xc4, 0x5dc + (seed % 300));
                w(c, OBJ + 0x70, if seed % 4 == 0 { 0 } else { OBJ2 });
                let vt = vtable(c, &mut r, &[0x48]);
                w(c, OBJ2 + 8, vt);
                w(c, OBJ2 + 0x50, 0);
                w(c, OBJ2 + 0x54, 0);
                w(c, OBJ2 + 0x58, 0);
                c.f[20] = r.edge_f32();
            },
        );
    }

    // ------------------------------------------------------- input and bolts

    fn wstr(c: &mut Cpu, a: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(a + i as u32, *b);
        }
        c.bus.write8(a + s.len() as u32, 0);
    }

    #[test]
    fn input_parser_lookup_modifier_matches() {
        const NAME: u32 = DATA + 0x1000;
        check(
            ee::s10::f_003217d0__generated,
            input_parser_lookup_modifier,
            12,
            &|c, seed| {
                let _ = base(c, seed, 0x0032_17d0, 0);
                let globals = [0x004a_3ef8u32, 0x004a_3f00, 0x004a_3f08, 0x0048_ddd8, 0x004a_3f10];
                for (i, g) in globals.iter().enumerate() {
                    let name = [b'a' + i as u8];
                    wstr(c, *g, &name);
                }
                let pick = seed % 6;
                if pick < 5 {
                    wstr(c, NAME, &[b'a' + pick as u8]);
                } else {
                    wstr(c, NAME, b"zz");
                }
                c.g[A1][0] = NAME as u64;
            },
        );
    }

    #[test]
    fn benew_player_int_is_message_flag_set_matches() {
        check(
            ee::s01::f_00147980__generated,
            benew_player_int_is_message_flag_set,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x0014_7980, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 40) as u64;
                for i in 0..8u32 {
                    w(c, 0x004a_6ca8 + i * 4, seed.wrapping_mul(0x0101_0101).wrapping_add(i));
                }
            },
        );
    }

    /// A node for the (bit, a2) key in the index manager's hash bucket.
    fn bolt_node(c: &mut Cpu, seed: u32, flags_off: u32, value_off: u32, ref_off: u32) {
        let _ = base(c, seed, 0x0019_cc30, 3);
        let bit = (seed % 32) as u32;
        let a2 = (seed % 4) as u32;
        let hash = (bit << 16) | a2;
        let bucket = hash % 0x190;
        let node = OBJ3;
        c.g[A0][0] = OBJ as u64;
        c.g[A1][0] = bit as u64;
        c.g[A2][0] = a2 as u64;
        c.g[A3][0] = if seed & 1 == 0 {
            0
        } else {
            0xffff_ffff_ffff_ffff
        };
        w(c, OBJ + 0x1900 + bucket * 4, node);
        w(c, OBJ + 0x1f40, 0);
        w(c, node, hash);
        c.bus.write16(node + flags_off, if seed % 4 == 0 { 0 } else { 0xffff });
        c.bus.write16(node + value_off, (seed % 7) as u16);
        c.bus.write16(node + ref_off, if seed % 3 == 0 { 0xffff } else { 1 });
    }

    #[test]
    fn bolt_file_index_manager_set_model_slot_matches() {
        check(
            ee::s03::f_0019cc30__generated,
            bolt_file_index_manager_set_model_slot,
            6,
            &|c, seed| bolt_node(c, seed, 6, 0xa, 8),
        );
    }

    #[test]
    fn bolt_file_index_manager_set_texture_slot_matches() {
        check(
            ee::s03::f_0019cd38__generated,
            bolt_file_index_manager_set_texture_slot,
            6,
            &|c, seed| bolt_node(c, seed, 4, 8, 0xa),
        );
    }

    // ------------------------------------------------------------- scripting

    #[test]
    fn wscript_man_create_local_tables_matches() {
        check(
            ee::s10::f_00308aa0__generated,
            wscript_man_create_local_tables,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x0030_8aa0, 2);
                c.g[A0][0] = OBJ as u64;
                let count = 1 + (seed % 3) as u32;
                w(c, OBJ + 0x28c, OBJ2);
                w(c, OBJ2, OBJ3);
                w(c, OBJ3 + 0x10, OBJ4);
                w(c, OBJ4 + 4, count);
                w(c, OBJ2 + 4, OBJ4);
                w(c, OBJ4 + 4, TBL);
                for i in 0..count {
                    let e = TBL + i * 0xc;
                    c.bus.write16(e, if seed % 2 == 0 { 2 } else { 0 });
                    w(c, e + 8, if seed % 3 == 0 { 0 } else { 1 });
                }
            },
        );
    }

    #[test]
    fn world_trigger_manager_count_active_others_matches() {
        check(
            ee::s08::f_002b6630__generated,
            world_trigger_manager_count_active_others,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x002b_6630, 1);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 3) as u64;
                w(c, OBJ, OBJ2);
                for i in 0..40u32 {
                    let e = OBJ2 + i * 0x70;
                    w(c, e, if seed % 2 == 0 && i % 3 == 0 { 1 } else { 0 });
                    w(c, e + 0xc, (seed % 3) as u32);
                }
            },
        );
    }

        // ------------------------------------------------------------ more screens

    #[test]
    fn cfe_popup_confirm_ctor_matches() {
        check(
            ee::s04::f_001d99c8__generated,
            cfe_popup_confirm_ctor,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x001d_99c8, 2);
                w(c, GAME_GP.wrapping_sub(0x10b4), (seed % 2) as u32);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = OBJ3 as u64;
                c.g[A3][0] = OBJ4 as u64;
                c.g[T0][0] = (seed % 5) as u64;
                c.g[T1][0] = (seed % 7) as u64;
                c.g[T2][0] = (seed % 9) as u64;
                c.g[T3][0] = (seed % 11) as u64;
                w(c, OBJ + 0x17c, 0);
                w(c, OBJ + 0x14c, 0);
            },
        );
    }

    /// A screen-owning state: engine at +0x10, a screen created or not.
    fn screen_init(c: &mut Cpu, seed: u32, salt: u64) {
        let _ = base(c, seed, salt, 2);
        c.g[A0][0] = OBJ as u64;
        w(c, OBJ + 0x10, OBJ2);
        w(c, OBJ + 0x40, 0);
        w(c, OBJ + 0x44, (seed % 4) as u32);
        w(c, OBJ2 + 0x10, OBJ3);
    }

        #[test]
    fn cfe_state_lodge_init_matches() {
        check(ee::s04::f_001f3790__generated, cfe_state_lodge_init, 6, &|c, seed| {
            screen_init(c, seed, 0x001f_3790)
        });
    }

    // ---------------------------------------------------------- audio and race

    #[test]
    fn ssx_audio_system_play_peak_transition_music_matches() {
        check(
            ee::s07::f_0028cd48__generated,
            ssx_audio_system_play_peak_transition_music,
            6,
            &|c, seed| {
                let mut r = base(c, seed, 0x0028_cd48, 3);
                c.g[A0][0] = OBJ as u64;
                let vt = vtable(c, &mut r, &[0x10, 0x20]);
                w(c, OBJ + 0x5558, vt);
                w(c, OBJ + 0x118, OBJ2);
                w(c, 0x004a_3628, 0);
            },
        );
    }

    #[test]
    fn cfe_request_line_buy_song_matches() {
        check(
            ee::s03::f_001988d8__generated,
            cfe_request_line_buy_song,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x0019_88d8, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 64) as u64;
                let mask = if seed % 3 == 0 {
                    0xffff_ffff_ffff_ffffu64
                } else {
                    0x0101_0101_0101_0101u64 << (seed % 8)
                };
                c.bus.write64(OBJ + 0x160, mask);
                c.bus.write64(OBJ + 0x158, seed as u64);
                c.bus.write8(OBJ + 0x44, (seed % 4) as u8);
                w(c, OBJ + 0x48, (seed % 9) as u32);
            },
        );
    }

    #[test]
    fn cfe_state_title_transition_in_matches() {
        check(
            ee::s02::f_001947f8__generated,
            cfe_state_title_transition_in,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x0019_47f8, 3);
                c.g[A0][0] = OBJ as u64;
                w(c, GAME_GP.wrapping_sub(0x1930), match seed % 4 {
                    0 => 0,
                    1 => 1,
                    _ => 2 + (seed % 3),
                });
                w(c, OBJ + 0x40, if seed % 3 == 0 { 0 } else { OBJ2 });
                w(c, OBJ2 + 8, OBJ3);
                c.bus.write16(OBJ3 + 0x38, (seed as u16) & !3);
                w(c, OBJ3 + 0x3c, FAKE);
                c.bus.write16(OBJ3 + 0x48, (seed as u16) & !3);
                w(c, OBJ3 + 0x4c, FAKE + 0x10);
            },
        );
    }

    #[test]
    fn ssx_audio_system_store_race_result_matches() {
        check(
            ee::s08::f_002a4660__generated,
            ssx_audio_system_store_race_result,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x002a_4660, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (seed % 7) as u64;
                c.g[A3][0] = (seed % 9) as u64;
                c.g[T0][0] = (seed & 1) as u64;
                let game = WORLD + 0x1000;
                w(c, game + 0xc0, OBJ2);
                w(c, OBJ2 + 0x98, (seed & 2) as u32);
                w(c, OBJ2 + 0x70, 5);
                w(c, OBJ2, (seed % 4) as u32);
                w(c, OBJ + 0x5804, 10);
                w(c, OBJ + 0x5808, 20);
            },
        );
    }

    #[test]
    fn rider_place_at_marker_matches() {
        check(ee::s00::f_0011de60__generated, rider_place_at_marker, 6, &|c, seed| {
            let mut r = base(c, seed, 0x0011_de60, 4);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = OBJ2 as u64;
            c.g[A2][0] = OBJ3 as u64;
            c.g[A3][0] = (seed % 4) as u64;
            w(c, OBJ + 0x790, OBJ4);
            let t = DATA + 0x10000;
            for i in 0..8u32 {
                w(c, t + 4 * i, r.edge_f32());
            }
        });
    }

    #[test]
    fn cps2_mem_card_do_get_save_dir_entry_matches() {
        check(
            ee::s08::f_002c42b0__generated,
            cps2_mem_card_do_get_save_dir_entry,
            4,
            &|c, seed| {
                let _ = base(c, seed, 0x002c_42b0, 4);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 3) as u64;
                w(c, OBJ + 0x1c, 0);
                w(c, OBJ + 0xc, 0);
                w(c, OBJ + 0x3c, 0);
                w(c, GAME_GP.wrapping_add(0x850), 0);
            },
        );
    }

    #[test]
    fn cfe_state_net_messenger_highlight_row_matches() {
        check(
            ee::s03::f_001a5fb8__generated,
            cfe_state_net_messenger_highlight_row,
            4,
            &|c, seed| {
                let _ = base(c, seed, 0x001a_5fb8, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 16) as u64;
                w(c, OBJ + 0x40, OBJ2);
                w(c, 0x0046_15b0, 0);
            },
        );
    }

    #[test]
    fn ov_screen_profile_save_pda_init_slot_labels_matches() {
        check(
            ee::s05::f_00212080__generated,
            ov_screen_profile_save_pda_init_slot_labels,
            4,
            &|c, seed| {
                let _ = base(c, seed, 0x0021_2080, 3);
                c.g[A0][0] = OBJ as u64;
                w(c, OBJ + 0x40, OBJ2);
                w(c, 0x004a_2138, 0);
                w(c, 0x0047_1d68, 0);
            },
        );
    }

    // ---------------------------------------------------------------- engine

        #[test]
    fn cflag_manager_add_flag_matches() {
        check(ee::s11::f_0034c548__generated, cflag_manager_add_flag, 6, &|c, seed| {
            let _ = base(c, seed, 0x0034_c548, 2);
            c.pc = 0x0034_c548;
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = (seed % 7) as u64;
            let used = (seed % 4) as u32;
            for i in 0..15u32 {
                let slot = OBJ + 0x20 + i * 0x188;
                w(c, slot + 0x5c, if i < used { 1 } else { 0 });
                w(c, slot + 4, (seed % 4) as u32);
            }
        });
    }

    #[test]
    fn mdf_file_create_model_matches() {
        check(ee::s10::f_00314cb0__generated, mdf_file_create_model, 4, &|c, seed| {
            let _ = base(c, seed, 0x0031_4cb0, 3);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = 1;
            w(c, OBJ + 8, 0x1000);
            w(c, OBJ + 0xc, 0x1000);
            w(c, 0x1074, (seed % 4) as u32);
            w(c, 0x1070, 0);
            w(c, 0x1000, 0);
        });
    }

    #[test]
    fn net_fe_state_create_keyboard_popup_matches() {
        check(
            ee::s03::f_001a91b0__generated,
            net_fe_state_create_keyboard_popup,
            4,
            &|c, seed| {
                let _ = base(c, seed, 0x001a_91b0, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = (seed % 5) as u64;
                c.g[A3][0] = (seed % 7) as u64;
                c.g[T0][0] = (seed % 9) as u64;
                w(c, OBJ + 0x10, OBJ2);
                w(c, OBJ + 0x40, OBJ3);
                w(c, OBJ3 + 0xd0, OBJ4);
                w(c, OBJ4 + 0x10, OBJ2);
                w(c, OBJ + 0x69c, 0);
            },
        );
    }

    #[test]
    fn ssx_audio_system_dj_say_20cc_matches() {
        check(
            ee::s08::f_002a24b0__generated,
            ssx_audio_system_dj_say_20cc,
            4,
            &|c, seed| {
                let _ = base(c, seed, 0x002a_24b0, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 5) as u64;
                w(c, GAME_GP.wrapping_add(0x173c), FAKE);
                w(c, OBJ + 0x5560, 0);
                w(c, OBJ + 0x5568, 0);
            },
        );
    }

    #[test]
    fn gm_handler7_v06_matches() {
        check(ee::s06::f_0023c560__generated, gm_handler7_v06, 6, &|c, seed| {
            let _ = base(c, seed, 0x0023_c560, 2);
            c.g[A0][0] = OBJ as u64;
            c.bus.write16(OBJ + 8, (seed as u16) & 0x1f);
            c.bus.write16(OBJ + 0xa, (seed as u16) & 7);
            let game = WORLD + 0x1000;
            let race = WORLD + 0x2000;
            w(c, race + 0x28, OBJ2);
            w(c, OBJ2 + 0x790, OBJ3);
            w(c, OBJ3 + 0x198, (seed % 1000) as u32);
            w(c, game + 0x100, 0);
        });
    }

        #[test]
    fn cfe_state_profile_load_on_widget_notify_matches() {
        check(
            ee::s02::f_0018eeb8__generated,
            cfe_state_profile_load_on_widget_notify,
            14,
            &|c, seed| {
                let _ = base(c, seed, 0x0018_eeb8, 1);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = match seed % 7 {
                    0 => 6,
                    1 => 8,
                    2 => 9,
                    3 => 10,
                    4 => 0,
                    5 => 5,
                    _ => (seed % 12) as u64,
                };
                w(c, GAME_GP.wrapping_sub(0x1b08), if seed % 3 == 0 { 2 } else { 0 });
                w(c, OBJ + 0x220, (seed % 2) as u32);
                w(c, OBJ2 + 0x428, 0);
                w(c, OBJ2 + 0x434, 0);
            },
        );
    }

        #[test]
    fn spline_modifier_ctor_2_matches() {
        check(
            ee::s11::f_003595d8__generated,
            spline_modifier_ctor_2,
            4,
            &|c, seed| {
                let _ = base(c, seed, 0x0035_95d8, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                w(c, OBJ2, OBJ3);
                c.bus.write16(OBJ3 + 0x10, (seed as u16) & !3);
                w(c, OBJ3 + 0x14, FAKE);
                w(c, OBJ + 0xa0, 0);
                w(c, OBJ + 0xb0, 0);
                w(c, OBJ + 0xc0, 0);
                w(c, OBJ + 0xd8, 0);
            },
        );
    }

}
