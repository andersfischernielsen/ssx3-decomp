//! Native in-place conversion of an owned linear indexed texture to format 10.

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0036_6e90,
    tex_man_copy_to_temp_tex,
    "TexMan_CopyToTempTex",
)];

const SWIZZLE_T8: Callee = Callee {
    addr: 0x0039_0198,
    f: ssx3_ee::s12::TexMan_SwizzleT8,
};

fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// `void TexMan_CopyToTempTex(TexMan* manager, TexInfo* texture)` (0x366e90).
/// Swizzle every mip through temporary storage back into its original pixel
/// allocation, then change the TexInfo format from linear 9 to swizzled 10.
/// The pixel pointer, palette pointer and TEX0 are retained.
pub fn tex_man_copy_to_temp_tex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0xa0);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S6, 0x30);
        c.g[A1][0] = 0x004a_0000;
        frame.save(c, S7, 0x20);
        c.g[A1][0] = sx(r(c, A1).wrapping_add(0x4060));
        frame.save(c, FP, 0x10);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, S0, 0x90);
        c.g[S6][0] = 0;
        frame.save_ra(c, 0);
        c.g[A2][0] = c.lw(r(c, S7).wrapping_add(0x1f58));
        c.g[A0][0] = c.lw(r(c, S4).wrapping_add(0x10));
        c.g[A2][0] ^= 0x100;
        call_ptr(c, 0x0031_7e30, 0x0036_6ee4)?;

        c.g[S5][0] = c.g[V0][0];
        c.g[A0][0] = 1;
        c.g[V0][0] = c.ld(r(c, S4).wrapping_add(0x38));
        c.g[A1][0] = c.lw(r(c, S4).wrapping_add(0x24));
        c.g[V1][0] = c.g[V0][0] >> 30;
        c.g[FP][0] = c.lw(r(c, S4).wrapping_add(0x18));
        c.g[V0][0] >>= 26;
        c.g[V1][0] &= 0xf;
        c.g[V0][0] &= 0xf;
        c.g[V1][0] = ((c.g[V1][0] << 32) as i64 >> 32) as u64;
        c.g[V0][0] = ((c.g[V0][0] << 32) as i64 >> 32) as u64;
        c.g[S3][0] = sx(r(c, A0) << (r(c, V1) & 31));
        c.g[S2][0] = sx(r(c, A0) << (r(c, V0) & 31));
        let no_mip_levels = (c.g[A1][0] as i64) < 0;
        c.g[S1][0] = c.lw(r(c, S4).wrapping_add(0x1c));
        if !no_mip_levels {
            loop {
                c.cycles += 28;
                c.mult(0, V1, S2, S3);
                c.g[A2][0] = u64::MAX;
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S5][0];
                c.g[S6][0] = sx(r(c, S6).wrapping_add(1));
                c.mult(0, V1, V1, FP);
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.g[S0][0] = sx(r(c, V1).wrapping_add(7));
                if c.g[V0][0] != 0 {
                    c.g[S0][0] = c.g[V1][0];
                }
                c.g[A2][0] = 0x100;
                c.g[S0][0] = sx((r(c, S0) as i32 >> 3) as u32);
                c.g[V1][0] = 0xff;
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = c.g[A2][0];
                }
                c.g[A2][0] = c.g[S0][0];
                call_ptr(c, 0x003e_6574, 0x0036_6f68)?;

                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[T0][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S7][0];
                c.g[A3][0] = c.g[S5][0];
                c.g[S1][0] = sx(r(c, S1).wrapping_add(r(c, S0)));
                SWIZZLE_T8.call(c, 0x0036_6f84)?;

                c.g[S2][0] = sx((r(c, S2) as i32 >> 1) as u32);
                c.g[V0][0] = c.lw(r(c, S4).wrapping_add(0x24));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S6][0] as i64)) as u64;
                let more_mips = c.g[V0][0] == 0;
                c.g[S3][0] = sx((r(c, S3) as i32 >> 1) as u32);
                if !more_mips {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0036_6f28);
                }
            }
        }

        c.cycles += 2;
        let no_scratch = c.g[S5][0] == 0;
        c.g[V0][0] = 10;
        if !no_scratch {
            c.cycles += 3;
            c.g[A0][0] = c.g[S5][0];
            call_ptr(c, 0x0031_7e98, 0x0036_6fa8)?;
            c.g[V0][0] = 10;
        }

        c.cycles += 13;
        frame.restore(c, S0, 0x90);
        c.sw(r(c, S4).wrapping_add(0xc), c.g[V0][0]);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
