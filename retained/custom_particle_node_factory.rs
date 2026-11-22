//! Native ParticleNode factory0x002fd420, retained uninstalled.
//!
//! Generated EE is authoritative. No tests, cargo, game or replay were
//! added or run, in accordance with the user's instruction.

use crate::call::{call_ptr, guest, Frame};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY, particle_node_factory, "WorldObjFactory_ParticleNode",
)];

const ENTRY: u32 = 0x002f_d420;
const ALLOCATE_DESC: u32 = 0x002f_d578;
const AFTER_ALLOC: u32 = 0x002f_d594;
const AFTER_COPY: u32 = 0x002f_d5b4;
const ARGUMENT_LOOP: u32 = 0x002f_d5d0;
const RESOLVE: u32 = 0x002f_d618;
const AFTER_MISSING_DELETE: u32 = 0x002f_d694;
const AFTER_ATTACH: u32 = 0x002f_d6ac;
const AFTER_ATTACHED_DELETE: u32 = 0x002f_d6c4;
const LIFE_BOUNDS: u32 = 0x002f_d6d0;
const AFTER_NODE_ALLOC: u32 = 0x002f_d6fc;
const FINISH: u32 = 0x002f_d714;
const END: u32 = 0x002f_d758;
const DEFAULTS: u32 = 0x004f_b640;
const PARAM_TYPES: u32 = 0x0044_60b0;
const OWNER_RID: u32 = 0xd8;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `Vec3* ParticleNode(Vec3* out, int count, NamedArg* arguments)`.
/// Allocate/copy a native0xe0-byte descriptor, apply named fields0..54,
/// and resolve its owner RID54 (-1 means the current script instance).
/// With a live owner, ask its current actor to accept modifier kind13.
/// Otherwise clamp descriptor[7] to descriptor[5] and construct a0x2c-byte
/// native Particle actor on that same Instance. There is no Instance clone
/// or explicit detached drawing ownership in this factory.
pub fn particle_node_factory(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x80 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 14;
                    Frame::open(c, 0x80);
                    c.g[V0][0] = c.lw(at(c, GP, 0x2514));
                    frame.save(c, S1, 0x60);
                    frame.save(c, S2, 0x50);
                    c.g[S1][0] = 0x0050_0000;
                    frame.save(c, S4, 0x30);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, S5, 0x20);
                    c.g[S4][0] = c.g[A1][0];
                    frame.save(c, S0, 0x70);
                    c.g[S5][0] = c.g[A2][0];
                    frame.save(c, S3, 0x40);
                    frame.save_ra(c, 0x10);
                    if c.g[V0][0] == 0 {
                        c.cycles += 72;
                        c.g[AT][0] = sx(0xbf80_0000);
                        c.mtc1(AT, 1);
                        c.g[V0][0] = sx(DEFAULTS);
                        c.g[AT][0] = 0x4080_0000;
                        c.mtc1(AT, 2);
                        c.g[A0][0] = 1;
                        c.mtc1(ZERO, 0);
                        c.g[V1][0] = 0xffff_ffff_ffff_0000;
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(AT, 3);
                        c.g[A2][0] = 16;
                        c.swc1(1, at(c, V0, 8));
                        c.g[V1][0] |= 0xffff;
                        c.swc1(2, at(c, V0, 0x10));
                        c.sw(r(c, S1).wrapping_sub(0x49c0), c.g[A0][0]);
                        c.sw(at(c, V0, 4), 0);
                        c.swc1(3, at(c, V0, 12));
                        for offset in (0x14..=0x80).step_by(4) {
                            c.swc1(0, at(c, V0, offset));
                        }
                        c.g[AT][0] = 0x41a0_0000;
                        c.mtc1(AT, 1);
                        c.swc1(3, at(c, V0, 0x84));
                        c.g[A1][0] = c.lbu(at(c, V0, 0xdc));
                        c.swc1(3, at(c, V0, 0xb4));
                        c.sw(at(c, V0, 0xc4), c.g[A2][0]);
                        c.swc1(0, at(c, V0, 0xcc));
                        c.swc1(1, at(c, V0, 0xd4));
                        c.sw(at(c, V0, OWNER_RID), c.g[V1][0]);
                        c.sw(at(c, GP, 0x2514), c.g[A0][0]);
                        for offset in [0x88, 0x8c, 0x90] {
                            c.swc1(0, at(c, V0, offset));
                        }
                        for offset in [0x94, 0x98, 0x9c, 0xa0, 0xa4] {
                            c.swc1(3, at(c, V0, offset));
                        }
                        for offset in [0xa8, 0xac, 0xb0, 0xb8, 0xbc, 0xc0] {
                            c.swc1(0, at(c, V0, offset));
                        }
                        c.sw(at(c, V0, 0xc8), 0);
                        c.sw(at(c, V0, 0xd0), c.g[A0][0]);
                        c.sb(r(c, SP), c.g[A1][0]);
                    }
                    pc = ALLOCATE_DESC;
                }
                ALLOCATE_DESC => {
                    c.cycles += 17;
                    c.g[A1][0] = 0x0049_0000;
                    c.g[A0][0] = 0xe0;
                    c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x68e8));
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    c.g[S3][0] = 0x004d_0000;
                    call_ptr(c, 0x0031_7d70, AFTER_ALLOC)?;
                    pc = AFTER_ALLOC;
                }
                AFTER_ALLOC => {
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A2][0] = 0xe0;
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    c.g[A1][0] = sx(r(c, S1).wrapping_sub(0x49c0));
                    c.g[V0][0] |= 0xffff;
                    c.g[A0][0] = c.g[S0][0];
                    c.sw(at(c, S0, OWNER_RID), c.g[V0][0]);
                    call_ptr(c, 0x003e_6574, AFTER_COPY)?;
                    pc = AFTER_COPY;
                }
                AFTER_COPY => {
                    let no_arguments = (c.g[S4][0] as i64) <= 0;
                    c.g[A2][0] = c.g[S5][0];
                    if no_arguments {
                        pc = RESOLVE;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = 0x0044_0000;
                        c.g[A1][0] = c.g[S4][0];
                        c.g[T0][0] = sx(PARAM_TYPES);
                        c.g[A3][0] = 2;
                        pc = ARGUMENT_LOOP;
                    }
                }
                ARGUMENT_LOOP => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw(r(c, A2));
                    c.g[V1][0] = c.lw(at(c, A2, 12));
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[A0][0] = sx(r(c, V0).wrapping_add(r(c, S0)));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, T0)));
                    c.g[V0][0] = c.lw(r(c, V0));
                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw(at(c, A2, 4));
                        c.cycles += 1;
                        c.sw(r(c, A0), c.g[V0][0]);
                    } else {
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[A3][0] {
                            c.g[V0][0] = c.lw(at(c, A2, 4));
                            c.cycles += 1;
                            c.sw(r(c, A0), c.g[V0][0]);
                        } else {
                            c.cycles += 4;
                            c.lwc1(0, at(c, A2, 4));
                            c.cvt_s_w(0, 0);
                            c.swc1(0, r(c, A0));
                        }
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx(r(c, A1).wrapping_sub(1));
                    let more = c.g[A1][0] != 0;
                    c.g[A2][0] = sx(at(c, A2, 16));
                    if more {
                        if back_edge_unwinds(c, ARGUMENT_LOOP) {
                            return Err(crate::call::Unwound);
                        }
                    } else {
                        pc = RESOLVE;
                    }
                }
                RESOLVE => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw(at(c, S0, OWNER_RID));
                    c.g[V0][0] = !c.g[A2][0];
                    let explicit = c.g[V0][0] != 0;
                    c.g[A1][0] = c.lw(at(c, GP, 0x16c8));
                    if !explicit {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, GP, 0xce8));
                        c.g[S1][0] = c.lw(at(c, V0, 0x290));
                    } else {
                        c.cycles += 8;
                        c.g[V0][0] = c.g[A2][0] & 255;
                        c.g[V0][0] = sx(r(c, V0) << 2);
                        c.g[V1][0] = c.lw(r(c, A1));
                        c.g[A0][0] = c.lw(at(c, V1, 8));
                        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                        c.g[V1][0] = c.lw(r(c, V0));
                        if c.g[V1][0] == 0 {
                            c.cycles += 2;
                            c.g[A2][0] = 0;
                        } else {
                            c.g[A0][0] = c.lw(at(c, V1, 0x1c));
                            c.cycles += 7;
                            c.g[V0][0] = sx(r(c, A2) >> 8);
                            c.g[V0][0] = sx(r(c, V0) << 2);
                            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                            c.g[V1][0] = c.lw(r(c, V0));
                            c.g[V0][0] = sx(r(c, V1) >> 8);
                            let live = c.g[V0][0] != 0;
                            c.g[V0][0] = sx(r(c, V0) << 2);
                            if !live { c.cycles += 1; c.g[V0][0] = 0; }
                            c.cycles += 1;
                            c.g[A2][0] = c.g[V0][0];
                        }
                        c.cycles += 1;
                        c.g[S1][0] = c.g[A2][0];
                    }
                    c.cycles += 2;
                    if c.g[S1][0] == 0 {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0031_7e50, AFTER_MISSING_DELETE)?;
                        pc = FINISH;
                    } else {
                        c.g[A0][0] = c.lw(at(c, S1, 12));
                        c.cycles += 2;
                        let no_node = c.g[A0][0] == 0;
                        c.g[A1][0] = 13;
                        if no_node {
                            c.cycles += 1;
                            c.lwc1(0, at(c, S0, 0x1c));
                            pc = LIFE_BOUNDS;
                        } else {
                            c.cycles += 4;
                            c.g[A2][0] = c.g[S0][0];
                            call_ptr(c, 0x002f_ae38, AFTER_ATTACH)?;
                            pc = AFTER_ATTACH;
                        }
                    }
                }
                AFTER_MISSING_DELETE | AFTER_ATTACHED_DELETE => pc = FINISH,
                AFTER_ATTACH => {
                    if c.g[V0][0] == 0 {
                        c.lwc1(0, at(c, S0, 0x1c));
                        pc = LIFE_BOUNDS;
                    } else {
                        c.cycles += 2;
                        if (c.g[V0][0] as i64) <= 0 {
                            pc = FINISH;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0031_7e50, AFTER_ATTACHED_DELETE)?;
                            pc = FINISH;
                        }
                    }
                }
                LIFE_BOUNDS => {
                    c.cycles += 5;
                    c.lwc1(1, at(c, S0, 0x14));
                    c.c_lt_s(1, 0);
                    if c.fcc() { c.swc1(1, at(c, S0, 0x1c)); }
                    c.cycles += 12;
                    c.g[A1][0] = 0x0049_0000;
                    c.g[A0][0] = 0x2c;
                    c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x68d0));
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    call_ptr(c, 0x0031_7d70, AFTER_NODE_ALLOC)?;
                    pc = AFTER_NODE_ALLOC;
                }
                AFTER_NODE_ALLOC => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 1;
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A3][0] = c.g[S0][0];
                    c.g[T0][0] = 0;
                    call_ptr(c, 0x0035_78a8, FINISH)?;
                    pc = FINISH;
                }
                FINISH => {
                    c.cycles += 17;
                    c.g[A1][0] = sx(r(c, S3).wrapping_sub(0x6f68));
                    c.ldl(V0, at(c, A1, 7));
                    c.ldr(V0, r(c, A1));
                    c.g[V1][0] = c.lw(at(c, A1, 8));
                    c.sdl(V0, at(c, S2, 7));
                    c.sdr(V0, r(c, S2));
                    c.sw(at(c, S2, 8), c.g[V1][0]);
                    c.g[V0][0] = c.g[S2][0];
                    frame.restore(c, S0, 0x70);
                    frame.restore(c, S1, 0x60);
                    frame.restore(c, S2, 0x50);
                    frame.restore(c, S3, 0x40);
                    frame.restore(c, S4, 0x30);
                    frame.restore(c, S5, 0x20);
                    frame.restore_ra(c, 0x10);
                    frame.close(c);
                    return Ok(());
                }
                _ => { c.jump_out(pc, ENTRY, END); return Ok(()); }
            }
        }
    });
}
