//! Staging module oc1053 (wave wave264): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1053`.

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
    (0x002d_81b0, active_avalanche_end, "Avalanche_Stop"),
    (
        0x002d_8258,
        avalanche_export_history_files,
        "Avalanche_ExportHistoryFiles",
    ),
    (
        0x002d_87d0,
        avalanche_load_history_file,
        "Avalanche_LoadHistoryFile",
    ),
];

// ---------------------------------------------------------------------------
// `Avalanche_UpdateInstance` (0x2d7ca8): advance one active avalanche node by
// `t` (scaled by the data block's rate), sample its history, and either copy
// the frame into the node and emit particles, or end the node. Returns in V0
// whether the node is still alive.

/// `tActiveAvalancheNode` fields.
const UPD_DATA: u32 = 0x2e0;
const UPD_FB4: u32 = 0xb4;
const UPD_CUR_POS: u32 = 0x60;
const UPD_CUR_MAT: u32 = 0x70;
const UPD_CUR_COLOUR: u32 = 0xc0;
const UPD_EMITTER: u32 = 0xd0;
const UPD_F1FC: u32 = 0x1fc;
/// `sAvData` fields: length, rate, type (u16), emits (u8).
const UPD_D_LENGTH: u32 = 0x100;
const UPD_D_RATE: u32 = 0x10c;
const UPD_D_TYPE: u32 = 0xf0;
const UPD_D_EMITS: u32 = 0xf2;
/// The sampled frame in the caller's stack: position, colour, matrix.
const UPD_OUT_COLOUR: u32 = 0x10;
const UPD_OUT_MAT: u32 = 0x20;

/// `bool Avalanche_UpdateInstance(self, float t)` (0x2d7ca8).
#[allow(dead_code)]
pub fn avalanche_update_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x70);
        frame.save_fpr(c, 20, 0x80);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x60);
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(UPD_DATA)).lw(c);
        let has_data = c.g[V0][0] != 0;
        c.mov_s(20, 12);
        if has_data {
            upd_sample_and_emit(c)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x70);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// The body for an instance with data (V0 = its data block on entry): sample
/// the history at `t * rate`; past the length, end the node; otherwise copy
/// the sampled frame into the node and emit (or end the node when it emits
/// nothing).
fn upd_sample_and_emit(c: &mut Cpu) -> Flow {
    c.cycles += 15;
    let data = r32(c, V0);
    c.lwc1(0, data.wrapping_add(UPD_D_RATE));
    c.g[A0][0] = sx(sp(c));
    c.g[A1][0] = sx(r32(c, S0));
    c.mul_s(20, 20, 0);
    c.mov_s(12, 20);
    Callee {
        addr: 0x002d_5778,
        f: ee::s09::Avalanche_SampleHistory,
    }
    .call(c, 0x002d_7ce8)?;
    c.g[AT][0] = 0x41f0_0000; // 30.0
    c.mtc1(AT, 0);
    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(UPD_DATA)).lw(c);
    c.mul_s(0, 20, 0);
    let length = r32(c, V0).wrapping_add(UPD_D_LENGTH);
    c.lwc1(1, length);
    c.c_lt_s(0, 1);
    let inside = c.fcc();
    c.g[A1][0] = sx(r32(c, S0).wrapping_add(UPD_CUR_POS));
    if !inside {
        return upd_past_end(c);
    }
    // Copy the sampled frame into the node: position (v0), matrix, colour.
    c.cycles += 19;
    c.lq(2, sp(c));
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(UPD_CUR_MAT));
    let out_mat = sp(c).wrapping_add(UPD_OUT_MAT);
    c.g[V1][0] = sx(out_mat);
    c.sq(2, r32(c, S0).wrapping_add(UPD_CUR_POS));
    c.lqc2(1, out_mat);
    c.lqc2(2, out_mat.wrapping_add(0x10));
    c.lqc2(3, out_mat.wrapping_add(0x20));
    c.lqc2(4, out_mat.wrapping_add(0x30));
    let cur_mat = r32(c, A0);
    c.sqc2(1, cur_mat);
    c.sqc2(2, cur_mat.wrapping_add(0x10));
    c.sqc2(3, cur_mat.wrapping_add(0x20));
    c.sqc2(4, cur_mat.wrapping_add(0x30));
    c.lq(2, sp(c).wrapping_add(UPD_OUT_COLOUR));
    c.g[T1][0] = Addr(r32(c, S0).wrapping_add(UPD_DATA)).lw(c);
    c.g[A0][0] = 2;
    c.sq(2, r32(c, S0).wrapping_add(UPD_CUR_COLOUR));
    c.g[V1][0] = Addr(r32(c, T1).wrapping_add(UPD_D_TYPE)).lhu(c);
    let typed = c.g[V1][0] == c.g[A0][0];
    c.g[A3][0] = sx(r32(c, S0).wrapping_add(UPD_CUR_COLOUR));
    if typed {
        return upd_alive(c);
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, T1).wrapping_add(UPD_D_EMITS)).lbu(c);
    let silent = c.g[V0][0] == 0;
    c.g[V0][0] = 0x0050_0000;
    if silent {
        return upd_alive(c);
    }
    upd_emit(c)
}

