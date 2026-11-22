//! Native ModObj_AddMultiSplineModifier (0x00355b30), retained uninstalled.
//! Generated EE owns registers, cycles and call order. The user disabled
//! automated tests and replays; no build or game/window operation was run.

use crate::call::{call_ptr, guest, Callee, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0035_5b30;
const AFTER_ALLOCATE: u32 = 0x0035_5b60;
const AFTER_CONSTRUCT: u32 = 0x0035_5b70;
const AFTER_INSTALL: u32 = 0x0035_5b7c;
const END: u32 = 0x0035_5b90;
const INSTANCE: u32 = 0x18;
const ALLOCATE: u32 = 0x0031_7d70;
const ALLOCATION_TAG: u32 = 0x0048_e9e8;
const CONSTRUCT: Callee = Callee {
    addr: 0x0035_9f88,
    f: ssx3_ee::s11::f_00359f88,
};
const INSTALL_TRANSFORM: Callee = Callee {
    addr: 0x0035_54b0,
    f: ssx3_ee::s11::f_003554b0,
};

/// `void ModObj_AddMultiSplineModifier(ModObj* node, const void* descriptor)`.
/// Allocate 0x58 bytes with pool flag 0x20000000 and tag 0x48e9e8, construct the
/// native MultiSplineModifier with the descriptor unchanged and the node's
/// Instance pointer at +0x18, then install the returned transform modifier.
///
/// This wrapper reads no descriptor field and converts no source mode, speed,
/// path, count, orientation or distance. The constructor at359f88 owns that
/// interpretation. Allocation failure is not checked in the generated body.
pub fn mod_obj_add_multi_spline_modifier(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x30 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 24;
                    Frame::open(c, 0x30);
                    c.g[A3][0] = 0;
                    frame.save(c, S1, 0x10);
                    c.g[A2][0] = 0x2000_0000;
                    frame.save(c, S0, 0x20);
                    c.g[S1][0] = c.g[A1][0];
                    c.g[S0][0] = c.g[A0][0];
                    c.g[A1][0] = 0x0049_0000;
                    frame.save_ra(c, 0);
                    c.g[A0][0] = 0x58;
                    c.g[A1][0] = sx(ALLOCATION_TAG);
                    call_ptr(c, ALLOCATE, AFTER_ALLOCATE)?;
                    pc = AFTER_ALLOCATE;
                }
                AFTER_ALLOCATE => {
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(INSTANCE));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[V0][0];
                    CONSTRUCT.call(c, AFTER_CONSTRUCT)?;
                    pc = AFTER_CONSTRUCT;
                }
                AFTER_CONSTRUCT => {
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    INSTALL_TRANSFORM.call(c, AFTER_INSTALL)?;
                    pc = AFTER_INSTALL;
                }
                AFTER_INSTALL => {
                    frame.restore(c, S0, 0x20);
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
        }
    })
}
