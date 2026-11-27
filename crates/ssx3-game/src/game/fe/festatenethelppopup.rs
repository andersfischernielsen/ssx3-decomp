//! `src/fe/festatenethelppopup.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::gp_18 as gp;
pub(crate) use crate::game::fe::fepopup::widget_v24;
pub(crate) use crate::game::fe::festatetrophyroom::oc943_vcall;
pub(crate) use crate::game::fe::festatetrophyroom::oc946_gp;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::p;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::movie::movieplayer::lw_at;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001d_a510,
        c_fe_state_net_help_popup_set_button_help,
        "cFEStateNetHelpPopup_SetButtonHelp",
    ),
    (
        0x001d_a530,
        c_fe_state_net_help_popup_set_body_text,
        "cFEStateNetHelpPopup_SetBodyText",
    ),
    (
        0x001d_9db8,
        c_fe_state_net_help_popup_init,
        "cFEStateNetHelpPopup_Init",
    ),
    (
        0x001d_9e20,
        c_fe_state_net_help_popup_v24,
        "cFEStateNetHelpPopup_v24",
    ),
    (
        0x001d_9e68,
        cfe_state_net_help_popup_update,
        "cFEStateNetHelpPopup_Update",
    ),
    (
        0x001d_a478,
        c_fe_state_net_help_popup_on_widget_action,
        "cFEStateNetHelpPopup_OnWidgetAction",
    ),
    (
        0x001d_a4a8,
        c_fe_state_net_help_popup_clear_fields,
        "cFEStateNetHelpPopup_ClearFields",
    ),
    (
        0x001d_a110,
        c_fe_state_net_help_popup_build_body_text,
        "cFEStateNetHelpPopup_BuildBodyText",
    ),
    (
        0x001d_a238,
        c_fe_state_net_help_popup_transition_in,
        "cFEStateNetHelpPopup_TransitionIn",
    ),
    (
        0x001d_a528,
        c_fe_state_net_help_popup_set_num_buttons,
        "cFEStateNetHelpPopup_SetNumButtons",
    ),
    (
        0x001d_a648,
        lobby_menu_init_create_account_page,
        "LobbyMenu_InitCreateAccountPage",
    ),
    (
        0x001d_a7e8,
        lobby_menu_init_login_page,
        "LobbyMenu_InitLoginPage",
    ),
    (
        0x001d_a988,
        lobby_menu_init_login_method_page,
        "LobbyMenu_InitLoginMethodPage",
    ),
    (
        0x001d_aac8,
        lobby_menu_init_create_persona_page,
        "LobbyMenu_InitCreatePersonaPage",
    ),
    (
        0x001d_abd0,
        lobby_menu_init_delete_persona_page_2,
        "LobbyMenu_InitDeletePersonaPage",
    ),
    (
        0x001d_acb8,
        lobby_menu_init_select_persona_page,
        "LobbyMenu_InitSelectPersonaPage",
    ),
    (
        0x001d_ae20,
        lobby_menu_init_select_room_page_2,
        "LobbyMenu_InitSelectRoomPage",
    ),
    (
        0x001d_af08,
        lobby_menu_init_room_occupants_page,
        "LobbyMenu_InitRoomOccupantsPage",
    ),
    (
        0x001d_b028,
        lobby_menu_init_chat_page,
        "LobbyMenu_InitChatPage",
    ),
];

// ---------------------------------------------------------------------------
// Net-help popup.

/// `void cFEStateNetHelpPopup_SetButtonHelp(cFEStateNetHelpPopup *this, int index, uint help, uint help2)`
/// (0x1da510): stores `help` and `help2` into the button record at
/// `this + index*4` (+0x68/+0x84).
pub fn c_fe_state_net_help_popup_set_button_help(c: &mut Cpu) {
    let this = c.g[A0][0];
    let index = c.g[A1][0];
    let help = c.g[A2][0];
    let help2 = c.g[A3][0];
    c.cycles += 6;
    c.g[A1][0] = sx((index as u32) << 2);
    c.g[V0][0] = sx((this as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[A0][0] = c.g[V0][0];
    Addr(c.g[V0][0] as u32).off(0x68).sw(c, help);
    Addr(c.g[A0][0] as u32).off(0x84).sw(c, help2);
    abi::ret(c);
}

/// `void cFEStateNetHelpPopup_SetBodyText(cFEStateNetHelpPopup *this, int index, uint text)`
/// (0x1da530): stores `text` at `this + index*4 + 0x58`.
pub fn c_fe_state_net_help_popup_set_body_text(c: &mut Cpu) {
    let this = c.g[A0][0];
    let index = c.g[A1][0];
    let text = c.g[A2][0];
    c.cycles += 4;
    c.g[A1][0] = sx((index as u32) << 2);
    c.g[A0][0] = sx((this as u32).wrapping_add(c.g[A1][0] as u32));
    Addr(c.g[A0][0] as u32).off(0x58).sw(c, text);
    abi::ret(c);
}

pub(crate) mod table {
    pub(crate) use crate::game::movie::movieplayer::table::UI_LIST_SET_FIRST_FLAGS;
}

pub(crate) mod callee {
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_BINDING_EVAL_FLOAT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
}

/// `cFEStateNetHelpPopup_OnWidgetAction(this, a1, msg)` 0x1da478: on a
/// "confirm" (message 6) put the popup's first-list entry into the pressed
/// state.
pub fn c_fe_state_net_help_popup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 6;
        let confirm = c.g[A2][0] == 6;
        frame.save_ra(c, 0);
        if confirm {
            c.cycles += 4;
            let this = abi::arg(c, 0);
            let obj = lw_at(c, this as u64, 0x10);
            c.g[A0][0] = obj;
            c.g[A1][0] = 1;
            c.g[A0][0] = sx((obj as u32).wrapping_add(0x18));
            call_ptr(c, table::UI_LIST_SET_FIRST_FLAGS, 0x001d_a498)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetHelpPopup_v24`: the net-help popup's virtual method 24
/// wrapper, with its "popup open" flag at `+0x50`.
pub fn c_fe_state_net_help_popup_v24(c: &mut Cpu) {
    guest(c, |c| widget_v24(c, 0x50, 0x001d_9e54))
}

// ---------------------------------------------------------------------------
// Front-end state initialisers: `StrHash` of a screen name, create the
// screen through the UI engine, store it and start its thread.

pub(crate) fn fe_state_screen_init(c: &mut Cpu, hash: u32, rets: [u32; 3]) -> Flow {
    c.cycles += 16;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    let this = c.g[A0][0];
    c.g[S0][0] = this;
    frame.save_ra(c, 0);
    frame.save(c, S1, 0x10);
    c.g[A0][0] = sx(hash);
    c.g[S1][0] = lw(c, p(this as u32, 0x10));
    call_ptr(c, 0x0031_7670, rets[0])?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = this;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0039_7cc0, rets[1])?;
    let no_screen = c.g[V0][0] == 0;
    sw(c, p(this as u32, 0x40), c.g[V0][0]);
    if !no_screen {
        c.cycles += 4;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        call_ptr(c, 0x0039_c870, rets[2])?;
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cFEStateNetHelpPopup_Init(cFEStateNetHelpPopup* self)` (0x1d9db8).
pub fn c_fe_state_net_help_popup_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0046_79a0, [0x001d_9ddc, 0x001d_9df0, 0x001d_9e08])
    })
}

