//! Staging module oc991 (wave wave259): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc991`.

#![allow(unused_imports)]

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Flow, Frame, Unwound};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001f_e4a8,
        ov_screen_save_replay_v37,
        "OVScreenSaveReplay_v37",
    ),
    (
        0x001f_ec40,
        ov_screen_save_records_shutdown,
        "OVScreenSaveRecords_Shutdown",
    ),
    (
        0x001f_f170,
        ov_screen_save_records_v37,
        "OVScreenSaveRecords_v37",
    ),
    (
        0x001f_f310,
        ov_screen_save_records_v39,
        "OVScreenSaveRecords_v39",
    ),
    (
        0x001f_f590,
        ov_screen_reward_list_init,
        "OVScreenRewardList_Init",
    ),
    (
        0x0020_0288,
        ov_screen_rewards_list_popup_on_widget_action,
        "OVScreenRewardsListPopup_OnWidgetAction",
    ),
    (
        0x0020_8518,
        ov_screen_map_on_widget_created,
        "OVScreenMap_OnWidgetCreated",
    ),
    (
        0x0020_8538,
        ov_screen_map_transition_in,
        "OVScreenMap_TransitionIn",
    ),
];

/// Direct callees the generated bodies reach by name (not through a table).
const MAP_EVENT_SELECT_WIDGET_CREATED: Callee = Callee {
    addr: 0x0020_09d0,
    f: ee::s05::f_002009d0,
};
const MAP_EVENT_SELECT_TRANSITION_IN: Callee = Callee {
    addr: 0x0020_0a70,
    f: ee::s05::f_00200a70,
};
const REWARD_STATE_OVERLAY_INIT: Callee = Callee {
    addr: 0x0020_a380,
    f: ee::s05::f_0020a380,
};
const REWARD_LIST_GET_ENTRY_TEXT: Callee = Callee {
    addr: 0x001f_f7b8,
    f: ee::s05::f_001ff7b8,
};

/// Return addresses of the twelve calls of the save-replay body at 0x1fe4a8
/// (taken from its `c.g[RA][0] = sx(...)` lines, in order).
const SAVE_REPLAY_RETS_1FE4A8: [u32; 12] = [
    0x001f_e4dc,
    0x001f_e4e8,
    0x001f_e500,
    0x001f_e50c,
    0x001f_e518,
    0x001f_e530,
    0x001f_e53c,
    0x001f_e548,
    0x001f_e560,
    0x001f_e56c,
    0x001f_e578,
    0x001f_e590,
];

/// The same twelve return addresses of the identical body at 0x1ff170.
const SAVE_REPLAY_RETS_1FF170: [u32; 12] = [
    0x001f_f1a4,
    0x001f_f1b0,
    0x001f_f1c8,
    0x001f_f1d4,
    0x001f_f1e0,
    0x001f_f1f8,
    0x001f_f204,
    0x001f_f210,
    0x001f_f228,
    0x001f_f234,
    0x001f_f240,
    0x001f_f258,
];

/// `cOVState_SAVEREPLAY_hideActionButtons(this, flag)` (0x1fe4a8), the same
/// body as the record screen's at 0x1ff170: when `this->+0x22c` differs from
/// `flag`, four text lookups each followed by a find and a dispatch through
/// the found object's class; then `this->+0x22c = flag`. `rets` are the
/// return addresses of the twelve calls, in order.
fn save_replay_body(c: &mut Cpu, rets: &[u32; 12]) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S1, 0x20);
    frame.save(c, S2, 0x10);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x30);
    c.g[S2][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x22c).lw(c);
    let unchanged = c.g[V0][0] == c.g[S2][0];
    c.g[S0][0] = c.g[S2][0] ^ 0x1;
    if !unchanged {
        c.cycles += 49;
        c.g[A0][0] = 0x004a_0000;
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(0x2090));
        call_ptr(c, 0x0031_7670, rets[0])?;
        save_replay_find_dispatch(c, rets[1], rets[2])?;
        c.g[A0][0] = 0x004a_0000;
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(0x2568));
        call_ptr(c, 0x0031_7670, rets[3])?;
        save_replay_find_dispatch(c, rets[4], rets[5])?;
        c.g[A0][0] = 0x0047_0000;
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(0x188));
        call_ptr(c, 0x0031_7670, rets[6])?;
        save_replay_find_dispatch(c, rets[7], rets[8])?;
        c.g[A0][0] = 0x004a_0000;
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(0x20a0));
        call_ptr(c, 0x0031_7670, rets[9])?;
        save_replay_find_dispatch(c, rets[10], rets[11])?;
        let flag = c.g[S2][0];
        Addr(c.g[S1][0] as u32).off(0x22c).sw(c, flag);
    }
    c.cycles += 6;
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `obj = cUIScreen_getObjectByHashName(this->+0x40)`, then the object's
/// class (`obj->+8`) gives a delta (`+0x48`) and a function (`+0x4c`), called
/// with `obj + delta` and `s0` as its second argument.
fn save_replay_find_dispatch(c: &mut Cpu, find_ret: u32, call_ret: u32) -> Flow {
    c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x40).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, find_ret)?;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x48).lh(c);
    c.g[A2][0] = Addr(c.g[V1][0] as u32).off(0x4c).lw(c);
    let t = c.g[A2][0] as u32;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, call_ret)?;
    Ok(())
}

