//! `src/fe/ovstatelodge.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::feasyncfile::gp_13 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::lw_5;
pub(crate) use crate::game::fe::fepopup::at as at_4;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_7;
pub(crate) use crate::game::fe::fepopupmisc::gp_2 as gp;
pub(crate) use crate::game::fe::fepopupmisc::lw_2;
pub(crate) use crate::game::fe::fepopupmisc::p_2 as p;
pub(crate) use crate::game::fe::fepopupmisc::sw;
pub(crate) use crate::game::fe::festatenethelppopup::fe_state_screen_init;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_4;
pub(crate) use crate::game::fe::festatetrophyroom::handle_list_key;
pub(crate) use crate::game::fe::festatetrophyroom::oc943_vcall;
pub(crate) use crate::game::fe::festatetrophyroom::oc945_at;
pub(crate) use crate::game::fe::festatetrophyroom::oc945_lb;
pub(crate) use crate::game::fe::festatetrophyroom::oc945_lh;
pub(crate) use crate::game::fe::festatetrophyroom::oc945_lw;
pub(crate) use crate::game::fe::festatetrophyroom::oc945_vcall;
pub(crate) use crate::game::fe::ovstatepause::lw;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001d_28f8,
        cfe_state_mountain_room_dtor,
        "cFEStateMountainRoom_dtor",
    ),
    (
        0x001d_2a90,
        fe_state_mountain_room_on_activate,
        "cFEStateMountainRoom_OnActivate",
    ),
    (
        0x001d_2ea0,
        cfe_state_mountain_room_handle_input,
        "cFEStateMountainRoom_HandleInput",
    ),
    (
        0x001d_2f40,
        cfe_state_mountain_room_on_widget_notify,
        "cFEStateMountainRoom_OnWidgetNotify",
    ),
    (
        0x001d_2f68,
        c_fe_state_mountain_room_on_widget_action,
        "cFEStateMountainRoom_OnWidgetAction",
    ),
    (
        0x001d_3120,
        c_fe_state_mountain_room_update,
        "cFEStateMountainRoom_Update",
    ),
    (
        0x001d_3160,
        cfe_state_mountain_room_request_image,
        "cFEStateMountainRoom_RequestImage",
    ),
    (
        0x001d_32e0,
        c_fe_state_mountain_room_show_image,
        "cFEStateMountainRoom_ShowImage",
    ),
    (
        0x001d_3890,
        c_fe_state_peak_room_init,
        "cFEStatePeakRoom_Init",
    ),
    (
        0x001d_3e08,
        fe_state_peak_room_on_widget_notify,
        "cFEStatePeakRoom_OnWidgetNotify",
    ),
    (
        0x001d_2678,
        c_fe_state_mountain_room_ctor,
        "cFEStateMountainRoom_ctor",
    ),
    (
        0x001d_2990,
        cfe_state_mountain_room_init,
        "cFEStateMountainRoom_Init",
    ),
    (
        0x001d_2ad0,
        c_fe_state_mountain_room_on_widget_created,
        "cFEStateMountainRoom_OnWidgetCreated",
    ),
    (
        0x001d_31c0,
        cfe_state_mountain_room_poll_image_load,
        "cFEStateMountainRoom_PollImageLoad",
    ),
    (
        0x001d_3340,
        c_fe_state_mountain_room_update_event_icons,
        "cFEStateMountainRoom_UpdateEventIcons",
    ),
    (
        0x001d_3780,
        cfe_state_peak_room_ctor,
        "cFEStatePeakRoom_ctor",
    ),
    (
        0x001d_38f8,
        c_fe_state_peak_room_on_widget_created,
        "cFEStatePeakRoom_OnWidgetCreated",
    ),
    (
        0x001d_3c60,
        cfe_state_peak_room_on_activate,
        "cFEStatePeakRoom_OnActivate",
    ),
    (
        0x001d_3c80,
        cfe_state_peak_room_on_widget_action,
        "cFEStatePeakRoom_OnWidgetAction",
    ),
    (
        0x001d_3ec8,
        cfe_state_peak_room_update_help_text,
        "cFEStatePeakRoom_UpdateHelpText",
    ),
    (
        0x001d_3f80,
        c_fe_state_peak_room_refresh_goal_list,
        "cFEStatePeakRoom_RefreshGoalList",
    ),
    (
        0x001d_4268,
        cfe_state_trophy_room_ctor,
        "cFEStateTrophyRoom_ctor",
    ),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {

    pub(crate) use crate::game::fe::fepopup::callee::C001D32E0;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::FE_STATE_ON_ACTIVATE;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::MOUNTAIN_ROOM_UPDATE_EVENT_ICONS;

    pub(crate) use crate::game::fe::ovstatepause::callee::MOUNTAIN_ROOM_POLL_IMAGE_LOAD;

    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_UPDATE_2;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {

    pub(crate) use crate::game::fe::festaterewards::table_callee::MUSIC_SLOT_RELEASE;

    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_STATE_DTOR;
}

/// `void cFEStateMountainRoom_OnActivate(self, arg)` (0x1D2A90): rebuild the
/// event icons, then run the shared state activation.
pub fn fe_state_mountain_room_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        callee::MOUNTAIN_ROOM_UPDATE_EVENT_ICONS.call(c, 0x001d_2aac)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::FE_STATE_ON_ACTIVATE.call(c, 0x001d_2ab8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMountainRoom_Update(State* s)` (0x1d3120): poll the room
/// image load unless `+0xbc` is all ones, then update the UI.
pub fn c_fe_state_mountain_room_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = lw(c, this, 0xbc);
        c.g[V0][0] = !(0u64 | c.g[V0][0]);
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            callee::MOUNTAIN_ROOM_POLL_IMAGE_LOAD.call(c, 0x001d_3148)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::UI_STATE_UPDATE_2.call(c, 0x001d_3150)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMountainRoom_RequestImage(cFEStateMountainRoom* self)`
/// (0x1d3160): if the image request has not started (`+0xbc < 0`) and an item
/// is queued (`+0xc0`), start loading item `+0xb0` through the bolt file of
/// the world's rider-0 manager.
pub fn cfe_state_mountain_room_request_image(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_2(c, p(c.g[S0][0], 0xbc));
        if (c.g[V0][0] as i64) >= 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[V1][0] = lw_2(c, p(c.g[S0][0], 0xc0));
            let no_item = c.g[V1][0] == 0;
            c.g[V0][0] = lw_2(c, gp(c, -0x848));
            if !no_item {
                c.cycles += 9;
                c.g[A3][0] = 0xa;
                c.g[A2][0] = lw_2(c, p(c.g[V1][0], 4));
                c.g[T0][0] = 1;
                c.g[A0][0] = lw_2(c, p(c.g[V0][0], 0x7c));
                c.g[T1][0] = 0;
                c.g[A1][0] = lw_2(c, p(c.g[S0][0], 0xb0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
                call_ptr(c, 0x0019_da10, 0x001d_31a8)?;
                sw(c, p(c.g[S0][0], 0xbc), c.g[V0][0]);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateMountainRoom_OnWidgetNotify(State*, void*, int key)`
/// (0x001d2f40).
pub fn cfe_state_mountain_room_on_widget_notify(c: &mut Cpu) {
    handle_list_key(c);
}

/// `void cFEStateMountainRoom_ShowImage(cFEStateMountainRoom* self, int a1)`
/// (0x1d32e0): show the room's image, and the second one when it has been
/// unlocked.
pub fn c_fe_state_mountain_room_show_image(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = c.g[A0][0];
        c.g[S0][0] = self_;
        c.g[A2][0] = Addr(at(c, S0, 0x48)).lw(c);
        c.g[V0][0] = Addr(r32_2(c, A2)).off(0x8).lw(c);
        c.g[A0][0] = Addr(r32_2(c, V0)).off(0x48).lh(c);
        c.g[V1][0] = Addr(r32_2(c, V0)).off(0x4c).lw(c);
        let target = r32_2(c, V1);
        c.g[A0][0] = abi::addu(c.g[A2][0], c.g[A0][0]);
        call_ptr(c, target, 0x001d_330c)?;
        c.g[V0][0] = Addr(at(c, S0, 0xc4)).lw(c);
        let locked = (c.g[V0][0] as i64) < 0;
        c.g[A1][0] = 0x1;
        if !locked {
            c.cycles += 6;
            c.g[A2][0] = Addr(at(c, S0, 0x4c)).lw(c);
            c.g[V0][0] = Addr(r32_2(c, A2)).off(0x8).lw(c);
            c.g[A0][0] = Addr(r32_2(c, V0)).off(0x48).lh(c);
            c.g[V1][0] = Addr(r32_2(c, V0)).off(0x4c).lw(c);
            let target = r32_2(c, V1);
            c.g[A0][0] = abi::addu(c.g[A2][0], c.g[A0][0]);
            call_ptr(c, target, 0x001d_3330)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStatePeakRoom_Init(cFEStatePeakRoom* self)` (0x1d3890).
pub fn c_fe_state_peak_room_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0046_7648, [0x001d_38b4, 0x001d_38c8, 0x001d_38e0])
    })
}

/// `void cFEStateMountainRoom_dtor(self, flags)` (0x1d28f8): release the room's
/// music slot and rider, repoint the vtable and run the base destructor.
pub fn cfe_state_mountain_room_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x470000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x67a8));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(c.g[S0][0] as u32).off(0x8).sw(c, c.g[V0][0]);
        c.g[A2][0] = lw_4(c, S0, 0xbc);
        let no_slot = (c.g[A2][0] as i64) < 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_slot {
            c.cycles += 7;
            c.g[V0][0] = Addr(gp_2(c, -0x848)).lw(c);
            c.g[A1][0] = lw_4(c, S0, 0xb0);
            c.g[A0][0] = lw_4(c, V0, 0x7c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
            call_ptr(c, table_callee::MUSIC_SLOT_RELEASE, 0x001d_2938)?;
            c.g[V0][0] = u64::MAX;
            Addr(c.g[S0][0] as u32).off(0xbc).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[A1][0] = lw_4(c, S0, 0xc4);
        let no_rider = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = Addr(gp_2(c, -0x854)).lw(c);
        if !no_rider {
            c.cycles += 5;
            c.g[V0][0] = lw_4(c, A2, 0x10d8);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x190).lh(c);
            c.g[V1][0] = lw_4(c, V0, 0x194);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x001d_2960)?;
        }
        c.cycles += 11;
        c.g[V0][0] = 0x470000;
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2f30));
        c.g[A1][0] = c.g[S1][0];
        Addr(c.g[S0][0] as u32).off(0x8).sw(c, c.g[V0][0]);
        call_ptr(c, table_callee::UI_STATE_DTOR, 0x001d_2978)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool cFEStateMountainRoom_HandleInput(self, input)` (0x1d2ea0): when the
/// widget is shown and its sound event fires, play the activate sound and
/// hide the widget.
pub fn cfe_state_mountain_room_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        c.g[V1][0] = lw_5(c, S0, 0x48);
        let no_widget = c.g[V1][0] == 0;
        c.g[V0][0] = 0;
        if !no_widget {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x14).lw(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
            c.g[V0][0] &= 1;
        }
        c.cycles += 2;
        let hidden = c.g[V0][0] == 0;
        c.g[V0][0] = 0;
        if !hidden {
            c.cycles += 7;
            c.g[V1][0] = Addr(abi::arg(c, 1)).off(8).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x30).lh(c);
            c.g[V0][0] = lw_5(c, V1, 0x34);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001d_2ee4)?;
            let click = c.g[V0][0] == 0;
            c.g[A1][0] = 3;
            if !click {
                c.cycles += 16;
                c.g[V0][0] = lw_5(c, S0, 0x10);
                c.g[V1][0] = sx(gp(c, 0x2968));
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x14).lw(c);
                if c.g[A0][0] == 0 {
                    c.g[A0][0] = c.g[V1][0];
                }
                c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x10).lh(c);
                c.g[A2][0] = lw_5(c, V0, 0x14);
                let t = c.g[A2][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                call_ptr(c, t, 0x001d_2f10)?;
                c.g[A3][0] = lw_5(c, S0, 0x48);
                c.g[A1][0] = 0;
                c.g[V0][0] = Addr(c.g[A3][0] as u32).off(8).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
                c.g[V1][0] = lw_5(c, V0, 0x4c);
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001d_2f2c)?;
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStatePeakRoom_OnWidgetNotify` (0x001d3e08).
//
// `int cFEStatePeakRoom::OnWidgetNotify(cFEState* self, UIWidget* item,
//  int key, int id)`: keys 8 and 9 report "handled with 0x100"; key 6 routes
// the selection into the lodge screen and reports 0x10/0x101 by whether the
// comparison succeeded; everything else reports 0x101.

