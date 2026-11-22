//! Native cFlagManager_GroupAddFlag (0x0034b038), retained uninstalled.
//! Generated EE is authoritative. No tests, cargo, replays or game/window
//! control were performed, as requested by the user.

use crate::call::{guest, Callee, Frame, Unwound};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0034_b038;
const COPY_UNALIGNED: u32 = 0x0034_b06c;
const COPY_ALIGNED: u32 = 0x0034_b0c4;
const COPY_TAIL: u32 = 0x0034_b0f0;
const AFTER_BUILD_MESH: u32 = 0x0034_b138;
const APPEND: u32 = 0x0034_b13c;
const END: u32 = 0x0034_b168;
const BUILD_MESH: Callee = Callee {
    addr: 0x0034_b228,
    f: ssx3_ee::s11::f_0034b228,
};

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void Group_Add(Group* group, const FlagKey* key, Instance* instance)`.
/// For the first member, copy the complete 0x58-byte key, capture the
/// native MDR resource ID from `*instance->model`, and build the mesh from
/// that instance. Later members reuse those group data. Append Instance*
/// at +0x94 and increment the member count at +0x5c. The inline storage
/// before the +0x184 vtable accommodates 60 pointers; this function has no
/// capacity check and performs no duplicate check or reference counting.
/// Mesh geometry and effect phases are initialized by BuildMesh, not here.
pub fn flag_manager_group_add(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x30 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 8;
                    Frame::open(c, 0x30);
                    frame.save(c, S0, 0x20);
                    frame.save(c, S1, 0x10);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[V0][0] = c.lw(at(c, S0, 0x5c));
                    let has_members = c.g[V0][0] != 0;
                    c.g[S1][0] = c.g[A2][0];
                    if has_members {
                        pc = APPEND;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.g[A1][0] | c.g[S0][0];
                        c.g[V0][0] &= 7;
                        let aligned = c.g[V0][0] == 0;
                        c.g[V1][0] = c.g[S0][0];
                        c.cycles += 1;
                        c.g[V0][0] = sx(at(c, A1, 0x40));
                        pc = if aligned { COPY_ALIGNED } else { COPY_UNALIGNED };
                    }
                }
                COPY_UNALIGNED => {
                    c.cycles += 19;
                    c.ldl(A0, at(c, A1, 7));
                    c.ldr(A0, r(c, A1));
                    c.ldl(A2, at(c, A1, 0xf));
                    c.ldr(A2, at(c, A1, 8));
                    c.ldl(A3, at(c, A1, 0x17));
                    c.ldr(A3, at(c, A1, 0x10));
                    c.ldl(T0, at(c, A1, 0x1f));
                    c.ldr(T0, at(c, A1, 0x18));
                    c.sdl(A0, at(c, V1, 7));
                    c.sdr(A0, r(c, V1));
                    c.sdl(A2, at(c, V1, 0xf));
                    c.sdr(A2, at(c, V1, 8));
                    c.sdl(A3, at(c, V1, 0x17));
                    c.sdr(A3, at(c, V1, 0x10));
                    c.sdl(T0, at(c, V1, 0x1f));
                    c.sdr(T0, at(c, V1, 0x18));
                    c.g[A1][0] = sx(at(c, A1, 0x20));
                    let more = c.g[A1][0] != c.g[V0][0];
                    c.g[V1][0] = sx(at(c, V1, 0x20));
                    if more {
                        if back_edge_unwinds(c, COPY_UNALIGNED) {
                            return Err(Unwound);
                        }
                    } else {
                        c.cycles += 2;
                        pc = COPY_TAIL;
                    }
                }
                COPY_ALIGNED => {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld(r(c, A1));
                    c.g[A2][0] = c.ld(at(c, A1, 8));
                    c.g[A3][0] = c.ld(at(c, A1, 0x10));
                    c.g[T0][0] = c.ld(at(c, A1, 0x18));
                    c.sd(r(c, V1), c.g[A0][0]);
                    c.sd(at(c, V1, 8), c.g[A2][0]);
                    c.sd(at(c, V1, 0x10), c.g[A3][0]);
                    c.sd(at(c, V1, 0x18), c.g[T0][0]);
                    c.g[A1][0] = sx(at(c, A1, 0x20));
                    let more = c.g[A1][0] != c.g[V0][0];
                    c.g[V1][0] = sx(at(c, V1, 0x20));
                    if more {
                        if back_edge_unwinds(c, COPY_ALIGNED) {
                            return Err(Unwound);
                        }
                    } else {
                        pc = COPY_TAIL;
                    }
                }
                COPY_TAIL => {
                    c.cycles += 19;
                    c.ldl(V0, at(c, A1, 7));
                    c.ldr(V0, r(c, A1));
                    c.ldl(A0, at(c, A1, 0xf));
                    c.ldr(A0, at(c, A1, 8));
                    c.ldl(A2, at(c, A1, 0x17));
                    c.ldr(A2, at(c, A1, 0x10));
                    c.sdl(V0, at(c, V1, 7));
                    c.sdr(V0, r(c, V1));
                    c.sdl(A0, at(c, V1, 0xf));
                    c.sdr(A0, at(c, V1, 8));
                    c.sdl(A2, at(c, V1, 0x17));
                    c.sdr(A2, at(c, V1, 0x10));
                    c.g[V0][0] = c.lw(at(c, S1, 0x80));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V1][0] = c.lw(r(c, V0));
                    c.g[RA][0] = sx(AFTER_BUILD_MESH);
                    c.sw(at(c, S0, 0x58), c.g[V1][0]);
                    BUILD_MESH.call(c, AFTER_BUILD_MESH)?;
                    pc = AFTER_BUILD_MESH;
                }
                AFTER_BUILD_MESH => {
                    c.g[V0][0] = c.lw(at(c, S0, 0x5c));
                    pc = APPEND;
                }
                APPEND => {
                    c.cycles += 11;
                    frame.restore_ra(c, 0);
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[V0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
                    c.sw(at(c, V0, 0x94), c.g[S1][0]);
                    frame.restore(c, S1, 0x10);
                    c.g[V1][0] = c.lw(at(c, S0, 0x5c));
                    c.g[V1][0] = sx(at(c, V1, 1));
                    c.sw(at(c, S0, 0x5c), c.g[V1][0]);
                    frame.restore(c, S0, 0x20);
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
