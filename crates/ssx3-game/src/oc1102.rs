//! Staging module oc1102 (wave wave268): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1102`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003c_36a8,
        snd_apply_group_effect_wrapper,
        "SND_ApplyGroupEffect_Wrapper",
    ),
    (0x003c_3728, snd_pan_lookup, "SND_PanLookup"),
    (0x003c_3780, snd_pan_fix_quadrant, "SND_PanFixQuadrant"),
    (0x003c_4978, snd_dma_to_spr, "SND_DmaToSpr"),
    (0x003c_4a38, snd_dma_from_spr, "SND_DmaFromSpr"),
    (
        0x003c_9da0,
        snd_pcm16_to_float_scalar,
        "SND_Pcm16ToFloatScalar",
    ),
    (0x003c_9e50, snd_pcm16_to_float, "SND_Pcm16ToFloat"),
    (
        0x003c_b328,
        snd_dsp_distortion_process,
        "SND_DspDistortion_Process",
    ),
];

/// Direct callees of this module's functions.
const APPLY_GROUP_EFFECT: Callee = Callee {
    addr: 0x003c_35f8,
    f: ee::s13::f_003c35f8,
};
const PCM16_SCALAR: Callee = Callee {
    addr: 0x003c_9da0,
    f: ee::s14::f_003c9da0,
};

/// `void SND_ApplyGroupEffect_Wrapper(int busIndex /*a0*/, int renderKind /*a1*/)`
/// (0x3c36a8): forwards `renderKind` to `SND_ApplyGroupEffect` and returns 0.
pub fn snd_apply_group_effect_wrapper(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        APPLY_GROUP_EFFECT.call(c, 0x003c_36b8)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `void SND_PanLookup(u32 azimuth /*a0*/, float *out /*a1*/)` (0x3c3728): writes
/// the two pan gains of `azimuth` (bits 6..9 select one of 256 entries of
/// `(x, y)` int16 pairs, scaled by 1/32767) to `out[0]` and `out[1]`.
pub fn snd_pan_lookup(c: &mut Cpu) {
    const PAN_TABLE: u32 = 0x0044_dd78;
    c.cycles += 10;
    c.g[A0][0] = sx((c.g[A0][0] as u32) >> 6);
    c.g[V0][0] = sx(PAN_TABLE);
    c.g[A0][0] &= 0x3fc;
    c.g[AT][0] = 0x46ff_fe00; // 32767.0f
    c.mtc1(1, 1);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = 1;
    loop {
        c.cycles += 9;
        c.g[V0][0] = c.lh(c.g[A0][0] as u32);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(2));
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.div_s(0, 0, 1);
        c.swc1(0, c.g[A1][0] as u32);
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003c_3750);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    ret(c);
}

