//! `src/fe/ovtemplatedialog.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::all_in_one_fe::oc999_yn_dtor;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::fe::feasyncfile::gp_13 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::gp_3 as gp_m2;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2 as lw_at;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::festateloadhint::accept_key_6;
pub(crate) use crate::game::fe::festatetrophyroom::handle_list_key;
pub(crate) use crate::game::movie::movieplayer::at;
pub(crate) use crate::game::movie::movieplayer::gp;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::world::wscriptcache::p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0020_a380, cov_state_overlay_init, "cOVStateOverlay_Init"),
    (
        0x0020_a8b0,
        game_ui_get_results_screen_id,
        "GameUI_GetResultsScreenId",
    ),
    (
        0x0020_cbe8,
        game_ui_can_open_pause_menu,
        "GameUI_CanOpenPauseMenu",
    ),
    (
        0x0020_cc60,
        game_ui_open_finish_screen,
        "GameUI_OpenFinishScreen",
    ),
    (
        0x0020_d110,
        ov_screen_pda_template_open,
        "OVScreenPDATemplate_Open",
    ),
    (
        0x0020_d308,
        ov_screen_overlay_menu_on_widget_notify,
        "OVScreenOverlayMenu_OnWidgetNotify",
    ),
    (
        0x0020_d318,
        ov_screen_overlay_menu_on_widget_action,
        "OVScreenOverlayMenu_OnWidgetAction",
    ),
    (
        0x0020_d4f0,
        ov_screen_yn_dialog_on_widget_created,
        "OVScreenYNDialog_OnWidgetCreated",
    ),
    (
        0x0020_d568,
        ov_screen_yn_dialog_on_widget_notify,
        "OVScreenYNDialog_OnWidgetNotify",
    ),
    (0x002c_26d0, ustr_sprintf, "UStr_Sprintf"),
    (
        0x0030_b8c0,
        w_script_man_is_mission_type2_active,
        "WScriptMan_IsMissionType2Active",
    ),
    (
        0x0020_cba0,
        game_ui_is_pause_menu_open,
        "GameUI_IsPauseMenuOpen",
    ),
    (
        0x0020_d190,
        ov_screen_pda_template_close,
        "OVScreenPDATemplate_Close",
    ),
    (0x0020_cfd0, game_ui_update_2, "GameUI_Update"),
    (
        0x0020_a8f8,
        game_ui_open_screen_for_state,
        "GameUI_OpenScreenForState",
    ),
    (
        0x0020_cb58,
        game_ui_is_replay_menu_open,
        "GameUI_IsReplayMenuOpen",
    ),
    (
        0x0020_ccf8,
        game_ui_handle_menu_result,
        "GameUI_HandleMenuResult",
    ),
    (
        0x0020_d1d8,
        ov_screen_overlay_menu_init,
        "OVScreenOverlayMenu_Init",
    ),
    (0x0020_ab50, game_ui_open_screen, "GameUI_OpenScreen"),
    (
        0x0020_a778,
        ovscreen_pda_template_init,
        "OVScreenPDATemplate_Init",
    ),
    (
        0x0020_bab8,
        game_ui_push_popup_screen,
        "GameUI_PushPopupScreen",
    ),
    (0x0020_ca10, gameui_open_pause_menu, "GameUI_OpenPauseMenu"),
    (
        0x0020_d338,
        ov_screen_yn_dialog_init,
        "OVScreenYNDialog_Init",
    ),
    (
        0x0020_d590,
        ov_screen_yn_dialog_on_widget_action,
        "OVScreenYNDialog_OnWidgetAction",
    ),
    (
        0x0020_dbd0,
        ovscreen_yn_dialog_quit_game_update,
        "OVScreenYNDialogQuitGame_Update",
    ),
    (
        0x0020_dc68,
        ov_screen_replay_menu_init,
        "OVScreenReplayMenu_Init",
    ),
    (
        0x0021_c708,
        ov_screen_overlay_menu_dtor,
        "OVScreenOverlayMenu_dtor",
    ),
    (
        0x0021_c730,
        ov_screen_overlay_menu_ctor,
        "OVScreenOverlayMenu_ctor",
    ),
    (
        0x0021_c798,
        ov_screen_yn_dialog_dtor,
        "OVScreenYNDialog_dtor",
    ),
    (
        0x0021_c7c0,
        ov_screen_yn_dialog_ctor,
        "OVScreenYNDialog_ctor",
    ),
    (
        0x0021_c828,
        ov_screen_yn_dialog_update,
        "OVScreenYNDialog_Update",
    ),
    (
        0x0021_c928,
        ov_screen_yn_dialog_quit_game_dtor,
        "OVScreenYNDialogQuitGame_dtor",
    ),
    (
        0x0021_c950,
        ov_screen_yn_dialog_quit_game_ctor,
        "OVScreenYNDialogQuitGame_ctor",
    ),
    (
        0x0021_ca88,
        ov_screen_pda_template_dtor,
        "OVScreenPDATemplate_dtor",
    ),
    (
        0x0021_caa8,
        ov_screen_pda_template_ctor,
        "OVScreenPDATemplate_ctor",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {

    pub(crate) use crate::game::ai::rider::callee::GAME_UI_OPEN_SCREEN;

    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;

    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;

    pub(crate) use crate::game::movie::movieplayer::callee::MEM_ALLOC_NAMED;

    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_MGR_ADD_BACKGROUND_STATE;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {

    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_INTERFACE_FV;
}

/// `int GameUI_GetResultsScreenId(uint state, int mode)` (0x20a8b0): maps a
/// race state to the results screen id (0x16, 0xb or 0xa).
pub fn game_ui_get_results_screen_id(c: &mut Cpu) {
    let state = c.g[A0][0];
    let mode = c.g[A1][0];
    c.cycles += 4;
    c.g[V0][0] = sx((state as u32).wrapping_sub(6));
    c.g[V0][0] = (c.g[V0][0] < 6) as u64;
    let not_six_to_eleven = c.g[V0][0] == 0;
    c.g[V0][0] = sx((state as u32).wrapping_sub(4));
    if not_six_to_eleven {
        c.cycles += 3;
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            let is_zero = state == 0;
            c.g[V0][0] = 3;
            if !is_zero {
                c.cycles += 2;
                if mode != c.g[V0][0] {
                    c.cycles += 2;
                    c.g[V0][0] = 0xa;
                    abi::ret(c);
                    return;
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 0xb;
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0x16;
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Formatting and the MPEG2 bit reader.

/// `int UStr_Sprintf(wchar_t* dst, const wchar_t* fmt, ...)` (0x2c26d0): the
/// varargs shim: spill `a2..a3`, `t0..t3` (at `+0x50`) and `f12`, `f14`,
/// `f16`, `f18` (at `+0x40`) into the frame, then `USTR_vsprintf(dst, fmt,
/// &a2)`.
pub fn ustr_sprintf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x80);
        c.sd(p(sp(c), 0x50), c.g[A2][0]);
        frame.save_ra(c, 0);
        c.g[A2][0] = sx(p(sp(c), 0x50));
        c.sd(p(sp(c), 0x58), c.g[A3][0]);
        c.sd(p(sp(c), 0x60), c.g[T0][0]);
        c.sd(p(sp(c), 0x68), c.g[T1][0]);
        c.sd(p(sp(c), 0x70), c.g[T2][0]);
        c.sd(p(sp(c), 0x78), c.g[T3][0]);
        c.swc1(12, p(sp(c), 0x40));
        c.swc1(14, p(sp(c), 0x44));
        c.swc1(16, p(sp(c), 0x48));
        c.swc1(18, p(sp(c), 0x4c));
        call_ptr(c, 0x003d_c450, 0x002c_2708)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int OVScreenOverlayMenu_OnWidgetNotify(OVScreen*, void*, int key)`
/// (0x0020d308).
pub fn ov_screen_overlay_menu_on_widget_notify(c: &mut Cpu) {
    accept_key_6(c);
}

/// `int OVScreenYNDialog_OnWidgetNotify(OVScreen*, void*, int key)`
/// (0x0020d568).
pub fn ov_screen_yn_dialog_on_widget_notify(c: &mut Cpu) {
    handle_list_key(c);
}

/// `void OVScreenOverlayMenu_OnWidgetAction(OVScreen*, void* item, int action)`
/// (0x0020d318): picking item action 5 sets the global overlay-menu flag
/// (`gp-0x9f4`, 0x4a26fc).
pub fn ov_screen_overlay_menu_on_widget_action(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 5;
    if abi::arg64(c, 1) != 0 {
        c.cycles += 2;
        let action = abi::arg64(c, 2);
        c.g[V0][0] = 1;
        if action == 5 {
            c.cycles += 1;
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[V0][0]);
        }
    }
    c.cycles += 2;
    abi::ret(c);
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

/// `void OVScreenPDATemplate_Open(void)` (0x0020d110): once per process, add
/// the PDA template background state.
pub fn ov_screen_pda_template_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = Addr(gp(c, -0x9e4)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let already = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !already {
            c.cycles += 20;
            c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
            c.g[A1][0] = sx(0x470000);
            c.g[A1][0] = sx(0x470000u32.wrapping_add(0x1a10));
            c.g[A0][0] = 0x48;
            c.g[V0][0] = Addr(at(c, V1, 0x84)).lw(c);
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            c.g[S1][0] = Addr(at(c, V0, 0x48)).lw(c);
            callee::MEM_ALLOC_NAMED.call(c, 0x0020_d14c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S0][0];
            callee::UI_STATE_CTOR.call(c, 0x0020_d15c)?;
            c.g[V1][0] = sx(0x470000);
            c.g[A0][0] = sx(word(c, S1).wrapping_add(0x18));
            c.g[V1][0] = sx(0x470000u32.wrapping_add(0x43f0));
            c.g[A1][0] = c.g[S0][0];
            Addr(at(c, S0, 0x8)).sw(c, c.g[V1][0]);
            Addr(gp(c, -0x9e4)).sw(c, c.g[S0][0]);
            callee::UI_STATE_MGR_ADD_BACKGROUND_STATE.call(c, 0x0020_d178)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameUI_OpenFinishScreen(void)` (0x20cc60): open the finish screen that
/// matches the current race mode and player count.
pub fn game_ui_open_finish_screen(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, table_callee::BE_GET, 0x0020_cc70)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, table_callee::BE_GET_INTERFACE_FV, 0x0020_cc7c)?;
        c.g[V1][0] = 0x530000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x49).lb(c);
        c.g[V1][0] = Addr(c.g[V1][0] as u32).off(0x48).lb(c);
        let no_mode = c.g[V1][0] == 0;
        c.g[A0][0] = 2;
        let mut stage: u8;
        if no_mode {
            stage = 1;
        } else {
            c.cycles += 2;
            let two_players = c.g[V0][0] != c.g[A0][0];
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            stage = if two_players { 2 } else { 1 };
        }
        if stage == 2 {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 3) as u64;
            let one_to_three = c.g[V0][0] != 0;
            c.g[V0][0] = 6;
            if one_to_three {
                stage = 3;
            } else {
                c.cycles += 2;
                let not_six = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 4;
                stage = if not_six { 4 } else { 3 };
            }
        }
        match stage {
            1 => {
                c.cycles += 4;
                c.g[A0][0] = 0xc;
                callee::GAME_UI_OPEN_SCREEN.call(c, 0x0020_cca4)?;
                frame.restore_ra(c, 0);
            }
            3 => {
                c.cycles += 4;
                c.g[A0][0] = 0xd;
                callee::GAME_UI_OPEN_SCREEN.call(c, 0x0020_ccc8)?;
                frame.restore_ra(c, 0);
            }
            _ => {
                c.cycles += 2;
                if c.g[V1][0] != c.g[V0][0] {
                    c.cycles += 2;
                    c.g[A0][0] = 0xc;
                    callee::GAME_UI_OPEN_SCREEN.call(c, 0x0020_cce8)?;
                } else {
                    c.cycles += 2;
                    Addr(gp_2(c, -0x9f4)).sw(c, c.g[A0][0]);
                }
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
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
        c.g[V1][0] = Addr(gp_3(c, -0x204)).lw(c);
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

// ---------------------------------------------------------------------------
// Replay start.

/// `void Replay_BeginPlayback(Replay* self, int a1, int a2, int a3)`
/// (0x26f8a0): start the replay from a snapshot unless one is already
/// running, then switch the replay cameras and tell the audio system.
pub fn replay_begin_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xe));
        c.g[V0][0] = (c.g[V0][0] < 2u64) as u64;
        c.g[V0][0] = c.g[V0][0] ^ 1;
        let busy = c.g[V0][0] == 0;
        c.g[S3][0] = c.g[A3][0];
        if !busy {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x608)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                call_ptr(c, 0x0026_f7f8, 0x0026_f8f0)?;
            }
            c.cycles += 13;
            c.g[A1][0] = 7;
            call_ptr(c, 0x0026_f4a0, 0x0026_f8fc)?;
            Addr(p(r32(c, S0), 0x618)).sw(c, 0);
            c.g[A1][0] = c.g[S2][0];
            Addr(p(r32(c, S0), 0x61c)).sw(c, 0);
            c.g[A0][0] = c.g[S0][0];
            Addr(p(r32(c, S0), 0x620)).sw(c, c.g[S1][0]);
            call_ptr(c, 0x0027_02f8, 0x0026_f914)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x0027_03f0, 0x0026_f920)?;
            if c.g[S1][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0026_ef30, 0x0026_f930)?;
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0026_ee68, 0x0026_f940)?;
            }
            c.cycles += 2;
            let have_a2 = c.g[S1][0] != 0;
            c.lq(S0, frame.at(c, 0x40));
            if have_a2 {
                c.cycles += 1;
                c.lq(S1, frame.at(c, 0x30));
            } else {
                c.cycles += 2;
                let have_a3 = c.g[S3][0] != 0;
                c.lq(S1, frame.at(c, 0x30));
                if !have_a3 {
                    c.cycles += 4;
                    call_ptr(c, 0x0028_b180, 0x0026_f958)?;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0028_f200, 0x0026_f960)?;
                    c.cycles += 1;
                    c.lq(S0, frame.at(c, 0x40));
                    c.cycles += 1;
                    c.lq(S1, frame.at(c, 0x30));
                }
            }
        } else {
            c.cycles += 1;
            c.lq(S0, frame.at(c, 0x40));
            c.cycles += 1;
            c.lq(S1, frame.at(c, 0x30));
        }
        c.cycles += 5;
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod game_ui {}

pub(crate) mod game_ui_callee {}

// ---------------------------------------------------------------------------
// GameUI_IsPauseMenuOpen (0x20cba0)

/// `gp-0x848`: the world (`D_004A28A8`).
pub(crate) const PAUSE_WORLD_GP: u32 = 0x848;

/// World: the game UI; GameUI +0x48: the UI root, whose state manager is at +0x18.
pub(crate) const PAUSE_WORLD_UI: u32 = 0x84;

pub(crate) const PAUSE_UI_ROOT: u32 = 0x48;

pub(crate) const PAUSE_ROOT_STATE_MGR: u32 = 0x18;

/// The pause menu state's name (`D_0046F818`).
pub(crate) const PAUSE_STATE_NAME: u32 = 0x0046_f818;

/// `GetHashValue32(const char*)`.
pub(crate) const PAUSE_HASH: u32 = 0x0031_7670;

/// `UIStateMgr_FindState(mgr, id)`.
pub(crate) const PAUSE_FIND_STATE: u32 = 0x0039_f9d8;

