//! `src/fe/festatecharequip.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Frame;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0015_d700, camera_base_init, "CameraBase_Init"),
    (0x0019_57d0, femodule_base_dtor, "FEModule_BaseDtor"),
    (0x0019_5880, fe_locale_ctor, "FELocale_ctor"),
    (0x0019_58f0, fe_locale_v01, "FELocale_v01"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_ALL_TABLES;
}

/// Cross-shard call targets (the generator emits `call_indirect` for them).
pub(crate) mod table {
    pub(crate) use crate::game::ai::computer::table::DELETE;
}

/// `void* FELocale_v01(FELocale* l, int flags)` (0x1958f0): the deleting
/// destructor of the locale object: set the base vtable, unload every table,
/// switch to the base vtable and `delete` for bit 0.
pub fn fe_locale_v01(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(0x0046_bf98);
    frame.save(c, S1, 0x10);
    frame.save(c, S0, 0x20);
    c.g[S1][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A1][0];
    c.sw(p(c, S1, 4), c.g[V0][0]);
    if callee::FE_LOCALE_UNLOAD_ALL_TABLES
        .call(c, 0x0019_5918)
        .is_err()
    {
        return;
    }
    c.g[V0][0] = sx(0x0046_d848);
    c.g[S0][0] &= 1;
    let keep = c.g[S0][0] != 0;
    c.sw(p(c, S1, 4), c.g[V0][0]);
    if keep {
        c.cycles += 2;
        c.g[A0][0] = c.g[S1][0];
        if call_ptr(c, table::DELETE, 0x0019_5934).is_err() {
            return;
        }
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void* FELocale_ctor(FELocale* self)` (0x195880): install both locale
/// v-tables, clear the flags and zero the 0x50-byte language table.
pub fn fe_locale_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x46d8_48;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(at(c, S0, 0)).sb(c, 0);
        Addr(at(c, S0, 1)).sb(c, 0);
        Addr(at(c, S0, 2)).sb(c, 0);
        Addr(at(c, S0, 4)).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx(at(c, S0, 3));
        call_ptr(c, 0x001d_cc40, 0x0019_58b0)?;
        c.g[V1][0] = 0x46bf_98;
        c.g[V0][0] = 4;
        Addr(at(c, S0, 0x58)).sb(c, 0);
        Addr(at(c, S0, 4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx(at(c, S0, 8));
        Addr(at(c, S0, 2)).sb(c, c.g[V0][0]);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x50;
        call_ptr(c, 0x0041_6210, 0x0019_58d8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void CameraBase_Init(CameraBase* cam)` (0x0015d700): clear the frame
/// (position, rotation, 1.0 scalar), load the three axes and the base
/// orientation into `+0x80`, then build the matrices.
pub fn camera_base_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.sw(sp(c), 0);
        c.g[V1][0] = c.g[A0][0];
        c.sw(sp(c).wrapping_add(4), 0);
        c.g[A1][0] = sx(0x004f_f140);
        c.sw(sp(c).wrapping_add(8), 0);
        c.sw(sp(c).wrapping_add(0xc), 0);
        c.sd(sp(c).wrapping_add(0x10), c.g[RA][0]);
        c.lq(6, sp(c));
        c.swc1(0, sp(c).wrapping_add(4));
        c.swc1(0, sp(c).wrapping_add(0xc));
        c.sw(sp(c), 0);
        c.sw(sp(c).wrapping_add(8), 0);
        c.swc1(12, c.g[V1][0] as u32);
        c.lq(2, sp(c));
        c.swc1(13, (c.g[V1][0] as u32).wrapping_add(4));
        c.swc1(14, (c.g[V1][0] as u32).wrapping_add(8));
        c.sq(6, (c.g[V1][0] as u32).wrapping_add(0x20));
        c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x30));
        c.swc1(12, (c.g[V1][0] as u32).wrapping_add(0xc));
        c.swc1(13, (c.g[V1][0] as u32).wrapping_add(0x10));
        c.swc1(14, (c.g[V1][0] as u32).wrapping_add(0x14));
        c.lq(2, c.g[A1][0] as u32);
        c.g[RA][0] = sx(0x0015_d770);
        c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x80));
        call_ptr(c, 0x0015_d928, 0x0015_d770)?; // CameraBase_UpdateMatrices
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0x10));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `FEModule_BaseDtor` (0x1957d0): the front-end module base destructor:
// release the async file, the world graphics manager, the rider previews, the
// bolt cache and the FX loader, install the base vtables and free the module
// when `flags & 1`.
pub fn femodule_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x470000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2788));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
        let no_file = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_file {
            c.cycles += 2;
            c.g[A1][0] = 0x3;
            call_ptr(c, 0x0039_7b70, 0x0019_5804)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        call_ptr(c, 0x0036_a020, 0x0019_580c)?;
        c.g[V0][0] = 0xb0000;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5aa0));
        let none = c.g[A0][0] == 0;
        if none {
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1a70));
        } else {
            c.cycles += 3;
            c.g[A1][0] = 0x3;
            call_ptr(c, 0x002d_bee0, 0x0019_5828)?;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1a70));
        }
        c.cycles += 13;
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0019_ce20, 0x0019_5834)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb0));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x001a_0420, 0x0019_5840)?;
        c.g[V0][0] = 0x460000;
        c.g[V1][0] = 0x470000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4750));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2690));
        c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0] & 0x1;
        let keep = c.g[A0][0] == 0;
        c.sw(c.g[S0][0] as u32, c.g[V1][0]);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0019_5868)?;
        }
        c.cycles += 5;
        c.lq(16, frame.at(c, 0x20));
        c.lq(17, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;

    use crate::difftest::fill;

    use crate::difftest::DATA;

    pub(crate) use crate::game::bx::bxstringctor::tests::alloc_base;

    pub(crate) use crate::game::render::particle::tests::gp_at;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_2;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::alloc;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_init;

    use ssx3_ee as ee;

    #[test]
    fn fe_locale_v01_matches() {
        check(
            ssx3_ee::s02::f_001958f0__generated,
            fe_locale_v01,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1958f0, 0);
                heap_init(c);
                let this = alloc(c, 0x100);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn fe_locale_ctor_matches() {
        check(
            ee::s02::f_00195880__generated,
            fe_locale_ctor,
            30,
            &|c, seed| {
                base(c, seed, 0x1958_80);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn camera_base_init_matches() {
        check(
            ee::s01::f_0015d700__generated,
            camera_base_init,
            80,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x15d7, 1);
                fill(c, &mut r, DATA, 0x90);
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
                c.f[14] = r.edge_f32();
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn femodule_base_dtor_matches() {
        check(
            ee::s02::f_001957d0__generated,
            femodule_base_dtor,
            30,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x1957);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(gp_at(-0x854), DATA + 0x3000);
                c.bus.write32(DATA + 0x3000 + 0x10d8, DATA + 0x4000);
                c.bus.write32(DATA + 0xc, 0);
                c.bus.write32(DATA + 0xb5aa0, 0);
            },
        );
    }
}
