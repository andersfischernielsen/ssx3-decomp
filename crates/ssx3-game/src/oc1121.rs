//! Staging module oc1121 (wave wave270): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1121`.

#![allow(unused_imports)]

use crate::abi;
use crate::call::{guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0040_60c8, libmpeg_decode_slices, "libmpeg_DecodeSlices"),
    (0x0040_6510, libmpeg_skipped_macroblock, "libmpeg_SkippedMacroblock"),
    (0x0040_6a78, libmpeg_decode_motion_vector, "libmpeg_DecodeMotionVector"),
    (0x0040_6b00, libmpeg_motion_vectors, "libmpeg_MotionVectors"),
    (0x0040_6ca0, libmpeg_motion_vector, "libmpeg_MotionVector"),
    (0x0040_6ec8, libmpeg_ipu_wait_cmd, "libmpeg_IpuWaitCmd"),
    (0x0040_74b8, libmpeg_slice_header_tail, "libmpeg_SliceHeaderTail"),
];

/// `sd reg, off(sp)` into the frame (the value is read before the store).
macro_rules! oc1121_sd {
    ($c:ident, $frame:expr, $off:expr, $reg:expr) => {{
        let v = $c.g[$reg][0];
        let a = $frame.at($c, $off);
        Addr(a).sd($c, v);
    }};
}

/// `ld reg, off(sp)` from the frame.
macro_rules! oc1121_ld {
    ($c:ident, $frame:expr, $off:expr, $reg:expr) => {{
        let a = $frame.at($c, $off);
        $c.g[$reg][0] = Addr(a).ld($c);
    }};
}

/// `libmpeg_DecodeSlice(d, n)` (0x406318): one slice; 1 and 3 ask for a re-run.
pub(crate) const OC1121_DECODE_SLICE: Callee = Callee {
    addr: 0x0040_6318,
    f: ee::s15::f_00406318,
};
/// `libmpeg_IpuWaitBusy(d)` (0x406e20).
pub(crate) const OC1121_IPU_WAIT_BUSY: Callee = Callee {
    addr: 0x0040_6e20,
    f: ee::s15::f_00406e20,
};
/// `libmpeg_WaitBdec(d)` (0x405b70): nonzero when the bitstream decoder failed.
pub(crate) const OC1121_WAIT_BDEC: Callee = Callee {
    addr: 0x0040_5b70,
    f: ee::s15::f_00405b70,
};
/// `libmpeg_AddPredAndStore(d, skipped)` (0x404b78).
pub(crate) const OC1121_ADD_PRED_AND_STORE: Callee = Callee {
    addr: 0x0040_4b78,
    f: ee::s15::f_00404b78,
};
/// `libmpeg_Error(d, msg)` (0x4034a8).
pub(crate) const OC1121_ERROR: Callee = Callee {
    addr: 0x0040_34a8,
    f: ee::s15::f_004034a8,
};
/// `libmpeg_GetBits(d, n)` (0x4072e8): reads `n` bits from the stream.
pub(crate) const OC1121_GET_BITS: Callee = Callee {
    addr: 0x0040_72e8,
    f: ee::s15::f_004072e8,
};
/// `libmpeg_FlushBits(d, n)` (0x4071f8).
pub(crate) const OC1121_FLUSH_BITS: Callee = Callee {
    addr: 0x0040_71f8,
    f: ee::s15::f_004071f8,
};
/// `libmpeg_IpuVdec(d, table)` (0x406f78): one variable-length code.
pub(crate) const OC1121_IPU_VDEC: Callee = Callee {
    addr: 0x0040_6f78,
    f: ee::s15::f_00406f78,
};
/// `libmpeg_DecodeDmVector(d)` (0x405e10).
pub(crate) const OC1121_DECODE_DM_VECTOR: Callee = Callee {
    addr: 0x0040_5e10,
    f: ee::s15::f_00405e10,
};
/// `libmpeg_MotionVector(...)` (0x406ca0): both vectors of one block.
pub(crate) const OC1121_MOTION_VECTOR: Callee = Callee {
    addr: 0x0040_6ca0,
    f: ee::s15::f_00406ca0,
};
/// `libmpeg_DecodeMotionVector(...)` (0x406a78): one vector component.
pub(crate) const OC1121_DECODE_MOTION_VECTOR: Callee = Callee {
    addr: 0x0040_6a78,
    f: ee::s15::f_00406a78,
};
/// `CallbackNodata(d)` (0x402c80): the host callback when the IPU stalls.
pub(crate) const OC1121_CALLBACK_NODATA: Callee = Callee {
    addr: 0x0040_2c80,
    f: ee::s15::f_00402c80,
};
/// `libmpeg_ExtraBitInformation(d)` (0x407a10).
pub(crate) const OC1121_EXTRA_BIT_INFO: Callee = Callee {
    addr: 0x0040_7a10,
    f: ee::s15::f_00407a10,
};