/// The sample is past the instance's length: a type-2 instance stays alive,
/// any other one is released and ends (V0 = 0).
fn upd_past_end(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(UPD_D_TYPE)).lhu(c);
    c.g[V0][0] = 2;
    let typed = c.g[V1][0] == c.g[V0][0];
    c.g[A0][0] = sx(r32(c, S0));
    if typed {
        return upd_alive(c);
    }
    c.cycles += 4;
    c.g[A1][0] = 0;
    Callee {
        addr: 0x002d_7dd8,
        f: ee::s09::Avalanche_ReleaseInstance,
    }
    .call(c, 0x002d_7db8)?;
    c.g[V0][0] = 0;
    Ok(())
}

/// The node stays alive (V0 = 1) after its frame was copied.
fn upd_alive(c: &mut Cpu) -> Flow {
    c.cycles += 1;
    c.g[V0][0] = 1;
    Ok(())
}

/// Emit particles from the node's emitter at the sampled position (the
/// `DynamicEmitter` call through the function table); the node stays alive.
fn upd_emit(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    c.lwc1(0, r32(c, S0).wrapping_add(UPD_FB4));
    c.g[A2][0] = sx(r32(c, V0).wrapping_sub(0xee0));
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(UPD_EMITTER));
    c.swc1(0, r32(c, S0).wrapping_add(UPD_F1FC));
    c.lwc1(0, r32(c, T1).wrapping_add(UPD_D_LENGTH));
    c.c_lt_s(20, 0);
    let before_end = c.fcc();
    c.g[T0][0] = 1;
    if !before_end {
        c.cycles += 1;
        c.g[T0][0] = 0;
    }
    c.cycles += 5;
    c.lwc1(12, r32(c, GP).wrapping_sub(0x3cd4));
    call_ptr(c, 0x0037_17c0, 0x002d_7d98)?;
    c.g[V0][0] = 1;
    Ok(())
}

// ---------------------------------------------------------------------------
// `Avalanche_Stop` (0x2d81b0): free the spawn buffer, release every live
// instance, stop the avalanche sound and count the danger manager's active
// avalanches down.