/// `bool GameUI_IsPauseMenuOpen(void)` (0x20cba0): whether the UI state
/// manager holds the pause menu state (found by the hash of its name).
pub fn game_ui_is_pause_menu_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(PAUSE_WORLD_GP)).lw(c);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x10);
        c.g[A0][0] = sx(PAUSE_STATE_NAME);
        c.g[V0][0] = Addr(r32(c, V1)).off(PAUSE_WORLD_UI).lw(c);
        let root = Addr(r32(c, V0)).off(PAUSE_UI_ROOT).lw(c);
        c.g[S0][0] = sx((root as u32).wrapping_add(PAUSE_ROOT_STATE_MGR));
        call_ptr(c, PAUSE_HASH, 0x0020_cbc8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, PAUSE_FIND_STATE, 0x0020_cbd4)?;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = (c.g[V0][0] != 0) as u64;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenPDATemplate_Close()` (0x20d190): unlink and delete the PDA
/// template (`gp_m2-0x9e4`), then clear the global.
pub fn ov_screen_pda_template_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(gp_m2(c, -0x9e4)).lw(c);
        let none = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 5;
            call_ptr(c, 0x0039_7740, 0x0020_d1a8)?;
            c.g[A2][0] = Addr(gp_m2(c, -0x9e4)).lw(c);
            let gone = c.g[A2][0] == 0;
            c.g[A1][0] = 3;
            if !gone {
                c.cycles += 5;
                let template = abi::r32(c, A2);
                c.g[V0][0] = Addr(template.wrapping_add(8)).lw(c);
                let vtable = c.g[V0][0] as u32;
                c.g[A0][0] = Addr(vtable.wrapping_add(8)).lh(c);
                c.g[V1][0] = Addr(vtable.wrapping_add(0xc)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx(template.wrapping_add(abi::r32(c, A0)));
                call_ptr(c, target, 0x0020_d1c8)?;
            }
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        Addr(gp_m2(c, -0x9e4)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// GameUI_Update (0x20cfd0)

/// The pending UI request (`gp-0x9f4`): 0 none, 1 menu result, 2 replay
/// finished, 3 close the PDA template, 4 back to the front end.
pub(crate) const GUU_REQUEST: u32 = 0x9f4;

/// The request jump table (5 words).
pub(crate) const GUU_TABLE: u32 = 0x0047_1ae0;

/// The world's game (`world+0x84`) fields: UI state manager (+0x48, its
/// stack at +0x18), replay (+0x28), flags (+0x200, word +8), mode (+0x214).
pub(crate) const GUU_GAME: u32 = 0x84;

pub(crate) const GUU_UI_MGR: u32 = 0x48;

pub(crate) const GUU_UI_STACK: u32 = 0x18;

pub(crate) const GUU_GET_ACTIVE_STATE: u32 = 0x0039_fa88;

pub(crate) const GUU_HANDLE_MENU_RESULT: Callee = Callee {
    addr: 0x0020_ccf8,
    f: ee::s05::GameUI_HandleMenuResult,
};

pub(crate) const GUU_REPLAY_FINISH: u32 = 0x0027_06f0;

pub(crate) const GUU_BE_GET_INTERFACE: u32 = 0x0014_dd58;

pub(crate) const GUU_CLOSE_ALL: u32 = 0x0039_f840;

pub(crate) const GUU_PDA_CLOSE: Callee = Callee {
    addr: 0x0020_d190,
    f: ee::s05::OVScreenPDATemplate_Close,
};

pub(crate) const GUU_REQUEST_STATE: u32 = 0x0023_1250;

/// `void GameUI_Update()` (0x20cfd0): act on the pending UI request.
/// 1: handle the menu result. 2: finish the replay, set the game's flag
/// word, and in mode 2 with the active UI state's id 8, touch BE
/// interface 7 and close every UI state if the flag at 0x534b30 is set.
/// 3: close the PDA template and every UI state. 4: request game state
/// 0xe (2, 0). Requests 2-4 are cleared first.
pub fn game_ui_update_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        let world = |c: &mut Cpu| Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        c.g[V1][0] = world(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, V1)).off(GUU_GAME).lw(c);
        c.g[S1][0] = Addr(r32(c, V0)).off(GUU_UI_MGR).lw(c);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(GUU_UI_STACK));
        c.g[S0][0] = 0;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, GUU_GET_ACTIVE_STATE, 0x0020_d000)?;
            c.g[S0][0] = c.g[V0][0];
        }

        c.cycles += 4;
        let request = Addr(r32(c, GP).wrapping_sub(GUU_REQUEST));
        c.g[V1][0] = request.lw(c);
        c.g[V0][0] = (c.g[V1][0] < 5) as u64;
        let in_table = c.g[V0][0] != 0;
        c.g[V0][0] = 0x47_0000;
        // The paths that end at the shared exit (restores s0 there).
        let mut tail = true;
        if in_table {
            c.cycles += 6;
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(GUU_TABLE & 0xffff));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = Addr(r32(c, V1)).lw(c);
            let target = r32(c, A0);
            match target {
                0x0020_d02c => {
                    c.cycles += 4;
                    GUU_HANDLE_MENU_RESULT.call(c, 0x0020_d034)?;
                    frame.restore(c, S0, 0x20);
                    tail = false;
                }
                0x0020_d03c => tail = guu_replay_finished(c)?,
                0x0020_d0b8 => {
                    c.cycles += 9;
                    request.sw(c, 0);
                    GUU_PDA_CLOSE.call(c, 0x0020_d0c0)?;
                    c.g[V0][0] = world(c);
                    c.g[V1][0] = Addr(r32(c, V0)).off(GUU_GAME).lw(c);
                    c.g[A0][0] = Addr(r32(c, V1)).off(GUU_UI_MGR).lw(c);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(GUU_UI_STACK));
                    call_ptr(c, GUU_CLOSE_ALL, 0x0020_d0d4)?;
                    frame.restore(c, S0, 0x20);
                    tail = false;
                }
                0x0020_d0dc => {
                    c.cycles += 7;
                    c.g[V0][0] = world(c);
                    c.g[A1][0] = 0xe;
                    request.sw(c, 0);
                    c.g[A2][0] = 2;
                    c.g[A0][0] = Addr(r32(c, V0)).off(GUU_GAME).lw(c);
                    c.g[A3][0] = 0;
                    call_ptr(c, GUU_REQUEST_STATE, 0x0020_d0f8)?;
                }
                0x0020_d0f8 => {}
                _ => {
                    c.jump_out(target, 0x0020_cfd0, 0x0020_d10c);
                    return Ok(());
                }
            }
        }
        if tail {
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

/// Request 2 of `GameUI_Update` (s0 = the active UI state or 0, s1 = the
/// UI manager). Ok(true) when it ends at the shared exit (which restores
/// s0), Ok(false) when it restored s0 itself (where the original does,
/// once before the BE call).
pub(crate) fn guu_replay_finished(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    let frame = Frame { size: 0x30 };
    c.cycles += 11;
    let gp = r32(c, GP);
    c.g[V1][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
    Addr(gp.wrapping_sub(GUU_REQUEST)).sw(c, 0);
    c.g[V0][0] = Addr(r32(c, V1)).off(GUU_GAME).lw(c);
    c.g[A0][0] = Addr(r32(c, V0)).off(0x28).lw(c);
    call_ptr(c, GUU_REPLAY_FINISH, 0x0020_d050)?;
    c.g[A1][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
    c.g[A0][0] = 1;
    let world = r32(c, A1);
    c.g[V0][0] = Addr(world).off(GUU_GAME).lw(c);
    c.g[V1][0] = Addr(r32(c, V0)).off(0x200).lw(c);
    Addr(r32(c, V1)).off(8).sw(c, 1);
    if c.g[S0][0] == 0 {
        return Ok(true);
    }
    c.cycles += 5;
    c.g[V0][0] = Addr(world).off(GUU_GAME).lw(c);
    c.g[A0][0] = 2;
    c.g[V1][0] = Addr(r32(c, V0)).off(0x214).lw(c);
    if c.g[V1][0] != c.g[A0][0] {
        frame.restore(c, S0, 0x20);
        return Ok(false);
    }
    c.cycles += 4;
    c.g[V1][0] = Addr(r32(c, S0)).off(0x98).lw(c);
    c.g[V0][0] = 8;
    let id8 = c.g[V1][0] == c.g[V0][0];
    frame.restore(c, S0, 0x20);
    if !id8 {
        return Ok(false);
    }
    c.cycles += 7;
    c.g[A0][0] = Addr(world).off(0x78).lw(c);
    c.g[A1][0] = 7;
    call_ptr(c, GUU_BE_GET_INTERFACE, 0x0020_d098)?;
    c.g[V1][0] = 0x53_0000;
    c.g[V0][0] = Addr(0x0053_4b30).lw(c);
    frame.restore(c, S0, 0x20);
    if c.g[V0][0] == 0 {
        return Ok(false);
    }
    c.cycles += 4;
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(GUU_UI_STACK));
    call_ptr(c, GUU_CLOSE_ALL, 0x0020_d0b0)?;
    frame.restore(c, S0, 0x20);
    Ok(false)
}

/// `GameUI_OpenScreenForState` (0x0020a8f8): Picks the overlay screen to open for a game-mode change (2: in-race pause/restart screen, 7: results) from the event setup, rewards and network state, then opens it (or sets the overlay status to 2). `(mode)`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn game_ui_open_screen_for_state(c: &mut Cpu) {
    guest(c, |c| {
        'b_20a934: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
            c.g[A0][0] = c.lw(c.g[A0][0] as u32);
            let t = c.g[A0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_20a934;
            } else {
                c.cycles += 1;
                c.g[A1][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
                break 'b_20a934;
            }
        }
        'b_20ab30: {
            c.cycles += 2;
            let t = c.g[A1][0] != 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            if t {
                break 'b_20ab30;
            } else {
                'b_20ab2c: {
                    'b_20ab14: {
                        'b_20a9ec: {
                            'b_20a98c: {
                                c.cycles += 15;
                                c.g[S3][0] = 0u64;
                                call_ptr(c, 0x0014dc80, 0x0020a944)?; // call_indirect
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0u64;
                                call_ptr(c, 0x0014dd58, 0x0020a950)?; // call_indirect
                                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[V0][0] = 0x530000;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                                c.g[A0][0] = 0x2;
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc0));
                                c.g[S1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                                c.g[S4][0] = c.lw(c.g[V1][0] as u32);
                                c.g[S0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                                let t = c.g[S2][0] == c.g[A0][0];
                                c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                                if t {
                                    break 'b_20a98c;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x7;
                                    if c.g[S2][0] == c.g[V0][0] {
                                        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x78));
                                        break 'b_20a9ec;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;

                                        if t {
                                            break 'b_20ab14;
                                        } else {
                                            break 'b_20a98c;
                                        }
                                    }
                                }
                            }
                            'b_20a9a0: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1ec8));
                                let t = c.g[V0][0] == 0u64;
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x9ec), 0u64);
                                if t {
                                    break 'b_20a9a0;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[S3][0] = 0x11;
                                    if t {
                                        break 'b_20ab14;
                                    } else {
                                        break 'b_20a9a0;
                                    }
                                }
                            }
                            'b_20a9b4: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x4;
                                let t = c.g[S0][0] != c.g[V0][0];

                                if t {
                                    break 'b_20a9b4;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[S2][0]);
                                    if t {
                                        break 'b_20ab14;
                                    } else {
                                        break 'b_20a9b4;
                                    }
                                }
                            }
                            'b_20a9c4: {
                                c.cycles += 2;
                                let t = c.g[S1][0] != 0u64;
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), 0u64);
                                if t {
                                    break 'b_20a9c4;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[S3][0] = 0x8;
                                    if t {
                                        break 'b_20ab14;
                                    } else {
                                        break 'b_20a9c4;
                                    }
                                }
                            }
                            'b_20a9d4: {
                                c.cycles += 2;
                                let t = c.g[V1][0] != c.g[S2][0];
                                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x4));
                                if t {
                                    break 'b_20a9d4;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[S3][0] = 0x15;
                                    if t {
                                        break 'b_20ab14;
                                    } else {
                                        break 'b_20a9d4;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[V1][0] = 0x7;
                            c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                            c.g[S3][0] = 0x9;
                            c.g[V0][0] = 0x14;
                            let t = true;
                            if c.g[V1][0] == 0 {
                                c.g[S3][0] = c.g[V0][0];
                            }
                            if t {
                                break 'b_20ab14;
                            } else {
                                break 'b_20a9ec;
                            }
                        }
                        'b_20aa08: {
                            c.cycles += 5;
                            c.g[A1][0] = 0x7;
                            call_ptr(c, 0x0014dd58, 0x0020a9f4)?; // call_indirect
                            c.g[V1][0] = 0x4;
                            let t = c.g[S0][0] != c.g[V1][0];

                            if t {
                                break 'b_20aa08;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[S0][0]);
                                if t {
                                    break 'b_20ab2c;
                                } else {
                                    break 'b_20aa08;
                                }
                            }
                        }
                        'b_20aa50: {
                            'b_20aa4c: {
                                c.cycles += 14;
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), 0u64);
                                call_ptr(c, 0x0014dc80, 0x0020aa10)?; // call_indirect
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0x8;
                                call_ptr(c, 0x0014dd58, 0x0020aa1c)?; // call_indirect
                                c.g[S0][0] = c.g[V0][0];
                                call_ptr(c, 0x0014dc80, 0x0020aa24)?; // call_indirect
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0xd;
                                call_ptr(c, 0x0014dd58, 0x0020aa30)?; // call_indirect
                                c.g[S2][0] = c.g[V0][0];
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                let t = (c.g[V0][0] as i64) >= 0;
                                c.g[V1][0] = 0u64;
                                if t {
                                    break 'b_20aa4c;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                                    let t = (c.g[V0][0] as i64) < 0;

                                    if t {
                                        break 'b_20aa50;
                                    } else {
                                        break 'b_20aa4c;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V1][0] = 0x1;
                            break 'b_20aa50;
                        }
                        'b_20aab8: {
                            'b_20aa84: {
                                c.cycles += 2;
                                let t = c.g[V1][0] == 0u64;
                                c.g[V0][0] = 0x530000;
                                if t {
                                    break 'b_20aa84;
                                } else {
                                    'b_20aa78: {
                                        c.cycles += 3;
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4b30));
                                        let t = c.g[V1][0] == 0u64;
                                        c.g[S0][0] = c.g[V0][0];
                                        if t {
                                            break 'b_20aa78;
                                        } else {
                                            c.cycles += 5;
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A0][0] = c.g[S1][0];
                                            call_ptr(c, 0x0020a8b0, 0x0020aa70)?; // GameUI_GetResultsScreenId
                                            let t = true;
                                            c.g[S3][0] = c.g[V0][0];
                                            if t {
                                                break 'b_20aab8;
                                            } else {
                                                break 'b_20aa78;
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f0), 0u64);
                                    let t = true;
                                    c.g[S3][0] = 0xf;
                                    if t {
                                        break 'b_20aab8;
                                    } else {
                                        break 'b_20aa84;
                                    }
                                }
                            }
                            'b_20aaa8: {
                                'b_20aaa0: {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.g[S2][0];
                                    call_ptr(c, 0x00158f30, 0x0020aa8c)?; // call_indirect
                                    let t = (c.g[V0][0] as i64) > 0;
                                    c.g[S3][0] = 0x10;
                                    if t {
                                        break 'b_20aaa0;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A1][0] = c.g[S4][0];
                                        if t {
                                            break 'b_20aaa8;
                                        } else {
                                            break 'b_20aaa0;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = true;
                                c.g[S0][0] = 0x530000;
                                if t {
                                    break 'b_20aab8;
                                } else {
                                    break 'b_20aaa8;
                                }
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x0020a8b0, 0x0020aab0)?; // GameUI_GetResultsScreenId
                            c.g[S0][0] = 0x530000;
                            c.g[S3][0] = c.g[V0][0];
                            break 'b_20aab8;
                        }
                        'b_20aaec: {
                            c.cycles += 3;
                            c.g[V0][0] = c.g[S1][0] ^ 0x6;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = 0u64;
                            if t {
                                break 'b_20aaec;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.g[S1][0] ^ 0x7;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.g[S1][0] ^ 0x8;
                                if t {
                                    break 'b_20aaec;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.g[S1][0] ^ 0x9;
                                    if t {
                                        break 'b_20aaec;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.g[S1][0] ^ 0xa;
                                        if t {
                                            break 'b_20aaec;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = c.g[S1][0] ^ 0xb;
                                            if t {
                                                break 'b_20aaec;
                                            } else {
                                                c.cycles += 1;
                                                c.g[A0][0] = (0u64 < c.g[V0][0]) as u64;
                                                break 'b_20aaec;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        'b_20ab00: {
                            c.cycles += 2;
                            let t = c.g[A0][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            if t {
                                break 'b_20ab00;
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                                call_ptr(c, 0x002706b8, 0x0020ab00)?; // call_indirect
                                break 'b_20ab00;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4b30));
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                        if t {
                            break 'b_20ab14;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = 0x8ca0;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x64), c.g[V0][0]);
                            break 'b_20ab14;
                        }
                    }
                    'b_20ab24: {
                        c.cycles += 2;
                        let t = c.g[S3][0] != 0u64;
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_20ab24;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[V0][0]);
                            if t {
                                break 'b_20ab2c;
                            } else {
                                break 'b_20ab24;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x0020ab50, 0x0020ab2c)?; // GameUI_OpenScreen
                    break 'b_20ab2c;
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_20ab30;
            }
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        Ok(())
    })
}

/// `bool GameUI_IsReplayMenuOpen(void)` (0x20cb58): whether the UI state stack
/// holds the "replay" menu state. Looks up the state by the hash of the
/// string at 0x471a40 in the stack at `world->module->+0x48 + 0x18`.
pub fn game_ui_is_replay_menu_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x10);
        c.g[A0][0] = sx(0x0047_1a40);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
        c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lw(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        // GetHashValue32(str_64replay).
        call_ptr(c, 0x0031_7670, 0x0020_cb80)?;
        // cUIStateStack_findState(stack, hash) != 0.
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_f9d8, 0x0020_cb8c)?;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void GameUI_HandleMenuResult(void)` (0x20ccf8; the reference calls it
/// `cOVStateManager_gotoNextOverlay`).
///
/// Acts on the result of the overlay menu that just closed: clears the
/// overlay status (`gp - 0x9f4`) and looks at the id of the current UI state
/// (`+0x98`, 8..35, a jump table at 0x471a50) and the menu's `mMiscData`
/// (`gp - 0x9f0`, a second table at 0x471ac0). Depending on both it opens
/// another overlay screen (`GameUI_OpenScreen` with 0x10, 0x11, 0x13, 0x17,
/// 0x18, 0x21, 0x23 or the results screen), stops the replay, starts a game
/// state, leaves the game, or sets the overlay status to 2 or 4.
pub fn game_ui_handle_menu_result(c: &mut Cpu) {
    if c.pc != 0x0020_ccf8 {
        return ee::s05::f_0020ccf8__entries(c);
    }
    guest(c, |c| {
        'b_20cfc0: {
            'b_20cfbc: {
                'b_20cfb8: {
                    c.cycles += 25;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), 0u64);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039fa88, 0x0020cd24)?;
                    c.g[S0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dc80, 0x0020cd2c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x0020cd38)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc0));
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8));
                    c.g[S1][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x5c12));
                    c.g[V0][0] = (c.g[V1][0] < 0x1cu64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S2][0] = c.lw(c.g[A0][0] as u32);
                    if t {
                        break 'b_20cfb8;
                    } else {
                        'b_20cfb0: {
                            'b_20cf9c: {
                                'b_20cf80: {
                                    'b_20cf44: {
                                        'b_20cf38: {
                                            'b_20cf20: {
                                                'b_20cf1c: {
                                                    'b_20ceb8: {
                                                        'b_20ce60: {
                                                            'b_20ce48: {
                                                                'b_20ce0c: {
                                                                    'b_20cdfc: {
                                                                        'b_20cdcc: {
                                                                            'b_20cda4: {
                                                                                c.cycles += 7;
                                                                                c.g[V0][0] =
                                                                                    0x470000;
                                                                                c.g[V1][0] =
                                                                                    sx((c.g[V1][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1a50));
                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                c.g[A0][0] = c
                                                                                    .lw(c.g[V1][0]
                                                                                        as u32);
                                                                                let t = c.g[A0][0]
                                                                                    as u32;
                                                                                match t {
                                                                                    0x0020cd78 => {
                                                                                        'l_20cd78: loop {
                                                                                            c.cycles += 5;
                                                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x9f0));
                                                                                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                                                                            c.g[V1][0] = (c.g[A0][0] < 0x8u64) as u64;
                                                                                            let t = c.g[V1][0] == 0u64;
                                                                                            c.g[V0][0] = 0x470000;
                                                                                            if t {
                                                                                                break 'b_20cfb8;
                                                                                            } else {
                                                                                                c.cycles += 6;
                                                                                                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1ac0));
                                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                                                                let t = c.g[A0][0] as u32;
                                                                                                match t {
0x0020cd78 => {
if event_due(c) { deliver_event_here(c, 0x0020cd78); } continue 'l_20cd78;
}
0x0020cda4 => {
break 'b_20cda4;
}
0x0020cdcc => {
break 'b_20cdcc;
}
0x0020cdfc => {
break 'b_20cdfc;
}
0x0020ce0c => {
break 'b_20ce0c;
}
0x0020ce48 => {
break 'b_20ce48;
}
0x0020ce60 => {
break 'b_20ce60;
}
0x0020ceb8 => {
break 'b_20ceb8;
}
0x0020cf1c => {
break 'b_20cf1c;
}
0x0020cf38 => {
break 'b_20cf38;
}
0x0020cf44 => {
break 'b_20cf44;
}
0x0020cf80 => {
break 'b_20cf80;
}
0x0020cf9c => {
break 'b_20cf9c;
}
0x0020cfb8 => {
break 'b_20cfb8;
}
_ => { c.jump_out(t, 0x0020ccf8, 0x0020cfd0); return Ok(()); }
}
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    0x0020cda4 => {
                                                                                        break 'b_20cda4;
                                                                                    }
                                                                                    0x0020cdcc => {
                                                                                        break 'b_20cdcc;
                                                                                    }
                                                                                    0x0020cdfc => {
                                                                                        break 'b_20cdfc;
                                                                                    }
                                                                                    0x0020ce0c => {
                                                                                        break 'b_20ce0c;
                                                                                    }
                                                                                    0x0020ce48 => {
                                                                                        break 'b_20ce48;
                                                                                    }
                                                                                    0x0020ce60 => {
                                                                                        break 'b_20ce60;
                                                                                    }
                                                                                    0x0020ceb8 => {
                                                                                        break 'b_20ceb8;
                                                                                    }
                                                                                    0x0020cf1c => {
                                                                                        break 'b_20cf1c;
                                                                                    }
                                                                                    0x0020cf38 => {
                                                                                        break 'b_20cf38;
                                                                                    }
                                                                                    0x0020cf44 => {
                                                                                        break 'b_20cf44;
                                                                                    }
                                                                                    0x0020cf80 => {
                                                                                        break 'b_20cf80;
                                                                                    }
                                                                                    0x0020cf9c => {
                                                                                        break 'b_20cf9c;
                                                                                    }
                                                                                    0x0020cfb8 => {
                                                                                        break 'b_20cfb8;
                                                                                    }
                                                                                    _ => {
                                                                                        c.jump_out(t, 0x0020ccf8, 0x0020cfd0);
                                                                                        return Ok(
                                                                                            (),
                                                                                        );
                                                                                    }
                                                                                }
                                                                            }
                                                                            c.cycles += 10;
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [GP][0]
                                                                                as u32)
                                                                                .wrapping_sub(
                                                                                    0x848,
                                                                                ));
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [V0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x84,
                                                                                ));
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [V1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x28,
                                                                                ));
                                                                            call_ptr(
                                                                                c, 0x002706f0,
                                                                                0x0020cdb4,
                                                                            )?;
                                                                            c.g[V0][0] = 0x1;
                                                                            c.g[A0][0] = 0x21;
                                                                            c.sw(
                                                                                (c.g[GP][0] as u32)
                                                                                    .wrapping_sub(
                                                                                        0xb5c,
                                                                                    ),
                                                                                c.g[V0][0],
                                                                            );
                                                                            call_ptr(
                                                                                c, 0x0020ab50,
                                                                                0x0020cdc4,
                                                                            )?;
                                                                            let t = true;
                                                                            c.lq(
                                                                                16,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x30,
                                                                                    ),
                                                                            );
                                                                            if t {
                                                                                break 'b_20cfbc;
                                                                            } else {
                                                                                break 'b_20cdcc;
                                                                            }
                                                                        }
                                                                        c.cycles += 12;
                                                                        c.g[V0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x848));
                                                                        c.g[V1][0] = c.lw((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x84));
                                                                        c.g[A0][0] = c.lw((c.g[V1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x28));
                                                                        call_ptr(
                                                                            c, 0x002706f0,
                                                                            0x0020cddc,
                                                                        )?;
                                                                        c.g[V0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x848));
                                                                        c.g[A1][0] = 0xd;
                                                                        c.g[A2][0] = 0u64;
                                                                        c.g[A3][0] = 0x1;
                                                                        c.g[A0][0] = c.lw((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x84));
                                                                        call_ptr(
                                                                            c, 0x00231250,
                                                                            0x0020cdf4,
                                                                        )?;
                                                                        let t = true;
                                                                        c.lq(
                                                                            16,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x30),
                                                                        );
                                                                        if t {
                                                                            break 'b_20cfbc;
                                                                        } else {
                                                                            break 'b_20cdfc;
                                                                        }
                                                                    }
                                                                    c.cycles += 4;
                                                                    c.g[A0][0] = 0x23;
                                                                    call_ptr(
                                                                        c, 0x0020ab50, 0x0020ce04,
                                                                    )?;
                                                                    let t = true;
                                                                    c.lq(
                                                                        16,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x30),
                                                                    );
                                                                    if t {
                                                                        break 'b_20cfbc;
                                                                    } else {
                                                                        break 'b_20ce0c;
                                                                    }
                                                                }
                                                                c.cycles += 15;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x848));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x84));
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                call_ptr(
                                                                    c, 0x002706f0, 0x0020ce1c,
                                                                )?;
                                                                c.g[A0][0] = 0x11;
                                                                call_ptr(
                                                                    c, 0x0020ab50, 0x0020ce24,
                                                                )?;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x848));
                                                                c.g[A1][0] = 0u64;
                                                                c.g[A2][0] = 0u64;
                                                                c.g[A3][0] = 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x84));
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                call_ptr(
                                                                    c, 0x0026f8a0, 0x0020ce40,
                                                                )?;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                if t {
                                                                    break 'b_20cfbc;
                                                                } else {
                                                                    break 'b_20ce48;
                                                                }
                                                            }
                                                            c.cycles += 6;
                                                            c.g[V0][0] = 0x1;
                                                            c.g[A0][0] = 0xf;
                                                            c.sw(
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x9f0),
                                                                c.g[V0][0],
                                                            );
                                                            call_ptr(c, 0x0020ab50, 0x0020ce58)?;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x30),
                                                            );
                                                            if t {
                                                                break 'b_20cfbc;
                                                            } else {
                                                                break 'b_20ce60;
                                                            }
                                                        }
                                                        'b_20cea8: {
                                                            c.cycles += 8;
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                            c.g[A1][0] = 0x7;
                                                            c.g[A0][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x78));
                                                            call_ptr(c, 0x0014dd58, 0x0020ce70)?;
                                                            c.g[V1][0] = 0x530000;
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x4b30));
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x204));
                                                            if t {
                                                                break 'b_20cea8;
                                                            } else {
                                                                'b_20ce94: {
                                                                    c.cycles += 2;
                                                                    let t = c.g[A0][0] == 0u64;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[GP][0] as u32)
                                                                            .wrapping_sub(0x848));
                                                                    if t {
                                                                        break 'b_20ce94;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = 0u64;
                                                                        call_ptr(
                                                                            c, 0x00257610,
                                                                            0x0020ce90,
                                                                        )?;
                                                                        c.g[V0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x848));
                                                                        break 'b_20ce94;
                                                                    }
                                                                }
                                                                c.cycles += 5;
                                                                c.g[A1][0] = 0u64;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x84));
                                                                call_ptr(
                                                                    c, 0x00230488, 0x0020cea0,
                                                                )?;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                if t {
                                                                    break 'b_20cfbc;
                                                                } else {
                                                                    break 'b_20cea8;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[A0][0] = 0x18;
                                                        call_ptr(c, 0x0020ab50, 0x0020ceb0)?;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x30),
                                                        );
                                                        if t {
                                                            break 'b_20cfbc;
                                                        } else {
                                                            break 'b_20ceb8;
                                                        }
                                                    }
                                                    'b_20ced8: {
                                                        c.cycles += 4;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x9f0));
                                                        c.g[V0][0] = 0x13;
                                                        let t = c.g[V1][0] != c.g[V0][0];

                                                        if t {
                                                            break 'b_20ced8;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[A0][0] = 0x13;
                                                            call_ptr(c, 0x0020ab50, 0x0020ced0)?;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x30),
                                                            );
                                                            if t {
                                                                break 'b_20cfbc;
                                                            } else {
                                                                break 'b_20ced8;
                                                            }
                                                        }
                                                    }
                                                    'b_20cf0c: {
                                                        c.cycles += 10;
                                                        call_ptr(c, 0x0014dc80, 0x0020cee0)?;
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[A1][0] = 0xd;
                                                        call_ptr(c, 0x0014dd58, 0x0020ceec)?;
                                                        c.g[S0][0] = c.g[V0][0];
                                                        c.g[A0][0] = c.g[S0][0];
                                                        call_ptr(c, 0x00158f30, 0x0020cef8)?;
                                                        let t = (c.g[V0][0] as i64) > 0;

                                                        if t {
                                                            break 'b_20cf0c;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A0][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_20cf20;
                                                            } else {
                                                                break 'b_20cf0c;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[A0][0] = 0x10;
                                                    call_ptr(c, 0x0020ab50, 0x0020cf14)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_20cfbc;
                                                    } else {
                                                        break 'b_20cf1c;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[A0][0] = c.g[S1][0];
                                                break 'b_20cf20;
                                            }
                                            c.cycles += 6;
                                            c.g[A1][0] = c.g[S2][0];
                                            call_ptr(c, 0x0020a8b0, 0x0020cf28)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            call_ptr(c, 0x0020ab50, 0x0020cf30)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            if t {
                                                break 'b_20cfbc;
                                            } else {
                                                break 'b_20cf38;
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x2;
                                        let t = true;
                                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[V0][0]);
                                        if t {
                                            break 'b_20cfb8;
                                        } else {
                                            break 'b_20cf44;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x9f0));
                                    c.g[V0][0] = 0x5;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 6i64) as u64;
                                    if t {
                                        break 'b_20cf9c;
                                    } else {
                                        'b_20cf6c: {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x2;
                                            if t {
                                                break 'b_20cf6c;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] == c.g[V0][0];
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_20cf80;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        17,
                                                        (c.g[SP][0] as u32).wrapping_add(0x20),
                                                    );
                                                    if t {
                                                        break 'b_20cfc0;
                                                    } else {
                                                        break 'b_20cf6c;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x8;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_20cfb0;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            if t {
                                                break 'b_20cfc0;
                                            } else {
                                                break 'b_20cf80;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 7;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                                call_ptr(c, 0x002706f0, 0x0020cf90)?;
                                c.g[V0][0] = 0x4;
                                let t = true;
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[V0][0]);
                                if t {
                                    break 'b_20cfb8;
                                } else {
                                    break 'b_20cf9c;
                                }
                            }
                            c.cycles += 5;
                            c.sw((c.g[GP][0] as u32).wrapping_sub(0x9e8), 0u64);
                            c.g[A0][0] = 0x17;
                            call_ptr(c, 0x0020ab50, 0x0020cfa8)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_20cfbc;
                            } else {
                                break 'b_20cfb0;
                            }
                        }
                        c.cycles += 2;
                        c.g[A0][0] = 0x18;
                        call_ptr(c, 0x0020ab50, 0x0020cfb8)?;
                        break 'b_20cfb8;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_20cfbc;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_20cfc0;
        }
        c.cycles += 4;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void cOVStateOverlayMenu::init(cOVState* this)` (0x20d1d8).
///
/// Creates the overlay menu screen (`onCreateScreen`), marks the
/// `overlayMenu` list object (flag `|= 8` at `+0x90`), then for the five
/// entries `overlayOpt%d`: entry 0 gets the label of option `D_004A2860[0]`
/// (`setUnicodeStringByID`, index stored at `+0x18`), entries 1..4 are hidden
/// (`setVisible(0)`) and made static (`setStatic(1)`) through their vtable.
pub fn ov_screen_overlay_menu_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S3][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S2][0] = 0u64;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S4][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x890));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.g[RA][0] = sx(0x0020d210);
        c.g[S6][0] = 0x470000;
        call_ptr(c, 0x0020a380, 0x0020d210)?;
        c.g[A0][0] = 0x470000;
        c.g[RA][0] = sx(0x0020d21c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1af0));
        call_ptr(c, 0x00317670, 0x0020d21c)?;
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x0020d228);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039d860, 0x0020d228)?;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x90));
        c.g[A0][0] = 0x440000;
        c.g[S5][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d10));
        c.g[V1][0] = c.g[V1][0] | 0x8;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x90), c.g[V1][0]);
        'l_20d240: loop {
            'b_20d2d0: {
                c.cycles += 12;
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020d250);
                c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x1b40));
                call_ptr(c, 0x00417828, 0x0020d250)?;
                c.g[RA][0] = sx(0x0020d258);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x0020d258)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x0020d264);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x0020d264)?;
                c.g[S1][0] = c.g[V0][0];
                if c.g[S1][0] == 0u64 {
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    break 'b_20d2d0;
                } else {
                    'b_20d2cc: {
                        'b_20d2a0: {
                            c.cycles += 2;
                            let t = (c.g[S2][0] as i64) > 0;
                            c.g[A1][0] = 0u64;
                            if t {
                                break 'b_20d2a0;
                            } else {
                                c.cycles += 10;
                                c.g[S0][0] = c.lw(c.g[S4][0] as u32);
                                c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                                c.g[RA][0] = sx(0x0020d28c);
                                c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                                call_ptr(c, 0x00317670, 0x0020d28c)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x0020d298);
                                c.g[A0][0] = c.g[S1][0];
                                call_ptr(c, 0x003a1000, 0x0020d298)?;
                                let t = true;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[S0][0]);
                                if t {
                                    break 'b_20d2cc;
                                } else {
                                    break 'b_20d2a0;
                                }
                            }
                        }
                        c.cycles += 11;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x0020d2b4);
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0020d2b4)?;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x0020d2cc);
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0020d2cc)?;
                        break 'b_20d2cc;
                    }
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    break 'b_20d2d0;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 5i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0020d240);
                }
                continue 'l_20d240;
            } else {
                c.cycles += 10;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

/// `GameUI_OpenScreen` (0x0020ab50): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn game_ui_open_screen(c: &mut Cpu) {
    if c.pc != 0x0020ab50 {
        return ee::s05::f_0020ab50__generated(c);
    }
    guest(c, |c| {
        'b_20abdc: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[T0][0] = 0u64;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = 0x1;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
            let t = c.g[S1][0] != c.g[V1][0];
            c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
            if t {
                break 'b_20abdc;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020ab9c);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020ab9c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020abac);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020abac)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3098));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                break 'b_20abdc;
            }
        }
        'b_20ac44: {
            c.cycles += 3;
            c.g[V0][0] = 0x2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x3;
            if t {
                break 'b_20ac44;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020ac00);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020ac00)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020ac10);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020ac10)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2fc0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x3;
                break 'b_20ac44;
            }
        }
        'b_20aca8: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x4;
            if t {
                break 'b_20aca8;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020ac64);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020ac64)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020ac74);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020ac74)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2ee8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x4;
                break 'b_20aca8;
            }
        }
        'b_20ad0c: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x5;
            if t {
                break 'b_20ad0c;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020acc8);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020acc8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020acd8);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020acd8)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2e10));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x5;
                break 'b_20ad0c;
            }
        }
        'b_20ad70: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x6;
            if t {
                break 'b_20ad70;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020ad2c);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020ad2c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020ad3c);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020ad3c)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2d38));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x6;
                break 'b_20ad70;
            }
        }
        'b_20add4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x7;
            if t {
                break 'b_20add4;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020ad90);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020ad90)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020ada0);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020ada0)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2c60));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x7;
                break 'b_20add4;
            }
        }
        'b_20ae34: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x8;
            if t {
                break 'b_20ae34;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020adf4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020adf4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020ae04);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020ae04)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2b88));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x8;
                break 'b_20ae34;
            }
        }
        'b_20ae94: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x9;
            if t {
                break 'b_20ae94;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020ae54);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020ae54)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020ae64);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020ae64)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2850));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x9;
                break 'b_20ae94;
            }
        }
        'b_20aef4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xa;
            if t {
                break 'b_20aef4;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020aeb4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020aeb4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020aec4);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020aec4)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2778));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0xa;
                break 'b_20aef4;
            }
        }
        'b_20af54: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xb;
            if t {
                break 'b_20af54;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020af14);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020af14)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020af24);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020af24)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4088));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0xb;
                break 'b_20af54;
            }
        }
        'b_20afb4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xc;
            if t {
                break 'b_20afb4;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020af74);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020af74)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020af84);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020af84)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3fb0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0xc;
                break 'b_20afb4;
            }
        }
        'b_20b02c: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xd;
            if t {
                break 'b_20b02c;
            } else {
                c.cycles += 28;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[RA][0] = sx(0x0020afd4);
                c.g[A0][0] = 0xa4;
                call_ptr(c, 0x00317d70, 0x0020afd4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020afe4);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020afe4)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e00));
                c.g[V1][0] = 0x1;
                c.g[A0][0] = 0x470000;
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1cc0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[RA][0] = sx(0x0020b020);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                call_ptr(c, 0x00317670, 0x0020b020)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.g[V0][0] = 0xd;
                break 'b_20b02c;
            }
        }
        'b_20b0a4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xe;
            if t {
                break 'b_20b0a4;
            } else {
                c.cycles += 28;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[RA][0] = sx(0x0020b04c);
                c.g[A0][0] = 0xa4;
                call_ptr(c, 0x00317d70, 0x0020b04c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b05c);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b05c)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3d28));
                c.g[V1][0] = 0x1;
                c.g[A0][0] = 0x470000;
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1cc0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[RA][0] = sx(0x0020b098);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                call_ptr(c, 0x00317670, 0x0020b098)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.g[V0][0] = 0xe;
                break 'b_20b0a4;
            }
        }
        'b_20b104: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xf;
            if t {
                break 'b_20b104;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b0c4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b0c4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b0d4);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b0d4)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4b80));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0xf;
                break 'b_20b104;
            }
        }
        'b_20b164: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x10;
            if t {
                break 'b_20b164;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b124);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b124)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b134);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b134)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34d0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x10;
                break 'b_20b164;
            }
        }
        'b_20b1c4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x11;
            if t {
                break 'b_20b1c4;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b184);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b184)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b194);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b194)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2108));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x11;
                break 'b_20b1c4;
            }
        }
        'b_20b224: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x12;
            if t {
                break 'b_20b224;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xb8;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b1e4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b1e4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b1f4);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b1f4)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4aa8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x12;
                break 'b_20b224;
            }
        }
        'b_20b25c: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x13;
            if t {
                break 'b_20b25c;
            } else {
                c.cycles += 12;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x274;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b244);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b244)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b254);
                c.g[A2][0] = 0x12;
                Callee {
                    addr: 0x001fdbf0,
                    f: ee::s05::OVScreenSaveReplay_ctor,
                }
                .call(c, 0x0020b254)?;
                c.g[T0][0] = c.g[V0][0];
                c.g[V0][0] = 0x13;
                break 'b_20b25c;
            }
        }
        'b_20b294: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x14;
            if t {
                break 'b_20b294;
            } else {
                c.cycles += 12;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x274;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b27c);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b27c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b28c);
                c.g[A2][0] = 0x13;
                Callee {
                    addr: 0x001fe968,
                    f: ee::s05::OVScreenSaveRecords_ctor,
                }
                .call(c, 0x0020b28c)?;
                c.g[T0][0] = c.g[V0][0];
                c.g[V0][0] = 0x14;
                break 'b_20b294;
            }
        }
        'b_20b2f4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x15;
            if t {
                break 'b_20b2f4;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b2b4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b2b4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b2c4);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b2c4)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x25c8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x15;
                break 'b_20b2f4;
            }
        }
        'b_20b354: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x16;
            if t {
                break 'b_20b354;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b314);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b314)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b324);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b324)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x26a0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x16;
                break 'b_20b354;
            }
        }
        'b_20b3b4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x17;
            if t {
                break 'b_20b3b4;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b374);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b374)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b384);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b384)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3ed8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x17;
                break 'b_20b3b4;
            }
        }
        'b_20b418: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x18;
            if t {
                break 'b_20b418;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b3d4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b3d4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b3e4);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b3e4)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x49d0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), 0u64);
                c.g[V0][0] = 0x18;
                break 'b_20b418;
            }
        }
        'b_20b480: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x19;
            if t {
                break 'b_20b480;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b438);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b438)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b448);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b448)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x48f8));
                c.g[V1][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x19;
                break 'b_20b480;
            }
        }
        'b_20b4e8: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1a;
            if t {
                break 'b_20b4e8;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b4a0);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b4a0)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b4b0);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b4b0)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4820));
                c.g[V1][0] = 0x2;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x1a;
                break 'b_20b4e8;
            }
        }
        'b_20b550: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1b;
            if t {
                break 'b_20b550;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b508);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b508)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b518);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b518)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4748));
                c.g[V1][0] = 0x3;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x1b;
                break 'b_20b550;
            }
        }
        'b_20b5b8: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1c;
            if t {
                break 'b_20b5b8;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b570);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b570)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b580);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b580)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4670));
                c.g[V1][0] = 0x4;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x1c;
                break 'b_20b5b8;
            }
        }
        'b_20b620: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1d;
            if t {
                break 'b_20b620;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b5d8);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b5d8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b5e8);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b5e8)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4598));
                c.g[V1][0] = 0x5;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x1d;
                break 'b_20b620;
            }
        }
        'b_20b688: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1e;
            if t {
                break 'b_20b688;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[RA][0] = sx(0x0020b640);
                c.g[A0][0] = 0xa0;
                call_ptr(c, 0x00317d70, 0x0020b640)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b650);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b650)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3688));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[RA][0] = sx(0x0020b680);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                call_ptr(c, 0x001f7548, 0x0020b680)?;
                c.g[T0][0] = c.g[S0][0];
                c.g[V0][0] = 0x1e;
                break 'b_20b688;
            }
        }
        'b_20b6f0: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1f;
            if t {
                break 'b_20b6f0;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[RA][0] = sx(0x0020b6a8);
                c.g[A0][0] = 0xa4;
                call_ptr(c, 0x00317d70, 0x0020b6a8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b6b8);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b6b8)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35b0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[RA][0] = sx(0x0020b6e8);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                call_ptr(c, 0x001f7738, 0x0020b6e8)?;
                c.g[T0][0] = c.g[S0][0];
                c.g[V0][0] = 0x1f;
                break 'b_20b6f0;
            }
        }
        'b_20b750: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x20;
            if t {
                break 'b_20b750;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b710);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b710)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b720);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b720)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3760));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x20;
                break 'b_20b750;
            }
        }
        'b_20b7cc: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
            if t {
                break 'b_20b7cc;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x1d4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b770);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b770)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b780);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b780)?;
                c.g[V0][0] = 0x4a0000;
                c.g[V1][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2088));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3320));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.g[A1][0] = 0xffffffffffffffff;
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[V0][0] = 0x7;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                'l_20b7b0: loop {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[V0][0] != c.g[A1][0];
                    c.g[T0][0] = c.g[S0][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0020b7b0);
                        }
                        continue 'l_20b7b0;
                    } else {
                        break 'b_20b7cc;
                    }
                }
            }
        }
        'b_20b838: {
            c.cycles += 3;
            c.g[V0][0] = 0x21;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x22;
            if t {
                break 'b_20b838;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[RA][0] = sx(0x0020b7f0);
                c.g[A0][0] = 0x390;
                call_ptr(c, 0x00317d70, 0x0020b7f0)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b800);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b800)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x33f8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[RA][0] = sx(0x0020b830);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                Callee {
                    addr: 0x002006b8,
                    f: ee::s05::EventSelect_ctor,
                }
                .call(c, 0x0020b830)?;
                c.g[T0][0] = c.g[S0][0];
                c.g[V0][0] = 0x22;
                break 'b_20b838;
            }
        }
        'b_20b870: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x23;
            if t {
                break 'b_20b870;
            } else {
                c.cycles += 12;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x180;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b858);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b858)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b868);
                c.g[A2][0] = 0x22;
                call_ptr(c, 0x001e3c00, 0x0020b868)?;
                c.g[T0][0] = c.g[V0][0];
                c.g[V0][0] = 0x23;
                break 'b_20b870;
            }
        }
        'b_20b8d0: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x24;
            if t {
                break 'b_20b8d0;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b890);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b890)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b8a0);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b8a0)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3c50));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x24;
                break 'b_20b8d0;
            }
        }
        'b_20b90c: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x25;
            if t {
                break 'b_20b90c;
            } else {
                c.cycles += 13;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x27c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b8f0);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b8f0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = 0x24;
                c.g[RA][0] = sx(0x0020b904);
                c.g[A3][0] = 0x2a;
                Callee {
                    addr: 0x00211088,
                    f: ee::s05::OVScreenProfileSavePDA_ctor,
                }
                .call(c, 0x0020b904)?;
                c.g[T0][0] = c.g[V0][0];
                c.g[V0][0] = 0x25;
                break 'b_20b90c;
            }
        }
        'b_20b944: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x26;
            if t {
                break 'b_20b944;
            } else {
                c.cycles += 12;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x234;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b92c);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b92c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b93c);
                c.g[A2][0] = 0x25;
                Callee {
                    addr: 0x00212138,
                    f: ee::s05::OVScreenAutosave_ctor,
                }
                .call(c, 0x0020b93c)?;
                c.g[T0][0] = c.g[V0][0];
                c.g[V0][0] = 0x26;
                break 'b_20b944;
            }
        }
        'b_20b9ac: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x27;
            if t {
                break 'b_20b9ac;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b964);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b964)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b974);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b974)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x44c0));
                c.g[V1][0] = 0x6;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x27;
                break 'b_20b9ac;
            }
        }
        'b_20ba18: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x28;
            if t {
                break 'b_20ba18;
            } else {
                c.cycles += 25;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b9cc);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b9cc)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b9dc);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b9dc)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2ab0));
                c.g[V1][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), 0u64);
                c.g[V0][0] = 0x28;
                break 'b_20ba18;
            }
        }
        'b_20ba78: {
            c.cycles += 2;
            if c.g[S1][0] != c.g[V0][0] {
                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x1c));
                break 'b_20ba78;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xe4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020ba38);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020ba38)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020ba48);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020ba48)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3248));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x1c));
                break 'b_20ba78;
            }
        }
        c.cycles += 16;
        c.g[V1][0] = 0xffffffffffffc0ff;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[T0][0];
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[A2][0] = 0u64;
        c.g[V0][0] = c.g[V0][0] | 0x100;
        c.g[A3][0] = 0u64;
        c.g[RA][0] = sx(0x0020ba9c);
        c.sw((c.g[T0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        call_ptr(c, 0x0039f320, 0x0020ba9c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `GameUI_PushPopupScreen` (0x0020bab8): builds the overlay/popup state object
/// for popup type `a0` (1..0x28: allocate, run its constructor, set vtable and
/// fields) and pushes it on the UI state stack. The generated body, with its
/// calls through `call_ptr` and unwinds through `?`.
pub fn game_ui_push_popup_screen(c: &mut Cpu) {
    guest(c, |c| {
        'b_20bb60: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S1][0] = c.g[A0][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S3][0] = 0u64;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[S2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x48));
            c.g[RA][0] = sx(0x0020baf4);
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039fa88, 0x0020baf4)?;
            c.g[S5][0] = c.g[V0][0];
            c.g[V0][0] = 0x1;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x2;
            if t {
                break 'b_20bb60;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020bb1c);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020bb1c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bb30);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bb30)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3098));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x2;
                break 'b_20bb60;
            }
        }
        'b_20bbc4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x3;
            if t {
                break 'b_20bbc4;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020bb80);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020bb80)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bb94);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bb94)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2fc0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x3;
                break 'b_20bbc4;
            }
        }
        'b_20bc28: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x4;
            if t {
                break 'b_20bc28;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020bbe4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020bbe4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bbf8);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bbf8)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2ee8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x4;
                break 'b_20bc28;
            }
        }
        'b_20bc8c: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x5;
            if t {
                break 'b_20bc8c;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020bc48);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020bc48)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bc5c);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bc5c)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2e10));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x5;
                break 'b_20bc8c;
            }
        }
        'b_20bcf0: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x6;
            if t {
                break 'b_20bcf0;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020bcac);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020bcac)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bcc0);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bcc0)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2d38));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x6;
                break 'b_20bcf0;
            }
        }
        'b_20bd54: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x7;
            if t {
                break 'b_20bd54;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020bd10);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020bd10)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bd24);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bd24)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2c60));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x7;
                break 'b_20bd54;
            }
        }
        'b_20bdb4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x8;
            if t {
                break 'b_20bdb4;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020bd74);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020bd74)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bd88);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bd88)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2b88));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x8;
                break 'b_20bdb4;
            }
        }
        'b_20be14: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x9;
            if t {
                break 'b_20be14;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020bdd4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020bdd4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bde8);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bde8)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2850));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x9;
                break 'b_20be14;
            }
        }
        'b_20be74: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xa;
            if t {
                break 'b_20be74;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020be34);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020be34)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020be48);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020be48)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2778));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0xa;
                break 'b_20be74;
            }
        }
        'b_20bed4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xb;
            if t {
                break 'b_20bed4;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020be94);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020be94)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bea8);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bea8)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4088));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0xb;
                break 'b_20bed4;
            }
        }
        'b_20bf34: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xc;
            if t {
                break 'b_20bf34;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020bef4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020bef4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bf08);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bf08)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3fb0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0xc;
                break 'b_20bf34;
            }
        }
        'b_20bfac: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xd;
            if t {
                break 'b_20bfac;
            } else {
                c.cycles += 28;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[RA][0] = sx(0x0020bf54);
                c.g[A0][0] = 0xa4;
                call_ptr(c, 0x00317d70, 0x0020bf54)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bf68);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bf68)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e00));
                c.g[V1][0] = 0x1;
                c.g[A0][0] = 0x470000;
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1cc0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[RA][0] = sx(0x0020bfa4);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                call_ptr(c, 0x00317670, 0x0020bfa4)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                c.g[V0][0] = 0xd;
                break 'b_20bfac;
            }
        }
        'b_20c024: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xe;
            if t {
                break 'b_20c024;
            } else {
                c.cycles += 28;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[RA][0] = sx(0x0020bfcc);
                c.g[A0][0] = 0xa4;
                call_ptr(c, 0x00317d70, 0x0020bfcc)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bfe0);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bfe0)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3d28));
                c.g[V1][0] = 0x1;
                c.g[A0][0] = 0x470000;
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1cc0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[RA][0] = sx(0x0020c01c);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                call_ptr(c, 0x00317670, 0x0020c01c)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                c.g[V0][0] = 0xe;
                break 'b_20c024;
            }
        }
        'b_20c084: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xf;
            if t {
                break 'b_20c084;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c044);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c044)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c058);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c058)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4b80));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0xf;
                break 'b_20c084;
            }
        }
        'b_20c0e4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x10;
            if t {
                break 'b_20c0e4;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c0a4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c0a4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c0b8);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c0b8)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34d0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x10;
                break 'b_20c0e4;
            }
        }
        'b_20c144: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x11;
            if t {
                break 'b_20c144;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c104);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c104)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c118);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c118)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2108));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x11;
                break 'b_20c144;
            }
        }
        'b_20c1a4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x12;
            if t {
                break 'b_20c1a4;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xb8;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c164);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c164)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c178);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c178)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4aa8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x12;
                break 'b_20c1a4;
            }
        }
        'b_20c1dc: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x13;
            if t {
                break 'b_20c1dc;
            } else {
                c.cycles += 12;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x274;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c1c4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c1c4)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020c1d4);
                c.g[A2][0] = 0x12;
                Callee {
                    addr: 0x001fdbf0,
                    f: ee::s05::OVScreenSaveReplay_ctor,
                }
                .call(c, 0x0020c1d4)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = 0x13;
                break 'b_20c1dc;
            }
        }
        'b_20c214: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x14;
            if t {
                break 'b_20c214;
            } else {
                c.cycles += 12;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x274;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c1fc);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c1fc)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020c20c);
                c.g[A2][0] = 0x13;
                Callee {
                    addr: 0x001fe968,
                    f: ee::s05::OVScreenSaveRecords_ctor,
                }
                .call(c, 0x0020c20c)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = 0x14;
                break 'b_20c214;
            }
        }
        'b_20c274: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x15;
            if t {
                break 'b_20c274;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c234);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c234)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c248);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c248)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x25c8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x15;
                break 'b_20c274;
            }
        }
        'b_20c2d4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x16;
            if t {
                break 'b_20c2d4;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c294);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c294)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c2a8);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c2a8)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x26a0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x16;
                break 'b_20c2d4;
            }
        }
        'b_20c334: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x17;
            if t {
                break 'b_20c334;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c2f4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c2f4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c308);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c308)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3ed8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x17;
                break 'b_20c334;
            }
        }
        'b_20c398: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x18;
            if t {
                break 'b_20c398;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c354);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c354)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c368);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c368)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x49d0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), 0u64);
                c.g[V0][0] = 0x18;
                break 'b_20c398;
            }
        }
        'b_20c400: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x19;
            if t {
                break 'b_20c400;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c3b8);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c3b8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c3cc);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c3cc)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x48f8));
                c.g[V1][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x19;
                break 'b_20c400;
            }
        }
        'b_20c468: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1a;
            if t {
                break 'b_20c468;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c420);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c420)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c434);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c434)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4820));
                c.g[V1][0] = 0x2;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x1a;
                break 'b_20c468;
            }
        }
        'b_20c4d0: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1b;
            if t {
                break 'b_20c4d0;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c488);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c488)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c49c);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c49c)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4748));
                c.g[V1][0] = 0x3;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x1b;
                break 'b_20c4d0;
            }
        }
        'b_20c538: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1c;
            if t {
                break 'b_20c538;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c4f0);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c4f0)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c504);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c504)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4670));
                c.g[V1][0] = 0x4;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x1c;
                break 'b_20c538;
            }
        }
        'b_20c5a0: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1d;
            if t {
                break 'b_20c5a0;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c558);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c558)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c56c);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c56c)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4598));
                c.g[V1][0] = 0x5;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x1d;
                break 'b_20c5a0;
            }
        }
        'b_20c608: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1e;
            if t {
                break 'b_20c608;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A3][0] = 0u64;
                c.g[RA][0] = sx(0x0020c5c0);
                c.g[A0][0] = 0xa0;
                call_ptr(c, 0x00317d70, 0x0020c5c0)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c5d4);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c5d4)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3688));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[RA][0] = sx(0x0020c604);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                call_ptr(c, 0x001f7548, 0x0020c604)?;
                c.g[V0][0] = 0x1e;
                break 'b_20c608;
            }
        }
        'b_20c670: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1f;
            if t {
                break 'b_20c670;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A3][0] = 0u64;
                c.g[RA][0] = sx(0x0020c628);
                c.g[A0][0] = 0xa4;
                call_ptr(c, 0x00317d70, 0x0020c628)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c63c);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c63c)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35b0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[RA][0] = sx(0x0020c66c);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                call_ptr(c, 0x001f7738, 0x0020c66c)?;
                c.g[V0][0] = 0x1f;
                break 'b_20c670;
            }
        }
        'b_20c6d0: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x20;
            if t {
                break 'b_20c6d0;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c690);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c690)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c6a4);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c6a4)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3760));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x20;
                break 'b_20c6d0;
            }
        }
        'b_20c734: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
            if t {
                break 'b_20c734;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x1d4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c6f0);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c6f0)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020c700);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c700)?;
                c.g[V0][0] = 0x4a0000;
                c.g[V1][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2088));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3320));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.g[A1][0] = 0xffffffffffffffff;
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[V0][0] = 0x7;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[S3][0] = c.g[S0][0];
                break 'b_20c734;
            }
        }
        'b_20c7a0: {
            c.cycles += 3;
            c.g[V0][0] = 0x21;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x22;
            if t {
                break 'b_20c7a0;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A3][0] = 0u64;
                c.g[RA][0] = sx(0x0020c758);
                c.g[A0][0] = 0x390;
                call_ptr(c, 0x00317d70, 0x0020c758)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c76c);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c76c)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x33f8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[RA][0] = sx(0x0020c79c);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                Callee {
                    addr: 0x002006b8,
                    f: ee::s05::EventSelect_ctor,
                }
                .call(c, 0x0020c79c)?;
                c.g[V0][0] = 0x22;
                break 'b_20c7a0;
            }
        }
        'b_20c7d8: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x23;
            if t {
                break 'b_20c7d8;
            } else {
                c.cycles += 12;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x180;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c7c0);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c7c0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020c7d0);
                c.g[A2][0] = 0x22;
                call_ptr(c, 0x001e3c00, 0x0020c7d0)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = 0x23;
                break 'b_20c7d8;
            }
        }
        'b_20c838: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x24;
            if t {
                break 'b_20c838;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c7f8);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c7f8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c80c);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c80c)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3c50));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x24;
                break 'b_20c838;
            }
        }
        'b_20c874: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x25;
            if t {
                break 'b_20c874;
            } else {
                c.cycles += 13;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x27c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c858);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c858)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = 0x24;
                c.g[RA][0] = sx(0x0020c86c);
                c.g[A3][0] = 0x2a;
                Callee {
                    addr: 0x00211088,
                    f: ee::s05::OVScreenProfileSavePDA_ctor,
                }
                .call(c, 0x0020c86c)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = 0x25;
                break 'b_20c874;
            }
        }
        'b_20c8ac: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x26;
            if t {
                break 'b_20c8ac;
            } else {
                c.cycles += 12;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x234;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c894);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c894)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020c8a4);
                c.g[A2][0] = 0x25;
                Callee {
                    addr: 0x00212138,
                    f: ee::s05::OVScreenAutosave_ctor,
                }
                .call(c, 0x0020c8a4)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = 0x26;
                break 'b_20c8ac;
            }
        }
        'b_20c914: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x27;
            if t {
                break 'b_20c914;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c8cc);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c8cc)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c8e0);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c8e0)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x44c0));
                c.g[V1][0] = 0x6;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x27;
                break 'b_20c914;
            }
        }
        'b_20c980: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x28;
            if t {
                break 'b_20c980;
            } else {
                c.cycles += 25;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c934);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c934)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c948);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c948)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2ab0));
                c.g[V1][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), 0u64);
                c.g[V0][0] = 0x28;
                break 'b_20c980;
            }
        }
        'b_20c9e0: {
            c.cycles += 2;
            if c.g[S1][0] != c.g[V0][0] {
                c.sw((c.g[S3][0] as u32).wrapping_add(0x20), c.g[S5][0]);
                break 'b_20c9e0;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xe4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c9a0);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c9a0)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c9b4);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c9b4)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3248));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S3][0] as u32).wrapping_add(0x20), c.g[S5][0]);
                break 'b_20c9e0;
            }
        }
        c.cycles += 12;
        c.g[A0][0] = c.g[S4][0];
        c.g[RA][0] = sx(0x0020c9ec);
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0039f290, 0x0020c9ec)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `OVScreenYNDialog_Init` (0x0020d338): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ov_screen_yn_dialog_init(c: &mut Cpu) {
    if c.pc != 0x0020d338 {
        return ee::s05::f_0020d338__generated(c);
    }
    guest(c, |c| 'l_20d338: loop {
        'b_20d388: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = 0x470000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1af8));
            c.g[RA][0] = sx(0x0020d35c);
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x0020d35c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x0020d370);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x0020d370)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_20d388;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x0020d388);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x0020d388)?;
                break 'b_20d388;
            }
        }
        'b_20d4e0: {
            'b_20d4c4: {
                c.cycles += 28;
                c.g[RA][0] = sx(0x0020d390);
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x0020a380,
                    f: ee::s05::cOVStateOverlay_Init,
                }
                .call(c, 0x0020d390)?;
                c.g[A0][0] = 0x4a0000;
                c.g[RA][0] = sx(0x0020d39c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x23e0));
                call_ptr(c, 0x00317670, 0x0020d39c)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x0020d3a8);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x0020d3a8)?;
                c.g[A0][0] = 0x4a0000;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), 0u64);
                c.g[RA][0] = sx(0x0020d3b8);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x23e8));
                call_ptr(c, 0x00317670, 0x0020d3b8)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x0020d3c4);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x0020d3c4)?;
                c.g[V1][0] = 0x1;
                c.g[A0][0] = 0x470000;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                c.g[RA][0] = sx(0x0020d3d8);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1b08));
                call_ptr(c, 0x00317670, 0x0020d3d8)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x0020d3e4);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x0020d3e4)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = (c.g[V1][0] < 0x7u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x470000;
                if t {
                    break 'b_20d4c4;
                } else {
                    'b_20d478: {
                        'b_20d470: {
                            'b_20d454: {
                                'b_20d44c: {
                                    'b_20d440: {
                                        'b_20d434: {
                                            'b_20d428: {
                                                'b_20d41c: {
                                                    c.cycles += 6;
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x1b60));
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add(c.g[V0][0] as u32));
                                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                    let t = c.g[A0][0] as u32;
                                                    match t {
                                                        0x0020d338 => {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x0020d338);
                                                            }
                                                            continue 'l_20d338;
                                                        }
                                                        0x0020d410 => {
                                                            c.cycles += 3;
                                                            c.g[A0][0] = 0x470000;
                                                            let t = true;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x1b18));
                                                            if t {
                                                                break 'b_20d478;
                                                            } else {
                                                                break 'b_20d41c;
                                                            }
                                                        }
                                                        0x0020d41c => {
                                                            break 'b_20d41c;
                                                        }
                                                        0x0020d428 => {
                                                            break 'b_20d428;
                                                        }
                                                        0x0020d434 => {
                                                            break 'b_20d434;
                                                        }
                                                        0x0020d440 => {
                                                            break 'b_20d440;
                                                        }
                                                        0x0020d44c => {
                                                            break 'b_20d44c;
                                                        }
                                                        0x0020d470 => {
                                                            break 'b_20d470;
                                                        }
                                                        _ => {
                                                            c.jump_out(t, 0x0020d338, 0x0020d4f0);
                                                            return Ok(());
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[A0][0] = 0x470000;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0xa48));
                                                if t {
                                                    break 'b_20d454;
                                                } else {
                                                    break 'b_20d428;
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[A0][0] = 0x470000;
                                            let t = true;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x9d0));
                                            if t {
                                                break 'b_20d454;
                                            } else {
                                                break 'b_20d434;
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x470000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x9e8));
                                        if t {
                                            break 'b_20d454;
                                        } else {
                                            break 'b_20d440;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x470000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1b48));
                                    if t {
                                        break 'b_20d454;
                                    } else {
                                        break 'b_20d44c;
                                    }
                                }
                                c.cycles += 2;
                                c.g[A0][0] = 0x470000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x9f8));
                                break 'b_20d454;
                            }
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x0020d45c);
                            call_ptr(c, 0x00317670, 0x0020d45c)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x0020d468);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x0020d468)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_20d4e0;
                            } else {
                                break 'b_20d470;
                            }
                        }
                        c.cycles += 2;
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10f8));
                        break 'b_20d478;
                    }
                    c.cycles += 19;
                    c.g[RA][0] = sx(0x0020d480);
                    call_ptr(c, 0x00317670, 0x0020d480)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0020d48c);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a1000, 0x0020d48c)?;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x0020d498);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1b30));
                    call_ptr(c, 0x00317670, 0x0020d498)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x0020d4a4);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x0020d4a4)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                    let t = c.g[A2][0] as u32;
                    c.g[RA][0] = sx(0x0020d4bc);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0020d4bc)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_20d4e0;
                    } else {
                        break 'b_20d4c4;
                    }
                }
            }
            c.cycles += 7;
            c.g[A0][0] = 0x470000;
            c.g[RA][0] = sx(0x0020d4d0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x9b8));
            call_ptr(c, 0x00317670, 0x0020d4d0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0020d4dc);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a1000, 0x0020d4dc)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_20d4e0;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `OVScreenYNDialog_OnWidgetAction` (0x0020d590): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ov_screen_yn_dialog_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x0020d590 {
        return ee::s05::f_0020d590__generated(c);
    }
    guest(c, |c| 'l_20d590: loop {
        'b_20dbb8: {
            'b_20dbb4: {
                'b_20dbb0: {
                    c.cycles += 16;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S0][0] = c.g[A2][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[RA][0] = sx(0x0020d5bc);
                    c.g[S2][0] = c.g[A1][0];
                    call_ptr(c, 0x0014dc80, 0x0020d5bc)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0020d5c8);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x0020d5c8)?;
                    let t = c.g[S2][0] == 0u64;
                    c.g[V0][0] = 0x5;
                    if t {
                        break 'b_20dbb0;
                    } else {
                        'b_20db64: {
                            'b_20d5e8: {
                                c.cycles += 2;
                                let t = c.g[S0][0] == c.g[V0][0];
                                c.g[V0][0] = 0x6;
                                if t {
                                    break 'b_20d5e8;
                                } else {
                                    c.cycles += 2;
                                    if c.g[S0][0] == c.g[V0][0] {
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
                                        break 'b_20db64;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                        if t {
                                            break 'b_20dbb4;
                                        } else {
                                            break 'b_20d5e8;
                                        }
                                    }
                                }
                            }
                            'b_20d61c: {
                                c.cycles += 10;
                                c.g[A0][0] = 0x470000;
                                c.g[RA][0] = sx(0x0020d5f4);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fb0));
                                call_ptr(c, 0x00317670, 0x0020d5f4)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x0020d600);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039c778, 0x0020d600)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[V0][0] = 0xffff;
                                let t = c.g[A1][0] == c.g[V0][0];
                                c.g[A1][0] = c.g[A1][0] & 0xffff;
                                if t {
                                    break 'b_20d61c;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x0020d61c);
                                    c.g[A2][0] = 0x1;
                                    call_ptr(c, 0x0039c870, 0x0020d61c)?;
                                    break 'b_20d61c;
                                }
                            }
                            'b_20db54: {
                                'b_20db50: {
                                    'b_20db24: {
                                        'b_20d638: {
                                            c.cycles += 3;
                                            c.g[A1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                            let t = c.g[A1][0] == 0u64;
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_20d638;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A1][0] == c.g[V0][0];
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                                if t {
                                                    break 'b_20db24;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        17,
                                                        (c.g[SP][0] as u32).wrapping_add(0x40),
                                                    );
                                                    if t {
                                                        break 'b_20dbb8;
                                                    } else {
                                                        break 'b_20d638;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
                                        c.g[V1][0] = (c.g[A0][0] < 0x7u64) as u64;
                                        let t = c.g[V1][0] == 0u64;
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                                        if t {
                                            break 'b_20dbb0;
                                        } else {
                                            'b_20daf8: {
                                                'b_20dab8: {
                                                    'b_20da58: {
                                                        'b_20d954: {
                                                            'b_20d904: {
                                                                'b_20d8f4: {
                                                                    'b_20d82c: {
                                                                        c.cycles += 7;
                                                                        c.g[V0][0] = 0x470000;
                                                                        c.g[V1][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1b80,
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[A0][0] =
                                                                            c.lw(c.g[V1][0] as u32);
                                                                        let t = c.g[A0][0] as u32;
                                                                        match t {
                                                                            0x0020d590 => {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0020d590);
                                                                                }
                                                                                continue 'l_20d590;
                                                                            }
                                                                            0x0020d664 => {
                                                                                'b_20d794: {
                                                                                    c.cycles += 8;
                                                                                    c.g[V1][0] =
                                                                                        0x530000;
                                                                                    c.g[S1][0] =
                                                                                        c.g[V1][0];
                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                                                                                    c.g[S3][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x4a));
                                                                                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6));
                                                                                    c.g[V0][0] = (c
                                                                                        .g[V0][0]
                                                                                        < 0x6u64)
                                                                                        as u64;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        == 0u64;
                                                                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x9e8));
                                                                                    if t {
                                                                                        break 'b_20d794;
                                                                                    } else {
                                                                                        'b_20d6e4: {
                                                                                            'b_20d6cc: {
                                                                                                c.cycles += 2;
                                                                                                let t = c.g[V0][0] == 0u64;
                                                                                                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_20d6cc;
                                                                                                } else {
                                                                                                    c.cycles += 16;
                                                                                                    c.g[V0][0] = 0x530000;
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x66e8));
                                                                                                    c.g[A2][0] = 0x440000;
                                                                                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                                                                                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x28f0));
                                                                                                    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                                                                                                    c.g[A0][0] = 0x3;
                                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[A0][0]);
                                                                                                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                                                                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[A1][0]);
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                                                                                                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                                                                                    let t = true;
                                                                                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[V1][0]);
                                                                                                    if t {
break 'b_20d6e4;
} else {
break 'b_20d6cc;
}
                                                                                                }
                                                                                            }
                                                                                            c.cycles += 6;
                                                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                                                                            c.g[RA][0] = sx(0x0020d6dc);
                                                                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                                                                                            call_ptr(c, 0x002706f0, 0x0020d6dc)?;
                                                                                            c.g[V0][0] = 0x3;
                                                                                            c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[V0][0]);
                                                                                            break 'b_20d6e4;
                                                                                        }
                                                                                        'b_20d71c: {
                                                                                            'b_20d718: {
                                                                                                'b_20d704: {
                                                                                                    'b_20d6fc: {
                                                                                                        c.cycles += 3;
                                                                                                        c.g[V0][0] = 0x7;
                                                                                                        let t = c.g[S3][0] == c.g[V0][0];
                                                                                                        c.g[S4][0] = 0xe;
                                                                                                        if t {
break 'b_20d6fc;
} else {
c.cycles += 3;
c.g[V0][0] = 0xa;
let t = c.g[S3][0] != c.g[V0][0];
c.g[V0][0] = 0x8;
if t {
break 'b_20d704;
} else {
break 'b_20d6fc;
}
}
                                                                                                    }
                                                                                                    c.cycles += 2;
                                                                                                    let t = true;
                                                                                                    c.g[S4][0] = 0xf;
                                                                                                    if t {
break 'b_20d718;
} else {
break 'b_20d704;
}
                                                                                                }
                                                                                                'b_20d714: {
                                                                                                    c.cycles += 2;
                                                                                                    let t = c.g[S3][0] == c.g[V0][0];
                                                                                                    c.g[V0][0] = 0xb;
                                                                                                    if t {
break 'b_20d714;
} else {
c.cycles += 2;
let t = c.g[S3][0] != c.g[V0][0];
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
if t {
break 'b_20d71c;
} else {
break 'b_20d714;
}
}
                                                                                                }
                                                                                                c.cycles += 1;
                                                                                                c.g[S4][0] = 0x10;
                                                                                                break 'b_20d718;
                                                                                            }
                                                                                            c.cycles += 1;
                                                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                            break 'b_20d71c;
                                                                                        }
                                                                                        c.cycles +=
                                                                                            30;
                                                                                        c.g[A1]
                                                                                            [0] =
                                                                                            0u64;
                                                                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5bc8));
                                                                                        c.g[RA][0] = sx(0x0020d72c);
                                                                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                                                                                        call_ptr(c, 0x0014dd58, 0x0020d72c)?;
                                                                                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                        c.g[S0]
                                                                                            [0] = c
                                                                                            .g[V0]
                                                                                            [0];
                                                                                        c.g[A0]
                                                                                            [0] =
                                                                                            0u64;
                                                                                        c.g[RA][0] = sx(0x0020d740);
                                                                                        c.g[S2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc0));
                                                                                        call_ptr(c, 0x0014de68, 0x0020d740)?;
                                                                                        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x48));
                                                                                        c.g[RA][0] = sx(0x0020d74c);
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[S0]
                                                                                            [0];
                                                                                        call_ptr(c, 0x00144df0, 0x0020d74c)?;
                                                                                        c.g[A1]
                                                                                            [0] = c
                                                                                            .g[S3]
                                                                                            [0];
                                                                                        c.g[RA][0] = sx(0x0020d758);
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[S0]
                                                                                            [0];
                                                                                        call_ptr(c, 0x001451e8, 0x0020d758)?;
                                                                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[S4][0]);
                                                                                        c.g[V1]
                                                                                            [0] =
                                                                                            0x1;
                                                                                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f34), c.g[S4][0]);
                                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f38), c.g[V1][0]);
                                                                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                                                                        let t = c.g
                                                                                            [V1][0]
                                                                                            as u32;
                                                                                        c.g[RA][0] = sx(0x0020d77c);
                                                                                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                                                                        call_ptr(c, t, 0x0020d77c)?;
                                                                                        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x4a));
                                                                                        c.g[RA][0] = sx(0x0020d788);
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[S2]
                                                                                            [0];
                                                                                        call_ptr(c, 0x00238160, 0x0020d788)?;
                                                                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[A1]
                                                                                            [0] =
                                                                                            0x1;
                                                                                        if t {
                                                                                            break 'b_20db54;
                                                                                        } else {
                                                                                            break 'b_20d794;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                'b_20d7ec: {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        == 0u64;
                                                                                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                                                                                    if t {
                                                                                        break 'b_20d7ec;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            20;
                                                                                        c.g[V1][0] = 0x530000;
                                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66e8));
                                                                                        c.g[A1][0] = 0x440000;
                                                                                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                                                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x28f0));
                                                                                        c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
                                                                                        c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                                                                        c.g[T0]
                                                                                            [0] =
                                                                                            0x3;
                                                                                        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                                                                        c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x84));
                                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[T0][0]);
                                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                                                                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[A2][0]);
                                                                                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                                                                        c.g[RA][0] = sx(0x0020d7e4);
                                                                                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[V0][0]);
                                                                                        call_ptr(c, 0x002302a8, 0x0020d7e4)?;
                                                                                        let t =
                                                                                            true;
                                                                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                                                                        if t {
                                                                                            break 'b_20dbb4;
                                                                                        } else {
                                                                                            break 'b_20d7ec;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 16;
                                                                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                                                                c.g[RA][0] =
                                                                                    sx(0x0020d7fc);
                                                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                                                                                call_ptr(
                                                                                    c, 0x002706f0,
                                                                                    0x0020d7fc,
                                                                                )?;
                                                                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                c.g[RA][0] =
                                                                                    sx(0x0020d808);
                                                                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                                                                                call_ptr(
                                                                                    c, 0x00238348,
                                                                                    0x0020d808,
                                                                                )?;
                                                                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                c.g[A1][0] = 0xd;
                                                                                c.g[A2][0] = 0u64;
                                                                                c.g[A3][0] = 0x1;
                                                                                c.g[RA][0] =
                                                                                    sx(0x0020d820);
                                                                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                                                                call_ptr(
                                                                                    c, 0x00231250,
                                                                                    0x0020d820,
                                                                                )?;
                                                                                c.g[V0][0] = 0x3;
                                                                                let t = true;
                                                                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[V0][0]);
                                                                                if t {
                                                                                    break 'b_20dbb0;
                                                                                } else {
                                                                                    break 'b_20d82c;
                                                                                }
                                                                            }
                                                                            0x0020d82c => {
                                                                                break 'b_20d82c;
                                                                            }
                                                                            0x0020d8f4 => {
                                                                                break 'b_20d8f4;
                                                                            }
                                                                            0x0020d904 => {
                                                                                break 'b_20d904;
                                                                            }
                                                                            0x0020d954 => {
                                                                                break 'b_20d954;
                                                                            }
                                                                            0x0020da58 => {
                                                                                break 'b_20da58;
                                                                            }
                                                                            0x0020dab8 => {
                                                                                break 'b_20dab8;
                                                                            }
                                                                            _ => {
                                                                                c.jump_out(
                                                                                    t, 0x0020d590,
                                                                                    0x0020dbd0,
                                                                                );
                                                                                return Ok(());
                                                                            }
                                                                        }
                                                                    }
                                                                    'b_20d868: {
                                                                        c.cycles += 12;
                                                                        c.g[V0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x848));
                                                                        c.g[V1][0] = c.lw((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x84));
                                                                        c.g[RA][0] = sx(0x0020d83c);
                                                                        c.g[A0][0] = c.lw((c.g[V1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x28));
                                                                        call_ptr(
                                                                            c, 0x002706f0,
                                                                            0x0020d83c,
                                                                        )?;
                                                                        c.g[RA][0] = sx(0x0020d844);
                                                                        c.sw(
                                                                            (c.g[S1][0] as u32)
                                                                                .wrapping_add(0x54),
                                                                            0u64,
                                                                        );
                                                                        call_ptr(
                                                                            c, 0x0014dc80,
                                                                            0x0020d844,
                                                                        )?;
                                                                        c.g[A0][0] = c.g[V0][0];
                                                                        c.g[RA][0] = sx(0x0020d850);
                                                                        c.g[A1][0] = 0x4;
                                                                        call_ptr(
                                                                            c, 0x0014dd58,
                                                                            0x0020d850,
                                                                        )?;
                                                                        c.g[A0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x204));
                                                                        let t = c.g[A0][0] == 0u64;
                                                                        c.g[V0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x848));
                                                                        if t {
                                                                            break 'b_20d868;
                                                                        } else {
                                                                            c.cycles += 3;
                                                                            c.g[RA][0] =
                                                                                sx(0x0020d864);
                                                                            c.g[A1][0] = 0u64;
                                                                            call_ptr(
                                                                                c, 0x00257610,
                                                                                0x0020d864,
                                                                            )?;
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [GP][0]
                                                                                as u32)
                                                                                .wrapping_sub(
                                                                                    0x848,
                                                                                ));
                                                                            break 'b_20d868;
                                                                        }
                                                                    }
                                                                    c.cycles += 13;
                                                                    c.g[A1][0] = 0x7;
                                                                    c.g[S0][0] = 0u64;
                                                                    c.g[RA][0] = sx(0x0020d878);
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x78));
                                                                    call_ptr(
                                                                        c, 0x0014dd58, 0x0020d878,
                                                                    )?;
                                                                    c.g[RA][0] = sx(0x0020d880);
                                                                    call_ptr(
                                                                        c, 0x0014dc80, 0x0020d880,
                                                                    )?;
                                                                    c.g[A0][0] = c.g[V0][0];
                                                                    c.g[RA][0] = sx(0x0020d88c);
                                                                    c.g[A1][0] = 0u64;
                                                                    call_ptr(
                                                                        c, 0x0014dd58, 0x0020d88c,
                                                                    )?;
                                                                    c.g[V1][0] = 0x530000;
                                                                    c.g[V0][0] = c
                                                                        .lb((c.g[V1][0] as u32)
                                                                            .wrapping_add(0x5c11));
                                                                    let t = c.g[V0][0] != 0u64;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[GP][0] as u32)
                                                                            .wrapping_sub(0x848));
                                                                    if t {
                                                                        break 'b_20db50;
                                                                    } else {
                                                                        'b_20d8dc: {
                                                                            c.cycles += 2;
                                                                            let t =
                                                                                c.g[S0][0] == 0u64;
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [GP][0]
                                                                                as u32)
                                                                                .wrapping_sub(
                                                                                    0x848,
                                                                                ));
                                                                            if t {
                                                                                break 'b_20d8dc;
                                                                            } else {
                                                                                c.cycles += 14;
                                                                                c.g[A1][0] =
                                                                                    0x470000;
                                                                                c.g[A3][0] = 0u64;
                                                                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x640));
                                                                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                                                                                c.g[A0][0] = 0x234;
                                                                                c.g[A2][0] = 0x100;
                                                                                c.g[RA][0] =
                                                                                    sx(0x0020d8c4);
                                                                                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                                                                                call_ptr(
                                                                                    c, 0x00317d70,
                                                                                    0x0020d8c4,
                                                                                )?;
                                                                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                                                                c.g[A0][0] =
                                                                                    c.g[V0][0];
                                                                                c.g[RA][0] =
                                                                                    sx(0x0020d8d4);
                                                                                c.g[A2][0] = 0x1;
                                                                                Callee { addr: 0x00212138, f: ee::s05::OVScreenAutosave_ctor }.call(c, 0x0020d8d4)?;
                                                                                let t = true;
                                                                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                                                if t {
                                                                                    break 'b_20daf8;
                                                                                } else {
                                                                                    break 'b_20d8dc;
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 6;
                                                                        c.g[RA][0] = sx(0x0020d8e4);
                                                                        c.g[A0][0] = 0x26;
                                                                        Callee { addr: 0x0020ab50, f: ee::s05::GameUI_OpenScreen }.call(c, 0x0020d8e4)?;
                                                                        c.g[RA][0] = sx(0x0020d8ec);
                                                                        Callee { addr: 0x0020d110, f: ee::s05::OVScreenPDATemplate_Open }.call(c, 0x0020d8ec)?;
                                                                        let t = true;
                                                                        c.lq(
                                                                            16,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x50),
                                                                        );
                                                                        if t {
                                                                            break 'b_20dbb4;
                                                                        } else {
                                                                            break 'b_20d8f4;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 4;
                                                                c.g[RA][0] = sx(0x0020d8fc);
                                                                c.g[A0][0] = 0x1d;
                                                                Callee {
                                                                    addr: 0x0020ab50,
                                                                    f: ee::s05::GameUI_OpenScreen,
                                                                }
                                                                .call(c, 0x0020d8fc)?;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x50),
                                                                );
                                                                if t {
                                                                    break 'b_20dbb4;
                                                                } else {
                                                                    break 'b_20d904;
                                                                }
                                                            }
                                                            c.cycles += 20;
                                                            c.g[A1][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x69c));
                                                            c.g[V0][0] = 0x530000;
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x66e8));
                                                            c.g[A2][0] = 0x440000;
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_sub(0x1));
                                                            c.g[A2][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_add(0x28f0));
                                                            c.g[V1][0] =
                                                                sx((c.g[A1][0] as u32) << 2);
                                                            c.g[A3][0] = 0x3;
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0xce8));
                                                            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                                            c.sw(
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x9f4),
                                                                c.g[A3][0],
                                                            );
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.sw(
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x69c),
                                                                c.g[A1][0],
                                                            );
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(c.g[A2][0] as u32));
                                                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                                            c.g[RA][0] = sx(0x0020d94c);
                                                            c.sw(
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x6a0),
                                                                c.g[V1][0],
                                                            );
                                                            call_ptr(c, 0x0030b758, 0x0020d94c)?;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x50),
                                                            );
                                                            if t {
                                                                break 'b_20dbb4;
                                                            } else {
                                                                break 'b_20d954;
                                                            }
                                                        }
                                                        c.cycles += 65;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x84));
                                                        c.g[RA][0] = sx(0x0020d964);
                                                        c.g[A0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x28));
                                                        call_ptr(c, 0x002706f0, 0x0020d964)?;
                                                        c.g[V0][0] = 0x3;
                                                        c.g[RA][0] = sx(0x0020d970);
                                                        c.sw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x9f4),
                                                            c.g[V0][0],
                                                        );
                                                        call_ptr(c, 0x0014dc80, 0x0020d970)?;
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[RA][0] = sx(0x0020d97c);
                                                        c.g[A1][0] = 0u64;
                                                        call_ptr(c, 0x0014dd58, 0x0020d97c)?;
                                                        c.g[RA][0] = sx(0x0020d984);
                                                        c.g[S0][0] = c.g[V0][0];
                                                        call_ptr(c, 0x0014dc80, 0x0020d984)?;
                                                        c.g[A1][0] = 0xa;
                                                        c.g[RA][0] = sx(0x0020d990);
                                                        c.g[A0][0] = c.g[V0][0];
                                                        call_ptr(c, 0x0014dd58, 0x0020d990)?;
                                                        c.g[S1][0] = c.g[V0][0];
                                                        c.g[RA][0] = sx(0x0020d99c);
                                                        c.g[A0][0] = 0u64;
                                                        call_ptr(c, 0x0014de68, 0x0020d99c)?;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x0020d9a8);
                                                        c.g[A1][0] = 0xc;
                                                        call_ptr(c, 0x001451e8, 0x0020d9a8)?;
                                                        c.g[A1][0] = 0x4;
                                                        c.g[RA][0] = sx(0x0020d9b4);
                                                        c.g[A0][0] = c.g[S0][0];
                                                        call_ptr(c, 0x00144df0, 0x0020d9b4)?;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xc));
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.g[V0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0xc));
                                                        let t = c.g[V0][0] as u32;
                                                        c.g[RA][0] = sx(0x0020d9c8);
                                                        c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                            .wrapping_add(c.g[A0][0] as u32));
                                                        call_ptr(c, t, 0x0020d9c8)?;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        c.g[A1][0] = 0xc;
                                                        c.g[RA][0] = sx(0x0020d9d8);
                                                        c.g[A0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0xc0));
                                                        call_ptr(c, 0x00238160, 0x0020d9d8)?;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x84));
                                                        c.g[RA][0] = sx(0x0020d9e8);
                                                        c.g[A0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0xc));
                                                        call_ptr(c, 0x00128a10, 0x0020d9e8)?;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        c.g[V0][0] = 0x1;
                                                        c.sw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x10),
                                                            c.g[V0][0],
                                                        );
                                                        c.g[A3][0] = 0u64;
                                                        c.g[A1][0] = 0x1;
                                                        c.g[A2][0] = 0x1;
                                                        c.g[RA][0] = sx(0x0020da08);
                                                        c.g[A0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x84));
                                                        call_ptr(c, 0x00231250, 0x0020da08)?;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        c.g[RA][0] = sx(0x0020da14);
                                                        c.g[A0][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x84));
                                                        call_ptr(c, 0x00230180, 0x0020da14)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        c.g[A1][0] = 0x1;
                                                        c.g[A2][0] = 0x2;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x84));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xc));
                                                        c.g[A3][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[RA][0] = sx(0x0020da34);
                                                        c.g[A0][0] = c
                                                            .lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0x18));
                                                        call_ptr(c, 0x0011de60, 0x0020da34)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        c.g[A1][0] = 0x1;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x84));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xc));
                                                        c.g[A2][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[RA][0] = sx(0x0020da50);
                                                        c.g[A0][0] = c
                                                            .lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x18));
                                                        call_ptr(c, 0x0011df18, 0x0020da50)?;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x50),
                                                        );
                                                        if t {
                                                            break 'b_20dbb4;
                                                        } else {
                                                            break 'b_20da58;
                                                        }
                                                    }
                                                    c.cycles += 24;
                                                    c.g[A2][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x69c)
                                                        );
                                                    c.g[V1][0] = 0x530000;
                                                    c.g[A1][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32)
                                                            .wrapping_add(0x66e8));
                                                    c.g[A2][0] =
                                                        sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                                    c.g[A0][0] = 0x440000;
                                                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
                                                    c.g[T0][0] = c
                                                        .lw((c.g[A1][0] as u32).wrapping_add(0x84));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add(c.g[V1][0] as u32));
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x28f0));
                                                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                                    c.g[A1][0] = 0x3;
                                                    c.g[A3][0] =
                                                        c.lw((c.g[T0][0] as u32).wrapping_add(0xc));
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                    c.sw(
                                                        (c.g[GP][0] as u32).wrapping_sub(0x9f4),
                                                        c.g[A1][0],
                                                    );
                                                    c.g[V0][0] = c
                                                        .lw((c.g[A3][0] as u32).wrapping_add(0x40));
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                                                    c.g[A0][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x18));
                                                    c.sw(
                                                        (c.g[GP][0] as u32).wrapping_sub(0x6a0),
                                                        c.g[A1][0],
                                                    );
                                                    c.g[RA][0] = sx(0x0020dab0);
                                                    c.sw(
                                                        (c.g[GP][0] as u32).wrapping_sub(0x69c),
                                                        c.g[A2][0],
                                                    );
                                                    call_ptr(c, 0x001253d0, 0x0020dab0)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x50),
                                                    );
                                                    if t {
                                                        break 'b_20dbb4;
                                                    } else {
                                                        break 'b_20dab8;
                                                    }
                                                }
                                                c.cycles += 16;
                                                c.g[V1][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                c.g[A1][0] = 0x470000;
                                                c.sw((c.g[S1][0] as u32).wrapping_add(0x54), 0u64);
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_sub(0x640));
                                                c.g[A0][0] = 0x27c;
                                                c.g[A2][0] = 0x100;
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                                                c.g[A3][0] = 0u64;
                                                c.g[RA][0] = sx(0x0020dae0);
                                                c.g[S0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                                                call_ptr(c, 0x00317d70, 0x0020dae0)?;
                                                c.g[A2][0] =
                                                    c.lbu((c.g[S1][0] as u32).wrapping_add(0x14));
                                                c.g[A3][0] = 0x18;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                                c.g[RA][0] = sx(0x0020daf4);
                                                c.g[A0][0] = c.g[V0][0];
                                                Callee {
                                                    addr: 0x00211088,
                                                    f: ee::s05::OVScreenProfileSavePDA_ctor,
                                                }
                                                .call(c, 0x0020daf4)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                break 'b_20daf8;
                                            }
                                            c.cycles += 11;
                                            c.g[A3][0] = 0xffffffffffffc0ff;
                                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[V1][0] = c.g[V1][0] & c.g[A3][0];
                                            c.g[A2][0] = 0u64;
                                            c.g[V1][0] = c.g[V1][0] | 0x100;
                                            c.g[A3][0] = 0u64;
                                            c.g[RA][0] = sx(0x0020db1c);
                                            c.sw(
                                                (c.g[V0][0] as u32).wrapping_add(0x1c),
                                                c.g[V1][0],
                                            );
                                            call_ptr(c, 0x0039f320, 0x0020db1c)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            if t {
                                                break 'b_20dbb4;
                                            } else {
                                                break 'b_20db24;
                                            }
                                        }
                                    }
                                    'b_20db4c: {
                                        c.cycles += 4;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
                                        c.g[V0][0] = 0x6;
                                        if c.g[V1][0] == c.g[V0][0] {
                                            c.sw(
                                                (c.g[S1][0] as u32).wrapping_add(0x54),
                                                c.g[A1][0],
                                            );
                                            break 'b_20db4c;
                                        } else {
                                            c.cycles += 6;
                                            c.g[A0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                            c.g[A1][0] = 0x1;
                                            c.g[RA][0] = sx(0x0020db44);
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                            call_ptr(c, 0x0039f190, 0x0020db44)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            if t {
                                                break 'b_20dbb4;
                                            } else {
                                                break 'b_20db4c;
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    break 'b_20db50;
                                }
                                c.cycles += 1;
                                c.g[A1][0] = 0u64;
                                break 'b_20db54;
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x0020db5c);
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                            call_ptr(c, 0x00230488, 0x0020db5c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                            if t {
                                break 'b_20dbb4;
                            } else {
                                break 'b_20db64;
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[V0][0] == c.g[S0][0];
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                        if t {
                            break 'b_20dbb4;
                        } else {
                            'b_20dba0: {
                                c.cycles += 10;
                                c.g[A0][0] = 0x470000;
                                c.g[RA][0] = sx(0x0020db78);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fb0));
                                call_ptr(c, 0x00317670, 0x0020db78)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x0020db84);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039c778, 0x0020db84)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[V0][0] = 0xffff;
                                let t = c.g[A1][0] == c.g[V0][0];
                                c.g[A1][0] = c.g[A1][0] & 0xffff;
                                if t {
                                    break 'b_20dba0;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x0020dba0);
                                    c.g[A2][0] = 0x1;
                                    call_ptr(c, 0x0039c870, 0x0020dba0)?;
                                    break 'b_20dba0;
                                }
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                            c.g[A1][0] = 0x1;
                            c.g[RA][0] = sx(0x0020dbb0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            call_ptr(c, 0x0039f190, 0x0020dbb0)?;
                            break 'b_20dbb0;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_20dbb4;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_20dbb8;
        }
        c.cycles += 6;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `OVScreenPDATemplate_Init` (0x20a778): fills the PDA template screen `s1`
/// (its picture handle at +0x40, its list at +0x10) and lays out its rows,
/// each row sized by the template's widget at +0xb0 and the division of the
/// world's counter by 0xe10 and 0xf.
pub fn ovscreen_pda_template_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x29;
        frame.save(c, S1, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        frame.save(c, S0, 0x50);

        // Block A: the title picture; a non-null handle is shown.
        c.g[A0][0] = sx(0x0047_0000u32.wrapping_add(0x1a10));
        c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
        Addr((c.g[S1][0] as u32).wrapping_add(0xc)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7670, 0x0020_a7a4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0020_a7b8)?;
        let t = c.g[V0][0] == 0;
        Addr((c.g[S1][0] as u32).wrapping_add(0x40)).sw(c, c.g[V0][0]);
        if !t {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0020_a7d0)?;
        }

        // Block B: the subtitle picture.
        c.cycles += 11;
        c.g[A0][0] = 0x004a_0000;
        c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2718));
        call_ptr(c, 0x0031_7670, 0x0020_a7e0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0020_a7f4)?;
        let t = c.g[V0][0] == 0;
        c.g[A1][0] = 0;
        if !t {
            c.cycles += 3;
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = 1;
            call_ptr(c, 0x0039_c870, 0x0020_a808)?;
        }

        // Block C: the row list; each row's widget is resolved by the table and
        // laid out by its own method.
        c.cycles += 9;
        c.g[A0][0] = 0x0047_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a20));
        call_ptr(c, 0x0031_7670, 0x0020_a814)?;
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0020_a820)?;
        c.g[S0][0] = c.g[V0][0];
        let t = c.g[S0][0] == 0;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        if !t {
            c.cycles += 10;
            c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xb0)).lw(c);
            c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8c)).lw(c);
            c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x4)).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0020_a848)?;
            c.g[A1][0] = c.g[V0][0];
            let t = c.g[A1][0] == 0;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
            if !t {
                // Block 860: the row height.
                c.cycles += 3;
                c.g[V1][0] = 0xe10;
                c.cycles += 14;
                c.g[T0][0] = 0xf;
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1c)).lw(c);
                c.g[T1][0] = (-10i64) as u64;
                c.g[A0][0] = c.g[SP][0];
                c.div(0, 2, 3);
                c.g[A3][0] = 0x43;
                c.g[V0][0] = c.lo[0];
                c.div(0, 2, 8);
                c.g[A2][0] = c.hi[0];
                c.g[A2][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                call_ptr(c, 0x002c_26d0, 0x0020_a88c)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x003a_0e90, 0x0020_a898)?;
            }
        }

        c.cycles += 5;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    });
}

/// `GameUI_OpenPauseMenu` (0x20ca10): opens the pause menu's screens in
/// order: the game-state screen (6), the list screen (2 or 1 or the
/// selection-dependent screen), and the screens 3, 4 and 5 by the player's
/// state, leaving through the saved return address.
pub fn gameui_open_pause_menu(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        'outer: {
            'p1: {
                call_ptr(c, 0x0014_dc80, 0x0020_ca20)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x0014_dd58, 0x0020_ca2c)?;
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                c.g[A1][0] = 7;
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x78)).lw(c);
                call_ptr(c, 0x0014_dd58, 0x0020_ca3c)?;
                c.g[V1][0] = 0x0053_0000;
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x4b30)).lw(c);
                let t = c.g[V0][0] == 0;
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
                if t {
                    break 'p1;
                }
                'q: {
                    c.cycles += 2;
                    let t = c.g[A0][0] == 0;
                    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                    if t {
                        break 'q;
                    }
                    'r: {
                        c.cycles += 3;
                        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
                        let t = c.g[V0][0] == 0;
                        if t {
                            break 'r;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x224)).lw(c);
                        let t = c.g[V0][0] == 0;
                        if t {
                            break 'r;
                        }
                        c.cycles += 3;
                        call_ptr(c, 0x0025_8c00, 0x0020_ca74)?;
                        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
                    }
                    c.cycles += 7;
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x0025_8790, 0x0020_ca80)?;
                    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
                    c.g[A0][0] = 0xe10;
                    c.g[V1][0] = 0x708;
                    Addr((c.g[V0][0] as u32).wrapping_add(0x64)).sw(c, c.g[V1][0]);
                    Addr((c.g[V0][0] as u32).wrapping_add(0x68)).sw(c, c.g[A0][0]);
                }
                c.cycles += 4;
                c.g[A0][0] = 6;
                Callee {
                    addr: 0x0020_ab50,
                    f: ee::s05::f_0020ab50,
                }
                .call(c, 0x0020_ca9c)?;
                frame.restore_ra(c, 0);
                break 'outer;
            }

            // Screen 4 when the list at +0xce8 reports a selection.
            'sblk: {
                c.cycles += 4;
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xce8)).lw(c);
                call_ptr(c, 0x0030_b8c0, 0x0020_caac)?;
                let t = c.g[V0][0] == 0;
                c.g[V1][0] = 0x0053_0000;
                if t {
                    break 'sblk;
                }
                c.cycles += 4;
                c.g[A0][0] = 2;
                Callee {
                    addr: 0x0020_ab50,
                    f: ee::s05::f_0020ab50,
                }
                .call(c, 0x0020_cabc)?;
                frame.restore_ra(c, 0);
                break 'outer;
            }

            // Screens 0, 1 and 3 by the state of the record at 0x535bc8.
            'tblk: {
                'ublk: {
                    c.cycles += 5;
                    c.g[A2][0] = c.g[V1][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x49)).lb(c);
                    let t = c.g[V0][0] != 0;
                    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                    if t {
                        break 'ublk;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x48)).lb(c);
                    c.g[V0][0] ^= 4;
                    let t = c.g[V0][0] == 0;
                    c.g[A1][0] = 0xb;
                    if t {
                        break 'tblk;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = 1;
                    Callee {
                        addr: 0x0020_ab50,
                        f: ee::s05::f_0020ab50,
                    }
                    .call(c, 0x0020_caf0)?;
                    frame.restore_ra(c, 0);
                    break 'outer;
                }
                c.cycles += 1;
                c.g[A1][0] = 0xb;
            }

            // Screen 4 when the state record's word at +0x214 differs.
            'wblk: {
                c.cycles += 4;
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x84)).lw(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x214)).lw(c);
                let t = c.g[V1][0] != c.g[A1][0];
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5bc8));
                if t {
                    break 'wblk;
                }
                c.cycles += 4;
                c.g[A0][0] = 4;
                Callee {
                    addr: 0x0020_ab50,
                    f: ee::s05::f_0020ab50,
                }
                .call(c, 0x0020_cb14)?;
                frame.restore_ra(c, 0);
                break 'outer;
            }

            // Screen 3 when the record's byte +0x48 is 4.
            'xblk: {
                c.cycles += 4;
                c.g[A0][0] = 4;
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lb(c);
                let t = c.g[V1][0] != c.g[A0][0];
                if t {
                    break 'xblk;
                }
                c.cycles += 4;
                c.g[A0][0] = 3;
                Callee {
                    addr: 0x0020_ab50,
                    f: ee::s05::f_0020ab50,
                }
                .call(c, 0x0020_cb34)?;
                frame.restore_ra(c, 0);
                break 'outer;
            }
            c.cycles += 3;
            c.g[A0][0] = 5;
            Callee {
                addr: 0x0020_ab50,
                f: ee::s05::f_0020ab50,
            }
            .call(c, 0x0020_cb44)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 3;
        Addr((c.g[GP][0] as u32).wrapping_sub(0xc88)).sb(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `OVScreenYNDialogQuitGame_Update` (0x20dbd0): per-frame update of the quit
/// confirmation dialog. Updates the screen, then (while the global dialog
/// pointer at gp-0x204 is set and its flags allow it) refreshes the dialog's
/// text and its widget at +0x10.
pub fn ovscreen_yn_dialog_quit_game_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0020_e900,
            f: ee::s05::f_0020e900,
        }
        .call(c, 0x0020_dbe4)?;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
        if c.g[V1][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            'dc0c: {
                'dc08: {
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x68)).lw(c);
                    let t = c.g[V0][0] == 0;
                    c.g[A0][0] = 0;
                    if t {
                        break 'dc08;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x64)).lw(c);
                    let t = c.g[V0][0] != 0;
                    if t {
                        break 'dc0c;
                    }
                }
                c.cycles += 1;
                c.g[A0][0] = 1;
            }
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                frame.restore(c, S0, 0x10);
            } else {
                'dc48: {
                    c.cycles += 10;
                    c.g[A0][0] = sx((0x0047_0000u32).wrapping_sub(0x1fb0));
                    call_ptr(c, 0x0031_7670, 0x0020_dc20)?;
                    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_c778, 0x0020_dc2c)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = 0xffff;
                    let t = c.g[A1][0] == c.g[V0][0];
                    c.g[A1][0] &= 0xffff;
                    if t {
                        break 'dc48;
                    }
                    c.cycles += 3;
                    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
                    c.g[RA][0] = sx(0x0020_dc48);
                    c.g[A2][0] = 1;
                    call_ptr(c, 0x0039_c870, 0x0020_dc48)?;
                }
                c.cycles += 5;
                c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x10)).lw(c);
                c.g[A1][0] = 1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039_f190, 0x0020_dc58)?;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------- OVScreenReplayMenu_Init

/// The replay screen object (`s1`) as an address.
pub(crate) fn replay_init_this(c: &Cpu) -> u32 {
    c.g[S1][0] as u32
}

/// `GP - off` (the game's globals sit below `gp`).
pub(crate) fn replay_init_gp(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

/// `void cOVState_REPLAY::onCreateScreen(cOVState_REPLAY* this)` (0x20dc68).
/// Resolves the screen's two UI hash names (`0x471a40`, `0x471a40` + the
/// replay tables), creates the replay screen with `cUIEngine_addScreenByHashName`
/// and plays its frame, then sets up the camera name, hides or shows the
/// replay's objects by hash name, and sets the state words at +0x9c..+0xb4
/// from the world's replay record (`world+0x84`, `+0x28`, flag `+0x608`).
pub fn ov_screen_replay_menu_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0x470000;
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1a40));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_7670, 0x0020_dc8c)?;
        c.g[A0][0] = c.g[S0][0];
        let name_hash = c.g[V0][0];
        Addr(replay_init_this(c).wrapping_add(0xc)).sw(c, name_hash);
        c.g[S0][0] = Addr(replay_init_this(c).wrapping_add(0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0020_dc9c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0020_dcb0)?;
        let screen = c.g[V0][0];
        Addr(replay_init_this(c).wrapping_add(0x40)).sw(c, screen);
        if screen != 0 {
            c.cycles += 4;
            c.g[A0][0] = screen;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0020_dcc8)?;
        }

        // Camera name, then the objects that show and hide with the screen.
        c.cycles += 15;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0020_a380, 0x0020_dcd0)?;
        Addr(replay_init_this(c).wrapping_add(0xa0)).sw(c, 0);
        c.g[A1][0] = 0x470000;
        Addr(replay_init_this(c).wrapping_add(0x9c)).sw(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1ba0));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0039_e8b8, 0x0020_dce8)?;
        c.g[A1][0] = 0x470000;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1bb0));
        call_ptr(c, 0x0039_e8b8, 0x0020_dcf8)?;
        let flag = Addr(replay_init_this(c).wrapping_add(0xa0)).lw(c);
        c.g[V0][0] = flag;
        c.g[A0][0] = c.g[S1][0];
        let shown_last = if flag == 0 {
            true
        } else {
            c.cycles += 9;
            c.g[A1][0] = 0x470000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1bc0));
            call_ptr(c, 0x0039_e8b8, 0x0020_dd10)?;
            c.g[A1][0] = 0x470000;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1bd0));
            call_ptr(c, 0x0039_e948, 0x0020_dd20)?;
            c.g[V0][0] = 0xf5;
            false
        };
        if shown_last {
            c.cycles += 8;
            c.g[A1][0] = 0x470000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1bc0));
            call_ptr(c, 0x0039_e948, 0x0020_dd34)?;
            c.g[A1][0] = 0x470000;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1bd0));
            call_ptr(c, 0x0039_e8b8, 0x0020_dd44)?;
            c.g[V0][0] = 0xf5;
        }

        // The replay state words.
        c.cycles += 13;
        c.g[V1][0] = 0x1e0;
        Addr(replay_init_this(c).wrapping_add(0xa8)).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        let v0 = c.g[V0][0];
        Addr(replay_init_this(c).wrapping_add(0xac)).sw(c, v0);
        let v1 = c.g[V1][0];
        Addr(replay_init_this(c).wrapping_add(0xb0)).sw(c, v1);
        Addr(replay_init_this(c).wrapping_add(0xa4)).sw(c, v0);
        call_ptr(c, 0x0020_e6b0, 0x0020_dd64)?;
        c.g[V0][0] = Addr(replay_init_gp(c, 0x848)).lw(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x28)).lw(c);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x608)).lw(c);
        let no_flag = c.g[V0][0] == 0;
        c.g[V0][0] = 0x1;
        if no_flag {
            c.cycles += 1;
            Addr(replay_init_this(c).wrapping_add(0xb4)).sw(c, 0);
        } else {
            c.cycles += 2;
            let v = c.g[V0][0];
            Addr(replay_init_this(c).wrapping_add(0xb4)).sw(c, v);
        }

        // The pending replay's object: its flags get bit 3 set.
        c.cycles += 13;
        c.g[A0][0] = 0x470000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1be0));
        call_ptr(c, 0x0031_7670, 0x0020_dd94)?;
        c.g[A0][0] = Addr(replay_init_this(c).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0020_dda0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x90)).lw(c);
        c.g[V0][0] = c.g[V0][0] | 0x8;
        let flags = c.g[V0][0];
        Addr((c.g[A0][0] as u32).wrapping_add(0x90)).sw(c, flags);
        c.g[V1][0] = Addr(replay_init_this(c).wrapping_add(0xb4)).lw(c);
        let no_state = c.g[V1][0] == 0;
        frame.restore(c, S0, 0x20);
        if !no_state {
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                c.g[A0][0] = 0x470000;
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0x2;
                call_ptr(c, 0x0039_b960, 0x0020_ddcc)?;
                c.g[A0][0] = 0x470000;
            }

            // Look up the second object and, if it exists, run its two
            // vtable methods (the replay's ticker and its end).
            c.cycles += 8;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1bf0));
            call_ptr(c, 0x0031_7670, 0x0020_ddd8)?;
            c.g[A0][0] = Addr(replay_init_this(c).wrapping_add(0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0020_dde4)?;
            c.g[S0][0] = c.g[V0][0];
            let no_object = c.g[S0][0] == 0;
            c.g[A1][0] = 0;
            if !no_object {
                c.cycles += 11;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
                let method = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method, 0x0020_de04)?;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[A1][0] = 0;
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x40)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x44)).lw(c);
                let method2 = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method2, 0x0020_de1c)?;
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

/// Vtable word of the `cOVState` family: `0x470000 + 0x4e08` (`0x474e08`), the
/// value the destructors and the overlay-menu constructors store at `+0x08`.
pub(crate) const OV_STATE_VTBL_LO: u32 = 0x0047_0000;

pub(crate) const OV_STATE_VTBL_BASE_OFF: u32 = 0x4e08;

/// Shared body of the four `cOVState` destructor wrappers (0x21c708, 0x21c798,
/// 0x21c848, 0x21c870): stores the base vtable at `this+0x08` and tail-calls the
/// UI state destructor (`UIState_dtor`, 0x39e390) returning to `ret`. The
/// callee's unwind returns without restoring the frame, as the generated code.
pub(crate) fn oc998_ov_state_dtor_body(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = OV_STATE_VTBL_LO as u64;
    frame.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(OV_STATE_VTBL_BASE_OFF));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    call_ptr(c, 0x0039_e390, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// Shared body of the three overlay-state constructors (0x21c730, 0x21c7c0,
/// 0x21c898). Runs `UIState_ctor` (0x39e2a0) on `this` (a0) returning to
/// `ui_ret`, then sets the overlay's owner word (`+0x98` = a2, the overlay id),
/// clears the `+0x48`, `+0x4c` and `+0x54` words, stores the vtable word at
/// `+0x08` (`vtbl_off` past `0x470000`, the last store wins) and the chat
/// buffer's first halfword (`str_sp_b[0]` at `0x4a2088`) at `+0x58`.
pub(crate) fn oc998_ov_state_ctor_body(
    c: &mut Cpu,
    ui_ret: u32,
    vtbl_off: u32,
) -> crate::call::Flow {
    c.cycles += 26;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A2][0];
    call_ptr(c, 0x0039_e2a0, ui_ret)?;
    c.g[V1][0] = OV_STATE_VTBL_LO as u64;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(OV_STATE_VTBL_BASE_OFF));
    c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
    c.g[A1][0] = 0x004a_0000;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
    c.g[V1][0] = OV_STATE_VTBL_LO as u64;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(vtbl_off));
    c.g[V0][0] = c.g[S0][0];
    frame.restore(c, S1, 0x10);
    c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
    c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
    frame.restore(c, S0, 0x20);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cOVTemplate_Simple` destructor for the overlay menu (0x21c708): base