/// `int libmpeg_DecodeSlices(MpegDec* d)` (0x4060c8): decodes the slices of a
/// picture. The slice count `n` (`d->w * d->h`, halved unless the picture is
/// a frame) drives `libmpeg_DecodeSlice` until it stops asking for a re-run
/// (result 1 or 3). Then it waits for the IPU and the bitstream decoder,
/// retries a failed slice once, reports errors through `libmpeg_Error`, and
/// returns whether the slice result was 0.
pub fn libmpeg_decode_slices(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x60);
        oc1121_sd!(c, frame, 0x40, S4);
        oc1121_sd!(c, frame, 0x30, S3);
        c.g[S4][0] = 0x1;
        oc1121_sd!(c, frame, 0x20, S2);
        c.g[S3][0] = 0x3;
        oc1121_sd!(c, frame, 0x10, S1);
        c.g[S2][0] = c.g[A0][0];
        oc1121_sd!(c, frame, 0x50, RA);
        oc1121_sd!(c, frame, 0x0, S0);

        let st = c.g[S2][0] as u32;
        Addr(st.wrapping_add(0x810)).sw(c, 0);
        c.g[V0][0] = Addr(st.wrapping_add(0x130)).lw(c);
        c.g[A0][0] = Addr(st.wrapping_add(0x12c)).lw(c);
        c.g[V1][0] = Addr(st.wrapping_add(0x174)).lw(c);
        c.mult(0, S1, A0, V0);
        Addr(st.wrapping_add(0x814)).sw(c, 0);
        c.g[V1][0] ^= 0x3;
        c.g[V0][0] = sx(((c.g[S1][0] as u32) as i32 >> 1) as u32);
        if c.g[V1][0] != 0 {
            c.g[S1][0] = c.g[V0][0];
        }

        // `'l_406118`: the outer loop (result 3 re-runs it).
        loop {
            c.cycles += 1;
            c.g[A0][0] = c.g[S2][0];
            // `'l_40611c`: the inner loop (result 1 re-runs it).
            let rerun_outer = loop {
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                OC1121_DECODE_SLICE.call(c, 0x0040_6124)?;
                c.g[S0][0] = c.g[V0][0];
                let again = c.g[S0][0] == c.g[S4][0];
                c.g[A0][0] = c.g[S2][0];
                if again {
                    if event_due(c) {
                        deliver_event_here(c, 0x0040_611c);
                    }
                    continue;
                }
                c.cycles += 2;
                if c.g[S0][0] == c.g[S3][0] {
                    if event_due(c) {
                        deliver_event_here(c, 0x0040_6118);
                    }
                    break true;
                }
                break false;
            };
            if !rerun_outer {
                break;
            }
        }

        // `'b_40616c`: wait for the IPU, then for the bitstream decoder.
        c.cycles += 6;
        c.g[A0][0] = c.g[S2][0];
        OC1121_IPU_WAIT_BUSY.call(c, 0x0040_6140)?;
        c.g[A0][0] = c.g[S2][0];
        OC1121_WAIT_BDEC.call(c, 0x0040_6148)?;
        let bdec_failed = c.g[V0][0] != 0;
        c.g[V1][0] = 0x1000_0000;
        if !bdec_failed {
            // `'b_406164`: a slice that failed earlier is retried once.
            c.cycles += 3;
            c.g[V0][0] = Addr(st.wrapping_add(0x10a0)).lw(c);
            let retry_clear = c.g[V0][0] == 0;
            c.g[S0][0] = 0x4;
            if retry_clear {
                c.cycles += 2;
                c.g[S0][0] = 0x2;
                c.g[V1][0] = 0x1000_0000;
            } else {
                c.cycles += 2;
                Addr(st.wrapping_add(0x10a0)).sw(c, 0);
            }
        }

        // Slice result flags and the status-register wait.
        c.cycles += 3;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
        c.g[S3][0] = (c.g[S0][0] < 0x1) as u64;
        c.g[V1][0] |= 0xd400;
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
            c.g[V0][0] &= 0x1;
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0040_6178);
            }
        }

        c.cycles += 2;
        let s0_nonzero = c.g[S0][0] != 0;
        c.g[V0][0] = (c.g[S1][0] < 0x2) as u64;
        if !s0_nonzero {
            c.cycles += 5;
            c.g[A1][0] = Addr(st.wrapping_add(0x810)).lw(c);
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = (c.g[A1][0] < 0x1) as u64;
            OC1121_ADD_PRED_AND_STORE.call(c, 0x0040_61ac)?;
            c.g[V0][0] = (c.g[S1][0] < 0x2) as u64;
        }

        // `'b_4061c4`: report a slice that stayed broken.
        c.cycles += 2;
        let broken = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S2][0];
        if broken {
            c.cycles += 3;
            c.g[A1][0] = 0x0049_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7f08));
            OC1121_ERROR.call(c, 0x0040_61c4)?;
        }

        c.cycles += 9;
        c.g[V0][0] = c.g[S3][0];
        oc1121_ld!(c, frame, 0x50, RA);
        oc1121_ld!(c, frame, 0x40, S4);
        oc1121_ld!(c, frame, 0x30, S3);
        oc1121_ld!(c, frame, 0x20, S2);
        oc1121_ld!(c, frame, 0x10, S1);
        oc1121_ld!(c, frame, 0x0, S0);
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_SkippedMacroblock(MpegDec* d, int* pmv, int* motionType,
/// int* fieldSelect, int* mbType)` (0x406510): sets up a skipped macroblock:
/// marks it, zeroes the predictors when the picture is a P field, picks the
/// motion type and field select from the picture structure, and clears the
/// macroblock's intra bit. Returns `ok` (0 after a field error, which is
/// reported through `libmpeg_Error`).
pub fn libmpeg_skipped_macroblock(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 0x140;
        oc1121_sd!(c, frame, 0x0, S0);
        c.g[T1][0] = 0x1;
        oc1121_sd!(c, frame, 0x10, RA);
        c.g[S0][0] = c.g[T0][0];
        c.g[T0][0] = 0x2;
        let st = c.g[A0][0] as u32;
        c.g[V0][0] = Addr(st.wrapping_add(0x810)).lw(c);
        c.mult(0, T2, V0, V1);
        c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(st));
        let one = c.g[T1][0];
        Addr((c.g[V0][0] as u32).wrapping_add(0x6cc)).sw(c, one);
        let one = c.g[T1][0];
        Addr(st.wrapping_add(0x1b0)).sw(c, one);
        c.g[V0][0] = Addr(st.wrapping_add(0x150)).lw(c);
        if c.g[V0][0] != c.g[T0][0] {
            c.g[V1][0] = Addr(st.wrapping_add(0x174)).lw(c);
        } else {
            c.cycles += 5;
            let pmv = c.g[A1][0] as u32;
            Addr(pmv).sw(c, 0);
            Addr(pmv.wrapping_add(0x14)).sw(c, 0);
            Addr(pmv.wrapping_add(0x10)).sw(c, 0);
            Addr(pmv.wrapping_add(0x4)).sw(c, 0);
            c.g[V1][0] = Addr(st.wrapping_add(0x174)).lw(c);
        }

        // `'b_406588`: motion type, then the field select.
        let field_select_done = 'motion: {
            c.cycles += 3;
            c.g[V0][0] = 0x3;
            if c.g[V1][0] != c.g[V0][0] {
                let two = c.g[T1][0];
                Addr(c.g[A2][0] as u32).sw(c, two);
                break 'motion false;
            }
            c.cycles += 2;
            let two = c.g[T0][0];
            Addr(c.g[A2][0] as u32).sw(c, two);
            true
        };
        if !field_select_done {
            c.cycles += 5;
            c.g[V0][0] = Addr(st.wrapping_add(0x174)).lw(c);
            c.g[V0][0] ^= 0x2;
            c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
            let bottom = c.g[V0][0];
            Addr(c.g[A3][0] as u32).sw(c, bottom);
            Addr((c.g[A3][0] as u32).wrapping_add(0x4)).sw(c, bottom);
        }

        // `'b_4065ac`: a field error is reported once.
        c.cycles += 4;
        c.g[V1][0] = Addr(st.wrapping_add(0x150)).lw(c);
        c.g[V0][0] = 0x1;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        } else {
            c.cycles += 5;
            c.g[A1][0] = 0x0049_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7f90));
            OC1121_ERROR.call(c, 0x0040_65a4)?;
            c.g[T1][0] = 0;
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        }

        c.cycles += 8;
        c.g[A0][0] = 0xffff_ffff_ffff_fffe;
        oc1121_ld!(c, frame, 0x10, RA);
        c.g[V0][0] = c.g[T1][0];
        let mask = c.g[A0][0];
        c.g[V1][0] &= mask;
        let flags = c.g[V1][0];
        Addr(c.g[S0][0] as u32).sw(c, flags);
        oc1121_ld!(c, frame, 0x0, S0);
        frame.close(c);
        Ok(())
    })
}

