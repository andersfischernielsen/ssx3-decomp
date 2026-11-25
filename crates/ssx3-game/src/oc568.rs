//! Staging module oc568 (wave l561.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc568`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0032_64b0,
        input_preproc_skip_block_comment,
        "InputPreproc_SkipBlockComment",
    ),
    (0x0032_b468, rle_encode, "RLE_Encode"),
    (
        0x0032_c648,
        sphere_tree_set_transform,
        "SphereTree_SetTransform",
    ),
    (
        0x0032_c770,
        sphere_tree_apply_transform,
        "SphereTree_ApplyTransform",
    ),
];

/// `base + off` as a 32-bit guest address (the `off(reg)` of the generated code).
fn sphere_addr(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `InputPreproc_Write` (0x326360): one character into the output buffer.
const SKIP_BLOCK_WRITE: Callee = Callee {
    addr: 0x0032_6360,
    f: ee::s10::f_00326360,
};
/// `InputPreproc_NextChar` (0x326478): advance the source by one character.
const SKIP_BLOCK_NEXT_CHAR: Callee = Callee {
    addr: 0x0032_6478,
    f: ee::s10::f_00326478,
};
/// `InputPreproc_EmptyStub` (0x326a20): the error report of an unterminated comment.
const SKIP_BLOCK_EMPTY_STUB: Callee = Callee {
    addr: 0x0032_6a20,
    f: ee::s10::f_00326a20,
};

/// `void InputPreproc_SkipBlockComment(self, src, a2, a3)` (0x3264b0): skips
/// a `/* ... */` comment that the preprocessor has begun. Newlines are written
/// through `InputPreproc_Write` and counted in `src->LineNumber` (`src + 0xc`),
/// and the first newline clears `single`. The closing `*/` ends the comment
/// (one space is written when no newline was seen). Input ending first is
/// reported through `InputPreproc_EmptyStub`. `CurrentChar` is `self + 0xc`.
pub fn input_preproc_skip_block_comment(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save(c, S7, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        // single: the comment is on one line until a newline clears it.
        c.g[S2][0] = 1;
        c.g[S3][0] = 0x2f; // '/'
        c.g[S4][0] = c.g[A2][0];
        c.g[S5][0] = c.g[A3][0];
        c.g[S6][0] = 0x2a; // '*'
        c.g[S7][0] = 0xa; // '\n'
        c.g[V0][0] = c.lw(sphere_addr(c.g[S0][0], 0xc));

        'resume: loop {
            'done: {
                'after_star: {
                    'err: {
                        'chr: {
                            // A newline: count it, write it, read on.
                            'nl: {
                                c.cycles += 2;
                                let t = c.g[V0][0] != c.g[S7][0];
                                c.g[A0][0] = c.g[S0][0];
                                if t {
                                    break 'nl;
                                }
                                c.cycles += 8;
                                c.g[V0][0] = c.lw(sphere_addr(c.g[S1][0], 0xc));
                                c.g[S2][0] = 0;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                                c.sw(sphere_addr(c.g[S1][0], 0xc), c.g[V0][0]);
                                c.g[A1][0] = c.lb(sphere_addr(c.g[S0][0], 0xc));
                                SKIP_BLOCK_WRITE.call(c, 0x0032_6520)?;
                                c.g[A0][0] = c.g[S0][0];
                                break 'chr;
                            }
                            // A '*': the comment ends at "*/", else read on.
                            'star: {
                                c.cycles += 2;
                                let t = c.g[V0][0] != c.g[S6][0];
                                if t {
                                    break 'star;
                                }
                                c.cycles += 5;
                                c.g[A1][0] = c.g[S1][0];
                                SKIP_BLOCK_NEXT_CHAR.call(c, 0x0032_6538)?;
                                c.g[V0][0] = c.lw(sphere_addr(c.g[S0][0], 0xc));
                                let t = c.g[V0][0] != c.g[S3][0];
                                c.g[A1][0] = c.g[S1][0];
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0032_6500);
                                    }
                                    continue 'resume;
                                }
                                c.cycles += 4;
                                c.g[A0][0] = c.g[S0][0];
                                SKIP_BLOCK_NEXT_CHAR.call(c, 0x0032_654c)?;
                                let t = c.g[S2][0] == 0;
                                c.g[A0][0] = c.g[S0][0];
                                if t {
                                    break 'after_star;
                                }
                                c.cycles += 4;
                                c.g[A1][0] = 0x20;
                                SKIP_BLOCK_WRITE.call(c, 0x0032_655c)?;
                                c.lq(16, sphere_addr(c.g[SP][0], 0x80));
                                break 'done;
                            }
                            // Any other character: at end of input (negative) give up.
                            c.cycles += 2;
                            let t = (c.g[V0][0] as i64) < 0;
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'err;
                            }
                        }
                        // Read the next character and go round again.
                        c.cycles += 4;
                        c.g[A1][0] = c.g[S1][0];
                        SKIP_BLOCK_NEXT_CHAR.call(c, 0x0032_6574)?;
                        c.g[V0][0] = c.lw(sphere_addr(c.g[S0][0], 0xc));
                        if event_due(c) {
                            deliver_event_here(c, 0x0032_6500);
                        }
                        continue 'resume;
                    }
                    // End of input inside the comment.
                    c.cycles += 6;
                    c.g[T0][0] = 0x490000;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S4][0];
                    c.g[A3][0] = c.g[S5][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1c28));
                    SKIP_BLOCK_EMPTY_STUB.call(c, 0x0032_6594)?;
                }
                c.cycles += 1;
                c.lq(16, sphere_addr(c.g[SP][0], 0x80));
            }
            c.cycles += 10;
            frame.restore(c, S1, 0x70);
            frame.restore(c, S2, 0x60);
            frame.restore(c, S3, 0x50);
            frame.restore(c, S4, 0x40);
            frame.restore(c, S5, 0x30);
            frame.restore(c, S6, 0x20);
            frame.restore(c, S7, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `int RLE_Encode(out, in, len, minRun)` (0x32b468, `sphTreeRLE`): run-length
/// encodes `len` bytes of `in` into `out`. A run of at least `minRun` bytes (2
/// at least, at most 127) is written as `count, byte`; other bytes go out as a
/// literal stretch `-n, bytes...` (at most 127). A zero byte ends the output,
/// and the result is the output length.
pub fn rle_encode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = 0x1;
        frame.save(c, S3, 0x40);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A3][0] as i64)) as u64;
        frame.save(c, S5, 0x20);
        c.g[S3][0] = c.g[A1][0]; // in
        frame.save(c, S4, 0x30);
        c.g[S5][0] = 0x2; // min run
        frame.save(c, S6, 0x10);
        c.g[S4][0] = sx(sphere_addr(c.g[S3][0], c.g[A2][0] as u32)); // end = in + len
        frame.save(c, S1, 0x60);
        c.g[S6][0] = c.g[A0][0]; // out (start)
        if c.g[V0][0] != 0 {
            c.g[S5][0] = c.g[A3][0];
        }
        frame.save(c, S0, 0x70);
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[S6][0]; // out (cursor)
        frame.save_ra(c, 0);

        'outer: loop {
            // while (in < end)
            c.cycles += 3;
            c.g[V0][0] = (c.g[S3][0] < c.g[S4][0]) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[A1][0] = c.g[S3][0];
            if t {
                break 'outer;
            }
            // A run starting at `in`, or a literal stretch: S0 = run, S2 = literal length.
            'run_or_lit: {
                c.cycles += 3;
                c.g[S0][0] = 0;
                let t = c.g[V0][0] == 0u64;
                c.g[S2][0] = 0;
                if t {
                    break 'run_or_lit;
                }
                c.cycles += 1;
                c.g[A0][0] = c.lbu(sphere_addr(c.g[A1][0], 0));
                'scan: loop {
                    // Measure the run at the cursor.
                    'run_len: {
                        c.cycles += 4;
                        c.g[S0][0] = 0x1;
                        c.g[V0][0] = c.lbu(sphere_addr(c.g[A1][0], 0x1));
                        let t = c.g[A0][0] != c.g[V0][0];
                        c.g[V1][0] = sx(sphere_addr(c.g[A1][0], 0x1));
                        if t {
                            break 'run_len;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[V1][0] < c.g[S4][0]) as u64;
                        let t = c.g[V0][0] == 0u64;
                        if t {
                            break 'run_len;
                        }
                        c.cycles += 1;
                        c.g[V1][0] = sx(sphere_addr(c.g[V1][0], 0x1));
                        'extend: loop {
                            c.cycles += 3;
                            c.g[V0][0] = c.lbu(sphere_addr(c.g[V1][0], 0));
                            let t = c.g[A0][0] != c.g[V0][0];
                            c.g[S0][0] = sx(sphere_addr(c.g[S0][0], 0x1));
                            if t {
                                break 'run_len;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[V1][0] < c.g[S4][0]) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = ((c.g[S0][0] as i64) < 127i64) as u64;
                            if t {
                                break 'run_len;
                            }
                            c.cycles += 2;
                            if c.g[V0][0] != 0u64 {
                                c.g[V1][0] = sx(sphere_addr(c.g[V1][0], 0x1));
                                if event_due(c) {
                                    deliver_event_here(c, 0x0032_b4e8);
                                }
                                continue 'extend;
                            } else {
                                break 'run_len;
                            }
                        }
                    }
                    // Clamp the run to the input, and drop a run of one byte at the end.
                    'run_clip: {
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[S4][0];
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                        if t {
                            break 'run_clip;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < 2i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                        if t {
                            break 'run_clip;
                        }
                        c.cycles += 3;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                        c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1));
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                    }
                    // A run that is long enough ends the literal stretch.
                    'run_end: {
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        if t {
                            break 'run_end;
                        }
                        c.cycles += 3;
                        c.g[S2][0] = sx(sphere_addr(c.g[S2][0], c.g[S0][0] as u32));
                        c.g[A1][0] = c.g[V1][0];
                        c.g[S0][0] = 0u64;
                    }
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 127i64) as u64;
                    if t {
                        break 'run_or_lit;
                    }
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = (c.g[A1][0] < c.g[S4][0]) as u64;
                    if t {
                        break 'run_or_lit;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.lbu(sphere_addr(c.g[A1][0], 0));
                        if event_due(c) {
                            deliver_event_here(c, 0x0032_b4c8);
                        }
                        continue 'scan;
                    } else {
                        break 'run_or_lit;
                    }
                }
            }
            // Emit: a literal stretch (-n, bytes) when S2 is set, then a run (count, byte).
            'emit: {
                c.cycles += 5;
                c.g[V1][0] = ((c.g[S2][0] as i64) < 128i64) as u64;
                c.g[V0][0] = 0x7f;
                if c.g[V1][0] == 0 {
                    c.g[S2][0] = c.g[V0][0];
                }
                let t = c.g[S2][0] == 0u64;
                c.g[A0][0] = c.g[S1][0];
                if t {
                    break 'emit;
                }
                c.cycles += 8;
                c.g[A1][0] = sx(0u32.wrapping_sub(c.g[S2][0] as u32));
                c.g[A2][0] = 0x1;
                c.g[S1][0] = sx(sphere_addr(c.g[S1][0], 0x1));
                call_ptr(c, 0x003e_6448, 0x0032_b578)?;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                let t = c.g[S2][0] == c.g[V0][0];
                if t {
                    break 'emit;
                }
                c.cycles += 2;
                c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                'copy: loop {
                    c.cycles += 7;
                    c.g[V0][0] = c.lbu(sphere_addr(c.g[S3][0], 0));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[S3][0] = sx(sphere_addr(c.g[S3][0], 0x1));
                    c.sb(sphere_addr(c.g[S1][0], 0), c.g[V0][0]);
                    let t = c.g[S2][0] != c.g[V1][0];
                    c.g[S1][0] = sx(sphere_addr(c.g[S1][0], 0x1));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0032_b590);
                        }
                        continue 'copy;
                    } else {
                        break 'emit;
                    }
                }
            }
            c.cycles += 2;
            let t = c.g[S0][0] == 0u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0032_b4ac);
                }
                continue 'outer;
            }
            c.cycles += 11;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 0x1;
            call_ptr(c, 0x003e_6448, 0x0032_b5c4)?;
            c.g[S1][0] = sx(sphere_addr(c.g[S1][0], 0x1));
            c.g[V0][0] = c.lbu(sphere_addr(c.g[S3][0], 0));
            c.g[S3][0] = sx(sphere_addr(c.g[S3][0], 0x1));
            c.sb(sphere_addr(c.g[S1][0], 0), c.g[V0][0]);
            c.g[S3][0] = sx(sphere_addr(c.g[S3][0], c.g[S0][0] as u32));
            c.g[S1][0] = sx(sphere_addr(c.g[S1][0], 0x1));
            if event_due(c) {
                deliver_event_here(c, 0x0032_b4ac);
            }
            continue 'outer;
        }
        // The terminating zero byte, then out - start.
        c.cycles += 16;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x1;
        c.g[S1][0] = sx(sphere_addr(c.g[S1][0], 0x1));
        call_ptr(c, 0x003e_6448, 0x0032_b5f4)?;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[S6][0] as u32));
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SphereTree_SetTransform(self, mat, scale)` (0x32c648, dev-build
/// `TransformAbs`): the four axes are the align axes (`gSphTreeAlignAxis`,
/// 0x4ff640) rotated by `mat` and scaled, their negations fill axes 4..7; the
/// centre is the rotated root centre times `scale`, and `self->scale = scale`.
pub fn sphere_tree_set_transform(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x0050_0000;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 0); // f0 = -1.0
    let frame = Frame::open(c, 0x40);
    c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x9c0));
    c.g[A3][0] = sx(sphere_addr(c.g[A0][0], 0x70));
    c.g[A2][0] = c.g[A0][0];
    c.g[T1][0] = 0x3;
    loop {
        c.cycles += 33;
        c.mfc1(2, 12);
        c.lqc2(4, (c.g[T0][0] as u32));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sphere_addr(c.g[SP][0], 0x20));
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
        c.lq(2, sphere_addr(c.g[SP][0], 0x20));
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x10));
        c.sq(2, sphere_addr(c.g[SP][0], 0x10));
        c.lqc2(8, sphere_addr(c.g[SP][0], 0x10));
        c.lqc2(4, sphere_addr(c.g[A1][0], 0));
        c.lqc2(5, sphere_addr(c.g[A1][0], 0x10));
        c.lqc2(6, sphere_addr(c.g[A1][0], 0x20));
        c.lqc2(7, sphere_addr(c.g[A1][0], 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, sphere_addr(c.g[SP][0], 0x20));
        c.lq(2, sphere_addr(c.g[SP][0], 0x20));
        c.sq(2, sphere_addr(c.g[A2][0], 0));
        c.sq(2, sphere_addr(c.g[SP][0], 0));
        c.mfc1(3, 0);
        c.lqc2(4, sphere_addr(c.g[A2][0], 0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sphere_addr(c.g[SP][0], 0x10));
        c.lq(2, sphere_addr(c.g[SP][0], 0x10));
        c.g[A2][0] = sx(sphere_addr(c.g[A2][0], 0x10));
        c.sq(2, sphere_addr(c.g[A3][0], 0));
        c.sq(2, sphere_addr(c.g[SP][0], 0));
        let t = (c.g[T1][0] as i64) >= 0;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x10));
        if !t {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_c668);
        }
    }
    c.cycles += 32;
    c.g[V0][0] = c.lw(sphere_addr(c.g[A0][0], 0x98));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 3); // f3 = 1.0
    c.lwc1(2, sphere_addr(c.g[V0][0], 0x1c));
    c.lwc1(0, sphere_addr(c.g[V0][0], 0x14));
    c.lwc1(1, sphere_addr(c.g[V0][0], 0x18));
    c.mul_s(2, 2, 12);
    c.mul_s(0, 0, 12);
    c.swc1(3, sphere_addr(c.g[SP][0], 0x1c));
    c.mul_s(1, 1, 12);
    c.swc1(2, sphere_addr(c.g[SP][0], 0x18));
    c.swc1(0, sphere_addr(c.g[SP][0], 0x10));
    c.swc1(1, sphere_addr(c.g[SP][0], 0x14));
    c.swc1(0, sphere_addr(c.g[SP][0], 0x20));
    c.swc1(1, sphere_addr(c.g[SP][0], 0x24));
    c.swc1(2, sphere_addr(c.g[SP][0], 0x28));
    c.lqc2(8, sphere_addr(c.g[SP][0], 0x10));
    c.lqc2(4, sphere_addr(c.g[A1][0], 0));
    c.lqc2(5, sphere_addr(c.g[A1][0], 0x10));
    c.lqc2(6, sphere_addr(c.g[A1][0], 0x20));
    c.lqc2(7, sphere_addr(c.g[A1][0], 0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, sphere_addr(c.g[SP][0], 0x30));
    c.lq(2, sphere_addr(c.g[SP][0], 0x30));
    c.swc1(12, sphere_addr(c.g[A0][0], 0x90));
    c.sq(2, sphere_addr(c.g[A0][0], 0x80));
    c.sq(2, sphere_addr(c.g[SP][0], 0));
    frame.close(c);
}

/// `void SphereTree_ApplyTransform(self, mat, scale)` (0x32c770, dev-build
/// `TransformRel`): the axes and the centre are transformed again by `mat`
/// and scaled by `scale` (f12) on top of the current ones; `self->scale`
/// grows by the same factor. The axes' negations are rebuilt as in
/// [`sphere_tree_set_transform`].
pub fn sphere_tree_apply_transform(c: &mut Cpu) {
    c.cycles += 6;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 0); // f0 = -1.0
    let frame = Frame::open(c, 0x30);
    c.g[A3][0] = sx(sphere_addr(c.g[A0][0], 0x70));
    c.g[A2][0] = c.g[A0][0];
    c.g[T0][0] = 0x3;
    loop {
        c.cycles += 32;
        c.lqc2(8, (c.g[A2][0] as u32));
        c.lqc2(4, (c.g[A1][0] as u32));
        c.lqc2(5, sphere_addr(c.g[A1][0], 0x10));
        c.lqc2(6, sphere_addr(c.g[A1][0], 0x20));
        c.lqc2(7, sphere_addr(c.g[A1][0], 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, sphere_addr(c.g[SP][0], 0x20));
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
        c.lq(2, sphere_addr(c.g[SP][0], 0x20));
        c.sq(2, sphere_addr(c.g[SP][0], 0x10));
        c.mfc1(3, 12);
        c.lqc2(4, sphere_addr(c.g[SP][0], 0x10));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sphere_addr(c.g[SP][0], 0x20));
        c.lq(2, sphere_addr(c.g[SP][0], 0x20));
        c.sq(2, sphere_addr(c.g[A2][0], 0));
        c.sq(2, sphere_addr(c.g[SP][0], 0));
        c.mfc1(3, 0);
        c.lqc2(4, sphere_addr(c.g[A2][0], 0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sphere_addr(c.g[SP][0], 0x10));
        c.lq(2, sphere_addr(c.g[SP][0], 0x10));
        c.g[A2][0] = sx(sphere_addr(c.g[A2][0], 0x10));
        c.sq(2, sphere_addr(c.g[A3][0], 0));
        c.sq(2, sphere_addr(c.g[SP][0], 0));
        let t = (c.g[T0][0] as i64) >= 0;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x10));
        if !t {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_c788);
        }
    }
    c.cycles += 36;
    c.lqc2(8, sphere_addr(c.g[A0][0], 0x80));
    c.lqc2(4, (c.g[A1][0] as u32));
    c.lqc2(5, sphere_addr(c.g[A1][0], 0x10));
    c.lqc2(6, sphere_addr(c.g[A1][0], 0x20));
    c.lqc2(7, sphere_addr(c.g[A1][0], 0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, sphere_addr(c.g[SP][0], 0x20));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 1); // f1 = 1.0
    c.lq(2, sphere_addr(c.g[SP][0], 0x20));
    c.lwc1(3, sphere_addr(c.g[A0][0], 0x90));
    c.sq(2, sphere_addr(c.g[SP][0], 0x10));
    c.swc1(1, sphere_addr(c.g[SP][0], 0x2c));
    c.mul_s(3, 3, 12);
    c.lwc1(0, sphere_addr(c.g[SP][0], 0x10));
    c.lwc1(1, sphere_addr(c.g[SP][0], 0x14));
    c.lwc1(2, sphere_addr(c.g[SP][0], 0x18));
    c.mul_s(0, 0, 12);
    c.mul_s(1, 1, 12);
    c.lwc1(4, sphere_addr(c.g[SP][0], 0x1c));
    c.mul_s(2, 2, 12);
    c.swc1(3, sphere_addr(c.g[A0][0], 0x90));
    c.swc1(0, sphere_addr(c.g[SP][0], 0));
    c.swc1(1, sphere_addr(c.g[SP][0], 0x4));
    c.swc1(2, sphere_addr(c.g[SP][0], 0x8));
    c.swc1(4, sphere_addr(c.g[SP][0], 0xc));
    c.swc1(12, sphere_addr(c.g[SP][0], 0x20));
    c.lq(2, sphere_addr(c.g[SP][0], 0));
    c.swc1(12, sphere_addr(c.g[SP][0], 0x24));
    c.sq(2, sphere_addr(c.g[A0][0], 0x80));
    c.swc1(12, sphere_addr(c.g[SP][0], 0x28));
    frame.close(c);
}

/// `int SphereTree_CollidePlane(self, plane, outPush, outContact)` (0x32c9b8):
/// builds the tree's buffer when it is compressed, stores the plane's
/// zero-w copy in the frame for the tree's sort axes, resets `worstCase`
/// and walks the tree with `SphereTree_checkPlaneCollision`. Returns 1 when
/// the plane is hit (`worstCase < 0`), else 0.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn sphere_tree_collide_plane(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[A0][0] = c.lw(sphere_addr(c.g[S0][0], 0x98)); // treeInfo
        c.g[V0][0] = c.lw(sphere_addr(c.g[A0][0], 0x8)); // isCompressed
        c.g[S3][0] = c.g[A3][0];
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            call_ptr(c, 0x0032_df28, 0x0032_c9f4)?; // SphereTree_AllocBuf(treeInfo)
            c.g[V1][0] = c.lw(sphere_addr(c.g[S0][0], 0x98));
            c.sw(sphere_addr(c.g[V1][0], 0x28), c.g[V0][0]); // bitTree
        }
        c.cycles += 23;
        // The plane's copy with w cleared, as the sort axes' argument.
        c.lq(2, sphere_addr(c.g[S1][0], 0));
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        c.sq(2, sphere_addr(c.g[SP][0], 0x10));
        c.sw(sphere_addr(c.g[SP][0], 0x1c), 0);
        c.lq(2, sphere_addr(c.g[SP][0], 0x10));
        c.sq(2, sphere_addr(c.g[SP][0], 0));
        call_ptr(c, 0x0032_db40, 0x0032_ca1c)?; // SphereTree_populateSortAxis_v
        c.sw(sphere_addr(c.g[S0][0], 0x94), 0); // worstCase = 0
        c.g[A1][0] = c.g[S1][0];
        c.g[T1][0] = c.g[S2][0];
        c.g[T2][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = sx(sphere_addr(c.g[S0][0], 0x80));
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        call_ptr(c, 0x0032_d208, 0x0032_ca40)?; // SphereTree_checkPlaneCollision
        c.lwc1(1, sphere_addr(c.g[S0][0], 0x94));
        c.mtc1(0, 0);
        c.c_lt_s(1, 0);
        let t = c.fcc();
        c.g[V0][0] = 0x1;
        if !t {
            c.cycles += 1;
            c.g[V0][0] = 0u64;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    /// The preprocessor (self) and the source cursor (src) of the comment
    /// test, and the text the source holds.
    const PP: u32 = DATA;
    const SRC: u32 = DATA + 0x100;
    const TEXT: u32 = DATA + 0x500;
    /// The RLE test's input and output.
    const RLE_IN: u32 = DATA + 0x800;
    const RLE_OUT: u32 = DATA + 0x1000;
    /// The gSphTreeAlignAxis table the transform reads (0x4ff640 .. +0x40).
    const ALIGN_AXIS: u32 = 0x004f_f640;

    fn skip_comment_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = sx(PP);
        c.g[A1][0] = sx(SRC);
        c.g[A2][0] = r.next_u32() as u64;
        c.g[A3][0] = r.next_u32() as u64;
        // Output: count, capacity, buffer (the real Write runs on them).
        c.bus.write32(PP, 0);
        c.bus.write32(PP + 4, 0x100);
        c.bus.write32(PP + 8, PP + 0x400);
        // CurrentChar: the comment's first character (or end of input).
        let first = [b'*', b'\n', b'a', b'*'][(seed % 4) as usize];
        c.bus.write32(
            PP + 0xc,
            if seed % 11 == 0 {
                u32::MAX
            } else {
                first as u32
            },
        );
        // The text: random characters, then "*/" in most seeds.
        let alphabet = [b'a', b'\n', b'*', b'/', b' '];
        let n = r.next_u32() % 10;
        let mut text: Vec<u8> = (0..n)
            .map(|_| alphabet[(r.next_u32() % 5) as usize])
            .collect();
        if seed % 3 != 0 {
            text.extend_from_slice(b"*/");
        }
        for (i, b) in text.iter().enumerate() {
            c.bus.write8(TEXT + i as u32, *b);
        }
        // The source cursor: base, length, position, line number.
        c.bus.write32(SRC, TEXT);
        c.bus.write32(SRC + 4, text.len() as u32);
        c.bus.write32(SRC + 8, TEXT);
        c.bus.write32(SRC + 0xc, r.next_u32() % 1000);
    }

    #[test]
    fn input_preproc_skip_block_comment_matches() {
        check(
            ee::s10::f_003264b0__generated,
            input_preproc_skip_block_comment,
            300,
            &|c, seed| skip_comment_setup(c, seed),
        );
    }

    fn rle_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x7e1c_0de5_0000_0001 ^ (seed as u64) << 8);
        scramble(c, &mut r);
        let len = r.next_u32() % 49;
        let min_run = r.next_u32() % 6;
        c.g[A0][0] = sx(RLE_OUT);
        c.g[A1][0] = sx(RLE_IN);
        c.g[A2][0] = len as u64;
        c.g[A3][0] = min_run as u64;
        // Bytes in runs of a few values, so both kinds of output occur.
        let mut byte = 0u8;
        for i in 0..len {
            if r.next_u32() % 3 == 0 {
                byte = (r.next_u32() % 3) as u8;
            }
            c.bus.write8(RLE_IN + i, byte);
        }
    }

    #[test]
    fn rle_encode_matches() {
        check(
            ee::s10::f_0032b468__generated,
            rle_encode,
            300,
            &|c, seed| rle_setup(c, seed),
        );
    }

    /// The transform's self (axes, centre, scale), its tree info (root centre)
    /// and the matrix at DATA+0x100.
    fn set_transform_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x5e7_0c64_0000_0001 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DATA + 0x100);
        c.f[12] = r.edge_f32();
        for i in 0..16u32 {
            c.bus.write32(ALIGN_AXIS + 4 * i, r.f32(2.0));
        }
        for i in 0..16u32 {
            c.bus.write32(DATA + 0x100 + 4 * i, r.f32(4.0));
        }
        let info = DATA + 0x200;
        c.bus.write32(DATA + 0x98, info);
        for i in 0..3u32 {
            c.bus.write32(info + 0x14 + 4 * i, r.f32(100.0));
        }
    }

    #[test]
    fn sphere_tree_set_transform_matches() {
        check(
            ee::s10::f_0032c648__generated,
            sphere_tree_set_transform,
            300,
            &|c, seed| set_transform_setup(c, seed),
        );
    }

    /// The transform's self (axes, centre at +0x80, scale at +0x90) and the matrix.
    fn apply_transform_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x0a99_17e5_0000_0001 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DATA + 0x100);
        c.f[12] = r.edge_f32();
        for i in 0..0x26u32 {
            c.bus.write32(DATA + 4 * i, r.f32(4.0));
        }
        c.bus.write32(DATA + 0x90, r.f32(2.0));
        for i in 0..16u32 {
            c.bus.write32(DATA + 0x100 + 4 * i, r.f32(4.0));
        }
    }

    #[test]
    fn sphere_tree_apply_transform_matches() {
        check(
            ee::s10::f_0032c770__generated,
            sphere_tree_apply_transform,
            300,
            &|c, seed| apply_transform_setup(c, seed),
        );
    }

    /// A leaf tree (depth 0, so `checkPlaneCollision` takes its leaf path),
    /// with the plane and the centre random. Odd seeds have a compressed tree,
    /// so `SphereTree_AllocBuf` runs on the zeroed cache.
    fn collide_plane_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x0c01_1de0_0000_0001 ^ seed as u64);
        scramble(c, &mut r);
        let this = DATA;
        let plane = DATA + 0x300;
        let tree = DATA + 0x1000;
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(plane);
        c.g[A2][0] = sx(DATA + 0x400);
        c.g[A3][0] = sx(DATA + 0x420);
        for i in 0..4u32 {
            c.bus.write32(plane + 4 * i, r.f32(2.0));
            c.bus.write32(this + 0x80 + 4 * i, r.f32(3.0));
        }
        c.bus.write32(this + 0x90, r.f32(1.5));
        c.bus.write32(this + 0x98, tree);
        c.bus.write32(tree + 0x8, (seed % 2) as u32); // isCompressed
        c.bus.write32(tree + 0xc, 0); // depth
        c.bus.write32(tree + 0x20, tree + 0x100); // levels
        c.bus.write32(tree + 0x100, r.f32(2.0)); // levels[0].radius
        c.bus.write32(tree + 0x28, tree + 0x200); // bitTree
    }
}
