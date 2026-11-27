//! Staging module oc1132 (wave wave271): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1132`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003b_8978, snd_set_lowpass, "SND_SetLowpass"),
    (
        0x0019_6f50,
        cfe_state_request_line_transition_in,
        "cFEStateRequestLine_TransitionIn",
    ),
    (
        0x0019_9f20,
        cfe_state_char_equip_on_widget_notify,
        "cFEStateCharEquip_OnWidgetNotify",
    ),
];

// ------------------------------------------------------------ SND_SetLowpass

const OC1132_SETLOWPASS_START: u32 = 0x003b_8978;

const OC1132_SETLOWPASS_END: u32 = 0x003b_89dc;

/// Loop head (apply the low-pass to the cursor's channel): the resume point
/// after an unwind at the back edge.
const OC1132_SETLOWPASS_HEAD: u32 = 0x003b_89a8;

const OC1132_HANDLE_TO_CHANNEL: Callee = Callee {
    addr: 0x003b_a6e8,
    f: ee::s13::SND_HandleToChannel,
};

const OC1132_NEXT_HANDLE_CHANNEL: Callee = Callee {
    addr: 0x003b_aa00,
    f: ee::s13::SND_NextHandleChannel,
};

const OC1132_APPLY_LOWPASS: Callee = Callee {
    addr: 0x003c_3178,
    f: ee::s13::SND_ChannelApplyLowpass,
};

/// `int SND_SetLowpass(int handle, int cutoff)` (0x3b8978): apply the low-pass
/// cutoff to every voice channel of a handle (`SND_ChannelApplyLowpass`).
/// Returns the handle's first channel (or the error from
/// `SND_HandleToChannel`, which skips the loop).
///
/// The original is a `match pc` dispatcher whose loop returns on an unwind
/// request, so the loop head is an entry point too (s0 the first channel, s1
/// the cutoff, a0 the cursor's channel, the cursor at `0(sp)`).
pub fn snd_set_lowpass(c: &mut Cpu) {
    let frame = Frame { size: 0x40 };
    let resumed = match c.pc {
        OC1132_SETLOWPASS_START => false,
        OC1132_SETLOWPASS_HEAD => true,
        pc => {
            c.jump_out(pc, OC1132_SETLOWPASS_START, OC1132_SETLOWPASS_END);
            return;
        }
    };
    guest(c, |c| {
        if !resumed {
            c.cycles += 9;
            Frame::open(c, frame.size);
            frame.save(c, S1, 0x20);
            frame.save(c, S0, 0x30);
            frame.save_ra(c, 0x10);
            c.g[S1][0] = c.g[A1][0];
            OC1132_HANDLE_TO_CHANNEL.call(c, 0x003b_8990)?;
            c.g[S0][0] = c.g[V0][0];
            let failed = (c.g[S0][0] as i64) < 0;
            c.g[V0][0] = u64::MAX;
            if failed {
                return oc1132_setlowpass_epilogue(c, frame);
            }
            // 0x3b899c: the cursor starts at -1; the first step only finds
            // the first channel.
            c.cycles += 2;
            let cursor = sp(c);
            c.sw(cursor, c.g[V0][0]);
        }
        let mut apply = resumed;
        loop {
            if apply {
                c.cycles += 2;
                c.g[A1][0] = c.g[S1][0];
                OC1132_APPLY_LOWPASS.call(c, 0x003b_89b0)?;
            }
            apply = true;
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            OC1132_NEXT_HANDLE_CHANNEL.call(c, 0x003b_89bc)?;
            let more = c.g[V0][0] != 0;
            let cursor = sp(c);
            c.g[A0][0] = c.lw(cursor);
            if !more {
                return oc1132_setlowpass_epilogue(c, frame);
            }
            if back_edge_unwinds(c, OC1132_SETLOWPASS_HEAD) {
                return Ok(());
            }
        }
    })
}

fn oc1132_setlowpass_epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 6;
    c.g[V0][0] = c.g[S0][0];
    frame.restore(c, S1, 0x20);
    frame.restore(c, S0, 0x30);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

// ------------------------------------- cFEStateRequestLine_TransitionIn

