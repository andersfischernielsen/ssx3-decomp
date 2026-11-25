//! Staging module oc662 (wave 234, list oc662): hand-written replacements, each
//! with its differential test against the generated body. Moved into the
//! source-tree units by `tools/reorg_modules.py apply oc662`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0034_c8e8, fence_ctor_3, "Fence_ctor_3"),
    (0x0034_cab8, fence_ctor, "Fence_ctor"),
    (0x0034_cbe8, fence_ctor_2, "Fence_ctor_2"),
    (0x0034_cc80, fence_propagate_to_neighbour, "Fence_PropagateToNeighbour"),
    (0x0034_cf98, fence_update_verts, "Fence_UpdateVerts"),
];

/// The 32-bit value of GPR `r` (the generated code's `c.g[r][0] as u32`).
#[inline(always)]
fn fence_gpr(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// `sp + off`, read from `sp` each time as the generated code does.
#[inline(always)]
fn fence_sp(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

/// `gp - off`: a float constant or global of the game's gp data.
#[inline(always)]
fn fence_gp(c: &Cpu, off: u32) -> u32 {
    fence_gpr(c, GP).wrapping_sub(off)
}

/// `void cFenceNode::cFenceNode(int bucket, cBxReplayFrame *data)` (0x34cbe8,
/// `Fence_ctor_2`): the replay constructor. The node is first built as a world
/// object (`WorldObj_ctor_2`), then its vtable and instance id are set, the
/// 0x24-byte fence state is read from the record (`stream->read`, the record's
/// vtable slot at +0x14, length 0x24), and the mesh is built. Returns the node.
pub fn fence_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(16, fence_sp(c, 0x20));
        c.sq(17, fence_sp(c, 0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0034_fb90, 0x0034_cc04)?;
        // Instance id -1 (the low half of the object's id word), the node's
        // bounding data words.
        c.g[V0][0] = u64::MAX;
        c.g[V1][0] = sx(0x0049_0000u32.wrapping_sub(0x600));
        c.sw(fence_gpr(c, S0).wrapping_add(0x1c), c.g[V0][0]);
        c.g[A0][0] = u64::MAX;
        c.sw(fence_gpr(c, S0).wrapping_add(0xc), c.g[V1][0]);
        // Sixteen steps of the stream's first word down to -1 (the loop runs
        // while V0 != A0).
        c.g[V0][0] = 0xf;
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx(fence_gpr(c, V0).wrapping_sub(0x1));
            let t = c.g[V0][0] != c.g[A0][0];
            c.g[A1][0] = sx(fence_gpr(c, S0).wrapping_add(0x1c));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0034_cc28);
                }
                continue;
            }
            break;
        }
        c.cycles += 14;
        c.g[V0][0] = c.lw(fence_gpr(c, S1));
        c.g[A2][0] = 0x24;
        c.g[A0][0] = c.lh(fence_gpr(c, V0).wrapping_add(0x10));
        c.g[V1][0] = c.lw(fence_gpr(c, V0).wrapping_add(0x14));
        let read = fence_gpr(c, V1);
        c.g[A0][0] = sx(fence_gpr(c, S1).wrapping_add(fence_gpr(c, A0)));
        call_ptr(c, read, 0x0034_cc5c)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0034_d1e8, 0x0034_cc64)?;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, fence_sp(c, 0x10));
        c.lq(16, fence_sp(c, 0x20));
        c.g[RA][0] = c.ld(sp(c));
        let t = fence_gpr(c, RA);
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void cFenceNode::cFenceNode(int bucket, tWorld *obj, ..., float force)`
/// (0x34cab8, `Fence_ctor`): the constructor for a fence built from a
/// `tMakeFenceData` record (`a3`, copied as 16 bytes into the node) and a
/// force. Builds the world object, the mesh and the impulse; the two
/// neighbour counters are set to 4.
pub fn fence_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 49;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.sq(17, fence_sp(c, 0x10));
        c.sq(16, fence_sp(c, 0x20));
        c.g[S1][0] = c.g[A3][0];
        c.g[S0][0] = c.g[A0][0];
        c.swc1(20, fence_sp(c, 0x30));
        c.g[A3][0] = c.g[A2][0];
        c.sd(sp(c), c.g[RA][0]);
        c.mov_s(20, 12);
        c.g[A2][0] = 0x9;
        call_ptr(c, 0x0034_fb00, 0x0034_cae4)?;
        c.g[V0][0] = u64::MAX;
        c.g[V1][0] = sx(0x0049_0000u32.wrapping_sub(0x600));
        c.sw(fence_gpr(c, S0).wrapping_add(0x1c), c.g[V0][0]);
        c.g[A0][0] = u64::MAX;
        c.sw(fence_gpr(c, S0).wrapping_add(0xc), c.g[V1][0]);
        c.g[V0][0] = 0xf;
        c.ldl(3, fence_gpr(c, S1).wrapping_add(0x7));
        c.ldr(3, fence_gpr(c, S1));
        c.ldl(4, fence_gpr(c, S1).wrapping_add(0xf));
        c.ldr(4, fence_gpr(c, S1).wrapping_add(0x8));
        c.sdl(3, fence_gpr(c, S0).wrapping_add(0x23));
        c.sdr(3, fence_gpr(c, S0).wrapping_add(0x1c));
        c.sdl(4, fence_gpr(c, S0).wrapping_add(0x2b));
        c.sdr(4, fence_gpr(c, S0).wrapping_add(0x24));
        c.g[V0][0] = u64::MAX;
        c.sw(fence_gpr(c, S0).wrapping_add(0x30), 0);
        c.sw(fence_gpr(c, S0).wrapping_add(0x2c), 0);
        c.g[A0][0] = c.g[S0][0];
        c.sw(fence_gpr(c, S0).wrapping_add(0x34), 0);
        c.sw(fence_gpr(c, S0).wrapping_add(0x1c), c.g[V0][0]);
        call_ptr(c, 0x0034_d1e8, 0x0034_cb48)?;
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0034_d6f0, 0x0034_cb54)?;
        c.g[V1][0] = 0x4;
        c.g[V0][0] = c.g[S0][0];
        c.sw(fence_gpr(c, S0).wrapping_add(0x3c), c.g[V1][0]);
        c.sw(fence_gpr(c, S0).wrapping_add(0x38), c.g[V1][0]);
        c.lq(16, fence_sp(c, 0x20));
        c.lq(17, fence_sp(c, 0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.lwc1(20, fence_sp(c, 0x30));
        let t = fence_gpr(c, RA);
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `cFenceNode::cFenceNode(int bucket, tWorld *obj, sFlagElem *desc, ...,
/// float force)` (0x34c8e8, `Fence_ctor_3`): the hit constructor. Builds the
/// world object, copies the 16-byte descriptor at `a3` into the node, builds
/// the mesh, then either sets the tip from the mesh's 0x40 word (the vertex
/// maths on the info block at `t0`, the board at +0x18) or from the fence's
/// own vertices; applies the impulse and propagates to the neighbours.
pub fn fence_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        'b64: {
            'b30: {
                c.cycles += 40;
                c.g[SP][0] = sx(sp(c).wrapping_sub(0x70));
                c.sq(18, fence_sp(c, 0x40));
                c.sq(16, fence_sp(c, 0x60));
                c.g[S2][0] = c.g[A3][0];
                c.g[S0][0] = c.g[A0][0];
                c.sq(17, fence_sp(c, 0x50));
                c.g[A3][0] = c.g[A2][0];
                c.sd(fence_sp(c, 0x30), c.g[RA][0]);
                c.g[S1][0] = c.g[T0][0];
                c.g[A2][0] = 0x9;
                call_ptr(c, 0x0034_fb00, 0x0034_c914)?;
                c.g[V0][0] = u64::MAX;
                c.g[V1][0] = sx(0x0049_0000u32.wrapping_sub(0x600));
                c.sw(fence_gpr(c, S0).wrapping_add(0x1c), c.g[V0][0]);
                c.g[A0][0] = u64::MAX;
                c.sw(fence_gpr(c, S0).wrapping_add(0xc), c.g[V1][0]);
                c.g[V0][0] = 0xf;
                c.ldl(3, fence_gpr(c, S2).wrapping_add(0x7));
                c.ldr(3, fence_gpr(c, S2));
                c.ldl(4, fence_gpr(c, S2).wrapping_add(0xf));
                c.ldr(4, fence_gpr(c, S2).wrapping_add(0x8));
                c.sdl(3, fence_gpr(c, S0).wrapping_add(0x23));
                c.sdr(3, fence_gpr(c, S0).wrapping_add(0x1c));
                c.sdl(4, fence_gpr(c, S0).wrapping_add(0x2b));
                c.sdr(4, fence_gpr(c, S0).wrapping_add(0x24));
                c.g[V0][0] = u64::MAX;
                c.sw(fence_gpr(c, S0).wrapping_add(0x30), 0);
                c.sw(fence_gpr(c, S0).wrapping_add(0x2c), 0);
                c.g[A0][0] = c.g[S0][0];
                c.sw(fence_gpr(c, S0).wrapping_add(0x34), 0);
                c.sw(fence_gpr(c, S0).wrapping_add(0x1c), c.g[V0][0]);
                call_ptr(c, 0x0034_d1e8, 0x0034_c978)?;
                c.g[V0][0] = c.lw(fence_gpr(c, S0).wrapping_add(0x40));
                c.sw(fence_gpr(c, S0).wrapping_add(0x38), 0);
                let t = c.g[V0][0] == 0;
                c.sw(fence_gpr(c, S0).wrapping_add(0x3c), 0);
                if t {
                    break 'b30;
                }
                c.cycles += 42;
                c.g[V1][0] = c.lw(fence_gpr(c, S0).wrapping_add(0x18));
                c.g[V0][0] = 0x500000;
                c.g[V0][0] = sx(fence_gpr(c, V0).wrapping_sub(0xec0));
                c.lwc1(1, fence_gp(c, 0x2ad8));
                c.g[V1][0] = sx(fence_gpr(c, V1).wrapping_add(0x10));
                c.lqc2(8, fence_gpr(c, V0));
                c.lqc2(4, fence_gpr(c, V1));
                c.lqc2(5, fence_gpr(c, V1).wrapping_add(0x10));
                c.lqc2(6, fence_gpr(c, V1).wrapping_add(0x20));
                c.lqc2(7, fence_gpr(c, V1).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, fence_sp(c, 0x20));
                c.lq(2, fence_sp(c, 0x20));
                c.sq(2, fence_sp(c, 0x10));
                c.lqc2(3, fence_sp(c, 0x10));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, fence_sp(c, 0x20));
                c.lq(2, fence_sp(c, 0x20));
                c.lwc1(0, fence_gpr(c, S1).wrapping_add(0x30));
                c.sq(2, sp(c));
                c.abs_s(0, 0);
                c.lqc2(3, sp(c));
                c.lqc2(5, fence_gpr(c, S1).wrapping_add(0x10));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(2, 4);
                c.mtc1(2, 12);
                c.mul_s(1, 12, 1);
                break 'b64;
            }
            c.cycles += 13;
            c.lwc1(0, fence_gp(c, 0x2ad4));
            c.lqc2(3, fence_gpr(c, S1).wrapping_add(0x20));
            c.lqc2(5, fence_gpr(c, S1).wrapping_add(0x10));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(2, 4);
            c.mtc1(2, 12);
            c.lwc1(1, fence_gpr(c, S1).wrapping_add(0x30));
            c.mul_s(0, 12, 0);
            c.abs_s(1, 1);
            break 'b64;
        }
        'b84: {
            c.cycles += 6;
            c.mul_s(12, 0, 1);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0034_d6f0, 0x0034_ca70)?;
            c.g[A1][0] = c.lw(fence_gpr(c, S0).wrapping_add(0x104));
            let t = c.g[A1][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if t {
                break 'b84;
            }
            c.cycles += 2;
            c.g[A2][0] = 0x1;
            call_ptr(c, 0x0034_cc80, 0x0034_ca84)?;
            break 'b84;
        }
        'b98: {
            c.cycles += 3;
            c.g[A1][0] = c.lw(fence_gpr(c, S0).wrapping_add(0x108));
            let t = c.g[A1][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if t {
                break 'b98;
            }
            c.cycles += 2;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0034_cc80, 0x0034_ca98)?;
            break 'b98;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, fence_sp(c, 0x50));
        c.lq(16, fence_sp(c, 0x60));
        c.lq(18, fence_sp(c, 0x40));
        c.g[RA][0] = c.ld(fence_sp(c, 0x30));
        let t = fence_gpr(c, RA);
        c.g[SP][0] = sx(sp(c).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `void cFenceNode::makeNeighbour(...)`-side propagation (0x34cc80,
/// `Fence_PropagateToNeighbour`): pushes the fence's sway to a neighbour
/// (`a1`) when the force is large enough. Its own force is the fence's
/// `ForceMax * +-0.4`; a neighbour object's vtable decides the kick, and a
/// free neighbour slot gets a new fence built with `Fence_ctor`.
pub fn fence_propagate_to_neighbour(c: &mut Cpu) {
    guest(c, |c| {
        'a: {
            'b: {
                'cc: {
                    c.cycles += 11;
                    c.g[SP][0] = sx(sp(c).wrapping_sub(0x50));
                    c.sq(16, fence_sp(c, 0x30));
                    c.sq(17, fence_sp(c, 0x20));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(18, fence_sp(c, 0x10));
                    c.g[S1][0] = c.g[A1][0];
                    c.sd(sp(c), c.g[RA][0]);
                    c.swc1(20, fence_sp(c, 0x40));
                    c.g[V0][0] = c.lw(fence_gpr(c, S0).wrapping_add(0x40));
                    let t = c.g[V0][0] == 0;
                    c.g[S2][0] = c.g[A2][0];
                    if t {
                        break 'cc;
                    }
                    c.cycles += 2;
                    if c.g[S1][0] == 0 {
                        c.lq(16, fence_sp(c, 0x30));
                        break 'b;
                    }
                    'd: {
                        'e: {
                            c.cycles += 6;
                            c.lwc1(1, fence_gpr(c, S0).wrapping_add(0x34));
                            c.mtc1(0, 0);
                            c.c_lt_s(1, 0);
                            let t = !c.fcc();
                            c.lwc1(1, fence_gpr(c, S0).wrapping_add(0x30));
                            if t {
                                break 'e;
                            }
                            c.cycles += 3;
                            c.lwc1(0, fence_gp(c, 0x2ad0));
                            c.g[A2][0] = c.lw(fence_gpr(c, S1).wrapping_add(0xc));
                            break 'd;
                        }
                        c.cycles += 2;
                        c.lwc1(0, fence_gp(c, 0x2acc));
                        c.g[A2][0] = c.lw(fence_gpr(c, S1).wrapping_add(0xc));
                        break 'd;
                    }
                    'f: {
                        c.cycles += 2;
                        let t = c.g[A2][0] == 0;
                        c.mul_s(20, 1, 0);
                        if t {
                            break 'f;
                        }
                        c.cycles += 8;
                        c.g[V1][0] = c.lw(fence_gpr(c, A2).wrapping_add(0xc));
                        c.g[A1][0] = 0x9;
                        c.g[A0][0] = c.lh(fence_gpr(c, V1).wrapping_add(0x50));
                        c.g[V0][0] = c.lw(fence_gpr(c, V1).wrapping_add(0x54));
                        let hook = fence_gpr(c, V0);
                        c.g[A0][0] = sx(fence_gpr(c, A2).wrapping_add(fence_gpr(c, A0)));
                        call_ptr(c, hook, 0x0034_cd00)?;
                        if c.g[V0][0] == 0 {
                            c.lq(16, fence_sp(c, 0x30));
                            break 'b;
                        }
                        c.cycles += 7;
                        c.g[AT][0] = 0x3e80_0000;
                        c.mtc1(1, 1);
                        c.abs_s(0, 20);
                        c.c_le_s(1, 0);
                        if !c.fcc() {
                            c.lq(16, fence_sp(c, 0x30));
                            break 'b;
                        }
                        'g: {
                            c.cycles += 2;
                            if c.g[S2][0] == 0 {
                                c.g[A0][0] = c.lw(fence_gpr(c, S0).wrapping_add(0x108));
                                break 'g;
                            }
                            c.cycles += 3;
                            c.g[A0][0] = c.lw(fence_gpr(c, S0).wrapping_add(0x104));
                            let t = c.g[A0][0] == 0;
                            c.lq(16, fence_sp(c, 0x30));
                            if t {
                                break 'b;
                            }
                            c.cycles += 3;
                            c.g[A0][0] = c.lw(fence_gpr(c, A0).wrapping_add(0xc));
                            let t = c.g[A0][0] == 0;
                            c.lq(17, fence_sp(c, 0x20));
                            if t {
                                break 'a;
                            }
                            c.cycles += 5;
                            c.g[A1][0] = c.g[S2][0];
                            c.mov_s(12, 20);
                            call_ptr(c, 0x0034_cde8, 0x0034_cd50)?;
                            c.lq(16, fence_sp(c, 0x30));
                            break 'b;
                        }
                        c.cycles += 2;
                        let t = c.g[A0][0] == 0;
                        c.lq(16, fence_sp(c, 0x30));
                        if t {
                            break 'b;
                        }
                        c.cycles += 3;
                        c.g[A0][0] = c.lw(fence_gpr(c, A0).wrapping_add(0xc));
                        let t = c.g[A0][0] == 0;
                        c.mov_s(12, 20);
                        if t {
                            break 'b;
                        }
                        c.cycles += 4;
                        c.g[A1][0] = 0;
                        call_ptr(c, 0x0034_cde8, 0x0034_cd74)?;
                        c.lq(16, fence_sp(c, 0x30));
                        break 'b;
                    }
                    c.cycles += 7;
                    c.g[AT][0] = 0x3e80_0000;
                    c.mtc1(1, 1);
                    c.abs_s(0, 20);
                    c.c_le_s(1, 0);
                    let t = !c.fcc();
                    c.g[A0][0] = 0x10c;
                    if t {
                        break 'cc;
                    }
                    c.cycles += 12;
                    c.g[A1][0] = 0x4a_0000;
                    c.g[A1][0] = sx(fence_gpr(c, A1).wrapping_add(0x4018));
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    call_ptr(c, 0x0031_7d70, 0x0034_cdac)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A3][0] = sx(fence_gpr(c, S0).wrapping_add(0x1c));
                    c.g[T0][0] = c.g[S2][0];
                    c.mov_s(12, 20);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0034_cab8, 0x0034_cdc8)?;
                    break 'cc;
                }
                c.cycles += 1;
                c.lq(16, fence_sp(c, 0x30));
            }
            c.cycles += 1;
            c.lq(17, fence_sp(c, 0x20));
        }
        c.cycles += 5;
        c.lq(18, fence_sp(c, 0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.lwc1(20, fence_sp(c, 0x40));
        let t = fence_gpr(c, RA);
        c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `void cFenceNode::updateGrid(cFenceNode *self)` (0x34cf98,
/// `Fence_UpdateVerts`): moves the fence's vertices with its sway. Adds the
/// fence's bend (`Force * Flexibility` along x, eight bend-table weights) to
/// the rest positions, writes the top row (or the whole grid) into the mesh's
/// vertex buffer, then hands it to the mesh's vertex setter (vtable slot at
/// +0x1c of the mesh's board). Does nothing without a mesh.
pub fn fence_update_verts(c: &mut Cpu) {
    guest(c, |c| {
        'b1d8: {
            c.cycles += 5;
            c.g[SP][0] = sx(sp(c).wrapping_sub(0x180));
            c.sd(fence_sp(c, 0x170), c.g[RA][0]);
            c.g[V0][0] = c.lw(fence_gpr(c, A0).wrapping_add(0x40));
            let t = c.g[V0][0] == 0;
            c.g[A2][0] = 0x7;
            if t {
                break 'b1d8;
            }
            c.cycles += 19;
            c.lwc1(0, fence_gpr(c, A0).wrapping_add(0x2c));
            c.lwc1(1, fence_gpr(c, A0).wrapping_add(0x24));
            c.g[V1][0] = sx(fence_sp(c, 0x10));
            c.g[AT][0] = 0x4348_0000;
            c.mtc1(1, 2);
            c.g[T1][0] = sx(fence_sp(c, 0x110));
            c.mul_s(0, 0, 1);
            c.sw(fence_sp(c, 0x4), 0);
            c.swc1(2, sp(c));
            c.g[A1][0] = sx(fence_sp(c, 0xb0));
            c.sw(fence_sp(c, 0x8), 0);
            c.g[A3][0] = u64::MAX;
            c.sw(fence_sp(c, 0xc), 0);
            c.mfc1(2, 0);
            c.lqc2(4, sp(c));
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp(c));
            // Seven rows of the first pass (A2 counts down from 7).
            'l1: loop {
                c.cycles += 7;
                c.g[A2][0] = sx(fence_gpr(c, A2).wrapping_sub(0x1));
                let t = c.g[A2][0] != c.g[A3][0];
                c.g[V0][0] = 0x450000;
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_cff8);
                    }
                    continue 'l1;
                }
                c.cycles += 3;
                c.g[A2][0] = c.g[V1][0];
                c.g[T0][0] = sx(fence_gpr(c, V0).wrapping_sub(0x5030));
                c.g[A3][0] = 0x7;
                'l2: loop {
                    c.cycles += 14;
                    c.lwc1(0, fence_gpr(c, T0));
                    c.g[V1][0] = c.g[A2][0];
                    c.mfc1(2, 0);
                    c.lqc2(4, sp(c));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, fence_sp(c, 0xa0));
                    c.g[A2][0] = sx(fence_gpr(c, A2).wrapping_add(0x10));
                    c.lq(2, fence_sp(c, 0xa0));
                    c.g[T0][0] = sx(fence_gpr(c, T0).wrapping_add(0x4));
                    c.g[A3][0] = sx(fence_gpr(c, A3).wrapping_sub(0x1));
                    c.sq(2, fence_sp(c, 0x90));
                    let t = (c.g[A3][0] as i64) >= 0;
                    c.sq(2, fence_gpr(c, V1));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0034_d020);
                        }
                        continue 'l2;
                    }
                    'b1c4: {
                        'b138: {
                            c.cycles += 7;
                            c.g[A2][0] = c.lw(fence_gpr(c, A0).wrapping_add(0x20));
                            c.g[V0][0] = 0xf;
                            c.g[V1][0] = u64::MAX;
                            c.g[V0][0] = 0x1;
                            let t = c.g[A2][0] != c.g[V0][0];
                            c.g[A3][0] = c.g[T1][0];
                            if t {
                                break 'b138;
                            }
                            c.cycles += 5;
                            c.g[V1][0] = sx(fence_sp(c, 0x10));
                            c.g[V0][0] = sx(fence_gpr(c, A0).wrapping_add(0xa4));
                            c.g[A2][0] = sx(fence_sp(c, 0xb0));
                            c.g[T0][0] = 0x7;
                            'l3: loop {
                                c.cycles += 42;
                                c.lwc1(4, fence_gpr(c, V1));
                                c.g[T0][0] = sx(fence_gpr(c, T0).wrapping_sub(0x1));
                                c.lwc1(3, fence_gpr(c, V1).wrapping_add(0x4));
                                c.lwc1(6, fence_gpr(c, V1).wrapping_add(0x8));
                                c.lwc1(0, fence_gpr(c, V0).wrapping_sub(0x60));
                                c.lwc1(1, fence_gpr(c, V0).wrapping_sub(0x5c));
                                c.lwc1(2, fence_gpr(c, V0).wrapping_sub(0x58));
                                c.add_s(0, 0, 4);
                                c.add_s(1, 1, 3);
                                c.lwc1(5, fence_gpr(c, V0));
                                c.add_s(2, 2, 6);
                                c.lwc1(4, fence_gpr(c, V0).wrapping_add(0x4));
                                c.swc1(0, fence_sp(c, 0x90));
                                c.swc1(1, fence_sp(c, 0x94));
                                c.swc1(2, fence_sp(c, 0x98));
                                c.ldl(9, fence_sp(c, 0x97));
                                c.ldr(9, fence_sp(c, 0x90));
                                c.g[T2][0] = c.lw(fence_sp(c, 0x98));
                                c.sdl(9, fence_gpr(c, A2).wrapping_add(0x7));
                                c.sdr(9, fence_gpr(c, A2));
                                c.sw(fence_gpr(c, A2).wrapping_add(0x8), c.g[T2][0]);
                                c.g[A2][0] = sx(fence_gpr(c, A2).wrapping_add(0xc));
                                c.lwc1(1, fence_gpr(c, V0).wrapping_add(0x8));
                                c.lwc1(2, fence_gpr(c, V1));
                                c.g[V0][0] = sx(fence_gpr(c, V0).wrapping_add(0xc));
                                c.lwc1(0, fence_gpr(c, V1).wrapping_add(0x4));
                                c.lwc1(3, fence_gpr(c, V1).wrapping_add(0x8));
                                c.add_s(5, 5, 2);
                                c.add_s(4, 4, 0);
                                c.g[V1][0] = sx(fence_gpr(c, V1).wrapping_add(0x10));
                                c.add_s(1, 1, 3);
                                c.swc1(5, fence_sp(c, 0x90));
                                c.swc1(4, fence_sp(c, 0x94));
                                c.swc1(1, fence_sp(c, 0x98));
                                c.ldl(9, fence_sp(c, 0x97));
                                c.ldr(9, fence_sp(c, 0x90));
                                c.g[T2][0] = c.lw(fence_sp(c, 0x98));
                                c.sdl(9, fence_gpr(c, A3).wrapping_add(0x7));
                                c.sdr(9, fence_gpr(c, A3));
                                c.sw(fence_gpr(c, A3).wrapping_add(0x8), c.g[T2][0]);
                                let t = (c.g[T0][0] as i64) >= 0;
                                c.g[A3][0] = sx(fence_gpr(c, A3).wrapping_add(0xc));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0034_d088);
                                    }
                                    continue 'l3;
                                }
                                c.cycles += 2;
                                c.g[A2][0] = c.lw(fence_gpr(c, A0).wrapping_add(0x40));
                                break 'b1c4;
                            }
                        }
                        c.cycles += 4;
                        c.g[A2][0] = sx(fence_sp(c, 0xb0));
                        c.g[V1][0] = sx(fence_sp(c, 0x10));
                        c.g[V0][0] = sx(fence_gpr(c, A0).wrapping_add(0x44));
                        c.g[T0][0] = 0x7;
                        'l4: loop {
                            c.cycles += 30;
                            c.lwc1(5, fence_gpr(c, V1));
                            c.g[T0][0] = sx(fence_gpr(c, T0).wrapping_sub(0x1));
                            c.lwc1(3, fence_gpr(c, V1).wrapping_add(0x4));
                            c.lwc1(4, fence_gpr(c, V1).wrapping_add(0x8));
                            c.lwc1(2, fence_gpr(c, V0));
                            c.g[V1][0] = sx(fence_gpr(c, V1).wrapping_add(0x10));
                            c.lwc1(0, fence_gpr(c, V0).wrapping_add(0x4));
                            c.lwc1(1, fence_gpr(c, V0).wrapping_add(0x8));
                            c.add_s(2, 2, 5);
                            c.add_s(0, 0, 3);
                            c.add_s(1, 1, 4);
                            c.swc1(2, fence_sp(c, 0x90));
                            c.swc1(0, fence_sp(c, 0x94));
                            c.swc1(1, fence_sp(c, 0x98));
                            c.ldl(9, fence_sp(c, 0x97));
                            c.ldr(9, fence_sp(c, 0x90));
                            c.g[T2][0] = c.lw(fence_sp(c, 0x98));
                            c.sdl(9, fence_gpr(c, A2).wrapping_add(0x7));
                            c.sdr(9, fence_gpr(c, A2));
                            c.sw(fence_gpr(c, A2).wrapping_add(0x8), c.g[T2][0]);
                            c.ldl(9, fence_gpr(c, V0).wrapping_add(0x67));
                            c.ldr(9, fence_gpr(c, V0).wrapping_add(0x60));
                            c.g[T2][0] = c.lw(fence_gpr(c, V0).wrapping_add(0x68));
                            c.sdl(9, fence_gpr(c, A3).wrapping_add(0x7));
                            c.sdr(9, fence_gpr(c, A3));
                            c.sw(fence_gpr(c, A3).wrapping_add(0x8), c.g[T2][0]);
                            c.g[A2][0] = sx(fence_gpr(c, A2).wrapping_add(0xc));
                            c.g[A3][0] = sx(fence_gpr(c, A3).wrapping_add(0xc));
                            let t = (c.g[T0][0] as i64) >= 0;
                            c.g[V0][0] = sx(fence_gpr(c, V0).wrapping_add(0xc));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0034_d148);
                                }
                                continue 'l4;
                            }
                            c.cycles += 1;
                            c.g[A2][0] = c.lw(fence_gpr(c, A0).wrapping_add(0x40));
                            break 'b1c4;
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(fence_gpr(c, A2).wrapping_add(0x4));
                    c.g[A0][0] = c.lh(fence_gpr(c, V0).wrapping_add(0x18));
                    c.g[V1][0] = c.lw(fence_gpr(c, V0).wrapping_add(0x1c));
                    let setter = fence_gpr(c, V1);
                    c.g[A0][0] = sx(fence_gpr(c, A2).wrapping_add(fence_gpr(c, A0)));
                    call_ptr(c, setter, 0x0034_d1d8)?;
                    break 'b1d8;
                }
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(fence_sp(c, 0x170));
        let t = fence_gpr(c, RA);
        c.g[SP][0] = sx(sp(c).wrapping_add(0x180));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use std::sync::atomic::AtomicUsize;

    /// The gate word: the fence fakes below answer only while it holds MAGIC
    /// (set by each test in its own machine's RAM).
    const OC662_CTRL: u32 = DATA + 0x20_0000;
    const OC662_MAGIC: u32 = 0x0662_0662;

    static OC662_PREV_WORLD_OBJ_CTOR: AtomicUsize = AtomicUsize::new(0);
    static OC662_PREV_WORLD_OBJ_CTOR_2: AtomicUsize = AtomicUsize::new(0);
    static OC662_PREV_ALLOC: AtomicUsize = AtomicUsize::new(0);

    /// `WorldObj_ctor` while the gate is set: logged, returns to `ra`.
    fn oc662_fake_world_obj_ctor(c: &mut Cpu) {
        if c.bus.read32(OC662_CTRL) != OC662_MAGIC {
            return fakes::fall_back(
                c,
                &OC662_PREV_WORLD_OBJ_CTOR,
                ee::s11::f_0034fb00__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0034_fb00, ra)
    }

    /// `WorldObj_ctor_2` while the gate is set: logged, returns to `ra`.
    fn oc662_fake_world_obj_ctor_2(c: &mut Cpu) {
        if c.bus.read32(OC662_CTRL) != OC662_MAGIC {
            return fakes::fall_back(
                c,
                &OC662_PREV_WORLD_OBJ_CTOR_2,
                ee::s11::f_0034fb90__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0034_fb90, ra)
    }

    /// `EA_MemAllocNamed` while the gate is set: logged, returns a pointer.
    fn oc662_fake_alloc(c: &mut Cpu) {
        if c.bus.read32(OC662_CTRL) != OC662_MAGIC {
            return fakes::fall_back(c, &OC662_PREV_ALLOC, ee::s10::f_00317d70__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_7d70, ra)
    }

    fn oc662_hooks() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_0034FB00),
            oc662_fake_world_obj_ctor,
            &OC662_PREV_WORLD_OBJ_CTOR,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_0034FB90),
            oc662_fake_world_obj_ctor_2,
            &OC662_PREV_WORLD_OBJ_CTOR_2,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc662_fake_alloc,
            &OC662_PREV_ALLOC,
        );
    }

    /// The world and the mesh manager `Fence_BuildMesh` reads: its indirect
    /// call goes to a fake, and the fence's board is at +0x18.
    fn oc662_world(c: &mut Cpu, fence: u32) {
        let world = DATA + 0x8_0000;
        let mgr = DATA + 0x8_1000;
        let board = DATA + 0x8_2000;
        c.bus.write32(GAME_GP + 0x2a90, world);
        c.bus.write32(world + 0x10d8, mgr);
        c.bus.write16(mgr + 0x260, 0);
        c.bus.write32(mgr + 0x264, FAKE + 0x20);
        c.bus.write32(fence + 0x18, board);
        for k in 0..0x10u32 {
            c.bus.write32(board + 4 * k, 0x3f80_0000 ^ k);
        }
    }

    /// The gp floats the fence code reads, from the test's random source.
    fn oc662_gp_floats(c: &mut Cpu, r: &mut Rng) {
        for off in [0x2ad8u32, 0x2ad4, 0x2ad0, 0x2acc, 0x2ac8, 0x2ac4, 0x2ac0] {
            c.bus.write32(GAME_GP.wrapping_sub(off), r.edge_f32());
        }
    }

    #[test]
    fn fence_ctor_2_matches() {
        oc662_hooks();
        check(
            ee::s11::f_0034cbe8__generated,
            fence_ctor_2,
            200,
            &|c, seed| {
                let mut r = Rng(0x0662_0001 ^ seed as u64);
                machine(c, seed, 0x34cbe8, 0);
                c.pc = 0x0034_cbe8;
                c.bus.write32(OC662_CTRL, OC662_MAGIC);
                let fence = DATA + 0x100;
                let stream = DATA + 0x300;
                let record = DATA + 0x400;
                oc662_world(c, fence);
                c.bus.write32(stream, record);
                c.bus.write16(record + 0x10, 0x20);
                c.bus.write32(record + 0x14, FAKE + 0x40);
                for k in 0..9u32 {
                    c.bus.write32(fence + 4 * k, r.next_u32());
                }
                c.g[A0][0] = sx(fence);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(stream);
            },
        );
    }

    #[test]
    fn fence_ctor_matches() {
        oc662_hooks();
        check(
            ee::s11::f_0034cab8__generated,
            fence_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x0662_0002 ^ seed as u64);
                machine(c, seed, 0x34cab8, 0);
                c.pc = 0x0034_cab8;
                c.bus.write32(OC662_CTRL, OC662_MAGIC);
                let fence = DATA + 0x100;
                let obj = DATA + 0x400;
                let desc = DATA + 0x500;
                oc662_world(c, fence);
                oc662_gp_floats(c, &mut r);
                for k in 0..4u32 {
                    c.bus.write32(desc + 4 * k, r.next_u32());
                }
                c.f[12] = r.edge_f32();
                c.g[A0][0] = sx(fence);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(obj);
                c.g[A3][0] = sx(desc);
            },
        );
    }

    #[test]
    fn fence_ctor_3_matches() {
        oc662_hooks();
        check(
            ee::s11::f_0034c8e8__generated,
            fence_ctor_3,
            200,
            &|c, seed| {
                let mut r = Rng(0x0662_0003 ^ seed as u64);
                machine(c, seed, 0x34c8e8, 0);
                c.pc = 0x0034_c8e8;
                c.bus.write32(OC662_CTRL, OC662_MAGIC);
                let fence = DATA + 0x100;
                let obj = DATA + 0x400;
                let desc = DATA + 0x500;
                let info = DATA + 0x600;
                oc662_world(c, fence);
                oc662_gp_floats(c, &mut r);
                for k in 0..4u32 {
                    c.bus.write32(desc + 4 * k, r.next_u32());
                }
                for k in 0..0x14u32 {
                    c.bus.write32(info + 4 * k, r.edge_f32());
                }
                c.g[A0][0] = sx(fence);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(obj);
                c.g[A3][0] = sx(desc);
                c.g[T0][0] = sx(info);
            },
        );
    }

    #[test]
    fn fence_propagate_to_neighbour_matches() {
        oc662_hooks();
        check(
            ee::s11::f_0034cc80__generated,
            fence_propagate_to_neighbour,
            300,
            &|c, seed| {
                let mut r = Rng(0x0662_0004 ^ seed as u64);
                machine(c, seed, 0x34cc80, 3);
                c.pc = 0x0034_cc80;
                c.bus.write32(OC662_CTRL, OC662_MAGIC);
                let fence = DATA + 0x100;
                let node = DATA + 0x400;
                let nodea = DATA + 0x500;
                let vobj = DATA + 0x600;
                let next = DATA + 0x900;
                oc662_gp_floats(c, &mut r);
                c.bus.write32(fence + 0x40, if seed % 5 == 0 { 0 } else { DATA + 0x200 });
                c.bus.write32(fence + 0x30, r.edge_f32());
                c.bus.write32(fence + 0x34, r.edge_f32());
                c.bus.write32(fence + 0x104, if seed % 3 == 0 { 0 } else { DATA + 0x800 });
                c.bus.write32(fence + 0x108, if seed % 4 == 0 { 0 } else { DATA + 0x800 });
                c.bus.write32(DATA + 0x800 + 0xc, next);
                c.bus.write32(node + 0xc, if seed % 7 == 0 { 0 } else { nodea });
                c.bus.write32(nodea + 0xc, vobj);
                c.bus.write16(vobj + 0x50, 0x20);
                c.bus.write32(vobj + 0x54, FAKE + 0x50);
                oc662_world(c, next);
                c.g[A0][0] = sx(fence);
                c.g[A1][0] = if seed % 6 == 0 { 0 } else { sx(node) };
                c.g[A2][0] = if seed % 2 == 0 { 0 } else { sx(DATA + 0x700) };
            },
        );
    }

    #[test]
    fn fence_update_verts_matches() {
        check(
            ee::s11::f_0034cf98__generated,
            fence_update_verts,
            300,
            &|c, seed| {
                let mut r = Rng(0x0662_0005 ^ seed as u64);
                machine(c, seed, 0x34cf98, 0);
                c.pc = 0x0034_cf98;
                let fence = DATA + 0x100;
                let mesh = DATA + 0x200;
                let table = DATA + 0x300;
                oc662_gp_floats(c, &mut r);
                c.bus.write32(fence + 0x40, if seed % 5 == 0 { 0 } else { mesh });
                c.bus.write32(fence + 0x20, if seed % 2 == 0 { 1 } else { 2 });
                c.bus.write32(fence + 0x24, r.f32(10.0));
                c.bus.write32(fence + 0x2c, r.f32(10.0));
                c.bus.write32(fence + 0x44, r.next_u32());
                c.bus.write32(fence + 0x104, 0);
                c.bus.write32(mesh + 4, table);
                c.bus.write16(table + 0x18, 0);
                c.bus.write32(table + 0x1c, FAKE + 0x60);
                c.bus.write32(fence + 0xa4, DATA + 0x500);
                for k in 0..0x40u32 {
                    c.bus.write32(DATA + 0x400 + 4 * k, r.next_u32());
                    c.bus.write32(DATA + 0x500 + 4 * k, r.next_u32());
                }
                c.g[A0][0] = sx(fence);
            },
        );
    }
}
