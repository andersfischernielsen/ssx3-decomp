//! Staging module oc1161 (wave wave273): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1161`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
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
    (0x003c_f298, snd_dsp_sum_free, "SND_DspSum_Free"),
    (0x003c_f2c0, snd_dsp_sum_init, "SND_DspSum_Init"),
    (0x003c_f408, snd_dsp_resonator_process, "SND_DspResonator_Process"),
    (0x003c_f498, snd_dsp_resonator_init, "SND_DspResonator_Init"),
    (0x003c_f4d0, snd_dsp_resonator_set_params, "SND_DspResonator_SetParams"),
    (0x003c_f5d0, snd_dsp_block_delay_process, "SND_DspBlockDelay_Process"),
    (0x003c_f6d0, snd_dsp_block_delay_free, "SND_DspBlockDelay_Free"),
    (0x003c_f6f8, snd_dsp_block_delay_init, "SND_DspBlockDelay_Init"),
];

/// Frees the temp buffer of a DSP node (`node+0x1c`, if non-zero) with
/// `SND_MemFree`. Shared by `SND_DspSum_Free` and `SND_DspBlockDelay_Free`
/// (both 0x10-byte frames); only the return address of the free call differs,
/// so it is a parameter.
fn snd_dsp_free_temp(c: &mut Cpu, free_ret: u32) -> Flow {
    c.cycles += 5;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    let node = c.g[A0][0] as u32;
    c.g[A0][0] = c.lw(node.wrapping_add(0x1c));
    let temp = c.g[A0][0];
    frame.restore_ra(c, 0);
    if temp != 0 {
        c.cycles += 3;
        call_ptr(c, 0x003b_5e98, free_ret)?;
        frame.restore_ra(c, 0);
    }
    c.cycles += 2;
    frame.close(c);
    Ok(())
}

/// `void SND_DspSum_Free(SFILTER* node)` (0x3cf298, 40 bytes): frees the
/// temp buffer of a mixer node (`node+0x1c`) if it has one.
pub fn snd_dsp_sum_free(c: &mut Cpu) {
    guest(c, |c| snd_dsp_free_temp(c, 0x003c_f2b4));
}

/// `void SND_DspSum_Init(SFILTER* node)` (0x3cf2c0, 60 bytes): sets the
/// mixer's restore and process entries (`node+4`, `node+0`) and clears its
/// chaining links, flags and temp buffer.
pub fn snd_dsp_sum_init(c: &mut Cpu) {
    c.cycles += 15;
    c.g[V0][0] = 0x3d_0000;
    c.g[V1][0] = 0x3d_0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xd68)); // SND_DspSum_Free
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xe88)); // SND_DspSum_Process
    let node = c.g[A0][0] as u32;
    c.sw(node.wrapping_add(0x4), c.g[V0][0]);
    c.sw(node, c.g[V1][0]);
    c.g[V0][0] = 0;
    c.sw(node.wrapping_add(0x20), 0);
    c.sw(node.wrapping_add(0x8), 0);
    c.sw(node.wrapping_add(0xc), 0);
    c.sw(node.wrapping_add(0x10), 0);
    c.sw(node.wrapping_add(0x14), 0);
    c.sb(node.wrapping_add(0x1b), 0);
    c.sw(node.wrapping_add(0x1c), 0);
    ret(c);
}