/// `int libmpeg_DecodeMotionVector(int* pred, int r_size, int motion_code,
/// int motion_residual, int full_pel)` (0x406a78): one motion vector
/// component. The predictor is halved for full-pel vectors, moved by the
/// coded difference (wrapped into the range `[-lim, lim)` of the r_size),
/// and stored back doubled for full-pel vectors.
pub fn libmpeg_decode_motion_vector(c: &mut Cpu) {
    c.cycles += 7;
    let pred_ptr = c.g[A0][0];
    c.g[T2][0] = pred_ptr;
    c.g[V0][0] = 0x10;
    c.g[A0][0] = Addr(pred_ptr as u32).lw(c);
    let r = (c.g[A1][0] as u32) & 31;
    c.g[T1][0] = sx(0x10u32 << r);
    let lim = (c.g[T1][0] as u32) as i32;
    let half = sx(((c.g[A0][0] as u32) as i32 >> 1) as u32);
    c.g[V1][0] = half;
    if c.g[T0][0] != 0 {
        c.g[A0][0] = half;
    }
    let vec = (c.g[A0][0] as u32) as i32;
    let mc = c.g[A2][0] as u32 as i32;
    let res = c.g[A3][0] as u32;
    let mut v = vec;
    // The wrap-around paths share one tail charge (`'b_406af0`).
    let mut wrapped = false;
    if (c.g[A2][0] as i64) > 0 {
        // motion_code > 0: add, wrap down from `lim`.
        c.cycles += 8;
        let pre = (vec as u32)
            .wrapping_add(1)
            .wrapping_add((mc as u32).wrapping_sub(1) << r)
            .wrapping_add(res) as i32;
        c.g[V1][0] = (pre < lim) as u64;
        v = pre;
        if pre >= lim {
            c.cycles += 3;
            v = pre.wrapping_sub(lim.wrapping_mul(2));
            wrapped = true;
        }
    } else {
        c.cycles += 2;
        if (c.g[A2][0] as i64) < 0 {
            // motion_code < 0: subtract, wrap up from `-lim`.
            c.cycles += 9;
            let shifted = (!(mc as u32)) << r;
            let pre = (vec as u32)
                .wrapping_sub(1)
                .wrapping_sub(shifted.wrapping_add(res)) as i32;
            let neg_lim = (lim as u32).wrapping_neg() as i32;
            c.g[V1][0] = (pre < neg_lim) as u64;
            v = pre;
            if pre < neg_lim {
                c.cycles += 1;
                v = pre.wrapping_add(lim.wrapping_mul(2));
            }
            wrapped = true;
        }
    }
    if wrapped {
        c.cycles += 1;
    }
    c.cycles += 3;
    c.g[A0][0] = sx(v as u32);
    c.g[V0][0] = sx(v as u32);
    if c.g[T0][0] != 0 {
        c.g[V0][0] = sx((v as u32) << 1);
    }
    let out = c.g[V0][0];
    Addr(pred_ptr as u32).sw(c, out);
    abi::ret(c);
}

