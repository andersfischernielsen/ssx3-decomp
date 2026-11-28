//! Staging module oc1592 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1592`.

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
    (
        0x0040_7dc0,
        libmpeg_decode_picture_slices,
        "libmpeg_DecodePictureSlices",
    ),
    (0x0040_7e90, libmpeg_output_picture, "libmpeg_OutputPicture"),
    (0x0040_86c0, libmpeg_output_frame, "libmpeg_OutputFrame"),
    (
        0x0040_87d0,
        libmpeg_output_field_pair,
        "libmpeg_OutputFieldPair",
    ),
    (
        0x0040_8ca8,
        libmpeg_ipu_from_dma,
        "libmpeg_IpuFromDmaHandler",
    ),
    (0x0040_8fb0, libmpeg_ipu_to_dma, "libmpeg_IpuToDmaHandler"),
    (0x0040_a320, libmc_delay_alarm, "libmc_DelayAlarmHandler"),
];

// Direct-call targets of the decode and output chain (readable aliases).
pub(crate) const DEC_CALL_ERROR: Callee = Callee {
    addr: 0x0040_34a8,
    f: ee::s15::libmpeg_Error,
};

pub(crate) const DEC_CALL_SLICES: Callee = Callee {
    addr: 0x0040_60c8,
    f: ee::s15::libmpeg_DecodeSlices,
};

pub(crate) const OUT_CALL_FRAME: Callee = Callee {
    addr: 0x0040_86c0,
    f: ee::s15::libmpeg_OutputFrame,
};

pub(crate) const OUT_CALL_FIELD_PAIR: Callee = Callee {
    addr: 0x0040_87d0,
    f: ee::s15::libmpeg_OutputFieldPair,
};

pub(crate) const OUT_CALL_COMPUTE_PTS: Callee = Callee {
    addr: 0x0040_8530,
    f: ee::s15::libmpeg_ComputePts,
};

pub(crate) const OUT_CALL_CHECK_BUFFER: Callee = Callee {
    addr: 0x0040_81d8,
    f: ee::s15::libmpeg_CheckOutputBuffer,
};

pub(crate) const OUT_CALL_CONVERT_CSC: Callee = Callee {
    addr: 0x0040_9088,
    f: ee::s15::libmpeg_ConvertPictureCsc,
};

pub(crate) const OUT_CALL_COPY_RAW: Callee = Callee {
    addr: 0x0040_8278,
    f: ee::s15::libmpeg_CopyOutputRaw,
};

pub(crate) const OUT_CALL_SET_DONE: Callee = Callee {
    addr: 0x0040_8508,
    f: ee::s15::libmpeg_SetOutputDone,
};

pub(crate) const ALARM_CALL_SIGNAL_SEMA: Callee = Callee {
    addr: 0x0042_3dd0,
    f: ee::s15::iSignalSema,
};

/// The record pointer `decState[0x120/4]` error tail: `libmpeg_Error(decState,
/// D_00498088)` (0x407e5c, the default picture record of OutputPicture's
/// decode chain). `a1` is the 0x4a0000 pattern set by the caller.
fn dec_error_tail(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7f78));
    c.g[A0][0] = c.g[S0][0];
    DEC_CALL_ERROR.call(c, 0x0040_7e5c)
}

/// `int libmpeg_DecodePictureSlices(decState)` (0x407dc0): picks the picture
/// record for the decoder state (3: +0x1c0, 1: +0x1d0, 2: +0x1e0; any other
/// state reports `D_00498088` and uses +0x1c0), decodes the slices and sets the
/// record's +0x28 flag when something was decoded. In state 3 with a pending
/// error (+0x120) it reports `D_00498068` and clears the error. Returns the
/// decode result.
pub fn libmpeg_decode_picture_slices(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x3;
        let a = frame.at(c, 0);
        let v = c.g[S0][0];
        c.sd(a, v);
        let a = frame.at(c, 0x20);
        let v = c.g[RA][0];
        c.sd(a, v);
        c.g[S0][0] = c.g[A0][0];
        let a = frame.at(c, 0x10);
        let v = c.g[S1][0];
        c.sd(a, v);
        let dec = c.g[S0][0] as u32;
        c.g[V1][0] = Addr(dec + 0x174).lw(c);
        let not_three = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = 0x2;
        if !not_three {
            c.cycles += 3;
            c.g[V0][0] = Addr(dec + 0x120).lw(c);
            let no_error = c.g[V0][0] == 0;
            c.g[V0][0] = 0x2;
            if !no_error {
                c.cycles += 6;
                c.g[A1][0] = 0x0049_8068;
                DEC_CALL_ERROR.call(c, 0x0040_7dfc)?;
                c.sw(dec + 0x120, 0);
                c.g[V1][0] = Addr(dec + 0x174).lw(c);
                c.g[V0][0] = 0x2;
            }
        }
        // The picture record for the state.
        c.cycles += 2;
        let state = c.g[V1][0];
        let is_two = state == 2;
        c.g[V0][0] = ((state as i64) < 3) as u64;
        if is_two {
            c.cycles += 2;
            c.g[S1][0] = Addr(dec + 0x1e0).lw(c);
        } else {
            c.cycles += 2;
            let below_three = c.g[V0][0] != 0;
            c.g[V0][0] = 0x1;
            if !below_three {
                // state >= 3
                c.cycles += 3;
                c.g[V0][0] = 0x3;
                c.g[A1][0] = 0x004a_0000;
                if state == 3 {
                    c.cycles += 2;
                    c.g[S1][0] = Addr(dec + 0x1c0).lw(c);
                } else {
                    c.cycles += 1;
                    c.g[S1][0] = Addr(dec + 0x1c0).lw(c);
                    dec_error_tail(c)?;
                }
            } else {
                // state 1, or a state that is not 2 and below 3
                c.cycles += 2;
                let is_one = state == c.g[V0][0];
                c.g[A1][0] = 0x004a_0000;
                if is_one {
                    c.cycles += 2;
                    c.g[S1][0] = Addr(dec + 0x1d0).lw(c);
                } else {
                    c.cycles += 2;
                    c.g[S1][0] = Addr(dec + 0x1c0).lw(c);
                    dec_error_tail(c)?;
                }
            }
        }
        // Decode the slices.
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        DEC_CALL_SLICES.call(c, 0x0040_7e64)?;
        c.g[V1][0] = c.g[V0][0];
        let decoded = c.g[V1][0] != 0;
        c.g[V0][0] = 0x1;
        if decoded {
            c.cycles += 1;
            let rec = c.g[S1][0] as u32;
            c.sw(rec + 0x28, 1);
        }
        c.cycles += 6;
        frame.restore_ra(c, 0x20);
        c.g[V0][0] = c.g[V1][0];
        let a = frame.at(c, 0x10);
        c.g[S1][0] = c.ld(a);
        let a = frame.at(c, 0);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    })
}

