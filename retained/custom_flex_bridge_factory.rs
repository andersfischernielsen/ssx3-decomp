//! Native builtin11 is FlexBridge, not a fence-neighbour command.
//!
//! Retained uninstalled from generated EE. Automated validation is disabled
//! by the user; this file adds no tests or runtime dispatch entries.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x002f_cdc8;
const COPY_DEFAULTS: u32 = 0x002f_ce68;
const AFTER_COPY: u32 = 0x002f_ce8c;
const ARGUMENT_LOOP: u32 = 0x002f_cea8;
const RESOLVE_OWNER: u32 = 0x002f_cef0;
const OWNER_READY: u32 = 0x002f_cf5c;
const AFTER_ATTACH: u32 = 0x002f_cf78;
const ALLOCATE: u32 = 0x002f_cf80;
const AFTER_ALLOCATE: u32 = 0x002f_cf98;
const FINISH: u32 = 0x002f_cfac;
const END: u32 = 0x002f_cfec;
const DEFAULTS: u32 = 0x004f_b5c0;
const PARAM_TYPES: u32 = 0x0044_6038;
const READY_GP: u32 = 0x2508;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 { r(c, register).wrapping_add(offset) }

/// `Vec3* WorldObjFactory_FlexBridge(Vec3* out, int count, NamedArg* args)`.
/// Native schema0..9 is `[float,float,float,float,float,float,float,int,int,0]`.
/// Defaults0..8 are `[500,1,2400,0.1,0.1,10,0,16,-1]`; the final byte is
/// preserved by the guarded static copy constructor. Param8 is the instance
/// RID (-1 selects the script owner). Try modifier kind5 on an existing node;
/// otherwise allocate0xb0 and call FlexBridge constructor346350. The wrapper
/// reads no rider/contact packet and returns the shared native zero Vec3.
pub fn world_obj_factory_flex_bridge(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xa0 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 13;
                    Frame::open(c, 0xa0);
                    c.g[V0][0] = c.lw(at(c, GP, READY_GP));
                    frame.save(c, S0, 0x90);
                    c.g[A3][0] = 0x0050_0000;
                    frame.save(c, S2, 0x70);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, S3, 0x60);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, S1, 0x80);
                    c.g[S3][0] = c.g[A2][0];
                    frame.save(c, S4, 0x50);
                    frame.save_ra(c, 0x40);
                    if c.g[V0][0] == 0 {
                        c.cycles += 27;
                        c.g[V0][0] = sx(DEFAULTS);
                        c.g[AT][0] = 0x43fa_0000;
                        c.mtc1(AT, 3);
                        c.g[A1][0] = c.lbu(at(c, V0, 0x24));
                        c.g[V1][0] = 0xffff_ffff_ffff_0000;
                        c.lwc1(4, r(c, GP).wrapping_sub(0x3390));
                        c.g[A2][0] = 0x10;
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(AT, 2);
                        c.g[V1][0] |= 0xffff;
                        c.g[AT][0] = 0x4516_0000;
                        c.mtc1(AT, 0);
                        c.g[A0][0] = 1;
                        c.g[AT][0] = 0x4120_0000;
                        c.mtc1(AT, 1);
                        c.swc1(3, r(c, A3).wrapping_sub(0x4a40));
                        c.sb(at(c, V0, 0x24), c.g[A1][0]);
                        c.swc1(2, at(c, V0, 4));
                        c.swc1(0, at(c, V0, 8));
                        c.swc1(4, at(c, V0, 0x10));
                        c.swc1(1, at(c, V0, 0x14));
                        c.sw(at(c, V0, 0x1c), c.g[A2][0]);
                        c.sw(at(c, V0, 0x20), c.g[V1][0]);
                        c.sw(at(c, GP, READY_GP), c.g[A0][0]);
                        c.swc1(4, at(c, V0, 0xc));
                        c.sw(at(c, V0, 0x18), 0);
                        c.sb(r(c, SP), c.g[A1][0]);
                    }
                    pc = COPY_DEFAULTS;
                }
                COPY_DEFAULTS => {
                    c.cycles += 11;
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    c.g[S1][0] = sx(at(c, SP, 0x10));
                    c.g[V0][0] |= 0xffff;
                    c.g[A2][0] = 0x28;
                    c.g[A1][0] = sx(r(c, A3).wrapping_sub(0x4a40));
                    c.sw(at(c, SP, 0x30), c.g[V0][0]);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S4][0] = 0x004d_0000;
                    call_ptr(c, 0x003e_6574, AFTER_COPY)?;
                    pc = AFTER_COPY;
                }
                AFTER_COPY => {
                    c.g[A2][0] = c.g[S3][0];
                    if (c.g[S0][0] as i64) <= 0 {
                        pc = RESOLVE_OWNER;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = 0x0044_0000;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[T0][0] = sx(PARAM_TYPES);
                        c.g[A3][0] = 2;
                        pc = ARGUMENT_LOOP;
                    }
                }
                ARGUMENT_LOOP => {
                    loop {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw(r(c, A2));
                        c.g[V1][0] = c.lw(at(c, A2, 0xc));
                        c.g[V0][0] = sx(r(c, V0) << 2);
                        c.g[A0][0] = sx(r(c, V0).wrapping_add(r(c, S1)));
                        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, T0)));
                        c.g[V0][0] = c.lw(r(c, V0));
                        let mut convert = false;
                        if c.g[V1][0] != c.g[V0][0] {
                            c.cycles += 2;
                            convert = c.g[V0][0] == c.g[A3][0];
                        }
                        if convert {
                            c.cycles += 4;
                            c.lwc1(0, at(c, A2, 4));
                            c.cvt_s_w(0, 0);
                            c.swc1(0, r(c, A0));
                        } else {
                            c.g[V0][0] = c.lw(at(c, A2, 4));
                            c.cycles += 1;
                            c.sw(r(c, A0), c.g[V0][0]);
                        }
                        c.cycles += 3;
                        c.g[A1][0] = sx(r(c, A1).wrapping_sub(1));
                        let more = c.g[A1][0] != 0;
                        c.g[A2][0] = sx(at(c, A2, 0x10));
                        if !more { break }
                        if event_due(c) { deliver_event_here(c, ARGUMENT_LOOP); }
                    }
                    pc = RESOLVE_OWNER;
                }
                RESOLVE_OWNER => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw(at(c, S1, 0x20));
                    c.g[V0][0] = !c.g[A2][0];
                    let explicit = c.g[V0][0] != 0;
                    c.g[A1][0] = c.lw(at(c, GP, 0x16c8));
                    if !explicit {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, GP, 0xce8));
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
                    pc = OWNER_READY;
                }
                OWNER_READY => {
                    c.cycles += 2;
                    if c.g[S0][0] == 0 { pc = FINISH; continue }
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(at(c, S0, 0xc));
                    let absent = c.g[A0][0] == 0;
                    c.g[A1][0] = 5;
                    if absent {
                        pc = ALLOCATE;
                    } else {
                        c.cycles += 4;
                        c.g[A2][0] = c.g[S1][0];
                        call_ptr(c, 0x002f_ae38, AFTER_ATTACH)?;
                        pc = AFTER_ATTACH;
                    }
                }
                AFTER_ATTACH => { pc = if c.g[V0][0] != 0 { FINISH } else { ALLOCATE }; }
                ALLOCATE => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x0049_0000;
                    c.g[A0][0] = 0xb0;
                    c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x6908));
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
                    call_ptr(c, 0x0034_6350, FINISH)?;
                    pc = FINISH;
                }
                FINISH => {
                    c.cycles += 16;
                    c.g[A1][0] = sx(r(c, S4).wrapping_sub(0x6f68));
                    c.ldl(V0, at(c, A1, 7));
                    c.ldr(V0, r(c, A1));
                    c.g[V1][0] = c.lw(at(c, A1, 8));
                    c.sdl(V0, at(c, S2, 7));
                    c.sdr(V0, r(c, S2));
                    c.sw(at(c, S2, 8), c.g[V1][0]);
                    c.g[V0][0] = c.g[S2][0];
                    frame.restore(c, S0, 0x90);
                    frame.restore(c, S1, 0x80);
                    frame.restore(c, S2, 0x70);
                    frame.restore(c, S3, 0x60);
                    frame.restore(c, S4, 0x50);
                    frame.restore_ra(c, 0x40);
                    frame.close(c);
                    return Ok(());
                }
                _ => { c.jump_out(pc, ENTRY, END); return Ok(()); }
            }
        }
    })
}
