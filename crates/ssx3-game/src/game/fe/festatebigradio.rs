//! `src/fe/festatebigradio.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
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
    (
        0x0019_2d80,
        cfe_state_bragging_rights_init,
        "cFEStateBraggingRights_Init",
    ),
    (
        0x0019_3318,
        c_fe_state_bragging_rights_render,
        "cFEStateBraggingRights_Render",
    ),
    (
        0x0019_3338,
        c_fe_state_bragging_rights_update,
        "cFEStateBraggingRights_Update",
    ),
    (
        0x0019_3650,
        c_fe_state_bragging_rights_on_widget_created,
        "cFEStateBraggingRights_OnWidgetCreated",
    ),
    (
        0x0019_38f8,
        c_fe_state_bragging_rights_on_widget_action,
        "cFEStateBraggingRights_OnWidgetAction",
    ),
    (
        0x0019_3c00,
        c_fe_state_bragging_rights_v25,
        "cFEStateBraggingRights_v25",
    ),
    (
        0x0019_3cf8,
        c_fe_state_bragging_rights_on_widget_notify,
        "cFEStateBraggingRights_OnWidgetNotify",
    ),
    (
        0x0019_3de0,
        c_fe_state_bragging_rights_dispatch_row_action,
        "cFEStateBraggingRights_DispatchRowAction",
    ),
    (
        0x0019_3f38,
        c_fe_state_bragging_rights_get_row_default,
        "cFEStateBraggingRights_GetRowDefault",
    ),
    (
        0x0019_4138,
        bragging_rights_v24,
        "cFEStateBraggingRights_v24",
    ),
    (
        0x0019_4168,
        bragging_rights_on_child_event,
        "cFEStateBraggingRights_OnChildEvent",
    ),
    (
        0x0019_41b0,
        bragging_rights_v31,
        "cFEStateBraggingRights_v31",
    ),
    (
        0x0019_3310,
        c_fe_state_bragging_rights_transition_in_00193310,
        "cFEStateBraggingRights_TransitionIn",
    ),
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

    /// `cFEStateBraggingRights_SetRowsVisible` (0x192740).
    pub(crate) const SET_ROWS_VISIBLE: Callee = Callee {
        addr: 0x0019_2740,
        f: ee::s02::f_00192740,
    };

    /// `cFEStateBraggingRights_SetRowMask` (0x192968).
    pub(crate) const SET_ROW_MASK: Callee = Callee {
        addr: 0x0019_2968,
        f: ee::s02::f_00192968,
    };

    /// `cFEStateBraggingRights_HideUnusedRows` (0x192970).
    pub(crate) const HIDE_UNUSED_ROWS: Callee = Callee {
        addr: 0x0019_2970,
        f: ee::s02::f_00192970,
    };

    /// `cFEStateBraggingRights_GetRowDefault` (0x193f38).
    pub(crate) const GET_ROW_DEFAULT: Callee = Callee {
        addr: 0x0019_3f38,
        f: ee::s02::f_00193f38,
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

/// `void cFEStateBraggingRights_Init(cFEStateBraggingRights* self)` (0x192d80).
///
/// Opens the "bragging rights" screen (`self+0x40`), hides the two player
/// menus when the attribute bit 4 is set, picks the row table (`self+0x738`)
/// and the row count of the current game event (jump table 0x45fd20, six
/// events), then for every row sets its label text and fills the two list
/// boxes (No/Yes, Off/5s..20s or Off/3000..15000 entries) with the current
/// back-end value as selection. Finally shows or hides the net-play widgets
/// by the net flag at `gp - 0x1680`.
pub fn cfe_state_bragging_rights_init(c: &mut Cpu) {
    if c.pc != 0x0019_2d80 {
        return cfe_state_bragging_rights_init_resume(c);
    }
    guest(c, |c| 'l_192d80: loop {
        'b_192dec: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[A0][0] = 0x460000;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x338));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x00192dc0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x00192dd4)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_192dec;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x00192dec)?;
                break 'b_192dec;
            }
        }
        'b_192e1c: {
            c.cycles += 6;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
            c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6d0));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            let t = c.g[V0][0] == 0u64;
            c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x704));
            if t {
                break 'b_192e1c;
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = 0u64;
                callee::SET_ROWS_VISIBLE.call(c, 0x00192e10)?;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = 0u64;
                callee::SET_ROWS_VISIBLE.call(c, 0x00192e1c)?;
                break 'b_192e1c;
            }
        }
        'b_192ea8: {
            c.cycles += 10;
            c.g[S5][0] = 0u64;
            call_ptr(c, 0x0014dc80, 0x00192e24)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x00192e30)?;
            c.g[V0][0] = 0x530000;
            c.g[A0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c12));
            c.g[V1][0] = (c.g[A0][0] < 0x6u64) as u64;
            let t = c.g[V1][0] == 0u64;
            c.g[S0][0] = 0u64;
            if t {
                break 'b_192ea8;
            } else {
                'b_192ea4: {
                    'b_192e9c: {
                        'b_192e90: {
                            'b_192e84: {
                                'b_192e78: {
                                    'b_192e6c: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x460000;
                                        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2e0));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x00192d80 => {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00192d80);
                                                }
                                                continue 'l_192d80;
                                            }
                                            0x00192e60 => {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    sx((c.g[GP][0] as u32).wrapping_sub(0x19e0));
                                                let t = true;
                                                c.g[S5][0] = 0x6;
                                                if t {
                                                    break 'b_192ea4;
                                                } else {
                                                    break 'b_192e6c;
                                                }
                                            }
                                            0x00192e6c => {
                                                break 'b_192e6c;
                                            }
                                            0x00192e78 => {
                                                break 'b_192e78;
                                            }
                                            0x00192e84 => {
                                                break 'b_192e84;
                                            }
                                            0x00192e90 => {
                                                break 'b_192e90;
                                            }
                                            0x00192e9c => {
                                                break 'b_192e9c;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x00192d80, 0x0019330c);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x19d8));
                                    let t = true;
                                    c.g[S5][0] = 0x7;
                                    if t {
                                        break 'b_192ea4;
                                    } else {
                                        break 'b_192e78;
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x19d0));
                                let t = true;
                                c.g[S5][0] = 0x7;
                                if t {
                                    break 'b_192ea4;
                                } else {
                                    break 'b_192e84;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x19c8));
                            let t = true;
                            c.g[S5][0] = 0x7;
                            if t {
                                break 'b_192ea4;
                            } else {
                                break 'b_192e90;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x19c0));
                        let t = true;
                        c.g[S5][0] = 0x5;
                        if t {
                            break 'b_192ea4;
                        } else {
                            break 'b_192e9c;
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x19b8));
                    c.g[S5][0] = 0x7;
                    break 'b_192ea4;
                }
                c.cycles += 1;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x738), c.g[V0][0]);
                break 'b_192ea8;
            }
        }
        'b_192ed4: {
            c.cycles += 2;
            let t = c.g[S5][0] == 0u64;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_192ed4;
            } else {
                c.cycles += 2;
                c.g[A1][0] = 0x1;
                'l_192eb8: loop {
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[A1][0] as u32) << ((c.g[A0][0] as u32) & 31));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = c.g[S0][0] | c.g[V1][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00192eb8);
                        }
                        continue 'l_192eb8;
                    } else {
                        break 'b_192ed4;
                    }
                }
            }
        }
        'b_193190: {
            c.cycles += 21;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S0][0];
            callee::SET_ROW_MASK.call(c, 0x00192ee0)?;
            c.g[S2][0] = 0u64;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S4][0];
            callee::SET_ROW_MASK.call(c, 0x00192ef0)?;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[A0][0] = c.g[S3][0];
            callee::HIDE_UNUSED_ROWS.call(c, 0x00192efc)?;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[A0][0] = c.g[S4][0];
            callee::HIDE_UNUSED_ROWS.call(c, 0x00192f08)?;
            c.g[V0][0] = 0x4a0000;
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14f8));
            c.g[V1][0] = c.lb(c.g[A2][0] as u32);
            c.g[A0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x1));
            c.sb(c.g[SP][0] as u32, c.g[V1][0]);
            c.sb((c.g[SP][0] as u32).wrapping_add(0x1), c.g[A0][0]);
            let t = c.g[S5][0] == 0u64;
            c.g[A0][0] = 0x4a0000;
            if t {
                break 'b_193190;
            } else {
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x31));
                'l_192f30: loop {
                    'b_192f88: {
                        c.cycles += 9;
                        c.g[A0][0] = c.g[SP][0];
                        c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                        call_ptr(c, 0x00317670, 0x00192f3c)?;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x00192f48)?;
                        c.g[S0][0] = c.g[V0][0];
                        let t = c.g[S0][0] == 0u64;
                        c.g[S3][0] = 0u64;
                        if t {
                            break 'b_192f88;
                        } else {
                            c.cycles += 13;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x738));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                            c.g[V0][0] = 0x440000;
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1108));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                            call_ptr(c, 0x00317670, 0x00192f78)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x00192f84)?;
                            c.g[S3][0] = 0u64;
                            break 'b_192f88;
                        }
                    }
                    c.cycles += 6;
                    c.g[S6][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[S7][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[S4][0] = 0x460000;
                    c.g[FP][0] = 0x4a0000;
                    c.g[V0][0] = 0x34;
                    'l_192fa0: loop {
                        'b_193170: {
                            'b_1930d4: {
                                'b_19309c: {
                                    'b_193030: {
                                        'b_192ff4: {
                                            'b_192fe0: {
                                                c.cycles += 10;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x738));
                                                c.mult(0, 4, 19, 2);
                                                c.sb((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[S2][0] as u32));
                                                c.g[A1][0] = c.lb(c.g[V1][0] as u32);
                                                c.g[V0][0] = sx((c.g[A0][0] as u32)
                                                    .wrapping_add(c.g[S1][0] as u32));
                                                c.g[V1][0] = ((c.g[A1][0] as i64) < 7i64) as u64;
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[S6][0] as u32));
                                                let t = c.g[V1][0] == 0u64;
                                                c.g[S0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x6d8));
                                                if t {
                                                    break 'b_192fe0;
                                                } else {
                                                    c.cycles += 2;
                                                    if (c.g[A1][0] as i64) > 0 {
                                                        c.g[A0][0] = 0x460000;
                                                        break 'b_192ff4;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[A1][0] == 0u64;

                                                        if t {
                                                            break 'b_193030;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[S3][0] = sx((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1));
                                                            if t {
                                                                break 'b_193170;
                                                            } else {
                                                                break 'b_192fe0;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x7;
                                            let t = c.g[A1][0] == c.g[V0][0];

                                            if t {
                                                break 'b_1930d4;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[S3][0] =
                                                    sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                                if t {
                                                    break 'b_193170;
                                                } else {
                                                    break 'b_192ff4;
                                                }
                                            }
                                        }
                                        c.cycles += 15;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                        call_ptr(c, 0x00317670, 0x00192ffc)?;
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[A2][0] = 0u64;
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x0039a568, 0x0019300c)?;
                                        c.g[A0][0] = 0x460000;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                                        call_ptr(c, 0x00317670, 0x00193018)?;
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A2][0] = 0x1;
                                        call_ptr(c, 0x0039a568, 0x00193028)?;
                                        let t = true;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x738));
                                        if t {
                                            break 'b_19309c;
                                        } else {
                                            break 'b_193030;
                                        }
                                    }
                                    c.cycles += 27;
                                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x20c0));
                                    call_ptr(c, 0x00317670, 0x00193038)?;
                                    c.g[A2][0] = 0u64;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x0039a568, 0x00193048)?;
                                    c.g[A2][0] = 0x4a0000;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x16e0));
                                    c.g[A2][0] = 0x5;
                                    call_ptr(c, 0x0039a3f8, 0x0019305c)?;
                                    c.g[V0][0] = 0x4a0000;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x16d8));
                                    c.g[A2][0] = 0xa;
                                    call_ptr(c, 0x0039a3f8, 0x00193070)?;
                                    c.g[V1][0] = 0x4a0000;
                                    c.g[A2][0] = 0xf;
                                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1758));
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x0039a3f8, 0x00193084)?;
                                    c.g[A2][0] = 0x4a0000;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1760));
                                    c.g[A2][0] = 0x14;
                                    call_ptr(c, 0x0039a3f8, 0x00193098)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x738));
                                    break 'b_19309c;
                                }
                                c.cycles += 14;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = c.g[S3][0];
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                                c.g[A2][0] = c.lb(c.g[V1][0] as u32);
                                callee::GET_ROW_DEFAULT.call(c, 0x001930b0)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                call_ptr(c, 0x0039a768, 0x001930c0)?;
                                c.g[A1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x0039a7a8, 0x001930cc)?;
                                let t = true;
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_193170;
                                } else {
                                    break 'b_1930d4;
                                }
                            }
                            c.cycles += 39;
                            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x20c0));
                            call_ptr(c, 0x00317670, 0x001930dc)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A2][0] = 0u64;
                            call_ptr(c, 0x0039a568, 0x001930ec)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1768));
                            c.g[A2][0] = 0xbb8;
                            call_ptr(c, 0x0039a3f8, 0x001930fc)?;
                            c.g[V0][0] = 0x4a0000;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1770));
                            c.g[A2][0] = 0x1388;
                            call_ptr(c, 0x0039a3f8, 0x00193110)?;
                            c.g[V1][0] = 0x4a0000;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1778));
                            c.g[A2][0] = 0x2710;
                            call_ptr(c, 0x0039a3f8, 0x00193124)?;
                            c.g[A1][0] = 0x4a0000;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1780));
                            c.g[A2][0] = 0x3a98;
                            call_ptr(c, 0x0039a3f8, 0x00193138)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x738));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = c.g[S3][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            c.g[A2][0] = c.lb(c.g[V1][0] as u32);
                            callee::GET_ROW_DEFAULT.call(c, 0x00193150)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                            call_ptr(c, 0x0039a768, 0x00193160)?;
                            c.g[A1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0039a7a8, 0x0019316c)?;
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                            break 'b_193170;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S3][0] as i64) < 2i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = 0x34;
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00192fa0);
                            }
                            continue 'l_192fa0;
                        } else {
                            c.cycles += 4;
                            c.g[S2][0] = c.g[S7][0];
                            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S5][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x31));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00192f30);
                                }
                                continue 'l_192f30;
                            } else {
                                c.cycles += 1;
                                c.g[A0][0] = 0x4a0000;
                                break 'b_193190;
                            }
                        }
                    }
                }
            }
        }
        'b_193260: {
            'b_19325c: {
                c.cycles += 8;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1580));
                call_ptr(c, 0x00317670, 0x00193198)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001931a4)?;
                c.g[A2][0] = c.g[V0][0];
                let t = c.g[A2][0] == 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                if t {
                    break 'b_19325c;
                } else {
                    'b_193248: {
                        c.cycles += 3;
                        c.g[V0][0] = c.g[V0][0] ^ 0x1;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        if t {
                            break 'b_193248;
                        } else {
                            'b_193208: {
                                c.cycles += 14;
                                c.g[A1][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001931d0)?;
                                c.g[A0][0] = 0x460000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x318));
                                call_ptr(c, 0x00317670, 0x001931dc)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001931e8)?;
                                c.g[A2][0] = c.g[V0][0];
                                let t = c.g[A2][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_193208;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00193208)?;
                                    break 'b_193208;
                                }
                            }
                            c.cycles += 9;
                            c.g[A0][0] = 0x460000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x308));
                            call_ptr(c, 0x00317670, 0x00193214)?;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00193220)?;
                            c.g[A2][0] = c.g[V0][0];
                            let t = c.g[A2][0] == 0u64;
                            c.g[A1][0] = 0u64;
                            if t {
                                break 'b_19325c;
                            } else {
                                c.cycles += 7;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00193240)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                                if t {
                                    break 'b_193260;
                                } else {
                                    break 'b_193248;
                                }
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0019325c)?;
                    break 'b_19325c;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
            break 'b_193260;
        }
        'b_1932e0: {
            c.cycles += 3;
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            let t = c.g[V0][0] != 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            if t {
                break 'b_1932e0;
            } else {
                'b_1932a4: {
                    c.cycles += 9;
                    c.g[A0][0] = 0x4a0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1588));
                    call_ptr(c, 0x00317670, 0x00193278)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00193284)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_1932a4;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001932a4)?;
                        break 'b_1932a4;
                    }
                }
                'b_1932dc: {
                    c.cycles += 9;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2f8));
                    call_ptr(c, 0x00317670, 0x001932b0)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001932bc)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_1932dc;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001932dc)?;
                        break 'b_1932dc;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                break 'b_1932e0;
            }
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        return Ok(());
    });
}

