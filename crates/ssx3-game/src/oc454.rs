//! Staging module oc454 (wave wave224): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc454`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_20c8, rider_uber_grind_sub_dtor, "RiderUberGrindSub_dtor"),
    (0x0014_2228, rider_wipeout_sub_dtor, "RiderWipeoutSub_dtor"),
    (0x0014_24d0, rider_finish_line_sub_dtor, "RiderFinishLineSub_dtor"),
    (0x0014_2630, rider_reset_sub_dtor, "RiderResetSub_dtor"),
    (0x0014_2790, rider_derived_dtor_458928, "Rider_DerivedDtor_458928"),
    (0x0014_28f0, rider_handplant_sub_dtor, "RiderHandplantSub_dtor"),
    (0x0014_41a0, danger_man_release_danger, "DangerMan_ReleaseDanger"),
    (0x0014_4630, rider_anim_ctor, "RiderAnim_ctor"),
    (0x0014_48b8, rider_tuning_static_init_thunk, "RiderTuning_StaticInitThunk"),
    (0x0014_5870, be_new_race_int_map_event_type, "BENewRaceInt_MapEventType"),
];

/// The table base the generated code builds with `lui 0x46` (`0x460000`):
/// the `Rider*Sub_dtor` bodies copy their constants from `HI - k`.
const HI: u32 = 0x0046_0000;

/// The shape of one `Rider*Sub_dtor` body. The six bodies run the same code;
/// they differ in the constants below, and their loop heads and return
/// addresses sit at fixed offsets from their entry.
#[derive(Clone, Copy)]
pub(crate) struct RiderSubDtorShape {
    /// The function's entry address.
    pub(crate) entry: u32,
    /// `HI - k1` is the first constant table (0x40 bytes); the second one
    /// sits `0x48` above it (`HI - (k1 - 0x48)`, 0xa8 bytes).
    pub(crate) k1: u32,
    /// Offset in `this` of the sub-object pointer the body writes into.
    pub(crate) field: u32,
    /// The `- kc` applied to that pointer before the frame offset is taken.
    pub(crate) kc: u32,
}

pub(crate) const RIDER_UBER_GRIND_SUB_DTOR: RiderSubDtorShape = RiderSubDtorShape {
    entry: 0x0014_20c8,
    k1: 0x72f8,
    field: 0x8,
    kc: 0x10,
};

pub(crate) const RIDER_WIPEOUT_SUB_DTOR: RiderSubDtorShape = RiderSubDtorShape {
    entry: 0x0014_2228,
    k1: 0x73f0,
    field: 0x80,
    kc: 0x90,
};

pub(crate) const RIDER_FINISH_LINE_SUB_DTOR: RiderSubDtorShape = RiderSubDtorShape {
    entry: 0x0014_24d0,
    k1: 0x74e8,
    field: 0x4,
    kc: 0x10,
};

pub(crate) const RIDER_RESET_SUB_DTOR: RiderSubDtorShape = RiderSubDtorShape {
    entry: 0x0014_2630,
    k1: 0x75e0,
    field: 0x8,
    kc: 0x10,
};

pub(crate) const RIDER_DERIVED_DTOR_458928: RiderSubDtorShape = RiderSubDtorShape {
    entry: 0x0014_2790,
    k1: 0x76d8,
    field: 0x0,
    kc: 0x10,
};

pub(crate) const RIDER_HANDPLANT_SUB_DTOR: RiderSubDtorShape = RiderSubDtorShape {
    entry: 0x0014_28f0,
    k1: 0x77d0,
    field: 0x14,
    kc: 0x20,
};

/// `lw this+field`: the sub-object pointer the body reads (re-read at the
/// points the generated code reads it).
fn rider_sub_ptr(c: &mut Cpu, s: RiderSubDtorShape) -> u64 {
    let a = (c.g[S0][0] as u32).wrapping_add(s.field);
    Addr(a).lw(c)
}

