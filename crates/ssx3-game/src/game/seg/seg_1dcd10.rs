//! `src/seg/seg_1dcd10.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_bd10, fx_loader_construct, "FXLoader_ctor"),
    (0x002d_bee0, fx_loader_dtor, "cFXLoader__cFXLoader"),
    (
        0x002d_c168,
        desc_key4_sort_compare,
        "desc_key4_sort_compare",
    ),
    (
        0x002d_c7b0,
        c_ps2_fog_particle_man_draw_particles,
        "FogParticleMan_DrawParticles",
    ),
];

/// `FXLoader_ctor` (0x002dbd10): `cFXLoader::cFXLoader`: loads the particle (and unless `particles_only`, the effects) texture files, binds the 81 effect shapes to texture ids and fills the ten effect texture slots. `(self, particles_only)`.
/// NOT INSTALLED (no OVERRIDES entry, no test): EAFile_LoadFileEx (pc 0x3de768) has no override slot and spins on test data.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn fx_loader_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2dbda4: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.g[V0][0] = 0x1;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S4][0] = 0x2;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
            if c.g[A1][0] != 0 {
                c.g[S4][0] = c.g[V0][0];
            }
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S0][0] = 0u64;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            let t = c.g[S4][0] == 0u64;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A0][0]);
            if t {
                break 'b_2dbda4;
            } else {
                c.cycles += 3;
                c.g[S3][0] = 0x1;
                c.g[S2][0] = 0x480000;
                c.g[S1][0] = 0x480000;
                'l_2dbd60: loop {
                    'b_2dbd98: {
                        'b_2dbd94: {
                            'b_2dbd88: {
                                'b_2dbd78: {
                                    c.cycles += 2;
                                    if c.g[S0][0] == 0u64 {
                                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x7918));
                                        break 'b_2dbd78;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S0][0] == c.g[S3][0];
                                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x7938));
                                        if t {
                                            break 'b_2dbd88;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                            if t {
                                                break 'b_2dbd98;
                                            } else {
                                                break 'b_2dbd78;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[A1][0] = 0x100;
                                call_ptr(c, 0x003e18c8, 0x002dbd80)?; // call_indirect
                                let t = true;
                                c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                                if t {
                                    break 'b_2dbd94;
                                } else {
                                    break 'b_2dbd88;
                                }
                            }
                            c.cycles += 3;
                            c.g[A1][0] = 0x100;
                            call_ptr(c, 0x003e18c8, 0x002dbd90)?; // call_indirect
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            break 'b_2dbd94;
                        }
                        c.cycles += 1;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        break 'b_2dbd98;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002dbd60);
                        }
                        continue 'l_2dbd60;
                    } else {
                        break 'b_2dbda4;
                    }
                }
            }
        }
        c.cycles += 9;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
        c.g[V0][0] = 0x490000;
        c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6e50));
        c.g[S1][0] = 0u64;
        c.g[FP][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10b0));
        c.g[S6][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf50));
        c.g[S7][0] = 0xc;
        c.g[A1][0] = 0u64;
        'l_2dbdc8: loop {
            c.cycles += 4;
            c.g[S0][0] = 0u64;
            c.g[S2][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
            'l_2dbdd8: loop {
                'b_2dbe04: {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.mult(0, 2, 17, 23);
                    if t {
                        break 'b_2dbe04;
                    } else {
                        c.cycles += 8;
                        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                        call_ptr(c, 0x003b3900, 0x002dbdf8)?; // call_indirect
                        c.g[A1][0] = c.g[V0][0];
                        if c.g[A1][0] == 0u64 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                            if event_due(c) {
                                deliver_event_here(c, 0x002dbdd8);
                            }
                            continue 'l_2dbdd8;
                        } else {
                            break 'b_2dbe04;
                        }
                    }
                }
                'b_2dbe44: {
                    c.cycles += 2;
                    let t = c.g[A1][0] == 0u64;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        break 'b_2dbe44;
                    } else {
                        c.cycles += 14;
                        c.mult(0, 6, 17, 23);
                        c.g[T3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                        c.g[V0][0] = 0x490000;
                        c.g[T0][0] = 0x1;
                        c.g[V1][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x10d8));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6e48));
                        c.g[T1][0] = 0xffffffffffffffff;
                        c.g[T2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[S5][0] as u32));
                        c.g[A3][0] = c.lw(c.g[T2][0] as u32);
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x002dbe44)?; // call_indirect
                        break 'b_2dbe44;
                    }
                }
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[S1][0] = c.g[S3][0];
                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                c.g[V0][0] = ((c.g[S1][0] as i64) < 81i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = 0u64;
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x002dbdc8);
                    }
                    continue 'l_2dbdc8;
                } else {
                    'b_2dbe88: {
                        c.cycles += 2;
                        let t = c.g[S4][0] == 0u64;
                        c.g[A0][0] = c.g[FP][0];
                        if t {
                            break 'b_2dbe88;
                        } else {
                            c.cycles += 3;
                            c.g[S1][0] = c.g[S4][0];
                            c.g[S0][0] = c.g[SP][0];
                            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                            'l_2dbe70: loop {
                                c.cycles += 5;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                                call_ptr(c, 0x002523a8, 0x002dbe7c)?; // call_indirect
                                if c.g[S1][0] != 0u64 {
                                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002dbe70);
                                    }
                                    continue 'l_2dbe70;
                                } else {
                                    c.cycles += 1;
                                    c.g[A0][0] = c.g[FP][0];
                                    break 'b_2dbe88;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x11c));
                    c.g[A1][0] = 0x9;
                    'l_2dbe90: loop {
                        c.cycles += 7;
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                        let t = (c.g[A1][0] as i64) >= 0;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002dbe90);
                            }
                            continue 'l_2dbe90;
                        } else {
                            c.cycles += 13;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                            c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                            c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                            c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                            c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                            let t = c.g[RA][0] as u32;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                            c.pc = t;
                            return Ok(());
                        }
                    }
                }
            }
        }
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    #[allow(unused_imports)]
    use crate::game::fe::ovstatereplay::tests::oc1499_install_fakes;
    pub(crate) use crate::game::fe::ovstatereplay::tests::GATE_DELETE;
    pub(crate) use crate::game::fe::ovstatereplay::tests::MAGIC;
    pub(crate) use crate::game::render::graphicsman::tests::oc1483_common;
    pub(crate) use crate::game::render::graphicsman::tests::oc1483_fog_setup;
    pub(crate) use crate::game::render::graphicsman::tests::oc1483_w;

    pub(crate) use crate::game::render::graphicsman::tests::OC1483_CTRL;
    pub(crate) use crate::game::render::graphicsman::tests::OC1483_GM;

    pub(crate) use crate::game::render::graphicsman::tests::OC1483_MAGIC;

    pub(crate) use crate::game::render::graphicsman::tests::OC1483_STREAM;
    pub(crate) use crate::game::render::particle::tests::oc1483_fill_f32;
    pub(crate) use crate::game::render::particle::tests::oc1483_v86_setup;

    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    pub(crate) const OC1483_THIS: u32 = DATA + 0x9_0000;

    /// The particle index table the draw reads (a global in the game image).
    pub(crate) const OC1483_TABLE: u32 = 0x0053_7ac0;

    macro_rules! oc1483_fakes {
        ($($name:ident, $addr:expr, $slot:path, $gen:path;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC1483_CTRL) != OC1483_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                    }
                    fakes::fake_call(c, $addr);
                    oc1483_after(c, $addr);
                }
            )*
            /// Puts every fake in its override slot (once per process).
            pub(crate) fn oc1483_hook() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    /// `VramHeap_Alloc` answers a small block number (the depth-fog code
    /// multiplies it by 0x1c to index a table).
    pub(crate) fn oc1483_after(c: &mut Cpu, target: u32) {
        if target == 0x0036_6ce0 {
            c.g[V0][0] = 1 + c.g[V1][0] % 3;
        }
    }

    oc1483_fakes! {
        oc1483_fake_0036aba0, 0x0036_aba0, ee::s12::OVERRIDE_0036ABA0, ee::s12::f_0036aba0__generated;
        oc1483_fake_0036a428, 0x0036_a428, ee::s12::OVERRIDE_0036A428, ee::s12::f_0036a428__generated;
        oc1483_fake_0038f460, 0x0038_f460, ee::s12::OVERRIDE_0038F460, ee::s12::f_0038f460__generated;
        oc1483_fake_00366ce0, 0x0036_6ce0, ee::s12::OVERRIDE_00366CE0, ee::s12::f_00366ce0__generated;
        oc1483_fake_0036aa60, 0x0036_aa60, ee::s12::OVERRIDE_0036AA60, ee::s12::f_0036aa60__generated;
        oc1483_fake_0038f668, 0x0038_f668, ee::s12::OVERRIDE_0038F668, ee::s12::f_0038f668__generated;
        oc1483_fake_0036ae20, 0x0036_ae20, ee::s12::OVERRIDE_0036AE20, ee::s12::f_0036ae20__generated;
        oc1483_fake_0036b158, 0x0036_b158, ee::s12::OVERRIDE_0036B158, ee::s12::f_0036b158__generated;
        oc1483_fake_003663d8, 0x0036_63d8, ee::s12::OVERRIDE_003663D8, ee::s12::f_003663d8__generated;
        oc1483_fake_00368138, 0x0036_8138, ee::s12::OVERRIDE_00368138, ee::s12::f_00368138__generated;
        oc1483_fake_0037d968, 0x0037_d968, ee::s12::OVERRIDE_0037D968, ee::s12::f_0037d968__generated;
    }

    /// The seeds take several distinct paths (disabled, unset, with and
    /// without the parameter reset, unwinds).
    #[test]
    fn ps2_graphics_man_render_depth_fog_paths() {
        use crate::difftest::{fresh, run_generated};
        let mut totals = std::collections::BTreeSet::new();
        for seed in 0..120 {
            let mut c = fresh();
            oc1483_fog_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s12::f_0036ac00__generated);
            totals.insert(c.cycles - before);
        }
        assert!(totals.len() >= 4, "only {} distinct paths", totals.len());
    }

    #[test]
    fn ps2_graphics_man_v86_paths() {
        use crate::difftest::{fresh, run_generated};
        let mut totals = std::collections::BTreeSet::new();
        for seed in 0..100 {
            let mut c = fresh();
            oc1483_v86_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s12::f_003816f0__generated);
            totals.insert(c.cycles - before);
        }
        assert!(totals.len() >= 3, "only {} distinct paths", totals.len());
    }

    /// `cPS2FogParticleMan_DrawParticles(this)`: draws `this[0]` fog particles
    /// in packets of at most `s7` sprites (the chunk size comes from the
    /// stream's free space divided by the graphics manager address, a quirk of
    /// the original); odd chunks add a terminating tag word.
    pub(crate) fn oc1483_particles_setup(c: &mut Cpu, seed: u32) {
        oc1483_hook();
        let mut r = fakes::machine(c, seed, 0x1483_0003, 16);
        oc1483_common(c, &mut r, seed);
        oc1483_w(c, GAME_GP + 0x2a90, OC1483_GM);
        // Free space word: about 2k..2k+1 chunks of the manager's address.
        let k = 1 + r.next_u32() % 4;
        oc1483_w(
            c,
            OC1483_STREAM + 0x10,
            OC1483_GM * k + r.next_u32() % OC1483_GM,
        );
        oc1483_w(c, OC1483_THIS, r.next_u32() % 31);
        oc1483_fill_f32(c, &mut r, OC1483_THIS + 0x10, 0x80, 50.0);
        for i in 0..40u32 {
            let idx = (r.next_u32() % 6) * 0x30;
            oc1483_w(c, OC1483_TABLE + 8 * i, idx);
        }
        c.g[A0][0] = OC1483_THIS as u64;
        c.pc = 0x002d_c7b0;
    }

    /// Both stream calls of the particle draw are reached (and some unwind).
    #[test]
    fn c_ps2_fog_particle_man_draw_particles_matches() {
        fakes::check_reaching(
            ee::s09::f_002dc7b0__generated,
            c_ps2_fog_particle_man_draw_particles,
            120,
            &[0x002d_c85c, 0x002d_caa0],
            &|c, seed| oc1483_particles_setup(c, seed),
        );
    }

    #[test]
    fn c_ps2_fog_particle_man_draw_particles_paths() {
        use crate::difftest::{fresh, run_generated};
        let mut totals = std::collections::BTreeSet::new();
        for seed in 0..120 {
            let mut c = fresh();
            oc1483_particles_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s09::f_002dc7b0__generated);
            totals.insert(c.cycles - before);
        }
        assert!(totals.len() >= 6, "only {} distinct paths", totals.len());
    }

    #[test]
    fn fx_loader_dtor_matches() {
        oc1499_install_fakes();
        check(
            ee::s09::f_002dbee0__generated,
            fx_loader_dtor,
            200,
            &|c: &mut Cpu, seed: u32| {
                let mut r = fakes::machine(c, seed, 0x2d_bee0, 0);
                c.bus.write32(GATE_DELETE, MAGIC);
                let grman = DATA + 0x3000;
                c.bus.write32(GAME_GP.wrapping_add(0x2a90), grman);
                // The texture manager at +0x10d8: this-delta at +0x190, method
                // at +0x194 (a miss-hook fake at FAKE).
                let texman = DATA + 0x4000;
                c.bus.write16(texman + 0x190, r.next_u32() as u16);
                c.bus.write32(texman + 0x194, fakes::FAKE);
                c.bus.write32(grman + 0x10d8, texman);
                // The 81 texture slots: about half empty (-1).
                for i in 0..81u32 {
                    let v = if r.next_u32() % 2 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 1000
                    };
                    c.bus.write32(grman + 0xf50 + 4 * i, v);
                }
                c.g[A0][0] = (DATA + 0x6000) as u64;
                // Bit 0 of flags: the object is deleted on half of the seeds.
                c.g[A1][0] = (r.next_u32() as u64 & !1) | (seed % 2) as u64;
            },
        );
    }

    /// Gate words and the magic that opens them (unique fixture addresses).
    pub(crate) const OC1574_MAGIC: u32 = 0x1574_0001;

    pub(crate) const OC1574_FILE_CTRL: u32 = DATA + 0x20_1008;

    pub(crate) const OC1574_SHAPE_CTRL: u32 = DATA + 0x20_100c;

    pub(crate) const OC1574_FREE_CTRL: u32 = DATA + 0x20_1010;

    pub(crate) const OC1574_SHAPE_SALT: u32 = DATA + 0x20_1014;

    pub(crate) static OC1574_PREV_FILE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1574_PREV_SHAPE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1574_PREV_FREE: AtomicUsize = AtomicUsize::new(0);

    /// `FILE_load`: a logged call returning a handle while its gate is on.
    pub(crate) fn oc1574_fake_file(c: &mut Cpu) {
        if c.bus.read32(OC1574_FILE_CTRL) != OC1574_MAGIC {
            return fakes::fall_back(c, &OC1574_PREV_FILE, ee::s14::f_003e18c8__generated);
        }
        fakes::fake_call(c, FXLOADER_FILE_LOAD);
    }

    /// `SHAPE_locatez`: found or not found, by a hash of the entry and a salt
    /// per seed, so both arms of the search run.
    pub(crate) fn oc1574_fake_shape(c: &mut Cpu) {
        if c.bus.read32(OC1574_SHAPE_CTRL) != OC1574_MAGIC {
            return fakes::fall_back(c, &OC1574_PREV_SHAPE, ee::s13::f_003b3900__generated);
        }
        let entry = c.g[A1][0] as u32;
        let salt = c.bus.read32(OC1574_SHAPE_SALT) as u64;
        let found = fakes::mix(entry as u64 ^ salt) % 3 != 0;
        fakes::fake_call(c, FXLOADER_SHAPE_LOCATEZ);
        c.g[V0][0] = if found {
            sx(DATA + 0x20_2000 + (entry & 0xfff))
        } else {
            0
        };
    }

    /// `MEM_free`: a logged call while its gate is on.
    pub(crate) fn oc1574_fake_free(c: &mut Cpu) {
        if c.bus.read32(OC1574_FREE_CTRL) != OC1574_MAGIC {
            return fakes::fall_back(c, &OC1574_PREV_FREE, ee::s06::f_002523a8__generated);
        }
        fakes::fake_call(c, FXLOADER_MEM_FREE);
    }

    pub(crate) fn oc1574_hook_fx() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E18C8),
            oc1574_fake_file,
            &OC1574_PREV_FILE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B3900),
            oc1574_fake_shape,
            &OC1574_PREV_SHAPE,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002523A8),
            oc1574_fake_free,
            &OC1574_PREV_FREE,
        );
    }

    /// The constructor's fixture: the texture manager pointer sits at gp+0x2a90
    /// (`D_BD10`), the manager holds the this-delta at +0x170 and the fake
    /// `AddTexture` at +0x174. Every seed picks `particles_only` (`a1`) and its
    /// own search outcomes.
    #[test]
    fn fx_loader_construct_matches() {
        oc1574_hook_fx();
        check_lohi(
            ee::s09::f_002dbd10__generated,
            fx_loader_construct,
            200,
            &|c, seed| {
                machine(c, seed, 0x2dbd, 0);
                c.bus.write32(OC1574_FILE_CTRL, OC1574_MAGIC);
                c.bus.write32(OC1574_SHAPE_CTRL, OC1574_MAGIC);
                c.bus.write32(OC1574_FREE_CTRL, OC1574_MAGIC);
                c.bus
                    .write32(OC1574_SHAPE_SALT, seed.wrapping_mul(0x9e37_79b9));
                let (grman, mgr) = (DATA + 0x30_000, DATA + 0x32_000);
                c.bus.write32(GAME_GP + 0x2a90, grman);
                c.bus.write32(grman + 0x10d8, mgr);
                c.bus.write16(mgr + 0x170, ((seed % 4) * 0x10) as u16);
                c.bus.write32(mgr + 0x174, fakes::FAKE + 0x10);
                c.g[A0][0] = (DATA + 0x30_800) as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x002d_bd10;
            },
        );
    }

    #[test]
    fn desc_key4_sort_compare_matches() {
        check(
            ee::s09::f_002dc168__generated,
            desc_key4_sort_compare,
            200,
            &|c, seed| {
                machine(c, seed, 0x2dc1, 0);
                let (a, b) = (DATA + 0x100, DATA + 0x200);
                c.bus.write32(a + 4, seed % 3);
                c.bus.write32(b + 4, (seed / 3) % 3);
                c.g[A0][0] = a as u64;
                c.g[A1][0] = b as u64;
                c.pc = 0x002d_c168;
            },
        );
    }
}

