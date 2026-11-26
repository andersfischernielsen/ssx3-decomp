//! `src/render/irradiance.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
pub(crate) use crate::game::fe::feasyncfile::lw_8 as lw;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::sub;
pub(crate) use crate::game::fe::feasyncfile::sw_5 as sw;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0038_adb0, irradiance_free, "Irradiance_Free"),
    (0x0038_acd0, irradiance_load_811, "Irradiance_Load"),
];

// ---------------------------------------------------------------------------
// Callees the generator emits as `call_indirect` (cross-shard or indirect).

pub(crate) mod addr {
    pub(crate) use crate::game::fe::feasyncfile::addr::FILE_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::addr::MEMCPY;
    pub(crate) use crate::game::fe::feasyncfile::addr::MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::addr::OPERATOR_NEW_NAMED;
}

// ---------------------------------------------------------------------------
// Render / FE.

/// `void Irradiance_Free(Irradiance* self)` (0x38adb0): release the two
/// allocation lists and clear the owner word.
pub fn irradiance_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let base = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(p(base, 4)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr(p(base, 0xc)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x0038_ade0)?;
            }
            c.g[A0][0] = Addr(p(base, 8)).lw(c);
            c.cycles += 2;
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x0038_adf4)?;
            }
            Addr(p(base, 4)).sw(c, 0);
            c.cycles += 3;
            Addr(p(base, 0xc)).sw(c, 0);
            Addr(p(base, 8)).sw(c, 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Irradiance_Load (0x0038acd0).
//
// NOTE: not installed (no `OVERRIDES` entry): the test cannot bound the work,
// because `FILE_load` waits for the IOP file-request queue, which never
// completes on a test machine. The port itself is exact.

/// `void cIrradianceDataBase_Load(cIrradianceDataBase* db)`:
/// load the irradiance data file, allocate the index table (`count * 8`) and
/// the sample array (`count * 0xa0`), copy both from the file image, then
/// release the file buffer.
pub fn irradiance_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 45;
        c.g[SP][0] = sub(c.g[SP][0], 0x60);
        c.g[A1][0] = 0x100;
        c.sq(16, p(c.g[SP][0] as u32, 0x50));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, p(c.g[SP][0] as u32, 0x40));
        c.g[A0][0] = sx(0x0049_2e68);
        c.sq(18, p(c.g[SP][0] as u32, 0x30));
        c.sq(19, p(c.g[SP][0] as u32, 0x20));
        c.sq(20, p(c.g[SP][0] as u32, 0x10));
        c.sd(p(c.g[SP][0] as u32, 0), c.g[RA][0]);
        call_ptr(c, addr::FILE_LOAD, 0x0038_ad00)?;
        // db->file = f; count = f[0]; index/samples allocated and copied.
        sw(c, p(c.g[S0][0] as u32, 0), c.g[V0][0]);
        c.g[S2][0] = 0xa0;
        c.g[A1][0] = sx(0x0049_2e80);
        c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
        c.g[V1][0] = lw(c, c.g[V0][0] as u32);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[S1][0] = sx((c.g[V1][0] as u32) << 3);
        c.mult(0, 18, 3, 18);
        sw(c, p(c.g[S0][0] as u32, 4), c.g[V1][0]);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, addr::OPERATOR_NEW_NAMED, 0x0038_ad34)?;
        c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[A1][0] = sx(0x0049_2e90);
        c.g[A3][0] = 0;
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = 0x200_0000;
        sw(c, p(c.g[S0][0] as u32, 0xc), c.g[V0][0]);
        call_ptr(c, addr::OPERATOR_NEW_NAMED, 0x0038_ad54)?;
        c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 0xc));
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S1][0];
        sw(c, p(c.g[S0][0] as u32, 8), c.g[V0][0]);
        call_ptr(c, addr::MEMCPY, 0x0038_ad68)?;
        c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 8));
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S2][0];
        call_ptr(c, addr::MEMCPY, 0x0038_ad78)?;
        c.g[A0][0] = lw(c, c.g[S0][0] as u32);
        let empty = c.g[A0][0] == 0;
        c.lq(16, p(c.g[SP][0] as u32, 0x50));
        if !empty {
            c.cycles += 3;
            call_ptr(c, addr::MEM_FREE, 0x0038_ad8c)?;
            c.lq(16, p(c.g[SP][0] as u32, 0x50));
        }
        c.cycles += 7;
        c.lq(17, p(c.g[SP][0] as u32, 0x40));
        c.lq(18, p(c.g[SP][0] as u32, 0x30));
        c.lq(19, p(c.g[SP][0] as u32, 0x20));
        c.lq(20, p(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        Ok(())
    });
}

