//! Staging module oc1123 (wave wave270): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1123`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0040_81d8, oc1123_check_output_buffer, "libmpeg_CheckOutputBuffer"),
    (0x0040_8530, oc1123_compute_pts, "libmpeg_ComputePts"),
    (0x0040_8af8, oc1123_ipu_from_dma_start, "libmpeg_IpuFromDmaStart"),
    (0x0040_8b80, oc1123_ipu_csc, "libmpeg_IpuCsc"),
    (0x0040_8df0, oc1123_ipu_csc_dma, "libmpeg_IpuCscDma"),
];

/// `sprintf` (0x417828).
const OC1123_SPRINTF: Callee = Callee {
    addr: 0x0041_7828,
    f: ee::s15::f_00417828,
};
/// `libmpeg_Error` (0x4034a8).
const OC1123_ERROR: Callee = Callee {
    addr: 0x0040_34a8,
    f: ee::s15::f_004034a8,
};
/// `DIntr` (0x42c078): disables interrupts, returns the previous state.
const OC1123_DINTR: Callee = Callee {
    addr: 0x0042_c078,
    f: ee::s15::f_0042c078,
};
/// `EIntr` (0x42c0c0): re-enables interrupts.
const OC1123_EINTR: Callee = Callee {
    addr: 0x0042_c0c0,
    f: ee::s15::f_0042c0c0,
};
/// `__muldi3` (0x4114d0): libgcc's 64-bit multiply.
const OC1123_MULDI3: Callee = Callee {
    addr: 0x0041_14d0,
    f: ee::s15::f_004114d0,
};
/// `libmpeg_IpuCommand` (0x406df0).
const OC1123_IPU_COMMAND: Callee = Callee {
    addr: 0x0040_6df0,
    f: ee::s15::f_00406df0,
};
/// `libmpeg_DispatchCallback` (0x402c30).
const OC1123_DISPATCH: Callee = Callee {
    addr: 0x0040_2c30,
    f: ee::s15::f_00402c30,
};
/// `AddDmacHandler` (0x423ac0): registers a DMA channel handler, returns its id.
const OC1123_ADD_HANDLER: Callee = Callee {
    addr: 0x0042_3ac0,
    f: ee::s15::f_00423ac0,
};
/// `RemoveDmacHandler` (0x423ad0).
const OC1123_REMOVE_HANDLER: Callee = Callee {
    addr: 0x0042_3ad0,
    f: ee::s15::f_00423ad0,
};
/// `EnableDmac` (0x4249b8).
const OC1123_ENABLE_DMAC: Callee = Callee {
    addr: 0x0042_49b8,
    f: ee::s15::f_004249b8,
};
/// `DisableDmac` (0x424950).
const OC1123_DISABLE_DMAC: Callee = Callee {
    addr: 0x0042_4950,
    f: ee::s15::f_00424950,
};

/// `int libmpeg_CheckOutputBuffer(MpegDec* dec, int* picSize)` (0x4081d8, 160
/// bytes). Whether the picture fits the decoder's output buffer: with
/// `dec[0xE0]` rows, the buffer is `dec[0xDC]` wide and `dec[0xE0]` high and the
/// picture `picSize[1]` x `picSize[2]`; without rows (`dec[0xE0] == 0`) the
/// buffer is `dec[0xE4]` bytes and the picture `picSize[3] * picSize[4]`. When
/// it does not fit, the required size goes to `libmpeg_Error` through a 256-byte
/// stack buffer. Returns the fits flag in v0.
pub fn oc1123_check_output_buffer(c: &mut Cpu) {
    guest(c, oc1123_check_output_buffer_body);
}

