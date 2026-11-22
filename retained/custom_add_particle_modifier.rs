//! Native ModObj_AddParticleModifier0x355cb0, retained uninstalled.
//!
//! Generated EE is authoritative. No tests, builds or game runs were added
//! or executed, in accordance with the user's instruction.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY,
    mod_obj_add_particle_modifier,
    "ModObj_AddParticleModifier",
)];

const ENTRY: u32 = 0x0035_5cb0;
const AFTER_ALLOCATE: u32 = 0x0035_5ce0;
const AFTER_CONSTRUCT: u32 = 0x0035_5cf0;
const AFTER_LINK: u32 = 0x0035_5cfc;
const END: u32 = 0x0035_5d10;
const INSTANCE: u32 = 0x18;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

/// `void ModObj_AddParticleModifier(ModObj* self, Description* desc)`.
/// Allocate a native0x1f0-byte modifier, construct it using the receiver's
/// instance and link it through ModObj_LinkParticleModifier0x355658. This
/// wrapper reads no descriptor fields and changes no instance flags itself;
/// list membership and rendering policy belong to the called linker.
pub fn mod_obj_add_particle_modifier(c: &mut Cpu) {
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
                    c.g[A0][0] = 0x1f0;
                    c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x1688));
                    call_ptr(c, 0x0031_7d70, AFTER_ALLOCATE)?;
                    pc = AFTER_ALLOCATE;
                }
                AFTER_ALLOCATE => {
                    c.g[A2][0] = c.lw(r(c, S0).wrapping_add(INSTANCE));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0034_58c0, AFTER_CONSTRUCT)?;
                    pc = AFTER_CONSTRUCT;
                }
                AFTER_CONSTRUCT => {
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0035_5658, AFTER_LINK)?;
                    pc = AFTER_LINK;
                }
                AFTER_LINK => {
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