/// `void SND_DspResonator_Process(RESON* node, int count, ?, ?)` (0x3cf408,
/// 140 bytes): runs the filter chained after `node` (`node+8`: a vtable whose
/// first word is the process function, called with the requester byte at
/// `node+0x1a`) and, if that leaves a positive count `n`, runs the resonator
/// over `n` samples (`SND_DspResonator_Run`). Returns the count (v0).
pub fn snd_dsp_resonator_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 19, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        let node = c.g[S1][0] as u32;
        c.g[A0][0] = c.lw(node.wrapping_add(0x8));
        let obj = c.g[A0][0] as u32;
        c.g[S3][0] = c.g[A3][0];
        let mut early = false;
        if obj != 0 {
            c.cycles += 8;
            c.g[V0][0] = c.lw(obj);
            c.g[A2][0] = c.g[S3][0];
            c.g[T0][0] = c.lbu(node.wrapping_add(0x1a));
            let target = c.g[V0][0] as u32;
            c.g[A3][0] = c.g[S2][0];
            call_ptr(c, target, 0x003c_f448)?;
            c.g[S0][0] = c.g[V0][0];
            let positive = (c.g[S0][0] as i64) > 0;
            c.g[A0][0] = c.g[S1][0];
            if !positive {
                c.cycles += 2;
                frame.restore(c, S0, 0x40);
                early = true;
            }
        } else {
            c.cycles += 2;
            c.g[S0][0] = c.g[A1][0];
            c.g[A0][0] = c.g[S1][0];
        }
        if !early {
            c.cycles += 6;
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S0][0];
            Callee {
                addr: 0x003c_f300,
                f: ee::s14::SND_DspResonator_Run,
            }
            .call(c, 0x003c_f474)?;
            c.g[V0][0] = c.g[S0][0];
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SND_DspResonator_Init(RESON* node)` (0x3cf498, 56 bytes): sets the
/// process entry (`node+0`, `SND_DspResonator_Process`) and clears the
/// chaining links, the state words and the flags.
pub fn snd_dsp_resonator_init(c: &mut Cpu) {
    c.cycles += 14;
    c.g[V0][0] = 0x3d_0000;
    let node = c.g[A0][0] as u32;
    c.sw(node.wrapping_add(0x3c), 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xbf8)); // SND_DspResonator_Process
    c.sw(node.wrapping_add(0x8), 0);
    c.sw(node, c.g[V0][0]);
    c.g[V0][0] = 0;
    c.sw(node.wrapping_add(0xc), 0);
    c.sw(node.wrapping_add(0x10), 0);
    c.sw(node.wrapping_add(0x14), 0);
    c.sb(node.wrapping_add(0x1b), 0);
    c.sw(node.wrapping_add(0x4), 0);
    c.sw(node.wrapping_add(0x34), 0);
    c.sw(node.wrapping_add(0x38), 0);
    ret(c);
}

/// `void SND_DspResonator_SetParams(RESON* node, const int* p)` (0x3cf4d0,
/// 252 bytes): derives the resonator's coefficients from the parameter block:
/// `p[0]`, `p[1]` and `p[3]` are fixed-point (>> 8 for the first two, plain
/// for the last, as floats, the first two as an int to `node+0x1c`); the
/// damping and frequency terms go through `SND_Cosf` and `SND_Sqrt1pSeries`.
/// The coefficients land at `node+0x1c..+0x30`.
pub fn snd_dsp_resonator_set_params(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 63;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let params = c.g[A1][0] as u32;
        c.g[V0][0] = c.lw(params);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 8) as u32);
        frame.save_fpr(c, 21, 0x28);
        c.f[2] = FpuF32::from_i32(c.g[V0][0] as u32 as i32).0;
        frame.save_fpr(c, 20, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 22, 0x30);
        c.g[AT][0] = 0x4049_0fdb;
        c.f[0] = c.g[AT][0] as u32;
        c.lwc1(22, params.wrapping_add(0xc));
        c.f[22] = FpuF32::from_i32(c.f[22] as i32).0;
        c.g[V1][0] = c.lw(params.wrapping_add(0x8));
        c.g[A2][0] = c.lw(params.wrapping_add(0x4));
        c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 8) as u32);
        c.g[AT][0] = 0x40c9_0fdb;
        c.f[12] = c.g[AT][0] as u32;
        c.f[4] = c.g[V1][0] as u32;
        c.f[4] = FpuF32::from_i32(c.f[4] as i32).0;
        c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 8) as u32);
        c.f[3] = c.g[A2][0] as u32;
        c.f[3] = FpuF32::from_i32(c.f[3] as i32).0;
        c.g[AT][0] = 0x3f80_0000;
        c.f[20] = c.g[AT][0] as u32;
        c.f[12] = (FpuF32(c.f[2]) * FpuF32(c.f[12])).0;
        c.g[AT][0] = 0x3b80_0000;
        c.f[1] = c.g[AT][0] as u32;
        c.f[0] = (FpuF32(c.f[4]) * FpuF32(c.f[0])).0;
        let node = c.g[S0][0] as u32;
        c.sw(node.wrapping_add(0x1c), c.g[A2][0]);
        c.swc1(2, node.wrapping_add(0x20));
        c.f[22] = (FpuF32(c.f[22]) * FpuF32(c.f[1])).0;
        c.f[12] = FpuF32(c.f[12]).div(FpuF32(c.f[3])).0;
        c.swc1(4, node.wrapping_add(0x24));
        c.f[0] = FpuF32(c.f[0]).div(FpuF32(c.f[3])).0;
        c.f[0] = (FpuF32(c.f[20]) - FpuF32(c.f[0])).0;
        c.f[21] = (FpuF32(c.f[0]) * FpuF32(c.f[0])).0;
        c.swc1(0, node.wrapping_add(0x28));
        call_ptr(c, 0x003b_c840, 0x003c_f57c)?;
        let node = c.g[S0][0] as u32;
        c.lwc1(12, node.wrapping_add(0x28));
        c.f[1] = (FpuF32(c.f[21]) + FpuF32(c.f[20])).0;
        c.f[12] = (FpuF32(c.f[12]) + FpuF32(c.f[12])).0;
        c.f[12] = FpuF32(c.f[12]).div(FpuF32(c.f[1])).0;
        c.f[12] = (FpuF32(c.f[12]) * FpuF32(c.f[0])).0;
        c.f[0] = (-FpuF32(c.f[12])).0;
        c.swc1(12, node.wrapping_add(0x2c));
        c.f[12] = (FpuF32(c.f[0]) * FpuF32(c.f[12])).0;
        Callee {
            addr: 0x003c_fa98,
            f: ee::s14::SND_Sqrt1pSeries,
        }
        .call(c, 0x003c_f5a0)?;
        c.f[20] = (FpuF32(c.f[20]) - FpuF32(c.f[21])).0;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x28);
        c.f[20] = (FpuF32(c.f[20]) * FpuF32(c.f[22])).0;
        frame.restore_fpr(c, 22, 0x30);
        c.f[20] = (FpuF32(c.f[20]) * FpuF32(c.f[0])).0;
        let node = c.g[S0][0] as u32;
        c.swc1(20, node.wrapping_add(0x30));
        frame.restore(c, S0, 0x10);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// `void SND_DspBlockDelay_Process(BDELAY* node, int count, ?, ...)` (0x3cf5d0,
/// 256 bytes): a delay line. A pending flag at `node+0x24` is cleared and
/// `node+0x26` set. The delay buffer (`node+0x1c`, `frames` at `node+0x20`) is
/// reallocated for `count` words when too short. Then, if `node+0x26` is set,
/// the chained filter (`node+8`, vtable word) is run and, if it reports a
/// positive count, the block is copied into the delay buffer and back to the
/// output; otherwise the input is copied straight to the output and the
/// pending flag is set. Returns the count (v0), or the chained filter's result
/// when it reports none.
pub fn snd_dsp_block_delay_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, 18, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 19, 0x20);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, 20, 0x10);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, 17, 0x40);
        frame.save_ra(c, 0);
        let node = c.g[S0][0] as u32;
        c.g[V0][0] = c.lh(node.wrapping_add(0x24));
        let t = c.g[V0][0] == 0u64;
        c.g[S4][0] = c.g[A2][0];
        if !t {
            c.cycles += 3;
            c.g[V0][0] = 1;
            c.sh(node.wrapping_add(0x24), 0);
            c.sh(node.wrapping_add(0x26), c.g[V0][0]);
        }
        // Delay buffer: grow when `frames < count` (signed).
        c.cycles += 4;
        c.g[V0][0] = c.lw(node.wrapping_add(0x20));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S3][0] as i64)) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[S1][0] = sx((c.g[S3][0] as u32) << 2);
        if !t {
            c.cycles += 3;
            c.g[A0][0] = c.lw(node.wrapping_add(0x1c));
            if c.g[A0][0] != 0 {
                c.cycles += 2;
                call_ptr(c, 0x003b_5e98, 0x003c_f634)?;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003b_5c60, 0x003c_f63c)?;
            c.sw(node.wrapping_add(0x1c), c.g[V0][0]);
            c.sw(node.wrapping_add(0x20), c.g[S3][0]);
        }
        c.cycles += 3;
        c.g[V0][0] = c.lh(node.wrapping_add(0x26));
        let t = c.g[V0][0] == 0u64;
        c.g[A1][0] = c.g[S3][0];
        let mut early = false;
        if t {
            // No chained filter output pending: copy the input straight out.
            c.cycles += 6;
            c.g[A1][0] = c.lw(node.wrapping_add(0x1c));
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x003c_51e8, 0x003c_f6a4)?;
            c.g[V0][0] = 1;
            c.sh(node.wrapping_add(0x24), c.g[V0][0]);
        } else {
            c.cycles += 8;
            c.g[A0][0] = c.lw(node.wrapping_add(0x8));
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S4][0];
            let obj = c.g[A0][0] as u32;
            c.g[V0][0] = c.lw(obj);
            let target = c.g[V0][0] as u32;
            c.g[T0][0] = 1;
            call_ptr(c, target, 0x003c_f668)?;
            let not_positive = (c.g[V0][0] as i64) <= 0;
            c.g[A0][0] = c.g[S2][0];
            if not_positive {
                early = true;
            } else {
                c.cycles += 9;
                c.g[A1][0] = c.g[S4][0];
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, 0x003c_51e8, 0x003c_f67c)?;
                c.g[A0][0] = c.lw(node.wrapping_add(0x1c));
                c.g[A1][0] = c.g[S4][0];
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, 0x003c_51e8, 0x003c_f68c)?;
                c.sh(node.wrapping_add(0x26), 0);
            }
        }
        if !early {
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SND_DspBlockDelay_Free(BDELAY* node)` (0x3cf6d0, 40 bytes): frees the
/// delay buffer (`node+0x1c`) if it has one.
pub fn snd_dsp_block_delay_free(c: &mut Cpu) {
    guest(c, |c| snd_dsp_free_temp(c, 0x003c_f6ec));
}