/// `cFEStateNetHelpPopup_ClearFields(cFEStateNetHelpPopup* self)` (0x1da4a8):
/// clear the popup's state words, its three counters, and its seven
/// `{a = 0; b = -1}` pairs (0x68 bytes apart).
pub fn c_fe_state_net_help_popup_clear_fields(c: &mut Cpu) {
    c.cycles += 4;
    let this = c.g[A0][0] as u32;
    sw(c, p(this, 0x48), 0);
    c.g[V1][0] = 2;
    sw(c, p(this, 0x54), 0);
    c.g[V0][0] = sx(p(this, 0x60));
    loop {
        c.cycles += 7;
        sw(c, c.g[V0][0] as u32, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x001d_a4b8);
        }
    }
    c.cycles += 5;
    sw(c, p(this, 0x64), 0);
    c.g[V0][0] = sx(p(this, 0x68));
    c.g[A1][0] = 0xffff_ffff_ffff_ffff;
    c.g[A0][0] = sx(p(this, 0x84));
    c.g[V1][0] = 6;
    loop {
        c.cycles += 7;
        sw(c, c.g[V0][0] as u32, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        sw(c, c.g[A0][0] as u32, c.g[A1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x001d_a4e8);
        }
    }
}

/// `cFEStateNetHelpPopup_Update(this)` (0x1d9e68): if neither help-popup slot
/// has its binding pressed, close the popup.
pub fn cfe_state_net_help_popup_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x60);
        frame.save_fpr(c, 20, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, 18, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, 19, 0x10);
        c.g[S2][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);
        c.g[S3][0] = 1;
        c.mtc1(0, 20);
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);

        loop {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0xb0).lw(c);
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
            let absent = c.g[V0][0] == 0;
            c.g[A1][0] = 0;
            if !absent {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[A1][0] = c.g[S3][0];
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = 0;
                }
            }

            c.cycles += 2;
            if c.g[A1][0] == 0 {
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 6;
                c.g[A1][0] = 0x99;
                callee::INPUT_BINDING_EVAL_FLOAT.call(c, 0x001d_9ed4)?;
                c.c_eq_s(0, 20);
                if !c.fcc() {
                    c.g[S1][0] = 1;
                }
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            }

            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 2) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_9ea0);
            }
        }

        c.cycles += 2;
        let found = c.g[S1][0] != 0;
        c.lq(16, frame.at(c, 0x40));
        if !found {
            c.cycles += 5;
            c.g[A0][0] = Addr(this).off(0x10).lw(c);
            c.g[A1][0] = 1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            callee::UI_STATE_MGR_CLOSE_TOP_STATES.call(c, 0x001d_9f0c)?;
            c.lq(16, frame.at(c, 0x40));
        }

        c.cycles += 7;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateNetHelpPopup_TransitionIn(self, bool first)` (0x001da238): on the first call, fills the help popup's title, description and bottom texts (or the empty string), hides the nine icons, then shows the rows below the line count, positions each row's icon 22 units per row down and sets the row texts. Returns 1.
/// Reproduces every register, cycle and call of the generated body; `pc != entry`
/// (a resume after an unwound callee) runs the generated body.
pub fn c_fe_state_net_help_popup_transition_in(c: &mut Cpu) {
    if c.pc != 0x001da238 {
        return ee::s04::f_001da238__generated(c);
    }
    guest(c, |c| {
        'b_1da448: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
            let t = c.g[A1][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            if t {
                break 'b_1da448;
            } else {
                'b_1da290: {
                    'b_1da280: {
                        c.cycles += 3;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
                        if c.g[A1][0] == 0u64 {
                            c.g[A1][0] = 0x4a0000;
                            break 'b_1da280;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001da278);
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa8));
                            call_ptr(c, 0x003a1000, 0x001da278)?;
                            let t = true;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x58));
                            if t {
                                break 'b_1da290;
                            } else {
                                break 'b_1da280;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa8));
                    c.g[RA][0] = sx(0x001da28c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                    call_ptr(c, 0x003a0c50, 0x001da28c)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x58));
                    break 'b_1da290;
                }
                'b_1da2b8: {
                    'b_1da2a8: {
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = 0x4a0000;
                        if t {
                            break 'b_1da2a8;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001da2a0);
                            c.g[A0][0] = c.g[S1][0];
                            Callee {
                                addr: 0x001da110,
                                f: ee::s04::cFEStateNetHelpPopup_BuildBodyText,
                            }
                            .call(c, 0x001da2a0)?;
                            let t = true;
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                            if t {
                                break 'b_1da2b8;
                            } else {
                                break 'b_1da2a8;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xac));
                    c.g[RA][0] = sx(0x001da2b4);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                    call_ptr(c, 0x003a0c50, 0x001da2b4)?;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                    break 'b_1da2b8;
                }
                'b_1da2e0: {
                    'b_1da2d0: {
                        c.cycles += 2;
                        if c.g[A1][0] == 0u64 {
                            c.g[A1][0] = 0x4a0000;
                            break 'b_1da2d0;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001da2c8);
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xb0));
                            call_ptr(c, 0x003a1000, 0x001da2c8)?;
                            let t = true;
                            c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0xd0));
                            if t {
                                break 'b_1da2e0;
                            } else {
                                break 'b_1da2d0;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xb0));
                    c.g[RA][0] = sx(0x001da2dc);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                    call_ptr(c, 0x003a0c50, 0x001da2dc)?;
                    c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0xd0));
                    break 'b_1da2e0;
                }
                c.cycles += 4;
                c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0xb4));
                c.g[S0][0] = c.g[S3][0];
                c.g[S2][0] = 0x8;
                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                'l_1da2f0: loop {
                    c.cycles += 10;
                    c.g[A1][0] = 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    oc943_vcall(c, A2, 0x48, 0x001da310)?;
                    if (c.g[S2][0] as i64) >= 0 {
                        c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                        if event_due(c) {
                            deliver_event_here(c, 0x001da2f0);
                        }
                        continue 'l_1da2f0;
                    } else {
                        c.cycles += 6;
                        c.g[S5][0] = c.g[S3][0];
                        c.g[S2][0] = c.g[S4][0];
                        c.g[S3][0] = 0u64;
                        c.g[S4][0] = 0u64;
                        c.g[S6][0] = 0x4a0000;
                        'l_1da330: loop {
                            'b_1da438: {
                                'b_1da418: {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = 0x1;
                                    if t {
                                        break 'b_1da418;
                                    } else {
                                        'b_1da408: {
                                            c.cycles += 46;
                                            c.g[A2][0] = c.lw(c.g[S2][0] as u32);
                                            oc943_vcall(c, A2, 0x48, 0x001da358)?;
                                            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x84));
                                            c.g[A1][0] = 0x1;
                                            c.g[S0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[S4][0] as u32));
                                            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.g[V0][0] =
                                                sx((c.g[S5][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                                            c.g[V1][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                            c.g[A2][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                            let t = c.g[A2][0] as u32;
                                            c.g[RA][0] = sx(0x001da388);
                                            c.g[A0][0] =
                                                sx((c.g[A3][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x001da388)?;
                                            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                            c.mtc1(19, 1);
                                            c.cvt_s_w(1, 1);
                                            c.g[AT][0] = 0x41b00000;
                                            c.mtc1(1, 0);
                                            c.g[A0][0] =
                                                sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[S4][0] as u32));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.g[V0][0] =
                                                sx((c.g[S5][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            c.mul_s(1, 1, 0);
                                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                            c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x4b));
                                            c.ldr(2, (c.g[V1][0] as u32).wrapping_add(0x44));
                                            c.g[A1][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                            c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                                            c.sdr(2, c.g[SP][0] as u32);
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                                            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                                            c.add_s(0, 0, 1);
                                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                                            c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                                            c.ldr(2, c.g[SP][0] as u32);
                                            c.g[A1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                            c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x4b));
                                            c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x44));
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x4c),
                                                c.g[A1][0],
                                            );
                                            c.g[A1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x68));
                                            if c.g[A1][0] == 0u64 {
                                                c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                                                break 'b_1da408;
                                            } else {
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x001da400);
                                                c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                                                call_ptr(c, 0x003a1000, 0x001da400)?;
                                                let t = true;
                                                c.g[S3][0] =
                                                    sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                                if t {
                                                    break 'b_1da438;
                                                } else {
                                                    break 'b_1da408;
                                                }
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x001da410);
                                        c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1408));
                                        call_ptr(c, 0x003a0c50, 0x001da410)?;
                                        let t = true;
                                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_1da438;
                                        } else {
                                            break 'b_1da418;
                                        }
                                    }
                                }
                                c.cycles += 8;
                                c.g[A2][0] = c.lw(c.g[S2][0] as u32);
                                c.g[A1][0] = 0u64;
                                oc943_vcall(c, A2, 0x48, 0x001da434)?;
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                break 'b_1da438;
                            }
                            c.cycles += 4;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = ((c.g[S3][0] as i64) < 7i64) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x001da330);
                                }
                                continue 'l_1da330;
                            } else {
                                break 'b_1da448;
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 11;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[V0][0] = 0x1;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateNetHelpPopup::SetNumButtons(int n)` (0x1da528): stores the
/// button count at +0x48.
pub fn c_fe_state_net_help_popup_set_num_buttons(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x48), c.g[A1][0]);
        ret(c);
        Ok(())
    })
}

