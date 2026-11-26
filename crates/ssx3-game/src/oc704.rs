//! Staging module oc704 (wave wave239): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc704`.

#![allow(unused_imports)]

use crate::abi;
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
        0x0030_3d90,
        wscript_fn_game_mode_matches,
        "WScriptFn_GameModeMatches",
    ),
    (
        0x0030_3e60,
        wscript_fn_event_matches,
        "WScriptFn_EventMatches",
    ),
    (
        0x0030_3ec8,
        wscript_fn_is_human_rider,
        "WScriptFn_IsHumanRider",
    ),
    (
        0x0030_3ff0,
        wscript_fn_set_pending_request_a,
        "WScriptFn_SetPendingRequestA",
    ),
    (
        0x0030_4100,
        wscript_fn_set_pending_request_b,
        "WScriptFn_SetPendingRequestB",
    ),
    (
        0x0030_4210,
        wscript_fn_set_pending_request_c,
        "WScriptFn_SetPendingRequestC",
    ),
    (
        0x0030_44a8,
        wscript_fn_game_mode_handler_v07,
        "WScriptFn_GameModeHandlerV07",
    ),
    (0x0030_4ff0, wscript_fn_nop_4ff0, "WScriptFn_Nop_4ff0"),
    (
        0x0030_4b38,
        wscript_fn_replay_highlight,
        "WScriptFn_ReplayHighlight",
    ),
];

/// `gp + off` (the game's gp-relative globals, with `off` as a wrapping offset).
fn ws_gp(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off)
}

