//! `src/render/ps2bezierman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::guest::Addr;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0038_aeb0, ps2_bezier_man_dtor, "PS2BezierMan_dtor"),
    (0x0038_ae60, ps2_bezier_man_ctor, "PS2BezierMan_ctor"),
];

// ---------------------------------------------------------------------------
// Callees (the generated aliases, so a replaced callee is reached too).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub const PS2_BEZIER_MAN_FREE_CACHES: Callee = Callee {
        addr: 0x0038_b0f8,
        f: ee::s12::f_0038b0f8,
    };

    pub const BEZIER_MAN_BASE_DTOR: Callee = Callee {
        addr: 0x0037_4ca8,
        f: ee::s12::f_00374ca8,
    };
}

/// `void* PS2BezierMan_dtor(this, flags)` (0x38aeb0): install the bezier
/// manager vtable, free its caches and run the bezier-base destructor.
pub fn ps2_bezier_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_add(0x3760));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(c.g[S0][0] as u32).off(4).sw(c, c.g[V0][0]);
        PS2_BEZIER_MAN_FREE_CACHES.call(c, 0x0038_aed8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        BEZIER_MAN_BASE_DTOR.call(c, 0x0038_aee4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `PS2BezierMan* PS2BezierMan_ctor(PS2BezierMan* this)` (0x38ae60).
///
/// Runs the `BezierManBase` constructor, installs the vtable (0x493760 at
/// `this+4`), constructs the patch cache at `this+0x10`, runs
/// `PS2BezierMan_Init` and returns `this`.
pub fn ps2_bezier_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0037_4c90,
            f: ee::s12::f_00374c90,
        }
        .call(c, 0x0038_ae74)?;
        c.g[V1][0] = 0x49_0000;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3760));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        Callee {
            addr: 0x0037_2b30,
            f: ee::s12::f_00372b30,
        }
        .call(c, 0x0038_ae88)?;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = 0x1;
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x0038_af30,
            f: ee::s12::f_0038af30,
        }
        .call(c, 0x0038_ae98)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;

    use crate::difftest::DATA;

    // -----------------------------------------------------------------------
    // 0x38aeb0 PS2BezierMan_dtor.

    #[test]
    fn ps2_bezier_man_dtor_matches() {
        check(
            ee::s12::f_0038aeb0__generated,
            ps2_bezier_man_dtor,
            8,
            &|c, seed| {
                let _r = machine(c, seed, 0x38ae_b0, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = ((seed as u64) & 0xfe) << 1; // flags bit 0 clear: no delete
                for off in [0x440u32, 0x444, 0x448] {
                    c.bus.write32(DATA + 0x1000 + off, 0);
                }
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;

    pub(crate) use crate::game::fe::debugfe::tests::oc793_machine;

    /// `PS2BezierMan_ctor`: the base and patch cache constructors are fakes.
    /// `PS2BezierMan_Init` (0x38af30, no override slot: a big real body)
    /// cannot be bounded on test data, so every seed unwinds in one of the
    /// first two calls and the path through `Init` is not covered (it is
    /// `v0 = this` plus the epilogue).
    #[test]
    fn ps2_bezier_man_ctor_matches() {
        covered(
            ee::s12::f_0038ae60__generated,
            ps2_bezier_man_ctor,
            60,
            &|c, seed| {
                let _ = oc793_machine(c, seed, 0x0038_ae60, 2);
                c.bus.write32(fakes::LOG_UNWIND_AT, seed % 2);
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }
}
