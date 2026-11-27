//! Staging module oc1116 (wave wave269): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1116`.

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
    (
        0x0040_2c80,
        libmpeg_callback_nodata,
        "libmpeg_CallbackNodata",
    ),
    (0x0040_2d50, libmpeg_work_area_init, "libmpeg_WorkAreaInit"),
    (0x0040_2d68, libmpeg_work_area_mark, "libmpeg_WorkAreaMark"),
    (
        0x0040_2d78,
        libmpeg_work_area_restore,
        "libmpeg_WorkAreaRestore",
    ),
    (
        0x0040_2d88,
        libmpeg_work_area_alloc,
        "libmpeg_WorkAreaAlloc",
    ),
    (
        0x0040_2f80,
        libmpeg_decode_frame_pic,
        "libmpeg_DecodeFramePic",
    ),
    (0x0040_3098, libmpeg_decode_picture, "libmpeg_DecodePicture"),
    (
        0x0040_30e0,
        libmpeg_decode_field_pic,
        "libmpeg_DecodeFieldPic",
    ),
];

// Direct-call targets of this module (address, readable alias in the shard).
const CALL_DISPATCH_CALLBACK: Callee = Callee {
    addr: 0x0040_2c30,
    f: ee::s15::libmpeg_DispatchCallback,
};
const CALL_CALLBACK_NODATA: Callee = Callee {
    addr: 0x0040_2c80,
    f: ee::s15::libmpeg_CallbackNodata,
};
const CALL_LIBMPEG_ERROR: Callee = Callee {
    addr: 0x0040_34a8,
    f: ee::s15::libmpeg_Error,
};
const CALL_SETUP_REF_FRAMES: Callee = Callee {
    addr: 0x0040_7f20,
    f: ee::s15::libmpeg_SetupRefFrames,
};
const CALL_DECODE_PICTURE_SLICES: Callee = Callee {
    addr: 0x0040_7dc0,
    f: ee::s15::libmpeg_DecodePictureSlices,
};
const CALL_GET_HEADER: Callee = Callee {
    addr: 0x0040_7520,
    f: ee::s15::libmpeg_GetHeader,
};
const CALL_FLUSH_PENDING: Callee = Callee {
    addr: 0x0040_3240,
    f: ee::s15::libmpeg_FlushPendingPicture,
};
const CALL_OUTPUT_PICTURE: Callee = Callee {
    addr: 0x0040_7e90,
    f: ee::s15::libmpeg_OutputPicture,
};
const CALL_DECODE_FIELD_PIC: Callee = Callee {
    addr: 0x0040_30e0,
    f: ee::s15::libmpeg_DecodeFieldPic,
};
const CALL_DECODE_FRAME_PIC: Callee = Callee {
    addr: 0x0040_2f80,
    f: ee::s15::libmpeg_DecodeFramePic,
};

