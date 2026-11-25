//! Staging module oc663 (wave g43.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc663`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0034_d1e8, fence_build_mesh, "Fence_BuildMesh"),
    (0x0034_ec58, floating_ctor, "Floating_ctor"),
];

/// `void cFenceNode_setupFence(cFenceNode* self)` (0x34d1e8): clears the
/// node links, marks the instance (flags: bit 1 off, bit 2 on), creates the
/// 8x2 grid mesh through the graphics manager, reads the instance's quad
/// (`GetMeshQuad`), fills the 16 vertices and the 16 uv and colour pairs by
/// interpolating the quad's edges in eight steps (1/7 each), stores them
/// through the grid mesh's vt[6], then rebuilds the grid (`Fence_UpdateVerts`).
///
/// The stack scratch slots are written as the original does: the grid
/// update reads the same stack layout, and the diff compares the stack.
pub fn fence_build_mesh(c: &mut Cpu) {
    const UPDATE_GRID: Callee = Callee {
        addr: 0x0034_cf98,
        f: ee::s11::Fence_UpdateVerts,
    };
    guest(c, |c| -> Flow {
        c.cycles += 46;
        let frame = Frame::open(c, 0x300);
        let s = sp(c);
        let at = |off: u32| s.wrapping_add(off);
        c.g[V1][0] = 0xffff_ffff_ffff_fffd;
        frame.save(c, S0, 0x2f0);
        frame.save(c, S1, 0x2e0);
        frame.save(c, S2, 0x2d0);
        frame.save(c, S3, 0x2c0);
        frame.save(c, S4, 0x2b0);
        frame.save(c, S5, 0x2a0);
        frame.save(c, S6, 0x290);
        frame.save(c, S7, 0x280);
        frame.save_ra(c, 0x270);
        c.g[A1][0] = 8;
        c.g[A2][0] = 2;
        c.g[A3][0] = 0x6000_0000;
        c.g[S1][0] = c.g[A0][0];
        let fence = r32(c, S1);

        // self->NextInstance = self->PrevInstance = 0; flags of the instance.
        Addr(fence.wrapping_add(0x104)).sw(c, 0);
        Addr(fence.wrapping_add(0x108)).sw(c, 0);
        c.g[A0][0] = Addr(fence.wrapping_add(0x18)).lw(c);
        c.g[S0][0] = Addr(r32(c, GP).wrapping_add(0x2a90)).lw(c);
        let inst = r32(c, A0);
        c.g[V0][0] = Addr(inst.wrapping_add(8)).lw(c);
        let flags = (c.g[V0][0] & c.g[V1][0]) | 4;
        c.g[V0][0] = flags;
        Addr(inst.wrapping_add(8)).sw(c, flags);

        // self->GridMesh = gm->CreateGridMesh(8, 2, 0x60000000) (vt slot 0x260).
        let mgr = r32(c, S0);
        c.g[V1][0] = Addr(mgr.wrapping_add(0x10d8)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x260)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x264)).lw(c);
        let create = r32(c, V0);
        c.g[A0][0] = sx(mgr.wrapping_add(r32(c, A0)));
        call_ptr(c, create, 0x0034_d25c)?;
        let grid = c.g[V0][0];
        Addr(r32(c, S1).wrapping_add(0x40)).sw(c, grid);

        c.g[T0][0] = sx(at(0x30));
        c.g[T1][0] = sx(at(0x50));
        c.g[S5][0] = sx(at(0xd0));
        c.g[S6][0] = sx(at(0x110));
        c.g[S4][0] = sx(at(0x114));
        c.g[S3][0] = sx(at(0x118));
        c.g[S2][0] = sx(at(0x11c));
        c.g[S7][0] = sx(at(0x90));
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = 3;
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x10d8)).lw(c);
        c.g[V0][0] = 3;
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = 3;
        c.g[V1][0] = u64::MAX;

        // gm->GetMeshQuad(instance, 0, pos, quv, qcol) (vt slot 0x308).
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            let t = c.g[V0][0] != c.g[V1][0];
            c.g[A2][0] = 0;
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0034_d2a0);
                }
                continue;
            }
            break;
        }
        c.cycles += 11;
        c.g[A0][0] = Addr(r32(c, A1).wrapping_add(0x308)).lh(c);
        c.g[V0][0] = Addr(r32(c, A1).wrapping_add(0x30c)).lw(c);
        c.g[A3][0] = c.g[SP][0];
        c.g[A1][0] = Addr(r32(c, S1).wrapping_add(0x18)).lw(c);
        let quad = r32(c, V0);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        call_ptr(c, quad, 0x0034_d2d4)?;

        // Eight columns of 1/7 steps, one vertex pair per column.
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = 0xf;
        c.g[V0][0] = 0xf;
        c.g[V1][0] = u64::MAX;
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            let t = c.g[V0][0] != c.g[V1][0];
            c.g[T0][0] = c.g[S2][0];
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0034_d2e8);
                }
                continue;
            }
            break;
        }
        c.cycles += 9;
        c.lwc1(9, r32(c, GP).wrapping_sub(0x2abc)); // f9 = 1/7
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 10); // f10 = 1.0
        c.g[A3][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S4][0];
        c.g[T1][0] = 0;
        c.g[A1][0] = c.g[S6][0];
        c.g[T2][0] = sx(at(0x90));

        loop {
            c.cycles += 179;
            c.lwc1(7, s);
            c.mtc1(T1, 8);
            c.cvt_s_w(8, 8); // f8 = column
            c.lwc1(0, at(0xc));
            c.g[V0][0] = 0xc;
            c.lwc1(6, at(4));
            c.mult(0, V1, T1, V0);
            c.lwc1(5, at(8));
            c.sub_s(0, 0, 7);
            c.lwc1(1, at(0x10));
            c.lwc1(2, at(0x14));
            c.sub_s(1, 1, 6);
            c.swc1(0, at(0x240));
            c.sub_s(2, 2, 5);
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, S1)));
            c.mul_s(3, 0, 8);
            c.g[V1][0] = sx((c.g[T1][0] as u32) << 3);
            c.mul_s(4, 1, 8);
            c.swc1(1, at(0x244));
            c.mul_s(0, 2, 8);
            c.swc1(2, at(0x248));
            c.mul_s(1, 3, 9);
            c.swc1(3, at(0x230));
            c.mul_s(2, 4, 9);
            c.swc1(4, at(0x234));
            c.mul_s(3, 0, 9);
            c.swc1(0, at(0x238));
            c.add_s(7, 7, 1);
            c.swc1(1, at(0x220));
            c.add_s(6, 6, 2);
            c.swc1(2, at(0x224));
            c.add_s(5, 5, 3);
            c.swc1(3, at(0x228));
            c.swc1(7, at(0x210));
            c.g[V1][0] = sx(r32(c, S5).wrapping_add(r32(c, V1)));
            c.swc1(6, at(0x214));
            c.g[T1][0] = sx(r32(c, T1).wrapping_add(1));
            c.swc1(5, at(0x218));
            c.g[A0][0] = ((c.g[T1][0] as i64) < 8) as u64;
            c.ldl(11, at(0x217));
            c.ldr(11, at(0x210));
            c.g[T4][0] = Addr(at(0x218)).lw(c);
            c.sdl(11, r32(c, V0).wrapping_add(0x4b));
            c.sdr(11, r32(c, V0).wrapping_add(0x44));
            let v = c.g[T4][0];
            Addr(r32(c, V0).wrapping_add(0x4c)).sw(c, v);

            // Position component pair, second vertex row.
            c.lwc1(5, at(0x18));
            c.lwc1(0, at(0x24));
            c.lwc1(6, at(0x1c));
            c.lwc1(7, at(0x20));
            c.sub_s(0, 0, 5);
            c.lwc1(1, at(0x28));
            c.lwc1(2, at(0x2c));
            c.sub_s(1, 1, 6);
            c.swc1(0, at(0x250));
            c.sub_s(2, 2, 7);
            c.mul_s(3, 0, 8);
            c.mul_s(4, 1, 8);
            c.swc1(1, at(0x254));
            c.mul_s(0, 2, 8);
            c.swc1(2, at(0x258));
            c.mul_s(1, 3, 9);
            c.swc1(3, at(0x230));
            c.mul_s(2, 4, 9);
            c.swc1(4, at(0x234));
            c.mul_s(3, 0, 9);
            c.swc1(0, at(0x238));
            c.add_s(5, 5, 1);
            c.swc1(1, at(0x220));
            c.add_s(6, 6, 2);
            c.swc1(2, at(0x224));
            c.add_s(7, 7, 3);
            c.swc1(3, at(0x228));
            c.swc1(5, at(0x210));
            c.swc1(6, at(0x214));
            c.swc1(7, at(0x218));
            c.ldl(11, at(0x217));
            c.ldr(11, at(0x210));
            c.g[T4][0] = Addr(at(0x218)).lw(c);
            c.sdl(11, r32(c, V0).wrapping_add(0xab));
            c.sdr(11, r32(c, V0).wrapping_add(0xa4));
            let v = c.g[T4][0];
            Addr(r32(c, V0).wrapping_add(0xac)).sw(c, v);

            // Colour pair, first row.
            c.lwc1(4, at(0x30));
            c.lwc1(5, at(0x34));
            c.lwc1(0, at(0x38));
            c.lwc1(1, at(0x3c));
            c.sub_s(0, 0, 4);
            c.sub_s(1, 1, 5);
            c.mul_s(2, 0, 8);
            c.swc1(0, at(0x260));
            c.mul_s(3, 1, 8);
            c.swc1(1, at(0x264));
            c.mul_s(0, 2, 9);
            c.swc1(2, at(0x230));
            c.mul_s(1, 3, 9);
            c.swc1(3, at(0x234));
            c.add_s(4, 4, 0);
            c.swc1(0, at(0x220));
            c.add_s(5, 5, 1);
            c.swc1(1, at(0x224));
            c.swc1(4, at(0x210));
            c.swc1(5, at(0x214));
            c.ldl(2, at(0x217));
            c.ldr(2, at(0x210));
            c.sdl(2, r32(c, T2).wrapping_add(0x7));
            c.sdr(2, r32(c, T2));
            c.g[T2][0] = sx(r32(c, T2).wrapping_add(8));

            // Colour pair, second row.
            c.lwc1(4, at(0x40));
            c.lwc1(5, at(0x44));
            c.lwc1(0, at(0x48));
            c.lwc1(1, at(0x4c));
            c.sub_s(0, 0, 4);
            c.sub_s(1, 1, 5);
            c.mul_s(2, 0, 8);
            c.swc1(0, at(0x240));
            c.mul_s(3, 1, 8);
            c.swc1(1, at(0x244));
            c.mul_s(0, 2, 9);
            c.swc1(2, at(0x230));
            c.mul_s(1, 3, 9);
            c.swc1(3, at(0x234));
            c.add_s(4, 4, 0);
            c.swc1(0, at(0x220));
            c.add_s(5, 5, 1);
            c.swc1(1, at(0x224));
            c.swc1(4, at(0x210));
            c.swc1(5, at(0x214));
            c.ldl(14, at(0x217));
            c.ldr(14, at(0x210));
            c.sdl(14, r32(c, V1).wrapping_add(0x7));
            c.sdr(14, r32(c, V1));
            c.swc1(10, r32(c, A1));

            // Vertex and uv offsets of the other pairs (A2, A3, T0, A1+0x80...).
            c.lwc1(1, at(0x54));
            c.lwc1(0, at(0x64));
            c.sub_s(0, 0, 1);
            c.mul_s(0, 0, 8);
            c.mul_s(0, 0, 9);
            c.add_s(1, 1, 0);
            c.swc1(1, r32(c, A2));
            c.lwc1(2, at(0x58));
            c.lwc1(0, at(0x68));
            c.sub_s(0, 0, 2);
            c.mul_s(0, 0, 8);
            c.mul_s(0, 0, 9);
            c.add_s(2, 2, 0);
            c.swc1(2, r32(c, A3));
            c.lwc1(1, at(0x5c));
            c.lwc1(0, at(0x6c));
            c.sub_s(0, 0, 1);
            c.mul_s(0, 0, 8);
            c.mul_s(0, 0, 9);
            c.add_s(1, 1, 0);
            c.swc1(1, r32(c, T0));
            c.swc1(10, r32(c, A1).wrapping_add(0x80));
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x10));
            c.lwc1(1, at(0x74));
            c.lwc1(0, at(0x84));
            c.sub_s(0, 0, 1);
            c.mul_s(0, 0, 8);
            c.mul_s(0, 0, 9);
            c.add_s(1, 1, 0);
            c.swc1(1, r32(c, A2).wrapping_add(0x80));
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
            c.lwc1(1, at(0x78));
            c.lwc1(0, at(0x88));
            c.sub_s(0, 0, 1);
            c.mul_s(0, 0, 8);
            c.mul_s(0, 0, 9);
            c.add_s(1, 1, 0);
            c.swc1(1, r32(c, A3).wrapping_add(0x80));
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(0x10));
            c.lwc1(1, at(0x7c));
            c.lwc1(0, at(0x8c));
            c.sub_s(0, 0, 1);
            c.mul_s(0, 0, 8);
            c.mul_s(0, 0, 9);
            c.add_s(1, 1, 0);
            c.swc1(1, r32(c, T0).wrapping_add(0x80));
            let more = c.g[A0][0] != 0;
            c.g[T0][0] = sx(r32(c, T0).wrapping_add(0x10));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0034_d328);
                }
                continue;
            }
            break;
        }

        // gridMesh->vt[6].fn(gridMesh, pos, col, uv), then rebuild the grid.
        c.cycles += 22;
        c.g[T0][0] = Addr(r32(c, S1).wrapping_add(0x40)).lw(c);
        c.g[A2][0] = c.g[S6][0];
        c.g[A3][0] = c.g[S7][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[V0][0] = Addr(r32(c, T0).wrapping_add(4)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x30)).lh(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x34)).lw(c);
        let vfn = r32(c, V1);
        c.g[A0][0] = sx(r32(c, T0).wrapping_add(r32(c, A0)));
        call_ptr(c, vfn, 0x0034_d618)?;
        c.g[A0][0] = c.g[S1][0];
        UPDATE_GRID.call(c, 0x0034_d620)?;

        frame.restore(c, S0, 0x2f0);
        frame.restore(c, S1, 0x2e0);
        frame.restore(c, S2, 0x2d0);
        frame.restore(c, S3, 0x2c0);
        frame.restore(c, S4, 0x2b0);
        frame.restore(c, S5, 0x2a0);
        frame.restore(c, S6, 0x290);
        frame.restore(c, S7, 0x280);
        frame.restore_ra(c, 0x270);
        frame.close(c);
        Ok(())
    });
}