/// Native resume of `cfe_state_bragging_rights_init` (converted from the `entries` body): any entry
/// other than 0x00192d80, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn cfe_state_bragging_rights_init_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00192d80 => {
                    c.cycles += 23;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[A0][0] = 0x460000;
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x338));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[RA][0] = sx(0x00192dc0);
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x00317670, 0x00192dc0)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00192dd4);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00397cc0, 0x00192dd4)?;
                    let t = c.g[V0][0] == 0u64;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                    if t {
                        pc = 0x00192dec;
                        continue;
                    }
                    pc = 0x00192ddc;
                    continue;
                }
                0x00192ddc => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x00192dec);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0039c870, 0x00192dec)?;
                    pc = 0x00192dec;
                    continue;
                }
                0x00192dec => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                    c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6d0));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x704));
                    if t {
                        pc = 0x00192e1c;
                        continue;
                    }
                    pc = 0x00192e04;
                    continue;
                }
                0x00192e04 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x00192e10);
                    c.g[A1][0] = 0u64;
                    Callee {
                        addr: 0x00192740,
                        f: ee::s02::cFEStateBraggingRights_SetRowsVisible,
                    }
                    .call(c, 0x00192e10)?;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x00192e1c);
                    c.g[A1][0] = 0u64;
                    Callee {
                        addr: 0x00192740,
                        f: ee::s02::cFEStateBraggingRights_SetRowsVisible,
                    }
                    .call(c, 0x00192e1c)?;
                    pc = 0x00192e1c;
                    continue;
                }
                0x00192e1c => {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x00192e24);
                    c.g[S5][0] = 0u64;
                    call_ptr(c, 0x0014dc80, 0x00192e24)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00192e30);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x00192e30)?;
                    c.g[V0][0] = 0x530000;
                    c.g[A0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c12));
                    c.g[V1][0] = (c.g[A0][0] < 0x6u64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[S0][0] = 0u64;
                    if t {
                        pc = 0x00192ea8;
                        continue;
                    }
                    pc = 0x00192e44;
                    continue;
                }
                0x00192e44 => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x460000;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2e0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00192e60 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x19e0));
                    let t = true;
                    c.g[S5][0] = 0x6;
                    if t {
                        pc = 0x00192ea4;
                        continue;
                    }
                    pc = 0x00192e6c;
                    continue;
                }
                0x00192e6c => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x19d8));
                    let t = true;
                    c.g[S5][0] = 0x7;
                    if t {
                        pc = 0x00192ea4;
                        continue;
                    }
                    pc = 0x00192e78;
                    continue;
                }
                0x00192e78 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x19d0));
                    let t = true;
                    c.g[S5][0] = 0x7;
                    if t {
                        pc = 0x00192ea4;
                        continue;
                    }
                    pc = 0x00192e84;
                    continue;
                }
                0x00192e84 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x19c8));
                    let t = true;
                    c.g[S5][0] = 0x7;
                    if t {
                        pc = 0x00192ea4;
                        continue;
                    }
                    pc = 0x00192e90;
                    continue;
                }
                0x00192e90 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x19c0));
                    let t = true;
                    c.g[S5][0] = 0x5;
                    if t {
                        pc = 0x00192ea4;
                        continue;
                    }
                    pc = 0x00192e9c;
                    continue;
                }
                0x00192e9c => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x19b8));
                    c.g[S5][0] = 0x7;
                    pc = 0x00192ea4;
                    continue;
                }
                0x00192ea4 => {
                    c.cycles += 1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x738), c.g[V0][0]);
                    pc = 0x00192ea8;
                    continue;
                }
                0x00192ea8 => {
                    c.cycles += 2;
                    let t = c.g[S5][0] == 0u64;
                    c.g[A0][0] = 0u64;
                    if t {
                        pc = 0x00192ed4;
                        continue;
                    }
                    pc = 0x00192eb0;
                    continue;
                }
                0x00192eb0 => {
                    c.cycles += 2;
                    c.g[A1][0] = 0x1;
                    pc = 0x00192eb8;
                    continue;
                }
                0x00192eb8 => {
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[A1][0] as u32) << ((c.g[A0][0] as u32) & 31));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = c.g[S0][0] | c.g[V1][0];
                    if t {
                        if back_edge_unwinds(c, 0x00192eb8) {
                            return Ok(());
                        }
                        pc = 0x00192eb8;
                        continue;
                    }
                    pc = 0x00192ed4;
                    continue;
                }
                0x00192ed4 => {
                    c.cycles += 21;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x00192ee0);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00192968,
                        f: ee::s02::cFEStateBraggingRights_SetRowMask,
                    }
                    .call(c, 0x00192ee0)?;
                    c.g[S2][0] = 0u64;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00192ef0);
                    c.g[A0][0] = c.g[S4][0];
                    Callee {
                        addr: 0x00192968,
                        f: ee::s02::cFEStateBraggingRights_SetRowMask,
                    }
                    .call(c, 0x00192ef0)?;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00192efc);
                    c.g[A0][0] = c.g[S3][0];
                    Callee {
                        addr: 0x00192970,
                        f: ee::s02::cFEStateBraggingRights_HideUnusedRows,
                    }
                    .call(c, 0x00192efc)?;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00192f08);
                    c.g[A0][0] = c.g[S4][0];
                    Callee {
                        addr: 0x00192970,
                        f: ee::s02::cFEStateBraggingRights_HideUnusedRows,
                    }
                    .call(c, 0x00192f08)?;
                    c.g[V0][0] = 0x4a0000;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14f8));
                    c.g[V1][0] = c.lb(c.g[A2][0] as u32);
                    c.g[A0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.sb(c.g[SP][0] as u32, c.g[V1][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x1), c.g[A0][0]);
                    let t = c.g[S5][0] == 0u64;
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        pc = 0x00193190;
                        continue;
                    }
                    pc = 0x00192f28;
                    continue;
                }
                0x00192f28 => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x31));
                    pc = 0x00192f30;
                    continue;
                }
                0x00192f30 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00192f3c);
                    c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                    call_ptr(c, 0x00317670, 0x00192f3c)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00192f48);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00192f48)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[S3][0] = 0u64;
                    if t {
                        pc = 0x00192f88;
                        continue;
                    }
                    pc = 0x00192f54;
                    continue;
                }
                0x00192f54 => {
                    c.cycles += 13;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x738));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                    c.g[V0][0] = 0x440000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1108));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[RA][0] = sx(0x00192f78);
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    call_ptr(c, 0x00317670, 0x00192f78)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00192f84);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x00192f84)?;
                    c.g[S3][0] = 0u64;
                    pc = 0x00192f88;
                    continue;
                }
                0x00192f88 => {
                    c.cycles += 6;
                    c.g[S6][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[S7][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[S4][0] = 0x460000;
                    c.g[FP][0] = 0x4a0000;
                    c.g[V0][0] = 0x34;
                    pc = 0x00192fa0;
                    continue;
                }
                0x00192fa0 => {
                    c.cycles += 10;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x738));
                    c.mult(0, 4, 19, 2);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.g[A1][0] = c.lb(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[V1][0] = ((c.g[A1][0] as i64) < 7i64) as u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                    let t = c.g[V1][0] == 0u64;
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6d8));
                    if t {
                        pc = 0x00192fe0;
                        continue;
                    }
                    pc = 0x00192fc8;
                    continue;
                }
                0x00192fc8 => {
                    c.cycles += 2;

                    if (c.g[A1][0] as i64) > 0 {
                        c.g[A0][0] = 0x460000;
                        pc = 0x00192ff4;
                        continue;
                    }
                    pc = 0x00192fd0;
                    continue;
                }
                0x00192fd0 => {
                    c.cycles += 2;
                    let t = c.g[A1][0] == 0u64;

                    if t {
                        pc = 0x00193030;
                        continue;
                    }
                    pc = 0x00192fd8;
                    continue;
                }
                0x00192fd8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00193170;
                        continue;
                    }
                    pc = 0x00192fe0;
                    continue;
                }
                0x00192fe0 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x7;
                    let t = c.g[A1][0] == c.g[V0][0];

                    if t {
                        pc = 0x001930d4;
                        continue;
                    }
                    pc = 0x00192fec;
                    continue;
                }
                0x00192fec => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00193170;
                        continue;
                    }
                    pc = 0x00192ff4;
                    continue;
                }
                0x00192ff4 => {
                    c.cycles += 15;
                    c.g[RA][0] = sx(0x00192ffc);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                    call_ptr(c, 0x00317670, 0x00192ffc)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x0019300c);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a568, 0x0019300c)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x00193018);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                    call_ptr(c, 0x00317670, 0x00193018)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00193028);
                    c.g[A2][0] = 0x1;
                    call_ptr(c, 0x0039a568, 0x00193028)?;
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x738));
                    if t {
                        pc = 0x0019309c;
                        continue;
                    }
                    pc = 0x00193030;
                    continue;
                }
                0x00193030 => {
                    c.cycles += 27;
                    c.g[RA][0] = sx(0x00193038);
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x20c0));
                    call_ptr(c, 0x00317670, 0x00193038)?;
                    c.g[A2][0] = 0u64;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00193048);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a568, 0x00193048)?;
                    c.g[A2][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x16e0));
                    c.g[RA][0] = sx(0x0019305c);
                    c.g[A2][0] = 0x5;
                    call_ptr(c, 0x0039a3f8, 0x0019305c)?;
                    c.g[V0][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x16d8));
                    c.g[RA][0] = sx(0x00193070);
                    c.g[A2][0] = 0xa;
                    call_ptr(c, 0x0039a3f8, 0x00193070)?;
                    c.g[V1][0] = 0x4a0000;
                    c.g[A2][0] = 0xf;
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1758));
                    c.g[RA][0] = sx(0x00193084);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a3f8, 0x00193084)?;
                    c.g[A2][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1760));
                    c.g[RA][0] = sx(0x00193098);
                    c.g[A2][0] = 0x14;
                    call_ptr(c, 0x0039a3f8, 0x00193098)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x738));
                    pc = 0x0019309c;
                    continue;
                }
                0x0019309c => {
                    c.cycles += 14;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S3][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.g[RA][0] = sx(0x001930b0);
                    c.g[A2][0] = c.lb(c.g[V1][0] as u32);
                    Callee {
                        addr: 0x00193f38,
                        f: ee::s02::cFEStateBraggingRights_GetRowDefault,
                    }
                    .call(c, 0x001930b0)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001930c0);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x0039a768, 0x001930c0)?;
                    c.g[A1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x001930cc);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a7a8, 0x001930cc)?;
                    let t = true;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00193170;
                        continue;
                    }
                    pc = 0x001930d4;
                    continue;
                }
                0x001930d4 => {
                    c.cycles += 39;
                    c.g[RA][0] = sx(0x001930dc);
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x20c0));
                    call_ptr(c, 0x00317670, 0x001930dc)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001930ec);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0039a568, 0x001930ec)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1768));
                    c.g[RA][0] = sx(0x001930fc);
                    c.g[A2][0] = 0xbb8;
                    call_ptr(c, 0x0039a3f8, 0x001930fc)?;
                    c.g[V0][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1770));
                    c.g[RA][0] = sx(0x00193110);
                    c.g[A2][0] = 0x1388;
                    call_ptr(c, 0x0039a3f8, 0x00193110)?;
                    c.g[V1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1778));
                    c.g[RA][0] = sx(0x00193124);
                    c.g[A2][0] = 0x2710;
                    call_ptr(c, 0x0039a3f8, 0x00193124)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1780));
                    c.g[RA][0] = sx(0x00193138);
                    c.g[A2][0] = 0x3a98;
                    call_ptr(c, 0x0039a3f8, 0x00193138)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x738));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S3][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.g[RA][0] = sx(0x00193150);
                    c.g[A2][0] = c.lb(c.g[V1][0] as u32);
                    Callee {
                        addr: 0x00193f38,
                        f: ee::s02::cFEStateBraggingRights_GetRowDefault,
                    }
                    .call(c, 0x00193150)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00193160);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x0039a768, 0x00193160)?;
                    c.g[A1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x0019316c);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a7a8, 0x0019316c)?;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    pc = 0x00193170;
                    continue;
                }
                0x00193170 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < 2i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x34;
                    if t {
                        if back_edge_unwinds(c, 0x00192fa0) {
                            return Ok(());
                        }
                        pc = 0x00192fa0;
                        continue;
                    }
                    pc = 0x0019317c;
                    continue;
                }
                0x0019317c => {
                    c.cycles += 4;
                    c.g[S2][0] = c.g[S7][0];
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S5][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x31));
                    if t {
                        if back_edge_unwinds(c, 0x00192f30) {
                            return Ok(());
                        }
                        pc = 0x00192f30;
                        continue;
                    }
                    pc = 0x0019318c;
                    continue;
                }
                0x0019318c => {
                    c.cycles += 1;
                    c.g[A0][0] = 0x4a0000;
                    pc = 0x00193190;
                    continue;
                }
                0x00193190 => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x00193198);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1580));
                    call_ptr(c, 0x00317670, 0x00193198)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001931a4);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001931a4)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                    if t {
                        pc = 0x0019325c;
                        continue;
                    }
                    pc = 0x001931b0;
                    continue;
                }
                0x001931b0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x00193248;
                        continue;
                    }
                    pc = 0x001931bc;
                    continue;
                }
                0x001931bc => {
                    c.cycles += 14;
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001931d0);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001931d0)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x001931dc);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x318));
                    call_ptr(c, 0x00317670, 0x001931dc)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001931e8);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001931e8)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00193208;
                        continue;
                    }
                    pc = 0x001931f4;
                    continue;
                }
                0x001931f4 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00193208);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00193208)?;
                    pc = 0x00193208;
                    continue;
                }
                0x00193208 => {
                    c.cycles += 9;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x00193214);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x308));
                    call_ptr(c, 0x00317670, 0x00193214)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00193220);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00193220)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x0019325c;
                        continue;
                    }
                    pc = 0x0019322c;
                    continue;
                }
                0x0019322c => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00193240);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00193240)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                    if t {
                        pc = 0x00193260;
                        continue;
                    }
                    pc = 0x00193248;
                    continue;
                }
                0x00193248 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0019325c);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0019325c)?;
                    pc = 0x0019325c;
                    continue;
                }
                0x0019325c => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                    pc = 0x00193260;
                    continue;
                }
                0x00193260 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    let t = c.g[V0][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    if t {
                        pc = 0x001932e0;
                        continue;
                    }
                    pc = 0x0019326c;
                    continue;
                }
                0x0019326c => {
                    c.cycles += 9;
                    c.g[A0][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x00193278);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1588));
                    call_ptr(c, 0x00317670, 0x00193278)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00193284);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00193284)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x001932a4;
                        continue;
                    }
                    pc = 0x00193290;
                    continue;
                }
                0x00193290 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001932a4);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001932a4)?;
                    pc = 0x001932a4;
                    continue;
                }
                0x001932a4 => {
                    c.cycles += 9;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x001932b0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2f8));
                    call_ptr(c, 0x00317670, 0x001932b0)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001932bc);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001932bc)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x001932dc;
                        continue;
                    }
                    pc = 0x001932c8;
                    continue;
                }
                0x001932c8 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001932dc);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001932dc)?;
                    pc = 0x001932dc;
                    continue;
                }
                0x001932dc => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    pc = 0x001932e0;
                    continue;
                }
                0x001932e0 => {
                    c.cycles += 11;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00192d80, 0x0019330c);
                    return Ok(());
                }
            }
        }
    });
}

/// `StrHash` (0x317670), `UIScreen_FindObject` (0x39d860) and the other callees of the
/// bragging-rights state, by address.
pub(crate) const OC849_STR_HASH: u32 = 0x0031_7670;

