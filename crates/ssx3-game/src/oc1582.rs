//! Staging module oc1582 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1582`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_2f20, lbl_00392f20_div8_tail, "lbl_00392F20"),
    (0x0039_2f38, lbl_00392f38_div4_tail, "lbl_00392F38"),
    (0x0039_2f50, lbl_00392f50_div2_tail, "lbl_00392F50"),
    (0x0039_2f5c, lbl_00392f5c_record_update, "lbl_00392F5C"),
    (0x0039_f290, ui_state_mgr_push_popup, "UIStateMgr_PushPopup"),
    (0x003a_38b8, lbl_003a38b8_scale12_tail, "lbl_003A38B8"),
    (0x003a_3994, lbl_003a3994_fscale_store_tail, "lbl_003A3994"),
    (0x003a_39a4, lbl_003a39a4_fscale_store_tail, "lbl_003A39A4"),
];

/// `0x00392f20` (no name): `A3 = A3 / 8` (signed, rounds toward zero: the
/// `A3 + 7` bias applies to negative values), then a tail jump to the shared
/// body at 0x392f5c.
pub fn lbl_00392f20_div8_tail(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x7));
    c.g[V0][0] = ((-1i64) < (c.g[A3][0] as i64)) as u64;
    if c.g[V0][0] != 0 {
        c.g[V1][0] = c.g[A3][0];
    }
    c.g[A3][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
    c.pc = 0x0039_2f5c;
    ee::s13::f_00392f5c(c);
}

/// `0x00392f38` (no name): `A3 = A3 / 4` (signed, rounds toward zero), then a
/// tail jump to the shared body at 0x392f5c.
pub fn lbl_00392f38_div4_tail(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3));
    c.g[V0][0] = ((-1i64) < (c.g[A3][0] as i64)) as u64;
    if c.g[V0][0] != 0 {
        c.g[V1][0] = c.g[A3][0];
    }
    c.g[A3][0] = sx(((c.g[V1][0] as u32) as i32 >> 2) as u32);
    c.pc = 0x0039_2f5c;
    ee::s13::f_00392f5c(c);
}

/// `0x00392f50` (no name): `A3 = A3 / 2` (signed, rounds toward zero), then a
/// tail jump to the shared body at 0x392f5c.
pub fn lbl_00392f50_div2_tail(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[A3][0] as u32) >> 31);
    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A3][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
    c.pc = 0x0039_2f5c;
    ee::s13::f_00392f5c(c);
}

/// `0x00392f5c` (no name): the shared record update of the three divide
/// wrappers above. It reads the record at T0 (+0x8 a store pointer, +0x18 and
/// +0x1c the divisor and the clamp operand, +0x20, +0x24, +0x28 the flags),
/// divides A3 by the record's value, multiplies the quotient back, and stores
/// the packed word at the pointer and at `ptr + 8`. Leaves LO/HI as the divide
/// and multiply left them.
pub fn lbl_00392f5c_record_update(c: &mut Cpu) {
    let rec = c.g[T0][0] as u32;

    // 'b_392f84: clamp the divisor and divide.
    c.cycles += 10;
    c.g[A0][0] = Addr(rec.wrapping_add(0x1c)).lw(c);
    c.g[V0][0] = Addr(rec.wrapping_add(0x18)).lw(c);
    c.g[A2][0] = c.g[V0][0];
    c.g[V1][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[V1][0] != 0 {
        c.g[V0][0] = c.g[A0][0];
    }
    c.g[A1][0] = ((c.g[A2][0] as i64) < (c.g[A0][0] as i64)) as u64;
    c.div(0, 7, 2);
    if c.g[A1][0] == 0 {
        c.g[A0][0] = c.g[A2][0];
    }

    // 'b_392f94: the quotient times the operand (A3 = V1 * A0, LO/HI set).
    c.cycles += 3;
    c.g[V1][0] = c.lo[0];
    c.mult(0, 7, 3, 4);

    // 'b_392ff4 / 'b_392ff0: pack the word.
    c.cycles += 20;
    c.g[V1][0] = Addr(rec.wrapping_add(0x1c)).lw(c);
    c.g[A1][0] = 0x0100_0000;
    c.g[V0][0] = Addr(rec.wrapping_add(0x18)).lw(c);
    c.g[A3][0] &= 0xff;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 8);
    c.g[A0][0] = Addr(rec.wrapping_add(0x8)).lw(c);
    c.g[V0][0] |= c.g[V1][0];
    Addr(rec.wrapping_add(0x2c)).sw(c, 0);
    c.g[V0][0] |= c.g[A1][0];
    c.g[V0][0] <<= 32;
    c.g[A1][0] = sx((c.g[A3][0] as u32) << 16);
    let (dst, word) = (c.g[A0][0] as u32, c.g[V0][0]);
    Addr(dst).sd(c, word);
    c.g[V1][0] = Addr(rec.wrapping_add(0x8)).lw(c);
    c.g[V0][0] = Addr(rec.wrapping_add(0x20)).lw(c);
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = Addr(rec.wrapping_add(0x28)).lw(c);
    c.g[V0][0] |= 0x60;
    c.g[A2][0] = Addr(rec.wrapping_add(0x24)).lw(c);
    let flag_zero = c.g[A0][0] == 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32) << 24);
    if flag_zero {
        c.cycles += 1;
        c.g[V0][0] = c.g[A2][0] | c.g[A1][0];
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.g[A1][0] | 0x8000;
        c.g[V0][0] = c.g[A2][0] | c.g[V0][0];
    }

    // Common exit of 'b_392ff4.
    c.cycles += 4;
    c.g[V0][0] |= c.g[V1][0];
    c.g[V0][0] <<= 32;
    let ra = c.g[RA][0] as u32;
    let (dst, word) = (c.g[A3][0] as u32, c.g[V0][0]);
    Addr(dst).sd(c, word);
    c.pc = ra;
}

