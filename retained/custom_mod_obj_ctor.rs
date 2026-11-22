//! Native ModObj constructor (0x00355280), retained uninstalled.
//!
//! Generated EE remains the authority for register/cycle/call order. No
//! automated tests or replays were added or run.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0035_5280, mod_obj_ctor, "ModObj_ctor")];

const VTABLE: u32 = 0x0c;
const FLAGS: u32 = 0x12;
const FIELD_1C: u32 = 0x1c;
const FIELD_20: u32 = 0x20;
const SCALE: u32 = 0x24;
const FIELD_28: u32 = 0x28;

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    (c.g[register][0] as u32).wrapping_add(offset)
}

/// `ModObj* ModObj_ctor(self, process_arg, kind, Instance*)`.
/// Forward all four arguments unchanged to WorldObj_ctor, install the native
/// modifier-object vtable, set flag bit 0 and unit scale, then clear the words
/// at +0x1c, +0x20 and +0x28. This body itself does not clone an
/// instance, register a process, or access current-script context; those
/// responsibilities, if any, belong to its WorldObj_ctor callee.
pub fn mod_obj_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0034_fb00, 0x0035_5294)?;

        c.g[V1][0] = c.lhu(at(c, S0, FLAGS));
        c.g[V0][0] = 0x0049_0000;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1028));
        c.sw(at(c, S0, VTABLE), c.g[V0][0]);
        c.g[V1][0] |= 1;
        c.swc1(0, at(c, S0, SCALE));
        c.g[V0][0] = c.g[S0][0];
        c.sh(at(c, S0, FLAGS), c.g[V1][0]);
        c.sw(at(c, S0, FIELD_1C), 0);
        c.sw(at(c, S0, FIELD_20), 0);
        c.sw(at(c, S0, FIELD_28), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
