//! Staging wave 201 (oc212): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size),
//! one function per writer. Each reproduces the generated body's registers,
//! cycles, memory and call order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_b3f8, rider_update_target_speed, "Rider_UpdateTargetSpeed"),
];

// ---------------------------------------------------------------------------
// Rider_UpdateTargetSpeed (0x11b3f8)

/// Fields, globals and literals of `Rider_UpdateTargetSpeed`.
mod target_speed {
    /// `BELibrary_GetInstance()` (through the function table).
    pub const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;
    /// `BELibrary_GetInterface(lib, 3)`.
    pub const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
    /// `float BECharInt_GetAttrib0Factor(iface, slot, override)`: the
    /// rider's speed attribute as a factor.
    pub const BE_CHAR_INT_GET_ATTRIB0_FACTOR: u32 = 0x0014_94c0;
    /// `Rider_GetMainState(rider)`.
    pub const RIDER_GET_MAIN_STATE: u32 = 0x0011_fe98;

    /// Rider: back-end player slot.
    pub const RIDER_SLOT: u32 = 0x86c;
    /// Rider: attribute override passed to the factor query.
    pub const ATTRIB_OVERRIDE: u32 = 0xb34;
    /// Rider: index of the course segment it is on.
    pub const SEGMENT_INDEX: u32 = 0x438;
    /// Rider: normalised speed (picks the speed band).
    pub const SPEED: u32 = 0x2fc;
    /// Rider: column blend of the speed table (used when the main state is 0).
    pub const COLUMN_BLEND: u32 = 0x220;
    /// Rider: an int state; from 11 up the target has a floor.
    pub const STATE: u32 = 0x2f4;
    /// Rider: two words compared for equality (no slow-down when equal).
    pub const LINE_A: u32 = 0x320;
    pub const LINE_B: u32 = 0x324;
    /// Rider: word cleared on every normal update.
    pub const CLEARED: u32 = 0x5a0;
    /// Rider: the smoothed target speed (written).
    pub const TARGET: u32 = 0x2e4;
    /// Rider: the target's floor when the state is 11 or more.
    pub const TARGET_FLOOR: u32 = 0x30c;

    /// The world (`gp-0x848`), its course (+0x84), the course's segment
    /// array (+0x44) of 0xb0-byte segments; a segment's +4 is the float
    /// that picks and blends the speed-table rows.
    pub const WORLD_GP: u32 = 0x848;
    pub const WORLD_COURSE: u32 = 0x84;
    pub const COURSE_SEGMENTS: u32 = 0x44;
    pub const SEGMENT_SIZE: u32 = 0xb0;
    pub const SEGMENT_SLOPE: u32 = 0x4;

    /// The speed table: 2 pages (0x60 apart) of rows (0x20) of 4 bands of
    /// 2 floats (columns).
    pub const SPEED_TABLE: u32 = 0x004a_6310;
    pub const PAGE: u32 = 0x60;

    /// `.lit4` (gp-relative): band limits 0.1, 0.4, 0.8; the fixed target
    /// 3333.33 (main state 1); the table-to-speed scale 27.78; smoothing
    /// 0.9 (rising) / 0.97 (falling); the stopped slow-down 0.98.
    pub const LIT_BAND_1: u32 = 0x79dc;
    pub const LIT_BAND_2: u32 = 0x79d8;
    pub const LIT_BAND_3: u32 = 0x79d4;
    pub const LIT_FIXED_TARGET: u32 = 0x79d0;
    pub const LIT_SCALE: u32 = 0x79cc;
    pub const LIT_KEEP_RISING: u32 = 0x79c8;
    pub const LIT_STOPPED: u32 = 0x79c4;
    pub const LIT_KEEP_FALLING: u32 = 0x79c0;
}

