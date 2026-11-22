//! Native WScriptFn_PlayObjectSound (0x002fff00), retained uninstalled.
//!
//! Generated EE is authoritative. No tests, build, replay or game execution
//! were added or run: the user has disabled automated validation.

use crate::call::{call_ptr, guest, Frame};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x002f_ff00;
const END: u32 = 0x0030_00a4;
const FRAME_BYTES: u32 = 0x70;
const DEFAULTS_INITIALIZED_GP: u32 = 0x2574;
const ARGUMENT_TYPES_GP: u32 = 0xb38;
const WORLD_GP: u32 = 0x16c8;
const SCRIPT_MANAGER_GP: u32 = 0xce8;
const CURRENT_INSTANCE: u32 = 0x290;
const GET_AUDIO: u32 = 0x0028_b180;
const PLAY_OBJECT_SOUND: u32 = 0x0029_74a0;
const COPY_BYTES: u32 = 0x003e_6574;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `Vec3 WScriptFn_PlayObjectSound(int count, NamedArg* args)`.
/// A0 is the result destination, A1 the count and A2 a 16-byte named-argument
/// array. Useful indices are 0:instance RID (default -1 means the script's
/// current instance) and 1:sound ID (default -1). The native type table is at
/// gp+b38; supplied values are converted only when an expected float differs
/// from the descriptor type. Resolved instances are passed to the native
/// audio system, and the global Vec3 at 4c9098 is copied to the result.
///
/// The frame, every native call continuation, argument-loop event head and
/// original block label remain resumable. The retained port is not installed.
pub fn play_object_sound(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: FRAME_BYTES };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 12;
                    Frame::open(c, FRAME_BYTES);
                    c.g[V0][0] = c.lw(at(c, GP, DEFAULTS_INITIALIZED_GP));
                    frame.save(c, S2, 0x40);
                    frame.save(c, S1, 0x50);
                    c.g[S2][0] = c.g[A2][0];
                    frame.save(c, S3, 0x30);
                    c.g[S1][0] = c.g[A1][0];
                    frame.save(c, S0, 0x60);
                    c.g[S3][0] = c.g[A0][0];
                    frame.save_ra(c, 0x20);
                    let initialized = c.g[V0][0] != 0;
                    c.g[A2][0] = 0x0050_0000;
                    pc = if initialized { 0x002f_ff58 } else { 0x002f_ff30 };
                }
                0x002f_ff30 => {
                    c.cycles += 10;
                    c.g[V1][0] = sx(r(c, A2).wrapping_sub(0x44f8));
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    c.g[A1][0] = c.lbu(at(c, V1, 8));
                    c.g[V0][0] |= 0xffff;
                    c.sw(r(c, A2).wrapping_sub(0x44f8), c.g[V0][0]);
                    c.g[A0][0] = 1;
                    c.sb(at(c, V1, 8), c.g[A1][0]);
                    c.sw(at(c, V1, 4), c.g[V0][0]);
                    c.sw(at(c, GP, DEFAULTS_INITIALIZED_GP), c.g[A0][0]);
                    c.sb(r(c, SP), c.g[A1][0]);
                    pc = 0x002f_ff58;
                }
                0x002f_ff58 => {
                    c.cycles += 10;
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    c.g[S0][0] = sx(at(c, SP, 0x10));
                    c.g[V0][0] |= 0xffff;
                    c.g[A1][0] = sx(r(c, A2).wrapping_sub(0x44f8));
                    c.g[A2][0] = 12;
                    c.sw(at(c, SP, 0x10), c.g[V0][0]);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, COPY_BYTES, 0x002f_ff78)?;
                    pc = 0x002f_ff78;
                }
                0x002f_ff78 => {
                    let no_arguments = (c.g[S1][0] as i64) <= 0;
                    c.g[A2][0] = c.g[S2][0];
                    pc = if no_arguments { 0x002f_ffd8 } else { 0x002f_ff80 };
                }
                0x002f_ff80 => {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[T0][0] = sx(at(c, GP, ARGUMENT_TYPES_GP));
                    c.g[A3][0] = 2;
                    pc = 0x002f_ff90;
                }
                0x002f_ff90 => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw(r(c, A2));
                    c.g[V1][0] = c.lw(at(c, A2, 0xc));
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[A0][0] = sx(r(c, V0).wrapping_add(r(c, S0)));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, T0)));
                    c.g[V0][0] = c.lw(r(c, V0));
                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw(at(c, A2, 4));
                        pc = 0x002f_ffc8;
                    } else {
                        pc = 0x002f_ffb0;
                    }
                }
                0x002f_ffb0 => {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = c.lw(at(c, A2, 4));
                        pc = 0x002f_ffc8;
                    } else {
                        pc = 0x002f_ffb8;
                    }
                }
                0x002f_ffb8 => {
                    c.cycles += 4;
                    c.lwc1(0, at(c, A2, 4));
                    c.cvt_s_w(0, 0);
                    c.swc1(0, r(c, A0));
                    pc = 0x002f_ffcc;
                }
                0x002f_ffc8 => {
                    c.cycles += 1;
                    c.sw(r(c, A0), c.g[V0][0]);
                    pc = 0x002f_ffcc;
                }
                0x002f_ffcc => {
                    c.cycles += 3;
                    c.g[A1][0] = sx(r(c, A1).wrapping_sub(1));
                    let more = c.g[A1][0] != 0;
                    c.g[A2][0] = sx(at(c, A2, 0x10));
                    if more {
                        if back_edge_unwinds(c, 0x002f_ff90) { return Ok(()) }
                        pc = 0x002f_ff90;
                    } else {
                        pc = 0x002f_ffd8;
                    }
                }
                0x002f_ffd8 => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw(r(c, S0));
                    c.g[V0][0] = !c.g[A2][0];
                    let explicit_instance = c.g[V0][0] != 0;
                    c.g[A1][0] = c.lw(at(c, GP, WORLD_GP));
                    pc = if explicit_instance { 0x002f_fff4 } else { 0x002f_ffe8 };
                }
                0x002f_ffe8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, GP, SCRIPT_MANAGER_GP));
                    c.g[S1][0] = c.lw(at(c, V0, CURRENT_INSTANCE));
                    pc = 0x0030_0044;
                }
                0x002f_fff4 => {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[A2][0] & 0xff;
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[V1][0] = c.lw(r(c, A1));
                    c.g[A0][0] = c.lw(at(c, V1, 8));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                    c.g[V1][0] = c.lw(r(c, V0));
                    if c.g[V1][0] != 0 {
                        c.g[A0][0] = c.lw(at(c, V1, 0x1c));
                        pc = 0x0030_001c;
                    } else {
                        pc = 0x0030_0014;
                    }
                }
                0x0030_0014 => {
                    c.cycles += 2;
                    c.g[A1][0] = 0;
                    pc = 0x0030_0040;
                }
                0x0030_001c => {
                    c.cycles += 7;
                    c.g[V0][0] = sx(r(c, A2) >> 8);
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                    c.g[V1][0] = c.lw(r(c, V0));
                    c.g[V0][0] = sx(r(c, V1) >> 8);
                    let resident = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    pc = if resident { 0x0030_003c } else { 0x0030_0038 };
                }
                0x0030_0038 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    pc = 0x0030_003c;
                }
                0x0030_003c => {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[V0][0];
                    pc = 0x0030_0040;
                }
                0x0030_0040 => {
                    c.cycles += 1;
                    c.g[S1][0] = c.g[A1][0];
                    pc = 0x0030_0044;
                }
                0x0030_0044 => {
                    c.cycles += 2;
                    let absent = c.g[S1][0] == 0;
                    c.g[V1][0] = 0x004d_0000;
                    pc = if absent { 0x0030_0068 } else { 0x0030_004c };
                }
                0x0030_004c => {
                    c.cycles += 7;
                    c.g[S0][0] = c.lw(at(c, S0, 4));
                    call_ptr(c, GET_AUDIO, 0x0030_0054)?;
                    pc = 0x0030_0054;
                }
                0x0030_0054 => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, PLAY_OBJECT_SOUND, 0x0030_0064)?;
                    pc = 0x0030_0064;
                }
                0x0030_0064 => {
                    c.g[V1][0] = 0x004d_0000;
                    pc = 0x0030_0068;
                }
                0x0030_0068 => {
                    c.cycles += 15;
                    c.g[V0][0] = c.g[S3][0];
                    c.g[A3][0] = sx(r(c, V1).wrapping_sub(0x6f68));
                    c.ldl(A0, at(c, A3, 7));
                    c.ldr(A0, r(c, A3));
                    c.g[A1][0] = c.lw(at(c, A3, 8));
                    c.sdl(A0, at(c, S3, 7));
                    c.sdr(A0, r(c, S3));
                    c.sw(at(c, S3, 8), c.g[A1][0]);
                    frame.restore(c, S0, 0x60);
                    frame.restore(c, S1, 0x50);
                    frame.restore(c, S2, 0x40);
                    frame.restore(c, S3, 0x30);
                    frame.restore_ra(c, 0x20);
                    frame.close(c);
                    return Ok(());
                }
                pc => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
        }
    });
}