/// `SND_PanFixQuadrant` (0x3c3780): the octant fix-up of a pan vector. Reads the
/// direction `ang` (a0) and the deltas `dx` (a3), `dy` (t0), turns the float
/// pair at `*x` (a1) and `*y` (a2) into the quadrant-fixed signs and axis flip.
/// Transcribed block for block from the generated body (every scratch register
/// and FPU compare is reproduced).
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn snd_pan_fix_quadrant(c: &mut Cpu) {
    'b_3c37f8: {
        'b_3c37c0: {
            'b_3c37a4: {
                c.cycles += 2;
                let t = (c.g[A3][0] as i64) >= 0;

                if t {
                    break 'b_3c37a4;
                } else {
                    c.cycles += 2;
                    let t = (c.g[T0][0] as i64) >= 0;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        break 'b_3c37c0;
                    } else {
                        c.cycles += 5;
                        c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
                        c.g[T1][0] = 0x2;
                        c.g[V0][0] = 0x1;
                        let t = true;
                        if c.g[V1][0] != 0 {
                            c.g[T1][0] = c.g[V0][0];
                        }
                        if t {
                            break 'b_3c37f8;
                        } else {
                            break 'b_3c37a4;
                        }
                    }
                }
            }
            'b_3c37bc: {
                c.cycles += 2;
                let t = (c.g[T0][0] as i64) < 0;
                c.g[T1][0] = 0x1;
                if t {
                    break 'b_3c37bc;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    c.g[V0][0] = 0x2;
                    let t = true;
                    if c.g[V1][0] != 0 {
                        c.g[T1][0] = c.g[V0][0];
                    }
                    if t {
                        break 'b_3c37f8;
                    } else {
                        break 'b_3c37bc;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0xffffffffffffffff;
            break 'b_3c37c0;
        }
        'b_3c37e8: {
            c.cycles += 4;
            c.g[V1][0] = sx(0u32.wrapping_sub((c.g[A3][0] as u32)));
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A3][0] as i64)) as u64;
            let t = (c.g[T0][0] as i64) >= 0;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[A3][0];
            }
            if t {
                break 'b_3c37e8;
            } else {
                c.cycles += 4;
                c.g[V0][0] = sx(0u32.wrapping_sub((c.g[T0][0] as u32)));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[T1][0] = 0x4;
                if t {
                    break 'b_3c37f8;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[T1][0] = 0x3;
                    if t {
                        break 'b_3c37f8;
                    } else {
                        break 'b_3c37e8;
                    }
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[T1][0] = 0x4;
        if t {
            break 'b_3c37f8;
        } else {
            c.cycles += 1;
            c.g[T1][0] = 0x3;
            break 'b_3c37f8;
        }
    }
    'b_3c3888: {
        'b_3c3880: {
            'b_3c3848: {
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4000));
                c.g[V0][0] = 0x7fff;
                c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A0][0] = 0u64;
                if t {
                    break 'b_3c3848;
                } else {
                    'b_3c382c: {
                        c.cycles += 7;
                        c.lwc1(1, (c.g[A1][0] as u32));
                        c.g[V1][0] = 0x4;
                        c.lwc1(0, (c.g[A2][0] as u32));
                        c.c_lt_s(1, 0);
                        let t = !c.fcc();
                        c.mov_s(2, 1);
                        if t {
                            break 'b_3c382c;
                        } else {
                            c.cycles += 1;
                            c.g[V1][0] = 0x3;
                            break 'b_3c382c;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[T1][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x3;
                    if t {
                        break 'b_3c3888;
                    } else {
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A0][0] = 0x2;
                        if t {
                            break 'b_3c3888;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V1][0] = c.g[T1][0] ^ 0x2;
                            if t {
                                break 'b_3c3880;
                            } else {
                                break 'b_3c3848;
                            }
                        }
                    }
                }
            }
            'b_3c3868: {
                c.cycles += 7;
                c.lwc1(1, (c.g[A1][0] as u32));
                c.g[V1][0] = 0x1;
                c.lwc1(0, (c.g[A2][0] as u32));
                c.c_lt_s(1, 0);
                let t = !c.fcc();
                c.mov_s(2, 1);
                if t {
                    break 'b_3c3868;
                } else {
                    c.cycles += 1;
                    c.g[V1][0] = 0x2;
                    break 'b_3c3868;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[T1][0] as i64) < 3i64) as u64;
            let t = c.g[V0][0] != 0u64;

            if t {
                break 'b_3c3888;
            } else {
                c.cycles += 3;
                c.g[A0][0] = 0x2;
                let t = c.g[V1][0] != c.g[A0][0];
                c.g[V1][0] = c.g[T1][0] ^ 0x3;
                if t {
                    break 'b_3c3888;
                } else {
                    break 'b_3c3880;
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 0x1;
        if c.g[V1][0] == 0 {
            c.g[A0][0] = c.g[V0][0];
        }
        break 'b_3c3888;
    }
    'b_3c3898: {
        c.cycles += 2;
        let t = (c.g[A3][0] as i64) >= 0;

        if t {
            break 'b_3c3898;
        } else {
            c.cycles += 2;
            c.neg_s(0, 2);
            c.swc1(0, (c.g[A1][0] as u32));
            break 'b_3c3898;
        }
    }
    'b_3c38ac: {
        c.cycles += 2;
        let t = (c.g[T0][0] as i64) >= 0;
        c.g[V0][0] = 0x1;
        if t {
            break 'b_3c38ac;
        } else {
            c.cycles += 3;
            c.lwc1(0, (c.g[A2][0] as u32));
            c.neg_s(0, 0);
            c.swc1(0, (c.g[A2][0] as u32));
            break 'b_3c38ac;
        }
    }
    c.cycles += 2;
    let t = c.g[A0][0] != c.g[V0][0];
    c.g[V0][0] = 0x2;
    if t {
        'b_3c38d8: {
            c.cycles += 2;
            let t = c.g[A0][0] != c.g[V0][0];

            if t {
                break 'b_3c38d8;
            } else {
                c.cycles += 3;
                c.lwc1(0, (c.g[A2][0] as u32));
                c.neg_s(0, 0);
                c.swc1(0, (c.g[A2][0] as u32));
                break 'b_3c38d8;
            }
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.pc = t;
        return;
    } else {
        c.cycles += 4;
        c.lwc1(0, (c.g[A1][0] as u32));
        c.neg_s(0, 0);
        let t = c.g[RA][0] as u32;
        c.swc1(0, (c.g[A1][0] as u32));
        c.pc = t;
        return;
    }
}

