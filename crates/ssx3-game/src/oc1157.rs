//! Staging module oc1157 (wave wave273): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1157`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_a000, snd_dec_eaxa_free, "SND_DecEaxa_Free"),
    (0x003c_a028, snd_dec_eaxa_init_mem, "SND_DecEaxa_InitMem"),
    (0x003c_a7b8, snd_dsp_lowpass_process, "SND_DspLowpass_Process"),
    (0x003c_a870, snd_dsp_lowpass_init, "SND_DspLowpass_Init"),
    (0x003c_a8a0, snd_dsp_lowpass_set_params, "SND_DspLowpass_SetParams"),
    (0x003c_a900, snd_dsp_fir_highpass_process, "SND_DspFirHighpass_Process"),
    (0x003c_a990, snd_dsp_fir_highpass_init, "SND_DspFirHighpass_Init"),
    (0x003c_a9d8, snd_dsp_fir_highpass_set_cutoff, "SND_DspFirHighpass_SetCutoff"),
];

/// `void SND_DecEaxa_Free(SfilterNode* node)` (0x3ca000): the restore hook of
/// the EA-XA unpacker. Frees the decoder at `node+0x1c` through
/// `SND_EaxaFree` when there is one.
pub fn snd_dec_eaxa_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x1c).lw(c);
        let has_decoder = c.g[A0][0] != 0;
        frame.restore_ra(c, 0);
        if has_decoder {
            c.cycles += 3;
            Callee {
                addr: 0x003c_cf18,
                f: ee::s14::SND_EaxaFree,
            }
            .call(c, 0x003c_a01c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void SND_DecEaxa_InitMem(SfilterNode* node, SfilterSource* source)`
/// (0x3ca028): set the node's filter, restore and frame-fetch hooks, allocate
/// a 0xa8-byte EA-XA decoder (`SND_EaxaAlloc`), reset it and feed it the
/// source's samples (`SND_EaxaSetInput`, `totalframes * 4` bytes). Returns
/// what `SND_EaxaSetInput` returned.
pub fn snd_dec_eaxa_init_mem(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x003d_0000;
        frame.save(c, S0, 0x20);
        c.g[V1][0] = 0x003d_0000;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        // SFILTER_unpackxaf and SFILTER_unpackxafrestore (SND_DecEaxa_Free).
        c.g[V0][0] = sx(0x003d_0000u32.wrapping_sub(0x60a8));
        c.g[V1][0] = sx(0x003d_0000u32.wrapping_sub(0x6000));
        let filter = c.g[V0][0];
        let restore = c.g[V1][0];
        Addr(c.g[S0][0] as u32).sw(c, filter);
        Addr(c.g[S0][0] as u32).off(0x4).sw(c, restore);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = 0xa8;
        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
        let psample = c.g[V0][0];
        Addr(c.g[S0][0] as u32).off(0x20).sw(c, psample);
        c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
        let frames = c.g[V1][0];
        Addr(c.g[S0][0] as u32).off(0x28).sw(c, 0);
        Addr(c.g[S0][0] as u32).off(0x24).sw(c, frames);
        Callee {
            addr: 0x003c_cef0,
            f: ee::s14::SND_EaxaAlloc,
        }
        .call(c, 0x003c_a074)?;
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x003c_cf40,
            f: ee::s14::SND_EaxaReset,
        }
        .call(c, 0x003c_a07c)?;
        // SFILTER_unpackgetframexaf (0x3c9ff8) goes to the source.
        let decoder = c.g[V0][0];
        Addr(c.g[S0][0] as u32).off(0x1c).sw(c, decoder);
        c.g[V1][0] = sx(0x003d_0000u32.wrapping_sub(0x6008));
        let getframe = c.g[V1][0];
        Addr(c.g[S1][0] as u32).off(0x28).sw(c, getframe);
        c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x24).lw(c);
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x20).lw(c);
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x1c).lw(c);
        c.g[A3][0] = c.g[A2][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
        Callee {
            addr: 0x003c_cf60,
            f: ee::s14::SND_EaxaSetInput,
        }
        .call(c, 0x003c_a0a4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SND_DspLowpass_Process(LowpassNode* self, int n, ...)` (0x3ca7b8):
/// the one-pole low-pass filter stage. The upstream filter (`self+8`, its
/// function at word 0) runs first, with the same `n` and the pair of sample
/// pointers swapped as the generated code passes them; an upstream result
/// <= 0 is returned at once. Otherwise each sample is filtered with the state
/// `st0 = st0 * st1 + st2 * in[i]` (state at `self+0x1c`) and written to
/// `out[i]`. The loop checks events at its back edge.
pub fn snd_dsp_lowpass_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A3][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x8).lw(c);
        c.g[S3][0] = c.g[A2][0];
        let mut upstream_done = false;
        if c.g[A0][0] != 0 {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
            c.g[T0][0] = Addr(c.g[S0][0] as u32).off(0x1a).lbu(c);
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S3][0];
            let target = c.g[V0][0] as u32;
            call_ptr(c, target, 0x003c_a7fc)?;
            upstream_done = (c.g[V0][0] as i64) <= 0;
        }
        if upstream_done {
            frame.restore(c, S0, 0x40);
        } else {
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
            if (c.g[S1][0] as i64) > 0 {
                c.cycles += 3;
                c.g[A3][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[V0][0] = c.g[S1][0];
                let st = c.g[A0][0] as u32;
                loop {
                    c.cycles += 13;
                    c.f[3] = Addr(c.g[A2][0] as u32).read_u32(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    c.f[1] = Addr(st).read_u32(c);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
                    c.f[2] = Addr(st + 4).read_u32(c);
                    c.f[0] = Addr(st + 8).read_u32(c);
                    c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[2])).0;
                    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[3])).0;
                    c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[0])).0;
                    let acc = FpuF32(c.f[1]);
                    Addr(st).write_fpu(c, acc);
                    Addr(c.g[A3][0] as u32).write_fpu(c, acc);
                    let more = c.g[V0][0] != 0;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(4));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x003c_a818);
                        }
                        continue;
                    }
                    break;
                }
            }
            c.cycles += 2;
            c.g[V0][0] = c.g[S1][0];
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SND_DspLowpass_Init(LowpassNode* self)` (0x3ca870): the filter
/// function is `SND_DspLowpass_Process`, no upstream, state cleared. Returns 0.
pub fn snd_dsp_lowpass_init(c: &mut Cpu) {
    c.cycles += 12;
    let node = Addr(c.g[A0][0] as u32);
    node.off(0x1c).sw(c, 0);
    c.g[V0][0] = sx(0x003d_0000u32.wrapping_sub(0x5848));
    node.off(0x8).sw(c, 0);
    let process = c.g[V0][0];
    node.sw(c, process);
    c.g[V0][0] = 0;
    node.off(0xc).sw(c, 0);
    node.off(0x10).sw(c, 0);
    node.off(0x14).sw(c, 0);
    node.off(0x1b).sb(c, 0);
    node.off(0x4).sw(c, 0);
    ret(c);
}

/// `void SND_DspLowpass_SetParams(LowpassNode* self, const int* params)`
/// (0x3ca8a0): the one-pole coefficients from `params`: the cutoff
/// `2 * p0 / p1` (stored at `+0x24` as the feedback, `1 - cutoff` at `+0x20`
/// as the leakage) scaled by the gain `p2 / 256`.
pub fn snd_dsp_lowpass_set_params(c: &mut Cpu) {
    c.cycles += 24;
    let node = Addr(c.g[A0][0] as u32);
    let params = Addr(c.g[A1][0] as u32);
    c.f[1] = FpuF32::from_i32(params.read_u32(c) as i32).0;
    c.g[AT][0] = 0x3f80_0000;
    c.f[2] = c.g[AT][0] as u32;
    c.f[0] = FpuF32::from_i32(params.off(0x4).read_u32(c) as i32).0;
    c.g[AT][0] = 0x3b80_0000;
    c.f[3] = c.g[AT][0] as u32;
    c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[1])).0;
    c.f[1] = FpuF32(c.f[1]).div(FpuF32(c.f[0])).0;
    c.f[2] = (FpuF32(c.f[2]) - FpuF32(c.f[1])).0;
    let cutoff = FpuF32(c.f[1]);
    node.off(0x24).write_fpu(c, cutoff);
    let leakage = FpuF32(c.f[2]);
    node.off(0x20).write_fpu(c, leakage);
    c.f[0] = FpuF32::from_i32(params.off(0x8).read_u32(c) as i32).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[3])).0;
    c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[0])).0;
    let inverse = FpuF32(c.f[1]);
    node.off(0x24).write_fpu(c, inverse);
    c.f[0] = FpuF32::from_i32(params.off(0x8).read_u32(c) as i32).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[3])).0;
    c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[0])).0;
    let leakage_scaled = FpuF32(c.f[2]);
    node.off(0x20).write_fpu(c, leakage_scaled);
    ret(c);
}

/// `int SND_DspFirHighpass_Process(FirNode* self, int n, ...)` (0x3ca900):
/// the FIR high-pass stage. The upstream filter (`self+8`) runs first and its
/// result is the sample count (`n` when there is no upstream); a count <= 0
/// is returned at once. Otherwise `SND_FirProcess` filters the samples in
/// `self+0x20` (the FIR state) and the count is returned.
pub fn snd_dsp_fir_highpass_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x8).lw(c);
        c.g[S3][0] = c.g[A3][0];
        let early_return = if c.g[A0][0] != 0 {
            c.cycles += 8;
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
            c.g[A2][0] = c.g[S3][0];
            c.g[T0][0] = Addr(c.g[S1][0] as u32).off(0x1a).lbu(c);
            c.g[A3][0] = c.g[S2][0];
            let target = c.g[V0][0] as u32;
            call_ptr(c, target, 0x003c_a940)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x20));
            (c.g[S0][0] as i64) <= 0
        } else {
            c.cycles += 2;
            c.g[S0][0] = c.g[A1][0];
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x20));
            false
        };
        if early_return {
            c.cycles += 2;
            frame.restore(c, S0, 0x40);
        } else {
            c.cycles += 6;
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S0][0];
            Callee {
                addr: 0x003c_ebe8,
                f: ee::s14::f_003cebe8,
            }
            .call(c, 0x003c_a96c)?;
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
    })
}

/// `void SND_DspFirHighpass_Init(FirNode* self)` (0x3ca990): the filter
/// function is `SND_DspFirHighpass_Process`, no upstream, then the FIR state
/// is reset (`SND_FirReset` on `self+0x20`). Returns 0.
pub fn snd_dsp_fir_highpass_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.g[A0][0];
        c.g[V1][0] = sx(0x003d_0000u32.wrapping_sub(0x5700));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
        let node = Addr(c.g[V0][0] as u32);
        let process = c.g[V1][0];
        node.sw(c, process);
        node.off(0x8).sw(c, 0);
        node.off(0xc).sw(c, 0);
        node.off(0x10).sw(c, 0);
        node.off(0x14).sw(c, 0);
        node.off(0x1b).sb(c, 0);
        node.off(0x4).sw(c, 0);
        Callee {
            addr: 0x003c_eb90,
            f: ee::s14::f_003ceb90,
        }
        .call(c, 0x003c_a9c8)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    })
}

/// `void SND_DspFirHighpass_SetCutoff(FirNode* self, const int* params)`
/// (0x3ca9d8): `cutLow = (p0 >> 7) / (p1 >> 8)` (float division, stored at
/// `self+0x114`), then `SND_FirDesign(&self->fir, 3)` designs the filter.
pub fn snd_dsp_fir_highpass_set_cutoff(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let node = c.g[A2][0] as u32;
        c.g[A0][0] = sx(node.wrapping_add(0x20));
        c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x4).lw(c);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 8) as u32);
        c.g[A1][0] = 3;
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
        c.f[1] = FpuF32::from_i32(c.g[V1][0] as u32 as i32).0;
        c.f[0] = FpuF32::from_i32(c.g[V0][0] as u32 as i32).0;
        let cut = FpuF32(c.f[0]).div(FpuF32(c.f[1]));
        c.f[0] = cut.0;
        Addr(node).off(0x114).write_fpu(c, cut);
        Callee {
            addr: 0x003c_e7c0,
            f: ee::s14::f_003ce7c0,
        }
        .call(c, 0x003c_aa18)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, machine_rt, FAKE, LOG_RESULTS};
    use crate::difftest::{check, elf_image, fill, scramble, Rng, DATA};

    /// `SND_DecEaxa_Free`: a node with no decoder (no call) or one the real
    /// `SND_EaxaFree` hands to the fake free hook.
    #[test]
    fn snd_dec_eaxa_free_matches() {
        covered(
            ee::s14::f_003ca000__generated,
            snd_dec_eaxa_free,
            300,
            &|c, seed| {
                machine(c, seed, 0x3c_a000, 1);
                let node = DATA + 0x2000;
                let decoder = if seed % 3 == 0 {
                    0
                } else {
                    DATA + 0x4000 + 0x100 * (seed % 16)
                };
                c.bus.write32(node + 0x1c, decoder);
                c.bus.write32(crate::game::lib::snd::SND_EAXA_FREE_HOOK, FAKE);
                c.g[A0][0] = sx(node);
            },
        );
    }

    /// `SND_DecEaxa_InitMem`: the decoder allocation goes to the fake hook;
    /// reset and set-input are real. Totals of 0..31 frames.
    #[test]
    fn snd_dec_eaxa_init_mem_matches() {
        covered(
            ee::s14::f_003ca028__generated,
            snd_dec_eaxa_init_mem,
            300,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x3c_a028, 3);
                let node = DATA + 0x2000;
                let source = DATA + 0x2400;
                c.bus.write32(crate::game::lib::snd::SND_EAXA_ALLOC_HOOK, FAKE);
                c.bus.write32(source, r.next_u32());
                c.bus.write32(source + 0xc, r.next_u32() % 32);
                c.g[A0][0] = sx(node);
                c.g[A1][0] = sx(source);
            },
        );
    }

    /// `SND_DspLowpass_Process`: no upstream, an upstream fake (positive
    /// result), or the real `SND_DspLowpass_Init` as upstream (returns 0, so
    /// the stage returns at once). Counts 0..8 cover the loop.
    #[test]
    fn snd_dsp_lowpass_process_matches() {
        covered(
            ee::s14::f_003ca7b8__generated,
            snd_dsp_lowpass_process,
            300,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x3c_a7b8, 1);
                let node = DATA + 0x2000;
                let upstream = DATA + 0x2800;
                let input = DATA + 0x3000;
                let output = DATA + 0x3400;
                let up = match seed % 5 {
                    0 => 0,
                    4 => 0x003c_a870,
                    _ => FAKE,
                };
                if up != 0 {
                    c.bus.write32(upstream, up);
                }
                c.bus.write32(node + 0x8, if up == 0 { 0 } else { upstream });
                c.bus.write8(node + 0x1a, r.next_u32() as u8);
                for off in [0x1c, 0x20, 0x24] {
                    c.bus.write32(node + off, r.f32(1.0));
                }
                for i in 0..8 {
                    c.bus.write32(input + 4 * i, r.f32(4.0));
                }
                c.g[A0][0] = sx(node);
                c.g[A1][0] = (seed % 9) as u64;
                c.g[A2][0] = sx(input);
                c.g[A3][0] = sx(output);
            },
        );
    }

    /// `SND_DspLowpass_Init`: clears the node and sets its filter function.
    #[test]
    fn snd_dsp_lowpass_init_matches() {
        check(
            ee::s14::f_003ca870__generated,
            snd_dsp_lowpass_init,
            300,
            &|c, seed| {
                let mut r = Rng(0x3c_a870 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let node = DATA + 0x2000;
                fill(c, &mut r, node, 16);
                c.g[A0][0] = sx(node);
            },
        );
    }

    /// `SND_DspLowpass_SetParams`: small parameters, a zero divisor on some
    /// seeds (infinities compare the same on both sides).
    #[test]
    fn snd_dsp_lowpass_set_params_matches() {
        check(
            ee::s14::f_003ca8a0__generated,
            snd_dsp_lowpass_set_params,
            300,
            &|c, seed| {
                let mut r = Rng(0x3c_a8a0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let node = DATA + 0x2000;
                let params = DATA + 0x3000;
                for i in 0..3 {
                    let v = if seed % 7 == 0 && i == 1 {
                        0
                    } else {
                        (r.next_u32() % 4096) as i32 - 2048
                    };
                    c.bus.write32(params + 4 * i, v as u32);
                }
                c.g[A0][0] = sx(node);
                c.g[A1][0] = sx(params);
            },
        );
    }

    /// `SND_DspFirHighpass_Process`: no upstream, a fake upstream whose forced
    /// result is the sample count, or the real Init (count 0: early return).
    #[test]
    fn snd_dsp_fir_highpass_process_matches() {
        covered(
            ee::s14::f_003ca900__generated,
            snd_dsp_fir_highpass_process,
            300,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x3c_a900, 1);
                let node = DATA + 0x2000;
                let upstream = DATA + 0x2800;
                let input = DATA + 0x3000;
                let output = DATA + 0x3400;
                let up = match seed % 5 {
                    0 => 0,
                    4 => 0x003c_a870,
                    _ => FAKE,
                };
                if up != 0 {
                    c.bus.write32(upstream, up);
                }
                c.bus.write32(node + 0x8, if up == 0 { 0 } else { upstream });
                // SND_FirProcess steps 16 samples (0x40 bytes) per pass and ends
                // only when its pointer hits the end: counts must be 16 * k.
                let count = 16 * (1 + seed % 3);
                c.bus.write32(LOG_RESULTS, count);
                for i in 0..48 {
                    c.bus.write32(input + 4 * i, r.f32(4.0));
                }
                c.g[A0][0] = sx(node);
                c.g[A1][0] = count as u64;
                c.g[A2][0] = sx(input);
                c.g[A3][0] = sx(output);
            },
        );
    }

    /// `SND_DspFirHighpass_Init`: sets the node and resets the FIR state.
    #[test]
    fn snd_dsp_fir_highpass_init_matches() {
        check(
            ee::s14::f_003ca990__generated,
            snd_dsp_fir_highpass_init,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_a990, 0);
                let node = DATA + 0x2000;
                fill(c, &mut r, node, 16);
                c.g[A0][0] = sx(node);
            },
        );
    }

    /// `SND_DspFirHighpass_SetCutoff`: signed and zero parameters, the
    /// game image loaded for the design's constants.
    #[test]
    fn snd_dsp_fir_highpass_set_cutoff_matches() {
        check(
            ee::s14::f_003ca9d8__generated,
            snd_dsp_fir_highpass_set_cutoff,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x3c_a9d8, 0);
                elf_image(c);
                let node = DATA + 0x2000;
                let params = DATA + 0x3000;
                c.bus.write32(
                    params,
                    ((r.next_u32() % 0x2_0000) as i32 - 0x1_0000) as u32,
                );
                c.bus.write32(
                    params + 4,
                    if seed % 6 == 0 {
                        0
                    } else {
                        r.next_u32() % 0x1_0000
                    },
                );
                c.g[A0][0] = sx(node);
                c.g[A1][0] = sx(params);
            },
        );
    }
}
