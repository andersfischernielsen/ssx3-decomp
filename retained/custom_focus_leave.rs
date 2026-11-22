//! Native WScriptMan_FocusLeave (0x0030a460), retained uninstalled.
//!
//! No automated tests or replays were added or run.
//! Generated EE is the authority for registers, cycles and call ordering.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0030_a460,
    wscript_man_focus_leave,
    "WScriptMan_FocusLeave",
)];

const INSTANCE_NODE: u32 = 0xc;
const NODE_VTABLE: u32 = 0xc;
const NODE_KIND: u32 = 0x10;
const NODE_INSTANCE: u32 = 0x18;
const DEAD_NODE_KIND: u64 = 6;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void WScriptMan_FocusLeave(WScriptMan* manager, Instance* instance)`.
/// A live node first gets its slot-0xa0 predicate and, when true, its
/// slot-0xb0 callback. Otherwise kind 6 is preserved; another node runs the
/// instance's script 3 when present, or its slot-0x118 cleanup when absent.
/// Without a node, only script 3 is considered. For the native AnimObject
/// vtable 0x490b10, slot 0x118 is WorldObj_Kill (0x34fd90). This function
/// reads node kind, not instance flag bits.
pub fn wscript_man_focus_leave(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw(at(c, S0, INSTANCE_NODE));
        let no_node = c.g[A1][0] == 0;
        c.g[S1][0] = c.g[A0][0];

        // The generated early tails reload S0 without a charged block;
        // its common 0x30a530 tail charges one additional cycle.
        let mut common_restore = true;
        'leave: {
            if !no_node {
                c.cycles += 7;
                c.g[V1][0] = c.lw(at(c, A1, NODE_VTABLE));
                c.g[A0][0] = c.lh(at(c, V1, 0xa0));
                c.g[V0][0] = c.lw(at(c, V1, 0xa4));
                let predicate = r(c, V0);
                c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
                call_ptr(c, predicate, 0x0030_a494)?;
                let predicate_false = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[S0][0];
                if !predicate_false {
                    c.cycles += 8;
                    c.g[A2][0] = c.lw(at(c, S0, INSTANCE_NODE));
                    c.g[V1][0] = c.lw(at(c, A2, NODE_VTABLE));
                    c.g[A0][0] = c.lh(at(c, V1, 0xb0));
                    c.g[V0][0] = c.lw(at(c, V1, 0xb4));
                    let callback = r(c, V0);
                    c.g[A0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
                    call_ptr(c, callback, 0x0030_a4b4)?;
                    common_restore = false;
                    break 'leave;
                }

                c.cycles += 5;
                c.g[A1][0] = c.lw(at(c, S0, INSTANCE_NODE));
                c.g[V1][0] = DEAD_NODE_KIND;
                c.g[V0][0] = c.lh(at(c, A1, NODE_KIND));
                let dead = c.g[V0][0] == c.g[V1][0];
                c.g[A0][0] = c.g[S1][0];
                if dead {
                    break 'leave;
                }

                c.cycles += 5;
                c.g[S0][0] = c.lw(at(c, A1, NODE_INSTANCE));
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0030_a2e8, 0x0030_a4dc)?; // HasObjScript3
                let no_script = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S1][0];
                if !no_script {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0030_a548, 0x0030_a4ec)?; // RunObjScript3
                    common_restore = false;
                    break 'leave;
                }

                c.cycles += 8;
                c.g[A1][0] = c.lw(at(c, S0, INSTANCE_NODE));
                c.g[V0][0] = c.lw(at(c, A1, NODE_VTABLE));
                c.g[A0][0] = c.lh(at(c, V0, 0x118));
                c.g[V1][0] = c.lw(at(c, V0, 0x11c));
                let cleanup = r(c, V1);
                c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
                call_ptr(c, cleanup, 0x0030_a50c)?;
                common_restore = false;
                break 'leave;
            }

            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0030_a2e8, 0x0030_a520)?; // HasObjScript3
            let no_script = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S1][0];
            if !no_script {
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0030_a548, 0x0030_a530)?; // RunObjScript3
            }
        }

        if common_restore {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
