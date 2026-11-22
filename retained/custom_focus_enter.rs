//! Native focus-entry object callback (0x0030a3a0), retained uninstalled.
//!
//! No automated tests or replays were added or run.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0030_a3a0,
    wscript_man_focus_enter,
    "WScriptMan_FocusEnter",
)];

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void WScriptMan_FocusEnter(WScriptMan* manager, Instance* instance)`.
/// An existing node handles focus through adjusted virtual slots +0xa0
/// (predicate) and, when permitted, +0xa8 (instance callback). It never
/// falls through to an instance script. Without a node, require the RID's
/// section descriptor to have type 3, then run object script kind 1 if it
/// exists. This body has no instance-flag predicate.
pub fn wscript_man_focus_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw(at(c, S0, 0xc));
        let has_node = c.g[A1][0] != 0;
        c.g[S1][0] = c.g[A0][0];
        if has_node {
            c.cycles += 7;
            c.g[V1][0] = c.lw(at(c, A1, 0xc));
            c.g[A0][0] = c.lh(at(c, V1, 0xa0));
            c.g[V0][0] = c.lw(at(c, V1, 0xa4));
            let predicate = r(c, V0);
            c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
            call_ptr(c, predicate, 0x0030_a3d4)?;
            if c.g[V0][0] != 0 {
                c.cycles += 9;
                c.g[A2][0] = c.lw(at(c, S0, 0xc));
                c.g[A1][0] = c.g[S0][0];
                c.g[V1][0] = c.lw(at(c, A2, 0xc));
                c.g[A0][0] = c.lh(at(c, V1, 0xa8));
                c.g[V0][0] = c.lw(at(c, V1, 0xac));
                let callback = r(c, V0);
                c.g[A0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
                call_ptr(c, callback, 0x0030_a3f8)?;
            }
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 11;
            c.g[A0][0] = c.lw(at(c, S1, 0x28c));
            c.g[A1][0] = 0xc;
            c.g[V1][0] = c.lbu(at(c, S0, 0x78));
            c.g[V0][0] = c.lw(at(c, A0, 4));
            c.mult(0, V1, V1, A1);
            c.g[A0][0] = c.lw(at(c, V0, 4));
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
            c.g[V0][0] = c.lh(r(c, V1));
            c.g[V0][0] ^= 3;
            if c.g[V0][0] == 0 {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0030_a270, 0x0030_a438)?;
                let has_script = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S1][0];
                if has_script {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0030_a298, 0x0030_a448)?;
                }
                c.cycles += 1;
            }
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
