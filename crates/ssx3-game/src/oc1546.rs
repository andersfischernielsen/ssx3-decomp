//! Staging module oc1546 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1546`.

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
        0x0026_9008,
        csetupman_client_recv_msg1,
        "cSetupMan_ClientRecvMsg1",
    ),
    (
        0x0026_90c8,
        csetupman_client_recv_msg3,
        "cSetupMan_ClientRecvMsg3",
    ),
    (
        0x0026_9478,
        csetupman_host_recv_msg7,
        "cSetupMan_HostRecvMsg7",
    ),
    (
        0x0026_96f8,
        csetupman_host_recv_msg13,
        "cSetupMan_HostRecvMsg13",
    ),
    (
        0x0026_97d0,
        csetupman_client_recv_msg6,
        "cSetupMan_ClientRecvMsg6",
    ),
    (
        0x0026_c998,
        playbackman_is_file_loaded,
        "PlaybackMan_IsFileLoaded",
    ),
    (
        0x0026_c9f0,
        playbackman_start_file_load,
        "PlaybackMan_StartFileLoad",
    ),
    (0x0028_38e8, scriptmovie_open, "ScriptMovie_Open"),
];

/// `cSetupMan_ClientRecvMsg1` (0x269008): builds the first client message of type
/// 0xf3. Stores `a2` into the object at `a0`, sets up the local character
/// (`cSetupMan_SetLocalCharacter`), allocates the 8-byte message and the 0xc-byte
/// list node, and links the node into the list at `*(gp-0xc8)`.
pub fn csetupman_client_recv_msg1(c: &mut Cpu) {
    guest(c, |c| {
        recv_message_body(c, 0xf3, [0x0026_9028, 0x0026_9044, 0x0026_9078])
    });
}

/// `cSetupMan_ClientRecvMsg3` (0x2690c8): as `ClientRecvMsg1`, message type 0xf4.
pub fn csetupman_client_recv_msg3(c: &mut Cpu) {
    guest(c, |c| {
        recv_message_body(c, 0xf4, [0x0026_90e8, 0x0026_9104, 0x0026_9138])
    });
}