fn oc1123_check_output_buffer_body(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x130);
    let sp0 = sp(c);
    c.sd(sp0 + 0x100, c.g[S0][0]);
    c.sd(sp0 + 0x120, c.g[RA][0]);
    c.g[S0][0] = c.g[A0][0];
    c.sd(sp0 + 0x110, c.g[S1][0]);
    let dec = c.g[S0][0] as u32;
    c.g[A0][0] = Addr(dec + 0xe0).lw(c);
    c.g[A2][0] = c.g[A1][0];
    let pic = c.g[A2][0] as u32;
    if c.g[A0][0] == 0 {
        // Flat buffer: dec[0xE4] against picSize[3] * picSize[4].
        c.cycles += 6;
        c.g[V1][0] = Addr(pic + 0xc).lw(c);
        c.g[A0][0] = Addr(pic + 0x10).lw(c);
        c.g[V0][0] = Addr(dec + 0xe4).lw(c);
        c.mult(0, V1, V1, A0);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        c.g[S1][0] = c.g[V0][0] ^ 0x1;
    } else {
        // Rows: the width against picSize[1], then the rows against picSize[2].
        c.cycles += 5;
        c.g[V0][0] = Addr(dec + 0xdc).lw(c);
        c.g[V1][0] = Addr(pic + 0x4).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        c.g[S1][0] = 0;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(pic + 0x8).lw(c);
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[S1][0] = c.g[V0][0] ^ 0x1;
        }
    }

    c.cycles += 2;
    c.g[V0][0] = c.g[S1][0];
    if c.g[S1][0] == 0 {
        // Does not fit: report the picture size.
        c.cycles += 10;
        c.g[A3][0] = Addr(pic + 0x8).lw(c);
        c.g[A1][0] = sx(0x004a_0000u32.wrapping_sub(0x7f58));
        c.g[A2][0] = Addr(pic + 0x4).lw(c);
        c.g[A0][0] = sx(sp0);
        OC1123_SPRINTF.call(c, 0x0040_8254)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(sp0);
        OC1123_ERROR.call(c, 0x0040_8260)?;
        c.g[V0][0] = c.g[S1][0];
    }
    c.cycles += 5;
    c.g[RA][0] = c.ld(sp0 + 0x120);
    c.g[S1][0] = c.ld(sp0 + 0x110);
    c.g[S0][0] = c.ld(sp0 + 0x100);
    frame.close(c);
    Ok(())
}

/// `long libmpeg_ComputePts(MpegDec* dec, PicRecord* pic, long* ptsOut,
/// long* dtsOut, long* flagsOut)` (0x408530, 396 bytes). Presentation and
/// decode timestamps of a picture and its flag word. The PTS comes from the
/// picture record, or is extrapolated from the last PTS (`dec[0x80]`) with the
/// frame rate (`dec[0x78]`, `dec[0x88]`, the round counter `dec[0x90]`) when the
/// record has none. A pending PTS hand-off (`dec[0xF8] == 2`) overrides it.
/// Returns the flag word in v0.
pub fn oc1123_compute_pts(c: &mut Cpu) {
    guest(c, oc1123_compute_pts_body);
}

fn oc1123_compute_pts_body(c: &mut Cpu) -> Flow {
    c.cycles += 18;
    let frame = Frame::open(c, 0xb0);
    let sp0 = sp(c);
    c.sd(sp0 + 0x90, c.g[FP][0]);
    c.sd(sp0 + 0x60, c.g[S5][0]);
    c.g[FP][0] = c.g[A3][0];
    c.sd(sp0 + 0x50, c.g[S4][0]);
    c.g[S5][0] = c.g[A2][0];
    c.sd(sp0 + 0x40, c.g[S3][0]);
    c.g[S4][0] = c.g[A1][0];
    c.sd(sp0 + 0xa0, c.g[RA][0]);
    c.g[S3][0] = c.g[A0][0];
    c.sd(sp0 + 0x80, c.g[S7][0]);
    c.sd(sp0 + 0x70, c.g[S6][0]);
    c.sd(sp0 + 0x30, c.g[S2][0]);
    c.sd(sp0 + 0x20, c.g[S1][0]);
    c.sd(sp0 + 0x10, c.g[S0][0]);

    let dec = c.g[S3][0] as u32;
    let pic = c.g[S4][0] as u32;
    let pts_out = c.g[S5][0] as u32;
    c.g[V0][0] = Addr(dec + 0x70).lw(c);
    let no_state = c.g[V0][0] == 0;
    // The fifth argument (flagsOut) is in t0; it is stored in the caller's
    // argument slot and read back at the end.
    c.sw(sp0, c.g[T0][0]);
    if no_state {
        c.cycles += 2;
        c.g[V0][0] = Addr(pic + 0x18).ld(c);
        c.sd(pts_out, c.g[V0][0]);
    } else {
        c.cycles += 3;
        c.g[V0][0] = Addr(pic + 0x18).ld(c);
        if (c.g[V0][0] as i64) >= 0 {
            c.sd(pts_out, c.g[V0][0]);
        } else {
            c.cycles += 3;
            c.g[S7][0] = Addr(dec + 0x80).lw(c);
            if (c.g[S7][0] as i64) < 0 {
                c.sd(pts_out, c.g[V0][0]);
            } else {
                oc1123_pts_extrapolate(c, dec, pts_out)?;
            }
        }
    }

    // A pending hand-off (fF8 == 2 with fF0 >= 0) overrides the PTS and
    // clears itself. The flags are taken from the picture either way.
    c.cycles += 4;
    c.g[V1][0] = Addr(dec + 0xf8).lw(c);
    c.g[V0][0] = 2;
    if c.g[V1][0] != c.g[V0][0] {
        c.g[A1][0] = Addr(pic + 0x40).lw(c);
    } else {
        c.cycles += 3;
        c.g[V0][0] = Addr(dec + 0xf0).ld(c);
        if (c.g[V0][0] as i64) < 0 {
            c.g[A1][0] = Addr(pic + 0x40).lw(c);
        } else {
            c.cycles += 5;
            c.sd(pts_out, c.g[V0][0]);
            c.g[V0][0] = u64::MAX;
            c.sw(dec + 0xf8, 0);
            c.sd(dec + 0xf0, c.g[V0][0]);
            c.g[A1][0] = Addr(pic + 0x40).lw(c);
        }
    }

    c.cycles += 31;
    c.g[A0][0] = Addr(pic + 0x3c).lw(c);
    c.g[V0][0] = Addr(pic + 0x34).lw(c);
    c.g[A1][0] = c.g[A1][0] << 5;
    c.g[A0][0] = c.g[A0][0] << 6;
    c.g[A2][0] = Addr(pic + 0x30).lw(c);
    c.g[A3][0] = Addr(pic + 0x2c).lw(c);
    c.g[A1][0] = c.g[A1][0] | c.g[A0][0];
    c.g[V1][0] = Addr(pic + 0x38).lw(c);
    c.g[V0][0] = c.g[V0][0] << 8;
    c.g[A0][0] = Addr(pic + 0x20).ld(c);
    c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
    c.g[A2][0] = c.g[A2][0] << 3;
    c.g[V1][0] = c.g[V1][0] << 7;
    // *dtsOut = picRecord->dts
    c.sd(c.g[FP][0] as u32, c.g[A0][0]);
    c.g[V1][0] = c.g[V1][0] | c.g[A2][0];
    c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
    c.g[RA][0] = c.ld(sp0 + 0xa0);
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[FP][0] = c.ld(sp0 + 0x90);
    c.g[V1][0] = Addr(sp0).lw(c);
    c.g[S7][0] = c.ld(sp0 + 0x80);
    c.g[S6][0] = c.ld(sp0 + 0x70);
    c.g[S5][0] = c.ld(sp0 + 0x60);
    c.g[S4][0] = c.ld(sp0 + 0x50);
    c.g[S3][0] = c.ld(sp0 + 0x40);
    c.g[S2][0] = c.ld(sp0 + 0x30);
    c.g[S1][0] = c.ld(sp0 + 0x20);
    c.g[S0][0] = c.ld(sp0 + 0x10);
    // *flagsOut = picFlags (the flags word, the return value too)
    c.sd(c.g[V1][0] as u32, c.g[V0][0]);
    frame.close(c);
    Ok(())
}

