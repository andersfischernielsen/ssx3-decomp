//! Staging module oc1100 (wave wave268): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1100`.

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

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003b_f8d0, snd_enc_filter_stage2, "SNDENC_FilterStage2"),
    (0x003b_f960, snd_enc_filter_stage3, "SNDENC_FilterStage3"),
    (0x003b_fac0, snd_enc_filter_stage4, "SNDENC_FilterStage4"),
    (0x003b_fb90, snd_enc_filter_stage5, "SNDENC_FilterStage5"),
    (0x003b_fc50, snd_enc_filter_stage6, "SNDENC_FilterStage6"),
    (0x003b_fd60, snd_enc_filter_stage7, "SNDENC_FilterStage7"),
    (0x003b_fe80, snd_enc_filter_stage8, "SNDENC_FilterStage8"),
    (
        0x003c_03e0,
        snd_enc_find_subband_peaks,
        "SNDENC_FindSubbandPeaks",
    ),
];

/// `SNDENC_FilterStage2` (0x003bf8d0): sound encoder filter pass 2 (VU0 macro
/// code). Clears the VF1 running sum, then for each of four 0x80-byte input rows
/// adds its eight quads into a running sum, stores the quads at `dst` (+0, +0x10,
/// ..) and the running sums at `dst + 0x100` (+0x100 .. +0x130) and advances both
/// pointers. Callers keep the generated register and cycle totals.
pub fn snd_enc_filter_stage2(c: &mut Cpu) {
    let mut src = c.g[A0][0] as u32;
    let mut dst = c.g[A1][0] as u32;
    c.cycles += 2;
    c.g[V0][0] = 0x4;
    c.vu0().sub(0b1111, 1, 0, Src::Vec(0));
    loop {
        c.cycles += 29;
        c.lqc2(2, src);
        c.lqc2(3, src.wrapping_add(0x10));
        c.lqc2(4, src.wrapping_add(0x20));
        c.lqc2(5, src.wrapping_add(0x30));
        c.lqc2(6, src.wrapping_add(0x40));
        c.lqc2(7, src.wrapping_add(0x50));
        c.lqc2(8, src.wrapping_add(0x60));
        c.lqc2(9, src.wrapping_add(0x70));
        c.vu0().add(0b1111, 1, 1, Src::Vec(2));
        c.vu0().add(0b1111, 2, 2, Src::Vec(3));
        c.vu0().add(0b1111, 3, 3, Src::Vec(4));
        c.vu0().add(0b1111, 4, 4, Src::Vec(5));
        c.vu0().add(0b1111, 5, 5, Src::Vec(6));
        c.vu0().add(0b1111, 6, 6, Src::Vec(7));
        c.vu0().add(0b1111, 7, 7, Src::Vec(8));
        c.vu0().add(0b1111, 8, 8, Src::Vec(9));
        c.sqc2(2, dst);
        c.sqc2(4, dst.wrapping_add(0x10));
        c.sqc2(6, dst.wrapping_add(0x20));
        c.sqc2(8, dst.wrapping_add(0x30));
        c.sqc2(1, dst.wrapping_add(0x100));
        c.sqc2(3, dst.wrapping_add(0x110));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.sqc2(5, dst.wrapping_add(0x120));
        src = src.wrapping_add(0x80);
        c.sqc2(7, dst.wrapping_add(0x130));
        c.vu0().vmove(0b1111, 1, 9);
        let t = c.g[V0][0] != 0u64;
        dst = dst.wrapping_add(0x40);
        c.g[A0][0] = sx(src);
        c.g[A1][0] = sx(dst);
        if !t {
            c.cycles += 2;
            ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003bf8d8);
        }
    }
}

