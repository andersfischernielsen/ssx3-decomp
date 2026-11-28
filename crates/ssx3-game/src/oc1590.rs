//! Staging module oc1590 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1590`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0040_39f0, libmpeg_setup_frame_buffers, "libmpeg_SetupFrameBuffers"),
    (0x0040_3ad0, libmpeg_load_iq_matrix, "libmpeg_LoadIqMatrix"),
    (
        0x0040_3d70,
        libmpeg_sequence_scalable_extension,
        "libmpeg_SequenceScalableExtension",
    ),
    (0x0040_3d80, libmpeg_unknown_extension, "libmpeg_UnknownExtension"),
    (
        0x0040_3d90,
        libmpeg_picture_spatial_scalable_extension,
        "libmpeg_PictureSpatialScalableExtension",
    ),
    (
        0x0040_3da0,
        libmpeg_picture_temporal_scalable_extension,
        "libmpeg_PictureTemporalScalableExtension",
    ),
    (0x0040_3db0, libmpeg_stop_dma_callback, "libmpeg_StopDmaCallback"),
    (0x0040_61e8, libmpeg_slice_header, "libmpeg_SliceHeader"),
];

/// `libmpeg_DispatchCallback` (0x402c30).
pub(crate) const MPEG_DISPATCH_CALLBACK: Callee = Callee {
    addr: 0x0040_2c30,
    f: ee::s15::libmpeg_DispatchCallback,
};
/// `libmpeg_IpuWaitBusy` (0x406e20).
pub(crate) const MPEG_IPU_WAIT_BUSY: Callee = Callee {
    addr: 0x0040_6e20,
    f: ee::s15::libmpeg_IpuWaitBusy,
};
/// `libmpeg_IpuCommand` (0x406df0).
pub(crate) const MPEG_IPU_COMMAND: Callee = Callee {
    addr: 0x0040_6df0,
    f: ee::s15::libmpeg_IpuCommand,
};
/// `DIntr` (0x42c078): disable interrupts, returns the previous state.
pub(crate) const MPEG_DINTR: Callee = Callee {
    addr: 0x0042_c078,
    f: ee::s15::DIntr,
};
/// `EIntr` (0x42c0c0): enable interrupts.
pub(crate) const MPEG_EINTR: Callee = Callee {
    addr: 0x0042_c0c0,
    f: ee::s15::EIntr,
};
/// `sceIpuStopDMA` (0x409400).
pub(crate) const MPEG_SCE_IPU_STOP_DMA: Callee = Callee {
    addr: 0x0040_9400,
    f: ee::s15::sceIpuStopDMA,
};
/// `libmpeg_NextStartCode` (0x407438).
pub(crate) const MPEG_NEXT_START_CODE: Callee = Callee {
    addr: 0x0040_7438,
    f: ee::s15::libmpeg_NextStartCode,
};
/// `libmpeg_ShowBits` (0x4070f0).
pub(crate) const MPEG_SHOW_BITS: Callee = Callee {
    addr: 0x0040_70f0,
    f: ee::s15::libmpeg_ShowBits,
};
/// `libmpeg_FlushBits` (0x4071f8).
pub(crate) const MPEG_FLUSH_BITS: Callee = Callee {
    addr: 0x0040_71f8,
    f: ee::s15::libmpeg_FlushBits,
};
/// `libmpeg_SliceHeaderTail` (0x4074b8).
pub(crate) const MPEG_SLICE_HEADER_TAIL: Callee = Callee {
    addr: 0x0040_74b8,
    f: ee::s15::libmpeg_SliceHeaderTail,
};
/// `libmpeg_MacroblockAddressIncrement` (0x405fb8).
pub(crate) const MPEG_MB_ADDR_INC: Callee = Callee {
    addr: 0x0040_5fb8,
    f: ee::s15::libmpeg_MacroblockAddressIncrement,
};
/// `libmpeg_Errorf` (0x403470).
pub(crate) const MPEG_ERRORF: Callee = Callee {
    addr: 0x0040_3470,
    f: ee::s15::libmpeg_Errorf,
};
/// `libmpeg_Error` (0x4034a8).
pub(crate) const MPEG_ERROR: Callee = Callee {
    addr: 0x0040_34a8,
    f: ee::s15::libmpeg_Error,
};

