//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for animation sampling (`AnimDataBase` channel curves, the
//! `AnimSequence` track clock), world lights, the UV scroll modifier and two
//! libgcc 64-bit division helpers, each with a differential test against the
//! generated body.
//!
//! Two of the originals (`AnimDataBase_EvalChanCurves`,
//! `WorldLightMan_LightContribution`) are `match pc` dispatchers: their loop
//! back edges return on an unwind request and the trampoline later re-enters
//! them at the loop head, so the replacements accept those heads as entry
//! points and keep the loop state in the registers the original uses
//! ([`crate::events`]).

use crate::abi;
use crate::call::{guest, Callee, Flow, Frame, Unwound};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_2c20, anim_data_base_eval_chan_curves, "AnimDataBase_EvalChanCurves"),
    (0x0031_1318, anim_data_base_sample_channel, "AnimDataBase_SampleChannel"),
    (0x0031_35b0, anim_sequence_advance_track, "AnimSequence_AdvanceTrack"),
    (0x0031_4418, anim_seq_list_collect_tracks, "AnimSeqList_CollectTracks"),
    (0x0035_f7d0, uv_scroll_modifier_update, "UVScrollModifier_Update"),
    (0x002f_5d30, world_light_man_light_contribution, "WorldLightMan_LightContribution"),
    (0x0041_1538, udivdi3, "__udivdi3"),
    (0x0042_dad8, umoddi3, "__umoddi3"),
];

// ---------------------------------------------------------------- libgcc

/// `count_leading_zeros` of libgcc's `longlong.h` (the C version: range
/// tests pick a byte, `__clz_tab` gives its bit length):
/// `32 - (bitlen(x >> a) + a)`. Charges the second range-test block (+6
/// above 0xffff, else +4) and the table block (+10); the caller charges the
/// first test. (`__clz_tab` is constant data, the bit length is computed.)
fn clz_tab(c: &mut Cpu, x: u32) -> u32 {
    let a = if x > 0xffff {
        c.cycles += 6;
        if x > 0x00ff_ffff {
            24
        } else {
            16
        }
    } else {
        c.cycles += 4;
        if x < 0x100 {
            0
        } else {
            8
        }
    };
    c.cycles += 10;
    let byte = x >> a;
    32 - ((32 - byte.leading_zeros()) + a)
}

/// One 16-bit digit of `__udiv_qrnnd_c` (`d` normalized, `top < d`):
/// `(top << 16 | part) / d` from the estimate `top / (d >> 16)`, corrected
/// at most twice. Returns the digit and the remainder. `block` = cycles of
/// the block that takes the estimate (the `divu` itself is charged by the
/// caller's block), `retry` = cycles of the second correction.
fn qrnnd_step(c: &mut Cpu, top: u32, part: u32, d: u32, block: u64, retry: u64) -> (u32, u32) {
    c.cycles += block;
    let (mut q, r) = abi::divu(c, top, d >> 16);
    let m = abi::mult(c, q, d & 0xffff);
    let mut r = (r << 16) | part;
    if r < m {
        c.cycles += 4;
        q = q.wrapping_sub(1);
        r = r.wrapping_add(d);
        if r < d {
            // carry out of `r + d`: certainly big enough
            c.cycles += 1;
        } else {
            c.cycles += 3;
            if r < m {
                c.cycles += retry + 1;
                q = q.wrapping_sub(1);
                r = r.wrapping_add(d);
            }
        }
    }
    (q, r.wrapping_sub(m))
}

/// The last digit of a `__udiv_qrnnd_c` whose remainder is not used (the
/// compiler drops it and adds `d` speculatively): 13 cycles, +3 per
/// correction test.
fn qrnnd_step_quotient(c: &mut Cpu, top: u32, part: u32, d: u32) -> u32 {
    c.cycles += 13;
    let (mut q, r) = abi::divu(c, top, d >> 16);
    let m = abi::mult(c, q, d & 0xffff);
    let r = (r << 16) | part;
    if r < m {
        c.cycles += 3;
        q = q.wrapping_sub(1);
        let r = r.wrapping_add(d);
        if r >= d {
            c.cycles += 3;
            if r < m {
                q = q.wrapping_sub(1);
            }
        }
    }
    q
}

/// `u64 __udivdi3(u64 n, u64 d)` (0x411538): libgcc's `__udivmoddi4`
/// without remainder (`UDIV_NEEDS_NORMALIZATION`, 16-bit digit division).
/// Returns the quotient in v0; v1 is left as `q_low << 32`. Division by zero
/// gives what the EE's `divu` by zero gives (`d0 = 1 / 0 = 0xffffffff`).
pub fn udivdi3(c: &mut Cpu) {
    c.cycles += 8;
    let (n, d) = (c.g[A0][0], c.g[A1][0]);
    let (n1, n0, d1, d0) = ((n >> 32) as u32, n as u32, (d >> 32) as u32, d as u32);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    // (high quotient word, low quotient word, cycles of the exit blocks)
    let (q1, q0, exit): (u32, u32, u64) = if d1 == 0 {
        c.cycles += 3;
        if d0 > n1 {
            // 0q = nn / 0D
            c.cycles += 3;
            let bm = clz_tab(c, d0);
            let (mut d0, mut n1, mut n0) = (d0, n1, n0);
            if bm != 0 {
                c.cycles += 5;
                n1 = (n1 << bm) | (n0 >> (32 - bm));
                d0 <<= bm;
                n0 <<= bm;
            }
            c.cycles += 6;
            let (hi, r) = qrnnd_step(c, n1, n0 >> 16, d0, 11, 3);
            c.cycles += 2;
            let lo = qrnnd_step_quotient(c, r, n0 & 0xffff, d0);
            c.cycles += 5;
            (0, hi << 16 | lo, 8)
        } else {
            // qq = NN / 0d
            c.cycles += 2;
            let mut d0 = d0;
            if d0 == 0 {
                // `d0 = 1 / d0`: the intentional division by zero.
                c.cycles += 3 + 6;
                d0 = abi::divu(c, 1, d1).0;
            }
            c.cycles += 2;
            let bm = clz_tab(c, d0);
            let (mut n1, mut n0) = (n1, n0);
            let q1;
            if bm == 0 {
                c.cycles += 5;
                n1 = n1.wrapping_sub(d0);
                q1 = 1;
            } else {
                c.cycles += 13;
                let b = 32 - bm;
                d0 <<= bm;
                let n2 = n1 >> b;
                n1 = (n1 << bm) | (n0 >> b);
                n0 <<= bm;
                let (hi, r) = qrnnd_step(c, n2, n1 >> 16, d0, 12, 3);
                c.cycles += 2;
                let (lo, r) = qrnnd_step(c, r, n1 & 0xffff, d0, 13, 2);
                c.cycles += 4;
                q1 = hi << 16 | lo;
                n1 = r;
            }
            c.cycles += 6;
            let (hi, r) = qrnnd_step(c, n1, n0 >> 16, d0, 11, 2);
            c.cycles += 2;
            let lo = qrnnd_step_quotient(c, r, n0 & 0xffff, d0);
            c.cycles += 5;
            (q1, hi << 16 | lo, 6)
        }
    } else {
        c.cycles += 3;
        if d1 > n1 {
            // 00 = nn / DD
            c.cycles += 2;
            (0, 0, 8)
        } else {
            // 0q = NN / dd
            c.cycles += 3;
            let bm = clz_tab(c, d1);
            if bm == 0 {
                c.cycles += 3;
                let q0 = if n1 > d1 {
                    1
                } else {
                    c.cycles += 3;
                    if n0 < d0 {
                        0
                    } else {
                        c.cycles += 2;
                        1
                    }
                };
                (0, q0, 8)
            } else {
                c.cycles += 15;
                let b = 32 - bm;
                let d1 = (d1 << bm) | (d0 >> b);
                let d0 = d0 << bm;
                let n2 = n1 >> b;
                let n1 = (n1 << bm) | (n0 >> b);
                let n0 = n0 << bm;
                let (hi, r) = qrnnd_step(c, n2, n1 >> 16, d1, 11, 3);
                c.cycles += 2;
                let (lo, n1) = qrnnd_step(c, r, n1 & 0xffff, d1, 13, 2);
                c.cycles += 10;
                let q0 = hi << 16 | lo;
                // umul_ppmm(m1, m0, q0, d0): one too many if q0*d0 > n1:n0
                abi::multu(c, q0, d0);
                let p = q0 as u64 * d0 as u64;
                let (m1, m0) = ((p >> 32) as u32, p as u32);
                if m1 > n1 {
                    (0, q0.wrapping_sub(1), 8)
                } else {
                    c.cycles += 2;
                    if m1 != n1 {
                        (0, q0, 6)
                    } else {
                        c.cycles += 3;
                        if m0 > n0 {
                            c.cycles += 1;
                            (0, q0.wrapping_sub(1), 8)
                        } else {
                            (0, q0, 5)
                        }
                    }
                }
            }
        }
    };
    c.cycles += exit;
    c.g[V1][0] = (q0 as u64) << 32;
    c.g[V0][0] = (q1 as u64) << 32 | q0 as u64;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    abi::ret(c);
}

/// A remainder-only digit step of `__umoddi3`'s inlined `udiv_qrnnd` (the
/// high digit of the `n2:n1 / d0` division): estimate block of 11 cycles,
/// corrections +4, then +3 and +1 (+1 more without a second correction).
/// Returns `r - m`. The `divu` is charged by the caller's block.
fn mod_step_first(c: &mut Cpu, top: u32, part: u32, d: u32) -> u32 {
    c.cycles += 11;
    let (q, r) = abi::divu(c, top, d >> 16);
    let m = abi::mult(c, q, d & 0xffff);
    let mut r = (r << 16) | part;
    if r < m {
        c.cycles += 4;
        r = r.wrapping_add(d);
        if r >= d {
            c.cycles += 3;
            if r < m {
                r = r.wrapping_add(d);
            } else {
                c.cycles += 1;
            }
            c.cycles += 1;
        }
    }
    r.wrapping_sub(m)
}

/// The other remainder-only digit steps of `__umoddi3`: estimate block of
/// 10 cycles, corrections +4, then +3 +1. Returns `r - m`.
fn mod_step(c: &mut Cpu, top: u32, part: u32, d: u32) -> u32 {
    c.cycles += 10;
    let (q, r) = abi::divu(c, top, d >> 16);
    let m = abi::mult(c, q, d & 0xffff);
    let mut r = (r << 16) | part;
    if r < m {
        c.cycles += 4;
        r = r.wrapping_add(d);
        if r >= d {
            c.cycles += 3 + 1;
            if r < m {
                r = r.wrapping_add(d);
            }
        }
    }
    r.wrapping_sub(m)
}

/// The last digit step of `__umoddi3`'s `d1 == 0` path: estimate block of
/// 10 cycles, corrections +4 then +2. Returns `(r - m, r)`; the corrected
/// `r` stays in v1.
fn mod_step_last(c: &mut Cpu, top: u32, part: u32, d: u32) -> (u32, u32) {
    c.cycles += 10;
    let (q, r) = abi::divu(c, top, d >> 16);
    let m = abi::mult(c, q, d & 0xffff);
    let mut r = (r << 16) | part;
    if r < m {
        c.cycles += 4;
        r = r.wrapping_add(d);
        if r >= d {
            c.cycles += 2;
            if r < m {
                r = r.wrapping_add(d);
            }
        }
    }
    (r.wrapping_sub(m), r)
}

