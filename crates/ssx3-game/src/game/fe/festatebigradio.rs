//! `src/fe/festatebigradio.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::feasyncfile::net_fe_state_ctor;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::festaterewards::gp_4 as gp_2;
pub(crate) use crate::game::fe::ovstatepause::p;
pub(crate) use crate::game::wscript::wscriptman::gp;
use crate::guest::Addr;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0019_3568,
        cfe_state_bragging_rights_on_activate,
        "cFEStateBraggingRights_OnActivate",
    ),
    (
        0x0019_4080,
        fe_state_bragging_rights_start_wait_anim,
        "cFEStateBraggingRights_StartWaitAnim",
    ),
    (
        0x0019_4440,
        c_fe_state_bragging_rights_on_child_event_quit,
        "cFEStateBraggingRights_OnChildEventQuit",
    ),
    (
        0x0019_4498,
        ui_object_show_clamp_alpha_half,
        "UIObject_ShowClampAlphaHalf",
    ),
    (
        0x001a_8918,
        net_fe_state_on_widget_action,
        "NetFEState_OnWidgetAction",
    ),
    (0x001a_9bc0, net_fe_state_show_popup, "NetFEState_ShowPopup"),
    (
        0x001b_fbb8,
        c_fe_state_online_main_menu_ctor,
        "cFEStateOnlineMainMenu_ctor",
    ),
    (0x0039_e4c0, ui_state_on_activate, "UIState_OnActivate"),
];

// ---------------------------------------------------------------------------
// Direct-call targets (the generated body's `c.pc = addr; Name(c)` calls).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::POPUP_CONFIRM_REBUILD;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;

    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;

    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;

    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;

    pub(crate) use crate::game::fe::festaterewards::callee::BRAGGING_RIGHTS_UPDATE_ROW_COLORS;

    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_ON_CHILD_EVENT;

    pub(crate) use crate::game::wscript::wscriptman::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_EMPTY_CHILD_HANDLER;

    pub(crate) use crate::game::wscript::wscriptman::callee::C_SETUP_MAN_SEND_QUIT;

    pub const UI_LIST_FIRST: Callee = Callee {
        addr: 0x0039_7840,
        f: ee::s13::f_00397840,
    };
}

/// `void* cFEStateOnlineMainMenu_ctor(void* self)` (0x1bfbb8): class 0x3b.
pub fn c_fe_state_online_main_menu_ctor(c: &mut Cpu) {
    net_fe_state_ctor(c, 0x3b, 0x46_ad48, 0x001b_fbd0)
}