/// The shared `Rider*Sub_dtor` body (`this` in a0, flags in a1: bit 1 calls
/// the base `Rider_dtor`, bit 0 frees `this`). With a1 == 0 it first copies the
/// two constant tables into the frame and points the sub-object at the copy.
pub(crate) fn rider_sub_dtor_body(c: &mut Cpu, s: RiderSubDtorShape) -> Flow {
    let tab_a = HI.wrapping_sub(s.k1);
    let k2 = s.k1 - 0x48;
    let tab_b = HI.wrapping_sub(k2);
    let head_copy = s.entry + 0x64;
    let head_tail = s.entry + 0xc8;
    let ret_dtor = s.entry + 0x134;
    let ret_free = s.entry + 0x148;

    c.cycles += 21;
    let frame = Frame::open(c, 0x130);
    c.g[V0][0] = HI as u64;
    frame.save(c, S0, 0x120);
    c.g[A2][0] = sx(tab_a);
    frame.save(c, S1, 0x110);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0x100);
    c.g[S1][0] = c.g[A1][0];
    c.g[A0][0] = HI as u64;
    c.g[A3][0] = c.g[V0][0];
    c.g[V1][0] = rider_sub_ptr(c, s);
    c.g[A0][0] = sx(HI.wrapping_sub(0x6470));
    let sub = c.g[V1][0] as u32;
    let table_a_ptr = c.g[A2][0];
    Addr(sub.wrapping_add(0x6e8)).sw(c, table_a_ptr);
    c.g[V1][0] = HI as u64;
    c.g[A1][0] = rider_sub_ptr(c, s);
    c.g[T2][0] = c.g[V1][0];
    c.g[V1][0] = sx(HI.wrapping_sub(k2));
    let sub = c.g[A1][0] as u32;
    let vtable = c.g[A0][0];
    Addr(sub.wrapping_add(0x6d0)).sw(c, vtable);
    c.g[V0][0] = rider_sub_ptr(c, s);
    let copy = c.g[S1][0] == 0;
    let sub = c.g[V0][0] as u32;
    let table_b_ptr = c.g[V1][0];
    Addr(sub.wrapping_add(0x6c0)).sw(c, table_b_ptr);

    if copy {
        c.cycles += 4;
        c.g[A1][0] = c.g[A2][0];
        c.g[V1][0] = c.g[SP][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
        c.g[T0][0] = sx(sp(c).wrapping_add(0x50));
        // Table A (0x40 bytes) into the frame at sp+0.
        loop {
            c.cycles += 11;
            let src = c.g[A1][0] as u32;
            c.g[A0][0] = Addr(src).ld(c);
            c.g[A2][0] = Addr(src.wrapping_add(0x8)).ld(c);
            c.g[T1][0] = Addr(src.wrapping_add(0x10)).ld(c);
            c.g[T3][0] = Addr(src.wrapping_add(0x18)).ld(c);
            let dst = c.g[V1][0] as u32;
            let (w0, w1, w2, w3) = (c.g[A0][0], c.g[A2][0], c.g[T1][0], c.g[T3][0]);
            Addr(dst).sd(c, w0);
            Addr(dst.wrapping_add(0x8)).sd(c, w1);
            Addr(dst.wrapping_add(0x10)).sd(c, w2);
            Addr(dst.wrapping_add(0x18)).sd(c, w3);
            c.g[A1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[A1][0] != c.g[V0][0];
            c.g[V1][0] = sx(dst.wrapping_add(0x20));
            if more {
                if event_due(c) {
                    deliver_event_here(c, head_copy);
                }
                continue;
            }
            break;
        }
        c.cycles += 14;
        c.g[V0][0] = rider_sub_ptr(c, s);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(s.k1));
        let src = c.g[A1][0] as u32;
        c.g[A2][0] = Addr(src).ld(c);
        let dst = c.g[V1][0] as u32;
        let w = c.g[A2][0];
        Addr(dst).sd(c, w);
        let sub = c.g[V0][0] as u32;
        let frame_ptr = c.g[SP][0];
        Addr(sub.wrapping_add(0x6e8)).sw(c, frame_ptr);
        c.g[A2][0] = sx((c.g[T2][0] as u32).wrapping_sub(k2));
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x8)).lhu(c);
        c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
        c.g[V0][0] = rider_sub_ptr(c, s);
        c.g[A1][0] = c.g[T0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(s.kc));
        c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
        let half = c.g[V1][0];
        Addr(sp(c).wrapping_add(0x8)).sh(c, half);
        // Table B (0xa0 bytes plus one doubleword) into the frame after A.
        loop {
            c.cycles += 11;
            let src = c.g[A2][0] as u32;
            c.g[V0][0] = Addr(src).ld(c);
            c.g[V1][0] = Addr(src.wrapping_add(0x8)).ld(c);
            c.g[A0][0] = Addr(src.wrapping_add(0x10)).ld(c);
            c.g[T3][0] = Addr(src.wrapping_add(0x18)).ld(c);
            let dst = c.g[A1][0] as u32;
            let (w0, w1, w2, w3) = (c.g[V0][0], c.g[V1][0], c.g[A0][0], c.g[T3][0]);
            Addr(dst).sd(c, w0);
            Addr(dst.wrapping_add(0x8)).sd(c, w1);
            Addr(dst.wrapping_add(0x10)).sd(c, w2);
            Addr(dst.wrapping_add(0x18)).sd(c, w3);
            c.g[A2][0] = sx(src.wrapping_add(0x20));
            let more = c.g[A2][0] != c.g[T1][0];
            c.g[A1][0] = sx(dst.wrapping_add(0x20));
            if more {
                if event_due(c) {
                    deliver_event_here(c, head_tail);
                }
                continue;
            }
            c.cycles += 10;
            let src = c.g[A2][0] as u32;
            c.g[V0][0] = Addr(src).ld(c);
            c.g[V1][0] = Addr(src.wrapping_add(0x8)).ld(c);
            let dst = c.g[A1][0] as u32;
            let (w0, w1) = (c.g[V0][0], c.g[V1][0]);
            Addr(dst).sd(c, w0);
            Addr(dst.wrapping_add(0x8)).sd(c, w1);
            c.g[V0][0] = sx(tab_b);
            c.g[A0][0] = rider_sub_ptr(c, s);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lhu(c);
            let frame_slot = c.g[T0][0];
            let sub = c.g[A0][0] as u32;
            Addr(sub.wrapping_add(0x6c0)).sw(c, frame_slot);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
            let half = c.g[V1][0];
            Addr(sp(c).wrapping_add(0x58)).sh(c, half);
            break;
        }
    }

    // Flags: bit 1 runs the base destructor, bit 0 frees the object.
    c.cycles += 3;
    let flags = c.g[S1][0];
    let no_base = (flags & 0x2) == 0;
    c.g[V0][0] = flags & 0x1;
    if !no_base {
        c.cycles += 4;
        c.g[A0][0] = rider_sub_ptr(c, s);
        c.g[A1][0] = 0;
        call_ptr(c, 0x0011_b978, ret_dtor)?;
        c.g[V0][0] = c.g[S1][0] & 0x1;
    }
    c.cycles += 2;
    if c.g[V0][0] == 0 {
        frame.restore(c, S0, 0x120);
    } else {
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_7e50, ret_free)?;
        frame.restore(c, S0, 0x120);
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x110);
    frame.restore_ra(c, 0x100);
    frame.close(c);
    Ok(())
}