/// `SNDENC_FilterStage3` (0x003bf960): sound encoder filter pass 3 (VU0 macro
/// code). Straight-line: sixteen quads from `src` are passed through a butterfly
/// of vector adds and stored back to `dst` in an interleaved order, with the
/// second half of the block handled in the same way. Pointers are not advanced.
pub fn snd_enc_filter_stage3(c: &mut Cpu) {
    let src = c.g[A0][0] as u32;
    let dst = c.g[A1][0] as u32;
    c.cycles += 88;
    c.lqc2(1, src);
    c.lqc2(2, src.wrapping_add(0x10));
    c.lqc2(3, src.wrapping_add(0x20));
    c.lqc2(4, src.wrapping_add(0x30));
    c.lqc2(5, src.wrapping_add(0x40));
    c.lqc2(6, src.wrapping_add(0x50));
    c.lqc2(7, src.wrapping_add(0x60));
    c.lqc2(8, src.wrapping_add(0x70));
    c.lqc2(9, src.wrapping_add(0x80));
    c.lqc2(10, src.wrapping_add(0x90));
    c.lqc2(11, src.wrapping_add(0xa0));
    c.lqc2(12, src.wrapping_add(0xb0));
    c.lqc2(13, src.wrapping_add(0xc0));
    c.lqc2(14, src.wrapping_add(0xd0));
    c.lqc2(15, src.wrapping_add(0xe0));
    c.lqc2(16, src.wrapping_add(0xf0));
    c.sqc2(1, dst.wrapping_add(0x80));
    c.vu0().add(0b1111, 1, 1, Src::Vec(2));
    c.vu0().add(0b1111, 2, 2, Src::Vec(3));
    c.vu0().add(0b1111, 3, 3, Src::Vec(4));
    c.vu0().add(0b1111, 4, 4, Src::Vec(5));
    c.vu0().add(0b1111, 5, 5, Src::Vec(6));
    c.vu0().add(0b1111, 6, 6, Src::Vec(7));
    c.vu0().add(0b1111, 7, 7, Src::Vec(8));
    c.vu0().add(0b1111, 8, 8, Src::Vec(9));
    c.vu0().add(0b1111, 9, 9, Src::Vec(10));
    c.vu0().add(0b1111, 10, 10, Src::Vec(11));
    c.vu0().add(0b1111, 11, 11, Src::Vec(12));
    c.vu0().add(0b1111, 12, 12, Src::Vec(13));
    c.vu0().add(0b1111, 13, 13, Src::Vec(14));
    c.vu0().add(0b1111, 14, 14, Src::Vec(15));
    c.vu0().add(0b1111, 15, 15, Src::Vec(16));
    c.sqc2(1, dst);
    c.sqc2(3, dst.wrapping_add(0x10));
    c.sqc2(5, dst.wrapping_add(0x20));
    c.sqc2(7, dst.wrapping_add(0x30));
    c.sqc2(9, dst.wrapping_add(0x40));
    c.sqc2(11, dst.wrapping_add(0x50));
    c.sqc2(13, dst.wrapping_add(0x60));
    c.sqc2(15, dst.wrapping_add(0x70));
    c.sqc2(2, dst.wrapping_add(0x90));
    c.sqc2(4, dst.wrapping_add(0xa0));
    c.sqc2(6, dst.wrapping_add(0xb0));
    c.sqc2(8, dst.wrapping_add(0xc0));
    c.sqc2(10, dst.wrapping_add(0xd0));
    c.sqc2(12, dst.wrapping_add(0xe0));
    c.sqc2(14, dst.wrapping_add(0xf0));
    c.lqc2(1, src.wrapping_add(0x100));
    c.lqc2(2, src.wrapping_add(0x120));
    c.lqc2(3, src.wrapping_add(0x140));
    c.lqc2(4, src.wrapping_add(0x160));
    c.lqc2(5, src.wrapping_add(0x180));
    c.lqc2(6, src.wrapping_add(0x1a0));
    c.lqc2(7, src.wrapping_add(0x1c0));
    c.lqc2(8, src.wrapping_add(0x1e0));
    c.sqc2(1, dst.wrapping_add(0x100));
    c.sqc2(2, dst.wrapping_add(0x110));
    c.sqc2(3, dst.wrapping_add(0x120));
    c.sqc2(4, dst.wrapping_add(0x130));
    c.sqc2(5, dst.wrapping_add(0x140));
    c.sqc2(6, dst.wrapping_add(0x150));
    c.sqc2(7, dst.wrapping_add(0x160));
    c.sqc2(8, dst.wrapping_add(0x170));
    c.lqc2(1, src.wrapping_add(0x110));
    c.lqc2(2, src.wrapping_add(0x130));
    c.lqc2(3, src.wrapping_add(0x150));
    c.lqc2(4, src.wrapping_add(0x170));
    c.lqc2(5, src.wrapping_add(0x190));
    c.lqc2(6, src.wrapping_add(0x1b0));
    c.lqc2(7, src.wrapping_add(0x1d0));
    c.lqc2(8, src.wrapping_add(0x1f0));
    c.sqc2(1, dst.wrapping_add(0x180));
    c.vu0().add(0b1111, 1, 1, Src::Vec(2));
    c.vu0().add(0b1111, 2, 2, Src::Vec(3));
    c.vu0().add(0b1111, 3, 3, Src::Vec(4));
    c.vu0().add(0b1111, 4, 4, Src::Vec(5));
    c.vu0().add(0b1111, 5, 5, Src::Vec(6));
    c.vu0().add(0b1111, 6, 6, Src::Vec(7));
    c.vu0().add(0b1111, 7, 7, Src::Vec(8));
    c.sqc2(1, dst.wrapping_add(0x190));
    c.sqc2(2, dst.wrapping_add(0x1a0));
    c.sqc2(3, dst.wrapping_add(0x1b0));
    c.sqc2(4, dst.wrapping_add(0x1c0));
    c.sqc2(5, dst.wrapping_add(0x1d0));
    c.sqc2(6, dst.wrapping_add(0x1e0));
    c.sqc2(7, dst.wrapping_add(0x1f0));
    ret(c);
}