/// Body shared by `ClientRecvMsg1` and `ClientRecvMsg3`: the same registers and
/// frame, only the message type (`s0`) and the return addresses differ. `rets` are
/// the returns after `SetLocalCharacter`, the 8-byte allocation and the 0xc-byte
/// allocation.
fn recv_message_body(c: &mut Cpu, msg_type: u64, rets: [u32; 3]) -> Flow {
    c.cycles += 32;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S0, 0x50);
    frame.save(c, S1, 0x40);
    c.g[S0][0] = msg_type;
    frame.save(c, S2, 0x30);
    frame.save_ra(c, 0x20);
    let obj = c.g[A0][0] as u32;
    let value = c.g[A2][0];
    c.sw(obj, value);
    Callee {
        addr: 0x0026_4cf0,
        f: ee::s07::f_00264cf0,
    }
    .call(c, rets[0])?;
    c.g[A1][0] = 0x0048_0000;
    c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
    c.g[A0][0] = 0x8;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, rets[1])?;
    c.g[S2][0] = c.g[V0][0];
    c.g[A1][0] = 0x0048_0000;
    c.g[V0][0] = 0x0048_0000;
    let (dst, val) = (c.g[S2][0] as u32, c.g[S0][0]);
    c.sw(dst, val);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
    let (dst, val) = ((c.g[S2][0] as u32).wrapping_add(4), c.g[V0][0]);
    c.sw(dst, val);
    c.g[A0][0] = 0xc;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xf0));
    let (dst, val) = (frame.at(c, 0x10), c.g[V0][0]);
    c.sw(dst, val);
    call_ptr(c, 0x0031_7d70, rets[2])?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
    if c.g[V0][0] != 0 {
        let (dst, val) = ((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S2][0]);
        c.sw(dst, val);
    }
    // Epilogue.
    c.cycles += 15;
    let at = frame.at(c, 0x10);
    c.g[V0][0] = c.lw(at);
    frame.restore(c, S0, 0x50);
    let (dst, val) = (c.g[A1][0] as u32, c.g[V0][0]);
    c.sw(dst, val);
    frame.restore(c, S1, 0x40);
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
    frame.restore(c, S2, 0x30);
    let (dst, val) = ((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
    c.sw(dst, val);
    frame.restore_ra(c, 0x20);
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
    let (dst, val) = (c.g[A0][0] as u32, c.g[A1][0]);
    c.sw(dst, val);
    let at = frame.at(c, 0x10);
    c.g[V0][0] = c.lw(at);
    let (dst, val) = ((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
    c.sw(dst, val);
    let (dst, val) = (sp(c), c.g[A1][0]);
    c.sw(dst, val);
    frame.close(c);
    Ok(())
}

/// `cSetupMan_HostRecvMsg7` (0x269478): the row message of type 0xfb. Allocates the
/// 0x10-byte message for row `a1`, fills it (the string at `0x481150`, the row),
/// allocates the 0xc-byte node and links it into the list at `*(gp-0xc8)`.
pub fn csetupman_host_recv_msg7(c: &mut Cpu) {
    guest(c, |c| row_message_body(c, [0x0026_94a4, 0x0026_94ec]));
}

/// `cSetupMan_ClientRecvMsg6` (0x2697d0): as `HostRecvMsg7`, with its own return
/// addresses.
pub fn csetupman_client_recv_msg6(c: &mut Cpu) {
    guest(c, |c| row_message_body(c, [0x0026_97fc, 0x0026_9844]));
}

/// Body shared by `HostRecvMsg7` and `ClientRecvMsg6`. `rets` are the returns after
/// the 0x10-byte allocation and the 0xc-byte allocation.
fn row_message_body(c: &mut Cpu, rets: [u32; 2]) -> Flow {
    c.cycles += 33;
    let frame = Frame::open(c, 0x50);
    c.g[A0][0] = 0x10;
    frame.save(c, S0, 0x40);
    c.g[A2][0] = 0x2000_0000;
    c.g[S0][0] = c.g[A1][0];
    frame.save(c, S1, 0x30);
    c.g[A1][0] = 0x0048_0000;
    frame.save_ra(c, 0x20);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf18));
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, rets[0])?;
    c.g[S1][0] = c.g[V0][0];
    c.g[V1][0] = 0xfb;
    c.g[V0][0] = 0x0048_0000;
    let (dst, val) = (c.g[S1][0] as u32, c.g[V1][0]);
    c.sw(dst, val);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1150));
    c.g[A0][0] = 0x1;
    let (dst, val) = ((c.g[S1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
    c.sw(dst, val);
    c.g[A1][0] = 0x0048_0000;
    let (dst, val) = ((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.sw(dst, val);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
    let (dst, val) = ((c.g[S1][0] as u32).wrapping_add(0xc), c.g[S0][0]);
    c.sw(dst, val);
    c.g[A0][0] = 0xc;
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xf0));
    let (dst, val) = (frame.at(c, 0x10), c.g[V0][0]);
    c.sw(dst, val);
    call_ptr(c, 0x0031_7d70, rets[1])?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
    if c.g[V0][0] != 0 {
        let (dst, val) = ((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S1][0]);
        c.sw(dst, val);
    }
    // Epilogue.
    c.cycles += 14;
    let at = frame.at(c, 0x10);
    c.g[V0][0] = c.lw(at);
    frame.restore(c, S0, 0x40);
    let (dst, val) = (c.g[A1][0] as u32, c.g[V0][0]);
    c.sw(dst, val);
    frame.restore(c, S1, 0x30);
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
    frame.restore_ra(c, 0x20);
    let (dst, val) = ((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
    c.sw(dst, val);
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
    let (dst, val) = (c.g[A0][0] as u32, c.g[A1][0]);
    c.sw(dst, val);
    let at = frame.at(c, 0x10);
    c.g[V0][0] = c.lw(at);
    let (dst, val) = ((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
    c.sw(dst, val);
    let (dst, val) = (sp(c), c.g[A1][0]);
    c.sw(dst, val);
    frame.close(c);
    Ok(())
}

/// `cSetupMan_HostRecvMsg13` (0x2696f8): the host's message 13. Reads the message
/// state at `a0` (a state of 8 becomes 7), sends the character message
/// (`cSetupMan_SendCharacterMsg`), then allocates the 0x10-byte message and the
/// 0xc-byte node and links the node into the list at `*(gp-0xc8)`.
pub fn csetupman_host_recv_msg13(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = 0x8;
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        let obj = c.g[A0][0] as u32;
        c.g[V0][0] = c.lw(obj);
        if c.g[V0][0] != c.g[V1][0] {
            c.g[A2][0] = c.lw(obj);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x7;
            let val = c.g[V0][0];
            c.sw(obj, val);
            c.g[A2][0] = c.lw(obj);
        }
        c.cycles += 30;
        c.g[A1][0] = 0xc;
        c.g[A3][0] = 0;
        Callee {
            addr: 0x0026_4e50,
            f: ee::s07::f_00264e50,
        }
        .call(c, 0x0026_972c)?;
        c.g[A1][0] = 0x0048_0000;
        c.g[A0][0] = 0xc;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf18));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_9744)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = 0xfe;
        c.g[V0][0] = 0x0048_0000;
        let (dst, val) = (c.g[S0][0] as u32, c.g[V1][0]);
        c.sw(dst, val);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x13c0));
        c.g[A0][0] = 0x1;
        let (dst, val) = ((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
        c.sw(dst, val);
        c.g[A1][0] = 0x0048_0000;
        let (dst, val) = ((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.sw(dst, val);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xf0));
        let (dst, val) = (frame.at(c, 0x10), c.g[V0][0]);
        c.sw(dst, val);
        call_ptr(c, 0x0031_7d70, 0x0026_9788)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
        if c.g[V0][0] != 0 {
            let (dst, val) = ((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S0][0]);
            c.sw(dst, val);
        }
        // Epilogue.
        c.cycles += 13;
        let at = frame.at(c, 0x10);
        c.g[V0][0] = c.lw(at);
        frame.restore(c, S0, 0x30);
        let (dst, val) = (c.g[A1][0] as u32, c.g[V0][0]);
        c.sw(dst, val);
        frame.restore_ra(c, 0x20);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        let (dst, val) = ((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        c.sw(dst, val);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        let (dst, val) = (c.g[A0][0] as u32, c.g[A1][0]);
        c.sw(dst, val);
        let at = frame.at(c, 0x10);
        c.g[V0][0] = c.lw(at);
        let (dst, val) = ((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
        c.sw(dst, val);
        let (dst, val) = (sp(c), c.g[A1][0]);
        c.sw(dst, val);
        frame.close(c);
        Ok(())
    });
}

/// `PlaybackMan_IsFileLoaded(self, file)` (0x26c998): finds the file's entry with
/// `PlaybackMan_FindFile`. A missing entry (negative index) gives 0. Otherwise the
/// 12-byte entry at `self+0x58` is consulted: a non-zero flag word gives 1; a zero
/// flag gives 1 only when `self+0x78` is negative, else 0.
pub fn playbackman_is_file_loaded(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0x00);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0026_cfe8,
            f: ee::s07::f_0026cfe8,
        }
        .call(c, 0x0026_c9ac)?;
        c.g[V1][0] = c.g[V0][0];
        let not_found = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 0xc;
        if not_found {
            c.cycles += 1;
            c.g[V0][0] = 0;
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
            c.mult(0, V0, V1, V0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            let flagged = c.g[V1][0] != 0;
            c.g[V0][0] = 1;
            if flagged {
                c.cycles += 1;
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                let negative = (c.g[V1][0] as i64) < 0;
                frame.restore(c, S0, 0x10);
                if !negative {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x00);
        frame.close(c);
        Ok(())
    });
}

/// `PlaybackMan_StartFileLoad(self, file)` (0x26c9f0): finds the file's entry; a
/// missing one gives 0. For an entry with no flag, it records the index at
/// `self+0x74`, sets `self+0x78` to -1, and asks `sprintf` and the loader
/// (0x3df690) for it; the loader's result (negative: -1 kept) goes to `self+0x78`,
/// and a non-negative result returns 1. An entry already flagged returns 1.
pub fn playbackman_start_file_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0x120);
        frame.save(c, S1, 0x110);
        frame.save_ra(c, 0x100);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0026_cfe8,
            f: ee::s07::f_0026cfe8,
        }
        .call(c, 0x0026_ca08)?;
        c.g[A1][0] = c.g[V0][0];
        let not_found = (c.g[A1][0] as i64) < 0;
        c.g[V0][0] = 0xc;
        if not_found {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
            c.mult(0, V0, A1, V0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            if c.g[V1][0] != 0 {
                c.g[V0][0] = 0x1;
            } else {
                c.cycles += 16;
                c.g[S1][0] = u64::MAX;
                let slot = c.g[S0][0] as u32;
                let (dst, val) = (slot.wrapping_add(0x74), c.g[A1][0]);
                c.sw(dst, val);
                let (dst, val) = (slot.wrapping_add(0x78), c.g[S1][0]);
                c.sw(dst, val);
                c.g[A2][0] = 0x004a_0000;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x324));
                c.g[A0][0] = c.g[SP][0];
                c.g[T1][0] = c.lw(c.g[V0][0] as u32);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3430));
                c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x320));
                c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                call_ptr(c, 0x0041_7828, 0x0026_ca58)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = 0x100;
                call_ptr(c, 0x003d_f690, 0x0026_ca64)?;
                let loader_negative = (c.g[V0][0] as i64) < 0;
                let (dst, val) = (slot.wrapping_add(0x78), c.g[V0][0]);
                c.sw(dst, val);
                if loader_negative {
                    c.cycles += 1;
                    let (dst, val) = (slot.wrapping_add(0x78), c.g[S1][0]);
                    c.sw(dst, val);
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0x1;
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    });
}