/// The tail of OutputPicture (0x407f14): stores 2 into the flag at +0xf8 when
/// it was 1, then restores `s0`, `ra` and the frame.
fn output_picture_tail(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    c.g[V0][0] = 1;
    let not_one = c.g[V1][0] != c.g[V0][0];
    let a = frame.at(c, 0x10);
    c.g[RA][0] = c.ld(a);
    if !not_one {
        c.cycles += 2;
        c.g[V0][0] = 2;
        let dec = c.g[S0][0] as u32;
        c.sw(dec + 0xf8, 2);
    }
    c.cycles += 3;
    let a = frame.at(c, 0);
    c.g[S0][0] = c.ld(a);
    frame.close(c);
}

/// `void libmpeg_OutputPicture(decState, picRecord, flag)` (0x407e90): when
/// `flag` is set, outputs the picture (one frame if the state is 3, else the
/// field pair of the two records), then the flag at +0xf8 moves from 1 to 2.
pub fn libmpeg_output_picture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A1][0];
        let a = frame.at(c, 0);
        let v = c.g[S0][0];
        c.sd(a, v);
        let a = frame.at(c, 0x10);
        let v = c.g[RA][0];
        c.sd(a, v);
        let flag = c.g[A2][0];
        c.g[S0][0] = c.g[A0][0];
        let dec = c.g[S0][0] as u32;
        // The shared 0x407efc tail (+1, flag reload) runs unless a frame was output.
        let mut shared_tail = true;
        if flag != 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(dec + 0x174).lw(c);
            c.g[V1][0] = 0x3;
            let not_three = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = Addr(dec + 0x150).lw(c);
            if !not_three {
                c.cycles += 2;
                if c.g[V0][0] != c.g[V1][0] {
                    c.g[A1][0] = Addr(dec + 0x1b8).lw(c);
                } else {
                    c.cycles += 1;
                    c.g[A1][0] = Addr(dec + 0x1c4).lw(c);
                }
                c.cycles += 5;
                c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
                c.g[A0][0] = c.g[S0][0];
                OUT_CALL_FRAME.call(c, 0x0040_7ed0)?;
                c.g[V1][0] = Addr(dec + 0xf8).lw(c);
                shared_tail = false;
            } else {
                c.cycles += 2;
                if c.g[V0][0] != c.g[V1][0] {
                    c.g[A1][0] = Addr(dec + 0x1c8).lw(c);
                    c.cycles += 1;
                    c.g[A2][0] = Addr(dec + 0x1d8).lw(c);
                } else {
                    c.cycles += 3;
                    c.g[A1][0] = Addr(dec + 0x1d4).lw(c);
                    c.g[A2][0] = Addr(dec + 0x1e4).lw(c);
                }
                c.cycles += 3;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
                c.g[A0][0] = c.g[S0][0];
                OUT_CALL_FIELD_PAIR.call(c, 0x0040_7efc)?;
            }
        }
        if shared_tail {
            c.cycles += 1;
            c.g[V1][0] = Addr(dec + 0xf8).lw(c);
        }
        output_picture_tail(c, frame);
        Ok(())
    })
}