/// `u64 __umoddi3(u64 n, u64 d)` (0x42dad8): libgcc's `__udivmoddi4` for the
/// remainder only. The remainder goes through a stack slot (`rp = sp`), so
/// it is also left at `sp - 0x30`; v1 keeps a path-dependent intermediate
/// (the last partial remainder, `n1 << 32`, or `n1 - m1`).
pub fn umoddi3(c: &mut Cpu) {
    c.cycles += 9;
    let (n, d) = (c.g[A0][0], c.g[A1][0]);
    let (n1, n0, d1, d0) = ((n >> 32) as u32, n as u32, (d >> 32) as u32, d as u32);
    let sp = (c.g[SP][0] as u32).wrapping_sub(0x30);
    c.g[SP][0] = sx(sp);
    // (remainder, v1 left behind, cycles of the store blocks)
    let (rem, v1, tail): (u64, u64, u64) = if d1 == 0 {
        c.cycles += 3;
        let (bm, d0, n1, n0) = if d0 > n1 {
            // 0q = nn / 0D
            c.cycles += 3;
            let bm = clz_tab(c, d0);
            let (mut d0, mut n1, mut n0) = (d0, n1, n0);
            if bm != 0 {
                c.cycles += 5;
                n1 = (n1 << bm) | (n0 >> (32 - bm));
                d0 <<= bm;
                n0 <<= bm;
            }
            c.cycles += 4;
            (bm, d0, n1, n0)
        } else {
            // qq = NN / 0d: reduce n1 first
            c.cycles += 2;
            let mut d0 = d0;
            if d0 == 0 {
                // `d0 = 1 / d0`: the intentional division by zero.
                c.cycles += 3 + 5;
                d0 = abi::divu(c, 1, d1).0;
            }
            c.cycles += 2;
            let bm = clz_tab(c, d0);
            let (mut n1, mut n0) = (n1, n0);
            if bm == 0 {
                c.cycles += 4;
                n1 = n1.wrapping_sub(d0);
            } else {
                c.cycles += 13;
                let b = 32 - bm;
                d0 <<= bm;
                let n2 = n1 >> b;
                n1 = (n1 << bm) | (n0 >> b);
                n0 <<= bm;
                let r = mod_step_first(c, n2, n1 >> 16, d0);
                c.cycles += 4;
                n1 = mod_step(c, r, n1 & 0xffff, d0);
            }
            c.cycles += 3;
            (bm, d0, n1, n0)
        };
        c.cycles += 3;
        let r = mod_step(c, n1, n0 >> 16, d0);
        c.cycles += 4;
        let (r0, last) = mod_step_last(c, r, n0 & 0xffff, d0);
        c.cycles += 2 + 4;
        ((r0 >> bm) as u64, sx(last), 1 + 3)
    } else {
        c.cycles += 3;
        if n1 < d1 {
            // 00 = nn / DD: the remainder is n
            c.cycles += 5;
            (n, (n1 as u64) << 32, 3)
        } else {
            c.cycles += 4;
            let bm = clz_tab(c, d1);
            if bm == 0 {
                c.cycles += 3;
                let (mut n1, mut n0) = (n1, n0);
                let sub = if n1 > d1 {
                    true
                } else {
                    c.cycles += 3;
                    n0 >= d0
                };
                if sub {
                    c.cycles += 4;
                    let low = n0.wrapping_sub(d0);
                    n1 = n1.wrapping_sub(d1).wrapping_sub((n0 < low) as u32);
                    n0 = low;
                }
                c.cycles += 2 + 4;
                ((n1 as u64) << 32 | n0 as u64, (n1 as u64) << 32, 1 + 3)
            } else {
                c.cycles += 15;
                let b = 32 - bm;
                let d1 = (d1 << bm) | (d0 >> b);
                let d0 = d0 << bm;
                let n2 = n1 >> b;
                let n1 = (n1 << bm) | (n0 >> b);
                let n0 = n0 << bm;
                let (hi, r) = qrnnd_step(c, n2, n1 >> 16, d1, 11, 3);
                c.cycles += 2;
                let (lo, n1) = qrnnd_step(c, r, n1 & 0xffff, d1, 13, 2);
                c.cycles += 11;
                let q0 = hi << 16 | lo;
                abi::multu(c, q0, d0);
                let p = q0 as u64 * d0 as u64;
                let (mut m1, mut m0) = ((p >> 32) as u32, p as u32);
                let too_big = if m1 > n1 {
                    true
                } else {
                    c.cycles += 2;
                    if m1 != n1 {
                        false
                    } else {
                        c.cycles += 2;
                        if m0 > n0 {
                            c.cycles += 1;
                            true
                        } else {
                            false
                        }
                    }
                };
                if too_big {
                    // sub_ddmmss(m1, m0, m1, m0, d1, d0)
                    c.cycles += 4;
                    let low = m0.wrapping_sub(d0);
                    m1 = m1.wrapping_sub(d1).wrapping_sub((m0 < low) as u32);
                    m0 = low;
                }
                c.cycles += 2 + 12;
                let low = n0.wrapping_sub(m0);
                let high = n1.wrapping_sub(m1).wrapping_sub((n0 < low) as u32);
                let rem = ((high >> bm) as u64) << 32 | ((high << b) | (low >> bm)) as u64;
                (rem, sx(n1.wrapping_sub(m1)), 1 + 3)
            }
        }
    };
    c.cycles += tail;
    Addr(sp).sd(c, rem);
    c.g[V0][0] = Addr(sp).ld(c);
    c.g[V1][0] = v1;
    c.g[SP][0] = sx(sp.wrapping_add(0x30));
    abi::ret(c);
}

// ---------------------------------------------------------------- AnimSequence

/// Direct-call targets of the animation functions (shard s10).
mod anim_callee {
    use super::Callee;
    use ssx3_ee::s10 as ee;

    pub const FIRE_TRIGGERS_AT_TIME: Callee =
        Callee { addr: 0x0031_3938, f: ee::AnimSequence_FireTriggersAtTime };
    pub const FIRE_END_TRIGGERS: Callee =
        Callee { addr: 0x0031_39a8, f: ee::AnimSequence_FireEndTriggers };
    pub const FIRE_TRIGGERS_IN_RANGE: Callee =
        Callee { addr: 0x0031_3868, f: ee::AnimSequence_FireTriggersInRange };
    pub const GET_TRACK_BLEND_WEIGHT: Callee =
        Callee { addr: 0x0031_3d70, f: ee::AnimSequence_GetTrackBlendWeight };
    pub const ALLOC_CHAN_CACHE: Callee =
        Callee { addr: 0x0031_0f18, f: ee::AnimDataBase_AllocChanCache };
    pub const DECODE_CHAN_CACHE: Callee =
        Callee { addr: 0x0031_30a8, f: ee::AnimDataBase_DecodeChanCache };
    pub const EVAL_CHAN_CURVES: Callee =
        Callee { addr: 0x0031_2c20, f: ee::AnimDataBase_EvalChanCurves };
}

/// `AnimSequence` layout used here (tracks of 0x1c bytes from +0).
mod seq {
    pub const TRACK_SIZE: u32 = 0x1c;
    /// Track fields: anim id, time, rate, length, loop mode (+0x1c, the
    /// word after the track's active flag).
    pub const T_ANIM: u32 = 0x4;
    pub const T_TIME: u32 = 0x8;
    pub const T_RATE: u32 = 0xc;
    pub const T_LENGTH: u32 = 0x10;
    pub const T_ACTIVE: u32 = 0x18;
    pub const T_MODE: u32 = 0x1c;
    /// Sequence fields.
    pub const BLEND_A: u32 = 0x60;
    pub const BLEND_B: u32 = 0x70;
    pub const LAYER: u32 = 0x80;
    pub const FLAGS: u32 = 0x84;
    pub const USER: u32 = 0x88;
    pub const RATE: u32 = 0x90;
    pub const FIRED_MASK: u32 = 0xb8;
    pub const END_FLAG: u32 = 0xc0;
    pub const DIRTY: u32 = 0xc4;
    pub const NEXT: u32 = 0xc8;
}

/// `int AnimSeqList_CollectTracks(AnimSeqList* list, TrackSample* out)`
/// (0x314418): for every sequence in the list (`+4` first, linked by
/// `+0xc8`) and each of its three tracks that is active, append a 0x50-byte
/// sample: anim id, time, the sequence's layer word (+0x84 → +8), the track
/// blend weight (`AnimSequence_GetTrackBlendWeight`, +0xc), the sequence's
/// user doubleword (+0x88 → +0x10), its two blend quadwords (+0x60/+0x70 →
/// +0x20/+0x30) and +0x80 (→ +0x40). Returns the number of samples.
///
/// The loop state lives in s0..s7 as in the original (event checks at
/// 0x314468 per track and 0x314458 per sequence).
pub fn anim_seq_list_collect_tracks(c: &mut Cpu) {
    const TRACK_HEAD: u32 = 0x0031_4468;
    const SEQ_HEAD: u32 = 0x0031_4458;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S6, 0x20);
        frame.save(c, S7, 0x10);
        frame.save(c, S0, 0x80);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x70);
        // s6 = count, s7 = out, s1 = sequence, s5 = out offset of the
        // sequence's first sample, s4 = track, s2/s3 = the track's time /
        // active field, s0 = next sample.
        c.g[S6][0] = 0;
        c.g[S7][0] = c.g[A1][0];
        let first = Addr(abi::arg(c, 0)).off(4).lw(c);
        c.g[S1][0] = first;
        if first != 0 {
            c.cycles += 2;
            c.g[S5][0] = 0;
            c.g[S4][0] = 0;
            loop {
                c.cycles += 4;
                let s = c.g[S1][0] as u32;
                c.g[S2][0] = sx(s.wrapping_add(seq::T_TIME));
                c.g[S3][0] = sx(s.wrapping_add(seq::T_ACTIVE));
                c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S7][0] as u32));
                loop {
                    c.cycles += 3;
                    let s = c.g[S1][0] as u32;
                    let active = Addr(c.g[S3][0] as u32).lw(c);
                    c.g[V0][0] = active;
                    c.g[A0][0] = c.g[S1][0];
                    if active != 0 {
                        c.cycles += 20;
                        let time_at = Addr(c.g[S2][0] as u32);
                        let out = Addr(c.g[S0][0] as u32);
                        let anim = time_at.off(0u32.wrapping_sub(seq::T_TIME - seq::T_ANIM)).lw(c);
                        c.g[V0][0] = anim;
                        c.g[A1][0] = c.g[S4][0];
                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x50));
                        out.sw(c, anim);
                        c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(1));
                        let time = time_at.read_fpu(c);
                        c.f[0] = time.0;
                        out.off(4).write_fpu(c, time);
                        c.lq(V0, s.wrapping_add(seq::BLEND_A));
                        c.sq(V0, out.0.wrapping_add(0x20));
                        c.lq(V1, s.wrapping_add(seq::BLEND_B));
                        c.sq(V1, out.0.wrapping_add(0x30));
                        let layer = Addr(s).off(seq::LAYER).lw(c);
                        c.g[V0][0] = layer;
                        out.off(0x40).sw(c, layer);
                        let flags = Addr(s).off(seq::FLAGS).lw(c);
                        c.g[V1][0] = flags;
                        out.off(8).sw(c, flags);
                        anim_callee::GET_TRACK_BLEND_WEIGHT.call(c, 0x0031_44b4)?;
                        let weight = FpuF32(c.f[0]);
                        out.off(0xc).write_fpu(c, weight);
                        let user = Addr(s).off(seq::USER).ld(c);
                        c.g[V0][0] = user;
                        out.off(0x10).sd(c, user);
                        c.g[S0][0] = sx(out.0.wrapping_add(0x50));
                    }
                    c.cycles += 5;
                    let track = (c.g[S4][0] as u32).wrapping_add(1);
                    c.g[S4][0] = sx(track);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(seq::TRACK_SIZE));
                    let more = (c.g[S4][0] as i64) < 3;
                    c.g[V0][0] = more as u64;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(seq::TRACK_SIZE));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, TRACK_HEAD);
                    }
                }
                c.cycles += 3;
                let next = Addr(c.g[S1][0] as u32).off(seq::NEXT).lw(c);
                c.g[S1][0] = next;
                c.g[S4][0] = 0;
                if next == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, SEQ_HEAD);
                }
            }
        }
        c.cycles += 12;
        c.g[V0][0] = c.g[S6][0];
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AnimSequence_AdvanceTrack(AnimSequence* seq, int track, float dt)`
/// (0x3135b0): move a track's time by `dt * track rate * sequence rate` and
/// fire the animation triggers passed over. Track 0 first clears the fired
/// masks and, if the sequence is dirty, fires the triggers at the old time.
/// A track with loop mode 0 clamps to `[0, length]` (firing the end triggers
/// when it hits an end); a looping track wraps by whole lengths, fires the
/// end triggers and the triggers of every range crossed (forward or
/// backward, at most one full extra loop). Only track 0 fires triggers.
/// Stores the new time; returns 0, with v1 the track.
pub fn anim_sequence_advance_track(c: &mut Cpu) {
    /// `AnimSequence_FireTriggersInRange(seq, lo, hi)` returning to `ret`.
    fn fire_range(c: &mut Cpu, lo: FpuF32, hi: FpuF32, ret: u32) -> Flow {
        c.g[A0][0] = c.g[S0][0];
        c.f[12] = lo.0;
        c.f[13] = hi.0;
        anim_callee::FIRE_TRIGGERS_IN_RANGE.call(c, ret)
    }
    /// `AnimSequence_FireEndTriggers(seq)` returning to `ret`.
    fn fire_end(c: &mut Cpu, ret: u32) -> Flow {
        c.g[A0][0] = c.g[S0][0];
        anim_callee::FIRE_END_TRIGGERS.call(c, ret)
    }
    const ZERO: FpuF32 = FpuF32::ZERO;
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save_fpr(c, 20, 0x40);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 23, 0x58);
        frame.save_fpr(c, 22, 0x50);
        frame.save_fpr(c, 21, 0x48);
        // s0 = seq, s2 = track index (the whole register: the tests below
        // compare all 64 bits)
        c.g[S2][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        let s = Addr(abi::arg(c, 0));
        let index = c.g[S2][0] as u32;
        let first_track = c.g[S2][0] == 0;
        let dt = FpuF32(c.f[12]);
        let track = s.off(abi::mult(c, index, seq::TRACK_SIZE));
        abi::set_word(c, V1, track.0);
        c.g[A0][0] = c.g[V1][0];
        c.g[V0][0] = c.g[V1][0];
        let seq_rate = s.off(seq::RATE).read_fpu(c);
        let rate = track.off(seq::T_RATE).read_fpu(c);
        let old_time = track.off(seq::T_TIME).read_fpu(c);
        let length = track.off(seq::T_LENGTH).read_fpu(c);
        let step = rate * seq_rate * dt;
        let mut time = old_time + step;
        c.f[0] = step.0;
        c.f[1] = seq_rate.0;
        c.f[20] = time.0;
        c.f[21] = length.0;
        c.f[22] = old_time.0;
        if first_track {
            c.cycles += 4;
            let dirty = s.off(seq::DIRTY).lw(c);
            c.g[V0][0] = dirty;
            s.off(seq::FIRED_MASK).sd(c, 0);
            s.off(seq::END_FLAG).sw(c, 0);
            if dirty != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                c.f[12] = old_time.0;
                anim_callee::FIRE_TRIGGERS_AT_TIME.call(c, 0x0031_362c)?;
                s.off(seq::DIRTY).sw(c, 0);
            }
        }
        c.cycles += 6;
        c.g[V0][0] = seq::TRACK_SIZE as u64;
        let track = s.off(abi::mult(c, index, seq::TRACK_SIZE));
        abi::set_word(c, V0, track.0);
        let mode = track.off(seq::T_MODE).lw(c);
        c.g[V1][0] = mode;
        // Every path that fires a range ends with `FireTriggersInRange(lo,
        // time)` returning to 0x3137a4 (`fire_to_time`), or with one other
        // range call; then v1 = 0x1c and the epilogue.
        'out: {
            if mode == 0 {
                // clamp to [0, length]
                c.cycles += 4;
                c.f[0] = 0;
                let at_start = time.le(ZERO);
                abi::set_fcc(c, at_start);
                let hit = if at_start {
                    c.cycles += 2;
                    Some((ZERO, 0x0031_3668, 4))
                } else {
                    c.cycles += 3;
                    let at_end = length.le(time);
                    abi::set_fcc(c, at_end);
                    if at_end {
                        c.cycles += 2;
                        Some((length, 0x0031_368c, 2))
                    } else {
                        None
                    }
                };
                if let Some((clamped, ret, cycles)) = hit {
                    time = clamped;
                    c.f[20] = time.0;
                    if !first_track {
                        c.cycles += 1;
                        c.g[V1][0] = seq::TRACK_SIZE as u64;
                        break 'out;
                    }
                    c.cycles += cycles;
                    fire_end(c, ret)?;
                }
                c.cycles += 2;
                c.g[V1][0] = seq::TRACK_SIZE as u64;
                if !first_track {
                    break 'out;
                }
                c.cycles += 2;
                fire_to_time(c, old_time, time)?;
            } else {
                // loop: wrap when outside [0, length)
                c.cycles += 4;
                c.f[23] = 0;
                let before = time.lt(ZERO);
                abi::set_fcc(c, before);
                let wrap = before || {
                    c.cycles += 3;
                    let past = length.le(time);
                    abi::set_fcc(c, past);
                    past
                };
                if !wrap {
                    c.cycles += 2;
                    c.g[V1][0] = seq::TRACK_SIZE as u64;
                    if !first_track {
                        break 'out;
                    }
                    c.cycles += 4;
                    fire_range(c, old_time, time, 0x0031_37c4)?;
                    c.cycles += 1;
                    c.g[V1][0] = seq::TRACK_SIZE as u64;
                    break 'out;
                }
                // loops = floor(time / length)
                c.cycles += 6;
                let ratio = time.div(length);
                let whole = FpuF32::from_i32(ratio.to_i32());
                c.f[0] = ratio.0;
                c.f[1] = whole.0;
                let negative = ratio.lt(whole);
                abi::set_fcc(c, negative);
                let floor = if negative {
                    c.cycles += 3;
                    c.g[AT][0] = 0x3f80_0000;
                    c.f[0] = FpuF32::ONE.0;
                    whole - FpuF32::ONE
                } else {
                    whole
                };
                c.f[1] = floor.0;
                c.cycles += 7;
                let loops = floor.to_i32();
                c.g[S1][0] = sx(loops as u32);
                let back = length * FpuF32::from_i32(loops);
                c.f[0] = back.0;
                time = time - back;
                c.f[20] = time.0;
                if !first_track {
                    c.cycles += 1;
                    c.g[V1][0] = seq::TRACK_SIZE as u64;
                    break 'out;
                }
                c.cycles += 5;
                fire_end(c, 0x0031_3700)?;
                let loops = c.g[S1][0] as i64;
                // the triggers between the old time and the end (or start)
                // of the clip, then those of whole loops and the new part
                let (lo, hi) = if loops >= 2 {
                    c.g[V0][0] = (loops < -1) as u64;
                    c.cycles += 3;
                    (ZERO, length)
                } else {
                    c.g[V0][0] = (loops < -1) as u64;
                    c.cycles += 2;
                    if loops < -1 {
                        c.cycles += 1;
                        (length, ZERO)
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = u64::MAX;
                        if loops == 1 {
                            c.cycles += 4;
                            let forward = old_time.le(time);
                            abi::set_fcc(c, forward);
                            if forward {
                                c.cycles += 2;
                                (ZERO, length)
                            } else {
                                c.cycles += 5;
                                fire_range(c, old_time, length, 0x0031_3750)?;
                                fire_to_time(c, ZERO, time)?;
                                break 'out;
                            }
                        } else {
                            c.cycles += 2;
                            c.g[V1][0] = seq::TRACK_SIZE as u64;
                            if loops != -1 {
                                break 'out;
                            }
                            c.cycles += 4;
                            let backward = time.le(old_time);
                            abi::set_fcc(c, backward);
                            if backward {
                                c.cycles += 1;
                                (length, ZERO)
                            } else {
                                c.cycles += 5;
                                fire_range(c, old_time, ZERO, 0x0031_3794)?;
                                fire_to_time(c, length, time)?;
                                break 'out;
                            }
                        }
                    }
                };
                c.cycles += 4;
                fire_range(c, lo, hi, 0x0031_377c)?;
                c.g[V1][0] = seq::TRACK_SIZE as u64;
            }
        }
        // epilogue: store the new time
        c.cycles += 14;
        frame.restore(c, S1, 0x20);
        let (index, size) = (c.g[S2][0] as u32, c.g[V1][0] as u32);
        let off = abi::mult(c, index, size);
        abi::set_word(c, A0, off);
        frame.restore_ra(c, 0);
        frame.restore(c, S2, 0x10);
        c.g[V0][0] = 0;
        frame.restore_fpr(c, 23, 0x58);
        frame.restore_fpr(c, 22, 0x50);
        frame.restore_fpr(c, 21, 0x48);
        let track = (c.g[S0][0] as u32).wrapping_add(off);
        abi::set_word(c, V1, track);
        let new_time = c.f[20];
        Addr(track).off(seq::T_TIME).write_u32(c, new_time);
        frame.restore(c, S0, 0x30);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    });

    /// The final `FireTriggersInRange(lo, time)` (5 cycles, returning to
    /// 0x3137a4), then v1 = 0x1c.
    fn fire_to_time(c: &mut Cpu, lo: FpuF32, time: FpuF32) -> Flow {
        c.cycles += 5;
        fire_range(c, lo, time, 0x0031_37a4)?;
        c.g[V1][0] = seq::TRACK_SIZE as u64;
        Ok(())
    }
}

