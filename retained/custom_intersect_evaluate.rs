//! Native IntersectInfo_Evaluate0x351538, retained uninstalled.
//!
//! Generated EE is authoritative. No tests, builds, replays or game runs
//! were added or executed, in accordance with the user's instruction.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0035_1538;
const CHANNEL: u32 = 0x0035_1588;
const KEY_RETURN: u32 = 0x0035_1600;
const NEXT_CHANNEL: u32 = 0x0035_162c;
const END: u32 = 0x0035_1660;
const SOURCE: u32 = 0x40;
const VALUES: u32 = 0x44;
const CHANNEL_MASK: u32 = 0x18;
const CHANNEL_TABLE: u32 = 0x20;
const SEGMENT_BYTES: u64 = 0x18;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// Apply the selected key's absolute-time cubic in native FPU order.
/// The range search does not substitute a local time or an endpoint value.
fn evaluate_key(c: &mut Cpu) {
    c.lwc1(0, r(c, V0));
    c.cycles += 10;
    c.lwc1(1, at(c, V0, 4));
    c.mul_s(0, 0, 20);
    c.lwc1(2, at(c, V0, 8));
    c.lwc1(3, at(c, V0, 12));
    c.add_s(0, 0, 1);
    c.mul_s(0, 0, 20);
    c.add_s(0, 0, 2);
    c.mul_s(0, 0, 20);
    c.add_s(0, 0, 3);
    c.swc1(0, r(c, S2));
}

/// `void IntersectInfo_Evaluate(IntersectInfo* self, float time)`.
/// Visit sixteen output lanes. For each enabled source-mask bit, use its
/// compact cached key index when lo<=time<hi; otherwise call the native
/// key finder. Evaluate a*t^3+b*t^2+c*t+d at the original time and replace
/// that lane. Disabled lanes keep their previous/base values. The wrapper
/// itself never clamps time and never restores the base value for a channel
/// whose key range starts later than the supplied time.
pub fn intersect_info_evaluate(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x90 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 20;
                    Frame::open(c, 0x90);
                    frame.save(c, S0, 0x70);
                    frame.save(c, S2, 0x50);
                    frame.save(c, S3, 0x40);
                    frame.save(c, S4, 0x30);
                    c.g[S3][0] = c.g[A0][0];
                    frame.save(c, S5, 0x20);
                    c.g[S0][0] = c.g[S3][0];
                    frame.save(c, S6, 0x10);
                    c.g[S5][0] = 1;
                    frame.save_fpr(c, 20, 0x80);
                    c.g[S2][0] = sx(at(c, S0, VALUES));
                    frame.save(c, S1, 0x60);
                    c.mov_s(20, 12);
                    frame.save_ra(c, 0);
                    c.g[S6][0] = SEGMENT_BYTES;
                    c.g[S4][0] = 15;
                    c.g[V0][0] = c.lw(at(c, S3, SOURCE));
                    c.g[S1][0] = c.lw(at(c, V0, CHANNEL_TABLE));
                    pc = CHANNEL;
                }
                CHANNEL => {
                    c.cycles += 8;
                    c.g[V1][0] = c.lw(at(c, S3, SOURCE));
                    c.g[S4][0] = sx(r(c, S4).wrapping_sub(1));
                    c.g[A1][0] = c.lw(r(c, S1));
                    c.g[A2][0] = 0;
                    c.g[V0][0] = c.lw(at(c, V1, CHANNEL_MASK));
                    c.g[V0][0] &= c.g[S5][0];
                    let disabled = c.g[V0][0] == 0;
                    c.g[S5][0] = sx(r(c, S5) << 1);
                    if disabled {
                        pc = NEXT_CHANNEL;
                        continue;
                    }

                    c.cycles += 13;
                    c.g[V0][0] = c.lw(r(c, S0));
                    c.g[A0][0] = c.g[A1][0];
                    c.g[V1][0] = c.lw(at(c, A1, 4));
                    c.mov_s(12, 20);
                    c.mult(0, V0, V0, S6);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[S1][0] = sx(at(c, S1, 4));
                    c.g[V0][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
                    c.lwc1(0, at(c, V0, 0x10));
                    c.c_le_s(0, 20);
                    let before = !c.fcc();
                    c.g[S0][0] = sx(at(c, S0, 4));
                    if !before {
                        c.cycles += 5;
                        c.lwc1(0, at(c, V0, 0x14));
                        c.c_lt_s(20, 0);
                        if c.fcc() { c.g[A2][0] = 1; }
                    }
                    c.cycles += 2;
                    if c.g[A2][0] == 0 {
                        c.cycles += 3;
                        call_ptr(c, 0x0035_1a80, KEY_RETURN)?;
                    }
                    pc = KEY_RETURN;
                }
                KEY_RETURN => {
                    evaluate_key(c);
                    pc = NEXT_CHANNEL;
                }
                NEXT_CHANNEL => {
                    c.cycles += 2;
                    let more = (c.g[S4][0] as i64) >= 0;
                    c.g[S2][0] = sx(at(c, S2, 4));
                    if more {
                        if event_due(c) { deliver_event_here(c, CHANNEL); }
                        pc = CHANNEL;
                    } else {
                        c.cycles += 11;
                        frame.restore(c, S0, 0x70);
                        frame.restore(c, S1, 0x60);
                        frame.restore(c, S2, 0x50);
                        frame.restore(c, S3, 0x40);
                        frame.restore(c, S4, 0x30);
                        frame.restore(c, S5, 0x20);
                        frame.restore(c, S6, 0x10);
                        frame.restore_ra(c, 0);
                        frame.restore_fpr(c, 20, 0x80);
                        frame.close(c);
                        return Ok(());
                    }
                }
                _ => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
        }
    })
}