pub fn fe_state_peak_room_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        'b_1d3ea8: {
            'b_1d3ea4: {
                'b_1d3e5c: {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.g[V0][0] = 0x6;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S2][0] = c.g[A3][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S3][0] = c.g[A1][0];
                    let t = c.g[A2][0] == c.g[V0][0];
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    if t {
                        break 'b_1d3e5c;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[A2][0] < 0x6u64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = 0x101;
                        if t {
                            break 'b_1d3ea8;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[A2][0] < 0xau64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = (c.g[A2][0] < 0x8u64) as u64;
                            if t {
                                break 'b_1d3ea4;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0x101;
                                if t {
                                    break 'b_1d3ea8;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[V0][0] = 0x100;
                                    if t {
                                        break 'b_1d3ea8;
                                    } else {
                                        break 'b_1d3e5c;
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 18;
                c.g[RA][0] = sx(0x001d3e64);
                call_ptr(c, 0x0014dc80, 0x001d3e64)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001d3e70);
                c.g[A1][0] = 0xd;
                call_ptr(c, 0x0014dd58, 0x001d3e70)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x74));
                c.g[RA][0] = sx(0x001d3e80);
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x00397870, 0x001d3e80)?;
                c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe0));
                c.g[A0][0] = c.g[S0][0];
                c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                c.g[RA][0] = sx(0x001d3e98);
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xdc));
                call_ptr(c, 0x00157bf0, 0x001d3e98)?;
                c.g[V1][0] = c.g[V0][0];
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = 0x10;
                if t {
                    break 'b_1d3ea8;
                } else {
                    break 'b_1d3ea4;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x101;
            break 'b_1d3ea8;
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `void func_001D2F68(void* self, void* widget, int msg)` (0x1d2f68): the
/// mountain room screen's widget action handler: message 5 replaces a room
/// image object with the next one from the factory, 6 does the same for a
/// specific index, and 1 sets the caption text of `self+0x54` from the
/// widget's index.
pub fn c_fe_state_mountain_room_on_widget_action(c: &mut Cpu) {
    let frame = Frame::open(c, 0x30);
    'b_1d3110: {
        'b_1d310c: {
            c.cycles += 7;
            c.g[V1][0] = c.g[A1][0];
            frame.save(c, S0, 0x20);
            frame.save(c, S1, 0x10);
            c.g[S0][0] = c.g[A0][0];
            let t = c.g[V1][0] == 0u64;
            sd(c, r32(c, SP), c.g[RA][0]);
            if t {
                break 'b_1d310c;
            } else {
                'b_1d3040: {
                    'b_1d302c: {
                        'b_1d3000: {
                            'b_1d2fbc: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x5;
                                let t = c.g[A2][0] == c.g[V0][0];
                                c.g[V0][0] = (c.g[A2][0] < 0x6u64) as u64;
                                if t {
                                    break 'b_1d2fbc;
                                } else {
                                    'b_1d2fa8: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_1d2fa8;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[A2][0] == c.g[V0][0] {
                                                c.g[V0][0] = lw_7(c, at_4(c, S0, 0x54));
                                                break 'b_1d3040;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                frame.restore(c, S0, 0x20);
                                                if t {
                                                    break 'b_1d3110;
                                                } else {
                                                    break 'b_1d2fa8;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x6;
                                    if c.g[A2][0] == c.g[V0][0] {
                                        c.g[V0][0] = lw_7(c, at_4(c, S0, 0x10));
                                        break 'b_1d3000;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        frame.restore(c, S0, 0x20);
                                        if t {
                                            break 'b_1d3110;
                                        } else {
                                            break 'b_1d2fbc;
                                        }
                                    }
                                }
                            }
                            'b_1d2fd8: {
                                c.cycles += 3;
                                c.g[A1][0] = lw_7(c, at_4(c, V1, 0x18));
                                if c.g[A1][0] != 0u64 {
                                    c.g[V0][0] = lw_7(c, at_4(c, S0, 0x10));
                                    break 'b_1d2fd8;
                                } else {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001d2fd0);
                                    c.g[A0][0] = c.g[S0][0];
                                    if callee::C001D32E0.call(c, 0x001d2fd0).is_err() {
                                        return;
                                    }
                                    let t = true;
                                    frame.restore(c, S0, 0x20);
                                    if t {
                                        break 'b_1d3110;
                                    } else {
                                        break 'b_1d2fd8;
                                    }
                                }
                            }
                            c.cycles += 10;
                            c.g[A2][0] = sx(r32(c, A1).wrapping_sub(0x1));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A3][0] = lw_7(c, r32(c, V0));
                            c.g[V1][0] = lw_7(c, at_4(c, A3, 0x4));
                            c.g[A0][0] = lh(c, at_4(c, V1, 0x20));
                            c.g[V0][0] = lw_7(c, at_4(c, V1, 0x24));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x001d2ff8);
                            c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                            if call_ptr(c, t, 0x001d2ff8).is_err() {
                                return;
                            }
                            let t = true;
                            c.g[A0][0] = lw_7(c, at_4(c, S0, 0x10));
                            if t {
                                break 'b_1d302c;
                            } else {
                                break 'b_1d3000;
                            }
                        }
                        c.cycles += 10;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = lw_7(c, at_4(c, V1, 0x18));
                        c.g[A3][0] = lw_7(c, r32(c, V0));
                        c.g[V1][0] = lw_7(c, at_4(c, A3, 0x4));
                        c.g[A0][0] = lh(c, at_4(c, V1, 0x28));
                        c.g[V0][0] = lw_7(c, at_4(c, V1, 0x2c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001d3020);
                        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                        if call_ptr(c, t, 0x001d3020).is_err() {
                            return;
                        }
                        if c.g[V0][0] == 0u64 {
                            frame.restore(c, S0, 0x20);
                            break 'b_1d3110;
                        } else {
                            c.cycles += 1;
                            c.g[A0][0] = lw_7(c, at_4(c, S0, 0x10));
                            break 'b_1d302c;
                        }
                    }
                    c.cycles += 5;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001d3038);
                    c.g[A0][0] = sx(at_4(c, A0, 0x18));
                    if call_ptr(c, 0x0039f400, 0x001d3038).is_err() {
                        return;
                    }
                    let t = true;
                    frame.restore(c, S0, 0x20);
                    if t {
                        break 'b_1d3110;
                    } else {
                        break 'b_1d3040;
                    }
                }
                c.cycles += 2;
                if c.g[V0][0] == 0u64 {
                    frame.restore(c, S0, 0x20);
                    break 'b_1d3110;
                } else {
                    'b_1d30e8: {
                        'b_1d30d8: {
                            'b_1d30bc: {
                                'b_1d309c: {
                                    'b_1d3090: {
                                        c.cycles += 3;
                                        c.g[A1][0] = lw_7(c, at_4(c, V1, 0x18));
                                        let t = c.g[A1][0] == c.g[A2][0];
                                        c.g[S1][0] = 0u64;
                                        if t {
                                            break 'b_1d3090;
                                        } else {
                                            'b_1d3088: {
                                                'b_1d3070: {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        ((c.g[A1][0] as i64) < 2i64) as u64;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x2;
                                                    if t {
                                                        break 'b_1d3070;
                                                    } else {
                                                        c.cycles += 2;
                                                        if c.g[A1][0] == 0u64 {
                                                            c.g[A0][0] = 0x460000;
                                                            break 'b_1d3088;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A2][0] = lw_7(c, at_4(c, S0, 0x54));
                                                            if t {
                                                                break 'b_1d30e8;
                                                            } else {
                                                                break 'b_1d3070;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[A1][0] == c.g[V0][0];
                                                c.g[V0][0] = 0x3;
                                                if t {
                                                    break 'b_1d309c;
                                                } else {
                                                    c.cycles += 2;
                                                    if c.g[A1][0] == c.g[V0][0] {
                                                        c.g[V0][0] = lw_7(c, at_4(c, S0, 0xb4));
                                                        break 'b_1d30bc;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[A2][0] = lw_7(c, at_4(c, S0, 0x54));
                                                        if t {
                                                            break 'b_1d30e8;
                                                        } else {
                                                            break 'b_1d3088;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A0][0] = sx(at_4(c, A0, 0x7450));
                                            if t {
                                                break 'b_1d30d8;
                                            } else {
                                                break 'b_1d3090;
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x460000;
                                    let t = true;
                                    c.g[A0][0] = sx(at_4(c, A0, 0x7460));
                                    if t {
                                        break 'b_1d30d8;
                                    } else {
                                        break 'b_1d309c;
                                    }
                                }
                                'b_1d30b4: {
                                    c.cycles += 3;
                                    c.g[V0][0] = lw_7(c, at_4(c, S0, 0xb4));
                                    if (c.g[V0][0] as i64) <= 0 {
                                        c.g[A0][0] = 0x460000;
                                        break 'b_1d30b4;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x460000;
                                        let t = true;
                                        c.g[A0][0] = sx(at_4(c, A0, 0x7460));
                                        if t {
                                            break 'b_1d30d8;
                                        } else {
                                            break 'b_1d30b4;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = true;
                                c.g[A0][0] = sx(at_4(c, A0, 0x7478));
                                if t {
                                    break 'b_1d30d8;
                                } else {
                                    break 'b_1d30bc;
                                }
                            }
                            'b_1d30d4: {
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[A0][0] = 0x460000;
                                    break 'b_1d30d4;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x460000;
                                    let t = true;
                                    c.g[A0][0] = sx(at_4(c, A0, 0x7460));
                                    if t {
                                        break 'b_1d30d8;
                                    } else {
                                        break 'b_1d30d4;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[A0][0] = sx(at_4(c, A0, 0x7490));
                            break 'b_1d30d8;
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001d30e0);
                        if call_ptr(c, 0x00317670, 0x001d30e0).is_err() {
                            return;
                        }
                        c.g[S1][0] = c.g[V0][0];
                        c.g[A2][0] = lw_7(c, at_4(c, S0, 0x54));
                        break 'b_1d30e8;
                    }
                    c.cycles += 9;
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = lw_7(c, at_4(c, A2, 0x8));
                    c.g[A0][0] = lh(c, at_4(c, V0, 0x48));
                    c.g[V1][0] = lw_7(c, at_4(c, V0, 0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001d3100);
                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x001d3100).is_err() {
                        return;
                    }
                    c.g[A0][0] = lw_7(c, at_4(c, S0, 0x54));
                    c.g[RA][0] = sx(0x001d310c);
                    c.g[A1][0] = c.g[S1][0];
                    if call_ptr(c, 0x003a1000, 0x001d310c).is_err() {
                        return;
                    }
                    break 'b_1d310c;
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
        break 'b_1d3110;
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_4(c, SP, 0x30));
    c.pc = t;
    return;
}

/// `cFEStateMountainRoom* cFEStateMountainRoom::ctor(this, a1, signed char player)`
/// (0x001d2678): base-state constructor, vtable, player slot, zeroed member
/// arrays, then the first character slot with a rider event record
/// (`BENewPlayerInt_GetCharStateFlag` over the three slots), the reward lookups,
/// two string-hash texture lookups, and `RequestImage(this)`; returns `this`.
///
/// Converted from the generated labelled-block body (every register, cycle and
/// call kept). Any other `pc` (a resume) runs the generated body.
pub fn c_fe_state_mountain_room_ctor(c: &mut Cpu) {
    if c.pc != 0x001d2678 {
        return ee::s04::f_001d2678__generated(c);
    }
    guest(c, |c| {
        c.cycles += 28;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = sx((c.g[A2][0] as u32) << 24);
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x001d26a4);
        c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
        call_ptr(c, 0x0039e2a0, 0x001d26a4)?;
        c.g[V0][0] = 0x2b;
        c.g[V1][0] = 0x470000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x67a8));
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = c.g[S0][0];
        c.sb((c.g[S1][0] as u32).wrapping_add(0x44), c.g[S0][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x001d26cc);
        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
        call_ptr(c, 0x001a1cd0, 0x001d26cc)?;
        c.sb((c.g[S1][0] as u32).wrapping_add(0x15), c.g[V0][0]);
        c.g[V1][0] = 0x4;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x48), 0u64);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x68));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x50), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x54), 0u64);
        'l_1d26e8: loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            let t = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001d26e8);
                }
                continue 'l_1d26e8;
            } else {
                c.cycles += 7;
                c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x58));
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x78));
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x84));
                c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x90));
                c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x9c));
                c.g[V1][0] = 0x2;
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                'l_1d2720: loop {
                    c.cycles += 7;
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[V1][0] as i64) >= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001d2720);
                        }
                        continue 'l_1d2720;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
                        c.g[V1][0] = 0x2;
                        'l_1d2748: loop {
                            c.cycles += 7;
                            c.sw(c.g[V0][0] as u32, 0u64);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[V1][0] as i64) >= 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x001d2748);
                                }
                                continue 'l_1d2748;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = 0x2;
                                'l_1d2770: loop {
                                    c.cycles += 7;
                                    c.sw(c.g[V0][0] as u32, 0u64);
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                    let t = (c.g[V1][0] as i64) >= 0;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x001d2770);
                                        }
                                        continue 'l_1d2770;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = 0x2;
                                        'l_1d2798: loop {
                                            c.cycles += 7;
                                            c.sw(c.g[V0][0] as u32, 0u64);
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                            let t = (c.g[V1][0] as i64) >= 0;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x001d2798);
                                                }
                                                continue 'l_1d2798;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    sx((c.g[A3][0] as u32).wrapping_add(0x8));
                                                c.g[V1][0] = 0x2;
                                                'l_1d27c0: loop {
                                                    c.cycles += 7;
                                                    c.sw(c.g[V0][0] as u32, 0u64);
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                                    let t = (c.g[V1][0] as i64) >= 0;
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x001d27c0);
                                                        }
                                                        continue 'l_1d27c0;
                                                    } else {
                                                        c.cycles += 13;
                                                        c.sw(
                                                            (c.g[S1][0] as u32).wrapping_add(0xa8),
                                                            0u64,
                                                        );
                                                        c.g[S0][0] = 0u64;
                                                        c.g[RA][0] = sx(0x001d27ec);
                                                        c.sw(
                                                            (c.g[S1][0] as u32).wrapping_add(0xac),
                                                            0u64,
                                                        );
                                                        call_ptr(c, 0x0014dc80, 0x001d27ec)?;
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[RA][0] = sx(0x001d27f8);
                                                        c.g[A1][0] = 0x1;
                                                        call_ptr(c, 0x0014dd58, 0x001d27f8)?;
                                                        c.g[S2][0] = c.g[V0][0];
                                                        c.g[A1][0] = c
                                                            .lb((c.g[S1][0] as u32)
                                                                .wrapping_add(0x44));
                                                        c.g[RA][0] = sx(0x001d2808);
                                                        c.g[A0][0] = c.g[S2][0];
                                                        call_ptr(c, 0x00147488, 0x001d2808)?;
                                                        c.sw(
                                                            (c.g[S1][0] as u32).wrapping_add(0xb0),
                                                            c.g[V0][0],
                                                        );
                                                        c.g[A1][0] = c
                                                            .lb((c.g[S1][0] as u32)
                                                                .wrapping_add(0x44));
                                                        'l_1d2810: loop {
                                                            'b_1d2828: {
                                                                c.cycles += 6;
                                                                c.g[A0][0] = c.g[S2][0];
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0xb0));
                                                                c.g[RA][0] = sx(0x001d2820);
                                                                c.g[A3][0] = c.g[S0][0];
                                                                call_ptr(
                                                                    c, 0x00145f90, 0x001d2820,
                                                                )?;
                                                                if c.g[V0][0] == 0u64 {
                                                                    c.sw(
                                                                        (c.g[S1][0] as u32)
                                                                            .wrapping_add(0xb4),
                                                                        c.g[S0][0],
                                                                    );
                                                                    break 'b_1d2828;
                                                                } else {
                                                                    break 'b_1d2828;
                                                                }
                                                            }
                                                            c.cycles += 4;
                                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[V0][0] =
                                                                ((c.g[S0][0] as i64) < 3i64) as u64;
                                                            if c.g[V0][0] != 0u64 {
                                                                c.g[A1][0] = c
                                                                    .lb((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x44));
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x001d2810,
                                                                    );
                                                                }
                                                                continue 'l_1d2810;
                                                            } else {
                                                                c.cycles += 48;
                                                                c.g[RA][0] = sx(0x001d2840);
                                                                call_ptr(
                                                                    c, 0x0014dc80, 0x001d2840,
                                                                )?;
                                                                c.g[A1][0] = 0x3;
                                                                c.g[RA][0] = sx(0x001d284c);
                                                                c.g[A0][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x0014dd58, 0x001d284c,
                                                                )?;
                                                                c.g[A0][0] = c
                                                                    .lb((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x44));
                                                                c.g[RA][0] = sx(0x001d2858);
                                                                c.g[S0][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x0014a130, 0x001d2858,
                                                                )?;
                                                                c.g[A0][0] = c.g[S0][0];
                                                                c.g[RA][0] = sx(0x001d2864);
                                                                c.g[A1][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x00148950, 0x001d2864,
                                                                )?;
                                                                c.g[RA][0] = sx(0x001d286c);
                                                                c.sw(
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0xb8),
                                                                    c.g[V0][0],
                                                                );
                                                                call_ptr(
                                                                    c, 0x0014dc80, 0x001d286c,
                                                                )?;
                                                                c.g[A0][0] = c.g[V0][0];
                                                                c.g[RA][0] = sx(0x001d2878);
                                                                c.g[A1][0] = 0xd;
                                                                call_ptr(
                                                                    c, 0x0014dd58, 0x001d2878,
                                                                )?;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0xb0));
                                                                c.g[A0][0] = c.g[V0][0];
                                                                c.g[RA][0] = sx(0x001d2888);
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0xb4));
                                                                call_ptr(
                                                                    c, 0x00156b28, 0x001d2888,
                                                                )?;
                                                                c.g[V1][0] = 0xffffffffffffffff;
                                                                c.g[A0][0] = 0x460000;
                                                                c.sw(
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0xc4),
                                                                    c.g[V1][0],
                                                                );
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x7208));
                                                                c.sw(
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0xbc),
                                                                    c.g[V1][0],
                                                                );
                                                                c.g[RA][0] = sx(0x001d28a4);
                                                                c.sw(
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0xc0),
                                                                    c.g[V0][0],
                                                                );
                                                                call_ptr(
                                                                    c, 0x00317670, 0x001d28a4,
                                                                )?;
                                                                c.g[A0][0] = c.g[S3][0];
                                                                c.g[RA][0] = sx(0x001d28b0);
                                                                c.g[A1][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x00398380, 0x001d28b0,
                                                                )?;
                                                                c.g[A0][0] = 0x460000;
                                                                c.sw(
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0xac),
                                                                    c.g[V0][0],
                                                                );
                                                                c.g[RA][0] = sx(0x001d28c0);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x7218));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x001d28c0,
                                                                )?;
                                                                c.g[A0][0] = c.g[S3][0];
                                                                c.g[RA][0] = sx(0x001d28cc);
                                                                c.g[A1][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x00398380, 0x001d28cc,
                                                                )?;
                                                                c.sw(
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0xa8),
                                                                    c.g[V0][0],
                                                                );
                                                                c.g[RA][0] = sx(0x001d28d8);
                                                                c.g[A0][0] = c.g[S1][0];
                                                                Callee { addr: 0x001d3160, f: ee::s04::cFEStateMountainRoom_RequestImage }.call(c, 0x001d28d8)?;
                                                                c.g[V0][0] = c.g[S1][0];
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x40),
                                                                );
                                                                c.lq(
                                                                    17,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                c.lq(
                                                                    18,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.lq(
                                                                    19,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                c.g[RA][0] =
                                                                    c.ld(c.g[SP][0] as u32);
                                                                let t = c.g[RA][0] as u32;
                                                                c.g[SP][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0x50));
                                                                c.pc = t;
                                                                return Ok(());
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}

/// `cFEStateMountainRoom_OnWidgetCreated(this, widget)` (0x001d2ad0): a screen widget was just created; its name hash (`widget+0x38`) is matched against the screen's known names and the widget is stored in the matching slot of `this` (title text, pass icon, level-number texture, race / slopestyle / superpipe / backcountry / big-air marker slots, peak pages 0-3, help text).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias). Every register,
/// cycle and call of the generated body is kept.
pub fn c_fe_state_mountain_room_on_widget_created(c: &mut Cpu) {
    if c.pc != 0x001d2ad0 {
        return ee::s04::f_001d2ad0__generated(c);
    }
    guest(c, |c| {
        'b_1d2e88: {
            'b_1d2b24: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S2][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                c.g[S1][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[A0][0] = 0x460000;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                c.g[RA][0] = sx(0x001d2afc);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e60));
                call_ptr(c, 0x00317670, 0x001d2afc)?;
                if c.g[S0][0] != c.g[V0][0] {
                    c.g[A0][0] = 0x460000;
                    break 'b_1d2b24;
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x001d2b10);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7238));
                    call_ptr(c, 0x00317670, 0x001d2b10)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001d2b1c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001d2b1c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    if t {
                        break 'b_1d2e88;
                    } else {
                        break 'b_1d2b24;
                    }
                }
            }
            'b_1d2e84: {
                'b_1d2dec: {
                    'b_1d2dcc: {
                        'b_1d2b3c: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2b2c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7250));
                            call_ptr(c, 0x00317670, 0x001d2b2c)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x4a0000;
                                break 'b_1d2b3c;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x48), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2b3c;
                                }
                            }
                        }
                        'b_1d2b54: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2b44);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f20));
                            call_ptr(c, 0x00317670, 0x001d2b44)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2b54;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x4c), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2b54;
                                }
                            }
                        }
                        'b_1d2b6c: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2b5c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7260));
                            call_ptr(c, 0x00317670, 0x001d2b5c)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2b6c;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x50), c.g[S1][0]);
                                if t {
                                    break 'b_1d2e84;
                                } else {
                                    break 'b_1d2b6c;
                                }
                            }
                        }
                        'b_1d2bf0: {
                            'b_1d2b8c: {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001d2b74);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7270));
                                call_ptr(c, 0x00317670, 0x001d2b74)?;
                                let t = c.g[S0][0] == c.g[V0][0];
                                c.g[A0][0] = 0x460000;
                                if t {
                                    break 'b_1d2b8c;
                                } else {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001d2b84);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7280));
                                    call_ptr(c, 0x00317670, 0x001d2b84)?;
                                    let t = c.g[S0][0] != c.g[V0][0];
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_1d2bf0;
                                    } else {
                                        break 'b_1d2b8c;
                                    }
                                }
                            }
                            c.cycles += 25;
                            c.g[V0][0] = 0x460000;
                            c.g[A0][0] = c.g[SP][0] | 0x5;
                            c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7298));
                            c.lwl(3, (c.g[T1][0] as u32).wrapping_add(0x3));
                            c.lwr(3, c.g[T1][0] as u32);
                            c.g[A3][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x4));
                            c.swl(3, (c.g[SP][0] as u32).wrapping_add(0x3));
                            c.swr(3, c.g[SP][0] as u32);
                            c.sb((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A3][0]);
                            c.g[A1][0] = 0u64;
                            c.g[RA][0] = sx(0x001d2bbc);
                            c.g[A2][0] = 0xf;
                            call_ptr(c, 0x00416210, 0x001d2bbc)?;
                            c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0xb8));
                            c.g[A0][0] = c.g[SP][0];
                            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
                            c.g[RA][0] = sx(0x001d2bd4);
                            c.sb((c.g[SP][0] as u32).wrapping_add(0x2), c.g[V0][0]);
                            call_ptr(c, 0x00317670, 0x001d2bd4)?;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                            c.g[RA][0] = sx(0x001d2be0);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x00398380, 0x001d2be0)?;
                            c.g[V1][0] = 0xffffffffffffffff;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x7c), c.g[V0][0]);
                            let t = true;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x78), c.g[V1][0]);
                            if t {
                                break 'b_1d2e84;
                            } else {
                                break 'b_1d2bf0;
                            }
                        }
                        'b_1d2c08: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2bf8);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x72a0));
                            call_ptr(c, 0x00317670, 0x001d2bf8)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2c08;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x58), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2c08;
                                }
                            }
                        }
                        'b_1d2c20: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2c10);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x72b0));
                            call_ptr(c, 0x00317670, 0x001d2c10)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2c20;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x5c), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2c20;
                                }
                            }
                        }
                        'b_1d2c38: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2c28);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x72c0));
                            call_ptr(c, 0x00317670, 0x001d2c28)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2c38;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x60), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2c38;
                                }
                            }
                        }
                        'b_1d2c50: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2c40);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x72d0));
                            call_ptr(c, 0x00317670, 0x001d2c40)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2c50;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x64), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2c50;
                                }
                            }
                        }
                        'b_1d2c68: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2c58);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x72e0));
                            call_ptr(c, 0x00317670, 0x001d2c58)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2c68;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x68), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2c68;
                                }
                            }
                        }
                        'b_1d2c80: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2c70);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x72f0));
                            call_ptr(c, 0x00317670, 0x001d2c70)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2c80;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x6c), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2c80;
                                }
                            }
                        }
                        'b_1d2c98: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2c88);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7308));
                            call_ptr(c, 0x00317670, 0x001d2c88)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2c98;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x70), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2c98;
                                }
                            }
                        }
                        'b_1d2cb0: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2ca0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7320));
                            call_ptr(c, 0x00317670, 0x001d2ca0)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2cb0;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x74), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2cb0;
                                }
                            }
                        }
                        'b_1d2cc8: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2cb8);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7338));
                            call_ptr(c, 0x00317670, 0x001d2cb8)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2cc8;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x90), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2cc8;
                                }
                            }
                        }
                        'b_1d2ce0: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2cd0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7348));
                            call_ptr(c, 0x00317670, 0x001d2cd0)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2ce0;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x94), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2ce0;
                                }
                            }
                        }
                        'b_1d2cf8: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2ce8);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7358));
                            call_ptr(c, 0x00317670, 0x001d2ce8)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2cf8;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2cf8;
                                }
                            }
                        }
                        'b_1d2d10: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2d00);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7368));
                            call_ptr(c, 0x00317670, 0x001d2d00)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2d10;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x78), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2d10;
                                }
                            }
                        }
                        'b_1d2d28: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2d18);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7380));
                            call_ptr(c, 0x00317670, 0x001d2d18)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2d28;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x7c), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2d28;
                                }
                            }
                        }
                        'b_1d2d40: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2d30);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7398));
                            call_ptr(c, 0x00317670, 0x001d2d30)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2d40;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x80), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2d40;
                                }
                            }
                        }
                        'b_1d2d58: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2d48);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x73b0));
                            call_ptr(c, 0x00317670, 0x001d2d48)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2d58;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x84), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2d58;
                                }
                            }
                        }
                        'b_1d2d70: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2d60);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x73d0));
                            call_ptr(c, 0x00317670, 0x001d2d60)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2d70;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x88), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2d70;
                                }
                            }
                        }
                        'b_1d2d88: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2d78);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x73f0));
                            call_ptr(c, 0x00317670, 0x001d2d78)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2d88;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x8c), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2d88;
                                }
                            }
                        }
                        'b_1d2da0: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2d90);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7410));
                            call_ptr(c, 0x00317670, 0x001d2d90)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2da0;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x9c), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2da0;
                                }
                            }
                        }
                        'b_1d2db8: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2da8);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7420));
                            call_ptr(c, 0x00317670, 0x001d2da8)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2db8;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0xa0), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2db8;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001d2dc0);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7430));
                        call_ptr(c, 0x00317670, 0x001d2dc0)?;
                        if c.g[S0][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x460000;
                            break 'b_1d2dec;
                        } else {
                            c.cycles += 1;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0xa4), c.g[S1][0]);
                            break 'b_1d2dcc;
                        }
                    }
                    c.cycles += 8;
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001d2de4);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001d2de4)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    if t {
                        break 'b_1d2e88;
                    } else {
                        break 'b_1d2dec;
                    }
                }
                'b_1d2e04: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d2df4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7440));
                    call_ptr(c, 0x00317670, 0x001d2df4)?;
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        break 'b_1d2e04;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
                        if t {
                            break 'b_1d2e84;
                        } else {
                            break 'b_1d2e04;
                        }
                    }
                }
                'b_1d2e20: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d2e0c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f28));
                    call_ptr(c, 0x00317670, 0x001d2e0c)?;
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        break 'b_1d2e20;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_1d2e84;
                        } else {
                            break 'b_1d2e20;
                        }
                    }
                }
                'b_1d2e3c: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d2e28);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f30));
                    call_ptr(c, 0x00317670, 0x001d2e28)?;
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        break 'b_1d2e3c;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x2;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_1d2e84;
                        } else {
                            break 'b_1d2e3c;
                        }
                    }
                }
                'b_1d2e58: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d2e44);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f38));
                    call_ptr(c, 0x00317670, 0x001d2e44)?;
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        break 'b_1d2e58;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x3;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_1d2e84;
                        } else {
                            break 'b_1d2e58;
                        }
                    }
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x001d2e60);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x488));
                call_ptr(c, 0x00317670, 0x001d2e60)?;
                let t = c.g[S0][0] != c.g[V0][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                if t {
                    break 'b_1d2e88;
                } else {
                    c.cycles += 7;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x54), c.g[S1][0]);
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001d2e84);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001d2e84)?;
                    break 'b_1d2e84;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            break 'b_1d2e88;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateMountainRoom_UpdateEventIcons(this)` (0x001d3340): walks mountains 0-2, peaks 0-1 and the up-to-five events of each (medal table at 0x45aad8), asks the trophy room for the event's reward and sets the marker widget of its kind (race, slopestyle, superpipe, big air, backcountry race / freestyle) to the icon of the result (`+0x78` icon, `+0x7c` texture).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias). Every register,
/// cycle and call of the generated body is kept.
pub fn c_fe_state_mountain_room_update_event_icons(c: &mut Cpu) {
    if c.pc != 0x001d3340 {
        return ee::s04::f_001d3340__generated(c);
    }
    guest(c, |c| {
        c.cycles += 20;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[S1][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
        c.sw(c.g[SP][0] as u32, 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
        'l_1d3390: loop {
            c.cycles += 22;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
            c.g[V1][0] = 0x28;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
            c.mult(0, 2, 2, 3);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), 0u64);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A0][0]);
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
            c.g[S7][0] = sx((c.g[V1][0] as u32) << 2);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
            c.g[S6][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[FP][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
            c.g[S4][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[S3][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[S5][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
            'l_1d33e8: loop {
                'b_1d372c: {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[A0][0]);
                    if t {
                        break 'b_1d372c;
                    } else {
                        c.cycles += 12;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                        c.g[V0][0] = 0xa;
                        c.g[A0][0] = 0x460000;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
                        c.lo[0] = c.g[V1][0];
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5528));
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                        c.madd(0, 2, 3, 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[S0][0] = c.lb(c.g[V0][0] as u32);
                        let t = (c.g[S0][0] as i64) < 0;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        if t {
                            break 'b_1d372c;
                        } else {
                            c.cycles += 19;
                            c.g[S2][0] = 0xffffffffffffffff;
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V1][0]);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x38), c.g[A0][0]);
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V1][0]);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[A0][0]);
                            c.g[A0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                            'l_1d3478: loop {
                                'b_1d36e4: {
                                    'b_1d3674: {
                                        'b_1d3658: {
                                            'b_1d34e8: {
                                                c.cycles += 7;
                                                c.g[A2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                                c.g[A1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0xb0));
                                                c.g[A3][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                                c.g[RA][0] = sx(0x001d348c);
                                                c.g[T0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                                Callee {
                                                    addr: 0x001ced90,
                                                    f: ee::s04::cFEStateTrophyRoom_QueryEventReward,
                                                }
                                                .call(c, 0x001d348c)?;
                                                let t = c.g[S0][0] != 0u64;
                                                c.g[V1][0] = c.g[V0][0];
                                                if t {
                                                    break 'b_1d34e8;
                                                } else {
                                                    'b_1d34ac: {
                                                        'b_1d34a8: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] =
                                                                (c.g[V1][0] < 0x4u64) as u64;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[V1][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x58));
                                                            if t {
                                                                break 'b_1d34a8;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0xa8));
                                                                if t {
                                                                    break 'b_1d34ac;
                                                                } else {
                                                                    break 'b_1d34a8;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 1;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0xac));
                                                        break 'b_1d34ac;
                                                    }
                                                    c.cycles += 15;
                                                    c.g[A0][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add(c.g[S3][0] as u32));
                                                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x7c),
                                                        c.g[A1][0],
                                                    );
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x78),
                                                        c.g[S2][0],
                                                    );
                                                    c.g[A0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x44));
                                                    c.g[S3][0] =
                                                        sx((c.g[S3][0] as u32).wrapping_add(0x4));
                                                    c.g[A1][0] = 0x1;
                                                    c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x44),
                                                        c.g[A0][0],
                                                    );
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                    let t = true;
                                                    c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                                                    if t {
                                                        break 'b_1d3658;
                                                    } else {
                                                        break 'b_1d34e8;
                                                    }
                                                }
                                            }
                                            'b_1d3548: {
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x1;
                                                let t = c.g[S0][0] != c.g[V0][0];
                                                c.g[V0][0] = 0x2;
                                                if t {
                                                    break 'b_1d3548;
                                                } else {
                                                    'b_1d350c: {
                                                        'b_1d3508: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] =
                                                                (c.g[V1][0] < 0x4u64) as u64;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[V1][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x6c));
                                                            if t {
                                                                break 'b_1d3508;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0xa8));
                                                                if t {
                                                                    break 'b_1d350c;
                                                                } else {
                                                                    break 'b_1d3508;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 1;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0xac));
                                                        break 'b_1d350c;
                                                    }
                                                    c.cycles += 15;
                                                    c.g[A0][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add(c.g[S4][0] as u32));
                                                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x7c),
                                                        c.g[A1][0],
                                                    );
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x78),
                                                        c.g[S2][0],
                                                    );
                                                    c.g[A0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                                    c.g[S4][0] =
                                                        sx((c.g[S4][0] as u32).wrapping_add(0x4));
                                                    c.g[A1][0] = 0x1;
                                                    c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                                    c.g[V0][0] =
                                                        c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x40),
                                                        c.g[A0][0],
                                                    );
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x4),
                                                        c.g[V0][0],
                                                    );
                                                    if t {
                                                        break 'b_1d3658;
                                                    } else {
                                                        break 'b_1d3548;
                                                    }
                                                }
                                            }
                                            'b_1d35a4: {
                                                c.cycles += 2;
                                                let t = c.g[S0][0] != c.g[V0][0];
                                                c.g[V0][0] = 0x3;
                                                if t {
                                                    break 'b_1d35a4;
                                                } else {
                                                    'b_1d3568: {
                                                        'b_1d3564: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] =
                                                                (c.g[V1][0] < 0x4u64) as u64;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[V1][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x90));
                                                            if t {
                                                                break 'b_1d3564;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0xa8));
                                                                if t {
                                                                    break 'b_1d3568;
                                                                } else {
                                                                    break 'b_1d3564;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 1;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0xac));
                                                        break 'b_1d3568;
                                                    }
                                                    c.cycles += 15;
                                                    c.g[A0][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add(c.g[S5][0] as u32));
                                                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x7c),
                                                        c.g[A1][0],
                                                    );
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x78),
                                                        c.g[S2][0],
                                                    );
                                                    c.g[A0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x3c));
                                                    c.g[S5][0] =
                                                        sx((c.g[S5][0] as u32).wrapping_add(0x4));
                                                    c.g[A1][0] = 0x1;
                                                    c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                                    c.g[V0][0] =
                                                        c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x3c),
                                                        c.g[A0][0],
                                                    );
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x8),
                                                        c.g[V0][0],
                                                    );
                                                    if t {
                                                        break 'b_1d3658;
                                                    } else {
                                                        break 'b_1d35a4;
                                                    }
                                                }
                                            }
                                            'b_1d3600: {
                                                c.cycles += 2;
                                                let t = c.g[S0][0] != c.g[V0][0];
                                                c.g[V0][0] = 0x4;
                                                if t {
                                                    break 'b_1d3600;
                                                } else {
                                                    'b_1d35c4: {
                                                        'b_1d35c0: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] =
                                                                (c.g[V1][0] < 0x4u64) as u64;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[V1][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x9c));
                                                            if t {
                                                                break 'b_1d35c0;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0xa8));
                                                                if t {
                                                                    break 'b_1d35c4;
                                                                } else {
                                                                    break 'b_1d35c0;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 1;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0xac));
                                                        break 'b_1d35c4;
                                                    }
                                                    c.cycles += 15;
                                                    c.g[A0][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add(c.g[S6][0] as u32));
                                                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x7c),
                                                        c.g[A1][0],
                                                    );
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x78),
                                                        c.g[S2][0],
                                                    );
                                                    c.g[A0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[S6][0] =
                                                        sx((c.g[S6][0] as u32).wrapping_add(0x4));
                                                    c.g[A1][0] = 0x1;
                                                    c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                                    c.g[V0][0] =
                                                        c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x38),
                                                        c.g[A0][0],
                                                    );
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0xc),
                                                        c.g[V0][0],
                                                    );
                                                    if t {
                                                        break 'b_1d3658;
                                                    } else {
                                                        break 'b_1d3600;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = c.g[S0][0] != c.g[V0][0];
                                            c.g[V0][0] = 0x5;
                                            if t {
                                                break 'b_1d3674;
                                            } else {
                                                'b_1d3620: {
                                                    'b_1d361c: {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = (c.g[V1][0] < 0x4u64) as u64;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V1][0] =
                                                            sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x78));
                                                        if t {
                                                            break 'b_1d361c;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0xa8));
                                                            if t {
                                                                break 'b_1d3620;
                                                            } else {
                                                                break 'b_1d361c;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 1;
                                                    c.g[A1][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0xac));
                                                    break 'b_1d3620;
                                                }
                                                c.cycles += 14;
                                                c.g[A0][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[S7][0] as u32));
                                                c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                                c.sw(
                                                    (c.g[V0][0] as u32).wrapping_add(0x7c),
                                                    c.g[A1][0],
                                                );
                                                c.sw(
                                                    (c.g[V0][0] as u32).wrapping_add(0x78),
                                                    c.g[S2][0],
                                                );
                                                c.g[A0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                                c.g[S7][0] =
                                                    sx((c.g[S7][0] as u32).wrapping_add(0x4));
                                                c.g[A1][0] = 0x1;
                                                c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x34),
                                                    c.g[A0][0],
                                                );
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x10),
                                                    c.g[V0][0],
                                                );
                                                break 'b_1d3658;
                                            }
                                        }
                                        c.cycles += 7;
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001d366c);
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001d366c)?;
                                        let t = true;
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                        if t {
                                            break 'b_1d36e4;
                                        } else {
                                            break 'b_1d3674;
                                        }
                                    }
                                    c.cycles += 2;
                                    if c.g[S0][0] != c.g[V0][0] {
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                        break 'b_1d36e4;
                                    } else {
                                        'b_1d3694: {
                                            'b_1d3690: {
                                                c.cycles += 3;
                                                c.g[V0][0] = (c.g[V1][0] < 0x4u64) as u64;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V1][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x84));
                                                if t {
                                                    break 'b_1d3690;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[A1][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0xa8));
                                                    if t {
                                                        break 'b_1d3694;
                                                    } else {
                                                        break 'b_1d3690;
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[A1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0xac));
                                            break 'b_1d3694;
                                        }
                                        c.cycles += 20;
                                        c.g[A0][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[FP][0] as u32));
                                        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x78), c.g[S2][0]);
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                        c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x4));
                                        c.g[A1][0] = 0x1;
                                        c.g[V0][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[A0][0]);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001d36e0);
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001d36e0)?;
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                        break 'b_1d36e4;
                                    }
                                }
                                c.cycles += 4;
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = ((c.g[V1][0] as i64) < 5i64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                                if t {
                                    break 'b_1d372c;
                                } else {
                                    c.cycles += 14;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                    c.g[V1][0] = 0xa;
                                    c.g[V0][0] = sx((c.g[A0][0] as u32) << 1);
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                    c.mult(0, 3, 4, 3);
                                    c.g[A0][0] = 0x460000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5528));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    c.g[S0][0] = c.lb(c.g[V0][0] as u32);
                                    if (c.g[S0][0] as i64) >= 0 {
                                        c.g[A0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                                        if event_due(c) {
                                            deliver_event_here(c, 0x001d3478);
                                        }
                                        continue 'l_1d3478;
                                    } else {
                                        break 'b_1d372c;
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 4i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x001d33e8);
                    }
                    continue 'l_1d33e8;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001d3390);
                        }
                        continue 'l_1d3390;
                    } else {
                        c.cycles += 12;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

/// `cFEStatePeakRoom_OnWidgetCreated(this, widget)` (0x001d38f8): a screen widget was just created; its name hash is matched against the known names (title / button texts by `this+0xe0`, peak page widgets, event markers from the name tables at 0x4415b8..) and the widget is stored in the matching slot of `this`.
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias). Every register,
/// cycle and call of the generated body is kept.
pub fn c_fe_state_peak_room_on_widget_created(c: &mut Cpu) {
    if c.pc != 0x001d38f8 {
        return ee::s04::f_001d38f8__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x001d38f8;
        loop {
            match pc {
                0x001d38f8 => {
                    c.cycles += 19;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S0][0] = c.g[A1][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[A0][0] = 0x460000;
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e60));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[RA][0] = sx(0x001d393c);
                    c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                    call_ptr(c, 0x00317670, 0x001d393c)?;
                    let t = c.g[S2][0] != c.g[V0][0];
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        pc = 0x001d39a4;
                        continue;
                    }
                    pc = 0x001d3944;
                    continue;
                }
                0x001d3944 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe0));
                    c.g[V0][0] = 0x2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x001d3960;
                        continue;
                    }
                    pc = 0x001d3954;
                    continue;
                }
                0x001d3954 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7658));
                    if t {
                        pc = 0x001d3970;
                        continue;
                    }
                    pc = 0x001d3960;
                    continue;
                }
                0x001d3960 => {
                    c.cycles += 2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001d398c;
                        continue;
                    }
                    pc = 0x001d3968;
                    continue;
                }
                0x001d3968 => {
                    c.cycles += 2;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7668));
                    pc = 0x001d3970;
                    continue;
                }
                0x001d3970 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x001d3978);
                    call_ptr(c, 0x00317670, 0x001d3978)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001d3984);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a1000, 0x001d3984)?;
                    let t = true;
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        pc = 0x001d39a4;
                        continue;
                    }
                    pc = 0x001d398c;
                    continue;
                }
                0x001d398c => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x001d3994);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7678));
                    call_ptr(c, 0x00317670, 0x001d3994)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001d39a0);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a1000, 0x001d39a0)?;
                    c.g[A0][0] = 0x4a0000;
                    pc = 0x001d39a4;
                    continue;
                }
                0x001d39a4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d39ac);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1fa8));
                    call_ptr(c, 0x00317670, 0x001d39ac)?;

                    if c.g[S2][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        pc = 0x001d39c0;
                        continue;
                    }
                    pc = 0x001d39b4;
                    continue;
                }
                0x001d39b4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xd4), c.g[V0][0]);
                    if t {
                        pc = 0x001d3c2c;
                        continue;
                    }
                    pc = 0x001d39c0;
                    continue;
                }
                0x001d39c0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d39c8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1fb0));
                    call_ptr(c, 0x00317670, 0x001d39c8)?;

                    if c.g[S2][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001d39dc;
                        continue;
                    }
                    pc = 0x001d39d0;
                    continue;
                }
                0x001d39d0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xd8), c.g[V0][0]);
                    if t {
                        pc = 0x001d3c2c;
                        continue;
                    }
                    pc = 0x001d39dc;
                    continue;
                }
                0x001d39dc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d39e4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x488));
                    call_ptr(c, 0x00317670, 0x001d39e4)?;
                    let t = c.g[S2][0] != c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x001d3abc;
                        continue;
                    }
                    pc = 0x001d39ec;
                    continue;
                }
                0x001d39ec => {
                    c.cycles += 8;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xd0), c.g[S0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001d3a04);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001d3a04)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    if t {
                        pc = 0x001d3c30;
                        continue;
                    }
                    pc = 0x001d3a0c;
                    continue;
                }
                0x001d3a0c => {
                    c.cycles += 15;
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe0));
                    c.g[A2][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x001d3a1c);
                    c.g[A0][0] = c.g[S7][0];
                    call_ptr(c, 0x00156a90, 0x001d3a1c)?;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[RA][0] = sx(0x001d3a28);
                    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x00317670, 0x001d3a28)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                    c.g[RA][0] = sx(0x001d3a34);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00398380, 0x001d3a34)?;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[A0][0] = 0xffffffffffffffff;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x7c), c.g[V0][0]);
                    let t = true;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x78), c.g[A0][0]);
                    if t {
                        pc = 0x001d3c2c;
                        continue;
                    }
                    pc = 0x001d3a48;
                    continue;
                }
                0x001d3a48 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S5][0]);
                    if t {
                        pc = 0x001d3c2c;
                        continue;
                    }
                    pc = 0x001d3a50;
                    continue;
                }
                0x001d3a50 => {
                    c.cycles += 3;
                    c.g[A1][0] = 0u64;
                    let t = true;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c), c.g[S0][0]);
                    if t {
                        pc = 0x001d3aa0;
                        continue;
                    }
                    pc = 0x001d3a5c;
                    continue;
                }
                0x001d3a5c => {
                    c.cycles += 3;
                    c.g[A1][0] = 0u64;
                    let t = true;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x58), c.g[S0][0]);
                    if t {
                        pc = 0x001d3aa0;
                        continue;
                    }
                    pc = 0x001d3a68;
                    continue;
                }
                0x001d3a68 => {
                    c.cycles += 3;
                    c.g[A1][0] = 0u64;
                    let t = true;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x80), c.g[S0][0]);
                    if t {
                        pc = 0x001d3aa0;
                        continue;
                    }
                    pc = 0x001d3a74;
                    continue;
                }
                0x001d3a74 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[A1][0] = 0u64;
                    let t = true;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x94), c.g[S0][0]);
                    if t {
                        pc = 0x001d3aa0;
                        continue;
                    }
                    pc = 0x001d3a84;
                    continue;
                }
                0x001d3a84 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[A1][0] = 0u64;
                    let t = true;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xa8), c.g[S0][0]);
                    if t {
                        pc = 0x001d3aa0;
                        continue;
                    }
                    pc = 0x001d3a94;
                    continue;
                }
                0x001d3a94 => {
                    c.cycles += 3;
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[A1][0] = 0u64;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xbc), c.g[S0][0]);
                    pc = 0x001d3aa0;
                    continue;
                }
                0x001d3aa0 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001d3ab4);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001d3ab4)?;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S4][0]);
                    if t {
                        pc = 0x001d3c2c;
                        continue;
                    }
                    pc = 0x001d3abc;
                    continue;
                }
                0x001d3abc => {
                    c.cycles += 11;
                    c.g[RA][0] = sx(0x001d3ac4);
                    c.g[S5][0] = 0u64;
                    call_ptr(c, 0x0014dc80, 0x001d3ac4)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001d3ad0);
                    c.g[A1][0] = 0xd;
                    call_ptr(c, 0x0014dd58, 0x001d3ad0)?;
                    c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x48));
                    c.g[S7][0] = c.g[V0][0];
                    c.g[V1][0] = 0x440000;
                    c.g[V0][0] = 0x440000;
                    c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x15e0));
                    c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_add(0x15f0));
                    pc = 0x001d3ae8;
                    continue;
                }
                0x001d3ae8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d3af0);
                    c.g[A0][0] = c.lw(c.g[S6][0] as u32);
                    call_ptr(c, 0x00317670, 0x001d3af0)?;
                    let t = c.g[S2][0] != c.g[V0][0];
                    c.g[A0][0] = c.g[S7][0];
                    if t {
                        pc = 0x001d3b38;
                        continue;
                    }
                    pc = 0x001d3af8;
                    continue;
                }
                0x001d3af8 => {
                    c.cycles += 8;
                    c.sw(c.g[S1][0] as u32, c.g[S0][0]);
                    c.g[T0][0] = c.g[S5][0];
                    c.g[A1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x44));
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xdc));
                    c.g[RA][0] = sx(0x001d3b10);
                    c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe0));
                    call_ptr(c, 0x00157bf0, 0x001d3b10)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        if back_edge_unwinds(c, 0x001d3a0c) {
                            return Ok(());
                        }
                        pc = 0x001d3a0c;
                        continue;
                    }
                    pc = 0x001d3b18;
                    continue;
                }
                0x001d3b18 => {
                    c.cycles += 8;
                    c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001d3b30);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001d3b30)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    if t {
                        pc = 0x001d3c30;
                        continue;
                    }
                    pc = 0x001d3b38;
                    continue;
                }
                0x001d3b38 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d3b40);
                    c.g[A0][0] = c.lw(c.g[S4][0] as u32);
                    call_ptr(c, 0x00317670, 0x001d3b40)?;
                    let t = c.g[S2][0] == c.g[V0][0];
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x001d3a48) {
                            return Ok(());
                        }
                        pc = 0x001d3a48;
                        continue;
                    }
                    pc = 0x001d3b48;
                    continue;
                }
                0x001d3b48 => {
                    c.cycles += 5;
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[S5][0] as i64) < 4i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x001d3ae8) {
                            return Ok(());
                        }
                        pc = 0x001d3ae8;
                        continue;
                    }
                    pc = 0x001d3b5c;
                    continue;
                }
                0x001d3b5c => {
                    c.cycles += 15;
                    c.g[V1][0] = 0x440000;
                    c.g[V0][0] = 0x440000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x15b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x15a0));
                    c.g[A3][0] = 0x440000;
                    c.g[A2][0] = 0x440000;
                    c.g[A1][0] = 0x440000;
                    c.g[A0][0] = 0x440000;
                    c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                    c.g[FP][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1558));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    c.g[S7][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1540));
                    c.g[S6][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1570));
                    c.g[S5][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1588));
                    c.g[S4][0] = 0u64;
                    pc = 0x001d3b98;
                    continue;
                }
                0x001d3b98 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(c.g[S5][0] as u32);
                    c.g[RA][0] = sx(0x001d3ba4);
                    c.g[S1][0] = sx((c.g[S4][0] as u32) << 2);
                    call_ptr(c, 0x00317670, 0x001d3ba4)?;
                    let t = c.g[S2][0] == c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x001d3a50) {
                            return Ok(());
                        }
                        pc = 0x001d3a50;
                        continue;
                    }
                    pc = 0x001d3bac;
                    continue;
                }
                0x001d3bac => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d3bb4);
                    c.g[A0][0] = c.lw(c.g[S6][0] as u32);
                    call_ptr(c, 0x00317670, 0x001d3bb4)?;
                    let t = c.g[S2][0] == c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x001d3a5c) {
                            return Ok(());
                        }
                        pc = 0x001d3a5c;
                        continue;
                    }
                    pc = 0x001d3bbc;
                    continue;
                }
                0x001d3bbc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d3bc4);
                    c.g[A0][0] = c.lw(c.g[S7][0] as u32);
                    call_ptr(c, 0x00317670, 0x001d3bc4)?;
                    let t = c.g[S2][0] == c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x001d3a68) {
                            return Ok(());
                        }
                        pc = 0x001d3a68;
                        continue;
                    }
                    pc = 0x001d3bcc;
                    continue;
                }
                0x001d3bcc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d3bd4);
                    c.g[A0][0] = c.lw(c.g[FP][0] as u32);
                    call_ptr(c, 0x00317670, 0x001d3bd4)?;
                    let t = c.g[S2][0] == c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x001d3a74) {
                            return Ok(());
                        }
                        pc = 0x001d3a74;
                        continue;
                    }
                    pc = 0x001d3bdc;
                    continue;
                }
                0x001d3bdc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d3be4);
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    call_ptr(c, 0x00317670, 0x001d3be4)?;
                    let t = c.g[S2][0] == c.g[V0][0];
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    if t {
                        if back_edge_unwinds(c, 0x001d3a84) {
                            return Ok(());
                        }
                        pc = 0x001d3a84;
                        continue;
                    }
                    pc = 0x001d3bec;
                    continue;
                }
                0x001d3bec => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d3bf4);
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    call_ptr(c, 0x00317670, 0x001d3bf4)?;
                    let t = c.g[S2][0] == c.g[V0][0];
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    if t {
                        if back_edge_unwinds(c, 0x001d3a94) {
                            return Ok(());
                        }
                        pc = 0x001d3a94;
                        continue;
                    }
                    pc = 0x001d3bfc;
                    continue;
                }
                0x001d3bfc => {
                    c.cycles += 12;
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x4));
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x4));
                    c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[V0][0] = ((c.g[S4][0] as i64) < 5i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x001d3b98) {
                            return Ok(());
                        }
                        pc = 0x001d3b98;
                        continue;
                    }
                    pc = 0x001d3c2c;
                    continue;
                }
                0x001d3c2c => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    pc = 0x001d3c30;
                    continue;
                }
                0x001d3c30 => {
                    c.cycles += 11;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x001d38f8, 0x001d3c5c);
                    return Ok(());
                }
            }
        }
    });
}

/// `lw` of the world pointer at `gp - 0x848`.
pub(crate) fn fe_state_world(c: &mut Cpu) -> u64 {
    c.lw((c.g[GP][0] as u32).wrapping_sub(0x848))
}

/// The slot-setup block shared by the three `cFEStateMountainRoom_Init`
/// blocks: fetch the slot's pointer (0x317670), look its object up (0x39d860)
/// and, when there is one, run the slot's init (0x3a0c50). `self_pc` is the
/// block's own return address.
pub(crate) fn mr_init_slot(
    c: &mut Cpu,
    a0_off: u32,
    ret_fetch: u32,
    ret_find: u32,
    a1_off: u32,
    self_pc: u32,
) -> Flow {
    c.cycles += 8;
    c.g[A0][0] = sx(0x004a_0000u32.wrapping_add(a0_off));
    call_ptr(c, 0x0031_7670, ret_fetch)?;
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, ret_find)?;
    let found = c.g[V0][0] != 0;
    c.g[A1][0] = 0x004a_0000;
    if found {
        c.cycles += 3;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(a1_off));
        call_ptr(c, 0x003a_0c50, self_pc)?;
    }
    Ok(())
}

/// `void cFEStateMountainRoom_Init(self)` (0x1d2990): store the room's
/// texture object (0x39 7cc0) and its first load, then run the three slot
/// setups and the two finishing calls (0x28b180, 0x28f140).
pub fn cfe_state_mountain_room_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x7228));
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x0031_7670, 0x001d_29b4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001d_29c8)?;
        let stored = c.g[V0][0] != 0;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        if stored {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001d_29e0)?;
        }
        mr_init_slot(c, 0x1f08, 0x001d_29ec, 0x001d_29f8, 0x14f8, 0x001d_2a0c)?;
        mr_init_slot(c, 0x1f10, 0x001d_2a18, 0x001d_2a24, 0x1500, 0x001d_2a38)?;
        mr_init_slot(c, 0x1f18, 0x001d_2a44, 0x001d_2a50, 0x1508, 0x001d_2a64)?;
        c.cycles += 10;
        call_ptr(c, 0x0028_b180, 0x001d_2a6c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 7;
        call_ptr(c, 0x0028_f140, 0x001d_2a78)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The `'b_1d3284` and `'b_1d32a4` blocks of `cFEStateMountainRoom_PollImageLoad`:
/// the chain that leaves a0 = the bit (`(x >> 6) & 1`) of the first nested
/// object's field, or 0 when a link is missing.
pub(crate) fn mr_image_bit_chain(c: &mut Cpu) {
    c.cycles += 2;
    let v1_missing = c.g[V1][0] == 0;
    c.g[V0][0] = 0;
    if !v1_missing {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
        c.g[V0][0] &= 1;
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
            c.g[A0][0] = c.g[V0][0] & 1;
        }
    }
}

/// The `'b_1d3268` block of `cFEStateMountainRoom_PollImageLoad`: ask 0x19e2b0
/// for the item's entry, start the texture load through the render vtable,
/// store the texture id in the room and the object, and set a0 for the chain.
pub(crate) fn mr_image_state(c: &mut Cpu) -> Flow {
    c.cycles += 23;
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xb0));
    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
    call_ptr(c, 0x0019_e2b0, 0x001d_320c)?;
    c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
    c.g[A2][0] = sx(0x004a_1f40);
    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
    c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
    c.g[A3][0] = 0;
    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[T0][0] = 1;
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
    c.g[T1][0] = 0xffff_ffff_ffff_ffff;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
    let method = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, 0x001d_3240)?;
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
    c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
    c.sw((c.g[V1][0] as u32).wrapping_add(0x78), c.g[V0][0]);
    c.sw((c.g[V1][0] as u32).wrapping_add(0x7c), 0);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
    let missing = c.g[V1][0] == 0;
    c.g[A0][0] = 0;
    if !missing {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
        c.g[A0][0] = c.g[V0][0] & 1;
    }
    Ok(())
}