// ---------------------------------------------------------------- AnimDataBase

/// `AnimDataBase` fields and the global channel caches.
mod adb {
    /// Clip table (0x14 bytes per clip: +4 first chunk, +0xa chunk count,
    /// +0xc frame count, +0xe frames per chunk).
    pub const CLIPS: u32 = 0x8;
    /// Chunk descriptors (8 bytes: +0 data offset, +4 cache bank, +5 cache
    /// slot or 0xff, +6 curve count).
    pub const CHUNKS: u32 = 0xc;
    /// Compressed channel data.
    pub const DATA: u32 = 0x14;
    /// Per cache bank: the entry pool and the LRU list head (entries of 0x24
    /// bytes: +0 owner descriptor, +4 decoded channel, +0x1c prev, +0x20
    /// next).
    pub const POOL: u32 = 0x004f_e7a0;
    pub const HEAD: u32 = 0x004f_e860;
    pub const ENTRY_SIZE: u32 = 0x24;
    pub const E_PREV: u32 = 0x1c;
    pub const E_NEXT: u32 = 0x20;
}

/// `void AnimDataBase_SampleChannel(AnimDataBase* db, float* out,
/// AnimRef* anim, int channel, int firstCurve, int maxCurves)` (0x311318):
/// sample one channel of the anim (`anim+0`: clip index << 8, `anim+4`:
/// time in seconds) at 30 frames per second, clamped to the last frame.
/// The clip is stored in chunks; the chunk's channel is taken from the
/// decode cache (moving its entry to the most-recently-used end) or decoded
/// into the least-recently-used entry, then its curves `firstCurve..` (at
/// most `maxCurves` and the chunk's curve count) are evaluated into `out` by
/// `AnimDataBase_EvalChanCurves` at the frame within the chunk.
pub fn anim_data_base_sample_channel(c: &mut Cpu) {
    const FPS: FpuF32 = FpuF32(0x41f0_0000); // 30.0
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 0x14;
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        frame.save_fpr(c, 20, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save(c, S4, 0x30);
        frame.save_ra(c, 0);
        // s2 = db, s3 = curve limit, s5 = first curve, s6 = out, s1 = chunk
        // descriptor, f20 = frame
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[T1][0];
        c.g[S5][0] = c.g[T0][0];
        c.g[S6][0] = c.g[A1][0];
        let db = Addr(abi::arg(c, 0));
        let anim = Addr(abi::arg(c, 2));
        let channel = abi::arg(c, 3);
        c.g[AT][0] = FPS.0 as u64;
        let anim_word = anim.lw(c) as u32;
        let clips = db.off(adb::CLIPS).lw(c) as u32;
        let time = anim.off(4).read_fpu(c);
        c.f[1] = time.0;
        let clip = Addr(clips.wrapping_add(abi::mult(c, anim_word >> 8, 0x14)));
        let mut frame_f = time * FPS;
        let last = (clip.off(0xc).lhu(c) as u32).wrapping_sub(1);
        let last_f = FpuF32::from_i32(last as i32);
        let past_end = last_f.le(frame_f);
        abi::set_fcc(c, past_end);
        // (chunk count, chunk, frames per chunk)
        let (chunks, chunk, chunk_len) = if !past_end {
            c.cycles += 8;
            let len = clip.off(0xe).lhu(c) as u32;
            let whole = frame_f.to_i32() as u32;
            let (q, _) = abi::div(c, whole, len.wrapping_sub(1));
            c.cycles += 3;
            (clip.off(0xa).lhu(c) as u32, q, len)
        } else {
            c.cycles += 6;
            let chunks = clip.off(0xa).lhu(c) as u32;
            frame_f = last_f;
            (chunks, chunks.wrapping_sub(1), clip.off(0xe).lhu(c) as u32)
        };
        c.cycles += 24;
        let row = abi::mult(c, channel, chunks);
        let first_chunk = clip.off(4).lw(c) as u32;
        let table = db.off(adb::CHUNKS).lw(c) as u32;
        let start = abi::mult(c, chunk, chunk_len.wrapping_sub(1));
        let desc = Addr(
            table
                .wrapping_add(first_chunk << 3)
                .wrapping_add(row << 3)
                .wrapping_add(chunk << 3),
        );
        c.g[S1][0] = sx(desc.0);
        let data = db.off(adb::DATA).lw(c) as u32;
        let start_f = FpuF32::from_i32(start as i32);
        c.f[0] = start_f.0;
        let src = data.wrapping_add(desc.lw(c) as u32);
        let slot = desc.off(5).lbu(c);
        let bank = desc.off(4).lbu(c) as u32;
        let curves = Addr(src).lbu(c);
        frame_f = frame_f - start_f;
        c.f[20] = frame_f.0;
        if (curves as i64) < (c.g[S3][0] as i64) {
            c.g[S3][0] = curves;
        }
        let head_slot = Addr(adb::HEAD.wrapping_add(bank << 2));
        if slot != 0xff {
            // cached: move the entry to the most-recently-used end (just
            // before the head), unless it is there already
            c.cycles += 14;
            let off = abi::mult(c, slot as u32, adb::ENTRY_SIZE);
            let pool = Addr(adb::POOL.wrapping_add(bank << 2)).lw(c) as u32;
            let head = head_slot.lw(c) as u32;
            let entry = Addr(pool.wrapping_add(off));
            if entry.0 == head {
                c.cycles += 3;
                let next = entry.off(adb::E_NEXT).lw(c);
                head_slot.sw(c, next);
                c.cycles += 1;
            } else {
                let tail = Addr(head).off(adb::E_PREV).lw(c) as u32;
                c.cycles += 2;
                if tail != entry.0 {
                    c.cycles += 16;
                    let prev = entry.off(adb::E_PREV).lw(c);
                    let next = entry.off(adb::E_NEXT).lw(c);
                    Addr(prev as u32).off(adb::E_NEXT).sw(c, next);
                    let prev = entry.off(adb::E_PREV).lw(c);
                    let next = entry.off(adb::E_NEXT).lw(c);
                    Addr(next as u32).off(adb::E_PREV).sw(c, prev);
                    let head = head_slot.lw(c);
                    let tail = Addr(head as u32).off(adb::E_PREV).lw(c);
                    entry.off(adb::E_PREV).sw(c, tail);
                    let head = head_slot.lw(c);
                    entry.off(adb::E_NEXT).sw(c, head);
                    let head = head_slot.lw(c);
                    let tail = Addr(head as u32).off(adb::E_PREV).lw(c);
                    Addr(tail as u32).off(adb::E_NEXT).sw(c, entry.0 as u64);
                    let head = head_slot.lw(c);
                    Addr(head as u32).off(adb::E_PREV).sw(c, entry.0 as u64);
                    c.cycles += 1;
                }
            }
            c.cycles += 7;
            abi::set_word(c, A0, entry.0.wrapping_add(4));
            eval_curves(c, 0x0031_14dc)?;
        } else {
            // not cached: take the least-recently-used entry (the head) and
            // decode the chunk into it
            c.cycles += 9;
            c.g[S4][0] = 0x50_0000;
            c.g[S0][0] = (bank << 2) as u64;
            let mut entry = head_slot.lw(c) as u32;
            let mut new_slot = None;
            if entry == 0 {
                c.cycles += 4;
                c.g[A0][0] = bank as u64;
                c.g[A1][0] = curves;
                anim_callee::ALLOC_CHAN_CACHE.call(c, 0x0031_1528)?;
                entry = c.g[V0][0] as u32;
            } else {
                c.cycles += 3;
                let owner = Addr(entry).lw(c) as u32;
                if owner != 0 {
                    // evict: the old owner loses its slot
                    c.cycles += 3;
                    new_slot = Some(Addr(owner).off(5).lbu(c));
                    Addr(owner).off(5).sb(c, slot);
                }
            }
            let new_slot = match new_slot {
                Some(s) => s,
                None => {
                    // (entry - pool) / 0x24 by the inverse of 9
                    c.cycles += 8;
                    let pool = Addr(adb::POOL.wrapping_add(bank << 2)).lw(c) as u32;
                    let scaled = abi::mult(c, entry.wrapping_sub(pool), 0x38e3_8e39);
                    sx(((scaled as i32) >> 2) as u32)
                }
            };
            c.cycles += 20;
            desc.off(5).sb(c, new_slot);
            let e = Addr(entry);
            e.sw(c, desc.0 as u64);
            let next = e.off(adb::E_NEXT).lw(c);
            head_slot.sw(c, next);
            c.g[S0][0] = sx(entry.wrapping_add(4));
            c.g[A0][0] = c.g[S0][0];
            c.g[A3][0] = db.off(adb::DATA).lw(c);
            let data = c.g[A3][0] as u32;
            let offset = desc.lw(c) as u32;
            c.g[A2][0] = desc.off(6).lhu(c);
            abi::set_word(c, A1, data.wrapping_add(offset));
            anim_callee::DECODE_CHAN_CACHE.call(c, 0x0031_1584)?;
            c.g[A0][0] = c.g[S0][0];
            eval_curves(c, 0x0031_159c)?;
        }
        frame.restore(c, S0, 0x70);
        c.cycles += 10;
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    });

    /// `AnimDataBase_EvalChanCurves(a0, firstCurve, limit, out, frame)`.
    fn eval_curves(c: &mut Cpu, ret: u32) -> Flow {
        c.f[12] = c.f[20];
        c.g[A1][0] = c.g[S5][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S6][0];
        anim_callee::EVAL_CHAN_CURVES.call(c, ret)
    }
}

