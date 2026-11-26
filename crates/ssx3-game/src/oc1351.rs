//! Staging module oc1351 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1351`.
//!
//! This module ports `LobbyApi_Update` (0x3e95c8), the per-frame lobby pump.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, returned, Callee};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x003e_95c8, lobby_api_update, "LobbyApi_Update")];

/// Per-frame lobby pump (`LobbyApi_Update`, 0x3e95c8): sample the clock,
/// run the watchdog, answer ping traffic, then drain one batch of inbound
/// server messages.
///
/// The client word at `+0x00` holds the transport handle and `+0x0c` the
/// state tag (`offl`, `term`, `rdir`, `skey`, `idle`, `conn`, `auth`,
/// `acct`). A null client returns at once; `term` and `offl` skip the
/// network below. Otherwise the pump re-arms the timeout from the sampled
/// tick, runs `LobbyApi_ProcessPings`, then loops on `ProtoAriesPeek`:
/// each message runs `LobbyApi_PopRequest` plus its handler, then the
/// matching tag reads update the cached session, room, user and rank
/// lists, the event flags and the registered callbacks (when set). A ping
/// timer re-arms the server ping on the way out. Tail calls route through
/// the override table so replaced callees are reached too.
pub fn lobby_api_update(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                // Client tick: a null client returns; else sample the clock.
                0x003e95c8 => {
                    c.cycles += 15;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x420));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x3e0));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x410));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x400));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x3f0));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x3d0));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x3c0));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x3b0));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x3a0));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x390));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x380), c.g[RA][0]);
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    let t = c.g[A0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x410));
                    if t {
                        pc = 0x003eae38;
                        continue;
                    }
                    pc = 0x003e9604;
                    continue;
                }
                // Watchdog: sample the tick, catch up a late client, time out.
                0x003e9604 => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x003e960c);
                    callee::PROTO_ARIES_TICK.call(c, 0x003e960c)?;
                    c.g[S7][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = 0x6f660000;
                    c.g[V0][0] = c.g[V0][0] | 0x666c;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x74650000;
                    if t {
                        pc = 0x003e9670;
                        continue;
                    }
                    pc = 0x003e9624;
                    continue;
                }
                0x003e9624 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = sx((c.g[S7][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    if t {
                        pc = 0x003e9644;
                        continue;
                    }
                    pc = 0x003e9630;
                    continue;
                }
                0x003e9630 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 5001i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                    if t {
                        pc = 0x003e9648;
                        continue;
                    }
                    pc = 0x003e963c;
                    continue;
                }
                0x003e963c => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    pc = 0x003e9644;
                    continue;
                }
                0x003e9644 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                    pc = 0x003e9648;
                    continue;
                }
                0x003e9648 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S7][0] as u32));
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.g[V0][0] = 0x74650000;
                    if t {
                        pc = 0x003e966c;
                        continue;
                    }
                    pc = 0x003e9654;
                    continue;
                }
                0x003e9654 => {
                    c.cycles += 6;
                    c.g[A1][0] = 0x74690000;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[A1][0] | 0x6d65;
                    c.g[RA][0] = sx(0x003e9668);
                    c.g[A2][0] = 0x800;
                    callee::LOBBY_API_TERMINATE.call(c, 0x003e9668)?;
                    c.g[V0][0] = 0x74650000;
                    pc = 0x003e966c;
                    continue;
                }
                0x003e966c => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    pc = 0x003e9670;
                    continue;
                }
                // State gate: `term` and `offl` skip the network below.
                0x003e9670 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x726d;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x14), c.g[S7][0]);
                    if t {
                        pc = 0x003eae34;
                        continue;
                    }
                    pc = 0x003e967c;
                    continue;
                }
                0x003e967c => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x6f660000;
                    c.g[V0][0] = c.g[V0][0] | 0x666c;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x410));
                    if t {
                        pc = 0x003eae38;
                        continue;
                    }
                    pc = 0x003e968c;
                    continue;
                }
                // Ping service, then take the first inbound message.
                0x003e968c => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x003e9694);
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_API_PROCESS_PINGS.call(c, 0x003e9694)?;
                    c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x364));
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x36c), c.g[V0][0]);
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x370), c.g[V1][0]);
                    if t {
                        pc = 0x003eade8;
                        continue;
                    }
                    pc = 0x003e96ac;
                    continue;
                }
                0x003e96ac => {
                    c.cycles += 1;
                    pc = 0x003e96b0;
                    continue;
                }
                // Message loop head: re-arm the watchdog, check `~png`.
                0x003e96b0 => {
                    c.cycles += 6;
                    c.g[V1][0] = 0x7e700000;
                    c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[V1][0] = c.g[V1][0] | 0x6e67;
                    let t = c.g[A0][0] != c.g[V1][0];
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    if t {
                        pc = 0x003e9728;
                        continue;
                    }
                    pc = 0x003e96c8;
                    continue;
                }
                0x003e96c8 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0xffffffffffff0000;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        pc = 0x003e9708;
                        continue;
                    }
                    pc = 0x003e96dc;
                    continue;
                }
                0x003e96dc => {
                    c.cycles += 11;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[RA][0] = sx(0x003e96e8);
                    c.g[A1][0] = 0x100;
                    callee::TAG_FIELD_STRNZCPY.call(c, 0x003e96e8)?;
                    c.g[A2][0] = 0x4a0000;
                    c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4908));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x003e9700);
                    c.g[A1][0] = 0x100;
                    callee::TAG_FIELD_SET_NUMBER.call(c, 0x003e9700)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x368), c.g[SP][0]);
                    c.g[A2][0] = 0xffffffffffffffff;
                    pc = 0x003e9708;
                    continue;
                }
                0x003e9708 => {
                    c.cycles += 8;
                    c.g[T0][0] = c.g[A2][0];
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    c.g[RA][0] = sx(0x003e9720);
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    callee::PROTO_ARIES_SEND.call(c, 0x003e9720)?;
                    let t = true;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    if t {
                        pc = 0x003eadd4;
                        continue;
                    }
                    pc = 0x003e9728;
                    continue;
                }
                // Dispatch: pop the pending request and run its handler.
                0x003e9728 => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x514));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x104), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x108), c.g[V0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[V1][0]);
                    let t = c.g[A3][0] == 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x100), 0u64);
                    if t {
                        pc = 0x003e9758;
                        continue;
                    }
                    pc = 0x003e9748;
                    continue;
                }
                0x003e9748 => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x518));
                    c.g[A0][0] = c.g[S3][0];
                    let t = c.g[A3][0] as u32;
                    c.g[RA][0] = sx(0x003e9758);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.pc = t;
                    call_indirect(c, t);
                    returned(c, 0x003e9758)?;
                    pc = 0x003e9758;
                    continue;
                }
                0x003e9758 => {
                    c.cycles += 7;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x003e9768);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
                    callee::LOBBY_API_POP_REQUEST.call(c, 0x003e9768)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x148));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                    if t {
                        pc = 0x003e9780;
                        continue;
                    }
                    pc = 0x003e9774;
                    continue;
                }
                0x003e9774 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S3][0];
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x003e9780);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.pc = t;
                    call_indirect(c, t);
                    returned(c, 0x003e9780)?;
                    pc = 0x003e9780;
                    continue;
                }
                0x003e9780 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x72640000;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] | 0x6972;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    if t {
                        pc = 0x003e9948;
                        continue;
                    }
                    pc = 0x003e9794;
                    continue;
                }
                0x003e9794 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x40640000;
                    c.g[V0][0] = c.g[V0][0] | 0x6972;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x360), 0u64);
                        pc = 0x003e9944;
                        continue;
                    }
                    pc = 0x003e97a4;
                    continue;
                }
                0x003e97a4 => {
                    c.cycles += 16;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A1][0] = 0x6c700000;
                    c.g[RA][0] = sx(0x003e97b4);
                    c.g[A1][0] = c.g[A1][0] | 0x7274;
                    callee::PROTO_ARIES_STATUS.call(c, 0x003e97b4)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A1][0] = 0x6c610000;
                    c.g[A1][0] = c.g[A1][0] | 0x6472;
                    c.g[RA][0] = sx(0x003e97c8);
                    c.g[S1][0] = c.g[V0][0];
                    callee::PROTO_ARIES_STATUS.call(c, 0x003e97c8)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x500), c.g[V0][0]);
                    c.g[RA][0] = sx(0x003e97dc);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4910));
                    callee::TAG_FIELD_FIND.call(c, 0x003e97dc)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    if t {
                        pc = 0x003e9824;
                        continue;
                    }
                    pc = 0x003e97e4;
                    continue;
                }
                0x003e97e4 => {
                    c.cycles += 16;
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e97f0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4890));
                    callee::TAG_FIELD_FIND.call(c, 0x003e97f0)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003e97fc);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_ADDRESS.call(c, 0x003e97fc)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4918));
                    c.g[RA][0] = sx(0x003e9810);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x3b4), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003e9810)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e981c);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003e981c)?;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x3b0), c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    pc = 0x003e9824;
                    continue;
                }
                0x003e9824 => {
                    c.cycles += 23;
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9830);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4920));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9830)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003e983c);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003e983c)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x48e0));
                    c.g[RA][0] = sx(0x003e9850);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x4b8), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003e9850)?;
                    c.g[V1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4bc));
                    c.g[A2][0] = 0x40;
                    c.g[RA][0] = sx(0x003e986c);
                    c.g[A3][0] = c.g[S0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e986c)?;
                    c.g[RA][0] = sx(0x003e9874);
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    callee::PROTO_ARIES_UNCONNECT.call(c, 0x003e9874)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x3b4));
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x636f0000;
                    if t {
                        pc = 0x003e9910;
                        continue;
                    }
                    pc = 0x003e9880;
                    continue;
                }
                0x003e9880 => {
                    c.cycles += 36;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[V0][0] = c.g[V0][0] | 0x6e6e;
                    c.g[RA][0] = sx(0x003e9890);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    callee::LOBBY_API_PING_SERVER.call(c, 0x003e9890)?;
                    c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x3b0));
                    c.g[A1][0] = 0x490000;
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x3b4));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f30));
                    c.g[RA][0] = sx(0x003e98a8);
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_API_DEBUG_PRINTF.call(c, 0x003e98a8)?;
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x3b0));
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[RA][0] = sx(0x003e98b8);
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x3b4));
                    callee::PROTO_ARIES_CONNECT.call(c, 0x003e98b8)?;
                    c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x500));
                    c.g[A2][0] = 0x4a0000;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4890));
                    c.g[A0][0] = c.g[SP][0];
                    c.sb(c.g[SP][0] as u32, 0u64);
                    c.g[RA][0] = sx(0x003e98d4);
                    c.g[A1][0] = 0x100;
                    callee::TAG_FIELD_SET_ADDRESS.call(c, 0x003e98d4)?;
                    c.g[A2][0] = 0x4a0000;
                    c.g[A3][0] = c.g[S1][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4918));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x003e98ec);
                    c.g[A1][0] = 0x100;
                    callee::TAG_FIELD_SET_NUMBER.call(c, 0x003e98ec)?;
                    c.g[A1][0] = 0x61640000;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A1][0] = c.g[A1][0] | 0x6472;
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x003e9908);
                    c.g[T0][0] = 0xffffffffffffffff;
                    callee::PROTO_ARIES_SEND.call(c, 0x003e9908)?;
                    let t = true;

                    if t {
                        pc = 0x003eae08;
                        continue;
                    }
                    pc = 0x003e9910;
                    continue;
                }
                0x003e9910 => {
                    c.cycles += 13;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9920);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4928));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9920)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3b8));
                    c.g[RA][0] = sx(0x003e9934);
                    c.g[A2][0] = 0x100;
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9934)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.g[V1][0] | 0x400;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x360), 0u64);
                    pc = 0x003e9944;
                    continue;
                }
                0x003e9944 => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    pc = 0x003e9948;
                    continue;
                }
                0x003e9948 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xffffffffffffffff;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x736b0000;
                    if t {
                        pc = 0x003e99c8;
                        continue;
                    }
                    pc = 0x003e9954;
                    continue;
                }
                0x003e9954 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[V0][0] = 0x736b0000;
                    if t {
                        pc = 0x003e99c8;
                        continue;
                    }
                    pc = 0x003e9960;
                    continue;
                }
                0x003e9960 => {
                    c.cycles += 26;
                    c.g[A2][0] = 0x4a0000;
                    c.g[A3][0] = 0x490000;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4930));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5fb0));
                    c.sb(c.g[SP][0] as u32, 0u64);
                    c.g[A1][0] = 0x100;
                    c.g[RA][0] = sx(0x003e9984);
                    c.g[T0][0] = 0xa;
                    callee::TAG_FIELD_SET_BINARY.call(c, 0x003e9984)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A1][0] = 0x736b0000;
                    c.g[A1][0] = c.g[A1][0] | 0x6579;
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x003e99a0);
                    c.g[T0][0] = 0xffffffffffffffff;
                    callee::PROTO_ARIES_SEND.call(c, 0x003e99a0)?;
                    c.g[V1][0] = 0x736b0000;
                    c.g[V0][0] = 0x636f0000;
                    c.g[A0][0] = 0x1;
                    c.g[V1][0] = c.g[V1][0] | 0x6579;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x504), c.g[A0][0]);
                    c.g[V0][0] = c.g[V0][0] | 0x6e6e;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x360), c.g[V0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x364), 0u64);
                    c.g[V0][0] = 0x736b0000;
                    pc = 0x003e99c8;
                    continue;
                }
                0x003e99c8 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] | 0x6579;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    if t {
                        pc = 0x003e9a7c;
                        continue;
                    }
                    pc = 0x003e99d8;
                    continue;
                }
                0x003e99d8 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] == c.g[A0][0];
                    c.g[V0][0] = 0x73650000;
                    if t {
                        pc = 0x003e99ec;
                        continue;
                    }
                    pc = 0x003e99e0;
                    continue;
                }
                0x003e99e0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6c65;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        pc = 0x003e9a80;
                        continue;
                    }
                    pc = 0x003e99ec;
                    continue;
                }
                0x003e99ec => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));

                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                        pc = 0x003e9a7c;
                        continue;
                    }
                    pc = 0x003e99f8;
                    continue;
                }
                0x003e99f8 => {
                    c.cycles += 2;

                    if c.g[V1][0] != c.g[A0][0] {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        pc = 0x003e9a24;
                        continue;
                    }
                    pc = 0x003e9a00;
                    continue;
                }
                0x003e9a00 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9a10);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4930));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9a10)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x24));
                    c.g[RA][0] = sx(0x003e9a20);
                    c.g[A2][0] = 0x10;
                    callee::TAG_FIELD_DECODE_HEX.call(c, 0x003e9a20)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    pc = 0x003e9a24;
                    continue;
                }
                0x003e9a24 => {
                    c.cycles += 12;
                    c.g[V1][0] = 0x69640000;
                    c.g[A1][0] = 0x636f0000;
                    c.g[V1][0] = c.g[V1][0] | 0x6c65;
                    c.g[V0][0] = c.g[V0][0] | 0x1;
                    c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x52c));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[A1][0] = c.g[A1][0] | 0x6e6e;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x360), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x364), 0u64);
                    let t = c.g[A3][0] == 0u64;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x18), c.g[S7][0]);
                    if t {
                        pc = 0x003e9a78;
                        continue;
                    }
                    pc = 0x003e9a54;
                    continue;
                }
                0x003e9a54 => {
                    c.cycles += 9;
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x530));
                    c.g[V0][0] = 0x3;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x104), c.g[A1][0]);
                    c.g[A0][0] = c.g[S3][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x100), c.g[V0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x108), 0u64);
                    let t = c.g[A3][0] as u32;
                    c.g[RA][0] = sx(0x003e9a78);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), 0u64);
                    c.pc = t;
                    call_indirect(c, t);
                    returned(c, 0x003e9a78)?;
                    pc = 0x003e9a78;
                    continue;
                }
                0x003e9a78 => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    pc = 0x003e9a7c;
                    continue;
                }
                0x003e9a7c => {
                    c.cycles += 1;
                    c.g[V0][0] = 0xffffffffffffffff;
                    pc = 0x003e9a80;
                    continue;
                }
                0x003e9a80 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[S0][0] = 0x61750000;
                    if t {
                        pc = 0x003e9ac4;
                        continue;
                    }
                    pc = 0x003e9a88;
                    continue;
                }
                0x003e9a88 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0xfffffffffefe0000;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    c.g[V0][0] = c.g[V0][0] | 0xfefe;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    if t {
                        pc = 0x003e9ac8;
                        continue;
                    }
                    pc = 0x003e9a9c;
                    continue;
                }
                0x003e9a9c => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x636f0000;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] | 0x6e6e;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = c.g[S3][0];
                    if t {
                        pc = 0x003e9ac4;
                        continue;
                    }
                    pc = 0x003e9ab0;
                    continue;
                }
                0x003e9ab0 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0xfffffffffefe0000;
                    c.g[A1][0] = c.g[A1][0] | 0xfefe;
                    c.g[RA][0] = sx(0x003e9ac0);
                    c.g[A2][0] = 0x800;
                    callee::LOBBY_API_TERMINATE.call(c, 0x003e9ac0)?;
                    c.g[S0][0] = 0x61750000;
                    pc = 0x003e9ac4;
                    continue;
                }
                0x003e9ac4 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    pc = 0x003e9ac8;
                    continue;
                }
                0x003e9ac8 => {
                    c.cycles += 3;
                    c.g[S0][0] = c.g[S0][0] | 0x7468;
                    let t = c.g[V0][0] != c.g[S0][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    if t {
                        pc = 0x003e9aec;
                        continue;
                    }
                    pc = 0x003e9ad4;
                    continue;
                }
                0x003e9ad4 => {
                    c.cycles += 6;
                    c.g[A1][0] = 0x61750000;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[A1][0] | 0x7468;
                    c.g[RA][0] = sx(0x003e9ae8);
                    c.g[A2][0] = 0x2;
                    callee::LOBBY_API_TERMINATE.call(c, 0x003e9ae8)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    pc = 0x003e9aec;
                    continue;
                }
                0x003e9aec => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[S0][0];
                    c.g[V0][0] = 0x61630000;
                    if t {
                        pc = 0x003e9b5c;
                        continue;
                    }
                    pc = 0x003e9af4;
                    continue;
                }
                0x003e9af4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x61630000;
                    if t {
                        pc = 0x003e9b5c;
                        continue;
                    }
                    pc = 0x003e9b00;
                    continue;
                }
                0x003e9b00 => {
                    c.cycles += 23;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4880));
                    c.g[V0][0] = c.g[V0][0] | 0x2;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    c.g[RA][0] = sx(0x003e9b20);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003e9b20)?;
                    c.g[A3][0] = 0x4a0000;
                    c.g[A2][0] = 0x40;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x134));
                    c.g[RA][0] = sx(0x003e9b38);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9b38)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9b48);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4890));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9b48)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e9b54);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_ADDRESS.call(c, 0x003e9b54)?;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x4fc), c.g[V0][0]);
                    c.g[V0][0] = 0x61630000;
                    pc = 0x003e9b5c;
                    continue;
                }
                0x003e9b5c => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[V0][0] = c.g[V0][0] | 0x6374;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x736e0000;
                    if t {
                        pc = 0x003e9b90;
                        continue;
                    }
                    pc = 0x003e9b6c;
                    continue;
                }
                0x003e9b6c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x736e0000;
                    if t {
                        pc = 0x003e9b90;
                        continue;
                    }
                    pc = 0x003e9b78;
                    continue;
                }
                0x003e9b78 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    c.g[V0][0] = c.g[V0][0] | 0x2;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[V0][0] = 0x736e0000;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    pc = 0x003e9b90;
                    continue;
                }
                0x003e9b90 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6170;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x73650000;
                    if t {
                        pc = 0x003e9bdc;
                        continue;
                    }
                    pc = 0x003e9b9c;
                    continue;
                }
                0x003e9b9c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x50c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[V0][0] != 0u64;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x50c), c.g[V0][0]);
                    if t {
                        pc = 0x003e9bd8;
                        continue;
                    }
                    pc = 0x003e9bac;
                    continue;
                }
                0x003e9bac => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x73650000;
                    if t {
                        pc = 0x003e9bdc;
                        continue;
                    }
                    pc = 0x003e9bb8;
                    continue;
                }
                0x003e9bb8 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9bc8);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x48e8));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9bc8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e9bd4);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003e9bd4)?;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x508), c.g[V0][0]);
                    pc = 0x003e9bd8;
                    continue;
                }
                0x003e9bd8 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x73650000;
                    pc = 0x003e9bdc;
                    continue;
                }
                0x003e9bdc => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[V0][0] = c.g[V0][0] | 0x6c65;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x70650000;
                    if t {
                        pc = 0x003e9c40;
                        continue;
                    }
                    pc = 0x003e9bec;
                    continue;
                }
                0x003e9bec => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x70650000;
                    if t {
                        pc = 0x003e9c40;
                        continue;
                    }
                    pc = 0x003e9bf8;
                    continue;
                }
                0x003e9bf8 => {
                    c.cycles += 18;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9c08);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4938));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9c08)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003e9c14);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003e9c14)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4940));
                    c.g[RA][0] = sx(0x003e9c28);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x504), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003e9c28)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e9c34);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003e9c34)?;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x510), c.g[V0][0]);
                    c.g[V0][0] = 0x70650000;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    pc = 0x003e9c40;
                    continue;
                }
                0x003e9c40 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7273;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2b730000;
                    if t {
                        pc = 0x003e9c88;
                        continue;
                    }
                    pc = 0x003e9c4c;
                    continue;
                }
                0x003e9c4c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x2b730000;
                    if t {
                        pc = 0x003e9c88;
                        continue;
                    }
                    pc = 0x003e9c58;
                    continue;
                }
                0x003e9c58 => {
                    c.cycles += 12;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9c68);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4948));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9c68)?;
                    c.g[A3][0] = 0x4a0000;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x174));
                    c.g[RA][0] = sx(0x003e9c80);
                    c.g[A2][0] = 0x40;
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9c80)?;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x18), c.g[S7][0]);
                    c.g[V0][0] = 0x2b730000;
                    pc = 0x003e9c88;
                    continue;
                }
                0x003e9c88 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[V0][0] = c.g[V0][0] | 0x6573;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2b6d0000;
                    if t {
                        pc = 0x003e9ec4;
                        continue;
                    }
                    pc = 0x003e9c98;
                    continue;
                }
                0x003e9c98 => {
                    c.cycles += 125;
                    c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x200));
                    c.g[A2][0] = 0x110;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003e9cac);
                    c.g[A0][0] = c.g[S1][0];
                    callee::MEMSET.call(c, 0x003e9cac)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9cbc);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4880));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9cbc)?;
                    c.g[S0][0] = 0x4a0000;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4888));
                    c.g[A2][0] = 0x20;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003e9cd8);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9cd8)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9ce8);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4950));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9ce8)?;
                    c.g[A2][0] = 0x20;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x220));
                    c.g[RA][0] = sx(0x003e9cfc);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9cfc)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9d0c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4958));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9d0c)?;
                    c.g[A2][0] = 0x20;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x240));
                    c.g[RA][0] = sx(0x003e9d20);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9d20)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9d30);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4960));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9d30)?;
                    c.g[A2][0] = 0x20;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x260));
                    c.g[RA][0] = sx(0x003e9d44);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9d44)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9d54);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4968));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9d54)?;
                    c.g[A2][0] = 0x10;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x280));
                    c.g[RA][0] = sx(0x003e9d68);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9d68)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9d78);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4970));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9d78)?;
                    c.g[A2][0] = 0x10;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x290));
                    c.g[RA][0] = sx(0x003e9d8c);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9d8c)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9d9c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4978));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9d9c)?;
                    c.g[A2][0] = 0x10;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2a0));
                    c.g[RA][0] = sx(0x003e9db0);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9db0)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9dc0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4980));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9dc0)?;
                    c.g[A2][0] = 0x10;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2b0));
                    c.g[RA][0] = sx(0x003e9dd4);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9dd4)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9de4);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4988));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9de4)?;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A2][0] = 0x40;
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2d0));
                    c.g[RA][0] = sx(0x003e9df8);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9df8)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9e08);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4890));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9e08)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003e9e14);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_ADDRESS.call(c, 0x003e9e14)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4990));
                    c.g[RA][0] = sx(0x003e9e28);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x2c0), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003e9e28)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003e9e34);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_ADDRESS.call(c, 0x003e9e34)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4998));
                    c.g[RA][0] = sx(0x003e9e48);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x2c4), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003e9e48)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003e9e54);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003e9e54)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49a0));
                    c.g[RA][0] = sx(0x003e9e68);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x2c8), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003e9e68)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e9e74);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_DATE.call(c, 0x003e9e74)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x534));
                    c.g[V1][0] = c.g[V1][0] | 0x200;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x2cc), c.g[V0][0]);
                    let t = c.g[T0][0] == 0u64;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    if t {
                        pc = 0x003e9ebc;
                        continue;
                    }
                    pc = 0x003e9e8c;
                    continue;
                }
                0x003e9e8c => {
                    c.cycles += 12;
                    c.g[V0][0] = 0x706c0000;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x538));
                    c.g[V1][0] = 0x4;
                    c.g[V0][0] = c.g[V0][0] | 0x6179;
                    c.g[A0][0] = c.g[S3][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x100), c.g[V1][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x104), c.g[V0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[A3][0]);
                    let t = c.g[T0][0] as u32;
                    c.g[RA][0] = sx(0x003e9ebc);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x108), 0u64);
                    c.pc = t;
                    call_indirect(c, t);
                    returned(c, 0x003e9ebc)?;
                    pc = 0x003e9ebc;
                    continue;
                }
                0x003e9ebc => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x2b6d0000;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    pc = 0x003e9ec4;
                    continue;
                }
                0x003e9ec4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7367;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2b770000;
                    if t {
                        pc = 0x003e9f64;
                        continue;
                    }
                    pc = 0x003e9ed0;
                    continue;
                }
                0x003e9ed0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x51c));
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x51c));
                    if t {
                        pc = 0x003e9f60;
                        continue;
                    }
                    pc = 0x003e9edc;
                    continue;
                }
                0x003e9edc => {
                    c.cycles += 18;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[V0][0] = 0x63680000;
                    c.g[V1][0] = 0x1;
                    c.g[V0][0] = c.g[V0][0] | 0x6174;
                    c.g[A1][0] = 0x4a0000;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x100), c.g[V1][0]);
                    c.g[A0][0] = c.g[A2][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x104), c.g[V0][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49a8));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x108), 0u64);
                    c.g[RA][0] = sx(0x003e9f0c);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[A2][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003e9f0c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e9f18);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_FLAGS.call(c, 0x003e9f18)?;
                    c.g[V1][0] = c.g[V0][0] & 0x4;
                    let t = c.g[V1][0] == 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x108), c.g[V0][0]);
                    if t {
                        pc = 0x003e9f30;
                        continue;
                    }
                    pc = 0x003e9f24;
                    continue;
                }
                0x003e9f24 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x63610000;
                    c.g[V0][0] = c.g[V0][0] | 0x7374;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x104), c.g[V0][0]);
                    pc = 0x003e9f30;
                    continue;
                }
                0x003e9f30 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x108));
                    c.g[V1][0] = 0x10000;
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x70720000;
                    if t {
                        pc = 0x003e9f4c;
                        continue;
                    }
                    pc = 0x003e9f44;
                    continue;
                }
                0x003e9f44 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V0][0] | 0x6976;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x104), c.g[V0][0]);
                    pc = 0x003e9f4c;
                    continue;
                }
                0x003e9f4c => {
                    c.cycles += 5;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x51c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x003e9f60);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.pc = t;
                    call_indirect(c, t);
                    returned(c, 0x003e9f60)?;
                    pc = 0x003e9f60;
                    continue;
                }
                0x003e9f60 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x2b770000;
                    pc = 0x003e9f64;
                    continue;
                }
                0x003e9f64 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[V0][0] = c.g[V0][0] | 0x686f;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2b720000;
                    if t {
                        pc = 0x003ea0b0;
                        continue;
                    }
                    pc = 0x003e9f74;
                    continue;
                }
                0x003e9f74 => {
                    c.cycles += 38;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9f84);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49b0));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9f84)?;
                    c.g[V1][0] = 0x4a0000;
                    c.g[A2][0] = 0x40;
                    c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x174));
                    c.g[A3][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x003e9fa0);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9fa0)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9fb0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49a8));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9fb0)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003e9fbc);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_FLAGS.call(c, 0x003e9fbc)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49b8));
                    c.g[RA][0] = sx(0x003e9fd0);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1b4), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003e9fd0)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003e9fdc);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003e9fdc)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49c0));
                    c.g[RA][0] = sx(0x003e9ff0);
                    c.g[S0][0] = c.g[V0][0];
                    callee::TAG_FIELD_FIND.call(c, 0x003e9ff0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e9ffc);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003e9ffc)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1b8));
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        pc = 0x003ea0a8;
                        continue;
                    }
                    pc = 0x003ea00c;
                    continue;
                }
                0x003ea00c => {
                    c.cycles += 25;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1b8), c.g[S0][0]);
                    c.g[RA][0] = sx(0x003ea018);
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_API_UPDATE_ROOM_COUNT.call(c, 0x003ea018)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003ea028);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49c8));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea028)?;
                    c.g[A3][0] = c.g[S1][0];
                    c.g[A2][0] = 0x40;
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1c0));
                    c.g[RA][0] = sx(0x003ea03c);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ea03c)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003ea04c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49d0));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea04c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ea058);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_FLAGS.call(c, 0x003ea058)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x324));
                    c.g[V1][0] = c.g[V1][0] | 0x8;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1bc), c.g[V0][0]);
                    let t = c.g[A0][0] == 0u64;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    if t {
                        pc = 0x003ea0a8;
                        continue;
                    }
                    pc = 0x003ea070;
                    continue;
                }
                0x003ea070 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x003ea088;
                        continue;
                    }
                    pc = 0x003ea078;
                    continue;
                }
                0x003ea078 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003ea080);
                    c.g[A1][0] = c.g[S1][0];
                    callee::LOBBY_LIST_REMOVE.call(c, 0x003ea080)?;
                    c.g[RA][0] = sx(0x003ea088);
                    c.g[A0][0] = c.g[V0][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003ea088)?;
                    pc = 0x003ea088;
                    continue;
                }
                0x003ea088 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003ea090);
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x324));
                    callee::LOBBY_HASH_POP_ANY.call(c, 0x003ea090)?;
                    c.g[S1][0] = c.g[V0][0];

                    if c.g[S1][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x328));
                        if c.cycles >= c.next_event {
                            c.pc = 0x003ea078;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003ea078;
                        continue;
                    }
                    pc = 0x003ea09c;
                    continue;
                }
                0x003ea09c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[V0][0] | 0x40;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    pc = 0x003ea0a8;
                    continue;
                }
                0x003ea0a8 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x2b720000;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    pc = 0x003ea0b0;
                    continue;
                }
                0x003ea0b0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6f6d;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2b700000;
                    if t {
                        pc = 0x003ea35c;
                        continue;
                    }
                    pc = 0x003ea0bc;
                    continue;
                }
                0x003ea0bc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x310));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x4a0000;
                    if t {
                        pc = 0x003ea358;
                        continue;
                    }
                    pc = 0x003ea0c8;
                    continue;
                }
                0x003ea0c8 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[RA][0] = sx(0x003ea0d4);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49d8));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea0d4)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ea0e0);
                    c.g[A1][0] = 0xffffffffffffffff;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003ea0e0)?;
                    c.g[S2][0] = c.g[V0][0];
                    let t = (c.g[S2][0] as i64) < 0;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x003ea358;
                        continue;
                    }
                    pc = 0x003ea0ec;
                    continue;
                }
                0x003ea0ec => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x49b0));
                    c.g[RA][0] = sx(0x003ea0fc);
                    c.g[A1][0] = c.g[S0][0];
                    callee::TAG_FIELD_FIND.call(c, 0x003ea0fc)?;

                    if c.g[V0][0] == 0u64 {
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x310));
                        pc = 0x003ea30c;
                        continue;
                    }
                    pc = 0x003ea104;
                    continue;
                }
                0x003ea104 => {
                    c.cycles += 69;
                    c.g[RA][0] = sx(0x003ea10c);
                    c.g[A0][0] = 0x68;
                    callee::LOBBY_MEM_ALLOC.call(c, 0x003ea10c)?;
                    c.g[S4][0] = c.g[V0][0];
                    c.g[A2][0] = 0x68;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x003ea120);
                    c.g[A1][0] = 0u64;
                    callee::MEMSET.call(c, 0x003ea120)?;
                    c.sw(c.g[S4][0] as u32, c.g[S2][0]);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003ea130);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea130)?;
                    c.g[S0][0] = 0x4a0000;
                    c.g[A2][0] = 0x20;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1c));
                    c.g[A3][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003ea14c);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ea14c)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003ea15c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49e0));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea15c)?;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A2][0] = 0x20;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x3c));
                    c.g[RA][0] = sx(0x003ea170);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ea170)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003ea180);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49a8));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea180)?;
                    c.g[A1][0] = 0xffffffffffffffff;
                    c.g[RA][0] = sx(0x003ea18c);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_FLAGS.call(c, 0x003ea18c)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49e8));
                    c.g[RA][0] = sx(0x003ea1a0);
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003ea1a0)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003ea1ac);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_ADDRESS.call(c, 0x003ea1ac)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49f0));
                    c.g[RA][0] = sx(0x003ea1c0);
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x64), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003ea1c0)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003ea1cc);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003ea1cc)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49f8));
                    c.g[RA][0] = sx(0x003ea1e0);
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003ea1e0)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003ea1ec);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003ea1ec)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[S4][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    c.g[RA][0] = sx(0x003ea204);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4870));
                    call_ptr(c, 0x0041_7828, 0x003ea204)?;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = 0x10000;
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x18));
                    if t {
                        pc = 0x003ea224;
                        continue;
                    }
                    pc = 0x003ea218;
                    continue;
                }
                0x003ea218 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x4a0000;
                    let t = true;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4a00));
                    if t {
                        pc = 0x003ea22c;
                        continue;
                    }
                    pc = 0x003ea224;
                    continue;
                }
                0x003ea224 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4a08));
                    pc = 0x003ea22c;
                    continue;
                }
                0x003ea22c => {
                    c.cycles += 13;
                    c.g[RA][0] = sx(0x003ea234);
                    callee::STRCPY.call(c, 0x003ea234)?;
                    c.sb((c.g[S4][0] as u32).wrapping_add(0x5c), 0u64);
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a10));
                    c.g[RA][0] = sx(0x003ea248);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea248)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ea254);
                    c.g[A1][0] = 0xffffffffffffffff;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003ea254)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x003ea26c;
                        continue;
                    }
                    pc = 0x003ea260;
                    continue;
                }
                0x003ea260 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4a18));
                    let t = true;
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x5c), c.g[V1][0]);
                    if t {
                        pc = 0x003ea280;
                        continue;
                    }
                    pc = 0x003ea26c;
                    continue;
                }
                0x003ea26c => {
                    c.cycles += 2;
                    let t = (c.g[A2][0] as i64) <= 0;
                    c.g[A1][0] = 0x4a0000;
                    if t {
                        pc = 0x003ea280;
                        continue;
                    }
                    pc = 0x003ea274;
                    continue;
                }
                0x003ea274 => {
                    c.cycles += 3;
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5c));
                    c.g[RA][0] = sx(0x003ea280);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a20));
                    call_ptr(c, 0x0041_7828, 0x003ea280)?;
                    pc = 0x003ea280;
                    continue;
                }
                0x003ea280 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x310));
                    c.g[RA][0] = sx(0x003ea28c);
                    c.g[A1][0] = c.lw(c.g[S4][0] as u32);
                    callee::LOBBY_HASH_FIND_INT.call(c, 0x003ea28c)?;
                    c.g[S1][0] = c.g[V0][0];
                    let t = c.g[S1][0] == 0u64;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x314));
                    if t {
                        pc = 0x003ea2ec;
                        continue;
                    }
                    pc = 0x003ea298;
                    continue;
                }
                0x003ea298 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003ea2a0);
                    c.g[A1][0] = c.g[S1][0];
                    callee::LOBBY_LIST_GET_DATA.call(c, 0x003ea2a0)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;

                    if t {
                        pc = 0x003ea2cc;
                        continue;
                    }
                    pc = 0x003ea2ac;
                    continue;
                }
                0x003ea2ac => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x5c));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 48i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 58i64) as u64;
                    if t {
                        pc = 0x003ea2cc;
                        continue;
                    }
                    pc = 0x003ea2bc;
                    continue;
                }
                0x003ea2bc => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5c));
                    if t {
                        pc = 0x003ea2cc;
                        continue;
                    }
                    pc = 0x003ea2c4;
                    continue;
                }
                0x003ea2c4 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x003ea2cc);
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5c));
                    callee::STRCPY.call(c, 0x003ea2cc)?;
                    pc = 0x003ea2cc;
                    continue;
                }
                0x003ea2cc => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x003ea2d4);
                    c.g[A0][0] = c.g[S0][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003ea2d4)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x314));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x003ea2e4);
                    c.g[A2][0] = c.g[S4][0];
                    callee::LOBBY_LIST_UPDATE.call(c, 0x003ea2e4)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003ea350;
                        continue;
                    }
                    pc = 0x003ea2ec;
                    continue;
                }
                0x003ea2ec => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x003ea2f4);
                    c.g[A1][0] = c.g[S4][0];
                    callee::LOBBY_LIST_ADD.call(c, 0x003ea2f4)?;
                    c.g[A1][0] = c.lw(c.g[S4][0] as u32);
                    c.g[A2][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ea304);
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x310));
                    callee::LOBBY_HASH_ADD_INT.call(c, 0x003ea304)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003ea350;
                        continue;
                    }
                    pc = 0x003ea30c;
                    continue;
                }
                0x003ea30c => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003ea314);
                    c.g[A1][0] = c.g[S2][0];
                    callee::LOBBY_HASH_FIND_INT.call(c, 0x003ea314)?;
                    c.g[S1][0] = c.g[V0][0];

                    if c.g[S1][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        pc = 0x003ea350;
                        continue;
                    }
                    pc = 0x003ea320;
                    continue;
                }
                0x003ea320 => {
                    c.cycles += 12;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x310));
                    c.g[RA][0] = sx(0x003ea32c);
                    c.g[A1][0] = c.g[S2][0];
                    callee::LOBBY_HASH_DEL_INT.call(c, 0x003ea32c)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x314));
                    c.g[RA][0] = sx(0x003ea338);
                    c.g[A1][0] = c.g[S1][0];
                    callee::LOBBY_LIST_REMOVE.call(c, 0x003ea338)?;
                    c.g[RA][0] = sx(0x003ea340);
                    c.g[A0][0] = c.g[V0][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003ea340)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[V0][0] | 0x20;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    pc = 0x003ea350;
                    continue;
                }
                0x003ea350 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V0][0] | 0x20;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    pc = 0x003ea358;
                    continue;
                }
                0x003ea358 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x2b700000;
                    pc = 0x003ea35c;
                    continue;
                }
                0x003ea35c => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[V0][0] = c.g[V0][0] | 0x6f70;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2b750000;
                    if t {
                        pc = 0x003ea468;
                        continue;
                    }
                    pc = 0x003ea36c;
                    continue;
                }
                0x003ea36c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x310));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x4a0000;
                    if t {
                        pc = 0x003ea460;
                        continue;
                    }
                    pc = 0x003ea378;
                    continue;
                }
                0x003ea378 => {
                    c.cycles += 13;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[RA][0] = sx(0x003ea384);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a28));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea384)?;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
                    c.g[A3][0] = 0x4a0000;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003ea3a0);
                    c.g[A2][0] = 0x200;
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ea3a0)?;
                    c.g[V0][0] = c.lb((c.g[SP][0] as u32).wrapping_add(0x160));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x2b750000;
                    if t {
                        pc = 0x003ea464;
                        continue;
                    }
                    pc = 0x003ea3ac;
                    continue;
                }
                0x003ea3ac => {
                    c.cycles += 1;
                    c.g[A2][0] = c.lbu(c.g[S0][0] as u32);
                    pc = 0x003ea3b0;
                    continue;
                }
                0x003ea3b0 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S2][0] = 0u64;
                    if t {
                        pc = 0x003ea3e8;
                        continue;
                    }
                    pc = 0x003ea3c0;
                    continue;
                }
                0x003ea3c0 => {
                    c.cycles += 2;
                    c.g[A1][0] = 0xa;
                    pc = 0x003ea3c8;
                    continue;
                }
                0x003ea3c8 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[A2][0] & 0xf;
                    c.mult(0, 3, 18, 5);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A2][0] = c.lbu(c.g[S0][0] as u32);
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x003ea3c8;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003ea3c8;
                        continue;
                    }
                    pc = 0x003ea3e8;
                    continue;
                }
                0x003ea3e8 => {
                    c.cycles += 2;
                    let t = c.g[S2][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003ea460;
                        continue;
                    }
                    pc = 0x003ea3f0;
                    continue;
                }
                0x003ea3f0 => {
                    c.cycles += 8;
                    c.g[V1][0] = c.lb(c.g[S0][0] as u32);
                    c.g[A2][0] = 0u64;
                    if c.g[V1][0] != 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    c.g[A1][0] = c.lbu(c.g[S0][0] as u32);
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1b8));
                    if t {
                        pc = 0x003ea438;
                        continue;
                    }
                    pc = 0x003ea410;
                    continue;
                }
                0x003ea410 => {
                    c.cycles += 2;
                    c.g[A3][0] = 0xa;
                    pc = 0x003ea418;
                    continue;
                }
                0x003ea418 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[A1][0] & 0xf;
                    c.mult(0, 3, 6, 7);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = c.lbu(c.g[S0][0] as u32);
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x003ea418;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003ea418;
                        continue;
                    }
                    pc = 0x003ea438;
                    continue;
                }
                0x003ea438 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lb(c.g[S0][0] as u32);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    let t = c.g[S2][0] == c.g[T0][0];
                    if c.g[V1][0] != 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    if t {
                        pc = 0x003ea454;
                        continue;
                    }
                    pc = 0x003ea448;
                    continue;
                }
                0x003ea448 => {
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x003ea454);
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_API_UPDATE_ROOM_COUNT.call(c, 0x003ea454)?;
                    pc = 0x003ea454;
                    continue;
                }
                0x003ea454 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);

                    if c.g[V0][0] != 0u64 {
                        c.g[A2][0] = c.lbu(c.g[S0][0] as u32);
                        if c.cycles >= c.next_event {
                            c.pc = 0x003ea3b0;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003ea3b0;
                        continue;
                    }
                    pc = 0x003ea460;
                    continue;
                }
                0x003ea460 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x2b750000;
                    pc = 0x003ea464;
                    continue;
                }
                0x003ea464 => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    pc = 0x003ea468;
                    continue;
                }
                0x003ea468 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7372;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2b720000;
                    if t {
                        pc = 0x003ea790;
                        continue;
                    }
                    pc = 0x003ea474;
                    continue;
                }
                0x003ea474 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x324));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x4a0000;
                    if t {
                        pc = 0x003ea78c;
                        continue;
                    }
                    pc = 0x003ea480;
                    continue;
                }
                0x003ea480 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[RA][0] = sx(0x003ea48c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49d8));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea48c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ea498);
                    c.g[A1][0] = 0xffffffffffffffff;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003ea498)?;
                    c.g[S2][0] = c.g[V0][0];
                    let t = (c.g[S2][0] as i64) < 0;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x003ea78c;
                        continue;
                    }
                    pc = 0x003ea4a4;
                    continue;
                }
                0x003ea4a4 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x49b0));
                    c.g[RA][0] = sx(0x003ea4b4);
                    c.g[A1][0] = c.g[S0][0];
                    callee::TAG_FIELD_FIND.call(c, 0x003ea4b4)?;

                    if c.g[V0][0] == 0u64 {
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x324));
                        pc = 0x003ea720;
                        continue;
                    }
                    pc = 0x003ea4bc;
                    continue;
                }
                0x003ea4bc => {
                    c.cycles += 75;
                    c.g[RA][0] = sx(0x003ea4c4);
                    c.g[A0][0] = 0x138;
                    callee::LOBBY_MEM_ALLOC.call(c, 0x003ea4c4)?;
                    c.g[S4][0] = c.g[V0][0];
                    c.g[A2][0] = 0x138;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x003ea4d8);
                    c.g[A1][0] = 0u64;
                    callee::MEMSET.call(c, 0x003ea4d8)?;
                    c.sw(c.g[S4][0] as u32, c.g[S2][0]);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003ea4e8);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea4e8)?;
                    c.g[S0][0] = 0x4a0000;
                    c.g[A2][0] = 0x20;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003ea504);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ea504)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003ea514);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49e8));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea514)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003ea520);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_ADDRESS.call(c, 0x003ea520)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a10));
                    c.g[RA][0] = sx(0x003ea534);
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003ea534)?;
                    c.g[A2][0] = 0x8;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x28));
                    c.g[RA][0] = sx(0x003ea548);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ea548)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003ea558);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49a8));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea558)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003ea564);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_FLAGS.call(c, 0x003ea564)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a30));
                    c.g[RA][0] = sx(0x003ea578);
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003ea578)?;
                    c.g[A2][0] = 0x80;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x003ea58c);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ea58c)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003ea59c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49c8));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea59c)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003ea5a8);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003ea5a8)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a38));
                    c.g[RA][0] = sx(0x003ea5bc);
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003ea5bc)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0xb8));
                    c.g[RA][0] = sx(0x003ea5d0);
                    c.g[A2][0] = 0x80;
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ea5d0)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x324));
                    c.g[RA][0] = sx(0x003ea5dc);
                    c.g[A1][0] = c.lw(c.g[S4][0] as u32);
                    callee::LOBBY_HASH_FIND_INT.call(c, 0x003ea5dc)?;
                    c.g[S1][0] = c.g[V0][0];
                    let t = c.g[S1][0] == 0u64;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x328));
                    if t {
                        pc = 0x003ea6e8;
                        continue;
                    }
                    pc = 0x003ea5e8;
                    continue;
                }
                0x003ea5e8 => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x003ea5f0);
                    c.g[A1][0] = c.g[S1][0];
                    callee::LOBBY_LIST_GET_DATA.call(c, 0x003ea5f0)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V1][0] = c.g[S4][0];
                    c.g[V0][0] = c.g[S4][0] | c.g[S0][0];
                    c.g[V0][0] = c.g[V0][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x003ea664;
                        continue;
                    }
                    pc = 0x003ea608;
                    continue;
                }
                0x003ea608 => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x120));
                    c.g[S5][0] = 0x4a0000;
                    pc = 0x003ea610;
                    continue;
                }
                0x003ea610 => {
                    c.cycles += 19;
                    c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(5, c.g[V1][0] as u32);
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                    c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x1f));
                    c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(5, c.g[A0][0] as u32);
                    c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x17));
                    c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                    c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x003ea610;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003ea610;
                        continue;
                    }
                    pc = 0x003ea65c;
                    continue;
                }
                0x003ea65c => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x003ea698;
                        continue;
                    }
                    pc = 0x003ea664;
                    continue;
                }
                0x003ea664 => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x120));
                    c.g[S5][0] = 0x4a0000;
                    pc = 0x003ea66c;
                    continue;
                }
                0x003ea66c => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[V1][0] as u32);
                    c.g[A1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A0][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x003ea66c;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003ea66c;
                        continue;
                    }
                    pc = 0x003ea698;
                    continue;
                }
                0x003ea698 => {
                    c.cycles += 20;
                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(8, c.g[V1][0] as u32);
                    c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x17));
                    c.ldr(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                    c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(8, c.g[A0][0] as u32);
                    c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.sdr(9, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x17));
                    c.g[A2][0] = c.g[S0][0];
                    c.sdr(2, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x328));
                    c.g[RA][0] = sx(0x003ea6d8);
                    c.g[A1][0] = c.g[S1][0];
                    callee::LOBBY_LIST_UPDATE.call(c, 0x003ea6d8)?;
                    c.g[RA][0] = sx(0x003ea6e0);
                    c.g[A0][0] = c.g[S4][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003ea6e0)?;
                    let t = true;
                    c.g[S4][0] = c.g[S0][0];
                    if t {
                        pc = 0x003ea704;
                        continue;
                    }
                    pc = 0x003ea6e8;
                    continue;
                }
                0x003ea6e8 => {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x003ea6f4);
                    c.g[S5][0] = 0x4a0000;
                    callee::LOBBY_LIST_ADD.call(c, 0x003ea6f4)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x324));
                    c.g[A2][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ea704);
                    c.g[A1][0] = c.lw(c.g[S4][0] as u32);
                    callee::LOBBY_HASH_ADD_INT.call(c, 0x003ea704)?;
                    pc = 0x003ea704;
                    continue;
                }
                0x003ea704 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = 0x200000;
                    c.g[V0][0] = c.g[A0][0] & c.g[V0][0];

                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[S3][0] as u32).wrapping_add(0x1b4), c.g[A0][0]);
                        pc = 0x003ea758;
                        continue;
                    }
                    pc = 0x003ea718;
                    continue;
                }
                0x003ea718 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    if t {
                        pc = 0x003ea75c;
                        continue;
                    }
                    pc = 0x003ea720;
                    continue;
                }
                0x003ea720 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x003ea72c);
                    c.g[S5][0] = 0x4a0000;
                    callee::LOBBY_HASH_FIND_INT.call(c, 0x003ea72c)?;
                    c.g[S1][0] = c.g[V0][0];

                    if c.g[S1][0] == 0u64 {
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                        pc = 0x003ea75c;
                        continue;
                    }
                    pc = 0x003ea738;
                    continue;
                }
                0x003ea738 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x324));
                    c.g[RA][0] = sx(0x003ea744);
                    c.g[A1][0] = c.g[S2][0];
                    callee::LOBBY_HASH_DEL_INT.call(c, 0x003ea744)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x328));
                    c.g[RA][0] = sx(0x003ea750);
                    c.g[A1][0] = c.g[S1][0];
                    callee::LOBBY_LIST_REMOVE.call(c, 0x003ea750)?;
                    c.g[RA][0] = sx(0x003ea758);
                    c.g[A0][0] = c.g[V0][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003ea758)?;
                    pc = 0x003ea758;
                    continue;
                }
                0x003ea758 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    pc = 0x003ea75c;
                    continue;
                }
                0x003ea75c => {
                    c.cycles += 12;
                    c.g[RA][0] = sx(0x003ea764);
                    c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x49f0));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea764)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ea770);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003ea770)?;
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1b8));
                    c.g[A2][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ea780);
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_API_UPDATE_ROOM_COUNT.call(c, 0x003ea780)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[V0][0] | 0x40;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    pc = 0x003ea78c;
                    continue;
                }
                0x003ea78c => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x2b720000;
                    pc = 0x003ea790;
                    continue;
                }
                0x003ea790 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[V0][0] = c.g[V0][0] | 0x6e6b;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2b730000;
                    if t {
                        pc = 0x003eaa04;
                        continue;
                    }
                    pc = 0x003ea7a0;
                    continue;
                }
                0x003ea7a0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x338));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x4a0000;
                    if t {
                        pc = 0x003ea9fc;
                        continue;
                    }
                    pc = 0x003ea7ac;
                    continue;
                }
                0x003ea7ac => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[RA][0] = sx(0x003ea7b8);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a40));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea7b8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ea7c4);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003ea7c4)?;
                    c.g[V1][0] = 0xfffffffffffffffe;
                    let t = c.g[V0][0] != c.g[V1][0];

                    if t {
                        pc = 0x003ea804;
                        continue;
                    }
                    pc = 0x003ea7d0;
                    continue;
                }
                0x003ea7d0 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x003ea7e8;
                        continue;
                    }
                    pc = 0x003ea7d8;
                    continue;
                }
                0x003ea7d8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003ea7e0);
                    c.g[A1][0] = c.g[S1][0];
                    callee::LOBBY_LIST_REMOVE.call(c, 0x003ea7e0)?;
                    c.g[RA][0] = sx(0x003ea7e8);
                    c.g[A0][0] = c.g[V0][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003ea7e8)?;
                    pc = 0x003ea7e8;
                    continue;
                }
                0x003ea7e8 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003ea7f0);
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x338));
                    callee::LOBBY_HASH_POP_ANY.call(c, 0x003ea7f0)?;
                    c.g[S1][0] = c.g[V0][0];

                    if c.g[S1][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x33c));
                        if c.cycles >= c.next_event {
                            c.pc = 0x003ea7d8;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003ea7d8;
                        continue;
                    }
                    pc = 0x003ea7fc;
                    continue;
                }
                0x003ea7fc => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003ea9f4;
                        continue;
                    }
                    pc = 0x003ea804;
                    continue;
                }
                0x003ea804 => {
                    c.cycles += 37;
                    c.g[RA][0] = sx(0x003ea80c);
                    c.g[A0][0] = 0xa8;
                    callee::LOBBY_MEM_ALLOC.call(c, 0x003ea80c)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[S2][0] = c.g[V0][0];
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49e8));
                    c.g[RA][0] = sx(0x003ea828);
                    c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea828)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003ea834);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003ea834)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49b0));
                    c.g[RA][0] = sx(0x003ea848);
                    c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003ea848)?;
                    c.g[S0][0] = 0x4a0000;
                    c.g[A2][0] = 0x20;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A3][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003ea864);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ea864)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003ea874);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a30));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea874)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
                    c.g[RA][0] = sx(0x003ea888);
                    c.g[A2][0] = 0x80;
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ea888)?;
                    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[V1][0] = ((c.g[V1][0] as i64) < 8192i64) as u64;

                    if c.g[V1][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x338));
                        pc = 0x003ea8a4;
                        continue;
                    }
                    pc = 0x003ea898;
                    continue;
                }
                0x003ea898 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1fff;
                    c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x338));
                    pc = 0x003ea8a4;
                    continue;
                }
                0x003ea8a4 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003ea8ac);
                    c.g[A1][0] = c.g[S4][0];
                    callee::LOBBY_HASH_FIND_STR.call(c, 0x003ea8ac)?;
                    c.g[S1][0] = c.g[V0][0];
                    let t = c.g[S1][0] != 0u64;
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    if t {
                        pc = 0x003ea8e4;
                        continue;
                    }
                    pc = 0x003ea8b8;
                    continue;
                }
                0x003ea8b8 => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) <= 0;

                    if t {
                        pc = 0x003ea9e8;
                        continue;
                    }
                    pc = 0x003ea8c0;
                    continue;
                }
                0x003ea8c0 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x33c));
                    c.g[RA][0] = sx(0x003ea8cc);
                    c.g[A1][0] = c.g[S2][0];
                    callee::LOBBY_LIST_ADD.call(c, 0x003ea8cc)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x338));
                    c.g[A1][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x003ea8dc);
                    c.g[A2][0] = c.g[V0][0];
                    callee::LOBBY_HASH_ADD_STR.call(c, 0x003ea8dc)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003ea9f4;
                        continue;
                    }
                    pc = 0x003ea8e4;
                    continue;
                }
                0x003ea8e4 => {
                    c.cycles += 2;

                    if (c.g[V0][0] as i64) <= 0 {
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x338));
                        pc = 0x003ea9bc;
                        continue;
                    }
                    pc = 0x003ea8ec;
                    continue;
                }
                0x003ea8ec => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x33c));
                    c.g[RA][0] = sx(0x003ea8f8);
                    c.g[A1][0] = c.g[S1][0];
                    callee::LOBBY_LIST_GET_DATA.call(c, 0x003ea8f8)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[V1][0] = c.g[S2][0];
                    c.g[V0][0] = c.g[S2][0] | c.g[A2][0];
                    c.g[V0][0] = c.g[V0][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[A2][0];
                    if t {
                        pc = 0x003ea968;
                        continue;
                    }
                    pc = 0x003ea910;
                    continue;
                }
                0x003ea910 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xa0));
                    pc = 0x003ea914;
                    continue;
                }
                0x003ea914 => {
                    c.cycles += 19;
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(7, c.g[V1][0] as u32);
                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x17));
                    c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x10));
                    c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x1f));
                    c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(7, c.g[A0][0] as u32);
                    c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x17));
                    c.sdr(9, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x1f));
                    c.sdr(5, (c.g[A0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x003ea914;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003ea914;
                        continue;
                    }
                    pc = 0x003ea960;
                    continue;
                }
                0x003ea960 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x003ea998;
                        continue;
                    }
                    pc = 0x003ea968;
                    continue;
                }
                0x003ea968 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xa0));
                    pc = 0x003ea96c;
                    continue;
                }
                0x003ea96c => {
                    c.cycles += 11;
                    c.g[A3][0] = c.ld(c.g[V1][0] as u32);
                    c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A0][0] as u32, c.g[A3][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x003ea96c;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003ea96c;
                        continue;
                    }
                    pc = 0x003ea998;
                    continue;
                }
                0x003ea998 => {
                    c.cycles += 9;
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(7, c.g[V1][0] as u32);
                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.g[A1][0] = c.g[S1][0];
                    c.sdr(7, c.g[A0][0] as u32);
                    c.g[RA][0] = sx(0x003ea9b4);
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x33c));
                    callee::LOBBY_LIST_UPDATE.call(c, 0x003ea9b4)?;
                    let t = true;

                    if t {
                        pc = 0x003ea9d8;
                        continue;
                    }
                    pc = 0x003ea9bc;
                    continue;
                }
                0x003ea9bc => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x003ea9c4);
                    c.g[A1][0] = c.g[S4][0];
                    callee::LOBBY_HASH_DEL_STR.call(c, 0x003ea9c4)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x33c));
                    c.g[RA][0] = sx(0x003ea9d0);
                    c.g[A1][0] = c.g[V0][0];
                    callee::LOBBY_LIST_REMOVE.call(c, 0x003ea9d0)?;
                    c.g[RA][0] = sx(0x003ea9d8);
                    c.g[A0][0] = c.g[V0][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003ea9d8)?;
                    pc = 0x003ea9d8;
                    continue;
                }
                0x003ea9d8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003ea9e0);
                    c.g[A0][0] = c.g[S2][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003ea9e0)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003ea9f4;
                        continue;
                    }
                    pc = 0x003ea9e8;
                    continue;
                }
                0x003ea9e8 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x003ea9f0);
                    c.g[A0][0] = c.g[S2][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003ea9f0)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    pc = 0x003ea9f4;
                    continue;
                }
                0x003ea9f4 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V0][0] | 0x80;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    pc = 0x003ea9fc;
                    continue;
                }
                0x003ea9fc => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x2b730000;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    pc = 0x003eaa04;
                    continue;
                }
                0x003eaa04 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6e70;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x6d6f0000;
                    if t {
                        pc = 0x003eac54;
                        continue;
                    }
                    pc = 0x003eaa10;
                    continue;
                }
                0x003eaa10 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x508));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 4i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x6d6f0000;
                    if t {
                        pc = 0x003eac54;
                        continue;
                    }
                    pc = 0x003eaa20;
                    continue;
                }
                0x003eaa20 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 8i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S6][0] = 0x14;
                    if t {
                        pc = 0x003eac50;
                        continue;
                    }
                    pc = 0x003eaa2c;
                    continue;
                }
                0x003eaa2c => {
                    c.cycles += 6;
                    c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x310));
                    c.mult(0, 2, 3, 22);
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x6d6f0000;
                    if t {
                        pc = 0x003eac54;
                        continue;
                    }
                    pc = 0x003eaa44;
                    continue;
                }
                0x003eaa44 => {
                    c.cycles += 44;
                    c.g[RA][0] = sx(0x003eaa4c);
                    c.g[A0][0] = 0xa8;
                    callee::LOBBY_MEM_ALLOC.call(c, 0x003eaa4c)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003eaa60);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49c8));
                    callee::TAG_FIELD_FIND.call(c, 0x003eaa60)?;
                    c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003eaa70);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003eaa70)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a10));
                    c.g[RA][0] = sx(0x003eaa84);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003eaa84)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003eaa90);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003eaa90)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49b0));
                    c.g[RA][0] = sx(0x003eaaa4);
                    c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003eaaa4)?;
                    c.g[S0][0] = 0x4a0000;
                    c.g[A2][0] = 0x20;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A3][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003eaac0);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003eaac0)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003eaad0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a30));
                    callee::TAG_FIELD_FIND.call(c, 0x003eaad0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
                    c.g[RA][0] = sx(0x003eaae4);
                    c.g[A2][0] = 0x80;
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003eaae4)?;
                    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[V1][0] = ((c.g[V1][0] as i64) < 8192i64) as u64;

                    if c.g[V1][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x508));
                        pc = 0x003eab00;
                        continue;
                    }
                    pc = 0x003eaaf4;
                    continue;
                }
                0x003eaaf4 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1fff;
                    c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x508));
                    pc = 0x003eab00;
                    continue;
                }
                0x003eab00 => {
                    c.cycles += 8;
                    c.g[A1][0] = c.g[S4][0];
                    c.mult(0, 3, 2, 22);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[RA][0] = sx(0x003eab14);
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    callee::LOBBY_HASH_FIND_STR.call(c, 0x003eab14)?;
                    c.g[S1][0] = c.g[V0][0];
                    let t = c.g[S1][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x508));
                    if t {
                        pc = 0x003eab58;
                        continue;
                    }
                    pc = 0x003eab20;
                    continue;
                }
                0x003eab20 => {
                    c.cycles += 14;
                    c.g[A1][0] = c.g[S2][0];
                    c.mult(0, 3, 2, 22);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S3][0] as u32));
                    c.g[RA][0] = sx(0x003eab34);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x314));
                    callee::LOBBY_LIST_ADD.call(c, 0x003eab34)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x508));
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S4][0];
                    c.mult(0, 2, 3, 22);
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[RA][0] = sx(0x003eab50);
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    callee::LOBBY_HASH_ADD_STR.call(c, 0x003eab50)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003eac48;
                        continue;
                    }
                    pc = 0x003eab58;
                    continue;
                }
                0x003eab58 => {
                    c.cycles += 12;
                    c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x314));
                    c.g[A1][0] = c.g[S1][0];
                    c.mult(0, 2, 2, 22);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[RA][0] = sx(0x003eab70);
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    callee::LOBBY_LIST_GET_DATA.call(c, 0x003eab70)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[V1][0] = c.g[S2][0];
                    c.g[V0][0] = c.g[S2][0] | c.g[A2][0];
                    c.g[V0][0] = c.g[V0][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[A2][0];
                    if t {
                        pc = 0x003eabe0;
                        continue;
                    }
                    pc = 0x003eab88;
                    continue;
                }
                0x003eab88 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xa0));
                    pc = 0x003eab8c;
                    continue;
                }
                0x003eab8c => {
                    c.cycles += 19;
                    c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(5, c.g[V1][0] as u32);
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x17));
                    c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x10));
                    c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x1f));
                    c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(5, c.g[A0][0] as u32);
                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x17));
                    c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x1f));
                    c.sdr(9, (c.g[A0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x003eab8c;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003eab8c;
                        continue;
                    }
                    pc = 0x003eabd8;
                    continue;
                }
                0x003eabd8 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x003eac10;
                        continue;
                    }
                    pc = 0x003eabe0;
                    continue;
                }
                0x003eabe0 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xa0));
                    pc = 0x003eabe4;
                    continue;
                }
                0x003eabe4 => {
                    c.cycles += 11;
                    c.g[A1][0] = c.ld(c.g[V1][0] as u32);
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A0][0] as u32, c.g[A1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x003eabe4;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003eabe4;
                        continue;
                    }
                    pc = 0x003eac10;
                    continue;
                }
                0x003eac10 => {
                    c.cycles += 14;
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[V1][0] as u32);
                    c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(2, c.g[A0][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x508));
                    c.g[V1][0] = 0x14;
                    c.g[A1][0] = c.g[S1][0];
                    c.mult(0, 4, 2, 3);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[RA][0] = sx(0x003eac3c);
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    callee::LOBBY_LIST_UPDATE.call(c, 0x003eac3c)?;
                    c.g[RA][0] = sx(0x003eac44);
                    c.g[A0][0] = c.g[S2][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003eac44)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    pc = 0x003eac48;
                    continue;
                }
                0x003eac48 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V0][0] | 0x80;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    pc = 0x003eac50;
                    continue;
                }
                0x003eac50 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x6d6f0000;
                    pc = 0x003eac54;
                    continue;
                }
                0x003eac54 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[V0][0] = c.g[V0][0] | 0x7665;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x726f0000;
                    if t {
                        pc = 0x003eac70;
                        continue;
                    }
                    pc = 0x003eac64;
                    continue;
                }
                0x003eac64 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6f6d;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x6d6f0000;
                    if t {
                        pc = 0x003eacd4;
                        continue;
                    }
                    pc = 0x003eac70;
                    continue;
                }
                0x003eac70 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x6d6f0000;
                    if t {
                        pc = 0x003eacd0;
                        continue;
                    }
                    pc = 0x003eac7c;
                    continue;
                }
                0x003eac7c => {
                    c.cycles += 10;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003eac8c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a48));
                    callee::TAG_FIELD_FIND.call(c, 0x003eac8c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003eac98);
                    c.g[A1][0] = 0xffffffffffffffff;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003eac98)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = (c.g[S0][0] as i64) < 0;
                    c.g[A1][0] = 0x4a0000;
                    if t {
                        pc = 0x003eaccc;
                        continue;
                    }
                    pc = 0x003eaca4;
                    continue;
                }
                0x003eaca4 => {
                    c.cycles += 10;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[RA][0] = sx(0x003eacb0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a50));
                    callee::TAG_FIELD_FIND.call(c, 0x003eacb0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003eacbc);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003eacbc)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003eaccc);
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_API_UPDATE_ROOM_COUNT.call(c, 0x003eaccc)?;
                    pc = 0x003eaccc;
                    continue;
                }
                0x003eaccc => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x6d6f0000;
                    pc = 0x003eacd0;
                    continue;
                }
                0x003eacd0 => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    pc = 0x003eacd4;
                    continue;
                }
                0x003eacd4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7665;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                        pc = 0x003eadd4;
                        continue;
                    }
                    pc = 0x003eace0;
                    continue;
                }
                0x003eace0 => {
                    c.cycles += 19;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003eacf0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a58));
                    callee::TAG_FIELD_FIND.call(c, 0x003eacf0)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003eacfc);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003eacfc)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a60));
                    c.g[RA][0] = sx(0x003ead10);
                    c.g[S0][0] = c.g[V0][0];
                    callee::TAG_FIELD_FIND.call(c, 0x003ead10)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ead1c);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003ead1c)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1b8));
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        pc = 0x003eadd0;
                        continue;
                    }
                    pc = 0x003ead2c;
                    continue;
                }
                0x003ead2c => {
                    c.cycles += 26;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1b8), c.g[S0][0]);
                    c.g[RA][0] = sx(0x003ead38);
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_API_UPDATE_ROOM_COUNT.call(c, 0x003ead38)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003ead48);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4880));
                    callee::TAG_FIELD_FIND.call(c, 0x003ead48)?;
                    c.g[A3][0] = 0x4a0000;
                    c.g[A2][0] = 0x40;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1c0));
                    c.g[RA][0] = sx(0x003ead60);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ead60)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003ead70);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a68));
                    callee::TAG_FIELD_FIND.call(c, 0x003ead70)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ead7c);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_FLAGS.call(c, 0x003ead7c)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x324));
                    c.g[V1][0] = c.g[V1][0] | 0x8;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1bc), c.g[V0][0]);
                    let t = c.g[A0][0] == 0u64;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    if t {
                        pc = 0x003eadd0;
                        continue;
                    }
                    pc = 0x003ead94;
                    continue;
                }
                0x003ead94 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x003eadb0;
                        continue;
                    }
                    pc = 0x003ead9c;
                    continue;
                }
                0x003ead9c => {
                    c.cycles += 1;
                    pc = 0x003eada0;
                    continue;
                }
                0x003eada0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003eada8);
                    c.g[A1][0] = c.g[S1][0];
                    callee::LOBBY_LIST_REMOVE.call(c, 0x003eada8)?;
                    c.g[RA][0] = sx(0x003eadb0);
                    c.g[A0][0] = c.g[V0][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003eadb0)?;
                    pc = 0x003eadb0;
                    continue;
                }
                0x003eadb0 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003eadb8);
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x324));
                    callee::LOBBY_HASH_POP_ANY.call(c, 0x003eadb8)?;
                    c.g[S1][0] = c.g[V0][0];

                    if c.g[S1][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x328));
                        if c.cycles >= c.next_event {
                            c.pc = 0x003eada0;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003eada0;
                        continue;
                    }
                    pc = 0x003eadc4;
                    continue;
                }
                0x003eadc4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[V0][0] | 0x40;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    pc = 0x003eadd0;
                    continue;
                }
                0x003eadd0 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    pc = 0x003eadd4;
                    continue;
                }
                0x003eadd4 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x003eade8);
                    c.g[T0][0] = 0u64;
                    callee::PROTO_ARIES_RECV.call(c, 0x003eade8)?;
                    pc = 0x003eade8;
                    continue;
                }
                // Peek the next message; empty means fall out of the loop.
                0x003eade8 => {
                    c.cycles += 8;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x36c));
                    c.g[A1][0] = c.g[FP][0];
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[RA][0] = sx(0x003eadfc);
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x370));
                    callee::PROTO_ARIES_PEEK.call(c, 0x003eadfc)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = (c.g[A2][0] as i64) >= 0;
                    c.g[V0][0] = 0xea60;
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x003e96b0;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003e96b0;
                        continue;
                    }
                    pc = 0x003eae08;
                    continue;
                }
                // Lists are sorted; re-arm the server ping when due.
                0x003eae08 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003eae10);
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_API_SORT_LISTS.call(c, 0x003eae10)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = (c.g[S7][0] < c.g[V0][0]) as u64;
                    if t {
                        pc = 0x003eae34;
                        continue;
                    }
                    pc = 0x003eae1c;
                    continue;
                }
                0x003eae1c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x410));
                    if t {
                        pc = 0x003eae38;
                        continue;
                    }
                    pc = 0x003eae24;
                    continue;
                }
                0x003eae24 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003eae2c);
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_API_PING_SERVER.call(c, 0x003eae2c)?;
                    c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x7530));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    pc = 0x003eae34;
                    continue;
                }
                // Epilogue: restore the frame and return to the caller.
                0x003eae34 => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x410));
                    pc = 0x003eae38;
                    continue;
                }
                0x003eae38 => {
                    c.cycles += 11;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x400));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x3f0));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x3e0));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x3d0));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x3c0));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x3b0));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x3a0));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x390));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x380));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x420));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003e95c8, 0x003eae64);
                    return Ok(());
                }
            }
        }
    })
}