/// `void libmpeg_OutputFrame(decState, picRecord, n)` (0x4086c0): computes the
/// pts of the picture, copies its header fields into the decoder state, checks
/// the output buffer and, when that is free and the record is ready (+0x28 == 1),
/// converts (or copies) the picture and reports it done.
pub fn libmpeg_output_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 45;
        let frame = Frame::open(c, 0x30);
        let a = frame.at(c, 0x10);
        let v = c.g[S1][0];
        c.sd(a, v);
        let a = frame.at(c, 0);
        let v = c.g[S0][0];
        c.sd(a, v);
        c.g[S1][0] = c.g[A1][0];
        let a = frame.at(c, 0x20);
        let v = c.g[RA][0];
        c.sd(a, v);
        c.g[S0][0] = c.g[A0][0];
        let dec = c.g[S0][0] as u32;
        let hv = Addr(dec + 0x858).lw(c);
        c.g[A3][0] = hv;
        let h = hv as u32;
        c.g[T0][0] = sx(h.wrapping_add(0x20));
        c.g[A2][0] = sx(h.wrapping_add(0x10));
        c.g[A3][0] = sx(h.wrapping_add(0x18));
        OUT_CALL_COMPUTE_PTS.call(c, 0x0040_86ec)?;
        let hv = Addr(dec + 0x858).lw(c);
        c.g[A3][0] = hv;
        let h = hv as u32;
        c.g[A2][0] = 0x0045_32e8;
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = c.ld(h.wrapping_add(0x20));
        c.g[A1][0] = c.g[S1][0];
        c.g[V1][0] = Addr(h.wrapping_add(0x10)).lw(c);
        c.g[V0][0] = ((c.g[V0][0] << 27) as i64 >> 32) as u64;
        let v1 = c.g[V1][0];
        c.sw(dec + 0x80, v1);
        c.g[V0][0] &= 0xf;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        let rec = c.g[S1][0] as u32;
        c.g[V1][0] = Addr(rec + 0x5c).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[A2][0] = Addr(c.g[V0][0] as u32).lwu(c);
        let v1 = c.g[V1][0];
        c.sw(dec + 0xcc, v1);
        let v2 = c.g[A2][0];
        c.sd(dec + 0x88, v2);
        c.g[V0][0] = Addr(rec + 0x60).lw(c);
        let v0 = c.g[V0][0];
        c.sw(dec + 0xd0, v0);
        c.g[V1][0] = Addr(rec + 0x44).lw(c);
        let v1 = c.g[V1][0];
        c.sw(dec + 0xb4, v1);
        c.g[V0][0] = Addr(rec + 0x48).lw(c);
        let v0 = c.g[V0][0];
        c.sw(dec + 0xb8, v0);
        c.g[V1][0] = Addr(rec + 0x4c).lw(c);
        let v1 = c.g[V1][0];
        c.sw(dec + 0xbc, v1);
        c.g[V0][0] = Addr(rec + 0x50).lw(c);
        let v0 = c.g[V0][0];
        c.sw(dec + 0xc0, v0);
        c.g[V1][0] = Addr(rec + 0x54).lw(c);
        let v1 = c.g[V1][0];
        c.sw(dec + 0xc4, v1);
        c.g[V0][0] = Addr(rec + 0x58).lw(c);
        let v0 = c.g[V0][0];
        c.sw(dec + 0xc8, v0);
        OUT_CALL_CHECK_BUFFER.call(c, 0x0040_876c)?;
        let rec = c.g[S1][0] as u32;
        let none = c.g[V0][0] == 0;
        c.g[V0][0] = 0x1;
        if none {
            c.cycles += 1;
            frame.restore_ra(c, 0x20);
        } else {
            c.cycles += 3;
            c.g[V1][0] = Addr(rec + 0x28).lw(c);
            let not_ready = c.g[V1][0] != c.g[V0][0];
            frame.restore_ra(c, 0x20);
            if !not_ready {
                c.cycles += 3;
                c.g[V0][0] = Addr(dec + 0xb0).lw(c);
                let no_csc = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[S1][0];
                if no_csc {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    OUT_CALL_COPY_RAW.call(c, 0x0040_87a4)?;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    OUT_CALL_CONVERT_CSC.call(c, 0x0040_8794)?;
                }
                c.g[A0][0] = c.g[S0][0];
                c.cycles += 5;
                frame.restore_ra(c, 0x20);
                let a = frame.at(c, 0x10);
                c.g[S1][0] = c.ld(a);
                let a = frame.at(c, 0);
                c.g[S0][0] = c.ld(a);
                frame.close(c);
                c.pc = OUT_CALL_SET_DONE.addr;
                (OUT_CALL_SET_DONE.f)(c);
                return Ok(());
            }
        }
        c.cycles += 4;
        let a = frame.at(c, 0x10);
        c.g[S1][0] = c.ld(a);
        let a = frame.at(c, 0);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_OutputFieldPair(decState, rec1, rec2, n)` (0x4087d0): the
/// two fields of an interlaced picture: their pts and header fields go into
/// the decoder state (the field order flag 0x40 when the state is 2), then the
/// output buffer is checked and, when both records are ready, the pair is
/// converted (or copied) and reported done.
pub fn libmpeg_output_field_pair(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = 0x2;
        let a = frame.at(c, 0x60);
        let v = c.g[S6][0];
        c.sd(a, v);
        let a = frame.at(c, 0x50);
        let v = c.g[S5][0];
        c.sd(a, v);
        c.g[S6][0] = 0;
        let a = frame.at(c, 0x20);
        let v = c.g[S2][0];
        c.sd(a, v);
        c.g[S5][0] = c.g[A2][0];
        let a = frame.at(c, 0x10);
        let v = c.g[S1][0];
        c.sd(a, v);
        let a = frame.at(c, 0x70);
        let v = c.g[RA][0];
        c.sd(a, v);
        c.g[S1][0] = c.g[A0][0];
        let a = frame.at(c, 0x40);
        let v = c.g[S4][0];
        c.sd(a, v);
        let a = frame.at(c, 0x30);
        let v = c.g[S3][0];
        c.sd(a, v);
        let a = frame.at(c, 0);
        let v = c.g[S0][0];
        c.sd(a, v);
        let dec = c.g[S1][0] as u32;
        c.g[V1][0] = Addr(dec + 0x174).lw(c);
        let not_two = c.g[V1][0] != c.g[V0][0];
        c.g[S2][0] = c.g[A1][0];
        if not_two {
            c.cycles += 2;
            c.g[S3][0] = c.g[S5][0];
            c.g[S4][0] = c.g[S2][0];
        } else {
            c.cycles += 4;
            c.g[S3][0] = c.g[S2][0];
            c.g[S4][0] = c.g[S5][0];
            c.g[S6][0] = 0x40;
        }
        c.cycles += 46;
        let hv = Addr(dec + 0x858).lw(c);
        c.g[A3][0] = hv;
        let h = hv as u32;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        c.g[S0][0] = 0x1;
        c.g[T0][0] = sx(h.wrapping_add(0x20));
        c.g[A2][0] = sx(h.wrapping_add(0x10));
        c.g[A3][0] = sx(h.wrapping_add(0x18));
        OUT_CALL_COMPUTE_PTS.call(c, 0x0040_8848)?;
        let hv = Addr(dec + 0x858).lw(c);
        c.g[A3][0] = hv;
        let h = hv as u32;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[V0][0] = Addr(h + 0x10).lw(c);
        c.g[T0][0] = sx(h.wrapping_add(0x38));
        c.g[A2][0] = sx(h.wrapping_add(0x28));
        let s0 = c.g[S0][0];
        c.sd(dec + 0x88, s0);
        let v0 = c.g[V0][0];
        c.sw(dec + 0x80, v0);
        c.g[A3][0] = sx(h.wrapping_add(0x30));
        OUT_CALL_COMPUTE_PTS.call(c, 0x0040_8870)?;
        let hv = Addr(dec + 0x858).lw(c);
        c.g[A3][0] = hv;
        let h = hv as u32;
        let s3 = c.g[S3][0] as u32;
        let s4 = c.g[S4][0] as u32;
        c.g[T0][0] = c.g[S6][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[V1][0] = Addr(h + 0x28).lw(c);
        let s0 = c.g[S0][0];
        c.sd(dec + 0x88, s0);
        let v1 = c.g[V1][0];
        c.sw(dec + 0x80, v1);
        c.g[V0][0] = c.ld(h + 0x20);
        c.g[A2][0] = Addr(s3 + 0x5c).lw(c);
        c.g[V0][0] |= c.g[T0][0];
        c.g[V1][0] = c.ld(h + 0x38);
        let a2 = c.g[A2][0];
        c.sw(dec + 0xcc, a2);
        let v0 = c.g[V0][0];
        c.sd(h + 0x20, v0);
        c.g[V1][0] |= c.g[T0][0];
        c.g[A2][0] = Addr(s3 + 0x60).lw(c);
        let v1 = c.g[V1][0];
        c.sd(h + 0x38, v1);
        let a2 = c.g[A2][0];
        c.sw(dec + 0xd0, a2);
        c.g[V0][0] = Addr(s3 + 0x44).lw(c);
        let v0 = c.g[V0][0];
        c.sw(dec + 0xb4, v0);
        c.g[V1][0] = Addr(s4 + 0x48).lw(c);
        let v1 = c.g[V1][0];
        c.sw(dec + 0xb8, v1);
        c.g[V0][0] = Addr(s3 + 0x50).lw(c);
        let v0 = c.g[V0][0];
        c.sw(dec + 0xc0, v0);
        c.g[V1][0] = Addr(s4 + 0x54).lw(c);
        let v1 = c.g[V1][0];
        c.sw(dec + 0xc4, v1);
        OUT_CALL_CHECK_BUFFER.call(c, 0x0040_88d8)?;
        let none = c.g[V0][0] == 0;
        c.g[V0][0] = 0x1;
        // The shared tail (+1, S6 reload) runs unless the second record is ready.
        let mut shared_tail = false;
        if none {
            c.cycles += 1;
            frame.restore_ra(c, 0x70);
            shared_tail = true;
        } else {
            c.cycles += 3;
            c.g[V1][0] = Addr(c.g[S2][0] as u32 + 0x28).lw(c);
            let rec1_not_one = c.g[V1][0] != c.g[V0][0];
            frame.restore_ra(c, 0x70);
            if rec1_not_one {
                shared_tail = true;
            } else {
                c.cycles += 3;
                let s5 = c.g[S5][0] as u32;
                c.g[V0][0] = Addr(s5 + 0x28).lw(c);
                let rec2_not_one = c.g[V0][0] != c.g[V1][0];
                let a = frame.at(c, 0x60);
                c.g[S6][0] = c.ld(a);
                if !rec2_not_one {
                    // Both records ready: convert or copy, then report done.
                    let s2 = c.g[S2][0] as u32;
                    c.cycles += 6;
                    c.g[V0][0] = Addr(s2 + 0x10).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    let v0 = c.g[V0][0];
                    c.sw(s2 + 0x10, v0);
                    c.g[V1][0] = Addr(dec + 0xb0).lw(c);
                    let no_csc = c.g[V1][0] == 0;
                    c.g[A0][0] = c.g[S1][0];
                    if no_csc {
                        c.cycles += 3;
                        c.g[A1][0] = c.g[S2][0];
                        OUT_CALL_COPY_RAW.call(c, 0x0040_8928)?;
                    } else {
                        c.cycles += 4;
                        c.g[A1][0] = c.g[S2][0];
                        OUT_CALL_CONVERT_CSC.call(c, 0x0040_8918)?;
                    }
                    c.g[V0][0] = Addr(s2 + 0x10).lw(c);
                    c.cycles += 13;
                    c.g[A0][0] = c.g[S1][0];
                    frame.restore_ra(c, 0x70);
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                    let v0 = c.g[V0][0];
                    c.sw(s2 + 0x10, v0);
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
                    let a = frame.at(c, 0);
                    c.g[S0][0] = c.ld(a);
                    frame.close(c);
                    c.pc = OUT_CALL_SET_DONE.addr;
                    (OUT_CALL_SET_DONE.f)(c);
                    return Ok(());
                }
            }
        }
        if shared_tail {
            c.cycles += 1;
            let a = frame.at(c, 0x60);
            c.g[S6][0] = c.ld(a);
        }
        // Epilogue (0x408968).
        c.cycles += 8;
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
        let a = frame.at(c, 0);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_IpuFromDmaHandler(xferState)` (0x408ca8): the DMA interrupt
