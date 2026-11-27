//! Staging module oc1129 (wave wave271): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1129`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x003d_5ee8,
    path_track_update_fade3,
    "PATH_TrackUpdateFade3",
)];

/// `void PATH_TrackUpdateFade3(Track* t)` (0x3d5ee8): third fade of a music
/// track (`+0x1c` current level byte, `+0x1d` spare, `+0x1e` target level
/// byte, `+0x18` start time). The curve (linear, square, inverse) and the
/// duration come from the track table; interpolates between the levels and
/// calls `PATH_PlatSetTrackLevel(handle, level)`; at the end sets the final
/// level and clears the fade.
pub fn path_track_update_fade3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x450000;
        frame.save(c, S0, 0x10);
        c.g[A3][0] &= 0xffff;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw(0x450000u32.wrapping_sub(0xbe0));
        c.g[A2][0] = 0xffff_ffff_ffff_ff80;
        let s0 = c.g[S0][0] as u32;
        c.g[V0][0] = c.lb(s0.wrapping_add(0x1e));
        c.g[T0][0] = 1;
        c.g[V1][0] = c.lb(s0.wrapping_add(0x1c));
        c.mtc1(V0, 2);
        c.cvt_s_w(2, 2);
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(AT, 3);
        c.mtc1(V1, 0);
        c.cvt_s_w(0, 0);
        let a0 = c.g[A0][0] as u32;
        c.g[V0][0] = c.lw(a0.wrapping_add(0x28));
        c.g[V1][0] = c.lhu(a0.wrapping_add(0x24));
        c.mtc1(0, 1);
        c.g[V0][0] &= 0x7f;
        c.sub_s(2, 2, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
        c.g[A3][0] |= c.g[V1][0];
        c.g[A3][0] &= c.g[A2][0];
        c.c_lt_s(2, 1);
        let ge = !c.fcc();
        c.g[A3][0] |= c.g[V0][0];
        if !ge {
            c.cycles += 1;
            c.g[T0][0] = 0;
        }
        // elapsed = now - start (unsigned) in f1
        c.cycles += 9;
        c.g[V0][0] = 0x450000;
        c.g[A0][0] = 0xffff_ffff_ffff_ff7f;
        c.g[A1][0] = c.lw(0x450000u32.wrapping_sub(0xbc8));
        c.g[V1][0] = sx((c.g[T0][0] as u32) << 7);
        c.g[V0][0] = c.lw(s0.wrapping_add(0x18));
        c.g[A3][0] &= c.g[A0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[A3][0] |= c.g[V1][0];
        if !neg {
            c.cycles += 4;
            c.mtc1(V0, 1);
            c.cvt_s_w(1, 1);
            c.g[V0][0] = sx((c.g[A3][0] as u32) >> 16);
        } else {
            c.cycles += 7;
            c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
            c.g[V0][0] &= 1;
            c.g[V0][0] |= c.g[V1][0];
            c.mtc1(V0, 1);
            c.cvt_s_w(1, 1);
            c.add_s(1, 1, 1);
            c.g[V0][0] = sx((c.g[A3][0] as u32) >> 16);
        }
        // duration in f0
        c.cycles += 2;
        if (c.g[V0][0] as i64) < 0 {
            c.g[V0][0] &= 1;
            c.cycles += 5;
            c.g[V1][0] = sx((c.g[A3][0] as u32) >> 17);
            c.g[V0][0] |= c.g[V1][0];
            c.mtc1(V0, 0);
            c.cvt_s_w(0, 0);
            c.add_s(0, 0, 0);
        } else {
            c.cycles += 4;
            c.mtc1(V0, 0);
            c.cvt_s_w(0, 0);
        }
        c.cycles += 7;
        c.div_s(0, 1, 0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        c.c_le_s(1, 0);
        let running = !c.fcc();
        c.g[V0][0] = c.g[A3][0] & 0x80;
        let s0 = c.g[S0][0] as u32;
        if !running {
            // finished: set the target level, clear the fade
            c.cycles += 10;
            c.g[A0][0] = c.lw(s0.wrapping_add(0x54));
            c.g[A1][0] = c.lb(s0.wrapping_add(0x1e));
            call_ptr(c, 0x002a_f580, 0x003d_6004)?;
            c.g[V1][0] = u64::MAX;
            c.g[V0][0] = u64::MAX;
            let s0 = c.g[S0][0] as u32;
            let (v0, v1) = (c.g[V0][0], c.g[V1][0]);
            c.sb(s0.wrapping_add(0x1c), v0);
            c.sb(s0.wrapping_add(0x1d), v1);
            c.sw(s0.wrapping_add(0x18), 0);
            c.sb(s0.wrapping_add(0x1e), v1);
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
            return fade3_exit(c, frame);
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.sub_s(0, 1, 0);
        }
        c.cycles += 4;
        c.g[V1][0] = c.g[A3][0] & 0x7f;
        c.g[V0][0] = 2;
        let is2 = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = (c.g[V1][0] < 3) as u64;
        if is2 {
            c.cycles += 3;
            c.mul_s(0, 0, 0);
            c.sub_s(3, 1, 0);
            c.cycles += 1;
            c.g[V0][0] = c.lb(s0.wrapping_add(0x1c));
        } else {
            c.cycles += 2;
            let ge3 = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
            if !ge3 {
                c.cycles += 2;
                if c.g[V1][0] == c.g[V0][0] {
                    c.sub_s(3, 1, 0);
                    c.cycles += 1;
                    c.g[V0][0] = c.lb(s0.wrapping_add(0x1c));
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.lb(s0.wrapping_add(0x1c));
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = 3;
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 4;
                    c.div_s(1, 1, 0);
                    c.g[AT][0] = 0x41c8_0000;
                    c.mtc1(AT, 0);
                    c.div_s(3, 1, 0);
                    c.cycles += 1;
                    c.g[V0][0] = c.lb(s0.wrapping_add(0x1c));
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.lb(s0.wrapping_add(0x1c));
                }
            }
        }
        // f1 = min(cur, tgt), f2 = |tgt - cur|
        c.cycles += 6;
        c.g[V1][0] = c.lb(s0.wrapping_add(0x1e));
        c.mtc1(V0, 1);
        c.cvt_s_w(1, 1);
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.mtc1(V1, 1);
            c.cvt_s_w(1, 1);
        }
        c.cycles += 5;
        c.mtc1(0, 0);
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.neg_s(2, 2);
        }
        c.cycles += 4;
        c.c_le_s(0, 3);
        if !c.fcc() {
            frame.restore(c, S0, 0x10);
            return fade3_exit(c, frame);
        }
        c.cycles += 6;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.c_le_s(3, 0);
        if !c.fcc() {
            frame.restore(c, S0, 0x10);
            return fade3_exit(c, frame);
        }
        c.cycles += 6;
        c.mul_s(0, 2, 3);
        c.add_s(3, 1, 0);
        c.cvt_w_s(0, 3);
        c.mfc1(A1, 0);
        c.g[A0][0] = c.lw(s0.wrapping_add(0x54));
        call_ptr(c, 0x002a_f580, 0x003d_60f8)?;
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        fade3_exit(c, frame)
    })
}