/// `cPS2FogParticleMan_DrawParticles` (0x002dc7b0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_ps2_fog_particle_man_draw_particles(c: &mut Cpu) {
    // no resume points: the generated body ignores `c.pc` and always starts at the
    // entry, so every pc runs the body below (the guard is not needed).
    guest(c, |c| {
        'b_2dc7f8: {
            c.cycles += 18;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
            c.g[V1][0] = 0x5;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S2][0] = 0u64;
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S6][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
            if c.g[V1][0] == 0u64 {
                break 'b_2dc7f8;
            } else {
                break 'b_2dc7f8;
            }
        }
        'b_2dcb50: {
            c.cycles += 11;
            c.g[S1][0] = c.lw(c.g[S6][0] as u32);
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
            c.g[S3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
            c.div(0, 2, 3);
            c.g[V0][0] = c.lo[0];
            let t = (c.g[S1][0] as i64) <= 0;
            c.g[S7][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            if t {
                break 'b_2dcb50;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x530000;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.g[FP][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7ac0));
                'l_2dc838: loop {
                    'b_2dc8a4: {
                        'b_2dc890: {
                            c.cycles += 16;
                            c.g[S0][0] = c.g[S7][0];
                            c.g[S5][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5a00));
                            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S0][0] as i64)) as u64;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
                            if c.g[V0][0] != 0 {
                                c.g[S0][0] = c.g[S1][0];
                            }
                            c.g[A1][0] = c.g[S5][0];
                            c.g[A2][0] = 0xffffffffffffffff;
                            c.g[RA][0] = sx(0x002dc85c);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x0038f460, 0x002dc85c)?;
                            c.g[T1][0] = c.g[V0][0];
                            c.sw(c.g[SP][0] as u32, 0u64);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
                            c.g[V0][0] = c.g[S0][0] & 0x1;
                            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                            let t = c.g[V0][0] != 0u64;
                            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x14));
                            if t {
                                break 'b_2dc890;
                            } else {
                                c.cycles += 6;
                                c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                                c.g[T4][0] = 0u64;
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                                let t = true;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3));
                                if t {
                                    break 'b_2dc8a4;
                                } else {
                                    break 'b_2dc890;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                        c.g[T4][0] = 0x1;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                        break 'b_2dc8a4;
                    }
                    'b_2dca80: {
                        c.cycles += 32;
                        c.g[A0][0] = 0x50000000;
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.g[A0][0] = c.g[V0][0] | c.g[A0][0];
                        c.g[V1][0] = c.g[S0][0] | 0x8000;
                        c.g[V0][0] = 0x10000000;
                        c.g[A2][0] = 0xa800;
                        c.g[A2][0] = c.g[A2][0] << 47;
                        c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
                        c.g[A2][0] = c.g[V1][0] | c.g[A2][0];
                        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        c.g[A0][0] = c.g[A0][0] << 32;
                        c.g[V1][0] = 0x14000000;
                        c.g[V1][0] = c.g[V1][0] << 32;
                        c.g[V1][0] = c.g[V1][0] | 0x8001;
                        c.g[A3][0] = 0x56;
                        c.g[T0][0] = 0x50000;
                        c.g[T0][0] = c.g[T0][0] | 0x2512;
                        c.sd(c.g[T1][0] as u32, c.g[A1][0]);
                        c.sd((c.g[T1][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                        c.g[S4][0] = c.g[V0][0];
                        c.sd((c.g[T1][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                        c.sd((c.g[T1][0] as u32).wrapping_add(0x30), c.g[A3][0]);
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        c.sd((c.g[T1][0] as u32).wrapping_add(0x40), c.g[A2][0]);
                        c.sd((c.g[T1][0] as u32).wrapping_add(0x48), c.g[T0][0]);
                        c.sd((c.g[T1][0] as u32).wrapping_add(0x8), 0u64);
                        c.sd((c.g[T1][0] as u32).wrapping_add(0x10), 0u64);
                        c.sd((c.g[T1][0] as u32).wrapping_add(0x28), 0u64);
                        c.sd((c.g[T1][0] as u32).wrapping_add(0x38), 0u64);
                        let t = c.g[V0][0] == 0u64;
                        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x50));
                        if t {
                            break 'b_2dca80;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = sx((c.g[S2][0] as u32) << 3);
                            c.g[T2][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S2][0] as u32));
                            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[FP][0] as u32));
                            c.g[T3][0] = 0x30;
                            'l_2dc938: loop {
                                c.cycles += 82;
                                c.g[A1][0] = c.lw(c.g[T0][0] as u32);
                                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x1));
                                c.lwc1(0, c.g[SP][0] as u32);
                                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x8));
                                c.mult(0, 5, 5, 11);
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                                c.g[A1][0] =
                                    sx((c.g[S6][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                c.lwc1(4, (c.g[A1][0] as u32).wrapping_add(0xc));
                                c.mul_s(0, 0, 4);
                                c.mul_s(1, 1, 4);
                                c.mfc1(2, 4);
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.mfc1(4, 1);
                                c.g[A3][0] = c.g[V0][0];
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                c.g[A3][0] = c.g[A3][0] << 32;
                                c.g[V0][0] = c.g[A0][0] << 32;
                                c.g[V0][0] = c.g[V1][0] | c.g[V0][0];
                                c.sd(c.g[T1][0] as u32, c.g[V0][0]);
                                c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x28));
                                c.lwc1(3, (c.g[A1][0] as u32).wrapping_add(0x24));
                                c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x2c));
                                c.cvt_w_s(1, 0);
                                c.mfc1(2, 1);
                                c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x20));
                                c.cvt_w_s(0, 3);
                                c.mfc1(6, 0);
                                c.cvt_w_s(0, 2);
                                c.mfc1(3, 0);
                                c.cvt_w_s(0, 1);
                                c.mfc1(4, 0);
                                c.g[V0][0] = c.g[V0][0] << 8;
                                c.g[V0][0] = c.g[A2][0] | c.g[V0][0];
                                c.g[V1][0] = c.g[V1][0] << 16;
                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                c.g[A0][0] = c.g[A0][0] << 24;
                                c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                                c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                                c.sd((c.g[T1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                                c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x4));
                                c.lwc1(1, c.g[A1][0] as u32);
                                c.mul_s(0, 0, 4);
                                c.cvt_w_s(3, 2);
                                c.mfc1(2, 3);
                                c.cvt_w_s(2, 1);
                                c.mfc1(4, 2);
                                c.cvt_w_s(1, 0);
                                c.mfc1(3, 1);
                                c.g[V0][0] = c.g[V0][0] << 16;
                                c.g[V0][0] = c.g[A0][0] | c.g[V0][0];
                                c.g[V1][0] = c.g[V1][0] << 32;
                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                c.sd((c.g[T1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
                                c.mul_s(1, 1, 4);
                                c.mul_s(0, 0, 4);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
                                c.mfc1(4, 0);
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                c.g[V0][0] = c.g[A0][0] << 32;
                                c.g[V0][0] = c.g[V1][0] | c.g[V0][0];
                                c.sd((c.g[T1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x18));
                                c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x14));
                                c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x10));
                                c.mul_s(0, 0, 4);
                                c.cvt_w_s(3, 2);
                                c.mfc1(2, 3);
                                c.cvt_w_s(2, 1);
                                c.mfc1(4, 2);
                                c.cvt_w_s(1, 0);
                                c.mfc1(3, 1);
                                c.g[V0][0] = c.g[V0][0] << 16;
                                c.g[V0][0] = c.g[A0][0] | c.g[V0][0];
                                c.g[V1][0] = c.g[V1][0] << 32;
                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                c.sd((c.g[T1][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                                let t = c.g[T2][0] != 0u64;
                                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x28));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002dc938);
                                    }
                                    continue 'l_2dc938;
                                } else {
                                    break 'b_2dca80;
                                }
                            }
                        }
                    }
                    'b_2dca94: {
                        c.cycles += 2;
                        let t = c.g[T4][0] == 0u64;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
                        if t {
                            break 'b_2dca94;
                        } else {
                            c.cycles += 3;
                            c.sd(c.g[T1][0] as u32, 0u64);
                            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
                            break 'b_2dca94;
                        }
                    }
                    'b_2dcb3c: {
                        c.cycles += 9;
                        c.g[A1][0] = c.g[T1][0];
                        c.g[RA][0] = sx(0x002dcaa0);
                        c.g[A2][0] = 0x4;
                        call_ptr(c, 0x0038f668, 0x002dcaa0)?;
                        c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18f0));
                        c.g[T1][0] = c.g[V0][0];
                        c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                        c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
                        if t {
                            break 'b_2dcb3c;
                        } else {
                            c.cycles += 33;
                            c.g[A1][0] = 0x60000;
                            c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = c.g[A1][0] | 0x8000;
                            c.g[V1][0] = 0xffffffffffffffe0;
                            c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
                            c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
                            c.g[V1][0] = 0x30000000;
                            c.g[V0][0] = c.g[V0][0] & 0x1f;
                            c.g[A2][0] = c.g[A2][0] | c.g[V1][0];
                            c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                            c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                            c.g[V1][0] = c.lw(c.g[A3][0] as u32);
                            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc0));
                            c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                            c.sw(c.g[A3][0] as u32, c.g[V1][0]);
                            c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
                            c.ldr(3, c.g[T0][0] as u32);
                            c.ldl(5, (c.g[T0][0] as u32).wrapping_add(0xf));
                            c.ldr(5, (c.g[T0][0] as u32).wrapping_add(0x8));
                            c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
                            c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                            c.sdr(3, c.g[V0][0] as u32);
                            c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0xf));
                            c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[S5][0]);
                            c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                            c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                            break 'b_2dcb3c;
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = c.g[S0][0];
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
                    let t = (c.g[S1][0] as i64) > 0;
                    c.g[S2][0] = c.g[S4][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002dc838);
                        }
                        continue 'l_2dc838;
                    } else {
                        break 'b_2dcb50;
                    }
                }
            }
        }
        c.cycles += 13;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
        c.pc = t;
        return Ok(());
    });
}

