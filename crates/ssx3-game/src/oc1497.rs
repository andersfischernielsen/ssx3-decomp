//! Staging module oc1497 (wave wave275): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1497`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_7248, rider_metrix_ctor, "RiderMetrix_ctor"),
    (0x002f_3550, c_splash_manager_ctor, "cSplashManager_ctor"),
    (
        0x0031_d738,
        c_csi_cubic_spline_init_with_start_slope,
        "cCSICubicSpline_InitWithStartSlope",
    ),
    (0x002b_b570, wp_file_set_data, "WPFile_SetData"),
];

/// `cRiderMetrix* RiderMetrix_ctor(cRiderMetrix* this)` (0x117248): clears
/// the HUD-effect, points and flag words, sets the points multiplier to 1.0
/// (`+0x1c4`), then resets the metrix. Returns `this`.
pub fn rider_metrix_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = Addr(c.g[S0][0] as u32);
        this.off(0x1ac).sw(c, 0);
        this.off(0x1b0).sw(c, 0);
        this.off(0x1b4).sw(c, 0);
        this.off(0x1c8).sw(c, 0);
        c.swc1(0, this.off(0x1c4).0);
        Callee {
            addr: 0x0011_7540,
            f: ee::s00::f_00117540,
        }
        .call(c, 0x0011_7278)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `cSplashManager* cSplashManager_ctor(cSplashManager* this, int bucket,
/// int viewIndex)` (0x2f3550): constructs the object node (`bucket` is in
/// `a1`, passed on unchanged), sets the view index, clears the 30 drop
/// entries (`+0x1c`, 0x3c apart) and the 30 crystal entries (from `+0x724`,
/// 0x4c apart), zeroes the camera-tracking and counter words, then resets
/// the manager. Returns `this`.
pub fn c_splash_manager_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1c));
        call_ptr(c, 0x0035_4648, 0x002f_3578)?;
        c.g[S3][0] = u64::MAX;
        c.g[V0][0] = 0x49_0000;
        let view = c.g[S0][0];
        Addr(c.g[S1][0] as u32).off(0x10).sw(c, view);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7dd0));
        Addr(c.g[S1][0] as u32).off(0x14).sw(c, 0);
        let v0 = c.g[V0][0];
        Addr(c.g[S1][0] as u32).off(0xc).sw(c, v0);
        c.g[S0][0] = 0x1d;
        Addr(c.g[S1][0] as u32).off(0x18).sw(c, 0);
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
            Callee {
                addr: 0x002f_2588,
                f: ee::s09::f_002f2588,
            }
            .call(c, 0x002f_35a4)?;
            let t = c.g[S0][0] != c.g[S3][0];
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3c));
            if !t {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002f_3598);
            }
        }
        c.cycles += 3;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x724));
        c.g[S2][0] = 0x1d;
        c.g[S3][0] = u64::MAX;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            Callee {
                addr: 0x002f_2d60,
                f: ee::s09::f_002f2d60,
            }
            .call(c, 0x002f_35c4)?;
            let t = c.g[S2][0] != c.g[S3][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4c));
            if !t {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002f_35b8);
            }
        }
        c.cycles += 19;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = c.g[S1][0];
        let this = Addr(c.g[S1][0] as u32);
        this.off(0x100c).sw(c, 0);
        this.off(0x1010).sw(c, 0);
        this.off(0x1014).sw(c, 0);
        this.off(0x1018).sw(c, 0);
        this.off(0x1020).sw(c, 0);
        this.off(0x1024).sw(c, 0);
        c.swc1(0, this.off(0x101c).0);
        Callee {
            addr: 0x002f_39c8,
            f: ee::s09::f_002f39c8,
        }
        .call(c, 0x002f_35f8)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cCSICubicSpline_InitWithStartSlope(float f12, float f13, spline* s)`
/// (0x31d738, `a0` is the spline): allocates the spline data
/// (`cCSICubicSpline_Alloc`), sets bit 0 of the spline's first word, stores
/// `f12` at data `+0x10` and `f13` at spline `+4`.
pub fn c_csi_cubic_spline_init_with_start_slope(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x28);
        frame.save_fpr(c, 20, 0x20);
        c.mov_s(21, 13);
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        Callee {
            addr: 0x0031_d660,
            f: ee::s10::f_0031d660,
        }
        .call(c, 0x0031_d75c)?;
        let spline = Addr(c.g[S0][0] as u32);
        let flags = spline.lw(c) | 1;
        let data = spline.off(0x8).lw(c);
        frame.restore_ra(c, 0);
        spline.sw(c, flags);
        c.swc1(20, (data as u32).wrapping_add(0x10));
        c.swc1(21, spline.off(0x4).0);
        frame.restore(c, S0, 0x10);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// `void WPFile_SetData(WPFile* f, int, void* data)` (0x2bb570): writes the
/// file's type word (2 at `+0`, 1 at `+2`), stores `a2` as its data at `+8`
/// and relocates the file (`WPFile_Relocate`).
pub fn wp_file_set_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[V0][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x2;
        c.g[V1][0] = 0x1;
        let file = Addr(c.g[V0][0] as u32);
        let kind = c.g[A0][0];
        file.sh(c, kind);
        let one = c.g[V1][0];
        file.off(0x2).sh(c, one);
        c.g[A0][0] = c.g[V0][0];
        let data = c.g[A2][0];
        file.off(0x8).sw(c, data);
        Callee {
            addr: 0x002b_b5f0,
            f: ee::s08::f_002bb5f0,
        }
        .call(c, 0x002b_b598)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, machine_rt};
    use crate::difftest::{check, fakes, fill, DATA};

    /// `RiderMetrix_ctor` against the generated body: random object words,
    /// the real reset runs underneath (its memset is a fake).
    #[test]
    fn rider_metrix_ctor_matches() {
        check(
            ee::s00::f_00117248__generated,
            rider_metrix_ctor,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x0011_7248, 6);
                let obj = DATA + 0x1000;
                fill(c, &mut r, obj, 0x200);
                c.g[A0][0] = obj as u64;
            },
        );
    }

    /// `cSplashManager_ctor` against the generated body: both 30-entry
    /// loops run to completion; the object node, drop and crystal
    /// constructors and the reset are real code over random object words.
    #[test]
    fn c_splash_manager_ctor_matches() {
        check(
            ee::s09::f_002f3550__generated,
            c_splash_manager_ctor,
            100,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x002f_3550, 6);
                let obj = DATA + 0x1000;
                fill(c, &mut r, obj, 0x1040 / 4);
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    /// Gate word of the `cCSICubicSpline_Alloc` fake (own RAM word).
    const OC1497_ALLOC_GATE: u32 = DATA + 0x3f00;
    const OC1497_ALLOC_MAGIC: u32 = 0x1497_a110;
    /// The slot body the fake replaced (for its gate-off path).
    static OC1497_ALLOC_PREV: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    /// `cCSICubicSpline_Alloc` with its allocator replaced: when the gate is
    /// set, it returns a fixed data block (`DATA+0x3300`) and returns, so the
    /// allocator walk (which spins on test data) never runs. Gate off: the
    /// replaced body.
    fn oc1497_spline_alloc_fake(c: &mut Cpu) {
        if c.bus.read32(OC1497_ALLOC_GATE) != OC1497_ALLOC_MAGIC {
            return fakes::fall_back(c, &OC1497_ALLOC_PREV, ee::s10::f_0031d660__generated);
        }
        c.g[V0][0] = (DATA + 0x3300) as u64;
        ret(c);
    }

    /// `cCSICubicSpline_InitWithStartSlope` against the generated body: the
    /// allocation (gated fake, same on both sides) and the two float stores,
    /// with random float inputs.
    #[test]
    fn c_csi_cubic_spline_init_with_start_slope_matches() {
        check(
            ee::s10::f_0031d738__generated,
            c_csi_cubic_spline_init_with_start_slope,
            200,
            &|c, seed| {
                fakes::hook(
                    std::ptr::addr_of_mut!(ee::s10::OVERRIDE_0031D660),
                    oc1497_spline_alloc_fake,
                    &OC1497_ALLOC_PREV,
                );
                let mut r = machine_rt(c, seed, 0x0031_d738, 4);
                c.bus.write32(OC1497_ALLOC_GATE, OC1497_ALLOC_MAGIC);
                let spline = DATA + 0x3200;
                fill(c, &mut r, spline, 16);
                c.bus.write32(spline + 8, DATA + 0x3300);
                fill(c, &mut r, DATA + 0x3300, 8);
                c.g[A0][0] = spline as u64;
                c.f[12] = r.f32(1000.0);
                c.f[13] = r.edge_f32();
            },
        );
    }

    /// `WPFile_SetData` against the generated body. `a2 = 0` makes the
    /// relocation take its null short path (no pointer walk), so the
    /// callee stays bounded on test data.
    #[test]
    fn wp_file_set_data_matches() {
        check(
            ee::s08::f_002bb570__generated,
            wp_file_set_data,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002b_b570, 2);
                let file = DATA + 0x3100;
                fill(c, &mut r, file, 8);
                c.g[A0][0] = file as u64;
                c.g[A2][0] = 0;
            },
        );
    }
}