/// `cOVState` vtable, then `UIState_dtor`.
pub fn ov_screen_overlay_menu_dtor(c: &mut Cpu) {
    guest(c, |c| oc998_ov_state_dtor_body(c, 0x0021_c724));
}

/// `cOVTemplate_Simple` constructor for the overlay menu (0x21c730): runs
/// `UIState_ctor`, then sets the overlay state (see `oc998_ov_state_ctor_body`) with
/// the overlay-menu vtable (`0x474d30`).
pub fn ov_screen_overlay_menu_ctor(c: &mut Cpu) {
    guest(c, |c| oc998_ov_state_ctor_body(c, 0x0021_c74c, 0x4d30));
}

/// `cOVTemplate_Dialog` destructor for the yes/no dialog (0x21c798).
pub fn ov_screen_yn_dialog_dtor(c: &mut Cpu) {
    guest(c, |c| oc998_ov_state_dtor_body(c, 0x0021_c7b4));
}

/// `cOVTemplate_Dialog` constructor for the yes/no dialog (0x21c7c0): the
/// shared overlay-state constructor with the dialog vtable (`0x474c58`).
pub fn ov_screen_yn_dialog_ctor(c: &mut Cpu) {
    guest(c, |c| oc998_ov_state_ctor_body(c, 0x0021_c7dc, 0x4c58));
}

