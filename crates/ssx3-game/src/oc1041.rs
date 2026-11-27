//! Staging module oc1041 (wave wave264): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1041`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x002d_5778,
    avalanche_sample_history,
    "Avalanche_SampleHistory",
)];

// Avalanche node (`self`) fields.
/// `self->frame`: the history frame the node was last advanced to.
pub(crate) const AVSH_FRAME: u32 = 0x0;
/// `self->pos` (t4Vector): the accumulated position.
pub(crate) const AVSH_POS: u32 = 0x10;
/// `self->mat` (4x4): the accumulated orientation.
pub(crate) const AVSH_MAT: u32 = 0x20;
/// `self->fB4`: the fade fraction `Avalanche_ComputeInstanceFade` writes.
pub(crate) const AVSH_FADE: u32 = 0xb4;
/// `self->emitter` (cDynamicColourEmitter).
pub(crate) const AVSH_EMITTER: u32 = 0xd0;
/// `self->f1FC`: the fade the last emitted particle used.
pub(crate) const AVSH_LAST_FADE: u32 = 0x1fc;
/// `self->data`: the packed history (see `AVSH_D_*`).
pub(crate) const AVSH_DATA: u32 = 0x2e0;
/// `self->emitting`: emit particles while replaying.
pub(crate) const AVSH_EMITTING: u32 = 0x2ec;
// Packed history (`self->data`) fields.
/// Start position (t4Vector) the history is reset to.
pub(crate) const AVSH_D_START: u32 = 0xe0;
/// Pointer to the 10-byte frame records.
pub(crate) const AVSH_D_FRAMES: u32 = 0xfc;
/// Frame count, as a float.
pub(crate) const AVSH_D_LENGTH: u32 = 0x100;
/// Playback rate multiplier, as a float.
pub(crate) const AVSH_D_RATE: u32 = 0x10c;

#[inline(always)]
fn avsh_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

#[inline(always)]
fn avsh_at_sub(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_sub(off)
}