/// `cAnimNode_cAnimNode1(self, a1, stream)` (0x34dac8), the stream-loaded
/// animation node: constructs the embedded ModObj (`ModObj_ctor_2` on
/// `self + 0x14`), sets the vtable, allocates one `tAnimMat` per mesh of
/// the node's model, builds the mesh cache (`AnimObject_AllocNodeInfo`),
/// reads its 20-byte header and one word from the stream, and evaluates the
/// first frame (`AnimObject_v52`). Returns `self`.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn anim_object_base_ctor_2(c: &mut Cpu) {
    const MOD_OBJ_CTOR_2: Callee = Callee {
        addr: 0x0035_52d8,
        f: ee::s11::ModObj_ctor_2,
    };
    const ALLOC_NODE_INFO: Callee = Callee {
        addr: 0x0034_e448,
        f: ee::s11::AnimObject_AllocNodeInfo,
    };
    const V52: Callee = Callee {
        addr: 0x0034_e348,
        f: ee::s11::AnimObject_v52,
    };
    guest(c, |c| -> Flow {
        c.cycles += 26;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        c.g[S0][0] = sx(r32(c, S1).wrapping_add(0x14));
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.g[A0][0] = c.g[S0][0];
        frame.save_ra(c, 0x10);
        c.g[S3][0] = sx(r32(c, S1).wrapping_add(0x44));
        MOD_OBJ_CTOR_2.call(c, 0x0034_daf8)?;

        // The vtable (0x490cc8) and the array's name (0x48e758).
        c.g[V1][0] = 0x490000;
        c.g[A1][0] = 0x490000;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0xcc8));
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x18a8));
        Addr(r32(c, S1).wrapping_add(0x20)).sw(c, c.g[V1][0]);
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x18)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x80)).lw(c);
        c.g[S0][0] = Addr(r32(c, V1).wrapping_add(4)).lw(c);
        c.g[A0][0] = sx(r32(c, S0) << 6);
        call_ptr(c, 0x0031_7e30, 0x0034_db28)?;
        let none = c.g[S0][0] == 0;
        c.g[V1][0] = sx(r32(c, S0).wrapping_sub(1));
        if !none {
            c.cycles += 2;
            c.g[A0][0] = u64::MAX;
        }
        c.cycles += 27;
        c.g[A0][0] = c.g[S1][0];
        let slots = c.g[V0][0];
        Addr(r32(c, S3)).sw(c, slots);
        ALLOC_NODE_INFO.call(c, 0x0034_db44)?;

        // stream->read(self, 20); stream->read(&skipped, 4).
        c.g[V1][0] = Addr(r32(c, S2)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0x14;
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x10)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x14)).lw(c);
        let read = r32(c, V0);
        c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
        call_ptr(c, read, 0x0034_db60)?;
        c.g[V1][0] = Addr(r32(c, S2)).lw(c);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 4;
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x10)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x14)).lw(c);
        let read = r32(c, V0);
        c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
        call_ptr(c, read, 0x0034_db7c)?;

        c.g[A0][0] = c.g[S1][0];
        V52.call(c, 0x0034_db84)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `cFloatingNode_cFloatingNode(self, bucket, instRecord, info)` (0x34ec58):