/// `cFEStateNetHelpPopup::BuildBodyText(self)` (0x1da110): collects the three
/// localised lines named by the ids at `self+0x58` (up to three, stopping at
/// the first zero id), allocates one buffer for them, concatenates the strings
/// into it and hands it to the popup body at `self+0xac`.
pub fn c_fe_state_net_help_popup_build_body_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S6, 0x20);
        frame.save(c, S0, 0x80);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, S1, 0x70);
        c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x58));
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[SP][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x40);
        c.g[S3][0] = 0;
        frame.save(c, S5, 0x30);
        c.g[S4][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[S5][0] = 0;
        'l_1da150: loop {
            'b_1da1a0: {
                c.cycles += 3;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
                let no_id = c.g[A1][0] == 0;
                c.g[V0][0] = Addr(oc946_gp(c, 0x848)).lw(c);
                if no_id {
                    break 'b_1da1a0;
                }
                c.cycles += 17;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                // sStrings: the vtable's slot 4 looks one id up
                c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8c)).lw(c);
                c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x4)).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
                let lookup = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, lookup, 0x001d_a180)?;
                c.g[A0][0] = c.g[V0][0];
                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                call_ptr(c, 0x003d_bca0, 0x001d_a18c)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = ((c.g[S2][0] as i64) < 3) as u64;
                let more = c.g[A0][0] != 0;
                c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x001d_a150);
                    }
                    continue 'l_1da150;
                }
                break 'b_1da1a0;
            }
            'b_1da1f0: {
                c.cycles += 2;
                let empty = (c.g[S4][0] as i64) <= 0;
                c.g[A1][0] = 0x460000;
                if empty {
                    break 'b_1da1f0;
                }
                c.cycles += 8;
                c.g[A0][0] = sx((c.g[S4][0] as u32) << 1);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a70));
                c.g[A2][0] = 0x100;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7e30, 0x001d_a1bc)?;
                c.g[S3][0] = c.g[V0][0];
                let none = (c.g[S5][0] as i64) <= 0;
                c.sh(c.g[S3][0] as u32, 0u64);
                if none {
                    break 'b_1da1f0;
                }
                c.cycles += 4;
                c.g[S1][0] = c.g[S5][0];
                c.g[S0][0] = c.g[SP][0];
                c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
                'l_1da1d8: loop {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    call_ptr(c, 0x002c_2688, 0x001d_a1e8)?;
                    if c.g[S1][0] != 0 {
                        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x001d_a1d8);
                        }
                        continue 'l_1da1d8;
                    }
                    break 'b_1da1f0;
                }
            }
            'b_1da210: {
                c.cycles += 5;
                c.g[A0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0xac)).lw(c);
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x003a_0d00, 0x001d_a1fc)?;
                let no_buf = c.g[S3][0] == 0;
                frame.restore(c, S0, 0x80);
                if no_buf {
                    break 'b_1da210;
                }
                c.cycles += 3;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x0031_7e98, 0x001d_a20c)?;
                frame.restore(c, S0, 0x80);
                break 'b_1da210;
            }
            c.cycles += 9;
            frame.restore(c, S1, 0x70);
            frame.restore(c, S2, 0x60);
            frame.restore(c, S3, 0x50);
            frame.restore(c, S4, 0x40);
            frame.restore(c, S5, 0x30);
            frame.restore(c, S6, 0x20);
            frame.restore_ra(c, 0x10);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `LobbyMenu_InitCreateAccountPage(self, vinfo, arg)` (0x1da648): builds the
/// create-account menu in place: the menu base, the title and spacer, the
/// login-name and password text items, the action and cancel items, then the
/// two name fields copied from `SSXApp`. Returns `self`.
pub fn lobby_menu_init_create_account_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 103;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S1, 0x80);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x90);
        frame.save(c, FP, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x70);
        c.g[FP][0] = c.g[A2][0];
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x134));
        frame.save(c, S5, 0x40);
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x150));
        frame.save(c, S6, 0x30);
        c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x168));
        frame.save(c, S7, 0x20);
        c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x194));
        frame.save_ra(c, 0);
        c.g[S6][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1c0));
        call_ptr(c, 0x002c_aa58, 0x001d_a698)?;
        c.g[S7][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1e4));
        c.g[V0][0] = 0x470000;
        c.g[A1][0] = 0x460000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2970));
        c.g[A0][0] = c.g[S3][0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x12c), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7a20));
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x130), c.g[V0][0]);
        call_ptr(c, 0x002c_d2a0, 0x001d_a6c0)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0xa;
        call_ptr(c, 0x002c_d350, 0x001d_a6cc)?;
        c.g[S0][0] = 0x4a0000;
        c.g[A1][0] = 0x460000;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1408));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7a30));
        c.g[A0][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, 0x002c_e418, 0x001d_a6e8)?;
        c.g[A1][0] = 0x460000;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7a40));
        call_ptr(c, 0x002c_e418, 0x001d_a6fc)?;
        c.g[A3][0] = 0;
        c.g[A2][0] = c.g[FP][0];
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x002c_cf98, 0x001d_a710)?;
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = c.g[S7][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7a50));
        call_ptr(c, 0x002c_cdf0, 0x001d_a720)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_a730)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_a740)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_a750)?;
        c.g[A1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_a760)?;
        c.g[A1][0] = c.g[S6][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_a770)?;
        c.g[A1][0] = c.g[S7][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_a780)?;
        c.g[V0][0] = Addr(oc946_gp(c, 0x848)).lw(c);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x180));
        c.g[A2][0] = 0xf;
        c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc4)).lw(c);
        c.sb((c.g[S1][0] as u32).wrapping_add(0x180), 0u64);
        call_ptr(c, 0x0041_6958, 0x001d_a798)?;
        c.g[V1][0] = Addr(oc946_gp(c, 0x848)).lw(c);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1ac));
        c.g[A2][0] = 0xf;
        c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc8)).lw(c);
        c.sb((c.g[S1][0] as u32).wrapping_add(0x1ac), 0u64);
        call_ptr(c, 0x0041_6958, 0x001d_a7b0)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `reg + off` as the 32-bit sum the generated code computes.