/// `void RiderUberGrindSub_dtor(Rider* this, int flags)` (0x1420c8).
pub fn rider_uber_grind_sub_dtor(c: &mut Cpu) {
    guest(c, |c| rider_sub_dtor_body(c, RIDER_UBER_GRIND_SUB_DTOR));
}

/// `void RiderWipeoutSub_dtor(Rider* this, int flags)` (0x142228).
pub fn rider_wipeout_sub_dtor(c: &mut Cpu) {
    guest(c, |c| rider_sub_dtor_body(c, RIDER_WIPEOUT_SUB_DTOR));
}

/// `void RiderFinishLineSub_dtor(Rider* this, int flags)` (0x1424d0).
pub fn rider_finish_line_sub_dtor(c: &mut Cpu) {
    guest(c, |c| rider_sub_dtor_body(c, RIDER_FINISH_LINE_SUB_DTOR));
}

/// `void RiderResetSub_dtor(Rider* this, int flags)` (0x142630).
pub fn rider_reset_sub_dtor(c: &mut Cpu) {
    guest(c, |c| rider_sub_dtor_body(c, RIDER_RESET_SUB_DTOR));
}

/// `void Rider_DerivedDtor_458928(Rider* this, int flags)` (0x142790).
pub fn rider_derived_dtor_458928(c: &mut Cpu) {
    guest(c, |c| rider_sub_dtor_body(c, RIDER_DERIVED_DTOR_458928));
}

/// `void RiderHandplantSub_dtor(Rider* this, int flags)` (0x1428f0).
pub fn rider_handplant_sub_dtor(c: &mut Cpu) {
    guest(c, |c| rider_sub_dtor_body(c, RIDER_HANDPLANT_SUB_DTOR));
}

/// `void DangerMan_ReleaseDanger(DangerMan* m, Danger* d)` (0x1441a0): the
/// danger's state word (+8) becomes 2, and the manager's count (+0x704)
/// goes down by one.
pub fn danger_man_release_danger(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x2;
    let state = c.g[V0][0];
    Addr((c.g[A1][0] as u32).wrapping_add(0x8)).sw(c, state);
    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x704)).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
    let count = c.g[V1][0];
    Addr((c.g[A0][0] as u32).wrapping_add(0x704)).sw(c, count);
    ret(c);
}

