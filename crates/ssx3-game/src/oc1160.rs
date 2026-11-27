//! Staging module oc1160 (wave wave273): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1160`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_eed0, snd_dsp_fir_lowpass_set_cutoff, "SND_DspFirLowpass_SetCutoff"),
    (0x003c_ef20, snd_dsp_fir_bandpass_process, "SND_DspFirBandpass_Process"),
    (0x003c_efb0, snd_dsp_fir_bandpass_init, "SND_DspFirBandpass_Init"),
    (0x003c_eff8, snd_dsp_fir_bandpass_set_range, "SND_DspFirBandpass_SetRange"),
    (0x003c_f060, snd_dsp_gain_process, "SND_DspGain_Process"),
    (0x003c_f128, snd_dsp_gain_init, "SND_DspGain_Init"),
    (0x003c_f158, snd_dsp_gain_set_gain, "SND_DspGain_SetGain"),
    (0x003c_f178, snd_dsp_sum_process, "SND_DspSum_Process"),
];

/// `void SFILTER_modifyLPFFIR8(LPFSTATE* self, int* params)`
/// (SND_DspFirLowpass_SetCutoff, 0x3ceed0): the low-pass cutoff becomes
/// `2 * (params[0] >> 8) / (params[1] >> 8)` (float), stored at `self + 0x118`,
/// and the filter at `self + 0x20` is redesigned by `SND_FirDesign` (type 2,
/// the high cut).
pub fn snd_dsp_fir_lowpass_set_cutoff(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, A2);
        c.g[A0][0] = sx(this.wrapping_add(0x20));
        let params = r32(c, A1);
        let p0 = Addr(params).lw(c);
        let p1 = Addr(params.wrapping_add(0x4)).lw(c);
        c.g[V0][0] = sx(((p0 as u32 as i32) >> 8) as u32);
        c.g[A1][0] = 2;
        let twice_high = FpuF32::from_i32(c.g[V0][0] as u32 as i32);
        c.f[0] = twice_high.0;
        c.g[V1][0] = sx(((p1 as u32 as i32) >> 8) as u32);
        let span = FpuF32::from_i32(c.g[V1][0] as u32 as i32);
        c.f[1] = span.0;
        let doubled = twice_high + twice_high;
        c.f[0] = doubled.0;
        let cutoff = doubled.div(span);
        c.f[0] = cutoff.0;
        c.g[RA][0] = sx(0x003c_ef14);
        Addr(this.wrapping_add(0x118)).write_fpu(c, cutoff);
        Callee {
            addr: 0x003c_e7c0,
            f: ee::s14::SND_FirDesign,
        }
        .call(c, 0x003c_ef14)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SFILTER_bpfFIR8(BPFSTATE* node, int count, int src, int dst)`
/// (SND_DspFirBandpass_Process, 0x3cef20). The upstream stage
/// `node->sfn.pfnnext` (when present) runs over the samples first; its result
/// replaces the count, and a result <= 0 ends the call with that result.
/// Then the band-pass FIR at `node + 0x20` filters `src` into `dst`
/// (`SND_FirProcess`). Returns the sample count.
pub fn snd_dsp_fir_bandpass_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        let node = r32(c, S1);
        c.g[A0][0] = Addr(node.wrapping_add(0x8)).lw(c);
        c.g[S3][0] = c.g[A3][0];
        let run_fir = if c.g[A0][0] != 0 {
            c.cycles += 8;
            c.g[V0][0] = Addr(r32(c, A0)).lw(c);
            c.g[A2][0] = c.g[S3][0];
            c.g[T0][0] = Addr(node.wrapping_add(0x1a)).lbu(c);
            let target = r32(c, V0);
            c.g[A3][0] = c.g[S2][0];
            call_ptr(c, target, 0x003c_ef60)?;
            c.g[S0][0] = c.g[V0][0];
            let positive = (c.g[S0][0] as i64) > 0;
            c.g[A0][0] = sx(node.wrapping_add(0x20));
            if !positive {
                c.cycles += 2;
                frame.restore(c, S0, 0x40);
            }
            positive
        } else {
            c.cycles += 2;
            c.g[S0][0] = c.g[A1][0];
            c.g[A0][0] = sx(node.wrapping_add(0x20));
            true
        };
        if run_fir {
            c.cycles += 6;
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S0][0];
            Callee {
                addr: 0x003c_ebe8,
                f: ee::s14::SND_FirProcess,
            }
            .call(c, 0x003c_ef8c)?;
            c.g[V0][0] = c.g[S0][0];
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SFILTER_createBPFFIR8(BPFSTATE* self)` (SND_DspFirBandpass_Init,
/// 0x3cefb0): the node's process entry (`SND_DspFirBandpass_Process`, 0x3cef20),
/// no upstream or downstream stage, the FIR state reset by `SND_FirReset`.
/// Returns 0.
pub fn snd_dsp_fir_bandpass_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x003d_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = c.g[A0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x10e0));
        let node = r32(c, V0);
        c.g[A0][0] = sx(node.wrapping_add(0x20));
        let process = c.g[V1][0];
        Addr(node).sw(c, process);
        Addr(node.wrapping_add(0x8)).sw(c, 0);
        Addr(node.wrapping_add(0xc)).sw(c, 0);
        Addr(node.wrapping_add(0x10)).sw(c, 0);
        Addr(node.wrapping_add(0x14)).sw(c, 0);
        Addr(node.wrapping_add(0x1b)).sb(c, 0);
        Addr(node.wrapping_add(0x4)).sw(c, 0);
        Callee {
            addr: 0x003c_eb90,
            f: ee::s14::SND_FirReset,
        }
        .call(c, 0x003c_efe8)?;
        c.g[V0][0] = 0;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SFILTER_modifyBPFFIR8(BPFSTATE* self, int* params)`
/// (SND_DspFirBandpass_SetRange, 0x3ceff8): the band edges are
/// `params[0] >> 7` and `params[1] >> 7` divided by `params[2] >> 8` (float),
/// stored at `self + 0x114` (low) and `self + 0x118` (high), and the filter at
/// `self + 0x20` is redesigned by `SND_FirDesign` (type 4, the band-pass).
pub fn snd_dsp_fir_bandpass_set_range(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[T0][0] = c.g[A0][0];
        let node = r32(c, T0);
        c.g[A0][0] = sx(node.wrapping_add(0x20));
        c.g[A1][0] = 4;
        let params = r32(c, V0);
        c.g[A2][0] = Addr(params.wrapping_add(0x8)).lw(c);
        c.g[A3][0] = Addr(params).lw(c);
        c.g[V1][0] = Addr(params.wrapping_add(0x4)).lw(c);
        c.g[A2][0] = sx(((c.g[A2][0] as u32 as i32) >> 8) as u32);
        c.g[A3][0] = sx(((c.g[A3][0] as u32 as i32) >> 7) as u32);
        let divisor = FpuF32::from_i32(c.g[A2][0] as u32 as i32);
        c.f[2] = divisor.0;
        c.g[V1][0] = sx(((c.g[V1][0] as u32 as i32) >> 7) as u32);
        let x = FpuF32::from_i32(c.g[A3][0] as u32 as i32);
        c.f[0] = x.0;
        let y = FpuF32::from_i32(c.g[V1][0] as u32 as i32);
        c.f[1] = y.0;
        let low = x.div(divisor);
        c.f[0] = low.0;
        Addr(node.wrapping_add(0x114)).write_fpu(c, low);
        let high = y.div(divisor);
        c.f[1] = high.0;
        Addr(node.wrapping_add(0x118)).write_fpu(c, high);
        Callee {
            addr: 0x003c_e7c0,
            f: ee::s14::SND_FirDesign,
        }
        .call(c, 0x003c_f054)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SFILTER_amplf`-style gain stage (SND_DspGain_Process, 0x3cf060): the