/// `cOVState_SAVEREPLAY_hideActionButtons` (0x1fe4a8).
pub fn ov_screen_save_replay_v37(c: &mut Cpu) {
    guest(c, |c| save_replay_body(c, &SAVE_REPLAY_RETS_1FE4A8));
}

/// `OVScreenSaveRecords_v37` (0x1ff170): the save-replay body with its own
/// return addresses.
pub fn ov_screen_save_records_v37(c: &mut Cpu) {
    guest(c, |c| save_replay_body(c, &SAVE_REPLAY_RETS_1FF170));
}

/// `OVScreenSaveRecords_Shutdown` (0x1fec40): `MemCardStateBase_Shutdown()`.
pub fn ov_screen_save_records_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x001d_58b8, 0x001f_ec50)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `OVScreenSaveRecords_v39` (0x1ff310): sets `this->+0x224` to `s0 = !a2`
/// after a lookup and two dispatches through found objects.
pub fn ov_screen_save_records_v39(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = 0x004a_0000;
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(0x2090));
        c.g[S0][0] = c.g[S0][0] ^ 0x1;
        call_ptr(c, 0x0031_7670, 0x001f_f33c)?;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_f348)?;
        c.g[A0][0] = 0x004a_0000;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(0x2568));
        call_ptr(c, 0x0031_7670, 0x001f_f358)?;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_f364)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x48).lh(c);
        c.g[A2][0] = Addr(c.g[V1][0] as u32).off(0x4c).lw(c);
        let t = c.g[A2][0] as u32;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x001f_f37c)?;
        let flag = c.g[S0][0];
        Addr(c.g[S1][0] as u32).off(0x224).sw(c, flag);
        c.g[A1][0] = c.g[S0][0];
        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x001f_f398)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `OVScreenRewardList_Init(this)` (0x1ff590): the reward list's text and