/// `UIStateMgr_PushPopup` (0x39f290, `cUIStateStack::pushPopup`): pushes the
/// popup object (S1 = the list head in A0) onto the state's list (A1). A state
/// whose popup slot (+0x20) is empty first takes the list's first element
/// there; the popup's +0x1c word gets its flag bits rebuilt, and the state's
/// +0x1c word receives the popup flags before `UIList_PushFront` runs.
pub fn ui_state_mgr_push_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x20)).lw(c);
        c.g[S1][0] = c.g[A0][0];

        // Empty popup slot: take the list's first element (a call).
        let mut skip_flags = false;
        if c.g[A2][0] == 0 {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: 0x0039_7840,
                f: ee::s13::f_00397840,
            }
            .call(c, 0x0039_f2b8)?;
            c.g[A2][0] = c.g[V0][0];
            if c.g[A2][0] == 0 {
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x1c)).lw(c);
                skip_flags = true;
            }
        }

        if !skip_flags {
            c.cycles += 13;
            let popup = c.g[A2][0] as u32;
            c.g[V1][0] = Addr(popup.wrapping_add(0x1c)).lw(c);
            c.g[A0][0] = 0xffff_ffff_ffff_ffdf;
            c.g[A1][0] = 0xffff_ffff_ffff_fff9;
            c.g[V1][0] &= c.g[A0][0];
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 1);
            c.g[V0][0] &= 0x3;
            c.g[V1][0] &= c.g[A1][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.g[V0][0] &= 0x3;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
            c.g[V1][0] |= c.g[V0][0];
            let flags = c.g[V1][0];
            Addr(popup.wrapping_add(0x1c)).sw(c, flags);
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x1c)).lw(c);
        }

        // 'b_39f2f8 exit: push the popup on the list.
        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[V0][0] |= 0x21;
        c.g[RA][0] = sx(0x0039_f30c);
        let (dst, word) = ((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        Addr(dst).sw(c, word);
        Callee {
            addr: 0x0039_78f0,
            f: ee::s13::f_003978f0,
        }
        .call(c, 0x0039_f30c)?;

        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `0x003a38b8` (no name): `V0 = A3 * 12` (LO/HI set), `V1 = word at T0+0x80`,
/// then a tail jump into the function at 0x3a38f8 (a resume point of the
/// dispatcher at 0x3a38f0).
pub fn lbl_003a38b8_scale12_tail(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0xc;
    c.g[V1][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x80)).lw(c);
    c.mult(0, 2, 7, 2);
    c.pc = 0x003a_38f8;
    call_indirect(c, 0x003a_38f8);
}

/// `0x003a3994` (no name): `f0 = f1 * *(GP-0x24e8)`, stored at T0+0x1c, then a
/// tail jump to 0x3a39e0.
pub fn lbl_003a3994_fscale_store_tail(c: &mut Cpu) {
    c.cycles += 4;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x24e8));
    c.mul_s(0, 1, 0);
    c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x1c));
    c.pc = 0x003a_39e0;
    ee::s13::f_003a39e0(c);
}

