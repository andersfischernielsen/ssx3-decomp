//! Staging module oc1058 (wave wave265): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1058`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Direct calls of the fog CLUT builder.
mod oc1058_callee {
    use super::{ee, Callee};

    pub const SPR_STREAM_BEGIN_BLOCK: Callee = Callee {
        addr: 0x0038_f460,
        f: ee::s12::SprStream_BeginBlock,
    };
    pub const SPR_STREAM_END_BLOCK: Callee = Callee {
        addr: 0x0038_f598,
        f: ee::s12::SprStream_EndBlock,
    };
}

/// The low 32 bits of general register `r`.
#[inline(always)]
fn fog_w(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// The address `off(sp)`.
#[inline(always)]
fn fog_sp(c: &Cpu, off: u32) -> u32 {
    (c.g[SP][0] as u32).wrapping_add(off)
}

/// Where the code continues after a fog mode's loop (the generated body's
/// join blocks `0x36a9d8`, `0x36a9dc` and `0x36a9e0`).
#[derive(Clone, Copy)]
enum FogTail {
    /// `cycles += 1; a2 = s2`, then `Count`.
    Clamp,
    /// `cycles += 1; v0 = a2 < 256`, then the opaque tail.
    Count,
    /// `v0 = a2 < 256` is already computed: straight to the opaque tail.
    Opaque,
}

/// Epilogue of the CLUT builder: restore s1..s6, `ra`, `f20`, pop the frame.
fn fog_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 10;
    frame.restore(c, S1, 0xa0);
    frame.restore(c, S2, 0x90);
    frame.restore(c, S3, 0x80);
    frame.restore(c, S4, 0x70);
    frame.restore(c, S5, 0x60);
    frame.restore(c, S6, 0x50);
    frame.restore_ra(c, 0x40);
    frame.restore_fpr(c, 20, 0xc0);
    frame.close(c);
}

/// Fog mode 0: linear ramp, `alpha = clamp(((max(i, n1) - n1) << 7) / cnt, 0,
/// 0x80)` for entries `i < n0`, stored at the swizzled index in `t0[...]`
/// (bits 3 and 4 of `i` swapped) as `colour | alpha << 24`.
fn fog_linear_loop(c: &mut Cpu) -> FogTail {
    c.cycles += 2;
    let empty = (c.g[S2][0] as i64) <= 0;
    c.g[A2][0] = 0u64;
    if empty {
        return FogTail::Clamp;
    }
    c.cycles += 3;
    c.g[T2][0] = 0xffffffffffffffe7;
    c.g[T1][0] = 0u64;
    c.g[A3][0] = 0x80;
    loop {
        c.cycles += 15;
        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[A2][0] as i64)) as u64;
        c.g[V1][0] = c.g[A2][0];
        if c.g[V0][0] == 0 {
            c.g[V1][0] = c.g[S1][0];
        }
        c.g[A0][0] = c.g[A2][0] & 0x8;
        c.g[V1][0] = sx(fog_w(c, V1).wrapping_sub(fog_w(c, S1)));
        c.g[A0][0] = sx(fog_w(c, A0) << 1);
        c.g[V1][0] = sx(fog_w(c, V1) << 7);
        c.g[A1][0] = c.g[A2][0] & c.g[T2][0];
        c.div(0, V1, S3);
        c.g[V0][0] = c.g[A2][0] & 0x10;
        c.g[A1][0] |= c.g[A0][0];
        c.g[V0][0] = sx((fog_w(c, V0) as i32 >> 1) as u32);
        c.g[A1][0] |= c.g[V0][0];
        c.cycles += 4;
        c.g[V1][0] = c.lo[0];
        c.g[A0][0] = c.g[V1][0];
        let negative = (c.g[A0][0] as i64) < 0;
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A0][0] as i64)) as u64;
        if negative {
            c.cycles += 1;
            c.g[V1][0] = 0u64;
        } else {
            c.cycles += 3;
            c.g[V1][0] = c.g[A3][0];
            if c.g[V0][0] == 0 {
                c.g[V1][0] = c.g[A0][0];
            }
        }
        c.cycles += 8;
        c.g[A0][0] = sx(fog_w(c, A1) << 2);
        c.g[V1][0] = sx(fog_w(c, V1) << 24);
        c.g[A0][0] = sx(fog_w(c, A0).wrapping_add(fog_w(c, T0)));
        c.g[V1][0] = c.g[S5][0] | c.g[V1][0];
        c.g[A2][0] = sx(fog_w(c, A2).wrapping_add(0x1));
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[S2][0] as i64)) as u64;
        let more = c.g[V0][0] != 0u64;
        c.sw(fog_w(c, A0), c.g[V1][0]);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0036_a738);
            }
            continue;
        }
        c.cycles += 2;
        c.g[A2][0] = c.g[S2][0];
        return FogTail::Count;
    }
}