/// The extrapolated PTS of `libmpeg_ComputePts` (its branch with a last PTS
/// and no PTS on the picture): `lastPts + (f78 * x >> 1) + y`, with `x` =
/// `dec[0x88]`, `y` = the odd-round correction, and the round counter bumped
/// when both frame-rate parities are odd. Writes the PTS to `pts_out`.
fn oc1123_pts_extrapolate(c: &mut Cpu, dec: u32, pts_out: u32) -> Flow {
    c.cycles += 28;
    c.g[S0][0] = Addr(dec + 0x88).ld(c);
    c.g[A1][0] = Addr(dec + 0x78).ld(c);
    c.g[S0][0] = c.g[S0][0] << 32;
    c.g[S0][0] = ((c.g[S0][0] as i64) >> 32) as u64;
    c.g[S2][0] = c.g[S0][0] & 0x1;
    c.g[A1][0] &= 0x1;
    c.g[A0][0] = c.g[S2][0];
    // bothOdd = (x & 1) * (f78 & 1)
    OC1123_MULDI3.call(c, 0x0040_85b0)?;
    c.g[S6][0] = Addr(dec + 0x90).lw(c);
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S6][0] & 0x1;
    // y = bothOdd * (roundCount & 1)
    OC1123_MULDI3.call(c, 0x0040_85c0)?;
    c.g[A0][0] = Addr(dec + 0x78).ld(c);
    c.g[S1][0] = c.g[V0][0] << 32;
    c.g[S1][0] = ((c.g[S1][0] as i64) >> 32) as u64;
    c.g[A1][0] = c.g[S0][0];
    // f78 * x, halved (bits 1..32) and added to lastPts and y
    OC1123_MULDI3.call(c, 0x0040_85d4)?;
    c.g[V0][0] = c.g[V0][0] << 31;
    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
    c.g[A0][0] = c.g[S2][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
    c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sd(pts_out, c.g[V0][0]);
    // (x & 1) * (f78 & 1): the round counter moves when it is non-zero
    c.g[A1][0] = Addr(dec + 0x78).ld(c);
    c.g[A1][0] &= 0x1;
    OC1123_MULDI3.call(c, 0x0040_85f8)?;
    let bump = c.g[V0][0] != 0;
    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
    if bump {
        c.cycles += 2;
        c.sw(dec + 0x90, c.g[V0][0]);
    }
    Ok(())
}