/// `cVidEngine_ReadyVideo(void** self, int file)` (0x2838e8), the `ScriptMovie_Open`
/// of the scripter: if `*self` already holds a movie it returns 0; otherwise it
/// allocates a 0x44-byte movie, constructs it, stores it in `*self`, sets the
/// default options and initialises the movie with them. A zero init result drops
/// the movie again (destroyed with `cMoviePlayer` dtor, arg 3, when one was
/// stored); a non-zero result returns 1.
pub fn scriptmovie_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        let slot = c.g[S0][0] as u32;
        c.g[V0][0] = c.lw(slot);
        c.g[S1][0] = c.g[A1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 25;
            c.g[A1][0] = 0x0048_0000;
            c.g[A2][0] = 0x100;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1fc8));
            c.g[A3][0] = 0;
            c.g[A0][0] = 0x44;
            call_ptr(c, 0x0031_7d70, 0x0028_3928)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0025_33c8, 0x0028_3930)?;
            let (dst, val) = (slot, c.g[V0][0]);
            c.sw(dst, val);
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0025_3390, 0x0028_393c)?;
            c.g[A0][0] = c.lw(slot);
            c.g[V0][0] = 0x1;
            c.g[V1][0] = 0x100;
            c.g[A2][0] = 0x9;
            let base = sp(c);
            let val = c.g[S1][0];
            c.sw(base, val);
            c.g[A1][0] = c.g[SP][0];
            let val = c.g[V0][0];
            c.sw(base.wrapping_add(0x8), val);
            let val = c.g[V1][0];
            c.sw(base.wrapping_add(0x18), val);
            let val = c.g[A2][0];
            c.sw(base.wrapping_add(0x1c), val);
            c.sw(base.wrapping_add(0xc), 0);
            c.sw(base.wrapping_add(0x10), 0);
            call_ptr(c, 0x0025_34a8, 0x0028_396c)?;
            let init_ok = c.g[V0][0] != 0;
            c.g[V0][0] = 0x1;
            if !init_ok {
                c.cycles += 3;
                c.g[A0][0] = c.lw(slot);
                if c.g[A0][0] == 0 {
                    c.sw(slot, 0);
                } else {
                    c.cycles += 3;
                    c.g[A1][0] = 0x3;
                    call_ptr(c, 0x0025_3418, 0x0028_3988)?;
                    c.sw(slot, 0);
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        // Epilogue.
        c.cycles += 5;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, check_reaching};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use ssx3_ee_rt::cpu::{reg::*, sx};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's fakes (own address and magic in the machine's RAM:
    /// other tests' machines run the real bodies).
    pub(crate) const OC1546_CTRL: u32 = DATA + 0xd_0000;
    pub(crate) const OC1546_MAGIC: u32 = 0x1546_f00d;
    /// Fixture objects, clear of the gate, the call log and the stack.
    pub(crate) const OC1546_OBJ: u32 = DATA + 0xc_0000;
    /// The allocations the message fixtures hand out.
    pub(crate) const OC1546_ALLOC1: u32 = OC1546_OBJ + 0x100;
    pub(crate) const OC1546_ALLOC2: u32 = OC1546_OBJ + 0x200;

    fn oc1546_gated(c: &mut Cpu) -> bool {
        c.bus.read32(OC1546_CTRL) == OC1546_MAGIC
    }

    // Plain fakes: log the call, return the forced result or a hash.

    fn oc1546_fake_alloc(c: &mut Cpu) {
        if !oc1546_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1546_fake_alloc),
                ee::s10::f_00317d70__generated,
            );
        }
        fakes::fake_call(c, 0x0031_7d70);
    }

    fn oc1546_fake_setlocal(c: &mut Cpu) {
        if !oc1546_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1546_fake_setlocal),
                ee::s07::f_00264cf0__generated,
            );
        }
        fakes::fake_call(c, 0x0026_4cf0);
    }

    fn oc1546_fake_sendchar(c: &mut Cpu) {
        if !oc1546_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1546_fake_sendchar),
                ee::s07::f_00264e50__generated,
            );
        }
        fakes::fake_call(c, 0x0026_4e50);
    }

    fn oc1546_fake_sprintf(c: &mut Cpu) {
        if !oc1546_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1546_fake_sprintf),
                ee::s15::f_00417828__generated,
            );
        }
        fakes::fake_call(c, 0x0041_7828);
    }

    fn oc1546_fake_movie_defaults(c: &mut Cpu) {
        if !oc1546_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1546_fake_movie_defaults),
                ee::s06::f_00253390__generated,
            );
        }
        fakes::fake_call(c, 0x0025_3390);
    }

    fn oc1546_fake_movie_dtor(c: &mut Cpu) {
        if !oc1546_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1546_fake_movie_dtor),
                ee::s06::f_00253418__generated,
            );
        }
        fakes::fake_call(c, 0x0025_3418);
    }

    // Mailbox fakes: the call is logged, then `v0` is the mailbox word at
    // `CTRL + 0x10 + 4k`, sign-extended (a zero or negative result, which the call log
    // cannot force).

    fn oc1546_fake_find(c: &mut Cpu) {
        if !oc1546_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1546_fake_find),
                ee::s07::f_0026cfe8__generated,
            );
        }
        let v = c.bus.read32(OC1546_CTRL + 0x10);
        fakes::fake_call(c, 0x0026_cfe8);
        c.g[V0][0] = sx(v);
    }

    fn oc1546_fake_load(c: &mut Cpu) {
        if !oc1546_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1546_fake_load),
                ee::s14::f_003df690__generated,
            );
        }
        let v = c.bus.read32(OC1546_CTRL + 0x14);
        fakes::fake_call(c, 0x003d_f690);
        c.g[V0][0] = sx(v);
    }

    fn oc1546_fake_movie_ctor(c: &mut Cpu) {
        if !oc1546_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1546_fake_movie_ctor),
                ee::s06::f_002533c8__generated,
            );
        }
        let v = c.bus.read32(OC1546_CTRL + 0x18);
        fakes::fake_call(c, 0x0025_33c8);
        c.g[V0][0] = sx(v);
    }

    fn oc1546_fake_movie_init(c: &mut Cpu) {
        if !oc1546_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1546_fake_movie_init),
                ee::s06::f_002534a8__generated,
            );
        }
        let v = c.bus.read32(OC1546_CTRL + 0x1c);
        fakes::fake_call(c, 0x0025_34a8);
        c.g[V0][0] = sx(v);
    }

    fn oc1546_hook() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc1546_fake_alloc,
            fakes::prev_cell(oc1546_fake_alloc),
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00264CF0),
            oc1546_fake_setlocal,
            fakes::prev_cell(oc1546_fake_setlocal),
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00264E50),
            oc1546_fake_sendchar,
            fakes::prev_cell(oc1546_fake_sendchar),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            oc1546_fake_sprintf,
            fakes::prev_cell(oc1546_fake_sprintf),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00253390),
            oc1546_fake_movie_defaults,
            fakes::prev_cell(oc1546_fake_movie_defaults),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00253418),
            oc1546_fake_movie_dtor,
            fakes::prev_cell(oc1546_fake_movie_dtor),
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026CFE8),
            oc1546_fake_find,
            fakes::prev_cell(oc1546_fake_find),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF690),
            oc1546_fake_load,
            fakes::prev_cell(oc1546_fake_load),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002533C8),
            oc1546_fake_movie_ctor,
            fakes::prev_cell(oc1546_fake_movie_ctor),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002534A8),
            oc1546_fake_movie_init,
            fakes::prev_cell(oc1546_fake_movie_init),
        );
    }

    /// The machine with this module's fakes hooked and the gate on.
    fn oc1546_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc1546_hook();
        let r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(OC1546_CTRL, OC1546_MAGIC);
        r
    }

    /// Message fixture: `*(gp-0xc8)` is the list object, its `+0xf0` the node the
    /// function reads, and the node's `+4` a valid link. The allocations are the
    /// logged calls `first` and `first + 1`, returning ALLOC1 and ALLOC2; ALLOC2's
    /// `+8` is zero or not.
    fn oc1546_message_setup(c: &mut Cpu, seed: u32, first: u32, calls: u32) -> Rng {
        let mut r = oc1546_setup(c, seed, 0x1546_1, calls);
        c.g[A0][0] = OC1546_OBJ as u64;
        c.g[A1][0] = r.next_u32() as u64;
        c.g[A2][0] = r.next_u32() as u64;
        let (list, node, link) = (OC1546_OBJ + 0x400, OC1546_OBJ + 0x500, OC1546_OBJ + 0x600);
        c.bus.write32(GAME_GP - 0xc8, list);
        c.bus.write32(list + 0xf0, node);
        c.bus.write32(node + 4, link);
        c.bus.write32(fakes::LOG_RESULTS + 4 * first, OC1546_ALLOC1);
        c.bus
            .write32(fakes::LOG_RESULTS + 4 * (first + 1), OC1546_ALLOC2);
        let tail = if seed % 2 == 0 { 0 } else { r.next_u32() | 1 };
        c.bus.write32(OC1546_ALLOC2 + 8, tail);
        r
    }

    /// File fixture: `self` (a0) has a 12-byte entry table at `+0x58` whose flag
    /// words are bits of `seed`; `self+0x78` is negative or positive; the find call
    /// returns an index (-1 for none) and the loader call a negative or positive
    /// result.
    fn oc1546_file_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        let mut r = oc1546_setup(c, seed, 0x1546_2, calls);
        let obj = OC1546_OBJ;
        let table = OC1546_OBJ + 0x1000;
        c.g[A0][0] = obj as u64;
        c.g[A1][0] = r.next_u32() as u64;
        c.bus.write32(obj + 0x58, table);
        c.bus
            .write32(obj + 0x78, if seed % 3 == 0 { u32::MAX } else { 5 });
        for i in 0u32..3 {
            c.bus.write32(table + 12 * i, r.next_u32());
            c.bus.write32(table + 12 * i + 4, (seed >> i) & 1);
        }
        let idx: i32 = if seed % 5 == 0 { -1 } else { (seed % 3) as i32 };
        c.bus.write32(OC1546_CTRL + 0x10, idx as u32);
        c.bus
            .write32(OC1546_CTRL + 0x14, if seed % 2 == 0 { u32::MAX } else { 7 });
        r
    }

    /// Movie fixture: `self` (a0) is a `void**` slot; it holds a movie (the early
    /// return) for a quarter of the seeds. The allocation (logged call 0) returns
    /// ALLOC1; the constructor and the init are mailbox fakes (zero included).
    fn oc1546_movie_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        let mut r = oc1546_setup(c, seed, 0x1546_3, calls);
        let slot = OC1546_OBJ + 0x800;
        c.g[A0][0] = slot as u64;
        c.g[A1][0] = r.next_u32() as u64;
        c.bus
            .write32(slot, if seed % 4 == 0 { OC1546_OBJ + 0x900 } else { 0 });
        c.bus.write32(
            OC1546_CTRL + 0x18,
            if seed % 5 == 0 { 0 } else { OC1546_OBJ + 0xa00 },
        );
        c.bus.write32(OC1546_CTRL + 0x1c, seed % 2);
        c.bus.write32(fakes::LOG_RESULTS, OC1546_ALLOC1);
        r
    }

    #[test]
    fn csetupman_client_recv_msg1_matches() {
        check(
            ee::s07::f_00269008__generated,
            csetupman_client_recv_msg1,
            240,
            &|c, seed| {
                oc1546_message_setup(c, seed, 1, 3);
            },
        );
    }

    #[test]
    fn csetupman_client_recv_msg3_matches() {
        check(
            ee::s07::f_002690c8__generated,
            csetupman_client_recv_msg3,
            240,
            &|c, seed| {
                oc1546_message_setup(c, seed, 1, 3);
            },
        );
    }

    #[test]
    fn csetupman_host_recv_msg7_matches() {
        check(
            ee::s07::f_00269478__generated,
            csetupman_host_recv_msg7,
            240,
            &|c, seed| {
                oc1546_message_setup(c, seed, 0, 2);
            },
        );
    }

    #[test]
    fn csetupman_client_recv_msg6_matches() {
        check(
            ee::s07::f_002697d0__generated,
            csetupman_client_recv_msg6,
            240,
            &|c, seed| {
                oc1546_message_setup(c, seed, 0, 2);
            },
        );
    }

    /// The state word at `a0` is 8 for a third of the seeds (the 7 branch) and a
    /// random value for the rest.
    #[test]
    fn csetupman_host_recv_msg13_matches() {
        check(
            ee::s07::f_002696f8__generated,
            csetupman_host_recv_msg13,
            240,
            &|c, seed| {
                let mut r = oc1546_message_setup(c, seed, 1, 3);
                let state = if seed % 3 == 0 { 8 } else { r.next_u32() };
                c.bus.write32(OC1546_OBJ, state);
            },
        );
    }

    #[test]
    fn playbackman_is_file_loaded_matches() {
        check(
            ee::s07::f_0026c998__generated,
            playbackman_is_file_loaded,
            240,
            &|c, seed| {
                oc1546_file_setup(c, seed, 1);
            },
        );
    }

    #[test]
    fn playbackman_start_file_load_matches() {
        check_reaching(
            ee::s07::f_0026c9f0__generated,
            playbackman_start_file_load,
            160,
            &[0x0026_ca64],
            &|c, seed| {
                oc1546_file_setup(c, seed, 3);
            },
        );
    }

    /// The dtor path (a movie was stored, the init failed) must be reached.
    #[test]
    fn scriptmovie_open_matches() {
        check_reaching(
            ee::s07::f_002838e8__generated,
            scriptmovie_open,
            160,
            &[0x0028_3988],
            &|c, seed| {
                oc1546_movie_setup(c, seed, 5);
            },
        );
    }
}
