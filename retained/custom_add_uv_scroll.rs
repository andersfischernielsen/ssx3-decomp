//! Native ModObj_AddUVScrollModifier (0x00355b90), retained uninstalled.
//! Generated EE is the authority. No tests, builds or replays were run:
//! automated validation is disabled by the user's instruction.

use crate::call::{call_ptr, guest, Callee, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0035_5b90,
    mod_obj_add_uv_scroll_modifier,
    "ModObj_AddUVScrollModifier",
)];

const ENTRY: u32 = 0x0035_5b90;
const END: u32 = 0x0035_5bec;
const SET_MODIFIER: Callee = Callee {
    addr: 0x0035_5550,
    f: ssx3_ee::s11::f_00355550,
};

/// `void ModObj_AddUVScrollModifier(ModObj* node, const void* desc)`.
/// Allocate a native 0x50-byte UVScrollModifier with pool flag 0x20000000,
/// pass the descriptor unchanged to its constructor, then install the
/// returned modifier on the node. This wrapper supplies no owner argument
/// and performs no source/native rate or mode conversion.
pub fn mod_obj_add_uv_scroll_modifier(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x30 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 23;
                    Frame::open(c, 0x30);
                    c.g[A2][0] = 0x2000_0000;
                    frame.save(c, S0, 0x20);
                    c.g[A3][0] = 0;
                    frame.save(c, S1, 0x10);
                    c.g[S0][0] = c.g[A1][0];
                    c.g[S1][0] = c.g[A0][0];
                    c.g[A1][0] = 0x0049_0000;
                    frame.save_ra(c, 0);
                    c.g[A0][0] = 0x50;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x16c0));
                    call_ptr(c, 0x0031_7d70, 0x0035_5bc0)?;
                    pc = 0x0035_5bc0;
                }
                0x0035_5bc0 => {
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0035_f6e8, 0x0035_5bcc)?;
                    pc = 0x0035_5bcc;
                }
                0x0035_5bcc => {
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[V0][0];
                    SET_MODIFIER.call(c, 0x0035_5bd8)?;
                    pc = 0x0035_5bd8;
                }
                0x0035_5bd8 => {
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
    });
}