/// `void UIState_OnActivate(state)` (0x39e4c0): notify the state's vtable slot
/// 0x68/0x6c of the first child widget (`list at +0x24`, child `+0x40`).
pub fn ui_state_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
        UI_LIST_FIRST.call(c, 0x0039_e4d8)?;
        c.g[A1][0] = c.g[V0][0];
        let found = c.g[A1][0] != 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
        if found {
            c.cycles += 5;
            let this = c.g[S0][0] as u32;
            c.g[V0][0] = Addr(this).off(8).lw(c);
            let vt = c.g[V0][0] as u32;
            c.g[A0][0] = Addr(vt).off(0x68).lh(c);
            c.g[V1][0] = Addr(vt).off(0x6c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0039_e4f8);
            c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0039_e4f8)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBraggingRights_OnChildEventQuit(cFEState* self, int a,
/// int event)` (0x194440): the quit child event: tell the setup manager to
/// quit and empty the child, otherwise forward the event.
pub fn c_fe_state_bragging_rights_on_child_event_quit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xf;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let quit = c.g[A2][0] != c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if !quit {
            c.cycles += 3;
            c.g[A0][0] = Addr(gp(c, 0x2dc)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 2;
                c.g[A1][0] = 0xf9;
                callee::C_SETUP_MAN_SEND_QUIT.call(c, 0x0019_446c)?;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_EMPTY_CHILD_HANDLER.call(c, 0x0019_4474)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x0019_4484)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void NetFEState_ShowPopup(NetFEState* self, Popup* popup)` (0x1a9bc0):
/// raise the popup's owning state or push the popup and rebuild its confirm
/// widget.
pub fn net_fe_state_show_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x20));
        c.sq(17, at(sp0, 0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd(sp0, c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = lw_at(c, c.g[S1][0], 0x40);
        c.g[V1][0] = lw_at(c, c.g[V0][0], 0xd0);
        c.g[A0][0] = lw_at(c, c.g[V1][0], 0x10);
        c.g[A0][0] = sx(at(c.g[A0][0] as u32, 0x18));
        call_ptr(c, callee::UI_LIST_FIRST_WRAPPER, 0x001a_9bec)?;
        c.g[V1][0] = c.g[V0][0];
        let popup = c.g[S0][0];
        c.g[V0][0] = lw_at(c, popup, 0x2b8);
        let owned = c.g[V0][0] != 0;
        // The final `+1; lq16` runs on every path except the push.
        let mut charge_one = true;
        if owned {
            c.g[V0][0] = lw_at(c, popup, 0x1c);
        } else {
            c.cycles += 2;
            let same = popup == c.g[V1][0];
            c.g[A1][0] = c.g[S0][0];
            if !same {
                c.cycles += 7;
                c.g[V0][0] = lw_at(c, c.g[S1][0], 0x40);
                c.g[V1][0] = lw_at(c, c.g[V0][0], 0xd0);
                c.g[A0][0] = lw_at(c, c.g[V1][0], 0x10);
                c.g[A0][0] = sx(at(c.g[A0][0] as u32, 0x18));
                call_ptr(c, callee::UI_STATE_MGR_PUSH_POPUP, 0x001a_9c18)?;
                c.lq(16, at(sp0, 0x20));
                charge_one = false;
            }
        }
        if owned {
            c.cycles += 6;
            c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x500;
            c.sw(at(popup as u32, 0x1c), c.g[V0][0]);
            c.g[A0][0] = lw_at(c, c.g[S1][0], 0x6a0);
            callee::POPUP_CONFIRM_REBUILD.call(c, 0x001a_9c38)?;
        }
        if charge_one {
            c.cycles += 1;
            c.lq(16, at(sp0, 0x20));
        }
        c.cycles += 4;
        c.lq(17, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI object alpha.

/// `void UIObject_ShowClampAlphaHalf(UIObject* self)` (0x194498): show the
/// object and clamp its colour alpha to 0.5.
pub fn ui_object_show_clamp_alpha_half(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = 1;
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0x40)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x44)).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
        call_ptr(c, target, 0x0019_44c0)?;
        c.ldl(2, p(r32(c, S0), 0x23));
        c.ldr(2, p(r32(c, S0), 0x1c));
        c.ldl(3, p(r32(c, S0), 0x2b));
        c.ldr(3, p(r32(c, S0), 0x24));
        c.sdl(2, p(r32(c, SP), 0x7));
        c.sdr(2, r32(c, SP));
        c.sdl(3, p(r32(c, SP), 0xf));
        c.sdr(3, p(r32(c, SP), 8));
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 1);
        c.lwc1(0, r32(c, SP));
        c.c_lt_s(1, 0);
        let clamped = c.fcc();
        c.g[A1][0] = c.g[SP][0];
        if clamped {
            c.cycles += 6;
            c.swc1(1, r32(c, SP));
            c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x58)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x5c)).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, target, 0x0019_4514)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void NetFEState_OnWidgetAction(self, item, msg)` (0x1a8918): on a `msg`
/// of 7, build the widget object, flag it and add it to the screen.
pub fn net_fe_state_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        let _msg = abi::arg64(c, 2);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = sx(self_);
        let no_widget = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        let mut tail = no_widget;
        if !no_widget {
            c.cycles += 3;
            c.g[V0][0] = 7;
            let other = c.g[A2][0] != c.g[V0][0];
            if !other {
                c.cycles += 23;
                c.g[A1][0] = 0x0046_0000;
                c.g[A3][0] = 0;
                c.g[A1][0] = sx(p(r32(c, A1), 0x1a70));
                c.g[A0][0] = 0x140;
                c.g[A2][0] = 0x100;
                call_ptr(c, callee::MEM_ALLOC, 0x001a_8954)?;
                c.g[A1][0] = c.lw(at_2(c, S1, 0x10));
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2, 0x001a_8964)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = 1;
                c.g[A1][0] = c.g[S0][0];
                c.sw(at_2(c, S0, 0x50), c.g[V0][0]);
                c.g[V0][0] = c.lw(at_2(c, S1, 8));
                c.g[A0][0] = c.lh(at_2(c, V0, 0xc8));
                c.g[V1][0] = c.lw(at_2(c, V0, 0xcc));
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                let target = r32(c, V1);
                call_ptr(c, target, 0x001a_8988)?;
                c.g[A0][0] = c.lw(at_2(c, S1, 0x10));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
                call_ptr(c, callee::UI_OBJECT_ADD_CHILD, 0x001a_8998)?;
                tail = true;
            }
        }
        if tail {
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
// FE state: bragging rights.

/// `void cFEStateBraggingRights_StartWaitAnim(cFEStateBraggingRights* s)`
/// (0x194080): start the idle "wait" animation of the screen's primary thread
/// — either the widget stored at `+0x6f8`/`+0x72c`, or the screen's `+0x40`
/// thread object, looking the animation up by hash.
pub fn fe_state_bragging_rights_start_wait_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        'outer: {
            'mid2: {
                'mid: {
                    frame.save(c, S0, 0x10);
                    frame.save_ra(c, 0);
                    let this = r(c, A0);
                    c.g[S0][0] = sx(this);
                    c.g[V0][0] = Addr(p(this, 0x6f8)).lw(c);
                    let has_anim = c.g[V0][0] == 0;
                    c.g[V0][0] = Addr(p(this, 0x72c)).lw(c);
                    if has_anim {
                        break 'mid;
                    }
                    'inner: {
                        c.cycles += 2;
                        if c.g[V0][0] != 0 {
                            c.g[V0][0] = Addr(p(r(c, S0), 0x6f8)).lw(c);
                            break 'inner;
                        }
                        c.cycles += 10;
                        c.g[A0][0] = sx(0x004a_1790);
                        c.g[RA][0] = sx(0x0019_40b0);
                        call_ptr(c, callee::FE_STRING_HASH, 0x0019_40b0)?;
                        c.g[A0][0] = Addr(p(r(c, S0), 0x40)).lw(c);
                        c.g[RA][0] = sx(0x0019_40bc);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039_c778, 0x0019_40bc)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        let missing = c.g[A1][0] == c.g[V0][0];
                        c.g[A1][0] = c.g[A1][0] & 0xffff;
                        if missing {
                            break 'mid2;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = Addr(p(r(c, S0), 0x40)).lw(c);
                        c.g[RA][0] = sx(0x0019_40d8);
                        c.g[A2][0] = 1;
                        call_ptr(c, 0x0039_c870, 0x0019_40d8)?;
                        frame.restore(c, 16, 0x10);
                        break 'outer;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        frame.restore(c, 16, 0x10);
                        break 'outer;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = Addr(p(r(c, S0), 0x72c)).lw(c);
                    break 'mid;
                }
                c.cycles += 2;
                let idle = c.g[V0][0] == 0;
                c.g[A0][0] = sx(0x004a_0000);
                if idle {
                    break 'mid2;
                }
                c.cycles += 9;
                c.g[RA][0] = sx(0x0019_40fc);
                c.g[A0][0] = sx(0x004a_1798);
                call_ptr(c, callee::FE_STRING_HASH, 0x0019_40fc)?;
                c.g[A0][0] = Addr(p(r(c, S0), 0x40)).lw(c);
                c.g[RA][0] = sx(0x0019_4108);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_c778, 0x0019_4108)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = 0xffff;
                let missing = c.g[A1][0] == c.g[V0][0];
                c.g[A1][0] = c.g[A1][0] & 0xffff;
                if missing {
                    break 'mid2;
                }
                c.cycles += 3;
                c.g[A0][0] = Addr(p(r(c, S0), 0x40)).lw(c);
                c.g[RA][0] = sx(0x0019_4124);
                c.g[A2][0] = 1;
                call_ptr(c, 0x0039_c870, 0x0019_4124)?;
                break 'mid2;
            }
            c.cycles += 1;
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Bragging rights screen.