/// for the IPU-to-memory transfer. Counts the finished chunk and starts the
/// next one (a full 0x3ff-quadword chunk, or the rest on the last one). Stops
/// when the IPU or the channel is busy. Returns 0; `+4` is set to 1 on a stop.
pub fn libmpeg_ipu_from_dma(c: &mut Cpu) {
    c.cycles += 13;
    c.sw(0x1000_e010, 8);
    c.g[T1][0] = c.g[A1][0];
    let x = c.g[T1][0] as u32;
    c.g[V0][0] = Addr(x).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    let v0 = c.g[V0][0];
    c.sw(x, v0);
    c.g[A0][0] = 0x1000_b020;
    c.g[V1][0] = c.lw(0x1000_b020);
    let busy = c.g[V1][0] != 0;
    c.g[V0][0] = 0x1;
    if !busy {
        c.cycles += 6;
        c.g[V0][0] = 0x1000_b000;
        c.g[V1][0] = c.lw(0x1000_b000);
        c.g[V1][0] &= 0x100;
        let channel_free = c.g[V1][0] == 0;
        c.g[V0][0] = 0x1;
        if channel_free {
            ipu_from_dma_next_chunk(c, x);
            c.cycles += 4;
            c.ei();
            c.g[V0][0] = 0;
            ret(c);
            return;
        }
    }
    c.cycles += 5;
    let v0 = c.g[V0][0];
    c.sw(x + 4, v0);
    c.ei();
    c.g[V0][0] = 0;
    ret(c);
}