/// Fog modes 1 (exponential, `alpha = exp(-d)`) and 2 (exponential squared,
/// `exp(-d * d)`), `d = density * scale * (n0 - max(i, n1) + 1) /
/// cnt`: the loop evaluates `exp` with a polynomial whose coefficients sit
/// at `gp - 0x289c` (mode 1) or `gp - 0x2884` (mode 2), clamps to `0..1` and
/// scales by 128.
fn fog_exp_loop(c: &mut Cpu, squared: bool) -> FogTail {
    c.cycles += 2;
    let empty = (c.g[S2][0] as i64) <= 0;
    c.g[A1][0] = 0u64;
    if empty {
        return FogTail::Clamp;
    }
    c.cycles += 15;
    let base = fog_w(c, GP).wrapping_sub(if squared { 0x2884 } else { 0x289c });
    c.lwc1(13, base);
    c.g[A2][0] = 0xffffffffffffffe7;
    c.lwc1(12, base.wrapping_add(0x4));
    c.lwc1(11, base.wrapping_add(0x8));
    c.lwc1(10, base.wrapping_add(0xc));
    c.lwc1(9, base.wrapping_add(0x10));
    c.lwc1(8, base.wrapping_add(0x14));
    c.g[AT][0] = 0x3f000000;
    c.mtc1(1, 7);
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 4);
    c.mtc1(0, 5);
    c.g[AT][0] = 0x43000000;
    c.mtc1(1, 6);
    let head = if squared { 0x0036_a910 } else { 0x0036_a800 };
    loop {
        c.cycles += if squared { 38 } else { 37 };
        c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.g[V0][0] = c.g[A1][0];
        if c.g[V1][0] == 0 {
            c.g[V0][0] = c.g[S1][0];
        }
        c.lwc1(2, fog_w(c, S4).wrapping_add(0xc));
        c.g[V0][0] = sx(fog_w(c, S2).wrapping_sub(fog_w(c, V0)));
        c.g[V1][0] = c.g[A1][0] & 0x8;
        c.g[V0][0] = sx(fog_w(c, V0).wrapping_add(0x1));
        c.mul_s(2, 2, 13);
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.g[V1][0] = sx(fog_w(c, V1) << 1);
        c.g[A0][0] = c.g[A1][0] & c.g[A2][0];
        c.g[V0][0] = c.g[A1][0] & 0x10;
        c.g[A0][0] |= c.g[V1][0];
        c.g[V0][0] = sx((fog_w(c, V0) as i32 >> 1) as u32);
        c.mul_s(2, 2, 0);
        c.g[A0][0] |= c.g[V0][0];
        c.mul_s(2, 2, 20);
        if squared {
            c.mul_s(2, 2, 2);
        }
        c.neg_s(2, 2);
        c.mul_s(0, 2, 12);
        c.add_s(0, 0, 11);
        c.mul_s(0, 0, 2);
        c.add_s(1, 0, 10);
        c.mul_s(0, 1, 2);
        c.add_s(0, 0, 9);
        c.mul_s(0, 0, 2);
        c.add_s(1, 0, 8);
        c.mul_s(0, 1, 2);
        c.add_s(0, 0, 7);
        c.mul_s(0, 0, 2);
        c.add_s(1, 0, 4);
        c.mul_s(0, 1, 2);
        c.add_s(0, 0, 4);
        c.c_le_s(5, 0);
        let negative = !c.fcc();
        c.mov_s(3, 5);
        if !negative {
            c.cycles += 1;
            c.min_s(3, 0, 4);
        }
        c.cycles += 11;
        c.mul_s(0, 3, 6);
        c.g[V1][0] = sx(fog_w(c, A0) << 2);
        c.g[V1][0] = sx(fog_w(c, V1).wrapping_add(fog_w(c, T0)));
        c.g[A1][0] = sx(fog_w(c, A1).wrapping_add(0x1));
        c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[S2][0] as i64)) as u64;
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);
        c.g[V0][0] = sx(fog_w(c, V0) << 24);
        c.g[V0][0] = c.g[S5][0] | c.g[V0][0];
        let more = c.g[A0][0] != 0u64;
        c.sw(fog_w(c, V1), c.g[V0][0]);
        if more {
            if event_due(c) {
                deliver_event_here(c, head);
            }
            continue;
        }
        if squared {
            return FogTail::Clamp;
        }
        c.cycles += 2;
        c.g[A2][0] = c.g[S2][0];
        return FogTail::Count;
    }
}

