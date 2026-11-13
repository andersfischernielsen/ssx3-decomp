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

/// `void GameUI_Update(void)` (0x20cfd0): run the pending UI action
/// (`gp-0x9f4`, through a jump table): 1 handles the menu result; 2 leaves
/// the replay (stop it, mark the HUD dirty and, online in the pause menu of
/// mode 2, close every UI state); 3 closes the PDA template and every UI
/// state; 4 requests game state 0xe. Actions 2-4 clear the pending action.
#[allow(dead_code)]
pub fn game_ui_update(c: &mut Cpu) {
    use game_ui::*;
    use game_ui_callee::*;
    guest(c, |c| {
        let gp = c.g[GP][0] as u32;
        let gpa = |off: u32| Addr(gp.wrapping_sub(off));
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = gpa(GP_WORLD).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(W_GAME)).lw(c);
        c.g[S1][0] = Addr((c.g[V0][0] as u32).wrapping_add(G_UI)).lw(c);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(UI_STACK));
        c.g[S0][0] = 0;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, GET_ACTIVE_STATE, 0x0020_d000)?;
            c.g[S0][0] = c.g[V0][0];
        }

        c.cycles += 4;
        c.g[V1][0] = gpa(GP_ACTION).lw(c);
        c.g[V0][0] = (c.g[V1][0] < 5) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[V0][0] = 0x0047_0000;
        // Whether the exit passes the block at 0x20d0f8 (+1 cycle).
        let mut via_d0f8 = true;
        if in_range {
            c.cycles += 6;
            c.g[V1][0] = sx(((c.g[V1][0] as u32) << 2).wrapping_add(JUMP_TABLE));
            c.g[V0][0] = sx(JUMP_TABLE);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let target = c.g[A0][0] as u32;
            match target {
                0x0020_d02c => {
                    // 1: the menu result.
                    c.cycles += 4;
                    HANDLE_MENU_RESULT.call(c, 0x0020_d034)?;
                    via_d0f8 = false;
                }
                0x0020_d03c => {
                    // 2: leave the replay.
                    c.cycles += 11;
                    c.g[V1][0] = gpa(GP_WORLD).lw(c);
                    gpa(GP_ACTION).sw(c, 0);
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(W_GAME)).lw(c);
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(G_REPLAY)).lw(c);
                    call_ptr(c, REPLAY_FINISH, 0x0020_d050)?;
                    c.g[A1][0] = gpa(GP_WORLD).lw(c);
                    let world = c.g[A1][0] as u32;
                    c.g[A0][0] = 1;
                    c.g[V0][0] = Addr(world.wrapping_add(W_GAME)).lw(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(G_HUD)).lw(c);
                    Addr((c.g[V1][0] as u32).wrapping_add(8)).sw(c, 1);
                    // The state is read from s0 again, as the original does
                    // (a callee that misbehaves on test data can change it).
                    if c.g[S0][0] != 0 {
                        via_d0f8 = false;
                        c.cycles += 5;
                        c.g[V0][0] = Addr(world.wrapping_add(W_GAME)).lw(c);
                        c.g[A0][0] = 2;
                        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(G_MODE)).lw(c);
                        if c.g[V1][0] == 2 {
                            c.cycles += 4;
                            c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(S_ID)).lw(c);
                            c.g[V0][0] = 8;
                            if c.g[V1][0] == 8 {
                                c.cycles += 7;
                                c.g[A0][0] = Addr(world.wrapping_add(W_BE)).lw(c);
                                c.g[A1][0] = 7;
                                call_ptr(c, BE_GET_INTERFACE, 0x0020_d098)?;
                                c.g[V1][0] = 0x0053_0000;
                                c.g[V0][0] = Addr(ONLINE).lw(c);
                                if c.g[V0][0] != 0 {
                                    c.cycles += 4;
                                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(UI_STACK));
                                    call_ptr(c, CLOSE_ALL, 0x0020_d0b0)?;
                                }
                            }
                        }
                    }
                }
                0x0020_d0b8 => {
                    // 3: close the PDA template and every UI state.
                    c.cycles += 9;
                    gpa(GP_ACTION).sw(c, 0);
                    PDA_TEMPLATE_CLOSE.call(c, 0x0020_d0c0)?;
                    c.g[V0][0] = gpa(GP_WORLD).lw(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(W_GAME)).lw(c);
                    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(G_UI)).lw(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(UI_STACK));
                    call_ptr(c, CLOSE_ALL, 0x0020_d0d4)?;
                    via_d0f8 = false;
                }
                0x0020_d0dc => {
                    // 4: request game state 0xe.
                    c.cycles += 7;
                    c.g[V0][0] = gpa(GP_WORLD).lw(c);
                    c.g[A1][0] = 0xe;
                    gpa(GP_ACTION).sw(c, 0);
                    c.g[A2][0] = 2;
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(W_GAME)).lw(c);
                    c.g[A3][0] = 0;
                    call_ptr(c, REQUEST_STATE, 0x0020_d0f8)?;
                }
                0x0020_d0f8 => {}
                t => {
                    c.jump_out(t, 0x0020_cfd0, 0x0020_d10c);
                    return Ok(());
                }
            }
        }
        if via_d0f8 {
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

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

    /// GameUI_Update: actions 0-6 and -1 (jump table written in), no UI
    /// manager or a state stack with none / an inactive / an active state
    /// (pause menu id 8 or other), game mode 2 or not, online or not. The
    /// real callees run; the BE interface lookup is a fake.
    #[test]
    #[ignore = "fixture unbounded: real callees jump through jump tables the test machine never loads; port parked, not installed"]
    fn game_ui_update_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA, GAME_GP};
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        check(ee::s05::f_0020cfd0, game_ui_update, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x20_cfd0, 3);
            c.pc = 0x0020_cfd0;
            let gp = GAME_GP;
            let (world, game, mgr) = (DATA, DATA + 0x1000, DATA + 0x2000);
            let (replay, hud) = (DATA + 0x3000, DATA + 0x3800);
            let (sentinel, n1, n2) = (DATA + 0x4000, DATA + 0x4100, DATA + 0x4200);
            // BELibrary dummy (GameUI_HandleMenuResult gets its instance).
            w(c, gp - 0x1f04, DATA + 0x3_0000);
            w(c, gp - 0x848, world);
            w(c, gp - 0x9e4, 0);
            let action = [0u32, 1, 2, 3, 4, 5, 6, u32::MAX][(seed % 8) as usize];
            w(c, gp - 0x9f4, action);
            for (i, t) in [
                0x0020_d0f8u32,
                0x0020_d02c,
                0x0020_d03c,
                0x0020_d0b8,
                0x0020_d0dc,
            ]
            .into_iter()
            .enumerate()
            {
                w(c, 0x0047_1ae0 + 4 * i as u32, t);
            }
            w(c, 0x0053_4b30, (seed % 3 != 0) as u32);
            w(c, world + 0x84, game);
            w(c, world + 0x78, r.next_u32());
            w(c, world + 0xc0, 0);
            w(c, game + 0x28, replay);
            w(c, replay + 0x61c, 0);
            w(c, game + 0x200, hud);
            w(c, game + 0x214, if seed % 5 == 0 { 1 } else { 2 });
            w(
                c,
                game + 0x48,
                if seed % 13 == 0 { 0xffff_ffe8 } else { mgr },
            );
            // The UI state stack at mgr+0x18: nodes {prev, next, .., flags
            // +0x1c (bit 5 active), id +0x98}, ending at a sentinel.
            w(c, sentinel, sentinel);
            w(c, sentinel + 4, sentinel);
            let id = if seed % 4 == 3 { 3 } else { 8 };
            for (n, next) in [(n1, n2), (n2, sentinel)] {
                w(c, n, mgr + 0x18);
                w(c, n + 4, next);
                w(c, n + 0x98, id);
            }
            w(
                c,
                n1 + 0x1c,
                if seed % 6 == 1 {
                    0x20
                } else {
                    r.next_u32() & !0x20
                },
            );
            w(
                c,
                n2 + 0x1c,
                r.next_u32() | if seed % 9 == 2 { 0 } else { 0x20 },
            );
            w(
                c,
                mgr + 0x18 + 4,
                match (seed / 3) % 3 {
                    0 => sentinel,
                    1 => n2,
                    _ => n1,
                },
            );
        });
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
}
