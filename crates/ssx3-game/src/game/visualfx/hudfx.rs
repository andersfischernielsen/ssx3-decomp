//! `src/visualfx/hudfx.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_6fa8, chudfx_ctor, "cHUDFX_ctor"),
    (0x0011_70a8, chudfx_show_number, "cHUDFX_ShowNumber"),
    (
        0x0011_7138,
        chudfx_show_delayed_00117138,
        "cHUDFX_ShowDelayed",
    ),
];

/// `cHUDFX *cHUDFX_ctor(cHUDFX *this)` (0x116fa8): set the state word at +0 to
/// 0x34 and return `this`.
pub fn chudfx_ctor(c: &mut Cpu) {
    c.cycles += 4;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = this.0 as u64;
    c.g[V1][0] = 0x34;
    Addr(c.g[V0][0] as u32).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void cHUDFX::set(id, int displayInt, float decayTime, int data)`
/// (0x001170a8): formats `a2` into the number text buffer (0x4a5e20, through
/// the format at 0x4a1070), sets it on the HUD slot `a0` with
/// `cHUDFX_SetText`, and stores the slot's type, value, duration and data.
pub fn chudfx_show_number(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x70);
        frame.save(c, 16, 0x50);
        frame.save(c, 20, 0x10);
        c.g[S0][0] = 0x4a0000;
        frame.save(c, 17, 0x40);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, 18, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 19, 0x20);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5e20));
        frame.save_fpr(c, 20, 0x60);
        c.g[S2][0] = c.g[A2][0];
        c.g[A1][0] = 0x4a0000;
        c.mov_s(20, 12);
        c.g[S3][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1070));
        call_ptr(c, 0x0041_7828, 0x0011_70f4)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0011_7008,
            f: ee::s00::f_00117008,
        }
        .call(c, 0x0011_7100)?;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[S2][0]);
        c.sw(c.g[S1][0] as u32, c.g[S4][0]);
        c.swc1(20, (c.g[S1][0] as u32).wrapping_add(0x4));
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[S3][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// `void cHUDFX_ShowDelayed(cHUDFX* fx, int fxId, char* text, int data, float decay)`
/// (0x117138): sets the text, then the id, data, decay (`-decay`, the current
/// decay), `MaxDecay = -1.0f` and `DisplayInt = 0`.
pub fn chudfx_show_delayed_00117138(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x40);
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        c.g[A1][0] = c.g[A2][0];
        Callee {
            addr: 0x0011_7008,
            f: ee::s00::cHUDFX_SetText,
        }
        .call(c, 0x0011_7168)?;
        c.neg_s(20, 20);
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 0);
        c.sw(c.g[S0][0] as u32, c.g[S1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[S2][0]);
        c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x8));
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    #[test]
    fn chudfx_ctor_matches() {
        check(ssx3_ee::s00::f_00116fa8, chudfx_ctor, 50, &|c, seed| {
            let mut r = Rng(0x4455_6677 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }

    use crate::difftest::fakes;

    use crate::difftest::fakes::machine;
    use crate::difftest::fill;
    pub(crate) use crate::game::be::belibrary::tests::OC1535_MAGIC;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    #[test]
    fn chudfx_show_number_matches() {
        check(
            ee::s00::f_001170a8__generated,
            chudfx_show_number,
            200,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1486_1170, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32());
                c.g[A3][0] = sx(r.next_u32());
                c.f[12] = r.edge_f32();
            },
        );
    }

    pub(crate) const OC1535_TEXT_GATE: u32 = DATA + 0x20_1100;

    pub(crate) static OC1535_TEXT_PREV: AtomicUsize = AtomicUsize::new(0);

    /// `cHUDFX_SetText` (0x117008), gated.
    pub(crate) fn oc1535_fake_set_text(c: &mut Cpu) {
        if c.bus.read32(OC1535_TEXT_GATE) != OC1535_MAGIC {
            return fakes::fall_back(c, &OC1535_TEXT_PREV, ee::s00::f_00117008__generated);
        }
        fakes::fake_call(c, 0x0011_7008);
    }

    #[test]
    fn chudfx_show_delayed_00117138_matches() {
        check(
            ee::s00::f_00117138__generated,
            chudfx_show_delayed_00117138,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x117138, 1);
                fakes::hook(
                    addr_of_mut!(ee::s00::OVERRIDE_00117008),
                    oc1535_fake_set_text,
                    &OC1535_TEXT_PREV,
                );
                c.bus.write32(OC1535_TEXT_GATE, OC1535_MAGIC);
                fill(c, &mut r, DATA + 0x3000, 0x10);
                c.g[A0][0] = sx(DATA + 0x3000);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(DATA + 0x3100);
                c.g[A3][0] = sx(r.next_u32());
                c.f[12] = r.f32(100.0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