/// `void libmpeg_IpuFromDmaStart(u32 dst, int byteCount)` (0x408af8, 132
/// bytes). Starts a DMA transfer from the IPU into RAM (channel 3, the
/// `0x1000B000` block: address, quadword count, start) with the interrupts
/// off, then restores the interrupt state (EIntr as a tail call when they were
/// on).
pub fn oc1123_ipu_from_dma_start(c: &mut Cpu) {
    guest(c, oc1123_ipu_from_dma_start_body);
}

fn oc1123_ipu_from_dma_start_body(c: &mut Cpu) -> Flow {
    c.cycles += 25;
    let frame = Frame::open(c, 0x30);
    let sp0 = sp(c);
    c.sd(sp0 + 0x10, c.g[S1][0]);
    c.sd(sp0, c.g[S0][0]);
    c.g[S1][0] = c.g[A1][0];
    c.sd(sp0 + 0x20, c.g[RA][0]);
    c.g[S0][0] = c.g[A0][0];
    OC1123_DINTR.call(c, 0x0040_8b14)?;
    c.g[V1][0] = 0x0fff_ffff;
    c.g[A2][0] = 0xffff_ffff_8000_0000;
    c.g[S0][0] &= c.g[V1][0];
    c.g[A1][0] = 0x1000_b010;
    c.g[S0][0] |= c.g[A2][0];
    // MADR = (dst & 0x0fffffff) | 0x80000000 (the uncached mirror)
    c.sw(c.g[A1][0] as u32, c.g[S0][0]);
    c.g[A0][0] = 0x1000_b020;
    c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 4) as u32);
    c.g[V1][0] = 0x1000_0000;
    // QWC = byteCount / 16
    c.sw(c.g[A0][0] as u32, c.g[S1][0]);
    c.g[V1][0] = 0x1000_b000;
    c.g[A0][0] = 0x100;
    c.sw(c.g[V1][0] as u32, c.g[A0][0]);
    let intr_was_off = c.g[V0][0] == 0;
    c.g[RA][0] = c.ld(sp0 + 0x20);
    if intr_was_off {
        c.cycles += 4;
        c.g[S1][0] = c.ld(sp0 + 0x10);
        c.g[S0][0] = c.ld(sp0);
        frame.close(c);
    } else {
        c.cycles += 4;
        c.g[S1][0] = c.ld(sp0 + 0x10);
        c.g[S0][0] = c.ld(sp0);
        // Tail call: EIntr returns straight to our caller.
        c.g[SP][0] = sx(sp0.wrapping_add(0x30));
        c.pc = 0x0042_c0c0;
        (OC1123_EINTR.f)(c);
    }
    Ok(())
}

/// `void libmpeg_IpuCsc(MpegDec* dec, u32 addr, int n)` (0x408b80, 296 bytes).
/// Converts `n` macroblocks to RGB with the IPU, the DMA writing to `addr`
/// (channel 3, quadword count `n << 6`). It waits for the IPU to go idle, runs
/// the IPU command `n | 0x70000000` through `libmpeg_IpuCommand`, then waits
/// for the DMA to finish. Interrupts are off during the start (EIntr when they
/// were on).
pub fn oc1123_ipu_csc(c: &mut Cpu) {
    guest(c, oc1123_ipu_csc_body);
}