/// model lookups, the overlay init, the entry text, and the two list
/// handles at `+0x9c` / `+0x40`. Returns through its three exits as the
/// generated body does (the one-cycle tail on the early exits).
pub fn ov_screen_reward_list_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x0047_0000;
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(0x3c8));
        c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x10).lw(c);
        call_ptr(c, 0x0031_7670, 0x001f_f5b4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_f5c8)?;
        let absent = c.g[V0][0] == 0;
        let found = c.g[V0][0];
        Addr(c.g[S1][0] as u32).off(0x40).sw(c, found);
        if !absent {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_f5e0)?;
        }

        c.cycles += 11;
        c.g[A0][0] = 0x0047_0000;
        c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_sub(0x1c88));
        call_ptr(c, 0x0031_7670, 0x001f_f5f0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_f604)?;
        let absent = c.g[V0][0] == 0;
        if !absent {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_f61c)?;
        }

        // The text and the entry block: a block that ends in the common tail.
        c.cycles += 16;
        c.g[A0][0] = c.g[S1][0];
        REWARD_STATE_OVERLAY_INIT.call(c, 0x001f_f624)?;
        let list = c.g[S1][0];
        Addr(list as u32).off(0x9c).sw(c, 0);
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S1][0];
        REWARD_LIST_GET_ENTRY_TEXT.call(c, 0x001f_f638)?;
        c.g[A0][0] = 0x0047_0000;
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_sub(0x17e8));
        call_ptr(c, 0x0031_7670, 0x001f_f644)?;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_f650)?;
        c.g[A2][0] = c.g[V0][0];
        let no_object = c.g[A2][0] == 0;
        c.g[V1][0] = 0xffff_ffff_ffff_ff7f;
        // Every path but the direct call below ends the block with one cycle
        // and `a1 = 0x4a0000`.
        let mut through_block_end = true;
        if !no_object {
            c.cycles += 8;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x14).lw(c);
            c.g[A0][0] = Addr(c.g[A2][0] as u32).off(0x96).lbu(c);
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            let flags = c.g[V0][0];
            Addr(c.g[A2][0] as u32).off(0x14).sw(c, flags);
            c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x9c).lw(c);
            c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[A0][0] == 0 {
                c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x90).lw(c);
                c.cycles += 2;
                c.g[V0][0] |= 0x8;
                let mode = c.g[V0][0];
                Addr(c.g[A2][0] as u32).off(0x90).sw(c, mode);
            } else {
                through_block_end = false;
                c.cycles += 5;
                c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0x9c).lbu(c);
                c.g[A0][0] = c.g[A2][0];
                call_ptr(c, 0x0039_b760, 0x001f_f688)?;
                c.g[A1][0] = 0x004a_0000;
            }
        }
        if through_block_end {
            c.cycles += 1;
            c.g[A1][0] = 0x004a_0000;
        }

        c.cycles += 12;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = sx(abi::r32(c, A1).wrapping_add(0x2570));
        call_ptr(c, 0x0039_e8b8, 0x001f_f6a8)?;
        c.g[A1][0] = 0x004a_0000;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = sx(abi::r32(c, A1).wrapping_add(0x2578));
        call_ptr(c, 0x0039_e8b8, 0x001f_f6b8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `OVScreenRewardsListPopup_OnWidgetAction(this, data, kind)` (0x200288):
/// the popup's action dispatch. Returns whether the body leaves through the
/// one-cycle tail (`'b_200370`) rather than straight to the epilogue.
pub fn ov_screen_rewards_list_popup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        let no_data = c.g[S0][0] == 0;
        c.g[S1][0] = c.g[A0][0];
        let tail = no_data || widget_action(c, frame)?;
        if tail {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// The kind dispatch of the widget action (`'b_20033c`). Returns true when
/// the body takes the `'b_200370` tail.
fn widget_action(c: &mut Cpu, frame: Frame) -> Result<bool, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = 0x5;
    let kind_five = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = 0x6;
    if kind_five {
        widget_action_redraw(c, frame)?;
        c.cycles += 5;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(0x18));
        call_ptr(c, 0x0039_f400, 0x0020_0334)?;
        return Ok(false);
    }
    c.cycles += 2;
    if c.g[A2][0] == c.g[V0][0] {
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x10).lw(c);
        return widget_action_notify(c);
    }
    c.cycles += 2;
    Ok(false)
}

/// Kind 5 (`'b_200328`): two float fields (1.0f) and a zeroed pair in the
/// frame, the vtable call with them, then the optional second call.
fn widget_action_redraw(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 16;
    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x8).lw(c);
    c.g[A1][0] = c.g[SP][0];
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
    frame.save_fpr(c, 0, 0x4);
    frame.save_fpr(c, 0, 0x0);
    Addr(frame.at(c, 0x8)).sw(c, 0);
    Addr(frame.at(c, 0xc)).sw(c, 0);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x0020_02f0)?;
    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x18).lw(c);
    c.g[V0][0] = 0x1;
    let other = c.g[V1][0] != c.g[V0][0];
    c.g[A1][0] = 0;
    if !other {
        c.cycles += 10;
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x10).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0x1;
        c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V1][0] = Addr(c.g[A3][0] as u32).off(0x4).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x20).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0020_0324)?;
        c.g[A1][0] = c.g[V0][0];
    }
    Ok(())
}