/// `int cFEStateRequestLine_TransitionIn(self, int first_time)` (0x196f50):
/// on the first transition in, refreshes the scroll arrows, the button text
/// and the help text; returns 0.
pub fn cfe_state_request_line_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        Frame::open(c, 0x20);
        let sq_at = sp(c).wrapping_add(0x10);
        c.sq(16, sq_at);
        let ra_at = sp(c);
        c.sd(ra_at, c.g[RA][0]);
        let skip = c.g[A1][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if !skip {
            c.cycles += 8;
            call_ptr(c, 0x0019_7db8, 0x0019_6f6c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0019_8340, 0x0019_6f78)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0019_81f8, 0x0019_6f84)?;
        }
        c.cycles += 5;
        let lq_at = sp(c).wrapping_add(0x10);
        c.lq(16, lq_at);
        c.g[V0][0] = 0;
        let ra_at = sp(c);
        c.g[RA][0] = c.ld(ra_at);
        let target = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        c.pc = target;
        Ok(())
    })
}

// ------------------------------------- cFEStateCharEquip_OnWidgetNotify

const OC1132_NOTIFY_START: u32 = 0x0019_9f20;

const OC1132_NOTIFY_END: u32 = 0x0019_a094;

/// Resume entry: the `return 1` of the scroll and highlightable cases (the
/// return address of `RefreshItemList`, and the target of an event check).
const OC1132_NOTIFY_RETURN_1: u32 = 0x0019_9f60;

/// The jump table of the widget kinds (0..9) in the game image.
pub(crate) const OC1132_NOTIFY_TABLE: u32 = 0x0046_08b0;

/// Equip-screen fields: item count, items (4 bytes each), selected widget,
/// mode, cost limits, prices, scroll position, equip-mode switch.
pub(crate) const OC1132_EQUIP_ITEM_COUNT: u32 = 0x958;

pub(crate) const OC1132_EQUIP_ITEMS: u32 = 0x124;

pub(crate) const OC1132_EQUIP_SELECTED: u32 = 0x5c;

pub(crate) const OC1132_EQUIP_MODE: u32 = 0xc8;

pub(crate) const OC1132_EQUIP_LIMIT: u32 = 0xc0;

pub(crate) const OC1132_EQUIP_BASE: u32 = 0xc4;

pub(crate) const OC1132_EQUIP_PRICES: u32 = 0xa78;

pub(crate) const OC1132_EQUIP_SCROLL: u32 = 0x120;

pub(crate) const OC1132_EQUIP_CAN_SWITCH: u32 = 0xae4;