/// `void libmpeg_MotionVectors(MpegDec* d, int* x, int* out, int* fieldSelect,
/// int idx, int mvCount, int k, rSizeH, rSizeV, dualPrime, mvScale)`
/// (0x406b00): the motion vectors of one partition. With one vector it reads
/// the field select (unless `k` or `dualPrime` decide it) and decodes one
/// vector; with two it reads both field selects and decodes both.
pub fn libmpeg_motion_vectors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xb0);
        c.g[V0][0] = 0x1;
        oc1121_sd!(c, frame, 0x90, FP);
        oc1121_sd!(c, frame, 0x80, S7);
        oc1121_sd!(c, frame, 0x70, S6);
        c.g[S7][0] = c.g[A2][0];
        oc1121_sd!(c, frame, 0x60, S5);
        oc1121_sd!(c, frame, 0x50, S4);
        c.g[S5][0] = c.g[T3][0];
        oc1121_sd!(c, frame, 0x40, S3);
        c.g[S4][0] = c.g[A1][0];
        oc1121_sd!(c, frame, 0x30, S2);
        oc1121_sd!(c, frame, 0x10, S0);
        c.g[S2][0] = c.g[A0][0];
        oc1121_sd!(c, frame, 0xa0, RA);
        c.g[S0][0] = c.g[T0][0];
        oc1121_sd!(c, frame, 0x20, S1);
        let field = c.g[A3][0];
        Addr(frame.at(c, 0)).sw(c, field);
        c.g[S6][0] = Addr(frame.at(c, 0xb0)).lw(c);
        c.g[S3][0] = Addr(frame.at(c, 0xb8)).lw(c);
        let two_vectors = c.g[T1][0] != c.g[V0][0];
        c.g[FP][0] = Addr(frame.at(c, 0xc0)).lw(c);

        if two_vectors {
            // Two vectors: both field selects, then both vectors (tail call).
            c.cycles += 42;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0x1;
            OC1121_GET_BITS.call(c, 0x0040_6b94)?;
            c.g[S1][0] = sx((c.g[S0][0] as u32) << 3);
            c.g[V1][0] = Addr(frame.at(c, 0)).lw(c);
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[A0][0] = c.g[S2][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A2][0] = c.g[S7][0];
            let bit = c.g[V0][0];
            Addr(c.g[S0][0] as u32).sw(c, bit);
            c.g[A3][0] = c.g[S5][0];
            c.g[T0][0] = c.g[S6][0];
            c.g[T1][0] = c.g[S3][0];
            c.g[T2][0] = c.g[FP][0];
            c.g[T3][0] = 0;
            OC1121_MOTION_VECTOR.call(c, 0x0040_6bcc)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0x1;
            OC1121_GET_BITS.call(c, 0x0040_6bd8)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
            let bit = c.g[V0][0];
            Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, bit);
            c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S7][0];
            c.g[A3][0] = c.g[S5][0];
            c.g[T0][0] = c.g[S6][0];
            c.g[T1][0] = c.g[S3][0];
            c.g[T2][0] = c.g[FP][0];
            oc1121_ld!(c, frame, 0xa0, RA);
            oc1121_ld!(c, frame, 0x90, FP);
            c.g[T3][0] = 0;
            oc1121_ld!(c, frame, 0x80, S7);
            oc1121_ld!(c, frame, 0x70, S6);
            oc1121_ld!(c, frame, 0x60, S5);
            oc1121_ld!(c, frame, 0x50, S4);
            oc1121_ld!(c, frame, 0x40, S3);
            oc1121_ld!(c, frame, 0x30, S2);
            oc1121_ld!(c, frame, 0x20, S1);
            oc1121_ld!(c, frame, 0x10, S0);
            c.g[SP][0] = sx(sp(c).wrapping_add(0xb0));
            c.pc = 0x0040_6ca0;
            (OC1121_MOTION_VECTOR.f)(c);
            return Ok(());
        }

        // One vector: the field select, unless `k` or `dualPrime` decide it.
        c.cycles += 2;
        if c.g[T2][0] != 0 {
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 3);
        } else {
            c.cycles += 2;
            if c.g[S3][0] != 0 {
                c.g[S0][0] = sx((c.g[S0][0] as u32) << 3);
            } else {
                c.cycles += 8;
                c.g[A1][0] = 0x1;
                OC1121_GET_BITS.call(c, 0x0040_6b70)?;
                c.g[A0][0] = Addr(frame.at(c, 0)).lw(c);
                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let bit = c.g[V0][0];
                Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, bit);
                let bit = c.g[V0][0];
                Addr(c.g[V1][0] as u32).sw(c, bit);
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32) << 3);
            }
        }

        // The vector itself, then its copy into the predictor slot (tail).
        c.cycles += 26;
        c.g[A0][0] = c.g[S2][0];
        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[A2][0] = c.g[S7][0];
        c.g[A3][0] = c.g[S5][0];
        c.g[T0][0] = c.g[S6][0];
        c.g[T1][0] = c.g[S3][0];
        c.g[T2][0] = c.g[FP][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[T3][0] = 0;
        OC1121_MOTION_VECTOR.call(c, 0x0040_6c5c)?;
        let vector = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(vector).lw(c);
        c.g[V1][0] = Addr(vector.wrapping_add(0x4)).lw(c);
        let word = c.g[V0][0];
        Addr(vector.wrapping_add(0x10)).sw(c, word);
        let word = c.g[V1][0];
        Addr(vector.wrapping_add(0x14)).sw(c, word);
        oc1121_ld!(c, frame, 0xa0, RA);
        oc1121_ld!(c, frame, 0x90, FP);
        oc1121_ld!(c, frame, 0x80, S7);
        oc1121_ld!(c, frame, 0x70, S6);
        oc1121_ld!(c, frame, 0x60, S5);
        oc1121_ld!(c, frame, 0x50, S4);
        oc1121_ld!(c, frame, 0x40, S3);
        oc1121_ld!(c, frame, 0x30, S2);
        oc1121_ld!(c, frame, 0x20, S1);
        oc1121_ld!(c, frame, 0x10, S0);
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_MotionVector(MpegDec* d, int* pred, int* dmv, int r_size1,
/// int r_size2, int has_dmv, int full_pel_flag, int full_pel)` (0x406ca0):
/// decodes both motion vector components of one vector (the second one with
/// its own motion code and residual), halves or doubles the predictor for
/// field pictures, and reads the dual-prime vector when present.
pub fn libmpeg_motion_vector(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0xa0);
        oc1121_sd!(c, frame, 0x30, S3);
        oc1121_sd!(c, frame, 0x20, S2);
        c.g[S3][0] = c.g[A0][0];
        oc1121_sd!(c, frame, 0x80, FP);
        c.g[S2][0] = c.g[A1][0];
        oc1121_sd!(c, frame, 0x70, S7);
        c.g[FP][0] = c.g[A2][0];
        oc1121_sd!(c, frame, 0x60, S6);
        c.g[S7][0] = c.g[T1][0];
        oc1121_sd!(c, frame, 0x50, S5);
        c.g[S6][0] = c.g[T2][0];
        oc1121_sd!(c, frame, 0x40, S4);
        c.g[S5][0] = c.g[T3][0];
        oc1121_sd!(c, frame, 0x10, S1);
        c.g[S4][0] = c.g[T0][0];
        oc1121_sd!(c, frame, 0x0, S0);
        c.g[S1][0] = c.g[A3][0];
        oc1121_sd!(c, frame, 0x90, RA);

        // The motion code of the first component.
        c.g[RA][0] = sx(0x0040_6cf4);
        c.g[A1][0] = 0x2;
        OC1121_IPU_VDEC.call(c, 0x0040_6cf4)?;
        c.g[S0][0] = c.g[V0][0];

        // The residual of the first component (`r_size` bits).
        let mut have_residual = false;
        if c.g[S1][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = c.g[S3][0];
            if c.g[S0][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = c.g[S1][0];
                OC1121_GET_BITS.call(c, 0x0040_6d0c)?;
                c.g[A3][0] = c.g[V0][0];
                have_residual = true;
            }
        }
        if !have_residual {
            c.cycles += 1;
            c.g[A3][0] = 0;
        }

        // `'b_406d44`: the first component, then the dual-prime vector.
        c.cycles += 7;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[T0][0] = c.g[S5][0];
        OC1121_DECODE_MOTION_VECTOR.call(c, 0x0040_6d2c)?;
        c.g[A0][0] = c.g[S3][0];
        if c.g[S7][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S3][0];
            OC1121_DECODE_DM_VECTOR.call(c, 0x0040_6d3c)?;
            let dmv = c.g[V0][0];
            Addr(c.g[FP][0] as u32).sw(c, dmv);
            c.g[A0][0] = c.g[S3][0];
        }

        // `'b_406d70`: the second component's motion code and residual.
        c.cycles += 4;
        c.g[RA][0] = sx(0x0040_6d4c);
        c.g[A1][0] = 0x2;
        OC1121_IPU_VDEC.call(c, 0x0040_6d4c)?;
        c.g[S0][0] = c.g[V0][0];
        let mut have_second_residual = false;
        if c.g[S4][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = c.g[S3][0];
            if c.g[S0][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = c.g[S4][0];
                OC1121_GET_BITS.call(c, 0x0040_6d64)?;
                c.g[A3][0] = c.g[V0][0];
                have_second_residual = true;
            }
        }
        if !have_second_residual {
            c.cycles += 1;
            c.g[A3][0] = 0;
        }

        // `'b_406d84`: the predictor of the field picture is halved.
        c.cycles += 2;
        let half_pel = c.g[S6][0] == 0;
        c.g[A1][0] = c.g[S4][0];
        if !half_pel {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x4)).lw(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
            let word = c.g[V0][0];
            Addr((c.g[S2][0] as u32).wrapping_add(0x4)).sw(c, word);
        }

        // `'b_406da8`: the second component, then the field doubling.
        c.cycles += 6;
        c.g[A2][0] = c.g[S0][0];
        c.g[T0][0] = c.g[S5][0];
        c.g[RA][0] = sx(0x0040_6d94);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
        OC1121_DECODE_MOTION_VECTOR.call(c, 0x0040_6d94)?;
        if c.g[S6][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x4)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
            let word = c.g[V0][0];
            Addr((c.g[S2][0] as u32).wrapping_add(0x4)).sw(c, word);
        }

        // `'b_406dc0`: the dual-prime vector of the second component.
        c.cycles += 2;
        let no_dmv = c.g[S7][0] == 0;
        oc1121_ld!(c, frame, 0x90, RA);
        if !no_dmv {
            c.cycles += 4;
            c.g[A0][0] = c.g[S3][0];
            OC1121_DECODE_DM_VECTOR.call(c, 0x0040_6db8)?;
            let dmv = c.g[V0][0];
            Addr((c.g[FP][0] as u32).wrapping_add(0x4)).sw(c, dmv);
            oc1121_ld!(c, frame, 0x90, RA);
        }

        c.cycles += 11;
        oc1121_ld!(c, frame, 0x80, FP);
        oc1121_ld!(c, frame, 0x70, S7);
        oc1121_ld!(c, frame, 0x60, S6);
        oc1121_ld!(c, frame, 0x50, S5);
        oc1121_ld!(c, frame, 0x40, S4);
        oc1121_ld!(c, frame, 0x30, S3);
        oc1121_ld!(c, frame, 0x20, S2);
        oc1121_ld!(c, frame, 0x10, S1);
        oc1121_ld!(c, frame, 0x0, S0);
        frame.close(c);
        Ok(())
    })
}