/// the moving-node constructor for floating (spring-mounted) nodes. Builds
/// the ModObj (kind 0xe), sets the vtable, copies the spring and damping
/// constants from `info` (torque scaled by 1e-6), zeroes the motion state,
/// takes the root position from the instance transform, allocates one frame
/// per mesh and sets the instance's flag bit 2 (bit 1 off).
pub fn floating_ctor(c: &mut Cpu) {
    const MOD_OBJ_CTOR: Callee = Callee {
        addr: 0x0035_5280,
        f: ee::s11::ModObj_ctor,
    };
    const ALLOC_BOUND_BOX: Callee = Callee {
        addr: 0x0034_eeb8,
        f: ee::s11::Floating_AllocBoundBox,
    };
    guest(c, |c| -> Flow {
        c.cycles += 56;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A0][0];
        c.g[A3][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A2][0] = 0xe;
        MOD_OBJ_CTOR.call(c, 0x0034_ec80)?;

        let me = r32(c, S1);
        let info = r32(c, S0);
        c.g[S2][0] = sx(me.wrapping_add(0x74));
        c.g[V1][0] = 0x490000;
        c.lwc1(3, r32(c, GP).wrapping_sub(0x2a9c)); // f3 = 1e-6
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x7a8));
        c.g[A1][0] = Addr(me.wrapping_add(0x18)).lw(c);
        Addr(me.wrapping_add(0xc)).sw(c, c.g[V1][0]);
        c.g[A0][0] = c.g[S1][0];
        c.lwc1(0, info.wrapping_add(4));
        c.swc1(0, me.wrapping_add(0x2c));
        c.lwc1(1, info.wrapping_add(8));
        c.swc1(1, me.wrapping_add(0x30));
        c.lwc1(0, info.wrapping_add(0x14));
        c.swc1(0, me.wrapping_add(0x34));
        c.lwc1(1, info.wrapping_add(0xc));
        c.swc1(1, me.wrapping_add(0x38));
        c.lwc1(2, info.wrapping_add(0x10));
        c.swc1(2, me.wrapping_add(0x3c));
        c.lwc1(0, info.wrapping_add(0x18));
        c.mul_s(0, 0, 3);
        c.swc1(0, me.wrapping_add(0x40));
        // The root position: the instance transform's translation quad.
        c.lq(2, r32(c, A1).wrapping_add(0x40));
        Addr(me.wrapping_add(0x54)).sw(c, 0);
        c.sq(2, sp(c));
        Addr(me.wrapping_add(0x5c)).sw(c, 0);
        c.lwc1(0, sp(c).wrapping_add(8));
        Addr(me.wrapping_add(0x60)).sw(c, 0);
        Addr(me.wrapping_add(0x64)).sw(c, 0);
        Addr(me.wrapping_add(0x6c)).sw(c, 0);
        Addr(me.wrapping_add(0x70)).sw(c, 0);
        Addr(me.wrapping_add(0x48)).sw(c, 0);
        Addr(me.wrapping_add(0x44)).sw(c, 0);
        Addr(me.wrapping_add(0x78)).sw(c, 0);
        c.swc1(0, me.wrapping_add(0x50));
        c.swc1(0, me.wrapping_add(0x4c));
        ALLOC_BOUND_BOX.call(c, 0x0034_ed0c)?;

        // One frame per mesh of the instance's model.
        c.g[V1][0] = Addr(r32(c, S1).wrapping_add(0x18)).lw(c);
        c.g[A1][0] = sx(0x490000u32.wrapping_sub(0x1780));
        c.g[A2][0] = 0x2000_0000;
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x80)).lw(c);
        c.g[A3][0] = 0;
        c.g[S0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
        c.g[A0][0] = sx(r32(c, S0) << 6);
        call_ptr(c, 0x0031_7e30, 0x0034_ed30)?;
        let none = c.g[S0][0] == 0;
        c.g[V1][0] = sx(r32(c, S0).wrapping_sub(1));
        if !none {
            c.cycles += 2;
            c.g[A0][0] = u64::MAX;
        }
        c.cycles += 18;
        let frames = c.g[V0][0];
        Addr(r32(c, S2)).sw(c, frames);
        c.g[A1][0] = 0xffff_ffff_ffff_fffd;
        c.g[V0][0] = c.g[S1][0];
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x12)).lhu(c);
        c.g[A2][0] = Addr(r32(c, S1).wrapping_add(0x18)).lw(c);
        c.g[A0][0] |= 1;
        frame.restore(c, S0, 0x40);
        Addr(r32(c, S1).wrapping_add(0x12)).sh(c, c.g[A0][0]);
        frame.restore(c, S1, 0x30);
        c.g[V1][0] = Addr(r32(c, A2).wrapping_add(8)).lw(c);
        frame.restore(c, S2, 0x20);
        c.g[V1][0] &= c.g[A1][0];
        frame.restore_ra(c, 0x10);
        c.g[V1][0] |= 4;
        let fl = c.g[V1][0];
        Addr(r32(c, A2).wrapping_add(8)).sw(c, fl);
        frame.close(c);
        Ok(())
    });
}