/// Epilogue of `PATH_TrackUpdateFade3`: restore `ra`, pop the frame, return.
fn fade3_exit(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{fakes, DATA};

    /// `PATH_TrackUpdateFade3`: finished and running fades, the four curve
    /// kinds, rising and falling levels, start time in the future (elapsed
    /// wraps), duration 0, out-of-range fractions, unwinds in the setter.
    #[test]
    fn path_track_update_fade3_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
                let mut r = fakes::machine(c, seed, 0x1129, 1);
                let cfg = DATA + 0x1000;
                let track = DATA + 0x2000;
                const LEVELS: [i8; 7] = [-1, 0, 5, 50, 127, -128, 20];
                const DURS: [u32; 6] = [0, 1, 100, 0x8000, 0xffff, 60];
                const AGES: [u32; 8] = [0, 10, 50, 59, 60, 61, 200, 0xffff_fff0];
                let curve = [0u32, 1, 2, 3, 5, 0x81, 2, 1][(seed % 8) as usize];
                let dur = DURS[(r.next_u32() % 6) as usize];
                let now = 1000 + r.next_u32() % 1000;
                let age = AGES[(r.next_u32() % 8) as usize];
                c.bus.write32(0x0044_f420, cfg);
                c.bus.write32(0x0044_f438, now);
                c.bus.write32(cfg + 0x28, curve | (r.next_u32() & 0xffff_ff00));
                c.bus.write16(cfg + 0x24, dur as u16);
                c.bus.write32(track + 0x18, now.wrapping_sub(age));
                c.bus.write8(track + 0x1c, LEVELS[(r.next_u32() % 7) as usize] as u8);
                c.bus.write8(track + 0x1e, LEVELS[(r.next_u32() % 7) as usize] as u8);
                c.bus.write32(track + 0x54, 0);
                c.g[A0][0] = sx(track);
                c.pc = 0x003d_5ee8;
        };
        // The setter (0x2af580) is a real function in the table, not a fake
        // miss: compare with `check` and require many distinct paths.
        crate::difftest::check(
            ee::s14::f_003d5ee8__generated,
            path_track_update_fade3,
            400,
            &setup,
        );
        let mut totals = std::collections::BTreeSet::new();
        for seed in 0..400 {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, ee::s14::f_003d5ee8__generated);
            totals.insert(c.cycles);
        }
        assert!(totals.len() >= 8, "only {} distinct cycle totals", totals.len());
    }
}