/// `void SND_DspBlockDelay_Init(BDELAY* node)` (0x3cf6f8, 72 bytes): sets the
/// process and restore entries (`node+0`, `node+4`), the pending flag
/// (`node+0x26` = 1) and clears the rest of the state.
pub fn snd_dsp_block_delay_init(c: &mut Cpu) {
    c.cycles += 18;
    c.g[V0][0] = 1;
    c.g[V1][0] = 0x3d_0000;
    c.g[A1][0] = 0x3d_0000;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x930)); // SND_DspBlockDelay_Free
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xa30)); // SND_DspBlockDelay_Process
    let node = c.g[A0][0] as u32;
    c.sh(node.wrapping_add(0x26), c.g[V0][0]);
    c.g[V0][0] = 0;
    c.sw(node.wrapping_add(0x4), c.g[V1][0]);
    c.sw(node, c.g[A1][0]);
    c.sw(node.wrapping_add(0x8), 0);
    c.sw(node.wrapping_add(0xc), 0);
    c.sw(node.wrapping_add(0x10), 0);
    c.sw(node.wrapping_add(0x14), 0);
    c.sb(node.wrapping_add(0x1b), 0);
    c.sw(node.wrapping_add(0x20), 0);
    c.sw(node.wrapping_add(0x1c), 0);
    c.sh(node.wrapping_add(0x24), 0);
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, fill, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the test fakes in the machine's own RAM: the fakes answer
    /// only when the fixture wrote [`OC1161_MAGIC`] there (otherwise they fall
    /// back to the body they replaced).
    const OC1161_GATE: u32 = DATA + 0x20_1161;
    const OC1161_MAGIC: u32 = 0x1161_c0de;

    static OC1161_PREV_FREE: AtomicUsize = AtomicUsize::new(0);
    static OC1161_PREV_ALLOC: AtomicUsize = AtomicUsize::new(0);
    static OC1161_PREV_COPY: AtomicUsize = AtomicUsize::new(0);

    /// Gated fake for `SND_MemFree`: logs the call and returns.
    fn oc1161_fake_free(c: &mut Cpu) {
        if c.bus.read32(OC1161_GATE) != OC1161_MAGIC {
            return fakes::fall_back(c, &OC1161_PREV_FREE, ee::s13::f_003b5e98__generated);
        }
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, 0x003b_5e98, ret);
    }

    /// Gated fake for `SND_MemAlloc`: logs the call and returns a pointer.
    fn oc1161_fake_alloc(c: &mut Cpu) {
        if c.bus.read32(OC1161_GATE) != OC1161_MAGIC {
            return fakes::fall_back(c, &OC1161_PREV_ALLOC, ee::s13::f_003b5c60__generated);
        }
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, 0x003b_5c60, ret);
    }

    /// Gated fake for `SND_MemCopy`: logs the call and returns.
    fn oc1161_fake_copy(c: &mut Cpu) {
        if c.bus.read32(OC1161_GATE) != OC1161_MAGIC {
            return fakes::fall_back(c, &OC1161_PREV_COPY, ee::s13::f_003c51e8__generated);
        }
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, 0x003c_51e8, ret);
    }

    /// Hook the three allocator and copy callees into their override slots
    /// (once per process) and write the gate into this test's machine. The
    /// fakes' forced results are small positive numbers: a chained filter
    /// reporting `n > 0` samples, and pointers that are never dereferenced.
    fn oc1161_install(c: &mut Cpu) {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B5E98),
            oc1161_fake_free,
            &OC1161_PREV_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B5C60),
            oc1161_fake_alloc,
            &OC1161_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C51E8),
            oc1161_fake_copy,
            &OC1161_PREV_COPY,
        );
        c.bus.write32(OC1161_GATE, OC1161_MAGIC);
        for k in 0..8 {
            c.bus.write32(fakes::LOG_RESULTS + 4 * k, 2);
        }
    }

    /// `SND_DspSum_Free`: a node with and without a temp buffer.
    #[test]
    fn snd_dsp_sum_free_matches() {
        check(
            ee::s14::f_003cf298__generated,
            snd_dsp_sum_free,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003c_f298, 2);
                oc1161_install(c);
                let node = DATA + 0x100;
                c.g[A0][0] = sx(node);
                let temp = if seed % 3 == 0 {
                    0
                } else {
                    DATA + 0x1000 + 16 * (r.next_u32() % 8)
                };
                c.bus.write32(node + 0x1c, temp);
            },
        );
    }

    /// `SND_DspBlockDelay_Free`: the same shape as the sum free.
    #[test]
    fn snd_dsp_block_delay_free_matches() {
        check(
            ee::s14::f_003cf6d0__generated,
            snd_dsp_block_delay_free,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003c_f6d0, 2);
                oc1161_install(c);
                let node = DATA + 0x100;
                c.g[A0][0] = sx(node);
                let temp = if seed % 3 == 0 {
                    0
                } else {
                    DATA + 0x1000 + 16 * (r.next_u32() % 8)
                };
                c.bus.write32(node + 0x1c, temp);
            },
        );
    }

    /// `SND_DspSum_Init` writes the same fields over random memory.
    #[test]
    fn snd_dsp_sum_init_matches() {
        check(
            ee::s14::f_003cf2c0__generated,
            snd_dsp_sum_init,
            100,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003c_f2c0, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                fill(c, &mut r, DATA + 0x100, 16);
            },
        );
    }

    /// `SND_DspResonator_Init` over random memory.
    #[test]
    fn snd_dsp_resonator_init_matches() {
        check(
            ee::s14::f_003cf498__generated,
            snd_dsp_resonator_init,
            100,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003c_f498, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                fill(c, &mut r, DATA + 0x100, 16);
            },
        );
    }

    /// `SND_DspBlockDelay_Init` over random memory.
    #[test]
    fn snd_dsp_block_delay_init_matches() {
        check(
            ee::s14::f_003cf6f8__generated,
            snd_dsp_block_delay_init,
            100,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003c_f6f8, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                fill(c, &mut r, DATA + 0x100, 16);
            },
        );
    }

    /// `SND_DspResonator_Process`: the chained filter is a real leaf that
    /// returns 0 (one seed in three), or the fake reporting 2 samples; the
    /// node may have no chained filter at all. Both the early exit and the
    /// resonator run are reached.
    #[test]
    fn snd_dsp_resonator_process_matches() {
        check(
            ee::s14::f_003cf408__generated,
            snd_dsp_resonator_process,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003c_f408, 2);
                oc1161_install(c);
                let node = DATA + 0x100;
                let obj = DATA + 0x200;
                fill(c, &mut r, node, 16);
                fill(c, &mut r, DATA + 0x400, 32);
                fill(c, &mut r, DATA + 0x800, 32);
                let target = if seed % 3 == 0 { 0x0014_06c0 } else { fakes::FAKE };
                c.bus.write32(obj, target);
                c.bus.write32(node + 0x8, if seed % 4 == 0 { 0 } else { obj });
                c.bus.write8(node + 0x1a, (r.next_u32() & 0xff) as u8);
                c.g[A0][0] = sx(node);
                c.g[A1][0] = sx(1 + seed % 4);
                c.g[A2][0] = sx(DATA + 0x400);
                c.g[A3][0] = sx(DATA + 0x800);
            },
        );
    }

    /// `SND_DspResonator_SetParams`: small fixed-point parameters, so the
    /// cosine argument stays in range.
    #[test]
    fn snd_dsp_resonator_set_params_matches() {
        check(
            ee::s14::f_003cf4d0__generated,
            snd_dsp_resonator_set_params,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003c_f4d0, 0);
                let node = DATA + 0x100;
                let params = DATA + 0x300;
                fill(c, &mut r, node, 16);
                c.bus
                    .write32(params, ((r.next_u32() % 0x1_0000) << 8) | (seed & 0xff));
                c.bus
                    .write32(params + 4, (((r.next_u32() % 0x7f) + 1) << 8) as u32);
                c.bus
                    .write32(params + 8, ((r.next_u32() % 0x1_0000) << 8) as u32);
                c.bus.write32(params + 0xc, r.next_u32() % 0x1000);
                c.g[A0][0] = sx(node);
                c.g[A1][0] = sx(params);
            },
        );
    }

    /// `SND_DspBlockDelay_Process`: every branch: the pending flags, the
    /// buffer grow with and without an old buffer, the chained filter
    /// reporting 0 or 2 samples, and the copy-through path.
    #[test]
    fn snd_dsp_block_delay_process_matches() {
        check(
            ee::s14::f_003cf5d0__generated,
            snd_dsp_block_delay_process,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003c_f5d0, 2);
                oc1161_install(c);
                let node = DATA + 0x100;
                let obj = DATA + 0x200;
                fill(c, &mut r, node, 16);
                fill(c, &mut r, DATA + 0x400, 32);
                fill(c, &mut r, DATA + 0x800, 32);
                let target = if seed % 3 == 0 { 0x0014_06c0 } else { fakes::FAKE };
                c.bus.write32(obj, target);
                c.bus.write32(node + 0x8, obj);
                // Pending flags at +0x24 (lh) and +0x26 (lh).
                let flag_a = if seed & 1 == 0 { 0 } else { r.next_u32() & 0xffff };
                let flag_b = if seed & 2 == 0 { 0 } else { 1 + (r.next_u32() & 0x7fff) };
                c.bus.write32(node + 0x24, flag_a | (flag_b << 16));
                let count = 1 + seed % 4;
                let frames = if seed % 5 == 0 { count + 3 } else { r.next_u32() % 3 };
                c.bus.write32(node + 0x20, frames);
                let temp = if seed % 7 == 0 { 0 } else { DATA + 0x1000 };
                c.bus.write32(node + 0x1c, temp);
                c.g[A0][0] = sx(node);
                c.g[A1][0] = sx(count);
                c.g[A2][0] = sx(DATA + 0x400);
                c.g[A3][0] = sx(DATA + 0x800);
            },
        );
    }
}