/// `SNDENC_FilterStage4` (0x003bfac0): sound encoder filter pass 4 (VU0 macro
/// code). A prologue adds eight input quads pairwise and stores them, then a
/// three-iteration loop (0x80 bytes of input and output per pass) repeats the
/// pairwise sum-and-store on the next block. Pointers and the counter are
/// written back before each event check and before the return.
pub fn snd_enc_filter_stage4(c: &mut Cpu) {
    let mut src = c.g[A0][0] as u32;
    let mut dst = c.g[A1][0] as u32;
    c.cycles += 26;
    c.lqc2(1, src);
    c.lqc2(2, src.wrapping_add(0x10));
    c.lqc2(3, src.wrapping_add(0x20));
    c.lqc2(4, src.wrapping_add(0x30));
    c.lqc2(5, src.wrapping_add(0x40));
    c.lqc2(6, src.wrapping_add(0x50));
    c.lqc2(7, src.wrapping_add(0x60));
    c.lqc2(8, src.wrapping_add(0x70));
    c.sqc2(1, dst.wrapping_add(0x40));
    c.vu0().add(0b1111, 1, 1, Src::Vec(2));
    c.vu0().add(0b1111, 2, 2, Src::Vec(3));
    c.vu0().add(0b1111, 3, 3, Src::Vec(4));
    c.vu0().add(0b1111, 4, 4, Src::Vec(5));
    c.vu0().add(0b1111, 5, 5, Src::Vec(6));
    c.vu0().add(0b1111, 6, 6, Src::Vec(7));
    c.vu0().add(0b1111, 7, 7, Src::Vec(8));
    c.sqc2(1, dst);
    c.sqc2(3, dst.wrapping_add(0x10));
    c.sqc2(5, dst.wrapping_add(0x20));
    c.sqc2(7, dst.wrapping_add(0x30));
    c.sqc2(2, dst.wrapping_add(0x50));
    c.g[V0][0] = 0x3;
    c.sqc2(4, dst.wrapping_add(0x60));
    src = src.wrapping_add(0x80);
    c.sqc2(6, dst.wrapping_add(0x70));
    dst = dst.wrapping_add(0x80);
    loop {
        c.cycles += 23;
        c.lqc2(1, src);
        c.lqc2(2, src.wrapping_add(0x10));
        c.lqc2(3, src.wrapping_add(0x20));
        c.lqc2(4, src.wrapping_add(0x30));
        c.lqc2(5, src.wrapping_add(0x40));
        c.lqc2(6, src.wrapping_add(0x50));
        c.lqc2(7, src.wrapping_add(0x60));
        c.lqc2(8, src.wrapping_add(0x70));
        c.sqc2(1, dst);
        c.sqc2(3, dst.wrapping_add(0x10));
        c.sqc2(5, dst.wrapping_add(0x20));
        c.sqc2(7, dst.wrapping_add(0x30));
        c.vu0().add(0b1111, 3, 2, Src::Vec(4));
        c.vu0().add(0b1111, 4, 4, Src::Vec(6));
        c.vu0().add(0b1111, 5, 6, Src::Vec(8));
        c.sqc2(2, dst.wrapping_add(0x40));
        c.sqc2(3, dst.wrapping_add(0x50));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.sqc2(4, dst.wrapping_add(0x60));
        src = src.wrapping_add(0x80);
        c.sqc2(5, dst.wrapping_add(0x70));
        let t = c.g[V0][0] != 0u64;
        dst = dst.wrapping_add(0x80);
        c.g[A0][0] = sx(src);
        c.g[A1][0] = sx(dst);
        if !t {
            c.cycles += 2;
            ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003bfb28);
        }
    }
}