/// `AnimDataBase_EvalChanCurves` entry points and jump tables.
mod eval {
    pub const ENTRY: u32 = 0x0031_2c20;
    pub const END: u32 = 0x0031_30a4;
    /// Loop heads (resume points after an unwind): the interpolating loop
    /// and the loop for frames that fall on keys.
    pub const LERP_HEAD: u32 = 0x0031_2d28;
    pub const KEY_HEAD: u32 = 0x0031_2f18;
    /// The curve-kind jump tables (`.rodata`) and their targets (left in a0
    /// as by the original's `lw`/`jr`).
    pub const LERP_TABLE_AT: u32 = 0x0048_a530;
    pub const KEY_TABLE_AT: u32 = 0x0048_a560;
    pub const LERP_TABLE: [u32; 10] = [
        0x0031_2d4c, 0x0031_2d54, 0x0031_2d60, 0x0031_2d78, 0x0031_2d9c, 0x0031_2dcc, 0x0031_2e08,
        0x0031_2e4c, 0x0031_2e90, 0x0031_2ed8,
    ];
    pub const KEY_TABLE: [u32; 10] = [
        0x0031_2f3c, 0x0031_2f44, 0x0031_2f4c, 0x0031_2f60, 0x0031_2f80, 0x0031_2fac, 0x0031_2fec,
        0x0031_3014, 0x0031_303c, 0x0031_3068,
    ];
    /// Curve record (0x14 bytes): +0 kind, +4.. coefficients / scale, bias
    /// and key index.
    pub const CURVE_SIZE: u32 = 0x14;
}

/// `void AnimDataBase_EvalChanCurves(Chan* ch, int first, int count, float*
/// out, float frame)` (0x312c20): evaluate `count` (at least one) curves of
/// a decoded channel at `frame`, one float each into `out`. Kinds: 0 zero,
/// 1 constant, 2..4 polynomials in `frame`, 5 two linear pieces split at a
/// knee, 6/7/8/9 keyed: u8 keys at full / half rate, u16 keys, f32 keys,
/// interpolated linearly between this frame's key and the next one (scale
/// and bias for the integer keys). When the frame (and the half-rate frame)
/// falls on a key the cheaper key loop runs instead.
///
/// The channel (`ch`): +0 curve records, +4 u8 keys, +8 half-rate u8 keys,
/// +0xc u16 keys, +0x10 f32 keys, +0x14..+0x17 their per-frame strides.
/// A dispatcher: re-entered at a loop head after an unwind, with the loop
/// state in t0..t9, s0, a2, a3, v1, f4, f5, f12.
pub fn anim_data_base_eval_chan_curves(c: &mut Cpu) {
    let interpolate = match c.pc {
        eval::ENTRY => eval_chan_prologue(c),
        eval::LERP_HEAD => true,
        eval::KEY_HEAD => false,
        pc => {
            c.jump_out(pc, eval::ENTRY, eval::END);
            return;
        }
    };
    let finished = if interpolate { eval_lerp_loop(c) } else { eval_key_loop(c) };
    if finished {
        c.cycles += 5;
        let frame = Frame { size: 0x40 };
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore(c, S3, 0);
        frame.close(c);
    }
}

/// The prologue of `AnimDataBase_EvalChanCurves`: key pointers for this
/// frame and the next, the fractions; true for the interpolating loop.
fn eval_chan_prologue(c: &mut Cpu) -> bool {
    const HALF: FpuF32 = FpuF32(0x3f00_0000);
    const ZERO: FpuF32 = FpuF32::ZERO;
    c.cycles += 45;
    let ch = Addr(abi::arg(c, 0));
    let (first, count) = (abi::arg(c, 1), abi::arg(c, 2));
    let t = FpuF32(c.f[12]);
    c.g[AT][0] = HALF.0 as u64;
    let i = t.to_i32() as u32;
    abi::set_word(c, V1, i);
    let t_half = t * HALF;
    let j = t_half.to_i32() as u32;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S0, 0x30);
    frame.save(c, S1, 0x20);
    frame.save(c, S2, 0x10);
    frame.save(c, S3, 0);
    let first_off = abi::mult(c, first, eval::CURVE_SIZE);
    let count_off = abi::mult(c, count, eval::CURVE_SIZE);
    let stride_byte = ch.off(0x14).lbu(c) as u32;
    let stride_half = ch.off(0x15).lbu(c) as u32;
    let stride_u16 = ch.off(0x16).lbu(c) as u32;
    let stride_f32 = ch.off(0x17).lbu(c) as u32;
    let half_off = abi::mult(c, j, stride_half);
    let records = ch.lw(c) as u32;
    let u16_off = abi::mult(c, i, stride_u16) << 1;
    let f32_off = abi::mult(c, i, stride_f32) << 2;
    let curve = records.wrapping_add(first_off);
    let eps = Addr((c.g[GP][0] as u32).wrapping_sub(0x3228)).read_fpu(c);
    let byte_off = abi::mult(c, i, stride_byte);
    let half_base = ch.off(8).lw(c) as u32;
    let f32_keys = (ch.off(0x10).lw(c) as u32).wrapping_add(f32_off);
    let byte_keys = (ch.off(4).lw(c) as u32).wrapping_add(byte_off);
    let u16_base = ch.off(0xc).lw(c) as u32;
    let u16_keys = u16_base.wrapping_add(u16_off);
    let half_keys = half_base.wrapping_add(half_off);
    let end = curve.wrapping_add(count_off);
    let j_f = FpuF32::from_i32(j as i32);
    let mut frac = t - FpuF32::from_i32(i as i32);
    let mut frac_half = t_half - j_f;
    // the loop state, in the original's registers
    c.g[S2][0] = stride_byte as u64;
    c.g[S3][0] = stride_half as u64;
    c.g[T8][0] = stride_u16 as u64;
    c.g[T9][0] = stride_f32 as u64;
    c.g[A1][0] = sx(half_base);
    c.g[V0][0] = sx(u16_base);
    abi::set_word(c, V1, byte_off);
    abi::set_word(c, T2, f32_off);
    abi::set_word(c, T3, u16_off);
    abi::set_word(c, T5, curve);
    abi::set_word(c, T6, half_keys);
    abi::set_word(c, T4, f32_keys);
    abi::set_word(c, T1, byte_keys);
    abi::set_word(c, T0, u16_keys);
    abi::set_word(c, A2, end);
    c.f[0] = j_f.0;
    c.f[1] = t_half.0;
    c.f[2] = eps.0;
    // fractions below epsilon count as on the key
    let small = frac.lt(eps);
    abi::set_fcc(c, small);
    if small {
        c.cycles += 1;
        frac = ZERO;
    }
    c.cycles += 3;
    let small = frac_half.lt(eps);
    abi::set_fcc(c, small);
    if small {
        c.cycles += 1;
        frac_half = ZERO;
    }
    c.f[4] = frac.0;
    c.f[5] = frac_half.0;
    c.cycles += 5;
    c.f[0] = 0;
    c.g[V0][0] = sx(stride_u16 << 1);
    let on_key = frac.eq_s(ZERO);
    abi::set_fcc(c, on_key);
    if on_key {
        c.cycles += 4;
        let on_half_key = frac_half.eq_s(ZERO);
        abi::set_fcc(c, on_half_key);
        if on_half_key {
            c.g[V1][0] = Addr(curve).lw(c);
            return false;
        }
        c.cycles += 1;
    }
    // the next frame's keys
    c.cycles += 6;
    abi::set_word(c, V1, stride_f32 << 2);
    abi::set_word(c, S0, u16_keys.wrapping_add(stride_u16 << 1));
    abi::set_word(c, T7, f32_keys.wrapping_add(stride_f32 << 2));
    abi::set_word(c, T3, byte_keys.wrapping_add(stride_byte));
    abi::set_word(c, T2, half_keys.wrapping_add(stride_half));
    c.g[V1][0] = Addr(curve).lw(c);
    true
}

/// The curve-kind dispatch both loops start with (v1 = kind): v0 =
/// 0x490000, and for kinds 0..9 the table pointer in v0/v1 and the target
/// in a0 (6 more cycles). None for other kinds.
fn eval_kind(c: &mut Cpu, table_at: u32, table: &[u32; 10]) -> Option<usize> {
    c.cycles += 3;
    let kind = c.g[V1][0];
    c.g[V0][0] = 0x49_0000;
    if kind >= 10 {
        return None;
    }
    c.cycles += 6;
    abi::set_word(c, V1, ((kind as u32) << 2).wrapping_add(table_at));
    abi::set_word(c, V0, table_at);
    abi::set_word(c, A0, table[kind as usize]);
    Some(kind as usize)
}

/// Horner evaluation of the polynomial kinds 2..4: `((c0 t + c1) t + c2)
/// t + c3` with the coefficients from +4; leaves the coefficients after the
/// first in f1.. and the result in f0.
fn eval_poly(c: &mut Cpu, curve: Addr, t: FpuF32, terms: u32) -> FpuF32 {
    let mut v = curve.off(4).read_fpu(c);
    for k in 1..terms {
        let coef = curve.off(4 + 4 * k).read_fpu(c);
        c.f[k as usize] = coef.0;
        v = t * v + coef;
    }
    c.f[0] = v.0;
    v
}

/// The interpolating loop of `AnimDataBase_EvalChanCurves` (head 0x312d28).
/// False when an event unwound the thread.
fn eval_lerp_loop(c: &mut Cpu) -> bool {
    loop {
        if let Some(kind) = eval_kind(c, eval::LERP_TABLE_AT, &eval::LERP_TABLE) {
            let curve = Addr(c.g[T5][0] as u32);
            let out = Addr(c.g[A3][0] as u32);
            let t = FpuF32(c.f[12]);
            match kind {
                0 => {
                    c.cycles += 2;
                    out.sw(c, 0);
                }
                1 => {
                    c.cycles += 3;
                    let v = curve.off(4).read_fpu(c);
                    c.f[0] = v.0;
                    out.write_fpu(c, v);
                }
                2..=4 => {
                    c.cycles += 3 * kind as u64;
                    let v = eval_poly(c, curve, t, kind as u32);
                    out.write_fpu(c, v);
                }
                5 => {
                    c.cycles += 6;
                    let knee = curve.off(0x10).read_fpu(c);
                    let a = curve.off(4).read_fpu(c);
                    let before = t.le(knee);
                    abi::set_fcc(c, before);
                    let b = curve.off(8).read_fpu(c);
                    let k = curve.off(0xc).read_fpu(c);
                    c.f[2] = b.0;
                    c.f[3] = k.0;
                    if before {
                        c.cycles += 4;
                        let v = t * a + b;
                        c.f[0] = v.0;
                        c.f[1] = a.0;
                        out.write_fpu(c, v);
                    } else {
                        c.cycles += 5 + 1 + 1;
                        let at_knee = a * knee + b;
                        let tail = k * (t - knee);
                        let v = at_knee + tail;
                        c.f[0] = tail.0;
                        c.f[1] = v.0;
                        out.write_fpu(c, v);
                    }
                }
                6 | 7 => {
                    // u8 keys (6: full rate, t1/t3, f4; 7: half rate, t6/t2, f5)
                    c.cycles += 17 + 1;
                    let (cur_base, next_base, frac) = if kind == 6 {
                        (c.g[T1][0] as u32, c.g[T3][0] as u32, FpuF32(c.f[4]))
                    } else {
                        (c.g[T6][0] as u32, c.g[T2][0] as u32, FpuF32(c.f[5]))
                    };
                    let index = curve.off(0xc).lw(c) as u32;
                    let scale = curve.off(8).read_fpu(c);
                    let bias = curve.off(4).read_fpu(c);
                    let (next_at, cur_at) = (next_base.wrapping_add(index), cur_base.wrapping_add(index));
                    let next = Addr(next_at).lbu(c);
                    let cur = Addr(cur_at).lbu(c);
                    abi::set_word(c, A0, next_at);
                    abi::set_word(c, V0, cur_at);
                    c.g[A1][0] = next;
                    c.g[V1][0] = cur;
                    let cur_f = FpuF32::from_i32(cur as i32);
                    let step = frac * (FpuF32::from_i32(next as i32) - cur_f);
                    let v = (cur_f + step) * scale + bias;
                    c.f[0] = step.0;
                    c.f[1] = v.0;
                    c.f[2] = bias.0;
                    c.f[3] = scale.0;
                    out.write_fpu(c, v);
                }
                8 => {
                    // u16 keys (t0, next s0)
                    c.cycles += 18 + 1;
                    let index = (curve.off(0xc).lw(c) as u32) << 1;
                    let scale = curve.off(8).read_fpu(c);
                    let bias = curve.off(4).read_fpu(c);
                    let next_at = index.wrapping_add(c.g[S0][0] as u32);
                    let cur_at = index.wrapping_add(c.g[T0][0] as u32);
                    let next = Addr(next_at).lhu(c);
                    let cur = Addr(cur_at).lhu(c);
                    abi::set_word(c, A1, next_at);
                    abi::set_word(c, V0, cur_at);
                    c.g[A0][0] = next;
                    c.g[V1][0] = cur;
                    let cur_f = FpuF32::from_i32(cur as i32);
                    let step = FpuF32(c.f[4]) * (FpuF32::from_i32(next as i32) - cur_f);
                    let v = (cur_f + step) * scale + bias;
                    c.f[0] = step.0;
                    c.f[1] = v.0;
                    c.f[2] = bias.0;
                    c.f[3] = scale.0;
                    out.write_fpu(c, v);
                }
                _ => {
                    // 9: f32 keys (t4, next t7)
                    c.cycles += 8 + 1 + 1;
                    let index = (curve.off(0xc).lw(c) as u32) << 2;
                    let next_at = index.wrapping_add(c.g[T7][0] as u32);
                    let cur_at = index.wrapping_add(c.g[T4][0] as u32);
                    abi::set_word(c, V1, next_at);
                    abi::set_word(c, V0, cur_at);
                    let next = Addr(next_at).read_fpu(c);
                    let cur = Addr(cur_at).read_fpu(c);
                    let step = FpuF32(c.f[4]) * (next - cur);
                    let v = cur + step;
                    c.f[0] = step.0;
                    c.f[1] = v.0;
                    out.write_fpu(c, v);
                }
            }
            c.cycles += 1;
            abi::set_word(c, A3, out.0.wrapping_add(4));
        }
        c.cycles += 3;
        let next = (c.g[T5][0] as u32).wrapping_add(eval::CURVE_SIZE);
        abi::set_word(c, T5, next);
        if c.g[T5][0] == c.g[A2][0] {
            Frame { size: 0x40 }.restore(c, S0, 0x30);
            return true;
        }
        c.cycles += 2;
        c.g[V1][0] = Addr(next).lw(c);
        if back_edge_unwinds(c, eval::LERP_HEAD) {
            return false;
        }
    }
}