#[inline(always)]
pub(crate) fn init_pages_at(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

/// `addiu reg, reg, off`: the sign-extended 32-bit sum, as a register value.
#[inline(always)]
pub(crate) fn init_pages_reg(c: &Cpu, r: usize, off: u32) -> u64 {
    sx(init_pages_at(c, r, off))
}

/// `void LobbyMenu_InitLoginPage(LobbyMenu* menu, void* state, u32 arg)`
/// (0x1da7e8): lay out the login page. Fills the page's widget slots at
/// `menu+0x12c..+0x1e4` through the menu-item and text calls, and clears the
/// first byte of the two text buffers at `menu+0x180` and `menu+0x1ac`.
pub fn lobby_menu_init_login_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 103;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S1, 0x80);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x90);
        frame.save(c, FP, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x70);
        c.g[FP][0] = c.g[A2][0];
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        c.g[S3][0] = init_pages_reg(c, S1, 0x134);
        frame.save(c, S5, 0x40);
        c.g[S2][0] = init_pages_reg(c, S1, 0x150);
        frame.save(c, S6, 0x30);
        c.g[S4][0] = init_pages_reg(c, S1, 0x168);
        frame.save(c, S7, 0x20);
        c.g[S5][0] = init_pages_reg(c, S1, 0x194);
        frame.save_ra(c, 0);
        c.g[S6][0] = init_pages_reg(c, S1, 0x1c0);
        call_ptr(c, 0x002c_aa58, 0x001d_a838)?;
        c.g[S7][0] = init_pages_reg(c, S1, 0x1e4);
        // Globals: 0x470000 - 0x2918 = D_0046D6E8, 0x460000 + 0x7a20 = D_00467A20.
        c.g[V0][0] = 0x47_0000 - 0x2918;
        c.g[A0][0] = c.g[S3][0];
        let label = c.g[V0][0];
        Addr(init_pages_at(c, S1, 0x12c)).sw(c, label);
        c.g[A1][0] = 0x46_0000 + 0x7a20;
        let text = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = text;
        Addr(init_pages_at(c, S1, 0x130)).sw(c, text);
        call_ptr(c, 0x002c_d2a0, 0x001d_a860)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0xa;
        call_ptr(c, 0x002c_d350, 0x001d_a86c)?;
        // D_004A1408 and the four globals below it are the widget data.
        c.g[S0][0] = 0x4a_0000 + 0x1408;
        c.g[A1][0] = 0x46_0000 + 0x7a30;
        c.g[A0][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, 0x002c_e418, 0x001d_a888)?;
        c.g[A1][0] = 0x46_0000 + 0x7a40;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S5][0];
        call_ptr(c, 0x002c_e418, 0x001d_a89c)?;
        c.g[A3][0] = 0;
        c.g[A2][0] = c.g[FP][0];
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x002c_cf98, 0x001d_a8b0)?;
        c.g[A1][0] = 0x46_0000 + 0x7a50;
        c.g[A0][0] = c.g[S7][0];
        call_ptr(c, 0x002c_cdf0, 0x001d_a8c0)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_a8d0)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_a8e0)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_a8f0)?;
        c.g[A1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_a900)?;
        c.g[A1][0] = c.g[S6][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_a910)?;
        c.g[A1][0] = c.g[S7][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_a920)?;
        // The world pointer at gp-0x848: its +0xc4 and +0xc8 are the two texts.
        let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[V0][0] = world;
        c.g[A0][0] = init_pages_reg(c, S1, 0x180);
        c.g[A2][0] = 0xf;
        let text0 = Addr((world as u32).wrapping_add(0xc4)).lw(c);
        c.g[A1][0] = text0;
        Addr(init_pages_at(c, S1, 0x180)).sb(c, 0);
        call_ptr(c, 0x0041_6958, 0x001d_a938)?;
        let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[V1][0] = world;
        c.g[A0][0] = init_pages_reg(c, S1, 0x1ac);
        c.g[A2][0] = 0xf;
        let text1 = Addr((world as u32).wrapping_add(0xc8)).lw(c);
        c.g[A1][0] = text1;
        Addr(init_pages_at(c, S1, 0x1ac)).sb(c, 0);
        call_ptr(c, 0x0041_6958, 0x001d_a950)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMenu_InitLoginMethodPage(LobbyMenu* menu, void* state, u32 arg)`
/// (0x1da988): lay out the login-method page. Builds the create-account
/// and login pages it hands over to, then fills the method list's slots.
pub fn lobby_menu_init_login_method_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 80;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x70);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        c.g[S4][0] = init_pages_reg(c, S0, 0x32c);
        frame.save(c, S6, 0x30);
        c.g[S5][0] = init_pages_reg(c, S0, 0x544);
        frame.save(c, S7, 0x20);
        c.g[S6][0] = init_pages_reg(c, S0, 0x55c);
        frame.save(c, FP, 0x10);
        c.g[S7][0] = init_pages_reg(c, S0, 0x578);
        frame.save_ra(c, 0);
        c.g[FP][0] = init_pages_reg(c, S0, 0x130);
        call_ptr(c, 0x002c_aa58, 0x001d_a9d8)?;
        // Globals: 0x470000 - 0x29c8 = D_0046D638, 0x460000 + 0x7a20 = D_00467A20.
        c.g[V0][0] = 0x47_0000 - 0x29c8;
        c.g[S2][0] = 0x46_0000 + 0x7a20;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[FP][0];
        c.g[A2][0] = c.g[S2][0];
        let label = c.g[V0][0];
        Addr(init_pages_at(c, S0, 0x12c)).sw(c, label);
        Callee {
            addr: 0x001d_a648,
            f: ee::s04::LobbyMenu_InitCreateAccountPage,
        }
        .call(c, 0x001d_a9fc)?;
        c.g[S1][0] = 0x46_0000 + 0x7a58;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[S3][0] = init_pages_reg(c, S0, 0x528);
        Callee {
            addr: 0x001d_a7e8,
            f: ee::s04::LobbyMenu_InitLoginPage,
        }
        .call(c, 0x001d_aa18)?;
        c.g[A1][0] = 0x46_0000 + 0x7a60;
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x002c_d2a0, 0x001d_aa28)?;
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = 0xa;
        call_ptr(c, 0x002c_d350, 0x001d_aa34)?;
        c.g[A1][0] = c.g[FP][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S6][0];
        call_ptr(c, 0x002c_ccc8, 0x001d_aa44)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S7][0];
        call_ptr(c, 0x002c_ccc8, 0x001d_aa54)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_aa64)?;
        c.g[A1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_aa74)?;
        c.g[A1][0] = c.g[S6][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_aa84)?;
        c.g[A1][0] = c.g[S7][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_aa94)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x80);
        frame.restore(c, S0, 0x90);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMenu_InitCreatePersonaPage(LobbyMenu* menu, void* state)`
