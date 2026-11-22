//! Native WScriptFn_CreateFlag0x002fc9c8, retained uninstalled.
//!
//! Generated EE is authoritative. Automated validation is disabled by the
//! user; no tests, cargo, replays or game/window control were performed.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x002f_c9c8;
const AFTER_DEFAULT_COPY: u32 = 0x002f_cabc;
const ARGUMENT_LOOP: u32 = 0x002f_cad8;
const RESOLVE_INSTANCE: u32 = 0x002f_cb20;
const CREATE_FLAG: u32 = 0x002f_cb8c;
const AFTER_ATTACH: u32 = 0x002f_cba8;
const ALLOCATE_FLAG: u32 = 0x002f_cbb0;
const AFTER_ALLOCATE: u32 = 0x002f_cbc8;
const FINISH: u32 = 0x002f_cbdc;
const END: u32 = 0x002f_cc20;
const DEFAULTS: u32 = 0x004f_b540;
const DEFAULTS_READY_GP: u32 = 0x2500;
const PARAM_TYPES: u32 = 0x0044_5fc0;
const WORLD_GP: u32 = 0x16c8;
const SCRIPT_MANAGER_GP: u32 = 0x0ce8;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `Vec3* CreateFlag(Vec3* out, int count, NamedArg* args)` (builtin12).
/// Param0 selects an instance RID (-1 = current script owner). Descriptor
/// fields0..3/25 are integers,4..24 floats; field26 starts with an opaque
/// preserved byte. Defaults0..25 are [-1,1,0,0, 1,25,1, 1,0,1, 1,0,1,
/// 1,0,1, 0,0,0,0,0,0,0,0,0,0]. The factory copies0x6c bytes, including
/// that opaque tail, before applying unchecked named indices and coercing
/// integer arguments supplied for float fields. An existing node first gets
/// AttachModifier(kind10,params); any nonzero result consumes the request.
/// Otherwise allocate0x1c and call Flag_ctor34add8(new,1,instance,params).
pub fn create_flag(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xe0 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 13;
                    Frame::open(c, 0xe0);
                    c.g[V0][0] = c.lw(at(c, GP, DEFAULTS_READY_GP));
                    frame.save(c, S4, 0x90);
                    frame.save(c, S0, 0xd0);
                    c.g[S4][0] = c.g[A2][0];
                    frame.save(c, S2, 0xb0);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, S1, 0xc0);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, S3, 0xa0);
                    c.g[A2][0] = 0x0050_0000;
                    frame.save_ra(c, 0x80);
                    if c.g[V0][0] == 0 {
                        c.cycles += 39;
                        c.g[V0][0] = sx(DEFAULTS);
                        c.g[V1][0] = 0xffff_ffff_ffff_0000;
                        c.mtc1(ZERO, 1);
                        c.g[V1][0] |= 0xffff;
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(AT, 0);
                        c.g[A0][0] = 1;
                        c.g[A1][0] = c.lbu(at(c, V0, 0x68));
                        c.g[AT][0] = 0x41c8_0000;
                        c.mtc1(AT, 2);
                        c.sw(r(c, A2).wrapping_sub(0x4ac0), c.g[V1][0]);
                        c.sb(at(c, V0, 0x68), c.g[A1][0]);
                        c.swc1(2, at(c, V0, 0x14));
                        c.swc1(0, at(c, V0, 0x3c));
                        c.swc1(1, at(c, V0, 0x60));
                        c.sw(at(c, GP, DEFAULTS_READY_GP), c.g[A0][0]);
                        c.sw(at(c, V0, 4), c.g[A0][0]);
                        c.sw(at(c, V0, 8), 0);
                        c.sw(at(c, V0, 12), 0);
                        for offset in [0x10, 0x18, 0x1c] { c.swc1(0, at(c, V0, offset)); }
                        c.swc1(1, at(c, V0, 0x20));
                        c.swc1(0, at(c, V0, 0x24));
                        c.swc1(0, at(c, V0, 0x28));
                        c.swc1(1, at(c, V0, 0x2c));
                        c.swc1(0, at(c, V0, 0x30));
                        c.swc1(0, at(c, V0, 0x34));
                        c.swc1(1, at(c, V0, 0x38));
                        for offset in (0x40..=0x5c).step_by(4) { c.swc1(1, at(c, V0, offset)); }
                        c.sw(at(c, V0, 0x64), 0);
                        c.sb(r(c, SP), c.g[A1][0]);
                    }
                    c.cycles += 11;
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    c.g[S1][0] = sx(at(c, SP, 0x10));
                    c.g[V0][0] |= 0xffff;
                    c.g[A1][0] = sx(r(c, A2).wrapping_sub(0x4ac0));
                    c.g[A2][0] = 0x6c;
                    c.sw(at(c, SP, 0x10), c.g[V0][0]);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S3][0] = 0x004d_0000;
                    call_ptr(c, 0x003e_6574, AFTER_DEFAULT_COPY)?;
                    pc = AFTER_DEFAULT_COPY;
                }
                AFTER_DEFAULT_COPY => {
                    let no_arguments = (c.g[S0][0] as i64) <= 0;
                    c.g[A2][0] = c.g[S4][0];
                    if no_arguments {
                        pc = RESOLVE_INSTANCE;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = 0x0044_0000;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[T0][0] = sx(r(c, V0).wrapping_add(0x5fc0));
                        c.g[A3][0] = 2;
                        pc = ARGUMENT_LOOP;
                    }
                }
                ARGUMENT_LOOP => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw(r(c, A2));
                    c.g[V1][0] = c.lw(at(c, A2, 12));
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
                    c.g[A2][0] = sx(at(c, A2, 16));
                    if more {
                        if event_due(c) { deliver_event_here(c, ARGUMENT_LOOP); }
                    } else {
                        pc = RESOLVE_INSTANCE;
                    }
                }
                RESOLVE_INSTANCE => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw(r(c, S1));
                    c.g[V0][0] = !c.g[A2][0];
                    let explicit_rid = c.g[V0][0] != 0;
                    c.g[A1][0] = c.lw(at(c, GP, WORLD_GP));
                    if !explicit_rid {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, GP, SCRIPT_MANAGER_GP));
                        c.g[S0][0] = c.lw(at(c, V0, 0x290));
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
                            let present = c.g[V0][0] != 0;
                            c.g[V0][0] = sx(r(c, V0) << 2);
                            if !present { c.cycles += 1; c.g[V0][0] = 0; }
                            c.cycles += 1;
                            c.g[A2][0] = c.g[V0][0];
                        }
                        c.cycles += 1;
                        c.g[S0][0] = c.g[A2][0];
                    }
                    pc = CREATE_FLAG;
                }
                CREATE_FLAG => {
                    c.cycles += 2;
                    if c.g[S0][0] == 0 {
                        pc = FINISH;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.lw(at(c, S0, 12));
                        let no_node = c.g[A0][0] == 0;
                        c.g[A1][0] = 10;
                        if no_node {
                            pc = ALLOCATE_FLAG;
                        } else {
                            c.cycles += 4;
                            c.g[A2][0] = c.g[S1][0];
                            call_ptr(c, 0x002f_ae38, AFTER_ATTACH)?;
                            pc = AFTER_ATTACH;
                        }
                    }
                }
                AFTER_ATTACH => {
                    pc = if c.g[V0][0] != 0 { FINISH } else { ALLOCATE_FLAG };
                }
                ALLOCATE_FLAG => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x004a_0000;
                    c.g[A0][0] = 0x1c;
                    c.g[A1][0] = sx(r(c, A1).wrapping_add(0x3c08));
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    call_ptr(c, 0x0031_7d70, AFTER_ALLOCATE)?;
                    pc = AFTER_ALLOCATE;
                }
                AFTER_ALLOCATE => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 1;
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A3][0] = c.g[S1][0];
                    call_ptr(c, 0x0034_add8, FINISH)?;
                    pc = FINISH;
                }
                FINISH => {
                    c.cycles += 16;
                    c.g[A1][0] = sx(r(c, S3).wrapping_sub(0x6f68));
                    c.ldl(V0, at(c, A1, 7));
                    c.ldr(V0, r(c, A1));
                    c.g[V1][0] = c.lw(at(c, A1, 8));
                    c.sdl(V0, at(c, S2, 7));
                    c.sdr(V0, r(c, S2));
                    c.sw(at(c, S2, 8), c.g[V1][0]);
                    c.g[V0][0] = c.g[S2][0];
                    frame.restore(c, S0, 0xd0);
                    frame.restore(c, S1, 0xc0);
                    frame.restore(c, S2, 0xb0);
                    frame.restore(c, S3, 0xa0);
                    frame.restore(c, S4, 0x90);
                    frame.restore_ra(c, 0x80);
                    frame.close(c);
                    return Ok(());
                }
                _ => { c.jump_out(pc, ENTRY, END); return Ok(()); }
            }
        }
    });
}