fn oc1123_ipu_csc_body(c: &mut Cpu) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x70);
    let sp0 = sp(c);
    c.g[V1][0] = 0x1000_2010;
    c.sd(sp0 + 0x50, c.g[S3][0]);
    c.sd(sp0 + 0x40, c.g[S2][0]);
    c.g[S3][0] = c.g[A2][0];
    c.sd(sp0 + 0x30, c.g[S1][0]);
    c.g[S2][0] = c.g[A0][0];
    c.sd(sp0 + 0x20, c.g[S0][0]);
    c.g[S1][0] = sx((c.g[S3][0] as u32) << 6);
    c.sd(sp0 + 0x60, c.g[RA][0]);
    c.g[S0][0] = c.g[A1][0];

    // Wait while the IPU is busy (IPU control word, sign bit).
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(0x1000_2010).lw(c);
        if (c.g[V0][0] as i64) >= 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0040_8bb0);
        }
    }

    c.cycles += 17;
    OC1123_DINTR.call(c, 0x0040_8bd4)?;
    c.g[V1][0] = 0x0fff_ffff;
    c.g[A1][0] = 0x1000_b010;
    c.g[V1][0] = c.g[S0][0] & c.g[V1][0];
    c.g[A0][0] = 0x1000_0000;
    // MADR = addr & 0x0fffffff
    c.sw(c.g[A1][0] as u32, c.g[V1][0]);
    c.g[A0][0] = 0x1000_b020;
    // QWC = n << 6
    c.sw(c.g[A0][0] as u32, c.g[S1][0]);
    c.g[V1][0] = 0x1000_b000;
    c.g[A0][0] = 0x100;
    c.sw(c.g[V1][0] as u32, c.g[A0][0]);
    if c.g[V0][0] == 0 {
        c.g[A1][0] = 0x7000_0000;
    } else {
        c.cycles += 3;
        OC1123_EINTR.call(c, 0x0040_8c18)?;
        c.g[A1][0] = 0x7000_0000;
    }

    c.cycles += 11;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S3][0] | c.g[A1][0];
    OC1123_IPU_COMMAND.call(c, 0x0040_8c28)?;
    c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x858)).lw(c);
    c.g[V0][0] = 0x4;
    c.sw(sp0, c.g[V0][0]);
    c.g[A1][0] = sx(sp0);
    OC1123_DISPATCH.call(c, 0x0040_8c3c)?;
    c.g[V1][0] = 0x1000_b000;

    // Wait for the DMA channel to finish (STR, bit 8 of CHCR).
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(0x1000_b000).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
        c.g[V0][0] &= 0x1;
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0040_8c48);
        }
    }

    c.cycles += 3;
    c.g[V1][0] = 0x1000_2010;
    // Then wait for the IPU to go idle again.
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(0x1000_2010).lw(c);
        if (c.g[V0][0] as i64) >= 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0040_8c70);
        }
    }

    c.cycles += 7;
    c.g[RA][0] = c.ld(sp0 + 0x60);
    c.g[S3][0] = c.ld(sp0 + 0x50);
    c.g[S2][0] = c.ld(sp0 + 0x40);
    c.g[S1][0] = c.ld(sp0 + 0x30);
    c.g[S0][0] = c.ld(sp0 + 0x20);
    frame.close(c);
    Ok(())
}

/// `void libmpeg_IpuCscDma(MpegDec* dec, u32 addr, int size)` (0x408df0, 448
/// bytes). The long-picture variant of `libmpeg_IpuCsc`: it registers a DMA
/// chunk handler (`0x408ca8`) that feeds the transfer in 1023-quadword chunks
/// (`n = size / 1023 + 1` of them, counted in `cnt` in the transfer state),
/// starts the IPU CSC command, waits for all chunks, and reports a handler
/// stop through `libmpeg_Error`. Interrupts are off during the start.
pub fn oc1123_ipu_csc_dma(c: &mut Cpu) {
    guest(c, oc1123_ipu_csc_dma_body);
}

