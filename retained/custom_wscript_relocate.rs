//! Native WorldScript relocation (0x003accd8), retained uninstalled.
//!
//! Generated EE is authoritative. Automated tests, builds and replays are
//! disabled for this writer by the user's instruction.

use crate::call::{call_ptr, guest, Frame};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(ENTRY, wscript_cache_relocate, "tWScriptCache_Relocate")];

const ENTRY: u32 = 0x003a_ccd8;
const END: u32 = 0x003a_d11c;
const DATA: u32 = 8;
const BANKS: u32 = 8;
const INSTANCE_REFS: u32 = 0x1c;
const COLLISION_REFS: u32 = 0x64;
const SPLINE_REFS: u32 = 0x44;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// Relocate one header pointer, retaining the native base/value registers.
#[inline]
fn header_pointer(c: &mut Cpu, base_register: usize, offset: u32) {
    c.g[base_register][0] = c.lw(at(c, S0, DATA));
    c.g[V0][0] = c.lw(at(c, base_register, offset));
    c.g[V0][0] = sx(r(c, base_register).wrapping_add(r(c, V0)));
    c.sw(at(c, base_register, offset), c.g[V0][0]);
}

/// `void tWScriptCache_Relocate(Item* item, World* world, int section)`.
/// Validate the version, relocate the script's offset tables, bind instance
/// properties/colliders, initialize rails and relocate process/mission code.
/// Rail attributes are packed {i16 flags, u16 surface}. The surface is sign
/// extended into WorldSpline+0x28; eligibility is duplicated into both halves
/// of WorldSpline+0x1c. Native rail spatial enable/disable is a separate API.
/// Every call return and event-check loop head can resume without relocating
/// already consumed offsets a second time.
pub fn wscript_cache_relocate(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x50 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 10;
                    Frame::open(c, 0x50);
                    frame.save(c, S0, 0x40);
                    frame.save(c, S1, 0x30);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save(c, S2, 0x20);
                    c.g[S1][0] = c.g[A1][0];
                    frame.save_ra(c, 0x10);
                    c.g[A3][0] = c.lw(at(c, S0, DATA));
                    let absent = c.g[A3][0] == 0;
                    c.g[S2][0] = c.g[A2][0];
                    pc = if absent { 0x003a_d0fc } else { 0x003a_cd00 };
                }
                0x003a_cd00 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lhu(r(c, A3));
                    c.g[V0][0] = 0x1000;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lh(at(c, S0, 2));
                        pc = 0x003a_cd2c;
                    } else {
                        c.cycles += 6;
                        c.g[A0][0] = c.lw(r(c, S1));
                        c.g[A1][0] = c.lw(at(c, A3, 0xc));
                        c.g[V0][0] = c.lw(r(c, A0));
                        c.g[V1][0] = c.lw(at(c, V0, 4));
                        if c.g[A1][0] == c.g[V1][0] {
                            c.g[V1][0] = c.lw(at(c, S0, DATA));
                            pc = 0x003a_cd44;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = c.lh(at(c, S0, 2));
                            pc = 0x003a_cd2c;
                        }
                    }
                }
                0x003a_cd2c => {
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.sw(at(c, S0, DATA), 0);
                        pc = 0x003a_cd40;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[A3][0];
                        call_ptr(c, 0x0031_7e98, 0x003a_cd3c)?;
                        pc = 0x003a_cd3c;
                    }
                }
                0x003a_cd3c => {
                    c.sw(at(c, S0, DATA), 0);
                    pc = 0x003a_cd40;
                }
                0x003a_cd40 => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw(at(c, S0, DATA));
                    pc = 0x003a_cd44;
                }
                0x003a_cd44 => {
                    c.cycles += 2;
                    let invalid = c.g[V1][0] == 0;
                    c.g[V0][0] = 3;
                    pc = if invalid { 0x003a_d100 } else { 0x003a_cd4c };
                }
                // Header pointers and the function descriptor table.
                0x003a_cd4c => {
                    c.cycles += 44;
                    c.g[V0][0] = c.lw(at(c, V1, 0x14));
                    c.g[A1][0] = 0;
                    c.g[V0][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
                    c.sw(at(c, V1, 0x14), c.g[V0][0]);
                    for (base, offset) in [(V1, 0x1c), (A0, 0x24), (V1, 0x2c),
                        (A0, 0x34), (V1, 0x3c), (A0, 0x40), (V1, 0x48),
                        (A0, 0x50), (V1, 0x58)] {
                        header_pointer(c, base, offset);
                    }
                    c.g[A0][0] = c.lw(at(c, S0, DATA));
                    c.g[V0][0] = c.lw(at(c, A0, 0x38));
                    let empty = (c.g[V0][0] as i64) <= 0;
                    c.g[V1][0] = c.lw(at(c, A0, 0x3c));
                    if empty { pc = 0x003a_ce28; } else {
                        c.cycles += 1;
                        pc = 0x003a_ce00;
                    }
                }
                0x003a_ce00 => {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw(r(c, V1));
                    c.g[A1][0] = sx(r(c, A1).wrapping_add(1));
                    c.g[V0][0] = sx(r(c, A0).wrapping_add(r(c, V0)));
                    c.sw(r(c, V1), c.g[V0][0]);
                    c.g[A0][0] = c.lw(at(c, S0, DATA));
                    c.g[V0][0] = c.lw(at(c, A0, 0x38));
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(4));
                    if more {
                        if back_edge_unwinds(c, 0x003a_ce00) { return Ok(()); }
                    } else {
                        c.cycles += 1;
                        c.g[A0][0] = c.lw(at(c, S0, DATA));
                        pc = 0x003a_ce28;
                    }
                }
                // Object property offsets are relative to the property blob.
                0x003a_ce28 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, A0, 0x4c));
                    let empty = (c.g[V0][0] as i64) <= 0;
                    c.g[A2][0] = 0;
                    if empty { pc = 0x003a_ce68; } else {
                        c.cycles += 1;
                        c.g[A1][0] = c.lw(at(c, A0, 0x50));
                        pc = 0x003a_ce38;
                    }
                }
                0x003a_ce38 => {
                    c.cycles += 12;
                    c.g[V1][0] = sx(r(c, A2) << 2);
                    c.g[A0][0] = c.lw(at(c, A0, 0x40));
                    c.g[A2][0] = sx(r(c, A2).wrapping_add(1));
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A1)));
                    c.g[V0][0] = c.lw(r(c, V1));
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V0)));
                    c.sw(r(c, V1), c.g[A0][0]);
                    c.g[A0][0] = c.lw(at(c, S0, DATA));
                    c.g[V0][0] = c.lw(at(c, A0, 0x4c));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[A1][0] = c.lw(at(c, A0, 0x50));
                        if back_edge_unwinds(c, 0x003a_ce38) { return Ok(()); }
                    } else { pc = 0x003a_ce68; }
                }
                // Bind the selected property to every compact instance RID.
                0x003a_ce68 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw(at(c, S0, DATA));
                    c.g[T0][0] = 0xffff_ffff_ffff_0000;
                    c.g[A3][0] = 0;
                    c.g[T0][0] |= 0xff00;
                    c.g[V1][0] = c.lw(at(c, V0, 0x44));
                    let empty = (c.g[V1][0] as i64) <= 0;
                    c.g[T0][0] = c.g[S2][0] | c.g[T0][0];
                    if empty { pc = 0x003a_cf38; } else {
                        c.cycles += 3;
                        c.g[T1][0] = 0xffff_ffff_ffff_0000;
                        c.g[A2][0] = c.lw(r(c, S1));
                        pc = 0x003a_ce90;
                    }
                }
                0x003a_ce90 => {
                    c.cycles += 8;
                    c.g[A1][0] = c.g[T0][0] & 255;
                    c.g[V0][0] = sx(r(c, A1) << 2);
                    c.g[V1][0] = sx(r(c, A3) << 8);
                    c.g[A0][0] = c.lw(at(c, A2, BANKS));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                    c.g[V0][0] = c.lw(r(c, V0));
                    let resident = c.g[V0][0] != 0;
                    c.g[T0][0] = c.g[A1][0] | c.g[V1][0];
                    if resident {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw(at(c, V0, INSTANCE_REFS));
                        c.g[V1][0] = sx(r(c, T0) >> 8);
                        c.g[V1][0] = sx(r(c, V1) << 2);
                        c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
                        c.g[A0][0] = c.lw(r(c, V1));
                        c.g[V0][0] = sx(r(c, A0) >> 8);
                        let found = c.g[V0][0] != 0;
                        c.g[V0][0] = sx(r(c, V0) << 2);
                        if !found { c.cycles += 1; c.g[V0][0] = 0; }
                        c.cycles += 1;
                        c.g[A2][0] = c.g[V0][0];
                    } else { c.cycles += 2; c.g[A2][0] = 0; }
                    c.cycles += 22;
                    c.g[V0][0] = c.lw(at(c, S0, DATA));
                    c.g[A0][0] = sx(r(c, A3) << 1);
                    c.g[A3][0] = sx(r(c, A3).wrapping_add(1));
                    c.g[V1][0] = c.lw(at(c, V0, 0x48));
                    c.g[A1][0] = c.lw(at(c, V0, 0x50));
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                    c.g[V0][0] = c.lhu(r(c, V1));
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A1)));
                    c.g[V1][0] = c.lw(r(c, V0));
                    c.sw(at(c, A2, 0x88), c.g[V1][0]);
                    c.g[V0][0] = c.lw(at(c, V1, 4));
                    c.g[V0][0] &= c.g[T1][0];
                    c.g[V1][0] = sx((r(c, V0) as i32 >> 16) as u32);
                    c.g[V0][0] |= c.g[V1][0];
                    c.g[V0][0] |= 2;
                    c.sw(at(c, A2, 8), c.g[V0][0]);
                    c.g[V1][0] = c.lw(at(c, S0, DATA));
                    c.g[V0][0] = c.lw(at(c, V1, 0x44));
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[A2][0] = c.lw(r(c, S1));
                        if back_edge_unwinds(c, 0x003a_ce90) { return Ok(()); }
                    } else { pc = 0x003a_cf38; }
                }
                // Collider handles belong only to native collision modes 1/3.
                0x003a_cf38 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(at(c, S0, DATA));
                    c.g[V0][0] = c.lw(at(c, V1, 0x4c));
                    let empty = (c.g[V0][0] as i64) <= 0;
                    c.g[A3][0] = 0;
                    if empty { pc = 0x003a_cfd8; } else {
                        c.cycles += 4;
                        c.g[T2][0] = 1;
                        c.g[T1][0] = 3;
                        c.g[V1][0] = c.lw(at(c, V1, 0x50));
                        pc = 0x003a_cf58;
                    }
                }
                0x003a_cf58 => {
                    c.cycles += 6;
                    c.g[V0][0] = sx(r(c, A3) << 2);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.g[A2][0] = c.lw(r(c, V0));
                    c.g[V0][0] = c.lw(r(c, A2));
                    let collider = if c.g[V0][0] == c.g[T2][0] {
                        c.g[A1][0] = c.lw(at(c, A2, 0xc));
                        true
                    } else {
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[T1][0] {
                            c.sw(at(c, A2, 0xc), 0);
                            false
                        } else {
                            c.cycles += 1;
                            c.g[A1][0] = c.lw(at(c, A2, 0xc));
                            true
                        }
                    };
                    if collider {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw(r(c, S1));
                        c.g[V1][0] = c.g[A1][0] & 255;
                        c.g[A0][0] = c.lw(at(c, V0, BANKS));
                        c.g[V1][0] = sx(r(c, V1) << 2);
                        c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                        c.g[V1][0] = c.lw(r(c, V1));
                        let absent = c.g[V1][0] == 0;
                        c.g[V0][0] = sx(r(c, A1) >> 8);
                        let mut found = false;
                        if !absent {
                            c.cycles += 7;
                            c.g[A0][0] = c.lw(at(c, V1, COLLISION_REFS));
                            c.g[V0][0] = sx(r(c, V0) << 2);
                            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                            c.g[V1][0] = c.lw(r(c, V0));
                            c.g[V0][0] = sx(r(c, V1) >> 8);
                            found = c.g[V0][0] != 0;
                            c.g[V0][0] = sx(r(c, V0) << 2);
                        }
                        if !found { c.cycles += 1; c.g[V0][0] = 0; }
                        c.cycles += 1;
                        c.sw(at(c, A2, 0xc), c.g[V0][0]);
                    }
                    c.cycles += 6;
                    c.g[V1][0] = c.lw(at(c, S0, DATA));
                    c.g[A3][0] = sx(r(c, A3).wrapping_add(1));
                    c.g[V0][0] = c.lw(at(c, V1, 0x4c));
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.lw(at(c, V1, 0x50));
                        if back_edge_unwinds(c, 0x003a_cf58) { return Ok(()); }
                    } else { pc = 0x003a_cfd8; }
                }
                // Initialize native rail surface and current/original flags.
                0x003a_cfd8 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(at(c, S0, DATA));
                    c.g[T1][0] = c.lw(at(c, V0, 0x54));
                    let empty = (c.g[T1][0] as i64) <= 0;
                    c.g[A3][0] = 0;
                    if empty { pc = 0x003a_d09c; } else {
                        c.cycles += 2;
                        c.g[T2][0] = 0xffff_ffff_ffff_0000;
                        pc = 0x003a_cff0;
                    }
                }
                0x003a_cff0 => {
                    c.cycles += 9;
                    c.g[A2][0] = c.lw(r(c, S1));
                    c.g[A1][0] = c.g[T0][0] & 255;
                    c.g[V0][0] = sx(r(c, A1) << 2);
                    c.g[V1][0] = sx(r(c, A3) << 8);
                    c.g[A0][0] = c.lw(at(c, A2, BANKS));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                    c.g[V0][0] = c.lw(r(c, V0));
                    let resident = c.g[V0][0] != 0;
                    c.g[T0][0] = c.g[A1][0] | c.g[V1][0];
                    if resident {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw(at(c, V0, SPLINE_REFS));
                        c.g[V1][0] = sx(r(c, T0) >> 8);
                        c.g[V1][0] = sx(r(c, V1) << 2);
                        c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
                        c.g[A0][0] = c.lw(r(c, V1));
                        c.g[V0][0] = sx(r(c, A0) >> 8);
                        let found = c.g[V0][0] != 0;
                        c.g[V0][0] = sx(r(c, V0) << 2);
                        if !found { c.cycles += 1; c.g[V0][0] = 0; }
                        c.cycles += 1;
                        c.g[A2][0] = c.g[V0][0];
                    } else { c.cycles += 2; c.g[A2][0] = 0; }
                    c.cycles += 21;
                    c.g[A0][0] = c.lw(at(c, S0, DATA));
                    c.g[V0][0] = sx(r(c, A3) << 2);
                    c.g[A3][0] = sx(r(c, A3).wrapping_add(1));
                    c.g[V1][0] = c.lw(at(c, A0, 0x58));
                    c.g[A1][0] = ((c.g[A3][0] as i64) < (c.g[T1][0] as i64)) as u64;
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.lwl(V1, at(c, V0, 3));
                    c.lwr(V1, r(c, V0));
                    c.swl(V1, at(c, SP, 3));
                    c.swr(V1, r(c, SP));
                    c.g[V1][0] = c.lhu(at(c, SP, 2));
                    c.g[V1][0] = sx(r(c, V1) << 16);
                    c.g[V1][0] = sx((r(c, V1) as i32 >> 16) as u32);
                    c.sw(at(c, A2, 0x28), c.g[V1][0]);
                    c.g[V0][0] = c.lh(r(c, SP));
                    c.g[V0][0] = sx(r(c, V0) << 16);
                    c.g[V1][0] = sx(r(c, V0) >> 16);
                    c.g[V0][0] &= c.g[T2][0];
                    c.g[V0][0] |= c.g[V1][0];
                    let more = c.g[A1][0] != 0;
                    c.sw(at(c, A2, 0x1c), c.g[V0][0]);
                    if more {
                        if back_edge_unwinds(c, 0x003a_cff0) { return Ok(()); }
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = c.lw(at(c, S0, DATA));
                        pc = 0x003a_d09c;
                    }
                }
                // Processes have 0x40-byte descriptors, missions 0x54 bytes.
                0x003a_d09c => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(at(c, V0, 0x28));
                    let empty = (c.g[V1][0] as i64) <= 0;
                    c.g[A0][0] = c.lw(at(c, V0, 0x2c));
                    if empty { pc = 0x003a_d0cc; } else {
                        c.cycles += 2;
                        c.g[A1][0] = c.g[V1][0];
                        pc = 0x003a_d0b0;
                    }
                }
                0x003a_d0b0 => {
                    c.cycles += 7;
                    c.g[V1][0] = c.lw(at(c, A0, 8));
                    c.g[A1][0] = sx(r(c, A1).wrapping_sub(1));
                    c.g[V0][0] = c.lw(at(c, S0, DATA));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.sw(at(c, A0, 8), c.g[V0][0]);
                    let more = c.g[A1][0] != 0;
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(0x40));
                    if more {
                        if back_edge_unwinds(c, 0x003a_d0b0) { return Ok(()); }
                    } else { pc = 0x003a_d0cc; }
                }
                0x003a_d0cc => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(at(c, S0, DATA));
                    c.g[V1][0] = c.lw(at(c, V0, 0x20));
                    let empty = (c.g[V1][0] as i64) <= 0;
                    c.g[A0][0] = c.lw(at(c, V0, 0x24));
                    if empty { pc = 0x003a_d0fc; } else {
                        c.cycles += 1;
                        c.g[A1][0] = c.g[V1][0];
                        pc = 0x003a_d0e0;
                    }
                }
                0x003a_d0e0 => {
                    c.cycles += 7;
                    c.g[V1][0] = c.lw(at(c, A0, 8));
                    c.g[A1][0] = sx(r(c, A1).wrapping_sub(1));
                    c.g[V0][0] = c.lw(at(c, S0, DATA));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.sw(at(c, A0, 8), c.g[V0][0]);
                    let more = c.g[A1][0] != 0;
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(0x54));
                    if more {
                        if back_edge_unwinds(c, 0x003a_d0e0) { return Ok(()); }
                    } else { pc = 0x003a_d0fc; }
                }
                0x003a_d0fc => {
                    c.cycles += 1;
                    c.g[V0][0] = 3;
                    pc = 0x003a_d100;
                }
                0x003a_d100 => {
                    c.cycles += 7;
                    c.sh(r(c, S0), c.g[V0][0]);
                    frame.restore(c, S0, 0x40);
                    frame.restore(c, S1, 0x30);
                    frame.restore(c, S2, 0x20);
                    frame.restore_ra(c, 0x10);
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