/// `Floating_ctor_2` (0x34ed88): the second floating-node constructor, the
/// same node without the spring setup. Builds the ModObj (`ModObj_ctor_2`),
/// sets the vtable and zeroes the motion slot, runs the node's virtual
/// update on `info` and bounds the node only when that reports a result
/// (`AllocBoundBox`), then the per-mesh frames and the flag bit 0 (`|= 1`).
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn floating_ctor_2(c: &mut Cpu) {
    const MOD_OBJ_CTOR_2: Callee = Callee {
        addr: 0x0035_52d8,
        f: ee::s11::ModObj_ctor_2,
    };
    const ALLOC_BOUND_BOX: Callee = Callee {
        addr: 0x0034_eeb8,
        f: ee::s11::Floating_AllocBoundBox,
    };
    guest(c, |c| -> Flow {
        c.cycles += 22;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A0][0];
        MOD_OBJ_CTOR_2.call(c, 0x0034_eda8)?;

        let me = r32(c, S1);
        c.g[V1][0] = 0x490000;
        Addr(me.wrapping_add(0x78)).sw(c, 0);
        c.g[V1][0] = sx(0x490000u32.wrapping_sub(0x7a8));
        c.g[A1][0] = c.g[SP][0];
        Addr(me.wrapping_add(0xc)).sw(c, c.g[V1][0]);
        c.g[A2][0] = 4;
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x10)).lh(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x14)).lw(c);
        let update = r32(c, V1);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        call_ptr(c, update, 0x0034_edd4)?;
        c.g[V0][0] = Addr(sp(c)).lw(c);
        if c.g[V0][0] == 0 {
            c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            ALLOC_BOUND_BOX.call(c, 0x0034_ede8)?;
            c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        }

        // The per-mesh frames (vtable slot 0x10/0x14 of the second argument).
        c.cycles += 18;
        c.g[A1][0] = sx(me.wrapping_add(0x2c));
        c.g[A2][0] = 0x48;
        c.g[S2][0] = sx(me.wrapping_add(0x74));
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x10)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x14)).lw(c);
        let bind = r32(c, V0);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        call_ptr(c, bind, 0x0034_ee08)?;
        c.g[V1][0] = Addr(r32(c, S1).wrapping_add(0x18)).lw(c);
        c.g[A1][0] = sx(0x490000u32.wrapping_sub(0x1780));
        c.g[A2][0] = 0x2000_0000;
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x80)).lw(c);
        c.g[A3][0] = 0;
        c.g[S0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
        c.g[A0][0] = sx(r32(c, S0) << 6);
        call_ptr(c, 0x0031_7e30, 0x0034_ee2c)?;
        let none = c.g[S0][0] == 0;
        c.g[V1][0] = sx(r32(c, S0).wrapping_sub(1));
        if !none {
            c.cycles += 1;
            c.g[A0][0] = u64::MAX;
        }
        c.cycles += 11;
        let frames = c.g[V0][0];
        Addr(r32(c, S2)).sw(c, frames);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x40);
        c.g[V1][0] = Addr(r32(c, S1).wrapping_add(0x12)).lhu(c);
        frame.restore(c, S2, 0x20);
        c.g[V1][0] |= 1;
        frame.restore_ra(c, 0x10);
        let flags = c.g[V1][0];
        Addr(r32(c, S1).wrapping_add(0x12)).sh(c, flags);
        frame.restore(c, S1, 0x30);
        frame.close(c);
        Ok(())
    });
}