/// The chunk part of the IPU-from-DMA handler: starts the next transfer when
/// the chunk count is below the last chunk (0x3ff quadwords), or the rest of
/// the data when it is the last one. Nothing when the count is past the last.
fn ipu_from_dma_next_chunk(c: &mut Cpu, x: u32) {
    c.cycles += 6;
    c.g[V1][0] = Addr(x + 0x10).lw(c);
    c.g[V0][0] = Addr(x).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
    let below_last = (c.g[V0][0] as i64) < (c.g[V1][0] as i64);
    c.g[V0][0] = below_last as u64;
    let not_below = c.g[V0][0] == 0;
    c.g[V0][0] = 0x1000_0000;
    if !not_below {
        c.cycles += 23;
        c.g[V1][0] = Addr(x + 0xc).lw(c);
        c.g[V0][0] = 0x1000_b010;
        c.g[A0][0] = 0xffc0;
        let v1 = c.g[V1][0];
        c.sw(0x1000_b010, v1);
        c.g[A1][0] = 0x100;
        c.sw(0x1000_b020, 0xffc0);
        c.g[V1][0] = 0x1000_0000;
        c.g[V0][0] = 0x7000_0000;
        c.sw(0x1000_b000, 0x100);
        c.g[V1][0] = 0x1000_2000;
        c.g[V0][0] = 0x7000_03ff;
        c.sw(0x1000_2000, 0x7000_03ff);
        c.g[A0][0] = 0xffc00;
        c.g[V1][0] = 0x0fff_ffff;
        c.g[V0][0] = Addr(x + 0xc).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xffc00));
        c.g[V0][0] &= c.g[V1][0];
        let v0 = c.g[V0][0];
        c.sw(x + 0xc, v0);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(x).lw(c);
    let not_last = c.g[V0][0] != c.g[V1][0];
    c.g[T0][0] = 0x100;
    if not_last {
        return;
    }
    c.cycles += 22;
    c.g[A1][0] = Addr(x).lw(c);
    c.g[A0][0] = 0x1000_b010;
    c.g[V1][0] = Addr(x + 8).lw(c);
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 10);
    c.g[A2][0] = Addr(x + 0xc).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    c.g[A1][0] = 0x1000_0000;
    let v1 = c.g[V1][0];
    c.sw(x + 8, v1);
    c.g[A3][0] = 0x7000_0000;
    let a2 = c.g[A2][0];
    c.sw(c.g[A0][0] as u32, a2);
    c.g[A1][0] = 0x1000_2000;
    c.g[V0][0] = Addr(x + 8).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
    let v0 = c.g[V0][0];
    c.sw(0x1000_b020, v0);
    let t0 = c.g[T0][0];
    c.sw(0x1000_b000, t0);
    c.g[V0][0] = Addr(x + 8).lw(c);
    c.g[V0][0] |= c.g[A3][0];
    let v0 = c.g[V0][0];
    c.sw(c.g[A1][0] as u32, v0);
}