/// `void cFEStateBraggingRights_OnActivate(cFEStateBraggingRights* self)`
/// (0x193568): when the screen is enabled, play the intro or the idle frame
/// animation depending on the state object's fields, then refresh both row
/// colour tables; when disabled, hand activation to the net state.
pub fn cfe_state_bragging_rights_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(gp_2(c, -0x1680)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.g[V0][0] ^ 1;
        let on = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A0][0];

        'outer: {
            if !on {
                c.cycles += 4;
                call_ptr(c, 0x001a_87d0, 0x0019_358c)?;
                frame.restore(c, 16, 0x10);
                break 'outer;
            }

            'frames: {
                'first: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039_e4c0, 0x0019_359c)?;
                    c.g[V0][0] = Addr(r(c, S0)).off(0x6f8).lw(c);
                    let idle = c.g[V0][0] == 0;
                    c.g[V0][0] = Addr(r(c, S0)).off(0x72c).lw(c);
                    if idle {
                        break 'first;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = Addr(r(c, S0)).off(0x6f8).lw(c);
                    } else {
                        c.cycles += 10;
                        c.g[A0][0] = 0x4a1790;
                        call_ptr(c, 0x0031_7670, 0x0019_35bc)?;
                        c.g[A0][0] = Addr(r(c, S0)).off(0x40).lw(c);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039_c778, 0x0019_35c8)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        let missing = c.g[A1][0] == c.g[V0][0];
                        c.g[A1][0] = c.g[A1][0] & 0xffff;
                        if missing {
                            break 'frames;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = Addr(r(c, S0)).off(0x40).lw(c);
                        c.g[A2][0] = 1;
                        call_ptr(c, 0x0039_c870, 0x0019_35e4)?;
                        break 'frames;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        break 'frames;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = Addr(r(c, S0)).off(0x72c).lw(c);
                }

                c.cycles += 2;
                let idle = c.g[V0][0] == 0;
                c.g[A0][0] = 0x4a0000;
                if idle {
                    break 'frames;
                }
                c.cycles += 9;
                c.g[A0][0] = sx(r(c, A0).wrapping_add(0x1798));
                call_ptr(c, 0x0031_7670, 0x0019_3608)?;
                c.g[A0][0] = Addr(r(c, S0)).off(0x40).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_c778, 0x0019_3614)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = 0xffff;
                let missing = c.g[A1][0] == c.g[V0][0];
                c.g[A1][0] = c.g[A1][0] & 0xffff;
                if missing {
                    break 'frames;
                }
                c.cycles += 3;
                c.g[A0][0] = Addr(r(c, S0)).off(0x40).lw(c);
                c.g[A2][0] = 1;
                call_ptr(c, 0x0039_c870, 0x0019_3630)?;
            }

            c.cycles += 5;
            c.g[A0][0] = sx(p(r(c, S0), 0x6d0));
            callee::BRAGGING_RIGHTS_UPDATE_ROW_COLORS.call(c, 0x0019_3638)?;
            c.g[A0][0] = sx(p(r(c, S0), 0x704));
            callee::BRAGGING_RIGHTS_UPDATE_ROW_COLORS.call(c, 0x0019_3640)?;
            frame.restore(c, 16, 0x10);
        }

        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;

    use crate::difftest::fakes::LOG_RESULTS;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::be::beintstat::tests::allocator;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::env;

    pub(crate) use crate::game::fe::feasyncfile::tests::net_fe_state_ctor_test;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_6 as slot;

    pub(crate) use crate::game::fe::feasyncfile::tests::w32;

    pub(crate) use crate::game::fe::feasyncfile::tests::EMPTY;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_2;

    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_3;

    pub(crate) use crate::game::fe::festaterewards::tests::strict;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;

    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::gp_at;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn c_fe_state_online_main_menu_ctor_matches() {
        net_fe_state_ctor_test(
            ssx3_ee::s03::f_001bfbb8__generated,
            c_fe_state_online_main_menu_ctor,
            0x16,
        );
    }

    // -----------------------------------------------------------------------
    // 0x39e4c0 UIState_OnActivate.

    #[test]
    fn ui_state_on_activate_matches() {
        check(
            ee::s13::f_0039e4c0__generated,
            ui_state_on_activate,
            16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39e4_c0, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                if seed % 2 == 0 {
                    c.bus.write32(this + 0x28, 0);
                } else {
                    let node = DATA + 0x300;
                    c.bus.write32(this + 0x28, node);
                    c.bus.write32(node + 4, r.next_u32() | 1);
                    let vt = vtable(c, &mut r, &[0x68]);
                    c.bus.write32(this + 8, vt);
                }
            },
        );
    }

    #[test]
    fn c_fe_state_bragging_rights_on_child_event_quit_matches() {
        check(
            ee::s02::f_00194440__generated,
            c_fe_state_bragging_rights_on_child_event_quit,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1944_40, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let setup = DATA + 0x800;
                clear(c, this, 0x100);
                clear(c, setup, 0x40);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0xf
                } else {
                    (seed % 16) as u64
                };
                c.bus
                    .write32(gp_at(0x2dc), if seed % 3 == 0 { 0 } else { setup });
            },
        );
    }

    #[test]
    fn net_fe_state_show_popup_matches() {
        check(
            ee::s03::f_001a9bc0__generated,
            net_fe_state_show_popup,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1a9bc0, 3);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                let screen = DATA + 0x2000;
                c.bus.write32(DATA + 0x40, screen);
                c.bus.write32(screen + 0xd0, DATA + 0x2100);
                c.bus.write32(DATA + 0x2100 + 0x10, DATA + 0x2200);
                c.bus.write32(DATA + 0x2200 + 0x18, 0);
                let vt = vtable(c, &mut r, &[0x18]);
                c.bus.write32(
                    LOG_RESULTS,
                    if seed % 3 == 0 {
                        DATA + 0x2200
                    } else {
                        DATA + 0x2300
                    },
                );
                c.bus.write32(DATA + 0x1000 + 0x2b8, (seed % 2) as u32);
                c.bus.write32(DATA + 0x1000 + 0x1c, 0x1234);
                c.bus.write32(DATA + 0x6a0, 0);
                let _ = vt;
            },
        );
    }

    #[test]
    fn ui_object_show_clamp_alpha_half_matches() {
        check(
            ee::s02::f_00194498__generated,
            ui_object_show_clamp_alpha_half,
            100,
            &|c, seed| {
                let mut r = base(c, seed, 0x19_4498);
                globals_2(c);
                let vt = vtable(c, &mut r, &[0x40, 0x58]);
                w(c, DATA + 8, vt);
                // The colour at +0x1c: x drives the clamp.
                c.bus.write32(
                    DATA + 0x1c,
                    if seed % 2 == 0 {
                        0x3f80_0000
                    } else {
                        0x3e80_0000
                    },
                );
                c.bus.write32(DATA + 0x20, 0x3f00_0000);
                c.bus.write32(DATA + 0x24, 0x3e80_0000);
                c.bus.write32(DATA + 0x28, 0x3f40_0000);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn net_fe_state_on_widget_action_matches() {
        check(
            ee::s03::f_001a8918__generated,
            net_fe_state_on_widget_action,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x1a89, 0x18);
                env(c);
                let this = DATA + 0x400;
                let item = DATA + 0x800;
                let vt = DATA + 0x1000;
                slot(c, vt, 0xc8, EMPTY);
                w32(c, this + 0x10, DATA + 0x1100);
                w32(c, this + 8, vt);
                w32(c, item + 0x18, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(item) };
                c.g[A2][0] = [7u64, 0, 9][(seed % 3) as usize];
            },
        );
    }

    // ------------------------------------------------------------ FE states

    #[test]
    fn fe_state_bragging_rights_start_wait_anim_matches() {
        check(
            ee::s02::f_00194080__generated,
            fe_state_bragging_rights_start_wait_anim,
            SEEDS,
            &|c, seed| {
                base_2(c, seed, 0x1940, 2);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0x6f8, seed & 1);
                w(c, DATA + 0x72c, (seed >> 1) & 1);
                w(c, DATA + 0x40, 0);
            },
        );
    }

    // ---------------------------------------------------- bragging rights

    #[test]
    fn cfe_state_bragging_rights_on_activate_matches() {
        strict(
            ee::s02::f_00193568__generated,
            cfe_state_bragging_rights_on_activate,
            90,
            &|c, seed| {
                let _ = base_3(c, seed, 0x1935, 6);
                allocator(c);
                c.g[A0][0] = DATA as u64;
                w(c, GAME_GP.wrapping_sub(0x1680), (seed % 2) as u32);
                w(c, DATA + 0x40, DATA + 0x800);
                w(c, DATA + 0x6f8, if seed % 3 == 0 { 0 } else { 1 });
                w(c, DATA + 0x72c, if seed % 4 < 2 { 0 } else { 1 });
                // A little screen data so the frame lookups differ.
                w(c, DATA + 0x800 + 0x38, DATA + 0x900);
                w(c, DATA + 0x900, 2);
                w(c, DATA + 0x804, (seed % 5) as u32);
            },
        );
    }
}