/// Kind 6 (`'b_20033c` exit): the notify call on the object at `this->+0x10`.
/// Always leaves through the tail.
fn widget_action_notify(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 10;
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x18).lw(c);
    c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V1][0] = Addr(c.g[A3][0] as u32).off(0x4).lw(c);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x28).lh(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x2c).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x0020_035c)?;
    let absent = c.g[V0][0] == 0;
    c.g[A1][0] = c.g[V0][0];
    if !absent {
        c.cycles += 3;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(0x18));
        call_ptr(c, 0x0039_f400, 0x0020_0370)?;
    }
    Ok(true)
}

/// `OVScreenMap_OnWidgetCreated(this)` (0x208518): `EventSelect_OnWidgetCreated(this + 0x9c)`.
pub fn ov_screen_map_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(0x9c));
        MAP_EVENT_SELECT_WIDGET_CREATED.call(c, 0x0020_8528)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `OVScreenMap_TransitionIn(this)` (0x208538): `EventSelect_TransitionIn(this + 0x9c)`.
pub fn ov_screen_map_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(0x9c));
        MAP_EVENT_SELECT_TRANSITION_IN.call(c, 0x0020_8548)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::{Rng, DATA};
    use std::ptr::addr_of_mut;

    /// Gate of this module's fakes: a word in the test machine's RAM. Only the
    /// machines of these tests carry it, so other tests run the real bodies.
    const OC991_GATE: u32 = DATA + 0x21_0000;
    const OC991_MAGIC: u32 = 0x0991_c0de;
    /// Mailbox: the result of the gated fake with slot `k` is the word at `OC991_MB + 4k`.
    const OC991_MB: u32 = DATA + 0x21_0100;
    /// Result of every indirect call the `miss` hook answers (table targets outside the image).
    const OC991_MISS: u32 = DATA + 0x21_0200;
    /// The `this` object of the overlay functions.
    const OC991_THIS: u32 = DATA + 0x4000;

    macro_rules! oc991_fake {
        ($name:ident, $target:expr, $gen:path) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC991_GATE) != OC991_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                fakes::fake_call(c, $target);
            }
        };
        ($name:ident, $target:expr, $gen:path, $slot:expr) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC991_GATE) != OC991_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                fakes::fake_call(c, $target);
                c.g[V0][0] = c.bus.read32(OC991_MB + 4 * $slot) as u64;
            }
        };
    }

    oc991_fake!(
        oc991_fake_str_hash,
        0x0031_7670,
        ee::s10::f_00317670__generated
    );
    oc991_fake!(
        oc991_fake_find_object,
        0x0039_d860,
        ee::s13::f_0039d860__generated,
        0
    );
    oc991_fake!(
        oc991_fake_397cc0,
        0x0039_7cc0,
        ee::s13::f_00397cc0__generated,
        1
    );
    oc991_fake!(
        oc991_fake_c870,
        0x0039_c870,
        ee::s13::f_0039c870__generated,
        2
    );
    oc991_fake!(
        oc991_fake_b760,
        0x0039_b760,
        ee::s13::f_0039b760__generated,
        3
    );
    oc991_fake!(
        oc991_fake_e8b8,
        0x0039_e8b8,
        ee::s13::f_0039e8b8__generated,
        4
    );
    oc991_fake!(
        oc991_fake_f400,
        0x0039_f400,
        ee::s13::f_0039f400__generated,
        5
    );
    oc991_fake!(
        oc991_fake_overlay_init,
        0x0020_a380,
        ee::s05::f_0020a380__generated,
        6
    );
    oc991_fake!(
        oc991_fake_event_widget,
        0x0020_09d0,
        ee::s05::f_002009d0__generated
    );
    oc991_fake!(
        oc991_fake_event_transition,
        0x0020_0a70,
        ee::s05::f_00200a70__generated
    );
    oc991_fake!(
        oc991_fake_memcard,
        0x001d_58b8,
        ee::s04::f_001d58b8__generated
    );

    /// The entry text of the reward list: a fake that also stores its
    /// mailbox word at `this + 0x9c` (the list's flag, which Init tests next).
    fn oc991_fake_entry_text(c: &mut Cpu) {
        if c.bus.read32(OC991_GATE) != OC991_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc991_fake_entry_text),
                ee::s05::f_001ff7b8__generated,
            );
        }
        let this = c.g[A0][0] as u32;
        fakes::fake_call(c, 0x001f_f7b8);
        let flag = c.bus.read32(OC991_MB + 4 * 7);
        c.bus.write32(this + 0x9c, flag);
    }

    /// The `miss` hook: an indirect call to a table target answers with the
    /// `OC991_MISS` word.
    fn oc991_miss(c: &mut Cpu, target: u32) {
        fakes::fake_call(c, target);
        c.g[V0][0] = c.bus.read32(OC991_MISS) as u64;
    }

    /// Puts every fake in its override slot (once per process).
    fn oc991_install() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc991_fake_str_hash,
            fakes::prev_cell(oc991_fake_str_hash),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc991_fake_find_object,
            fakes::prev_cell(oc991_fake_find_object),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00397CC0),
            oc991_fake_397cc0,
            fakes::prev_cell(oc991_fake_397cc0),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039C870),
            oc991_fake_c870,
            fakes::prev_cell(oc991_fake_c870),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039B760),
            oc991_fake_b760,
            fakes::prev_cell(oc991_fake_b760),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E8B8),
            oc991_fake_e8b8,
            fakes::prev_cell(oc991_fake_e8b8),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039F400),
            oc991_fake_f400,
            fakes::prev_cell(oc991_fake_f400),
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0020A380),
            oc991_fake_overlay_init,
            fakes::prev_cell(oc991_fake_overlay_init),
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_001FF7B8),
            oc991_fake_entry_text,
            fakes::prev_cell(oc991_fake_entry_text),
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_002009D0),
            oc991_fake_event_widget,
            fakes::prev_cell(oc991_fake_event_widget),
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00200A70),
            oc991_fake_event_transition,
            fakes::prev_cell(oc991_fake_event_transition),
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001D58B8),
            oc991_fake_memcard,
            fakes::prev_cell(oc991_fake_memcard),
        );
    }

    /// Scrambled registers and the gated fakes; the machine's miss hook
    /// answers table calls.
    fn oc991_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc991_install();
        let r = machine(c, seed, salt, calls);
        c.hooks.miss = oc991_miss;
        c.bus.write32(OC991_GATE, OC991_MAGIC);
        r
    }

    fn oc991_set(c: &mut Cpu, slot: u32, value: u32) {
        c.bus.write32(OC991_MB + 4 * slot, value);
    }

    /// A class table at `vt` with a random delta and a table target at each
    /// `slots` offset (the delta is the halfword before the target word).
    fn oc991_vtable(c: &mut Cpu, r: &mut Rng, vt: u32, slots: &[u32]) {
        for &off in slots {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + off, delta as u16);
            c.bus.write32(vt + off + 4, fakes::FAKE + off);
        }
    }

    /// The found object (mailbox slot 0) with its class at `obj + 8`.
    fn oc991_objects(c: &mut Cpu, r: &mut Rng) -> u32 {
        let vt = DATA + 0x2100;
        let obj = DATA + 0x2000;
        oc991_vtable(c, r, vt, &[0x48]);
        c.bus.write32(obj + 8, vt);
        oc991_set(c, 0, obj);
        obj
    }

    fn oc991_setup_replay(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = oc991_machine(c, seed, salt, 12);
        oc991_objects(c, &mut r);
        let flag = seed % 2;
        let stored = if seed % 3 == 0 { flag } else { flag ^ 1 };
        c.bus.write32(OC991_THIS + 0x22c, stored);
        c.g[A0][0] = sx(OC991_THIS);
        c.g[A1][0] = flag as u64;
    }

    #[test]
    fn ov_screen_save_replay_v37_matches() {
        check(
            ee::s05::f_001fe4a8__generated,
            ov_screen_save_replay_v37,
            200,
            &|c, seed| oc991_setup_replay(c, seed, 0x1fe4),
        );
    }

    #[test]
    fn ov_screen_save_records_v37_matches() {
        check(
            ee::s05::f_001ff170__generated,
            ov_screen_save_records_v37,
            200,
            &|c, seed| oc991_setup_replay(c, seed, 0x1ff1),
        );
    }

    #[test]
    fn ov_screen_save_records_shutdown_matches() {
        check(
            ee::s05::f_001fec40__generated,
            ov_screen_save_records_shutdown,
            40,
            &|c, seed| {
                let _r = oc991_machine(c, seed, 0x1fec, 1);
            },
        );
    }

    #[test]
    fn ov_screen_save_records_v39_matches() {
        check(
            ee::s05::f_001ff310__generated,
            ov_screen_save_records_v39,
            200,
            &|c, seed| {
                let mut r = oc991_machine(c, seed, 0x1ff3, 6);
                oc991_objects(c, &mut r);
                c.g[A0][0] = sx(OC991_THIS);
                c.g[A2][0] = (seed % 2) as u64;
            },
        );
    }

    fn oc991_setup_init(c: &mut Cpu, seed: u32) {
        let mut r = oc991_machine(c, seed, 0x1ff5, 13);
        let obj = oc991_objects(c, &mut r);
        c.bus.write32(obj + 0x14, r.next_u32());
        c.bus.write8(obj + 0x96, (seed % 5) as u8);
        if seed % 3 == 0 {
            oc991_set(c, 0, 0);
        }
        // Both 0x397cc0 calls answer with this word (null sometimes).
        oc991_set(c, 1, if seed % 4 == 0 { 0 } else { DATA + 0x3100 });
        // The entry text writes the list flag; odd seeds make the byte test true.
        oc991_set(c, 7, if seed % 2 == 0 { 0 } else { 0x100 });
        c.bus.write32(OC991_THIS + 0x10, DATA + 0x3000);
        c.g[A0][0] = sx(OC991_THIS);
    }

    #[test]
    fn ov_screen_reward_list_init_matches() {
        check(
            ee::s05::f_001ff590__generated,
            ov_screen_reward_list_init,
            200,
            &|c, seed| oc991_setup_init(c, seed),
        );
    }

    fn oc991_setup_widget_action(c: &mut Cpu, seed: u32) {
        let mut r = oc991_machine(c, seed, 0x2002, 6);
        let vt = DATA + 0x2100;
        oc991_vtable(c, &mut r, vt, &[0x20, 0x28, 0x58]);
        // A zero data pointer takes the early exit.
        let arg: u32 = if seed % 7 == 0 { 0 } else { DATA + 0x1000 };
        if arg != 0 {
            c.bus.write32(arg + 8, vt);
            c.bus.write32(arg + 0x18, if seed % 2 == 0 { 1 } else { 0 });
        }
        let p = DATA + 0x2400;
        let obj = DATA + 0x2800;
        c.bus.write32(OC991_THIS + 0x10, p);
        c.bus.write32(p, obj);
        c.bus.write32(obj + 4, vt);
        // The notify call's result decides the tail (zero or not).
        c.bus.write32(
            OC991_MISS,
            if (seed / 3) % 2 == 0 { 0 } else { 0x0081_2340 },
        );
        c.g[A0][0] = sx(OC991_THIS);
        c.g[A1][0] = arg as u64;
        c.g[A2][0] = match seed % 3 {
            0 => 5,
            1 => 6,
            _ => 7 + (seed % 4) as u64,
        };
    }

    #[test]
    fn ov_screen_rewards_list_popup_on_widget_action_matches() {
        check(
            ee::s05::f_00200288__generated,
            ov_screen_rewards_list_popup_on_widget_action,
            200,
            &|c, seed| oc991_setup_widget_action(c, seed),
        );
    }

    #[test]
    fn ov_screen_map_on_widget_created_matches() {
        check(
            ee::s05::f_00208518__generated,
            ov_screen_map_on_widget_created,
            40,
            &|c, seed| {
                let _r = oc991_machine(c, seed, 0x2085, 1);
                c.g[A0][0] = sx(OC991_THIS);
            },
        );
    }

    #[test]
    fn ov_screen_map_transition_in_matches() {
        check(
            ee::s05::f_00208538__generated,
            ov_screen_map_transition_in,
            40,
            &|c, seed| {
                let _r = oc991_machine(c, seed, 0x2085, 1);
                c.g[A0][0] = sx(OC991_THIS);
            },
        );
    }
}
