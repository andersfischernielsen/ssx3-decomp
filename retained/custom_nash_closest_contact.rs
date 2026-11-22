//! Native closest-contact selection, retained uninstalled.
//!
//! Reference only, with no override registration or automated test.

use crate::call::{call_ptr, guest, Frame};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0033_6850;
const END: u32 = 0x0033_69d8;
const FRAME_BYTES: u32 = 0x2050;
const QUERY_VTABLE: u32 = 0x50;
const CONTACT_BYTES: u32 = 0x80;
const CONTACT_FRACTION: u32 = 0x40;
const CONTACT_INSTANCE: u32 = 0x50;
const CONTACT_OBJECT: u32 = 0x54;
const INSTANCE_RID: u32 = 0x78;
const OBJECT_ORDER: u32 = 0x150;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `float Nash_FindClosestContact(Nash* world, Volume* query,
/// Contact* output, IgnoreList* ignored)` (0x336850).
///
/// Collect at most64 native contacts and choose the fraction nearest the
/// query's reference fraction (query vslot+18), not necessarily nearest its
/// start. Equal distances prefer an instance contact, then lower instance
/// RID or lower object ordering word. Copy the complete128-byte contact and
/// return its actual fraction; return-1 on no contacts. Filtering is entirely
/// delegated to Nash_CollideVolume335d78 with the original query and ignore
/// list. In particular this selector adds no kind2/surface/platform filter.
/// S0 is query, S3 output, S2 count, S1 chosen contact index.
pub fn nash_find_closest_contact(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: FRAME_BYTES };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 10;
                    Frame::open(c, FRAME_BYTES);
                    c.g[V0][0] = 0x3f;
                    frame.save(c, S0, 0x2040);
                    c.g[V1][0] = u64::MAX;
                    frame.save(c, S3, 0x2010);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, S1, 0x2030);
                    c.g[S3][0] = c.g[A2][0];
                    frame.save(c, S2, 0x2020);
                    frame.save_ra(c, 0x2000);
                    pc = 0x0033_6878;
                }
                0x0033_6878 => {
                    // Native64-iteration setup delay also forwards ignored.
                    c.cycles += 7;
                    c.g[V0][0] = sx(at(c, V0, u32::MAX));
                    let more = c.g[V0][0] != c.g[V1][0];
                    c.g[T0][0] = c.g[A3][0];
                    if more {
                        if back_edge_unwinds(c, 0x0033_6878) {
                            return Err(crate::call::Unwound);
                        }
                    } else {
                        pc = 0x0033_6894;
                    }
                }
                0x0033_6894 => {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = 0x40;
                    call_ptr(c, 0x0033_5d78, 0x0033_68a4)?;
                    pc = 0x0033_68a4;
                }
                0x0033_68a4 => {
                    c.g[S2][0] = c.g[V0][0];
                    if c.g[S2][0] != 0 {
                        c.g[V0][0] = c.lw(at(c, S0, QUERY_VTABLE));
                        pc = 0x0033_68c0;
                    } else {
                        pc = 0x0033_68b0;
                    }
                }
                0x0033_68b0 => {
                    c.cycles += 4;
                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                    c.mtc1(AT, 0);
                    frame.restore(c, S0, 0x2040);
                    pc = 0x0033_69c0;
                }
                0x0033_68c0 => {
                    c.cycles += 13;
                    c.g[S1][0] = 0;
                    c.g[A0][0] = c.lh(at(c, V0, 0x18));
                    c.g[V1][0] = c.lw(at(c, V0, 0x1c));
                    let target = r(c, V1);
                    c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                    call_ptr(c, target, 0x0033_68d4)?;
                    pc = 0x0033_68d4;
                }
                0x0033_68d4 => {
                    // f2 reference, f1 distance of selected contact0.
                    c.mov_s(2, 0);
                    c.g[A3][0] = 1;
                    c.lwc1(0, at(c, SP, CONTACT_FRACTION));
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[S2][0] as i64)) as u64;
                    c.sub_s(0, 0, 2);
                    c.abs_s(1, 0);
                    let only_one = c.g[V0][0] == 0;
                    c.g[A2][0] = sx(at(c, SP, CONTACT_BYTES));
                    pc = if only_one { 0x0033_698c } else { 0x0033_68f4 };
                }
                0x0033_68f4 => {
                    c.cycles += 1;
                    pc = 0x0033_68f8;
                }
                0x0033_68f8 => {
                    c.cycles += 6;
                    c.lwc1(0, at(c, A2, CONTACT_FRACTION));
                    c.sub_s(0, 0, 2);
                    c.abs_s(0, 0);
                    c.c_lt_s(0, 1);
                    pc = if c.fcc() { 0x0033_6910 } else { 0x0033_691c };
                }
                0x0033_6910 => {
                    c.cycles += 3;
                    c.mov_s(1, 0);
                    c.g[S1][0] = c.g[A3][0];
                    pc = 0x0033_697c;
                }
                0x0033_691c => {
                    c.cycles += 4;
                    c.c_eq_s(0, 1);
                    c.g[V0][0] = sx(r(c, S1) << 7);
                    pc = if c.fcc() { 0x0033_692c } else { 0x0033_697c };
                }
                0x0033_692c => {
                    // Tied candidate: V1 addresses the previous winner.
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(at(c, A2, CONTACT_INSTANCE));
                    let no_instance = c.g[A0][0] == 0;
                    c.g[V1][0] = sx(r(c, V0).wrapping_add(r(c, SP)));
                    pc = if no_instance { 0x0033_6958 } else { 0x0033_6938 };
                }
                0x0033_6938 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, V1, CONTACT_INSTANCE));
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.lw(at(c, A0, INSTANCE_RID));
                        pc = 0x0033_694c;
                    } else {
                        pc = 0x0033_6944;
                    }
                }
                0x0033_6944 => {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    pc = 0x0033_6978;
                }
                0x0033_694c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, V0, INSTANCE_RID));
                    c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                    pc = 0x0033_6978;
                }
                0x0033_6958 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, V1, CONTACT_INSTANCE));
                    let winner_has_instance = c.g[V0][0] != 0;
                    c.g[V0][0] = 0;
                    pc = if winner_has_instance { 0x0033_6978 } else { 0x0033_6964 };
                }
                0x0033_6964 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw(at(c, V1, CONTACT_OBJECT));
                    c.g[A0][0] = c.lw(at(c, A2, CONTACT_OBJECT));
                    c.g[A1][0] = c.lw(at(c, V1, OBJECT_ORDER));
                    c.g[V0][0] = c.lw(at(c, A0, OBJECT_ORDER));
                    c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
                    pc = 0x0033_6978;
                }
                0x0033_6978 => {
                    c.cycles += 1;
                    if c.g[V0][0] != 0 { c.g[S1][0] = c.g[A3][0]; }
                    pc = 0x0033_697c;
                }
                0x0033_697c => {
                    c.cycles += 4;
                    c.g[A3][0] = sx(at(c, A3, 1));
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[S2][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A2][0] = sx(at(c, A2, CONTACT_BYTES));
                    if more {
                        if back_edge_unwinds(c, 0x0033_68f8) {
                            return Err(crate::call::Unwound);
                        }
                        pc = 0x0033_68f8;
                    } else {
                        pc = 0x0033_698c;
                    }
                }
                0x0033_698c => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(r(c, S1) << 7);
                    c.g[V1][0] = c.g[S3][0];
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, SP)));
                    c.g[A0][0] = sx(at(c, V0, CONTACT_BYTES));
                    pc = 0x0033_699c;
                }
                0x0033_699c => {
                    // Copy the chosen128-byte contact, preserving scratch GPRs.
                    c.cycles += 7;
                    c.lq(A1, r(c, V0));
                    c.lq(A2, at(c, V0, 0x10));
                    c.sq(A1, r(c, V1));
                    c.sq(A2, at(c, V1, 0x10));
                    c.g[V0][0] = sx(at(c, V0, 0x20));
                    let more = c.g[V0][0] != c.g[A0][0];
                    c.g[V1][0] = sx(at(c, V1, 0x20));
                    if more {
                        if back_edge_unwinds(c, 0x0033_699c) {
                            return Err(crate::call::Unwound);
                        }
                    } else {
                        pc = 0x0033_69b8;
                    }
                }
                0x0033_69b8 => {
                    c.cycles += 2;
                    c.lwc1(0, at(c, S3, CONTACT_FRACTION));
                    frame.restore(c, S0, 0x2040);
                    pc = 0x0033_69c0;
                }
                0x0033_69c0 => {
                    c.cycles += 6;
                    frame.restore(c, S1, 0x2030);
                    frame.restore(c, S2, 0x2020);
                    frame.restore(c, S3, 0x2010);
                    frame.restore_ra(c, 0x2000);
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
