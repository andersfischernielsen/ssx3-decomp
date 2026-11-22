//! Native ParticleModifier constructor (0x003458c0), retained uninstalled.
//!
//! Generated EE is authoritative. No tests, builds or game runs were added
//! or executed, in accordance with the user's instruction.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY,
    particle_modifier_ctor,
    "ParticleModifier_ctor",
)];

const ENTRY: u32 = 0x0034_58c0;
const AFTER_BASE: u32 = 0x0034_58e0;
const AFTER_PARTICLE_CTOR: u32 = 0x0034_58f4;
const AFTER_NODE_MATRIX: u32 = 0x0034_591c;
const COPY_INSTANCE_MATRIX: u32 = 0x0034_5948;
const INITIALIZE_PARTICLES: u32 = 0x0034_596c;
const AFTER_INITIALIZE: u32 = 0x0034_597c;
const END: u32 = 0x0034_59a8;

const INSTANCE: u32 = 0x0c;
const INSTANCE_NODE: u32 = 0x0c;
const INSTANCE_MATRIX: u32 = 0x10;
const ROOT_MATRIX: u32 = 0x10;
const PARTICLES: u32 = 0x50;
const DESCRIPTION: u32 = 0x1e0;
const STATE: u32 = 0x1e4;
const LIVE_COUNT_GP: u32 = 0x0f08;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `ParticleModifier* ParticleModifier_ctor(self, Description*, Instance*)`.
/// Keep the descriptor at +0x1e0 and initialize the particle engine embedded
/// at +0x50. Its initial matrix is the owner's current node matrix (virtual
/// +0xc0/+0xc4), or its static instance matrix when no node is installed.
/// The descriptor is forwarded intact to 0x3705e0; this wrapper interprets
/// none of its colour, sprite, lifetime or launch fields. The last word is
/// initialized to zero and the native live-modifier count is incremented.
pub fn particle_modifier_ctor(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x30 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 18;
                    Frame::open(c, 0x30);
                    frame.save(c, S0, 0x20);
                    frame.save(c, S1, 0x10);
                    c.g[S0][0] = c.g[A1][0];
                    c.g[S1][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[A1][0] = c.g[A2][0];
                    call_ptr(c, 0x0034_5798, AFTER_BASE)?;
                    pc = AFTER_BASE;
                }
                AFTER_BASE => {
                    c.g[V1][0] = 0x0049_0000;
                    c.g[A0][0] = sx(at(c, S1, PARTICLES));
                    c.g[V1][0] = sx(at(c, V1, 0x12b0));
                    c.sw(at(c, S1, 8), c.g[V1][0]);
                    call_ptr(c, 0x0037_0018, AFTER_PARTICLE_CTOR)?;
                    pc = AFTER_PARTICLE_CTOR;
                }
                AFTER_PARTICLE_CTOR => {
                    c.g[V1][0] = c.lw(at(c, S1, INSTANCE));
                    c.sw(at(c, S1, DESCRIPTION), c.g[S0][0]);
                    c.g[A1][0] = c.lw(at(c, V1, INSTANCE_NODE));
                    let has_node = c.g[A1][0] != 0;
                    c.g[V1][0] = sx(at(c, V1, INSTANCE_MATRIX));
                    if has_node {
                        c.cycles += 16;
                        c.g[V1][0] = c.lw(at(c, A1, 0xc));
                        c.g[A0][0] = c.lh(at(c, V1, 0xc0));
                        c.g[V0][0] = c.lw(at(c, V1, 0xc4));
                        let target = r(c, V0);
                        c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
                        call_ptr(c, target, AFTER_NODE_MATRIX)?;
                        pc = AFTER_NODE_MATRIX;
                    } else {
                        pc = COPY_INSTANCE_MATRIX;
                    }
                }
                AFTER_NODE_MATRIX => {
                    c.g[V1][0] = sx(at(c, S1, ROOT_MATRIX));
                    c.lqc2(1, r(c, V0));
                    c.lqc2(2, at(c, V0, 0x10));
                    c.lqc2(3, at(c, V0, 0x20));
                    c.lqc2(4, at(c, V0, 0x30));
                    c.sqc2(1, r(c, V1));
                    c.sqc2(2, at(c, V1, 0x10));
                    c.sqc2(3, at(c, V1, 0x20));
                    c.sqc2(4, at(c, V1, 0x30));
                    c.g[V0][0] = c.g[V1][0];
                    pc = INITIALIZE_PARTICLES;
                }
                COPY_INSTANCE_MATRIX => {
                    c.cycles += 9;
                    c.g[V0][0] = sx(at(c, S1, ROOT_MATRIX));
                    c.lqc2(1, r(c, V1));
                    c.lqc2(2, at(c, V1, 0x10));
                    c.lqc2(3, at(c, V1, 0x20));
                    c.lqc2(4, at(c, V1, 0x30));
                    c.sqc2(1, r(c, V0));
                    c.sqc2(2, at(c, V0, 0x10));
                    c.sqc2(3, at(c, V0, 0x20));
                    c.sqc2(4, at(c, V0, 0x30));
                    pc = INITIALIZE_PARTICLES;
                }
                INITIALIZE_PARTICLES => {
                    c.cycles += 14;
                    c.g[A2][0] = c.lw(at(c, S1, DESCRIPTION));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = sx(at(c, S1, PARTICLES));
                    call_ptr(c, 0x0037_05e0, AFTER_INITIALIZE)?;
                    pc = AFTER_INITIALIZE;
                }
                AFTER_INITIALIZE => {
                    c.g[V1][0] = c.lw(at(c, GP, LIVE_COUNT_GP));
                    c.g[V0][0] = c.g[S1][0];
                    c.sw(at(c, S1, STATE), 0);
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(1));
                    frame.restore(c, S0, 0x20);
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    c.sw(at(c, GP, LIVE_COUNT_GP), c.g[V1][0]);
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