/// `void RiderAnim_ctor(RiderAnim* this)` (0x144630): the base
/// `AnimController_ctor`, then the vtable word at +0x58 (0x458458) and two
/// cleared words (+0x60, +0x64).
pub fn rider_anim_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0031_18d0, 0x0014_4644)?;
        c.g[V1][0] = sx(HI.wrapping_sub(0x7ba8));
        c.g[V0][0] = c.g[S0][0];
        let this = c.g[S0][0] as u32;
        Addr(this.wrapping_add(0x60)).sw(c, 0);
        let vtable = c.g[V1][0];
        Addr(this.wrapping_add(0x58)).sw(c, vtable);
        Addr(this.wrapping_add(0x64)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void RiderTuning_StaticInitThunk(void)` (0x1448b8): `RiderTuning_StaticInit(1, 0xffff)`.
pub fn rider_tuning_static_init_thunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x1;
        c.g[A1][0] = 0xffff;
        Callee {
            addr: 0x0013_fb20,
            f: ee::s01::f_0013fb20,
        }
        .call(c, 0x0014_48cc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int BENewRaceInt_MapEventType(int kind)` (0x145870): kinds below 0x11
/// index a table of jump targets at 0x45c150 (the case value 0..=3 or 0xc
/// each); other kinds and unknown targets return 0xc or leave the function
/// through `jump_out`.
pub fn be_new_race_int_map_event_type(c: &mut Cpu) {
    if c.pc != 0x0014_5870 {
        return ee::s01::f_00145870__entries(c);
    }
    c.cycles += 3;
    c.g[V0][0] = (c.g[A1][0] < 0x11) as u64;
    let in_range = c.g[V0][0] != 0;
    c.g[V0][0] = HI as u64;
    if !in_range {
        c.cycles += 2;
        c.g[V0][0] = 0xc;
        return ret(c);
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V0][0] = sx(HI.wrapping_sub(0x5eb0));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
    let target = c.g[A0][0] as u32;
    let result = match target {
        0x0014_5894 => 0xc,
        0x0014_589c => 0x0,
        0x0014_58a4 => 0x1,
        0x0014_58ac => 0x3,
        0x0014_58b4 => 0x2,
        _ => {
            c.jump_out(target, 0x0014_5870, 0x0014_58c4);
            return;
        }
    };
    c.cycles += 2;
    c.g[V0][0] = result;
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fall_back, hook, machine};
    use crate::difftest::{check, fill, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Gate word for the callee fakes (in the fixture area above the
    /// shared test data): the fakes run only while it holds the magic.
    const DTOR_GATE: u32 = DATA + 0x20_0000;
    const DTOR_MAGIC: u32 = 0x5353_3343;
    /// The fixture's inner sub-object (the one the body writes into).
    const INNER: u32 = DATA + 0x1000;

    static OC454_PREV_RIDER_DTOR: AtomicUsize = AtomicUsize::new(0);
    static OC454_PREV_FREE: AtomicUsize = AtomicUsize::new(0);

    /// The base `Rider_dtor` (gated): records its argument, returns.
    fn oc454_fake_rider_dtor(c: &mut Cpu) {
        if c.bus.read32(DTOR_GATE) != DTOR_MAGIC {
            return fall_back(c, &OC454_PREV_RIDER_DTOR, ee::s00::f_0011b978__generated);
        }
        let this = c.g[A0][0] as u32;
        let flags = c.g[A1][0] as u32;
        c.bus.write32(DTOR_GATE + 4, this);
        c.bus.write32(DTOR_GATE + 8, flags);
        c.cycles += 5;
        c.g[V0][0] = 0;
        ret(c);
    }

    /// `__builtin_delete` (gated): records its argument, returns.
    fn oc454_fake_free(c: &mut Cpu) {
        if c.bus.read32(DTOR_GATE) != DTOR_MAGIC {
            return fall_back(c, &OC454_PREV_FREE, ee::s10::f_00317e50__generated);
        }
        let this = c.g[A0][0] as u32;
        c.bus.write32(DTOR_GATE + 12, this);
        c.cycles += 5;
        ret(c);
    }

    /// Hooks the two callee fakes into their override slots (once per process).
    fn oc454_install_fakes() {
        use std::ptr::addr_of_mut;
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_0011B978),
            oc454_fake_rider_dtor,
            &OC454_PREV_RIDER_DTOR,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc454_fake_free,
            &OC454_PREV_FREE,
        );
    }

    /// The `Rider*Sub_dtor` fixture: random junk in the constant tables, the
    /// object's sub-object pointer at `INNER`, and flags 0..=3 by seed (flags
    /// 0 copies the tables; the others run the callees).
    fn sub_dtor_setup(c: &mut Cpu, seed: u32, s: RiderSubDtorShape) {
        oc454_install_fakes();
        let mut r = machine(c, seed, 0x454_0100 + s.entry as u64, 0);
        fill(c, &mut r, HI.wrapping_sub(s.k1) - 0x20, 0x60);
        c.bus.write32(DTOR_GATE, DTOR_MAGIC);
        c.bus.write32(DATA + s.field, INNER);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = (seed % 4) as u64;
    }

    #[test]
    fn rider_uber_grind_sub_dtor_matches() {
        check(ee::s01::f_001420c8__generated, rider_uber_grind_sub_dtor, 60, &|c, seed| {
            sub_dtor_setup(c, seed, RIDER_UBER_GRIND_SUB_DTOR)
        });
    }

    #[test]
    fn rider_wipeout_sub_dtor_matches() {
        check(ee::s01::f_00142228__generated, rider_wipeout_sub_dtor, 60, &|c, seed| {
            sub_dtor_setup(c, seed, RIDER_WIPEOUT_SUB_DTOR)
        });
    }

    #[test]
    fn rider_finish_line_sub_dtor_matches() {
        check(ee::s01::f_001424d0__generated, rider_finish_line_sub_dtor, 60, &|c, seed| {
            sub_dtor_setup(c, seed, RIDER_FINISH_LINE_SUB_DTOR)
        });
    }

    #[test]
    fn rider_reset_sub_dtor_matches() {
        check(ee::s01::f_00142630__generated, rider_reset_sub_dtor, 60, &|c, seed| {
            sub_dtor_setup(c, seed, RIDER_RESET_SUB_DTOR)
        });
    }

    #[test]
    fn rider_derived_dtor_458928_matches() {
        check(ee::s01::f_00142790__generated, rider_derived_dtor_458928, 60, &|c, seed| {
            sub_dtor_setup(c, seed, RIDER_DERIVED_DTOR_458928)
        });
    }

    #[test]
    fn rider_handplant_sub_dtor_matches() {
        check(ee::s01::f_001428f0__generated, rider_handplant_sub_dtor, 60, &|c, seed| {
            sub_dtor_setup(c, seed, RIDER_HANDPLANT_SUB_DTOR)
        });
    }

    #[test]
    fn danger_man_release_danger_matches() {
        check(ee::s01::f_001441a0__generated, danger_man_release_danger, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x454_0200, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            let count = if seed % 3 == 0 { 0 } else { r.next_u32() };
            c.bus.write32(DATA + 0x704, count);
        });
    }

    #[test]
    fn rider_anim_ctor_matches() {
        check(ee::s01::f_00144630__generated, rider_anim_ctor, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x454_0300, 0);
            c.g[A0][0] = sx(DATA + 0x2000);
            fill(c, &mut r, DATA + 0x2000, 0x20);
        });
    }

    #[test]
    fn rider_tuning_static_init_thunk_matches() {
        check(
            ee::s01::f_001448b8__generated,
            rider_tuning_static_init_thunk,
            8,
            &|c, seed| {
                let _ = machine(c, seed, 0x454_0400, 0);
            },
        );
    }

    #[test]
    fn be_new_race_int_map_event_type_matches() {
        check(
            ee::s01::f_00145870__generated,
            be_new_race_int_map_event_type,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x454_0500, 0);
                c.pc = 0x0014_5870;
                c.g[A1][0] = if seed % 5 == 0 {
                    0x11 + (seed as u64 % 7)
                } else {
                    (seed % 0x11) as u64
                };
                // The jump-target table: the five cases, and a stray target.
                let cases = [
                    0x0014_5894u32,
                    0x0014_589c,
                    0x0014_58a4,
                    0x0014_58ac,
                    0x0014_58b4,
                    0x0014_5900,
                ];
                for i in 0..0x11u32 {
                    let t = cases[((seed + i) % 6) as usize];
                    c.bus.write32(0x0045_c150 + 4 * i, t);
                }
            },
        );
    }
}