/// `void PS2GraphicsMan_BuildDepthFogClut(PS2GraphicsMan *man, void *mat,
/// int idx)` (0x36a428): build the 256-entry depth fog CLUT of fog `idx`.
///
/// Transforms the fog's near and far depth (`man+idx*0x20+0x6c64`: nearZ,
/// farZ, mode, density, colour) through the view matrix
/// `man+idx*0x40+0x5930`, divides by w, and turns them into CLUT indexes
/// `n0` (near, 0..255) and `n1` (far). When the cache at `gp+0x10b8..`
/// already holds the same indexes, colour, mode and density, it returns. Else
/// it stores the new key and fills the 0x400-byte block that
/// `SprStream_BeginBlock` hands out: entries below `n0` get an alpha by the
/// fog mode (0 linear, 1 exponential, 2 exponential squared), the rest are
/// opaque (0x80), entries swizzled with `i & ~0x18 | (i & 8) << 1 | (i & 0x10)
/// >> 1`; `SprStream_EndBlock` then starts the upload.
pub fn ps2_graphics_man_build_depth_fog_clut(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 77;
        c.g[A3][0] = c.g[A2][0];
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S4, 0x70);
        frame.save(c, S0, 0xb0);
        frame.save(c, S1, 0xa0);
        frame.save(c, S2, 0x90);
        frame.save(c, S3, 0x80);
        frame.save(c, S5, 0x60);
        frame.save(c, S6, 0x50);
        frame.save_ra(c, 0x40);
        frame.save_fpr(c, 20, 0xc0);

        // s4 = &man->fog[idx]; v1 = a2 = &man->mats[idx].
        c.g[V0][0] = sx(fog_w(c, A3) << 5);
        c.g[V0][0] = sx(fog_w(c, V0).wrapping_add(0x6c64));
        c.g[S4][0] = sx(fog_w(c, A0).wrapping_add(fog_w(c, V0)));
        c.g[A2][0] = sx(fog_w(c, A3) << 6);
        c.g[V1][0] = sx(fog_w(c, A2).wrapping_add(fog_w(c, A0)));
        c.g[V1][0] = sx(fog_w(c, V1).wrapping_add(0x5930));
        c.g[A2][0] = sx(fog_w(c, A2).wrapping_add(fog_w(c, A0)));
        c.g[A2][0] = sx(fog_w(c, A2).wrapping_add(0x5930));
        c.g[T3][0] = c.g[A1][0];
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 2);
        c.g[A0][0] = 0xff;

        // a = mats[idx] * (0, 0, nearZ, 1).
        c.lwc1(0, fog_w(c, S4));
        c.swc1(2, fog_sp(c, 0x1c));
        c.swc1(0, fog_sp(c, 0x18));
        c.sw(fog_sp(c, 0x10), 0u64);
        c.sw(fog_sp(c, 0x14), 0u64);
        c.lqc2(8, fog_sp(c, 0x10));
        c.lqc2(4, fog_w(c, V1));
        c.lqc2(5, fog_w(c, V1).wrapping_add(0x10));
        c.lqc2(6, fog_w(c, V1).wrapping_add(0x20));
        c.lqc2(7, fog_w(c, V1).wrapping_add(0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, fog_sp(c, 0x20));
        // b = mats[idx] * (0, 0, farZ, 1).
        c.lwc1(0, fog_w(c, S4).wrapping_add(0x4));
        c.lq(2, fog_sp(c, 0x20));
        c.swc1(0, fog_sp(c, 0x28));
        c.sq(2, fog_sp(c, 0));
        c.swc1(2, fog_sp(c, 0x2c));
        c.sw(fog_sp(c, 0x20), 0u64);
        c.sw(fog_sp(c, 0x24), 0u64);
        c.lqc2(8, fog_sp(c, 0x20));
        c.lqc2(4, fog_w(c, A2));
        c.lqc2(5, fog_w(c, A2).wrapping_add(0x10));
        c.lqc2(6, fog_w(c, A2).wrapping_add(0x20));
        c.lqc2(7, fog_w(c, A2).wrapping_add(0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, fog_sp(c, 0x30));
        // a *= 1 / a.w ; b *= 1 / b.w.
        c.lq(2, fog_sp(c, 0x30));
        c.lwc1(0, fog_sp(c, 0xc));
        c.sq(2, fog_sp(c, 0x10));
        c.div_s(0, 2, 0);
        c.mfc1(2, 0);
        c.lqc2(4, fog_sp(c, 0));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, fog_sp(c, 0));
        c.lwc1(1, fog_sp(c, 0x1c));
        c.div_s(2, 2, 1);
        c.mfc1(2, 2);
        c.lqc2(4, fog_sp(c, 0x10));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, fog_sp(c, 0x10));

        // n0 = clamp((int)a.z >> 8, 0, 255) in s2.
        c.lwc1(0, fog_sp(c, 0x8));
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);
        c.g[V0][0] = sx(((fog_w(c, V0)) as i32 >> 8) as u32);
        let below = (c.g[V0][0] as i64) < 0;
        c.g[S2][0] = c.g[V0][0];
        if below {
            c.cycles += 1;
            c.g[S2][0] = 0;
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[S2][0] = c.g[A0][0];
            }
        }
        // n1 = clamp((int)b.z >> 8, 0, 255) in s1.
        c.cycles += 6;
        c.lwc1(0, fog_sp(c, 0x18));
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);
        c.g[V0][0] = sx(((fog_w(c, V0)) as i32 >> 8) as u32);
        let below = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = 0xff;
        if below {
            c.cycles += 1;
            c.g[S1][0] = 0;
        } else {
            c.cycles += 4;
            c.g[S1][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[S1][0] = c.g[V1][0];
            }
        }

        // cnt = max(n0 - n1 + 1, 1) in s3; colour key in s5; f20 = 1 / cnt.
        c.cycles += 41;
        c.g[AT][0] = 0x437f0000;
        c.mtc1(1, 0);
        c.g[V0][0] = sx(fog_w(c, S2).wrapping_sub(fog_w(c, S1)));
        c.lwc1(1, fog_w(c, S4).wrapping_add(0x18));
        c.g[S3][0] = sx(fog_w(c, V0).wrapping_add(0x1));
        c.lwc1(2, fog_w(c, S4).wrapping_add(0x14));
        c.g[V0][0] = (0i64 < (c.g[S3][0] as i64)) as u64;
        c.lwc1(3, fog_w(c, S4).wrapping_add(0x1c));
        c.mul_s(1, 1, 0);
        c.mul_s(2, 2, 0);
        c.g[A0][0] = 0x1;
        c.mul_s(3, 3, 0);
        if c.g[V0][0] == 0 {
            c.g[S3][0] = c.g[A0][0];
        }
        c.cvt_w_s(0, 1);
        c.mfc1(3, 0);
        c.g[A0][0] = sx(fog_w(c, A3) << 2);
        c.cvt_w_s(0, 2);
        c.mfc1(5, 0);
        c.g[T0][0] = c.g[A0][0];
        c.cvt_w_s(0, 3);
        c.mfc1(2, 0);
        c.g[A3][0] = sx(fog_w(c, GP).wrapping_add(0x2948));
        c.g[A2][0] = sx(fog_w(c, GP).wrapping_add(0x10b8));
        c.g[V1][0] = sx(fog_w(c, V1) << 8);
        c.mtc1(19, 1);
        c.cvt_s_w(1, 1);
        c.g[A1][0] |= c.g[V1][0];
        c.g[A0][0] = sx(fog_w(c, A0).wrapping_add(fog_w(c, A2)));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.g[V0][0] = sx(fog_w(c, V0) << 16);
        c.g[T2][0] = c.g[A2][0];
        c.g[S5][0] = c.g[A1][0] | c.g[V0][0];
        c.g[V1][0] = c.lw(fog_w(c, A0));
        c.div_s(20, 0, 1);
        c.g[A1][0] = sx(fog_w(c, GP).wrapping_add(0x2938));
        c.g[A2][0] = sx(fog_w(c, GP).wrapping_add(0x2940));
        c.g[T1][0] = sx(fog_w(c, GP).wrapping_add(0x2950));
        c.g[T4][0] = sx(fog_w(c, GP).wrapping_add(0x2958));
        let cold = c.g[V1][0] == 0u64;
        c.g[V0][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, A1)));

        // Cache check: valid, n0, n1, colour, mode, density all equal -> done.
        // `refill` is false on a hit; `slot_cycle` charges the extra cycle of
        // the paths that run through the join block 0x36a698.
        let mut hit = false;
        let mut slot_cycle = false;
        'cache: {
            if cold {
                slot_cycle = true;
                break 'cache;
            }
            c.cycles += 3;
            c.g[V1][0] = c.lw(fog_w(c, V0));
            let miss = c.g[S2][0] != c.g[V1][0];
            c.g[V1][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, T2)));
            if miss {
                break 'cache;
            }
            c.cycles += 4;
            c.g[V0][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, A2)));
            c.g[V1][0] = c.lw(fog_w(c, V0));
            let miss = c.g[S1][0] != c.g[V1][0];
            c.g[V1][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, T2)));
            if miss {
                break 'cache;
            }
            c.cycles += 4;
            c.g[V0][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, A3)));
            c.g[V1][0] = c.lw(fog_w(c, V0));
            let miss = c.g[S5][0] != c.g[V1][0];
            c.g[V1][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, T2)));
            if miss {
                break 'cache;
            }
            c.cycles += 5;
            c.g[V0][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, T1)));
            c.g[A0][0] = c.lw(fog_w(c, S4).wrapping_add(0x8));
            c.g[V1][0] = c.lw(fog_w(c, V0));
            let miss = c.g[A0][0] != c.g[V1][0];
            c.g[V1][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, T2)));
            if miss {
                break 'cache;
            }
            c.cycles += 7;
            c.g[V0][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, T4)));
            c.lwc1(1, fog_w(c, S4).wrapping_add(0xc));
            c.lwc1(0, fog_w(c, V0));
            c.c_eq_s(1, 0);
            let same = c.fcc();
            frame.restore(c, S0, 0xb0);
            if same {
                hit = true;
            } else {
                slot_cycle = true;
            }
        }
        if hit {
            fog_epilogue(c, frame);
            return Ok(());
        }
        if slot_cycle {
            c.cycles += 1;
            c.g[V1][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, T2)));
        }

        // Miss: store the key, take the block.
        c.cycles += 23;
        c.g[S0][0] = 0x1;
        c.sw(fog_w(c, V1), c.g[S0][0]);
        c.g[V0][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, A1)));
        c.sw(fog_w(c, V0), c.g[S2][0]);
        c.g[V1][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, A2)));
        c.sw(fog_w(c, V1), c.g[S1][0]);
        c.g[V0][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, A3)));
        c.sw(fog_w(c, V0), c.g[S5][0]);
        c.g[V1][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, T1)));
        c.g[S6][0] = c.lw(fog_w(c, GP).wrapping_add(0x1384));
        c.g[T0][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, T4)));
        c.g[V0][0] = c.lw(fog_w(c, S4).wrapping_add(0x8));
        c.g[A1][0] = c.g[T3][0];
        c.g[A0][0] = c.g[S6][0];
        c.g[A2][0] = 0x400;
        c.sw(fog_w(c, V1), c.g[V0][0]);
        c.g[A3][0] = 0u64;
        c.lwc1(0, fog_w(c, S4).wrapping_add(0xc));
        c.swc1(0, fog_w(c, T0));
        oc1058_callee::SPR_STREAM_BEGIN_BLOCK.call(c, 0x0036_a6ec)?;
        c.g[V1][0] = c.lw(fog_w(c, S4).wrapping_add(0x8));
        let mode_exp = c.g[V1][0] == c.g[S0][0];
        c.g[T0][0] = c.g[V0][0];

        // Mode dispatch: 1 exponential, 2 exponential squared, 0 linear;
        // anything else writes only the opaque tail.
        let join = if mode_exp {
            fog_exp_loop(c, false)
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
            let ge2 = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x2;
            if !ge2 {
                c.cycles += 2;
                let linear = c.g[V1][0] == 0u64;
                c.g[A2][0] = c.g[S2][0];
                if linear {
                    fog_linear_loop(c)
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 256i64) as u64;
                    FogTail::Opaque
                }
            } else {
                c.cycles += 2;
                let squared = c.g[V1][0] == c.g[V0][0];
                c.g[A2][0] = c.g[S2][0];
                if squared {
                    fog_exp_loop(c, true)
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 256i64) as u64;
                    FogTail::Opaque
                }
            }
        };
        if matches!(join, FogTail::Clamp) {
            c.cycles += 1;
            c.g[A2][0] = c.g[S2][0];
        }
        if !matches!(join, FogTail::Opaque) {
            c.cycles += 1;
            c.g[V0][0] = ((c.g[A2][0] as i64) < 256i64) as u64;
        }

        // Opaque tail: entries n0..255 get alpha 0x80.
        c.cycles += 2;
        let none = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0xffffffff80000000;
        if !none {
            c.cycles += 2;
            c.g[A3][0] = 0xffffffffffffffe7;
            c.g[A1][0] = c.g[S5][0] | c.g[V0][0];
            loop {
                c.cycles += 13;
                c.g[A0][0] = c.g[A2][0] & 0x8;
                c.g[V0][0] = c.g[A2][0] & c.g[A3][0];
                c.g[V1][0] = c.g[A2][0] & 0x10;
                c.g[A0][0] = sx(fog_w(c, A0) << 1);
                c.g[V1][0] = sx((fog_w(c, V1) as i32 >> 1) as u32);
                c.g[V0][0] |= c.g[A0][0];
                c.g[V0][0] |= c.g[V1][0];
                c.g[A2][0] = sx(fog_w(c, A2).wrapping_add(0x1));
                c.g[V0][0] = sx(fog_w(c, V0) << 2);
                c.g[V1][0] = ((c.g[A2][0] as i64) < 256i64) as u64;
                c.g[V0][0] = sx(fog_w(c, V0).wrapping_add(fog_w(c, T0)));
                let more = c.g[V1][0] != 0u64;
                c.sw(fog_w(c, V0), c.g[A1][0]);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0036_a9f0);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = 0x400;
        c.g[A2][0] = 0x4;
        oc1058_callee::SPR_STREAM_END_BLOCK.call(c, 0x0036_aa34)?;
        frame.restore(c, S0, 0xb0);
        fog_epilogue(c, frame);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0036_a428,
    ps2_graphics_man_build_depth_fog_clut,
    "PS2GraphicsMan_BuildDepthFogClut",
)];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{
        check_reaching, fake, fall_back, hook, LOG_RESULTS, LOG_UNWIND_AT,
    };
    use crate::difftest::{fresh, run_generated, scramble, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes (they only act when it holds
    /// `OC1058_MAGIC` in the machine's RAM).
    const OC1058_CTRL: u32 = DATA + 0xf_0000;
    const OC1058_MAGIC: u32 = 0x1058_f06c;
    /// The block `SprStream_BeginBlock` hands out in the fixture.
    const OC1058_BLOCK: u32 = DATA + 0x1_0000;
    static OC1058_PREV_BEGIN: AtomicUsize = AtomicUsize::new(0);
    static OC1058_PREV_END: AtomicUsize = AtomicUsize::new(0);

    fn oc1058_fake_begin(c: &mut Cpu) {
        if c.bus.read32(OC1058_CTRL) != OC1058_MAGIC {
            return fall_back(c, &OC1058_PREV_BEGIN, ee::s12::f_0038f460__generated);
        }
        fake(c, 0x0038_f460, c.g[RA][0] as u32);
    }

    fn oc1058_fake_end(c: &mut Cpu) {
        if c.bus.read32(OC1058_CTRL) != OC1058_MAGIC {
            return fall_back(c, &OC1058_PREV_END, ee::s12::f_0038f598__generated);
        }
        fake(c, 0x0038_f598, c.g[RA][0] as u32);
    }

    fn oc1058_install() {
        hook(
            std::ptr::addr_of_mut!(ee::s12::OVERRIDE_0038F460),
            oc1058_fake_begin,
            &OC1058_PREV_BEGIN,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s12::OVERRIDE_0038F598),
            oc1058_fake_end,
            &OC1058_PREV_END,
        );
    }

    fn oc1058_idx(seed: u32) -> u32 {
        (seed / 3) % 4
    }

    /// The fixture with an empty cache. Seed bits: `seed % 9` the cache
    /// variant (see `oc1058_setup`), `seed / 9 % 6` the fog mode, `seed / 54 % 8`
    /// the depth regime (clamps, `cnt <= 0`, `n0 == 0`), `seed / 5 % 4` the w.
    fn oc1058_build(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x1058_0000_dead ^ (seed as u64) << 7);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(OC1058_CTRL, OC1058_MAGIC);
        let unwind = if seed % 7 == 0 {
            0
        } else if seed % 11 == 0 {
            1
        } else {
            u32::MAX
        };
        c.bus.write32(LOG_UNWIND_AT, unwind);
        c.bus.write32(LOG_RESULTS, OC1058_BLOCK);

        let idx = oc1058_idx(seed);
        let man = DATA;
        c.g[A0][0] = man as u64;
        c.g[A1][0] = (DATA + 0x2_0000) as u64;
        c.g[A2][0] = idx as u64;

        let w0 = [1.0f32, 2.0, 0.5, 4.0][((seed / 5) % 4) as usize];
        let (n0, n1): (i32, i32) = match (seed / 54) % 8 {
            0 => (96, 16),
            1 => (-5, -9),
            2 => (300, 40),
            3 => (80, -4),
            4 => (48, 300),
            5 => (0, 0),
            6 => (
                r.next_u32() as i32 % 400 - 50,
                r.next_u32() as i32 % 400 - 50,
            ),
            _ => (10, 10),
        };
        let z = |n: i32| (((n * 256 + 100) as f32) * w0).to_bits();
        let mode = [0u32, 1, 2, 3, 0xffff_ffff, 1][((seed / 9) % 6) as usize];
        let density = [0.0f32, 0.5, 1.0, 3.5][(seed % 4) as usize];

        // The fog record at man + idx * 0x20 + 0x6c64.
        let fog = man + idx * 0x20 + 0x6c64;
        c.bus.write32(fog, z(n0));
        c.bus.write32(fog + 0x4, z(n1));
        c.bus.write32(fog + 0x8, mode);
        c.bus.write32(fog + 0xc, density.to_bits());
        for (i, off) in [0x14u32, 0x18, 0x1c].into_iter().enumerate() {
            let v = ((r.next_u32() % 256) as f32 / 255.0).min(1.0) * if i == 0 { 1.0 } else { 0.9 };
            c.bus.write32(fog + off, v.to_bits());
        }
        // The matrix at man + idx * 0x40 + 0x5930: z' = z, w' = w0.
        let mat = man + idx * 0x40 + 0x5930;
        let rows: [[f32; 4]; 4] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, w0],
        ];
        for (ri, row) in rows.iter().enumerate() {
            for (ci, v) in row.iter().enumerate() {
                c.bus
                    .write32(mat + 16 * ri as u32 + 4 * ci as u32, v.to_bits());
            }
        }
        // exp polynomial coefficients (modes 1 and 2) in the data segment.
        for i in 0..12u32 {
            let v = 0.01f32 + 0.07 * (i % 6) as f32;
            c.bus.write32(
                GAME_GP.wrapping_sub(0x289c).wrapping_add(4 * i),
                v.to_bits(),
            );
        }
        c.bus.write32(GAME_GP + 0x1384, DATA + 0x3_0000);
        // Empty cache.
        c.bus.write32(GAME_GP + 0x10b8 + 4 * idx, 0);
    }

    /// Cache variants by `seed % 9`: 0 empty; 1..=5 filled from a real run with
    /// one key mismatching (n0, n1, colour, mode, density); 6 all equal (hit);
    /// 7 and 8 valid but random.
    fn oc1058_setup(c: &mut Cpu, seed: u32) {
        oc1058_build(c, seed);
        let idx = oc1058_idx(seed);
        let words = [0x10b8u32, 0x2938, 0x2940, 0x2948, 0x2950, 0x2958];
        let variant = seed % 9;
        if (1..=6).contains(&variant) {
            let mut t = fresh();
            oc1058_build(&mut t, seed);
            t.bus.write32(LOG_UNWIND_AT, u32::MAX);
            run_generated(&mut t, ssx3_ee::s12::f_0036a428__generated);
            for (k, off) in words.into_iter().enumerate() {
                let a = GAME_GP + off + 4 * idx;
                let mut v = t.bus.read32(a);
                if k >= 1 && k as u32 == variant {
                    v ^= 0x100;
                }
                c.bus.write32(a, v);
            }
        } else if variant >= 7 {
            let mut r = Rng(0x7777_0000 ^ seed as u64);
            for off in words {
                c.bus.write32(GAME_GP + off + 4 * idx, r.next_u32());
            }
            c.bus.write32(GAME_GP + 0x10b8 + 4 * idx, 1);
        }
    }

    /// Every fog mode (linear, exponential, exponential squared and an
    /// unknown one), the depth clamps, `cnt <= 0`, `n0 == 0`, every cache key
    /// mismatch and the hit, and an unwind in either callee. The `v0 == 0`
    /// skip of the opaque tail needs `n0 > 255`, which the clamp rules out.
    #[test]
    fn build_depth_fog_clut_matches() {
        oc1058_install();
        check_reaching(
            ssx3_ee::s12::f_0036a428__generated,
            ps2_graphics_man_build_depth_fog_clut,
            440,
            &[0x0036_a6ec, 0x0036_aa34],
            &oc1058_setup,
        );
    }
}