/// (0x1daac8): lay out the create-persona page: its widget slots at
/// `menu+0x134..`, the two text labels, and the persona list rows.
pub fn lobby_menu_init_create_persona_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 65;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        c.g[S3][0] = init_pages_reg(c, S0, 0x134);
        frame.save(c, S5, 0x10);
        c.g[S4][0] = init_pages_reg(c, S0, 0x150);
        frame.save_ra(c, 0);
        c.g[S2][0] = init_pages_reg(c, S0, 0x168);
        call_ptr(c, 0x002c_aa58, 0x001d_ab00)?;
        c.g[S5][0] = init_pages_reg(c, S0, 0x194);
        // Globals: 0x470000 - 0x2a78 = D_0046D588, 0x460000 + 0x7a70 = D_00467A70.
        c.g[V0][0] = 0x47_0000 - 0x2a78;
        c.g[A1][0] = 0x46_0000 + 0x7a70;
        c.g[A0][0] = c.g[S3][0];
        let label = c.g[V0][0];
        Addr(init_pages_at(c, S0, 0x12c)).sw(c, label);
        let text = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = text;
        Addr(init_pages_at(c, S0, 0x130)).sw(c, text);
        call_ptr(c, 0x002c_d2a0, 0x001d_ab28)?;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x002c_d350, 0x001d_ab34)?;
        c.g[A1][0] = 0x46_0000;
        c.g[A2][0] = 0x4a_0000 + 0x1408;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0x46_0000 + 0x7a80;
        call_ptr(c, 0x002c_e418, 0x001d_ab4c)?;
        c.g[A2][0] = 0x46_0000 + 0x7a90;
        c.g[A3][0] = 0;
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x002c_cf98, 0x001d_ab64)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_ab74)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_ab84)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_ab94)?;
        c.g[A1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_aba4)?;
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

/// `void LobbyMenu_InitDeletePersonaPage(LobbyMenu* menu, void* state)`
/// (0x1dabd0): lay out the delete-persona page. Runs the row-count loop
/// over the persona list (four passes) and then the label loop.
pub fn lobby_menu_init_delete_persona_page_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        c.g[S2][0] = init_pages_reg(c, S1, 0x168);
        frame.save_ra(c, 0);
        c.g[S3][0] = 3;
        call_ptr(c, 0x002c_aa58, 0x001d_ac00)?;
        c.g[S4][0] = u64::MAX;
        // Globals: 0x470000 - 0x2a20 = D_0046D5E0, 0x460000 + 0x7a98 = D_00467A98.
        c.g[V0][0] = 0x47_0000 - 0x2a20;
        c.g[A1][0] = 0x46_0000 + 0x7a98;
        let label = c.g[V0][0];
        Addr(init_pages_at(c, S1, 0x12c)).sw(c, label);
        c.g[A0][0] = init_pages_reg(c, S1, 0x134);
        let text = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = text;
        Addr(init_pages_at(c, S1, 0x130)).sw(c, text);
        call_ptr(c, 0x002c_d2a0, 0x001d_ac28)?;
        c.g[A0][0] = init_pages_reg(c, S1, 0x150);
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x002c_d350, 0x001d_ac34)?;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
            call_ptr(c, 0x002c_cf08, 0x001d_ac44)?;
            let more = c.g[S3][0] != c.g[S4][0];
            c.g[S2][0] = init_pages_reg(c, S2, 0x24);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001d_ac38);
                }
                continue;
            }
            break;
        }
        c.cycles += 11;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = init_pages_reg(c, S1, 0x134);
        c.g[A2][0] = u64::MAX;
        c.g[S0][0] = init_pages_reg(c, S1, 0x168);
        call_ptr(c, 0x002c_bcc8, 0x001d_ac60)?;
        c.g[S2][0] = 3;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = init_pages_reg(c, S1, 0x150);
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_ac74)?;
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = u64::MAX;
            c.g[S0][0] = init_pages_reg(c, S0, 0x24);
            call_ptr(c, 0x002c_bcc8, 0x001d_ac88)?;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[A1][0] = c.g[S0][0];
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001d_ac78);
                }
                continue;
            }
            break;
        }
        c.cycles += 9;
        c.g[V0][0] = c.g[S1][0];
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

/// `void LobbyMenu_InitSelectPersonaPage(LobbyMenu* menu, void* a1, void* state,
/// void* a3)` (0x1dacb8): lay out the select-persona page. Builds the
/// create-persona and delete-persona pages, the persona list rows and their
/// label cells.
pub fn lobby_menu_init_select_persona_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S3, 0x30);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S4, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[S4][0] = init_pages_reg(c, S3, 0x518);
        call_ptr(c, 0x002c_aa58, 0x001d_acf0)?;
        c.g[S5][0] = 3;
        // Globals: 0x470000 - 0x2ad0 = D_0046D530, 0x460000 + 0x7aa8 = D_00467AA8.
        c.g[V0][0] = 0x47_0000 - 0x2ad0;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = init_pages_reg(c, S3, 0x130);
        let label = c.g[V0][0];
        Addr(init_pages_at(c, S3, 0x12c)).sw(c, label);
        c.g[S0][0] = u64::MAX;
        Callee {
            addr: 0x001d_aac8,
            f: ee::s04::LobbyMenu_InitCreatePersonaPage,
        }
        .call(c, 0x001d_ad10)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = init_pages_reg(c, S3, 0x2e8);
        Callee {
            addr: 0x001d_abd0,
            f: ee::s04::LobbyMenu_InitDeletePersonaPage,
        }
        .call(c, 0x001d_ad1c)?;
        let text = Addr(c.g[S2][0] as u32).lw(c);
        c.g[V0][0] = text;
        c.g[A1][0] = 0x46_0000 + 0x7aa8;
        c.g[A0][0] = init_pages_reg(c, S3, 0x4e4);
        Addr(init_pages_at(c, S3, 0x4e0)).sw(c, text);
        call_ptr(c, 0x002c_d2a0, 0x001d_ad34)?;
        c.g[A0][0] = init_pages_reg(c, S3, 0x500);
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x002c_d350, 0x001d_ad40)?;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S4][0];
            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_sub(1));
            call_ptr(c, 0x002c_cf08, 0x001d_ad4c)?;
            let more = c.g[S5][0] != c.g[S0][0];
            c.g[S4][0] = init_pages_reg(c, S4, 0x24);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001d_ad40);
                }
                continue;
            }
            break;
        }
        c.cycles += 25;
        c.g[A0][0] = init_pages_reg(c, S3, 0x5a8);
        c.g[A2][0] = 0x46_0000 + 0x7a70;
        c.g[S4][0] = c.g[A0][0];
        c.g[A1][0] = init_pages_reg(c, S3, 0x130);
        c.g[S0][0] = init_pages_reg(c, S3, 0x5c4);
        call_ptr(c, 0x002c_ccc8, 0x001d_ad70)?;
        c.g[S2][0] = c.g[S0][0];
        c.g[A2][0] = 0x46_0000 + 0x7a98;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = init_pages_reg(c, S3, 0x2e8);
        call_ptr(c, 0x002c_ccc8, 0x001d_ad88)?;
        c.g[S0][0] = init_pages_reg(c, S3, 0x518);
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = init_pages_reg(c, S3, 0x4e4);
        c.g[A2][0] = u64::MAX;
        c.g[S1][0] = 3;
        call_ptr(c, 0x002c_bcc8, 0x001d_ada0)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = init_pages_reg(c, S3, 0x500);
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_adb0)?;
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 7;
            c.g[A0][0] = c.g[S3][0];
            c.g[A2][0] = u64::MAX;
            c.g[S0][0] = init_pages_reg(c, S0, 0x24);
            call_ptr(c, 0x002c_bcc8, 0x001d_adc8)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[A1][0] = c.g[S0][0];
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001d_adb8);
                }
                continue;
            }
            break;
        }
        c.cycles += 18;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S3][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_ade4)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S3][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_adf4)?;
        c.g[V0][0] = c.g[S3][0];
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

