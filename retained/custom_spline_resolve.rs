//! Native WorldSpline segment relocation0x3aa960, retained uninstalled.
//! No tests, builds, replays or game runs: disabled by the user.

use crate::abi;
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(ENTRY, spline_resolve, "cPS2ChunkResolve_v18")];

const ENTRY: u32 = 0x003a_a960;
const LOOP: u32 = 0x003a_a988;
const END: u32 = 0x003a_aa08;
const SEGMENT_COUNT: u32 = 0x20;
const SEGMENT_POINTER: u32 = 0x24;
const SEGMENTS: u32 = 0x30;
const SEGMENT_BYTES: u32 = 0x90;
const PREVIOUS: u32 = 0x60;
const NEXT: u32 = 0x64;
const OWNER: u32 = 0x68;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `int cPS2ChunkResolve_v18(void* unused, WorldSpline* spline)`.
/// Publish the inline 0x90-byte segment array at spline+0x24. For each
/// segment, -1 prev/next becomes null; every other serialized value becomes
/// the adjacent local segment pointer, wrapping around the array. Set its
/// owner to this spline. The serialized non-sentinel indices themselves
/// are not used. Neither the spatial prefix+0..+0xc nor bounds are touched.
pub fn spline_resolve(c: &mut Cpu) {
    let mut has_segments = true;
    match c.pc {
        ENTRY => {
            c.cycles += 7;
            c.g[V0][0] = c.lw(at(c, A1, SEGMENT_COUNT));
            c.g[T5][0] = sx(at(c, A1, SEGMENTS));
            c.g[T1][0] = 1;
            c.sw(at(c, A1, SEGMENT_POINTER), c.g[T5][0]);
            c.g[T0][0] = 0;
            has_segments = (c.g[V0][0] as i64) > 0;
            c.g[A3][0] = sx(r(c, V0).wrapping_sub(1));
            if has_segments {
                c.cycles += 3;
                c.g[T3][0] = SEGMENT_BYTES as u64;
                c.g[T2][0] = u64::MAX;
                c.g[A2][0] = c.g[T5][0];
            }
        }
        LOOP => {}
        pc => {
            c.jump_out(pc, ENTRY, END);
            return;
        }
    }

    if has_segments {
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c, A2, PREVIOUS));
            let no_previous = c.g[V0][0] == c.g[T2][0];
            c.mult(0, V1, A3, T3);
            if no_previous {
                c.cycles += 1;
                c.sw(at(c, A2, PREVIOUS), 0);
            } else {
                c.cycles += 3;
                c.g[V0][0] = sx(r(c, V1).wrapping_add(r(c, T5)));
                c.sw(at(c, A2, PREVIOUS), c.g[V0][0]);
            }

            c.cycles += 6;
            c.g[A0][0] = SEGMENT_BYTES as u64;
            c.mult(0, V0, T0, A0);
            c.g[V1][0] = sx(r(c, V0).wrapping_add(r(c, T5)));
            c.g[V0][0] = c.lw(at(c, V1, NEXT));
            let no_next = c.g[V0][0] == c.g[T2][0];
            c.mult(0, T4, T1, A0);
            if no_next {
                c.cycles += 1;
                c.sw(at(c, V1, NEXT), 0);
            } else {
                c.cycles += 3;
                c.g[V0][0] = sx(r(c, T4).wrapping_add(r(c, T5)));
                c.sw(at(c, V1, NEXT), c.g[V0][0]);
            }

            c.cycles += 13;
            c.g[V0][0] = c.lw(at(c, A1, SEGMENT_COUNT));
            c.g[T1][0] = sx(r(c, T1).wrapping_add(1));
            c.sw(at(c, A2, OWNER), c.g[A1][0]);
            c.g[A3][0] = sx(r(c, A3).wrapping_add(1));
            c.g[A0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[T0][0] = sx(r(c, T0).wrapping_add(1));
            c.g[V1][0] = c.lw(at(c, A1, SEGMENT_COUNT));
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 { c.g[A3][0] = 0; }
            if c.g[A0][0] == 0 { c.g[T1][0] = 0; }
            c.g[V1][0] = ((c.g[T0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            let more = c.g[V1][0] != 0;
            c.g[A2][0] = sx(at(c, A2, SEGMENT_BYTES));
            if !more { break }
            if event_due(c) { deliver_event_here(c, LOOP); }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}