pub(crate) mod callee {
    pub(crate) use crate::game::dirtysock::buddyapi::callee::{
        LOBBY_HASH_ADD_STR, LOBBY_HASH_DEL_STR, LOBBY_HASH_FIND_STR, LOBBY_HASH_POP_ANY,
        LOBBY_LIST_ADD, LOBBY_LIST_GET_DATA, LOBBY_LIST_REMOVE, LOBBY_MEM_ALLOC, LOBBY_MEM_FREE,
        MEMSET, PROTO_ARIES_CONNECT, PROTO_ARIES_PEEK, PROTO_ARIES_RECV, PROTO_ARIES_SEND,
        PROTO_ARIES_TICK, PROTO_ARIES_UNCONNECT, STRCPY, TAG_FIELD_FIND, TAG_FIELD_GET_STRING,
        TAG_FIELD_SET_NUMBER, TAG_FIELD_STRNZCPY,
    };
    pub(crate) use crate::game::dirtysock::lobbyapi::callee::{
        LOBBY_API_DEBUG_PRINTF, LOBBY_API_POP_REQUEST, LOBBY_HASH_FIND_INT, LOBBY_LIST_UPDATE,
        TAG_FIELD_GET_ADDRESS,
    };
    pub(crate) use crate::game::dirtysock::tags::callee::TAG_FIELD_SET_ADDRESS;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::TAG_FIELD_GET_NUMBER;

