//! Staging module oc881 (wave wave252): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc881`.

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
        0x001b_2048,
        cfe_state_chat_room_dtor,
        "cFEStateChatRoom_dtor",
    ),
    (
        0x001b_1f88,
        cfe_state_chat_room_ctor,
        "cFEStateChatRoom_ctor",
    ),
    (
        0x001b_2cd0,
        cfe_state_chat_room_add_chat_line,
        "cFEStateChatRoom_AddChatLine",
    ),
    (0x001b_2da0, cfe_state_chat_room_v25, "cFEStateChatRoom_v25"),
];

/// `cFEStateChatRoom::~cFEStateChatRoom(this)` (0x1b2048): stores the chat
/// room vtable (0x46ae58) at +8, then tail-calls the base `NetFEState`
/// destructor, which returns here.
pub fn cfe_state_chat_room_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x0047_0000u32.wrapping_sub(0x51a8));
        c.g[V1][0] = c.g[A0][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x001a_85d0, 0x001b_2064)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateChatRoom::cFEStateChatRoom(this)` (0x1b1f88): the base
/// `NetFEState` constructor, then the chat room's vtable (0x46ae58), hash
/// name 0x3d at +0xc, two zeroed 8-byte slots below the chat-line table at
/// +0x704, the counters at +0x6ec..+0x6f4, the 1.0f/0 stack values copied
/// into the text fields at +0x72c..+0x73b, and the name string (the word at
/// `gp-0xc8` names the table, its +0x80 word the string) copied to +0x70c.
pub fn cfe_state_chat_room_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001a_8500, 0x001b_1fa0)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = sx(0x0047_0000u32.wrapping_sub(0x51a8));
        c.g[V0][0] = 0x3d;
        c.sw(this.wrapping_add(0xc), c.g[V0][0]);
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.g[V0][0] = sx(this.wrapping_add(0x704));
        c.sw(this.wrapping_add(0x6f8), 0);
        c.g[V1][0] = 0x1;
        loop {
            c.cycles += 7;
            c.sw((c.g[V0][0] as u32).wrapping_sub(0x8), 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            c.sw(c.g[V0][0] as u32, 0);
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001b_1fc0);
            }
        }
        c.cycles += 26;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = sx(this.wrapping_add(0x70c));
        c.sw(this.wrapping_add(0x6ec), 0);
        let a = frame.at(c, 0xc);
        c.swc1(0, a);
        let a = frame.at(c, 0x0);
        c.swc1(0, a);
        let a = frame.at(c, 0x4);
        c.swc1(0, a);
        let a = frame.at(c, 0x8);
        c.swc1(0, a);
        c.sw(this.wrapping_add(0x6f0), 0);
        c.sw(this.wrapping_add(0x6f4), 0);
        let a = frame.at(c, 0x7);
        c.ldl(2, a);
        let a = frame.at(c, 0x0);
        c.ldr(2, a);
        let a = frame.at(c, 0xf);
        c.ldl(3, a);
        let a = frame.at(c, 0x8);
        c.ldr(3, a);
        c.sdl(2, this.wrapping_add(0x733));
        c.sdr(2, this.wrapping_add(0x72c));
        c.sdl(3, this.wrapping_add(0x73b));
        c.sdr(3, this.wrapping_add(0x734));
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x80));
        call_ptr(c, 0x0041_66f4, 0x001b_2030)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// The scroll-and-trim half of `cFEStateChatRoom::AddChatLine`: runs with
/// `v0` = the chat room's stat block (`+0x6f0`) and `s5` = the new line.
fn add_chat_line_scroll(c: &mut Cpu, this: u32) -> Flow {
    c.cycles += 28;
    c.g[S0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0xc8));
    c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x8));
    c.g[S3][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0xc6));
    c.g[S1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0xc4));
    c.g[S2][0] = (c.g[S0][0] < 0x1u64) as u64;
    c.g[A0][0] = c.g[SP][0];
    call_ptr(c, 0x0031_7fe8, 0x001b_2d24)?;
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = sx(0x004a_1a50);
    call_ptr(c, 0x0031_8910, 0x001b_2d34)?;
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0xc));
    call_ptr(c, 0x0031_89a0, 0x001b_2d40)?;
    let s = sp(c);
    c.g[A1][0] = c.lw(s);
    c.g[A0][0] = c.lw(this.wrapping_add(0x6f0));
    call_ptr(c, 0x003a_19f8, 0x001b_2d4c)?;
    c.g[A0][0] = c.lw(this.wrapping_add(0x6f0));
    c.g[A1][0] = 0x40;
    c.g[A3][0] = c.g[S0][0];
    c.g[T0][0] = c.g[S1][0];
    c.g[T1][0] = c.g[S3][0];
    c.g[T2][0] = c.g[S2][0];
    c.g[A2][0] = 0x6;
    c.g[T3][0] = 0x1;
    call_ptr(c, 0x001a_83d8, 0x001b_2d70)?;
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x0031_8260, 0x001b_2d7c)?;
    Ok(())
}

/// `void cFEStateChatRoom::AddChatLine(this, line)` (0x1b2cd0): when the room
/// has a stat block (`+0x40` and `+0x6f0` non-zero), reads its three 16-bit
/// fields, formats the line through the text helpers and scrolls the chat
/// text (`cUITextScroll_TrimAndScroll`, 0x1a83d8).
pub fn cfe_state_chat_room_add_chat_line(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save_ra(c, 0x10);
        let this = c.g[S4][0] as u32;
        c.g[V0][0] = c.lw(this.wrapping_add(0x40));
        c.g[S5][0] = c.g[A1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw(this.wrapping_add(0x6f0));
            c.g[A0][0] = c.g[SP][0];
            if c.g[V0][0] != 0 {
                add_chat_line_scroll(c, this)?;
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// A string-table lookup: `a0 = 0x460000 + off` through `0x317670`, which
/// returns to `ret` with the result in `v0`.
fn v25_lookup(c: &mut Cpu, off: u32, ret: u32) -> Flow {
    c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(off));
    call_ptr(c, 0x0031_7670, ret)
}

/// `0x1da510(this, index, v0, count)`: the chat room's table setup call with
/// `a2` = the lookup result in `v0`.
fn v25_emit(c: &mut Cpu, index: u64, count: u64, ret: u32) -> Flow {
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = index;
    c.g[A3][0] = count;
    call_ptr(c, 0x001d_a510, ret)
}

/// `void cFEStateChatRoom::v25(this)` (0x1b2da0): a straight sequence of
/// table setup calls; the lookup results are stored at +0x54 and passed on.
pub fn cfe_state_chat_room_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 70;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x6;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_a528, 0x001b_2dbc)?;
        v25_lookup(c, 0x3b70, 0x001b_2dc8)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[V0][0]);
        v25_lookup(c, 0x3b88, 0x001b_2dd8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x001d_a530, 0x001b_2de8)?;
        v25_lookup(c, 0x3ba8, 0x001b_2df4)?;
        v25_emit(c, 0, 3, 0x001b_2e08)?;
        v25_lookup(c, 0x3bc0, 0x001b_2e14)?;
        v25_emit(c, 1, 0, 0x001b_2e28)?;
        v25_lookup(c, 0x3bd8, 0x001b_2e34)?;
        v25_emit(c, 2, 6, 0x001b_2e48)?;
        v25_lookup(c, 0x1458, 0x001b_2e54)?;
        v25_emit(c, 3, 2, 0x001b_2e68)?;
        v25_lookup(c, 0x14e8, 0x001b_2e74)?;
        v25_emit(c, 4, 1, 0x001b_2e88)?;
        v25_lookup(c, 0x3bf0, 0x001b_2e94)?;
        v25_emit(c, 5, 8, 0x001b_2ea8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};

    #[test]
    fn cfe_state_chat_room_dtor_matches() {
        check(
            ee::s03::f_001b2048__generated,
            cfe_state_chat_room_dtor,
            200,
            &|c, seed| {
                let mut r = Rng(0x2048_0001 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x001b_2048;
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn cfe_state_chat_room_ctor_matches() {
        check(
            ee::s03::f_001b1f88__generated,
            cfe_state_chat_room_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x1f88_0002 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x001b_1f88;
                c.g[A0][0] = sx(DATA);
                c.g[GP][0] = GAME_GP as u64;
                // gp-0xc8 names the table; its +0x80 word is the name string.
                c.bus.write32(GAME_GP.wrapping_sub(0xc8), DATA + 0x400);
                c.bus.write32(DATA + 0x480, DATA + 0x500);
                for (i, b) in b"chat\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x500 + i as u32, *b);
                }
            },
        );
    }

    #[test]
    fn cfe_state_chat_room_add_chat_line_matches() {
        check(
            ee::s03::f_001b2cd0__generated,
            cfe_state_chat_room_add_chat_line,
            200,
            &|c, seed| {
                let mut r = Rng(0x2cd0_0003 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x001b_2cd0;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                let stats = DATA + 0x1000;
                // Covers both early exits (+0x40 and +0x6f0 zero) and the full path.
                c.bus
                    .write32(DATA + 0x40, if seed % 4 == 0 { 0 } else { 1 });
                c.bus
                    .write32(DATA + 0x6f0, if seed % 4 == 1 { 0 } else { stats });
                c.bus.write16(stats + 0xc8, (seed % 3) as u16);
                c.bus.write16(stats + 0xc6, (r.next_u32() & 0xffff) as u16);
                c.bus.write16(stats + 0xc4, (r.next_u32() & 0xffff) as u16);
                fill(c, &mut r, DATA + 0x2000, 16);
            },
        );
    }

    #[test]
    fn cfe_state_chat_room_v25_matches() {
        check(
            ee::s03::f_001b2da0__generated,
            cfe_state_chat_room_v25,
            200,
            &|c, seed| {
                let mut r = Rng(0x2da0_0004 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x001b_2da0;
                c.g[A1][0] = sx(DATA);
            },
        );
    }
}