/// The key loop of `AnimDataBase_EvalChanCurves` (head 0x312f18): every
/// keyed curve reads this frame's key. False when an event unwound the
/// thread.
fn eval_key_loop(c: &mut Cpu) -> bool {
    loop {
        if let Some(kind) = eval_kind(c, eval::KEY_TABLE_AT, &eval::KEY_TABLE) {
            let curve = Addr(c.g[T5][0] as u32);
            let out = Addr(c.g[A3][0] as u32);
            let t = FpuF32(c.f[12]);
            match kind {
                0 => {
                    c.cycles += 2;
                    out.sw(c, 0);
                }
                1 => {
                    c.cycles += 2 + 1;
                    let v = curve.off(4).read_fpu(c);
                    c.f[0] = v.0;
                    out.write_fpu(c, v);
                }
                2..=4 => {
                    c.cycles += 3 * kind as u64 - 1 + 1;
                    let v = eval_poly(c, curve, t, kind as u32);
                    out.write_fpu(c, v);
                }
                5 => {
                    c.cycles += 6;
                    let knee = curve.off(0x10).read_fpu(c);
                    let a = curve.off(4).read_fpu(c);
                    let before = t.le(knee);
                    abi::set_fcc(c, before);
                    let b = curve.off(8).read_fpu(c);
                    let k = curve.off(0xc).read_fpu(c);
                    c.f[2] = b.0;
                    c.f[3] = k.0;
                    if before {
                        c.cycles += 3 + 1;
                        let v = t * a + b;
                        c.f[0] = v.0;
                        c.f[1] = a.0;
                        out.write_fpu(c, v);
                    } else {
                        c.cycles += 7;
                        let at_knee = a * knee + b;
                        let tail = k * (t - knee);
                        let v = at_knee + tail;
                        c.f[0] = tail.0;
                        c.f[1] = v.0;
                        out.write_fpu(c, v);
                    }
                }
                6 | 7 => {
                    c.cycles += 10 + 1;
                    let base = if kind == 6 { c.g[T1][0] as u32 } else { c.g[T6][0] as u32 };
                    let at = base.wrapping_add(curve.off(0xc).lw(c) as u32);
                    let scale = curve.off(8).read_fpu(c);
                    let bias = curve.off(4).read_fpu(c);
                    let key = Addr(at).lbu(c);
                    abi::set_word(c, V1, at);
                    c.g[V0][0] = key;
                    let v = FpuF32::from_i32(key as i32) * scale + bias;
                    c.f[0] = v.0;
                    c.f[1] = bias.0;
                    c.f[2] = scale.0;
                    out.write_fpu(c, v);
                }
                8 => {
                    c.cycles += 11 + 1;
                    let index = (curve.off(0xc).lw(c) as u32) << 1;
                    let scale = curve.off(8).read_fpu(c);
                    let bias = curve.off(4).read_fpu(c);
                    let at = index.wrapping_add(c.g[T0][0] as u32);
                    let key = Addr(at).lhu(c);
                    abi::set_word(c, V1, at);
                    c.g[V0][0] = key;
                    let v = FpuF32::from_i32(key as i32) * scale + bias;
                    c.f[0] = v.0;
                    c.f[1] = bias.0;
                    c.f[2] = scale.0;
                    out.write_fpu(c, v);
                }
                _ => {
                    c.cycles += 4 + 1;
                    let index = (curve.off(0xc).lw(c) as u32) << 2;
                    let at = index.wrapping_add(c.g[T4][0] as u32);
                    abi::set_word(c, V0, at);
                    let v = Addr(at).read_fpu(c);
                    c.f[0] = v.0;
                    out.write_fpu(c, v);
                }
            }
            c.cycles += 1;
            abi::set_word(c, A3, out.0.wrapping_add(4));
        }
        c.cycles += 3;
        let next = (c.g[T5][0] as u32).wrapping_add(eval::CURVE_SIZE);
        abi::set_word(c, T5, next);
        if c.g[T5][0] == c.g[A2][0] {
            c.cycles += 1;
            Frame { size: 0x40 }.restore(c, S0, 0x30);
            return true;
        }
        c.g[V1][0] = Addr(next).lw(c);
        if back_edge_unwinds(c, eval::KEY_HEAD) {
            return false;
        }
    }
}

// ---------------------------------------------------------------- UV scroll

/// `UVScrollModifier` fields.
mod uv {
    /// Mode: 2 / 6 reverse the scroll when a cycle ends; 6 scrolls with a
    /// linear ramp up and down over the cycle.
    pub const MODE: u32 = 0x0;
    /// Time in the current cycle and the cycle length.
    pub const TIME: u32 = 0x4;
    pub const CYCLE: u32 = 0x8;
    /// Delay before the scroll (re)starts.
    pub const DELAY: u32 = 0xc;
    /// A phase accumulator and its step (wrapped by the global at gp-0x29cc).
    pub const PHASE: u32 = 0x10;
    pub const PHASE_STEP: u32 = 0x14;
    /// UV offset and velocity.
    pub const U: u32 = 0x30;
    pub const V: u32 = 0x34;
    pub const DU: u32 = 0x38;
    pub const DV: u32 = 0x3c;
    pub const RUNNING: u32 = 0x40;
}

/// `void UVScrollModifier_Update(UVScrollModifier* m)` (0x35f7d0): advance
/// a texture scroll by one frame (1 / the frame rate at `*(gp+0x2a74)+0x10`).
/// While stopped it waits out the delay; while running it advances the
/// phase, ends the cycle after `CYCLE` (reversing the velocity in modes 2
/// and 6), and moves the UV offset by the velocity (mode 6: by the area
/// under a triangular speed ramp), wrapping it into [-1, 1].
pub fn uv_scroll_modifier_update(c: &mut Cpu) {
    const ZERO: FpuF32 = FpuF32::ZERO;
    const ONE: FpuF32 = FpuF32::ONE;
    const MINUS_ONE: FpuF32 = FpuF32(0xbf80_0000);
    const HALF: FpuF32 = FpuF32(0x3f00_0000);
    let m = Addr(abi::arg(c, 0));
    let gp = c.g[GP][0] as u32;
    c.cycles += 6;
    let sp = (c.g[SP][0] as u32).wrapping_sub(0x40);
    c.g[SP][0] = sx(sp);
    let stack = Addr(sp);
    // the registers a caller can see afterwards
    let mut f0 = FpuF32(c.f[0]);
    let mut f1 = FpuF32(c.f[1]);
    let mut v0 = c.g[V0][0];
    let mut v1 = c.g[V1][0];
    let cycle = m.off(uv::CYCLE).read_fpu(c);
    let has_cycle = ZERO.lt(cycle);
    abi::set_fcc(c, has_cycle);
    'out: {
        if !has_cycle {
            c.cycles += 4;
            f0 = m.off(uv::DELAY).read_fpu(c);
            let has_delay = ZERO.lt(f0);
            abi::set_fcc(c, has_delay);
            if !has_delay {
                break 'out;
            }
        }
        c.cycles += 11;
        let clock = Addr(gp.wrapping_add(0x2a74)).lw(c);
        c.g[AT][0] = ONE.0 as u64;
        f1 = FpuF32::from_i32(Addr(clock as u32).off(0x10).read_u32(c) as i32);
        let old_time = m.off(uv::TIME).read_fpu(c);
        v0 = m.off(uv::RUNNING).lw(c);
        f0 = ONE.div(f1);
        let time = old_time + f0;
        m.off(uv::TIME).write_fpu(c, time);
        if v0 == 0 {
            // stopped: start once the delay has passed
            c.cycles += 4;
            f0 = m.off(uv::DELAY).read_fpu(c);
            let waited = f0.le(time);
            abi::set_fcc(c, waited);
            if waited {
                c.cycles += 4;
                let start = ZERO.lt(cycle);
                abi::set_fcc(c, start);
                m.off(uv::TIME).write_fpu(c, ZERO);
                if start {
                    c.cycles += 2;
                    v0 = 1;
                    m.off(uv::RUNNING).sw(c, 1);
                }
            }
            break 'out;
        }
        // the phase accumulator
        c.cycles += 5;
        f1 = m.off(uv::PHASE_STEP).read_fpu(c);
        let still = f1.eq_s(ZERO);
        abi::set_fcc(c, still);
        if !still {
            c.cycles += 7;
            let step = f1;
            f0 = m.off(uv::PHASE).read_fpu(c) + step;
            let wrap = Addr(gp.wrapping_sub(0x29cc)).read_fpu(c);
            let over = wrap.le(step);
            abi::set_fcc(c, over);
            m.off(uv::PHASE).write_fpu(c, f0);
            let wrapped = if over {
                c.cycles += 2;
                Some(step - wrap)
            } else {
                c.cycles += 4;
                let under = step.le(wrap);
                abi::set_fcc(c, under);
                if under {
                    c.cycles += 1;
                    Some(step + wrap)
                } else {
                    None
                }
            };
            if let Some(s) = wrapped {
                f0 = s;
                c.cycles += 2;
                m.off(uv::PHASE_STEP).write_fpu(c, s);
            }
        }
        f1 = m.off(uv::TIME).read_fpu(c);
        // end of a cycle
        c.cycles += 5;
        f0 = m.off(uv::CYCLE).read_fpu(c);
        let ended = f0.le(f1);
        abi::set_fcc(c, ended);
        if ended {
            c.cycles += 7;
            m.off(uv::TIME).sw(c, 0);
            f1 = m.off(uv::DELAY).read_fpu(c);
            f0 = m.off(uv::TIME).read_fpu(c);
            let pause = f0.lt(f1);
            abi::set_fcc(c, pause);
            if pause {
                m.off(uv::RUNNING).sw(c, 0);
            }
            c.cycles += 4;
            let mode = m.off(uv::MODE).lw(c);
            v1 = mode;
            v0 = 6;
            let reverse = mode == 2 || {
                c.cycles += 2;
                mode == 6
            };
            if reverse {
                // velocity = -velocity, through a stack doubleword
                c.cycles += 11;
                f0 = -m.off(uv::DU).read_fpu(c);
                f1 = -m.off(uv::DV).read_fpu(c);
                stack.write_fpu(c, f0);
                stack.off(4).write_fpu(c, f1);
                let pair = stack.read_u64_unaligned(c);
                m.off(uv::DU).write_u64_unaligned(c, pair);
            }
        }
        v0 = m.off(uv::RUNNING).lw(c);
        c.cycles += 2;
        let running = v0 != 0;
        v0 = 6;
        if !running {
            break 'out;
        }
        // move the UV offset
        c.cycles += 3;
        let mode = m.off(uv::MODE).lw(c);
        v1 = mode;
        if mode != 6 {
            c.cycles += 7;
            let u = m.off(uv::U).read_fpu(c) + m.off(uv::DU).read_fpu(c);
            let v = m.off(uv::V).read_fpu(c) + m.off(uv::DV).read_fpu(c);
            m.off(uv::U).write_fpu(c, u);
            m.off(uv::V).write_fpu(c, v);
        } else {
            // ramp: the speed rises linearly over the first half of the
            // cycle (counted from the end when the velocity is negative)
            c.cycles += 6;
            let du = m.off(uv::DU).read_fpu(c);
            let negative = du.lt(ZERO);
            abi::set_fcc(c, negative);
            let cycle = m.off(uv::CYCLE).read_fpu(c);
            let (t, spill_up, spill_down) = if negative {
                c.cycles += 8;
                (m.off(uv::TIME).read_fpu(c), 0x10, 0x20)
            } else {
                c.cycles += 9;
                (cycle - m.off(uv::TIME).read_fpu(c), 0x30, 0x10)
            };
            let first_half = t.lt(cycle * HALF);
            abi::set_fcc(c, first_half);
            let dv = m.off(uv::DV).read_fpu(c);
            let rate = ONE.div(cycle);
            let (du_t, dv_t) = if first_half {
                // offset += velocity * t / cycle
                c.cycles += 17;
                let (du_t, dv_t) = (du * t, dv * t);
                stack.off(spill_up).write_fpu(c, du_t);
                stack.off(spill_up + 4).write_fpu(c, dv_t);
                (du_t, dv_t)
            } else {
                // offset += velocity * (cycle - t) / cycle
                c.cycles += (if negative { 11 } else { 10 }) + 6;
                let rest = cycle - t;
                let (du_t, dv_t) = (du * rest, dv * rest);
                stack.off(spill_down + 4).write_fpu(c, dv_t);
                stack.off(spill_down).write_fpu(c, du_t);
                (du_t, dv_t)
            };
            let (step_u, step_v) = (du_t * rate, dv_t * rate);
            let v = m.off(uv::V).read_fpu(c) + step_v;
            let u = m.off(uv::U).read_fpu(c) + step_u;
            stack.off(4).write_fpu(c, step_v);
            stack.write_fpu(c, step_u);
            c.cycles += 3;
            m.off(uv::V).write_fpu(c, v);
            m.off(uv::U).write_fpu(c, u);
        }
        // wrap the offset into [-1, 1]
        c.cycles += 6;
        f1 = m.off(uv::U).read_fpu(c);
        let over = ONE.lt(f1);
        abi::set_fcc(c, over);
        let wrapped = if over {
            c.cycles += 2;
            Some(f1 - ONE)
        } else {
            c.cycles += 6;
            f0 = MINUS_ONE;
            let under = f1.lt(MINUS_ONE);
            abi::set_fcc(c, under);
            if under {
                c.cycles += 1;
                Some(f1 + ONE)
            } else {
                None
            }
        };
        if let Some(w) = wrapped {
            f0 = w;
            c.cycles += 2;
            m.off(uv::U).write_fpu(c, w);
        }
        f1 = m.off(uv::V).read_fpu(c);
        c.cycles += 5;
        let over = ONE.lt(f1);
        abi::set_fcc(c, over);
        if over {
            c.cycles += 3;
            f0 = f1 - ONE;
            m.off(uv::V).write_fpu(c, f0);
        } else {
            c.cycles += 5;
            f0 = MINUS_ONE;
            let under = f1.lt(MINUS_ONE);
            abi::set_fcc(c, under);
            if under {
                c.cycles += 3;
                f0 = f1 + ONE;
                m.off(uv::V).write_fpu(c, f0);
            }
        }
    }
    c.cycles += 2;
    c.f[0] = f0.0;
    c.f[1] = f1.0;
    c.g[V0][0] = v0;
    c.g[V1][0] = v1;
    c.g[SP][0] = sx(sp.wrapping_add(0x40));
    abi::ret(c);
}

