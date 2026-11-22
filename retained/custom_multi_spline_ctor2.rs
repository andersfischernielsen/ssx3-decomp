//! Native MultiSplineModifier_ctor_2 (0x0035a118), retained uninstalled.
//!
//! This is the stream/replay constructor, not the parameter-block constructor.
//! Generated EE owns registers, cycles and calls. The user's test ban applies;
//! no automated tests, cargo, replay or game/window operation was performed.

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0035_a118;
const AFTER_PARAMETERS: u32 = 0x0035_a184;
const AFTER_PATH: u32 = 0x0035_a190;
const AFTER_OWNER: u32 = 0x0035_a1a4;
const AFTER_ARRAYS: u32 = 0x0035_a1b0;
const INSTANCE_LOOP: u32 = 0x0035_a1c0;
const AFTER_HANDLE: u32 = 0x0035_a1d8;
const AFTER_MANAGER: u32 = 0x0035_a1e0;
const AFTER_INSTANCE: u32 = 0x0035_a1ec;
const CLONE_INSTANCES: u32 = 0x0035_a210;
const AFTER_CLONE: u32 = 0x0035_a218;
const AFTER_INSERT: u32 = 0x0035_a220;
const END: u32 = 0x0035_a250;

const VTABLE: u32 = 0x0048_f168;
const COUNT: u32 = 0x04;
const DIRTY: u32 = 0x30;
const OWNER: u32 = 0x40;
const INSTANCES: u32 = 0x44;
const FOLLOW: u32 = 0x48;

