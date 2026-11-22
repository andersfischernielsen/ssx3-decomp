//! Native RollerModifier_v10 (0x361cd8), retained uninstalled.
//!
//! Generated EE controls register, memory, cycle and call order. No test,
//! build, replay or game execution accompanies this translation.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0036_1cd8;
const RETURN_FROM_INIT: u32 = 0x0036_1cec;
const END: u32 = 0x0036_1cf8;
const TIMER: u32 = 0x20;
const INIT_BODY: u32 = 0x0035_dde8;

/// `void* RollerModifier_v10(RollerModifier* self, ...)`.
/// Clear the native lifetime/activity timer at +0x20, then forward every
/// argument unchanged to RollerModifier_InitBody. Preserve the native stack
/// and return continuation when that callee suspends execution.
pub fn roller_modifier_v10(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x10 };
        match c.pc {
            ENTRY => {
                c.cycles += 8;
                Frame::open(c, 0x10);
                c.g[V0][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.g[RA][0] = sx(RETURN_FROM_INIT);
                c.sw((c.g[V0][0] as u32).wrapping_add(TIMER), 0);
                call_ptr(c, INIT_BODY, RETURN_FROM_INIT)?;
            }
            RETURN_FROM_INIT => {}
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