/// Shared body of `SND_DmaToSpr` and `SND_DmaFromSpr`: waits until the channel is
/// idle, programs its memory address, SPR address and quadword count, starts it
/// (STR, bit 8 of CHCR), and waits again when flags bit 16 asks for a
/// synchronous transfer.
struct SprDma {
    /// The channel's CHCR address; MADR, SADR and QWC are at +0x10, +0x80, +0x20.
    chcr: u32,
    /// The memory address is `a0` (else `a1`); the SPR address is the other one.
    madr_from_a0: bool,
    /// Event head of the wait before the start.
    busy_head: u32,
    /// Event head of the wait after the start.
    done_head: u32,
}

fn spr_dma_start(c: &mut Cpu, d: SprDma) {
    // wait while the channel is still busy
    c.cycles += 7;
    c.g[T0][0] = c.g[A0][0];
    if (c.lw(d.chcr) & 0x100) != 0 {
        c.cycles += 1;
        loop {
            c.cycles += 7;
            if (c.lw(d.chcr) & 0x100) != 0 {
                if event_due(c) {
                    deliver_event_here(c, d.busy_head);
                }
                continue;
            }
            break;
        }
    }
    // program and start
    c.cycles += 17;
    let (madr_v, sadr_v) = if d.madr_from_a0 {
        (c.g[T0][0], c.g[A1][0])
    } else {
        (c.g[A1][0], c.g[T0][0])
    };
    c.g[V0][0] = (d.chcr + 0x10) as u64;
    c.sw(d.chcr + 0x10, madr_v);
    c.sw(d.chcr + 0x80, sadr_v);
    c.sw(d.chcr + 0x20, c.g[A2][0]);
    c.g[A0][0] = d.chcr as u64;
    c.g[V1][0] = 0x100;
    c.sw(d.chcr, c.g[V1][0]);
    c.g[V0][0] = c.g[A3][0] & 0x10000;
    // flags bit 16: wait for the transfer to finish
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.g[V0][0] = c.lw(d.chcr) & 0x100;
        let idle = c.g[V0][0] == 0;
        c.g[V1][0] = 0x1000_0000;
        if !idle {
            c.cycles += 1;
            c.g[V1][0] = d.chcr as u64;
            loop {
                c.cycles += 7;
                c.g[V0][0] = c.lw(d.chcr) & 0x100;
                if c.g[V0][0] != 0 {
                    if event_due(c) {
                        deliver_event_here(c, d.done_head);
                    }
                    continue;
                }
                break;
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `void SND_DmaToSpr(u32 memAddr /*a0*/, u32 sprAddr /*a1*/, int qwc /*a2*/,
/// u32 flags /*a3*/)` (0x3c4978): copies `qwc` quadwords from memory to the
/// scratchpad through DMA channel 9 (`0x1000D400`). Flag 0x10000 waits for the
/// transfer to finish.
pub fn snd_dma_to_spr(c: &mut Cpu) {
    spr_dma_start(
        c,
        SprDma {
            chcr: 0x1000_D400,
            madr_from_a0: true,
            busy_head: 0x003c_4998,
            done_head: 0x003c_4a10,
        },
    );
}

/// `void SND_DmaFromSpr(u32 sprAddr /*a0*/, u32 memAddr /*a1*/, int qwc /*a2*/,
/// u32 flags /*a3*/)` (0x3c4a38): copies `qwc` quadwords from the scratchpad to
/// memory through DMA channel 8 (`0x1000D000`). Flag 0x10000 waits for the
/// transfer to finish.
pub fn snd_dma_from_spr(c: &mut Cpu) {
    spr_dma_start(
        c,
        SprDma {
            chcr: 0x1000_D000,
            madr_from_a0: false,
            busy_head: 0x003c_4a58,
            done_head: 0x003c_4ad0,
        },
    );
}

/// `void SND_Pcm16ToFloatScalar(int n /*a0*/, short *src /*a1*/, float *dst /*a2*/)`
/// (0x3c9da0): converts `n` int16 samples to floats. Scalar head until `dst`
/// is 16-byte aligned, then (for `n >= 4`) four samples per iteration.
pub fn snd_pcm16_to_float_scalar(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A3][0] = c.g[A2][0];
    c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
    c.g[A2][0] = c.g[A0][0] & 0x3;
    c.g[T4][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let head = c.g[A2][0] != 0;
    c.g[V1][0] = c.g[A1][0];
    if head {
        c.cycles += 2;
        c.g[A1][0] = c.g[A2][0];
        loop {
            c.cycles += 8;
            c.g[V0][0] = c.lh(c.g[V1][0] as u32);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(2));
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.swc1(0, c.g[A3][0] as u32);
            let more = c.g[A1][0] != 0;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x003c_9dc0);
                }
                continue;
            }
            break;
        }
    }
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A0][0] as i64) < 4) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        c.g[A0][0] = c.g[V1][0];
        c.g[A1][0] = c.g[A3][0];
        c.g[A2][0] = c.g[T4][0];
        loop {
            c.cycles += 19;
            c.g[T0][0] = c.lh(c.g[A0][0] as u32);
            c.g[T1][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x2));
            c.g[T2][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
            c.g[T3][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x6));
            c.mtc1(8, 0);
            c.mtc1(9, 1);
            c.mtc1(10, 2);
            c.mtc1(11, 3);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
            c.cvt_s_w(0, 0);
            c.cvt_s_w(1, 1);
            c.cvt_s_w(2, 2);
            c.cvt_s_w(3, 3);
            let out = c.g[A1][0] as u32;
            c.swc1(0, out.wrapping_sub(0x10));
            c.swc1(1, out.wrapping_sub(0xc));
            c.swc1(2, out.wrapping_sub(0x8));
            c.swc1(3, out.wrapping_sub(0x4));
            let more = c.g[A1][0] != c.g[A2][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x003c_9df8);
                }
                continue;
            }
            break;
        }
    }
    c.cycles += 2;
    ret(c);
}

