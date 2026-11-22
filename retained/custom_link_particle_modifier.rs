//! Native ModObj_LinkParticleModifier0x355658, retained uninstalled.
//!
//! Generated EE is authoritative. No tests, builds or game runs were added
//! or executed, in accordance with the user's instruction.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY,
    mod_obj_link_particle_modifier,
    "ModObj_LinkParticleModifier",
)];

const ENTRY: u32 = 0x0035_5658;
const AFTER_CREATE_BLOCK: u32 = 0x0035_5680;
const AFTER_LINK: u32 = 0x0035_5690;
const END: u32 = 0x0035_56a4;
const MODIFIER_BLOCK: u32 = 0x1c;
const PARTICLE_LIST: u32 = 0x10;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

/// `void ModObj_LinkParticleModifier(ModObj* self, ParticleModifier* effect)`.
/// Lazily create the modifier block, then push `effect` onto the native
/// particle list at block+0x10. The linker makes no instance flag change;
/// drawing and effect lifetime are owned by the native consumers of that list.
pub fn mod_obj_link_particle_modifier(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x30 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 8;
                    Frame::open(c, 0x30);
                    frame.save(c, S0, 0x20);
                    frame.save(c, S1, 0x10);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(MODIFIER_BLOCK));
                    let has_block = c.g[V0][0] != 0;
                    c.g[S1][0] = c.g[A1][0];
                    if !has_block {
                        c.cycles += 2;
                        call_ptr(c, 0x0035_5468, AFTER_CREATE_BLOCK)?;
                    }
                    pc = AFTER_CREATE_BLOCK;
                }
                AFTER_CREATE_BLOCK => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw(r(c, S0).wrapping_add(MODIFIER_BLOCK));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = ssx3_ee_rt::cpu::sx(r(c, A0).wrapping_add(PARTICLE_LIST));
                    call_ptr(c, 0x0034_5700, AFTER_LINK)?;
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