/// `void LobbyMenu_InitSelectRoomPage(LobbyMenu* menu, void* state)`
/// (0x1dae20): lay out the select-room page. Same shape as the
/// delete-persona page, with twelve row passes instead of four.
pub fn lobby_menu_init_select_room_page_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        c.g[S2][0] = init_pages_reg(c, S1, 0x168);
        frame.save_ra(c, 0);
        c.g[S3][0] = 0xb;
        call_ptr(c, 0x002c_aa58, 0x001d_ae50)?;
        c.g[S4][0] = u64::MAX;
        // Globals: 0x470000 - 0x2b28 = D_0046D4D8, 0x460000 + 0x7ab8 = D_00467AB8.
        c.g[V0][0] = 0x47_0000 - 0x2b28;
        c.g[A1][0] = 0x46_0000 + 0x7ab8;
        let label = c.g[V0][0];
        Addr(init_pages_at(c, S1, 0x12c)).sw(c, label);
        c.g[A0][0] = init_pages_reg(c, S1, 0x134);
        let text = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = text;
        Addr(init_pages_at(c, S1, 0x130)).sw(c, text);
        call_ptr(c, 0x002c_d2a0, 0x001d_ae78)?;
        c.g[A0][0] = init_pages_reg(c, S1, 0x150);
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x002c_d350, 0x001d_ae84)?;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
            call_ptr(c, 0x002c_cf08, 0x001d_ae94)?;
            let more = c.g[S3][0] != c.g[S4][0];
            c.g[S2][0] = init_pages_reg(c, S2, 0x24);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001d_ae88);
                }
                continue;
            }
            break;
        }
        c.cycles += 11;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = init_pages_reg(c, S1, 0x134);
        c.g[A2][0] = u64::MAX;
        c.g[S0][0] = init_pages_reg(c, S1, 0x168);
        call_ptr(c, 0x002c_bcc8, 0x001d_aeb0)?;
        c.g[S2][0] = 0xb;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = init_pages_reg(c, S1, 0x150);
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_aec4)?;
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = u64::MAX;
            c.g[S0][0] = init_pages_reg(c, S0, 0x24);
            call_ptr(c, 0x002c_bcc8, 0x001d_aed8)?;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[A1][0] = c.g[S0][0];
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001d_aec8);
                }
                continue;
            }
            break;
        }
        c.cycles += 9;
        c.g[V0][0] = c.g[S1][0];
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

/// `void LobbyMenu_InitRoomOccupantsPage(LobbyMenu* menu, void* a1, void* a2)`
/// (0x1daf08): lay out the room-occupants page: the occupant rows at
/// `menu+0x16c` and `menu+0x138..`, and the two label cells.
pub fn lobby_menu_init_room_occupants_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[S3][0] = init_pages_reg(c, S2, 0x16c);
        call_ptr(c, 0x002c_aa58, 0x001d_af38)?;
        c.g[S4][0] = 0x17;
        // Globals: 0x470000 - 0x2b80 = D_0046D480, 0x460000 + 0x7ac8 = D_00467AC8.
        c.g[V0][0] = 0x47_0000 - 0x2b80;
        c.g[A1][0] = 0x46_0000 + 0x7ac8;
        let label = c.g[V0][0];
        Addr(init_pages_at(c, S2, 0x12c)).sw(c, label);
        c.g[A0][0] = init_pages_reg(c, S2, 0x138);
        let first = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V1][0] = first;
        c.g[S0][0] = u64::MAX;
        Addr(init_pages_at(c, S2, 0x130)).sw(c, first);
        let second = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = second;
        Addr(init_pages_at(c, S2, 0x134)).sw(c, second);
        call_ptr(c, 0x002c_d2a0, 0x001d_af6c)?;
        c.g[A0][0] = init_pages_reg(c, S2, 0x154);
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x002c_d350, 0x001d_af78)?;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S3][0];
            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(1));
            call_ptr(c, 0x002c_cf08, 0x001d_af84)?;
            let more = c.g[S4][0] != c.g[S0][0];
            c.g[S3][0] = init_pages_reg(c, S3, 0x24);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001d_af78);
                }
                continue;
            }
            break;
        }
        c.cycles += 19;
        c.g[A0][0] = init_pages_reg(c, S2, 0x4cc);
        c.g[A2][0] = 0x46_0000 + 0x7ad8;
        c.g[S3][0] = c.g[A0][0];
        c.g[A3][0] = 0;
        c.g[A1][0] = u64::MAX - 1;
        call_ptr(c, 0x002c_cf98, 0x001d_afa8)?;
        c.g[S0][0] = init_pages_reg(c, S2, 0x16c);
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = init_pages_reg(c, S2, 0x138);
        c.g[A2][0] = u64::MAX;
        c.g[S1][0] = 0x17;
        call_ptr(c, 0x002c_bcc8, 0x001d_afc0)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = init_pages_reg(c, S2, 0x154);
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_afd0)?;
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 7;
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = u64::MAX;
            c.g[S0][0] = init_pages_reg(c, S0, 0x24);
            call_ptr(c, 0x002c_bcc8, 0x001d_afe8)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[A1][0] = c.g[S0][0];
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001d_afd8);
                }
                continue;
            }
            break;
        }
        c.cycles += 13;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_b004)?;
        c.g[V0][0] = c.g[S2][0];
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

