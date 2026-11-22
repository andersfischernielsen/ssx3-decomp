//! Uninstalled port of the ModObj contact-to-script forwarding wrapper.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

// Kept out of runtime dispatch: automated validation is disabled by the user.
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

/// `ModObj_RunScriptEvent19b8(self, trigger_b, trigger_a, rider_interface)`
/// (0x34fe00): ignore `self` and forward the three contact arguments to
/// `WScriptMan_ObjTriggerHook` (0x2d19b8). That hook supplies the script
/// manager singleton and calls `WScriptMan_OnObjectTrigger` (0x30a060),
/// which dispatches the contacted instance's script kind 2.
pub fn mod_obj_run_script_event19b8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[A0][0] = c.g[A1][0];
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A3][0];
        call_ptr(c, 0x002d_19b8, 0x0034_fe18)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