/// `int libmpeg_IpuWaitCmd(MpegDec* d)` (0x406ec8): waits until the IPU
/// command register has a command (its top bit clear) or reports an error
/// (bit 14 of the control register). Stalled waits call the host callback
/// every 5001 polls. Returns the command word.
pub fn libmpeg_ipu_wait_cmd(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x1000_0000;
        oc1121_sd!(c, frame, 0x20, S2);
        c.g[V0][0] |= 0x2000;
        oc1121_sd!(c, frame, 0x30, RA);
        c.g[S2][0] = c.g[A0][0];
        oc1121_sd!(c, frame, 0x10, S1);
        c.g[A1][0] = 0;
        oc1121_sd!(c, frame, 0x0, S0);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).ld(c);
        let ready = (c.g[A0][0] as i64) >= 0;
        oc1121_ld!(c, frame, 0x30, RA);
        // Whether the tail (`V0 = cmd`, one cycle) runs: not on the error exit.
        let mut tail = true;
        if !ready {
            c.cycles += 6;
            c.g[V0][0] = 0x1000_2010;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V1][0] &= 0x4000;
            let error_flag = c.g[V1][0] != 0;
            c.g[V0][0] = c.g[A0][0];
            if error_flag {
                tail = false;
            } else {
                c.cycles += 6;
                c.g[S1][0] = 0x1000_2000;
                c.g[S0][0] = 0x1000_2010;
                c.g[V0][0] = c.g[A1][0];
                loop {
                    // `'b_406f40`: count the poll, call back every 5001 polls.
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 5001) as u64;
                    let below = c.g[V0][0] != 0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                    if !below {
                        c.cycles += 3;
                        c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x858)).lw(c);
                        OC1121_CALLBACK_NODATA.call(c, 0x0040_6f3c)?;
                        c.g[A1][0] = 0;
                    }
                    c.cycles += 3;
                    c.g[A0][0] = Addr(c.g[S1][0] as u32).ld(c);
                    let cmd_ready = (c.g[A0][0] as i64) >= 0;
                    oc1121_ld!(c, frame, 0x30, RA);
                    if cmd_ready {
                        break;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[V0][0] &= 0x4000;
                    let idle = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[A1][0];
                    if idle {
                        if event_due(c) {
                            deliver_event_here(c, 0x0040_6f28);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    oc1121_ld!(c, frame, 0x30, RA);
                    break;
                }
            }
        }
        if tail {
            c.cycles += 1;
            c.g[V0][0] = c.g[A0][0];
        }
        c.cycles += 5;
        oc1121_ld!(c, frame, 0x20, S2);
        oc1121_ld!(c, frame, 0x10, S1);
        oc1121_ld!(c, frame, 0x0, S0);
        frame.close(c);
        Ok(())
    })
}

