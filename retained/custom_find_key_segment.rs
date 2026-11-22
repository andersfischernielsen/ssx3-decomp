//! Native IntersectInfo_FindKeySegment0x351a80, retained uninstalled.
//!
//! Generated EE is authoritative. No tests, builds, replays or game runs
//! were added or executed, in accordance with the user's instruction.

use crate::abi;
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0035_1a80;
const SCAN: u32 = 0x0035_1af8;
const END: u32 = 0x0035_1b40;
const ENTRY_BYTES: u32 = 24;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

#[inline]
fn finish(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    abi::ret(c);
}

/// `Key* FindKeySegment(Channel* channel, int* cursor, float t)`.
/// Try the key immediately after the cached cursor using [lo,hi). On a
/// miss, scan from zero using only t<hi, stopping at the last key. Thus a
/// time before the first lo returns key0 and a time after the last hi
/// returns the last key. There is no sentinel/default key and no clamp or
/// change to F12: the caller extrapolates the selected absolute-time cubic.
pub fn intersect_info_find_key_segment(c: &mut Cpu) {
    match c.pc {
        ENTRY => {
            c.cycles += 7;
            c.g[V0][0] = c.lw(r(c, A1));
            c.g[A2][0] = sx(r(c, V0).wrapping_add(1));
            c.sw(r(c, A1), c.g[A2][0]);
            c.g[V0][0] = c.lw(r(c, A0));
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let next_outside = c.g[V0][0] == 0;
            c.g[V0][0] = ENTRY_BYTES as u64;
            if next_outside {
                c.cycles += 1;
                c.g[A2][0] = c.lw(at(c, A0, 4));
            } else {
                c.cycles += 8;
                c.g[V1][0] = c.lw(at(c, A0, 4));
                c.mult(0, V0, A2, V0);
                c.g[A2][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
                c.lwc1(0, at(c, A2, 16));
                c.c_le_s(0, 12);
                let before_next = !c.fcc();
                c.g[A3][0] = 0;
                if !before_next {
                    c.cycles += 5;
                    c.lwc1(0, at(c, A2, 20));
                    c.c_lt_s(12, 0);
                    if c.fcc() { c.g[A3][0] = 1; }
                }
                c.cycles += 2;
                if c.g[A3][0] != 0 {
                    finish(c);
                    return;
                }
                c.g[A2][0] = c.lw(at(c, A0, 4));
            }

            c.cycles += 5;
            c.sw(r(c, A1), 0);
            c.g[V0][0] = c.lw(r(c, A0));
            c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
            if (c.g[V0][0] as i64) <= 0 {
                finish(c);
                return;
            }
        }
        SCAN => {}
        pc => {
            c.jump_out(pc, ENTRY, END);
            return;
        }
    }

    loop {
        c.cycles += 5;
        c.lwc1(0, at(c, A2, 20));
        c.c_lt_s(12, 0);
        let below_end = c.fcc();
        c.g[V0][0] = 1;
        if !below_end {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            finish(c);
            return;
        }

        c.cycles += 8;
        c.g[V1][0] = c.lw(r(c, A1));
        c.g[V1][0] = sx(r(c, V1).wrapping_add(1));
        c.sw(r(c, A1), c.g[V1][0]);
        c.g[V0][0] = c.lw(r(c, A0));
        c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V1][0] != 0;
        c.g[A2][0] = sx(at(c, A2, ENTRY_BYTES));
        if !more {
            finish(c);
            return;
        }
        if event_due(c) { deliver_event_here(c, SCAN); }
    }
}
