//! Native MultiSplineModifier_CloneInstances0x0035a458, retained uninstalled.
//!
//! Generated EE is authoritative. Automated tests and replays are disabled by
//! the user; no cargo, build, game launch or window control was performed.

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0035_a458;
const AFTER_COPY_FIRST: u32 = 0x0035_a498;
const COPY_REMAINING: u32 = 0x0035_a4c8;
const AFTER_INSTANCE_MANAGER: u32 = 0x0035_a4d0;
const AFTER_COPY_INSTANCE: u32 = 0x0035_a4ec;
const FINISH: u32 = 0x0035_a530;
const END: u32 = 0x0035_a550;

const COUNT: u32 = 0x04;
const SOURCE_INSTANCE: u32 = 0x40;
const INSTANCES: u32 = 0x44;
const INSTANCE_FLAGS: u32 = 0x08;
const INSTANCE_RID: u32 = 0x78;
const INSTANCE_BYTES: u64 = 0xa0;

const COPY_INSTANCE: Callee = Callee {
    addr: 0x0035_1270,
    f: ssx3_ee::s11::InstancesMan_CopyInstance,
};

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void MultiSplineModifier_CloneInstances(MultiSplineModifier* self)`.
///
/// Copy the complete0xa0-byte source instance into the separately allocated
/// first instance, preserving that first clone's native RID. Remaining
/// instances use InstancesMan_CopyInstance, then clear flag0x40, set0x20
/// and clear0x100. Neither path explicitly forces visibility bits1/2.
/// The first clone is accessed even when count is zero; allocation and count
/// validation belong to the constructor. No instance teardown occurs here.
pub fn multi_spline_modifier_clone_instances(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x60 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 23;
                    Frame::open(c, frame.size);
                    c.g[A2][0] = INSTANCE_BYTES;
                    frame.save(c, S1, 0x40);
                    frame.save(c, S2, 0x30);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, S0, 0x50);
                    c.g[S2][0] = 1;
                    frame.save(c, S3, 0x20);
                    frame.save(c, S4, 0x10);
                    frame.save_ra(c, 0);
                    c.g[V1][0] = c.lw(at(c, S1, INSTANCES));
                    c.g[A1][0] = c.lw(at(c, S1, SOURCE_INSTANCE));
                    c.g[V0][0] = c.lw(r(c, V1));
                    c.g[S0][0] = c.lw(at(c, V0, INSTANCE_RID));
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x003e_6574, AFTER_COPY_FIRST)?;
                    pc = AFTER_COPY_FIRST;
                }
                AFTER_COPY_FIRST => {
                    c.g[V0][0] = c.lw(at(c, S1, INSTANCES));
                    c.g[V1][0] = c.lw(r(c, V0));
                    c.sw(at(c, V1, INSTANCE_RID), c.g[S0][0]);
                    c.g[V0][0] = c.lw(at(c, S1, COUNT));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let no_more = c.g[V0][0] == 0;
                    frame.restore(c, S0, 0x50);
                    if no_more {
                        pc = FINISH;
                    } else {
                        c.cycles += 5;
                        c.g[S4][0] = 0xffff_ffff_ffff_0000;
                        c.g[S3][0] = 0xffff_ffff_ffff_0000;
                        c.g[S4][0] |= 0xffbf;
                        c.g[S3][0] |= 0xfeff;
                        pc = COPY_REMAINING;
                    }
                }
                COPY_REMAINING => {
                    c.cycles += 26;
                    c.g[S0][0] = sx(r(c, S2) << 2);
                    call_ptr(c, 0x002d_1ce0, AFTER_INSTANCE_MANAGER)?;
                    pc = AFTER_INSTANCE_MANAGER;
                }
                AFTER_INSTANCE_MANAGER => {
                    c.g[V1][0] = c.lw(at(c, S1, INSTANCES));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.lw(at(c, S1, SOURCE_INSTANCE));
                    c.g[S2][0] = sx(r(c, S2).wrapping_add(1));
                    c.g[V1][0] = sx(r(c, S0).wrapping_add(r(c, V1)));
                    c.g[A1][0] = c.lw(r(c, V1));
                    COPY_INSTANCE.call(c, AFTER_COPY_INSTANCE)?;
                    pc = AFTER_COPY_INSTANCE;
                }
                AFTER_COPY_INSTANCE => {
                    c.g[V0][0] = c.lw(at(c, S1, INSTANCES));
                    c.g[V0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
                    c.g[A0][0] = c.lw(r(c, V0));
                    c.g[V1][0] = c.lw(at(c, A0, INSTANCE_FLAGS));
                    c.g[V1][0] &= c.g[S4][0];
                    c.g[V1][0] |= 0x20;
                    c.sw(at(c, A0, INSTANCE_FLAGS), c.g[V1][0]);
                    c.g[V0][0] = c.lw(at(c, S1, INSTANCES));
                    c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
                    c.g[A0][0] = c.lw(r(c, S0));
                    c.g[V0][0] = c.lw(at(c, A0, INSTANCE_FLAGS));
                    c.g[V0][0] &= c.g[S3][0];
                    c.sw(at(c, A0, INSTANCE_FLAGS), c.g[V0][0]);
                    c.g[V1][0] = c.lw(at(c, S1, COUNT));
                    c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let more = c.g[V1][0] != 0;
                    frame.restore(c, S0, 0x50);
                    if more {
                        if back_edge_unwinds(c, COPY_REMAINING) {
                            return Ok(());
                        }
                        pc = COPY_REMAINING;
                    } else {
                        pc = FINISH;
                    }
                }
                FINISH => {
                    c.cycles += 7;
                    frame.restore(c, S1, 0x40);
                    frame.restore(c, S2, 0x30);
                    frame.restore(c, S3, 0x20);
                    frame.restore(c, S4, 0x10);
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