/// `void cIrradianceDataBase_Load(db)` (`Irradiance_Load`, 0x0038acd0): load the
/// irradiance file, allocate the index table (`count * 8`) and the sample array
/// (`count * 0xa0`), copy both from the file image and release the file buffer.
/// The normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn irradiance_load_811(c: &mut Cpu) {
    if c.pc != 0x0038acd0 {
        return ee::s12::f_0038acd0__generated(c);
    }
    guest(c, |c| {
        'b_38ad90: {
            c.cycles += 45;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.g[A1][0] = 0x100;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[A0][0] = 0x490000;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x0038ad00);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2e68));
            call_ptr(c, 0x003e18c8, 0x0038ad00)?;
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.g[S2][0] = 0xa0;
            c.g[A1][0] = 0x490000;
            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2e80));
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            c.g[S1][0] = sx((c.g[V1][0] as u32) << 3);
            c.mult(0, 18, 3, 18);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
            c.g[RA][0] = sx(0x0038ad34);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x00317e30, 0x0038ad34)?;
            c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[A1][0] = 0x490000;
            c.g[A3][0] = 0u64;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2e90));
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = 0x2000000;
            c.g[RA][0] = sx(0x0038ad54);
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            call_ptr(c, 0x00317e30, 0x0038ad54)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x0038ad68);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            call_ptr(c, 0x0041605c, 0x0038ad68)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x0038ad78);
            c.g[A2][0] = c.g[S2][0];
            call_ptr(c, 0x0041605c, 0x0038ad78)?;
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            let t = c.g[A0][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            if t {
                break 'b_38ad90;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0038ad8c);
                call_ptr(c, 0x00317e98, 0x0038ad8c)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_38ad90;
            }
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    pub(crate) use crate::game::ui::uiengine::tests::oc811_install_fakes;

    use crate::difftest::DATA;

    use ssx3_ee as ee;

    #[test]
    fn irradiance_free_matches() {
        check(
            ee::s12::f_0038adb0__generated,
            irradiance_free,
            24,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0038_adb0, 1);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(DATA + 4, if seed % 3 == 0 { 0 } else { r.next_u32() | 1 });
                c.bus.write32(DATA + 8, 0);
                c.bus.write32(DATA + 0xc, 0);
            },
        );
    }

    use crate::difftest::fakes::LOG_RESULTS;

    pub(crate) use crate::game::ui::uiengine::tests::OC811_CTRL;
    pub(crate) use crate::game::ui::uiengine::tests::OC811_MAGIC;
    pub(crate) use crate::game::ui::uiengine::tests::OC811_MAIL;

    /// Irradiance_Load: the loader, both allocations, both copies and the
    /// release of the file buffer (skipped when the loader returns 0), with
    /// the callees faked; fake call n unwinds on some seeds.
    #[test]
    fn irradiance_load_matches() {
        oc811_install_fakes();
        check(
            ee::s12::f_0038acd0__generated,
            irradiance_load_811,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0038_acd0, 6);
                c.pc = 0x0038_acd0;
                c.bus.write32(OC811_CTRL, OC811_MAGIC);
                c.bus.write32(OC811_MAIL, (seed % 4 == 0) as u32);
                let file = DATA + 0x1000;
                c.bus.write32(file, r.next_u32() % 7);
                c.bus.write32(LOG_RESULTS, file);
                c.g[A0][0] = DATA as u64;
                for w in 0..4 {
                    c.bus.write32(DATA + 4 * w, r.next_u32());
                }
            },
        );
    }
}
