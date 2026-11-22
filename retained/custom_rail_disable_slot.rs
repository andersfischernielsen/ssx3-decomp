//! Native resource-slot spatial removal (0x00358780), retained uninstalled.
//!
//! This function resolves the resource table at bank+0x44 and removes linked
//! spline segments from spatial list 2. No tests, builds or replays were run.

use crate::call::{call_ptr, guest, Frame};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY, rail_man_disable_rail, "RailMan_DisableRail",
)];

const ENTRY: u32 = 0x0035_8780;
const END: u32 = 0x0035_8994;
const SLOT_KEYS: u32 = 0x18;
const RESOURCE_TABLE: u32 = 0x44;
const ITEM_COUNT: u32 = 0x20;
const FIRST_ITEM: u32 = 0x24;
const NEXT_ITEM: u32 = 0x64;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void RailMan_DisableRail(manager, slotIndex)` (0x358780).
/// Resolve the slot's compact key through section `key & 255`, resource
/// index `key >> 8`, and the section's +0x44 packed-reference table. For
/// each linked segment, construct its authored AABB and remove it from spatial
/// list 2. Free an octant root only when its eight children and three lists
/// are all empty. The slot key and resource's property flags are untouched.
/// The original assumes the resource resolves; a missing reference retains
/// its original null-page reads rather than introducing a new policy here.
pub fn rail_man_disable_rail(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xa0 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 20;
                    Frame::open(c, 0xa0);
                    c.g[A1][0] = sx(r(c, A1) << 2);
                    frame.save(c, S1, 0x70);
                    c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
                    frame.save(c, S2, 0x60);
                    frame.save(c, S3, 0x50);
                    frame.save(c, S4, 0x40);
                    frame.save_ra(c, 0x30);
                    frame.save_fpr(c, 20, 0x90);
                    frame.save(c, S0, 0x80);
                    c.g[S0][0] = c.lw(at(c, A1, SLOT_KEYS));
                    call_ptr(c, 0x002d_1bd8, 0x0035_87b0)?;
                    pc = 0x0035_87b0;
                }
                0x0035_87b0 => {
                    c.g[A0][0] = c.lw(r(c, V0));
                    c.g[V0][0] = c.g[S0][0] & 255;
                    c.g[V1][0] = c.lw(at(c, A0, 8));
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.g[V1][0] = c.lw(r(c, V0));
                    if c.g[V1][0] != 0 {
                        c.g[A0][0] = c.lw(at(c, V1, RESOURCE_TABLE));
                        pc = 0x0035_87d8;
                    } else {
                        c.cycles += 2;
                        c.g[S4][0] = 0;
                        pc = 0x0035_87fc;
                    }
                }
                0x0035_87d8 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx(r(c, S0) >> 8);
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                    c.g[V1][0] = c.lw(r(c, V0));
                    c.g[V0][0] = sx(r(c, V1) >> 8);
                    let resident = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    if !resident {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                    c.cycles += 1;
                    c.g[S4][0] = c.g[V0][0];
                    pc = 0x0035_87fc;
                }
                0x0035_87fc => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(at(c, S4, ITEM_COUNT));
                    c.g[S2][0] = 0;
                    let empty = (c.g[V0][0] as i64) <= 0;
                    c.g[S1][0] = c.lw(at(c, S4, FIRST_ITEM));
                    if empty {
                        pc = 0x0035_8970;
                    } else {
                        c.cycles += 3;
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(AT, 20);
                        c.g[S3][0] = u64::MAX;
                        pc = 0x0035_8818;
                    }
                }
                0x0035_8818 => {
                    c.cycles += 44;
                    c.lwc1(0, at(c, S1, 0x6c));
                    c.swc1(0, r(c, SP));
                    c.lwc1(1, at(c, S1, 0x70));
                    c.swc1(1, at(c, SP, 4));
                    c.lwc1(2, at(c, S1, 0x74));
                    c.swc1(20, at(c, SP, 0xc));
                    c.swc1(2, at(c, SP, 8));
                    c.lwc1(0, at(c, S1, 0x78));
                    c.swc1(0, at(c, SP, 0x10));
                    c.lwc1(1, at(c, S1, 0x7c));
                    c.swc1(1, at(c, SP, 0x14));
                    c.lwc1(0, at(c, S1, 0x80));
                    c.swc1(20, at(c, SP, 0x1c));
                    c.swc1(0, at(c, SP, 0x18));
                    call_ptr(c, 0x002d_1be0, 0x0035_8854)?;
                    pc = 0x0035_8854;
                }
                0x0035_8854 => {
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = sx(at(c, SP, 0x20));
                    c.g[V0][0] = c.lw(at(c, S0, 0xa0));
                    c.g[A1][0] = c.g[SP][0];
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(1));
                    c.sw(at(c, S0, 0xa0), c.g[V0][0]);
                    call_ptr(c, 0x0032_8f28, 0x0035_8870)?;
                    pc = 0x0035_8870;
                }
                0x0035_8870 => {
                    c.g[V0][0] = c.lw(at(c, SP, 0x24));
                    c.g[V1][0] = sx(at(c, S0, 0x50));
                    c.g[A0][0] = c.lw(at(c, SP, 0x28));
                    c.g[A1][0] = 2;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[T0][0] = c.lw(at(c, SP, 0x2c));
                    if c.g[V0][0] != 0 { c.g[V1][0] = c.g[S0][0]; }
                    c.g[A0][0] = ((c.g[S3][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    c.g[V0][0] = sx(at(c, V1, 0x28));
                    c.g[T0][0] = ((c.g[S3][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    if c.g[A0][0] != 0 { c.g[V0][0] = c.g[V1][0]; }
                    c.g[A2][0] = c.g[S1][0];
                    c.g[S0][0] = sx(at(c, V0, 0x14));
                    c.g[A3][0] = sx(at(c, SP, 0x20));
                    if c.g[T0][0] != 0 { c.g[S0][0] = c.g[V0][0]; }
                    c.g[A0][0] = c.lw(at(c, S0, 0x10));
                    c.g[T0][0] = c.g[S0][0];
                    call_ptr(c, 0x0032_84b8, 0x0035_88b8)?;
                    pc = 0x0035_88b8;
                }
                0x0035_88b8 => {
                    c.g[V1][0] = c.lw(at(c, S0, 0x10));
                    c.g[V0][0] = c.lw(r(c, V1));
                    if c.g[V0][0] != 0 {
                        c.g[S2][0] = sx(r(c, S2).wrapping_add(1));
                        pc = 0x0035_891c;
                    } else {
                        pc = 0x0035_88c8;
                    }
                }
                // The original unrolls the remaining seven child checks.
                // Each successful early check increments the item counter in
                // its branch delay slot; retain that exact register timing.
                0x0035_88c8 | 0x0035_88d4 | 0x0035_88e0 |
                0x0035_88ec | 0x0035_88f8 => {
                    let offset = match pc {
                        0x0035_88c8 => 4,
                        0x0035_88d4 => 8,
                        0x0035_88e0 => 0xc,
                        0x0035_88ec => 0x10,
                        _ => 0x14,
                    };
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, V1, offset));
                    if c.g[V0][0] != 0 {
                        c.g[S2][0] = sx(r(c, S2).wrapping_add(1));
                        pc = 0x0035_891c;
                    } else {
                        pc += 0xc;
                    }
                }
                0x0035_8904 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, V1, 0x18));
                    let occupied = c.g[V0][0] != 0;
                    c.g[S2][0] = sx(r(c, S2).wrapping_add(1));
                    pc = if occupied { 0x0035_891c } else { 0x0035_8910 };
                }
                0x0035_8910 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, V1, 0x1c));
                    if c.g[V0][0] == 0 {
                        c.cycles += 3;
                        c.g[V1][0] = sx(at(c, V1, 0x20));
                        c.g[A0][0] = 0;
                        pc = 0x0035_8930;
                    } else {
                        pc = 0x0035_891c;
                    }
                }
                0x0035_891c => {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    pc = 0x0035_894c;
                }
                0x0035_8930 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r(c, V1));
                    let occupied = c.g[V0][0] != 0;
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(1));
                    if occupied {
                        if back_edge_unwinds(c, 0x0035_891c) {
                            return Err(crate::call::Unwound);
                        }
                        pc = 0x0035_891c;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[V1][0] = sx(at(c, V1, 4));
                        if more {
                            if back_edge_unwinds(c, 0x0035_8930) {
                                return Err(crate::call::Unwound);
                            }
                            pc = 0x0035_8930;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = 1;
                            pc = 0x0035_894c;
                        }
                    }
                }
                0x0035_894c => {
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = c.lw(at(c, S4, ITEM_COUNT));
                        pc = 0x0035_8964;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = c.lw(at(c, S0, 0x10));
                        call_ptr(c, 0x0031_7e50, 0x0035_895c)?;
                        pc = 0x0035_895c;
                    }
                }
                0x0035_895c => {
                    c.sw(at(c, S0, 0x10), 0);
                    c.g[V0][0] = c.lw(at(c, S4, ITEM_COUNT));
                    pc = 0x0035_8964;
                }
                0x0035_8964 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = c.lw(at(c, S1, NEXT_ITEM));
                    if more {
                        if back_edge_unwinds(c, 0x0035_8818) {
                            return Err(crate::call::Unwound);
                        }
                        pc = 0x0035_8818;
                    } else {
                        pc = 0x0035_8970;
                    }
                }
                0x0035_8970 => {
                    c.cycles += 9;
                    frame.restore(c, S0, 0x80);
                    frame.restore(c, S1, 0x70);
                    frame.restore(c, S2, 0x60);
                    frame.restore(c, S3, 0x50);
                    frame.restore(c, S4, 0x40);
                    frame.restore_ra(c, 0x30);
                    frame.restore_fpr(c, 20, 0x90);
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
