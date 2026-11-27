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

pub(crate) mod game_ui {

    /// `gp-0x848`: the world; its game module (`+0x84`) and BE library
    /// (`+0x78`).
    pub const GP_WORLD: u32 = 0x848;

    pub const W_GAME: u32 = 0x84;

    pub const W_BE: u32 = 0x78;

    /// `gp-0x9f4`: the pending UI action (0 none, 1 menu result, 2 leave the
    /// replay, 3 close the PDA template, 4 request game state 0xe).
    pub const GP_ACTION: u32 = 0x9f4;

    /// Game module: replay (`+0x28`), UI manager (`+0x48`, its state stack at
    /// `+0x18`), the HUD block (`+0x200`, its "dirty" word at `+8`), the game
    /// mode (`+0x214`).
    pub const G_REPLAY: u32 = 0x28;

    pub const G_UI: u32 = 0x48;

    pub const UI_STACK: u32 = 0x18;

    pub const G_HUD: u32 = 0x200;

    pub const G_MODE: u32 = 0x214;

    /// UI state id (`+0x98`); 8 is the pause menu.
    pub const S_ID: u32 = 0x98;

    /// The action jump table.
    pub const JUMP_TABLE: u32 = 0x0047_1ae0;

    /// The "online" flag word the pause-menu exit checks.
    pub const ONLINE: u32 = 0x0053_4b30;
}

pub(crate) mod game_ui_callee {
    use super::ee;
    use super::Callee;

    pub const HANDLE_MENU_RESULT: Callee = Callee {
        addr: 0x0020_ccf8,
        f: ee::s05::GameUI_HandleMenuResult,
    };

    pub const PDA_TEMPLATE_CLOSE: Callee = Callee {
        addr: 0x0020_d190,
        f: ee::s05::OVScreenPDATemplate_Close,
    };

    /// Called through the function table.
    pub const GET_ACTIVE_STATE: u32 = 0x0039_fa88;

    pub const REPLAY_FINISH: u32 = 0x0027_06f0;

    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;

    pub const CLOSE_ALL: u32 = 0x0039_f840;

    pub const REQUEST_STATE: u32 = 0x0023_1250;
}

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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
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
}