/// `SNDENC_FilterStage5` (0x003bfb90): sound encoder filter pass 5 (VU0 macro
/// code). Sums and differences of quad pairs from each 0x80-byte input block,
/// stored to `dst`, repeated for three blocks in a loop with the event check at
/// the back edge.
pub fn snd_enc_filter_stage5(c: &mut Cpu) {
    let mut src = c.g[A0][0] as u32;
    let mut dst = c.g[A1][0] as u32;
    c.cycles += 23;
    c.lqc2(1, src);
    c.lqc2(2, src.wrapping_add(0x10));
    c.lqc2(3, src.wrapping_add(0x20));
    c.lqc2(4, src.wrapping_add(0x30));
    c.lqc2(5, src.wrapping_add(0x40));
    c.lqc2(6, src.wrapping_add(0x50));
    c.lqc2(7, src.wrapping_add(0x60));
    c.lqc2(8, src.wrapping_add(0x70));
    c.vu0().add(0b1111, 9, 1, Src::Vec(2));
    c.vu0().add(0b1111, 10, 3, Src::Vec(4));
    c.vu0().add(0b1111, 11, 2, Src::Vec(3));
    c.vu0().add(0b1111, 12, 6, Src::Vec(8));
    c.sqc2(9, dst);
    c.sqc2(10, dst.wrapping_add(0x10));
    c.sqc2(1, dst.wrapping_add(0x20));
    c.sqc2(11, dst.wrapping_add(0x30));
    c.sqc2(5, dst.wrapping_add(0x40));
    c.sqc2(7, dst.wrapping_add(0x50));
    c.g[V0][0] = 0x3;
    c.sqc2(6, dst.wrapping_add(0x60));
    src = src.wrapping_add(0x80);
    c.sqc2(12, dst.wrapping_add(0x70));
    dst = dst.wrapping_add(0x80);
    loop {
        c.cycles += 22;
        c.lqc2(1, src);
        c.lqc2(2, src.wrapping_add(0x10));
        c.lqc2(3, src.wrapping_add(0x20));
        c.lqc2(4, src.wrapping_add(0x30));
        c.lqc2(5, src.wrapping_add(0x40));
        c.lqc2(6, src.wrapping_add(0x50));
        c.lqc2(7, src.wrapping_add(0x60));
        c.lqc2(8, src.wrapping_add(0x70));
        c.vu0().add(0b1111, 9, 2, Src::Vec(4));
        c.vu0().add(0b1111, 10, 6, Src::Vec(8));
        c.sqc2(1, dst);
        c.sqc2(3, dst.wrapping_add(0x10));
        c.sqc2(2, dst.wrapping_add(0x20));
        c.sqc2(9, dst.wrapping_add(0x30));
        c.sqc2(5, dst.wrapping_add(0x40));
        c.sqc2(7, dst.wrapping_add(0x50));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.sqc2(6, dst.wrapping_add(0x60));
        src = src.wrapping_add(0x80);
        c.sqc2(10, dst.wrapping_add(0x70));
        let t = c.g[V0][0] != 0u64;
        dst = dst.wrapping_add(0x80);
        c.g[A0][0] = sx(src);
        c.g[A1][0] = sx(dst);
        if !t {
            c.cycles += 2;
            ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003bfbec);
        }
    }
}

/// `SNDENC_FilterStage6` (0x003bfc50): sound encoder filter pass 6 (VU0 macro
/// code). Works in place on `src` (a0): butterflies of the quads, scaled by the
/// coefficients loaded from `coef` (a1, VF16), three blocks of 0x80 bytes.
pub fn snd_enc_filter_stage6(c: &mut Cpu) {
    let mut src = c.g[A0][0] as u32;
    let coef = c.g[A1][0] as u32;
    c.cycles += 34;
    c.lqc2(16, coef);
    c.lqc2(1, src);
    c.lqc2(2, src.wrapping_add(0x10));
    c.lqc2(3, src.wrapping_add(0x20));
    c.lqc2(4, src.wrapping_add(0x30));
    c.lqc2(5, src.wrapping_add(0x40));
    c.lqc2(6, src.wrapping_add(0x50));
    c.lqc2(7, src.wrapping_add(0x60));
    c.lqc2(8, src.wrapping_add(0x70));
    c.vu0().add(0b1111, 9, 1, Src::Vec(2));
    c.vu0().mul(0b1111, 10, 4, Src::Bc(16, X));
    c.vu0().mul(0b1111, 11, 6, Src::Bc(16, X));
    c.vu0().mul(0b1111, 12, 8, Src::Bc(16, X));
    c.vu0().mul(0b1111, 9, 9, Src::Bc(16, X));
    c.vu0().sub(0b1111, 4, 3, Src::Vec(10));
    c.vu0().add(0b1111, 3, 3, Src::Vec(10));
    c.vu0().sub(0b1111, 6, 5, Src::Vec(11));
    c.vu0().add(0b1111, 5, 5, Src::Vec(11));
    c.vu0().sub(0b1111, 2, 1, Src::Vec(9));
    c.vu0().add(0b1111, 1, 1, Src::Vec(9));
    c.vu0().sub(0b1111, 8, 7, Src::Vec(12));
    c.vu0().add(0b1111, 7, 7, Src::Vec(12));
    c.vu0().mul(0b1111, 2, 2, Src::Bc(16, Y));
    c.vu0().mul(0b1111, 1, 1, Src::Bc(16, Z));
    c.sqc2(3, src.wrapping_add(0x20));
    c.sqc2(4, src.wrapping_add(0x30));
    c.sqc2(5, src.wrapping_add(0x40));
    c.sqc2(6, src.wrapping_add(0x50));
    c.sqc2(7, src.wrapping_add(0x60));
    c.sqc2(8, src.wrapping_add(0x70));
    c.sqc2(1, src);
    c.g[V0][0] = 0x3;
    c.sqc2(2, src.wrapping_add(0x10));
    src = src.wrapping_add(0x80);
    loop {
        c.cycles += 31;
        c.lqc2(1, src);
        c.lqc2(2, src.wrapping_add(0x10));
        c.lqc2(3, src.wrapping_add(0x20));
        c.lqc2(4, src.wrapping_add(0x30));
        c.lqc2(5, src.wrapping_add(0x40));
        c.lqc2(6, src.wrapping_add(0x50));
        c.lqc2(7, src.wrapping_add(0x60));
        c.lqc2(8, src.wrapping_add(0x70));
        c.vu0().mul(0b1111, 9, 2, Src::Bc(16, X));
        c.vu0().mul(0b1111, 10, 4, Src::Bc(16, X));
        c.vu0().mul(0b1111, 11, 6, Src::Bc(16, X));
        c.vu0().mul(0b1111, 12, 8, Src::Bc(16, X));
        c.vu0().sub(0b1111, 2, 1, Src::Vec(9));
        c.vu0().add(0b1111, 1, 1, Src::Vec(9));
        c.vu0().sub(0b1111, 4, 3, Src::Vec(10));
        c.vu0().add(0b1111, 3, 3, Src::Vec(10));
        c.vu0().sub(0b1111, 6, 5, Src::Vec(11));
        c.vu0().add(0b1111, 5, 5, Src::Vec(11));
        c.vu0().sub(0b1111, 8, 7, Src::Vec(12));
        c.vu0().add(0b1111, 7, 7, Src::Vec(12));
        c.sqc2(1, src);
        c.sqc2(2, src.wrapping_add(0x10));
        c.sqc2(3, src.wrapping_add(0x20));
        c.sqc2(4, src.wrapping_add(0x30));
        c.sqc2(5, src.wrapping_add(0x40));
        c.sqc2(6, src.wrapping_add(0x50));
        c.sqc2(7, src.wrapping_add(0x60));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.sqc2(8, src.wrapping_add(0x70));
        let t = c.g[V0][0] != 0u64;
        src = src.wrapping_add(0x80);
        c.g[A0][0] = sx(src);
        if !t {
            c.cycles += 2;
            ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003bfcd8);
        }
    }
}

