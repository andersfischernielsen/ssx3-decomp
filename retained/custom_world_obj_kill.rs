//! Native WorldObj_Kill (0x0034fd90), retained uninstalled.
//!
//! Generated EE is authoritative. No tests, cargo, game or replay were run
//! by this writer, in accordance with the user's instruction.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY,
    world_obj_kill,
    "WorldObj_Kill",
)];

const ENTRY: u32 = 0x0034_fd90;
const RETURN_FROM_DESTROY: u32 = 0x0034_fdf0;
const END: u32 = 0x0034_fe00;
const INSTANCE: u32 = 0x18;
const INSTANCE_FLAGS: u32 = 8;

/// `void WorldObj_Kill(WorldObj* self)`.
/// Restore the instance's low flag half from its saved high half and set
/// bit 1, then synchronously invoke the adjusted virtual destroying
/// destructor (slot +8/+c), with flags 3. This is an immediate native
/// destruction call, not a queued kill or a dead-node replacement.
pub fn world_obj_kill(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x10 };
        match c.pc {
            ENTRY => {
                c.cycles += 18;
                Frame::open(c, 0x10);
                c.g[A2][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.g[A0][0] = 0xffff_ffff_ffff_0000;
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(INSTANCE));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(INSTANCE_FLAGS));
                c.g[V0][0] &= c.g[A0][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(INSTANCE_FLAGS), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(INSTANCE));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(INSTANCE_FLAGS));
                c.g[V1][0] = sx(((c.g[V0][0] as u32 as i32) >> 16) as u32);
                c.g[V0][0] |= c.g[V1][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(INSTANCE_FLAGS), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(INSTANCE));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(INSTANCE_FLAGS));
                c.g[V0][0] |= 2;
                let absent = c.g[A2][0] == 0;
                c.sw((c.g[V1][0] as u32).wrapping_add(INSTANCE_FLAGS), c.g[V0][0]);
                if !absent {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                    c.g[A1][0] = 3;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let destroy = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, destroy, RETURN_FROM_DESTROY)?;
                }
            }
            RETURN_FROM_DESTROY => {}
            pc => {
                c.jump_out(pc, ENTRY, END);
                return Ok(());
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