/// `void Avalanche_Stop(self)` (0x2d81b0).
pub fn active_avalanche_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x8)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        } else {
            c.cycles += 4;
            call_ptr(c, 0x0031_7e98, 0x002d_81d8)?;
            Addr(r32(c, S1).wrapping_add(0x8)).sw(c, 0);
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        }
        c.cycles += 2;
        let ended = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !ended {
            stop_end_live(c)?;
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The `*(int*)self != 0` branch of `Avalanche_Stop`: release the instance
/// list at +4, clear the flag, stop the sound and count down the danger
/// manager's active avalanches.
fn stop_end_live(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[S0][0] = Addr(r32(c, S1).wrapping_add(0x4)).lw(c);
    let empty = c.g[S0][0] == 0;
    c.g[A0][0] = c.g[S0][0];
    if !empty {
        c.cycles += 1;
        loop {
            c.cycles += 5;
            c.g[A1][0] = 1;
            Callee {
                addr: 0x002d_7dd8,
                f: ee::s09::Avalanche_ReleaseInstance,
            }
            .call(c, 0x002d_8200)?;
            c.g[S0][0] = Addr(r32(c, S0).wrapping_add(0x2e4)).lw(c);
            let more = c.g[S0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_81f8);
            }
        }
    }
    c.cycles += 12;
    Addr(r32(c, S1)).sw(c, 0);
    call_ptr(c, 0x0028_b180, 0x002d_8214)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0029_def0, 0x002d_8220)?;
    c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0xc)).lw(c);
    c.g[V1][0] = Addr(r32(c, A0).wrapping_add(0xa8)).lw(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x708)).lw(c);
    let zero = c.g[V0][0] == 0;
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
    if !zero {
        c.cycles += 1;
        let count = c.g[V0][0];
        Addr(r32(c, V1).wrapping_add(0x708)).sw(c, count);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// `Avalanche_ExportHistoryFiles` (0x2d8258): write the three history files.

/// `void Avalanche_ExportHistoryFiles(void)` (0x2d8258).
pub fn avalanche_export_history_files(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(0x004a_3ad0);
        c.g[A0][0] = 1;
        Callee {
            addr: 0x002d_83b8,
            f: ee::s09::Avalanche_WriteHistoryFile,
        }
        .call(c, 0x002d_8270)?;
        c.g[A1][0] = sx(0x004a_3ad8);
        c.g[A0][0] = 1;
        Callee {
            addr: 0x002d_83b8,
            f: ee::s09::Avalanche_WriteHistoryFile,
        }
        .call(c, 0x002d_8280)?;
        c.g[A1][0] = sx(0x004a_3ae0);
        c.g[A0][0] = 0;
        Callee {
            addr: 0x002d_83b8,
            f: ee::s09::Avalanche_WriteHistoryFile,
        }
        .call(c, 0x002d_8290)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `Avalanche_WriteData` (0x2d82a0): write `rows` rows of `stride` bytes. A
// direct write goes out as it is; otherwise each row is reversed into a
// buffer first. Returns `stride * rows`.

/// `int Avalanche_WriteData(direct, handle, offset, src, stride, rows)`
/// (0x2d82a0).
#[allow(dead_code)]
pub fn avalanche_write_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[T1][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[T0][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S2, 0x40);
        c.g[S5][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        if c.g[A0][0] != 0 {
            c.cycles += 9;
            c.mult(0, 0, S3, S1);
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S5][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.lo[0];
            c.g[T0][0] = 0x64;
            call_ptr(c, 0x003d_ec80, 0x002d_82f4)?;
            c.mult(0, 2, S3, S1);
        } else {
            wd_buffered(c)?;
        }
        // The epilogue (`'b_2d8394`).
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The buffered branch of `Avalanche_WriteData`: copy every row reversed
/// into a buffer, write the buffer, and free it. Leaves V0 = stride * rows.
fn wd_buffered(c: &mut Cpu) -> Flow {
    c.cycles += 11;
    c.mult(0, 0, S3, S1);
    c.g[A1][0] = sx(0x004a_3ae8);
    c.g[A2][0] = 0;
    c.g[A0][0] = c.lo[0];
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7e30, 0x002d_8318)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[V1][0] = 0;
    let no_rows = (c.g[S1][0] as i64) <= 0;
    c.g[A1][0] = c.g[S2][0];
    if !no_rows {
        loop {
            // One row: the bytes from `end` back to `src`, into the buffer.
            c.cycles += 4;
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, S3)));
            c.g[A2][0] = sx(r32(c, V1).wrapping_add(1));
            let empty_row = c.g[S0][0] == c.g[A0][0];
            c.g[V1][0] = c.g[A0][0];
            if !empty_row {
                loop {
                    c.cycles += 7;
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                    c.g[V0][0] = Addr(r32(c, V1)).lbu(c);
                    let byte = c.g[V0][0];
                    Addr(r32(c, A1)).sb(c, byte);
                    let more = c.g[S0][0] != c.g[V1][0];
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_8338);
                    }
                }
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[A2][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
            let more_rows = c.g[V0][0] != 0;
            c.g[S0][0] = c.g[A0][0];
            if !more_rows {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_8328);
            }
        }
    }
    c.cycles += 9;
    c.mult(0, 0, S3, S1);
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S5][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A3][0] = c.lo[0];
    c.g[T0][0] = 0x64;
    call_ptr(c, 0x003d_ec80, 0x002d_8380)?;
    let no_buf = c.g[S2][0] == 0;
    c.mult(0, 2, S3, S1);
    if !no_buf {
        c.cycles += 3;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0031_7e98, 0x002d_8390)?;
        c.mult(0, 2, S3, S1);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// `Avalanche_LoadHistoryFile` (0x2d87d0): open the history file of the
// current location, read its header, and load the history when the magic
// matches; then apply the loaded history.

