//! Native port of `cFlagManager_GroupRemoveFlag` (0x0034b168).
//!
//! This function removes a group member; it does not construct or deform
//! geometry. The port remains uninstalled while automated validation is disabled.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0034_b168;
const SCAN: u32 = 0x0034_b1a8;
const REMOVED: u32 = 0x0034_b1fc;
const RETURN: u32 = 0x0034_b218;
const END: u32 = 0x0034_b228;
const FREE_MESH: u32 = 0x0034_b7b8;

// Descriptor fields observed by this function. Members are compared by their
// RID, not their pointer. The model identity is the first word of INSTANCE_MODEL.
const GROUP_MODEL_ID: u32 = 0x58;
const GROUP_MEMBER_COUNT: u32 = 0x5c;
const GROUP_MEMBERS: u32 = 0x94;
const INSTANCE_RID: u32 = 0x78;
const INSTANCE_MODEL: u32 = 0x80;

#[inline]
fn field(c: &Cpu, reg: usize, offset: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(offset)
}

/// Remove the member identified by `a1` from the group in `a0`.
///
/// Removal swaps the last member into the vacated slot, clears that final slot,
/// and frees the group's mesh when its count becomes zero. Returns one when a
/// member was removed, zero for an empty group, incompatible model, or absent RID.
pub fn flag_manager_group_remove_flag(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x10 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 6;
                    Frame::open(c, 0x10);
                    c.g[A3][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[A2][0] = c.lw(field(c, A3, GROUP_MEMBER_COUNT));
                    let empty = c.g[A2][0] == 0;
                    c.g[V0][0] = 0;
                    if empty {
                        pc = RETURN;
                        continue;
                    }

                    c.cycles += 5;
                    c.g[V0][0] = c.lw(field(c, A1, INSTANCE_MODEL));
                    c.g[A0][0] = c.lw(field(c, A3, GROUP_MODEL_ID));
                    c.g[V1][0] = c.lw(field(c, V0, 0));
                    let incompatible = c.g[V1][0] != c.g[A0][0];
                    c.g[V0][0] = 0;
                    if incompatible {
                        pc = RETURN;
                        continue;
                    }

                    c.cycles += 2;
                    let nonpositive = (c.g[A2][0] as i64) <= 0;
                    c.g[T0][0] = 0;
                    if nonpositive {
                        pc = RETURN;
                        continue;
                    }

                    c.cycles += 3;
                    c.g[A2][0] = sx(field(c, A3, GROUP_MEMBERS));
                    c.g[T1][0] = c.g[A2][0];
                    pc = SCAN;
                }
                SCAN => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(field(c, A2, 0));
                    c.g[A0][0] = c.lw(field(c, A1, INSTANCE_RID));
                    c.g[V1][0] = c.lw(field(c, V0, INSTANCE_RID));
                    let different = c.g[V1][0] != c.g[A0][0];
                    c.g[V0][0] = c.lw(field(c, A3, GROUP_MEMBER_COUNT));
                    if different {
                        c.cycles += 4;
                        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                        c.g[V0][0] =
                            ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, SCAN);
                            }
                            pc = SCAN;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = 0;
                            pc = RETURN;
                        }
                        continue;
                    }

                    c.cycles += 14;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx(
                        (c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32),
                    );
                    c.g[V1][0] = c.lw(field(c, V0, 0));
                    c.sw(field(c, A2, 0), c.g[V1][0]);
                    c.g[V0][0] = c.lw(field(c, A3, GROUP_MEMBER_COUNT));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx(
                        (c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32),
                    );
                    c.sw(field(c, V0, 0), 0);
                    c.g[V1][0] = c.lw(field(c, A3, GROUP_MEMBER_COUNT));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                    let remains = c.g[V1][0] != 0;
                    c.sw(field(c, A3, GROUP_MEMBER_COUNT), c.g[V1][0]);
                    if !remains {
                        c.cycles += 2;
                        c.g[RA][0] = sx(REMOVED);
                        c.g[A0][0] = c.g[A3][0];
                        call_ptr(c, FREE_MESH, REMOVED)?;
                    }
                    pc = REMOVED;
                }
                REMOVED => {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    pc = RETURN;
                }
                RETURN => {
                    c.cycles += 3;
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