/// `0x003a39a4` (no name): `f0 = f1 * *(GP-0x24e4)`, stored at T0+0x20, then a
/// tail jump to 0x3a39e0.
pub fn lbl_003a39a4_fscale_store_tail(c: &mut Cpu) {
    c.cycles += 4;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x24e4));
    c.mul_s(0, 1, 0);
    c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x20));
    c.pc = 0x003a_39e0;
    ee::s13::f_003a39e0(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check_lohi, check_paths_lohi, Rng, DATA, GAME_GP};

    /// The record at `DATA + 0x1000` that `lbl_00392f5c_record_update` reads
    /// through T0. The stores go to `DATA + 0x2000` (and `+8` of it). The
    /// divisor is non-zero; the flag at +0x28 is zero on every third seed.
    fn oc1582_write_record(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let rec = DATA + 0x1000;
        c.g[T0][0] = rec as u64;
        c.bus.write32(rec + 0x8, DATA + 0x2000);
        c.bus.write32(rec + 0x18, (r.next_u32() % 40) + 1);
        let operand = if seed % 2 == 0 {
            r.next_u32() % 40
        } else {
            r.next_u32()
        };
        c.bus.write32(rec + 0x1c, operand);
        c.bus.write32(rec + 0x20, r.next_u32());
        c.bus.write32(rec + 0x24, r.next_u32());
        c.bus.write32(rec + 0x28, if seed % 3 == 0 { 0 } else { r.next_u32() });
    }

    /// A3 from a few sign classes: large, small positive, small negative.
    fn oc1582_a3(r: &mut Rng, seed: u32) -> u64 {
        let v = match seed % 3 {
            0 => r.next_u32() as i32,
            1 => (r.next_u32() % 300) as i32,
            _ => -((r.next_u32() % 300) as i32) - 1,
        };
        sx(v as u32)
    }

    #[test]
    fn lbl_00392f20_matches() {
        check_lohi(
            ee::s13::f_00392f20__generated,
            lbl_00392f20_div8_tail,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1582_2f20, 0);
                c.pc = 0x0039_2f20;
                oc1582_write_record(c, &mut r, seed);
                c.g[A3][0] = oc1582_a3(&mut r, seed);
            },
        );
    }

    #[test]
    fn lbl_00392f38_matches() {
        check_lohi(
            ee::s13::f_00392f38__generated,
            lbl_00392f38_div4_tail,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1582_2f38, 0);
                c.pc = 0x0039_2f38;
                oc1582_write_record(c, &mut r, seed);
                c.g[A3][0] = oc1582_a3(&mut r, seed);
            },
        );
    }

    #[test]
    fn lbl_00392f50_matches() {
        check_lohi(
            ee::s13::f_00392f50__generated,
            lbl_00392f50_div2_tail,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1582_2f50, 0);
                c.pc = 0x0039_2f50;
                oc1582_write_record(c, &mut r, seed);
                c.g[A3][0] = oc1582_a3(&mut r, seed);
            },
        );
    }

    /// Both arms of the flag test (38 and 40 cycles) must be taken.
    #[test]
    fn lbl_00392f5c_matches() {
        check_paths_lohi(
            ee::s13::f_00392f5c__generated,
            lbl_00392f5c_record_update,
            300,
            &[38, 40],
            &|c, seed| {
                let mut r = machine(c, seed, 0x1582_2f5c, 0);
                c.pc = 0x0039_2f5c;
                oc1582_write_record(c, &mut r, seed);
                c.g[A3][0] = sx(r.next_u32());
            },
        );
    }

    /// A list that is empty (popup slot zero, first element points to itself
    /// or to a second node) and one that already holds a popup. The callees
    /// (UIList_First, UIList_PushFront) are the real ones on DATA addresses.
    #[test]
    fn ui_state_mgr_push_popup_matches() {
        check_lohi(
            ee::s13::f_0039f290__generated,
            ui_state_mgr_push_popup,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1582_f290, 0);
                c.pc = 0x0039_f290;
                let state = DATA + 0x3000;
                let head = DATA + 0x3100;
                let node = DATA + 0x3200;
                let other = DATA + 0x3300;
                c.g[A0][0] = head as u64;
                c.g[A1][0] = state as u64;
                let popup = if seed % 3 == 0 { 0 } else { node };
                c.bus.write32(state + 0x20, popup);
                c.bus.write32(state + 0x1c, r.next_u32());
                c.bus.write32(head + 0x4, node);
                let next = if seed % 5 == 0 { node } else { other };
                c.bus.write32(node + 0x4, next);
                c.bus.write32(node, r.next_u32());
                c.bus.write32(node + 0x1c, r.next_u32());
                c.bus.write32(other, r.next_u32());
                c.bus.write32(other + 0x4, r.next_u32());
            },
        );
    }

    /// The tail jump into the dispatcher at 0x3a38f8; A1 = 0 makes the callee
    /// (0x3a39e0) return at once, and the store lands inside DATA.
    #[test]
    fn lbl_003a38b8_matches() {
        check_lohi(
            ee::s13::f_003a38b8__generated,
            lbl_003a38b8_scale12_tail,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1582_38b8, 0);
                c.pc = 0x003a_38b8;
                c.g[T0][0] = (DATA + 0x1000) as u64;
                c.bus.write32(DATA + 0x1000 + 0x80, DATA + 0x3000);
                c.g[A3][0] = (r.next_u32() % 0x100) as u64;
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn lbl_003a3994_matches() {
        check_lohi(
            ee::s13::f_003a3994__generated,
            lbl_003a3994_fscale_store_tail,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1582_3994, 0);
                c.pc = 0x003a_3994;
                c.bus.write32(GAME_GP.wrapping_sub(0x24e8), r.f32(1000.0));
                c.g[T0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn lbl_003a39a4_matches() {
        check_lohi(
            ee::s13::f_003a39a4__generated,
            lbl_003a39a4_fscale_store_tail,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1582_39a4, 0);
                c.pc = 0x003a_39a4;
                c.bus.write32(GAME_GP.wrapping_sub(0x24e4), r.f32(1000.0));
                c.g[T0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = 0;
            },
        );
    }
}