/// `address = reg + off` (32-bit wrap), as the generated code forms it.
#[inline(always)]
pub(crate) fn oc849_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `void cFEStateBraggingRights_Render(self)` (0x193318): tail wrapper around
/// `UIState_Render`.
pub fn c_fe_state_bragging_rights_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0039_e6b8, 0x0019_3328)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateBraggingRights_SetRowsVisible` (0x192740).
pub(crate) const OC849_SET_ROWS_VISIBLE: Callee = Callee {
    addr: 0x0019_2740,
    f: ee::s02::f_00192740,
};

/// `cFEStateBraggingRights_HideUnusedRows` (0x192970), through the generated
/// function (which dispatches through the override slot once installed).
pub(crate) const OC849_HIDE_UNUSED_ROWS: Callee = Callee {
    addr: 0x0019_2970,
    f: ee::s02::f_00192970,
};

/// `void cFEStateBraggingRights_Update(self)` (0x193338): for each local player
/// panel (one online, two offline) read the controller id, feed it to the UI input
/// object, and run the confirm / cancel logic on the player's menu control
/// (`self + 0x6d0 + 0x34 * panel`): confirm calls the menu's vtable slot 0x78 with
/// the input; cancel redraws the row visibility, rewinds the screen frame by label
/// and, online, calls `func_00265700` on the setup manager.
pub fn c_fe_state_bragging_rights_update(c: &mut Cpu) {
    guest(c, |c| {
        let is_net = |c: &mut Cpu| c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
        c.cycles += 25;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = is_net(c);
        frame.save(c, S2, 0x70);
        c.g[V1][0] = 0x1;
        frame.save(c, S5, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S6, 0x30);
        c.g[V0][0] ^= 0x1;
        frame.save(c, S7, 0x20);
        c.g[S5][0] = 0;
        frame.save(c, S0, 0x90);
        c.g[S7][0] = 0x2;
        frame.save(c, S1, 0x80);
        if c.g[V0][0] == 0 {
            c.g[S7][0] = c.g[V1][0];
        }
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.lw(oc849_at(c, S2, 0x10));
        c.g[S3][0] = c.lw(oc849_at(c, A0, 0xc));
        call_ptr(c, 0x0014_dc80, 0x0019_3388)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014_dd58, 0x0019_3394)?;
        c.g[S6][0] = c.g[V0][0];
        if c.g[S7][0] != 0 {
            c.cycles += 1;
            c.g[V0][0] = is_net(c);
            loop {
                // s5 = loop counter, s4 = panel index (0 online).
                c.cycles += 3;
                c.g[V0][0] ^= 0x1;
                let offline = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S5][0];
                if offline {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x0014_7318, 0x0019_33c8)?;
                    c.g[S4][0] = c.g[S5][0];
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x0014_7318, 0x0019_33b8)?;
                    c.g[S4][0] = 0;
                }
                'panel: {
                    'confirm_checked: {
                        // input->setController(id)
                        c.cycles += 15;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                        c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                        c.g[V0][0] = c.lw(oc849_at(c, S3, 0x8));
                        c.g[A1][0] &= 0xff;
                        c.g[A0][0] = c.lh(oc849_at(c, V0, 0x160));
                        c.g[A2][0] = c.lw(oc849_at(c, V0, 0x164));
                        let method = c.g[A2][0] as u32;
                        c.g[A0][0] = sx(oc849_at(c, S3, 0).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, method, 0x0019_33ec)?;
                        // input->confirmPressed()
                        c.g[V1][0] = c.lw(oc849_at(c, S3, 0x8));
                        c.g[A0][0] = c.lh(oc849_at(c, V1, 0x140));
                        c.g[V0][0] = c.lw(oc849_at(c, V1, 0x144));
                        let method = c.g[V0][0] as u32;
                        c.g[A0][0] = sx(oc849_at(c, S3, 0).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, method, 0x0019_3400)?;
                        if c.g[V0][0] == 0 {
                            c.g[V1][0] = c.lw(oc849_at(c, S3, 0x8));
                            break 'confirm_checked;
                        }
                        c.cycles += 6;
                        c.g[A0][0] = 0x34;
                        c.mult(0, 3, 20, 4);
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.g[V1][0] = c.lw(oc849_at(c, V0, 0x6f8));
                        if c.g[V1][0] != 0 {
                            c.g[V1][0] = c.lw(oc849_at(c, S3, 0x8));
                            break 'confirm_checked;
                        }
                        // menu->vtable[0xf](input): the panel's menu takes the input.
                        c.cycles += 9;
                        c.g[A2][0] = c.lw(oc849_at(c, V0, 0x6d0));
                        c.g[A1][0] = c.g[S3][0];
                        c.g[V1][0] = c.lw(oc849_at(c, A2, 0x8));
                        c.g[A0][0] = c.lh(oc849_at(c, V1, 0x78));
                        c.g[V0][0] = c.lw(oc849_at(c, V1, 0x7c));
                        let method = c.g[V0][0] as u32;
                        c.g[A0][0] = sx(oc849_at(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, method, 0x0019_343c)?;
                        c.g[S5][0] = sx(oc849_at(c, S5, 0x1));
                        break 'panel;
                    }
                    // input->cancelPressed()
                    c.cycles += 6;
                    c.g[A0][0] = c.lh(oc849_at(c, V1, 0x30));
                    c.g[V0][0] = c.lw(oc849_at(c, V1, 0x34));
                    let method = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(oc849_at(c, S3, 0).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0019_3454)?;
                    if c.g[V0][0] == 0 {
                        c.g[S5][0] = sx(oc849_at(c, S5, 0x1));
                        break 'panel;
                    }
                    'cancel_done: {
                        c.cycles += 6;
                        c.g[S0][0] = 0x34;
                        c.mult(0, 3, 20, 16);
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.g[V1][0] = c.lw(oc849_at(c, V0, 0x6f8));
                        let skip = c.g[V1][0] == 0;
                        c.g[V0][0] = 0x4a_0000;
                        if skip {
                            break 'cancel_done;
                        }
                        // Reset the row visibility and the label prefix on the stack.
                        c.cycles += 22;
                        c.g[A0][0] = sx(frame.at(c, 0x3));
                        c.g[A1][0] = 0;
                        c.g[V1][0] = sx(oc849_at(c, V0, 0x1788));
                        c.g[A3][0] = c.lb(oc849_at(c, V1, 0));
                        c.g[T0][0] = c.lb(oc849_at(c, V1, 0x1));
                        c.g[T1][0] = c.lb(oc849_at(c, V1, 0x2));
                        let p = frame.at(c, 0);
                        c.sb(p, c.g[A3][0]);
                        let p = frame.at(c, 0x1);
                        c.sb(p, c.g[T0][0]);
                        let p = frame.at(c, 0x2);
                        c.sb(p, c.g[T1][0]);
                        c.g[A2][0] = 0x2;
                        call_ptr(c, 0x0041_6210, 0x0019_34a0)?;
                        c.mult(0, 17, 20, 16);
                        c.g[A1][0] = 0;
                        c.g[S0][0] = sx(oc849_at(c, S1, 0x6d0));
                        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        c.g[A0][0] = c.g[S0][0];
                        OC849_SET_ROWS_VISIBLE.call(c, 0x0019_34b8)?;
                        c.g[A1][0] = c.lw(oc849_at(c, S2, 0x40));
                        c.g[A0][0] = c.g[S0][0];
                        OC849_HIDE_UNUSED_ROWS.call(c, 0x0019_34c4)?;
                        let first_panel = c.g[S4][0] == 0;
                        c.g[V0][0] = 0x32;
                        if !first_panel {
                            c.cycles += 1;
                            let p = frame.at(c, 0);
                            c.sb(p, c.g[V0][0]);
                        }
                        // screen->gotoFrame(label)
                        c.cycles += 13;
                        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.g[A1][0] = 0;
                        c.g[A0][0] = c.lw(oc849_at(c, V0, 0x6d0));
                        call_ptr(c, 0x0039_b960, 0x0019_34e0)?;
                        c.g[A0][0] = c.g[SP][0];
                        call_ptr(c, 0x0031_7670, 0x0019_34e8)?;
                        c.g[A0][0] = c.lw(oc849_at(c, S2, 0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039_c778, 0x0019_34f4)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        let no_frame = c.g[A1][0] == c.g[V0][0];
                        c.g[A1][0] &= 0xffff;
                        if !no_frame {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw(oc849_at(c, S2, 0x40));
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x0039_c870, 0x0019_3510)?;
                        }
                        c.cycles += 4;
                        c.g[V0][0] = is_net(c);
                        c.g[V0][0] ^= 0x1;
                        if c.g[V0][0] != 0 {
                            c.g[S5][0] = sx(oc849_at(c, S5, 0x1));
                            break 'panel;
                        }
                        c.cycles += 2;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2dc));
                        call_ptr(c, 0x0026_5700, 0x0019_3528)?;
                    }
                    c.cycles += 1;
                    c.g[S5][0] = sx(oc849_at(c, S5, 0x1));
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[S7][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = is_net(c);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0019_33a0);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `widget->vtable[slot](a1)` as the compiler emitted it for the `+0x38/0x3c`
/// slot (`setFocus(1)`): `a0 = widget + lh(vt + 0x38)`, target `lw(vt + 0x3c)`.
pub(crate) fn oc849_widget_focus(c: &mut Cpu, ret: u32) -> Flow {
    c.g[A1][0] = 0x1;
    c.g[V0][0] = c.lw(oc849_at(c, S2, 0x8));
    c.g[A0][0] = c.lh(oc849_at(c, V0, 0x38));
    c.g[V1][0] = c.lw(oc849_at(c, V0, 0x3c));
    let method = c.g[V1][0] as u32;
    c.g[A0][0] = sx(oc849_at(c, S2, 0).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, ret)
}

/// The two back-end interface lookups (`cBE_getInterface(1)` and `(2)`) of the
/// player-name widgets; leaves `s1` = interface 1, `s0` = interface 2.
pub(crate) fn oc849_player_name_text(
    c: &mut Cpu,
    rets: [u32; 4],
    player: u64,
    charge: u64,
) -> Flow {
    c.cycles += charge;
    call_ptr(c, 0x0014_dc80, rets[0])?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0x1;
    call_ptr(c, 0x0014_dd58, rets[1])?;
    c.g[S1][0] = c.g[V0][0];
    call_ptr(c, 0x0014_dc80, rets[2])?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x0014_dd58, rets[3])?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = player;
    Ok(())
}

/// `void cFEStateBraggingRights_OnWidgetCreated(self, widget)` (0x193650): file the
/// widget the screen created by its name hash: "1Menu" / "2Menu" become the
/// player's menu control (`self + 0x6d0` / `+ 0x704`, focused, flagged), the
/// player name texts get the rider's short name, the two info groups go to
/// `self + 0x6d4` / `+ 0x708`, and the row widgets "11".."28" (player, row) fill
/// the item slots (`self + 0x6d8 + 0x34 * player + 4 * row`), flagged static.
pub fn c_fe_state_bragging_rights_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S6, 0x40);
        frame.save(c, S2, 0x80);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, S1, 0x90);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S3, 0x70);
        c.g[A0][0] = 0x4a_0000;
        frame.save(c, S4, 0x60);
        c.g[A0][0] = sx(oc849_at(c, A0, 0x17a0));
        frame.save(c, S5, 0x50);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        frame.save(c, S0, 0xa0);
        c.g[S0][0] = c.lw(oc849_at(c, S2, 0x38));
        call_ptr(c, OC849_STR_HASH, 0x0019_3694)?;
        // true: left through the `+1` exit (the lq of s0 comes after it).
        let counted_exit: bool = 'main: {
            // "1Menu" / "2Menu": the player's menu control.
            let menu_found = 'menu: {
                if c.g[S0][0] == c.g[V0][0] {
                    c.cycles += 9;
                    c.sw(oc849_at(c, S6, 0x6d0), 0);
                    oc849_widget_focus(c, 0x0019_36b8)?;
                    c.sw(oc849_at(c, S6, 0x6d0), c.g[S2][0]);
                    break 'menu true;
                }
                c.g[A0][0] = 0x4a_0000;
                c.cycles += 5;
                c.g[S0][0] = c.lw(oc849_at(c, S2, 0x38));
                c.g[A0][0] = sx(oc849_at(c, A0, 0x17a8));
                call_ptr(c, OC849_STR_HASH, 0x0019_36cc)?;
                if c.g[S0][0] == c.g[V0][0] {
                    c.cycles += 8;
                    c.sw(oc849_at(c, S6, 0x704), 0);
                    oc849_widget_focus(c, 0x0019_36f0)?;
                    c.sw(oc849_at(c, S6, 0x704), c.g[S2][0]);
                    break 'menu true;
                }
                c.g[A0][0] = 0x46_0000;
                false
            };
            if menu_found {
                c.cycles += 7;
                c.g[V0][0] = c.lw(oc849_at(c, S2, 0x14));
                c.g[V1][0] = c.lw(oc849_at(c, S2, 0x90));
                c.g[V0][0] |= 0x1;
                c.g[V1][0] |= 0x1;
                c.sw(oc849_at(c, S2, 0x14), c.g[V0][0]);
                c.sw(oc849_at(c, S2, 0x90), c.g[V1][0]);
                break 'main true;
            }
            // "Player1_title" / "Player1_name": a1 = 0; the Player2 pair: a1 = 1.
            c.cycles += 5;
            c.g[S0][0] = c.lw(oc849_at(c, S2, 0x38));
            c.g[A0][0] = sx(oc849_at(c, A0, 0).wrapping_sub(0x2c8));
            call_ptr(c, OC849_STR_HASH, 0x0019_371c)?;
            let mut player1 = c.g[S0][0] == c.g[V0][0];
            c.g[A0][0] = 0x46_0000;
            if !player1 {
                c.cycles += 5;
                c.g[S0][0] = c.lw(oc849_at(c, S2, 0x38));
                c.g[A0][0] = sx(oc849_at(c, A0, 0).wrapping_sub(0x2b8));
                call_ptr(c, OC849_STR_HASH, 0x0019_3730)?;
                player1 = c.g[S0][0] == c.g[V0][0];
                c.g[A0][0] = 0x46_0000;
            }
            let mut player_text = false;
            if player1 {
                oc849_player_name_text(
                    c,
                    [0x0019_3740, 0x0019_374c, 0x0019_3754, 0x0019_3760],
                    0,
                    14,
                )?;
                player_text = true;
            } else {
                c.cycles += 5;
                c.g[S0][0] = c.lw(oc849_at(c, S2, 0x38));
                c.g[A0][0] = sx(oc849_at(c, A0, 0).wrapping_sub(0x2a8));
                call_ptr(c, OC849_STR_HASH, 0x0019_377c)?;
                let mut player2 = c.g[S0][0] == c.g[V0][0];
                c.g[A0][0] = 0x46_0000;
                if !player2 {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw(oc849_at(c, S2, 0x38));
                    c.g[A0][0] = sx(oc849_at(c, A0, 0).wrapping_sub(0x298));
                    call_ptr(c, OC849_STR_HASH, 0x0019_3790)?;
                    player2 = c.g[S0][0] == c.g[V0][0];
                    c.g[A0][0] = 0x46_0000;
                }
                if player2 {
                    oc849_player_name_text(
                        c,
                        [0x0019_37a0, 0x0019_37ac, 0x0019_37b4, 0x0019_37c0],
                        1,
                        13,
                    )?;
                    player_text = true;
                }
            }
            if player_text {
                // cUIText_setAsciiString(widget, cBECharacterInterface_getShortName(...))
                c.cycles += 10;
                call_ptr(c, 0x0014_6e98, 0x0019_37d4)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0014_ee58, 0x0019_37e0)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_0c50, 0x0019_37ec)?;
                frame.restore(c, S0, 0xa0);
                break 'main false;
            }
            // "player1_info" / "player2_info": the info groups.
            c.cycles += 5;
            c.g[S0][0] = c.lw(oc849_at(c, S2, 0x38));
            c.g[A0][0] = sx(oc849_at(c, A0, 0).wrapping_sub(0x288));
            call_ptr(c, OC849_STR_HASH, 0x0019_3800)?;
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 2;
                c.sw(oc849_at(c, S6, 0x6d4), c.g[S2][0]);
                break 'main true;
            }
            c.g[A0][0] = 0x46_0000;
            c.cycles += 5;
            c.g[S0][0] = c.lw(oc849_at(c, S2, 0x38));
            c.g[A0][0] = sx(oc849_at(c, A0, 0).wrapping_sub(0x278));
            call_ptr(c, OC849_STR_HASH, 0x0019_381c)?;
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 2;
                c.sw(oc849_at(c, S6, 0x708), c.g[S2][0]);
                break 'main true;
            }
            let p = frame.at(c, 0x2);
            c.sb(p, 0);
            // The row widgets: names "<player><row>" for 2 players x 8 rows.
            c.cycles += 5;
            c.g[S3][0] = 0;
            c.g[FP][0] = 0x34;
            c.g[S7][0] = 0xffff_ffff_ffff_fffe;
            c.mult(0, 3, 19, 30);
            loop {
                c.cycles += 6;
                c.g[S1][0] = 0;
                c.g[S4][0] = sx(oc849_at(c, S3, 0x31));
                c.g[S5][0] = sx(oc849_at(c, S3, 0x1));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S6][0] as u32));
                c.g[S0][0] = sx(oc849_at(c, V0, 0x6d8));
                loop {
                    c.cycles += 8;
                    c.g[V0][0] = sx(oc849_at(c, S1, 0x31));
                    let p = frame.at(c, 0);
                    c.sb(p, c.g[S4][0]);
                    let p = frame.at(c, 0x1);
                    c.sb(p, c.g[V0][0]);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, OC849_STR_HASH, 0x0019_386c)?;
                    c.g[V1][0] = c.lw(oc849_at(c, S2, 0x38));
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[S1][0] = sx(oc849_at(c, S1, 0x1));
                    } else {
                        c.cycles += 12;
                        c.sw(c.g[S0][0] as u32, c.g[S2][0]);
                        c.g[A1][0] = 0x1;
                        c.sw(oc849_at(c, S2, 0x18), c.g[S3][0]);
                        c.g[V0][0] = c.lw(oc849_at(c, S2, 0x14));
                        c.g[V1][0] = c.lw(oc849_at(c, S2, 0x8));
                        c.g[V0][0] &= c.g[S7][0];
                        c.sw(oc849_at(c, S2, 0x14), c.g[V0][0]);
                        c.g[A0][0] = c.lh(oc849_at(c, V1, 0xc0));
                        c.g[V0][0] = c.lw(oc849_at(c, V1, 0xc4));
                        let method = c.g[V0][0] as u32;
                        c.g[A0][0] = sx(oc849_at(c, S2, 0).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, method, 0x0019_38a4)?;
                        c.g[S1][0] = sx(oc849_at(c, S1, 0x1));
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 8i64) as u64;
                    let more_rows = c.g[V0][0] != 0;
                    c.g[S0][0] = sx(oc849_at(c, S0, 0x4));
                    if more_rows {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019_3858);
                        }
                        continue;
                    }
                    break;
                }
                c.cycles += 4;
                c.g[S3][0] = c.g[S5][0];
                c.g[V0][0] = ((c.g[S3][0] as i64) < 2i64) as u64;
                let more_players = c.g[V0][0] != 0;
                c.mult(0, 3, 19, 30);
                if more_players {
                    if event_due(c) {
                        deliver_event_here(c, 0x0019_3840);
                    }
                    continue;
                }
                break 'main true;
            }
        };
        if counted_exit {
            c.cycles += 1;
            frame.restore(c, S0, 0xa0);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateBraggingRights_OnWidgetAction(self, widget, event)` (0x1938f8): the