// ---------------------------------------------------------------- world lights

/// `WorldLightMan_LightContribution` entry points, light fields and jump
/// tables.
mod light {
    pub const ENTRY: u32 = 0x002f_5d30;
    pub const END: u32 = 0x002f_6164;
    /// Loop heads of the two `cone^exponent` loops (resume points).
    pub const POWI_INNER: u32 = 0x002f_5f48;
    pub const POWI_PENUMBRA: u32 = 0x002f_6018;
    /// 0 ambient, 1 spot / point (cone), 2 directional-like (no cone).
    pub const TYPE: u32 = 0x10;
    pub const INTENSITY: u32 = 0x14;
    pub const SCALE: u32 = 0x18;
    pub const RANGE: u32 = 0x1c;
    /// Cosines of the inner and outer cone angles.
    pub const COS_INNER: u32 = 0x5c;
    pub const COS_OUTER: u32 = 0x60;
    /// Distance falloff power (s8, 1..3) and cone exponent (s8).
    pub const FALLOFF: u32 = 0x64;
    pub const EXPONENT: u32 = 0x65;
    /// The exponent switches' jump tables (`.rodata`) and their targets.
    pub const INNER_TABLE_AT: u32 = 0x0048_7cf0;
    pub const PENUMBRA_TABLE_AT: u32 = 0x0048_7d10;
    pub const INNER_TABLE: [u32; 5] = [0x002f_5eb8, 0x002f_5ec8, 0x002f_5ed4, 0x002f_5ee0, 0x002f_5ef0];
    pub const PENUMBRA_TABLE: [u32; 5] =
        [0x002f_5f90, 0x002f_5fa4, 0x002f_5fa0, 0x002f_5fac, 0x002f_5fbc];
}

/// `WorldLightMan_LightDistance` (0x2f6168, s09).
const LIGHT_DISTANCE: Callee = Callee { addr: 0x002f_6168, f: ssx3_ee::s09::WorldLightMan_LightDistance };

/// An FPU register as a float.
#[inline]
fn fr(c: &Cpu, r: usize) -> FpuF32 {
    FpuF32(c.f[r])
}

/// `float WorldLightMan_LightContribution(WorldLightMan* m, Vec3* point,
/// Light* light)` (0x2f5d30): the light's intensity at a point. Ambient
/// lights give their scale (+0x18); a cone light (type 1) gives
/// `scale * intensity * falloff * attenuation * cone` when the point is
/// inside the outer cone, with the cone factor `cos^exp` inside the inner
/// cone and `inner^exp * (cos - outer) / (inner - outer)` in the penumbra;
/// type 2 gives `scale * intensity * falloff * attenuation`. The distance,
/// its inverse and the cone cosine come from `WorldLightMan_LightDistance`
/// (into the stack frame). Other types give 0.
///
/// A dispatcher: the `^exp` loops can unwind and are re-entered at their
/// heads with the state in v1, f0..f6 and s0.
pub fn world_light_man_light_contribution(c: &mut Cpu) {
    guest(c, |c| match c.pc {
        light::ENTRY => light_contribution(c),
        light::POWI_INNER => {
            light_powi(c, 1, light::POWI_INNER)?;
            inner_cone_finish(c);
            light_epilogue(c);
            Ok(())
        }
        light::POWI_PENUMBRA => {
            light_powi(c, 3, light::POWI_PENUMBRA)?;
            c.cycles += 1;
            c.f[0] = c.f[3];
            c.cycles += 1;
            c.f[3] = c.f[0];
            penumbra_finish(c);
            light_epilogue(c);
            Ok(())
        }
        pc => {
            c.jump_out(pc, light::ENTRY, light::END);
            Ok(())
        }
    })
}

