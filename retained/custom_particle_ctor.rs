//! Native Particle_ctor_2 (0x003578a8), retained uninstalled.
//!
//! Generated EE is authoritative. No automated tests, builds or game runs
//! were added or executed, in accordance with the user's instruction.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY,
    particle_ctor,
    "Particle_ctor_2",
)];

const ENTRY: u32 = 0x0035_78a8;
const AFTER_BASE: u32 = 0x0035_78d8;
const AFTER_TAKEOVER: u32 = 0x0035_78f0;
const AFTER_ADD_PARTICLE: u32 = 0x0035_78fc;
const END: u32 = 0x0035_7918;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

/// `Particle* Particle_ctor(self, process_arg, Instance*, Description*, owner)`.
/// Construct a native kind13 ModObj, install the Particle vtable, forward
/// `owner` to ModObj_TakeOverObject and add the particle descriptor through
/// ModObj_AddParticleModifier. No instance flags or descriptor fields are
/// read or changed directly here; those policies belong to the callees.
pub fn particle_ctor(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x40 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 28;
                    Frame::open(c, 0x40);
                    c.g[V0][0] = c.g[A2][0];
                    frame.save(c, S0, 0x30);
                    c.g[A2][0] = 13;
                    frame.save(c, S2, 0x10);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save(c, S1, 0x20);
                    c.g[S2][0] = c.g[A3][0];
                    c.g[S1][0] = c.g[T0][0];
                    frame.save_ra(c, 0);
                    c.g[A3][0] = c.g[V0][0];
                    call_ptr(c, 0x0035_5280, AFTER_BASE)?;
                    pc = AFTER_BASE;
                }
                AFTER_BASE => {
                    c.g[V1][0] = 0x0049_0000;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V1][0] = sx(r(c, V1).wrapping_sub(0x11a0));
                    c.g[A0][0] = c.g[S0][0];
                    c.sw(r(c, S0).wrapping_add(0xc), c.g[V1][0]);
                    call_ptr(c, 0x0035_5f10, AFTER_TAKEOVER)?;
                    pc = AFTER_TAKEOVER;
                }
                AFTER_TAKEOVER => {
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0035_5cb0, AFTER_ADD_PARTICLE)?;
                    pc = AFTER_ADD_PARTICLE;
                }
                AFTER_ADD_PARTICLE => {
                    c.g[V0][0] = c.g[S0][0];
                    frame.restore(c, S1, 0x20);
                    frame.restore(c, S0, 0x30);
                    frame.restore(c, S2, 0x10);
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
