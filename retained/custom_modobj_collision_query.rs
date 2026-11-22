//! Native ModObj collision-delegation query (0x00356a28), retained uninstalled.
//!
//! Generated EE is authoritative. No automated tests, builds, replays or
//! collision-policy changes were made by this writer, as requested.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(ENTRY, mod_obj_collision_query, "ModObj_v38")];

const ENTRY: u32 = 0x0035_6a28;
const RETURN_FROM_NODE: u32 = 0x0035_6a5c;
const END: u32 = 0x0035_6a70;
const MODIFIER_BLOCK: u32 = 0x1c;
const PRIMARY_NODE: u32 = 0;

/// `int ModObj_v38(ModObj* self)`.
/// Return zero without a modifier block or its primary node. Otherwise call
/// the primary node's virtual +0xa0/+0xa4 query and return its result unchanged.
/// This is ModObj's +0x130/+0x134 virtual used by Collide_VolumeObject to ask
/// whether the actor supplies its own collision query. UV/TexFlip slots in the
/// modifier block are independent of this primary-node slot.
pub fn mod_obj_collision_query(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x10 };
        match c.pc {
            ENTRY => {
                c.cycles += 5;
                Frame::open(c, 0x10);
                frame.save_ra(c, 0);
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(MODIFIER_BLOCK));
                let no_block = c.g[V1][0] == 0;
                c.g[V0][0] = 0;
                if no_block {
                    c.cycles += 1;
                    frame.restore_ra(c, 0);
                } else {
                    c.cycles += 3;
                    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(PRIMARY_NODE));
                    let no_node = c.g[A1][0] == 0;
                    frame.restore_ra(c, 0);
                    if !no_node {
                        c.cycles += 7;
                        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                        let target = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(RETURN_FROM_NODE);
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, RETURN_FROM_NODE)?;
                        frame.restore_ra(c, 0);
                    }
                }
            }
            RETURN_FROM_NODE => frame.restore_ra(c, 0),
            pc => {
                c.jump_out(pc, ENTRY, END);
                return Ok(());
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}
