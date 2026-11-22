//! Native TexFlipModifier_Init (0x0035f218), retained uninstalled.
//! No tests, builds or replays: automated validation is disabled by the user.

use crate::call::{call_ptr, guest, Frame, Unwound};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_f218, tex_flip_modifier_init, "TexFlipModifier_Init"),
];

const OWNER: u32 = 0x18;
const MATERIAL_COUNT: u32 = 0x1c;
const FRAME_COUNT: u32 = 0x20;
const FRAME: u32 = 0x14;
const OUTPUT_HEADER: u32 = 0x24;
const OUTPUT_POINTERS: u32 = 0x28;
const OUTPUT_MATERIALS: u32 = 0x48;
const OWNER_MATERIALS: u32 = 0x94;
const MATERIAL_FRAMES: u32 = 0x10;
const MATERIAL_BYTES: u32 = 0x14;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void TexFlipModifier_Init(TexFlipModifier* self)`.
/// The owner at +0x18 has a material list at +0x94: a count followed by
/// material pointers. A material's optional +0x10 points to a flip table
/// `{u32 count; {u16 texture; u16 opaque;} frames[count]}`. The first such
/// table supplies the sequence length. Output at self+0x24 is another
/// count/pointer list. Animated materials receive private 20-byte records
/// at self+0x48+20*i; other materials keep their original pointer.
///
/// The native allocation accommodates eight material slots. This function
/// deliberately retains the original unchecked count; importers must keep
/// the source owner within that native capacity rather than truncate it.
pub fn tex_flip_modifier_init(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x20 };
        let mut pc = c.pc;
        loop {
            match pc {
                0x0035_f218 => {
                    c.cycles += 11;
                    Frame::open(c, 0x20);
                    c.g[A2][0] = 0;
                    frame.save(c, S0, 0x10);
                    frame.save_ra(c, 0);
                    c.g[S0][0] = c.g[A0][0];
                    c.sw(at(c, S0, FRAME_COUNT), 0);
                    c.g[A0][0] = c.lw(at(c, S0, OWNER));
                    c.g[V0][0] = c.lw(at(c, A0, OWNER_MATERIALS));
                    c.g[V1][0] = c.lw(r(c, V0));
                    let empty = (c.g[V1][0] as i64) <= 0;
                    c.sw(at(c, S0, MATERIAL_COUNT), c.g[V1][0]);
                    pc = if empty { 0x0035_f284 } else { 0x0035_f244 };
                }
                0x0035_f244 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(at(c, A0, OWNER_MATERIALS));
                    pc = 0x0035_f26c;
                }
                0x0035_f24c => {
                    c.cycles += 1;
                    pc = 0x0035_f250;
                }
                0x0035_f250 => {
                    c.cycles += 4;
                    c.g[A2][0] = sx(r(c, A2).wrapping_add(1));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let finished = c.g[V0][0] == 0;
                    c.g[A0][0] = sx(r(c, A2) << 2);
                    pc = if finished { 0x0035_f284 } else { 0x0035_f260 };
                }
                0x0035_f260 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(at(c, S0, OWNER));
                    c.g[V0][0] = c.lw(at(c, V1, OWNER_MATERIALS));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                    pc = 0x0035_f26c;
                }
                0x0035_f26c => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(at(c, V0, 4));
                    c.g[V0][0] = c.lw(at(c, V1, MATERIAL_FRAMES));
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = c.lw(at(c, S0, MATERIAL_COUNT));
                        if back_edge_unwinds(c, 0x0035_f250) { return Err(Unwound); }
                        pc = 0x0035_f250;
                    } else {
                        pc = 0x0035_f27c;
                    }
                }
                0x0035_f27c => {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(r(c, V0));
                    c.sw(at(c, S0, FRAME_COUNT), c.g[V0][0]);
                    pc = 0x0035_f284;
                }
                0x0035_f284 => {
                    c.cycles += 11;
                    c.g[A1][0] = c.lw(at(c, S0, FRAME));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0035_f378, 0x0035_f290)?;
                    pc = 0x0035_f290;
                }
                0x0035_f290 => {
                    // The original 11-cycle block spans both calls.
                    c.g[A2][0] = 0xc4;
                    c.g[A0][0] = sx(at(c, S0, OUTPUT_HEADER));
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x003e_6448, 0x0035_f2a0)?;
                    pc = 0x0035_f2a0;
                }
                0x0035_f2a0 => {
                    c.g[V0][0] = c.lw(at(c, S0, MATERIAL_COUNT));
                    c.g[A2][0] = 0;
                    let empty = (c.g[V0][0] as i64) <= 0;
                    c.sw(at(c, S0, OUTPUT_HEADER), c.g[V0][0]);
                    pc = if empty { 0x0035_f368 } else { 0x0035_f2b0 };
                }
                0x0035_f2b0 => {
                    c.cycles += 4;
                    c.g[T3][0] = sx(at(c, S0, OUTPUT_POINTERS));
                    c.g[T4][0] = MATERIAL_BYTES as u64;
                    c.g[T5][0] = sx(at(c, S0, OUTPUT_MATERIALS));
                    c.g[V0][0] = c.lw(at(c, S0, OWNER));
                    pc = 0x0035_f2c0;
                }
                0x0035_f2c0 => {
                    c.cycles += 7;
                    c.g[A0][0] = sx(r(c, A2) << 2);
                    c.g[V1][0] = c.lw(at(c, V0, OWNER_MATERIALS));
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                    c.g[T0][0] = c.lw(at(c, V1, 4));
                    c.g[V0][0] = c.lw(at(c, T0, MATERIAL_FRAMES));
                    let plain = c.g[V0][0] == 0;
                    c.mult(0, V1, A2, T4);
                    pc = if plain { 0x0035_f348 } else { 0x0035_f2dc };
                }
                0x0035_f2dc => {
                    c.cycles += 11;
                    c.g[A0][0] = sx(r(c, T3).wrapping_add(r(c, A0)));
                    c.g[T1][0] = sx(r(c, A2).wrapping_add(1));
                    c.g[T2][0] = u64::MAX;
                    c.g[A1][0] = sx(at(c, T5, 2));
                    c.g[A3][0] = 2;
                    c.g[V1][0] = sx(at(c, V1, OUTPUT_MATERIALS));
                    c.g[V1][0] = sx(r(c, S0).wrapping_add(r(c, V1)));
                    c.sw(r(c, A0), c.g[V1][0]);
                    c.g[V0][0] = c.lw(at(c, T0, 0xc));
                    c.sw(at(c, V1, 0xc), c.g[V0][0]);
                    pc = 0x0035_f308;
                }
                0x0035_f308 => {
                    c.cycles += 7;
                    c.sh(r(c, A1), c.g[T2][0]);
                    c.g[A3][0] = sx(r(c, A3).wrapping_sub(1));
                    let more = (c.g[A3][0] as i64) >= 0;
                    c.g[A1][0] = sx(at(c, A1, 2));
                    if more {
                        if back_edge_unwinds(c, 0x0035_f308) { return Err(Unwound); }
                        pc = 0x0035_f308;
                    } else {
                        pc = 0x0035_f324;
                    }
                }
                0x0035_f324 => {
                    c.cycles += 9;
                    c.mult(0, V0, A2, T4);
                    c.g[A0][0] = c.lw(at(c, S0, FRAME));
                    c.g[A0][0] = sx(r(c, A0) << 2);
                    c.g[A1][0] = sx(r(c, V0).wrapping_add(r(c, S0)));
                    c.g[V0][0] = c.lw(at(c, T0, MATERIAL_FRAMES));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                    c.g[V1][0] = c.lhu(at(c, V0, 4));
                    c.sh(at(c, A1, OUTPUT_MATERIALS), c.g[V1][0]);
                    pc = 0x0035_f354;
                }
                0x0035_f348 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx(r(c, T3).wrapping_add(r(c, A0)));
                    c.g[T1][0] = sx(r(c, A2).wrapping_add(1));
                    c.sw(r(c, V0), c.g[T0][0]);
                    pc = 0x0035_f354;
                }
                0x0035_f354 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(at(c, S0, MATERIAL_COUNT));
                    c.g[A2][0] = c.g[T1][0];
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = c.lw(at(c, S0, OWNER));
                        if back_edge_unwinds(c, 0x0035_f2c0) { return Err(Unwound); }
                        pc = 0x0035_f2c0;
                    } else {
                        pc = 0x0035_f368;
                    }
                }
                0x0035_f368 => {
                    c.cycles += 4;
                    frame.restore(c, S0, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0035_f218, 0x0035_f378);
                    return Err(Unwound);
                }
            }
        }
    });
}
