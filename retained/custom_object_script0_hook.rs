//! Native object-script-zero forwarding hook (0x002d1a78), uninstalled.
//!
//! No tests or replays were added or run.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x002d_1a78,
    wscript_man_run_obj_script0_hook,
    "WScriptMan_RunObjScript0Hook",
)];

/// `void RunObjScript0Hook(object_or_id)`.
/// Forward the unchanged A0 word as A1 to `WScriptMan_RunObjScript0`,
/// using the global manager at gp+0xce8 as A0. This wrapper performs no
/// lifecycle check and does not identify the event that invoked kind zero.
pub fn wscript_man_run_obj_script0_hook(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[A1][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
        frame.save_ra(c, 0);
        call_ptr(c, 0x0030_a610, 0x002d_1a90)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