const LOAD_PATH: Callee = Callee {
    addr: 0x0034_5570,
    f: ssx3_ee::s11::SplineModifier_LoadPath,
};
const ALLOC_ARRAYS: Callee = Callee {
    addr: 0x0035_a328,
    f: ssx3_ee::s11::MultiSplineModifier_AllocArrays,
};
const ALLOC_BY_HANDLE: Callee = Callee {
    addr: 0x0035_11d0,
    f: ssx3_ee::s11::InstancesMan_AllocByHandle,
};
const CLONE: Callee = Callee {
    addr: 0x0035_a458,
    f: ssx3_ee::s11::MultiSplineModifier_CloneInstances,
};
const INSERT: Callee = Callee {
    addr: 0x0035_a5f8,
    f: ssx3_ee::s11::MultiSplineModifier_InsertNodeInstances,
};

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `MultiSplineModifier* ctor_2(MultiSplineModifier* self, Stream* stream)`.
/// Read the 0x34-byte saved parameter state at self+4 and the path descriptor
/// at +0x48, then obtain the owner Instance* from stream slot3. Allocate the
/// modifier's arrays and resolve each saved instance handle through the native
/// InstancesMan. Finally clone/setup those instances, insert their spatial
/// nodes and mark the modifier dirty. No SSF parameter conversion occurs here.
pub fn multi_spline_modifier_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x70 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 41;
                    Frame::open(c, frame.size);
                    c.g[V0][0] = 0x0049_0000;
                    frame.save(c, S0, 0x60);
                    c.g[V0][0] = sx(VTABLE);
                    frame.save(c, S2, 0x40);
                    c.g[S0][0] = 0xffff_ffff_ffff_0000;
                    frame.save(c, S1, 0x50);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, S3, 0x30);
                    c.g[S0][0] |= 0xffff;
                    frame.save(c, S4, 0x20);
                    c.g[A2][0] = 0x34;
                    frame.save_ra(c, 0x10);
                    c.g[S4][0] = c.g[A1][0];
                    c.sw(r(c, S2), c.g[V0][0]);
                    c.g[A1][0] = sx(at(c, S2, COUNT));
                    c.sw(at(c, S2, FOLLOW), c.g[S0][0]);
                    c.g[S1][0] = sx(at(c, S2, FOLLOW));
                    c.sw(at(c, S2, FOLLOW + 8), 0);
                    c.g[S3][0] = 0;
                    c.sw(at(c, S2, FOLLOW + 4), 0);
                    c.sw(at(c, S2, FOLLOW + 12), 0);
                    c.g[V0][0] = c.lw(r(c, S4));
                    c.g[A0][0] = c.lh(at(c, V0, 0x10));
                    c.g[V1][0] = c.lw(at(c, V0, 0x14));
                    let target = r(c, V1);
                    c.g[A0][0] = sx(r(c, S4).wrapping_add(r(c, A0)));
                    call_ptr(c, target, AFTER_PARAMETERS)?;
                    pc = AFTER_PARAMETERS;
                }
                AFTER_PARAMETERS => {
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S4][0];
                    LOAD_PATH.call(c, AFTER_PATH)?;
                    pc = AFTER_PATH;
                }
                AFTER_PATH => {
                    c.g[V1][0] = c.lw(r(c, S4));
                    c.g[A0][0] = c.lh(at(c, V1, 0x18));
                    c.g[V0][0] = c.lw(at(c, V1, 0x1c));
                    let target = r(c, V0);
                    c.g[A0][0] = sx(r(c, S4).wrapping_add(r(c, A0)));
                    call_ptr(c, target, AFTER_OWNER)?;
                    pc = AFTER_OWNER;
                }
                AFTER_OWNER => {
                    c.sw(at(c, S2, OWNER), c.g[V0][0]);
                    c.g[A0][0] = c.g[S2][0];
                    ALLOC_ARRAYS.call(c, AFTER_ARRAYS)?;
                    pc = AFTER_ARRAYS;
                }
                AFTER_ARRAYS => {
                    c.g[V0][0] = c.lw(at(c, S2, COUNT));
                    let empty = (c.g[V0][0] as i64) <= 0;
                    c.sw(r(c, SP), c.g[S0][0]);
                    if empty {
                        pc = CLONE_INSTANCES;
                    } else {
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(r(c, S4));
                        pc = INSTANCE_LOOP;
                    }
                }
                INSTANCE_LOOP => {
                    c.cycles += 20;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = 4;
                    c.g[A0][0] = c.lh(at(c, V1, 0x10));
                    c.g[V0][0] = c.lw(at(c, V1, 0x14));
                    let target = r(c, V0);
                    c.g[A0][0] = sx(r(c, S4).wrapping_add(r(c, A0)));
                    call_ptr(c, target, AFTER_HANDLE)?;
                    pc = AFTER_HANDLE;
                }
                AFTER_HANDLE => {
                    call_ptr(c, 0x002d_1ce0, AFTER_MANAGER)?;
                    pc = AFTER_MANAGER;
                }
                AFTER_MANAGER => {
                    c.g[A1][0] = c.lw(r(c, SP));
                    c.g[A0][0] = c.g[V0][0];
                    ALLOC_BY_HANDLE.call(c, AFTER_INSTANCE)?;
                    pc = AFTER_INSTANCE;
                }
                AFTER_INSTANCE => {
                    c.g[A0][0] = c.lw(at(c, S2, INSTANCES));
                    c.g[V1][0] = sx(r(c, S3) << 2);
                    c.g[S3][0] = sx(r(c, S3).wrapping_add(1));
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                    c.sw(r(c, V1), c.g[V0][0]);
                    c.g[V0][0] = c.lw(at(c, S2, COUNT));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        pc = CLONE_INSTANCES;
                    } else {
                        c.g[V1][0] = c.lw(r(c, S4));
                        if event_due(c) {
                            deliver_event_here(c, INSTANCE_LOOP);
                        }
                        pc = INSTANCE_LOOP;
                    }
                }
                CLONE_INSTANCES => {
                    c.cycles += 15;
                    c.g[A0][0] = c.g[S2][0];
                    CLONE.call(c, AFTER_CLONE)?;
                    pc = AFTER_CLONE;
                }
                AFTER_CLONE => {
                    c.g[A0][0] = c.g[S2][0];
                    INSERT.call(c, AFTER_INSERT)?;
                    pc = AFTER_INSERT;
                }
                AFTER_INSERT => {
                    c.g[V1][0] = 1;
                    c.g[V0][0] = c.g[S2][0];
                    c.sw(at(c, S2, DIRTY), c.g[V1][0]);
                    frame.restore(c, S0, 0x60);
                    frame.restore(c, S1, 0x50);
                    frame.restore(c, S2, 0x40);
                    frame.restore(c, S3, 0x30);
                    frame.restore(c, S4, 0x20);
                    frame.restore_ra(c, 0x10);
                    frame.close(c);
                    return Ok(());
                }
                other => {
                    c.jump_out(other, ENTRY, END);
                    return Ok(());
                }
            }
        }
    });
}
