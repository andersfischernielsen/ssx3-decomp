//! Staging module oc517 (wave wave230): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc517`.

#![allow(unused_imports)]

use crate::abi::{r32, ret, set_word};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0023_73b8, input_sync_update, "InputSync_Update")];

/// Entry and end of `InputSync_Update` (0x2373b8..0x237944).
const UPDATE_ENTRY: u32 = 0x0023_73b8;
const UPDATE_END: u32 = 0x0023_7944;
/// Its stack frame (`addiu sp, sp, -0x160`).
const UPDATE_FRAME: u32 = 0x160;
/// The shared epilogue block (restores s0..s4 and `ra`, returns).
const UPDATE_EPILOGUE: u32 = 0x0023_7924;

/// `InputSync_UnpackSample(this, src, dst)` (0x237be8).
const UPDATE_UNPACK_SAMPLE: Callee = Callee {
    addr: 0x0023_7be8,
    f: ee::s06::f_00237be8,
};
/// `InputSync_SendSample(this, sample)` (0x237a98).
const UPDATE_SEND_SAMPLE: Callee = Callee {
    addr: 0x0023_7a98,
    f: ee::s06::f_00237a98,
};
/// `cGameComm_PopPacket(gc, channel, dst, size)` (0x257bd8).
const UPDATE_POP_PACKET: Callee = Callee {
    addr: 0x0025_7bd8,
    f: ee::s06::f_00257bd8,
};
/// `cGameComm_GetInputDelay(gc)` (0x258360).
const UPDATE_INPUT_DELAY: Callee = Callee {
    addr: 0x0025_8360,
    f: ee::s06::f_00258360,
};

/// `gp - off`: the address of a gp-relative global.
#[inline(always)]
fn update_gp_slot(c: &Cpu, off: u32) -> u32 {
    r32(c, GP).wrapping_sub(off)
}