/// `int cFEStateCharEquip_OnWidgetNotify(self, int, int kind, int index)`
/// (0x199f20): the equip screen's widget notifications. Kinds 2 and 3
/// (scrolling) refresh the item list and return 1; 0 asks whether the item
/// is selectable (flag 4); 1 whether the index is highlightable; 6 (next)
/// checks the item can be bought (flag 0x20, cost, funds): 0x101 ok,
/// 0x110 refuse; 7 (previous) steps the scroll position back; 8 (option)
/// toggles buy/equip mode when allowed; 9 returns 0x100; others 0x101.
///
/// The original is a `match pc` dispatcher (a jump table over the kind); the
/// only back edge is the highlightable case, so `0x199f60` is its resume
/// entry (also the return address of the item list refresh).
pub fn cfe_state_char_equip_on_widget_notify(c: &mut Cpu) {
    let resumed = match c.pc {
        OC1132_NOTIFY_START => false,
        OC1132_NOTIFY_RETURN_1 => true,
        pc => {
            c.jump_out(pc, OC1132_NOTIFY_START, OC1132_NOTIFY_END);
            return;
        }
    };
    guest(c, |c| {
        let frame = Frame { size: 0x10 };
        if resumed {
            return oc1132_notify_return_1(c, frame);
        }
        c.cycles += 5;
        Frame::open(c, frame.size);
        c.g[T0][0] = c.g[A0][0];
        let kind = c.g[A2][0];
        c.g[V0][0] = (kind < 0xa) as u64;
        frame.save_ra(c, 0);
        if kind >= 0xa {
            return oc1132_notify_return_101(c, frame);
        }
        c.cycles += 7;
        c.g[V0][0] = 0x46_0000;
        c.g[V1][0] = sx((kind as u32) << 2);
        c.g[V0][0] = sx(0x46_0000u32.wrapping_add(0x8b0));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let entry = Addr(c.g[V1][0] as u32).lw(c);
        c.g[A0][0] = entry;
        let this = c.g[T0][0] as u32;
        let index = c.g[A3][0];
        match entry as u32 {
            // Scroll up/down.
            0x0019_9f50 => {
                c.cycles += 4;
                c.g[A0][0] = c.g[T0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                call_ptr(c, 0x0019_b618, OC1132_NOTIFY_RETURN_1)?;
                oc1132_notify_return_1(c, frame)
            }
            // Is the item selectable?
            0x0019_9f68 => {
                c.cycles += 4;
                let count = Addr(this.wrapping_add(OC1132_EQUIP_ITEM_COUNT)).lw(c);
                c.g[V0][0] = ((index as i64) < (count as i64)) as u64;
                let inside = c.g[V0][0] != 0;
                c.g[V0][0] = sx((index as u32) << 2);
                if !inside {
                    return oc1132_notify_return_0(c, frame);
                }
                c.cycles += 6;
                c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
                let item = Addr((c.g[V0][0] as u32).wrapping_add(OC1132_EQUIP_ITEMS)).lw(c);
                c.g[V1][0] = item;
                let flags = Addr((item as u32).wrapping_add(0x34)).lw(c);
                c.g[V0][0] = flags & 0x4;
                c.g[V0][0] = (0 < c.g[V0][0]) as u64;
                oc1132_notify_exit(c, frame)
            }
            // Is the index highlightable?
            0x0019_9f90 => {
                c.cycles += 2;
                let zero = c.g[A3][0] == 0;
                c.g[V0][0] = 1;
                if zero {
                    return oc1132_notify_exit(c, frame);
                }
                c.cycles += 3;
                let count = Addr(this.wrapping_add(OC1132_EQUIP_ITEM_COUNT)).lw(c);
                c.g[V0][0] = count;
                let empty = count == 0;
                c.g[V0][0] = ((index as i64) < (count as i64)) as u64;
                if empty {
                    if back_edge_unwinds(c, OC1132_NOTIFY_RETURN_1) {
                        return Ok(());
                    }
                    return oc1132_notify_return_1(c, frame);
                }
                c.cycles += 2;
                let inside = c.g[V0][0] != 0;
                c.g[V0][0] = 1;
                if inside {
                    return oc1132_notify_exit(c, frame);
                }
                oc1132_notify_return_0(c, frame)
            }
            // Next: can the item be bought?
            0x0019_9fb4 => {
                c.cycles += 4;
                let selected = Addr(this.wrapping_add(OC1132_EQUIP_SELECTED)).lw(c);
                c.g[A2][0] = selected;
                let current = Addr((selected as u32).wrapping_add(0x95)).lbu(c);
                c.g[A0][0] = current;
                let negative = (current as i64) < 0;
                c.g[V0][0] = 0x110;
                if negative {
                    return oc1132_notify_exit(c, frame);
                }
                c.cycles += 4;
                let count = Addr(this.wrapping_add(OC1132_EQUIP_ITEM_COUNT)).lw(c);
                c.g[V0][0] = ((current as i64) < (count as i64)) as u64;
                let inside = c.g[V0][0] != 0;
                c.g[V0][0] = sx((current as u32) << 2);
                if !inside {
                    return oc1132_notify_return_110(c, frame);
                }
                c.cycles += 6;
                c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
                let item = Addr((c.g[V0][0] as u32).wrapping_add(OC1132_EQUIP_ITEMS)).lw(c);
                c.g[A1][0] = item;
                let flags = Addr((item as u32).wrapping_add(0x34)).lw(c);
                c.g[V1][0] = flags & 0x20;
                let owned = c.g[V1][0] != 0;
                c.g[V0][0] = 0x101;
                if owned {
                    return oc1132_notify_exit(c, frame);
                }
                c.cycles += 4;
                let mode = Addr(this.wrapping_add(OC1132_EQUIP_MODE)).lw(c);
                c.g[V0][0] = mode ^ 0x1;
                if c.g[V0][0] != 0 {
                    // Not store mode: the price table (0x199ffc..0x19a018).
                    let offset = Addr((selected as u32).wrapping_add(0x98)).lbu(c);
                    c.g[V0][0] = offset;
                    return oc1132_notify_check_funds(c, frame);
                }
                c.cycles += 3;
                let cost = Addr((item as u32).wrapping_add(0xe)).lh(c);
                c.g[V1][0] = cost;
                let free = (cost as i64) <= 0;
                c.g[V0][0] = 0xa;
                if !free {
                    c.cycles += 1;
                    c.mult(0, 3, 3, 2);
                }
                c.cycles += 3;
                let limit = Addr(this.wrapping_add(OC1132_EQUIP_LIMIT)).lw(c);
                c.g[V0][0] = ((c.g[V1][0] as i64) < (limit as i64)) as u64;
                oc1132_notify_return_101(c, frame)
            }
            // Previous: step the scroll position back.
            0x0019_a048 => {
                c.cycles += 3;
                let scroll = Addr(this.wrapping_add(OC1132_EQUIP_SCROLL)).lw(c);
                c.g[V1][0] = scroll;
                let at_start = (scroll as i64) <= 0;
                c.g[V1][0] = sx((scroll as u32).wrapping_sub(1));
                if at_start {
                    return oc1132_notify_return_101(c, frame);
                }
                c.cycles += 3;
                c.g[V0][0] = 1;
                let stepped = c.g[V1][0];
                Addr(this.wrapping_add(OC1132_EQUIP_SCROLL)).sw(c, stepped);
                oc1132_notify_exit(c, frame)
            }
            // Option: switch the equip mode when allowed.
            0x0019_a060 => {
                c.cycles += 3;
                let allowed = Addr(this.wrapping_add(OC1132_EQUIP_CAN_SWITCH)).lw(c);
                c.g[V0][0] = allowed;
                let denied = allowed == 0;
                c.g[V0][0] = 0x100;
                if denied {
                    return oc1132_notify_exit(c, frame);
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[T0][0];
                call_ptr(c, 0x0019_a3d0, 0x0019_a074)?;
                c.g[V0][0] = 0x101;
                oc1132_notify_exit(c, frame)
            }
            // Misc.
            0x0019_a07c => {
                c.cycles += 2;
                c.g[V0][0] = 0x100;
                oc1132_notify_exit(c, frame)
            }
            // Kinds 4 and 5 go straight to the default.
            0x0019_a084 => oc1132_notify_return_101(c, frame),
            target => {
                c.jump_out(target, OC1132_NOTIFY_START, OC1132_NOTIFY_END);
                Ok(())
            }
        }
    })
}

/// 0x199ffc..0x19a040 tail of "next": funds check with the price table (v0 =
/// the widget's scroll offset on entry).
fn oc1132_notify_check_funds(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    let this = c.g[T0][0] as u32;
    c.cycles += 10;
    let base = Addr(this.wrapping_add(OC1132_EQUIP_BASE)).lw(c);
    c.g[A1][0] = base;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    let funds = Addr((c.g[GP][0] as u32).wrapping_sub(0x1818)).lw(c);
    c.g[A0][0] = funds;
    c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
    let price = Addr((c.g[V0][0] as u32).wrapping_add(OC1132_EQUIP_PRICES)).lw(c);
    c.g[V1][0] = price;
    c.g[V1][0] = sx((price as u32).wrapping_add(base as u32));
    c.g[A0][0] = ((funds as i64) < (c.g[V1][0] as i64)) as u64;
    let short = c.g[A0][0] == 0;
    c.g[V0][0] = 0x101;
    if short {
        return oc1132_notify_exit(c, frame);
    }
    oc1132_notify_return_110(c, frame)
}

/// 0x19a040: return 0x110.
fn oc1132_notify_return_110(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 2;
    c.g[V0][0] = 0x110;
    oc1132_notify_exit(c, frame)
}

/// 0x199fac: return 0.
fn oc1132_notify_return_0(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 2;
    c.g[V0][0] = 0;
    oc1132_notify_exit(c, frame)
}

/// 0x199f60: return 1.
fn oc1132_notify_return_1(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 2;
    c.g[V0][0] = 1;
    oc1132_notify_exit(c, frame)
}

/// 0x19a084: return 0x101.
fn oc1132_notify_return_101(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 1;
    c.g[V0][0] = 0x101;
    oc1132_notify_exit(c, frame)
}

/// 0x19a088: the epilogue.
fn oc1132_notify_exit(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::fakes::{self, check_reaching, covered, machine};
    use crate::difftest::{check, check_lohi, elf_image, fill, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // ---------------------------------------------------------- SND_SetLowpass

    crate::difftest::events::driven!(oc1132_gen_set_lowpass, ee::s13::f_003b8978__generated);

    crate::difftest::events::driven!(oc1132_hand_set_lowpass, snd_set_lowpass);

    /// The SND_SetVolume fixture (channels in the driver state block, some
    /// sharing a handle, no voices listed) with some channels filtered (flag
    /// 4, so the real `SND_ChannelApplyLowpass` computes the cutoff but has
    /// no voice to set) and a random output rate.
    fn oc1132_lowpass_fixture(c: &mut Cpu, r: &mut Rng) {
        use crate::game::lib::snd::tests::set_volume_setup;
        set_volume_setup(c, r);
        let state = 0x0050_a8e8u32;
        let n = c.bus.read16(state + 0x17e) as u32;
        let channels = c.bus.read32(state + 0x1f0);
        c.bus.write16(state + 0x2a, r.next_u32() as u16);
        for i in 0..n {
            let flags = if r.next_u32() % 2 == 0 { 0xc } else { 0x8 };
            c.bus.write16(channels + i * 0x8c + 0x24, flags);
        }
    }

    /// SND_SetLowpass: bad handles (loop skipped), single and multi-channel
    /// handles, filtered or not; and with events at the loop's back edge
    /// (delivered or unwinding, then resumed at the loop head).
    #[test]
    fn snd_set_lowpass_matches() {
        check(
            ee::s13::f_003b8978__generated,
            snd_set_lowpass,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8978, 0);
                c.pc = 0x003b_8978;
                oc1132_lowpass_fixture(c, &mut r);
            },
        );
        check_events(
            oc1132_gen_set_lowpass,
            oc1132_hand_set_lowpass,
            600,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8979, 0);
                oc1132_lowpass_fixture(c, &mut r);
                schedule(c, &mut r, seed, 0x003b_8978)
            },
        );
    }

    // ------------------------------------------ fakes for the front-end callees

    /// Gate word of the oc1132 fakes (a fixture address of this machine only).
    const OC1132_GATE: u32 = DATA + 0x0020_1320;

    const OC1132_MAGIC: u32 = 0x0c11_3200;

    static OC1132_PREV_197DB8: AtomicUsize = AtomicUsize::new(0);

    static OC1132_PREV_198340: AtomicUsize = AtomicUsize::new(0);

    static OC1132_PREV_1981F8: AtomicUsize = AtomicUsize::new(0);

    static OC1132_PREV_19B618: AtomicUsize = AtomicUsize::new(0);

    static OC1132_PREV_19A3D0: AtomicUsize = AtomicUsize::new(0);

    fn oc1132_gated(c: &mut Cpu, target: u32, prev: &AtomicUsize, generated: fn(&mut Cpu)) {
        if c.bus.read32(OC1132_GATE) != OC1132_MAGIC {
            return fakes::fall_back(c, prev, generated);
        }
        fakes::fake_call(c, target);
    }

    fn oc1132_fake_197db8(c: &mut Cpu) {
        oc1132_gated(c, 0x0019_7db8, &OC1132_PREV_197DB8, ee::s02::f_00197db8__generated)
    }

    fn oc1132_fake_198340(c: &mut Cpu) {
        oc1132_gated(c, 0x0019_8340, &OC1132_PREV_198340, ee::s03::f_00198340__generated)
    }

    fn oc1132_fake_1981f8(c: &mut Cpu) {
        oc1132_gated(c, 0x0019_81f8, &OC1132_PREV_1981F8, ee::s02::f_001981f8__generated)
    }

    fn oc1132_fake_19b618(c: &mut Cpu) {
        oc1132_gated(c, 0x0019_b618, &OC1132_PREV_19B618, ee::s03::f_0019b618__generated)
    }

    fn oc1132_fake_19a3d0(c: &mut Cpu) {
        oc1132_gated(c, 0x0019_a3d0, &OC1132_PREV_19A3D0, ee::s03::f_0019a3d0__generated)
    }

    fn oc1132_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00197DB8),
            oc1132_fake_197db8,
            &OC1132_PREV_197DB8,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_00198340),
            oc1132_fake_198340,
            &OC1132_PREV_198340,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_001981F8),
            oc1132_fake_1981f8,
            &OC1132_PREV_1981F8,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_0019B618),
            oc1132_fake_19b618,
            &OC1132_PREV_19B618,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_0019A3D0),
            oc1132_fake_19a3d0,
            &OC1132_PREV_19A3D0,
        );
    }

    // ----------------------------------- cFEStateRequestLine_TransitionIn

    /// cFEStateRequestLine_TransitionIn: first transition in (three calls:
    /// scroll arrows, button text, help text; any may unwind) or not (no
    /// calls).
    #[test]
    fn cfe_state_request_line_transition_in_matches() {
        oc1132_install_fakes();
        covered(
            ee::s02::f_00196f50__generated,
            cfe_state_request_line_transition_in,
            90,
            &|c, seed| {
                machine(c, seed, 0x196f50, 3);
                c.bus.write32(OC1132_GATE, OC1132_MAGIC);
                c.pc = 0x0019_6f50;
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(seed | 1) };
            },
        );
    }

    // ------------------------------------ cFEStateCharEquip_OnWidgetNotify

    const OC1132_SELF: u32 = DATA + 0x2000;

    const OC1132_ITEMS: u32 = DATA + 0x8000;

    const OC1132_SELECTED: u32 = DATA + 0x9000;

    /// The equip screen: 0..11 items with random flags (4: selectable, 0x20:
    /// owned), costs, prices, a selected widget whose scroll position is
    /// inside or outside the item list, store or equip mode. Kind and index
    /// cycle through every case (kinds 10 and 11 are out of the table).
    fn oc1132_notify_fixture(c: &mut Cpu, seed: u32, with_fakes: bool) -> Rng {
        let mut r = machine(c, seed, 0x199f20, if with_fakes { 1 } else { 0 });
        elf_image(c);
        c.bus.write32(OC1132_GATE, OC1132_MAGIC);
        c.pc = 0x0019_9f20;
        fill(c, &mut r, OC1132_SELF, 0xb00 / 4);
        fill(c, &mut r, OC1132_SELECTED, 0x100 / 4);
        let count = if seed % 7 == 0 { 0 } else { 1 + r.next_u32() % 9 };
        c.bus.write32(OC1132_SELF + OC1132_EQUIP_ITEM_COUNT, count);
        for i in 0..12u32 {
            let item = OC1132_ITEMS + 0x40 * i;
            fill(c, &mut r, item, 0x10);
            let flags = (r.next_u32() & 0x3f) & ![0, 4, 0x20][(r.next_u32() % 3) as usize];
            c.bus.write32(item + 0x34, flags);
            let cost = [0u16, 1, 5, 0x40, 0xffff][(r.next_u32() % 5) as usize];
            c.bus.write16(item + 0xe, cost);
            c.bus.write32(OC1132_SELF + OC1132_EQUIP_ITEMS + 4 * i, item);
        }
        c.bus.write32(OC1132_SELF + OC1132_EQUIP_SELECTED, OC1132_SELECTED);
        c.bus.write8(OC1132_SELECTED + 0x95, (r.next_u32() % 12) as u8);
        c.bus.write8(OC1132_SELECTED + 0x98, (r.next_u32() % 4) as u8);
        c.bus
            .write32(OC1132_SELF + OC1132_EQUIP_MODE, (seed / 3 % 2) as u32);
        c.bus.write32(OC1132_SELF + OC1132_EQUIP_LIMIT, r.next_u32() % 0x500);
        c.bus.write32(OC1132_SELF + OC1132_EQUIP_BASE, r.next_u32() % 0x100);
        c.bus.write32(
            OC1132_SELF + OC1132_EQUIP_SCROLL,
            [0u32, 1, 5, 0xffff_ffff][(r.next_u32() % 4) as usize],
        );
        c.bus.write32(
            OC1132_SELF + OC1132_EQUIP_CAN_SWITCH,
            (r.next_u32() % 2) as u32,
        );
        for i in 0..12u32 {
            c.bus
                .write32(OC1132_SELF + OC1132_EQUIP_PRICES - 4 * 3 + 4 * i, r.next_u32() % 0x200);
        }
        // The funds (gp - 0x1818).
        c.bus
            .write32(GAME_GP.wrapping_sub(0x1818), r.next_u32() % 0x800);
        r
    }

    fn oc1132_notify_regs(c: &mut Cpu, seed: u32, r: &mut Rng) {
        c.g[GP][0] = GAME_GP as u64;
        c.g[A0][0] = sx(OC1132_SELF);
        c.g[A2][0] = (seed % 12) as u64;
        // The index is valid for the selectable case; the highlightable one
        // also sees zero and negative ones.
        let index = match r.next_u32() % 4 {
            0 => 0,
            1 => 1 + r.next_u32() % 11,
            2 => r.next_u32() % 3,
            _ => (c.bus.read32(OC1132_SELF + OC1132_EQUIP_ITEM_COUNT)).wrapping_add(r.next_u32() % 3),
        };
        let index = if seed % 12 == 1 && r.next_u32() % 5 == 0 {
            sx(0xffff_fffe)
        } else if seed % 12 == 0 {
            sx(index.min(c.bus.read32(OC1132_SELF + OC1132_EQUIP_ITEM_COUNT).saturating_sub(1)))
        } else {
            sx(index)
        };
        c.g[A3][0] = index;
    }

    /// cFEStateCharEquip_OnWidgetNotify, all branches: every kind, the
    /// out-of-table kinds, item and scroll variations; LO/HI compared (the
    /// "next" case multiplies the cost by ten).
    #[test]
    fn cfe_state_char_equip_on_widget_notify_matches() {
        oc1132_install_fakes();
        check_lohi(
            ee::s03::f_00199f20__generated,
            cfe_state_char_equip_on_widget_notify,
            480,
            &|c, seed| {
                let mut r = oc1132_notify_fixture(c, seed, false);
                oc1132_notify_regs(c, seed, &mut r);
            },
        );
        // The two calls (item list refresh, buy/equip toggle) are reached,
        // some of them unwinding.
        check_reaching(
            ee::s03::f_00199f20__generated,
            cfe_state_char_equip_on_widget_notify,
            240,
            &[0x0019_9f60, 0x0019_a074],
            &|c, seed| {
                let mut r = oc1132_notify_fixture(c, seed, true);
                oc1132_notify_regs(c, seed, &mut r);
                if seed % 12 != 2 && seed % 12 != 3 && seed % 12 != 8 {
                    c.g[A2][0] = [2u64, 3, 8][(seed % 3) as usize];
                }
                if c.g[A2][0] == 8 {
                    c.bus.write32(OC1132_SELF + OC1132_EQUIP_CAN_SWITCH, 1);
                }
            },
        );
    }

    crate::difftest::events::driven!(oc1132_gen_notify, ee::s03::f_00199f20__generated);

    crate::difftest::events::driven!(oc1132_hand_notify, cfe_state_char_equip_on_widget_notify);

    /// The highlightable case with an empty list (the branch that checks
    /// events), with events delivered or unwinding there; the unwound call
    /// resumes at 0x199f60.
    #[test]
    fn cfe_state_char_equip_on_widget_notify_events() {
        check_events(oc1132_gen_notify, oc1132_hand_notify, 300, &|c, seed| {
            let mut r = oc1132_notify_fixture(c, seed, false);
            c.bus.write32(OC1132_SELF + OC1132_EQUIP_ITEM_COUNT, 0);
            let mode = schedule(c, &mut r, seed, 0x0019_9f20);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = sx(OC1132_SELF);
            c.g[A2][0] = 1;
            c.g[A3][0] = 1 + (seed % 5) as u64;
            mode
        });
    }
}
