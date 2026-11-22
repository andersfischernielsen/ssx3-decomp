//! Native WScriptFn_AddMultiSplineModifier0x002fe0c0, retained uninstalled.
//!
//! Generated EE is authoritative. The user disabled automated tests, builds,
//! replays and game/window control for this porting writer.

use crate::call::{call_ptr, guest, Frame};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x002f_e0c0;
const COPY_DEFAULTS: u32 = 0x002f_e15c;
const AFTER_COPY: u32 = 0x002f_e180;
const ARGUMENT_LOOP: u32 = 0x002f_e198;
const RESOLVE_INSTANCE: u32 = 0x002f_e1e0;
const RESOLVE_NODE: u32 = 0x002f_e240;
const AFTER_QUERY: u32 = 0x002f_e270;
const AFTER_ADD: u32 = 0x002f_e280;
const FINISH: u32 = 0x002f_e284;
const END: u32 = 0x002f_e2c0;

const DEFAULTS: u32 = 0x004f_b7a8;
const DEFAULTS_READY_GP: u32 = 0x2528;
const PARAMETER_TYPES: u32 = 0x0044_6208;
const WORLD_GP: u32 = 0x16c8;
const SCRIPT_MANAGER_GP: u32 = 0x0ce8;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `Vec3* AddMultiSplineModifier(Vec3* out, int count, NamedArg* arguments)`.
///
/// Copy the0x34-byte default descriptor, then apply unchecked named fields.
/// Types0..12 are [1,1,1,2,2,2,2,2,2,2,1,1,1]. Defaults0..11 are
/// [-1,-1,3,20f,0f,0f,1f,0f,0f,0f,1,2]; the last word retains an opaque
/// byte at+30. Type mismatch only converts integer bits when a float field
/// is expected; all other cases copy the raw word. Param0 is instance RID
/// (-1=current owner), and param1 must differ from-1. An existing node must
/// accept its adjusted virtual slot16(+80/+84), then receive355b30(node,
/// descriptor). This wrapper creates no node and resolves no spline itself.
/// The hidden result always receives the shared native zero Vec3.
pub fn add_multi_spline_modifier(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xa0 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 12;
                    Frame::open(c, frame.size);
                    c.g[V0][0] = c.lw(at(c, GP, DEFAULTS_READY_GP));
                    frame.save(c, S0, 0x90);
                    c.g[T0][0] = 0x0050_0000;
                    frame.save(c, S2, 0x70);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, S3, 0x60);
                    c.g[S2][0] = c.g[A2][0];
                    frame.save(c, S1, 0x80);
                    c.g[S3][0] = c.g[A0][0];
                    let initialized = c.g[V0][0] != 0;
                    frame.save_ra(c, 0x50);
                    if !initialized {
                        c.cycles += 27;
                        c.g[V0][0] = sx(DEFAULTS);
                        c.g[V1][0] = 0xffff_ffff_ffff_0000;
                        c.mtc1(ZERO, 0);
                        c.g[V1][0] |= 0xffff;
                        c.g[A3][0] = c.lbu(at(c, V0, 0x30));
                        c.g[A0][0] = 3;
                        c.g[AT][0] = 0x41a0_0000;
                        c.mtc1(AT, 2);
                        c.g[A1][0] = 2;
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(AT, 1);
                        c.g[A2][0] = 1;
                        c.sw(r(c, T0).wrapping_sub(0x4858), c.g[V1][0]);
                        c.sb(at(c, V0, 0x30), c.g[A3][0]);
                        c.sw(at(c, V0, 4), c.g[V1][0]);
                        c.sw(at(c, V0, 8), c.g[A0][0]);
                        c.swc1(2, at(c, V0, 0xc));
                        c.swc1(1, at(c, V0, 0x18));
                        c.swc1(0, at(c, V0, 0x24));
                        c.sw(at(c, V0, 0x2c), c.g[A1][0]);
                        c.sw(at(c, GP, DEFAULTS_READY_GP), c.g[A2][0]);
                        for offset in [0x10, 0x14, 0x1c, 0x20] {
                            c.swc1(0, at(c, V0, offset));
                        }
                        c.sw(at(c, V0, 0x28), c.g[A2][0]);
                        c.sb(r(c, SP), c.g[A3][0]);
                    }
                    pc = COPY_DEFAULTS;
                }
                COPY_DEFAULTS => {
                    c.cycles += 11;
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    c.g[S1][0] = sx(at(c, SP, 0x10));
                    c.g[V0][0] |= 0xffff;
                    c.g[A2][0] = 0x34;
                    c.g[A1][0] = sx(r(c, T0).wrapping_sub(0x4858));
                    c.sw(at(c, SP, 0x14), c.g[V0][0]);
                    c.g[A0][0] = c.g[S1][0];
                    c.sw(at(c, SP, 0x10), c.g[V0][0]);
                    call_ptr(c, 0x003e_6574, AFTER_COPY)?;
                    pc = AFTER_COPY;
                }
                AFTER_COPY => {
                    let no_arguments = (c.g[S0][0] as i64) <= 0;
                    c.g[A2][0] = c.g[S2][0];
                    if no_arguments {
                        pc = RESOLVE_INSTANCE;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = 0x0044_0000;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[T0][0] = sx(PARAMETER_TYPES);
                        c.g[A3][0] = 2;
                        pc = ARGUMENT_LOOP;
                    }
                }
                ARGUMENT_LOOP => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw(r(c, A2));
                    c.g[V1][0] = c.lw(at(c, A2, 0xc));
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[A0][0] = sx(r(c, V0).wrapping_add(r(c, S1)));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, T0)));
                    c.g[V0][0] = c.lw(r(c, V0));
                    let mut converted = false;
                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw(at(c, A2, 4));
                    } else {
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[A3][0] {
                            c.g[V0][0] = c.lw(at(c, A2, 4));
                        } else {
                            c.cycles += 4;
                            c.lwc1(0, at(c, A2, 4));
                            c.cvt_s_w(0, 0);
                            c.swc1(0, r(c, A0));
                            converted = true;
                        }
                    }
                    if !converted {
                        c.cycles += 1;
                        c.sw(r(c, A0), c.g[V0][0]);
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx(r(c, A1).wrapping_sub(1));
                    let more = c.g[A1][0] != 0;
                    c.g[A2][0] = sx(at(c, A2, 0x10));
                    if more {
                        if back_edge_unwinds(c, ARGUMENT_LOOP) {
                            return Ok(());
                        }
                    } else {
                        pc = RESOLVE_INSTANCE;
                    }
                }
                RESOLVE_INSTANCE => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw(r(c, S1));
                    c.g[V0][0] = !c.g[A2][0];
                    let explicit_owner = c.g[V0][0] != 0;
                    c.g[A1][0] = c.lw(at(c, GP, WORLD_GP));
                    if !explicit_owner {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, GP, SCRIPT_MANAGER_GP));
                        c.g[V1][0] = c.lw(at(c, V0, 0x290));
                    } else {
                        c.cycles += 8;
                        c.g[V0][0] = c.g[A2][0] & 0xff;
                        c.g[V0][0] = sx(r(c, V0) << 2);
                        c.g[V1][0] = c.lw(r(c, A1));
                        c.g[A0][0] = c.lw(at(c, V1, 8));
                        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                        c.g[V1][0] = c.lw(r(c, V0));
                        let no_bank = c.g[V1][0] == 0;
                        c.g[V0][0] = sx(r(c, A2) >> 8);
                        if no_bank {
                            c.cycles += 1;
                            c.g[V0][0] = 0;
                        } else {
                            c.cycles += 7;
                            c.g[A0][0] = c.lw(at(c, V1, 0x1c));
                            c.g[V0][0] = sx(r(c, V0) << 2);
                            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                            c.g[V1][0] = c.lw(r(c, V0));
                            c.g[V0][0] = sx(r(c, V1) >> 8);
                            let present = c.g[V0][0] != 0;
                            c.g[V0][0] = sx(r(c, V0) << 2);
                            if !present {
                                c.cycles += 1;
                                c.g[V0][0] = 0;
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.g[V0][0];
                    }
                    pc = RESOLVE_NODE;
                }
                RESOLVE_NODE => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(at(c, S1, 4));
                    c.g[V0][0] = !c.g[V0][0];
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0x004d_0000;
                        pc = FINISH;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[S0][0] = c.lw(at(c, V1, 0xc));
                    let no_node = c.g[S0][0] == 0;
                    c.g[V1][0] = 0x004d_0000;
                    if no_node {
                        pc = FINISH;
                        continue;
                    }
                    c.cycles += 7;
                    c.g[V1][0] = c.lw(at(c, S0, 0xc));
                    c.g[A0][0] = c.lh(at(c, V1, 0x80));
                    c.g[V0][0] = c.lw(at(c, V1, 0x84));
                    let target = r(c, V0);
                    c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                    call_ptr(c, target, AFTER_QUERY)?;
                    pc = AFTER_QUERY;
                }
                AFTER_QUERY => {
                    let unsupported = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    if unsupported {
                        pc = AFTER_ADD;
                    } else {
                        c.cycles += 2;
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x0035_5b30, AFTER_ADD)?;
                        pc = AFTER_ADD;
                    }
                }
                AFTER_ADD => {
                    c.cycles += 1;
                    c.g[V1][0] = 0x004d_0000;
                    pc = FINISH;
                }
                FINISH => {
                    c.cycles += 15;
                    c.g[V0][0] = c.g[S3][0];
                    c.g[A3][0] = sx(r(c, V1).wrapping_sub(0x6f68));
                    c.ldl(A0, at(c, A3, 7));
                    c.ldr(A0, r(c, A3));
                    c.g[A1][0] = c.lw(at(c, A3, 8));
                    c.sdl(A0, at(c, S3, 7));
                    c.sdr(A0, r(c, S3));
                    c.sw(at(c, S3, 8), c.g[A1][0]);
                    frame.restore(c, S0, 0x90);
                    frame.restore(c, S1, 0x80);
                    frame.restore(c, S2, 0x70);
                    frame.restore(c, S3, 0x60);
                    frame.restore_ra(c, 0x50);
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