/// `int libmpeg_SliceHeaderTail(MpegDec* d)` (0x4074b8): reads the last
/// fields of a slice header (5 bits into `d+0x1b4`, then one flag bit); when
/// the flag is set it skips 1 bit, flushes 7 bits and reads the extra
/// information. Returns 0.
pub fn libmpeg_slice_header_tail(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x5;
        oc1121_sd!(c, frame, 0x0, S0);
        oc1121_sd!(c, frame, 0x10, RA);
        c.g[S0][0] = c.g[A0][0];
        OC1121_GET_BITS.call(c, 0x0040_74d0)?;
        let bits = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x1b4)).sw(c, bits);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x1;
        OC1121_GET_BITS.call(c, 0x0040_74e0)?;
        let flag_clear = c.g[V0][0] == 0;
        oc1121_ld!(c, frame, 0x10, RA);
        if !flag_clear {
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x1;
            OC1121_GET_BITS.call(c, 0x0040_74f4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x7;
            OC1121_FLUSH_BITS.call(c, 0x0040_7500)?;
            c.g[A0][0] = c.g[S0][0];
            OC1121_EXTRA_BIT_INFO.call(c, 0x0040_7508)?;
            oc1121_ld!(c, frame, 0x10, RA);
        }
        c.cycles += 4;
        c.g[V0][0] = 0;
        oc1121_ld!(c, frame, 0x0, S0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of this module's callee fakes: they answer only while this word
    /// holds `OC1121_MAGIC` (otherwise the body they replaced runs).
    const OC1121_CTRL: u32 = DATA + 0x000f_0000;
    const OC1121_MAGIC: u32 = 0x3131_4c43;
    /// Results the fakes return, one word per logged fake call.
    const OC1121_MBOX: u32 = DATA + 0x000f_1000;

    static OC1121_PREV_DECODE_SLICE: AtomicUsize = AtomicUsize::new(0);
    static OC1121_PREV_IPU_WAIT_BUSY: AtomicUsize = AtomicUsize::new(0);
    static OC1121_PREV_WAIT_BDEC: AtomicUsize = AtomicUsize::new(0);
    static OC1121_PREV_ADD_PRED: AtomicUsize = AtomicUsize::new(0);
    static OC1121_PREV_ERROR: AtomicUsize = AtomicUsize::new(0);
    static OC1121_PREV_GET_BITS: AtomicUsize = AtomicUsize::new(0);
    static OC1121_PREV_FLUSH_BITS: AtomicUsize = AtomicUsize::new(0);
    static OC1121_PREV_IPU_VDEC: AtomicUsize = AtomicUsize::new(0);
    static OC1121_PREV_DECODE_DM: AtomicUsize = AtomicUsize::new(0);
    static OC1121_PREV_MOTION_VECTOR: AtomicUsize = AtomicUsize::new(0);
    static OC1121_PREV_DECODE_MV: AtomicUsize = AtomicUsize::new(0);
    static OC1121_PREV_CALLBACK: AtomicUsize = AtomicUsize::new(0);
    static OC1121_PREV_EXTRA_BIT: AtomicUsize = AtomicUsize::new(0);

    /// Log the call, return the next mailbox word in v0.
    fn oc1121_answer(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let result = c.bus.read32(OC1121_MBOX + 4 * (n % 64));
        fakes::fake_call(c, target);
        c.g[V0][0] = result as u64;
    }

    /// A gated callee fake: answers from the mailbox while the gate is on,
    /// else runs the body it replaced.
    macro_rules! oc1121_fake {
        ($fake:ident, $prev:ident, $target:expr, $body:path) => {
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC1121_CTRL) != OC1121_MAGIC {
                    return fakes::fall_back(c, &$prev, $body);
                }
                oc1121_answer(c, $target);
            }
        };
    }

    oc1121_fake!(
        oc1121_fake_decode_slice,
        OC1121_PREV_DECODE_SLICE,
        0x0040_6318,
        ee::s15::f_00406318__generated
    );
    oc1121_fake!(
        oc1121_fake_ipu_wait_busy,
        OC1121_PREV_IPU_WAIT_BUSY,
        0x0040_6e20,
        ee::s15::f_00406e20__generated
    );
    oc1121_fake!(
        oc1121_fake_wait_bdec,
        OC1121_PREV_WAIT_BDEC,
        0x0040_5b70,
        ee::s15::f_00405b70__generated
    );
    oc1121_fake!(
        oc1121_fake_add_pred,
        OC1121_PREV_ADD_PRED,
        0x0040_4b78,
        ee::s15::f_00404b78__generated
    );
    oc1121_fake!(
        oc1121_fake_error,
        OC1121_PREV_ERROR,
        0x0040_34a8,
        ee::s15::f_004034a8__generated
    );
    oc1121_fake!(
        oc1121_fake_get_bits,
        OC1121_PREV_GET_BITS,
        0x0040_72e8,
        ee::s15::f_004072e8__generated
    );
    oc1121_fake!(
        oc1121_fake_flush_bits,
        OC1121_PREV_FLUSH_BITS,
        0x0040_71f8,
        ee::s15::f_004071f8__generated
    );
    oc1121_fake!(
        oc1121_fake_ipu_vdec,
        OC1121_PREV_IPU_VDEC,
        0x0040_6f78,
        ee::s15::f_00406f78__generated
    );
    oc1121_fake!(
        oc1121_fake_decode_dm,
        OC1121_PREV_DECODE_DM,
        0x0040_5e10,
        ee::s15::f_00405e10__generated
    );
    oc1121_fake!(
        oc1121_fake_motion_vector,
        OC1121_PREV_MOTION_VECTOR,
        0x0040_6ca0,
        ee::s15::f_00406ca0__generated
    );
    oc1121_fake!(
        oc1121_fake_decode_mv,
        OC1121_PREV_DECODE_MV,
        0x0040_6a78,
        ee::s15::f_00406a78__generated
    );
    oc1121_fake!(
        oc1121_fake_extra_bit,
        OC1121_PREV_EXTRA_BIT,
        0x0040_7a10,
        ee::s15::f_00407a10__generated
    );

    /// The host callback: logs the call and makes the IPU command ready, so
    /// the poll loop ends.
    fn oc1121_fake_callback(c: &mut Cpu) {
        if c.bus.read32(OC1121_CTRL) != OC1121_MAGIC {
            return fakes::fall_back(c, &OC1121_PREV_CALLBACK, ee::s15::f_00402c80__generated);
        }
        fakes::fake_call(c, 0x0040_2c80);
        c.bus.write64(0x1000_2000, 0x5);
    }

    /// Hook this module's fakes into their override slots (once per process).
    fn oc1121_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00406318),
            oc1121_fake_decode_slice,
            &OC1121_PREV_DECODE_SLICE,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00406E20),
            oc1121_fake_ipu_wait_busy,
            &OC1121_PREV_IPU_WAIT_BUSY,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00405B70),
            oc1121_fake_wait_bdec,
            &OC1121_PREV_WAIT_BDEC,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00404B78),
            oc1121_fake_add_pred,
            &OC1121_PREV_ADD_PRED,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004034A8),
            oc1121_fake_error,
            &OC1121_PREV_ERROR,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004072E8),
            oc1121_fake_get_bits,
            &OC1121_PREV_GET_BITS,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004071F8),
            oc1121_fake_flush_bits,
            &OC1121_PREV_FLUSH_BITS,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00406F78),
            oc1121_fake_ipu_vdec,
            &OC1121_PREV_IPU_VDEC,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00405E10),
            oc1121_fake_decode_dm,
            &OC1121_PREV_DECODE_DM,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00406CA0),
            oc1121_fake_motion_vector,
            &OC1121_PREV_MOTION_VECTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00406A78),
            oc1121_fake_decode_mv,
            &OC1121_PREV_DECODE_MV,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00402C80),
            oc1121_fake_callback,
            &OC1121_PREV_CALLBACK,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00407A10),
            oc1121_fake_extra_bit,
            &OC1121_PREV_EXTRA_BIT,
        );
    }

    /// Gate this module's fakes on in the test machine and fill the mailbox.
    fn oc1121_arm(c: &mut Cpu, results: &[u32]) {
        c.bus.write32(OC1121_CTRL, OC1121_MAGIC);
        for (k, v) in results.iter().enumerate() {
            c.bus.write32(OC1121_MBOX + 4 * k as u32, *v);
        }
    }

    #[test]
    fn libmpeg_decode_slices_matches() {
        oc1121_install_fakes();
        check(
            ee::s15::f_004060c8__generated,
            libmpeg_decode_slices,
            300,
            &|c, seed| {
                let mut r = Rng(0x4060_c800 ^ seed as u64);
                scramble(c, &mut r);
                // Slice results: re-runs (1, 3), a clean one (0), errors (2, 4).
                let mut results = [0u32; 8];
                for v in results.iter_mut() {
                    *v = [0, 1, 2, 3, 4][(r.next_u32() % 5) as usize];
                }
                oc1121_arm(c, &results);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x12c, 2 + r.next_u32() % 6);
                c.bus.write32(DATA + 0x130, 1 + r.next_u32() % 4);
                c.bus.write32(DATA + 0x174, [3, 2, 1][seed as usize % 3]);
                c.bus.write32(DATA + 0x10a0, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(0x1000_d400, 0);
            },
        );
    }

    #[test]
    fn libmpeg_skipped_macroblock_matches() {
        oc1121_install_fakes();
        check(
            ee::s15::f_00406510__generated,
            libmpeg_skipped_macroblock,
            300,
            &|c, seed| {
                let mut r = Rng(0x4065_1000 ^ seed as u64);
                scramble(c, &mut r);
                oc1121_arm(c, &[]);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2_0000);
                c.g[A2][0] = sx(DATA + 0x2_0100);
                c.g[A3][0] = sx(DATA + 0x2_0200);
                c.g[T0][0] = sx(DATA + 0x2_0300);
                c.bus.write32(DATA + 0x810, r.next_u32() % 4);
                c.bus.write32(DATA + 0x150, [0, 1, 2, 3][seed as usize % 4]);
                c.bus.write32(DATA + 0x174, [1, 2, 3][(seed / 4) as usize % 3]);
                c.bus.write32(DATA + 0x2_0300, r.next_u32());
            },
        );
    }

    #[test]
    fn libmpeg_decode_motion_vector_matches() {
        check(
            ee::s15::f_00406a78__generated,
            libmpeg_decode_motion_vector,
            300,
            &|c, seed| {
                let mut r = Rng(0x6a78_0001 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let mc = (r.next_u32() % 33) as i32 - 16;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(r.next_u32() % 4);
                c.g[A2][0] = sx(mc as u32);
                c.g[A3][0] = sx(r.next_u32() % 9);
                c.g[T0][0] = (seed % 2) as u64;
                c.bus.write32(DATA, ((r.next_u32() % 400) as i32 - 200) as u32);
            },
        );
    }

    #[test]
    fn libmpeg_motion_vectors_matches() {
        oc1121_install_fakes();
        check(
            ee::s15::f_00406b00__generated,
            libmpeg_motion_vectors,
            300,
            &|c, seed| {
                let mut r = Rng(0x6b00_0000 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let results: Vec<u32> = (0..16).map(|_| r.next_u32() % 3).collect();
                oc1121_arm(c, &results);
                let stack = crate::difftest::SP;
                c.g[SP][0] = sx(stack);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x3_0000);
                c.g[A2][0] = sx(DATA + 0x3_1000);
                c.g[A3][0] = sx(DATA + 0x3_2000);
                c.g[T0][0] = (seed % 3) as u64;
                c.g[T1][0] = 1 + (seed % 2) as u64;
                c.g[T2][0] = ((seed / 2) % 2) as u64;
                c.g[T3][0] = (r.next_u32() % 4) as u64;
                c.bus.write32(stack + 0xb0, r.next_u32() % 4);
                c.bus.write32(stack + 0xb8, (seed / 4) % 2);
                c.bus.write32(stack + 0xc0, r.next_u32() % 4);
            },
        );
    }

    #[test]
    fn libmpeg_motion_vector_matches() {
        oc1121_install_fakes();
        check(
            ee::s15::f_00406ca0__generated,
            libmpeg_motion_vector,
            300,
            &|c, seed| {
                let mut r = Rng(0x6ca0_0000 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let results: Vec<u32> = (0..16).map(|_| r.next_u32() % 5).collect();
                oc1121_arm(c, &results);
                c.g[SP][0] = sx(crate::difftest::SP);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2_0000);
                c.g[A2][0] = sx(DATA + 0x2_1000);
                c.g[A3][0] = sx(r.next_u32() % 3);
                c.g[T0][0] = (r.next_u32() % 3) as u64;
                c.g[T1][0] = (seed % 2) as u64;
                c.g[T2][0] = ((seed / 2) % 2) as u64;
                c.g[T3][0] = ((seed / 4) % 2) as u64;
                c.bus.write32(DATA + 0x2_0004, r.next_u32() % 64);
            },
        );
    }

    #[test]
    fn libmpeg_ipu_wait_cmd_matches() {
        oc1121_install_fakes();
        check(
            ee::s15::f_00406ec8__generated,
            libmpeg_ipu_wait_cmd,
            200,
            &|c, seed| {
                let mut r = Rng(0x6ec8_0000 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                oc1121_arm(c, &[]);
                c.g[A0][0] = sx(DATA);
                // Ready (seed % 3 == 0), error flag set (1), or polled to the callback (2).
                let cmd = if seed % 3 == 0 {
                    (r.next_u32() & 0x7fff_ffff) as u64
                } else {
                    (r.next_u32() as u64) | 0x8000_0000_0000_0000
                };
                c.bus.write64(0x1000_2000, cmd);
                c.bus.write32(0x1000_2010, if seed % 3 == 1 { 0x4000 } else { 0 });
            },
        );
    }

    #[test]
    fn libmpeg_slice_header_tail_matches() {
        oc1121_install_fakes();
        check(
            ee::s15::f_004074b8__generated,
            libmpeg_slice_header_tail,
            200,
            &|c, seed| {
                let mut r = Rng(0x74b8_0000 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                // First bits (any), the flag (zero or not), then the tail's results.
                let mut results = vec![r.next_u32(), seed % 2];
                results.extend((0..6).map(|_| r.next_u32() % 3));
                oc1121_arm(c, &results);
                c.g[A0][0] = sx(DATA);
            },
        );
    }
}
