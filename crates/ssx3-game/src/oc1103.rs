//! Staging module oc1103 (wave wave268): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1103`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003c_b4a8,
        snd_dsp_distortion_set_amount,
        "SND_DspDistortion_SetAmount",
    ),
    (
        0x003c_b528,
        snd_dsp_distortion_init,
        "SND_DspDistortion_Init",
    ),
    (
        0x003c_bc00,
        snd_ee_set_output_buffer,
        "SND_EeSetOutputBuffer",
    ),
    (0x003c_bc20, snd_ee_fx_lookup_param, "SND_EeFxLookupParam"),
    (0x003c_f300, snd_dsp_resonator_run, "SND_DspResonator_Run"),
    (0x003d_0b78, path_skip_cond_block, "PATH_SkipCondBlock"),
    (0x003d_0ca0, path_get_track_status, "PATH_GetTrackStatus"),
];

/// Offsets of the distortion filter (`SND_DspDistortion_SetAmount`): the
/// `A[]`, `B[]`, `C[]` tables of 8 shorts each.
const DISTORTION_A: u32 = 0x20;
const DISTORTION_B: u32 = 0x30;
const DISTORTION_C: u32 = 0x40;

/// `void SND_DspDistortion_SetAmount(DistortionState *f, int x)` (0x3cb4a8):
/// fills the eight entries of the A/B/C tables from the amount `x`.
pub fn snd_dsp_distortion_set_amount(c: &mut Cpu) {
    c.cycles += 17;
    let filter = r32(c, A0);
    c.g[V0][0] = 0x7d00;
    c.g[A2][0] = 0x7fff;
    // clipLevel = (x * 0x7d00) >> 15 (the generated mult writes rd = a1)
    let x = r32(c, A1);
    c.g[A1][0] = sx(abi::mult(c, x, 0x7d00));
    c.g[V1][0] = 0x3f_de00;
    c.g[A3][0] = sx(filter.wrapping_add(0x42));
    c.g[T0][0] = 0x6;
    c.g[A1][0] = abi::sra(c.g[A1][0], 15);
    // ceiling = 0x7fff - clipLevel
    c.g[A2][0] = abi::subu(c.g[A2][0], c.g[A1][0]);
    c.g[V0][0] = abi::sll(c.g[A2][0], 16);
    // A[0] = clipLevel - 0x8000
    c.g[A1][0] = abi::subu(c.g[A1][0], 0x8000);
    c.g[V0][0] = abi::sra(c.g[V0][0], 16);
    let a0v = c.g[A1][0];
    Addr(filter.wrapping_add(DISTORTION_A)).sh(c, a0v);
    // C[0] = 0x3fde00 / (short)ceiling
    let (num, den) = (r32(c, V1), r32(c, V0));
    abi::div(c, num, den);
    let ceiling = c.g[A2][0];
    Addr(filter.wrapping_add(DISTORTION_B)).sh(c, ceiling);
    c.cycles += 3;
    c.g[V1][0] = c.lo[0];
    let c0 = c.g[V1][0];
    Addr(filter.wrapping_add(DISTORTION_C)).sh(c, c0);
    // for (i = 1; i < 8; i++) copy A[0], B[0], C[0] into entry i
    loop {
        c.cycles += 9;
        c.g[V0][0] = Addr(filter.wrapping_add(DISTORTION_A)).lhu(c);
        c.g[T0][0] = abi::subu(c.g[T0][0], 1);
        let a_val = c.g[V0][0];
        Addr(r32(c, A3).wrapping_sub(0x20)).sh(c, a_val);
        c.g[V1][0] = Addr(filter.wrapping_add(DISTORTION_B)).lhu(c);
        let b_val = c.g[V1][0];
        Addr(r32(c, A3).wrapping_sub(0x10)).sh(c, b_val);
        c.g[V0][0] = Addr(filter.wrapping_add(DISTORTION_C)).lhu(c);
        let c_val = c.g[V0][0];
        Addr(r32(c, A3)).sh(c, c_val);
        let more = (c.g[T0][0] as i64) >= 0;
        c.g[A3][0] = abi::addu(c.g[A3][0], 2);
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_b4f8);
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `void SND_DspDistortion_Init(DistortionState *f)` (0x3cb528): stores the
/// address of the distortion process (0x3cb328) in the node's first word.
pub fn snd_dsp_distortion_init(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = abi::subu(0x3d_0000, 0x4cd8);
    let v = c.g[V0][0];
    Addr(r32(c, A0)).sw(c, v);
    ret(c);
}

/// `void SND_EeSetOutputBuffer(u32 buf)` (0x3cbc00): stores the buffer
/// pointer in the mixer block's wet-bus word at 0x515b54.
pub fn snd_ee_set_output_buffer(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x51_0000;
    let addr = r32(c, V0).wrapping_add(0x5b54);
    let buf = c.g[A0][0];
    Addr(addr).sw(c, buf);
    ret(c);
}

/// `int SND_EeFxLookupParam(int effectChannels, int mixerChannels,
/// int channelCode)` (0x3cbc20): one byte from the 3-D parameter table at
/// 0x44e770 (dimensions 6 x 0x24 bytes; the generated index is folded).
pub fn snd_ee_fx_lookup_param(c: &mut Cpu) {
    c.cycles += 14;
    c.g[V0][0] = 0x6;
    c.g[A1][0] = abi::subu(c.g[A1][0], 1);
    let a1 = r32(c, A1);
    c.g[A1][0] = sx(abi::mult(c, a1, 6));
    c.g[A0][0] = abi::subu(c.g[A0][0], 1);
    c.g[V1][0] = 0x24;
    c.g[V0][0] = 0x45_0000;
    let a0 = r32(c, A0);
    c.g[A0][0] = sx(abi::mult(c, a0, 0x24));
    c.g[V0][0] = abi::subu(c.g[V0][0], 0x1890);
    c.g[A1][0] = abi::subu(c.g[A1][0], 1);
    c.g[A2][0] = abi::addu(c.g[A2][0], c.g[A1][0]);
    c.g[A2][0] = abi::addu(c.g[A2][0], c.g[A0][0]);
    c.g[A2][0] = abi::addu(c.g[A2][0], c.g[V0][0]);
    c.g[V0][0] = Addr(r32(c, A2)).lbu(c);
    ret(c);
}

/// FPU register `n` as a value.
fn resonator_fpu(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

/// Store an FPU register.
fn resonator_set_f(c: &mut Cpu, n: usize, v: FpuF32) {
    c.f[n] = v.0;
}

/// `void SND_DspResonator_Run(RESONSTATE *s, int n, float *in, float *out)`
/// (0x3cf300): a two-pole resonator over `n` samples. When the resonance is
/// in range (bandwidth below 2*rate/pi, 0 < frequency < rate/2) the output is
/// `a0 * (in + 1e-20) + 2 r cos(theta) y1 - r^2 y2`; otherwise the input is
/// copied through. Output and loop state go back into the struct.
pub fn snd_dsp_resonator_run(c: &mut Cpu) {
    c.cycles += 12;
    let state = r32(c, A0);
    c.g[V0][0] = Addr(state.wrapping_add(0x1c)).lw(c);
    c.g[AT][0] = 0x4049_0fdb; // 3.1415927f
    let pi = FpuF32(r32(c, AT));
    resonator_set_f(c, 2, pi);
    // (float)rate, 2 * rate / pi, bandwidth below it?
    let rate = FpuF32::from_i32(r32(c, V0) as i32);
    resonator_set_f(c, 0, rate);
    let two_rate = resonator_fpu(c, 0) + resonator_fpu(c, 0);
    resonator_set_f(c, 0, two_rate);
    let limit = resonator_fpu(c, 0).div(resonator_fpu(c, 2));
    resonator_set_f(c, 0, limit);
    let bw = Addr(state.wrapping_add(0x24)).read_fpu(c);
    resonator_set_f(c, 1, bw);
    let bw_ok = {
        let (a, b) = (resonator_fpu(c, 1), resonator_fpu(c, 0));
        abi::c_lt_s(c, a, b)
    };
    let mut resonant = false;
    if bw_ok {
        c.cycles += 6;
        let freq = Addr(state.wrapping_add(0x20)).read_fpu(c);
        resonator_set_f(c, 1, freq);
        resonator_set_f(c, 0, FpuF32(0));
        let freq_pos = {
            let (a, b) = (resonator_fpu(c, 0), resonator_fpu(c, 1));
            abi::c_lt_s(c, a, b)
        };
        c.g[V0][0] = abi::sra(c.g[V0][0], 1);
        if freq_pos {
            c.cycles += 5;
            let half = FpuF32::from_i32(r32(c, V0) as i32);
            resonator_set_f(c, 0, half);
            let below_half = {
                let (a, b) = (resonator_fpu(c, 1), resonator_fpu(c, 0));
                abi::c_lt_s(c, a, b)
            };
            if below_half {
                resonant = true;
            }
        }
    }
    if resonant {
        c.cycles += 2;
        if (c.g[A1][0] as i64) > 0 {
            c.cycles += 3;
            c.g[AT][0] = 0x1e3c_e508; // 1e-20f
            let tiny = FpuF32(r32(c, AT));
            resonator_set_f(c, 7, tiny);
            loop {
                c.cycles += 24;
                let st = r32(c, A0);
                let a0 = Addr(st.wrapping_add(0x28)).read_fpu(c);
                resonator_set_f(c, 0, a0);
                c.g[A1][0] = abi::subu(c.g[A1][0], 1);
                let input = Addr(r32(c, A2)).read_fpu(c);
                resonator_set_f(c, 3, input);
                let r2 = resonator_fpu(c, 0) + resonator_fpu(c, 0);
                resonator_set_f(c, 2, r2);
                let costh = Addr(st.wrapping_add(0x2c)).read_fpu(c);
                resonator_set_f(c, 5, costh);
                let in_eps = resonator_fpu(c, 3) + resonator_fpu(c, 7);
                resonator_set_f(c, 3, in_eps);
                let y1 = Addr(st.wrapping_add(0x38)).read_fpu(c);
                resonator_set_f(c, 6, y1);
                let gain = Addr(st.wrapping_add(0x30)).read_fpu(c);
                resonator_set_f(c, 1, gain);
                let rr = resonator_fpu(c, 0) * resonator_fpu(c, 0);
                resonator_set_f(c, 0, rr);
                let r2c = resonator_fpu(c, 2) * resonator_fpu(c, 5);
                resonator_set_f(c, 2, r2c);
                let y2 = Addr(st.wrapping_add(0x3c)).read_fpu(c);
                resonator_set_f(c, 4, y2);
                let scaled = resonator_fpu(c, 1) * resonator_fpu(c, 3);
                resonator_set_f(c, 1, scaled);
                c.g[A2][0] = abi::addu(c.g[A2][0], 4);
                let rry2 = resonator_fpu(c, 0) * resonator_fpu(c, 4);
                resonator_set_f(c, 0, rry2);
                let r2cy1 = resonator_fpu(c, 2) * resonator_fpu(c, 6);
                resonator_set_f(c, 2, r2cy1);
                let sum = resonator_fpu(c, 1) + resonator_fpu(c, 2);
                resonator_set_f(c, 1, sum);
                let out = resonator_fpu(c, 1) - resonator_fpu(c, 0);
                resonator_set_f(c, 1, out);
                Addr(r32(c, A3)).write_fpu(c, out);
                let y1_new = Addr(st.wrapping_add(0x38)).read_fpu(c);
                resonator_set_f(c, 0, y1_new);
                Addr(st.wrapping_add(0x3c)).write_fpu(c, y1_new);
                let stored = Addr(r32(c, A3)).read_fpu(c);
                resonator_set_f(c, 1, stored);
                c.g[A3][0] = abi::addu(c.g[A3][0], 4);
                let more = c.g[A1][0] != 0;
                Addr(st.wrapping_add(0x38)).write_fpu(c, stored);
                if !more {
                    c.cycles += 2;
                    ret(c);
                    return;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_f370);
                }
            }
        }
    }
    // Copy the input through (the fallback path).
    if !resonant {
        c.cycles += 2;
    }
    if (c.g[A1][0] as i64) > 0 {
        loop {
            c.cycles += 7;
            let sample = Addr(r32(c, A2)).read_fpu(c);
            resonator_set_f(c, 0, sample);
            c.g[A1][0] = abi::subu(c.g[A1][0], 1);
            c.g[A2][0] = abi::addu(c.g[A2][0], 4);
            Addr(r32(c, A3)).write_fpu(c, sample);
            let more = c.g[A1][0] != 0;
            c.g[A3][0] = abi::addu(c.g[A3][0], 4);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_f3e0);
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `void PATH_SkipCondBlock(PATHAction *a, int n, PATHAction *end)`
/// (0x3d0b78): walks action records (16 bytes, type byte at +9, flags at +8)
/// from `a + 0x10` up to `end`, skipping conditional blocks until their
/// matching end, and sets the done bit on the way out.
pub fn path_skip_cond_block(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A0][0] = abi::addu(c.g[A0][0], 0x10);
    c.g[V0][0] = abi::sltu(c.g[A2][0], c.g[A0][0]);
    c.g[A3][0] = 0;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        let negative = (c.g[A1][0] as i64) < 0;
        c.g[T4][0] = 0x1;
        if !negative {
            c.cycles += 6;
            c.g[T3][0] = 0x10_0000;
            c.g[T0][0] = 0;
            c.g[T2][0] = 0x20_0000;
            c.g[T1][0] = 0x400_0000;
            c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x9)).lbu(c);
            loop {
                'record: {
                    c.cycles += 2;
                    let not_cond = c.g[V0][0] != c.g[T4][0];
                    c.g[V1][0] = 0x30_0000;
                    if not_cond {
                        break 'record;
                    }
                    'masked: {
                        c.cycles += 4;
                        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x8)).lw(c);
                        c.g[V0][0] &= c.g[V1][0];
                        if c.g[V0][0] != c.g[T3][0] {
                            break 'masked;
                        }
                        c.cycles += 2;
                        c.g[A3][0] = abi::addu(c.g[A3][0], 1);
                        break 'record;
                    }
                    'end_test: {
                        c.cycles += 2;
                        if c.g[V0][0] == c.g[V1][0] {
                            c.g[V0][0] = c.g[A3][0];
                            break 'end_test;
                        }
                        c.cycles += 2;
                        let not_t2 = c.g[V0][0] != c.g[T2][0];
                        c.g[V0][0] = c.g[A3][0];
                        if not_t2 {
                            break 'record;
                        }
                    }
                    c.cycles += 4;
                    c.g[V1][0] = abi::subu(c.g[A1][0], 1);
                    c.g[V0][0] = abi::slt(c.g[T0][0], c.g[V0][0]);
                    c.g[A3][0] = abi::subu(c.g[A3][0], 1);
                    if c.g[V0][0] == 0 {
                        c.g[A1][0] = c.g[V1][0];
                    }
                }
                c.cycles += 2;
                if (c.g[A1][0] as i64) > 0 {
                    c.g[A0][0] = abi::addu(c.g[A0][0], 0x10);
                } else {
                    c.cycles += 4;
                    let flags = Addr(r32(c, A0).wrapping_add(0x8)).lw(c);
                    let set = flags | c.g[T1][0];
                    c.g[V0][0] = set;
                    Addr(r32(c, A0).wrapping_add(0x8)).sw(c, set);
                    c.g[A0][0] = abi::addu(c.g[A0][0], 0x10);
                }
                c.cycles += 3;
                c.g[V0][0] = abi::sltu(c.g[A2][0], c.g[A0][0]);
                if c.g[V0][0] != 0 {
                    break;
                }
                c.cycles += 2;
                if (c.g[A1][0] as i64) < 0 {
                    break;
                }
                c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x9)).lbu(c);
                if event_due(c) {
                    deliver_event_here(c, 0x003d_0ba8);
                }
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `void PATH_SkipRemainingActions(PATHAction *a, PATHAction *end)`
/// (0x3d0c20): sets the done bit (0x4000000) of each action record from
/// `a + 0x10` up to `end`, stopping at the first record of another type.
#[allow(dead_code)]
pub fn path_skip_remaining_actions(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A0][0] = abi::addu(c.g[A0][0], 0x10);
    c.g[V0][0] = abi::sltu(c.g[A1][0], c.g[A0][0]);
    let past_end = c.g[V0][0] != 0;
    c.g[V0][0] = 0x1;
    if !past_end {
        c.cycles += 3;
        c.g[V1][0] = Addr(r32(c, A0).wrapping_add(0x9)).lbu(c);
        let first_is_one = c.g[V1][0] == c.g[V0][0];
        c.g[A2][0] = 0x400_0000;
        if !first_is_one {
            c.cycles += 3;
            c.g[V1][0] = 0x1;
            c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x8)).lw(c);
            loop {
                c.cycles += 6;
                c.g[V0][0] |= c.g[A2][0];
                let done = c.g[V0][0];
                Addr(r32(c, A0).wrapping_add(0x8)).sw(c, done);
                c.g[A0][0] = abi::addu(c.g[A0][0], 0x10);
                if abi::sltu(c.g[A1][0], c.g[A0][0]) != 0 {
                    break;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x9)).lbu(c);
                if c.g[V0][0] == c.g[V1][0] {
                    break;
                }
                c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x003d_0c48);
                }
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `int PATH_GetTrackStatus(int id)` (0x3d0ca0): the status of the track
/// `PATH_FindTrack(id)` finds.
pub fn path_get_track_status(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee {
            addr: 0x003d_5290,
            f: ee::s14::PATH_FindTrack,
        }
        .call(c, 0x003d_0cb0)?;
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x003d_0cc8,
            f: ee::s14::PATH_TrackGetStatus,
        }
        .call(c, 0x003d_0cb8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, check_lohi, fill, Rng, DATA};

    /// Action records for the path walkers: 16 bytes each, a flag word at +8
    /// (one of the conditional masks or random) and a type byte at +9 that is
    /// often 1 (the conditional type).
    fn path_records(c: &mut Cpu, r: &mut Rng, base: u32, count: u32) {
        for k in 0..count {
            let rec = base + 0x10 * k;
            let flags = match r.next_u32() % 5 {
                0 => 0x10_0000,
                1 => 0x20_0000,
                2 => 0x30_0000,
                3 => 0,
                _ => r.next_u32(),
            };
            c.bus.write32(rec + 8, flags);
            let kind = if r.next_u32() % 3 == 0 {
                1
            } else {
                (r.next_u32() % 4) as u8
            };
            c.bus.write8(rec + 9, kind);
        }
    }

    #[test]
    fn snd_dsp_distortion_set_amount_matches() {
        check_lohi(
            ee::s14::f_003cb4a8__generated,
            snd_dsp_distortion_set_amount,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3cb4a8, 0);
                c.g[A0][0] = sx(DATA);
                let x = match seed % 4 {
                    0 => 0x7fff,
                    1 => r.next_u32() % 0x1_0000,
                    _ => r.next_u32(),
                };
                c.g[A1][0] = sx(x);
                fill(c, &mut r, DATA, 0x20);
            },
        );
    }

    #[test]
    fn snd_dsp_distortion_init_matches() {
        check(
            ee::s14::f_003cb528__generated,
            snd_dsp_distortion_init,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3cb528, 0);
                let node = DATA + 0x100 + 0x10 * (seed % 4);
                c.g[A0][0] = sx(node);
                c.bus.write32(node, r.next_u32());
            },
        );
    }

    #[test]
    fn snd_ee_set_output_buffer_matches() {
        check(
            ee::s14::f_003cbc00__generated,
            snd_ee_set_output_buffer,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3cbc00, 0);
                c.g[A0][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn snd_ee_fx_lookup_param_matches() {
        check_lohi(
            ee::s14::f_003cbc20__generated,
            snd_ee_fx_lookup_param,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3cbc20, 0);
                // Table bytes around the lookup window (0x44e770 +/- the index).
                fill(c, &mut r, 0x44_e700, 0x40);
                c.g[A0][0] = sx(seed % 7);
                c.g[A1][0] = sx((seed / 7) % 7);
                c.g[A2][0] = sx((seed / 49) % 9);
            },
        );
    }

    #[test]
    fn snd_dsp_resonator_run_matches() {
        check(
            ee::s14::f_003cf300__generated,
            snd_dsp_resonator_run,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3cf300, 0);
                let st = DATA;
                c.g[A0][0] = sx(st);
                c.g[A1][0] = sx(if seed % 4 == 0 { 0 } else { 1 + seed % 9 });
                c.g[A2][0] = sx(DATA + 0x200);
                c.g[A3][0] = sx(DATA + 0x400);
                c.bus.write32(st + 0x1c, 44_100);
                // bandwidth: in range most of the time, out of range one time in five
                let bw = if seed % 5 == 0 {
                    1.0e9f32
                } else {
                    (r.next_u32() % 500) as f32
                };
                c.bus.write32(st + 0x24, bw.to_bits());
                // resonant frequency: negative, above rate/2, or in range
                let freq = match seed % 5 {
                    1 => -1.0f32,
                    2 => 30_000.0f32,
                    _ => 100.0 + (r.next_u32() % 20_000) as f32,
                };
                c.bus.write32(st + 0x20, freq.to_bits());
                let unit = |r: &mut Rng| (r.next_u32() % 100) as f32 / 100.0;
                c.bus.write32(st + 0x28, unit(&mut r).to_bits());
                c.bus.write32(st + 0x2c, (unit(&mut r) - 0.5).to_bits());
                c.bus
                    .write32(st + 0x30, ((r.next_u32() % 100) as f32 / 50.0).to_bits());
                c.bus
                    .write32(st + 0x38, (unit(&mut r) * 2.0 - 1.0).to_bits());
                c.bus
                    .write32(st + 0x3c, (unit(&mut r) * 2.0 - 1.0).to_bits());
                for k in 0..16 {
                    let v = ((r.next_u32() % 2000) as f32) / 100.0 - 10.0;
                    c.bus.write32(DATA + 0x200 + 4 * k, v.to_bits());
                }
            },
        );
    }

    #[test]
    fn path_skip_cond_block_matches() {
        check(
            ee::s14::f_003d0b78__generated,
            path_skip_cond_block,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d0b78, 0);
                let count = seed % 9;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(match seed % 4 {
                    0 => 0xffff_ffff,
                    1 => 0,
                    2 => 3,
                    _ => r.next_u32() % 6,
                });
                c.g[A2][0] = sx(DATA + 0x10 * count);
                path_records(c, &mut r, DATA, 10);
            },
        );
    }

    #[test]
    fn path_get_track_status_matches() {
        // Real callees (FindTrack, TrackGetStatus) on a machine with empty
        // track slots: the null-track path, plus the call protocol.
        check(
            ee::s14::f_003d0ca0__generated,
            path_get_track_status,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d0ca0, 0);
                c.g[A0][0] = sx(r.next_u32());
            },
        );
    }
}
