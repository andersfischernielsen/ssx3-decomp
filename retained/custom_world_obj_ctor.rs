//! Native WorldObj constructor (0x0034fb00), retained uninstalled.
//!
//! Generated EE is the authority. No tests or replays were added or run.

use crate::call::{call_ptr, guest, Callee, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0034_fb00, world_obj_ctor, "WorldObj_ctor")];

const SYNC_PROCESS_CTOR: Callee = Callee {
    addr: 0x0035_4850,
    f: ssx3_ee::s11::f_00354850,
};
const LINK_INSTANCE: Callee = Callee {
    addr: 0x0034_fc78,
    f: ssx3_ee::s11::f_0034fc78,
};
const WORLD_OBJ_VTABLE: u32 = 0x0049_1c80;
const INSTANCE_FLAGS: u32 = 8;
const INSTANCE: u32 = 0x18;
const VTABLE: u32 = 0xc;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `WorldObj* WorldObj_ctor(WorldObj* self, int process_arg, int kind,
/// Instance* instance)`.
///
/// Construct the SyncProcess with the incoming arguments, install the base
/// WorldObj vtable, store Instance* at +0x18 and delegate its linkage to
/// WorldObj_LinkInstance. Unless instance flag 0x100 is set, register the
/// instance with World_AddFocusObject for kinds other than 6, 0x10 and 0x16.
/// This body does not itself write instance flags or a script-manager field;
/// process registration/backlink details remain in the delegated callees.
pub fn world_obj_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0]; // kind
        c.g[S2][0] = c.g[A0][0]; // WorldObj subobject
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A3][0]; // Instance*
        SYNC_PROCESS_CTOR.call(c, 0x0034_fb24)?;

        c.g[V1][0] = 0x0049_0000;
        c.sw(at(c, S2, INSTANCE), c.g[S0][0]);
        c.g[V1][0] = sx(WORLD_OBJ_VTABLE);
        c.g[A0][0] = c.g[S2][0];
        c.sw(at(c, S2, VTABLE), c.g[V1][0]);
        c.g[A1][0] = c.g[S0][0];
        LINK_INSTANCE.call(c, 0x0034_fb40)?;

        c.g[V0][0] = c.lw(at(c, S0, INSTANCE_FLAGS));
        c.g[V0][0] &= 0x100;
        let skip_focus = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S2][0];
        if !skip_focus {
            c.cycles += 3;
            c.g[V0][0] = 6;
            let kind_six = c.g[S1][0] == c.g[V0][0];
            c.g[V0][0] = 0x10;
            let mut common_tail = true;
            if !kind_six {
                c.cycles += 2;
                let kind_sixteen = c.g[S1][0] == c.g[V0][0];
                c.g[V0][0] = 0x16;
                if !kind_sixteen {
                    c.cycles += 2;
                    let kind_twenty_two = c.g[S1][0] == c.g[V0][0];
                    c.g[V0][0] = c.g[S2][0];
                    if kind_twenty_two {
                        // This branch targets 0x34fb78 directly, unlike the
                        // other exclusions which pass the +1-cycle tail.
                        common_tail = false;
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x002d_1bf0, 0x0034_fb74)?;
                    }
                }
            }
            if common_tail {
                c.cycles += 1;
                c.g[V0][0] = c.g[S2][0];
            }
        }

        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