/// `FXLoader` destructor (0x002dbee0, `cFXLoader__cFXLoader`): releases the 81
/// texture handles in the graphics manager's slot array (+0xf50, -1 = empty):
/// each live handle is removed through the texture manager (the sub-object at
/// +0x10d8: this-delta at +0x190, method at +0x194) and its slot reset to -1.
/// With bit 0 of `flags` set it then frees the object (`operator delete`).
pub fn fx_loader_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        c.g[S1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a90)).lw(c);
        frame.save(c, S0, 0x60);
        frame.save(c, S2, 0x40);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xf50));
        frame.save(c, S3, 0x30);
        c.g[S2][0] = 0x50;
        frame.save(c, S4, 0x20);
        c.g[S3][0] = u64::MAX;
        frame.save(c, S5, 0x10);
        c.g[S4][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S5][0] = c.g[A0][0];
        loop {
            c.cycles += 3;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
            if c.g[A1][0] == c.g[S3][0] {
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            } else {
                c.cycles += 7;
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10d8)).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x190)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x194)).lw(c);
                let method = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method, 0x002d_bf38)?;
                let empty = c.g[S3][0];
                Addr(c.g[S0][0] as u32).sw(c, empty);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            }
            c.cycles += 2;
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_bf18);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S4][0] & 1;
        let keep = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x60);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S5][0];
            call_ptr(c, 0x0031_7e50, 0x002d_bf5c)?;
            frame.restore(c, S0, 0x60);
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Texture files the constructor loads (`FILE_load(name, 0x100)`).
pub(crate) const FXLOADER_FILE_LOAD: u32 = 0x003e_18c8;