    use super::ee;
    use super::Callee;

    pub(crate) const LOBBY_API_TERMINATE: Callee = Callee {
        addr: 0x003e_7f70,
        f: ee::s14::LobbyApi_Terminate,
    };
    pub(crate) const LOBBY_API_PROCESS_PINGS: Callee = Callee {
        addr: 0x003e_7c40,
        f: ee::s14::LobbyApi_ProcessPings,
    };
    pub(crate) const LOBBY_API_UPDATE_ROOM_COUNT: Callee = Callee {
        addr: 0x003e_7b00,
        f: ee::s14::LobbyApi_UpdateRoomCount,
    };
    pub(crate) const LOBBY_API_PING_SERVER: Callee = Callee {
        addr: 0x003e_9590,
        f: ee::s14::LobbyApi_PingServer,
    };
    pub(crate) const LOBBY_API_SORT_LISTS: Callee = Callee {
        addr: 0x003e_7ba0,
        f: ee::s14::LobbyApi_SortLists,
    };
    pub(crate) const PROTO_ARIES_STATUS: Callee = Callee {
        addr: 0x003f_57c8,
        f: ee::s14::ProtoAriesStatus,
    };
    pub(crate) const TAG_FIELD_SET_BINARY: Callee = Callee {
        addr: 0x003e_c300,
        f: ee::s14::TagFieldSetBinary,
    };
    pub(crate) const TAG_FIELD_DECODE_HEX: Callee = Callee {
        addr: 0x003e_d578,
        f: ee::s14::TagFieldDecodeHex,
    };
    pub(crate) const TAG_FIELD_GET_FLAGS: Callee = Callee {
        addr: 0x003e_cfb0,
        f: ee::s14::TagFieldGetFlags,
    };
    pub(crate) const TAG_FIELD_GET_DATE: Callee = Callee {
        addr: 0x003e_db08,
        f: ee::s14::TagFieldGetDate,
    };
    pub(crate) const LOBBY_HASH_ADD_INT: Callee = Callee {
        addr: 0x003e_eda0,
        f: ee::s14::Lobby_HashAddInt,
    };
    pub(crate) const LOBBY_HASH_DEL_INT: Callee = Callee {
        addr: 0x003e_ef40,
        f: ee::s14::Lobby_HashDelInt,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes;
    use crate::difftest::{
        check, check_paths, elf_image, fresh, Rng, DATA, GAME_GP, RA as TEST_RA,
    };
    use ssx3_ee as ee;
    use ssx3_ee_rt::cpu::reg::*;
    use ssx3_ee_rt::Cpu;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Seeds per row (every row runs its setup on each seed, both sides).
    const SEEDS: u32 = 24;
    /// Function entry (every full-pump test starts here).
    const ENTRY: u32 = 0x003e_95c8;
    /// Resume head at the message dispatcher (message rows start here).
    const DISPATCH: u32 = 0x003e_9728;
    /// Resume head at the ping-reply check (the tagged-`~png` row starts here).
    const PING_CHECK: u32 = 0x003e_96b0;
    /// Resume head at the chat-message arm (the `+msg` row starts here;
    /// DISPATCH never routes there in the test fixtures).
    const CHAT_ARM: u32 = 0x003e_9c98;
    /// Resume head at the room-name-list arm (the `+rnl` rows start here;
    /// DISPATCH never routes there in the test fixtures).
    const RNL_ARM: u32 = 0x003e_a7a0;

    /// Lobby client block under test (0x600 bytes).
    const API: u32 = DATA;
    /// Tag area the scripted peek hands out (never parsed: tag reads are faked).
    const TAGPTR: u32 = DATA + 0x0c00;
    /// Readable node the list-data fake hands out.
    const NODE: u32 = DATA + 0x1800;
    /// Bump pool for the allocator fakes.
    const POOL: u32 = DATA + 0x2000;

    /// Scripted-result cells (all `u32`, set per row).
    const CELLS: u32 = DATA + 0x1000;
    /// Gate word: the fakes below run only while it holds the magic.
    const CTRL: u32 = CELLS + 0x00;
    const MAGIC: u32 = 0x0c13_5101;
    const TICK: u32 = CELLS + 0x04;
    const BASE_TICK: u32 = 0x0010_0000;
    const PEEK_RET: u32 = CELLS + 0x08;
    const PEEK_N: u32 = CELLS + 0x0c;
    const PEEK_TYPE: u32 = CELLS + 0x10;
    const PEEK_BC: u32 = CELLS + 0x14;
    const PEEK_PTR: u32 = CELLS + 0x18;
    const FIND: u32 = CELLS + 0x1c;
    const GETNUM: u32 = CELLS + 0x20;
    const GETADDR: u32 = CELLS + 0x24;
    const GETFLAGS: u32 = CELLS + 0x28;
    const GETDATE: u32 = CELLS + 0x2c;
    const POP_RET: u32 = CELLS + 0x30;
    const HFIND: u32 = CELLS + 0x34;
    const HPOP: u32 = CELLS + 0x38;
    const HPOP_N: u32 = CELLS + 0x3c;
    const HPOP_LIMIT: u32 = CELLS + 0x40;
    const LADD: u32 = CELLS + 0x44;
    const LREM: u32 = CELLS + 0x48;
    const STATUS_RET: u32 = CELLS + 0x4c;
    const BUMP: u32 = CELLS + 0x50;
    /// Bytes the string getter hands out (fixture-set per row, NUL-padded).
    const STRING_SRC: u32 = CELLS + 0x100;
    /// Bytes the hex decoder hands out.
    const HEX_SRC: u32 = CELLS + 0x180;
    /// Per-fake call counters (asserted nonzero to prove a branch ran).
    const CNT: u32 = CELLS + 0x200;
    const N_TICK: u32 = 0;
    const N_PEEK: u32 = 1;
    const N_RECV: u32 = 2;
    const N_SEND: u32 = 3;
    const N_STATUS: u32 = 4;
    const N_UNCONN: u32 = 5;
    const N_CONN: u32 = 6;
    const N_TERM: u32 = 7;
    const N_PINGS: u32 = 8;
    const N_POP: u32 = 9;
    const N_SORT: u32 = 10;
    const N_PINGSRV: u32 = 11;
    const N_ROOMCNT: u32 = 12;
    const N_PRINTF: u32 = 13;
    const N_FIND: u32 = 14;
    const N_GETNUM: u32 = 15;
    const N_GETSTR: u32 = 16;
    const N_GETADDR: u32 = 17;
    const N_GETFLAGS: u32 = 18;
    const N_GETDATE: u32 = 19;
    const N_DECHEX: u32 = 20;
    const N_SETBIN: u32 = 21;
    const N_SETNUM: u32 = 22;
    const N_SETADDR: u32 = 23;
    const N_STRNZ: u32 = 24;
    const N_HFINDI: u32 = 25;
    const N_HDELI: u32 = 26;
    const N_HADDI: u32 = 27;
    const N_HFINDS: u32 = 28;
    const N_HADDS: u32 = 29;
    const N_HDELS: u32 = 30;
    const N_HPOP: u32 = 31;
    const N_LADD: u32 = 32;
    const N_LREM: u32 = 33;
    const N_LDATA: u32 = 34;
    const N_LUPD: u32 = 35;
    const N_EA_NEW: u32 = 36;
    const N_EA_FREE: u32 = 37;

    /// Client state tags (word at `+0x0c`).
    const ST_OFFL: u32 = 0x6f66_666c;
    const ST_TERM: u32 = 0x7465_726d;
    const ST_SKEY: u32 = 0x736b_6579;
    const ST_IDLE: u32 = 0x6964_6c65;
    const ST_CONN: u32 = 0x636f_6e6e;
    const ST_RDIR: u32 = 0x7264_6972;
    const ST_AUTH: u32 = 0x6175_7468;
    const ST_ACCT: u32 = 0x6163_6374;
    /// Message types (word at frame `+0x360`).
    const MSG_PING: u32 = 0x7e70_6e67;
    const MSG_DIR: u32 = 0x4064_6972;
    const MSG_SELE: u32 = 0x7365_6c65;
    const MSG_SNAP: u32 = 0x736e_6170;
    const MSG_PERS: u32 = 0x7065_7273;
    const MSG_SESS: u32 = 0x2b73_6573;
    const MSG_MSG: u32 = 0x2b6d_7367;
    const MSG_WHO: u32 = 0x2b77_686f;
    const MSG_ROM: u32 = 0x2b72_6f6d;
    const MSG_POP: u32 = 0x2b70_6f70;
    const MSG_USR: u32 = 0x2b75_7372;
    const MSG_RNL: u32 = 0x2b72_6e6c;
    const MSG_SNP: u32 = 0x2b73_6e70;
    const MSG_MOVE: u32 = 0x6d6f_7665;
    const MSG_ROOM: u32 = 0x726f_6f6d;
    const MSG_AUTH: u32 = 0x6175_7468;
    const MSG_ACCT: u32 = 0x6163_6374;
    /// Chat callback leaf: returns 0, touches nothing (0x1406c0).
    const LEAF: u32 = 0x0014_06c0;
    /// Reads the room word back (`+0x6e` is a word index).
    fn room(c: &mut Cpu) -> u32 {
        c.bus.read32(API + 4 * 0x6e)
    }
    /// Reads the event-flags word back.
    fn flags(c: &mut Cpu) -> u32 {
        c.bus.read32(API + 0x08)
    }
    /// Reads the state tag back.
    fn state(c: &mut Cpu) -> u32 {
        c.bus.read32(API + 0x0c)
    }
    /// How many times fake `i` ran.
    fn hits(c: &mut Cpu, i: u32) -> u32 {
        c.bus.read32(CNT + 4 * i)
    }

    fn oc1351_gate(c: &mut Cpu) -> bool {
        c.bus.read32(CTRL) == MAGIC
    }
    fn oc1351_done(c: &mut Cpu, v0: u64, cycles: u64) {
        c.g[V0][0] = v0;
        c.cycles += cycles;
        c.pc = c.g[RA][0] as u32;
    }
    fn oc1351_bump(c: &mut Cpu, i: u32) {
        let a = CNT + 4 * i;
        let v = c.bus.read32(a) + 1;
        c.bus.write32(a, v);
    }

    fn oc1351_fake_tick(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_TICK, ee::s14::ProtoAriesTick);
        }
        oc1351_bump(c, N_TICK);
        let v0 = c.bus.read32(TICK) as u64;
        oc1351_done(c, v0, 7);
    }
    static OC1351_PREV_TICK: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_peek(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_PEEK, ee::s14::ProtoAriesPeek);
        }
        oc1351_bump(c, N_PEEK);
        let n = c.bus.read32(PEEK_N);
        c.bus.write32(PEEK_N, n + 1);
        if n == 0 {
            let sp = c.g[SP][0] as u32;
            let ty = c.bus.read32(PEEK_TYPE);
            let bc = c.bus.read32(PEEK_BC);
            let pp = c.bus.read32(PEEK_PTR);
            c.bus.write32(sp.wrapping_add(0x360), ty);
            c.bus.write32(sp.wrapping_add(0x364), bc);
            c.bus.write32(sp.wrapping_add(0x368), pp);
            let v0 = c.bus.read32(PEEK_RET) as u64;
            oc1351_done(c, v0, 9);
        } else {
            oc1351_done(c, 0xffff_ffff_ffff_ffff, 9);
        }
    }
    static OC1351_PREV_PEEK: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_recv(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_RECV, ee::s14::ProtoAriesRecv);
        }
        oc1351_bump(c, N_RECV);
        oc1351_done(c, 0, 5);
    }
    static OC1351_PREV_RECV: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_send(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_SEND, ee::s14::ProtoAriesSend);
        }
        oc1351_bump(c, N_SEND);
        oc1351_done(c, 0, 9);
    }
    static OC1351_PREV_SEND: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_status(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_STATUS, ee::s14::ProtoAriesStatus);
        }
        oc1351_bump(c, N_STATUS);
        let v0 = c.bus.read32(STATUS_RET) as u64;
        oc1351_done(c, v0, 7);
    }
    static OC1351_PREV_STATUS: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_unconnect(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_UNCONN, ee::s14::ProtoAriesUnconnect);
        }
        oc1351_bump(c, N_UNCONN);
        oc1351_done(c, 0, 5);
    }
    static OC1351_PREV_UNCONN: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_connect(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_CONN, ee::s14::ProtoAriesConnect);
        }
        oc1351_bump(c, N_CONN);
        oc1351_done(c, 0, 5);
    }
    static OC1351_PREV_CONN: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_terminate(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_TERM, ee::s14::LobbyApi_Terminate);
        }
        oc1351_bump(c, N_TERM);
        c.bus.write32(API + 0x0c, ST_TERM);
        oc1351_done(c, 0, 9);
    }
    static OC1351_PREV_TERM: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_pings(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_PINGS, ee::s14::LobbyApi_ProcessPings);
        }
        oc1351_bump(c, N_PINGS);
        oc1351_done(c, 0, 11);
    }
    static OC1351_PREV_PINGS: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_pop(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_POP, ee::s14::LobbyApi_PopRequest);
        }
        oc1351_bump(c, N_POP);
        let out = c.g[A1][0] as u32;
        let h = c.bus.read32(POP_RET);
        c.bus.write32(out + 0x38, h);
        oc1351_done(c, h as u64, 11);
    }
    static OC1351_PREV_POP: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_sort(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_SORT, ee::s14::LobbyApi_SortLists);
        }
        oc1351_bump(c, N_SORT);
        oc1351_done(c, 0, 7);
    }
    static OC1351_PREV_SORT: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_pingserver(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_PINGSRV, ee::s14::LobbyApi_PingServer);
        }
        oc1351_bump(c, N_PINGSRV);
        oc1351_done(c, 0, 7);
    }
    static OC1351_PREV_PINGSRV: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_roomcount(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_ROOMCNT, ee::s14::LobbyApi_UpdateRoomCount);
        }
        oc1351_bump(c, N_ROOMCNT);
        oc1351_done(c, 0, 7);
    }
    static OC1351_PREV_ROOMCNT: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_printf(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_PRINTF, ee::s14::LobbyApi_DebugPrintf);
        }
        oc1351_bump(c, N_PRINTF);
        oc1351_done(c, 0, 5);
    }
    static OC1351_PREV_PRINTF: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_find(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_FIND, ee::s14::TagFieldFind);
        }
        oc1351_bump(c, N_FIND);
        let v0 = c.bus.read32(FIND) as u64;
        oc1351_done(c, v0, 6);
    }
    static OC1351_PREV_FIND: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_getnum(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_GETNUM, ee::s14::TagFieldGetNumber);
        }
        oc1351_bump(c, N_GETNUM);
        // Sign-extended: the game compares the result against sign-extended
        // sentinels (e.g. -2 for "remove all").
        let v0 = c.bus.read32(GETNUM) as i32 as u64;
        oc1351_done(c, v0, 6);
    }
    static OC1351_PREV_GETNUM: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_getstr(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_GETSTR, ee::s14::TagFieldGetString);
        }
        oc1351_bump(c, N_GETSTR);
        let dst = c.g[A1][0] as u32;
        let max = c.g[A2][0] as u32;
        let n = (max as usize).min(64);
        let words = n / 4;
        for i in 0..words {
            let w = c.bus.read32(STRING_SRC + 4 * i as u32);
            c.bus.write32(dst + 4 * i as u32, w);
        }
        for i in 4 * words..n {
            let b = (c.bus.read32(STRING_SRC + (i as u32 & !3)) >> (8 * (i as u32 & 3))) as u8;
            c.bus.write8(dst + i as u32, b);
        }
        oc1351_done(c, 0, 8);
    }
    static OC1351_PREV_GETSTR: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_getaddr(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_GETADDR, ee::s14::TagFieldGetAddress);
        }
        oc1351_bump(c, N_GETADDR);
        let v0 = c.bus.read32(GETADDR) as u64;
        oc1351_done(c, v0, 6);
    }
    static OC1351_PREV_GETADDR: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_getflags(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_GETFLAGS, ee::s14::TagFieldGetFlags);
        }
        oc1351_bump(c, N_GETFLAGS);
        let v0 = c.bus.read32(GETFLAGS) as u64;
        oc1351_done(c, v0, 6);
    }
    static OC1351_PREV_GETFLAGS: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_getdate(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_GETDATE, ee::s14::TagFieldGetDate);
        }
        oc1351_bump(c, N_GETDATE);
        let v0 = c.bus.read32(GETDATE) as u64;
        oc1351_done(c, v0, 6);
    }
    static OC1351_PREV_GETDATE: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_dechex(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_DECHEX, ee::s14::TagFieldDecodeHex);
        }
        oc1351_bump(c, N_DECHEX);
        let dst = c.g[A1][0] as u32;
        for i in 0..4u32 {
            let w = c.bus.read32(HEX_SRC + 4 * i);
            c.bus.write32(dst + 4 * i, w);
        }
        oc1351_done(c, 0, 8);
    }
    static OC1351_PREV_DECHEX: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_setbin(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_SETBIN, ee::s14::TagFieldSetBinary);
        }
        oc1351_bump(c, N_SETBIN);
        oc1351_done(c, 0, 7);
    }
    static OC1351_PREV_SETBIN: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_setnum(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_SETNUM, ee::s14::TagFieldSetNumber);
        }
        oc1351_bump(c, N_SETNUM);
        oc1351_done(c, 0, 6);
    }
    static OC1351_PREV_SETNUM: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_setaddr(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_SETADDR, ee::s14::TagFieldSetAddress);
        }
        oc1351_bump(c, N_SETADDR);
        oc1351_done(c, 0, 6);
    }
    static OC1351_PREV_SETADDR: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_strnz(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_STRNZ, ee::s14::TagFieldStrnzcpy);
        }
        oc1351_bump(c, N_STRNZ);
        let dst = c.g[A0][0] as u32;
        let src = c.g[A2][0] as u32;
        for i in 0..8u32 {
            let b = (c.bus.read32(src + (i & !3)) >> (8 * (i & 3))) as u8;
            c.bus.write8(dst + i, b);
        }
        c.bus.write8(dst + 8, 0);
        oc1351_done(c, 0, 6);
    }
    static OC1351_PREV_STRNZ: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_hfindi(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_HFINDI, ee::s14::Lobby_HashFindInt);
        }
        oc1351_bump(c, N_HFINDI);
        let v0 = c.bus.read32(HFIND) as u64;
        oc1351_done(c, v0, 6);
    }
    static OC1351_PREV_HFINDI: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_hdeli(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_HDELI, ee::s14::Lobby_HashDelInt);
        }
        oc1351_bump(c, N_HDELI);
        oc1351_done(c, 0, 6);
    }
    static OC1351_PREV_HDELI: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_haddi(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_HADDI, ee::s14::Lobby_HashAddInt);
        }
        oc1351_bump(c, N_HADDI);
        oc1351_done(c, 0, 6);
    }
    static OC1351_PREV_HADDI: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_hfinds(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_HFINDS, ee::s14::Lobby_HashFindStr);
        }
        oc1351_bump(c, N_HFINDS);
        let v0 = c.bus.read32(HFIND) as u64;
        oc1351_done(c, v0, 6);
    }
    static OC1351_PREV_HFINDS: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_hadds(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_HADDS, ee::s14::Lobby_HashAddStr);
        }
        oc1351_bump(c, N_HADDS);
        oc1351_done(c, 0, 6);
    }
    static OC1351_PREV_HADDS: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_hdels(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_HDELS, ee::s14::Lobby_HashDelStr);
        }
        oc1351_bump(c, N_HDELS);
        oc1351_done(c, 0, 6);
    }
    static OC1351_PREV_HDELS: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_hpop(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_HPOP, ee::s14::Lobby_HashPopAny);
        }
        oc1351_bump(c, N_HPOP);
        let n = c.bus.read32(HPOP_N);
        c.bus.write32(HPOP_N, n + 1);
        if n < c.bus.read32(HPOP_LIMIT) {
            let v0 = c.bus.read32(HPOP) as u64;
            oc1351_done(c, v0, 6);
        } else {
            oc1351_done(c, 0, 6);
        }
    }
    static OC1351_PREV_HPOP: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_ladd(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_LADD, ee::s14::Lobby_ListAdd);
        }
        oc1351_bump(c, N_LADD);
        let v0 = c.bus.read32(LADD) as u64;
        oc1351_done(c, v0, 7);
    }
    static OC1351_PREV_LADD: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_lrem(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_LREM, ee::s14::Lobby_ListRemove);
        }
        oc1351_bump(c, N_LREM);
        let v0 = c.bus.read32(LREM) as u64;
        oc1351_done(c, v0, 7);
    }
    static OC1351_PREV_LREM: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_ldata(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_LDATA, ee::s14::Lobby_ListGetData);
        }
        oc1351_bump(c, N_LDATA);
        oc1351_done(c, NODE as u64, 6);
    }
    static OC1351_PREV_LDATA: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_lupd(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_LUPD, ee::s14::Lobby_ListUpdate);
        }
        oc1351_bump(c, N_LUPD);
        oc1351_done(c, 0, 6);
    }
    static OC1351_PREV_LUPD: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_ea_new(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_EA_NEW, ee::s10::f_00317e30);
        }
        oc1351_bump(c, N_EA_NEW);
        let size = (c.g[A0][0] as u32 + 15) & !15;
        let at = c.bus.read32(BUMP);
        c.bus.write32(BUMP, at + size);
        c.g[V0][0] = at as u64;
        c.g[V1][0] = 0;
        c.cycles += 11;
        c.pc = c.g[RA][0] as u32;
    }
    static OC1351_PREV_EA_NEW: AtomicUsize = AtomicUsize::new(0);
    fn oc1351_fake_ea_free(c: &mut Cpu) {
        if !oc1351_gate(c) {
            return fakes::fall_back(c, &OC1351_PREV_EA_FREE, ee::s10::f_00317e98);
        }
        oc1351_bump(c, N_EA_FREE);
        c.g[V0][0] = 0;
        c.g[V1][0] = 0;
        c.cycles += 5;
        c.pc = c.g[RA][0] as u32;
    }
    static OC1351_PREV_EA_FREE: AtomicUsize = AtomicUsize::new(0);

    /// Install every fake once per process (later calls are no-ops).
    pub(crate) fn oc1351_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003F5D88),
            oc1351_fake_tick,
            &OC1351_PREV_TICK,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003F5A70),
            oc1351_fake_peek,
            &OC1351_PREV_PEEK,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003F5C38),
            oc1351_fake_recv,
            &OC1351_PREV_RECV,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003F5910),
            oc1351_fake_send,
            &OC1351_PREV_SEND,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003F57C8),
            oc1351_fake_status,
            &OC1351_PREV_STATUS,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003F5738),
            oc1351_fake_unconnect,
            &OC1351_PREV_UNCONN,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003F5638),
            oc1351_fake_connect,
            &OC1351_PREV_CONN,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E7F70),
            oc1351_fake_terminate,
            &OC1351_PREV_TERM,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E7C40),
            oc1351_fake_pings,
            &OC1351_PREV_PINGS,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E77F0),
            oc1351_fake_pop,
            &OC1351_PREV_POP,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E7BA0),
            oc1351_fake_sort,
            &OC1351_PREV_SORT,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E9590),
            oc1351_fake_pingserver,
            &OC1351_PREV_PINGSRV,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E7B00),
            oc1351_fake_roomcount,
            &OC1351_PREV_ROOMCNT,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E76A8),
            oc1351_fake_printf,
            &OC1351_PREV_PRINTF,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EB6D8),
            oc1351_fake_find,
            &OC1351_PREV_FIND,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003ECF30),
            oc1351_fake_getnum,
            &OC1351_PREV_GETNUM,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003ED378),
            oc1351_fake_getstr,
            &OC1351_PREV_GETSTR,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003ECFF8),
            oc1351_fake_getaddr,
            &OC1351_PREV_GETADDR,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003ECFB0),
            oc1351_fake_getflags,
            &OC1351_PREV_GETFLAGS,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EDB08),
            oc1351_fake_getdate,
            &OC1351_PREV_GETDATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003ED578),
            oc1351_fake_dechex,
            &OC1351_PREV_DECHEX,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EC300),
            oc1351_fake_setbin,
            &OC1351_PREV_SETBIN,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EBE40),
            oc1351_fake_setnum,
            &OC1351_PREV_SETNUM,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EBFC8),
            oc1351_fake_setaddr,
            &OC1351_PREV_SETADDR,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EBB60),
            oc1351_fake_strnz,
            &OC1351_PREV_STRNZ,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EEEE8),
            oc1351_fake_hfindi,
            &OC1351_PREV_HFINDI,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EEF40),
            oc1351_fake_hdeli,
            &OC1351_PREV_HDELI,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EEDA0),
            oc1351_fake_haddi,
            &OC1351_PREV_HADDI,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EEC88),
            oc1351_fake_hfinds,
            &OC1351_PREV_HFINDS,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EEB30),
            oc1351_fake_hadds,
            &OC1351_PREV_HADDS,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EED08),
            oc1351_fake_hdels,
            &OC1351_PREV_HDELS,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EE9A0),
            oc1351_fake_hpop,
            &OC1351_PREV_HPOP,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EE320),
            oc1351_fake_ladd,
            &OC1351_PREV_LADD,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EE480),
            oc1351_fake_lrem,
            &OC1351_PREV_LREM,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EE468),
            oc1351_fake_ldata,
            &OC1351_PREV_LDATA,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EE3E0),
            oc1351_fake_lupd,
            &OC1351_PREV_LUPD,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc1351_fake_ea_new,
            &OC1351_PREV_EA_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1351_fake_ea_free,
            &OC1351_PREV_EA_FREE,
        );
    }

    /// Shared base: installs the fakes, sets gp, fills the client block and
    /// scratch areas with distinctive nonzero words, zeroes the callback
    /// slots and list tables (rows opt in), and writes default scripted
    /// cells (rows tweak from here).
    pub(crate) fn oc1351_base(c: &mut Cpu) {
        oc1351_install_fakes();
        c.g[GP][0] = GAME_GP as u64;
        for i in 0..0x180u32 {
            c.bus.write32(API + 4 * i, 0xC135_0000 | (i + 1));
        }
        for i in 0..0x50u32 {
            c.bus.write32(TAGPTR + 4 * i, 0x07A6_0000 | (i + 1));
        }
        for i in 0..0x50u32 {
            c.bus.write32(NODE + 4 * i, 0xB0DE_0000 | (i + 1));
        }
        for i in 0..0x400u32 {
            c.bus.write32(POOL + 4 * i, 0xE001_0000 | (i + 1));
        }
        for i in 0..11u32 {
            c.bus.write32(API + 0x514 + 4 * i, 0);
        }
        for i in 0..16u32 {
            c.bus.write32(API + 0x310 + 4 * i, 0);
        }
        c.bus.write32(API + 0x08, 0);
        c.bus.write32(API + 0x0c, ST_CONN);
        c.bus.write32(API + 0x10, 0);
        c.bus.write32(API + 0x14, 0);
        c.bus.write32(API + 0x18, 0);
        c.bus.write32(API + 0x1c, 0);
        c.bus.write32(API + 4 * 0x6e, 99);
        c.bus.write32(CTRL, MAGIC);
        c.bus.write32(TICK, BASE_TICK);
        c.bus.write32(PEEK_RET, 0xffff_ffff);
        c.bus.write32(PEEK_N, 0);
        c.bus.write32(PEEK_TYPE, 0);
        c.bus.write32(PEEK_BC, 0);
        c.bus.write32(PEEK_PTR, TAGPTR);
        c.bus.write32(FIND, 0);
        c.bus.write32(GETNUM, 0x2e);
        c.bus.write32(GETADDR, 0);
        c.bus.write32(GETFLAGS, 0);
        c.bus.write32(GETDATE, 0x5d1e_0001);
        c.bus.write32(POP_RET, 0);
        c.bus.write32(HFIND, 0);
        c.bus.write32(HPOP, NODE);
        c.bus.write32(HPOP_N, 0);
        c.bus.write32(HPOP_LIMIT, 0);
        c.bus.write32(LADD, NODE);
        c.bus.write32(LREM, NODE);
        c.bus.write32(STATUS_RET, 0x0700_0001);
        c.bus.write32(BUMP, POOL);
        for i in 0..16u32 {
            c.bus.write32(STRING_SRC + 4 * i, 0);
        }
        for i in 0..4u32 {
            c.bus.write32(HEX_SRC + 4 * i, 0x5EED_0000 | (i + 1));
        }
        for i in 0..38u32 {
            c.bus.write32(CNT + 4 * i, 0);
        }
        c.g[S3][0] = API as u64;
        c.g[S7][0] = BASE_TICK as u64;
        c.g[S0][0] = 0x9C13_5000;
        c.g[S1][0] = 0x9C13_5001;
        c.g[S2][0] = 0x9C13_5002;
        c.g[S4][0] = 0x9C13_5004;
        c.g[S5][0] = 0x9C13_5005;
        c.g[S6][0] = 0x9C13_5006;
        c.g[FP][0] = 0x9C13_501c;
        c.g[V0][0] = 0x9C13_5002_0000;
        c.g[V1][0] = 0x9C13_5003_0000;
        c.g[A0][0] = 0x9C13_5004_0000;
        c.g[A1][0] = 0x9C13_5005_0000;
        c.g[A2][0] = 0x9C13_5006_0000;
        c.g[A3][0] = 0x9C13_5007_0000;
        c.g[RA][0] = TEST_RA as u64;
    }

    /// Full-pump entry: the argument register points at the client.
    fn oc1351_entry(c: &mut Cpu) {
        c.g[A0][0] = API as u64;
        c.pc = ENTRY;
    }

    /// Direct entry at a resume head: carve the frame the prologue would
    /// have made, stash the sentinel return address in the saved-ra slot,
    /// and rewire the callee-saved registers.
    fn oc1351_direct(c: &mut Cpu, head: u32) {
        let sp = (c.g[SP][0] as u32).wrapping_sub(0x420);
        c.g[SP][0] = sp as u64;
        for i in 0..0x108u32 {
            c.bus.write32(sp + 4 * i, 0xD135_0000 | (i + 1));
        }
        c.bus.write32(sp + 0x380, TEST_RA);
        c.pc = head;
    }

    /// Message-row entry at the dispatcher: the type, subtype and payload
    /// words the peek would have delivered.
    fn oc1351_msg_at(c: &mut Cpu, ty: u32, bc: u32, head: u32) {
        oc1351_base(c);
        oc1351_direct(c, head);
        let sp = c.g[SP][0] as u32;
        c.bus.write32(sp + 0x360, ty);
        c.bus.write32(sp + 0x364, bc);
        c.bus.write32(sp + 0x368, TAGPTR);
    }
    fn oc1351_msg(c: &mut Cpu, ty: u32, bc: u32) {
        oc1351_msg_at(c, ty, bc, DISPATCH);
    }

    /// The string the string-getter hands out (NUL-padded by the base).
    fn oc1351_string(c: &mut Cpu, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(STRING_SRC + i as u32, *b);
        }
    }

    /// A real heap pointer from the bump pool (the free fake is a no-op,
    /// so anything it frees stays valid).
    fn oc1351_alloc(c: &mut Cpu, size: u32) -> u32 {
        c.g[A0][0] = size as u64;
        ee::s06::Lobby_MemAlloc(c);
        c.g[V0][0] as u32
    }

    fn oc1351_setup_null(c: &mut Cpu, _seed: u32) {
        oc1351_base(c);
        c.bus.write32(API, 0);
        oc1351_entry(c);
    }
    fn oc1351_setup_offl(c: &mut Cpu, _seed: u32) {
        oc1351_base(c);
        c.bus.write32(API, 0xC135_0001);
        c.bus.write32(API + 0x0c, ST_OFFL);
        oc1351_entry(c);
    }
    fn oc1351_setup_term(c: &mut Cpu, _seed: u32) {
        oc1351_base(c);
        c.bus.write32(API, 0xC135_0001);
        c.bus.write32(API + 0x0c, ST_CONN);
        c.bus.write32(API + 0x10, BASE_TICK - 100);
        c.bus.write32(API + 0x14, 0);
        oc1351_entry(c);
    }
    fn oc1351_setup_idle(c: &mut Cpu, _seed: u32) {
        oc1351_base(c);
        c.bus.write32(API, 0xC135_0001);
        c.bus.write32(API + 0x0c, ST_CONN);
        c.bus.write32(API + 0x10, BASE_TICK + 50);
        c.bus.write32(API + 0x14, BASE_TICK);
        c.bus.write32(API + 0x18, 0);
        oc1351_entry(c);
    }
    fn oc1351_setup_pingtail(c: &mut Cpu, _seed: u32) {
        oc1351_setup_idle(c, 0);
        c.bus.write32(API + 0x18, BASE_TICK - 1);
    }
    fn oc1351_setup_ping(c: &mut Cpu, _seed: u32) {
        oc1351_setup_idle(c, 0);
        c.bus.write32(API + 0x1c, 0xffff_ffff);
        c.bus.write32(PEEK_RET, 0);
        c.bus.write32(PEEK_TYPE, MSG_PING);
        c.bus.write32(PEEK_BC, 0);
        c.bus.write32(PEEK_PTR, TAGPTR);
    }
    fn oc1351_setup_pingtag(c: &mut Cpu, _seed: u32) {
        oc1351_base(c);
        oc1351_direct(c, PING_CHECK);
        c.g[V0][0] = 0xEA60;
        let sp = c.g[SP][0] as u32;
        c.bus.write32(sp + 0x360, MSG_PING);
        c.bus.write32(sp + 0x364, 0);
        c.bus.write32(sp + 0x368, STRING_SRC);
        c.bus.write32(API + 0x1c, 0x1234_5678);
        c.bus.write32(API + 0x10, BASE_TICK + 50);
        c.bus.write32(API + 0x14, BASE_TICK);
        oc1351_string(c, b"ping-tag");
    }
    fn oc1351_setup_rdir_noroot(c: &mut Cpu, _seed: u32) {
        oc1351_msg(c, MSG_DIR, 0);
        // The directory arm only runs in the rdir state.
        c.bus.write32(API + 0x0c, ST_RDIR);
        oc1351_string(c, b"rdir-cache");
    }
    fn oc1351_setup_rdir_root(c: &mut Cpu, _seed: u32) {
        oc1351_msg(c, MSG_DIR, 0);
        // The directory arm only runs in the rdir state.
        c.bus.write32(API + 0x0c, ST_RDIR);
        c.bus.write32(FIND, TAGPTR);
        oc1351_string(c, b"rdir-root");
    }
    fn oc1351_setup_skey(c: &mut Cpu, _seed: u32) {
        oc1351_msg(c, 0xffff_ffff, 0xffff_ffff);
    }
    fn oc1351_setup_skey2(c: &mut Cpu, _seed: u32) {
        oc1351_msg(c, ST_SKEY, 0);
        c.bus.write32(API + 0x0c, ST_SKEY);
    }
    fn oc1351_setup_auth(c: &mut Cpu, _seed: u32) {
        oc1351_msg(c, MSG_AUTH, 0);
        c.bus.write32(GETADDR, 0x0A00_0001);
        oc1351_string(c, b"auth-name");
    }
    fn oc1351_setup_acct(c: &mut Cpu, _seed: u32) {
        oc1351_msg(c, MSG_ACCT, 0);
    }
    fn oc1351_setup_snap(c: &mut Cpu, _seed: u32) {
        oc1351_msg(c, MSG_SNAP, 0);
        c.bus.write32(API + 0x50c, 1);
        c.bus.write32(GETNUM, 0x41);
    }
    fn oc1351_setup_sele(c: &mut Cpu, _seed: u32) {
        oc1351_msg(c, MSG_SELE, 0);
        c.bus.write32(GETNUM, 0x42);
    }
    fn oc1351_setup_pers(c: &mut Cpu, _seed: u32) {
        oc1351_msg(c, MSG_PERS, 0);
        oc1351_string(c, b"persona");
    }
    fn oc1351_setup_sess(c: &mut Cpu, _seed: u32) {
        oc1351_msg(c, MSG_SESS, 0);
        oc1351_string(c, b"sess-data");
    }
    fn oc1351_setup_chat(c: &mut Cpu, seed: u32) {
        // Direct arm entry: DISPATCH never routes '+msg' here in these
        // fixtures (PopRequest/peek runs on stack garbage and falls out
        // before the type dispatch).
        oc1351_msg_at(c, MSG_MSG, 0, CHAT_ARM);
        c.bus.write32(API + 0x51c, LEAF);
        c.bus
            .write32(GETFLAGS, [0u32, 4, 0x10000][seed as usize % 3]);
    }
    fn oc1351_setup_who(c: &mut Cpu, seed: u32) {
        oc1351_msg(c, MSG_WHO, 0);
        c.bus.write32(API + 0x324, 0xC135_0324);
        c.bus.write32(GETNUM, if seed % 2 == 0 { 99 } else { 100 });
        oc1351_string(c, b"who-room");
    }
    fn oc1351_setup_rom_del(c: &mut Cpu, _seed: u32) {
        oc1351_msg(c, MSG_ROM, 0);
        c.bus.write32(API + 0x310, 0xC135_0310);
    }
    fn oc1351_setup_rom_add(c: &mut Cpu, _seed: u32) {
        elf_image(c);
        oc1351_msg(c, MSG_ROM, 0);
        c.bus.write32(API + 0x310, 0xC135_0310);
        c.bus.write32(FIND, TAGPTR);
        oc1351_string(c, b"rom-room");
    }
    fn oc1351_setup_pop(c: &mut Cpu, _seed: u32) {
        oc1351_msg(c, MSG_POP, 0);
        c.bus.write32(API + 0x310, 0xC135_0310);
        oc1351_string(c, b"3 7 0");
    }
    /// Full-pump variant of the `+pop` row: same scripted traffic, but
    /// entered at the top so the driven tests run the whole loop.
    fn oc1351_setup_pop_entry(c: &mut Cpu, _seed: u32) {
        oc1351_setup_idle(c, 0);
        c.bus.write32(API + 0x310, 0xC135_0310);
        c.bus.write32(PEEK_RET, 0);
        c.bus.write32(PEEK_TYPE, MSG_POP);
        c.bus.write32(PEEK_BC, 0);
        c.bus.write32(PEEK_PTR, TAGPTR);
        oc1351_string(c, b"3 7 0");
    }
    fn oc1351_setup_usr_add(c: &mut Cpu, _seed: u32) {
        oc1351_msg(c, MSG_USR, 0);
        c.bus.write32(API + 0x324, 0xC135_0324);
        c.bus.write32(FIND, TAGPTR);
        c.bus.write32(GETFLAGS, 0x0020_0001);
        oc1351_string(c, b"usr-name");
    }
    fn oc1351_setup_usr_del(c: &mut Cpu, _seed: u32) {
        oc1351_msg(c, MSG_USR, 0);
        c.bus.write32(API + 0x324, 0xC135_0324);
        let node = oc1351_alloc(c, 0x138);
        c.bus.write32(HFIND, node);
        c.bus.write32(LREM, node);
        c.g[RA][0] = TEST_RA as u64;
        c.g[A0][0] = API as u64;
        c.pc = DISPATCH;
    }
    fn oc1351_setup_rnl_add(c: &mut Cpu, _seed: u32) {
        // Direct arm entry: DISPATCH never routes '+rnl' here in these
        // fixtures.
        oc1351_msg_at(c, MSG_RNL, 0, RNL_ARM);
        c.bus.write32(API + 0x338, 0xC135_0338);
        c.bus.write32(GETNUM, 5);
        oc1351_string(c, b"rnl-name");
    }
    fn oc1351_setup_rnl_clear(c: &mut Cpu, _seed: u32) {
        // Direct arm entry: DISPATCH never routes '+rnl' here in these
        // fixtures.
        oc1351_msg_at(c, MSG_RNL, 0, RNL_ARM);
        c.bus.write32(API + 0x338, 0xC135_0338);
        c.bus.write32(GETNUM, 0xffff_fffe);
        c.bus.write32(HPOP_LIMIT, 2);
    }
    fn oc1351_setup_snp(c: &mut Cpu, _seed: u32) {
        oc1351_msg(c, MSG_SNP, 0);
        c.bus.write32(API + 0x508, 5);
        c.bus.write32(API + 0x374, 0xC135_0374);
        c.bus.write32(GETNUM, 6);
    }
    fn oc1351_setup_move(c: &mut Cpu, _seed: u32) {
        oc1351_msg(c, MSG_MOVE, 0);
        c.bus.write32(API + 0x324, 0xC135_0324);
        c.bus.write32(GETNUM, 100);
        oc1351_string(c, b"move-room");
    }
    fn oc1351_setup_room(c: &mut Cpu, _seed: u32) {
        oc1351_msg(c, MSG_ROOM, 0);
    }

    fn oc1351_check_null(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
    }
    fn oc1351_check_offl(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert!(hits(c, N_TICK) > 0);
    }
    fn oc1351_check_term(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(state(c), ST_TERM);
        assert!(hits(c, N_TERM) > 0);
    }
    fn oc1351_check_idle(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert!(hits(c, N_PINGS) > 0);
        assert!(hits(c, N_PEEK) > 0);
        assert!(hits(c, N_SORT) > 0);
        assert_eq!(hits(c, N_SEND), 0);
    }
    fn oc1351_check_pingtail(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert!(hits(c, N_PINGSRV) > 0);
    }
    fn oc1351_check_ping(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(c.bus.read32(API + 0x10), BASE_TICK + 0xEA60);
        assert!(hits(c, N_SEND) > 0);
        assert!(hits(c, N_RECV) > 0);
    }
    fn oc1351_check_pingtag(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(c.bus.read32(API + 0x10), BASE_TICK + 0xEA60);
        assert!(hits(c, N_STRNZ) > 0);
        assert!(hits(c, N_SETNUM) > 0);
    }
    fn oc1351_check_rdir_noroot(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x400, 0);
        assert_eq!(c.bus.read32(API + 0x3b8), u32::from_le_bytes(*b"rdir"));
    }
    fn oc1351_check_rdir_root(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(state(c), ST_CONN);
        assert!(hits(c, N_CONN) > 0);
        assert!(hits(c, N_PRINTF) > 0);
        assert!(hits(c, N_SEND) > 0);
        assert_eq!(c.bus.read32(API + 0x500), 0x0700_0001);
    }
    fn oc1351_check_skey(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(state(c), ST_SKEY);
        assert!(hits(c, N_SETBIN) > 0);
        assert!(hits(c, N_SEND) > 0);
    }
    fn oc1351_check_skey2(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(state(c), ST_IDLE);
        assert_ne!(flags(c) & 1, 0);
        // Rearm deadline: the tick plus the 0x7530 rearm window (not the bare tick).
        assert_eq!(c.bus.read32(API + 0x18), BASE_TICK + 0x7530);
        assert!(hits(c, N_DECHEX) > 0);
    }
    fn oc1351_check_auth(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(state(c), ST_AUTH);
        assert_ne!(flags(c) & 2, 0);
        assert_eq!(c.bus.read32(API + 0x134), u32::from_le_bytes(*b"auth"));
        assert_eq!(c.bus.read32(API + 0x4fc), 0x0A00_0001);
    }
    fn oc1351_check_acct(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(state(c), ST_ACCT);
        assert_ne!(flags(c) & 2, 0);
    }
    fn oc1351_check_snap(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(c.bus.read32(API + 0x508), 0x41);
    }
    fn oc1351_check_sele(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(c.bus.read32(API + 0x504), 0x42);
        assert_eq!(c.bus.read32(API + 0x510), 0x42);
    }
    fn oc1351_check_pers(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(c.bus.read32(API + 0x174), u32::from_le_bytes(*b"pers"));
        // Rearm deadline: the tick plus the 0x7530 rearm window (not the bare tick).
        assert_eq!(c.bus.read32(API + 0x18), BASE_TICK + 0x7530);
    }
    fn oc1351_check_sess(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x200, 0);
        assert_eq!(c.bus.read32(API + 0x200), u32::from_le_bytes(*b"sess"));
    }
    fn oc1351_check_chat(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        // The chat arm clears the message scratch area, raises the chat
        // flag and consults the tag flags (no +0x1b4 store on this path).
        assert_ne!(flags(c) & 0x200, 0);
        assert_eq!(c.bus.read32(API + 0x200), 0);
        assert!(hits(c, N_GETFLAGS) > 0);
    }
    fn oc1351_check_who(c: &mut Cpu, seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        if seed % 2 == 0 {
            assert_eq!(room(c), 99);
            assert_eq!(flags(c) & (8 | 0x40), 0);
        } else {
            assert_eq!(room(c), 100);
            assert_eq!(flags(c) & (8 | 0x40), 8 | 0x40);
            assert_eq!(c.bus.read32(API + 0x1c0), u32::from_le_bytes(*b"who-"));
        }
    }
    fn oc1351_check_rom_del(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x20, 0);
        assert!(hits(c, N_HFINDI) > 0);
    }
    fn oc1351_check_rom_add(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x20, 0);
        assert!(hits(c, N_LADD) > 0);
        assert!(hits(c, N_EA_NEW) > 0);
    }
    fn oc1351_check_pop(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert!(hits(c, N_ROOMCNT) > 0);
    }
    fn oc1351_check_usr_add(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x40, 0);
        assert_eq!(c.bus.read32(API + 0x1b4), 0x0020_0001);
    }
    fn oc1351_check_usr_del(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x40, 0);
        assert!(hits(c, N_EA_FREE) > 0);
    }
    fn oc1351_check_rnl_add(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x80, 0);
    }
    fn oc1351_check_rnl_clear(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x80, 0);
        assert!(hits(c, N_EA_FREE) >= 2);
    }
    fn oc1351_check_snp(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x80, 0);
    }
    fn oc1351_check_move(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(room(c), 100);
        assert_eq!(flags(c) & (8 | 0x40), 8 | 0x40);
    }
    fn oc1351_check_room(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert!(hits(c, N_ROOMCNT) > 0);
    }

    /// Every named state: its entry head, its fixture, and what proves the
    /// state ran (a stored marker plus the fake-call counters).
    struct Oc1351Row {
        _name: &'static str,
        setup: fn(&mut Cpu, u32),
        check_row: fn(&mut Cpu, u32),
    }
    const OC1351_ROWS: &[Oc1351Row] = &[
        Oc1351Row {
            _name: "null",
            setup: oc1351_setup_null,
            check_row: oc1351_check_null,
        },
        Oc1351Row {
            _name: "offl",
            setup: oc1351_setup_offl,
            check_row: oc1351_check_offl,
        },
        Oc1351Row {
            _name: "term",
            setup: oc1351_setup_term,
            check_row: oc1351_check_term,
        },
        Oc1351Row {
            _name: "idle",
            setup: oc1351_setup_idle,
            check_row: oc1351_check_idle,
        },
        Oc1351Row {
            _name: "pingtail",
            setup: oc1351_setup_pingtail,
            check_row: oc1351_check_pingtail,
        },
        Oc1351Row {
            _name: "ping",
            setup: oc1351_setup_ping,
            check_row: oc1351_check_ping,
        },
        Oc1351Row {
            _name: "pingtag",
            setup: oc1351_setup_pingtag,
            check_row: oc1351_check_pingtag,
        },
        Oc1351Row {
            _name: "rdir_noroot",
            setup: oc1351_setup_rdir_noroot,
            check_row: oc1351_check_rdir_noroot,
        },
        Oc1351Row {
            _name: "rdir_root",
            setup: oc1351_setup_rdir_root,
            check_row: oc1351_check_rdir_root,
        },
        Oc1351Row {
            _name: "skey",
            setup: oc1351_setup_skey,
            check_row: oc1351_check_skey,
        },
        Oc1351Row {
            _name: "skey2",
            setup: oc1351_setup_skey2,
            check_row: oc1351_check_skey2,
        },
        Oc1351Row {
            _name: "auth",
            setup: oc1351_setup_auth,
            check_row: oc1351_check_auth,
        },
        Oc1351Row {
            _name: "acct",
            setup: oc1351_setup_acct,
            check_row: oc1351_check_acct,
        },
        Oc1351Row {
            _name: "snap",
            setup: oc1351_setup_snap,
            check_row: oc1351_check_snap,
        },
        Oc1351Row {
            _name: "sele",
            setup: oc1351_setup_sele,
            check_row: oc1351_check_sele,
        },
        Oc1351Row {
            _name: "pers",
            setup: oc1351_setup_pers,
            check_row: oc1351_check_pers,
        },
        Oc1351Row {
            _name: "sess",
            setup: oc1351_setup_sess,
            check_row: oc1351_check_sess,
        },
        Oc1351Row {
            _name: "chat",
            setup: oc1351_setup_chat,
            check_row: oc1351_check_chat,
        },
        Oc1351Row {
            _name: "who",
            setup: oc1351_setup_who,
            check_row: oc1351_check_who,
        },
        Oc1351Row {
            _name: "rom_del",
            setup: oc1351_setup_rom_del,
            check_row: oc1351_check_rom_del,
        },
        Oc1351Row {
            _name: "rom_add",
            setup: oc1351_setup_rom_add,
            check_row: oc1351_check_rom_add,
        },
        Oc1351Row {
            _name: "pop",
            setup: oc1351_setup_pop,
            check_row: oc1351_check_pop,
        },
        Oc1351Row {
            _name: "usr_add",
            setup: oc1351_setup_usr_add,
            check_row: oc1351_check_usr_add,
        },
        Oc1351Row {
            _name: "usr_del",
            setup: oc1351_setup_usr_del,
            check_row: oc1351_check_usr_del,
        },
        Oc1351Row {
            _name: "rnl_add",
            setup: oc1351_setup_rnl_add,
            check_row: oc1351_check_rnl_add,
        },
        Oc1351Row {
            _name: "rnl_clear",
            setup: oc1351_setup_rnl_clear,
            check_row: oc1351_check_rnl_clear,
        },
        Oc1351Row {
            _name: "snp",
            setup: oc1351_setup_snp,
            check_row: oc1351_check_snp,
        },
        Oc1351Row {
            _name: "move",
            setup: oc1351_setup_move,
            check_row: oc1351_check_move,
        },
        Oc1351Row {
            _name: "room",
            setup: oc1351_setup_room,
            check_row: oc1351_check_room,
        },
    ];

    #[test]
    fn lobby_api_update_states() {
        for row in OC1351_ROWS {
            check(
                ee::s14::f_003e95c8__generated,
                lobby_api_update,
                SEEDS,
                &|c, seed| {
                    (row.setup)(c, seed);
                },
            );
            for seed in 0..SEEDS {
                let mut c = fresh();
                (row.setup)(&mut c, seed);
                ee::s14::f_003e95c8__generated(&mut c);
                (row.check_row)(&mut c, seed);
            }
        }
    }

    #[test]
    fn lobby_api_update_paths() {
        check_paths(
            ee::s14::f_003e95c8__generated,
            lobby_api_update,
            25,
            &[
                26,  // null client: entry + epilogue
                48,  // offl: entry + tick + gate + epilogue
                68,  // term: entry + tick + timeout + terminate + exit
                239, // idle: entry + tick + pings + peek(-1) + sort + exit
                252, // ping tail: idle + server ping + rearm
            ],
            &|c, seed| match seed % 5 {
                0 => oc1351_setup_null(c, seed),
                1 => oc1351_setup_offl(c, seed),
                2 => oc1351_setup_term(c, seed),
                3 => oc1351_setup_idle(c, seed),
                _ => oc1351_setup_pingtail(c, seed),
            },
        );
    }

    driven!(gen_update_driven, ee::s14::f_003e95c8__generated);
    driven!(hand_update_driven, lobby_api_update);

    fn oc1351_events_idle_setup(c: &mut Cpu, seed: u32) -> Events {
        let mut r = Rng(0x0c13_5101_0000_0001 ^ seed as u64);
        oc1351_setup_ping(c, seed);
        schedule(c, &mut r, seed, ENTRY)
    }

    fn oc1351_events_pop_setup(c: &mut Cpu, seed: u32) -> Events {
        let mut r = Rng(0x0c13_5101_0000_0002 ^ seed as u64);
        oc1351_setup_pop_entry(c, seed);
        schedule(c, &mut r, seed, ENTRY)
    }

    #[test]
    fn lobby_api_update_events_idle() {
        check_events(
            gen_update_driven,
            hand_update_driven,
            60,
            &oc1351_events_idle_setup,
        );
    }

    #[test]
    fn lobby_api_update_events_pop() {
        check_events(
            gen_update_driven,
            hand_update_driven,
            60,
            &oc1351_events_pop_setup,
        );
    }

    use crate::difftest::run_generated;
}