/// `tFrameData Avalanche_SampleHistory(tFrameData *out, sAvAct *self, float time)`
/// (0x2d5778; the reference calls it `tActiveAvalancheNode_getFrameData`):
/// samples the packed avalanche history at `time` (30 Hz frames, 10 bytes per
/// frame: velocity, colour, axis, angle). It first updates the instance fade
/// (`Avalanche_ComputeInstanceFade`), then, when the frame differs from
/// `self->frame`, restarts from the start pose if it goes backwards and replays
/// every frame up to it (accumulating position and rotation with `Math_SinCos`,
/// and emitting particles through `DynamicEmitter_SpawnAt` while `emitting`),
/// and finally writes the position, the orientation and the interpolated colour
/// of the sampled frame into `out`.
///
/// Registers as the generated body: s1 = self, s4 = frame, f20 = time (then the
/// emit time), f22 = frac, f26 = 1 - frac; sp+0xf0 = out, sp+0x100 = &self->mat.
#[allow(unused_assignments, unused_variables, unused_labels)]
pub fn avalanche_sample_history(c: &mut Cpu) {
    guest(c, |c| {
        'b_2d581c: {
            c.cycles += 38;
            c.g[SP][0] = sx(avsh_at_sub(c, SP, 0x1f0));
            c.sq(17, avsh_at(c, SP, 0x190));
            c.swc1(20, avsh_at(c, SP, 0x1b0));
            c.g[S1][0] = c.g[A1][0];
            c.mov_s(20, 12);
            c.sw(avsh_at(c, SP, 0xf0), c.g[A0][0]);
            c.sq(20, avsh_at(c, SP, 0x160));
            c.swc1(22, avsh_at(c, SP, 0x1c0));
            c.g[A0][0] = c.g[S1][0];
            c.sq(16, avsh_at(c, SP, 0x1a0));
            c.sq(18, avsh_at(c, SP, 0x180));
            c.sq(19, avsh_at(c, SP, 0x170));
            c.sq(21, avsh_at(c, SP, 0x150));
            c.sq(22, avsh_at(c, SP, 0x140));
            c.sq(23, avsh_at(c, SP, 0x130));
            c.sq(30, avsh_at(c, SP, 0x120));
            c.sd(avsh_at(c, SP, 0x110), c.g[RA][0]);
            c.swc1(26, avsh_at(c, SP, 0x1e0));
            c.swc1(25, avsh_at(c, SP, 0x1d8));
            c.swc1(24, avsh_at(c, SP, 0x1d0));
            c.swc1(23, avsh_at(c, SP, 0x1c8));
            c.swc1(21, avsh_at(c, SP, 0x1b8));
            call_ptr(c, 0x002d_7c00, 0x002d_57d4)?;
            c.g[AT][0] = 0x41f00000;
            c.mtc1(1, 0);
            c.g[A0][0] = c.lw(avsh_at(c, S1, AVSH_DATA));
            c.mul_s(20, 20, 0);
            c.lwc1(1, avsh_at(c, A0, AVSH_D_LENGTH));
            c.cvt_w_s(0, 20);
            c.mfc1(20, 0);
            c.cvt_w_s(0, 1);
            c.mfc1(3, 0);
            c.mtc1(20, 0);
            c.cvt_s_w(0, 0);
            c.g[V0][0] = sx(avsh_at_sub(c, V1, 0x1));
            c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let t = c.g[V0][0] != 0u64;
            c.sub_s(22, 20, 0);
            if t {
                break 'b_2d581c;
            } else {
                c.cycles += 3;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 22);
                c.g[S4][0] = sx(avsh_at_sub(c, V1, 0x2));
                break 'b_2d581c;
            }
        }
        // Replay the recorded frames up to `frame`, restarting when going back.
        'b_2d5e30: {
            'b_2d5e2c: {
                c.cycles += 16;
                c.g[V0][0] = sx(avsh_at(c, S1, AVSH_MAT));
                c.g[V1][0] = sx(avsh_at(c, SP, 0xe8));
                c.sw(avsh_at(c, SP, 0x100), c.g[V0][0]);
                c.g[S7][0] = sx(avsh_at(c, SP, 0x20));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                c.g[FP][0] = sx(avsh_at(c, SP, 0x90));
                c.sw(avsh_at(c, SP, 0xf4), c.g[V1][0]);
                c.g[V1][0] = sx(avsh_at(c, SP, 0xec));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.sw(avsh_at(c, SP, 0xf8), c.g[V1][0]);
                c.g[V1][0] = c.lw(avsh_at(c, SP, 0xf0));
                c.sub_s(26, 0, 22);
                c.g[V1][0] = sx(avsh_at(c, V1, 0x20));
                let t = c.g[S4][0] == c.g[V0][0];
                c.sw(avsh_at(c, SP, 0xfc), c.g[V1][0]);
                if t {
                    break 'b_2d5e2c;
                } else {
                    'b_2d58a4: {
                        c.cycles += 2;
                        let t = c.g[S4][0] != 0u64;
                        c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if t {
                            break 'b_2d58a4;
                        } else {
                            c.cycles += 16;
                            c.sw((c.g[S1][0] as u32), 0u64);
                            c.g[V1][0] = 0x500000;
                            c.g[V1][0] = sx(avsh_at_sub(c, V1, 0xe60));
                            c.lq(2, avsh_at(c, A0, AVSH_D_START));
                            c.sq(2, avsh_at(c, S1, AVSH_POS));
                            c.g[A0][0] = c.lw(avsh_at(c, SP, 0x100));
                            c.lqc2(1, (c.g[V1][0] as u32));
                            c.lqc2(2, avsh_at(c, V1, 0x10));
                            c.lqc2(3, avsh_at(c, V1, 0x20));
                            c.lqc2(4, avsh_at(c, V1, 0x30));
                            c.sqc2(1, (c.g[A0][0] as u32));
                            c.sqc2(2, avsh_at(c, A0, 0x10));
                            c.sqc2(3, avsh_at(c, A0, 0x20));
                            c.sqc2(4, avsh_at(c, A0, 0x30));
                            let t = true;
                            c.g[A2][0] = 0xa;
                            if t {
                                break 'b_2d5e30;
                            } else {
                                break 'b_2d58a4;
                            }
                        }
                    }
                    'b_2d58e0: {
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = 0x500000;
                        if t {
                            break 'b_2d58e0;
                        } else {
                            c.cycles += 13;
                            c.sw((c.g[S1][0] as u32), 0u64);
                            c.g[V1][0] = sx(avsh_at_sub(c, V1, 0xe60));
                            c.lq(2, avsh_at(c, A0, AVSH_D_START));
                            c.sq(2, avsh_at(c, S1, AVSH_POS));
                            c.g[V0][0] = c.lw(avsh_at(c, SP, 0x100));
                            c.lqc2(1, (c.g[V1][0] as u32));
                            c.lqc2(2, avsh_at(c, V1, 0x10));
                            c.lqc2(3, avsh_at(c, V1, 0x20));
                            c.lqc2(4, avsh_at(c, V1, 0x30));
                            c.sqc2(1, (c.g[V0][0] as u32));
                            c.sqc2(2, avsh_at(c, V0, 0x10));
                            c.sqc2(3, avsh_at(c, V0, 0x20));
                            c.sqc2(4, avsh_at(c, V0, 0x30));
                            break 'b_2d58e0;
                        }
                    }
                    // Walk the recorded frames one step at a time (emit particles while `emitting`).
                    'b_2d5e28: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                        c.mtc1(0, 23);
                        let t = c.g[V0][0] == c.g[S4][0];
                        c.mov_s(24, 0);
                        if t {
                            break 'b_2d5e28;
                        } else {
                            c.cycles += 4;
                            c.lwc1(25, avsh_at_sub(c, GP, 0x3d14));
                            c.g[S6][0] = c.lw(avsh_at(c, SP, 0x100));
                            c.g[S5][0] = 0xa;
                            'l_2d5900: loop {
                                'b_2d5b5c: {
                                    'b_2d5b58: {
                                        c.cycles += 5;
                                        c.g[V0][0] = c.lw(avsh_at(c, S1, AVSH_EMITTING));
                                        c.g[S3][0] = sx(avsh_at(c, SP, 0x40));
                                        c.g[S0][0] = sx(avsh_at(c, SP, 0xe0));
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[S2][0] = sx(avsh_at(c, SP, 0xe4));
                                        if t {
                                            break 'b_2d5b58;
                                        } else {
                                            c.cycles += 9;
                                            c.g[AT][0] = 0x41f00000;
                                            c.mtc1(1, 1);
                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                                            c.mul_s(0, 23, 1);
                                            c.cvt_w_s(2, 0);
                                            c.mfc1(2, 2);
                                            if c.g[V0][0] != c.g[V1][0] {
                                                c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                                break 'b_2d5b5c;
                                            } else {
                                                c.cycles += 4;
                                                c.g[AT][0] = 0x3c000000;
                                                c.mtc1(1, 21);
                                                c.mul_s(10, 23, 1);
                                                'l_2d5948: loop {
                                                    'b_2d5984: {
                                                        c.cycles += 12;
                                                        c.g[A0][0] =
                                                            c.lw(avsh_at(c, S1, AVSH_DATA));
                                                        c.lwc1(3, avsh_at(c, A0, AVSH_D_LENGTH));
                                                        c.cvt_w_s(0, 10);
                                                        c.mfc1(5, 0);
                                                        c.cvt_w_s(0, 3);
                                                        c.mfc1(3, 0);
                                                        c.mtc1(5, 0);
                                                        c.cvt_s_w(0, 0);
                                                        c.g[V0][0] = sx(avsh_at_sub(c, V1, 0x1));
                                                        c.g[V0][0] = ((c.g[A1][0] as i64)
                                                            < (c.g[V0][0] as i64))
                                                            as u64;
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.sub_s(8, 10, 0);
                                                        if t {
                                                            break 'b_2d5984;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[AT][0] = 0x3f800000;
                                                            c.mtc1(1, 8);
                                                            c.g[A1][0] =
                                                                sx(avsh_at_sub(c, V1, 0x2));
                                                            break 'b_2d5984;
                                                        }
                                                    }
                                                    'b_2d59b4: {
                                                        c.cycles += 11;
                                                        c.lwc1(0, avsh_at(c, GP, 0x974));
                                                        c.sub_s(9, 24, 8);
                                                        c.lwc1(2, avsh_at(c, A0, AVSH_D_RATE));
                                                        c.lwc1(1, avsh_at_sub(c, GP, 0x3d10));
                                                        c.mul_s(0, 0, 2);
                                                        c.mul_s(1, 3, 1);
                                                        c.add_s(0, 23, 0);
                                                        c.c_lt_s(0, 1);
                                                        let t = !c.fcc();
                                                        c.mov_s(20, 1);
                                                        if t {
                                                            break 'b_2d59b4;
                                                        } else {
                                                            c.cycles += 1;
                                                            c.mov_s(20, 0);
                                                            break 'b_2d59b4;
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.c_lt_s(10, 3);
                                                    let t = !c.fcc();
                                                    c.mov_s(23, 20);
                                                    if t {
                                                        break 'b_2d5b58;
                                                    } else {
                                                        'b_2d5b20: {
                                                            c.cycles += 87;
                                                            c.mult(0, 5, 5, 21);
                                                            c.g[V0][0] =
                                                                c.lw(avsh_at(c, A0, AVSH_D_FRAMES));
                                                            c.g[V1][0] = 0x500000;
                                                            c.g[T1][0] =
                                                                sx(avsh_at(c, S1, AVSH_EMITTER));
                                                            c.g[T2][0] =
                                                                sx(avsh_at_sub(c, V1, 0xee0));
                                                            c.g[A3][0] = sx(avsh_at(c, SP, 0x10));
                                                            c.g[T0][0] = 0x1;
                                                            c.g[V0][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add((c.g[V0][0] as u32)));
                                                            c.g[V1][0] = c.lb(avsh_at(c, V0, 0x2));
                                                            c.g[A0][0] = c.lb((c.g[V0][0] as u32));
                                                            c.g[A2][0] = c.lb(avsh_at(c, V0, 0x1));
                                                            c.mtc1(3, 1);
                                                            c.cvt_s_w(1, 1);
                                                            c.mtc1(4, 2);
                                                            c.cvt_s_w(2, 2);
                                                            c.sw(avsh_at(c, SP, 0x1c), 0u64);
                                                            c.mtc1(6, 0);
                                                            c.cvt_s_w(0, 0);
                                                            c.mul_s(1, 1, 8);
                                                            c.mul_s(2, 2, 8);
                                                            c.mul_s(0, 0, 8);
                                                            c.add_s(1, 1, 1);
                                                            c.add_s(2, 2, 2);
                                                            c.add_s(0, 0, 0);
                                                            c.swc1(1, avsh_at(c, SP, 0x18));
                                                            c.swc1(2, avsh_at(c, SP, 0x10));
                                                            c.swc1(0, avsh_at(c, SP, 0x14));
                                                            c.lqc2(3, avsh_at(c, SP, 0x10));
                                                            c.lqc2(4, avsh_at(c, S1, AVSH_POS));
                                                            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                                            c.sqc2(5, avsh_at(c, SP, 0x20));
                                                            c.lq(2, avsh_at(c, SP, 0x20));
                                                            c.g[V1][0] =
                                                                c.lw(avsh_at(c, S1, AVSH_DATA));
                                                            c.sq(2, (c.g[SP][0] as u32));
                                                            c.lwc1(6, avsh_at(c, GP, 0x9b8));
                                                            c.g[V0][0] =
                                                                c.lw(avsh_at(c, V1, AVSH_D_FRAMES));
                                                            c.lwc1(7, avsh_at(c, S1, AVSH_FADE));
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add((c.g[V0][0] as u32)));
                                                            c.g[V1][0] = c.lbu(avsh_at(c, A1, 0xf));
                                                            c.g[V0][0] = c.lbu(avsh_at(c, A1, 0x3));
                                                            c.mtc1(3, 5);
                                                            c.cvt_s_w(5, 5);
                                                            c.g[A0][0] = c.lbu(avsh_at(c, A1, 0xd));
                                                            c.mtc1(2, 2);
                                                            c.cvt_s_w(2, 2);
                                                            c.g[A2][0] = c.lbu(avsh_at(c, A1, 0x4));
                                                            c.g[V0][0] = c.lbu(avsh_at(c, A1, 0x5));
                                                            c.mtc1(4, 4);
                                                            c.cvt_s_w(4, 4);
                                                            c.g[V1][0] = c.lbu(avsh_at(c, A1, 0xe));
                                                            c.mtc1(6, 1);
                                                            c.cvt_s_w(1, 1);
                                                            c.mtc1(2, 0);
                                                            c.cvt_s_w(0, 0);
                                                            c.swc1(7, avsh_at(c, SP, 0x2c));
                                                            c.mtc1(3, 3);
                                                            c.cvt_s_w(3, 3);
                                                            c.mul_s(5, 5, 8);
                                                            c.mul_s(2, 2, 9);
                                                            c.mul_s(0, 0, 9);
                                                            c.mul_s(4, 4, 8);
                                                            c.mul_s(1, 1, 9);
                                                            c.mul_s(3, 3, 8);
                                                            c.add_s(2, 2, 4);
                                                            c.add_s(0, 0, 5);
                                                            c.add_s(1, 1, 3);
                                                            c.mul_s(2, 2, 21);
                                                            c.mul_s(0, 0, 21);
                                                            c.mul_s(1, 1, 21);
                                                            c.swc1(2, avsh_at(c, SP, 0x20));
                                                            c.swc1(0, avsh_at(c, SP, 0x28));
                                                            c.swc1(1, avsh_at(c, SP, 0x24));
                                                            c.mfc1(3, 6);
                                                            c.lqc2(4, avsh_at(c, SP, 0x20));
                                                            c.qmtc2(3, 3);
                                                            c.vu0().mul(
                                                                0b1111,
                                                                5,
                                                                4,
                                                                Src::Bc(3, X),
                                                            );
                                                            c.sqc2(5, avsh_at(c, SP, 0x30));
                                                            c.lq(2, avsh_at(c, SP, 0x30));
                                                            c.sq(2, avsh_at(c, SP, 0x10));
                                                            c.lwc1(1, avsh_at(c, S1, AVSH_FADE));
                                                            c.g[V0][0] =
                                                                c.lw(avsh_at(c, S1, AVSH_DATA));
                                                            c.swc1(
                                                                1,
                                                                avsh_at(c, S1, AVSH_LAST_FADE),
                                                            );
                                                            c.lwc1(
                                                                0,
                                                                avsh_at(c, V0, AVSH_D_LENGTH),
                                                            );
                                                            c.c_lt_s(10, 0);
                                                            if !c.fcc() {
                                                                c.g[T0][0] = 0u64;
                                                                break 'b_2d5b20;
                                                            } else {
                                                                break 'b_2d5b20;
                                                            }
                                                        }
                                                        c.cycles += 14;
                                                        c.lwc1(12, avsh_at_sub(c, GP, 0x3d0c));
                                                        c.g[A0][0] = c.g[T1][0];
                                                        c.g[A2][0] = c.g[T2][0];
                                                        c.g[A1][0] = c.g[SP][0];
                                                        call_ptr(c, 0x003717c0, 0x002d5b34)?;
                                                        c.g[AT][0] = 0x41f00000;
                                                        c.mtc1(1, 1);
                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                                        c.mul_s(0, 20, 1);
                                                        c.cvt_w_s(2, 0);
                                                        c.mfc1(3, 2);
                                                        if c.g[V1][0] == c.g[V0][0] {
                                                            c.mul_s(10, 23, 1);
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x002d5948);
                                                            }
                                                            continue 'l_2d5948;
                                                        } else {
                                                            break 'b_2d5b58;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                    break 'b_2d5b5c;
                                }
                                'b_2d5c60: {
                                    'b_2d5c58: {
                                        c.cycles += 55;
                                        c.g[A0][0] = c.lw(avsh_at(c, S1, AVSH_DATA));
                                        c.mult(0, 2, 2, 21);
                                        c.mtc1(0, 3);
                                        c.g[V1][0] = c.lw(avsh_at(c, A0, AVSH_D_FRAMES));
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.g[A0][0] = c.lb(avsh_at(c, V0, 0x2));
                                        c.g[V1][0] = c.lb((c.g[V0][0] as u32));
                                        c.g[A1][0] = c.lb(avsh_at(c, V0, 0x1));
                                        c.mtc1(4, 1);
                                        c.cvt_s_w(1, 1);
                                        c.mtc1(3, 2);
                                        c.cvt_s_w(2, 2);
                                        c.swc1(3, avsh_at(c, SP, 0xc));
                                        c.mtc1(5, 0);
                                        c.cvt_s_w(0, 0);
                                        c.add_s(1, 1, 1);
                                        c.add_s(2, 2, 2);
                                        c.add_s(0, 0, 0);
                                        c.swc1(1, avsh_at(c, SP, 0x8));
                                        c.swc1(2, (c.g[SP][0] as u32));
                                        c.swc1(0, avsh_at(c, SP, 0x4));
                                        c.lqc2(3, avsh_at(c, S1, AVSH_POS));
                                        c.lqc2(4, (c.g[SP][0] as u32));
                                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, avsh_at(c, S1, AVSH_POS));
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                        c.g[A0][0] = c.lw(avsh_at(c, S1, AVSH_DATA));
                                        c.mult(0, 2, 2, 21);
                                        c.g[V1][0] = c.lw(avsh_at(c, A0, AVSH_D_FRAMES));
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.g[A0][0] = c.lb(avsh_at(c, V0, 0x6));
                                        c.g[A1][0] = c.lb(avsh_at(c, V0, 0x7));
                                        c.mtc1(4, 2);
                                        c.cvt_s_w(2, 2);
                                        c.g[V1][0] = c.lb(avsh_at(c, V0, 0x8));
                                        c.mtc1(5, 0);
                                        c.cvt_s_w(0, 0);
                                        c.mtc1(3, 1);
                                        c.cvt_s_w(1, 1);
                                        c.mul_s(5, 2, 25);
                                        c.mul_s(6, 0, 25);
                                        c.mul_s(4, 1, 25);
                                        c.mul_s(0, 5, 5);
                                        c.swc1(5, avsh_at(c, SP, 0x20));
                                        c.mul_s(2, 6, 6);
                                        c.swc1(6, avsh_at(c, SP, 0x24));
                                        c.mul_s(1, 4, 4);
                                        c.swc1(4, avsh_at(c, SP, 0x28));
                                        c.add_s(0, 0, 2);
                                        c.add_s(0, 0, 1);
                                        c.sqrt_s(0, 0);
                                        c.c_eq_s(0, 3);
                                        if c.fcc() {
                                            c.swc1(5, avsh_at(c, SP, 0x10));
                                            break 'b_2d5c58;
                                        } else {
                                            c.cycles += 8;
                                            c.div_s(0, 24, 0);
                                            c.mul_s(1, 4, 0);
                                            c.mul_s(2, 5, 0);
                                            c.mul_s(0, 6, 0);
                                            c.swc1(1, avsh_at(c, SP, 0x18));
                                            c.swc1(2, avsh_at(c, SP, 0x10));
                                            let t = true;
                                            c.swc1(0, avsh_at(c, SP, 0x14));
                                            if t {
                                                break 'b_2d5c60;
                                            } else {
                                                break 'b_2d5c58;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    c.swc1(6, avsh_at(c, SP, 0x14));
                                    c.swc1(4, avsh_at(c, SP, 0x18));
                                    break 'b_2d5c60;
                                }
                                c.cycles += 114;
                                c.lwc1(0, avsh_at(c, SP, 0x10));
                                c.lwc1(1, avsh_at(c, SP, 0x14));
                                c.lwc1(2, avsh_at(c, SP, 0x18));
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                c.swc1(0, (c.g[SP][0] as u32));
                                c.swc1(1, avsh_at(c, SP, 0x4));
                                c.mult(0, 4, 4, 21);
                                c.swc1(2, avsh_at(c, SP, 0x8));
                                c.sw(avsh_at(c, SP, 0xc), 0u64);
                                c.lwc1(0, avsh_at_sub(c, GP, 0x3d08));
                                c.g[V1][0] = c.lw(avsh_at(c, S1, AVSH_DATA));
                                c.g[V0][0] = c.lw(avsh_at(c, V1, AVSH_D_FRAMES));
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[V1][0] = c.lbu(avsh_at(c, A0, 0x9));
                                c.mtc1(3, 12);
                                c.cvt_s_w(12, 12);
                                c.mul_s(12, 12, 0);
                                c.lqc2(1, (c.g[S6][0] as u32));
                                c.lqc2(2, avsh_at(c, S6, 0x10));
                                c.lqc2(3, avsh_at(c, S6, 0x20));
                                c.lqc2(4, avsh_at(c, S6, 0x30));
                                c.sqc2(1, (c.g[S3][0] as u32));
                                c.sqc2(2, avsh_at(c, S3, 0x10));
                                c.sqc2(3, avsh_at(c, S3, 0x20));
                                c.sqc2(4, avsh_at(c, S3, 0x30));
                                c.g[V0][0] = 0x500000;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[V1][0] = sx(avsh_at_sub(c, V0, 0xed0));
                                c.lq(5, (c.g[V1][0] as u32));
                                c.lq(2, avsh_at(c, SP, 0x70));
                                c.sq(5, avsh_at(c, SP, 0x70));
                                c.sq(2, avsh_at(c, SP, 0x80));
                                c.g[A1][0] = c.g[S2][0];
                                call_ptr(c, 0x0031be50, 0x002d5ce8)?;
                                c.lwc1(11, avsh_at(c, SP, 0xe4));
                                c.lwc1(0, (c.g[SP][0] as u32));
                                c.sub_s(1, 24, 11);
                                c.lwc1(3, avsh_at(c, SP, 0x8));
                                c.lwc1(7, avsh_at(c, SP, 0x4));
                                c.lwc1(8, avsh_at(c, SP, 0xe0));
                                c.mul_s(2, 1, 3);
                                c.lq(2, avsh_at(c, SP, 0x80));
                                c.mul_s(4, 1, 0);
                                c.sw(avsh_at(c, S1, 0x50), 0u64);
                                c.mul_s(1, 1, 7);
                                c.sw(avsh_at(c, S1, 0x54), 0u64);
                                c.mul_s(13, 8, 3);
                                c.sw(avsh_at(c, S1, 0x58), 0u64);
                                c.mul_s(12, 8, 0);
                                c.swc1(24, avsh_at(c, S1, 0x5c));
                                c.mul_s(9, 2, 3);
                                c.sq(2, avsh_at(c, S1, 0x50));
                                c.mul_s(10, 4, 3);
                                c.sw(avsh_at(c, S1, 0x2c), 0u64);
                                c.mul_s(5, 2, 0);
                                c.sw(avsh_at(c, S1, 0x3c), 0u64);
                                c.mul_s(6, 4, 0);
                                c.sw(avsh_at(c, S1, 0x4c), 0u64);
                                c.mul_s(3, 1, 3);
                                c.mul_s(0, 1, 0);
                                c.mul_s(8, 8, 7);
                                c.mul_s(2, 2, 7);
                                c.mul_s(4, 4, 7);
                                c.mul_s(1, 1, 7);
                                c.add_s(5, 5, 8);
                                c.sub_s(0, 0, 13);
                                c.sub_s(2, 2, 12);
                                c.add_s(9, 9, 11);
                                c.swc1(5, avsh_at(c, S1, 0x28));
                                c.add_s(6, 6, 11);
                                c.swc1(0, avsh_at(c, S1, 0x24));
                                c.add_s(4, 4, 13);
                                c.swc1(2, avsh_at(c, S1, 0x38));
                                c.sub_s(10, 10, 8);
                                c.swc1(9, avsh_at(c, S1, 0x48));
                                c.add_s(1, 1, 11);
                                c.swc1(6, avsh_at(c, S1, AVSH_MAT));
                                c.add_s(3, 3, 12);
                                c.swc1(4, avsh_at(c, S1, 0x30));
                                c.swc1(10, avsh_at(c, S1, 0x40));
                                c.swc1(1, avsh_at(c, S1, 0x34));
                                c.swc1(3, avsh_at(c, S1, 0x44));
                                c.lqc2(4, (c.g[S6][0] as u32));
                                c.lqc2(5, avsh_at(c, S6, 0x10));
                                c.lqc2(6, avsh_at(c, S6, 0x20));
                                c.lqc2(7, avsh_at(c, S6, 0x30));
                                c.lqc2(8, (c.g[S3][0] as u32));
                                c.lqc2(9, avsh_at(c, S3, 0x10));
                                c.lqc2(10, avsh_at(c, S3, 0x20));
                                c.lqc2(11, avsh_at(c, S3, 0x30));
                                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                                c.vu0().mula(0b1111, 4, Src::Bc(9, X));
                                c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
                                c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
                                c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
                                c.vu0().mula(0b1111, 4, Src::Bc(10, X));
                                c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
                                c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
                                c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
                                c.vu0().mula(0b1111, 4, Src::Bc(11, X));
                                c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
                                c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
                                c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
                                c.sqc2(12, (c.g[S6][0] as u32));
                                c.sqc2(13, avsh_at(c, S6, 0x10));
                                c.sqc2(14, avsh_at(c, S6, 0x20));
                                c.sqc2(15, avsh_at(c, S6, 0x30));
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                c.g[V0][0] = sx(avsh_at(c, V0, 0x1));
                                let t = c.g[V0][0] != c.g[S4][0];
                                c.sw((c.g[S1][0] as u32), c.g[V0][0]);
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002d5900);
                                    }
                                    continue 'l_2d5900;
                                } else {
                                    break 'b_2d5e28;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.sw(avsh_at(c, S1, AVSH_EMITTING), 0u64);
                    break 'b_2d5e2c;
                }
            }
            c.cycles += 1;
            c.g[A2][0] = 0xa;
            break 'b_2d5e30;
        }
        // Result: interpolate frame `frame` with `frac` into the output record.
        'b_2d6038: {
            'b_2d6030: {
                c.cycles += 118;
                c.g[V1][0] = c.lw(avsh_at(c, S1, AVSH_DATA));
                c.mult(0, 6, 20, 6);
                c.g[AT][0] = 0x3c000000;
                c.mtc1(1, 6);
                c.g[V0][0] = c.lw(avsh_at(c, V1, AVSH_D_FRAMES));
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[V1][0] = c.lb(avsh_at(c, V0, 0x2));
                c.g[A0][0] = c.lb((c.g[V0][0] as u32));
                c.g[A1][0] = c.lb(avsh_at(c, V0, 0x1));
                c.mtc1(3, 1);
                c.cvt_s_w(1, 1);
                c.mtc1(4, 2);
                c.cvt_s_w(2, 2);
                c.sw(avsh_at(c, SP, 0x7c), 0u64);
                c.mtc1(5, 0);
                c.cvt_s_w(0, 0);
                c.mul_s(1, 1, 22);
                c.mul_s(2, 2, 22);
                c.mul_s(0, 0, 22);
                c.add_s(1, 1, 1);
                c.add_s(2, 2, 2);
                c.add_s(0, 0, 0);
                c.swc1(1, avsh_at(c, SP, 0x78));
                c.swc1(2, avsh_at(c, SP, 0x70));
                c.swc1(0, avsh_at(c, SP, 0x74));
                c.lqc2(3, avsh_at(c, SP, 0x70));
                c.lqc2(4, avsh_at(c, S1, AVSH_POS));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, avsh_at(c, SP, 0x80));
                c.lq(2, avsh_at(c, SP, 0x80));
                c.g[V1][0] = c.lw(avsh_at(c, S1, AVSH_DATA));
                c.sq(2, (c.g[SP][0] as u32));
                c.sq(2, avsh_at(c, SP, 0x60));
                c.lwc1(8, avsh_at(c, GP, 0x9b8));
                c.g[V0][0] = c.lw(avsh_at(c, V1, AVSH_D_FRAMES));
                c.lwc1(7, avsh_at(c, S1, AVSH_FADE));
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[V1][0] = c.lbu(avsh_at(c, V0, 0xf));
                c.g[A0][0] = c.lbu(avsh_at(c, V0, 0x3));
                c.mtc1(3, 5);
                c.cvt_s_w(5, 5);
                c.g[A1][0] = c.lbu(avsh_at(c, V0, 0xd));
                c.mtc1(4, 2);
                c.cvt_s_w(2, 2);
                c.g[A3][0] = c.lbu(avsh_at(c, V0, 0x4));
                c.g[V1][0] = c.lbu(avsh_at(c, V0, 0xe));
                c.mtc1(5, 4);
                c.cvt_s_w(4, 4);
                c.g[A0][0] = c.lbu(avsh_at(c, V0, 0x5));
                c.mtc1(7, 1);
                c.cvt_s_w(1, 1);
                c.mtc1(3, 3);
                c.cvt_s_w(3, 3);
                c.swc1(7, avsh_at(c, SP, 0x7c));
                c.mtc1(4, 0);
                c.cvt_s_w(0, 0);
                c.mul_s(2, 2, 26);
                c.mul_s(4, 4, 22);
                c.mul_s(0, 0, 26);
                c.mul_s(1, 1, 26);
                c.mul_s(3, 3, 22);
                c.mul_s(5, 5, 22);
                c.add_s(2, 2, 4);
                c.add_s(1, 1, 3);
                c.add_s(0, 0, 5);
                c.mul_s(2, 2, 6);
                c.mul_s(1, 1, 6);
                c.mul_s(0, 0, 6);
                c.swc1(2, avsh_at(c, SP, 0x70));
                c.swc1(1, avsh_at(c, SP, 0x74));
                c.swc1(0, avsh_at(c, SP, 0x78));
                c.mfc1(3, 8);
                c.lqc2(4, avsh_at(c, SP, 0x70));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, avsh_at(c, SP, 0x80));
                c.lq(2, avsh_at(c, SP, 0x80));
                c.sq(2, avsh_at(c, SP, 0x60));
                c.sq(2, avsh_at(c, SP, 0x10));
                c.g[V1][0] = c.lw(avsh_at(c, SP, 0x100));
                c.lqc2(1, (c.g[V1][0] as u32));
                c.lqc2(2, avsh_at(c, V1, 0x10));
                c.lqc2(3, avsh_at(c, V1, 0x20));
                c.lqc2(4, avsh_at(c, V1, 0x30));
                c.sqc2(1, (c.g[S7][0] as u32));
                c.sqc2(2, avsh_at(c, S7, 0x10));
                c.sqc2(3, avsh_at(c, S7, 0x20));
                c.sqc2(4, avsh_at(c, S7, 0x30));
                c.g[V0][0] = c.lw(avsh_at(c, S1, AVSH_DATA));
                c.lwc1(1, avsh_at_sub(c, GP, 0x3d04));
                c.g[V1][0] = c.lw(avsh_at(c, V0, AVSH_D_FRAMES));
                c.mtc1(0, 4);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V0][0] = c.lb(avsh_at(c, A2, 0x6));
                c.g[A0][0] = c.lb(avsh_at(c, A2, 0x7));
                c.g[V1][0] = c.lb(avsh_at(c, A2, 0x8));
                c.mtc1(2, 3);
                c.cvt_s_w(3, 3);
                c.mtc1(4, 0);
                c.cvt_s_w(0, 0);
                c.mtc1(3, 2);
                c.cvt_s_w(2, 2);
                c.mul_s(3, 3, 1);
                c.mul_s(6, 0, 1);
                c.mul_s(5, 2, 1);
                c.mul_s(0, 3, 3);
                c.swc1(3, avsh_at(c, SP, 0x80));
                c.mul_s(2, 6, 6);
                c.swc1(6, avsh_at(c, SP, 0x84));
                c.mul_s(1, 5, 5);
                c.swc1(5, avsh_at(c, SP, 0x88));
                c.add_s(0, 0, 2);
                c.add_s(0, 0, 1);
                c.sqrt_s(1, 0);
                c.c_eq_s(1, 4);
                if c.fcc() {
                    c.swc1(3, avsh_at(c, SP, 0x70));
                    break 'b_2d6030;
                } else {
                    c.cycles += 10;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.div_s(0, 0, 1);
                    c.mul_s(1, 5, 0);
                    c.mul_s(2, 3, 0);
                    c.mul_s(0, 6, 0);
                    c.swc1(1, avsh_at(c, SP, 0x78));
                    c.swc1(2, avsh_at(c, SP, 0x70));
                    let t = true;
                    c.swc1(0, avsh_at(c, SP, 0x74));
                    if t {
                        break 'b_2d6038;
                    } else {
                        break 'b_2d6030;
                    }
                }
            }
            c.cycles += 2;
            c.swc1(6, avsh_at(c, SP, 0x74));
            c.swc1(5, avsh_at(c, SP, 0x78));
            break 'b_2d6038;
        }
        c.cycles += 150;
        c.lwc1(1, avsh_at(c, SP, 0x74));
        c.g[A0][0] = 0xa;
        c.lwc1(0, avsh_at(c, SP, 0x78));
        c.mult(0, 4, 20, 4);
        c.lwc1(2, avsh_at(c, SP, 0x70));
        c.g[S0][0] = c.g[S7][0];
        c.swc1(1, avsh_at(c, SP, 0x64));
        c.swc1(0, avsh_at(c, SP, 0x68));
        c.swc1(2, avsh_at(c, SP, 0x60));
        c.lwc1(0, avsh_at_sub(c, GP, 0x3d00));
        c.g[V0][0] = c.lw(avsh_at(c, S1, AVSH_DATA));
        c.sw(avsh_at(c, SP, 0x6c), 0u64);
        c.lwc1(1, avsh_at_sub(c, GP, 0x3cfc));
        c.g[V1][0] = c.lw(avsh_at(c, V0, AVSH_D_FRAMES));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[V0][0] = c.lbu(avsh_at(c, A0, 0x9));
        c.mtc1(2, 12);
        c.cvt_s_w(12, 12);
        c.mul_s(12, 12, 0);
        c.mul_s(12, 22, 12);
        c.mul_s(12, 12, 1);
        c.lqc2(1, (c.g[S0][0] as u32));
        c.lqc2(2, avsh_at(c, S0, 0x10));
        c.lqc2(3, avsh_at(c, S0, 0x20));
        c.lqc2(4, avsh_at(c, S0, 0x30));
        c.sqc2(1, (c.g[FP][0] as u32));
        c.sqc2(2, avsh_at(c, FP, 0x10));
        c.sqc2(3, avsh_at(c, FP, 0x20));
        c.sqc2(4, avsh_at(c, FP, 0x30));
        c.g[A0][0] = 0x500000;
        c.lq(2, avsh_at(c, SP, 0xc0));
        c.g[V1][0] = sx(avsh_at_sub(c, A0, 0xed0));
        c.lq(5, (c.g[V1][0] as u32));
        c.g[A0][0] = c.lw(avsh_at(c, SP, 0xf4));
        c.sq(5, avsh_at(c, SP, 0xc0));
        c.g[A1][0] = c.lw(avsh_at(c, SP, 0xf8));
        c.sq(2, avsh_at(c, SP, 0xd0));
        call_ptr(c, 0x0031be50, 0x002d60d0)?;
        c.lwc1(11, avsh_at(c, SP, 0xec));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 5);
        c.lwc1(0, avsh_at(c, SP, 0x60));
        c.sub_s(1, 5, 11);
        c.lwc1(3, avsh_at(c, SP, 0x68));
        c.lwc1(7, avsh_at(c, SP, 0x64));
        c.lwc1(8, avsh_at(c, SP, 0xe8));
        c.mul_s(2, 1, 3);
        c.swc1(5, avsh_at(c, SP, 0x5c));
        c.mul_s(4, 1, 0);
        c.lq(2, avsh_at(c, SP, 0xd0));
        c.mul_s(1, 1, 7);
        c.sw(avsh_at(c, SP, 0x50), 0u64);
        c.mul_s(13, 8, 3);
        c.sw(avsh_at(c, SP, 0x54), 0u64);
        c.mul_s(12, 8, 0);
        c.sw(avsh_at(c, SP, 0x58), 0u64);
        c.mul_s(9, 2, 3);
        c.sq(2, avsh_at(c, SP, 0x50));
        c.mul_s(10, 4, 3);
        c.sw(avsh_at(c, SP, 0x2c), 0u64);
        c.mul_s(5, 2, 0);
        c.sw(avsh_at(c, SP, 0x3c), 0u64);
        c.mul_s(6, 4, 0);
        c.sw(avsh_at(c, SP, 0x4c), 0u64);
        c.mul_s(3, 1, 3);
        c.mul_s(0, 1, 0);
        c.mul_s(8, 8, 7);
        c.mul_s(2, 2, 7);
        c.mul_s(4, 4, 7);
        c.mul_s(1, 1, 7);
        c.add_s(5, 5, 8);
        c.sub_s(0, 0, 13);
        c.sub_s(2, 2, 12);
        c.add_s(9, 9, 11);
        c.swc1(5, avsh_at(c, SP, 0x28));
        c.add_s(6, 6, 11);
        c.swc1(0, avsh_at(c, SP, 0x24));
        c.add_s(4, 4, 13);
        c.swc1(2, avsh_at(c, SP, 0x38));
        c.sub_s(10, 10, 8);
        c.swc1(9, avsh_at(c, SP, 0x48));
        c.add_s(1, 1, 11);
        c.swc1(6, avsh_at(c, SP, 0x20));
        c.add_s(3, 3, 12);
        c.swc1(4, avsh_at(c, SP, 0x30));
        c.swc1(10, avsh_at(c, SP, 0x40));
        c.swc1(1, avsh_at(c, SP, 0x34));
        c.swc1(3, avsh_at(c, SP, 0x44));
        c.lqc2(4, (c.g[S0][0] as u32));
        c.lqc2(5, avsh_at(c, S0, 0x10));
        c.lqc2(6, avsh_at(c, S0, 0x20));
        c.lqc2(7, avsh_at(c, S0, 0x30));
        c.lqc2(8, (c.g[FP][0] as u32));
        c.lqc2(9, avsh_at(c, FP, 0x10));
        c.lqc2(10, avsh_at(c, FP, 0x20));
        c.lqc2(11, avsh_at(c, FP, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.vu0().mula(0b1111, 4, Src::Bc(9, X));
        c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
        c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
        c.vu0().mula(0b1111, 4, Src::Bc(10, X));
        c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
        c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
        c.vu0().mula(0b1111, 4, Src::Bc(11, X));
        c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
        c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
        c.sqc2(12, (c.g[S0][0] as u32));
        c.sqc2(13, avsh_at(c, S0, 0x10));
        c.sqc2(14, avsh_at(c, S0, 0x20));
        c.sqc2(15, avsh_at(c, S0, 0x30));
        c.lq(3, avsh_at(c, SP, 0x10));
        c.g[A0][0] = c.lw(avsh_at(c, SP, 0xf0));
        c.lq(2, (c.g[SP][0] as u32));
        c.sq(3, avsh_at(c, A0, 0x10));
        c.sq(2, (c.g[A0][0] as u32));
        c.g[V0][0] = c.lw(avsh_at(c, SP, 0xfc));
        c.lqc2(1, (c.g[S0][0] as u32));
        c.lqc2(2, avsh_at(c, S0, 0x10));
        c.lqc2(3, avsh_at(c, S0, 0x20));
        c.lqc2(4, avsh_at(c, S0, 0x30));
        c.sqc2(1, (c.g[V0][0] as u32));
        c.sqc2(2, avsh_at(c, V0, 0x10));
        c.sqc2(3, avsh_at(c, V0, 0x20));
        c.sqc2(4, avsh_at(c, V0, 0x30));
        c.g[V0][0] = c.lw(avsh_at(c, SP, 0xf0));
        c.lq(16, avsh_at(c, SP, 0x1a0));
        c.lq(17, avsh_at(c, SP, 0x190));
        c.lq(18, avsh_at(c, SP, 0x180));
        c.lq(19, avsh_at(c, SP, 0x170));
        c.lq(20, avsh_at(c, SP, 0x160));
        c.lq(21, avsh_at(c, SP, 0x150));
        c.lq(22, avsh_at(c, SP, 0x140));
        c.lq(23, avsh_at(c, SP, 0x130));
        c.lq(30, avsh_at(c, SP, 0x120));
        c.g[RA][0] = c.ld(avsh_at(c, SP, 0x110));
        c.lwc1(26, avsh_at(c, SP, 0x1e0));
        c.lwc1(25, avsh_at(c, SP, 0x1d8));
        c.lwc1(24, avsh_at(c, SP, 0x1d0));
        c.lwc1(23, avsh_at(c, SP, 0x1c8));
        c.lwc1(22, avsh_at(c, SP, 0x1c0));
        c.lwc1(21, avsh_at(c, SP, 0x1b8));
        c.lwc1(20, avsh_at(c, SP, 0x1b0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(avsh_at(c, SP, 0x1f0));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{elf_image, fill, fresh, put_vec, run_generated, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this test's fake in the machine's RAM.
    const OC1041_CTRL: u32 = DATA + 0x21_0040;
    const OC1041_MAGIC: u32 = 0x0c10_4141;
    const OC1041_SELF: u32 = DATA + 0x1000;
    const OC1041_OUT: u32 = DATA + 0x2000;
    const OC1041_DATA: u32 = DATA + 0x3000;
    const OC1041_FRAMES: u32 = DATA + 0x4000;

    static OC1041_PREV_SPAWN_AT: AtomicUsize = AtomicUsize::new(0);

    /// `DynamicEmitter_SpawnAt` stand-in: logs every register the caller set
    /// up and returns (or unwinds), unless the gate word is off.
    fn oc1041_fake_spawn_at(c: &mut Cpu) {
        if c.bus.read32(OC1041_CTRL) != OC1041_MAGIC {
            return fakes::fall_back(c, &OC1041_PREV_SPAWN_AT, ee::s12::f_003717c0__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0037_17c0, ra);
    }

    /// A packed history of 3..8 frames, a node at some frame and a sample
    /// time that is the same frame, frame 0, a step back, a step forward or
    /// past the end; emitting or not; zero and non-zero rotation axes.
    fn oc1041_setup(c: &mut Cpu, seed: u32) -> Events {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_003717C0),
            oc1041_fake_spawn_at,
            &OC1041_PREV_SPAWN_AT,
        );
        let mut r = machine(c, seed, 0x2d5778, 4);
        elf_image(c);
        c.bus.write32(OC1041_CTRL, OC1041_MAGIC);
        // cAvalanche_m_UpdateTimeStep = 1/60, PatchColourScale2 = 1.
        c.bus.write32(GAME_GP.wrapping_add(0x974), 0x3c88_8889);
        c.bus.write32(GAME_GP.wrapping_add(0x9b8), 0x3f80_0000);

        let len = 3 + seed % 6;
        let len_f = if seed % 5 == 4 {
            len as f32 + 0.5
        } else {
            len as f32
        };
        // The history.
        c.bus.write32(OC1041_DATA + AVSH_D_LENGTH, len_f.to_bits());
        let life = match seed % 4 {
            0 => 1000.0f32,
            1 => 0.1,
            2 => len as f32,
            _ => f32::from_bits(r.f32(20.0)).abs(),
        };
        c.bus.write32(OC1041_DATA + 0x104, life.to_bits());
        let flat = r.f32(2.0);
        c.bus.write32(OC1041_DATA + 0x108, flat);
        let mode = if seed % 3 == 0 {
            2
        } else {
            (r.next_u32() & 0xffff) as u16
        };
        c.bus.write16(OC1041_DATA + 0xf0, mode);
        let rate = match seed % 5 {
            3 => 4.0f32,
            1 => 0.25,
            _ => 1.0,
        };
        c.bus.write32(OC1041_DATA + AVSH_D_RATE, rate.to_bits());
        put_vec(
            c,
            OC1041_DATA + AVSH_D_START,
            [r.f32(5.0), r.f32(5.0), r.f32(5.0), r.f32(5.0)],
        );
        c.bus.write32(OC1041_DATA + AVSH_D_FRAMES, OC1041_FRAMES);
        fill(c, &mut r, OC1041_FRAMES, 0x40);
        // Frame records are 10 bytes: vel 0..2, colour 3..5, axis 6..8, angle 9.
        for i in 0..len + 3 {
            let rec = OC1041_FRAMES + 10 * i;
            if seed % 4 == 0 || (seed % 7 == 0 && i % 2 == 0) {
                for k in 6..9 {
                    c.bus.write8(rec + k, 0);
                }
            } else if r.next_u32() % 5 == 0 {
                c.bus.write8(rec + 6, 0x40);
            }
        }

        // The node.
        let target = (seed / 2) % (len + 2);
        // The frame the function samples (clamped to the last pair).
        let target_eff = if target + 1 >= len { len - 2 } else { target };
        let frame_now = match (seed / 3) % 5 {
            0 => target_eff,
            1 => 0,
            2 => target_eff + 1,
            3 => target_eff.saturating_sub(1),
            _ => r.next_u32() % (len + 1),
        };
        c.bus.write32(OC1041_SELF + AVSH_FRAME, frame_now);
        c.bus
            .write32(OC1041_SELF + AVSH_EMITTING, (seed % 3 != 1) as u32);
        put_vec(
            c,
            OC1041_SELF + AVSH_POS,
            [r.f32(9.0), r.f32(9.0), r.f32(9.0), r.f32(9.0)],
        );
        for i in 0..16 {
            c.bus.write32(OC1041_SELF + AVSH_MAT + 4 * i, r.f32(4.0));
        }
        c.bus.write32(OC1041_SELF + AVSH_FADE, r.f32(1.0));
        c.bus.write32(OC1041_SELF + AVSH_DATA, OC1041_DATA);
        for i in 0..4 {
            c.bus.write32(OC1041_OUT + 4 * i, 0xdead_0000 + i);
        }

        let frac = [0.0f32, 0.25, 0.75][(seed % 3) as usize];
        c.g[A0][0] = OC1041_OUT as u64;
        c.g[A1][0] = OC1041_SELF as u64;
        let mode = schedule(c, &mut r, seed, 0x002d_5778);
        c.f[12] = ((target as f32 + frac) / 30.0).to_bits();
        if mode != Events::Off {
            c.next_event = c.cycles;
        }
        mode
    }

    /// `Avalanche_SampleHistory` against the generated body: same registers,
    /// cycles, memory and `DynamicEmitter_SpawnAt` call logs (the call and
    /// its unwind through the gated fake), with event delivery in both
    /// replay loops.
    #[test]
    fn avalanche_sample_history_matches() {
        check_events(
            ssx3_ee::s09::f_002d5778__generated,
            avalanche_sample_history,
            240,
            &oc1041_setup,
        );
    }

    /// The fixture reaches the reset, the replay and the emit paths.
    #[test]
    fn avalanche_sample_history_paths() {
        let mut totals = std::collections::BTreeSet::new();
        for seed in 0..240 {
            let mut c = fresh();
            oc1041_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ssx3_ee::s09::f_002d5778__generated);
            totals.insert(c.cycles - before);
        }
        assert!(totals.len() >= 12, "only {} distinct paths", totals.len());
    }
}
