//! Native World_ResetSections (0x003a6800), retained uninstalled.
//!
//! The generated EE body is authoritative. No automated tests, builds or
//! replays were added or run, in accordance with the user's instruction.

use crate::call::{call_ptr, guest, Frame};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x003a_6800,
    world_reset_sections,
    "World_ResetSections",
)];

const ENTRY: u32 = 0x003a_6800;
const END: u32 = 0x003a_6928;
const INSTANCE_FLAGS: u32 = 8;
const INSTANCE_REFS: u32 = 0x1c;
const SECTION_STRIDE: u32 = 0x58;
const SECTION_INSTANCE_COUNT: u32 = 0x26;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void World_ResetSections(World* world)` (0x3a6800).
/// Restore every resident instance's current flags from its saved upper
/// halfword, then set the native static-instance bit 1. Missing section
/// tables and missing compact references are skipped. The original signed
/// right shift is intentional: an upper bit 31 set sign-extends the result.
///
/// This resets instance flags only; it does not traverse WorldSpline entries
/// or reset spline eligibility. Both nested loop back edges and the native
/// stream-query return sites remain resumable.
pub fn world_reset_sections(c: &mut Cpu) {
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
                    c.g[S0][0] = 0xffff_ffff_ffff_0000;
                    frame.save(c, S2, 0x10);
                    c.g[S0][0] |= 0xffff;
                    frame.save_ra(c, 0);
                    c.g[S2][0] = c.g[A0][0];
                    c.g[S1][0] = 0;
                    pc = 0x003a_68f8;
                }
                0x003a_6828 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = sx(at(c, A0, 0x10));
                    call_ptr(c, 0x003a_9858, 0x003a_6834)?;
                    pc = 0x003a_6834;
                }
                0x003a_6834 => {
                    let missing = c.g[V0][0] == 0;
                    c.g[T2][0] = sx(r(c, S1).wrapping_add(1));
                    pc = if missing { 0x003a_68f4 } else { 0x003a_683c };
                }
                0x003a_683c => {
                    c.cycles += 13;
                    c.g[A0][0] = c.lw(r(c, S2));
                    c.g[V1][0] = SECTION_STRIDE as u64;
                    c.g[V0][0] = 0xffff_ffff_ffff_ff00;
                    c.mult(0, V1, S1, V1);
                    c.g[A1][0] = c.lw(r(c, A0));
                    c.g[S0][0] &= c.g[V0][0];
                    c.g[A2][0] = c.g[S1][0] & 0xff;
                    c.g[A3][0] = 0;
                    c.g[V0][0] = c.lw(at(c, A1, 0x14));
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
                    c.g[T1][0] = c.lh(at(c, V1, SECTION_INSTANCE_COUNT));
                    let empty = c.g[T1][0] as i64 <= 0;
                    c.g[S0][0] |= c.g[A2][0];
                    pc = if empty { 0x003a_68f4 } else { 0x003a_6870 };
                }
                0x003a_6870 => {
                    c.cycles += 2;
                    c.g[T3][0] = 0xffff_ffff_ffff_0000;
                    c.g[V0][0] = c.lw(r(c, S2));
                    pc = 0x003a_6878;
                }
                0x003a_6878 => {
                    c.cycles += 13;
                    c.g[V1][0] = c.g[S0][0] & 0xff;
                    c.g[A1][0] = sx(r(c, A3) << 8);
                    c.g[A0][0] = sx(r(c, V1) << 2);
                    c.g[A2][0] = c.lw(at(c, V0, 8));
                    c.g[S0][0] = c.g[V1][0] | c.g[A1][0];
                    c.g[A3][0] = sx(r(c, A3).wrapping_add(1));
                    c.g[V0][0] = sx(r(c, S0) >> 8);
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, A2)));
                    c.g[T0][0] = ((c.g[A3][0] as i64) < (c.g[T1][0] as i64)) as u64;
                    c.g[A0][0] = c.lw(r(c, A0));
                    c.g[V1][0] = sx(r(c, V0) << 2);
                    let missing = c.g[A0][0] == 0;
                    c.g[A1][0] = 0;
                    pc = if missing { 0x003a_68cc } else { 0x003a_68ac };
                }
                0x003a_68ac => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(at(c, A0, INSTANCE_REFS));
                    c.g[V0][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
                    c.g[V1][0] = c.lw(r(c, V0));
                    c.g[V0][0] = sx(r(c, V1) >> 8);
                    let missing = c.g[V0][0] == 0;
                    c.g[A0][0] = 0;
                    pc = if missing { 0x003a_68c8 } else { 0x003a_68c4 };
                }
                0x003a_68c4 => {
                    c.cycles += 1;
                    c.g[A0][0] = sx(r(c, V0) << 2);
                    pc = 0x003a_68c8;
                }
                0x003a_68c8 => {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[A0][0];
                    pc = 0x003a_68cc;
                }
                0x003a_68cc => {
                    c.cycles += 2;
                    pc = if c.g[A1][0] == 0 { 0x003a_68ec } else { 0x003a_68d4 };
                }
                0x003a_68d4 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(at(c, A1, INSTANCE_FLAGS));
                    c.g[V0][0] &= c.g[T3][0];
                    c.g[V1][0] = sx(((r(c, V0) as i32) >> 16) as u32);
                    c.g[V0][0] |= c.g[V1][0];
                    c.g[V0][0] |= 2;
                    c.sw(at(c, A1, INSTANCE_FLAGS), c.g[V0][0]);
                    pc = 0x003a_68ec;
                }
                0x003a_68ec => {
                    c.cycles += 2;
                    if c.g[T0][0] != 0 {
                        c.g[V0][0] = c.lw(r(c, S2));
                        if back_edge_unwinds(c, 0x003a_6878) {
                            return Err(crate::call::Unwound);
                        }
                        pc = 0x003a_6878;
                    } else {
                        pc = 0x003a_68f4;
                    }
                }
                0x003a_68f4 => {
                    c.cycles += 1;
                    c.g[S1][0] = c.g[T2][0];
                    pc = 0x003a_68f8;
                }
                0x003a_68f8 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw(r(c, S2));
                    c.g[A0][0] = sx(at(c, A0, 0x10));
                    call_ptr(c, 0x003a_9800, 0x003a_6904)?;
                    pc = 0x003a_6904;
                }
                0x003a_6904 => {
                    c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = c.lw(r(c, S2));
                        if back_edge_unwinds(c, 0x003a_6828) {
                            return Err(crate::call::Unwound);
                        }
                        pc = 0x003a_6828;
                    } else {
                        pc = 0x003a_6910;
                    }
                }
                0x003a_6910 => {
                    c.cycles += 6;
                    frame.restore(c, S0, 0x30);
                    frame.restore(c, S1, 0x20);
                    frame.restore(c, S2, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                other => {
                    c.jump_out(other, ENTRY, END);
                    return Ok(());
                }
            }
        }
    });
}