/// `void libmpeg_IpuToDmaHandler(xferState)` (0x408fb0): the DMA interrupt for
/// the memory-to-IPU transfer. Sends the next chunk (at most 0xffff quadwords)
/// and advances the transfer. Returns 1 when nothing is left, else 0.
pub fn libmpeg_ipu_to_dma(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V1][0] = 0x10;
    c.sw(0x1000_e010, 0x10);
    c.g[A2][0] = c.g[A1][0];
    let x = c.g[A2][0] as u32;
    c.g[V0][0] = Addr(x).lw(c);
    let n = c.g[V0][0];
    c.g[A3][0] = 0xffff;
    if n == 0 {
        c.cycles += 4;
        c.ei();
        c.g[V0][0] = 0x1;
        ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = (0xffffu64 < n) as u64;
    c.g[V1][0] = 0x1000_0000;
    if c.g[V0][0] == 0 {
        // At most 0xffff left: send all of it.
        c.cycles += 12;
        c.g[A0][0] = Addr(x + 4).lw(c);
        let a0 = c.g[A0][0];
        c.sw(0x1000_b410, a0);
        c.g[V1][0] = 0x1000_b420;
        c.g[V0][0] = 0x1000_b400;
        c.g[A1][0] = 0x101;
        c.g[A0][0] = Addr(x).lw(c);
        let a0 = c.g[A0][0];
        c.sw(0x1000_b420, a0);
        c.sw(0x1000_b400, 0x101);
        c.sw(x, 0);
    } else {
        // More than 0xffff: send 0xffff and advance the address and count.
        c.cycles += 22;
        c.g[A0][0] = Addr(x + 4).lw(c);
        let a0 = c.g[A0][0];
        c.sw(0x1000_b410, a0);
        c.g[V1][0] = 0x1000_b420;
        c.sw(0x1000_b420, 0xffff);
        c.g[V0][0] = 0x1000_b400;
        c.g[V1][0] = 0x101;
        c.sw(0x1000_b400, 0x101);
        c.g[A1][0] = 0x000f_fff0;
        c.g[A0][0] = 0x0fff_ffff;
        c.g[V1][0] = Addr(x + 4).lw(c);
        c.g[V0][0] = Addr(x).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V1][0] &= c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A3][0] as u32));
        let v0 = c.g[V0][0];
        c.sw(x, v0);
        let v1 = c.g[V1][0];
        c.sw(x + 4, v1);
    }
    c.cycles += 4;
    c.ei();
    c.g[V0][0] = 0;
    ret(c);
}

