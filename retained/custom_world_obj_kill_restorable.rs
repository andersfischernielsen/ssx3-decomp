//! Native WorldObj_KillRestorable (0x0034fd00), retained uninstalled.
//!
//! Generated EE is authoritative. No tests, cargo, game or replay were run
//! by this writer, in accordance with the user's instruction.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY,
    world_obj_kill_restorable,
    "WorldObj_KillRestorable",
)];

const ENTRY: u32 = 0x0034_fd00;
const RETURN_FROM_SCRIPT: u32 = 0x0034_fd1c;
const RESTORE_FLAGS: u32 = 0x0034_fd24;
const RETURN_FROM_DESTROY: u32 = 0x0034_fd7c;
const EPILOGUE: u32 = 0x0034_fd80;
const END: u32 = 0x0034_fd90;
const INSTANCE: u32 = 0x18;
const INSTANCE_FLAGS: u32 = 8;

/// `void WorldObj_KillRestorable(WorldObj* self, int run_kill_script)`.
/// With a nonzero second argument, synchronously call RunKillScript0x34fcc0
/// (instance event4, as recorded in docs/investigation/naming.md);
/// a nonzero script result suppresses all default teardown. Otherwise restore
/// the instance flags from their saved high half, preserve low bits0x300,
/// set bit1 and invoke the adjusted destroying destructor with flags3.
/// This body creates neither a DeadNode nor a delayed deletion request.
pub fn world_obj_kill_restorable(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x20 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 5;
                    Frame::open(c, 0x20);
                    frame.save(c, S0, 0x10);
                    frame.save_ra(c, 0);
                    let run_script = c.g[A1][0] != 0;
                    c.g[S0][0] = c.g[A0][0];
                    if run_script {
                        c.cycles += 4;
                        call_ptr(c, 0x0034_fcc0, RETURN_FROM_SCRIPT)?;
                        pc = RETURN_FROM_SCRIPT;
                    } else {
                        pc = RESTORE_FLAGS;
                    }
                }
                RETURN_FROM_SCRIPT => {
                    if c.g[V0][0] != 0 {
                        frame.restore(c, S0, 0x10);
                        pc = EPILOGUE;
                    } else {
                        pc = RESTORE_FLAGS;
                    }
                }
                RESTORE_FLAGS => {
                    c.cycles += 16;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(INSTANCE));
                    c.g[V1][0] = 0xffff_ffff_ffff_0000;
                    c.g[V1][0] |= 0x300;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(INSTANCE_FLAGS));
                    c.g[V0][0] &= c.g[V1][0];
                    c.sw((c.g[A1][0] as u32).wrapping_add(INSTANCE_FLAGS), c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(INSTANCE));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(INSTANCE_FLAGS));
                    c.g[V1][0] = sx(((c.g[V0][0] as u32 as i32) >> 16) as u32);
                    c.g[V0][0] |= c.g[V1][0];
                    c.sw((c.g[A0][0] as u32).wrapping_add(INSTANCE_FLAGS), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(INSTANCE));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(INSTANCE_FLAGS));
                    c.g[V0][0] |= 2;
                    let absent = c.g[S0][0] == 0;
                    c.sw((c.g[V1][0] as u32).wrapping_add(INSTANCE_FLAGS), c.g[V0][0]);
                    if !absent {
                        c.cycles += 6;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        c.g[A1][0] = 3;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(8));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        let destroy = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, destroy, RETURN_FROM_DESTROY)?;
                    }
                    pc = RETURN_FROM_DESTROY;
                }
                RETURN_FROM_DESTROY => {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                    pc = EPILOGUE;
                }
                EPILOGUE => {
                    c.cycles += 3;
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                pc => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
        }
    });
}