/// upstream stage (`node->sfn.pfnnext`, when present) runs first and a result
/// <= 0 ends the call with that result. Otherwise every 16-sample block of
/// `src` is scaled by the gain at `node + 0x1c` into `dst` (VU0 `mul` by the
/// broadcast gain). Returns the sample count.
pub fn snd_dsp_gain_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        let node = r32(c, S0);
        c.g[A0][0] = Addr(node.wrapping_add(0x8)).lw(c);
        let chained = c.g[A0][0] != 0;
        c.g[S3][0] = c.g[A2][0];
        let mut early = false;
        if chained {
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, A0)).lw(c);
            c.g[T0][0] = Addr(node.wrapping_add(0x1a)).lbu(c);
            c.g[A2][0] = c.g[S2][0];
            let target = r32(c, V0);
            c.g[A3][0] = c.g[S3][0];
            call_ptr(c, target, 0x003c_f0a4)?;
            if (c.g[V0][0] as i64) <= 0 {
                frame.restore(c, S0, 0x40);
                early = true;
            }
        }
        if !early {
            c.cycles += 8;
            c.f[0] = Addr(node.wrapping_add(0x1c)).read_u32(c);
            c.g[V0][0] = sx(c.f[0]);
            c.g[A0][0] = c.g[S1][0];
            c.qmtc2(V0, 1);
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = c.g[S2][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            loop {
                c.cycles += 15;
                let src = r32(c, A2);
                c.lqc2(2, src);
                c.lqc2(3, src.wrapping_add(0x10));
                c.lqc2(4, src.wrapping_add(0x20));
                c.lqc2(5, src.wrapping_add(0x30));
                c.g[A2][0] = sx(src.wrapping_add(0x40));
                c.vu0().mul(0b1111, 2, 2, Src::Bc(1, X));
                c.vu0().mul(0b1111, 3, 3, Src::Bc(1, X));
                c.vu0().mul(0b1111, 4, 4, Src::Bc(1, X));
                c.vu0().mul(0b1111, 5, 5, Src::Bc(1, X));
                let dst = r32(c, A3);
                c.sqc2(2, dst);
                c.sqc2(3, dst.wrapping_add(0x10));
                c.sqc2(4, dst.wrapping_add(0x20));
                c.sqc2(5, dst.wrapping_add(0x30));
                let more = c.g[A0][0] != c.g[A2][0];
                c.g[A3][0] = sx(dst.wrapping_add(0x40));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_f0cc);
                    }
                    continue;
                }
                c.cycles += 2;
                c.g[V0][0] = c.g[S1][0];
                frame.restore(c, S0, 0x40);
                break;
            }
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SFILTER_createAMPLF(AMPLFSTATE* pas)` (SND_DspGain_Init, 0x3cf128): the
/// node's process entry (`SND_DspGain_Process`, 0x3cf060), no upstream stage,
/// gain 0 and the cleared state words. Leaf: returns 0 in v0.
pub fn snd_dsp_gain_init(c: &mut Cpu) {
    c.cycles += 12;
    let node = r32(c, A0);
    c.g[V0][0] = 0x003d_0000;
    Addr(node.wrapping_add(0x1c)).sw(c, 0);
    c.g[V0][0] = sx(0x003d_0000u32.wrapping_sub(0xfa0));
    Addr(node.wrapping_add(0x8)).sw(c, 0);
    let process = c.g[V0][0];
    Addr(node).sw(c, process);
    c.g[V0][0] = 0;
    Addr(node.wrapping_add(0xc)).sw(c, 0);
    Addr(node.wrapping_add(0x10)).sw(c, 0);
    Addr(node.wrapping_add(0x14)).sw(c, 0);
    Addr(node.wrapping_add(0x1b)).sb(c, 0);
    Addr(node.wrapping_add(0x4)).sw(c, 0);
    ret(c);
}

/// `void SFILTER_modifyAMPLF(AMPLFSTATE* node, int* params)`
/// (SND_DspGain_SetGain, 0x3cf158): the gain is `(float)params[0] / 256`,
/// stored at `node + 0x1c`.
pub fn snd_dsp_gain_set_gain(c: &mut Cpu) {
    c.cycles += 7;
    let params = r32(c, A1);
    let raw = Addr(params).read_fpu(c);
    let level = FpuF32::from_i32(raw.0 as i32);
    let scale = FpuF32(0x3b80_0000);
    c.g[AT][0] = 0x3b80_0000;
    c.f[1] = scale.0;
    let gain = level * scale;
    c.f[0] = gain.0;
    Addr(r32(c, A0).wrapping_add(0x1c)).write_fpu(c, gain);
    ret(c);
}

/// `int SFILTER_mixer(...)` (SND_DspSum_Process, 0x3cf178): the sum stage.
/// The temporary buffer `node + 0x1c` (`tempbufframes` at `node + 0x20`) is
/// regrown with `SND_MemAlloc` (and the old block freed) when it holds fewer
/// samples than `count`. Then the upstream stage `pfnnext` runs into `src`
/// and `pfnnext2` into the temporary buffer; a result <= 0 from either ends
/// the call with that result. Otherwise the temporary buffer is added into
/// `dst` in 16-sample blocks (VU0 `add`). Returns the sample count.
pub fn snd_dsp_sum_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        let node = r32(c, S0);
        c.g[V0][0] = Addr(node.wrapping_add(0x20)).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S1][0] as i64)) as u64;
        c.g[S3][0] = c.g[A3][0];
        if c.g[V0][0] != 0 {
            // Regrow the temporary buffer.
            c.cycles += 3;
            c.g[A0][0] = Addr(node.wrapping_add(0x1c)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                call_ptr(c, 0x003b_5e98, 0x003c_f1c0)?;
            }
            let count = c.g[S1][0];
            Addr(node.wrapping_add(0x20)).sw(c, count);
            c.cycles += 3;
            c.g[A0][0] = sx((c.g[S1][0] as u32) << 2);
            call_ptr(c, 0x003b_5c60, 0x003c_f1cc)?;
            let block = c.g[V0][0];
            Addr(node.wrapping_add(0x1c)).sw(c, block);
        }
        // The two upstream stages.
        c.cycles += 9;
        c.g[A0][0] = Addr(node.wrapping_add(0x8)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[V0][0] = Addr(r32(c, A0)).lw(c);
        c.g[T0][0] = 1;
        let first = r32(c, V0);
        call_ptr(c, first, 0x003c_f1ec)?;
        c.g[A2][0] = c.g[S2][0];
        // A first result <= 0, or the mix's loop exit, reaches the common
        // tail (one more cycle and the restore of s0).
        let mut reached_tail = false;
        if (c.g[V0][0] as i64) <= 0 {
            reached_tail = true;
        } else {
            c.cycles += 8;
            c.g[A0][0] = Addr(node.wrapping_add(0xc)).lw(c);
            c.g[A3][0] = Addr(node.wrapping_add(0x1c)).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[V0][0] = Addr(r32(c, A0)).lw(c);
            c.g[T0][0] = 2;
            let second = r32(c, V0);
            call_ptr(c, second, 0x003c_f20c)?;
            if (c.g[V0][0] as i64) <= 0 {
                frame.restore(c, S0, 0x40);
            } else {
                // Add the temporary buffer into dst, 16 samples per block.
                c.cycles += 6;
                c.g[V0][0] = Addr(node.wrapping_add(0x1c)).lw(c);
                c.g[A0][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                loop {
                    c.cycles += 19;
                    let tmp = r32(c, A1);
                    let dst = r32(c, A2);
                    c.lqc2(2, tmp);
                    c.lqc2(3, tmp.wrapping_add(0x10));
                    c.lqc2(4, tmp.wrapping_add(0x20));
                    c.lqc2(5, tmp.wrapping_add(0x30));
                    c.lqc2(6, dst);
                    c.lqc2(7, dst.wrapping_add(0x10));
                    c.lqc2(8, dst.wrapping_add(0x20));
                    c.lqc2(9, dst.wrapping_add(0x30));
                    c.g[A1][0] = sx(tmp.wrapping_add(0x40));
                    c.vu0().add(0b1111, 6, 6, Src::Vec(2));
                    c.vu0().add(0b1111, 7, 7, Src::Vec(3));
                    c.vu0().add(0b1111, 8, 8, Src::Vec(4));
                    c.vu0().add(0b1111, 9, 9, Src::Vec(5));
                    c.sqc2(6, dst);
                    c.sqc2(7, dst.wrapping_add(0x10));
                    c.sqc2(8, dst.wrapping_add(0x20));
                    c.sqc2(9, dst.wrapping_add(0x30));
                    let more = c.g[A0][0] != c.g[A1][0];
                    c.g[A2][0] = sx(dst.wrapping_add(0x40));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x003c_f22c);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S1][0];
                    break;
                }
                reached_tail = true;
            }
        }
        if reached_tail {
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, fill, Rng, DATA};

    const OC1160_NODE: u32 = DATA + 0x1000;
    const OC1160_PARAMS: u32 = DATA + 0x2000;
    const OC1160_SRC: u32 = DATA + 0x3000;
    const OC1160_DST: u32 = DATA + 0x4000;
    const OC1160_MOCK_RESULTS: u32 = DATA + 0x6000;
    const OC1160_NEW_TEMP: u32 = DATA + 0x9000;
    const OC1160_OBJ: u32 = DATA + 0x8000;
    const OC1160_OBJ2: u32 = DATA + 0xb000;
    const OC1160_OLD_TEMP: u32 = DATA + 0xa000;
    const OC1160_FIR_GATE: u32 = DATA + 0x7000;
    const OC1160_ALLOC_GATE: u32 = DATA + 0x7010;
    const OC1160_ALLOC_RESULT: u32 = DATA + 0x7014;
    const OC1160_FREE_GATE: u32 = DATA + 0x7020;
    const OC1160_GATE_MAGIC: u32 = 0x1160_c0de;
    /// Two entries of the test's vtable callee (the miss hook answers them).
    const OC1160_MOCK_FIRST: u32 = fakes::FAKE + 0x800;
    const OC1160_MOCK_SECOND: u32 = fakes::FAKE + 0x810;

    static OC1160_FIR_PREV: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);
    static OC1160_ALLOC_PREV: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);
    static OC1160_FREE_PREV: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    /// Gated stand-in for `SND_FirDesign` (the filter design is a callee, not
    /// under test): with its gate set it returns at once.
    fn oc1160_fir_design_fake(c: &mut Cpu) {
        if c.bus.read32(OC1160_FIR_GATE) != OC1160_GATE_MAGIC {
            return fakes::fall_back(c, &OC1160_FIR_PREV, ee::s14::f_003ce7c0__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// Gated stand-in for `SND_MemAlloc`: returns the block the test set.
    fn oc1160_alloc_fake(c: &mut Cpu) {
        if c.bus.read32(OC1160_ALLOC_GATE) != OC1160_GATE_MAGIC {
            return fakes::fall_back(c, &OC1160_ALLOC_PREV, ee::s13::f_003b5c60__generated);
        }
        c.g[V0][0] = sx(c.bus.read32(OC1160_ALLOC_RESULT));
        c.pc = c.g[RA][0] as u32;
    }

    /// Gated stand-in for `SND_MemFree`: returns at once.
    fn oc1160_free_fake(c: &mut Cpu) {
        if c.bus.read32(OC1160_FREE_GATE) != OC1160_GATE_MAGIC {
            return fakes::fall_back(c, &OC1160_FREE_PREV, ee::s13::f_003b5e98__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    fn oc1160_install_fakes() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003CE7C0),
            oc1160_fir_design_fake,
            &OC1160_FIR_PREV,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003B5C60),
            oc1160_alloc_fake,
            &OC1160_ALLOC_PREV,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003B5E98),
            oc1160_free_fake,
            &OC1160_FREE_PREV,
        );
    }

    /// Calls to the two mock entries return the words at `OC1160_MOCK_RESULTS`
    /// (and `+4`) in v0; other calls go to the default miss hook.
    fn oc1160_mock_miss(c: &mut Cpu, target: u32) {
        let slot = match target {
            OC1160_MOCK_FIRST => 0,
            OC1160_MOCK_SECOND => 4,
            _ => return fakes::fake_call(c, target),
        };
        c.g[V0][0] = sx(c.bus.read32(OC1160_MOCK_RESULTS + slot));
        c.pc = c.g[RA][0] as u32;
    }

    /// Filter parameters: the values shifted by 8 (or 7) must be sane.
    fn oc1160_fill_params(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(OC1160_PARAMS, r.next_u32());
        c.bus.write32(OC1160_PARAMS + 4, r.next_u32());
        c.bus
            .write32(OC1160_PARAMS + 8, ((r.next_u32() % 500) + 1) << 8);
    }

    #[test]
    fn snd_dsp_fir_lowpass_set_cutoff_matches() {
        oc1160_install_fakes();
        check(
            ee::s14::f_003ceed0__generated,
            snd_dsp_fir_lowpass_set_cutoff,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3ceed0, 0);
                c.bus.write32(OC1160_FIR_GATE, OC1160_GATE_MAGIC);
                fill(c, &mut r, OC1160_NODE, 0x80);
                c.g[A0][0] = sx(OC1160_NODE);
                c.g[A1][0] = sx(OC1160_PARAMS);
                c.bus.write32(OC1160_PARAMS, r.next_u32());
                // The divisor (params[1] >> 8) is never zero.
                c.bus
                    .write32(OC1160_PARAMS + 4, ((r.next_u32() % 4000) + 1) << 8);
            },
        );
    }

    #[test]
    fn snd_dsp_fir_bandpass_set_range_matches() {
        oc1160_install_fakes();
        check(
            ee::s14::f_003ceff8__generated,
            snd_dsp_fir_bandpass_set_range,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3ceff8, 0);
                c.bus.write32(OC1160_FIR_GATE, OC1160_GATE_MAGIC);
                fill(c, &mut r, OC1160_NODE, 0x80);
                c.g[A0][0] = sx(OC1160_NODE);
                c.g[A1][0] = sx(OC1160_PARAMS);
                oc1160_fill_params(c, &mut r);
            },
        );
    }

    #[test]
    fn snd_dsp_fir_bandpass_init_matches() {
        check(
            ee::s14::f_003cefb0__generated,
            snd_dsp_fir_bandpass_init,
            100,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3cefb0, 0);
                fill(c, &mut r, OC1160_NODE, 0x80);
                c.g[A0][0] = sx(OC1160_NODE);
            },
        );
    }

    #[test]
    fn snd_dsp_fir_bandpass_process_matches() {
        check(
            ee::s14::f_003cef20__generated,
            snd_dsp_fir_bandpass_process,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3cef20, 0);
                c.hooks.miss = oc1160_mock_miss;
                fill(c, &mut r, OC1160_NODE, 0x80);
                fill(c, &mut r, OC1160_SRC, 0x40);
                let count = 16 * (1 + seed % 3);
                c.g[A0][0] = sx(OC1160_NODE);
                c.g[A1][0] = sx(count);
                c.g[A2][0] = sx(OC1160_SRC);
                c.g[A3][0] = sx(OC1160_DST);
                // The upstream stage is present in three seeds of four.
                let chained = seed % 4 != 0;
                c.bus
                    .write32(OC1160_NODE + 0x8, if chained { OC1160_OBJ } else { 0 });
                c.bus.write32(OC1160_OBJ, OC1160_MOCK_FIRST);
                c.bus.write8(OC1160_NODE + 0x1a, r.next_u32() as u8);
                // Its result: <= 0 ends the call, else the new sample count.
                let result = match seed % 5 {
                    0 => 0,
                    1 => (-1i32) as u32,
                    _ => 16 * (1 + r.next_u32() % 3),
                };
                c.bus.write32(OC1160_MOCK_RESULTS, result);
                // The FIR flag (`SND_FirProcess` takes its first branch when set).
                c.bus.write8(OC1160_NODE + 0x11c, (seed & 1) as u8);
            },
        );
    }

    #[test]
    fn snd_dsp_gain_init_matches() {
        check(
            ee::s14::f_003cf128__generated,
            snd_dsp_gain_init,
            100,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3cf128, 0);
                fill(c, &mut r, OC1160_NODE, 0x80);
                c.g[A0][0] = sx(OC1160_NODE);
            },
        );
    }

    #[test]
    fn snd_dsp_gain_set_gain_matches() {
        check(
            ee::s14::f_003cf158__generated,
            snd_dsp_gain_set_gain,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3cf158, 0);
                fill(c, &mut r, OC1160_NODE, 0x40);
                c.g[A0][0] = sx(OC1160_NODE);
                c.g[A1][0] = sx(OC1160_PARAMS);
                let v = (r.next_u32() % 2001) as i32 - 1000;
                c.bus.write32(OC1160_PARAMS, v as u32);
            },
        );
    }

    #[test]
    fn snd_dsp_gain_process_matches() {
        check(
            ee::s14::f_003cf060__generated,
            snd_dsp_gain_process,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3cf060, 0);
                c.hooks.miss = oc1160_mock_miss;
                fill(c, &mut r, OC1160_NODE, 0x80);
                fill(c, &mut r, OC1160_SRC, 0x40);
                c.g[A0][0] = sx(OC1160_NODE);
                c.g[A1][0] = sx(16 * (1 + seed % 3));
                c.g[A2][0] = sx(OC1160_SRC);
                c.g[A3][0] = sx(OC1160_DST);
                let chained = seed % 4 != 0;
                c.bus
                    .write32(OC1160_NODE + 0x8, if chained { OC1160_OBJ } else { 0 });
                c.bus.write32(OC1160_OBJ, OC1160_MOCK_FIRST);
                c.bus.write8(OC1160_NODE + 0x1a, r.next_u32() as u8);
                let result = match seed % 5 {
                    0 => 0,
                    1 => (-1i32) as u32,
                    _ => r.next_u32() % 100,
                };
                c.bus.write32(OC1160_MOCK_RESULTS, result);
                c.bus.write32(OC1160_NODE + 0x1c, r.f32(4.0));
            },
        );
    }

    #[test]
    fn snd_dsp_sum_process_matches() {
        oc1160_install_fakes();
        check(
            ee::s14::f_003cf178__generated,
            snd_dsp_sum_process,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3cf178, 0);
                c.hooks.miss = oc1160_mock_miss;
                fill(c, &mut r, OC1160_NODE, 0x80);
                fill(c, &mut r, OC1160_SRC, 0x40);
                fill(c, &mut r, OC1160_DST, 0x40);
                fill(c, &mut r, OC1160_OLD_TEMP, 0x40);
                fill(c, &mut r, OC1160_NEW_TEMP, 0x40);
                let count = 16 * (1 + seed % 3);
                c.g[A0][0] = sx(OC1160_NODE);
                c.g[A1][0] = sx(count);
                c.g[A2][0] = sx(OC1160_SRC);
                c.g[A3][0] = sx(OC1160_DST);
                c.bus.write32(OC1160_NODE + 0x8, OC1160_OBJ);
                c.bus.write32(OC1160_OBJ, OC1160_MOCK_FIRST);
                c.bus.write32(OC1160_NODE + 0xc, OC1160_OBJ2);
                c.bus.write32(OC1160_OBJ2, OC1160_MOCK_SECOND);
                // The two stage results: <= 0 ends early, else the mix.
                let first = match seed % 4 {
                    0 => 0,
                    1 => (-1i32) as u32,
                    _ => 16 * (1 + r.next_u32() % 3),
                };
                let second = if seed % 6 == 1 {
                    (-1i32) as u32
                } else {
                    16 * (1 + r.next_u32() % 3)
                };
                c.bus.write32(OC1160_MOCK_RESULTS, first);
                c.bus.write32(OC1160_MOCK_RESULTS + 4, second);
                // The temporary buffer: big enough, or regrown (with or without
                // an old block to free).
                let grow = seed % 3 == 0;
                if grow {
                    c.bus.write32(OC1160_NODE + 0x20, 0);
                } else {
                    c.bus.write32(OC1160_NODE + 0x20, count);
                }
                let old = if !grow || seed % 2 == 0 {
                    OC1160_OLD_TEMP
                } else {
                    0
                };
                c.bus.write32(OC1160_NODE + 0x1c, old);
                c.bus.write32(OC1160_ALLOC_GATE, OC1160_GATE_MAGIC);
                c.bus.write32(OC1160_ALLOC_RESULT, OC1160_NEW_TEMP);
                c.bus.write32(OC1160_FREE_GATE, OC1160_GATE_MAGIC);
            },
        );
    }
}