/// Stack frame epilogue of `libmpeg_SliceHeader`'s exit block: reload the
/// saved registers with `ld` (8 bytes each), then return through `ra`.
fn mpeg_slice_header_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 9;
    let a = frame.at(c, 0x60);
    c.g[RA][0] = c.ld(a);
    let a = frame.at(c, 0x50);
    c.g[S5][0] = c.ld(a);
    let a = frame.at(c, 0x40);
    c.g[S4][0] = c.ld(a);
    let a = frame.at(c, 0x30);
    c.g[S3][0] = c.ld(a);
    let a = frame.at(c, 0x20);
    c.g[S2][0] = c.ld(a);
    let a = frame.at(c, 0x10);
    c.g[S1][0] = c.ld(a);
    let a = frame.at(c, 0x00);
    c.g[S0][0] = c.ld(a);
    frame.close(c);
}

/// `void libmpeg_SetupFrameBuffers(...)` (0x4039f0, a leaf): lays out the
/// three planes of a frame buffer. It stores the uncached (`| 0x2000_0000`)
/// addresses of the y, u and v planes for the two field halves into the
/// argument pointers (a0..a3, t0..t3), with the chroma stride derived from
/// the width word at +0x70 and the height word at +0x78 (both on the stack).
pub fn libmpeg_setup_frame_buffers(c: &mut Cpu) {
    c.cycles += 56;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
    c.g[T6][0] = 0x0fff_0000;
    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x78));
    c.g[T6][0] |= 0xffff;
    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x70));
    c.sd((c.g[SP][0] as u32), c.g[S0][0]);
    c.mult(0, 3, 3, 2);
    c.g[S0][0] = 0xffff_ffff_ffff_ffff;
    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
    c.g[S2][0] = 0x180;
    c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x58));
    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
    c.g[S0][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1ff));
    if c.g[S0][0] != 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    c.g[S1][0] = c.g[T4][0] & c.g[T6][0];
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 9) as u32);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S3][0]);
    c.mult(0, 3, 2, 18);
    c.g[T5][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[S3][0] = 0x2000_0000;
    c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S4][0]);
    c.g[S4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x68));
    c.g[S1][0] |= c.g[S3][0];
    c.g[T7][0] = c.g[T5][0] & c.g[T6][0];
    c.sw((c.g[A0][0] as u32), c.g[S1][0]);
    c.g[T4][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T4][0] as u32)));
    c.g[T7][0] |= c.g[S3][0];
    c.mult(0, 3, 2, 18);
    c.mult(0, 4, 2, 18);
    c.sw((c.g[A1][0] as u32), c.g[T7][0]);
    c.g[T4][0] &= c.g[T6][0];
    c.g[T4][0] |= c.g[S3][0];
    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[S0][0] = c.ld(c.g[SP][0] as u32);
    c.g[T5][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T5][0] as u32)));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
    c.g[V1][0] = c.g[S4][0] & c.g[T6][0];
    c.g[T5][0] &= c.g[T6][0];
    c.g[V1][0] |= c.g[S3][0];
    c.g[V0][0] &= c.g[T6][0];
    c.sw((c.g[A2][0] as u32), c.g[V1][0]);
    c.g[T5][0] |= c.g[S3][0];
    c.sw((c.g[A3][0] as u32), c.g[S1][0]);
    c.g[V0][0] |= c.g[S3][0];
    c.sw((c.g[T0][0] as u32), c.g[T7][0]);
    c.sw((c.g[T1][0] as u32), c.g[V1][0]);
    c.sw((c.g[T2][0] as u32), c.g[T4][0]);
    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x50));
    c.sw((c.g[T3][0] as u32), c.g[T5][0]);
    c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
    c.sw((c.g[V1][0] as u32), c.g[V0][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    ret(c);
}

/// `void libmpeg_LoadIqMatrix(sys, matrix, srcAddr)` (0x403ad0): wait for
/// the IPU, stop the DMA with interrupts off, program the IPU source address
/// (`srcAddr & 0xfffffff`, 4 quadwords) and the IPU command word (0x101,
/// the load-IQ-matrix command), then wait again and dispatch the callback.
/// `sys` is the MPEG object, its callback argument at +0x858.
pub fn libmpeg_load_iq_matrix(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x2;
        let a = frame.at(c, 0x40);
        c.sd(a, c.g[S2][0]);
        let a = frame.at(c, 0x30);
        c.sd(a, c.g[S1][0]);
        c.g[S2][0] = c.g[A1][0];
        let a = frame.at(c, 0x20);
        c.sd(a, c.g[S0][0]);
        c.g[S1][0] = c.g[A0][0];
        let a = frame.at(c, 0x50);
        c.sd(a, c.g[RA][0]);
        c.g[S0][0] = c.g[A2][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x858));
        let a = frame.at(c, 0x00);
        c.sw(a, c.g[V0][0]);
        MPEG_DISPATCH_CALLBACK.call(c, 0x0040_3b04)?;
        c.g[A0][0] = c.g[S1][0];
        MPEG_IPU_WAIT_BUSY.call(c, 0x0040_3b0c)?;
        c.g[V0][0] = 0x1000_0000;
        c.g[A0][0] = c.g[S1][0];
        c.g[V0][0] |= 0x2000;
        c.sw(c.g[V0][0] as u32, 0);
        MPEG_IPU_WAIT_BUSY.call(c, 0x0040_3b20)?;
        MPEG_DINTR.call(c, 0x0040_3b28)?;
        c.g[V1][0] = 0x0fff_0000;
        c.g[A0][0] = 0x1000_0000;
        c.g[V1][0] |= 0xffff;
        c.g[A0][0] |= 0xb410;
        c.g[S0][0] &= c.g[V1][0];
        c.g[A1][0] = 0x1000_0000;
        c.sw(c.g[A0][0] as u32, c.g[S0][0]);
        c.g[A1][0] |= 0xb420;
        c.g[A0][0] = 0x4;
        c.g[V1][0] = 0x1000_0000;
        c.sw(c.g[A1][0] as u32, c.g[A0][0]);
        c.g[V1][0] |= 0xb400;
        c.g[A0][0] = 0x101;
        c.sw(c.g[V1][0] as u32, c.g[A0][0]);
        // The interrupt state DIntr returned: re-enable only if it was set.
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            MPEG_EINTR.call(c, 0x0040_3b70)?;
        }
        c.g[A1][0] = c.g[S2][0];
        c.cycles += 15;
        c.g[A0][0] = c.g[S1][0];
        MPEG_IPU_COMMAND.call(c, 0x0040_3b7c)?;
        c.g[A0][0] = c.g[S1][0];
        MPEG_IPU_WAIT_BUSY.call(c, 0x0040_3b84)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x858));
        c.g[V0][0] = 0x3;
        let a = frame.at(c, 0x00);
        c.sw(a, c.g[V0][0]);
        c.g[A1][0] = c.g[SP][0];
        MPEG_DISPATCH_CALLBACK.call(c, 0x0040_3b98)?;
        let a = frame.at(c, 0x50);
        c.g[RA][0] = c.ld(a);
        let a = frame.at(c, 0x40);
        c.g[S2][0] = c.ld(a);
        let a = frame.at(c, 0x30);
        c.g[S1][0] = c.ld(a);
        let a = frame.at(c, 0x20);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    })
}