/// `void Rider_UpdateTargetSpeed(Rider* r)` (0x11b3f8): the rider's target
/// speed. A bilinear lookup in the speed table (rows by the slope of the
/// segment the rider is on, columns by `COLUMN_BLEND`, band by the
/// normalised speed) gives one value per table page; the two pages are
/// blended by the back end's speed attribute `a` (`factor + 1 - 1`), scaled
/// by 27.78 (x0.98 when not moving and `LINE_A != LINE_B`), and the stored
/// target eases toward that (keeping 0.9 of the old value when rising, 0.97
/// when falling). Main state 1 sets the target to 3333.33 instead. From
/// `STATE` 11 up, a moving rider's target is at least `TARGET_FLOOR`.
pub fn rider_update_target_speed(c: &mut Cpu) {
    use self::target_speed::*;
    use crate::abi;
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;
    const ONE_BITS: u64 = 0x3f80_0000;
    let one = FpuF32::ONE;
    let zero = FpuF32::ZERO;
    guest(c, |c| {
        let lit = |c: &mut Cpu, off: u32| Addr(r32(c, GP).wrapping_sub(off)).read_fpu(c);

        c.cycles += 31;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_fpr(c, 21, 0x48);
        frame.save_fpr(c, 20, 0x40);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let rider = Addr(r32(c, S0));

        // The speed attribute from the back end.
        call_ptr(c, BE_LIBRARY_GET_INSTANCE, 0x0011_b418)?;
        c.g[AT][0] = ONE_BITS;
        c.f[20] = one.0;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        call_ptr(c, BE_LIBRARY_GET_INTERFACE, 0x0011_b42c)?;
        c.g[A2][0] = rider.off(ATTRIB_OVERRIDE).lw(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = rider.off(RIDER_SLOT).lw(c);
        call_ptr(c, BE_CHAR_INT_GET_ATTRIB0_FACTOR, 0x0011_b43c)?;

        // The segment the rider is on.
        let world = Addr(r32(c, GP).wrapping_sub(WORLD_GP)).lw(c);
        c.g[V0][0] = world;
        c.g[A2][0] = SEGMENT_SIZE as u64;
        let index = rider.off(SEGMENT_INDEX).lw(c) as u32;
        let factor = FpuF32(c.f[0]) + one;
        c.f[0] = factor.0;
        let course = Addr(world as u32).off(WORLD_COURSE).lw(c);
        c.g[A1][0] = course;
        c.g[A0][0] = c.g[S0][0];
        let seg_off = abi::mult(c, index, SEGMENT_SIZE);
        c.g[V1][0] = sx(seg_off);
        let segments = Addr(course as u32).off(COURSE_SEGMENTS).lw(c);
        c.g[V0][0] = segments;
        let attrib = factor - one;
        c.f[21] = attrib.0;
        let segment = Addr((segments as u32).wrapping_add(seg_off));
        c.g[S1][0] = sx(segment.0);
        Callee {
            addr: RIDER_GET_MAIN_STATE,
            f: ee::s00::Rider_GetMainState,
        }
        .call(c, 0x0011_b468)?;
        let state = c.g[V0][0];
        c.g[A2][0] = state;

        // Row position: slope > 0 picks rows (1, 2) at weight slope, else
        // rows (0, 1) at weight slope + 1. Main states other than 0 use 1.
        let mut slope = one;
        if state == 0 {
            c.cycles += 1;
            slope = segment.off(SEGMENT_SLOPE).read_fpu(c);
        }
        c.f[1] = slope.0;
        c.cycles += 5;
        c.f[0] = zero.0;
        let upper = zero.lt(slope);
        abi::set_fcc(c, upper);
        if !upper {
            c.cycles += 1;
        }
        c.cycles += 3;
        let row = upper as u64;
        c.g[V0][0] = row;
        c.g[V1][0] = row;
        let mut t = slope;
        if !upper {
            c.cycles += 1;
            t = t + one;
        }
        c.f[3] = t.0;

        // Speed band: < 0.1, < 0.4, < 0.8, else 3.
        c.cycles += 6;
        let speed = rider.off(SPEED).read_fpu(c);
        c.f[1] = speed.0;
        let limit = lit(c, LIT_BAND_1);
        c.f[0] = limit.0;
        let mut band = 0u32;
        let mut cond = limit.le(speed);
        if cond {
            c.cycles += 5;
            let limit = lit(c, LIT_BAND_2);
            c.f[0] = limit.0;
            cond = limit.le(speed);
            band = 1;
            if cond {
                c.cycles += 5;
                let limit = lit(c, LIT_BAND_3);
                c.f[0] = limit.0;
                cond = speed.lt(limit);
                band = if cond {
                    c.cycles += 1;
                    2
                } else {
                    3
                };
            }
        }
        abi::set_fcc(c, cond);
        c.g[A0][0] = band as u64;

        // Column blend.
        c.cycles += 4;
        c.g[AT][0] = ONE_BITS;
        let s = if state == 0 {
            rider.off(COLUMN_BLEND).read_fpu(c)
        } else {
            one
        };
        c.f[2] = s.0;

        // Bilinear weights and the two row pointers.
        c.cycles += 21;
        let one_minus_s = one - s;
        let one_minus_t = one - t;
        c.f[0] = one_minus_t.0;
        c.f[1] = one_minus_s.0;
        c.f[7] = (t * s).0;
        c.f[6] = (t * one_minus_s).0;
        c.f[5] = (one_minus_t * s).0;
        c.f[4] = (one_minus_t * one_minus_s).0;
        let col = band << 3;
        c.g[V1][0] = sx(col.wrapping_add((row as u32) << 5).wrapping_add(SPEED_TABLE));
        c.g[A0][0] = sx(col.wrapping_add(((row as u32) + 1) << 5).wrapping_add(SPEED_TABLE));
        c.g[V0][0] = sx(sp(c));
        c.g[A1][0] = 1;

        // One bilinear value per table page into sp[0], sp[4].
        loop {
            c.cycles += 17;
            let lo_row = Addr(r32(c, V1));
            let hi_row = Addr(r32(c, A0));
            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
            let p0 = FpuF32(c.f[4]) * lo_row.read_fpu(c);
            let p3 = FpuF32(c.f[6]) * hi_row.read_fpu(c);
            let p2 = FpuF32(c.f[5]) * lo_row.off(4).read_fpu(c);
            let p1 = FpuF32(c.f[7]) * hi_row.off(4).read_fpu(c);
            c.g[A0][0] = sx(hi_row.0.wrapping_add(PAGE));
            c.g[V1][0] = sx(lo_row.0.wrapping_add(PAGE));
            let value = p0 + p3 + p2 + p1;
            c.f[0] = value.0;
            c.f[1] = p1.0;
            c.f[2] = p2.0;
            c.f[3] = p3.0;
            Addr(r32(c, V0)).write_fpu(c, value);
            let more = (c.g[A1][0] as i64) >= 0;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0011_b548);
            }
        }

        c.cycles += 3;
        c.g[V0][0] = 1;
        let page0 = Addr(sp(c)).read_fpu(c);
        c.f[2] = page0.0;
        let target = if state == 1 {
            c.cycles += 3;
            let fixed = lit(c, LIT_FIXED_TARGET);
            c.f[2] = fixed.0;
            fixed
        } else {
            // Pages blended by the attribute, scaled to a speed.
            c.cycles += 14;
            c.g[AT][0] = ONE_BITS;
            let page1 = Addr(sp(c).wrapping_add(4)).read_fpu(c);
            let scale = lit(c, LIT_SCALE);
            c.f[3] = scale.0;
            let b = attrib * page1;
            c.g[V1][0] = rider.off(LINE_A).lw(c);
            let a = (one - attrib) * page0;
            c.f[0] = a.0;
            c.g[V0][0] = rider.off(LINE_B).lw(c);
            rider.off(CLEARED).sw(c, 0);
            let mut keep = lit(c, LIT_KEEP_RISING);
            let blended = b + a;
            c.f[1] = blended.0;
            let mut wanted = blended * scale;
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 6;
                let speed = rider.off(SPEED).read_fpu(c);
                c.f[1] = speed.0;
                c.f[0] = zero.0;
                let stopped = speed.le(zero);
                abi::set_fcc(c, stopped);
                if stopped {
                    c.cycles += 3;
                    let k = lit(c, LIT_STOPPED);
                    c.f[0] = k.0;
                    wanted = wanted * k;
                }
            } else {
                c.cycles += 1;
            }
            let current = rider.off(TARGET).read_fpu(c);

            // Ease toward it.
            c.cycles += 3;
            let falling = wanted.lt(current);
            abi::set_fcc(c, falling);
            if falling {
                c.cycles += 1;
                keep = lit(c, LIT_KEEP_FALLING);
            }
            c.f[4] = keep.0;
            c.cycles += 7;
            c.g[AT][0] = ONE_BITS;
            let kept = keep * current;
            c.f[1] = kept.0;
            let moved = (one - keep) * wanted;
            c.f[0] = moved.0;
            let next = moved + kept;
            c.f[2] = next.0;
            next
        };
        c.g[V0][0] = rider.off(STATE).lw(c);

        c.cycles += 3;
        let below = ((c.g[V0][0] as i64) < 11) as u64;
        c.g[V0][0] = below;
        rider.off(TARGET).write_fpu(c, target);
        if below == 0 {
            c.cycles += 6;
            let speed = rider.off(SPEED).read_fpu(c);
            c.f[1] = speed.0;
            c.f[0] = zero.0;
            let moving = zero.lt(speed);
            abi::set_fcc(c, moving);
            if moving {
                c.cycles += 4;
                let floored = rider.off(TARGET_FLOOR).read_fpu(c).max(target);
                c.f[0] = floored.0;
                rider.off(TARGET).write_fpu(c, floored);
            }
        } else {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 6;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------ Rider_UpdateTargetSpeed

    mod target_speed_fixture {
        use super::super::target_speed::*;
        use crate::difftest::events::{schedule, Events};
        use crate::difftest::fakes::{machine, vtable};
        use crate::difftest::{fill, put_words, Rng, DATA, GAME_GP};
        use ssx3_ee_rt::cpu::{reg::*, sx};
        use ssx3_ee_rt::Cpu;

        pub const RIDER: u32 = DATA + 0x20_0000;
        const WORLD: u32 = DATA + 0x21_0000;
        const COURSE: u32 = DATA + 0x21_0100;
        const OWNER: u32 = DATA + 0x21_0200;
        const SEGMENTS: u32 = DATA + 0x22_0000;
        const BE_LIBRARY_GP: u32 = 0x1f04;
        const BE_LIBRARY_INDEX_GP: u32 = 0x1efc;
        const BE_LIBRARIES: u32 = 0x0053_08a8;
        /// The `.lit4` values at gp-0x79dc .. gp-0x79c0.
        const LITS: [u32; 8] = [
            0x3dcc_cccd, 0x3ecc_cccd, 0x3f4c_cccd, 0x4550_5556,
            0x41de_38e4, 0x3f66_6666, 0x3f7a_e148, 0x3f78_51ec,
        ];

        fn pick(r: &mut Rng, xs: &[f32]) -> u32 {
            xs[(r.next_u32() as usize) % xs.len()].to_bits()
        }

        /// A rider on a course segment, the back end (library singleton,
        /// an interface whose query method is a fake, player and character
        /// tables for the attribute factor), the speed table and literals.
        /// `calls`: fake calls that may unwind (0 for the event test).
        pub fn setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
            let mut r = machine(c, seed, 0x11_b3f8, calls);

            // Back end: the library exists; interface 0's vtable slot 8 is
            // a fake (it returns the interface).
            c.bus.write32(GAME_GP - BE_LIBRARY_GP, DATA + 0x30_0000);
            c.bus.write32(GAME_GP - BE_LIBRARY_INDEX_GP, 0);
            fill(c, &mut r, 0x0053_0000, 0x1100 / 4);
            fill(c, &mut r, 0x0053_4f00, 0x1100 / 4);
            let lib = DATA + 0x31_0000;
            c.bus.write32(BE_LIBRARIES, lib);
            let vt = vtable(c, &mut r, &[8]);
            c.bus.write32(lib + 4, vt);
            // Player slots -> rider records (some without a profile).
            for p in 0..8 {
                c.bus.write32(0x0053_05b0 + 4 * p, r.next_u32() % 8);
            }
            for s in 0..8 {
                if r.next_u32() % 3 == 0 {
                    c.bus.write32(0x0053_5b20 + s * 0x1c + 0xc, u32::MAX);
                }
            }

            // The rider.
            fill(c, &mut r, RIDER, 0xc00 / 4);
            c.g[A0][0] = sx(RIDER);
            c.bus.write32(RIDER + RIDER_SLOT, r.next_u32() % 8);
            let ov = if r.next_u32() % 2 == 0 { r.next_u32() % 4 } else { 0u32.wrapping_sub(r.next_u32() % 3) };
            c.bus.write32(RIDER + ATTRIB_OVERRIDE, ov);
            let index = r.next_u32() % 16;
            c.bus.write32(RIDER + SEGMENT_INDEX, index);
            c.bus.write32(RIDER + 0x77c, OWNER);
            // Main state: 0 (table rows by slope), 1 (fixed target), other.
            let state = match seed % 4 {
                0 | 1 => 0,
                2 => 1,
                _ => 2 + r.next_u32() % 20,
            };
            c.bus.write32(OWNER + 0xde0, state);
            let speed = pick(&mut r, &[-0.5, 0.0, 0.05, 0.1, 0.2, 0.4, 0.6, 0.8, 1.2]);
            c.bus.write32(RIDER + SPEED, speed);
            c.bus.write32(RIDER + COLUMN_BLEND, r.f32(1.0) & 0x7fff_ffff);
            c.bus.write32(RIDER + STATE, r.next_u32() % 22);
            let line = r.next_u32();
            c.bus.write32(RIDER + LINE_A, line);
            c.bus.write32(RIDER + LINE_B, if r.next_u32() % 2 == 0 { line } else { line ^ 1 });
            c.bus.write32(RIDER + TARGET, r.f32(40.0) & 0x7fff_ffff);
            c.bus.write32(RIDER + TARGET_FLOOR, r.f32(40.0) & 0x7fff_ffff);

            // The world, course and segment slope.
            c.bus.write32(GAME_GP - WORLD_GP, WORLD);
            c.bus.write32(WORLD + WORLD_COURSE, COURSE);
            c.bus.write32(COURSE + COURSE_SEGMENTS, SEGMENTS);
            let slope = if r.next_u32() % 5 == 0 { r.edge_f32() } else { r.f32(1.0) };
            c.bus.write32(SEGMENTS + index * SEGMENT_SIZE + SEGMENT_SLOPE, slope);

            // Speed table and literals.
            for i in 0..0x100 / 4 {
                let v = r.f32(1.5);
                c.bus.write32(SPEED_TABLE + 4 * i, v);
            }
            put_words(c, GAME_GP - LIT_BAND_1, &LITS);
            c.pc = 0x0011_b3f8;
            r
        }

        /// [`setup`] entered on a running thread with an event due at the
        /// loop's back edge (off / unwind request / delivered).
        pub fn setup_events(c: &mut Cpu, seed: u32) -> Events {
            let mut r = setup(c, seed, 0);
            schedule(c, &mut r, seed, 0x0011_b3f8)
        }
    }

    /// `Rider_UpdateTargetSpeed`: every path (main state 0/1/other, slope
    /// sign, the four speed bands, lines equal or not, stopped, rising or
    /// falling, the state-11 floor), the back-end calls unwinding, LO/HI of
    /// the segment `mult`, and a real event at the table loop's back edge.
    #[test]
    fn rider_update_target_speed_matches() {
        use crate::difftest::check_lohi;
        use crate::difftest::events::check_events;
        use crate::difftest::fakes::covered;
        covered(ee::s00::f_0011b3f8__generated, super::rider_update_target_speed, 300, &|c, seed| {
            target_speed_fixture::setup(c, seed, 1);
        });
        check_lohi(ee::s00::f_0011b3f8__generated, super::rider_update_target_speed, 100, &|c, seed| {
            target_speed_fixture::setup(c, seed, 1);
        });
        check_events(
            ee::s00::f_0011b3f8__generated,
            super::rider_update_target_speed,
            90,
            &target_speed_fixture::setup_events,
        );
    }
}