/// `Floating_v50` (0x34ef40): the floating node's per-frame update. When the
/// node has a second part (`self + 0x78`), it blends two of its sub-objects'
/// matrices' rows with VU0 (`sub`, then `add`) into the part's rows, then
/// calls the part's virtual at `0x2d1be0` and `0x3291e0`. Otherwise it calls
/// `ModObj_v50` on the node.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn floating_v50(c: &mut Cpu) {
    const MOD_OBJ_V50: Callee = Callee {
        addr: 0x0035_68b0,
        f: ee::s11::ModObj_v50,
    };
    guest(c, |c| -> Flow {
        c.cycles += 7;
        let frame = Frame::open(c, 0xa0);
        let s = sp(c);
        let at = |off: u32| s.wrapping_add(off);
        frame.save(c, S0, 0x90);
        frame.save_ra(c, 0x80);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x78)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 52;
            c.lq(3, r32(c, A0));
            c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
            c.sq(3, at(0x20));
            c.lq(2, r32(c, A0).wrapping_add(0x10));
            c.sq(2, at(0x30));
            c.lwc1(0, r32(c, A0).wrapping_add(0x20));
            Addr(at(0x4c)).sw(c, 0);
            c.swc1(0, at(0x48));
            c.swc1(0, at(0x40));
            c.swc1(0, at(0x44));
            c.g[A0][0] = Addr(r32(c, A1).wrapping_add(0xc0)).lh(c);
            c.g[V0][0] = Addr(r32(c, A1).wrapping_add(0xc4)).lw(c);
            let part = r32(c, V0);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            call_ptr(c, part, 0x0034_ef94)?;
            c.lq(3, r32(c, V0).wrapping_add(0x30));
            c.sq(3, at(0x60));
            c.lqc2(3, at(0x60));
            c.lqc2(4, at(0x40));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(0x70));
            c.lq(2, at(0x70));
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
            c.sq(2, s);
            c.sq(2, at(0x50));
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0xc0)).lh(c);
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0xc4)).lw(c);
            let part = r32(c, V0);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            call_ptr(c, part, 0x0034_efcc)?;
            c.lq(3, r32(c, V0).wrapping_add(0x30));
            c.sq(3, at(0x60));
            c.lqc2(3, at(0x60));
            c.lqc2(4, at(0x40));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(0x70));
            c.lq(2, at(0x70));
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x78)).lw(c);
            c.lq(3, s);
            c.sq(2, at(0x10));
            c.sq(2, at(0x50));
            c.sq(3, r32(c, A0));
            c.lq(2, at(0x10));
            c.sq(2, r32(c, A0).wrapping_add(0x10));
            c.g[S0][0] = Addr(r32(c, S0).wrapping_add(0x18)).lw(c);
            call_ptr(c, 0x002d_1be0, 0x0034_f00c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.g[A3][0] = c.g[SP][0];
            c.g[T0][0] = sx(at(0x20));
            call_ptr(c, 0x0032_91e0, 0x0034_f024)?;
            frame.restore(c, S0, 0x90);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            MOD_OBJ_V50.call(c, 0x0034_f034)?;
            frame.restore(c, S0, 0x90);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, elf_image, fill, DATA, GAME_GP, SP as STACK};

    /// A node's virtual table: slot `off` is a 16-bit delta and the function
    /// pointer `FAKE + 0x10 * i` (a logged, forced-result fake call).
    fn slot(c: &mut Cpu, vt: u32, off: u32, i: u32, delta: u16) {
        c.bus.write16(vt + off, delta);
        c.bus.write32(vt + off + 4, FAKE + 0x10 * i);
    }

    /// The `ModObj_ctor` fixture of `anim_object_base_ctor_matches`: process
    /// lists, world -> focus, the node's model with `defs` definitions.
    fn mod_obj_fixture(c: &mut Cpu, r: &mut crate::difftest::Rng, node: u32, defs: u32) {
        const PROCS: u32 = DATA + 0x1000;
        const TAILS: u32 = DATA + 0x1400;
        const WORLD: u32 = DATA + 0x2000;
        const FOCUS: u32 = DATA + 0x3000;
        const DEF: u32 = DATA + 0x400;
        c.bus.write32(GAME_GP + 0x2898 + 4, PROCS);
        for i in 0..4 {
            c.bus.write32(PROCS + 0x44 * i, r.next_u32());
            c.bus.write32(PROCS + 0x44 * i + 8, TAILS + 0x40 * i);
            c.bus.write32(PROCS + 0x44 * i + 0x28, TAILS + 0x40 * i + 0x20);
        }
        fill(c, r, 0x0044_aff0, 0x20);
        c.bus.write32(GAME_GP - 0x848, WORLD);
        c.bus.write32(WORLD + 0x84, WORLD + 0x100);
        c.bus.write32(WORLD + 0x100 + 0xc, WORLD + 0x200);
        c.bus.write32(WORLD + 0x200 + 0xa4, FOCUS);
        c.bus.write32(FOCUS + 0xd4, r.next_u32());
        c.bus.write32(FOCUS + 0xd8, r.next_u32() % 8);
        c.bus.write32(node + 0x80, DEF);
        c.bus.write32(DEF + 4, defs);
        c.bus.write32(DEF + 8, DEF + 0x100);
        for i in 0..defs {
            c.bus.write32(DEF + 0x100 + 0x10 * i + 8, 0);
        }
    }

    #[test]
    fn fence_build_mesh_matches() {
        const SELF: u32 = DATA + 0x1000;
        const INST: u32 = DATA + 0x2000;
        const MGR: u32 = DATA + 0x3000;
        const MGR_VT: u32 = DATA + 0x4000;
        const GRID: u32 = DATA + 0x5000;
        const GRID_VT: u32 = DATA + 0x6000;
        check(ee::s11::f_0034d1e8__generated, fence_build_mesh, 120, &|c, seed| {
            elf_image(c);
            let mut r = machine(c, seed, 0x34_d1e8, 0);
            c.pc = 0x0034_d1e8;
            fill(c, &mut r, SELF, 0x60);
            fill(c, &mut r, INST, 0x40);
            fill(c, &mut r, STACK - 0x300, 0x40);
            c.bus.write32(SELF + 0x18, INST);
            c.bus.write32(GAME_GP + 0x2a90, MGR);
            c.bus.write32(MGR + 0x10d8, MGR_VT);
            slot(c, MGR_VT, 0x260, 0, 0x40);
            slot(c, MGR_VT, 0x308, 1, 0x20);
            c.bus.write32(GRID + 4, GRID_VT);
            slot(c, GRID_VT, 0x30, 2, 0x10);
            c.bus.write32(LOG_RESULTS, GRID);
            c.g[A0][0] = sx(SELF);
        });
    }

    /// `cAnimNode_cAnimNode1`: the ModObj fixture, the stream's two reads
    /// (fake virtuals), a model of 1..3 meshes for the array and the cache.


    /// `cFloatingNode_cFloatingNode`: the ModObj fixture (kind 0xe), the
    /// info record's constants, one or two meshes for the frame array.
    #[test]
    fn floating_ctor_matches() {
        use crate::game::worldpainter::quadtree::tests::alloc_machine;
        const SELF: u32 = DATA + 0x100;
        const NODE: u32 = DATA + 0x200;
        const INFO: u32 = DATA + 0x700;
        check(
            ee::s11::f_0034ec58__generated,
            floating_ctor,
            120,
            &|c, seed| {
                elf_image(c);
                let mut r = alloc_machine(c, seed, 0x34_ec58);
                c.pc = 0x0034_ec58;
                fill(c, &mut r, NODE, 0x20);
                fill(c, &mut r, NODE + 0x40, 4);
                fill(c, &mut r, INFO, 0x10);
                mod_obj_fixture(c, &mut r, NODE, 1 + seed % 2);
                c.bus.write32(SELF + 0x18, NODE);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(seed % 4);
                c.g[A2][0] = sx(NODE);
                c.g[A3][0] = sx(INFO);
            },
        );
    }

}