/// The tail of `cFEStateMountainRoom_PollImageLoad` after the state block:
/// the `'b_1d32b8` block (show the image when the bit chain says so, else
/// keep the request), then the bolt release (0x19dc20) and the handle reset.
pub(crate) fn mr_image_tail(c: &mut Cpu) -> Flow {
    // 'b_1d32b4 is entered when a0 (the request flag) is non-zero.
    c.cycles += 2;
    let a0_zero = c.g[A0][0] == 0;
    c.g[A0][0] = 0;
    if !a0_zero {
        mr_image_bit_chain(c);
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xb0));
            return mr_image_release(c);
        }
        c.cycles += 2;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_32e0, 0x001d_32b4)?;
    }
    c.cycles += 1;
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xb0));
    mr_image_release(c)
}

/// The final call of `cFEStateMountainRoom_PollImageLoad`: release the bolt
/// bank of the room (0x19dc20) and set its handle to -1.
pub(crate) fn mr_image_release(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
    call_ptr(c, 0x0019_dc20, 0x001d_32c4)?;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
    Ok(())
}

/// `void cFEStateMountainRoom_PollImageLoad(self)` (0x1d31c0): the room's
/// counterpart of the preview poll: when a request is pending and the bolt
/// manager reports it done, start the texture and attach it; a not-yet-done
/// request returns at once.
pub fn cfe_state_mountain_room_poll_image_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = fe_state_world(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
        let idle = c.g[A0][0] == 0;
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1a70));
        if !idle {
            c.cycles += 6;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xb0));
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
            call_ptr(c, 0x0019_e238, 0x001d_31f8)?;
            let ready = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S1][0];
            if ready {
                mr_image_state(c)?;
                mr_image_tail(c)?;
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

/// `void cFEStatePeakRoom_ctor(self, kind, ..., ...)` (0x1d3780): set the
/// room's type fields, load its textures (0x1a1cd0, 0x14dc80, 0x14dd58,
/// 0x147488) and clear the three slot tables with the two loops (each slot
/// word and the last field are zeroed).
pub fn cfe_state_peak_room_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = sx(((c.g[A2][0] as u32) << 24) as u32);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
        call_ptr(c, 0x0039_e2a0, 0x001d_37a8)?;
        c.g[V0][0] = 0x2c;
        c.g[V1][0] = sx(0x0047_0000u32.wrapping_sub(0x6878));
        c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = c.g[S0][0];
        c.sb((c.g[S2][0] as u32).wrapping_add(0x44), c.g[S0][0]);
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
        call_ptr(c, 0x001a_1cd0, 0x001d_37d0)?;
        c.sb((c.g[S2][0] as u32).wrapping_add(0x15), c.g[V0][0]);
        call_ptr(c, 0x0014_dc80, 0x001d_37d8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x001d_37e4)?;
        c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_7488, 0x001d_37f0)?;
        c.sw((c.g[S2][0] as u32).wrapping_add(0xdc), c.g[V0][0]);
        c.g[V1][0] = 3;
        c.sw((c.g[S2][0] as u32).wrapping_add(0xe0), c.g[S1][0]);
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x54));
        loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_3800);
            }
        }
        c.cycles += 7;
        c.g[A3][0] = sx((c.g[S2][0] as u32).wrapping_add(0xbc));
        c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xa8));
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x94));
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6c));
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x58));
        c.g[T0][0] = 4;
        loop {
            c.cycles += 14;
            c.sw(c.g[V0][0] as u32, 0);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(1));
            c.sw(c.g[V1][0] as u32, 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
            c.sw(c.g[A0][0] as u32, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            c.sw(c.g[A1][0] as u32, 0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            c.sw(c.g[A2][0] as u32, 0);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
            c.sw(c.g[A3][0] as u32, 0);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
            let more = (c.g[T0][0] as i64) >= 0;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_3838);
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0xd0), 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStatePeakRoom_OnActivate(self)` (0x1d3c60): a forwarding
/// wrapper around 0x39e4c0.
pub fn cfe_state_peak_room_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0039_e4c0, 0x001d_3c70)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStatePeakRoom_RefreshGoalList(sRoom* self, int peak)` (0x001d3f80): hides the six widget rows of the five goal slots (icon, box, name, bar, medal, lock), then for each challenge of the peak shows its name, bar and box and either the unlocked icon (texture looked up through the reward entry's hash) with the medal, or the lock.
/// Reproduces every register, cycle and call of the generated body; `pc != entry`
/// (a resume after an unwound callee) runs the generated body.
pub fn c_fe_state_peak_room_refresh_goal_list(c: &mut Cpu) {
    if c.pc != 0x001d3f80 {
        return ee::s04::f_001d3f80__generated(c);
    }
    guest(c, |c| {
        c.cycles += 26;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S1][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c));
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x80));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S2][0] = 0u64;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0xbc));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0xa8));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x94));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[S6][0] = sx((c.g[S1][0] as u32).wrapping_add(0x58));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.sw(c.g[SP][0] as u32, c.g[A1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[RA][0] = sx(0x001d3fe0);
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe0));
        Callee {
            addr: 0x001ce6f0,
            f: ee::s04::cFEStateTrophyRoom_CountEventEntries,
        }
        .call(c, 0x001d3fe0)?;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
        'l_1d3fe8: loop {
            c.cycles += 52;
            c.g[A1][0] = 0u64;
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
            oc943_vcall(c, A2, 0x48, 0x001d400c)?;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A1][0] = 0u64;
            c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c));
            oc943_vcall(c, A2, 0x48, 0x001d402c)?;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A1][0] = 0u64;
            c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x80));
            oc943_vcall(c, A2, 0x48, 0x001d404c)?;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A1][0] = 0u64;
            c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x94));
            oc943_vcall(c, A2, 0x48, 0x001d406c)?;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A1][0] = 0u64;
            c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa8));
            oc943_vcall(c, A2, 0x48, 0x001d408c)?;
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A1][0] = 0u64;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
            oc943_vcall(c, A2, 0x48, 0x001d40ac)?;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 5i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001d3fe8);
                }
                continue 'l_1d3fe8;
            } else {
                'b_1d4238: {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x001d40c0);
                    c.g[S2][0] = 0u64;
                    call_ptr(c, 0x0014dc80, 0x001d40c0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001d40cc);
                    c.g[A1][0] = 0xd;
                    call_ptr(c, 0x0014dd58, 0x001d40cc)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S7][0] = c.g[S4][0];
                    if t {
                        break 'b_1d4238;
                    } else {
                        c.cycles += 5;
                        c.g[FP][0] = c.g[S3][0];
                        c.g[S4][0] = c.g[S6][0];
                        c.g[S3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[S6][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        'l_1d40f0: loop {
                            'b_1d4214: {
                                'b_1d41f4: {
                                    c.cycles += 36;
                                    c.g[A3][0] = c.lw(c.g[SP][0] as u32);
                                    c.g[T0][0] = c.g[S2][0];
                                    c.g[A0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xdc));
                                    c.g[RA][0] = sx(0x001d4108);
                                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe0));
                                    Callee {
                                        addr: 0x001ced90,
                                        f: ee::s04::cFEStateTrophyRoom_QueryEventReward,
                                    }
                                    .call(c, 0x001d4108)?;
                                    c.g[A2][0] = c.lw(c.g[SP][0] as u32);
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                                    c.g[A3][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x001d4120);
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe0));
                                    Callee {
                                        addr: 0x001ce758,
                                        f: ee::s04::cFEStateTrophyRoom_SetEventName,
                                    }
                                    .call(c, 0x001d4120)?;
                                    c.g[A2][0] = c.lw(c.g[S3][0] as u32);
                                    c.g[A1][0] = 0x1;
                                    oc943_vcall(c, A2, 0x48, 0x001d413c)?;
                                    c.g[A2][0] = c.lw(c.g[S5][0] as u32);
                                    c.g[A1][0] = 0x1;
                                    oc943_vcall(c, A2, 0x48, 0x001d4158)?;
                                    c.g[A2][0] = c.lw(c.g[S6][0] as u32);
                                    c.g[A1][0] = 0x1;
                                    oc943_vcall(c, A2, 0x48, 0x001d4174)?;
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    let t = c.g[S0][0] == c.g[V1][0];
                                    c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                                    if t {
                                        break 'b_1d41f4;
                                    } else {
                                        c.cycles += 29;
                                        c.g[A2][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001d418c);
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                        call_ptr(c, 0x00156ae0, 0x001d418c)?;
                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[RA][0] = sx(0x001d4198);
                                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                        call_ptr(c, 0x00317670, 0x001d4198)?;
                                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                                        c.g[RA][0] = sx(0x001d41a4);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x00398380, 0x001d41a4)?;
                                        c.g[V1][0] = c.lw(c.g[S4][0] as u32);
                                        c.g[A1][0] = 0x1;
                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x7c), c.g[V0][0]);
                                        c.g[V0][0] = 0xffffffffffffffff;
                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                                        c.g[A2][0] = c.lw(c.g[S4][0] as u32);
                                        oc943_vcall(c, A2, 0x48, 0x001d41d0)?;
                                        c.g[A2][0] = c.lw(c.g[S7][0] as u32);
                                        c.g[A1][0] = 0x1;
                                        oc943_vcall(c, A2, 0x48, 0x001d41ec)?;
                                        let t = true;
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                        if t {
                                            break 'b_1d4214;
                                        } else {
                                            break 'b_1d41f4;
                                        }
                                    }
                                }
                                c.cycles += 8;
                                c.g[A2][0] = c.lw(c.g[FP][0] as u32);
                                c.g[A1][0] = 0x1;
                                oc943_vcall(c, A2, 0x48, 0x001d4210)?;
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                break 'b_1d4214;
                            }
                            c.cycles += 9;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x4));
                            c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                            c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x4));
                            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x001d40f0);
                                }
                                continue 'l_1d40f0;
                            } else {
                                break 'b_1d4238;
                            }
                        }
                    }
                }
                c.cycles += 12;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
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
        }
    });
}