/// Shared body of `WScriptFn_GameModeMatches` (0x303d90) and
/// `WScriptFn_EventMatches` (0x303e60): `a0 < limit` and the word at
/// `0x440000 + table` xor the byte at `0x530000 + byte` is zero. Otherwise 0.
/// Both `BE` calls return to `ret1` / `ret2`.
fn ws_mode_table_matches(c: &mut Cpu, limit: u64, table: u32, byte: u32, ret1: u32, ret2: u32) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = (c.g[S0][0] < limit) as u64;
        frame.save_ra(c, 0);
        if c.g[V0][0] != 0 {
            c.cycles += 14;
            call_ptr(c, 0x0014_dc80, ret1)?; // cBE_getBE()
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_dd58, ret2)?; // BELibrary_GetInterface(be, 0)
            c.g[V1][0] = 0x0044_0000;
            c.g[V0][0] = 0x0053_0000;
            c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(table));
            c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(byte)).lb(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
            c.g[V0][0] ^= c.g[A1][0];
            c.g[V0][0] = (c.g[V0][0] < 1) as u64;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool WScriptFn_GameModeMatches(int mode)` (0x303d90): `mode < 4` and the
/// BE's table entry `0x4465e8[mode]` matches the byte `cGamePath` (0x535c11).
pub fn wscript_fn_game_mode_matches(c: &mut Cpu) {
    ws_mode_table_matches(c, 4, 0x0000_65e8, 0x0000_5c11, 0x0030_3db8, 0x0030_3dc4)
}

/// `bool WScriptFn_EventMatches(int event)` (0x303e60): as the game-mode test
/// for `event < 15`, with the table at 0x446618 and `cGameEvent` (0x535c12).
pub fn wscript_fn_event_matches(c: &mut Cpu) {
    ws_mode_table_matches(c, 0xf, 0x0000_6618, 0x0000_5c12, 0x0030_3e88, 0x0030_3e94)
}

/// `WScriptFn_IsHumanRider(luno var self)` (0x303ec8): sets the var's type to
/// 1 and its int to a flag from `WScriptMan->Being`, asking the luno var
/// helper at 0x226610 (`castInt`) for the int's address. A being of -1 gives
/// the flag 0; otherwise a function-table call on the being's record decides
/// it (its result xor 1).
pub fn wscript_fn_is_human_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = Addr(ws_gp(c, 0xce8)).lw(c);
        frame.save(c, S1, 0x10);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(c.g[V1][0] as u32).lw(c);
        let no_being = c.g[V1][0] == c.g[V0][0];
        c.g[S1][0] = c.g[A0][0];
        if no_being {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 18;
            c.g[A0][0] = Addr(ws_gp(c, 0u32.wrapping_sub(0x848))).lw(c);
            c.g[A1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x84)).lw(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lw(c);
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
            if c.g[A0][0] == 0 {
                c.g[V1][0] = 0;
            }
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6c0));
            if c.g[V1][0] == 0 {
                c.g[A0][0] = 0;
            }
            c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x6c0)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x48)).lh(c);
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x4c)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            call_ptr(c, target, 0x0030_3f2c)?;
            c.g[V0][0] ^= 1;
        }
        c.cycles += 2;
        let zero = c.g[V0][0] == 0;
        c.g[S0][0] = 1;
        if zero {
            c.cycles += 5;
            c.g[V0][0] = 1;
            c.g[A0][0] = c.g[S1][0];
            let sv = c.g[V0][0];
            Addr((c.g[S1][0] as u32).wrapping_add(0x8)).sw(c, sv);
            call_ptr(c, 0x0022_6610, 0x0030_3f64)?;
            Addr(c.g[V0][0] as u32).sw(c, 0);
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            let sv = c.g[S0][0];
            Addr((c.g[S1][0] as u32).wrapping_add(0x8)).sw(c, sv);
            call_ptr(c, 0x0022_6610, 0x0030_3f4c)?;
            let sv = c.g[S0][0];
            Addr(c.g[V0][0] as u32).sw(c, sv);
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The block fill shared by the `WScriptFn_SetPendingRequest*` bodies and
/// `WScriptFn_GameModeHandlerV07`: the static block at `block` gets a copy of
/// `copy_len` bytes (the copy call returns to `copy_ret`), then the loop
/// stores each parameter into it (a float when the table kind is 2 and the
/// kind differs). `s0` holds the block's stack copy, `a0` the parameter
/// count's base; the loop's event check reports `head`.
#[derive(Clone, Copy)]
struct FillBlock {
    /// The static block's address (`a1` of the copy call).
    block: u32,
    /// Bytes copied (`a2` of the copy call).
    copy_len: u64,
    /// Cycles charged on entering the loop (3 for the 12-byte fills, 4 for the 8-byte ones).
    enter_cycles: u64,
    /// Return address of the copy call.
    copy_ret: u32,
    /// gp offset of the parameter kind table.
    kind_table: u32,
    /// Loop head (the event check's pc).
    head: u32,
}

fn ws_fill_block(c: &mut Cpu, p: FillBlock) -> Flow {
    c.cycles += 7;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = p.copy_len;
    c.g[A1][0] = sx(p.block);
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x003e_6574, p.copy_ret)?;
    let skip = (c.g[S1][0] as i64) <= 0;
    c.g[A2][0] = c.g[S2][0];
    if !skip {
        c.cycles += p.enter_cycles;
        c.g[A1][0] = c.g[S1][0];
        c.g[T0][0] = sx(ws_gp(c, p.kind_table));
        c.g[A3][0] = 2;
        loop {
            let s0 = c.g[S0][0] as u32;
            c.cycles += 8;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(s0));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
            if c.g[V1][0] == c.g[V0][0] {
                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x4)).lw(c);
                c.cycles += 1;
                let (dst, v) = (c.g[A0][0] as u32, c.g[V0][0]);
                Addr(dst).sw(c, v);
            } else {
                c.cycles += 2;
                if c.g[V0][0] != c.g[A3][0] {
                    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x4)).lw(c);
                    c.cycles += 1;
                    let (dst, v) = (c.g[A0][0] as u32, c.g[V0][0]);
                    Addr(dst).sw(c, v);
                } else {
                    c.cycles += 4;
                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                    c.cvt_s_w(0, 0);
                    c.swc1(0, c.g[A0][0] as u32);
                }
            }
            c.cycles += 3;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
            let more = c.g[A1][0] != 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, p.head);
            }
        }
    }
    Ok(())
}

/// The return of the `WScriptFn_SetPendingRequest*` and `GameModeHandlerV07`
/// bodies: the 12-byte (or 8-byte) copy from the nil value at `0x4c9098` to
/// the sret pointer `s3`, then the frame's restore and return.
fn ws_sret_return(c: &mut Cpu, frame: Frame) {
    c.g[V1][0] = 0x004d_0000;
    c.g[V0][0] = c.g[S3][0];
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
    c.ldl(A0, (c.g[A3][0] as u32).wrapping_add(0x7));
    c.ldr(A0, c.g[A3][0] as u32);
    c.g[A1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x8)).lw(c);
    c.sdl(A0, (c.g[S3][0] as u32).wrapping_add(0x7));
    c.sdr(A0, c.g[S3][0] as u32);
    let sret = (c.g[S3][0] as u32).wrapping_add(0x8);
    let v = c.g[A1][0];
    Addr(sret).sw(c, v);
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
}

/// `WScriptFn_SetPendingRequest{A,B,C}`'s call: the block's two words go to
/// the setter with the WScriptMan pointer, then the sret return. The
/// setter's return address is `ret`.
fn ws_pending_call(c: &mut Cpu, frame: Frame, setter: Callee, ret: u32) -> Flow {
    c.cycles += 20;
    let s0 = c.g[S0][0] as u32;
    c.g[A2][0] = Addr(s0.wrapping_add(0x4)).lw(c);
    c.g[A1][0] = Addr(s0).lw(c);
    c.g[A0][0] = Addr(ws_gp(c, 0xce8)).lw(c);
    setter.call(c, ret)?;
    ws_sret_return(c, frame);
    Ok(())
}

/// `void WScriptFn_SetPendingRequestA(sret, int count, Param* params)`
/// (0x303ff0): on the first call clears the static request block (`0x4fbc98`:
/// its 0 word, -1 word and flag byte), then copies `count` parameters into
/// the block and passes its two words to `WScriptMan_SetPendingRequestA`.
pub fn wscript_fn_set_pending_request_a(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(ws_gp(c, 0x26f4)).lw(c);
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        c.g[A2][0] = 0x0050_0000;
        if !initialised {
            c.cycles += 8;
            let block = (c.g[A2][0] as u32).wrapping_sub(0x4368);
            Addr(block).sw(c, 0);
            c.g[V0][0] = sx(block);
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = 1;
            c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lbu(c);
            let sv = c.g[V1][0];
            Addr((c.g[V0][0] as u32).wrapping_add(0x4)).sw(c, sv);
            let sv = c.g[A0][0];
            Addr(ws_gp(c, 0x26f4)).sw(c, sv);
            let sv = c.g[A1][0];
            Addr(sp(c)).sb(c, sv);
        }
        ws_fill_block(
            c,
            FillBlock {
                block: 0x0050_0000 - 0x4368,
                copy_len: 0xc,
                enter_cycles: 3,
                copy_ret: 0x0030_4054,
                kind_table: 0xc50,
                head: 0x0030_4068,
            },
        )?;
        ws_pending_call(
            c,
            frame,
            Callee {
                addr: 0x0030_b4c0,
                f: ee::s10::f_0030b4c0,
            },
            0x0030_40c0,
        )
    })
}

/// `void WScriptFn_SetPendingRequestB(sret, int count, Param* params)`
/// (0x304100): the same as A with its own flag word (`gp+0x26f8`), block
/// (`0x4fbca8`), kind table and setter (`WScriptMan_SetPendingRequestB`).
pub fn wscript_fn_set_pending_request_b(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(ws_gp(c, 0x26f8)).lw(c);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        c.g[A1][0] = 0x0050_0000;
        if !initialised {
            c.cycles += 8;
            let block = (c.g[A1][0] as u32).wrapping_sub(0x4358);
            Addr(block).sw(c, 0);
            c.g[V0][0] = sx(block);
            c.g[V1][0] = 0x1;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lbu(c);
            let sv = c.g[V1][0];
            Addr(ws_gp(c, 0x26f8)).sw(c, sv);
            let sv = c.g[A0][0];
            Addr((c.g[V0][0] as u32).wrapping_add(0x8)).sb(c, sv);
            Addr((c.g[V0][0] as u32).wrapping_add(0x4)).sw(c, 0);
            let sv = c.g[A0][0];
            Addr(sp(c)).sb(c, sv);
        }
        ws_fill_block(
            c,
            FillBlock {
                block: 0x0050_0000 - 0x4358,
                copy_len: 0xc,
                enter_cycles: 3,
                copy_ret: 0x0030_4164,
                kind_table: 0xc58,
                head: 0x0030_4178,
            },
        )?;
        ws_pending_call(
            c,
            frame,
            Callee {
                addr: 0x0030_b4d8,
                f: ee::s10::f_0030b4d8,
            },
            0x0030_41d0,
        )
    })
}

/// `void WScriptFn_SetPendingRequestC(sret, int count, Param* params)`
/// (0x304210): as A with flag `gp+0x26fc`, block `0x4fbcb8`, kind table
/// `gp+0xc60` and setter `WScriptMan_SetPendingRequestC`.
pub fn wscript_fn_set_pending_request_c(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(ws_gp(c, 0x26fc)).lw(c);
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        c.g[A2][0] = 0x0050_0000;
        if !initialised {
            c.cycles += 8;
            let block = (c.g[A2][0] as u32).wrapping_sub(0x4348);
            Addr(block).sw(c, 0);
            c.g[V0][0] = sx(block);
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = 1;
            c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lbu(c);
            let sv = c.g[V1][0];
            Addr((c.g[V0][0] as u32).wrapping_add(0x4)).sw(c, sv);
            let sv = c.g[A0][0];
            Addr(ws_gp(c, 0x26fc)).sw(c, sv);
            let sv = c.g[A1][0];
            Addr(sp(c)).sb(c, sv);
        }
        ws_fill_block(
            c,
            FillBlock {
                block: 0x0050_0000 - 0x4348,
                copy_len: 0xc,
                enter_cycles: 3,
                copy_ret: 0x0030_4274,
                kind_table: 0xc60,
                head: 0x0030_4288,
            },
        )?;
        ws_pending_call(
            c,
            frame,
            Callee {
                addr: 0x0030_b4f0,
                f: ee::s10::f_0030b4f0,
            },
            0x0030_42e0,
        )
    })
}

/// `void WScriptFn_GameModeHandlerV07(sret, int count, Param* params)`
/// (0x3044a8): the parameter block fill over the 8-byte static block at
/// `gp+0x2710` (first use: its -1 word, the flag `gp+0x2718`, a byte copy),
/// then the game mode manager's rider gate with the being's word (0x238550).
pub fn wscript_fn_game_mode_handler_v07(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(ws_gp(c, 0x2718)).lw(c);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        if !initialised {
            c.cycles += 6;
            c.g[V0][0] = Addr(ws_gp(c, 0x2714)).lbu(c);
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = 1;
            let (word, flag) = (c.g[V1][0], c.g[A0][0]);
            Addr(ws_gp(c, 0x2710)).sw(c, word);
            Addr(ws_gp(c, 0x2718)).sw(c, flag);
            let byte = c.g[V0][0];
            Addr(sp(c)).sb(c, byte);
        }
        let block = ws_gp(c, 0x2710);
        ws_fill_block(
            c,
            FillBlock {
                block,
                copy_len: 8,
                enter_cycles: 4,
                copy_ret: 0x0030_4500,
                kind_table: 0xc70,
                head: 0x0030_4518,
            },
        )?;
        c.cycles += 22;
        c.g[V0][0] = Addr(ws_gp(c, 0xce8)).lw(c);
        c.g[V1][0] = Addr(ws_gp(c, 0u32.wrapping_sub(0x848))).lw(c);
        c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc0)).lw(c);
        let s0 = c.g[S0][0] as u32;
        c.g[A2][0] = Addr(s0).lw(c);
        call_ptr(c, 0x0023_8550, 0x0030_4578)?;
        ws_sret_return(c, frame);
        Ok(())
    })
}

/// `void WScriptFn_Nop_4ff0(sret, int count, Param* params)` (0x304ff0): the
/// parameter block fill over the static block `0x4fbd30` (first use clears
/// it and sets the flag `gp+0x2748`), then the nil return. Nothing else runs.
pub fn wscript_fn_nop_4ff0(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(ws_gp(c, 0x2748)).lw(c);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        c.g[A1][0] = 0x0050_0000;
        if !initialised {
            c.cycles += 8;
            let block = (c.g[A1][0] as u32).wrapping_sub(0x42d0);
            Addr(block).sw(c, 0);
            c.g[V0][0] = sx(block);
            c.g[V1][0] = 0x1;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lbu(c);
            let flag = c.g[V1][0];
            Addr(ws_gp(c, 0x2748)).sw(c, flag);
            let byte = c.g[A0][0];
            Addr((c.g[V0][0] as u32).wrapping_add(0x8)).sb(c, byte);
            Addr((c.g[V0][0] as u32).wrapping_add(0x4)).sw(c, 0);
            Addr(sp(c)).sb(c, byte);
        }
        ws_fill_block(
            c,
            FillBlock {
                block: 0x0050_0000 - 0x42d0,
                copy_len: 0xc,
                enter_cycles: 3,
                copy_ret: 0x0030_5054,
                kind_table: 0xc98,
                head: 0x0030_5068,
            },
        )?;
        c.cycles += 16;
        ws_sret_return(c, frame);
        Ok(())
    })
}

/// `void WScriptFn_ReplayHighlight(sret, int count, Param* params)`
/// (0x304b38): the parameter block fill over the 8-byte block at `gp+0x2728`
/// (first use: its -1 word, flag `gp+0x2730`, a byte copy), then one call
/// (0x2709d8) with the world's `+0x84 +0x28` record, the WScriptMan pointer's
/// word and the block's first word.
pub fn wscript_fn_replay_highlight(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(ws_gp(c, 0x2730)).lw(c);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        if !initialised {
            c.cycles += 6;
            c.g[V0][0] = Addr(ws_gp(c, 0x272c)).lbu(c);
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = 1;
            let (word, flag) = (c.g[V1][0], c.g[A0][0]);
            Addr(ws_gp(c, 0x2728)).sw(c, word);
            Addr(ws_gp(c, 0x2730)).sw(c, flag);
            let byte = c.g[V0][0];
            Addr(sp(c)).sb(c, byte);
        }
        let block = ws_gp(c, 0x2728);
        ws_fill_block(
            c,
            FillBlock {
                block,
                copy_len: 8,
                enter_cycles: 4,
                copy_ret: 0x0030_4b90,
                kind_table: 0xc80,
                head: 0x0030_4ba8,
            },
        )?;
        c.cycles += 23;
        c.g[V0][0] = Addr(ws_gp(c, 0u32.wrapping_sub(0x848))).lw(c);
        c.g[V1][0] = Addr(ws_gp(c, 0xce8)).lw(c);
        c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
        c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x28)).lw(c);
        let s0 = c.g[S0][0] as u32;
        c.g[A2][0] = Addr(s0).lw(c);
        call_ptr(c, 0x0027_09d8, 0x0030_4c0c)?;
        ws_sret_return(c, frame);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP};

    /// The BELibrary dummy at gp-0x1f04 (the BE getters' state).
    fn be_dummy(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3000);
        fill(c, &mut r, DATA + 0x3100, 0x200);
        r
    }

    #[test]
    fn wscript_fn_game_mode_matches_matches() {
        check(
            ee::s10::f_00303d90__generated,
            wscript_fn_game_mode_matches,
            200,
            &|c, seed| {
                let mut r = be_dummy(c, seed, 0x3d90);
                c.g[A0][0] = (seed % 7) as u64;
                for i in 0..4u32 {
                    c.bus.write32(0x0044_65e8 + 4 * i, r.next_u32() % 3);
                }
                c.bus.write8(0x0053_5c11, (seed % 3) as u8);
            },
        );
    }

    #[test]
    fn wscript_fn_event_matches_matches() {
        check(
            ee::s10::f_00303e60__generated,
            wscript_fn_event_matches,
            200,
            &|c, seed| {
                let mut r = be_dummy(c, seed, 0x3e60);
                c.g[A0][0] = (seed % 18) as u64;
                for i in 0..16u32 {
                    c.bus.write32(0x0044_6618 + 4 * i, r.next_u32() % 3);
                }
                c.bus.write8(0x0053_5c12, (seed % 3) as u8);
            },
        );
    }

    /// The WScriptMan pointer (gp+0xce8) and its game state: the being index
    /// (or -1) and the chain `world->+0x84->+0xc[being]->+0x28` to a record
    /// whose `+0x6c0` points at a `{delta, method}` pair.
    fn is_human_rider_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = machine(c, seed, 0x3ec8, 2);
        let wsm = DATA + 0x100;
        let being = if seed % 5 == 4 { u32::MAX } else { seed % 4 };
        c.bus.write32(GAME_GP.wrapping_add(0xce8), wsm);
        c.bus.write32(wsm, being);
        let world = DATA + 0x200;
        let arr = DATA + 0x300;
        let tbl = DATA + 0x400;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x84, arr);
        c.bus.write32(arr + 0xc, tbl);
        let obj = DATA + 0x600;
        let a1rec = DATA + 0x700;
        c.bus.write32(
            tbl + 4 * (being & 3) + 0x28,
            if seed % 6 == 0 { 0 } else { obj },
        );
        c.bus.write32(obj + 0x6c0, a1rec);
        c.bus.write16(a1rec + 0x48, (r.next_u32() & 0xfff0) as u16);
        c.bus
            .write32(a1rec + 0x4c, crate::difftest::fakes::FAKE + 0x20);
        r
    }

    #[test]
    fn wscript_fn_is_human_rider_matches() {
        crate::difftest::fakes::covered(
            ee::s10::f_00303ec8__generated,
            wscript_fn_is_human_rider,
            200,
            &|c, seed| {
                let mut r = is_human_rider_fixture(c, seed);
                c.g[A0][0] = DATA as u64 + 0x800;
                c.bus.write32(DATA + 0x800 + 0x8, r.next_u32());
            },
        );
    }

    /// The request block's flag word, the setter's record, and a kind table
    /// with random kinds; the parameters follow at `DATA + 0x2100`.
    fn pending_fixture(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP.wrapping_add(0xce8), DATA + 0x4000);
        fill(c, &mut r, DATA + 0x4000, 0x40);
        for off in [0xc50u32, 0xc58, 0xc60] {
            for i in 0..3u32 {
                c.bus
                    .write32(GAME_GP.wrapping_add(off + 4 * i), r.next_u32() % 3);
            }
        }
        for i in 0..3u32 {
            let p = DATA + 0x2100 + 16 * i;
            c.bus.write32(p, r.next_u32() % 3);
            c.bus.write32(p + 4, r.next_u32());
            c.bus.write32(p + 0xc, r.next_u32() % 3);
        }
        r
    }

    fn pending_args(c: &mut Cpu, seed: u32) {
        c.g[A0][0] = DATA as u64 + 0x2000;
        c.g[A1][0] = (seed % 4) as u64;
        c.g[A2][0] = DATA as u64 + 0x2100;
    }

    #[test]
    fn wscript_fn_set_pending_request_a_matches() {
        check(
            ee::s10::f_00303ff0__generated,
            wscript_fn_set_pending_request_a,
            200,
            &|c, seed| {
                let _r = pending_fixture(c, seed, 0x3ff0);
                c.bus.write32(
                    GAME_GP.wrapping_add(0x26f4),
                    if seed % 2 == 0 { 0 } else { 1 },
                );
                pending_args(c, seed);
            },
        );
    }

    #[test]
    fn wscript_fn_set_pending_request_b_matches() {
        check(
            ee::s10::f_00304100__generated,
            wscript_fn_set_pending_request_b,
            200,
            &|c, seed| {
                let _r = pending_fixture(c, seed, 0x4100);
                c.bus.write32(
                    GAME_GP.wrapping_add(0x26f8),
                    if seed % 2 == 0 { 0 } else { 1 },
                );
                pending_args(c, seed);
            },
        );
    }

    #[test]
    fn wscript_fn_set_pending_request_c_matches() {
        check(
            ee::s10::f_00304210__generated,
            wscript_fn_set_pending_request_c,
            200,
            &|c, seed| {
                let _r = pending_fixture(c, seed, 0x4210);
                c.bus.write32(
                    GAME_GP.wrapping_add(0x26fc),
                    if seed % 2 == 0 { 0 } else { 1 },
                );
                pending_args(c, seed);
            },
        );
    }

    #[test]
    fn wscript_fn_game_mode_handler_v07_matches() {
        check(
            ee::s10::f_003044a8__generated,
            wscript_fn_game_mode_handler_v07,
            200,
            &|c, seed| {
                let mut r = pending_fixture(c, seed, 0x44a8);
                c.bus.write32(
                    GAME_GP.wrapping_add(0x2718),
                    if seed % 2 == 0 { 0 } else { 1 },
                );
                c.bus.write8(GAME_GP.wrapping_add(0x2714), (seed % 3) as u8);
                let wsm = DATA + 0x4000;
                c.bus.write32(wsm, r.next_u32() % 4);
                c.bus.write32(DATA + 0x5000 + 0xc0, DATA + 0x5100);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x5000);
                pending_args(c, seed);
            },
        );
    }

    #[test]
    fn wscript_fn_nop_4ff0_matches() {
        check(
            ee::s10::f_00304ff0__generated,
            wscript_fn_nop_4ff0,
            200,
            &|c, seed| {
                let _r = pending_fixture(c, seed, 0x4ff0);
                c.bus.write32(
                    GAME_GP.wrapping_add(0x2748),
                    if seed % 2 == 0 { 0 } else { 1 },
                );
                c.bus.write8(GAME_GP.wrapping_add(0x2744), (seed % 3) as u8);
                pending_args(c, seed);
            },
        );
    }

    #[test]
    fn wscript_fn_replay_highlight_matches() {
        check(
            ee::s10::f_00304b38__generated,
            wscript_fn_replay_highlight,
            200,
            &|c, seed| {
                let mut r = pending_fixture(c, seed, 0x4b38);
                c.bus.write32(
                    GAME_GP.wrapping_add(0x2730),
                    if seed % 2 == 0 { 0 } else { 1 },
                );
                c.bus.write8(GAME_GP.wrapping_add(0x272c), (seed % 3) as u8);
                let world = DATA + 0x5000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, DATA + 0x5100);
                c.bus.write32(DATA + 0x5100 + 0x28, DATA + 0x5200);
                c.bus.write32(DATA + 0x4000, r.next_u32() % 4);
                pending_args(c, seed);
            },
        );
    }
}