/// `SNDENC_FilterStage7` (0x003bfd60): sound encoder filter pass 7 (VU0 macro
/// code). Scales the input quads from `src` (a0) by the coefficients at `coef`
/// (a1, VF15/VF16), forms sums and differences, and stores them to `out` (a2),
/// three blocks of 0x80 bytes in a loop.
pub fn snd_enc_filter_stage7(c: &mut Cpu) {
    let mut src = c.g[A0][0] as u32;
    let coef = c.g[A1][0] as u32;
    let mut out = c.g[A2][0] as u32;
    c.cycles += 35;
    c.lqc2(15, coef);
    c.lqc2(16, coef.wrapping_add(0x10));
    c.lqc2(1, src);
    c.lqc2(2, src.wrapping_add(0x10));
    c.lqc2(3, src.wrapping_add(0x20));
    c.lqc2(4, src.wrapping_add(0x30));
    c.lqc2(5, src.wrapping_add(0x40));
    c.lqc2(6, src.wrapping_add(0x50));
    c.lqc2(7, src.wrapping_add(0x60));
    c.lqc2(8, src.wrapping_add(0x70));
    c.vu0().add(0b1111, 9, 1, Src::Vec(3));
    c.vu0().add(0b1111, 10, 2, Src::Vec(4));
    c.vu0().sub(0b1111, 11, 4, Src::Vec(2));
    c.vu0().sub(0b1111, 12, 3, Src::Vec(1));
    c.vu0().mul(0b1111, 9, 9, Src::Bc(15, X));
    c.vu0().mul(0b1111, 10, 10, Src::Bc(15, Y));
    c.vu0().mul(0b1111, 11, 11, Src::Bc(15, Z));
    c.vu0().mul(0b1111, 12, 12, Src::Bc(15, W));
    c.vu0().mul(0b1111, 7, 7, Src::Bc(16, X));
    c.vu0().mul(0b1111, 8, 8, Src::Bc(16, Y));
    c.sqc2(9, out);
    c.sqc2(10, out.wrapping_add(0x10));
    c.sqc2(11, out.wrapping_add(0x20));
    c.sqc2(12, out.wrapping_add(0x30));
    c.vu0().add(0b1111, 9, 5, Src::Vec(7));
    c.vu0().add(0b1111, 10, 6, Src::Vec(8));
    c.vu0().sub(0b1111, 11, 6, Src::Vec(8));
    c.vu0().sub(0b1111, 12, 5, Src::Vec(7));
    c.sqc2(9, out.wrapping_add(0x40));
    c.sqc2(10, out.wrapping_add(0x50));
    c.g[V0][0] = 0x3;
    c.sqc2(11, out.wrapping_add(0x60));
    src = src.wrapping_add(0x80);
    c.sqc2(12, out.wrapping_add(0x70));
    out = out.wrapping_add(0x80);
    loop {
        c.cycles += 32;
        c.lqc2(1, src);
        c.lqc2(2, src.wrapping_add(0x10));
        c.lqc2(3, src.wrapping_add(0x20));
        c.lqc2(4, src.wrapping_add(0x30));
        c.lqc2(5, src.wrapping_add(0x40));
        c.lqc2(6, src.wrapping_add(0x50));
        c.lqc2(7, src.wrapping_add(0x60));
        c.lqc2(8, src.wrapping_add(0x70));
        c.vu0().mul(0b1111, 3, 3, Src::Bc(16, X));
        c.vu0().mul(0b1111, 4, 4, Src::Bc(16, Y));
        c.vu0().mul(0b1111, 7, 7, Src::Bc(16, X));
        c.vu0().mul(0b1111, 8, 8, Src::Bc(16, Y));
        c.vu0().add(0b1111, 9, 1, Src::Vec(3));
        c.vu0().add(0b1111, 10, 2, Src::Vec(4));
        c.vu0().sub(0b1111, 11, 2, Src::Vec(4));
        c.vu0().sub(0b1111, 12, 1, Src::Vec(3));
        c.sqc2(9, out);
        c.sqc2(10, out.wrapping_add(0x10));
        c.sqc2(11, out.wrapping_add(0x20));
        c.sqc2(12, out.wrapping_add(0x30));
        c.vu0().add(0b1111, 9, 5, Src::Vec(7));
        c.vu0().add(0b1111, 10, 6, Src::Vec(8));
        c.vu0().sub(0b1111, 11, 6, Src::Vec(8));
        c.vu0().sub(0b1111, 12, 5, Src::Vec(7));
        c.sqc2(9, out.wrapping_add(0x40));
        c.sqc2(10, out.wrapping_add(0x50));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.sqc2(11, out.wrapping_add(0x60));
        src = src.wrapping_add(0x80);
        c.sqc2(12, out.wrapping_add(0x70));
        let t = c.g[V0][0] != 0u64;
        out = out.wrapping_add(0x80);
        c.g[A0][0] = sx(src);
        c.g[A2][0] = sx(out);
        if !t {
            c.cycles += 2;
            ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003bfdec);
        }
    }
}

