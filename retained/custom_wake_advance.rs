//! Native cBoardWakeFX_AdvanceRing0x2de058, retained uninstalled.
//!
//! No tests, builds, replays or game runs: the user disabled them.
//! Generated EE defines register, scratch-memory and cycle behavior.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 { c.g[reg][0] as u32 }
#[inline]
fn at(c: &Cpu, reg: usize, offset: u32) -> u32 { r(c, reg).wrapping_add(offset) }

/// `void cBoardWakeFX_AdvanceRing(cBoardWakeFX* self)`.
///
/// Advance the bounded wake history and obtain a colour from the rider's
/// interface at +0x6c0. Its virtual slot +0x38 returns the rider index (native
/// Rider_GetIndex0x140b80, rider+0x86c), used to select that rider's colour in
/// the 0xf0-byte entries at 0x4fa398. Double each ARGB float, clamp to [0,1],
/// retain the colour at self+0xa0, and write integer RGB*128 to every ring's
/// new vertex. Vertex alpha is initialized separately by BuildTrail.
pub fn board_wake_fx_advance_ring(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.lwc1(1, r(c, GP).wrapping_sub(0x3bdc));
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[AT][0] = 0x4080_0000;
        c.mtc1(AT, 2);
        c.lwc1(0, at(c, S0, 0x38));
        c.add_s(0, 0, 1);
        c.c_le_s(2, 0);
        let wrap_phase = c.fcc();
        c.swc1(0, at(c, S0, 0x38));
        if wrap_phase {
            c.cycles += 2;
            c.sub_s(0, 0, 2);
            c.swc1(0, at(c, S0, 0x38));
        }

        c.cycles += 5;
        c.g[V1][0] = c.lw(at(c, S0, 4));
        c.g[V0][0] = c.lw(at(c, S0, 0xb8));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let grow_count = c.g[V0][0] != 0;
        c.g[V0][0] = sx(r(c, V1).wrapping_add(1));
        if grow_count {
            c.cycles += 1;
            c.sw(at(c, S0, 4), c.g[V0][0]);
        }

        c.cycles += 4;
        c.g[V1][0] = c.lw(at(c, S0, 0xb8));
        c.g[V0][0] = c.lw(at(c, S0, 8));
        // The generated zero-cap branch rejoins immediately. Preserve the
        // signed DIV instruction's zero-divisor behavior rather than guard it.
        c.cycles += 43;
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
        c.g[A1][0] = c.lw(r(c, S0));
        c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
        c.div(0, V0, V1);
        c.g[A2][0] = sx(at(c, A1, 0x6c0));
        c.g[A0][0] = c.hi[0];
        c.sw(at(c, S0, 8), c.g[A0][0]);
        c.g[V1][0] = c.lw(at(c, A1, 0x6c0));
        c.g[A0][0] = c.lh(at(c, V1, 0x38));
        c.g[V0][0] = c.lw(at(c, V1, 0x3c));
        let target = r(c, V0);
        c.g[RA][0] = sx(0x002d_e0ec);
        c.g[A0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
        call_ptr(c, target, 0x002d_e0ec)?;

        // Surface colour table: ARGB at entry+0..c, stride 0xf0. Keep the
        // unaligned copy instructions and stack bytes of the native body.
        c.g[A0][0] = 0xf0;
        c.g[V1][0] = 0x500000;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = sx(r(c, V1).wrapping_sub(0x5c68));
        c.mtc1(ZERO, 4);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 5);
        c.mov_s(6, 4);
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
        c.ldl(V1, at(c, V0, 7));
        c.ldr(V1, r(c, V0));
        c.ldl(A0, at(c, V0, 0xf));
        c.ldr(A0, at(c, V0, 8));
        c.sdl(V1, frame.at(c, 7));
        c.sdr(V1, frame.at(c, 0));
        c.sdl(A0, frame.at(c, 0xf));
        c.sdr(A0, frame.at(c, 8));
        c.lwc1(0, frame.at(c, 4));
        c.lwc1(1, frame.at(c, 0));
        c.add_s(3, 0, 0);
        c.lwc1(2, frame.at(c, 8));
        c.lwc1(0, frame.at(c, 0xc));
        c.add_s(1, 1, 1);
        c.add_s(2, 2, 2);
        c.add_s(0, 0, 0);
        c.swc1(3, frame.at(c, 4));
        c.c_le_s(4, 3);
        c.swc1(1, frame.at(c, 0));
        c.swc1(2, frame.at(c, 8));
        let red_nonnegative = c.fcc();
        c.swc1(0, frame.at(c, 0xc));
        if red_nonnegative {
            c.cycles += 1;
            c.min_s(6, 3, 5);
        }

        c.cycles += 4;
        c.c_le_s(4, 2);
        c.swc1(6, frame.at(c, 4));
        let green_nonnegative = c.fcc();
        c.mov_s(3, 4);
        if green_nonnegative {
            c.cycles += 1;
            c.min_s(3, 2, 5);
        }

        c.cycles += 4;
        c.c_le_s(4, 0);
        c.swc1(3, frame.at(c, 8));
        let blue_nonnegative = c.fcc();
        c.mov_s(2, 4);
        if blue_nonnegative {
            c.cycles += 1;
            c.min_s(2, 0, 5);
        }

        c.cycles += 4;
        c.c_le_s(4, 1);
        c.swc1(2, frame.at(c, 0xc));
        let alpha_nonnegative = c.fcc();
        c.mov_s(0, 4);
        if alpha_nonnegative {
            c.cycles += 1;
            c.min_s(0, 1, 5);
        }

        // Preserve clamped ARGB, construct wrapping texture coordinate,
        // and convert the three surface RGB components to integer GS values.
        c.cycles += 70;
        c.swc1(0, frame.at(c, 0));
        c.g[A0][0] = 0x30;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 1);
        c.g[A3][0] = 0x30;
        c.ldl(V0, frame.at(c, 7));
        c.ldr(V0, frame.at(c, 0));
        c.ldl(V1, frame.at(c, 0xf));
        c.ldr(V1, frame.at(c, 8));
        c.sdl(V0, at(c, S0, 0xa7));
        c.sdr(V0, at(c, S0, 0xa0));
        c.sdl(V1, at(c, S0, 0xaf));
        c.sdr(V1, at(c, S0, 0xa8));
        c.g[T0][0] = 3;
        c.g[AT][0] = 0x3c80_0000;
        c.mtc1(AT, 2);
        c.g[A2][0] = sx(at(c, S0, 0x14));
        c.g[V0][0] = c.lw(at(c, S0, 8));
        c.lwc1(0, at(c, S0, 0x40));
        c.cvt_s_w(0, 0);
        c.g[V1][0] = c.lw(at(c, S0, 0x10));
        c.mult(0, V0, V0, A0);
        c.g[AT][0] = 0x4280_0000;
        c.mtc1(AT, 5);
        c.mul_s(0, 0, 1);
        c.g[AT][0] = 0x4200_0000;
        c.mtc1(AT, 4);
        c.g[AT][0] = 0x4300_0000;
        c.mtc1(AT, 3);
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
        c.swc1(0, r(c, V0));
        c.g[V1][0] = c.lw(at(c, S0, 8));
        c.g[V0][0] = c.lw(at(c, S0, 0x10));
        c.mult(0, V1, V1, A0);
        c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
        c.lwc1(1, r(c, V1));
        c.mul_s(2, 1, 2);
        c.cvt_w_s(0, 2);
        c.mfc1(V0, 0);
        c.mtc1(V0, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 5);
        c.add_s(0, 0, 4);
        c.sub_s(1, 1, 0);
        c.swc1(1, r(c, V1));
        c.g[V0][0] = c.lw(at(c, S0, 8));
        c.lwc1(0, at(c, S0, 0xa4));
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = c.lw(at(c, S0, 0x10));
        c.mul_s(0, 0, 3);
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
        c.cvt_w_s(1, 0);
        c.swc1(1, at(c, V0, 0x10));
        c.g[V1][0] = c.lw(at(c, S0, 8));
        c.lwc1(0, at(c, S0, 0xa8));
        c.mult(0, V1, V1, A0);
        c.g[V0][0] = c.lw(at(c, S0, 0x10));
        c.mul_s(0, 0, 3);
        c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
        c.cvt_w_s(1, 0);
        c.swc1(1, at(c, V1, 0x14));
        c.g[V0][0] = c.lw(at(c, S0, 8));
        c.lwc1(0, at(c, S0, 0xac));
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = c.lw(at(c, S0, 0x10));
        c.mul_s(0, 0, 3);
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
        c.cvt_w_s(1, 0);
        c.swc1(1, at(c, V0, 0x18));

        // Four side trails inherit the same texture coordinate and RGB.
        loop {
            c.cycles += 35;
            c.g[V0][0] = c.lw(at(c, S0, 8));
            c.g[T0][0] = sx(r(c, T0).wrapping_sub(1));
            c.g[V1][0] = c.lw(at(c, S0, 0x10));
            c.mult(0, V0, V0, A3);
            c.g[A0][0] = c.lw(r(c, A2));
            c.g[V1][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
            c.lwc1(0, r(c, V1));
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
            c.swc1(0, r(c, V0));
            c.g[V1][0] = c.lw(at(c, S0, 8));
            c.g[A0][0] = c.lw(at(c, S0, 0x10));
            c.mult(0, V1, V1, A3);
            c.g[A1][0] = c.lw(r(c, A2));
            c.g[A0][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
            c.g[V0][0] = c.lw(at(c, A0, 0x10));
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A1)));
            c.sw(at(c, V1, 0x10), c.g[V0][0]);
            c.g[A0][0] = c.lw(at(c, S0, 8));
            c.g[V1][0] = c.lw(at(c, S0, 0x10));
            c.mult(0, A0, A0, A3);
            c.g[A1][0] = c.lw(r(c, A2));
            c.g[V1][0] = sx(r(c, A0).wrapping_add(r(c, V1)));
            c.g[V0][0] = c.lw(at(c, V1, 0x14));
            c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, A1)));
            c.sw(at(c, A0, 0x14), c.g[V0][0]);
            c.g[V1][0] = c.lw(at(c, S0, 8));
            c.g[A0][0] = c.lw(at(c, S0, 0x10));
            c.mult(0, V1, V1, A3);
            c.g[A1][0] = c.lw(r(c, A2));
            c.g[A2][0] = sx(at(c, A2, 4));
            c.g[A0][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
            c.g[V0][0] = c.lw(at(c, A0, 0x18));
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A1)));
            let more = (c.g[T0][0] as i64) >= 0;
            c.sw(at(c, V1, 0x18), c.g[V0][0]);
            if !more { break; }
            if event_due(c) { deliver_event_here(c, 0x002d_e2c0); }
        }

        c.cycles += 7;
        c.g[V0][0] = c.lw(at(c, S0, 0x40));
        frame.restore_ra(c, 0x10);
        c.g[V0][0] = sx(r(c, V0).wrapping_add(1));
        c.sw(at(c, S0, 0x40), c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    });
}
