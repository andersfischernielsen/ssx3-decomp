//! Native WorldObj_RunKillScript (0x0034fcc0), retained uninstalled.
//! No tests, builds, game runs or replays: disabled by the user.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY,
    world_obj_run_kill_script,
    "WorldObj_RunKillScript",
)];

const ENTRY: u32 = 0x0034_fcc0;
const RETURN_FROM_SCRIPT: u32 = 0x0034_fcd0;
const END: u32 = 0x0034_fce0;
const INSTANCE: u32 = 0x18;

/// `int WorldObj_RunKillScript(WorldObj* self)`.
/// Forward the instance at self+0x18 to WScriptMan_TryRunKillScript
/// (0x2d19e8), retaining its V0 result unchanged. The naming record maps
/// that helper to instance event 4; this wrapper does not write an event
/// parameter into A1/A2 or decide whether dispatch is immediate/queued.
pub fn world_obj_run_kill_script(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x10 };
        match c.pc {
            ENTRY => {
                c.cycles += 7;
                Frame::open(c, 0x10);
                frame.save_ra(c, 0);
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(INSTANCE));
                call_ptr(c, 0x002d_19e8, RETURN_FROM_SCRIPT)?;
            }
            RETURN_FROM_SCRIPT => {}
            pc => {
                c.jump_out(pc, ENTRY, END);
                return Ok(());
            }
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