/// `SHAPE_locatez(file, entry)`: the effect shape of an entry in a file.
pub(crate) const FXLOADER_SHAPE_LOCATEZ: u32 = 0x003b_3900;

/// `MEM_free(file)`.
pub(crate) const FXLOADER_MEM_FREE: u32 = 0x0025_23a8;

/// `cFXLoader::cFXLoader(this, particles_only)` (0x2dbd10): loads the particle
/// texture file (and the effects file unless `particles_only`), binds each of
/// the 81 effect shapes to a texture id in the texture manager, frees the
/// files and fills the ten effect texture slots. Returns `this`.
pub fn fx_loader_construct(c: &mut Cpu) {
    guest(c, fxloader_body);
}

pub(crate) fn fxloader_body(c: &mut Cpu) -> Flow {
    c.cycles += 17;
    let frame = Frame::open(c, 0xc0);
    c.g[V0][0] = 0x1;
    frame.save(c, S4, 0x70);
    frame.save(c, S0, 0xb0);
    c.g[S4][0] = 0x2;
    frame.save(c, S1, 0xa0);
    if c.g[A1][0] != 0 {
        c.g[S4][0] = c.g[V0][0];
    }
    frame.save(c, S2, 0x90);
    c.g[S0][0] = 0;
    frame.save(c, S3, 0x80);
    frame.save(c, S5, 0x60);
    frame.save(c, S6, 0x50);
    frame.save(c, S7, 0x40);
    frame.save(c, FP, 0x30);
    frame.save_ra(c, 0x20);
    let this = c.g[A0][0];
    Addr(frame.at(c, 0x10)).sw(c, this);

    // Load the files: index 0 (particles) to sp+0, index 1 (effects) to sp+4.
    if c.g[S4][0] != 0 {
        c.cycles += 3;
        c.g[S3][0] = 0x1;
        c.g[S2][0] = 0x0048_0000;
        c.g[S1][0] = 0x0048_0000;
        loop {
            c.cycles += 2;
            // The arms share the `'b_2dbd94` tail (+1, next index) except the
            // plain increment of the last arm, which skips it.
            let mut shared_tail = true;
            if c.g[S0][0] == 0 {
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x7918));
                c.cycles += 4;
                c.g[A1][0] = 0x100;
                call_ptr(c, FXLOADER_FILE_LOAD, 0x002d_bd80)?;
                let handle = c.g[V0][0];
                Addr(frame.at(c, 0)).sw(c, handle);
            } else {
                c.cycles += 2;
                let second = c.g[S0][0] == c.g[S3][0];
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x7938));
                if second {
                    c.cycles += 3;
                    c.g[A1][0] = 0x100;
                    call_ptr(c, FXLOADER_FILE_LOAD, 0x002d_bd90)?;
                    let handle = c.g[V0][0];
                    Addr(frame.at(c, 4)).sw(c, handle);
                } else {
                    c.cycles += 2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    shared_tail = false;
                }
            }
            if shared_tail {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_bd60);
            }
        }
    }

    // Bind the 81 effect shapes to texture ids in the texture manager.
    c.cycles += 9;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a90)).lw(c);
    c.g[V0][0] = 0x0049_0000;
    c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6e50));
    c.g[S1][0] = 0;
    c.g[FP][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10b0));
    c.g[S6][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf50));
    c.g[S7][0] = 0xc;
    c.g[A1][0] = 0;
    loop {
        c.cycles += 4;
        c.g[S0][0] = 0;
        c.g[S2][0] = sx((c.g[S1][0] as u32) << 2);
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
        // Search the files for shape k (index i while i < n).
        loop {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
            let none_left = c.g[V0][0] == 0;
            c.mult(0, 2, 17, 23);
            if none_left {
                break;
            }
            c.cycles += 8;
            c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
            call_ptr(c, FXLOADER_SHAPE_LOCATEZ, 0x002d_bdf8)?;
            c.g[A1][0] = c.g[V0][0];
            if c.g[A1][0] == 0 {
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                if event_due(c) {
                    deliver_event_here(c, 0x002d_bdd8);
                }
                continue;
            }
            break;
        }
        // The shape's texture id (-1 when it was not found).
        c.cycles += 2;
        let not_found = c.g[A1][0] == 0;
        c.g[V0][0] = u64::MAX;
        if !not_found {
            c.cycles += 14;
            c.mult(0, 6, 17, 23);
            c.g[T3][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a90)).lw(c);
            c.g[V0][0] = 0x0049_0000;
            c.g[T0][0] = 0x1;
            c.g[V1][0] = Addr((c.g[T3][0] as u32).wrapping_add(0x10d8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6e48));
            c.g[T1][0] = u64::MAX;
            c.g[T2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x170)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x174)).lw(c);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[S5][0] as u32));
            c.g[A3][0] = Addr(c.g[T2][0] as u32).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x002d_be44)?;
        }
        c.cycles += 6;
        c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[S1][0] = c.g[S3][0];
        let tex_id = c.g[V0][0];
        Addr(c.g[V1][0] as u32).sw(c, tex_id);
        c.g[V0][0] = ((c.g[S1][0] as i64) < 81) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A1][0] = 0;
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x002d_bdc8);
            }
            continue;
        }
        break;
    }

    // Free the files.
    c.cycles += 2;
    let with_effects = c.g[S4][0] != 0;
    c.g[A0][0] = c.g[FP][0];
    if with_effects {
        c.cycles += 3;
        c.g[S1][0] = c.g[S4][0];
        c.g[S0][0] = c.g[SP][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        loop {
            c.cycles += 5;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            call_ptr(c, FXLOADER_MEM_FREE, 0x002d_be7c)?;
            if c.g[S1][0] != 0 {
                c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x002d_be70);
                }
                continue;
            }
            c.cycles += 1;
            c.g[A0][0] = c.g[FP][0];
            break;
        }
    }

    // Copy the ten effect texture slots (effect map 71..80) into dst[0..9].
    c.cycles += 2;
    c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x11c));
    c.g[A1][0] = 0x9;
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
        let slot = c.g[V0][0];
        Addr(c.g[A0][0] as u32).sw(c, slot);
        let more = (c.g[A1][0] as i64) >= 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x002d_be90);
            }
            continue;
        }
        break;
    }
    c.cycles += 13;
    c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
    frame.restore(c, S0, 0xb0);
    frame.restore(c, S1, 0xa0);
    frame.restore(c, S2, 0x90);
    frame.restore(c, S3, 0x80);
    frame.restore(c, S4, 0x70);
    frame.restore(c, S5, 0x60);
    frame.restore(c, S6, 0x50);
    frame.restore(c, S7, 0x40);
    frame.restore(c, FP, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

/// The comparator of the descending key sort (0x2dc168): compares the word at
/// +4 of `a0` and `a1`; `1` when `a < b`, `0` when equal, `-1` otherwise.
pub fn desc_key4_sort_compare(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x4)).lw(c);
    c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x4)).lw(c);
    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    let less = c.g[V0][0] != 0;
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
    if less {
        c.cycles += 2;
        c.g[V0][0] = 0x1;
    } else {
        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0u64;
        }
    }
    ret(c);
}
