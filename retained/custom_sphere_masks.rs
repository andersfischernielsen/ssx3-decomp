//! Native sphere-tree occupancy-mask cache (0x0032df28), retained uninstalled.
//!
//! Generated EE is authoritative. No tests, cargo, game or replay were run;
//! automated validation is disabled by the user's instruction.

use crate::call::{call_ptr, guest, Frame};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(ENTRY, sphere_tree_cache_get_masks, "SphereTreeCache_GetMasks")];

const ENTRY: u32 = 0x0032_df28;
const END: u32 = 0x0032_e100;
const UNPACK_MASKS: u32 = 0x0032_b620;
const SMALL_BUFFERS: u32 = 0xed8;
const SMALL_KEYS: u32 = 0xedc;
const SMALL_COUNT: u32 = 0xee0;
const SMALL_NEXT: u32 = 0xee4;
const LARGE_BUFFERS: u32 = 0xee8;
const LARGE_KEYS: u32 = 0xeec;
const LARGE_COUNT: u32 = 0xef0;
const LARGE_NEXT: u32 = 0xef4;
const DEPTH_SPLIT: u32 = 0xef8;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `u8* SphereTreeCache_GetMasks(SphereData* data)`.
/// Choose the small/large cache using data+0xc and the GP-relative threshold.
/// A matching key at the remembered slot or elsewhere returns that buffer;
/// a miss unpacks data+0x24 into the next cyclic buffer and records its key.
/// The encoded/raw discriminator is not inspected here: callers invoke this
/// cache when their occupancy-mask pointer needs materializing.
pub fn sphere_tree_cache_get_masks(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x30 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 10;
                    Frame::open(c, 0x30);
                    c.g[V1][0] = c.lw(at(c, GP, DEPTH_SPLIT));
                    frame.save(c, S1, 0x10);
                    frame.save(c, S0, 0x20);
                    c.g[S1][0] = sx(at(c, A0, 4));
                    frame.save_ra(c, 0);
                    c.g[V0][0] = c.lw(at(c, A0, 0xc));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let small = c.g[V0][0] != 0;
                    c.g[S0][0] = c.lw(r(c, A0));
                    pc = if small { 0x0032_df50 } else { 0x0032_e008 };
                }
                0x0032_df50 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw(at(c, A0, 4));
                    c.g[A2][0] = c.lw(at(c, GP, SMALL_KEYS));
                    c.g[A1][0] = sx(r(c, V0) << 2);
                    c.g[V1][0] = sx(r(c, A1).wrapping_add(r(c, A2)));
                    c.g[V0][0] = c.lw(r(c, V1));
                    let hit = c.g[V0][0] == c.g[S0][0];
                    c.g[V0][0] = c.lw(at(c, GP, SMALL_COUNT));
                    pc = if hit { 0x0032_df6c } else { 0x0032_df78 };
                }
                0x0032_df6c => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(at(c, GP, SMALL_BUFFERS));
                    c.g[V1][0] = sx(r(c, A1).wrapping_add(r(c, V1)));
                    pc = 0x0032_e0e8;
                }
                0x0032_df78 => {
                    c.cycles += 2;
                    let empty = (c.g[V0][0] as i64) <= 0;
                    c.g[V1][0] = 0;
                    pc = if empty { 0x0032_dfa4 } else { 0x0032_df80 };
                }
                0x0032_df80 => {
                    c.cycles += 2;
                    c.g[A3][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[A2][0];
                    pc = 0x0032_df88;
                }
                0x0032_df88 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r(c, A1));
                    let hit = c.g[V0][0] == c.g[S0][0];
                    c.g[A2][0] = sx(r(c, V1) << 2);
                    pc = if hit { 0x0032_e030 } else { 0x0032_df94 };
                }
                0x0032_df94 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(1));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A1][0] = sx(r(c, A1).wrapping_add(4));
                    pc = if more { 0x0032_df88 } else { 0x0032_dfa4 };
                    if more && back_edge_unwinds(c, pc) { return Err(crate::call::Unwound) }
                }
                0x0032_dfa4 => {
                    c.cycles += 21;
                    c.g[V0][0] = c.lw(at(c, GP, SMALL_NEXT));
                    c.g[V1][0] = c.lw(at(c, GP, SMALL_BUFFERS));
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[A1][0] = c.lw(at(c, A0, 0x24));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.g[A0][0] = c.lw(r(c, V0));
                    call_ptr(c, UNPACK_MASKS, 0x0032_dfc0)?;
                    pc = 0x0032_dfc0;
                }
                0x0032_dfc0 => {
                    c.g[V1][0] = c.lw(at(c, GP, SMALL_NEXT));
                    c.g[V0][0] = c.lw(at(c, GP, SMALL_KEYS));
                    c.g[V1][0] = sx(r(c, V1) << 2);
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
                    c.sw(r(c, V1), c.g[S0][0]);
                    c.g[A0][0] = c.lw(at(c, GP, SMALL_NEXT));
                    c.sw(r(c, S1), c.g[A0][0]);
                    c.g[V0][0] = c.lw(at(c, GP, SMALL_NEXT));
                    c.g[V1][0] = c.lw(at(c, GP, SMALL_COUNT));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(1));
                    c.div(0, V0, V1);
                    c.sw(at(c, GP, SMALL_NEXT), c.g[V0][0]);
                    pc = 0x0032_dff8;
                }
                0x0032_dff8 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(at(c, GP, SMALL_BUFFERS));
                    c.g[A0][0] = c.hi[0];
                    c.sw(at(c, GP, SMALL_NEXT), c.g[A0][0]);
                    pc = 0x0032_e0dc;
                }
                0x0032_e008 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw(at(c, A0, 4));
                    c.g[A2][0] = c.lw(at(c, GP, LARGE_KEYS));
                    c.g[A1][0] = sx(r(c, V0) << 2);
                    c.g[V1][0] = sx(r(c, A1).wrapping_add(r(c, A2)));
                    c.g[V0][0] = c.lw(r(c, V1));
                    let hit = c.g[V0][0] == c.g[S0][0];
                    c.g[V0][0] = c.lw(at(c, GP, LARGE_COUNT));
                    pc = if hit { 0x0032_e024 } else { 0x0032_e050 };
                }
                0x0032_e024 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(at(c, GP, LARGE_BUFFERS));
                    c.g[V1][0] = sx(r(c, A1).wrapping_add(r(c, V1)));
                    pc = 0x0032_e0e8;
                }
                0x0032_e030 | 0x0032_e040 => {
                    c.cycles += 4;
                    c.sw(r(c, S1), c.g[V1][0]);
                    c.g[V1][0] = c.lw(at(c, GP, if pc == 0x0032_e030 { SMALL_BUFFERS } else { LARGE_BUFFERS }));
                    c.g[V1][0] = sx(r(c, A2).wrapping_add(r(c, V1)));
                    pc = 0x0032_e0e8;
                }
                0x0032_e050 => {
                    c.cycles += 2;
                    let empty = (c.g[V0][0] as i64) <= 0;
                    c.g[V1][0] = 0;
                    pc = if empty { 0x0032_e07c } else { 0x0032_e058 };
                }
                0x0032_e058 => {
                    c.cycles += 2;
                    c.g[A3][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[A2][0];
                    pc = 0x0032_e060;
                }
                0x0032_e060 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r(c, A1));
                    let hit = c.g[V0][0] == c.g[S0][0];
                    c.g[A2][0] = sx(r(c, V1) << 2);
                    pc = if hit { 0x0032_e040 } else { 0x0032_e06c };
                    if hit && back_edge_unwinds(c, pc) { return Err(crate::call::Unwound) }
                }
                0x0032_e06c => {
                    c.cycles += 4;
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(1));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A1][0] = sx(r(c, A1).wrapping_add(4));
                    pc = if more { 0x0032_e060 } else { 0x0032_e07c };
                    if more && back_edge_unwinds(c, pc) { return Err(crate::call::Unwound) }
                }
                0x0032_e07c => {
                    c.cycles += 21;
                    c.g[V0][0] = c.lw(at(c, GP, LARGE_NEXT));
                    c.g[V1][0] = c.lw(at(c, GP, LARGE_BUFFERS));
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[A1][0] = c.lw(at(c, A0, 0x24));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.g[A0][0] = c.lw(r(c, V0));
                    call_ptr(c, UNPACK_MASKS, 0x0032_e098)?;
                    pc = 0x0032_e098;
                }
                0x0032_e098 => {
                    c.g[V1][0] = c.lw(at(c, GP, LARGE_NEXT));
                    c.g[V0][0] = c.lw(at(c, GP, LARGE_KEYS));
                    c.g[V1][0] = sx(r(c, V1) << 2);
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
                    c.sw(r(c, V1), c.g[S0][0]);
                    c.g[A0][0] = c.lw(at(c, GP, LARGE_NEXT));
                    c.sw(r(c, S1), c.g[A0][0]);
                    c.g[V0][0] = c.lw(at(c, GP, LARGE_NEXT));
                    c.g[V1][0] = c.lw(at(c, GP, LARGE_COUNT));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(1));
                    c.div(0, V0, V1);
                    c.sw(at(c, GP, LARGE_NEXT), c.g[V0][0]);
                    pc = 0x0032_e0d0;
                }
                0x0032_e0d0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, GP, LARGE_BUFFERS));
                    c.g[A0][0] = c.hi[0];
                    c.sw(at(c, GP, LARGE_NEXT), c.g[A0][0]);
                    pc = 0x0032_e0dc;
                }
                0x0032_e0dc => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(r(c, S1));
                    c.g[V1][0] = sx(r(c, V1) << 2);
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
                    pc = 0x0032_e0e8;
                }
                0x0032_e0e8 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(r(c, V1));
                    frame.restore(c, S0, 0x20);
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => { c.jump_out(pc, ENTRY, END); return Ok(()) }
            }
        }
    });
}