/// `void Avalanche_LoadHistoryFile(void)` (0x2d87d0).
pub fn avalanche_load_history_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x140);
        frame.save(c, S0, 0x130);
        frame.save(c, S1, 0x120);
        frame.save_ra(c, 0x110);
        c.g[S0][0] = 0x004a_0000;
        call_ptr(c, 0x0014_dc80, 0x002d_87e8)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x002d_87f4)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x3af0));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0014_4be0, 0x002d_8804)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0014_4c48, 0x002d_8810)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = sx(0x004a_3ad8);
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x0041_7828, 0x002d_8828)?;
        c.g[V1][0] = 1;
        Addr(r32(c, GP).wrapping_add(0x9cc)).sw(c, 0);
        let one = c.g[V1][0];
        Addr(r32(c, GP).wrapping_add(0x9d4)).sw(c, one);
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 1;
        c.g[A2][0] = 0x64;
        c.g[A3][0] = sx(sp(c).wrapping_add(0x100));
        call_ptr(c, 0x003d_ebf0, 0x002d_8848)?;
        if c.g[V0][0] != 0 {
            lhf_read_file(c)?;
        }
        // The tail (`'b_2d892c`): the history is applied either way.
        c.cycles += 7;
        Addr(r32(c, GP).wrapping_add(0x95c)).sw(c, 0);
        Callee {
            addr: 0x002d_a028,
            f: ee::s09::Avalanche_ApplyHistoryData,
        }
        .call(c, 0x002d_8934)?;
        frame.restore(c, S0, 0x130);
        frame.restore(c, S1, 0x120);
        frame.restore_ra(c, 0x110);
        frame.close(c);
        Ok(())
    })
}