/// bragging-rights screen's widget events (1..9, jump table at 0x45fdc0). 1/2
/// recolour the widget's menu rows, 5 (a row confirmed) marks the menu confirmed
/// and, once both are, asks the engine object for the next state and switches to
/// it, 6 (cancel) switches state (or opens the "cancel challenge" popup online),
/// 7 forwards to `NetFEState_OnWidgetAction`, 9 stores the selected value in the
/// BE rules; the other events do nothing. `gp-0x1680` is the online flag.
/// Resumes at any point other than the entry through the generated body.
pub fn c_fe_state_bragging_rights_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001938f8 {
        return c_fe_state_bragging_rights_on_widget_action_resume(c);
    }
    guest(c, |c| 'l_1938f8: loop {
        'b_193be0: {
            'b_193bdc: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = (c.g[V1][0] < 0x9u64) as u64;
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S3][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S4][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[V0][0] == 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_193bdc;
                } else {
                    'b_193b4c: {
                        'b_193b28: {
                            'b_193b08: {
                                'b_193a1c: {
                                    c.cycles += 7;
                                    c.g[V0][0] = 0x460000;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x240));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x001938f8 => {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x001938f8);
                                            }
                                            continue 'l_1938f8;
                                        }
                                        0x00193948 => {
                                            c.cycles += 4;
                                            c.g[S0][0] =
                                                sx((c.g[S3][0] as u32).wrapping_add(0x6d0));
                                            c.g[S1][0] = 0u64;
                                            c.g[S5][0] = 0xffffffffffffffff;
                                            c.g[S2][0] = c.g[S0][0];
                                            'l_193958: loop {
                                                'b_1939bc: {
                                                    'b_1939a4: {
                                                        c.cycles += 5;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x00193964);
                                                        c.g[A1][0] = c.g[S4][0];
                                                        Callee { addr: 0x00192918, f: ee::s02::cFEStateBraggingRights_FindRowIndex }.call(c, 0x00193964)?;
                                                        let t = c.g[V0][0] == c.g[S5][0];
                                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                            .wrapping_sub(0x1680));
                                                        if t {
                                                            break 'b_1939a4;
                                                        } else {
                                                            'b_19398c: {
                                                                c.cycles += 3;
                                                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                                let t = c.g[V0][0] != 0u64;
                                                                c.g[A0][0] = c.g[S2][0];
                                                                if t {
                                                                    break 'b_19398c;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = c.g[S1][0] != 0u64;

                                                                    if t {
                                                                        break 'b_19398c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x00193988);
                                                                        c.g[A0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x2dc));
                                                                        call_ptr(
                                                                            c, 0x00265688,
                                                                            0x00193988,
                                                                        )?;
                                                                        c.g[A0][0] = c.g[S2][0];
                                                                        break 'b_19398c;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 6;
                                                            c.g[RA][0] = sx(0x00193994);
                                                            c.g[A1][0] = 0x1;
                                                            Callee { addr: 0x00192740, f: ee::s02::cFEStateBraggingRights_SetRowsVisible }.call(c, 0x00193994)?;
                                                            c.g[RA][0] = sx(0x0019399c);
                                                            c.g[A0][0] = c.g[S3][0];
                                                            Callee { addr: 0x00194080, f: ee::s02::cFEStateBraggingRights_StartWaitAnim }.call(c, 0x0019399c)?;
                                                            let t = true;
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x1680));
                                                            if t {
                                                                break 'b_1939bc;
                                                            } else {
                                                                break 'b_1939a4;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.g[S1][0] =
                                                        sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                                    c.g[S2][0] =
                                                        sx((c.g[S2][0] as u32).wrapping_add(0x34));
                                                    c.g[V0][0] =
                                                        ((c.g[S1][0] as i64) < 2i64) as u64;
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[S0][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_add(0x34));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x00193958);
                                                        }
                                                        continue 'l_193958;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                            .wrapping_sub(0x1680));
                                                        break 'b_1939bc;
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                if c.g[V0][0] == 0u64 {
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x60),
                                                    );
                                                    break 'b_193be0;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S3][0] as u32).wrapping_add(0x6f8)
                                                        );
                                                    if c.g[V0][0] == 0u64 {
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x60),
                                                        );
                                                        break 'b_193be0;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x72c));
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A1][0] = c.g[S3][0];
                                                        if t {
                                                            break 'b_193bdc;
                                                        } else {
                                                            c.cycles += 10;
                                                            c.g[V0][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[A2][0] = c.lw((c.g[S4][0] as u32)
                                                                .wrapping_add(0x18));
                                                            c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                                                            c.g[V1][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[A0][0] = c.lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0x20));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x24));
                                                            let t = c.g[V0][0] as u32;
                                                            c.g[RA][0] = sx(0x00193a00);
                                                            c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            call_ptr(c, t, 0x00193a00)?;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A1][0] = c.g[V0][0];
                                                            if t {
                                                                break 'b_193bdc;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S3][0] as u32)
                                                                        .wrapping_add(0x10));
                                                                c.g[RA][0] = sx(0x00193a14);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x18));
                                                                call_ptr(
                                                                    c, 0x0039f400, 0x00193a14,
                                                                )?;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x60),
                                                                );
                                                                if t {
                                                                    break 'b_193be0;
                                                                } else {
                                                                    break 'b_193a1c;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        0x00193a1c => {
                                            break 'b_193a1c;
                                        }
                                        0x00193b08 => {
                                            break 'b_193b08;
                                        }
                                        0x00193b28 => {
                                            break 'b_193b28;
                                        }
                                        0x00193b4c => {
                                            break 'b_193b4c;
                                        }
                                        0x00193bdc => {
                                            break 'b_193bdc;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x001938f8, 0x00193c00);
                                            return Ok(());
                                        }
                                    }
                                }
                                'b_193ab8: {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                                        break 'b_193ab8;
                                    } else {
                                        c.cycles += 35;
                                        c.g[RA][0] = sx(0x00193a34);
                                        c.g[A0][0] = c.g[S3][0];
                                        call_ptr(c, 0x001a9c50, 0x00193a34)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A0][0] = 0x460000;
                                        c.g[V0][0] = 0x2;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x268));
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                                        c.g[RA][0] = sx(0x00193a50);
                                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                                        call_ptr(c, 0x00317670, 0x00193a50)?;
                                        c.g[V1][0] = 0x3;
                                        c.g[A0][0] = 0x460000;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                                        c.g[RA][0] = sx(0x00193a68);
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                                        call_ptr(c, 0x00317670, 0x00193a68)?;
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[A1][0] = 0u64;
                                        c.g[RA][0] = sx(0x00193a78);
                                        c.g[A0][0] = c.g[S1][0];
                                        call_ptr(c, 0x001c57c0, 0x00193a78)?;
                                        c.g[A0][0] = 0x460000;
                                        c.g[RA][0] = sx(0x00193a84);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                        call_ptr(c, 0x00317670, 0x00193a84)?;
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x00193a94);
                                        c.g[A1][0] = 0x1;
                                        call_ptr(c, 0x001c57c0, 0x00193a94)?;
                                        c.g[V0][0] = 0x838;
                                        c.g[V1][0] = 0x1;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                                        c.g[A0][0] = c.g[S3][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x168), c.g[V1][0]);
                                        c.g[RA][0] = sx(0x00193ab0);
                                        c.g[A1][0] = c.g[S0][0];
                                        call_ptr(c, 0x001a9bc0, 0x00193ab0)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                        if t {
                                            break 'b_193be0;
                                        } else {
                                            break 'b_193ab8;
                                        }
                                    }
                                }
                                'b_193aec: {
                                    c.cycles += 10;
                                    c.g[A1][0] = c.g[S3][0];
                                    c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x18));
                                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x00193ad8);
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00193ad8)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = c.g[V0][0];
                                    if t {
                                        break 'b_193aec;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                                        c.g[RA][0] = sx(0x00193aec);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                        call_ptr(c, 0x0039f400, 0x00193aec)?;
                                        break 'b_193aec;
                                    }
                                }
                                c.cycles += 7;
                                c.g[A0][0] = 0x530000;
                                c.g[A1][0] = 0u64;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                                c.g[RA][0] = sx(0x00193b00);
                                c.g[A2][0] = 0x20;
                                call_ptr(c, 0x003e6448, 0x00193b00)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                if t {
                                    break 'b_193be0;
                                } else {
                                    break 'b_193b08;
                                }
                            }
                            c.cycles += 8;
                            c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x18));
                            c.g[A0][0] = 0x34;
                            c.mult(0, 4, 16, 4);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6d0));
                            c.g[RA][0] = sx(0x00193b20);
                            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            Callee {
                                addr: 0x001927b0,
                                f: ee::s02::cFEStateBraggingRights_UpdateRowColors,
                            }
                            .call(c, 0x00193b20)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                            if t {
                                break 'b_193be0;
                            } else {
                                break 'b_193b28;
                            }
                        }
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                        c.g[V0][0] = c.g[V0][0] ^ 0x1;
                        let t = c.g[V0][0] != 0u64;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        if t {
                            break 'b_193be0;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S3][0];
                            c.g[RA][0] = sx(0x00193b44);
                            c.g[A1][0] = c.g[S4][0];
                            call_ptr(c, 0x001a8918, 0x00193b44)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                            if t {
                                break 'b_193be0;
                            } else {
                                break 'b_193b4c;
                            }
                        }
                    }
                    'b_193b90: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                        c.g[V0][0] = c.g[V0][0] ^ 0x1;
                        if c.g[V0][0] != 0u64 {
                            c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x18));
                            break 'b_193b90;
                        } else {
                            c.cycles += 13;
                            c.g[A1][0] = c.g[S4][0];
                            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6d0));
                            c.g[RA][0] = sx(0x00193b6c);
                            c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2dc));
                            Callee {
                                addr: 0x00192918,
                                f: ee::s02::cFEStateBraggingRights_FindRowIndex,
                            }
                            .call(c, 0x00193b6c)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00193b78);
                            c.g[A0][0] = c.g[S4][0];
                            call_ptr(c, 0x0039a738, 0x00193b78)?;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x00193b88);
                            c.g[A2][0] = c.g[V0][0];
                            call_ptr(c, 0x00265568, 0x00193b88)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                            if t {
                                break 'b_193be0;
                            } else {
                                break 'b_193b90;
                            }
                        }
                    }
                    c.cycles += 12;
                    c.g[A0][0] = 0x34;
                    c.g[A1][0] = c.g[S4][0];
                    c.mult(0, 4, 16, 4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6d0));
                    c.g[RA][0] = sx(0x00193ba8);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    Callee {
                        addr: 0x00192918,
                        f: ee::s02::cFEStateBraggingRights_FindRowIndex,
                    }
                    .call(c, 0x00193ba8)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x738));
                    c.g[A0][0] = 0xffffffffffffffff;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S1][0] = c.lb(c.g[V1][0] as u32);
                    if c.g[S1][0] == c.g[A0][0] {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        break 'b_193be0;
                    } else {
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x00193bc8);
                        c.g[A0][0] = c.g[S4][0];
                        call_ptr(c, 0x0039a738, 0x00193bc8)?;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x00193bdc);
                        c.g[A3][0] = c.g[V0][0];
                        Callee {
                            addr: 0x00193de0,
                            f: ee::s02::cFEStateBraggingRights_DispatchRowAction,
                        }
                        .call(c, 0x00193bdc)?;
                        break 'b_193bdc;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            break 'b_193be0;
        }
        c.cycles += 8;
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

/// Native resume of `c_fe_state_bragging_rights_on_widget_action` (converted from the `entries` body): any entry
/// other than 0x001938f8, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn c_fe_state_bragging_rights_on_widget_action_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x001938f8 => {
                    c.cycles += 13;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = (c.g[V1][0] < 0x9u64) as u64;
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S4][0] = c.g[A1][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[V0][0] == 0u64;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    if t {
                        pc = 0x00193bdc;
                        continue;
                    }
                    pc = 0x0019392c;
                    continue;
                }
                0x0019392c => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x460000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x240));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00193948 => {
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6d0));
                    c.g[S1][0] = 0u64;
                    c.g[S5][0] = 0xffffffffffffffff;
                    c.g[S2][0] = c.g[S0][0];
                    pc = 0x00193958;
                    continue;
                }
                0x00193958 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00193964);
                    c.g[A1][0] = c.g[S4][0];
                    Callee {
                        addr: 0x00192918,
                        f: ee::s02::cFEStateBraggingRights_FindRowIndex,
                    }
                    .call(c, 0x00193964)?;
                    let t = c.g[V0][0] == c.g[S5][0];
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                    if t {
                        pc = 0x001939a4;
                        continue;
                    }
                    pc = 0x0019396c;
                    continue;
                }
                0x0019396c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[S2][0];
                    if t {
                        pc = 0x0019398c;
                        continue;
                    }
                    pc = 0x00193978;
                    continue;
                }
                0x00193978 => {
                    c.cycles += 2;
                    let t = c.g[S1][0] != 0u64;

                    if t {
                        pc = 0x0019398c;
                        continue;
                    }
                    pc = 0x00193980;
                    continue;
                }
                0x00193980 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00193988);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2dc));
                    call_ptr(c, 0x00265688, 0x00193988)?;
                    c.g[A0][0] = c.g[S2][0];
                    pc = 0x0019398c;
                    continue;
                }
                0x0019398c => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x00193994);
                    c.g[A1][0] = 0x1;
                    Callee {
                        addr: 0x00192740,
                        f: ee::s02::cFEStateBraggingRights_SetRowsVisible,
                    }
                    .call(c, 0x00193994)?;
                    c.g[RA][0] = sx(0x0019399c);
                    c.g[A0][0] = c.g[S3][0];
                    Callee {
                        addr: 0x00194080,
                        f: ee::s02::cFEStateBraggingRights_StartWaitAnim,
                    }
                    .call(c, 0x0019399c)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                    if t {
                        pc = 0x001939bc;
                        continue;
                    }
                    pc = 0x001939a4;
                    continue;
                }
                0x001939a4 => {
                    c.cycles += 5;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x34));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x34));
                    if t {
                        if back_edge_unwinds(c, 0x00193958) {
                            return Ok(());
                        }
                        pc = 0x00193958;
                        continue;
                    }
                    pc = 0x001939b8;
                    continue;
                }
                0x001939b8 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                    pc = 0x001939bc;
                    continue;
                }
                0x001939bc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;

                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        pc = 0x00193be0;
                        continue;
                    }
                    pc = 0x001939c8;
                    continue;
                }
                0x001939c8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6f8));

                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        pc = 0x00193be0;
                        continue;
                    }
                    pc = 0x001939d4;
                    continue;
                }
                0x001939d4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x72c));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S3][0];
                    if t {
                        pc = 0x00193bdc;
                        continue;
                    }
                    pc = 0x001939e0;
                    continue;
                }
                0x001939e0 => {
                    c.cycles += 10;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                    c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x18));
                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00193a00);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00193a00)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        pc = 0x00193bdc;
                        continue;
                    }
                    pc = 0x00193a08;
                    continue;
                }
                0x00193a08 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x00193a14);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039f400, 0x00193a14)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x00193be0;
                        continue;
                    }
                    pc = 0x00193a1c;
                    continue;
                }
                0x00193a1c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                        pc = 0x00193ab8;
                        continue;
                    }
                    pc = 0x00193a2c;
                    continue;
                }
                0x00193a2c => {
                    c.cycles += 35;
                    c.g[RA][0] = sx(0x00193a34);
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x001a9c50, 0x00193a34)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    c.g[V0][0] = 0x2;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x268));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                    c.g[RA][0] = sx(0x00193a50);
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                    call_ptr(c, 0x00317670, 0x00193a50)?;
                    c.g[V1][0] = 0x3;
                    c.g[A0][0] = 0x460000;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                    c.g[RA][0] = sx(0x00193a68);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                    call_ptr(c, 0x00317670, 0x00193a68)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x00193a78);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x001c57c0, 0x00193a78)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x00193a84);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                    call_ptr(c, 0x00317670, 0x00193a84)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00193a94);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x001c57c0, 0x00193a94)?;
                    c.g[V0][0] = 0x838;
                    c.g[V1][0] = 0x1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                    c.g[A0][0] = c.g[S3][0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x168), c.g[V1][0]);
                    c.g[RA][0] = sx(0x00193ab0);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x001a9bc0, 0x00193ab0)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x00193be0;
                        continue;
                    }
                    pc = 0x00193ab8;
                    continue;
                }
                0x00193ab8 => {
                    c.cycles += 10;
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x18));
                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00193ad8);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00193ad8)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        pc = 0x00193aec;
                        continue;
                    }
                    pc = 0x00193ae0;
                    continue;
                }
                0x00193ae0 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x00193aec);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039f400, 0x00193aec)?;
                    pc = 0x00193aec;
                    continue;
                }
                0x00193aec => {
                    c.cycles += 7;
                    c.g[A0][0] = 0x530000;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                    c.g[RA][0] = sx(0x00193b00);
                    c.g[A2][0] = 0x20;
                    call_ptr(c, 0x003e6448, 0x00193b00)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x00193be0;
                        continue;
                    }
                    pc = 0x00193b08;
                    continue;
                }
                0x00193b08 => {
                    c.cycles += 8;
                    c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x18));
                    c.g[A0][0] = 0x34;
                    c.mult(0, 4, 16, 4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6d0));
                    c.g[RA][0] = sx(0x00193b20);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    Callee {
                        addr: 0x001927b0,
                        f: ee::s02::cFEStateBraggingRights_UpdateRowColors,
                    }
                    .call(c, 0x00193b20)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x00193be0;
                        continue;
                    }
                    pc = 0x00193b28;
                    continue;
                }
                0x00193b28 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    let t = c.g[V0][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x00193be0;
                        continue;
                    }
                    pc = 0x00193b38;
                    continue;
                }
                0x00193b38 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x00193b44);
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x001a8918, 0x00193b44)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x00193be0;
                        continue;
                    }
                    pc = 0x00193b4c;
                    continue;
                }
                0x00193b4c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;

                    if c.g[V0][0] != 0u64 {
                        c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x18));
                        pc = 0x00193b90;
                        continue;
                    }
                    pc = 0x00193b5c;
                    continue;
                }
                0x00193b5c => {
                    c.cycles += 13;
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6d0));
                    c.g[RA][0] = sx(0x00193b6c);
                    c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2dc));
                    Callee {
                        addr: 0x00192918,
                        f: ee::s02::cFEStateBraggingRights_FindRowIndex,
                    }
                    .call(c, 0x00193b6c)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00193b78);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x0039a738, 0x00193b78)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00193b88);
                    c.g[A2][0] = c.g[V0][0];
                    call_ptr(c, 0x00265568, 0x00193b88)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x00193be0;
                        continue;
                    }
                    pc = 0x00193b90;
                    continue;
                }
                0x00193b90 => {
                    c.cycles += 12;
                    c.g[A0][0] = 0x34;
                    c.g[A1][0] = c.g[S4][0];
                    c.mult(0, 4, 16, 4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6d0));
                    c.g[RA][0] = sx(0x00193ba8);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    Callee {
                        addr: 0x00192918,
                        f: ee::s02::cFEStateBraggingRights_FindRowIndex,
                    }
                    .call(c, 0x00193ba8)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x738));
                    c.g[A0][0] = 0xffffffffffffffff;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S1][0] = c.lb(c.g[V1][0] as u32);

                    if c.g[S1][0] == c.g[A0][0] {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        pc = 0x00193be0;
                        continue;
                    }
                    pc = 0x00193bc0;
                    continue;
                }
                0x00193bc0 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x00193bc8);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x0039a738, 0x00193bc8)?;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00193bdc);
                    c.g[A3][0] = c.g[V0][0];
                    Callee {
                        addr: 0x00193de0,
                        f: ee::s02::cFEStateBraggingRights_DispatchRowAction,
                    }
                    .call(c, 0x00193bdc)?;
                    pc = 0x00193bdc;
                    continue;
                }
                0x00193bdc => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    pc = 0x00193be0;
                    continue;
                }
                0x00193be0 => {
                    c.cycles += 8;
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
                }
                _ => {
                    c.jump_out(pc, 0x001938f8, 0x00193c00);
                    return Ok(());
                }
            }
        }
    });
}

/// `void cFEStateBraggingRights_v25(self, popup)` (0x193c00): fill the help
/// popup of the bragging-rights screen: four options, the screen name hash and
/// two description lines, then the four option button texts and their button ids
/// (hash of each string from the game data at 0x45fde8..0x45fe98).
pub fn c_fe_state_bragging_rights_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 61;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A1][0] = 0x4;
        c.g[RA][0] = sx(0x00193c1c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001da528, 0x00193c1c)?;
        c.g[A0][0] = 0x460000;
        c.g[RA][0] = sx(0x00193c28);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x218));
        call_ptr(c, 0x00317670, 0x00193c28)?;
        c.g[A0][0] = 0x460000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[V0][0]);
        c.g[RA][0] = sx(0x00193c38);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x200));
        call_ptr(c, 0x00317670, 0x00193c38)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00193c48);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x001da530, 0x00193c48)?;
        c.g[A0][0] = 0x460000;
        c.g[RA][0] = sx(0x00193c54);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1e0));
        call_ptr(c, 0x00317670, 0x00193c54)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00193c64);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x001da530, 0x00193c64)?;
        c.g[A0][0] = 0x460000;
        c.g[RA][0] = sx(0x00193c70);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c0));
        call_ptr(c, 0x00317670, 0x00193c70)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x00193c84);
        c.g[A3][0] = 0x4;
        call_ptr(c, 0x001da510, 0x00193c84)?;
        c.g[A0][0] = 0x460000;
        c.g[RA][0] = sx(0x00193c90);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1a0));
        call_ptr(c, 0x00317670, 0x00193c90)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        c.g[RA][0] = sx(0x00193ca4);
        c.g[A3][0] = 0x3;
        call_ptr(c, 0x001da510, 0x00193ca4)?;
        c.g[A0][0] = 0x460000;
        c.g[RA][0] = sx(0x00193cb0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x180));
        call_ptr(c, 0x00317670, 0x00193cb0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0x2;
        c.g[RA][0] = sx(0x00193cc4);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x001da510, 0x00193cc4)?;
        c.g[A0][0] = 0x460000;
        c.g[RA][0] = sx(0x00193cd0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x168));
        call_ptr(c, 0x00317670, 0x00193cd0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0x3;
        c.g[RA][0] = sx(0x00193ce4);
        c.g[A3][0] = 0x1;
        call_ptr(c, 0x001da510, 0x00193ce4)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `int cFEStateBraggingRights_OnWidgetNotify(self, ctrl, msg)` (0x193cf8): the
/// notify messages 5..9 (jump table at 0x45feb0) of the screen's two menus
/// (`self+0x6d0`, `self+0x704`, found by their control pointer): 5 recolours the
/// menu (and sends the highlight message when online), 6 and 7 answer whether the
/// screen may leave, 8/9 and anything else answer 0x100/0x101.
/// Resumes at any point other than the entry through the generated body.
pub fn c_fe_state_bragging_rights_on_widget_notify(c: &mut Cpu) {
    if c.pc != 0x00193cf8 {
        return c_fe_state_bragging_rights_on_widget_notify_resume(c);
    }
    guest(c, |c| 'l_193cf8: loop {
        'b_193d34: {
            'b_193d24: {
                c.cycles += 8;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.g[A3][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x6d0));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x6d0));
                let t = c.g[V0][0] != c.g[A1][0];
                c.g[T0][0] = 0u64;
                if t {
                    break 'b_193d24;
                } else {
                    c.cycles += 3;
                    c.g[S0][0] = c.g[A0][0];
                    let t = true;
                    c.g[T0][0] = 0x1;
                    if t {
                        break 'b_193d34;
                    } else {
                        break 'b_193d24;
                    }
                }
            }
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x704));
            c.g[S0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x704));
            c.g[V0][0] = c.g[V0][0] ^ c.g[A1][0];
            if c.g[V0][0] != 0 {
                c.g[S0][0] = 0u64;
            }
            break 'b_193d34;
        }
        'b_193dcc: {
            'b_193dc8: {
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5));
                c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x460000;
                if t {
                    break 'b_193dc8;
                } else {
                    'b_193db8: {
                        'b_193d98: {
                            'b_193d64: {
                                c.cycles += 6;
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x150));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                let t = c.g[A0][0] as u32;
                                match t {
                                    0x00193cf8 => {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00193cf8);
                                        }
                                        continue 'l_193cf8;
                                    }
                                    0x00193d5c => {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = 0x100;
                                        if t {
                                            break 'b_193dcc;
                                        } else {
                                            break 'b_193d64;
                                        }
                                    }
                                    0x00193d64 => {
                                        break 'b_193d64;
                                    }
                                    0x00193d98 => {
                                        break 'b_193d98;
                                    }
                                    0x00193db8 => {
                                        break 'b_193db8;
                                    }
                                    _ => {
                                        c.jump_out(t, 0x00193cf8, 0x00193ddc);
                                        return Ok(());
                                    }
                                }
                            }
                            'b_193d88: {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                let t = c.g[V0][0] != 0u64;

                                if t {
                                    break 'b_193d88;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[T0][0] == 0u64;

                                    if t {
                                        break 'b_193d88;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x95));
                                        c.g[RA][0] = sx(0x00193d88);
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2dc));
                                        call_ptr(c, 0x00265458, 0x00193d88)?;
                                        break 'b_193d88;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x00193d90);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001927b0,
                                f: ee::s02::cFEStateBraggingRights_UpdateRowColors,
                            }
                            .call(c, 0x00193d90)?;
                            let t = true;
                            c.g[V0][0] = 0x101;
                            if t {
                                break 'b_193dcc;
                            } else {
                                break 'b_193d98;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x6f8));
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_193dcc;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x72c));
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = 0x101;
                            if t {
                                break 'b_193dcc;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_193dcc;
                                } else {
                                    break 'b_193db8;
                                }
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                    c.g[V1][0] = c.g[V1][0] ^ 0x1;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_193dcc;
                    } else {
                        break 'b_193dc8;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x101;
            break 'b_193dcc;
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// Native resume of `c_fe_state_bragging_rights_on_widget_notify` (converted from the `entries` body): any entry
/// other than 0x00193cf8, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn c_fe_state_bragging_rights_on_widget_notify_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00193cf8 => {
                    c.cycles += 8;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.g[A3][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x6d0));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x6d0));
                    let t = c.g[V0][0] != c.g[A1][0];
                    c.g[T0][0] = 0u64;
                    if t {
                        pc = 0x00193d24;
                        continue;
                    }
                    pc = 0x00193d18;
                    continue;
                }
                0x00193d18 => {
                    c.cycles += 3;
                    c.g[S0][0] = c.g[A0][0];
                    let t = true;
                    c.g[T0][0] = 0x1;
                    if t {
                        pc = 0x00193d34;
                        continue;
                    }
                    pc = 0x00193d24;
                    continue;
                }
                0x00193d24 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x704));
                    c.g[S0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x704));
                    c.g[V0][0] = c.g[V0][0] ^ c.g[A1][0];
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = 0u64;
                    }
                    pc = 0x00193d34;
                    continue;
                }
                0x00193d34 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5));
                    c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x00193dc8;
                        continue;
                    }
                    pc = 0x00193d44;
                    continue;
                }
                0x00193d44 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x150));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00193d5c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x100;
                    if t {
                        pc = 0x00193dcc;
                        continue;
                    }
                    pc = 0x00193d64;
                    continue;
                }
                0x00193d64 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        pc = 0x00193d88;
                        continue;
                    }
                    pc = 0x00193d74;
                    continue;
                }
                0x00193d74 => {
                    c.cycles += 2;
                    let t = c.g[T0][0] == 0u64;

                    if t {
                        pc = 0x00193d88;
                        continue;
                    }
                    pc = 0x00193d7c;
                    continue;
                }
                0x00193d7c => {
                    c.cycles += 3;
                    c.g[A1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x95));
                    c.g[RA][0] = sx(0x00193d88);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2dc));
                    call_ptr(c, 0x00265458, 0x00193d88)?;
                    pc = 0x00193d88;
                    continue;
                }
                0x00193d88 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00193d90);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001927b0,
                        f: ee::s02::cFEStateBraggingRights_UpdateRowColors,
                    }
                    .call(c, 0x00193d90)?;
                    let t = true;
                    c.g[V0][0] = 0x101;
                    if t {
                        pc = 0x00193dcc;
                        continue;
                    }
                    pc = 0x00193d98;
                    continue;
                }
                0x00193d98 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x6f8));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00193dcc;
                        continue;
                    }
                    pc = 0x00193da4;
                    continue;
                }
                0x00193da4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x72c));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x101;
                    if t {
                        pc = 0x00193dcc;
                        continue;
                    }
                    pc = 0x00193db0;
                    continue;
                }
                0x00193db0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00193dcc;
                        continue;
                    }
                    pc = 0x00193db8;
                    continue;
                }
                0x00193db8 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                    c.g[V1][0] = c.g[V1][0] ^ 0x1;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00193dcc;
                        continue;
                    }
                    pc = 0x00193dc8;
                    continue;
                }
                0x00193dc8 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x101;
                    pc = 0x00193dcc;
                    continue;
                }
                0x00193dcc => {
                    c.cycles += 4;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00193cf8, 0x00193ddc);
                    return Ok(());
                }
            }
        }
    });
}