/// `SNDENC_FilterStage8` (0x003bfe80): sound encoder filter pass 8 (VU0 macro
/// code). Like stage 7 with the second coefficient quad at `coef + 0x20`: a
/// prologue, then a loop over three 0x80-byte blocks of `src` writing `out`.
pub fn snd_enc_filter_stage8(c: &mut Cpu) {
    let mut src = c.g[A0][0] as u32;
    let coef = c.g[A1][0] as u32;
    let mut out = c.g[A2][0] as u32;
    c.cycles += 38;
    c.lqc2(15, coef);
    c.lqc2(16, coef.wrapping_add(0x10));
    c.lqc2(1, src);
    c.lqc2(2, src.wrapping_add(0x10));
    c.lqc2(3, src.wrapping_add(0x20));
    c.lqc2(4, src.wrapping_add(0x30));
    c.lqc2(5, src.wrapping_add(0x40));
    c.lqc2(6, src.wrapping_add(0x50));
    c.lqc2(7, src.wrapping_add(0x60));
    c.lqc2(8, src.wrapping_add(0x70));
    c.vu0().add(0b1111, 9, 1, Src::Vec(5));
    c.vu0().add(0b1111, 10, 2, Src::Vec(6));
    c.vu0().add(0b1111, 11, 3, Src::Vec(7));
    c.vu0().add(0b1111, 12, 4, Src::Vec(8));
    c.vu0().mul(0b1111, 9, 9, Src::Bc(15, X));
    c.vu0().mul(0b1111, 10, 10, Src::Bc(15, Y));
    c.vu0().mul(0b1111, 11, 11, Src::Bc(15, Z));
    c.vu0().mul(0b1111, 12, 12, Src::Bc(15, W));
    c.sqc2(9, out);
    c.sqc2(10, out.wrapping_add(0x10));
    c.sqc2(11, out.wrapping_add(0x20));
    c.sqc2(12, out.wrapping_add(0x30));
    c.vu0().sub(0b1111, 9, 8, Src::Vec(4));
    c.vu0().sub(0b1111, 10, 7, Src::Vec(3));
    c.vu0().sub(0b1111, 11, 6, Src::Vec(2));
    c.vu0().sub(0b1111, 12, 5, Src::Vec(1));
    c.vu0().mul(0b1111, 9, 9, Src::Bc(16, X));
    c.vu0().mul(0b1111, 10, 10, Src::Bc(16, Y));
    c.vu0().mul(0b1111, 11, 11, Src::Bc(16, Z));
    c.vu0().mul(0b1111, 12, 12, Src::Bc(16, W));
    c.sqc2(9, out.wrapping_add(0x40));
    c.sqc2(10, out.wrapping_add(0x50));
    c.g[V0][0] = 0x3;
    c.sqc2(11, out.wrapping_add(0x60));
    src = src.wrapping_add(0x80);
    c.sqc2(12, out.wrapping_add(0x70));
    out = out.wrapping_add(0x80);
    c.lqc2(16, coef.wrapping_add(0x20));
    loop {
        c.cycles += 32;
        c.lqc2(1, src);
        c.lqc2(2, src.wrapping_add(0x10));
        c.lqc2(3, src.wrapping_add(0x20));
        c.lqc2(4, src.wrapping_add(0x30));
        c.lqc2(5, src.wrapping_add(0x40));
        c.lqc2(6, src.wrapping_add(0x50));
        c.lqc2(7, src.wrapping_add(0x60));
        c.lqc2(8, src.wrapping_add(0x70));
        c.vu0().mul(0b1111, 5, 5, Src::Bc(16, X));
        c.vu0().mul(0b1111, 6, 6, Src::Bc(16, Y));
        c.vu0().mul(0b1111, 7, 7, Src::Bc(16, Z));
        c.vu0().mul(0b1111, 8, 8, Src::Bc(16, W));
        c.vu0().add(0b1111, 9, 1, Src::Vec(5));
        c.vu0().add(0b1111, 10, 2, Src::Vec(6));
        c.vu0().add(0b1111, 11, 3, Src::Vec(7));
        c.vu0().add(0b1111, 12, 4, Src::Vec(8));
        c.sqc2(9, out);
        c.sqc2(10, out.wrapping_add(0x10));
        c.sqc2(11, out.wrapping_add(0x20));
        c.sqc2(12, out.wrapping_add(0x30));
        c.vu0().sub(0b1111, 9, 4, Src::Vec(8));
        c.vu0().sub(0b1111, 10, 3, Src::Vec(7));
        c.vu0().sub(0b1111, 11, 2, Src::Vec(6));
        c.vu0().sub(0b1111, 12, 1, Src::Vec(5));
        c.sqc2(9, out.wrapping_add(0x40));
        c.sqc2(10, out.wrapping_add(0x50));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.sqc2(11, out.wrapping_add(0x60));
        src = src.wrapping_add(0x80);
        c.sqc2(12, out.wrapping_add(0x70));
        let t = c.g[V0][0] != 0u64;
        out = out.wrapping_add(0x80);
        c.g[A0][0] = sx(src);
        c.g[A2][0] = sx(out);
        if !t {
            c.cycles += 2;
            ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003bff18);
        }
    }
}

