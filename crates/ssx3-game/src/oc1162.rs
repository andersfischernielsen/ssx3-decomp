//! Staging module oc1162 (wave wave273): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1162`.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X, Y};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_f740, snd_dsp_buffer_source_process, "SND_DspBufferSource_Process"),
    (0x003c_f7b0, snd_dsp_pass_thru_init, "SND_DspPassThru_Init"),
    (0x003c_f7d0, snd_dsp_allpass_process, "SND_DspAllpass_Process"),
    (0x003c_f958, snd_dsp_allpass_free, "SND_DspAllpass_Free"),
    (0x003c_f998, snd_dsp_allpass_init, "SND_DspAllpass_Init"),
    (0x003c_f9d8, snd_dsp_allpass_set_params, "SND_DspAllpass_SetParams"),
    (0x003c_fa98, snd_sqrt_1p_series, "SND_Sqrt1pSeries"),
    (0x003c_fb38, snd_calc_prime_delay, "SND_CalcPrimeDelay"),
];

// Offsets of the echo effect state (`SFILTER` in the reference) that the
// functions below read and write.
/// `pfnnext`: the chained stage's object (its first word is the callback).
const SFX_NEXT: u32 = 0x08;
/// A byte flag handed to the chained stage (`clean`).
const SFX_CLEAN: u32 = 0x1a;
/// `pdelaybuf`: the 16-byte aligned delay line.
const SFX_DELAY: u32 = 0x1c;
/// `punaligneddelaybuf`: what `SND_MemAlloc` returned for the delay line.
const SFX_RAW: u32 = 0x20;
/// `sampleindex`: the write position in the delay line, in samples.
const SFX_INDEX: u32 = 0x24;
/// `bufsamples`: the delay line's length in samples.
const SFX_SIZE: u32 = 0x28;
/// `fblevels` (the feedback level), with `fbneglevels` at +0x34.
const SFX_LEVELS: u32 = 0x30;