fn oc1123_ipu_csc_dma_body(c: &mut Cpu) -> Flow {
    c.cycles += 16;
    c.g[A3][0] = 0x3ff;
    let frame = Frame::open(c, 0x80);
    c.div(0, A2, A3);
    let sp0 = sp(c);
    c.sd(sp0 + 0x40, c.g[S0][0]);
    // addr + 0xFFC00, masked to 28 bits
    c.g[V0][0] = 0x000f_fc00;
    c.g[S0][0] = c.g[A1][0];
    c.g[V1][0] = 0x0fff_ffff;
    c.sd(sp0 + 0x50, c.g[S1][0]);
    c.sd(sp0 + 0x70, c.g[RA][0]);
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sd(sp0 + 0x60, c.g[S2][0]);
    c.g[V0][0] &= c.g[V1][0];

    // The transfer state on the stack: cnt (+0x20), stop (+0x24), size
    // (+0x28), addr (+0x2c), n (+0x30).
    c.cycles += 12;
    c.g[V1][0] = 0x1000_0000;
    c.sw(sp0 + 0x28, c.g[A2][0]);
    c.g[S1][0] = c.g[A0][0];
    c.sw(sp0 + 0x2c, c.g[V0][0]);
    c.g[A3][0] = sx(sp0 + 0x20);
    c.sw(sp0 + 0x24, 0);
    c.g[V1][0] |= 0x2010;
    c.sw(sp0 + 0x20, 0);
    c.g[T0][0] = c.lo[0];
    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
    c.sw(sp0 + 0x30, c.g[T0][0]);

    // Wait while the IPU is busy.
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(0x1000_2010).lw(c);
        if (c.g[V0][0] as i64) >= 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0040_8e60);
        }
    }

    c.cycles += 30;
    c.g[A1][0] = 0x0041_0000;
    c.g[A0][0] = 0x3;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7358));
    c.g[A2][0] = 0;
    // Chunk handler 0x408ca8 on DMA channel 3.
    OC1123_ADD_HANDLER.call(c, 0x0040_8e90)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[V1][0] = 0x8;
    c.g[V0][0] = 0x1000_e010;
    c.g[A0][0] = 0x3;
    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
    OC1123_ENABLE_DMAC.call(c, 0x0040_8eac)?;
    OC1123_DINTR.call(c, 0x0040_8eb4)?;
    c.g[V1][0] = 0x0fff_ffff;
    c.g[A0][0] = 0x1000_b010;
    c.g[V1][0] = c.g[S0][0] & c.g[V1][0];
    c.g[A1][0] = 0x1000_0000;
    // MADR = addr & 0x0fffffff
    c.sw(c.g[A0][0] as u32, c.g[V1][0]);
    c.g[A1][0] |= 0xb020;
    c.g[V1][0] = 0xffc0;
    c.g[A0][0] = 0x1000_0000;
    // QWC = 0xffc0 (one chunk)
    c.sw(c.g[A1][0] as u32, c.g[V1][0]);
    c.g[A0][0] |= 0xb000;
    c.g[V1][0] = 0x100;
    c.sw(c.g[A0][0] as u32, c.g[V1][0]);
    let intr_was_off = c.g[V0][0] == 0;
    c.g[V0][0] = 0x1000_0000;
    if !intr_was_off {
        c.cycles += 3;
        OC1123_EINTR.call(c, 0x0040_8efc)?;
        c.g[V0][0] = 0x1000_0000;
    }

    // Start the IPU CSC command (0x70000000 | 3 ... the DMA-driven form).
    c.cycles += 12;
    c.g[V1][0] = 0x7000_0000;
    c.g[V0][0] |= 0x2000;
    c.g[V1][0] |= 0x3ff;
    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
    c.g[A2][0] = 0x4;
    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x858)).lw(c);
    c.g[A1][0] = sx(sp0);
    c.sw(sp0, c.g[A2][0]);
    OC1123_DISPATCH.call(c, 0x0040_8f24)?;
    c.g[A0][0] = Addr(sp0 + 0x24).lw(c);
    c.g[V1][0] = Addr(sp0 + 0x30).lw(c);

    // Wait until every chunk has been fed (cnt < n).
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(sp0 + 0x20).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0040_8f30);
        }
    }

    // A handler stop is reported through libmpeg_Error.
    c.cycles += 2;
    let stopped = c.g[A0][0] != 0;
    c.g[A0][0] = c.g[S1][0];
    if stopped {
        c.cycles += 3;
        c.g[A1][0] = sx(0x004a_0000u32.wrapping_sub(0x7f28));
        OC1123_ERROR.call(c, 0x0040_8f60)?;
    }

    c.cycles += 2;
    c.g[V1][0] = 0x1000_2010;
    // Wait for the IPU to go idle.
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(0x1000_2010).lw(c);
        if (c.g[V0][0] as i64) >= 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0040_8f68);
        }
    }

    c.cycles += 11;
    c.g[A0][0] = 0x3;
    OC1123_DISABLE_DMAC.call(c, 0x0040_8f8c)?;
    c.g[A1][0] = c.g[S2][0];
    c.g[A0][0] = 0x3;
    OC1123_REMOVE_HANDLER.call(c, 0x0040_8f98)?;
    c.g[RA][0] = c.ld(sp0 + 0x70);
    c.g[S2][0] = c.ld(sp0 + 0x60);
    c.g[S1][0] = c.ld(sp0 + 0x50);
    c.g[S0][0] = c.ld(sp0 + 0x40);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, Rng, DATA};
    use std::ptr::addr_of_mut;

    /// Gate of the callee fakes: they answer only while this word holds
    /// `OC1123_MAGIC` (otherwise the bodies they replaced run).
    const OC1123_CTRL: u32 = DATA + 0x0079_0000;
    const OC1123_MAGIC: u32 = 0x1123_c0de;
    /// The DIntr result the fake returns (0 or not), and the stop flag the
    /// chunk-handler fake leaves in the transfer state.
    const OC1123_INTR: u32 = DATA + 0x0079_0010;
    const OC1123_STOP: u32 = DATA + 0x0079_0018;
    /// The decoder state and the picture record.
    const OC1123_DEC: u32 = DATA + 0x0079_1000;
    const OC1123_PIC: u32 = DATA + 0x0079_2000;
    /// ComputePts' outputs: PTS, DTS, flags.
    const OC1123_OUT: u32 = DATA + 0x0079_3000;
    /// The mpeg system state (its +0x858 field is the dispatch object).
    const OC1123_MPEG: u32 = DATA + 0x0079_4000;
    const OC1123_OBJ: u32 = DATA + 0x0079_5000;

    fn oc1123_gated(c: &mut Cpu) -> bool {
        c.bus.read32(OC1123_CTRL) == OC1123_MAGIC
    }

    /// A callee fake that logs the call and returns (gate on), or runs the
    /// body it replaced (gate off).
    macro_rules! oc1123_fake {
        ($name:ident, $target:expr, $generated:path) => {
            fn $name(c: &mut Cpu) {
                if !oc1123_gated(c) {
                    return fakes::fall_back(c, fakes::prev_cell($name), $generated);
                }
                fakes::fake_call(c, $target);
            }
        };
    }

    oc1123_fake!(oc1123_fake_sprintf, 0x0041_7828, ee::s15::f_00417828__generated);
    oc1123_fake!(oc1123_fake_error, 0x0040_34a8, ee::s15::f_004034a8__generated);
    oc1123_fake!(oc1123_fake_eintr, 0x0042_c0c0, ee::s15::f_0042c0c0__generated);
    oc1123_fake!(oc1123_fake_ipu_command, 0x0040_6df0, ee::s15::f_00406df0__generated);
    oc1123_fake!(oc1123_fake_enable, 0x0042_49b8, ee::s15::f_004249b8__generated);
    oc1123_fake!(oc1123_fake_disable, 0x0042_4950, ee::s15::f_00424950__generated);
    oc1123_fake!(oc1123_fake_remove, 0x0042_3ad0, ee::s15::f_00423ad0__generated);

    /// DIntr: returns the state the test put in OC1123_INTR (zero included).
    fn oc1123_fake_dintr(c: &mut Cpu) {
        if !oc1123_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1123_fake_dintr),
                ee::s15::f_0042c078__generated,
            );
        }
        let state = c.bus.read32(OC1123_INTR);
        fakes::fake_call(c, 0x0042_c078);
        c.g[V0][0] = state as u64;
    }

    /// AddDmacHandler: the chunk handler is never run; the transfer state
    /// (a3) is left as if every chunk was fed, with the stop flag from the test.
    fn oc1123_fake_add_handler(c: &mut Cpu) {
        if !oc1123_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1123_fake_add_handler),
                ee::s15::f_00423ac0__generated,
            );
        }
        let state = c.g[A3][0] as u32;
        let n = c.bus.read32(state + 0x10);
        let stop = c.bus.read32(OC1123_STOP);
        fakes::fake_call(c, 0x0042_3ac0);
        c.bus.write32(state, n);
        c.bus.write32(state + 4, stop);
    }

    /// DispatchCallback: the DMA the caller started has finished (CHCR's STR
    /// bit is cleared, the way the IPU would clear it), and the IPU is idle:
    /// the CSC command has no input in the test machine, so BUSY would stay
    /// set. A CTRL write with RST (bit 30) resets the IPU and clears BUSY.
    fn oc1123_fake_dispatch(c: &mut Cpu) {
        if !oc1123_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1123_fake_dispatch),
                ee::s15::f_00402c30__generated,
            );
        }
        fakes::fake_call(c, 0x0040_2c30);
        c.sw(0x1000_b000, 0);
        c.sw(0x1000_2010, 1u64 << 30);
    }

    fn oc1123_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            oc1123_fake_sprintf,
            fakes::prev_cell(oc1123_fake_sprintf),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004034A8),
            oc1123_fake_error,
            fakes::prev_cell(oc1123_fake_error),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042C078),
            oc1123_fake_dintr,
            fakes::prev_cell(oc1123_fake_dintr),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042C0C0),
            oc1123_fake_eintr,
            fakes::prev_cell(oc1123_fake_eintr),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00406DF0),
            oc1123_fake_ipu_command,
            fakes::prev_cell(oc1123_fake_ipu_command),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00402C30),
            oc1123_fake_dispatch,
            fakes::prev_cell(oc1123_fake_dispatch),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00423AC0),
            oc1123_fake_add_handler,
            fakes::prev_cell(oc1123_fake_add_handler),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00423AD0),
            oc1123_fake_remove,
            fakes::prev_cell(oc1123_fake_remove),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004249B8),
            oc1123_fake_enable,
            fakes::prev_cell(oc1123_fake_enable),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00424950),
            oc1123_fake_disable,
            fakes::prev_cell(oc1123_fake_disable),
        );
    }

    fn oc1123_u64(r: &mut Rng) -> u64 {
        (r.next_u32() as u64) << 32 | r.next_u32() as u64
    }

    /// Small signed values (-8..=15) as 32-bit words.
    fn oc1123_small(r: &mut Rng) -> u32 {
        (r.next_u32() % 24).wrapping_sub(8)
    }

    #[test]
    fn oc1123_check_output_buffer_matches() {
        oc1123_install_fakes();
        check(
            ee::s15::f_004081d8__generated,
            oc1123_check_output_buffer,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1123_0001, 2);
                c.pc = 0x0040_81d8;
                c.bus.write32(OC1123_CTRL, OC1123_MAGIC);
                // Rows case (seed % 4 != 0) and the flat case (rows == 0).
                c.bus.write32(OC1123_DEC + 0xdc, oc1123_small(&mut r));
                let rows = if seed % 4 == 0 { 0 } else { oc1123_small(&mut r) };
                c.bus.write32(OC1123_DEC + 0xe0, rows);
                c.bus.write32(OC1123_DEC + 0xe4, oc1123_small(&mut r));
                c.bus.write32(OC1123_PIC + 0x4, oc1123_small(&mut r));
                c.bus.write32(OC1123_PIC + 0x8, oc1123_small(&mut r));
                c.bus.write32(OC1123_PIC + 0xc, oc1123_small(&mut r));
                c.bus.write32(OC1123_PIC + 0x10, oc1123_small(&mut r));
                c.g[A0][0] = sx(OC1123_DEC);
                c.g[A1][0] = sx(OC1123_PIC);
            },
        );
    }

    #[test]
    fn oc1123_compute_pts_matches() {
        check(
            ee::s15::f_00408530__generated,
            oc1123_compute_pts,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1123_0002, 0);
                c.pc = 0x0040_8530;
                c.g[A0][0] = sx(OC1123_DEC);
                c.g[A1][0] = sx(OC1123_PIC);
                c.g[A2][0] = sx(OC1123_OUT);
                c.g[A3][0] = sx(OC1123_OUT + 0x8);
                c.g[T0][0] = sx(OC1123_OUT + 0x10);
                let d = OC1123_DEC;
                // No state (f70 == 0) on a quarter of the seeds.
                let f70 = if seed % 4 == 0 { 0 } else { r.next_u32() | 0x1 };
                c.bus.write32(d + 0x70, f70);
                c.bus.write64(d + 0x78, oc1123_u64(&mut r));
                c.bus.write64(d + 0x88, oc1123_u64(&mut r));
                // The last PTS: negative on a fifth of the seeds.
                let last = r.next_u32();
                let last = if seed % 5 == 0 {
                    last | 0x8000_0000
                } else {
                    last & 0x7fff_ffff
                };
                c.bus.write32(d + 0x80, last);
                c.bus.write32(d + 0x90, r.next_u32());
                // The hand-off: fF8 == 2 on a seventh, fF0 negative on half.
                let f0 = oc1123_u64(&mut r);
                let f0 = if seed % 2 == 0 {
                    f0 | (1u64 << 63)
                } else {
                    f0 & !(1u64 << 63)
                };
                c.bus.write64(d + 0xf0, f0);
                c.bus.write32(d + 0xf8, if seed % 7 == 0 { 2 } else { r.next_u32() % 4 });
                // The picture record: PTS negative on a third of the seeds.
                let p = OC1123_PIC;
                let pts = oc1123_u64(&mut r);
                let pts = if seed % 3 == 0 {
                    pts | (1u64 << 63)
                } else {
                    pts & !(1u64 << 63)
                };
                c.bus.write64(p + 0x18, pts);
                c.bus.write64(p + 0x20, oc1123_u64(&mut r));
                for off in [0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40] {
                    c.bus.write32(p + off, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn oc1123_ipu_from_dma_start_matches() {
        oc1123_install_fakes();
        check(
            ee::s15::f_00408af8__generated,
            oc1123_ipu_from_dma_start,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1123_0003, 2);
                c.pc = 0x0040_8af8;
                c.bus.write32(OC1123_CTRL, OC1123_MAGIC);
                // Interrupts were off (0) or on, on alternate seeds.
                let intr = if seed % 2 == 0 { 0 } else { r.next_u32() | 0x1 };
                c.bus.write32(OC1123_INTR, intr);
                c.g[A0][0] = sx(r.next_u32());
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn oc1123_ipu_csc_matches() {
        oc1123_install_fakes();
        check(
            ee::s15::f_00408b80__generated,
            oc1123_ipu_csc,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1123_0004, 2);
                c.pc = 0x0040_8b80;
                c.bus.write32(OC1123_CTRL, OC1123_MAGIC);
                let intr = if seed % 2 == 0 { 0 } else { r.next_u32() | 0x1 };
                c.bus.write32(OC1123_INTR, intr);
                c.bus.write32(OC1123_MPEG + 0x858, OC1123_OBJ);
                c.g[A0][0] = sx(OC1123_MPEG);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32() % 64);
            },
        );
    }

    #[test]
    fn oc1123_ipu_csc_dma_matches() {
        oc1123_install_fakes();
        check(
            ee::s15::f_00408df0__generated,
            oc1123_ipu_csc_dma,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1123_0005, 2);
                c.pc = 0x0040_8df0;
                c.bus.write32(OC1123_CTRL, OC1123_MAGIC);
                let intr = if seed % 2 == 0 { 0 } else { r.next_u32() | 0x1 };
                c.bus.write32(OC1123_INTR, intr);
                // The chunk handler reports a stop on a fifth of the seeds.
                c.bus.write32(OC1123_STOP, if seed % 5 == 0 { 1 } else { 0 });
                c.bus.write32(OC1123_MPEG + 0x858, OC1123_OBJ);
                c.g[A0][0] = sx(OC1123_MPEG);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32() % 0x2000);
            },
        );
    }
}