/// `void LobbyMenu_InitChatPage(LobbyMenu* menu, void* state)` (0x1db028):
/// lay out the chat page. Fills the 14 chat-line slots from the table at
/// `gp+0x7d70` and the two label cells.
pub fn lobby_menu_init_chat_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        c.g[S2][0] = init_pages_reg(c, S1, 0x194);
        frame.save_ra(c, 0);
        c.g[S3][0] = 0xd;
        call_ptr(c, 0x002c_aa58, 0x001d_b058)?;
        c.g[S4][0] = u64::MAX;
        // Globals: 0x470000 - 0x2bd8 = D_0046D428, 0x460000 + 0x7ae0 = D_00467AE0.
        c.g[V0][0] = 0x47_0000 - 0x2bd8;
        c.g[A1][0] = 0x46_0000 + 0x7ae0;
        let label = c.g[V0][0];
        Addr(init_pages_at(c, S1, 0x12c)).sw(c, label);
        c.g[A0][0] = init_pages_reg(c, S1, 0x134);
        let text = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = text;
        Addr(init_pages_at(c, S1, 0x130)).sw(c, text);
        call_ptr(c, 0x002c_d2a0, 0x001d_b080)?;
        c.g[A1][0] = 0xe;
        c.g[A0][0] = init_pages_reg(c, S1, 0x150);
        call_ptr(c, 0x002c_d350, 0x001d_b08c)?;
        c.g[A1][0] = 0x46_0000 + 0x7af0;
        c.g[A2][0] = 0x4a_0000 + 0x1408;
        c.g[A0][0] = init_pages_reg(c, S1, 0x168);
        call_ptr(c, 0x002c_e418, 0x001d_b0a4)?;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
            call_ptr(c, 0x002c_cf08, 0x001d_b0b4)?;
            let more = c.g[S3][0] != c.g[S4][0];
            c.g[S2][0] = init_pages_reg(c, S2, 0x24);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001d_b0a8);
                }
                continue;
            }
            break;
        }
        c.cycles += 13;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = init_pages_reg(c, S1, 0x134);
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_b0d0)?;
        c.g[S2][0] = 0;
        c.g[S0][0] = init_pages_reg(c, S1, 0x194);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = init_pages_reg(c, S1, 0x150);
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_b0e4)?;
        c.g[V0][0] = 0x46_0000;
        c.g[S3][0] = init_pages_reg(c, V0, 0x7d70);
        loop {
            c.cycles += 12;
            let index = c.g[S2][0];
            Addr(init_pages_at(c, S0, 0x8)).sw(c, index);
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = u64::MAX;
            let row = Addr(c.g[S3][0] as u32).lw(c);
            c.g[V0][0] = row;
            c.g[S2][0] = init_pages_reg(c, S2, 0x1);
            c.g[S3][0] = init_pages_reg(c, S3, 0x4);
            Addr(init_pages_at(c, S0, 0x14)).sw(c, row);
            call_ptr(c, 0x002c_bcc8, 0x001d_b114)?;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 14) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = init_pages_reg(c, S0, 0x24);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001d_b0f0);
                }
                continue;
            }
            break;
        }
        c.cycles += 9;
        c.g[V0][0] = c.g[S1][0];
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::oc943_install_fakes;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    #[test]
    fn c_fe_state_net_help_popup_set_button_help_matches() {
        check(
            ssx3_ee::s04::f_001da510,
            c_fe_state_net_help_popup_set_button_help,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c10_0009 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                c.g[A3][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_net_help_popup_set_body_text_matches() {
        check(
            ssx3_ee::s04::f_001da530,
            c_fe_state_net_help_popup_set_body_text,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c10_000a ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            },
        );
    }

    use crate::difftest::fakes::machine;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD;
    pub(crate) use crate::game::fe::fepopup::tests::popup_v24_machine;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;

    #[test]
    fn c_fe_state_net_help_popup_on_widget_action_matches() {
        check(
            ssx3_ee::s04::f_001da478__generated,
            c_fe_state_net_help_popup_on_widget_action,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0xac, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.g[A2][0] = if seed % 3 == 0 { 6 } else { 7 };
                c.bus.write32(DATA + 0x10, DATA + 0x3000);
            },
        );
    }

    #[test]
    fn c_fe_state_net_help_popup_v24_matches() {
        check(
            ee::s04::f_001d9e20__generated,
            c_fe_state_net_help_popup_v24,
            120,
            &|c, seed| {
                popup_v24_machine(c, seed, 0x1d9e_20, 0x50);
            },
        );
    }

    #[test]
    fn c_fe_state_net_help_popup_init_matches() {
        check(
            ssx3_ee::s04::f_001d9db8__generated,
            c_fe_state_net_help_popup_init,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x1d9db8);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }

    #[test]
    fn c_fe_state_net_help_popup_clear_fields_matches() {
        check(
            ssx3_ee::s04::f_001da4a8__generated,
            c_fe_state_net_help_popup_clear_fields,
            100,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1da4a8);
                c.g[A0][0] = DATA as u64;
                for i in 0..0x60u32 / 4 {
                    c.bus.write32(DATA + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn cfe_state_net_help_popup_update_matches() {
        check(
            ee::s04::f_001d9e68__generated,
            cfe_state_net_help_popup_update,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d9e, 3);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                for i in 0..2u32 {
                    if seed % (i + 2) != 0 {
                        let bind = DATA + 0x2000 + 0x40 * i;
                        c.bus.write32(WORLD + 0xb0 + 4 * i, bind);
                        c.bus.write32(bind, DATA + 0x2400 + 0x40 * i);
                        c.bus.write32(DATA + 0x2400 + 0x40 * i, (seed % 2) as u32);
                    }
                }
                let _ = &mut r;
            },
        );
    }

    use crate::difftest::fakes;

    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::fe::festatetrophyroom::tests::oc943_open;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::oc943_widget;

    pub(crate) use crate::game::fe::festatetrophyroom::tests::oc946_install_fakes;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::oc946_setup;

    pub(crate) use crate::game::fe::festatetrophyroom::tests::OC946_MBX;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::OC946_OBJ;

    /// `TransitionIn(self, first)`: nothing on a repeat call; the first call sets the three
    /// texts (string id or empty), hides the nine icons and fills the rows below the state
    /// (row shown, icon shown and moved down 22 per row, row text).
    #[test]
    fn c_fe_state_net_help_popup_transition_in_matches() {
        oc943_install_fakes();
        fakes::covered(
            ee::s04::f_001da238__generated,
            c_fe_state_net_help_popup_transition_in,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1da238, 40);
                oc943_open(c, seed);
                let this = DATA + 0x1000;
                c.bus.write32(this + 0x54, (seed % 2) * r.next_u32());
                c.bus.write32(this + 0x58, (seed / 2) % 2);
                c.bus.write32(this + 0x64, (seed / 4) % 2 * r.next_u32());
                for (k, off) in [0xa8u32, 0xac, 0xb0].into_iter().enumerate() {
                    let widget = DATA + 0x4000 + 0x100 * k as u32;
                    c.bus.write32(this + off, widget);
                    oc943_widget(c, widget);
                }
                c.bus.write32(this + 0x48, (seed / 8) % 9);
                for i in 0..7u32 {
                    let row = DATA + 0x6000 + 0x100 * i;
                    c.bus.write32(this + 0xb4 + 4 * i, row);
                    oc943_widget(c, row);
                    c.bus.write32(
                        this + 0x68 + 4 * i,
                        if (seed + i) % 3 == 0 { 0 } else { 0x77 + i },
                    );
                    c.bus.write32(this + 0x84 + 4 * i, (seed + i * 3) % 9);
                }
                for j in 0..9u32 {
                    let icon = DATA + 0x5000 + 0x100 * j;
                    c.bus.write32(this + 0xd0 + 4 * j, icon);
                    oc943_widget(c, icon);
                    for k in 0..3 {
                        c.bus.write32(icon + 0x44 + 4 * k, r.f32(100.0));
                    }
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 5 == 0 { 0 } else { 1 };
                c.pc = 0x001d_a238;
            },
        );
    }

    #[test]
    fn c_fe_state_net_help_popup_build_body_text_matches() {
        oc946_install_fakes();
        check(
            ee::s04::f_001da110__generated,
            c_fe_state_net_help_popup_build_body_text,
            200,
            &|c, seed| {
                let mut r = oc946_setup(c, seed, 0xa110);
                let ssx_slot = oc946_gp(c, 0x848);
                c.g[A0][0] = sx(OC946_OBJ + 0x2000);
                let this = OC946_OBJ + 0x2000;
                // the ids: 0 to 3 of them, the list stops at the first zero
                let n = (seed % 4) as u32;
                for i in 0..3u32 {
                    let id = if i < n { 0x100 + i } else { 0 };
                    c.bus.write32(this + 0x58 + 4 * i, id);
                }
                c.bus.write32(ssx_slot, OC946_OBJ + 0x800);
                c.bus.write32(OC946_OBJ + 0x800 + 0x8c, OC946_OBJ + 0x1000);
                c.bus.write32(OC946_OBJ + 0x1000 + 4, OC946_OBJ + 0x1400);
                c.bus.write16(OC946_OBJ + 0x1400 + 0x20, 0x10);
                c.bus.write32(OC946_OBJ + 0x1400 + 0x24, 0x0014_dc80);
                c.bus.write32(this + 0xac, OC946_OBJ + 0x1800);
                c.bus
                    .write32(OC946_MBX + 8, if seed % 5 == 0 { 0 } else { 5 + seed % 7 });
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn c_fe_state_net_help_popup_set_num_buttons_matches() {
        check(
            ee::s04::f_001da528__generated,
            c_fe_state_net_help_popup_set_num_buttons,
            100,
            &|c, seed| {
                let mut r = Rng(0xda528 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(OC946_OBJ);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_init_create_account_page_matches() {
        oc946_install_fakes();
        check(
            ee::s04::f_001da648__generated,
            lobby_menu_init_create_account_page,
            200,
            &|c, seed| {
                let mut r = oc946_setup(c, seed, 0xa648);
                let ssx_slot = oc946_gp(c, 0x848);
                c.g[A0][0] = sx(OC946_OBJ + 0x2000);
                c.g[A1][0] = sx(OC946_OBJ + 0x2400);
                c.g[A2][0] = sx(OC946_OBJ + 0x2c00);
                c.bus.write32(OC946_OBJ + 0x2400, OC946_OBJ + 0x2800);
                c.bus.write32(ssx_slot, OC946_OBJ + 0x800);
                c.bus.write32(OC946_OBJ + 0x800 + 0xc4, r.next_u32());
                c.bus.write32(OC946_OBJ + 0x800 + 0xc8, r.next_u32());
            },
        );
    }

    /// The lobby menu object the `LobbyMenu_Init*Page` bodies fill.
    pub(crate) const OBJ: u32 = DATA + 0x9_0000;

    /// The world pointer at `gp - 0x848`; its `+0xc4` and `+0xc8` are text pointers.
    pub(crate) const WORLD_2: u32 = DATA + 0x9_8000;

    /// A zeroed text buffer (the world's text pointers point here).
    pub(crate) const TEXT: u32 = DATA + 0x9_8100;

    /// An argument word the bodies read through `a1`.
    pub(crate) const ARG: u32 = DATA + 0x9_8200;

    /// A scrambled machine with the allocator, the world and text fixture,
    /// and the four argument registers `args`.
    pub(crate) fn init_pages_setup(c: &mut Cpu, seed: u32, salt: u64, args: [u64; 4]) {
        fakes::machine_rt(c, seed, salt, 0);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD_2);
        c.bus.write32(WORLD_2 + 0xc4, TEXT);
        c.bus.write32(WORLD_2 + 0xc8, TEXT + 0x40);
        c.bus.write32(ARG, seed);
        c.bus.write32(ARG + 4, seed.wrapping_mul(0x9e37));
        c.g[A0][0] = args[0];
        c.g[A1][0] = args[1];
        c.g[A2][0] = args[2];
        c.g[A3][0] = args[3];
    }

    /// `a2` as a pointer (even seeds) or a plain word (odd seeds).
    pub(crate) fn init_pages_arg2(seed: u32) -> u64 {
        if seed % 2 == 0 {
            (ARG + 0x10) as u64
        } else {
            seed as u64
        }
    }

    #[test]
    fn lobby_menu_init_login_page_matches() {
        check(
            ssx3_ee::s04::f_001da7e8__generated,
            lobby_menu_init_login_page,
            20,
            &|c, seed| {
                init_pages_setup(
                    c,
                    seed,
                    0x1da7e8,
                    [OBJ as u64, ARG as u64, init_pages_arg2(seed), 0],
                );
            },
        );
    }

    #[test]
    fn lobby_menu_init_login_method_page_matches() {
        check(
            ssx3_ee::s04::f_001da988__generated,
            lobby_menu_init_login_method_page,
            20,
            &|c, seed| {
                init_pages_setup(
                    c,
                    seed,
                    0x1da988,
                    [OBJ as u64, ARG as u64, init_pages_arg2(seed), 0],
                );
            },
        );
    }

    #[test]
    fn lobby_menu_init_create_persona_page_matches() {
        check(
            ssx3_ee::s04::f_001daac8__generated,
            lobby_menu_init_create_persona_page,
            20,
            &|c, seed| {
                init_pages_setup(c, seed, 0x1daac8, [OBJ as u64, ARG as u64, 0, 0]);
            },
        );
    }

    #[test]
    fn lobby_menu_init_delete_persona_page_matches() {
        check(
            ssx3_ee::s04::f_001dabd0__generated,
            lobby_menu_init_delete_persona_page_2,
            20,
            &|c, seed| {
                init_pages_setup(c, seed, 0x1dabd0, [OBJ as u64, ARG as u64, 0, 0]);
            },
        );
    }

    #[test]
    fn lobby_menu_init_select_persona_page_matches() {
        check(
            ssx3_ee::s04::f_001dacb8__generated,
            lobby_menu_init_select_persona_page,
            20,
            &|c, seed| {
                init_pages_setup(
                    c,
                    seed,
                    0x1dacb8,
                    [
                        OBJ as u64,
                        ARG as u64,
                        init_pages_arg2(seed),
                        (ARG + 0x20) as u64,
                    ],
                );
            },
        );
    }

    #[test]
    fn lobby_menu_init_select_room_page_matches() {
        check(
            ssx3_ee::s04::f_001dae20__generated,
            lobby_menu_init_select_room_page_2,
            20,
            &|c, seed| {
                init_pages_setup(c, seed, 0x1dae20, [OBJ as u64, ARG as u64, 0, 0]);
            },
        );
    }

    #[test]
    fn lobby_menu_init_room_occupants_page_matches() {
        check(
            ssx3_ee::s04::f_001daf08__generated,
            lobby_menu_init_room_occupants_page,
            20,
            &|c, seed| {
                init_pages_setup(
                    c,
                    seed,
                    0x1daf08,
                    [OBJ as u64, ARG as u64, init_pages_arg2(seed), 0],
                );
            },
        );
    }

    #[test]
    fn lobby_menu_init_chat_page_matches() {
        check(
            ssx3_ee::s04::f_001db028__generated,
            lobby_menu_init_chat_page,
            20,
            &|c, seed| {
                init_pages_setup(c, seed, 0x1db028, [OBJ as u64, ARG as u64, 0, 0]);
            },
        );
    }
}
