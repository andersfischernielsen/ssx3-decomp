//! Native ModObj_SetTransformModifier3554b0, retained uninstalled.
//! The user disabled automated tests and replays; no build or game/window
//! operation was performed for this translation.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0035_54b0;
const AFTER_PREPARE: u32 = 0x0035_54dc;
const AFTER_CREATE_BLOCK: u32 = 0x0035_54f0;
const AFTER_FREE_TRANSFORM: u32 = 0x0035_5500;
const INSTALL: u32 = 0x0035_5504;
const AFTER_FINISH: u32 = 0x0035_551c;
const END: u32 = 0x0035_5550;

const VTABLE: u32 = 0x0c;
const INSTANCE: u32 = 0x18;
const MODIFIER_BLOCK: u32 = 0x1c;
const BLOCK_TRANSFORM: u32 = 0;
const PREPARE_SLOT: u32 = 0x188;
const FINISH_SLOT: u32 = 0x180;
const CREATE_BLOCK: u32 = 0x0035_5468;
const FREE_TRANSFORM: u32 = 0x0035_30d0;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn field(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void ModObj_SetTransformModifier(ModObj* self, TransformModifier* value)`.
/// The transform pointer lives at offset0 of the modifier block referenced
/// by self+1c. Prepare the object through virtual slot49, create that block
/// if missing or free its former transform otherwise, install the new raw
/// pointer, then notify through virtual slot48. Finally change the owning
/// instance's flags to `(flags & ~0x20) | 0x40`. The function returns those
/// resulting flags in V0, rather than a modifier pointer or success boolean.
pub fn mod_obj_set_transform_modifier(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x30 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 14;
                    Frame::open(c, frame.size);
                    frame.save(c, S0, 0x20);
                    frame.save(c, S1, 0x10);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[S1][0] = c.g[A1][0];
                    c.g[V0][0] = c.lw(field(c, S0, VTABLE));
                    c.g[A0][0] = c.lh(field(c, V0, PREPARE_SLOT));
                    c.g[V1][0] = c.lw(field(c, V0, PREPARE_SLOT + 4));
                    let target = r(c, V1);
                    c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                    call_ptr(c, target, AFTER_PREPARE)?;
                    pc = AFTER_PREPARE;
                }
                AFTER_PREPARE => {
                    c.g[A0][0] = c.lw(field(c, S0, MODIFIER_BLOCK));
                    if c.g[A0][0] != 0 {
                        c.cycles += 3;
                        call_ptr(c, FREE_TRANSFORM, AFTER_FREE_TRANSFORM)?;
                        pc = AFTER_FREE_TRANSFORM;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, CREATE_BLOCK, AFTER_CREATE_BLOCK)?;
                        pc = AFTER_CREATE_BLOCK;
                    }
                }
                AFTER_CREATE_BLOCK | AFTER_FREE_TRANSFORM => {
                    c.g[V0][0] = c.lw(field(c, S0, MODIFIER_BLOCK));
                    pc = INSTALL;
                }
                INSTALL => {
                    c.cycles += 18;
                    c.sw(field(c, V0, BLOCK_TRANSFORM), c.g[S1][0]);
                    c.g[V1][0] = c.lw(field(c, S0, VTABLE));
                    c.g[A0][0] = c.lh(field(c, V1, FINISH_SLOT));
                    c.g[V0][0] = c.lw(field(c, V1, FINISH_SLOT + 4));
                    let target = r(c, V0);
                    c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                    call_ptr(c, target, AFTER_FINISH)?;
                    pc = AFTER_FINISH;
                }
                AFTER_FINISH => {
                    c.g[A0][0] = c.lw(field(c, S0, INSTANCE));
                    c.g[V1][0] = 0xffff_ffff_ffff_0000;
                    c.g[V1][0] |= 0xffdf;
                    frame.restore(c, S0, 0x20);
                    c.g[V0][0] = c.lw(field(c, A0, 8));
                    frame.restore(c, S1, 0x10);
                    c.g[V0][0] &= c.g[V1][0];
                    frame.restore_ra(c, 0);
                    c.g[V0][0] |= 0x40;
                    c.sw(field(c, A0, 8), c.g[V0][0]);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
        }
    })
}