/// The body from the entry: false results end in `f0 = 0`.
fn light_contribution(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = 1;
    frame.save(c, S0, 0x20);
    frame.save_ra(c, 0x10);
    c.g[S0][0] = c.g[A2][0];
    let l = Addr(c.g[S0][0] as u32);
    let kind = l.off(light::TYPE).lw(c);
    c.g[V1][0] = kind;
    c.g[V0][0] = ((kind as i64) < 2) as u64;
    let lit = if kind == 1 {
        cone_light(c, l)?
    } else {
        c.cycles += 2;
        if (kind as i64) < 2 {
            c.cycles += 2;
            if kind == 0 {
                c.f[0] = l.off(light::SCALE).read_u32(c);
                true
            } else {
                c.cycles += 2;
                false
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 3;
            if kind == 2 {
                plain_light(c, l)?
            } else {
                c.cycles += 2;
                false
            }
        }
    };
    if !lit {
        c.cycles += 1;
        c.f[0] = 0;
    }
    light_epilogue(c);
    Ok(())
}

/// 0x2f6154: restore s0 and ra, return.
fn light_epilogue(c: &mut Cpu) {
    c.cycles += 4;
    let frame = Frame { size: 0x30 };
    frame.restore(c, S0, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
}

/// `WorldLightMan_LightDistance(m, point, light, &dist, &invDist,
/// cosOut)` with the outputs in the frame (sp, sp|4, `cos_at`).
fn light_distance(c: &mut Cpu, cos_at: u64, ret: u32) -> Flow {
    c.cycles += 7;
    let sp = c.g[SP][0];
    c.g[A2][0] = c.g[S0][0];
    c.g[A3][0] = sp;
    c.g[T0][0] = sp | 4;
    c.g[T1][0] = cos_at;
    LIGHT_DISTANCE.call(c, ret)
}

/// Distance attenuation into f4: 1 when the light's range (+0x1c) is
/// below the threshold at gp-`at[0]` or the distance (sp+0) is within
/// gp-`at[1]`, else `1 - (d - start) * slope(gp-`at[2]`)`. Then the
/// inverse distance scaled by 100 into f1 and its test against 1 (`near`);
/// f2 = 1.0, v1 = the falloff power.
fn light_attenuation(c: &mut Cpu, l: Addr, at: [u32; 3]) -> bool {
    let gp = c.g[GP][0] as u32;
    let stack = Addr(c.g[SP][0] as u32);
    c.cycles += 6;
    let range = l.off(light::RANGE).read_fpu(c);
    let small = Addr(gp.wrapping_sub(at[0])).read_fpu(c);
    c.f[1] = range.0;
    c.f[0] = small.0;
    let no_range = range.lt(small);
    abi::set_fcc(c, no_range);
    let d = stack.read_fpu(c);
    c.f[2] = d.0;
    let over = if no_range {
        None
    } else {
        c.cycles += 5;
        let start = Addr(gp.wrapping_sub(at[1])).read_fpu(c);
        c.f[0] = start.0;
        let within = d.le(start);
        abi::set_fcc(c, within);
        if within {
            None
        } else {
            Some(d - start)
        }
    };
    c.g[AT][0] = FpuF32::ONE.0 as u64;
    match over {
        None => {
            c.cycles += 4;
            c.f[4] = FpuF32::ONE.0;
        }
        Some(over) => {
            c.cycles += 6;
            let slope = Addr(gp.wrapping_sub(at[2])).read_fpu(c);
            c.f[1] = slope.0;
            c.f[2] = FpuF32::ONE.0;
            c.f[4] = (FpuF32::ONE - over * slope).0;
        }
    }
    let inv = stack.off(4).read_fpu(c);
    c.f[0] = inv.0;
    c.cycles += 9;
    let x = inv * FpuF32(0x42c8_0000);
    c.f[1] = x.0;
    c.f[2] = FpuF32::ONE.0;
    let near = x.lt(FpuF32::ONE);
    abi::set_fcc(c, near);
    c.g[V1][0] = l.off(light::FALLOFF).lb(c);
    near
}

/// Type 1: a cone light. False when outside the cone or out of reach.
fn cone_light(c: &mut Cpu, l: Addr) -> Result<bool, Unwound> {
    let cos_at = c.g[SP][0] | 8;
    light_distance(c, cos_at, 0x002f_5d8c)?;
    let stack = Addr(c.g[SP][0] as u32);
    let cos = stack.off(8).read_fpu(c);
    c.f[1] = cos.0;
    if c.g[V0][0] == 0 {
        return Ok(false);
    }
    c.cycles += 6;
    let cos_outer = l.off(light::COS_OUTER).read_fpu(c);
    c.f[0] = cos_outer.0;
    c.f[5] = cos.0;
    c.f[6] = cos_outer.0;
    let in_cone = cos_outer.le(cos);
    abi::set_fcc(c, in_cone);
    if !in_cone {
        return Ok(false);
    }
    let near = light_attenuation(c, l, [0x38b0, 0x38ac, 0x38a8]);
    // falloff in f1: (100 / d)^power when near, else 1
    let mode = c.g[V1][0];
    let x = fr(c, 1);
    let mut falloff_one = !near;
    let mut intensity_loaded = false;
    if near {
        c.cycles += 3;
        c.g[V0][0] = ((mode as i64) < 3) as u64;
        if mode == 2 {
            c.cycles += 2;
            c.f[1] = (x * x).0;
        } else {
            c.cycles += 2;
            let low = (mode as i64) < 3;
            c.g[V0][0] = 1;
            if low {
                c.cycles += 2;
                if mode == 1 {
                    c.f[0] = l.off(light::INTENSITY).read_u32(c);
                    intensity_loaded = true;
                } else {
                    c.cycles += 2;
                    falloff_one = true;
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = 3;
                if mode == 3 {
                    let x2 = x * x;
                    c.f[0] = x2.0;
                    c.cycles += 2;
                    c.f[1] = (x2 * x).0;
                } else {
                    c.cycles += 2;
                    falloff_one = true;
                }
            }
        }
    }
    if falloff_one {
        c.cycles += 2;
        c.f[1] = FpuF32::ONE.0;
    }
    if !intensity_loaded {
        c.cycles += 1;
        c.f[0] = l.off(light::INTENSITY).read_u32(c);
    }
    // f2 = falloff * intensity * attenuation; f1 = inner cosine
    c.cycles += 6;
    let cos_inner = l.off(light::COS_INNER).read_fpu(c);
    c.f[3] = cos_inner.0;
    let lit = fr(c, 1) * fr(c, 0);
    c.f[0] = lit.0;
    let inside = cos_inner.le(cos);
    abi::set_fcc(c, inside);
    c.f[1] = cos_inner.0;
    c.f[2] = (lit * fr(c, 4)).0;
    let exponent = l.off(light::EXPONENT).lb(c);
    if inside {
        // cone = cos^exp (f1)
        c.cycles += 4;
        c.g[V1][0] = exponent;
        c.g[V0][0] = 0x48_0000;
        if exponent < 5 {
            c.cycles += 6;
            let e = exponent as usize;
            abi::set_word(c, V1, ((e as u32) << 2).wrapping_add(light::INNER_TABLE_AT));
            abi::set_word(c, V0, light::INNER_TABLE_AT);
            abi::set_word(c, A0, light::INNER_TABLE[e]);
            let cos = fr(c, 5);
            match e {
                0 => {
                    c.cycles += 4;
                    c.g[AT][0] = FpuF32::ONE.0 as u64;
                    c.f[1] = FpuF32::ONE.0;
                }
                1 => {
                    c.cycles += 3;
                    c.f[1] = cos.0;
                }
                2 => {
                    c.cycles += 3;
                    c.f[1] = (cos * cos).0;
                }
                3 => {
                    c.cycles += 4;
                    let p = cos * cos;
                    c.f[0] = p.0;
                    c.f[1] = (p * cos).0;
                }
                _ => {
                    c.cycles += 5;
                    let p = cos * cos * cos;
                    c.f[0] = p.0;
                    c.f[1] = (p * cos).0;
                }
            }
            c.f[4] = l.off(light::SCALE).read_u32(c);
        } else {
            c.cycles += 5;
            c.f[1] = 0;
            let cos = fr(c, 5);
            let zero = cos.eq_s(FpuF32::ZERO);
            abi::set_fcc(c, zero);
            c.f[0] = cos.0;
            if zero {
                c.cycles += 2;
                c.f[4] = l.off(light::SCALE).read_u32(c);
            } else {
                powi_setup(c, 5, 1, l)?;
                if c.g[V1][0] != 0 {
                    light_powi(c, 1, light::POWI_INNER)?;
                }
            }
        }
        inner_cone_finish(c);
    } else {
        // penumbra: cone = inner^exp * (cos - outer) / (inner - outer)
        c.cycles += 4;
        c.g[V1][0] = exponent;
        c.g[V0][0] = 0x48_0000;
        let inner = fr(c, 3);
        if exponent < 5 {
            c.cycles += 6;
            let e = exponent as usize;
            abi::set_word(c, V1, ((e as u32) << 2).wrapping_add(light::PENUMBRA_TABLE_AT));
            abi::set_word(c, V0, light::PENUMBRA_TABLE_AT);
            abi::set_word(c, A0, light::PENUMBRA_TABLE[e]);
            match e {
                0 => {
                    c.cycles += 4;
                    c.g[AT][0] = FpuF32::ONE.0 as u64;
                    c.f[3] = FpuF32::ONE.0;
                }
                1 => {
                    c.cycles += 2;
                }
                2 => {
                    c.cycles += 1 + 2;
                    c.f[3] = (inner * inner).0;
                }
                3 => {
                    c.cycles += 4;
                    let p = inner * inner;
                    c.f[0] = p.0;
                    c.f[3] = (p * inner).0;
                }
                _ => {
                    c.cycles += 5;
                    let p = inner * inner * inner;
                    c.f[0] = p.0;
                    c.f[3] = (p * inner).0;
                }
            }
            c.f[4] = l.off(light::SCALE).read_u32(c);
        } else {
            c.cycles += 5;
            c.f[3] = 0;
            let zero = inner.eq_s(FpuF32::ZERO);
            abi::set_fcc(c, zero);
            c.f[0] = inner.0;
            if zero {
                c.cycles += 3;
                c.f[0] = 0;
                c.f[4] = l.off(light::SCALE).read_u32(c);
            } else {
                powi_setup(c, 1, 3, l)?;
                if c.g[V1][0] != 0 {
                    light_powi(c, 3, light::POWI_PENUMBRA)?;
                }
                c.cycles += 1;
                c.f[0] = c.f[3];
            }
            c.cycles += 1;
            c.f[3] = c.f[0];
        }
        penumbra_finish(c);
    }
    Ok(true)
}

/// Before a `^exp` loop (exponent in v1, base register `base`, not 0): a
/// negative exponent inverts the base (f0 = 1 / base, v1 = -v1); the
/// accumulator `acc` = 1, f4 = the light's scale.
fn powi_setup(c: &mut Cpu, base: usize, acc: usize, l: Addr) -> Flow {
    c.cycles += 2;
    if (c.g[V1][0] as i64) < 0 {
        c.cycles += 4;
        c.g[AT][0] = FpuF32::ONE.0 as u64;
        let n = c.g[V1][0] as u32;
        abi::set_word(c, V1, 0u32.wrapping_sub(n));
        c.f[0] = FpuF32::ONE.div(fr(c, base)).0;
    }
    c.cycles += 4;
    c.g[AT][0] = FpuF32::ONE.0 as u64;
    c.f[acc] = FpuF32::ONE.0;
    c.f[4] = l.off(light::SCALE).read_u32(c);
    Ok(())
}

/// The `^exp` loop by squaring (head 0x2f5f48 / 0x2f6018): `acc *= f0`
/// for each set bit of v1, `f0 *= f0`, `v1 >>= 1` until v1 is 0. Err when
/// an event unwound the thread at the back edge.
fn light_powi(c: &mut Cpu, acc: usize, head: u32) -> Flow {
    loop {
        c.cycles += 3;
        let n = c.g[V1][0];
        c.g[V0][0] = n & 1;
        if n & 1 != 0 {
            c.f[acc] = (fr(c, acc) * fr(c, 0)).0;
        }
        c.cycles += 3;
        let n = sx(((n as u32) as i32 >> 1) as u32);
        c.g[V1][0] = n;
        let b = fr(c, 0);
        c.f[0] = (b * b).0;
        if n == 0 {
            return Ok(());
        }
        if back_edge_unwinds(c, head) {
            return Err(Unwound);
        }
    }
}

/// 0x2f5f60: inside the inner cone, f2 *= cone (f1); then the result.
fn inner_cone_finish(c: &mut Cpu) {
    c.cycles += 2;
    c.f[2] = (fr(c, 2) * fr(c, 1)).0;
    spot_finish(c);
}

/// 0x2f6038: in the penumbra, f2 *= inner^exp (f3) * (cos - outer) /
/// (inner - outer); then the result.
fn penumbra_finish(c: &mut Cpu) {
    c.cycles += 5;
    let width = fr(c, 1) - fr(c, 6);
    c.f[1] = width.0;
    let into = fr(c, 5) - fr(c, 6);
    let f2 = fr(c, 2) * fr(c, 3);
    let ramp = into.div(width);
    c.f[0] = ramp.0;
    c.f[2] = (f2 * ramp).0;
    spot_finish(c);
}

/// 0x2f604c: f0 = scale (f4) * f2.
fn spot_finish(c: &mut Cpu) {
    c.cycles += 2;
    c.f[0] = (fr(c, 4) * fr(c, 2)).0;
}

/// Type 2: no cone. False when out of reach.
fn plain_light(c: &mut Cpu, l: Addr) -> Result<bool, Unwound> {
    light_distance(c, 0, 0x002f_6068)?;
    if c.g[V0][0] == 0 {
        return Ok(false);
    }
    let near = light_attenuation(c, l, [0x38a4, 0x38a0, 0x389c]);
    // falloff in f2: (100 / d)^power when near, else 1
    let mode = c.g[V1][0];
    let x = fr(c, 1);
    let mut intensity_loaded = false;
    if near {
        c.cycles += 3;
        c.g[V0][0] = ((mode as i64) < 3) as u64;
        if mode == 2 {
            c.cycles += 2;
            c.f[2] = (x * x).0;
        } else {
            c.cycles += 2;
            let low = (mode as i64) < 3;
            c.g[V0][0] = 1;
            if low {
                c.cycles += 2;
                if mode == 1 {
                    c.f[2] = x.0;
                } else {
                    c.cycles += 2;
                    c.f[1] = l.off(light::INTENSITY).read_u32(c);
                    intensity_loaded = true;
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = 3;
                if mode == 3 {
                    let x2 = x * x;
                    c.f[0] = x2.0;
                    c.cycles += 2;
                    c.f[2] = (x2 * x).0;
                } else {
                    c.cycles += 2 + 2;
                    c.g[AT][0] = FpuF32::ONE.0 as u64;
                    c.f[2] = FpuF32::ONE.0;
                }
            }
        }
    }
    if !intensity_loaded {
        c.cycles += 1;
        c.f[1] = l.off(light::INTENSITY).read_u32(c);
    }
    c.cycles += 5;
    let scale = l.off(light::SCALE).read_fpu(c);
    let lit = fr(c, 2) * fr(c, 1);
    c.f[1] = lit.0;
    let lit = lit * fr(c, 4);
    c.f[2] = lit.0;
    c.f[0] = (scale * lit).0;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fill, put_words, scramble, Rng, DATA, GAME_GP};
    use ssx3_ee as ee;

    fn f(x: f32) -> u32 {
        x.to_bits()
    }

    fn pick<T: Copy>(r: &mut Rng, xs: &[T]) -> T {
        xs[r.next_u32() as usize % xs.len()]
    }

    // ------------------------------------------------------------ libgcc

    /// `__clz_tab` (bit length of each byte) where the generated code reads
    /// it (the test machine's RAM starts empty).
    fn clz_table(c: &mut Cpu, at: u32) {
        for i in 0..256u32 {
            c.bus.write8(at + i, (32 - i.leading_zeros()) as u8);
        }
    }

    /// Operands for every path of `__udivmoddi4`: d1 == 0 with d0 above or
    /// below n1, d == 0, normalized and unnormalized d1, q estimates that
    /// need one or two corrections, q0*d0 just above n.
    fn div_setup(c: &mut Cpu, seed: u32, table: u32) {
        let mut r = Rng(0x0d1f_d1f0_5eed_0001 ^ (seed as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15));
        scramble(c, &mut r);
        clz_table(c, table);
        let w = |r: &mut Rng| -> u32 {
            match r.next_u32() % 6 {
                0 => 0,
                1 => 0xffff_ffff,
                2 => 0x8000_0000 | r.next_u32() >> 16,
                3 => r.next_u32() >> (r.next_u32() % 32),
                _ => r.next_u32(),
            }
        };
        let n = (w(&mut r) as u64) << 32 | w(&mut r) as u64;
        let d: u64 = match seed % 10 {
            0 => w(&mut r) as u64,
            1 => (r.next_u32() >> (r.next_u32() % 32)) as u64,
            2 => 0,
            3 => (0x8000_0000 | r.next_u32()) as u64 | ((r.next_u32() as u64) << 32 | 0x8000_0000_0000_0000),
            4 => {
                // d0 > n1 with a high half that makes the estimate too big
                let d0 = 0x8000_ffffu32.wrapping_sub(r.next_u32() % 16);
                let n1 = d0.wrapping_sub(1 + r.next_u32() % 4);
                c.g[A0][0] = (n1 as u64) << 32 | r.next_u32() as u64;
                c.g[A1][0] = d0 as u64;
                return;
            }
            5 => {
                // d1 normalized after a shift, n just above a multiple
                let d = ((r.next_u32() >> (r.next_u32() % 24)) as u64) << 32 | r.next_u32() as u64;
                let q = (r.next_u32() % 5) as u64;
                c.g[A0][0] = d.wrapping_mul(q).wrapping_add(r.next_u32() as u64 % 3);
                c.g[A1][0] = d;
                return;
            }
            6 => n >> (r.next_u32() % 64),
            7 => n.wrapping_sub(r.next_u32() as u64 % 4),
            _ => (w(&mut r) as u64) << 32 | w(&mut r) as u64,
        };
        c.g[A0][0] = n;
        c.g[A1][0] = d;
    }

    #[test]
    fn udivdi3_matches() {
        check(ee::s15::f_00411538__generated, udivdi3, 500, &|c, seed| div_setup(c, seed, 0x0049_97a0));
    }

    #[test]
    fn umoddi3_matches() {
        check(ee::s15::f_0042dad8__generated, umoddi3, 500, &|c, seed| div_setup(c, seed, 0x0049_af18));
    }

    // ------------------------------------------------------------ AnimSequence

    /// A sequence at DATA (random fields, no triggers so the trigger
    /// callees take their short paths).
    fn sequence(c: &mut Cpu, r: &mut Rng, s: u32) {
        fill(c, r, s, 0x34);
        c.bus.write32(s + 0xac, 0);
        for k in 0..3 {
            let t = s + k * 0x1c;
            c.bus.write32(t + 0x8, r.f32(4.0));
            c.bus.write32(t + 0x10, f(pick(r, &[0.5, 1.0, 2.0, 3.0])));
            c.bus.write32(t + 0x14, r.f32(1.0));
            c.bus.write32(t + 0x18, if r.next_u32() % 3 == 0 { 0 } else { 1 });
        }
        c.bus.write32(s + 0x94, r.f32(1.0));
        c.bus.write32(s + 0xa4, f(pick(r, &[0.0, 0.25, 0.5])));
        c.bus.write32(s + 0xa8, f(pick(r, &[0.0, 0.25, 0.5])));
    }

    #[test]
    fn anim_seq_list_collect_tracks_matches() {
        check(ee::s10::f_00314418__generated, anim_seq_list_collect_tracks, 200, &|c, seed| {
            let mut r = Rng(0xc011_ec75 ^ (seed as u64) << 12);
            scramble(c, &mut r);
            let list = DATA;
            let out = DATA + 0x8000;
            let mut link = list + 4;
            for k in 0..seed % 4 {
                let s = DATA + 0x1000 + k * 0x400;
                sequence(c, &mut r, s);
                c.bus.write32(link, s);
                link = s + 0xc8;
            }
            c.bus.write32(link, 0);
            c.g[A0][0] = sx(list);
            c.g[A1][0] = sx(out);
        });
    }

    #[test]
    fn anim_sequence_advance_track_matches() {
        check(ee::s10::f_003135b0__generated, anim_sequence_advance_track, 400, &|c, seed| {
            let mut r = Rng(0xadfa_7ace ^ (seed as u64) << 12);
            scramble(c, &mut r);
            let s = DATA;
            sequence(c, &mut r, s);
            let index = [0u32, 0, 1, 2][seed as usize % 4];
            let t = s + index * 0x1c;
            let length = pick(&mut r, &[2.0f32, 1.0, 0.5]);
            c.bus.write32(t + 0x10, f(length));
            // loop mode: 0 clamps, anything else loops
            c.bus.write32(t + 0x1c, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
            let old = pick(&mut r, &[0.5f32, 0.0, 0.4, 5.0, -3.0, -0.5, 1.9]);
            c.bus.write32(t + 0x8, f(old * length));
            let dt = pick(&mut r, &[0.1f32, -0.7, 2.6, 7.0, -5.0, 0.0, 1.5, -2.0, 3.0, -0.25, -3.5]);
            if seed % 7 == 3 {
                c.bus.write32(t + 0xc, r.edge_f32());
                c.bus.write32(s + 0x90, r.edge_f32());
                c.f[12] = r.edge_f32();
            } else {
                c.bus.write32(t + 0xc, f(1.0));
                c.bus.write32(s + 0x90, f(pick(&mut r, &[1.0f32, 0.5, 2.0])));
                c.f[12] = f(dt * length);
            }
            c.bus.write32(s + 0xc4, (seed % 3 == 0) as u32);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = index as u64;
        });
    }

    // ------------------------------------------------------------ AnimDataBase

    /// The jump tables and the epsilon `AnimDataBase_EvalChanCurves` reads.
    fn eval_constants(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        put_words(c, eval::LERP_TABLE_AT, &eval::LERP_TABLE);
        put_words(c, eval::KEY_TABLE_AT, &eval::KEY_TABLE);
        c.bus.write32(GAME_GP - 0x3228, f(0.01));
    }

    /// Curve records at `recs` (kinds 0..10, 10 = none) and keys at `keys`.
    fn curves(c: &mut Cpu, r: &mut Rng, recs: u32, keys: u32, n: u32) {
        fill(c, r, keys, 0x100);
        for k in 0..n {
            let rec = recs + k * 0x14;
            let kind = r.next_u32() % 11;
            c.bus.write32(rec, kind);
            for i in 1..5 {
                c.bus.write32(rec + 4 * i, r.f32(4.0));
            }
            if kind >= 6 {
                c.bus.write32(rec + 0xc, r.next_u32() % 8);
            }
            if kind == 5 && r.next_u32() % 2 == 0 {
                c.bus.write32(rec + 0x10, f((r.next_u32() % 16) as f32));
            }
        }
    }

    /// A decoded channel at `ch`: records at `recs`, key tables in `keys`.
    fn channel(c: &mut Cpu, r: &mut Rng, ch: u32, recs: u32, keys: u32) {
        c.bus.write32(ch, recs);
        c.bus.write32(ch + 0x4, keys);
        c.bus.write32(ch + 0x8, keys + 0x100);
        c.bus.write32(ch + 0xc, keys + 0x200);
        c.bus.write32(ch + 0x10, keys + 0x300);
        for i in 0..4 {
            c.bus.write8(ch + 0x14 + i, (r.next_u32() % 4) as u8);
        }
        fill(c, r, keys, 0x100);
    }

    #[test]
    fn anim_data_base_eval_chan_curves_matches() {
        check(ee::s10::f_00312c20__generated, anim_data_base_eval_chan_curves, 400, &|c, seed| {
            let mut r = Rng(0xe7a1_c0e5 ^ (seed as u64) << 12);
            scramble(c, &mut r);
            eval_constants(c);
            let (ch, recs, keys, out) = (DATA, DATA + 0x100, DATA + 0x1000, DATA + 0x3000);
            channel(c, &mut r, ch, recs, keys);
            curves(c, &mut r, recs, keys + 0x400, 12);
            let frame = (r.next_u32() % 16) as f32;
            let t = match seed % 6 {
                0 => frame - frame % 2.0,                 // on a key and a half-rate key
                1 => frame - frame % 2.0 + 1.0,           // on a key only
                2 => frame + 0.004,                       // within epsilon of a key
                3 => frame + 0.5,
                _ => frame + (r.next_u32() % 1000) as f32 / 1000.0,
            };
            c.f[12] = f(t);
            let first = r.next_u32() % 4;
            c.g[A0][0] = sx(ch);
            c.g[A1][0] = first as u64;
            c.g[A2][0] = (1 + r.next_u32() % 6) as u64;
            c.g[A3][0] = sx(out);
            c.pc = eval::ENTRY;
            if seed % 5 == 4 {
                // re-entry at a loop head after an unwind: the frame is
                // open and the loop state is in the registers
                c.pc = if seed % 2 == 0 { eval::LERP_HEAD } else { eval::KEY_HEAD };
                c.g[SP][0] = sx(crate::difftest::SP - 0x40);
                let cur = recs + first * 0x14;
                c.g[T5][0] = sx(cur);
                c.g[A2][0] = sx(cur + (1 + r.next_u32() % 5) * 0x14);
                c.g[V1][0] = c.bus.read32(cur) as u64;
                for (reg, base) in [(T1, 0u32), (T3, 0x10), (T6, 0x100), (T2, 0x110), (T0, 0x200), (S0, 0x210), (T4, 0x300), (T7, 0x310)] {
                    c.g[reg][0] = sx(keys + base + (r.next_u32() % 8) * 4);
                }
                c.f[4] = r.f32(1.0);
                c.f[5] = r.f32(1.0);
            }
        });
    }

    /// A clip, chunk descriptors, compressed chunk data and two cache banks
    /// (four entries each in a random LRU order) for
    /// `AnimDataBase_SampleChannel`. Case (seed % 5): 0 the cached entry is
    /// the head, 1 the tail, 2 in the middle; 3 a miss evicting the head's
    /// owner, 4 a miss on an unowned head.
    fn sample_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x5a3b_1e00 ^ (seed as u64) << 12);
        scramble(c, &mut r);
        eval_constants(c);
        let (db, anim, clips, table, data) = (DATA, DATA + 0x80, DATA + 0x100, DATA + 0x200, DATA + 0x2000);
        let (recs, keys, out) = (DATA + 0x6000, DATA + 0x7000, DATA + 0x9000);
        c.bus.write32(db + 0x8, clips);
        c.bus.write32(db + 0xc, table);
        c.bus.write32(db + 0x14, data);
        // clip 1: 20 frames in chunks of 8
        let clip = clips + 0x14;
        c.bus.write32(clip + 4, 1);
        c.bus.write16(clip + 0xa, 3);
        c.bus.write16(clip + 0xc, 20);
        c.bus.write16(clip + 0xe, 8);
        c.bus.write32(anim, 1 << 8 | (r.next_u32() & 0xff));
        let frame = (r.next_u32() % 2600) as f32 / 100.0;
        c.bus.write32(anim + 4, f(frame / 30.0));
        curves(c, &mut r, recs, keys + 0x400, 12);
        let case = seed % 5;
        let bank = r.next_u32() % 2;
        // the entries' order in each bank's LRU list
        let rot = r.next_u32() % 4;
        let order: [u32; 4] = if r.next_u32() % 2 == 0 {
            [rot, (rot + 1) % 4, (rot + 2) % 4, (rot + 3) % 4]
        } else {
            [rot, (rot + 3) % 4, (rot + 2) % 4, (rot + 1) % 4]
        };
        let old_owner = DATA + 0x300;
        c.bus.write32(old_owner + 4, r.next_u32());
        for b in 0..2 {
            let pool = DATA + 0x4000 + b * 0x200;
            c.bus.write32(adb::POOL + 4 * b, pool);
            c.bus.write32(adb::HEAD + 4 * b, pool + order[0] * 0x24);
            for i in 0..4 {
                let e = pool + order[i] * 0x24;
                c.bus.write32(e + 0x1c, pool + order[(i + 3) % 4] * 0x24);
                c.bus.write32(e + 0x20, pool + order[(i + 1) % 4] * 0x24);
                c.bus.write32(e, if case == 3 { old_owner } else { 0 });
                channel(c, &mut r, e + 4, recs, keys);
            }
        }
        let slot = match case {
            0 => order[0],
            1 => order[3],
            2 => order[1 + r.next_u32() as usize % 2],
            _ => 0xff,
        };
        for k in 0..8 {
            let d = table + k * 8;
            c.bus.write32(d, k * 0x40);
            c.bus.write8(d + 4, bank as u8);
            c.bus.write8(d + 5, slot as u8);
            c.bus.write16(d + 6, (r.next_u32() % 4) as u16);
            // compressed data: curve count, then kind nibbles (all 0)
            let src = data + k * 0x40;
            c.bus.write8(src, (1 + r.next_u32() % 3) as u8);
            c.bus.write8(src + 1, 0);
            c.bus.write8(src + 2, 0);
        }
        c.g[A0][0] = sx(db);
        c.g[A1][0] = sx(out);
        c.g[A2][0] = sx(anim);
        c.g[A3][0] = (r.next_u32() % 2) as u64;
        c.g[T0][0] = (r.next_u32() % 2) as u64;
        c.g[T1][0] = (1 + r.next_u32() % 6) as u64;
    }

    #[test]
    fn anim_data_base_sample_channel_matches() {
        check(ee::s10::f_00311318__generated, anim_data_base_sample_channel, 300, &sample_setup);
    }

    // ------------------------------------------------------------ UV scroll

    #[test]
    fn uv_scroll_modifier_update_matches() {
        check(ee::s12::f_0035f7d0__generated, uv_scroll_modifier_update, 400, &|c, seed| {
            let mut r = Rng(0x0005_c011 ^ (seed as u64) << 12);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let clock = DATA + 0x400;
            c.bus.write32(GAME_GP + 0x2a74, clock);
            c.bus.write32(clock + 0x10, pick(&mut r, &[30u32, 60, 25]));
            c.bus.write32(GAME_GP - 0x29cc, f(1.0));
            // 4-aligned, sometimes not 8-aligned (the unaligned velocity store)
            let m = DATA + 4 * (seed % 2);
            fill(c, &mut r, m, 0x12);
            let cycle = pick(&mut r, &[0.0f32, 1.0, 0.5, 2.0, -1.0]);
            c.bus.write32(m, pick(&mut r, &[0u32, 2, 6, 6, 1]));
            c.bus.write32(m + 0x8, f(cycle));
            let time = pick(&mut r, &[0.0f32, 0.99, 0.3, 0.7, 1.5, 0.45]) * cycle.abs().max(0.5);
            c.bus.write32(m + 0x4, f(time));
            c.bus.write32(m + 0xc, f(pick(&mut r, &[0.0f32, 0.5, -1.0, 3.0, 0.01])));
            c.bus.write32(m + 0x10, r.f32(2.0));
            c.bus.write32(m + 0x14, f(pick(&mut r, &[0.0f32, 0.3, -0.3, 2.5, -2.5, 1.0])));
            c.bus.write32(m + 0x30, r.f32(1.5));
            c.bus.write32(m + 0x34, r.f32(1.5));
            c.bus.write32(m + 0x38, r.f32(0.6));
            c.bus.write32(m + 0x3c, r.f32(0.6));
            c.bus.write32(m + 0x40, (seed % 3 != 0) as u32);
            c.g[A0][0] = sx(m);
        });
    }

    // ------------------------------------------------------------ lights

    #[test]
    fn world_light_man_light_contribution_matches() {
        check(ee::s09::f_002f5d30__generated, world_light_man_light_contribution, 500, &|c, seed| {
            let mut r = Rng(0x0119_4700 ^ (seed as u64) << 12);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            put_words(c, light::INNER_TABLE_AT, &light::INNER_TABLE);
            put_words(c, light::PENUMBRA_TABLE_AT, &light::PENUMBRA_TABLE);
            // range threshold, attenuation start and slope (both light types)
            for (at, v) in [(0x38b0, 1.0f32), (0x38ac, 2.0), (0x38a8, 0.1), (0x38a4, 1.0), (0x38a0, 2.0), (0x389c, 0.1)] {
                c.bus.write32(GAME_GP - at, f(v));
            }
            let (point, l) = (DATA, DATA + 0x100);
            fill(c, &mut r, l, 0x20);
            c.bus.write32(l + 0x10, pick(&mut r, &[0u32, 1, 1, 1, 1, 2, 2, 3, 0xffff_ffff]));
            c.bus.write32(l + 0x14, r.f32(2.0));
            c.bus.write32(l + 0x18, r.f32(2.0));
            // ranges up to 1000: distances above 100 make the falloff term
            // (100 / d) drop below 1
            c.bus.write32(l + 0x1c, f(pick(&mut r, &[0.5f32, 5.0, 100.0, 40.0, 1000.0, 1000.0])));
            // the light at the origin pointing along -z or +z; the point
            // on the z axis or off it
            let dir = pick(&mut r, &[-1.0f32, 1.0]);
            for (i, v) in [0.0f32, 0.0, dir].iter().enumerate() {
                c.bus.write32(l + 0x2c + 4 * i as u32, f(*v));
                c.bus.write32(l + 0x38 + 4 * i as u32, 0);
            }
            let dist = pick(&mut r, &[0.0f32, 0.5, 1.5, 3.0, 10.0, 60.0, 150.0, 400.0, 250.0]);
            let side = pick(&mut r, &[0.0f32, 0.0, 0.3, 1.0, 5.0]);
            c.bus.write32(point, f(side));
            c.bus.write32(point + 4, 0);
            c.bus.write32(point + 8, f(dist * pick(&mut r, &[1.0f32, -1.0])));
            c.bus.write32(l + 0x5c, f(pick(&mut r, &[0.95f32, 0.5, 0.0, -0.2])));
            c.bus.write32(l + 0x60, f(pick(&mut r, &[0.7f32, 0.2, -1.0, 0.99])));
            c.bus.write8(l + 0x64, pick(&mut r, &[0xffu8, 0, 1, 2, 3, 4]));
            c.bus.write8(l + 0x65, pick(&mut r, &[0xfdu8, 0xff, 0, 1, 2, 3, 4, 5, 7, 12, 0x80]));
            c.g[A0][0] = sx(DATA + 0x200);
            c.g[A1][0] = sx(point);
            c.g[A2][0] = sx(l);
            c.pc = light::ENTRY;
            if seed % 6 == 5 {
                // re-entry at a `^exp` loop head after an unwind
                c.pc = if seed % 4 == 1 { light::POWI_INNER } else { light::POWI_PENUMBRA };
                c.g[SP][0] = sx(crate::difftest::SP - 0x30);
                c.g[S0][0] = sx(l);
                c.g[V1][0] = (1 + r.next_u32() % 40) as u64;
                for i in 0..7 {
                    c.f[i] = r.f32(1.5);
                }
            }
        });
    }
}