/// Unsupported MPEG extension stub (0x403d70): `libmpeg_Error` with the
/// message at `0x497d58`. A tail call: `ra` is left for `libmpeg_Error`.
pub fn libmpeg_sequence_scalable_extension(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A1][0] = sx(0x0049_0000u32.wrapping_add(0x7d58));
    c.pc = 0x0040_34a8;
    ee::s15::libmpeg_Error(c);
}

/// Unsupported MPEG extension stub (0x403d80): `libmpeg_Error` with the
/// message at `0x497d88`. A tail call.
pub fn libmpeg_unknown_extension(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A1][0] = sx(0x0049_0000u32.wrapping_add(0x7d88));
    c.pc = 0x0040_34a8;
    ee::s15::libmpeg_Error(c);
}

/// Unsupported picture spatial scalable extension stub (0x403d90):
/// `libmpeg_Error` with the message at `0x497da0`. A tail call.
pub fn libmpeg_picture_spatial_scalable_extension(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A1][0] = sx(0x0049_0000u32.wrapping_add(0x7da0));
    c.pc = 0x0040_34a8;
    ee::s15::libmpeg_Error(c);
}

/// Unsupported picture temporal scalable extension stub (0x403da0):
/// `libmpeg_Error` with the message at `0x497dd8`. A tail call.
pub fn libmpeg_picture_temporal_scalable_extension(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A1][0] = sx(0x0049_0000u32.wrapping_add(0x7dd8));
    c.pc = 0x0040_34a8;
    ee::s15::libmpeg_Error(c);
}

