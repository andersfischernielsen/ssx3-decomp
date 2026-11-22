//! Native section allocator (0x003a7878), retained uninstalled.
//!
//! Generated EE is authoritative. No tests, builds or replays are added or
//! run: automated validation is disabled by the user's instruction.

use crate::call::{call_ptr, guest, Frame};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (ENTRY, section_allocator_alloc, "SectionAllocator_Alloc"),
];

const ENTRY: u32 = 0x003a_7878;
const END: u32 = 0x003a_7a20;
const POOL: u32 = 0;
const BLOCK_COUNT: u32 = 8;
const HEAD: u32 = 0xc;
const CURRENT: u32 = 0x10;
const DATA: u32 = 0;
const CAPACITY: u32 = 4;
const USED: u32 = 8;
const PADDING: u32 = 0xc;
const NEXT: u32 = 0x14;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void* SectionAllocator_Alloc(SectionAllocator* a, int size, u32 flags)`.
/// Align within a current/previous block to `4 << ((flags >> 24) & 15)`.
/// Existing blocks must have strictly more room than padding plus size. If
/// none fits, claim a pool block, link it to the section, and return its data.
///
/// Native behavior does NOT check a freshly claimed block's capacity against
/// the request. An importer/runtime extension must provide a large enough
/// backing allocation before this native path records `used += size`.
/// Block fields are {data,capacity,used,padding,...,next}; section fields are
/// {pool,...,blockCount,head,current}. Pool ownership supplies the lifetime.
pub fn section_allocator_alloc(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x40 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 10;
                    Frame::open(c, 0x40);
                    frame.save(c, S0, 0x30);
                    frame.save(c, S1, 0x20);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save(c, S2, 0x10);
                    c.g[S1][0] = c.g[A1][0];
                    frame.save_ra(c, 0);
                    c.g[V0][0] = c.lw(at(c, S0, BLOCK_COUNT));
                    let existing = c.g[V0][0] != 0;
                    c.g[S2][0] = c.g[A2][0];
                    pc = if existing { 0x003a_78cc } else { 0x003a_78a0 };
                }
                0x003a_78a0 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.lw(at(c, S0, POOL));
                    call_ptr(c, 0x003a_7768, 0x003a_78a8)?;
                    pc = 0x003a_78a8;
                }
                0x003a_78a8 => {
                    c.sw(at(c, V0, USED), 0);
                    c.sw(at(c, V0, PADDING), 0);
                    c.g[V1][0] = c.lw(at(c, S0, HEAD));
                    c.sw(at(c, V0, NEXT), c.g[V1][0]);
                    c.sw(at(c, S0, CURRENT), c.g[V0][0]);
                    c.g[V1][0] = c.lw(at(c, S0, BLOCK_COUNT));
                    c.sw(at(c, S0, HEAD), c.g[V0][0]);
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(1));
                    c.sw(at(c, S0, BLOCK_COUNT), c.g[V1][0]);
                    pc = 0x003a_78cc;
                }
                0x003a_78cc => {
                    c.cycles += 9;
                    c.g[T3][0] = c.lw(at(c, S0, CURRENT));
                    c.g[V0][0] = sx(r(c, S2) >> 24);
                    c.g[V0][0] &= 0xf;
                    c.g[V1][0] = 4;
                    c.g[A3][0] = c.lw(at(c, T3, USED));
                    c.g[A2][0] = sx(r(c, V1) << (r(c, V0) & 31));
                    c.g[T1][0] = 0;
                    pc = 0x003a_78f0;
                }
                0x003a_78f0 => {
                    c.cycles += 10;
                    c.div(0, A3, A2);
                    c.g[A1][0] = c.lw(at(c, T3, CAPACITY));
                    c.g[A0][0] = c.hi[0];
                    c.g[V1][0] = sx(r(c, A2).wrapping_sub(r(c, A0)));
                    if c.g[A0][0] != 0 { c.g[T1][0] = c.g[V1][0]; }
                    c.g[V0][0] = sx(r(c, A3).wrapping_add(r(c, T1)));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S1)));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    let no_room = c.g[V0][0] == 0;
                    c.g[V1][0] = sx(r(c, T1).wrapping_add(r(c, S1)));
                    pc = if no_room { 0x003a_7954 } else { 0x003a_7918 };
                }
                0x003a_7918 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(at(c, T3, DATA));
                    c.g[V1][0] = sx(r(c, A3).wrapping_add(r(c, V1)));
                    c.sw(at(c, T3, USED), c.g[V1][0]);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A3)));
                    pc = 0x003a_793c;
                }
                0x003a_792c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(at(c, A3, DATA));
                    c.g[V1][0] = sx(r(c, T0).wrapping_add(r(c, V1)));
                    c.sw(at(c, A3, USED), c.g[V1][0]);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, T0)));
                    pc = 0x003a_793c;
                }
                0x003a_793c => {
                    c.cycles += 6;
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, T1)));
                    c.g[A0][0] = c.lw(at(c, S0, CURRENT));
                    c.g[V1][0] = c.lw(at(c, A0, PADDING));
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, T1)));
                    c.sw(at(c, A0, PADDING), c.g[V1][0]);
                    pc = 0x003a_7a08;
                }
                0x003a_7954 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(at(c, S0, BLOCK_COUNT));
                    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                    pc = if c.g[V0][0] != 0 { 0x003a_79c8 } else { 0x003a_7964 };
                }
                0x003a_7964 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, S0, HEAD));
                    let empty = c.g[V0][0] == 0;
                    c.sw(at(c, S0, CURRENT), c.g[V0][0]);
                    pc = if empty { 0x003a_79c8 } else { 0x003a_7970 };
                }
                0x003a_7970 => {
                    c.cycles += 2;
                    c.g[T4][0] = 0;
                    pc = 0x003a_7978;
                }
                0x003a_7978 => {
                    c.cycles += 3;
                    c.g[A3][0] = c.lw(at(c, S0, CURRENT));
                    let first = c.g[A3][0] == c.g[T3][0];
                    c.g[T2][0] = c.g[A3][0];
                    pc = if first { 0x003a_79bc } else { 0x003a_7984 };
                }
                0x003a_7984 => {
                    c.cycles += 4;
                    c.g[T0][0] = c.lw(at(c, A3, USED));
                    c.g[T1][0] = 0;
                    pc = 0x003a_7994;
                }
                0x003a_7994 => {
                    c.cycles += 10;
                    c.div(0, T0, A2);
                    c.g[A1][0] = c.lw(at(c, A3, CAPACITY));
                    c.g[A0][0] = c.hi[0];
                    c.g[V1][0] = sx(r(c, A2).wrapping_sub(r(c, A0)));
                    if c.g[A0][0] != 0 { c.g[T1][0] = c.g[V1][0]; }
                    c.g[V0][0] = sx(r(c, T0).wrapping_add(r(c, T1)));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S1)));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    let fits = c.g[V0][0] != 0;
                    c.g[V1][0] = sx(r(c, T1).wrapping_add(r(c, S1)));
                    if fits {
                        if back_edge_unwinds(c, 0x003a_792c) { return Err(crate::call::Unwound) }
                        pc = 0x003a_792c;
                    } else {
                        pc = 0x003a_79bc;
                    }
                }
                0x003a_79bc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, T2, NEXT));
                    let more = c.g[V0][0] != 0;
                    c.sw(at(c, S0, CURRENT), c.g[V0][0]);
                    if more {
                        if back_edge_unwinds(c, 0x003a_7978) { return Err(crate::call::Unwound) }
                        pc = 0x003a_7978;
                    } else {
                        pc = 0x003a_79c8;
                    }
                }
                0x003a_79c8 => {
                    c.cycles += 16;
                    c.g[A0][0] = c.lw(at(c, S0, POOL));
                    call_ptr(c, 0x003a_7768, 0x003a_79d0)?;
                    pc = 0x003a_79d0;
                }
                0x003a_79d0 => {
                    c.sw(at(c, V0, USED), 0);
                    c.sw(at(c, V0, PADDING), 0);
                    c.g[V1][0] = c.lw(at(c, S0, HEAD));
                    c.sw(at(c, V0, NEXT), c.g[V1][0]);
                    c.sw(at(c, S0, CURRENT), c.g[V0][0]);
                    c.g[A0][0] = c.lw(at(c, S0, BLOCK_COUNT));
                    c.sw(at(c, S0, HEAD), c.g[V0][0]);
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(1));
                    c.sw(at(c, S0, BLOCK_COUNT), c.g[A0][0]);
                    c.g[V1][0] = c.lw(at(c, V0, USED));
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, S1)));
                    c.sw(at(c, V0, USED), c.g[V1][0]);
                    c.g[A0][0] = c.lw(at(c, S0, CURRENT));
                    c.g[V0][0] = c.lw(at(c, A0, DATA));
                    pc = 0x003a_7a08;
                }
                0x003a_7a08 => {
                    c.cycles += 6;
                    frame.restore(c, S0, 0x30);
                    frame.restore(c, S1, 0x20);
                    frame.restore(c, S2, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
        }
    });
}