/// `void cFEStateBraggingRights_DispatchRowAction(self, player, kind, value)`
/// (0x193de0): store a menu row's value (kinds 0..7, jump table at 0x45fed0) in
/// the BE rules entry (`0x5308b8 + 12 * index`) as a bit or word, for `player` or
/// the other player (index 1 when `player` is 0). Other kinds do nothing.
/// Resumes at any point other than the entry through the generated body.
pub fn c_fe_state_bragging_rights_dispatch_row_action(c: &mut Cpu) {
    if c.pc != 0x00193de0 {
        return c_fe_state_bragging_rights_dispatch_row_action_resume(c);
    }
    guest(c, |c| {
        'b_193f30: {
            c.cycles += 3;
            c.g[V0][0] = (c.g[A2][0] < 0x8u64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[T0][0] = c.g[A1][0];
            if t {
                break 'b_193f30;
            } else {
                'b_193e48: {
                    'b_193f14: {
                        'b_193ef4: {
                            'b_193ed4: {
                                'b_193eb4: {
                                    c.cycles += 7;
                                    c.g[V0][0] = 0x460000;
                                    c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x130));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x00193e08 => {
                                            c.cycles += 8;
                                            c.g[A0][0] = (c.g[T0][0] < 0x1u64) as u64;
                                            c.g[V1][0] = 0xc;
                                            c.mult(0, 4, 4, 3);
                                            c.g[V0][0] = 0x530000;
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            let t = c.g[RA][0] as u32;
                                            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                                            c.pc = t;
                                            return Ok(());
                                        }
                                        0x00193e28 => {
                                            c.cycles += 8;
                                            c.g[V0][0] = 0xc;
                                            c.g[A1][0] = (c.g[T0][0] < 0x1u64) as u64;
                                            c.mult(0, 5, 5, 2);
                                            c.g[V1][0] = 0x530000;
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                                            c.g[A0][0] = c.g[A3][0] & 0x1;
                                            c.g[A2][0] = 0xfffffffffffffff7;
                                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                                            break 'b_193e48;
                                        }
                                        0x00193e60 => {
                                            c.cycles += 13;
                                            c.g[V0][0] = 0xc;
                                            c.g[A0][0] = (c.g[T0][0] < 0x1u64) as u64;
                                            c.mult(0, 4, 4, 2);
                                            c.g[V1][0] = 0x530000;
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                                            c.g[A1][0] = 0xfffffffffffffffe;
                                            c.g[A2][0] = c.g[A3][0] & 0x1;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                            c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                                            c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                                            let t = c.g[RA][0] as u32;
                                            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                                            c.pc = t;
                                            return Ok(());
                                        }
                                        0x00193e94 => {
                                            c.cycles += 8;
                                            c.g[A1][0] = 0xc;
                                            c.g[V1][0] = 0x530000;
                                            c.mult(0, 5, 8, 5);
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                                            c.g[A0][0] = c.g[A3][0] & 0x1;
                                            c.g[A2][0] = 0xffffffffffffffef;
                                            let t = true;
                                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 4);
                                            if t {
                                                break 'b_193e48;
                                            } else {
                                                break 'b_193eb4;
                                            }
                                        }
                                        0x00193eb4 => {
                                            break 'b_193eb4;
                                        }
                                        0x00193ed4 => {
                                            break 'b_193ed4;
                                        }
                                        0x00193ef4 => {
                                            break 'b_193ef4;
                                        }
                                        0x00193f14 => {
                                            break 'b_193f14;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x00193de0, 0x00193f38);
                                            return Ok(());
                                        }
                                    }
                                }
                                c.cycles += 8;
                                c.g[A1][0] = 0xc;
                                c.g[V1][0] = 0x530000;
                                c.mult(0, 5, 8, 5);
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                                c.g[A0][0] = c.g[A3][0] & 0x1;
                                c.g[A2][0] = 0xffffffffffffffdf;
                                let t = true;
                                c.g[A0][0] = sx((c.g[A0][0] as u32) << 5);
                                if t {
                                    break 'b_193e48;
                                } else {
                                    break 'b_193ed4;
                                }
                            }
                            c.cycles += 8;
                            c.g[A1][0] = 0xc;
                            c.g[V1][0] = 0x530000;
                            c.mult(0, 5, 8, 5);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                            c.g[A0][0] = c.g[A3][0] & 0x1;
                            c.g[A2][0] = 0xfffffffffffffffd;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
                            if t {
                                break 'b_193e48;
                            } else {
                                break 'b_193ef4;
                            }
                        }
                        c.cycles += 8;
                        c.g[A1][0] = 0xc;
                        c.g[V1][0] = 0x530000;
                        c.mult(0, 5, 8, 5);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                        c.g[A0][0] = c.g[A3][0] & 0x1;
                        c.g[A2][0] = 0xfffffffffffffffb;
                        let t = true;
                        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                        if t {
                            break 'b_193e48;
                        } else {
                            break 'b_193f14;
                        }
                    }
                    c.cycles += 7;
                    c.g[A0][0] = (c.g[T0][0] < 0x1u64) as u64;
                    c.g[V1][0] = 0xc;
                    c.mult(0, 4, 4, 3);
                    c.g[V0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A3][0]);
                    break 'b_193f30;
                }
                c.cycles += 6;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                let t = c.g[RA][0] as u32;
                c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                c.pc = t;
                return Ok(());
            }
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.pc = t;
        return Ok(());
    });
}

/// Native resume of `c_fe_state_bragging_rights_dispatch_row_action` (converted from the `entries` body): any entry
/// other than 0x00193de0, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn c_fe_state_bragging_rights_dispatch_row_action_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00193de0 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[A2][0] < 0x8u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[T0][0] = c.g[A1][0];
                    if t {
                        pc = 0x00193f30;
                        continue;
                    }
                    pc = 0x00193dec;
                    continue;
                }
                0x00193dec => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x460000;
                    c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x130));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00193e08 => {
                    c.cycles += 8;
                    c.g[A0][0] = (c.g[T0][0] < 0x1u64) as u64;
                    c.g[V1][0] = 0xc;
                    c.mult(0, 4, 4, 3);
                    c.g[V0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.pc = t;
                    return Ok(());
                }
                0x00193e28 => {
                    c.cycles += 8;
                    c.g[V0][0] = 0xc;
                    c.g[A1][0] = (c.g[T0][0] < 0x1u64) as u64;
                    c.mult(0, 5, 5, 2);
                    c.g[V1][0] = 0x530000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                    c.g[A0][0] = c.g[A3][0] & 0x1;
                    c.g[A2][0] = 0xfffffffffffffff7;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                    pc = 0x00193e48;
                    continue;
                }
                0x00193e48 => {
                    c.cycles += 6;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                    let t = c.g[RA][0] as u32;
                    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                    c.pc = t;
                    return Ok(());
                }
                0x00193e60 => {
                    c.cycles += 13;
                    c.g[V0][0] = 0xc;
                    c.g[A0][0] = (c.g[T0][0] < 0x1u64) as u64;
                    c.mult(0, 4, 4, 2);
                    c.g[V1][0] = 0x530000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                    c.g[A1][0] = 0xfffffffffffffffe;
                    c.g[A2][0] = c.g[A3][0] & 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                    let t = c.g[RA][0] as u32;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                    c.pc = t;
                    return Ok(());
                }
                0x00193e94 => {
                    c.cycles += 8;
                    c.g[A1][0] = 0xc;
                    c.g[V1][0] = 0x530000;
                    c.mult(0, 5, 8, 5);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                    c.g[A0][0] = c.g[A3][0] & 0x1;
                    c.g[A2][0] = 0xffffffffffffffef;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 4);
                    if t {
                        if back_edge_unwinds(c, 0x00193e48) {
                            return Ok(());
                        }
                        pc = 0x00193e48;
                        continue;
                    }
                    pc = 0x00193eb4;
                    continue;
                }
                0x00193eb4 => {
                    c.cycles += 8;
                    c.g[A1][0] = 0xc;
                    c.g[V1][0] = 0x530000;
                    c.mult(0, 5, 8, 5);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                    c.g[A0][0] = c.g[A3][0] & 0x1;
                    c.g[A2][0] = 0xffffffffffffffdf;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 5);
                    if t {
                        if back_edge_unwinds(c, 0x00193e48) {
                            return Ok(());
                        }
                        pc = 0x00193e48;
                        continue;
                    }
                    pc = 0x00193ed4;
                    continue;
                }
                0x00193ed4 => {
                    c.cycles += 8;
                    c.g[A1][0] = 0xc;
                    c.g[V1][0] = 0x530000;
                    c.mult(0, 5, 8, 5);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                    c.g[A0][0] = c.g[A3][0] & 0x1;
                    c.g[A2][0] = 0xfffffffffffffffd;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
                    if t {
                        if back_edge_unwinds(c, 0x00193e48) {
                            return Ok(());
                        }
                        pc = 0x00193e48;
                        continue;
                    }
                    pc = 0x00193ef4;
                    continue;
                }
                0x00193ef4 => {
                    c.cycles += 8;
                    c.g[A1][0] = 0xc;
                    c.g[V1][0] = 0x530000;
                    c.mult(0, 5, 8, 5);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                    c.g[A0][0] = c.g[A3][0] & 0x1;
                    c.g[A2][0] = 0xfffffffffffffffb;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                    if t {
                        if back_edge_unwinds(c, 0x00193e48) {
                            return Ok(());
                        }
                        pc = 0x00193e48;
                        continue;
                    }
                    pc = 0x00193f14;
                    continue;
                }
                0x00193f14 => {
                    c.cycles += 7;
                    c.g[A0][0] = (c.g[T0][0] < 0x1u64) as u64;
                    c.g[V1][0] = 0xc;
                    c.mult(0, 4, 4, 3);
                    c.g[V0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A3][0]);
                    pc = 0x00193f30;
                    continue;
                }
                0x00193f30 => {
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;

                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00193de0, 0x00193f38);
                    return Ok(());
                }
            }
        }
    });
}