/// `cOVTemplate_Dialog_onUpdate` (0x21c828): runs the overlay state's update
/// (`cOVStateOverlay_Update`, 0x20e900) and returns.
pub fn ov_screen_yn_dialog_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee {
            addr: 0x0020_e900,
            f: ee::s05::f_0020e900,
        }
        .call(c, 0x0021_c838)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `OVScreenYNDialogQuitGame_dtor` (0x21c928).
pub fn ov_screen_yn_dialog_quit_game_dtor(c: &mut Cpu) {
    guest(c, |c| oc999_yn_dtor(c, 0x0021_c944));
}

/// `OVScreenYNDialogQuitGame` constructor (0x21c950): `this` (a0), overlay id
/// (a2). Runs the base `UIState` constructor (0x39e2a0) with the arguments
/// unchanged, then stores the overlay id at +0x98, clears the transition and
/// pause-menu flags (+0x48, +0x4c) and the PDA flag (+0x54), sets the vtable to
/// the quit-game variant (0x4748f8), copies the first halfword of the chat
/// string (0x4a2088) to +0x58 and the dialog type 1 to +0x9c. Returns `this`.
pub fn ov_screen_yn_dialog_quit_game_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x0021_c96c)?;
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x98), c.g[S1][0]);
        c.g[V1][0] = 0x0047_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4e08));
        c.sw(this.wrapping_add(0x48), 0u64);
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.g[A2][0] = 0x004a_0000;
        c.sw(this.wrapping_add(0x4c), 0u64);
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x54), 0u64);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x48f8));
        c.g[A1][0] = 0x1;
        c.g[V0][0] = c.g[S0][0];
        c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sh(this.wrapping_add(0x58), c.g[A0][0]);
        c.sw(this.wrapping_add(0x9c), c.g[A1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// The front-end screens and states below derive from the UI state base:
// `UIState` constructor at 0x39e2a0 and destructor at 0x39e390, both reached
// through the function table (`call_ptr`). Their vtables sit in the game
// image at 0x47xxxx (the `+8` word of each object).

/// `void OVScreenPDATemplate_dtor(OVScreenPDATemplate *self)` (0x21ca88): runs
/// the UIState destructor on `self`.
pub fn ov_screen_pda_template_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0039_e390, 0x0021_ca98)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenPDATemplate *OVScreenPDATemplate_ctor(OVScreenPDATemplate *self)`
/// (0x21caa8): runs the UIState constructor, then sets the screen's vtable
/// (0x4743f0) at `+8`. Returns `self` in `v0`.
pub fn ov_screen_pda_template_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e2a0, 0x0021_cabc)?;
        c.g[V1][0] = sx(0x0047_43f0);
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::fe::ovstatemap::tests::oc992_install_fakes;
    pub(crate) use crate::game::main::game::tests::oc804_install_fakes;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc774_install_fakes;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::tests::oc784_install_fakes;

    use crate::difftest::fakes;

    use crate::difftest::fakes::machine;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;

    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_2 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::VT;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_6 as WORLD;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_2;

    pub(crate) use crate::game::fe::festaterewards::tests::w as w_4;

    pub(crate) use crate::game::fe::ovstatemap::tests::str_hash;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;

    pub(crate) use crate::game::movie::movieplayer::tests::w as w_3;

    pub(crate) use crate::game::movie::movieplayer::tests::W;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn game_ui_get_results_screen_id_matches() {
        check(
            ssx3_ee::s05::f_0020a8b0,
            game_ui_get_results_screen_id,
            400,
            &|c, seed| {
                let mut r = Rng(0x0c10_0011 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = match seed % 6 {
                    0 => 6 + (seed as u64 % 6),
                    1 => 4 + (seed as u64 % 2),
                    2 => 0,
                    3 => 2,
                    4 => (1u64 << 32) | 2,
                    _ => (r.next_u32() % 16) as u64,
                };
                c.g[A1][0] = if seed % 2 == 0 {
                    3
                } else {
                    (r.next_u32() % 5) as u64
                };
            },
        );
    }

    #[test]
    fn ustr_sprintf_matches() {
        check(
            ee::s08::f_002c26d0__generated,
            ustr_sprintf,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c_26d0, 0);
                let dst = DATA + 0x1000;
                let fmt = DATA + 0x2000;
                let spec = [0x25u16, 0x64, 0x20, 0x25, 0x69, 0x0a, 0];
                for (i, w) in spec.iter().enumerate() {
                    c.bus.write16(fmt + 2 * i as u32, *w);
                }
                c.g[A0][0] = sx(dst);
                c.g[A1][0] = sx(fmt);
                c.g[A2][0] = (r.next_u32() % 0x1000) as u64;
                c.g[A3][0] = (r.next_u32() % 0x1000) as u64;
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn ov_screen_overlay_menu_on_widget_action_matches() {
        check(
            ssx3_ee::s05::f_0020d318__generated,
            ov_screen_overlay_menu_on_widget_action,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x20d318, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x10) };
                c.g[A2][0] = if seed % 2 == 0 { 5 } else { (seed % 8) as u64 };
            },
        );
    }

    /// Tiny generated leaves used as deterministic vtable targets.
    pub(crate) const ZERO_STUB: u32 = 0x0020_2768;
    // `EventSelect_HandleInput`: returns 0
    pub(crate) const ONE_STUB: u32 = 0x0021_cb68;

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
    fn game_ui_can_open_pause_menu_matches() {
        check(
            ee::s05::f_0020cbe8__generated,
            game_ui_can_open_pause_menu,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x20cbe8, 5);
                be_library(c);
                heap_init(c);
                world(c);
                let race = DATA + 0x1200;
                if seed % 2 == 0 {
                    w_2(c, race + 0x48, 0);
                } else {
                    w_2(c, race + 0x48, DATA + 0x400);
                }
            },
        );
    }

    #[test]
    fn ov_screen_yn_dialog_on_widget_created_matches() {
        check(
            ee::s05::f_0020d4f0__generated,
            ov_screen_yn_dialog_on_widget_created,
            SEEDS_2,
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
                w_2(c, widget + 0x38, want);
                w_2(c, this + 0x9c, if seed % 3 == 0 { 6 } else { 5 });
            },
        );
    }

    #[test]
    fn ov_screen_pda_template_open_matches() {
        check(
            ee::s05::f_0020d110__generated,
            ov_screen_pda_template_open,
            8,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x20d1_10, 4);
                w_3(
                    c,
                    GAME_GP.wrapping_sub(0x9e4),
                    if seed % 2 == 0 { 0 } else { DATA + 0x1000 },
                );
                let game = W + 0x1000;
                w_3(c, game + 0x48, DATA + 0x2000);
                c.g[A0][0] = (seed + 1) as u64;
            },
        );
    }

    #[test]
    fn game_ui_open_finish_screen_matches() {
        check(
            ee::s05::f_0020cc60__generated,
            game_ui_open_finish_screen,
            60,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x20cc60, 4);
                w_4(c, 0x0053_5bc8 + 0x48, (seed % 8) as u32);
                w_4(c, 0x0053_5bc8 + 0x49, (seed / 8 % 4) as u32);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn cov_state_overlay_init_matches() {
        check(
            ee::s05::f_0020a380__generated,
            cov_state_overlay_init,
            6,
            &|c, seed| {
                let _ = base_3(c, seed, 0x0020_a380, 3);
                c.g[A0][0] = OBJ_2 as u64;
                w_5(c, OBJ_2 + 0x50, 0);
                w_5(c, OBJ_2 + 0x10, OBJ2_2);
                let game = WORLD + 0x1000;
                w_5(c, game + 0x64, if seed % 3 == 0 { 0 } else { OBJ3_2 });
                w_5(
                    c,
                    OBJ3_2 + 0x10,
                    match seed % 4 {
                        0 => 1,
                        1 => 3,
                        _ => OBJ4_2,
                    },
                );
                w_5(c, OBJ2_2 + 0x18, 0);
            },
        );
    }

    /// The world -> game UI -> root -> state manager chain and a manager
    /// whose two lists (background, stack) hold 0..3 states; one of them is
    /// the pause menu (id = the hash of its name) on some seeds. The hash and
    /// the search are real.
    #[test]
    fn game_ui_is_pause_menu_open_matches() {
        use crate::difftest::{check, elf_image, GAME_GP};
        check(
            ee::s05::f_0020cba0__generated,
            game_ui_is_pause_menu_open,
            200,
            &|c, seed| {
                elf_image(c);
                let mut r = fakes::machine(c, seed, 0x20_cba0, 0);
                let world = DATA + 0x100;
                let ui = DATA + 0x200;
                let root = DATA + 0x300;
                let mgr = root + PAUSE_ROOT_STATE_MGR;
                c.bus.write32(GAME_GP - PAUSE_WORLD_GP, world);
                c.bus.write32(world + PAUSE_WORLD_UI, ui);
                c.bus.write32(ui + PAUSE_UI_ROOT, root);
                let name: Vec<u8> = (0..64)
                    .map(|i| c.bus.read8(PAUSE_STATE_NAME + i))
                    .take_while(|&b| b != 0)
                    .collect();
                let pause = crate::game::hashvalue::str_hash(&name);
                let open = seed % 3 == 0;
                // Background list at mgr+0, stack at mgr+0x1c (anchor link at +4),
                // states 0x40 apart, each list ending at a self-linked sentinel.
                for (k, (list, base)) in [(mgr, DATA + 0x1000), (mgr + 0x1c, DATA + 0x2000)]
                    .into_iter()
                    .enumerate()
                {
                    let n = r.next_u32() % 4;
                    let mut prev = list;
                    for i in 0..n {
                        let s = base + 0x40 * i;
                        let id = if open && k as u32 == seed / 3 % 2 && i + 1 == n {
                            pause
                        } else {
                            r.next_u32() % 6
                        };
                        c.bus.write32(s + 0xc, id);
                        c.bus.write32(prev + 4, s);
                        c.bus.write32(s, prev);
                        prev = s;
                    }
                    let sentinel = base + 0x800;
                    c.bus.write32(prev + 4, sentinel);
                    c.bus.write32(sentinel, prev);
                    c.bus.write32(sentinel + 4, sentinel);
                }
                c.pc = 0x0020_cba0;
            },
        );
    }

    use crate::difftest::fakes::FAKE;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot as slot_m2;

    #[test]
    fn ov_screen_pda_template_close_matches() {
        check(
            ee::s05::f_0020d190__generated,
            ov_screen_pda_template_close,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x20d1_90, 2);
                let template = if seed % 3 == 0 { 0 } else { DATA + 0x100 };
                c.bus.write32(GAME_GP - 0x9e4, template);
                if template != 0 {
                    let vt = DATA + 0x800;
                    c.bus.write32(template + 8, vt);
                    slot_m2(c, &mut r, vt, 8, FAKE + 0x10);
                }
            },
        );
    }

    use crate::difftest::elf_image;
    use crate::difftest::fakes::covered;
    use crate::difftest::fill;

    pub(crate) use crate::game::ai::motion::wipeoutmotion::tests::OC360_CTRL;
    pub(crate) use crate::game::ai::motion::wipeoutmotion::tests::OC360_MAGIC;
    use std::sync::atomic::AtomicUsize;

    pub(crate) use crate::game::ai::motion::wipeoutmotion::tests::oc360_fake;
    pub(crate) use crate::game::ai::motion::wipeoutmotion::tests::oc360_gated;
    pub(crate) use crate::game::ai::motion::wipeoutmotion::tests::OC360_ACTIVE_STATE;

    // ------------------------------------------- GameUI_Update

    oc360_gated!(
        oc360_fake_ui_active_state,
        OC360_PREV_UI_ACTIVE_STATE,
        0x0039_fa88,
        ee::s13::f_0039fa88__generated
    );

    oc360_gated!(
        oc360_fake_replay_finish,
        OC360_PREV_REPLAY_FINISH,
        0x0027_06f0,
        ee::s07::f_002706f0__generated
    );

    oc360_gated!(
        oc360_fake_be_get_interface,
        OC360_PREV_BE_GET_INTERFACE,
        0x0014_dd58,
        ee::s01::f_0014dd58__generated
    );

    oc360_gated!(
        oc360_fake_be_get_instance,
        OC360_PREV_BE_GET_INSTANCE,
        0x0014_dc80,
        ee::s01::f_0014dc80__generated
    );

    oc360_gated!(
        oc360_fake_ui_close_all,
        OC360_PREV_UI_CLOSE_ALL,
        0x0039_f840,
        ee::s13::f_0039f840__generated
    );

    oc360_gated!(
        oc360_fake_pda_close,
        OC360_PREV_PDA_CLOSE,
        0x0020_d190,
        ee::s05::f_0020d190__generated
    );

    oc360_gated!(
        oc360_fake_request_state,
        OC360_PREV_REQUEST_STATE,
        0x0023_1250,
        ee::s06::f_00231250__generated
    );

    pub(crate) fn oc360_install_ui_fakes() {
        use std::ptr::addr_of_mut;
        {
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_0039FA88),
                oc360_fake_ui_active_state,
                &OC360_PREV_UI_ACTIVE_STATE,
            );
            fakes::hook(
                addr_of_mut!(ee::s07::OVERRIDE_002706F0),
                oc360_fake_replay_finish,
                &OC360_PREV_REPLAY_FINISH,
            );
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
                oc360_fake_be_get_interface,
                &OC360_PREV_BE_GET_INTERFACE,
            );
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
                oc360_fake_be_get_instance,
                &OC360_PREV_BE_GET_INSTANCE,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_0039F840),
                oc360_fake_ui_close_all,
                &OC360_PREV_UI_CLOSE_ALL,
            );
            fakes::hook(
                addr_of_mut!(ee::s05::OVERRIDE_0020D190),
                oc360_fake_pda_close,
                &OC360_PREV_PDA_CLOSE,
            );
            fakes::hook(
                addr_of_mut!(ee::s06::OVERRIDE_00231250),
                oc360_fake_request_state,
                &OC360_PREV_REQUEST_STATE,
            );
        }
    }

    /// Requests 0-6 (5 and 6 are outside the table); the UI stack at 0
    /// (no state lookup) or not; an active state or none, its id 8 or
    /// not; game mode 2 or not; the close flag at 0x534b30 set or not.
    /// Request 1 runs the real `GameUI_HandleMenuResult` on its short path
    /// (the state's id outside 8..0x23).
    #[test]
    fn game_ui_update_matches_2() {
        oc360_install_ui_fakes();
        covered(
            ee::s05::f_0020cfd0__generated,
            game_ui_update_2,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0020_cfd0, 4);
                c.pc = 0x0020_cfd0;
                let world = DATA + 0xa_0000;
                let game = world + 0x100;
                let mgr = world + 0x400;
                let flags = world + 0x500;
                let state = world + 0x600;
                let ptrs = world + 0x700;
                c.bus.write32(OC360_CTRL, OC360_MAGIC);
                fill(c, &mut r, world, 0x200);
                let request = seed % 7;
                c.bus.write32(GAME_GP - GUU_REQUEST, request);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + GUU_GAME, game);
                c.bus.write32(world + 0xc0, ptrs);
                let mgr_word = if r.next_u32() % 5 == 0 {
                    0u32.wrapping_sub(GUU_UI_STACK)
                } else {
                    mgr
                };
                c.bus.write32(game + GUU_UI_MGR, mgr_word);
                c.bus.write32(game + 0x200, flags);
                c.bus.write32(game + 0x214, 1 + r.next_u32() % 2);
                let active = if request != 1 && r.next_u32() % 4 == 0 {
                    0
                } else {
                    state
                };
                c.bus.write32(OC360_ACTIVE_STATE, active);
                let id = if request == 1 {
                    [3, 0x40][r.next_u32() as usize % 2]
                } else {
                    [8, 8, 3][r.next_u32() as usize % 3]
                };
                c.bus.write32(state + 0x98, id);
                c.bus.write32(0x0053_4b30, r.next_u32() % 2);
            },
        );
    }

    pub(crate) use crate::game::fe::festateloadscreen::tests::oc444_run;

    pub(crate) use crate::game::fe::festateloadscreen::tests::OC444_GM;

    // ------------------------------------------- GameUI_OpenScreenForState.

    /// `GameUI_OpenScreenForState(mode)`: modes 2 and 7 and an unknown one,
    /// with a replay running or not.
    #[test]
    fn game_ui_open_screen_for_state_matches() {
        oc444_run(
            ee::s05::f_0020a8f8__generated,
            game_ui_open_screen_for_state,
            0x0020_a8f8,
            0x20_a8f8,
            150,
            &|c, r, seed| {
                c.g[A0][0] = [2u32, 7, 7, 2, 3][seed as usize % 5] as u64;
                c.bus.write32(OC444_GM, r.next_u32() % 4);
            },
        );
    }

    /// `gp-0x848` -> world; `world+0x84` -> module; `module+0x48` = `state - 0x18`,
    /// so the replay code's `S0` is `state`.
    pub(crate) fn setup_world(c: &mut Cpu, state: u32) {
        let world = DATA + 0x1000;
        let module = DATA + 0x2000;
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP - 0x848, world);
        c.bus.write32(world + 0x84, module);
        c.bus.write32(module + 0x48, state.wrapping_sub(0x18));
    }

    #[test]
    fn game_ui_is_replay_menu_open_matches() {
        check(
            ee::s05::f_0020cb58__generated,
            game_ui_is_replay_menu_open,
            200,
            &|c, seed| {
                let mut r = Rng(0x0020_cb58 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0020_cb58;
                setup_world(c, DATA + 0x3000);
                // The stack is left empty: findState returns 0 on both sides.
                let _ = seed;
            },
        );
    }

    pub(crate) use crate::game::fe::festatecharsetup::tests::oc774_arm;
    pub(crate) use crate::game::fe::festatecharsetup::tests::oc774_force;
    pub(crate) use crate::game::fe::festatecharsetup::tests::oc774_force_zero;
    pub(crate) use crate::game::fe::festatecharsetup::tests::OC774_MOD;
    pub(crate) use crate::game::fe::festatecharsetup::tests::OC774_UI;
    pub(crate) use crate::game::fe::festatecharsetup::tests::OC774_WORLD;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::oc784_arm;

    pub(crate) use crate::game::fe::festatetrophyroom::tests::OC784_OBJ;

    pub(crate) const OC774_STATE: u32 = DATA + 0x3_0000;

    pub(crate) const OC774_REWARD: u32 = DATA + 0x3_1000;

    /// `GameUI_HandleMenuResult`: every UI state id the table distinguishes
    /// (8..35 plus out-of-range ones) with every `mMiscData` (0..8, 19, an
    /// out-of-range one), reward interface with and without awards, network
    /// game or not; `GameUI_OpenScreen` and `0x257610` are the real functions.
    #[test]
    fn game_ui_handle_menu_result_matches() {
        oc774_install_fakes();
        fakes::covered(
            ee::s05::f_0020ccf8__generated,
            game_ui_handle_menu_result,
            600,
            &|c, seed| {
                let mut r = machine(c, seed, 0x774d, 3);
                elf_image(c);
                oc774_arm(c);
                c.pc = 0x0020_ccf8;
                let state = [8u32, 9, 10, 11, 15, 16, 20, 21, 22, 35, 12, 40][(seed % 12) as usize];
                let misc = [0u32, 1, 2, 3, 4, 5, 6, 7, 8, 19, 9, 10][((seed / 12) % 12) as usize];
                c.bus.write32(GAME_GP - 0x848, OC774_WORLD);
                c.bus.write32(OC774_WORLD + 0x84, OC774_MOD);
                c.bus.write32(OC774_WORLD + 0xc0, OC774_UI);
                c.bus.write32(OC774_MOD + 0x48, OC774_UI);
                c.bus.write32(OC774_STATE + 0x98, state);
                c.bus.write32(GAME_GP - 0x9f0, misc);
                c.bus.write8(0x0053_5c12, r.next_u32() as u8);
                c.bus.write32(0x0053_4b30, ((seed / 144) % 2) as u32);
                c.bus.write32(
                    GAME_GP - 0x204,
                    if (seed / 288) % 2 == 0 { 0 } else { OC774_UI },
                );
                oc774_force(c, 0x0039_fa88, OC774_STATE);
                oc774_force(c, 0x0014_dd58, OC774_REWARD);
                c.bus.write32(OC774_REWARD + 0x10, ((seed / 7) % 2) as u32);
                if (seed / 5) % 2 == 0 {
                    oc774_force_zero(c, 0x0015_8f30);
                } else {
                    oc774_force(c, 0x0015_8f30, 2);
                }
                oc774_force(c, 0x0020_a8b0, [1u32, 2, 3, 5, 7][(seed % 5) as usize]);
            },
        );
    }

    /// The mailbox word of the fake for `addr` (0: the fake's own pointer).
    pub(crate) fn oc784_mail(addr: u32) -> u32 {
        DATA + 0x30_1000 + (addr & 0xffc)
    }

    /// Make the fake for `addr` return `v` in `v0` (stored +3).
    pub(crate) fn oc784_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc784_mail(addr), v.wrapping_add(3));
    }

    /// Make the fake for `addr` return 0.
    pub(crate) fn oc784_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc784_mail(addr), 1);
    }

    pub(crate) const OC784_FILE: u32 = DATA + 0x2_0000;

    /// `OVScreenOverlayMenu_Init`: the overlay list and the five option
    /// widgets (each with a vtable of faked `setVisible`/`setStatic`), or no
    /// widgets at all (every lookup returns 0).
    #[test]
    fn ov_screen_overlay_menu_init_matches() {
        oc784_install_fakes();
        fakes::covered(
            ee::s05::f_0020d1d8__generated,
            ov_screen_overlay_menu_init,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x784e, 20);
                oc784_arm(c);
                c.g[A0][0] = sx(OC784_OBJ);
                c.bus.write32(OC784_OBJ + 0x40, r.next_u32());
                let vt = fakes::vtable(c, &mut r, &[0x40, 0x48]);
                c.bus.write32(OC784_FILE + 8, vt);
                c.bus.write32(OC784_FILE + 0x90, r.next_u32());
                c.bus.write32(OC784_FILE + 0x18, r.next_u32());
                if seed % 4 == 0 {
                    oc784_force_zero(c, 0x0039_d860);
                } else {
                    oc784_force(c, 0x0039_d860, OC784_FILE);
                }
            },
        );
    }

    pub(crate) use crate::game::bx::bxstringctor::tests::oc804_fixture;

    pub(crate) use crate::game::bx::bxstringctor::tests::OC804_WORLD;

    pub(crate) const OC804_X: u32 = DATA + 0x26_0000;

    /// `GameUI_OpenScreen(id)`: builds the front-end screen object of `id`
    /// (an allocation, the base constructor, the vtable and the id stored, a
    /// few ids call their own constructor) and opens it. Id 0x23's
    /// constructor (`OVScreenProfileSavePDA_ctor`, no override slot) stays
    /// out: the real one would run on fixture data.
    #[test]
    fn game_ui_open_screen_matches() {
        oc804_install_fakes();
        fakes::covered(
            ee::s05::f_0020ab50__generated,
            game_ui_open_screen,
            172,
            &|c, seed| {
                let mut r = oc804_fixture(c, seed, 0x0020_ab50, 4);
                let id = match seed % 0x2b {
                    0x23 => 0x50,
                    k => k,
                };
                fill(c, &mut r, OC804_X, 0x40);
                c.bus.write32(OC804_WORLD + 0x84, OC804_X);
                c.bus.write32(OC804_X + 0x48, r.next_u32());
                c.g[A0][0] = id as u64;
                c.pc = 0x0020_ab50;
            },
        );
    }

    use crate::difftest::fakes::machine_rt as machine_m2;
    use crate::difftest::fakes::LOG_COUNT;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fresh;
    use crate::difftest::run_generated;
    pub(crate) use crate::game::fe::all_in_one_fe::tests::oc999_yn_setup;

    pub(crate) use crate::game::fe::fepopupmisc::tests::oc1000_install_fakes;
    pub(crate) use crate::game::fe::fepopupmisc::tests::oc1000_object;

    pub(crate) use crate::game::fe::fepopupmisc::tests::SEEDS_12 as SEEDS_3;
    pub(crate) use crate::game::fe::ovstatemap::tests::oc992_base;
    pub(crate) use crate::game::fe::ovstatemap::tests::oc992_force;

    pub(crate) use crate::game::fe::ovstatemap::tests::OC992_CLS;

    pub(crate) use crate::game::fe::ovstatemap::tests::OC992_O;
    pub(crate) use crate::game::fe::ovstatemap::tests::OC992_P;
    pub(crate) use crate::game::fe::ovstatemap::tests::OC992_Q;
    pub(crate) use crate::game::fe::ovstatemap::tests::OC992_R;
    pub(crate) use crate::game::fe::ovstatemap::tests::OC992_TGT;
    pub(crate) use crate::game::fe::ovstatemap::tests::OC992_W;
    use std::ptr::addr_of_mut;

    /// Every popup type 0..=0x2b (cases 1..0x28 plus the fall-through), the
    /// constructors and allocator faked, some runs unwinding mid-call.
    #[test]
    fn game_ui_push_popup_screen_matches() {
        covered(
            ee::s05::f_0020bab8__generated,
            game_ui_push_popup_screen,
            440,
            &|c, seed| {
                machine_m2(c, seed, 0x0020_bab8, 4);
                c.pc = 0x0020_bab8;
                // world (gp-0x848) -> +0x84 -> +0x48 = UI engine object
                let world = DATA + 0x1000;
                let a = DATA + 0x2000;
                let ui = DATA + 0x3000;
                let gp = c.g[GP][0] as u32;
                c.bus.write32(gp.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, a);
                c.bus.write32(a + 0x48, ui);
                c.g[A0][0] = (seed % 0x2c) as u64;
            },
        );
    }

    /// Gate word of this module's fakes (inside the machine's own RAM, away from
    /// the fakes' call log at `DATA + 0x100000`).
    pub(crate) const OC983_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC983_MAGIC: u32 = 0x0983_0983;

    /// Bit n set: fake call n returns 0 (the "failed / null" branches).
    pub(crate) const OC983_ZMASK: u32 = OC983_CTRL + 4;

    pub(crate) const OC983_SELF: u32 = DATA + 0x1000;

    pub(crate) const OC983_WIDGET: u32 = DATA + 0x2000;

    pub(crate) const OC983_GAME: u32 = DATA + 0x3000;

    /// A fake callee: logs like `fakes::fake_call`, then zeroes `v0` when the
    /// fixture's mask asks for it for this call index.
    pub(crate) fn oc983_call(c: &mut Cpu, addr: u32) {
        let n = c.bus.read32(LOG_COUNT);
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc == ra && n < 32 && (c.bus.read32(OC983_ZMASK) >> n) & 1 != 0 {
            c.g[V0][0] = 0;
        }
    }

    pub(crate) fn oc983_miss(c: &mut Cpu, target: u32) {
        oc983_call(c, target);
    }

    macro_rules! oc983_fakes {
        ($install:ident; $(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC983_CTRL) != OC983_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc983_call(c, $addr);
                }
            )*
            fn $install() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc983_fakes!(
        oc983_install;
        (oc983_fake_0011de60, OC983_PREV_0011DE60, ee::s00::OVERRIDE_0011DE60, ee::s00::f_0011de60__generated, 0x0011de60),
        (oc983_fake_0011df18, OC983_PREV_0011DF18, ee::s00::OVERRIDE_0011DF18, ee::s00::f_0011df18__generated, 0x0011df18),
        (oc983_fake_001253d0, OC983_PREV_001253D0, ee::s00::OVERRIDE_001253D0, ee::s00::f_001253d0__generated, 0x001253d0),
        (oc983_fake_00128a10, OC983_PREV_00128A10, ee::s00::OVERRIDE_00128A10, ee::s00::f_00128a10__generated, 0x00128a10),
        (oc983_fake_00144df0, OC983_PREV_00144DF0, ee::s01::OVERRIDE_00144DF0, ee::s01::f_00144df0__generated, 0x00144df0),
        (oc983_fake_001451e8, OC983_PREV_001451E8, ee::s01::OVERRIDE_001451E8, ee::s01::f_001451e8__generated, 0x001451e8),
        (oc983_fake_0014dc80, OC983_PREV_0014DC80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated, 0x0014dc80),
        (oc983_fake_0014dd58, OC983_PREV_0014DD58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated, 0x0014dd58),
        (oc983_fake_0014de68, OC983_PREV_0014DE68, ee::s01::OVERRIDE_0014DE68, ee::s01::f_0014de68__generated, 0x0014de68),
        (oc983_fake_001d8240, OC983_PREV_001D8240, ee::s04::OVERRIDE_001D8240, ee::s04::f_001d8240__generated, 0x001d8240),
        (oc983_fake_001d97e0, OC983_PREV_001D97E0, ee::s04::OVERRIDE_001D97E0, ee::s04::f_001d97e0__generated, 0x001d97e0),
        (oc983_fake_00208bb8, OC983_PREV_00208BB8, ee::s05::OVERRIDE_00208BB8, ee::s05::f_00208bb8__generated, 0x00208bb8),
        (oc983_fake_00208f88, OC983_PREV_00208F88, ee::s05::OVERRIDE_00208F88, ee::s05::f_00208f88__generated, 0x00208f88),
        (oc983_fake_002095e8, OC983_PREV_002095E8, ee::s05::OVERRIDE_002095E8, ee::s05::f_002095e8__generated, 0x002095e8),
        (oc983_fake_00209648, OC983_PREV_00209648, ee::s05::OVERRIDE_00209648, ee::s05::f_00209648__generated, 0x00209648),
        (oc983_fake_0020a088, OC983_PREV_0020A088, ee::s05::OVERRIDE_0020A088, ee::s05::f_0020a088__generated, 0x0020a088),
        (oc983_fake_0020a380, OC983_PREV_0020A380, ee::s05::OVERRIDE_0020A380, ee::s05::f_0020a380__generated, 0x0020a380),
        (oc983_fake_0020ab50, OC983_PREV_0020AB50, ee::s05::OVERRIDE_0020AB50, ee::s05::f_0020ab50__generated, 0x0020ab50),
        (oc983_fake_0020d110, OC983_PREV_0020D110, ee::s05::OVERRIDE_0020D110, ee::s05::f_0020d110__generated, 0x0020d110),
        (oc983_fake_00211088, OC983_PREV_00211088, ee::s05::OVERRIDE_00211088, ee::s05::f_00211088__generated, 0x00211088),
        (oc983_fake_00212138, OC983_PREV_00212138, ee::s05::OVERRIDE_00212138, ee::s05::f_00212138__generated, 0x00212138),
        (oc983_fake_00227f80, OC983_PREV_00227F80, ee::s05::OVERRIDE_00227F80, ee::s05::f_00227f80__generated, 0x00227f80),
        (oc983_fake_00230180, OC983_PREV_00230180, ee::s06::OVERRIDE_00230180, ee::s06::f_00230180__generated, 0x00230180),
        (oc983_fake_002302a8, OC983_PREV_002302A8, ee::s06::OVERRIDE_002302A8, ee::s06::f_002302a8__generated, 0x002302a8),
        (oc983_fake_00230488, OC983_PREV_00230488, ee::s06::OVERRIDE_00230488, ee::s06::f_00230488__generated, 0x00230488),
        (oc983_fake_00231250, OC983_PREV_00231250, ee::s06::OVERRIDE_00231250, ee::s06::f_00231250__generated, 0x00231250),
        (oc983_fake_00238160, OC983_PREV_00238160, ee::s06::OVERRIDE_00238160, ee::s06::f_00238160__generated, 0x00238160),
        (oc983_fake_00238348, OC983_PREV_00238348, ee::s06::OVERRIDE_00238348, ee::s06::f_00238348__generated, 0x00238348),
        (oc983_fake_00241710, OC983_PREV_00241710, ee::s06::OVERRIDE_00241710, ee::s06::f_00241710__generated, 0x00241710),
        (oc983_fake_00257610, OC983_PREV_00257610, ee::s06::OVERRIDE_00257610, ee::s06::f_00257610__generated, 0x00257610),
        (oc983_fake_002706f0, OC983_PREV_002706F0, ee::s07::OVERRIDE_002706F0, ee::s07::f_002706f0__generated, 0x002706f0),
        (oc983_fake_0028b180, OC983_PREV_0028B180, ee::s07::OVERRIDE_0028B180, ee::s07::f_0028b180__generated, 0x0028b180),
        (oc983_fake_0028fae0, OC983_PREV_0028FAE0, ee::s07::OVERRIDE_0028FAE0, ee::s07::f_0028fae0__generated, 0x0028fae0),
        (oc983_fake_0030b758, OC983_PREV_0030B758, ee::s10::OVERRIDE_0030B758, ee::s10::f_0030b758__generated, 0x0030b758),
        (oc983_fake_00317670, OC983_PREV_00317670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated, 0x00317670),
        (oc983_fake_00317d70, OC983_PREV_00317D70, ee::s10::OVERRIDE_00317D70, ee::s10::f_00317d70__generated, 0x00317d70),
        (oc983_fake_00397cc0, OC983_PREV_00397CC0, ee::s13::OVERRIDE_00397CC0, ee::s13::f_00397cc0__generated, 0x00397cc0),
        (oc983_fake_0039c778, OC983_PREV_0039C778, ee::s13::OVERRIDE_0039C778, ee::s13::f_0039c778__generated, 0x0039c778),
        (oc983_fake_0039c870, OC983_PREV_0039C870, ee::s13::OVERRIDE_0039C870, ee::s13::f_0039c870__generated, 0x0039c870),
        (oc983_fake_0039d860, OC983_PREV_0039D860, ee::s13::OVERRIDE_0039D860, ee::s13::f_0039d860__generated, 0x0039d860),
        (oc983_fake_0039e8b8, OC983_PREV_0039E8B8, ee::s13::OVERRIDE_0039E8B8, ee::s13::f_0039e8b8__generated, 0x0039e8b8),
        (oc983_fake_0039e948, OC983_PREV_0039E948, ee::s13::OVERRIDE_0039E948, ee::s13::f_0039e948__generated, 0x0039e948),
        (oc983_fake_0039f190, OC983_PREV_0039F190, ee::s13::OVERRIDE_0039F190, ee::s13::f_0039f190__generated, 0x0039f190),
        (oc983_fake_0039f320, OC983_PREV_0039F320, ee::s13::OVERRIDE_0039F320, ee::s13::f_0039f320__generated, 0x0039f320),
        (oc983_fake_003a0d00, OC983_PREV_003A0D00, ee::s13::OVERRIDE_003A0D00, ee::s13::f_003a0d00__generated, 0x003a0d00),
        (oc983_fake_003a0e90, OC983_PREV_003A0E90, ee::s13::OVERRIDE_003A0E90, ee::s13::f_003a0e90__generated, 0x003a0e90),
        (oc983_fake_003a1000, OC983_PREV_003A1000, ee::s13::OVERRIDE_003A1000, ee::s13::f_003a1000__generated, 0x003a1000),
        (oc983_fake_00417828, OC983_PREV_00417828, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated, 0x00417828),
    );

    /// A fixture word: 0, 1, a small number, a pointer into the filled area or a
    /// fake function address (so a call through a loaded pointer is a fake call).
    pub(crate) fn oc983_word(r: &mut Rng) -> u32 {
        match r.next_u32() % 8 {
            0..=2 => 0,
            3 => 1,
            4 => r.next_u32() % 16,
            5 | 6 => DATA + (r.next_u32() & 0x3ff0),
            _ => FAKE + 0x10 * (r.next_u32() % 4),
        }
    }

    /// Scrambled machine with the game image, this module's fakes (gate open, a
    /// random mask of calls that return 0), a sparse random data area
    /// (`DATA..DATA+0x4000`), some forced call results pointing into it and the
    /// game object at `gp - 0x848`.
    pub(crate) fn oc983_fixture(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc983_install();
        let mut r = machine(c, seed, salt, calls);
        elf_image(c);
        c.hooks.miss = oc983_miss;
        c.bus.write32(OC983_CTRL, OC983_MAGIC);
        let zero_mask = r.next_u32() & r.next_u32();
        c.bus.write32(OC983_ZMASK, zero_mask);
        for i in 0..0x1000u32 {
            let w = oc983_word(&mut r);
            c.bus.write32(DATA + 4 * i, w);
        }
        for n in 0..8u32 {
            if r.next_u32() & 1 != 0 {
                let p = DATA + (r.next_u32() & 0x3ff0);
                c.bus.write32(LOG_RESULTS + 4 * n, p);
            }
        }
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC983_GAME);
        r
    }

    /// The number of distinct cycle totals the generated body takes over the seeds.
    pub(crate) fn oc983_paths(f: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) -> usize {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, f);
            seen.insert(c.cycles - before);
        }
        seen.len()
    }

    /// `OVScreenYNDialog_Init(self)`: creates the dialog screen, runs the state
    /// overlay init and sets the button texts by the dialog type at `+0x9c`
    /// (a jump table of 7 types; larger types skip the texts).
    pub(crate) fn oc983_init_setup(c: &mut Cpu, seed: u32) {
        let _r = oc983_fixture(c, seed, 0x20d338, 14);
        c.pc = 0x0020_d338;
        c.g[A0][0] = sx(OC983_SELF);
        c.bus.write32(OC983_SELF + 0x9c, seed % 9);
    }

    #[test]
    fn ov_screen_yn_dialog_init_matches() {
        check(
            ee::s05::f_0020d338__generated,
            ov_screen_yn_dialog_init,
            300,
            &oc983_init_setup,
        );
        let n = oc983_paths(ee::s05::f_0020d338__generated, 300, &oc983_init_setup);
        assert!(n >= 6, "only {n} distinct paths");
    }

    /// `OVScreenYNDialog_OnWidgetAction(self, widget, event)`: events 5 (a
    /// button chosen: by the widget's misc data and the dialog type at `+0x9c`
    /// restart, quit, save, ... the race) and 6 (back), with the game event
    /// kind (`gp - 0x69c`) and the pause-dialog flag (`gp - 0x9e8`) varying.
    pub(crate) fn oc983_yn_action_setup(c: &mut Cpu, seed: u32) {
        let _r = oc983_fixture(c, seed, 0x20d590, 40);
        c.pc = 0x0020_d590;
        c.g[A0][0] = sx(OC983_SELF);
        let widget = if seed >= 216 && seed % 5 == 0 {
            0
        } else {
            OC983_WIDGET
        };
        c.g[A1][0] = sx(widget);
        c.g[A2][0] = sx([5, 5, 5, 5, 5, 5, 6, 7, 0][(seed % 9) as usize]);
        c.bus.write32(OC983_WIDGET + 0x18, (seed / 9) % 3);
        c.bus.write32(OC983_SELF + 0x9c, (seed / 27) % 8);
        c.bus
            .write32(GAME_GP.wrapping_sub(0x69c), (seed / 5) % 8 + 5);
        c.bus.write32(GAME_GP.wrapping_sub(0x9e8), (seed / 2) % 2);
    }

    #[test]
    fn ov_screen_yn_dialog_on_widget_action_matches() {
        check(
            ee::s05::f_0020d590__generated,
            ov_screen_yn_dialog_on_widget_action,
            300,
            &oc983_yn_action_setup,
        );
        let n = oc983_paths(ee::s05::f_0020d590__generated, 300, &oc983_yn_action_setup);
        assert!(n >= 10, "only {n} distinct paths");
    }

    pub(crate) const OC992_S: u32 = DATA + 0x0023_2000;

    #[test]
    fn ovscreen_pda_template_init_matches() {
        oc992_install_fakes();
        check(
            ee::s05::f_0020a778__generated,
            ovscreen_pda_template_init,
            200,
            &|c, seed| {
                let mut r = oc992_base(c, seed, 0x0992_a778);
                c.pc = 0x0020_a778;
                c.g[A0][0] = sx(OC992_O);
                c.bus.write32(OC992_O + 0x10, OC992_P);
                oc992_force(c, 0x0039_7cc0, if seed % 2 == 0 { 0 } else { OC992_Q });
                oc992_force(c, 0x0039_d860, if seed % 3 == 0 { 0 } else { OC992_Q });
                c.bus.write32(OC992_Q + 0xb0, r.next_u32());
                c.bus.write32(OC992_Q + 8, OC992_CLS);
                // The row widget's method table (+0x8c of the world, +4, +0x20/+0x24).
                c.bus.write32(OC992_W + 0x8c, OC992_R);
                c.bus.write32(OC992_R + 4, OC992_S);
                c.bus.write16(OC992_S + 0x20, 0x10);
                c.bus.write32(OC992_S + 0x24, OC992_TGT);
                c.bus.write32(OC992_W + 0x1c, r.next_u32());
            },
        );
    }

    #[test]
    fn gameui_open_pause_menu_matches() {
        oc992_install_fakes();
        check(
            ee::s05::f_0020ca10__generated,
            gameui_open_pause_menu,
            200,
            &|c, seed| {
                let mut r = oc992_base(c, seed, 0x0992_ca10);
                c.pc = 0x0020_ca10;
                c.bus.write32(GAME_GP.wrapping_sub(0x204), OC992_O);
                c.bus
                    .write32(0x0053_4b30, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(OC992_W + 0x84, OC992_P);
                c.bus
                    .write32(OC992_P + 0x224, if seed % 2 == 0 { 0 } else { 1 });
                c.bus
                    .write32(OC992_P + 0x214, if seed % 4 == 0 { 0xb } else { 0x4 });
                c.bus.write32(GAME_GP.wrapping_add(0xce8), OC992_Q);
                c.bus.write8(0x0053_5bc8 + 0x49, (seed % 2) as u8);
                c.bus.write8(
                    0x0053_5bc8 + 0x48,
                    if seed % 5 == 0 {
                        4
                    } else {
                        r.next_u32() as u8 | 1
                    },
                );
            },
        );
    }

    #[test]
    fn ovscreen_yn_dialog_quit_game_update_matches() {
        oc992_install_fakes();
        check(
            ee::s05::f_0020dbd0__generated,
            ovscreen_yn_dialog_quit_game_update,
            200,
            &|c, seed| {
                let mut r = oc992_base(c, seed, 0x0992_dbd0);
                c.pc = 0x0020_dbd0;
                c.g[A0][0] = sx(OC992_O);
                c.bus.write32(OC992_O + 0x10, OC992_P);
                c.bus.write32(OC992_O + 0x40, OC992_Q);
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x204),
                    if seed % 6 == 0 { 0 } else { OC992_R },
                );
                c.bus
                    .write32(OC992_R + 0x68, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(
                    OC992_R + 0x64,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                oc992_force(c, 0x0039_c778, if seed % 2 == 0 { 0xffff } else { 7 });
            },
        );
    }

    #[test]
    fn ov_screen_replay_menu_init_matches() {
        check(
            ee::s05::f_0020dc68__generated,
            ov_screen_replay_menu_init,
            200,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine_rt(c, seed, 0x0020_dc68, 4);
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                fill(c, &mut r, this, 0x40);
                let world = DATA + 0x2000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, DATA + 0x2100);
                c.bus.write32(DATA + 0x2100 + 0x28, DATA + 0x2200);
                let flag = if seed % 2 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(DATA + 0x2200 + 0x608, flag);
            },
        );
    }

    /// Gate word of this module's fakes (its own address and magic in the
    /// machine's RAM, so other tests' fakes stay off).
    pub(crate) const OC998_CTRL: u32 = DATA + 0x0020_9980;

    pub(crate) const OC998_MAGIC: u32 = 0x0998_0c0d;

    pub(crate) static OC998_PREV_UI_STATE_CTOR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC998_PREV_UI_STATE_DTOR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC998_PREV_OV_STATE_UPDATE: AtomicUsize = AtomicUsize::new(0);

    /// `UIState_ctor` with its gate on: logs the call and returns to the
    /// caller, so only the wrapper's own work is compared.
    pub(crate) fn oc998_fake_ui_state_ctor(c: &mut Cpu) {
        if c.bus.read32(OC998_CTRL) != OC998_MAGIC {
            return fakes::fall_back(c, &OC998_PREV_UI_STATE_CTOR, ee::s13::f_0039e2a0__generated);
        }
        fakes::fake_call(c, 0x0039_e2a0);
    }

    /// `UIState_dtor` with its gate on (see `oc998_fake_ui_state_ctor`).
    pub(crate) fn oc998_fake_ui_state_dtor(c: &mut Cpu) {
        if c.bus.read32(OC998_CTRL) != OC998_MAGIC {
            return fakes::fall_back(c, &OC998_PREV_UI_STATE_DTOR, ee::s13::f_0039e390__generated);
        }
        fakes::fake_call(c, 0x0039_e390);
    }

    /// `cOVStateOverlay_Update` with its gate on (see `oc998_fake_ui_state_ctor`).
    pub(crate) fn oc998_fake_ov_state_update(c: &mut Cpu) {
        if c.bus.read32(OC998_CTRL) != OC998_MAGIC {
            return fakes::fall_back(
                c,
                &OC998_PREV_OV_STATE_UPDATE,
                ee::s05::f_0020e900__generated,
            );
        }
        fakes::fake_call(c, 0x0020_e900);
    }

    /// Puts this module's three gated fakes in their override slots (once per
    /// process; with the gate off they run the body they replaced).
    pub(crate) fn oc998_hook() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E2A0),
            oc998_fake_ui_state_ctor,
            &OC998_PREV_UI_STATE_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E390),
            oc998_fake_ui_state_dtor,
            &OC998_PREV_UI_STATE_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0020E900),
            oc998_fake_ov_state_update,
            &OC998_PREV_OV_STATE_UPDATE,
        );
    }

    /// Scrambled registers and the gate on; the object at `DATA + 0x2000`
    /// (varied by seed) is the `this` argument. `calls` sets how early the
    /// fakes may unwind.
    pub(crate) fn oc998_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) {
        machine(c, seed, salt, calls);
        c.bus.write32(OC998_CTRL, OC998_MAGIC);
        c.g[A0][0] = (DATA + 0x2000 + (seed % 4) * 0x100) as u64;
    }

    #[test]
    fn ov_screen_overlay_menu_dtor_matches() {
        oc998_hook();
        check(
            ee::s05::f_0021c708__generated,
            ov_screen_overlay_menu_dtor,
            200,
            &|c, seed| oc998_setup(c, seed, 0x0021_c708, 2),
        );
    }

    #[test]
    fn ov_screen_overlay_menu_ctor_matches() {
        oc998_hook();
        check(
            ee::s05::f_0021c730__generated,
            ov_screen_overlay_menu_ctor,
            200,
            &|c, seed| oc998_setup(c, seed, 0x0021_c730, 2),
        );
    }

    #[test]
    fn ov_screen_yn_dialog_dtor_matches() {
        oc998_hook();
        check(
            ee::s05::f_0021c798__generated,
            ov_screen_yn_dialog_dtor,
            200,
            &|c, seed| oc998_setup(c, seed, 0x0021_c798, 2),
        );
    }

    #[test]
    fn ov_screen_yn_dialog_ctor_matches() {
        oc998_hook();
        check(
            ee::s05::f_0021c7c0__generated,
            ov_screen_yn_dialog_ctor,
            200,
            &|c, seed| oc998_setup(c, seed, 0x0021_c7c0, 2),
        );
    }

    #[test]
    fn ov_screen_yn_dialog_update_matches() {
        oc998_hook();
        check(
            ee::s05::f_0021c828__generated,
            ov_screen_yn_dialog_update,
            200,
            &|c, seed| oc998_setup(c, seed, 0x0021_c828, 1),
        );
    }

    #[test]
    fn ov_screen_yn_dialog_quit_game_dtor_matches() {
        check(
            ee::s05::f_0021c928__generated,
            ov_screen_yn_dialog_quit_game_dtor,
            200,
            &|c, seed| oc999_yn_setup(c, seed, 0x0021_c928),
        );
    }

    #[test]
    fn ov_screen_yn_dialog_quit_game_ctor_matches() {
        check(
            ee::s05::f_0021c950__generated,
            ov_screen_yn_dialog_quit_game_ctor,
            200,
            &|c, seed| oc999_yn_setup(c, seed, 0x0021_c950),
        );
    }

    #[test]
    fn ov_screen_pda_template_dtor_matches() {
        oc1000_install_fakes();
        covered(
            ee::s05::f_0021ca88__generated,
            ov_screen_pda_template_dtor,
            SEEDS_3,
            &|c, seed| {
                oc1000_object(c, seed, 0x21ca88, 1);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    #[test]
    fn ov_screen_pda_template_ctor_matches() {
        oc1000_install_fakes();
        covered(
            ee::s05::f_0021caa8__generated,
            ov_screen_pda_template_ctor,
            SEEDS_3,
            &|c, seed| {
                oc1000_object(c, seed, 0x21caa8, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
            },
        );
    }
}