/// `int SND_DspBufferSource_Process(effect *e, int n, int, float *dst)`
/// (0x3cf740, `SFILTER_src`): copies `n` samples from the effect's source
/// buffer (`pdata`) to `dst` and advances `pdata` past them. Returns `n`.
pub fn snd_dsp_buffer_source_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = sx((c.g[S1][0] as u32) << 2);
        c.g[A2][0] = c.g[S2][0];
        c.g[A0][0] = c.g[A3][0];
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(SFX_DELAY).lw(c);
        call_ptr(c, 0x003c_51e8, 0x003c_f770)?;
        let fx = Addr(c.g[S0][0] as u32);
        c.g[V1][0] = fx.off(SFX_DELAY).lw(c);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, 17, 0x20);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
        frame.restore_ra(c, 0);
        let advanced = c.g[V1][0];
        fx.off(SFX_DELAY).sw(c, advanced);
        frame.restore(c, 16, 0x30);
        frame.restore(c, 18, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void SND_DspPassThru_Init(effect *e)` (0x3cf7b0, `SFILTER_createSOURCE`):
/// clears the chain links, the flag and the delay line pointer.
pub fn snd_dsp_pass_thru_init(c: &mut Cpu) {
    c.cycles += 8;
    let fx = Addr(c.g[A0][0] as u32);
    c.g[V0][0] = 0;
    fx.off(SFX_DELAY).sw(c, 0);
    fx.off(0x08).sw(c, 0);
    fx.off(0x0c).sw(c, 0);
    fx.off(0x10).sw(c, 0);
    fx.off(0x14).sw(c, 0);
    fx.off(0x1b).sb(c, 0);
    ret(c);
}

/// `void SND_DspAllpass_Process(effect *e, int n, float *src, float *dst)`
/// (0x3cf7d0, `SFILTER_echo`): the echo effect's process step. The chained
/// stage (the callback at `*e->pfnnext`) runs first and its return value
/// (samples it took) ends the call when it is zero or negative. The rest is
/// blended through the delay line in chunks of at most the line's end: each
/// 16-sample chunk mixes the delayed samples with the input through the
/// 4x4 VU0 block (levels in `fblevels`), writes the mix back to the delay
/// line and to `dst`. Returns `v0 = n` when it runs to the end.
pub fn snd_dsp_allpass_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, 19, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        let fx = Addr(c.g[S0][0] as u32);
        c.g[A0][0] = fx.off(SFX_NEXT).lw(c);
        c.g[S3][0] = c.g[A3][0];
        if c.g[A0][0] != 0 {
            c.cycles += 8;
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
            c.g[T0][0] = fx.off(SFX_CLEAN).lbu(c);
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = c.g[S2][0];
            let callback = c.g[V0][0] as u32;
            call_ptr(c, callback, 0x003c_f814)?;
            c.g[T2][0] = c.g[V0][0];
            if (c.g[T2][0] as i64) <= 0 {
                frame.restore(c, 16, 0x40);
                allpass_process_epilogue(c, frame);
                return Ok(());
            }
        }
        // The stage is done: blend the rest through the delay line.
        let fx = Addr(c.g[S0][0] as u32);
        c.cycles += 4;
        c.g[T5][0] = c.g[S2][0];
        c.g[T4][0] = c.g[S3][0];
        c.g[T3][0] = c.g[S1][0];
        if (c.g[S1][0] as i64) > 0 {
            c.cycles += 2;
            c.g[T7][0] = fx.off(SFX_SIZE).lw(c);
            c.g[T6][0] = sx(fx.0.wrapping_add(SFX_LEVELS));
            'chunks: loop {
                // Wrap the write position when it has reached the line's end.
                c.cycles += 4;
                c.g[V0][0] = fx.off(SFX_INDEX).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T7][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    fx.off(SFX_INDEX).sw(c, 0);
                }
                c.cycles += 16;
                c.g[T1][0] = fx.off(SFX_INDEX).lw(c);
                c.g[A0][0] = fx.off(SFX_SIZE).lw(c);
                c.g[V0][0] = fx.off(SFX_DELAY).lw(c);
                c.g[A1][0] = sx((c.g[T1][0] as u32) << 2);
                c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[T1][0] as u32));
                c.g[V1][0] = ((c.g[T3][0] as i64) < (c.g[T2][0] as i64)) as u64;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                if c.g[V1][0] != 0 {
                    c.g[T2][0] = c.g[T3][0];
                }
                c.g[A2][0] = c.g[T2][0];
                c.g[A0][0] = c.g[T6][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
                c.g[A3][0] = c.g[T5][0];
                c.g[T0][0] = c.g[T4][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A3][0] as u32));
                let levels = c.g[A0][0] as u32;
                c.lqc2(1, levels);
                // One 16-sample block per pass (four vectors).
                'vectors: loop {
                    c.cycles += 36;
                    let delay = c.g[A1][0] as u32;
                    let src = c.g[A3][0] as u32;
                    let out = c.g[T0][0] as u32;
                    c.lqc2(4, delay);
                    c.lqc2(5, delay.wrapping_add(0x10));
                    c.lqc2(6, delay.wrapping_add(0x20));
                    c.lqc2(7, delay.wrapping_add(0x30));
                    c.lqc2(8, src);
                    c.lqc2(9, src.wrapping_add(0x10));
                    c.lqc2(10, src.wrapping_add(0x20));
                    c.lqc2(11, src.wrapping_add(0x30));
                    c.vu0().mul(0b1111, 12, 4, Src::Bc(1, X));
                    c.vu0().mul(0b1111, 13, 5, Src::Bc(1, X));
                    c.vu0().mul(0b1111, 14, 6, Src::Bc(1, X));
                    c.vu0().mul(0b1111, 15, 7, Src::Bc(1, X));
                    c.vu0().add(0b1111, 12, 12, Src::Vec(8));
                    c.vu0().add(0b1111, 13, 13, Src::Vec(9));
                    c.vu0().add(0b1111, 14, 14, Src::Vec(10));
                    c.vu0().add(0b1111, 15, 15, Src::Vec(11));
                    c.vu0().mul(0b1111, 8, 12, Src::Bc(1, Y));
                    c.vu0().mul(0b1111, 9, 13, Src::Bc(1, Y));
                    c.vu0().mul(0b1111, 10, 14, Src::Bc(1, Y));
                    c.vu0().mul(0b1111, 11, 15, Src::Bc(1, Y));
                    c.vu0().add(0b1111, 8, 8, Src::Vec(4));
                    c.vu0().add(0b1111, 9, 9, Src::Vec(5));
                    c.vu0().add(0b1111, 10, 10, Src::Vec(6));
                    c.vu0().add(0b1111, 11, 11, Src::Vec(7));
                    c.g[A3][0] = sx(src.wrapping_add(0x40));
                    c.g[T0][0] = sx(out.wrapping_add(0x40));
                    c.sqc2(12, delay);
                    c.sqc2(13, delay.wrapping_add(0x10));
                    c.sqc2(14, delay.wrapping_add(0x20));
                    c.sqc2(15, delay.wrapping_add(0x30));
                    c.sqc2(8, out);
                    c.sqc2(9, out.wrapping_add(0x10));
                    c.sqc2(10, out.wrapping_add(0x20));
                    c.sqc2(11, out.wrapping_add(0x30));
                    let more_blocks = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx(delay.wrapping_add(0x40));
                    if more_blocks {
                        if event_due(c) {
                            deliver_event_here(c, 0x003c_f888);
                        }
                        continue 'vectors;
                    }
                    break 'vectors;
                }
                c.cycles += 7;
                c.g[V1][0] = sx((c.g[T2][0] as u32) << 2);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                let index = c.g[T1][0];
                fx.off(SFX_INDEX).sw(c, index);
                c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(c.g[T2][0] as u32));
                let more_chunks = (c.g[T3][0] as i64) > 0;
                c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(c.g[V1][0] as u32));
                if !more_chunks {
                    break 'chunks;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_f838);
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, 16, 0x40);
        allpass_process_epilogue(c, frame);
        Ok(())
    })
}

/// The shared exit of `SND_DspAllpass_Process`: the callee-saved registers
/// it saved, `ra`, and the frame.
fn allpass_process_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 6;
    frame.restore(c, 17, 0x30);
    frame.restore(c, 18, 0x20);
    frame.restore(c, 19, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void SND_DspAllpass_Free(effect *e)` (0x3cf958, `SFILTER_echorestore`):
/// frees the delay line's raw allocation (if any) and clears both pointers.
pub fn snd_dsp_allpass_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(SFX_RAW).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 5;
            call_ptr(c, 0x003b_5e98, 0x003c_f97c)?;
            let fx = Addr(c.g[S0][0] as u32);
            fx.off(SFX_DELAY).sw(c, 0);
            fx.off(SFX_RAW).sw(c, 0);
        }
        frame.restore(c, 16, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SND_DspAllpass_Init(effect *e)` (0x3cf998, `SFILTER_createECHO`):
/// installs the echo's process and restore entries (the code addresses
/// 0x3cf7d0 and 0x3cf958) and clears the state.
pub fn snd_dsp_allpass_init(c: &mut Cpu) {
    c.cycles += 15;
    let fx = Addr(c.g[A0][0] as u32);
    c.g[V0][0] = sx(0x003d_0000u32.wrapping_sub(0x6a8));
    c.g[V1][0] = sx(0x003d_0000u32.wrapping_sub(0x830));
    let restore_fn = c.g[V0][0];
    fx.off(0x04).sw(c, restore_fn);
    let process_fn = c.g[V1][0];
    fx.sw(c, process_fn);
    c.g[V0][0] = 0;
    fx.off(SFX_DELAY).sw(c, 0);
    fx.off(0x08).sw(c, 0);
    fx.off(0x0c).sw(c, 0);
    fx.off(0x10).sw(c, 0);
    fx.off(0x14).sw(c, 0);
    fx.off(0x1b).sb(c, 0);
    fx.off(SFX_RAW).sw(c, 0);
    ret(c);
}

/// `void SND_DspAllpass_SetParams(effect *e, int *params)` (0x3cf9d8,
/// `SFILTER_modifyECHO`): frees the old delay line, then sizes a new one
/// from the parameters: `params[0] >> 1` is the feedback gain (scaled by
/// 1/127), `params[1] >> 8` the rate and `params[2] >> 8` the delay in
/// milliseconds. The delay line length is the prime-delay of that
/// (`SND_CalcPrimeDelay`), rounded up to 16 bytes and zeroed.
pub fn snd_dsp_allpass_set_params(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 19, 0x10);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        let params = Addr(c.g[A1][0] as u32);
        c.g[S2][0] = params.off(0x08).lw(c);
        c.g[S0][0] = params.lw(c);
        c.g[S1][0] = params.off(0x04).lw(c);
        c.g[S2][0] = sx(((c.g[S2][0] as u32) as i32 >> 8) as u32);
        c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 1) as u32);
        c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 8) as u32);
        Callee {
            addr: 0x003c_f958,
            f: ee::s14::f_003cf958,
        }
        .call(c, 0x003c_fa10)?;
        // Feedback level: gain (as a float) times 1/127, and its negative.
        let k = FpuF32(0x3c01_0204);
        c.g[AT][0] = 0x3c01_0204;
        let gain = FpuF32::from_i32(c.g[S0][0] as u32 as i32);
        let fb = gain * k;
        let neg = FpuF32(fb.0 ^ 0x8000_0000);
        abi::set_fpr(c, 0, fb);
        abi::set_fpr(c, 1, neg);
        abi::set_fpr(c, 2, k);
        Addr(c.g[S3][0] as u32).off(0x30).write_fpu(c, fb);
        Addr(c.g[S3][0] as u32).off(0x34).write_fpu(c, neg);
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x003c_fb38,
            f: ee::s14::f_003cfb38,
        }
        .call(c, 0x003c_fa40)?;
        c.g[S0][0] = sx((c.g[V0][0] as u32) << 2);
        let samples = c.g[V0][0];
        Addr(c.g[S3][0] as u32).off(0x28).sw(c, samples);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x003b_5c60, 0x003c_fa50)?;
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
        c.g[A2][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32) >> 4);
        let raw = c.g[V0][0];
        Addr(c.g[S3][0] as u32).off(SFX_RAW).sw(c, raw);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.g[A1][0] = 0;
        let aligned = c.g[V1][0];
        Addr(c.g[S3][0] as u32).off(SFX_DELAY).sw(c, aligned);
        c.g[A0][0] = c.g[V1][0];
        call_ptr(c, 0x0041_6210, 0x003c_fa74)?;
        Addr(c.g[S3][0] as u32).off(SFX_INDEX).sw(c, 0);
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float SND_Sqrt1pSeries(float x)` (0x3cfa98, `SNDI_rootof1plusx`): the
/// series of sqrt(1 + x) to the seventh order, with the coefficients the
/// game uses. Returns it in f0; leaves the partial terms in f1..f7 as the
/// original does.
pub fn snd_sqrt_1p_series(c: &mut Cpu) {
    c.cycles += 39;
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(1, 0);
    c.g[AT][0] = 0xffff_ffff_be80_0000;
    c.mtc1(1, 1);
    c.mul_s(0, 12, 0);
    c.g[AT][0] = 0xffff_ffff_bf00_0000;
    c.mtc1(1, 2);
    c.g[AT][0] = 0xffff_ffff_bf20_0000;
    c.mtc1(1, 4);
    c.g[AT][0] = 0xffff_ffff_bf33_3333;
    c.mtc1(1, 5);
    c.mul_s(1, 0, 1);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 3);
    c.g[AT][0] = 0xffff_ffff_bf40_0000;
    c.mtc1(1, 6);
    c.add_s(0, 0, 3);
    c.g[AT][0] = 0xffff_ffff_bf49_23a3;
    c.mtc1(1, 7);
    c.mul_s(1, 1, 12);
    c.mul_s(2, 1, 2);
    c.add_s(0, 0, 1);
    c.mul_s(2, 2, 12);
    c.mul_s(4, 2, 4);
    c.add_s(0, 0, 2);
    c.mul_s(4, 4, 12);
    c.mul_s(5, 4, 5);
    c.add_s(0, 0, 4);
    c.mul_s(5, 5, 12);
    c.mul_s(6, 5, 6);
    c.add_s(0, 0, 5);
    c.mul_s(6, 6, 12);
    c.mul_s(7, 6, 7);
    c.add_s(0, 0, 6);
    c.mul_s(7, 7, 12);
    c.add_s(0, 0, 7);
    ret(c);
}