/// `void libmc_DelayAlarmHandler(alarmId, ticks, sema)` (0x40a320): signals the
/// semaphore (the third argument) through `iSignalSema`.
pub fn libmc_delay_alarm(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A0][0] = c.g[A2][0];
    c.pc = ALARM_CALL_SIGNAL_SEMA.addr;
    (ALARM_CALL_SIGNAL_SEMA.f)(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::{Rng, DATA};
    use std::ptr::addr_of_mut;

    /// Gate of callee fake `k`: `OC1592_MAGIC` there turns it on, the word
    /// after holds its return value.
    const OC1592_MAGIC: u32 = 0x4d50_3e92;

    const fn oc1592_gate(k: u32) -> u32 {
        DATA + 0xf_e000 + 0x10 * k
    }

    macro_rules! oc1592_fake {
        ($name:ident, $k:expr, $addr:expr, $gen:path) => {
            fn $name(c: &mut Cpu) {
                let gate = oc1592_gate($k);
                if c.bus.read32(gate) != OC1592_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
                if c.pc == ra {
                    c.g[V0][0] = sx(c.bus.read32(gate + 4));
                }
            }
        };
    }

    oc1592_fake!(
        oc1592_fake_error,
        0,
        0x0040_34a8,
        ee::s15::f_004034a8__generated
    );
    oc1592_fake!(
        oc1592_fake_slices,
        1,
        0x0040_60c8,
        ee::s15::f_004060c8__generated
    );
    oc1592_fake!(
        oc1592_fake_frame,
        2,
        0x0040_86c0,
        ee::s15::f_004086c0__generated
    );
    oc1592_fake!(
        oc1592_fake_field,
        3,
        0x0040_87d0,
        ee::s15::f_004087d0__generated
    );
    oc1592_fake!(
        oc1592_fake_pts,
        4,
        0x0040_8530,
        ee::s15::f_00408530__generated
    );
    oc1592_fake!(
        oc1592_fake_check,
        5,
        0x0040_81d8,
        ee::s15::f_004081d8__generated
    );
    oc1592_fake!(
        oc1592_fake_csc,
        6,
        0x0040_9088,
        ee::s15::f_00409088__generated
    );
    oc1592_fake!(
        oc1592_fake_copy,
        7,
        0x0040_8278,
        ee::s15::f_00408278__generated
    );
    oc1592_fake!(
        oc1592_fake_setdone,
        8,
        0x0040_8508,
        ee::s15::f_00408508__generated
    );
    oc1592_fake!(
        oc1592_fake_semaphore,
        9,
        0x0042_3dd0,
        ee::s15::f_00423dd0__generated
    );

    fn oc1592_install() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004034A8),
            oc1592_fake_error,
            fakes::prev_cell(oc1592_fake_error),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004060C8),
            oc1592_fake_slices,
            fakes::prev_cell(oc1592_fake_slices),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004086C0),
            oc1592_fake_frame,
            fakes::prev_cell(oc1592_fake_frame),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004087D0),
            oc1592_fake_field,
            fakes::prev_cell(oc1592_fake_field),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00408530),
            oc1592_fake_pts,
            fakes::prev_cell(oc1592_fake_pts),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004081D8),
            oc1592_fake_check,
            fakes::prev_cell(oc1592_fake_check),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00409088),
            oc1592_fake_csc,
            fakes::prev_cell(oc1592_fake_csc),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00408278),
            oc1592_fake_copy,
            fakes::prev_cell(oc1592_fake_copy),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00408508),
            oc1592_fake_setdone,
            fakes::prev_cell(oc1592_fake_setdone),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00423DD0),
            oc1592_fake_semaphore,
            fakes::prev_cell(oc1592_fake_semaphore),
        );
    }

    /// Arms every callee fake; the return value of fake `k` is bit `k` of `seed`.
    fn oc1592_arm(c: &mut Cpu, seed: u32) {
        for k in 0..10 {
            c.bus.write32(oc1592_gate(k), OC1592_MAGIC);
            c.bus.write32(oc1592_gate(k) + 4, (seed >> k) & 1);
        }
    }

    const OC1592_DEC: u32 = DATA + 0x1000;
    const OC1592_REC: u32 = DATA + 0x2000;
    const OC1592_HANDLE: u32 = DATA + 0x3000;

    /// A decoder state with the picture records and the record fields the
    /// output chain reads.
    fn oc1592_decoder(c: &mut Cpu, seed: u32) {
        c.bus.write32(OC1592_DEC + 0x858, OC1592_HANDLE);
        c.bus.write32(OC1592_DEC + 0x1c0, OC1592_REC);
        c.bus.write32(OC1592_DEC + 0x1d0, OC1592_REC + 0x100);
        c.bus.write32(OC1592_DEC + 0x1e0, OC1592_REC + 0x200);
        for off in [
            0x10u32, 0x18, 0x20, 0x24, 0x28, 0x30, 0x38, 0x3c, 0x44, 0x48, 0x4c, 0x50, 0x54, 0x58,
            0x5c, 0x60,
        ] {
            c.bus.write32(
                OC1592_HANDLE + off,
                seed.wrapping_mul(0x9e37).wrapping_add(off),
            );
        }
        for rec in [OC1592_REC, OC1592_REC + 0x100, OC1592_REC + 0x200] {
            for off in [
                0x10u32, 0x28, 0x44, 0x48, 0x4c, 0x50, 0x54, 0x58, 0x5c, 0x60,
            ] {
                c.bus.write32(rec + off, seed.wrapping_add(off));
            }
        }
    }

    fn oc1592_dec_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x1592, 2);
        oc1592_arm(c, seed);
        oc1592_decoder(c, seed);
        c.g[A0][0] = OC1592_DEC as u64;
        let state = [0u32, 1, 2, 3, 4, 0xffff_ffff][(seed % 6) as usize];
        c.bus.write32(OC1592_DEC + 0x174, state);
        c.bus
            .write32(OC1592_DEC + 0x120, if (seed / 6) % 2 == 0 { 0 } else { 5 });
        c.bus.write32(OC1592_REC + 0x28, seed % 2);
    }

    fn oc1592_opic_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x1593, 2);
        oc1592_arm(c, seed);
        oc1592_decoder(c, seed);
        c.g[A0][0] = OC1592_DEC as u64;
        c.g[A1][0] = OC1592_REC as u64;
        c.g[A2][0] = if seed % 4 == 0 { 0 } else { 1 };
        let state = [3u32, 2, 1, 5][((seed / 4) % 4) as usize];
        c.bus.write32(OC1592_DEC + 0x174, state);
        c.bus
            .write32(OC1592_DEC + 0x150, if (seed / 16) % 2 == 0 { 3 } else { 1 });
        for off in [0x1b8u32, 0x1c4, 0x1c8, 0x1d4, 0x1d8, 0x1e4] {
            c.bus.write32(OC1592_DEC + off, OC1592_REC + off);
        }
        c.bus.write32(OC1592_DEC + 0xf8, (seed / 32) % 3);
    }

    fn oc1592_oframe_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x1594, 2);
        oc1592_arm(c, seed);
        oc1592_decoder(c, seed);
        c.g[A0][0] = OC1592_DEC as u64;
        c.g[A1][0] = OC1592_REC as u64;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.bus.write32(OC1592_REC + 0x28, seed % 3);
        c.bus.write32(OC1592_DEC + 0xb0, (seed / 3) % 2);
    }

    fn oc1592_ofield_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x1595, 2);
        oc1592_arm(c, seed);
        oc1592_decoder(c, seed);
        c.g[A0][0] = OC1592_DEC as u64;
        c.g[A1][0] = OC1592_REC as u64;
        c.g[A2][0] = (OC1592_REC + 0x100) as u64;
        c.g[A3][0] = 1;
        let state = [2u32, 1, 3, 2][(seed % 4) as usize];
        c.bus.write32(OC1592_DEC + 0x174, state);
        c.bus.write32(OC1592_DEC + 0xb0, (seed / 4) % 2);
        c.bus
            .write32(OC1592_REC + 0x28, if (seed / 8) % 4 == 0 { 2 } else { 1 });
        c.bus.write32(
            OC1592_REC + 0x100 + 0x28,
            if (seed / 16) % 2 == 0 { 1 } else { 2 },
        );
    }

    fn oc1592_ipu_from_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x1596, 2);
        oc1592_arm(c, seed);
        let x = OC1592_REC + 0x800;
        c.g[A1][0] = x as u64;
        c.bus.write32(x, seed % 3);
        c.bus.write32(x + 0x10, 1 + seed % 3);
        c.bus.write32(x + 0xc, 0x0010_0000 + seed * 0x1000);
        c.bus.write32(x + 8, 0x8000 + seed * 0x10);
        c.bus.write32(x + 4, 0);
        c.bus
            .write32(0x1000_b020, if seed % 5 == 0 { 1 } else { 0 });
        c.bus
            .write32(0x1000_b000, if seed % 7 == 0 { 0x100 } else { 0 });
    }

    fn oc1592_ipu_to_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x1597, 1);
        let x = OC1592_REC + 0x900;
        c.g[A1][0] = x as u64;
        c.bus
            .write32(x, [0u32, 0x10, 0x1_0000, 0x2_8000][(seed % 4) as usize]);
        c.bus.write32(x + 4, 0x0001_2345 + seed * 0x40);
    }

    fn oc1592_alarm_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x1598, 1);
        oc1592_arm(c, seed);
        c.g[A2][0] = (seed as u64) * 3 + 1;
    }

    #[test]
    fn libmpeg_decode_picture_slices_matches() {
        oc1592_install();
        check(
            ee::s15::f_00407dc0__generated,
            libmpeg_decode_picture_slices,
            48,
            &oc1592_dec_setup,
        );
    }

    #[test]
    fn libmpeg_output_picture_matches() {
        oc1592_install();
        check(
            ee::s15::f_00407e90__generated,
            libmpeg_output_picture,
            48,
            &oc1592_opic_setup,
        );
    }

    #[test]
    fn libmpeg_output_frame_matches() {
        oc1592_install();
        check(
            ee::s15::f_004086c0__generated,
            libmpeg_output_frame,
            48,
            &oc1592_oframe_setup,
        );
    }

    #[test]
    fn libmpeg_output_field_pair_matches() {
        oc1592_install();
        check(
            ee::s15::f_004087d0__generated,
            libmpeg_output_field_pair,
            48,
            &oc1592_ofield_setup,
        );
    }

    #[test]
    fn libmpeg_ipu_from_dma_matches() {
        oc1592_install();
        check(
            ee::s15::f_00408ca8__generated,
            libmpeg_ipu_from_dma,
            32,
            &oc1592_ipu_from_setup,
        );
    }

    #[test]
    fn libmpeg_ipu_to_dma_matches() {
        oc1592_install();
        check(
            ee::s15::f_00408fb0__generated,
            libmpeg_ipu_to_dma,
            16,
            &oc1592_ipu_to_setup,
        );
    }

    #[test]
    fn libmc_delay_alarm_matches() {
        oc1592_install();
        check(
            ee::s15::f_0040a320__generated,
            libmc_delay_alarm,
            16,
            &oc1592_alarm_setup,
        );
    }
}