/// `int cFEStateBraggingRights_GetRowDefault(self, player, kind)` (0x193f38):
/// read back the BE rules value of a menu row (kinds 0..7, jump table at
/// 0x45fef0); -1 for other kinds.
/// Resumes at any point other than the entry through the generated body.
pub fn c_fe_state_bragging_rights_get_row_default(c: &mut Cpu) {
    if c.pc != 0x00193f38 {
        return c_fe_state_bragging_rights_get_row_default_resume(c);
    }
    guest(c, |c| {
        c.cycles += 3;
        c.g[V0][0] = (c.g[A2][0] < 0x8u64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x460000;
        if t {
            c.cycles += 2;
            let t = c.g[RA][0] as u32;
            c.g[V0][0] = 0xffffffffffffffff;
            c.pc = t;
            return Ok(());
        } else {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x110));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
            let t = c.g[A0][0] as u32;
            match t {
                0x00193f5c => {
                    c.cycles += 8;
                    c.g[V0][0] = 0xc;
                    c.g[A0][0] = (c.g[A1][0] < 0x1u64) as u64;
                    c.mult(0, 4, 4, 2);
                    c.g[V1][0] = 0x530000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.pc = t;
                    return Ok(());
                }
                0x00193f7c => {
                    c.cycles += 10;
                    c.g[V1][0] = (c.g[A1][0] < 0x1u64) as u64;
                    c.g[A0][0] = 0xc;
                    c.mult(0, 3, 3, 4);
                    c.g[V0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00193fa4 => {
                    c.cycles += 9;
                    c.g[V1][0] = (c.g[A1][0] < 0x1u64) as u64;
                    c.g[A0][0] = 0xc;
                    c.mult(0, 3, 3, 4);
                    c.g[V0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00193fc8 => {
                    c.cycles += 9;
                    c.g[V1][0] = 0xc;
                    c.g[V0][0] = 0x530000;
                    c.mult(0, 3, 5, 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00193fec => {
                    c.cycles += 9;
                    c.g[V1][0] = 0xc;
                    c.g[V0][0] = 0x530000;
                    c.mult(0, 3, 5, 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00194010 => {
                    c.cycles += 9;
                    c.g[V1][0] = 0xc;
                    c.g[V0][0] = 0x530000;
                    c.mult(0, 3, 5, 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00194034 => {
                    c.cycles += 9;
                    c.g[V1][0] = 0xc;
                    c.g[V0][0] = 0x530000;
                    c.mult(0, 3, 5, 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00194058 => {
                    c.cycles += 8;
                    c.g[V0][0] = 0xc;
                    c.g[A0][0] = (c.g[A1][0] < 0x1u64) as u64;
                    c.mult(0, 4, 4, 2);
                    c.g[V1][0] = 0x530000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(t, 0x00193f38, 0x00194080);
                    return Ok(());
                }
            }
        }
    });
}

/// Native resume of `c_fe_state_bragging_rights_get_row_default` (converted from the `entries` body): any entry
/// other than 0x00193f38, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn c_fe_state_bragging_rights_get_row_default_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00193f38 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[A2][0] < 0x8u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x00194078;
                        continue;
                    }
                    pc = 0x00193f44;
                    continue;
                }
                0x00193f44 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x110));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00193f5c => {
                    c.cycles += 8;
                    c.g[V0][0] = 0xc;
                    c.g[A0][0] = (c.g[A1][0] < 0x1u64) as u64;
                    c.mult(0, 4, 4, 2);
                    c.g[V1][0] = 0x530000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.pc = t;
                    return Ok(());
                }
                0x00193f7c => {
                    c.cycles += 10;
                    c.g[V1][0] = (c.g[A1][0] < 0x1u64) as u64;
                    c.g[A0][0] = 0xc;
                    c.mult(0, 3, 3, 4);
                    c.g[V0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00193fa4 => {
                    c.cycles += 9;
                    c.g[V1][0] = (c.g[A1][0] < 0x1u64) as u64;
                    c.g[A0][0] = 0xc;
                    c.mult(0, 3, 3, 4);
                    c.g[V0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00193fc8 => {
                    c.cycles += 9;
                    c.g[V1][0] = 0xc;
                    c.g[V0][0] = 0x530000;
                    c.mult(0, 3, 5, 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00193fec => {
                    c.cycles += 9;
                    c.g[V1][0] = 0xc;
                    c.g[V0][0] = 0x530000;
                    c.mult(0, 3, 5, 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00194010 => {
                    c.cycles += 9;
                    c.g[V1][0] = 0xc;
                    c.g[V0][0] = 0x530000;
                    c.mult(0, 3, 5, 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00194034 => {
                    c.cycles += 9;
                    c.g[V1][0] = 0xc;
                    c.g[V0][0] = 0x530000;
                    c.mult(0, 3, 5, 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00194058 => {
                    c.cycles += 8;
                    c.g[V0][0] = 0xc;
                    c.g[A0][0] = (c.g[A1][0] < 0x1u64) as u64;
                    c.mult(0, 4, 4, 2);
                    c.g[V1][0] = 0x530000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.pc = t;
                    return Ok(());
                }
                0x00194078 => {
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = 0xffffffffffffffff;
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00193f38, 0x00194080);
                    return Ok(());
                }
            }
        }
    });
}

/// Gp-relative word that is 1 while the online front end is up
/// (`isOn00194138` in the reference).
pub(crate) const OC851_ONLINE_FLAG: u32 = 0x1680;

/// `void* cFEStateBraggingRights_handleEvent(self, int msgId, int data)`
/// (0x194138): while the online flag at `gp-0x1680` is 1, forward to
/// `cFEStateNet_handleEvent` (0x1a8b88); otherwise return 0.
pub fn bragging_rights_v24(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let flag = Addr(r32(c, GP).wrapping_sub(OC851_ONLINE_FLAG)).lw(c);
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = flag ^ 1;
        frame.save_ra(c, 0);
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            call_ptr(c, 0x001a_8b88, 0x0019_415c)?;
        } else {
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBraggingRights_onPopupEvent(self, cUIState* popup, int event)`
/// (0x194168): while online, popup id 0x838 (field +0x18) goes to
/// `OnChildEventQuit` (0x194440), any other to `cFEStateNet_onPopupEvent`
/// (0x1a97b8).
pub fn bragging_rights_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let flag = Addr(r32(c, GP).wrapping_sub(OC851_ONLINE_FLAG)).lw(c);
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = flag ^ 1;
        frame.save_ra(c, 0);
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 4;
            c.g[V1][0] = Addr(r32(c, A1).wrapping_add(0x18)).lw(c);
            c.g[V0][0] = 0x838;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 4;
                Callee {
                    addr: 0x0019_4440,
                    f: ee::s02::f_00194440,
                }
                .call(c, 0x0019_4194)?;
                frame.restore_ra(c, 0);
            } else {
                c.cycles += 2;
                call_ptr(c, 0x001a_97b8, 0x0019_41a4)?;
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// Stride of one `cBraggingRightsMenu` entry (the row tables use `* 0x34`).
pub(crate) const OC851_MENU_STRIDE: u32 = 0x34;

/// `int cFEStateBraggingRights_handleMessage(self, int msg, data)` (0x1941b0):
/// the message switch of the Bragging Rights screen. Messages 0xee..0x113
/// index a jump table at 0x45ff40; the cases are row selection (251), list
/// box selection (252), confirm (253), unconfirm and prompt (254), the
/// network back-outs (238, 241, 248, 250, 274, 275) and the chat/online main
/// menu switch (268). Everything else goes to `NetFEState` (0x1a8e40).
///
/// The generated body has a resume twin; this entry keeps the guard.
pub fn bragging_rights_v31(c: &mut Cpu) {
    if c.pc != 0x0019_41b0 {
        return bragging_rights_v31_resume(c);
    }
    guest(c, |c| {
        loop {
            c.cycles += 12;
            let frame = Frame::open(c, 0x70);
            c.g[V1][0] = sx(r32(c, A1).wrapping_sub(0xee));
            frame.save(c, 17, 0x50);
            c.g[V0][0] = (c.g[V1][0] < 0x26) as u64;
            frame.save(c, 18, 0x40);
            c.g[S1][0] = c.g[A0][0];
            frame.save(c, 16, 0x60);
            c.g[S2][0] = c.g[A2][0];
            frame.save(c, 19, 0x30);
            frame.save(c, 20, 0x20);
            let in_range = c.g[V0][0] != 0;
            frame.save_ra(c, 0x10);
            // Tail entry: 2 = 0x194418 (v0 = 0, then 1), 1 = 0x19441c, 0 = 0x194420.
            let tail: u8;
            let mut target = 0x0019_4404u32;
            if in_range {
                c.cycles += 7;
                c.g[V0][0] = 0x0046_0000;
                c.g[V1][0] = sx(r32(c, V1) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xc0));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[A0][0] = Addr(r32(c, V1)).lw(c);
                target = r32(c, A0);
            }
            match target {
                0x0019_41b0 => {
                    if event_due(c) {
                        deliver_event_here(c, 0x0019_41b0);
                    }
                    continue;
                }
                // Message 251: select an entry of the player's menu.
                0x0019_41fc => {
                    c.cycles += 9;
                    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                    c.g[V1][0] = OC851_MENU_STRIDE as u64;
                    c.g[A1][0] = Addr(r32(c, S2).wrapping_add(0xc)).lbu(c);
                    c.mult(0, 4, 2, 3);
                    c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x6d0)).lw(c);
                    call_ptr(c, 0x0039_b960, 0x0019_4218)?;
                    c.g[V0][0] = 0;
                    tail = 1;
                }
                // Message 252: choose a list box entry by key.
                0x0019_4220 => {
                    c.cycles += 14;
                    c.g[V1][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                    c.g[S0][0] = OC851_MENU_STRIDE as u64;
                    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(0xc)).lw(c);
                    c.g[A2][0] = c.g[SP][0];
                    c.mult(0, 4, 3, 16);
                    c.g[A1][0] = Addr(r32(c, S2).wrapping_add(0x10)).lw(c);
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                    c.g[S3][0] = Addr(r32(c, V1).wrapping_add(0x6d8)).lw(c);
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x0039_a768, 0x0019_4250)?;
                    let found = c.g[V0][0] != 0;
                    c.g[A1][0] = Addr(sp(c)).lbu(c);
                    if !found {
                        tail = 2;
                    } else {
                        c.cycles += 14;
                        c.g[A0][0] = c.g[S3][0];
                        call_ptr(c, 0x0039_a7a8, 0x0019_4260)?;
                        c.g[A0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                        c.g[A1][0] = c.g[S3][0];
                        c.mult(0, 4, 4, 16);
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6d0));
                        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                        Callee {
                            addr: 0x0019_2918,
                            f: ee::s02::f_00192918,
                        }
                        .call(c, 0x0019_4278)?;
                        c.g[V1][0] = Addr(r32(c, S1).wrapping_add(0x738)).lw(c);
                        c.g[A0][0] = 0xffff_ffff_ffff_ffff;
                        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                        c.g[S4][0] = Addr(r32(c, V1)).lb(c);
                        let none = c.g[S4][0] == c.g[A0][0];
                        c.g[V0][0] = 0;
                        if none {
                            tail = 1;
                        } else {
                            c.cycles += 10;
                            c.g[S0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                            c.g[A0][0] = c.g[S3][0];
                            call_ptr(c, 0x0039_a738, 0x0019_429c)?;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A2][0] = c.g[S4][0];
                            c.g[A3][0] = c.g[V0][0];
                            Callee {
                                addr: 0x0019_3de0,
                                f: ee::s02::f_00193de0,
                            }
                            .call(c, 0x0019_42b0)?;
                            c.g[V0][0] = 0;
                            tail = 1;
                        }
                    }
                }
                // Message 253: mark the player's menu confirmed.
                0x0019_42b8 => {
                    c.cycles += 11;
                    c.g[A0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                    c.g[V0][0] = OC851_MENU_STRIDE as u64;
                    c.g[A1][0] = 1;
                    c.mult(0, 4, 4, 2);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6d0));
                    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                    Callee {
                        addr: 0x0019_2740,
                        f: ee::s02::f_00192740,
                    }
                    .call(c, 0x0019_42d4)?;
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x0019_4080,
                        f: ee::s02::f_00194080,
                    }
                    .call(c, 0x0019_42dc)?;
                    c.g[V0][0] = 0;
                    tail = 1;
                }
                // Messages 238, 241, 248, 250, 274, 275: network back-outs.
                0x0019_42e4 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x001a_b288, 0x0019_42ec)?;
                    c.g[V0][0] = 0;
                    tail = 1;
                }
                // Message 268: switch to the chat room or the online main menu.
                0x0019_42f4 => {
                    c.cycles += 4;
                    c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(0xc8)).lw(c);
                    call_ptr(c, 0x0025_da90, 0x0019_42fc)?;
                    let lobby = c.g[V0][0] != 0;
                    c.g[A1][0] = 0x0046_0000;
                    if lobby {
                        c.cycles += 10;
                        c.g[A0][0] = 0x73c;
                        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0xf0));
                        c.g[A2][0] = 0;
                        c.g[A3][0] = 0;
                        call_ptr(c, 0x0031_7d70, 0x0019_4318)?;
                        c.g[A1][0] = Addr(r32(c, S1).wrapping_add(0x10)).lw(c);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x001b_1f88, 0x0019_4324)?;
                        c.g[A0][0] = c.g[S1][0];
                    } else {
                        c.cycles += 10;
                        c.g[A1][0] = 0x0046_0000;
                        c.g[A0][0] = 0x6d0;
                        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0xd8));
                        c.g[A2][0] = 0;
                        c.g[A3][0] = 0;
                        call_ptr(c, 0x0031_7d70, 0x0019_4344)?;
                        c.g[A1][0] = Addr(r32(c, S1).wrapping_add(0x10)).lw(c);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x001b_fbb8, 0x0019_4350)?;
                        c.g[A0][0] = c.g[S1][0];
                    }
                    c.cycles += 4;
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x001a_8a40, 0x0019_435c)?;
                    c.g[V0][0] = 0;
                    tail = 1;
                }
                // Message 254: unconfirm, enable the items, play the frame.
                0x0019_4364 => {
                    c.cycles += 22;
                    c.g[A0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                    c.g[S0][0] = OC851_MENU_STRIDE as u64;
                    c.g[A1][0] = 0;
                    c.mult(0, 4, 4, 16);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6d0));
                    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                    Callee {
                        addr: 0x0019_2740,
                        f: ee::s02::f_00192740,
                    }
                    .call(c, 0x0019_4380)?;
                    c.g[A0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                    c.g[A1][0] = Addr(r32(c, S1).wrapping_add(0x40)).lw(c);
                    c.mult(0, 4, 4, 16);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6d0));
                    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                    Callee {
                        addr: 0x0019_2970,
                        f: ee::s02::f_00192970,
                    }
                    .call(c, 0x0019_4398)?;
                    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                    c.g[A1][0] = 0;
                    c.mult(0, 3, 2, 16);
                    c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, S1)));
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x6d0)).lw(c);
                    call_ptr(c, 0x0039_b960, 0x0019_43b0)?;
                    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                    c.g[A0][0] = 0x004a_0000;
                    if c.g[V0][0] != 0 {
                        c.cycles += 1;
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x17b0));
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = 0x004a_0000;
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1788));
                    }
                    c.cycles += 9;
                    call_ptr(c, 0x0031_7670, 0x0019_43d4)?;
                    c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x40)).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_c778, 0x0019_43e0)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = 0xffff;
                    let missing = c.g[A1][0] == c.g[V0][0];
                    c.g[A1][0] &= 0xffff;
                    if missing {
                        tail = 2;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x40)).lw(c);
                        c.g[A2][0] = 1;
                        call_ptr(c, 0x0039_c870, 0x0019_43fc)?;
                        c.g[V0][0] = 0;
                        tail = 1;
                    }
                }
                // Anything else (the default of the jump table): the network state.
                _ if target == 0x0019_4404 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S2][0];
                    call_ptr(c, 0x001a_8e40, 0x0019_4410)?;
                    frame.restore(c, 16, 0x60);
                    tail = 0;
                }
                _ => {
                    c.jump_out(target, 0x0019_41b0, 0x0019_443c);
                    return Err(crate::call::Unwound);
                }
            }
            if tail >= 2 {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            if tail >= 1 {
                c.cycles += 1;
                frame.restore(c, 16, 0x60);
            }
            c.cycles += 7;
            frame.restore(c, 17, 0x50);
            frame.restore(c, 18, 0x40);
            frame.restore(c, 19, 0x30);
            frame.restore(c, 20, 0x20);
            frame.restore_ra(c, 0x10);
            frame.close(c);
            return Ok(());
        }
    })
}

/// Native resume of `bragging_rights_v31` (converted from the `entries` body): any entry
/// other than 0x001941b0, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn bragging_rights_v31_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x001941b0 => {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xee));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[V0][0] = (c.g[V1][0] < 0x26u64) as u64;
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S2][0] = c.g[A2][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[V0][0] == 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    if t {
                        pc = 0x00194404;
                        continue;
                    }
                    pc = 0x001941e0;
                    continue;
                }
                0x001941e0 => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x460000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xc0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001941fc => {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = 0x34;
                    c.g[A1][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0xc));
                    c.mult(0, 4, 2, 3);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[RA][0] = sx(0x00194218);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6d0));
                    call_ptr(c, 0x0039b960, 0x00194218)?;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0019441c;
                        continue;
                    }
                    pc = 0x00194220;
                    continue;
                }
                0x00194220 => {
                    c.cycles += 14;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[S0][0] = 0x34;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    c.g[A2][0] = c.g[SP][0];
                    c.mult(0, 4, 3, 16);
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x6d8));
                    c.g[RA][0] = sx(0x00194250);
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x0039a768, 0x00194250)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.lbu(c.g[SP][0] as u32);
                    if t {
                        pc = 0x00194418;
                        continue;
                    }
                    pc = 0x00194258;
                    continue;
                }
                0x00194258 => {
                    c.cycles += 14;
                    c.g[RA][0] = sx(0x00194260);
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x0039a7a8, 0x00194260)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.g[S3][0];
                    c.mult(0, 4, 4, 16);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6d0));
                    c.g[RA][0] = sx(0x00194278);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    Callee {
                        addr: 0x00192918,
                        f: ee::s02::cFEStateBraggingRights_FindRowIndex,
                    }
                    .call(c, 0x00194278)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x738));
                    c.g[A0][0] = 0xffffffffffffffff;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S4][0] = c.lb(c.g[V1][0] as u32);
                    let t = c.g[S4][0] == c.g[A0][0];
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0019441c;
                        continue;
                    }
                    pc = 0x00194290;
                    continue;
                }
                0x00194290 => {
                    c.cycles += 10;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[RA][0] = sx(0x0019429c);
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x0039a738, 0x0019429c)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x001942b0);
                    c.g[A3][0] = c.g[V0][0];
                    Callee {
                        addr: 0x00193de0,
                        f: ee::s02::cFEStateBraggingRights_DispatchRowAction,
                    }
                    .call(c, 0x001942b0)?;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0019441c;
                        continue;
                    }
                    pc = 0x001942b8;
                    continue;
                }
                0x001942b8 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = 0x34;
                    c.g[A1][0] = 0x1;
                    c.mult(0, 4, 4, 2);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6d0));
                    c.g[RA][0] = sx(0x001942d4);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    Callee {
                        addr: 0x00192740,
                        f: ee::s02::cFEStateBraggingRights_SetRowsVisible,
                    }
                    .call(c, 0x001942d4)?;
                    c.g[RA][0] = sx(0x001942dc);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x00194080,
                        f: ee::s02::cFEStateBraggingRights_StartWaitAnim,
                    }
                    .call(c, 0x001942dc)?;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0019441c;
                        continue;
                    }
                    pc = 0x001942e4;
                    continue;
                }
                0x001942e4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001942ec);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x001ab288, 0x001942ec)?;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0019441c;
                        continue;
                    }
                    pc = 0x001942f4;
                    continue;
                }
                0x001942f4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001942fc);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    call_ptr(c, 0x0025da90, 0x001942fc)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x460000;
                    if t {
                        pc = 0x0019432c;
                        continue;
                    }
                    pc = 0x00194304;
                    continue;
                }
                0x00194304 => {
                    c.cycles += 10;
                    c.g[A0][0] = 0x73c;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf0));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x00194318);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x00194318)?;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x00194324);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x001b1f88, 0x00194324)?;
                    let t = true;
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        pc = 0x00194354;
                        continue;
                    }
                    pc = 0x0019432c;
                    continue;
                }
                0x0019432c => {
                    c.cycles += 10;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x6d0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd8));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x00194344);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x00194344)?;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x00194350);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x001bfbb8, 0x00194350)?;
                    c.g[A0][0] = c.g[S1][0];
                    pc = 0x00194354;
                    continue;
                }
                0x00194354 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0019435c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x001a8a40, 0x0019435c)?;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0019441c;
                        continue;
                    }
                    pc = 0x00194364;
                    continue;
                }
                0x00194364 => {
                    c.cycles += 22;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[S0][0] = 0x34;
                    c.g[A1][0] = 0u64;
                    c.mult(0, 4, 4, 16);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6d0));
                    c.g[RA][0] = sx(0x00194380);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    Callee {
                        addr: 0x00192740,
                        f: ee::s02::cFEStateBraggingRights_SetRowsVisible,
                    }
                    .call(c, 0x00194380)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.mult(0, 4, 4, 16);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6d0));
                    c.g[RA][0] = sx(0x00194398);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    Callee {
                        addr: 0x00192970,
                        f: ee::s02::cFEStateBraggingRights_HideUnusedRows,
                    }
                    .call(c, 0x00194398)?;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.mult(0, 3, 2, 16);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[RA][0] = sx(0x001943b0);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6d0));
                    call_ptr(c, 0x0039b960, 0x001943b0)?;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));

                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = 0x4a0000;
                        pc = 0x001943c8;
                        continue;
                    }
                    pc = 0x001943bc;
                    continue;
                }
                0x001943bc => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x4a0000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1788));
                    if t {
                        pc = 0x001943cc;
                        continue;
                    }
                    pc = 0x001943c8;
                    continue;
                }
                0x001943c8 => {
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x17b0));
                    pc = 0x001943cc;
                    continue;
                }
                0x001943cc => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x001943d4);
                    call_ptr(c, 0x00317670, 0x001943d4)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001943e0);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039c778, 0x001943e0)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = 0xffff;
                    let t = c.g[A1][0] == c.g[V0][0];
                    c.g[A1][0] = c.g[A1][0] & 0xffff;
                    if t {
                        pc = 0x00194418;
                        continue;
                    }
                    pc = 0x001943f0;
                    continue;
                }
                0x001943f0 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001943fc);
                    c.g[A2][0] = 0x1;
                    call_ptr(c, 0x0039c870, 0x001943fc)?;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0019441c;
                        continue;
                    }
                    pc = 0x00194404;
                    continue;
                }
                0x00194404 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00194410);
                    c.g[A2][0] = c.g[S2][0];
                    call_ptr(c, 0x001a8e40, 0x00194410)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x00194420;
                        continue;
                    }
                    pc = 0x00194418;
                    continue;
                }
                0x00194418 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x0019441c;
                    continue;
                }
                0x0019441c => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    pc = 0x00194420;
                    continue;
                }
                0x00194420 => {
                    c.cycles += 7;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x001941b0, 0x0019443c);
                    return Ok(());
                }
            }
        }
    });
}

/// `cFEStateBraggingRights_TransitionIn` (0x0019_3310): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_bragging_rights_transition_in_00193310(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
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

    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::put_words;
    use crate::difftest::Rng;
    use crate::difftest::SP as OC849_STACK;
    use crate::game::main::loadscreens::tests::oc736_fake_call;
    use crate::game::main::loadscreens::tests::oc736_fixture;
    use crate::game::main::loadscreens::tests::oc736_force;
    use crate::game::main::loadscreens::tests::oc736_force_echo;
    use crate::game::main::loadscreens::tests::oc736_force_zero;
    use crate::game::main::loadscreens::tests::FX_CTRL;
    use crate::game::main::loadscreens::tests::FX_MAGIC;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    pub(crate) const OC848_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC848_SELF: u32 = DATA + 0x1_0000;

    pub(crate) const OC848_SCREEN: u32 = DATA + 0x2_0000;

    pub(crate) const OC848_OBJ: u32 = DATA + 0x3_0000;

    pub(crate) const OC848_VT: u32 = DATA + 0x4_0000;

    pub(crate) const OC848_LISTS: u32 = DATA + 0x5_0000;

    pub(crate) const OC848_ZEROS: u32 = DATA + 0x6_0000;

    pub(crate) const OC848_ENGINE: u32 = DATA + 0x7_0000;

    /// The miss hook of the fixture: logs every call like the plain fake and
    /// gives `addNewScreen` a screen (or none) and `UIScreen_FindObject` a
    /// widget with a vtable (or none), so the body's pointer chasing stays in
    /// the fixture's data.
    pub(crate) fn oc848_hook(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        let salt = c.bus.read32(OC848_CTRL) as u64;
        fakes::fake_call(c, target);
        if c.pc != ra {
            return;
        }
        match target {
            0x0039_7cc0 => {
                c.g[V0][0] = if salt & 1 == 1 {
                    0
                } else {
                    OC848_SCREEN as u64
                };
            }
            0x0039_d860 => {
                c.g[V0][0] = if fakes::mix(salt.wrapping_add(n as u64)) >> 7 & 3 == 0 {
                    0
                } else {
                    (OC848_OBJ + (n % 8) * 0x40) as u64
                };
            }
            _ => {}
        }
    }

    pub(crate) const OC848_GATE: u32 = OC848_CTRL + 4;

    pub(crate) const OC848_MAGIC: u32 = 0x4f43_3834;

    macro_rules! oc848_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC848_GATE) != OC848_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc848_hook(c, $addr);
                }
            )*
            /// Gated fakes of the callees (and of the file-load request they
            /// would reach), active only when the fixture writes the magic word.
            pub(crate) fn oc848_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc848_fakes!(
        (
            oc848_fk_00397cc0,
            OC848_PREV_00397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x00397CC0
        ),
        (
            oc848_fk_0039c870,
            OC848_PREV_0039C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039C870
        ),
        (
            oc848_fk_0039d860,
            OC848_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039D860
        ),
        (
            oc848_fk_003a1000,
            OC848_PREV_003A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003A1000
        ),
        (
            oc848_fk_0039a568,
            OC848_PREV_0039A568,
            ee::s13::OVERRIDE_0039A568,
            ee::s13::f_0039a568__generated,
            0x0039A568
        ),
        (
            oc848_fk_0039a3f8,
            OC848_PREV_0039A3F8,
            ee::s13::OVERRIDE_0039A3F8,
            ee::s13::f_0039a3f8__generated,
            0x0039A3F8
        ),
        (
            oc848_fk_0039a768,
            OC848_PREV_0039A768,
            ee::s13::OVERRIDE_0039A768,
            ee::s13::f_0039a768__generated,
            0x0039A768
        ),
        (
            oc848_fk_0039a7a8,
            OC848_PREV_0039A7A8,
            ee::s13::OVERRIDE_0039A7A8,
            ee::s13::f_0039a7a8__generated,
            0x0039A7A8
        ),
        (
            oc848_fk_00317670,
            OC848_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc848_fk_0014dc80,
            OC848_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014DC80
        ),
        (
            oc848_fk_0014dd58,
            OC848_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014DD58
        ),
        (
            oc848_fk_003de670,
            OC848_PREV_003DE670,
            ee::s14::OVERRIDE_003DE670,
            ee::s14::f_003de670__generated,
            0x003DE670
        ),
    );

    /// `cFEStateBraggingRights_Init`: every game event (the six jump table
    /// cases, out-of-range events and a corrupt table entry), with and
    /// without a screen, the attribute bit that hides the menus, found and
    /// missing widgets, net play on and off, and the unwinds of its calls.
    #[test]
    fn cfe_state_bragging_rights_init_matches() {
        let table: [u32; 6] = [
            0x0019_2e60,
            0x0019_2e6c,
            0x0019_2e84,
            0x0019_2e78,
            0x0019_2e90,
            0x0019_2e9c,
        ];
        oc848_install_fakes();
        crate::difftest::check(
            ee::s02::f_00192d80__generated,
            cfe_state_bragging_rights_init,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0848_192d, 90);
                c.hooks.miss = oc848_hook;
                c.bus.write32(OC848_GATE, OC848_MAGIC);
                elf_image(c);
                c.pc = 0x0019_2d80;
                c.bus.write32(OC848_CTRL, r.next_u32());
                if seed % 5 == 1 {
                    let salt = c.bus.read32(OC848_CTRL) | 1;
                    c.bus.write32(OC848_CTRL, salt);
                } else {
                    let salt = c.bus.read32(OC848_CTRL) & !1;
                    c.bus.write32(OC848_CTRL, salt);
                }
                c.g[A0][0] = sx(OC848_SELF);
                c.bus.write32(OC848_SELF + 0x10, OC848_ENGINE);
                c.bus
                    .write32(OC848_SELF + 0x1c, if seed % 3 == 0 { 0x10 } else { 0 });
                // The two player menus: child list heads in zeroed memory.
                for menu in [0x6d0u32, 0x704] {
                    c.bus.write32(OC848_SELF + menu + 8, OC848_ZEROS);
                }
                // The list box pointers (`self + 0x6d8 + player * 0x34 + row * 4`).
                for i in 0..0x24u32 {
                    c.bus
                        .write32(OC848_SELF + 0x6d8 + 4 * i, OC848_LISTS + 0x10 * i);
                }
                // Vtables of the widgets the fake `FindObject` returns.
                for n in 0..8u32 {
                    c.bus.write32(OC848_OBJ + n * 0x40 + 8, OC848_VT);
                }
                for (off, i) in [(0x40u32, 0u32), (0x48, 1)] {
                    c.bus.write16(OC848_VT + off, 0);
                    c.bus.write32(OC848_VT + off + 4, fakes::FAKE + 0x10 * i);
                }
                put_words(c, 0x0045_fd20, &table);
                let event = seed % 8;
                c.bus.write8(0x0053_5c12, event as u8);
                if seed % 29 == 0 && event < 6 {
                    c.bus.write32(0x0045_fd20 + 4 * event, 0x0019_2e00);
                }
                c.bus.write32(GAME_GP.wrapping_sub(0x1680), (seed / 8) % 2);
            },
        );
    }

    /// The state object, the screen value, the items and their vtable, the
    /// back-end object, and the menu controls' objects (all clear of the fake
    /// log and of the fixture of `oc736_fixture`).
    pub(crate) const OC849_THIS: u32 = DATA + 0x70_0000;

    pub(crate) const OC849_SCREEN: u32 = DATA + 0x70_1000;

    pub(crate) const OC849_ITEM: u32 = DATA + 0x71_0000;

    pub(crate) const OC849_VT: u32 = DATA + 0x72_0000;

    pub(crate) const OC849_WIDGET: u32 = DATA + 0x72_1000;

    pub(crate) const OC849_ENG: u32 = DATA + 0x73_0000;

    pub(crate) const OC849_BE: u32 = DATA + 0x73_1000;

    pub(crate) const OC849_BEVT: u32 = DATA + 0x73_2000;

    pub(crate) const OC849_MENU: u32 = DATA + 0x74_0000;

    pub(crate) const OC849_MENUVT: u32 = DATA + 0x75_0000;

    /// `RiderIface_HasVisited`: a real leaf that returns 0, as a method target.
    pub(crate) const OC849_ZERO: u32 = 0x0014_06c0;

    /// Gated fakes (gate: `FX_CTRL == FX_MAGIC`, written by `oc736_fixture`) for
    /// every callee with an override slot; with the gate off they run the body
    /// they replaced.
    macro_rules! oc849_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(FX_CTRL) != FX_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc736_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc849_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc849_fakes!(
        (
            oc849_fake_317670,
            OC849_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc849_fake_39d860,
            OC849_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc849_fake_39b960,
            OC849_PREV_39B960,
            ee::s13::OVERRIDE_0039B960,
            ee::s13::f_0039b960__generated,
            0x0039_b960
        ),
        (
            oc849_fake_39c778,
            OC849_PREV_39C778,
            ee::s13::OVERRIDE_0039C778,
            ee::s13::f_0039c778__generated,
            0x0039_c778
        ),
        (
            oc849_fake_39c870,
            OC849_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc849_fake_3a0c50,
            OC849_PREV_3A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50
        ),
        (
            oc849_fake_39e6b8,
            OC849_PREV_39E6B8,
            ee::s13::OVERRIDE_0039E6B8,
            ee::s13::f_0039e6b8__generated,
            0x0039_e6b8
        ),
        (
            oc849_fake_14dc80,
            OC849_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc849_fake_14dd58,
            OC849_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc849_fake_147318,
            OC849_PREV_147318,
            ee::s01::OVERRIDE_00147318,
            ee::s01::f_00147318__generated,
            0x0014_7318
        ),
        (
            oc849_fake_146e98,
            OC849_PREV_146E98,
            ee::s01::OVERRIDE_00146E98,
            ee::s01::f_00146e98__generated,
            0x0014_6e98
        ),
        (
            oc849_fake_14ee58,
            OC849_PREV_14EE58,
            ee::s01::OVERRIDE_0014EE58,
            ee::s01::f_0014ee58__generated,
            0x0014_ee58
        ),
        (
            oc849_fake_416210,
            OC849_PREV_416210,
            ee::s15::OVERRIDE_00416210,
            ee::s15::f_00416210__generated,
            0x0041_6210
        ),
        (
            oc849_fake_265700,
            OC849_PREV_265700,
            ee::s07::OVERRIDE_00265700,
            ee::s07::f_00265700__generated,
            0x0026_5700
        ),
        (
            oc849_fake_192740,
            OC849_PREV_192740,
            ee::s02::OVERRIDE_00192740,
            ee::s02::f_00192740__generated,
            0x0019_2740
        ),
    );

    /// Zero `bytes` bytes at `base`.
    pub(crate) fn oc849_clear(c: &mut Cpu, base: u32, bytes: u32) {
        for i in 0..bytes / 4 {
            c.bus.write32(base + 4 * i, 0);
        }
    }

    /// A vtable method slot `{delta, target}`.
    pub(crate) fn oc849_slot(c: &mut Cpu, vt: u32, off: u32, delta: u16, target: u32) {
        c.bus.write16(vt + off, delta);
        c.bus.write32(vt + off + 4, target);
    }

    /// The common machine: the game image (string tables), the gated fakes and
    /// the shared fixture.
    pub(crate) fn oc849_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc849_install_fakes();
        elf_image(c);
        oc736_fixture(c, seed, salt, calls)
    }

    /// The item / widget vtable: `setStatic` at 0x40, `setVisible` at 0x48,
    /// `setFocus` at 0x38, flag setter at 0xc0 (all fake methods).
    pub(crate) fn oc849_item_vtable(c: &mut Cpu, seed: u32) {
        let delta = 0x10 * (seed % 3) as u16;
        for (i, off) in [0x38u32, 0x40, 0x48, 0xc0].into_iter().enumerate() {
            oc849_slot(c, OC849_VT, off, delta, fakes::FAKE + 0x10 * i as u32);
        }
    }

    /// One call to `UIState_Render`, which also unwinds at chosen seeds.
    #[test]
    fn render_matches() {
        check(
            ee::s02::f_00193318__generated,
            c_fe_state_bragging_rights_render,
            60,
            &|c, seed| {
                oc849_base(c, seed, 0x19_3318, 1);
                c.g[A0][0] = sx(OC849_THIS);
                c.pc = 0x0019_3318;
            },
        );
    }

    /// Online (one panel) and offline (two), confirm / cancel / neither
    /// presses, the panel flag set or clear, the screen frame found or not.
    #[test]
    fn update_matches() {
        check_lohi(
            ee::s02::f_00193338__generated,
            c_fe_state_bragging_rights_update,
            240,
            &|c, seed| {
                let mut r = oc849_base(c, seed, 0x19_3338, 14);
                oc849_clear(c, OC849_THIS, 0x800);
                for base in [
                    OC849_ENG,
                    OC849_BE,
                    OC849_BEVT,
                    OC849_MENU,
                    OC849_MENU + 0x100,
                ] {
                    oc849_clear(c, base, 0x200);
                }
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x1680), (seed % 2) as u32);
                c.bus.write32(GAME_GP + 0x2dc, r.next_u32());
                c.bus.write32(OC849_THIS + 0x10, OC849_ENG);
                c.bus.write32(OC849_THIS + 0x40, OC849_SCREEN);
                c.bus.write32(OC849_ENG + 0xc, OC849_BE);
                c.bus.write32(OC849_BE + 8, OC849_BEVT);
                let delta = 0x10 * (seed % 3) as u16;
                oc849_slot(c, OC849_BEVT, 0x160, delta, fakes::FAKE + 0x10);
                let confirm = if (seed / 2) % 3 == 0 {
                    OC849_ZERO
                } else {
                    fakes::FAKE + 0x20
                };
                oc849_slot(c, OC849_BEVT, 0x140, delta, confirm);
                let cancel = if (seed / 6) % 3 == 0 {
                    OC849_ZERO
                } else {
                    fakes::FAKE + 0x30
                };
                oc849_slot(c, OC849_BEVT, 0x30, delta, cancel);
                oc849_slot(c, OC849_MENUVT, 0x78, delta, fakes::FAKE + 0x40);
                oc849_item_vtable(c, seed);
                c.bus.write32(OC849_WIDGET + 8, OC849_VT);
                for i in 0..2u32 {
                    let ctl = OC849_THIS + 0x6d0 + 0x34 * i;
                    let menu = OC849_MENU + 0x100 * i;
                    c.bus.write32(ctl, menu);
                    c.bus.write32(menu + 8, OC849_MENUVT);
                    c.bus.write32(ctl + 0x28, (seed >> (3 + i)) & 1);
                    c.bus.write32(ctl + 0x2c, r.next_u32() & 0xff);
                    for j in 0..8u32 {
                        if (seed >> (j % 5)) & 1 == 1 {
                            let item = OC849_ITEM + 0x100 * (8 * i + j);
                            c.bus.write32(ctl + 8 + 4 * j, item);
                            c.bus.write32(item + 8, OC849_VT);
                        }
                    }
                }
                oc736_force_echo(c, 0x0031_7670);
                if seed % 5 == 0 {
                    oc736_force(c, 0x0039_c778, 0xffff);
                }
                if seed % 3 == 0 {
                    oc736_force_zero(c, 0x0039_d860);
                } else {
                    oc736_force(c, 0x0039_d860, OC849_WIDGET);
                }
                c.g[A0][0] = sx(OC849_THIS);
                c.pc = 0x0019_3338;
            },
        );
    }

    /// Every widget kind the screen creates: the two menus, the four player
    /// name texts, the two info groups, the row widgets (name hash equals the
    /// stack string) and an unknown widget.
    #[test]
    fn on_widget_created_matches() {
        check_lohi(
            ee::s02::f_00193650__generated,
            c_fe_state_bragging_rights_on_widget_created,
            240,
            &|c, seed| {
                let mut r = oc849_base(c, seed, 0x19_3650, 14);
                oc849_clear(c, OC849_THIS, 0x800);
                oc849_clear(c, OC849_WIDGET, 0x100);
                oc849_item_vtable(c, seed);
                c.bus.write32(OC849_WIDGET + 8, OC849_VT);
                let names = [
                    0x004a_17a0u32,
                    0x004a_17a8,
                    0x0045_fd38,
                    0x0045_fd48,
                    0x0045_fd58,
                    0x0045_fd68,
                    0x0045_fd78,
                    0x0045_fd88,
                    OC849_STACK - 0xb0,
                    OC849_STACK - 0xb0,
                    0,
                    0x1234,
                ];
                c.bus
                    .write32(OC849_WIDGET + 0x38, names[(seed % 12) as usize]);
                c.bus.write32(OC849_WIDGET + 0x14, r.next_u32());
                c.bus.write32(OC849_WIDGET + 0x90, r.next_u32());
                oc736_force_echo(c, 0x0031_7670);
                c.g[A0][0] = sx(OC849_THIS);
                c.g[A1][0] = sx(OC849_WIDGET);
                c.pc = 0x0019_3650;
            },
        );
    }

    pub(crate) const OC850_CTRL: u32 = DATA + 0x28_0000;

    pub(crate) const OC850_MAGIC: u32 = 0x4f43_3835;

    pub(crate) const OC850_SELF: u32 = DATA + 0x20_0000;

    pub(crate) const OC850_ENGINE: u32 = DATA + 0x20_1000;

    pub(crate) const OC850_POPUP: u32 = DATA + 0x20_2000;

    pub(crate) const OC850_MAP: u32 = DATA + 0x20_3000;

    pub(crate) const OC850_OBJ: u32 = DATA + 0x20_4000;

    pub(crate) const OC850_VT: u32 = DATA + 0x20_5000;

    pub(crate) const OC850_SETUP: u32 = DATA + 0x20_6000;

    pub(crate) const OC850_WIDGET: u32 = DATA + 0x21_0000;

    pub(crate) const OC850_CTRLPTR: u32 = DATA + 0x21_1000;

    /// A real leaf returning 0 (a virtual method that answers "no state").
    /// The online flag word (`gp-0x1680`).
    pub(crate) const OC850_ONLINE_FLAG: u32 = crate::difftest::GAME_GP - 0x1680;

    pub(crate) const OC850_ZERO_LEAF: u32 = 0x0014_06c0;

    macro_rules! oc850_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC850_CTRL) != OC850_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc850_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc850_fakes!(
        (
            oc850_fake_192740,
            OC850_PREV_192740,
            ee::s02::OVERRIDE_00192740,
            ee::s02::f_00192740__generated,
            0x0019_2740
        ),
        (
            oc850_fake_194080,
            OC850_PREV_194080,
            ee::s02::OVERRIDE_00194080,
            ee::s02::f_00194080__generated,
            0x0019_4080
        ),
        (
            oc850_fake_265458,
            OC850_PREV_265458,
            ee::s07::OVERRIDE_00265458,
            ee::s07::f_00265458__generated,
            0x0026_5458
        ),
        (
            oc850_fake_265568,
            OC850_PREV_265568,
            ee::s07::OVERRIDE_00265568,
            ee::s07::f_00265568__generated,
            0x0026_5568
        ),
        (
            oc850_fake_265688,
            OC850_PREV_265688,
            ee::s07::OVERRIDE_00265688,
            ee::s07::f_00265688__generated,
            0x0026_5688
        ),
        (
            oc850_fake_39f400,
            OC850_PREV_39F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400
        ),
        (
            oc850_fake_39a738,
            OC850_PREV_39A738,
            ee::s13::OVERRIDE_0039A738,
            ee::s13::f_0039a738__generated,
            0x0039_a738
        ),
        (
            oc850_fake_1a8918,
            OC850_PREV_1A8918,
            ee::s03::OVERRIDE_001A8918,
            ee::s03::f_001a8918__generated,
            0x001a_8918
        ),
        (
            oc850_fake_1a9bc0,
            OC850_PREV_1A9BC0,
            ee::s03::OVERRIDE_001A9BC0,
            ee::s03::f_001a9bc0__generated,
            0x001a_9bc0
        ),
        (
            oc850_fake_1c57c0,
            OC850_PREV_1C57C0,
            ee::s03::OVERRIDE_001C57C0,
            ee::s03::f_001c57c0__generated,
            0x001c_57c0
        ),
        (
            oc850_fake_3e6448,
            OC850_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
    );

    /// A bragging-rights screen object with its two menus (`+0x6d0`, `+0x704`,
    /// 0x34 bytes each: control pointer, 8 row widgets from `+8`, confirmed flag
    /// at `+0x28`), the engine/vtable chain behind `self+0x10` and the online
    /// flag at `gp-0x1680`. `rows` puts the widget into random row slots.
    pub(crate) fn oc850_screen(c: &mut Cpu, r: &mut Rng, rows: bool) {
        oc850_tables(c);
        c.bus.write32(OC850_CTRL, OC850_MAGIC);
        c.bus.write32(OC850_SELF + 0x10, OC850_ENGINE);
        c.bus.write32(OC850_SELF + 0x6a0, OC850_POPUP);
        c.bus.write32(OC850_SELF + 0x738, OC850_MAP);
        c.bus.write32(OC850_ENGINE, OC850_OBJ);
        c.bus.write32(OC850_OBJ + 4, OC850_VT);
        for off in [0x20u32, 0x28] {
            c.bus
                .write16(OC850_VT + off, (r.next_u32() as i16 as i32 & !3) as u16);
        }
        for off in [0x24u32, 0x2c] {
            let target = if r.next_u32() % 3 == 0 {
                OC850_ZERO_LEAF
            } else {
                FAKE + off
            };
            c.bus.write32(OC850_VT + off, target);
        }
        for i in 0..8 {
            let v = match r.next_u32() % 4 {
                0 => 0xffff_ffffu32,
                _ => r.next_u32() % 8,
            };
            c.bus.write8(OC850_MAP + i, v as u8);
        }
        c.bus.write32(
            OC850_ONLINE_FLAG,
            [0u32, 1, 1, 2][(r.next_u32() % 4) as usize],
        );
        c.bus.write32(crate::difftest::GAME_GP + 0x2dc, OC850_SETUP);
        for menu in 0..2u32 {
            let base = OC850_SELF + 0x6d0 + 0x34 * menu;
            c.bus.write32(base, OC850_CTRLPTR + 0x100 * menu);
            c.bus.write32(base + 0x28, r.next_u32() % 2);
            if rows {
                for k in 0..8u32 {
                    if r.next_u32() % 3 == 0 {
                        c.bus.write32(base + 8 + 4 * k, OC850_WIDGET);
                    }
                }
            }
        }
        c.bus.write32(OC850_WIDGET + 0x18, r.next_u32() % 2);
        c.bus.write32(OC850_WIDGET + 0x8, OC850_VT);
    }

    /// The jump tables of the five functions (game data the test RAM lacks).
    pub(crate) fn oc850_tables(c: &mut Cpu) {
        let tables: [(u32, &[u32]); 4] = [
            (
                0x0045_fdc0,
                &[
                    0x0019_3b08,
                    0x0019_3b08,
                    0x0019_3bdc,
                    0x0019_3bdc,
                    0x0019_3948,
                    0x0019_3a1c,
                    0x0019_3b28,
                    0x0019_3bdc,
                    0x0019_3b4c,
                ],
            ),
            (
                0x0045_feb0,
                &[
                    0x0019_3d64,
                    0x0019_3d98,
                    0x0019_3db8,
                    0x0019_3d5c,
                    0x0019_3d5c,
                ],
            ),
            (
                0x0045_fed0,
                &[
                    0x0019_3e08,
                    0x0019_3e28,
                    0x0019_3e60,
                    0x0019_3e94,
                    0x0019_3eb4,
                    0x0019_3ed4,
                    0x0019_3ef4,
                    0x0019_3f14,
                ],
            ),
            (
                0x0045_fef0,
                &[
                    0x0019_3f5c,
                    0x0019_3f7c,
                    0x0019_3fa4,
                    0x0019_3fc8,
                    0x0019_3fec,
                    0x0019_4010,
                    0x0019_4034,
                    0x0019_4058,
                ],
            ),
        ];
        for (base, words) in tables {
            crate::difftest::put_words(c, base, words);
        }
    }

    /// Events 0..=10 (1..=9 take the jump table), online or not, rows with and
    /// without the widget, virtual methods answering a state or nothing.
    #[test]
    fn bragging_rights_on_widget_action_matches() {
        oc850_install_fakes();
        check(
            ee::s02::f_001938f8__generated,
            c_fe_state_bragging_rights_on_widget_action,
            11 * 24,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1938f8, 6);
                c.pc = 0x0019_38f8;
                let event = seed % 11;
                oc850_screen(c, &mut r, matches!(event, 5 | 9) && seed % 5 != 0);
                c.g[A0][0] = OC850_SELF as u64;
                c.g[A1][0] = OC850_WIDGET as u64;
                c.g[A2][0] = event as u64;
            },
        );
    }

    /// The help popup is filled through real setters and the string hashes.
    #[test]
    fn bragging_rights_v25_matches() {
        check(
            ee::s02::f_00193c00__generated,
            c_fe_state_bragging_rights_v25,
            8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x193c00, 0);
                c.pc = 0x0019_3c00;
                c.g[A0][0] = OC850_SELF as u64;
                c.g[A1][0] = (DATA + 0x30_0000 + (r.next_u32() & 0xf_fff0)) as u64;
            },
        );
    }

    /// Messages 3..=11 on the first menu, the second menu and neither, online or
    /// not, with the "confirmed" flags both ways.
    #[test]
    fn bragging_rights_on_widget_notify_matches() {
        oc850_install_fakes();
        check(
            ee::s02::f_00193cf8__generated,
            c_fe_state_bragging_rights_on_widget_notify,
            9 * 3 * 12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x193cf8, 4);
                c.pc = 0x0019_3cf8;
                oc850_screen(c, &mut r, false);
                let msg = 3 + seed % 9;
                let which = (seed / 9) % 3;
                let ctrl = OC850_CTRLPTR + 0x40;
                c.bus.write32(
                    OC850_SELF + 0x6d0,
                    if which == 0 { ctrl } else { 0x1111_0000 },
                );
                c.bus.write32(
                    OC850_SELF + 0x704,
                    if which == 1 { ctrl } else { 0x2222_0000 },
                );
                c.bus.write32(OC850_SELF + 0x72c, r.next_u32() % 2);
                c.g[A0][0] = OC850_SELF as u64;
                c.g[A1][0] = ctrl as u64;
                c.g[A2][0] = msg as u64;
            },
        );
    }

    /// Every kind 0..=9 (8 and 9 do nothing) for player 0, 1 and others.
    #[test]
    fn bragging_rights_dispatch_row_action_matches() {
        check_lohi(
            ee::s02::f_00193de0__generated,
            c_fe_state_bragging_rights_dispatch_row_action,
            10 * 4 * 6,
            &|c, seed| {
                let mut r = Rng(0x193d_e000 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.pc = 0x0019_3de0;
                oc850_tables(c);
                crate::difftest::fill(c, &mut r, 0x0053_08b8, 6);
                c.g[A1][0] = [0u64, 1, 2, r.next_u32() as u64][(seed / 10 % 4) as usize];
                c.g[A2][0] = (seed % 10) as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    /// Every kind 0..=9 (8 and 9 answer -1) for player 0, 1 and others.
    #[test]
    fn bragging_rights_get_row_default_matches() {
        check_lohi(
            ee::s02::f_00193f38__generated,
            c_fe_state_bragging_rights_get_row_default,
            10 * 4 * 6,
            &|c, seed| {
                let mut r = Rng(0x193f_3800 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.pc = 0x0019_3f38;
                oc850_tables(c);
                crate::difftest::fill(c, &mut r, 0x0053_08b8, 6);
                c.g[A1][0] = [0u64, 1, 2, r.next_u32() as u64][(seed / 10 % 4) as usize];
                c.g[A2][0] = (seed % 10) as u64;
            },
        );
    }

    /// Gate of the oc851 fakes (a magic word in the machine's RAM) and the
    /// mailbox of forced results, one word per callee at `OC851_MAIL + (addr & 0xffc)`.
    pub(crate) const OC851_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC851_MAIL: u32 = OC851_CTRL + 0x100;

    pub(crate) const OC851_MAGIC: u32 = 0x4f43_3835;

    /// Fixture objects (clear of the fakes' result pointers at `DATA..DATA+0x40000`).
    pub(crate) const OC851_SELF: u32 = DATA + 0x8_1000;

    pub(crate) const OC851_VT_SELF: u32 = DATA + 0x8_2000;

    pub(crate) const OC851_OBJ2: u32 = DATA + 0x8_3000;

    pub(crate) const OC851_MAP: u32 = DATA + 0x8_4000;

    pub(crate) const OC851_LISTBOX: u32 = DATA + 0x8_5000;

    pub(crate) const OC851_IFACE: u32 = DATA + 0x8_6000;

    /// A forced result for a callee: 0 default pointer, 1 return 0, 2 return
    /// 0xffff, 3 return 1.
    pub(crate) fn oc851_mail(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(OC851_MAIL + (addr & 0xffc), v);
    }

    /// The fake callee: logs like `fakes::fake_call`, then applies the mailbox.
    pub(crate) fn oc851_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(OC851_MAIL + (addr & 0xffc));
        fakes::fake_call(c, addr);
        if c.pc == ra {
            match mail {
                1 => c.g[V0][0] = 0,
                2 => c.g[V0][0] = 0xffff,
                3 => c.g[V0][0] = 1,
                _ => {}
            }
        }
    }

    macro_rules! oc851_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC851_CTRL) != OC851_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc851_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc851_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc851_fakes!(
        (
            oc851_fake_194440,
            OC851_PREV_194440,
            ee::s02::OVERRIDE_00194440,
            ee::s02::f_00194440__generated,
            0x0019_4440
        ),
        (
            oc851_fake_192740,
            OC851_PREV_192740,
            ee::s02::OVERRIDE_00192740,
            ee::s02::f_00192740__generated,
            0x0019_2740
        ),
        (
            oc851_fake_194080,
            OC851_PREV_194080,
            ee::s02::OVERRIDE_00194080,
            ee::s02::f_00194080__generated,
            0x0019_4080
        ),
        (
            oc851_fake_39b960,
            OC851_PREV_39B960,
            ee::s13::OVERRIDE_0039B960,
            ee::s13::f_0039b960__generated,
            0x0039_b960
        ),
        (
            oc851_fake_39a768,
            OC851_PREV_39A768,
            ee::s13::OVERRIDE_0039A768,
            ee::s13::f_0039a768__generated,
            0x0039_a768
        ),
        (
            oc851_fake_39a7a8,
            OC851_PREV_39A7A8,
            ee::s13::OVERRIDE_0039A7A8,
            ee::s13::f_0039a7a8__generated,
            0x0039_a7a8
        ),
        (
            oc851_fake_39a738,
            OC851_PREV_39A738,
            ee::s13::OVERRIDE_0039A738,
            ee::s13::f_0039a738__generated,
            0x0039_a738
        ),
        (
            oc851_fake_39c778,
            OC851_PREV_39C778,
            ee::s13::OVERRIDE_0039C778,
            ee::s13::f_0039c778__generated,
            0x0039_c778
        ),
        (
            oc851_fake_39c870,
            OC851_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc851_fake_39d860,
            OC851_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc851_fake_39e4c0,
            OC851_PREV_39E4C0,
            ee::s13::OVERRIDE_0039E4C0,
            ee::s13::f_0039e4c0__generated,
            0x0039_e4c0
        ),
        (
            oc851_fake_39f400,
            OC851_PREV_39F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400
        ),
        (
            oc851_fake_25da90,
            OC851_PREV_25DA90,
            ee::s06::OVERRIDE_0025DA90,
            ee::s06::f_0025da90__generated,
            0x0025_da90
        ),
        (
            oc851_fake_317670,
            OC851_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc851_fake_317d70,
            OC851_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc851_fake_1d22f0,
            OC851_PREV_1D22F0,
            ee::s04::OVERRIDE_001D22F0,
            ee::s04::f_001d22f0__generated,
            0x001d_22f0
        ),
        (
            oc851_fake_1d2598,
            OC851_PREV_1D2598,
            ee::s04::OVERRIDE_001D2598,
            ee::s04::f_001d2598__generated,
            0x001d_2598
        ),
        (
            oc851_fake_1bfbb8,
            OC851_PREV_1BFBB8,
            ee::s03::OVERRIDE_001BFBB8,
            ee::s03::f_001bfbb8__generated,
            0x001b_fbb8
        ),
    );

    /// The machine with the fakes switched on and random mailbox answers for
    /// the callees whose results the ports branch on.
    pub(crate) fn oc851_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc851_install_fakes();
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(OC851_CTRL, OC851_MAGIC);
        for addr in [
            0x0039_a768u32,
            0x0025_da90,
            0x0039_c778,
            0x0039_d860,
            0x0031_7670,
        ] {
            let v = r.next_u32() % 4;
            oc851_mail(c, addr, v);
        }
        r
    }

    /// A vtable at `vt` whose slot pairs `off`/`off+4` hold `{delta, fake i}`.
    pub(crate) fn oc851_vtable(c: &mut Cpu, r: &mut Rng, vt: u32, slots: &[u32]) {
        for (i, &off) in slots.iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + off, delta as u16);
            c.bus.write32(vt + off + 4, FAKE + 0x10 * i as u32);
        }
    }

    /// `cFEStateBraggingRights_v24`: flag off returns 0, flag on forwards to
    /// the net state's handler (real; out-of-range message ends in a vtable call).
    #[test]
    fn bragging_rights_v24_matches() {
        fakes::covered(
            ee::s02::f_00194138__generated,
            bragging_rights_v24,
            120,
            &|c, seed| {
                let mut r = oc851_machine(c, seed, 0x194138, 1);
                oc851_vtable(c, &mut r, OC851_VT_SELF, &[0xf8]);
                c.bus.write32(OC851_SELF + 8, OC851_VT_SELF);
                c.bus.write32(
                    GAME_GP - OC851_ONLINE_FLAG,
                    [1, 1, 0, 5, 1][seed as usize % 5],
                );
                c.g[A0][0] = sx(OC851_SELF);
                c.g[A1][0] = [0u64, 0x10, 0x1000][seed as usize % 3];
                c.pc = 0x0019_4138;
            },
        );
    }

    /// `cFEStateBraggingRights_OnChildEvent`: flag off does nothing, popup
    /// 0x838 goes to OnChildEventQuit (fake), any other to the net state's popup handler.
    #[test]
    fn bragging_rights_on_child_event_matches() {
        check(
            ee::s02::f_00194168__generated,
            bragging_rights_on_child_event,
            120,
            &|c, seed| {
                let _ = oc851_machine(c, seed, 0x194168, 2);
                c.bus
                    .write32(GAME_GP - OC851_ONLINE_FLAG, [1, 1, 0, 1][seed as usize % 4]);
                c.bus.write32(OC851_SELF + 0x69c, 0);
                c.bus
                    .write32(OC851_OBJ2 + 0x18, [0x838, 0x500, 0x838][seed as usize % 3]);
                c.g[A0][0] = sx(OC851_SELF);
                c.g[A1][0] = sx(OC851_OBJ2);
                c.g[A2][0] = (seed % 17) as u64;
                c.pc = 0x0019_4168;
            },
        );
    }

    /// The message-switch jump table at 0x45ff40 (index = message - 0xee).
    pub(crate) fn oc851_v31_table(c: &mut Cpu, msg: u32) {
        let mut t = [0x0019_4404u32; 0x26];
        for i in [0usize, 3, 10, 12, 36, 37] {
            t[i] = 0x0019_42e4;
        }
        t[0xfb - 0xee] = 0x0019_41fc;
        t[0xfc - 0xee] = 0x0019_4220;
        t[0xfd - 0xee] = 0x0019_42b8;
        t[0xfe - 0xee] = 0x0019_4364;
        t[0x10c - 0xee] = 0x0019_42f4;
        // A table entry outside the function: the `jump_out` exit.
        t[0x105 - 0xee] = 0x0040_0000;
        let _ = msg;
        for (i, v) in t.iter().enumerate() {
            c.bus.write32(0x0045_ff40 + 4 * i as u32, *v);
        }
    }

    /// `cFEStateBraggingRights_v31`: every message of the switch, the default
    /// (net state handler), out-of-range messages and a table entry outside
    /// the function. Not covered: a table entry equal to the function start
    /// (a restart that never ends in the test machine).
    #[test]
    fn bragging_rights_v31_matches() {
        const MSGS: [u32; 20] = [
            251, 252, 252, 252, 253, 254, 254, 268, 268, 238, 241, 248, 250, 274, 275, 0x100, 0x50,
            0x2000, 0x105, 252,
        ];
        check_lohi(
            ee::s02::f_001941b0__generated,
            bragging_rights_v31,
            400,
            &|c, seed| {
                let mut r = oc851_machine(c, seed, 0x1941b0, 3);
                let msg = MSGS[seed as usize % MSGS.len()];
                oc851_v31_table(c, msg);
                c.bus.write32(GAME_GP - 0xc8, DATA + 0x1_0000);
                c.bus.write32(OC851_SELF + 0x10, OC851_IFACE);
                c.bus.write32(OC851_SELF + 0x40, DATA + 0x2_0000);
                c.bus.write32(OC851_SELF + 0x738, OC851_MAP);
                for p in 0..2u32 {
                    let menu = OC851_SELF + 0x6d0 + p * OC851_MENU_STRIDE;
                    c.bus.write32(menu, DATA + 0x3_0000 + p * 0x100);
                    for k in 0..8u32 {
                        c.bus
                            .write32(menu + 8 + 4 * k, OC851_LISTBOX + (p * 8 + k) * 0x100);
                    }
                }
                for k in 0..16u32 {
                    let b = if r.next_u32() % 3 == 0 {
                        0xff
                    } else {
                        (k % 8) as u8
                    };
                    c.bus.write8(OC851_MAP + k, b);
                }
                let panel = r.next_u32() % 2;
                c.bus.write32(OC851_OBJ2 + 8, panel);
                c.bus.write32(OC851_OBJ2 + 0xc, r.next_u32() % 9);
                c.bus.write32(OC851_OBJ2 + 0x10, r.next_u32());
                c.g[A0][0] = sx(OC851_SELF);
                c.g[A1][0] = sx(msg);
                c.g[A2][0] = sx(OC851_OBJ2);
                c.pc = 0x0019_41b0;
            },
        );
    }

    /// `cFEStateBraggingRights_TransitionIn` (0x0019_3310) against the generated body.
    #[test]
    fn c_fe_state_bragging_rights_transition_in_00193310_matches() {
        check(
            ee::s02::f_00193310__generated,
            c_fe_state_bragging_rights_transition_in_00193310,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x193310, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