/// The opened-file branch of `Avalanche_LoadHistoryFile`: drop the old file
/// data when it is owned, read the 8-byte header, and when its magic matches
/// read the history into a new buffer.
fn lhf_read_file(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    Callee {
        addr: 0x002d_9a80,
        f: ee::s09::Avalanche_FreeHistories,
    }
    .call(c, 0x002d_8858)?;
    c.g[A0][0] = Addr(r32(c, GP).wrapping_add(0x9c4)).lw(c);
    let has_data = c.g[A0][0] != 0;
    c.g[V0][0] = Addr(r32(c, GP).wrapping_add(0x9c8)).lw(c);
    // The file data is freed when it exists and is owned; an unowned one
    // skips the shared tail (`break 'b_2d8878`).
    let mut tail = true;
    if has_data {
        c.cycles += 2;
        let owned = c.g[V0][0] != 0;
        c.g[A1][0] = sx(sp(c));
        if owned {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e98, 0x002d_8874)?;
        } else {
            tail = false;
        }
    }
    if tail {
        c.cycles += 1;
        c.g[A1][0] = sx(sp(c));
    }
    // Read the 8-byte header into a new buffer.
    c.cycles += 16;
    c.g[A0][0] = 8;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7e30, 0x002d_8888)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = Addr(sp(c).wrapping_add(0x100)).lw(c);
    c.g[A1][0] = 0;
    c.g[A2][0] = c.g[S0][0];
    c.g[A3][0] = 8;
    c.g[T0][0] = 0x64;
    call_ptr(c, 0x003d_ec60, 0x002d_88a4)?;
    c.g[V1][0] = 0x02be_0000 | 0xef00;
    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
    let magic_ok = c.g[V0][0] == c.g[V1][0];
    if !magic_ok {
        // Bad magic: drop the header buffer and the file data pointer.
        c.cycles += 2;
        if c.g[S0][0] == 0 {
            Addr(r32(c, GP).wrapping_add(0x9c4)).sw(c, 0);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e98, 0x002d_88c8)?;
            Addr(r32(c, GP).wrapping_add(0x9c4)).sw(c, 0);
        }
        c.cycles += 2;
        Addr(r32(c, GP).wrapping_add(0x9c8)).sw(c, 0);
        return Ok(());
    }
    // Good magic: the size is in the header; free the header buffer.
    c.cycles += 2;
    let hdr_null = c.g[S0][0] == 0;
    c.g[S1][0] = Addr(r32(c, S0).wrapping_add(0x4)).lw(c);
    if !hdr_null {
        c.cycles += 2;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_7e98, 0x002d_88e4)?;
    }
    // Allocate the history buffer, read it, and close the file.
    c.cycles += 18;
    c.g[A1][0] = c.g[SP][0];
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7e30, 0x002d_88f8)?;
    c.g[A0][0] = Addr(sp(c).wrapping_add(0x100)).lw(c);
    c.g[V1][0] = 1;
    c.g[A2][0] = c.g[V0][0];
    c.g[A3][0] = sx(r32(c, S1).wrapping_add(8));
    let one = c.g[V1][0];
    Addr(r32(c, GP).wrapping_add(0x9c8)).sw(c, one);
    c.g[A1][0] = 0;
    c.g[T0][0] = 0x64;
    let buf = c.g[V0][0];
    Addr(r32(c, GP).wrapping_add(0x9c4)).sw(c, buf);
    call_ptr(c, 0x003d_ec60, 0x002d_891c)?;
    c.g[A0][0] = Addr(sp(c).wrapping_add(0x100)).lw(c);
    c.g[A1][0] = 0x64;
    call_ptr(c, 0x003d_eca0, 0x002d_8928)?;
    Addr(r32(c, GP).wrapping_add(0x9d4)).sw(c, 0);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::machine_rt;
    use crate::difftest::fill;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    /// An active node at `DATA + 0x100` with its data block at `DATA + 0x600`.
    /// Type 2 stays alive; the length (0.01 s or 1000 s) decides whether the
    /// sample is past the end; `emits` decides whether the node emits.
    fn upd_fixture(c: &mut Cpu, seed: u32) {
        let this = DATA + 0x100;
        let data = DATA + 0x600;
        c.g[A0][0] = this as u64;
        c.bus
            .write32(this + UPD_DATA, if seed % 11 == 0 { 0 } else { data });
        c.bus
            .write16(data + UPD_D_TYPE, if seed % 3 == 0 { 2 } else { 1 });
        c.bus.write8(data + UPD_D_EMITS, (seed % 2) as u8);
        let length: f32 = if seed % 4 == 0 { 0.01 } else { 1000.0 };
        c.bus.write32(data + UPD_D_LENGTH, length.to_bits());
        c.bus.write32(data + UPD_D_RATE, 1.0f32.to_bits());
        c.bus.write32(data + 0xfc, data + 0x200);
        c.f[12] = match seed % 5 {
            0 => 0.5f32.to_bits(),
            1 => 0.02f32.to_bits(),
            _ => 1.0f32.to_bits(),
        };
    }

    /// The world, the audio system and one active node (`DATA + 0x100`) with
    /// up to two instances on its list, for `Avalanche_Stop`.
    fn stop_fixture(c: &mut Cpu, seed: u32) {
        let aud = DATA + 0x1_0000;
        c.bus.write32(GAME_GP + 0x410, aud);
        c.bus.write32(aud + 0x6040, 5);
        let wld = DATA + 0x2_0000;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), wld);
        c.bus.write32(wld + 0x84, wld + 0x100);
        c.bus.write32(wld + 0x100 + 0xc, wld + 0x200);
        c.bus.write32(wld + 0x200 + 0xa8, wld + 0x300);
        c.bus.write32(wld + 0x300 + 0x708, 3);
        let this = DATA + 0x100;
        c.g[A0][0] = this as u64;
        c.bus.write32(this, if seed % 4 == 0 { 0 } else { 1 });
        c.bus.write32(this + 8, 0);
        let first = DATA + 0x400;
        let second = DATA + 0x500;
        let count = seed % 3;
        c.bus.write32(this + 4, if count == 0 { 0 } else { first });
        if count >= 1 {
            // Dead instances (no data): ReleaseInstance returns at once.
            c.bus.write32(first + 0x2e0, 0);
            c.bus
                .write32(first + 0x2e4, if count == 2 { second } else { 0 });
        }
        if count == 2 {
            c.bus.write32(second + 0x2e0, 0);
            c.bus.write32(second + 0x2e4, 0);
        }
    }

    #[test]
    fn active_avalanche_end_matches() {
        check(
            ee::s09::f_002d81b0__generated,
            active_avalanche_end,
            120,
            &|c, seed| {
                let _r = machine_rt(c, seed, 0x2d81b0, 3);
                elf_image(c);
                stop_fixture(c, seed);
            },
        );
    }

    #[test]
    fn avalanche_export_history_files_matches() {
        check(
            ee::s09::f_002d8258__generated,
            avalanche_export_history_files,
            40,
            &|c, seed| {
                let _r = machine_rt(c, seed, 0x2d8258, 2);
                elf_image(c);
            },
        );
    }

    #[test]
    fn avalanche_load_history_file_matches() {
        check(
            ee::s09::f_002d87d0__generated,
            avalanche_load_history_file,
            60,
            &|c, seed| {
                let _r = machine_rt(c, seed, 0x2d87d0, 2);
                elf_image(c);
                c.bus.write32(GAME_GP + 0x9c4, 0);
                c.bus.write32(GAME_GP + 0x9c8, seed % 2);
            },
        );
    }
}
