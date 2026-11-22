//! Native Fence_PropagateToNeighbour34cc80, retained uninstalled.
//!
//! Generated EE owns registers, cycles, branches and call order. No tests,
//! cargo, replay or game/window operations were added or run.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0034_cc80;
const QUERY_NEIGHBOUR: u32 = 0x0034_cce0;
const AFTER_TYPE_QUERY: u32 = 0x0034_cd00;
const AFTER_LEFT_KICK: u32 = 0x0034_cd50;
const RIGHT_NEIGHBOUR: u32 = 0x0034_cd58;
const AFTER_RIGHT_KICK: u32 = 0x0034_cd74;
const CREATE_NEIGHBOUR: u32 = 0x0034_cd7c;
const AFTER_ALLOCATE: u32 = 0x0034_cdac;
const RESTORE_S0: u32 = 0x0034_cdc8;
const RESTORE_S1: u32 = 0x0034_cdcc;
const RETURN: u32 = 0x0034_cdd0;
const END: u32 = 0x0034_cde8;

const INSTANCE_NODE: u32 = 0x0c;
const DESCRIPTOR: u32 = 0x1c;
const AMPLITUDE: u32 = 0x30;
const VELOCITY: u32 = 0x34;
const RENDER_HANDLE: u32 = 0x40;
const LEFT_INSTANCE: u32 = 0x104;
const RIGHT_INSTANCE: u32 = 0x108;

fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void Fence_PropagateToNeighbour(Fence* self, Instance* neighbour, int side)`.
/// A live render handle permits a signed fraction of the current sway to
/// reach a neighbour. Existing nodes must be Fence type9; qualifying force
/// then kicks the selected stored neighbour (+104/+108). An instance with
/// no node receives a newly allocated0x10c-byte Fence, using the source
/// fence's inline16-byte descriptor at+1c and the computed F12 impulse.
pub fn fence_propagate_to_neighbour(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x50 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 11;
                    Frame::open(c, 0x50);
                    frame.save(c, S0, 0x30);
                    frame.save(c, S1, 0x20);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save(c, S2, 0x10);
                    c.g[S1][0] = c.g[A1][0];
                    frame.save_ra(c, 0);
                    frame.save_fpr(c, 20, 0x40);
                    c.g[V0][0] = c.lw(at(c, S0, RENDER_HANDLE));
                    let drawable = c.g[V0][0] != 0;
                    c.g[S2][0] = c.g[A2][0];
                    if !drawable {
                        pc = RESTORE_S0;
                        continue;
                    }
                    c.cycles += 2;
                    if c.g[S1][0] == 0 {
                        frame.restore(c, S0, 0x30);
                        pc = RESTORE_S1;
                        continue;
                    }
                    c.cycles += 6;
                    c.lwc1(1, at(c, S0, VELOCITY));
                    c.mtc1(ZERO, 0);
                    c.c_lt_s(1, 0);
                    let negative = c.fcc();
                    c.lwc1(1, at(c, S0, AMPLITUDE));
                    if negative {
                        c.cycles += 3;
                        c.lwc1(0, r(c, GP).wrapping_sub(0x2ad0));
                        c.g[A2][0] = c.lw(at(c, S1, INSTANCE_NODE));
                    } else {
                        c.cycles += 2;
                        c.lwc1(0, r(c, GP).wrapping_sub(0x2acc));
                        c.g[A2][0] = c.lw(at(c, S1, INSTANCE_NODE));
                    }
                    pc = QUERY_NEIGHBOUR;
                }
                QUERY_NEIGHBOUR => {
                    c.cycles += 2;
                    let unowned = c.g[A2][0] == 0;
                    c.mul_s(20, 1, 0);
                    if unowned {
                        pc = CREATE_NEIGHBOUR;
                        continue;
                    }
                    c.cycles += 8;
                    c.g[V1][0] = c.lw(at(c, A2, 0xc));
                    c.g[A1][0] = 9;
                    c.g[A0][0] = c.lh(at(c, V1, 0x50));
                    c.g[V0][0] = c.lw(at(c, V1, 0x54));
                    let target = r(c, V0);
                    c.g[A0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
                    call_ptr(c, target, AFTER_TYPE_QUERY)?;
                    pc = AFTER_TYPE_QUERY;
                }
                AFTER_TYPE_QUERY => {
                    if c.g[V0][0] == 0 {
                        frame.restore(c, S0, 0x30);
                        pc = RESTORE_S1;
                        continue;
                    }
                    c.cycles += 7;
                    c.g[AT][0] = 0x3e80_0000;
                    c.mtc1(AT, 1);
                    c.abs_s(0, 20);
                    c.c_le_s(1, 0);
                    if !c.fcc() {
                        frame.restore(c, S0, 0x30);
                        pc = RESTORE_S1;
                        continue;
                    }
                    c.cycles += 2;
                    if c.g[S2][0] == 0 {
                        c.g[A0][0] = c.lw(at(c, S0, RIGHT_INSTANCE));
                        pc = RIGHT_NEIGHBOUR;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(at(c, S0, LEFT_INSTANCE));
                    let missing = c.g[A0][0] == 0;
                    frame.restore(c, S0, 0x30);
                    if missing {
                        pc = RESTORE_S1;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(at(c, A0, INSTANCE_NODE));
                    let missing = c.g[A0][0] == 0;
                    frame.restore(c, S1, 0x20);
                    if missing {
                        pc = RETURN;
                        continue;
                    }
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S2][0];
                    c.mov_s(12, 20);
                    call_ptr(c, 0x0034_cde8, AFTER_LEFT_KICK)?;
                    pc = AFTER_LEFT_KICK;
                }
                AFTER_LEFT_KICK | AFTER_RIGHT_KICK => {
                    frame.restore(c, S0, 0x30);
                    pc = RESTORE_S1;
                }
                RIGHT_NEIGHBOUR => {
                    c.cycles += 2;
                    let missing = c.g[A0][0] == 0;
                    frame.restore(c, S0, 0x30);
                    if missing {
                        pc = RESTORE_S1;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(at(c, A0, INSTANCE_NODE));
                    let missing = c.g[A0][0] == 0;
                    c.mov_s(12, 20);
                    if missing {
                        pc = RESTORE_S1;
                        continue;
                    }
                    c.cycles += 4;
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0034_cde8, AFTER_RIGHT_KICK)?;
                    pc = AFTER_RIGHT_KICK;
                }
                CREATE_NEIGHBOUR => {
                    c.cycles += 7;
                    c.g[AT][0] = 0x3e80_0000;
                    c.mtc1(AT, 1);
                    c.abs_s(0, 20);
                    c.c_le_s(1, 0);
                    let too_small = !c.fcc();
                    c.g[A0][0] = 0x10c;
                    if too_small {
                        pc = RESTORE_S0;
                        continue;
                    }
                    c.cycles += 12;
                    c.g[A1][0] = 0x004a_0000;
                    c.g[A1][0] = sx(r(c, A1).wrapping_add(0x4018));
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    call_ptr(c, 0x0031_7d70, AFTER_ALLOCATE)?;
                    pc = AFTER_ALLOCATE;
                }
                AFTER_ALLOCATE => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A3][0] = sx(at(c, S0, DESCRIPTOR));
                    c.g[T0][0] = c.g[S2][0];
                    c.mov_s(12, 20);
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x0034_cab8, RESTORE_S0)?;
                    pc = RESTORE_S0;
                }
                RESTORE_S0 => {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x30);
                    pc = RESTORE_S1;
                }
                RESTORE_S1 => {
                    c.cycles += 1;
                    frame.restore(c, S1, 0x20);
                    pc = RETURN;
                }
                RETURN => {
                    c.cycles += 5;
                    frame.restore(c, S2, 0x10);
                    frame.restore_ra(c, 0);
                    frame.restore_fpr(c, 20, 0x40);
                    frame.close(c);
                    return Ok(());
                }
                _ => { c.jump_out(pc, ENTRY, END); return Ok(()); }
            }
        }
    })
}
