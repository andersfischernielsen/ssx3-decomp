//! Retail shared FX texture-bank loading and graphics texture-table binding.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x002d_bd10, fx_loader_ctor, "FXLoader_ctor")];

const TEXTURE_NAMES: u32 = 0x0048_91b0;
const PARTICLE_BANK: u32 = 0x0048_7918;
const EFFECTS_BANK: u32 = 0x0048_7938;

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// `FXLoader* FXLoader_ctor(FXLoader* self, bool particles_only)` (0x2dbd10).
/// Search `particle.ssh`, then (unless particles-only) `effects.ssh`, for the
/// 81 retail four-character names and bind each handle at GraphicsMan+0xf50.
/// Entry 23 is `exlm`, the anger exclamation used at GraphicsMan+0xfac.
pub fn fx_loader_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xc0);
        c.g[V0][0] = 1;
        frame.save(c, S4, 0x70);
        frame.save(c, S0, 0xb0);
        c.g[S4][0] = 2;
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
        let no_banks = c.g[S4][0] == 0;
        c.sw(r(c, SP).wrapping_add(0x10), c.g[A0][0]);
        if !no_banks {
            c.cycles += 3;
            c.g[S3][0] = 1;
            c.g[S2][0] = 0x0048_0000;
            c.g[S1][0] = 0x0048_0000;
            loop {
                c.cycles += 2;
                if c.g[S0][0] == 0 {
                    c.g[A0][0] = sx(r(c, S2).wrapping_add(PARTICLE_BANK & 0xffff));
                    c.cycles += 4;
                    c.g[A1][0] = 0x100;
                    call_ptr(c, 0x003e_18c8, 0x002d_bd80)?;
                    c.sw(r(c, SP), c.g[V0][0]);
                    c.cycles += 1;
                    c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                } else {
                    c.cycles += 2;
                    let effects = c.g[S0][0] == c.g[S3][0];
                    c.g[A0][0] = sx(r(c, S1).wrapping_add(EFFECTS_BANK & 0xffff));
                    if effects {
                        c.cycles += 3;
                        c.g[A1][0] = 0x100;
                        call_ptr(c, 0x003e_18c8, 0x002d_bd90)?;
                        c.sw(r(c, SP).wrapping_add(4), c.g[V0][0]);
                        c.cycles += 1;
                        c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                    } else {
                        c.cycles += 2;
                        c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                    }
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

        c.cycles += 9;
        c.g[V1][0] = c.lw(r(c, GP).wrapping_add(0x2a90));
        c.g[V0][0] = 0x0049_0000;
        c.g[S5][0] = sx(TEXTURE_NAMES);
        c.g[S1][0] = 0;
        c.g[FP][0] = sx(r(c, V1).wrapping_add(0x10b0));
        c.g[S6][0] = sx(r(c, V1).wrapping_add(0xf50));
        c.g[S7][0] = 12;
        c.g[A1][0] = 0;
        loop {
            c.cycles += 4;
            c.g[S0][0] = 0;
            c.g[S2][0] = sx(r(c, S1) << 2);
            c.g[S3][0] = sx(r(c, S1).wrapping_add(1));
            loop {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                let exhausted = c.g[V0][0] == 0;
                c.mult(0, V0, S1, S7);
                if exhausted {
                    break;
                }
                c.cycles += 8;
                c.g[V1][0] = sx(r(c, S0) << 2);
                c.g[A1][0] = sx(r(c, V0).wrapping_add(r(c, S5)));
                c.g[V0][0] = sx(r(c, SP).wrapping_add(r(c, V1)));
                c.g[A0][0] = c.lw(r(c, V0));
                call_ptr(c, 0x003b_3900, 0x002d_bdf8)?;
                c.g[A1][0] = c.g[V0][0];
                if c.g[A1][0] != 0 {
                    break;
                }
                c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                if event_due(c) {
                    deliver_event_here(c, 0x002d_bdd8);
                }
            }
            c.cycles += 2;
            let missing_shape = c.g[A1][0] == 0;
            c.g[V0][0] = u64::MAX;
            if !missing_shape {
                c.cycles += 14;
                c.mult(0, A2, S1, S7);
                c.g[T3][0] = c.lw(r(c, GP).wrapping_add(0x2a90));
                c.g[V0][0] = 0x0049_0000;
                c.g[T0][0] = 1;
                c.g[V1][0] = c.lw(r(c, T3).wrapping_add(0x10d8));
                c.g[V0][0] = sx(0x0048_91b8);
                c.g[T1][0] = u64::MAX;
                c.g[T2][0] = sx(r(c, A2).wrapping_add(r(c, V0)));
                c.g[A0][0] = c.lh(r(c, V1).wrapping_add(0x170));
                c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x174));
                c.g[A2][0] = sx(r(c, A2).wrapping_add(r(c, S5)));
                c.g[A3][0] = c.lw(r(c, T2));
                let load_shape = r(c, V0);
                c.g[A0][0] = sx(r(c, T3).wrapping_add(r(c, A0)));
                call_ptr(c, load_shape, 0x002d_be44)?;
            }
            c.cycles += 6;
            c.g[V1][0] = sx(r(c, S6).wrapping_add(r(c, S2)));
            c.g[S1][0] = c.g[S3][0];
            c.sw(r(c, V1), c.g[V0][0]);
            c.g[V0][0] = ((c.g[S1][0] as i64) < 81) as u64;
            let more_textures = c.g[V0][0] != 0;
            c.g[A1][0] = 0;
            if !more_textures {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_bdc8);
            }
        }

        c.cycles += 2;
        let no_banks = c.g[S4][0] == 0;
        c.g[A0][0] = c.g[FP][0];
        if !no_banks {
            c.cycles += 3;
            c.g[S1][0] = c.g[S4][0];
            c.g[S0][0] = c.g[SP][0];
            c.g[A0][0] = c.lw(r(c, S0));
            loop {
                c.cycles += 5;
                c.g[S1][0] = sx(r(c, S1).wrapping_sub(1));
                c.g[S0][0] = sx(r(c, S0).wrapping_add(4));
                call_ptr(c, 0x0025_23a8, 0x002d_be7c)?;
                if c.g[S1][0] == 0 {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[FP][0];
                    break;
                }
                c.g[A0][0] = c.lw(r(c, S0));
                if event_due(c) {
                    deliver_event_here(c, 0x002d_be70);
                }
            }
        }
        c.cycles += 2;
        c.g[V1][0] = sx(r(c, S6).wrapping_add(0x11c));
        c.g[A1][0] = 9;
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.lw(r(c, V1));
            c.g[A1][0] = sx(r(c, A1).wrapping_sub(1));
            c.g[V1][0] = sx(r(c, V1).wrapping_add(4));
            c.sw(r(c, A0), c.g[V0][0]);
            let more_aliases = (c.g[A1][0] as i64) >= 0;
            c.g[A0][0] = sx(r(c, A0).wrapping_add(4));
            if !more_aliases {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_be90);
            }
        }
        c.cycles += 13;
        c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x10));
        for (reg, offset) in [
            (S0, 0xb0),
            (S1, 0xa0),
            (S2, 0x90),
            (S3, 0x80),
            (S4, 0x70),
            (S5, 0x60),
            (S6, 0x50),
            (S7, 0x40),
            (FP, 0x30),
        ] {
            frame.restore(c, reg, offset);
        }
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}