// ---------------------------------------------------------------------------
// cFEStatePeakRoom::OnWidgetAction (0x1d3c80, 388 bytes)
// ---------------------------------------------------------------------------

/// Where the widget-action body leaves to: the restores it still owes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum PeakTail {
    /// `cycles += 1; lq s0`, then `B`.
    A,
    /// `cycles += 1; lq s1`, then `C`.
    B,
    /// The epilogue (`lq s2, s3, ra`, `sp`).
    C,
}

pub(crate) type PeakFlow = Result<PeakTail, Unwound>;

/// The shared epilogue of the widget-action paths.
pub(crate) fn peak_finish(c: &mut Cpu, frame: Frame, from: PeakTail) {
    if from == PeakTail::A {
        c.cycles += 1;
        frame.restore(c, S0, 0x40);
    }
    if from != PeakTail::C {
        c.cycles += 1;
        frame.restore(c, S1, 0x30);
    }
    c.cycles += 5;
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void cFEStatePeakRoom::OnWidgetAction(self, widget, kind)` (0x1d3c80): with a
/// widget, the action `kind` picks the room's path: 5 opens the popup (BE
/// lookup, the popup's virtual call, then the slot update), 1 refreshes the help
/// text and goal list, 6 walks the four goal entries of `self+0x48`, any other
/// kind leaves at once. `self` lives in s1, the widget in s2.
pub fn cfe_state_peak_room_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        let tail = if c.g[S2][0] == 0 {
            PeakTail::A
        } else {
            peak_action_body(c, frame)?
        };
        peak_finish(c, frame, tail);
        Ok(())
    })
}

/// Dispatch on the action kind (`a2`).
pub(crate) fn peak_action_body(c: &mut Cpu, frame: Frame) -> PeakFlow {
    c.cycles += 3;
    c.g[V0][0] = 5;
    let is_popup = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = (c.g[A2][0] < 6) as u64;
    if is_popup {
        return peak_popup_path(c, frame);
    }
    c.cycles += 2;
    let at_least_six = c.g[V0][0] == 0;
    c.g[V0][0] = 1;
    if at_least_six {
        c.cycles += 3;
        c.g[V0][0] = 6;
        let is_goals = c.g[A2][0] == c.g[V0][0];
        frame.restore(c, S0, 0x40);
        if is_goals {
            return peak_goal_path(c, frame);
        }
        c.cycles += 2;
        frame.restore(c, S1, 0x30);
        return Ok(PeakTail::C);
    }
    c.cycles += 2;
    let is_help = c.g[A2][0] == c.g[V0][0];
    frame.restore(c, S0, 0x40);
    if is_help {
        return peak_help_path(c);
    }
    c.cycles += 2;
    frame.restore(c, S1, 0x30);
    Ok(PeakTail::C)
}

/// Kind 5: the BE lookup and the popup object's virtual call, then the slot
/// update through `self+0x10`.
pub(crate) fn peak_popup_path(c: &mut Cpu, frame: Frame) -> PeakFlow {
    c.cycles += 13;
    call_ptr(c, 0x0014_dc80, 0x001d_3ce4)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xd;
    call_ptr(c, 0x0014_dd58, 0x001d_3cf0)?;
    c.g[A1][0] = oc945_lb(c, S1, 0x44);
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = oc945_lw(c, S1, 0xdc);
    c.g[A3][0] = oc945_lw(c, S1, 0xe0);
    c.g[T0][0] = oc945_lw(c, S2, 0x18);
    call_ptr(c, 0x0015_7bf0, 0x001d_3d08)?;
    let found = c.g[V0][0] != 0;
    c.g[A1][0] = c.g[S1][0];
    if !found {
        return Ok(PeakTail::A);
    }
    c.cycles += 13;
    c.g[V0][0] = oc945_lw(c, S1, 0x10);
    c.g[A2][0] = oc945_lw(c, S1, 0xe0);
    c.g[A3][0] = oc945_lw(c, V0, 0);
    c.g[A0][0] = oc945_lw(c, S2, 0x18);
    c.g[A2][0] = sx(r32(c, A2) << 16);
    c.g[V1][0] = oc945_lw(c, A3, 4);
    c.g[A2][0] = c.g[A2][0] | c.g[A0][0];
    c.g[A0][0] = oc945_lh(c, V1, 0x20);
    c.g[V0][0] = oc945_lw(c, V1, 0x24);
    let target = r32(c, V0);
    c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
    call_ptr(c, target, 0x001d_3d3c)?;
    c.g[A0][0] = oc945_lw(c, S1, 0x10);
    peak_slot_path(c, frame)
}

/// The shared tail of the popup and goal paths: `self+0x10`'s slot call
/// (`a1` = `v0`), then the epilogue through [`PeakTail::A`].
pub(crate) fn peak_slot_path(c: &mut Cpu, frame: Frame) -> PeakFlow {
    c.cycles += 5;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
    call_ptr(c, 0x0039_f400, 0x001d_3dc8)?;
    frame.restore(c, S0, 0x40);
    Ok(PeakTail::B)
}

/// Kind 1: refresh the help text and the goal list.
pub(crate) fn peak_help_path(c: &mut Cpu) -> PeakFlow {
    c.cycles += 6;
    c.g[A1][0] = oc945_lw(c, S2, 0x18);
    c.g[A0][0] = c.g[S1][0];
    Callee {
        addr: 0x001d_3ec8,
        f: ee::s04::f_001d3ec8,
    }
    .call(c, 0x001d_3ddc)?;
    c.g[A1][0] = oc945_lw(c, S2, 0x18);
    c.g[A0][0] = c.g[S1][0];
    Callee {
        addr: 0x001d_3f80,
        f: ee::s04::f_001d3f80,
    }
    .call(c, 0x001d_3de8)?;
    Ok(PeakTail::A)
}

/// Kind 6: the four goal entries at `self+0x48` (a loop that calls each
/// entry's virtual method), then the slot call on `self+0x10`.
pub(crate) fn peak_goal_path(c: &mut Cpu, frame: Frame) -> PeakFlow {
    c.cycles += 7;
    c.g[V0][0] = oc945_lw(c, S1, 0xe0);
    c.g[S0][0] = sx(r32(c, S1).wrapping_add(0x48));
    c.g[V1][0] = oc945_lw(c, S2, 0x18);
    c.g[V0][0] = sx(r32(c, V0) << 16);
    c.g[S2][0] = 3;
    c.g[S3][0] = c.g[V0][0] | c.g[V1][0];
    loop {
        c.cycles += 3;
        c.g[A2][0] = oc945_lw(c, S0, 0);
        let empty = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !empty {
            c.cycles += 5;
            oc945_vcall(c, A2, 0x001d_3d80)?;
        }
        c.cycles += 3;
        c.g[S2][0] = sx(r32(c, S2).wrapping_sub(1));
        let more = (c.g[S2][0] as i64) >= 0;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x001d_3d60);
        }
    }
    c.cycles += 11;
    c.g[V0][0] = oc945_lw(c, S1, 0x10);
    c.g[A2][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A3][0] = oc945_lw(c, V0, 0);
    c.g[V1][0] = oc945_lw(c, A3, 4);
    c.g[A0][0] = oc945_lh(c, V1, 0x28);
    c.g[V0][0] = oc945_lw(c, V1, 0x2c);
    let target = r32(c, V0);
    c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
    call_ptr(c, target, 0x001d_3db0)?;
    let no_result = c.g[V0][0] == 0;
    frame.restore(c, S0, 0x40);
    if no_result {
        return Ok(PeakTail::B);
    }
    c.cycles += 1;
    c.g[A0][0] = oc945_lw(c, S1, 0x10);
    peak_slot_path(c, frame)
}

// ---------------------------------------------------------------------------
// cFEStatePeakRoom::UpdateHelpText (0x1d3ec8, 184 bytes)
// ---------------------------------------------------------------------------

/// `void cFEStatePeakRoom::UpdateHelpText(self, id)` (0x1d3ec8): when the room
/// has its help object (`self+0xd0`), look up the text for `id` (the BE table
/// and two strings at 0x467688 and 0x4676a0) and hand it to the object's
/// virtual method.
pub fn cfe_state_peak_room_update_help_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = oc945_lw(c, S0, 0xd0);
        c.g[S1][0] = c.g[A1][0];
        if c.g[V0][0] != 0 {
            help_text_lookup(c)?;
            c.cycles += 6;
            c.g[A1][0] = 1;
            oc945_vcall(c, A2, 0x001d_3f6c)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The text lookup of [`cfe_state_peak_room_update_help_text`]: sets `a2` to the
/// help object.
pub(crate) fn help_text_lookup(c: &mut Cpu) -> Flow {
    c.cycles += 13;
    call_ptr(c, 0x0014_dc80, 0x001d_3ef0)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xd;
    call_ptr(c, 0x0014_dd58, 0x001d_3efc)?;
    c.g[A1][0] = oc945_lb(c, S0, 0x44);
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = oc945_lw(c, S0, 0xdc);
    c.g[T0][0] = c.g[S1][0];
    c.g[A3][0] = oc945_lw(c, S0, 0xe0);
    call_ptr(c, 0x0015_7bf0, 0x001d_3f14)?;
    if c.g[V0][0] == 0 {
        c.g[A0][0] = 0x0046_0000;
        c.cycles += 6;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x76a0));
        call_ptr(c, 0x0031_7670, 0x001d_3f44)?;
        c.g[A0][0] = oc945_lw(c, S0, 0xd0);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_1000, 0x001d_3f50)?;
        c.g[A2][0] = oc945_lw(c, S0, 0xd0);
    } else {
        c.cycles += 8;
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x7688));
        call_ptr(c, 0x0031_7670, 0x001d_3f28)?;
        c.g[A0][0] = oc945_lw(c, S0, 0xd0);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_1000, 0x001d_3f34)?;
        c.g[A2][0] = oc945_lw(c, S0, 0xd0);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// cFEStateTrophyRoom (0x1d4268 .. 0x1d4a20)
// ---------------------------------------------------------------------------

/// `cFEStateTrophyRoom::cFEStateTrophyRoom` (0x1d4268, 252 bytes): the UI state
/// base constructor, then the trophy widget's text fields, the eight slots at
/// `self+0x74` set to -1, and the trophy image request.
pub fn cfe_state_trophy_room_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = sx((c.g[A2][0] as u32) << 24);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A3][0];
        c.g[S2][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
        call_ptr(c, 0x0039_e2a0, 0x001d_4298)?;
        c.g[V0][0] = 0x2d;
        c.g[V1][0] = sx(0x0047_0000u32.wrapping_sub(0x6948));
        let a = oc945_at(c, S3, 0xc);
        c.sw(a, c.g[V0][0]);
        let a = oc945_at(c, S3, 0x8);
        c.sw(a, c.g[V1][0]);
        c.g[A1][0] = c.g[S0][0];
        let a = oc945_at(c, S3, 0x44);
        c.sb(a, c.g[S0][0]);
        c.g[V0][0] = oc945_lw(c, S3, 0x10);
        c.g[A0][0] = oc945_lw(c, V0, 0);
        call_ptr(c, 0x001a_1cd0, 0x001d_42c0)?;
        let a = oc945_at(c, S3, 0x48);
        c.sw(a, 0);
        let a = oc945_at(c, S3, 0x15);
        c.sb(a, c.g[V0][0]);
        call_ptr(c, 0x0014_dc80, 0x001d_42cc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x001d_42d8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = oc945_lb(c, S3, 0x44);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_7488, 0x001d_42e8)?;
        let a = oc945_at(c, S3, 0x50);
        c.sw(a, c.g[V0][0]);
        c.g[V0][0] = oc945_lw(c, S0, 0xc);
        c.g[A0][0] = oc945_lh(c, V0, 0x10);
        c.g[V1][0] = oc945_lw(c, V0, 0x14);
        let target = r32(c, V1);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x001d_4300)?;
        let a = oc945_at(c, S3, 0x54);
        c.sw(a, c.g[S1][0]);
        c.g[A0][0] = u64::MAX;
        let a = oc945_at(c, S3, 0x58);
        c.sw(a, c.g[S2][0]);
        c.g[V0][0] = sx(oc945_at(c, S3, 0x74));
        let a = oc945_at(c, S3, 0x5c);
        c.sw(a, 0);
        c.g[V1][0] = 4;
        loop {
            c.cycles += 7;
            let a = r32(c, V0).wrapping_sub(0x14);
            c.sw(a, c.g[A0][0]);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
            let a = r32(c, V0);
            c.sw(a, c.g[A0][0]);
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_4318);
            }
        }
        c.cycles += 12;
        c.g[V0][0] = u64::MAX;
        c.g[A0][0] = c.g[S3][0];
        let a = oc945_at(c, S3, 0x9c);
        c.sw(a, c.g[V0][0]);
        Callee {
            addr: 0x001d_4918,
            f: ee::s04::f_001d4918,
        }
        .call(c, 0x001d_4344)?;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::fe::festaterewards::tests::oc942_install;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::oc943_install_fakes;

    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::fe::debugfe::tests::w as w_2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::put_str_2 as put_str;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_3;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_3;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;

    pub(crate) use crate::game::fe::fepopupmisc::tests::setup;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base;

    pub(crate) use crate::game::fe::festaterewards::tests::slot_2;

    pub(crate) use crate::game::fe::festatebuyattrib::cfe_state_buy_attrib_on_widget_notify;
    pub(crate) use crate::game::fe::festatetrophyroom::cfe_state_rewards_room_on_widget_notify;
    pub(crate) use crate::game::fe::festatetrophyroom::mem_card_state_base_on_widget_notify;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::fe::ovstateenterlodge::ov_screen_enter_lodge_on_widget_notify;
    pub(crate) use crate::game::fe::ovtemplatedialog::ov_screen_yn_dialog_on_widget_notify;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup as setup_2;

    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS;

    #[test]
    fn fe_state_mountain_room_on_activate_matches() {
        check(
            ssx3_ee::s04::f_001d2a90__generated,
            fe_state_mountain_room_on_activate,
            40,
            &|c, seed| {
                machine(c, seed, 0x1d2a_90, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_mountain_room_update_matches() {
        check(
            ssx3_ee::s04::f_001d3120__generated,
            c_fe_state_mountain_room_update,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x25, 1);
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 0xbc, if seed % 2 == 0 { 0 } else { 0xffff_ffff });
            },
        );
    }

    #[test]
    fn cfe_state_mountain_room_request_image_matches() {
        check(
            ssx3_ee::s04::f_001d3160__generated,
            cfe_state_mountain_room_request_image,
            120,
            &|c, seed| {
                let _r = setup(c, seed, 0x1d3160);
                let this = DATA;
                c.g[A0][0] = this as u64;
                // Both cheap exits on some seeds, the request path on the rest.
                c.bus
                    .write32(this + 0xbc, if seed % 4 == 0 { 0 } else { 0xffff_ffff });
                c.bus
                    .write32(this + 0xc0, if seed % 3 == 0 { 0 } else { DATA + 0x300 });
                c.bus.write32(this + 0xb0, 0);
                // The world, its rider-0 manager, and slot 0 of the bolt-file
                // table already holding the requested item: `FindSlotByName`
                // matches (strcmp of the two equal names) and returns slot 0,
                // so the callee takes the refcount path instead of the
                // free-slot/evict/load walk.
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x800);
                c.bus.write32(DATA + 0x87c, DATA + 0x1000);
                c.bus.write32(DATA + 0x304, DATA + 0x100);
                put_str(c, DATA + 0x100, "bolt");
                put_str(c, DATA + 0x2a74, "bolt");
            },
        );
    }

    #[test]
    fn handle_list_key_family_matches() {
        check(
            ssx3_ee::s04::f_001d2f40__generated,
            cfe_state_mountain_room_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d2f40, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
        check(
            ssx3_ee::s04::f_001d4f68__generated,
            cfe_state_rewards_room_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d4f68, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
        check(
            ssx3_ee::s04::f_001d5460__generated,
            mem_card_state_base_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d5460, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
        check(
            ssx3_ee::s04::f_001f4a38__generated,
            cfe_state_buy_attrib_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x1f4a38, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
        check(
            ssx3_ee::s04::f_001f72b8__generated,
            ov_screen_enter_lodge_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x1f72b8, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
        check(
            ssx3_ee::s05::f_0020d568__generated,
            ov_screen_yn_dialog_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x20d568, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
    }

    /// A fake vtable page.
    pub(crate) const VT: u32 = DATA + 0x8000;

    /// A vtable slot `{ short delta; short; fn }` at `vt + off`.
    pub(crate) fn slot(c: &mut Cpu, vt: u32, off: u32, delta: i16) {
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, fakes::FAKE);
    }

    #[test]
    fn c_fe_state_mountain_room_show_image_matches() {
        check(
            ssx3_ee::s04::f_001d32e0__generated,
            c_fe_state_mountain_room_show_image,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x1d32e0, 0);
                w_2(c, OBJ + 0x48, OBJ2);
                w_2(c, OBJ + 0x4c, OBJ3);
                w_2(c, OBJ2 + 0x8, VT);
                w_2(c, OBJ3 + 0x8, VT);
                slot(c, VT, 0x48, 0);
                w_2(
                    c,
                    OBJ + 0xc4,
                    if seed % 2 == 0 { 0xffff_ffff } else { seed },
                );
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_peak_room_init_matches() {
        check(
            ssx3_ee::s04::f_001d3890__generated,
            c_fe_state_peak_room_init,
            40,
            &|c, seed| {
                let _r = setup_2(c, seed, 0x1d3890);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }

    #[test]
    fn cfe_state_mountain_room_dtor_matches() {
        check(
            ee::s04::f_001d28f8__generated,
            cfe_state_mountain_room_dtor,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x1d28f8, 3);
                let obj = DATA + 0x1000;
                let world = DATA + 0x8_0000;
                let game = world + 0x1000;
                let reg = DATA + 0x2_0000;
                let vt = DATA + 0x3_0000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(
                    c,
                    obj + 0xbc,
                    if seed % 2 == 0 {
                        0xffff_ffff
                    } else {
                        (seed % 8) as u32
                    },
                );
                w(c, obj + 0xb0, r.next_u32());
                w(
                    c,
                    obj + 0xc4,
                    if seed % 3 == 0 {
                        0xffff_ffff
                    } else {
                        (seed % 8) as u32
                    },
                );
                w(c, world + 0x7c, game);
                w(c, GAME_GP.wrapping_sub(0x854), reg);
                w(c, reg + 0x10d8, vt);
                slot_2(c, vt, 0x190, 0, FAKE);
            },
        );
    }

    #[test]
    fn cfe_state_mountain_room_handle_input_matches() {
        check(
            ssx3_ee::s04::f_001d2ea0__generated,
            cfe_state_mountain_room_handle_input,
            20,
            &|c, seed| {
                let _r = base_2(c, seed, 0x1d2ea0, 2);
                let this = DATA + 0x1000;
                let input = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = input as u64;
                let widget = DATA + 0x3000;
                w_3(c, this + 0x48, if seed % 3 == 0 { 0 } else { widget });
                w_3(c, widget + 0x14, if seed % 2 == 0 { 0 } else { 1 << 6 });
                w_3(c, input + 8, DATA + 0x4000);
                let zero_fn = ssx3_ee::s09::f_002cc098 as fn(&mut Cpu) as usize as u32;
                c.bus.write16(DATA + 0x4000 + 0x30, 0x2000);
                w_3(
                    c,
                    DATA + 0x4000 + 0x34,
                    if seed % 2 == 0 { zero_fn } else { 1 },
                );
                w_3(c, this + 0x10, DATA + 0x5000);
                w_3(c, DATA + 0x5000 + 0x14, 0);
                w_3(c, GAME_GP.wrapping_add(0x2968), DATA + 0x6000);
                w_3(c, DATA + 0x6000, DATA + 0x6100);
                c.bus.write16(DATA + 0x6100 + 0x10, 0);
                w_3(c, DATA + 0x6100 + 0x14, 0);
                w_3(c, widget + 8, DATA + 0x7000);
                c.bus.write16(DATA + 0x7000 + 0x48, 0);
                w_3(c, DATA + 0x7000 + 0x4c, 0);
            },
        );
    }

    #[test]
    fn fe_state_peak_room_on_widget_notify_matches() {
        strict(
            ee::s04::f_001d3e08__generated,
            fe_state_peak_room_on_widget_notify,
            120,
            &|c, seed| {
                let mut r = Rng(0x001d_3e08 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = [0u64, 5, 7, 8, 9, 10, 12][(seed % 7) as usize];
                c.g[A3][0] = sx(DATA + 0x200);
            },
        );
    }

    pub(crate) const ROOM: u32 = DATA + 0x1_0000;

    pub(crate) const ROOM_WIDGET: u32 = DATA + 0x1_1000;

    pub(crate) const ROOM_FACTORY: u32 = DATA + 0x1_2000;

    #[test]
    fn c_fe_state_mountain_room_on_widget_action_matches() {
        check_all(
            ee::s04::f_001d2f68__generated,
            c_fe_state_mountain_room_on_widget_action,
            24,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x1d2f_68, 8);
                c.g[A0][0] = sx(ROOM);
                c.g[A1][0] = sx(if seed % 5 == 0 { 0 } else { ROOM_WIDGET });
                c.g[A2][0] = sx([5u32, 6, 1, 0, 2][seed as usize % 5]);
                c.bus.write32(ROOM_WIDGET + 0x18, r.next_u32() % 4);
                c.bus.write32(ROOM + 0x10, ROOM_FACTORY);
                let vt = vtable(c, &mut r, &[0x20, 0x24, 0x28, 0x2c]);
                c.bus.write32(ROOM_FACTORY, vt);
                c.bus
                    .write32(ROOM + 0x54, if seed % 3 == 0 { 0 } else { DATA + 0x1_3000 });
                c.bus.write32(ROOM + 0xb4, (seed % 3) as u32);
                let vt = vtable(c, &mut r, &[0x48, 0x4c]);
                c.bus.write32(DATA + 0x1_3000 + 8, vt);
            },
        );
    }

    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::machine_rt;

    use crate::difftest::fresh;

    pub(crate) use crate::game::fe::feasyncfile::c_fe_state_stats_on_widget_created;

    pub(crate) use crate::game::fe::festaterewards::tests::oc935_arm;
    pub(crate) use crate::game::fe::festaterewards::tests::oc935_mail;

    pub(crate) use crate::game::fe::festaterewards::tests::oc935_require_sequences;
    pub(crate) use crate::game::fe::festaterewards::tests::oc935_val;

    pub(crate) use crate::game::fe::festaterewards::tests::OC935_SELF;
    pub(crate) use crate::game::fe::festaterewards::tests::OC935_ZERO;

    pub(crate) use crate::game::fe::festatetrophyroom::tests::oc943_force;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::oc943_open;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::oc943_widget;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::oc945_force;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::oc945_machine;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::oc945_zeros;

    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Fixture for `cFEStateMountainRoom_ctor`: the three character-slot
    /// probes answer zero or not, a negative or positive player byte, and
    /// calls unwind at random points.
    pub(crate) fn oc935_mountain_room_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1d2678, 28);
        elf_image(c);
        oc935_arm(c, 99);
        c.pc = 0x001d_2678;
        let this = OC935_SELF;
        c.bus.write32(this + 0x10, DATA + 0x4000);
        c.bus.write32(DATA + 0x4000, DATA + 0x4100);
        let probe: [u32; 4] = std::array::from_fn(|_| {
            if r.next_u32() % 2 == 0 {
                OC935_ZERO
            } else {
                oc935_val(1)
            }
        });
        oc935_mail(c, 28, probe);
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(DATA + 0x2000);
        c.g[A2][0] = (r.next_u32() % 256) as u64;
    }

    #[test]
    fn c_fe_state_mountain_room_ctor_matches() {
        check(
            ee::s04::f_001d2678__generated,
            c_fe_state_mountain_room_ctor,
            200,
            &oc935_mountain_room_setup,
        );
        oc935_require_sequences(
            ee::s04::f_001d2678__generated,
            200,
            6,
            &oc935_mountain_room_setup,
        );
    }

    /// The fixture's control words: the gate of the fakes (`OC936_MAGIC`), the
    /// per-callee call counters and the result table the fakes answer from.
    pub(crate) const OC936_CTRL: u32 = DATA + 0x28_0000;

    pub(crate) const OC936_GATE: u32 = OC936_CTRL;

    pub(crate) const OC936_MAGIC: u32 = 0x4f43_3936;

    pub(crate) const OC936_SEEN: u32 = OC936_CTRL + 0x100;

    pub(crate) const OC936_TAB: u32 = OC936_CTRL + 0x400;

    /// The screen state `this`, the widget under test, the widget vtable and
    /// eight widget objects the state's slots point at.
    pub(crate) const OC936_THIS: u32 = DATA + 0x1000;

    pub(crate) const OC936_WIDGET: u32 = DATA + 0x2000;

    pub(crate) const OC936_VT: u32 = DATA + 0x3000;

    pub(crate) const OC936_WIDGETS: u32 = DATA + 0x4000;

    pub(crate) const OC936_OBJ: u32 = DATA + 0x6000;

    /// A fake callee: logs like the plain fake; `StrHash` answers its
    /// argument (so a widget whose hash is a name's address matches that
    /// name); the BE reward queries and the goal check answer from the
    /// fixture's result table (zero included).
    pub(crate) fn oc936_hook(c: &mut Cpu, target: u32) {
        let a0 = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        let slot = OC936_SEEN + 4 * ((target >> 4) & 0xff);
        let k = c.bus.read32(slot);
        c.bus.write32(slot, k + 1);
        fakes::fake_call(c, target);
        if c.pc != ra {
            return;
        }
        match target {
            0x0031_7670 => c.g[V0][0] = a0,
            0x0015_8910 | 0x0015_89b0 | 0x0015_8a50 | 0x0015_7bf0 => {
                let v = c.bus.read32(OC936_TAB + 4 * (k & 15));
                c.g[V0][0] = v as u64;
            }
            _ => {}
        }
    }

    macro_rules! oc936_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC936_GATE) != OC936_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc936_hook(c, $addr);
                }
            )*
            pub(crate) fn oc936_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc936_fakes!(
        (
            oc936_fk_317670,
            OC936_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc936_fk_398380,
            OC936_PREV_398380,
            ee::s13::OVERRIDE_00398380,
            ee::s13::f_00398380__generated,
            0x0039_8380
        ),
        (
            oc936_fk_3a1000,
            OC936_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc936_fk_416210,
            OC936_PREV_416210,
            ee::s15::OVERRIDE_00416210,
            ee::s15::f_00416210__generated,
            0x0041_6210
        ),
        (
            oc936_fk_14dc80,
            OC936_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc936_fk_14dd58,
            OC936_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc936_fk_156a90,
            OC936_PREV_156A90,
            ee::s01::OVERRIDE_00156A90,
            ee::s01::f_00156a90__generated,
            0x0015_6a90
        ),
        (
            oc936_fk_157bf0,
            OC936_PREV_157BF0,
            ee::s01::OVERRIDE_00157BF0,
            ee::s01::f_00157bf0__generated,
            0x0015_7bf0
        ),
        (
            oc936_fk_158910,
            OC936_PREV_158910,
            ee::s01::OVERRIDE_00158910,
            ee::s01::f_00158910__generated,
            0x0015_8910
        ),
        (
            oc936_fk_1589b0,
            OC936_PREV_1589B0,
            ee::s01::OVERRIDE_001589B0,
            ee::s01::f_001589b0__generated,
            0x0015_89b0
        ),
        (
            oc936_fk_158a50,
            OC936_PREV_158A50,
            ee::s01::OVERRIDE_00158A50,
            ee::s01::f_00158a50__generated,
            0x0015_8a50
        ),
        (
            oc936_fk_39ab00,
            OC936_PREV_39AB00,
            ee::s13::OVERRIDE_0039AB00,
            ee::s13::f_0039ab00__generated,
            0x0039_ab00
        ),
        (
            oc936_fk_39b760,
            OC936_PREV_39B760,
            ee::s13::OVERRIDE_0039B760,
            ee::s13::f_0039b760__generated,
            0x0039_b760
        ),
        (
            oc936_fk_417828,
            OC936_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
    );

    /// The four functions: entry, generated body, hand port.
    pub(crate) fn oc936_kind(kind: u32) -> (u32, GuestFn, GuestFn) {
        match kind {
            0 => (
                0x001d_2ad0,
                ee::s04::f_001d2ad0__generated,
                c_fe_state_mountain_room_on_widget_created,
            ),
            1 => (
                0x001d_3340,
                ee::s04::f_001d3340__generated,
                c_fe_state_mountain_room_update_event_icons,
            ),
            2 => (
                0x001d_38f8,
                ee::s04::f_001d38f8__generated,
                c_fe_state_peak_room_on_widget_created,
            ),
            _ => (
                0x001c_2f00,
                ee::s03::f_001c2f00__generated,
                c_fe_state_stats_on_widget_created,
            ),
        }
    }

    /// Fixture: the game image (name tables, medal table), the gated fakes,
    /// a screen state full of widget pointers, the widget under test with
    /// name hash `hash`, and a result table for the BE queries. The fields
    /// come from their own generator, so a dry run (`unwind` off) and the
    /// real run see the same screen.
    pub(crate) fn oc936_fixture(c: &mut Cpu, seed: u32, kind: u32, hash: u32, unwind: bool) {
        let _ = machine(
            c,
            seed,
            0x0936_0000 + kind as u64,
            if unwind { 40 } else { 0 },
        );
        elf_image(c);
        oc936_install_fakes();
        let mut r = Rng(0x9360_0001 ^ (seed as u64) << 8 ^ (kind as u64) << 40);
        c.bus.write32(OC936_GATE, OC936_MAGIC);
        for i in 0..16u32 {
            let v = match r.next_u32() % 6 {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => 5,
                _ => 0x0001_0000,
            };
            c.bus.write32(OC936_TAB + 4 * i, v);
        }
        // Widget vtable: halfword delta + method at every 8 bytes.
        for off in (0x20..0x80).step_by(8) {
            c.bus.write16(OC936_VT + off, 0);
            c.bus
                .write32(OC936_VT + off + 4, fakes::FAKE + 0x10 * (off / 8));
        }
        for i in 0..8u32 {
            c.bus.write32(OC936_WIDGETS + 0x100 * i + 8, OC936_VT);
        }
        c.bus.write32(OC936_WIDGET + 8, OC936_VT);
        c.bus.write32(OC936_WIDGET + 0x38, hash);
        c.bus.write32(OC936_WIDGET + 0x14, r.next_u32());
        c.bus.write32(OC936_WIDGET + 0x90, r.next_u32());
        c.bus.write32(OC936_WIDGET + 0x7c, r.next_u32());
        for w in (0..0x800).step_by(4) {
            let p = OC936_WIDGETS + 0x100 * (r.next_u32() % 8);
            c.bus.write32(OC936_THIS + w, p);
        }
        c.bus.write32(OC936_THIS + 0x10, OC936_OBJ);
        c.bus.write8(OC936_THIS + 0x44, (r.next_u32() % 2) as u8);
        c.bus.write8(OC936_THIS + 0xb8, (r.next_u32() % 4) as u8);
        c.bus.write32(OC936_THIS + 0xe0, r.next_u32() % 4);
        c.bus.write32(OC936_THIS + 0xdc, r.next_u32() % 4);
        c.bus.write32(OC936_THIS + 0xa8, r.next_u32());
        c.bus.write32(OC936_THIS + 0xac, r.next_u32());
        c.pc = oc936_kind(kind).0;
        c.g[A0][0] = sx(OC936_THIS);
        c.g[A1][0] = sx(OC936_WIDGET);
    }

    /// The name addresses the generated body hashes when nothing matches (the
    /// `a0` of each `StrHash` call), in call order.
    pub(crate) fn oc936_names(kind: u32, seed: u32) -> Vec<u32> {
        let mut c = fresh();
        oc936_fixture(&mut c, seed, kind, 0, false);
        (oc936_kind(kind).1)(&mut c);
        let n = c.bus.read32(fakes::LOG_COUNT);
        let mut out = Vec::new();
        for i in 0..n {
            let p = fakes::LOG_START + 0x220 * i;
            if c.bus.read32(p) == 0x0031_7670 {
                out.push(c.bus.read32(p + 0x50));
            }
        }
        out
    }

    /// Widget name = the `seed`-th name hashed (every name is reached), or
    /// none at all for some seeds.
    pub(crate) fn oc936_pick(kind: u32, seed: u32) -> u32 {
        let names = oc936_names(kind, seed);
        if names.is_empty() || seed % 11 == 10 {
            0
        } else {
            names[(seed as usize) % names.len()]
        }
    }

    pub(crate) fn oc936_check(kind: u32, seeds: u32) {
        let (_, generated, hand) = oc936_kind(kind);
        check_lohi(generated, hand, seeds, &|c, seed| {
            let h = oc936_pick(kind, seed);
            oc936_fixture(c, seed, kind, h, true)
        });
    }

    /// The fixture reaches at least `min` distinct paths (cycle totals of the
    /// generated body, no unwinds).
    pub(crate) fn oc936_paths(kind: u32, seeds: u32, min: usize) {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            let h = oc936_pick(kind, seed);
            oc936_fixture(&mut c, seed, kind, h, false);
            let before = c.cycles;
            (oc936_kind(kind).1)(&mut c);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= min, "paths {} {seen:?}", seen.len());
    }

    /// `cFEStateMountainRoom_OnWidgetCreated`: each known widget name, an
    /// unknown one, and the unwinds of its calls.
    #[test]
    fn c_fe_state_mountain_room_on_widget_created_matches() {
        oc936_check(0, 200);
    }

    #[test]
    fn c_fe_state_mountain_room_on_widget_created_paths() {
        oc936_paths(0, 200, 25);
    }

    /// `cFEStateMountainRoom_UpdateEventIcons`: the medal table walk with
    /// reward results below and above the icon threshold, and unwinds.
    #[test]
    fn c_fe_state_mountain_room_update_event_icons_matches() {
        oc936_check(1, 120);
    }

    #[test]
    fn c_fe_state_mountain_room_update_event_icons_paths() {
        oc936_paths(1, 120, 5);
    }

    /// `cFEStatePeakRoom_OnWidgetCreated`: each known widget name (both
    /// `this+0xe0` states), the marker name tables, the goal check results.
    #[test]
    fn c_fe_state_peak_room_on_widget_created_matches() {
        oc936_check(2, 240);
    }

    #[test]
    fn c_fe_state_peak_room_on_widget_created_paths() {
        oc936_paths(2, 240, 15);
    }

    /// `cFEStateStats_OnWidgetCreated`: each known widget name including the
    /// `sprintf` names, and unwinds.
    #[test]
    fn c_fe_state_stats_on_widget_created_matches() {
        oc936_check(3, 240);
    }

    #[test]
    fn c_fe_state_stats_on_widget_created_paths() {
        oc936_paths(3, 240, 19);
    }

    /// Gate word of the fakes (their result cells follow it).
    pub(crate) const OC942_GATE: u32 = DATA + 0x20_0000;

    pub(crate) const OC942_MAGIC: u32 = 0x4f43_3942;

    /// One 32-bit result cell per fake, indexed by the fake's number.
    pub(crate) const OC942_RES: u32 = DATA + 0x20_0010;

    pub(crate) const WORLD: u32 = DATA + 0x1000;

    /// The bolt manager holder: the manager is `MGR + 0x1a70`.
    pub(crate) const MGR: u32 = DATA + 0x2000;

    /// `*(gp - 0x854)`: the render context, with its vtable at `+0x10d8`.
    pub(crate) const RENDER: u32 = DATA + 0x4000;

    pub(crate) const VT_2: u32 = DATA + 0x5000;

    pub(crate) const VT2: u32 = DATA + 0x6000;

    pub(crate) const OBJ2_2: u32 = DATA + 0x7000;

    /// The bolt entry returned by 0x19e2b0 (its table offset at +0x14).
    pub(crate) const ENTRY: u32 = DATA + 0x8000;

    pub(crate) const OBJ_2: u32 = DATA + 0x9000;

    pub(crate) const OBJ3_2: u32 = DATA + 0xa000;

    pub(crate) const SELF: u32 = DATA + 0x400;

    pub(crate) fn oc942_install_base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        oc942_install();
        let r = machine_rt(c, seed, salt, 6);
        c.bus.write32(OC942_GATE, OC942_MAGIC);
        c.bus.write32(GAME_GP - 0x848, WORLD);
        c.bus.write32(WORLD + 0x7c, MGR);
        c.bus.write32(GAME_GP - 0x854, RENDER);
        c.bus.write32(RENDER + 0x10d8, VT_2);
        c.bus.write32(VT_2 + 0x174, FAKE);
        c.bus.write32(VT2 + 0x4c, FAKE);
        c.bus.write32(OBJ2_2 + 8, VT2);
        r
    }

    pub(crate) fn oc942_cell(c: &mut Cpu, idx: u32, v: u32) {
        c.bus.write32(OC942_RES + 4 * idx, v);
    }

    /// Fill the bolt entry the poll reads: the table offset at +0x14 and the
    /// two 16-bit values at +4 and +6 (the three orders of the state).
    pub(crate) fn oc942_entry(c: &mut Cpu, seed: u32) {
        c.bus.write32(ENTRY + 0x14, 0x40);
        let (a, b) = match seed % 3 {
            0 => (5u16, 2u16),
            1 => (2, 5),
            _ => (3, 3),
        };
        c.bus.write16(ENTRY + 0x40 + 4, a);
        c.bus.write16(ENTRY + 0x40 + 6, b);
    }

    #[test]
    fn cfe_state_mountain_room_init_matches() {
        check(
            ee::s04::f_001d2990__generated,
            cfe_state_mountain_room_init,
            60,
            &|c, seed| {
                let _r = oc942_install_base(c, seed, 0x1d_2990);
                c.g[A0][0] = sx(SELF);
                c.bus.write32(SELF + 0x10, OBJ_2);
                oc942_cell(c, 5, OBJ3_2);
                oc942_cell(c, 6, if seed % 2 == 0 { 0 } else { OBJ3_2 + 0x40 });
                oc942_cell(c, 8, if seed % 3 == 0 { 0 } else { OBJ3_2 + 0x80 });
            },
        );
    }

    #[test]
    fn cfe_state_mountain_room_poll_image_load_matches() {
        check(
            ee::s04::f_001d31c0__generated,
            cfe_state_mountain_room_poll_image_load,
            60,
            &|c, seed| {
                let _r = oc942_install_base(c, seed, 0x1d_31c0);
                c.g[A0][0] = sx(SELF);
                c.bus
                    .write32(SELF + 0x50, if seed % 5 == 0 { 0 } else { OBJ2_2 });
                c.bus.write32(SELF + 0xb0, 0x77 + seed);
                c.bus.write32(SELF + 0xbc, 0x88 + seed);
                c.bus.write32(SELF + 0x48, OBJ3_2);
                c.bus.write32(SELF + 0x4c, OBJ3_2);
                c.bus
                    .write32(OBJ3_2 + 0x14, if seed % 2 == 0 { 0x40 } else { 0 });
                c.bus.write32(OBJ2_2 + 0x78, 0);
                oc942_cell(c, 1, if seed % 3 == 0 { 0 } else { 1 });
                oc942_cell(c, 2, ENTRY);
                oc942_entry(c, seed);
                oc942_cell(c, 3, seed);
            },
        );
    }

    #[test]
    fn cfe_state_peak_room_ctor_matches() {
        check(
            ee::s04::f_001d3780__generated,
            cfe_state_peak_room_ctor,
            60,
            &|c, seed| {
                let _r = oc942_install_base(c, seed, 0x1d_3780);
                c.g[A0][0] = sx(SELF);
                c.g[A2][0] = (seed as u64) & 0xff;
                c.g[A3][0] = 0x10 * (seed as u64 % 4);
                c.bus.write32(SELF + 0x10, OBJ_2);
                oc942_cell(c, 14, 3 + seed);
                oc942_cell(c, 15, 4);
                oc942_cell(c, 16, 0x5 + seed);
                oc942_cell(c, 17, 0x6 + seed);
            },
        );
    }

    #[test]
    fn cfe_state_peak_room_on_activate_matches() {
        check(
            ee::s04::f_001d3c60__generated,
            cfe_state_peak_room_on_activate,
            60,
            &|c, seed| {
                let _r = oc942_install_base(c, seed, 0x1d_3c60);
                c.g[A0][0] = sx(SELF);
            },
        );
    }

    /// `RefreshGoalList(self, peak)`: the six rows of five widgets, the event count of
    /// (mountain, peak) from the real table, and per goal either a locked (reward -1) or
    /// an unlocked icon (reward id, texture handle store). The real `QueryEventReward` and
    /// `SetEventName` run on faked back-end callees.
    #[test]
    fn c_fe_state_peak_room_refresh_goal_list_matches() {
        oc943_install_fakes();
        fakes::covered(
            ee::s04::f_001d3f80__generated,
            c_fe_state_peak_room_refresh_goal_list,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d3f80, 30);
                elf_image(c);
                oc943_open(c, seed);
                let this = DATA + 0x1000;
                for (k, base) in [0x58u32, 0x6c, 0x80, 0x94, 0xa8, 0xbc]
                    .into_iter()
                    .enumerate()
                {
                    for j in 0..5u32 {
                        let widget = DATA + 0x2000 + 0x100 * (k as u32 * 5 + j);
                        c.bus.write32(this + base + 4 * j, widget);
                        oc943_widget(c, widget);
                    }
                }
                c.bus.write8(this + 0x44, (seed % 3) as u8);
                c.bus.write32(this + 0x10, DATA + 0x8000);
                c.bus.write32(this + 0xdc, 7 + seed % 2);
                c.bus.write32(this + 0xe0, (seed / 4) % 3);
                for idx in 40..=43 {
                    oc943_force(c, idx, if seed % 3 == 0 { u32::MAX } else { seed % 7 });
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 4) as u64;
                c.pc = 0x001d_3f80;
            },
        );
    }

    #[test]
    fn cfe_state_peak_room_on_widget_action_matches() {
        check(
            ee::s04::f_001d3c80__generated,
            cfe_state_peak_room_on_widget_action,
            300,
            &|c, seed| {
                let mut r = oc945_machine(c, seed, 0x3c80);
                let obj = DATA + 0xa_0000;
                let widget = DATA + 0xa_1000;
                let kinds = [5u64, 1, 6, 0, 2, 7, 3, 4, 5, 1, 6];
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = if seed % 9 == 0 { 0 } else { sx(widget) };
                c.g[A2][0] = kinds[(seed % 11) as usize];
                c.pc = 0x001d_3c80;
                let (p1, p2, p3) = (DATA + 0xa_2000, DATA + 0xa_3000, DATA + 0xa_4000);
                c.bus.write32(obj + 0x10, p1);
                c.bus.write32(p1, p2);
                c.bus.write32(p2 + 4, p3);
                c.bus.write16(p3 + 0x20, 0x10);
                c.bus.write32(p3 + 0x24, 0x0019_da10);
                c.bus.write16(p3 + 0x28, 0x20);
                c.bus.write32(p3 + 0x2c, 0x0019_dc20);
                c.bus.write32(obj + 0xe0, r.next_u32());
                c.bus.write32(obj + 0xdc, r.next_u32());
                c.bus.write8(obj + 0x44, r.next_u32() as u8);
                c.bus.write32(widget + 0x18, r.next_u32());
                let vt = DATA + 0xa_6000;
                c.bus.write16(vt + 0x48, 8);
                c.bus.write32(vt + 0x4c, 0x0039_e4c0);
                for k in 0..4 {
                    let item = DATA + 0xa_5000 + 0x20 * k;
                    let entry = if (seed >> k) & 1 == 0 { item } else { 0 };
                    c.bus.write32(obj + 0x48 + 4 * k, entry);
                    c.bus.write32(item + 8, vt);
                }
                oc945_zeros(c, &mut r);
            },
        );
    }

    #[test]
    fn cfe_state_peak_room_update_help_text_matches() {
        check(
            ee::s04::f_001d3ec8__generated,
            cfe_state_peak_room_update_help_text,
            300,
            &|c, seed| {
                let mut r = oc945_machine(c, seed, 0x3ec8);
                let obj = DATA + 0xa_7000;
                let help = DATA + 0xa_8000;
                let vt = DATA + 0xa_9000;
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = r.next_u32() as u64;
                c.pc = 0x001d_3ec8;
                c.bus
                    .write32(obj + 0xd0, if seed % 5 == 0 { 0 } else { help });
                c.bus.write32(help + 8, vt);
                c.bus.write16(vt + 0x48, 4);
                c.bus.write32(vt + 0x4c, 0x0039_e4c0);
                c.bus.write8(obj + 0x44, r.next_u32() as u8);
                c.bus.write32(obj + 0xdc, r.next_u32());
                c.bus.write32(obj + 0xe0, r.next_u32());
                oc945_zeros(c, &mut r);
            },
        );
    }

    #[test]
    fn cfe_state_trophy_room_ctor_matches() {
        check(
            ee::s04::f_001d4268__generated,
            cfe_state_trophy_room_ctor,
            300,
            &|c, seed| {
                let mut r = oc945_machine(c, seed, 0x4268);
                let obj = DATA + 0xa_a000;
                let p = DATA + 0xa_b000;
                let text = DATA + 0xa_c000;
                let q = DATA + 0xa_d000;
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = sx(r.next_u32());
                c.g[T0][0] = sx(r.next_u32());
                c.pc = 0x001d_4268;
                c.bus.write32(obj + 0x10, p);
                c.bus.write32(p, r.next_u32());
                // The string object returned by the BE lookup (call 3).
                c.bus.write32(text + 0xc, q);
                c.bus.write16(q + 0x10, 0x10);
                c.bus.write32(q + 0x14, 0x0019_da10);
                oc945_zeros(c, &mut r);
                oc945_force(c, 3, text);
            },
        );
    }
}