/// `SNDENC_FindSubbandPeaks` (0x003c03e0): per subband, masks the sign bit off
/// the sixteen quads of a 0x100-byte block of `src` (a0) and max-reduces them
/// into one quad of four 32-bit peaks, stored at `dst` (a1). Six blocks; `src`
/// advances 0x100 and `dst` 0x10 per block.
pub fn snd_enc_find_subband_peaks(c: &mut Cpu) {
    let mut src = c.g[A0][0] as u32;
    let mut dst = c.g[A1][0] as u32;
    c.cycles += 3;
    c.pnor(14, 0, 0);
    c.psrlw(14, 14, 1);
    c.g[V0][0] = 0x6;
    loop {
        c.cycles += 60;
        c.lq(6, src);
        c.lq(7, src.wrapping_add(0x40));
        c.lq(8, src.wrapping_add(0x80));
        c.lq(9, src.wrapping_add(0xc0));
        c.pand(6, 6, 14);
        c.pand(7, 7, 14);
        c.pand(8, 8, 14);
        c.pand(9, 9, 14);
        c.lq(10, src.wrapping_add(0x10));
        c.lq(11, src.wrapping_add(0x50));
        c.lq(12, src.wrapping_add(0x90));
        c.lq(13, src.wrapping_add(0xd0));
        c.pand(10, 10, 14);
        c.pand(11, 11, 14);
        c.pand(12, 12, 14);
        c.pand(13, 13, 14);
        c.pmaxw(6, 6, 10);
        c.pmaxw(7, 7, 11);
        c.pmaxw(8, 8, 12);
        c.pmaxw(9, 9, 13);
        c.lq(10, src.wrapping_add(0x20));
        c.lq(11, src.wrapping_add(0x60));
        c.lq(12, src.wrapping_add(0xa0));
        c.lq(13, src.wrapping_add(0xe0));
        c.pand(10, 10, 14);
        c.pand(11, 11, 14);
        c.pand(12, 12, 14);
        c.pand(13, 13, 14);
        c.pmaxw(6, 6, 10);
        c.pmaxw(7, 7, 11);
        c.pmaxw(8, 8, 12);
        c.pmaxw(9, 9, 13);
        c.lq(10, src.wrapping_add(0x30));
        c.lq(11, src.wrapping_add(0x70));
        c.lq(12, src.wrapping_add(0xb0));
        c.lq(13, src.wrapping_add(0xf0));
        c.pand(10, 10, 14);
        c.pand(11, 11, 14);
        c.pand(12, 12, 14);
        c.pand(13, 13, 14);
        c.pmaxw(6, 6, 10);
        c.pmaxw(7, 7, 11);
        c.pmaxw(8, 8, 12);
        c.pmaxw(9, 9, 13);
        c.pextlw(10, 7, 6);
        c.pextuw(7, 7, 6);
        c.pextlw(11, 9, 8);
        c.pextuw(9, 9, 8);
        c.pcpyld(6, 11, 10);
        c.pcpyud(8, 10, 11);
        c.pcpyld(10, 9, 7);
        c.pcpyud(11, 7, 9);
        c.pmaxw(6, 6, 8);
        c.pmaxw(10, 10, 11);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.pmaxw(6, 6, 10);
        src = src.wrapping_add(0x100);
        c.sq(6, dst);
        let t = c.g[V0][0] != 0u64;
        dst = dst.wrapping_add(0x10);
        c.g[A0][0] = sx(src);
        c.g[A1][0] = sx(dst);
        if !t {
            c.cycles += 2;
            ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c03ec);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA};

    /// Quad-aligned input block for the filter passes (0x800 bytes).
    pub(crate) const OC1100_SRC: u32 = DATA + 0x1000;
    /// Coefficient quads read through `a1` by stages 6 to 8.
    pub(crate) const OC1100_COEF: u32 = DATA + 0x2000;
    /// Output block (0x800 bytes).
    pub(crate) const OC1100_DST: u32 = DATA + 0x4000;

    pub(crate) fn oc1100_fill(c: &mut Cpu, r: &mut Rng, base: u32, bytes: u32) {
        for i in (0..bytes).step_by(4) {
            c.bus.write32(base + i, r.next_u32());
        }
    }

    /// Scramble every register, fill the blocks with random words and set the
    /// argument registers `a0`, `a1`, `a2`.
    pub(crate) fn oc1100_setup(c: &mut Cpu, seed: u32, salt: u64, a0: u32, a1: u32, a2: u32) {
        let mut r = Rng(salt ^ seed as u64);
        scramble(c, &mut r);
        oc1100_fill(c, &mut r, OC1100_SRC, 0x800);
        oc1100_fill(c, &mut r, OC1100_COEF, 0x40);
        oc1100_fill(c, &mut r, OC1100_DST, 0x800);
        c.g[A0][0] = a0 as u64;
        c.g[A1][0] = a1 as u64;
        c.g[A2][0] = a2 as u64;
    }

    #[test]
    fn snd_enc_filter_stage2_matches() {
        check(
            ee::s13::f_003bf8d0__generated,
            snd_enc_filter_stage2,
            200,
            &|c, seed| {
                oc1100_setup(c, seed, 0x3bf8d0, OC1100_SRC, OC1100_DST, 0);
            },
        );
    }

    #[test]
    fn snd_enc_filter_stage3_matches() {
        check(
            ee::s13::f_003bf960__generated,
            snd_enc_filter_stage3,
            200,
            &|c, seed| {
                oc1100_setup(c, seed, 0x3bf960, OC1100_SRC, OC1100_DST, 0);
            },
        );
    }

    #[test]
    fn snd_enc_filter_stage4_matches() {
        check(
            ee::s13::f_003bfac0__generated,
            snd_enc_filter_stage4,
            200,
            &|c, seed| {
                oc1100_setup(c, seed, 0x3bfac0, OC1100_SRC, OC1100_DST, 0);
            },
        );
    }

    #[test]
    fn snd_enc_filter_stage5_matches() {
        check(
            ee::s13::f_003bfb90__generated,
            snd_enc_filter_stage5,
            200,
            &|c, seed| {
                oc1100_setup(c, seed, 0x3bfb90, OC1100_SRC, OC1100_DST, 0);
            },
        );
    }

    #[test]
    fn snd_enc_filter_stage6_matches() {
        check(
            ee::s13::f_003bfc50__generated,
            snd_enc_filter_stage6,
            200,
            &|c, seed| {
                oc1100_setup(c, seed, 0x3bfc50, OC1100_SRC, OC1100_COEF, 0);
            },
        );
    }

    #[test]
    fn snd_enc_filter_stage7_matches() {
        check(
            ee::s13::f_003bfd60__generated,
            snd_enc_filter_stage7,
            200,
            &|c, seed| {
                oc1100_setup(c, seed, 0x3bfd60, OC1100_SRC, OC1100_COEF, OC1100_DST);
            },
        );
    }

    #[test]
    fn snd_enc_filter_stage8_matches() {
        check(
            ee::s13::f_003bfe80__generated,
            snd_enc_filter_stage8,
            200,
            &|c, seed| {
                oc1100_setup(c, seed, 0x3bfe80, OC1100_SRC, OC1100_COEF, OC1100_DST);
            },
        );
    }

    #[test]
    fn snd_enc_find_subband_peaks_matches() {
        check(
            ee::s13::f_003c03e0__generated,
            snd_enc_find_subband_peaks,
            200,
            &|c, seed| {
                oc1100_setup(c, seed, 0x3c03e0, OC1100_SRC, OC1100_DST, 0);
            },
        );
    }
}
