//! Native SND_BankRelocatePatches (0x3b6788), retained uninstalled.
//!
//! The generated EE body defines registers, memory, cycles and call order.
//! No automated test, build, replay or game execution accompanies this port.

use crate::call::{call_ptr, guest, Frame};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x003b_6788;
const END: u32 = 0x003b_6820;
const PARSE_NEXT: u32 = 0x003b_67c0;
const AFTER_PARSE: u32 = 0x003b_67c8;
const AFTER_RELOCATE: u32 = 0x003b_67e8;
const AFTER_EVENTS: u32 = 0x003b_67f4;
const FRAME_BYTES: u32 = 0x140;
const CURSOR: u32 = 0xd0;
const PATCH_MASK: u32 = 0x18;
const PARSE_PATCH: u32 = 0x003b_aac0;
const RELOCATE_PATCH: u32 = 0x003c_3ea8;
const FIRE_EVENTS: u32 = 0x003b_5f98;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// `int SND_BankRelocatePatches(mask, patch_stream, arg2, arg3)`.
/// Parse records after the stream's platform dword. Records whose parsed
/// +0x18 flags intersect `mask` are passed to the native relocation callback
/// and then its event dispatcher. The callback owns the actual relocation;
/// this function does not identify or rewrite individual serialized tags.
/// Preserve all native call returns and the event-check loop head.
pub fn snd_bank_relocate_patches(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: FRAME_BYTES };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 14;
                    Frame::open(c, FRAME_BYTES);
                    c.g[A1][0] = sx(r(c, A1).wrapping_add(4));
                    frame.save(c, S1, 0x120);
                    frame.save(c, S2, 0x110);
                    c.g[S1][0] = sx(r(c, SP).wrapping_add(CURSOR));
                    frame.save(c, S3, 0x100);
                    c.g[S2][0] = c.g[A3][0];
                    frame.save(c, S4, 0xf0);
                    c.g[S3][0] = c.g[A2][0];
                    frame.save(c, S0, 0x130);
                    c.g[S4][0] = c.g[A0][0];
                    frame.save_ra(c, 0xe0);
                    c.sw(frame.at(c, CURSOR), c.g[A1][0]);
                    c.g[A0][0] = c.g[S1][0];
                    pc = PARSE_NEXT;
                }
                PARSE_NEXT => {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, PARSE_PATCH, AFTER_PARSE)?;
                    pc = AFTER_PARSE;
                }
                AFTER_PARSE => {
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = c.lhu(frame.at(c, PATCH_MASK));
                    c.g[V0][0] &= c.g[S4][0];
                    let relocate = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[SP][0];
                    if relocate {
                        c.cycles += 6;
                        c.g[A1][0] = c.g[S3][0];
                        c.g[A2][0] = c.g[S2][0];
                        call_ptr(c, RELOCATE_PATCH, AFTER_RELOCATE)?;
                        pc = AFTER_RELOCATE;
                    } else {
                        pc = AFTER_EVENTS;
                    }
                }
                AFTER_RELOCATE => {
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, FIRE_EVENTS, AFTER_EVENTS)?;
                    pc = AFTER_EVENTS;
                }
                AFTER_EVENTS => {
                    c.cycles += 2;
                    let more = c.g[S0][0] != 0;
                    c.g[A0][0] = c.g[S1][0];
                    if more {
                        if back_edge_unwinds(c, PARSE_NEXT) {
                            return Ok(());
                        }
                        pc = PARSE_NEXT;
                    } else {
                        c.cycles += 9;
                        frame.restore(c, S0, 0x130);
                        c.g[V0][0] = 0;
                        frame.restore(c, S1, 0x120);
                        frame.restore(c, S2, 0x110);
                        frame.restore(c, S3, 0x100);
                        frame.restore(c, S4, 0xf0);
                        frame.restore_ra(c, 0xe0);
                        frame.close(c);
                        return Ok(());
                    }
                }
                pc => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
        }
    });
}