/// Frame-restore and return tail of `SND_Pcm16ToFloat`.
fn pcm16_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 5;
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// The whole block through `SND_Pcm16ToFloatScalar` (misaligned source or
/// destination): `n`, `src`, `dst` are the saved arguments.
fn pcm16_whole_scalar(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 6;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S1][0];
    PCM16_SCALAR.call(c, 0x003c_9e94)?;
    frame.restore(c, S0, 0x30);
    Ok(())
}

/// `void SND_Pcm16ToFloat(int n /*a0*/, short *src /*a1*/, float *dst /*a2*/)`
/// (0x3c9e50): int16 to float conversion. Misaligned `src` or `dst` (16-byte
/// for `dst`) goes through the scalar routine; otherwise a scalar head for the
/// `n & 15` remainder, then eight-sample VU0 blocks (`itof`).
pub fn snd_pcm16_to_float(c: &mut Cpu) {
    c.cycles += 10;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S2, 0x10);
    frame.save(c, S0, 0x30);
    c.g[S2][0] = c.g[A1][0];
    frame.save(c, S1, 0x20);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[V0][0] = c.g[S2][0] & 7;
    c.g[S1][0] = c.g[A2][0];
    if c.g[V0][0] != 0 {
        if pcm16_whole_scalar(c, frame).is_err() {
            return;
        }
        pcm16_epilogue(c, frame);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[S1][0] & 0xf;
    c.g[A0][0] = c.g[S0][0] & 0xf;
    if c.g[V0][0] != 0 {
        if pcm16_whole_scalar(c, frame).is_err() {
            return;
        }
        pcm16_epilogue(c, frame);
        return;
    }
    // the remainder head
    c.cycles += 2;
    if c.g[A0][0] != 0 {
        c.cycles += 8;
        c.g[S0][0] &= 0x7ff0;
        c.g[A1][0] = sx((c.g[S0][0] as u32) << 1);
        c.g[A2][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        if PCM16_SCALAR.call(c, 0x003c_9ebc).is_err() {
            return;
        }
        if (c.g[S0][0] as i64) <= 0 {
            frame.restore(c, S0, 0x30);
            pcm16_epilogue(c, frame);
            return;
        }
    }
    // the VU0 blocks of eight samples
    c.cycles += 3;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S1][0];
    loop {
        c.cycles += 28;
        c.g[T0][0] = c.ld(c.g[A1][0] as u32);
        c.g[T1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
        c.g[T2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
        c.g[T3][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
        c.pextlh(8, 8, 0);
        c.pextlh(9, 9, 0);
        c.pextlh(10, 10, 0);
        c.pextlh(11, 11, 0);
        c.psraw(8, 8, 16);
        c.psraw(9, 9, 16);
        c.psraw(10, 10, 16);
        c.psraw(11, 11, 16);
        c.qmtc2(8, 1);
        c.qmtc2(9, 2);
        c.qmtc2(10, 3);
        c.qmtc2(11, 4);
        c.vu0().itof(0, 0b1111, 1, 1);
        c.vu0().itof(0, 0b1111, 2, 2);
        c.vu0().itof(0, 0b1111, 3, 3);
        c.vu0().itof(0, 0b1111, 4, 4);
        c.sqc2(1, c.g[A2][0] as u32);
        c.sqc2(2, (c.g[A2][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
        c.sqc2(3, (c.g[A2][0] as u32).wrapping_add(0x20));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
        c.sqc2(4, (c.g[A2][0] as u32).wrapping_add(0x30));
        if (c.g[A0][0] as i64) > 0 {
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x40));
            if event_due(c) {
                deliver_event_here(c, 0x003c_9ed0);
            }
            continue;
        }
        break;
    }
    c.cycles += 1;
    frame.restore(c, S0, 0x30);
    pcm16_epilogue(c, frame);
}

/// `int SND_DspDistortion_Process(Clipper *k /*a0*/, int n /*a1*/, short *src /*a2*/,
/// short *dst /*a3*/)` (0x3cb328): hard-clips four int16 samples at a time between
/// `k->s20` and `k->s30`, scales by `k->s40 / 128`, writes `dst`. First calls the
/// optional hook `k->f8` (its result, when not positive, is returned as is).
/// Transcribed block for block from the generated body: the clip compares,
/// the scratch registers and the multiply order are reproduced.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn snd_dsp_distortion_process(c: &mut Cpu) {
    'b_3cb490: {
        'b_3cb374: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[A2][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S2][0] = c.g[A3][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            let t = c.g[A0][0] == 0u64;
            c.g[S3][0] = c.g[A1][0];
            if t {
                break 'b_3cb374;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                c.g[T0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x1a));
                c.g[A2][0] = c.g[S2][0];
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x003cb36c);
                c.g[A3][0] = c.g[S1][0];
                c.pc = t;
                call_indirect(c, t);
                if c.pc != 0x003cb36c {
                    return;
                }
                if (c.g[V0][0] as i64) <= 0 {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    break 'b_3cb490;
                } else {
                    break 'b_3cb374;
                }
            }
        }
        'b_3cb488: {
            c.cycles += 4;
            c.g[A3][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S2][0];
            let t = (c.g[S3][0] as i64) <= 0;
            c.g[T3][0] = 0u64;
            if t {
                break 'b_3cb488;
            } else {
                c.cycles += 1;
                'l_3cb388: loop {
                    'b_3cb3b0: {
                        'b_3cb3ac: {
                            c.cycles += 5;
                            c.g[A0][0] = c.lh((c.g[A3][0] as u32));
                            c.g[V1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x30));
                            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_3cb3ac;
                            } else {
                                c.cycles += 4;
                                c.g[V1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x20));
                                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[T2][0] = c.g[A0][0];
                                if t {
                                    break 'b_3cb3b0;
                                } else {
                                    break 'b_3cb3ac;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[T2][0] = c.g[V1][0];
                        break 'b_3cb3b0;
                    }
                    'b_3cb3dc: {
                        c.cycles += 6;
                        c.g[A0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x2));
                        c.g[V0][0] = sx((c.g[A1][0] as u32) << 16);
                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                        c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                        let t = c.g[V1][0] != 0u64;
                        c.g[T1][0] = c.g[V0][0];
                        if t {
                            break 'b_3cb3dc;
                        } else {
                            c.cycles += 4;
                            c.g[V1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x20));
                            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[T1][0] = c.g[V1][0];
                            if t {
                                break 'b_3cb3dc;
                            } else {
                                c.cycles += 1;
                                c.g[T1][0] = c.g[A0][0];
                                break 'b_3cb3dc;
                            }
                        }
                    }
                    'b_3cb408: {
                        c.cycles += 6;
                        c.g[A0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = sx((c.g[A1][0] as u32) << 16);
                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                        c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                        let t = c.g[V1][0] != 0u64;
                        c.g[T0][0] = c.g[V0][0];
                        if t {
                            break 'b_3cb408;
                        } else {
                            c.cycles += 4;
                            c.g[V1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x20));
                            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[T0][0] = c.g[V1][0];
                            if t {
                                break 'b_3cb408;
                            } else {
                                c.cycles += 1;
                                c.g[T0][0] = c.g[A0][0];
                                break 'b_3cb408;
                            }
                        }
                    }
                    'b_3cb434: {
                        c.cycles += 6;
                        c.g[V1][0] = sx((c.g[A1][0] as u32) << 16);
                        c.g[A0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x6));
                        c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
                        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A1][0] = c.g[V1][0];
                        if t {
                            break 'b_3cb434;
                        } else {
                            c.cycles += 4;
                            c.g[V1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x20));
                            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A1][0] = c.g[V1][0];
                            if t {
                                break 'b_3cb434;
                            } else {
                                c.cycles += 1;
                                c.g[A1][0] = c.g[A0][0];
                                break 'b_3cb434;
                            }
                        }
                    }
                    c.cycles += 21;
                    c.g[V1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x4));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = ((c.g[T3][0] as i64) < (c.g[S3][0] as i64)) as u64;
                    c.mult(0, 3, 10, 3);
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 7) as u32);
                    c.sh((c.g[A2][0] as u32), c.g[V1][0]);
                    c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.mult(0, 2, 9, 2);
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
                    c.sh((c.g[A2][0] as u32).wrapping_add(0x2), c.g[V0][0]);
                    c.g[V1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.mult(0, 3, 8, 3);
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 7) as u32);
                    c.sh((c.g[A2][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.mult(0, 2, 5, 2);
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
                    c.sh((c.g[A2][0] as u32).wrapping_add(0x6), c.g[V0][0]);
                    let t = c.g[A0][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003cb388);
                        }
                        continue 'l_3cb388;
                    } else {
                        break 'b_3cb488;
                    }
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[S3][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        break 'b_3cb490;
    }
    c.cycles += 6;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = t;
    return;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::{fill, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's SND_ApplyGroupEffect fake (0 = the real callee).
    const OC1102_CTRL: u32 = DATA + 0x36_0000;
    const OC1102_MAGIC: u32 = 0x1102_0c01;
    /// The `a0` of the fake's last call.
    const OC1102_LOG: u32 = OC1102_CTRL + 4;
    static OC1102_PREV_GROUP: AtomicUsize = AtomicUsize::new(0);

    fn oc1102_fake_group(c: &mut Cpu) {
        if c.bus.read32(OC1102_CTRL) != OC1102_MAGIC {
            return fakes::fall_back(c, &OC1102_PREV_GROUP, ee::s13::f_003c35f8__generated);
        }
        let a0 = c.g[A0][0] as u32;
        c.bus.write32(OC1102_LOG, a0);
        c.pc = c.g[RA][0] as u32;
    }

    fn oc1102_hook() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C35F8),
            oc1102_fake_group,
            &OC1102_PREV_GROUP,
        );
    }

    #[test]
    fn snd_apply_group_effect_wrapper_matches() {
        oc1102_hook();
        check(
            ee::s13::f_003c36a8__generated,
            snd_apply_group_effect_wrapper,
            200,
            &|c, seed| {
                let mut r = Rng(0x36a8_0001 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(OC1102_CTRL, OC1102_MAGIC);
                c.g[A0][0] = sx(r.next_u32());
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn snd_pan_lookup_matches() {
        check(
            ee::s13::f_003c3728__generated,
            snd_pan_lookup,
            300,
            &|c, seed| {
                let mut r = Rng(0x3728_0001 ^ seed as u64);
                scramble(c, &mut r);
                // the 256 (x, y) int16 pairs at 0x44dd78, two per word
                for i in 0..256u32 {
                    let w = r.next_u32();
                    c.bus.write32(0x0044_dd78 + i * 4, w);
                }
                c.g[A0][0] = sx(r.next_u32());
                c.g[A1][0] = sx(DATA + 0x40);
            },
        );
    }

    #[test]
    fn snd_pan_fix_quadrant_matches() {
        check(
            ee::s13::f_003c3780__generated,
            snd_pan_fix_quadrant,
            300,
            &|c, seed| {
                let mut r = Rng(0x3780_0001 ^ seed as u64);
                scramble(c, &mut r);
                // both sides of the 0x4000 window of the angle, and every sign of dx/dy
                let ang = if seed % 2 == 0 {
                    0x4000 + r.next_u32() % 0x8000
                } else {
                    r.next_u32()
                };
                c.g[A0][0] = sx(ang);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x104);
                c.bus.write32(DATA + 0x100, r.f32(100.0));
                c.bus.write32(DATA + 0x104, r.f32(100.0));
                let dx = (r.next_u32() % 201) as i32 - 100;
                let dy = (r.next_u32() % 201) as i32 - 100;
                c.g[A3][0] = sx(dx as u32);
                c.g[T0][0] = sx(dy as u32);
            },
        );
    }

    /// Shared setup of the two SPR DMA tests: random memory at `DATA`, the four
    /// arguments, and flags with and without the wait bit.
    fn dma_setup(c: &mut Cpu, seed: u32, mem_arg_a0: bool) {
        let mut r = Rng(0x4978_0001 ^ seed as u64);
        scramble(c, &mut r);
        fill(c, &mut r, DATA, 0x100);
        let mem = sx(DATA + 0x100 * (seed % 4));
        let spr = sx((seed * 0x40) & 0x3ff0);
        if mem_arg_a0 {
            c.g[A0][0] = mem;
            c.g[A1][0] = spr;
        } else {
            c.g[A0][0] = spr;
            c.g[A1][0] = mem;
        }
        c.g[A2][0] = sx(seed % 4);
        c.g[A3][0] = if seed % 2 == 0 { 0x10000 } else { 0 };
    }

    #[test]
    fn snd_dma_to_spr_matches() {
        check(
            ee::s13::f_003c4978__generated,
            snd_dma_to_spr,
            200,
            &|c, seed| {
                dma_setup(c, seed, true);
            },
        );
    }

    #[test]
    fn snd_dma_from_spr_matches() {
        check(
            ee::s13::f_003c4a38__generated,
            snd_dma_from_spr,
            200,
            &|c, seed| {
                dma_setup(c, seed, false);
            },
        );
    }

    #[test]
    fn snd_pcm16_to_float_scalar_matches() {
        check(
            ee::s14::f_003c9da0__generated,
            snd_pcm16_to_float_scalar,
            300,
            &|c, seed| {
                let mut r = Rng(0x9da0_0001 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x3400, 0x80);
                c.g[A0][0] = sx(seed % 21);
                c.g[A1][0] = sx(DATA + 0x3400 + (seed % 2) * 2);
                c.g[A2][0] = sx(DATA + 0x3800 + (seed % 3) * 4);
            },
        );
    }

    #[test]
    fn snd_pcm16_to_float_matches() {
        check(
            ee::s14::f_003c9e50__generated,
            snd_pcm16_to_float,
            300,
            &|c, seed| {
                let mut r = Rng(0x9e50_0001 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x3400, 0x80);
                // misaligned source (every third seed), misaligned destination (every
                // fifth), counts 0..40 cover the scalar head, the remainder and the
                // VU0 blocks
                let src = DATA + 0x3400 + if seed % 3 == 1 { 2 } else { 0 };
                let dst = DATA + 0x3800 + if seed % 5 == 2 { 4 } else { 0 };
                c.g[A0][0] = sx(seed % 40);
                c.g[A1][0] = sx(src);
                c.g[A2][0] = sx(dst);
            },
        );
    }

    #[test]
    fn snd_dsp_distortion_process_matches() {
        check(
            ee::s14::f_003cb328__generated,
            snd_dsp_distortion_process,
            300,
            &|c, seed| {
                let mut r = Rng(0xcb32_0001 ^ seed as u64);
                scramble(c, &mut r);
                let clip = DATA + 0x3000;
                // the clipper: f8 hook at +8 (none, or a leaf that returns 0 at 0x1406c0),
                // the limits at +0x20 and +0x30, the gain at +0x40, b1A at +0x1a
                let hook_obj = DATA + 0x3100;
                c.bus
                    .write32(clip + 8, if seed % 2 == 0 { 0 } else { hook_obj });
                c.bus.write32(hook_obj, 0x0014_06c0);
                c.bus.write32(clip + 0x18, r.next_u32());
                c.bus.write32(clip + 0x20, r.next_u32());
                c.bus.write32(clip + 0x30, r.next_u32());
                c.bus.write32(clip + 0x40, r.next_u32());
                fill(c, &mut r, DATA + 0x3400, 0x40);
                c.g[A0][0] = sx(clip);
                c.g[A1][0] = sx((seed % 9).wrapping_sub(1));
                c.g[A2][0] = sx(DATA + 0x3400);
                c.g[A3][0] = sx(DATA + 0x3800);
            },
        );
    }
}