/// `int libmpeg_StopDmaCallback(sys)` (0x403db0): stop the IPU DMA channel
/// at the `+0x4c` field of the sub-object that `sys+0x40` points at; returns 1.
pub fn libmpeg_stop_dma_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let a = (c.g[A0][0] as u32).wrapping_add(0x40);
        c.g[A0][0] = c.lw(a);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4c));
        MPEG_SCE_IPU_STOP_DMA.call(c, 0x0040_3dc4)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0x1;
        frame.close(c);
        Ok(())
    })
}

/// `int libmpeg_SliceHeader(self, ...)` (0x4061e8): read the slice start code
/// (`ShowBits(32)` after `NextStartCode`), reject start codes outside 0x101..0x1af
/// (returns 2 after `libmpeg_Errorf`), then read the slice's macroblock fields
/// (`FlushBits(32)`, `SliceHeaderTail`, `MacroblockAddressIncrement`), store the
/// macroblock address and increment, and clear the predictors. A flag at +0x11c
/// set by the address increment makes it report an error (returns 1).
pub fn libmpeg_slice_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x70);
        let a = frame.at(c, 0x00);
        c.sd(a, c.g[S0][0]);
        let a = frame.at(c, 0x50);
        c.sd(a, c.g[S5][0]);
        c.g[S0][0] = c.g[A0][0];
        let a = frame.at(c, 0x40);
        c.sd(a, c.g[S4][0]);
        c.g[S5][0] = c.g[A2][0];
        let a = frame.at(c, 0x20);
        c.sd(a, c.g[S2][0]);
        c.g[S4][0] = c.g[A3][0];
        let a = frame.at(c, 0x10);
        c.sd(a, c.g[S1][0]);
        let a = frame.at(c, 0x60);
        c.sd(a, c.g[RA][0]);
        c.g[S1][0] = c.g[T0][0];
        let a = frame.at(c, 0x30);
        c.sd(a, c.g[S3][0]);
        let a = (c.g[S0][0] as u32).wrapping_add(0x11c);
        c.sw(a, 0);
        MPEG_NEXT_START_CODE.call(c, 0x0040_6220)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x20;
        MPEG_SHOW_BITS.call(c, 0x0040_622c)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x101));
        c.g[V0][0] = (c.g[V0][0] < 0xafu64) as u64;
        let start_code_ok = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S0][0];
        if !start_code_ok {
            c.cycles += 6;
            c.g[A1][0] = sx(0x0049_0000u32.wrapping_add(0x7f28));
            c.g[A2][0] = c.g[S2][0];
            MPEG_ERRORF.call(c, 0x0040_6250)?;
            c.g[V0][0] = 0x2;
            mpeg_slice_header_exit(c, frame);
            return Ok(());
        }
        // The rest of the slice header.
        c.cycles += 12;
        c.g[A1][0] = 0x20;
        MPEG_FLUSH_BITS.call(c, 0x0040_6260)?;
        c.g[A0][0] = c.g[S0][0];
        MPEG_SLICE_HEADER_TAIL.call(c, 0x0040_6268)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        MPEG_MB_ADDR_INC.call(c, 0x0040_6274)?;
        c.g[A2][0] = c.g[V0][0];
        let a = c.g[S4][0] as u32;
        c.sw(a, c.g[A2][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x11c));
        let no_error = c.g[V0][0] == 0;
        c.g[A1][0] = 0x0049_0000;
        if !no_error {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(0x0049_0000u32.wrapping_add(0x7f50));
            MPEG_ERROR.call(c, 0x0040_6294)?;
            c.g[V0][0] = 0x1;
            mpeg_slice_header_exit(c, frame);
            return Ok(());
        }
        // Success: store the macroblock address and increment, clear the predictors.
        c.cycles += 21;
        c.g[V0][0] = c.g[S2][0] & 0xff;
        c.g[V1][0] = sx((c.g[S3][0] as u32) << 7);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x12c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        c.g[A1][0] = 0x1;
        c.mult(0, 2, 3, 4);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        c.g[V0][0] = 0;
        let a = c.g[S5][0] as u32;
        c.sw(a, c.g[V1][0]);
        let a = c.g[S4][0] as u32;
        c.sw(a, c.g[A1][0]);
        let a = (c.g[S0][0] as u32).wrapping_add(0x1b0);
        c.sw(a, c.g[A1][0]);
        let s1 = c.g[S1][0] as u32;
        c.sw(s1.wrapping_add(0x8), 0);
        c.sw(s1.wrapping_add(0x14), 0);
        c.sw(s1.wrapping_add(0x10), 0);
        c.sw(s1.wrapping_add(0x4), 0);
        c.sw(s1, 0);
        c.sw(s1.wrapping_add(0x1c), 0);
        c.sw(s1.wrapping_add(0x18), 0);
        c.sw(s1.wrapping_add(0xc), 0);
        mpeg_slice_header_exit(c, frame);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::ee;
    use super::*;
    use crate::difftest::fakes::{fall_back, hook, prev_cell};
    use crate::difftest::{check, fresh, run_generated, scramble, Rng, DATA, SP as SP_IN};
    use ssx3_ee_rt::cpu::reg::*;
    use ssx3_ee_rt::cpu::sx;
    use ssx3_ee_rt::Cpu;
    use std::ptr::addr_of_mut;

    /// Gate word: the fakes answer only when a test wrote `MAGIC` here.
    const MPEG_TEST_GATE: u32 = DATA + 0x20_0000;
    const MPEG_TEST_MAGIC: u32 = 0x1590_0c1a;
    /// Forced result of fake `n` (the value its `v0` gets) at `FORCED + 4n`.
    const MPEG_TEST_FORCED: u32 = DATA + 0x20_0100;
    /// Call count of fake `n` at `COUNTS + 4n`.
    const MPEG_TEST_COUNTS: u32 = DATA + 0x20_0200;
    /// Set: `libmpeg_MacroblockAddressIncrement` sets the slice's error flag.
    const MPEG_TEST_MB_FLAG: u32 = DATA + 0x20_0300;
    /// Number of fakes (index order: see `mpeg_test_install`).
    const MPEG_TEST_FAKES: usize = 13;

    /// Records a call of fake `idx` and forces its `v0`.
    fn mpeg_test_hit(c: &mut Cpu, idx: u32) {
        let n = c.bus.read32(MPEG_TEST_COUNTS + 4 * idx);
        c.bus.write32(MPEG_TEST_COUNTS + 4 * idx, n + 1);
        c.g[V0][0] = sx(c.bus.read32(MPEG_TEST_FORCED + 4 * idx));
    }

    /// A gated fake for the callee at `$generated`: index `$idx`.
    macro_rules! mpeg_test_fake {
        ($name:ident, $idx:expr, $generated:path) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(MPEG_TEST_GATE) != MPEG_TEST_MAGIC {
                    return fall_back(c, prev_cell($name), $generated);
                }
                mpeg_test_hit(c, $idx);
                ret(c);
            }
        };
    }

    mpeg_test_fake!(mpeg_test_fake_dispatch, 0, ee::s15::f_00402c30__generated);
    mpeg_test_fake!(mpeg_test_fake_ipu_wait, 1, ee::s15::f_00406e20__generated);
    mpeg_test_fake!(mpeg_test_fake_dintr, 2, ee::s15::f_0042c078__generated);
    mpeg_test_fake!(mpeg_test_fake_eintr, 3, ee::s15::f_0042c0c0__generated);
    mpeg_test_fake!(mpeg_test_fake_ipu_command, 4, ee::s15::f_00406df0__generated);
    mpeg_test_fake!(mpeg_test_fake_stop_dma, 5, ee::s15::f_00409400__generated);
    mpeg_test_fake!(mpeg_test_fake_next_start_code, 6, ee::s15::f_00407438__generated);
    mpeg_test_fake!(mpeg_test_fake_show_bits, 7, ee::s15::f_004070f0__generated);
    mpeg_test_fake!(mpeg_test_fake_flush_bits, 8, ee::s15::f_004071f8__generated);
    mpeg_test_fake!(mpeg_test_fake_slice_header_tail, 9, ee::s15::f_004074b8__generated);
    mpeg_test_fake!(mpeg_test_fake_errorf, 11, ee::s15::f_00403470__generated);
    mpeg_test_fake!(mpeg_test_fake_error, 12, ee::s15::f_004034a8__generated);

    /// The macroblock address increment fake: as the others, and with the
    /// error flag set it marks the slice (`self+0x11c = 1`), as the real
    /// callee does for a slice it rejects.
    fn mpeg_test_fake_mb_addr_inc(c: &mut Cpu) {
        if c.bus.read32(MPEG_TEST_GATE) != MPEG_TEST_MAGIC {
            return fall_back(
                c,
                prev_cell(mpeg_test_fake_mb_addr_inc),
                ee::s15::f_00405fb8__generated,
            );
        }
        mpeg_test_hit(c, 10);
        if c.bus.read32(MPEG_TEST_MB_FLAG) != 0 {
            let this = c.g[A0][0] as u32;
            c.bus.write32(this.wrapping_add(0x11c), 1);
        }
        ret(c);
    }

    /// Hooks every fake into its callee's override slot (each once per process).
    fn mpeg_test_install() {
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00402C30),
            mpeg_test_fake_dispatch,
            prev_cell(mpeg_test_fake_dispatch),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00406E20),
            mpeg_test_fake_ipu_wait,
            prev_cell(mpeg_test_fake_ipu_wait),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042C078),
            mpeg_test_fake_dintr,
            prev_cell(mpeg_test_fake_dintr),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042C0C0),
            mpeg_test_fake_eintr,
            prev_cell(mpeg_test_fake_eintr),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00406DF0),
            mpeg_test_fake_ipu_command,
            prev_cell(mpeg_test_fake_ipu_command),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00409400),
            mpeg_test_fake_stop_dma,
            prev_cell(mpeg_test_fake_stop_dma),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00407438),
            mpeg_test_fake_next_start_code,
            prev_cell(mpeg_test_fake_next_start_code),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_004070F0),
            mpeg_test_fake_show_bits,
            prev_cell(mpeg_test_fake_show_bits),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_004071F8),
            mpeg_test_fake_flush_bits,
            prev_cell(mpeg_test_fake_flush_bits),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_004074B8),
            mpeg_test_fake_slice_header_tail,
            prev_cell(mpeg_test_fake_slice_header_tail),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00405FB8),
            mpeg_test_fake_mb_addr_inc,
            prev_cell(mpeg_test_fake_mb_addr_inc),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00403470),
            mpeg_test_fake_errorf,
            prev_cell(mpeg_test_fake_errorf),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_004034A8),
            mpeg_test_fake_error,
            prev_cell(mpeg_test_fake_error),
        );
    }

    /// Turns the fakes on in this machine.
    fn mpeg_test_gate(c: &mut Cpu) {
        c.bus.write32(MPEG_TEST_GATE, MPEG_TEST_MAGIC);
    }

    /// Runs the generated `f` for `seed` and returns the fake call counts.
    fn mpeg_test_counts(f: GuestFn, setup: &dyn Fn(&mut Cpu, u32), seed: u32) -> Vec<u32> {
        let mut c = fresh();
        setup(&mut c, seed);
        run_generated(&mut c, f);
        (0..MPEG_TEST_FAKES as u32)
            .map(|i| c.bus.read32(MPEG_TEST_COUNTS + 4 * i))
            .collect()
    }

    /// Every fake in `idxs` is called by some seed's generated run.
    fn mpeg_test_reach(f: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32), idxs: &[usize]) {
        let mut hit = vec![false; MPEG_TEST_FAKES];
        for seed in 0..seeds {
            for (i, n) in mpeg_test_counts(f, setup, seed).iter().enumerate() {
                hit[i] |= *n > 0;
            }
        }
        for &i in idxs {
            assert!(hit[i], "fake {i} never called");
        }
    }

    /// The stub fixture: gated fakes, scrambled registers.
    fn mpeg_test_stub_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x403d_70 ^ (seed as u64) << 20);
        scramble(c, &mut r);
        mpeg_test_gate(c);
    }

    #[test]
    fn libmpeg_setup_frame_buffers_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = Rng(0x4039_f0 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            c.g[A1][0] = (DATA + 0x2000) as u64;
            c.g[A2][0] = (DATA + 0x3000) as u64;
            c.g[A3][0] = (DATA + 0x4000) as u64;
            c.g[T0][0] = (DATA + 0x5000) as u64;
            c.g[T1][0] = (DATA + 0x6000) as u64;
            c.g[T2][0] = (DATA + 0x7000) as u64;
            c.g[T3][0] = (DATA + 0x8000) as u64;
            // The stack arguments: store pointer (+0x50), two counts and
            // two sizes whose product takes both signs.
            c.bus.write32(SP_IN, DATA + 0x9000);
            c.bus.write32(SP_IN + 0x08, r.next_u32());
            c.bus.write32(SP_IN + 0x10, r.next_u32());
            c.bus.write32(SP_IN + 0x18, r.next_u32());
            let w = r.next_u32() % 1000;
            let h = r.next_u32() % 600;
            let w = if seed % 3 == 0 { w.wrapping_neg() } else { w };
            c.bus.write32(SP_IN + 0x20, w);
            c.bus.write32(SP_IN + 0x28, h);
        };
        check(ee::s15::f_004039f0__generated, libmpeg_setup_frame_buffers, 200, setup);
    }

    #[test]
    fn libmpeg_load_iq_matrix_matches() {
        mpeg_test_install();
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = Rng(0x403a_d0 ^ (seed as u64) << 24);
            scramble(c, &mut r);
            mpeg_test_gate(c);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            c.g[A1][0] = (DATA + 0x2000) as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.bus.write32(DATA + 0x1000 + 0x858, DATA + 0x3000);
            // DIntr's result: odd seeds re-enable the interrupts.
            let intr = if seed % 2 == 0 { 0 } else { 1 + r.next_u32() % 0xffff };
            c.bus.write32(MPEG_TEST_FORCED + 4 * 2, intr);
        };
        check(ee::s15::f_00403ad0__generated, libmpeg_load_iq_matrix, 200, setup);
        mpeg_test_reach(ee::s15::f_00403ad0__generated, 200, setup, &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn libmpeg_extension_stubs_match() {
        mpeg_test_install();
        check(
            ee::s15::f_00403d70__generated,
            libmpeg_sequence_scalable_extension,
            20,
            &mpeg_test_stub_setup,
        );
        check(
            ee::s15::f_00403d80__generated,
            libmpeg_unknown_extension,
            20,
            &mpeg_test_stub_setup,
        );
        check(
            ee::s15::f_00403d90__generated,
            libmpeg_picture_spatial_scalable_extension,
            20,
            &mpeg_test_stub_setup,
        );
        check(
            ee::s15::f_00403da0__generated,
            libmpeg_picture_temporal_scalable_extension,
            20,
            &mpeg_test_stub_setup,
        );
    }

    #[test]
    fn libmpeg_stop_dma_callback_matches() {
        mpeg_test_install();
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = Rng(0x403d_b0 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            mpeg_test_gate(c);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            c.bus.write32(DATA + 0x1000 + 0x40, DATA + 0x2000);
        };
        check(ee::s15::f_00403db0__generated, libmpeg_stop_dma_callback, 40, setup);
        mpeg_test_reach(ee::s15::f_00403db0__generated, 4, setup, &[5]);
    }

    #[test]
    fn libmpeg_slice_header_matches() {
        mpeg_test_install();
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = Rng(0x4061_e8 ^ (seed as u64) << 24);
            scramble(c, &mut r);
            mpeg_test_gate(c);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = (DATA + 0x2000) as u64;
            c.g[A3][0] = (DATA + 0x2010) as u64;
            c.g[T0][0] = (DATA + 0x2100) as u64;
            c.bus.write32(DATA + 0x1000 + 0x12c, 1 + r.next_u32() % 40);
            c.bus.write32(DATA + 0x1000 + 0x1b0, r.next_u32());
            // Seeds 0 mod 4: start code out of range; 1 mod 4: in range and
            // the address increment flags the slice; others: in range.
            let start_code = match seed % 4 {
                0 => 0x1b0 + seed % 0x40,
                _ => 0x101 + (seed * 7) % 0xaf,
            };
            c.bus.write32(MPEG_TEST_FORCED + 4 * 7, start_code);
            c.bus.write32(MPEG_TEST_FORCED + 4 * 9, r.next_u32());
            c.bus.write32(MPEG_TEST_FORCED + 4 * 10, r.next_u32());
            c.bus.write32(MPEG_TEST_MB_FLAG, (seed % 4 == 1) as u32);
        };
        check(ee::s15::f_004061e8__generated, libmpeg_slice_header, 200, setup);
        mpeg_test_reach(
            ee::s15::f_004061e8__generated,
            200,
            setup,
            &[6, 7, 8, 9, 10, 11, 12],
        );
    }
}