/// `int SND_CalcPrimeDelay(int rate, int delayMs)` (0x3cfb38,
/// `SNDI_findmult16`): the delay length in samples, a prime multiple of 16
/// (`SND_NextPrimeDelay(rate, delayMs / 16) * 16`).
pub fn snd_calc_prime_delay(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
        let frame = Frame::open(c, 0x10);
        if c.g[V0][0] != 0 {
            c.g[V1][0] = c.g[A1][0];
        }
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(((c.g[V1][0] as u32) as i32 >> 4) as u32);
        Callee {
            addr: 0x003c_fb98,
            f: ee::s14::f_003cfb98,
        }
        .call(c, 0x003c_fb58)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, fill, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The echo effect state (0x40 bytes) used by the tests.
    const OC1162_EFFECT: u32 = DATA + 0x1000;
    /// The object the chained stage reads its callback from.
    const OC1162_NEXT_OBJ: u32 = DATA + 0x2000;
    /// The parameter block `SetParams` reads.
    const OC1162_PARAMS: u32 = DATA + 0x0800;
    /// The delay line of the process tests.
    const OC1162_DELAY: u32 = DATA + 0x4000;
    /// Source and destination sample buffers.
    const OC1162_SRC: u32 = DATA + 0x8000;
    const OC1162_DST: u32 = DATA + 0xa000;
    /// The old raw allocation `Free` releases.
    const OC1162_RAW_OLD: u32 = DATA + 0x6000;
    /// The block the gated allocator fake returns (beyond the fake log).
    const OC1162_ALLOC_BLOCK: u32 = DATA + 0x20_0000;
    /// The gate word of the allocator fake, and its magic.
    const OC1162_GATE: u32 = DATA + 0xf_f000;
    const OC1162_MAGIC: u32 = 0x1162_0b0b;

    static OC1162_PREV_MEM_ALLOC: AtomicUsize = AtomicUsize::new(0);

    /// `SND_MemAlloc` (0x3b5c60) with the gate on: a logged call returning
    /// the block the setup forced; with the gate off, the body it replaced.
    fn oc1162_fake_mem_alloc(c: &mut Cpu) {
        if c.bus.read32(OC1162_GATE) != OC1162_MAGIC {
            return fakes::fall_back(c, &OC1162_PREV_MEM_ALLOC, ee::s13::f_003b5c60__generated);
        }
        fakes::fake_call(c, 0x003b_5c60)
    }

    fn oc1162_hook() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B5C60),
            oc1162_fake_mem_alloc,
            &OC1162_PREV_MEM_ALLOC,
        );
    }

    fn buffer_source_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x0b5, 0);
        let words = r.next_u32() % 40;
        c.g[A0][0] = sx(OC1162_EFFECT);
        c.g[A1][0] = sx(words);
        c.g[A2][0] = sx(r.next_u32());
        c.g[A3][0] = sx(OC1162_DST);
        c.bus.write32(OC1162_EFFECT + 0x1c, OC1162_SRC);
        fill(c, &mut r, OC1162_SRC, 0x80);
        fill(c, &mut r, OC1162_DST, 0x80);
    }

    fn allpass_process_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x0c1162, 1);
        let n = if seed % 6 == 0 {
            0
        } else {
            16 * (1 + r.next_u32() % 12)
        };
        c.g[A0][0] = sx(OC1162_EFFECT);
        c.g[A1][0] = sx(n);
        c.g[A2][0] = sx(OC1162_SRC);
        c.g[A3][0] = sx(OC1162_DST);
        // Seeds: no chained stage, a stage that returns 0 (the real leaf at
        // 0x1406c0), or a fake stage that returns a count (and may unwind).
        let next = if seed % 5 == 0 { 0 } else { OC1162_NEXT_OBJ };
        c.bus.write32(OC1162_EFFECT + 0x08, next);
        let callback = match seed % 5 {
            1 => 0x0014_06c0,
            _ => fakes::FAKE,
        };
        c.bus.write32(OC1162_NEXT_OBJ, callback);
        c.bus.write32(OC1162_EFFECT + 0x1c, OC1162_DELAY);
        c.bus.write32(OC1162_EFFECT + 0x20, 0);
        c.bus.write32(OC1162_EFFECT + 0x24, 16 * (r.next_u32() % 5));
        c.bus.write32(OC1162_EFFECT + 0x28, 16 * (1 + r.next_u32() % 4));
        for k in 0..4 {
            c.bus.write32(OC1162_EFFECT + 0x30 + 4 * k, r.f32(1.0));
        }
        fill(c, &mut r, OC1162_DELAY, 0x100);
        fill(c, &mut r, OC1162_SRC, 0x100);
        fill(c, &mut r, OC1162_DST, 0x100);
    }

    fn free_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x0f7, 0);
        c.g[A0][0] = sx(OC1162_EFFECT);
        fill(c, &mut r, OC1162_EFFECT, 0x10);
        c.bus.write32(
            OC1162_EFFECT + 0x20,
            if seed % 3 == 0 { 0 } else { OC1162_RAW_OLD },
        );
    }

    fn set_params_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x0a5e, 0);
        // The allocator fake is gated on, and its first call returns the block.
        c.bus.write32(OC1162_GATE, OC1162_MAGIC);
        c.bus.write32(fakes::LOG_RESULTS, OC1162_ALLOC_BLOCK);
        c.g[A0][0] = sx(OC1162_EFFECT);
        c.g[A1][0] = sx(OC1162_PARAMS);
        let gain = (r.next_u32() % 200) as i32 - 100;
        let rate = if seed % 7 == 0 {
            0
        } else {
            1 + r.next_u32() % 60
        };
        let delay = (r.next_u32() % 2000) as i32 - if seed % 3 == 0 { 1000 } else { 0 };
        c.bus.write32(OC1162_PARAMS, (gain << 1) as u32);
        c.bus.write32(OC1162_PARAMS + 4, rate << 8);
        c.bus.write32(OC1162_PARAMS + 8, (delay << 8) as u32);
        fill(c, &mut r, OC1162_EFFECT, 0x10);
        c.bus.write32(
            OC1162_EFFECT + 0x20,
            if seed % 2 == 0 { 0 } else { OC1162_RAW_OLD },
        );
    }

    #[test]
    fn snd_dsp_buffer_source_process_matches() {
        check(
            ee::s14::f_003cf740__generated,
            snd_dsp_buffer_source_process,
            200,
            &buffer_source_setup,
        );
    }

    #[test]
    fn snd_dsp_pass_thru_init_matches() {
        check(
            ee::s14::f_003cf7b0__generated,
            snd_dsp_pass_thru_init,
            200,
            &|c, seed| {
                let mut r = Rng(0x1162_7b0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(OC1162_EFFECT);
                fill(c, &mut r, OC1162_EFFECT, 0x10);
            },
        );
    }

    #[test]
    fn snd_dsp_allpass_process_matches() {
        check(
            ee::s14::f_003cf7d0__generated,
            snd_dsp_allpass_process,
            300,
            &allpass_process_setup,
        );
    }

    #[test]
    fn snd_dsp_allpass_free_matches() {
        check(
            ee::s14::f_003cf958__generated,
            snd_dsp_allpass_free,
            200,
            &free_setup,
        );
    }

    #[test]
    fn snd_dsp_allpass_init_matches() {
        check(
            ee::s14::f_003cf998__generated,
            snd_dsp_allpass_init,
            200,
            &|c, seed| {
                let mut r = Rng(0x1162_998 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(OC1162_EFFECT);
                fill(c, &mut r, OC1162_EFFECT, 0x10);
            },
        );
    }

    #[test]
    fn snd_dsp_allpass_set_params_matches() {
        oc1162_hook();
        check(
            ee::s14::f_003cf9d8__generated,
            snd_dsp_allpass_set_params,
            200,
            &set_params_setup,
        );
    }

    #[test]
    fn snd_sqrt_1p_series_matches() {
        check(
            ee::s14::f_003cfa98__generated,
            snd_sqrt_1p_series,
            300,
            &|c, seed| {
                let mut r = Rng(0x1162_fa98 ^ seed as u64);
                scramble(c, &mut r);
                c.f[12] = if seed % 3 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(1.5)
                };
            },
        );
    }

    #[test]
    fn snd_calc_prime_delay_matches() {
        check(
            ee::s14::f_003cfb38__generated,
            snd_calc_prime_delay,
            200,
            &|c, seed| {
                let mut r = Rng(0x1162_fb38 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(1 + r.next_u32() % 60);
                let delay = (r.next_u32() % 3000) as i32 - if seed % 4 == 0 { 1500 } else { 0 };
                c.g[A1][0] = sx(delay as u32);
            },
        );
    }
}