/// `void libmpeg_CallbackNodata(mpeg)` (0x402c80): registers the one-entry
/// callback table `{1}` with `libmpeg_DispatchCallback` and returns 1.
pub fn libmpeg_callback_nodata(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0x20);
        c.g[A1][0] = c.g[SP][0];
        Addr(frame.at(c, 0)).sw(c, c.g[V0][0]);
        CALL_DISPATCH_CALLBACK.call(c, 0x0040_2c98)?;
        frame.restore_ra(c, 0x20);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_WorkAreaInit(area, base, size)` (0x402d50): a work area is
/// `{base, size, cursor, mark}` at +0, +4, +8 and +0xc; the cursor and the
/// mark start at `base`.
pub fn libmpeg_work_area_init(c: &mut Cpu) {
    c.cycles += 5;
    let area = Addr(c.g[A0][0] as u32);
    area.off(0xc).sw(c, c.g[A1][0]);
    area.off(4).sw(c, c.g[A2][0]);
    area.sw(c, c.g[A1][0]);
    area.off(8).sw(c, c.g[A1][0]);
    ret(c);
}

/// `void libmpeg_WorkAreaMark(area)` (0x402d68): records the cursor (+8) as
/// the mark (+0xc), so a later restore can rewind to it.
pub fn libmpeg_work_area_mark(c: &mut Cpu) {
    c.cycles += 3;
    let area = Addr(c.g[A0][0] as u32);
    c.g[V0][0] = area.off(8).lw(c);
    area.off(0xc).sw(c, c.g[V0][0]);
    ret(c);
}

/// `void libmpeg_WorkAreaRestore(area)` (0x402d78): rewinds the cursor (+8)
/// to the mark (+0xc).
pub fn libmpeg_work_area_restore(c: &mut Cpu) {
    c.cycles += 3;
    let area = Addr(c.g[A0][0] as u32);
    c.g[V0][0] = area.off(0xc).lw(c);
    area.off(8).sw(c, c.g[V0][0]);
    ret(c);
}

/// `void* libmpeg_WorkAreaAlloc(dec, pool, size, align)` (0x402d88): bump
/// allocates `size` bytes at an `align` boundary from the pool `{base, size,
/// cursor}`. Returns the block start; when the block would run past the
/// pool's end it reports through `libmpeg_Error` (the 0x497c48 message) and
/// returns 0.
pub fn libmpeg_work_area_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[T0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.cycles += 13;
        let pool = Addr(c.g[A1][0] as u32);
        c.g[V0][0] = pool.off(8).lw(c);
        c.g[A0][0] = pool.off(4).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[V1][0] = pool.lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.divu(0, 2, 7);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = c.lo[0];
        c.mult(0, 2, 2, 7);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V1][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
        if c.g[V1][0] != 0 {
            c.g[A1][0] = 0x0049_0000;
            c.cycles += 4;
            c.g[A0][0] = c.g[T0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7c48));
            CALL_LIBMPEG_ERROR.call(c, 0x0040_2de4)?;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 2;
            pool.off(8).sw(c, c.g[A0][0]);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_DecodeFramePic(mpeg, arg1, arg2)` (0x402f80): decodes one
/// frame picture. With the second limit (`arg2`) not -1 and `arg1 >= arg2`
/// the picture is skipped: reference frames are set up and the callback is
/// registered (`S3 = 1`). Otherwise the decoder's counters are set up, the
/// reference frames are set up and the slices are decoded. Then the
/// picture is output, the counters advance, and `S2` (the picture result)
/// is returned.
pub fn libmpeg_decode_frame_pic(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = u64::MAX;
        Addr(frame.at(c, 0x30)).sd(c, c.g[S3][0]);
        Addr(frame.at(c, 0x10)).sd(c, c.g[S1][0]);
        c.g[S3][0] = 0;
        Addr(frame.at(c, 0x40)).sd(c, c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        Addr(frame.at(c, 0x20)).sd(c, c.g[S2][0]);
        Addr(frame.at(c, 0)).sd(c, c.g[S0][0]);
        let no_limit = c.g[A2][0] == c.g[V0][0];
        c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x40).lw(c);
        let mut skipped = false;
        if !no_limit {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
            skipped = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
        }
        if skipped {
            c.cycles += 6;
            c.g[A1][0] = 0;
            CALL_SETUP_REF_FRAMES.call(c, 0x0040_2ffc)?;
            c.g[S3][0] = 1;
            c.g[S2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            CALL_CALLBACK_NODATA.call(c, 0x0040_300c)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(8).lw(c);
            let counted = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if !counted {
                c.cycles += 3;
                Addr(c.g[S1][0] as u32).off(8).sw(c, 0);
                c.g[V0][0] = 1;
                Addr(c.g[S0][0] as u32).off(8).sw(c, c.g[V0][0]);
            }
            c.cycles += 4;
            c.g[A1][0] = 0;
            CALL_SETUP_REF_FRAMES.call(c, 0x0040_2fd8)?;
            let no_slices = c.g[V0][0] == 0;
            c.g[V1][0] = 0;
            if !no_slices {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                CALL_DECODE_PICTURE_SLICES.call(c, 0x0040_2fe8)?;
                c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
            }
            c.cycles += 2;
            c.g[S2][0] = c.g[V1][0];
        }
        // Output and counters.
        c.cycles += 8;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x118).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
        CALL_OUTPUT_PICTURE.call(c, 0x0040_301c)?;
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x174).lw(c);
        c.g[V0][0] = 3;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0xac).lw(c);
        } else {
            c.cycles += 2;
            if c.g[S3][0] != 0 {
                c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0xac).lw(c);
            } else {
                c.cycles += 4;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x120).lw(c);
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
                Addr(c.g[S0][0] as u32).off(0x120).sw(c, c.g[V0][0]);
                c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0xac).lw(c);
            }
        }
        c.cycles += 6;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x118).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        Addr(c.g[S1][0] as u32).off(8).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x120).lw(c);
        c.g[V0][0] = c.g[S2][0];
        if c.g[V1][0] == 0 {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x118).lw(c);
            c.g[V1][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            Addr(c.g[S0][0] as u32).off(0x118).sw(c, c.g[V0][0]);
            Addr(c.g[S0][0] as u32).off(4).sw(c, c.g[V1][0]);
            c.g[V0][0] = c.g[S2][0];
        }
        c.cycles += 7;
        c.g[RA][0] = Addr(frame.at(c, 0x40)).ld(c);
        c.g[S3][0] = Addr(frame.at(c, 0x30)).ld(c);
        c.g[S2][0] = Addr(frame.at(c, 0x20)).ld(c);
        c.g[S1][0] = Addr(frame.at(c, 0x10)).ld(c);
        c.g[S0][0] = Addr(frame.at(c, 0)).ld(c);
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_DecodePicture(mpeg)` (0x403098): a progressive picture
/// (`decoder+0x174 == 3`) goes to `libmpeg_DecodeFramePic`, anything else to
/// `libmpeg_DecodeFieldPic`, both with the mpeg object.
pub fn libmpeg_decode_picture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = 3;
        c.g[A0][0] = Addr(c.g[A3][0] as u32).off(0x40).lw(c);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x174).lw(c);
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 3;
            c.g[A0][0] = c.g[A3][0];
            CALL_DECODE_FRAME_PIC.call(c, 0x0040_30d0)?;
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[A3][0];
            CALL_DECODE_FIELD_PIC.call(c, 0x0040_30c0)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_DecodeFieldPic(mpeg, arg1, arg2)` (0x4030e0): decodes one
/// field picture. The header comes from `libmpeg_GetHeader`; a failed header
/// flushes the pending picture and returns 0. A header that matches the
/// decoder's expected picture kind sets up the reference frames and slices,
/// outputs the picture, advances the counters and returns `S4`.
pub fn libmpeg_decode_field_pic(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x60);
        'epilogue: {
            c.g[V0][0] = u64::MAX;
            Addr(frame.at(c, 0x30)).sd(c, c.g[S3][0]);
            Addr(frame.at(c, 0x20)).sd(c, c.g[S2][0]);
            c.g[S3][0] = 0;
            Addr(frame.at(c, 0x50)).sd(c, c.g[RA][0]);
            c.g[S2][0] = c.g[A0][0];
            Addr(frame.at(c, 0x40)).sd(c, c.g[S4][0]);
            Addr(frame.at(c, 0x10)).sd(c, c.g[S1][0]);
            Addr(frame.at(c, 0)).sd(c, c.g[S0][0]);
            c.g[S0][0] = Addr(c.g[S2][0] as u32).off(0x40).lw(c);
            let mut s3_set = c.g[A2][0] == c.g[V0][0];
            Addr(c.g[S0][0] as u32).off(0x120).sw(c, 0);
            if !s3_set {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                s3_set = c.g[V0][0] != 0;
            }
            if s3_set {
                c.cycles += 2;
                c.g[S3][0] = 1;
            }
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(8).lw(c);
            c.cycles += 2;
            let counted = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if !counted {
                c.cycles += 3;
                Addr(c.g[S2][0] as u32).off(8).sw(c, 0);
                c.g[V0][0] = 1;
                Addr(c.g[S0][0] as u32).off(8).sw(c, c.g[V0][0]);
            }
            // The reference frames, and the slices when the field has them.
            c.cycles += 4;
            c.g[A1][0] = 0;
            CALL_SETUP_REF_FRAMES.call(c, 0x0040_3144)?;
            c.g[S1][0] = 1;
            let mut to_tail = true;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                if c.g[S3][0] == 0 {
                    to_tail = false;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    CALL_DECODE_PICTURE_SLICES.call(c, 0x0040_315c)?;
                    c.g[S1][0] = 1;
                }
            }
            if to_tail {
                c.cycles += 1;
                c.g[A0][0] = c.g[S0][0];
            }
            // The picture header.
            c.cycles += 4;
            Addr(c.g[S0][0] as u32).off(0x120).sw(c, c.g[S1][0]);
            CALL_GET_HEADER.call(c, 0x0040_316c)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0xd4).lw(c);
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                CALL_FLUSH_PENDING.call(c, 0x0040_317c)?;
                Addr(c.g[S0][0] as u32).sw(c, c.g[S1][0]);
                c.g[V0][0] = 0;
                break 'epilogue;
            }
            c.cycles += 6;
            c.g[V1][0] = 2;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x174).lw(c);
            c.g[V0][0] ^= 1;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[S1][0];
            }
            let mismatch = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = u64::MAX;
            if mismatch {
                break 'epilogue;
            }
            // The field's slices, then its output.
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            CALL_SETUP_REF_FRAMES.call(c, 0x0040_31ac)?;
            c.g[V1][0] = 0;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[S1][0];
            }
            let no_slices = c.g[V1][0] == 0;
            c.g[S4][0] = 0;
            let mut to_output = false;
            if !no_slices {
                c.cycles += 2;
                if c.g[S3][0] == 0 {
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x118).lw(c);
                    to_output = true;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    CALL_DECODE_PICTURE_SLICES.call(c, 0x0040_31cc)?;
                    if c.g[V0][0] != 0 {
                        c.g[S4][0] = c.g[S1][0];
                    }
                }
            }
            if !to_output {
                c.cycles += 1;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x118).lw(c);
            }
            c.cycles += 15;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
            CALL_OUTPUT_PICTURE.call(c, 0x0040_31e0)?;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x118).lw(c);
            c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0xac).lw(c);
            Addr(c.g[S0][0] as u32).off(0x120).sw(c, 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            Addr(c.g[S2][0] as u32).off(8).sw(c, c.g[V0][0]);
            c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x118).lw(c);
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            Addr(c.g[S0][0] as u32).off(0x118).sw(c, c.g[V1][0]);
            let picture_skipped = c.g[S3][0] != 0;
            Addr(c.g[S0][0] as u32).off(4).sw(c, c.g[V0][0]);
            if !picture_skipped {
                c.cycles += 2;
                c.g[A0][0] = c.g[S2][0];
                CALL_CALLBACK_NODATA.call(c, 0x0040_3218)?;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S4][0];
        }
        c.cycles += 8;
        c.g[RA][0] = Addr(frame.at(c, 0x50)).ld(c);
        c.g[S4][0] = Addr(frame.at(c, 0x40)).ld(c);
        c.g[S3][0] = Addr(frame.at(c, 0x30)).ld(c);
        c.g[S2][0] = Addr(frame.at(c, 0x20)).ld(c);
        c.g[S1][0] = Addr(frame.at(c, 0x10)).ld(c);
        c.g[S0][0] = Addr(frame.at(c, 0)).ld(c);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word, result mailboxes of this module's fakes.
    pub(crate) const OC1116_CTRL: u32 = DATA + 0xf_c000;
    pub(crate) const OC1116_MAGIC: u32 = 0x4d50_3163;
    /// Mailbox word `k` (one per fake, in the `oc1116_fake_*` order below).
    pub(crate) const OC1116_RET: u32 = OC1116_CTRL + 0x100;
    /// The mpeg object (its decoder pointer at +0x40) and the decoder state.
    pub(crate) const OC1116_MPEG: u32 = DATA + 0x2000;
    pub(crate) const OC1116_DEC: u32 = DATA + 0x3000;
    /// A work area and a bump pool `{base, size, cursor}`.
    pub(crate) const OC1116_AREA: u32 = DATA + 0x5000;
    pub(crate) const OC1116_POOL: u32 = DATA + 0x6000;

    pub(crate) static OC1116_PREV_SETUP: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC1116_PREV_SLICES: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC1116_PREV_HEADER: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC1116_PREV_FLUSH: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC1116_PREV_OUTPUT: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC1116_PREV_FIELD: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC1116_PREV_FRAME: AtomicUsize = AtomicUsize::new(0);

    /// Log the call and return the mailbox word `k` (zero-extended into `v0`).
    pub(crate) fn oc1116_fake(c: &mut Cpu, addr: u32, k: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc == ra {
            c.g[V0][0] = sx(c.bus.read32(OC1116_RET + 4 * k));
        }
    }

    macro_rules! oc1116_gated {
        ($name:ident, $addr:expr, $k:expr, $prev:ident, $gen:path) => {
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1116_CTRL) != OC1116_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                oc1116_fake(c, $addr, $k)
            }
        };
    }
    oc1116_gated!(
        oc1116_fake_setup,
        0x0040_7f20,
        0,
        OC1116_PREV_SETUP,
        ee::s15::f_00407f20__generated
    );
    oc1116_gated!(
        oc1116_fake_slices,
        0x0040_7dc0,
        1,
        OC1116_PREV_SLICES,
        ee::s15::f_00407dc0__generated
    );
    oc1116_gated!(
        oc1116_fake_header,
        0x0040_7520,
        2,
        OC1116_PREV_HEADER,
        ee::s15::f_00407520__generated
    );
    oc1116_gated!(
        oc1116_fake_flush,
        0x0040_3240,
        3,
        OC1116_PREV_FLUSH,
        ee::s15::f_00403240__generated
    );
    oc1116_gated!(
        oc1116_fake_output,
        0x0040_7e90,
        4,
        OC1116_PREV_OUTPUT,
        ee::s15::f_00407e90__generated
    );
    oc1116_gated!(
        oc1116_fake_field,
        0x0040_30e0,
        5,
        OC1116_PREV_FIELD,
        ee::s15::f_004030e0__generated
    );
    oc1116_gated!(
        oc1116_fake_frame,
        0x0040_2f80,
        6,
        OC1116_PREV_FRAME,
        ee::s15::f_00402f80__generated
    );

    pub(crate) fn oc1116_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00407F20),
            oc1116_fake_setup,
            &OC1116_PREV_SETUP,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00407DC0),
            oc1116_fake_slices,
            &OC1116_PREV_SLICES,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00407520),
            oc1116_fake_header,
            &OC1116_PREV_HEADER,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00403240),
            oc1116_fake_flush,
            &OC1116_PREV_FLUSH,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00407E90),
            oc1116_fake_output,
            &OC1116_PREV_OUTPUT,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004030E0),
            oc1116_fake_field,
            &OC1116_PREV_FIELD,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00402F80),
            oc1116_fake_frame,
            &OC1116_PREV_FRAME,
        );
    }

    /// The mailboxes of the fakes, in the `k` order above.
    pub(crate) fn oc1116_rets(c: &mut Cpu, rets: [u32; 7]) {
        for (k, v) in rets.iter().enumerate() {
            c.bus.write32(OC1116_RET + 4 * k as u32, *v);
        }
    }

    /// A decoder state the field and frame paths read: the mpeg object at
    /// `MPEG` with its decoder at `+0x40`. The `DispatchCallback` table word
    /// (`DEC+0x14`) is zero, so the real `CallbackNodata` returns at once.
    pub(crate) fn oc1116_decoder(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(OC1116_CTRL, OC1116_MAGIC);
        c.bus.write32(OC1116_MPEG + 0x40, OC1116_DEC);
        c.bus.write32(OC1116_MPEG + 8, r.next_u32());
        let d = OC1116_DEC;
        c.bus.write32(d, r.next_u32() % 0x1000);
        c.bus.write32(d + 4, r.next_u32() % 0x10);
        c.bus.write32(
            d + 8,
            if seed % 2 == 0 {
                0
            } else {
                1 + r.next_u32() % 3
            },
        );
        c.bus.write32(d + 0x14, 0);
        c.bus.write32(d + 0xac, r.next_u32() % 4);
        c.bus.write32(d + 0x118, r.next_u32() % 5);
        c.bus.write32(d + 0x120, (seed / 3) % 2);
        c.bus.write32(d + 0xd4, seed % 3);
        c.bus.write32(d + 0x174, 1 + (seed / 2) % 2);
    }

    /// A full 64-bit register value from the generator.
    fn oc1116_wide(r: &mut Rng) -> u64 {
        ((r.next_u32() as u64) << 32) | r.next_u32() as u64
    }

    /// `CallbackNodata`: with the decoder's callback table empty the real
    /// `DispatchCallback` returns at once; the null mpeg and the null
    /// decoder take its early exits.
    #[test]
    fn libmpeg_callback_nodata_matches() {
        check(
            ee::s15::f_00402c80__generated,
            libmpeg_callback_nodata,
            200,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x402c80, 0);
                c.bus.write32(OC1116_DEC + 0x14, 0);
                c.bus.write32(
                    OC1116_MPEG + 0x40,
                    if seed % 3 == 0 { 0 } else { OC1116_DEC },
                );
                c.g[A0][0] = if seed % 4 == 0 { 0 } else { sx(OC1116_MPEG) };
                c.g[A1][0] = oc1116_wide(&mut r);
                c.pc = 0x0040_2c80;
            },
        );
    }

    /// `WorkAreaInit`: the four words of the area, from arbitrary 64-bit
    /// arguments (only the low 32 bits are stored).
    #[test]
    fn libmpeg_work_area_init_matches() {
        check(
            ee::s15::f_00402d50__generated,
            libmpeg_work_area_init,
            200,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x402d50, 0);
                c.g[A0][0] = sx(OC1116_AREA);
                c.g[A1][0] = oc1116_wide(&mut r);
                c.g[A2][0] = oc1116_wide(&mut r);
                for k in 0..4u32 {
                    c.bus.write32(OC1116_AREA + 4 * k, r.next_u32());
                }
                c.pc = 0x0040_2d50;
            },
        );
    }

    /// `WorkAreaMark`: the cursor (+8) is copied to the mark (+0xc).
    #[test]
    fn libmpeg_work_area_mark_matches() {
        check(
            ee::s15::f_00402d68__generated,
            libmpeg_work_area_mark,
            200,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x402d68, 0);
                c.g[A0][0] = sx(OC1116_AREA);
                c.bus.write32(OC1116_AREA + 8, r.next_u32());
                c.bus.write32(OC1116_AREA + 0xc, r.next_u32());
                c.pc = 0x0040_2d68;
            },
        );
    }

    /// `WorkAreaRestore`: the mark (+0xc) is copied back to the cursor (+8).
    #[test]
    fn libmpeg_work_area_restore_matches() {
        check(
            ee::s15::f_00402d78__generated,
            libmpeg_work_area_restore,
            200,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x402d78, 0);
                c.g[A0][0] = sx(OC1116_AREA);
                c.bus.write32(OC1116_AREA + 8, r.next_u32());
                c.bus.write32(OC1116_AREA + 0xc, r.next_u32());
                c.pc = 0x0040_2d78;
            },
        );
    }

    /// `WorkAreaAlloc`: the bump pool `{base 0x1000, size, cursor}` with
    /// power-of-two alignments. Small pools overflow (the `libmpeg_Error`
    /// leg, which takes the fast `DefaultErrorPrint` path because the
    /// decoder's `+0x858` is zero and the printf words are null); big pools
    /// take the success leg and advance the cursor.
    #[test]
    fn libmpeg_work_area_alloc_matches() {
        check(
            ee::s15::f_00402d88__generated,
            libmpeg_work_area_alloc,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x402d88, 0);
                c.bus.write32(OC1116_DEC + 0x858, 0);
                c.bus.write32(0x0049_7c68, 0);
                c.bus.write32(0x0049_7ce8, 0);
                let base = 0x1000u32;
                let pool_size: u32 = if seed % 3 == 0 { 0x100 } else { 0x1000 };
                let cursor = base + r.next_u32() % (pool_size + 0x40);
                let align = 1u32 << (seed % 5);
                let size = 1 + r.next_u32() % 0x80;
                c.bus.write32(OC1116_POOL, base);
                c.bus.write32(OC1116_POOL + 4, pool_size);
                c.bus.write32(OC1116_POOL + 8, cursor);
                c.g[A0][0] = sx(OC1116_DEC);
                c.g[A1][0] = sx(OC1116_POOL);
                c.g[A2][0] = size as u64;
                c.g[A3][0] = align as u64;
                c.pc = 0x0040_2d88;
            },
        );
    }

    /// `DecodePicture`: the progressive case (`+0x174 == 3`) and the field
    /// case, each through its gated fake (the callee bodies are not run).
    #[test]
    fn libmpeg_decode_picture_matches() {
        oc1116_install_fakes();
        check(
            ee::s15::f_00403098__generated,
            libmpeg_decode_picture,
            200,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x403098, 0);
                oc1116_decoder(c, &mut r, seed);
                c.bus.write32(
                    OC1116_DEC + 0x174,
                    if seed % 2 == 0 { 3 } else { 1 + (seed / 2) % 2 },
                );
                oc1116_rets(c, [0; 7]);
                c.g[A0][0] = sx(OC1116_MPEG);
                c.pc = 0x0040_3098;
            },
        );
    }

    /// `DecodeFramePic`: the skipped picture (`arg1 >= arg2`, or `arg2 == -1`
    /// with the limit off), the counted and uncounted decoder, the slices
    /// with and without the reference frames, and both tail counters.
    #[test]
    fn libmpeg_decode_frame_pic_matches() {
        oc1116_install_fakes();
        check(
            ee::s15::f_00402f80__generated,
            libmpeg_decode_frame_pic,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x402f80, 0);
                oc1116_decoder(c, &mut r, seed);
                oc1116_rets(c, [(seed % 3 != 0) as u32, (seed / 3) % 2, 0, 0, 0, 0, 0]);
                c.g[A0][0] = sx(OC1116_MPEG);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = if seed % 4 == 0 {
                    u64::MAX
                } else {
                    (seed / 4 % 3) as u64
                };
                c.pc = 0x0040_2f80;
            },
        );
    }

    /// `DecodeFieldPic`: every header result (failed, nonzero, the
    /// `+0xd4 == 1` case), the limit and the reference/slice legs, and the
    /// output tail. `CallbackNodata` runs for real.
    #[test]
    fn libmpeg_decode_field_pic_matches() {
        oc1116_install_fakes();
        check(
            ee::s15::f_004030e0__generated,
            libmpeg_decode_field_pic,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x4030e0, 0);
                oc1116_decoder(c, &mut r, seed);
                oc1116_rets(
                    c,
                    [
                        (seed % 3 != 0) as u32,
                        (seed / 3) % 2,
                        (seed % 5 != 0) as u32,
                        0,
                        0,
                        0,
                        0,
                    ],
                );
                c.g[A0][0] = sx(OC1116_MPEG);
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = if seed % 7 == 0 {
                    u64::MAX
                } else {
                    (seed / 7 % 3) as u64
                };
                c.pc = 0x0040_30e0;
            },
        );
    }
}