/// `off(reg)`: an address off a register.
#[inline(always)]
fn update_reg_off(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// Signed `a < b` as the compiled `slt` leaves it (0 or 1, 64-bit).
#[inline(always)]
fn update_slt(a: u64, b: u64) -> u64 {
    ((a as i64) < (b as i64)) as u64
}

/// Event check at a loop back edge with the loop head `head`: when an event
/// is due, deliver it with `pc = head`; an unwind request returns from the
/// body (the trampoline re-enters at `head`).
#[inline(always)]
fn update_back_edge(c: &mut Cpu, head: u32) -> Flow {
    if event_due(c) && deliver_event(c, head) {
        return Err(Unwound);
    }
    Ok(())
}

/// One 0x20-byte step of the sample copy with unaligned accesses (`ldl`/`ldr`
/// pairs, the generated loop at 0x2374ac), from `src` to `dst`.
#[inline(always)]
fn update_copy_unaligned(c: &mut Cpu, src: u32, dst: u32) {
    c.ldl(A1, src.wrapping_add(0x7));
    c.ldr(A1, src);
    c.ldl(A3, src.wrapping_add(0xf));
    c.ldr(A3, src.wrapping_add(0x8));
    c.ldl(T2, src.wrapping_add(0x17));
    c.ldr(T2, src.wrapping_add(0x10));
    c.ldl(T3, src.wrapping_add(0x1f));
    c.ldr(T3, src.wrapping_add(0x18));
    c.sdl(A1, dst.wrapping_add(0x7));
    c.sdr(A1, dst);
    c.sdl(A3, dst.wrapping_add(0xf));
    c.sdr(A3, dst.wrapping_add(0x8));
    c.sdl(T2, dst.wrapping_add(0x17));
    c.sdr(T2, dst.wrapping_add(0x10));
    c.sdl(T3, dst.wrapping_add(0x1f));
    c.sdr(T3, dst.wrapping_add(0x18));
}

/// One 0x20-byte step of the sample copy with aligned doublewords (the
/// generated loop at 0x237504), from `src` to `dst`.
#[inline(always)]
fn update_copy_aligned(c: &mut Cpu, src: u32, dst: u32) {
    c.g[A1][0] = c.ld(src);
    c.g[A3][0] = c.ld(src.wrapping_add(0x8));
    c.g[T2][0] = c.ld(src.wrapping_add(0x10));
    c.g[T3][0] = c.ld(src.wrapping_add(0x18));
    let (a1, a3, t2, t3) = (c.g[A1][0], c.g[A3][0], c.g[T2][0], c.g[T3][0]);
    c.sd(dst, a1);
    c.sd(dst.wrapping_add(0x8), a3);
    c.sd(dst.wrapping_add(0x10), t2);
    c.sd(dst.wrapping_add(0x18), t3);
}

/// `void InputSync_Update(InputSync *self)` (0x2373b8): for each synced pad,
/// fill its sample ring. A remote pad (no source pad) takes the network
/// packets while the ring is short (or the game comms are idle, a neutral
/// sample otherwise). A local pad (a source pad) samples its buttons every
/// `numsamples` ticks, sends them, and refreshes the send limit from the
/// comms' input delay. Returns early while the comms are below state 14.
///
/// The body is the generated block structure, one arm per block, so a call
/// that unwinds or an event that asks for a thread switch resumes at the
/// same block (`c.pc`), as the compiled code does. The struct fields:
/// self `+0` sendlimit, `+4` latency, `+8` numsamples, `+0xc` tick, `+0x10`
/// numsyncedpads, `+0x14` syncedpads (stride 0x18: `+0` samples, `+4`
/// readpos, `+8` writepos, `+0xc` sourcepad).
pub fn input_sync_update(c: &mut Cpu) {
    guest(c, input_sync_update_body);
}

fn input_sync_update_body(c: &mut Cpu) -> Flow {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            // Prologue: `if (GameComm()->mState < 14) return;`.
            0x0023_73b8 => {
                c.cycles += 12;
                let frame = Frame::open(c, UPDATE_FRAME);
                let gc_slot = update_gp_slot(c, 0x204);
                c.g[V1][0] = c.lw(gc_slot);
                frame.save(c, S2, 0x130);
                frame.save(c, S0, 0x150);
                frame.save(c, S1, 0x140);
                frame.save(c, S3, 0x120);
                frame.save(c, S4, 0x110);
                frame.save_ra(c, 0x100);
                let state_slot = update_reg_off(c, V1, 0x70);
                c.g[V0][0] = c.lw(state_slot);
                c.g[V0][0] = update_slt(c.g[V0][0], 14);
                let t = c.g[V0][0] != 0;
                c.g[S2][0] = c.g[A0][0];
                pc = if t { UPDATE_EPILOGUE } else { 0x0023_73e8 };
                continue;
            }
            // `for (i = 0; i < self->numsyncedpads; i++)`: the test before the loop.
            0x0023_73e8 => {
                c.cycles += 3;
                let count_slot = update_reg_off(c, S2, 0x10);
                c.g[V0][0] = c.lw(count_slot);
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[S0][0] = 0;
                pc = if t { UPDATE_EPILOGUE } else { 0x0023_73f4 };
                continue;
            }
            0x0023_73f4 => {
                c.cycles += 3;
                set_word(c, S4, r32(c, SP).wrapping_add(0x70));
                c.g[V0][0] = 0x18;
                pc = 0x0023_7400;
                continue;
            }
            // Loop head: `psp = &syncedpads[i]`; a local pad (source pad set) goes to 0x237720.
            0x0023_7400 => {
                c.cycles += 6;
                c.mult(0, V0, S0, V0);
                set_word(c, V0, r32(c, V0).wrapping_add(0x14));
                set_word(c, S1, r32(c, S2).wrapping_add(r32(c, V0)));
                let pad_source_slot = update_reg_off(c, S1, 0xc);
                c.g[V1][0] = c.lw(pad_source_slot);
                if c.g[V1][0] != 0 {
                    let sendlimit_slot = update_reg_off(c, S2, 0);
                    c.g[V0][0] = c.lw(sendlimit_slot);
                    pc = 0x0023_7720;
                    continue;
                }
                pc = 0x0023_7418;
                continue;
            }
            // Remote pad: `if (sp.sourcepad == 0)`, with `net` from the comms state.
            0x0023_7418 => {
                c.cycles += 4;
                c.g[V1][0] = c.lw(update_gp_slot(c, 0x204));
                let sync_slot = update_reg_off(c, V1, 0x40);
                c.g[V0][0] = c.lw(sync_slot);
                let t = c.g[V0][0] == 0;
                c.g[A0][0] = 0;
                pc = if t { 0x0023_743c } else { 0x0023_7428 };
                continue;
            }
            0x0023_7428 => {
                c.cycles += 3;
                let error_slot = update_reg_off(c, V1, 0x4);
                c.g[V0][0] = c.lw(error_slot);
                let t = c.g[V0][0] != 0;
                pc = if t { 0x0023_743c } else { 0x0023_7434 };
                continue;
            }
            0x0023_7434 => {
                c.cycles += 2;
                let connected_slot = update_reg_off(c, V1, 0);
                c.g[V0][0] = c.lw(connected_slot);
                c.g[A0][0] = (0u64 < c.g[V0][0]) as u64;
                pc = 0x0023_743c;
                continue;
            }
            // `net` is `a0`; s3 = i + 1 for the loop tail.
            0x0023_743c => {
                c.cycles += 2;
                let t = c.g[A0][0] == 0;
                set_word(c, S3, r32(c, S0).wrapping_add(0x1));
                pc = if t { 0x0023_75a4 } else { 0x0023_7444 };
                continue;
            }
            // Network `while` loop: its condition sits at 0x23755c.
            0x0023_7444 => {
                c.cycles += 2;
                let latency_slot = update_reg_off(c, S2, 0x4);
                c.g[V0][0] = c.lw(latency_slot);
                pc = 0x0023_755c;
                continue;
            }
            0x0023_744c => {
                c.cycles += 1;
                pc = 0x0023_7450;
                continue;
            }
            // Network body: decode the packet (a2 holds the packet index s0 here).
            0x0023_7450 => {
                c.cycles += 6;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[SP][0];
                UPDATE_UNPACK_SAMPLE.call(c, 0x0023_745c)?;
                pc = 0x0023_745c;
                continue;
            }
            // Resume after the unpack.
            0x0023_745c => {
                let numsamples_slot = update_reg_off(c, S2, 0x8);
                c.g[V0][0] = c.lw(numsamples_slot);
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[A3][0] = 0;
                pc = if t { 0x0023_7558 } else { 0x0023_7468 };
                continue;
            }
            // Network copy loop over numsamples: write the sample into the ring.
            0x0023_7468 => {
                c.cycles += 4;
                c.g[T1][0] = 0;
                c.g[T0][0] = 0x64;
                let latency_slot = update_reg_off(c, S2, 0x4);
                c.g[A0][0] = c.lw(latency_slot);
                pc = 0x0023_7478;
                continue;
            }
            0x0023_7478 => {
                c.cycles += 4;
                c.g[A2][0] = c.g[SP][0];
                let writepos_slot = update_reg_off(c, S1, 0x8);
                c.g[V0][0] = c.lw(writepos_slot);
                pc = 0x0023_7488;
                continue;
            }
            0x0023_7488 => {
                c.cycles += 8;
                c.div(0, V0, A0);
                let samples_slot = update_reg_off(c, S1, 0);
                c.g[A1][0] = c.lw(samples_slot);
                c.g[V1][0] = c.hi[0];
                c.mult(0, V1, V1, T0);
                set_word(c, V1, r32(c, V1).wrapping_add(r32(c, A1)));
                c.g[V0][0] = c.g[V1][0] & 0x7;
                let t = c.g[V0][0] == 0;
                set_word(c, A0, r32(c, A3).wrapping_add(0x1));
                pc = if t { 0x0023_7500 } else { 0x0023_74a8 };
                continue;
            }
            0x0023_74a8 => {
                c.cycles += 1;
                set_word(c, V0, r32(c, SP).wrapping_add(0x60));
                pc = 0x0023_74ac;
                continue;
            }
            // Unaligned 0x20-byte copy steps until a2 reaches the sample end.
            0x0023_74ac => {
                c.cycles += 19;
                let src = r32(c, A2);
                let dst = r32(c, V1);
                update_copy_unaligned(c, src, dst);
                set_word(c, A2, src.wrapping_add(0x20));
                let t = c.g[A2][0] != c.g[V0][0];
                set_word(c, V1, dst.wrapping_add(0x20));
                if t {
                    update_back_edge(c, 0x0023_74ac)?;
                    pc = 0x0023_74ac;
                    continue;
                }
                pc = 0x0023_74f8;
                continue;
            }
            0x0023_74f8 => {
                c.cycles += 2;
                pc = 0x0023_7530;
                continue;
            }
            0x0023_7500 => {
                c.cycles += 1;
                set_word(c, V0, r32(c, SP).wrapping_add(0x60));
                pc = 0x0023_7504;
                continue;
            }
            // Aligned doubleword copy steps.
            0x0023_7504 => {
                c.cycles += 11;
                let src = r32(c, A2);
                let dst = r32(c, V1);
                update_copy_aligned(c, src, dst);
                set_word(c, A2, src.wrapping_add(0x20));
                let t = c.g[A2][0] != c.g[V0][0];
                set_word(c, V1, dst.wrapping_add(0x20));
                if t {
                    update_back_edge(c, 0x0023_7504)?;
                    pc = 0x0023_7504;
                    continue;
                }
                pc = 0x0023_7530;
                continue;
            }
            // The last word, then `s1->writepos++` and the inner `for` test.
            0x0023_7530 => {
                c.cycles += 10;
                let src = r32(c, A2);
                c.g[V0][0] = c.lw(src);
                let dst = r32(c, V1);
                let word = c.g[V0][0];
                c.sw(dst, word);
                let writepos_slot = update_reg_off(c, S1, 0x8);
                c.g[V0][0] = c.lw(writepos_slot);
                c.g[A3][0] = c.g[A0][0];
                set_word(c, V0, r32(c, V0).wrapping_add(0x1));
                let writepos_value = c.g[V0][0];
                c.sw(writepos_slot, writepos_value);
                let numsamples_slot = update_reg_off(c, S2, 0x8);
                c.g[V1][0] = c.lw(numsamples_slot);
                c.g[V1][0] = update_slt(c.g[A3][0], c.g[V1][0]);
                if c.g[V1][0] != 0 {
                    let latency_slot = update_reg_off(c, S2, 0x4);
                    c.g[A0][0] = c.lw(latency_slot);
                    update_back_edge(c, 0x0023_7478)?;
                    pc = 0x0023_7478;
                    continue;
                }
                pc = 0x0023_7558;
                continue;
            }
            0x0023_7558 => {
                c.cycles += 1;
                let latency_slot = update_reg_off(c, S2, 0x4);
                c.g[V0][0] = c.lw(latency_slot);
                pc = 0x0023_755c;
                continue;
            }
            // While condition: `writepos - readpos < latency + 1 - numsamples`.
            // s0 becomes the compressed-sample buffer (sp + 0x70).
            0x0023_755c => {
                c.cycles += 9;
                let writepos_slot = update_reg_off(c, S1, 0x8);
                c.g[V1][0] = c.lw(writepos_slot);
                let readpos_slot = update_reg_off(c, S1, 0x4);
                c.g[A1][0] = c.lw(readpos_slot);
                set_word(c, V0, r32(c, V0).wrapping_add(0x1));
                let numsamples_slot = update_reg_off(c, S2, 0x8);
                c.g[A0][0] = c.lw(numsamples_slot);
                set_word(c, V1, r32(c, V1).wrapping_sub(r32(c, A1)));
                set_word(c, V0, r32(c, V0).wrapping_sub(r32(c, A0)));
                c.g[V1][0] = update_slt(c.g[V1][0], c.g[V0][0]);
                let t = c.g[V1][0] == 0;
                set_word(c, S0, r32(c, SP).wrapping_add(0x70));
                if t {
                    pc = 0x0023_7910;
                    continue;
                }
                pc = 0x0023_7580;
                continue;
            }
            // Network: `cGameComm_PopPacket(GameComm(), 0, compressed, 0x1c)`.
            0x0023_7580 => {
                c.cycles += 7;
                c.g[A0][0] = c.lw(update_gp_slot(c, 0x204));
                c.g[A1][0] = 0;
                c.g[A2][0] = c.g[S0][0];
                c.g[A3][0] = 0x1c;
                UPDATE_POP_PACKET.call(c, 0x0023_7594)?;
                pc = 0x0023_7594;
                continue;
            }
            // Resume after `PopPacket`: a packet came in, so unpack it.
            0x0023_7594 => {
                let t = c.g[V0][0] != 0;
                c.g[A2][0] = c.g[S0][0];
                if t {
                    update_back_edge(c, 0x0023_7450)?;
                    pc = 0x0023_7450;
                    continue;
                }
                pc = 0x0023_759c;
                continue;
            }
            0x0023_759c => {
                c.cycles += 2;
                let t = true;
                let numsamples_slot = update_reg_off(c, S2, 0x10);
                c.g[V0][0] = c.lw(numsamples_slot);
                pc = if t { 0x0023_7914 } else { 0x0023_75a4 };
                continue;
            }
            // Remote pad with the comms idle: a neutral sample, unpacked,
            // then the ring fill.
            0x0023_75a4 => {
                c.cycles += 20;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x1c;
                call_ptr(c, 0x003e_6448, 0x0023_75b4)?;
                pc = 0x0023_75b4;
                continue;
            }
            // Resume after `EA_MemSet`: flag the neutral sample as 24 buttons, unpack it.
            0x0023_75b4 => {
                c.g[V0][0] = 0x18;
                c.g[A0][0] = c.g[S2][0];
                let compressed_buttons = r32(c, SP).wrapping_add(0x70);
                let value = c.g[V0][0];
                c.sb(compressed_buttons, value);
                c.g[A1][0] = c.g[SP][0];
                c.g[A2][0] = c.g[S4][0];
                UPDATE_UNPACK_SAMPLE.call(c, 0x0023_75cc)?;
                pc = 0x0023_75cc;
                continue;
            }
            // Resume after the second unpack: fill the ring while it is short.
            0x0023_75cc => {
                let latency_slot = update_reg_off(c, S2, 0x4);
                c.g[V0][0] = c.lw(latency_slot);
                let writepos_slot = update_reg_off(c, S1, 0x8);
                c.g[V1][0] = c.lw(writepos_slot);
                let readpos_slot = update_reg_off(c, S1, 0x4);
                c.g[A0][0] = c.lw(readpos_slot);
                set_word(c, V0, r32(c, V0).wrapping_add(0x1));
                let numsamples_slot = update_reg_off(c, S2, 0x8);
                c.g[A1][0] = c.lw(numsamples_slot);
                set_word(c, V1, r32(c, V1).wrapping_sub(r32(c, A0)));
                set_word(c, V0, r32(c, V0).wrapping_sub(r32(c, A1)));
                c.g[V1][0] = update_slt(c.g[V1][0], c.g[V0][0]);
                if c.g[V1][0] == 0 {
                    let count_slot = update_reg_off(c, S2, 0x10);
                    c.g[V0][0] = c.lw(count_slot);
                    pc = 0x0023_7914;
                    continue;
                }
                pc = 0x0023_75f4;
                continue;
            }
            0x0023_75f4 => {
                c.cycles += 1;
                pc = 0x0023_75f8;
                continue;
            }
            // Ring fill loop head (`numsamples <= 0` skips the body).
            0x0023_75f8 => {
                c.cycles += 2;
                let t = (c.g[A1][0] as i64) <= 0;
                c.g[A3][0] = 0;
                pc = if t { 0x0023_76f0 } else { 0x0023_7600 };
                continue;
            }
            0x0023_7600 => {
                c.cycles += 4;
                c.g[T1][0] = 0;
                c.g[T0][0] = 0x64;
                let latency_slot = update_reg_off(c, S2, 0x4);
                c.g[A0][0] = c.lw(latency_slot);
                pc = 0x0023_7610;
                continue;
            }
            0x0023_7610 => {
                c.cycles += 4;
                c.g[A2][0] = c.g[SP][0];
                let writepos_slot = update_reg_off(c, S1, 0x8);
                c.g[V0][0] = c.lw(writepos_slot);
                pc = 0x0023_7620;
                continue;
            }
            0x0023_7620 => {
                c.cycles += 8;
                c.div(0, V0, A0);
                let samples_slot = update_reg_off(c, S1, 0);
                c.g[A1][0] = c.lw(samples_slot);
                c.g[V1][0] = c.hi[0];
                c.mult(0, V1, V1, T0);
                set_word(c, V1, r32(c, V1).wrapping_add(r32(c, A1)));
                c.g[V0][0] = c.g[V1][0] & 0x7;
                let t = c.g[V0][0] == 0;
                set_word(c, A0, r32(c, A3).wrapping_add(0x1));
                pc = if t { 0x0023_7698 } else { 0x0023_7640 };
                continue;
            }
            0x0023_7640 => {
                c.cycles += 1;
                set_word(c, V0, r32(c, SP).wrapping_add(0x60));
                pc = 0x0023_7644;
                continue;
            }
            0x0023_7644 => {
                c.cycles += 19;
                let src = r32(c, A2);
                let dst = r32(c, V1);
                update_copy_unaligned(c, src, dst);
                set_word(c, A2, src.wrapping_add(0x20));
                let t = c.g[A2][0] != c.g[V0][0];
                set_word(c, V1, dst.wrapping_add(0x20));
                if t {
                    update_back_edge(c, 0x0023_7644)?;
                    pc = 0x0023_7644;
                    continue;
                }
                pc = 0x0023_7690;
                continue;
            }
            0x0023_7690 => {
                c.cycles += 2;
                pc = 0x0023_76c8;
                continue;
            }
            0x0023_7698 => {
                c.cycles += 1;
                set_word(c, V0, r32(c, SP).wrapping_add(0x60));
                pc = 0x0023_769c;
                continue;
            }
            0x0023_769c => {
                c.cycles += 11;
                let src = r32(c, A2);
                let dst = r32(c, V1);
                update_copy_aligned(c, src, dst);
                set_word(c, A2, src.wrapping_add(0x20));
                let t = c.g[A2][0] != c.g[V0][0];
                set_word(c, V1, dst.wrapping_add(0x20));
                if t {
                    update_back_edge(c, 0x0023_769c)?;
                    pc = 0x0023_769c;
                    continue;
                }
                pc = 0x0023_76c8;
                continue;
            }
            0x0023_76c8 => {
                c.cycles += 10;
                let src = r32(c, A2);
                c.g[V0][0] = c.lw(src);
                let dst = r32(c, V1);
                let word = c.g[V0][0];
                c.sw(dst, word);
                let writepos_slot = update_reg_off(c, S1, 0x8);
                c.g[V0][0] = c.lw(writepos_slot);
                c.g[A3][0] = c.g[A0][0];
                set_word(c, V0, r32(c, V0).wrapping_add(0x1));
                let writepos_value = c.g[V0][0];
                c.sw(writepos_slot, writepos_value);
                let numsamples_slot = update_reg_off(c, S2, 0x8);
                c.g[V1][0] = c.lw(numsamples_slot);
                c.g[V1][0] = update_slt(c.g[A3][0], c.g[V1][0]);
                if c.g[V1][0] != 0 {
                    let latency_slot = update_reg_off(c, S2, 0x4);
                    c.g[A0][0] = c.lw(latency_slot);
                    update_back_edge(c, 0x0023_7610)?;
                    pc = 0x0023_7610;
                    continue;
                }
                pc = 0x0023_76f0;
                continue;
            }
            // Ring fill done: the while condition again (writepos - readpos < ...).
            0x0023_76f0 => {
                c.cycles += 10;
                let latency_slot = update_reg_off(c, S2, 0x4);
                c.g[V0][0] = c.lw(latency_slot);
                let writepos_slot = update_reg_off(c, S1, 0x8);
                c.g[V1][0] = c.lw(writepos_slot);
                let readpos_slot = update_reg_off(c, S1, 0x4);
                c.g[A0][0] = c.lw(readpos_slot);
                set_word(c, V0, r32(c, V0).wrapping_add(0x1));
                let numsamples_slot = update_reg_off(c, S2, 0x8);
                c.g[A1][0] = c.lw(numsamples_slot);
                set_word(c, V1, r32(c, V1).wrapping_sub(r32(c, A0)));
                set_word(c, V0, r32(c, V0).wrapping_sub(r32(c, A1)));
                c.g[V1][0] = update_slt(c.g[V1][0], c.g[V0][0]);
                let t = c.g[V1][0] != 0;
                if t {
                    update_back_edge(c, 0x0023_75f8)?;
                    pc = 0x0023_75f8;
                    continue;
                }
                pc = 0x0023_7718;
                continue;
            }
            0x0023_7718 => {
                c.cycles += 2;
                let t = true;
                let count_slot = update_reg_off(c, S2, 0x10);
                c.g[V0][0] = c.lw(count_slot);
                pc = if t { 0x0023_7914 } else { 0x0023_7720 };
                continue;
            }
            // Local pad (source pad set): the ring fill is due while the ring is short.
            0x0023_7720 => {
                c.cycles += 9;
                let writepos_slot = update_reg_off(c, S1, 0x8);
                c.g[V1][0] = c.lw(writepos_slot);
                let readpos_slot = update_reg_off(c, S1, 0x4);
                c.g[A0][0] = c.lw(readpos_slot);
                set_word(c, V0, r32(c, V0).wrapping_add(0x1));
                let numsamples_slot = update_reg_off(c, S2, 0x8);
                c.g[A1][0] = c.lw(numsamples_slot);
                set_word(c, V1, r32(c, V1).wrapping_sub(r32(c, A0)));
                set_word(c, V0, r32(c, V0).wrapping_sub(r32(c, A1)));
                c.g[V1][0] = update_slt(c.g[V1][0], c.g[V0][0]);
                let t = c.g[V1][0] == 0;
                set_word(c, S3, r32(c, S0).wrapping_add(0x1));
                if t {
                    pc = 0x0023_7910;
                    continue;
                }
                pc = 0x0023_7744;
                continue;
            }
            // `if (++self->tick % self->numsamples == 0)`.
            0x0023_7744 => {
                c.cycles += 6;
                let tick_slot = update_reg_off(c, S2, 0xc);
                c.g[V0][0] = c.lw(tick_slot);
                set_word(c, V0, r32(c, V0).wrapping_add(0x1));
                c.div(0, V0, A1);
                let tick_value = c.g[V0][0];
                c.sw(tick_slot, tick_value);
                pc = 0x0023_775c;
                continue;
            }
            0x0023_775c => {
                c.cycles += 3;
                c.g[V1][0] = c.hi[0];
                if c.g[V1][0] != 0 {
                    let count_slot = update_reg_off(c, S2, 0x10);
                    c.g[V0][0] = c.lw(count_slot);
                    pc = 0x0023_7914;
                    continue;
                }
                pc = 0x0023_7768;
                continue;
            }
            // Local sample: the pad's buttons go into the sample at sp + 0x90.
            0x0023_7768 => {
                c.cycles += 6;
                let pad_slot = update_reg_off(c, S1, 0xc);
                c.g[V1][0] = c.lw(pad_slot);
                c.g[A1][0] = 0;
                set_word(c, S0, r32(c, SP).wrapping_add(0x90));
                let buttons_slot = r32(c, V1);
                c.g[V0][0] = c.lw(buttons_slot);
                let t = (c.g[V0][0] as i64) <= 0;
                let sample_count_slot = r32(c, SP).wrapping_add(0x90);
                let value = c.g[V0][0];
                c.sw(sample_count_slot, value);
                pc = if t { 0x0023_77c0 } else { 0x0023_7780 };
                continue;
            }
            0x0023_7780 => {
                c.cycles += 4;
                c.g[A2][0] = c.g[V1][0];
                set_word(c, A0, r32(c, SP).wrapping_add(0x94));
                set_word(c, V1, r32(c, A2).wrapping_add(0x4));
                pc = 0x0023_7790;
                continue;
            }
            0x0023_7790 => {
                c.cycles += 5;
                let count_slot = r32(c, A2);
                c.g[V0][0] = c.lw(count_slot);
                c.mtc1(0, 0);
                c.g[V0][0] = update_slt(c.g[A1][0], c.g[V0][0]);
                if c.g[V0][0] != 0 {
                    let float_slot = r32(c, V1);
                    c.lwc1(0, float_slot);
                    pc = 0x0023_77a4;
                    continue;
                }
                pc = 0x0023_77a4;
                continue;
            }
            0x0023_77a4 => {
                c.cycles += 7;
                let out_slot = r32(c, A0);
                c.swc1(0, out_slot);
                set_word(c, A1, r32(c, A1).wrapping_add(0x1));
                set_word(c, A0, r32(c, A0).wrapping_add(0x4));
                let count_slot = r32(c, SP).wrapping_add(0x90);
                c.g[V0][0] = c.lw(count_slot);
                c.g[V0][0] = update_slt(c.g[A1][0], c.g[V0][0]);
                let t = c.g[V0][0] != 0;
                set_word(c, V1, r32(c, V1).wrapping_add(0x1c));
                if t {
                    update_back_edge(c, 0x0023_7790)?;
                    pc = 0x0023_7790;
                    continue;
                }
                pc = 0x0023_77c0;
                continue;
            }
            // `InputSync_SendSample(self, localsample)`.
            0x0023_77c0 => {
                c.cycles += 6;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S0][0];
                UPDATE_SEND_SAMPLE.call(c, 0x0023_77cc)?;
                pc = 0x0023_77cc;
                continue;
            }
            // Resume after `SendSample`: copy the sample into the ring.
            0x0023_77cc => {
                let numsamples_slot = update_reg_off(c, S2, 0x8);
                c.g[V0][0] = c.lw(numsamples_slot);
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[A3][0] = 0;
                pc = if t { 0x0023_78c8 } else { 0x0023_77d8 };
                continue;
            }
            0x0023_77d8 => {
                c.cycles += 4;
                c.g[T1][0] = 0;
                c.g[T0][0] = 0x64;
                let latency_slot = update_reg_off(c, S2, 0x4);
                c.g[A0][0] = c.lw(latency_slot);
                pc = 0x0023_77e8;
                continue;
            }
            0x0023_77e8 => {
                c.cycles += 4;
                c.g[A2][0] = c.g[S0][0];
                let writepos_slot = update_reg_off(c, S1, 0x8);
                c.g[V0][0] = c.lw(writepos_slot);
                pc = 0x0023_77f8;
                continue;
            }
            0x0023_77f8 => {
                c.cycles += 8;
                c.div(0, V0, A0);
                let samples_slot = update_reg_off(c, S1, 0);
                c.g[A1][0] = c.lw(samples_slot);
                c.g[V1][0] = c.hi[0];
                c.mult(0, V1, V1, T0);
                set_word(c, V1, r32(c, V1).wrapping_add(r32(c, A1)));
                c.g[V0][0] = c.g[V1][0] & 0x7;
                let t = c.g[V0][0] == 0;
                set_word(c, A0, r32(c, A3).wrapping_add(0x1));
                pc = if t { 0x0023_7870 } else { 0x0023_7818 };
                continue;
            }
            0x0023_7818 => {
                c.cycles += 1;
                set_word(c, V0, r32(c, SP).wrapping_add(0xf0));
                pc = 0x0023_781c;
                continue;
            }
            0x0023_781c => {
                c.cycles += 19;
                let src = r32(c, A2);
                let dst = r32(c, V1);
                update_copy_unaligned(c, src, dst);
                set_word(c, A2, src.wrapping_add(0x20));
                let t = c.g[A2][0] != c.g[V0][0];
                set_word(c, V1, dst.wrapping_add(0x20));
                if t {
                    update_back_edge(c, 0x0023_781c)?;
                    pc = 0x0023_781c;
                    continue;
                }
                pc = 0x0023_7868;
                continue;
            }
            0x0023_7868 => {
                c.cycles += 2;
                pc = 0x0023_78a0;
                continue;
            }
            0x0023_7870 => {
                c.cycles += 1;
                set_word(c, V0, r32(c, SP).wrapping_add(0xf0));
                pc = 0x0023_7874;
                continue;
            }
            0x0023_7874 => {
                c.cycles += 11;
                let src = r32(c, A2);
                let dst = r32(c, V1);
                update_copy_aligned(c, src, dst);
                set_word(c, A2, src.wrapping_add(0x20));
                let t = c.g[A2][0] != c.g[V0][0];
                set_word(c, V1, dst.wrapping_add(0x20));
                if t {
                    update_back_edge(c, 0x0023_7874)?;
                    pc = 0x0023_7874;
                    continue;
                }
                pc = 0x0023_78a0;
                continue;
            }
            0x0023_78a0 => {
                c.cycles += 10;
                let src = r32(c, A2);
                c.g[V0][0] = c.lw(src);
                let dst = r32(c, V1);
                let word = c.g[V0][0];
                c.sw(dst, word);
                let writepos_slot = update_reg_off(c, S1, 0x8);
                c.g[V0][0] = c.lw(writepos_slot);
                c.g[A3][0] = c.g[A0][0];
                set_word(c, V0, r32(c, V0).wrapping_add(0x1));
                let writepos_value = c.g[V0][0];
                c.sw(writepos_slot, writepos_value);
                let numsamples_slot = update_reg_off(c, S2, 0x8);
                c.g[V1][0] = c.lw(numsamples_slot);
                c.g[V1][0] = update_slt(c.g[A3][0], c.g[V1][0]);
                if c.g[V1][0] != 0 {
                    let latency_slot = update_reg_off(c, S2, 0x4);
                    c.g[A0][0] = c.lw(latency_slot);
                    update_back_edge(c, 0x0023_77e8)?;
                    pc = 0x0023_77e8;
                    continue;
                }
                pc = 0x0023_78c8;
                continue;
            }
            // `n = (int)(cGameComm_getLatency(GameComm()) / SSXApp->TimerPeriod)`,
            // `if (n <= 1) n = 2; sendlimit = n * numsamples`, capped at latency.
            0x0023_78c8 => {
                c.cycles += 17;
                c.g[A0][0] = c.lw(update_gp_slot(c, 0x204));
                UPDATE_INPUT_DELAY.call(c, 0x0023_78d0)?;
                pc = 0x0023_78d0;
                continue;
            }
            // Resume after `cGameComm_GetInputDelay`: the send limit.
            0x0023_78d0 => {
                c.g[V0][0] = c.lw(update_gp_slot(c, 0x848));
                c.g[V1][0] = 0x1;
                c.g[A0][0] = 0x2;
                let numsamples_slot = update_reg_off(c, S2, 0x8);
                c.g[A1][0] = c.lw(numsamples_slot);
                let period_slot = update_reg_off(c, V0, 0x14);
                c.lwc1(1, period_slot);
                let latency_slot = update_reg_off(c, S2, 0x4);
                c.g[A2][0] = c.lw(latency_slot);
                c.div_s(0, 0, 1);
                c.cvt_w_s(1, 0);
                c.mfc1(2, 1);
                c.g[V1][0] = update_slt(c.g[V1][0], c.g[V0][0]);
                if c.g[V1][0] == 0 {
                    c.g[V0][0] = c.g[A0][0];
                }
                c.mult(0, V0, V0, A1);
                c.g[V1][0] = update_slt(c.g[A2][0], c.g[V0][0]);
                let t = c.g[V1][0] == 0;
                let sendlimit_slot = r32(c, S2);
                let sendlimit = c.g[V0][0];
                c.sw(sendlimit_slot, sendlimit);
                pc = if t { 0x0023_7910 } else { 0x0023_790c };
                continue;
            }
            0x0023_790c => {
                c.cycles += 1;
                let sendlimit_slot = r32(c, S2);
                let latency = c.g[A2][0];
                c.sw(sendlimit_slot, latency);
                pc = 0x0023_7910;
                continue;
            }
            0x0023_7910 => {
                c.cycles += 1;
                let count_slot = update_reg_off(c, S2, 0x10);
                c.g[V0][0] = c.lw(count_slot);
                pc = 0x0023_7914;
                continue;
            }
            // Loop tail: `i = i + 1`, while `i < numsyncedpads` (back edge with event check).
            0x0023_7914 => {
                c.cycles += 4;
                c.g[S0][0] = c.g[S3][0];
                c.g[V0][0] = update_slt(c.g[S0][0], c.g[V0][0]);
                let t = c.g[V0][0] != 0;
                c.g[V0][0] = 0x18;
                if t {
                    update_back_edge(c, 0x0023_7400)?;
                    pc = 0x0023_7400;
                    continue;
                }
                pc = UPDATE_EPILOGUE;
                continue;
            }
            // Epilogue: restore s0..s4 and `ra`, pop the frame, return.
            0x0023_7924 => {
                c.cycles += 8;
                let frame = Frame { size: UPDATE_FRAME };
                frame.restore(c, S0, 0x150);
                frame.restore(c, S1, 0x140);
                frame.restore(c, S2, 0x130);
                frame.restore(c, S3, 0x120);
                frame.restore(c, S4, 0x110);
                frame.restore_ra(c, 0x100);
                frame.close(c);
                return Ok(());
            }
            _ => {
                c.jump_out(pc, UPDATE_ENTRY, UPDATE_END);
                return Ok(());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, DATA, GAME_GP};

    /// The object `InputSync` (self), the GameComm, the SSXApp world and the
    /// comm-system input-delay table, and the sample buffers.
    const SELF: u32 = DATA + 0x2000;
    const GC: u32 = DATA + 0x3000;
    const SAMPLES: u32 = DATA + 0x4000;
    const WORLD: u32 = DATA + 0x5000;
    const DELAY_TABLE: u32 = DATA + 0x6000;
    const DELAY_OBJ: u32 = DATA + 0x6100;
    const DELAY_VT: u32 = DATA + 0x6200;
    const LOCAL_PADS: u32 = DATA + 0x7000;

    /// Both sides from the same state. Remote seeds keep `writepos` far enough
    /// ahead that the network `while` loop never runs (its body calls
    /// `cGameComm_PopPacket`, which is not faked here); the local seeds
    /// keep `mSyncInput` at 0 so `SendSample` does not send.
    #[test]
    fn input_sync_update_matches() {
        check(ee::s06::f_002373b8__generated, input_sync_update, 200, &|c, seed| {
            let _r = machine(c, seed, 0x2373_b8, 0);
            c.pc = UPDATE_ENTRY;
            c.g[A0][0] = SELF as u64;
            let local = seed % 3 == 0;
            // The network branch: mSyncInput, no error, connected (see below).
            let net = !local && seed % 4 != 1 && seed % 4 != 2 && seed % 5 != 3;
            c.bus.write32(GAME_GP.wrapping_sub(0x204), GC);
            c.bus
                .write32(GC + 0x70, if seed % 7 == 0 { 10 } else { 14 + seed % 2 });
            c.bus
                .write32(GC + 0x40, if local || seed % 4 == 1 { 0 } else { 1 });
            c.bus.write32(GC + 4, if seed % 4 == 2 { 1 } else { 0 });
            c.bus.write32(GC, if seed % 5 == 3 { 0 } else { 1 });
            c.bus.write32(GC + 0x6c, 0);
            c.bus
                .write32(GC + 0x120, ((seed % 9) as f32 + 0.5).to_bits());
            c.bus.write32(GAME_GP.wrapping_sub(0x238), DELAY_TABLE);
            c.bus.write32(DELAY_TABLE, DELAY_OBJ);
            c.bus.write32(DELAY_OBJ + 8, DELAY_VT);
            c.bus.write32(DELAY_VT + 0x2c, FAKE);
            c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
            c.bus.write32(WORLD + 0x14, (1.0f32 / 60.0).to_bits());

            let count = (seed / 3 % 4) as u32;
            let latency = 4 + seed % 5;
            let numsamples = 1 + seed % 3;
            c.bus.write32(SELF, seed % 9);
            c.bus.write32(SELF + 4, latency);
            c.bus.write32(SELF + 8, numsamples);
            c.bus.write32(SELF + 0xc, seed.wrapping_mul(7));
            c.bus.write32(SELF + 0x10, count);
            for i in 0..count {
                let pad = SELF + 0x14 + 0x18 * i;
                let buffer = SAMPLES + 0x200 * i;
                c.bus.write32(pad, buffer);
                if local {
                    let read = (seed + i) % 3;
                    c.bus.write32(pad + 4, read);
                    c.bus.write32(pad + 8, read + (seed >> i) % 7);
                    c.bus.write32(pad + 0xc, LOCAL_PADS + 0x100 * i);
                    let source = LOCAL_PADS + 0x100 * i;
                    c.bus.write32(source, (seed + i) % 4);
                    for k in 0..4 {
                        c.bus
                            .write32(source + 4 + 4 * k, ((seed + k) as f32 * 0.25).to_bits());
                    }
                } else {
                    c.bus.write32(pad + 4, 0);
                    c.bus
                        .write32(pad + 8, if net { 16 } else { (seed >> i) % 7 });
                    c.bus.write32(pad + 0xc, 0);
                }
            }
        });
    }
}
